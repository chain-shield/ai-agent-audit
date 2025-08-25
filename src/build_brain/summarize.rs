/// Protocol and file summarization using LLMs.
///
/// This module generates intelligent summaries of smart contract protocols and
/// individual source files using OpenAI models. Provides cached summarization
/// for protocol overviews and contextual information for AI analysis.
use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use rig::{
    client::CompletionClient,
    providers::openai::{self, GPT_4O, O3},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

use crate::{
    cost::cost_data::add_to_inference_cost_by_type,
    prepare_code::git_clone::RepoPaths,
    utils::{
        check_folder_name::{is_script_file, is_test_file},
        contract_name_check::has_non_mock_contract,
        extract_retry::extractor_with_retry,
    },
};
use crate::{cost::cost_data::LlmCostType, llm_review::context_state};

use super::slither_ffi::cache_key;

/// Global cache for file summaries to avoid redundant LLM calls
pub static FILE_SUMMARY_CACHE: Lazy<Arc<Mutex<HashMap<String, Vec<SrcFileSummary>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents a summary of a source file with metadata
#[derive(Default, Debug, Clone)]
pub struct SrcFileSummary {
    /// Source file name
    pub filename: String,
    /// AI-generated summary of the file's purpose and functionality
    pub summary: String,
}

/// Structured response format for LLM file summarization
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileSummary {
    /// The generated summary text
    pub summary: String,
}

pub async fn summarize_docs(
    repo: &RepoPaths,
    current_context: &str,
) -> Result<Vec<SrcFileSummary>> {
    let key = cache_key(&repo.root, "docs-summary");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        return Ok(cached.clone());
    }

    let documentation = repo.extract_content_from_docs()?;
    let mut doc_summaries = Vec::new();

    let mut docs_plus_context = format!("\n ## DOCUMENTATION: \n\n {}\n\n", documentation);
    docs_plus_context.push_str("\n ## CURRENT SECURITY AUDIT CONTEXT \n\n");
    docs_plus_context.push_str(&format!("\n #### The Documentation Summary should NOT contain content that is already included below.\n\n {} \n\n", current_context));

    let openai_client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);

    info!("generate summmary of all major files and docs in repo...");
    let preamble =
        "You are a senior solidity dev and expert solidity security researcher. Please provide detailed and comprehensive summary 
        of below DOCUMENTATION. Should be up to 4000 words, but no longer.  Should cover **all relevant details** that a security researcher 
        should know about this protocol to do a proper smart contract audit. ALSO, exclude any information from the summary that is already
        included in below CURRENT SECURITY AUDIT CONTEXT, because both DOCUMENTATION and CURRENT SECURITY AUDIT CONTEXT will be provide as
        context for an llm to do a security scan of protocol code.  So its important there is NO duplicate information between DOCUMENTATION 
        and CURRENT SECURITY AUDIT CONTEXT ";
    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(O3)
        .preamble(preamble)
        .build();

    add_to_inference_cost_by_type(
        &format!("{}{}", preamble, documentation),
        LlmCostType::OpenaiO3Input,
    )
    .await;

    info!("summarizing documentation");

    let doc_summary = match extractor_with_retry(
        &ai_summary_agent,
        &docs_plus_context,
        LlmCostType::OpenaiO3Output,
    )
    .await
    {
        Ok(res) => SrcFileSummary {
            filename: "readme.md".to_string(),
            summary: res.summary,
        },
        Err(e) => {
            log::error!("❌ summarizing readme.md failed: {e}");
            SrcFileSummary {
                filename: "readme.md".to_string(),
                summary: String::new(),
            }
        }
    };

    log::info!("readme.md summary => {:#?}", doc_summary);
    doc_summaries.push(doc_summary);

    summaries_cache.insert(key, doc_summaries.clone());
    Ok(doc_summaries)
}

