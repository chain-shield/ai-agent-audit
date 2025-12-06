/// Phase 7: Professional competition-grade report generation
///
/// This phase generates professional markdown reports for validated findings
/// with passing PoC tests, formatted for bug bounty platforms (Code4rena, Sherlock, etc).
use crate::{
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::{context_state::get_metadata_context, semaphore::VERIFY_SEM},
        findings::findings::{Finding, Findings},
        phases::{add_poc_findings::PocStatus, verify_findings::FindingStatus},
        prompt_support::create_report_prompt::generate_create_report_prompt,
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
pub struct CompetitionReport {
    pub github_urls: Vec<String>,
    pub report: String,
}

/// Executes the professional report generation phase
///
/// Generates competition-grade markdown reports for findings that have:
/// - Status: Valid
/// - PoC Test Status: AllTestPass
///
/// Reports are formatted according to the audit platform (Code4rena, Sherlock, etc)
/// and include GitHub URLs with line numbers for all relevant code.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 7: Generating Professional Report for each Validated Finding");

    let mut handles = vec![];
    let arc_findings = Arc::new(findings);
    let context = get_metadata_context(repo).await.unwrap_or_else(|| {
        log::warn!("No metadata context found, using empty context");
        String::new()
    });
    let code_and_context = generate_code_content_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);
    let arc_repo = Arc::new(repo.clone());

    let finding_count = arc_findings.findings.len();
    // Pre-allocate vector with default reports for all findings
    // Only indices where status==Valid && poc_test_status==AllTestPass will be updated
    // This ensures safe indexed access in the concurrent tasks below
    let professional_reports: Arc<Mutex<Vec<CompetitionReport>>> = Arc::new(Mutex::new(vec![
            CompetitionReport::default();
            finding_count
        ]));

    info!("");

    for i in 0..finding_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_findings = Arc::clone(&arc_findings);
        let arc_professional_reports = Arc::clone(&professional_reports);
        let repo_clone = Arc::clone(&arc_repo);
        let arc_agent = Arc::clone(&agent);
        let sem = Arc::clone(&VERIFY_SEM);

        if arc_findings.findings[i]
            .status
            .as_ref()
            .is_some_and(|s| s.contains(&FindingStatus::Valid))
            && arc_findings.findings[i].poc_test_status == Some(PocStatus::AllTestPass)
        {
            handles.push(tokio::spawn(async move {
                // ── acquire permit ────────────────────────
                let _permit = sem.acquire_owned().await.expect("semaphore closed");
                let result: Result<()> = async {
                    let create_report_instructions = generate_create_report_prompt(
                        &repo_clone,
                        &arc_findings.findings[i],
                        &codeblock_plus_context,
                    )?;

                    info!("creating report for finding #{}", i + 1);
                    let report: CompetitionReport = arc_agent
                        .extract_with_retry(&create_report_instructions)
                        .await?;

                    let mut reports = arc_professional_reports.lock().await;
                    reports[i] = report;

                    Ok(())
                }
                .await;

                if let Err(e) = result {
                    log::error!("Error generating report for finding {}: {:?}", i, e);
                }
            }));
        }
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let reports = professional_reports.lock().await;
    let findings_with_reports: Vec<Finding> = arc_findings
        .as_ref()
        .findings
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            if !reports[idx].report.is_empty() {
                let enriched_finding = Finding {
                    competition_report: Some(reports[idx].clone()),
                    ..f.clone()
                };
                enriched_finding
            } else {
                f.clone()
            }
        })
        .collect();

    // Count only findings that actually received professional reports
    let report_count = reports.iter().filter(|r| !r.report.is_empty()).count();
    info!(
        "✅ Phase 7 complete: {} Findings with Passing PoCs and Professional Reports!",
        report_count
    );

    Ok(Findings {
        findings: findings_with_reports,
    })
}

/// Generates the combined content and context block for additional context
///
/// Combines the contract code with additional context information
/// in a structured format for optimal report generation.
fn generate_code_content_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\n# SOLIDITY CONTRACT WHERE FINDING WAS DISCOVERED\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context.push_str("\n\n ## ADDITIONAL CONTEXT \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}
