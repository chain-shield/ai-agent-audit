use crate::{
    llm_review::{
        agent::{
            agent_enums::AIAgent,
            agent_factory::{AgentConfig, AgentFactory},
        },
        pattern_phases,
        threat_models::{
            actors::Actors,
            invariants::{ContractInvariants, InvariantFinding, InvariantType},
            issues::{IssuePrompt, IssueStructTrait},
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use anyhow::Result;
use nanoid::nanoid;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub async fn generate_actors(codeblock: &str, repo: &RepoPaths) -> Result<Actors> {
    let actor_discovery_agent = generate_openai_agent(repo)?;

    // Phase 1: Generate actors and their capabilities
    log::info!("PRE AUDIT PHASE: GENERATE ACTORS");
    let actors: Actors =
        pattern_phases::generate_actors::execute(codeblock, &actor_discovery_agent, repo).await?;

    let actor_count = actors.actors.len();
    log::info!("total of {} Actors found!", actor_count);

    Ok(actors)
}

pub async fn generate_invariants(codeblock: &str, repo: &RepoPaths) -> Result<ContractInvariants> {
    let invariant_prompt = IssuePrompt::Invariant(InvariantType::iter().collect());

    let invariant_discovery_agent = generate_gemini_agent(repo)?;
    let invariant_verify_agent = generate_openai_agent(repo)?;

    // Phase 1: Generate actors and their capabilities
    log::info!("PRE AUDIT PHASE: DISCOVER INVARIANTS");

    // Phase 1: Generate invariants
    log::info!("PHASE 1: GENERATE INVARIANTS");
    let raw_invariants: ContractInvariants = pattern_phases::generate_patterns::execute(
        invariant_prompt,
        codeblock,
        &invariant_discovery_agent,
        repo,
    )
    .await?;

    log::info!(
        "total of {} invariant found!",
        raw_invariants.invariants.len()
    );

    let invariants_with_id: ContractInvariants = ContractInvariants {
        invariants: raw_invariants
            .invariants
            .into_iter()
            .map(|inv| InvariantFinding {
                id: Some(nanoid!()),
                ..inv
            })
            .collect(),
    };

    // Phase 2: Verify invariants
    log::info!("PHASE 2: VERIFY INVARIANTS");
    let verified_invariants = if !invariants_with_id.issues().is_empty() {
        pattern_phases::verify_patterns::verify_invariants(
            invariants_with_id,
            codeblock,
            &invariant_verify_agent,
            repo,
        )
        .await?
    } else {
        ContractInvariants::default()
    };

    log::info!(
        "{} verified invariants found!",
        verified_invariants.invariants.len()
    );

    Ok(verified_invariants)
}

pub fn generate_openai_agent(repo: &RepoPaths) -> Result<Arc<AIAgent>> {
    // custom agent for digging up list of actors
    let config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.2")
        .with_preamble("You are a world-class expert at Solidity EVM smart contract auditing.")
        .with_file_retrieval(false)
        .with_openai_reasoning_effort("high");

    let agent = Arc::new(AgentFactory::create_openai_agent(&config)?);

    Ok(agent)
}

pub fn generate_gemini_agent(repo: &RepoPaths) -> Result<Arc<AIAgent>> {
    // custom agent for digging up list of actors
    let config = AgentConfig::new(Some(repo.clone()))
        .with_temperature(1.0)
        .with_model("gemini-3-pro-preview")
        .with_preamble("You are a world-class expert at Solidity EVM smart contract auditing.");

    let agent = Arc::new(AgentFactory::create_gemini_agent(&config)?);

    Ok(agent)
}
