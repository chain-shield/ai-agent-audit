# Import-Only Fallback: Implementation Checklist

## Pre-Implementation Review

### Documentation Review
- [ ] Read `IMPORT_FALLBACK_SUMMARY.md` - Executive summary
- [ ] Read `IMPORT_ONLY_FALLBACK_PLAN.md` - High-level architecture
- [ ] Read `IMPORT_FALLBACK_IMPLEMENTATION.md` - Detailed specification
- [ ] Read `IMPORT_FALLBACK_EXAMPLE.md` - Concrete walkthrough
- [ ] Review Mermaid diagram - Visual algorithm representation

### Design Approval
- [ ] Algorithm approved (3-level import traversal)
- [ ] Exclusion rules approved (interfaces, libraries, standard libs)
- [ ] Depth semantics approved (contracts_with_depth = L1 ∪ L2)
- [ ] Cycle detection approach approved (visited_files HashSet)
- [ ] Token budget strategy approved (defer to assembly phase)

## Phase 1: Helper Function Implementation

### `extract_contracts_from_file()` Function
- [ ] Create function signature in `src/enumerator/codeblocks.rs`
- [ ] Add comprehensive documentation comment
- [ ] Implement file reading with `fs::read_to_string()`
- [ ] Use `SOLIDITY_REGEXES.contract_decl` for parsing
- [ ] Filter out interfaces (contract_type == Interface)
- [ ] Filter out libraries (contract_type == Library)
- [ ] Filter out standard libraries (`is_standard_library_contract_name()`)
- [ ] Filter out mocks (path contains "/mocks/" or "/test/")
- [ ] Filter out test contracts (name contains "mock" case-insensitive)
- [ ] Return `Vec<String>` of concrete contract names
- [ ] Add error handling for file read failures

### Unit Tests for `extract_contracts_from_file()`
- [ ] Test with file containing multiple contracts
- [ ] Test with file containing interfaces (should exclude)
- [ ] Test with file containing libraries (should exclude)
- [ ] Test with file containing abstract contracts (should include)
- [ ] Test with file containing mocks (should exclude)
- [ ] Test with OpenZeppelin file (should exclude)
- [ ] Test with empty file
- [ ] Test with file read error

## Phase 2: Main Fallback Function Implementation

### `generate_contracts_via_import_traversal()` Function
- [ ] Create function signature in `src/enumerator/codeblocks.rs`
- [ ] Add comprehensive documentation comment
- [ ] Initialize data structures (level_0, level_1, level_2, level_3, visited_files)
- [ ] Implement Level 0 processing (main contract)
  - [ ] Get file for main_contract
  - [ ] Add to visited_files
  - [ ] Call `detect_source_code_dependencies()`
  - [ ] Extract contracts from source_files
  - [ ] Add interface_implementations
  - [ ] Populate level_1_contracts
- [ ] Implement Level 1 processing (direct imports)
  - [ ] Loop through level_1_contracts
  - [ ] Get file for each contract
  - [ ] Check visited_files (skip if visited)
  - [ ] Add to visited_files
  - [ ] Call `detect_source_code_dependencies()`
  - [ ] Extract contracts from source_files
  - [ ] Add interface_implementations
  - [ ] Populate level_2_contracts
- [ ] Implement Level 2 processing (transitive imports)
  - [ ] Same logic as Level 1
  - [ ] Populate level_3_contracts
- [ ] Implement Level 3 processing (deep imports)
  - [ ] Track files only (no further recursion)
- [ ] Assemble results
  - [ ] contracts = L0 ∪ L1 ∪ L2 ∪ L3
  - [ ] contracts_with_depth = L1 ∪ L2
  - [ ] token_count = 0 (placeholder)
- [ ] Add error handling for file lookup failures
- [ ] Add logging for debugging

