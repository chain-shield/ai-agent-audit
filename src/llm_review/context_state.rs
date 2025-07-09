/// Global context state management for AI analysis.
///
/// This module manages shared protocol metadata context that is generated once
/// and reused across all AI agents for consistent analysis. Provides thread-safe
/// access to protocol information including summaries and semantic data.

use once_cell::sync::Lazy;
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

use crate::prepare_code::git_clone::RepoPaths;
use super::prompt_content::generate_context_for_code_review;

/// Global metadata context shared across all AI agents
static METADATA_CONTEXT: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(String::new())));

/// Generates and caches protocol metadata context for AI analysis.
///
/// This function creates comprehensive context information including protocol
/// summaries, contract relationships, and semantic data that is shared across
/// all AI agents for consistent analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_path` - Path to semantic analysis database
pub async fn generate_and_save_metadata_context(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> anyhow::Result<()> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let mut metadata = metadata_context.lock().await;
    let context = generate_context_for_code_review(repo, semantics_path).await?;

    *metadata = context.clone();
    Ok(())
}

/// Retrieves the cached metadata context for AI analysis.
///
/// Returns the protocol metadata context that was previously generated and
/// cached for use across all AI agents.
///
/// # Returns
/// * `String` - Cached protocol metadata context
pub async fn get_metadata_context() -> anyhow::Result<String> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let metadata = metadata_context.lock().await;

    Ok(metadata.clone())
}
