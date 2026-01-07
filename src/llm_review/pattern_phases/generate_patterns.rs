/// Phase 2: Parallel vulnerability detection across multiple AI agents
///
/// This phase orchestrates parallel security analysis using multiple AI agents
/// to discover potential vulnerabilities in smart contracts.
use crate::{
    config::INVARIANT_RUNS,
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::context_state::{generate_audit_scope, get_metadata_context},
        dynamic_prompts::{
            invariants::{generate_invariant_prompt, get_invariant_json},
            prompt_index,
        },
        threat_models::issues::{IssuePrompt, IssueStructTrait},
    },
    prepare_code::git_clone::RepoPaths,
    reporting::save_file,
};
use log::info;
use serde::de::DeserializeOwned;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

/// Executes the findings generation phase
///
/// Runs parallel security analysis across multiple AI agents to discover
/// potential vulnerabilities in the provided smart contract code.
pub async fn execute<T>(
    issue_prompt: IssuePrompt,
    code: &str,
    arc_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<T>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
{
    let issue_title = match issue_prompt {
        IssuePrompt::Combined(_) => "vulnerability patterns",
        IssuePrompt::Invariant(_) => "invariants",
    };
    info!(
        "🔍 Phase 1: Generating {} from contract codebase...",
        issue_title
    );

    let mut handles = vec![];
    let all_patterns = Arc::new(Mutex::new(T::default()));

    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");

    let audit_scope = generate_audit_scope(repo).await?;
    let section_9_header = prompt_index::generated_section_header("ADDITIONAL CONTEXT", 9);
    let section_10_header = prompt_index::generated_section_header(
        "AUDIT SCOPE AND KEY INVARIANTS PROVIDED BY CLIENT",
        10,
    );

    let combined_context = if audit_scope.is_empty() {
        format!(
            r#"

{section_9_header}

{context}
"#
        )
    } else {
        format!(
            r#"

{section_9_header}

{context}

{section_10_header}

{audit_scope}
"#
        )
    };

    let codeblock = Arc::new(code.to_string());

    let added_content_from_brain = Arc::new(combined_context);
    // Note: `codeblock` already contains Section 8 with subsections 8.1-8.6
    let code_plus_context = format!(
        r#"

{codeblock}

{added_content_from_brain}
"#
    );

    // Simple local closure to DRY out spawn logic without extra generics
    let mut spawn_run = |prompt: Arc<String>, run_index: usize| {
        let agent = Arc::clone(arc_agent);
        let shared_patterns = Arc::clone(&all_patterns);

        handles.push(tokio::spawn(async move {
            if let Err(e) =
                run_security_prompt(agent, issue_title, prompt, run_index, shared_patterns).await
            {
                log::error!("Prompt task failed: {e:#}");
            }
        }));
    };

    match issue_prompt {
        IssuePrompt::Combined((_, _, _)) => {
            log::warn!("Scanning vulnerability patterns has been depreciated!")
        }
        IssuePrompt::Invariant(invariants) => {
            let inv_prompt = Arc::new(generate_invariant_prompt(&invariants));
            let json_requirement_prompt = Arc::new(get_invariant_json(&invariants));
            let prompt = Arc::new(format!(
                "{inv_prompt}{code_plus_context}{json_requirement_prompt}"
            ));

            save_file::save_file_locally(&prompt, &PathBuf::from("find_invariant_prompt.md"))?;
            // info!("invariant prompt => {}", prompt);
            for run in 0..INVARIANT_RUNS {
                spawn_run(Arc::clone(&prompt), run + 1);
            }
        }
    }

    // Wait for ALL tasks to complete
    for handle in handles {
        handle.await?; // Will error if task panicked
    }

    let findings = all_patterns.lock().await;

    if !findings.issues().is_empty() {
        info!(
            "✅ Phase 1 complete: {} {} BEFORE deduping",
            issue_title,
            findings.issues().len()
        );
    }
    Ok(findings.clone())
}

/// Executes one LLM-prompt round and merges the returned findings into the shared findings collection
///
/// This function handles individual security analysis rounds, managing prompt generation,
/// LLM interaction, and result aggregation.
pub async fn run_security_prompt<T>(
    agent: Arc<AIAgent>,
    title: &str,
    prompt: Arc<String>,
    idx_of_review_round: usize,
    shared_patterns: Arc<Mutex<T>>,
) -> Result<()>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
{
    // 2. Send to the right provider
    info!(
        "---- #{} LLM analysis Round for Finding {}----",
        idx_of_review_round, title
    );
    let patterns: T = agent.extract_with_retry(&prompt).await?;

    let issues_found = patterns.issues().len();
    info!("{} {} found!", issues_found, title);

    // 3. Merge results (if any) into the shared accumulator
    if issues_found > 0 {
        let mut guard = shared_patterns.lock().await;
        guard.issues_mut().extend_from_slice(patterns.issues());
    }

    Ok(())
}

/// Generates the combined content and context block for LLM analysis
///
/// Combines the contract code with additional context information
/// in a structured format for optimal LLM processing.
pub fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let section_9_header = prompt_index::generated_section_header("ADDITIONAL CONTEXT", 9);

    format!(
        r#"
{codeblock}

{section_9_header}

{added_context}
"#
    )
}
