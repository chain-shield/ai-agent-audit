use std::fs;
use tempfile::TempDir;

use ai_agent_audit::utils::check_folder_name::is_library_package_json;

#[test]
fn test_lib_package_json_detection() {
    // Create a temporary directory structure
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create various directory structures
    // 1. Direct lib folder: root/lib/package.json
    let lib_dir = root.join("lib");
    fs::create_dir_all(&lib_dir).unwrap();
    let lib_pkg = lib_dir.join("package.json");
    fs::write(&lib_pkg, "{}").unwrap();

    // 2. Nested lib folder: root/node_modules/lib/package.json
    let nested_lib = root.join("node_modules").join("lib");
    fs::create_dir_all(&nested_lib).unwrap();
    let nested_lib_pkg = nested_lib.join("package.json");
    fs::write(&nested_lib_pkg, "{}").unwrap();

    // 3. Library folder: root/library/package.json
    let library_dir = root.join("library");
    fs::create_dir_all(&library_dir).unwrap();
    let library_pkg = library_dir.join("package.json");
    fs::write(&library_pkg, "{}").unwrap();

    // 4. Deeply nested: root/packages/contracts/lib/utils/package.json
    let deep_lib = root
        .join("packages")
        .join("contracts")
        .join("lib")
        .join("utils");
    fs::create_dir_all(&deep_lib).unwrap();
    let deep_lib_pkg = deep_lib.join("package.json");
    fs::write(&deep_lib_pkg, "{}").unwrap();

    // 5. Root package.json (should NOT match as lib config)
    let root_pkg = root.join("package.json");
    fs::write(&root_pkg, "{}").unwrap();

    // 6. Non-lib folder: root/src/package.json (should NOT match)
    let src_dir = root.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    let src_pkg = src_dir.join("package.json");
    fs::write(&src_pkg, "{}").unwrap();

    // Test detection
    println!("\n=== Testing lib package.json detection ===");

    println!("\n1. Direct lib folder:");
    println!("   Path: {}", lib_pkg.display());
    let result1 = is_library_package_json(&lib_pkg, root);
    println!("   Result: {}", result1);
    assert!(result1, "Should detect package.json in lib folder");

    println!("\n2. Nested lib folder:");
    println!("   Path: {}", nested_lib_pkg.display());
    let result2 = is_library_package_json(&nested_lib_pkg, root);
    println!("   Result: {}", result2);
    assert!(result2, "Should detect package.json in nested lib folder");

    println!("\n3. Library folder:");
    println!("   Path: {}", library_pkg.display());
    let result3 = is_library_package_json(&library_pkg, root);
    println!("   Result: {}", result3);
    assert!(result3, "Should detect package.json in library folder");

    println!("\n4. Deeply nested lib:");
    println!("   Path: {}", deep_lib_pkg.display());
    let result4 = is_library_package_json(&deep_lib_pkg, root);
    println!("   Result: {}", result4);
    assert!(
        result4,
        "Should detect package.json in deeply nested lib folder"
    );

    println!("\n5. Root package.json:");
    println!("   Path: {}", root_pkg.display());
    let result5 = is_library_package_json(&root_pkg, root);
    println!("   Result: {}", result5);
    assert!(
        !result5,
        "Should NOT detect root package.json as lib config"
    );

    println!("\n6. Non-lib folder (src):");
    println!("   Path: {}", src_pkg.display());
    let result6 = is_library_package_json(&src_pkg, root);
    println!("   Result: {}", result6);
    assert!(
        !result6,
        "Should NOT detect package.json in src folder as lib config"
    );

    println!("\n=== All tests passed ===\n");
}

#[test]
fn test_lib_folder_file_counting() {
    // Create a temporary directory structure with multiple files in lib
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create lib folder with multiple files
    let lib_dir = root.join("lib");
    fs::create_dir_all(&lib_dir).unwrap();

    // Add various files
    fs::write(lib_dir.join("package.json"), "{}").unwrap();
    fs::write(lib_dir.join("index.js"), "// code").unwrap();
    fs::write(lib_dir.join("utils.ts"), "// code").unwrap();
    fs::write(lib_dir.join("README.md"), "# Readme").unwrap();

    // Create nested structure
    let lib_utils = lib_dir.join("utils");
    fs::create_dir_all(&lib_utils).unwrap();
    fs::write(lib_utils.join("helper.js"), "// code").unwrap();
    fs::write(lib_utils.join("types.ts"), "// code").unwrap();

    // Count all files in lib
    let mut lib_file_count = 0;
    for entry in walkdir::WalkDir::new(&lib_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file() {
            lib_file_count += 1;
            println!("Found file in lib: {}", entry.path().display());
        }
    }

    println!("\nTotal files in lib folder: {}", lib_file_count);
    assert_eq!(lib_file_count, 6, "Should find 6 files in lib folder");

    // Check package.json detection
    let pkg_json = lib_dir.join("package.json");
    let is_lib_pkg = is_library_package_json(&pkg_json, root);
    println!("Is lib package.json: {}", is_lib_pkg);
    assert!(is_lib_pkg, "Should detect package.json in lib folder");
}
