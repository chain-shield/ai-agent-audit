# AI Agent Audit Performance Deep Dive Plan

## Purpose

This plan turns the performance deep dive into an audit-style research program for AI Agent Audit. The goal is not runtime speed. Performance means the app's ability to discover, deduplicate, validate, prove, and report unique valid High/Medium Solidity/EVM findings.

The north star is simple:

- Maximize unique accepted H/M true positives in complex protocols.
- Minimize false positives that survive to report-ready or judge-sim accepted output.
- Preserve true positives through Rust discovery, Rust verification, global dedup, three-shot validation, PoC generation, report review, and judge simulation.
- Favor small code, prompt, config, schema, and orchestration changes over broad rewrites.

A proposed change is worth pursuing only if controlled benchmark evidence shows that it materially improves unique H/M recall, precision, dedup quality, validation quality, PoC/report readiness, or stage survival.

## Benchmark Definition

The benchmark endpoint is the full product flow:

1. Code4rena contest repository and scope/docs ingestion.
2. Rust discovery and verification.
3. Rust global dedup and report export.
4. GUI-supervised three-shot validation.
5. R5/R6 PoC creation and verification.
6. R7/R8 finding report creation and review.
7. R9 judge simulation.
8. Corpus-backed scoring against archived Code4rena accepted and rejected primary findings.

Rust report generation alone is not the endpoint. A finding is only fully successful when it survives to a report-ready or judge-accepted three-shot artifact and maps to a unique accepted C4 H/M root.

## Source Of Truth

The local Code4rena corpus is the primary ground truth:

```text
benchmarks/code4rena-corpus/competitions/<slug>/
  final_findings.json
  final_report.html
  accepted_findings.md
  rejected_primaries.md
  ground_truth/
    accepted/*.md
    rejected/*.md
  submissions/
    primaries.jsonl
    rejected_primaries.jsonl
```

Accepted H/M roots come from `final_findings.json` and `ground_truth/accepted/*.md`. False-positive references come from `submissions/rejected_primaries.jsonl` and `ground_truth/rejected/*.md`, including invalid, low/QA, duplicate, and otherwise non-accepted primaries.

Each benchmark run should write scorecards next to the run artifacts:

```text
benchmarks/code4rena-<slug>/runs/<project-id>/<run-id>/
  run_manifest.json
  raw_candidates.jsonl
  dedup_clusters.jsonl
  verification_decisions.jsonl
  finding_lifecycle.jsonl
  final_candidates.jsonl
  benchmark_score.json
  benchmark_score.md
```

Cross-run comparisons should be written under:

```text
benchmarks/code4rena-<slug>/comparisons/<comparison-id>/
  benchmark_compare.json
  benchmark_compare.md
```

## Scoring Tool

Use the corpus-aware scorer:

```bash
python3 scripts/score_code4rena_benchmark.py score \
  --slug 2025-11-megapot \
  --run-dir benchmarks/code4rena-2025-11-megapot/runs/2025-11-megapot-538649/megapot-clean-r1r2-10-inv3-actor2-001 \
  --three-shot-run-id run-001 \
  --prompt-version v2
```

Compare runs:

```bash
python3 scripts/score_code4rena_benchmark.py compare \
  --score benchmarks/code4rena-2025-11-megapot/runs/<project-id>/<baseline-run>/benchmark_score.json \
  --score benchmarks/code4rena-2025-11-megapot/runs/<project-id>/<experiment-run>/benchmark_score.json \
  --out-dir benchmarks/code4rena-2025-11-megapot/comparisons/<comparison-id>
```

The scorer performs deterministic local matching. It does not call an LLM. It uses weighted title/body/source-evidence similarity to match app candidates to accepted roots and rejected primaries. Ambiguous or low-confidence matches should be reviewed manually before making product claims.

## Primary Metrics

Core output metrics:

