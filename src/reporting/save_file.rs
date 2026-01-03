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
pub fn save_audit_report(filename: &str, markdown: &str, repo: &RepoPaths) -> anyhow::Result<()> {
    let dir = format!("{}/report", repo.repo_name);
    let output_dir = Path::new(&dir);
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
    // log::info!("saving {}\n", filename.display());
    file.write_all(content.as_bytes())?;

    Ok(())
}
