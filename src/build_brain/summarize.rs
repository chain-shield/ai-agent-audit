/// Protocol and file summarization using LLMs.
///
/// This module generates intelligent summaries of smart contract protocols and
/// individual source files using OpenAI models. Provides cached summarization
/// for protocol overviews and contextual information for AI analysis.
use anyhow::Result;
use log::info;
use rig::{
    client::CompletionClient,
    providers::openai::{self, O3},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use strum_macros::EnumString;
use tokio::sync::Semaphore;

use crate::{
    build_brain::summarize_db::{
        get_file_summary_from_db, get_summaries_from_db, insert_file_summaries_to_db,
        insert_file_summary_to_db,
    },
    cost::cost_data::{add_to_inference_cost_by_type, TokenType},
    llm_review::enums::AgentMetadata,
    prepare_code::git_clone::RepoPaths,
    utils::{contract_name_check::has_non_mock_contract, extract_retry::extractor_with_retry},
};
use crate::{llm_review::context_state, utils::check_folder_name::is_script_file};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, strum_macros::Display)]
pub enum FileSummaryType {
    Source,
    DeployScript,
    OutOfScope,
}

/// Represents a summary of a source file with metadata
#[derive(Default, Debug, Clone)]
pub struct SrcFileSummary {
    /// Source file name
    pub filename: String,
    /// AI-generated summary of the file's purpose and functionality
    pub summary: String,
    pub file_type: Option<FileSummaryType>,
}

/// Structured response format for LLM file summarization
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileSummary {
    /// The generated summary text
    pub summary: String,
}

// pub const MAX_WORDS_CONTRACT_SUMMARY: u16 = 300;
// pub const MAX_WORDS_FUNCTION_SUMMARY: u16 = 50;
// pub const MAX_CHARS_STORAGE_DESC: u16 = 50;
pub const MAX_WORDS_CONTRACT_SUMMARY: u16 = 100;
pub const MAX_WORDS_FUNCTION_SUMMARY: u16 = 20;
pub const MAX_CHARS_STORAGE_DESC: u16 = 20;

