/// Unit tests for file summary database operations
///
/// This test verifies:
/// 1. Database schema creation
/// 2. Single file summary insertion
/// 3. Batch file summary insertion
/// 4. File summary retrieval (single and batch)
/// 5. Contract category serialization/deserialization
/// 6. File type serialization/deserialization
use ai_agent_audit::{
    build_brain::{
        summarize::{FileSummaryType, SrcFileSummary},
        summarize_db::{
            get_file_summary_from_db, get_summaries_from_db, insert_file_summaries_to_db,
        },
    },
    config::AuditType,
    llm_review::contract::contract_category::ContractCategory,
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use std::path::PathBuf;

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
fn test_single_file_summary_insert_and_retrieve() {
    let repo = create_test_repo_paths("test-single-insert");

    // Create a test summary
    let summary = SrcFileSummary {
        filename: "src/TestContract.sol".to_string(),
        summary: "A test contract for unit testing database operations.".to_string(),
        contract_category: Some(ContractCategory::ERC20Token),
        file_type: Some(FileSummaryType::Source),
    };

    // Insert the summary (using batch insert with single element)
    insert_file_summaries_to_db(std::slice::from_ref(&summary), &repo)
        .expect("Failed to insert summary");

    // Retrieve the summary
    let retrieved = get_file_summary_from_db(&summary.filename, &repo)
        .expect("Failed to retrieve summary")
        .expect("Summary not found in database");

    // Verify all fields match
    assert_eq!(summary.filename, retrieved.filename);
    assert_eq!(summary.summary, retrieved.summary);
    assert_eq!(summary.contract_category, retrieved.contract_category);
    assert_eq!(summary.file_type, retrieved.file_type);

    println!("✅ Single file summary insert and retrieve test passed");
}

#[test]
fn test_batch_file_summaries_insert_and_retrieve() {
    let repo = create_test_repo_paths("test-batch-insert");

    // Create multiple test summaries with different categories
    let summaries = vec![
        SrcFileSummary {
            filename: "src/Token.sol".to_string(),
            summary: "An ERC20 token implementation.".to_string(),
            contract_category: Some(ContractCategory::ERC20Token),
            file_type: Some(FileSummaryType::Source),
        },
        SrcFileSummary {
            filename: "src/NFT.sol".to_string(),
            summary: "An NFT collection contract.".to_string(),
            contract_category: Some(ContractCategory::NFTCollection),
            file_type: Some(FileSummaryType::Source),
        },
        SrcFileSummary {
            filename: "src/Vault.sol".to_string(),
            summary: "A vault with share-based accounting.".to_string(),
            contract_category: Some(ContractCategory::VaultShareBased),
            file_type: Some(FileSummaryType::Source),
        },
        SrcFileSummary {
            filename: "src/Oracle.sol".to_string(),
            summary: "A price oracle for external data feeds.".to_string(),
            contract_category: Some(ContractCategory::OraclePriceFeed),
            file_type: Some(FileSummaryType::Source),
        },
        SrcFileSummary {
            filename: "script/Deploy.s.sol".to_string(),
            summary: "Deployment script for the protocol.".to_string(),
            contract_category: Some(ContractCategory::Unknown),
            file_type: Some(FileSummaryType::DeployScript),
        },
    ];

    // Insert all summaries
    insert_file_summaries_to_db(&summaries, &repo).expect("Failed to insert summaries");

    // Retrieve all summaries
    let retrieved = get_summaries_from_db(&repo).expect("Failed to retrieve summaries");

    // Verify count matches
    assert_eq!(
        summaries.len(),
        retrieved.len(),
        "Retrieved count should match inserted count"
    );

    // Verify each summary
    for original in &summaries {
        let found = retrieved
            .iter()
            .find(|s| s.filename == original.filename)
            .unwrap_or_else(|| panic!("Summary not found: {}", original.filename));

        assert_eq!(original.filename, found.filename);
        assert_eq!(original.summary, found.summary);
        assert_eq!(original.contract_category, found.contract_category);
        assert_eq!(original.file_type, found.file_type);
    }

    println!("✅ Batch file summaries insert and retrieve test passed");
}

#[test]
fn test_contract_category_serialization() {
    let repo = create_test_repo_paths("test-category-serialization");

    // Test all contract categories
    let categories = [
        ContractCategory::SignatureValidation,
        ContractCategory::VaultShareBased,
        ContractCategory::OraclePriceFeed,
        ContractCategory::MathLibrary,
        ContractCategory::TokenTransferLibrary,
        ContractCategory::GovernanceTimeLock,
        ContractCategory::ProxyUpgradeable,
        ContractCategory::StakingRewards,
        ContractCategory::BridgeCrossChain,
        ContractCategory::AMMDex,
        ContractCategory::LendingBorrowing,
        ContractCategory::FactoryDeployer,
        ContractCategory::ERC20Token,
        ContractCategory::NFTCollection,
        ContractCategory::AirdropDistributor,
        ContractCategory::EscrowVesting,
        ContractCategory::MarketplaceExchange,
        ContractCategory::RandomnessRaffleLottery,
        // New utility library categories
        ContractCategory::ByteManipulationLibrary,
        ContractCategory::EncodingDecodingLibrary,
        ContractCategory::StorageHelperLibrary,
        ContractCategory::ErrorDefinitionLibrary,
        ContractCategory::AccessControlModifier,
        ContractCategory::ReentrancyGuardLibrary,
        ContractCategory::SimulationTestingHelper,
        ContractCategory::Unknown,
    ];

    for (i, category) in categories.iter().enumerate() {
        let summary = SrcFileSummary {
            filename: format!("src/Contract{}.sol", i),
            summary: format!("Test contract for category: {}", category),
            contract_category: Some(*category),
            file_type: Some(FileSummaryType::Source),
        };

        // Insert and retrieve
        insert_file_summaries_to_db(std::slice::from_ref(&summary), &repo)
            .expect("Failed to insert summary");
        let retrieved = get_file_summary_from_db(&summary.filename, &repo)
            .expect("Failed to retrieve summary")
            .expect("Summary not found");

        // Verify category matches
        assert_eq!(
            summary.contract_category, retrieved.contract_category,
            "Category mismatch for {}",
            category
        );
    }

    println!(
        "✅ Contract category serialization test passed for {} categories",
        categories.len()
    );
}

#[test]
fn test_file_type_serialization() {
    let repo = create_test_repo_paths("test-file-type-serialization");

    // Test all file types
    let file_types = [
        FileSummaryType::Source,
        FileSummaryType::DeployScript,
        FileSummaryType::OutOfScope,
    ];

    for (i, file_type) in file_types.iter().enumerate() {
        let summary = SrcFileSummary {
            filename: format!("src/File{}.sol", i),
            summary: format!("Test file for type: {:?}", file_type),
            contract_category: Some(ContractCategory::Unknown),
            file_type: Some(file_type.clone()),
        };

        // Insert and retrieve
        insert_file_summaries_to_db(std::slice::from_ref(&summary), &repo)
            .expect("Failed to insert summary");
        let retrieved = get_file_summary_from_db(&summary.filename, &repo)
            .expect("Failed to retrieve summary")
            .expect("Summary not found");

        // Verify file type matches
        assert_eq!(
            summary.file_type, retrieved.file_type,
            "File type mismatch for {:?}",
            file_type
        );
    }

    println!(
        "✅ File type serialization test passed for {} types",
        file_types.len()
    );
}

#[test]
fn test_duplicate_insert_fails() {
    let repo = create_test_repo_paths("test-duplicate-insert");

    let filename = "src/DuplicateTest.sol".to_string();

    // Insert initial summary
    let initial = SrcFileSummary {
        filename: filename.clone(),
        summary: "Initial summary.".to_string(),
        contract_category: Some(ContractCategory::Unknown),
        file_type: Some(FileSummaryType::Source),
    };
    insert_file_summaries_to_db(std::slice::from_ref(&initial), &repo)
        .expect("Failed to insert initial summary");

    // Try to insert again with same filename - should fail due to PRIMARY KEY constraint
    let duplicate = SrcFileSummary {
        filename: filename.clone(),
        summary: "Duplicate summary.".to_string(),
        contract_category: Some(ContractCategory::ERC20Token),
        file_type: Some(FileSummaryType::Source),
    };
    let result = insert_file_summaries_to_db(&[duplicate], &repo);

    // Verify it fails
    assert!(
        result.is_err(),
        "Duplicate insert should fail due to PRIMARY KEY constraint"
    );

    // Verify original summary is still in database
    let retrieved = get_file_summary_from_db(&filename, &repo)
        .expect("Failed to retrieve summary")
        .expect("Summary not found");

    assert_eq!(initial.summary, retrieved.summary);
    assert_eq!(initial.contract_category, retrieved.contract_category);

    println!("✅ Duplicate insert fails test passed");
}

#[test]
fn test_retrieve_nonexistent_summary() {
    let repo = create_test_repo_paths("test-nonexistent");

    // Try to retrieve a summary that doesn't exist
    let result = get_file_summary_from_db("src/NonExistent.sol", &repo)
        .expect("Database query should succeed");

    assert!(result.is_none(), "Should return None for nonexistent file");

    println!("✅ Retrieve nonexistent summary test passed");
}
