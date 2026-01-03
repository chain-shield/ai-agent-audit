/// Function summarization using Slither analysis.
///
/// This module extracts function metadata from Slither's function summary printer,
/// parsing visibility, modifiers, and mutability information for all functions
/// across contracts in a repository.
use anyhow::Result;
use serde::Deserialize;

use crate::{
    build_brain::slither_ffi::{run_printer, run_printer_monorepo},
    prepare_code::git_clone::RepoPaths,
};

/// Comprehensive metadata for a smart contract function
#[derive(Debug, Deserialize, Clone)]
pub struct FnSummary {
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
    /// Function visibility (external/public/internal/private)
    pub visibility: String,
    /// Applied modifiers (e.g., ["onlyOwner", "nonReentrant"])
    #[serde(default)]
    pub modifiers: Vec<String>,
    /// State mutability (view/pure/payable/nonpayable)
    #[serde(default)]
    pub mutability: String,
}

/// Parses Slither's function summary table format into structured data.
///
/// Processes the text output from Slither's function-summary printer,
/// extracting function metadata for each contract in the repository.
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

pub async fn get_function_summaries(repo: &RepoPaths) -> Result<Vec<FnSummary>> {
    let raw = match repo.monorepo_folders {
        Some(_) => run_printer_monorepo(repo, "function-summary").await?,
        None => run_printer(repo, "function-summary", None).await?,
    };
    Ok(parse_table(&raw))
}
