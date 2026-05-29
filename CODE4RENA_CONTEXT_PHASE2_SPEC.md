# Code4rena Contest Context Auto-Generation Spec

## Product Goal

Add first-class Code4rena contest context extraction so the app can accept a Code4rena contest GitHub URL and automatically generate the context artifacts it needs for audit discovery: contest scope, protocol docs, known issues, V12 exclusions, trusted roles, invariants, and in-code documentation.

## Target User

- Primary: internal auditor/researcher running controlled Code4rena benchmark experiments.
- Secondary: developer maintaining benchmark automation and comparing future algorithm/config changes against archived ground truth.

## Supported Platforms

- Local macOS development environment used by this repository.
- Rust CLI execution via `cargo run --release`.
- Existing local audit workspace under `~/Desktop/Audit` unless overridden.
- Public GitHub/Code4rena/protocol documentation URLs reachable from the runner.
- No browser session should be required for this feature.

## Primary Workflow

1. User supplies a Code4rena contest GitHub repo, or a source repo plus a Code4rena contest repo.
2. App prepares the repository as today.
3. App automatically generates the three context artifacts required by discovery:
   - `<protocol>-docs.md`
   - `<protocol>-scope.md`
   - `<protocol>-scope.txt`
4. App preserves user-supplied context files. If `custom_doc`, `audit_scope`, or `scoped_files` are provided, those exact files win for that artifact. Missing artifacts may still be generated.
5. Discovery uses docs/scope context without PoC submission requirements.
6. Post-discovery validation can use a validation-only sidecar containing PoC/test requirements.
7. Benchmark telemetry records repo identity, scope/docs paths, code folders, run id, and relevant config constants.

## Supported Inputs

- `repo`: normal source repo or Code4rena contest wrapper repo.
- `audit_type: Code4rena`.
- New optional field: `code4rena_contest_repo`.
- New optional field: `code4rena_contest_url`.
- Existing manual overrides:
  - `custom_doc`
  - `audit_scope`
  - `scoped_files`
  - `context.files`
  - `context.urls`

`code4rena_contest_repo` and `code4rena_contest_url` are aliases for the contest context source. At most one should be set.

## Core Features

### C4 Contest Context Detection

- If `audit_type == Code4rena` and `repo` matches `https://github.com/code-423n4/<slug>`, treat `repo` as the contest context source.
- If `code4rena_contest_repo` is supplied, use it as contest context while analyzing `repo`.
- If neither is true, retain the current generic generated-context behavior.

### Required Generated Artifacts

For artifact prefix `<protocol>`:

- `<protocol>-scope.txt`: exact in-scope Solidity files.
- `<protocol>-scope.md`: audit scope, exclusions, known issues, V12 known issues, trusted roles, invariants, areas of concern.
- `<protocol>-docs.md`: protocol documentation, architecture, mechanics, flows, external integrations, and scoped-code documentation.
- `<protocol>-validation.md`: validation-only sidecar for PoC/test requirements and contest submission mechanics. This must not be injected into discovery context.
- `<protocol>-context-sources.json`: provenance for every source used or skipped.

Manual override behavior:

- If `custom_doc` is supplied, do not overwrite or replace it with generated docs.
- If `audit_scope` is supplied, do not overwrite or replace it with generated scope markdown.
- If `scoped_files` is supplied, do not overwrite or replace it with generated scope text.
- If only one or two manual artifacts are supplied, generate only the missing artifacts.
- Generated artifacts should live under the configured `context.output_dir`.

### Scope Extraction

Preference order:

1. Contest `scope.txt` if present.
2. README `Files in scope` table.
3. README scope section containing direct `.sol` links or paths.
4. Fail closed if explicit C4 scope exists but cannot map to local files.
5. Generic fallback to configured `code_folders` is allowed only when no explicit C4 scope exists.

Scope parser requirements:

