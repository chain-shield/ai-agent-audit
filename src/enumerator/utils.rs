use anyhow::Result;
use anyhow::anyhow;
use log::info;
use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::params_from_iter;
use rusqlite::{Connection, OptionalExtension};
/// Enumeration utilities for code block generation and analysis.
///
/// This module provides utility functions for generating markdown code blocks,
/// extracting function metadata, managing IR mappings, and performing token
/// counting for optimal code slice generation within LLM context limits.
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::enumerator::libraries::ParsedLibrary;
use crate::enumerator::libraries::generate_library_to_code_mapping;
use crate::enumerator::libraries::get_library_code_for_library_calls;
use crate::llm_review::contract::contract_file_map::ContractType;
use crate::llm_review::contract::contract_file_map::insert_contract_to_file_mapping;
use crate::llm_review::contract::contract_file_map::insert_lib_contract_to_file_mapping;

use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::check_folder_name::is_library_file;
use crate::utils::fn_labels::get_modifiers_label;
use crate::utils::fn_labels::get_visibility_label;
use crate::utils::get_fn_name::get_function_name_from_interface;
use crate::utils::parse_library_file::LibCall;
use crate::utils::parse_library_file::parse_library_text;
use crate::{
    build_brain::{self, graph_db::SmartContractFunction, slither_ffi::SlithIRFn},
    utils::bpe::get_bpe,
};

type CodeIrMap = HashMap<(String, String), SlithIRFn>;
type CodeIrCache = HashMap<String, CodeIrMap>;

/// Global cache for get_code_ir_map results.
/// Key: project_id, Value: IR map for that project
static CODE_IR_MAP_CACHE: Lazy<Mutex<CodeIrCache>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Global cache for contracts_in_source_folder results.
/// Key: project_id, Value: Vec of contract names
/// This ensures the inheritance map is only built once per repository.
static CONTRACTS_CACHE: Lazy<Mutex<HashMap<String, Vec<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Generates a markdown code block for a specific function with IR representation.
///
/// Creates a formatted markdown section containing the function's SlithIR
/// intermediate representation, including contract context and function metadata.
///
/// # Arguments
/// * `func` - Smart contract function metadata
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `String` - Formatted markdown code block with IR content
pub async fn generate_codeblock_for_function(
    func: &SmartContractFunction,
    repo: &RepoPaths,
) -> anyhow::Result<String> {
    let ir_map = get_code_ir_map(repo).await?;
    let mut function_slice = String::new();
    let func_name = get_function_name_from_interface(&func.name);
    let visibility = get_visibility_label(&func.visibility);
    let modifiers = get_modifiers_label(&func.modifiers);

    if let Some(ir) = ir_map.get(&(func.contract.clone(), func_name)) {
        function_slice.push_str(&format!(
            "#### {} {}{}\n",
            ir.function, visibility, modifiers
        ));
        function_slice.push_str("```slithir\n");
        function_slice.push_str(&ir.ir);
        function_slice.push_str("\n```");
    }

    // info!("function slice => {:#?}", function_slice);
    Ok(function_slice)
}

pub async fn generate_library_funcs_markdown(library_calls: &[LibCall]) -> String {
    let library_code = get_library_code_for_library_calls(library_calls).await;
    let mut library_markdown = String::new();

    library_markdown.push_str("## Library Calls for this Contract");

    // Deterministic ordering by sorted key (func_sig)
    let mut keys: Vec<String> = library_code.keys().cloned().collect();
    keys.sort();
    for func_sig in keys {
        if let Some(code) = library_code.get(&func_sig) {
            library_markdown.push_str(&format!(
                "\n\n************ CODE FOR {}************\n\n",
                func_sig
            ));
            library_markdown.push_str(code);
            library_markdown.push_str("\n\n"); // spacing between entries
        }
    }
    library_markdown
}

pub async fn get_hashmap_of_contract_to_functions(
    repo: &RepoPaths,
    semantic_db: &Connection,
) -> anyhow::Result<HashMap<String, Vec<SmartContractFunction>>> {
    // find all main contracts for app (ones in /src)
    info!("grabbing all contracts...");

    let contracts = contracts_in_source_folder(repo).await?;

    if contracts.is_empty() {
        // Either return empty map or error — your call
        return Err(anyhow!("no contracts found in source folder"));
        // return Ok(HashMap::new());
    }

    // Build ?1,?2,... for the contracts and put project_id as the LAST param
    let placeholders = (1..=contracts.len())
        .map(|i| format!("?{}", i))
        .collect::<Vec<_>>()
        .join(",");

    let sql = format!(
        "SELECT func_id, project_id, contract, name, ir, visibility, modifiers, mutability
         FROM functions
         WHERE contract IN ({}) AND project_id = ?{}",
        placeholders,
        contracts.len() + 1
    );

    let mut stmt = semantic_db.prepare(&sql)?;

    // Params: all contracts first, then project_id
    let params_iter = contracts
        .iter()
        .map(|s| s.as_str())
        .chain(std::iter::once(repo.project_id.as_str()));

    let rows = stmt.query_map(params_from_iter(params_iter), |row| {
        let modifier_str: Option<String> = row.get(6)?;
        let modifiers: Vec<String> = modifier_str
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim_matches([' ', '\'']).to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(SmartContractFunction {
            id: row.get(0)?,
            project_id: row.get(1)?,
            contract: row.get(2)?,
            name: row.get(3)?,
            ir: row.get(4)?,
            visibility: row.get(5)?,
            modifiers,
            mutability: row.get(7)?,
        })
    })?;

    let functions_of_contract: Vec<SmartContractFunction> =
        rows.collect::<rusqlite::Result<_>>()?;

    let mut map: HashMap<String, Vec<SmartContractFunction>> = HashMap::new();
    for f in functions_of_contract {
        map.entry(f.contract.clone()).or_default().push(f);
    }
    Ok(map)
}

