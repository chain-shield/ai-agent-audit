/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    error::Result,
    llm_review::{
        context_state::get_metadata_context,
        enums::{AIAgent, Severity},
        findings::{Finding, Findings},
        pattern_phases::pattern_to_findings::generate_content_plus_context_block,
        prompt_support::{
            make_poc_prompt::{generate_poc_prompt, generate_rewrite_poc_prompt},
            post_poc::POST_CREATE_POC,
        },
        semaphore::POC_SEM,
        utils::{
            prompt_context::{FindingReportType, generate_prompt_for_issue_check},
            save_run_poc::save_and_run_poc_test,
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use log::{info, warn};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum PocStatus {
    AllTestPass,
    FailingTests,
    #[default]
    ErrorRunningTests,
    FindingIsInvalid,
}

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PocTest {
    pub finding_hash: String,
    pub poc_test_code: String,
    pub poc_test_file: PathBuf,
    pub poc_test_filename: String,
    pub poc_test_command: String,
    pub poc_test_output: String,
    pub poc_test_status: PocStatus,
}

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GeneratePocTest {
    pub poc_test_code: String,
    pub command_to_run_test: String,
    pub cannot_create_poc_because_finding_invalid: Option<bool>,
    /// LLM commentary on PoC status and next steps
    pub commentary: Option<String>,
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
    info!("🔬 Phase 6: Writing PoC tests for each Medium and High finding...");

    let mut handles = vec![];
    // NOTE: writing PoC ONLY for Critical, High, and Medium findings
    let m_and_h_findings = Arc::new(Findings {
        findings: findings
            .findings
            .iter()
            .filter(|f| f.severity != Severity::Low && f.severity != Severity::Info)
            .map(|f| f.to_owned())
            .collect(),
    });
    let context = match get_metadata_context(repo).await {
        Some(ctx) => ctx,
        None => {
            log::warn!("Metadata context not available, continuing PoC generation without it");
            String::new()
        }
    };
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);
    let repo_paths = Arc::new(repo.clone());

    let number_of_findings = m_and_h_findings.findings.len();
    let poc_tests: Arc<Mutex<Vec<PocTest>>> = Arc::new(Mutex::new(Vec::new()));

    info!("now writing PoC for each finding...");

    for i in 0..number_of_findings {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_findings = Arc::clone(&m_and_h_findings);
        let arc_repo = Arc::clone(&repo_paths);
        let arc_agent = Arc::clone(&agent);
        let arc_poc_test_vec = Arc::clone(&poc_tests);
        let sem = Arc::clone(&POC_SEM);

        handles.push(tokio::spawn(async move {
            // ── acquire permit ────────────────────────
            let _permit = match sem.acquire_owned().await {
                Ok(permit) => permit,
                Err(e) => {
                    log::error!(
                        "Failed to acquire semaphore permit for finding #{}: {:?}",
                        i + 1,
                        e
                    );
                    return;
                }
            };

            let result: Result<()> = async {
                let raw_filename = arc_findings.findings[i].title.replace(" ", "-");
                let truncated_name: String = raw_filename.chars().take(20).collect();
                let filename = format!(
                    "{}-{}.t.sol",
                    arc_findings.findings[i].severity.as_initial(),
                    truncated_name
                );
                let poc_prompt = generate_poc_prompt(&filename, &arc_repo)?;
                let instruction_prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings.findings[i],
                    &poc_prompt,
                    POST_CREATE_POC,
                    FindingReportType::NoPoC,
                );

                // add to cost
                info!("writiing PoC for finding #{}", i + 1);
                let poc_test_data: GeneratePocTest =
                    arc_agent.extract_with_retry(&instruction_prompt).await?;

                let mut poc_test = PocTest {
                    finding_hash: arc_findings.findings[i].hash_derived(),
                    poc_test_code: poc_test_data.poc_test_code,
                    poc_test_file: arc_repo.poc.test_folder.join(&filename),
                    poc_test_filename: filename,
                    poc_test_command: poc_test_data.command_to_run_test,
                    poc_test_status: PocStatus::ErrorRunningTests,
                    poc_test_output: String::new(),
                };

                // Initial attempt to save and run the PoC test
                let mut poc_pass_attempt = 1;
                if let Err(e) = save_and_run_poc_test(&mut poc_test, &arc_repo) {
                    log::error!(
                        "Failed to save/run PoC test for finding #{}: {:?}",
                        i + 1,
                        e
                    );
                    poc_test.poc_test_status = PocStatus::ErrorRunningTests;
                    poc_test.poc_test_output = format!("Error: {:?}", e);
                }

                // Retry loop: up to 5 attempts total (initial + 4 retries)
                while poc_pass_attempt < 5 && poc_test.poc_test_status != PocStatus::AllTestPass {
                    poc_pass_attempt += 1;
                    info!("PoC attempt {} for finding #{}", poc_pass_attempt, i + 1);

                    let retest_prompt = generate_rewrite_poc_prompt(&poc_test, &arc_repo)?;
                    let retest_instruction_prompt = generate_prompt_for_issue_check(
                        &codeblock_plus_context,
                        &arc_findings.findings[i],
                        &retest_prompt,
                        POST_CREATE_POC,
                        FindingReportType::NoPoC,
                    );
                    let updated_poc_test_data: GeneratePocTest = arc_agent
                        .extract_with_retry(&retest_instruction_prompt)
                        .await?;

                    if updated_poc_test_data.cannot_create_poc_because_finding_invalid != Some(true)
                    {
                        // Update the PoC test with new code and command
                        poc_test.poc_test_code = updated_poc_test_data.poc_test_code;
                        poc_test.poc_test_command = updated_poc_test_data.command_to_run_test;

                        // Save and run the updated PoC test
                        if let Err(e) = save_and_run_poc_test(&mut poc_test, &arc_repo) {
                            log::error!(
                                "Failed to save/run PoC test retry #{} for finding #{}: {:?}",
                                poc_pass_attempt,
                                i + 1,
                                e
                            );
                            poc_test.poc_test_status = PocStatus::ErrorRunningTests;
                            poc_test.poc_test_output = format!("Error: {:?}", e);
                            // Continue to next retry attempt
                        }
                    } else {
                        poc_test.poc_test_status = PocStatus::FindingIsInvalid;
                        break;
                    }

                    match poc_test.poc_test_status {
                        PocStatus::FailingTests => warn!(
                            "Some Poc tests are failing for: {}",
                            arc_findings.findings[i].title
                        ),
                        PocStatus::AllTestPass => info!(
                            "All PoC tests passing test for: {}",
                            arc_findings.findings[i].title
                        ),
                        PocStatus::ErrorRunningTests => warn!(
                            "Error running PoC test for: {}",
                            arc_findings.findings[i].title
                        ),
                        PocStatus::FindingIsInvalid => warn!(
                            "Cannot write test, Invalid Finding: {}",
                            arc_findings.findings[i].title
                        ),
                    }
                }

                // save final PoC results
                let mut poc_test_vec = arc_poc_test_vec.lock().await;
                poc_test_vec.push(poc_test);
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

    // Add final runnable PoCs to Findings
    let poc_tests_final = poc_tests.lock().await;
    let finding_with_pocs: Vec<Finding> = findings
        .findings
        .iter()
        .map(|f| {
            let poc_test: Option<PocTest> = poc_tests_final
                .iter()
                .find(|t| t.finding_hash == f.hash_derived())
                .cloned();
            match poc_test {
                Some(poc_test) => Finding {
                    proof_of_code: Some(poc_test.poc_test_code),
                    poc_test_file: Some(poc_test.poc_test_file),
                    poc_test_command: Some(poc_test.poc_test_command),
                    poc_test_status: Some(poc_test.poc_test_status),
                    ..f.clone()
                },
                None => f.clone(),
            }
        })
        .collect();
    info!(
        "✅ Phase 6 complete: {} Findings with PoC tests!",
        m_and_h_findings.findings.len()
    );

    Ok(Findings {
        findings: finding_with_pocs,
    })
}
