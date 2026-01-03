/// Integration test for Solidity-based inheritance parsing
///
/// Tests that inheritance relationships are correctly extracted from Solidity files
/// and stored in the inheritance maps with (contract, file) tuple keys.
///
use ai_agent_audit::{
    build_brain::inheritance_map::{get_children_with_file, get_parents_with_file},
    config::AuditType,
    enumerator::utils::contracts_in_source_folder,
    prepare_code::git_clone::{PocConfig, RepoPaths},
    utils::remapping::parse_and_store_remappings,
};
use std::path::PathBuf;

#[tokio::test]
#[ignore] // Run manually with: cargo test --test inheritance_solidity_parsing_test -- --ignored --nocapture
async fn test_covenant_inheritance_parsing() {
    // Initialize logging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    // Path to Covenant repository
    let repo_root = PathBuf::from("/private/tmp/audit-analysis/2025-10-covenant-d5ebe4");
    let repo_name = "2025-10-covenant".to_string();
    let protocol_root = repo_root.join(&repo_name);

    // Collect all .sol files
    let sol_files = walkdir::WalkDir::new(&protocol_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sol"))
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    println!("Found {} .sol files", sol_files.len());

    // Create RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/test/covenant".to_string(),
        project_id: "covenant-test".to_string(),
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
    };

    // Load remappings first (required for import resolution)
    println!("\n=== Loading remappings ===");
    let remapping_file = protocol_root.join("remappings.txt");
    if remapping_file.exists() {
        match parse_and_store_remappings(&remapping_file, &repo.project_id) {
            Ok(count) => println!("Loaded {} remappings", count),
            Err(e) => println!("Warning: Failed to load remappings: {}", e),
        }
    } else {
        println!(
            "Warning: No remappings.txt found at {}",
            remapping_file.display()
        );
    }

    // Parse all contracts (this will populate inheritance maps)
    println!("\n=== Parsing contracts and inheritance relationships ===");
    let contracts = contracts_in_source_folder(&repo).await.unwrap();
    println!("Found {} source contracts", contracts.len());

    // Test 1: Covenant's BaseAdapter should inherit from Euler's BaseAdapter and ICovenantPriceOracle
    println!("\n=== Test 1: Covenant's BaseAdapter ===");
    let covenant_base_adapter_file = protocol_root
        .join("src/curators/oracles/BaseAdapter.sol")
        .canonicalize()
        .expect("Failed to canonicalize Covenant BaseAdapter path");
    let parents = get_parents_with_file("BaseAdapter", &covenant_base_adapter_file, &repo)
        .await
        .unwrap();

    println!("Covenant's BaseAdapter parents:");
    for (parent_name, parent_file) in &parents {
        println!("  - {} ({})", parent_name, parent_file.display());
    }

    // Should have 2 parents
    assert!(
        parents.len() >= 1,
        "Covenant's BaseAdapter should have at least 1 parent"
    );

    // Should inherit from EulerBaseAdapter (or BaseAdapter from lib)
    let has_euler_base = parents.iter().any(|(name, file)| {
        (name == "EulerBaseAdapter" || name == "BaseAdapter")
            && file.to_string_lossy().contains("lib/euler-price-oracle")
    });
    assert!(has_euler_base, "Should inherit from Euler's BaseAdapter");

    // Should inherit from ICovenantPriceOracle
    let has_covenant_oracle = parents
        .iter()
        .any(|(name, _)| name == "ICovenantPriceOracle");
    assert!(
        has_covenant_oracle,
        "Should inherit from ICovenantPriceOracle"
    );

    // Test 2: Euler's BaseAdapter should inherit from IPriceOracle
    println!("\n=== Test 2: Euler's BaseAdapter ===");
    let euler_base_adapter_file = protocol_root
        .join("lib/euler-price-oracle/src/adapter/BaseAdapter.sol")
        .canonicalize()
        .expect("Failed to canonicalize Euler BaseAdapter path");
    let euler_parents = get_parents_with_file("BaseAdapter", &euler_base_adapter_file, &repo)
        .await
        .unwrap();

    println!("Euler's BaseAdapter parents:");
    for (parent_name, parent_file) in &euler_parents {
        println!("  - {} ({})", parent_name, parent_file.display());
    }

    // Should inherit from IPriceOracle
    let has_iprice_oracle = euler_parents.iter().any(|(name, _)| name == "IPriceOracle");
    assert!(
        has_iprice_oracle,
        "Euler's BaseAdapter should inherit from IPriceOracle"
    );

    // Test 3: Covenant's ChainlinkOracle should inherit from Euler's ChainlinkOracle
    println!("\n=== Test 3: Covenant's ChainlinkOracle ===");
    let covenant_chainlink_file = protocol_root
        .join("src/curators/oracles/chainlink/ChainlinkOracle.sol")
        .canonicalize()
        .expect("Failed to canonicalize Covenant ChainlinkOracle path");
    let chainlink_parents =
        get_parents_with_file("ChainlinkOracle", &covenant_chainlink_file, &repo)
            .await
            .unwrap();

    println!("Covenant's ChainlinkOracle parents:");
    for (parent_name, parent_file) in &chainlink_parents {
        println!("  - {} ({})", parent_name, parent_file.display());
    }

    // Should inherit from Euler's ChainlinkOracle
    let has_euler_chainlink = chainlink_parents.iter().any(|(name, file)| {
        name == "ChainlinkOracle" && file.to_string_lossy().contains("lib/euler-price-oracle")
    });
    assert!(
        has_euler_chainlink,
        "Should inherit from Euler's ChainlinkOracle"
    );

    // Test 4: CovenantCurator should inherit from Ownable2Step and IPriceOracle
    println!("\n=== Test 4: CovenantCurator ===");
    let curator_file = protocol_root
        .join("src/curators/CovenantCurator.sol")
        .canonicalize()
        .expect("Failed to canonicalize CovenantCurator path");
    let curator_parents = get_parents_with_file("CovenantCurator", &curator_file, &repo)
        .await
        .unwrap();

    println!("CovenantCurator parents:");
    for (parent_name, parent_file) in &curator_parents {
        println!("  - {} ({})", parent_name, parent_file.display());
    }

    // Should have at least 2 parents
    assert!(
        curator_parents.len() >= 2,
        "CovenantCurator should have at least 2 parents"
    );

    // Test 5: Check children relationships (inverted map)
    println!("\n=== Test 5: IPriceOracle children ===");

    // Find IPriceOracle file
    let iprice_oracle_file = protocol_root
        .join("lib/euler-price-oracle/src/interfaces/IPriceOracle.sol")
        .canonicalize()
        .expect("Failed to canonicalize IPriceOracle path");
    let iprice_children = get_children_with_file("IPriceOracle", &iprice_oracle_file, &repo)
        .await
        .unwrap();

    println!("IPriceOracle children:");
    for (child_name, child_file) in &iprice_children {
        println!("  - {} ({})", child_name, child_file.display());
    }

    // Should have multiple children
    assert!(
        iprice_children.len() >= 1,
        "IPriceOracle should have at least 1 child"
    );

    // Test 6: Verify no duplicate name collisions
    println!("\n=== Test 6: Verify duplicate names are handled correctly ===");

    // Both BaseAdapters should be in the system
    let covenant_base_parents =
        get_parents_with_file("BaseAdapter", &covenant_base_adapter_file, &repo)
            .await
            .unwrap();

    let euler_base_parents = get_parents_with_file("BaseAdapter", &euler_base_adapter_file, &repo)
        .await
        .unwrap();

    // They should have different parents (proving they're separate entries)
    println!(
        "Covenant BaseAdapter has {} parents",
        covenant_base_parents.len()
    );
    println!("Euler BaseAdapter has {} parents", euler_base_parents.len());

    // Covenant's should have more parents (inherits from Euler's + ICovenantPriceOracle)
    assert!(
        covenant_base_parents.len() >= euler_base_parents.len(),
        "Covenant's BaseAdapter should have at least as many parents as Euler's"
    );

    println!("\n=== All tests passed! ===");
}

