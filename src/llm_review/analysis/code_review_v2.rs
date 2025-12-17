use crate::config::{
    ACTOR_RUNS, ALL_PATTERN_APPROACH, CREATE_TESTS, DIRECT_TO_FINDING_MODE,
    NICHE_PATTERN_ANALYSIS_MODE, SKIP_LIBRARIES, SKIP_PATTERN_RUNS,
};
use crate::enumerator::codeblock_db::CodeBlocksDb;
use crate::error::{AuditError, Result};
use crate::llm_review::analysis::semaphore::CONTRACT_REVEW_SEM;
use crate::llm_review::contract::contract_category::{
    get_contract_spec_from_category, ContractCategory,
};
use crate::llm_review::contract::contract_file_map::ContractType;
use crate::llm_review::dynamic_prompts::actors::{
    generate_formated_list_from_actor_data, generate_formatted_actor_abuse_list,
};
use crate::llm_review::findings::findings::{Finding, CLAUDE_4_5_SONNET};
use crate::llm_review::utils::contract_in_scope::contract_scope_and_type;
use crate::llm_review::{agent::agent_enums::AIAgent, phases};
use crate::llm_review::{
    agent::agent_factory::{AgentConfig, AgentFactory},
    analysis::analysis_db::FindingsDb,
    findings::findings::Findings,
    pattern_phases,
    threat_models::{
        actors::{ActorAbuses, Actors},
        invariants::{ContractInvariants, InvariantFinding, InvariantStatus, InvariantType},
        issues::{IssuePrompt, IssueStructTrait},
        pattern_category::PatternCategory,
        patterns::Patterns,
    },
};
use crate::prepare_code::git_clone::RepoPaths;
use crate::reporting::patterns::save_patterns;
use log::info;
use nanoid::nanoid;
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

    // ONLY audit these
    let custom_scoped_contracts = Some(vec![
        "Jackpot".to_string(),
        "JackpotBridgeManager".to_string(),
    ]);
    // let custom_scoped_contracts: Option<Vec<_>> = None;

    // skip these contracts
    // let custom_out_of_scoped_contracts: Option<Vec<String>> = Some(vec![
    //     "Core".to_string(),
    //     "FlashAccountant".to_string(),
    //     "TWAMM".to_string(),
    //     "BasePositions".to_string(),
    //     "MEVCapture".to_string(),
    // ]);
    let custom_out_of_scoped_contracts: Option<Vec<String>> = None;

    let (
        ai_verify_agent,
        pattern_discovery_agent,
        finding_ai_verify_agent,
        finding_discovery_agent,
    ) = generate_ai_agents(repo).await?;

    let findings_db = Arc::new(Mutex::new(FindingsDb::open()?));

    let mut contract_handles = Vec::new();

    for (contract, (codeblock, contract_category)) in contracts.into_iter() {
        info!("\n\n-------- contract {} ---------------\n\n", contract);

        if let Some(scoped_contracts) = &custom_scoped_contracts {
            if !scoped_contracts.contains(&contract) {
                info!("contract {} is NOT in custom scope", contract);
                continue;
            }
        }

        if let Some(out_of_scope) = &custom_out_of_scoped_contracts {
            if out_of_scope.contains(&contract) {
                info!("contract {} is NOT in custom scope", contract);
                continue;
            }
        }

        // check contract in inscope!
        let (is_contract_in_scope, contract_type_option) =
            contract_scope_and_type(&contract, repo).await?;

        let contract_type = contract_type_option.unwrap_or(ContractType::Contract);

        if !is_contract_in_scope {
            info!("{} {} is NOT in scope", contract_type.to_string(), contract);
            continue;
        }

        if SKIP_LIBRARIES && contract_type == ContractType::Library {
            continue;
        }

        info!("{} {} is in scope", contract_type.to_string(), contract);

        // Clone shared state for the spawned task
        let verify_agent = Arc::clone(&ai_verify_agent);
        let finding_verify_agent = Arc::clone(&finding_ai_verify_agent);
        let pattern_discovery_agent = Arc::clone(&pattern_discovery_agent);
        let finding_discovery_agent = Arc::clone(&finding_discovery_agent);
        let results_db = Arc::clone(&findings_db);
        let all_issues = Arc::clone(&all_security_issues);
        let repo_clone = repo.clone();

        // semaphore
        let sem = Arc::clone(&CONTRACT_REVEW_SEM);

        contract_handles.push(tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async move {
                // Run pattern, invariant, and actor analysis concurrently within this task
                let (patterns_res, invariants_res, actors_res) = {
                    let pattern_categories =
                        get_pattern_category_from_contract_category(contract_category);
                    tokio::join!(
                        process_patterns(
                            &codeblock,
                            pattern_categories,
                            &pattern_discovery_agent,
                            &finding_discovery_agent,
                            &verify_agent,
                            &repo_clone
                        ),
                        process_invariants(
                            &codeblock,
                            &pattern_discovery_agent,
                            &finding_discovery_agent,
                            &verify_agent,
                            &repo_clone
                        ),
                        process_actors(
                            &codeblock,
                            &pattern_discovery_agent,
                            &finding_discovery_agent,
                            &verify_agent,
                            &repo_clone
                        )
                    )
                };

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
                if let Ok(acts) = actors_res {
                    raw_findings.findings.extend(acts.findings);
                } else if let Err(e) = actors_res {
                    log::error!("actor analysis failed: {:#}", e);
                }

                if !raw_findings.findings.is_empty() {
                    // add uuid to each finding to uniquely identify

                    let findings_with_id: Findings = Findings {
                        findings: raw_findings
                            .findings
                            .into_iter()
                            .map(|f| Finding {
                                id: Some(nanoid!()),
                                ..f
                            })
                            .collect(),
                    };

                    // Phase 4: Verify findings and remove false positives
                    let mut verify_findings = phases::verify_rounds::execute_rounds(
                        findings_with_id,
                        &codeblock,
                        &finding_verify_agent,
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
                        use crate::llm_review::analysis::semaphore::POC_SEM;
                        let poc_sem = Arc::clone(&POC_SEM);
                        let _poc_permit =
                            poc_sem.acquire_owned().await.expect("POC semaphore closed");

                        match phases::add_poc_findings::execute(
                            verify_findings.clone(),
                            &codeblock,
                            &finding_verify_agent,
                            &repo_clone,
                        )
                        .await
                        {
                            Ok(findings_with_pocs) => {
                                verify_findings = findings_with_pocs;
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
                        verify_findings.clone(),
                        &codeblock,
                        &finding_verify_agent,
                        &repo_clone,
                    )
                    .await
                    {
                        Ok(findings_with_reports) => {
                            verify_findings = findings_with_reports;
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
                    if let Err(e) = db.insert_findings(&verify_findings, &repo_clone) {
                        log::warn!("Failed to save findings to database: {}", e);
                    }

                    // Extend the aggregate findings
                    let mut all_findings = all_issues.lock().await;
                    all_findings.findings.extend(verify_findings.findings);
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
) -> Result<(Arc<AIAgent>, Arc<AIAgent>, Arc<AIAgent>, Arc<AIAgent>)> {
    info!("setting up AI agents...");

    // Enhanced preamble for verification agent
    let verify_preamble = "

    You are **SoliditySec-Verifier**, a Solidity EVM senior smart-contract auditor, and top Code4rena judge, specializing on
    verifying reported findings, writing comprehensive reports of findings, and creating
    rigorous PoC tests that validate the findings.";

    // Create verification agent using OpenAI O3
    let verify_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.2")
        .with_preamble(verify_preamble)
        .with_file_picker(false) // Disabled to avoid rate limits
        .with_openai_reasoning_effort("high");

    let _verify_config_gemini = AgentConfig::new(Some(repo.clone()))
        .with_temperature(1.0)
        .with_model("gemini-3-pro-preview")
        .with_preamble(verify_preamble);

    let _finding_verify_config = AgentConfig::new(Some(repo.clone()))
        .with_temperature(0.2)
        .with_model(CLAUDE_4_5_SONNET)
        .with_max_tokens(64_000)
        .with_preamble(verify_preamble)
        .with_file_picker(false) // Disabled to avoid rate limits
        .with_file_retrieval(false);

    let ai_finding_verify_agent = Arc::new(AgentFactory::create_openai_agent(&verify_config)?);
    // let ai_pattern_verify_agent =
    //     Arc::new(AgentFactory::create_gemini_agent(&verify_config_gemini)?);
    // let finding_ai_verify_agent = Arc::new(AgentFactory::create_anthropic_agent(
    //     &finding_verify_config,
    // )?);

    // Enhanced preamble for discovery agents
    let solidity_auditor_preamble = r#"You are a world-class expert at smart contract auditing, renowned for finding the most complex and tricky vulnerabilities in EVM Solidity codebases. You consistently land valid solo High and Medium findings in competitive audit contests. Prioritize semantic, multi-step, cross-contract, and incentive-based attack paths over syntactic pattern matching."#;
    // let _discovery_config_claude = AgentConfig::new(Some(repo.clone()))
    //     .with_temperature(1.0)
    //     .with_model(CLAUDE_4_5_SONNET)
    //     .with_max_tokens(64_000)
    //     .with_preamble(solidity_auditor_preamble)
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_file_retrieval(false);

    let pattern_discovery_config_gemini = AgentConfig::new(Some(repo.clone()))
        .with_temperature(1.0)
        .with_model("gemini-3-pro-preview")
        .with_preamble(solidity_auditor_preamble);

    let _pattern_discovery_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.1")
        .with_preamble(solidity_auditor_preamble)
        .with_file_retrieval(false)
        .with_openai_reasoning_effort("high")
        .with_file_picker(false);

    // let pattern_discovery_agent = Arc::new(AgentFactory::create_openai_agent(
    //     &pattern_discovery_config,
    // )?);
    // let finding_discovery_agent = Arc::new(AgentFactory::create_openai_agent(
    //     &pattern_discovery_config,
    // )?);

    let pattern_discovery_gemini_agent = Arc::new(AgentFactory::create_gemini_agent(
        &pattern_discovery_config_gemini,
    )?);
    // let ai_discovery_agent = Arc::new(AgentFactory::create_anthropic_agent(
    //     &discovery_config_claude,
    // )?);

    Ok((
        ai_finding_verify_agent.clone(),
        pattern_discovery_gemini_agent.clone(),
        ai_finding_verify_agent,
        pattern_discovery_gemini_agent,
    ))
}

fn get_pattern_category_from_contract_category(
    contract_category: ContractCategory,
) -> Vec<PatternCategory> {
    if ALL_PATTERN_APPROACH {
        return vec![PatternCategory::R1, PatternCategory::R2];
    }
    let default_pattern_categories = vec![
        PatternCategory::Top,
        PatternCategory::MostObserved,
        PatternCategory::Rare,
        PatternCategory::Frequent,
        PatternCategory::Top,
        PatternCategory::Frequent,
    ];

    // if NICHE_PATTERN_ANALYSIS is false than always return default_pattern_categories
    if contract_category == ContractCategory::Unknown || !NICHE_PATTERN_ANALYSIS_MODE {
        return default_pattern_categories;
    }

    if let Some(contract_spec) = get_contract_spec_from_category(&contract_category) {
        vec![
            contract_spec.pattern_category.clone(),
            PatternCategory::General,
            PatternCategory::Top,
            PatternCategory::MostObserved,
            PatternCategory::Rare,
            PatternCategory::Frequent,
        ]
    } else {
        default_pattern_categories
    }
}

/// Process pattern analysis: generate, verify, and convert to findings
async fn process_patterns(
    codeblock: &str,
    pattern_categories: Vec<PatternCategory>,
    pattern_discovery_agent: &Arc<AIAgent>,
    finding_discovery_agent: &Arc<AIAgent>,
    ai_verify_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    // check flag
    if SKIP_PATTERN_RUNS {
        return Ok(Findings::default());
    }

    let pattern_prompt = IssuePrompt::Pattern(pattern_categories);

    let findings = if DIRECT_TO_FINDING_MODE {
        info!("PHASE 1-3: GENERATE FINDINGS DIRECT FROM PATTERN");
        pattern_phases::generate_direct_findings::execute(
            pattern_prompt,
            codeblock,
            pattern_discovery_agent,
            repo,
        )
        .await?
    } else {
        // Phase 1: Generate patterns
        info!("PHASE 1: GENERATE PATTERNS");
        let raw_patterns: Patterns = pattern_phases::generate_patterns::execute(
            pattern_prompt,
            codeblock,
            pattern_discovery_agent,
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
        let finding_from_patterns = if !verified_patterns.issues().is_empty() {
            // save patterns to file (by contract)
            save_patterns(&verified_patterns.patterns, repo).await?;

            pattern_phases::multipattern_to_findings::execute(
                verified_patterns,
                codeblock,
                finding_discovery_agent,
                repo,
            )
            .await?
        } else {
            Findings::default()
        };
        finding_from_patterns
    };

    Ok(findings)
}

/// Process ipattern_discovery_config_gemininvariant analysis: generate, verify, and convert to findings
async fn process_invariants(
    codeblock: &str,
    invariant_discovery_agent: &Arc<AIAgent>,
    finding_discovery_agent: &Arc<AIAgent>,
    ai_verify_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    let invariant_prompt = IssuePrompt::Invariant(InvariantType::iter().collect());

    // Phase 1: Generate invariants
    info!("PHASE 1: GENERATE INVARIANTS");
    let mut raw_invariants: ContractInvariants = pattern_phases::generate_patterns::execute(
        invariant_prompt,
        codeblock,
        invariant_discovery_agent,
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
        let findings_from_invariants = pattern_phases::multipattern_to_findings::execute(
            verified_invariants,
            codeblock,
            finding_discovery_agent,
            repo,
        )
        .await?;

        Ok(findings_from_invariants)
    } else {
        Ok(Findings::default())
    }
}

/// Process actor-centric analysis: generate actors, enumerate abuses, verify, and convert to findings
///
/// This function implements the complete actor-centric threat modeling workflow:
/// 1. Generate actors and their capabilities from the contract
/// 2. Enumerate potential abuses for each actor capability
/// 3. Verify that abuses are legitimate security issues
/// 4. Convert verified abuses into detailed security findings
async fn process_actors(
    codeblock: &str,
    actor_abuse_discovery_agent: &Arc<AIAgent>,
    finding_discovery_agent: &Arc<AIAgent>,
    ai_verify_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    // check if actor thread analysis is enabled
    if ACTOR_RUNS == 0 {
        return Ok(Findings::default());
    };

    // custom agent for digging up list of actors
    let actor_discovery_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.2")
        .with_preamble("You are a world-class expert at Solidity EVM smart contract auditing.")
        .with_file_retrieval(false)
        .with_openai_reasoning_effort("high");

    let actor_discovery_agent =
        Arc::new(AgentFactory::create_openai_agent(&actor_discovery_config)?);

    // Phase 1: Generate actors and their capabilities
    info!("PHASE 1: GENERATE ACTORS");
    let actors: Actors =
        pattern_phases::generate_actors::execute(codeblock, &actor_discovery_agent, repo).await?;

    let actor_count = actors.actors.len();
    info!("total of {} Actors found!", actor_count);
    let actor_list = generate_formated_list_from_actor_data(&actors.actors);
    info!("{}", actor_list);

    let actor_prompt = IssuePrompt::Actor(actors.actors);

    // Phase 2: Generate actor abuses (potential exploits for each actor capability)
    info!("PHASE 2: GENERATE ACTOR ABUSES");
    let actor_abuses = if actor_count > 0 {
        pattern_phases::generate_patterns::execute(
            actor_prompt,
            codeblock,
            actor_abuse_discovery_agent,
            repo,
        )
        .await?
    } else {
        ActorAbuses::default()
    };

    // Phase 3: Verify actor abuses are legitimate security issues
    info!("PHASE 3: VERIFY ACTOR ABUSES");
    let verified_abuses = if !actor_abuses.abuses.is_empty() {
        pattern_phases::verify_patterns::verify_actor_abuses(
            actor_abuses,
            codeblock,
            ai_verify_agent,
            repo,
        )
        .await?
    } else {
        ActorAbuses::default()
    };

    let abuses = generate_formatted_actor_abuse_list(&verified_abuses.abuses);
    info!("{}", abuses);

    // Phase 4: Convert verified actor abuses into detailed security findings
    info!("PHASE 4: GENERATE FINDINGS FROM ACTOR ABUSES");
    if !verified_abuses.issues().is_empty() {
        let findings_from_actors = pattern_phases::multipattern_to_findings::execute(
            verified_abuses,
            codeblock,
            finding_discovery_agent,
            repo,
        )
        .await?;

        Ok(findings_from_actors)
    } else {
        Ok(Findings::default())
    }
}