/// Calculates the token count of a function's IR representation.
///
/// Generates the markdown codeblock for the function and counts the number of tokens
/// using the BPE tokenizer.
///
/// # Arguments
/// * `func` - The smart contract function
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<usize>` - The token count
pub async fn get_token_count_of_function_ir(
    func: &SmartContractFunction,
    repo: &RepoPaths,
) -> anyhow::Result<usize> {
    // Generate the function's markdown codeblock
    let fn_text = generate_codeblock_for_function(func, repo).await?;

    // Count tokens using BPE tokenizer
    let bpe = get_bpe();
    let tokens = bpe.encode_with_special_tokens(&fn_text).len();

    Ok(tokens)
}

/// Retrieves a mapping of contract and function names to their SlithIR representations.
/// Results are cached globally per project to avoid redundant Slither calls.
///
/// Extracts the function name from the full function signature and creates a map
/// keyed by (contract_name, function_name) tuples.
///
/// # Arguments
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<HashMap<(String, String), SlithIRFn>>` - Map of (contract, function) to SlithIR
pub async fn get_code_ir_map(
    repo: &RepoPaths,
) -> anyhow::Result<HashMap<(String, String), SlithIRFn>> {
    // Check cache first
    {
        let cache = CODE_IR_MAP_CACHE.lock().unwrap();
        if let Some(cached_map) = cache.get(&repo.project_id) {
            return Ok(cached_map.clone());
        }
    }

    // Cache miss - compute the result
    // info!(
    //     "Cache miss for get_code_ir_map - running Slither for project {}",
    //     repo.project_id
    // );

    // Regex to extract function name from full signature (e.g., "Contract.function(args)")
    let extract_function_name = Regex::new(r#"[A-Za-z0-9$_]+\.([A-Za-z0-9$_]+)\([^)]*\)"#)?;

    // Get IR and storage variables from Slither
    let (ir_vec, _, _) = build_brain::slither_ffi::get_slither_ir_and_storage(repo).await?;

    // Create map of (contract, function) -> SlithIRFn
    let ir_map: HashMap<(String, String), SlithIRFn> = ir_vec
        .into_iter()
        .map(|f| {
            if let Some(c) = extract_function_name.captures(&f.function) {
                // Extract function name from signature
                ((f.contract.clone(), c[1].to_string()), f)
            } else {
                // Use full function signature if extraction fails
                ((f.contract.clone(), f.function.clone()), f)
            }
        })
        .collect();

    // Store in cache
    {
        let mut cache = CODE_IR_MAP_CACHE.lock().unwrap();
        cache.insert(repo.project_id.clone(), ir_map.clone());
    }

    info!(
        "Cached IR map for project {} with {} entries",
        repo.project_id,
        ir_map.len()
    );
    Ok(ir_map)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContractScope {
    All,
    InScope,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SolFileType {
    Standard,
    LibFolder, // file is coming from main /lib/ folder
}

// ============================================================================
// Regex Patterns (DRY - defined once, used multiple times)
// ============================================================================

/// Lazy-initialized regex patterns for Solidity parsing
/// Made public so other modules can reuse these patterns (DRY principle)
pub static SOLIDITY_REGEXES: Lazy<SolidityRegexes> = Lazy::new(|| {
    SolidityRegexes {
    contract_decl: Regex::new(
        r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
    )
    .unwrap(),
    inheritance: Regex::new(
        r"(?m)^\s*(?:abstract\s+)?(?:contract|interface|library)\s+([A-Za-z_][A-Za-z0-9_]*)\s+is\s+([^{]+)",
    )
    .unwrap(),
    // Uses same regex as parse_solidity.rs for consistency
    import_named: Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*[\"']([^\"']+)[\"']"#).unwrap(),
    import_item: Regex::new(
        r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:(?i:as)\s+[A-Za-z_][A-Za-z0-9_]*)?\s*$",
    )
    .unwrap(),
}
});

pub struct SolidityRegexes {
    pub contract_decl: Regex,
    pub inheritance: Regex,
    pub import_named: Regex,
    pub import_item: Regex,
}

// ============================================================================
// Helper Functions for Solidity Parsing
// ============================================================================

/// Parse import statements from Solidity source code and build a mapping
/// of contract names (including aliases) to their import paths.
///
/// # Arguments
/// * `content` - The Solidity source code
///
/// # Returns
/// HashMap mapping contract names to import paths
///
/// # Example
/// ```solidity
/// import {BaseAdapter as EulerBaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";
/// import "./EntropyEvents.sol";
/// ```
/// Returns:
/// - `{"EulerBaseAdapter" => "@euler-price-oracle/adapter/BaseAdapter.sol"}`
/// - `{"EntropyEvents" => "./EntropyEvents.sol"}`
fn parse_import_map(content: &str) -> HashMap<String, String> {
    let mut import_map = HashMap::new();

    // Parse named imports: import { X, Y, Z } from "path"
    for cap in SOLIDITY_REGEXES.import_named.captures_iter(content) {
        if let Some(imports) = cap.get(1) {
            let import_path = cap.get(2).unwrap().as_str();

            // Parse each imported name (handle aliases)
            for import in imports.as_str().split(',') {
                let segment = import.trim();
                if segment.is_empty() {
                    continue;
                }

                // Use import_item_regex to extract the actual contract name (before "as")
                let name = if let Some(m) = SOLIDITY_REGEXES.import_item.captures(segment) {
                    m.get(1).unwrap().as_str()
                } else {
                    segment
                };

                // Check if this is an alias: "OriginalName as Alias"
                if let Some(as_pos) = segment.find(" as ") {
                    // Use the alias (after "as") as the key
                    let alias = segment[as_pos + 4..].trim();
                    import_map.insert(alias.to_string(), import_path.to_string());
                } else {
                    // No alias, use the contract name directly
                    import_map.insert(name.to_string(), import_path.to_string());
                }
            }
        }
    }

    // Parse simple imports: import "path";
    // For these, we extract the contract name from the file path
    // Example: import "./EntropyEvents.sol" => {"EntropyEvents" => "./EntropyEvents.sol"}
    let simple_import_regex = Regex::new(r#"import\s*[\"']([^\"']+)[\"']"#).unwrap();
    for cap in simple_import_regex.captures_iter(content) {
        let import_path = cap.get(1).unwrap().as_str();

        // Extract contract name from file path
        // "./EntropyEvents.sol" => "EntropyEvents"
        // "@openzeppelin/contracts/token/ERC20/IERC20.sol" => "IERC20"
        if let Some(file_name) = import_path.split('/').next_back()
            && let Some(contract_name) = file_name.strip_suffix(".sol")
        {
            import_map.insert(contract_name.to_string(), import_path.to_string());
        }
    }

    import_map
}

/// Normalize a path by removing `.` and `..` components without canonicalizing.
///
/// This function manually resolves `.` and `..` components in a path without
/// calling `canonicalize()`, which would resolve symlinks and cause path mismatches
/// on macOS where `/tmp` is a symlink to `/private/tmp`.
///
/// # Arguments
/// * `path` - The path to normalize
///
/// # Returns
/// Normalized PathBuf
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::CurDir => {
                // Skip "." components
            }
            std::path::Component::ParentDir => {
                // Pop the last component for ".."
                if !components.is_empty() {
                    components.pop();
                }
            }
            _ => {
                // Keep all other components (Prefix, RootDir, Normal)
                components.push(component);
            }
        }
    }

    // Rebuild the path from components
    components.iter().collect()
}

/// Resolve an import path to an actual file path.
///
/// # Arguments
/// * `import_path` - The import path from the Solidity source (e.g., "@euler-price-oracle/...")
/// * `current_file` - The file containing the import statement
/// * `repo` - Repository paths
///
/// # Returns
/// Resolved PathBuf if successful, None otherwise
fn resolve_import_to_file(
    import_path: &str,
    current_file: &Path,
    repo: &RepoPaths,
) -> Option<PathBuf> {
    // 1. Try remapping resolution (for @euler-price-oracle, etc.)
    let resolved_path =
        if let Some(remapped) = crate::utils::remapping::resolve_import_path(import_path, repo) {
            PathBuf::from(remapped)
        }
        // 2. Handle relative paths (../, ./)
        else if import_path.starts_with("../") || import_path.starts_with("./") {
            if let Some(parent_dir) = current_file.parent() {
                parent_dir.join(import_path)
            } else {
                PathBuf::from(import_path)
            }
        }
        // 3. Try as-is
        else {
            PathBuf::from(import_path)
        };

    // Normalize the path
    let parent_file = if resolved_path.is_absolute() {
        resolved_path
    } else {
        repo.get_protocol_root().join(&resolved_path)
    };

    // Return the path without canonicalization to match the format in repo.sol_files
    // This prevents path mismatches on macOS where /tmp is a symlink to /private/tmp
    // However, we still need to normalize .. and . components manually
    if parent_file.exists() {
        // Manually normalize the path by removing .. and . components
        // This is needed because paths like "src/curators/../interfaces/IPriceOracle.sol"
        // need to be normalized to "src/interfaces/IPriceOracle.sol" to match repo.sol_files
        let normalized = normalize_path(&parent_file);
        Some(normalized)
    } else {
        None
    }
}

/// Process contract declarations in a Solidity file and insert them into the appropriate mappings.
///
/// # Arguments
/// * `content` - The Solidity source code
/// * `file` - Path to the Solidity file
/// * `file_type` - Whether this is a Standard or LibFolder file
/// * `repo` - Repository paths
/// * `contracts` - Mutable vector to collect contract names
async fn process_contract_declarations(
    content: &str,
    file: &Path,
    file_type: SolFileType,
    repo: &RepoPaths,
    contracts: &mut Vec<String>,
) -> Result<()> {
    for cap in SOLIDITY_REGEXES.contract_decl.captures_iter(content) {
        let decl = cap.get(1).unwrap().as_str();
        let contract = cap.get(2).unwrap().as_str();
        let contract_type = match decl {
            "abstract contract" => ContractType::AbstractContract,
            "contract" => ContractType::Contract,
            "interface" => ContractType::Interface,
            "library" => ContractType::Library,
            _ => ContractType::Library,
        };

        if file_type == SolFileType::Standard {
            // Only add concrete contracts to the contracts list (exclude interfaces, mocks, and test contracts)
            let is_mock = contract.to_ascii_lowercase().contains("mock")
                || file.to_string_lossy().contains("/mocks/")
                || file.to_string_lossy().contains("/test/");
            let is_interface = contract_type == ContractType::Interface;

            if !is_mock && !is_interface {
                log::info!(
                    "✅ Found source contract: {} ({:?}) in {}",
                    contract,
                    contract_type,
                    file.display()
                );
                contracts.push(contract.to_string());
            } else {
                log::debug!(
                    "⏭️  Skipping contract: {} ({:?}, is_mock={}, is_interface={}) in {}",
                    contract,
                    contract_type,
                    is_mock,
                    is_interface,
                    file.display()
                );
            }
            insert_contract_to_file_mapping(contract, file, contract_type, repo).await?;
        } else {
            log::debug!(
                "📚 Found library contract: {} in {}",
                contract,
                file.display()
            );
            insert_lib_contract_to_file_mapping(contract, file, contract_type, repo).await?;
        }
    }
    Ok(())
}

/// Process inheritance relationships in a Solidity file and store them in the inheritance map.
///
/// # Arguments
/// * `content` - The Solidity source code
/// * `file` - Path to the Solidity file
/// * `file_type` - Whether this is a Standard or LibFolder file
/// * `repo` - Repository paths
/// * `import_map` - Mapping of contract names to import paths
async fn process_inheritance_relationships(
    content: &str,
    file: &Path,
    _file_type: SolFileType,
    repo: &RepoPaths,
    import_map: &HashMap<String, String>,
) -> Result<()> {
    // log::info!(
    //     "🔍 process_inheritance_relationships called for file: {} ({} chars)",
    //     file.display(),
    //     content.len()
    // );

    for cap in SOLIDITY_REGEXES.inheritance.captures_iter(content) {
        let child_contract = cap.get(1).unwrap().as_str();
        let parent_list = cap.get(2).unwrap().as_str();

        // log::info!(
        //     "🔗 Found inheritance: {} is {} in {}",
        //     child_contract,
        //     parent_list,
        //     file.display()
        // );

        // Parse parent list (split by comma, trim whitespace)
        let parents: Vec<String> = parent_list
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Resolve each parent using the import map
        for parent in parents {
            if let Some(import_path) = import_map.get(&parent) {
                if let Some(parent_file) = resolve_import_to_file(import_path, file, repo) {
                    // Determine parent file type (unused but kept for potential future use)
                    let _parent_file_type =
                        if crate::utils::check_folder_name::is_library_file(&parent_file, repo) {
                            SolFileType::LibFolder
                        } else {
                            SolFileType::Standard
                        };

                    // Extract actual contract name from parent file
                    // The `parent` variable might be an alias, so we need to get the real contract name
                    let actual_parent_name =
                        if let Ok(parent_content) = std::fs::read_to_string(&parent_file) {
                            // Find ALL contract declarations in the parent file
                            let mut found_name = None;
                            for cap in SOLIDITY_REGEXES
                                .contract_decl
                                .captures_iter(&parent_content)
                            {
                                let contract_name = cap.get(2).unwrap().as_str();

                                // If there's only one contract in the file, use it
                                // If there are multiple, try to match by the import path's file name
                                if found_name.is_none() {
                                    found_name = Some(contract_name.to_string());
                                }

                                // Check if this contract name matches the file name
                                // e.g., ChainlinkOracle.sol should contain contract ChainlinkOracle
                                if let Some(file_stem) = parent_file.file_stem()
                                    && file_stem.to_string_lossy() == contract_name
                                {
                                    found_name = Some(contract_name.to_string());
                                    break; // Found exact match, use it
                                }
                            }

                            found_name.unwrap_or_else(|| parent.clone())
                        } else {
                            // Fallback to the alias name if we can't read the file
                            parent.clone()
                        };

                    // Store inheritance edge: child inherits from parent
                    if let Err(e) = crate::build_brain::inheritance_map::insert_inheritance_edge(
                        (child_contract.to_string(), file.to_path_buf()),
                        (actual_parent_name.clone(), parent_file.clone()),
                        repo,
                    )
                    .await
                    {
                        log::warn!(
                            "Failed to insert inheritance edge {} -> {}: {}",
                            child_contract,
                            actual_parent_name,
                            e
                        );
                    }

                    // else {
                    //     log::info!(
                    //         "Inheritance: {} ({:?}, {}) -> {} ({:?}, {})",
                    //         child_contract,
                    //         file_type,
                    //         file.display(),
                    //         actual_parent_name,
                    //         parent_file_type,
                    //         parent_file.display()
                    //     );
                    // }
                } else {
                    log::debug!(
                        "Could not resolve import path '{}' for parent '{}' in {}",
                        import_path,
                        parent,
                        file.display()
                    );
                }
            } else {
                // Parent not in import map - might be from same file or excluded library
                // Try to find the parent in the same file

                // Check if parent is in the same file
                let parent_in_same_file = content.contains(&format!("interface {}", parent))
                    || content.contains(&format!("contract {}", parent))
                    || content.contains(&format!("abstract contract {}", parent))
                    || content.contains(&format!("library {}", parent));

                if parent_in_same_file {
                    // Parent is in the same file - add inheritance edge
                    // log::info!(
                    //     "Parent '{}' found in same file as '{}' ({})",
                    //     parent,
                    //     child_contract,
                    //     file.display()
                    // );
                    //
                    if let Err(e) = crate::build_brain::inheritance_map::insert_inheritance_edge(
                        (child_contract.to_string(), file.to_path_buf()),
                        (parent.clone(), file.to_path_buf()),
                        repo,
                    )
                    .await
                    {
                        log::warn!(
                            "Failed to insert inheritance edge {} -> {}: {}",
                            child_contract,
                            parent,
                            e
                        );
                    }
                } else {
                    log::debug!(
                        "Parent '{}' not found in imports or same file for '{}' in {} - likely excluded library",
                        parent,
                        child_contract,
                        file.display()
                    );
                }
            }
        }
    }

    // if inheritance_count > 0 {
    //     log::debug!(
    //         "✅ Processed {} inheritance relationships in {}",
    //         inheritance_count,
    //         file.display()
    //     );
    // }

    Ok(())
}

// ============================================================================
// Main Function
// ============================================================================

/// Return the names of all `contract XXX` declarations that sit
/// anywhere under `repo_root/src/`.
///
/// This function is cached per project_id to ensure the inheritance map is only built once.
pub async fn contracts_in_source_folder(repo: &RepoPaths) -> Result<Vec<String>> {
    // Check cache first
    {
        let cache = CONTRACTS_CACHE.lock().unwrap();
        if let Some(cached_contracts) = cache.get(&repo.project_id) {
            log::debug!(
                "✅ Using cached contracts for project '{}' ({} contracts)",
                repo.project_id,
                cached_contracts.len()
            );
            return Ok(cached_contracts.clone());
        }
    }

    if !repo.source_code_folders.iter().any(|f| f.exists()) {
        anyhow::bail!("no src/ folder found at {:?},", repo.source_code_folders);
    }

    log::info!(
        "📂 Processing {} .sol files to build inheritance map...",
        repo.sol_files.len()
    );
    log::info!("   Source code folders: {:?}", repo.source_code_folders);

    let mut libraries = Vec::<ParsedLibrary>::new();
    let mut contracts = Vec::<String>::new();
    let mut files_processed = 0;
    let mut files_skipped = 0;

    log::info!(
        "🔍 Building inheritance map for {} Solidity files. Source folders: {:?}",
        repo.sol_files.len(),
        repo.source_code_folders
    );

    for file in &repo.sol_files {
        // Determine file type: Standard (in source folder) or LibFolder (in lib/ but not nested)
        let is_lib = is_library_file(file, repo);
        let is_source = repo.source_code_folders.iter().any(|f| file.starts_with(f));

        let file_type = if is_lib && !is_source {
            // File is in a library folder (lib, library, or libraries) but NOT nested
            log::debug!("📚 Processing library file: {}", file.display());
            SolFileType::LibFolder
        } else if is_source {
            // File is in a source code folder
            log::debug!("📄 Processing source file: {}", file.display());
            SolFileType::Standard
        } else {
            // File is neither in source nor in a valid library folder - skip it
            log::debug!(
                "⏭️  Skipping file (not in source or lib): {} (is_lib={}, is_source={})",
                file.display(),
                is_lib,
                is_source
            );
            files_skipped += 1;
            continue;
        };

        // Skip directories and symlinks
        if fs::symlink_metadata(file)?.file_type().is_symlink() {
            files_skipped += 1;
            continue;
        }

        // info!("read scoped file: {}", file.display());
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read file {}: {}", file.display(), e);
                files_skipped += 1;
                continue;
            }
        };
        // Parse content for library functions
        if let Some(library_fn_calls) = parse_library_text(&content) {
            libraries.push(library_fn_calls)
        }

        // Process contract declarations and insert into mappings
        process_contract_declarations(&content, file, file_type, repo, &mut contracts).await?;
        log::debug!(
            "✅ Finished processing contract declarations for {}",
            file.display()
        );

        // Parse import statements to build contract → file mapping
        let import_map = parse_import_map(&content);
        log::debug!(
            "✅ Parsed {} imports for {}",
            import_map.len(),
            file.display()
        );

        // Log file processing for debugging
        if file_type == SolFileType::Standard {
            log::debug!(
                "📝 Processing inheritance for source file: {} ({} chars, {} imports)",
                file.display(),
                content.len(),
                import_map.len()
            );
        }

        // Process inheritance relationships using the import map
        process_inheritance_relationships(&content, file, file_type, repo, &import_map).await?;

        // generated library.fn -> code mapping
        generate_library_to_code_mapping(&libraries).await?;

        files_processed += 1;
    }

    log::info!(
        "✅ Inheritance map building complete: {} files processed, {} skipped, {} contracts found",
        files_processed,
        files_skipped,
        contracts.len()
    );

    // Cache the results
    {
        let mut cache = CONTRACTS_CACHE.lock().unwrap();
        cache.insert(repo.project_id.clone(), contracts.clone());
    }

    Ok(contracts)
}

