use crate::cost::cost_data::TokenType;
/// LLM extraction with retry logic and cost tracking.
///
/// This module provides robust LLM interaction utilities with automatic retry
/// mechanisms for handling rate limits, network issues, and parsing errors,
/// while tracking inference costs across different providers.
use crate::cost::cost_data::add_to_inference_cost_by_type;
use crate::llm_review::agent::agent_enums::AgentMetadata;
use crate::llm_review::findings::findings::FromLLMJson;
use rig::agent::Agent;
use rig::completion::CompletionError;
use rig::completion::CompletionModel;
use rig::completion::Prompt;
use rig::completion::PromptError;
use rig::extractor::ExtractionError;
use rig::extractor::Extractor;
use rig::http_client;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde::de::Error as _; // <- bring the trait’s methods into scope
use serde_json::Error as JsonError;
use std::{thread, time::Duration};

/// Maximum retry attempts for failed LLM requests
const MAX_ATTEMPTS: usize = 10;

/// Retries LLM extraction with exponential backoff and cost tracking.
///
/// Handles common LLM API issues including rate limits, network errors,
/// and JSON parsing failures with automatic retry logic.
pub async fn extractor_with_retry<M, T>(
    extractor: &Extractor<M, T>,
    input: &str,
    metadata: &AgentMetadata,
) -> Result<T, ExtractionError>
where
    M: CompletionModel,
    T: JsonSchema + for<'a> Deserialize<'a> + Send + Sync,
{
    let delay = Duration::from_millis(500);

    for attempt in 1..=MAX_ATTEMPTS {
        // NOTE: Input cost is tracked by caller before calling this function
        // Do NOT track input cost here to avoid double-counting

        add_to_inference_cost_by_type(input, &metadata, TokenType::Input).await;
        match extractor.extract(input).await {
            Ok(data) => return Ok(data),
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
    metadata: &AgentMetadata,
) -> Result<T, JsonError>
where
    M: CompletionModel,
    T: DeserializeOwned,
{
    for attempt in 1..=MAX_ATTEMPTS {
        /* ────── 1. ask the model ───────────────────────────────────────── */
        // NOTE: Input cost is tracked by caller before calling this function
        // Do NOT track input cost here to avoid double-counting

        // add cost calc
        add_to_inference_cost_by_type(input, metadata, TokenType::Input).await;
        let raw = match agent.prompt(input).await {
            Ok(txt) => txt,
            // Convert prompt error to JsonError
            Err(e) if should_retry_prompt_err(&e) && attempt < MAX_ATTEMPTS => {
                eprintln!("LLM backend busy ({e}) – retry {attempt}/{MAX_ATTEMPTS}");
                // add to cost (input tokens)
                continue;
            }
            Err(e) => return Err(JsonError::custom(format!("prompt failed: {e}"))),
        };
        // log::info!("json => {:#?}", raw);

        // add to cost (output tokens)
        add_to_inference_cost_by_type(&raw, metadata, TokenType::Output).await;

        /* ────── 2. try to parse JSON ───────────────────────────────────── */
        match FromLLMJson::parse_from_llm_response(&raw) {
            Ok(f) => return Ok(f), // ✅ success
            Err(e) => {
                let msg = e.to_string();

                // Check if we should retry based on the original error
                let should_retry = should_retry_based_on_error(&msg) && attempt < MAX_ATTEMPTS;

                if should_retry {
                    eprintln!("parse error – retrying {attempt}/{MAX_ATTEMPTS}");
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
    const ERR_SUBSTRINGS: &[&str] = &[
        "server_error",
        "server error",
        "status 5", // any 5xx
        "502 bad gateway",
        "error occurred while processing your request",
        "503 service unavailable",
        "504 gateway timeout",
        "429 resource unavailable",
        "too many requests",
        "rate limit",
        "timeout",
        "timed out",
        "connection reset",
        "connection refused",
        "broken pipe",
        "temporarily unavailable",
        "upstream error",
        "unexpected",
        "invalid",
        "syntax",
        "parse",
        "json",
        "deserialize",
    ];

    // if error message contains any of the ERR_SUBSTRINGS then retry
    ERR_SUBSTRINGS
        .iter()
        .any(|needle| error_msg.to_ascii_lowercase().contains(needle))
}

/*──────────────── helper ───────────────────────────────────────────────*/
/// `true`  → retry is warranted  
/// `false` → give up / bubble the error
fn should_retry_prompt_err(e: &PromptError) -> bool {
    match e {
        // Unpack the CompletionError variant
        PromptError::CompletionError(inner) => match inner {
            /* 1) HTTP transport layer issues -------------------------- */
            CompletionError::HttpError(http_err) => match http_err {
                // Status code–based retry logic
                http_client::Error::InvalidStatusCode(code)
                | http_client::Error::InvalidStatusCodeWithMessage(code, _) => {
                    let code = code.as_u16();
                    // 429 Too-Many-Requests or any 5xx server error
                    code == 429 || (500..=599).contains(&code)
                }
                // Network/transport errors from the underlying HTTP client
                http_client::Error::Instance(_) => true,
                _ => false,
            },

            /* 2) Provider said "I'm busy / overloaded / rate-limited"  */
            CompletionError::ProviderError(msg) | CompletionError::ResponseError(msg) => {
                let m = msg.to_lowercase();
                // Transient provider-side issues we should retry
                m.contains("overload")
                    || m.contains("rate limit")
                    || m.contains("busy")
                    || m.contains("try again later")
                    || m.contains("server_error")
                    || m.contains("server error")
                    || m.contains("internal server error")
                    || m.contains("error occurred while processing your request")
                    || m.contains("help.openai.com")
                    || m.contains("empty")  // Gemini sometimes returns empty responses
                    || m.contains("no message") // "Response contained no message or tool call (empty)"
            }

            /* 3) Anything else – usually not transient */
            _ => false,
        },

        /* Tool-call failures, depth-limit, etc. -> *not* transient */
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig::completion::CompletionError;
    use rig::completion::PromptError;

    #[test]
    fn test_should_retry_on_gemini_empty_response() {
        // Test the exact error message from Gemini
        let err = PromptError::CompletionError(CompletionError::ResponseError(
            "Response contained no message or tool call (empty)".to_string(),
        ));
        assert!(
            should_retry_prompt_err(&err),
            "Should retry on Gemini empty response error"
        );
    }

    #[test]
    fn test_should_retry_on_empty_keyword() {
        let err = PromptError::CompletionError(CompletionError::ResponseError(
            "Response is empty".to_string(),
        ));
        assert!(
            should_retry_prompt_err(&err),
            "Should retry on 'empty' keyword"
        );
    }

    #[test]
    fn test_should_retry_on_no_message_keyword() {
        let err = PromptError::CompletionError(CompletionError::ResponseError(
            "No message received from provider".to_string(),
        ));
        assert!(
            should_retry_prompt_err(&err),
            "Should retry on 'no message' keyword"
        );
    }

    #[test]
    fn test_should_retry_on_rate_limit() {
        let err = PromptError::CompletionError(CompletionError::ProviderError(
            "Rate limit exceeded".to_string(),
        ));
        assert!(should_retry_prompt_err(&err), "Should retry on rate limit");
    }

    #[test]
    fn test_should_retry_on_server_error() {
        let err = PromptError::CompletionError(CompletionError::ResponseError(
            "Internal server error".to_string(),
        ));
        assert!(
            should_retry_prompt_err(&err),
            "Should retry on server error"
        );
    }

    #[test]
    fn test_should_not_retry_on_invalid_api_key() {
        let err = PromptError::CompletionError(CompletionError::ResponseError(
            "Invalid API key".to_string(),
        ));
        assert!(
            !should_retry_prompt_err(&err),
            "Should NOT retry on invalid API key"
        );
    }
}
