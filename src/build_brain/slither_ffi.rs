/// Slither static analyzer interface.
///
/// This module provides an interface to the Slither static analysis tool.
/// It handles extraction of IR, call graphs, inheritance data, and storage
/// layouts with caching.
use anyhow::{Result, anyhow};
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::cost::cost_data::get_token_count;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::check_folder_name::contains_build_config;
use crate::utils::runtime_deps::{RuntimeDependency, ensure_runtime_dependencies};

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
    let code_root = repo.get_protocol_root();

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
    detect_project_type_at_path(&repo.get_protocol_root())
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

    // 1. Prefer explicit Foundry build artifacts when present. Mixed repos can
    // carry both Hardhat and Foundry configs, but if the app just built `out/`,
    // Slither should read the Foundry project instead of chasing Hardhat.
    if has_foundry && has_out_dir {
        return ProjectType::Foundry;
    }

    // 2. Prefer explicit Hardhat configuration even if Foundry is also present
    if has_hardhat_config && has_package_json {
        // Hardhat projects typically have package.json and hardhat config
        // Some repos include foundry.toml for tooling; Hardhat should take precedence here
        return ProjectType::Hardhat;
    }

    // 3. Check for hybrid Foundry + Node.js package manager projects (but not Hardhat)
    if has_foundry
        && has_package_json
        && (has_yarn_lock || has_npm_lock || has_pnpm_lock)
        && !has_hardhat_config
    {
        // Projects that use Foundry for Solidity compilation but npm/yarn for dependencies
        return ProjectType::FoundryYarn;
    }

    // 4. Check for Truffle projects
    if has_truffle_config && has_package_json {
        return ProjectType::Truffle;
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
        repo.get_protocol_root()
    };

    // Detect project type based on the actual target path (which may include subfolder)
    let project_type = detect_project_type_at_path(&target_path);

    let mut args = Vec::new();

    // Note: We don't set FOUNDRY_PROFILE for Slither because custom build
    // profiles may reference solc versions not available locally. Slither will
    // use foundry.toml's default profile or auto-detect settings.

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
            args.extend([
                "--compile-force-framework".to_string(),
                "hardhat".to_string(),
            ]);
            if use_ignore_compile
                && let Some(artifacts_dir) = hardhat_artifacts_directory(&target_path)
            {
                args.extend([
                    "--hardhat-ignore-compile".to_string(),
                    "--hardhat-artifacts-directory".to_string(),
                    artifacts_dir,
                ]);
            }
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
        repo.get_protocol_root()
            .strip_prefix(&repo.root)
            .ok()
            .filter(|relative| !relative.as_os_str().is_empty())
            .map(|relative| relative.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string())
    };
    args.push(target_dir);

    // log::info!("Detected project type: {:?}", project_type);

    args
}

