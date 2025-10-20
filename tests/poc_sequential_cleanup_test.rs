use ai_agent_audit::{
    llm_review::{
        agent_factory::build_agent,
        config::{Finding, Severity, VulnerabilityType},
        phases::add_poc_findings::{FindingsWithPoC, add_poc_findings},
        prompt_support::make_codeblock::CodeBlockPlusContext,
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

/// Test that when PoC #1 fails, it gets cleaned up and doesn't interfere with PoC #2
#[tokio::test]
async fn test_sequential_poc_with_cleanup() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    // Finding #1: Will fail (invalid contract reference)
    let failing_finding = Finding {
        title: "Invalid Finding That Will Fail".to_string(),
        severity: Severity::High,
        exploit_type: VulnerabilityType::Reentrancy,
        contract_name: "NonExistentContract".to_string(),
        function_name: "nonExistentFunction".to_string(),
        description: "This finding references a contract that doesn't exist".to_string(),
        impact: "This will fail to compile".to_string(),
        proof_of_concept: "The contract NonExistentContract doesn't exist in the codebase"
            .to_string(),
        mitigation: "N/A".to_string(),
        code_snippet: "// This contract doesn't exist\ncontract NonExistentContract { }"
            .to_string(),
    };

    // Finding #2: Valid finding (from puppy-raffle)
    let valid_finding = Finding {
        title: "Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS".to_string(),
        severity: Severity::Medium,
        exploit_type: VulnerabilityType::GasGriefBlockLimit,
        contract_name: "PuppyRaffle".to_string(),
        function_name: "enterRaffle".to_string(),
        description: "The enterRaffle function contains an unbounded O(n^2) duplicate check that can cause gas-based DoS".to_string(),
        impact: "As the number of players grows, the gas cost increases quadratically, eventually exceeding block gas limits".to_string(),
        proof_of_concept: "Add 100+ players to demonstrate quadratic gas growth".to_string(),
        mitigation: "Use a mapping to track duplicates in O(1) time".to_string(),
        code_snippet: r#"
function enterRaffle(address[] memory newPlayers) public payable {
    for (uint256 i = 0; i < newPlayers.length; i++) {
        for (uint256 j = 0; j < players.length; j++) {
            require(players[j] != newPlayers[i], "Duplicate player");
        }
    }
}
"#.to_string(),
    };

    let findings = FindingsWithPoC {
        findings: vec![failing_finding, valid_finding],
        findings_with_poc: HashMap::new(),
    };

    // Use the actual puppy-raffle repository
    let repo_root = PathBuf::from("/private/tmp/audit-analysis/4-puppy-raffle-audit-3ff0f0");
    let repo_name = "4-puppy-raffle-audit";
    let test_folder = repo_root.join(&repo_name).join("test");

    // Clean up any existing test files
    if test_folder.exists() {
        for entry in std::fs::read_dir(&test_folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sol") {
                std::fs::remove_file(&path).ok();
            }
        }
    }

    let repo_paths = RepoPaths {
        root: repo_root.clone(),
        repo_name: repo_name.to_string(),
        sol_files: vec![
            repo_root.join(&repo_name).join("src/PuppyRaffle.sol"),
        ],
        source_code_folders: vec![
            repo_root.join(&repo_name).join("src"),
        ],
        poc: PocConfig {
            test_folder: test_folder.clone(),
            instructions: "Use Foundry/Forge for testing. Import from 'forge-std/Test.sol'. The contract uses Solidity 0.7.6.".to_string(),
            template: String::new(),
        },
    };

    // Build agent
    let agent = build_agent().await.expect("Failed to build agent");

    // Load actual contract code
    let contract_code =
        std::fs::read_to_string(repo_root.join(&repo_name).join("src/PuppyRaffle.sol"))
            .expect("Failed to read PuppyRaffle.sol");

    let code_context = CodeBlockPlusContext {
        code_block: contract_code,
        context: vec![],
    };

    println!("🚀 Starting sequential PoC test with cleanup...");
    println!("📁 Test folder: {}", test_folder.display());
    println!("\n📋 Testing scenario:");
    println!("  1. Finding #1 (invalid) - should FAIL and be DELETED");
    println!("  2. Finding #2 (valid) - should PASS without interference from #1");

    // Run PoC generation
    let result = add_poc_findings(
        Arc::new(findings),
        Arc::new(code_context),
        Arc::new(repo_paths),
        Arc::new(agent),
        None,
    )
    .await;

    println!("\n✅ PoC generation completed");

    // Check results
    if let Ok(findings_with_poc) = result {
        println!("\n📊 PoC Results:");

        let mut failed_count = 0;
        let mut passed_count = 0;

        for (i, (hash, poc_test)) in findings_with_poc.findings_with_poc.iter().enumerate() {
            println!(
                "\n  Finding #{}: {}",
                i + 1,
                findings_with_poc.findings[i].title
            );
            println!("    Hash: {}", hash);
            println!("    Status: {:?}", poc_test.poc_test_status);
            println!("    File: {}", poc_test.poc_test_file.display());
            println!("    File exists: {}", poc_test.poc_test_file.exists());

            if poc_test.poc_test_status
                == ai_agent_audit::llm_review::phases::add_poc_findings::PocStatus::AllTestPass
            {
                passed_count += 1;
                assert!(
                    poc_test.poc_test_file.exists(),
                    "Passing PoC file should exist"
                );
            } else {
                failed_count += 1;
                assert!(
                    !poc_test.poc_test_file.exists(),
                    "Failed PoC file should be deleted"
                );
            }
        }

        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 Summary:");
        println!("  ✅ Passed: {}", passed_count);
        println!("  ❌ Failed (and cleaned up): {}", failed_count);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Verify at least one passed and one failed
        assert!(passed_count >= 1, "At least one PoC should pass");
        assert!(
            failed_count >= 1,
            "At least one PoC should fail (for testing cleanup)"
        );
    } else {
        panic!("PoC generation failed: {:?}", result);
    }

    println!(
        "\n✅ Test passed - failed PoCs are cleaned up and don't interfere with subsequent PoCs!"
    );
}
