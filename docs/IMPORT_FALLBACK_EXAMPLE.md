# Import-Only Fallback: Concrete Example

## Example Repository Structure

```
src/
├── Core.sol              // Main contract
│   └── imports: IPool, Math, Events
├── interfaces/
│   └── IPool.sol         // Interface (EXCLUDE)
│       └── imports: IERC20
├── libraries/
│   ├── Math.sol          // Library in /src (INCLUDE - non-standard source library)
│   └── Events.sol        // Library in /src (INCLUDE - non-standard source library)
├── Pool.sol              // Concrete contract (implements IPool)
│   └── imports: SafeERC20, ReentrancyGuard
├── SafeERC20.sol         // Utility contract
│   └── imports: IERC20
└── ReentrancyGuard.sol   // Utility contract
    └── imports: (none)

lib/
└── openzeppelin/         // Standard library (EXCLUDE)
    └── IERC20.sol
```

## Traversal Example: main_contract = "Core"

### Level 0: Main Contract
```
Input: "Core"

1. Get file: src/Core.sol
2. visited_files = {src/Core.sol}
3. detect_source_code_dependencies("Core"):
   - source_files: {interfaces/IPool.sol, libraries/Math.sol, libraries/Events.sol}
   - interface_implementations: {("Pool", src/Pool.sol)}
4. Extract contracts from source_files:
   - interfaces/IPool.sol → "IPool" (SKIP - interface)
   - libraries/Math.sol → "Math" (INCLUDE - library in /src source folder)
   - libraries/Events.sol → "Events" (INCLUDE - library in /src source folder)
5. Add interface_implementations:
   - "Pool" → level_1_contracts

Result:
- level_0_contracts = {"Core"}
- level_1_contracts = {"Pool", "Math", "Events"}
- visited_files = {src/Core.sol}
```

### Level 1: Direct Imports
```
Input: level_1_contracts = {"Pool", "Math", "Events"}

For "Pool":
1. Get file: src/Pool.sol
2. visited_files = {src/Core.sol, src/Pool.sol}
3. detect_source_code_dependencies("Pool"):
   - source_files: {src/SafeERC20.sol, src/ReentrancyGuard.sol}
   - interface_implementations: {}
4. Extract contracts from source_files:
   - src/SafeERC20.sol → "SafeERC20" → level_2_contracts
   - src/ReentrancyGuard.sol → "ReentrancyGuard" → level_2_contracts

For "Math":
1. Get file: src/libraries/Math.sol
2. visited_files = {src/Core.sol, src/Pool.sol, src/libraries/Math.sol}
3. detect_source_code_dependencies("Math"):
   - source_files: {}
4. No new contracts

For "Events":
1. Get file: src/libraries/Events.sol
2. visited_files = {src/Core.sol, src/Pool.sol, src/libraries/Math.sol, src/libraries/Events.sol}
3. detect_source_code_dependencies("Events"):
   - source_files: {}
4. No new contracts

Result:
- level_2_contracts = {"SafeERC20", "ReentrancyGuard"}
- visited_files = {src/Core.sol, src/Pool.sol, src/libraries/Math.sol, src/libraries/Events.sol}
```

### Level 2: Transitive Imports
```
Input: level_2_contracts = {"SafeERC20", "ReentrancyGuard"}

For "SafeERC20":
1. Get file: src/SafeERC20.sol
2. visited_files = {src/Core.sol, src/Pool.sol, src/SafeERC20.sol}
3. detect_source_code_dependencies("SafeERC20"):
   - source_files: {lib/openzeppelin/IERC20.sol}
   - interface_implementations: {}
4. Extract contracts from source_files:
   - lib/openzeppelin/IERC20.sol → "IERC20" (SKIP - standard library)

For "ReentrancyGuard":
1. Get file: src/ReentrancyGuard.sol
2. visited_files = {src/Core.sol, src/Pool.sol, src/SafeERC20.sol, src/ReentrancyGuard.sol}
3. detect_source_code_dependencies("ReentrancyGuard"):
   - source_files: {}
   - interface_implementations: {}
4. No new contracts

Result:
- level_3_contracts = {}
- visited_files = {src/Core.sol, src/Pool.sol, src/SafeERC20.sol, src/ReentrancyGuard.sol}
```

