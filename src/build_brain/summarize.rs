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
use std::sync::Arc;
use strum_macros::EnumString;
use tokio::sync::Semaphore;

use crate::utils::check_folder_name::is_script_file;
use crate::{
    build_brain::{
        slither_ffi::get_all_files_src,
        summarize_db::{
            get_file_summary_from_db, get_summaries_from_db, insert_file_summaries_to_db,
            insert_file_summary_to_db,
        },
    },
    cost::cost_data::{TokenType, add_to_inference_cost_by_type},
    llm_review::{
        agent::agent_enums::{AgentMetadata, all_enum_variants, generate_enum_list},
        contract::contract_category::{
            ContractCategory, generate_formated_list_of_contract_categories,
        },
    },
    prepare_code::git_clone::RepoPaths,
    utils::{contract_name_check::has_non_mock_contract, extract_retry::extractor_with_retry},
};

/// Check if a file path contains standard library folders that should be excluded from summarization.
/// Returns true if the file is a standard library file (should be excluded).
fn is_standard_library_file(file_path: &std::path::Path) -> bool {
    let path_str = file_path.to_string_lossy().to_lowercase();

    // Exclude node_modules entirely
    if path_str.contains("node_modules") {
        return true;
    }

    // Exclude test folders (contains mocks and test contracts)
    if path_str.contains("/test/") || path_str.contains("/tests/") {
        return true;
    }

    // Exclude audit folders (contains flattened contracts, PoCs, and audit artifacts)
    if path_str.contains("/audit/") || path_str.contains("/audits/") {
        return true;
    }

    // Common standard library patterns in /lib/ folders
    let standard_lib_patterns = [
        // Foundry standard libraries
        "/lib/forge-std/",
        "/lib/ds-test/",
        // OpenZeppelin
        "/lib/openzeppelin-contracts/",
        "/lib/openzeppelin-contracts-upgradeable/",
        "/lib/@openzeppelin/",
        // Solmate
        "/lib/solmate/",
        // Solady
        "/lib/solady/",
        // PRBMath
        "/lib/prb-math/",
        "/lib/prb-test/",
        // Chainlink
        "/lib/chainlink/",
        "/lib/chainlink-brownie-contracts/",
        // Uniswap
        "/lib/v2-core/",
        "/lib/v2-periphery/",
        "/lib/v3-core/",
        "/lib/v3-periphery/",
        "/lib/uniswap-v2-core/",
        "/lib/uniswap-v2-periphery/",
        "/lib/uniswap-v3-core/",
        "/lib/uniswap-v3-periphery/",
        "/lib/uniswapv2/",
        "/lib/zuniswapv2/",
        // Polygon/FX Portal
        "/lib/fx-portal/",
        // Gnosis Safe
        "/lib/safe-contracts/",
        "/lib/safe-smart-account/",
        // ERC standards
        "/lib/erc721a/",
        "/lib/erc1155/",
        "/lib/erc4626/",
        // Testing libraries
        "/lib/weird-erc20/",
        // Common utilities
        "/lib/create2-helpers/",
        "/lib/multicall/",
        "/lib/permit2/",
        // Aave
        "/lib/aave-v3-core/",
        "/lib/aave-v3-periphery/",
        // Compound
        "/lib/compound-protocol/",
        // Curve
        "/lib/curve-contract/",
        // Balancer
        "/lib/balancer-v2-monorepo/",
        // Maker
        "/lib/dss/",
        // Synthetix
        "/lib/synthetix/",
        // Yearn
        "/lib/yearn-vaults/",
        // Sushiswap
        "/lib/sushiswap/",
        // 1inch
        "/lib/1inch/",
        // 0x
        "/lib/0x-monorepo/",
        // Optimism
        "/lib/optimism/",
        // Arbitrum
        "/lib/arbitrum/",
        "/lib/nitro-contracts/",
        // zkSync
        "/lib/zksync/",
        "/lib/era-contracts/",
        // LayerZero
        "/lib/layerzero/",
        "/lib/solidity-examples/",
        // Axelar
        "/lib/axelar-gmp-sdk-solidity/",
        // Wormhole
        "/lib/wormhole/",
        // Hyperlane
        "/lib/hyperlane-monorepo/",
        // Connext
        "/lib/nxtp/",
        // Common test helpers
        "/lib/test/",
        "/lib/testing/",
        // Hardhat plugins
        "/lib/hardhat-deploy/",
        // Other common libraries
        "/lib/clones-with-immutable-args/",
        "/lib/create3-factory/",
        "/lib/erc4337/",
        "/lib/account-abstraction/",
        "/lib/seaport/",
        "/lib/murky/",
        "/lib/ens-contracts/",
        "/lib/canonical-weth/",
    ];

    // Check if path contains any standard library pattern
    standard_lib_patterns
        .iter()
        .any(|pattern| path_str.contains(pattern))
}

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
    pub contract_category: Option<ContractCategory>,
    pub file_type: Option<FileSummaryType>,
}

