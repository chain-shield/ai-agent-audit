use std::collections::HashSet;

use crate::{
    llm_review::{
        dynamic_prompts::findings_template::{
            generate_findings_prompt, generate_findings_prompt_for_multiple_patterns,
        },
        enums::EnumData,
        patterns::{Pattern, Patterns, VulnerabilityPattern, VulnerabilityPatternSpec},
        utils::prompt_context::{generate_formatted_multiple_patterns, generate_formatted_pattern},
    },
    prepare_code::git_clone::RepoPaths,
};

pub fn generate_pattern_to_findings_prompt(pattern: &Pattern, repo: &RepoPaths) -> String {
    let pattern_data: VulnerabilityPatternSpec = pattern.issue_type.get_spec();
    let pattern_full_spec = generate_formatted_pattern(pattern);
    let pattern_type = "Security Vulnerability Pattern";

    generate_findings_prompt(
        pattern_type,
        &pattern_data.definition,
        &pattern_full_spec,
        &pattern.issue_type,
        &repo,
    )
}

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
