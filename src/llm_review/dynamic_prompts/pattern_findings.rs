use std::collections::HashSet;

use crate::{
    llm_review::{
        dynamic_prompts::findings_template::generate_findings_prompt_for_multiple_patterns,
        threat_models::patterns::{Patterns, VulnerabilityPattern},
        utils::prompt_context::generate_formatted_multiple_patterns,
    },
    prepare_code::git_clone::RepoPaths,
};

pub fn generate_multipattern_to_findings_prompt(patterns: &Patterns, repo: &RepoPaths) -> String {
    let vulnerability_patterns: Vec<VulnerabilityPattern> = patterns
        .patterns
        .iter()
        .map(|p| p.issue_type)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let pattern_full_spec = generate_formatted_multiple_patterns(&patterns.patterns);
    let pattern_type = "Security Vulnerability Patterns";

    generate_findings_prompt_for_multiple_patterns(
        pattern_type,
        &pattern_full_spec,
        &vulnerability_patterns,
        repo,
    )
}