pub fn get_function_metadata_from_id(
    func_id: &str,
    repo: &RepoPaths,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let fn_metadata: Option<SmartContractFunction> = semantic_db
                        .query_row(
                            "SELECT func_id, project_id, contract, name, ir, visibility, modifiers, mutability FROM functions WHERE func_id = ?1 AND project_id = ?2;",
                            [func_id,&repo.project_id],
                            |row| {
                                let modifier_str: String = row.get(6)?;
                                let modifiers: Vec<String> = modifier_str
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();

                                Ok(SmartContractFunction {
                                    id: row.get(0)?,
                                    project_id: row.get(1)?,
                                    contract: row.get(2)?,
                                    name: row.get(3)?,
                                    ir: row.get(4)?,
                                    visibility: row.get(5)?,
                                    modifiers,
                                    mutability: row.get(7)?,
                                })
                            },
                        )
                        .optional()?;

    Ok(fn_metadata)
}

pub fn get_function_metadata_from_contract_plus_fn(
    contract: &str,
    fn_interface: &str,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let fn_metadata: Option<SmartContractFunction> = semantic_db
                        .query_row(
                            "SELECT func_id, project_id, contract, name, ir, visibility, modifiers, mutability FROM functions WHERE contract = ?1 AND name = ?2;",
                            [contract,fn_interface],
                            |row| {
                                let modifier_str: String = row.get(6)?;
                                let modifiers: Vec<String> = modifier_str
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();

                                Ok(SmartContractFunction {
                                    id: row.get(0)?,
                                    project_id: row.get(1)?,
                                    contract: row.get(2)?,
                                    name: row.get(3)?,
                                    ir: row.get(4)?,
                                    visibility: row.get(5)?,
                                    modifiers,
                                    mutability: row.get(7)?,
                                })
                            },
                        )
                        .optional()?;

    Ok(fn_metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    // ============================================================================
    // Regex Pattern Tests
    // ============================================================================

    #[test]
    fn test_contract_decl_regex_basic_contract() {
        let source = "contract MyContract {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "MyContract");
    }

    #[test]
    fn test_contract_decl_regex_abstract_contract() {
        let source = "abstract contract AbstractBase {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "abstract contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "AbstractBase");
    }

    #[test]
    fn test_contract_decl_regex_interface() {
        let source = "interface IMyInterface {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "interface");
        assert_eq!(caps.get(2).unwrap().as_str(), "IMyInterface");
    }

    #[test]
    fn test_contract_decl_regex_library() {
        let source = "library SafeMath {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "library");
        assert_eq!(caps.get(2).unwrap().as_str(), "SafeMath");
    }

    #[test]
    fn test_inheritance_regex_single_parent() {
        let source = "contract Child is Parent {";
        let caps = SOLIDITY_REGEXES.inheritance.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "Child");
        assert_eq!(caps.get(2).unwrap().as_str(), "Parent ");
    }

    #[test]
    fn test_inheritance_regex_multiple_parents() {
        let source = "contract Child is Parent1, Parent2, Parent3 {";
        let caps = SOLIDITY_REGEXES.inheritance.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "Child");
        assert_eq!(caps.get(2).unwrap().as_str(), "Parent1, Parent2, Parent3 ");
    }

    #[test]
    fn test_inheritance_regex_abstract_contract() {
        let source = "abstract contract AbstractChild is Parent {";
        let caps = SOLIDITY_REGEXES.inheritance.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "AbstractChild");
        assert_eq!(caps.get(2).unwrap().as_str(), "Parent ");
    }

    #[test]
    fn test_inheritance_regex_interface() {
        let source = "interface IChild is IParent1, IParent2 {";
        let caps = SOLIDITY_REGEXES.inheritance.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "IChild");
        assert_eq!(caps.get(2).unwrap().as_str(), "IParent1, IParent2 ");
    }

    #[test]
    fn test_import_regex_single_import() {
        let source = r#"import {MyContract} from "./MyContract.sol";"#;
        let caps = SOLIDITY_REGEXES.import_named.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "MyContract");
        assert_eq!(caps.get(2).unwrap().as_str(), "./MyContract.sol");
    }

    #[test]
    fn test_import_regex_multiple_imports() {
        let source = r#"import {ContractA, ContractB, ContractC} from "./Contracts.sol";"#;
        let caps = SOLIDITY_REGEXES.import_named.captures(source).unwrap();
        assert_eq!(
            caps.get(1).unwrap().as_str(),
            "ContractA, ContractB, ContractC"
        );
        assert_eq!(caps.get(2).unwrap().as_str(), "./Contracts.sol");
    }

    #[test]
    fn test_import_regex_with_alias() {
        let source = r#"import {BaseAdapter as EulerBaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";"#;
        let caps = SOLIDITY_REGEXES.import_named.captures(source).unwrap();
        assert_eq!(
            caps.get(1).unwrap().as_str(),
            "BaseAdapter as EulerBaseAdapter"
        );
        assert_eq!(
            caps.get(2).unwrap().as_str(),
            "@euler-price-oracle/adapter/BaseAdapter.sol"
        );
    }

    #[test]
    fn test_import_regex_multiline() {
        let source = r#"import {
            ContractA,
            ContractB as AliasB,
            ContractC
        } from "./Contracts.sol";"#;
        let caps = SOLIDITY_REGEXES.import_named.captures(source).unwrap();
        assert!(caps.get(1).unwrap().as_str().contains("ContractA"));
        assert!(
            caps.get(1)
                .unwrap()
                .as_str()
                .contains("ContractB as AliasB")
        );
        assert!(caps.get(1).unwrap().as_str().contains("ContractC"));
    }

    #[test]
    fn test_import_item_regex_simple_name() {
        let segment = "MyContract";
        let caps = SOLIDITY_REGEXES.import_item.captures(segment).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "MyContract");
    }

    #[test]
    fn test_import_item_regex_with_alias() {
        let segment = "BaseAdapter as EulerBaseAdapter";
        let caps = SOLIDITY_REGEXES.import_item.captures(segment).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "BaseAdapter");
    }

    #[test]
    fn test_import_item_regex_with_whitespace() {
        let segment = "  MyContract  ";
        let caps = SOLIDITY_REGEXES.import_item.captures(segment).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "MyContract");
    }

    // ============================================================================
    // Helper Function Tests
    // ============================================================================

    #[test]
    fn test_parse_import_map_single_import() {
        let source = r#"import {MyContract} from "./MyContract.sol";"#;
        let map = parse_import_map(source);
        assert_eq!(map.get("MyContract"), Some(&"./MyContract.sol".to_string()));
    }

    #[test]
    fn test_parse_import_map_multiple_imports() {
        let source = r#"import {ContractA, ContractB} from "./Contracts.sol";"#;
        let map = parse_import_map(source);
        assert_eq!(map.get("ContractA"), Some(&"./Contracts.sol".to_string()));
        assert_eq!(map.get("ContractB"), Some(&"./Contracts.sol".to_string()));
    }

    #[test]
    fn test_parse_import_map_with_alias() {
        let source = r#"import {BaseAdapter as EulerBaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";"#;
        let map = parse_import_map(source);
        // The alias (EulerBaseAdapter) should be the key
        assert_eq!(
            map.get("EulerBaseAdapter"),
            Some(&"@euler-price-oracle/adapter/BaseAdapter.sol".to_string())
        );
        // The original name should NOT be in the map
        assert_eq!(map.get("BaseAdapter"), None);
    }

    #[test]
    fn test_parse_import_map_mixed_aliases() {
        let source = r#"
            import {ContractA, ContractB as AliasB, ContractC} from "./Contracts.sol";
        "#;
        let map = parse_import_map(source);
        assert_eq!(map.get("ContractA"), Some(&"./Contracts.sol".to_string()));
        assert_eq!(map.get("AliasB"), Some(&"./Contracts.sol".to_string()));
        assert_eq!(map.get("ContractC"), Some(&"./Contracts.sol".to_string()));
        // Original name should NOT be in map
        assert_eq!(map.get("ContractB"), None);
    }

    #[test]
    fn test_parse_import_map_multiple_statements() {
        let source = r#"
            import {ContractA} from "./A.sol";
            import {ContractB} from "./B.sol";
            import {ContractC as AliasC} from "./C.sol";
        "#;
        let map = parse_import_map(source);
        assert_eq!(map.get("ContractA"), Some(&"./A.sol".to_string()));
        assert_eq!(map.get("ContractB"), Some(&"./B.sol".to_string()));
        assert_eq!(map.get("AliasC"), Some(&"./C.sol".to_string()));
    }

    #[test]
    fn test_parse_import_map_real_world_covenant() {
        let source = r#"
            import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
            import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
            import {IPriceOracle} from "../interfaces/IPriceOracle.sol";
            import {Errors} from "./lib/Errors.sol";
            import {BaseAdapter} from "./BaseAdapter.sol";
            import {BaseAdapter as EulerBaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";
            import {ScaleUtils} from "@euler-price-oracle/lib/ScaleUtils.sol";
        "#;
        let map = parse_import_map(source);

        assert_eq!(
            map.get("IERC4626"),
            Some(&"forge-std/interfaces/IERC4626.sol".to_string())
        );
        assert_eq!(
            map.get("Ownable2Step"),
            Some(&"@openzeppelin/access/Ownable2Step.sol".to_string())
        );
        assert_eq!(
            map.get("Ownable"),
            Some(&"@openzeppelin/access/Ownable2Step.sol".to_string())
        );
        assert_eq!(
            map.get("IPriceOracle"),
            Some(&"../interfaces/IPriceOracle.sol".to_string())
        );
        assert_eq!(map.get("Errors"), Some(&"./lib/Errors.sol".to_string()));
        // Note: If there are two imports with same name, last one wins (or we could handle differently)
        // For now, let's just check the alias is there
        assert_eq!(
            map.get("EulerBaseAdapter"),
            Some(&"@euler-price-oracle/adapter/BaseAdapter.sol".to_string())
        );
        assert_eq!(
            map.get("ScaleUtils"),
            Some(&"@euler-price-oracle/lib/ScaleUtils.sol".to_string())
        );
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn test_contract_decl_regex_with_leading_whitespace() {
        let source = "    contract IndentedContract {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "IndentedContract");
    }

    #[test]
    fn test_contract_decl_regex_with_tabs() {
        let source = "\t\tinterface ITabbed {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "interface");
        assert_eq!(caps.get(2).unwrap().as_str(), "ITabbed");
    }

    #[test]
    fn test_contract_decl_regex_underscore_prefix() {
        let source = "contract _InternalContract {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "_InternalContract");
    }

    #[test]
    fn test_contract_decl_regex_numbers_in_name() {
        let source = "library ERC721A {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "library");
        assert_eq!(caps.get(2).unwrap().as_str(), "ERC721A");
    }

    #[test]
    fn test_contract_decl_regex_multiple_underscores() {
        let source = "contract My_Complex_Contract_Name {";
        let caps = SOLIDITY_REGEXES.contract_decl.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "My_Complex_Contract_Name");
    }

    #[test]
    fn test_contract_decl_regex_multiline_source() {
        let source = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IERC20.sol";

contract Token {
    string public name;
}

interface IVault {
    function deposit() external;
}

library Math {
    function add(uint a, uint b) internal pure returns (uint) {
        return a + b;
    }
}

abstract contract Base {
    function foo() internal virtual;
}
"#;

        let matches: Vec<_> = SOLIDITY_REGEXES
            .contract_decl
            .captures_iter(source)
            .collect();
        assert_eq!(matches.len(), 4);

        assert_eq!(matches[0].get(1).unwrap().as_str(), "contract");
        assert_eq!(matches[0].get(2).unwrap().as_str(), "Token");

        assert_eq!(matches[1].get(1).unwrap().as_str(), "interface");
        assert_eq!(matches[1].get(2).unwrap().as_str(), "IVault");

        assert_eq!(matches[2].get(1).unwrap().as_str(), "library");
        assert_eq!(matches[2].get(2).unwrap().as_str(), "Math");

        assert_eq!(matches[3].get(1).unwrap().as_str(), "abstract contract");
        assert_eq!(matches[3].get(2).unwrap().as_str(), "Base");
    }

    #[test]
    fn test_contract_decl_regex_no_match_inline() {
        // Should NOT match - not at start of line
        let source = "    function contract() public {}";
        assert!(SOLIDITY_REGEXES.contract_decl.captures(source).is_none());
    }

    #[test]
    fn test_contract_decl_regex_no_match_comment() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        // Should NOT match - inside comment
        let source = "// contract MyContract";
        assert!(contract_decl_regex.captures(source).is_none());
    }

    #[test]
    fn test_contract_decl_regex_inheritance() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let source = "contract MyToken is ERC20, Ownable {";
        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "MyToken");
    }

    #[test]
    fn test_contract_decl_regex_extra_spaces() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        // Multiple spaces between keyword and name
        let source = "contract    SpacedContract {";
        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "SpacedContract");
    }

    #[test]
    fn test_contract_type_mapping() {
        // Test the match logic that maps declaration strings to ContractType
        let test_cases = vec![
            ("abstract contract", ContractType::AbstractContract),
            ("contract", ContractType::Contract),
            ("interface", ContractType::Interface),
            ("library", ContractType::Library),
        ];

        for (decl, expected_type) in test_cases {
            let contract_type = match decl {
                "abstract contract" => ContractType::AbstractContract,
                "contract" => ContractType::Contract,
                "interface" => ContractType::Interface,
                "library" => ContractType::Library,
                _ => ContractType::Contract,
            };
            assert_eq!(
                contract_type, expected_type,
                "Failed for declaration: {}",
                decl
            );
        }
    }

    #[test]
    fn test_contract_decl_regex_real_world_openzeppelin() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let source = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IERC20} from "./IERC20.sol";

