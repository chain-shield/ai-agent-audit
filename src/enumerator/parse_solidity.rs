use crate::enumerator::interface_implementations::find_implementations_for_interfaces;
use crate::enumerator::utils::SolFileType;
use crate::llm_review::contract::contract_file_map::ContractType;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::check_folder_name::is_library_file;
use crate::utils::remapping::resolve_import_path;
use crate::{
    llm_review::contract::contract_file_map::{get_file_from_contract, get_file_from_lib_contract},
    utils::contract_name_check::contains_contract_reference,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{path::PathBuf, sync::Mutex};
use tokio::fs;

use anyhow::Result;
use log::info;
use std::collections::{HashMap, HashSet};

/// Global cache for source code dependency detection results.
/// Key format: "repo_hash:contract_name"
/// Value: (HashSet<contracts>, HashSet<interfaces>)
static SOURCE_DEPENDENCY_CACHE: Lazy<Mutex<HashMap<String, ImportDependencies>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static CONTRACT_TO_SCRIPT_CACHE: Lazy<Mutex<HashMap<String, HashSet<PathBuf>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Result of parsing all imports from a Solidity source file.
///
/// This struct contains all dependencies extracted from import statements,
/// categorized by type and location.
#[derive(Debug, Clone, Default)]
pub struct ImportDependencies {
    /// Library files (from imports starting with '@')
    /// Example: "@openzeppelin/contracts/token/ERC20/ERC20.sol"
    pub lib_files: HashSet<PathBuf>,

    /// Source files (from relative imports like "./", "../")
    /// Example: "./interfaces/ICovenant.sol"
    pub source_files: HashSet<PathBuf>,

    /// Interfaces with their file paths
    /// Key: Interface name, Value: File path
    /// Example: ("IPriceOracle", "/path/to/IPriceOracle.sol")
    pub interfaces: HashSet<(String, PathBuf)>,

    /// Interface implementations (contracts that implement detected interfaces)
    /// Key: Contract name, Value: File path
    /// Example: ("CovenantCurator", "/path/to/CovenantCurator.sol")
    /// This is populated by searching the entire codebase for implementations
    pub interface_implementations: HashMap<String, PathBuf>,
}

/// Parse all import statements from Solidity source code and extract dependencies.
///
/// This function provides a comprehensive analysis of all imports in a Solidity file,
/// resolving paths and categorizing dependencies by type. It obviates the need for
/// multiple regex searches by parsing all imports in a single pass.
///
/// # Algorithm
/// 1. Parse all import statements (both named and simple imports)
/// 2. For library imports (starting with '@'):
///    - Resolve using remappings
///    - Add to `lib_files` set
/// 3. For each named import {X, Y, Z}:
///    - Check if it's an interface using `get_file_from_contract`
///    - If interface, add to `interfaces` map with file path
/// 4. For non-library import paths:
///    - Resolve relative paths (../, ./)
///    - Use one of the imported names to get correct file path
///    - Add to `source_files` set
///
/// # Arguments
/// * `source_code` - The Solidity source code to parse
/// * `current_file` - The file containing this source code (for relative path resolution)
/// * `repo` - Repository paths for remapping and file resolution
///
/// # Returns
/// * `Ok(ImportDependencies)` - All dependencies categorized by type
/// * `Err` - If file reading or path resolution fails
///
/// # Example
/// ```ignore
/// let deps = parse_all_import_dependencies(&source_code, &file, &repo).await?;
///
/// // Add lib files to context
/// for lib_file in deps.lib_files {
///     context.add_file(lib_file);
/// }
///
/// // Add source files to context
/// for source_file in deps.source_files {
///     context.add_file(source_file);
/// }
///
/// // Find root implementations of interfaces
/// for (interface_name, interface_file) in deps.interfaces {
///     let children = get_children(&interface_name, &interface_file, &repo).await?;
///     // Process children...
/// }
/// ```
pub async fn parse_all_import_dependencies(
    source_code: &str,
    current_file: &PathBuf,
    repo: &RepoPaths,
) -> Result<ImportDependencies> {
    let mut lib_files = HashSet::new();
    let mut source_files = HashSet::new();
    let mut interfaces: HashSet<(String, PathBuf)> = HashSet::new();

    // Regex patterns for parsing imports
    let import_regex = Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*[\"']([^\"']+)[\"']"#).unwrap();
    let import_item_regex =
        Regex::new(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:(?i:as)\s+[A-Za-z_][A-Za-z0-9_]*)?\s*$")
            .unwrap();
    let simple_import_regex = Regex::new(r#"import\s*[\"']([^\"']+)[\"']"#).unwrap();

    // Parse named imports: import { X, Y, Z } from "path"
    for cap in import_regex.captures_iter(source_code) {
        if let Some(imports) = cap.get(1) {
            let import_path = cap.get(2).unwrap().as_str();

            // Collect all imported names from this statement
            let mut imported_names = Vec::new();
            for import in imports.as_str().split(',') {
                let segment = import.trim();
                if segment.is_empty() {
                    continue;
                }

                // Extract the actual contract name (before "as" if aliased)
                let name = if let Some(m) = import_item_regex.captures(segment) {
                    m.get(1).unwrap().as_str()
                } else {
                    segment
                };

                imported_names.push(name.to_string());
            }

            // Handle library imports (starting with '@')
            if import_path.starts_with('@') {
                if let Some(resolved_path) = resolve_import_path(import_path, repo) {
                    if !should_exclude_this_library(&resolved_path) {
                        let full_path = repo.root.join(&repo.repo_name).join(&resolved_path);
                        // DO NOT canonicalize! On macOS, /tmp is a symlink to /private/tmp,
                        // and canonicalization resolves symlinks, causing path mismatches.
                        if full_path.exists() {
                            lib_files.insert(full_path);
                        }
                    }
                }
                // For library imports, also check if any imported names are interfaces
                for name in imported_names {
                    // Check in lib contracts
                    if let Some((file, contract_type)) =
                        get_file_from_lib_contract(&name, repo).await
                    {
                        if contract_type == ContractType::Interface {
                            interfaces.insert((name, file));
                        }
                    }
                }
            } else {
                // Non-library import - resolve the file path
                // Use the first imported name to get the correct file path
                if let Some(first_name) = imported_names.first() {
                    // Try source contracts first
                    if let Some((file, _)) = get_file_from_contract(first_name, repo).await {
                        source_files.insert(file.clone());

                        // Check all imported names for interfaces
                        for name in &imported_names {
                            if let Some((name_file, contract_type)) =
                                get_file_from_contract(name, repo).await
                            {
                                if contract_type == ContractType::Interface {
                                    interfaces.insert((name.clone(), name_file));
                                }
                            }
                        }
                    } else {
                        // Fallback: try to resolve path manually
                        let resolved_path =
                            if import_path.starts_with("../") || import_path.starts_with("./") {
                                if let Some(parent_dir) = current_file.parent() {
                                    parent_dir.join(import_path)
                                } else {
                                    PathBuf::from(import_path)
                                }
                            } else {
                                repo.root.join(&repo.repo_name).join(import_path)
                            };

                        // DO NOT canonicalize! On macOS, /tmp is a symlink to /private/tmp,
                        // and canonicalization resolves symlinks, causing path mismatches.
                        if resolved_path.exists() {
                            source_files.insert(resolved_path);
                        }
                    }
                }
            }
        }
    }

    // Parse simple imports: import "path";
    for cap in simple_import_regex.captures_iter(source_code) {
        let import_path = cap.get(1).unwrap().as_str();

        if import_path.starts_with('@') {
            // Library import
            if let Some(resolved_path) = resolve_import_path(import_path, repo) {
                if !should_exclude_this_library(&resolved_path) {
                    let full_path = repo.root.join(&repo.repo_name).join(&resolved_path);
                    // DO NOT canonicalize! On macOS, /tmp is a symlink to /private/tmp,
                    // and canonicalization resolves symlinks, causing path mismatches.
                    if full_path.exists() {
                        lib_files.insert(full_path);
                    }
                }
            }
        } else {
            // Source import - resolve path
            let resolved_path = if import_path.starts_with("../") || import_path.starts_with("./") {
                if let Some(parent_dir) = current_file.parent() {
                    parent_dir.join(import_path)
                } else {
                    PathBuf::from(import_path)
                }
            } else {
                repo.root.join(&repo.repo_name).join(import_path)
            };

            // DO NOT canonicalize! On macOS, /tmp is a symlink to /private/tmp,
            // and canonicalization resolves symlinks, causing path mismatches.
            if resolved_path.exists() {
                source_files.insert(resolved_path);
            }
        }
    }

    Ok(ImportDependencies {
        lib_files,
        source_files,
        interfaces,
        interface_implementations: HashMap::new(), // Will be populated by detect_source_code_dependencies
    })
}

/// Detect source code dependencies from a file path (without caching).
///
/// This is a lower-level function that works directly with file paths.
/// Use this when you already have the file path and want to analyze its dependencies.
/// This function does NOT use caching and does NOT recursively analyze transitive dependencies.
///
/// # Arguments
/// * `file` - The file path to analyze
/// * `repo` - Repository paths
///
/// # Returns
/// * `ImportDependencies` - The direct dependencies found in the file
async fn detect_dependencies_from_file_internal(
    file: &PathBuf,
    repo: &RepoPaths,
) -> Result<ImportDependencies> {
    // Skip excluded libraries (OpenZeppelin, forge-std, etc.)
    if let Some(file_str) = file.to_str() {
        if should_exclude_this_library(file_str) {
            return Ok(ImportDependencies::default());
        }
    }

    // Read the source code
    let source_code = match fs::read_to_string(file).await {
        Ok(content) => content,
        Err(_) => {
            return Ok(ImportDependencies::default());
        }
    };

    // Parse imports from this file
    parse_all_import_dependencies(&source_code, file, repo).await
}

/// # Returns
/// * `HashSet<String>` - contracts & interfaces found in source
pub async fn detect_source_code_dependencies(
    contract: &str,
    repo: &RepoPaths,
) -> Result<ImportDependencies> {
    // info!("SCANNING for SOURCE CODE DEPENDENCIES");
    // Create cache key: "repo_hash:contract_name"
    let cache_key = format!("{}:{}", repo.unique_repo_hash(), contract);

    // Check cache first
    {
        let cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
        if let Some(cached_result) = cache.get(&cache_key) {
            // info!(
            //     "cache hit for source dependencies of contract {} (repo: {})",
            //     contract,
            //     repo.unique_repo_hash()
            // );
            return Ok(cached_result.clone());
        }
    }

    info!(
        "cache miss for source dependencies of contract {} (repo: {}), analyzing...",
        contract,
        repo.unique_repo_hash()
    );

    // Get the source file for this contract
    // Try source files first, then library files
    let (file, _) = match get_file_from_contract(contract, repo).await {
        Some(f) => f,
        None => {
            // Try library files
            match get_file_from_lib_contract(contract, repo).await {
                Some(f) => f,
                None => {
                    info!(
                        "could not find file for contract {} to detect dependencies (checked both source and lib files)",
                        contract
                    );
                    // Cache the empty result to avoid re-attempting failed lookups
                    let mut cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
                    cache.insert(cache_key, ImportDependencies::default());
                    return Ok(ImportDependencies::default());
                }
            }
        }
    };

    // Skip excluded libraries (OpenZeppelin, forge-std, etc.)
    if let Some(file_str) = file.to_str() {
        if should_exclude_this_library(file_str) {
            info!(
                "skipping excluded library contract {} at {}",
                contract,
                file.display()
            );
            // Cache the empty result to avoid re-processing excluded libraries
            let mut cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
            cache.insert(cache_key, ImportDependencies::default());
            return Ok(ImportDependencies::default());
        }
    }

    // Read the source code
    let source_code = match fs::read_to_string(&file).await {
        Ok(content) => content,
        Err(e) => {
            info!(
                "could not read file {} for dependency detection: {}",
                file.display(),
                e
            );
            return Ok(ImportDependencies::default());
        }
    };

    let mut import_deps = parse_all_import_dependencies(&source_code, &file, repo).await?;

    // TRANSITIVE DEPENDENCIES: Analyze imports of imported source files (1 level deep)
    // This catches cases like: SynthToken imports ISynthToken, which imports ICovenant
    // We need ICovenant to understand the types used in ISynthToken
    let mut transitive_source_files = HashSet::new();
    let mut transitive_lib_files = HashSet::new();
    let mut transitive_interfaces = HashSet::new();

    // Analyze imports of source files
    for source_file in &import_deps.source_files {
        let transitive_deps = detect_dependencies_from_file_internal(source_file, repo).await?;
        transitive_source_files.extend(transitive_deps.source_files);
        transitive_lib_files.extend(transitive_deps.lib_files);
        transitive_interfaces.extend(transitive_deps.interfaces);
    }

    // Analyze imports of interfaces (they can import types, other interfaces, etc.)
    for (_, interface_file) in &import_deps.interfaces {
        let transitive_deps = detect_dependencies_from_file_internal(interface_file, repo).await?;
        transitive_source_files.extend(transitive_deps.source_files);
        transitive_lib_files.extend(transitive_deps.lib_files);
        transitive_interfaces.extend(transitive_deps.interfaces);
    }

    // Add transitive dependencies to the main import_deps
    import_deps.source_files.extend(transitive_source_files);
    import_deps.lib_files.extend(transitive_lib_files);
    import_deps.interfaces.extend(transitive_interfaces);

    // INTERFACE IMPLEMENTATIONS: Find ALL contracts that implement detected interfaces
    // This is critical for security analysis - when code uses an interface (e.g., IPriceOracle),
    // we need to analyze ALL possible implementations (e.g., CovenantCurator, PythOracle, ChainlinkOracle)
    // because any of them could be used at runtime (runtime polymorphism)
    info!(
        "🔍 Searching for implementations of {} interfaces",
        import_deps.interfaces.len()
    );

    // Pass the full (interface_name, interface_file_path) tuples for accurate lookup
    // This prevents collisions when multiple interfaces have the same name in different directories
    let implementations =
        find_implementations_for_interfaces(&import_deps.interfaces, repo).await?;

    info!(
        "📊 Found {} total implementations across all interfaces",
        implementations.len()
    );

    // SECOND LEVEL TRANSITIVE: Analyze imports of interface implementations
    // This catches cases like: Covenant uses ILiquidExchangeModel → LatentSwapLEX implements it → LatentSwapLEX uses IPriceOracle
    // We need to find ALL implementations of IPriceOracle (CovenantCurator, ChainlinkOracle, PythOracle)
    info!(
        "🔍 Analyzing transitive dependencies of {} interface implementations",
        implementations.len()
    );

    let mut implementation_interfaces = HashSet::new();
    for (_, impl_file) in &implementations {
        let impl_deps = detect_dependencies_from_file_internal(impl_file, repo).await?;
        implementation_interfaces.extend(impl_deps.interfaces);
    }

    import_deps.interface_implementations = implementations;

    // Find implementations of these newly discovered interfaces
    if !implementation_interfaces.is_empty() {
        info!(
            "🔍 Found {} additional interfaces from implementation analysis",
            implementation_interfaces.len()
        );

        let additional_implementations =
            find_implementations_for_interfaces(&implementation_interfaces, repo).await?;

        info!(
            "📊 Found {} additional implementations from transitive analysis",
            additional_implementations.len()
        );

        // Merge with existing implementations
        import_deps
            .interface_implementations
            .extend(additional_implementations);
    }

    info!(
        "cached source dependencies for contract {} (repo: {}): {} lib files, {} src files,\n 
            {} interfaces + child contracts, {} interface implimentations",
        contract,
        repo.unique_repo_hash(),
        import_deps.lib_files.len(),
        import_deps.source_files.len(),
        import_deps.interfaces.len(),
        import_deps.interface_implementations.len()
    );

    // Cache the result before returning
    {
        let mut cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
        cache.insert(cache_key, import_deps.clone());
    }

    Ok(import_deps)
}

pub async fn get_contract_type(
    contract: &str,
    file: &PathBuf,
    repo: &RepoPaths,
) -> Option<ContractType> {
    let file_type = get_file_type(file, repo);

    let contract_info_option = match file_type {
        SolFileType::Standard => get_file_from_contract(contract, repo).await,
        SolFileType::LibFolder => get_file_from_lib_contract(contract, repo).await,
    };

    match contract_info_option {
        Some((_, contract_type)) => Some(contract_type),
        None => None,
    }
}

pub fn get_file_type(file: &PathBuf, repo: &RepoPaths) -> SolFileType {
    let file_type = if is_library_file(file, repo) {
        SolFileType::LibFolder
    } else {
        SolFileType::Standard
    };

    file_type
}
pub async fn detect_scripts_connected_to_contract(
    contract: &str,
    repo: &RepoPaths,
) -> Result<HashSet<PathBuf>> {
    // Create cache key: "repo_hash:contract_name"
    let cache_key = format!("{}:{}-deploy-script", repo.unique_repo_hash(), contract);

    // Check cache first
    {
        let cache = CONTRACT_TO_SCRIPT_CACHE.lock().unwrap();
        if let Some(cached_result) = cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }
    }

    let mut scripts = HashSet::new();

    for script in &repo.script_files {
        // Read the source code
        let source_code = match fs::read_to_string(script).await {
            Ok(content) => content,
            Err(e) => {
                info!(
                    "could not read file {} for dependency detection: {}",
                    script.display(),
                    e
                );
                continue;
            }
        };

        // Strip comments and string literals to avoid false positives (e.g., "XOR (^)" in comments)
        let source_code = strip_comments_and_strings(&source_code);

        if contains_contract_reference(contract, &source_code) {
            scripts.insert(script.clone());
        }
    }

    // Cache the result before returning
    {
        let mut cache = CONTRACT_TO_SCRIPT_CACHE.lock().unwrap();
        cache.insert(cache_key, scripts.clone());
    }

    Ok(scripts)
}

fn strip_comments_and_strings(src: &str) -> String {
    // Compile-once regexes
    static RE_BLOCK: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)/\*.*?\*/").unwrap());
    static RE_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)//[^\n]*").unwrap());
    static RE_DQ: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?s)\"(?:\\.|[^\"\\])*\""#).unwrap());
    static RE_SQ: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)'(?:\\.|[^'\\])*'").unwrap());

    // Remove strings first so comment markers inside strings are ignored
    let mut out = RE_DQ.replace_all(src, "\"\"").into_owned();
    out = RE_SQ.replace_all(&out, "''").into_owned();

    // Remove block and line comments
    out = RE_BLOCK.replace_all(&out, " ").into_owned();
    out = RE_LINE.replace_all(&out, " ").into_owned();

    out
}

