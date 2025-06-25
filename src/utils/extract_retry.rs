use crate::llm_review::config::Findings;
use rig::agent::Agent;
use rig::completion::CompletionModel;
use rig::completion::Prompt;
use rig::extractor::ExtractionError;
use rig::extractor::Extractor;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::Error as _; // <- bring the trait’s methods into scope
use serde_json::{Error as JsonError, error::Category as JsonCat};
use std::{thread, time::Duration};
use tokio::time::sleep;

const MAX_ATTEMPTS: usize = 3;

/// Retry `extractor.extract(input)` until it succeeds
/// or we exhaust `max_attempts`.
pub async fn extract_with_retry<M, T>(
    extractor: &Extractor<M, T>,
    input: &str,
) -> Result<T, ExtractionError>
where
    M: CompletionModel,
    T: JsonSchema + for<'a> Deserialize<'a> + Send + Sync,
{
    let delay = Duration::from_millis(500);

    for attempt in 1..=MAX_ATTEMPTS {
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
pub async fn agent_extract_with_retry<M>(
    agent: &Agent<M>,
    input: &str,
) -> Result<Findings, JsonError>
where
    M: CompletionModel,
{
    for attempt in 1..=MAX_ATTEMPTS {
        /* ────── 1. ask the model ───────────────────────────────────────── */
        let raw = match agent.prompt(input).await {
            Ok(txt) => txt,
            Err(e) => {
                // Convert prompt error to JsonError
                return Err(JsonError::custom(format!("prompt failed: {e}")));
            }
        };
        // log::info!("json => {:#?}", raw);

        /* ────── 2. try to parse JSON ───────────────────────────────────── */
        match Findings::parse_from_llm_response(&raw) {
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
fn should_retry_parse_error(e: &Box<dyn std::error::Error>) -> bool {
    // Check if it's a JSON parsing error that we should retry
    if let Some(_json_err) = e.downcast_ref::<JsonError>() {
        true // Retry JSON errors
    } else {
        // You can add more specific logic here based on the actual error types
        // that `parse_from_llm_response` returns
        true
    }
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
        || error_msg.contains("overloaded")
    // Add more conditions based on what errors you typically see
}

fn should_retry_json(e: &JsonError) -> bool {
    matches!(e.classify(), JsonCat::Syntax | JsonCat::Data)
}

// Option 3: Create a custom error type that can handle both
#[derive(Debug)]
pub enum ExtractError {
    Json(JsonError),
    Prompt(String),
    Parse(Box<dyn std::error::Error>),
    Exhausted,
}

impl std::fmt::Display for ExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractError::Json(e) => write!(f, "JSON error: {}", e),
            ExtractError::Prompt(e) => write!(f, "Prompt error: {}", e),
            ExtractError::Parse(e) => write!(f, "Parse error: {}", e),
            ExtractError::Exhausted => write!(f, "Exhausted retries"),
        }
    }
}
