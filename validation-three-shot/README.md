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
8. optionally, round 7 creates one submission-ready report per verified finding
9. optionally, round 8 independently reviews and repairs each report, or flags it `Need Further Review`
10. a separate scoring worker scores the assembled raw-validation run

For `validation_profile: code4rena-bounty`, the same controller uses bounty-specific prompts:

1. round 1 screens for bounty scope, known issues, previous audits, closed bounty reports, sponsor OOS, and Code4rena global bounty OOS
2. round 2 screens for current-code bug existence, concrete exploitability, unprivileged attacker preconditions, and plausible runnable PoC
3. round 3 keeps only findings that map to Code4rena bounty Critical/High criteria
4. round 4 dedup/group, PoC creation, PoC validation, report creation, and final review stay
5. round 4a V12 sweep is disabled for bounties

For `validation_profile: immunefi-bounty`, the controller uses Immunefi-specific prompts:

1. round 1 screens for asset scope, impact scope, Primacy of Impact vs Primacy of Rules, known issues, prior audits, program OOS, prohibited behavior, and Immunefi-wide OOS
2. round 2 screens for current-code bug existence, concrete exploitability, unprivileged attacker preconditions, and plausible later local PoC
3. round 3 classifies severity against the generated per-bounty Immunefi severity rubric
4. round 4 dedup/group, PoC creation, PoC validation, report creation, and final review stay
5. round 4a V12 sweep is disabled for bounties

Recommended worker split:

- all R1-R9 workers must be launched with the configured minimal Codex worker launcher, currently `/Users/apmfree/codex-minimal-worker`, so plugin MCP servers are not loaded into every parallel worker while normal local file access and native Codex web search remain available. Web search here means the native `--search` flag only; it must not re-enable browser, GitHub, or other MCP servers.
- round 1 scope worker: `gpt-5.5` with `xhigh`
- round 2 token worker: `gpt-5.5` with `xhigh`
- round 3 final validation worker: `gpt-5.5` with `xhigh`
- optional round 4 canonicalization worker: `gpt-5.4` with `high`
- optional round 4a V12 sweep worker: `gpt-5.4` with `high`
- optional round 5 PoC generation workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- optional round 6 PoC verification workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- optional round 7 report generation workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- optional round 8 report review workers: one fresh `gpt-5.5` / `xhigh` worker per finding
- optional round 9 judge simulation workers: one fresh `gpt-5.5` / `xhigh` worker per reviewed report
- scoring worker: `gpt-5.4` with `xhigh`
- PoC workers must produce submission-ready artifacts: filenames include finding IDs for traceability, Solidity names avoid pipeline IDs and use descriptive vulnerability names, comments tersely explain setup/trigger/proof, and each final finding preferably gets one standalone PoC file. Existing template files are read-only seeds: workers copy, rename, and edit the copied finding-specific file rather than touching the template.
- Report workers must produce ultra-concise submission-ready reports: one fresh worker per finding, about 200-300 words excluding code, two primary sections plus proof of concept, and `Need Further Review` when not submission-ready.

Each validation round should be run by a separate spawned minimal Codex worker with cleared context. For R5-R9, each individual finding/report must get its own fresh minimal worker and isolated prompt/output files. Minimal workers are expected to read the local files listed in their prompt and may use native web search to verify cited public source URLs.

Configuration:

- `validation-three-shot/config.yaml` is the default production config.
- Change `benchmark`, `run_id`, `paths.source_root`, `paths.audit_root`, and `paths.audit_report` there for a new contest.
- `paths.source_root` is the full path to the contest source/workspace bundle. It is not assumed to live under `~/Desktop/Audit/<benchmark>`.
- `paths.audit_root` is the full path to this tool's output folder for the benchmark. If `paths.audit_report` is omitted, the workflow reads `report/audit-report.md` under `paths.audit_root`.
- `paths.audit_report` is the full path to the AI audit results file containing `## [H-1]. Title` style finding headings. It is not assumed to live under this repository.
- Optional `paths.truth_file` can be configured for local benchmark scoring when a truth artifact is available.
- `context_docs` is the authoritative YAML list of files every three-shot worker must read. It supports exact paths and glob patterns, including `{{SOURCE_ROOT}}` and `{{THREE_SHOT_ROOT}}` placeholders.
- The reusable V12 duplicate decision checklist is stored at `validation-three-shot/v12-checklist.md`; include it in `context_docs`.
- The reusable Code4rena bounty criteria summary is stored at `validation-three-shot/code4rena-bounty-criteria.md`; include it in `context_docs` for `validation_profile: code4rena-bounty`.
- Immunefi bounty configs must include the generated per-protocol files under `audit-docs/<protocol>/`, especially `<protocol>-immunefi-bounty-rules.md` and `<protocol>-immunefi-severity-rubric.md`.
- Benchmark prior findings can be a standalone `v12-findings.md` entry or embedded in a `*-scope.md` file. If embedded, omit the standalone `v12-findings.md` entry from `context_docs`.
- `prompt_version` is the reusable prompt/artifact namespace. Keep `v2` for the current production prompts unless intentionally testing a new prompt version.
- Worker model, reasoning, and launcher settings also live there. Keep `workers.default.launcher` pointed at the minimal Codex worker launcher unless intentionally debugging a single worker.
- Each `prepare-*` command emits `worker_launcher`, `requires_minimal_codex_worker`, `worker_spawn_command_template`, and, when `--write-prompt` is used, an exact `worker_spawn_command`; use those fields instead of the desktop `spawn_agent` path for production fanout.
- CLI flags still work as one-off overrides.
- Live worker prompts live in profile subfolders under `validation-three-shot/prompts/`.
- The old top-level `validation-prompts/` directory and old `validation-three-shot/validation-prompts/` directory are not required by this workflow.

