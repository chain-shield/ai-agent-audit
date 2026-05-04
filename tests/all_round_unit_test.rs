use ai_agent_audit::llm_review::phases::{
    rounds::all_rounds::{AllRoundLegitAnalysis, VerifyAllRound},
    verify_rounds::{FindingAnalysis, FindingStatus},
};

fn valid_analysis() -> AllRoundLegitAnalysis {
    AllRoundLegitAnalysis {
        finding_id: "H-1".to_string(),
        finding_title: "Reentrancy vulnerability".to_string(),
        does_bug_exist: true,
        safeguard_against_it: false,
        by_design: false,
        in_scope: true,
        exploitable: true,
        requires_user_mistake_without_protocol_fault: false,
        requires_privileged_or_compromised_actor: false,
        future_speculation: false,
        justification: "Valid reentrancy vulnerability".to_string(),
    }
}

#[test]
fn test_all_round_valid_finding() {
    let status = valid_analysis().get_finding_status_array_from_analysis();

    assert_eq!(status, None);
}

#[test]
fn test_all_round_bug_does_not_exist() {
    let mut analysis = valid_analysis();
    analysis.finding_title = "Fake vulnerability".to_string();
    analysis.does_bug_exist = false;
    analysis.justification = "Bug does not exist".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidBugDoesNotExist]));
}

#[test]
fn test_all_round_safeguard_in_place() {
    let mut analysis = valid_analysis();
    analysis.finding_title = "Reentrancy with guard".to_string();
    analysis.safeguard_against_it = true;
    analysis.justification = "Has nonReentrant modifier".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidSafeGuardInPlace]));
}

#[test]
fn test_all_round_out_of_scope() {
    let mut analysis = valid_analysis();
    analysis.finding_id = "M-1".to_string();
    analysis.finding_title = "Centralization risk".to_string();
    analysis.in_scope = false;
    analysis.justification = "Centralization risks are out of scope".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidOutOfScope]));
}

#[test]
fn test_all_round_by_design() {
    let mut analysis = valid_analysis();
    analysis.finding_id = "M-2".to_string();
    analysis.finding_title = "Intentional behavior".to_string();
    analysis.by_design = true;
    analysis.justification = "Documented as intentional".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidByDesign]));
}

#[test]
fn test_all_round_not_exploitable() {
    let mut analysis = valid_analysis();
    analysis.finding_id = "M-7".to_string();
    analysis.finding_title = "Theoretical issue".to_string();
    analysis.exploitable = false;
    analysis.justification = "Cannot create PoC".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidNotExploitable]));
}

#[test]
fn test_all_round_user_mistake_without_protocol_fault() {
    let mut analysis = valid_analysis();
    analysis.finding_id = "M-3".to_string();
    analysis.finding_title = "User mistake".to_string();
    analysis.requires_user_mistake_without_protocol_fault = true;
    analysis.justification = "Requires user to approve malicious contract".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidUserErrorOrMistake]));
}

#[test]
fn test_all_round_privileged_or_compromised_actor() {
    let mut analysis = valid_analysis();
    analysis.finding_id = "M-4".to_string();
    analysis.finding_title = "Admin misconfiguration".to_string();
    analysis.requires_privileged_or_compromised_actor = true;
    analysis.justification = "Requires admin to choose bad parameters".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidGovernanceRisk]));
}

#[test]
fn test_all_round_future_speculation() {
    let mut analysis = valid_analysis();
    analysis.finding_id = "M-5".to_string();
    analysis.finding_title = "Future integration risk".to_string();
    analysis.future_speculation = true;
    analysis.justification = "Only affects future integrations".to_string();

    let status = analysis.get_finding_status_array_from_analysis();

    assert_eq!(status, Some(vec![FindingStatus::InvalidFutureSpeculation]));
}