fn hardhat_artifacts_directory(target_path: &Path) -> Option<String> {
    const CANDIDATES: [&str; 5] = [
        "artifacts",
        "build/contracts",
        "build/artifacts",
        "build",
        "packages/hardhat/artifacts",
    ];

    CANDIDATES
        .iter()
        .find(|candidate| target_path.join(candidate).join("build-info").is_dir())
        .or_else(|| {
            CANDIDATES
                .iter()
                .find(|candidate| target_path.join(candidate).is_dir())
        })
        .map(|candidate| (*candidate).to_string())
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
    ensure_runtime_dependencies("Slither detector", &[RuntimeDependency::Slither])?;
    let mut args = build_slither_args(repo, None, None, false, true);
    // Add detector-specific arguments
    let target = args
        .pop()
        .expect("build_slither_args should always include target");
    args.push("--exclude-dependencies".to_string());
    args.push(target);

    let out = Command::new("slither")
        .current_dir(&repo.root)
        .args(&args)
        .output()?;

    if !out.status.success()
        && let Ok(text) = run_detector_per_file(repo, None)
    {
        printer_cache.insert(key, text.clone());
        return Ok(text);
    }

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
    ensure_runtime_dependencies(
        &format!("Slither printer `{}`", printer),
        &[RuntimeDependency::Slither],
    )?;

    let args = build_slither_args(repo, Some(printer), subfolder.clone(), false, true);
    let output = Command::new("slither")
        .current_dir(&repo.root)
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
        if let Ok(text) = run_printer_per_file(repo, printer, subfolder.clone()) {
            let mut printer_cache = cache.lock().await;
            printer_cache.insert(key, text.clone());
            return Ok(text);
        }
        let output_detail = slither_output_detail(&output);
        return Err(anyhow!(
            "Slither {} failed with exit code {:?}.{}",
            printer,
            output.status.code(),
            output_detail
        ));
    }

    // Ensure we got some output
    if text.trim().is_empty() {
        if let Ok(text) = run_printer_per_file(repo, printer, subfolder.clone()) {
            let mut printer_cache = cache.lock().await;
            printer_cache.insert(key, text.clone());
            return Ok(text);
        }
        return Err(anyhow!(
            "Slither ran but produced no `{}` output.{}",
            printer,
            slither_output_detail(&output)
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
    let mut args = build_slither_args(repo, Some(printer), subfolder.clone(), true, true);

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

    ensure_runtime_dependencies("Slither inheritance printer", &[RuntimeDependency::Slither])?;

    let out = Command::new("slither")
        .current_dir(&repo.root)
        .args(&args)
        .output()?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);
        log::error!(
            "Slither {} failed with exit code: {:?}",
            printer,
            out.status.code()
        );
        if slither_verbose_output_enabled() {
            log::error!("Stdout: {}", stdout);
            log::error!("Stderr: {}", stderr);
        } else {
            log::error!(
                "Slither stdout/stderr suppressed. Set AI_AGENT_AUDIT_SLITHER_VERBOSE=1 to include command output."
            );
        }
        anyhow::bail!(
            "slither {} failed with exit code {:?}.{}",
            printer,
            out.status.code(),
            slither_output_detail(&out)
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

#[derive(Debug)]
struct SlitherAttempt {
    label: String,
    current_dir: PathBuf,
    args: Vec<String>,
}

fn slither_target_path(repo: &RepoPaths, subfolder: Option<&Path>) -> PathBuf {
    subfolder
        .map(Path::to_path_buf)
        .unwrap_or_else(|| repo.get_protocol_root())
}

fn build_slither_args_with_dot_target(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
    json_output: bool,
    use_ignore_compile: bool,
) -> Vec<String> {
    let mut args = build_slither_args(
        repo,
        Some(printer),
        subfolder,
        json_output,
        use_ignore_compile,
    );
    if let Some(target) = args.last_mut() {
        *target = ".".to_string();
    }
    args
}

fn build_foundry_slither_args(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
    json_output: bool,
    use_ignore_compile: bool,
    dot_target: bool,
) -> Vec<String> {
    let target_path = slither_target_path(repo, subfolder.as_deref());
    let mut args = Vec::new();

    args.extend([
        "--compile-force-framework".to_string(),
        "foundry".to_string(),
    ]);

    if use_ignore_compile && target_path.join("out").exists() {
        args.extend([
            "--foundry-ignore-compile".to_string(),
            "--foundry-out-directory".to_string(),
            "out".to_string(),
        ]);
    }

    args.extend(["--print".to_string(), printer.to_string()]);
    args.extend([
        "--exclude-low".to_string(),
        "--exclude-medium".to_string(),
        "--exclude-high".to_string(),
        "--exclude-informational".to_string(),
        "--disable-color".to_string(),
    ]);

    if json_output {
        args.extend(["--json".to_string(), "-".to_string()]);
    }

    let target = if dot_target {
        ".".to_string()
    } else if let Some(folder) = subfolder {
        folder
            .strip_prefix(&repo.root)
            .unwrap_or(&folder)
            .to_string_lossy()
            .to_string()
    } else {
        repo.get_protocol_root()
            .strip_prefix(&repo.root)
            .ok()
            .filter(|relative| !relative.as_os_str().is_empty())
            .map(|relative| relative.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string())
    };
    args.push(target);

    args
}

fn has_foundry_config(path: &Path) -> bool {
    path.join("foundry.toml").exists() || path.join("forge.toml").exists()
}

fn json_slither_attempts(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
) -> Vec<SlitherAttempt> {
    let target_path = slither_target_path(repo, subfolder.as_deref());
    let mut attempts = vec![
        SlitherAttempt {
            label: "default target with existing artifacts".to_string(),
            current_dir: repo.root.clone(),
            args: build_slither_args(repo, Some(printer), subfolder.clone(), true, true),
        },
        SlitherAttempt {
            label: "default target with Slither compile".to_string(),
            current_dir: repo.root.clone(),
            args: build_slither_args(repo, Some(printer), subfolder.clone(), true, false),
        },
        SlitherAttempt {
            label: "protocol cwd with existing artifacts".to_string(),
            current_dir: target_path.clone(),
            args: build_slither_args_with_dot_target(repo, printer, subfolder.clone(), true, true),
        },
        SlitherAttempt {
            label: "protocol cwd with Slither compile".to_string(),
            current_dir: target_path.clone(),
            args: build_slither_args_with_dot_target(repo, printer, subfolder.clone(), true, false),
        },
    ];

    if has_foundry_config(&target_path) {
        attempts.extend([
            SlitherAttempt {
                label: "forced Foundry target with existing artifacts".to_string(),
                current_dir: repo.root.clone(),
                args: build_foundry_slither_args(
                    repo,
                    printer,
                    subfolder.clone(),
                    true,
                    true,
                    false,
                ),
            },
            SlitherAttempt {
                label: "forced Foundry target with Slither compile".to_string(),
                current_dir: repo.root.clone(),
                args: build_foundry_slither_args(
                    repo,
                    printer,
                    subfolder.clone(),
                    true,
                    false,
                    false,
                ),
            },
            SlitherAttempt {
                label: "protocol cwd forced Foundry with existing artifacts".to_string(),
                current_dir: target_path.clone(),
                args: build_foundry_slither_args(
                    repo,
                    printer,
                    subfolder.clone(),
                    true,
                    true,
                    true,
                ),
            },
            SlitherAttempt {
                label: "protocol cwd forced Foundry with Slither compile".to_string(),
                current_dir: target_path,
                args: build_foundry_slither_args(repo, printer, subfolder, true, false, true),
            },
        ]);
    }

    attempts
}

fn run_slither_attempt(attempt: &SlitherAttempt) -> Result<Output> {
    Ok(Command::new("slither")
        .current_dir(&attempt.current_dir)
        .args(&attempt.args)
        .output()?)
}

fn scoped_slither_file_targets(repo: &RepoPaths, current_dir: &Path) -> Result<Vec<String>> {
    let mut files = repo.extract_scoped_files()?;
    if files.is_empty() {
        files = repo.sol_files.clone();
    }

    let mut targets = files
        .into_iter()
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == OsStr::new("sol"))
        })
        .filter(|path| path.starts_with(current_dir))
        .filter_map(|path| {
            path.strip_prefix(current_dir)
                .ok()
                .map(|relative| relative.to_string_lossy().to_string())
        })
        .filter(|target| !target.contains("/node_modules/"))
        .filter(|target| !target.starts_with("node_modules/"))
        .collect::<Vec<_>>();

    targets.sort();
    targets.dedup();
    Ok(targets)
}

