use crate::build_brain::graph_db::SmartContractFunction;
/// This module provides functionality for generating code slices from smart contract
/// analysis seeds. It traverses the contract call graph to create comprehensive
/// markdown codeblocks containing relevant code, IR, and storage information.
use crate::enumerator::codeblock_cache::{get_cached_codeblock, set_codeblock_cache};
use crate::enumerator::codeblock_db::MarkdownCodeblock;
use crate::enumerator::utils::{
    generate_code_slice_for_storage, generate_codeblock_for_function,
    get_function_metadata_from_id, get_hashmap_of_contract_to_functions,
    get_token_count_of_function_ir,
};

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::{HashSet, VecDeque};
use std::path::Path;
use uuid::Uuid;

use super::codeblock_db::CodeBlocksDb;

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
pub async fn generate_codeblock_from_codebase(
    repo_root: &Path,
    semantic_db: &Connection,
    codeblock_db: &CodeBlocksDb,
    max_depth: usize,
    token_budget: usize,
) -> Result<()> {
    //extract the contract, and all its functions, the slither seed file references
    // the slither issue may be scoped to 1 function in 1 contract, however we pull the
    // ENTIRE contract so there is more context for llm
    log::info!("getting contract to func mapping");
    let contract_to_func_map = get_hashmap_of_contract_to_functions(repo_root, semantic_db)?;

    for (contract, functions_of_contract) in contract_to_func_map {
        // Check if codeblock already generated for this seed

        log::info!("contract => {:#?}", contract);
        log::info!("fn count of contract => {:#?}", functions_of_contract.len());
        if let Some(_) = get_cached_codeblock(&contract).await {
            // Save seed-to-codeblock mapping in the database
            continue;
        };
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
            info!("contract {} / func {} added...", func.contract, func.name);

            all_funcs_connected_to_contract.push(func.clone());

            // get token count of new fn + IR + storage
            let token_count_fn_ir_storage =
                get_token_count_of_function_ir(&func, repo_root).await?;
            info!("token_count_fn_ir_storage => {}", token_count_fn_ir_storage);

            // check budget, make sure not exceeding token context window
            if token_count + token_count_fn_ir_storage > token_budget {
                break; // budget exhausted
            }

            // update token count
            token_count += token_count_fn_ir_storage;

            info!("token_count => {}", token_count);
            if depth < max_depth {
                let mut statement =
                    semantic_db.prepare("SELECT callee FROM edges WHERE caller = ?1;")?;
                let rows = statement.query_map([&func.id], |r| r.get::<_, String>(0))?;
                for callee in rows.flatten() {
                    let callee_fn = get_function_metadata_from_id(&callee, semantic_db)?;
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

        let codeblock = MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            contract: contract.clone(),
            tokens: token_count,
            content: markdown_codeblock_for_llm,
        };
        // 4. store
        codeblock_db.insert_codeblock(&codeblock)?;

        // save to cache
        set_codeblock_cache(&contract, &codeblock).await;

        info!("codeblock => {:#?}", codeblock);
    }

    Ok(())
}
