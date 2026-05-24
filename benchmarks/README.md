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
  baseline_report.md
```

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
