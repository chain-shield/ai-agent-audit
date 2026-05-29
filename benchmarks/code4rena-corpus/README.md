# Code4rena Corpus

This folder is the benchmark archive for Code4rena competitions from 2024 through 2026.

The corpus has two layers:

- Public layer: contest metadata, repo links, scope/report pages, final report text, and accepted High/Medium report findings.
- Authenticated layer: primary submissions, duplicate/triage state, rejected primaries, judge/sponsor rationale, and rejection categories.

Public Code4rena pages expose audit metadata and final reports. Submission lists and individual submission pages require a logged-in Code4rena session, so those files are marked `requires_authentication` until captured through the browser-backed workflow.

## Layout

```text
benchmarks/code4rena-corpus/
  manifest.json
  manifest.jsonl
  needs-auth-submissions.todo.jsonl
  schemas/
    competition.schema.json
    submission.schema.json
  competitions/
    2026-03-intuition/
      competition.json
      final_report.html
      final_report.txt
      final_findings.json
      submissions/
        README.md
        primaries.jsonl
        rejected_primaries.jsonl
        raw/
```

## Public Archive Command

```bash
python3 scripts/archive_code4rena_corpus.py discover \
  --out benchmarks/code4rena-corpus \
  --year-start 2024 \
  --year-end 2026 \
  --all-leagues
```

Use `--all-leagues` for the full multi-codebase corpus. Omit it to use the script default Solidity/EVM-oriented league filter: `ETH`, `eth`, `Blast`, and `Hyperliquid`. Each competition stores the raw public audit API payload, repo links, report URL, report text, and parsed H/M report headings when a final report is available.

## Authenticated Submission Capture

For each `competition.json`, the authenticated endpoint is:

```text
https://code4rena.com/api/v1/audits/{uid}/submissions/csv
```

The archiver can normalize a CSV captured from a logged-in Code4rena browser session:

```bash
python3 scripts/archive_code4rena_corpus.py import-submissions-csv \
  --out benchmarks/code4rena-corpus \
  --slug 2026-03-intuition \
  --csv /path/to/downloaded/submissions.csv
```

The normalized output preserves raw rows and writes:

- `submissions/all.jsonl`
- `submissions/primaries.jsonl`
- `submissions/rejected_primaries.jsonl`

Rejected primary rows get a best-effort `rejection_reason_category` from the taxonomy in `schemas/submission.schema.json`; manual review can override the category while retaining the raw reason text.

When the Codex Chrome connector is available, authenticated primary submission capture can also be run from the connected browser runtime by importing:

```text
scripts/code4rena_authenticated_browser_scrape.mjs
```

That browser scrape writes the same normalized files plus raw per-submission detail text under `submissions/raw/`.

## Benchmark Use

For AI Agent Audit evaluation, use:

- `competition.json` for repo URL, source scope, commit hints, and contest metadata.
- `final_findings.json` as accepted H/M ground truth from the final report.
- `submissions/primaries.jsonl` for the full primary submission set.
- `submissions/rejected_primaries.jsonl` for false-positive calibration and verifier/dedup regression tests.

Every generated file should keep source URLs and capture timestamps so later benchmark runs can be traced back to the archived primary material.
