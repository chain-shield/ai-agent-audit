use crate::{
    build_brain::slither_ffi::get_all_files_src, llm_review::phases::add_poc_findings::PocTest,
    prepare_code::git_clone::RepoPaths,
};
use regex::Regex;
use std::fs;

/// Extracts the Solidity version from source files in the repository
fn detect_solidity_version(repo: &RepoPaths) -> Option<String> {
    let pragma_regex = Regex::new(r"pragma\s+solidity\s+([^;]+);").ok()?;

    // Check source files for pragma statements
    for sol_file in &repo.sol_files {
        if let Ok(content) = fs::read_to_string(sol_file) {
            if let Some(captures) = pragma_regex.captures(&content) {
                if let Some(version) = captures.get(1) {
                    return Some(version.as_str().trim().to_string());
                }
            }
        }
    }

    None
}

pub fn generate_poc_prompt(filename: &str, repo: impl AsRef<RepoPaths>) -> anyhow::Result<String> {
    let repo = repo.as_ref();
    let code_root = repo.root.join(&repo.repo_name);
    let instructions = &repo.poc.instructions;
    let test_folder = repo.poc.test_folder.strip_prefix(&code_root)?;
    let filename = test_folder.join(filename);
    let file_location = filename.display();
    let poc_template = &repo.poc.template;
    let file_structure = get_all_files_src(repo)?;

    // Detect Solidity version from the codebase
    let solidity_version = detect_solidity_version(repo).unwrap_or_else(|| "^0.8.0".to_string());

    Ok(format!(
        r#"
Your task: write a fully runnable Proof of Concept (PoC) test that rigorously demonstrates the security vulnerability described in the finding report provided below.

## Deliverables
1. **poc_test_code**: Complete, runnable PoC test code (using the PoC template if provided)
2. **command_to_run_test**: Exact command to run the PoC test (e.g., `forge test --match-test testExploit -vvv`)
3. **commentary**: Brief explanation of your PoC approach, what it demonstrates, and any important notes (e.g., "This PoC demonstrates reentrancy by calling refund() recursively before state update")
4. **cannot_create_poc_because_finding_invalid**: (Optional) Set to `true` only if you determine the finding is actually invalid and a PoC cannot be created. If omitted, defaults to `false`.

## CRITICAL: Solidity Version Compatibility
**You MUST use the exact Solidity version from the protocol: `pragma solidity {solidity_version};`**
Do NOT use a different Solidity version, as this will cause compilation errors.

## Instructions for Creating PoC
{instructions}

## File Location
Your PoC test will be saved to: `{file_location}`

## Protocol Directory Structure
{file_structure}

## PoC Template (REQUIRED if provided)
{poc_template}

"#
    ))
}

pub fn generate_rewrite_poc_prompt(
    poc_test: &PocTest,
    repo: impl AsRef<RepoPaths>,
) -> anyhow::Result<String> {
    let repo = repo.as_ref();
    let instructions = &repo.poc.instructions;
    let file_location = poc_test.poc_test_file.display();
    let poc_template = &repo.poc.template;
    let failing_poc = &poc_test.poc_test_code;
    let test_output = &poc_test.poc_test_output;
    let command = &poc_test.poc_test_command;
    let file_structure = get_all_files_src(repo)?;

    // Detect Solidity version from the codebase
    let solidity_version = detect_solidity_version(repo).unwrap_or_else(|| "^0.8.0".to_string());

    Ok(format!(
        r#"
Your task: Fix the failing PoC test. The test either has compilation errors, runtime errors, or failing assertions (see test output below).

## Deliverables
1. **poc_test_code**: Revised, fully functional PoC test code
2. **command_to_run_test**: Command to run the PoC test (likely same as below, but update if needed)
3. **commentary**: Explain what was wrong with the previous PoC, what you fixed, and the current status (e.g., "Fixed import path issue - changed '../src/Contract.sol' to 'src/Contract.sol'. Test should now compile and pass.")
4. **cannot_create_poc_because_finding_invalid**: (Optional) Set to `true` ONLY if you determine the underlying security finding is actually invalid and no PoC can be created. If omitted, defaults to `false`.

## CRITICAL: Solidity Version Compatibility
**You MUST use the exact Solidity version from the protocol: `pragma solidity {solidity_version};`**
The error below may be caused by using the wrong Solidity version. Check the pragma statement first!

## Current PoC Code (Not Passing)
```solidity
{failing_poc}
```

## Test Output (Errors/Failures)
```
{test_output}
```

## Command Used to Run PoC
```bash
{command}
```

## Common Issues to Check:
1. **Solidity Version Mismatch**: MOST COMMON - Using wrong pragma version (must be `{solidity_version}`)
2. **Compilation Errors**: Missing imports, incorrect contract names, undefined variables
3. **Runtime Errors**: Incorrect addresses, missing setup, wrong function signatures
4. **Failing Assertions**: Exploit not working as expected, incorrect expected values
5. **Gas Issues**: Out of gas, need to adjust gas limits
6. **State Setup**: Missing initial state, incorrect balances, wrong permissions
7. **Path Issues**: Incorrect import paths, wrong contract references

## Instructions for Creating PoC
{instructions}

## File Location
`{file_location}`

## Protocol Directory Structure
{file_structure}

## PoC Template (REQUIRED if provided)
{poc_template}

### Debugging Tips:
- **FIRST**: Check if your pragma solidity version matches `{solidity_version}` - this is the most common error!
- Read the error message carefully and identify the root cause
- Check if all contract imports are correct and accessible
- Verify that all addresses and function calls match the actual protocol
- Ensure the exploit logic correctly demonstrates the vulnerability
- If the finding appears invalid after investigation, set `cannot_create_poc_because_finding_invalid: true`
"#
    ))
}