pub fn should_exclude_this_library(file_path: &str) -> bool {
    file_path.contains("openzeppelin")
        || file_path.contains("forge-std")
        || file_path.contains("ds-test")
        || file_path.contains("erc4626-tests")
        || file_path.contains("halmos-cheatcodes")
        || file_path.contains("solmate")
        || file_path.contains("prb-test")
        || file_path.contains("solady")
}

/// Check if a type name is a Solidity built-in type or common library type
///
/// Covers all Solidity built-in types and common false positives:
/// - Value types: address, bool, string, bytes (lowercase in Solidity)
/// - Integer types: uint8-uint256, int8-int256 (lowercase in Solidity)
/// - Fixed bytes: bytes1-bytes32 (lowercase in Solidity)
/// - Common libraries: Math, SafeMath, Strings, etc. (PascalCase)
///
/// Note: Solidity built-in types are lowercase, but our regex captures PascalCase types.
/// We filter by checking if the LOWERCASE version is a built-in type.
// fn is_solidity_builtin(name: &str) -> bool {
//     let name_lower = name.to_lowercase();
//
//     // Exact matches for common types (check lowercase version)
//     if matches!(
//         name_lower.as_str(),
//         // Solidity built-in types (always lowercase in actual code)
//         "string" | "bytes" | "address" | "uint" | "int" | "bool" |
//         // Memory location keywords
//         "memory" | "storage" | "calldata"
//     ) {
//         return true;
//     }
//
//     // Common OpenZeppelin/library names (PascalCase - check original name)
//     if matches!(
//         name,
//         "Math"
//             | "SafeMath"
//             | "Strings"
//             | "Arrays"
//             | "EnumerableSet"
//             | "EnumerableMap"
//             | "Counters"
//             | "SafeCast"
//             | "SignedMath"
//             | "Checkpoints"
//             | "Context"
//             | "Ownable"
//     ) {
//         return true;
//     }
//
//     // Pattern matches for sized types (check lowercase version)
//     // uint8, uint16, uint24, ..., uint256
//     if name_lower.starts_with("uint") && name_lower.len() > 4 {
//         if let Ok(_) = name_lower[4..].parse::<u16>() {
//             return true;
//         }
//     }
//
//     // int8, int16, int24, ..., int256
//     if name_lower.starts_with("int") && name_lower.len() > 3 {
//         if let Ok(_) = name_lower[3..].parse::<u16>() {
//             return true;
//         }
//     }
//
//     // bytes1, bytes2, ..., bytes32
//     if name_lower.starts_with("bytes") && name_lower.len() > 5 {
//         if let Ok(n) = name_lower[5..].parse::<u8>() {
//             return n >= 1 && n <= 32;
//         }
//     }
//
//     false
// }
//
/// Detect contracts and interfaces referenced in source code.
///
/// Slither's call graph does NOT include:
/// - Constructor calls: `new ContractName(...)`
/// - Interface casts: `IERC20(address)`
/// - Type declarations: `IERC20 public token`
/// - Import statements: `import { IERC20 } from "..."`
///
/// This function parses source code to find all these patterns.
///
/// # Arguments
/// * `contract` - The contract name to analyze
/// * `repo` - Repository paths

