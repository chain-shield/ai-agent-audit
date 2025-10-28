# IPriceOracle Inheritance Hierarchy Test

## Overview

This test verifies that the custom Slither inheritance runner (`run_printer_json_inheritance`) correctly captures the complete inheritance hierarchy for the Covenant protocol's `IPriceOracle` interface, including:

- ✅ Contracts in `lib/euler-price-oracle/` (project-specific library)
- ✅ Contracts in `src/` (main project code)
- ❌ Proper filtering of standard libraries (forge-std, openzeppelin, solady, etc.)

## Expected Inheritance Hierarchy

```
IPriceOracle (interface)
├─ BaseAdapter (lib/euler-price-oracle/src/adapter/BaseAdapter.sol) [ABSTRACT]
│  ├─ ChainlinkOracle (lib/euler-price-oracle/src/adapter/chainlink/ChainlinkOracle.sol) [EULER]
│  │  └─ ChainlinkOracle (src/curators/oracles/chainlink/ChainlinkOracle.sol) [COVENANT]
│  │
│  └─ PythOracle (lib/euler-price-oracle/src/adapter/pyth/PythOracle.sol) [EULER]
│     └─ PythOracle (src/curators/oracles/pyth/PythOracle.sol) [COVENANT]
│
├─ CovenantCurator (src/curators/CovenantCurator.sol) [DIRECT]
│
└─ EulerRouter (lib/euler-price-oracle/src/EulerRouter.sol) [EULER - OOS]
```

## Prerequisites

### 1. Repository Location

The test expects the Covenant repository to be located at:
```
/private/tmp/audit-analysis/2025-10-covenant-d5ebe4/2025-10-covenant
```

This is the Docker volume mount point used during audits.

### 2. Docker Setup

Ensure Docker is running and the Slither image is available:
```bash
docker pull ghcr.io/trailofbits/eth-security-toolbox:nightly
```

### 3. Repository Build

The repository should be built before running the test:
```bash
cd /private/tmp/audit-analysis/2025-10-covenant-d5ebe4/2025-10-covenant
forge build
```

## Running the Test

### Basic Run (with output)
```bash
cargo test --test inheritance_iprice_oracle_test -- --ignored --nocapture
```

### Run with detailed logging
```bash
RUST_LOG=info cargo test --test inheritance_iprice_oracle_test -- --ignored --nocapture
```

### Run with debug logging (shows Slither commands)
```bash
RUST_LOG=debug cargo test --test inheritance_iprice_oracle_test -- --ignored --nocapture
```

## Test Output

The test will output:

### 1. All Inheritance Edges
```
=== All Inheritance Edges (child → parent) ===
  BaseAdapter → IPriceOracle
  ChainlinkOracle → BaseAdapter
  ChainlinkOracle → BaseAdapter
  PythOracle → BaseAdapter
  PythOracle → BaseAdapter
  CovenantCurator → IPriceOracle
  EulerRouter → IPriceOracle
```

### 2. Inheritance Tree
```
=== IPriceOracle Inheritance Tree ===
├─ IPriceOracle
  ├─ BaseAdapter
    ├─ ChainlinkOracle
    ├─ ChainlinkOracle
    ├─ PythOracle
    ├─ PythOracle
  ├─ CovenantCurator
  ├─ EulerRouter
```

### 3. Verification Results
```
=== Verification: Direct Children of IPriceOracle ===
  ✓ BaseAdapter (expected)
  ✓ CovenantCurator (expected)
  ✓ EulerRouter (expected)

=== Verification: Children of BaseAdapter ===
  ✓ ChainlinkOracle (expected)
  ✓ PythOracle (expected)

=== Verification: Euler Price Oracle Contracts Included ===
  ✓ BaseAdapter (from lib/euler-price-oracle)
  ✓ EulerRouter (from lib/euler-price-oracle)

=== Verification: Standard Libraries Excluded ===
  ✓ Test (should be excluded)
  ✓ Script (should be excluded)
  ✓ Ownable (should be excluded)
  ✓ ERC20 (should be excluded)
  ✓ DSTest (should be excluded)
```

### 4. Summary Statistics
```
=== Summary Statistics ===
  Total inheritance edges: 7
  Total parent contracts: 3
  IPriceOracle direct children: 3
  BaseAdapter direct children: 4
```

