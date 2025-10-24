use std::fs;
use tempfile::TempDir;

use ai_agent_audit::utils::check_folder_name::is_library_file;

#[test]
fn test_library_file_detection() {
    // Create a temporary directory structure
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    println!("\n=== Testing library file detection ===");
    println!("Root: {}\n", root.display());

    // Test case 1: File in direct lib folder - SHOULD BE ACCEPTED
    let lib_dir = root.join("lib");
    fs::create_dir_all(&lib_dir).unwrap();
    let lib_file = lib_dir.join("Token.sol");
    fs::write(&lib_file, "contract Token {}").unwrap();

    println!("1. Direct lib folder:");
    println!("   Path: {}", lib_file.display());
    let result1 = is_library_file(&lib_file, root);
    println!("   Result: {} (expected: true)\n", result1);
    assert!(result1, "Should detect file in lib folder");

    // Test case 2: File in nested lib folder - SHOULD BE REJECTED
    let nested_lib = root.join("lib").join("openzeppelin").join("lib");
    fs::create_dir_all(&nested_lib).unwrap();
    let nested_lib_file = nested_lib.join("SafeMath.sol");
    fs::write(&nested_lib_file, "library SafeMath {}").unwrap();

    println!("2. Nested lib/lib folder:");
    println!("   Path: {}", nested_lib_file.display());
    let result2 = is_library_file(&nested_lib_file, root);
    println!("   Result: {} (expected: false)\n", result2);
    assert!(
        !result2,
        "Should REJECT file in nested lib/lib folder"
    );

    // Test case 3: File in library folder - SHOULD BE ACCEPTED
    let library_dir = root.join("library");
    fs::create_dir_all(&library_dir).unwrap();
    let library_file = library_dir.join("Utils.sol");
    fs::write(&library_file, "library Utils {}").unwrap();

    println!("3. Library folder:");
    println!("   Path: {}", library_file.display());
    let result3 = is_library_file(&library_file, root);
    println!("   Result: {} (expected: true)\n", result3);
    assert!(result3, "Should detect file in library folder");

    // Test case 4: File in libraries folder - SHOULD BE ACCEPTED
    let libraries_dir = root.join("libraries");
    fs::create_dir_all(&libraries_dir).unwrap();
    let libraries_file = libraries_dir.join("Math.sol");
    fs::write(&libraries_file, "library Math {}").unwrap();

    println!("4. Libraries folder:");
    println!("   Path: {}", libraries_file.display());
    let result4 = is_library_file(&libraries_file, root);
    println!("   Result: {} (expected: true)\n", result4);
    assert!(result4, "Should detect file in libraries folder");

    // Test case 5: File in src folder - SHOULD BE REJECTED
    let src_dir = root.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    let src_file = src_dir.join("MyContract.sol");
    fs::write(&src_file, "contract MyContract {}").unwrap();

    println!("5. Src folder:");
    println!("   Path: {}", src_file.display());
    let result5 = is_library_file(&src_file, root);
    println!("   Result: {} (expected: false)\n", result5);
    assert!(!result5, "Should NOT detect file in src folder as library");

    // Test case 6: File in node_modules/lib - SHOULD BE ACCEPTED
    let node_modules_lib = root.join("node_modules").join("lib");
    fs::create_dir_all(&node_modules_lib).unwrap();
    let node_modules_file = node_modules_lib.join("External.sol");
    fs::write(&node_modules_file, "contract External {}").unwrap();

    println!("6. node_modules/lib folder:");
    println!("   Path: {}", node_modules_file.display());
    let result6 = is_library_file(&node_modules_file, root);
    println!("   Result: {} (expected: true)\n", result6);
    assert!(
        result6,
        "Should detect file in node_modules/lib folder"
    );

    // Test case 7: File in lib/utils subdirectory - SHOULD BE ACCEPTED
    let lib_utils = root.join("lib").join("utils");
    fs::create_dir_all(&lib_utils).unwrap();
    let lib_utils_file = lib_utils.join("Helper.sol");
    fs::write(&lib_utils_file, "library Helper {}").unwrap();

    println!("7. lib/utils subdirectory:");
    println!("   Path: {}", lib_utils_file.display());
    let result7 = is_library_file(&lib_utils_file, root);
    println!("   Result: {} (expected: true)\n", result7);
    assert!(
        result7,
        "Should detect file in lib/utils subdirectory"
    );

    // Test case 8: File in library/lib - SHOULD BE REJECTED (2 segments)
    let library_lib = root.join("library").join("lib");
    fs::create_dir_all(&library_lib).unwrap();
    let library_lib_file = library_lib.join("Mixed.sol");
    fs::write(&library_lib_file, "contract Mixed {}").unwrap();

    println!("8. library/lib folder (2 segments):");
    println!("   Path: {}", library_lib_file.display());
    let result8 = is_library_file(&library_lib_file, root);
    println!("   Result: {} (expected: false)\n", result8);
    assert!(
        !result8,
        "Should REJECT file in library/lib folder (2 segments)"
    );

    // Test case 9: File in contracts/lib - SHOULD BE ACCEPTED
    let contracts_lib = root.join("contracts").join("lib");
    fs::create_dir_all(&contracts_lib).unwrap();
    let contracts_lib_file = contracts_lib.join("ContractLib.sol");
    fs::write(&contracts_lib_file, "library ContractLib {}").unwrap();

    println!("9. contracts/lib folder:");
    println!("   Path: {}", contracts_lib_file.display());
    let result9 = is_library_file(&contracts_lib_file, root);
    println!("   Result: {} (expected: true)\n", result9);
    assert!(
        result9,
        "Should detect file in contracts/lib folder"
    );

    // Test case 10: Root file - SHOULD BE REJECTED
    let root_file = root.join("RootContract.sol");
    fs::write(&root_file, "contract RootContract {}").unwrap();

    println!("10. Root file:");
    println!("   Path: {}", root_file.display());
    let result10 = is_library_file(&root_file, root);
    println!("   Result: {} (expected: false)\n", result10);
    assert!(!result10, "Should NOT detect root file as library");

    println!("=== All library file detection tests passed ===\n");
}

