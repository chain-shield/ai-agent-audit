/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, LlmCostType},
    error::Result,
    llm_review::{
        config::{Finding, Findings},
        context_state::{generate_audit_scope, get_metadata_context},
        enums::AIAgent,
        phases::verify_findings::{
            deserialize_bool_from_str_or_bool, generate_content_plus_context_block,
        },
        prompt_support::{
            post_verify::POST_IN_SCOPE_VERIFY, pre_verify::PRE_IN_SCOPE_VERIFY,
            verify_prompt::VERIFY_IN_SCOPE_PROMPT,
        },
        semaphore::VERIFY_SEM,
        utils::prompt_context::generate_prompt_for_issue_check,
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InScope {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_vulnerability_in_scope: bool,
    pub why_its_not_in_scope: Option<String>,
}
/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 3: Deduplicating and verifying findings...");
    let audit_scope = generate_audit_scope(repo).await?;

    // sanity check
    if audit_scope.is_empty() {
        // no scope provided so just return findings
        log::error!("No scope provided for this protoocol");
        return Ok(findings);
    }

    let mut handles = vec![];
    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);

    let findings_count = findings.findings.len();
    let in_scope_finding_vec: Arc<Mutex<Vec<bool>>> =
        Arc::new(Mutex::new(vec![true; findings_count]));

    info!("checking if each finding is In Scope...");

    let in_scope_verify_prompt = Arc::new(format!(
        "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN SCOPE ARE LEGIT \n\n{}",
        VERIFY_IN_SCOPE_PROMPT, audit_scope
    ));

    let findings_vec = findings.findings;
    let findings_arc = Arc::new(findings_vec);

    for i in 0..findings_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_findings = Arc::clone(&findings_arc);
        let arc_agent = Arc::clone(&agent);
        let arc_in_scope_findings_vec = Arc::clone(&in_scope_finding_vec);
        let verify_prompt_and_scope = Arc::clone(&in_scope_verify_prompt);
        let sem = Arc::clone(&VERIFY_SEM);

        handles.push(tokio::spawn(async move {
            // ── acquire permit ────────────────────────
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async {
                let instruction_prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings[i],
                    PRE_IN_SCOPE_VERIFY,
                    &verify_prompt_and_scope,
                    POST_IN_SCOPE_VERIFY,
                );

                // add to cost
                add_to_inference_cost_by_type(&instruction_prompt, LlmCostType::OpenaiO3Input)
                    .await;
                info!("scope checking finding #{}", i + 1);
                let is_in_scope_struct: InScope =
                    arc_agent.extract_with_retry(&instruction_prompt).await?;

                let is_finding_in_scope = is_in_scope_struct.is_vulnerability_in_scope;
                if !is_finding_in_scope {
                    info!(
                        "{} is NOT in scope => {}",
                        arc_findings[i].title(),
                        is_in_scope_struct.why_its_not_in_scope.unwrap_or_default()
                    );
                }
                let mut legit_in_scope_findings_vec = arc_in_scope_findings_vec.lock().await;
                legit_in_scope_findings_vec[i] = is_finding_in_scope;

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error verifying finding {}: {:?}", i, e);
            }
        }));
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let in_scope_vec = in_scope_finding_vec.lock().await;
    let in_scope_findings: Vec<Finding> = findings_arc
        .iter()
        .enumerate()
        .filter(|(idx, _)| in_scope_vec[*idx])
        .map(|(_, f)| f.clone())
        .collect();

    info!(
        "✅ Phase 4 complete: {} In Scope Findings!",
        in_scope_findings.len()
    );

    Ok(Findings {
        findings: in_scope_findings,
    })
}