- Accepted H/M total: number of unique accepted C4 H/M roots in the benchmark.
- Rust final accepted-root recall: accepted roots found in `final_candidates.jsonl`.
- Report-ready accepted-root recall: accepted roots that reach R8-ready report output.
- Judge-accepted end-to-end recall: accepted roots that reach R9 accepted output.
- Final-stage precision: unique accepted roots represented in final-stage candidates divided by final-stage candidates.
- Judge-stage precision: unique accepted roots represented in R9 candidates divided by R9 candidates.
- Final-stage false positives: report-ready or judge-stage candidates that do not match accepted roots.
- Matched rejected-primary FPs: final-stage candidates that match rejected/low/duplicate/invalid C4 primaries.
- Unadjudicated final-stage candidates: final-stage candidates that match neither accepted nor rejected corpus references above threshold.

Stage survival metrics:

- Discovery recall.
- Rust dedup-kept recall.
- Rust post-verification recall.
- Rust final recall.
- Three-shot R3 recall.
- Three-shot submission-candidate recall.
- PoC-created recall.
- PoC-verified recall.
- Report-created recall.
- Report-ready recall.
- Judge-accepted recall.

Dedup metrics:

- Under-merge groups: multiple final-stage candidates map to the same accepted root.
- Over-merge suspects: accepted roots present in raw discovery but absent after dedup/finalization.
- Cluster purity review: dedup clusters should group one root cause, not merely one contract/function neighborhood.

Loss-stage labels:

- `context_or_discovery_miss`
- `dedup_or_rust_verification_loss`
- `rust_verification_loss`
- `rust_final_report_omission`
- `three_shot_validation_loss`
- `three_shot_canonicalization_or_v12_loss`
- `poc_creation_loss`
- `poc_verification_loss`
- `report_creation_loss`
- `report_review_loss`
- `judge_simulation_reject_or_missing`
- `survived_end_to_end`

## Phase 1: Algorithm Trace

Status: complete.

Deliverables:

- Map the pipeline from repo prep through final report production.
- Identify every gate where a real finding can be created, merged, downgraded, discarded, or fail to become a professional report.
- Document the Rust and three-shot stages as one product pipeline.

Key files:

- `src/prepare_code/git_clone.rs`
- `src/prepare_code/audit_context.rs`
- `src/enumerator/codeblocks.rs`
- `src/llm_review/analysis/code_review_v2.rs`
- `src/llm_review/findings/findings.rs`
- `src/llm_review/phases/verify_rounds.rs`
- `src/reporting/three_shot_config.rs`
- `src/reporting/validation_supervision.rs`
- `scripts/three_shot_round.py`

## Phase 2: Instrumentation And Scoring Harness

Status: in progress.

Implemented:

- Rust benchmark telemetry for run config, codeblocks, raw candidates, dedup clusters, verification decisions, lifecycle records, and final candidates.
- Code4rena context auto-generation from contest repositories.
- Three-shot job emission and R1-R9 artifact capture.
- Corpus-aware benchmark scorer in `scripts/score_code4rena_benchmark.py`.
- Per-run `benchmark_score.json` and `benchmark_score.md`.
- Cross-run `benchmark_compare.json` and `benchmark_compare.md`.

Remaining hardening:

- Tune deterministic matching thresholds across several contests.
- Add optional manual adjudication overrides for ambiguous matches.
- Add confidence labels to scorecards.
- Add CI smoke tests for scorer parsing and scorecard rendering.

Exit criteria:

- Every benchmark run can be scored from archived artifacts without hand-built truth tables.
- The scorecard identifies accepted roots found/missed, final-stage FPs, rejected-primary matches, and failure-stage counts.

## Phase 3: Megapot Baseline

Use `2025-11-megapot` as the first clean small benchmark because it has a compact Solidity scope and complete Code4rena corpus artifacts.

Current clean-run config:

```text
R1_RUNS = 10
R2_RUNS = 10
INVARIANT_RUNS = 3
ACTOR_RUNS = 2
benchmark run id = megapot-clean-r1r2-10-inv3-actor2-001
three-shot run id = run-001
```

