# Megapot Clean Run Contract

This file is intentionally separate from `CODE4RENA_CONTEXT_PHASE2_SPEC.md`.

Codex must not run the full Megapot audit. The user will run it after the Code4rena context-generation code changes are implemented, validated, and committed.

## Purpose

Run a clean Megapot Phase 2 benchmark using the current algorithm, current LLM models, generated Code4rena context artifacts, and benchmark telemetry.

## Preconditions

Do not start the full run until:

- Code changes for Code4rena context auto-generation are committed.
- All P0/P1 validation assertions in `CODE4RENA_CONTEXT_PHASE2_SPEC.md` pass.
- `validation_supervision: "gui"` is enabled in the run config so the final three-shot validation/report workflow is emitted after the Rust audit report.
- Generated Megapot context artifacts have been produced and spot-checked:
  - `<protocol>-docs.md`
  - `<protocol>-scope.md`
  - `<protocol>-scope.txt`
  - `<protocol>-validation.md`
  - `<protocol>-context-sources.json`
- `run_manifest.json` records the config constants listed in the spec.
- User confirms the run id.

## Planned Run Id

```text
megapot-clean-r1r2-10-inv3-actor2-001
```

## User-Owned Run Command

Codex should provide this command to the user, not execute it:

```bash
AI_AGENT_AUDIT_BENCHMARK_TELEMETRY=1 \
AI_AGENT_AUDIT_BENCHMARK_RUN_ID=megapot-clean-r1r2-10-inv3-actor2-001 \
AI_AGENT_AUDIT_BENCHMARK_DIR=benchmarks/code4rena-2025-11-megapot/runs \
cargo run --release -- --config audit-docs/megapot-auto.yaml
```

If `audit-docs/megapot-auto.yaml` is not the final generated config name, update the command before handing it to the user.

## Expected Benchmark References

- Ground truth: `benchmarks/code4rena-corpus/competitions/2025-11-megapot/ground_truth/index.md`
- Accepted H/M count: 11
- Rejected primary count: 24
- Contract plus AbstractContract artifact target: <= 20

## Post-Run Checks

After the user completes the run, analyze:

- `run_manifest.json`
- `codeblock_manifest.jsonl`
- `raw_candidates.jsonl`
- `dedup_clusters.jsonl`
- `verification_decisions.jsonl`
- `final_candidates.jsonl`
- `finding_lifecycle.jsonl`
- `validation-three-shot/jobs/<benchmark>/<run-id>/manifest.json`
- `validation-three-shot/jobs/<benchmark>/<run-id>/events.jsonl`
- R1-R9 three-shot outputs, PoC runs/reviews, finding reports, report reviews, judge simulations, and scoring artifacts listed by the three-shot job manifest

Compare final output against Megapot ground truth to compute:

- accepted H/M true positives,
- accepted H/M false negatives,
- rejected-primary false positives,
- dedupe quality,
- Rust verification false rejects,
- three-shot validation false rejects,
- PoC readiness,
- professional report readiness,
- final three-shot report survival rate.
