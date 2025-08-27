use crate::llm_review::{
    enums::EnumString,
    findings::Finding,
    invariants::InvariantFinding,
    patterns::{ImpactHint, Pattern},
};

pub fn generate_prompt_for_issue_check(
    code: &str,
    finding: &Finding,
    pre_instructions: &str,
    instructions: &str,
    post_instructions: &str,
) -> String {
    let mut prompt = format!("{}{}{}", pre_instructions, instructions, post_instructions);

    prompt.push_str("\n\n");
    prompt.push_str("## REPORT FOR SECURITY ISSUE");
    prompt.push_str("\n\n");

    let report = get_finding_report(finding, None);
    prompt.push_str(&report);
    prompt.push_str("\n\n");

    prompt.push_str("## CODEBASE WHERE ISSUE WAS FOUND");
    prompt.push_str("\n\n");

    prompt.push_str(code);

    prompt
}

pub fn generate_formatted_invariant_finding(invariant: &InvariantFinding) -> String {
    let mut invariant_finding = String::new();

    invariant_finding.push_str(&format!(
        "\n\n ### Invariant Type: {}\n",
        &invariant.inv_type.as_str()
    ));

    invariant_finding.push_str(&format!(
        "\n ### Relevant Function/Location: {}.{}\n",
        invariant.contract, invariant.function
    ));

    invariant_finding.push_str("\n ### Description/Code Snippet\n");
    invariant_finding.push_str(&invariant.desc.as_str());

    invariant_finding.push_str("\n ### Checks\n");
    invariant_finding.push_str(&invariant.checks.join(", "));

    invariant_finding.push_str(&format!("\n ### Status: {}\n", &invariant.status.as_str()));

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
        &pattern.issue_type.as_str()
    ));

    pattern_list.push_str(&format!(
        "\n ### Relevant Function/Location: {}.{}\n",
        pattern.contract, pattern.function
    ));

    pattern_list.push_str("\n ### Title\n");
    pattern_list.push_str(&pattern.description.as_str());

    pattern_list.push_str("\n ### Description/Code Snippet\n");
    pattern_list.push_str(&pattern.description.as_str());

    pattern_list.push_str("\n ### Static Signals\n");
    pattern_list.push_str(&pattern.static_signals.join(", "));

    pattern_list.push_str("\n ### Assets at Risk\n");
    pattern_list.push_str(&pattern.assets_at_risk.join(", "));

    pattern_list.push_str(&format!(
        "\n ### Minimum Privilege Required to Exploit Vulnerability: {}\n",
        &pattern.privilege.as_str()
    ));

    pattern_list.push_str(&format!(
        "\n ### Impact: {}\n",
        &pattern.impact.unwrap_or(ImpactHint::Low).as_str()
    ));

    pattern_list
}

pub fn get_finding_report(finding: &Finding, index: Option<usize>) -> String {
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
            finding.severity.as_str(),
            finding.title
        ));
    }
    //derived from
    findings_report.push_str("## Derived From Pattern/Invariant\n");
    findings_report.push_str(&finding.derived_from.clone().unwrap_or_default());
    findings_report.push_str("\n\n");
    //type
    findings_report.push_str("## Exploit Type\n");
    findings_report.push_str(&finding.exploit_type.as_str());
    findings_report.push_str("\n\n");

    //location
    findings_report.push_str("## Location\n");
    findings_report.push_str(&format!("{}.{}", finding.contract, finding.function));
    findings_report.push_str("\n\n");

    //privilege
    findings_report.push_str("## Minimim Privilege Required\n");
    findings_report.push_str(&finding.privilege.as_str());
    findings_report.push_str("\n\n");

    //description
    findings_report.push_str("## Description\n");
    findings_report.push_str(&finding.description.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //impact
    findings_report.push_str("## Impact\n");
    findings_report.push_str(&finding.impact.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //POC
    findings_report.push_str("## Proof of Concept\n");
    findings_report.push_str(&finding.proof_of_concept.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Proof of Code
    findings_report.push_str("## Proof of Code\n");
    findings_report.push_str(&finding.proof_of_code.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Suggested Fix
    findings_report.push_str("## Suggested Mitigation\n");
    findings_report.push_str(&finding.mitigation.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    findings_report.push_str("\n");
    findings_report
}
