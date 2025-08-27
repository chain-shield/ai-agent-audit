use crate::llm_review::{
    dynamic_prompts::findings_template::generate_findings_prompt,
    enums::EnumData,
    patterns::{Pattern, VulnerabilityPatternSpec},
    utils::prompt_context::generate_formatted_pattern,
};

pub fn generate_pattern_to_findings_prompt(pattern: &Pattern) -> String {
    let pattern_data: VulnerabilityPatternSpec = pattern.issue_type.get_spec();
    let pattern_full_spec = generate_formatted_pattern(pattern);
    let pattern_type = "Security Vulnerability Pattern";

    generate_findings_prompt(
        pattern_type,
        &pattern_data.definition,
        &pattern_full_spec,
        &pattern.issue_type,
        &pattern.title,
    )
}
