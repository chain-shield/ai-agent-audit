use crate::{
    llm_review::{
        agent::{
            agent_enums::AIAgent,
            agent_factory::{AgentConfig, AgentFactory},
        },
        pattern_phases,
        threat_models::{
            actors::{Actor, Actors},
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
    // let actor_discovery_agent = generate_gemini_agent(repo)?;
    let actor_discovery_agent = generate_openai_agent(repo, "medium")?;
    let actor_verify_agent = generate_openai_agent(repo, "high")?;

    // Phase 1: Generate actors and their capabilities
    log::info!("PRE AUDIT PHASE: GENERATE ACTORS");
    // let actors: Actors =
    //     pattern_phases::generate_actors::execute(codeblock, &actor_discovery_agent, repo).await?;
    let actors: Actors = pattern_phases::generate_patterns::execute(
        IssuePrompt::Actor,
        codeblock,
        &actor_discovery_agent,
        repo,
    )
    .await?;

    let actor_count = actors.actors.len();
    log::info!("total of {} Actors found!", actor_count);

    let actors_with_id: Actors = Actors {
        actors: actors
            .actors
            .into_iter()
            .map(|a| Actor {
                id: Some(nanoid!()),
                ..a
            })
            .collect(),
    };

    // Phase 2: Verify actors
    log::info!("PHASE 2: VERIFY actors");
    let verified_actors = if !actors_with_id.issues().is_empty() {
        pattern_phases::verify_patterns::verify_actors(
            actors_with_id,
            codeblock,
            &actor_verify_agent,
            repo,
        )
        .await?
    } else {
        Actors::default()
    };

    log::info!("{} verified actors found!", verified_actors.actors.len());

    Ok(verified_actors)
}

pub async fn generate_invariants(codeblock: &str, repo: &RepoPaths) -> Result<ContractInvariants> {
    let invariant_prompt = IssuePrompt::Invariant(InvariantType::iter().collect());

    // let invariant_discovery_agent = generate_invariant_gemini_agent(repo)?;
    let invariant_discovery_agent = generate_invariant_openai_agent(repo, "medium")?;
    let invariant_verify_agent = generate_openai_agent(repo, "high")?;

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

pub fn generate_openai_agent(repo: &RepoPaths, reasoning_effort: &str) -> Result<Arc<AIAgent>> {
    // custom agent for digging up list of actors
    let config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.2")
        .with_preamble("You are a world-class expert at Solidity EVM smart contract auditing.")
        .with_file_retrieval(false)
        .with_openai_reasoning_effort(reasoning_effort);

    let agent = Arc::new(AgentFactory::create_openai_agent(&config)?);

    Ok(agent)
}

const INVARIANT_DISCOVERY_SYSTEM_PROMPT: &'static str = r#"
You are a world-class expert at Solidity EVM smart contract auditing. You specialize in
discovering invariants in complex solidity codebases.

## How to think

When designing each invariant:

1. Model the contract and its role
   - Identify what the contract is for (e.g. signature validation, vault, permissions, recovery module, oracle, router).
   - Identify who the key actors are (owners, signers, admins, modules, external protocols).

2. Extract candidate invariants from the spec and context
   - Translate any stated invariants or assumptions in the docs/scope into precise, checkable properties.
   - Think about:
     - Access control and privilege boundaries.
     - Balance and accounting relationships.
     - Nonces, counters, and sequencing.
     - Configuration / image hash / checkpointer behavior.
     - Cross-contract or cross-chain relationships if referenced.

3. Make them machine-checkable
   - Express each invariant as a clear predicate over contract state and/or events.
   - Use concrete conditions like:
     - Relationships between balances and totals.
     - Relationships between stored configuration and computed hashes.
     - Conditions on who is allowed to perform which actions under which flags/modes.
     - Temporal properties across function calls (e.g. nonces, cooldowns, checkpoints).

4. Actively search for violations
   - For each invariant, scan the code for:
     - Branches that skip checks (e.g. flag bits, mode switches, early returns).
     - Edge cases in loops, array indexing, or boundary conditions.
     - Multi-step flows (chained signatures, batched calls, upgradable configs) where state may drift from the intended invariant.
   - If you find a credible way the invariant could be broken, mark it as PossibleViolation (or equivalent status) and describe:
     - The pre-state (relevant configuration / storage / role assumptions).
     - The actions or sequence of calls.
     - The post-state and why it violates the invariant.
     - The likely impact.

5. Coverage vs signal
   - It is acceptable to include some simpler invariants if they help cover more potential High/Medium issues.
   - Still avoid vague or purely stylistic "invariants"; each one should correspond to a concrete, checkable property whose violation could matter in practice.
"#;

pub fn generate_invariant_openai_agent(
    repo: &RepoPaths,
    reasoning_effort: &str,
) -> Result<Arc<AIAgent>> {
    // custom agent for digging up list of actors
    let config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.2")
        .with_preamble(INVARIANT_DISCOVERY_SYSTEM_PROMPT)
        .with_file_retrieval(false)
        .with_openai_reasoning_effort(reasoning_effort);

    let agent = Arc::new(AgentFactory::create_openai_agent(&config)?);

    Ok(agent)
}

pub fn generate_invariant_gemini_agent(repo: &RepoPaths) -> Result<Arc<AIAgent>> {
    // custom agent for digging up list of actors
    let config = AgentConfig::new(Some(repo.clone()))
        .with_model("gemini-3-pro-preview")
        .with_preamble(INVARIANT_DISCOVERY_SYSTEM_PROMPT)
        .with_file_retrieval(false);

    let agent = Arc::new(AgentFactory::create_gemini_agent(&config)?);

    Ok(agent)
}

pub fn generate_gemini_agent(repo: &RepoPaths) -> Result<Arc<AIAgent>> {
    // custom agent for digging up list of actors
    let config = AgentConfig::new(Some(repo.clone()))
        .with_temperature(1.0)
        .with_model("gemini-3-pro-preview")
        .with_preamble("You are a world-class Solidity EVM security researcher.");

    let agent = Arc::new(AgentFactory::create_gemini_agent(&config)?);

    Ok(agent)
}
