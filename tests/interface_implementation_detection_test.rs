use ai_agent_audit::config::AuditType;
use ai_agent_audit::enumerator::interface_implementations::build_and_get_interface_implementation_index;
use ai_agent_audit::prepare_code::git_clone::{PocConfig, RepoPaths};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_interface_implementation_detection_with_interfaces_folder() {
    // Create a temporary directory structure that mimics a real project
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    println!("\n=== Testing Interface Implementation Detection ===");
    println!("Root: {}\n", root.display());

    // Create directory structure:
    // src/
    //   interfaces/
    //     IJackpot.sol
    //   Jackpot.sol

    let interfaces_dir = root.join("src/interfaces");
    fs::create_dir_all(&interfaces_dir).unwrap();

    let src_dir = root.join("src");

    // Create IJackpot interface
    let ijackpot_file = interfaces_dir.join("IJackpot.sol");
    fs::write(
        &ijackpot_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IJackpot {
    function buyTicket() external payable;
    function drawWinner() external;
}
"#,
    )
    .unwrap();

    // Create Jackpot contract that implements IJackpot
    let jackpot_file = src_dir.join("Jackpot.sol");
    fs::write(
        &jackpot_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IJackpot} from "./interfaces/IJackpot.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";

/**
 * @title Jackpot
 * @notice Main jackpot contract
 */
contract Jackpot is IJackpot, Ownable2Step {
    function buyTicket() external payable override {
        // Implementation
    }

    function drawWinner() external override {
        // Implementation
    }
}
"#,
    )
    .unwrap();

    // Create IERC20 interface (standard library interface)
    let ierc20_file = interfaces_dir.join("IERC20.sol");
    fs::write(
        &ierc20_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}
"#,
    )
    .unwrap();

    // Create a contract that implements IERC20
    let token_file = src_dir.join("Token.sol");
    fs::write(
        &token_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20} from "./interfaces/IERC20.sol";

contract Token is IERC20 {
    mapping(address => uint256) public balances;

    function transfer(address to, uint256 amount) external override returns (bool) {
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }

    function balanceOf(address account) external view override returns (uint256) {
        return balances[account];
    }
}
"#,
    )
    .unwrap();

    // Create RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/test/test-repo".to_string(),
        project_id: "test-repo-123".to_string(),
        root: root.to_path_buf(),
        sol_files: vec![
            ijackpot_file.clone(),
            jackpot_file.clone(),
            ierc20_file.clone(),
            token_file.clone(),
        ],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![src_dir.clone()],
        docs: vec![],
        repo_name: "test-repo".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "abc123".to_string(),
        audit_type: AuditType::Client,
        poc: PocConfig::default(),
    };

    // Debug: Print all files being scanned
    println!("📁 Files in repo:");
    for file in &repo.sol_files {
        println!("   - {}", file.display());

        // Read and check what extract_all_interface_implementations finds
        if let Ok(content) = std::fs::read_to_string(file) {
            use ai_agent_audit::enumerator::utils::SOLIDITY_REGEXES;
            let mut found_contracts = Vec::new();
            for cap in SOLIDITY_REGEXES.inheritance.captures_iter(&content) {
                let contract_name = cap.get(1).unwrap().as_str();
                let parent_list = cap.get(2).unwrap().as_str();
                found_contracts.push(format!("{} is {}", contract_name, parent_list));
            }
            if !found_contracts.is_empty() {
                println!("     Found contracts: {:?}", found_contracts);
            }
        }
    }

    // First, build the inheritance map by scanning all Solidity files
    // This is required before building the interface implementation index
    println!("\n🔨 Building inheritance map...");
    use ai_agent_audit::enumerator::utils::contracts_in_source_folder;
    let _ = contracts_in_source_folder(&repo).await.unwrap();
    println!("✅ Inheritance map built");

    // Build the interface implementation index
    println!("\n🔨 Building interface implementation index...");
    let index = build_and_get_interface_implementation_index(&repo)
        .await
        .unwrap();

    println!("\n📊 Interface Implementation Index:");
    println!("   Total interfaces found: {}", index.len());
    for ((interface_name, interface_file), implementations) in &index {
        println!(
            "\n   Interface: {} ({})",
            interface_name,
            interface_file.file_name().unwrap().to_string_lossy()
        );
        for (contract_name, contract_file) in implementations {
            println!(
                "     ✅ {} ({})",
                contract_name,
                contract_file.file_name().unwrap().to_string_lossy()
            );
        }
    }

    // Verify IJackpot has Jackpot as implementation
    let ijackpot_key = ("IJackpot".to_string(), ijackpot_file.clone());
    assert!(
        index.contains_key(&ijackpot_key),
        "❌ IJackpot interface should be in the index"
    );

    let jackpot_implementations = index.get(&ijackpot_key).unwrap();
    assert_eq!(
        jackpot_implementations.len(),
        1,
        "❌ IJackpot should have exactly 1 implementation"
    );
    assert_eq!(
        jackpot_implementations[0].0, "Jackpot",
        "❌ IJackpot should be implemented by Jackpot contract"
    );

    println!("\n✅ Test passed: IJackpot -> Jackpot implementation found!");

    // Verify IERC20 has Token as implementation
    let ierc20_key = ("IERC20".to_string(), ierc20_file.clone());
    assert!(
        index.contains_key(&ierc20_key),
        "❌ IERC20 interface should be in the index"
    );

    let token_implementations = index.get(&ierc20_key).unwrap();
    assert_eq!(
        token_implementations.len(),
        1,
        "❌ IERC20 should have exactly 1 implementation"
    );
    assert_eq!(
        token_implementations[0].0, "Token",
        "❌ IERC20 should be implemented by Token contract"
    );

    println!("✅ Test passed: IERC20 -> Token implementation found!");
}

