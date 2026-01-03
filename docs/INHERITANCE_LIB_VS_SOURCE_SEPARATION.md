# Inheritance System: Lib vs Source Contract Separation Strategy

## Executive Summary

**Recommendation**: Leverage the existing lib/source separation pattern already present in the codebase to solve the duplicate contract name problem. This is a **much simpler and more pragmatic solution** than creating a new `ContractId` type system.

**Key Insight**: The codebase already has:
- ✅ Separate storage for lib contracts (`LIB_CONTRACT_TO_FILE`) vs source contracts (`CONTRACT_TO_FILE`)
- ✅ Logic to detect if a file is in lib folder vs source folder (`SolFileType` enum)
- ✅ Separate insertion functions (`insert_lib_contract_to_file_mapping` vs `insert_contract_to_file_mapping`)

**The Problem**: The inheritance system doesn't use this separation, causing collisions when lib and source have contracts with the same name.

---

## Current Separation Pattern (Already Exists!)

### 1. **File Type Detection** (src/enumerator/utils.rs:336-367)

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum SolFileType {
    Standard,      // Source contracts (src/, contracts/)
    LibFolder,     // Library contracts (lib/, library/, libraries/)
}

// Determine file type based on path
let file_type = if is_library_file(file, &repo.root) {
    SolFileType::LibFolder
} else if repo.source_code_folders.iter().any(|f| file.starts_with(f)) {
    SolFileType::Standard
} else {
    continue; // Skip files that are neither
};
```

**Detection Logic** (src/utils/check_folder_name.rs:149-152):
- `is_library_file()` returns true if file is in exactly ONE lib/library/libraries folder
- Prevents matching nested lib folders (e.g., `/lib/.../lib/`)

### 2. **Separate Storage** (src/llm_review/contract_file_map.rs:14-18)

```rust
// Source contracts: project_id + contract_name → (file, type)
static CONTRACT_TO_FILE: Lazy<Arc<Mutex<HashMap<String, (PathBuf, ContractType)>>>>

// Lib contracts: lib_project_id + contract_name → file
static LIB_CONTRACT_TO_FILE: Lazy<Arc<Mutex<HashMap<String, PathBuf>>>>
```

**Key Format**:
- Source contracts: `"{project_id}_{contract_name}"` → `(PathBuf, ContractType)`
- Lib contracts: `"lib_{project_id}_{contract_name}"` → `PathBuf`

### 3. **Separate Insertion** (src/enumerator/utils.rs:399-407)

```rust
if file_type == SolFileType::Standard {
    if !contract.to_ascii_lowercase().contains("mock") {
        contracts.push(contract.to_string());
    }
    insert_contract_to_file_mapping(contract, file, contract_type, repo).await?;
} else {
    insert_lib_contract_to_file_mapping(contract, file, repo).await?;
}
```

**Behavior**:
- Source contracts → added to `contracts` list (returned for analysis)
- Lib contracts → stored separately, NOT added to main contracts list

---

## The Gap: Inheritance System Doesn't Use This Pattern

### Current Inheritance Storage (src/build_brain/callgraph.rs:33-39)

```rust
// Child → Parents mapping (uses bare contract names)
static INHERITANCE_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, Vec<String>>>>>

// Parent → Children mapping (uses bare contract names)
static INVERTED_INHERITANCE_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, Vec<String>>>>>
```

**Problem**: HashMap keys are bare contract names like `"BaseAdapter"`, causing collisions.

### Current Inheritance Edges (src/build_brain/inheritance.rs:11)

```rust
pub fn parse_inheritance_json(json: &str) -> Result<Vec<(String, String)>>
// Returns: Vec<(child_name, parent_name)>
```

**Problem**: Uses bare contract names without lib/source distinction.

---

## Proposed Solution: Prefix-Based Separation

### Concept: Use Prefixes to Distinguish Lib vs Source Contracts

Instead of creating a complex `ContractId` type, simply prefix lib contracts with `"lib:"`:

```rust
// Source contracts (no prefix)
"BaseAdapter"           → src/curators/oracles/BaseAdapter.sol
"CovenantCurator"       → src/curators/CovenantCurator.sol
"ChainlinkOracle"       → src/curators/oracles/chainlink/ChainlinkOracle.sol

