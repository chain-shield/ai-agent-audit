/// Comprehensive tests for detect_source_code_dependencies and parse_all_import_dependencies
///
/// Tests the refactored import parsing system that uses ImportDependencies struct
/// with lib_files, source_files, and interfaces_and_children fields.
///
use ai_agent_audit::{
    config::AuditType,
    enumerator::{
        parse_solidity::{detect_source_code_dependencies, parse_all_import_dependencies},
        utils::contracts_in_source_folder,
    },
    prepare_code::git_clone::RepoPaths,
    utils::remapping::parse_and_store_remappings,
};
use std::path::PathBuf;

/// Unit test: Test parse_all_import_dependencies with mock Solidity code
#[tokio::test]
async fn test_parse_all_import_dependencies_unit() {
    // Initialize logging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    let mock_solidity = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20, SafeERC20} from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import {IPriceOracle} from "./interfaces/IPriceOracle.sol";
import {BaseAdapter as EulerBaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";
import {Errors} from "./lib/Errors.sol";
import "../utils/Helper.sol";

contract TestContract is IPriceOracle {
    using SafeERC20 for IERC20;
    
    function test() external {}
}
"#;

    // Create minimal RepoPaths for testing
    let repo_root = PathBuf::from("/tmp/test-repo");
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-parse-imports".to_string(),
        root: repo_root.clone(),
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
        commit_hash: "test123".to_string(),
        audit_type: AuditType::Client,
    };

    let current_file = repo_root.join("src/TestContract.sol");

    // Parse imports
    let result = parse_all_import_dependencies(mock_solidity, &current_file, &repo).await;

    // Should succeed even without actual files
    assert!(
        result.is_ok(),
        "parse_all_import_dependencies should succeed"
    );

    let deps = result.unwrap();

    println!("\n=== Parse Results ===");
    println!("Library files: {}", deps.lib_files.len());
    println!("Source files: {}", deps.source_files.len());
    println!("Interfaces and children: {}", deps.interfaces.len());

    // Note: Without actual files, we can't resolve paths, but the function should not crash
    // The actual file resolution is tested in integration tests
}

/// Integration test: Test detect_source_code_dependencies with Covenant repository
#[tokio::test]
#[ignore] // Run with: cargo test --test detect_source_code_dependencies_test -- --ignored --nocapture
async fn test_detect_source_code_dependencies_covenant() {
    // Initialize logging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    // Setup Covenant repository
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
        project_id: "covenant-test-deps".to_string(),
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
    };

    // Load remappings
    println!("\n=== Loading remappings ===");
    let remapping_file = protocol_root.join("remappings.txt");
    if remapping_file.exists() {
        match parse_and_store_remappings(&remapping_file, &repo.project_id) {
            Ok(count) => println!("Loaded {} remappings", count),
            Err(e) => panic!("Failed to load remappings: {}", e),
        }
    }

    // Parse all contracts to populate mappings
    println!("\n=== Parsing contracts ===");
    let contracts = contracts_in_source_folder(&repo).await.unwrap();
    println!("Found {} contracts", contracts.len());

    // Test 1: CovenantCurator dependencies
    println!("\n=== Test 1: CovenantCurator dependencies ===");
    let deps = detect_source_code_dependencies("CovenantCurator", &repo)
        .await
        .unwrap();

    println!("\nCovenantCurator dependencies:");
    println!("  Library files: {}", deps.lib_files.len());
    for lib_file in &deps.lib_files {
        println!("    - {}", lib_file.display());
    }

    println!("  Source files: {}", deps.source_files.len());
    for source_file in &deps.source_files {
        println!("    - {}", source_file.display());
    }

    println!("  Interfaces and children: {}", deps.interfaces.len());
    for (name, file) in &deps.interfaces {
        println!("    - {} ({})", name, file.display());
    }

    // Verify CovenantCurator has expected dependencies
    assert!(
        deps.source_files
            .iter()
            .any(|f| f.to_string_lossy().contains("IPriceOracle.sol")),
        "Should have IPriceOracle source file"
    );

    assert!(
        deps.interfaces
            .iter()
            .any(|(name, _)| name == "IPriceOracle"),
        "Should detect IPriceOracle as interface"
    );

    // Test 2: ChainlinkOracle dependencies
    println!("\n=== Test 2: ChainlinkOracle dependencies ===");
    let deps = detect_source_code_dependencies("ChainlinkOracle", &repo)
        .await
        .unwrap();

    println!("\nChainlinkOracle dependencies:");
    println!("  Library files: {}", deps.lib_files.len());
    println!("  Source files: {}", deps.source_files.len());
    println!("  Interfaces and children: {}", deps.interfaces.len());

    // ChainlinkOracle should have Euler library dependencies
    assert!(
        deps.lib_files
            .iter()
            .any(|f| f.to_string_lossy().contains("euler-price-oracle")),
        "Should have Euler library files"
    );

    // Test 3: BaseAdapter dependencies
    println!("\n=== Test 3: BaseAdapter dependencies ===");
    let deps = detect_source_code_dependencies("BaseAdapter", &repo)
        .await
        .unwrap();

    println!("\nBaseAdapter dependencies:");
    println!("  Library files: {}", deps.lib_files.len());
    println!("  Source files: {}", deps.source_files.len());
    println!("  Interfaces and children: {}", deps.interfaces.len());

    // BaseAdapter should have ICovenantPriceOracle interface
    assert!(
        deps.interfaces
            .iter()
            .any(|(name, _)| name == "ICovenantPriceOracle"),
        "Should detect ICovenantPriceOracle as interface"
    );

    println!("\n=== All tests passed! ===");
}

