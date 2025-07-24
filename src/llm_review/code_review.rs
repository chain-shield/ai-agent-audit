use crate::config::audit_config;
use crate::error::Result;
use crate::llm_review::config::CLAUDE_4_0_SONNET;
use crate::prepare_code::git_clone::RepoPaths;
use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        agent_factory::{AgentConfig, AgentFactory},
        config::{ContractInvariants, Findings},
        context_state::get_metadata_context,
    },
};
use log::info;
use rig::providers::openai::O3;
use std::{path::PathBuf, sync::Arc};
use tokio::fs;

use super::contract_file_map::get_file_from_contract;
use super::{enums::AIAgent, phases};

/// Multi-LLM security analysis orchestration.
///
/// This module coordinates parallel security analysis across multiple LLM providers,
/// implements verification and deduplication workflows, and manages cost tracking.

/// Orchestrates comprehensive security analysis of smart contracts using multiple LLM providers.
///
/// This function performs parallel vulnerability detection across multiple AI agents,
/// implements verification and deduplication workflows, and returns categorized findings.
///
/// # Arguments
/// * `codeblocks_path` - Path to the database containing generated code blocks
///
/// # Returns
/// * `HashMap<String, Findings>` - Security findings organized by contract
/// * `Vec<ContractInvariants>` - Protocol invariant analysis results
pub async fn review_codebase_for_security_issues(
    codeblocks_path: &PathBuf,
    repo: &RepoPaths,
) -> Result<(Findings, Vec<ContractInvariants>)> {
    let mut all_security_issues = Findings {
        findings: Vec::new(),
    };
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    let (ai_verify_agent, second_ai_verify_agent, ai_discovery_agents) =
        generate_ai_agents(repo).await?;

    let invariant_findings = Vec::<ContractInvariants>::new();

    let metadata_context = get_metadata_context().await?;

    for (contract, codeblock) in contracts.into_iter() {
        info!("\n\n-------- contract {} ---------------\n\n", contract);

        // grab additional context from RAG
        // let rag_context = get_rag_for_security_query(&codeblock, repo).await?;
        // let audit_context = format!(
        //     "\n## CONTEXT \n\n {} \n\n {}",
        //     metadata_context, rag_context
        // );

        // // Phase 1: Pre-fetch strategic files once to avoid rate limits during parallel analysis
        // let prefetched_files =
        //     phases::prefetch_context::execute(&codeblock, repo, &ai_planning_agent).await?;

        // Combine original context with prefetched files
        // let metadata_context = if prefetched_files.is_empty() {
        //     metadata_context.to_string()
        // } else {
        //     format!(
        //         "{}\n\n## Additional Protocol Files for More Context ------------------\n\n{}",
        //         metadata_context, prefetched_files
        //     )
        // };

        // Phase 1 generated enhanced codeblock
        let codeblock = enhance_codeblock(&contract, &codeblock, repo).await?;
        // info!("codeblock => {}", codeblock);

        // // Phase 2: Generate findings using parallel AI agents
        let raw_findings = phases::generate_findings::execute(
            &contract,
            &codeblock,
            &metadata_context,
            &ai_discovery_agents,
        )
        .await?;

        if !raw_findings.findings.is_empty() {
            // Phase 3: Verify findings and remove false positives
            info!("verify findings round 1....................\n\n");
            let verified_findings = phases::verify_findings::execute(
                raw_findings,
                &codeblock,
                &ai_verify_agent,
                &metadata_context,
            )
            .await?;

            info!("verify findings round 2....................\n\n");
            let double_verified_findings = phases::verify_findings::execute(
                verified_findings,
                &codeblock,
                &second_ai_verify_agent,
                &metadata_context,
            )
            .await?;

            // Phase 4: Quality check and enhance findings
            let final_findings = phases::quality_check::execute(
                double_verified_findings,
                &codeblock,
                &ai_verify_agent,
                &metadata_context,
            )
            .await?;

            all_security_issues.findings.extend(final_findings.findings);

            // TODO - save issues to Findings db
        }
    }

    // dedup combined findings
    let deduped_security_bugs = all_security_issues.dedup().await?;

    Ok((deduped_security_bugs, invariant_findings))
}

