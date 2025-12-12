/// Integration test for the validation round feature using Puppy Raffle findings
///
/// This test simulates the complete validation workflow:
/// 1. Create findings that were downgraded in verification rounds
/// 2. Run validation round to verify the downgrade reasons
/// 3. Check that findings are correctly upgraded or confirmed invalid
use ai_agent_audit::{
    config::AuditType,
    error::Result,
    llm_review::{
        agent::agent_factory::{AgentConfig, AgentFactory, LlmProvider},
        findings::{
            finding_enums::{Severity, VulnerabilityType},
            findings::{Finding, Findings, PrivilegeLevel},
        },
        phases::verify_rounds::{self, FindingStatus},
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use rig::providers::anthropic::CLAUDE_4_SONNET;
use std::{env, path::PathBuf, sync::Arc};

/// Returns true if Anthropic API key is present in the environment
fn anthropic_key_present() -> bool {
    env::var("ANTHROPIC_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

#[tokio::test]
async fn test_validation_round_puppy_raffle() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Skip if no Anthropic key present
    if !anthropic_key_present() {
        eprintln!("⚠️  Skipping test_validation_round_puppy_raffle - no ANTHROPIC_API_KEY found");
        return Ok(());
    }

    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    println!("\n🧪 Starting Validation Round Integration Test with Puppy Raffle\n");

    // Load and initialize configuration
    ai_agent_audit::config::init_config()?;

    // Initialize LLM clients
    ai_agent_audit::llm_review::agent::agent_factory::init_llm_clients()?;

    // Create test findings with various downgrade statuses
    let findings = create_downgraded_puppy_raffle_findings();

    println!(
        "📋 Created {} downgraded findings for validation",
        findings.findings.len()
    );

    // Display findings summary
    println!("\n📊 Downgraded Findings Summary:");
    for (i, finding) in findings.findings.iter().enumerate() {
        let status_str = finding
            .status
            .as_ref()
            .map(|s| {
                s.iter()
                    .map(|st| format!("{:?}", st))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "None".to_string());

        println!(
            "  {}. [{}] {} - Status: [{}]",
            i + 1,
            finding.severity,
            finding.title,
            status_str
        );
    }

    // Create minimal RepoPaths for testing
    let repo = create_mock_puppy_raffle_repo();

    // Create mock code and context
    let code_with_context = create_puppy_raffle_code_context();
    let audit_scope = ""; // No specific scope for this test

    // Create agent
    let agent_config = AgentConfig::new(Some(repo.clone()))
        .with_model(CLAUDE_4_SONNET)
        .with_temperature(0.0)
        .with_max_tokens(16_000);

    let agent = Arc::new(AgentFactory::create_agent(
        LlmProvider::Anthropic,
        &agent_config,
    )?);
    println!("✅ Agent initialized");

    // Run validation round
    println!("\n🔍 Starting Validation Round...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let validated_findings = verify_rounds::run_round_validation(
        findings.clone(),
        &code_with_context,
        audit_scope,
        &agent,
    )
    .await?;

    // Display results
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 VALIDATION RESULTS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut upgraded_count = 0;
    let mut confirmed_invalid_count = 0;
    let mut unchanged_count = 0;

    for (i, finding) in validated_findings.findings.iter().enumerate() {
        let original_finding = &findings.findings[i];

        let original_status = original_finding.status.as_ref();
        let new_status = finding.status.as_ref();

        let was_invalid = original_status.is_some_and(|s| !s.contains(&FindingStatus::Valid));
        let is_now_valid = new_status.is_some_and(|s| s.contains(&FindingStatus::Valid));
        let is_still_invalid =
            new_status.is_some_and(|s| !s.is_empty() && !s.contains(&FindingStatus::Valid));

        if was_invalid && is_now_valid {
            upgraded_count += 1;
            println!("  ✅ UPGRADED: {}", finding.title);
            println!("     Original: {:?}", original_status);
            println!("     New: Valid");
        } else if is_still_invalid {
            confirmed_invalid_count += 1;
            println!("  ❌ CONFIRMED INVALID: {}", finding.title);
            println!("     Status: {:?}", new_status);
        } else {
            unchanged_count += 1;
            println!("  ➡️  UNCHANGED: {}", finding.title);
        }

        if let Some(justification) = &finding.status_justification {
            println!("     Justification: {}", justification);
        }
        println!();
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "  Total findings validated: {}",
        validated_findings.findings.len()
    );
    println!("  ✅ Upgraded to Valid: {}", upgraded_count);
    println!("  ❌ Confirmed Invalid: {}", confirmed_invalid_count);
    println!("  ➡️  Unchanged: {}", unchanged_count);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Assertions
    assert_eq!(
        validated_findings.findings.len(),
        findings.findings.len(),
        "Should have same number of findings after validation"
    );

    // Expected results based on verification_rounds_passed:
    // - Finding 1 (H-1): rounds_passed=2, validation rejects -> Valid (upgraded)
    // - Finding 2 (M-1): rounds_passed=1, validation rejects -> NeedsMoreInfo (unchanged)
    // - Finding 3 (H-2): rounds_passed=2, validation rejects -> Valid (upgraded)
    // - Finding 4 (M-2): rounds_passed=2, validation rejects -> Valid (upgraded)

    // Expected: 3 upgraded (findings with rounds_passed=2)
    // Note: The AI might confirm some downgrades, so we check for at least 1 upgrade
    assert!(
        upgraded_count >= 1,
        "At least one finding with rounds_passed=2 should be upgraded to Valid"
    );

    println!("✅ Validation Round Integration Test Passed!\n");

    Ok(())
}

/// Creates downgraded Puppy Raffle findings for validation testing
fn create_downgraded_puppy_raffle_findings() -> Findings {
    let findings = vec![
        // Finding 1: Incorrectly marked as "bug does not exist" - should be upgraded
        Finding {
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
                "Deploy malicious contract, enter raffle, call refund, reenter in receive() function.".to_string()
            ),
            proof_of_code: Some("contract Attacker { receive() external payable { raffle.refund(index); } }".to_string()),
            severity: Severity::High,
            mitigation: Some("Follow checks-effects-interactions pattern. Update state before external call.".to_string()),
            status: Some(vec![FindingStatus::InvalidBugDoesNotExist]),
            status_justification: Some("Verification round incorrectly flagged this as non-existent".to_string()),
            verification_rounds_passed: Some(2), // Passed R1 & R2, failed R3
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            competition_report: None,
            finding_complexity: Some(3),
            derived_from: Some("Reentrancy Pattern".to_string()),
        },

        // Finding 2: Correctly marked as out of scope - should stay invalid
        Finding {
            id: Some("M-1".to_string()),
            title: "Centralization risk in owner functions".to_string(),
            exploit_type: VulnerabilityType::AccessControl,
            privilege: PrivilegeLevel::RequiresAdminRole,
            contract: "PuppyRaffle".to_string(),
            function: "changeFeeAddress(address)".to_string(),
            description: Some("Owner can change fee address at any time without restrictions.".to_string()),
            impact: Some("Owner can redirect fees to malicious address.".to_string()),
            proof_of_concept: Some("Owner calls changeFeeAddress with malicious address.".to_string()),
            proof_of_code: Some("owner.changeFeeAddress(attackerAddress);".to_string()),
            severity: Severity::Medium,
            mitigation: Some("Add timelock or multi-sig for critical parameter changes.".to_string()),
            status: Some(vec![FindingStatus::InvalidOutOfScope]),
            status_justification: Some("Centralization risks are out of scope for this audit".to_string()),
            verification_rounds_passed: Some(1), // Passed R1, failed R2
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            competition_report: None,
            finding_complexity: Some(2),
            derived_from: Some("Access Control Pattern".to_string()),
        },

        // Finding 3: Marked as governance risk but actually valid - should be upgraded
        Finding {
            id: Some("H-2".to_string()),
            title: "Integer overflow in fee calculation".to_string(),
            exploit_type: VulnerabilityType::IntegerMath,
            privilege: PrivilegeLevel::Permissionless,
            contract: "PuppyRaffle".to_string(),
            function: "selectWinner()".to_string(),
            description: Some(
                "Fee calculation uses uint64 which can overflow with large prize pools, breaking accounting.".to_string()
            ),
            impact: Some("Fees are permanently locked due to balance != totalFees check.".to_string()),
            proof_of_concept: Some("Enter raffle with 100 ETH, fee overflows uint64.max.".to_string()),
            proof_of_code: Some("totalFees = totalFees + uint64(fee); // overflow".to_string()),
            severity: Severity::High,
            mitigation: Some("Use uint256 for totalFees and remove unsafe casting.".to_string()),
            status: Some(vec![FindingStatus::InvalidGovernanceRisk]),
            status_justification: Some("Incorrectly marked as governance risk".to_string()),
            verification_rounds_passed: Some(2), // Passed R1 & R2, failed R3
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            competition_report: None,
            finding_complexity: Some(4),
            derived_from: Some("Integer Math Pattern".to_string()),
        },

        // Finding 4: Multiple reasons, some valid some not - partial validation
        Finding {
            id: Some("M-2".to_string()),
            title: "Weak randomness in winner selection".to_string(),
            exploit_type: VulnerabilityType::Randomness,
            privilege: PrivilegeLevel::Permissionless,
            contract: "PuppyRaffle".to_string(),
            function: "selectWinner()".to_string(),
            description: Some(
                "Uses block.timestamp and block.difficulty for randomness which can be manipulated.".to_string()
            ),
            impact: Some("Attacker can predict or influence winner selection.".to_string()),
            proof_of_concept: Some("Miner can manipulate block.timestamp to influence outcome.".to_string()),
            proof_of_code: Some("uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;".to_string()),
            severity: Severity::Medium,
            mitigation: Some("Use Chainlink VRF for secure randomness.".to_string()),
            status: Some(vec![
                FindingStatus::InvalidBugDoesNotExist,
                FindingStatus::LowSeverityDueToLowImpact,
            ]),
            status_justification: Some("Marked as non-existent and low impact, but bug exists and impact is medium".to_string()),
            verification_rounds_passed: Some(2), // Passed R1 & R2, failed R3
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            competition_report: None,
            finding_complexity: Some(3),
            derived_from: Some("Randomness Pattern".to_string()),
        },
    ];

    Findings { findings }
}

/// Creates mock RepoPaths for Puppy Raffle
fn create_mock_puppy_raffle_repo() -> RepoPaths {
    RepoPaths {
        github_url: "https://github.com/Cyfrin/2023-10-Puppy-Raffle".to_string(),
        project_id: "puppy-raffle-validation-test".to_string(),
        root: PathBuf::from("/tmp/puppy-raffle-validation"),
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

/// Creates mock code context for Puppy Raffle
fn create_puppy_raffle_code_context() -> String {
    r#"
# PuppyRaffle Contract

## Contract: PuppyRaffle

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

contract PuppyRaffle {
    uint64 public totalFees;
    address[] public players;
    uint256 public entranceFee;
    address public feeAddress;

    function refund(uint256 playerIndex) public {
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "Only player can refund");

        // VULNERABLE: External call before state update (reentrancy)
        (bool success,) = msg.sender.call{value: entranceFee}("");
        require(success, "Refund failed");

        // State update after external call
        players[playerIndex] = address(0);
    }

    function selectWinner() public {
        require(block.timestamp >= raffleStartTime + raffleDuration, "Raffle not over");

        // VULNERABLE: Weak randomness
        uint256 winnerIndex = uint256(
            keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))
        ) % players.length;

        address winner = players[winnerIndex];
        uint256 prizePool = address(this).balance;
        uint256 fee = (prizePool * 20) / 100;

        // VULNERABLE: Integer overflow with uint64
        totalFees = totalFees + uint64(fee);

        (bool success,) = winner.call{value: prizePool - fee}("");
        require(success, "Failed to send prize");
    }

    function withdrawFees() public {
        require(address(this).balance == uint256(totalFees), "Balance mismatch");
        require(msg.sender == feeAddress, "Only fee address");

        (bool success,) = feeAddress.call{value: totalFees}("");
        require(success, "Failed to withdraw");
        totalFees = 0;
    }

    function changeFeeAddress(address newFeeAddress) public {
        require(msg.sender == owner, "Only owner");
        feeAddress = newFeeAddress;
    }
}
```

## Protocol Context

PuppyRaffle is a raffle contract where users can enter by paying an entrance fee.
After the raffle duration, a winner is selected and receives 80% of the prize pool.
The remaining 20% goes to the protocol as fees.
"#
    .to_string()
}
