/// Phase 2: Parallel vulnerability detection across multiple AI agents
///
/// This phase orchestrates parallel security analysis using multiple AI agents
/// to discover potential vulnerabilities in smart contracts.
use crate::{
    config::{AuditType, AUDIT_TYPE, DISCOVERY_RUNS},
    error::Result,
    llm_review::{
        context_state::{generate_audit_scope, get_metadata_context, ContextType},
        enums::AIAgent,
        findings::{generated_llm_prompt, Findings},
        prompt_support::{post_prompt::generate_post_prompt, pre_prompt::generate_pre_prompt},
    },
    master_prompts::{code4rena::CODE4RENA_PROMPT, prompt_2x_aa::PROMPT_2X_AA},
    prepare_code::git_clone::RepoPaths,
};
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Executes the findings generation phase
///
/// Runs parallel security analysis across multiple AI agents to discover
/// potential vulnerabilities in the provided smart contract code.
pub async fn execute(
    contract: &str,
    code: &str,
    // agents: &Vec<Arc<AIAgent>>,
    arc_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 1: Generating findings from contract codebase...");

    let mut handles = vec![];
    let all_findings = Arc::new(Mutex::new(Findings {
        findings: Vec::new(),
    }));

    let context = get_metadata_context(repo, &ContextType::Full)
        .await
        .expect("could not extract context");

    let audit_scope = generate_audit_scope(repo).await?;

    let contract = Arc::new(contract.to_string());
    let codeblock = Arc::new(code.to_string());

    let added_content_from_brain = Arc::new(context.to_string());

    let llm_instructions = match AUDIT_TYPE {
        AuditType::Code4rena | AuditType::Sherlock => CODE4RENA_PROMPT,
        _ => PROMPT_2X_AA,
    };

    // for (run, arc_agent) in agents.iter().enumerate() {
    for run in 0..DISCOVERY_RUNS {
        // PAUSED FOR COMPETITIVE AUDIT, only focused on critical issues in code
        // for (i, prompt) in [PROMPT_2X_AA, PROMPT_2X_BB].into_iter().enumerate() {
        for (i, prompt) in [llm_instructions].into_iter().enumerate() {
            let agent = Arc::clone(arc_agent);
            let combined_findings = Arc::clone(&all_findings);
            let contract_name = Arc::clone(&contract);
            let code = Arc::clone(&codeblock);
            let added_content = Arc::clone(&added_content_from_brain);
            let prompt_plus_scope = if audit_scope.is_empty() {
                Arc::new(prompt.to_string())
            } else {
                Arc::new(format!(
                    "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY REPORT FINDINGS WITHIN SCOPE \n\n{}",
                    prompt, &audit_scope
                ))
            };

            // info!("discovery prompt + scope => {}", prompt_plus_scope);

            handles.push(tokio::spawn(async move {
                if let Err(e) = run_security_prompt(
                    agent,
                    contract_name,
                    code,
                    added_content,
                    prompt_plus_scope,
                    (run + 1) * (i + 1),
                    combined_findings,
                )
                .await
                {
                    log::error!("Prompt task failed: {e:#}");
                }
            }));
        }
    }

    // Wait for ALL tasks to complete
    for handle in handles {
        handle.await?; // Will error if task panicked
    }

    let findings = all_findings.lock().await;

    if !findings.findings.is_empty() {
        info!(
            "✅ Phase 2 complete: {} findings BEFORE deduping",
            findings.findings.len()
        );
    }
    Ok(findings.clone())
}

/// Executes one LLM-prompt round and merges the returned findings into the shared findings collection
///
/// This function handles individual security analysis rounds, managing prompt generation,
/// LLM interaction, and result aggregation.
pub async fn run_security_prompt(
    agent: Arc<AIAgent>,
    contract_name: Arc<String>,
    code: Arc<String>,
    added_context: Arc<String>,
    instructions: Arc<String>,
    idx_of_review_round: usize,
    shared_findings: Arc<Mutex<Findings>>,
) -> Result<()> {
    // 1. Build full prompt
    let post_prompt = generate_post_prompt(&contract_name);
    let pre_prompt = generate_pre_prompt(&contract_name);
    // TODO - add conditional here if its code4rena run then different PRE and POST
    let prompt_header =
        generated_llm_prompt(&contract_name, &instructions, &pre_prompt, &post_prompt);
    let prompt_body = generate_content_plus_context_block(&code, &added_context);
    let full_prompt = format!("{prompt_header}{prompt_body}");

    // 2. Send to the right provider
    info!(
        "----{} contract LLM analysis Round #{}----",
        contract_name, idx_of_review_round
    );
    let findings: Findings = agent.extract_with_retry(&full_prompt).await?;

    let issues_found = findings.findings.len();
    info!("{} issues found!", issues_found);

    // 3. Merge results (if any) into the shared accumulator
    if issues_found > 0 {
        let mut guard = shared_findings.lock().await;
        guard.findings.extend(findings.findings);
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
