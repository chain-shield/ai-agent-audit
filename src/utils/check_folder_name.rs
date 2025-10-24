use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::prepare_code::git_clone::extract_list_of_files;

/// Returns true if the file path contains exactly ONE occurrence of any segment from the list.
/// This prevents matching files in nested lib folders (e.g., /lib/.../lib/).
///
/// # Arguments
/// * `file` - The file path to check
/// * `root` - The repository root path
/// * `segments` - List of directory names to match (e.g., ["lib", "library", "libraries"])
///
/// # Returns
/// * `true` if exactly one segment is found in the path
/// * `false` if zero segments, multiple segments, or if the path is a directory
fn path_has_one_segment(file: &Path, root: &Path, segments: &[&str]) -> bool {
    if file.is_dir() {
        return false;
    }
    // Walk up ancestors and check if any directory name equals one of the segments
    // then count how many times, should only happen once
    let mut cursor = file.parent();
    let mut match_count = 0;
    while let Some(dir) = cursor {
        if dir == root {
            break;
        }
        if let Some(name) = dir.file_name().and_then(|n| n.to_str()) {
            let lname = name.to_ascii_lowercase();
            if segments.iter().any(|s| lname == *s) {
                match_count += 1;
            }
        }
        cursor = dir.parent();
    }
    match_count == 1
}

fn path_has_parent_segment(file: &Path, segments: &[&str]) -> bool {
    if file.is_dir() {
        return false;
    }
    // Walk up ancestors and check if any directory name equals one of the segments
    match file.parent() {
        Some(dir) => {
            if let Some(name) = dir.file_name().and_then(|n| n.to_str()) {
                let lname = name.to_ascii_lowercase();
                if segments.iter().any(|s| lname == *s) {
                    return true;
                }
            }
        }
        None => {
            return false;
        }
    }
    false
}
// check if folder contains build config
pub fn contains_build_config(dir: &PathBuf) -> bool {
    let foundry = dir.join("foundry.toml");
    let hardhat_js = dir.join("hardhat.config.js");
    let hardhat_ts = dir.join("hardhat.config.ts");
    let hardhat_cjs = dir.join("hardhat.config.cjs");

    foundry.is_file() || hardhat_js.is_file() || hardhat_ts.is_file() || hardhat_cjs.is_file()
}

pub fn is_test_file(file: &Path, root: &Path) -> bool {
    path_has_one_segment(file, root, &["test", "tests"])
}

pub fn is_script_file(file: &Path) -> bool {
    path_has_parent_segment(file, &["script", "scripts", "deploy"])
}

pub fn is_root_config_file(file: &Path, root: &Path) -> bool {
    // make sure file is in root
    if let Some(parent) = file.parent() {
        if parent != root {
            return false;
        }
    } else {
        return false;
    };

    is_file_config(file)
}

// check if config file is contained in one of the monorepos listed in monorepo_file (contains list of repos)
pub fn is_monorepo_config_file(
    file: &Path,
    root: &Path,
    monorepo_file: &Option<PathBuf>,
) -> Result<bool> {
    if let Some(repo_list_file) = monorepo_file {
        let list_of_paths = extract_list_of_files(&repo_list_file, &root.to_path_buf())?;
        let full_repo_list_paths: Vec<PathBuf> =
            list_of_paths.iter().map(|r| root.join(r)).collect();
        Ok(full_repo_list_paths.iter().any(|p| file.starts_with(p))
            && !file.to_string_lossy().contains("node_modules")
            && !file.to_string_lossy().contains("lib")
            && is_file_config(file))
    } else {
        return Ok(false);
    }
}

pub fn is_file_config(file: &Path) -> bool {
    let filename = file.file_name().unwrap_or_default().to_string_lossy();

    let config_files = [
        "foundry.toml",
        "hardhat.config.js",
        "remappings.txt",
        "package.json",
        ".env.sample",
    ];

    let filename_str = filename.as_ref();
    config_files.iter().any(|f| *f == filename_str)
}

pub fn is_library_package_json(file: &Path, root: &Path) -> bool {
    let filename = file.file_name().unwrap_or_default().to_string_lossy();

    if filename != "package.json" {
        return false;
    }

    path_has_one_segment(file, root, &["lib", "library", "libraries"])
}

/// Returns true if the file is in a library folder (lib, library, or libraries)
/// but NOT in a nested library folder (e.g., /lib/.../lib/).
///
/// This is used to include external dependencies from the main /lib/ folder
/// while excluding nested dependencies.
///
/// # Arguments
/// * `file` - The file path to check
/// * `root` - The repository root path
///
/// # Returns
/// * `true` if the file is in exactly ONE library folder segment
/// * `false` if in nested library folders or not in a library folder at all
pub fn is_library_file(file: &Path, root: &Path) -> bool {
    (file.starts_with("/lib") || file.starts_with("lib"))
        && path_has_one_segment(file, root, &["lib", "library", "libraries"])
}
