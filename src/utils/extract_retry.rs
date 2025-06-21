use rig::completion::CompletionModel;
use rig::extractor::ExtractionError;
use rig::extractor::Extractor;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{thread, time::Duration};

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
