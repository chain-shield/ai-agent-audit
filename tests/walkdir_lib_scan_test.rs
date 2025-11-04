use ignore::gitignore::GitignoreBuilder;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Test WalkDir scanning on actual local repository
/// This test uses a local copy of the repo to verify lib folder scanning works
#[test]
fn test_walkdir_scans_lib_folders_in_local_repo() {
    // Path to your local repo copy
    let local_repo = PathBuf::from(
        "/Users/apmfree/Desktop/Audit/2025-09-summer-fi-governance-v2-chainshieldai/summer-earn-protocol",
    );

    // Skip test if path doesn't exist
    if !local_repo.exists() {
        println!(
            "Skipping test - local repo not found at: {}",
            local_repo.display()
        );
        return;
    }

    println!("\n=== Testing WalkDir on Local Repo ===");
    println!("Repo path: {}", local_repo.display());

    // Set up gitignore patterns (same as production code)
    let mut ign = GitignoreBuilder::new(&local_repo);
    ign.add_line(None, "dist").unwrap();
    ign.add_line(None, "out").unwrap();
    ign.add_line(None, "node_modules").unwrap();
    let ign = ign.build().unwrap();

    // Counters
    let mut total_files_scanned = 0;
    let mut lib_files_scanned = 0;
    let mut package_json_total = 0;
    let mut package_json_in_lib = 0;

    let mut lib_file_examples: Vec<PathBuf> = Vec::new();
    let mut package_json_examples: Vec<PathBuf> = Vec::new();
    let mut lib_package_json_examples: Vec<PathBuf> = Vec::new();

    // Walk the directory (same logic as production code)
    for entry in WalkDir::new(&local_repo)
        .into_iter()
        .filter_entry(|e| {
            let path = e.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            // Skip out/, cache/, and .git (same as production)
            !(name == "out" || name == "cache" || name == ".git")
        })
        .filter_map(Result::ok)
    {
        let path = entry.path();

        // Skip directories
        if entry.file_type().is_dir() {
            continue;
        }

        // Skip if gitignore or symlink (same as production)
        if ign.matched(path, false).is_ignore() {
            continue;
        }
        if std::fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            continue;
        }

        // Count every file that passes filters
        total_files_scanned += 1;

        // Check if file is under any lib*/library* folder
        let is_in_lib = is_in_lib_folder(path, &local_repo);
        if is_in_lib {
            lib_files_scanned += 1;
            if lib_file_examples.len() < 10 {
                lib_file_examples.push(path.to_path_buf());
            }
        }

        // Check if it's a package.json
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        if filename == "package.json" {
            package_json_total += 1;
            if package_json_examples.len() < 10 {
                package_json_examples.push(path.to_path_buf());
            }

            if is_in_lib {
                package_json_in_lib += 1;
                if lib_package_json_examples.len() < 10 {
                    lib_package_json_examples.push(path.to_path_buf());
                }
            }
        }
    }

    // Print results
    println!("\n=== Scan Results ===");
    println!("Total files scanned: {}", total_files_scanned);
    println!("Files in lib*/library* folders: {}", lib_files_scanned);
    println!("Total package.json files: {}", package_json_total);
    println!("package.json in lib*/library*: {}", package_json_in_lib);

    println!("\n=== First 10 Files in lib* Folders ===");
    for (i, path) in lib_file_examples.iter().enumerate() {
        println!("  {}. {}", i + 1, path.display());
    }

    println!("\n=== First 10 package.json Files (anywhere) ===");
    for (i, path) in package_json_examples.iter().enumerate() {
        println!("  {}. {}", i + 1, path.display());
    }

    println!("\n=== package.json Files in lib* Folders ===");
    if lib_package_json_examples.is_empty() {
        println!("  NONE FOUND!");
        println!("\n  This means either:");
        println!("  1. There are no package.json files in lib folders");
        println!("  2. The lib folders are empty (dependencies not installed)");
        println!("  3. The lib folders are gitignored or symlinked");
    } else {
        for (i, path) in lib_package_json_examples.iter().enumerate() {
            println!("  {}. {}", i + 1, path.display());
        }
    }

    // Check if lib folder exists
    let lib_folder = local_repo.join("lib");
    println!("\n=== Lib Folder Check ===");
    println!("lib folder path: {}", lib_folder.display());
    println!("lib folder exists: {}", lib_folder.exists());

    if lib_folder.exists() {
        // Count items in lib folder
        let lib_contents: Vec<_> = std::fs::read_dir(&lib_folder)
            .map(|entries| entries.filter_map(Result::ok).collect())
            .unwrap_or_default();

        println!("lib folder item count: {}", lib_contents.len());
        println!("\nlib folder contents:");
        for entry in lib_contents.iter().take(20) {
            let path = entry.path();
            let is_dir = path.is_dir();
            let is_symlink = std::fs::symlink_metadata(&path)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);
            println!(
                "  - {} {}{}",
                path.file_name().unwrap_or_default().to_string_lossy(),
                if is_dir { "(dir)" } else { "(file)" },
                if is_symlink { " [symlink]" } else { "" }
            );
        }

        // Check if lib subdirectories are empty
        for entry in lib_contents.iter() {
            let path = entry.path();
            if path.is_dir() {
                let subdir_count = std::fs::read_dir(&path)
                    .map(|entries| entries.count())
                    .unwrap_or(0);
                println!(
                    "\n  {} contains {} items",
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    subdir_count
                );

                if subdir_count == 0 {
                    println!("    ⚠️  EMPTY! Dependencies not installed.");
                }
            }
        }
    }

    println!("\n=== Test Complete ===\n");

    // Assertions
    assert!(total_files_scanned > 0, "Should scan at least some files");

    // Note: We don't assert lib_package_json_examples > 0 because the lib folder might be empty
    // This test is for diagnostics to see what WalkDir actually finds
}

/// Helper function to check if a file is under any lib*/library*/node_modules folder
fn is_in_lib_folder(file: &Path, root: &Path) -> bool {
    let mut cursor = file.parent();
    while let Some(dir) = cursor {
        if dir == root {
            break;
        }
        if let Some(name) = dir.file_name().and_then(|n| n.to_str()) {
            let lname = name.to_ascii_lowercase();
            if lname == "lib" || lname.contains("library") || lname == "node_modules" {
                return true;
            }
        }
        cursor = dir.parent();
    }
    false
}

#[test]
fn test_lib_folder_detection_logic() {
    // Test the helper function with mock paths
    let root = PathBuf::from("/repo");

    // Should match
    assert!(is_in_lib_folder(
        &PathBuf::from("/repo/lib/package.json"),
        &root
    ));
    assert!(is_in_lib_folder(
        &PathBuf::from("/repo/lib/utils/package.json"),
        &root
    ));
    assert!(is_in_lib_folder(
        &PathBuf::from("/repo/packages/lib/package.json"),
        &root
    ));
    assert!(is_in_lib_folder(
        &PathBuf::from("/repo/library/package.json"),
        &root
    ));

    // Should NOT match
    assert!(!is_in_lib_folder(
        &PathBuf::from("/repo/package.json"),
        &root
    ));
    assert!(!is_in_lib_folder(
        &PathBuf::from("/repo/src/package.json"),
        &root
    ));
    assert!(!is_in_lib_folder(
        &PathBuf::from("/repo/node_modules/package.json"),
        &root
    ));

    println!("✓ Lib folder detection logic works correctly");
}
