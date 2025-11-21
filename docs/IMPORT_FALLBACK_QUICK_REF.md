# Import-Only Fallback: Quick Reference Card

## User Requirements (Verbatim)

> "lets include non-standard libraries that are in a source code folder (typically /src, defined by repo.source_code_folders). this is aligned with what current BFS code does."

> "instead of setting depth to 3, set it to param `max_depth` which is 3 by default, but we could tweak at anytime."

> "i am assuming contract_to_func_map will return empty of slither fails. that is how we decide to switch to fallback mode. should also log warning as well."

## Implementation Checklist

### Function 1: `extract_contracts_from_file()`

**Signature**:
```rust
async fn extract_contracts_from_file(
    file: &PathBuf,
    repo: &RepoPaths,
) -> Result<Vec<String>>
```

**Key Logic**:
```rust
// Check if file is in source code folder
let is_source_file = repo.source_code_folders
    .iter()
    .any(|src| file.starts_with(src));

// For each contract declaration:
// - Skip if type is "interface"
// - Skip if type is "library" AND NOT is_source_file
// - Skip if is_standard_library_contract_name(name)
// - Skip if path contains "/mocks/" or "/test/"
// - Skip if name contains "mock" (case-insensitive)
// - Otherwise, include
```

**What to Include**:
- ✅ Concrete contracts
- ✅ Abstract contracts
- ✅ Libraries in source code folders (e.g., /src/libraries/)

**What to Exclude**:
- ❌ Interfaces
- ❌ Libraries in lib/ folders
- ❌ Standard libraries (OpenZeppelin, forge-std, etc.)
- ❌ Mocks and test contracts

### Function 2: `generate_contracts_via_import_traversal()`

**Signature**:
```rust
async fn generate_contracts_via_import_traversal(
    main_contract: &str,
    repo: &RepoPaths,
    max_depth: usize,
    token_budget: usize,
) -> Result<(HashSet<String>, HashSet<String>, usize)>
```

**Key Algorithm**:
```rust
// Initialize
let mut levels: Vec<HashSet<String>> = vec![HashSet::new(); max_depth + 1];
levels[0].insert(main_contract.to_string());
let mut visited_files = HashSet::new();

// For each level from 0 to max_depth-1:
for current_depth in 0..max_depth {
    for contract in &levels[current_depth].clone() {
        // Get file for contract
        // Skip if already visited
        // Add to visited_files
        // Call detect_source_code_dependencies()
        // Extract contracts from source_files using extract_contracts_from_file()
        // Add to levels[current_depth + 1]
    }
}

// Assemble results
let contracts: HashSet<String> = levels.iter().flatten().cloned().collect();
let contracts_with_depth: HashSet<String> = levels[1].union(&levels[2]).cloned().collect();

Ok((contracts, contracts_with_depth, 0))
```

**Return Values**:
- `contracts`: All contracts from level 0 to max_depth
- `contracts_with_depth`: Contracts at depth 1-2 only (matches BFS)
- `token_count`: 0 (placeholder, calculated later)

### Integration Point

**Location**: `src/enumerator/codeblocks.rs`, line 64-66

**Detection Logic**:
```rust
let contract_to_func_map = get_hashmap_of_contract_to_functions(repo, semantic_db).await?;

// Check if Slither succeeded
let slither_available = !contract_to_func_map.is_empty();

if slither_available {
    // Use existing BFS traversal (lines 66-140)
    for (main_contract, functions_of_contract) in contract_to_func_map {
        // ... existing BFS logic
    }
} else {
    // NEW: Import-only fallback
    warn!("Slither call graph unavailable. Falling back to import-only traversal.");
    
    // Get all in-scope contracts
    let all_contracts = get_all_in_scope_contracts(repo).await?;
    
    for main_contract in all_contracts {
        let (contracts, contracts_with_depth, _) = 
            generate_contracts_via_import_traversal(&main_contract, repo, max_depth, token_budget).await?;
        
        // Continue with existing logic at line 142 (parent contracts, assembly, etc.)
        // ... rest of codeblock generation
    }
}
```

**Warning Message**:
```rust
warn!(
    "Slither call graph unavailable for contract '{}'. Falling back to import-only traversal.",
    main_contract
);
```

## Key Differences from Original Plan

| Aspect | Original Plan | Updated Plan |
|--------|---------------|--------------|
| Library handling | Exclude ALL libraries | Include libraries in source folders |
| Max depth | Hardcoded 3 | Parameter `max_depth` (default 3) |
| Fallback detection | Check Slither errors | Check `contract_to_func_map.is_empty()` |
| Warning logging | Generic message | Specific message with contract name |

## Testing Strategy

1. **Unit test**: `extract_contracts_from_file()` with library in /src → should INCLUDE
2. **Unit test**: `extract_contracts_from_file()` with library in /lib → should EXCLUDE
3. **Unit test**: `generate_contracts_via_import_traversal()` with max_depth=1, 2, 3
4. **Integration test**: Run on Ekubo (Slither fails) → should use fallback
5. **Integration test**: Run on normal repo (Slither works) → should use BFS

## Files to Modify

1. **src/enumerator/codeblocks.rs** - Add two new functions (after line 803)
2. **src/enumerator/codeblocks.rs** - Modify `generate_codeblock_from_codebase()` (lines 64-66) - AFTER APPROVAL

## Success Criteria

- ✅ Includes non-standard libraries in source code folders
- ✅ Uses max_depth parameter (not hardcoded)
- ✅ Detects Slither failure via empty contract_to_func_map
- ✅ Logs warning when fallback is used
- ✅ Works on Ekubo repository where Slither fails
- ✅ Produces reasonable code blocks for AI analysis

