# Import-Only Fallback: Detailed Implementation Specification

## Function Signatures

### 1. Extract Contracts from File
```rust
/// Extract all contract names from a Solidity file.
///
/// Excludes:
/// - Interfaces (contract_type == Interface)
/// - Libraries UNLESS they are in source code folders (repo.source_code_folders)
/// - Standard library contracts (OpenZeppelin, forge-std, etc.)
/// - Mock/test contracts
///
/// Includes:
/// - Concrete contracts
/// - Abstract contracts
/// - Libraries in source code folders (e.g., /src) - matches BFS behavior
///
/// # Arguments
/// * `file` - Path to Solidity file
/// * `repo` - Repository paths
///
/// # Returns
/// * `Vec<String>` - List of contract names found in the file
async fn extract_contracts_from_file(
    file: &PathBuf,
    repo: &RepoPaths,
) -> Result<Vec<String>>
```

**Implementation Details**:
- Read file content with `fs::read_to_string()`
- Use `SOLIDITY_REGEXES.contract_decl` to find all declarations
- Check if file is in source code folder: `repo.source_code_folders.iter().any(|src| file.starts_with(src))`
- For each match:
  - Extract declaration type (contract, interface, library, abstract contract)
  - Extract contract name
  - Skip if type is "interface"
  - Skip if type is "library" AND file is NOT in source code folders
  - Skip if `is_standard_library_contract_name(name)` returns true
  - Skip if file path contains "/mocks/" or "/test/"
  - Skip if contract name contains "mock" (case-insensitive)
  - Add to results
- Return Vec<String>

### 2. Main Fallback Function
```rust
/// Generate contract sets using import-only traversal (fallback when Slither fails).
///
/// This function replaces the BFS call graph traversal (lines 85-140 in codeblocks.rs)
/// when Slither is unavailable or fails. It discovers contracts by traversing import
/// dependencies up to `max_depth` levels deep.
///
/// # Algorithm
/// 1. Level 0: Start with main_contract
/// 2. For each level from 1 to max_depth:
///    - Analyze imports of contracts from previous level
///    - Discover contracts at current level
/// 3. Respect token budget at each level
/// 4. Track visited files to avoid cycles
///
/// # Arguments
/// * `main_contract` - The contract to analyze
/// * `repo` - Repository paths
/// * `max_depth` - Maximum import traversal depth (default 3, configurable)
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `(contracts, contracts_with_depth, token_count)` where:
///   - `contracts`: All unique contracts discovered (excluding libs/interfaces)
///   - `contracts_with_depth`: Contracts at depth 1-2 (for compatibility with BFS)
///   - `token_count`: Total tokens consumed
async fn generate_contracts_via_import_traversal(
    main_contract: &str,
    repo: &RepoPaths,
    max_depth: usize,
    token_budget: usize,
) -> Result<(HashSet<String>, HashSet<String>, usize)>
```

## Detailed Implementation Steps

### Step 1: Initialize Data Structures
```rust
let mut level_0_contracts = HashSet::new();
level_0_contracts.insert(main_contract.to_string());

let mut level_1_contracts = HashSet::new();
let mut level_2_contracts = HashSet::new();
let mut level_3_contracts = HashSet::new();

let mut visited_files = HashSet::new();
let mut token_count = 0_usize;
```

### Step 2: Process Level 0 (Main Contract)
```rust
// Get file for main contract
let (main_file, _) = get_file_from_contract(main_contract, repo)
    .await
    .ok_or_else(|| anyhow::anyhow!("Could not find file for main contract: {}", main_contract))?;

visited_files.insert(main_file.clone());

// Get imports
let import_deps = detect_source_code_dependencies(main_contract, repo).await?;

// Extract contracts from source files
for source_file in &import_deps.source_files {
    if visited_files.contains(source_file) {
        continue;
    }
    
    let contracts = extract_contracts_from_file(source_file, repo).await?;
    for contract in contracts {
        if !is_standard_library_contract_name(&contract) && !is_standard_interface_name(&contract) {
            level_1_contracts.insert(contract);
        }
    }
}

// Add interface implementations
for (impl_contract, _) in import_deps.interface_implementations {
    if !is_standard_library_contract_name(&impl_contract) && !is_standard_interface_name(&impl_contract) {
        level_1_contracts.insert(impl_contract);
    }
}

// Update token count (estimate based on file size or use actual token counting)
// For now, we'll defer token counting until assembly phase
```

