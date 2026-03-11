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

type ContractFileEntry = (PathBuf, ContractType);
type ContractFileMap = HashMap<String, ContractFileEntry>;
type SharedContractFileMap = Arc<Mutex<ContractFileMap>>;

/// Global metadata context shared across all AI agents
static CONTRACT_TO_FILE: Lazy<SharedContractFileMap> =
    Lazy::new(|| Arc::new(Mutex::new(ContractFileMap::new())));

static LIB_CONTRACT_TO_FILE: Lazy<SharedContractFileMap> =
    Lazy::new(|| Arc::new(Mutex::new(ContractFileMap::new())));

#[derive(PartialEq, Eq, Debug, Clone, Copy, strum_macros::Display)]
pub enum ContractType {
    Contract,
    AbstractContract,
    Interface,
    Library,
}

pub async fn insert_contract_to_file_mapping(
    contract: &str,
    file: &Path,
    contract_type: ContractType,
    repo: &RepoPaths,
) -> anyhow::Result<()> {
    let map = Arc::clone(&CONTRACT_TO_FILE);
    let mut contract_file_map = map.lock().await;

    let key = format!("{}_{}", repo.project_id, contract);

    contract_file_map.insert(key, (file.to_owned(), contract_type));

    Ok(())
}

pub async fn get_file_from_contract(
    contract: &str,
    repo: &RepoPaths,
) -> Option<(PathBuf, ContractType)> {
    let map = Arc::clone(&CONTRACT_TO_FILE);
    let contract_file_map = map.lock().await;

    let key = format!("{}_{}", repo.project_id, contract);

    // log::info!("getting file for contract {}", contract);
    contract_file_map.get(&key).cloned()
}

pub async fn insert_lib_contract_to_file_mapping(
    contract: &str,
    file: &Path,
    contract_type: ContractType,
    repo: &RepoPaths,
) -> anyhow::Result<()> {
    let map = Arc::clone(&LIB_CONTRACT_TO_FILE);
    let mut contract_file_map = map.lock().await;

    let key = format!("lib_{}_{}", repo.project_id, contract);

    contract_file_map.insert(key, (file.to_owned(), contract_type));

    Ok(())
}

pub async fn get_file_from_lib_contract(
    contract: &str,
    repo: &RepoPaths,
) -> Option<(PathBuf, ContractType)> {
    let map = Arc::clone(&LIB_CONTRACT_TO_FILE);
    let contract_file_map = map.lock().await;

    let key = format!("lib_{}_{}", repo.project_id, contract);

    // log::info!("getting file for contract {}", contract);
    contract_file_map.get(&key).cloned()
}
