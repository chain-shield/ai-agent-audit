use ai_agent_audit::{
    config::AuditType, enumerator::parse_solidity::detect_scripts_connected_to_contract,
    prepare_code::git_clone::RepoPaths,
};
use tempfile::TempDir;
use tokio::fs;

/// Helper to create a test RepoPaths with script files
async fn create_test_repo_with_scripts(
    scripts: Vec<(&str, &str)>, // (filename, content)
) -> (TempDir, RepoPaths) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_path_buf();

    // Use timestamp to ensure unique repo names and hashes across tests
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let repo_name = format!("test-repo-{}", timestamp);
    let commit_hash = format!("commit-{}", timestamp);

    let repo_root = root.join(&repo_name);

    fs::create_dir_all(&repo_root).await.unwrap();

    let mut script_files = Vec::new();

    for (filename, content) in scripts {
        let script_path = repo_root.join(filename);
        if let Some(parent) = script_path.parent() {
            fs::create_dir_all(parent).await.unwrap();
        }
        fs::write(&script_path, content).await.unwrap();
        script_files.push(script_path);
    }

    let repo_paths = RepoPaths {
        github_url: format!("https://github.com/test/{}", repo_name),
        project_id: "test-project".to_string(),
        root: root.clone(),
        sol_files: vec![],
        test_files: vec![],
        script_files,
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![repo_root.clone()],
        docs: vec![],
        monorepo_folders: None,
        audit_scope: None,
        scoped_files: None,
        excluded_folders: None,
        repo_name,
        commit_hash,
        audit_type: AuditType::Code4rena,
    };

    (temp_dir, repo_paths)
}

#[tokio::test]
async fn test_detect_scripts_with_direct_contract_reference() {
    let scripts = vec![
        (
            "script/Deploy.s.sol",
            r#"
                // SPDX-License-Identifier: MIT
                pragma solidity ^0.8.0;
                
                import {MyToken} from "../src/MyToken.sol";
                
                contract DeployScript {
                    function run() external {
                        MyToken token = new MyToken();
                    }
                }
            "#,
        ),
        (
            "script/Setup.s.sol",
            r#"
                // SPDX-License-Identifier: MIT
                pragma solidity ^0.8.0;
                
                import {OtherContract} from "../src/OtherContract.sol";
                
                contract SetupScript {
                    function run() external {
                        OtherContract other = new OtherContract();
                    }
                }
            "#,
        ),
    ];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(
        result.len(),
        1,
        "Should find exactly 1 script referencing MyToken"
    );

    let script_name = result
        .iter()
        .next()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(script_name, "Deploy.s.sol");
}

#[tokio::test]
async fn test_detect_scripts_with_multiple_references() {
    let scripts = vec![
        (
            "script/Deploy.s.sol",
            r#"
                import {MyToken} from "../src/MyToken.sol";
                contract DeployScript {
                    MyToken token;
                }
            "#,
        ),
        (
            "script/Setup.s.sol",
            r#"
                import {MyToken} from "../src/MyToken.sol";
                contract SetupScript {
                    function setup() {
                        MyToken token = MyToken(address(0x123));
                    }
                }
            "#,
        ),
        (
            "script/Unrelated.s.sol",
            r#"
                import {OtherContract} from "../src/OtherContract.sol";
                contract UnrelatedScript {}
            "#,
        ),
    ];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(result.len(), 2, "Should find 2 scripts referencing MyToken");

    let script_names: Vec<String> = result
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
        .collect();

    assert!(script_names.contains(&"Deploy.s.sol".to_string()));
    assert!(script_names.contains(&"Setup.s.sol".to_string()));
    assert!(!script_names.contains(&"Unrelated.s.sol".to_string()));
}

#[tokio::test]
async fn test_detect_scripts_no_matches() {
    let scripts = vec![(
        "script/Deploy.s.sol",
        r#"
                import {OtherContract} from "../src/OtherContract.sol";
                contract DeployScript {
                    OtherContract other;
                }
            "#,
    )];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(
        result.len(),
        0,
        "Should find no scripts referencing MyToken"
    );
}

#[tokio::test]
async fn test_detect_scripts_ignores_comments() {
    let scripts = vec![(
        "script/Deploy.s.sol",
        r#"
                // This script deploys MyToken
                /* 
                 * MyToken is deployed here
                 */
                import {OtherContract} from "../src/OtherContract.sol";
                contract DeployScript {
                    // MyToken token; // commented out
                    OtherContract other;
                }
            "#,
    )];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(result.len(), 0, "Should not match MyToken in comments");
}

#[tokio::test]
async fn test_detect_scripts_ignores_string_literals() {
    let scripts = vec![(
        "script/Deploy.s.sol",
        r#"
                import {OtherContract} from "../src/OtherContract.sol";
                contract DeployScript {
                    string public name = "MyToken Deployer";
                    OtherContract other;
                }
            "#,
    )];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(
        result.len(),
        0,
        "Should not match MyToken in string literals"
    );
}

#[tokio::test]
async fn test_detect_scripts_word_boundary_matching() {
    let scripts = vec![
        (
            "script/Deploy.s.sol",
            r#"
                import {MyTokenFactory} from "../src/MyTokenFactory.sol";
                contract DeployScript {
                    MyTokenFactory factory;
                }
            "#,
        ),
        (
            "script/Setup.s.sol",
            r#"
                import {MyToken} from "../src/MyToken.sol";
                contract SetupScript {
                    MyToken token;
                }
            "#,
        ),
    ];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    // Should only match "MyToken" exactly, not "MyTokenFactory"
    assert_eq!(
        result.len(),
        1,
        "Should match MyToken but not MyTokenFactory"
    );

    let script_name = result
        .iter()
        .next()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(script_name, "Setup.s.sol");
}

#[tokio::test]
async fn test_detect_scripts_caching() {
    let scripts = vec![(
        "script/Deploy.s.sol",
        r#"
                import {MyToken} from "../src/MyToken.sol";
                contract DeployScript {
                    MyToken token;
                }
            "#,
    )];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    // First call - should populate cache
    let result1 = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    // Second call - should use cache
    let result2 = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(result1, result2, "Cached result should match first result");
    assert_eq!(result1.len(), 1);
}

#[tokio::test]
async fn test_detect_scripts_empty_script_list() {
    let scripts = vec![];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(
        result.len(),
        0,
        "Should return empty set when no scripts exist"
    );
}

#[tokio::test]
async fn test_detect_scripts_with_new_keyword() {
    let scripts = vec![(
        "script/Deploy.s.sol",
        r#"
                contract DeployScript {
                    function run() external {
                        MyToken token = new MyToken();
                        token.initialize();
                    }
                }
            "#,
    )];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(
        result.len(),
        1,
        "Should detect MyToken used with 'new' keyword"
    );
}

#[tokio::test]
async fn test_detect_scripts_with_type_casting() {
    let scripts = vec![(
        "script/Interact.s.sol",
        r#"
                contract InteractScript {
                    function run() external {
                        MyToken token = MyToken(0x1234567890123456789012345678901234567890);
                        token.transfer(address(0), 100);
                    }
                }
            "#,
    )];

    let (_temp_dir, repo) = create_test_repo_with_scripts(scripts).await;

    let result = detect_scripts_connected_to_contract("MyToken", &repo)
        .await
        .unwrap();

    assert_eq!(
        result.len(),
        1,
        "Should detect MyToken used in type casting"
    );
}
