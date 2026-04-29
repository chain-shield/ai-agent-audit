use ai_agent_audit::{
    config::AuditType,
    error::Result,
    llm_review::{
        agent::agent_factory::{AgentConfig, AgentFactory, LlmProvider},
        findings::{
            finding_enums::{Severity, VulnerabilityType},
            findings::{Finding, Findings, PrivilegeLevel},
        },
        phases::verify_rounds,
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use rig::providers::anthropic::CLAUDE_4_SONNET;
use std::{path::PathBuf, sync::Arc};

/// Integration test for single-round verification with real AI
/// This test requires ANTHROPIC_API_KEY to be set
#[tokio::test]
#[ignore = "requires live Anthropic credentials"]
async fn test_all_round_integration_with_ai() -> Result<()> {
    // Skip if no API key
    if std::env::var("ANTHROPIC_API_KEY").is_err() {
        println!("⏭️  Skipping integration test - ANTHROPIC_API_KEY not set");
        return Ok(());
    }

    println!("\n🧪 Starting All-Round Integration Test with Real AI\n");

    // Create test findings - mix of valid and invalid
    let findings = create_test_findings();

    println!(
        "📋 Created {} test findings for verification\n",
        findings.findings.len()
    );

    for (i, finding) in findings.findings.iter().enumerate() {
        println!(
            "  {}. [{}] {} - {}",
            i + 1,
            finding.severity,
            finding.title,
            finding.exploit_type
        );
    }

    // Create minimal RepoPaths for testing
    let repo = create_mock_repo();

    // Create mock code and context
    let code_with_context = create_test_code_context();
    let audit_scope = "All findings are in scope for this test audit.";

    // Create agent
    let agent_config = AgentConfig::new(Some(repo.clone()))
        .with_model(CLAUDE_4_SONNET)
        .with_temperature(0.0)
        .with_max_tokens(16_000);

    let agent = Arc::new(AgentFactory::create_agent(
        LlmProvider::Anthropic,
        &agent_config,
    )?);
    println!("✅ Agent initialized\n");

    // Run all-round verification
    println!("🔍 Starting All-Round Verification...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let verified_findings =
        verify_rounds::run_all_round(findings.clone(), &code_with_context, audit_scope, &agent)
            .await?;

    // Display results
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 VERIFICATION RESULTS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut valid_count = 0;
    let mut invalid_count = 0;

    for finding in verified_findings.findings.iter() {
        if finding.status.is_none() {
            valid_count += 1;
            println!("  ✅ VALID: {}", finding.title);
            if let Some(justification) = &finding.status_justification {
                println!("     Justification: {}", justification);
            }
        } else {
            invalid_count += 1;
            println!("  ❌ INVALID: {}", finding.title);
            println!("     Status: {:?}", finding.status);
            if let Some(justification) = &finding.status_justification {
                println!("     Justification: {}", justification);
            }
        }
        println!();
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "  Total findings verified: {}",
        verified_findings.findings.len()
    );
    println!("  ✅ Valid: {}", valid_count);
    println!("  ❌ Invalid: {}", invalid_count);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Assertions
    assert_eq!(
        verified_findings.findings.len(),
        findings.findings.len(),
        "Should have same number of findings after verification"
    );

    // At least some findings should be processed
    assert!(
        valid_count > 0 || invalid_count > 0,
        "Verification should have processed at least some findings"
    );

    // Verify that all findings have either status or justification
    for finding in &verified_findings.findings {
        assert!(
            finding.status.is_some() || finding.status_justification.is_some(),
            "Each finding should have status or justification"
        );
    }

    println!("✅ All-Round Integration Test Passed!\n");

    Ok(())
}

/// Creates test findings for verification
fn create_test_findings() -> Findings {
    let findings = vec![
        // Finding 1: Valid reentrancy vulnerability
        Finding {
            id: Some("H-1".to_string()),
            title: "Reentrancy vulnerability in withdraw function".to_string(),
            exploit_type: VulnerabilityType::Reentrancy,
            privilege: PrivilegeLevel::Permissionless,
            contract: "Vault".to_string(),
            function: "withdraw(uint256)".to_string(),
            description: Some(
                "The withdraw function makes an external call before updating state, allowing reentrancy attacks.".to_string()
            ),
            impact: Some(
                "An attacker can drain all funds from the contract by reentering the withdraw function.".to_string()
            ),
            proof_of_concept: Some(
                "Deploy malicious contract, deposit funds, call withdraw, reenter in receive() function.".to_string()
            ),
            proof_of_code: Some("contract Attacker { receive() external payable { vault.withdraw(amount); } }".to_string()),
            severity: Severity::High,
            mitigation: Some("Follow checks-effects-interactions pattern. Update state before external call.".to_string()),
            status: None,
            status_justification: None,
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            competition_report: None,
            finding_complexity: Some(3),
            derived_from: Some("Reentrancy Pattern".to_string()),
        },

        // Finding 2: Invalid - bug doesn't exist
        Finding {
            id: Some("H-2".to_string()),
            title: "Fake integer overflow vulnerability".to_string(),
            exploit_type: VulnerabilityType::IntegerMath,
            privilege: PrivilegeLevel::Permissionless,
            contract: "Token".to_string(),
            function: "transfer(address,uint256)".to_string(),
            description: Some("Claims integer overflow in transfer function.".to_string()),
            impact: Some("Could allow unlimited token minting.".to_string()),
            proof_of_concept: Some("Transfer max uint256 amount.".to_string()),
            proof_of_code: Some("token.transfer(recipient, type(uint256).max);".to_string()),
            severity: Severity::High,
            mitigation: Some("Use SafeMath library.".to_string()),
            status: None,
            status_justification: None,
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            competition_report: None,
            finding_complexity: Some(2),
            derived_from: Some("Integer Math Pattern".to_string()),
        },

        // Finding 3: Invalid - out of scope (centralization)
        Finding {
            id: Some("M-1".to_string()),
            title: "Owner can pause contract at any time".to_string(),
            exploit_type: VulnerabilityType::AccessControl,
            privilege: PrivilegeLevel::RequiresAdminRole,
            contract: "Vault".to_string(),
            function: "pause()".to_string(),
            description: Some("Owner has unrestricted ability to pause the contract.".to_string()),
            impact: Some("Owner can prevent all user withdrawals.".to_string()),
            proof_of_concept: Some("Owner calls pause() function.".to_string()),
            proof_of_code: Some("owner.pause();".to_string()),
            severity: Severity::Medium,
            mitigation: Some("Add timelock or multi-sig for pause function.".to_string()),
            status: None,
            status_justification: None,
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            competition_report: None,
            finding_complexity: Some(1),
            derived_from: Some("Access Control Pattern".to_string()),
        },
    ];

    Findings { findings }
}

/// Creates mock RepoPaths for testing
fn create_mock_repo() -> RepoPaths {
    RepoPaths {
        github_url: "https://github.com/test/vault.git".to_string(),
        project_id: "test-vault-all-round".to_string(),
        root: PathBuf::from("/tmp/test-vault"),
        sol_files: vec![
            PathBuf::from("src/Vault.sol"),
            PathBuf::from("src/Token.sol"),
        ],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![PathBuf::from("src")],
        docs: vec![],
        repo_name: "test-vault".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "abc123".to_string(),
        audit_type: AuditType::Code4rena,
        poc: PocConfig::default(),
    }
}

/// Creates mock code context for testing
fn create_test_code_context() -> String {
    r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Vault {
    mapping(address => uint256) public balances;

    function deposit() external payable {
        balances[msg.sender] += msg.value;
    }

    function withdraw(uint256 amount) external {
        require(balances[msg.sender] >= amount, "Insufficient balance");

        // VULNERABLE: External call before state update
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");

        balances[msg.sender] -= amount;
    }

    function pause() external onlyOwner {
        _pause();
    }
}

contract Token {
    mapping(address => uint256) public balances;

    // Solidity 0.8+ has built-in overflow protection
    function transfer(address to, uint256 amount) external {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
}
"#
    .to_string()
}
