use crate::config::{CREATE_TESTS, SKIP_ACTOR_PATTERN_RUNS, SKIP_LIBRARIES};
use crate::enumerator::codeblock_db::CodeBlocksDb;
use crate::error::{AuditError, Result};
use crate::llm_review::analysis::context_state::{
    generate_multi_modal_context, get_multi_modal_context,
};
use crate::llm_review::analysis::semaphore::CONTRACT_REVEW_SEM;
use crate::llm_review::contract::contract_file_map::ContractType;
use crate::llm_review::findings::findings::{Finding, CLAUDE_4_5_SONNET};
use crate::llm_review::utils::contract_in_scope::contract_scope_and_type;
use crate::llm_review::{agent::agent_enums::AIAgent, phases};
use crate::llm_review::{
    agent::agent_factory::{AgentConfig, AgentFactory},
    analysis::analysis_db::FindingsDb,
    findings::findings::Findings,
    pattern_phases,
    threat_models::{issues::IssuePrompt, pattern_category::PatternCategory},
};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::logging::print_first_n_lines;
use log::info;
use nanoid::nanoid;
use std::{path::PathBuf, sync::Arc};
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
    let custom_scoped_contracts = Some(vec!["Jackpot".to_string()]);
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

    let (_, pattern_discovery_agent, finding_ai_verify_agent, _) = generate_ai_agents(repo).await?;

    let findings_db = Arc::new(Mutex::new(FindingsDb::open()?));

    let mut contract_handles = Vec::new();

    for (contract, (codeblock, _)) in contracts.into_iter() {
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

        // generate list of potential bad actors for this contract and invariants
        // NOTE: below is needed to cache results
        let _ = generate_multi_modal_context(&codeblock, &contract, repo).await?;

        // Clone shared state for the spawned task
        let finding_verify_agent = Arc::clone(&finding_ai_verify_agent);
        let pattern_discovery_agent = Arc::clone(&pattern_discovery_agent);
        let results_db = Arc::clone(&findings_db);
        let all_issues = Arc::clone(&all_security_issues);
        let repo_clone = repo.clone();

        // semaphore
        let sem = Arc::clone(&CONTRACT_REVEW_SEM);

        contract_handles.push(tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async move {
                let raw_findings = process_combined_patterns(
                    &codeblock,
                    &contract,
                    vec![PatternCategory::R1, PatternCategory::R2],
                    &pattern_discovery_agent,
                    &repo_clone,
                )
                .await?;

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
    let solidity_auditor_preamble = r#"You are a world-class expert at smart contract auditing, renowned for finding the most complex and tricky vulnerabilities in EVM Solidity codebases. You consistently land valid solo High and Medium findings in competitive audit contests. You are an expert at unearthing high value semantic, multi-step, cross-contract, and incentive-based attack paths."#;
    // let _discovery_config_claude = AgentConfig::new(Some(repo.clone()))
    //     .with_temperature(1.0)
    //     .with_model(CLAUDE_4_5_SONNET)
    //     .with_max_tokens(64_000)
    //     .with_preamble(solidity_auditor_preamble)
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_file_retrieval(false);

    let pattern_discovery_config_gemini = AgentConfig::new(Some(repo.clone()))
        .with_temperature(1.0)
        .with_top_p(0.95)
        .with_model("gemini-3-pro-preview")
        .with_preamble(solidity_auditor_preamble);

    // let pattern_discovery_config_claude = AgentConfig::new(Some(repo.clone()))
    //     .with_temperature(0.2)
    //     .with_model(CLAUDE_4_5_SONNET)
    //     .with_max_tokens(64_000)
    //     .with_preamble(solidity_auditor_preamble)
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_file_retrieval(false);

    let _pattern_discovery_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.2")
        .with_preamble(solidity_auditor_preamble)
        .with_file_retrieval(false)
        .with_openai_reasoning_effort("high")
        .with_file_picker(false);

    let pattern_discovery_agent = Arc::new(AgentFactory::create_gemini_agent(
        &pattern_discovery_config_gemini,
    )?);

    // let pattern_discovery_agent = Arc::new(AgentFactory::create_anthropic_agent(
    //     &pattern_discovery_config_claude,
    // )?);
    //
    Ok((
        ai_finding_verify_agent.clone(),
        pattern_discovery_agent.clone(),
        ai_finding_verify_agent,
        pattern_discovery_agent,
    ))
}

async fn process_combined_patterns(
    codeblock: &str,
    contract: &str,
    pattern_categories: Vec<PatternCategory>,
    pattern_discovery_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    // check flag
    if SKIP_ACTOR_PATTERN_RUNS {
        return Ok(Findings::default());
    }

    let multi_modal_context = get_multi_modal_context(&contract, repo).await;
    let multimodal = multi_modal_context.expect("could not unwrap multimodal_context, generate_multi_modal_context(...) must be called first");
    print_first_n_lines(30, &multimodal.actors);
    print_first_n_lines(30, &multimodal.invariants);

    let actors = if !multimodal.actors.is_empty() {
        Some(multimodal.actors)
    } else {
        None
    };

    let invariants = if !multimodal.invariants.is_empty() {
        Some(multimodal.invariants)
    } else {
        None
    };

    let pattern_prompt = IssuePrompt::Combined((pattern_categories, actors, invariants));

    info!("PHASE 1-3: GENERATE FINDINGS DIRECT FROM PATTERN");
    let findings = pattern_phases::generate_direct_findings::execute(
        pattern_prompt,
        codeblock,
        pattern_discovery_agent,
        repo,
    )
    .await?;

    Ok(findings)
}
