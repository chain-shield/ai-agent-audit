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
        utils::{
            prompt_context::{generate_prompt_for_issue_check, FindingReportType},
            save_run_poc::save_and_run_poc_test,
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use log::{info, warn};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

/// Sanitize a string to be safe for use as a filename
/// Replaces all characters that are not alphanumeric, dash, or underscore with a dash
fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// Extract test function names from Solidity code
/// Returns the first test function found (function name starting with "test")
fn extract_test_function_name(code: &str) -> Option<String> {
    // Regex to match Solidity function declarations starting with "test"
    // Matches: function testSomething() public { ... }
    let re = regex::Regex::new(r"function\s+(test\w+)\s*\(").ok()?;

    re.captures(code)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

/// Build the forge test command programmatically
/// This ensures consistent command format and correct relative paths
fn build_forge_test_command(
    test_file_relative_path: &str,
    test_function_name: Option<&str>,
) -> String {
    match test_function_name {
        Some(name) => format!(
            "forge test --match-path {} --match-test {} -vvv",
            test_file_relative_path, name
        ),
        None => format!("forge test --match-path {} -vvv", test_file_relative_path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        // Test parentheses
        assert_eq!(
            sanitize_filename("Unbounded O(n^2) duplicate scan"),
            "Unbounded-O-n-2--duplicate-scan"
        );

        // Test spaces
        assert_eq!(
            sanitize_filename("Rounding dust from 80/20 split"),
            "Rounding-dust-from-80-20-split"
        );

        // Test special characters
        assert_eq!(
            sanitize_filename("Test: with/special\\chars*and?more!"),
            "Test--with-special-chars-and-more-"
        );

        // Test already clean
        assert_eq!(
            sanitize_filename("Already-clean_filename123"),
            "Already-clean_filename123"
        );
    }

    #[test]
    fn test_extract_test_function_name() {
        // Test basic function
        let code = r#"
            function testExploit() public {
                // test code
            }
        "#;
        assert_eq!(
            extract_test_function_name(code),
            Some("testExploit".to_string())
        );

        // Test function with parameters
        let code = r#"
            function testReentrancy(uint256 amount) public {
                // test code
            }
        "#;
        assert_eq!(
            extract_test_function_name(code),
            Some("testReentrancy".to_string())
        );

        // Test multiple functions (should return first)
        let code = r#"
            function setUp() public {}
            function testAccessControl() public {}
            function testAnother() public {}
        "#;
        assert_eq!(
            extract_test_function_name(code),
            Some("testAccessControl".to_string())
        );

        // Test no test function
        let code = r#"
            function setUp() public {}
            function helper() internal {}
        "#;
        assert_eq!(extract_test_function_name(code), None);
    }

    #[test]
    fn test_build_forge_test_command() {
        // With test function name
        assert_eq!(
            build_forge_test_command("test/MyTest.t.sol", Some("testExploit")),
            "forge test --match-path test/MyTest.t.sol --match-test testExploit -vvv"
        );

        // Without test function name
        assert_eq!(
            build_forge_test_command("test/MyTest.t.sol", None),
            "forge test --match-path test/MyTest.t.sol -vvv"
        );
    }
}

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

    // NOTE: writing PoC ONLY for Critical, High, and Medium findings
    let m_and_h_findings: Vec<Finding> = findings
        .findings
        .iter()
        .filter(|f| f.severity != Severity::Low && f.severity != Severity::Info)
        .map(|f| f.to_owned())
        .collect();

    let context = match get_metadata_context(repo).await {
        Some(ctx) => ctx,
        None => {
            log::warn!("Metadata context not available, continuing PoC generation without it");
            String::new()
        }
    };
    let code_and_context = generate_content_plus_context_block(code, &context);

    let mut poc_tests: Vec<PocTest> = Vec::new();

    info!(
        "now writing PoC for {} findings sequentially...",
        m_and_h_findings.len()
    );

    // Process each finding sequentially (no multithreading needed since POC_SEM = 1)
    for (i, finding) in m_and_h_findings.iter().enumerate() {
        let result: Result<()> = async {
            // Sanitize the title to create a safe filename
            let raw_filename = sanitize_filename(&finding.title);
            let truncated_name: String = raw_filename.chars().take(20).collect();
            let filename = format!(
                "{}-{}.t.sol",
                finding.severity.as_initial(),
                truncated_name
            );
            let poc_prompt = generate_poc_prompt(&filename, repo)?;
            let instruction_prompt = generate_prompt_for_issue_check(
                &code_and_context,
                finding,
                &poc_prompt,
                POST_CREATE_POC,
                FindingReportType::NoPoC,
            );

            // add to cost
            info!("writing PoC for finding #{}", i + 1);
            let poc_test_data: GeneratePocTest =
                agent.extract_with_retry(&instruction_prompt).await?;

            // Extract test function name from the generated code
            let test_function_name = extract_test_function_name(&poc_test_data.poc_test_code);

            // Build the forge test command programmatically with relative path
            let code_root = repo.root.join(&repo.repo_name);
            let test_file_path = repo.poc.test_folder.join(&filename);
            let relative_path = test_file_path
                .strip_prefix(&code_root)
                .unwrap_or(&test_file_path)
                .to_string_lossy()
                .to_string();
            let command =
                build_forge_test_command(&relative_path, test_function_name.as_deref());

            let mut poc_test = PocTest {
                finding_hash: finding.hash_derived(),
                poc_test_code: poc_test_data.poc_test_code,
                poc_test_file: test_file_path,
                poc_test_filename: filename,
                poc_test_command: command,
                poc_test_status: PocStatus::ErrorRunningTests,
                poc_test_output: String::new(),
            };

            // Initial attempt to save and run the PoC test
            let mut poc_pass_attempt = 1;
            if let Err(e) = save_and_run_poc_test(&mut poc_test, repo) {
                log::error!(
                    "Failed to save/run PoC test for finding #{}: {:?}",
                    i + 1,
                    e
                );
                poc_test.poc_test_status = PocStatus::ErrorRunningTests;
                poc_test.poc_test_output = format!("Error: {:?}", e);
            }
            print_test_status(&poc_test, finding);

            info!(
                "AI Agent: {}",
                poc_test_data.commentary.clone().unwrap_or_default()
            );


            // Retry loop: up to 5 attempts total (initial + 4 retries)
            while poc_pass_attempt < 5 && poc_test.poc_test_status != PocStatus::AllTestPass {
                poc_pass_attempt += 1;
                info!("PoC attempt {} for finding #{}", poc_pass_attempt, i + 1);

                let retest_prompt = generate_rewrite_poc_prompt(&poc_test, repo)?;
                let retest_instruction_prompt = generate_prompt_for_issue_check(
                    &code_and_context,
                    finding,
                    &retest_prompt,
                    POST_CREATE_POC,
                    FindingReportType::NoPoC,
                );
                let updated_poc_test_data: GeneratePocTest = agent
                    .extract_with_retry(&retest_instruction_prompt)
                    .await?;

                info!(
                    "AI Agent: {}",
                    updated_poc_test_data.commentary.clone().unwrap_or_default()
                );

                if updated_poc_test_data.cannot_create_poc_because_finding_invalid != Some(true)
                {
                    // Update the PoC test with new code
                    poc_test.poc_test_code = updated_poc_test_data.poc_test_code;

                    // Extract test function name from the updated code
                    let test_function_name =
                        extract_test_function_name(&poc_test.poc_test_code);

                    // Rebuild the command with the new test function name
                    let code_root = repo.root.join(&repo.repo_name);
                    let relative_path = poc_test
                        .poc_test_file
                        .strip_prefix(&code_root)
                        .unwrap_or(&poc_test.poc_test_file)
                        .to_string_lossy()
                        .to_string();
                    poc_test.poc_test_command =
                        build_forge_test_command(&relative_path, test_function_name.as_deref());

                    // Save and run the updated PoC test
                    if let Err(e) = save_and_run_poc_test(&mut poc_test, repo) {
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

                print_test_status(&poc_test, finding);
            }

            // If PoC has compilation errors, delete the test file to prevent cross-contamination
            // with subsequent PoC tests (since Foundry compiles all test files).
            // Note: We only delete ErrorRunningTests (compilation errors), not FailingTests
            // (which compile but have failing assertions - those won't break other tests).
            if matches!(
                poc_test.poc_test_status,
                PocStatus::ErrorRunningTests | PocStatus::FindingIsInvalid
            ) {
                if poc_test.poc_test_file.exists() {
                    log::warn!(
                        "🗑️  Deleting PoC test file with compilation errors to prevent cross-contamination: {}",
                        poc_test.poc_test_file.display()
                    );
                    if let Err(e) = std::fs::remove_file(&poc_test.poc_test_file) {
                        log::error!(
                            "Failed to delete PoC test file {}: {:?}",
                            poc_test.poc_test_file.display(),
                            e
                        );
                    }
                }
            }

            // save final PoC results
            poc_tests.push(poc_test);
            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error creating PoC for finding {}: {:?}", i, e);
        }
    }

    // Add final runnable PoCs to Findings
    let finding_with_pocs: Vec<Finding> = findings
        .findings
        .iter()
        .map(|f| {
            let poc_test: Option<&PocTest> = poc_tests
                .iter()
                .find(|t| t.finding_hash == f.hash_derived());
            match poc_test {
                Some(poc_test) => Finding {
                    proof_of_code: Some(poc_test.poc_test_code.clone()),
                    poc_test_file: Some(poc_test.poc_test_file.clone()),
                    poc_test_command: Some(poc_test.poc_test_command.clone()),
                    poc_test_status: Some(poc_test.poc_test_status),
                    ..f.clone()
                },
                None => f.clone(),
            }
        })
        .collect();
    info!(
        "✅ Phase 6 complete: {} Findings with PoC tests!",
        m_and_h_findings.len()
    );

    Ok(Findings {
        findings: finding_with_pocs,
    })
}

fn print_test_status(poc_test: &PocTest, finding: &Finding) {
    match poc_test.poc_test_status {
        PocStatus::FailingTests => warn!("Some Poc tests are failing for: {}", finding.title),
        PocStatus::AllTestPass => info!("All PoC tests passing test for: {}", finding.title),
        PocStatus::ErrorRunningTests => warn!("Error running PoC test for: {}", finding.title),
        PocStatus::FindingIsInvalid => {
            warn!("Cannot write test, Invalid Finding: {}", finding.title)
        }
    }
}
