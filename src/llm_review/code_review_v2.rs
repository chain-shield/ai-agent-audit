use crate::enumerator::codeblock_db::CodeBlocksDb;
use crate::error::{AuditError, Result};
use crate::llm_review::semaphore::CONTRACT_REVEW_SEM;
use crate::llm_review::{
    agent_factory::{AgentConfig, AgentFactory},
    analysis_db::FindingsDb,
    context_state::generate_audit_scope,
    findings::Findings,
    findings::CLAUDE_4_0_SONNET,
    invariants::{ContractInvariants, InvariantFinding, InvariantStatus, InvariantType},
    issues::{IssuePrompt, IssueStructTrait},
    pattern_category::PatternCategory,
    pattern_phases,
    patterns::Patterns,
};
use crate::prepare_code::git_clone::RepoPaths;
use log::info;
use rig::providers::openai::O3;
use std::{path::PathBuf, sync::Arc};
use strum::IntoEnumIterator;
use tokio::fs;
use tokio::sync::Mutex;

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
pub async fn review_codebase_for_security_issues_v2(
    codeblocks_path: &PathBuf,
    repo: &RepoPaths,
) -> Result<Findings> {
    let all_security_issues = Arc::new(Mutex::new(Findings {
        findings: Vec::new(),
    }));
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts(repo)?;
    let audit_scope = Arc::new(generate_audit_scope(repo).await?);

    let (ai_verify_agent, ai_discovery_agent) = generate_ai_agents(repo).await?;

    let findings_db = Arc::new(Mutex::new(FindingsDb::open()?));

    let mut contract_handles = Vec::new();

    for (contract, codeblock) in contracts.into_iter() {
        info!("\n\n-------- contract {} ---------------\n\n", contract);

        // Clone shared state for the spawned task
        let verify_agent = Arc::clone(&ai_verify_agent);
        let discovery_agent = Arc::clone(&ai_discovery_agent);
        let scope = Arc::clone(&audit_scope);
        let results_db = Arc::clone(&findings_db);
        let all_issues = Arc::clone(&all_security_issues);
        let repo_clone = repo.clone();
        let contract_clone = contract.clone();
        let codeblock_clone = codeblock.clone();

        // semaphore
        let sem = Arc::clone(&CONTRACT_REVEW_SEM);

        contract_handles.push(tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async move {
                // Generate enhanced codeblock (includes file context)
                let codeblock = enhance_codeblock(&contract_clone, &codeblock_clone, &repo_clone)
                    .await
                    .map_err(|e| {
                        use std::io::{Error as IoError, ErrorKind};
                        AuditError::file_system(
                            "enhance_codeblock",
                            format!("could not generate codeblock: {e}"),
                            IoError::new(ErrorKind::Other, e.to_string()),
                        )
                    })?;

                // Run pattern and invariant analysis concurrently within this task
                let (patterns_res, invariants_res) = tokio::join!(
                    process_patterns(&codeblock, &discovery_agent, &verify_agent, &repo_clone),
                    process_invariants(&codeblock, &discovery_agent, &verify_agent, &repo_clone)
                );

                let mut raw_findings = Findings::default();
                if let Ok(pats) = patterns_res {
                    raw_findings.findings.extend(pats.findings);
                } else if let Err(e) = patterns_res {
                    log::error!("pattern analysis failed: {:#}", e);
                }
                if let Ok(invs) = invariants_res {
                    raw_findings.findings.extend(invs.findings);
                } else if let Err(e) = invariants_res {
                    log::error!("invariant analysis failed: {:#}", e);
                }

                if !raw_findings.findings.is_empty() {
                    // Phase 3: Verify findings and remove false positives
                    let mut verify_findings = phases::verify_findings::execute(
                        raw_findings,
                        &codeblock,
                        &verify_agent,
                        &repo_clone,
                    )
                    .await?;

                    // Phase 3a: Scope findings (only if scope provided)
                    if !scope.is_empty() {
                        verify_findings = phases::scope_findings::execute(
                            verify_findings,
                            &codeblock,
                            &verify_agent,
                            &repo_clone,
                        )
                        .await?;
                    }

                    // Phase 4: Quality check and enhance findings
                    let final_findings = phases::quality_check::execute(
                        verify_findings,
                        &codeblock,
                        &verify_agent,
                        &repo_clone,
                    )
                    .await?;

                    // Save findings to database before extending
                    let db = results_db.lock().await;
                    if let Err(e) = db.insert_findings(&final_findings, &repo_clone) {
                        log::warn!("Failed to save findings to database: {}", e);
                    }

                    // Extend the aggregate findings
                    let mut all_findings = all_issues.lock().await;
                    all_findings.findings.extend(final_findings.findings);
                }
                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error processing contract reviews: {:#}", e);
            }
            Ok::<_, AuditError>(())
        }));
    }

    for handle in contract_handles {
        let _ = handle.await?;
    }

    // dedup combined findings
    let security_issues = all_security_issues.lock().await;
    let deduped = security_issues.clone().dedup().await?;
    drop(security_issues);

    Ok(deduped)
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
pub async fn generate_ai_agents(repo: &RepoPaths) -> Result<(Arc<AIAgent>, Arc<AIAgent>)> {
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
    let verify_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5")
        // .with_openai_service_tier("flex")
        .with_preamble(verify_preamble)
        .with_file_picker(false); // Disabled to avoid rate limits

    let _ = AgentConfig::new(Some(repo.clone()))
        .with_temperature(1.0)
        .with_model(CLAUDE_4_0_SONNET)
        .with_max_tokens(64_000)
        .with_preamble(verify_preamble)
        .with_file_picker(false) // Disabled to avoid rate limits
        .with_file_retrieval(false);

    let ai_verify_agent = Arc::new(AgentFactory::create_openai_agent(&verify_config)?);

    // Enhanced preamble for discovery agents
    let solidity_auditor_preamble = "You are a world-class expert at smart contract auditing, renowned for your ability to find the most complex and trickiest security vulnerabilities in Solidity codebases.";

    let _gemini_config = AgentConfig::new(Some(repo.clone()))
        .with_temperature(1.0)
        .with_model("gemini-2.5-pro")
        .with_preamble(solidity_auditor_preamble)
        .with_file_retrieval(false)
        .with_file_picker(false);

    let openai_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5")
        .with_preamble(solidity_auditor_preamble)
        .with_file_retrieval(false)
        // .with_openai_service_tier("flex")
        .with_openai_reasoning_effort("high")
        .with_file_picker(false);
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_dynamic_context(false);
    //
    let ai_discovery_agent = Arc::new(AgentFactory::create_openai_agent(&openai_config)?);

    // let ai_planning_agent = Arc::new(AgentFactory::create_gemini_agent(&gemini_config)?);
    // info!("Created {} discovery agents", ai_discovery_agents.len());

    Ok((ai_verify_agent, ai_discovery_agent))
}

