use std::collections::HashSet;

use crate::{
    llm_review::{
        dynamic_prompts::{
            findings_template::generate_findings_prompt_for_multiple_patterns, invariants,
        },
        threat_models::invariants::{ContractInvariants, InvariantType},
    },
    prepare_code::git_clone::RepoPaths,
};

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
    let pattern_full_spec = invariants::generate_full_list_of_invariant_findings(&invariants);
    let pattern_type = "Invariants";

    generate_findings_prompt_for_multiple_patterns(
        pattern_type,
        &pattern_full_spec,
        &vulnerability_patterns,
        repo,
    )
}
