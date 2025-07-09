use crate::{
    cost::cost_data::{
        add_to_inference_cost_by_agent, add_to_inference_cost_by_type, LlmCostType, TokenType,
    },
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        config::{
            generated_llm_prompt, ContractInvariants, Finding, VulnerabilityQualityCheck,
            CLAUDE_4_0_SONNET,
        },
        context_state::get_metadata_context,
        prompt_content::generate_prompt_for_issue_check,
        prompt_support::{
            post_prompt::POST_PROMPT, post_qualify::POST_QUALIFY, post_verify::POST_VERIFY,
            pre_prompt::PRE_PROMPT, pre_qualify::PRE_QUALIFY, pre_verify::PRE_VERIFY,
            qualify_prompt::QUALIFY_PROMPT, verify_prompt::VERIFY_PROMPT,
        },
        review_utils::{
            build_anthropic_agent, build_deepseek_agent, build_gemini_agent, build_openai_agent,
        },
    },
    master_prompts::{prompt_2x_aa::PROMPT_2X_AA, prompt_2x_bb::PROMPT_2X_BB},
};
use anyhow::Result;
use log::info;
use rig::{
    client::ProviderClient,
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        deepseek::{self, DEEPSEEK_CHAT},
        gemini::{self},
        openai::{self, GPT_4O, O3},
    },
};
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::Mutex;

use super::{
    config::{Findings, LegitVulnerability},
    enums::AIAgent,
};

/// Multi-LLM security analysis orchestration.
///
/// This module coordinates parallel security analysis across multiple LLM providers,
/// implements verification and deduplication workflows, and manages cost tracking.

/// Number of discovery rounds per contract for comprehensive analysis
const DISCOVER_RUNS: usize = 3;
/// Maximum runs for Claude 3.7 Sonnet (cost optimization)
const MAX_CLAUDE_RUNS_3_7: usize = 2;
/// Maximum runs for Claude 4.0 Sonnet (highest cost tier)
const MAX_CLAUDE_RUNS_4_0: usize = 1;

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
) -> Result<(HashMap<String, Findings>, Vec<ContractInvariants>)> {
    let mut all_security_issues = HashMap::<String, Findings>::new();
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    let (ai_verify_agent, ai_discovery_agents) = generate_ai_agents().await?;

    let invariant_findings = Vec::<ContractInvariants>::new();

    let added_context_from_ai_brain = get_metadata_context().await?;

    for (contract, codeblock) in contracts.into_iter() {
        info!("contract => {}", contract);
        info!("codeblock => {}", codeblock);

        let raw_findings = Findings::generate_findings_from_contract_codebase(
            &contract,
            &codeblock,
            &added_context_from_ai_brain,
            &ai_discovery_agents,
        )
        .await?;

        if !raw_findings.findings.is_empty() {
            info!(
                "# of findings BEFORE deduping => {}",
                raw_findings.findings.len()
            );

            let deduped_and_verified_findings = raw_findings
                .dedup_and_verify_with_llm(&codeblock, &ai_verify_agent)
                .await?;

            let quality_checked_and_updated_findings = deduped_and_verified_findings
                .quality_check_with_llm(&codeblock, &ai_verify_agent)
                .await?;

            all_security_issues.insert(contract.to_string(), quality_checked_and_updated_findings);

            // TODO - save issues to Findings db
        }
    }

    // info!("standard security findings => {:#?}", all_security_issues);
    // info!("invariant findings => {:#?}", invariant_findings);
    Ok((all_security_issues, invariant_findings))
}

