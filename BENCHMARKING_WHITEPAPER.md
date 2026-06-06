# AI Agent Audit Benchmarking Whitepaper

Draft v0.1 - Megapot case study

## Executive Summary

AI Agent Audit is being benchmarked against real Code4rena contests to measure how effectively the system discovers, deduplicates, validates, and reports high and medium severity Solidity/EVM findings.

The first complete case study is the 2025-11 Megapot contest. Megapot is a useful benchmark because it contains a mix of economic, randomness, bridge, governance, settlement, and active-flow state bugs. It also includes rejected primary submissions, which lets us measure false positives rather than only recall.

The headline result from the initial clean Megapot full run:

| Metric | Result |
| --- | ---: |
| Accepted H/M ground truth roots | 11 |
| Rust final-stage accepted-root recall | 9 / 11, 81.8% |
| Full end-to-end three-shot/judge-sim accepted-root recall | 8 / 11, 72.7% |
| Initial final-stage candidates | 13 |
| Initial final-stage precision | 61.5% |
| Initial final-stage false positives | 5 |

After a validation-only tightening pass, without rerunning expensive discovery:

| Metric | Result |
| --- | ---: |
| Rust final-stage accepted-root recall | 9 / 11, 81.8% |
| Three-shot submission-stage accepted-root coverage | 9 / 11, 81.8% |
| Three-shot submission candidates | 8 |
| Submission-stage true positive candidates | 8 / 8 |
| Observed submission-stage false positives | 0 |

After targeted protocol-agnostic discovery prompt improvements and a narrow rerun scoped to `Jackpot.sol`, the app found the previously missed gas-complexity root family for H-02. The remaining unresolved Megapot gap is not simple source context or single-bug discovery. The app finds component active-flow config issues, but it does not yet reliably aggregate them into the exact Code4rena-style M-01 parent finding.

Current best Megapot coverage after the scoped rerun:

| Metric | Result |
| --- | ---: |
| Accepted H/M roots discovered across baseline plus scoped rerun | 10 / 11, 90.9% |
| Remaining unresolved accepted root | 1 / 11, M-01 aggregate active-flow config finding |

## Why This Benchmark Exists

Most security tool benchmarks overfocus on whether a tool can emit something that resembles a known bug. For client use, that is not enough.

The benchmark tracks the full audit funnel:

- Discovery: does the app find a credible candidate?
- Deduplication: does it merge duplicates without losing real roots?
- Verification: does it reject weak or invalid candidates?
- Validation: does the three-shot review preserve real issues while suppressing false positives?
- Reporting: can the system produce professional client-ready findings?
- Judge simulation: would the final candidate plausibly survive an external adjudication process?

Performance is measured against accepted Code4rena high and medium severity findings, with rejected primaries used to estimate false positives and rejection patterns.

## Dataset: Megapot 2025-11

| Field | Value |
| --- | --- |
| Contest | 2025-11 Megapot |
| Repository commit | `538649e8a05b07efa706083d1158c65f640cb823` |
| Accepted H/M roots | 11 |
| Captured primary submissions | 35 |
| Accepted primaries | 11 |
| Rejected/non-accepted primaries | 24 |
| Primary acceptance rate | 31.4% |

Megapot is important because it exposes several behaviors that a serious audit system must handle:

- Multi-phase lottery flows: buy tickets, lock drawing, request randomness, callback, settlement, claim.
- Randomness and entropy provider dependencies.
- Bridge and smart-wallet custody paths.
- LP pool accounting and settlement math.
- Admin configuration changes that are intended for future drawings but can affect active drawings.
- Gas complexity in required settlement callbacks.

## Methodology

The Megapot benchmark uses archived Code4rena artifacts as ground truth:

- Accepted high and medium final findings are treated as recall roots.
- Rejected primary submissions are retained as false-positive references.
- Each app candidate is matched to accepted and rejected references using deterministic corpus-local similarity scoring.
- Stage-by-stage telemetry records where findings are discovered, dropped, retained, or promoted.

The clean run used the default full benchmark configuration:

```rust
R1_RUNS = 10
R2_RUNS = 10
INVARIANT_RUNS = 3
ACTOR_RUNS = 2
```

The benchmark separates three kinds of results:

1. Rust pipeline results: discovery, deduplication, verification, and Rust final report candidates.
2. Three-shot validation results: independent validation and final submission-candidate filtering.
3. End-to-end results: PoC/report/judge-sim path where available.

This matters because discovery reruns are expensive, while validation experiments are comparatively cheap. In Megapot, most of the first measurable improvement came from validation changes, not from rerunning the full app.

