use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::logging::print_first_n_lines;
use crate::utils::remapping::resolve_import_path;
use crate::{
    llm_review::contract_file_map::get_file_from_contract,
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
static SOURCE_DEPENDENCY_CACHE: Lazy<Mutex<HashMap<String, (HashSet<String>, HashSet<String>)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static CONTRACT_TO_SCRIPT_CACHE: Lazy<Mutex<HashMap<String, HashSet<PathBuf>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// # Returns
/// * `HashSet<String>` - contracts & interfaces found in source
pub async fn detect_source_code_dependencies(
    contract: &str,
    repo: &RepoPaths,
) -> Result<(HashSet<String>, HashSet<String>)> {
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

    // info!(
    //     "cache miss for source dependencies of contract {} (repo: {}), analyzing...",
    //     contract,
    //     repo.unique_repo_hash()
    // );

    let mut contracts = HashSet::new();
    let mut interfaces = HashSet::new();
    let mut lib_files = HashSet::new();

    // Get the source file for this contract
    let file = match get_file_from_contract(contract, repo).await {
        Some(f) => f,
        None => {
            info!(
                "could not find file for contract {} to detect dependencies",
                contract
            );
            // Cache the empty result to avoid re-attempting failed lookups
            let mut cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
            cache.insert(cache_key, (HashSet::new(), HashSet::new()));
            return Ok((HashSet::new(), HashSet::new()));
        }
    };

    // Read the source code
    let source_code = match fs::read_to_string(&file).await {
        Ok(content) => content,
        Err(e) => {
            info!(
                "could not read file {} for dependency detection: {}",
                file.display(),
                e
            );
            return Ok((HashSet::new(), HashSet::new()));
        }
    };

    // Parse imports on raw source BEFORE stripping string literals
    {
        // 3. Import statements: import { ContractName } from "..."
        // Note: Handles multiple comma-separated imports correctly via split(',')
        // Supports aliasing: import { X as Y } → record X (ignore alias)
        // If starts with 'I', could be interface OR contract → add to BOTH (defensive)
        // Semicolon removed from regex to support multiline imports; allow no space after 'from'
        let import_regex =
            Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*[\"']([^\"']+)[\"']"#).unwrap();
        let import_item_regex =
            Regex::new(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:(?i:as)\s+[A-Za-z_][A-Za-z0-9_]*)?\s*$")
                .unwrap();
        // info!("READING IMPORT PATH...");
        for cap in import_regex.captures_iter(&source_code) {
            if let Some(imports) = cap.get(1) {
                let import_path = cap.get(2).unwrap().as_str();
                // info!("IMPORT PATH: {}", import_path);
                // Track external library files (imports starting with '@')
                if import_path.starts_with("@") {
                    info!("LIB DEP FOUND: {}", import_path);
                    if let Some(resolved_path) = resolve_import_path(import_path, repo) {
                        info!("RESOLVED PATH: {}", resolved_path);
                        if !should_exclude_this_library(&resolved_path) {
                            info!("ADDING RESOLVED PATH {} to LIB FILES", resolved_path);
                            lib_files.insert(resolved_path);
                        }
                        continue; // We have the file path, skip name extraction
                    } else {
                        // No remapping found - log warning and fall through to extract names
                        log::warn!(
                            "No remapping found for external import '{}' in contract {}",
                            import_path,
                            contract
                        );
                    }
                }

                for import in imports.as_str().split(',') {
                    let segment = import.trim();
                    if segment.is_empty() {
                        continue;
                    }
                    let name = if let Some(m) = import_item_regex.captures(segment) {
                        m.get(1).unwrap().as_str()
                    } else {
                        // Fallback: use the trimmed segment as-is
                        segment
                    };
                    if name.starts_with('I')
                        && name.len() > 1
                        && name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                    {
                        // info!(
                        //     "detected import (interface or contract): {} in {}",
                        //     name, contract
                        // );
                        interfaces.insert(name.to_string()); // Try as interface
                        contracts.insert(name.to_string()); // Also try as contract
                    } else if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                        // info!("detected contract import: {} in {}", name, contract);
                        contracts.insert(name.to_string());
                    }
                }
            } else {
                info!("cound not parse imports for");
                print_first_n_lines(10, &source_code);
            }
        }

        // Also handle simple import statements: import "path"; (no named imports)
        let simple_import_regex = Regex::new(r#"import\s*[\"']([^\"']+)[\"']"#).unwrap();
        for cap in simple_import_regex.captures_iter(&source_code) {
            let import_path = cap.get(1).unwrap().as_str();
            // info!("IMPORT PATH (simple): {}", import_path);
            if import_path.starts_with("@") {
                info!("LIB DEP FOUND (simple): {}", import_path);
                if let Some(resolved_path) = resolve_import_path(import_path, repo) {
                    info!("RESOLVED PATH (simple): {}", resolved_path);
                    if !should_exclude_this_library(&resolved_path) {
                        info!("ADDING RESOLVED PATH {} to LIB FILES", resolved_path);
                        lib_files.insert(resolved_path);
                    }
                } else {
                    log::warn!(
                        "No remapping found for external import '{}' in contract {} (simple)",
                        import_path,
                        contract
                    );
                }
            }
        }
    }

    // Strip comments and string literals to avoid false positives (e.g., "XOR (^)" in comments)
    let source_code = strip_comments_and_strings(&source_code);

    // Collect event and error identifiers so we can exclude them from dependency candidates
    let mut banned_identifiers: HashSet<String> = HashSet::new();

    // Declarations
    let event_decl_regex = Regex::new(r"\bevent\s+([A-Z][A-Za-z0-9_]*)\b").unwrap();
    for cap in event_decl_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            banned_identifiers.insert(name.as_str().to_string());
        }
    }
    let error_decl_regex = Regex::new(r"\berror\s+([A-Z][A-Za-z0-9_]*)\b").unwrap();
    for cap in error_decl_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            banned_identifiers.insert(name.as_str().to_string());
        }
    }

    // Usages (emit/revert) — capture event/error identifiers even if declared elsewhere
    let emit_usage_regex = Regex::new(r"\bemit\s+([A-Z][A-Za-z0-9_]*)\s*\(").unwrap();
    for cap in emit_usage_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            banned_identifiers.insert(name.as_str().to_string());
        }
    }
    let revert_usage_regex = Regex::new(r"\brevert\s+([A-Z][A-Za-z0-9_]*)\s*\(").unwrap();
    for cap in revert_usage_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            banned_identifiers.insert(name.as_str().to_string());
        }
    }

    // 1. Constructor calls: new ContractName(...)
    let new_contract_regex = Regex::new(r"\bnew\s+([A-Z][A-Za-z0-9_]*)\s*\(").unwrap();
    for cap in new_contract_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let contract_name = name.as_str().to_string();
            // info!(
            //     "detected constructor call: new {}(...) in {}",
            //     contract_name, contract
            // );
            contracts.insert(contract_name);
        }
    }

    // 2. Interface/Contract casts: InterfaceName(address)
    // Matches: IERC20(token), IUniswapV2Pair(pair), ImmutableCreate2Factory(addr), etc.
    // If starts with 'I', could be interface OR contract → add to BOTH (defensive)
    let type_cast_regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s*\([^)]*\)").unwrap();
    for cap in type_cast_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            // Filter out common Solidity keywords and built-in types
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected type cast (interface or contract): {}(...) in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!("detected contract cast: {}(...) in {}", type_name, contract);
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // Imports already parsed earlier on raw source before stripping string literals

    // 4. Type declarations: ContractName/InterfaceName public/private/internal variable
    // Matches: IERC20 public token, TSwapPool private pool, ImmutableCreate2Factory immutable factory, etc.
    // Variable names: start with [a-zA-Z_], can contain [a-zA-Z0-9_]*
    // If starts with 'I', could be interface OR contract → add to BOTH (defensive)
    let type_decl_regex = Regex::new(
        r"\b([A-Z][A-Za-z0-9_]*)\s+(?:public|private|internal|immutable|constant)\s+[a-zA-Z_][a-zA-Z0-9_]*",
    )
    .unwrap();
    for cap in type_decl_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected type declaration (interface or contract): {} in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract type declaration: {} in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 5. Array types: ContractName[] or InterfaceName[size]
    // Matches: IVault[] public vaults, TSwapPool[10] pools, etc.
    // Detects BOTH interfaces and contracts
    let array_type_regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s*\[\s*\d*\s*\]").unwrap();
    for cap in array_type_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected array type (interface or contract): {}[] in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract array type: {}[] in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 6. Mapping value types: mapping(... => ContractName/InterfaceName)
    // Matches: mapping(address => IStrategy), mapping(uint => TSwapPool), etc.
    // Detects BOTH interfaces and contracts
    let mapping_value_regex =
        Regex::new(r"mapping\s*\([^)]+\s*=>\s*([A-Z][A-Za-z0-9_]*)\s*\)").unwrap();
    for cap in mapping_value_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected mapping value (interface or contract): mapping(...=> {}) in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract mapping value: mapping(...=> {}) in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 7. Mapping key types: mapping(ContractName/InterfaceName => ...)
    // Matches: mapping(IToken => uint256), mapping(TSwapPool => address), etc.
    // Detects BOTH interfaces and contracts
    let mapping_key_regex = Regex::new(r"mapping\s*\(\s*([A-Z][A-Za-z0-9_]*)\s*=>").unwrap();
    for cap in mapping_key_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected mapping key (interface or contract): mapping({} => ...) in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract mapping key: mapping({} => ...) in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 8. Function parameters: function foo(ContractName/InterfaceName param, ...)
    // Matches ALL parameters: function swap(IERC20 tokenIn, IPool pool, TSwapPool factory)
    // First extracts function signatures, then parses parameters within them
    // This avoids false positives from variable declarations and struct fields
    let function_sig_regex =
        Regex::new(r"function\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(([^)]*)\)").unwrap();
    let param_type_regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s+[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    for func_match in function_sig_regex.captures_iter(&source_code) {
        if let Some(params) = func_match.get(1) {
            // Now parse parameter types within this function signature only
            for param_cap in param_type_regex.captures_iter(params.as_str()) {
                if let Some(name) = param_cap.get(1) {
                    let type_name = name.as_str();
                    if !is_solidity_builtin(type_name) {
                        if type_name.starts_with('I')
                            && type_name.len() > 1
                            && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                        {
                            // info!(
                            //     "detected function parameter (interface or contract): {} in {}",
                            //     type_name, contract
                            // );
                            interfaces.insert(type_name.to_string()); // Try as interface
                            contracts.insert(type_name.to_string()); // Also try as contract
                        } else {
                            // info!(
                            //     "detected contract function parameter: {} in {}",
                            //     type_name, contract
                            // );
                            contracts.insert(type_name.to_string());
                        }
                    }
                }
            }
        }
    }

    // 9. Function return types: returns (ContractName/InterfaceName)
    // Matches: returns (IVault vault), returns (TSwapPool), etc.
    // Detects BOTH interfaces and contracts
    let return_type_regex = Regex::new(r"returns\s*\([^)]*\b([A-Z][A-Za-z0-9_]*)\b").unwrap();
    for cap in return_type_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected return type (interface or contract): {} in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract return type: {} in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 10. Struct fields: struct Data { ContractName/InterfaceName field; }
    // Matches ALL fields with semicolons: struct PoolData { IERC20 token; TSwapPool pool; }
    // First extracts full struct definitions, then parses fields within them
    // Uses semicolon to properly delimit field boundaries and handles multi-line structs
    let struct_block_regex = Regex::new(r"struct\s+[A-Z][A-Za-z0-9_]*\s*\{([^}]+)\}").unwrap();
    let struct_field_regex =
        Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s+[a-zA-Z_][a-zA-Z0-9_]*\s*;").unwrap();
    for struct_match in struct_block_regex.captures_iter(&source_code) {
        if let Some(struct_body) = struct_match.get(1) {
            // Now find all field types within this struct body
            for field_cap in struct_field_regex.captures_iter(struct_body.as_str()) {
                if let Some(name) = field_cap.get(1) {
                    let type_name = name.as_str();
                    if !is_solidity_builtin(type_name) {
                        if type_name.starts_with('I')
                            && type_name.len() > 1
                            && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                        {
                            // info!(
                            //     "detected struct field (interface or contract): {} in {}",
                            //     type_name, contract
                            // );
                            interfaces.insert(type_name.to_string()); // Try as interface
                            contracts.insert(type_name.to_string()); // Also try as contract
                        } else {
                            // info!(
                            //     "detected contract struct field: {} in {}",
                            //     type_name, contract
                            // );
                            contracts.insert(type_name.to_string());
                        }
                    }
                }
            }
        }
    }

    // Validate discovered names against known mappings to avoid false positives
    let mut filtered_contracts: HashSet<String> = HashSet::new();
    for c in &contracts {
        // Exclude events and errors discovered by regex
        if banned_identifiers.contains(c) {
            continue;
        }
        if let Some(path) = get_file_from_contract(c, repo).await {
            // Drop standard libraries by name or by path
            if is_standard_library_contract_name(c)
                || path_is_standard_lib(&path)
                || is_standard_interface_name(c)
            {
                continue;
            }
            filtered_contracts.insert(c.clone());
        } else {
            info!(
                "ignoring source dependency candidate '{}' (no contract file found)",
                c
            );
        }
    }

    let mut filtered_interfaces: HashSet<String> = HashSet::new();
    for i in &interfaces {
        if is_standard_interface_name(i) {
            continue;
        }
        // Exclude events and errors discovered by regex
        if banned_identifiers.contains(i) {
            continue;
        }
        if get_file_from_contract(i, repo).await.is_some() {
            filtered_interfaces.insert(i.clone());
        } else {
            info!(
                "ignoring source dependency candidate '{}' (no interface file found)",
                i
            );
        }
    }

    // info!(
    //     "cached source dependencies for contract {} (repo: {}): {} contracts, {} interfaces",
    //     contract,
    //     repo.unique_repo_hash(),
    //     filtered_contracts.len(),
    //     filtered_interfaces.len()
    // );

    let mut filtered_sources = HashSet::new();
    filtered_sources.extend(filtered_contracts);
    filtered_sources.extend(filtered_interfaces);

    // Cache the result before returning
    {
        let mut cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
        cache.insert(cache_key, (filtered_sources.clone(), lib_files.clone()));
    }

    Ok((filtered_sources, lib_files))
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

fn should_exclude_this_library(file_path: &str) -> bool {
    file_path.contains("openzeppelin")
        || file_path.contains("forge-std")
        || file_path.contains("ds-test")
        || file_path.contains("erc4626-tests")
        || file_path.contains("halmos-cheatcodes")
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
fn is_solidity_builtin(name: &str) -> bool {
    let name_lower = name.to_lowercase();

    // Exact matches for common types (check lowercase version)
    if matches!(
        name_lower.as_str(),
        // Solidity built-in types (always lowercase in actual code)
        "string" | "bytes" | "address" | "uint" | "int" | "bool" |
        // Memory location keywords
        "memory" | "storage" | "calldata"
    ) {
        return true;
    }

    // Common OpenZeppelin/library names (PascalCase - check original name)
    if matches!(
        name,
        "Math"
            | "SafeMath"
            | "Strings"
            | "Arrays"
            | "EnumerableSet"
            | "EnumerableMap"
            | "Counters"
            | "SafeCast"
            | "SignedMath"
            | "Checkpoints"
            | "Context"
            | "Ownable"
    ) {
        return true;
    }

    // Pattern matches for sized types (check lowercase version)
    // uint8, uint16, uint24, ..., uint256
    if name_lower.starts_with("uint") && name_lower.len() > 4 {
        if let Ok(_) = name_lower[4..].parse::<u16>() {
            return true;
        }
    }

    // int8, int16, int24, ..., int256
    if name_lower.starts_with("int") && name_lower.len() > 3 {
        if let Ok(_) = name_lower[3..].parse::<u16>() {
            return true;
        }
    }

    // bytes1, bytes2, ..., bytes32
    if name_lower.starts_with("bytes") && name_lower.len() > 5 {
        if let Ok(n) = name_lower[5..].parse::<u8>() {
            return n >= 1 && n <= 32;
        }
    }

    false
}

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
