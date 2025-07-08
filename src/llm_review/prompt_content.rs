use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::build_brain::slither_ffi::{cache_key, get_all_files_src, run_printer};
use crate::build_brain::summarize;
use crate::prepare_code::git_clone::RepoPaths;

use super::config::Finding;

/// Global cache keyed by (repo_root, printer) tuple stringified
pub static PROMPT_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub async fn generate_slither_metadata_prompt_context(
    repo: &RepoPaths,
    _semantics_path: &Path,
) -> Result<String> {
    let key = cache_key(&repo.root, "prompt_context");
    let cache = Arc::clone(&PROMPT_CONTEXT);
    let mut context_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = context_cache.get(&key) {
        return Ok(cached.clone());
    }

    // 1 . gather IR + storage  (re-use existing function)
    info!("get contract summary and source files");
    // let callgraph = callgraph::get_enriched_funcs_and_edges(repo_root, &semantics_path).await?;
    // let inheritance = inheritance::generate_slither_inheritance(repo_root).await?;
    let contract_summary = run_printer(repo, "contract-summary").await?;
    let src_file_list = get_all_files_src(repo);

    let mut prompt_context = String::new();

    prompt_context.push_str("\n## Slither Contract Summary\n");
    prompt_context.push_str(&contract_summary);
    prompt_context.push_str("\n## List of Files in Src Folder\n");
    prompt_context.push_str(&src_file_list);
    // prompt_context.push_str("\n## Slither Call Graph\n");
    // prompt_context.push_str(&callgraph);
    // prompt_context.push_str("\n## Slither Inheritance Json\n");
    // prompt_context.push_str(&inheritance);
    // prompt_context.push_str("\n## Slither Detector\n");
    // prompt_context.push_str(&slither_scan_results);

    info!(
        "slither metadata prompt context size ==> {}",
        prompt_context.len()
    );
    context_cache.insert(key, prompt_context.clone());
    Ok(prompt_context)
}

pub async fn generate_context_for_code_review(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<String> {
    log::info!("generate slither metadata");
    let slither_metadata = generate_slither_metadata_prompt_context(repo, &semantics_path).await?;
    log::info!("generate summary of all files");
    let summaries = summarize::summarize_src_files(repo, &semantics_path).await?;
    let mut file_summaries = String::new();

    for summary in summaries {
        file_summaries.push_str(&format!("\n## {} summary\n", summary.filename));
        file_summaries.push_str(&summary.summary);
        file_summaries.push_str("\n\n");
    }

    let mut full_prompt_context = String::new();
    full_prompt_context.push_str(&slither_metadata);
    full_prompt_context.push_str(&file_summaries);

    info!("full prompt context SIZE => {}", full_prompt_context.len());

    Ok(full_prompt_context)
}

pub fn generate_prompt_for_issue_check(
    code: &str,
    finding: &Finding,
    pre_instructions: &str,
    instructions: &str,
    post_instructions: &str,
) -> String {
    let mut prompt = format!("{}{}{}", pre_instructions, instructions, post_instructions);

    prompt.push_str("\n\n");
    prompt.push_str("## REPORT FOR SECURITY ISSUE");
    prompt.push_str("\n\n");

    let report = get_finding_report(finding);
    prompt.push_str(&report);
    prompt.push_str("\n\n");

    prompt.push_str("## CODEBASE WHERE ISSUE WAS FOUND");
    prompt.push_str("\n\n");

    prompt.push_str(code);

    prompt
}

fn get_finding_report(finding: &Finding) -> String {
    let mut findings_report = String::new();
    //title
    findings_report.push_str(&format!(
        "## [Severity-{}]. {}\n\n",
        finding.severity.as_str(),
        finding.title()
    ));

    //description
    findings_report.push_str("## Description\n");
    findings_report.push_str(&finding.description.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //impact
    findings_report.push_str("## Impact\n");
    findings_report.push_str(&finding.impact.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //POC
    findings_report.push_str("## Proof of Concept\n");
    findings_report.push_str(&finding.proof_of_concept.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Proof of Code
    findings_report.push_str("## Proof of Code\n");
    findings_report.push_str(&finding.proof_of_code.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Suggested Fix
    findings_report.push_str("## Suggested Mitigation\n");
    findings_report.push_str(&finding.mitigation.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    findings_report.push_str("\n");
    findings_report
}
