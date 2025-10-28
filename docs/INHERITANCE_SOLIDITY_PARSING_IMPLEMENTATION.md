# Inheritance Parsing: Direct Solidity Parsing Implementation Plan

## Executive Summary

**Approach**: Parse inheritance relationships directly from Solidity source files in `src/enumerator/utils.rs::contracts_in_source_folder()`.

**Key Insight**: We already iterate through every Solidity file and have file paths. We can extract inheritance relationships (`contract X is Y, Z`) and build a complete inheritance map with `(contract, file)` tuples as keys.

**Data Structure**:
```rust
// Key: (contract_name, file_path)
// Value: Vec of parent (contract_name, file_path) tuples
type InheritanceMap = HashMap<(String, PathBuf), Vec<(String, PathBuf)>>;

// Also maintain inverted map for parent → children lookups
type InvertedInheritanceMap = HashMap<(String, PathBuf), Vec<(String, PathBuf)>>;
```

---

## Why This Approach is Superior

### ✅ **Advantages Over Slither-Based Parsing**

1. **No Ambiguity** - `(contract, file)` tuples are naturally unique
2. **Complete Control** - We parse Solidity ourselves, no external tool quirks
3. **File Paths Included** - Already have the file when parsing
4. **Already in Right Place** - `contracts_in_source_folder` reads every file
5. **Deterministic** - No reliance on Slither's internal ID system
6. **Handles Duplicates Perfectly** - Different files = different keys
7. **Faster** - No need to run Slither inheritance printer
8. **More Reliable** - Slither can fail on complex projects

### ✅ **We Already Have Everything**

- ✅ File iteration in `contracts_in_source_folder`
- ✅ Regex for contract declarations
- ✅ File-to-contract mappings (`CONTRACT_TO_FILE`, `LIB_CONTRACT_TO_FILE`)
- ✅ Lib vs source detection (`SolFileType`)
- ✅ Content reading and parsing infrastructure

---

## Solidity Inheritance Syntax

### Examples from Covenant Repo

```solidity
// Single inheritance
contract CovenantCurator is Ownable2Step, IPriceOracle {

// Multiple inheritance with abstract
abstract contract BaseAdapter is EulerBaseAdapter, ICovenantPriceOracle {

// Interface inheritance
interface ICovenantPriceOracle is IPriceOracle {

// Complex inheritance
contract Covenant is NoDelegateCall, Ownable2Step {
```

### Regex Pattern

```rust
// Match: (abstract )?(contract|interface|library) NAME is PARENT1, PARENT2, ...
let inheritance_regex = Regex::new(
    r"(?:abstract\s+)?(?:contract|interface|library)\s+(\w+)\s+is\s+([^{]+)"
)?;

// Capture groups:
// 1: Contract name (e.g., "BaseAdapter")
// 2: Parent list (e.g., "EulerBaseAdapter, ICovenantPriceOracle")
```

### Parsing Parent List

```rust
// Split by comma and trim whitespace
let parents: Vec<String> = parent_list
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect();
```

---

## Implementation Plan

### Phase 1: Add Inheritance Parsing to `contracts_in_source_folder`

**File**: `src/enumerator/utils.rs`

**Location**: Inside the file iteration loop (around line 389)

**Steps**:
1. Add inheritance regex alongside existing `contract_decl_regex`
2. For each file, extract inheritance relationships
3. Resolve parent contract names to file paths using existing mappings
4. Store in global inheritance maps

**Pseudocode**:
```rust
// At top of function, add new regex
let inheritance_regex = Regex::new(
    r"(?:abstract\s+)?(?:contract|interface|library)\s+(\w+)\s+is\s+([^{]+)"
)?;

// Inside file iteration loop (after parsing contract declarations)
for cap in inheritance_regex.captures_iter(&content) {
    let child_contract = cap.get(1).unwrap().as_str();
    let parent_list = cap.get(2).unwrap().as_str();
    
    // Parse parent list
    let parents: Vec<String> = parent_list
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    // Resolve parent file paths and store relationships
    for parent in parents {
        // Resolve parent to file path (check both source and lib)
        let parent_file = resolve_contract_file(&parent, repo).await?;
        
        // Store in inheritance map
        insert_inheritance_edge(
            (child_contract, file.to_path_buf()),
            (parent, parent_file),
            repo
        ).await?;
    }
}
```

### Phase 2: Create Global Inheritance Maps

**File**: `src/llm_review/contract_file_map.rs` (or new file `src/build_brain/inheritance_map.rs`)

**Data Structures**:
```rust
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::path::PathBuf;

// Child → Parents mapping
// Key: (child_contract, child_file)
// Value: Vec of (parent_contract, parent_file)
static INHERITANCE_MAP: Lazy<Arc<Mutex<HashMap<String, HashMap<(String, PathBuf), Vec<(String, PathBuf)>>>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// Parent → Children mapping (inverted)
// Key: (parent_contract, parent_file)
// Value: Vec of (child_contract, child_file)
static INVERTED_INHERITANCE_MAP: Lazy<Arc<Mutex<HashMap<String, HashMap<(String, PathBuf), Vec<(String, PathBuf)>>>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
```

