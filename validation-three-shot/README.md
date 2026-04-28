# Validation Three-Shot

This folder stores the production three-shot validation workflow, worker prompts,
configuration, and run artifacts.

Three-shot means:

1. round 1 screens only for scope / known issues / V12 duplicates
2. round 2 screens only for unsupported-token / unsupported-ERC20 behavior
3. round 3 runs the full validation gates only on the survivors
4. optionally, round 4 canonicalizes the assembled H/M candidates and gently drops duplicate-equivalent findings
5. optionally, round 4a performs a stricter V12 / prior-finding overlap sweep on post-R4 candidates
6. optionally, round 5 creates runnable PoCs for post-R4/R4a submission candidates
7. optionally, round 6 independently verifies or repairs PoCs and invalidates only unprovable findings
8. optionally, round 7 creates one Code4rena submission-ready report per verified finding
9. optionally, round 8 independently reviews and repairs each report, or flags it `Need Further Review`
10. a separate scoring worker scores the assembled raw-validation run

Recommended worker split:

- all R1-R8 workers must be launched with the configured minimal Codex worker launcher, currently `/Users/apmfree/codex-minimal-worker`, so plugin MCP servers are not loaded into every parallel worker
- round 1 scope worker: `gpt-5.5` with `xhigh`
- round 2 token worker: `gpt-5.5` with `xhigh`
- round 3 final validation worker: `gpt-5.5` with `xhigh`
- optional round 4 canonicalization worker: `gpt-5.4` with `high`
- optional round 4a V12 sweep worker: `gpt-5.4` with `high`
- optional round 5 PoC generation workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- optional round 6 PoC verification workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- optional round 7 C4 report generation workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- optional round 8 C4 report review workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- scoring worker: `gpt-5.4` with `xhigh`
- PoC workers must produce C4-ready artifacts: filenames include finding IDs for traceability, Solidity names avoid pipeline IDs and use descriptive vulnerability names, comments tersely explain setup/trigger/proof, and each final finding preferably gets one standalone PoC file. Existing template files are read-only seeds: workers copy, rename, and edit the copied finding-specific file rather than touching the template.
- Report workers must produce ultra-concise C4-ready reports: one fresh worker per finding, about 200-300 words excluding code, two primary sections plus proof of concept, and `Need Further Review` when not submission-ready.

Each validation round should be run by a separate spawned minimal Codex worker with cleared context. For R5-R8, each individual finding must get its own fresh minimal worker and isolated prompt/output files.

Configuration:

- `validation-three-shot/config.yaml` is the default production config.
- Change `benchmark`, `run_id`, `paths.source_root`, and `paths.audit_report` there for a new contest.
- `paths.source_root` is the full path to the contest source/workspace bundle. It is not assumed to live under `~/Desktop/Audit/<benchmark>`.
- `paths.audit_report` is the full path to the AI audit results file containing `## [H-1]. Title` style finding headings. It is not assumed to live under this repository.
- Optional `paths.truth_file` can be configured for local benchmark scoring when a truth artifact is available.
- `context_docs` is the authoritative YAML list of files every three-shot worker must read. It supports exact paths and glob patterns, including `{{SOURCE_ROOT}}` and `{{THREE_SHOT_ROOT}}` placeholders.
- The reusable V12 duplicate decision checklist is stored at `validation-three-shot/v12-checklist.md`; include it in `context_docs`.
- Benchmark prior findings can be a standalone `v12-findings.md` entry or embedded in a `*-scope.md` file. If embedded, omit the standalone `v12-findings.md` entry from `context_docs`.
- `prompt_version` is the reusable prompt/artifact namespace. Keep `v2` for the current production prompts unless intentionally testing a new prompt version.
- Worker model, reasoning, and launcher settings also live there. Keep `workers.default.launcher` pointed at the minimal Codex worker launcher unless intentionally debugging a single worker.
- Each `prepare-*` command emits `worker_launcher`, `requires_minimal_codex_worker`, `worker_spawn_command_template`, and, when `--write-prompt` is used, an exact `worker_spawn_command`; use those fields instead of the desktop `spawn_agent` path for production fanout.
- CLI flags still work as one-off overrides.
- All live worker prompts and shared validation rubrics live in `validation-three-shot/prompts/`.
- The old top-level `validation-prompts/` directory and old `validation-three-shot/validation-prompts/` directory are not required by this workflow.

Editable round prompts:

- `validation-three-shot/prompts/r1.md`
- `validation-three-shot/prompts/r2.md`
- `validation-three-shot/prompts/r3.md`
- `validation-three-shot/prompts/r4.md`
- `validation-three-shot/prompts/r4a.md`
- `validation-three-shot/prompts/r5.md`
- `validation-three-shot/prompts/r6.md`
- `validation-three-shot/prompts/r7.md`
- `validation-three-shot/prompts/r8.md`
- `validation-three-shot/prompts/validation-v2.md`
- `validation-three-shot/prompts/scoring.md`

Editable prompt changelog:

- `validation-three-shot/prompts/CHANGELOG.md`

Primary motivation:

