# Inheritance Parsing: Import-Based Implementation

## Overview

This document describes the **import-based inheritance parsing** implementation that resolves parent contracts by analyzing import statements rather than guessing file locations.

## Problem Solved

### Previous Approach (Guessing)
```rust
// We see: contract BaseAdapter is EulerBaseAdapter
// We guess: "Is EulerBaseAdapter in source? No. Is it in lib? Yes."
// Problem: What if there are multiple EulerBaseAdapter contracts in different lib folders?
```

### New Approach (Import-Based)
```solidity
import {BaseAdapter as EulerBaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";

abstract contract BaseAdapter is EulerBaseAdapter, ICovenantPriceOracle {
    // ...
}
```

**We extract the exact file path from the import statement!**

## Implementation Details

### Step 1: Parse Import Statements

For each Solidity file, we build a mapping: `contract_name → import_path`

```rust
// Uses same regex as parse_solidity.rs for consistency
let import_regex = Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*[\"']([^\"']+)[\"']"#).unwrap();
let import_item_regex = Regex::new(
    r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:(?i:as)\s+[A-Za-z_][A-Za-z0-9_]*)?\s*$"
).unwrap();
```

**Handles:**
- Named imports: `import {ContractA, ContractB} from "path"`
- Aliases: `import {BaseAdapter as EulerBaseAdapter} from "path"`
- Multiple imports: `import {A, B, C as D} from "path"`

**Creates mapping:**
```rust
HashMap<String, String> {
    "EulerBaseAdapter" => "@euler-price-oracle/adapter/BaseAdapter.sol",
    "ICovenantPriceOracle" => "../interfaces/ICovenantPriceOracle.sol",
}
```

### Step 2: Parse Inheritance Relationships

```rust
let inheritance_regex = Regex::new(
    r"(?m)^\s*(?:abstract\s+)?(?:contract|interface|library)\s+([A-Za-z_][A-Za-z0-9_]*)\s+is\s+([^{]+)"
).unwrap();
```

Extracts:
- Child contract name
- Parent list (comma-separated)

### Step 3: Resolve Import Paths

For each parent contract, look it up in the import map and resolve the path:

```rust
if let Some(import_path) = import_map.get(&parent) {
    // 1. Try remapping resolution (for @euler-price-oracle, etc.)
    let resolved_path = if let Some(remapped) = 
        crate::utils::remapping::resolve_import_path(import_path, repo)
    {
        PathBuf::from(remapped)
    }
    // 2. Handle relative paths (../, ./)
    else if import_path.starts_with("../") || import_path.starts_with("./") {
        if let Some(parent_dir) = file.parent() {
            parent_dir.join(import_path)
        } else {
            PathBuf::from(import_path)
        }
    }
    // 3. Try as-is
    else {
        PathBuf::from(import_path)
    };
}
```

### Step 4: Store Inheritance Edge

```rust
crate::build_brain::inheritance_map::insert_inheritance_edge(
    (child_contract.to_string(), file.to_path_buf()),
    (parent.clone(), parent_file.clone()),
    repo,
).await
```

## Path Resolution Strategy

### 1. Remapping Resolution (Highest Priority)
- Uses `crate::utils::remapping::resolve_import_path()`
- Handles: `@euler-price-oracle` → `lib/euler-price-oracle`
- Reads from `remappings.txt` or `foundry.toml`

### 2. Relative Path Resolution
- Handles: `../interfaces/ICovenantPriceOracle.sol`
- Resolves relative to the current file's directory

### 3. Absolute Path Resolution
- Handles: `src/curators/interfaces/ICovenantPriceOracle.sol`
- Resolves relative to repo root

### 4. Canonicalization
- Uses `PathBuf::canonicalize()` to resolve `.` and `..` components
- Ensures consistent path representation

## Example: Covenant's BaseAdapter

### Input File: `src/curators/oracles/BaseAdapter.sol`

```solidity
import {BaseAdapter as EulerBaseAdapter, IERC20} from "@euler-price-oracle/adapter/BaseAdapter.sol";
import {ICovenantPriceOracle} from "../interfaces/ICovenantPriceOracle.sol";

abstract contract BaseAdapter is EulerBaseAdapter, ICovenantPriceOracle {
    // ...
}
```

### Step 1: Import Map
```rust
{
    "EulerBaseAdapter" => "@euler-price-oracle/adapter/BaseAdapter.sol",
    "IERC20" => "@euler-price-oracle/adapter/BaseAdapter.sol",
    "ICovenantPriceOracle" => "../interfaces/ICovenantPriceOracle.sol",
}
```

