//! Integration tests that print dynamic prompt outputs for visual review

use ai_agent_audit::llm_review::{
    dynamic_prompts::{
        findings_template as ft, inv_findings as inv_to_find, invariants as inv_prompts,
        pattern_findings as pat_to_find, patterns as pat_prompts,
    },
    enums::EnumData,
    findings::PrivilegeLevel,
    invariants::{InvariantFinding, InvariantStatus, InvariantType},
    pattern_category::PatternCategory,
    patterns::{Pattern, VulnerabilityPattern},
};

fn sample_invariant_finding() -> InvariantFinding {
    InvariantFinding {
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
    let inv_types = vec![
        InvariantType::Arithmetic,
        InvariantType::Balance,
        InvariantType::Permission,
    ];
    let inv_prompt = inv_prompts::generate_invariant_prompt(&inv_types);
    println!("\n===== Invariant Discovery Prompt =====\n{}\n", inv_prompt);

    let inv_finding = sample_invariant_finding();
    let inv_verify_prompt = inv_prompts::generate_invariant_verify_prompt(&inv_finding);
    println!(
        "\n===== Invariant Verify Prompt =====\n{}\n",
        inv_verify_prompt
    );

    let inv_json = inv_prompts::get_invariant_json(&inv_types);
    println!("\n===== Invariant JSON Schema =====\n{}\n", inv_json);

    let inv_verify_json = inv_prompts::get_invariant_verify_json();
    println!(
        "\n===== Invariant Verify JSON Schema =====\n{}\n",
        inv_verify_json
    );

    let inv_to_findings = inv_to_find::generate_invariant_to_findings(&inv_finding);
    println!(
        "\n===== Invariant -> Findings Prompt =====\n{}\n",
        inv_to_findings
    );
}

#[test]
fn print_pattern_prompts() {
    let category = PatternCategory::EconomicHit;
    let cat_prompt = pat_prompts::generate_pattern_category_prompt(&category);
    println!(
        "\n===== Pattern Category Prompt ({:?}) =====\n{}\n",
        category, cat_prompt
    );

    let pattern = sample_pattern();
    let verify_prompt = pat_prompts::generate_pattern_verify_prompt(&pattern);
    println!("\n===== Pattern Verify Prompt =====\n{}\n", verify_prompt);

    // Minimal JSON schemas
    let issues = [
        VulnerabilityPattern::Reentrancy,
        VulnerabilityPattern::ExternalCallAfterStateChange,
    ];
    let schema = pat_prompts::get_pattern_json_requirement(&issues);
    println!("\n===== Pattern JSON Schema =====\n{}\n", schema);

    let verify_schema = pat_prompts::get_pattern_verify_json();
    println!(
        "\n===== Pattern Verify JSON Schema =====\n{}\n",
        verify_schema
    );

    let pat_to_findings = pat_to_find::generate_pattern_to_findings_prompt(&pattern);
    println!(
        "\n===== Pattern -> Findings Prompt =====\n{}\n",
        pat_to_findings
    );
}

#[test]
fn print_findings_template_prompts() {
    // Use a vulnerability pattern spec to fill template
    let issue_type = VulnerabilityPattern::Reentrancy;
    let spec = issue_type.get_spec();

    let issue_title = "Security Vulnerability Pattern";
    let issue_definition = spec.definition;

    // Reuse the full report from a sample Pattern
    let pattern = sample_pattern();
    let issue_full_spec =
        ai_agent_audit::llm_review::utils::prompt_context::generate_formatted_pattern(&pattern);

    let templated = ft::generate_findings_prompt(
        issue_title,
        issue_definition,
        &issue_full_spec,
        &issue_type,
        issue_definition,
    );
    println!(
        "\n===== Findings Template with VulnerabilityPattern =====\n{}\n",
        templated
    );

    let findings_json = ft::get_findings_json_requirement(&issue_type, issue_definition);
    println!(
        "\n===== Findings JSON (from VulnerabilityPattern) =====\n{}\n",
        findings_json
    );
}
