use ai_agent_audit::{
    config::{AuditType, init_config},
    llm_review::{
        agent::agent_factory::{AgentConfig, AgentFactory, init_llm_clients},
        findings::{
            finding_enums::{Severity, VulnerabilityType},
            findings::{Finding, Findings, PrivilegeLevel},
        },
        phases::add_poc_findings,
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

/// Integration test for PoC generation workflow using puppy-raffle findings
///
/// This test:
/// 1. Loads real findings from puppy-raffle audit
/// 2. Uses Claude API to generate PoC tests
/// 3. Saves and runs the PoC tests in Docker
/// 4. Verifies the workflow completes without crashing
#[tokio::test]
#[ignore = "requires live Anthropic credentials and a prepared local repo workspace"]
async fn test_poc_workflow_with_puppy_raffle() {
    // Load env
    dotenv().ok();

    // Skip if no Anthropic key present
    if !anthropic_key_present() {
        eprintln!("⚠️  Skipping test_poc_workflow_with_puppy_raffle - no ANTHROPIC_API_KEY found");
        return;
    }

    // Initialize logger (avoid panic if another test already set the logger)
    let _ = env_logger::try_init();

    // Initialize configuration from environment
    init_config().expect("init_config() should succeed when Anthropic API key is present");

    // Initialize LLM clients
    init_llm_clients().expect("init_llm_clients() should succeed");

    // Create test findings based on puppy-raffle audit report
    let findings = create_puppy_raffle_findings();

    println!("📋 Created {} test findings", findings.findings.len());

    // Setup repo paths for puppy-raffle
    let repo = create_puppy_raffle_repo_paths();

    println!("📁 Repo root: {}", repo.root.display());
    println!("📝 PoC instructions: {}", repo.poc.instructions);

    // Create a simple codeblock (in real scenario this would be the full contract code)
    let codeblock = r#"
// PuppyRaffle contract code would be here
// For this test, we're focusing on the PoC generation workflow
"#;

    // Create Claude agent for PoC generation using AgentFactory
    let agent_config = AgentConfig::new(Some(repo.clone()))
        .with_model("claude-3-7-sonnet-20250219")
        .with_temperature(0.3);

    let agent = Arc::new(
        AgentFactory::create_anthropic_agent(&agent_config).expect("Failed to build Claude agent"),
    );

    println!("🤖 Using Claude 3.7 Sonnet for PoC generation");
    println!("🚀 Starting PoC generation workflow...\n");

    // Execute the PoC generation phase
    let result = add_poc_findings::execute(findings, codeblock, &agent, &repo).await;

    match result {
        Ok(findings_with_pocs) => {
            println!("✅ PoC generation workflow completed successfully!");
            println!("📊 Results:");
            for (i, finding) in findings_with_pocs.findings.iter().enumerate() {
                println!("\n  Finding #{}: {}", i + 1, finding.title);
                println!("    Severity: {:?}", finding.severity);
                if let Some(status) = &finding.poc_test_status {
                    println!("    PoC Status: {:?}", status);
                }
                if let Some(file) = &finding.poc_test_file {
                    println!("    PoC File: {}", file.display());
                }
                if let Some(cmd) = &finding.poc_test_command {
                    println!("    PoC Command: {}", cmd);
                }
            }
            println!("\n✅ Test passed - workflow did not crash!");
        }
        Err(e) => {
            println!("❌ PoC generation workflow failed: {:?}", e);
            panic!(
                "PoC workflow should not fail completely, but got error: {:?}",
                e
            );
        }
    }
}

/// Creates test findings based on puppy-raffle audit report
fn create_puppy_raffle_findings() -> Findings {
    let findings = vec![
        // H-2: Fee downcast overflow
        Finding {
            id: Some("H-2".to_string()),
            derived_from: Some("uint64 downcast overflows fees; breaks balance==totalFees invariant and locks fees".to_string()),
            title: "Fee downcast in PuppyRaffle.selectWinner truncates 20% fee, breaking balance==totalFees invariant and bricking withdrawFees".to_string(),
            exploit_type: VulnerabilityType::IntegerMath,
            privilege: PrivilegeLevel::Permissionless,
            contract: "PuppyRaffle".to_string(),
            function: "selectWinner".to_string(),
            description: Some("totalFees is a uint64 accumulator while fee is computed in uint256 and downcasted without checks. For large pots, (players.length * entranceFee * 20%) exceeds 2^64-1 wei (~18.4467 ETH), so uint64(fee) truncates modulo 2^64.".to_string()),
            impact: Some("Any user can trigger a single-round fee that exceeds 2^64-1 wei (≈18.4467 ETH), causing uint64 truncation of the recorded fees. This permanently bricks withdrawFees and locks all accrued protocol revenue.".to_string()),
            proof_of_concept: Some("1) Attacker supplies N unique players such that 20% of the pot > 2^64-1 wei. For entranceFee = 1 ETH, N ≥ 93 is sufficient (0.2 × 93 ETH > 18.4467 ETH). 2) After duration elapses, call selectWinner(). 3) The contract keeps the full 20% fee in its balance, but totalFees gets truncated by uint64 casting (fee mod 2^64). 4) Any attempt to call withdrawFees reverts because address(this).balance != totalFees, permanently locking fees.".to_string()),
            proof_of_code: None,
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            severity: Severity::High,
            mitigation: Some("Change totalFees from uint64 to uint256 to match the fee calculation type.".to_string()),
            status: None,
            status_justification: Some("High severity because it permanently locks protocol revenue".to_string()),
            competition_report: None,
            finding_complexity: Some(4),
        },
        // H-3: Reentrancy
        Finding {
            id: Some("H-3".to_string()),
            derived_from: Some("Refund reentrancy drains multiple tickets due to external call before state update".to_string()),
            title: "PuppyRaffle.refund reentrancy drains entire pot via Address.sendValue before zeroing player slot".to_string(),
            exploit_type: VulnerabilityType::Reentrancy,
            privilege: PrivilegeLevel::Permissionless,
            contract: "PuppyRaffle".to_string(),
            function: "refund".to_string(),
            description: Some("The refund function calls Address.sendValue before updating the players array, allowing reentrancy attacks to drain the contract.".to_string()),
            impact: Some("Attacker can drain the entire raffle pot by reentering the refund function before their player slot is zeroed.".to_string()),
            proof_of_concept: Some("1) Attacker enters raffle with a malicious contract. 2) Attacker calls refund(). 3) In the receive() fallback, attacker reenters refund() before players[playerIndex] is set to address(0). 4) Repeat until pot is drained.".to_string()),
            proof_of_code: None,
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            severity: Severity::High,
            mitigation: Some("Follow checks-effects-interactions pattern: update players[playerIndex] = address(0) before calling sendValue.".to_string()),
            status: None,
            status_justification: Some("High severity because it allows complete drainage of raffle funds".to_string()),
            competition_report: None,
            finding_complexity: Some(3),
        },
    ];

    Findings { findings }
}

/// Creates RepoPaths configuration for puppy-raffle test
fn create_puppy_raffle_repo_paths() -> RepoPaths {
    let root = PathBuf::from("/private/tmp/audit-analysis/4-puppy-raffle-audit-3ff0f0");
    let repo_name = "4-puppy-raffle-audit".to_string();

    // Read PoC instructions from puppy-test.md
    let instructions = std::fs::read_to_string("puppy-test.md")
        .unwrap_or_else(|_| {
            "Write a runnable PoC, will be saved to /test folder. PoC should rigorously demonstrate. Do not use mock contracts, unless you have no choice.".to_string()
        });

    let poc_config = PocConfig {
        instructions,
        test_folder: root.join(&repo_name).join("test"),
        template: String::new(), // No template for this test
    };

    // Include the main contract file for Solidity version detection
    let puppy_raffle_contract = root.join(&repo_name).join("src").join("PuppyRaffle.sol");

    RepoPaths {
        github_url: "https://github.com/Cyfrin/2023-10-Puppy-Raffle".to_string(),
        project_id: "puppy-raffle-test".to_string(),
        root: root.clone(),
        sol_files: vec![puppy_raffle_contract], // Include main contract for version detection
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![],
        docs: vec![],
        repo_name,
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960".to_string(),
        audit_type: AuditType::Code4rena,
        poc: poc_config,
    }
}
