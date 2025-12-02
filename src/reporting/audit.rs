use std::collections::HashMap;
use std::sync::OnceLock;

use crate::{
    config::CREATE_TESTS,
    llm_review::{
        findings::{
            finding_enums::Severity,
            findings::{Finding, Findings},
        },
        phases::verify_findings::{FindingConfidence, FindingStatus},
        utils::prompt_context::{self, FindingReportType},
    },
    prepare_code::git_clone::RepoPaths,
};
/// Professional audit report generation with findings categorization.
///
/// This module generates comprehensive security audit reports in Markdown format,
/// supporting both paid (full details) and free (limited) report versions with
/// severity-based finding organization and protocol overviews.

/// Severity levels for organizing findings in reports
const SEVERITIES: [Severity; 5] = [
    Severity::Critical,
    Severity::High,
    Severity::Medium,
    Severity::Low,
    Severity::Info,
];

#[derive(Debug)]
pub enum ReportDataType {
    Summary,
    Full,
}

#[derive(Debug)]
pub enum ReportType {
    Severity,
    Pattern,
}
/// Generates a comprehensive audit report with findings and protocol analysis.
///
/// Creates a professional Markdown audit report including protocol overview,
/// finding summaries, and detailed vulnerability descriptions organized by severity.
///
/// # Arguments
/// * `issues` - Security findings organized by contract
/// * `invariants` - Protocol invariant analysis results
/// * `repo` - Repository paths and metadata
/// * `semantics_path` - Path to semantic analysis database
/// * `report_type` - Report detail level (Free/Paid)
///
/// # Returns
/// * `String` - Complete audit report in Markdown format
pub async fn generated_audit_report(
    findings: &Findings,
    repo: &RepoPaths,
    report_type: ReportType,
) -> anyhow::Result<String> {
    let mut audit_report = String::new();
    let protocol_name = repo.repo_name.replace("-", " ");
    let commit = repo.commit_hash.clone();

    let report_title = format!("# {} - Findings Report\n", protocol_name);
    let subtitle = format!("## Commit hash: {}\n\n", commit);

    // generated report
    audit_report.push_str(&report_title);
    audit_report.push_str(&subtitle);
    // audit_report.push_str("## Protocol Overview \n\n");

    // log::info!("generate summary of protocol");
    // let protocol_overview = summarize::summarize_protocol(repo, None).await?;
    //
    // audit_report.push_str(&protocol_overview);

    let summary = match report_type {
        ReportType::Severity => get_finding_summary(&findings),
        ReportType::Pattern => get_finding_summary_by_pattern(&findings, ReportDataType::Summary),
    };

    audit_report.push_str(&summary);

    let finding_count = get_list_of_issues_by_severity(&findings);

    audit_report.push_str(&finding_count);

    let findings_report = match report_type {
        ReportType::Severity => get_full_finding_report(&findings),
        ReportType::Pattern => get_finding_summary_by_pattern(&findings, ReportDataType::Full),
    };

    audit_report.push_str(&findings_report);

    Ok(audit_report)
}

fn get_full_finding_report(findings: &Findings) -> String {
    let mut findings_report = String::new();

    findings_report.push_str("\n");
    for severity in SEVERITIES {
        let report_by_severity = get_finding_report_by_severity(findings, severity);
        findings_report.push_str(&report_by_severity);
    }
    findings_report.push_str("\n");

    findings_report
}

fn get_finding_report_by_severity(findings: &Findings, severity: Severity) -> String {
    let findings_by_severity = findings.filter_by_severity(severity);
    let mut findings_report = String::new();

    if !findings_by_severity.is_empty() {
        findings_report.push_str(&format!("\n# {} Risk Findings\n\n", severity.to_string()));

        for (i, finding) in findings_by_severity.iter().enumerate() {
            findings_report.push_str(&prompt_context::get_finding_report(
                finding,
                Some(i),
                FindingReportType::Enhanced,
            ));
        }
    } else {
        return String::new();
    }
    findings_report.push_str("\n");
    findings_report
}