### Level 3: Deep Imports
```
Input: level_3_contracts = {}

Nothing to process.
```

### Final Assembly
```
contracts = level_0 ∪ level_1 ∪ level_2 ∪ level_3
          = {"Core"} ∪ {"Pool", "Math", "Events"} ∪ {"SafeERC20", "ReentrancyGuard"} ∪ {}
          = {"Core", "Pool", "Math", "Events", "SafeERC20", "ReentrancyGuard"}

contracts_with_depth = level_1 ∪ level_2
                     = {"Pool", "Math", "Events"} ∪ {"SafeERC20", "ReentrancyGuard"}
                     = {"Pool", "Math", "Events", "SafeERC20", "ReentrancyGuard"}

Return: (
    contracts = {"Core", "Pool", "Math", "Events", "SafeERC20", "ReentrancyGuard"},
    contracts_with_depth = {"Pool", "Math", "Events", "SafeERC20", "ReentrancyGuard"},
    token_count = 0  // Calculated later during assembly
)
```

## What Gets Excluded

### Excluded by Type
- **Interfaces**: IPool, IERC20 (detected by contract_type == Interface)
- **Libraries in lib/ folders**: Standard library contracts (OpenZeppelin, forge-std, etc.)

### Included by Type (NEW - matches BFS behavior)
- **Libraries in /src folders**: Math, Events (non-standard source libraries)
- **Concrete contracts**: Core, Pool, SafeERC20, ReentrancyGuard
- **Abstract contracts**: Any abstract contracts in source folders

### Excluded by Standard Library Check
- **OpenZeppelin**: IERC20 from lib/openzeppelin/ (is_standard_library_contract_name)
- **Forge-std**: Any contracts from lib/forge-std/
- **Solmate**: Any contracts from lib/solmate/

### Excluded by Path
- **Mocks**: Any contract in /mocks/ directory
- **Tests**: Any contract in /test/ directory

## Circular Import Example

```
src/
├── A.sol → imports B
└── B.sol → imports A
```

### Traversal with Cycle Detection
```
Level 0: main_contract = "A"
1. Get file: src/A.sol
2. visited_files = {src/A.sol}
3. Imports: {src/B.sol}
4. Extract: "B" → level_1_contracts

Level 1: "B"
1. Get file: src/B.sol
2. Check: src/B.sol in visited_files? NO
3. visited_files = {src/A.sol, src/B.sol}
4. Imports: {src/A.sol}
5. Extract: "A" → level_2_contracts

Level 2: "A"
1. Get file: src/A.sol
2. Check: src/A.sol in visited_files? YES ✅
3. SKIP (cycle detected)

Result: No infinite loop, both contracts discovered
```

## Integration with Existing Code

After generating `(contracts, contracts_with_depth, token_count)`, the existing code at **line 142** continues unchanged:

```rust
// Line 142: get collection of all inherited and called contracts
let mut contracts_with_parents = HashSet::new();

// Add parents of main contract (up to 1 level)
let parents_of_main = inheritance_map::get_parents(&main_contract, SolFileType::Standard, repo).await?;
// ... rest of existing logic unchanged
```

The fallback seamlessly replaces lines 85-140 while maintaining compatibility with lines 142-803.

## Key Insights

1. **Depth Semantics**: 
   - Level 0 = main contract (depth 0 in BFS)
   - Level 1 = direct imports (depth 1 in BFS)
   - Level 2 = transitive imports (depth 2 in BFS)
   - Level 3 = deep imports (depth 3 in BFS)

2. **contracts_with_depth**:
   - Contains depth 1-2 contracts (matches BFS behavior at line 105-107)
   - Used for import dependency analysis at line 226-237

3. **Exclusions**:
   - Interfaces and libraries excluded from contract sets
   - Standard libraries (OpenZeppelin, forge-std) excluded
   - Mocks and tests excluded
   - But their files are still tracked in visited_files to avoid re-processing

4. **Token Budget**:
   - Initial implementation: collect all contracts, let assembly phase handle budget
   - Future optimization: add early termination if needed

