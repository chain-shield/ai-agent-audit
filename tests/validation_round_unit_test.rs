/// Unit tests for the validation round feature
/// Tests the ValidateLegitAnalysis logic for validating downgraded findings
use ai_agent_audit::llm_review::{
    findings::{
        finding_enums::{Severity, VulnerabilityType},
        findings::{Finding, Findings, PrivilegeLevel},
    },
    phases::{
        rounds::validate_round::{
            ValidateLegitAnalysis, generate_dynamic_validation_json,
            generate_round_validation_prompt,
        },
        verify_rounds::FindingStatus,
    },
};

#[test]
fn test_validate_all_reasons_confirmed_invalid() {
    // Test: All downgrade reasons are confirmed (true) -> finding stays invalid
    let finding = create_test_finding_with_status(vec![
        FindingStatus::InvalidBugDoesNotExist,
        FindingStatus::InvalidOutOfScope,
    ]);

    let validation = ValidateLegitAnalysis {
        finding_id: "test-1".to_string(),
        finding_title: "Test Finding".to_string(),
        does_bug_really_not_exist: Some(true),
        is_really_out_of_scope: Some(true),
        justification: Some("Both reasons confirmed".to_string()),
        ..Default::default()
    };

    let result = validation.get_fixed_finding_status(&finding);

    assert!(result.is_some());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 2);
    assert!(statuses.contains(&FindingStatus::InvalidBugDoesNotExist));
    assert!(statuses.contains(&FindingStatus::InvalidOutOfScope));
}

#[test]
fn test_validate_all_reasons_rejected_upgrade_to_valid() {
    // Test: All downgrade reasons are rejected (false) -> finding upgraded to Valid
    let finding = create_test_finding_with_status(vec![
        FindingStatus::InvalidBugDoesNotExist,
        FindingStatus::InvalidUserErrorOrMistake,
    ]);

    let validation = ValidateLegitAnalysis {
        finding_id: "test-2".to_string(),
        finding_title: "Test Finding".to_string(),
        does_bug_really_not_exist: Some(false),
        is_really_user_error_or_mistake: Some(false),
        justification: Some("Both reasons rejected - finding is valid".to_string()),
        ..Default::default()
    };

    let result = validation.get_fixed_finding_status(&finding);

    // When all reasons are rejected, should return None (will be upgraded to Valid)
    assert!(result.is_none());
}

#[test]
fn test_validate_partial_confirmation() {
    // Test: Some reasons confirmed, some rejected -> only confirmed reasons remain
    let finding = create_test_finding_with_status(vec![
        FindingStatus::InvalidBugDoesNotExist,
        FindingStatus::InvalidGovernanceRisk,
        FindingStatus::InvalidUserErrorOrMistake,
    ]);

    let validation = ValidateLegitAnalysis {
        finding_id: "test-3".to_string(),
        finding_title: "Test Finding".to_string(),
        does_bug_really_not_exist: Some(false), // Rejected
        is_really_governance_risk: Some(true),  // Confirmed
        is_really_user_error_or_mistake: Some(false), // Rejected
        justification: Some("Only governance risk is valid".to_string()),
        ..Default::default()
    };

    let result = validation.get_fixed_finding_status(&finding);

    assert!(result.is_some());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 1);
    assert!(statuses.contains(&FindingStatus::InvalidGovernanceRisk));
}

#[test]
fn test_validate_low_severity_reasons() {
    // Test: Low severity reasons (impact/likelihood) are validated correctly
    let finding = create_test_finding_with_status(vec![
        FindingStatus::LowSeverityDueToLowImpact,
        FindingStatus::LowSeverityDueToRareLikelihood,
    ]);

    let validation = ValidateLegitAnalysis {
        finding_id: "test-4".to_string(),
        finding_title: "Test Finding".to_string(),
        is_really_low_impact: Some(true),
        is_really_low_likelihood: Some(false), // Rejected
        justification: Some("Impact is low but likelihood is not rare".to_string()),
        ..Default::default()
    };

    let result = validation.get_fixed_finding_status(&finding);

    assert!(result.is_some());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 1);
    assert!(statuses.contains(&FindingStatus::LowSeverityDueToLowImpact));
}

#[test]
fn test_validate_finding_already_valid() {
    // Test: Finding already marked as Valid should remain Valid
    let finding = create_test_finding_with_status(vec![FindingStatus::Valid]);

    let validation = ValidateLegitAnalysis {
        finding_id: "test-5".to_string(),
        finding_title: "Test Finding".to_string(),
        ..Default::default()
    };

    let result = validation.get_fixed_finding_status(&finding);

    assert!(result.is_some());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 1);
    assert!(statuses.contains(&FindingStatus::Valid));
}

#[test]
fn test_validate_finding_no_status() {
    // Test: Finding with no status should return None
    let mut finding = create_test_finding();
    finding.status = None;

    let validation = ValidateLegitAnalysis {
        finding_id: "test-6".to_string(),
        finding_title: "Test Finding".to_string(),
        ..Default::default()
    };

    let result = validation.get_fixed_finding_status(&finding);
    assert!(result.is_none());
}

