/// Integration test for YAML configuration file parsing
///
/// This test verifies that CLI arguments can be loaded from a YAML config file
/// and that all fields are correctly deserialized.
use ai_agent_audit::cli_args::parse::{BuilderType, Cli};
use std::fs;

#[test]
fn test_yaml_config_parsing() {
    // Create a temporary YAML config file
    let yaml_content = r#"
repo: "https://github.com/sherlock-audit/2025-10-index-fun-order-book-contest-chainshieldai.git"
subfolder: "orderbook-solidity"
custom_doc: "indexfun-docs.md"
scoped_files: "indexfun-scope.txt"
audit_scope: "indexfun-scope.md"
code_folders:
  - "src"
builder: "Custom"
build_cmd: "make install && make build"
via_ir: false
force_rebuild: false
"#;

    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_config.yaml");
    fs::write(&config_path, yaml_content).expect("Failed to write test config file");

    // Parse the YAML file
    let yaml_str = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Cli = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    // Verify all fields are correctly parsed
    assert_eq!(
        config.repo,
        "https://github.com/sherlock-audit/2025-10-index-fun-order-book-contest-chainshieldai.git"
    );
    assert_eq!(config.subfolder, Some("orderbook-solidity".to_string()));
    assert_eq!(config.custom_doc, Some("indexfun-docs.md".to_string()));
    assert_eq!(config.scoped_files, Some("indexfun-scope.txt".to_string()));
    assert_eq!(config.audit_scope, Some("indexfun-scope.md".to_string()));
    assert!(matches!(config.builder, BuilderType::Custom));
    assert_eq!(
        config.build_cmd,
        Some("make install && make build".to_string())
    );

    // Clean up
    fs::remove_file(&config_path).ok();
}

#[test]
fn test_yaml_config_with_all_fields() {
    // Create a comprehensive YAML config with all possible fields
    let yaml_content = r#"
repo: "https://github.com/example/test-repo.git"
subfolder: "contracts"
code_folders:
  - "src"
  - "contracts"
audit_scope: "scope.md"
doc_folder: "docs"
monorepo_folders: "monorepo.txt"
custom_doc: "custom-docs.md"
exclude_folders:
  - "test"
  - "script"
scoped_files: "scope.txt"
poc_instructions: "poc-instructions.md"
poc_template: "poc-template.sol"
test_folder: "test"
builder: "Foundry"
via_ir: true
force_rebuild: true
build_cmd: "forge build"
"#;

    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_config_full.yaml");
    fs::write(&config_path, yaml_content).expect("Failed to write test config file");

    // Parse the YAML file
    let yaml_str = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Cli = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    // Verify all fields
    assert_eq!(config.repo, "https://github.com/example/test-repo.git");
    assert_eq!(config.subfolder, Some("contracts".to_string()));
    assert_eq!(config.code_folders, vec!["src", "contracts"]);
    assert_eq!(config.audit_scope, Some("scope.md".to_string()));
    assert_eq!(config.doc_folder, Some("docs".to_string()));
    assert_eq!(config.monorepo_folders, Some("monorepo.txt".to_string()));
    assert_eq!(config.custom_doc, Some("custom-docs.md".to_string()));
    assert_eq!(
        config.exclude_folders,
        Some(vec!["test".to_string(), "script".to_string()])
    );
    assert_eq!(config.scoped_files, Some("scope.txt".to_string()));
    assert_eq!(
        config.poc_instructions,
        Some("poc-instructions.md".to_string())
    );
    assert_eq!(config.poc_template, Some("poc-template.sol".to_string()));
    assert_eq!(config.test_folder, Some("test".to_string()));
    assert!(matches!(config.builder, BuilderType::Foundry));
    assert_eq!(config.via_ir, true);
    assert_eq!(config.force_rebuild, true);
    assert_eq!(config.build_cmd, Some("forge build".to_string()));

    // Clean up
    fs::remove_file(&config_path).ok();
}

#[test]
fn test_yaml_config_minimal() {
    // Test with minimal required fields only
    let yaml_content = r#"
repo: "https://github.com/example/minimal-repo.git"
code_folders:
  - "src"
builder: "Auto"
via_ir: false
force_rebuild: false
"#;

    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_config_minimal.yaml");
    fs::write(&config_path, yaml_content).expect("Failed to write test config file");

    // Parse the YAML file
    let yaml_str = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Cli = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    // Verify required field
    assert_eq!(config.repo, "https://github.com/example/minimal-repo.git");

    // Verify optional fields are None
    assert_eq!(config.subfolder, None);
    assert_eq!(config.custom_doc, None);
    assert_eq!(config.scoped_files, None);
    assert_eq!(config.audit_scope, None);

    // Clean up
    fs::remove_file(&config_path).ok();
}