### Step 2: Inheritance Parsing
- Child: `BaseAdapter`
- Parents: `["EulerBaseAdapter", "ICovenantPriceOracle"]`

### Step 3: Path Resolution

**Parent 1: EulerBaseAdapter**
- Import path: `@euler-price-oracle/adapter/BaseAdapter.sol`
- Remapping: `lib/euler-price-oracle/src/adapter/BaseAdapter.sol`
- File type: `SolFileType::LibFolder`

**Parent 2: ICovenantPriceOracle**
- Import path: `../interfaces/ICovenantPriceOracle.sol`
- Relative resolution: `src/curators/interfaces/ICovenantPriceOracle.sol`
- File type: `SolFileType::Standard`

### Step 4: Stored Edges

```rust
// Edge 1
child: ("BaseAdapter", "src/curators/oracles/BaseAdapter.sol")
parent: ("EulerBaseAdapter", "lib/euler-price-oracle/src/adapter/BaseAdapter.sol")

// Edge 2
child: ("BaseAdapter", "src/curators/oracles/BaseAdapter.sol")
parent: ("ICovenantPriceOracle", "src/curators/interfaces/ICovenantPriceOracle.sol")
```

## Benefits

### ✅ **Accurate**
- No guessing - uses exact import paths from source code
- Handles duplicate contract names correctly

### ✅ **Handles Aliases**
- `import {BaseAdapter as EulerBaseAdapter}` → correctly maps `EulerBaseAdapter` to the file

### ✅ **Supports All Import Styles**
- Remappings: `@euler-price-oracle`
- Relative paths: `../interfaces/ICovenantPriceOracle.sol`
- Absolute paths: `src/curators/interfaces/ICovenantPriceOracle.sol`

### ✅ **Consistent with Existing Code**
- Uses same regex patterns as `parse_solidity.rs`
- Reuses `resolve_import_path()` from `utils/remapping.rs`

### ✅ **Disambiguates Lib vs Source**
- Uses `is_library_file()` to determine `SolFileType`
- Stores `(contract, file)` tuples as unique keys

## Edge Cases Handled

### 1. Parent Not in Imports
```solidity
// Parent from same file - no import needed
contract Parent { }
contract Child is Parent { }
```
**Handling**: Logged as debug message, skipped (same-file inheritance not critical for analysis)

### 2. Parent from Excluded Library
```solidity
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
contract MyContract is Ownable { }
```
**Handling**: Import path won't resolve (excluded), logged as debug message

### 3. Multiple Contracts in Same Import
```solidity
import {BaseAdapter as EulerBaseAdapter, IERC20} from "@euler-price-oracle/adapter/BaseAdapter.sol";
```
**Handling**: Both `EulerBaseAdapter` and `IERC20` mapped to the same file

### 4. Alias Handling
```solidity
import {BaseAdapter as EulerBaseAdapter} from "...";
contract MyAdapter is EulerBaseAdapter { } // Uses alias
```
**Handling**: Import map uses the alias as the key

## Files Modified

### `src/enumerator/utils.rs`
- Added import parsing logic (lines 427-464)
- Added inheritance resolution logic (lines 466-556)
- Uses existing regex patterns from `parse_solidity.rs`

### `src/build_brain/inheritance_map.rs`
- New module for inheritance map storage
- Global HashMaps with `(contract, file)` tuple keys
- API functions: `get_parents()`, `get_children()` with `SolFileType` parameter

### `tests/inheritance_solidity_parsing_test.rs`
- Integration test for Covenant repository
- Verifies correct inheritance relationships

## Next Steps

1. ✅ **Implementation Complete** - Import-based parsing implemented
2. ⏳ **Testing** - Run integration test on Covenant repository
3. ⏳ **Update API** - Update `get_parents()` and `get_children()` in `callgraph.rs`
4. ⏳ **Update Call Sites** - Update all callers to pass `SolFileType` parameter
5. ⏳ **Deprecate Slither** - Remove Slither-based inheritance parsing

## Logging

The implementation includes comprehensive logging:

```rust
log::info!(
    "Inheritance: {} ({:?}, {}) -> {} ({:?}, {})",
    child_contract,
    file_type,
    file.display(),
    parent,
    parent_file_type,
    parent_file.display()
);
```

Example output:
```
Inheritance: BaseAdapter (Standard, src/curators/oracles/BaseAdapter.sol) 
  -> EulerBaseAdapter (LibFolder, lib/euler-price-oracle/src/adapter/BaseAdapter.sol)
```