/// Process pattern analysis: generate, verify, and convert to findings
async fn process_patterns(
    codeblock: &str,
    ai_discovery_agent: &Arc<AIAgent>,
    ai_verify_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    let pattern_category_c4: Vec<PatternCategory> = PatternCategory::iter()
        .filter(|p| *p == PatternCategory::Top || *p == PatternCategory::Frequent)
        .collect();

    let pattern_prompt = IssuePrompt::Pattern(pattern_category_c4);

    // Phase 1: Generate patterns
    info!("PHASE 1: GENERATE PATTERNS");
    let raw_patterns: Patterns = pattern_phases::generate_patterns::execute(
        pattern_prompt,
        codeblock,
        ai_discovery_agent,
        repo,
    )
    .await?;

    // Phase 2: Verify patterns
    info!("PHASE 2: VERIFY PATTERNS");
    let verified_patterns = if !raw_patterns.issues().is_empty() {
        pattern_phases::verify_patterns::verify_patterns(
            raw_patterns,
            codeblock,
            ai_verify_agent,
            repo,
        )
        .await?
    } else {
        Patterns::default()
    };

    info!("PHASE 3: GENERATE FINDINGS FROM PATTERNS");

    if !verified_patterns.issues().is_empty() {
        let findings_from_patterns = pattern_phases::pattern_to_findings::execute(
            verified_patterns,
            codeblock,
            ai_discovery_agent,
            repo,
        )
        .await?;

        Ok(findings_from_patterns)
    } else {
        Ok(Findings::default())
    }
}

/// Process invariant analysis: generate, verify, and convert to findings
async fn process_invariants(
    codeblock: &str,
    ai_discovery_agent: &Arc<AIAgent>,
    ai_verify_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    let invariant_prompt = IssuePrompt::Invariant(InvariantType::iter().collect());

    // Phase 1: Generate invariants
    info!("PHASE 1: GENERATE INVARIANTS");
    let mut raw_invariants: ContractInvariants = pattern_phases::generate_patterns::execute(
        invariant_prompt,
        codeblock,
        ai_discovery_agent,
        repo,
    )
    .await?;

    info!(
        "total of {} invariant found!",
        raw_invariants.invariants.len()
    );

    // Filter for invariant violations only
    let violations: Vec<InvariantFinding> = raw_invariants
        .issues()
        .iter()
        .filter(|inv| inv.status == InvariantStatus::Holds)
        .map(|inv| inv.to_owned())
        .collect();

    raw_invariants = ContractInvariants {
        invariants: violations,
    };

    info!(
        "{} invariant violations found!",
        raw_invariants.invariants.len()
    );

    // Phase 2: Verify invariants
    info!("PHASE 2: VERIFY INVARIANTS");
    let verified_invariants = if !raw_invariants.issues().is_empty() {
        pattern_phases::verify_patterns::verify_invariants(
            raw_invariants,
            codeblock,
            ai_verify_agent,
            repo,
        )
        .await?
    } else {
        ContractInvariants::default()
    };

    info!("PHASE 3: GENERATE FINDINGS FROM INVARIANTS");

    if !verified_invariants.issues().is_empty() {
        let findings_from_invariants = pattern_phases::pattern_to_findings::execute(
            verified_invariants,
            codeblock,
            ai_discovery_agent,
            repo,
        )
        .await?;

        Ok(findings_from_invariants)
    } else {
        Ok(Findings::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_processing_structure() {
        // This test verifies the threading structure compiles and runs
        // without actually calling the AI agents (which would require setup)

        let handles: Vec<tokio::task::JoinHandle<Result<Findings>>> = Vec::new();

        // Verify we can create the handle structure
        assert_eq!(handles.len(), 0);

        // Test that our Result<Findings> type works correctly
        let test_findings = Findings::default();
        assert!(test_findings.findings.is_empty());
    }
}