fn solc_remaps_arg(current_dir: &Path) -> Option<String> {
    let remappings = current_dir.join("remappings.txt");
    let content = fs::read_to_string(remappings).ok()?;
    let remaps = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ");
    (!remaps.is_empty()).then_some(remaps)
}

fn append_solc_remaps(args: &mut Vec<String>, current_dir: &Path) {
    if let Some(remaps) = solc_remaps_arg(current_dir) {
        args.extend(["--solc-remaps".to_string(), remaps]);
    }
}

fn single_file_slither_args(
    current_dir: &Path,
    target: &str,
    printer: Option<&str>,
    json_output: bool,
) -> Vec<String> {
    let mut args = vec![target.to_string()];

    if let Some(printer) = printer {
        args.extend(["--print".to_string(), printer.to_string()]);
    }

    args.extend([
        "--exclude-low".to_string(),
        "--exclude-medium".to_string(),
        "--exclude-high".to_string(),
        "--exclude-informational".to_string(),
        "--disable-color".to_string(),
    ]);

    append_solc_remaps(&mut args, current_dir);

    if json_output {
        args.extend(["--json".to_string(), "-".to_string()]);
    }

    args
}

fn run_printer_json_per_file(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
) -> Result<String> {
    let current_dir = slither_target_path(repo, subfolder.as_deref());
    let targets = scoped_slither_file_targets(repo, &current_dir)?;
    let mut elements = Vec::<serde_json::Value>::new();
    let mut failures = Vec::<String>::new();

    for target in targets {
        let attempt = SlitherAttempt {
            label: format!("single-file {printer} fallback for {target}"),
            current_dir: current_dir.clone(),
            args: single_file_slither_args(&current_dir, &target, Some(printer), true),
        };
        let output = run_slither_attempt(&attempt)?;
        if !output.status.success() {
            failures.push(slither_failure_summary(&attempt, &output));
            continue;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) else {
            failures.push(format!(
                "{} produced invalid JSON.{}",
                attempt.label,
                slither_output_detail(&output)
            ));
            continue;
        };

        let Some(printers) = json
            .get("results")
            .and_then(|results| results.get("printers"))
            .and_then(|printers| printers.as_array())
        else {
            continue;
        };

        for printer in printers {
            if let Some(file_elements) = printer.get("elements").and_then(|items| items.as_array())
            {
                elements.extend(file_elements.iter().cloned());
            }
        }
    }

    if elements.is_empty() {
        anyhow::bail!(
            "Slither {printer} single-file fallback produced no output from {}. Failures:\n{}",
            current_dir.display(),
            failures.join("\n\n")
        );
    }

    if !failures.is_empty() {
        log::warn!(
            "Slither {} single-file fallback skipped {} files. First failure: {}",
            printer,
            failures.len(),
            failures.first().cloned().unwrap_or_default()
        );
    }

    Ok(serde_json::json!({
        "success": true,
        "error": null,
        "results": {
            "printers": [{
                "elements": elements,
                "description": "",
                "markdown": "",
                "first_markdown_element": "",
                "id": "single-file-fallback",
                "printer": printer
            }]
        }
    })
    .to_string())
}

