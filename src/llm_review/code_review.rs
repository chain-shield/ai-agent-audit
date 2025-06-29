use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        config::{generated_llm_prompt, ContractInvariants, Finding, CLAUDE_4_0_SONNET},
        invariants::INVARIANTS,
        prompt_content::{generate_context_for_code_review, generate_prompt_for_verifying_issue},
        prompt_support::{post_prompt::POST_PROMPT, pre_prompt::PRE_PROMPT},
        review_utils::{build_anthropic_agent, build_openai_agent},
    },
    master_prompts::{prompt_2x_aa::PROMPT_2X_AA, prompt_2x_bb::PROMPT_2X_BB},
    utils::extract_retry::agent_extract_with_retry,
};
use anyhow::Result;
use log::info;
use rig::{
    agent::Agent,
    client::ProviderClient,
    extractor::Extractor,
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        gemini::{self},
        openai::{self, O3},
    },
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::sync::Mutex;

use super::config::{Findings, LegitVulnerability};

pub enum AIAgent {
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    Openai(Agent<openai::CompletionModel>),
    Gemini(Agent<gemini::completion::CompletionModel>),
}

impl AIAgent {
    pub async fn extract_with_retry<T>(&self, prompt: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic(model) => Ok(agent_extract_with_retry::<_, T>(model, prompt).await?),
            AIAgent::Openai(model) => Ok(agent_extract_with_retry::<_, T>(model, prompt).await?),
            AIAgent::Gemini(model) => Ok(agent_extract_with_retry::<_, T>(model, prompt).await?),
        }
    }
}

pub enum AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    Anthropic(Extractor<anthropic::completion::CompletionModel, T>),
    Openai(Extractor<openai::CompletionModel, T>),
    Gemini(Extractor<gemini::completion::CompletionModel, T>),
}

