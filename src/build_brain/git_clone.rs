/// This module handles the intake of repositories for analysis.
/// It provides functionality to clone repositories, filter files based on extensions,
/// and organize them for further processing.
use anyhow::Result;
use git2::Repository;
use ignore::gitignore::GitignoreBuilder;
use std::path::PathBuf;
use tempfile::TempDir;
use walkdir::WalkDir;

/// Contains paths to the repository root and relevant files.
/// This struct organizes the paths to Solidity files and documentation
/// that will be processed for analysis.
#[derive(Debug)]
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
    // 1. Create a temporary parent directory (will not auto-delete once we .into_path())
    let tmp = TempDir::new()?;
    let tmp_path = tmp.keep();

    // 2. Extract & sanitize the repo name
    let repo_name = url
        .trim_end_matches(".git")
        .rsplit('/')
        .next()
        .unwrap_or("repo")
        .to_string();

    // 3. First clone into a stub directory
    let stub = tmp_path.join("repo-stub");
    let repo = Repository::clone(url, &stub)?;

    // 4. Read HEAD and get the first 6 chars of the commit SHA
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    let commit_hash = commit.id().to_string();
    let short_hash = &commit_hash[..6];

    // 5. Build the final directory name and rename
    let root = tmp_path.join(format!("{}-{}", &repo_name, short_hash));
    std::fs::rename(&stub, &root)?;

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
        if ign.matched(path, false).is_ignore() {
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
