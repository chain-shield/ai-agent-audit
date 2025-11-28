use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

use ai_agent_audit::config::AuditType;
use ai_agent_audit::prepare_code::git_clone::{PocConfig, RepoPaths};
use ai_agent_audit::utils::remapping::{
    get_all_remappings, parse_and_store_remappings, resolve_import_path,
};

// Helper function to create a minimal test RepoPaths
fn create_test_repo(project_id: &str, root: PathBuf) -> RepoPaths {
    RepoPaths {
        github_url: "https://github.com/test/test.git".to_string(),
        project_id: project_id.to_string(),
        root,
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
        commit_hash: "0000000000000000000000000000000000000000".to_string(),
        audit_type: AuditType::Client,
        poc: PocConfig {
            test_folder: PathBuf::from("test"),
            template: String::new(),
            instructions: String::new(),
        },
    }
}

#[test]
fn test_remapping_integration() {
    // Create a temporary directory structure
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_path_buf();

    // Create a realistic remappings.txt file
    let remapping_file = root.join("remappings.txt");
    let mut file = fs::File::create(&remapping_file).unwrap();
    writeln!(
        file,
        r#"
# OpenZeppelin contracts
@openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/
@openzeppelin/contracts-upgradeable/=node_modules/@openzeppelin/contracts-upgradeable/

# Uniswap
@uniswap/v3-core/=lib/v3-core/
@uniswap/v3-periphery/=lib/v3-periphery/

# Other dependencies
@ensdomains/=node_modules/@ensdomains/
@chainlink/contracts/=lib/chainlink/contracts/

# Non-@ remappings (should be ignored)
forge-std/=lib/forge-std/src/
ds-test/=lib/forge-std/lib/ds-test/src/
solmate/=lib/solmate/src/

# Comments and empty lines should be ignored
// Another comment style
"#
    )
    .unwrap();

    let project_id = "test-integration";
    let repo = create_test_repo(project_id, root.clone());

    println!("\n=== Testing Remapping Integration ===\n");

    // Parse and store remappings
    println!("1. Parsing remappings.txt...");
    let count = parse_and_store_remappings(&remapping_file, project_id).unwrap();
    println!("   Parsed {} remappings\n", count);
    assert_eq!(count, 9); // All remappings (@ prefixed and non-@ prefixed)

    // Get all remappings
    println!("2. Retrieving all remappings...");
    let all_remappings = get_all_remappings(&repo).unwrap();
    println!("   Total remappings: {}", all_remappings.len());
    for (prefix, target) in all_remappings.iter() {
        println!("   {} -> {}", prefix, target);
    }
    println!();
    assert_eq!(all_remappings.len(), 9);

    // Test individual remapping retrieval via get_all_remappings
    println!("3. Testing individual remapping retrieval...");
    let oz_target = all_remappings.get("@openzeppelin/contracts/");
    assert_eq!(
        oz_target,
        Some(&"lib/openzeppelin-contracts/contracts/".to_string())
    );
    println!("   ✓ @openzeppelin/contracts/ -> {:?}\n", oz_target);

    // Test import path resolution
    println!("4. Testing import path resolution...");

    let test_cases = vec![
        (
            "@openzeppelin/contracts/token/ERC20/ERC20.sol",
            Some("lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol"),
        ),
        (
            "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol",
            Some("node_modules/@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol"),
        ),
        (
            "@uniswap/v3-periphery/interfaces/ISwapRouter.sol",
            Some("lib/v3-periphery/interfaces/ISwapRouter.sol"),
        ),
        (
            "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol",
            Some("lib/chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol"),
        ),
        (
            "./MyContract.sol",
            None, // Relative import, no remapping
        ),
        (
            "contracts/MyContract.sol",
            None, // No matching remapping
        ),
        (
            "forge-std/Test.sol",
            Some("lib/forge-std/src/Test.sol"), // Non-@ remappings are now included
        ),
    ];

    for (import_path, expected) in test_cases {
        let resolved = resolve_import_path(import_path, &repo);
        println!("   {} -> {:?}", import_path, resolved);
        assert_eq!(resolved, expected.map(|s| s.to_string()));
    }

    println!("\n=== All integration tests passed ===\n");
}

