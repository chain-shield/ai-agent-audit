use std::{fs::File, io::Write};

use crate::build_brain::git_clone::RepoPaths;

pub fn save_audit_report(markdown: &str, repo: &RepoPaths) -> anyhow::Result<()> {
    let filename = format!(
        "{}-{}-audit-report.md",
        &repo.repo_name,
        &repo.commit_hash[..6]
    );
    save_file_locally(markdown, &filename)?;

    Ok(())
}

pub fn save_file_locally(content: &str, filename: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
