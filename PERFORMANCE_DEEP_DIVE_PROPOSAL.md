# Proposal: AI Agent Audit Performance Deep Dive

## Executive Summary

This proposal lays out a focused deep dive into AI Agent Audit with one goal: identify the smallest set of code and prompt changes that can materially improve the app's ability to discover unique valid High and Medium Solidity/EVM findings.

For this work, "performance" does not mean runtime speed. It means audit performance:

- More unique High/Medium true positives in complex Solidity protocols.
- Better root-cause deduplication, triage, and validation.
- Fewer false positives that survive to the final report.
- Fewer true positives lost during deduplication or verification.

The research posture should be ruthless: ignore algorithm ideas that are merely interesting. A change is worth pursuing only if it can plausibly move accepted unique H/M output, precision, or validation quality by a large amount while keeping the refactor small.

## Why This Matters

The current strategic benchmark is Code4rena-grade performance. Code4rena is useful because it gives external judging, unique finding grouping, High/Medium severity decisions, and public reports. Code4rena also defines "signal" as an accuracy metric: valid High/Medium findings divided by High/Medium submissions. That maps well to this app's core product question: can AI Agent Audit generate more valid H/M findings without flooding judges, clients, or the founder with junk?

The repo already contains useful seed assets:

- `C4_APPROVED_FINDINGS.md`
- `C4_REJECTED_FINDINGS_KEY.md`
- `C4_LOW_QA_INVALID_FINDINGS.md`
- `c4_snaps/`
- Historical run outputs for `2025-11-megapot`, `2025-12-panoptic`, `2026-01-olas`, `2026-03-intuition`, and `2026-04-monetrix`

These make it possible to build a local benchmark loop before running expensive new audits.

## Current Algorithm To Understand

The first workstream is a deep map of how the app works today, not just at module level but at "where can a valid finding be created or killed?" level.

Current pipeline, as implemented:

1. Repository preparation: clone, build, scope/docs generation, config normalization.
2. Static analysis: Slither-derived call graph, function metadata, IR, storage/function summaries.
3. Codeblock generation: per-contract slices using call graph BFS, inheritance, imports, token budget, and fallback traversal.
4. Metadata context: protocol-level docs/scope context.
5. Per-contract multimodal context: actor discovery plus invariant discovery.
6. Pattern discovery: direct finding generation from selected pattern categories, currently focused through `R1` and `R2` pattern libraries with actor and invariant variants.
7. Local per-contract deduplication.
8. Verification: universal invalidity checks over bug existence, safeguards, scope, by-design behavior, exploitability, user error, privileged actor assumptions, and future speculation.
9. Downgrade validation: second pass that challenges invalid/low labels.
10. Global deduplication and report generation.

Key files to study:

- `src/main.rs`
- `src/prepare_code/git_clone.rs`
- `src/prepare_code/audit_context.rs`
- `src/build_brain/enrichment.rs`
- `src/enumerator/codeblocks.rs`
- `src/llm_review/analysis/code_review_v2.rs`
- `src/llm_review/analysis/pre_audit_analysis.rs`
- `src/llm_review/pattern_phases/generate_direct_findings.rs`
- `src/llm_review/phases/verify_rounds.rs`
- `src/llm_review/findings/findings.rs`
- `src/llm_review/threat_models/pattern_category.rs`

The deep dive should produce a "finding lifecycle" diagram showing every gate where a true bug can be missed, hallucinated, duplicated, downgraded, or discarded.

## Research Questions

The deep dive should answer these questions with evidence:

1. Context quality: For each accepted C4 finding, did the relevant contract/codeblock contain enough source, neighboring functions, docs, and cross-contract context for an LLM to find it?
2. Hypothesis generation: If the context was present, did discovery fail because the prompt did not ask the right question, the pattern category was missing, the run count was too low, or the model did not explore deeply enough?
3. Actor/invariant value: Do actor and invariant prompts increase unique valid H/M discovery, or do they mostly add noisy candidates?
4. Dedup behavior: Does dedup correctly merge duplicate reports by root cause without merging distinct H/M issues in the same contract/function?
5. Verification behavior: Which true positives are killed by safeguards, scope, governance-risk, unsupported-token, user-error, speculation, or insufficient-impact logic?
6. Severity behavior: Are true H/M issues downgraded to Low/QA because the verifier lacks contest-specific impact reasoning?
7. Benchmark leakage: Can we separate retrospective C4 benchmark runs from live, no-leakage evaluations?
8. Small-change leverage: Which improvements can be made as config, prompt, scoring, telemetry, or shallow orchestration changes rather than a rewrite?

## Benchmark Plan

### Tier 0: Local Olas Replay

Use `2026-01-olas` as the first benchmark because the repo already has approved findings, rejected mappings, snapshots, and prior app output.

Tasks:

- Create a canonical accepted root-cause set from `C4_APPROVED_FINDINGS.md`.
- Create a representative rejected/invalid set from `C4_REJECTED_FINDINGS_KEY.md`.
- Parse the app's generated reports into normalized candidate findings.
- Match candidates to accepted roots by contract, function, title, impact, and description similarity, with manual adjudication for ambiguous cases.
- Label every missed approved finding by failure stage: context missing, discovery miss, dedup over-merge, verification false reject, severity downgrade, report omission.