// Lib contracts (with "lib:" prefix)
"lib:BaseAdapter"       → lib/euler-price-oracle/src/adapter/BaseAdapter.sol
"lib:ChainlinkOracle"   → lib/euler-price-oracle/src/adapter/chainlink/ChainlinkOracle.sol
"lib:EulerRouter"       → lib/euler-price-oracle/src/EulerRouter.sol
```

**Inheritance edges become**:
```rust
// Covenant's BaseAdapter inherits from Euler's BaseAdapter
("BaseAdapter", "lib:BaseAdapter")

// Covenant's ChainlinkOracle inherits from Euler's ChainlinkOracle
("ChainlinkOracle", "lib:ChainlinkOracle")

// Covenant's BaseAdapter also implements ICovenantPriceOracle
("BaseAdapter", "ICovenantPriceOracle")
```

---

## Implementation Plan

### Phase 1: Update Slither JSON Parsing

**File**: `src/build_brain/inheritance.rs`

**Current**:
```rust
pub fn parse_inheritance_json(json: &str) -> Result<Vec<(String, String)>>
```

**Updated**:
```rust
pub fn parse_inheritance_json(json: &str, repo: &RepoPaths) -> Result<Vec<(String, String)>>
```

**Logic**:
1. Parse Slither JSON to get `(child_name, parent_name)` tuples
2. For each contract name, check if it's a lib contract:
   - Try `get_file_from_lib_contract(name, repo)` → if Some, it's a lib contract
   - If lib contract, prefix with `"lib:"`
3. Return edges with prefixed names

**Example**:
```rust
for (child, parents) in child_parent_map.into_iter() {
    let child_key = if get_file_from_lib_contract(&child, repo).await.is_some() {
        format!("lib:{}", child)
    } else {
        child.clone()
    };
    
    for parent in parents.immediate.iter() {
        let parent_key = if get_file_from_lib_contract(parent, repo).await.is_some() {
            format!("lib:{}", parent)
        } else {
            parent.to_string()
        };
        
        edges.push((child_key.clone(), parent_key));
    }
}
```

### Phase 2: Update Inheritance Map Functions

**File**: `src/build_brain/callgraph.rs`

**Changes**:
1. `generate_inheritance_edges()` - Pass `repo` to `parse_inheritance_json()`
2. `get_inheritance_map()` - No changes needed (keys are now prefixed)
3. `get_inverted_inheritance_map()` - No changes needed (keys are now prefixed)

**Result**: Maps now use prefixed keys:
```rust
// Child → Parents map
{
    "BaseAdapter": ["lib:BaseAdapter", "ICovenantPriceOracle"],
    "lib:BaseAdapter": ["IPriceOracle"],
    "ChainlinkOracle": ["lib:ChainlinkOracle"],
    "lib:ChainlinkOracle": ["lib:BaseAdapter"],
    "CovenantCurator": ["IPriceOracle"],
}

