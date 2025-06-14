use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, default::Default, path::Path, process::Command, sync::Arc};

use crate::build_brain::slither_ffi::{cache_key, PRINTER_OUTPUT_CACHE};

#[derive(Debug, Clone, Default)]
pub struct DotFunc {
    pub full_id: String,  // "3895_changeFeeAddress"
    pub contract: String, // PuppyRaffle
    pub name: String,     // changeFeeAddress
}

#[derive(Debug)]
pub struct DotEdge {
    pub caller: String, // DotFunc.full_id
    pub callee: String,
}

/// Step 1: run Slither and grab the JSON envelope
pub async fn generate_slither_call_graph(repo: &Path) -> Result<String> {
    let key = cache_key(repo, "call-graph");
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    let out = Command::new("slither")
        .current_dir(repo)
        .args([".", "--print", "call-graph", "--json", "-"])
        .output()?;
    anyhow::ensure!(out.status.success(), "slither call-graph failed");

    let text = String::from_utf8_lossy(&out.stdout).into_owned();

    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
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
pub fn parse_dot_blobs(blobs: &[String]) -> Result<(Vec<DotFunc>, Vec<DotEdge>)> {
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
                    caller: e[1].to_string(),
                    callee: e[2].to_string(),
                });
            }
        }
    }
    Ok((funcs.into_values().collect(), edges))
}
