//! Interface Implementation Detection
//!
//! This module provides O(1) lookup for interface implementations using a global cache.
//!
//! ## Algorithm
//! 1. Scan ALL contracts ONCE and extract their interface parents
//! 2. Build a HashMap: interface_name -> Vec<(contract_name, file_path)>
//! 3. Cache the result per repository
//! 4. All subsequent lookups are O(1)
//!
//! ## Example
//! ```ignore
//! // Scan finds:
//! // - contract CovenantCurator is Ownable2Step, IPriceOracle { ... }
//! // - contract PythOracle is BaseAdapter, IPriceOracle { ... }
//! // - contract MyToken is ERC20, IERC20 { ... }
//! //
//! // Builds index:
//! // {
//! //   "IPriceOracle": [("CovenantCurator", path1), ("PythOracle", path2)],
//! //   "IERC20": [("MyToken", path3)],
//! // }
//! //
//! // Then lookup is O(1):
//! // get_implementations("IPriceOracle") -> [("CovenantCurator", path1), ("PythOracle", path2)]
//! ```

use crate::prepare_code::git_clone::RepoPaths;
use anyhow::Result;
use log::{info, warn};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Mutex;
use tokio::fs;

/// Global cache for interface → implementations mapping.
/// Key: repo_hash
/// Value: HashMap<(interface_name, interface_file_path), Vec<(contract_name, contract_file_path)>>
///
/// Using (interface_name, interface_file_path) as key prevents collisions when:
/// - lib/openzeppelin/interfaces/IERC20.sol
/// - src/interfaces/IERC20.sol
/// Both have the same interface name but different implementations!
///
/// This is built ONCE per repository by scanning all contracts and extracting
/// their interface parents. Then lookups are O(1).
static INTERFACE_IMPLEMENTATIONS_CACHE: Lazy<
    Mutex<HashMap<String, HashMap<(String, PathBuf), Vec<(String, PathBuf)>>>>,
> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Recursively find all interfaces that a contract implements (direct and indirect).
///
/// This function traverses the inheritance hierarchy to find ALL interfaces, including:
/// - Direct implementations: `contract MyToken is IERC20`
/// - Indirect implementations: `contract MyToken is BaseToken` where `BaseToken is IERC20`
///
/// # Example
/// ```ignore
/// // Inheritance hierarchy:
/// // interface IPriceOracle { }
/// // contract BaseAdapter is IPriceOracle { }
/// // contract ChainlinkOracle is BaseAdapter { }
/// //
/// // find_all_interfaces_recursive("ChainlinkOracle", ...) returns:
/// // [("IPriceOracle", "/path/to/IPriceOracle.sol")]
/// ```
fn find_all_interfaces_recursive<'a>(
    contract_name: &'a str,
    contract_file: &'a PathBuf,
    repo: &'a RepoPaths,
    visited: &'a mut HashSet<(String, PathBuf)>,
) -> Pin<Box<dyn Future<Output = Result<Vec<(String, PathBuf)>>> + Send + 'a>> {
    use crate::build_brain::inheritance_map::get_parents_with_file;
    use crate::enumerator::parse_solidity::get_contract_type;
    use crate::llm_review::contract::contract_file_map::ContractType;

    Box::pin(async move {
        let mut interfaces = Vec::new();

        // Prevent infinite recursion
        if visited.contains(&(contract_name.to_string(), contract_file.clone())) {
            return Ok(interfaces);
        }
        visited.insert((contract_name.to_string(), contract_file.clone()));

        // Get direct parents
        let parents = get_parents_with_file(contract_name, contract_file, repo).await?;

        if parents.is_empty() {
            log::debug!(
                "⚠️  No parents found in inheritance map for contract '{}' at '{}'",
                contract_name,
                contract_file.display()
            );
            return Ok(interfaces);
        }

        log::debug!(
            "✅ Found {} parents for contract '{}': {:?}",
            parents.len(),
            contract_name,
            parents.iter().map(|(name, _)| name).collect::<Vec<_>>()
        );

        for (parent_name, parent_file) in parents {
            // Check if this parent is an interface
            if let Some(contract_type) = get_contract_type(&parent_name, &parent_file, repo).await {
                if contract_type == ContractType::Interface {
                    // Found an interface!
                    interfaces.push((parent_name.clone(), parent_file.clone()));
                }
            }

            // Recursively check parent's parents (to find indirect interfaces)
            let parent_interfaces =
                find_all_interfaces_recursive(&parent_name, &parent_file, repo, visited).await?;
            interfaces.extend(parent_interfaces);
        }

        Ok(interfaces)
    })
}

