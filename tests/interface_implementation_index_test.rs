/// Integration test for interface implementation index generation
///
/// This test builds the complete interface → implementation index for the Covenant repository
/// and saves all results to a markdown file for manual inspection.
///
/// The test verifies:
/// - Interface detection across the codebase
/// - Implementation discovery for each interface
/// - Accurate file path resolution
/// - Proper handling of lib/ vs src/ interfaces
///
use ai_agent_audit::{
    build_brain::enrichment,
    config::AuditType,
    enumerator::{
        interface_implementations::build_and_get_interface_implementation_index,
        utils::contracts_in_source_folder,
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
    utils::remapping::parse_and_store_remappings,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Create RepoPaths for Covenant repository
///
/// NOTE: Update this path to point to your local Covenant repository clone.
/// The repository should be a full clone with source files, not just test files.
fn create_covenant_repo_paths() -> RepoPaths {
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let protocol_root = PathBuf::from(format!("{}/Desktop/Audit/2025-10-covenant", home_dir));
    let repo_name = "2025-10-covenant".to_string();

    // The root should be the parent directory, not the protocol root itself
    // This matches the structure used by clone_and_filter_git_repo()
    let repo_root = protocol_root.parent().unwrap().to_path_buf();

    // Collect all .sol files
    let sol_files = walkdir::WalkDir::new(&protocol_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sol"))
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    RepoPaths {
        github_url: "https://github.com/test/covenant".to_string(),
        project_id: "covenant-interface-index-test".to_string(),
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
        commit_hash: "d5ebe4".to_string(),
        audit_type: AuditType::Client,
        poc: PocConfig::default(),
    }
}

/// Format file path relative to repository root for display
fn display_file(file: &Path, repo: &RepoPaths) -> String {
    file.strip_prefix(&repo.root)
        .unwrap_or(file)
        .display()
        .to_string()
}

/// Generate markdown report of the interface implementation index
fn generate_markdown_report(
    index: &HashMap<(String, PathBuf), Vec<(String, PathBuf)>>,
    repo: &RepoPaths,
) -> String {
    let mut md = String::new();

    md.push_str("# Interface Implementation Index\n\n");
    md.push_str(&format!("**Repository**: {}\n", repo.repo_name));
    md.push_str(&format!("**Commit**: {}\n", repo.commit_hash));
    md.push_str(&format!("**Total Interfaces Found**: {}\n\n", index.len()));

    md.push_str("---\n\n");
    md.push_str("## Table of Contents\n\n");

    // Generate TOC
    let mut interfaces: Vec<_> = index.keys().collect();
    interfaces.sort_by(|a, b| a.0.cmp(&b.0));

    for (interface_name, _) in &interfaces {
        md.push_str(&format!(
            "- [{}](#{})\n",
            interface_name,
            interface_name.to_lowercase()
        ));
    }

    md.push_str("\n---\n\n");
    md.push_str("## Interface Details\n\n");

    // Generate detailed sections for each interface
    for (interface_name, interface_file) in &interfaces {
        let implementations = index
            .get(&(interface_name.to_string(), interface_file.to_path_buf()))
            .unwrap();

        md.push_str(&format!("### {}\n\n", interface_name));
        md.push_str(&format!(
            "**Interface File**: `{}`\n\n",
            display_file(interface_file, repo)
        ));
        md.push_str(&format!(
            "**Implementation Count**: {}\n\n",
            implementations.len()
        ));

        if implementations.is_empty() {
            md.push_str("*No implementations found*\n\n");
        } else {
            md.push_str("**Implementations**:\n\n");

            // Sort implementations by contract name
            let mut sorted_impls = implementations.clone();
            sorted_impls.sort_by(|a, b| a.0.cmp(&b.0));

            for (contract_name, contract_file) in sorted_impls {
                let file_display = display_file(&contract_file, repo);
                md.push_str(&format!("- **{}**\n", contract_name));
                md.push_str(&format!("  - File: `{}`\n", file_display));

                // Categorize by location
                let location = if file_display.contains("/lib/") {
                    "External Library"
                } else if file_display.contains("/src/") {
                    "Source Code"
                } else if file_display.contains("/test/") {
                    "Test"
                } else {
                    "Other"
                };
                md.push_str(&format!("  - Location: {}\n", location));
            }
            md.push('\n');
        }

        md.push_str("---\n\n");
    }

    // Add statistics section
    md.push_str("## Statistics\n\n");

    let total_implementations: usize = index.values().map(|v| v.len()).sum();
    let avg_implementations = if index.is_empty() {
        0.0
    } else {
        total_implementations as f64 / index.len() as f64
    };

    md.push_str(&format!("- **Total Interfaces**: {}\n", index.len()));
    md.push_str(&format!(
        "- **Total Implementations**: {}\n",
        total_implementations
    ));
    md.push_str(&format!(
        "- **Average Implementations per Interface**: {:.2}\n\n",
        avg_implementations
    ));

    // Count by location
    let mut src_count = 0;
    let mut lib_count = 0;
    let mut test_count = 0;
    let mut other_count = 0;

    for implementations in index.values() {
        for (_, file) in implementations {
            let file_str = display_file(file, repo);
            if file_str.contains("/src/") {
                src_count += 1;
            } else if file_str.contains("/lib/") {
                lib_count += 1;
            } else if file_str.contains("/test/") {
                test_count += 1;
            } else {
                other_count += 1;
            }
        }
    }

    md.push_str("**Implementations by Location**:\n\n");
    md.push_str(&format!("- Source Code (`/src/`): {}\n", src_count));
    md.push_str(&format!("- External Libraries (`/lib/`): {}\n", lib_count));
    md.push_str(&format!("- Tests (`/test/`): {}\n", test_count));
    md.push_str(&format!("- Other: {}\n\n", other_count));

    md
}

#[tokio::test]
#[ignore] // Run with: cargo test --test interface_implementation_index_test -- --ignored --nocapture
async fn test_build_interface_implementation_index_covenant() {
    // Initialize logging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    println!("\n🔍 Building Interface Implementation Index for Covenant Repository\n");

    let repo = create_covenant_repo_paths();

    // Verify the repository exists
    let repo_path = repo.root.join(&repo.repo_name);
    if !repo_path.exists() {
        println!(
            "⏭️  Skipping test - repository not found at {:?}",
            repo_path
        );
        println!("   Please clone the Covenant repository to this location first.");
        return;
    }

    println!("✅ Repository found at: {:?}", repo_path);
    println!("📁 Total .sol files: {}", repo.sol_files.len());

    // Check if we have source files (not just test files)
    let source_files: Vec<_> = repo
        .sol_files
        .iter()
        .filter(|f| !f.to_string_lossy().contains("/test/"))
        .collect();

    if source_files.is_empty() {
        println!("\n⚠️  WARNING: No source files found (only test files)!");
        println!("   This repository may not have been fully cloned.");
        println!("   The test will continue but may not find any interfaces.");
    } else {
        println!("   Source files (non-test): {}", source_files.len());
    }

    // Parse remappings (required for import resolution)
    println!("\n📋 Parsing remappings...");
    let remapping_file = repo_path.join("remappings.txt");
    if remapping_file.exists() {
        match parse_and_store_remappings(&remapping_file, &repo.project_id) {
            Ok(count) => println!("   Loaded {} remappings", count),
            Err(e) => println!("   Warning: Failed to load remappings: {}", e),
        }
    } else {
        println!("   No remappings.txt found, skipping");
    }

    // Build contract-to-file mapping (required for get_contract_type)
    println!("🗂️  Building contract-to-file mapping...");
    contracts_in_source_folder(&repo)
        .await
        .expect("Failed to build contract mapping");

    // Build semantic database with Slither (required for inheritance map)
    println!("🔧 Building semantic database with Slither...");
    println!("   (This may take a few minutes on first run)");
    let _semantics_db = enrichment::build_semantics_db_from_call_graph(repo.clone())
        .await
        .expect("Failed to build semantic database");
    println!("   ✅ Semantic database built successfully");

    // Build the interface implementation index
    println!("\n🏗️  Building interface implementation index...");
    let index = build_and_get_interface_implementation_index(&repo)
        .await
        .expect("Failed to build interface implementation index");

    println!("\n✅ Index built successfully!");
    println!("   - Total interfaces found: {}", index.len());
    println!(
        "   - Total implementations: {}",
        index.values().map(|v| v.len()).sum::<usize>()
    );

    // Generate markdown report
    println!("\n📝 Generating markdown report...");
    let markdown = generate_markdown_report(&index, &repo);

    // Save to file
    let output_file = PathBuf::from("covenant-interface-implementation-index.md");
    fs::write(&output_file, markdown).expect("Failed to write markdown file");

    println!("✅ Report saved to: {}", output_file.display());
    println!("\n🎉 Test complete! Review the markdown file for detailed results.");
}
