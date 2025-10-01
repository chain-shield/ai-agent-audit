use anyhow::anyhow;
use anyhow::Result;
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
use std::path::PathBuf;
use std::sync::Mutex;

use crate::enumerator::libraries::generate_library_to_code_mapping;
use crate::enumerator::libraries::get_library_code_for_library_calls;
use crate::enumerator::libraries::ParsedLibrary;
use crate::llm_review::contract_file_map::insert_contract_to_file_mapping;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::fn_labels::get_modifiers_label;
use crate::utils::fn_labels::get_visibility_label;
use crate::utils::get_fn_name::get_function_name_from_func_id;
use crate::utils::get_fn_name::get_function_name_from_interface;
use crate::utils::parse_library_file::parse_library_text;
use crate::utils::parse_library_file::LibCall;
use crate::{
    build_brain::{
        self,
        graph_db::SmartContractFunction,
        slither_ffi::{SlithIRFn, StorageVar},
    },
    utils::bpe::get_bpe,
};

/// Global cache for get_code_ir_map results.
/// Key: project_id, Value: IR map for that project
static CODE_IR_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<(String, String), SlithIRFn>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Global cache for get_storage_map results.
/// Key: project_id, Value: Storage map for that project
static STORAGE_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, Vec<StorageVar>>>>> =
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

/// Generates a markdown codeblock for a contract's storage layout.
///
/// Retrieves the storage variables for a contract and formats them as a markdown codeblock.
///
/// # Arguments
/// * `contract` - The name of the contract
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<String>` - The generated markdown codeblock
pub async fn generate_code_slice_for_storage(
    contract: &str,
    repo: &RepoPaths,
) -> anyhow::Result<String> {
    let storage_map = get_storage_map(repo).await?;
    let mut storage_slice = String::new();
    if let Some(vars) = storage_map.get(contract) {
        storage_slice.push_str(&format!("### Storage layout ({}) \n\n", contract));
        storage_slice.push_str("```text\n");
        for v in vars {
            storage_slice.push_str(&format!("{} {}\n", v.name, v.r#type));
        }
        storage_slice.push_str("\n```");
    }

    Ok(storage_slice)
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

    let contracts = contracts_in_source_folder(repo, &ContractScope::All).await?;

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
    info!(
        "Cache miss for get_code_ir_map - running Slither for project {}",
        repo.project_id
    );

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

async fn get_storage_map(repo: &RepoPaths) -> anyhow::Result<HashMap<String, Vec<StorageVar>>> {
    // Check cache first
    {
        let cache = STORAGE_MAP_CACHE.lock().unwrap();
        if let Some(cached_map) = cache.get(&repo.project_id) {
            return Ok(cached_map.clone());
        }
    }

    // Cache miss - compute the result
    info!(
        "Cache miss for get_storage_map - running Slither for project {}",
        repo.project_id
    );

    let (_, storage_vec, _) = build_brain::slither_ffi::get_slither_ir_and_storage(repo).await?;

    let storage_map: HashMap<String, Vec<StorageVar>> = {
        let mut m = HashMap::<String, Vec<StorageVar>>::new();
        for v in storage_vec {
            m.entry(v.contract.clone()).or_default().push(v);
        }
        m
    };

    // Store in cache
    {
        let mut cache = STORAGE_MAP_CACHE.lock().unwrap();
        cache.insert(repo.project_id.clone(), storage_map.clone());
    }

    info!(
        "Cached storage map for project {} with {} contracts",
        repo.project_id,
        storage_map.len()
    );
    Ok(storage_map)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContractScope {
    All,
    InScope,
}
/// Return the names of all `contract XXX` declarations that sit
/// anywhere under `repo_root/src/`.
pub async fn contracts_in_source_folder(
    repo: &RepoPaths,
    scope: &ContractScope,
) -> Result<Vec<String>> {
    if !repo.source_code_folder.exists() {
        anyhow::bail!(
            "no src/ folder found at {},",
            repo.source_code_folder.display()
        );
    }
    // get exclusions if any
    let excluded_folders = repo.excluded_folders.clone().unwrap_or(Vec::new());
    let scoped_files = repo.extract_scoped_files()?;
    let mut libraries = Vec::<ParsedLibrary>::new();

    // Regex matches `contract Foo`, or `library FooMath` ignores `interface`
    // let re = Regex::new(r"(?m)^\s*contract\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let re = Regex::new(r"(?m)^\s*(?:contract|library)\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let mut contracts = Vec::<String>::new();

    let in_scope_files: &Vec<PathBuf> =
        if !scoped_files.is_empty() && *scope == ContractScope::InScope {
            &scoped_files
        } else {
            &repo
                .sol_files
                .iter()
                .filter(|f| f.starts_with(&repo.source_code_folder))
                .filter(|f| {
                    *scope == ContractScope::All
                        || !excluded_folders
                            .iter()
                            .any(|excluded| f.starts_with(excluded))
                })
                .map(|f| f.to_owned())
                .collect()
        };

    for file in in_scope_files {
        // ✅ is in src ?
        // if !file.starts_with(&src_root) {
        //     continue;
        // }

        // 🚫 Skip if path contains /lib/ or /mock/
        if scoped_files.is_empty()
            && file.components().any(|comp| {
                let part = comp.as_os_str().to_ascii_lowercase();
                part.to_string_lossy().to_ascii_lowercase().contains("mock")
            })
        {
            continue;
        }
        // Skip directories and symlinks
        if fs::symlink_metadata(file)?.file_type().is_symlink() {
            continue;
        }

        // info!("read scoped file: {}", file.display());
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read file {}: {}", file.display(), e);
                continue;
            }
        };
        // print_first_n_lines(20, &content);
        // parse content for library
        if let Some(library_fn_calls) = parse_library_text(&content) {
            // info!("parsed : {}", library_fn_calls.name);
            libraries.push(library_fn_calls)
        }

        for cap in re.captures_iter(&content) {
            if let Some(contract_name) = cap.get(1) {
                let contract = contract_name.as_str();
                if !contract.to_ascii_lowercase().contains("mock") {
                    contracts.push(contract.to_string());
                    // info!(
                    //     "adding contract {} and file {} to map",
                    //     contract,
                    //     file.display()
                    // );
                    // record in contract to file hashmap
                    insert_contract_to_file_mapping(contract, file, repo).await?;
                }
            }
        }
        // generated library.fn -> code mapping
        generate_library_to_code_mapping(&libraries).await?;
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
