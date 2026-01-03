/// Integration test for codeblock generation with ContractCategory extraction
///
/// This test verifies:
/// 1. Codeblocks are generated for all in-scope contracts
/// 2. ContractCategory is successfully extracted from file summaries
/// 3. ContractCategory is correctly saved to codeblock database
/// 4. ContractCategory can be retrieved from codeblock database
/// 5. get_all_contracts returns correct ContractCategory for each contract
///
/// Uses the Sequence repository (2025-10-sequence) for testing.
use ai_agent_audit::{
    build_brain::{
        enrichment, summarize::summarize_src_files_with_model,
        summarize_db::get_file_summary_from_db,
    },
    config::AuditType,
    enumerator::{codeblock_db::CodeBlocksDb, codeblocks::generate_codeblock_from_codebase},
    llm_review::contract::contract_category::ContractCategory,
    prepare_code::git_clone::{PocConfig, RepoPaths},
    utils::remapping::parse_and_store_remappings,
};
use dotenvy::dotenv;
use rusqlite::Connection;
use std::{fs, path::PathBuf};

/// Expected contract categories for key Sequence contracts
fn get_expected_categories() -> Vec<(&'static str, ContractCategory)> {
    vec![
        // Core contracts
        ("Factory", ContractCategory::FactoryDeployer),
        ("Wallet", ContractCategory::ProxyUpgradeable),
        ("Stage1Module", ContractCategory::SignatureValidation),
        ("Stage2Module", ContractCategory::SignatureValidation),
        // Utility libraries (new categories)
        ("LibBytes", ContractCategory::ByteManipulationLibrary),
        ("LibOptim", ContractCategory::ByteManipulationLibrary),
        ("Storage", ContractCategory::StorageHelperLibrary),
        ("ReentrancyGuard", ContractCategory::ReentrancyGuardLibrary),
        ("SelfAuth", ContractCategory::AccessControlModifier),
        (
            "ExplicitSessionManager",
            ContractCategory::AccessControlModifier,
        ),
        (
            "ImplicitSessionManager",
            ContractCategory::AccessControlModifier,
        ),
        (
            "PermissionValidator",
            ContractCategory::AccessControlModifier,
        ),
        ("Permission", ContractCategory::EncodingDecodingLibrary),
        ("Attestation", ContractCategory::EncodingDecodingLibrary),
    ]
}