GUI-supervised validation:

- Audit runs emit Codex GUI supervision jobs by default after report export.
- Disable job emission with `--validation-supervision off`, `validation_supervision: off` in the audit YAML config, or `AI_AGENT_AUDIT_VALIDATION_SUPERVISION=off`.
- Explicit `--validation-supervision gui`, `validation_supervision: gui`, or `AI_AGENT_AUDIT_VALIDATION_SUPERVISION=gui` keep the default enabled behavior.
- The audit app still only produces deterministic artifacts. It does not run R1-R9 directly.
- After report export, the app writes a ready-marked job folder:
  - `validation-three-shot/jobs/<benchmark>/<run-id>/manifest.json`
  - `validation-three-shot/jobs/<benchmark>/<run-id>/config.yaml`
  - `validation-three-shot/jobs/<benchmark>/<run-id>/supervisor.md`
  - `validation-three-shot/jobs/<benchmark>/<run-id>/status.md`
  - `validation-three-shot/jobs/<benchmark>/<run-id>/events.jsonl`
  - `validation-three-shot/jobs/<benchmark>/<run-id>/ready`
- `ready` is written last. Codex GUI supervisors should ignore job folders without this marker.
- If the same `<benchmark>/<run-id>` already has a non-terminal `manifest.json` status, such as `pending` or `running`, the audit app refuses to overwrite that job. Delete the job folder or change `run_id` after the current validation job is resolved.
- The manifest points at the job-local `config.yaml` snapshot, not the shared `validation-three-shot/<benchmark>-config.yaml`, so later audit runs cannot mutate the config a running GUI supervisor is using.
- The intended trigger is a dedicated Codex heartbeat supervisor that wakes every 5 minutes, scans `validation-three-shot/jobs/*/*/ready`, claims pending manifests, runs preflight, then follows the job-specific `supervisor.md`.
- The GUI supervisor provides commentary and status updates; minimal Codex workers launched from the emitted `worker_spawn_command` do the production R1-R9 work.

Editable round prompts:

- `validation-three-shot/prompts/code4rena/r1.md` through `r9.md`
- `validation-three-shot/prompts/code4rena-bounty/r1.md` through `r9.md`
- `validation-three-shot/prompts/immunefi-bounty/r1.md` through `r9.md`, plus optional `r3a.md`
- `validation-three-shot/prompts/default/r1.md` through `r9.md`
- `validation-three-shot/prompts/validation-v2.md`
- `validation-three-shot/prompts/<profile>/score.md`

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
python3 scripts/three_shot_round.py prepare-feasibility --write-prompt /tmp/three-shot-r3a.md
python3 scripts/three_shot_round.py apply-feasibility
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
python3 scripts/three_shot_round.py prepare-judge --write-prompt /tmp/three-shot-r9.md
python3 scripts/three_shot_round.py score-prompt --write-prompt /tmp/three-shot-score.md
```

`prepare-feasibility` / `apply-feasibility` are only active for `validation_profile: immunefi-bounty` when `rounds.r3a_feasibility_gate: true`; other profiles skip this stage.

Production R5/R6 runs use the full post-R4/R4a submission candidate file as the queue:

```text
validation-three-shot/submission-candidates/<prompt-version>/<benchmark>-<run-id>.md
```

However, `prepare-poc` emits exactly one R5 worker unit at a time. By default it selects the next unfinished finding; pass `--finding-id H-1` to prepare a specific unit. Spawn a fresh minimal Codex worker using the returned `worker_launcher` for that one prompt, wait for it to complete, then call `prepare-poc` again until it returns `"worker_type": "none"`.

After all R5 units complete, run `assemble-poc` to create the aggregate R5 table/JSON. R6 follows the same per-finding model: `prepare-poc-review` emits exactly one review worker for the next R5-complete and R6-unfinished finding, and `assemble-poc-review` creates the aggregate R6 table/JSON after every review unit completes.

R7 and R8 also follow the same per-finding model. `prepare-report` emits one report-writing worker for the next R6-verified finding without an R7 report. `prepare-report-review` emits one review worker for the next completed R7 report without an R8 review. Every R8 worker either marks the report `Ready`, fixes it and marks `Fixed And Ready`, or flags `Need Further Review` with detailed notes.

R9 follows the same per-report model. `prepare-judge` emits one non-mutating judge-simulation worker for the next completed R8 report without an R9 simulation. Every R9 worker writes a Markdown/JSON decision under `validation-three-shot/judge-simulations/<prompt-version>/<benchmark>-<run-id>/` and does not edit the report or PoC.

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
