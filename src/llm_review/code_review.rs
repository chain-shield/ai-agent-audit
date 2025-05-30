use anyhow::Result;
use log::info;
use rig::{
    completion::Prompt,
    providers::{
        gemini::{self, completion::GEMINI_1_5_PRO},
        openai::{self, GPT_4O},
    },
};
use serde_json::to_string_pretty;
use std::path::{Path, PathBuf};

use crate::{
    ai_bot::agent,
    enumerator::slice_db::CodeBlocksDb,
    llm_review::config::Finding,
    utils::logging::{print_first_four_lines, print_schema},
};

use super::config::{Findings, SECURITY_PROMPTS};

pub async fn review_codebase_for_security_issues(
    repo_root: &Path,
    codeblocks_path: &PathBuf,
) -> Result<()> {
    let mut all_security_issues = Vec::<Finding>::new();
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    // let openai_client = openai::Client::from_env();
    let gemini_client = gemini::Client::from_env();

    info!("setting up AI extractor...");
    let ai_audit_agent = gemini_client.agent(GEMINI_1_5_PRO).build();

    print_schema();

    for instructions_to_find_security_issue in SECURITY_PROMPTS {
        for (contract, codeblock) in contracts.iter() {
            let mut prompt_string = String::new();
            // append constract code to prompt instruction string

            prompt_string.push_str(instructions_to_find_security_issue);
            info!("instructions...");
            print_first_four_lines(&instructions_to_find_security_issue);
            prompt_string.push_str("/n");

            prompt_string.push_str(codeblock);
            info!("codeblock...");
            print_first_four_lines(&codeblock);

            info!("generating additional context for query...");
            let added_context_from_ai_brain =
                agent::get_context_for_security_query(&prompt_string, repo_root).await?;

            prompt_string.push_str("/n");
            prompt_string.push_str(&added_context_from_ai_brain);
            info!("added_context_from_ai_brain..");
            print_first_four_lines(&added_context_from_ai_brain);

            let issues = ai_audit_agent.prompt(&prompt_string).await?;
            // let findings = issues.findings.clone().unwrap();
            let formatted_json = to_string_pretty(&issues)?;
            info!("findings for {}:\n{}", contract, formatted_json);
            // TODO - parse issues
            // all_security_issues.push(issues);
        }

        // TODO - save issues to Findings db
    }

    Ok(())
}