- Accept one-line space-separated `scope.txt`.
- Accept newline-separated `scope.txt`.
- Normalize leading `./`.
- Preserve paths relative to the analyzed protocol root.
- For wrapper repos that include source code directly, paths map as-is.
- For source repo plus contest repo, map paths by suffix against local Solidity files.
- Emit warnings for in-scope entries that do not exist locally.
- If zero entries map from an explicit C4 scope source, fail before discovery.

### Documentation Extraction

The collector must use both contest-provided docs and local repo docs.

Contest README sources:

- Overview
- Links section
- Previous audits
- Documentation links
- Website links only when they point to documentation or developer docs
- Publicly known issues
- Areas of concern
- Main invariants
- Trusted roles
- V12 links

Protocol docs linked from README:

- GitHub markdown/blob/raw files.
- GitHub `docs/` tree links, with bounded traversal of text-like files.
- GitBook/Docusaurus/Mintlify style docs sites when they expose readable HTML.
- `llms.txt`, `sitemap.xml`, or known docs navigation pages when available.
- PDFs when they are protocol specs or prior audits, with text extraction and provenance.

Repo-local docs:

- Root `README.md`.
- Other root-level Markdown/text files that are likely protocol, scope, architecture, audit, security, risk, invariant, setup, or design documentation.
- `docs/**/*.md`, `docs/**/*.mdx`, `docs/**/*.txt`.
- `SECURITY.md`.
- `audits/**/*.md`, `audits/**/*.txt`.
- Known issue files and prior audit notes.

Root-level Markdown/text filtering:

- Include filenames such as `ARCHITECTURE.md`, `SECURITY.md`, `INVARIANTS.md`, `KNOWN_ISSUES.md`, `AUDIT*.md`, `DESIGN*.md`, `SPEC*.md`, `SCOPE*.md`, `RISK*.md`, `PROTOCOL*.md`, `CONTRACTS*.md`, `DEPLOY*.md`, and `README*.md`.
- Include root docs referenced from the contest README even when their names do not match the keyword list.
- Exclude obvious changelogs, license files, contribution guides, package manager notes, generated reports, and dependency/vendor documentation unless the contest README links to them as audit-relevant.

Scoped-code documentation:

- For every file in `<protocol>-scope.txt`, extract:
  - file-level comments,
  - contract/interface/library NatSpec,
  - public/external function NatSpec,
  - state variable comments where useful,
  - event/error comments,
  - inheritance and imported local dependencies.
- Summarize this into `<protocol>-docs.md`; preserve raw source references in `<protocol>-context-sources.json`.

Fetch constraints:

- No broad web crawling.
- Contest README links to user/developer/protocol documentation are required candidate inputs for generated `docs.md`.
- Follow first-level README links and bounded documentation navigation.
- Linked docs do not recursively fan out except through a recognized docs navigation source.
- Enforce token and fetch budgets.
- Every skipped link must be recorded with a reason.

### V12 Findings

- Detect V12 links in Code4rena README.
- Fetch markdown V12 findings when present.
- Add them to `<protocol>-scope.md` as known issues / out-of-scope exclusions.
- Preserve:
  - title,
  - severity if present,
  - affected file/function if present,
  - source URL,
  - short summary.
- If no V12 links are present, record `No V12 findings detected` in context provenance.
- V12 context is Code4rena competition-only and must not be used for Code4rena bounty or Immunefi bounty modes.

### PoC Requirements

- Do not include PoC submission requirements in discovery prompts or discovery docs.
- Preserve PoC/test requirements in `<protocol>-validation.md` for post-discovery validation and three-shot validation.
- Existing `poc_instructions`, `poc_template`, and `test_folder` config fields continue to work.

### Telemetry Config Snapshot

Benchmark `run_manifest.json` must include:

