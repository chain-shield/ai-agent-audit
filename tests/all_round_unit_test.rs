use ai_agent_audit::llm_review::phases::{
    rounds::{
        all_rounds::{AllRoundLegitAnalysis, VerifyAllRound},
        round_1::{Impact, Likelihood},
    },
    verify_rounds::{FindingAnalysis, FindingStatus},
};

#[test]
fn test_all_round_valid_finding() {
    // Finding that passes all checks
    let analysis = AllRoundLegitAnalysis {
        finding_id: "H-1".to_string(),
        finding_title: "Reentrancy vulnerability".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::High,
        likelihood: Likelihood::Common,
        justification: "Valid reentrancy vulnerability".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    // All checks pass, should return None (will be labeled as Valid)
    assert_eq!(status, None);
}

#[test]
fn test_all_round_bug_does_not_exist() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "H-1".to_string(),
        finding_title: "Fake vulnerability".to_string(),
        does_bug_exist: false, // Bug doesn't exist
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::High,
        likelihood: Likelihood::Common,
        justification: "Bug does not exist".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidBugDoesNotExist]));
}

#[test]
fn test_all_round_safeguard_in_place() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "H-1".to_string(),
        finding_title: "Reentrancy with guard".to_string(),
        does_bug_exist: true,
        safeguard_against_it: true, // Has safeguard
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::High,
        likelihood: Likelihood::Common,
        justification: "Has nonReentrant modifier".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidSafeGuardInPlace]));
}

#[test]
fn test_all_round_out_of_scope() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-1".to_string(),
        finding_title: "Centralization risk".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: false, // Out of scope
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Medium,
        likelihood: Likelihood::Common,
        justification: "Centralization risks are out of scope".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidOutOfScope]));
}

#[test]
fn test_all_round_by_design() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-2".to_string(),
        finding_title: "Intentional behavior".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: true, // By design
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Medium,
        likelihood: Likelihood::Common,
        justification: "Documented as intentional".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidByDesign]));
}

#[test]
fn test_all_round_not_exploitable() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-7".to_string(),
        finding_title: "Theoretical issue".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: false, // Not exploitable
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Medium, // Changed to Medium to avoid low impact flag
        likelihood: Likelihood::Occasional, // Changed to Occasional to avoid rare likelihood flag
        justification: "Cannot create PoC".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidNotExploitable]));
}

#[test]
fn test_all_round_user_error() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-3".to_string(),
        finding_title: "User mistake".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: true, // User error
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Medium,
        likelihood: Likelihood::Occasional,
        justification: "Requires user to approve malicious contract".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidUserErrorOrMistake]));
}

#[test]
fn test_all_round_governance_risk() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-4".to_string(),
        finding_title: "Admin misconfiguration".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: true, // Governance risk
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Medium,
        likelihood: Likelihood::Occasional,
        justification: "Admin can prevent by choosing correct parameters".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidGovernanceRisk]));
}

#[test]
fn test_all_round_future_speculation() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-5".to_string(),
        finding_title: "Future integration risk".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: true, // Future speculation
        non_standard_token: false,
        impact: Impact::Medium, // Changed to Medium to avoid low impact flag
        likelihood: Likelihood::Occasional, // Changed to Occasional to avoid rare likelihood flag
        justification: "Only affects future integrations".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidFutureSpeculation]));
}

#[test]
fn test_all_round_non_standard_token() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-6".to_string(),
        finding_title: "Fee-on-transfer token issue".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: true,           // Non-standard token
        impact: Impact::Medium,             // Changed to Medium to avoid low impact flag
        likelihood: Likelihood::Occasional, // Changed to Occasional to avoid rare likelihood flag
        justification: "Only affects fee-on-transfer tokens".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidERC20EdgeCase]));
}

#[test]
fn test_all_round_low_impact() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "L-4".to_string(),
        finding_title: "Dust amount issue".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Low, // Low impact
        likelihood: Likelihood::Common,
        justification: "Only affects dust amounts".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::LowSeverityDueToLowImpact]));
}

#[test]
fn test_all_round_rare_likelihood() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "L-5".to_string(),
        finding_title: "Rare condition".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::High,
        likelihood: Likelihood::Rare, // Rare likelihood
        justification: "Requires extreme market conditions".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(
        status,
        Some(vec![FindingStatus::LowSeverityDueToRareLikelihood])
    );
}

#[test]
fn test_all_round_multiple_issues() {
    // Finding with multiple downgrade reasons
    let analysis = AllRoundLegitAnalysis {
        finding_id: "M-5".to_string(),
        finding_title: "Multiple issues".to_string(),
        does_bug_exist: false, // Bug doesn't exist
        safeguard_against_it: false,
        by_design: false,
        in_scope: false, // Out of scope
        exploitable: true,
        user_error_or_mistake: true, // User error
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Medium,
        likelihood: Likelihood::Common,
        justification: "Multiple reasons for downgrade".to_string(),
    };

    let status = analysis.get_finding_status_array_from_analysis();

    // Should have all three downgrade reasons
    assert!(status.is_some());
    let statuses = status.unwrap();
    assert_eq!(statuses.len(), 3);
    assert!(statuses.contains(&FindingStatus::InvalidBugDoesNotExist));
    assert!(statuses.contains(&FindingStatus::InvalidOutOfScope));
    assert!(statuses.contains(&FindingStatus::InvalidUserErrorOrMistake));
}

