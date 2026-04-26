# Validation Three-Shot

This folder stores the production three-shot validation workflow, worker prompts,
configuration, and run artifacts.

Three-shot means:

1. round 1 screens only for scope / known issues / V12 duplicates
2. round 2 screens only for unsupported-token / unsupported-ERC20 behavior
3. round 3 runs the full validation gates only on the survivors
4. optionally, round 4 canonicalizes the assembled H/M candidates and gently drops duplicate-equivalent findings
5. a separate scoring worker scores the assembled raw-validation run

Recommended worker split:

- round 1 scope worker: `gpt-5.5` with `xhigh`
- round 2 token worker: `gpt-5.5` with `xhigh`
- round 3 final validation worker: `gpt-5.5` with `xhigh`
- optional round 4 canonicalization worker: `gpt-5.4` with `high`
- scoring worker: `gpt-5.4` with `xhigh`

Each validation round should be run by a separate spawned worker with cleared context. The controller emits `requires_fresh_worker_context: true` for rounds 1-3.

Configuration:

- `validation-three-shot/config.yaml` is the default production config.
- Change `benchmark`, `prompt_version`, and `run_id` there for a new contest.
- Worker model and reasoning settings also live there.
- CLI flags still work as one-off overrides.
- Baseline validation prompts live in `validation-three-shot/validation-prompts/`.
- The old top-level `validation-prompts/` directory is not required by this workflow.

Editable round prompts:

- `validation-three-shot/prompts/r1.md`
- `validation-three-shot/prompts/r2.md`
- `validation-three-shot/prompts/r3.md`
- `validation-three-shot/prompts/r4.md`
- `validation-three-shot/prompts/scoring.md`

Editable validation baseline:

- `validation-three-shot/validation-prompts/v2.md`
- `validation-three-shot/validation-prompts/CHANGELOG.md`

Primary motivation:

- cheaply eliminate obvious out-of-scope / known-issue / unsupported-token false positives
- preserve recall for the remaining real H/M roots

Future cleanup / canonicalization guidance:

- raw validation should keep same-family findings when each one independently looks valid
- any later cleanup worker should drop only duplicate-equivalent findings
- do not dedup merely because two findings share a broad root cause, bug family, or root-cause group
- round 4 root-cause groups are for later report/PoC planning and are not automatic drop criteria
- there is no reward penalty for submitting some duplicates to C4, but early over-deduping creates false negatives

Mandatory benchmark docs emphasized by this workflow:

- `README.md`
- `olas-scope.md`
- `olas-docs.md`
- `v12-findings.md`
- `v12-checklist.md`

Recommended workflow:

```bash
python3 scripts/three_shot_round.py prepare-scope --write-prompt /tmp/three-shot-r1.md
python3 scripts/three_shot_round.py prepare-token --write-prompt /tmp/three-shot-r2.md
python3 scripts/three_shot_round.py prepare-final --write-prompt /tmp/three-shot-r3.md
python3 scripts/three_shot_round.py assemble
python3 scripts/three_shot_round.py prepare-dedup --write-prompt /tmp/three-shot-r4.md
python3 scripts/three_shot_round.py apply-dedup
python3 scripts/three_shot_round.py score-prompt --write-prompt /tmp/three-shot-score.md
```

Optional deterministic local scorer:

```bash
python3 scripts/three_shot_round.py score-local
```

Override the config for a one-off run:

```bash
python3 scripts/three_shot_round.py prepare-scope --config validation-three-shot/config.yaml --benchmark 2026-04-example --run-id run-001
```

Artifacts:

- `validation-three-shot/scope-screens/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/token-screens/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/stage3-runs/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/runs/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/dedup-screens/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/submission-candidates/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/results/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/results/<prompt-version>/<benchmark>-<run-id>.json`
