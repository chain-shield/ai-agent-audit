/// Slither static analyzer interface with Docker integration.
///
/// This module provides a secure interface to Slither static analysis tool,
/// running all operations in Docker containers for security. Handles extraction
/// of IR, call graphs, inheritance data, and storage layouts with caching.
use anyhow::{anyhow, Result};
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::build_brain::inheritance;
use crate::build_brain::parsers::parse_slithir_contract_summary;
use crate::build_brain::summarize::summarize_src_files;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_doc_file::extract_content_from_docs;

use super::callgraph;
use super::parsers::{parse_slither, parse_slithir_ir_code, parse_storage};

/// Global cache for Slither printer outputs to avoid redundant analysis.
/// Key format: "{repo_root}:{printer_name}"
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractSummary {
    /// Name of the contract containing this function
    pub contract: String,
    /// list of function
    pub content: String,
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

pub fn get_all_files_src(repo: &RepoPaths) -> String {
    repo.sol_files
        .iter()
        .filter(|p| {
            // Check if the file itself is a symlink
            match fs::symlink_metadata(p) {
                Ok(metadata) => !metadata.file_type().is_symlink(),
                Err(_) => {
                    log::warn!("Skipping unreadable path: {:?}", p);
                    false
                }
            }
        })
        .filter_map(|path| {
            let path_str = path.to_string_lossy();
            path_str
                .find(&format!("{}/src", &repo.repo_name))
                .map(|idx| path_str[idx..].to_string())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub async fn run_slither_detector(repo: &RepoPaths) -> Result<String> {
    let key = cache_key(&repo.root, "detector");
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither detector");
    let volume = format!("{}:/workspace", &repo.root.display());
    let out = Command::new("docker")
        .args([
            "run",
            "--read-only",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,            // Use the already-built repo folder
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--exclude-dependencies",
            "--foundry-out-directory", // Specify where to find Forge build artifacts
            "out",
        ])
        .output()?;

    // anyhow::ensure!(out.status.success(), "slither --sarif failed");
    //
    // Use whichever stream is non-empty (some printers output to stdout, others to stderr)
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&out.stderr).into_owned();
    }

    info!("slither analysis complete with size {}", text.len());
    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
}

/// Runs a single Slither printer and captures its output.
///
/// This function executes the Slither static analysis tool with a specific printer
/// and returns the captured output as a string.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @param printer - Name of the Slither printer to run (e.g., "slithir-ssa", "variable-order")
/// @return Result containing the printer's output as a string
pub async fn run_printer(repo: &RepoPaths, printer: &str) -> Result<String> {
    let key = cache_key(&repo.root, printer);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither printer: {}", printer);
    let volume = format!("{}:/workspace", &repo.root.display());
    let output = Command::new("docker")
        .args([
            "run",
            "--read-only",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,            // Use the already-built repo folder
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--foundry-out-directory",  // Specify where to find Forge build artifacts
            "out",
            "--print",
            printer,
            "--exclude-low",
            "--exclude-medium",
            "--exclude-high",
            "--exclude-informational",
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

    info!("{} printer complete with size {}", printer, text.len());
    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
}

// use for inheritance and call-graph
pub async fn run_printer_json(repo: &RepoPaths, printer: &str) -> Result<String> {
    let key = cache_key(&repo.root, printer);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither printer: {}", printer);
    let volume = format!("{}:/workspace", &repo.root.display());
    let out = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--foundry-out-directory",  // Specify where to find Forge build artifacts
            "out",
            "--print",
            printer,
            "--exclude-low",
            "--exclude-medium",
            "--exclude-high",
            "--exclude-informational",
            "--disable-color",
            "--json",
            "-",
        ])
        .output()?;

    anyhow::ensure!(out.status.success(), format!("slither {} failed", printer));

    let text = String::from_utf8_lossy(&out.stdout).into_owned();

    // Save to cache and return
    info!("{} print complete with size {}", printer, text.len());
    printer_cache.insert(key, text.clone());

    Ok(text)
}
/// Runs both Slither printers and returns the parsed IR and storage information.
///
/// This function is the main public interface for extracting SlithIR and storage
/// information from Solidity contracts. It runs both the slithir-ssa and variable-order
/// printers and parses their output.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @return Result containing a tuple of SlithIRFn and StorageVar vectors
pub async fn get_slither_ir_and_storage_for_codeblockcodeblock(
    repo: &RepoPaths,
) -> Result<(Vec<SlithIRFn>, Vec<StorageVar>, Vec<String>)> {
    // Run the slithir-ssa printer to get IR information
    let ir_raw = run_printer(repo, "slithir-ssa").await?;

    // Run the variable-order printer to get storage information
    let storage_raw = run_printer(repo, "variable-order").await?;
    // info!("storage raw => {}", storage_raw);

    let slither_scan_results = run_slither_detector(repo).await?;

    // Parse both outputs and return the results
    Ok((
        parse_slithir_ir_code(&ir_raw),
        parse_storage(&storage_raw),
        parse_slither(&slither_scan_results),
    ))
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
pub async fn save_code_metadata_and_analysis_to_txt_files(
    repo: &RepoPaths,
    dir: &Path,
    semantics_path: &Path,
) -> Result<Vec<PathBuf>> {
    // 1 . gather IR + storage  (re-use existing function)
    info!("get ir and storage chunks");
    let (_, _, slither_scan_vec) = get_slither_ir_and_storage_for_codeblockcodeblock(repo).await?;
    // info!("storage vec => {:?}", storage_vec);

    let (funcs, edges) = callgraph::get_dot_funcs_and_dot_edges(repo).await?;
    let inheritance_json = run_printer_json(repo, "inheritance").await?;
    let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
    let contract_summary = run_printer(repo, "contract-summary").await?;
    let contract_summary_vec = parse_slithir_contract_summary(&contract_summary);
    let src_file_list = get_all_files_src(repo);
    let summaries = summarize_src_files(repo, &semantics_path).await?;

    // 2 . serialise each artefact → one text file
    let mut out_paths = Vec::new();

    info!("save src file list to txt");
    let file_list = dir.join("src_files.txt");
    info!("src file list => {}", src_file_list);
    fs::write(&file_list, src_file_list)?;
    out_paths.push(file_list);

    info!("convert file summaries to txt files");
    for sum in &summaries {
        let meta = format!("{}::file_summary", sum.filename);
        // info!("contract meta => {}", meta);
        let body = format!("\nfile: {}\n{}", sum.filename, sum.summary);
        // info!("{}", body);
        let p = dir.join(meta.replace("::", "_").replace("/", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }
    info!("convert contract summary to txt files");
    for c in &contract_summary_vec {
        let meta = format!("{}::contract_summary", c.contract);
        // info!("contract meta => {}", meta);
        let body = format!("\nContract: {}\n{}", c.contract, c.content);
        // info!("{}", body);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph functions to txt files");
    for f in &funcs {
        let meta = format!("function::{}::{}", f.full_id, f.contract);
        // info!("fn meta => {}", meta);
        let body = format!("{} {} {}", f.full_id, f.contract, f.name);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph edges to txt files");
    for e in &edges {
        let meta = format!("caller::{}::callee::{}", e.caller, e.callee);
        // info!("caller callee meta => {}", meta);
        let body = format!("{} {}", e.caller, e.callee);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph edges to txt files");
    for edge in &inheritance_edges {
        let meta = format!("child::{}::parent::{}", edge.0, edge.1);
        // info!("edges meta => {}", meta);
        let body = format!("{} {}", edge.0, edge.1);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert slither scan results to txt files");
    for (i, issue) in slither_scan_vec.iter().enumerate() {
        let meta = format!("{} slither code issue", i);
        let p = dir.join(meta.replace(" ", "_") + ".txt");
        fs::write(&p, issue)?;
        out_paths.push(p);
    }

    info!(
        "slither issues found, fn ir, storage var files => {:?}",
        out_paths.len()
    );

    Ok(out_paths)
}

pub fn cache_key(repo_root: &Path, printer: &str) -> String {
    format!("{}::{}", repo_root.display(), printer)
}
