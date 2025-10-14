# Deployment Scripts Feature - Documentation

## Overview

The deployment scripts feature automatically detects and includes relevant deployment/setup scripts in the codeblock context sent to the LLM for security analysis. This helps the AI understand how contracts are deployed and initialized, which is crucial for identifying deployment-related vulnerabilities.

## Implementation

### Location
- **Main Integration**: `src/enumerator/codeblocks.rs` (lines 285-329)
- **Detection Logic**: `src/enumerator/parse_solidity.rs` (lines 471-517)
- **Helper Function**: `src/utils/contract_name_check.rs` (lines 24-33)

### How It Works

1. **Script Detection** (`detect_scripts_connected_to_contract`)
   - Scans all files in `repo.script_files` (populated during repository preparation)
   - Reads each script file and strips comments/strings to avoid false positives
   - Uses word-boundary regex matching to find exact contract name references
   - Returns a `HashSet<PathBuf>` of matching script files
   - **Caching**: Results are cached per repo+contract to avoid redundant file I/O

2. **Integration into Codeblocks**
   - After adding supporting contracts and interfaces
   - Before final token count verification
   - Respects the token budget (150,000 tokens by default)
   - Logs which scripts are added or skipped

3. **Comment/String Stripping** (`strip_comments_and_strings`)
   - Removes block comments (`/* ... */`)
   - Removes line comments (`// ...`)
   - Removes double-quoted strings (`"..."`)
   - Removes single-quoted strings (`'...'`)
   - Prevents false positives from contract names in comments or strings

4. **Contract Reference Detection** (`contains_contract_reference`)
   - Uses word-boundary regex: `\b{ContractName}\b`
   - Ensures exact matches (e.g., "MyToken" won't match "MyTokenFactory")
   - Case-sensitive matching

### Code Flow

```
build_codeblock_for_contract()
  ↓
[Add main contract]
  ↓
[Add supporting contracts/interfaces]
  ↓
[Add deployment scripts] ← NEW FEATURE
  ↓
  detect_scripts_connected_to_contract(contract, repo)
    ↓
    Check cache
    ↓
    For each script in repo.script_files:
      ↓
      Read file content
      ↓
      Strip comments and strings
      ↓
      Check if contains_contract_reference(contract, content)
      ↓
      If yes, add to results
    ↓
    Cache results
    ↓
  Return HashSet<PathBuf>
  ↓
  For each script:
    ↓
    Check token budget
    ↓
    If within budget: add to codeblock
    ↓
    If exceeds budget: skip and log
  ↓
[Final token count verification]
```

## Test Coverage

### Test File
`tests/deployment_scripts_detection_test.rs`

### Test Cases (10 total, all passing ✅)

1. **`test_detect_scripts_with_direct_contract_reference`**
   - Tests basic detection of scripts that import and use a contract
   - Verifies only the correct script is detected (not unrelated scripts)

2. **`test_detect_scripts_with_multiple_references`**
   - Tests detection when multiple scripts reference the same contract
   - Verifies all matching scripts are found

3. **`test_detect_scripts_no_matches`**
   - Tests that no scripts are returned when contract isn't referenced
   - Ensures false negatives don't occur

4. **`test_detect_scripts_ignores_comments`**
   - Tests that contract names in comments are ignored
   - Covers both line comments (`//`) and block comments (`/* */`)

5. **`test_detect_scripts_ignores_string_literals`**
   - Tests that contract names in string literals are ignored
   - Prevents false positives from strings like `"MyToken Deployer"`

6. **`test_detect_scripts_word_boundary_matching`**
   - Tests that exact word matching works correctly
   - Ensures "MyToken" doesn't match "MyTokenFactory"

7. **`test_detect_scripts_caching`**
   - Tests that results are cached correctly
   - Verifies subsequent calls return the same results

8. **`test_detect_scripts_empty_script_list`**
   - Tests behavior when no script files exist
   - Ensures graceful handling of empty input

9. **`test_detect_scripts_with_new_keyword`**
   - Tests detection when contract is instantiated with `new`
   - Example: `MyToken token = new MyToken();`

10. **`test_detect_scripts_with_type_casting`**
    - Tests detection when contract is used in type casting
    - Example: `MyToken token = MyToken(0x123...);`

### Test Helper

**`create_test_repo_with_scripts()`**
- Creates temporary directory with test script files
- Uses nanosecond timestamps to ensure unique repo names/hashes
- Prevents cache collisions between parallel tests
- Returns `(TempDir, RepoPaths)` tuple

## Usage Example

When analyzing a contract like `MyToken`, the system will:

1. Find all scripts in the repository
2. Check each script for references to "MyToken"
3. Add matching scripts to the codeblock (within token budget)

**Example Script Detection:**

```solidity
// script/Deploy.s.sol
import {MyToken} from "../src/MyToken.sol";

contract DeployScript {
    function run() external {
        MyToken token = new MyToken();  // ← "MyToken" detected here
        token.initialize();
    }
}
```

This script would be included in the codeblock for `MyToken` analysis.

## Benefits

1. **Better Context**: LLM sees how contracts are deployed and initialized
2. **Deployment Vulnerabilities**: Can detect issues like:
   - Missing initialization calls
   - Incorrect constructor parameters
   - Deployment order dependencies
   - Access control setup issues
3. **Token Efficient**: Only includes relevant scripts (not all scripts)
4. **Cached**: Fast performance through intelligent caching

## Configuration

- **Token Budget**: Controlled by `TOKEN_BUDGET` constant (default: 150,000)
- **Script Detection**: Automatic based on file paths in `repo.script_files`
- **Caching**: Automatic per-repo-per-contract basis

## Logging

The feature provides detailed logging:

```
✅ Adding 'script: script/Deploy.s.sol' (1234 tokens) - total: 50000/150000 tokens
⏭️ Skipping 'script: Setup.s.sol' (5000 tokens) - would exceed budget (155000/150000 tokens)
```

## Future Enhancements

Potential improvements:
- Prioritize scripts by relevance (e.g., scripts that deploy vs. scripts that interact)
- Support for multi-contract deployment scripts
- Detection of deployment dependencies between contracts
- Analysis of deployment script security issues

## Related Files

- `src/prepare_code/git_clone.rs` - Populates `repo.script_files` during repo preparation
- `src/utils/check_folder_name.rs` - Identifies script files by folder name
- `src/enumerator/codeblock_cache.rs` - General codeblock caching infrastructure

