# Import-Only Fallback: Executive Summary

## Problem
Slither fails on Ekubo repository due to unsupported `clz()` opcode in Solidity 0.8.31, preventing code block generation.

## Solution
Implement import-only traversal as a fallback mechanism when Slither is unavailable.

## Approach

### Current: BFS with Slither Call Graph (Lines 85-140)
```rust
// Query semantic_db for function call edges
SELECT callee FROM edges WHERE caller = ?

// Traverse call graph breadth-first up to max_depth
// Discover contracts via runtime function calls
```

### Fallback: Import-Based Traversal (NEW)
```rust
// Parse import statements from Solidity source files
// Traverse import dependencies up to max_depth (3 levels)
// Discover contracts via compile-time imports
```

## Key Design Principles

1. **Drop-in Replacement**: Returns same data structure as BFS
   - `contracts: HashSet<String>`
   - `contracts_with_depth: HashSet<String>`
   - `token_count: usize`

2. **Depth Semantics Match**:
   - Level 0 = main contract (depth 0)
   - Levels 1 to max_depth = imports at each depth
   - `contracts_with_depth` = Level 1 ∪ Level 2 (matches BFS behavior)
   - Uses `max_depth` parameter (default 3, configurable)

3. **Exclusions Match**:
   - Standard libraries (OpenZeppelin, forge-std)
   - Standard interfaces (IERC20, IERC721, etc.)
   - Interfaces (only concrete contracts + source libraries)
   - Libraries in lib/ folders (but INCLUDE libraries in /src - matches BFS)
   - Mocks and test contracts

4. **Cycle Detection**: Track visited files to prevent infinite loops

5. **No Integration Yet**: Implement as separate functions, await approval before wiring up

## Implementation Plan

### Phase 1: Helper Function (READY FOR REVIEW)
```rust
/// Extract all contract names from a Solidity file
async fn extract_contracts_from_file(
    file: &PathBuf,
    repo: &RepoPaths,
) -> Result<Vec<String>>
```

**Location**: `src/enumerator/codeblocks.rs` (after line 803)

**Logic**:
- Read file with `fs::read_to_string()`
- Use `SOLIDITY_REGEXES.contract_decl` to find declarations
- Check if file is in source code folders (repo.source_code_folders)
- Filter out interfaces, standard libs, mocks
- Filter out libraries UNLESS they are in source code folders
- Return Vec of contract names (concrete contracts + source libraries)

### Phase 2: Main Fallback Function (READY FOR REVIEW)
```rust
/// Generate contract sets using import-only traversal
async fn generate_contracts_via_import_traversal(
    main_contract: &str,
    repo: &RepoPaths,
    max_depth: usize,
    token_budget: usize,
) -> Result<(HashSet<String>, HashSet<String>, usize)>
```

**Location**: `src/enumerator/codeblocks.rs` (after line 803)

**Algorithm**:
1. Level 0: Start with main_contract
2. For each level from 1 to max_depth:
   - Analyze imports of contracts from previous level
   - Discover contracts at current level
3. Assemble: contracts = L0 ∪ L1 ∪ ... ∪ L_max_depth
4. Assemble: contracts_with_depth = L1 ∪ L2
5. Return (contracts, contracts_with_depth, token_count)

### Phase 3: Integration (AWAITING APPROVAL)
```rust
// In generate_codeblock_from_codebase(), replace lines 85-140:

// Check if Slither call graph is available by checking if contract_to_func_map is empty
let slither_available = !contract_to_func_map.is_empty();

let (contracts, contracts_with_depth, token_count) =
    if slither_available {
        // Existing BFS traversal
        ...
    } else {
        // NEW: Import-only fallback
        warn!(
            "Slither call graph unavailable for contract '{}'. Falling back to import-only traversal.",
            main_contract
        );
        generate_contracts_via_import_traversal(&main_contract, repo, max_depth, token_budget).await?
    };

// Continue with existing logic at line 142 (unchanged)
```

## Documentation Provided

1. **IMPORT_ONLY_FALLBACK_PLAN.md** - High-level architecture and algorithm
2. **IMPORT_FALLBACK_IMPLEMENTATION.md** - Detailed implementation specification
3. **IMPORT_FALLBACK_EXAMPLE.md** - Concrete walkthrough with example repository
4. **IMPORT_FALLBACK_SUMMARY.md** - This executive summary
5. **Mermaid Diagram** - Visual representation of traversal algorithm

## Next Steps

### For User Review
1. Review the algorithm in `IMPORT_ONLY_FALLBACK_PLAN.md`
2. Review the implementation spec in `IMPORT_FALLBACK_IMPLEMENTATION.md`
3. Review the concrete example in `IMPORT_FALLBACK_EXAMPLE.md`
4. Provide feedback or approval

### After Approval
1. Implement `extract_contracts_from_file()` function
2. Implement `generate_contracts_via_import_traversal()` function
3. Add unit tests for both functions
4. Test on Ekubo repository (real-world validation)
5. Wire up integration in `generate_codeblock_from_codebase()`
6. Test end-to-end with Ekubo

## Success Metrics

- ✅ Fallback generates contract sets without Slither
- ✅ Handles circular imports gracefully
- ✅ Excludes standard libraries and interfaces
- ✅ Matches depth semantics of BFS (contracts_with_depth = depth 1-2)
- ✅ Works on Ekubo repository where Slither fails
- ✅ Produces reasonable code blocks for AI analysis

## Risk Mitigation

**Risk**: Import-only might miss runtime dependencies (factory patterns, dynamic calls)

**Mitigation**: 
- Use as fallback only (BFS is still primary when Slither works)
- Log which method was used for debugging
- Import-only still captures most dependencies (types, constructors, interfaces)
- Better to have partial context than no context at all

**Risk**: Token budget might be exceeded

**Mitigation**:
- Initial implementation: collect all contracts, let assembly phase handle budget
- Future: add early termination if needed
- Existing code already has budget enforcement at assembly time

**Risk**: Circular imports cause infinite loops

**Mitigation**:
- Track visited files in HashSet
- Skip files already processed
- Tested with circular import example (A → B → A)

## User Feedback Incorporated ✅

1. **Include non-standard libraries in source code folders** ✅
   - Libraries in repo.source_code_folders (typically /src) are INCLUDED
   - This matches current BFS behavior which only excludes standard libraries
   - Filter logic: Skip libraries UNLESS they are in source code folders

2. **Use max_depth parameter instead of hardcoded 3** ✅
   - Function signature uses `max_depth: usize` parameter
   - Allows runtime configuration (default 3, but adjustable)
   - Matches BFS behavior which also uses max_depth parameter

3. **Detect Slither failure via empty contract_to_func_map** ✅
   - Check `contract_to_func_map.is_empty()` to determine if Slither succeeded
   - If empty, fall back to import-only traversal
   - Log warning message when fallback is used

## Ready to Proceed

All user feedback has been incorporated into the plan. Ready to implement:
- ✅ `extract_contracts_from_file()` - with source library inclusion logic
- ✅ `generate_contracts_via_import_traversal()` - with max_depth parameter
- ✅ Integration detection - via contract_to_func_map.is_empty()
- ✅ Warning logging - when fallback is triggered

