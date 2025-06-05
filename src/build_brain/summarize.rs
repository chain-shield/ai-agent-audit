use anyhow::Result;
use once_cell::sync::Lazy;
use rig::providers::gemini;
use rig::providers::gemini::completion::GEMINI_1_5_PRO;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use walkdir::WalkDir;

use crate::llm_review::prompt_content;

use super::slither_ffi::cache_key;

/// Global cache keyed by (repo_root, printer) tuple stringified
pub static FILE_SUMMARY_CACHE: Lazy<Arc<Mutex<HashMap<String, Vec<SrcFileSummary>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
#[derive(Debug, Clone)]
pub struct SrcFileSummary {
    pub filename: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileSummary {
    pub summary: Option<String>,
}

pub async fn summarize_src_files(repo_root: &Path) -> Result<Vec<SrcFileSummary>> {
    let key = cache_key(repo_root, "file_summaries");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        return Ok(cached.clone());
    }

    // Initialize vectors to store file paths
    let mut summaries = Vec::<SrcFileSummary>::new();

    let gemini_client = gemini::Client::from_env();

    let context = prompt_content::generate_slither_metadata_prompt_context(repo_root).await?;

    let ai_summary_agent = gemini_client
        .extractor::<FileSummary>(GEMINI_1_5_PRO)
        .preamble("Please provided content, that will be either code or documentation, in 500 words or less. Format in markdown for easy reading.")
        .context(&context)
        .build();

    // Walk through the repository and collect relevant files
    for entry in WalkDir::new(&repo_root).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        // Skip directories
        if !path.is_file() {
            continue;
        }
        let is_readme = path
            .file_name()
            .map(|f| f.to_ascii_lowercase() == "readme.md")
            .unwrap_or(false);

        let is_sol_in_src = path.extension().map_or(false, |ext| ext == "sol")
            && path.starts_with(&repo_root.join("src"));

        if is_readme || is_sol_in_src {
            let content = fs::read_to_string(path)?;
            let summary = ai_summary_agent.extract(content).await?;
            let file_summary = summary.summary.unwrap_or_default();

            // filename is relative to root folder ie src/PuppyRaffle.sol
            let file = path.strip_prefix(repo_root)?.to_string_lossy().to_string();

            summaries.push(SrcFileSummary {
                filename: file,
                summary: file_summary,
            })
        }
    }

    summaries_cache.insert(key, summaries.clone());
    Ok(summaries)
}
