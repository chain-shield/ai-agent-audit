# Plan: Remove Deprecated In-Process PoC Generation

Status: Approved and implemented

## Problem

The main Rust audit pipeline still contains legacy PoC-generation plumbing even though PoC creation and verification have moved to the `validation-three-shot` workflow.

Current failure example:

```text
valid test folder - however PoC instructions missing!
```

Root cause:

- `clone_and_filter_git_repo()` defaults to `<repo>/test`.
- If that folder exists and no PoC instructions are configured, repo prep panics.
- The actual in-process PoC generation phase is disabled behind `CREATE_TESTS = false`, so this validation is obsolete and now breaks normal runs.

## Scope

Remove the deprecated **in-process PoC test generation feature** from the main Rust audit pipeline.

Keep:

- `validation-three-shot` R5/R6 PoC generation and verification.
- Textual `proof_of_concept` fields used by findings and reports.
- Textual `proof_of_code` fields. These remain intentional because requiring the discovery agent to sketch even a non-runnable proof in code improves valid finding output.
- Validation prompts that ask whether a minimal reproducible PoC is conceptually possible.
- Three-shot config fields under `validation-three-shot/config.yaml`.

Remove:

- Main CLI/YAML PoC config fields.
- Repo prep PoC instruction/template/test-folder loading.
- `RepoPaths.poc`.
- `CREATE_TESTS`.
- `code_review_v2` Phase 6 in-process PoC generation.
- `code_review_v2` Phase 7 professional report generation if it depends on in-process passing PoCs.
- Legacy modules that only support in-process PoC file writing/running.
- Tests whose only purpose is legacy in-process PoC generation.

## Detailed Removal Plan

### 1. Remove CLI/YAML PoC Surface

Files:

- `src/cli_args/parse.rs`
- `examples/audit-config.example.yaml`
- `puppy.yaml`
- `README.md`

Changes:

- Delete `poc_instructions`, `poc_template`, and `test_folder` from `Cli`.
- Delete YAML merge logic for those fields.
- Remove commented PoC config examples from example YAMLs.
- Update README configuration table and path convention docs.
- Replace the README note "automatic PoC generation is disabled" with "PoC generation now lives in validation-three-shot."

Expected outcome:

- User configs no longer expose dead settings.
- Existing repos with `test/` folders no longer require unrelated PoC instructions.

### 2. Remove Repo Prep PoC Loading And Panic

File:

- `src/prepare_code/git_clone.rs`

Changes:

- Delete `PocConfig`.
- Delete `RepoPaths.poc`.
- Delete PoC instruction/template file reads.
- Delete test-folder defaulting and validation.
- Delete the panic path:

```rust
panic!("valid test folder - however PoC instructions missing!");
```

Expected outcome:

- `clone_and_filter_git_repo()` only prepares audit context, code files, docs, source folders, tests/scripts/config lists, scope, and repo metadata.
- Presence of a repo `test/` folder is harmless.

### 3. Remove Main Pipeline Phase 6 And Phase 7

File:

- `src/llm_review/analysis/code_review_v2.rs`

Changes:

- Remove the whole Phase 6 block that calls `phases::add_poc_findings::execute`.
- Remove `CREATE_TESTS` import.
- Remove the Phase 7 block that calls `phases::create_report::execute`, unless we explicitly repurpose it to create reports for valid findings without requiring `PocStatus::AllTestPass`.

Recommendation:

- Delete Phase 7 for now because three-shot owns competition-ready PoC/report generation.
- Keep final audit report generation through `reporting/audit.rs`.

Expected outcome:

- Main audit pipeline stops pretending it can create/run PoC tests.
- Three-shot remains the source of truth for runnable PoCs and C4-ready reports.

### 4. Delete Legacy In-Process PoC Modules

Files likely removable:

- `src/llm_review/phases/add_poc_findings.rs`
- `src/llm_review/phases/create_report.rs`
- `src/llm_review/prompt_support/make_poc_prompt.rs`
- `src/llm_review/prompt_support/pre_poc.rs`
- `src/llm_review/prompt_support/post_poc.rs`
- `src/llm_review/prompt_support/create_report_prompt.rs` if only used by deleted `create_report.rs`
- `src/llm_review/utils/save_run_poc.rs`

Module exports:

- Remove matching exports from `src/lib.rs`.

Expected outcome:

