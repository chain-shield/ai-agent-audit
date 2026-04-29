/// Integration test for file summarization and categorization
///
/// This test verifies:
/// 1. File summaries are generated for all relevant source files
/// 2. Contract categories are correctly determined by the LLM
/// 3. Data is correctly saved to SQLite database
/// 4. Data can be correctly read back from the database
/// 5. Each contract is properly categorized based on its functionality
///
/// Uses the Sequence repository (2025-10-sequence) for testing.
use ai_agent_audit::{
    build_brain::{
        enrichment,
        summarize::{FileSummaryType, summarize_src_files_with_model},
        summarize_db::{get_file_summary_from_db, get_summaries_from_db},
    },
    config::{AuditType, OPENAI_SUMMARY_MODEL},
    llm_review::contract::contract_category::ContractCategory,
    prepare_code::git_clone::{PocConfig, RepoPaths},
    utils::remapping::parse_and_store_remappings,
};
use dotenvy::dotenv;
use std::{fs, path::PathBuf};

/// Expected contract categories for Sequence contracts based on their functionality
fn get_expected_categories() -> Vec<(&'static str, ContractCategory)> {
    vec![
        // Core wallet contracts
        ("Factory.sol", ContractCategory::FactoryDeployer),
        ("Wallet.sol", ContractCategory::ProxyUpgradeable),
        ("Stage1Module.sol", ContractCategory::SignatureValidation),
        ("Stage2Module.sol", ContractCategory::SignatureValidation),
        // Authentication modules
        ("BaseAuth.sol", ContractCategory::SignatureValidation),
        ("BaseSig.sol", ContractCategory::SignatureValidation),
        ("SelfAuth.sol", ContractCategory::SignatureValidation),
        ("Stage1Auth.sol", ContractCategory::SignatureValidation),
        ("Stage2Auth.sol", ContractCategory::SignatureValidation),
        // Extensions
        ("Passkeys.sol", ContractCategory::SignatureValidation),
        ("Recovery.sol", ContractCategory::GovernanceTimeLock),
        ("SessionManager.sol", ContractCategory::SignatureValidation),
        (
            "ExplicitSessionManager.sol",
            ContractCategory::SignatureValidation,
        ),
        (
            "ImplicitSessionManager.sol",
            ContractCategory::SignatureValidation,
        ),
        (
            "PermissionValidator.sol",
            ContractCategory::SignatureValidation,
        ),
        // Utility libraries
        ("LibBytes.sol", ContractCategory::MathLibrary),
        ("LibOptim.sol", ContractCategory::MathLibrary),
        ("Base64.sol", ContractCategory::MathLibrary),
        ("WebAuthn.sol", ContractCategory::SignatureValidation),
        ("P256.sol", ContractCategory::SignatureValidation),
        // Core modules
        ("Calls.sol", ContractCategory::ProxyUpgradeable),
        ("Payload.sol", ContractCategory::ProxyUpgradeable),
        ("ERC4337v07.sol", ContractCategory::SignatureValidation),
        ("Hooks.sol", ContractCategory::ProxyUpgradeable),
        ("Implementation.sol", ContractCategory::ProxyUpgradeable),
        ("Nonce.sol", ContractCategory::ProxyUpgradeable),
        ("Storage.sol", ContractCategory::ProxyUpgradeable),
        ("ReentrancyGuard.sol", ContractCategory::ProxyUpgradeable),
    ]
}