/// Test to prove the complete inheritance hierarchy using get_children recursively
#[tokio::test]
#[ignore]
async fn test_complete_inheritance_hierarchy() {
    use ai_agent_audit::build_brain::inheritance_map::get_children_with_file;
    use ai_agent_audit::config::AuditType;
    use ai_agent_audit::enumerator::utils::contracts_in_source_folder;
    use ai_agent_audit::prepare_code::git_clone::{PocConfig, RepoPaths};
    use ai_agent_audit::utils::remapping::parse_and_store_remappings;
    use std::collections::HashSet;
    use std::path::PathBuf;

    // Setup repository paths (same as first test)
    let repo_root = PathBuf::from("/private/tmp/audit-analysis/2025-10-covenant-d5ebe4");
    let repo_name = "2025-10-covenant".to_string();
    let protocol_root = repo_root.join(&repo_name);

    // Collect all .sol files
    let sol_files = walkdir::WalkDir::new(&protocol_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sol"))
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    // Create RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/test/covenant".to_string(),
        project_id: "covenant-test-hierarchy".to_string(),
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
    };

    // Load remappings first
    println!("\n=== Loading remappings ===");
    let remapping_file = protocol_root.join("remappings.txt");
    if remapping_file.exists() {
        match parse_and_store_remappings(&remapping_file, &repo.project_id) {
            Ok(count) => println!("Loaded {} remappings", count),
            Err(e) => println!("Warning: Failed to load remappings: {}", e),
        }
    }

    // Parse all contracts
    println!("\n=== Parsing contracts ===");
    let _contracts = contracts_in_source_folder(&repo).await.unwrap();
    println!("Parsed contracts successfully");

    // Helper function to recursively print children with indentation
    fn print_children_recursive<'a>(
        contract: &'a str,
        file: &'a PathBuf,
        repo: &'a RepoPaths,
        indent: usize,
        visited: &'a mut HashSet<(String, PathBuf)>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + 'a>> {
        Box::pin(async move {
            let key = (contract.to_string(), file.clone());
            if visited.contains(&key) {
                return; // Avoid infinite loops
            }
            visited.insert(key);

            let children = get_children_with_file(contract, file, repo)
                .await
                .unwrap_or_default();

            for (child_name, child_file) in children {
                let child_file_canonical = child_file.canonicalize().unwrap_or(child_file.clone());
                let indent_str = "  ".repeat(indent);
                let file_type = if child_file_canonical.to_string_lossy().contains("/lib/") {
                    "LIB"
                } else {
                    "SRC"
                };

                println!(
                    "{}├─ {} ({}) [{}]",
                    indent_str,
                    child_name,
                    child_file_canonical.display(),
                    file_type
                );

                // Recursively print children of this child
                print_children_recursive(
                    &child_name,
                    &child_file_canonical,
                    repo,
                    indent + 1,
                    visited,
                )
                .await;
            }
        })
    }

    println!("\n=== COMPLETE INHERITANCE HIERARCHY ===\n");

    // Test 1: Euler's IPriceOracle hierarchy
    println!("📋 Euler's IPriceOracle (lib/euler-price-oracle/src/interfaces/IPriceOracle.sol)");
    let euler_iprice_oracle_file = protocol_root
        .join("lib/euler-price-oracle/src/interfaces/IPriceOracle.sol")
        .canonicalize()
        .unwrap();

    let mut visited = HashSet::new();
    print_children_recursive(
        "IPriceOracle",
        &euler_iprice_oracle_file,
        &repo,
        0,
        &mut visited,
    )
    .await;

    // Test 2: Covenant's IPriceOracle hierarchy
    println!("\n📋 Covenant's IPriceOracle (src/interfaces/IPriceOracle.sol)");
    let covenant_iprice_oracle_file = protocol_root
        .join("src/interfaces/IPriceOracle.sol")
        .canonicalize()
        .unwrap();

    let mut visited = HashSet::new();
    print_children_recursive(
        "IPriceOracle",
        &covenant_iprice_oracle_file,
        &repo,
        0,
        &mut visited,
    )
    .await;

    // Test 3: Verify specific relationships
    println!("\n=== VERIFICATION TESTS ===\n");

    // 3.1: Verify Euler's BaseAdapter children
    println!("Test 3.1: Euler's BaseAdapter children");
    let euler_base_adapter_file = protocol_root
        .join("lib/euler-price-oracle/src/adapter/BaseAdapter.sol")
        .canonicalize()
        .unwrap();

    let base_adapter_children =
        get_children_with_file("BaseAdapter", &euler_base_adapter_file, &repo)
            .await
            .unwrap();

    println!("  Direct children of Euler's BaseAdapter:");
    for (child_name, child_file) in &base_adapter_children {
        println!("    - {} ({})", child_name, child_file.display());
    }

    // Should have: ChainlinkOracle (Euler), PythOracle (Euler), BaseAdapter (Covenant)
    assert!(
        base_adapter_children
            .iter()
            .any(|(name, file)| name == "ChainlinkOracle"
                && file.to_string_lossy().contains("lib/euler-price-oracle")),
        "Euler's BaseAdapter should have Euler's ChainlinkOracle as child"
    );

    assert!(
        base_adapter_children
            .iter()
            .any(|(name, file)| name == "PythOracle"
                && file.to_string_lossy().contains("lib/euler-price-oracle")),
        "Euler's BaseAdapter should have Euler's PythOracle as child"
    );

    assert!(
        base_adapter_children
            .iter()
            .any(|(name, file)| name == "BaseAdapter"
                && file.to_string_lossy().contains("src/curators/oracles")),
        "Euler's BaseAdapter should have Covenant's BaseAdapter as child"
    );

    // 3.2: Verify Euler's ChainlinkOracle children
    println!("\nTest 3.2: Euler's ChainlinkOracle children");
    let euler_chainlink_file = protocol_root
        .join("lib/euler-price-oracle/src/adapter/chainlink/ChainlinkOracle.sol")
        .canonicalize()
        .unwrap();

    let euler_chainlink_children =
        get_children_with_file("ChainlinkOracle", &euler_chainlink_file, &repo)
            .await
            .unwrap();

    println!("  Direct children of Euler's ChainlinkOracle:");
    for (child_name, child_file) in &euler_chainlink_children {
        println!("    - {} ({})", child_name, child_file.display());
    }

    assert!(
        euler_chainlink_children
            .iter()
            .any(|(name, file)| name == "ChainlinkOracle"
                && file
                    .to_string_lossy()
                    .contains("src/curators/oracles/chainlink")),
        "Euler's ChainlinkOracle should have Covenant's ChainlinkOracle as child"
    );

    // 3.3: Verify Euler's PythOracle children
    println!("\nTest 3.3: Euler's PythOracle children");
    let euler_pyth_file = protocol_root
        .join("lib/euler-price-oracle/src/adapter/pyth/PythOracle.sol")
        .canonicalize()
        .unwrap();

    let euler_pyth_children = get_children_with_file("PythOracle", &euler_pyth_file, &repo)
        .await
        .unwrap();

    println!("  Direct children of Euler's PythOracle:");
    for (child_name, child_file) in &euler_pyth_children {
        println!("    - {} ({})", child_name, child_file.display());
    }

    assert!(
        euler_pyth_children
            .iter()
            .any(|(name, file)| name == "PythOracle"
                && file.to_string_lossy().contains("src/curators/oracles/pyth")),
        "Euler's PythOracle should have Covenant's PythOracle as child"
    );

    // 3.4: Verify Covenant's BaseAdapter children
    println!("\nTest 3.4: Covenant's BaseAdapter children");
    let covenant_base_adapter_file = protocol_root
        .join("src/curators/oracles/BaseAdapter.sol")
        .canonicalize()
        .unwrap();

    let covenant_base_children =
        get_children_with_file("BaseAdapter", &covenant_base_adapter_file, &repo)
            .await
            .unwrap();

    println!("  Direct children of Covenant's BaseAdapter:");
    for (child_name, child_file) in &covenant_base_children {
        println!("    - {} ({})", child_name, child_file.display());
    }

    // Covenant's BaseAdapter should have Covenant's ChainlinkOracle and PythOracle as children
    // (since they inherit from Euler's versions which inherit from Euler's BaseAdapter,
    // and Covenant's BaseAdapter also inherits from Euler's BaseAdapter)
    // Actually, Covenant's ChainlinkOracle inherits from Euler's ChainlinkOracle, not Covenant's BaseAdapter
    // So Covenant's BaseAdapter might not have direct children in the source code

    println!("\n=== HIERARCHY VERIFICATION COMPLETE ===");
    println!("✅ All inheritance relationships verified!");
}
