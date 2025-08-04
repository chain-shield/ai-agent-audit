use anyhow::anyhow;
use anyhow::Result;
use log::info;
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

use crate::llm_review::contract_file_map::insert_contract_to_file_mapping;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::fn_labels::get_modifiers_label;
use crate::utils::fn_labels::get_visibility_label;
use crate::utils::get_fn_name::get_function_name;
use crate::{
    build_brain::{
        self,
        graph_db::SmartContractFunction,
        slither_ffi::{SlithIRFn, StorageVar},
    },
    utils::bpe::get_bpe,
};

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
    let func_name = get_function_name(&func.name);
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

pub async fn get_hashmap_of_contract_to_functions(
    repo: &RepoPaths,
    semantic_db: &Connection,
) -> anyhow::Result<HashMap<String, Vec<SmartContractFunction>>> {
    // find all main contracts for app (ones in /src)
    info!("grabbing all contracts...");
    let contracts_in_src_folder = contracts_in_source_folder(repo).await?;

    let placeholders = contracts_in_src_folder
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(",");

    let mut statement = semantic_db.prepare(&format!(
        "SELECT id, contract, name, ir, visibility, modifiers, mutability FROM functions WHERE contract IN ({})",
        placeholders
    ))?;

    let rows = statement.query_map(params_from_iter(contracts_in_src_folder), |row| {
        // info!("rows => {:#?}", row);
        let modifier_str: String = row.get(4)?;
        let modifiers: Vec<String> = modifier_str
            .split(',')
            .map(|s| s.trim_matches([' ', '\'']).to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(SmartContractFunction {
            id: row.get(0)?,
            contract: row.get(1)?,
            name: row.get(2)?,
            ir: row.get(3)?,
            visibility: row.get(4)?,
            modifiers,
            mutability: row.get(6)?,
        })
    })?;
    let functions_of_contract: Vec<SmartContractFunction> =
        rows.collect::<rusqlite::Result<_>>()?;

    if functions_of_contract.is_empty() {
        return Err(anyhow!("no entry fn found"));
    }
    let mut map: HashMap<String, Vec<SmartContractFunction>> = HashMap::new();

    for func in functions_of_contract {
        map.entry(func.contract.clone()).or_default().push(func)
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

    // info!("ir_map => {:#?}", ir_map);
    Ok(ir_map)
}

async fn get_storage_map(repo: &RepoPaths) -> anyhow::Result<HashMap<String, Vec<StorageVar>>> {
    let (_, storage_vec, _) = build_brain::slither_ffi::get_slither_ir_and_storage(repo).await?;

    let storage_map: HashMap<String, Vec<StorageVar>> = {
        let mut m = HashMap::<String, Vec<StorageVar>>::new();
        for v in storage_vec {
            m.entry(v.contract.clone()).or_default().push(v);
        }
        m
    };
    Ok(storage_map)
}

/// Return the names of all `contract XXX` declarations that sit
/// anywhere under `repo_root/src/`.
pub async fn contracts_in_source_folder(repo: &RepoPaths) -> Result<Vec<String>> {
    if !repo.source_code_folder.exists() {
        anyhow::bail!(
            "no src/ folder found at {},",
            repo.source_code_folder.display()
        );
    }
    // get exclusions if any
    let excluded_folders = repo.excluded_folders.clone().unwrap_or(Vec::new());

    // Regex matches `contract Foo`, ignores `interface` / `library`
    let re = Regex::new(r"(?m)^\s*contract\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let mut contracts = Vec::<String>::new();

    let in_scope_files: &Vec<PathBuf> = &repo
        .sol_files
        .iter()
        .filter(|f| f.starts_with(&repo.source_code_folder))
        .filter(|f| {
            !excluded_folders
                .iter()
                .any(|excluded| f.starts_with(excluded))
        })
        .map(|f| f.to_owned())
        .collect();

    for file in in_scope_files {
        // ✅ is in src ?
        // if !file.starts_with(&src_root) {
        //     continue;
        // }

        // 🚫 Skip if path contains /lib/ or /mock/
        if file.components().any(|comp| {
            let part = comp.as_os_str().to_ascii_lowercase();
            part == "lib"
                || part == "library"
                || part.to_string_lossy().to_ascii_lowercase().contains("mock")
                || part
                    .to_string_lossy()
                    .to_ascii_lowercase()
                    .contains("helper")
        }) {
            continue;
        }
        // Skip directories and symlinks
        if fs::symlink_metadata(file)?.file_type().is_symlink() {
            continue;
        }

        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read file {}: {}", file.display(), e);
                continue;
            }
        };

        for cap in re.captures_iter(&content) {
            if let Some(contract_name) = cap.get(1) {
                let contract = contract_name.as_str();
                if !contract.to_ascii_lowercase().contains("mock") {
                    contracts.push(contract.to_string());

                    // record in contract to file hashmap
                    insert_contract_to_file_mapping(contract, file, repo).await?;
                }
            }
        }
    }
    Ok(contracts)
}

pub fn get_function_metadata_from_id(
    id: &str,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let fn_metadata: Option<SmartContractFunction> = semantic_db
                        .query_row(
                            "SELECT id, contract, name, ir, visibility, modifiers, mutability FROM functions WHERE id = ?1;",
                            [id],
                            |row| {
                                let modifier_str: String = row.get(4)?;
                                let modifiers: Vec<String> = modifier_str
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();

                                Ok(SmartContractFunction {
                                    id: row.get(0)?,
                                    contract: row.get(1)?,
                                    name: row.get(2)?,
                                    ir: row.get(3)?,
                                    visibility: row.get(4)?,
                                    modifiers,
                                    mutability: row.get(6)?,
                                })
                            },
                        )
                        .optional()?;

    Ok(fn_metadata)
}
