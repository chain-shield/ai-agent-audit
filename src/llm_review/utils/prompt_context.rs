use crate::llm_review::{
    findings::Finding,
    invariants::InvariantFinding,
    patterns::{ImpactHint, Pattern},
    phases::verify_findings::{FindingConfidence, FindingStatus},
};

pub fn generate_prompt_for_issue_check(
    code: &str,
    finding: &Finding,
    instructions: &str,
    post_instructions: &str,
    report_type: FindingReportType,
) -> String {
    let mut prompt = format!("{}{}", instructions, post_instructions);

    prompt.push_str("\n\n");
    prompt.push_str("## REPORT FOR SECURITY FINDING");
    prompt.push_str("\n\n");

    let report = get_finding_report(finding, None, report_type);
    prompt.push_str(&report);
    prompt.push_str("\n\n");

    prompt.push_str("## CODEBASE WHERE FINDING WAS FOUND");
    prompt.push_str("\n\n");

    prompt.push_str(code);

    prompt
}

pub fn generate_formatted_invariant_finding(invariant: &InvariantFinding) -> String {
    let mut invariant_finding = String::new();

    invariant_finding.push_str(&format!(
        "\n\n ### Invariant Type: {}\n",
        &invariant.inv_type.to_string()
    ));

    invariant_finding.push_str(&format!(
        "\n ### Relevant Function/Location: {}.{}\n",
        invariant.contract, invariant.function
    ));

    invariant_finding.push_str("\n ### Description/Code Snippet\n");
    invariant_finding.push_str(&invariant.desc.to_string());

    invariant_finding.push_str("\n ### Checks\n");
    invariant_finding.push_str(&invariant.checks.join(", "));

    invariant_finding.push_str(&format!(
        "\n ### Status: {}\n",
        &invariant.status.to_string()
    ));

    invariant_finding.push_str("\n ### Pre-State\n");
    invariant_finding.push_str(&invariant.pre_state.clone().unwrap_or_default());

    invariant_finding.push_str("\n ### Post-State\n");
    invariant_finding.push_str(&invariant.post_state.clone().unwrap_or_default());

    invariant_finding.push_str("\n ### Impact\n");
    invariant_finding.push_str(&invariant.impact.clone().unwrap_or_default());

    invariant_finding
}

pub fn generate_formatted_pattern(pattern: &Pattern) -> String {
    let mut pattern_list = String::new();

    pattern_list.push_str(&format!(
        "\n\n ### Issue Type: {}\n",
        &pattern.issue_type.to_string()
    ));

    pattern_list.push_str(&format!(
        "\n ### Relevant Function/Location: {}.{}\n",
        pattern.contract, pattern.function
    ));

    pattern_list.push_str("\n ### Title\n");
    pattern_list.push_str(&pattern.description.to_string());

    pattern_list.push_str("\n ### Description/Code Snippet\n");
    pattern_list.push_str(&pattern.description.to_string());

    pattern_list.push_str("\n ### Static Signals\n");
    pattern_list.push_str(&pattern.static_signals.join(", "));

    pattern_list.push_str("\n ### Assets at Risk\n");
    pattern_list.push_str(&pattern.assets_at_risk.join(", "));

    pattern_list.push_str(&format!(
        "\n ### Minimum Privilege Required to Exploit Vulnerability: {}\n",
        &pattern.privilege.to_string()
    ));

    pattern_list.push_str(&format!(
        "\n ### Impact: {}\n",
        &pattern.impact.unwrap_or(ImpactHint::Low).to_string()
    ));

    pattern_list
}

#[derive(PartialEq, Eq)]
pub enum FindingReportType {
    Standard,
    Enhanced,
    NoPoC,
}

pub fn get_finding_report(
    finding: &Finding,
    index: Option<usize>,
    report_type: FindingReportType,
) -> String {
    let mut findings_report = String::new();

    if let Some(inx) = index {
        // title with index
        findings_report.push_str(&format!(
            "## [{}-{}]. {}\n\n",
            finding.severity.as_initial(),
            inx + 1,
            finding.title
        ));
    } else {
        //title without index
        findings_report.push_str(&format!(
            "## [Severity-{}]. {}\n\n",
            finding.severity.to_string(),
            finding.title
        ));
    }
    //derived from
    findings_report.push_str("## Derived From Pattern/Invariant\n");
    findings_report.push_str(&finding.derived_from.clone().unwrap_or_default());
    findings_report.push_str("\n\n");
    //type
    findings_report.push_str("## Exploit Type\n");
    findings_report.push_str(&finding.exploit_type.to_string());
    findings_report.push_str("\n\n");

    //location
    findings_report.push_str("## Location\n");
    findings_report.push_str(&format!("{}.{}", finding.contract, finding.function));
    findings_report.push_str("\n\n");

    // for final report include status, confidence, and complexity
    if report_type == FindingReportType::Enhanced {
        findings_report.push_str(&format!(
            "## Finding Status: {}\n",
            finding.status.unwrap_or_default().to_string()
        ));
        if finding.status == Some(FindingStatus::NeedsMoreInfo) {
            findings_report.push_str(&format!(
                "### Finding Status Justification: {}\n",
                finding.status_justification.clone().unwrap_or_default()
            ));
        }
        findings_report.push_str(&format!(
            "## Status Confidence: {}\n",
            finding.status_confidence.unwrap_or_default().to_string()
        ));
        if finding.status_confidence == Some(FindingConfidence::SomeWhatConfident) {
            findings_report.push_str(&format!(
                "### Finding Confidence Justification: {}\n",
                finding
                    .status_confidence_justification
                    .clone()
                    .unwrap_or_default()
            ));
        }
        findings_report.push_str(&format!(
            "### Finding Complexity: {}\n",
            finding.finding_complexity.unwrap_or_default()
        ));
        findings_report.push_str(&format!(
            "### PoC Test Status: {}\n",
            finding.poc_test_status.unwrap_or_default().to_string()
        ));
    }

    //privilege
    findings_report.push_str(&format!(
        "## Minimim Privilege Required:{}\n",
        finding.privilege.to_string()
    ));
    findings_report.push_str("\n\n");

    //description
    findings_report.push_str("## Description\n");
    findings_report.push_str(&finding.description.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //impact
    findings_report.push_str("## Impact\n");
    findings_report.push_str(&finding.impact.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    // for final report include status, confidence, and complexity
    if report_type == FindingReportType::Enhanced {
        findings_report.push_str("## Command to Run Test\n");
        findings_report.push_str(&finding.poc_test_command.clone().unwrap_or_default());
        findings_report.push_str("\n\n");
    }
    //POC
    findings_report.push_str("## Proof of Concept\n");
    findings_report.push_str(&finding.proof_of_concept.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    if report_type != FindingReportType::NoPoC {
        //Proof of Code
        findings_report.push_str("## Proof of Code\n");
        findings_report.push_str(&finding.proof_of_code.clone().unwrap_or_default());
        findings_report.push_str("\n\n");
    }

    //Suggested Fix
    findings_report.push_str("## Suggested Mitigation\n");
    findings_report.push_str(&finding.mitigation.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    findings_report.push_str("\n");
    findings_report
}
