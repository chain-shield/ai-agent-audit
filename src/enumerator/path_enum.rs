use crate::build_brain::graph_db::SmartContractFunction;
/// This module provides functionality for generating code slices from smart contract
/// analysis seeds. It traverses the contract call graph to create comprehensive
/// markdown codeblocks containing relevant code, IR, and storage information.
use crate::build_brain::slither_ffi::{SlithIRFn, StorageVar};
use crate::enumerator::codeblock_cache::{get_cached_codeblock, set_codeblock_cache};
use crate::static_scanning::seed_db::Seed;
use crate::utils::bpe::get_bpe;

use super::slice_db::{MarkdownCodeblock, SeedSlice, SliceDb};
use crate::build_brain;
use anyhow::{anyhow, Result};
use log::info;
use regex::Regex;
use rusqlite::{Connection, OptionalExtension};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use uuid::Uuid;

/// Generates a markdown codeblock from a Slither analysis seed.
///
/// This function performs the following steps:
/// 1. Checks if the codeblock is already cached to avoid redundant processing
/// 2. Extracts the contract and all its functions from the seed file
/// 3. Performs a breadth-first search (BFS) through the call graph up to max_depth
/// 4. Assembles a markdown codeblock containing:
///    - Storage layout for each contract
///    - SlithIR representation for each function
/// 5. Saves the generated codeblock to the database and cache
///
/// # Arguments
/// * `repo_root` - Path to the repository root
/// * `seed` - The Slither analysis seed containing vulnerability information
/// * `semantic_db` - Database connection containing semantic information about the contracts
/// * `slice_db` - Database for storing generated code slices
/// * `max_depth` - Maximum depth for BFS traversal of the call graph
/// * `token_budget` - Maximum token count for the generated codeblock
///
/// # Returns
/// * `Result<()>` - Ok if successful, Error otherwise
pub async fn generate_codeblock_from_slither_seed(
    repo_root: &Path,
    seed: &Seed,
    semantic_db: &Connection,
    slice_db: &SliceDb,
    max_depth: usize,
    token_budget: usize,
) -> Result<()> {
    // Check if codeblock already generated for this seed
    if let Some(codeblock) = get_cached_codeblock(&seed.file).await {
        // Save seed-to-codeblock mapping in the database
        save_seed_slice_to_db(&seed.id, &codeblock.id, slice_db)?;
        return Ok(());
    };

    //extract the contract, and all its functions, the slither seed file references
    // the slither issue may be scoped to 1 function in 1 contract, however we pull the
    // ENTIRE contract so there is more context for llm
    let functions_of_contract = get_contract_and_its_functions_from_seed_file(seed, semantic_db)?;

    // 2. BFS until depth / token budget
    let mut frontier: VecDeque<(SmartContractFunction, usize)> = VecDeque::new();
    for func in functions_of_contract {
        frontier.push_back((func, 0_usize))
    }
    let mut visited = HashSet::new();
    let mut contracts = HashSet::new();
    let mut all_funcs_connected_to_contract = Vec::<SmartContractFunction>::new();
    let mut token_count = 0_usize;

    while let Some((func, depth)) = frontier.pop_front() {
        if !visited.insert(func.id.clone()) {
            continue;
        }

        //keep track of unique contract traversed in BPS
        contracts.insert(func.contract.clone());

        all_funcs_connected_to_contract.push(func.clone());

        // get token count of new fn + IR + storage
        let token_count_fn_ir_storage = get_token_count_of_function_ir(&func, repo_root).await?;
        // info!("token_count_fn_ir_storage => {}", token_count_fn_ir_storage);

        // check budget, make sure not exceeding token context window
        if token_count + token_count_fn_ir_storage > token_budget {
            break; // budget exhausted
        }

        // update token count
        token_count += token_count_fn_ir_storage;

        // info!("token_count => {}", token_count);
        if depth < max_depth {
            let mut statement =
                semantic_db.prepare("SELECT callee FROM edges WHERE caller = ?1;")?;
            let rows = statement.query_map([&func.id], |r| r.get::<_, String>(0))?;
            for callee in rows.flatten() {
                let callee_fn: Option<SmartContractFunction> = semantic_db
                    .query_row(
                        "SELECT id, contract, name FROM functions WHERE id = ?1;",
                        [&callee],
                        |row| {
                            Ok(SmartContractFunction {
                                id: row.get(0)?,
                                contract: row.get(1)?,
                                name: row.get(2)?,
                            })
                        },
                    )
                    .optional()?;
                let Some(callee_fn) = callee_fn else {continue};

                frontier.push_back((callee_fn, depth + 1));
            }
        }
    }

    // ── 3.  Assemble final Markdown body ────────────────────────────
    let mut markdown_codeblock_for_llm = String::new();
    // list storage vars
    for contract in &contracts {
        let storage_var_ir = generate_code_slice_for_storage(contract, repo_root).await?;
        // loop through and add all functions of contract
        markdown_codeblock_for_llm.push_str(&storage_var_ir);
        markdown_codeblock_for_llm.push('\n');
    }
    for func in &all_funcs_connected_to_contract {
        let function_ir_code = generate_codeblock_for_function(func, repo_root).await?;
        markdown_codeblock_for_llm.push_str(&function_ir_code);
        markdown_codeblock_for_llm.push('\n');
    }
    info!(
        "markdown codeblock size ==> {:#?}",
        markdown_codeblock_for_llm.len()
    );

    let md_codeblock_id = Uuid::new_v4().to_string();

    save_seed_slice_to_db(&seed.id, &md_codeblock_id, slice_db)?;

    let codeblock = MarkdownCodeblock {
        id: md_codeblock_id,
        tokens: token_count,
        content: markdown_codeblock_for_llm,
    };
    // 4. store
    slice_db.insert_codeblock(&codeblock)?;

    // save to cache
    set_codeblock_cache(&seed.file, &codeblock).await;

    // info!("codeblock => {:#?}", codeblock);

    Ok(())
}

