use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{path::Path, process::Command, sync::Arc};

use crate::build_brain::slither_ffi::run_printer;

/// What we keep for every function
#[derive(Debug, Deserialize, Clone)]
pub struct FnSummary {
    pub contract: String,
    pub name: String,
    pub visibility: String, // external / public / internal / private
    #[serde(default)]
    pub modifiers: Vec<String>, // e.g. ["onlyOwner","nonReentrant"]
    #[serde(default)]
    pub mutability: String, // view / pure / payable / nonpayable
}

#[derive(Debug, Deserialize)]
struct PrinterRaw {
    printer: String,
    elements: String, // table text
}
/* ------------------------------------------------------------------------- */
/* 1. Run Slither with the `function-summary` printer                         */
/* ------------------------------------------------------------------------- */

/// Execute Slither and return raw JSON
// async fn run_function_summary(repo_root: &Path) -> Result<String> {
//     let key = cache_key(repo_root, "function-summary");
//     let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
//     let mut printer_cache = cache.lock().await;
//
//     let out = Command::new("slither")
//         .current_dir(repo_root)
//         .args([
//             ".", // analyse current dir
//             "--foundry-ignore-compile",
//             "--foundry-out-directory",
//             "out",
//             "--print",
//             "function-summary",
//             "--json",
//             "-", // dump JSON to stdout
//         ])
//         .output()?;
//
//     if !out.status.success() {
//         return Err(anyhow!(
//             "slither function-summary failed:\n{}",
//             String::from_utf8_lossy(&out.stderr)
//         ));
//     }
//     let fn_summary_json = String::from_utf8_lossy(&out.stdout).into_owned();
//     // Save to cache and return
//     printer_cache.insert(key, fn_summary_json.clone());
//     Ok(fn_summary_json)
// }

fn parse_table(block: &str) -> Vec<FnSummary> {
    let mut out = Vec::new();

    let mut lines = block.lines();
    let mut current_line = lines.next().unwrap_or_default();
    let mut inside_fn_block = false;

    // skip any lines before first table heading
    while !current_line.starts_with("Contract ") {
        current_line = lines.next().unwrap_or("EOF");

        // if file is empty!
        if current_line == "EOF" {
            return Vec::new();
        }
    }

    // grab first contract
    let mut current_contract = get_contract_name(current_line);
    current_line = lines.next().unwrap_or_default();

    while current_line != "EOF" {
        // grab functions
        if current_line.starts_with("Contract ") && !current_line.starts_with("Contract vars") {
            // new contract
            current_contract = get_contract_name(current_line);
            current_line = lines.next().unwrap_or("EOF");
            continue;
        } else if current_line.is_empty() {
            inside_fn_block = false;
            current_line = lines.next().unwrap_or("EOF");
            continue;
        }

        if current_line.starts_with("|") {
            let cols: Vec<_> = current_line.split('|').map(|c| c.trim()).collect();
            if cols.len() < 4 || cols[1].is_empty() || cols[1] == "Modifiers" {
                current_line = lines.next().unwrap_or("EOF");
                continue; // header or empty
            } else if cols[1] == "Function" {
                // new fn table found!
                inside_fn_block = true;
                current_line = lines.next().unwrap_or("EOF");
                continue;
            }

            if inside_fn_block {
                // cols[1]   Function
                // cols[2]   Visibility
                // cols[3]   Modifiers
                let func_name = cols[1];
                let modifiers = if cols[3] == "[]" {
                    vec![]
                } else {
                    // remove '[' and ']'
                    let raw = cols[3].to_string();
                    let trimmed: String = raw.chars().skip(1).take(raw.len() - 2).collect();
                    trimmed
                        .split(',')
                        .map(|s| s.trim_matches([' ', '\'']).to_string())
                        .collect::<Vec<_>>()
                };
                out.push(FnSummary {
                    contract: current_contract.clone(),
                    name: func_name.to_string(),
                    visibility: cols[2].to_string(),
                    modifiers,
                    mutability: String::new(), // not present in this table
                });
            }
        }

        // go to next line
        current_line = lines.next().unwrap_or("EOF");
    }

    // log::info!("function summaries ==> {:#?}", out);
    out
}

pub fn get_contract_name(line: &str) -> String {
    // grab first contract
    log::info!("line with Contract => {}", line);
    line.split_ascii_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string()
}

pub async fn get_function_summaries(repo_root: &Path) -> Result<Vec<FnSummary>> {
    // 1. run slither
    let raw = run_printer(repo_root, "function-summary").await?;

    Ok(parse_table(&raw))
}
