use crate::llm_review::config::generated_llm_prompt;
use crate::{
    llm_review::prompt_support::{post_prompt::POST_PROMPT, pre_prompt::PRE_PROMPT},
    utils::extract_retry::agent_extract_with_retry,
};
use anyhow::Result;
use log::info;
use rig::{
    client::CompletionClient,
    providers::{
        anthropic::{self},
        gemini::{self},
        openai::{self},
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{code_review::AIAgent, config::Findings};

pub fn build_anthropic_agent(
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

pub fn build_openai_agent(
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

pub fn build_gemini_agent(
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

// ---------------------------  NEW MODULE-LEVEL HELPER  ---------------------------

/// Executes one LLM-prompt round and merges the returned findings into the shared
/// `Arc<Mutex<Findings>>`.
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
        let findings = match agent.as_ref() {
            AIAgent::Anthropic(model) => {
                info!("---- Claude Round #{}----", idx_of_review_round);
                agent_extract_with_retry(model, &full_prompt).await?
            }
            AIAgent::Openai(model) => {
                info!("---- OpenAI Round #{} ----", idx_of_review_round);
                agent_extract_with_retry(model, &full_prompt).await?
            }
            AIAgent::Gemini(model) => {
                info!("---- Gemini Round #{} ----", idx_of_review_round);
                agent_extract_with_retry(model, &full_prompt).await?
            }
        };

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
