/// Call graph analysis and DOT format parsing.
///
/// This module processes Slither's call graph output in DOT format, extracting
/// function relationships and building traversable graph structures for code
/// slice generation and dependency analysis.
use anyhow::Result;
use regex::Regex;
use rusqlite::Connection;
use serde::Deserialize;
use std::{collections::HashMap, default::Default, path::Path};

use crate::{
    enumerator::utils::get_function_metadata_from_id,
    prepare_code::git_clone::RepoPaths,
    utils::fn_labels::{get_modifiers_label, get_visibility_label},
};

use super::{graph_db::SmartContractFunction, slither_ffi::run_printer_json};

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
    let json = run_printer_json(repo, "call-graph").await?;
    let blobs = extract_dot_blobs(&json)?;
    parse_dot_blobs(&blobs, repo)
}

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
