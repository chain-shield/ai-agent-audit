use crate::{
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::context_state::{generate_audit_scope, get_metadata_context},
        dynamic_prompts::{self, prompt_index},
        threat_models::actors::Actors,
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;
use std::sync::Arc;

pub async fn execute(code: &str, arc_agent: &Arc<AIAgent>, repo: &RepoPaths) -> Result<Actors> {
    info!("🔍 Phase 0: Generating Actors from contract codebase...");

    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");

    let audit_scope = generate_audit_scope(repo).await?;
    let section_8_header = prompt_index::generated_section_header("SOLIDITY CODE TO REVIEW", 8);
    let section_9_header = prompt_index::generated_section_header("ADDITIONAL CONTEXT", 9);
    let section_10_header = prompt_index::generated_section_header(
        "AUDIT SCOPE AND KEY INVARIANTS PROVIDED BY CLIENT",
        10,
    );

    let combined_context = if audit_scope.is_empty() {
        format!(
            r#"

{section_9_header}

{context}
"#
        )
    } else {
        format!(
            r#"

{section_9_header}

{context}

{section_10_header}

{audit_scope}
"#
        )
    };

    // Build code + context block
    let code_plus_context = format!(
        r#"

{section_8_header}

{code}

{combined_context}
"#
    );

    // Construct prompt
    let instruction_prompt = dynamic_prompts::actors::generate_actors_prompt();
    let json_requirement_prompt = dynamic_prompts::actors::get_actor_list_json();
    let prompt = format!("{instruction_prompt}{code_plus_context}{json_requirement_prompt}");

    // Single LLM call - no need for threads since we only run once per contract
    info!("---- LLM analysis for Enumerating Actors ----");
    let actors: Actors = arc_agent.extract_with_retry(&prompt).await?;

    let actors_found = actors.actors.len();
    if actors_found > 0 {
        info!(
            "✅ Finding Actors Phase complete: {} actors found",
            actors_found
        );
    } else {
        info!("✅ Finding Actors Phase complete: No actors found");
    }

    Ok(actors)
}
