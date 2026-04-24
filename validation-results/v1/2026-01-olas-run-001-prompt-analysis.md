<!-- PROMPT ANALYSIS STATUS: COMPLETE -->
# Prompt Analysis

- Benchmark: `2026-01-olas`
- Prompt version tested: `v1`
- Run id: `run-001`

## Artifact List Reviewed

- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-prompts/v1.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-results/v1/2026-01-olas-run-001.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-runs/v1/2026-01-olas-run-001.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-results/v1/2026-01-olas-run-001-worker-log.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2026-01-olas/report/audit-report.md`
- `/Users/apmfree/.ai-agent-audit-validation-truth/2026-01-olas.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/C4_APPROVED_FINDINGS.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/APPROVED_FINDINGS_KEY.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/C4_LOW_QA_INVALID_FINDINGS.md`

## Current Scored Metrics Summary

- H/M confusion matrix: `TP=24`, `FP=23`, `TN=128`, `FN=19`
- H/M recall: `55.8%`
- H/M precision: `51.1%`
- H/M specificity: `84.8%`
- H/M abstentions / `Needs Review`: `5 / 199`
- Unique approved H/M findings present in the report: `15`
- Unique approved H/M findings accepted as `Valid` with `High` / `Medium` severity: `10`
- Present-root recall: `66.7%`
- End-to-end unique recall: `43.5%`

## Executive Summary Of The Biggest Validation Failure Modes

`v1` failed in both directions for the same structural reason: it did not keep bug existence, root-cause quality, and H/M severity proof separate enough.

The prompt was too permissive on the false-positive side because it:

- explicitly said "When in doubt: Valid"
- treated "missing standard protection" as near-default H/M
- lacked a gate for "real symptom vs independent root cause"
- did not ask hard enough whether the attack was economically meaningful, supported, and independently material

The prompt was too conservative on the false-negative side because it:

- gave too much credit to nominal safeguards that existed only in name, on another path, or in fail-open form
- treated realistic timing, sequencing, and market conditions as grounds for downgrading otherwise real H/M issues
- used documentation, governance, configuration, or "known issue" reasoning too aggressively against runtime failures
- accepted some approved H/M rows as real bugs but still pushed them down to `Low / QA`

The core lesson is that the next prompt should not simply "accept less" or "accept more." It should:

- reject unsupported, derivative, bounded, or uneconomic H/M claims more reliably
- promote real fail-open, liveness, accounting, authorization, and pricing failures when the runtime path remains materially exposed

## False Positives / Over-Severity

### Main Reasons Low / QA / Invalid Findings Were Mistakenly Accepted As H/M

1. Unsupported or non-standard asset behavior was treated as in-scope H/M too often.

- Representative rows: `M-2`, `H-9`, `M-10`, `M-128`, `M-130`, `M-134`, `M-135`, `M-136`, `M-137`
- Common pattern: non-standard `approve` / transfer / compatibility semantics were treated like protocol-critical vulnerabilities even when the behavior depended on assets with special semantics that were not clearly first-class supported.
- Relevant judge-rationale pattern from `C4_LOW_QA_INVALID_FINDINGS.md`: issues depending on unsupported, OOS, or otherwise special-case token behavior were repeatedly classified as invalid or non-rewardable.

2. Derivative symptoms were promoted as independent H/M roots.

- Representative rows: `H-84`, `M-119`, `H-120`, `M-151`
- Common pattern: a report identified a real weakness near an execution surface, but the judged H/M issue lived deeper in the validation or pricing logic. The prompt did not force the worker to ask whether the report described the actual root cause or only a downstream symptom.
- Relevant judge-rationale pattern: `F-1` was judged low because the real issue lived in the shared price-validation logic, not in every call site that could have been patched defensively.

3. Economically irrational or operationally minor griefing was promoted into H/M.

- Representative rows: `H-1`, `H-35`, `H-36`, `H-84`
- Common pattern: if a path was permissionless and could revert or delay a workflow, `v1` leaned too quickly toward H/M without asking whether the attacker had a rational incentive, whether the effect was persistent, or whether existing parameters already bounded the grief.
- Relevant judge-rationale patterns:
  - `F-5`: same-block donation griefing was treated as low because it required attacker spend and could be deterred operationally
  - `F-9`: attacker economics mattered; a theoretically possible loop was still rejected when the attack was not rationally incentivized

4. Deployment, configuration, or trusted-component assumptions passed through as live H/M bugs.

- Representative rows: `H-23`, `H-49`, `M-151`
- Common pattern: the prompt's governance and speculation filters were not strong enough to stop reports that depended on special deployment posture, trusted-component semantics, or configuration assumptions rather than a live in-scope runtime failure.
- Relevant judge-rationale patterns:
  - `F-10`: by-design and unused-component behavior should not be promoted into H/M
  - `F-14`: misreading the semantics of a pricing component led to an invalid oracle-staleness claim
  - `F-20`: configuration drift that belongs to known-issue or operator-responsibility space should not be treated as a live H/M code exploit

5. Evidence quality did not sufficiently constrain H/M acceptance.

- Representative recurring pattern from the judge rationales: `F-11`, `F-15`, `F-17`, and `F-19` were rejected because the PoC or exploit evidence was mandatory and missing or insufficient.
- Prompt implication: when a report's H/M case depends on exploitability evidence rather than obvious code proof, the worker needs explicit permission to withhold H/M rather than auto-promote on intuition.

## False Negatives / Under-Severity

### Main Reasons True H/M Findings Were Mistakenly Marked Low / QA, Invalid, Or Needs Review

1. Nominal safeguards were over-credited even when they failed open, were non-binding, or did not constrain the actual execution path.

- Approved roots and rows:
  - `F-204`: `H-76`, `H-124`, `H-125`, `M-123`
  - `F-469`: `H-114`, `H-116`, `H-117`
  - `F-6`: `H-195`
  - `F-432`: `H-90`
- Failure mode: the prompt rewarded the mere presence of oracle checks, slippage logic, or pricing validation language even when the judged issue was that those protections did not actually bind execution or preserve liveness.
- Reusable lesson: "guard exists" is not the same as "guard works on this path."

2. Realistic timing or sequencing conditions were mistaken for low likelihood.

- Approved roots and rows:
  - `F-104`: `M-65`
  - `F-166`: `M-108`
  - `F-206`: `M-31`
  - `F-67`: `M-156`, `H-157`, `M-162`
- Failure mode: the prompt downgraded or rejected issues when exploitation required common runtime conditions such as stale state, ordinary timing windows, or realistic sequencing. Those conditions were treated like speculative edge cases even though the approved findings considered them live and meaningful.
- Reusable lesson: timing-sensitive does not mean implausible if the condition is permissionless and realistic.

3. Real bugs were sometimes accepted, but their severity was still pushed down to `Low / QA`.

- Approved roots and rows:
  - `F-469`: `H-114`, `H-116`, `H-117`
  - `F-6`: `H-195`
- Failure mode: the worker saw a real issue but treated it as only low-severity because the prompt did not clearly state when a fail-open guard, liveness break, or exploitable pricing flaw still qualifies as H/M despite conditions or partial safeguards.
- Reusable lesson: once bug existence is established, severity must be judged on the real exposed path, not on the fact that some nominal protection also exists elsewhere.

4. Runtime binding failures were misclassified as governance, setup, or design concerns.

- Approved roots and rows:
  - `F-67`: `M-156`, `H-157`, `M-162`
  - `F-8`: `H-45`
  - `F-175`: `M-62`, `M-121`
  - `F-362`: `M-205`
- Failure mode: the prompt's governance / documentation / configuration language encouraged the worker to dismiss issues whenever the system looked configurable, manually coordinated, or policy-driven, even when the judged issue was that the code still failed to bind values, preserve state, or maintain accounting under expected operation.
- Reusable lesson: "admin can configure something" does not save a runtime invariant if the code path is still exposed after correct setup.

5. `Needs Review` was used where the approved H/M case was already strong enough.

- Representative rows: `M-97`, `M-100`, `M-194`
- Failure mode: instead of resolving whether the runtime path was materially exposed, the worker abstained. In this benchmark, those abstentions hurt recall without a compensating precision gain.
- Reusable lesson: use `Needs Review` for true ambiguity, not for already-established root families where only the prompt's severity language is indecisive.

## Severity-Calibration Mistakes

The most important calibration errors were directional:

- Over-severity:
  - non-standard asset incompatibilities were treated like protocol H/M
  - nuisance griefing and bounded operational disruption were treated like major DoS
  - downstream symptoms of deeper bugs were treated as separate H/M findings
  - pricing or slippage complaints with meaningful existing bounds were over-promoted

- Under-severity:
  - fail-open pricing protections were treated as if they were effective safeguards
  - stale-state, stale-authorization, and binding failures were treated as merely operational
  - realistic sequencing requirements were treated as if they made the issue remote or speculative
  - liveness and accounting breaks were underweighted when they were not "instant total drain" style issues

The prompt needs a clearer distinction between these categories:

- `Invalid`: bug does not exist, is OOS, depends on unsupported behavior, is by-design, is purely documentation, is speculative, or lacks the minimum evidence required to claim the reported exploit class
- `Low / QA`: real issue, but independently minor, bounded, derivative, operationally low impact, or economically irrational
- `Medium` / `High`: real, in-scope, independently material, and realistically exploitable under current code and expected operation

## Gate-Level Mistakes

### False-Positive Side

- Pre-gate sanity check:
  - traced whether "a bug exists" but not whether the report identified the right root cause or an independent H/M issue
- Impact gate:
  - too eager to map "real issue" or "missing standard protection" into `Medium`
- Likelihood gate:
  - did not ask whether the attack was rational, persistent, or materially low-cost
- Governance / speculation gates:
  - not strict enough on deployment, trusted-component, and support-assumption dependent reports
- Safeguards gate:
  - treated the absence of one ideal mitigation as enough to accept H/M, even when existing runtime bounds likely kept the issue below the bar
- Final tie-breaker:
  - the explicit "When in doubt -> Valid" instruction overrode the intended conservatism of the gate system

### False-Negative Side

- Pre-gate invariant check:
  - leaned too hard on explicit documentation instead of real runtime requirements
- Impact gate:
  - undervalued liveness, accounting, and fail-open pricing issues that were materially exposed
- Likelihood gate:
  - discounted realistic timing and sequencing conditions too aggressively
- Governance / documentation gates:
  - used operator-responsibility reasoning to dismiss bugs that still occurred after correct expected use
- Safeguards gate:
  - gave too much credit to nominal protections that were stale, incomplete, bypassable, or ineffective on the actual path

## Protocol-Agnostic Lessons

1. Separate three questions every time:
   - Does the bug exist?
   - Is the reported root cause independently in-scope and correctly framed?
   - Does the actual runtime impact justify H/M?

2. A real bug is not automatically a High/Medium finding.

3. Missing a standard protection is not automatically H/M. The worker must still check whether existing runtime bounds materially cap the risk.

4. A named safeguard only counts if it constrains the exact execution path at issue. Fail-open, stale, incomplete, or wrong-path safeguards should not drive downgrades.

5. A downstream symptom, alternate patch site, or weaker restatement of a deeper vulnerability should not become an independent H/M unless it creates its own independent material impact.

6. Timing-sensitive, sequencing-sensitive, or market-sensitive does not mean low likelihood if the conditions are realistic and permissionless.

7. Unsupported external behavior, non-standard token semantics, or deployment misconfiguration should not be treated as protocol H/M unless the protocol explicitly supports that behavior or depends on it in normal operation.

8. Economically irrational griefing often belongs in `Low / QA` or `Invalid`, even if the revert path is real.

9. Documentation can explain intent, but it does not rescue a runtime path that still fails under intended use.

10. `Needs Review` should be used sparingly. It is not a substitute for severity calibration.

## Prompt-Revision Recommendations

### Recommendations Aimed At Recall Improvement

1. Add an explicit rule that nominal guards do not count if they are fail-open, non-binding on the path, stale, or materially bypassable.

2. Clarify that realistic timing, sequencing, or market conditions do not automatically make a finding speculative or low likelihood.

3. Clarify that correct admin setup does not defeat a finding if the runtime path still fails after expected use.

4. Add explicit examples showing that liveness, pricing, stale-state, stale-authorization, and accounting issues can still be H/M even when they are not "instant drain all funds" bugs.

5. Tighten use of `Needs Review` so established root families with clear runtime exposure are resolved rather than punted.

### Recommendations Aimed At Precision Improvement

1. Add a dedicated root-cause / independence gate that rejects or downgrades derivative symptoms, alternate patch sites, and non-independent restatements.

2. Tighten supported-asset and supported-integration assumptions so non-standard token behavior does not enter the H/M bucket by default.

3. Add an explicit economic-plausibility check for griefing and DoS claims.

4. Replace "When in doubt -> Valid" with a balanced tie-breaker:
   - uncertain bug existence -> `Invalid`
   - real bug but independent H/M not proven -> `Low / QA` or `Needs Review`
   - real bug with independently proven material exposure -> `Medium` / `High`

5. Clarify that bounded or operationally minor issues remain `Low / QA` even if they involve a real bug.

6. Explicitly allow evidence-quality downgrades when the report's H/M claim depends on PoC-grade exploit proof that is missing or insufficient.
