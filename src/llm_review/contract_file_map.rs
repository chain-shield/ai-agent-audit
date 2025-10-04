/// Global context state management for contract -> file mapping
///
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::prepare_code::git_clone::RepoPaths;

/// Global metadata context shared across all AI agents
static CONTRACT_TO_FILE: Lazy<Arc<Mutex<HashMap<String, PathBuf>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, PathBuf>::new())));

pub async fn insert_contract_to_file_mapping(
    contract: &str,
    file: &Path,
    repo: &RepoPaths,
) -> anyhow::Result<()> {
    let map = Arc::clone(&CONTRACT_TO_FILE);
    let mut contract_file_map = map.lock().await;

    let key = format!("{}_{}", repo.project_id, contract);

    contract_file_map.insert(key, file.to_owned());

    Ok(())
}

pub async fn get_file_from_contract(contract: &str, repo: &RepoPaths) -> Option<PathBuf> {
    let map = Arc::clone(&CONTRACT_TO_FILE);
    let contract_file_map = map.lock().await;

    let key = format!("{}_{}", repo.project_id, contract);

    // log::info!("getting file for contract {}", contract);
    let file = contract_file_map.get(&key).cloned();

    file
}
