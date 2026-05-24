/// Unit tests for codeblock database operations.
///
/// These tests verify insertion, retrieval, project isolation, and conflict
/// updates for generated codeblocks. Contract categories are intentionally not
/// part of the codeblock model anymore.
use ai_agent_audit::{
    config::AuditType,
    enumerator::codeblock_db::{CodeBlocksDb, MarkdownCodeblock},
    prepare_code::git_clone::RepoPaths,
};
use std::path::PathBuf;
use uuid::Uuid;

fn create_test_repo_paths(test_name: &str) -> RepoPaths {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let project_id = format!("{}-{}", test_name, timestamp);

    RepoPaths {
        github_url: "https://github.com/test/test-repo".to_string(),
        project_id,
        root: PathBuf::from("/tmp/test"),
        sol_files: vec![],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![],
        docs: vec![],
        repo_name: "test-repo".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "test".to_string(),
        audit_type: AuditType::Client,
    }
}

fn temp_db(repo: &RepoPaths) -> (PathBuf, CodeBlocksDb) {
    let db_path = std::env::temp_dir().join(format!("test-codeblock-{}.db", repo.project_id));
    let db = CodeBlocksDb::open(&db_path).expect("Failed to create database");
    (db_path, db)
}

fn codeblock(repo: &RepoPaths, contract: &str, tokens: usize, content: &str) -> MarkdownCodeblock {
    MarkdownCodeblock {
        id: Uuid::new_v4().to_string(),
        project_id: repo.project_id.clone(),
        contract: contract.to_string(),
        tokens,
        content: content.to_string(),
    }
}

#[test]
fn test_single_codeblock_insert_and_retrieve() {
    let repo = create_test_repo_paths("test-single-codeblock");
    let (db_path, db) = temp_db(&repo);

    let codeblock = codeblock(
        &repo,
        "TestToken",
        5000,
        "// Test ERC20 token contract\ncontract TestToken { }",
    );

    db.insert_codeblock(&codeblock)
        .expect("Failed to insert codeblock");

    let retrieved_content = db
        .get_code_for_contract(&codeblock.contract, &repo)
        .expect("Failed to retrieve codeblock");
    assert_eq!(codeblock.content, retrieved_content);

    let all_contracts = db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");
    assert_eq!(all_contracts.len(), 1);
    assert_eq!(
        all_contracts.get("TestToken"),
        Some(&codeblock.content),
        "get_all_contracts should return content by contract name"
    );

    std::fs::remove_file(db_path).ok();
}

#[test]
fn test_batch_codeblocks_insert_and_retrieve() {
    let repo = create_test_repo_paths("test-batch-codeblocks");
    let (db_path, db) = temp_db(&repo);

    let codeblocks = vec![
        codeblock(
            &repo,
            "MyToken",
            5000,
            "// ERC20 token\ncontract MyToken { }",
        ),
        codeblock(
            &repo,
            "MyNFT",
            7500,
            "// NFT collection\ncontract MyNFT { }",
        ),
        codeblock(
            &repo,
            "MyProxy",
            3000,
            "// Upgradeable proxy\ncontract MyProxy { }",
        ),
        codeblock(
            &repo,
            "MathLib",
            2000,
            "// Math library\nlibrary MathLib { }",
        ),
    ];

    for codeblock in &codeblocks {
        db.insert_codeblock(codeblock)
            .expect("Failed to insert codeblock");
    }

    let all_contracts = db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");
    assert_eq!(all_contracts.len(), codeblocks.len());

    for codeblock in &codeblocks {
        assert_eq!(
            all_contracts.get(&codeblock.contract),
            Some(&codeblock.content),
            "Contract {} not found or content mismatch",
            codeblock.contract
        );
    }

    std::fs::remove_file(db_path).ok();
}

#[test]
fn test_codeblock_update_on_conflict() {
    let repo = create_test_repo_paths("test-codeblock-update");
    let (db_path, db) = temp_db(&repo);

    let initial_codeblock = codeblock(&repo, "UpdateTest", 5000, "// Initial version");
    db.insert_codeblock(&initial_codeblock)
        .expect("Failed to insert initial codeblock");

    let updated_codeblock = codeblock(&repo, "UpdateTest", 7500, "// Updated version");
    db.insert_codeblock(&updated_codeblock)
        .expect("Failed to insert updated codeblock");

    let all_contracts = db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");
    assert_eq!(all_contracts.len(), 1);
    assert_eq!(
        all_contracts.get("UpdateTest"),
        Some(&updated_codeblock.content)
    );

    std::fs::remove_file(db_path).ok();
}