/// Saves a mapping between a seed and a codeblock to the database.
///
/// Creates a new SeedSlice entry with a unique ID and inserts it into the database.
///
/// # Arguments
/// * `seed_id` - The ID of the seed
/// * `codeblock_id` - The ID of the codeblock
/// * `slice_db` - The database to save the mapping to
///
/// # Returns
/// * `Result<()>` - Ok if successful, Error otherwise
fn save_seed_slice_to_db(seed_id: &str, codeblock_id: &str, slice_db: &SliceDb) -> Result<()> {
    let seed_slice = SeedSlice {
        id: Uuid::new_v4().to_string(),
        codeblock_id: codeblock_id.to_string(),
        seed_id: seed_id.to_string(),
        status: "NEW".into(),
    };

    slice_db.insert_seed_slice(&seed_slice)?;
    info!("seed slice => {}", seed_slice.id);

    Ok(())
}

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
async fn generate_codeblock_for_function(
    func: &SmartContractFunction,
    repo: &Path,
) -> anyhow::Result<String> {
    let ir_map = get_code_ir_map(repo).await?;
    let mut function_slice = String::new();
    if let Some(ir) = ir_map.get(&(func.contract.clone(), func.name.clone())) {
        function_slice.push_str(&format!("#### {}\n", ir.function));
        function_slice.push_str("```slithir\n");
        function_slice.push_str(&ir.ir);
        function_slice.push_str("\n```");
    }

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
async fn generate_code_slice_for_storage(contract: &str, repo: &Path) -> anyhow::Result<String> {
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
fn get_contract_and_its_functions_from_seed_file(
    seed: &Seed,
    semantic_db: &Connection,
) -> anyhow::Result<Vec<SmartContractFunction>> {
    let filename = Path::new(&seed.file)
        .file_stem() // "PuppyRaffle.sol" → "PuppyRaffle"
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("invalid seed.file"))?;

    let mut statement =
        semantic_db.prepare("SELECT id, contract, name FROM functions WHERE contract LIKE ?1")?;

    let rows = statement.query_map([format!("%{}%", filename)], |row| {
        Ok(SmartContractFunction {
            id: row.get(0)?,
            contract: row.get(1)?,
            name: row.get(2)?,
        })
    })?;
    let functions_of_contract: Vec<SmartContractFunction> =
        rows.collect::<rusqlite::Result<_>>()?;

    if functions_of_contract.is_empty() {
        return Err(anyhow!("no entry fn found for seed {}", seed.id));
    }

    Ok(functions_of_contract)
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
async fn get_token_count_of_function_ir(
    func: &SmartContractFunction,
    repo: &Path,
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
async fn get_code_ir_map(repo: &Path) -> anyhow::Result<HashMap<(String, String), SlithIRFn>> {
    // Regex to extract function name from full signature (e.g., "Contract.function(args)")
    let extract_function_name = Regex::new(r#"[A-Za-z0-9$_]+\.([A-Za-z0-9$_]+)\([^)]*\)"#)?;

    // Get IR and storage variables from Slither
    let (ir_vec, _) =
        build_brain::slither_ffi::get_ir_and_storage_vars_for_each_function(repo).await?;

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

    Ok(ir_map)
}

async fn get_storage_map(repo: &Path) -> anyhow::Result<HashMap<String, Vec<StorageVar>>> {
    let (_, storage_vec) =
        build_brain::slither_ffi::get_ir_and_storage_vars_for_each_function(repo).await?;

    let storage_map: HashMap<String, Vec<StorageVar>> = {
        let mut m = HashMap::<String, Vec<StorageVar>>::new();
        for v in storage_vec {
            m.entry(v.contract.clone()).or_default().push(v);
        }
        m
    };
    Ok(storage_map)
}
