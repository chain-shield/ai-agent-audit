use anyhow::Result;
use log::info;
use rig::{
    completion::Prompt,
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        openai::{self, O3},
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        config::{generated_llm_prompt, ContractInvariants, LanguageModel, LANGUAGE_MODEL},
        invariants::INVARIANTS,
        prompt_content::generate_context_for_code_review,
        prompt_support::{post_prompt::POST_PROMPT, pre_prompt::PRE_PROMPT},
    },
    utils::{extract_retry::extract_with_retry, logging::print_first_four_lines},
};

use super::config::{Findings, SECURITY_PROMPTS};

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

    info!("setting up AI agent...");
    let openai_agent = openai_client
        .agent(O3)
        .context(&added_context_from_ai_brain)
        .build();
    let openai_extractor = openai_client
        .extractor::<Findings>(O3)
        .preamble("You are an expert smart-contract security auditor.")
        .context(&added_context_from_ai_brain)
        .build();
    let anthropic_extractor = anthropic_client
        .extractor::<Findings>(CLAUDE_3_7_SONNET)
        .preamble("You are an expert smart-contract security auditor.")
        .build();

    let mut invariant_findings = Vec::<ContractInvariants>::new();

    for (contract, codeblock) in contracts.iter() {
        info!("contract => {}", contract);
        info!("codeblock => {}", codeblock);
        let mut security_findings = Findings {
            findings: Vec::new(),
        };

        //SCAN FOR INVARIANTS
        info!("submitting invariant prompt to openai");
        let invariants_response = openai_agent.prompt(INVARIANTS).await?;

        info!("parsing invariant prompt");
        let invariants = ContractInvariants::parse_from_json(&invariants_response)?;

        if !invariants.invariants.is_empty() {
            invariant_findings.push(invariants);
        }

        for security_issue_prompt in SECURITY_PROMPTS {
            // SCAN FOR STANDARD SECURITY ISSUES

            let findings = if LANGUAGE_MODEL == LanguageModel::OpenAI {
                let prompt_string = generate_llm_prompt_for_security_issue(
                    contract,
                    security_issue_prompt,
                    codeblock,
                    "",
                );

                info!("submitting security vulnerability prompt to openai");
                // let findings = openai_extractor.extract(&prompt_string).await?;
                let findings = extract_with_retry(&openai_extractor, &prompt_string).await?;
                // info!("findings for {}:\n{:?}", contract, findings);

                // let findings = Findings::parse_from_json(&findings)?;
                findings
            } else {
                let prompt_string = generate_llm_prompt_for_security_issue(
                    contract,
                    security_issue_prompt,
                    codeblock,
                    &added_context_from_ai_brain,
                );

                info!("submitting security vulnerability prompt to openai");
                // let findings = openai_extractor.extract(&prompt_string).await?;
                let findings = extract_with_retry(&anthropic_extractor, &prompt_string).await?;
                // info!("findings for {}:\n{:?}", contract, findings);

                // let findings = Findings::parse_from_json(&findings)?;
                findings
            };

            if !findings.findings.is_empty() {
                security_findings.findings.extend(findings.findings);
            }
        }

        if !security_findings.findings.is_empty() {
            // TODO (OPTIONAL) - to additional 'open ended' run to see if llm can find any other
            // issues

            // info!("contract findings => {:#?}", security_findings.findings);
            // dedup

            all_security_issues.insert(contract.to_string(), security_findings);

            // TODO - save issues to Findings db
        }
    }

    info!("standard security findings => {:#?}", all_security_issues);
    info!("invariant findings => {:#?}", invariant_findings);
    Ok((all_security_issues, invariant_findings))
}

fn generate_llm_prompt_for_security_issue(
    contract: &str,
    instructions: &str,
    codeblock: &str,
    added_context: &str,
) -> String {
    // let mut prompt_string = generated_llm_prompt(contract, &instructions, PRE_PROMPT, POST_PROMPT);
    let mut prompt_string = generated_llm_prompt(contract, &instructions, "", POST_PROMPT);
    // append constract code to prompt instruction string
    info!("instructions...");
    print_first_four_lines(&prompt_string);

    let codeblock_plus_context = generate_content_plus_context_block(codeblock, added_context);
    // let codeblock_plus_context = codeblock;

    prompt_string.push_str(&codeblock_plus_context);

    prompt_string
}

fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("/n");

    code_plus_context.push_str(codeblock);
    info!("codeblock...");
    print_first_four_lines(&codeblock);

    code_plus_context.push_str("/n");
    code_plus_context.push_str(&added_context);
    info!("added_context_from_ai_brain..");
    print_first_four_lines(&added_context);

    code_plus_context
}
