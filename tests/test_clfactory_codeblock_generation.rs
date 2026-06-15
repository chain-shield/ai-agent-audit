use ai_agent_audit::{
    build_brain::enrichment,
    config::AuditType,
    enumerator::{codeblock_db::CodeBlocksDb, codeblocks::generate_codeblock_from_codebase},
    prepare_code::git_clone::RepoPaths,
};
use std::path::PathBuf;

/// Integration test for CLFactory codeblock generation
///
/// This test uses the actual Hybra Finance repository to verify that
/// codeblock generation detects all expected contracts.
#[tokio::test]
#[ignore] // Ignore by default since it requires the actual repository
async fn test_clfactory_codeblock_generation() {
    // Initialize logger to see debug output
    let _ = env_logger::builder().is_test(true).try_init();

    let Some(repo_root) = std::env::var_os("AI_AGENT_AUDIT_CLFACTORY_TEST_REPO").map(PathBuf::from)
    else {
        println!("Skipping test - set AI_AGENT_AUDIT_CLFACTORY_TEST_REPO to the local Hybra repo");
        return;
    };

    if !repo_root.exists() {
        println!(
            "⏭️  Skipping test - repository not found at: {:?}",
            repo_root
        );
        return;
    }

    println!("✅ Repository found at: {:?}", repo_root);

    // Manually construct RepoPaths for the existing repository
    // This mimics what clone_and_filter_git_repo does but for a local directory
    let repo_name = "2025-10-hybra-finance".to_string();
    let commit_hash = "test-integration-74c2b9".to_string();
    let project_id = format!("{}-{}", repo_name, &commit_hash[..6]);

    // Find all .sol files
    let mut sol_files = Vec::new();
    let mut test_files = Vec::new();
    let mut script_files = Vec::new();

    for entry in walkdir::WalkDir::new(&repo_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("sol") {
            let path_str = path.to_string_lossy();

            if path_str.contains("/test/") || path_str.contains("/tests/") {
                test_files.push(path.to_path_buf());
            } else if path_str.contains("/script/") || path_str.contains("/scripts/") {
                script_files.push(path.to_path_buf());
            }

            sol_files.push(path.to_path_buf());
        }
    }

    let source_code_folders = vec![
        repo_root.join("cl/contracts"),
        repo_root.join("ve33/contracts"),
    ];

    let repo = RepoPaths {
        github_url: format!("https://github.com/test/{}", repo_name),
        project_id: project_id.clone(),
        root: repo_root.parent().unwrap().to_path_buf(),
        sol_files: sol_files.clone(),
        test_files,
        script_files,
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders,
        docs: vec![],
        repo_name: repo_name.clone(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: commit_hash.clone(),
        audit_type: AuditType::Code4rena,
    };

    println!("\n📁 Repository structure:");
    println!("  - Total .sol files: {}", repo.sol_files.len());
    println!("  - Test files: {}", repo.test_files.len());
    println!("  - Script files: {}", repo.script_files.len());

    // Build semantic database
    println!("\n🔨 Building semantic database...");
    let db_path = enrichment::build_semantics_db_from_call_graph(repo.clone())
        .await
        .expect("Failed to build semantic database");

    println!("✅ Semantic database built at: {:?}", db_path);

    // Open the semantic database
    let semantic_db =
        rusqlite::Connection::open(&db_path).expect("Failed to open semantic database");

    // Create codeblocks database
    let codeblocks_db_path = repo_root
        .parent()
        .unwrap()
        .join(format!("{}-codeblocks-test.db", project_id));
    let codeblocks_db =
        CodeBlocksDb::open(&codeblocks_db_path).expect("Failed to create codeblocks database");

    // Generate codeblocks
    println!("\n🔍 Generating codeblocks...");
    let max_depth = 3;
    let token_budget = 115_000;

    generate_codeblock_from_codebase(&repo, &semantic_db, &codeblocks_db, max_depth, token_budget)
        .await
        .expect("Failed to generate codeblocks");

    println!("\n✅ Codeblocks generated successfully!");

    // Query the codeblocks database for CLFactory
    println!("\n🔍 Checking CLFactory codeblock...");

    let clfactory_markdown = codeblocks_db
        .get_code_for_contract("CLFactory", &repo)
        .expect("Failed to query CLFactory codeblock");

    println!("\n📊 CLFactory Codeblock Stats:");
    let token_count = clfactory_markdown.split_whitespace().count(); // Rough estimate
    println!("  - Estimated token count: {}", token_count);
    println!("  - Token budget: {}", token_budget);
    println!(
        "  - Budget used: {:.1}%",
        (token_count as f64 / token_budget as f64) * 100.0
    );
    println!(
        "  - Content length: {} characters",
        clfactory_markdown.len()
    );

    // Count contracts/interfaces/libraries in the codeblock
    let contract_count = clfactory_markdown.matches("\ncontract ").count();
    let interface_count = clfactory_markdown.matches("\ninterface ").count();
    let library_count = clfactory_markdown.matches("\nlibrary ").count();

    println!("\n📊 Contracts/Interfaces/Libraries in codeblock:");
    println!("  - Contracts: {}", contract_count);
    println!("  - Interfaces: {}", interface_count);
    println!("  - Libraries: {}", library_count);
    println!(
        "  - Total: {}",
        contract_count + interface_count + library_count
    );

    // Extract and list all contract/interface/library names
    println!("\n📋 Detected contracts/interfaces/libraries:");
    for line in clfactory_markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("contract ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("library ")
        {
            // Extract just the name
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                println!("  - {} {}", parts[0], parts[1]);
            }
        }
    }

    // Check if deployment script is included
    let has_deployment_script = clfactory_markdown.contains("DEPLOYMENT SCRIPTS");
    println!("\n📜 Deployment script included: {}", has_deployment_script);

    // Assertions
    println!("\n🧪 Running assertions...");

    // Should have more than 11 contracts total
    let total_items = contract_count + interface_count + library_count;
    assert!(
        total_items >= 20,
        "Expected at least 20 contracts/interfaces/libraries, found {}",
        total_items
    );

    // Should use a reasonable amount of the token budget
    assert!(
        token_count > 30_000,
        "Expected token count > 30,000, found {}",
        token_count
    );

    // Should include deployment script
    assert!(
        has_deployment_script,
        "Expected deployment script to be included"
    );

    // Should include key contracts
    assert!(
        clfactory_markdown.contains("contract CLPool")
            || clfactory_markdown.contains("interface ICLPool"),
        "Expected CLPool to be included"
    );

    assert!(
        clfactory_markdown.contains("interface IGaugeManager")
            || clfactory_markdown.contains("contract GaugeManager"),
        "Expected GaugeManager to be included"
    );

    println!("\n✅ All assertions passed!");

    // Clean up
    std::fs::remove_file(&db_path).ok();
    std::fs::remove_file(&codeblocks_db_path).ok();
}
