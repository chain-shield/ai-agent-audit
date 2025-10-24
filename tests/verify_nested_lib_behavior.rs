use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use ai_agent_audit::utils::check_folder_name::is_library_package_json;

/// This test verifies the exact behavior of path_has_one_segment
/// to ensure we understand what "exactly 1 segment" means
#[test]
fn test_verify_segment_counting_behavior() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    println!("\n=== Verifying segment counting behavior ===\n");

    // Case 1: root/lib/package.json
    // Path components: root -> lib -> package.json
    // Segments matching ["lib", "library", "libraries"]: 1 (just "lib")
    // Expected: ACCEPT (count == 1)
    let case1 = root.join("lib");
    fs::create_dir_all(&case1).unwrap();
    let case1_pkg = case1.join("package.json");
    fs::write(&case1_pkg, "{}").unwrap();
    
    println!("Case 1: root/lib/package.json");
    println!("  Segments: lib");
    println!("  Count: 1");
    let result1 = is_library_package_json(&case1_pkg, root);
    println!("  Result: {} (expected: true)\n", result1);
    assert!(result1);

    // Case 2: root/packages/contracts/lib/utils/package.json
    // Path components: root -> packages -> contracts -> lib -> utils -> package.json
    // Segments matching: 1 (just "lib")
    // Expected: ACCEPT (count == 1)
    let case2 = root.join("packages").join("contracts").join("lib").join("utils");
    fs::create_dir_all(&case2).unwrap();
    let case2_pkg = case2.join("package.json");
    fs::write(&case2_pkg, "{}").unwrap();
    
    println!("Case 2: root/packages/contracts/lib/utils/package.json");
    println!("  Segments: lib");
    println!("  Count: 1");
    let result2 = is_library_package_json(&case2_pkg, root);
    println!("  Result: {} (expected: true)\n", result2);
    assert!(result2);

    // Case 3: root/lib/some-package/lib/package.json
    // Path components: root -> lib -> some-package -> lib -> package.json
    // Segments matching: 2 (lib appears twice)
    // Expected: REJECT (count == 2, not 1)
    let case3 = root.join("lib").join("some-package").join("lib");
    fs::create_dir_all(&case3).unwrap();
    let case3_pkg = case3.join("package.json");
    fs::write(&case3_pkg, "{}").unwrap();
    
    println!("Case 3: root/lib/some-package/lib/package.json");
    println!("  Segments: lib, lib");
    println!("  Count: 2");
    let result3 = is_library_package_json(&case3_pkg, root);
    println!("  Result: {} (expected: false)\n", result3);
    assert!(!result3);

    // Case 4: root/library/lib/package.json
    // Path components: root -> library -> lib -> package.json
    // Segments matching: 2 ("library" and "lib")
    // Expected: REJECT (count == 2, not 1)
    let case4 = root.join("library").join("lib");
    fs::create_dir_all(&case4).unwrap();
    let case4_pkg = case4.join("package.json");
    fs::write(&case4_pkg, "{}").unwrap();
    
    println!("Case 4: root/library/lib/package.json");
    println!("  Segments: library, lib");
    println!("  Count: 2");
    let result4 = is_library_package_json(&case4_pkg, root);
    println!("  Result: {} (expected: false)\n", result4);
    assert!(!result4);

    // Case 5: root/lib/libraries/package.json
    // Path components: root -> lib -> libraries -> package.json
    // Segments matching: 2 ("lib" and "libraries")
    // Expected: REJECT (count == 2, not 1)
    let case5 = root.join("lib").join("libraries");
    fs::create_dir_all(&case5).unwrap();
    let case5_pkg = case5.join("package.json");
    fs::write(&case5_pkg, "{}").unwrap();
    
    println!("Case 5: root/lib/libraries/package.json");
    println!("  Segments: lib, libraries");
    println!("  Count: 2");
    let result5 = is_library_package_json(&case5_pkg, root);
    println!("  Result: {} (expected: false)\n", result5);
    assert!(!result5);

    // Case 6: root/node_modules/lib/package.json
    // Path components: root -> node_modules -> lib -> package.json
    // Segments matching: 1 (just "lib")
    // Expected: ACCEPT (count == 1)
    let case6 = root.join("node_modules").join("lib");
    fs::create_dir_all(&case6).unwrap();
    let case6_pkg = case6.join("package.json");
    fs::write(&case6_pkg, "{}").unwrap();
    
    println!("Case 6: root/node_modules/lib/package.json");
    println!("  Segments: lib");
    println!("  Count: 1");
    let result6 = is_library_package_json(&case6_pkg, root);
    println!("  Result: {} (expected: true)\n", result6);
    assert!(result6);

    // Case 7: root/src/package.json
    // Path components: root -> src -> package.json
    // Segments matching: 0 (no match)
    // Expected: REJECT (count == 0, not 1)
    let case7 = root.join("src");
    fs::create_dir_all(&case7).unwrap();
    let case7_pkg = case7.join("package.json");
    fs::write(&case7_pkg, "{}").unwrap();
    
    println!("Case 7: root/src/package.json");
    println!("  Segments: (none)");
    println!("  Count: 0");
    let result7 = is_library_package_json(&case7_pkg, root);
    println!("  Result: {} (expected: false)\n", result7);
    assert!(!result7);

    // Case 8: root/package.json
    // Path components: root -> package.json
    // Segments matching: 0 (no match)
    // Expected: REJECT (count == 0, not 1)
    let case8_pkg = root.join("package.json");
    fs::write(&case8_pkg, "{}").unwrap();
    
    println!("Case 8: root/package.json");
    println!("  Segments: (none)");
    println!("  Count: 0");
    let result8 = is_library_package_json(&case8_pkg, root);
    println!("  Result: {} (expected: false)\n", result8);
    assert!(!result8);

    println!("=== All verification tests passed ===\n");
}