fn get_finding_summary(findings: &Findings) -> String {
    let mut findings_summary = String::new();

    findings_summary.push_str("\n");
    for severity in SEVERITIES {
        let summary_for_severity = get_finding_summary_by_severity(findings, severity);
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

fn get_finding_summary_by_severity(findings: &Findings, severity: Severity) -> String {
    let findings_by_severity = findings.filter_by_severity(severity);
    let mut findings_summary = String::new();

    if !findings_by_severity.is_empty() {
        findings_summary.push_str(&format!("## {} Risk Findings\n\n", severity.to_string()));

        for (i, finding) in findings_by_severity.iter().enumerate() {
            findings_summary.push_str(&format!(
                "[{}-{}]. {}\n\n **Derived From** : {}\n\n",
                severity.as_initial(),
                i + 1,
                finding.title,
                &finding.derived_from.clone().unwrap_or_default()
            ));
            findings_summary.push_str(&format!(
                "Finding Status: {}\n",
                finding.status.unwrap_or_default().to_string()
            ));
            if finding.status == Some(FindingStatus::NeedsMoreInfo) {
                findings_summary.push_str(&format!(
                    "Finding Status Justification: {}\n",
                    finding.status_justification.clone().unwrap_or_default()
                ));
            }
            findings_summary.push_str(&format!(
                "Status Confidence: {}\n",
                finding.status_confidence.unwrap_or_default().to_string()
            ));
            if finding.status_confidence == Some(FindingConfidence::SomeWhatConfident) {
                findings_summary.push_str(&format!(
                    "Finding Confidence Justification: {}\n",
                    finding
                        .status_confidence_justification
                        .clone()
                        .unwrap_or_default()
                ));
            }
            findings_summary.push_str(&format!(
                "Finding Complexity: {}\n",
                finding.finding_complexity.unwrap_or_default()
            ));
            findings_summary.push_str(&format!("Privilege: {}\n", finding.privilege.to_string()));

            if CREATE_TESTS {
                findings_summary.push_str(&format!(
                    "Poc Test Status: {}\n\n",
                    finding.poc_test_status.unwrap_or_default().to_string()
                ));
            }
        }
    } else {
        return String::new();
    }
    findings_summary
}

// Cache grouped findings by pattern once to ensure stable ordering across multiple calls
static FINDINGS_BY_PATTERN_CACHE: OnceLock<Vec<(String, Vec<Finding>)>> = OnceLock::new();

fn get_finding_summary_by_pattern(findings: &Findings, report_type: ReportDataType) -> String {
    let mut findings_summary = String::new();

    // Initialize the cache once with insertion-order grouping based on the incoming findings
    // ************************************************************************************
    let grouped_findings = FINDINGS_BY_PATTERN_CACHE.get_or_init(|| {
        let mut map: HashMap<String, Vec<Finding>> = HashMap::new();
        let mut order: Vec<String> = Vec::new();

        for f in findings.findings.iter() {
            let key = f
                .derived_from
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());
            if !map.contains_key(&key) {
                order.push(key.clone());
            }
            map.entry(key).or_insert_with(Vec::new).push(f.clone());
        }

        // Rehydrate into a Vec following first-seen key order to keep output stable
        let mut grouped: Vec<(String, Vec<Finding>)> = Vec::new();
        for k in order {
            if let Some(v) = map.remove(&k) {
                grouped.push((k, v));
            }
        }
        grouped
    });
    // ************************************************************************************

    if grouped_findings.is_empty() {
        return String::new();
    }

    findings_summary.push_str("##Findings by Pattern\n");

    let mut num = 1;
    for (pattern, findings_vec) in grouped_findings.iter() {
        findings_summary.push_str(&format!("\n\n **Derived From** : {}\n\n", pattern));
        for f in findings_vec {
            match report_type {
                ReportDataType::Summary => {
                    findings_summary.push_str(&format!(
                        "[{}-{}]. {}\n",
                        f.severity.as_initial(),
                        num,
                        f.title
                    ));

                    findings_summary.push_str(&format!(
                        "Finding Status: {}\n",
                        f.status.unwrap_or_default().to_string()
                    ));
                    if f.status == Some(FindingStatus::NeedsMoreInfo) {
                        findings_summary.push_str(&format!(
                            "Finding Status Justification: {}\n",
                            f.status_justification.clone().unwrap_or_default()
                        ));
                    }
                    findings_summary.push_str(&format!(
                        "Status Confidence: {}\n",
                        f.status_confidence.unwrap_or_default().to_string()
                    ));
                    if f.status_confidence == Some(FindingConfidence::SomeWhatConfident) {
                        findings_summary.push_str(&format!(
                            "Finding Confidence Justification: {}\n",
                            f.status_confidence_justification
                                .clone()
                                .unwrap_or_default()
                        ));
                    }
                    findings_summary.push_str(&format!(
                        "Finding Complexity: {}\n",
                        f.finding_complexity.unwrap_or_default()
                    ));
                    findings_summary.push_str(&format!("Privilege: {}\n", f.privilege.to_string()));

                    if CREATE_TESTS {
                        findings_summary.push_str(&format!(
                            "Poc Test Status: {}\n\n",
                            f.poc_test_status.unwrap_or_default().to_string()
                        ));
                    }
                }
                ReportDataType::Full => {
                    findings_summary.push_str(&prompt_context::get_finding_report(
                        f,
                        Some(num - 1),
                        FindingReportType::Enhanced,
                    ));
                }
            }
            num += 1;
        }
        findings_summary.push_str("\n");
    }

    findings_summary
}
