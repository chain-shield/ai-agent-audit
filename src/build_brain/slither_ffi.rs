/// This module provides an interface to the Slither static analysis tool for Solidity.
/// It handles running Slither printers, parsing their output, and extracting useful information
/// such as SlithIR (intermediate representation) and storage variable details.
use anyhow::{anyhow, Result};
use log::{debug, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Global cache keyed by (repo_root, printer) tuple stringified
pub static PRINTER_OUTPUT_CACHE: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents a single function's SlithIR (intermediate representation).
///
/// SlithIR is Slither's intermediate representation of Solidity code, which
/// makes it easier to analyze the code's behavior and identify potential issues.
#[derive(Debug, Serialize, Deserialize)]
pub struct SlithIRFn {
    /// Name of the contract containing this function
    pub contract: String,
    /// Name of the function
    pub function: String,
    /// The SlithIR representation of the function's code
    pub ir: String,
}

/// Represents a storage variable in a Solidity contract.
///
/// This struct contains information about a storage variable, including
/// its name, type, and the contract it belongs to.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageVar {
    /// Name of the contract containing this storage variable
    pub contract: String,
    /// Name of the storage variable
    pub name: String,
    /// Data type of the storage variable (e.g., "uint256", "address", etc.)
    pub r#type: String,
}

// get_IR_of_codebase()

/// Runs a single Slither printer and captures its output.
///
/// This function executes the Slither static analysis tool with a specific printer
/// and returns the captured output as a string.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @param printer - Name of the Slither printer to run (e.g., "slithir-ssa", "variable-order")
/// @return Result containing the printer's output as a string
// TODO - UPDATE TO CACHE based on repo hash/commit hash
async fn run_printer(repo_root: &Path, printer: &str) -> Result<String> {
    let key = cache_key(repo_root, printer);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    // Execute Slither with the specified printer
    let output = Command::new("slither")
        .current_dir(repo_root)
        .args(&[
            ".",
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--foundry-out-directory",  // Specify where to find Forge build artifacts
            "out",
            "--print", // Specify which printer to run
            printer,
            "--disable-color", // Disable ANSI color codes for easier parsing
        ])
        .stdout(Stdio::piped()) // Capture printer text from stdout
        .stderr(Stdio::piped()) // Capture banner & errors from stderr
        .output()?;

    // Use whichever stream is non-empty (some printers output to stdout, others to stderr)
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).into_owned();
    }

    // Ensure we got some output
    if text.trim().is_empty() {
        return Err(anyhow!("Slither ran but produced no `{}` output", printer));
    }

    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
}

/// Parses the output of the Slither 'slithir-ssa' printer into a vector of SlithIRFn structs.
///
/// This function processes the text output from Slither's slithir-ssa printer,
/// which contains the SlithIR representation of functions in Solidity contracts.
/// It extracts the contract name, function name, and IR content for each function.
///
/// @param text - The raw text output from the slithir-ssa printer
/// @return Vector of SlithIRFn structs containing the parsed data
pub fn parse_slithir(text: &str) -> Vec<SlithIRFn> {
    let mut current_contract = String::new();
    let mut current_fn = String::new();
    let mut buf = String::new();
    let mut out = Vec::new();

    // info!("text in parse_slithir {}", text.len());
    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract ") {
            current_contract = line["Contract ".len()..].trim_end_matches(':').to_owned();
        }
        // Parse function lines (format: "\tFunction functionName:")
        else if line.starts_with("\tFunction ") {
            // Flush previous function data if we have any
            if !current_fn.is_empty() {
                let ir_content = replace_special_character(&buf);
                let ir_content_cleaned =
                    ir_content.replace("IRs:\n", "").replace("Expression:", "");
                out.push(SlithIRFn {
                    contract: current_contract.clone(),
                    function: current_fn.clone(),
                    ir: ir_content_cleaned,
                });
            }
            // Extract new function name and reset buffer
            current_fn = line.trim()[9..].trim_end_matches(':').to_owned();
            buf.clear();
        }
        // Parse IR lines (format: "\t\t<ir content>")
        else if line.starts_with("\t\t") {
            buf.push_str(line.trim_start());
            buf.push('\n');
        }
    }

    // Flush the last function after processing all lines
    if !current_fn.is_empty() && !buf.trim().is_empty() && buf.trim().len() > 10 {
        let ir_content = replace_special_character(&buf);
        out.push(SlithIRFn {
            contract: current_contract,
            function: current_fn,
            ir: ir_content,
        });
    } else if !current_fn.is_empty() {
        info!(
            "Skipping empty or small IR for {}::{}",
            current_contract, current_fn
        );
    }

    // info!("functions => {:#?}", out);
    out
}

