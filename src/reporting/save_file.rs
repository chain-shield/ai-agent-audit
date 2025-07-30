use super::audit::ReportType;
use crate::prepare_code::git_clone::RepoPaths;
/// File saving utilities for audit reports and analysis data.
///
/// This module provides functions to save audit reports and other analysis
/// outputs to the local filesystem with appropriate naming conventions.
use std::{fs::File, io::Write, path::Path};

/// Saves an audit report to a file with appropriate naming.
///
/// Creates a markdown file with the audit report content using a standardized
/// naming convention that includes the repository hash and report type.
///
/// # Arguments
/// * `markdown` - The audit report content in Markdown format
/// * `repo` - Repository paths and metadata for naming
/// * `report_type` - Report type (Free/Paid) for filename suffix
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

    let output_dir = Path::new(&repo.repo_name);
    let filename = format!("{}{}-audit-report.md", repo.unique_repo_hash(), suffix);
    let full_path = output_dir.join(filename);
    save_file_locally(markdown, &full_path)?;

    Ok(())
}

/// Saves content to a local file.
///
/// Generic file saving utility that writes string content to a specified filename.
///
/// # Arguments
/// * `content` - String content to write to file
/// * `filename` - Target filename for the content
pub fn save_file_locally(content: &str, filename: &Path) -> anyhow::Result<()> {
    if let Some(parent) = filename.parent() {
        std::fs::create_dir_all(parent)?; // ✅ Create folder if missing
    }

    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
