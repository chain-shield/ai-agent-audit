use std::{collections::HashSet, path::Path};

use crate::{
    llm_review::{
        threat_models::patterns::Pattern,
        utils::prompt_context::generate_formatted_multiple_patterns,
    },
    prepare_code::git_clone::RepoPaths,
    reporting::save_file::save_file_locally,
};

pub async fn save_patterns(patterns: &[Pattern], repo: &RepoPaths) -> anyhow::Result<()> {
    if patterns.is_empty() {
        return Ok(());
    }
    // grab all solidity contracts from database
    log::info!("saving patterns to file");
    let dir = format!("{}/patterns", repo.repo_name);
    let output_dir = Path::new(&dir);

    let pattern_report = generate_pattern_full_report(patterns);

    let contract = patterns
        .first()
        .cloned()
        .unwrap_or(Pattern::default())
        .contract;

    let filename = format!("{}-patterns.md", contract);
    let full_path = output_dir.join(filename);
    save_file_locally(&pattern_report, &full_path)?;
    Ok(())
}

fn generate_pattern_full_report(patterns: &[Pattern]) -> String {
    if patterns.is_empty() {
        return String::new();
    }

    let mut report = String::new();

    let pattern_count = patterns.len();
    let unique_issue_types: String = patterns
        .iter()
        .map(|p| p.issue_type)
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|i| format!("- {i}\n"))
        .collect();

    report.push_str(&format!(
        "## Verified Patterns Found: {}\n\n",
        pattern_count
    ));
    report.push_str("## Verified Patterns Found in following Categories:\n\n");
    report.push_str(&format!("{}\n\n", unique_issue_types));

    let pattern_summary: String = patterns
        .iter()
        .map(|p| format!("\n{}\n", p.title))
        .collect();

    report.push_str("\n## Summary of Patterns\n");
    report.push_str(&pattern_summary);

    report.push_str("\n## Patterns\n");

    let pattern_content = generate_formatted_multiple_patterns(patterns);
    report.push_str(&pattern_content);

    report
}
