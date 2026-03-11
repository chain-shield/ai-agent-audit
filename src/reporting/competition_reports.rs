use log::info;

use crate::{
    llm_review::{
        findings::findings::{Finding, Findings},
        phases::create_report::CompetitionReport,
    },
    prepare_code::git_clone::RepoPaths,
    reporting::save_file::save_audit_report,
    utils::file_security::sanitize_filename,
};

pub fn generate_and_save_pro_reports(findings: &Findings, repo: &RepoPaths) -> anyhow::Result<()> {
    // grab all solidity contracts from database
    info!("saving professional finding to file");

    for finding in &findings.findings {
        if let Some(report) = &finding.competition_report
            && !report.report.is_empty()
        {
            let final_report = generate_final_pro_report(report, finding);
            let raw_filename = sanitize_filename(&finding.title);
            let mut filename: String = raw_filename.chars().take(40).collect();
            filename = format!("{}.md", filename);
            save_audit_report(&filename, &final_report, repo)?;
        }
    }
    Ok(())
}

fn generate_final_pro_report(report: &CompetitionReport, finding: &Finding) -> String {
    let mut final_report = String::new();
    final_report.push_str("## Relevant Code Snippets\n");

    for snippet in &report.github_urls {
        final_report.push_str(&format!("{}\n", snippet));
    }

    final_report.push_str("\n\n");
    final_report.push_str(&report.report);
    final_report.push_str("\n\n");
    final_report.push_str("## Proof of Code\n");
    final_report.push_str("\n\n");
    final_report.push_str(&finding.proof_of_code.clone().unwrap_or_default());

    final_report
}
