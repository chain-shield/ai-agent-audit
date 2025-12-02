use std::collections::HashSet;

use crate::{
    llm_review::{
        agent::agent_enums::EnumData,
        dynamic_prompts::findings_template::{
            generate_findings_prompt, generate_findings_prompt_for_multiple_patterns,
        },
        threat_models::invariants::{
            ContractInvariants, InvariantFinding, InvariantSpec, InvariantType,
        },
        utils::prompt_context::{
            generate_formatted_invariant_finding, generate_formatted_multiple_invariant_findings,
        },
    },
    prepare_code::git_clone::RepoPaths,
};

pub fn generate_invariant_to_findings(invariant: &InvariantFinding, repo: &RepoPaths) -> String {
    let pattern_data: InvariantSpec = invariant.inv_type.get_spec();
    let pattern_full_spec = generate_formatted_invariant_finding(invariant);
    let invariant_title = "Invariant Violation";

    generate_findings_prompt(
        invariant_title,
        &pattern_data.definition,
        &pattern_full_spec,
        &invariant.inv_type,
        repo,
    )
}

pub fn generate_multi_invariant_to_findings_prompt(
    invariants: &ContractInvariants,
    repo: &RepoPaths,
) -> String {
    let vulnerability_patterns: Vec<InvariantType> = invariants
        .invariants
        .iter()
        .map(|p| p.inv_type)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let pattern_full_spec = generate_formatted_multiple_invariant_findings(&invariants.invariants);
    let pattern_type = "Invariants";

    generate_findings_prompt_for_multiple_patterns(
        pattern_type,
        &pattern_full_spec,
        &vulnerability_patterns,
        repo,
    )
}
