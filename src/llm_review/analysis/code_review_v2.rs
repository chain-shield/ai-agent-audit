use crate::config::{
    OPENAI_MODEL, OPENAI_REASONING_EFFORT, SKIP_ACTOR_PATTERN_RUNS, SKIP_LIBRARIES,
};
use crate::enumerator::codeblock_db::CodeBlocksDb;
use crate::error::{AuditError, Result};
use crate::llm_review::analysis::context_state::{
    generate_multi_modal_context, get_multi_modal_context,
};
use crate::llm_review::analysis::pre_audit_analysis::generate_discovery_agent;
use crate::llm_review::analysis::semaphore::CONTRACT_REVEW_SEM;
use crate::llm_review::contract::contract_file_map::ContractType;
use crate::llm_review::findings::findings::Finding;
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
    // let custom_scoped_contracts = Some(vec![
    //     "GovernorOLAS",
    //     "GovernorTimelockControl",
    //     "BridgeMessenger",
    //     "VerifyBridgedData",
    //     "GuardCM",
    //     "ServiceManager",
    //     "StakingBase",
    //     "RecoveryModule",
    //     "ServiceManagerProxy",
    //     "SafeMultisigWithRecoveryModule",
    //     "Tokenomics",
    //     "LiquidityManagerCore",
    //     "BuyBackBurner",
    //     "DefaultTargetDispenserL2",
    //     "LiquidityManagerProxy",
    // ]);
    let custom_scoped_contracts: Option<Vec<String>> = None;

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

        if let Some(scoped_contracts) = &custom_scoped_contracts
            && !scoped_contracts.contains(&contract)
        {
            info!("contract {} is NOT in custom scope", contract);
            continue;
        }

        if let Some(out_of_scope) = &custom_out_of_scoped_contracts
            && out_of_scope.contains(&contract)
        {
            info!("contract {} is NOT in custom scope", contract);
            continue;
        }

        // check contract in inscope!
        let (is_contract_in_scope, contract_type_option) =
            contract_scope_and_type(&contract, repo).await?;

        let contract_type = contract_type_option.unwrap_or(ContractType::Contract);

        if !is_contract_in_scope {
            info!("{} {} is NOT in scope", contract_type, contract);
            continue;
        }

        if SKIP_LIBRARIES && contract_type == ContractType::Library {
            continue;
        }

        info!("{} {} is in scope", contract_type, contract);

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
                    let verify_findings = phases::verify_rounds::execute_rounds(
                        findings_with_id,
                        &codeblock,
                        &finding_verify_agent,
                        &repo_clone,
                    )
                    .await?;

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
        .with_model(OPENAI_MODEL)
        .with_preamble(verify_preamble)
        .with_file_picker(false) // Disabled to avoid rate limits
        .with_openai_reasoning_effort(OPENAI_REASONING_EFFORT);

    let ai_finding_verify_agent = Arc::new(AgentFactory::create_openai_agent(&verify_config)?);
    // let ai_pattern_verify_agent =
    //     Arc::new(AgentFactory::create_gemini_agent(&verify_config_gemini)?);
    // let finding_ai_verify_agent = Arc::new(AgentFactory::create_anthropic_agent(
    //     &finding_verify_config,
    // )?);

    // Enhanced preamble for discovery agents
    let solidity_auditor_preamble = r#"

    You are a world-class expert at smart contract auditing, renowned for finding the most complex and tricky vulnerabilities in EVM Solidity codebases. You consistently land valid solo High and Medium findings in competitive audit contests. You are an expert at unearthing high value semantic, multi-step, cross-contract, and incentive-based attack paths.

	    ## Exploit guidelines

	    - Severity priority: **Theft > DoS > accounting mismatch**.
	    - Bigger **blast radius** and simpler execution are more valuable.
	    - Assert conditions using `assertGt` / `assertEq`, not just logs.
	    - For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.

	    ## Rules

	    - Only report exploits **directly tied** to the provided list of security vulnerability patterns, **not** unrelated issues.
	    - Only analyze code **actually present** in the codebase. 
	    - Prefer exploits accessible to **unprivileged EOAs**; if an exploit requires a trusted role, make that clear via the `"privilege"` field (as specified in the JSON instructions).
	    - Focus on **present-state** bugs in the current code. Ignore one-time deployment/upgrade windows unless the same condition can be recreated or abused permissionlessly later.
	    - A valid finding must be:
	    - In-scope,
	    - Backed by a credible exploit path,
	    - And clearly Valid finding according to the rubric.
	    - If nothing meets these criteria, return `{{"findings":[]}}`.

	    ### Semantic / multi-step hunting checklist (apply to EACH critical flow): 
	    1. Identify state vars + who can change them between txs. 
	    2. Mark snapshot vs live reads (values cached vs reread later). 
	    3. Enumerate cross-contract edges (external calls, hooks, callbacks, token/oracle/governance modules). 
	    4. For randomness/entropy: test “repeat/correlate/control inputs” scenarios. 
	    5. Incentives/griefing: who profits from delay/failure/DoS? 
	    6. Synthesize 2+ attack sequences (3–6 steps) before concluding “no issue”.

	    ## Example: Incentives / Game Theory (Governance / Griefing DoS)

	    **Pattern:** “Attack is rational because attacker profits from delay/failure; no code bug needed besides an incentive misalignment.”

	    **Setup (typical):**

	    * Protocol has an on-chain action `executeProposal()` / `finalizeEpoch()` / `distributeRewards()` / `rebalance()` that must run for system health.
	    * Anyone *can* call it, but it is **unprofitable** or **costly** to call (gas-heavy, or caller gets slashed / pays).
	    * Meanwhile a subset of users **benefit if it does NOT execute** (e.g., avoid liquidation, keep emissions flowing, block parameter decrease).

	    **How to hunt it:**

	    1. Identify an **“eventual progress” dependency** (something must happen for safety or fairness).
	    2. Check **who pays** to trigger it (gas/penalty) vs **who benefits** if it’s delayed.
	    3. If the beneficiaries can rationally outbid helpers / keepers, it becomes a **credible DoS-by-incentive**.

	    **Exploit hypothesis:**

	    1. Attacker takes a position that becomes bad if `finalize()/execute()` runs (e.g., undercollateralized loan, governance parameter about to tighten).
	    2. Attacker ensures the only way to progress is calling a gas-heavy function with no reward.
	    3. Rational actors don’t call it; attacker can also grief callers (e.g., via MEV/backrun if relevant).
	    4. Protocol remains in stale state; attacker avoids liquidation / keeps emissions / blocks risk reduction.
	    5. Attacker exits position once favorable, or repeats each epoch.

	    **Impact phrasing:**

	    * “Functionally permanent DoS because rational actors are disincentivized to execute the necessary transition.”

	    **Mitigation:**

	    * Add **keeper incentive** (caller reward funded from protocol fees) or make execution **cheap/batched**, and/or add a **fallback** path (permissioned keeper / bounded work per call).

	    ## Example: Semantic (Snapshot vs Live Read / Mid-Flow Parameter Change)

	    **Pattern:** value is read twice across calls/txs; attacker changes it in between.

	    **Exploit hypothesis:**

	    1. User starts flow that assumes `feeRate` / `oraclePrice` / `config` stays constant.
	    2. Before finalization, attacker/admin/MEV changes config or price.
	    3. Final step rereads “live” value and applies it inconsistently → user underpays / over-withdraws / bypasses checks.

	    **Mitigation:** snapshot the value once and reuse, or enforce bounds/time validity.

	    ## Example: Probabilistic (Same Seed / Correlated “Randomness”)

	    **Pattern:** randomness looks fine syntactically but is **correlated/reused**.

	    **Exploit hypothesis:**

	    1. Protocol uses `seed = keccak(blockhash, user, timestamp)` in multiple places or multiple draws.
	    2. Attacker chooses inputs / timing so draws become correlated (or repeats seed).
	    3. Outcomes become predictable / biased → attacker wins raffle/selection more than expected.

	    **Mitigation:** domain-separate draws, use commit-reveal / VRF / include unique nonces per draw.

    "#;
    // let _discovery_config_claude = AgentConfig::new(Some(repo.clone()))
    //     .with_temperature(1.0)
    //     .with_model(CLAUDE_4_5_SONNET)
    //     .with_max_tokens(64_000)
    //     .with_preamble(solidity_auditor_preamble)
    //     .with_file_picker(false) // Disabled to avoid rate limits
    //     .with_file_retrieval(false);

    let pattern_discovery_agent = generate_discovery_agent(repo, solidity_auditor_preamble)?;

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

    let multi_modal_context = get_multi_modal_context(contract, repo).await;
    let multimodal = multi_modal_context.expect("could not unwrap multimodal_context, generate_multi_modal_context(...) must be called first");

    let actors = if !multimodal.actors.actors.is_empty() {
        Some(multimodal.actors)
    } else {
        None
    };

    let invariants = if !multimodal.invariants.invariants.is_empty() {
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
