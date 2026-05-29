# Phase 2: Benchmark Telemetry

Phase 2 adds opt-in JSONL telemetry so benchmark runs can show where findings are created, merged, rejected, retained, or lost. The benchmark endpoint is the full product flow: Rust discovery/verification/report generation plus final GUI-supervised three-shot validation and professional report production.

Telemetry is disabled by default. Enable it for controlled benchmark runs:

```bash
AI_AGENT_AUDIT_BENCHMARK_TELEMETRY=1 \
AI_AGENT_AUDIT_BENCHMARK_RUN_ID=olas-baseline-001 \
cargo run --release -- --config audit-docs/olas.yaml
```

Optional output root:

```bash
AI_AGENT_AUDIT_BENCHMARK_DIR=benchmarks/code4rena-2026-01-olas/runs
```

If no output root is provided, records are written under:

```text
.ai-agent-audit/benchmark-runs/<project-id>/<run-id>/
```

## Artifacts

### `run_manifest.json`

Written once after repository preparation. Captures repo identity, commit, audit type, scope/docs paths, source folders, and telemetry directory.

### `codeblock_manifest.jsonl`

One record per generated contract codeblock.

Key fields:

- `contract`
- `max_depth`
- `token_budget`
- `final_token_count`
- `traversal_mode`
- `called_contracts`
- `contracts_with_depth`
- `parent_contracts`
- `included_files`
- dependency file sets and skip counts

Use this to answer: did the accepted finding's required code and files make it into the LLM context?

### `raw_candidates.jsonl`

One record per reviewed contract after discovery and candidate IDs are assigned.

Key fields:

- `contract`
- `finding_count`
- `findings`

Use this to answer: did discovery generate any candidate for the accepted root?

### `dedup_clusters.jsonl`

One record per dedup stage.

Current stages:

- `per_contract_pre_verify`
- `global_final`

Key fields:

- `input_count`
- `output_count`
- `clusters`
- per-cluster `inputs`, `kept`, and `dropped`

Use this to answer: did dedup over-merge distinct findings or under-merge duplicates?

### `verification_decisions.jsonl`

Records universal verification, downgrade validation, retention decisions, and post-verification retained findings.

Current stages:

- `universal_verification`
- `downgrade_validation`
- `retention_decisions`
- `post_verification_retained`

Use this to answer: did verification falsely reject an accepted H/M finding, or falsely retain an invalid finding?

### `final_candidates.jsonl`

One record containing globally deduped final candidates returned to report generation.

Use this as the final candidate set for matching against `benchmarks/code4rena-2026-01-olas/manifest.yaml`.

### `finding_lifecycle.jsonl`

One record per candidate per lifecycle stage.

Current stages:

- `discovery_raw`
- `post_verification_retained`
- `final_report_candidate`

Use this to connect a raw candidate ID to later validation and final report survival.

### Three-Shot Validation Artifacts

The Rust telemetry above stops at the Rust report-generation boundary. A complete benchmark also includes the final three-shot validation phase emitted under:

```text
validation-three-shot/jobs/<benchmark>/<run-id>/
```

Capture at least:

- `manifest.json`
- `config.yaml`
- `supervisor.md`
- `status.md`
- `events.jsonl`
- final expected outputs listed in `manifest.expected_outputs`

Also preserve R1-R9 artifacts under the profile/run directories described in `validation-three-shot/README.md`, including assembled validation JSON, canonicalized findings, V12/prior sweep outputs, PoC runs, PoC reviews, finding reports, report reviews, judge simulations, and scoring outputs.

Use these artifacts to answer: did a candidate that survived Rust verification become a three-shot validated, PoC-ready, professional report-ready H/M finding?

## First Olas Baseline Procedure

1. Run the Olas audit with telemetry enabled and a fixed run id.
2. Check `codeblock_manifest.jsonl` for each accepted root in `benchmarks/code4rena-2026-01-olas/manifest.yaml`.
3. Match `raw_candidates.jsonl` to accepted roots.
4. Inspect `dedup_clusters.jsonl` for accepted roots that disappear before verification.
5. Inspect `verification_decisions.jsonl` for accepted roots that are tagged invalid, low, or dropped by retention.
6. Match `final_candidates.jsonl` to accepted and rejected references.
7. Wait for the `validation-three-shot/jobs/<benchmark>/<run-id>/manifest.json` status to reach `completed`.
8. Match three-shot R1-R9 outputs, PoC reviews, report reviews, judge simulations, and scoring outputs to accepted and rejected references.
9. Write the baseline report with TP, FP, FN, dedup, Rust false-reject, three-shot false-reject, PoC readiness, report readiness, and judge-simulation metrics.
