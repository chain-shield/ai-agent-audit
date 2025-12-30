use std::sync::Arc;

use crate::{
    config::CREATE_TESTS,
    llm_review::{
        analysis::context_state::MultiModalContext,
        findings::findings::{Finding, Findings},
        threat_models::patterns::Pattern,
    },
    utils::finding_status_string::finding_status_to_string,
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

pub fn generate_prompt_for_multi_finding_issue_check(
    code: &str,
    finding: &Findings,
    instructions: &str,
    post_instructions: &str,
    report_type: FindingReportType,
) -> String {
    let mut prompt = instructions.to_string();

    prompt.push_str("\n\n");
    prompt.push_str("## **SECURITY FINDINGS TO EVALUATE**");
    prompt.push_str("\n\n");

    for finding in &finding.findings {
        let report = get_finding_report(finding, None, report_type);
        prompt.push_str(&report);
        prompt.push_str("\n\n");
    }

    prompt.push_str("## CODEBASE WHERE FINDINGS WERE FOUND");
    prompt.push_str("\n\n");

    prompt.push_str(code);
    prompt.push_str("\n\n");
    prompt.push_str(post_instructions);

    prompt
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
    pattern_list.push_str(&pattern.title.to_string());

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

    // pattern_list.push_str(&format!(
    //     "\n ### Impact: {}\n",
    //     &pattern.impact.unwrap_or(ImpactHint::Low).to_string()
    // ));

    pattern_list
}

pub fn generate_formatted_abbreviated_patterns(patterns: &[Pattern]) -> String {
    let mut pattern_list = String::new();

    for pattern in patterns {
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
    }

    pattern_list
}

pub fn generate_formatted_multiple_patterns(patterns: &[Pattern]) -> String {
    let mut pattern_list = String::new();

    for pattern in patterns {
        let pattern_details = generate_formatted_pattern(pattern);

        pattern_list.push_str("\n");
        pattern_list.push_str(&pattern_details);
        pattern_list.push_str("\n");
    }

    pattern_list
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FindingReportType {
    Standard,
    Enhanced,
    NoPoC,
}

pub fn get_finding_summary_report(finding: &Finding, index: usize) -> String {
    let mut findings_summary = String::new();
    findings_summary.push_str(&format!(
        "\n\n[{}-{}]. {}\n",
        finding.severity.as_initial(),
        index,
        finding.title
    ));
    findings_summary.push_str(&format!(
        "**Derived From** : {}\n",
        finding.derived_from.clone().unwrap_or_default()
    ));
    findings_summary.push_str(&format!(
        "Finding Status: {}\n",
        finding_status_to_string(finding)
    ));
    findings_summary.push_str(&format!("Privilege: {}\n", finding.privilege.to_string()));

    if CREATE_TESTS {
        findings_summary.push_str(&format!(
            "Poc Test Status: {}\n\n",
            finding.poc_test_status.unwrap_or_default().to_string()
        ));
    }

    findings_summary
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

    findings_report.push_str(&format!(
        "## id: {}\n\n",
        &finding.id.clone().unwrap_or_default()
    ));

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
            finding_status_to_string(finding)
        ));
        findings_report.push_str(&format!(
            "### Finding Status Justification: {}\n",
            finding.status_justification.clone().unwrap_or_default()
        ));
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