/// Structured response format for LLM file summarization
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileSummary {
    /// The generated summary text
    pub summary: String,
    pub contract_category: ContractCategory,
}

pub const MAX_WORDS_CONTRACT_SUMMARY: u16 = 100;
pub const MAX_WORDS_FUNCTION_SUMMARY: u16 = 20;
pub const MAX_CHARS_STORAGE_DESC: u16 = 20;
// pub const MAX_WORDS_CONTRACT_SUMMARY: u16 = 40;
// pub const MAX_WORDS_FUNCTION_SUMMARY: u16 = 10;
// pub const MAX_CHARS_STORAGE_DESC: u16 = 16;

pub async fn summarize_src_files(repo: &RepoPaths) -> Result<Vec<SrcFileSummary>> {
    summarize_src_files_with_model(repo, "gpt-5").await
}

pub async fn summarize_src_files_with_model(
    repo: &RepoPaths,
    model: &str,
) -> Result<Vec<SrcFileSummary>> {
    // pull summaries from db if avaliable
    let summaries = get_summaries_from_db(repo)?;
    if !summaries.is_empty() {
        info!("retrived summaries from db");
        return Ok(summaries);
    }
    info!("generating file summaries...");

    let openai_client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);

    let mut context = String::new();
    let src_file_list = get_all_files_src(repo)?;

    // prompt_context.push_str("\n## Slither Contract Summary\n");
    // prompt_context.push_str(&contract_summary);
    context.push_str("\n## Main List of Files in Project\n\n");
    context.push_str(&src_file_list);
    context.push_str("\n\n");

    // add docs to context
    let documentation = repo.extract_content_from_docs()?;
    context.push_str("\n ## DOCUMENTATION: \n\n ");
    context.push_str(&documentation);

    let contract_category_enum_list =
        generate_enum_list(all_enum_variants::<ContractCategory>().as_slice());

    let contract_category_descriptions = generate_formated_list_of_contract_categories(
        all_enum_variants::<ContractCategory>().as_slice(),
    );

    info!("generate summmary of all major files and docs in repo...");
    let preamble_source_summary = format!(
        r#"You are a senior solidity dev. 

        ## Tasks


        1. Please summarize below source code. Format in markdown for easy reading. Start with a {MAX_WORDS_CONTRACT_SUMMARY} word or less summary of the contract, that includes  purpose trust model (user funds? admin?), also major entrypoints. Then list storage vars plus optional {MAX_CHARS_STORAGE_DESC} max chars description for each. For EACH function provide full interface; it should include visibility, modifiers, and mutability. Adjacent to function interface, add {MAX_WORDS_FUNCTION_SUMMARY} word max natspec for EACH function. Respond only with valid JSON matching the schema!

        2. Determine which Category the contract falls into from the list below:
        {contract_category_descriptions}

        ## DELIVERABLES
        1. summary of code
        2. Contract Category, pick one: {contract_category_enum_list}

        "#
    );
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
        .extractor::<FileSummary>(model)
        .preamble(&preamble_source_summary)
        .context(&context)
        .build();

    let ai_deploy_summary_agent = openai_client
        .extractor::<FileSummary>(model)
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

        // Skip standard library files (node_modules, OpenZeppelin, Forge-std, etc.)
        if is_standard_library_file(file) {
            continue;
        }

        // check if lib folder or test files
        if file.to_string_lossy().contains(".t.sol") {
            continue;
        }

        // check if file is in scope for summary (source or script) for foundry projects
        if file.extension().map_or(false, |ext| ext == "sol")
            && !file
                .file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |n| n.ends_with(".t.sol"))
        {
            if is_script_file(file) {
                current_file_summary_type = FileSummaryType::DeployScript;
            } else if repo.source_code_folders.iter().any(|f| file.starts_with(f)) {
                info!("adding {} to summary stack", file.display());
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

    let summary_count = work_items
        .iter()
        .filter(|(_, _, file_type)| *file_type == FileSummaryType::Source)
        .count();

    info!("summarizing {} files...", summary_count);
    if summary_count > 200 {
        panic!("over 200 files are going to be summarized, please check if this is correct!");
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
                        contract_category: Some(res.contract_category),
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
        info!(
            "contract category: {}",
            summary.contract_category.unwrap_or_default().to_string()
        );
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
