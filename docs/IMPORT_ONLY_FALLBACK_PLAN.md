# Import-Only Fallback Plan for Code Block Generation

## Problem Statement

When Slither fails (e.g., due to unsupported Solidity opcodes like `clz()` in 0.8.31), we need a fallback mechanism to generate code blocks without relying on call graph data.

## Current Architecture (Lines 85-140 in codeblocks.rs)

### BFS Traversal with Slither Call Graph
```rust
// 1. Initialize frontier with all functions of main contract
let mut frontier: VecDeque<(SmartContractFunction, usize)> = VecDeque::new();
for func in functions_of_contract {
    frontier.push_back((func, 0_usize))
}

// 2. BFS traversal through call graph
while let Some((func, depth)) = frontier.pop_front() {
    // Track visited functions
    // Track unique contracts (excluding standard libs/interfaces)
    // Track contracts at depth 1-2 in contracts_with_depth
    // Check token budget
    // Query semantic_db for callees: SELECT callee FROM edges WHERE caller = ?
    // Add callees to frontier if depth < max_depth
}

// Result: 
// - contracts: HashSet<String> - all unique contracts discovered
// - contracts_with_depth: HashSet<String> - contracts at depth 1-2
```

### Post-BFS Processing (Lines 142-250)
```rust
// 1. Add parent contracts (inheritance)
// 2. Detect import dependencies for main contract + contracts_with_depth
// 3. Assemble final code block with proper ordering
```

## Proposed Solution: Import-Only Traversal

### High-Level Algorithm

**Goal**: Replace BFS call graph traversal (lines 85-140) with import-based traversal when Slither fails.

**Input**: 
- `main_contract: String` - The contract to analyze
- `repo: &RepoPaths` - Repository paths
- `max_depth: usize` - Maximum import traversal depth (3 levels)
- `token_budget: usize` - Maximum tokens per code block

**Output**:
- `contracts: HashSet<String>` - All contracts discovered (excluding libs/interfaces)
- `contracts_with_depth: HashSet<String>` - Contracts at depth 1-2

### Detailed Algorithm

```
1. INITIALIZATION
   - level_0_contracts = {main_contract}
   - level_1_contracts = HashSet::new()
   - level_2_contracts = HashSet::new()
   - level_3_contracts = HashSet::new()
   - visited_files = HashSet::new()  // Track files to avoid cycles
   - token_count = 0

2. LEVEL 0: Main Contract
   - Get file for main_contract
   - Add to visited_files
   - Parse imports using detect_source_code_dependencies()
   - Extract contracts from ImportDependencies:
     * source_files → extract contract names → level_1_contracts
     * interface_implementations → level_1_contracts
   - Update token_count
   - Check token budget

3. LEVEL 1: Direct Imports
   For each contract in level_1_contracts:
     - Skip if standard lib/interface
     - Get file for contract
     - Skip if already in visited_files
     - Add to visited_files
     - Parse imports using detect_source_code_dependencies()
     - Extract contracts → level_2_contracts
     - Update token_count
     - Break if token budget exceeded

4. LEVEL 2: Transitive Imports
   For each contract in level_2_contracts:
     - Skip if standard lib/interface
     - Get file for contract
     - Skip if already in visited_files
     - Add to visited_files
     - Parse imports using detect_source_code_dependencies()
     - Extract contracts → level_3_contracts
     - Update token_count
     - Break if token budget exceeded

5. LEVEL 3: Deep Transitive Imports
   For each contract in level_3_contracts:
     - Skip if standard lib/interface
     - Get file for contract
     - Skip if already in visited_files
     - Add to visited_files
     - Update token_count
     - Break if token budget exceeded

6. ASSEMBLE RESULTS
   - contracts = level_0 ∪ level_1 ∪ level_2 ∪ level_3
   - contracts_with_depth = level_1 ∪ level_2  (matches BFS behavior)
   - Return (contracts, contracts_with_depth, token_count)
```

### Key Design Decisions

1. **Exclude Libraries and Interfaces (with Exception for Non-Standard Source Libraries)**
   - Use `is_standard_library_contract_name()` to skip OpenZeppelin, forge-std, etc.
   - Use `is_standard_interface_name()` to skip standard interfaces
   - **INCLUDE non-standard libraries in source code folders** (repo.source_code_folders, typically /src)
   - This matches current BFS behavior which includes source libraries
   - Only track concrete contracts + source libraries

2. **Extract Contracts from Files**
   - Use `SOLIDITY_REGEXES.contract_decl` from `utils.rs`
   - Regex: `r"(?m)^\s*(abstract\s+contract|contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)"`
   - Filter out interfaces
   - Filter out libraries UNLESS they are in source code folders (repo.source_code_folders)

3. **Token Budget Tracking**
   - Use existing `get_token_count_of_function_ir()` for consistency
   - Alternative: Estimate tokens from file size (faster but less accurate)
   - Break traversal early if budget exceeded

