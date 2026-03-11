use crate::build_brain::graph_db::SmartContractFunction;
use crate::cost::cost_data::get_token_count;
use crate::enumerator::parse_solidity::is_standard_library_contract_name;
use crate::enumerator::utils::{
    get_function_metadata_from_contract_plus_fn, get_function_metadata_from_id,
};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::{
    get_function_name_from_func_id, get_function_name_from_interface, string_starts_with_char,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Mutex;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::HashMap;

type InternalCallEdgeMap = HashMap<String, (String, String)>;
type InternalCallEdgeCache = HashMap<String, InternalCallEdgeMap>;

/// Global cache for internal_call_edge_map results.
/// Key format: "ir_block_hash:contract_name"
static INTERNAL_CALL_EDGE_CACHE: Lazy<Mutex<InternalCallEdgeCache>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

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
            check_internal_calls_to_find_contract_of_fn_call(func_id, contract, &parent_func.ir)
                .unwrap_or_default();

        if new_contract.is_empty() || is_standard_library_contract_name(&new_contract) {
            return Ok(None);
        }

        // info!(
        //     "in INTERNAL_CALL found contract: {}, and fn sig {}",
        //     new_contract, fn_sig
        // );

        callee_fn =
            get_function_metadata_from_contract_plus_fn(&new_contract, &fn_sig, semantic_db)?
                .unwrap_or_default();

        // info!(
        //     "found IR for fn sig with token count: {}",
        //     get_token_count(&callee_fn.ir)
        // );

        if callee_fn.id.is_empty() || is_standard_library_contract_name(&callee_fn.contract) {
            return Ok(None);
        }
    } else if is_standard_library_contract_name(&callee_fn.contract) {
        return Ok(None);
    }

    // handle functions called by contract interfaces (starts with 'I')
    let updated_callee_fn = if callee_fn.ir.is_empty()
        && string_starts_with_char(&callee_fn.contract, 'I')
    {
        find_fn_metadata_from_contract_interface_call(&callee_fn, semantic_db)?.unwrap_or(callee_fn)
    } else {
        callee_fn
    };

    if is_standard_library_contract_name(&updated_callee_fn.contract) {
        return Ok(None);
    }

    // handle edge case where contract cannot be found in protocol but Mock exists
    let final_update_callee_fn = if updated_callee_fn.ir.is_empty() {
        find_fn_metadata_from_potential_contract_mock(&updated_callee_fn, semantic_db)?
            .unwrap_or(updated_callee_fn)
    } else {
        updated_callee_fn
    };

    if is_standard_library_contract_name(&final_update_callee_fn.contract) {
        return Ok(None);
    }
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
            // info!(
            //     "no interface connected name found for {}, with fn interface: {}",
            //     new_contract, func.name
            // );
            let next_func = SmartContractFunction {
                contract: new_contract,
                ..func.clone()
            };
            find_fn_metadata_from_contract_interface_call(&next_func, semantic_db)
        } else {
            // info!(
            //     "contract found for {} with IR of token count: {}",
            //     new_contract,
            //     get_token_count(&func_metadata.ir)
            // );
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
/// Results are cached globally to avoid redundant parsing of the same IR blocks.
pub fn internal_call_edge_map(
    ir_block: &str,
    current_contract: &str,
) -> HashMap<String, (String, String)> {
    // Create a cache key from IR block hash and contract name
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    ir_block.hash(&mut hasher);
    current_contract.hash(&mut hasher);
    let cache_key = format!("{}:{}", hasher.finish(), current_contract);

    // Check cache first
    {
        let cache = INTERNAL_CALL_EDGE_CACHE.lock().unwrap();
        if let Some(cached_result) = cache.get(&cache_key) {
            return cached_result.clone();
        }
    }

    // Cache miss - compute the result
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

    // Store in cache
    {
        let mut cache = INTERNAL_CALL_EDGE_CACHE.lock().unwrap();
        cache.insert(cache_key, out.clone());
    }

    out
}
