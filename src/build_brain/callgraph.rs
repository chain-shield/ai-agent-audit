/// Call graph analysis and DOT format parsing.
///
/// This module processes Slither's call graph output in DOT format, extracting
/// function relationships and building traversable graph structures for code
/// slice generation and dependency analysis.
use anyhow::Result;
use log::warn;
use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::Connection;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    default::Default,
    path::Path,
    sync::Mutex,
};

use crate::{
    build_brain::{inheritance, slither_ffi},
    enumerator::utils::get_function_metadata_from_id,
    prepare_code::git_clone::RepoPaths,
    utils::{
        check_folder_name::contains_build_config,
        fn_labels::{get_modifiers_label, get_visibility_label},
    },
};

use super::graph_db::SmartContractFunction;

/// Global cache for inheritance map.
/// Key: project_id, Value: child → parents mapping
static INHERITANCE_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, Vec<String>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Global cache for inheritance map.
/// Key: project_id, Value: parent → children mapping
static INVERTED_INHERITANCE_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, Vec<String>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
/// Represents a function node in the call graph

#[derive(Debug, Clone, Default)]
pub struct DotFunc {
    /// Unique identifier from Slither (e.g., "3895_changeFeeAddress")
    pub full_id: String,
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
}

/// Represents a call relationship between two functions
#[derive(Debug)]
pub struct DotEdge {
    pub project_id: String,
    /// Calling function's full_id
    pub caller: String,
    /// Called function's full_id
    pub callee: String,
}

/// Extracts call graph functions and edges from Slither analysis.
///
/// This function orchestrates the complete call graph extraction process by
/// running Slither's call-graph printer and parsing the resulting DOT format.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `(Vec<DotFunc>, Vec<DotEdge>)` - Functions and their call relationships
pub async fn get_dot_funcs_and_dot_edges(repo: &RepoPaths) -> Result<(Vec<DotFunc>, Vec<DotEdge>)> {
    // let json = run_printer_json(repo, "call-graph", None).await?;
    // let blobs = extract_dot_blobs(&json)?;
    let blobs = generate_call_graph_blobs(&repo).await?;
    parse_dot_blobs(&blobs, repo)
}

pub async fn generate_call_graph_blobs(repo: &RepoPaths) -> Result<Vec<String>> {
    let folders = repo.extract_monorepo_folders()?;

    if folders.is_empty() {
        let json = slither_ffi::run_printer_json(&repo, "call-graph", None).await?;
        Ok(extract_dot_blobs(&json)?)
    } else {
        let mut total_output = Vec::<String>::new();
        for folder in folders {
            if contains_build_config(&folder) {
                let json = slither_ffi::run_printer_json(&repo, "call-graph", Some(folder)).await?;
                let blobs = extract_dot_blobs(&json)?;
                if !blobs.is_empty() {
                    total_output.extend(blobs);
                }
            }
        }
        Ok(total_output)
    }
}

