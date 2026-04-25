# Validation Three-Shot

This folder stores three-shot validation experiments.

Three-shot means:

1. round 1 screens only for scope / known issues / V12 duplicates
2. round 2 screens only for unsupported-token / unsupported-ERC20 behavior
3. round 3 runs the full validation gates only on the survivors
4. a separate scoring worker scores the assembled final run

Recommended worker split:

- round 1 scope worker: `gpt-5.5` with `xhigh`
- round 2 token worker: `gpt-5.5` with `xhigh`
- round 3 final validation worker: `gpt-5.5` with `xhigh`
- scoring worker: `gpt-5.4` with `xhigh`

Each validation round should be run by a separate spawned worker with cleared context. The controller emits `requires_fresh_worker_context: true` for rounds 1-3.

Editable round prompts:

- `validation-three-shot/prompts/r1.md`
- `validation-three-shot/prompts/r2.md`
- `validation-three-shot/prompts/r3.md`

Primary motivation:

- cheaply eliminate obvious out-of-scope / known-issue / unsupported-token false positives
- preserve recall for the remaining real H/M roots

Mandatory benchmark docs emphasized by this workflow:

- `README.md`
- `olas-scope.md`
- `olas-docs.md`
- `v12-findings.md`
- `v12-checklist.md`

Recommended workflow:

```bash
python3 scripts/three_shot_round.py prepare-scope --prompt-version v2 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/three-shot-r1.md
python3 scripts/three_shot_round.py prepare-token --prompt-version v2 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/three-shot-r2.md
python3 scripts/three_shot_round.py prepare-final --prompt-version v2 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/three-shot-r3.md
python3 scripts/three_shot_round.py assemble --prompt-version v2 --benchmark 2026-01-olas --run-id run-001
python3 scripts/three_shot_round.py score-prompt --prompt-version v2 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/three-shot-score.md
```

Optional deterministic local scorer:

```bash
python3 scripts/three_shot_round.py score-local --prompt-version v2 --benchmark 2026-01-olas --run-id run-001
```

Artifacts:

- `validation-three-shot/scope-screens/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/token-screens/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/stage3-runs/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/runs/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/results/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/results/<prompt-version>/<benchmark>-<run-id>.json`
