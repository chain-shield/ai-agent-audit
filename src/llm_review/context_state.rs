use once_cell::sync::Lazy;

use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

use super::prompt_content::generate_context_for_code_review;

static METADATA_CONTEXT: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(String::new())));

pub async fn generate_and_save_metadata_context(
    repo_root: &Path,
    semantics_path: &Path,
) -> anyhow::Result<()> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let mut metadata = metadata_context.lock().await;
    let context = generate_context_for_code_review(repo_root, semantics_path).await?;

    *metadata = context.clone();
    Ok(())
}

pub async fn get_metadata_context() -> anyhow::Result<String> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let metadata = metadata_context.lock().await;

    Ok(metadata.clone())
}