/// Heuristic: common library contracts (by name) that we never want to include as called contracts
pub fn is_standard_library_contract_name(name: &str) -> bool {
    // Narrower than starts_with("ERC"): only match ERC followed by one or more digits (e.g., ERC20, ERC721, ERC1155, ERC4626, ERC165, etc.)
    static ERC_NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^ERC\d+").unwrap());
    matches!(
        name,
        "SafeERC20"
            | "SafeCast"
            | "SafeMath"
            | "Math"
            | "AccessControl"
            | "SignedMath"
            | "SafeTransferLib"
            | "FixedPointMathLib"
            | "Address"
            | "StorageSlot"
            | "EnumerableSet"
            | "EnumerableMap"
            | "MerkleProof"
            | "ECDSA"
            | "EIP712"
            | "MessageHashUtils"
    ) || ERC_NUM_RE.is_match(name)
}

/// Heuristic: treat well-known standard interfaces as skippable
pub fn is_standard_interface_name(name: &str) -> bool {
    static IERC_NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^IERC\d+").unwrap());
    IERC_NUM_RE.is_match(name) || name.starts_with("IAccessControl")
}

pub fn path_is_standard_lib(path: &std::path::Path) -> bool {
    let p = path.to_string_lossy().to_ascii_lowercase();
    p.contains("openzeppelin") || p.contains("solmate") || p.contains("forge-std")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to extract imports from source code using the same logic as detect_source_code_dependencies
    fn extract_imports_from_source(source_code: &str) -> (HashSet<String>, HashSet<String>) {
        let mut contracts = HashSet::new();
        let mut interfaces = HashSet::new();

        // Use same regex as production code (captures both names and path)
        let import_regex = Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']"#).unwrap();
        let import_item_regex =
            Regex::new(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:(?i:as)\s+[A-Za-z_][A-Za-z0-9_]*)?\s*$")
                .unwrap();

        for cap in import_regex.captures_iter(source_code) {
            if let Some(imports) = cap.get(1) {
                // cap.get(2) contains the import path, but we don't need it for name extraction tests
                for import in imports.as_str().split(',') {
                    let segment = import.trim();
                    if segment.is_empty() {
                        continue;
                    }
                    let name = if let Some(m) = import_item_regex.captures(segment) {
                        m.get(1).unwrap().as_str()
                    } else {
                        segment
                    };
                    if name.starts_with('I')
                        && name.len() > 1
                        && name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                    {
                        interfaces.insert(name.to_string());
                        contracts.insert(name.to_string());
                    } else if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                        contracts.insert(name.to_string());
                    }
                }
            }
        }

        (contracts, interfaces)
    }

    /// Helper function to extract import paths from source code
    fn extract_import_paths(source_code: &str) -> Vec<String> {
        let mut paths = Vec::new();
        // Named imports: import { ... } from "path"
        let import_regex = Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']"#).unwrap();
        for cap in import_regex.captures_iter(source_code) {
            if let Some(path) = cap.get(2) {
                paths.push(path.as_str().to_string());
            }
        }
        // Simple imports: import "path"
        let simple_import_regex = Regex::new(r#"import\s*["']([^"']+)["']"#).unwrap();
        for cap in simple_import_regex.captures_iter(source_code) {
            if let Some(path) = cap.get(1) {
                paths.push(path.as_str().to_string());
            }
        }
        paths
    }

    #[test]
    fn test_import_simple_contract() {
        let source = r#"import { MyContract } from "./MyContract.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        assert!(contracts.contains("MyContract"));
        assert!(!interfaces.contains("MyContract"));
        assert_eq!(contracts.len(), 1);
        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_import_simple_interface() {
        let source = r#"import { IMyInterface } from "./IMyInterface.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        // Interfaces starting with 'I' followed by uppercase are added to BOTH
        assert!(contracts.contains("IMyInterface"));
        assert!(interfaces.contains("IMyInterface"));
        assert_eq!(contracts.len(), 1);
        assert_eq!(interfaces.len(), 1);
    }

    #[test]
    fn test_import_with_alias_contract() {
        let source = r#"import { MyContract as MC } from "./MyContract.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        // Should capture "MyContract", not "MC"
        assert!(contracts.contains("MyContract"));
        assert!(!contracts.contains("MC"));
        assert_eq!(contracts.len(), 1);
        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_import_with_alias_interface() {
        let source = r#"import { IToken as Token } from "./IToken.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        // Should capture "IToken", not "Token"
        assert!(contracts.contains("IToken"));
        assert!(interfaces.contains("IToken"));
        assert!(!contracts.contains("Token"));
        assert_eq!(contracts.len(), 1);
        assert_eq!(interfaces.len(), 1);
    }

    #[test]
    fn test_import_multiple_no_alias() {
        let source = r#"import { ContractA, ContractB, IInterface } from "./contracts.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        assert!(contracts.contains("ContractA"));
        assert!(contracts.contains("ContractB"));
        assert!(contracts.contains("IInterface"));
        assert!(interfaces.contains("IInterface"));
        assert_eq!(contracts.len(), 3);
        assert_eq!(interfaces.len(), 1);
    }

    #[test]
    fn test_import_multiple_with_aliases() {
        let source =
            r#"import { ContractA as CA, IToken as Token, ContractB } from "./contracts.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        // Should capture original names, not aliases
        assert!(contracts.contains("ContractA"));
        assert!(contracts.contains("IToken"));
        assert!(contracts.contains("ContractB"));
        assert!(!contracts.contains("CA"));
        assert!(!contracts.contains("Token"));
        assert!(interfaces.contains("IToken"));
        assert_eq!(contracts.len(), 3);
        assert_eq!(interfaces.len(), 1);
    }

    #[test]
    fn test_import_mixed_case_as_keyword() {
        // Test case-insensitive "as" keyword
        let source1 = r#"import { MyContract AS MC } from "./MyContract.sol";"#;
        let source2 = r#"import { MyContract As MC } from "./MyContract.sol";"#;
        let source3 = r#"import { MyContract aS MC } from "./MyContract.sol";"#;

        for source in &[source1, source2, source3] {
            let (contracts, _) = extract_imports_from_source(source);
            assert!(
                contracts.contains("MyContract"),
                "Failed for source: {}",
                source
            );
            assert!(!contracts.contains("MC"), "Failed for source: {}", source);
        }
    }

    #[test]
    fn test_import_with_whitespace_variations() {
        let sources = vec![
            r#"import {MyContract} from "./MyContract.sol";"#,
            r#"import { MyContract } from "./MyContract.sol";"#,
            r#"import {  MyContract  } from "./MyContract.sol";"#,
            r#"import { MyContract as MC } from "./MyContract.sol";"#,
            r#"import {MyContract as MC} from "./MyContract.sol";"#,
            r#"import {  MyContract  as  MC  } from "./MyContract.sol";"#,
        ];

        for source in sources {
            let (contracts, _) = extract_imports_from_source(source);
            assert!(
                contracts.contains("MyContract"),
                "Failed for source: {}",
                source
            );
            assert!(!contracts.contains("MC"), "Failed for source: {}", source);
        }
    }

    #[test]
    fn test_import_empty_segments() {
        // Edge case: extra commas or whitespace
        let source = r#"import { ContractA, , ContractB } from "./contracts.sol";"#;
        let (contracts, _) = extract_imports_from_source(source);

        assert!(contracts.contains("ContractA"));
        assert!(contracts.contains("ContractB"));
        // Should handle empty segments gracefully
        assert_eq!(contracts.len(), 2);
    }

    #[test]
    fn test_import_lowercase_ignored() {
        // Lowercase identifiers should be ignored (not contracts/interfaces)
        let source = r#"import { myContract, anotherOne } from "./contracts.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        assert_eq!(contracts.len(), 0);
        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_import_interface_not_starting_with_i() {
        // Interface that doesn't start with 'I' should be treated as contract only
        let source = r#"import { MyInterface } from "./MyInterface.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        assert!(contracts.contains("MyInterface"));
        assert!(!interfaces.contains("MyInterface"));
        assert_eq!(contracts.len(), 1);
        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_import_i_followed_by_lowercase() {
        // "Ifoo" - 'I' followed by lowercase should be treated as regular contract
        let source = r#"import { Ifoo } from "./Ifoo.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        assert!(contracts.contains("Ifoo"));
        assert!(!interfaces.contains("Ifoo"));
        assert_eq!(contracts.len(), 1);
        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_import_single_i() {
        // Edge case: single letter "I"
        let source = r#"import { I } from "./I.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        // Single 'I' doesn't meet the criteria (len > 1 && second char uppercase)
        assert!(contracts.contains("I"));
        assert!(!interfaces.contains("I"));
        assert_eq!(contracts.len(), 1);
        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_import_complex_real_world_example() {
        let source = r#"
            import { IERC20 as Token, IUniswapV2Router02 as Router } from "@uniswap/v2-periphery/contracts/interfaces/IUniswapV2Router02.sol";
            import { SafeERC20, Address } from "@openzeppelin/contracts/utils/Address.sol";
            import { MyVault, IStrategy as Strategy } from "./MyVault.sol";
        "#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        // Original names should be captured
        assert!(contracts.contains("IERC20"));
        assert!(contracts.contains("IUniswapV2Router02"));
        assert!(contracts.contains("SafeERC20"));
        assert!(contracts.contains("Address"));
        assert!(contracts.contains("MyVault"));
        assert!(contracts.contains("IStrategy"));

        // Aliases should NOT be captured
        assert!(!contracts.contains("Token"));
        assert!(!contracts.contains("Router"));
        assert!(!contracts.contains("Strategy"));

        // Interfaces (I + uppercase)
        assert!(interfaces.contains("IERC20"));
        assert!(interfaces.contains("IUniswapV2Router02"));
        assert!(interfaces.contains("IStrategy"));
        assert!(!interfaces.contains("SafeERC20"));
        assert!(!interfaces.contains("Address"));
        assert!(!interfaces.contains("MyVault"));
    }

    #[test]
    fn test_import_multiline() {
        let source = r#"
            import {
                ContractA,
                ITokenB as TokenB,
                ContractC
            } from "./contracts.sol";
        "#;
        let (contracts, _) = extract_imports_from_source(source);

        assert!(contracts.contains("ContractA"));
        assert!(contracts.contains("ITokenB"));
        assert!(contracts.contains("ContractC"));
        assert!(!contracts.contains("TokenB"));
    }

    #[test]
    fn test_import_underscore_in_names() {
        let source = r#"import { My_Contract, IToken_V2 as Token } from "./contracts.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        assert!(contracts.contains("My_Contract"));
        assert!(contracts.contains("IToken_V2"));
        assert!(interfaces.contains("IToken_V2"));
        assert!(!contracts.contains("Token"));
    }

    #[test]
    fn test_import_numbers_in_names() {
        let source = r#"import { ERC20Token, IERC721A as NFT } from "./contracts.sol";"#;
        let (contracts, interfaces) = extract_imports_from_source(source);

        assert!(contracts.contains("ERC20Token"));
        assert!(contracts.contains("IERC721A"));
        assert!(interfaces.contains("IERC721A"));
        assert!(!contracts.contains("NFT"));
    }

    // ========== Import Path Extraction Tests ==========

    #[test]
    fn test_extract_import_path_simple() {
        let source = r#"import { MyContract } from "./MyContract.sol";"#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "./MyContract.sol");
    }

    #[test]
    fn test_extract_import_path_external() {
        let source = r#"import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";"#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "@openzeppelin/contracts/token/ERC20/IERC20.sol");
    }

    #[test]
    fn test_extract_import_path_multiple_imports() {
        let source = r#"
            import { ContractA } from "./ContractA.sol";
            import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
            import { IUniswap } from "@uniswap/v3-core/interfaces/IUniswap.sol";
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], "./ContractA.sol");
        assert_eq!(paths[1], "@openzeppelin/contracts/token/ERC20/IERC20.sol");
        assert_eq!(paths[2], "@uniswap/v3-core/interfaces/IUniswap.sol");
    }

    #[test]
    fn test_extract_import_path_multiline_no_semicolon_on_same_line() {
        // Test that semicolon on separate line doesn't break parsing
        let source = r#"
            import {
                IERC20,
                SafeERC20
            } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol"
            ;
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 1);
        assert_eq!(
            paths[0],
            "@openzeppelin/contracts/token/ERC20/SafeERC20.sol"
        );
    }

    #[test]
    fn test_extract_import_path_no_semicolon() {
        // Edge case: missing semicolon (malformed but should still parse)
        let source = r#"import { MyContract } from "./MyContract.sol""#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "./MyContract.sol");
    }

    #[test]
    fn test_import_no_space_between_from_and_quote() {
        let source = r#"import{D}from"./D.sol";"#;
        let paths = extract_import_paths(source);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "./D.sol");
    }

    #[test]
    fn test_extract_import_path_single_quotes() {
        let source = r#"import { MyContract } from './MyContract.sol';"#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "./MyContract.sol");
    }

    #[test]
    fn test_extract_import_path_mixed_quotes() {
        let source = r#"
            import { ContractA } from "./ContractA.sol";
            import { ContractB } from './ContractB.sol';
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], "./ContractA.sol");
        assert_eq!(paths[1], "./ContractB.sol");
    }

    #[test]
    fn test_extract_import_path_relative_paths() {
        let source = r#"
            import { A } from "./A.sol";
            import { B } from "../B.sol";
            import { C } from "../../C.sol";
            import { D } from "./nested/D.sol";
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 4);
        assert_eq!(paths[0], "./A.sol");
        assert_eq!(paths[1], "../B.sol");
        assert_eq!(paths[2], "../../C.sol");
        assert_eq!(paths[3], "./nested/D.sol");
    }

    #[test]
    fn test_extract_import_path_absolute_paths() {
        let source = r#"
            import { A } from "contracts/A.sol";
            import { B } from "src/contracts/B.sol";
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], "contracts/A.sol");
        assert_eq!(paths[1], "src/contracts/B.sol");
    }

    #[test]
    fn test_extract_import_path_with_aliases() {
        // Path extraction should work regardless of aliases
        let source = r#"import { MyContract as MC, IToken as Token } from "./contracts.sol";"#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "./contracts.sol");
    }

    #[test]
    fn test_extract_import_path_multiline_complex() {
        let source = r#"
            import {
                IERC20 as Token,
                IUniswapV2Router02 as Router,
                SafeERC20,
                Address
            } from "@openzeppelin/contracts/utils/Address.sol";
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "@openzeppelin/contracts/utils/Address.sol");
    }

    #[test]
    fn test_extract_import_path_external_providers() {
        let source = r#"
            import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
            import { IUniswapV2Router } from "@uniswap/v2-periphery/contracts/interfaces/IUniswapV2Router.sol";
            import { AggregatorV3Interface } from "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";
            import { ENS } from "@ensdomains/ens-contracts/contracts/registry/ENS.sol";
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 4);
        assert!(paths[0].starts_with("@openzeppelin/"));
        assert!(paths[1].starts_with("@uniswap/"));
        assert!(paths[2].starts_with("@chainlink/"));
        assert!(paths[3].starts_with("@ensdomains/"));
    }

    #[test]
    fn test_extract_import_path_whitespace_variations() {
        // Note: Regex requires at least one whitespace between 'from' and the quote
        let source = r#"
            import  {  B  }  from  "./B.sol"  ;
            import { C } from   "./C.sol";
            import{D}from"./D.sol";
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], "./B.sol");
        assert_eq!(paths[1], "./C.sol");
        assert_eq!(paths[2], "./D.sol");
    }

    #[test]
    fn test_extract_import_path_empty_source() {
        let source = "";
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn test_extract_import_path_no_imports() {
        let source = r#"
            contract MyContract {
                function test() public {}
            }
        "#;
        let paths = extract_import_paths(source);

        assert_eq!(paths.len(), 0);
    }
    #[test]
    fn test_user_sample_imports_paths() {
        let source = r#"import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
import {IPriceOracle} from "../interfaces/IPriceOracle.sol";
import {Errors} from "./lib/Errors.sol";

import {BaseAdapter} from "./BaseAdapter.sol";
import {IPriceOracle} from "../../interfaces/IPriceOracle.sol";
import {ScaleUtils} from "@euler-price-oracle/lib/ScaleUtils.sol";
import {Errors} from "../lib/Errors.sol";

import {ICovenant, MarketId, MarketParams, MarketState, SwapParams, RedeemParams, MintParams, SynthTokens, TokenPrices, AssetType} from "./interfaces/ICovenant.sol";
import {ILiquidExchangeModel} from "./interfaces/ILiquidExchangeModel.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
import {NoDelegateCall} from "./libraries/NoDelegateCall.sol";
import {ValidationLogic} from "./libraries/ValidationLogic.sol";
import {MarketParamsLib} from "./libraries/MarketParams.sol";
import {MulticallLib} from "./libraries/Multicall.sol";
import {Errors} from "./libraries/Errors.sol";
import {Events} from "./libraries/Events.sol";"#;

        let paths = extract_import_paths(source);
        assert_eq!(paths.len(), 19);
        assert_eq!(
            paths,
            vec![
                "forge-std/interfaces/IERC4626.sol",
                "@openzeppelin/access/Ownable2Step.sol",
                "../interfaces/IPriceOracle.sol",
                "./lib/Errors.sol",
                "./BaseAdapter.sol",
                "../../interfaces/IPriceOracle.sol",
                "@euler-price-oracle/lib/ScaleUtils.sol",
                "../lib/Errors.sol",
                "./interfaces/ICovenant.sol",
                "./interfaces/ILiquidExchangeModel.sol",
                "@openzeppelin/token/ERC20/IERC20.sol",
                "@openzeppelin/token/ERC20/utils/SafeERC20.sol",
                "@openzeppelin/access/Ownable2Step.sol",
                "./libraries/NoDelegateCall.sol",
                "./libraries/ValidationLogic.sol",
                "./libraries/MarketParams.sol",
                "./libraries/Multicall.sol",
                "./libraries/Errors.sol",
                "./libraries/Events.sol",
            ]
        );
    }

    #[test]
    fn test_user_sample_imports_names() {
        let source = r#"import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
import {IPriceOracle} from "../interfaces/IPriceOracle.sol";
import {Errors} from "./lib/Errors.sol";

import {BaseAdapter} from "./BaseAdapter.sol";
import {IPriceOracle} from "../../interfaces/IPriceOracle.sol";
import {ScaleUtils} from "@euler-price-oracle/lib/ScaleUtils.sol";
import {Errors} from "../lib/Errors.sol";

import {ICovenant, MarketId, MarketParams, MarketState, SwapParams, RedeemParams, MintParams, SynthTokens, TokenPrices, AssetType} from "./interfaces/ICovenant.sol";
import {ILiquidExchangeModel} from "./interfaces/ILiquidExchangeModel.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
import {NoDelegateCall} from "./libraries/NoDelegateCall.sol";
import {ValidationLogic} from "./libraries/ValidationLogic.sol";
import {MarketParamsLib} from "./libraries/MarketParams.sol";
import {MulticallLib} from "./libraries/Multicall.sol";
import {Errors} from "./libraries/Errors.sol";
import {Events} from "./libraries/Events.sol";"#;

        let (contracts, interfaces) = extract_imports_from_source(source);

        // Must appear in BOTH contracts and interfaces
        for iface in [
            "IERC4626",
            "IPriceOracle",
            "ICovenant",
            "ILiquidExchangeModel",
            "IERC20",
        ] {
            assert!(contracts.contains(iface), "contracts missing {}", iface);
            assert!(interfaces.contains(iface), "interfaces missing {}", iface);
        }

        // Expected contracts (non-interfaces)
        for name in [
            "Ownable2Step",
            "Ownable",
            "Errors",
            "BaseAdapter",
            "ScaleUtils",
            "MarketId",
            "MarketParams",
            "MarketState",
            "SwapParams",
            "RedeemParams",
            "MintParams",
            "SynthTokens",
            "TokenPrices",
            "AssetType",
            "SafeERC20",
            "NoDelegateCall",
            "ValidationLogic",
            "MarketParamsLib",
            "MulticallLib",
            "Events",
        ] {
            assert!(contracts.contains(name), "contracts missing {}", name);
        }
    }
}