**Helper Functions**:
```rust
/// Insert an inheritance edge: child inherits from parent
pub async fn insert_inheritance_edge(
    child: (String, PathBuf),
    parent: (String, PathBuf),
    repo: &RepoPaths,
) -> Result<()> {
    let project_id = repo.project_id.clone();
    
    // Insert into child → parents map
    let mut map = INHERITANCE_MAP.lock().await;
    map.entry(project_id.clone())
        .or_insert_with(HashMap::new)
        .entry(child.clone())
        .or_insert_with(Vec::new)
        .push(parent.clone());
    
    // Insert into parent → children map (inverted)
    let mut inv_map = INVERTED_INHERITANCE_MAP.lock().await;
    inv_map.entry(project_id)
        .or_insert_with(HashMap::new)
        .entry(parent)
        .or_insert_with(Vec::new)
        .push(child);
    
    Ok(())
}

/// Get all parents of a contract
pub async fn get_parents_with_file(
    contract: &str,
    file: &Path,
    repo: &RepoPaths,
) -> Result<Vec<(String, PathBuf)>> {
    let map = INHERITANCE_MAP.lock().await;
    
    if let Some(project_map) = map.get(&repo.project_id) {
        if let Some(parents) = project_map.get(&(contract.to_string(), file.to_path_buf())) {
            return Ok(parents.clone());
        }
    }
    
    Ok(Vec::new())
}

/// Get all children of a contract
pub async fn get_children_with_file(
    contract: &str,
    file: &Path,
    repo: &RepoPaths,
) -> Result<Vec<(String, PathBuf)>> {
    let inv_map = INVERTED_INHERITANCE_MAP.lock().await;
    
    if let Some(project_map) = inv_map.get(&repo.project_id) {
        if let Some(children) = project_map.get(&(contract.to_string(), file.to_path_buf())) {
            return Ok(children.clone());
        }
    }
    
    Ok(Vec::new())
}
```

### Phase 3: Resolve Parent Contract to File Path

**Challenge**: When we see `contract BaseAdapter is EulerBaseAdapter`, we need to find the file for `EulerBaseAdapter`.

**Solution**: Use existing contract-to-file mappings!

```rust
/// Resolve a contract name to its file path
/// Checks both source and lib contract mappings
pub async fn resolve_contract_file(
    contract: &str,
    repo: &RepoPaths,
) -> Result<Option<PathBuf>> {
    // First check source contracts
    if let Some((file, _)) = get_file_from_contract(contract, repo).await {
        return Ok(Some(file));
    }
    
    // Then check lib contracts
    if let Some(file) = get_file_from_lib_contract(contract, repo).await {
        return Ok(Some(file));
    }
    
    // Contract not found (might be from excluded lib like OpenZeppelin)
    Ok(None)
}
```

**Handling Missing Parents**:
```rust
for parent in parents {
    if let Some(parent_file) = resolve_contract_file(&parent, repo).await? {
        insert_inheritance_edge(
            (child_contract.to_string(), file.to_path_buf()),
            (parent.clone(), parent_file),
            repo
        ).await?;
    } else {
        // Parent not found - likely from excluded library (OZ, Solmate, etc.)
        info!("Parent contract '{}' not found for '{}' - likely from excluded library", 
              parent, child_contract);
    }
}
```

### Phase 4: Update Existing API Functions

**File**: `src/build_brain/callgraph.rs`

**Current Functions** (to deprecate or update):
- `generate_inheritance_edges()` - Replace with Solidity parsing
- `get_parents()` - Update to use new map
- `get_children()` - Update to use new map

**New API**:
```rust
/// Get parents of a contract (auto-resolves file path)
pub async fn get_parents(contract: &str, repo: &RepoPaths) -> Result<Vec<String>> {
    // Resolve contract to file
    let file = resolve_contract_file(contract, repo).await?
        .ok_or_else(|| anyhow!("Contract '{}' not found", contract))?;
    
    // Get parents with files
    let parents_with_files = get_parents_with_file(contract, &file, repo).await?;
    
    // Extract just the contract names
    Ok(parents_with_files.into_iter().map(|(name, _)| name).collect())
}

/// Get children of a contract (auto-resolves file path)
pub async fn get_children(contract: &str, repo: &RepoPaths) -> Result<Vec<String>> {
    // Resolve contract to file
    let file = resolve_contract_file(contract, repo).await?
        .ok_or_else(|| anyhow!("Contract '{}' not found", contract))?;
    
    // Get children with files
    let children_with_files = get_children_with_file(contract, &file, repo).await?;
    
    // Extract just the contract names
    Ok(children_with_files.into_iter().map(|(name, _)| name).collect())
}
```

**Backward Compatibility**: Existing call sites continue to work!

---

## Handling Edge Cases

### Case 1: Duplicate Contract Names

**Example**: Both `src/BaseAdapter.sol` and `lib/euler/BaseAdapter.sol` exist.