Current scorecard:

```text
benchmarks/code4rena-2025-11-megapot/runs/2025-11-megapot-538649/megapot-clean-r1r2-10-inv3-actor2-001/benchmark_score.md
```

Initial baseline signal:

- 11 accepted H/M C4 roots in ground truth.
- 8 accepted roots survived to judge-accepted output.
- 5 final-stage candidates were not matched to accepted roots.
- 4 final-stage candidates matched rejected/duplicate/low C4 primary references.
- Main losses: two discovery/context misses and one three-shot canonicalization/V12-stage loss.

This baseline should be treated as a working scorer output, not a public claim, until threshold tuning and manual adjudication are complete.

## Phase 4: Failure Analysis

Status: in progress.

Implemented:

- `scripts/score_code4rena_benchmark.py analyze` writes per-run `failure_analysis.json` and `failure_analysis.md`.
- Accepted-root analysis records context coverage, raw discovery presence, Rust dedup/verification/export survival, three-shot survival, PoC/report/judge survival, loss stage, and next analysis focus.
- Final-stage FP analysis records rejected-primary matches, rejection category, expected removal gate, and whether the issue looks like a duplicate variant, severity overstatement, true FP, or unadjudicated candidate.

For each accepted root:

1. Determine whether relevant scoped files and docs were present in generated context.
2. Determine whether a matching raw candidate appeared.
3. Determine whether the root survived per-contract dedup.
4. Determine whether Rust verification retained or downgraded it.
5. Determine whether global dedup/report export retained it.
6. Determine whether R1/R2/R3 three-shot validation retained it.
7. Determine whether R4/R4a canonicalization/V12 sweep retained it.
8. Determine whether R5/R6 produced and verified a PoC.
9. Determine whether R7/R8 produced a ready report.
10. Determine whether R9 judge simulation accepted it.

For each final-stage FP:

1. Match against rejected primaries where possible.
2. Record rejection category: invalid, low/QA, duplicate, out-of-scope, insufficient, trusted-role, known issue, or unadjudicated.
3. Identify the stage that should have removed or merged it.
4. Identify whether the issue is a true FP, duplicate variant, severity overstatement, or potentially novel.

Megapot baseline Phase 4 output:

```text
benchmarks/code4rena-2025-11-megapot/runs/2025-11-megapot-538649/megapot-clean-r1r2-10-inv3-actor2-001/failure_analysis.md
```

Initial bottleneck read:

- Two accepted roots had all referenced source files in context but no raw matching candidate, which points to discovery gaps rather than scope extraction gaps.
- One accepted root survived Rust final export but did not survive three-shot canonicalization/V12 retention.
- Four final-stage FPs currently map to rejected duplicate primaries, which points to dedup/canonicalization quality as a high-leverage improvement area.

## Whitepaper Evidence Preservation

Every benchmark used for future public claims must have a local evidence pack.
Evidence packs are not summaries; they preserve the underlying data needed to
recalculate and audit the claim.

Implemented:

- `scripts/score_code4rena_benchmark.py archive-evidence` copies benchmark telemetry, scorecards, failure analysis, Code4rena ground truth, generated audit context, three-shot artifacts, and comparison outputs into `benchmarks/whitepaper-data/<slug>/<run-id>/`.
- `evidence_manifest.json` and `evidence_manifest.md` record source paths, archive paths, byte sizes, artifact categories, and SHA-256 hashes.

Megapot baseline evidence pack:

```text
benchmarks/whitepaper-data/2025-11-megapot/megapot-clean-r1r2-10-inv3-actor2-001/evidence_manifest.md
```

Whitepaper rules:

- Do not cite any benchmark number unless its scorecard, failure analysis, corpus ground truth, and validation artifacts are present in an evidence pack.
- Low-confidence matches and unadjudicated candidates must be manually reviewed before they become external claims.
- Preserve before/after evidence packs for every experiment, not only the winning run.

