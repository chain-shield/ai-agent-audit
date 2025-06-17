use anyhow::Result;
use log::info;
use rig::{
    completion::Prompt,
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        openai::{self, GPT_4O, O3},
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        config::{generated_llm_prompt, ContractInvariants, DuplicateFindings, Finding},
        invariants::INVARIANTS,
        prompt_content::generate_context_for_code_review,
        prompt_support::{dedup::DEDUP_PROMPT, post_prompt::POST_PROMPT, pre_prompt::PRE_PROMPT},
    },
    utils::logging::print_first_four_lines,
};

use super::config::{Findings, SECURITY_PROMPTS};

pub async fn review_codebase_for_security_issues(
    repo_root: &Path,
    codeblocks_path: &PathBuf,
    semantics_path: &Path,
) -> Result<()> {
    let mut all_security_issues = HashMap::<String, Vec<Finding>>::new();
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    // let deepseek_client = deepseek::Client::from_env();
    // let gemini_client = gemini::Client::from_env();
    let openai_client = openai::Client::from_env();
    let anthropic_client = anthropic::Client::from_env();

    info!("setting up AI agent...");
    // let ai_audit_agent = gemini_client.agent(GEMINI_1_5_PRO).build();
    let openai_agent = openai_client.agent(O3).build();
    // let ai_audit_agent = deepseek_client.agent(DEEPSEEK_CHAT).build();
    let ai_audit_agent = anthropic_client
        .agent(CLAUDE_3_7_SONNET)
        .max_tokens(64_000)
        .temperature(0.8)
        .build();

    info!("generating additional context for query...");
    let added_context_from_ai_brain =
        generate_context_for_code_review(repo_root, &semantics_path).await?;
    let mut invariant_findings = Vec::<ContractInvariants>::new();

    for (contract, codeblock) in contracts.iter() {
        info!("contract => {}", contract);
        info!("codeblock => {}", codeblock);
        let mut security_findings = Vec::<Finding>::new();

        //SCAN FOR INVARIANTS
        let invariant_prompt =
            generate_llm_prompt_for_invariants(codeblock, &added_context_from_ai_brain);
        info!("submitting invariant prompt to openai");
        let invariants_response = openai_agent.prompt(&invariant_prompt).await?;

        info!("parsing invariant prompt");
        let invariants = ContractInvariants::parse_from_json(&invariants_response)?;

        if !invariants.invariants.is_empty() {
            invariant_findings.push(invariants);
        }

        // SCAN FOR STANDARD SECURITY ISSUES
        for instructions_to_find_security_issue in SECURITY_PROMPTS.iter().take(1) {
            let prompt_string = generate_llm_prompt_for_security_issue(
                contract,
                instructions_to_find_security_issue,
                codeblock,
                &added_context_from_ai_brain,
            );

            let security_issues_response = ai_audit_agent.prompt(&prompt_string).await?;
            info!("findings for {}:\n{}", contract, security_issues_response);

            let findings = Findings::parse_from_json(&security_issues_response)?;

            if !findings.findings.is_empty() {
                security_findings.extend(findings.findings);
            }
        }

        //find any dups security issues
        if !security_findings.is_empty() {
            // TODO (OPTIONAL) - to additional 'open ended' run to see if llm can find any other
            // issues

            info!("contract findings => {:#?}", security_findings);
            // dedup
            let contract_findings = remove_duplicate_issues(security_findings).await?;

            all_security_issues.insert(contract.to_string(), contract_findings);

            // TODO - save issues to Findings db
        }
    }

    info!("standard security findings => {:#?}", all_security_issues);
    info!("invariant findings => {:#?}", invariant_findings);
    Ok(())
}

fn generate_llm_prompt_for_security_issue(
    contract: &str,
    instructions: &str,
    codeblock: &str,
    added_context: &str,
) -> String {
    let mut prompt_string = generated_llm_prompt(contract, &instructions, PRE_PROMPT, POST_PROMPT);
    // append constract code to prompt instruction string
    info!("instructions...");
    print_first_four_lines(&prompt_string);

    let codeblock_plus_context = generate_content_plus_context_block(codeblock, added_context);

    prompt_string.push_str(&codeblock_plus_context);

    prompt_string
}

fn generate_llm_prompt_for_invariants(codeblock: &str, added_context: &str) -> String {
    let mut prompt_string = INVARIANTS.to_string();
    // append constract code to prompt instruction string
    info!("instructions...");
    print_first_four_lines(&prompt_string);

    let codeblock_plus_context = generate_content_plus_context_block(codeblock, added_context);

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

async fn remove_duplicate_issues(findings: Vec<Finding>) -> Result<Vec<Finding>> {
    let contract_findings_json = serde_json::to_string(&findings).unwrap();

    let openai_client = openai::Client::from_env();
    let ai_verify_agent = openai_client
        .extractor::<DuplicateFindings>(GPT_4O)
        .preamble(DEDUP_PROMPT)
        .build();

    let duplicate_findings = ai_verify_agent.extract(contract_findings_json).await?;
    info!("dup findings => {:#?}", duplicate_findings);

    let clean_findings = findings
        .into_iter()
        .filter(|f| !duplicate_findings.titles.contains(&f.title))
        .collect();

    Ok(clean_findings)
}