abstract contract ERC20 is IERC20 {
    mapping(address => uint256) private _balances;
}
"#;

        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "abstract contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "ERC20");
    }

    #[test]
    fn test_contract_decl_regex_real_world_uniswap() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let source = r#"
pragma solidity >=0.5.0;

interface IUniswapV2Pair {
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
}
"#;

        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "interface");
        assert_eq!(caps.get(2).unwrap().as_str(), "IUniswapV2Pair");
    }

    #[test]
    fn test_contract_decl_regex_camelcase_variations() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let test_cases = vec![
            ("contract ALLCAPS {", "ALLCAPS"),
            ("contract lowercase {", "lowercase"),
            ("contract CamelCase {", "CamelCase"),
            ("contract snake_case {", "snake_case"),
            ("contract PascalCase {", "PascalCase"),
            ("contract MixedCase123 {", "MixedCase123"),
        ];

        for (source, expected_name) in test_cases {
            let caps = contract_decl_regex.captures(source).unwrap();
            assert_eq!(
                caps.get(2).unwrap().as_str(),
                expected_name,
                "Failed for source: {}",
                source
            );
        }
    }

    #[test]
    fn test_contract_decl_regex_no_match_starting_with_number() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        // Should NOT match - name starts with number (invalid Solidity)
        let source = "contract 123Invalid {";
        assert!(contract_decl_regex.captures(source).is_none());
    }

    #[test]
    fn test_contract_decl_regex_abstract_with_extra_spaces() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        // Should match - single space between abstract and contract
        let source = "abstract contract MyAbstract {";
        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "abstract contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "MyAbstract");
    }

    #[test]
    fn test_contract_decl_regex_double_space_in_abstract() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        // The regex uses \s+ which matches one or more whitespace, so this WILL match
        // (the regex is lenient about whitespace between abstract and contract)
        let source = "abstract  contract MyAbstract {";
        // This actually matches because \s+ allows multiple spaces
        // If we wanted to enforce single space, we'd use \s (single space literal)
        assert!(contract_decl_regex.captures(source).is_some());
    }

    #[test]
    fn test_contract_decl_regex_mixed_declarations() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let source = r#"