#[test]
fn test_validate_all_invalid_status_types() {
    // Test: All invalid status types are handled correctly
    let finding = create_test_finding_with_status(vec![
        FindingStatus::InvalidBugDoesNotExist,
        FindingStatus::InvalidOutOfScope,
        FindingStatus::InvalidUserErrorOrMistake,
        FindingStatus::InvalidGovernanceRisk,
        FindingStatus::InvalidERC20EdgeCase,
        FindingStatus::InvalidNotExploitable,
        FindingStatus::InvalidFutureSpeculation,
        FindingStatus::InvalidByDesign,
        FindingStatus::InvalidSafeGuardInPlace,
    ]);

    let validation = ValidateLegitAnalysis {
        finding_id: "test-7".to_string(),
        finding_title: "Test Finding".to_string(),
        does_bug_really_not_exist: Some(true),
        is_really_out_of_scope: Some(true),
        is_really_user_error_or_mistake: Some(true),
        is_really_governance_risk: Some(true),
        is_really_non_standard_token: Some(true),
        is_really_not_exploitable: Some(true),
        is_really_future_speculation: Some(true),
        is_bug_really_by_design: Some(true),
        is_there_really_safeguard_against_it: Some(true),
        justification: Some("All reasons confirmed".to_string()),
        ..Default::default()
    };

    let result = validation.get_fixed_finding_status(&finding);

    assert!(result.is_some());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 9);
}

#[test]
fn test_generate_validation_prompt_single_finding() {
    // Test: Prompt generation for a single downgraded finding
    let finding = create_test_finding_with_status(vec![FindingStatus::InvalidBugDoesNotExist]);
    let findings = Findings {
        findings: vec![finding],
    };

    let prompt = generate_round_validation_prompt(&findings);

    assert!(prompt.contains("evaluate EACH of the below triaged 1 security findings"));
    assert!(prompt.contains("finding does not exist"));
    assert!(prompt.contains("Justifcation for downgrade"));
}

#[test]
fn test_generate_validation_prompt_multiple_findings() {
    // Test: Prompt generation for multiple downgraded findings
    let finding1 = create_test_finding_with_status(vec![FindingStatus::InvalidBugDoesNotExist]);
    let finding2 = create_test_finding_with_status(vec![FindingStatus::InvalidGovernanceRisk]);
    let findings = Findings {
        findings: vec![finding1, finding2],
    };

    let prompt = generate_round_validation_prompt(&findings);

    assert!(prompt.contains("evaluate EACH of the below triaged 2 security findings"));
    assert!(prompt.contains("finding does not exist"));
    assert!(prompt.contains("governance risk"));
}

#[test]
fn test_generate_dynamic_validation_json_single_finding() {
    // Test: Dynamic JSON generation for a single finding
    let finding = create_test_finding_with_status(vec![
        FindingStatus::InvalidBugDoesNotExist,
        FindingStatus::InvalidOutOfScope,
    ]);
    let findings = Findings {
        findings: vec![finding],
    };

    let json = generate_dynamic_validation_json(&findings);

    assert!(json.contains("OUTPUT REQUIREMENTS"));
    assert!(json.contains("\"finding_id\""));
    assert!(json.contains("\"finding_title\""));
    assert!(json.contains("\"does_bug_really_not_exist\": true | false"));
    assert!(json.contains("\"is_really_out_of_scope\": true | false"));
    assert!(json.contains("\"justification\""));
}

#[test]
fn test_generate_dynamic_validation_json_multiple_findings() {
    // Test: Dynamic JSON generation for multiple findings with different statuses
    let finding1 = create_test_finding_with_status(vec![FindingStatus::InvalidBugDoesNotExist]);
    let finding2 = create_test_finding_with_status(vec![
        FindingStatus::InvalidGovernanceRisk,
        FindingStatus::LowSeverityDueToLowImpact,
    ]);
    let findings = Findings {
        findings: vec![finding1, finding2],
    };

    let json = generate_dynamic_validation_json(&findings);

    // Should have two finding objects
    assert!(json.contains("\"does_bug_really_not_exist\": true | false"));
    assert!(json.contains("\"is_really_governance_risk\": true | false"));
    assert!(json.contains("\"is_really_low_impact\": true | false"));
}

fn create_test_finding() -> Finding {
    Finding {
        id: Some("test-finding-1".to_string()),
        title: "Test Reentrancy Vulnerability".to_string(),
        exploit_type: VulnerabilityType::Reentrancy,
        privilege: PrivilegeLevel::Permissionless,
        contract: "TestContract".to_string(),
        function: "testFunction".to_string(),
        description: Some("Test description".to_string()),
        impact: Some("Test impact".to_string()),
        proof_of_concept: Some("Test PoC".to_string()),
        proof_of_code: Some("Test code".to_string()),
        severity: Severity::High,
        mitigation: Some("Test mitigation".to_string()),
        status: None,
        status_justification: Some("Test justification".to_string()),
        poc_test_file: None,
        poc_test_command: None,
        poc_test_status: None,
        competition_report: None,
        finding_complexity: Some(3),
        derived_from: Some("Test Pattern".to_string()),
    }
}

fn create_test_finding_with_status(statuses: Vec<FindingStatus>) -> Finding {
    let mut finding = create_test_finding();
    finding.status = Some(statuses);
    finding.status_justification = Some("Downgraded for testing".to_string());
    finding
}