### Unit Tests for `generate_contracts_via_import_traversal()`
- [ ] Test with simple linear imports (A → B → C)
- [ ] Test with circular imports (A → B → A)
- [ ] Test with diamond pattern (A → B,C; B,C → D)
- [ ] Test with standard library exclusion
- [ ] Test with interface exclusion
- [ ] Test with library exclusion
- [ ] Test depth tracking (verify contracts_with_depth)
- [ ] Test with contract not found (should continue gracefully)
- [ ] Test with max_depth = 1, 2, 3

## Phase 3: Integration Testing (Before Wiring Up)

### Standalone Testing
- [ ] Test on Ekubo repository
  - [ ] Run `generate_contracts_via_import_traversal("Core", &repo, 3, 150_000)`
  - [ ] Verify contracts set is reasonable
  - [ ] Verify contracts_with_depth is reasonable
  - [ ] Compare with manual analysis of imports
  - [ ] Check for any missing critical contracts
  - [ ] Check for any incorrectly included contracts
- [ ] Test on other repositories where Slither works
  - [ ] Compare BFS results vs Import-Only results
  - [ ] Identify differences
  - [ ] Verify differences are expected (runtime vs compile-time deps)

### Performance Testing
- [ ] Measure execution time on Ekubo
- [ ] Measure execution time on large repository
- [ ] Compare with BFS execution time
- [ ] Verify no memory leaks
- [ ] Verify no infinite loops

## Phase 4: Integration (AWAITING APPROVAL)

### Detect Slither Availability
- [ ] Add function to check if Slither call graph succeeded
- [ ] Check if semantic_db has edges table populated
- [ ] Add logging for which method is being used

### Wire Up Fallback
- [ ] Modify `generate_codeblock_from_codebase()` at line 85
- [ ] Add conditional: if slither_available { BFS } else { Import-Only }
- [ ] Add warning log when using fallback
- [ ] Ensure seamless transition to line 142 (existing logic)
- [ ] Test end-to-end on Ekubo
- [ ] Test end-to-end on repository where Slither works (verify BFS still used)

### Integration Tests
- [ ] Test Ekubo end-to-end (Slither fails → fallback used)
- [ ] Test normal repository (Slither works → BFS used)
- [ ] Verify code blocks are generated correctly in both cases
- [ ] Verify audit reports are generated correctly in both cases

## Phase 5: Documentation and Cleanup

### Code Documentation
- [ ] Add inline comments explaining algorithm
- [ ] Document why we exclude interfaces/libraries
- [ ] Document depth semantics
- [ ] Document cycle detection

### User Documentation
- [ ] Update README with fallback mechanism
- [ ] Add troubleshooting section for Slither failures
- [ ] Document when fallback is used vs BFS

### Cleanup
- [ ] Remove any debug logging
- [ ] Remove any commented-out code
- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy` and fix warnings
- [ ] Run all tests and verify they pass

## Success Criteria

### Functional Requirements
- [x] Fallback generates contract sets without Slither
- [x] Handles circular imports gracefully
- [x] Excludes standard libraries and interfaces
- [x] Matches depth semantics of BFS (contracts_with_depth = depth 1-2)
- [ ] Works on Ekubo repository where Slither fails
- [ ] Produces reasonable code blocks for AI analysis

### Non-Functional Requirements
- [ ] Execution time < 10 seconds for typical repository
- [ ] No memory leaks
- [ ] No infinite loops
- [ ] Graceful error handling (no panics)
- [ ] Clear logging for debugging

### Code Quality
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] No clippy warnings
- [ ] Code is well-documented
- [ ] Follows existing code style

## Rollback Plan

If fallback doesn't work as expected:
1. Keep functions but don't wire up integration
2. Continue using BFS-only approach
3. Manually handle repositories where Slither fails
4. Revisit design based on learnings

## Notes

- **DO NOT** modify `generate_codeblock_from_codebase()` until user approval
- **DO** implement both functions as standalone for review
- **DO** add comprehensive tests before integration
- **DO** test on Ekubo before integration
- **DO** log which method is used (BFS vs Import-Only) for debugging