fn run_printer_per_file(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
) -> Result<String> {
    let current_dir = slither_target_path(repo, subfolder.as_deref());
    let targets = scoped_slither_file_targets(repo, &current_dir)?;
    let mut text = String::new();
    let mut failures = Vec::<String>::new();

    for target in targets {
        let attempt = SlitherAttempt {
            label: format!("single-file {printer} fallback for {target}"),
            current_dir: current_dir.clone(),
            args: single_file_slither_args(&current_dir, &target, Some(printer), false),
        };
        let output = run_slither_attempt(&attempt)?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let file_text = if stdout.trim().is_empty() {
            stderr.as_ref()
        } else {
            stdout.as_ref()
        };

        if output.status.success() && !file_text.trim().is_empty() {
            text.push_str(file_text);
            text.push('\n');
        } else {
            failures.push(slither_failure_summary(&attempt, &output));
        }
    }

    if text.trim().is_empty() {
        anyhow::bail!(
            "Slither {printer} single-file fallback produced no output from {}. Failures:\n{}",
            current_dir.display(),
            failures.join("\n\n")
        );
    }

    if !failures.is_empty() {
        log::warn!(
            "Slither {} single-file fallback skipped {} files. First failure: {}",
            printer,
            failures.len(),
            failures.first().cloned().unwrap_or_default()
        );
    }

    Ok(text)
}

fn run_detector_per_file(repo: &RepoPaths, subfolder: Option<PathBuf>) -> Result<String> {
    let current_dir = slither_target_path(repo, subfolder.as_deref());
    let targets = scoped_slither_file_targets(repo, &current_dir)?;
    let mut text = String::new();
    let mut failures = Vec::<String>::new();

    for target in targets {
        let attempt = SlitherAttempt {
            label: format!("single-file detector fallback for {target}"),
            current_dir: current_dir.clone(),
            args: single_file_slither_args(&current_dir, &target, None, false),
        };
        let output = run_slither_attempt(&attempt)?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let file_text = if stdout.trim().is_empty() {
            stderr.as_ref()
        } else {
            stdout.as_ref()
        };

        if output.status.success() && !file_text.trim().is_empty() {
            text.push_str(file_text);
            text.push('\n');
        } else {
            failures.push(slither_failure_summary(&attempt, &output));
        }
    }

    if text.trim().is_empty() {
        anyhow::bail!(
            "Slither detector single-file fallback produced no output from {}. Failures:\n{}",
            current_dir.display(),
            failures.join("\n\n")
        );
    }

    if !failures.is_empty() {
        log::warn!(
            "Slither detector single-file fallback skipped {} files. First failure: {}",
            failures.len(),
            failures.first().cloned().unwrap_or_default()
        );
    }

    Ok(text)
}

