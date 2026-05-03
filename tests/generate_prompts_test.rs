//! Integration tests that print dynamic prompt outputs for visual review

use ai_agent_audit::{
    config::AuditType,
    llm_review::{
        dynamic_prompts::{
            findings as findings_prompts, findings_template as ft, invariants as inv_prompts,
        },
        findings::findings::PrivilegeLevel,
        threat_models::{
            invariants::{ContractInvariants, InvariantFinding, InvariantStatus, InvariantType},
            pattern_category::PatternCategory,
            patterns::{Pattern, VulnerabilityPattern},
        },
        utils::prompt_context,
    },
    prepare_code::git_clone::RepoPaths,
};
use std::path::PathBuf;

/// Helper to create a mock RepoPaths for testing
fn mock_repo_paths() -> RepoPaths {
    RepoPaths {
        github_url: "https://github.com/test/test-repo".to_string(),
        project_id: "test-project".to_string(),
        root: PathBuf::from("/tmp/test"),
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
        commit_hash: "test-commit-hash".to_string(),
        audit_type: AuditType::Code4rena,
    }
}

fn sample_invariant_finding() -> InvariantFinding {
    InvariantFinding {
        id: None,
        inv_type: InvariantType::Arithmetic,
        contract: "Vault".to_string(),
        function: "deposit".to_string(),
        predicate: "totalSupply() == sum(balances[*])".to_string(),
        desc: "Supply must equal the sum of balances".to_string(),
        checks: vec!["after deposit".into(), "after withdraw".into()],
        status: InvariantStatus::PossibleViolation,
        pre_state: Some("user has 100 tokens".into()),
        post_state: Some("totalSupply increased but balances didn't".into()),
        impact: Some("accounting mismatch may cause loss".into()),
    }
}

fn sample_pattern() -> Pattern {
    Pattern {
        issue_type: VulnerabilityPattern::Reentrancy,
        title: "reentrancy".to_string(),
        contract: "Vault".to_string(),
        function: "withdraw".to_string(),
        description: "External call made before state update allows reentrancy".to_string(),
        static_signals: vec![
            "external call before state change".into(),
            "no reentrancy guard".into(),
        ],
        assets_at_risk: vec!["treasury".into()],
        privilege: PrivilegeLevel::Permissionless,
        impact: None,
    }
}

#[test]
fn print_invariant_prompts() {
    let inv_types = [
        InvariantType::Arithmetic,
        InvariantType::Balance,
        InvariantType::Permission,
    ];
    let inv_prompt = inv_prompts::generate_invariant_prompt(&inv_types);
    println!("\n===== Invariant Discovery Prompt =====\n{}\n", inv_prompt);

    let inv_finding = sample_invariant_finding();
    let invariants = ContractInvariants {
        invariants: vec![inv_finding.clone()],
    };
    let inv_verify_prompt = inv_prompts::generate_all_invariants_verify_prompt(&invariants);
    println!(
        "\n===== All Invariants Verify Prompt =====\n{}\n",
        inv_verify_prompt
    );

    let inv_json = inv_prompts::get_invariant_json(&inv_types);
    println!("\n===== Invariant JSON Schema =====\n{}\n", inv_json);

    let inv_verify_json = inv_prompts::get_all_invariants_verify_json();
    println!(
        "\n===== All Invariants Verify JSON Schema =====\n{}\n",
        inv_verify_json
    );

    let inv_to_findings = inv_prompts::generate_formatted_invariant_finding(&inv_finding);
    println!(
        "\n===== Formatted Invariant Finding =====\n{}\n",
        inv_to_findings
    );
}

#[test]
fn print_pattern_prompts() {
    let category = PatternCategory::SignatureValidation;
    let repo = mock_repo_paths();
    let cat_prompt =
        findings_prompts::generate_pattern_category_to_findings_prompt(&category, &repo).unwrap();
    println!(
        "\n===== Pattern Category To Findings Prompt ({:?}) =====\n{}\n",
        category, cat_prompt
    );

    let pattern = sample_pattern();
    let formatted_pattern = prompt_context::generate_formatted_pattern(&pattern);
    println!("\n===== Formatted Pattern =====\n{}\n", formatted_pattern);

    let issues = [
        VulnerabilityPattern::Reentrancy,
        VulnerabilityPattern::ExternalCallAfterStateChange,
    ];
    let pre_schema = ft::get_pre_json_requirement_for_multipattern(
        &issues,
        "security vulnerability pattern",
        &repo,
    );
    println!("\n===== Pattern Pre-JSON Schema =====\n{}\n", pre_schema);

    let post_schema = ft::get_post_json_requirement_for_multipattern(
        &issues,
        "security vulnerability pattern",
        &repo,
    );
    println!("\n===== Pattern Post-JSON Schema =====\n{}\n", post_schema);
}

#[test]
fn print_findings_template_prompts() {
    let repo = mock_repo_paths();
    let issues = [
        VulnerabilityPattern::Reentrancy,
        VulnerabilityPattern::ExternalCallAfterStateChange,
    ];
    let templated = ft::get_pre_json_requirement_for_multipattern(
        &issues,
        "security vulnerability pattern",
        &repo,
    );
    println!(
        "\n===== Findings Template Pre-JSON Requirement =====\n{}\n",
        templated
    );

    let findings_json = ft::get_json_requirement(&issues, "security vulnerability pattern", &repo);
    println!(
        "\n===== Findings JSON Requirement =====\n{}\n",
        findings_json
    );
}