#[test]
fn test_all_round_multiple_issues() {
    let mut analysis = valid_analysis();
    analysis.finding_id = "M-5".to_string();
    analysis.finding_title = "Multiple issues".to_string();
    analysis.does_bug_exist = false;
    analysis.in_scope = false;
    analysis.requires_user_mistake_without_protocol_fault = true;
    analysis.justification = "Multiple reasons for invalidation".to_string();

    let statuses = analysis
        .get_finding_status_array_from_analysis()
        .expect("expected invalidation statuses");

    assert_eq!(statuses.len(), 3);
    assert!(statuses.contains(&FindingStatus::InvalidBugDoesNotExist));
    assert!(statuses.contains(&FindingStatus::InvalidOutOfScope));
    assert!(statuses.contains(&FindingStatus::InvalidUserErrorOrMistake));
}

#[test]
fn test_all_round_generate_verify_json() {
    let json = AllRoundLegitAnalysis::generate_verify_json();

    assert!(json.contains("\"findings\""));
    assert!(json.contains("\"finding_id\""));
    assert!(json.contains("\"finding_title\""));
    assert!(json.contains("\"does_bug_exist\""));
    assert!(json.contains("\"safeguard_against_it\""));
    assert!(json.contains("\"in_scope\""));
    assert!(json.contains("\"by_design\""));
    assert!(json.contains("\"exploitable\""));
    assert!(json.contains("\"requires_user_mistake_without_protocol_fault\""));
    assert!(json.contains("\"requires_privileged_or_compromised_actor\""));
    assert!(json.contains("\"future_speculation\""));
    assert!(json.contains("\"justification\""));

    assert!(!json.contains("\"impact\""));
    assert!(!json.contains("\"likelihood\""));
    assert!(!json.contains("\"user_error_or_mistake\""));
    assert!(!json.contains("\"governance_risk\""));
    assert!(!json.contains("\"non_standard_token\""));
}

#[test]
fn test_all_round_generate_verify_prompt() {
    let prompt = AllRoundLegitAnalysis::generate_verify_prompt();

    assert!(prompt.contains("not a severity judge"));
    assert!(prompt.contains("When uncertain, keep the finding alive"));
    assert!(prompt.contains("ROOT CAUSE EXISTS"));
    assert!(prompt.contains("COMPLETE SAFEGUARD EXISTS"));
    assert!(prompt.contains("MECHANICAL ANALYZED-CODE SCOPE"));
    assert!(prompt.contains("EXPLICITLY BY DESIGN"));
    assert!(prompt.contains("CURRENTLY EXPLOITABLE"));
    assert!(prompt.contains("REQUIRES PRIVILEGED OR COMPROMISED ACTOR"));
    assert!(prompt.contains("REQUIRES USER MISTAKE WITHOUT PROTOCOL FAULT"));
    assert!(prompt.contains("FUTURE SPECULATION"));

    assert!(prompt.contains("does bug exist"));
    assert!(prompt.contains("safeguard against it"));
    assert!(prompt.contains("in scope"));
    assert!(prompt.contains("by design"));
    assert!(prompt.contains("exploitable"));
    assert!(prompt.contains("requires user mistake without protocol fault"));
    assert!(prompt.contains("requires privileged or compromised actor"));
    assert!(prompt.contains("future speculation"));
    assert!(prompt.contains("justification"));

    assert!(!prompt.contains("IMPACT CLASSIFICATION CHECK"));
    assert!(!prompt.contains("LIKELIHOOD ASSESSMENT CHECK"));
    assert!(!prompt.contains("NON-STANDARD ERC20 TOKEN CHECK"));
    assert!(!prompt.contains("non standard token"));
}

#[test]
fn test_all_round_print_analysis_results() {
    let mut analysis = valid_analysis();
    analysis.finding_title = "Test Finding".to_string();
    analysis.justification = "Test justification".to_string();

    analysis.print_analysis_results();
}

#[test]
fn test_verify_all_round_struct() {
    let analysis1 = valid_analysis();
    let mut analysis2 = valid_analysis();
    analysis2.finding_id = "M-1".to_string();
    analysis2.finding_title = "Finding 2".to_string();
    analysis2.does_bug_exist = false;
    analysis2.justification = "Test 2".to_string();

    let verify_round = VerifyAllRound {
        findings: vec![analysis1, analysis2],
    };

    assert_eq!(verify_round.findings.len(), 2);
    assert_eq!(verify_round.findings[0].finding_id, "H-1");
    assert_eq!(verify_round.findings[1].finding_id, "M-1");
}