## What the Test Verifies

### ✅ Positive Assertions

1. **IPriceOracle has children** - The interface should have at least one implementing contract
2. **Direct children are found** - BaseAdapter, CovenantCurator, EulerRouter
3. **BaseAdapter has children** - ChainlinkOracle and PythOracle (both Euler and Covenant versions)
4. **Euler contracts included** - Contracts from `lib/euler-price-oracle/` are captured
5. **Inheritance map is cached** - The global cache is populated correctly

### ❌ Negative Assertions

1. **Standard libraries excluded** - forge-std, openzeppelin, solady contracts are NOT included
2. **Testing frameworks excluded** - ds-test, prb-test, etc. are NOT included

## Troubleshooting

### Test fails with "Repository not found"

**Problem**: The repository path doesn't exist.

**Solution**: Ensure the Covenant repo is cloned to the correct location:
```bash
ls -la /private/tmp/audit-analysis/2025-10-covenant-d5ebe4/2025-10-covenant
```

### Test fails with "Slither failed"

**Problem**: Slither can't analyze the repository.

**Solution**: 
1. Ensure Docker is running
2. Build the repository first: `cd /path/to/repo && forge build`
3. Check Slither can run manually:
```bash
docker run --rm -v /private/tmp/audit-analysis/2025-10-covenant-d5ebe4:/workspace \
  -w /workspace \
  ghcr.io/trailofbits/eth-security-toolbox:nightly \
  slither --print inheritance --json - \
  --filter-paths "lib/forge-std|lib/openzeppelin-contracts|lib/solady" \
  2025-10-covenant
```

### IPriceOracle has no children

**Problem**: The inheritance analysis didn't find any children for IPriceOracle.

**Solution**:
1. Check if the contracts actually inherit from IPriceOracle
2. Verify the filter-paths aren't excluding project code
3. Check the Slither output manually (see above command)

### Standard library contracts are included

**Problem**: Contracts like `Test`, `Ownable`, etc. appear in the inheritance edges.

**Solution**: This is a warning, not a failure. The test will note it but continue. If this happens:
1. Verify the `--filter-paths` argument is correct
2. Check if the project has custom contracts with these names (not from standard libs)

## Implementation Details

### Custom Slither Runner

The test uses `run_printer_json_inheritance()` which:

1. Builds standard Slither args via `build_slither_args()`
2. Adds `--filter-paths` flag with exclusion patterns:
   ```
   lib/forge-std|
   lib/openzeppelin-contracts|
   lib/solady|
   lib/ds-test|
   lib/erc4626-tests|
   lib/halmos-cheatcodes|
   lib/solmate|
   lib/prb-test|
   node_modules
   ```
3. Overrides any `slither.config.json` exclusions
4. Caches results with a `_filtered` suffix

### API Functions Used

- `generate_inheritance_edges(repo)` - Returns all (child, parent) tuples
- `get_children(contract, repo)` - Returns direct children of a contract
- `get_inverted_inheritance_map(repo)` - Returns parent → children map

### Caching

The test uses the global inheritance cache, so:
- First run will execute Slither (slow)
- Subsequent runs use cached data (fast)
- Cache key includes `_filtered` suffix to avoid conflicts

## Related Files

- **Implementation**: `src/build_brain/slither_ffi.rs` - `run_printer_json_inheritance()`
- **Call Graph**: `src/build_brain/callgraph.rs` - `generate_inheritance_edges()`
- **Parsing**: `src/build_brain/inheritance.rs` - `parse_inheritance_json()`
- **Test**: `tests/inheritance_iprice_oracle_test.rs`

## Success Criteria

The test passes if:

1. ✅ Repository exists and is accessible
2. ✅ Slither runs successfully
3. ✅ IPriceOracle has at least one child
4. ✅ All expected direct children are found (BaseAdapter, CovenantCurator, EulerRouter)
5. ✅ BaseAdapter has children (ChainlinkOracle, PythOracle)
6. ✅ Euler contracts are included in the edges
7. ✅ No standard library contracts are included (warning only)

## Notes

- The test is marked with `#[ignore]` to prevent it from running in CI/CD
- Run explicitly with `--ignored` flag
- Use `--nocapture` to see detailed output
- The test is async and uses `tokio::test`
- Logging is initialized with `env_logger` for debugging

