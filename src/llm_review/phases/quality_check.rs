/// Phase 4: Quality assurance and final finding refinement
/// 
/// This phase performs final quality checks on verified findings and enhances
/// them with improved details, impact analysis, and mitigation strategies.

use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, LlmCostType},
    error::Result,
    llm_review::{
        config::{Findings, Finding, VulnerabilityQualityCheck},
        enums::AIAgent,
        prompt_context::generate_prompt_for_issue_check,
        prompt_support::{
            pre_qualify::PRE_QUALIFY,
            qualify_prompt::QUALIFY_PROMPT,
            post_qualify::POST_QUALIFY,
        },
    },
};
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Executes the quality check phase
/// 
/// Performs final quality assurance on verified findings, enhancing them
/// with improved details and ensuring they meet quality standards.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    context: &str,
) -> Result<Findings> {
    info!("🔍 Phase 4: Quality checking findings...");
    
    let mut handles = vec![];
    let findings = Arc::new(findings.dedup().await?);
    let code_and_context = generate_content_plus_context_block(code, context);
    let arc_code_context = Arc::new(code_and_context);

    let finding_count = findings.findings.len();
    // create vec (is_quality_check_passed, updated_finding) for each finding
    // assume all initially pass
    let quality_check_passed_vec: Arc<Mutex<Vec<(bool, Finding)>>> =
        Arc::new(Mutex::new(vec![(true, Finding::default()); finding_count]));

    info!("now quality check on each finding...");

    for i in 0..finding_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_agent = Arc::clone(agent);
        let arc_findings = Arc::clone(&findings);
        let arc_legit_findings_vec = Arc::clone(&quality_check_passed_vec);
        
        handles.push(tokio::spawn(async move {
            let result: Result<()> = async {
                let prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings.findings[i],
                    PRE_QUALIFY,
                    QUALIFY_PROMPT,
                    POST_QUALIFY,
                );
                add_to_inference_cost_by_type(&prompt, LlmCostType::OpenaiO3Input).await;
                info!("quality checking finding #{}", i + 1);
                let qualify_checked_finding: VulnerabilityQualityCheck =
                    arc_agent.extract_with_retry(&prompt).await?;

                let quality_check_passed = qualify_checked_finding.is_quality_check_passed;
                if !quality_check_passed {
                    info!(
                        "{} did not pass quality check => {:?}",
                        arc_findings.findings[i].title(),
                        qualify_checked_finding.where_quality_lacks
                    );
                    info!("{:#?}", &qualify_checked_finding);
                    let updated_finding = Finding {
                        impact: Some(qualify_checked_finding.impact.clone().unwrap_or(
                            arc_findings.findings[i].impact.clone().unwrap_or_default(),
                        )),
                        proof_of_code: Some(
                            qualify_checked_finding.proof_of_code.clone().unwrap_or(
                                arc_findings.findings[i]
                                    .proof_of_code
                                    .clone()
                                    .unwrap_or_default(),
                            ),
                        ),
                        proof_of_concept: Some(
                            qualify_checked_finding.proof_of_concept.clone().unwrap_or(
                                arc_findings.findings[i]
                                    .proof_of_concept
                                    .clone()
                                    .unwrap_or_default(),
                            ),
                        ),
                        mitigation: Some(
                            qualify_checked_finding.mitigation.clone().unwrap_or(
                                arc_findings.findings[i]
                                    .mitigation
                                    .clone()
                                    .unwrap_or_default(),
                            ),
                        ),
                        severity: qualify_checked_finding
                            .severity
                            .unwrap_or(arc_findings.findings[i].severity),
                        ..arc_findings.findings[i].clone()
                    };
                    let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                    legit_findings_vec[i] = (quality_check_passed, updated_finding);
                } else {
                    let mut quality_checked_findings_vec = arc_legit_findings_vec.lock().await;
                    quality_checked_findings_vec[i] = (true, arc_findings.findings[i].clone());
                }

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error verifying finding {}: {:?}", i, e);
            }
        }));
    }

    // Wait for all quality check tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let qualify_checked_findings_vec = quality_check_passed_vec.lock().await;
    let qualified_findings: Vec<Finding> = findings
        .as_ref()
        .findings
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            if qualify_checked_findings_vec[idx].0 {
                // if quality check passes, no changes needed
                f.clone()
            } else {
                // if failed submit updated finding
                qualify_checked_findings_vec[idx].1.clone()
            }
        })
        .collect();

    let num_findings_updated = qualify_checked_findings_vec
        .iter()
        .filter(|(passed, _)| !*passed)
        .count();

    info!(
        "✅ Phase 4 complete: {} Verified Findings with {} updated findings!",
        qualified_findings.len(),
        num_findings_updated
    );

    Ok(Findings {
        findings: qualified_findings,
    })
}

/// Generates the combined content and context block for quality check analysis
/// 
/// Combines the contract code with additional context information
/// in a structured format for optimal quality check processing.
fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\nSOLIDITY CONTRACT + STORAGE TO CODE REVIEW\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context.push_str("\n\n ## ADDITIONAL CONTEXT \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}
