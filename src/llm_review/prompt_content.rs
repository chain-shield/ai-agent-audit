use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::build_brain::slither_ffi::{cache_key, get_all_files_src, run_printer};
use crate::build_brain::{callgraph, inheritance, summarize};

/// Global cache keyed by (repo_root, printer) tuple stringified
pub static PROMPT_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub async fn generate_slither_metadata_prompt_context(
    repo_root: &Path,
    semantics_path: &Path,
) -> Result<String> {
    let key = cache_key(repo_root, "prompt_context");
    let cache = Arc::clone(&PROMPT_CONTEXT);
    let mut context_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = context_cache.get(&key) {
        return Ok(cached.clone());
    }

    // 1 . gather IR + storage  (re-use existing function)
    info!("get ir and storage chunks");
    // let slither_scan_results = run_slither_sarif(repo_root)?;
    let callgraph = callgraph::get_enriched_funcs_and_edges(repo_root, &semantics_path).await?;
    let inheritance = inheritance::generate_slither_inheritance(repo_root).await?;
    let contract_summary = run_printer(repo_root, "contract-summary").await?;
    let src_file_list = get_all_files_src(repo_root).await?;

    let mut prompt_context = String::new();

    prompt_context.push_str("\n## Slither Contract Summary\n");
    prompt_context.push_str(&contract_summary);
    prompt_context.push_str("\n## List of Files in Src Folder\n");
    prompt_context.push_str(&src_file_list);
    prompt_context.push_str("\n## Slither Call Graph\n");
    prompt_context.push_str(&callgraph);
    prompt_context.push_str("\n## Slither Inheritance Json\n");
    prompt_context.push_str(&inheritance);
    // prompt_context.push_str("\n## Slither Detector\n");
    // prompt_context.push_str(&slither_scan_results);

    info!(
        "slither metadata prompt context size ==> {}",
        prompt_context.len()
    );
    context_cache.insert(key, prompt_context.clone());
    Ok(prompt_context)
}

pub async fn generate_abridged_slither_metadata_prompt_context(
    repo_root: &Path,
    semantics_path: &Path,
) -> Result<String> {
    let key = cache_key(repo_root, "abridged_prompt_context");
    let cache = Arc::clone(&PROMPT_CONTEXT);
    let mut context_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = context_cache.get(&key) {
        return Ok(cached.clone());
    }

    // 1 . gather IR + storage  (re-use existing function)
    info!("get ir and storage chunks");
    let inheritance = inheritance::generate_slither_inheritance(repo_root).await?;
    let callgraph = callgraph::get_enriched_funcs_and_edges(repo_root, &semantics_path).await?;
    let contract_summary = run_printer(repo_root, "contract-summary").await?;
    let src_file_list = get_all_files_src(repo_root).await?;

    let mut prompt_context = String::new();

    prompt_context.push_str("\n## Slither Contract Summary\n");
    prompt_context.push_str(&contract_summary);
    prompt_context.push_str("\n## List of Files in Src Folder\n");
    prompt_context.push_str(&src_file_list);
    prompt_context.push_str("\n## Slither Call Graph\n");
    prompt_context.push_str(&callgraph);
    prompt_context.push_str("\n## Slither Inheritance Json\n");
    prompt_context.push_str(&inheritance);

    info!(
        "slither metadata (abridged) prompt context size ==> {}",
        prompt_context.len()
    );
    context_cache.insert(key, prompt_context.clone());
    Ok(prompt_context)
}

pub async fn generate_context_for_code_review(
    repo_root: &Path,
    semantics_path: &Path,
) -> Result<String> {
    let slither_metadata =
        generate_abridged_slither_metadata_prompt_context(repo_root, &semantics_path).await?;
    let summaries = summarize::summarize_src_files(repo_root, &semantics_path).await?;
    let mut file_summaries = String::new();

    for summary in summaries {
        file_summaries.push_str(&format!("\n## {} summary\n", summary.filename));
        file_summaries.push_str(&summary.summary);
        file_summaries.push_str("\n\n");
    }

    let mut full_prompt_context = String::new();
    full_prompt_context.push_str(&slither_metadata);
    full_prompt_context.push_str(&file_summaries);

    info!("full prompt content SIZE => {}", full_prompt_context.len());
    Ok(full_prompt_context)
}