/// Generates inheritance edges (child → parent) for all contracts in the repository.
///
/// Uses custom inheritance printer that includes all project code (including lib folders
/// like euler-price-oracle) while excluding only standard libraries (forge-std,
/// openzeppelin-contracts, solady). This overrides any slither.config.json exclusions.
///
/// # Returns
/// * `Vec<(String, String)>` - List of (child, parent) tuples representing inheritance edges
// pub async fn generate_inheritance_edges(repo: &RepoPaths) -> Result<Vec<(String, String)>> {
//     let folders = repo.extract_monorepo_folders()?;
//
//     if folders.is_empty() {
//         // Use custom inheritance printer with standard library filtering
//         let json = slither_ffi::run_printer_json_inheritance(&repo, None).await?;
//         Ok(inheritance::parse_inheritance_json(&json)?)
//     } else {
//         let mut total_output = Vec::<(String, String)>::new();
//         for folder in folders {
//             if contains_build_config(&folder) {
//                 // Use custom inheritance printer with standard library filtering
//                 let json = slither_ffi::run_printer_json_inheritance(&repo, Some(folder)).await?;
//                 let edges = inheritance::parse_inheritance_json(&json)?;
//                 if !edges.is_empty() {
//                     total_output.extend(edges);
//                 }
//             }
//         }
//         Ok(total_output)
//     }
// }
//
// /// Get all parent contracts for a given child contract (direct parents only).
// ///
// /// # Arguments
// /// * `child` - The child contract name
// /// * `inheritance_map` - The inheritance map from get_inheritance_map()
// ///
// /// # Returns
// /// * `Vec<String>` - List of direct parent contracts (empty if no parents)
// pub async fn _get_parents(child_contract: &str, repo: &RepoPaths) -> Result<Vec<String>> {
//     let inheritance_map = get_inheritance_map(repo).await?;
//     if let Some(parents) = inheritance_map.get(child_contract) {
//         Ok(parents.clone())
//     } else {
//         Ok(Vec::new())
//     }
// }
//
// pub async fn _get_children(parent_contract: &str, repo: &RepoPaths) -> Result<Vec<String>> {
//     let inheritance_map = get_inverted_inheritance_map(repo).await?;
//     if let Some(children) = inheritance_map.get(parent_contract) {
//         Ok(children.clone())
//     } else {
//         Ok(Vec::new())
//     }
// }
//
// /// Retrieves a cached inheritance map: child contract → list of parent contracts.
// /// Results are cached globally per project to avoid redundant Slither calls.
// ///
// /// # Arguments
// /// * `repo` - Repository paths and metadata
// ///
// /// # Returns
// /// * `HashMap<String, Vec<String>>` - Map of child contract to all its parent contracts
// pub async fn _get_inheritance_map(repo: &RepoPaths) -> Result<HashMap<String, Vec<String>>> {
//     // Check cache first
//     {
//         let cache = INHERITANCE_MAP_CACHE.lock().unwrap();
//         if let Some(cached_map) = cache.get(&repo.project_id) {
//             return Ok(cached_map.clone());
//         }
//     }
//
//     // Cache miss - compute the result
//     // log::info!(
//     //     "Cache miss for get_inheritance_map - running Slither for project {}",
//     //     repo.project_id
//     // );
//
//     // Get inheritance edges (child, parent) tuples
//     let edges = generate_inheritance_edges(repo).await?;
//
//     // Build child → parents map
//     let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
//     for (child, parent) in edges {
//         inheritance_map
//             .entry(child)
//             .or_insert_with(Vec::new)
//             .push(parent);
//     }
//
//     // Store in cache
//     {
//         let mut cache = INHERITANCE_MAP_CACHE.lock().unwrap();
//         cache.insert(repo.project_id.clone(), inheritance_map.clone());
//     }
//
//     log::info!(
//         "Cached inheritance map for project {} with {} contracts",
//         repo.project_id,
//         inheritance_map.len()
//     );
//
//     Ok(inheritance_map)
// }
//
// // Build parent → children map
// pub async fn get_inverted_inheritance_map(
//     repo: &RepoPaths,
// ) -> Result<HashMap<String, Vec<String>>> {
//     // Check cache first
//     {
//         let cache = INVERTED_INHERITANCE_MAP_CACHE.lock().unwrap();
//         if let Some(cached_map) = cache.get(&repo.project_id) {
//             return Ok(cached_map.clone());
//         }
//     }
//
//     // Get inheritance edges (child, parent) tuples
//     let edges = generate_inheritance_edges(repo).await?;
//     let mut tmp: HashMap<String, HashSet<String>> = HashMap::new();
//     for (child, parent) in edges {
//         if child == "IPriceOracle" || parent == "IPriceOracle" {
//             warn!("round 1: key: {}, inserting : {}", parent, child);
//         }
//         tmp.entry(parent).or_default().insert(child);
//     }
//
//     // Dedup + deterministic order
//     let mut inverted_inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
//     for (parent, children_set) in tmp {
//         let mut children: Vec<_> = children_set.into_iter().collect();
//         if parent == "IPriceOracle" {
//             warn!("parent: {}, children: {:?}", parent, children);
//         }
//         children.sort_unstable();
//         inverted_inheritance_map.insert(parent, children);
//     }
//
//     // cache
//     INVERTED_INHERITANCE_MAP_CACHE
//         .lock()
//         .unwrap()
//         .insert(repo.project_id.clone(), inverted_inheritance_map.clone());
//
//     log::info!(
//         "Cached inverted inheritance map for project {} with {} parent entries",
//         repo.project_id,
//         inverted_inheritance_map.len()
//     );
//     // Build parent → children map
//
//     Ok(inverted_inheritance_map)
// }
// /// Get all ancestors up to a specified depth (parents, grandparents, etc.).
// ///
// /// # Arguments
// /// * `child` - The child contract name
// /// * `inheritance_map` - The inheritance map from get_inheritance_map()
// /// * `max_depth` - Maximum depth to traverse (1 = direct parents only, 2 = parents + grandparents)
// ///
// /// # Returns
// /// * `Vec<String>` - List of all ancestor contracts up to max_depth (deduplicated)
// pub fn get_ancestors(
//     child: &str,
//     inheritance_map: &HashMap<String, Vec<String>>,
//     max_depth: usize,
// ) -> Vec<String> {
//     let mut ancestors = Vec::new();
//     let mut visited = std::collections::HashSet::new();
//     let mut current_level = vec![child.to_string()];
//
//     for _ in 0..max_depth {
//         let mut next_level = Vec::new();
//
//         for contract in &current_level {
//             if let Some(parents) = inheritance_map.get(contract) {
//                 for parent in parents {
//                     if visited.insert(parent.clone()) {
//                         ancestors.push(parent.clone());
//                         next_level.push(parent.clone());
//                     }
//                 }
//             }
//         }
//
//         if next_level.is_empty() {
//             break;
//         }
//
//         current_level = next_level;
//     }
//
//     ancestors
// }
//
/// Step 2: pull every DOT file’s `content` string
pub fn extract_dot_blobs(json: &str) -> Result<Vec<String>> {
    #[derive(Deserialize)]
    struct DotFile {
        #[serde(rename = "type")]
        _ty: String,
        name: DotName,
    }
    #[derive(Deserialize)]
    struct DotName {
        content: String,
    }
    #[derive(Deserialize)]
    struct Printer {
        elements: Vec<DotFile>,
    }
    #[derive(Deserialize)]
    struct Root {
        results: Results,
    }
    #[derive(Deserialize)]
    struct Results {
        #[serde(default)]
        printers: Vec<Printer>,
    }

    let root: Root = serde_json::from_str(json)?;
    let mut out = Vec::new();
    for printer in root.results.printers {
        for file in printer.elements {
            out.push(file.name.content);
        }
    }
    Ok(out)
}

