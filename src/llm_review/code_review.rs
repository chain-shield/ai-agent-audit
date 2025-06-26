use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        config::{
            generated_llm_prompt, ContractInvariants, CLAUDE_4_0_SONNET, INSTRUCTION_PROMPTS,
        },
        invariants::INVARIANTS,
        prompt_content::generate_context_for_code_review,
        prompt_support::{post_prompt::POST_PROMPT, pre_prompt::PRE_PROMPT},
    },
    master_prompts::master_prompt::MASTER_SECURITY_PROMPT,
    utils::extract_retry::agent_extract_with_retry,
};
use anyhow::Result;
use log::info;
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        gemini::{self},
        openai::{self, GPT_4O, O3},
    },
};
use std::sync::Arc;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::sync::Mutex;

use super::config::Findings;

pub enum AIAgent {
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    Openai(Agent<openai::CompletionModel>),
    Gemini(Agent<gemini::completion::CompletionModel>),
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
    // let openai_client = openai::Client::from_env();

    info!("generating additional context for query...");
    let added_context_from_ai_brain =
        generate_context_for_code_review(repo_root, &semantics_path).await?;

    info!("CONTEXT => {:#?}", added_context_from_ai_brain);

    info!("setting up AI agent...");
    let mut ai_agents = Vec::new();
    let anthropic_agent_3_7_t0 = Arc::new(build_anthropic_agent(
        &anthropic_client,
        0.0,
        CLAUDE_3_7_SONNET,
        64_000,
    ));
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
    // ai_agents.push(anthropic_agent_4_0_t1.clone());
    ai_agents.push(anthropic_agent_3_7_t1.clone());
    ai_agents.push(anthropic_agent_4_0_t1.clone());
    // ai_agents.push(anthropic_agent_3_7_t1.clone());
    // ai_agents.push(anthropic_agent_3_7_t1.clone());
    // ai_agents.push(anthropic_agent_4_0_t1);
    ai_agents.push(anthropic_agent_3_7_t1);

    let mut invariant_findings = Vec::<ContractInvariants>::new();

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

        for (run, arc_agent) in ai_agents.iter().enumerate() {
            for (i, instructions) in INSTRUCTION_PROMPTS.into_iter().enumerate() {
                let agent = Arc::clone(arc_agent);
                let combined_findings = Arc::clone(&security_findings);
                let contract_name = Arc::clone(&contract);
                let code = Arc::clone(&codeblock);
                let added_content = Arc::clone(&added_content_from_brain);

                handles.push(tokio::spawn(async move {
                let result: Result<()> = async move {

                    let  prompt_string = generated_llm_prompt(&contract_name, instructions, PRE_PROMPT, POST_PROMPT);

                    let findings = match agent.as_ref() {
                        AIAgent::Anthropic(anthropic_agent) => {
                            let anthropic_prompt = format!("{}{}",prompt_string,generate_content_plus_context_block(&code , &added_content));
                            info!(
                                "------------------------------ROUND #{}: Claude------------------------------",
                                (i+1) * (run + 1)
                            );
                            let anthropic_findings =
                                agent_extract_with_retry(&anthropic_agent, &anthropic_prompt).await?;
                            info!(
                                "found {} issue with claude!",
                                anthropic_findings.findings.len()
                            );
                            anthropic_findings
                        }

                        AIAgent::Openai(openai_agent) => {
                            let openai_prompt = format!("{}{}",prompt_string,generate_content_plus_context_block(&code ,""));

                            info!(
                                "------------------------------ROUND #{}: Openai------------------------------",
                                run + 1
                            );
                            let openai_findings =
                                agent_extract_with_retry(&openai_agent, &openai_prompt).await?;
                            info!(
                                "found {} issue with openai!",
                                openai_findings.findings.len()
                            );
                            openai_findings
                        }

                        AIAgent::Gemini(gemini_agent) => {
                            let gemini_prompt = format!("{}{}",prompt_string,generate_content_plus_context_block(&code ,""));
                            info!(
                                "------------------------------ROUND #{}: Gemini------------------------------",
                                run + 1
                            );
                            let gemini_findings =
                                agent_extract_with_retry(&gemini_agent, &gemini_prompt).await?;
                            info!(
                                "found {} issue with openai!",
                                gemini_findings.findings.len()
                            );
                            gemini_findings
                        }
                    };

                if !findings.findings.is_empty() {
                    let mut finding_lock = combined_findings.lock().await;
                    finding_lock.findings.extend(findings.findings);
                }


            Ok(())
        }
        .await;

                if let Err(e) = result {
                    log::error!("Error processing user: {:#}", e);
                }
            }));
            }
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

        let final_findings = security_findings.lock().await;

        if !final_findings.findings.is_empty() {
            // TODO (OPTIONAL) - to additional 'open ended' run to see if llm can find any other
            // issues

            // info!("contract findings => {:#?}", security_findings.findings);
            // dedup
            info!(
                "# of findings BEFORE deduping => {}",
                final_findings.findings.len()
            );
            let deduped_findings = final_findings.clone().dedup().await?;

            info!(
                "# of findings AFTER deduping => {}",
                deduped_findings.findings.len()
            );
            all_security_issues.insert(contract.to_string(), deduped_findings);

            // TODO - save issues to Findings db
        }
    }

    // info!("standard security findings => {:#?}", all_security_issues);
    // info!("invariant findings => {:#?}", invariant_findings);
    Ok((all_security_issues, invariant_findings))
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

fn build_anthropic_agent(
    client: &anthropic::Client,
    temperature: f64,
    model: &str,
    max_tokens: u64,
) -> AIAgent {
    let agent = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .max_tokens(max_tokens)
        .temperature(temperature)
        .build();

    AIAgent::Anthropic(agent)
}

fn build_openai_agent(
    client: &openai::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Openai(builder.context(added_context).build()),
        None => AIAgent::Openai(builder.build()),
    }
}

fn build_gemini_agent(
    client: &gemini::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Gemini(builder.context(added_context).build()),
        None => AIAgent::Gemini(builder.build()),
    }
}