- cheaply eliminate obvious out-of-scope / known-issue / unsupported-token false positives
- preserve recall for the remaining real H/M roots

Future cleanup / canonicalization guidance:

- raw validation should keep same-family findings when each one independently looks valid
- any later cleanup worker should drop only duplicate-equivalent findings
- do not dedup merely because two findings share a broad root cause, bug family, or root-cause group
- round 4 root-cause groups are for later report/PoC planning and are not automatic drop criteria
- there is no reward penalty for submitting some duplicates to C4, but early over-deduping creates false negatives

Benchmark docs emphasized by this workflow via `context_docs`:

- `README.md`
- `*-scope.md`
- `*-docs.md`
- benchmark prior findings, either standalone `v12-findings.md` or embedded in scope/docs
- shared `validation-three-shot/v12-checklist.md`

Recommended workflow:

```bash
python3 scripts/three_shot_round.py prepare-scope --write-prompt /tmp/three-shot-r1.md
python3 scripts/three_shot_round.py prepare-token --write-prompt /tmp/three-shot-r2.md
python3 scripts/three_shot_round.py prepare-final --write-prompt /tmp/three-shot-r3.md
python3 scripts/three_shot_round.py assemble
python3 scripts/three_shot_round.py prepare-dedup --write-prompt /tmp/three-shot-r4.md
python3 scripts/three_shot_round.py apply-dedup
python3 scripts/three_shot_round.py prepare-v12-sweep --write-prompt /tmp/three-shot-r4a.md
python3 scripts/three_shot_round.py apply-v12-sweep
python3 scripts/three_shot_round.py prepare-poc --write-prompt /tmp/three-shot-r5.md
python3 scripts/three_shot_round.py assemble-poc
python3 scripts/three_shot_round.py prepare-poc-review --write-prompt /tmp/three-shot-r6.md
python3 scripts/three_shot_round.py assemble-poc-review
python3 scripts/three_shot_round.py prepare-report --write-prompt /tmp/three-shot-r7.md
python3 scripts/three_shot_round.py prepare-report-review --write-prompt /tmp/three-shot-r8.md
python3 scripts/three_shot_round.py score-prompt --write-prompt /tmp/three-shot-score.md
```

Production R5/R6 runs use the full post-R4/R4a submission candidate file as the queue:

```text
validation-three-shot/submission-candidates/<prompt-version>/<benchmark>-<run-id>.md
```

However, `prepare-poc` emits exactly one R5 worker unit at a time. By default it selects the next unfinished finding; pass `--finding-id H-1` to prepare a specific unit. Spawn a fresh minimal Codex worker using the returned `worker_launcher` for that one prompt, wait for it to complete, then call `prepare-poc` again until it returns `"worker_type": "none"`.

After all R5 units complete, run `assemble-poc` to create the aggregate R5 table/JSON. R6 follows the same per-finding model: `prepare-poc-review` emits exactly one review worker for the next R5-complete and R6-unfinished finding, and `assemble-poc-review` creates the aggregate R6 table/JSON after every review unit completes.

R7 and R8 also follow the same per-finding model. `prepare-report` emits one report-writing worker for the next R6-verified finding without an R7 report. `prepare-report-review` emits one review worker for the next completed R7 report without an R8 review. Every R8 worker either marks the report `Ready`, fixes it and marks `Fixed And Ready`, or flags `Need Further Review` with detailed notes.

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
- `validation-three-shot/v12-sweeps/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/submission-candidates/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/inputs/r5/<prompt-version>/<benchmark>-<run-id>/<finding-id>.md`
- `validation-three-shot/poc-runs/<prompt-version>/<benchmark>-<run-id>/<finding-id>.md`
- `validation-three-shot/poc-runs/<prompt-version>/<benchmark>-<run-id>/<finding-id>.json`
- `validation-three-shot/poc-runs/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/poc-runs/<prompt-version>/<benchmark>-<run-id>.json`
- `validation-three-shot/poc-verification/<prompt-version>/<benchmark>-<run-id>/<finding-id>.md`
- `validation-three-shot/poc-verification/<prompt-version>/<benchmark>-<run-id>/<finding-id>.json`
- `validation-three-shot/poc-verification/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/poc-verification/<prompt-version>/<benchmark>-<run-id>.json`
- `validation-three-shot/inputs/r7/<prompt-version>/<benchmark>-<run-id>/<finding-id>.md`
- `validation-three-shot/finding-reports/<prompt-version>/<benchmark>-<run-id>/<finding-id>.md`
- `validation-three-shot/finding-reports/<prompt-version>/<benchmark>-<run-id>/<finding-id>.json`
- `validation-three-shot/inputs/r8/<prompt-version>/<benchmark>-<run-id>/<finding-id>.md`
- `validation-three-shot/finding-report-reviews/<prompt-version>/<benchmark>-<run-id>/<finding-id>.md`
- `validation-three-shot/finding-report-reviews/<prompt-version>/<benchmark>-<run-id>/<finding-id>.json`
- `validation-three-shot/results/<prompt-version>/<benchmark>-<run-id>.md`
- `validation-three-shot/results/<prompt-version>/<benchmark>-<run-id>.json`