pub async fn review_codebase_for_security_issues(
    repo_root: &Path,
    codeblocks_path: &PathBuf,
    semantics_path: &Path,
) -> Result<(HashMap<String, Findings>, Vec<ContractInvariants>)> {
    let mut all_security_issues = HashMap::<String, Findings>::new();
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    // let deepseek_client = deepseek::Client::from_env();
    // let gemini_client = gemini::Client::from_env();
    let anthropic_client = anthropic::Client::from_env();
    let openai_client = openai::Client::from_env();

    info!("generating additional context for query...");
    let added_context_from_ai_brain =
        generate_context_for_code_review(repo_root, &semantics_path).await?;

    info!("CONTEXT => {:#?}", added_context_from_ai_brain);

    info!("setting up AI agents...");
    // extractors
    let openai_verify_agent = Arc::new(build_openai_agent(
        &openai_client,
        1.0,
        O3,
        "You are SoliditySec-Verifier, a senior smart-contract auditor.",
        Some(&added_context_from_ai_brain),
    ));

    // agents
    let mut ai_agents = Vec::new();
    let mut ai_agents_critical = Vec::new();
    let anthropic_agent_3_7_t1 = Arc::new(build_anthropic_agent(
        &anthropic_client,
        1.0,
        CLAUDE_3_7_SONNET,
        64_000,
    ));
    let anthropic_agent_4_0_t1 = Arc::new(build_anthropic_agent(
        &anthropic_client,
        1.0,
        CLAUDE_4_0_SONNET,
        64_000,
    ));
    ai_agents.push(anthropic_agent_3_7_t1.clone());
    ai_agents.push(anthropic_agent_4_0_t1.clone());
    ai_agents.push(anthropic_agent_3_7_t1.clone());
    ai_agents.push(anthropic_agent_4_0_t1.clone());
    ai_agents.push(anthropic_agent_3_7_t1.clone());

    ai_agents_critical.push(anthropic_agent_3_7_t1.clone());
    ai_agents_critical.push(anthropic_agent_3_7_t1.clone());
    ai_agents_critical.push(anthropic_agent_3_7_t1.clone());
    ai_agents_critical.push(anthropic_agent_3_7_t1.clone());
    ai_agents_critical.push(anthropic_agent_3_7_t1.clone());
    ai_agents_critical.push(anthropic_agent_4_0_t1.clone());
    ai_agents_critical.push(anthropic_agent_4_0_t1.clone());
    ai_agents_critical.push(anthropic_agent_4_0_t1.clone());
    let invariant_findings = Vec::<ContractInvariants>::new();

    for (contract, codeblock) in contracts.into_iter() {
        info!("contract => {}", contract);
        info!("codeblock => {}", codeblock);
        let mut handles = vec![];
        let security_findings = Arc::new(Mutex::new(Findings {
            findings: Vec::new(),
        }));
        let contract = Arc::new(contract);
        let codeblock = Arc::new(codeblock);
        let added_content_from_brain = Arc::new(added_context_from_ai_brain.clone());

        // round for critical bugs
        for (run, arc_agent) in ai_agents_critical.iter().enumerate() {
            let agent = Arc::clone(arc_agent);
            let combined_findings = Arc::clone(&security_findings);
            let contract_name = Arc::clone(&contract);
            let code = Arc::clone(&codeblock);
            let added_content = Arc::clone(&added_content_from_brain);

            handles.push(tokio::spawn(async move {
                if let Err(e) = run_security_prompt(
                    agent,
                    contract_name,
                    code,
                    added_content,
                    PROMPT_2X_AA,
                    run + 1,
                    combined_findings,
                )
                .await
                {
                    log::error!("Prompt task failed: {e:#}");
                }
            }));
        }

        // round for standard bugs
        for (run, arc_agent) in ai_agents.iter().enumerate() {
            let agent = Arc::clone(arc_agent);
            let combined_findings = Arc::clone(&security_findings);
            let contract_name = Arc::clone(&contract);
            let code = Arc::clone(&codeblock);
            let added_content = Arc::clone(&added_content_from_brain);
            let current_run = ai_agents_critical.len() + run + 1;

            handles.push(tokio::spawn(async move {
                if let Err(e) = run_security_prompt(
                    agent,
                    contract_name,
                    code,
                    added_content,
                    PROMPT_2X_BB,
                    current_run,
                    combined_findings,
                )
                .await
                {
                    log::error!("Prompt task failed: {e:#}");
                }
            }));
        }

        //SCAN FOR INVARIANTS
        // info!("submitting invariant prompt to openai");
        // let invariants_response = openai_agent_1st_pass.prompt(INVARIANTS).await?;
        //
        // info!("parsing invariant prompt");
        // let invariants = ContractInvariants::parse_from_json(&invariants_response)?;
        //
        // if !invariants.invariants.is_empty() {
        //     invariant_findings.push(invariants);
        // }
        // for security_issue in SECURITY_PROMPT_ENUMS {
        // SCAN FOR STANDARD SECURITY ISSUES
        // let findings = if LANGUAGE_MODEL == LanguageModel::OpenAI {
        //     let prompt_string = generate_llm_prompt_for_security_issue(
        //         contract,
        //         &security_issue.prompt(),
        //         codeblock,
        //         "",
        //     );
        //
        //     info!("submitting security vulnerability prompt to openai");
        //
        //     let findings = agent_extract_with_retry(openai_agent, &prompt_string).await?;
        //     findings
        // } else {
        //     let prompt_string = generate_llm_prompt_for_security_issue(
        //         contract,
        //         &security_issue.prompt(),
        //         codeblock,
        //         &added_context_from_ai_brain,
        //     );
        //
        //     info!("submitting security vulnerability prompt to anthropic");
        //     info!(
        //         "-----------------------ROUND #{}-----------------------",
        //         run
        //     );
        //     info!("{} Issue", security_issue.as_fancy_str());
        //     let findings = agent_extract_with_retry(&anthropic_agents[run], &prompt_string).await?;
        //
        //     findings
        // }
        //

        // Wait for ALL tasks to complete
        for handle in handles {
            handle.await?; // Will error if task panicked
        }

        let raw_findings = security_findings.lock().await;

        if !raw_findings.findings.is_empty() {
            // TODO (OPTIONAL) - to additional 'open ended' run to see if llm can find any other
            // issues

            // info!("contract findings => {:#?}", security_findings.findings);
            // dedup
            info!(
                "# of findings BEFORE deduping => {}",
                raw_findings.findings.len()
            );
            let deduped_findings = Arc::new(raw_findings.clone().dedup().await?);
            let mut handles = Vec::new();
            let dedup_finding_count = deduped_findings.findings.len();
            let is_legit_finding_vec: Arc<Mutex<Vec<bool>>> =
                Arc::new(Mutex::new(vec![true; dedup_finding_count]));

            info!("# of findings AFTER deduping => {}", dedup_finding_count);

            info!("now verifying each finding...");

            for i in 0..dedup_finding_count {
                let code = Arc::clone(&codeblock);
                let arc_agent = Arc::clone(&openai_verify_agent);
                let arc_findings = Arc::clone(&deduped_findings);
                let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);
                handles.push(tokio::spawn(async move {
                    let result: anyhow::Result<()> = async {
                        let content =
                            generate_prompt_for_verifying_issue(&code, &arc_findings.findings[i]);
                        info!("verifying finding #{}", i);
                        let is_legit_struct: LegitVulnerability =
                            arc_agent.extract_with_retry(&content).await?;

                        let is_finding_legit = is_legit_struct.is_legit_vulnerability;
                        if !is_finding_legit {
                            info!(
                                "{} is NOT legit => {}",
                                arc_findings.findings[i].title(),
                                is_legit_struct.why_its_not_legit.unwrap_or_default()
                            );
                        }
                        let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                        legit_findings_vec[i] = is_finding_legit;

                        // add
                        Ok(())
                    }
                    .await;

                    if let Err(e) = result {
                        log::error!("Error verifying finding {}: {:?}", i, e);
                    }
                }));
            }

            // optionally await them all
            for h in handles {
                let _ = h.await;
            }

            let legit_findings_vec = is_legit_finding_vec.lock().await;
            let verified_findings: Vec<Finding> = deduped_findings
                .as_ref()
                .findings
                .iter()
                .enumerate()
                .filter(|(idx, _)| legit_findings_vec[*idx])
                .map(|(_, f)| f.clone())
                .collect();

            info!(
                "-------------{} Verified Findings!-----------------",
                verified_findings.len()
            );

            all_security_issues.insert(
                contract.to_string(),
                Findings {
                    findings: verified_findings,
                },
            );

            // TODO - save issues to Findings db
        }
    }

    // info!("standard security findings => {:#?}", all_security_issues);
    // info!("invariant findings => {:#?}", invariant_findings);
    Ok((all_security_issues, invariant_findings))
}