#[test]
fn test_remapping_with_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_path_buf();

    println!("\n=== Testing Multiple Remapping Files ===\n");

    // Create first remappings.txt (Foundry style)
    let remapping1 = root.join("remappings.txt");
    let mut file1 = fs::File::create(&remapping1).unwrap();
    writeln!(
        file1,
        r#"
@openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/
@uniswap/v3-core/=lib/v3-core/
"#
    )
    .unwrap();

    // Create second remapping.txt (alternative name) in a different temp dir
    let temp_dir2 = TempDir::new().unwrap();
    let root2 = temp_dir2.path().to_path_buf();
    let remapping2 = root2.join("remapping.txt");
    let mut file2 = fs::File::create(&remapping2).unwrap();
    writeln!(
        file2,
        r#"
@chainlink/contracts/=lib/chainlink/contracts/
@ensdomains/=node_modules/@ensdomains/
"#
    )
    .unwrap();

    let project1 = "project-foundry";
    let project2 = "project-hardhat";
    let repo1 = create_test_repo(project1, root.clone());
    let repo2 = create_test_repo(project2, root2.clone());

    // Parse both files for different projects
    println!("1. Parsing first remappings.txt for project-foundry...");
    let count1 = parse_and_store_remappings(&remapping1, project1).unwrap();
    println!("   Parsed {} remappings\n", count1);

    println!("2. Parsing second remapping.txt for project-hardhat...");
    let count2 = parse_and_store_remappings(&remapping2, project2).unwrap();
    println!("   Parsed {} remappings\n", count2);

    // Verify each project has its own remappings
    println!("3. Verifying project isolation...");
    let remappings1 = get_all_remappings(&repo1).unwrap();
    let remappings2 = get_all_remappings(&repo2).unwrap();

    println!("   Project 1 has {} remappings", remappings1.len());
    println!("   Project 2 has {} remappings", remappings2.len());

    assert_eq!(remappings1.len(), 2);
    assert_eq!(remappings2.len(), 2);

    // Verify correct remappings for each project
    assert!(remappings1.contains_key("@openzeppelin/contracts/"));
    assert!(remappings1.contains_key("@uniswap/v3-core/"));
    assert!(!remappings1.contains_key("@chainlink/contracts/"));

    assert!(remappings2.contains_key("@chainlink/contracts/"));
    assert!(remappings2.contains_key("@ensdomains/"));
    assert!(!remappings2.contains_key("@openzeppelin/contracts/"));

    println!("   ✓ Projects have isolated remappings\n");

    // Test resolution for each project
    println!("4. Testing import resolution per project...");
    let oz_import = "@openzeppelin/contracts/token/ERC20/ERC20.sol";
    let chainlink_import = "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

    let resolved1 = resolve_import_path(oz_import, &repo1);
    let resolved2 = resolve_import_path(oz_import, &repo2);
    let resolved3 = resolve_import_path(chainlink_import, &repo1);
    let resolved4 = resolve_import_path(chainlink_import, &repo2);

    println!("   Project 1 resolves OZ: {:?}", resolved1);
    println!("   Project 2 resolves OZ: {:?}", resolved2);
    println!("   Project 1 resolves Chainlink: {:?}", resolved3);
    println!("   Project 2 resolves Chainlink: {:?}", resolved4);

    assert!(resolved1.is_some()); // Project 1 has OZ
    assert!(resolved2.is_none()); // Project 2 doesn't have OZ
    assert!(resolved3.is_none()); // Project 1 doesn't have Chainlink
    assert!(resolved4.is_some()); // Project 2 has Chainlink

    println!("\n=== All multi-file tests passed ===\n");
}

#[test]
fn test_remapping_edge_cases() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_path_buf();

    println!("\n=== Testing Remapping Edge Cases ===\n");

    // Create remappings with various edge cases
    let remapping_file = root.join("remappings.txt");
    let mut file = fs::File::create(&remapping_file).unwrap();
    writeln!(
        file,
        r#"
# Edge case: trailing slashes
@openzeppelin/contracts/=lib/oz/contracts/
@uniswap/v3-core=lib/v3-core

# Edge case: whitespace
  @chainlink/contracts/  =  lib/chainlink/

# Edge case: empty lines


# Edge case: only comments
# @commented/out/=lib/commented/

# Edge case: no @ prefix (should be ignored)
solmate/=lib/solmate/src/
forge-std/=lib/forge-std/src/

# Edge case: multiple = signs
@weird/path/=lib/weird=path/
"#
    )
    .unwrap();

    let project_id = "edge-cases";
    let repo = create_test_repo(project_id, root.clone());

    println!("1. Parsing edge case remappings...");
    let count = parse_and_store_remappings(&remapping_file, project_id).unwrap();
    println!("   Parsed {} remappings\n", count);

    let remappings = get_all_remappings(&repo).unwrap();

    println!("2. Verifying parsed remappings:");
    for (prefix, target) in remappings.iter() {
        println!("   '{}' -> '{}'", prefix, target);
    }
    println!();

    // Verify whitespace is trimmed
    assert!(remappings.contains_key("@chainlink/contracts/"));
    let chainlink_target = remappings.get("@chainlink/contracts/").unwrap();
    assert_eq!(chainlink_target, "lib/chainlink/");

    // Verify non-@ remappings are now included (implementation changed)
    assert!(remappings.contains_key("solmate/"));
    assert!(remappings.contains_key("forge-std/"));

    // Verify commented lines are ignored
    assert!(!remappings.contains_key("@commented/out/"));

    println!("3. Testing resolution with edge cases...");
    let resolved = resolve_import_path(
        "@chainlink/contracts/interfaces/AggregatorV3Interface.sol",
        &repo,
    );
    println!("   Resolved: {:?}", resolved);
    assert_eq!(
        resolved,
        Some("lib/chainlink/interfaces/AggregatorV3Interface.sol".to_string())
    );

    println!("\n=== All edge case tests passed ===\n");
}