// combine codeblock with original file context (that codeblock came from)
// this contains natspec and additional context
pub async fn enhance_codeblock(
    contract: &str,
    codeblock: &str,
    repo: &RepoPaths,
) -> anyhow::Result<String> {
    let file = get_file_from_contract(contract, repo).await?;

    let file_content = fs::read_to_string(&file).await?;

    let filename = file.strip_prefix(&repo.root)?;
    info!("{} contains contract {}", filename.display(), contract);

    let enhanced_block = format!(
        "{} \n\n {}: \n\n {}",
        codeblock,
        filename.display(),
        file_content
    );

    Ok(enhanced_block)
}
pub async fn generate_ai_agents(
    repo: &RepoPaths,
) -> Result<(Arc<AIAgent>, Arc<AIAgent>, Vec<Arc<AIAgent>>)> {
    info!("setting up AI agents...");

    // Enhanced preamble for verification agent
    let verify_preamble = "

You are **SoliditySec-Verifier**, a senior smart-contract auditor focused on
*confirming* reported issues.";

    // You have access to retrieve_file_content tool that can search through different types of code files:
    // - 'source': Main application code and smart contracts
    // - 'test': Test files and test cases
    // - 'script': Deployment and build scripts
    // - 'library': Library and utility code
    //
    // Use them to:
    // 1. Check that a reported vulnerability exists in the *current* source code.
    // 2. Check if vulnerability is accurately reported
    // 3. Cross-reference with tests to understand intended behaviour.
    // 4. Inspect deployment scripts for mis-configurations.
    // 5. Verify library or inherited-contract logic.
    //
    // When formulating queries for the retrieve_file_content, keep them concise and focused (**under 1000 words**) to avoid exceeding embedding model context limits.
    // ";

    // Create verification agent using OpenAI O3
    let verify_config = AgentConfig::new(repo.clone())
        .with_temperature(1.0)
        .with_model(O3)
        .with_preamble(verify_preamble)
        .with_file_picker(false); // Disabled to avoid rate limits

    let second_verify_config = AgentConfig::new(repo.clone())
        .with_temperature(1.0)
        .with_model(CLAUDE_4_0_SONNET)
        .with_max_tokens(64_000)
        .with_preamble(verify_preamble)
        .with_file_picker(false) // Disabled to avoid rate limits
        .with_file_retrieval(false);

    let ai_verify_agent = Arc::new(AgentFactory::create_openai_agent(&verify_config)?);
    let second_ai_verify_agent =
        Arc::new(AgentFactory::create_anthropic_agent(&second_verify_config)?);

    // Enhanced preamble for discovery agents
    let solidity_auditor_preamble = "You are a world-class expert at smart contract auditing, renowned for your ability to find the most complex and trickiest security vulnerabilities in Solidity codebases.";

    // Create discovery agents using Gemini models
    let mut ai_discovery_agents = Vec::new();

    let gemini_config = AgentConfig::new(repo.clone())
        .with_temperature(1.0)
        .with_model("gemini-2.5-pro")
        .with_preamble(solidity_auditor_preamble)
        .with_file_retrieval(false)
        .with_file_picker(false);

    // let claude_config = AgentConfig::new(repo.clone())
    //     .with_temperature(1.0)
    //     .with_model(CLAUDE_3_7_SONNET)
    //     .with_preamble(solidity_auditor_preamble)
    //     .with_max_tokens(64_000)
    //     .with_file_retrieval(true)
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_dynamic_context(false);
    //
    for _ in 0..audit_config().runs {
        let agent = Arc::new(AgentFactory::create_gemini_agent(&gemini_config)?);
        ai_discovery_agents.push(agent);
    }

    // let ai_planning_agent = Arc::new(AgentFactory::create_gemini_agent(&gemini_config)?);
    // info!("Created {} discovery agents", ai_discovery_agents.len());

    Ok((ai_verify_agent, second_ai_verify_agent, ai_discovery_agents))
}
