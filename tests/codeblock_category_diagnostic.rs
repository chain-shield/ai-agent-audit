/// Diagnostic test to check ContractCategory saving and retrieval in codeblocks database
///
/// This test verifies:
/// 1. File summaries have ContractCategory populated
/// 2. extract_contract_category_from_contract() retrieves correct category
/// 3. Codeblocks are saved with correct ContractCategory
/// 4. get_all_contracts() retrieves correct ContractCategory
use ai_agent_audit::{
    build_brain::{inheritance_map::resolve_contract_file, summarize_db::get_file_summary_from_db},
    config::AuditType,
    enumerator::{
        codeblock_db::CodeBlocksDb, codeblocks::extract_contract_category_from_contract,
        utils::SolFileType,
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use std::path::PathBuf;

#[tokio::test]
#[ignore] // Run with: cargo test --test codeblock_category_diagnostic -- --ignored --nocapture
async fn diagnose_contract_category_issue() {
    dotenvy::dotenv().ok();

    println!("\n🔍 Diagnostic Test: ContractCategory Saving and Retrieval");
    println!("{}", "=".repeat(80));

    // Setup repository path
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let protocol_root = PathBuf::from(format!("{}/Desktop/Audit/2025-11-sequence", home_dir));
    let repo_name = "2025-11-sequence".to_string();

    if !protocol_root.exists() {
        panic!(
            "❌ Repository not found at: {}\nPlease ensure the Sequence repository is cloned to this location.",
            protocol_root.display()
        );
    }

    println!("\n📂 Repository: {}", protocol_root.display());

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

    // Create RepoPaths (use existing project_id to read existing database)
    let project_id = "sequence-audit".to_string(); // Use the actual project_id from your run

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
    // STEP 1: Check file summaries database
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n📝 Step 1: Checking file summaries database...");

    let test_contracts = vec![
        "Factory",
        "Wallet",
        "LibBytes",
        "Storage",
        "ReentrancyGuard",
        "SelfAuth",
    ];

    for contract_name in &test_contracts {
        // Resolve contract file
        let file = resolve_contract_file(contract_name, SolFileType::Standard, &repo)
            .await
            .ok()
            .flatten();

        if let Some(file_path) = file {
            let filename = file_path.to_string_lossy().to_string();
            println!("\n  🔍 Contract: {}", contract_name);
            println!("     File: {}", filename);

            // Get file summary
            match get_file_summary_from_db(&filename, &repo) {
                Ok(Some(summary)) => {
                    println!("     Summary exists: ✅");
                    println!("     ContractCategory: {:?}", summary.contract_category);
                }
                Ok(None) => {
                    println!("     Summary exists: ❌ (not found)");
                }
                Err(e) => {
                    println!("     Error reading summary: {}", e);
                }
            }

            // Test extract_contract_category_from_contract
            match extract_contract_category_from_contract(contract_name, &repo).await {
                Ok(Some(category)) => {
                    println!("     extract_contract_category: {:?}", category);
                }
                Ok(None) => {
                    println!("     extract_contract_category: None");
                }
                Err(e) => {
                    println!("     extract_contract_category error: {}", e);
                }
            }
        } else {
            println!("\n  ⚠️  Contract: {} - file not found", contract_name);
        }
    }

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 2: Check codeblocks database
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n\n📦 Step 2: Checking codeblocks database...");

    let codeblocks_db_path = repo_root.join("codeblocks.db");

    if !codeblocks_db_path.exists() {
        println!(
            "❌ Codeblocks database not found at: {:?}",
            codeblocks_db_path
        );
        println!("   Please run the codeblock generation first.");
        return;
    }

    let codeblocks_db =
        CodeBlocksDb::open(&codeblocks_db_path).expect("Failed to open codeblocks database");

    // Get all contracts
    let all_contracts = codeblocks_db
        .get_all_contracts(&repo)
        .expect("Failed to get all contracts");

    println!(
        "📊 Total contracts in codeblocks database: {}",
        all_contracts.len()
    );

    // Count by category
    let mut category_counts = std::collections::HashMap::new();
    for (_, (_, category)) in &all_contracts {
        *category_counts.entry(category.clone()).or_insert(0) += 1;
    }

    println!("\n📊 Contracts by category:");
    let mut sorted_categories: Vec<_> = category_counts.iter().collect();
    sorted_categories.sort_by_key(|(_, count)| std::cmp::Reverse(**count));

    for (category, count) in sorted_categories {
        println!("  - {:?}: {}", category, count);
    }

    // Check specific contracts
    println!("\n🔍 Checking specific contracts in codeblocks database:");
    for contract_name in &test_contracts {
        if let Some((_, category)) = all_contracts.get(*contract_name) {
            println!("  - {}: {:?}", contract_name, category);
        } else {
            println!("  - {}: ❌ Not found in database", contract_name);
        }
    }

    // ────────────────────────────────────────────────────────────────────────────
    // STEP 3: Direct SQL query to check raw data
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n\n🔍 Step 3: Direct SQL query to check raw database values...");

    use rusqlite::Connection;
    let conn = Connection::open(&codeblocks_db_path).expect("Failed to open database");

    let mut stmt = conn
        .prepare(
            "SELECT contract, contract_category FROM codeblocks WHERE project_id = ?1 LIMIT 10",
        )
        .expect("Failed to prepare statement");

    let rows = stmt
        .query_map([&repo.project_id], |row| {
            Ok((
                row.get::<_, String>(0)?, // contract
                row.get::<_, String>(1)?, // contract_category (raw string)
            ))
        })
        .expect("Failed to query");

    println!("📊 First 10 contracts (raw SQL):");
    for (i, row) in rows.enumerate() {
        if let Ok((contract, category_str)) = row {
            println!("  {}. {}: \"{}\"", i + 1, contract, category_str);
        }
    }

    // ────────────────────────────────────────────────────────────────────────────
    // Summary
    // ────────────────────────────────────────────────────────────────────────────
    println!("\n{}", "=".repeat(80));
    println!("✅ Diagnostic Test Complete!");
    println!("{}", "=".repeat(80));
}
