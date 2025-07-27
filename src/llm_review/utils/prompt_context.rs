use crate::llm_review::config::Finding;

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

    let report = get_finding_report(finding);
    prompt.push_str(&report);
    prompt.push_str("\n\n");

    prompt.push_str("## CODEBASE WHERE ISSUE WAS FOUND");
    prompt.push_str("\n\n");

    prompt.push_str(code);

    prompt
}

fn get_finding_report(finding: &Finding) -> String {
    let mut findings_report = String::new();
    //title
    findings_report.push_str(&format!(
        "## [Severity-{}]. {}\n\n",
        finding.severity.as_str(),
        finding.title()
    ));

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
