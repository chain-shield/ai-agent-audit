# Inheritance System: Duplicate Contract Names Analysis

## Problem Statement

**Critical Issue**: Our inheritance system assumes contract names are unique within a project, but this assumption is **VIOLATED** when:
- Projects use external libraries that define contracts with common names (e.g., `BaseAdapter`, `ChainlinkOracle`)
- The project extends/overrides these contracts with the same name in their own codebase

### Real-World Example: Covenant Protocol

```
Euler's BaseAdapter:     lib/euler-price-oracle/src/adapter/BaseAdapter.sol
Covenant's BaseAdapter:  src/curators/oracles/BaseAdapter.sol

Euler's ChainlinkOracle: lib/euler-price-oracle/src/adapter/chainlink/ChainlinkOracle.sol
Covenant's ChainlinkOracle: src/curators/oracles/chainlink/ChainlinkOracle.sol
```

**Current Behavior**: Slither returns edges like `BaseAdapter → BaseAdapter`, which our system cannot distinguish.

---

## Current Architecture

### 1. Data Structures

#### Inheritance Edges
```rust
// src/build_brain/inheritance.rs
pub fn parse_inheritance_json(json: &str) -> Result<Vec<(String, String)>>
// Returns: Vec<(child_name, parent_name)>
```

**Problem**: Uses bare contract names without file paths.

#### Inheritance Maps
```rust
// src/build_brain/callgraph.rs

// Child → Parents mapping
static INHERITANCE_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, Vec<String>>>>>

// Parent → Children mapping  
static INVERTED_INHERITANCE_MAP_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, Vec<String>>>>>
```

**Problem**: HashMap keys are bare contract names, causing collisions.

#### Contract-to-File Mapping
```rust
// src/llm_review/contract_file_map.rs

static CONTRACT_TO_FILE: Lazy<Arc<Mutex<HashMap<String, (PathBuf, ContractType)>>>>

// Key format: "{project_id}_{contract_name}"
```

**Problem**: Only one file per contract name per project. Last write wins.

---

## Usage Analysis

### 1. **Core Inheritance Functions** (src/build_brain/callgraph.rs)

#### `generate_inheritance_edges(repo)` → `Vec<(String, String)>`
- **Used by**: `get_inheritance_map`, `get_inverted_inheritance_map`, `enrichment.rs`
- **Impact**: Returns duplicate names without disambiguation
- **Fix Required**: ✅ Change return type to include file paths

#### `get_parents(contract, repo)` → `Vec<String>`
- **Used by**: `codeblocks.rs` (lines 135, 150)
- **Impact**: Returns parent names without file paths
- **Current Behavior**: If "BaseAdapter" has parents, returns ALL parents of ALL BaseAdapters
- **Fix Required**: ✅ Need to disambiguate which BaseAdapter

#### `get_children(contract, repo)` → `Vec<String>`
- **Used by**: `parse_solidity.rs` (line 519)
- **Impact**: Returns child names without file paths
- **Current Behavior**: If "IPriceOracle" has children, returns ALL children including duplicates
- **Fix Required**: ✅ Need to disambiguate which contract

#### `get_inheritance_map(repo)` → `HashMap<String, Vec<String>>`
- **Used by**: `get_parents`
- **Impact**: Map keys are contract names (collision-prone)
- **Fix Required**: ✅ Change key to unique identifier

#### `get_inverted_inheritance_map(repo)` → `HashMap<String, Vec<String>>`
- **Used by**: `get_children`, tests
- **Impact**: Map keys are contract names (collision-prone)
- **Fix Required**: ✅ Change key to unique identifier

#### `get_ancestors(child, inheritance_map, max_depth)` → `Vec<String>`
- **Used by**: Internal traversal
- **Impact**: Traverses using bare names
- **Fix Required**: ✅ Update to use unique identifiers

---

### 2. **Code Block Generation** (src/enumerator/codeblocks.rs)