### Tier 1: Historical Local Runs

Extend the harness to existing local outputs:

- `2025-11-megapot`
- `2025-12-panoptic`
- `2026-03-intuition`
- `2026-04-monetrix`

Use these to check whether improvements generalize beyond Olas.

### Tier 2: Public Code4rena Reports

Select 3 to 5 finalized public C4 contests with:

- Complex Solidity/EVM codebases.
- Published H/M findings.
- Available audit repos and scopes.
- Enough findings to avoid noisy single-bug benchmarks.

Run retrospective audits only after public results are finalized. Tag any known AI/V12 findings as known issues so the metric does not confuse "found by app" with "award eligible in live contest."

### Tier 3: Live No-Leakage Evaluation

After Tier 0-2 improvements are selected, run one live or freshly finalized contest under no-leakage rules:

- Freeze prompts/config before results are known.
- Do not ingest public report data.
- Log all candidates and validation outcomes.
- Compare after final judging.

## Metrics

Primary metrics:

- Unique H/M true positives: count of accepted unique High/Medium root causes matched by the app.
- H/M recall: accepted H/M roots found divided by accepted H/M roots in benchmark.
- H/M precision: valid H/M candidates divided by all final H/M candidates.
- False positive survival rate: invalid/low/QA candidates that survive validation as H/M.
- False negative stage attribution: where accepted findings were lost.

Dedup metrics:

- Cluster purity: all findings in a dedup cluster share the same root cause.
- Over-merge rate: distinct accepted roots incorrectly collapsed.
- Under-merge rate: duplicate reports kept as separate findings.
- Root-cause stability: same issue receives stable canonical ID across runs.

Validation metrics:

- False reject rate: accepted H/M killed or downgraded by verification.
- False accept rate: rejected/invalid findings retained as reportable H/M.
- Status alignment: verifier status matches judge-style invalid basis.
- Judge-readiness: report contains location, exploit path, impact, and sufficient proof.

Suggested north-star metric:

```text
validated_unique_hm_score =
  unique_HM_true_positives
  * precision_weight
  * dedup_quality_weight
```

This avoids optimizing for raw candidate volume. The app should not get credit for finding 20 variants of the same root cause or for keeping many speculative H/M candidates alive.

## Low-Refactor Research Tracks

### Track A: Instrument First

Add structured run artifacts before changing algorithms:

- Candidate finding JSON after discovery.
- Dedup clusters and pairwise duplicate decisions.
- Verification inputs, statuses, and final retain/drop decision.
- Codeblock manifest per contract: included contracts, depth, token count, source files.
- Per-finding lifecycle record from raw candidate to final report.

Expected leverage: high. Without this, every improvement is guesswork.

Refactor size: small. Mostly additive logging/serialization around existing data structures.

### Track B: Context Recall Audit

For each accepted C4 finding, answer: "Could the current codeblock have found this?"

Minimal implementation:

- Build a small script/harness that maps approved findings to contract/function/file names.
- Check whether the relevant files/functions appear in generated codeblocks.
- Classify misses as scope extraction, Slither graph, BFS depth, import fallback, inheritance/interface, token budget, docs/scope, or category selection problems.

Expected leverage: very high if many misses are context failures. No prompt can recover a bug whose critical code path is absent.

Refactor size: small to medium. Likely config and codeblock assembly improvements, not pipeline rewrite.

### Track C: Failure-Stage Replay

Replay accepted findings through the pipeline as "known candidates" to test downstream gates:

- Feed approved findings directly into dedup.
- Feed approved findings directly into verification.
- Compare which accepted findings are killed and why.

Expected leverage: high. This isolates validation false negatives from discovery misses.

Refactor size: small. Add a benchmark-only harness, not production logic.

### Track D: Dedup Upgrade Without Rewriting Discovery

Current dedup mostly buckets by `contract-function` and uses title/description similarity plus LLM pairwise checks. That can miss cross-function duplicates and can over-collapse distinct bugs in the same function.

Research options:

- Add canonical root-cause fingerprints: affected state, violated invariant, attacker action, profit/loss path.
- Dedup by semantic root cause before title similarity.
- Preserve alternates as evidence under one canonical finding instead of dropping them entirely.
- Keep "possible distinct root cause" clusters when confidence is low.

Expected leverage: medium to high. Better dedup improves signal and prevents valid variants from being thrown away.

Refactor size: small to medium. Mostly schema and prompt changes around dedup.

### Track E: Verification False-Reject Reduction

The verifier is intentionally strict. The key question is whether it is too strict against valid H/M findings.

Research options:

- Split verification into "hard invalid" and "needs evidence" instead of binary drop.
- Add contest-specific exception handling for issues commonly misclassified as governance, user error, unsupported-token, or speculation.
- Make verifier cite exact code/docs evidence for each invalidating claim.
- Add a final "appeal" pass only for findings that match high-value exploit shapes but were downgraded.