### Step 3: Process Level 1 (Direct Imports)
```rust
for contract in &level_1_contracts.clone() {
    // Get file
    let file_opt = get_file_from_contract(contract, repo).await
        .or_else(|| get_file_from_lib_contract(contract, repo).await);
    
    let (file, _) = match file_opt {
        Some(f) => f,
        None => {
            warn!("Could not find file for contract: {}", contract);
            continue;
        }
    };
    
    // Skip if already visited
    if visited_files.contains(&file) {
        continue;
    }
    visited_files.insert(file.clone());
    
    // Get imports
    let import_deps = detect_source_code_dependencies(contract, repo).await?;
    
    // Extract contracts from source files
    for source_file in &import_deps.source_files {
        if visited_files.contains(source_file) {
            continue;
        }
        
        let contracts = extract_contracts_from_file(source_file, repo).await?;
        for c in contracts {
            if !is_standard_library_contract_name(&c) && !is_standard_interface_name(&c) {
                level_2_contracts.insert(c);
            }
        }
    }
    
    // Add interface implementations
    for (impl_contract, _) in import_deps.interface_implementations {
        if !is_standard_library_contract_name(&impl_contract) && !is_standard_interface_name(&impl_contract) {
            level_2_contracts.insert(impl_contract);
        }
    }
}
```

### Step 4: Process Level 2 (Transitive Imports)
```rust
// Same logic as Level 1, but populate level_3_contracts
// (Code structure identical to Step 3, just different level variables)
```

### Step 5: Process Level 3 (Deep Transitive Imports)
```rust
// Only track contracts, don't recurse further
for contract in &level_3_contracts.clone() {
    let file_opt = get_file_from_contract(contract, repo).await
        .or_else(|| get_file_from_lib_contract(contract, repo).await);
    
    if let Some((file, _)) = file_opt {
        visited_files.insert(file);
    }
}
```

### Step 6: Assemble Results
```rust
// Combine all levels
let mut contracts = HashSet::new();
contracts.extend(level_0_contracts);
contracts.extend(level_1_contracts.clone());
contracts.extend(level_2_contracts.clone());
contracts.extend(level_3_contracts);

// contracts_with_depth = depth 1-2 (matches BFS behavior)
let mut contracts_with_depth = HashSet::new();
contracts_with_depth.extend(level_1_contracts);
contracts_with_depth.extend(level_2_contracts);

// Token count will be calculated during assembly phase (existing logic)
// For now, return 0 as placeholder
Ok((contracts, contracts_with_depth, token_count))
```

## Edge Cases to Handle

1. **Circular Imports**: A imports B, B imports A
   - Solution: `visited_files` HashSet prevents infinite loops

2. **Contract Not Found**: File lookup fails
   - Solution: Log warning and continue (don't fail entire analysis)

3. **Multiple Contracts in One File**: File contains Contract1, Contract2, Interface1
   - Solution: `extract_contracts_from_file()` returns Vec, we process all

4. **Same Contract Name in Different Files**: src/Token.sol and lib/Token.sol
   - Solution: Track by file path in `visited_files`, not contract name

5. **Token Budget Exceeded**: Should we stop early?
   - Solution: For initial implementation, collect all contracts and let assembly phase handle budget
   - Future: Add early termination if needed

## Testing Checklist

- [ ] Test `extract_contracts_from_file()` with file containing multiple contracts
- [ ] Test `extract_contracts_from_file()` with file containing interfaces (should exclude)
- [ ] Test `extract_contracts_from_file()` with file containing libraries (should exclude)
- [ ] Test circular import detection (A → B → A)
- [ ] Test depth tracking (verify contracts_with_depth contains only depth 1-2)
- [ ] Test with Ekubo repository (real-world case where Slither fails)
- [ ] Verify standard library exclusion (OpenZeppelin, forge-std)
- [ ] Verify mock/test contract exclusion

## Location in Codebase

**File**: `src/enumerator/codeblocks.rs`

**Add after line 803** (end of file):
- `extract_contracts_from_file()` function
- `generate_contracts_via_import_traversal()` function

**DO NOT modify** `generate_codeblock_from_codebase()` yet - awaiting user approval.

