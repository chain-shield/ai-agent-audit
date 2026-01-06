use crate::llm_review::{
    dynamic_prompts::prompt_index::{self, generated_sub_header},
    threat_models::patterns::{
        VulnerabilityPattern, VulnerabilityPatternSpec, VULNERABILITY_PATTERN_LIBRARY,
    },
};
use rand::seq::SliceRandom;

pub fn generate_formated_list_from_pattern_data(
    patterns_to_use: &[VulnerabilityPattern],
    section_num: u8,
) -> (String, String) {
    let mut top_patterns_spec: Vec<VulnerabilityPatternSpec> = VULNERABILITY_PATTERN_LIBRARY
        .iter()
        .filter(|v| patterns_to_use.contains(&v.key))
        .map(|v| v.to_owned())
        .collect();

    // Randomize the order of patterns
    let mut rng = rand::rng();
    top_patterns_spec.shuffle(&mut rng);

    // list of patterns
    let mut top_patterns_list = String::new();

    //pattern index
    let mut pattern_index = prompt_index::generated_table_of_context_header(
        "Vulnerability Pattern Catalog",
        section_num,
    );

    for (section, pattern) in top_patterns_spec.iter().enumerate() {
        // update table of contents with new entry
        pattern_index.push_str(&format!(
            "-{}.{} {}\n",
            section_num,
            section + 1,
            pattern.key.to_pretty_str()
        ));

        //add new pattern to list
        top_patterns_list.push_str(&generated_sub_header(
            &pattern.key.to_pretty_str(),
            section_num,
            section + 1,
        ));
        top_patterns_list.push_str("\n\n");
        top_patterns_list.push_str("### Vulnerability Pattern\n");
        top_patterns_list.push_str(&pattern.key.to_string());
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Definition\n");
        top_patterns_list.push_str(pattern.definition);
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Static Signals\n");
        top_patterns_list.push_str(&pattern.static_signals.join("\n"));
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Examples\n");
        top_patterns_list.push_str(&pattern.examples.join("\n"));
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Impact Hint\n");
        top_patterns_list.push_str(&pattern.impact_hint.to_string());
        top_patterns_list.push_str("\n\n");
    }

    (top_patterns_list, pattern_index)
}