/// Step 3: regex-scan DOT text → nodes & edges
pub fn parse_dot_blobs(blobs: &[String], repo: &RepoPaths) -> Result<(Vec<DotFunc>, Vec<DotEdge>)> {
    let node_re = Regex::new(r#""(\d+)_([A-Za-z0-9$_]+)" \[label"#)?;
    let edge_re = Regex::new(r#""(\d+_[^"]+)" -> "(\d+_[^"]+)""#)?;
    let cluster_re = Regex::new(r#"cluster_(\d+)_([A-Za-z0-9$_]+) \{"#)?;
    let mut funcs = HashMap::<String, DotFunc>::new();
    let mut edges = Vec::<DotEdge>::new();

    for blob in blobs {
        let mut contract = String::new();
        for line in blob.lines() {
            if let Some(c) = cluster_re.captures(line) {
                contract = c[2].to_string(); // e.g., PuppyRaffle
            }
            if let Some(c) = node_re.captures(line) {
                let full = c[1].to_string() + "_" + &c[2];
                let func = DotFunc {
                    full_id: full.clone(),
                    contract: contract.clone(),
                    name: c[2].to_string(),
                };
                funcs.entry(full).or_insert(func);
            }
            if let Some(e) = edge_re.captures(line) {
                edges.push(DotEdge {
                    project_id: repo.project_id.clone(),
                    caller: e[1].to_string(),
                    callee: e[2].to_string(),
                });
            }
        }
    }
    Ok((funcs.into_values().collect(), edges))
}

pub async fn get_enriched_funcs_and_edges(
    repo: &RepoPaths,
    semantic_path: &Path,
) -> Result<String> {
    let mut enriched_edges = Vec::<DotEdge>::new();
    let mut enriched_funcs = Vec::<DotFunc>::new();
    let semantic_db = Connection::open(semantic_path)?;

    let (funcs, edges) = get_dot_funcs_and_dot_edges(repo).await?;

    for edge in edges {
        let enriched_callee = match get_function_metadata_from_id(&edge.callee, repo, &semantic_db)?
        {
            Some(callee_fn) => generated_enriched_fn_label(&edge.callee, callee_fn),
            None => edge.callee,
        };
        let enriched_caller = match get_function_metadata_from_id(&edge.caller, repo, &semantic_db)?
        {
            Some(callee_fn) => generated_enriched_fn_label(&edge.caller, callee_fn),
            None => edge.caller,
        };
        enriched_edges.push(DotEdge {
            project_id: repo.project_id.clone(),
            callee: enriched_callee,
            caller: enriched_caller,
        });
    }

    for func in funcs {
        let enriched_func_name =
            match get_function_metadata_from_id(&func.full_id, repo, &semantic_db)? {
                Some(full_func) => generated_enriched_fn_label(&func.name, full_func),
                None => func.name,
            };
        enriched_funcs.push(DotFunc {
            full_id: func.full_id,
            contract: func.contract,
            name: enriched_func_name,
        });
    }

    // log::info!("enriched edges => {:#?}", enriched_edges);

    let funcs_string: String = enriched_funcs
        .iter()
        .map(|f| {
            format!(
                "id: {}, contract: {}, name: {}",
                f.full_id, f.contract, f.name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let edges_string: String = enriched_edges
        .iter()
        .map(|e| format!("{} -> {}", e.caller, e.callee))
        .collect::<Vec<_>>()
        .join("\n");

    let mut final_dot_string = String::new();

    final_dot_string.push_str("\n\n### Functions\n\n");
    final_dot_string.push_str(&funcs_string);
    final_dot_string.push_str("\n\n### Dot Edges (Caller -> Callee)\n\n");
    final_dot_string.push_str(&edges_string);

    // log::info!("final dot string => {}", final_dot_string);
    Ok(final_dot_string)
}

fn generated_enriched_fn_label(fn_id: &str, fn_metadata: SmartContractFunction) -> String {
    let visibility = get_visibility_label(&fn_metadata.visibility);
    let modifiers = get_modifiers_label(&fn_metadata.modifiers);

    format!("{} {}{}", fn_id, visibility, modifiers)
}