pub async fn summarize_src_files(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<Vec<SrcFileSummary>> {
    // pull summaries from db if avaliable
    let summaries = get_summaries_from_db(repo)?;
    if !summaries.is_empty() {
        info!("retrived summaries from db");
        return Ok(summaries);
    }
    info!("generating file summaries...");

    let openai_client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);

    let mut context =
        context_state::generate_slither_metadata_prompt_context(repo, &semantics_path).await?;

    // add docs to context
    let documentation = repo.extract_content_from_docs()?;
    context.push_str("\n ## DOCUMENTATION: \n\n ");
    context.push_str(&documentation);

    // info!("slither metadata => {:#?}", context);
    info!("generate summmary of all major files and docs in repo...");
    let preamble_source_summary = format!(
        "You are a senior solidity dev. Please summarize below source code. Format in markdown for easy reading. Start with a {MAX_WORDS_CONTRACT_SUMMARY} word or less summary of the contract, that includes  purpose trust model (user funds? admin?), also major entrypoints. Then list storage vars plus optional {MAX_CHARS_STORAGE_DESC} max chars description for each. For EACH function provide full interface; it should include visibility, modifiers, and mutability. Adjacent to function interface, add {MAX_WORDS_FUNCTION_SUMMARY} word max natspec for EACH function. Respond only with valid JSON matching the schema!");
    let preamble_deploy_script_summary = format!(
        r#"
You are a senior Web3 deploy engineer. Summarize the deployment script (TS/JS/Hardhat/Ignition/Foundry). Use markdown. Start with a 100-word summary: purpose, target networks/env, trust model (who holds keys/roles), major steps.
Then sections:
1) Inputs/Config: env vars, CLI args, constants/defaults, network logic, preconditions (≤20 chars each).
2) Dependencies: external libs/tools (ethers/hardhat/ignition/viem), prior contracts/artifacts.
3) Contracts Deployed/Interacted: for each—name, method (new/deploy/module/proxy type), constructor/init args (symbolic), post-deploy actions, outputs.
4) Steps (ordered): each step’s anchor (function/task); 20-word max note of effects; key params.
5) Permissions/Trust: ownership transfers, roles, approvals; who controls what after.
6) Post-Deploy Outputs: verification, artifacts, addresses/registries, exports.
7) Safety/Idempotency: skip-if-deployed, waits/confirmations, gas/network settings, failure handling.
8) Script Functions/Tasks: each full interface (name(params): returns; visibility/exported; async); 20-word (max) purpose/effects.
Respond only with valid JSON matching the schema!
"#
    );

    let ai_summary_agent = openai_client
        .extractor::<FileSummary>("gpt-5")
        .preamble(&preamble_source_summary)
        .context(&context)
        .build();

    let ai_deploy_summary_agent = openai_client
        .extractor::<FileSummary>("gpt-5")
        .preamble(&preamble_deploy_script_summary)
        .context(&context)
        .build();
    // ---------------------------------------------
    // 1.  PREP – collect the  files we want to summarize first
    // ---------------------------------------------
    let mut work_items = Vec::new();

    let monorepo_folders = repo.extract_monorepo_folders()?;
    let mut current_file_summary_type = FileSummaryType::OutOfScope;

    // Walk through the repository and collect relevant files
    for file in &repo.sol_files {
        // if protocol in monorepo make sure file to summarize is in the monorepo
        if !monorepo_folders.is_empty() {
            let is_in_monorepo = monorepo_folders.iter().any(|dir| file.starts_with(dir));

            if !is_in_monorepo {
                continue;
            }
        }

        // check if lib folder or test files
        if file.to_string_lossy().contains("lib") || file.to_string_lossy().contains("t.sol") {
            continue;
        }

        // check if file is in scope for summary (source or script) for foundry projects
        if file.extension().map_or(false, |ext| ext == "sol")
            && !file
                .file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |n| n.ends_with("t.sol"))
        {
            if is_script_file(file) {
                current_file_summary_type = FileSummaryType::DeployScript;
            } else if repo.source_code_folders.iter().any(|f| file.starts_with(f)) {
                current_file_summary_type = FileSummaryType::Source;
            } else {
                current_file_summary_type = FileSummaryType::OutOfScope;
            }
        }

        // check file type for hardhat
        if current_file_summary_type == FileSummaryType::OutOfScope
            && file.extension().map_or(false, |ext| ext == "ts")
            && is_script_file(file)
        {
            current_file_summary_type = FileSummaryType::DeployScript;
        }

        if current_file_summary_type == FileSummaryType::OutOfScope {
            continue;
        }

        // Skip directories and symlinks
        if !file.is_file() || fs::symlink_metadata(file)?.file_type().is_symlink() {
            continue;
        }

        let content = fs::read_to_string(&file)?;

        // skip if content does not have have at least one line that start with contract and contract
        // name does NOT contain 'mock' (case insensative)
        let has_non_mock_contract = has_non_mock_contract(&content);

        if !has_non_mock_contract {
            continue;
        }

        // push full path & content into the work queue
        work_items.push((file.to_owned(), content, current_file_summary_type.clone()));
    }

    // let mut files_to_summarize = String::new();
    //
    // for (file, _, file_type) in work_items {
    //     files_to_summarize.push_str(&format!(
    //         "file: {}, type: {}\n",
    //         file.display(),
    //         file_type.to_string()
    //     ));
    // }
    // info!("{}", files_to_summarize);

    let max_parallel = 50;
    let sem = Arc::new(Semaphore::new(max_parallel));
    let agent = Arc::new(ai_summary_agent); // the OpenAI client
    let deploy_agent = Arc::new(ai_deploy_summary_agent); // the OpenAI client
    let mut handles = Vec::new();

    for (file, content, file_type) in work_items {
        let sem = sem.clone();

        // NOTE: skipping deploy script summaries
        if file_type != FileSummaryType::Source {
            continue;
        }

        let agent = if file_type == FileSummaryType::Source {
            agent.clone()
        } else {
            deploy_agent.clone()
        };

        let repo_root = repo.root.clone();

        let handle = tokio::spawn(async move {
            // acquire permit – blocks if `max_parallel` already in-flight
            let _permit = sem.acquire_owned().await.unwrap();

            let metadata = AgentMetadata {
                model: O3.to_string(),
                ..Default::default()
            };

            info!("summarizing {}", file.display());

            match extractor_with_retry(&agent, &content, &metadata).await {
                Ok(res) => {
                    let filename = file
                        .strip_prefix(&repo_root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .to_string();

                    add_to_inference_cost_by_type(&res.summary, &metadata, TokenType::Output).await;
                    Some(SrcFileSummary {
                        filename,
                        summary: res.summary,
                        file_type: Some(file_type),
                    })
                }
                Err(e) => {
                    log::error!("❌ summarizing {} failed: {e}", file.display());
                    None
                }
            }
        });
        handles.push(handle);
    }

    // wait for all tasks
    let mut summaries = Vec::new();
    for h in handles {
        if let Some(s) = h.await? {
            summaries.push(s);
        }
    }

    // save files to db
    insert_file_summaries_to_db(&summaries, repo)?;

    for summary in &summaries {
        info!("filename: {}", summary.filename);
        info!("summary size: {}", summary.summary.len())
    }

    Ok(summaries)
}
//
pub async fn summarize_protocol(repo: &RepoPaths, context: Option<&str>) -> Result<String> {
    // pull protocol-summary from db, if avaliable
    let protocol_summary = get_file_summary_from_db("protocol-summary", repo)?;
    if let Some(summary) = protocol_summary {
        info!("retriving protocol from db");
        return Ok(summary.summary);
    }
    info!("generating protocol summary...");

    // if no cached context is required
    let context = context.expect("if summary of protocol is not cached must provide context");

    let openai_client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);

    log::info!("generate context for code review");
    let preamble = "You are a senior solidity dev. Given the context provided for solidity smart contract protocol, please create a max 4000 word detailed summary of this protocol explaining what it is, and how it works. Format in markdown for easy reading. Respond only with valid JSON matching the schema!";

    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(O3)
        .preamble(preamble)
        .build();

    log::info!("extracting protocol summary");
    // rerun if NoDataExtracted Error

    let metadata = AgentMetadata {
        model: O3.to_string(),
        ..Default::default()
    };

    let summary = extractor_with_retry(&ai_summary_agent, &context, &metadata).await?;

    add_to_inference_cost_by_type(&summary.summary, &metadata, TokenType::Output).await;

    // save to db
    insert_file_summary_to_db("protocol-summary", &summary.summary, repo)?;

    log::info!("protocol summary => {:#?}", summary);

    Ok(summary.summary)
}