#[tokio::test]
async fn test_interface_outside_interfaces_folder() {
    // Test case where interface is NOT in an interfaces/ folder
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    println!("\n=== Testing Interface NOT in interfaces/ folder ===");
    println!("Root: {}\n", root.display());

    let src_dir = root.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // Create interface in src/ (not in interfaces/)
    let imanager_file = src_dir.join("IManager.sol");
    fs::write(
        &imanager_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IManager {
    function manage() external;
}
"#,
    )
    .unwrap();

    // Create contract that implements IManager
    let manager_file = src_dir.join("Manager.sol");
    fs::write(
        &manager_file,
        r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IManager} from "./IManager.sol";

contract Manager is IManager {
    function manage() external override {
        // Implementation
    }
}
"#,
    )
    .unwrap();

    // Create RepoPaths
    let repo = RepoPaths {
        github_url: "https://github.com/test/test-repo-2".to_string(),
        project_id: "test-repo-456".to_string(),
        root: root.to_path_buf(),
        sol_files: vec![imanager_file.clone(), manager_file.clone()],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![src_dir.clone()],
        docs: vec![],
        repo_name: "test-repo-2".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "def456".to_string(),
        audit_type: AuditType::Client,
        poc: PocConfig::default(),
    };

    // First, build the inheritance map by scanning all Solidity files
    // This is required before building the interface implementation index
    println!("\n🔨 Building inheritance map...");
    use ai_agent_audit::enumerator::utils::contracts_in_source_folder;
    let _ = contracts_in_source_folder(&repo).await.unwrap();
    println!("✅ Inheritance map built");

    // Build the interface implementation index
    println!("\n🔨 Building interface implementation index...");
    let index = build_and_get_interface_implementation_index(&repo)
        .await
        .unwrap();

    println!("📊 Interface Implementation Index:");
    println!("   Total interfaces found: {}", index.len());
    for ((interface_name, interface_file), implementations) in &index {
        println!(
            "\n   Interface: {} ({})",
            interface_name,
            interface_file.file_name().unwrap().to_string_lossy()
        );
        for (contract_name, contract_file) in implementations {
            println!(
                "     ✅ {} ({})",
                contract_name,
                contract_file.file_name().unwrap().to_string_lossy()
            );
        }
    }

    // Verify IManager has Manager as implementation
    let imanager_key = ("IManager".to_string(), imanager_file.clone());
    assert!(
        index.contains_key(&imanager_key),
        "❌ IManager interface should be in the index"
    );

    let manager_implementations = index.get(&imanager_key).unwrap();
    assert_eq!(
        manager_implementations.len(),
        1,
        "❌ IManager should have exactly 1 implementation"
    );
    assert_eq!(
        manager_implementations[0].0, "Manager",
        "❌ IManager should be implemented by Manager contract"
    );

    println!("\n✅ Test passed: IManager -> Manager implementation found!");
}
