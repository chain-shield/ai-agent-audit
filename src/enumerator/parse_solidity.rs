use crate::llm_review::contract_file_map::get_file_from_contract;
use crate::prepare_code::git_clone::RepoPaths;
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Mutex;
use tokio::fs;

use anyhow::Result;
use log::info;
use std::collections::{HashMap, HashSet};

/// Global cache for source code dependency detection results.
/// Key format: "repo_hash:contract_name"
/// Value: (HashSet<contracts>, HashSet<interfaces>)
static SOURCE_DEPENDENCY_CACHE: Lazy<Mutex<HashMap<String, HashSet<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// # Returns
/// * `(HashSet<String>, HashSet<String>)` - (contracts, interfaces) found in source
pub async fn detect_source_code_dependencies(
    contract: &str,
    repo: &RepoPaths,
) -> Result<HashSet<String>> {
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
            cache.insert(cache_key, HashSet::new());
            return Ok(HashSet::new());
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
            return Ok(HashSet::new());
        }
    };
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

    // 3. Import statements: import { ContractName } from "..."
    // Note: Handles multiple comma-separated imports correctly via split(',')
    // If starts with 'I', could be interface OR contract → add to BOTH (defensive)
    let import_regex = Regex::new(r#"import\s*\{([^}]+)\}\s*from"#).unwrap();
    for cap in import_regex.captures_iter(&source_code) {
        if let Some(imports) = cap.get(1) {
            for import in imports.as_str().split(',') {
                let name = import.trim();
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
        }
    }

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
        cache.insert(cache_key, filtered_sources.clone());
    }

    Ok(filtered_sources)
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
