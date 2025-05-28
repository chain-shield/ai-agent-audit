/// This module handles the intake of repositories for analysis.
/// It provides functionality to clone repositories, filter files based on extensions,
/// and organize them for further processing.
use anyhow::Result;
use git2::Repository;
use ignore::gitignore::GitignoreBuilder;
use std::path::PathBuf;
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
    // Clone the repository to a temporary directory
    let root = tempfile::tempdir()?.keep();
    Repository::clone(url, &root)?;

    // Set up gitignore patterns to drop build artifacts / node_modules in one pass
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    ign.add_line(None, "node_modules")?;
    let ign = ign.build()?;

    // Initialize vectors to store file paths
    let mut sol_files = Vec::new();
    let mut docs = Vec::new();

    // Walk through the repository and collect relevant files
    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        // Skip ignored paths
        if ign.matched(path, false).is_ignore() {
            continue;
        }
        match path.extension().and_then(|extension| extension.to_str()) {
            // Collect Solidity files
            Some("sol") => sol_files.push(path.to_path_buf()),
            // Accept any Markdown / reStructuredText / plain‑text file *whose name is README.md*
            Some("md")
                // if path
                //     .file_name()
                //     .and_then(|file| file.to_str()) // Option<&str>
                //     .map(|filename| filename.eq_ignore_ascii_case("README.md"))
                //     .unwrap_or(false) =>
                => docs.push(path.to_path_buf()),
            _ => {}
        }
    }

    // Return the collected paths
    Ok(RepoPaths {
        root,
        sol_files,
        docs,
    })
}
