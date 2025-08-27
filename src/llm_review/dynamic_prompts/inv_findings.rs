use crate::llm_review::{
    dynamic_prompts::findings_template::generate_findings_prompt,
    enums::EnumData,
    invariants::{InvariantFinding, InvariantSpec},
    utils::prompt_context::generate_formatted_invariant_finding,
};

pub fn generate_invariant_to_findings(invariant: &InvariantFinding) -> String {
    let pattern_data: InvariantSpec = invariant.inv_type.get_spec();
    let pattern_full_spec = generate_formatted_invariant_finding(invariant);
    let invariant_title = "Invariant Violation";

    generate_findings_prompt(
        invariant_title,
        &pattern_data.definition,
        &pattern_full_spec,
        &invariant.inv_type,
        &invariant.predicate,
    )
}
