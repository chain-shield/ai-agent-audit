/// Integration test for path canonicalization in inheritance map
///
/// This test verifies that the inheritance map correctly handles path variations
/// (e.g., /tmp/... vs /private/tmp/... on macOS) by canonicalizing all paths
/// before storing and querying.
use ai_agent_audit::{
    build_brain::inheritance_map::{get_parents_with_file, insert_inheritance_edge},
    config::AuditType,
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_path_canonicalization_in_inheritance_map() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test Solidity files
    let contracts_dir = temp_path.join("contracts");
    fs::create_dir_all(&contracts_dir)
        .await
        .expect("Failed to create contracts dir");

    let interface_file = contracts_dir.join("IToken.sol");
    let contract_file = contracts_dir.join("Token.sol");

    fs::write(
        &interface_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IToken {
    function transfer(address to, uint256 amount) external returns (bool);
}
"#,
    )
    .await
    .expect("Failed to write interface file");

    fs::write(
        &contract_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IToken.sol";

contract Token is IToken {
    function transfer(address to, uint256 amount) external returns (bool) {
        return true;
    }
}
"#,
    )
    .await
    .expect("Failed to write contract file");

    // Create a mock RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/test/test".to_string(),
        project_id: "test-path-canonicalization".to_string(),
        root: temp_path.to_path_buf(),
        sol_files: vec![interface_file.clone(), contract_file.clone()],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![contracts_dir.clone()],
        docs: vec![],
        poc: PocConfig {
            instructions: String::new(),
            template: String::new(),
            test_folder: temp_path.join("test"),
        },
        repo_name: "test".to_string(),
        audit_scope: None,
        audit_type: AuditType::Client,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "abc123".to_string(),
    };

    // Insert inheritance edge using the original path
    let child = ("Token".to_string(), contract_file.clone());
    let parent = ("IToken".to_string(), interface_file.clone());

    insert_inheritance_edge(child, parent, &repo)
        .await
        .expect("Failed to insert inheritance edge");

    // Query using the canonicalized path (which might have /private/ prefix on macOS)
    let canonical_contract_file = contract_file
        .canonicalize()
        .expect("Failed to canonicalize contract file");

    let parents = get_parents_with_file("Token", &canonical_contract_file, &repo)
        .await
        .expect("Failed to get parents");

    // Verify that we found the parent even though the paths might differ
    assert_eq!(
        parents.len(),
        1,
        "Expected to find 1 parent for Token contract"
    );
    assert_eq!(parents[0].0, "IToken", "Expected parent to be IToken");

    // Also test querying with the non-canonicalized path
    let parents_non_canonical = get_parents_with_file("Token", &contract_file, &repo)
        .await
        .expect("Failed to get parents with non-canonical path");

    assert_eq!(
        parents_non_canonical.len(),
        1,
        "Expected to find 1 parent even with non-canonical path"
    );
    assert_eq!(
        parents_non_canonical[0].0, "IToken",
        "Expected parent to be IToken"
    );

    println!("✅ Path canonicalization test passed!");
    println!("   Original path: {}", contract_file.display());
    println!("   Canonical path: {}", canonical_contract_file.display());
    println!("   Both paths correctly resolved to the same inheritance data");
}

#[tokio::test]
async fn test_interface_implementation_with_path_variations() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test Solidity files
    let contracts_dir = temp_path.join("contracts");
    fs::create_dir_all(&contracts_dir)
        .await
        .expect("Failed to create contracts dir");

    let interface_file = contracts_dir.join("IJackpot.sol");
    let contract_file = contracts_dir.join("Jackpot.sol");

    fs::write(
        &interface_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IJackpot {
    function play() external;
}
"#,
    )
    .await
    .expect("Failed to write interface file");

    fs::write(
        &contract_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IJackpot.sol";

contract Jackpot is IJackpot {
    function play() external {}
}
"#,
    )
    .await
    .expect("Failed to write contract file");

    // Create a mock RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/test/jackpot".to_string(),
        project_id: "test-jackpot-paths".to_string(),
        root: temp_path.to_path_buf(),
        sol_files: vec![interface_file.clone(), contract_file.clone()],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![contracts_dir.clone()],
        docs: vec![],
        poc: PocConfig {
            instructions: String::new(),
            template: String::new(),
            test_folder: temp_path.join("test"),
        },
        repo_name: "jackpot".to_string(),
        audit_scope: None,
        audit_type: AuditType::Client,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "def456".to_string(),
    };

    // Insert inheritance edge
    let child = ("Jackpot".to_string(), contract_file.clone());
    let parent = ("IJackpot".to_string(), interface_file.clone());

    insert_inheritance_edge(child, parent, &repo)
        .await
        .expect("Failed to insert inheritance edge");

    // Test with various path representations
    let paths_to_test = vec![
        contract_file.clone(),
        contract_file
            .canonicalize()
            .unwrap_or_else(|_| contract_file.clone()),
    ];

    for (i, path) in paths_to_test.iter().enumerate() {
        let parents = get_parents_with_file("Jackpot", path, &repo)
            .await
            .expect("Failed to get parents");

        assert_eq!(
            parents.len(),
            1,
            "Test variant {}: Expected to find 1 parent for Jackpot contract",
            i
        );
        assert_eq!(
            parents[0].0, "IJackpot",
            "Test variant {}: Expected parent to be IJackpot",
            i
        );

        println!("✅ Test variant {} passed with path: {}", i, path.display());
    }

    println!("✅ All path variation tests passed!");
}