fn slither_command_for_log(attempt: &SlitherAttempt) -> String {
    format!(
        "(cd {} && slither {})",
        attempt.current_dir.display(),
        attempt
            .args
            .iter()
            .map(|arg| {
                if arg.chars().any(char::is_whitespace) {
                    format!("{arg:?}")
                } else {
                    arg.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn slither_output_preview(text: &str) -> String {
    const MAX_CHARS: usize = 12_000;
    let mut chars = text.chars();
    let preview = chars.by_ref().take(MAX_CHARS).collect::<String>();
    if chars.next().is_some() {
        format!(
            "{preview}\n... output truncated; rerun the command above for full Slither output ..."
        )
    } else {
        preview
    }
}

fn slither_combined_output(output: &Output) -> String {
    format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn slither_has_foundry_artifact_schema_error(output: &Output) -> bool {
    let text = slither_combined_output(output);
    text.contains("KeyError: 'output'") && text.contains("crytic_compile")
}

fn slither_has_ir_ssa_error(output: &Output) -> bool {
    let text = slither_combined_output(output);
    text.contains("Failed to convert IR to SSA")
        || text.contains("slithir/utils/ssa.py")
        || text.contains("AssertionError")
}

fn slither_attempt_uses_foundry_artifacts(attempt: &SlitherAttempt) -> bool {
    attempt
        .args
        .iter()
        .any(|arg| arg == "--foundry-ignore-compile")
}

fn slither_verbose_output_enabled() -> bool {
    std::env::var_os("AI_AGENT_AUDIT_SLITHER_VERBOSE").is_some()
}

fn slither_output_detail(output: &Output) -> String {
    if !slither_verbose_output_enabled() {
        return " Slither stdout/stderr suppressed; set AI_AGENT_AUDIT_SLITHER_VERBOSE=1 to include command output.".to_string();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!(
        "\nStdout: {}\nStderr: {}",
        slither_output_preview(&stdout),
        slither_output_preview(&stderr)
    )
}

fn slither_failure_summary(attempt: &SlitherAttempt, output: &Output) -> String {
    let mut summary = format!(
        "{} failed with exit code {:?}.",
        attempt.label,
        output.status.code()
    );
    if slither_verbose_output_enabled() {
        summary.push_str(&format!(
            "\nCommand: {}{}",
            slither_command_for_log(attempt),
            slither_output_detail(output)
        ));
    } else {
        summary.push_str(
            " Set AI_AGENT_AUDIT_SLITHER_VERBOSE=1 to include command and stdout/stderr.",
        );
    }
    summary
}

fn run_slither_json_with_fallbacks(
    repo: &RepoPaths,
    printer: &str,
    subfolder: Option<PathBuf>,
) -> Result<String> {
    let attempts = json_slither_attempts(repo, printer, subfolder.clone());
    let mut failures = Vec::new();
    let mut skip_foundry_artifact_attempts = false;

    for (idx, attempt) in attempts.iter().enumerate() {
        if skip_foundry_artifact_attempts && slither_attempt_uses_foundry_artifacts(attempt) {
            log::warn!(
                "Skipping Slither {} attempt `{}` after Foundry artifact schema failure",
                printer,
                attempt.label
            );
            continue;
        }

        if idx > 0 {
            log::warn!(
                "Retrying Slither {} using {}",
                printer,
                attempt.label.as_str()
            );
        }

        log::debug!(
            "Running Slither {} attempt `{}`: {}",
            printer,
            attempt.label,
            slither_command_for_log(attempt)
        );

        let output = run_slither_attempt(attempt)?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        if output.status.success() && !stdout.trim().is_empty() {
            if idx > 0 {
                log::warn!(
                    "Slither {} succeeded after fallback attempt `{}`",
                    printer,
                    attempt.label
                );
            }
            return Ok(stdout);
        }

        let summary = if output.status.success() {
            format!(
                "{} succeeded but produced empty JSON output.{}",
                attempt.label,
                slither_output_detail(&output)
            )
        } else {
            slither_failure_summary(attempt, &output)
        };

        if idx == 0 {
            log::warn!(
                "Slither {} failed on primary attempt; trying fallbacks. {}",
                printer,
                summary
            );
        } else {
            log::warn!("Slither {} fallback failed. {}", printer, summary);
        }
        failures.push(summary);

        if slither_has_foundry_artifact_schema_error(&output) {
            skip_foundry_artifact_attempts = true;
        }

        if printer == "call-graph" && slither_has_ir_ssa_error(&output) {
            log::warn!(
                "Slither {} hit an internal IR/SSA failure; switching to scoped single-file fallback",
                printer
            );
            if let Ok(json) = run_printer_json_per_file(repo, printer, subfolder.clone()) {
                log::warn!(
                    "Slither {} succeeded with single-file fallback after project-level IR/SSA failure",
                    printer
                );
                return Ok(json);
            }
        }
    }

    if printer == "call-graph"
        && let Ok(json) = run_printer_json_per_file(repo, printer, subfolder)
    {
        log::warn!(
            "Slither {} succeeded with single-file fallback after project-level attempts failed",
            printer
        );
        return Ok(json);
    }

    anyhow::bail!(
        "slither {} failed after {} attempts:\n\n{}",
        printer,
        failures.len(),
        failures.join("\n\n")
    )
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

    ensure_runtime_dependencies(
        &format!("Slither JSON printer `{}`", printer),
        &[RuntimeDependency::Slither],
    )?;

    let text = run_slither_json_with_fallbacks(repo, printer, subfolder)?;

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
/// Runs Slither printers and returns parsed IR and detector information.
///
/// This function is the main public interface for extracting SlithIR
/// information from Solidity contracts. It runs the slithir-ssa printer
/// and parses its output. The storage return slot is retained for API
/// stability, but it is currently always empty.
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

pub fn cache_key(repo_root: &Path, printer: &str, subfolder: Option<PathBuf>) -> String {
    match subfolder {
        Some(folder) => format!("{}-{}::{}", repo_root.display(), folder.display(), printer),
        None => format!("{}::{}", repo_root.display(), printer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AuditType;

    fn test_repo(root: PathBuf, scoped_files: Option<PathBuf>) -> RepoPaths {
        RepoPaths {
            github_url: "https://github.com/example/repo".to_string(),
            project_id: "repo-abcdef".to_string(),
            root: root.clone(),
            sol_files: vec![root.join("repo/contracts/Fallback.sol")],
            test_files: vec![],
            script_files: vec![],
            config_files: vec![],
            lib_config_files: vec![],
            source_code_folders: vec![root.join("repo/contracts")],
            docs: vec![],
            repo_name: "repo".to_string(),
            audit_scope: None,
            excluded_folders: None,
            scoped_files,
            monorepo_folders: None,
            commit_hash: "abcdef1234567890abcdef1234567890abcdef12".to_string(),
            audit_type: AuditType::ImmunefiBugBounty,
        }
    }

    #[test]
    fn solc_remaps_arg_reads_non_empty_remappings() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(
            tmp.path().join("remappings.txt"),
            "\n# comment\n@openzeppelin/=node_modules/@openzeppelin/\nfoo/=lib/foo/\n",
        )
        .unwrap();

        assert_eq!(
            solc_remaps_arg(tmp.path()).as_deref(),
            Some("@openzeppelin/=node_modules/@openzeppelin/ foo/=lib/foo/")
        );
    }

    #[test]
    fn single_file_slither_args_include_remappings_and_json() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(
            tmp.path().join("remappings.txt"),
            "@openzeppelin/=node_modules/@openzeppelin/\n",
        )
        .unwrap();

        let args = single_file_slither_args(
            tmp.path(),
            "contracts/Registry.sol",
            Some("call-graph"),
            true,
        );

        assert!(args.contains(&"contracts/Registry.sol".to_string()));
        assert!(args.contains(&"--print".to_string()));
        assert!(args.contains(&"call-graph".to_string()));
        assert!(args.contains(&"--solc-remaps".to_string()));
        assert!(args.contains(&"@openzeppelin/=node_modules/@openzeppelin/".to_string()));
        assert!(args.contains(&"--json".to_string()));
        assert!(args.contains(&"-".to_string()));
    }

    #[test]
    fn scoped_slither_targets_prefer_scope_file_and_filter_to_current_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let repo_root = tmp.path().join("repo");
        fs::create_dir_all(repo_root.join("contracts")).unwrap();
        fs::create_dir_all(repo_root.join("node_modules/pkg")).unwrap();
        fs::write(
            repo_root.join("contracts/Registry.sol"),
            "contract Registry {}\n",
        )
        .unwrap();
        fs::write(
            repo_root.join("node_modules/pkg/Dependency.sol"),
            "contract Dependency {}\n",
        )
        .unwrap();
        let scope = tmp.path().join("scope.txt");
        fs::write(
            &scope,
            "./contracts/Registry.sol\n./node_modules/pkg/Dependency.sol\n",
        )
        .unwrap();
        let repo = test_repo(tmp.path().to_path_buf(), Some(scope));

        let targets = scoped_slither_file_targets(&repo, &repo_root).unwrap();

        assert_eq!(targets, vec!["contracts/Registry.sol"]);
    }
}
