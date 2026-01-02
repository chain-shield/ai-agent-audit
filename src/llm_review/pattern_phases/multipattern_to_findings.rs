/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    config::{ACTOR_DISCOVERY_RUNS, INVARIANT_DISCOVERY_RUNS, PATTERN_DISCOVERY_RUNS},
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::{
            context_state::{generate_audit_scope, get_metadata_context},
            semaphore::GENERAL_SEM,
        },
        findings::findings::Findings,
        threat_models::issues::{IssueStructTrait, IssueTrait},
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute<T>(
    patterns: T,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
    <T as IssueStructTrait>::Spec: Send + Sync + Clone + DeserializeOwned + IssueTrait + 'static,
{
    let issue_title = patterns.issue_title();
    info!("🔍 Phase 3: Mining Findings from each {}...", issue_title);

    let all_findings = Arc::new(Mutex::new(Findings {
        findings: Vec::new(),
        checked_for: None,
    }));

    let mut handles = vec![];
    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);
    let audit_scope = Arc::new(generate_audit_scope(repo).await?);
    let arc_repo = Arc::new(repo.clone());
    let arc_patterns = Arc::new(patterns.clone());

    let runs = if issue_title == "invariant" {
        INVARIANT_DISCOVERY_RUNS
    } else if issue_title == "actor exploit" {
        ACTOR_DISCOVERY_RUNS
    } else {
        PATTERN_DISCOVERY_RUNS
    };

    for i in 0..runs {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_agent = Arc::clone(&agent);
        let patterns_clone = Arc::clone(&arc_patterns);
        let sem = Arc::clone(&GENERAL_SEM);
        let repo_clone = Arc::clone(&arc_repo);
        let shared_findings = Arc::clone(&all_findings);
        let title = issue_title.clone();
        let scope = Arc::clone(&audit_scope);

        handles.push(tokio::spawn(async move {
            // ── acquire permit ────────────────────────
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async {
                info!("Round {} of mining {} to generate findings", i + 1, title,);
                let core_instructions = patterns_clone.multi_issue_to_findings_prompt(&repo_clone);
                // info!("core instructions: {}", core_instructions);

                let instruction_prompt = if scope.is_empty() {
                    core_instructions
                } else {
                    format!(
                    "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY REPORT FINDINGS WITHIN SCOPE \n\n{}",
                    &core_instructions, &scope)
                };

                let json_requirement_prompt =
                    patterns_clone.multi_issue_findings_json_required_prompt(&repo_clone);

                let full_prompt = format!(
                    "{}{}{}",
                    instruction_prompt, codeblock_plus_context, json_requirement_prompt
                );

                let findings: Findings = arc_agent.extract_with_retry(&full_prompt).await?;

                let issues_found = findings.findings.len();
                info!("{} findings found from {}!", issues_found, title);

                // 3. Merge results (if any) into the shared accumulator
                if issues_found > 0 {
                    let mut guard = shared_findings.lock().await;
                    guard.findings.extend(findings.findings);
                }

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error extracting findings from pattern: {}", e);
            }
        }));
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let findings = all_findings.lock().await;

    if !findings.findings.is_empty() {
        info!(
            "✅ Phase 3 complete: {} findings BEFORE deduping",
            findings.findings.len()
        );
    }
    Ok(findings.clone())
}

pub fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context
        .push_str("\n\n ## ADDITIONAL CONTEXT TO ASSIST WITH SECURITY REVIEW OF ABOVE CODE \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}