pub async fn generate_ai_agents() -> anyhow::Result<(Arc<AIAgent>, Vec<Arc<AIAgent>>)> {
    let deepseek_client = deepseek::Client::from_env();
    let gemini_client = gemini::Client::from_env();
    let anthropic_client = anthropic::Client::from_env();
    let openai_client = openai::Client::from_env();

    let added_context_from_ai_brain = get_metadata_context().await?;

    info!("setting up AI agents...");
    // extractors
    let ai_verify_agent = Arc::new(build_openai_agent(
        &openai_client,
        1.0,
        O3,
        "You are SoliditySec-Verifier, a senior smart-contract auditor.",
        Some(&added_context_from_ai_brain),
    ));

    /*
        let openai_agent = Arc::new(build_openai_agent(
            &openai_client,
            1.0,
            GPT_4O,
            "You are a world renowned expert in smart-contract security auditing,
                known for your uncanny ability to find all security bugs in a protocol,
                even the obscure ones.",
            None,
        ));

        let deepseek_agent = Arc::new(build_deepseek_agent(
            &deepseek_client,
            1.0,
            DEEPSEEK_CHAT,
            None,
        ));

        let gemini_agent = Arc::new(build_gemini_agent(
            &gemini_client,
            1.0,
            "gemini-2.5-pro",
            None,
        ));

    */

    // agents
    let mut ai_discovery_agents = Vec::new();
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

    // TODO - unpause once antrhopic credits run out
    // for _ in 0..DISCOVER_RUNS {
    //     ai_discovery_agents.push(gemini_agent.clone());
    // }

    // TODO - use anthropic for testing until credits run out
    for _ in 0..MAX_CLAUDE_RUNS_3_7 {
        ai_discovery_agents.push(anthropic_agent_3_7_t1.clone());
    }
    for _ in 0..MAX_CLAUDE_RUNS_4_0 {
        ai_discovery_agents.push(anthropic_agent_4_0_t1.clone());
    }
    Ok((ai_verify_agent, ai_discovery_agents))
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
    // 1. Build full prompt
    let prompt_header = generated_llm_prompt(&contract_name, instructions, PRE_PROMPT, POST_PROMPT);
    let prompt_body = generate_content_plus_context_block(&code, &added_context);
    let full_prompt = format!("{prompt_header}{prompt_body}");

    // add to cost
    // add_to_inference_cost_by_type(&full_prompt, LlmCostType::GeminiInput).await;
    add_to_inference_cost_by_agent(&full_prompt, &agent, TokenType::Input).await;

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
//
impl Findings {
    pub async fn generate_findings_from_contract_codebase(
        contract: &str,
        code: &str,
        context: &str,
        agents: &Vec<Arc<AIAgent>>,
    ) -> anyhow::Result<Self> {
        let mut handles = vec![];
        let all_findings = Arc::new(Mutex::new(Findings {
            findings: Vec::new(),
        }));
        let contract = Arc::new(contract.to_string());
        let codeblock = Arc::new(code.to_string());
        let added_content_from_brain = Arc::new(context.to_string());

        for (run, arc_agent) in agents.iter().enumerate() {
            for (i, prompt) in [PROMPT_2X_AA, PROMPT_2X_BB].into_iter().enumerate() {
                let agent = Arc::clone(arc_agent);
                let combined_findings = Arc::clone(&all_findings);
                let contract_name = Arc::clone(&contract);
                let code = Arc::clone(&codeblock);
                let added_content = Arc::clone(&added_content_from_brain);

                handles.push(tokio::spawn(async move {
                    if let Err(e) = run_security_prompt(
                        agent,
                        contract_name,
                        code,
                        added_content,
                        prompt,
                        (run + 1) * (i + 1),
                        combined_findings,
                    )
                    .await
                    {
                        log::error!("Prompt task failed: {e:#}");
                    }
                }));
            }
        }

        // Wait for ALL tasks to complete
        for handle in handles {
            handle.await?; // Will error if task panicked
        }

        let findings = all_findings.lock().await;

        if !findings.findings.is_empty() {
            info!(
                "# of findings BEFORE deduping => {}",
                findings.findings.len()
            );
        }
        Ok(findings.clone())
    }

    pub async fn dedup_and_verify_with_llm(
        &self,
        code: &str,
        agent: &Arc<AIAgent>,
    ) -> anyhow::Result<Self> {
        let mut handles = vec![];
        let deduped_findings = Arc::new(self.clone().dedup().await?);
        let codeblock = Arc::new(code.to_string());

        let dedup_finding_count = deduped_findings.findings.len();
        let is_legit_finding_vec: Arc<Mutex<Vec<bool>>> =
            Arc::new(Mutex::new(vec![true; dedup_finding_count]));

        info!("# of findings AFTER deduping => {}", dedup_finding_count);

        info!("now verifying each finding...");

        for i in 0..dedup_finding_count {
            let code = Arc::clone(&codeblock);
            let arc_agent = Arc::clone(agent);
            let arc_findings = Arc::clone(&deduped_findings);
            let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);
            handles.push(tokio::spawn(async move {
                let result: anyhow::Result<()> = async {
                    let instruction_prompt = generate_prompt_for_issue_check(
                        &code,
                        &arc_findings.findings[i],
                        PRE_VERIFY,
                        VERIFY_PROMPT,
                        POST_VERIFY,
                    );

                    // add to cost
                    let context = get_metadata_context().await?;
                    add_to_inference_cost_by_type(
                        &format!("{}{}", instruction_prompt, context),
                        LlmCostType::OpenaiO3Input,
                    )
                    .await;
                    info!("verifying finding #{}", i);
                    let is_legit_struct: LegitVulnerability =
                        arc_agent.extract_with_retry(&instruction_prompt).await?;

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

        Ok(Findings {
            findings: verified_findings,
        })
    }

    pub async fn quality_check_with_llm(
        self,
        code: &str,
        agent: &Arc<AIAgent>,
    ) -> anyhow::Result<Self> {
        let mut handles = vec![];
        let findings = Arc::new(self.clone().dedup().await?);
        let codeblock = Arc::new(code.to_string());

        let finding_count = findings.findings.len();
        // create vec (is_quality_check_passed, updated_finding) for each finding
        // assume all initially pass
        let quality_check_passed_vec: Arc<Mutex<Vec<(bool, Finding)>>> =
            Arc::new(Mutex::new(vec![(true, Finding::default()); finding_count]));

        info!("now quality check on each finding...");

        for i in 0..finding_count {
            let code = Arc::clone(&codeblock);
            let arc_agent = Arc::clone(agent);
            let arc_findings = Arc::clone(&findings);
            let arc_legit_findings_vec = Arc::clone(&quality_check_passed_vec);
            handles.push(tokio::spawn(async move {
                let result: anyhow::Result<()> = async {
                    let prompt = generate_prompt_for_issue_check(
                        &code,
                        &arc_findings.findings[i],
                        PRE_QUALIFY,
                        QUALIFY_PROMPT,
                        POST_QUALIFY,
                    );
                    let context = get_metadata_context().await?;
                    add_to_inference_cost_by_type(
                        &format!("{}{}", prompt, context),
                        LlmCostType::OpenaiO3Input,
                    )
                    .await;
                    info!("quality checking finding #{}", i);
                    let qualify_checked_finding: VulnerabilityQualityCheck =
                        arc_agent.extract_with_retry(&prompt).await?;

                    let quality_check_passed = qualify_checked_finding.is_quality_check_passed;
                    if !quality_check_passed {
                        info!(
                            "{} did not pass quality check => {:?}",
                            arc_findings.findings[i].title(),
                            qualify_checked_finding.where_quality_lacks
                        );
                        info!("{:#?}", &qualify_checked_finding);
                        let updated_finding = Finding {
                            impact: Some(qualify_checked_finding.impact.clone().unwrap_or(
                                arc_findings.findings[i].impact.clone().unwrap_or_default(),
                            )),
                            proof_of_code: Some(
                                qualify_checked_finding.proof_of_code.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .proof_of_code
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            proof_of_concept: Some(
                                qualify_checked_finding.proof_of_concept.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .proof_of_concept
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            mitigation: Some(
                                qualify_checked_finding.mitigation.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .mitigation
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            severity: qualify_checked_finding
                                .severity
                                .unwrap_or(arc_findings.findings[i].severity),
                            ..arc_findings.findings[i].clone()
                        };
                        let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                        legit_findings_vec[i] = (quality_check_passed, updated_finding);
                    } else {
                        let mut quality_checked_findings_vec = arc_legit_findings_vec.lock().await;
                        quality_checked_findings_vec[i] = (true, arc_findings.findings[i].clone());
                    }

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

        let qualify_checked_findings_vec = quality_check_passed_vec.lock().await;
        let qualified_findings: Vec<Finding> = findings
            .as_ref()
            .findings
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                if qualify_checked_findings_vec[idx].0 {
                    // if quality check passes , no changes needed
                    f.clone()
                } else {
                    // if failed submit updated finding
                    qualify_checked_findings_vec[idx].1.clone()
                }
            })
            .collect();

        let num_findings_updated = qualify_checked_findings_vec
            .iter()
            .filter(|(passed, _)| !*passed)
            .count();

        info!(
            "{} Verified Findings with {} updated findings!",
            qualified_findings.len(),
            num_findings_updated
        );

        Ok(Findings {
            findings: qualified_findings,
        })
    }
}
