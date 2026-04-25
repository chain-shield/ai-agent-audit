# Validation One-Shot

This folder stores one-shot validation experiments.

One-shot means:

- one worker validates the full benchmark report in one pass
- the worker still uses the same rigorous validation prompt and evidence rules
- a separate worker scores the completed one-shot run using the normal H/M validation rubric

Recommended worker split:

- one-shot validation worker: `gpt-5.5` with `xhigh` reasoning
- one-shot scoring worker: `gpt-5.4` with `xhigh` reasoning

Goal:

- if one worker can achieve roughly `50%` to `60%` H/M precision while preserving extremely high recall, the serialized loop may be unnecessary for that benchmark class

Recommended workflow:

```bash
python3 scripts/oneshot_round.py prepare --prompt-version v1 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/one-shot-worker.md
```

After the validation worker fills the run file, generate the separate scoring worker prompt with:

```bash
python3 scripts/oneshot_round.py score-prompt --prompt-version v1 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/one-shot-scoring-worker.md
```

Optional deterministic local scorer for offline comparison:

```bash
python3 scripts/oneshot_round.py score-local --prompt-version v1 --benchmark 2026-01-olas --run-id run-001
```

Artifacts:

- `validation-one-shot/runs/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-one-shot/results/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-one-shot/results/<prompt-version>/<benchmark>-<run-id>.json`
