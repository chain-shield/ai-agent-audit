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
        summarize::{self, summarize_protocol},
    },
    prepare_code::git_clone::RepoPaths,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextType {
    Full,
    Abridged,
}

/// Global metadata context shared across all AI agents

pub static PROMPT_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub static METADATA_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn get_context_key(repo: &RepoPaths, context_type: &ContextType) -> String {
    match context_type {
        ContextType::Full => cache_key(&repo.root, "prompt_context"),
        ContextType::Abridged => cache_key(&repo.root, "abridged_prompt_context"),
    }
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
    let full_context =
        generate_context_for_code_review(repo, semantics_path, &ContextType::Full).await?;
    let abridged_context =
        generate_context_for_code_review(repo, semantics_path, &ContextType::Abridged).await?;
    let protocol_summary = summarize_protocol(repo, Some(&full_context)).await?;

    metadata.push_str(&format!(
        "\n## PROTOCOL OVERVIEW:\n\n{}\n\n",
        protocol_summary
    ));

    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let mut metadata_cache = metadata_context.lock().await;

    // saving full context
    metadata_cache.insert(
        get_context_key(repo, &ContextType::Full),
        format!("{}{}", metadata, full_context),
    );

    // saving abridge context
    metadata_cache.insert(
        get_context_key(repo, &ContextType::Abridged),
        format!("{}{}", metadata, abridged_context),
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
pub async fn get_metadata_context(repo: &RepoPaths, context_type: &ContextType) -> Option<String> {
    let key = get_context_key(repo, context_type);
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let metadata_cache = metadata_context.lock().await;
    // Return cached output if exists
    metadata_cache.get(&key).cloned()
}

pub async fn generate_context_for_code_review(
    repo: &RepoPaths,
    semantics_path: &Path,
    context_type: &ContextType,
) -> Result<String> {
    log::info!("generate slither metadata");
    let slither_metadata = generate_slither_metadata_prompt_context(repo, &semantics_path).await?;
    log::info!("generate summary of all files");

    let mut full_prompt_context = String::new();

    let mut file_summaries = String::new();
    if *context_type == ContextType::Full {
        let summaries = summarize::summarize_src_files(repo, &semantics_path).await?;
        for summary in summaries {
            file_summaries.push_str(&format!("\n## SUMMARY OF FILE: {}\n", summary.filename));
            file_summaries.push_str(&summary.summary);
            file_summaries.push_str("\n\n");
        }
        full_prompt_context.push_str(&file_summaries);
        full_prompt_context.push_str(&slither_metadata);
    }

    // let docs = summarize::summarize_docs(repo, &full_prompt_context).await?;
    let documentation = repo.extract_content_from_docs()?;
    // let mut doc_summaries = String::new();
    // for doc_summary in &docs {
    //     doc_summaries.push_str("\n\n");
    //     doc_summaries.push_str(&doc_summary.summary);
    //     doc_summaries.push_str("\n\n");
    // }
    // adding FULL DOCS not doc_summaries
    full_prompt_context.push_str("\n ## DOCUMENTATION: \n\n ");
    full_prompt_context.push_str(&documentation);

    if *context_type == ContextType::Full {
        let config_files_content = repo.extract_content_from_config_files()?;
        // adding config files: foundry.toml, package.json, etc
        full_prompt_context.push_str("\n ## CONFIG FILES: \n\n ");
        full_prompt_context.push_str(&config_files_content);
    }

    log::info!("documentation full size => {}", documentation.len());
    match context_type {
        ContextType::Full => {
            log::info!("full prompt context SIZE => {}", full_prompt_context.len());
        }
        ContextType::Abridged => {
            log::info!(
                "abridge prompt context SIZE => {}",
                full_prompt_context.len()
            );
        }
    }

    Ok(full_prompt_context)
}

pub async fn generate_audit_scope(repo: &RepoPaths) -> Result<String> {
    let key = cache_key(&repo.root, "audit_scope");

    // Return cached output if exists
    if let Some(cached) = METADATA_CONTEXT.lock().await.get(&key).cloned() {
        return Ok(cached);
    }

    let audit_scope = repo.extract_content_from_scope_file()?;

    // 1 . gather IR + storage  (re-use existing function)
    log::info!("get audit scope from file...");
    log::info!("audit scope size => {}", audit_scope.len());

    let cache = Arc::clone(&METADATA_CONTEXT);
    let mut context_cache = cache.lock().await;
    context_cache.insert(key, audit_scope.clone());
    Ok(audit_scope)
}

pub async fn generate_slither_metadata_prompt_context(
    repo: &RepoPaths,
    _semantics_path: &Path,
) -> Result<String> {
    let key = cache_key(&repo.root, "prompt_context");
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
        "slither metadata prompt context size ==> {}",
        prompt_context.len()
    );

    let cache = Arc::clone(&PROMPT_CONTEXT);
    let mut context_cache = cache.lock().await;
    context_cache.insert(key, prompt_context.clone());
    Ok(prompt_context)
}