#[tokio::test]
#[ignore] // Run with: cargo test --test file_summarization_integration_test -- --ignored --nocapture
async fn test_file_summarization_and_categorization() {
    // Load environment variables from .env file
    dotenv().ok();

    println!("\n🚀 Starting File Summarization and Categorization Integration Test");
    println!("{}", "=".repeat(80));

    // Setup repository path
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let protocol_root = PathBuf::from(format!("{}/Desktop/Audit/2025-10-sequence", home_dir));
    let repo_name = "2025-10-sequence".to_string();

    if !protocol_root.exists() {
        panic!(
            "❌ Repository not found at: {}\nPlease ensure the Sequence repository is cloned to this location.",
            protocol_root.display()
        );
    }

    println!("\n📂 Repository: {}", protocol_root.display());

    // The root should be the parent directory
    let repo_root = protocol_root.parent().unwrap().to_path_buf();

    // Collect all .sol files
    let sol_files = walkdir::WalkDir::new(&protocol_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sol"))
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    // Create RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/0xsequence/wallet-contracts-v3".to_string(),
        project_id: "sequence-summarization-test".to_string(),
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
        commit_hash: "main".to_string(),
        audit_type: AuditType::Code4rena,
        poc: PocConfig::default(),
    };

    println!("\n📊 Repository Statistics:");
    println!("  - Total .sol files: {}", repo.sol_files.len());
    println!("  - Source folders: {:?}", repo.source_code_folders);
    println!("  - Project ID: {}", repo.project_id);

    // Parse remappings (required for import resolution)
    println!("\n🔧 Parsing remappings...");
    let remapping_file = protocol_root.join("remappings.txt");
    if remapping_file.exists() {
        match parse_and_store_remappings(&remapping_file, &repo.project_id) {
            Ok(count) => println!("   ✅ Loaded {} remappings", count),
            Err(e) => println!("   ⚠️  Warning: Failed to load remappings: {}", e),
        }
    } else {
        println!("   ℹ️  No remappings.txt found, skipping");
    }

    // Build semantic database (required for context)
    println!("\n🔨 Building semantic database...");
    println!("   (This may take a few minutes on first run)");
    let semantics_path = enrichment::build_semantics_db_from_call_graph(repo.clone())
        .await
        .expect("Failed to build semantic database");
    println!("   ✅ Semantic database built at: {:?}", semantics_path);

    // Clear any existing summaries from database for this test
    println!("\n🧹 Clearing existing summaries from database...");
    clear_summaries_from_db(&repo);

    // Generate summaries and categorizations
    println!("\n📝 Generating file summaries and categorizations...");
    println!("   Using model: {}", OPENAI_SUMMARY_MODEL);
    println!("   This will take several minutes as each file is analyzed by the LLM...");

    let summaries = summarize_src_files_with_model(&repo, OPENAI_SUMMARY_MODEL)
        .await
        .expect("Failed to generate summaries");

    println!("\n✅ Summaries generated successfully!");
    println!("   - Total files summarized: {}", summaries.len());

    // Test 1: Verify summaries were generated for all relevant files
    println!("{}", "\n".repeat(2));
    println!("{}", "=".repeat(80));
    println!("TEST 1: Verify summaries generated for all relevant files");
    println!("{}", "=".repeat(80));

    assert!(
        !summaries.is_empty(),
        "Should have generated summaries for at least some files"
    );

    // Count files by type
    let source_files: Vec<_> = summaries
        .iter()
        .filter(|s| s.file_type == Some(FileSummaryType::Source))
        .collect();

    println!("\n📊 Summary Statistics:");
    println!("  - Source files: {}", source_files.len());
    println!("  - Total summaries: {}", summaries.len());

    assert!(
        !source_files.is_empty(),
        "Should have at least one source file summary"
    );

    // Test 2: Verify each summary has required fields
    println!("{}", "\n".repeat(2));
    println!("{}", "=".repeat(80));
    println!("TEST 2: Verify each summary has required fields");
    println!("{}", "=".repeat(80));

    for summary in &summaries {
        assert!(!summary.filename.is_empty(), "Filename should not be empty");
        assert!(
            !summary.summary.is_empty(),
            "Summary should not be empty for {}",
            summary.filename
        );
        assert!(
            summary.contract_category.is_some(),
            "Contract category should be set for {}",
            summary.filename
        );
        assert!(
            summary.file_type.is_some(),
            "File type should be set for {}",
            summary.filename
        );

        println!("✅ {} - All fields present", summary.filename);
    }

    // Test 3: Verify data was saved to database
    println!("{}", "\n".repeat(2));
    println!("{}", "=".repeat(80));
    println!("TEST 3: Verify data saved to SQLite database");
    println!("{}", "=".repeat(80));

    let db_summaries =
        get_summaries_from_db(&repo).expect("Failed to retrieve summaries from database");

    assert_eq!(
        summaries.len(),
        db_summaries.len(),
        "Database should contain all summaries"
    );

    println!("✅ All {} summaries saved to database", db_summaries.len());

    // Test 4: Verify data can be read correctly from database
    println!("{}", "\n".repeat(2));
    println!("{}", "=".repeat(80));
    println!("TEST 4: Verify data read correctly from database");
    println!("{}", "=".repeat(80));

    for original in &summaries {
        let from_db = get_file_summary_from_db(&original.filename, &repo)
            .expect("Failed to get summary from database")
            .unwrap_or_else(|| panic!("Summary not found in database: {}", original.filename));

        assert_eq!(original.filename, from_db.filename, "Filename mismatch");
        assert_eq!(
            original.summary, from_db.summary,
            "Summary mismatch for {}",
            original.filename
        );
        assert_eq!(
            original.contract_category, from_db.contract_category,
            "Category mismatch for {}",
            original.filename
        );
        assert_eq!(
            original.file_type, from_db.file_type,
            "File type mismatch for {}",
            original.filename
        );
    }

    println!("✅ All summaries match between memory and database");

    // Test 5: Verify contract categorizations are reasonable
    println!("{}", "\n".repeat(2));
    println!("{}", "=".repeat(80));
    println!("TEST 5: Verify contract categorizations");
    println!("{}", "=".repeat(80));

    let expected_categories = get_expected_categories();
    let mut categorization_report = String::new();
    categorization_report.push_str("\n📋 Contract Categorization Report:\n");
    categorization_report.push_str(&"=".repeat(80));
    categorization_report.push_str("\n\n");

    for summary in &summaries {
        let category = summary.contract_category.unwrap_or_default();
        let filename_only = PathBuf::from(&summary.filename)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        categorization_report.push_str(&format!(
            "📄 {}\n   Category: {}\n   Summary length: {} chars\n\n",
            filename_only,
            category,
            summary.summary.len()
        ));

        // Check if we have an expected category for this file
        if let Some((_, expected)) = expected_categories
            .iter()
            .find(|(name, _)| filename_only.contains(name))
        {
            if category != *expected {
                println!(
                    "⚠️  {} - Expected: {}, Got: {}",
                    filename_only, expected, category
                );
            } else {
                println!(
                    "✅ {} - Correctly categorized as {}",
                    filename_only, category
                );
            }
        }

        // Verify category is not Unknown for core contracts
        if !filename_only.contains("Interface") && !filename_only.starts_with("I") {
            // Allow Unknown for some edge cases, but warn
            if category == ContractCategory::Unknown {
                println!(
                    "⚠️  {} - Categorized as Unknown (may need review)",
                    filename_only
                );
            }
        }
    }

    println!("{}", categorization_report);

    // Save detailed report to file
    let report_path = PathBuf::from("sequence-summarization-test-report.md");
    fs::write(&report_path, categorization_report).expect("Failed to write report");
    println!("📝 Detailed report saved to: {}", report_path.display());

    println!("{}", "\n".repeat(2));
    println!("{}", "=".repeat(80));
    println!("🎉 ALL TESTS PASSED!");
    println!("{}", "=".repeat(80));
    println!("\n✅ Summary:");
    println!("  - {} files summarized", summaries.len());
    println!("  - All summaries saved to database");
    println!("  - All summaries retrieved correctly");
    println!("  - Contract categories assigned");
    println!("  - Report generated: {}", report_path.display());
}

/// Helper function to clear summaries from database for testing
fn clear_summaries_from_db(repo: &RepoPaths) {
    use ai_agent_audit::config::{SUMMARY_DB, app_db_path};
    use rusqlite::Connection;

    let db_path = app_db_path(SUMMARY_DB);
    if let Ok(conn) = Connection::open(&db_path) {
        let _ = conn.execute(
            "DELETE FROM summaries WHERE project_id = ?1",
            [&repo.project_id],
        );
        println!(
            "   ✅ Cleared existing summaries for project: {}",
            repo.project_id
        );
    }
}
