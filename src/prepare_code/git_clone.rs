/// Repository preparation and Docker-based building.
///
/// This module handles secure repository cloning in Docker containers,
/// auto-detection of build systems (Foundry/Hardhat), and file filtering
/// for smart contract analysis.
use anyhow::{Context, Result};
use ignore::gitignore::GitignoreBuilder;
use log::info;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::cli_args::parse::Cli;
use crate::config::audit_config;
use crate::utils::file_security::{validate_repo_url, validate_safe_path};

/// Build flags for forge compilation
#[derive(Debug, Clone, Copy)]
pub enum BuildFlags {
    /// Standard forge build
    Standard,
    /// Forge build with --via-ir --build-info flags
    ViaIr,
}

/// Contains paths to the repository root and relevant files.
/// This struct organizes the paths to Solidity files and documentation
/// that will be processed for analysis.
#[derive(Debug, Clone)]
pub struct RepoPaths {
    /// Path to the repository root directory
    pub root: PathBuf,
    /// Paths to all Solidity (.sol) files in the repository
    pub sol_files: Vec<PathBuf>,
    /// Paths to documentation files (README.md, etc.)
    pub docs: Vec<PathBuf>,
    /// e.g. `"my-cool-repo"`
    pub repo_name: String,
    /// full 40-char SHA, e.g. `"1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t"`
    pub commit_hash: String,
}

impl RepoPaths {
    /// Generates a unique identifier for the repository using name and short commit hash.
    /// Used for creating unique vector database collections and cache keys.
    pub fn unique_repo_hash(&self) -> String {
        format!(
            "{}-{}",
            self.repo_name.replace("/", "-"),
            &self.commit_hash[..6]
        )
    }
}

/// Clones a repository and builds it in a secure Docker environment.
///
/// This function performs the complete repository preparation workflow:
/// 1. Creates a Docker volume for isolated analysis
/// 2. Clones the repository using Trail of Bits security toolbox
/// 3. Auto-detects and builds with Foundry or Hardhat
/// 4. Filters and organizes Solidity files and documentation
/// 5. Extracts commit hash for unique identification
///
/// # Arguments
/// * `url` - Git repository URL to clone and analyze
/// * `subfolder` - Optional subfolder name to analyze within the repository
/// * `build_flags` - Build flags for forge compilation
///
/// # Returns
/// * `RepoPaths` - Organized repository paths and metadata
///
/// # Security
/// All operations are performed in isolated Docker containers to prevent
/// malicious code execution on the host system.
pub fn clone_and_filter_git_repo(
    // url: &str,
    // subfolder: Option<&str>,
    // build_flags: BuildFlags,
    cli: &Cli,
) -> Result<RepoPaths> {
    // 🔐 Validate the repository URL for safety
    validate_repo_url(&cli.repo)?;

    // 2. Extract & sanitize the repo name
    let mut repo_name = cli
        .repo
        .trim_end_matches(".git")
        .rsplit('/')
        .next()
        .unwrap_or("repo")
        .to_string();

    // Append subfolder to repo_name if specified
    if let Some(sf) = &cli.subfolder {
        repo_name = format!("{}/{}", repo_name, sf);
    }
    info!("repo_name ==> {}", repo_name);

    // 4. Read HEAD and get the first 6 chars of the commit SHA
    let commit_hash = get_commit_hash(&cli.repo)?;
    let short_hash = &commit_hash[..6];

    // 5. git clone, install, and build in secure docker container
    // returns dierctory where files are located
    let root = clone_and_build_repo(cli, &repo_name, short_hash)?;

    // 6. Build .gitignore matcher
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    ign.add_line(None, "node_modules")?;
    let ign = ign.build()?;

    // Determine the search root - if subfolder is specified, search within that subdirectory
    let search_root = root.join(&repo_name);
    info!("search_root => {}", search_root.display());

    // Validate that the search root exists
    if !search_root.exists() {
        anyhow::bail!(
            "Specified path '{}' does not exist in the repository",
            repo_name
        );
    }

    // Initialize vectors to store file paths
    let mut sol_files = Vec::new();
    let mut docs = Vec::new();
    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_entry(|e| {
            let path = e.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            // Skip out/, cache/, and .git
            !(name == "out" || name == "cache" || name == ".git")
        })
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if entry.file_type().is_dir() {
            continue;
        }
        // skip if gitignore or simlink
        if ign.matched(path, false).is_ignore()
            || fs::symlink_metadata(path)?.file_type().is_symlink()
        {
            continue;
        }

        //only get md docs from /src folder /src/*.md
        match path.extension().and_then(|e| e.to_str()) {
            Some("sol") => sol_files.push(path.to_path_buf()),
            Some("md") if path.parent().map_or(false, |p| p == search_root) => {
                docs.push(path.to_path_buf())
            }
            _ => {}
        }
    }

    // Return the collected paths
    Ok(RepoPaths {
        root,
        sol_files,
        docs,
        repo_name,
        commit_hash,
    })
}

pub fn clone_and_build_repo(cli: &Cli, repo_name: &str, commit_hash: &str) -> Result<PathBuf> {
    let docker_volume = format!(
        "{}/{}-{}",
        audit_config().docker_volume,
        repo_name.replace("/", "-"),
        &commit_hash[..6]
    );
    let docker_path = PathBuf::from(&docker_volume);

    if docker_path.exists() {
        log::warn!(
            "Docker volume {} already exists. Removing for clean build.",
            docker_path.display()
        );
        fs::remove_dir_all(&docker_path).with_context(|| {
            format!(
                "Failed to remove existing docker volume {}",
                docker_path.display()
            )
        })?;
    }

    // if github repo clones to multiple sub folders with different apps
    // then repo_name will be something like contracts/plume
    // git clone will clone to contracts (repo_root) and then we cd into plume
    let repo_root = repo_name.split('/').next().unwrap_or(repo_name);

    // Shallow clone for speed and security
    log::info!("git cloning repo...");

    let build_command = cli.generate_build_command();
    let repo_url = &cli.repo;

    let clone_and_build_command =
        format!("git clone --depth=1 {repo_url} {repo_root} && cd {repo_name} && {build_command}");

    let status = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/workspace", docker_volume),
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "bash",
            "-c",
            &clone_and_build_command,
        ])
        .status()
        .context("Failed to clone and build repository in Docker")?;

    if !status.success() {
        // 🔐 Validate the constructed docker volume path
        anyhow::bail!("Clone and Build failed in Docker");
    }

    Ok(PathBuf::from(docker_volume))
}

fn get_commit_hash(repo_url: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["ls-remote", repo_url, "HEAD"])
        .output()
        .context("Failed to run git ls-remote")?;

    if !output.status.success() {
        anyhow::bail!(
            "git ls-remote failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commit_hash = stdout
        .split_whitespace()
        .next()
        .context("Unexpected ls-remote output format")?
        .to_string();

    Ok(commit_hash)
}