- `R1_RUNS`
- `R2_RUNS`
- `INVARIANT_RUNS`
- `ACTOR_RUNS`
- `PATTERN_DISCOVERY_RUNS`
- `INVARIANT_DISCOVERY_RUNS`
- `ACTOR_DISCOVERY_RUNS`
- `MAX_PATTERN_RUN_TOP`
- `MAX_PATTERN_RUN_RARE`
- `MAX_PATTERN_RUN_MOST`
- `MAX_PATTERN_RUN_FREQUENT`
- `MAX_PATTERN_RELEVANT_FREQUENT`
- `MAX_PATTERN_LIBRARY`
- `MAX_PATTERN_NICHE`
- `MAX_PATTERN_GENERAL`
- `MAX_DEPTH`
- `TOKEN_BUDGET`
- `SKIP_LIBRARIES`
- `SKIP_INVARIANT_RUNS`
- `SKIP_ACTOR_PATTERN_RUNS`
- `OPENAI_MODEL`
- `OPENAI_REASONING_EFFORT`
- `OPENAI_DEDUP_MODEL`
- `OPENAI_DEDUP_REASONING_EFFORT`
- `DISCOVERY_PROVIDER`
- provider-specific discovery model settings

## Non-Goals

- Do not run Megapot discovery as part of this implementation job.
- Do not change finding-generation prompts except to route improved context.
- Do not include PoC rules in discovery context.
- Do not scrape authenticated C4 submissions during app run; benchmark ground truth already lives in `benchmarks/code4rena-corpus`.
- Do not broaden audit scope beyond C4-declared scope.
- Do not silently audit all files if explicit C4 scope fails to map.

## Data Flow

```text
CLI/YAML
  -> Repo preparation
  -> Code4rena contest context resolver
  -> README/scope/out-of-scope/V12/docs collectors
  -> Scope mapper against local repo files
  -> Scoped-code documentation extractor
  -> Generated artifacts
       - docs.md
       - scope.md
       - scope.txt
       - validation.md
       - context-sources.json
  -> RepoPaths
  -> Discovery
  -> Verification/validation
  -> Report and benchmark telemetry
```

## Privacy And Security Expectations

- Only fetch public Code4rena/GitHub/protocol documentation URLs.
- Do not send private files to external fetchers.
- Record every remote source used.
- Fail closed on scope ambiguity.
- Do not trust protocol docs over source code when validating exploitability; docs define intent, code defines behavior.

## Error States

- Contest README missing: warn and fall back to generic context only if user supplied manual scope/docs; otherwise fail for C4 auto-context.
- Scope file/table found but zero local files map: fail.
- Some scope paths missing: warn and continue only if at least one scoped file maps.
- V12 link fetch failure: warn and record failed source; do not fail discovery.
- Docs link fetch failure: warn and record failed source.
- PDF extraction failure: warn and record failed source.
- Generated docs/scope exceed token budget: summarize; never silently truncate.
- User-supplied artifact path missing: fail with actionable error.

## Performance Expectations

- Context collection should complete before expensive discovery.
- Remote docs fetches should be bounded by URL count, byte size, and token budget.
- Scope extraction and local documentation scanning should be deterministic and cheap compared with Slither/LLM review.
- Context generation should avoid extra LLM use where deterministic extraction is sufficient.

## UX And Accessibility Expectations

- CLI logs must clearly state which context artifacts were generated, reused, or supplied by the user.
- Failures must name the missing or unmapped file and the source that declared it.
- Generated Markdown must use normal headings, tables, and bullet lists so it is readable in terminals, editors, and rendered Markdown views.
- `context-sources.json` must be human-inspectable enough to debug why a doc, V12 link, or scope source was included or skipped.
- No hidden browser login or manual website step should be required for context generation.

## Implementation Milestones

### Milestone 1: Config And Telemetry

- Add `code4rena_contest_repo` / `code4rena_contest_url` to CLI/YAML config.
- Add config snapshot object to benchmark `run_manifest.json`.
- Preserve existing configs without migration.

### Milestone 2: Deterministic Contest Collector