/// Parses the output of the Slither 'variable-order' printer into a vector of StorageVar structs.
///
/// This function processes the text output from Slither's variable-order printer,
/// which contains information about storage variables in Solidity contracts.
/// It extracts the contract name, variable name, and variable type for each storage variable.
///
/// @param text - The raw text output from the variable-order printer
/// @return Vector of StorageVar structs containing the parsed data
pub fn parse_storage(text: &str) -> Vec<StorageVar> {
    let mut current_contract = String::new();
    let mut vars = Vec::new();

    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract") && line.ends_with(':') {
            current_contract = line["Contract".len()..]
                .trim_end_matches(':')
                .trim()
                .to_owned();
        }
        // Parse variable lines (format: "| <index> | <name> | <type> | <...> |")
        else if line.starts_with('|') && line.contains('|') {
            let cols: Vec<_> = line.split('|').map(|c| c.trim()).collect();
            // Check if this is a valid variable line (has enough columns and not a header)
            if cols.len() >= 3 && cols[1] != "Name" && !cols[1].is_empty() && !cols[2].is_empty() {
                // cols[1] is contract_name.function_name.  need to parse
                let (contract, function) = split_str_by_period(cols[1]).unwrap();
                vars.push(StorageVar {
                    contract,
                    name: function,
                    r#type: cols[2].to_owned(),
                });
            } else {
                debug!(
                    "Skipping invalid storage var in {}: {:?}",
                    current_contract, cols
                );
            }
        }
    }

    // info!("storage => {:#?}", vars.len());
    vars
}
// splits "first.last" => ("first","last")
fn split_str_by_period(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None // Invalid format
    }
}
/// Replces special characters in the SlithIR text with their ASCII equivalents.
///
/// This function replaces the Greek letter phi (ϕ) with the ASCII string "phi"
/// to ensure the text can be properly processed and displayed.
///
/// @param text - The text containing special characters
/// @return String with special characters replaced
pub fn replace_special_character(text: &str) -> String {
    // Replace the Greek letter phi (ϕ) with "phi"
    let cleaned_text = text.trim().replace("ϕ", "phi");

    cleaned_text
}
/// Runs both Slither printers and returns the parsed IR and storage information.
///
/// This function is the main public interface for extracting SlithIR and storage
/// information from Solidity contracts. It runs both the slithir-ssa and variable-order
/// printers and parses their output.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @return Result containing a tuple of SlithIRFn and StorageVar vectors
pub async fn get_ir_and_storage_vars_for_each_function(
    repo_root: &Path,
) -> Result<(Vec<SlithIRFn>, Vec<StorageVar>)> {
    // Run the slithir-ssa printer to get IR information
    let ir_raw = run_printer(repo_root, "slithir-ssa").await?;

    // Run the variable-order printer to get storage information
    let storage_raw = run_printer(repo_root, "variable-order").await?;
    // info!("storage raw => {}", storage_raw);

    // Parse both outputs and return the results
    Ok((parse_slithir(&ir_raw), parse_storage(&storage_raw)))
}

/// Dumps IR and storage information to individual text files in a directory.
///
/// This function extracts SlithIR and storage information from Solidity contracts
/// and writes each function's IR and each storage variable's information to separate
/// text files in the specified directory.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @param dir - Path to the directory where the text files will be written
/// @return Result containing a vector of paths to the created files
pub async fn save_ir_and_storage_vars_to_txt_files(
    repo_root: &Path,
    dir: &Path,
) -> Result<Vec<PathBuf>> {
    // 1 . gather IR + storage  (re-use existing function)
    info!("get ir and storage chunks");
    let (ir_vec, storage_vec) = get_ir_and_storage_vars_for_each_function(repo_root).await?;
    // info!("storage vec => {:?}", storage_vec);

    // 2 . serialise each artefact → one text file
    let mut out_paths = Vec::new();

    info!("convert fn ir to txt files");
    for fn_ir in &ir_vec {
        let meta = format!("IR:{}::{}", fn_ir.contract, fn_ir.function);
        // info!("fn ir meta => {}", meta);

        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, &fn_ir.ir)?;
        out_paths.push(p);
    }

    info!("convert storage vars to txt files");
    for var in &storage_vec {
        let meta = format!("STORAGE:{}::{}", var.contract, var.name);
        // info!("storage meta => {}", meta);
        let body = format!("{} {}", var.name, var.r#type);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("fn ir and storage var files => {:?}", out_paths.len());

    Ok(out_paths)
}

fn cache_key(repo_root: &Path, printer: &str) -> String {
    format!("{}::{}", repo_root.display(), printer)
}