/// Build a complete index of all interface implementations in the repository.
///
/// This function scans ALL Solidity files ONCE and builds a HashMap:
/// - Key: (interface_name, interface_file_path)
/// - Value: Vec of (contract_name, contract_file_path) that implement this interface
///
/// **Uses recursion** to find both direct and indirect implementations:
/// - Direct: `contract MyToken is IERC20`
/// - Indirect: `contract MyToken is BaseToken` where `BaseToken is IERC20`
///
/// This is O(n × d) to build (where d = max inheritance depth) but then O(1) for lookups!
///
/// # Example
/// ```ignore
/// let index = build_interface_implementation_index(&repo).await?;
/// // Returns: {
/// //   ("IPriceOracle", "/path/to/IPriceOracle.sol"): [
/// //     ("CovenantCurator", "/path/to/CovenantCurator.sol"),  // Direct
/// //     ("BaseAdapter", "/path/to/BaseAdapter.sol"),          // Direct
/// //     ("ChainlinkOracle", "/path/to/ChainlinkOracle.sol"),  // Indirect (via BaseAdapter)
/// //     ("PythOracle", "/path/to/PythOracle.sol"),            // Indirect (via BaseAdapter)
/// //   ],
/// // }
/// ```
pub async fn build_interface_implementation_index(
    repo: &RepoPaths,
) -> Result<HashMap<(String, PathBuf), Vec<(String, PathBuf)>>> {
    let mut index: HashMap<(String, PathBuf), Vec<(String, PathBuf)>> = HashMap::new();

    info!("🔍 Building interface implementation index with recursive traversal...");
    info!("   Total .sol files in repo: {}", repo.sol_files.len());

    // Debug: Show first 20 files to see what's in the list
    // info!("   First 20 files in repo.sol_files:");
    // for (i, file) in repo.sol_files.iter().take(20).enumerate() {
    //     info!("     {}. {}", i + 1, file.display());
    // }

    let mut total_contracts = 0;
    let mut total_implementations = 0;
    let mut total_skipped = 0;

    // Scan ALL Solidity files in the repository
    for sol_file in &repo.sol_files {
        // Skip standard libraries (openzeppelin, forge-std), tests, and mocks
        if should_skip_file(sol_file) {
            total_skipped += 1;
            continue;
        }

        // Read the file content
        let content = match fs::read_to_string(sol_file).await {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Log which files we're scanning (first 10 only to avoid spam)
        // if total_contracts < 10 {
        //     info!("   Scanning file: {}", sol_file.display());
        // }

        total_contracts += 1;

        // Extract ALL parent names from contracts in this file
        let implementations = extract_all_interface_implementations(&content);
        // info!(
        //     "implementations for {}: \n {:?}",
        //     sol_file.display(),
        //     implementations
        // );

        for (contract_name, parent_names) in implementations {
            // Use the file path as-is (no canonicalization) to match the format in the inheritance map
            // On macOS, /tmp is a symlink to /private/tmp, and canonicalization would resolve
            // the symlink causing path mismatches
            use crate::build_brain::inheritance_map::canonicalize_path;
            let canonical_sol_file = canonicalize_path(sol_file);

            log::debug!(
                "   🔍 Looking up interfaces for contract '{}' at '{}' with parents: {:?}",
                contract_name,
                sol_file.display(),
                parent_names
            );
            // Recursively find ALL interfaces this contract implements (direct + indirect)
            let mut visited = HashSet::new();
            let interfaces = find_all_interfaces_recursive(
                &contract_name,
                &canonical_sol_file,
                repo,
                &mut visited,
            )
            .await?;

            log::debug!(
                "      Found {} interfaces for '{}': {:?}",
                interfaces.len(),
                contract_name,
                interfaces.iter().map(|(name, _)| name).collect::<Vec<_>>()
            );

            // Add this contract to the index for each interface it implements
            for (interface_name, interface_file) in interfaces {
                log::debug!(
                    "      Adding implementation: {} implements {}",
                    contract_name,
                    interface_name
                );

                index
                    .entry((interface_name.clone(), interface_file.clone()))
                    .or_insert_with(Vec::new)
                    .push((contract_name.clone(), canonical_sol_file.clone()));
                total_implementations += 1;
            }
        }
    }

    info!(
        "📊 Interface implementation index built: {} contracts scanned, {} skipped, {} implementations found across {} unique interfaces",
        total_contracts,
        total_skipped,
        total_implementations,
        index.len()
    );

    Ok(index)
}

/// Build and return the complete interface implementation index for a repository.
///
/// This is a public wrapper for testing and debugging purposes.
/// It builds the index if not cached, or returns the cached version.
///
/// # Arguments
/// * `repo` - Repository paths
///
/// # Returns
/// HashMap of (interface_name, interface_file_path) -> Vec<(contract_name, contract_file_path)>
///
/// # Example
/// ```ignore
/// let index = build_and_get_interface_implementation_index(&repo).await?;
/// for ((interface_name, interface_file), implementations) in index {
///     println!("Interface: {} at {:?}", interface_name, interface_file);
///     for (contract_name, contract_file) in implementations {
///         println!("  - {} at {:?}", contract_name, contract_file);
///     }
/// }
/// ```
pub async fn build_and_get_interface_implementation_index(
    repo: &RepoPaths,
) -> Result<HashMap<(String, PathBuf), Vec<(String, PathBuf)>>> {
    let repo_hash = &repo.project_id;

    // Check if already cached
    {
        let cache = INTERFACE_IMPLEMENTATIONS_CACHE.lock().unwrap();
        if let Some(cached_index) = cache.get(repo_hash) {
            info!("✅ Using cached interface implementation index");
            return Ok(cached_index.clone());
        }
    }

    // Build the index
    let index = build_interface_implementation_index(repo).await?;

    // Cache it
    {
        let mut cache = INTERFACE_IMPLEMENTATIONS_CACHE.lock().unwrap();
        cache.insert(repo_hash.clone(), index.clone());
    }

    Ok(index)
}

/// Find all implementations for multiple interfaces (O(1) lookup after initial build)
///
/// # Arguments
/// * `interfaces_with_files` - Set of (interface_name, interface_file_path) tuples to search for
/// * `repo` - Repository paths
///
/// # Returns
/// HashMap of contract name -> file path for all implementations of any of the interfaces
pub async fn find_implementations_for_interfaces(
    interfaces_with_files: &HashSet<(String, PathBuf)>,
    repo: &RepoPaths,
) -> Result<HashMap<String, PathBuf>> {
    // Use the existing build_and_get function which handles caching
    let index = build_and_get_interface_implementation_index(repo).await?;

    // O(1) lookup for each interface
    let mut all_implementations = HashMap::new();
    for (interface_name, interface_file) in interfaces_with_files {
        let key = (interface_name.clone(), interface_file.clone());
        if let Some(implementations) = index.get(&key) {
            info!(
                "✅ Found {} implementations of {}",
                implementations.len(),
                interface_name,
            );
            for (contract_name, file_path) in implementations {
                all_implementations.insert(contract_name.clone(), file_path.clone());
            }
        } else {
            warn!(
                "❌ No implementations found for interface '{}'",
                interface_name,
            );
        }
    }

    Ok(all_implementations)
}

/// Extract ALL interface implementations from a Solidity file using the shared regex.
///
/// This function uses the same SOLIDITY_REGEXES.inheritance regex that's used throughout
/// the codebase for consistency (DRY principle).
///
/// # Example
/// ```ignore
/// // Input file content:
/// // contract CovenantCurator is Ownable2Step, IPriceOracle {
/// // contract PythOracle is BaseAdapter, IPriceOracle {
/// // contract MyToken is ERC20, IERC20 {
/// //
/// // Returns:
/// // [
/// //   ("CovenantCurator", ["Ownable2Step", "IPriceOracle"]),
/// //   ("PythOracle", ["BaseAdapter", "IPriceOracle"]),
/// //   ("MyToken", ["ERC20", "IERC20"]),
/// // ]
/// ```
///
/// # Arguments
/// * `content` - The Solidity source code
///
/// # Returns
/// Vec of (contract_name, Vec<parent_names>)
///
/// Note: We return ALL parent names, not just interfaces. The filtering to actual interfaces
/// happens in build_interface_implementation_index() using get_parents_with_file() and get_contract_type().
fn extract_all_interface_implementations(content: &str) -> Vec<(String, Vec<String>)> {
    use crate::enumerator::utils::SOLIDITY_REGEXES;

    let mut results = Vec::new();

    // Use the shared inheritance regex from utils.rs (DRY)
    // Regex: r"(?m)^\s*(?:abstract\s+)?(?:contract|interface|library)\s+([A-Za-z_][A-Za-z0-9_]*)\s+is\s+([^{]+)"
    for cap in SOLIDITY_REGEXES.inheritance.captures_iter(content) {
        let contract_name = cap.get(1).unwrap().as_str().to_string();
        let parent_list = cap.get(2).unwrap().as_str();

        // Parse parent list (split by comma, trim whitespace)
        let parents: Vec<String> = parent_list
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if !parents.is_empty() {
            results.push((contract_name, parents));
        }
    }

    results
}

/// OLD FUNCTION - kept for backward compatibility with tests
/// Extract contract name if the file implements the given interface
///
/// Looks for patterns like:
/// - `contract CovenantCurator is Ownable2Step, IPriceOracle {`
/// - `contract MyContract is IPriceOracle {`
/// - `contract Foo is Bar, IPriceOracle, Baz {`
///
/// # Arguments
/// * `content` - The Solidity source code
/// * `interface_name` - The interface name to search for
///
/// # Returns
/// Some(contract_name) if the file implements the interface, None otherwise
#[allow(dead_code)]
fn extracts_contract_implementing_interface(content: &str, interface_name: &str) -> Option<String> {
    // Look for contract/abstract contract declarations
    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
            continue;
        }

        // Look for contract declarations
        if (trimmed.starts_with("contract ") || trimmed.starts_with("abstract contract "))
            && trimmed.contains(" is ")
        {
            // Extract the inheritance list
            // Example: "contract CovenantCurator is Ownable2Step, IPriceOracle {"

            // Get the part after "is"
            if let Some(is_pos) = trimmed.find(" is ") {
                let after_is = &trimmed[is_pos + 4..]; // Skip " is "

                // Get the part before the opening brace
                let inheritance_part = if let Some(brace_pos) = after_is.find('{') {
                    &after_is[..brace_pos]
                } else {
                    after_is
                };

                // Split by commas to get individual parent contracts/interfaces
                let parents: Vec<&str> = inheritance_part.split(',').map(|s| s.trim()).collect();

                // Check if our interface is in the list
                if parents.iter().any(|p| *p == interface_name) {
                    // Extract the contract name
                    // Example: "contract CovenantCurator is ..." -> "CovenantCurator"
                    let contract_part = if trimmed.starts_with("abstract contract ") {
                        &trimmed[18..] // Skip "abstract contract "
                    } else {
                        &trimmed[9..] // Skip "contract "
                    };

                    if let Some(space_pos) = contract_part.find(' ') {
                        let contract_name = contract_part[..space_pos].trim();
                        return Some(contract_name.to_string());
                    }
                }
            }
        }
    }

    None
}