pub async fn summarize_src_files(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<Vec<SrcFileSummary>> {
    let key = cache_key(&repo.root, "file_summaries");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        return Ok(cached.clone());
    }

    let openai_client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);

    let context =
        context_state::generate_slither_metadata_prompt_context(repo, &semantics_path).await?;

    // info!("slither metadata => {:#?}", context);
    info!("generate summmary of all major files and docs in repo...");
    let preamble ="You are a senior solidity dev. Please summarize below content (source code, tests, or deploy scripts). Format in markdown for easy reading. 
                    Please write 200 word or less summary for each contract plus contract definition, 100 words or less summary 
                    of each function + function interface, and 50 word or less explanation of each storage variable + variable defintion. If docs 
                    please summarize each section of the docs with 150 words or less, max 500 words total for each doc file. 
                    Respond only with valid JSON matching the schema!";
    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(GPT_4O)
        .preamble(preamble)
        .context(&context)
        .build();

    // ---------------------------------------------
    // 1.  PREP – collect the  files we want to summarize first
    // ---------------------------------------------
    let mut work_items = Vec::new();

    let protocol_root = repo.get_protocol_root();
    // Walk through the repository and collect relevant files
    for file in &repo.sol_files {
        let is_file_we_want_summary_of = (file.extension().map_or(false, |ext| ext == "sol")
            && file.starts_with(&repo.source_code_folder))
            || is_test_file(file, protocol_root.as_path())
            || is_script_file(file, protocol_root.as_path());

        if !is_file_we_want_summary_of {
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
        work_items.push((file.to_owned(), content));
    }

    let max_parallel = 20;
    let sem = Arc::new(Semaphore::new(max_parallel));
    let agent = Arc::new(ai_summary_agent); // the OpenAI client
    let mut handles = Vec::new();

    for (file, content) in work_items {
        let sem = sem.clone();
        let agent = agent.clone();
        let repo_root = repo.root.clone();

        let handle = tokio::spawn(async move {
            // acquire permit – blocks if `max_parallel` already in-flight
            let _permit = sem.acquire_owned().await.unwrap();

            add_to_inference_cost_by_type(
                &format!("{}{}", preamble, content),
                LlmCostType::Openai4oInput,
            )
            .await;

            info!("summarizing {}", file.display());

            match extractor_with_retry(&agent, &content, LlmCostType::Openai4oOutput).await {
                Ok(res) => {
                    let filename = file
                        .strip_prefix(&repo_root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .to_string();
                    Some(SrcFileSummary {
                        filename,
                        summary: res.summary,
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

    log::info!("summaries => {:#?}", summaries);

    summaries_cache.insert(key, summaries.clone());
    Ok(summaries)
}

pub async fn summarize_protocol(repo: &RepoPaths, context: Option<&str>) -> Result<String> {
    let key = cache_key(&repo.root, "protocol-summary");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        let summary = cached
            .first()
            .unwrap_or(&SrcFileSummary::default())
            .summary
            .clone();
        return Ok(summary);
    }

    // if no cached context is required
    let context = context.expect("if summary of protocol is not cached must provide context");

    // Initialize vectors to store file paths
    let mut summaries = Vec::<SrcFileSummary>::new();

    let openai_client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);

    log::info!("generate context for code review");
    let preamble= "You are a senior solidity dev. Given the context provided for solidity smart contract protocol, 
                   please create a max 200 word summary of this protocol explaining what it is, and how it works.  Format 
                   in markdown for easy reading. Respond only with valid JSON matching the schema!";

    let ai_summary_agent = openai_client
        .extractor::<FileSummary>("gpt-5")
        .preamble(preamble)
        .build();

    log::info!("extracting protocol summary");
    // rerun if NoDataExtracted Error

    add_to_inference_cost_by_type(
        &format!("{}{}", preamble, context),
        LlmCostType::Openai5Input,
    )
    .await;

    let summary =
        extractor_with_retry(&ai_summary_agent, &context, LlmCostType::Openai5Output).await?;

    log::info!("protocol summary => {:#?}", summary);

    summaries.push(SrcFileSummary {
        filename: "protocol-summary".to_string(),
        summary: summary.summary.clone(),
    });

    summaries_cache.insert(key, summaries.clone());
    Ok(summary.summary)
}
