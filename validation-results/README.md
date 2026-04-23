# Validation Results

This folder stores scored summaries for each validation run.

## Naming Convention

- `prompt-version` = exact prompt file stem, for example `v1`, `v2`, `v3`

- `benchmark` = exact top-level benchmark folder slug, for example `2026-01-olas`

- `run-id` = sequential run number for a given `(prompt-version, benchmark)` pair, for example `run-001`

The `run-id` must match the raw validation run that was scored.

Example:

- `validation-results/v1/2026-01-olas-run-001.md`

## Recommended Contents

Each results file should include:

- benchmark name
- prompt version tested
- run id
- scoring scope, with the primary confusion matrix computed on `H-` / `M-` findings only
- `TP`, `FP`, `TN`, `FN` on that H/M subset
- recall on that H/M subset
- precision on that H/M subset
- specificity on that H/M subset
- abstention / `Needs Review` rate on that H/M subset, if used
- total canonical approved H/M findings
- unique approved H/M findings present in the report
- unique approved H/M findings accepted as `Valid`
- present-root recall
- end-to-end unique recall
- top false-negative patterns
- top false-positive patterns
- checklist gates that caused the most mistakes
- promotion decision for the next prompt version

## Suggested Layout

```text
validation-results/
  v1/
    2026-01-olas-run-001.md
  v2/
    2026-01-olas-run-001.md
    2025-12-panoptic-run-001.md
```
