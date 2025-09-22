use crate::build_brain::graph_db::SmartContractFunction;
use crate::cost::cost_data::get_token_count;
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
use crate::utils::get_fn_name::{
    get_function_name_from_func_id, get_function_name_from_interface, string_starts_with_char,
};
use once_cell::sync::Lazy;
use regex::Regex;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::{HashMap, HashSet, VecDeque};
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
        // Check if codeblock already generated
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
                    if let Some(callee_fn) = robust_extract_fn_metadata_from_func_id(
                        &callee,
                        &func,
                        &contract,
                        semantic_db,
                        repo,
                    )? {
                        frontier.push_back((callee_fn, depth + 1));
                    }
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
        // let mut library_calls = Vec::<LibCall>::new();
        for func in &all_funcs_connected_to_contract {
            let function_ir_code = generate_codeblock_for_function(func, repo).await?;
            // check function IR for LIBRARY_CALL
            // library_calls.extend(collect_library_calls(&function_ir_code));
            markdown_codeblock_for_llm.push_str(&function_ir_code);
            markdown_codeblock_for_llm.push('\n');
        }

        // Deduplicate library calls (by library + canonical_sig)
        // let total_occurrences = library_calls.len();
        // let mut seen_pairs = _HashSet::new();
        // library_calls.retain(|c| seen_pairs.insert((c.library.clone(), c.canonical_sig.clone())));
        // info!(
        //     "library calls found: occurrences={}, unique_functions={}",
        //     total_occurrences,
        //     library_calls.len()
        // );

        // // extract IR for library calls if any unique calls remain
        // if !library_calls.is_empty() {
        //     let code_for_library_calls = generate_library_funcs_markdown(&library_calls).await;
        //     // Only append if we actually have emitted function bodies
        //     if code_for_library_calls.contains("************ CODE FOR ") {
        //         info!(
        //             "adding {} library function code snippets to codeblock",
        //             library_calls.len()
        //         );
        //         print_first_n_lines(50, &code_for_library_calls);
        //         // visual separation before library section
        //         markdown_codeblock_for_llm.push_str("\n\n---\n\n");
        //         markdown_codeblock_for_llm.push_str(&code_for_library_calls);
        //     } else {
        //         info!("no library code available to append (no matches in mapping)");
        //     }
        // } else {
        //     info!("no library calls detected in IR; skipping library section");
        // }

        info!(
            "markdown codeblock token count ==> {:#?}",
            get_token_count(&markdown_codeblock_for_llm)
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

pub fn robust_extract_fn_metadata_from_func_id(
    func_id: &str,
    parent_func: &SmartContractFunction,
    contract: &str,
    semantic_db: &Connection,
    repo: &RepoPaths,
) -> Result<Option<SmartContractFunction>> {
    let mut callee_fn =
        get_function_metadata_from_id(func_id, repo, semantic_db)?.unwrap_or_default();

    // if unwarp was not successful
    if callee_fn.id.is_empty() {
        // info!("could not find caller to {}", &func_id);

        let (new_contract, fn_sig) =
            check_internal_calls_to_find_contract_of_fn_call(func_id, &contract, &parent_func.ir)
                .unwrap_or_default();

        if new_contract.is_empty() {
            return Ok(None);
        }

        info!(
            "in INTERNAL_CALL found contract: {}, and fn sig {}",
            new_contract, fn_sig
        );

        callee_fn =
            get_function_metadata_from_contract_plus_fn(&new_contract, &fn_sig, semantic_db)?
                .unwrap_or_default();

        info!(
            "found IR for fn sig with token count: {}",
            get_token_count(&callee_fn.ir)
        );

        if callee_fn.id.is_empty() {
            return Ok(None);
        }
    }

    // handle functions called by contract interfaces (starts with 'I')
    let updated_callee_fn = if callee_fn.ir.is_empty()
        && string_starts_with_char(&callee_fn.contract, 'I')
    {
        find_fn_metadata_from_contract_interface_call(&callee_fn, semantic_db)?.unwrap_or(callee_fn)
    } else {
        callee_fn
    };

    // handle edge case where contract cannot be found in protocol but Mock exists
    let final_update_callee_fn = if updated_callee_fn.ir.is_empty() {
        find_fn_metadata_from_potential_contract_mock(&updated_callee_fn, semantic_db)?
            .unwrap_or(updated_callee_fn)
    } else {
        updated_callee_fn
    };

    Ok(Some(final_update_callee_fn))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallEdge {
    /// Internal call; `contract` is None if unqualified (assume current contract)
    Internal {
        contract: Option<String>,
        fn_sig: String,
    },
}

// if contract name in function is I<contract_name> or II<contract_name>
// extract contract_name and see if can use contract_name + function name to find IR
fn find_fn_metadata_from_contract_interface_call(
    func: &SmartContractFunction,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    if string_starts_with_char(&func.contract, 'I') {
        // info!(
        //     "looking for contract name {} from interface {}...",
        //     func.contract, func.name
        // );
        let mut new_contract = func.contract.clone();
        new_contract.remove(0);
        let func_metadata =
            get_function_metadata_from_contract_plus_fn(&new_contract, &func.name, semantic_db)?
                .unwrap_or_default();

        if func_metadata.id.is_empty() {
            // recursive call incase contract name has MULTIPLE I's (ie. II<contract_name>)
            info!(
                "no interface connected name found for {}, with fn interface: {}",
                new_contract, func.name
            );
            let next_func = SmartContractFunction {
                contract: new_contract,
                ..func.clone()
            };
            find_fn_metadata_from_contract_interface_call(&next_func, semantic_db)
        } else {
            info!(
                "contract found for {} with IR of token count: {}",
                new_contract,
                get_token_count(&func_metadata.ir)
            );
            Ok(Some(func_metadata))
        }
    } else {
        // EDGE CASE: check if mock function exits
        let func_metadata = find_fn_metadata_from_potential_contract_mock(func, semantic_db)?;
        Ok(func_metadata)
    }
}

// see if <contract_name>Mock exists
fn find_fn_metadata_from_potential_contract_mock(
    func: &SmartContractFunction,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let new_contract = format!("{}{}", func.contract, "Mock");

    // info!(
    //     "looking for mock of contract {} with fn interface: {}...",
    //     func.contract, func.name
    // );
    let func_metadata =
        get_function_metadata_from_contract_plus_fn(&new_contract, &func.name, semantic_db)?
            .unwrap_or_default();

    if func_metadata.id.is_empty() {
        return Ok(None);
    }

    info!(
        "mock contract found for {} with ir of token count: {}",
        new_contract,
        get_token_count(&func_metadata.ir)
    );
    Ok(Some(func_metadata))
}

pub fn check_internal_calls_to_find_contract_of_fn_call(
    func_id: &str,
    contract: &str,
    ir_of_parent_fn: &str,
) -> Option<(String, String)> {
    // extract function name from func_id (i.e. 124_func => func)
    let fn_name = get_function_name_from_func_id(func_id);

    // get internal call fn -> contract mapping
    let fn_to_contract_map = internal_call_edge_map(ir_of_parent_fn, contract);
    fn_to_contract_map.get(&fn_name).cloned()
}

/// Parse a single SlithIR SSA line for INTERNAL_(DYNAMIC_)CALL and LIBRARY_CALL.
/// Returns a normalized call edge with canonical signature "name(type1,type2)".
pub fn parse_call_edge_line(line: &str) -> Option<CallEdge> {
    // INTERNAL_CALL with qualifier: Contract.fn(types)(args...)
    static RE_INTERNAL_QUAL: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"INTERNAL_(?:DYNAMIC_)?CALL\s*,\s*([A-Za-z_]\w*)\.([A-Za-z_]\w*)\(([^)]*)\)"#)
            .unwrap()
    });

    // INTERNAL_CALL without qualifier: fn(types)(args...)  (assume same contract)
    static RE_INTERNAL_UNQUAL: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"INTERNAL_(?:DYNAMIC_)?CALL\s*,\s*([A-Za-z_]\w*)\(([^)]*)\)"#).unwrap()
    });

    if let Some(c) = RE_INTERNAL_QUAL.captures(line) {
        let contract = c.get(1).unwrap().as_str().to_string();
        let name = c.get(2).unwrap().as_str();
        let params = c.get(3).map(|m| m.as_str()).unwrap_or("").trim();
        let canonical_sig = format!("{}({})", name, params);
        return Some(CallEdge::Internal {
            contract: Some(contract),
            fn_sig: canonical_sig,
        });
    }

    if let Some(c) = RE_INTERNAL_UNQUAL.captures(line) {
        let name = c.get(1).unwrap().as_str();
        let params = c.get(2).map(|m| m.as_str()).unwrap_or("").trim();
        let canonical_sig = format!("{}({})", name, params);
        return Some(CallEdge::Internal {
            contract: None,
            fn_sig: canonical_sig,
        });
    }

    None
}

/// Collect call edges from a whole IR block; supply `current_contract` to resolve unqualified INTERNAL_CALLs.
pub fn internal_call_edge_map(
    ir_block: &str,
    current_contract: &str,
) -> HashMap<String, (String, String)> {
    // Returns fn_name -> (callee_contract, fn_name). Libraries will use library name as "contract".
    let mut out = HashMap::new();
    for line in ir_block.lines() {
        if let Some(edge) = parse_call_edge_line(line) {
            match edge {
                CallEdge::Internal { contract, fn_sig } => {
                    let callee_contract = contract.unwrap_or_else(|| current_contract.to_string());
                    let fn_name = get_function_name_from_interface(&fn_sig);
                    out.insert(fn_name, (callee_contract, fn_sig));
                }
            }
        }
    }
    out
}
