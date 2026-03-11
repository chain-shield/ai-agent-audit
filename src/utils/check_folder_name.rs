use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::prepare_code::git_clone::{RepoPaths, extract_list_of_files};

/// Returns true if the file path contains exactly ONE occurrence of any segment from the list.
/// This prevents matching files in nested lib folders (e.g., /lib/.../lib/).
///
/// IMPORTANT: This counts ALL occurrences of ANY segment in the list.
/// For example, if segments = ["lib", "node_modules"]:
/// - path/lib/foo → 1 occurrence (lib) → returns true
/// - path/node_modules/foo → 1 occurrence (node_modules) → returns true
/// - path/lib/node_modules/foo → 2 occurrences (lib + node_modules) → returns false
/// - path/lib/foo/lib/bar → 2 occurrences (lib + lib) → returns false
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
    // Count TOTAL occurrences across ALL segments
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

/// Returns true if the file path contains ANY of the specified segments.
/// Used to detect if a path contains lib/library/libraries folders.
///
/// # Arguments
/// * `file` - The file path to check
/// * `root` - The root directory of the repository
/// * `segments` - The segment names to check for (e.g., ["lib", "library"])
///
/// # Returns
/// * `true` if any segment is found in the path
/// * `false` if no segments are found
fn has_any_segment(file: &Path, root: &Path, segments: &[&str]) -> bool {
    let mut cursor = file.parent();
    while let Some(dir) = cursor {
        if dir == root {
            break;
        }
        if let Some(name) = dir.file_name().and_then(|n| n.to_str()) {
            let lname = name.to_ascii_lowercase();
            if segments.iter().any(|s| lname == *s) {
                return true;
            }
        }
        cursor = dir.parent();
    }
    false
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
pub fn contains_build_config(dir: &Path) -> bool {
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
        let list_of_paths = extract_list_of_files(repo_list_file, root)?;
        let full_repo_list_paths: Vec<PathBuf> =
            list_of_paths.iter().map(|r| root.join(r)).collect();
        Ok(full_repo_list_paths.iter().any(|p| file.starts_with(p))
            && !file.to_string_lossy().contains("node_modules")
            && !file.to_string_lossy().contains("lib")
            && is_file_config(file))
    } else {
        Ok(false)
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
    config_files.contains(&filename_str)
}

pub fn is_library_package_json(file: &Path, root: &Path) -> bool {
    let filename = file.file_name().unwrap_or_default().to_string_lossy();

    if filename != "package.json" {
        return false;
    }

    // Check if path contains node_modules
    let has_node_modules = has_any_segment(file, root, &["node_modules"]);
    let has_lib_folders = has_any_segment(file, root, &["lib", "library", "libraries"]);

    // For node_modules packages
    if has_node_modules {
        // CRITICAL: Reject if path contains BOTH node_modules AND lib/library/libraries
        // This prevents: node_modules/ethers/lib/package.json
        if has_lib_folders {
            return false;
        }

        // CRITICAL: Reject if path contains multiple node_modules
        // This prevents: node_modules/pkg/node_modules/nested/package.json
        if !path_has_one_segment(file, root, &["node_modules"]) {
            return false;
        }

        // Extract package name from path: node_modules/{package} or node_modules/@scope/{package}
        let Some(package_name) = extract_package_name_from_node_modules(file) else {
            return false;
        };

        // Check if this package is listed in root package.json
        let root_package_json = root.join("package.json");
        if !root_package_json.exists() {
            return false;
        }

        // Read and parse root package.json
        let Ok(content) = std::fs::read_to_string(&root_package_json) else {
            return false;
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
            return false;
        };

        // Check if package exists in dependencies or devDependencies
        let is_direct_dep = json
            .get("dependencies")
            .and_then(|deps| deps.get(&package_name))
            .is_some()
            || json
                .get("devDependencies")
                .and_then(|deps| deps.get(&package_name))
                .is_some();

        return is_direct_dep;
    }

    // For lib/library/libraries folders (Foundry projects)
    // Reject if contains node_modules (already handled above)
    if has_lib_folders && !has_node_modules {
        return path_has_one_segment(file, root, &["lib", "library", "libraries"]);
    }

    false
}

/// Extract package name from a path in node_modules.
/// Examples:
/// - node_modules/lodash/package.json -> Some("lodash")
/// - node_modules/@openzeppelin/contracts/package.json -> Some("@openzeppelin/contracts")
/// - node_modules/pkg/node_modules/nested/package.json -> Some("pkg") (extracts first occurrence)
pub fn extract_package_name_from_node_modules(file: &Path) -> Option<String> {
    let components: Vec<_> = file.components().collect();

    // Find the node_modules component
    let node_modules_idx = components
        .iter()
        .position(|c| c.as_os_str() == "node_modules")?;

    // Get the component right after node_modules
    let first_component = components.get(node_modules_idx + 1)?;
    let first_name = first_component.as_os_str().to_str()?;

    // Check if it's a scoped package (starts with @)
    if first_name.starts_with('@') {
        // Scoped package: @scope/package
        let second_component = components.get(node_modules_idx + 2)?;
        let second_name = second_component.as_os_str().to_str()?;
        Some(format!("{}/{}", first_name, second_name))
    } else {
        // Regular package: package
        Some(first_name.to_string())
    }
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
/// * `true` if the file is in exactly ONE library folder segment AND is NOT in code source folder
/// * `false` if in nested library folders or not in a library folder at all
pub fn is_library_file(file: &Path, repo: &RepoPaths) -> bool {
    let is_source = repo.source_code_folders.iter().any(|f| file.starts_with(f));
    path_has_one_segment(
        file,
        &repo.root,
        &["lib", "library", "libraries", "node_modules"],
    ) && !is_source
}
