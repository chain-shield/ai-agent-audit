/// LLM extraction with retry logic and cost tracking.
///
/// This module provides robust LLM interaction utilities with automatic retry
/// mechanisms for handling rate limits, network issues, and parsing errors,
/// while tracking inference costs across different providers.
use crate::cost::cost_data::add_to_inference_cost_by_type;
use crate::cost::cost_data::LlmCostType;
use crate::llm_review::config::FromLLMJson;
use reqwest::StatusCode;
use rig::agent::Agent;
use rig::completion::CompletionError;
use rig::completion::CompletionModel;
use rig::completion::Prompt;
use rig::completion::PromptError;
use rig::extractor::ExtractionError;
use rig::extractor::Extractor;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::de::Error as _; // <- bring the trait’s methods into scope
use serde::Deserialize;
use serde_json::Error as JsonError;
use std::{thread, time::Duration};

/// Maximum retry attempts for failed LLM requests
const MAX_ATTEMPTS: usize = 3;

/// Retries LLM extraction with exponential backoff and cost tracking.
///
/// Handles common LLM API issues including rate limits, network errors,
/// and JSON parsing failures with automatic retry logic.
pub async fn extractor_with_retry<M, T>(
    extractor: &Extractor<M, T>,
    input: &str,
    llm_cost_type: LlmCostType,
) -> Result<T, ExtractionError>
where
    M: CompletionModel,
    T: JsonSchema + for<'a> Deserialize<'a> + Send + Sync,
{
    let delay = Duration::from_millis(500);

    for attempt in 1..=MAX_ATTEMPTS {
        // add to cost
        add_to_inference_cost_by_type(input, llm_cost_type).await;

        match extractor.extract(input).await {
            Ok(data) => return Ok(data), // ✅ parsed JSON
            Err(ExtractionError::NoData) if attempt < MAX_ATTEMPTS => {
                eprintln!("No data extracted – (attempt {attempt}/{MAX_ATTEMPTS})");
                thread::sleep(delay);
            }
            Err(e) => return Err(e), // network / OpenAI errors → bubble up
        }
    }

    Err(ExtractionError::NoData)
}

pub async fn agent_extract_with_retry<M, T>(
    agent: &Agent<M>,
    input: &str,
    llm_cost_type: LlmCostType,
) -> Result<T, JsonError>
where
    M: CompletionModel,
    T: DeserializeOwned,
{
    for attempt in 1..=MAX_ATTEMPTS {
        /* ────── 1. ask the model ───────────────────────────────────────── */
        let raw = match agent.prompt(input).await {
            Ok(txt) => txt,
            // Convert prompt error to JsonError
            Err(e) if should_retry_prompt_err(&e) && attempt < MAX_ATTEMPTS => {
                eprintln!("LLM backend busy ({e}) – retry {attempt}/{MAX_ATTEMPTS}");
                continue;
            }
            Err(e) => return Err(JsonError::custom(format!("prompt failed: {e}"))),
        };
        // log::info!("json => {:#?}", raw);

        // add to cost
        add_to_inference_cost_by_type(&raw, llm_cost_type).await;

        /* ────── 2. try to parse JSON ───────────────────────────────────── */
        match FromLLMJson::parse_from_llm_response(&raw) {
            Ok(f) => return Ok(f), // ✅ success
            Err(e) => {
                let msg = e.to_string();
                // Check if we should retry based on the original error
                let should_retry = should_retry_based_on_error(&msg) && attempt < MAX_ATTEMPTS;

                if should_retry {
                    eprintln!("parse error ({msg}) – retrying {attempt}/{MAX_ATTEMPTS}");
                    // sleep(delay).await; --> NOT Send
                    continue;
                } else {
                    // Convert the error to JsonError and return
                    return Err(JsonError::custom(format!("parse failed: {msg}")));
                }
            }
        }
    }
    // This point is only reached if all attempts exhausted
    Err(JsonError::custom("exhausted retries – still no data"))
}
// Helper function to determine if we should retry based on the original error
fn should_retry_based_on_error(e: &str) -> bool {
    let error_msg = e.to_string().to_lowercase();

    // Retry on common parsing issues that might be fixed by the LLM on retry
    error_msg.contains("unexpected")
        || error_msg.contains("invalid")
        || error_msg.contains("syntax")
        || error_msg.contains("parse")
        || error_msg.contains("json")
        || error_msg.contains("deserialize")
    // Add more conditions based on what errors you typically see
}

/*──────────────── helper ───────────────────────────────────────────────*/
/// `true`  → retry is warranted  
/// `false` → give up / bubble the error
fn should_retry_prompt_err(e: &PromptError) -> bool {
    match e {
        // Unpack the CompletionError variant  ──────────────────────────
        PromptError::CompletionError(inner) => match inner {
            /* 1) HTTP transport layer issues -------------------------- */
            CompletionError::HttpError(http_err) => {
                // 1a) Too-Many-Requests (OpenAI & friends)
                if http_err.status() == Some(StatusCode::TOO_MANY_REQUESTS) {
                    return true;
                }
                // 1b) Any 5xx server error
                if let Some(status) = http_err.status() {
                    if status.is_server_error() {
                        return true;
                    }
                }
                // 1c) Network time-outs
                if http_err.is_timeout() {
                    return true;
                }
                false
            }

            /* 2) Provider said “I’m busy / overloaded / rate-limited”  */
            CompletionError::ProviderError(msg) | CompletionError::ResponseError(msg) => {
                let m = msg.to_lowercase();
                m.contains("overload")
                    || m.contains("rate limit")
                    || m.contains("busy")
                    || m.contains("try again later")
            }

            /* 3) Anything else – usually not transient */
            _ => false,
        },

        /* Tool-call failures, depth-limit, etc. -> *not* transient */
        _ => false,
    }
}
