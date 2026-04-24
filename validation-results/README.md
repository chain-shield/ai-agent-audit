# Validation Results

This folder stores the live per-run results dashboard plus the final scored summary for each validation run.

## Naming Convention

- `prompt-version` = exact prompt file stem, for example `v1`, `v2`, `v3`

- `benchmark` = exact top-level benchmark folder slug, for example `2026-01-olas`

- `run-id` = sequential run number for a given `(prompt-version, benchmark)` pair, for example `run-001`

The `run-id` must match the raw validation run that was scored.

Example:

- `validation-results/v1/2026-01-olas-run-001.md`
- `validation-results/v1/2026-01-olas-run-001-worker-log.md`
- `validation-results/v1/2026-01-olas-run-001-worker-log.jsonl`

## Recommended Contents

Each results file should include:

- a live raw-validation progress section that updates after each completed finding
- the current per-finding decision table for all processed findings so far
- benchmark name
- prompt version tested
- run id
- scoring scope, with the primary confusion matrix computed on `H-` / `M-` findings only
- in that primary matrix, run-side positives are only rows marked `Decision=Valid` with `Severity Assessment=High` or `Medium`
- run-side `Valid` rows scored as `Low / QA` or `Low / Unclear`, plus `Invalid` rows, count as negatives for the primary matrix
- `Needs Review` remains an abstention bucket outside `TP` / `FP` / `TN` / `FN`
- `TP`, `FP`, `TN`, `FN` on that severity-aware H/M subset
- recall on that H/M subset
- precision on that H/M subset
- specificity on that H/M subset
- abstention / `Needs Review` rate on that H/M subset, if used
- total canonical approved H/M findings
- unique approved H/M findings present in the report
- unique approved H/M findings accepted as `Valid` with `High` / `Medium` severity
- present-root recall
- end-to-end unique recall
- top false-negative patterns
- top false-positive patterns
- checklist gates that caused the most mistakes
- promotion decision for the next prompt version

## Worker Logs

Each run should also maintain durable worker lifecycle logs in this folder.

The worker log is the detailed event stream, while the main run results file is the human-readable dashboard for the current iteration.

- `validation-results/<prompt-version>/<benchmark>-<run-id>-worker-log.md`
  - human-readable log of spawned, completed, failed, shutdown, and backfilled worker events
  - for completed raw workers, include the exact finding block that was appended to the raw run file

- `validation-results/<prompt-version>/<benchmark>-<run-id>-worker-log.jsonl`
  - machine-readable companion log with one JSON object per worker event
  - useful for replay, dashboards, and auditing whether the orchestrator skipped or retried any findings

## Suggested Layout

```text
validation-results/
  v1/
    2026-01-olas-run-001.md
    2026-01-olas-run-001-worker-log.md
    2026-01-olas-run-001-worker-log.jsonl
  v2/
    2026-01-olas-run-001.md
    2025-12-panoptic-run-001.md
```