/// Check if a file should be skipped during implementation search
///
/// We skip:
/// - Standard libraries (OpenZeppelin, forge-std, ds-test, erc4626-tests, halmos-cheatcodes, solmate, prb-test)
/// - Files in node_modules/
/// - Files in test/, tests/, or mocks/ folders
/// - Files NOT in source_code_folders (e.g., if source is contracts/, skip files in src/)
///
/// We DO NOT skip:
/// - Project-specific lib folders like euler-price-oracle
/// - Files in interfaces/ directory (they may contain implementations!)
/// - Files in node_modules/ (external libraries may contain implementations, e.g., @flarenetwork/flare-periphery-contracts)
///
/// Note: We used to skip /interfaces/ but that was wrong because:
/// 1. Contracts can implement interfaces in the same file/folder
/// 2. We need to scan ALL contracts to find implementations
/// 3. The interface detection logic already filters out interface definitions
///
/// Note: We used to skip node_modules/ but that was wrong because:
/// 1. External libraries (e.g., @flarenetwork/flare-periphery-contracts) may contain implementations
/// 2. The inheritance map scans node_modules/ via is_library_file()
/// 3. We need consistency between inheritance map and interface implementation index
fn should_skip_file(file: &PathBuf) -> bool {
    use crate::enumerator::parse_solidity::should_exclude_this_library;

    let file_str = file.to_string_lossy();

    // Use the same standard library exclusion logic as the rest of the codebase
    // Note: should_exclude_this_library() already excludes OpenZeppelin, Forge, etc.
    // so we don't need to separately exclude node_modules/
    if should_exclude_this_library(&file_str)
        || file_str.contains("/test/")
        || file_str.contains("/tests/")
        || file_str.contains("/mocks/")
        || file_str.contains("/mock/")
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_contract_implementing_interface() {
        let code = r#"
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            import {IPriceOracle} from "../interfaces/IPriceOracle.sol";
            
            contract CovenantCurator is Ownable2Step, IPriceOracle {
                function getQuote() external view returns (uint256) {
                    return 100;
                }
            }
        "#;

        let result = extracts_contract_implementing_interface(code, "IPriceOracle");
        assert_eq!(result, Some("CovenantCurator".to_string()));
    }

    #[test]
    fn test_extract_contract_not_implementing_interface() {
        let code = r#"
            contract MyContract is Ownable {
                function foo() external {}
            }
        "#;

        let result = extracts_contract_implementing_interface(code, "IPriceOracle");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_abstract_contract_implementing_interface() {
        let code = r#"
            abstract contract BaseOracle is IPriceOracle {
                function getQuote() external view virtual returns (uint256);
            }
        "#;

        let result = extracts_contract_implementing_interface(code, "IPriceOracle");
        assert_eq!(result, Some("BaseOracle".to_string()));
    }

    #[test]
    fn test_extract_contract_multiple_interfaces() {
        let code = r#"
            contract MultiImpl is IERC20, IPriceOracle, IOwnable {
                // implementation
            }
        "#;

        let result = extracts_contract_implementing_interface(code, "IPriceOracle");
        assert_eq!(result, Some("MultiImpl".to_string()));
    }

    #[test]
    fn test_extract_all_interface_implementations() {
        let code = r#"
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;

            contract CovenantCurator is Ownable2Step, IPriceOracle {
                function getQuote() external view returns (uint256) {
                    return 100;
                }
            }

            contract PythOracle is BaseAdapter, IPriceOracle {
                function getQuote() external view returns (uint256) {
                    return 200;
                }
            }

            contract MyToken is ERC20, IERC20 {
                // implementation
            }

            abstract contract BaseOracle is IOracle {
                // abstract implementation
            }
        "#;

        let results = extract_all_interface_implementations(code);

        // Should find 4 contracts with parents
        assert_eq!(results.len(), 4);

        // Check CovenantCurator - returns ALL parents, not just interfaces
        let covenant = results
            .iter()
            .find(|(name, _)| name == "CovenantCurator")
            .unwrap();
        assert_eq!(covenant.1, vec!["Ownable2Step", "IPriceOracle"]);

        // Check PythOracle
        let pyth = results
            .iter()
            .find(|(name, _)| name == "PythOracle")
            .unwrap();
        assert_eq!(pyth.1, vec!["BaseAdapter", "IPriceOracle"]);

        // Check MyToken
        let token = results.iter().find(|(name, _)| name == "MyToken").unwrap();
        assert_eq!(token.1, vec!["ERC20", "IERC20"]);

        // Check BaseOracle
        let base = results
            .iter()
            .find(|(name, _)| name == "BaseOracle")
            .unwrap();
        assert_eq!(base.1, vec!["IOracle"]);
    }

    #[test]
    fn test_extract_all_interface_implementations_no_inheritance() {
        let code = r#"
            contract MyContract {
                // No inheritance
            }
        "#;

        let results = extract_all_interface_implementations(code);

        // Should find no contracts with inheritance
        assert_eq!(results.len(), 0);
    }
}