#[test]
fn test_library_file_edge_cases() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    println!("\n=== Testing library file edge cases ===\n");

    // Edge case 1: lib/lib/lib (triple nested) - SHOULD BE REJECTED
    let triple_lib = root.join("lib").join("lib").join("lib");
    fs::create_dir_all(&triple_lib).unwrap();
    let triple_lib_file = triple_lib.join("Triple.sol");
    fs::write(&triple_lib_file, "contract Triple {}").unwrap();

    println!("1. Triple nested lib/lib/lib:");
    println!("   Path: {}", triple_lib_file.display());
    let result1 = is_library_file(&triple_lib_file, root);
    println!("   Result: {} (expected: false)\n", result1);
    assert!(!result1, "Should REJECT triple nested lib folders");

    // Edge case 2: libraries/library/lib - SHOULD BE REJECTED (3 segments)
    let multi_segment = root.join("libraries").join("library").join("lib");
    fs::create_dir_all(&multi_segment).unwrap();
    let multi_segment_file = multi_segment.join("Multi.sol");
    fs::write(&multi_segment_file, "contract Multi {}").unwrap();

    println!("2. libraries/library/lib (3 segments):");
    println!("   Path: {}", multi_segment_file.display());
    let result2 = is_library_file(&multi_segment_file, root);
    println!("   Result: {} (expected: false)\n", result2);
    assert!(
        !result2,
        "Should REJECT path with 3 library segments"
    );

    // Edge case 3: Very deep lib path - SHOULD BE ACCEPTED (only 1 lib)
    let deep_lib = root
        .join("lib")
        .join("vendor")
        .join("openzeppelin")
        .join("contracts")
        .join("token");
    fs::create_dir_all(&deep_lib).unwrap();
    let deep_lib_file = deep_lib.join("ERC20.sol");
    fs::write(&deep_lib_file, "contract ERC20 {}").unwrap();

    println!("3. Deep lib path (lib/vendor/openzeppelin/contracts/token):");
    println!("   Path: {}", deep_lib_file.display());
    let result3 = is_library_file(&deep_lib_file, root);
    println!("   Result: {} (expected: true)\n", result3);
    assert!(
        result3,
        "Should ACCEPT deep path with only 1 lib segment"
    );

    // Edge case 4: Case sensitivity - "LIB" folder - SHOULD BE ACCEPTED
    let upper_lib = root.join("LIB");
    fs::create_dir_all(&upper_lib).unwrap();
    let upper_lib_file = upper_lib.join("Upper.sol");
    fs::write(&upper_lib_file, "contract Upper {}").unwrap();

    println!("4. Uppercase LIB folder:");
    println!("   Path: {}", upper_lib_file.display());
    let result4 = is_library_file(&upper_lib_file, root);
    println!("   Result: {} (expected: true)\n", result4);
    assert!(
        result4,
        "Should ACCEPT uppercase LIB folder (case-insensitive)"
    );

    println!("=== All edge case tests passed ===\n");
}