#[test]
fn test_all_round_generate_verify_json() {
    let json = AllRoundLegitAnalysis::generate_verify_json();

    // Verify JSON structure contains all required fields
    assert!(json.contains("\"findings\""));
    assert!(json.contains("\"finding_id\""));
    assert!(json.contains("\"finding_title\""));
    assert!(json.contains("\"does_bug_exist\""));
    assert!(json.contains("\"safeguard_against_it\""));
    assert!(json.contains("\"in_scope\""));
    assert!(json.contains("\"by_design\""));
    assert!(json.contains("\"exploitable\""));
    assert!(json.contains("\"impact\""));
    assert!(json.contains("\"likelihood\""));
    assert!(json.contains("\"user_error_or_mistake\""));
    assert!(json.contains("\"governance_risk\""));
    assert!(json.contains("\"future_speculation\""));
    assert!(json.contains("\"non_standard_token\""));
    assert!(json.contains("\"justification\""));

    // Verify enum values are present
    assert!(json.contains("High"));
    assert!(json.contains("Medium"));
    assert!(json.contains("Low"));
    assert!(json.contains("Common"));
    assert!(json.contains("Occasional"));
    assert!(json.contains("Rare"));
}

#[test]
fn test_all_round_generate_verify_prompt() {
    let prompt = AllRoundLegitAnalysis::generate_verify_prompt();

    // Verify prompt contains all 11 checks
    assert!(prompt.contains("VERIFY SECURITY FINDING EXISTS"));
    assert!(prompt.contains("EXISTING SAFEGUARDS CHECK"));
    assert!(prompt.contains("SCOPE CHECK"));
    assert!(prompt.contains("BY DESIGN"));
    assert!(prompt.contains("EXPLOITABILITY"));
    assert!(prompt.contains("IMPACT CLASSIFICATION CHECK"));
    assert!(prompt.contains("LIKELIHOOD ASSESSMENT CHECK"));
    assert!(prompt.contains("USER ERROR CHECK"));
    assert!(prompt.contains("GOVERNANCE/CENTRALIZATION RISK"));
    assert!(prompt.contains("SPECULATION CHECK"));
    assert!(prompt.contains("NON-STANDARD ERC20 TOKEN CHECK"));

    // Verify step-by-step verification process
    assert!(prompt.contains("Step 1: Trace the Code Path"));
    assert!(prompt.contains("Step 2: Verify Invariant Actually Exists"));
    assert!(prompt.contains("Step 3: Reproduce the Issue"));

    // Verify output requirements
    assert!(prompt.contains("OUTPUT REQUIREMENTS"));
    assert!(prompt.contains("finding id"));
    assert!(prompt.contains("finding_title"));
    assert!(prompt.contains("does bug exist"));
    assert!(prompt.contains("safeguard against it"));
    assert!(prompt.contains("in scope"));
    assert!(prompt.contains("by design"));
    assert!(prompt.contains("exploitable"));
    assert!(prompt.contains("impact"));
    assert!(prompt.contains("likelihood"));
    assert!(prompt.contains("user error or mistake"));
    assert!(prompt.contains("governance risk"));
    assert!(prompt.contains("future speculation"));
    assert!(prompt.contains("non standard token"));
    assert!(prompt.contains("justification"));
}

#[test]
fn test_all_round_round_number() {
    assert_eq!(AllRoundLegitAnalysis::round_number(), 3);
}

#[test]
fn test_all_round_print_analysis_results() {
    let analysis = AllRoundLegitAnalysis {
        finding_id: "H-1".to_string(),
        finding_title: "Test Finding".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::High,
        likelihood: Likelihood::Common,
        justification: "Test justification".to_string(),
    };

    // This should not panic
    analysis.print_analysis_results();
}

#[test]
fn test_verify_all_round_struct() {
    let analysis1 = AllRoundLegitAnalysis {
        finding_id: "H-1".to_string(),
        finding_title: "Finding 1".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::High,
        likelihood: Likelihood::Common,
        justification: "Test 1".to_string(),
    };

    let analysis2 = AllRoundLegitAnalysis {
        finding_id: "M-1".to_string(),
        finding_title: "Finding 2".to_string(),
        does_bug_exist: false,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        user_error_or_mistake: false,
        governance_risk: false,
        future_speculation: false,
        non_standard_token: false,
        impact: Impact::Medium,
        likelihood: Likelihood::Occasional,
        justification: "Test 2".to_string(),
    };

    let verify_round = VerifyAllRound {
        findings: vec![analysis1, analysis2],
    };

    assert_eq!(verify_round.findings.len(), 2);
    assert_eq!(verify_round.findings[0].finding_id, "H-1");
    assert_eq!(verify_round.findings[1].finding_id, "M-1");
}
