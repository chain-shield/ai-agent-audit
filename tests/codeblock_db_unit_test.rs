/// Unit tests for codeblock database operations with ContractCategory support
///
/// This test verifies:
/// 1. Database schema creation with contract_category column
/// 2. Single codeblock insertion with ContractCategory
/// 3. Batch codeblock insertion with different ContractCategories
/// 4. Codeblock retrieval (single and batch) with ContractCategory preservation
/// 5. ContractCategory serialization/deserialization in database
/// 6. get_all_contracts returns correct ContractCategory for each contract
use ai_agent_audit::{
    config::AuditType,
    enumerator::codeblock_db::{CodeBlocksDb, MarkdownCodeblock},
    llm_review::contract::contract_category::ContractCategory,
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use std::path::PathBuf;
use uuid::Uuid;

/// Create a test RepoPaths instance with unique project ID
fn create_test_repo_paths(test_name: &str) -> RepoPaths {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Create unique project ID using test name and timestamp
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
        poc: PocConfig::default(),
    }
}

#[test]
fn test_single_codeblock_insert_and_retrieve() {
    let repo = create_test_repo_paths("test-single-codeblock");

    // Create temporary database
    let db_path = std::env::temp_dir().join(format!("test-codeblock-{}.db", repo.project_id));
    let db = CodeBlocksDb::open(&db_path).expect("Failed to create database");

    // Create a test codeblock with ContractCategory
    let codeblock = MarkdownCodeblock {
        id: Uuid::new_v4().to_string(),
        project_id: repo.project_id.clone(),
        contract: "TestToken".to_string(),
        contract_category: ContractCategory::ERC20Token,
        tokens: 5000,
        content: "// Test ERC20 token contract\ncontract TestToken { }".to_string(),
    };

    // Insert the codeblock
    db.insert_codeblock(&codeblock)
        .expect("Failed to insert codeblock");

    // Retrieve the codeblock
    let retrieved_content = db
        .get_code_for_contract(&codeblock.contract, &repo)
        .expect("Failed to retrieve codeblock");

    // Verify content matches
    assert_eq!(codeblock.content, retrieved_content);

    // Retrieve via get_all_contracts to verify ContractCategory
    let all_contracts = db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");

    assert_eq!(all_contracts.len(), 1);
    let (retrieved_content_2, retrieved_category) =
        all_contracts.get("TestToken").expect("Contract not found");

    assert_eq!(&codeblock.content, retrieved_content_2);
    assert_eq!(&codeblock.contract_category, retrieved_category);

    println!("✅ Single codeblock insert and retrieve test passed");

    // Cleanup
    std::fs::remove_file(db_path).ok();
}

#[test]
fn test_batch_codeblocks_insert_and_retrieve() {
    let repo = create_test_repo_paths("test-batch-codeblocks");

    // Create temporary database
    let db_path = std::env::temp_dir().join(format!("test-codeblock-{}.db", repo.project_id));
    let db = CodeBlocksDb::open(&db_path).expect("Failed to create database");

    // Create multiple test codeblocks with different categories
    let codeblocks = vec![
        MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: "MyToken".to_string(),
            contract_category: ContractCategory::ERC20Token,
            tokens: 5000,
            content: "// ERC20 token\ncontract MyToken { }".to_string(),
        },
        MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: "MyNFT".to_string(),
            contract_category: ContractCategory::NFTCollection,
            tokens: 7500,
            content: "// NFT collection\ncontract MyNFT { }".to_string(),
        },
        MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: "MyProxy".to_string(),
            contract_category: ContractCategory::ProxyUpgradeable,
            tokens: 3000,
            content: "// Upgradeable proxy\ncontract MyProxy { }".to_string(),
        },
        MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: "MathLib".to_string(),
            contract_category: ContractCategory::ByteManipulationLibrary,
            tokens: 2000,
            content: "// Byte manipulation library\nlibrary MathLib { }".to_string(),
        },
    ];

    // Insert all codeblocks
    for codeblock in &codeblocks {
        db.insert_codeblock(codeblock)
            .expect("Failed to insert codeblock");
    }

    // Retrieve all contracts
    let all_contracts = db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");

    // Verify count
    assert_eq!(all_contracts.len(), 4);

    // Verify each contract has correct category and content
    for codeblock in &codeblocks {
        let (retrieved_content, retrieved_category) = all_contracts
            .get(&codeblock.contract)
            .expect(&format!("Contract {} not found", codeblock.contract));

        assert_eq!(&codeblock.content, retrieved_content);
        assert_eq!(&codeblock.contract_category, retrieved_category);
    }

    println!("✅ Batch codeblocks insert and retrieve test passed");

    // Cleanup
    std::fs::remove_file(db_path).ok();
}