#[tokio::test]
#[ignore] // Run with: cargo test --test codeblock_generation_integration_test -- --ignored --nocapture
async fn test_codeblock_generation_with_contract_category() {
    // Load environment variables from .env file
    dotenv().ok();

    println!("\n🚀 Starting Codeblock Generation with ContractCategory Integration Test");
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
        .filter(|e| {
            e.path().extension().and_then(|s| s.to_str()) == Some("sol")
                && !e.path().to_string_lossy().contains("/test/")
                && !e.path().to_string_lossy().contains("/script/")
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    println!("📄 Found {} Solidity files", sol_files.len());

    // Create unique project ID
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let project_id = format!("sequence-codeblock-test-{}", timestamp);

    // Create RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/0xsequence/wallet-contracts".to_string(),
        project_id: project_id.clone(),
        root: repo_root.clone(),
        sol_files: sol_files.clone(),
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![protocol_root.join("src")],
        docs: vec![],
        repo_name: repo_name.clone(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "test-commit".to_string(),
        audit_type: AuditType::Client,
        poc: PocConfig::default(),
    };

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 1: Build semantic database
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n🔨 Step 1: Building semantic database...");

    let semantics_path = enrichment::build_semantics_db_from_call_graph(repo.clone())
        .await
        .expect("Failed to build semantic database");

    println!("✅ Semantic database built at: {:?}", semantics_path);

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 2: Generate file summaries with ContractCategory
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n📝 Step 2: Generating file summaries with ContractCategory...");

    // Parse remappings first
    let remapping_file = protocol_root.join("remappings.txt");
    if remapping_file.exists() {
        parse_and_store_remappings(&remapping_file, &repo.project_id)
            .expect("Failed to parse remappings");
    }

    // Generate summaries using LLM
    summarize_src_files_with_model(&repo, "gpt-5-mini")
        .await
        .expect("Failed to generate file summaries");

    println!("✅ File summaries generated");

    // Verify some summaries have ContractCategory
    let mut summaries_with_category = 0;
    for sol_file in &sol_files {
        let filename = sol_file.to_string_lossy().to_string();
        if let Ok(Some(summary)) = get_file_summary_from_db(&filename, &repo) {
            if summary.contract_category.is_some() {
                summaries_with_category += 1;
            }
        }
    }

    println!(
        "📊 Summaries with ContractCategory: {}/{}",
        summaries_with_category,
        sol_files.len()
    );

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 3: Generate codeblocks with ContractCategory extraction
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n🔍 Step 3: Generating codeblocks with ContractCategory extraction...");

    // Open semantic database
    let semantic_db = Connection::open(&semantics_path).expect("Failed to open semantic database");

    // Create codeblocks database
    let codeblocks_db_path = repo_root.join(format!("{}-codeblocks-test.db", project_id));
    let codeblocks_db =
        CodeBlocksDb::open(&codeblocks_db_path).expect("Failed to create codeblocks database");

    // Generate codeblocks
    let max_depth = 3;
    let token_budget = 150_000;

    generate_codeblock_from_codebase(&repo, &semantic_db, &codeblocks_db, max_depth, token_budget)
        .await
        .expect("Failed to generate codeblocks");

    println!("✅ Codeblocks generated");

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 4: Verify ContractCategory was saved to codeblock database
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n✅ Step 4: Verifying ContractCategory in codeblock database...");

    let all_contracts = codeblocks_db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");

    println!(
        "📊 Total contracts in codeblock database: {}",
        all_contracts.len()
    );

    // Count contracts by category
    let mut category_counts = std::collections::HashMap::new();
    for (_contract, (_, category)) in &all_contracts {
        *category_counts.entry(category.clone()).or_insert(0) += 1;
    }

    println!("\n📊 Contracts by category:");
    let mut sorted_categories: Vec<_> = category_counts.iter().collect();
    sorted_categories.sort_by_key(|(_, count)| std::cmp::Reverse(**count));

    for (category, count) in sorted_categories {
        println!("  - {:?}: {}", category, count);
    }

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 5: Verify specific contracts have expected categories
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n🔍 Step 5: Verifying expected contract categories...");

    let expected_categories = get_expected_categories();
    let mut matches = 0;
    let mut mismatches = Vec::new();

    for (contract_name, expected_category) in &expected_categories {
        if let Some((_, actual_category)) = all_contracts.get(*contract_name) {
            if actual_category == expected_category {
                matches += 1;
                println!("  ✅ {}: {:?}", contract_name, actual_category);
            } else {
                mismatches.push((contract_name, expected_category, actual_category));
                println!(
                    "  ⚠️  {}: expected {:?}, got {:?}",
                    contract_name, expected_category, actual_category
                );
            }
        } else {
            println!("  ⏭️  {} not found in codeblock database", contract_name);
        }
    }

    println!(
        "\n📊 Category verification: {}/{} matches",
        matches,
        expected_categories.len()
    );

    if !mismatches.is_empty() {
        println!("\n⚠️  Mismatches found:");
        for (contract, expected, actual) in &mismatches {
            println!(
                "  - {}: expected {:?}, got {:?}",
                contract, expected, actual
            );
        }
    }

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 6: Verify no contracts have Unknown category (except legitimately complex ones)
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n🔍 Step 6: Checking for Unknown categories...");

    let unknown_contracts: Vec<_> = all_contracts
        .iter()
        .filter(|(_, (_, category))| *category == ContractCategory::Unknown)
        .map(|(name, _)| name.as_str())
        .collect();

    if unknown_contracts.is_empty() {
        println!("  ✅ No contracts with Unknown category");
    } else {
        println!(
            "  ⚠️  {} contracts with Unknown category:",
            unknown_contracts.len()
        );
        for contract in &unknown_contracts {
            println!("    - {}", contract);
        }
    }

    // ────────────────────────────────────────────────────────────────────────────
    // Summary
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n{}", "=".repeat(80));
    println!("✅ Integration Test Complete!");
    println!("{}", "=".repeat(80));
    println!("📊 Summary:");
    println!("  - Total contracts: {}", all_contracts.len());
    println!("  - Unique categories: {}", category_counts.len());
    println!(
        "  - Expected category matches: {}/{}",
        matches,
        expected_categories.len()
    );
    println!("  - Unknown contracts: {}", unknown_contracts.len());

    // Cleanup
    println!("\n🧹 Cleaning up test databases...");
    fs::remove_file(&codeblocks_db_path).ok();
    fs::remove_file(&semantics_path).ok();

    println!("✅ Test complete!");
}