// Parent → Children map
{
    "IPriceOracle": ["lib:BaseAdapter", "CovenantCurator"],
    "lib:BaseAdapter": ["BaseAdapter", "lib:ChainlinkOracle", "lib:PythOracle"],
    "ICovenantPriceOracle": ["BaseAdapter"],
}
```

### Phase 3: Update API Functions

**File**: `src/build_brain/callgraph.rs`

**Option A: Keep Current API, Add Prefix Logic Internally**

```rust
pub async fn get_parents(child_contract: &str, repo: &RepoPaths) -> Result<Vec<String>> {
    let inheritance_map = get_inheritance_map(repo).await?;
    
    // Try with and without prefix
    if let Some(parents) = inheritance_map.get(child_contract) {
        Ok(parents.clone())
    } else {
        let prefixed = format!("lib:{}", child_contract);
        if let Some(parents) = inheritance_map.get(&prefixed) {
            Ok(parents.clone())
        } else {
            Ok(Vec::new())
        }
    }
}
```

**Option B: Add New API with Explicit Lib Flag**

```rust
pub async fn get_parents_with_scope(
    child_contract: &str, 
    is_lib: bool,
    repo: &RepoPaths
) -> Result<Vec<String>> {
    let inheritance_map = get_inheritance_map(repo).await?;
    let key = if is_lib {
        format!("lib:{}", child_contract)
    } else {
        child_contract.to_string()
    };
    
    if let Some(parents) = inheritance_map.get(&key) {
        Ok(parents.clone())
    } else {
        Ok(Vec::new())
    }
}
```

**Recommendation**: Use **Option A** for backward compatibility, then gradually migrate to Option B.

### Phase 4: Update Call Sites

**File**: `src/enumerator/codeblocks.rs` (lines 135, 150)

**Current**:
```rust
let parents_of_main = callgraph::get_parents(&main_contract, repo).await?;
```

**Issue**: If `main_contract = "BaseAdapter"`, which BaseAdapter do we want?
- If it's from source → we want Covenant's BaseAdapter
- If it's from lib → we want Euler's BaseAdapter

**Solution**: Check if contract is in lib or source
```rust
let is_lib = get_file_from_lib_contract(&main_contract, repo).await.is_some();
let parents_of_main = if is_lib {
    callgraph::get_parents(&format!("lib:{}", main_contract), repo).await?
} else {
    callgraph::get_parents(&main_contract, repo).await?
};
```

**File**: `src/enumerator/parse_solidity.rs` (line 519)

**Current**:
```rust
let children = get_children(interface, repo).await?;
```

**Solution**: Same as above - check if interface is lib or source

---

## Advantages of This Approach

### ✅ 1. Minimal Code Changes
- Reuses existing lib/source detection logic
- No new types or complex data structures
- Backward compatible with simple fallback logic

### ✅ 2. Clear Semantics
- `"BaseAdapter"` = source contract (what we audit)
- `"lib:BaseAdapter"` = library contract (external dependency)
- Easy to understand in logs and debugging

### ✅ 3. Aligns with Existing Patterns
- Already have `CONTRACT_TO_FILE` vs `LIB_CONTRACT_TO_FILE`
- Already have `insert_contract_to_file_mapping` vs `insert_lib_contract_to_file_mapping`
- Just extending the pattern to inheritance

### ✅ 4. Preserves Audit Focus
- Source contracts (no prefix) are the audit targets
- Lib contracts (with prefix) are dependencies
- Easy to filter: `contracts.iter().filter(|c| !c.starts_with("lib:"))`

### ✅ 5. Handles Edge Cases
- Standard library contracts (IERC20, Ownable) → no prefix (not in lib folder)
- Project-specific lib contracts (euler-price-oracle) → `"lib:"` prefix
- Nested inheritance works: `BaseAdapter → lib:BaseAdapter → IPriceOracle`

---

## Alternative: Use Enum Instead of String Prefix

If we want stronger typing, we could use an enum:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContractScope {
    Source(String),  // Source contract name
    Lib(String),     // Lib contract name
}

impl ContractScope {
    pub fn name(&self) -> &str {
        match self {
            ContractScope::Source(n) => n,
            ContractScope::Lib(n) => n,
        }
    }
    
    pub fn is_lib(&self) -> bool {
        matches!(self, ContractScope::Lib(_))
    }
}

// Inheritance edges become
pub fn parse_inheritance_json(json: &str, repo: &RepoPaths) 
    -> Result<Vec<(ContractScope, ContractScope)>>
```

