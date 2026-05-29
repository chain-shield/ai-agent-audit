# Benchmark Schema

This directory stores reproducible benchmark definitions for AI Agent Audit performance research. The goal is to measure audit quality, not runtime.

Primary benchmark questions:

- How many unique accepted High/Medium roots did the app find?
- Which accepted roots were missed, and at what stage?
- Which false positives survived as reportable H/M?
- Did dedup merge only true duplicates?
- Did verification retain valid findings and reject invalid ones?

## Directory Layout

```text
benchmarks/
  README.md
  code4rena-2026-01-olas/
    manifest.yaml
```

Future Phase 2 output should add generated run artifacts under a separate run directory, for example:

```text
benchmarks/code4rena-2026-01-olas/runs/run-001/
  raw_candidates.jsonl
  dedup_clusters.jsonl
  verification_decisions.jsonl
  codeblock_manifest.jsonl
  finding_lifecycle.jsonl
  final_candidates.jsonl
  benchmark_score.json
  benchmark_score.md
```

## Corpus-Aware Scoring

Use the Code4rena corpus scorer for benchmark runs whose ground truth lives under
`benchmarks/code4rena-corpus/competitions/<slug>/`:

```bash
python3 scripts/score_code4rena_benchmark.py score \
  --slug 2025-11-megapot \
  --run-dir benchmarks/code4rena-2025-11-megapot/runs/2025-11-megapot-538649/megapot-clean-r1r2-10-inv3-actor2-001 \
  --three-shot-run-id run-001 \
  --prompt-version v2
```

The scorer writes `benchmark_score.json` and `benchmark_score.md` into the run
directory. It automatically matches app candidates to archived accepted H/M
findings, matches unmatched final-stage candidates against rejected primaries,
and attributes accepted-root losses by pipeline stage.

Compare before/after experiments with:

```bash
python3 scripts/score_code4rena_benchmark.py compare \
  --score benchmarks/code4rena-2025-11-megapot/runs/<project-id>/<baseline-run>/benchmark_score.json \
  --score benchmarks/code4rena-2025-11-megapot/runs/<project-id>/<experiment-run>/benchmark_score.json \
  --out-dir benchmarks/code4rena-2025-11-megapot/comparisons/<comparison-id>
```

## Phase 4 Failure Analysis

After a run has a scorecard, generate root-by-root failure analysis:

```bash
python3 scripts/score_code4rena_benchmark.py analyze \
  --score benchmarks/code4rena-2025-11-megapot/runs/<project-id>/<run-id>/benchmark_score.json
```

This writes `failure_analysis.json` and `failure_analysis.md` next to the
scorecard. The analysis checks accepted-root context coverage, raw discovery,
Rust dedup/verification/export survival, three-shot survival, PoC/report/judge
survival, and final-stage FP triage.

## Whitepaper Evidence Packs

Preserve all data needed for later whitepaper analysis with:

```bash
python3 scripts/score_code4rena_benchmark.py archive-evidence \
  --score benchmarks/code4rena-2025-11-megapot/runs/<project-id>/<run-id>/benchmark_score.json
```

The archive command copies benchmark telemetry, scorecards, failure analysis,
Code4rena corpus ground truth, generated audit context docs, three-shot
validation artifacts, and cross-run comparisons into:

```text
benchmarks/whitepaper-data/<slug>/<run-id>/
  evidence_manifest.json
  evidence_manifest.md
  files/
```

The manifest records source paths, archive paths, byte sizes, categories, and
SHA-256 hashes so whitepaper claims can be traced back to immutable local
evidence.

## Manifest Schema

```yaml
benchmark: string
audit_type: Code4rena | Code4renaBounty | Sherlock | Cantina | Client
status: phase_1_manifest | baseline_ready | evaluated
source:
  repo_url: string
  commit_hash: string | unknown
  source_root: string
  audit_root: string
  audit_report: string
ground_truth:
  accepted_roots_file: string
  rejected_references_file: string
  snapshots_dir: string
metrics:
  accepted_high_count: number
  accepted_medium_count: number
  accepted_total_hm_count: number
accepted_roots:
  - id: string
    severity: High | Medium
    title: string
    source_url: string
    root_contracts: [string]
    root_functions: [string]
    expected_pattern_family: string
    notes: string
```

## Match Types

- `same_root_cause`: candidate and benchmark root describe the same bug.
- `duplicate_variant`: candidate is a duplicate or weaker variant of a matched root.
- `related_not_same`: same area or family, different root cause.
- `false_positive`: candidate does not match an accepted root and maps to invalid/low/QA behavior.
- `unknown`: manual adjudication needed.

## Terminal Lifecycle Labels

- `matched_true_positive`
- `unmatched_false_positive`
- `context_missing_scope`
- `context_missing_graph`
- `context_missing_token_budget`
- `context_present_discovery_miss`
- `discovery_extraction_error`
- `dedup_overmerge`
- `dedup_undermerge`
- `verification_false_reject`
- `validation_false_accept`
- `severity_false_downgrade`
- `report_omission`