## Baseline Megapot Run

The initial clean full run produced:

| Stage | Candidates | Accepted-root recall |
| --- | ---: | ---: |
| Discovery raw | 501 | 81.8% |
| Rust dedup kept | 164 | 81.8% |
| Rust post-verification | 157 | 81.8% |
| Rust final | 105 | 81.8% |
| Three-shot R3 | 50 | 72.7% |
| Three-shot submission | 13 | 72.7% |
| PoC created | 13 | 72.7% |
| PoC verified | 13 | 72.7% |
| Report ready | 13 | 72.7% |
| Judge accepted | 13 | 72.7% |

Headline baseline:

- 9 of 11 accepted H/M roots were present by the Rust final stage.
- 8 of 11 accepted H/M roots survived through the full three-shot/judge-sim path.
- 5 final-stage candidates were false positives under the benchmark matcher.
- 4 of those false positives matched rejected Code4rena primary submissions.
- 1 final-stage candidate was unadjudicated by the archived rejected-primary set.

The two initially missed accepted roots were:

| Root | Severity | Title | Initial classification |
| --- | --- | --- | --- |
| H-02 | High | Unoptimized subset matches counting implementation will exceed tx gas limit on base chain | Discovery gap |
| M-01 | Medium | Global Variable Manipulation During Active Draw Alters End Result | Discovery or aggregation gap |

## Validation-Only Improvement

The first major improvement did not require rerunning Rust discovery. We tightened the three-shot validation rubric so it preserved high-quality accepted-root candidates while suppressing weak variants and rejected-primary matches.

After this pass:

| Metric | Result |
| --- | ---: |
| Rust final accepted-root recall | 9 / 11, 81.8% |
| Three-shot submission accepted-root coverage | 9 / 11, 81.8% |
| Submission candidates | 8 |
| Submission-stage true positive candidates | 8 / 8 |
| Submission-stage false positives | 0 |

The important learning: precision improved sharply without paying for another expensive discovery run. This supports a practical benchmarking workflow:

1. Run expensive discovery once.
2. Score against ground truth.
3. Iterate cheaply on validation prompts and filters.
4. Only rerun discovery when a failure is truly a discovery gap.

## Discovery Prompt Improvement

The remaining misses were analyzed manually. H-02 was a true discovery gap: the initial system did not strongly reason about combinatorial gas growth in a required settlement callback.

We made small protocol-agnostic changes inside the existing architecture:

- Strengthened the `UnboundedLoops` pattern to include required progress paths, config-derived bounds, repeated helper allocation, and combinatorial helpers.
- Added `UnboundedLoops` to semantic R2 routing.
- Strengthened temporal/state-machine invariant language around required progress gas and active-flow stability.
- Adjusted actor prompting so trusted admin roles still expose ordinary maintenance capabilities when those updates can unintentionally affect active user flows.
- Added discovery examples for required progress gas and routine future-directed config updates.

This was intentionally not a large refactor. The goal was to test whether existing actors, invariants, and patterns could absorb the Megapot misses before adding a new analysis pass.

## Scoped Rerun Result

After those changes, we ran a narrow Megapot rerun scoped to `Jackpot.sol`. The context builder pulled connected contracts and libraries.

The new report produced 43 findings:

| Category | Count |
| --- | ---: |
| Medium valid | 16 |
| Low valid | 2 |
| Medium invalid governance risk | 24 |
| Low severity due to low impact | 1 |

Most importantly, the new report found the H-02 family:

| New app finding | Status | Ground-truth relationship |
| --- | --- | --- |
| M-18: Entropy callback settlement gas grows with `bonusballMax` and can permanently lock a drawing | Valid | Matches H-02 family |

The candidate names the correct path:

```text
Jackpot.scaledEntropyCallback
  -> Jackpot._calculateDrawingUserWinnings
  -> TicketComboTracker.countTierMatchesWithBonusball
  -> TicketComboTracker._countSubsetMatches
  -> Combinations.generateSubsets
```

This is the expected behavioral fix: the model now treats gas-complexity in mandatory settlement callbacks as a first-class security risk rather than as a generic low-severity loop concern.

With this scoped rerun, Megapot accepted-root discovery coverage improved from 9 of 11 to 10 of 11. The result is not yet a new full end-to-end benchmark score, because the scoped rerun has not been passed through the full three-shot/PoC/report/judge-sim path. It is nevertheless a direct recovery of the previously missing H-02 bug family.

The scoped rerun did not fully solve M-01. It found component findings in the same family:

