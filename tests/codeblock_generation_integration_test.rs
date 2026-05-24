/// Integration test for codeblock generation.
///
/// This verifies codeblocks are generated for in-scope contracts and retrievable
/// from the codeblock database without relying on deprecated file summaries or
/// contract categories.
use ai_agent_audit::{
    build_brain::enrichment,
    config::AuditType,
    enumerator::{codeblock_db::CodeBlocksDb, codeblocks::generate_codeblock_from_codebase},
    prepare_code::git_clone::RepoPaths,
    utils::remapping::parse_and_store_remappings,
};
use dotenvy::dotenv;
use rusqlite::Connection;
use std::{fs, path::PathBuf};

#[tokio::test]
#[ignore] // Run with: cargo test --test codeblock_generation_integration_test -- --ignored --nocapture
async fn test_codeblock_generation() {
    dotenv().ok();

    println!("\n🚀 Starting Codeblock Generation Integration Test");
    println!("{}", "=".repeat(80));

    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let protocol_root = PathBuf::from(format!("{}/Desktop/Audit/2025-10-sequence", home_dir));
    let repo_name = "2025-10-sequence".to_string();

    if !protocol_root.exists() {
        panic!(
            "❌ Repository not found at: {}\nPlease ensure the Sequence repository is cloned to this location.",
            protocol_root.display()
        );
    }

    let repo_root = protocol_root.parent().unwrap().to_path_buf();
    let sol_files = walkdir::WalkDir::new(&protocol_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().and_then(|s| s.to_str()) == Some("sol")
                && !e.path().to_string_lossy().contains("/test/")
                && !e.path().to_string_lossy().contains("/script/")
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let project_id = format!("sequence-codeblock-test-{}", timestamp);

    let repo = RepoPaths {
        github_url: "https://github.com/0xsequence/wallet-contracts".to_string(),
        project_id: project_id.clone(),
        root: repo_root.clone(),
        sol_files,
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![protocol_root.join("src")],
        docs: vec![],
        repo_name,
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "test-commit".to_string(),
        audit_type: AuditType::Client,
    };

    let remapping_file = protocol_root.join("remappings.txt");
    if remapping_file.exists() {
        parse_and_store_remappings(&remapping_file, &repo.project_id)
            .expect("Failed to parse remappings");
    }

    println!("\n🔨 Building semantic database...");
    let semantics_path = enrichment::build_semantics_db_from_call_graph(repo.clone())
        .await
        .expect("Failed to build semantic database");

    let semantic_db = Connection::open(&semantics_path).expect("Failed to open semantic database");
    let codeblocks_db_path = repo_root.join(format!("{}-codeblocks-test.db", project_id));
    let codeblocks_db =
        CodeBlocksDb::open(&codeblocks_db_path).expect("Failed to create codeblocks database");

    generate_codeblock_from_codebase(&repo, &semantic_db, &codeblocks_db, 3, 150_000)
        .await
        .expect("Failed to generate codeblocks");

    let all_contracts = codeblocks_db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");

    assert!(
        !all_contracts.is_empty(),
        "Codeblock generation should persist at least one contract"
    );
    for (contract, content) in &all_contracts {
        assert!(
            !contract.trim().is_empty(),
            "Contract name should not be empty"
        );
        assert!(
            content.contains("contract")
                || content.contains("library")
                || content.contains("interface"),
            "Codeblock for {contract} should contain Solidity source"
        );
    }

    println!("✅ Generated {} codeblocks", all_contracts.len());

    fs::remove_file(&codeblocks_db_path).ok();
    fs::remove_file(&semantics_path).ok();
}