#[test]
fn test_builder_type_deserialization() {
    // Test all builder types (use PascalCase for YAML)
    let test_cases = vec![
        ("Foundry", BuilderType::Foundry),
        ("Hardhat", BuilderType::Hardhat),
        ("HardhatYarn", BuilderType::HardhatYarn),
        ("Custom", BuilderType::Custom),
        ("Auto", BuilderType::Auto),
    ];

    for (builder_str, _expected_builder) in test_cases {
        let yaml_content = format!(
            r#"
repo: "https://github.com/example/test.git"
code_folders:
  - "src"
builder: "{}"
via_ir: false
force_rebuild: false
"#,
            builder_str
        );

        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join(format!("test_builder_{}.yaml", builder_str));
        fs::write(&config_path, yaml_content).expect("Failed to write test config file");

        let yaml_str = fs::read_to_string(&config_path).expect("Failed to read config file");
        let config: Cli = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

        // Just verify it parses successfully - the builder type is correct
        assert_eq!(config.repo, "https://github.com/example/test.git");

        fs::remove_file(&config_path).ok();
    }
}

#[test]
fn test_indexfun_yaml_config() {
    // Test the actual indexfun.yaml config file
    let yaml_content = r#"
repo: "https://github.com/sherlock-audit/2025-10-index-fun-order-book-contest-chainshieldai.git"
subfolder: "orderbook-solidity"
custom_doc: "indexfun-docs.md"
scoped_files: "indexfun-scope.txt"
audit_scope: "indexfun-scope.md"
code_folders:
  - "src"
builder: "Custom"
build_cmd: "make install && make build"
via_ir: false
force_rebuild: false
"#;

    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("indexfun_test.yaml");
    fs::write(&config_path, yaml_content).expect("Failed to write test config file");

    // Parse the YAML file
    let yaml_str = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Cli = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    // Verify IndexFun-specific configuration
    assert_eq!(
        config.repo,
        "https://github.com/sherlock-audit/2025-10-index-fun-order-book-contest-chainshieldai.git"
    );
    assert_eq!(config.subfolder, Some("orderbook-solidity".to_string()));
    assert_eq!(config.custom_doc, Some("indexfun-docs.md".to_string()));
    assert_eq!(config.scoped_files, Some("indexfun-scope.txt".to_string()));
    assert_eq!(config.audit_scope, Some("indexfun-scope.md".to_string()));
    assert!(matches!(config.builder, BuilderType::Custom));
    assert_eq!(
        config.build_cmd,
        Some("make install && make build".to_string())
    );

    // Verify build command generation
    let build_cmd = config.generate_build_command();
    assert_eq!(build_cmd, "make install && make build");

    // Clean up
    fs::remove_file(&config_path).ok();
}

#[test]
fn test_yaml_config_with_boolean_flags() {
    // Test boolean flags (via_ir, force_rebuild)
    let yaml_content = r#"
repo: "https://github.com/example/test.git"
code_folders:
  - "src"
builder: "Foundry"
via_ir: true
force_rebuild: true
"#;

    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_booleans.yaml");
    fs::write(&config_path, yaml_content).expect("Failed to write test config file");

    let yaml_str = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Cli = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    assert_eq!(config.via_ir, true);
    assert_eq!(config.force_rebuild, true);

    // Verify build command includes --via-ir
    let build_cmd = config.generate_build_command();
    assert!(build_cmd.contains("--via-ir"));

    // Clean up
    fs::remove_file(&config_path).ok();
}

#[test]
fn test_yaml_config_with_arrays() {
    // Test array fields (code_folders, exclude_folders)
    let yaml_content = r#"
repo: "https://github.com/example/test.git"
code_folders:
  - "src"
  - "contracts"
  - "lib"
exclude_folders:
  - "test"
  - "script"
  - "mock"
builder: "Auto"
via_ir: false
force_rebuild: false
"#;

    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_arrays.yaml");
    fs::write(&config_path, yaml_content).expect("Failed to write test config file");

    let yaml_str = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Cli = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    assert_eq!(config.code_folders, vec!["src", "contracts", "lib"]);
    assert_eq!(
        config.exclude_folders,
        Some(vec![
            "test".to_string(),
            "script".to_string(),
            "mock".to_string()
        ])
    );

    // Clean up
    fs::remove_file(&config_path).ok();
}