- Implement C4 contest README/scope/out-of-scope collector.
- Add V12 link detection and fetch.
- Add README scope-table extraction.
- Add exact scope mapping and fail-closed behavior.

### Milestone 3: Documentation Collector

- Add bounded GitHub docs tree traversal.
- Add docs-site fetch support.
- Add PDF text extraction path or explicitly gated fallback with provenance.
- Add repo-local docs discovery, including relevant root-level Markdown/text beyond `README.md`.
- Add scoped-code documentation extraction.

### Milestone 4: Artifact Rendering

- Render `docs.md`, `scope.md`, `scope.txt`, `validation.md`, and `context-sources.json`.
- Ensure PoC requirements appear only in `validation.md`.
- Respect manual overrides per artifact.

### Milestone 5: Tests And Integration

- Unit tests for scope parsing and path mapping.
- Fixture tests for Megapot, Olas, Intuition, Monetrix, and Panoptic.
- Integration test for generated Megapot artifacts from fixtures.
- Final local validation with lint, unit tests, targeted integration tests, and fixture-based context generation checks.

## Validation Contract

### Functional Assertions

- `ASSERT-C4-DETECT-001`: When `audit_type=Code4rena` and `repo` is `github.com/code-423n4/<slug>`, the app treats the repo as a Code4rena contest context source.
- `ASSERT-C4-DETECT-002`: When `code4rena_contest_repo` is set, the app uses it for contest context while analyzing the configured `repo`.
- `ASSERT-C4-SCOPE-001`: Megapot `scope.txt` is parsed into 16 in-scope entries.
- `ASSERT-C4-SCOPE-002`: Intuition one-line `scope.txt` is parsed into all declared entries, including periphery paths.
- `ASSERT-C4-SCOPE-003`: Monetrix one-line `scope.txt` is parsed into all declared entries.
- `ASSERT-C4-SCOPE-004`: Olas README scope table is parsed when no local `scope.txt` is available.
- `ASSERT-C4-SCOPE-005`: Explicit C4 scope that maps to zero local files fails before discovery.
- `ASSERT-C4-SCOPE-006`: Generated `<protocol>-scope.txt` contains only normalized repo-relative paths, one per line.
- `ASSERT-C4-OVERRIDE-001`: User-supplied `custom_doc` is not overwritten by generated docs.
- `ASSERT-C4-OVERRIDE-002`: User-supplied `audit_scope` is not overwritten by generated scope markdown.
- `ASSERT-C4-OVERRIDE-003`: User-supplied `scoped_files` is not overwritten by generated scope text.
- `ASSERT-C4-OVERRIDE-004`: If only one manual artifact is supplied, the two missing artifacts are generated.
- `ASSERT-C4-DOCS-001`: Generated `<protocol>-docs.md` includes protocol overview and architecture/mechanics from the contest README.
- `ASSERT-C4-DOCS-002`: Generated `<protocol>-docs.md` includes README-linked user/developer/protocol documentation when a docs link is fetchable.
- `ASSERT-C4-DOCS-003`: Generated `<protocol>-docs.md` includes scoped-code documentation summaries for in-scope contracts.
- `ASSERT-C4-DOCS-004`: GitHub `docs/` tree links are traversed under budget and included or skipped with provenance.
- `ASSERT-C4-DOCS-005`: Relevant root-level Markdown/text files beyond `README.md` are included when they are protocol, scope, security, architecture, audit, invariant, risk, design, or deployment docs.
- `ASSERT-C4-V12-001`: Megapot/Intuition/Olas V12 links are detected when present.
- `ASSERT-C4-V12-002`: Fetched V12 findings are rendered in `<protocol>-scope.md` as known issues / out-of-scope exclusions.
- `ASSERT-C4-V12-003`: A contest without V12 links records `No V12 findings detected` in provenance.
- `ASSERT-C4-POC-001`: PoC/test submission requirements are excluded from discovery docs.
- `ASSERT-C4-POC-002`: PoC/test submission requirements are preserved in `<protocol>-validation.md`.
- `ASSERT-TELEM-001`: Benchmark `run_manifest.json` includes the full config snapshot listed in this spec.

