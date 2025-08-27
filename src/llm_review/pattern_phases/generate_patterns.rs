/// Phase 2: Parallel vulnerability detection across multiple AI agents
///
/// This phase orchestrates parallel security analysis using multiple AI agents
/// to discover potential vulnerabilities in smart contracts.
use crate::{
    config::INVARIANT_RUNS,
    cost::cost_data::{add_to_inference_cost_by_type, LlmCostType},
    error::Result,
    llm_review::{
        context_state::{get_metadata_context, ContextType},
        dynamic_prompts::{self, invariants::generate_invariant_prompt},
        enums::AIAgent,
        issues::{IssuePrompt, IssueStructTrait},
        pattern_category::get_category_library_spec,
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;
use serde::de::DeserializeOwned;
use std::sync::Arc;
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
        IssuePrompt::Pattern(_) => "vulnerability patterns",
        IssuePrompt::Invariant(_) => "invariants",
    };
    info!(
        "🔍 Phase 1: Generating {} from contract codebase...",
        issue_title
    );

    let mut handles = vec![];
    let all_patterns = Arc::new(Mutex::new(T::default()));

    let context = get_metadata_context(repo, &ContextType::Full)
        .await
        .expect("could not extract context");

    let codeblock = Arc::new(code.to_string());

    let added_content_from_brain = Arc::new(context.to_string());

    // Simple local closure to DRY out spawn logic without extra generics
    let mut spawn_run = |prompt: Arc<String>, run_index: usize| {
        let agent = Arc::clone(arc_agent);
        let combined = Arc::clone(&all_patterns);
        let code = Arc::clone(&codeblock);
        let ctx = Arc::clone(&added_content_from_brain);
        handles.push(tokio::spawn(async move {
            if let Err(e) = run_security_prompt(agent, code, ctx, prompt, run_index, combined).await
            {
                log::error!("Prompt task failed: {e:#}");
            }
        }));
    };

    match issue_prompt {
        IssuePrompt::Pattern(pattern_category) => {
            for (i, category) in pattern_category.into_iter().enumerate() {
                let category_spec =
                    get_category_library_spec(&category).expect("could not extract category spec");
                let category_prompt = Arc::new(
                    dynamic_prompts::patterns::generate_pattern_category_prompt(&category),
                );

                for run in 0..category_spec.runs {
                    spawn_run(Arc::clone(&category_prompt), (run + 1) * (i + 1));
                }
            }
        }
        IssuePrompt::Invariant(invariants) => {
            let inv_prompt = Arc::new(generate_invariant_prompt(&invariants));
            for run in 0..INVARIANT_RUNS {
                spawn_run(Arc::clone(&inv_prompt), run + 1);
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
    code: Arc<String>,
    added_context: Arc<String>,
    instructions: Arc<String>,
    idx_of_review_round: usize,
    shared_patterns: Arc<Mutex<T>>,
) -> Result<()>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
{
    let prompt_body = generate_content_plus_context_block(code.as_str(), added_context.as_str());
    let full_prompt = format!("{instructions}{prompt_body}");
    // info!("prompt instructions:\n\n {}", instructions);

    // add to cost
    add_to_inference_cost_by_type(&full_prompt, LlmCostType::Openai5Input).await;

    // 2. Send to the right provider
    info!("----LLM analysis Round #{}----", idx_of_review_round);
    let patterns: T = agent.extract_with_retry(&full_prompt).await?;

    let issues_found = patterns.issues().len();
    info!("{} issues found!", issues_found);

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
fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\nSOLIDITY CONTRACT + STORAGE TO CODE REVIEW\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context.push_str("\n\n ## ADDITIONAL CONTEXT \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}
