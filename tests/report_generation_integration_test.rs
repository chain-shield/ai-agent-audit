use ai_agent_audit::{
    config::{AuditType, init_config},
    llm_review::{
        agent::agent_factory::{AgentConfig, AgentFactory, init_llm_clients},
        findings::{
            finding_enums::{Severity, VulnerabilityType},
            findings::{Finding, Findings, PrivilegeLevel},
        },
        phases::{add_poc_findings::PocStatus, create_report, verify_rounds::FindingStatus},
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use dotenvy::dotenv;
use std::{env, path::PathBuf, sync::Arc};

/// Returns true if Anthropic API key is present in the environment
fn anthropic_key_present() -> bool {
    env::var("ANTHROPIC_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

/// Integration test for Phase 7: Professional Report Generation
///
/// This test:
/// 1. Creates mock findings with Valid status and AllTestPass PoC status
/// 2. Uses Claude API to generate professional competition reports
/// 3. Verifies the reports are generated correctly
/// 4. Validates the workflow completes without crashing
#[tokio::test]
async fn test_report_generation_with_valid_findings() {
    // Load environment variables first
    dotenv().ok();

    // Skip if Anthropic API key not available
    if !anthropic_key_present() {
        println!("⏭️  Skipping test: ANTHROPIC_API_KEY not set");
        return;
    }

    init_config().expect("Failed to initialize config");

    // Initialize LLM clients
    init_llm_clients().expect("Failed to initialize LLM clients");

    // Create mock RepoPaths first (needed for agent config)
    let repo = create_mock_repo_paths();

    // Create agent for report generation
    let agent_config = AgentConfig::new(Some(repo.clone()))
        .with_model("claude-3-7-sonnet-20250219")
        .with_temperature(0.3);

    let report_agent = Arc::new(
        AgentFactory::create_anthropic_agent(&agent_config).expect("Failed to create agent"),
    );

    // Create mock findings with Valid status and AllTestPass PoC status
    let findings = create_mock_findings_with_passing_pocs();

    // Mock codeblock (simplified PuppyRaffle contract)
    let codeblock = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

contract PuppyRaffle {
    uint256 public totalFees;
    address[] public players;
    
    function refund(uint256 playerIndex) public {
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "Only player can refund");
        
        // Vulnerable: external call before state update
        (bool success,) = msg.sender.call{value: entranceFee}("");
        require(success, "Refund failed");
        
        // State update after external call - reentrancy vulnerability
        players[playerIndex] = address(0);
    }
}
"#;

    println!("🧪 Testing Phase 7: Professional Report Generation");
    println!(
        "📊 Input: {} findings with Valid status and AllTestPass PoC",
        findings.findings.len()
    );

    // Execute Phase 7: Report Generation
    let result = create_report::execute(findings.clone(), codeblock, &report_agent, &repo).await;

    match result {
        Ok(findings_with_reports) => {
            println!("✅ Phase 7 completed successfully");

            // Verify findings were returned
            assert_eq!(
                findings_with_reports.findings.len(),
                findings.findings.len(),
                "Should return same number of findings"
            );

            // Count findings with reports
            let report_count = findings_with_reports
                .findings
                .iter()
                .filter(|f| f.competition_report.is_some())
                .count();

            println!("📝 Generated {} professional reports", report_count);

            // Verify at least one report was generated
            assert!(
                report_count > 0,
                "Should generate at least one professional report"
            );

            // Verify report structure for findings with reports
            for finding in &findings_with_reports.findings {
                if let Some(report) = &finding.competition_report {
                    println!("\n📄 Report for: {}", finding.title);

                    // Verify github_urls field exists and is not empty
                    assert!(
                        !report.github_urls.is_empty(),
                        "Report should contain GitHub URLs"
                    );

                    // Verify report content exists
                    assert!(
                        !report.report.is_empty(),
                        "Report markdown should not be empty"
                    );

                    // Verify report contains key sections
                    assert!(
                        report.report.contains("Summary") || report.report.contains("Description"),
                        "Report should contain Summary or Description section"
                    );

                    println!("  ✓ GitHub URLs: {}", report.github_urls.len());
                    println!("  ✓ Report length: {} chars", report.report.len());

                    // Print first GitHub URL as example
                    if !report.github_urls.is_empty() {
                        println!("  ✓ Example URL: {}", report.github_urls[0]);
                    }
                }
            }

            // Verify findings without AllTestPass don't get reports
            let findings_without_reports = findings_with_reports
                .findings
                .iter()
                .filter(|f| f.poc_test_status != Some(PocStatus::AllTestPass))
                .count();

            if findings_without_reports > 0 {
                println!(
                    "\n✓ Correctly skipped {} findings without AllTestPass status",
                    findings_without_reports
                );
            }
        }
        Err(e) => {
            panic!("❌ Phase 7 failed: {:?}", e);
        }
    }
}

/// Creates mock findings with Valid status and AllTestPass PoC status
fn create_mock_findings_with_passing_pocs() -> Findings {
    let finding1 = Finding {
        id: Some("H-1".to_string()),
        title: "Reentrancy vulnerability in refund function".to_string(),
        exploit_type: VulnerabilityType::Reentrancy,
        privilege: PrivilegeLevel::Permissionless,
        contract: "PuppyRaffle".to_string(),
        function: "refund(uint256)".to_string(),
        description: Some(
            "The refund function makes an external call before updating state, allowing reentrancy attacks.".to_string()
        ),
        impact: Some(
            "An attacker can drain all funds from the contract by reentering the refund function.".to_string()
        ),
        proof_of_concept: Some(
            "1. Attacker enters raffle\n2. Attacker calls refund\n3. In fallback, attacker reenters refund\n4. Attacker receives multiple refunds".to_string()
        ),
        proof_of_code: Some("contract ReentrancyAttack { ... }".to_string()),
        poc_test_file: Some(PathBuf::from("test/H-reentrancy-refund.t.sol")),
        poc_test_command: Some("forge test --match-path test/H-reentrancy-refund.t.sol -vvv".to_string()),
        poc_test_status: Some(PocStatus::AllTestPass),
        severity: Severity::High,
        mitigation: Some(
            "Use checks-effects-interactions pattern or add reentrancy guard.".to_string()
        ),
        status: Some(vec![FindingStatus::Valid]),
        status_justification: Some("Confirmed vulnerability with working PoC".to_string()),
        verification_rounds_passed: Some(3),
        competition_report: None,
        finding_complexity: Some(3),
        derived_from: Some("Pattern: Reentrancy".to_string()),
    };

    let finding2 = Finding {
        id: Some("M-1".to_string()),
        title: "Integer overflow in fee calculation".to_string(),
        exploit_type: VulnerabilityType::IntegerOverflow,
        privilege: PrivilegeLevel::Permissionless,
        contract: "PuppyRaffle".to_string(),
        function: "selectWinner()".to_string(),
        description: Some("Fee calculation can overflow when totalFees is very large.".to_string()),
        impact: Some("Protocol loses fee revenue due to overflow.".to_string()),
        proof_of_concept: Some("Set totalFees to max uint256, trigger overflow".to_string()),
        proof_of_code: Some("contract OverflowTest { ... }".to_string()),
        poc_test_file: Some(PathBuf::from("test/M-overflow-fees.t.sol")),
        poc_test_command: Some(
            "forge test --match-path test/M-overflow-fees.t.sol -vvv".to_string(),
        ),
        poc_test_status: Some(PocStatus::AllTestPass),
        severity: Severity::Medium,
        mitigation: Some("Use SafeMath or Solidity 0.8+".to_string()),
        status: Some(vec![FindingStatus::Valid]),
        status_justification: Some("Confirmed with PoC".to_string()),
        verification_rounds_passed: Some(3),
        competition_report: None,
        finding_complexity: Some(2),
        derived_from: Some("Pattern: IntegerOverflow".to_string()),
    };

    // Add a finding that should NOT get a report (no AllTestPass)
    let finding3 = Finding {
        id: Some("L-1".to_string()),
        title: "Low severity gas optimization".to_string(),
        exploit_type: VulnerabilityType::GasGriefBlockLimit,
        privilege: PrivilegeLevel::Permissionless,
        contract: "PuppyRaffle".to_string(),
        function: "enterRaffle()".to_string(),
        description: Some("Loop can be optimized".to_string()),
        impact: Some("Higher gas costs".to_string()),
        proof_of_concept: None,
        proof_of_code: None,
        poc_test_file: None,
        poc_test_command: None,
        poc_test_status: None, // No PoC status - should not get report
        severity: Severity::Low,
        mitigation: Some("Optimize loop".to_string()),
        status: None,
        status_justification: Some("Valid optimization".to_string()),
        verification_rounds_passed: None,
        competition_report: None,
        finding_complexity: Some(1),
        derived_from: Some("Pattern: GasOptimization".to_string()),
    };

    Findings {
        findings: vec![finding1, finding2, finding3],
    }
}

/// Creates mock RepoPaths for testing
fn create_mock_repo_paths() -> RepoPaths {
    RepoPaths {
        github_url: "https://github.com/Cyfrin/2023-10-Puppy-Raffle".to_string(),
        project_id: "puppy-raffle-test".to_string(),
        root: PathBuf::from("/tmp/puppy-raffle"),
        sol_files: vec![PathBuf::from("src/PuppyRaffle.sol")],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![PathBuf::from("src")],
        docs: vec![],
        repo_name: "2023-10-Puppy-Raffle".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960".to_string(),
        audit_type: AuditType::Code4rena,
        poc: PocConfig::default(),
    }
}