#[test]
fn test_contract_category_serialization_in_db() {
    let repo = create_test_repo_paths("test-category-serialization");

    // Create temporary database
    let db_path = std::env::temp_dir().join(format!("test-codeblock-{}.db", repo.project_id));
    let db = CodeBlocksDb::open(&db_path).expect("Failed to create database");

    // Test all 26 contract categories
    let categories = vec![
        ContractCategory::SignatureValidation,
        ContractCategory::OraclePriceFeed,
        ContractCategory::TokenTransferLibrary,
        ContractCategory::GovernanceTimeLock,
        ContractCategory::ProxyUpgradeable,
        ContractCategory::StakingRewards,
        ContractCategory::BridgeCrossChain,
        ContractCategory::AMMDex,
        ContractCategory::LendingBorrowing,
        ContractCategory::FactoryDeployer,
        ContractCategory::MathLibrary,
        ContractCategory::VaultShareBased,
        ContractCategory::ERC20Token,
        ContractCategory::NFTCollection,
        ContractCategory::AirdropDistributor,
        ContractCategory::EscrowVesting,
        ContractCategory::MarketplaceExchange,
        ContractCategory::RandomnessRaffleLottery,
        ContractCategory::ByteManipulationLibrary,
        ContractCategory::EncodingDecodingLibrary,
        ContractCategory::StorageHelperLibrary,
        ContractCategory::ErrorDefinitionLibrary,
        ContractCategory::AccessControlModifier,
        ContractCategory::ReentrancyGuardLibrary,
        ContractCategory::SimulationTestingHelper,
        ContractCategory::Unknown,
    ];

    // Insert a codeblock for each category
    for (i, category) in categories.iter().enumerate() {
        let codeblock = MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: format!("Contract{}", i),
            contract_category: category.clone(),
            tokens: 1000,
            content: format!("// Contract with category {:?}", category),
        };

        db.insert_codeblock(&codeblock)
            .expect(&format!("Failed to insert codeblock for {:?}", category));
    }

    // Retrieve all and verify categories
    let all_contracts = db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");

    assert_eq!(all_contracts.len(), categories.len());

    for (i, expected_category) in categories.iter().enumerate() {
        let contract_name = format!("Contract{}", i);
        let (_, retrieved_category) = all_contracts
            .get(&contract_name)
            .expect(&format!("Contract {} not found", contract_name));

        assert_eq!(
            expected_category, retrieved_category,
            "Category mismatch for {}",
            contract_name
        );
    }

    println!(
        "✅ Contract category serialization test passed for {} categories",
        categories.len()
    );

    // Cleanup
    std::fs::remove_file(db_path).ok();
}

#[test]
fn test_codeblock_update_on_conflict() {
    let repo = create_test_repo_paths("test-codeblock-update");

    // Create temporary database
    let db_path = std::env::temp_dir().join(format!("test-codeblock-{}.db", repo.project_id));
    let db = CodeBlocksDb::open(&db_path).expect("Failed to create database");

    // Insert initial codeblock
    let initial_codeblock = MarkdownCodeblock {
        id: Uuid::new_v4().to_string(),
        project_id: repo.project_id.clone(),
        contract: "UpdateTest".to_string(),
        contract_category: ContractCategory::ERC20Token,
        tokens: 5000,
        content: "// Initial version".to_string(),
    };

    db.insert_codeblock(&initial_codeblock)
        .expect("Failed to insert initial codeblock");

    // Insert updated codeblock with same project_id and contract (should update)
    let updated_codeblock = MarkdownCodeblock {
        id: Uuid::new_v4().to_string(),
        project_id: repo.project_id.clone(),
        contract: "UpdateTest".to_string(),
        contract_category: ContractCategory::NFTCollection, // Changed category
        tokens: 7500,                                       // Changed tokens
        content: "// Updated version".to_string(),          // Changed content
    };

    db.insert_codeblock(&updated_codeblock)
        .expect("Failed to insert updated codeblock");

    // Retrieve and verify it was updated
    let all_contracts = db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");

    assert_eq!(all_contracts.len(), 1); // Should still be only 1 contract

    let (retrieved_content, retrieved_category) =
        all_contracts.get("UpdateTest").expect("Contract not found");

    assert_eq!(&updated_codeblock.content, retrieved_content);
    assert_eq!(&updated_codeblock.contract_category, retrieved_category);

    println!("✅ Codeblock update on conflict test passed");

    // Cleanup
    std::fs::remove_file(db_path).ok();
}
