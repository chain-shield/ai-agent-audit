# Parse Import Dependencies Function

## Overview

The `parse_all_import_dependencies()` function provides a comprehensive, single-pass analysis of all import statements in a Solidity source file. It obviates the need for multiple regex searches by parsing all imports once and categorizing dependencies by type and location.

## Function Signature

```rust
pub async fn parse_all_import_dependencies(
    source_code: &str,
    current_file: &PathBuf,
    repo: &RepoPaths,
) -> Result<ImportDependencies>
```

## Return Type

```rust
#[derive(Debug, Clone)]
pub struct ImportDependencies {
    /// Library files (from imports starting with '@')
    /// Example: "@openzeppelin/contracts/token/ERC20/ERC20.sol"
    pub lib_files: HashSet<PathBuf>,
    
    /// Source files (from relative imports like "./", "../")
    /// Example: "./interfaces/ICovenant.sol"
    pub source_files: HashSet<PathBuf>,
    
    /// Interfaces with their file paths
    /// Key: Interface name, Value: File path
    /// Example: ("IPriceOracle", "/path/to/IPriceOracle.sol")
    pub interfaces: HashMap<String, PathBuf>,
}
```

## Algorithm

### 1. Parse Named Imports

```solidity
import {IERC20, SafeERC20} from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import {IPriceOracle} from "./interfaces/IPriceOracle.sol";
import {BaseAdapter as EulerBaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";
```

For each named import:
1. Extract all imported names (handling aliases)
2. Determine if import path is a library (`@`) or source file
3. Resolve the file path using remappings or relative path resolution
4. Check each imported name to see if it's an interface using `get_file_from_contract()` or `get_file_from_lib_contract()`
5. Add to appropriate collections

### 2. Parse Simple Imports

```solidity
import "./utils/Helper.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
```

For each simple import:
1. Determine if it's a library (`@`) or source file
2. Resolve the file path
3. Add to appropriate collection

### 3. Library Filtering

The function uses `should_exclude_this_library()` to filter out standard libraries:
- OpenZeppelin (`openzeppelin`)
- Forge Standard Library (`forge-std`)
- DS-Test (`ds-test`)
- ERC4626 Tests (`erc4626-tests`)
- Halmos Cheatcodes (`halmos-cheatcodes`)

**Rationale**: These are well-audited standard libraries that don't need to be included in the analysis context.

## Path Resolution Strategy

### 1. Library Imports (Starting with `@`)

```solidity
import {BaseAdapter} from "@euler-price-oracle/adapter/BaseAdapter.sol";
```

- Uses `resolve_import_path()` to apply remappings from `remappings.txt`
- Example: `@euler-price-oracle` → `lib/euler-price-oracle`
- Constructs full path: `repo.root/repo.repo_name/lib/euler-price-oracle/adapter/BaseAdapter.sol`
- Canonicalizes to resolve `.` and `..` components

### 2. Relative Imports

```solidity
import {ICovenantPriceOracle} from "../interfaces/ICovenantPriceOracle.sol";
import {Errors} from "./lib/Errors.sol";
```

- Resolves relative to `current_file.parent()`
- Example: If current file is `/src/curators/oracles/BaseAdapter.sol`
  - `../interfaces/ICovenantPriceOracle.sol` → `/src/curators/interfaces/ICovenantPriceOracle.sol`
  - `./lib/Errors.sol` → `/src/curators/oracles/lib/Errors.sol`

### 3. Absolute Imports

```solidity
import {ICovenant} from "src/interfaces/ICovenant.sol";
```

- Resolves relative to `repo.root/repo.repo_name`
- Example: `src/interfaces/ICovenant.sol` → `/repo/root/repo_name/src/interfaces/ICovenant.sol`

## Interface Detection

The function determines if an imported name is an interface by:

1. **For library imports**: Calls `get_file_from_lib_contract(name, repo)` and checks if `contract_type == ContractType::Interface`
2. **For source imports**: Calls `get_file_from_contract(name, repo)` and checks if `contract_type == ContractType::Interface`

This requires that `contracts_in_source_folder()` has been called first to populate the contract-to-file mappings.

## Usage Example

```rust
// Load remappings
parse_and_store_remappings(&remapping_file, &repo.project_id)?;

// Parse all contracts to populate mappings
contracts_in_source_folder(&repo).await?;

// Parse imports from a specific file
let source_code = tokio::fs::read_to_string(&file).await?;
let deps = parse_all_import_dependencies(&source_code, &file, &repo).await?;

// Add lib files to context
for lib_file in deps.lib_files {
    context.add_file(lib_file);
}

// Add source files to context
for source_file in deps.source_files {
    context.add_file(source_file);
}

// Find root implementations of interfaces
for (interface_name, interface_file) in deps.interfaces {
    let children = get_children(&interface_name, &interface_file, &repo).await?;
    for (child_name, child_file) in children {
        // Process child implementations...
    }
}
```

## Test Results

### Test 1: CovenantCurator.sol

**Imports**:
```solidity
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
import {IPriceOracle} from "../interfaces/IPriceOracle.sol";
import {Errors} from "./lib/Errors.sol";
```

**Results**:
- **Library files**: 0 (OpenZeppelin and forge-std excluded)
- **Source files**: 2 (`IPriceOracle.sol`, `Errors.sol`)
- **Interfaces**: 1 (`IPriceOracle`)

### Test 2: ChainlinkOracle.sol

**Results**:
- **Library files**: 3 (Euler ChainlinkOracle, ScaleUtils, AggregatorV3Interface)
- **Source files**: 2 (`Errors.sol`, `IPriceOracle.sol`)
- **Interfaces**: 2 (`IPriceOracle`, `AggregatorV3Interface`)

### Test 3: BaseAdapter.sol

**Results**:
- **Library files**: 1 (Euler BaseAdapter)
- **Source files**: 2 (`ICovenantPriceOracle.sol`, `Errors.sol`)
- **Interfaces**: 2 (`IERC20`, `ICovenantPriceOracle`)

## Benefits

1. **Single Pass**: Parses all imports in one pass instead of multiple regex searches
2. **Comprehensive**: Captures all dependencies (lib files, source files, interfaces)
3. **Accurate Path Resolution**: Uses remappings and relative path resolution
4. **Interface Detection**: Automatically identifies interfaces using contract-to-file mappings
5. **Filtered**: Excludes standard libraries (OpenZeppelin, forge-std) to reduce noise
6. **Type-Safe**: Returns structured data with clear categorization

## Integration Points

- **Remapping System**: Uses `resolve_import_path()` from `src/utils/remapping.rs`
- **Contract Mappings**: Uses `get_file_from_contract()` and `get_file_from_lib_contract()` from `src/llm_review/contract_file_map.rs`
- **Inheritance System**: Interfaces can be used with `get_children()` to find implementations
- **Context Building**: All files can be added to AI agent context for comprehensive analysis

## Future Improvements

1. **Caching**: Add caching for parsed imports to avoid re-parsing the same file
2. **Transitive Dependencies**: Option to recursively parse imports of imported files
3. **Dependency Graph**: Build a complete dependency graph for the entire codebase
4. **Circular Detection**: Detect and report circular dependencies
5. **Unused Import Detection**: Identify imports that are never used in the code