4. **Cycle Detection**
   - Track visited files (not contracts, since same contract can be in multiple files)
   - Use `HashSet<PathBuf>` for O(1) lookup

5. **Depth Mapping**
   - `contracts_with_depth` should contain depth 1-2 contracts (matches BFS behavior at line 105-107)
   - This ensures compatibility with existing code at lines 226-237

6. **Max Depth Parameter**
   - Use `max_depth` parameter (default 3) instead of hardcoded depth
   - Allows flexibility to adjust traversal depth at runtime
   - Matches BFS behavior which also uses `max_depth` parameter

## Implementation Plan

### Step 1: Create Helper Function to Extract Contracts from File
```rust
/// Extract all contract names from a Solidity file (excluding interfaces and libraries)
async fn extract_contracts_from_file(
    file: &PathBuf,
    repo: &RepoPaths,
) -> Result<Vec<String>>
```

### Step 2: Create Main Fallback Function
```rust
/// Generate contract sets using import-only traversal (fallback when Slither fails)
///
/// Returns: (contracts, contracts_with_depth, token_count)
async fn generate_contracts_via_import_traversal(
    main_contract: &str,
    repo: &RepoPaths,
    max_depth: usize,
    token_budget: usize,
) -> Result<(HashSet<String>, HashSet<String>, usize)>
```

### Step 3: Integration Point (NOT IMPLEMENTED YET - AWAITING APPROVAL)
```rust
// In generate_codeblock_from_codebase(), around line 85:

// Check if Slither call graph is available by checking if contract_to_func_map is empty
let slither_available = !contract_to_func_map.is_empty();

let (contracts, contracts_with_depth, token_count) =
    if slither_available {
        // Existing BFS traversal (lines 85-140)
        ...
    } else {
        // NEW: Import-only fallback
        warn!(
            "Slither call graph unavailable for contract '{}'. Falling back to import-only traversal.",
            main_contract
        );
        generate_contracts_via_import_traversal(&main_contract, repo, max_depth, token_budget).await?
    };

// Continue with existing logic at line 142 (parent contracts, etc.)
```

## Testing Strategy

1. **Unit Tests**
   - Test `extract_contracts_from_file()` with various Solidity files
   - Test cycle detection (A imports B, B imports A)
   - Test token budget enforcement

2. **Integration Test**
   - Run on Ekubo repository (where Slither fails)
   - Compare contract sets with manual analysis
   - Verify token counts are reasonable

## Files to Modify

1. **src/enumerator/codeblocks.rs** (NEW FUNCTION ONLY - no integration yet)
   - Add `extract_contracts_from_file()` helper
   - Add `generate_contracts_via_import_traversal()` fallback function
   - DO NOT modify `generate_codeblock_from_codebase()` yet

2. **src/enumerator/utils.rs** (if needed)
   - Expose any additional helper functions

## Success Criteria

✅ Fallback function generates contract sets without Slither
✅ Respects token budget limits
✅ Excludes standard libraries and interfaces
✅ Handles circular imports gracefully
✅ Matches depth semantics of BFS traversal (depth 1-2 in contracts_with_depth)
✅ Ready for integration after user approval

## Comparison: BFS vs Import-Only Traversal

### What BFS Captures (that Import-Only Might Miss)
- **Runtime dependencies**: Contracts called via function calls but not imported
- **Dynamic calls**: `address(contract).call()` patterns
- **Factory patterns**: Contracts created via `new ContractName()`
- **Delegate calls**: Contracts used via delegatecall

### What Import-Only Captures (that BFS Might Miss)
- **Type dependencies**: Contracts used only for type declarations
- **Constructor dependencies**: Contracts passed to constructors
- **Interface implementations**: Concrete contracts implementing imported interfaces
- **Transitive type dependencies**: Types imported by imported files

### Trade-offs

| Aspect | BFS (Slither) | Import-Only |
|--------|---------------|-------------|
| **Requires Slither** | ✅ Yes | ❌ No |
| **Compilation needed** | ✅ Yes | ❌ No |
| **Runtime dependencies** | ✅ Captures | ❌ Misses |
| **Type dependencies** | ⚠️ Partial | ✅ Complete |
| **Speed** | ⚠️ Slower (needs compilation) | ✅ Faster (parse only) |
| **Robustness** | ❌ Fails on unsupported opcodes | ✅ Always works |
| **Accuracy** | ✅ High for runtime | ✅ High for compile-time |

### Recommendation
Use **BFS as primary**, **Import-Only as fallback**:
1. Try Slither-based BFS first
2. If Slither fails, fall back to Import-Only
3. Log which method was used for debugging

This gives us the best of both worlds:
- Maximum accuracy when Slither works
- Graceful degradation when Slither fails
- Always produces a result (never fails completely)

