/// Slither static analyzer interface with Docker integration.
///
/// This module provides a secure interface to Slither static analysis tool,
/// running all operations in Docker containers for security. Handles extraction
/// of IR, call graphs, inheritance data, and storage layouts with caching.
use anyhow::{Result, anyhow};
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
/// Return “context file list” as LF-separated string.
pub fn get_all_files_src(repo: &RepoPaths) -> String {
    //   e.g.,   contracts/plume/
    let code_root = repo.root.join(&repo.repo_name);

    let mut files = Vec::<String>::new();

    for path in &repo.sol_files {
        // fast skip: must be under src/ and not a symlink
        if !path.starts_with(&code_root) {
            continue;
        }

        // ADD LIB exclusion
        let lib_folder = code_root.join("lib");
        let node_modules_folder = code_root.join("node_modules");
        if path.starts_with(lib_folder) || path.starts_with(node_modules_folder) {
            continue;
        }

        if fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(true)
        {
            continue;
        }

        // build a relative "short path"
        let rel = path.strip_prefix(&code_root).unwrap_or(path);
        let rel_str = rel.to_string_lossy();

        if should_skip(&rel_str) {
            continue;
        }

        files.push(rel_str.to_string());
    }

    // stable ordering helps diffing prompts
    files.sort();
    files.join("\n")
}

fn should_skip(rel: &str) -> bool {
    let lowercase = rel.to_ascii_lowercase();

    // 1. third-party deps: vendor/*/contracts/**   OR   node_modules/**/contracts/**
    if (lowercase.contains("/vendor/") && lowercase.contains("/contracts/"))
        || (lowercase.contains("/node_modules/") && lowercase.contains("/contracts/"))
        || lowercase.contains("/forge-std/")
    {
        return true;
    }

    // 2. other large externals you may add later
    if lowercase.starts_with("lib/") && lowercase.contains("/contracts/") {
        return true;
    }

    false
}

/// Detects the project type based on configuration files present in the repository.
#[derive(Debug, Clone, Copy)]
enum ProjectType {
    Foundry,
    FoundryYarn, // Hybrid projects with both Foundry and Yarn/npm
    Hardhat,
    Truffle,
    Generic,
}

fn detect_project_type(repo: &RepoPaths) -> ProjectType {
    let repo_path = repo.root.join(&repo.repo_name);

    // Check for various configuration files
    let has_foundry =
        repo_path.join("foundry.toml").exists() || repo_path.join("forge.toml").exists();
    let has_package_json = repo_path.join("package.json").exists();
    let has_yarn_lock = repo_path.join("yarn.lock").exists();
    let has_npm_lock = repo_path.join("package-lock.json").exists();
    let has_pnpm_lock = repo_path.join("pnpm-lock.yaml").exists();
    let has_hardhat_config = repo_path.join("hardhat.config.js").exists()
        || repo_path.join("hardhat.config.ts").exists()
        || repo_path.join("hardhat.config.cjs").exists();
    let has_truffle_config =
        repo_path.join("truffle-config.js").exists() || repo_path.join("truffle.js").exists();
    let has_out_dir = repo_path.join("out").exists();
    let has_artifacts_dir = repo_path.join("artifacts").exists();

    // Priority-based detection to handle overlapping configurations

    // 1. Check for hybrid Foundry + Node.js package manager projects
    if has_foundry && has_package_json && (has_yarn_lock || has_npm_lock || has_pnpm_lock) {
        // This covers projects that use Foundry for Solidity compilation but npm/yarn for dependencies
        return ProjectType::FoundryYarn;
    }

    // 2. Check for Hardhat projects (even if they also have Foundry config)
    if has_hardhat_config && has_package_json {
        // Hardhat projects typically have package.json and hardhat config
        // Note: Some projects might have both Hardhat and Foundry, but Hardhat takes precedence
        // if there's an explicit Hardhat config
        return ProjectType::Hardhat;
    }

    // 3. Check for Truffle projects
    if has_truffle_config && has_package_json {
        return ProjectType::Truffle;
    }

    // 4. Check for pure Foundry projects (with build artifacts)
    if has_foundry && has_out_dir {
        return ProjectType::Foundry;
    }

    // 5. Fallback: if foundry.toml exists but no clear package manager setup
    if has_foundry {
        return ProjectType::Foundry;
    }

    // 6. Check for Hardhat without explicit config (artifacts directory suggests Hardhat)
    if has_package_json && has_artifacts_dir && !has_out_dir {
        return ProjectType::Hardhat;
    }

    ProjectType::Generic
}

