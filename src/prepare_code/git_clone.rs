/// This module handles the intake of repositories for analysis.
/// It provides functionality to clone repositories, filter files based on extensions,
/// and organize them for further processing.
use anyhow::{Context, Result};
use ignore::gitignore::GitignoreBuilder;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

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
    pub fn unique_repo_hash(&self) -> String {
        format!("{}-{}", self.repo_name, &self.commit_hash[..6])
    }
}

pub const DOCKER_VOLUME: &str = "/tmp/audit-analysis";

/// Clones a repository from a URL and filters its files.
///
/// This function:
/// 1. Clones the repository to a temporary directory
/// 2. Filters out build artifacts and node_modules
/// 3. Collects all Solidity files and README documentation
///
/// @param url - URL of the Git repository to clone
/// @return Result containing the filtered repository paths
pub fn clone_and_filter_git_repo(url: &str) -> Result<RepoPaths> {
    // // 1. Create a temporary parent directory (will not auto-delete once we .into_path())
    // let tmp = TempDir::new()?;
    // let tmp_path = tmp.keep();

    // 2. Extract & sanitize the repo name
    let repo_name = url
        .trim_end_matches(".git")
        .rsplit('/')
        .next()
        .unwrap_or("repo")
        .to_string();

    // // 3. First clone into a stub directory
    // let stub = tmp_path.join("repo-stub");

    // 4. Read HEAD and get the first 6 chars of the commit SHA
    let commit_hash = get_commit_hash(url)?;
    let short_hash = &commit_hash[..6];

    // 5. git clone, install, and build in secure docker container
    // returns dierctory where files are located
    let root = clone_and_build_repo(url, &repo_name, short_hash)?;

    // 6. Build .gitignore matcher
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    ign.add_line(None, "node_modules")?;
    let ign = ign.build()?;

    // Initialize vectors to store file paths
    let mut sol_files = Vec::new();
    let mut docs = Vec::new();
    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        // skip if gitignore or simlink
        if ign.matched(path, false).is_ignore()
            || fs::symlink_metadata(path)?.file_type().is_symlink()
        {
            continue;
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some("sol") => sol_files.push(path.to_path_buf()),
            Some("md")
                if path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .map(|f| f.eq_ignore_ascii_case("README.md"))
                    .unwrap_or(false) =>
            {
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

pub fn clone_and_build_repo(repo_url: &str, repo_name: &str, commit_hash: &str) -> Result<PathBuf> {
    let docker_volume = format!("{}/{}-{}", DOCKER_VOLUME, repo_name, &commit_hash[..6]);

    // Shallow clone for speed and security
    log::info!("git cloning repo...");
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
            &format!(
                "git clone --depth=1 {repo_url} {repo_name} && \
             cd {repo_name} && \
             if [ -f foundry.toml ]; then forge install && forge build; \
             elif [ -f hardhat.config.js ] || [ -f hardhat.config.ts ]; then \
             npm install -g hardhat && npm install && npx hardhat compile; \
             else echo 'No build system detected'; fi"
            ),
        ])
        .status()
        .context("Failed to clone and build repository in Docker")?;

    if !status.success() {
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
