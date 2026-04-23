# Validation Runs

This folder stores raw validator outputs.

## Naming Convention

- `prompt-version` = exact prompt file stem, for example `v1`, `v2`, `v3`

- `benchmark` = exact top-level benchmark folder slug, for example `2026-01-olas`

- `run-id` = sequential run number for a given `(prompt-version, benchmark)` pair, for example `run-001`

To choose the next `run-id`, scan the existing files under `validation-runs/<prompt-version>/` for the same benchmark and increment the highest existing run number. If no prior run exists for that pair, start at `run-001`.

## Layout

```text
validation-runs/
  v1/
    2026-01-olas-run-001.md
    2026-01-olas-run-001.jsonl
    2026-01-olas-run-002.md
  v2/
    2026-01-olas-run-001.md
    2025-12-panoptic-run-001.md
```

## Rules

- Keep one subfolder per prompt version.

- Keep one pair of files per `(benchmark, run-id)` inside that prompt-version folder.

- Use one fresh raw-validation worker per finding, then aggregate by appending into the run file.

- Preserve raw reasoning so later prompt revisions can be traced back to real misses.

- Do not overwrite older runs unless you are intentionally refreshing the exact same `(prompt-version, benchmark, run-id)` pair.

- Raw validation should be grounded in the actual benchmark source repository and relevant benchmark docs when they exist, not just the finding text or summarized benchmark report.

- Do not run two raw-validation workers concurrently against the same raw run file.

- Use [validation_loop.py](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/scripts/validation_loop.py) to initialize raw files, identify the next missing finding, and generate the one-finding worker prompt.

Recommended command:

```bash
python3 scripts/validation_loop.py next --prompt-version v2 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/validation-worker.md
```

## Recommended Raw Run Shape

Initialize the raw markdown file early and keep appending one finding block at a time.

Recommended scaffold:

```md
# <benchmark> Raw Validation <run-id>

Status: In progress
Benchmark report: `...`
Validation prompt: `...`

## Per-Finding Validation

<!-- APPEND FINDING BLOCKS ABOVE THIS LINE -->
```

Each raw-validation worker should:

- read the current raw run file;
- append exactly one new finding block above the anchor comment;
- avoid rewriting previously completed finding blocks;
- exit after that one append succeeds.