/// Builds Slither command arguments based on the detected project type.
fn build_slither_args(repo: &RepoPaths, printer: Option<&str>, json_output: bool) -> Vec<String> {
    let project_type = detect_project_type(repo);
    let mut args = vec![
        "run".to_string(),
        "--rm".to_string(),
        "-v".to_string(),
        format!("{}:/workspace", repo.root.display()),
        "-w".to_string(),
        "/workspace".to_string(),
        "ghcr.io/trailofbits/eth-security-toolbox:nightly".to_string(),
        "slither".to_string(),
        repo.repo_name.clone(),
    ];

    // Add project-specific arguments
    match project_type {
        ProjectType::Foundry => {
            args.extend([
                "--foundry-ignore-compile".to_string(),
                "--foundry-out-directory".to_string(),
                "out".to_string(),
            ]);
        }
        ProjectType::FoundryYarn => {
            // For hybrid Foundry + Yarn projects, let Slither compile the contracts
            // since the build artifacts might not be in the expected Foundry format
            // or the project might need npm dependencies to compile properly
            log::info!("FoundryYarn project detected - letting Slither handle compilation");
            // Don't add any ignore-compile flags - let Slither compile from source
        }
        ProjectType::Hardhat => {
            // Hardhat projects typically compile to artifacts/contracts
            if repo.root.join(&repo.repo_name).join("artifacts").exists() {
                args.extend([
                    "--hardhat-ignore-compile".to_string(),
                    "--hardhat-artifacts-directory".to_string(),
                    "artifacts".to_string(),
                ]);
            }
        }
        ProjectType::Truffle => {
            // Truffle projects typically compile to build/contracts
            if repo.root.join(&repo.repo_name).join("build").exists() {
                args.extend([
                    "--truffle-ignore-compile".to_string(),
                    "--truffle-build-directory".to_string(),
                    "build".to_string(),
                ]);
            }
        }
        ProjectType::Generic => {
            // For generic projects, try to detect common build directories
            let repo_path = repo.root.join(&repo.repo_name);
            if repo_path.join("out").exists() {
                args.extend([
                    "--foundry-ignore-compile".to_string(),
                    "--foundry-out-directory".to_string(),
                    "out".to_string(),
                ]);
            } else if repo_path.join("artifacts").exists() {
                args.extend([
                    "--hardhat-ignore-compile".to_string(),
                    "--hardhat-artifacts-directory".to_string(),
                    "artifacts".to_string(),
                ]);
            } else if repo_path.join("build").exists() {
                args.extend([
                    "--truffle-ignore-compile".to_string(),
                    "--truffle-build-directory".to_string(),
                    "build".to_string(),
                ]);
            }
            // If no build directory found, let Slither try to compile
        }
    }

    // Add printer-specific arguments
    if let Some(printer_name) = printer {
        args.extend(["--print".to_string(), printer_name.to_string()]);
    }

    // Add common arguments
    args.extend([
        "--exclude-low".to_string(),
        "--exclude-medium".to_string(),
        "--exclude-high".to_string(),
        "--exclude-informational".to_string(),
        "--disable-color".to_string(),
    ]);

    // Add JSON output if requested
    if json_output {
        args.extend(["--json".to_string(), "-".to_string()]);
    }

    log::info!("Detected project type: {:?}", project_type);

    args
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
    let mut args = build_slither_args(repo, None, false);
    // Add detector-specific arguments
    args.push("--exclude-dependencies".to_string());

    let out = Command::new("docker").args(&args).output()?;

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

/// Runs a single Slither printer with fallback strategies for hybrid projects.
///
/// This function executes the Slither static analysis tool with a specific printer
/// and returns the captured output as a string. For hybrid projects (e.g., Yarn + Foundry),
/// it will try multiple approaches if the first one fails.
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
    let args = build_slither_args(repo, Some(printer), false);
    let output = Command::new("docker")
        .args(&args)
        .stdout(Stdio::piped()) // Capture printer text from stdout
        .stderr(Stdio::piped()) // Capture banner & errors from stderr
        .output()?;

    // Use whichever stream is non-empty (some printers output to stdout, others to stderr)
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).into_owned();
    }

    // Check if the command failed and provide better error information
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow!(
            "Slither {} failed with exit code {:?}.\nStdout: {}\nStderr: {}",
            printer,
            output.status.code(),
            stdout,
            stderr
        ));
    }

    // Ensure we got some output
    if text.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Slither ran but produced no `{}` output. Stderr: {}",
            printer,
            stderr
        ));
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
    let args = build_slither_args(repo, Some(printer), true);
    let out = Command::new("docker").args(&args).output()?;

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
