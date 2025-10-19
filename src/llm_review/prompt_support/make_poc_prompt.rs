use crate::{
    build_brain::slither_ffi::get_all_files_src, llm_review::phases::add_poc_findings::PocTest,
    prepare_code::git_clone::RepoPaths,
};

pub fn generate_poc_prompt(filename: &str, repo: impl AsRef<RepoPaths>) -> anyhow::Result<String> {
    let repo = repo.as_ref();
    let code_root = repo.root.join(&repo.repo_name);
    let instructions = &repo.poc.instructions;
    let test_folder = repo.poc.test_folder.strip_prefix(&code_root)?;
    let filename = test_folder.join(filename);
    let file_location = filename.display();
    let poc_template = &repo.poc.template;
    let file_structure = get_all_files_src(repo)?;

    Ok(format!(
        r#"
Your task: write runnable PoC that rigorously demonstrates security finding below.

## Deliverables 
1. Runnable PoC test (using PoC template if provided)
2. Command to run PoC test (i.e. forge test ...)

## Follow These instructions
{instructions}

##  Where PoC will be saved
{file_location}

## Main Protocol Directory & File Structure
{file_structure}

## PoC Template to use to create tests (if provided, you are REQUIRED to use)
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

    Ok(format!(
        r#"
Your task: fix PoC - it has either has failing tests, or errored out (see test output below).

## Deliverables 
1. poc_test_code: Revised PoC test (using PoC template if provided)
2. command_to_run_test :Command to run PoC test, likely same as command provided below, if not provide correct command (i.e. forge test ...)
3. cannot_create_poc_because_finding_invalid: (Optional Field) If cannot create PoC because you discover that finding is invalid, return true. If field not proivided, assumed false 

## PoC test that is Not Passing (or Erroring Out) 
{failing_poc}

## Test Output 
{test_output}

## Command Used to Run PoC
{command}

## Instruction to Creating PoC
{instructions}

##  Where PoC is saved
{file_location}

## Main Protocol Directory & File Structure
{file_structure}

## PoC Template to use to create tests (if provided, you are REQUIRED to use)
{poc_template}

"#
    ))
}