Expected leverage: very high if accepted C4 findings are currently killed during verification.

Refactor size: small. Mostly prompt/schema/status changes and retention policy.

### Track F: Pattern Coverage And Scheduling

Current discovery focuses on combined `R1` and `R2` categories, with actors and invariants as context. The full pattern library is larger than what is actively scheduled.

Research options:

- Measure which accepted findings map to active vs inactive pattern categories.
- Add a lightweight protocol classifier to select niche category prompts per contract.
- Use accepted-miss analysis to update pattern category mappings.
- Avoid increasing raw run count unless it improves unique TP per candidate.

Expected leverage: high if accepted findings map to omitted categories.

Refactor size: small. Mostly config/category scheduling changes.

### Track G: Reportability And PoC Readiness

Code4rena expects H/M submissions to have strong proof, and Solidity/EVM contests often require runnable PoCs unless an exception applies. Discovery quality should be measured partly by whether the finding can become a judge-ready report.

Research options:

- Add "proof gap" tags during validation.
- Separate "valid-looking issue" from "submission-ready H/M."
- For high-value candidates, require a concrete exploit path and minimal test strategy before final H/M retention.

Expected leverage: medium. This may not find more bugs, but should improve signal and reduce rejected submissions.

Refactor size: small to medium.

## Proposed Phases

### Phase 1: Algorithm Trace And Benchmark Design

Duration: 2 to 3 days.

Deliverables:

- Finding lifecycle diagram.
- Module-level algorithm notes.
- Benchmark schema for accepted, rejected, candidate, match, and lifecycle records.
- Initial Olas benchmark manifest.

Exit criteria:

- We can explain how a finding moves through the app.
- We know where to instrument without large refactors.

### Phase 2: Instrumentation And Olas Baseline

Duration: 3 to 5 days.

Deliverables:

- Structured JSON artifacts for discovery, dedup, verification, and final findings.
- Olas baseline report with TP, FP, FN, dedup, and validation metrics.
- Failure-stage attribution for every approved Olas H/M finding.

Exit criteria:

- We know whether the largest performance loss is context, discovery, dedup, validation, or severity/reportability.

### Phase 3: High-Leverage Experiments

Duration: 1 to 2 weeks.

Run only experiments that target the top failure stages from Phase 2.

Likely experiments:

- Codeblock inclusion fixes if accepted bugs lack context.
- Pattern scheduling changes if accepted bugs map to missing categories.
- Verification retention changes if accepted bugs are falsely killed.
- Dedup root-cause fingerprinting if distinct issues are merged or duplicates survive.

Exit criteria:

- Each experiment has before/after metrics on Olas and at least one additional historical benchmark.
- Keep only changes that materially improve unique H/M TP, precision, or false-reject rate.

### Phase 4: Cross-Benchmark Validation

Duration: 3 to 5 days.

Deliverables:

- Results across 3 to 5 benchmarks.
- Recommended production patch set.
- "Do not pursue" list of ideas that did not move the needle.

Exit criteria:

- Improvements are not Olas-specific.
- Refactor scope is still small enough to land safely.

### Phase 5: Live Or Fresh Contest Evaluation

Duration: one contest cycle or one freshly finalized contest.

Deliverables:

- No-leakage run config and logs.
- Post-judging comparison.
- Updated performance scorecard.

Exit criteria:

- Evidence that improvements transfer to a realistic contest workflow.

## Definition Of A 10x Improvement

Because benchmark baselines may vary by protocol, use two definitions:

1. Absolute output: roughly 10x more accepted unique H/M findings per complex protocol at comparable human review time.
2. Signal-adjusted output: a large increase in unique H/M true positives while preserving or improving H/M precision and dedup quality.

A change that creates many more candidates but lowers precision is not a 10x improvement. A change that finds one extra valid Medium while doubling false positives is probably not worth it. A change that recovers several accepted H/M findings by fixing one pipeline failure mode is exactly the kind of change this project should hunt.

## What Not To Do

Avoid:

- A broad rewrite of the whole audit pipeline.
- Adding many more LLM rounds without proof of unique TP gain.
- Optimizing for raw finding count.
- Prompt churn without benchmark artifacts.
- Treating all public C4 findings as equal without grouping by root cause and eligibility.
- Letting retrospective benchmark leakage contaminate live evaluation.

## Final Deliverables

The deep dive should end with:

- A concise technical writeup of how the algorithm works.
- A benchmark harness and reproducible baseline.
- A ranked list of top performance bottlenecks.
- A small proposed patch set with expected impact.
- Before/after metrics across local C4-style benchmarks.
- A list of rejected research ideas and why they did not move the needle.
- A recommendation for the next live Code4rena evaluation.

## References

- [Code4rena submission guidelines](https://docs.code4rena.com/competitions/submission-guidelines)
- [Code4rena signal metrics](https://docs.code4rena.com/roles/signal)
- [Code4rena awarding model](https://docs.code4rena.com/awarding)
- [Code4rena bounty severity criteria](https://docs.code4rena.com/bounties/bounty-criteria)