- No dead code around generated test files, command construction, retrying failed PoCs, or in-process report generation.

### 5. Clean Finding Model Carefully

Files:

- `src/llm_review/findings/findings.rs`
- `src/llm_review/utils/prompt_context.rs`
- `src/llm_review/analysis/analysis_db.rs`
- `src/reporting/audit.rs`
- `src/reporting/competition_reports.rs`

Recommendation:

- Keep `proof_of_concept`.
- Consider keeping `proof_of_code` for now because current discovery JSON schemas and tests still emit it, even if we stop trying to compile/run it.
- Remove `poc_test_file`, `poc_test_command`, and `poc_test_status`.
- Remove `competition_report` only if `reporting/competition_reports.rs` is also removed or rewritten.

Why:

- `proof_of_concept` is still useful in normal reports.
- `proof_of_code` may still be useful as a textual snippet generated by discovery, but it should no longer mean "saved and executed test."
- Test-run metadata belongs to three-shot artifacts, not the core finding struct.

Report cleanup:

- Remove `CREATE_TESTS` gates from `reporting/audit.rs` and `prompt_context.rs`.
- Remove "PoC Test Status" and "Command to Run Test" from enhanced reports.
- Keep "Proof of Concept".
- Keep or rename "Proof of Code" depending on whether discovery still emits code snippets.

### 6. Remove `CREATE_TESTS`

Files:

- `src/config.rs`
- Any imports in reporting/prompt utils/code review

Changes:

- Delete `pub const CREATE_TESTS: bool = false`.
- Delete conditional report output based on `CREATE_TESTS`.

Expected outcome:

- No compile-time flag for a removed feature.

### 7. Update Tests

Remove tests that only validate legacy in-process PoC generation:

- `tests/poc_workflow_integration_test.rs`
- `tests/report_generation_integration_test.rs` if `create_report.rs` is deleted.

Update tests that construct `RepoPaths`:

- Remove `poc: PocConfig::default()` from many fixtures.
- Remove imports of `PocConfig`.

Update tests that construct `Finding`:

- Remove `poc_test_file`, `poc_test_command`, `poc_test_status`.
- Keep `proof_of_concept`.
- Keep `proof_of_code` initially unless we decide to remove that field too.

Likely impacted tests include:

- `tests/deployment_scripts_detection_test.rs`
- `tests/generate_prompts_test.rs`
- `tests/integration_all_round.rs`
- `tests/integration_validation_round.rs`
- `tests/validation_round_unit_test.rs`
- `tests/library_file_detection_test.rs`
- `tests/detect_source_code_dependencies_test.rs`
- `tests/codeblock_generation_integration_test.rs`
- `tests/inheritance_solidity_parsing_test.rs`
- `tests/codeblock_category_diagnostic.rs`
- `tests/summarize_db_unit_test.rs`
- `tests/path_canonicalization_test.rs`
- `tests/remapping_integration_test.rs`
- `tests/parse_import_dependencies_test.rs`
- `tests/codeblock_db_unit_test.rs`
- `tests/file_summarization_integration_test.rs`

### 8. Verify

Run:

```bash
cargo fmt
cargo check
cargo test
```

Also run the specific repro path that currently panics:

```bash
cargo run --release -- --config <legion-config.yaml>
```

Acceptance criteria:

- No panic when a cloned repo has a `test/` folder and no PoC instructions.
- `rg "poc_instructions|poc_template|test_folder|CREATE_TESTS|add_poc_findings|save_run_poc|make_poc_prompt"` returns no matches in `src/`, except for intentional three-shot or docs references.
- Main audit report still includes textual proof-of-concept content.
- `validation-three-shot` PoC workflow remains untouched.
- `cargo check` passes.
- Relevant tests pass or are intentionally removed/updated.

## Open Decisions

1. Should `proof_of_code` remain as a textual field, or should it be removed/renamed now?
2. Should `src/reporting/competition_reports.rs` be deleted because three-shot owns C4-ready reports?
3. Should `main.rs` continue returning early after repo prep, or is that temporary debug code unrelated to this cleanup?

## Recommended Decision

Approved cleanup boundary:

- Remove all in-process PoC file generation, test execution, and PoC config.
- Keep textual `proof_of_concept`.
- Keep textual `proof_of_code` as a discovery forcing function, not as a saved/runnable test contract.
- Leave `validation-three-shot` completely untouched.
