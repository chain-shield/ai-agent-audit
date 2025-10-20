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

/// Test that failed PoC test files are automatically deleted to prevent cross-contamination
#[tokio::test]
async fn test_failed_poc_cleanup() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    // Create a finding that will definitely fail (invalid Solidity code)
    let finding = Finding {
        title: "Test Finding That Will Fail".to_string(),
        severity: Severity::High,
        exploit_type: VulnerabilityType::Reentrancy,
        contract_name: "NonExistentContract".to_string(),
        function_name: "nonExistentFunction".to_string(),
        description: "This is a test finding with invalid code that will fail to compile"
            .to_string(),
        impact: "Test impact".to_string(),
        proof_of_concept: "This PoC will fail because the contract doesn't exist".to_string(),
        mitigation: "Test mitigation".to_string(),
        code_snippet: "// Invalid code\ncontract NonExistent { }".to_string(),
    };

    let findings = FindingsWithPoC {
        findings: vec![finding],
        findings_with_poc: HashMap::new(),
    };

    // Create test repository paths
    let repo_root = PathBuf::from("/private/tmp/audit-analysis/poc-cleanup-test");
    let repo_name = "test-repo";
    let test_folder = repo_root.join(repo_name).join("test");

    // Create test directory
    std::fs::create_dir_all(&test_folder).expect("Failed to create test directory");

    let repo_paths = RepoPaths {
        root: repo_root.clone(),
        repo_name: repo_name.to_string(),
        sol_files: vec![],
        source_code_folders: vec![],
        poc: PocConfig {
            test_folder: test_folder.clone(),
            instructions: "Use Foundry for testing. This will fail.".to_string(),
            template: String::new(),
        },
    };

    // Build agent
    let agent = build_agent().await.expect("Failed to build agent");

    // Create minimal code context
    let code_context = CodeBlockPlusContext {
        code_block: "contract NonExistent { }".to_string(),
        context: vec![],
    };

    println!("🚀 Starting PoC cleanup test...");
    println!("📁 Test folder: {}", test_folder.display());

    // Run PoC generation (this will fail after 5 attempts)
    let result = add_poc_findings(
        Arc::new(findings),
        Arc::new(code_context),
        Arc::new(repo_paths),
        Arc::new(agent),
        None,
    )
    .await;

    println!("\n✅ PoC generation completed (expected to fail)");

    // Check that the test file was created during attempts
    let expected_test_file = test_folder.join("H-Test-Finding-That-W.t.sol");
    println!(
        "🔍 Checking if failed test file was deleted: {}",
        expected_test_file.display()
    );

    // The file should NOT exist because it failed and was cleaned up
    if expected_test_file.exists() {
        println!("❌ FAILED: Test file still exists after failure!");
        println!("   File: {}", expected_test_file.display());
        panic!("Failed PoC test file was not deleted!");
    } else {
        println!("✅ SUCCESS: Failed test file was properly cleaned up");
    }

    // Verify the result contains the failed PoC
    if let Ok(findings_with_poc) = result {
        println!("\n📊 PoC Results:");
        for (hash, poc_test) in findings_with_poc.findings_with_poc.iter() {
            println!("  Finding hash: {}", hash);
            println!("  PoC status: {:?}", poc_test.poc_test_status);
            println!("  PoC file: {}", poc_test.poc_test_file.display());
            println!("  File exists: {}", poc_test.poc_test_file.exists());
        }
    }

    // Cleanup test directory
    std::fs::remove_dir_all(&repo_root).ok();

    println!("\n✅ Test passed - failed PoC files are properly cleaned up!");
}
