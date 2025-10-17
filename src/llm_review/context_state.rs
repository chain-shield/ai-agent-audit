use anyhow::Result;
/// Global context state management for AI analysis.
///
/// This module manages shared protocol metadata context that is generated once
/// and reused across all AI agents for consistent analysis. Provides thread-safe
/// access to protocol information including summaries and semantic data.
use once_cell::sync::Lazy;
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    build_brain::{
        slither_ffi::{cache_key, get_all_files_src},
        summarize::{summarize_protocol, summarize_src_files, FileSummaryType},
    },
    cost::cost_data::get_token_count,
    prepare_code::git_clone::RepoPaths,
};

/// Global metadata context shared across all AI agents

pub static PROMPT_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub static METADATA_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn get_context_key(repo: &RepoPaths) -> String {
    cache_key(&repo.root, "prompt_context", None)
}
/// Generates and caches protocol metadata context for AI analysis.
///
/// This function creates comprehensive context information including protocol
/// summaries, contract relationships, and semantic data that is shared across
/// all AI agents for consistent analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_path` - Path to semantic analysis database
pub async fn generate_and_save_metadata_context(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> anyhow::Result<()> {
    let mut metadata = String::new();

    // full context
    let full_context = generate_context_for_code_review(repo, semantics_path).await?;

    let summaries = summarize_src_files(repo, semantics_path).await?;

    // add file summaries to context summary
    let mut full_context_plus_summaries = full_context.clone();
    full_context_plus_summaries.push_str("\n## SUMMARY OF SOURCE CODE FILES\n\n");
    for summary in summaries {
        let file_type = summary.file_type.unwrap_or(FileSummaryType::OutOfScope);
        if file_type == FileSummaryType::Source {
            full_context_plus_summaries.push_str(&format!("### Summary of {}\n", summary.filename));
            full_context_plus_summaries.push_str(&summary.summary);
            full_context_plus_summaries.push_str("\n");
        }
    }

    let protocol_summary = summarize_protocol(repo, Some(&full_context_plus_summaries)).await?;

    metadata.push_str(&format!(
        "\n## PROTOCOL OVERVIEW:\n\n{}\n\n",
        protocol_summary
    ));

    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let mut metadata_cache = metadata_context.lock().await;

    // saving full context
    metadata_cache.insert(
        get_context_key(repo),
        format!("{}{}", metadata, full_context),
    );

    Ok(())
}

/// Retrieves the cached metadata context for AI analysis.
///
/// Returns the protocol metadata context that was previously generated and
/// cached for use across all AI agents.
///
/// # Returns
/// * `String` - Cached protocol metadata context
pub async fn get_metadata_context(repo: &RepoPaths) -> Option<String> {
    let key = get_context_key(repo);
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let metadata_cache = metadata_context.lock().await;
    // Return cached output if exists
    metadata_cache.get(&key).cloned()
}

pub async fn generate_context_for_code_review(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<String> {
    log::info!("generate slither metadata");
    let slither_metadata = generate_slither_metadata_prompt_context(repo, &semantics_path).await?;
    log::info!("generate summary of all files");

    let mut full_prompt_context = String::new();

    // NOTE: now excluding file summarizes from metadata
    // let mut file_summaries = String::new();
    // let summaries = summarize::summarize_src_files(repo, &semantics_path).await?;
    // for summary in summaries {
    //     let file_type = summary.file_type.unwrap_or(FileSummaryType::OutOfScope);
    //     if file_type == FileSummaryType::DeployScript {
    //         file_summaries.push_str(&format!(
    //             "\n## SUMMARY OF DEPLOY SCRIPT: {}\n",
    //             summary.filename
    //         ));
    //         file_summaries.push_str(&summary.summary);
    //         file_summaries.push_str("\n\n");
    //     }
    // }
    // full_prompt_context.push_str(&file_summaries);
    full_prompt_context.push_str(&slither_metadata);

    // let docs = summarize::summarize_docs(repo, &full_prompt_context).await?;
    let documentation = repo.extract_content_from_docs()?;
    // adding FULL DOCS not doc_summaries
    full_prompt_context.push_str("\n ## DOCUMENTATION: \n\n ");
    full_prompt_context.push_str(&documentation);

    let lib_config_headers = repo.extract_lib_config_headers()?;
    full_prompt_context.push_str("\n ## PACKAGE.JSON HEADERS OF LIB PACKAGES: \n");
    full_prompt_context.push_str("\n Note: Check for important lib version info\n\n ");
    full_prompt_context.push_str("\n When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.\n\n ");
    full_prompt_context.push_str(&lib_config_headers);

    let config_files_content = repo.extract_content_from_config_files()?;
    // adding config files: foundry.toml, package.json, etc
    full_prompt_context.push_str("\n ## CONFIG FILES: \n");
    full_prompt_context.push_str("\n Note: Check for important package version info.\n\n ");
    full_prompt_context.push_str(&config_files_content);

    log::info!(
        "documentation full token count => {}",
        get_token_count(&documentation)
    );
    log::info!(
        "prompt context token count => {}",
        get_token_count(&full_prompt_context)
    );

    Ok(full_prompt_context)
}

pub async fn generate_audit_scope(repo: &RepoPaths) -> Result<String> {
    let key = cache_key(&repo.root, "audit_scope", None);

    // Return cached output if exists
    if let Some(cached) = METADATA_CONTEXT.lock().await.get(&key).cloned() {
        return Ok(cached);
    }

    let mut audit_scope = "#r 

        ## Privileged Roles 

        All Privileged Roles are TRUSTED by default unless listed as untrusted below.

        Errors and misuse committed by admin (or any other privileged role) are considered
        **governance risk, NOT vulnerabilities**.  They will get marked as Low or Informational.

        *Caveat*: If the admin can accidentally brick the protocol even while following spec (no malice or error) 
        — that can rise to Medium. 
        Example: a valid function like updateFee() can unintentionally revert all 
        deposits if called with a certain boundary value, even though the admin followed expected usage.
#".to_string();

    let protocol_specific_audit_scope = repo.extract_content_from_scope_file()?;

    audit_scope.push_str(&format!("\n\n {}", protocol_specific_audit_scope));

    // 1 . gather IR + storage  (re-use existing function)
    log::info!("get audit scope from file...");
    log::info!(
        "audit scope token count => {}",
        get_token_count(&audit_scope)
    );

    let cache = Arc::clone(&METADATA_CONTEXT);
    let mut context_cache = cache.lock().await;
    context_cache.insert(key, audit_scope.clone());
    Ok(audit_scope)
}

pub async fn generate_slither_metadata_prompt_context(
    repo: &RepoPaths,
    _semantics_path: &Path,
) -> Result<String> {
    let key = cache_key(&repo.root, "prompt_context", None);
    // Return cached output if exists
    // Return cached output if exists
    if let Some(cached) = PROMPT_CONTEXT.lock().await.get(&key).cloned() {
        return Ok(cached);
    }

    // 1 . gather IR + storage  (re-use existing function)
    log::info!("get contract summary and source files");
    // let callgraph = callgraph::get_enriched_funcs_and_edges(repo_root, &semantics_path).await?;
    // let inheritance = inheritance::generate_slither_inheritance(repo_root).await?;
    // let contract_summary = run_printer(repo, "contract-summary").await?;
    let src_file_list = get_all_files_src(repo)?;
    log::info!("src file list => {}", src_file_list);

    let mut prompt_context = String::new();

    // prompt_context.push_str("\n## Slither Contract Summary\n");
    // prompt_context.push_str(&contract_summary);
    prompt_context.push_str("\n## Main List of Files in Project\n\n");
    prompt_context.push_str(&src_file_list);
    prompt_context.push_str("\n\n");
    // prompt_context.push_str("\n## Slither Call Graph\n");
    // prompt_context.push_str(&callgraph);
    // prompt_context.push_str("\n## Slither Inheritance Json\n");
    // prompt_context.push_str(&inheritance);
    // prompt_context.push_str("\n## Slither Detector\n");
    // prompt_context.push_str(&slither_scan_results);

    log::info!(
        "file list context token count ==> {}",
        get_token_count(&prompt_context)
    );

    let cache = Arc::clone(&PROMPT_CONTEXT);
    let mut context_cache = cache.lock().await;
    context_cache.insert(key, prompt_context.clone());
    Ok(prompt_context)
}
