use anyhow::Result;
use log::info;
use rig::{
    completion::Prompt,
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        gemini::{self, completion::GEMINI_1_5_PRO},
        openai::{self, GPT_4O, GPT_4_TURBO},
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::time::{sleep, Duration};

use crate::{
    ai_bot::agent,
    enumerator::slice_db::CodeBlocksDb,
    llm_review::{
        config::{generated_llm_prompt, DuplicateFindings, Finding},
        prompt_content::generate_context_for_code_review,
        prompt_support::{dedup::DEDUP_PROMPT, post_prompt::POST_PROMPT, pre_prompt::PRE_PROMPT},
    },
    utils::logging::print_first_four_lines,
};

use super::config::{Findings, SECURITY_PROMPTS};

pub async fn review_codebase_for_security_issues(
    repo_root: &Path,
    codeblocks_path: &PathBuf,
) -> Result<()> {
    let mut all_security_issues = HashMap::<String, Vec<Finding>>::new();
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    // let deepseek_client = deepseek::Client::from_env();
    let gemini_client = gemini::Client::from_env();
    let anthropic_client = anthropic::Client::from_env();

    info!("setting up AI extractor...");
    // let ai_audit_agent = gemini_client.agent(GEMINI_1_5_PRO).build();
    // let ai_audit_agent = openai_client.agent(GPT_4O).build();
    // let ai_audit_agent = deepseek_client.agent(DEEPSEEK_CHAT).build();
    let ai_audit_agent = anthropic_client
        .agent(CLAUDE_3_7_SONNET)
        .max_tokens(64_000)
        .temperature(0.8)
        .build();

    for (contract, codeblock) in contracts.iter() {
        info!("contract => {}", contract);
        info!("codeblock => {}", codeblock);
        let mut contract_findings = Vec::<Finding>::new();

        for instructions_to_find_security_issue in SECURITY_PROMPTS.iter().take(5) {
            let mut prompt_string = generated_llm_prompt(
                contract,
                &instructions_to_find_security_issue,
                PRE_PROMPT,
                POST_PROMPT,
            );
            // append constract code to prompt instruction string
            info!("instructions...");
            print_first_four_lines(&prompt_string);

            // prompt_string.push_str(instructions_to_find_security_issue);
            prompt_string.push_str("/n");

            prompt_string.push_str(codeblock);
            info!("codeblock...");
            print_first_four_lines(&codeblock);

            info!("generating additional context for query...");
            // let added_context_from_ai_brain =
            //     agent::get_context_for_security_query(&prompt_string, repo_root).await?;

            let added_context_from_ai_brain = generate_context_for_code_review(repo_root).await?;

            prompt_string.push_str("/n");
            prompt_string.push_str(&added_context_from_ai_brain);
            info!("added_context_from_ai_brain..");
            print_first_four_lines(&added_context_from_ai_brain);

            let issues = ai_audit_agent.prompt(&prompt_string).await?;
            // let findings = issues.findings.clone().unwrap();
            info!("findings for {}:\n{}", contract, issues);

            let findings = Findings::parse_from_json(&issues)?;

            if !findings.findings.is_empty() {
                contract_findings.extend(findings.findings);
            }
            info!("contract findings => {:#?}", contract_findings);
        }

        //find any dups security issues
        if !contract_findings.is_empty() {
            // TODO (OPTIONAL) - to additional 'open ended' run to see if llm can find any other
            // issues

            // dedup
            let contract_findings = remove_duplicate_issues(contract_findings).await?;

            all_security_issues.insert(contract.to_string(), contract_findings);

            // TODO - save issues to Findings db
        }
    }

    info!("all findings => {:#?}", all_security_issues);
    Ok(())
}

async fn remove_duplicate_issues(findings: Vec<Finding>) -> Result<Vec<Finding>> {
    let contract_findings_json = serde_json::to_string(&findings).unwrap();

    let openai_client = openai::Client::from_env();
    let ai_verify_agent = openai_client
        .extractor::<DuplicateFindings>(GPT_4O)
        .preamble(DEDUP_PROMPT)
        .build();

    let duplicate_findings = ai_verify_agent.extract(contract_findings_json).await?;

    let clean_findings = findings
        .into_iter()
        .filter(|f| {
            let title = f.title.clone().unwrap_or_default();
            !duplicate_findings.titles.contains(&title)
        })
        .collect();

    Ok(clean_findings)
}
