/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, LlmCostType},
    error::Result,
    llm_review::{
        config::{Finding, Findings, LegitVulnerability},
        enums::AIAgent,
        prompt_context::generate_prompt_for_issue_check,
        prompt_support::{
            post_verify::POST_VERIFY, pre_verify::PRE_VERIFY, verify_prompt::VERIFY_PROMPT,
        },
    },
};
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    context: &str,
) -> Result<Findings> {
    info!("🔍 Phase 3: Deduplicating and verifying findings...");

    let mut handles = vec![];
    let deduped_findings = Arc::new(findings.dedup().await?);
    let code_and_context = generate_content_plus_context_block(code, context);
    let arc_code_context = Arc::new(code_and_context);

    let dedup_finding_count = deduped_findings.findings.len();
    let is_legit_finding_vec: Arc<Mutex<Vec<bool>>> =
        Arc::new(Mutex::new(vec![true; dedup_finding_count]));

    info!("# of findings AFTER deduping => {}", dedup_finding_count);
    info!("now verifying each finding...");

    for i in 0..dedup_finding_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_agent = Arc::clone(agent);
        let arc_findings = Arc::clone(&deduped_findings);
        let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);

        handles.push(tokio::spawn(async move {
            let result: Result<()> = async {
                let instruction_prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings.findings[i],
                    PRE_VERIFY,
                    VERIFY_PROMPT,
                    POST_VERIFY,
                );

                // add to cost
                add_to_inference_cost_by_type(&instruction_prompt, LlmCostType::OpenaiO3Input)
                    .await;
                info!("verifying finding #{}", i + 1);
                let is_legit_struct: LegitVulnerability =
                    arc_agent.extract_with_retry(&instruction_prompt).await?;

                let is_finding_legit = is_legit_struct.is_legit_vulnerability;
                if !is_finding_legit {
                    info!(
                        "{} is NOT legit => {}",
                        arc_findings.findings[i].title(),
                        is_legit_struct.why_its_not_legit.unwrap_or_default()
                    );
                }
                let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                legit_findings_vec[i] = is_finding_legit;

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

    let legit_findings_vec = is_legit_finding_vec.lock().await;
    let verified_findings: Vec<Finding> = deduped_findings
        .as_ref()
        .findings
        .iter()
        .enumerate()
        .filter(|(idx, _)| legit_findings_vec[*idx])
        .map(|(_, f)| f.clone())
        .collect();

    info!(
        "✅ Phase 3 complete: {} Verified Findings!",
        verified_findings.len()
    );

    Ok(Findings {
        findings: verified_findings,
    })
}

/// Generates the combined content and context block for verification analysis
///
/// Combines the contract code with additional context information
/// in a structured format for optimal verification processing.
fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\n# SOLIDITY CONTRACT + STORAGE TO CODE REVIEW\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context
        .push_str("\n\n ## ADDITIONAL CONTEXT TO ASSIST WITH SECURITY REVIEW OF ABOVE CODE \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}