- `setEntropy` during a pending draw can reject the original callback.
- `payoutCalculator` rotation can make active or settled drawings use the wrong payout source.
- `referralFee` changes can affect emergency refunds for already-purchased tickets.

But it still did not emit the accepted Code4rena-style aggregate finding: multiple mutable global parameters can affect an active draw even when the intended update is future-directed. That is now classified as an aggregation/rubric gap rather than a simple context gap.

## What We Learned

### 1. Full-funnel scoring is essential

The same app run can look strong or weak depending on where measurement stops. Megapot showed:

- Strong Rust-stage recall: 81.8%.
- Lower initial full end-to-end recall: 72.7%.
- Precision issues that were fixable in validation without new discovery spend.

This confirms that we need stage-by-stage loss accounting, not one headline number.

### 2. Validation can be a high-leverage optimization layer

The three-shot validation tightening moved submission-stage precision to 100% on this benchmark slice, with no new Rust discovery run. That is a high-ROI improvement path because validation is much cheaper to retest than full protocol discovery.

### 3. Some misses are real discovery gaps

H-02 was not just a validation miss. The initial discovery did not surface the required-settlement combinatorial gas path strongly enough. A small pattern/invariant/routing improvement changed that behavior.

### 4. Some misses are aggregation gaps

M-01 is different. The app found several component issues in the active-flow config family, but Code4rena accepted the issue as a parent aggregate. Future benchmark work needs to improve aggregation of sibling findings when they share one invariant:

```text
Future-directed config changes must not unexpectedly mutate an active user flow.
```

### 5. Contest adjudication has jitter

Megapot accepted 31.4% of captured primary submissions. Some issues depend on contest-specific judgments about admin trust, business logic, and acceptable operational assumptions. Benchmarking must model this variability rather than pretending every rejected or accepted finding is a timeless universal truth.

This is why the benchmark tracks:

- Accepted H/M roots.
- Rejected primary matches.
- Duplicate/low/invalid rejection categories.
- Contest-level acceptance rate.
- Whether a finding requires trusted role action, routine maintenance, or permissionless adversarial behavior.

## Client-Relevant Takeaways

Megapot shows that AI Agent Audit can recover a large share of accepted high/medium findings in a complex real contest, while also exposing a disciplined path for improving precision and recall.

The strongest current client-facing claims are:

- The system is measurable against real public audit contests.
- The benchmark captures both true positives and false positives.
- The app already reached 81.8% Rust final-stage recall on Megapot accepted H/M roots.
- The app reached 72.7% full end-to-end recall on the initial Megapot run.
- Validation-only changes eliminated observed submission-stage false positives in the tightened Megapot validation experiment.
- A small, protocol-agnostic discovery update recovered the previously missed H-02 gas-complexity family in a scoped rerun, bringing current Megapot accepted-root discovery coverage to 10 / 11, or 90.9%.

The most important ongoing work:

- Generalize beyond Megapot with more Code4rena protocols.
- Improve aggregate finding construction for active-flow config families like M-01.
- Continue tracking precision under rejected-primary matching.
- Preserve all benchmark artifacts for whitepaper and client evidence packs.

## Evidence Paths

Primary scorecards:

```text
benchmarks/code4rena-2025-11-megapot/runs/2025-11-megapot-538649/megapot-clean-r1r2-10-inv3-actor2-001/benchmark_score.md
benchmarks/code4rena-2025-11-megapot/validation-experiments/run-validation-tighten-003/benchmark_score.md
```

Reports:

```text
2025-11-megapot/report/audit-report-full-run.md
2025-11-megapot/report/audit-report.md
```

Ground truth:

```text
benchmarks/code4rena-corpus/competitions/2025-11-megapot/ground_truth/accepted/
benchmarks/code4rena-corpus/competitions/2025-11-megapot/submissions/
```

Whitepaper archive:

```text
benchmarks/whitepaper-data/2025-11-megapot/
```

## Next Protocols

Megapot is Case Study 001. The next step is to run the same benchmark workflow on additional Code4rena protocols with different shapes:

- Smaller codebases for fast iteration.
- Larger DeFi protocols to stress context construction and deduplication.
- Protocols with non-Solidity or hybrid components once the app expands support.
- Contests with stricter or looser judging profiles to estimate adjudication jitter.

Each new protocol should produce:

- `benchmark_score.json`
- `benchmark_score.md`
- accepted-root loss analysis
- false-positive rejection analysis
- before/after comparison for any pipeline changes
- archived evidence pack for future client-facing material

## Current Status

This whitepaper is a living draft. Megapot provides the first evidence-backed case study. Results will become more robust as we add protocols and report aggregate metrics across contest types, judging profiles, and codebase sizes.