**Lines 135-157**: Adds parent contracts to code blocks
```rust
let parents_of_main = callgraph::get_parents(&main_contract, repo).await?;
for parent in &parents_of_main {
    if !is_standard_interface_name(parent) && !is_standard_library_contract_name(parent) {
        contracts_with_parents.insert(parent.clone());
    }
}
```

**Impact**: 
- If `main_contract = "BaseAdapter"` (Covenant's), we might get parents of Euler's BaseAdapter
- Could include wrong contracts in code blocks
- **Severity**: HIGH - affects audit scope

**Fix Required**: ✅ Need to pass file path or unique ID to `get_parents`

---

### 3. **Source Dependency Detection** (src/enumerator/parse_solidity.rs)

**Lines 502-547**: Finds children of interfaces
```rust
for interface in &filtered_interfaces {
    if let Some((_, contract_type)) = get_file_from_contract(interface, repo).await {
        if contract_type != ContractType::Interface {
            continue;
        }
        let children = get_children(interface, repo).await?;
        // Process children...
    }
}
```

**Impact**:
- If interface name is duplicated, gets children of ALL interfaces with that name
- Could miss or incorrectly include contracts
- **Severity**: MEDIUM - affects dependency detection

**Fix Required**: ✅ Need file-aware `get_children`

---

### 4. **Semantic Database** (src/build_brain/enrichment.rs)

**Line 145**: Stores inheritance edges in database
```rust
let inheritance_edges = callgraph::generate_inheritance_edges(&repo).await?;
// Stores edges in semantic database
```

**Impact**:
- Database stores `(child_name, parent_name)` tuples
- No way to distinguish duplicate names in queries
- **Severity**: MEDIUM - affects semantic search

**Fix Required**: ✅ Store file paths in database

---

### 5. **Contract-to-File Mapping** (src/llm_review/contract_file_map.rs)

**Lines 28-42**: Inserts contract → file mapping
```rust
pub async fn insert_contract_to_file_mapping(
    contract: &str,
    file: &Path,
    contract_type: ContractType,
    repo: &RepoPaths,
) -> anyhow::Result<()> {
    let key = format!("{}_{}", repo.project_id, contract);
    contract_file_map.insert(key, (file.to_owned(), contract_type));
    Ok(())
}
```

**Impact**:
- If "BaseAdapter" is inserted twice, **last write wins**
- Euler's BaseAdapter might overwrite Covenant's (or vice versa)
- **Severity**: CRITICAL - loses file mapping data

**Fix Required**: ✅ Change key to include file path hash or use different structure

---

## Proposed Solutions

### Option 1: Fully Qualified Contract Names (Recommended)

**Concept**: Use `ContractName@FilePath` as unique identifier

```rust
// New type for unique contract identification
pub struct ContractId {
    pub name: String,
    pub file_path: PathBuf,  // Relative to repo root
}

impl ContractId {
    pub fn to_key(&self) -> String {
        format!("{}@{}", self.name, self.file_path.display())
    }
}

// Updated return types
pub async fn generate_inheritance_edges(repo: &RepoPaths) 
    -> Result<Vec<(ContractId, ContractId)>>

pub async fn get_parents(contract: &ContractId, repo: &RepoPaths) 
    -> Result<Vec<ContractId>>

pub async fn get_children(contract: &ContractId, repo: &RepoPaths) 
    -> Result<Vec<ContractId>>
```

**Pros**:
- ✅ Completely eliminates ambiguity
- ✅ Preserves all information
- ✅ Easy to debug (can see file paths)

**Cons**:
- ❌ Breaking change to all call sites
- ❌ More complex API
- ❌ Larger memory footprint

---

### Option 2: File Path Hash Suffix

**Concept**: Append hash of file path to contract name

```rust
pub fn make_unique_contract_name(name: &str, file: &Path) -> String {
    let hash = hash_path(file);  // e.g., first 8 chars of SHA256
    format!("{}#{}", name, hash)
}

// Example:
// "BaseAdapter#a1b2c3d4" (Euler's)
// "BaseAdapter#e5f6g7h8" (Covenant's)
```

**Pros**:
- ✅ Minimal API changes (still uses String)
- ✅ Unique identifiers
- ✅ Smaller memory footprint

**Cons**:
- ❌ Hash collisions possible (though unlikely)
- ❌ Less readable in logs/debugging
- ❌ Need to maintain name→file mapping separately

---

### Option 3: Hierarchical HashMap

**Concept**: Two-level HashMap: `contract_name → file_path → data`

```rust
// Contract name → (file path → parents)
type InheritanceMap = HashMap<String, HashMap<PathBuf, Vec<(String, PathBuf)>>>;

pub async fn get_parents(contract: &str, file: &Path, repo: &RepoPaths) 
    -> Result<Vec<(String, PathBuf)>>
```

**Pros**:
- ✅ Preserves contract names as-is
- ✅ Explicit file path in API
- ✅ Easy to query all contracts with same name

**Cons**:
- ❌ More complex data structure
- ❌ Requires file path in all API calls
- ❌ Harder to cache efficiently

---

## Recommended Approach

**Use Option 1 (Fully Qualified Contract Names)** with phased rollout:

### Phase 1: Internal Data Structures
1. Create `ContractId` struct
2. Update `parse_inheritance_json` to return `Vec<(ContractId, ContractId)>`
3. Update inheritance map caches to use `ContractId` keys
4. Update `generate_inheritance_edges` return type

### Phase 2: Core APIs
5. Update `get_parents` and `get_children` to accept/return `ContractId`
6. Update `get_inheritance_map` and `get_inverted_inheritance_map` key types
7. Add helper functions to convert between `String` and `ContractId`

### Phase 3: Call Sites
8. Update `codeblocks.rs` to use `ContractId`
9. Update `parse_solidity.rs` to use `ContractId`
10. Update `enrichment.rs` to store `ContractId` in database
11. Update contract-to-file mapping to use `ContractId` keys

### Phase 4: Backward Compatibility
12. Add `get_parents_by_name` that returns ALL parents of ALL contracts with that name
13. Add deprecation warnings to old APIs
14. Update tests

---

## Migration Strategy

### Step 1: Add ContractId without breaking changes
```rust
// New struct
pub struct ContractId { name: String, file_path: PathBuf }

// New functions (don't touch old ones yet)
pub async fn get_parents_v2(contract: &ContractId, repo: &RepoPaths) -> Result<Vec<ContractId>>
pub async fn get_children_v2(contract: &ContractId, repo: &RepoPaths) -> Result<Vec<ContractId>>
```

### Step 2: Update Slither parsing to extract file paths
```rust
// Check if Slither JSON includes file paths
// If not, we need to match contract names to files using contract_file_map
```

### Step 3: Gradually migrate call sites
- Start with `parse_solidity.rs` (most affected)
- Then `codeblocks.rs`
- Finally `enrichment.rs`

### Step 4: Remove old APIs
- Once all call sites migrated, remove `get_parents`, `get_children` (old versions)
- Rename `get_parents_v2` → `get_parents`

---

## Open Questions

1. **Does Slither JSON include file paths?**
   - Need to check actual JSON output
   - If not, we need to resolve names to files using `contract_file_map`

2. **How to handle standard library contracts?**
   - Should we still use bare names for `IERC20`, `Ownable`, etc.?
   - Or always use full paths?

3. **Performance impact?**
   - `ContractId` with `PathBuf` is larger than `String`
   - Need to measure memory/performance impact

4. **Database schema changes?**
   - Semantic database stores inheritance edges
   - Need to update schema to include file paths

---

## Next Steps

1. ✅ **Examine Slither JSON output** - Check if file paths are available
2. ⏳ **Prototype ContractId** - Create struct and basic functions
3. ⏳ **Update parse_inheritance_json** - Extract file paths if available
4. ⏳ **Create migration plan** - Detailed step-by-step with testing
5. ⏳ **Update tests** - Ensure all tests pass with new system

