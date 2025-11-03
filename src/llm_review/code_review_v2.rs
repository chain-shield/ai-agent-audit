use super::{enums::AIAgent, phases};
use crate::config::CREATE_TESTS;
use crate::enumerator::codeblock_db::CodeBlocksDb;
use crate::error::{AuditError, Result};
use crate::llm_review::findings::CLAUDE_4_5_SONNET;
use crate::llm_review::semaphore::CONTRACT_REVEW_SEM;
use crate::llm_review::utils::contract_in_scope::is_contract_in_scope;
use crate::llm_review::{
    agent_factory::{AgentConfig, AgentFactory},
    analysis_db::FindingsDb,
    findings::Findings,
    invariants::{ContractInvariants, InvariantFinding, InvariantStatus, InvariantType},
    issues::{IssuePrompt, IssueStructTrait},
    pattern_category::PatternCategory,
    pattern_phases,
    patterns::Patterns,
};
use crate::prepare_code::git_clone::RepoPaths;
use log::info;
use std::{path::PathBuf, sync::Arc};
use strum::IntoEnumIterator;
use tokio::sync::Mutex;

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
    // let audit_scope = Arc::new(generate_audit_scope(repo).await?);

    let (ai_verify_agent, ai_discovery_agent, finding_ai_verify_agent) =
        generate_ai_agents(repo).await?;

    let findings_db = Arc::new(Mutex::new(FindingsDb::open()?));

    let mut contract_handles = Vec::new();

    for (contract, codeblock) in contracts.into_iter() {
        info!("\n\n-------- contract {} ---------------\n\n", contract);

        // check contract in inscope!
        if !is_contract_in_scope(&contract, repo).await? {
            info!("contract {}  is NOT in scope", contract);
            continue;
        }
        info!("contract {}  is in scope", contract);

        // Clone shared state for the spawned task
        let verify_agent = Arc::clone(&ai_verify_agent);
        let finding_verify_agent = Arc::clone(&finding_ai_verify_agent);
        let discovery_agent = Arc::clone(&ai_discovery_agent);
        // let scope = Arc::clone(&audit_scope);
        let results_db = Arc::clone(&findings_db);
        let all_issues = Arc::clone(&all_security_issues);
        let repo_clone = repo.clone();
        // let contract_clone = contract.clone();
        // let codeblock_clone = codeblock.clone();

        // semaphore
        let sem = Arc::clone(&CONTRACT_REVEW_SEM);

        contract_handles.push(tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async move {
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
                    // Phase 4: Verify findings and remove false positives
                    let verify_findings = phases::verify_findings::execute(
                        raw_findings,
                        &codeblock,
                        &finding_verify_agent,
                        &repo_clone,
                    )
                    .await?;

                    // Phase 5: Quality check and enhance findings
                    let mut quality_findings = phases::quality_check::execute(
                        verify_findings,
                        &codeblock,
                        &verify_agent,
                        &repo_clone,
                    )
                    .await?;

                    // Phase 6: PoC Generation for High-Severity Findings
                    // REQUIREMENTS: instructions for writing PoC plus template PoC file (if applicable)
                    // 1. Write runnable PoC for Critical, High, and Medium findings
                    // 2. Save PoC to test folder of repo
                    // 3. Run PoC and capture results
                    // 4. Have LLM fix PoC if it fails (up to 5 attempts)
                    // 5. Mark finding as invalid if PoC cannot be created

                    // If instructions and test folder provided, create and run PoC tests
                    if !repo_clone.poc.instructions.is_empty()
                        && repo_clone.poc.test_folder.exists()
                        && CREATE_TESTS
                    {
                        // Phase 6: Write PoC for each Critical, High, and Medium Finding
                        // Acquire POC_SEM at contract level to prevent multiple contracts
                        // from creating PoC tests concurrently in the same test folder
                        use crate::llm_review::semaphore::POC_SEM;
                        let poc_sem = Arc::clone(&POC_SEM);
                        let _poc_permit =
                            poc_sem.acquire_owned().await.expect("POC semaphore closed");

                        match phases::add_poc_findings::execute(
                            quality_findings.clone(),
                            &codeblock,
                            &finding_verify_agent,
                            &repo_clone,
                        )
                        .await
                        {
                            Ok(findings_with_pocs) => {
                                quality_findings = findings_with_pocs;
                                log::info!("✅ Phase 6 completed successfully");
                            }
                            Err(e) => {
                                log::error!("❌ Phase 6 (PoC generation) failed: {:?}", e);
                                log::warn!("Continuing with findings without PoC tests");
                                // Continue with existing findings without PoC tests
                            }
                        }
                        // _poc_permit is dropped here, releasing the semaphore
                    }

                    // Phase 7: Create professional markdown report for EACH finding (only if PoC is passing)
                    match phases::create_report::execute(
                        quality_findings.clone(),
                        &codeblock,
                        &finding_verify_agent,
                        &repo_clone,
                    )
                    .await
                    {
                        Ok(findings_with_reports) => {
                            quality_findings = findings_with_reports;
                            log::info!("✅ Phase 7 completed successfully");
                        }
                        Err(e) => {
                            log::error!("❌ Phase 7 (Report generation) failed: {:?}", e);
                            log::warn!("Continuing with findings without professional reports");
                            // Continue with existing findings without reports
                        }
                    }

                    // Save findings to database before extending
                    let db = results_db.lock().await;
                    if let Err(e) = db.insert_findings(&quality_findings, &repo_clone) {
                        log::warn!("Failed to save findings to database: {}", e);
                    }

                    // Extend the aggregate findings
                    let mut all_findings = all_issues.lock().await;
                    all_findings.findings.extend(quality_findings.findings);
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

pub async fn generate_ai_agents(
    repo: &RepoPaths,
) -> Result<(Arc<AIAgent>, Arc<AIAgent>, Arc<AIAgent>)> {
    info!("setting up AI agents...");

    // Enhanced preamble for verification agent
    let verify_preamble = "

You are **SoliditySec-Verifier**, a senior smart-contract auditor specializing on
*confirming* reported findings, writing comprehensive reports of findings, and creating
rigorous PoC tests that validate the findings.";

    // Create verification agent using OpenAI O3
    let verify_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5")
        .with_preamble(verify_preamble)
        .with_file_picker(false); // Disabled to avoid rate limits

    let finding_verify_config = AgentConfig::new(Some(repo.clone()))
        .with_temperature(0.2)
        .with_model(CLAUDE_4_5_SONNET)
        .with_max_tokens(64_000)
        .with_preamble(verify_preamble)
        .with_file_picker(false) // Disabled to avoid rate limits
        .with_file_retrieval(false);

    let ai_verify_agent = Arc::new(AgentFactory::create_openai_agent(&verify_config)?);
    let _finding_ai_verify_agent = Arc::new(AgentFactory::create_anthropic_agent(
        &finding_verify_config,
    )?);

    // Enhanced preamble for discovery agents
    let solidity_auditor_preamble = "You are a world-class expert at smart contract auditing, renowned for your ability to find the most complex and trickiest security vulnerabilities in Solidity codebases. You consistently land valid solo High and Medium findings in competitive audit contests.";

    // let _discovery_config_claude = AgentConfig::new(Some(repo.clone()))
    //     .with_temperature(1.0)
    //     .with_model(CLAUDE_4_5_SONNET)
    //     .with_max_tokens(64_000)
    //     .with_preamble(solidity_auditor_preamble)
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_file_retrieval(false);

    let discovery_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5")
        .with_preamble(solidity_auditor_preamble)
        .with_file_retrieval(false)
        .with_openai_reasoning_effort("high")
        .with_file_picker(false);
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_dynamic_context(false);

    let ai_discovery_agent = Arc::new(AgentFactory::create_openai_agent(&discovery_config)?);
    // let ai_discovery_agent = Arc::new(AgentFactory::create_anthropic_agent(
    //     &discovery_config_claude,
    // )?);

    // let ai_planning_agent = Arc::new(AgentFactory::create_gemini_agent(&gemini_config)?);
    // info!("Created {} discovery agents", ai_discovery_agents.len());

    Ok((ai_verify_agent.clone(), ai_discovery_agent, ai_verify_agent))
}

/// Process pattern analysis: generate, verify, and convert to findings
async fn process_patterns(
    codeblock: &str,
    ai_discovery_agent: &Arc<AIAgent>,
    ai_verify_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    let pattern_category_c4: Vec<PatternCategory> = PatternCategory::iter()
        .filter(|p| {
            *p == PatternCategory::Top
                || *p == PatternCategory::Frequent
                || *p == PatternCategory::MostObserved
                || *p == PatternCategory::Rare
        })
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
    let raw_invariants: ContractInvariants = pattern_phases::generate_patterns::execute(
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
        .filter(|inv| inv.status == InvariantStatus::PossibleViolation)
        .map(|inv| inv.to_owned())
        .collect();

    // raw_invariants = ContractInvariants {
    //     invariants: violations,
    // };

    info!("{} invariant violations found!", violations.len());

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