### Security And Scope Assertions

- `ASSERT-SEC-SCOPE-001`: The app does not silently fall back to all Solidity files when explicit C4 scope fails to map.
- `ASSERT-SEC-SCOPE-002`: Every remote source used appears in `<protocol>-context-sources.json`.
- `ASSERT-SEC-SCOPE-003`: Every skipped remote source appears in `<protocol>-context-sources.json` with a reason.
- `ASSERT-SEC-SCOPE-004`: V12 known issues are C4 competition-only and are not injected for bounty audit types.

### UX Assertions

- `ASSERT-UX-001`: Context generation logs identify generated, reused, and user-supplied artifacts.
- `ASSERT-UX-002`: Scope mapping failures name the failing scope source and at least one unmapped path example.
- `ASSERT-UX-003`: Generated Markdown renders with clear headings for scope, docs, known issues, V12 findings, invariants, and validation-only PoC rules.
- `ASSERT-UX-004`: `context-sources.json` can be inspected to determine why each README link was fetched or skipped.

### Error Handling Assertions

- `ASSERT-ERR-001`: Missing user-supplied context artifact path fails with a clear error.
- `ASSERT-ERR-002`: Failed remote docs fetch records a warning and does not panic.
- `ASSERT-ERR-003`: Failed V12 fetch records a warning and does not panic.
- `ASSERT-ERR-004`: Generated context over token budget is summarized or rejected; it is never silently truncated.

### Test Assertions

- `ASSERT-TEST-001`: `cargo fmt -- --check` passes after implementation.
- `ASSERT-TEST-002`: `cargo clippy --all-targets -- -D warnings` passes, or any pre-existing lint failures are documented with evidence and no new lint failures are introduced.
- `ASSERT-TEST-003`: Unit tests cover `scope.txt` parsing, README scope-table parsing, path normalization, manual override behavior, V12 link detection, and provenance recording.
- `ASSERT-TEST-004`: Fixture integration tests cover Megapot, Olas, Intuition, Monetrix, and Panoptic context extraction without live network or LLM calls.
- `ASSERT-TEST-005`: A fixture-based context generation integration test produces the five expected generated artifacts without running full discovery.
- `ASSERT-TEST-006`: Benchmark telemetry manifest test proves the config snapshot is present.

### Manual Validation Assertions

- `ASSERT-MANUAL-001`: Generated Megapot `docs.md` is readable and useful as audit context.
- `ASSERT-MANUAL-002`: Generated Megapot `scope.md` clearly separates in-scope files, out-of-scope files, known issues, V12 issues, invariants, and trusted roles.
- `ASSERT-MANUAL-003`: Generated Megapot `scope.txt` matches the C4 contest scope exactly after normalization.
- `ASSERT-MANUAL-004`: The app prints or documents the exact command a user can run for a full audit, but does not run the audit automatically.

## Worker Plan

### Orchestrator

Responsibilities:

- Own this spec and validation contract.
- Keep implementation scoped to C4 contest context and telemetry.
- Review every worker result before merging.
- Run fix loops until required assertions pass.
- Decide whether any remaining P2 items are accepted or deferred.

### Worker 1: Config, Telemetry, And C4 Entry Wiring

Scope:

- `src/parse` or the current CLI/YAML config module.
- `src/prepare_code/git_clone.rs`.
- `src/benchmark/telemetry.rs`.
- README/example config updates if needed.

Tasks:

- Add `code4rena_contest_repo` / `code4rena_contest_url`.
- Wire C4 contest source selection into context generation.
- Add benchmark config snapshot to `run_manifest.json`.
- Preserve current behavior for existing YAMLs.

Worker output:

- Changed files.
- New config examples.
- Evidence for `ASSERT-C4-DETECT-*` and `ASSERT-TELEM-001`.

