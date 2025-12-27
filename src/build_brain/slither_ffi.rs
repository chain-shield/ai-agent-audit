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

use crate::build_brain::parsers::parse_slithir_contract_summary;
use crate::build_brain::summarize::summarize_src_files;
use crate::cost::cost_data::get_token_count;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::check_folder_name::contains_build_config;

use super::callgraph;
use super::parsers::{parse_slither, parse_slithir_ir_code};

/// Global cache for Slither printer outputs to avoid redundant analysis.
/// Key format: "{repo_root}:{printer_name}"
pub static PRINTER_OUTPUT_CACHE: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents a single function's SlithIR (intermediate representation).
///
/// SlithIR is Slither's intermediate representation of Solidity code, which
/// makes it easier to analyze the code's behavior and identify potential issues.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageVar {
    /// Name of the contract containing this storage variable
    pub contract: String,
    /// Name of the storage variable
    pub name: String,
    /// Data type of the storage variable (e.g., "uint256", "address", etc.)
    pub r#type: String,
}
/// Return “context file list” as LF-separated string.
pub fn get_all_files_src(repo: &RepoPaths) -> Result<String> {
    //   e.g.,   contracts/plume/
    let code_root = repo.root.join(&repo.repo_name);

    let mut files = Vec::<String>::new();

    let scoped_files = repo.extract_scoped_files()?;

    let main_files = if scoped_files.is_empty() {
        &repo.sol_files
    } else {
        &scoped_files
    };

    for path in main_files {
        // fast skip: must be under protocol root and not a symlink
        if !path.starts_with(&code_root) {
            continue;
        }

        // ADD LIB exclusion
        let lib_folder = code_root.join("lib");
        let test_folder = code_root.join("test");
        let script_folder = code_root.join("script");
        let node_modules_folder = code_root.join("node_modules");
        if path.starts_with(lib_folder)
            || path.starts_with(node_modules_folder)
            || path.starts_with(test_folder)
            || path.starts_with(script_folder)
        {
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
    Ok(files.join("\n"))
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

#[allow(dead_code)]
fn detect_project_type(repo: &RepoPaths) -> ProjectType {
    detect_project_type_at_path(&repo.root.join(&repo.repo_name))
}

fn detect_project_type_at_path(repo_path: &Path) -> ProjectType {
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

    // 1. Prefer explicit Hardhat configuration even if Foundry is also present
    if has_hardhat_config && has_package_json {
        // Hardhat projects typically have package.json and hardhat config
        // Some repos include foundry.toml for tooling; Hardhat should take precedence here
        return ProjectType::Hardhat;
    }

    // 2. Check for hybrid Foundry + Node.js package manager projects (but not Hardhat)
    if has_foundry
        && has_package_json
        && (has_yarn_lock || has_npm_lock || has_pnpm_lock)
        && !has_hardhat_config
    {
        // Projects that use Foundry for Solidity compilation but npm/yarn for dependencies
        return ProjectType::FoundryYarn;
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
pub fn build_slither_args(
    repo: &RepoPaths,
    printer: Option<&str>,
    subfolder: Option<PathBuf>,
    json_output: bool,
    use_ignore_compile: bool,
) -> Vec<String> {
    // Determine the actual target directory for Slither analysis
    let target_path = if let Some(ref folder) = subfolder {
        folder.clone()
    } else {
        repo.root.join(&repo.repo_name)
    };

    // Detect project type based on the actual target path (which may include subfolder)
    let project_type = detect_project_type_at_path(&target_path);

    let mut args = vec![
        "run".to_string(),
        "--rm".to_string(),
        "-v".to_string(),
        format!("{}:/workspace", repo.root.display()),
        "-w".to_string(),
        "/workspace".to_string(),
    ];

    // Note: We don't set FOUNDRY_PROFILE for Slither because:
    // 1. Custom build profiles may reference solc versions not available in Docker
    // 2. Slither will use foundry.toml's default profile or auto-detect settings
    // 3. For static analysis, exact compiler version match is less critical than for builds

    args.extend([
        "trailofbits/eth-security-toolbox:nightly".to_string(),
        "slither".to_string(),
    ]);

    // Add project-specific arguments (collect flags first; add target last)
    match project_type {
        ProjectType::Foundry | ProjectType::FoundryYarn => {
            // Try to use existing build artifacts if requested and available
            // This is faster but may fail with newer Foundry versions
            if use_ignore_compile && target_path.join("out").exists() {
                args.extend([
                    "--foundry-ignore-compile".to_string(),
                    "--foundry-out-directory".to_string(),
                    "out".to_string(),
                ]);
                log::debug!(
                    "Foundry project detected. Attempting to use existing build artifacts from out/"
                );
            } else {
                log::debug!("Foundry project detected. Slither will handle compilation.");
            }
        }
        ProjectType::Hardhat => {
            // Hardhat projects typically compile to artifacts
            let artifacts_dir = if target_path.join("artifacts").exists() {
                "artifacts"
            } else if target_path
                .join("packages")
                .join("hardhat")
                .join("artifacts")
                .exists()
            {
                // handle monorepos like packages/hardhat
                "packages/hardhat/artifacts"
            } else {
                "artifacts"
            };
            args.extend([
                "--compile-force-framework".to_string(),
                "hardhat".to_string(),
                "--hardhat-ignore-compile".to_string(),
                "--hardhat-artifacts-directory".to_string(),
                artifacts_dir.to_string(),
            ]);
        }
        ProjectType::Truffle => {
            // Truffle projects typically compile to build/contracts
            if target_path.join("build").exists() {
                args.extend([
                    "--truffle-ignore-compile".to_string(),
                    "--truffle-build-directory".to_string(),
                    "build".to_string(),
                ]);
            }
        }
        ProjectType::Generic => {
            // For generic projects, let Slither auto-detect and compile
            // This is more reliable than trying to use pre-built artifacts
            log::debug!(
                "Generic project detected. Slither will auto-detect project type and compile."
            );
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

    // Add the target (project directory) last; Slither CLI prefers positional at the end
    let target_dir = if let Some(folder) = subfolder {
        // folder is an absolute path, we need to make it relative to repo.root
        let relative_folder = folder.strip_prefix(&repo.root).unwrap_or(&folder);
        relative_folder.to_string_lossy().to_string()
    } else {
        repo.repo_name.clone()
    };
    args.push(target_dir);

    // log::info!("Detected project type: {:?}", project_type);

    args
}

pub async fn run_slither_detector(repo: &RepoPaths) -> Result<String> {
    let key = cache_key(&repo.root, "detector", None);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither detector");
    let mut args = build_slither_args(repo, None, None, false, false);
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

    info!(
        "slither analysis complete with token count: {}",
        get_token_count(&text)
    );
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
pub async fn run_printer(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
) -> Result<String> {
    let key = cache_key(&repo.root, printer, subfolder.clone());
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);

    // Check cache
    {
        let printer_cache = cache.lock().await;
        if let Some(cached) = printer_cache.get(&key) {
            return Ok(cached.clone());
        }
    } // Lock is dropped here

    log::info!("Running Slither printer: {}", printer);

    let args = build_slither_args(repo, Some(printer), subfolder, false, false);
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

    info!(
        "{} printer complete with token count: {}",
        printer,
        get_token_count(&text)
    );
    // Save to cache and return
    let mut printer_cache = cache.lock().await;
    printer_cache.insert(key, text.clone());
    Ok(text)
}
pub async fn run_printer_monorepo(repo: &RepoPaths, printer: &str) -> Result<String> {
    let mut total_output = String::new();
    let folders = repo.extract_monorepo_folders()?;

    for folder in folders {
        if contains_build_config(&folder) {
            let output = run_printer(repo, printer, Some(folder)).await?;
            total_output.push_str(&format!("\n{}\n", output));
        }
    }
    Ok(total_output)
}

/// Custom Slither runner for inheritance analysis with standard library filtering.
///
/// This function runs Slither's inheritance printer with explicit path filtering to:
/// 1. Include all project code (including lib folders like euler-price-oracle)
/// 2. Exclude only standard libraries and testing frameworks
/// 3. Override any slither.config.json exclusions
///
/// Excluded libraries:
/// - forge-std: Foundry standard library
/// - openzeppelin-contracts: OpenZeppelin contracts
/// - solady: Solady library
/// - ds-test: DappTools testing framework
/// - erc4626-tests: ERC4626 testing suite
/// - halmos-cheatcodes: Halmos symbolic testing cheatcodes
/// - solmate: Solmate library (includes testing utilities)
/// - prb-test: PRBTest testing framework
/// - node_modules: npm dependencies
///
/// DEPRECATED: This Slither-based inheritance extraction is no longer used.
/// Use the custom Solidity parsing in `src/enumerator/utils.rs` instead.
///
/// This ensures we capture inheritance relationships for all project contracts,
/// including those in lib/ folders that are part of the project scope.
#[deprecated(note = "Use custom Solidity parsing via contracts_in_source_folder() instead")]
pub async fn run_printer_json_inheritance(
    repo: &RepoPaths,
    subfolder: Option<PathBuf>,
) -> Result<String> {
    let printer = "inheritance";
    let key = format!(
        "{}_filtered",
        cache_key(&repo.root, printer, subfolder.clone())
    );
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither inheritance printer with standard library filtering");

    // Build base args without the target directory
    let mut args = build_slither_args(repo, Some(printer), subfolder.clone(), true, false);

    // Remove the last argument (target directory) temporarily
    let target_dir = args
        .pop()
        .expect("build_slither_args should always include target");

    // Add filter-paths to exclude standard libraries and testing frameworks
    // but include project lib folders. This overrides any slither.config.json exclusions
    args.push("--filter-paths".to_string());
    args.push(
        "lib/forge-std|\
         lib/openzeppelin-contracts|\
         lib/solady|\
         lib/ds-test|\
         lib/erc4626-tests|\
         lib/halmos-cheatcodes|\
         lib/solmate|\
         lib/prb-test|\
         node_modules"
            .to_string(),
    );

    // Add the target directory back at the end
    args.push(target_dir);

    // Log the full docker command for debugging
    // log::info!("Docker command: docker {}", args.join(" "));

    let out = Command::new("docker").args(&args).output()?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);
        log::error!(
            "Slither {} failed with exit code: {:?}",
            printer,
            out.status.code()
        );
        log::error!("Stdout: {}", stdout);
        log::error!("Stderr: {}", stderr);
        anyhow::bail!(
            "slither {} failed with exit code {:?}\nStdout: {}\nStderr: {}",
            printer,
            out.status.code(),
            stdout,
            stderr
        );
    }

    let text = String::from_utf8_lossy(&out.stdout).into_owned();

    // Save to cache and return
    info!(
        "{} print complete with token count: {}",
        printer,
        get_token_count(&text)
    );
    printer_cache.insert(key, text.clone());

    Ok(text)
}

// use for call-graph (inheritance now uses run_printer_json_inheritance)
pub async fn run_printer_json(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
) -> Result<String> {
    let key = cache_key(&repo.root, printer, subfolder.clone());
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);

    // Check cache
    {
        let printer_cache = cache.lock().await;
        if let Some(cached) = printer_cache.get(&key) {
            return Ok(cached.clone());
        }
    } // Lock is dropped here

    // log::info!("Running Slither printer: {}", printer);

    // Try with --foundry-ignore-compile first (faster if artifacts are compatible)
    let args = build_slither_args(repo, Some(printer), subfolder.clone(), true, true);

    // Log the full docker command for debugging
    // log::info!("Docker command: docker {}", args.join(" "));

    let out = Command::new("docker").args(&args).output()?;

    let text = if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);

        // Debug: log what we captured
        log::debug!(
            "Slither {} failed. Stderr length: {}, Stdout length: {}",
            printer,
            stderr.len(),
            stdout.len()
        );
        log::debug!(
            "Stderr preview: {}",
            &stderr.chars().take(500).collect::<String>()
        );
        log::debug!(
            "Stdout preview: {}",
            &stdout.chars().take(500).collect::<String>()
        );

        // Check if failure is due to incompatible build artifacts
        // Empty output with exit code 1 often indicates compilation failure with --foundry-ignore-compile
        let is_artifact_error = stderr.contains("KeyError: 'output'")
            || stderr.contains("hardhat_like_parsing")
            || stdout.contains("KeyError: 'output'")
            || stdout.contains("hardhat_like_parsing")
            || (stderr.is_empty() && stdout.is_empty()); // Empty output suggests early compilation failure

        if is_artifact_error {
            log::warn!(
                "Slither {} failed with artifact parsing error. Retrying without --foundry-ignore-compile...",
                printer
            );

            // Retry without --foundry-ignore-compile
            let args_no_ignore =
                build_slither_args(repo, Some(printer), subfolder.clone(), true, false);
            let out_retry = Command::new("docker").args(&args_no_ignore).output()?;

            if !out_retry.status.success() {
                let stderr_retry = String::from_utf8_lossy(&out_retry.stderr);
                let stdout_retry = String::from_utf8_lossy(&out_retry.stdout);
                log::error!(
                    "Slither {} failed even after retry with exit code: {:?}",
                    printer,
                    out_retry.status.code()
                );
                log::error!("Stdout: {}", stdout_retry);
                log::error!("Stderr: {}", stderr_retry);
                anyhow::bail!(
                    "slither {} failed with exit code {:?}\nStdout: {}\nStderr: {}",
                    printer,
                    out_retry.status.code(),
                    stdout_retry,
                    stderr_retry
                );
            }

            String::from_utf8_lossy(&out_retry.stdout).into_owned()
        } else {
            // Different error, fail immediately
            log::error!(
                "Slither {} failed with exit code: {:?}",
                printer,
                out.status.code()
            );
            log::error!("Stdout: {}", stdout);
            log::error!("Stderr: {}", stderr);
            anyhow::bail!(
                "slither {} failed with exit code {:?}\nStdout: {}\nStderr: {}",
                printer,
                out.status.code(),
                stdout,
                stderr
            );
        }
    } else {
        String::from_utf8_lossy(&out.stdout).into_owned()
    };

    // Save to cache and return
    info!(
        "{} print complete with token count:  {}",
        printer,
        get_token_count(&text)
    );
    let mut printer_cache = cache.lock().await;
    printer_cache.insert(key, text.clone());

    Ok(text)
}
/// Runs Slither printers and returns the parsed IR information.
///
/// This function is the main public interface for extracting SlithIR
/// information from Solidity contracts. It runs the slithir-ssa printer
/// and parses its output.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @return Result containing a tuple of SlithIRFn vectors and Slither detector results
pub async fn get_slither_ir_and_storage(
    repo: &RepoPaths,
) -> Result<(Vec<SlithIRFn>, Vec<StorageVar>, Vec<String>)> {
    // Run the slithir-ssa printer to get IR information
    let ir_raw = match repo.monorepo_folders {
        Some(_) => run_printer_monorepo(repo, "slithir-ssa").await?,
        None => run_printer(repo, "slithir-ssa", None).await?,
    };

    let slither_scan_results = run_slither_detector(repo).await?;

    // Parse IR output and return empty storage vars (no longer used)
    Ok((
        parse_slithir_ir_code(&ir_raw),
        Vec::new(), // Storage vars no longer extracted - dead code path
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
    let (_, _, slither_scan_vec) = get_slither_ir_and_storage(repo).await?;
    // info!("storage vec => {:?}", storage_vec);

    // Use monorepo-aware functions for call graph and inheritance
    let (funcs, edges) = callgraph::get_dot_funcs_and_dot_edges(repo).await?;
    // let inheritance_edges = callgraph::generate_inheritance_edges(repo).await?;
    let contract_summary = match repo.monorepo_folders {
        Some(_) => run_printer_monorepo(repo, "contract-summary").await?,
        None => run_printer(repo, "contract-summary", None).await?,
    };
    let contract_summary_vec = parse_slithir_contract_summary(&contract_summary);
    let src_file_list = get_all_files_src(repo)?;
    let summaries = summarize_src_files(repo, &semantics_path).await?;

    // 2 . serialise each artefact → one text file
    let mut out_paths = Vec::new();

    info!("save src file list to txt");
    let file_list = dir.join("src_files.txt");
    info!("src file list => {}", src_file_list);
    fs::write(&file_list, src_file_list)?;
    out_paths.push(file_list);

    // info!("convert file summaries to txt files");
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

    // info!("convert call graph edges to txt files");
    // for edge in &inheritance_edges {
    //     let meta = format!("child::{}::parent::{}", edge.0, edge.1);
    //     // info!("edges meta => {}", meta);
    //     let body = format!("{} {}", edge.0, edge.1);
    //     let p = dir.join(meta.replace("::", "_") + ".txt");
    //     fs::write(&p, body)?;
    //     out_paths.push(p);
    // }
    //
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

pub fn cache_key(repo_root: &Path, printer: &str, subfolder: Option<PathBuf>) -> String {
    match subfolder {
        Some(folder) => format!("{}-{}::{}", repo_root.display(), folder.display(), printer),
        None => format!("{}::{}", repo_root.display(), printer),
    }
}