/// Edge case test: Contract with no imports
#[tokio::test]
async fn test_no_imports() {
    let mock_solidity = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleContract {
    uint256 public value;
    
    function setValue(uint256 _value) external {
        value = _value;
    }
}
"#;

    let repo_root = PathBuf::from("/tmp/test-repo");
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-no-imports".to_string(),
        root: repo_root.clone(),
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
        commit_hash: "test123".to_string(),
        audit_type: AuditType::Client,
    };

    let current_file = repo_root.join("src/SimpleContract.sol");
    let deps = parse_all_import_dependencies(mock_solidity, &current_file, &repo)
        .await
        .unwrap();

    assert_eq!(deps.lib_files.len(), 0, "Should have no library files");
    assert_eq!(deps.source_files.len(), 0, "Should have no source files");
    assert_eq!(deps.interfaces.len(), 0, "Should have no interfaces");
}

/// Edge case test: Multiline imports
#[tokio::test]
async fn test_multiline_imports() {
    let mock_solidity = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {
    IERC20,
    SafeERC20,
    ERC20
} from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import {
    IPriceOracle,
    IOracle
} from "./interfaces/IPriceOracle.sol";

contract MultilineImportContract {
    function test() external {}
}
"#;

    let repo_root = PathBuf::from("/tmp/test-repo");
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-multiline".to_string(),
        root: repo_root.clone(),
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
        commit_hash: "test123".to_string(),
        audit_type: AuditType::Client,
    };

    let current_file = repo_root.join("src/MultilineImportContract.sol");
    let result = parse_all_import_dependencies(mock_solidity, &current_file, &repo).await;

    assert!(
        result.is_ok(),
        "Should handle multiline imports without crashing"
    );
}

/// Edge case test: Imports with aliases
#[tokio::test]
async fn test_imports_with_aliases() {
    let mock_solidity = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20 as Token} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {BaseAdapter as EulerAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";
import {IPriceOracle as IOracle} from "./interfaces/IPriceOracle.sol";

contract AliasedImportContract {
    Token public token;

    function test() external {}
}
"#;

    let repo_root = PathBuf::from("/tmp/test-repo");
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-aliases".to_string(),
        root: repo_root.clone(),
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
        commit_hash: "test123".to_string(),
        audit_type: AuditType::Client,
    };

    let current_file = repo_root.join("src/AliasedImportContract.sol");
    let result = parse_all_import_dependencies(mock_solidity, &current_file, &repo).await;

    assert!(
        result.is_ok(),
        "Should handle aliased imports without crashing"
    );
}

/// Edge case test: Simple imports (no named imports)
#[tokio::test]
async fn test_simple_imports() {
    let mock_solidity = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./interfaces/IPriceOracle.sol";
import "../utils/Helper.sol";

contract SimpleImportContract {
    function test() external {}
}
"#;

    let repo_root = PathBuf::from("/tmp/test-repo");
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-simple-imports".to_string(),
        root: repo_root.clone(),
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
        commit_hash: "test123".to_string(),
        audit_type: AuditType::Client,
    };

    let current_file = repo_root.join("src/SimpleImportContract.sol");
    let result = parse_all_import_dependencies(mock_solidity, &current_file, &repo).await;

    assert!(
        result.is_ok(),
        "Should handle simple imports without crashing"
    );
}

/// Edge case test: Deeply nested relative imports
#[tokio::test]
async fn test_deeply_nested_imports() {
    let mock_solidity = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IVault} from "../../../interfaces/vault/IVault.sol";
import {Helper} from "../../utils/helpers/Helper.sol";
import {Constants} from "../../../../lib/Constants.sol";

contract DeeplyNestedContract {
    function test() external {}
}
"#;

    let repo_root = PathBuf::from("/tmp/test-repo");
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-nested".to_string(),
        root: repo_root.clone(),
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
        commit_hash: "test123".to_string(),
        audit_type: AuditType::Client,
    };

    let current_file = repo_root.join("src/contracts/core/vault/DeeplyNestedContract.sol");
    let result = parse_all_import_dependencies(mock_solidity, &current_file, &repo).await;

    assert!(
        result.is_ok(),
        "Should handle deeply nested relative imports without crashing"
    );
}

/// Edge case test: Mixed import styles
#[tokio::test]
async fn test_mixed_import_styles() {
    let mock_solidity = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Named imports
import {IERC20, SafeERC20} from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

// Simple import
import "@openzeppelin/contracts/access/Ownable.sol";

// Aliased import
import {IPriceOracle as IOracle} from "./interfaces/IPriceOracle.sol";

// Relative simple import
import "../utils/Helper.sol";

// Multiple named imports with aliases
import {
    BaseAdapter as Adapter,
    CrossAdapter,
    FixedRateOracle as FRO
} from "@euler-price-oracle/adapter/BaseAdapter.sol";

contract MixedImportContract {
    function test() external {}
}
"#;

    let repo_root = PathBuf::from("/tmp/test-repo");
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-mixed".to_string(),
        root: repo_root.clone(),
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
        commit_hash: "test123".to_string(),
        audit_type: AuditType::Client,
    };

    let current_file = repo_root.join("src/MixedImportContract.sol");
    let result = parse_all_import_dependencies(mock_solidity, &current_file, &repo).await;

    assert!(
        result.is_ok(),
        "Should handle mixed import styles without crashing"
    );
}
