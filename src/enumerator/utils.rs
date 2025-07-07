use std::{collections::HashMap, path::Path};

use anyhow::anyhow;
use anyhow::Result;
use log::info;
use regex::Regex;
use rusqlite::params_from_iter;
use rusqlite::{Connection, OptionalExtension};
use std::fs;
use walkdir::WalkDir;

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

/// Generates a markdown codeblock for a specific function.
///
/// Retrieves the SlithIR representation of the function and formats it as a markdown codeblock.
///
/// # Arguments
/// * `func` - The smart contract function to generate a codeblock for
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<String>` - The generated markdown codeblock
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

/// Extracts all functions from a contract referenced in a seed file.
///
/// Queries the semantic database to find all functions belonging to the contract
/// mentioned in the seed file.
///
/// # Arguments
/// * `seed` - The Slither analysis seed
/// * `semantic_db` - Database connection containing semantic information about the contracts
///
/// # Returns
/// * `anyhow::Result<Vec<SmartContractFunction>>` - List of functions in the contract
// pub fn get_contract_and_its_functions_from_seed_file(
//     file: &str, // src/PupplyRaffle
//     semantic_db: &Connection,
// ) -> anyhow::Result<Vec<SmartContractFunction>> {
//     let filename = Path::new(file)
//         .file_stem() // "PuppyRaffle.sol" → "PuppyRaffle"
//         .and_then(|s| s.to_str())
//         .ok_or_else(|| anyhow!("invalid file"))?;
//
//     let mut statement =
//         semantic_db.prepare("SELECT id, contract, name FROM functions WHERE contract LIKE ?1")?;
//
//     let rows = statement.query_map([format!("%{}%", filename)], |row| {
//         Ok(SmartContractFunction {
//             id: row.get(0)?,
//             contract: row.get(1)?,
//             name: row.get(2)?,
//         })
//     })?;
//     let functions_of_contract: Vec<SmartContractFunction> =
//         rows.collect::<rusqlite::Result<_>>()?;
//
//     if functions_of_contract.is_empty() {
//         return Err(anyhow!("no entry fn found for file {}", file));
//     }
//
//     Ok(functions_of_contract)
// }

pub fn get_hashmap_of_contract_to_functions(
    repo: &RepoPaths,
    semantic_db: &Connection,
) -> anyhow::Result<HashMap<String, Vec<SmartContractFunction>>> {
    // find all main contracts for app (ones in /src)
    info!("grabbing all contracts...");
    let contracts_in_src_folder = contracts_in_src(repo)?;

    let placeholders = contracts_in_src_folder
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(",");

    // info!("placeholders => {:#?}", placeholders);

    let mut statement = semantic_db.prepare(&format!(
        "SELECT id, contract, name, visibility, modifiers, mutability FROM functions WHERE contract IN ({})",
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
            visibility: row.get(3)?,
            modifiers,
            mutability: row.get(5)?,
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

    // info!("map ==> {:#?}", map);
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
async fn get_code_ir_map(repo: &RepoPaths) -> anyhow::Result<HashMap<(String, String), SlithIRFn>> {
    // Regex to extract function name from full signature (e.g., "Contract.function(args)")
    let extract_function_name = Regex::new(r#"[A-Za-z0-9$_]+\.([A-Za-z0-9$_]+)\([^)]*\)"#)?;

    // Get IR and storage variables from Slither
    let (ir_vec, _, _) = build_brain::slither_ffi::get_slither_metadata_and_issues(repo).await?;

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
    let (_, storage_vec, _) =
        build_brain::slither_ffi::get_slither_metadata_and_issues(repo).await?;

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
pub fn contracts_in_src(repo: &RepoPaths) -> Result<Vec<String>> {
    let src_root = repo.root.join(repo.repo_name.clone()).join("src");
    if !src_root.exists() {
        anyhow::bail!("no src/ folder found at {}", src_root.display());
    }

    // Regex matches `contract Foo`, ignores `interface` / `library`
    let re = Regex::new(r"(?m)^\s*contract\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let mut out = Vec::<String>::new();

    for entry in WalkDir::new(&src_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sol"))
    {
        // Skip directories and symlinks
        if fs::symlink_metadata(entry.path())?.file_type().is_symlink() {
            continue;
        }

        let content = fs::read_to_string(entry.path())?;
        for cap in re.captures_iter(&content) {
            out.push(cap[1].to_string());
        }
    }
    Ok(out)
}

pub fn get_function_metadata_from_id(
    id: &str,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let fn_metadata: Option<SmartContractFunction> = semantic_db
                        .query_row(
                            "SELECT id, contract, name, visibility, modifiers, mutability FROM functions WHERE id = ?1;",
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
                                    visibility: row.get(3)?,
                                    modifiers,
                                    mutability: row.get(5)?,
                                })
                            },
                        )
                        .optional()?;

    Ok(fn_metadata)
}