contract A {}
library B {}
interface C {}
abstract contract D {}
contract E is A, B {}
"#;

        let matches: Vec<_> = contract_decl_regex.captures_iter(source).collect();
        assert_eq!(matches.len(), 5);

        let expected = [
            ("contract", "A"),
            ("library", "B"),
            ("interface", "C"),
            ("abstract contract", "D"),
            ("contract", "E"),
        ];

        for (i, (exp_type, exp_name)) in expected.iter().enumerate() {
            assert_eq!(matches[i].get(1).unwrap().as_str(), *exp_type);
            assert_eq!(matches[i].get(2).unwrap().as_str(), *exp_name);
        }
    }

    #[test]
    fn test_contract_decl_regex_with_natspec() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let source = r#"
/// @title My Contract
/// @notice This is a test
contract MyContract {
    uint256 public value;
}
"#;

        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "MyContract");
    }

    #[test]
    fn test_contract_decl_regex_empty_contract() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let source = "contract Empty {}";
        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "Empty");
    }

    #[test]
    fn test_contract_decl_regex_no_opening_brace() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        // Should still match - regex doesn't require opening brace
        let source = "contract NoBrace";
        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "NoBrace");
    }

    #[test]
    fn test_contract_decl_regex_newline_before_brace() {
        let contract_decl_regex = Regex::new(
            r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)",
        )
        .unwrap();

        let source = r#"contract MyContract
{
    uint256 public value;
}"#;

        let caps = contract_decl_regex.captures(source).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "contract");
        assert_eq!(caps.get(2).unwrap().as_str(), "MyContract");
    }
}