**Solution**: The `(contract, file)` tuple key naturally handles this!

```rust
// Two separate entries in the map:
inheritance_map.insert(
    ("BaseAdapter", "src/curators/oracles/BaseAdapter.sol"),
    vec![
        ("EulerBaseAdapter", "lib/euler-price-oracle/src/adapter/BaseAdapter.sol"),
        ("ICovenantPriceOracle", "src/curators/interfaces/ICovenantPriceOracle.sol"),
    ]
);

inheritance_map.insert(
    ("BaseAdapter", "lib/euler-price-oracle/src/adapter/BaseAdapter.sol"),
    vec![
        ("IPriceOracle", "lib/euler-price-oracle/src/interfaces/IPriceOracle.sol"),
    ]
);
```

### Case 2: Parent Contract Not Found

**Example**: `contract MyToken is ERC20` where ERC20 is from OpenZeppelin (excluded).

**Solution**: Skip the edge, log a warning.

```rust
if let Some(parent_file) = resolve_contract_file(&parent, repo).await? {
    insert_inheritance_edge(...).await?;
} else {
    info!("Parent '{}' not found - likely from excluded library", parent);
}
```

### Case 3: Import Aliases

**Example**: 
```solidity
import {BaseAdapter as EulerBaseAdapter} from "lib/euler/BaseAdapter.sol";
contract MyAdapter is EulerBaseAdapter { }
```

**Solution**: The alias `EulerBaseAdapter` won't resolve to a file. We'd need import parsing to handle this.

**Mitigation**: This is rare. For now, log a warning. Can enhance later with import parsing.

---

## Testing Strategy

### Unit Tests

**File**: `tests/inheritance_parsing_test.rs`

```rust
#[tokio::test]
async fn test_parse_simple_inheritance() {
    let content = r#"
        contract Child is Parent {
            function foo() public {}
        }
    "#;
    
    let relationships = parse_inheritance_from_content(content);
    assert_eq!(relationships.len(), 1);
    assert_eq!(relationships[0], ("Child", vec!["Parent"]));
}

#[tokio::test]
async fn test_parse_multiple_inheritance() {
    let content = r#"
        abstract contract BaseAdapter is EulerBaseAdapter, ICovenantPriceOracle {
            // ...
        }
    "#;
    
    let relationships = parse_inheritance_from_content(content);
    assert_eq!(relationships[0].1, vec!["EulerBaseAdapter", "ICovenantPriceOracle"]);
}
```

### Integration Test

**File**: `tests/inheritance_covenant_test.rs`

```rust
#[tokio::test]
#[ignore]
async fn test_covenant_inheritance_parsing() {
    let repo = RepoPaths::new("/private/tmp/audit-analysis/2025-10-covenant-d5ebe4/2025-10-covenant");
    
    // Parse all contracts
    contracts_in_source_folder(&repo).await.unwrap();
    
    // Check BaseAdapter inheritance
    let parents = get_parents_with_file(
        "BaseAdapter",
        Path::new("src/curators/oracles/BaseAdapter.sol"),
        &repo
    ).await.unwrap();
    
    assert_eq!(parents.len(), 2);
    assert!(parents.iter().any(|(name, _)| name == "EulerBaseAdapter"));
    assert!(parents.iter().any(|(name, _)| name == "ICovenantPriceOracle"));
}
```

---

## Migration Path

### Step 1: Implement Parsing (Non-Breaking)
- Add inheritance regex to `contracts_in_source_folder`
- Create new inheritance maps
- Add helper functions
- **No changes to existing code**

### Step 2: Add Tests
- Unit tests for regex parsing
- Integration tests with Covenant repo
- Verify correct relationships extracted

### Step 3: Update API Functions (Backward Compatible)
- Update `get_parents()` to use new map
- Update `get_children()` to use new map
- Keep same function signatures

### Step 4: Update Call Sites (Gradual)
- `src/enumerator/codeblocks.rs` - Use new API
- `src/enumerator/parse_solidity.rs` - Use new API
- `src/build_brain/enrichment.rs` - Use new API

### Step 5: Deprecate Slither Inheritance
- Remove `generate_inheritance_edges()` Slither calls
- Remove `parse_inheritance_json()`
- Clean up old code

---

## Performance Considerations

### Regex Compilation
- Compile regex once at function start (already done for `contract_decl_regex`)
- Reuse across all files

### File Reading
- Already reading files for contract declaration parsing
- No additional I/O overhead

### Memory
- Inheritance map size: ~O(number of contracts × average parents)
- For typical projects: <1000 contracts × 2 parents = ~2000 entries
- Negligible memory impact

---

## Next Steps

1. ⏳ **Implement inheritance regex** in `contracts_in_source_folder`
2. ⏳ **Create global inheritance maps** in new module
3. ⏳ **Add `resolve_contract_file` helper**
4. ⏳ **Test with Covenant repo**
5. ⏳ **Update API functions** for backward compatibility
6. ⏳ **Update call sites** one by one
7. ⏳ **Remove Slither inheritance code**