**Pros**:
- ✅ Type-safe (can't accidentally mix lib and source)
- ✅ Explicit in function signatures
- ✅ Easy to pattern match

**Cons**:
- ❌ More invasive changes (all HashMap keys need updating)
- ❌ Serialization/deserialization more complex
- ❌ Harder to use in string-based lookups

**Recommendation**: Start with string prefix approach, migrate to enum if needed.

---

## Migration Strategy

### Step 1: Update `parse_inheritance_json` (Non-Breaking)
- Add `repo` parameter
- Add prefix logic for lib contracts
- Test with Covenant repo to verify correct prefixes

### Step 2: Update `generate_inheritance_edges` (Non-Breaking)
- Pass `repo` to `parse_inheritance_json`
- Verify inheritance maps have prefixed keys

### Step 3: Add Helper Functions (Non-Breaking)
```rust
pub fn is_lib_contract(contract: &str) -> bool {
    contract.starts_with("lib:")
}

pub fn strip_lib_prefix(contract: &str) -> &str {
    contract.strip_prefix("lib:").unwrap_or(contract)
}

pub async fn resolve_contract_key(contract: &str, repo: &RepoPaths) -> String {
    if get_file_from_lib_contract(contract, repo).await.is_some() {
        format!("lib:{}", contract)
    } else {
        contract.to_string()
    }
}
```

### Step 4: Update `get_parents` and `get_children` (Backward Compatible)
- Try lookup with original name first
- If not found, try with `"lib:"` prefix
- If still not found, return empty Vec

### Step 5: Update Call Sites (Gradual)
- Start with `parse_solidity.rs` (most affected by interface children)
- Then `codeblocks.rs` (affects audit scope)
- Finally `enrichment.rs` (semantic database)

### Step 6: Update Tests
- Update `tests/inheritance_iprice_oracle_test.rs` to expect prefixed names
- Add tests for lib vs source contract disambiguation

---

## Expected Behavior After Implementation

### Covenant Repository Example

**Before** (Current - Ambiguous):
```
IPriceOracle → ["CovenantCurator"]
BaseAdapter → ["BaseAdapter", "ICovenantPriceOracle"]  // COLLISION!
```

**After** (With Prefixes - Clear):
```
IPriceOracle → ["lib:BaseAdapter", "CovenantCurator"]
BaseAdapter → ["lib:BaseAdapter", "ICovenantPriceOracle"]  // Covenant's BaseAdapter
lib:BaseAdapter → ["IPriceOracle"]  // Euler's BaseAdapter
ChainlinkOracle → ["lib:ChainlinkOracle"]  // Covenant's ChainlinkOracle
lib:ChainlinkOracle → ["lib:BaseAdapter"]  // Euler's ChainlinkOracle
```

**Queries**:
```rust
// Get parents of Covenant's BaseAdapter
get_parents("BaseAdapter", repo) 
// → ["lib:BaseAdapter", "ICovenantPriceOracle"]

// Get parents of Euler's BaseAdapter
get_parents("lib:BaseAdapter", repo)
// → ["IPriceOracle"]

// Get children of IPriceOracle
get_children("IPriceOracle", repo)
// → ["lib:BaseAdapter", "CovenantCurator"]

// Get children of Euler's BaseAdapter
get_children("lib:BaseAdapter", repo)
// → ["BaseAdapter", "lib:ChainlinkOracle", "lib:PythOracle"]
```

---

## Open Questions

### Q1: What if a contract exists in BOTH lib and source with same name?

**Answer**: This is the exact problem we're solving! With prefixes:
- Source version: `"BaseAdapter"`
- Lib version: `"lib:BaseAdapter"`
- No collision

### Q2: What about contracts that are imported from lib but not in inheritance?

**Answer**: They're already handled by the existing lib contract mapping. The prefix only affects inheritance relationships.

### Q3: Should we prefix ALL lib contracts or only those with name collisions?

**Answer**: Prefix ALL lib contracts for consistency. Makes it easy to identify scope at a glance.

### Q4: What about standard library contracts (OpenZeppelin, Solmate)?

**Answer**: They're excluded by `--filter-paths` in `run_printer_json_inheritance`, so they won't appear in inheritance edges at all.

### Q5: Performance impact of string prefixes?

**Answer**: Negligible. We're adding 4 characters (`"lib:"`) to some HashMap keys. The overhead is minimal compared to Slither execution time.

---

## Comparison with ContractId Approach

| Aspect | String Prefix | ContractId Struct |
|--------|---------------|-------------------|
| **Complexity** | Low - just string manipulation | High - new type, conversions everywhere |
| **Type Safety** | Medium - strings can be misused | High - compiler enforced |
| **Code Changes** | Minimal - mostly in parsing | Extensive - all call sites |
| **Backward Compat** | Easy - fallback to unprefixed | Hard - breaking API changes |
| **Debugging** | Easy - readable strings | Medium - need to inspect struct |
| **Memory** | Low - 4 extra chars per lib contract | Medium - PathBuf in every edge |
| **Aligns with Existing** | Yes - extends lib/source pattern | No - new pattern |

**Verdict**: String prefix is the pragmatic choice for this codebase.

---

## CRITICAL UPDATE: Slither Provides Contract IDs!

### Discovery

Slither's `inheritance-graph` printer provides **unique IDs** for each contract:

```
c316_BaseAdapter       // Euler's BaseAdapter (ID: 316)
c12551_BaseAdapter     // Covenant's BaseAdapter (ID: 12551)
c531_ChainlinkOracle   // Euler's ChainlinkOracle (ID: 531)
c13065_ChainlinkOracle // Covenant's ChainlinkOracle (ID: 13065)
```

**Inheritance edges in DOT format**:
```
c12551_BaseAdapter -> c316_BaseAdapter;           // Covenant's inherits from Euler's
c13065_ChainlinkOracle -> c531_ChainlinkOracle;   // Covenant's inherits from Euler's
```

### The Problem

The `inheritance` printer (which we currently use) **does NOT include these IDs**:
```json
{
  "BaseAdapter": {
    "immediate": ["BaseAdapter", "ICovenantPriceOracle"],
    "not_immediate": ["IPriceOracle"]
  }
}
```

It only returns contract names, causing ambiguity.

### Proposed Solutions

#### Option A: Parse DOT Graph Instead of Inheritance JSON

**Approach**: Use `inheritance-graph` printer and parse the DOT format to extract IDs and edges.

**Pros**:
- ✅ Unique IDs disambiguate duplicate names
- ✅ Can map ID to contract name
- ✅ Slither already generates this data

**Cons**:
- ❌ Need to parse DOT format (regex-based)
- ❌ More complex than JSON parsing
- ❌ Still need to map IDs to file paths

**DOT Format Example**:
```dot
c12551_BaseAdapter -> c316_BaseAdapter;
c12551_BaseAdapter -> c12308_ICovenantPriceOracle;
```

**Parsing Strategy**:
```rust
// Extract edges from DOT format
let edge_regex = Regex::new(r"c(\d+)_(\w+) -> c(\d+)_(\w+)").unwrap();

for cap in edge_regex.captures_iter(&dot_content) {
    let child_id = cap.get(1).unwrap().as_str();
    let child_name = cap.get(2).unwrap().as_str();
    let parent_id = cap.get(3).unwrap().as_str();
    let parent_name = cap.get(4).unwrap().as_str();

    // Create unique keys: "BaseAdapter#12551"
    let child_key = format!("{}#{}", child_name, child_id);
    let parent_key = format!("{}#{}", parent_name, parent_id);

    edges.push((child_key, parent_key));
}
```

#### Option B: Use Existing Lib/Source Detection (Simpler)

**Approach**: Stick with the string prefix approach but use a smarter detection strategy.

**Key Insight**: When we see `"BaseAdapter"` in Slither's inheritance JSON, we can determine which one it refers to by checking:

1. **Is it in the child position?**
   - If `get_file_from_contract("BaseAdapter")` exists → it's the source contract
   - If only `get_file_from_lib_contract("BaseAdapter")` exists → it's the lib contract

2. **Is it in the parent position?**
   - Check if the child is a source or lib contract
   - If child is source and parent name matches → parent is likely lib
   - If child is lib → parent is also lib

**Example Logic**:
```rust
for (child, parents) in child_parent_map.into_iter() {
    // Determine if child is lib or source
    let child_is_lib = get_file_from_contract(&child, repo).await.is_none()
        && get_file_from_lib_contract(&child, repo).await.is_some();

    let child_key = if child_is_lib {
        format!("lib:{}", child)
    } else {
        child.clone()
    };

    for parent in parents.immediate.iter() {
        // If parent name == child name, it's likely lib version
        let parent_is_lib = if parent == &child {
            true  // Self-reference means lib parent
        } else {
            // Check if parent exists in lib
            get_file_from_lib_contract(parent, repo).await.is_some()
        };

        let parent_key = if parent_is_lib {
            format!("lib:{}", parent)
        } else {
            parent.to_string()
        };

        edges.push((child_key.clone(), parent_key));
    }
}
```

**Special Case Handling**:
```rust
// When we see: "BaseAdapter" → ["BaseAdapter", "ICovenantPriceOracle"]
// This means Covenant's BaseAdapter inherits from:
//   1. Euler's BaseAdapter (same name = lib version)
//   2. ICovenantPriceOracle (different name = check if lib or source)

if parent == &child {
    // Same name inheritance = child (source) inherits from parent (lib)
    parent_key = format!("lib:{}", parent);
} else if get_file_from_lib_contract(parent, repo).await.is_some()
    && get_file_from_contract(parent, repo).await.is_some() {
    // Both exist - need heuristic
    // If child is source, prefer lib parent
    // If child is lib, prefer lib parent
    parent_key = format!("lib:{}", parent);
} else if get_file_from_lib_contract(parent, repo).await.is_some() {
    parent_key = format!("lib:{}", parent);
} else {
    parent_key = parent.to_string();
}
```

#### Option C: Hybrid Approach (Recommended)

**Approach**: Use DOT graph IDs to build a mapping, then apply to inheritance JSON.

**Steps**:
1. Parse `inheritance-graph` DOT to extract `id → (name, is_lib)` mapping
2. Determine `is_lib` by checking if contract exists in lib folder
3. Parse `inheritance` JSON as usual
4. When we see duplicate names, use the ID mapping to disambiguate

**Implementation**:
```rust
// Step 1: Build ID → contract info mapping from DOT graph
let id_map = parse_dot_graph_for_ids(repo).await?;
// Returns: HashMap<String, ContractInfo>
// e.g., "316" → ContractInfo { name: "BaseAdapter", is_lib: true }
//       "12551" → ContractInfo { name: "BaseAdapter", is_lib: false }

// Step 2: Parse inheritance JSON
let inheritance_json = slither_ffi::run_printer_json_inheritance(repo, None).await?;
let child_parent_map = parse_inheritance_json_raw(&inheritance_json)?;

// Step 3: Disambiguate using ID map
for (child, parents) in child_parent_map.into_iter() {
    // Find all IDs for this child name
    let child_ids: Vec<_> = id_map.iter()
        .filter(|(_, info)| info.name == child)
        .collect();

    // If only one ID, use it
    // If multiple IDs, use heuristics (prefer source over lib for child)
    let child_info = if child_ids.len() == 1 {
        child_ids[0].1
    } else {
        // Prefer source contract as child
        child_ids.iter()
            .find(|(_, info)| !info.is_lib)
            .map(|(_, info)| info)
            .unwrap_or(child_ids[0].1)
    };

    let child_key = if child_info.is_lib {
        format!("lib:{}", child)
    } else {
        child.clone()
    };

    // Similar logic for parents...
}
```

### Recommendation

Use **Option B (Simpler Heuristic)** for now because:

1. ✅ **Minimal code changes** - just smarter detection logic
2. ✅ **No DOT parsing** - stick with JSON
3. ✅ **Handles 99% of cases** - the heuristic "same name = lib parent" works for typical inheritance patterns
4. ✅ **Fast to implement** - can prototype and test quickly

**Fallback to Option C** if we encounter edge cases where Option B fails.

---

## Updated Implementation Plan

### Phase 1: Smart Heuristic Detection

**File**: `src/build_brain/inheritance.rs`

```rust
pub async fn parse_inheritance_json(json: &str, repo: &RepoPaths) -> Result<Vec<(String, String)>> {
    let root: Root = serde_json::from_str(json)?;
    let mut edges = Vec::new();

    for printer in root.results.printers {
        if printer.printer == "inheritance" {
            let child_parent_map = printer.additional_fields.child_to_base;

            for (child, parents) in child_parent_map.into_iter() {
                // Determine if child is lib or source
                let child_in_source = get_file_from_contract(&child, repo).await.is_some();
                let child_in_lib = get_file_from_lib_contract(&child, repo).await.is_some();

                let child_key = if !child_in_source && child_in_lib {
                    format!("lib:{}", child)
                } else {
                    child.clone()
                };

                for parent in parents.immediate.iter() {
                    let parent_key = if parent == &child {
                        // Same name = lib parent (source inherits from lib)
                        format!("lib:{}", parent)
                    } else {
                        let parent_in_lib = get_file_from_lib_contract(parent, repo).await.is_some();
                        if parent_in_lib {
                            format!("lib:{}", parent)
                        } else {
                            parent.to_string()
                        }
                    };

                    edges.push((child_key.clone(), parent_key));
                }
            }
        }
    }

    Ok(edges)
}
```

### Phase 2: Test with Covenant Repo

Expected edges:
```rust
("BaseAdapter", "lib:BaseAdapter")           // Covenant's inherits from Euler's
("BaseAdapter", "ICovenantPriceOracle")      // Covenant's implements interface
("lib:BaseAdapter", "IPriceOracle")          // Euler's implements interface
("ChainlinkOracle", "lib:ChainlinkOracle")   // Covenant's inherits from Euler's
("lib:ChainlinkOracle", "lib:BaseAdapter")   // Euler's inherits from Euler's BaseAdapter
("CovenantCurator", "IPriceOracle")          // Direct implementation
```

### Phase 3: Handle Edge Cases

If we find cases where the heuristic fails, implement Option C (DOT graph parsing).

---

## Next Steps

1. ✅ **Document the approach** (this file)
2. ✅ **Discover Slither IDs** (done - found in DOT graph)
3. ⏳ **Implement Option B heuristic** in `parse_inheritance_json`
4. ⏳ **Test with Covenant repo** to verify correct edges
5. ⏳ **Update helper functions** (`get_parents`, `get_children`)
6. ⏳ **Update call sites** one by one
7. ⏳ **Update tests** to expect prefixed names
8. ⏳ **Verify end-to-end** with full audit run