## Phase 5: High-Leverage Experiment Tracks

Only run experiments that target measured bottlenecks.

Track A: Context Recall

- Check accepted roots against codeblock manifests.
- Fix scope/docs extraction, imported dependency inclusion, call graph depth, inheritance/interface expansion, or token-budget behavior only where accepted roots lacked context.

Track B: Discovery Coverage

- Map missed accepted roots to pattern families.
- Add or reschedule pattern categories only when a missing family has benchmark support.
- Avoid increasing run counts unless unique TP per candidate improves.

Track C: Dedup

- Add root-cause fingerprints: violated invariant, affected state, attacker action, loss path, contract/function path.
- Preserve evidence variants under a canonical finding instead of losing them.
- Treat low-confidence same-area findings as separate until validation, not automatic drops.

Track D: Rust Verification

- Split hard-invalid labels from evidence-needed labels.
- Track accepted roots killed by governance-risk, user-error, unsupported-token, scope, safeguards, speculation, and severity gates.
- Add appeal or retention behavior only for benchmark-proven false rejects.

Track E: Three-Shot Validation

- Reduce false rejects in R1-R4a without letting rejected primaries survive.
- Treat PoC/report gaps as repairable when the bug is otherwise valid.
- Tighten R9 acceptance when C4 rejected-primary analogs exist.

Track F: Reportability

- Score reports for source location, exploit path, impact, proof, and mitigation.
- Distinguish technically valid findings from submission-ready/client-ready findings.

## Phase 6: Cross-Benchmark Validation

After one Megapot-improving patch is proposed, validate across at least three additional contests selected from the archived corpus.

Recommended benchmark mix:

- One small EVM contest with 5-15 accepted H/M roots.
- One medium DeFi contest with 15-30 accepted H/M roots.
- One large contest where dedup pressure is high.
- One contest with many rejected primaries to stress FP filtering.

Run the same scoring command for every run and compare:

- Baseline vs experiment.
- Recall delta.
- Precision delta.
- Matched rejected-primary FP delta.
- Dedup under-merge delta.
- Loss-stage movement.

Keep only changes that improve at least one primary metric without materially harming precision or report readiness.

## Phase 7: Live Or Fresh No-Leakage Evaluation

For a live or freshly finalized contest:

- Freeze prompts, config, and scorer thresholds before results are known.
- Do not ingest final report, submissions, V12 findings, or public discussion until after the run is complete.
- Preserve run manifests, context docs, Rust telemetry, three-shot prompts, worker outputs, PoCs, reports, reviews, judge simulations, and scorecards.
- After public judging, import ground truth and score once.

This is the only evidence suitable for strong external claims.

## Guardrails

Do not:

- Optimize raw candidate count.
- Treat retrospective public findings as live performance.
- Count duplicate variants as multiple wins.
- Count Rust-only findings as product-complete.
- Treat judge simulation as a replacement for C4 ground truth.
- Land broad rewrites without per-stage benchmark evidence.
- Add expensive LLM rounds unless unique TP per final candidate improves.

## Definition Of 10x

Use two definitions:

1. Absolute output: roughly 10x more accepted unique H/M roots per complex protocol at comparable review effort.
2. Signal-adjusted output: a large increase in unique accepted H/M roots while preserving or improving final precision, rejected-primary filtering, dedup quality, PoC readiness, and report readiness.

A change that finds more candidates but lowers final precision is not a 10x improvement. A change that finds valid bugs but cannot produce ready reports is incomplete. A change that recovers several accepted roots by fixing one measured failure stage is the kind of change this project should prioritize.

## Final Deliverables

- Reproducible scorecards for every benchmark run.
- Cross-run comparison reports for each experiment.
- A ranked bottleneck list by failure stage.
- A short recommended patch set with expected metric impact.
- A rejected-ideas list with measured reasons.
- A no-leakage evaluation plan for the next live or fresh contest.
