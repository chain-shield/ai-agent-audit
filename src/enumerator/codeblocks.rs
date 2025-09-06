use crate::build_brain::graph_db::SmartContractFunction;
/// Intelligent code slicing for focused AI analysis.
///
/// This module generates contextual code blocks by traversing call graphs and
/// assembling relevant code, IR, and storage information within token budgets
/// for optimal LLM analysis.
use crate::enumerator::codeblock_cache::{get_cached_codeblock, set_codeblock_cache};
use crate::enumerator::codeblock_db::MarkdownCodeblock;
use crate::enumerator::utils::{
    generate_code_slice_for_storage, generate_codeblock_for_function,
    get_function_metadata_from_contract_plus_fn, get_function_metadata_from_id,
    get_hashmap_of_contract_to_functions, get_token_count_of_function_ir,
};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::string_starts_with_char;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::{HashSet, VecDeque};
use uuid::Uuid;

use super::codeblock_db::CodeBlocksDb;

/// Generates contextual code blocks for each contract using call graph traversal.
///
/// This function creates focused code slices by:
/// 1. Checking cache to avoid redundant processing
/// 2. Performing breadth-first search through call graphs up to max_depth
/// 3. Respecting token budgets for LLM context limits
/// 4. Assembling markdown with storage layouts and SlithIR representations
/// 5. Caching results for efficient reprocessing
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantic_db` - Database containing call graph and function data
/// * `codeblock_db` - Database for storing generated code blocks
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `Result<()>` - Success or error
pub async fn generate_codeblock_from_codebase(
    repo: &RepoPaths,
    semantic_db: &Connection,
    codeblock_db: &CodeBlocksDb,
    max_depth: usize,
    token_budget: usize,
) -> Result<()> {
    log::info!("getting contract to func mapping");
    let contract_to_func_map = get_hashmap_of_contract_to_functions(repo, semantic_db).await?;

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
            // info!("contract {} / func {} added...", func.contract, func.name);

            all_funcs_connected_to_contract.push(func.clone());

            // get token count of new fn + IR + storage
            let token_count_fn_ir_storage = get_token_count_of_function_ir(&func, repo).await?;
            // info!("token_count_fn_ir_storage => {}", token_count_fn_ir_storage);

            // check budget, make sure not exceeding token context window
            if token_count + token_count_fn_ir_storage > token_budget {
                break; // budget exhausted
            }

            // update token count
            token_count += token_count_fn_ir_storage;

            // info!("token_count => {}", token_count);
            if depth < max_depth {
                let mut statement = semantic_db
                    .prepare("SELECT callee FROM edges WHERE caller = ?1 AND project_id = ?2;")?;
                let rows =
                    statement.query_map([&func.id, &repo.project_id], |r| r.get::<_, String>(0))?;
                for callee in rows.flatten() {
                    let callee_fn_option =
                        get_function_metadata_from_id(&callee, repo, semantic_db)?;
                    let Some(callee_fn) = callee_fn_option else { continue };
                    // if callee_fn.ir.is_empty() && callee_fn.contract starts with I THEN see if
                    // you can find without I
                    let updated_callee_fn = if callee_fn.ir.is_empty()
                        && string_starts_with_char(&callee_fn.contract, 'I')
                    {
                        // remove 'I' from contract name
                        let mut new_contract = callee_fn.contract.clone();
                        new_contract.remove(0);
                        // extract real fn + ir not just interface
                        let new_callee_option = get_function_metadata_from_contract_plus_fn(
                            &new_contract,
                            &callee_fn.name,
                            semantic_db,
                        )?;
                        if let Some(new_callee) = new_callee_option {
                            new_callee
                        } else {
                            callee_fn.clone()
                        }
                    } else {
                        callee_fn.clone()
                    };

                    frontier.push_back((updated_callee_fn, depth + 1));
                }
            }
        }

        // ── 3.  Assemble final Markdown body ────────────────────────────
        let mut markdown_codeblock_for_llm = String::new();
        // list storage vars
        for contract in &contracts {
            let storage_var_ir = generate_code_slice_for_storage(contract, repo).await?;
            // loop through and add all functions of contract
            markdown_codeblock_for_llm.push_str(&storage_var_ir);
            markdown_codeblock_for_llm.push('\n');
        }
        for func in &all_funcs_connected_to_contract {
            let function_ir_code = generate_codeblock_for_function(func, repo).await?;
            markdown_codeblock_for_llm.push_str(&function_ir_code);
            markdown_codeblock_for_llm.push('\n');
        }
        info!(
            "markdown codeblock size ==> {:#?}",
            markdown_codeblock_for_llm.len()
        );

        let codeblock = MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: contract.clone(),
            tokens: token_count,
            content: markdown_codeblock_for_llm,
        };
        // 4. store
        codeblock_db.insert_codeblock(&codeblock)?;

        // save to cache
        set_codeblock_cache(&contract, &codeblock).await;

        // if codeblock.contract.contains("LaunchpadV2Pair") {
        //     info!("CONTRACT => {}", codeblock.contract);
        //     info!("codeblock => {}", codeblock.content);
        // }
    }

    Ok(())
}
