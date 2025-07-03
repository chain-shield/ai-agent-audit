use std::{fs::File, io::Write};

use crate::build_brain::git_clone::RepoPaths;

use super::audit::ReportType;

pub fn save_audit_report(
    markdown: &str,
    repo: &RepoPaths,
    report_type: ReportType,
) -> anyhow::Result<()> {
    let suffix = if report_type == ReportType::Free {
        "-free"
    } else {
        ""
    };
    let filename = format!("{}{}-audit-report.md", repo.unique_repo_hash(), suffix);
    save_file_locally(markdown, &filename)?;

    Ok(())
}

pub fn save_file_locally(content: &str, filename: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
