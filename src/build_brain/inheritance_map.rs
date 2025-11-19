/// Global inheritance map management
///
/// Stores inheritance relationships as (contract_name, file_path) tuples.
/// This naturally handles duplicate contract names across different files.
///
use anyhow::Result;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::{
    enumerator::utils::SolFileType, prepare_code::git_clone::RepoPaths,
    utils::display_file::display_file,
};

/// DO NOT canonicalize paths! On macOS, /tmp is a symlink to /private/tmp,
/// and canonicalization resolves symlinks, causing path mismatches.
/// Instead, just return the path as-is to ensure consistency.
pub fn canonicalize_path(path: &Path) -> PathBuf {
    path.to_path_buf()
}

/// Child → Parents mapping
/// Key: project_id
/// Value: HashMap of (child_contract, child_file) → Vec<(parent_contract, parent_file)>
static INHERITANCE_MAP: Lazy<
    Arc<Mutex<HashMap<String, HashMap<(String, PathBuf), Vec<(String, PathBuf)>>>>>,
> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Parent → Children mapping (inverted)
/// Key: project_id
/// Value: HashMap of (parent_contract, parent_file) → Vec<(child_contract, child_file)>
static INVERTED_INHERITANCE_MAP: Lazy<
    Arc<Mutex<HashMap<String, HashMap<(String, PathBuf), Vec<(String, PathBuf)>>>>>,
> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Insert an inheritance edge: child inherits from parent
///
/// # Arguments
/// * `child` - (contract_name, file_path) tuple for the child contract
/// * `parent` - (contract_name, file_path) tuple for the parent contract
/// * `repo` - Repository paths for project_id
pub async fn insert_inheritance_edge(
    child: (String, PathBuf),
    parent: (String, PathBuf),
    repo: &RepoPaths,
) -> Result<()> {
    let project_id = repo.project_id.clone();

    // Canonicalize paths to ensure consistent lookups
    let child_canonical = (child.0.clone(), canonicalize_path(&child.1));
    let parent_canonical = (parent.0.clone(), canonicalize_path(&parent.1));

    // log::info!(
    //     "📝 Inserting inheritance edge: {} ({}) -> {} ({})",
    //     child_canonical.0,
    //     child_canonical.1.display(),
    //     parent_canonical.0,
    //     parent_canonical.1.display()
    // );
    //
    // Insert into child → parents map
    {
        let mut map = INHERITANCE_MAP.lock().await;
        map.entry(project_id.clone())
            .or_insert_with(HashMap::new)
            .entry(child_canonical.clone())
            .or_insert_with(Vec::new)
            .push(parent_canonical.clone());
    }

    // Insert into parent → children map (inverted)
    {
        let mut inv_map = INVERTED_INHERITANCE_MAP.lock().await;
        inv_map
            .entry(project_id)
            .or_insert_with(HashMap::new)
            .entry(parent_canonical)
            .or_insert_with(Vec::new)
            .push(child_canonical);
    }

    Ok(())
}

/// Get all parents of a contract given its name and file path
///
/// # Arguments
/// * `contract` - Contract name
/// * `file` - File path where the contract is defined
/// * `repo` - Repository paths for project_id
///
/// # Returns
/// Vector of (parent_contract_name, parent_file_path) tuples
pub async fn get_parents_with_file(
    contract: &str,
    file: &Path,
    repo: &RepoPaths,
) -> Result<Vec<(String, PathBuf)>> {
    let map = INHERITANCE_MAP.lock().await;

    // Canonicalize the file path before lookup
    let canonical_file = canonicalize_path(file);

    if let Some(project_map) = map.get(&repo.project_id) {
        if let Some(parents) = project_map.get(&(contract.to_string(), canonical_file.clone())) {
            return Ok(parents.clone());
        }
    } else {
        log::debug!(
            "No inheritance map found for project_id '{}'",
            repo.project_id
        );
    }

    Ok(Vec::new())
}

/// Get all children of a contract given its name and file path
///
/// # Arguments
/// * `contract` - Contract name
/// * `file` - File path where the contract is defined
/// * `repo` - Repository paths for project_id
///
/// # Returns
/// Vector of (child_contract_name, child_file_path) tuples
pub async fn get_children_with_file(
    contract: &str,
    file: &Path,
    repo: &RepoPaths,
) -> Result<Vec<(String, PathBuf)>> {
    let inv_map = INVERTED_INHERITANCE_MAP.lock().await;

    // Canonicalize the file path before lookup
    let canonical_file = canonicalize_path(file);

    if let Some(project_map) = inv_map.get(&repo.project_id) {
        if let Some(children) = project_map.get(&(contract.to_string(), canonical_file.clone())) {
            log::info!(
                "✅ Found {} children for '{}' at '{}'",
                children.len(),
                contract,
                display_file(&canonical_file, repo),
            );
            return Ok(children.clone());
        } else {
            log::info!(
                "No children found for '{}' at '{}'",
                contract,
                display_file(&canonical_file, repo),
            );
        }
    } else {
        log::warn!(
            "❌ No inheritance map found for project_id '{}'",
            repo.project_id
        );
    }

    Ok(Vec::new())
}

