use std::{collections::HashMap, path::Path};

use crate::{
    build_brain::summarize,
    llm_review::{
        config::{ContractInvariants, Findings},
        enums::Severity,
    },
    prepare_code::git_clone::RepoPaths,
};

const SEVERITIES: [Severity; 4] = [
    Severity::High,
    Severity::Medium,
    Severity::Low,
    Severity::Info,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportType {
    Free,
    Paid,
}

pub async fn generated_audit_report(
    issues: &HashMap<String, Findings>,
    invariants: &[ContractInvariants],
    repo: &RepoPaths,
    semantics_path: &Path,
    report_type: ReportType,
) -> anyhow::Result<String> {
    let mut audit_report = String::new();
    let protocol_name = repo.repo_name.replace("-", " ");
    let commit = repo.commit_hash.clone();

    let report_title = format!("# {} - Findings Report\n", protocol_name);
    let subtitle = format!("## Commit hash: {}\n\n", commit);

    let findings = combine_findings_for_all_contracts(issues.clone());

    // generated report
    audit_report.push_str(&report_title);
    audit_report.push_str(&subtitle);
    audit_report.push_str("## Protocol Overview \n\n");

    log::info!("generate summary of protocol");
    let protocol_overview = summarize::summarize_protocol(repo, semantics_path).await?;

    audit_report.push_str(&protocol_overview);

    let summary = get_finding_summary(&findings, report_type);

    audit_report.push_str(&summary);

    let finding_count = get_list_of_issues_by_severity(&findings);

    audit_report.push_str(&finding_count);

    let findings_report = get_finding_report(&findings, report_type);

    audit_report.push_str(&findings_report);

    // let invariants_report = get_invariant_report(invariants);
    //
    // audit_report.push_str(&invariants_report);

    Ok(audit_report)
}

// finding hashmap => vec
fn combine_findings_for_all_contracts(findings_hash: HashMap<String, Findings>) -> Findings {
    // all security issue findings
    let mut findings = Findings {
        findings: Vec::new(),
    };

    // conbine finding for each contract
    for f in findings_hash.into_values() {
        findings.findings.extend(f.findings);
    }

    findings
}

// finding hashmap => vec
fn combine_invariants_for_all_contracts(
    invariants_vec: &[ContractInvariants],
) -> ContractInvariants {
    // all security issue findings
    let mut all_invariants = ContractInvariants {
        invariants: Vec::new(),
        ..Default::default()
    };

    // conbine finding for each contract
    for inv in invariants_vec {
        all_invariants.invariants.extend(inv.invariants.clone());
    }

    all_invariants
}

// TODO - for free report exclude H and M
fn get_finding_report(findings: &Findings, report_type: ReportType) -> String {
    let mut findings_report = String::new();

    findings_report.push_str("\n");
    for severity in SEVERITIES {
        let report_by_severity = get_finding_report_by_severity(findings, severity);
        if report_type == ReportType::Paid
            || (severity != Severity::High && severity != Severity::Medium)
        {
            findings_report.push_str(&report_by_severity);
        }
    }
    findings_report.push_str("\n");

    findings_report
}

fn get_finding_report_by_severity(findings: &Findings, severity: Severity) -> String {
    let findings_by_severity = findings.filter_by_severity(severity);
    let mut findings_report = String::new();

    if !findings_by_severity.is_empty() {
        findings_report.push_str(&format!("\n# {} Risk Findings\n\n", severity.as_str()));

        for (i, finding) in findings_by_severity.iter().enumerate() {
            //title
            findings_report.push_str(&format!(
                "## [{}-{}]. {}\n\n",
                severity.as_initial(),
                i + 1,
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
        }
    } else {
        return String::new();
    }
    findings_report.push_str("\n");
    findings_report
}

fn get_invariant_report(invariants: &[ContractInvariants]) -> String {
    let mut findings_report = String::new();

    let contract_invariants = combine_invariants_for_all_contracts(invariants);

    let violations = contract_invariants.get_all_violations();

    if !violations.is_empty() {
        findings_report.push_str(&format!(
            "\n# {} Invariant Violations\n\n",
            violations.len()
        ));

        for (i, violation) in violations.into_iter().enumerate() {
            //title
            findings_report.push_str(&format!(
                "## {}. {} Violation\n\n",
                i + 1,
                violation.inv_type.as_str(),
            ));

            //description
            findings_report.push_str("## Description\n");
            findings_report.push_str(&violation.desc);
            findings_report.push_str("\n\n");

            //impact
            findings_report.push_str("## Impact\n");
            findings_report.push_str(&violation.impact.unwrap_or_default());
            findings_report.push_str("\n\n");

            //POC
            findings_report.push_str("## Proof of Concept\n");
            findings_report.push_str(&violation.poc.unwrap_or_default());
            findings_report.push_str("\n\n");

            //Proof of Code
            findings_report.push_str("## Pre-State\n");
            findings_report.push_str(&violation.pre_state.unwrap_or_default());
            findings_report.push_str("\n\n");

            //Proof of Code
            findings_report.push_str("## Post-State\n");
            findings_report.push_str(&violation.post_state.unwrap_or_default());
            findings_report.push_str("\n\n");

            //Suggested Fix
            findings_report.push_str("## Suggested Mitigation\n");
            findings_report.push_str(&violation.mitigation.unwrap_or_default());
            findings_report.push_str("\n\n");
        }
    } else {
        return String::new();
    }
    findings_report.push_str("\n");
    findings_report
}

fn get_finding_summary(findings: &Findings, report_type: ReportType) -> String {
    let mut findings_summary = String::new();

    findings_summary.push_str("\n");
    for severity in SEVERITIES {
        let summary_for_severity = get_finding_summary_by_severity(findings, severity, report_type);
        findings_summary.push_str(&summary_for_severity);
    }
    findings_summary.push_str("\n");

    findings_summary
}

fn get_list_of_issues_by_severity(findings: &Findings) -> String {
    let mut list_severity_count = String::new();
    let severity_count_map = findings.count_by_severity();

    list_severity_count.push_str("\n### Number of Findings\n");
    for s in SEVERITIES {
        let count = severity_count_map.get(&s).copied().unwrap_or(0);
        list_severity_count.push_str(&format!("- {}: {}\n", s.as_initial(), count));
    }
    list_severity_count.push_str("\n");
    list_severity_count
}

fn get_finding_summary_by_severity(
    findings: &Findings,
    severity: Severity,
    report_type: ReportType,
) -> String {
    let findings_by_severity = findings.filter_by_severity(severity);
    let mut findings_summary = String::new();

    if !findings_by_severity.is_empty() {
        findings_summary.push_str(&format!("## {} Risk Findings\n", severity.as_str()));

        for (i, finding) in findings_by_severity.iter().enumerate() {
            let title = if report_type == ReportType::Paid {
                finding.title()
            } else {
                finding.free_report_title()
            };
            findings_summary.push_str(&format!(
                "[{}-{}]. {}\n",
                severity.as_initial(),
                i + 1,
                title
            ));
        }
    } else {
        return String::new();
    }
    findings_summary
}
