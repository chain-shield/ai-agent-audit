use crate::llm_review::config::Findings;
use rig::agent::Agent;
use rig::completion::CompletionModel;
use rig::completion::Prompt;
use rig::extractor::ExtractionError;
use rig::extractor::Extractor;
use schemars::JsonSchema;
use serde::de::Error as _; // <- bring the trait’s methods into scope
use serde::Deserialize;
use serde_json::{error::Category as JsonCat, Error as JsonError};
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
) -> Result<Findings, serde_json::Error>
where
    M: CompletionModel,
    // T: JsonSchema + for<'a> Deserialize<'a> + Send + Sync,
{
    let delay = Duration::from_millis(1000);
    for attempt in 1..=MAX_ATTEMPTS {
        /* ────── 1. ask the model ───────────────────────────────────────── */
        let raw = match agent.prompt(input).await {
            Ok(txt) => txt,
            Err(e) => {
                // Fabricate a serde-json error to satisfy the function signature
                return Err(JsonError::custom(format!("prompt failed: {e}")));
            }
        };

        log::info!("anthropic json => {:#?}", raw);

        /* ────── 2. try to parse JSON ───────────────────────────────────── */
        match Findings::parse_from_json(&raw) {
            Ok(f) => return Ok(f), // ✅ success
            Err(e) if should_retry_json(&e) && attempt < MAX_ATTEMPTS => {
                eprintln!("parse error ({e}) – retrying {attempt}/{MAX_ATTEMPTS}");
                sleep(delay).await;
                continue;
            }
            Err(e) => return Err(e), // last error
        }
    }

    // This point is only reached if all attempts exhausted with `NoData`
    Err(JsonError::custom("exhausted retries – still no data"))
}

fn should_retry_json(e: &JsonError) -> bool {
    matches!(e.classify(), JsonCat::Syntax | JsonCat::Data)
}