/// Executes one LLM-prompt round and merges the returned findings into the shared `Arc<Mutex<Findings>>`.
///
/// Splitting the logic out of the for-loop keeps the main auditor-loop readable
/// and makes it far easier to unit-test this piece in isolation.
pub async fn run_security_prompt(
    agent: Arc<AIAgent>,
    contract_name: Arc<String>,
    code: Arc<String>,
    added_context: Arc<String>,
    instructions: &'static str,
    idx_of_review_round: usize,
    shared_findings: Arc<Mutex<Findings>>,
) -> Result<()> {
    const FINDINGS_THRESHOLD: usize = 4; // must find 4 issues
    const MAX_RETRY: usize = 3; // if findings less than threshold

    // 1. Build full prompt
    let prompt_header = generated_llm_prompt(&contract_name, instructions, PRE_PROMPT, POST_PROMPT);
    let prompt_body = generate_content_plus_context_block(&code, &added_context);
    let full_prompt = format!("{prompt_header}{prompt_body}");
    let mut get_enough_findings = false;
    let mut retries = 0;

    while !get_enough_findings {
        // 2. Send to the right provider
        info!("----LLM analysis Round #{}----", idx_of_review_round);
        let findings: Findings = agent.extract_with_retry(&full_prompt).await?;

        let issues_found = findings.findings.len();
        info!("{} issues found!", issues_found);

        // 3. Merge results (if any) into the shared accumulator
        if issues_found > 0 {
            let mut guard = shared_findings.lock().await;
            guard.findings.extend(findings.findings);
        }

        // run again if less than 4 findings
        if issues_found >= FINDINGS_THRESHOLD || retries == MAX_RETRY {
            get_enough_findings = true;
        } else {
            retries += 1;
            info!("not enough issues found - retry {}/{}", retries, MAX_RETRY);
        }
    }
    Ok(())
}

fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n");

    code_plus_context.push_str(codeblock);
    // print_first_four_lines(&codeblock);

    code_plus_context.push_str("\n");
    code_plus_context.push_str(&added_context);
    // print_first_four_lines(&added_context);

    code_plus_context
}
