use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use ai_agent_audit::utils::check_folder_name::is_library_package_json;

#[test]
fn test_nested_lib_folders_should_be_rejected() {
    // Create a temporary directory structure
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Test case: root/lib/some-package/lib/package.json
    // This has TWO "lib" segments in the path - should be REJECTED
    let nested_lib_lib = root.join("lib").join("some-package").join("lib");
    fs::create_dir_all(&nested_lib_lib).unwrap();
    let nested_lib_lib_pkg = nested_lib_lib.join("package.json");
    fs::write(&nested_lib_lib_pkg, "{}").unwrap();

    // Test case: root/lib/package.json
    // This has ONE "lib" segment - should be ACCEPTED
    let single_lib = root.join("lib");
    fs::create_dir_all(&single_lib).unwrap();
    let single_lib_pkg = single_lib.join("package.json");
    fs::write(&single_lib_pkg, "{}").unwrap();

    // Test case: root/packages/lib/package.json
    // This has ONE "lib" segment - should be ACCEPTED
    let packages_lib = root.join("packages").join("lib");
    fs::create_dir_all(&packages_lib).unwrap();
    let packages_lib_pkg = packages_lib.join("package.json");
    fs::write(&packages_lib_pkg, "{}").unwrap();

    // Test case: root/lib/utils/package.json
    // This has ONE "lib" segment - should be ACCEPTED
    let lib_utils = root.join("lib").join("utils");
    fs::create_dir_all(&lib_utils).unwrap();
    let lib_utils_pkg = lib_utils.join("package.json");
    fs::write(&lib_utils_pkg, "{}").unwrap();

    // Test case: root/library/lib/package.json
    // This has TWO segments (library + lib) - should be REJECTED
    let library_lib = root.join("library").join("lib");
    fs::create_dir_all(&library_lib).unwrap();
    let library_lib_pkg = library_lib.join("package.json");
    fs::write(&library_lib_pkg, "{}").unwrap();

    println!("\n=== Testing nested lib folder rejection ===");

    println!("\n1. Nested lib/lib (should be REJECTED):");
    println!("   Path: {}", nested_lib_lib_pkg.display());
    let result1 = is_library_package_json(&nested_lib_lib_pkg, root);
    println!("   Result: {}", result1);
    assert!(
        !result1,
        "Should REJECT package.json in nested lib/lib folder"
    );

    println!("\n2. Single lib (should be ACCEPTED):");
    println!("   Path: {}", single_lib_pkg.display());
    let result2 = is_library_package_json(&single_lib_pkg, root);
    println!("   Result: {}", result2);
    assert!(result2, "Should ACCEPT package.json in single lib folder");

    println!("\n3. packages/lib (should be ACCEPTED):");
    println!("   Path: {}", packages_lib_pkg.display());
    let result3 = is_library_package_json(&packages_lib_pkg, root);
    println!("   Result: {}", result3);
    assert!(
        result3,
        "Should ACCEPT package.json in packages/lib folder"
    );

    println!("\n4. lib/utils (should be ACCEPTED):");
    println!("   Path: {}", lib_utils_pkg.display());
    let result4 = is_library_package_json(&lib_utils_pkg, root);
    println!("   Result: {}", result4);
    assert!(result4, "Should ACCEPT package.json in lib/utils folder");

    println!("\n5. library/lib (should be REJECTED - has 2 segments):");
    println!("   Path: {}", library_lib_pkg.display());
    let result5 = is_library_package_json(&library_lib_pkg, root);
    println!("   Result: {}", result5);
    assert!(
        !result5,
        "Should REJECT package.json in library/lib folder (2 segments)"
    );

    println!("\n=== All tests passed ===\n");
}

#[test]
fn test_path_has_one_segment_logic() {
    // Manual verification of the logic
    let root = PathBuf::from("/repo");

    // Simulate the path_has_one_segment logic
    fn count_segments(file: &PathBuf, root: &PathBuf, segments: &[&str]) -> usize {
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
        match_count
    }

    // Test cases
    let segments = &["lib", "library", "libraries"];

    // Should have count = 1
    let path1 = PathBuf::from("/repo/lib/package.json");
    assert_eq!(count_segments(&path1, &root, segments), 1);

    let path2 = PathBuf::from("/repo/packages/lib/package.json");
    assert_eq!(count_segments(&path2, &root, segments), 1);

    let path3 = PathBuf::from("/repo/lib/utils/package.json");
    assert_eq!(count_segments(&path3, &root, segments), 1);

    // Should have count = 2 (nested)
    let path4 = PathBuf::from("/repo/lib/some-package/lib/package.json");
    assert_eq!(count_segments(&path4, &root, segments), 2);

    let path5 = PathBuf::from("/repo/library/lib/package.json");
    assert_eq!(count_segments(&path5, &root, segments), 2);

    // Should have count = 0
    let path6 = PathBuf::from("/repo/src/package.json");
    assert_eq!(count_segments(&path6, &root, segments), 0);

    let path7 = PathBuf::from("/repo/package.json");
    assert_eq!(count_segments(&path7, &root, segments), 0);

    println!("✅ All segment counting tests passed");
}