/// Resolve a contract name to its file path based on file type
///
/// # Arguments
/// * `contract` - Contract name to resolve
/// * `file_type` - Whether to look in Standard (source) or LibFolder
/// * `repo` - Repository paths
///
/// # Returns
/// Optional file path if contract is found
pub async fn resolve_contract_file(
    contract: &str,
    file_type: SolFileType,
    repo: &RepoPaths,
) -> Result<Option<PathBuf>> {
    use crate::llm_review::contract::contract_file_map::{
        get_file_from_contract, get_file_from_lib_contract,
    };

    match file_type {
        SolFileType::Standard => {
            // Look in source contracts
            if let Some((file, _)) = get_file_from_contract(contract, repo).await {
                Ok(Some(file))
            } else {
                Ok(None)
            }
        }
        SolFileType::LibFolder => {
            // Look in lib contracts
            if let Some((file, _)) = get_file_from_lib_contract(contract, repo).await {
                Ok(Some(file))
            } else {
                Ok(None)
            }
        }
    }
}

/// Get all parents of a contract, automatically resolving the file path
///
/// # Arguments
/// * `contract` - Contract name
/// * `file_type` - Whether to look in Standard (source) or LibFolder
/// * `repo` - Repository paths
///
/// # Returns
/// Vector of parent contract names (without file paths)
pub async fn get_parents(
    contract: &str,
    file_type: SolFileType,
    repo: &RepoPaths,
) -> Result<Vec<(String, PathBuf)>> {
    // Resolve contract to file
    let file = match resolve_contract_file(contract, file_type, repo).await? {
        Some(f) => f,
        None => {
            log::warn!(
                "Contract '{}' not found in {:?} files. Returning empty parents list.",
                contract,
                file_type
            );
            return Ok(Vec::new());
        }
    };

    // Get parents with files
    let parents_with_files = get_parents_with_file(contract, &file, repo).await?;

    // Extract just the contract names
    Ok(parents_with_files)
}

/// Get all children of a contract, automatically resolving the file path
///
/// # Arguments
/// * `contract` - Contract name
/// * `file_type` - Whether to look in Standard (source) or LibFolder
/// * `repo` - Repository paths
///
/// # Returns
/// Vector of child contract names (without file paths)
pub async fn get_children(
    contract: &str,
    file_type: SolFileType,
    repo: &RepoPaths,
) -> Result<Vec<(String, PathBuf)>> {
    // Resolve contract to file
    let file = match resolve_contract_file(contract, file_type, repo).await? {
        Some(f) => f,
        None => {
            log::warn!(
                "Contract '{}' not found in {:?} files. Returning empty children list.",
                contract,
                file_type
            );
            return Ok(Vec::new());
        }
    };

    // Get children with files
    let children_with_files = get_children_with_file(contract, &file, repo).await?;

    // Extract just the contract names
    Ok(children_with_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AuditType;
    use crate::prepare_code::git_clone::PocConfig;

    fn create_test_repo() -> RepoPaths {
        RepoPaths {
            github_url: "https://github.com/test/test".to_string(),
            project_id: "test_project".to_string(),
            root: PathBuf::from("/tmp"),
            sol_files: vec![],
            test_files: vec![],
            script_files: vec![],
            config_files: vec![],
            lib_config_files: vec![],
            source_code_folders: vec![],
            docs: vec![],
            repo_name: "test_repo".to_string(),
            audit_scope: None,
            excluded_folders: None,
            scoped_files: None,
            monorepo_folders: None,
            commit_hash: "1234567890abcdef".to_string(),
            audit_type: AuditType::Client,
            poc: PocConfig {
                test_folder: PathBuf::from("/tmp/pocs"),
                template: String::new(),
                instructions: String::new(),
            },
        }
    }

    #[tokio::test]
    async fn test_insert_and_retrieve_inheritance() {
        let repo = create_test_repo();

        let child = ("ChildContract".to_string(), PathBuf::from("src/Child.sol"));
        let parent = (
            "ParentContract".to_string(),
            PathBuf::from("src/Parent.sol"),
        );

        // Insert edge
        insert_inheritance_edge(child.clone(), parent.clone(), &repo)
            .await
            .unwrap();

        // Retrieve parents
        let parents =
            get_parents_with_file("ChildContract", &PathBuf::from("src/Child.sol"), &repo)
                .await
                .unwrap();

        assert_eq!(parents.len(), 1);
        assert_eq!(parents[0].0, "ParentContract");
        assert_eq!(parents[0].1, PathBuf::from("src/Parent.sol"));

        // Retrieve children
        let children =
            get_children_with_file("ParentContract", &PathBuf::from("src/Parent.sol"), &repo)
                .await
                .unwrap();

        assert_eq!(children.len(), 1);
        assert_eq!(children[0].0, "ChildContract");
        assert_eq!(children[0].1, PathBuf::from("src/Child.sol"));
    }
}
