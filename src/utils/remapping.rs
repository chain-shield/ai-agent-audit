/// Solidity import remapping utilities.
///
/// This module provides global access to Solidity import remappings parsed from
/// remappings.txt files. Remappings are used by Foundry and Hardhat to resolve
/// import paths like `@openzeppelin/contracts/` to actual file system paths.
use anyhow::{Context, Result};
use log::info;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use crate::prepare_code::git_clone::RepoPaths;

/// Global remapping cache.
/// Key: project_id, Value: HashMap of remapping prefix -> target path
/// Example: "@openzeppelin/contracts/" -> "lib/openzeppelin-contracts/contracts/"
static REMAPPING_CACHE: OnceCell<Mutex<HashMap<String, HashMap<String, String>>>> = OnceCell::new();

/// Parse a remappings.txt file and store the mappings in the global cache.
///
/// Remappings.txt format (Foundry/Hardhat):
/// ```text
/// @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/
/// forge-std/=lib/forge-std/src/
/// @ensdomains/=node_modules/@ensdomains/
/// ```
///
/// Only remappings starting with '@' are stored (common convention for external dependencies).
///
/// # Arguments
/// * `remapping_file` - Path to the remappings.txt file
/// * `project_id` - Unique identifier for the project (used as cache key)
///
/// # Returns
/// * `Ok(usize)` - Number of remappings parsed and stored
/// * `Err` - If file cannot be read or parsed
pub fn parse_and_store_remappings(remapping_file: &Path, project_id: &str) -> Result<usize> {
    let content = fs::read_to_string(remapping_file).with_context(|| {
        format!(
            "Failed to read remappings file: {}",
            remapping_file.display()
        )
    })?;

    let mut remappings = HashMap::new();
    let mut count = 0;

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }

        // Parse remapping: "prefix=target" or "prefix/=target/"
        if let Some((prefix, target)) = line.split_once('=') {
            let prefix = prefix.trim();
            let target = target.trim();

            // Only store remappings that start with '@' (common convention)
            if prefix.starts_with('@') {
                info!(
                    "Parsed remapping: {} -> {} (project: {})",
                    prefix, target, project_id
                );
                remappings.insert(prefix.to_string(), target.to_string());
                count += 1;
            }
        }
    }

    // Store in global cache
    let cache = REMAPPING_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cache_lock = cache.lock().unwrap();
    cache_lock.insert(project_id.to_string(), remappings);

    info!(
        "Stored {} remappings for project {} from {}",
        count,
        project_id,
        remapping_file.display()
    );

    Ok(count)
}

/// Get a remapping target for a given prefix.
///
/// # Arguments
/// * `project_id` - Unique identifier for the project
/// * `prefix` - The import prefix to resolve (e.g., "@openzeppelin/contracts/")
///
/// # Returns
/// * `Some(String)` - The target path if a remapping exists
/// * `None` - If no remapping found for this prefix or project
///
/// # Example
/// ```ignore
/// if let Some(target) = get_remapping("my-project", "@openzeppelin/contracts/") {
///     println!("Resolves to: {}", target);
/// }
/// ```
pub fn get_remapping(project_id: &str, prefix: &str) -> Option<String> {
    let cache = REMAPPING_CACHE.get()?;
    let cache_lock = cache.lock().unwrap();
    let project_remappings = cache_lock.get(project_id)?;
    project_remappings.get(prefix).cloned()
}

/// Get all remappings for a project.
///
/// # Arguments
/// * `project_id` - Unique identifier for the project
///
/// # Returns
/// * `Some(HashMap)` - All remappings for the project
/// * `None` - If no remappings found for this project
pub fn get_all_remappings(repo: &RepoPaths) -> Option<HashMap<String, String>> {
    let cache = REMAPPING_CACHE.get()?;
    let cache_lock = cache.lock().unwrap();
    cache_lock.get(&repo.project_id).cloned()
}

/// Resolve an import path using remappings.
///
/// Attempts to resolve a Solidity import path by checking if it starts with
/// any known remapping prefix and replacing it with the target path.
///
/// # Arguments
/// * `project_id` - Unique identifier for the project
/// * `import_path` - The import path from Solidity code (e.g., "@openzeppelin/contracts/token/ERC20/ERC20.sol")
///
/// # Returns
/// * `Some(String)` - The resolved path if a remapping matches
/// * `None` - If no remapping matches this import path
///
/// # Example
/// ```ignore
/// let resolved = resolve_import_path(
///     "my-project",
///     "@openzeppelin/contracts/token/ERC20/ERC20.sol"
/// );
/// // Returns: Some("lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol")
/// ```
pub fn resolve_import_path(import_path: &str, repo: &RepoPaths) -> Option<String> {
    let remappings = get_all_remappings(repo)?;

    // Try to find a matching prefix
    for (prefix, target) in remappings.iter() {
        if import_path.starts_with(prefix) {
            let resolved = import_path.replacen(prefix, target, 1);
            return Some(resolved);
        }
    }

    None
}
