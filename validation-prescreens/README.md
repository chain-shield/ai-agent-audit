# Validation Prescreens

This folder stores recall-first prescreen artifacts built from raw validation runs.

Purpose:

- weed out obviously bad findings before the main validation rounds
- preserve as many real findings as possible for downstream review
- keep a machine-readable candidate list that later rounds can consume

Critical design rule:

- prescreen reuses the same rigorous per-finding validation worker that the main loop uses
- prescreen does not switch to a simplified "bug-exists only" prompt
- only the downstream keep-set scoring is lighter / more recall-protective

Recommended baseline:

- source prompt version: `v1`
- one fresh worker per finding, exactly like the existing raw validation pass
- derive the prescreen keep-set from the completed raw run instead of rescoring by hand
- keep the worker task hard and fully structured; use the prescreen layer only to decide what advances

Current builder:

```bash
python3 scripts/prescreen_round.py build --prompt-version v1 --benchmark 2026-01-olas --run-id run-001
```

Artifacts:

- `validation-prescreens/<prompt-version>/<benchmark>-<run-id>.md`
  - human-readable summary, scoring, and kept-candidate table
- `validation-prescreens/<prompt-version>/<benchmark>-<run-id>-kept.json`
  - machine-readable keep-set for possible downstream filtering

Default keep rule:

- `bug-exists-not-no`
  - keep a finding if the raw worker did not conclude `Bug Exists: No`
  - this is intentionally more recall-protective than the main-round severity rubric
  - it still comes from the same rigorous raw validation output rather than a separate easy prompt
