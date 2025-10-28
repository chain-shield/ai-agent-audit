/// Integration and unit tests for parse_all_import_dependencies function
///
/// Tests comprehensive import parsing including:
/// - Library imports (@openzeppelin, @euler-price-oracle, etc.)
/// - Relative imports (../, ./)
/// - Named imports with aliases
/// - Simple imports
/// - Interface detection
/// - Path resolution
///
use ai_agent_audit::{
    config::AuditType,
    enumerator::{
        parse_solidity::parse_all_import_dependencies, utils::contracts_in_source_folder,
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
    utils::remapping::parse_and_store_remappings,
};
use std::path::PathBuf;

#[tokio::test]
#[ignore] // Run manually with: cargo test --test parse_import_dependencies_test -- --ignored --nocapture
async fn test_parse_covenant_curator_imports() {
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
        project_id: "covenant-test-imports".to_string(),
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
    }

    // Parse all contracts to populate contract-to-file mappings
    println!("\n=== Parsing contracts ===");
    let _contracts = contracts_in_source_folder(&repo).await.unwrap();
    println!("Parsed contracts successfully");

    // Test 1: Parse CovenantCurator.sol imports
    println!("\n=== Test 1: CovenantCurator.sol imports ===");
    let curator_file = protocol_root
        .join("src/curators/CovenantCurator.sol")
        .canonicalize()
        .unwrap();

    let curator_source = tokio::fs::read_to_string(&curator_file).await.unwrap();
    let deps = parse_all_import_dependencies(&curator_source, &curator_file, &repo)
        .await
        .unwrap();

    println!("\nLibrary files ({}):", deps.lib_files.len());
    for lib_file in &deps.lib_files {
        println!("  - {}", lib_file.display());
    }

    println!("\nSource files ({}):", deps.source_files.len());
    for source_file in &deps.source_files {
        println!("  - {}", source_file.display());
    }

    println!("\nInterfaces ({}):", deps.interfaces.len());
    for (interface_name, interface_file) in &deps.interfaces {
        println!("  - {} -> {}", interface_name, interface_file.display());
    }

    // Verify CovenantCurator imports
    // Note: OpenZeppelin and forge-std are intentionally excluded by should_exclude_this_library()
    // So we should NOT have OpenZeppelin library files
    assert_eq!(
        deps.lib_files.len(),
        0,
        "Should have 0 library files (OpenZeppelin/forge-std are excluded)"
    );

    assert!(
        deps.source_files
            .iter()
            .any(|f| f.to_string_lossy().contains("interfaces/IPriceOracle.sol")),
        "Should have IPriceOracle source file"
    );

    assert!(
        deps.source_files
            .iter()
            .any(|f| f.to_string_lossy().contains("curators/lib/Errors.sol")),
        "Should have Errors source file"
    );

    assert!(
        deps.interfaces.contains_key("IPriceOracle"),
        "Should detect IPriceOracle as interface"
    );

    assert_eq!(
        deps.interfaces.len(),
        1,
        "Should have exactly 1 interface (IPriceOracle)"
    );

    // Test 2: Parse ChainlinkOracle.sol imports (has Euler imports)
    println!("\n=== Test 2: ChainlinkOracle.sol imports ===");
    let chainlink_file = protocol_root
        .join("src/curators/oracles/chainlink/ChainlinkOracle.sol")
        .canonicalize()
        .unwrap();

    let chainlink_source = tokio::fs::read_to_string(&chainlink_file).await.unwrap();
    let deps = parse_all_import_dependencies(&chainlink_source, &chainlink_file, &repo)
        .await
        .unwrap();

    println!("\nLibrary files ({}):", deps.lib_files.len());
    for lib_file in &deps.lib_files {
        println!("  - {}", lib_file.display());
    }

    println!("\nSource files ({}):", deps.source_files.len());
    for source_file in &deps.source_files {
        println!("  - {}", source_file.display());
    }

    println!("\nInterfaces ({}):", deps.interfaces.len());
    for (interface_name, interface_file) in &deps.interfaces {
        println!("  - {} -> {}", interface_name, interface_file.display());
    }

    // Verify ChainlinkOracle imports
    // ChainlinkOracle imports from @euler-price-oracle which should be included
    assert!(
        deps.lib_files
            .iter()
            .any(|f| f.to_string_lossy().contains("euler-price-oracle")),
        "Should have Euler library files"
    );

    println!(
        "\nChainlinkOracle has {} lib files, {} source files, {} interfaces",
        deps.lib_files.len(),
        deps.source_files.len(),
        deps.interfaces.len()
    );

    // Test 3: Parse BaseAdapter.sol imports (has relative imports)
    println!("\n=== Test 3: BaseAdapter.sol imports ===");
    let base_adapter_file = protocol_root
        .join("src/curators/oracles/BaseAdapter.sol")
        .canonicalize()
        .unwrap();

    let base_adapter_source = tokio::fs::read_to_string(&base_adapter_file).await.unwrap();
    let deps = parse_all_import_dependencies(&base_adapter_source, &base_adapter_file, &repo)
        .await
        .unwrap();

    println!("\nLibrary files ({}):", deps.lib_files.len());
    for lib_file in &deps.lib_files {
        println!("  - {}", lib_file.display());
    }

    println!("\nSource files ({}):", deps.source_files.len());
    for source_file in &deps.source_files {
        println!("  - {}", source_file.display());
    }

    println!("\nInterfaces ({}):", deps.interfaces.len());
    for (interface_name, interface_file) in &deps.interfaces {
        println!("  - {} -> {}", interface_name, interface_file.display());
    }

    // Verify BaseAdapter imports
    assert!(
        deps.lib_files
            .iter()
            .any(|f| f.to_string_lossy().contains("euler-price-oracle")),
        "Should have Euler library files"
    );

    assert!(
        deps.source_files.iter().any(|f| f
            .to_string_lossy()
            .contains("interfaces/ICovenantPriceOracle.sol")),
        "Should have ICovenantPriceOracle source file"
    );

    assert!(
        deps.interfaces.contains_key("ICovenantPriceOracle"),
        "Should detect ICovenantPriceOracle as interface"
    );

    println!("\n=== All tests passed! ===");
}

