use std::collections::HashSet;

use crate::{
    llm_review::{
        dynamic_prompts::{
            actors::{generate_formatted_actor_abuse, generate_formatted_actor_abuse_list},
            findings_template::{
                generate_findings_prompt, generate_findings_prompt_for_multiple_patterns,
            },
        },
        threat_models::{
            actors::{ActorAbuse, ActorAbuses},
            patterns::VulnerabilityPattern,
        },
    },
    prepare_code::git_clone::RepoPaths,
};

pub fn generate_actor_to_findings(actor_abuse: &ActorAbuse, repo: &RepoPaths) -> String {
    let pattern_full_spec = generate_formatted_actor_abuse(actor_abuse);
    let invariant_title = "Malicious Actor";

    generate_findings_prompt(
        invariant_title,
        &actor_abuse.title,
        &pattern_full_spec,
        &actor_abuse.category,
        repo,
    )
}

pub fn generate_multi_actor_to_findings_prompt(
    actor_abuses: &ActorAbuses,
    repo: &RepoPaths,
) -> String {
    let vulnerability_patterns: Vec<VulnerabilityPattern> = actor_abuses
        .abuses
        .iter()
        .map(|p| p.category)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let pattern_full_spec = generate_formatted_actor_abuse_list(&actor_abuses.abuses);
    let pattern_type = "Malicious Actors";

    generate_findings_prompt_for_multiple_patterns(
        pattern_type,
        &pattern_full_spec,
        &vulnerability_patterns,
        repo,
    )
}