### Worker 2: Code4rena Context Collector

Scope:

- New or existing context collector code under `src/prepare_code/`.
- Test fixtures under `tests/fixtures/code4rena/` or equivalent.

Tasks:

- Implement C4 README, scope, out-of-scope, V12, and linked-doc source collection.
- Implement deterministic scope extraction and path mapping.
- Implement fail-closed behavior for unmapped explicit scope.
- Record provenance and warnings.

Worker output:

- Changed files.
- Fixture coverage notes.
- Evidence for scope, V12, provenance, and error assertions.

### Worker 3: Documentation And Scoped-Code Extractor

Scope:

- Documentation extraction/rendering modules.
- No telemetry or CLI changes.

Tasks:

- Add repo-local documentation discovery.
- Add bounded GitHub docs tree traversal.
- Add docs-site text extraction support.
- Add scoped-code NatSpec/comment extraction.
- Ensure README-linked protocol/user/developer docs are included in docs generation under budget.
- Keep PoC requirements out of discovery docs and in validation sidecar.

Worker output:

- Changed files.
- Source budget behavior.
- Evidence for docs and PoC assertions.

### Worker 4: Test And Lint Worker

Scope:

- Tests only, unless a minimal production patch is needed to make tests possible.

Tasks:

- Add unit tests for parsers/mappers/renderers.
- Add fixture integration tests for Megapot, Olas, Intuition, Monetrix, Panoptic.
- Run:
  - `cargo fmt -- --check`
  - `cargo test`
  - `cargo clippy --all-targets -- -D warnings`
- Report exact failing tests/lints and required fixes.

Worker output:

- Commands run.
- Pass/fail status.
- Assertion coverage matrix.
- Coverage gaps.

### Worker 5: Code Review Worker

Scope:

- Read-only review of final diff.

Tasks:

- Review for scope expansion bugs, context poisoning, silent fallback, missing provenance, path mapping errors, brittle tests, and regression risk.
- Provide file/line findings ordered by severity.
- Re-review after fixes.

Worker output:

- Findings with severity.
- Required fixes.
- Final no-blocking-issues statement.

### Worker 6: Final Integration Validation Worker

Scope:

- End-to-end validation only.

Tasks:

- Run fixture-based context generation checks.
- Verify generated artifacts:
  - `<protocol>-docs.md`
  - `<protocol>-scope.md`
  - `<protocol>-scope.txt`
  - `<protocol>-validation.md`
  - `<protocol>-context-sources.json`
- Verify manual override behavior with a small fixture config.
- Verify benchmark `run_manifest.json` config snapshot.
- Do not run full Megapot discovery.

Worker output:

- Integration commands.
- Artifact paths.
- Validation assertion status.
- Final pass/fail recommendation for handing the full-audit command to the user.

## Final Validation Commands

Required before the implementation is considered ready to hand the user a full-audit command:

```bash
cargo fmt -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Required targeted checks:

```bash
cargo test code4rena_context
cargo test audit_context
```

Required context-generation integration command will be finalized during implementation. Preferred shape:

```bash
cargo test code4rena_context_fixture_generates_artifacts
```

If the app adds a context-only subcommand, Worker 6 may also validate that command against a fixture config. Live Megapot execution is intentionally outside this implementation job; the command for the user-owned run lives in `MEGAPOT_CLEAN_RUN_CONTRACT.md`.

## Open Questions

1. Should `code4rena_contest_url` accept the C4 web audit URL in addition to the GitHub repo URL?
2. Should PDF extraction be required for initial implementation or allowed as best-effort with provenance?
3. Should generated context include full V12 finding bodies or summarized V12 finding rows?
4. Should manual override behavior require all three artifacts to be supplied, or is partial override generation acceptable? This spec assumes partial override generation.
5. Should generated context include full raw docs excerpts in `context-sources.json`, or only source metadata plus rendered summaries?