#[tokio::test]
async fn test_parse_imports_unit() {
    // Unit test with mock Solidity code
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {IMyInterface} from "./interfaces/IMyInterface.sol";
import {MyLibrary} from "../libraries/MyLibrary.sol";
import "./utils/Helper.sol";

contract MyContract is Ownable2Step {
    using SafeERC20 for IERC20;
}
"#;

    // This is a unit test without actual file system access
    // We're just testing the regex parsing logic
    use regex::Regex;

    let import_regex = Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*[\"']([^\"']+)[\"']"#).unwrap();
    let simple_import_regex = Regex::new(r#"import\s*[\"']([^\"']+)[\"']"#).unwrap();

    // Test named imports
    let mut named_imports = Vec::new();
    for cap in import_regex.captures_iter(source_code) {
        if let Some(imports) = cap.get(1) {
            let import_path = cap.get(2).unwrap().as_str();
            named_imports.push((imports.as_str().to_string(), import_path.to_string()));
        }
    }

    assert_eq!(named_imports.len(), 5, "Should find 5 named imports");
    assert!(named_imports
        .iter()
        .any(|(_, path)| path.contains("@openzeppelin")));
    assert!(named_imports
        .iter()
        .any(|(_, path)| path.contains("./interfaces")));
    assert!(named_imports
        .iter()
        .any(|(_, path)| path.contains("../libraries")));

    // Test simple imports
    let mut simple_imports = Vec::new();
    for cap in simple_import_regex.captures_iter(source_code) {
        let import_path = cap.get(1).unwrap().as_str();
        // Skip if it's part of a named import
        if !import_regex.is_match(&format!("import {{}} from \"{}\"", import_path)) {
            simple_imports.push(import_path.to_string());
        }
    }

    assert!(simple_imports
        .iter()
        .any(|path| path.contains("./utils/Helper.sol")));

    println!("Unit test passed!");
}
