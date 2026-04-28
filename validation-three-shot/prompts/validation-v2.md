# H/M Validation Checklist

Your task: decide whether a reported finding is `Valid` and, if valid, whether it deserves `High`, `Medium`, or only `Low / QA`.

Primary objective: maximize recall of real approved-root H/M findings while keeping H/M precision near or above 50%.

That means:
- do not reject or downgrade a real H/M runtime bug merely because it is conditional, timing-sensitive, market-sensitive, or triggered during expected admin / keeper / governance workflows
- do not accept unsupported, speculative, derivative, or operationally minor reports as H/M just to boost recall

## Mandatory Stage 0: Scope And Known-Issue Screen

Before any bug-existence, severity, or exploitability analysis, read the benchmark context artifacts listed in the active worker prompt. Those files are injected from `validation-three-shot/config.yaml` via `context_docs` and are authoritative for client-declared scope and known-issue filtering.

The configured context may include README files, scope files, docs files, standalone prior-finding files, embedded V12/prior-finding context, and shared workflow checklists. Do not assume a fixed filename convention beyond the explicit file list provided by the controller.

If some optional files are absent from the configured context, proceed with the available benchmark artifacts, but do not assume scope from prose outside the configured context list and configured source root.

If a finding can already be excluded at this stage as out-of-scope, client-known, duplicate-of-V12, unsupported-asset behavior, or deployment / setup only, exclude it immediately instead of carrying it deeper into severity analysis.

## Evidence Hierarchy

Evaluate in this order:

1. Current code path and live runtime invariant
2. Independent root cause and exploit path
3. Material H/M impact under expected operation
4. Supporting PoC or concrete exploit reasoning

Never collapse these into one question. A real bug can still be `Low / QA`, and a severe-sounding report can still be `Invalid`.

## Core Validation Principles

1. Separate bug existence, root-cause independence, and severity.
2. A real bug is not automatically an H/M finding.
3. A named safeguard only counts if it materially constrains the exact execution path at issue.
4. Realistic timing, sequencing, stale-state windows, and ordinary market conditions do not automatically make an issue low likelihood.
5. A workflow being owner-only, governance-triggered, or maintenance-triggered does not make a finding low by default if permissionless actors can still exploit the path once that workflow is used.
6. Unsupported ERC20 or exotic token semantics are out of scope unless the benchmark docs explicitly say the protocol supports them.
7. Unsupported integrations, future component failures, and deployment / setup mistakes are not current H/M code vulnerabilities by default.
8. In raw validation, do not dedupe report findings against other report findings. Evaluate each finding on its own code path, bug existence, and H/M impact.
9. Use `Needs Review` sparingly. It is for genuine unresolved ambiguity after tracing the code, not for severity indecision on an already-proven bug family.

## Pre-Gate Sanity Check: Verify The Bug Exists

Before any severity analysis:

- locate the exact code path mentioned in the report
- trace execution step by step
- verify the claimed invariant is real and relevant to the current path
- confirm the harmful state is reachable under current code

Mark `Invalid` if:
- the code path does not exist
- the report describes impossible execution flow
- the invariant is invented or contradicted by the code
- the PoC or reasoning does not actually produce the claimed failure mode

## Gate 1: Scope, Prior-Finding, And Per-Finding Materiality

Before deciding whether a report is an independent H/M issue, explicitly check the configured client-declared scope and known-issue artifacts listed in the active worker prompt.

Treat those files as authoritative context for benchmark scope and V12 / client-known duplicate filtering.

V12 duplicate / out-of-scope rule:

```text
Same root cause as V12?
├─ NO → Submit
└─ YES → Same impact?
    ├─ NO → Submit
    └─ YES → Materially different exploit, victim, value flow, or mitigation?
        ├─ YES → Submit only if the delta is clear in one sentence
        └─ NO → Exclude (known V12 / prior-finding overlap)
```

If the report is already captured in the benchmark V12/prior-findings context with the same root cause and same practical harm, exclude it as duplicate / out of scope for this benchmark unless there is a clear material difference in exploit path, victim, value flow, impact class, or mitigation.

Severity mismatch alone is not a material difference. If V12 is High and the candidate is Medium, or V12 is Low and the candidate is High/Medium, still exclude when the root cause and practical harm are the same.

If the root is the same but the exploit path, affected victim/value flow, impact class, or mitigation is materially different, the finding can still be in scope and should be analyzed on the merits.

After V12 / client-known filtering, stop deduping against other report findings.

Evaluate each report finding as if it were unique.

Do not reject or downgrade a report because:
- another finding in the same family already exists
- another finding seems like a stronger or more canonical articulation
- multiple findings might later be merged in cleanup

Only judge whether this finding itself proves:
- a real bug on the described code path
- a realistic exploit or failure path under current code
- material H/M impact on its own merits

It is acceptable in raw validation to keep multiple same-family findings if each one independently satisfies the H/M standard. A later cleanup worker can merge duplicates more carefully. Early deduping is disallowed here because it creates avoidable false negatives.

## Gate 2: Supported-Behavior Check

Unsupported ERC20 behavior is out of scope unless the benchmark docs explicitly say that exact token class or semantic must be supported.

Mark `Invalid` or `Low / QA` if the H/M claim depends on:
- unsupported non-standard ERC20 behavior
- USDT-style missing return values or forced zero-reset allowance quirks when support is not explicitly declared in the configured benchmark context docs
- fee-on-transfer / rebasing / exotic-decimals semantics that are not explicitly supported
- view-only, event-only, or cosmetic inconsistencies without functional harm

Do not upgrade unsupported-token incompatibility into H/M merely because the token is common or widely used elsewhere. The question is whether this benchmark explicitly requires support for that token behavior.

Treat only explicitly supported assets and integrations as in-scope for this gate. Do not dismiss a bug just because the path touches an external system if the benchmark docs show the protocol currently depends on that exact supported path in normal operation.

## Gate 3: Root-Cause Independence

A report is independently valid only if it identifies a live broken invariant or unsafe assumption, not merely a downstream symptom of another already-known root.

Mark `Invalid` or `Low / QA` if:
- it is only an alternate patch location for the same already-known V12 issue
- it describes a consequence without a distinct exploitable condition
- it depends entirely on another invalid premise
- it is only a generalized "missing validation" claim with no demonstrated harmful state

Do not reject merely because a later cleanup pass may combine same-family findings. This gate is about whether the finding's own root and exploit reasoning are real, not whether the final report count should be lower.

## Gate 4: Safeguards And State Transitions

Identify every relevant existing safeguard:
- access control
- whitelists / allowlists
- caps / limits
- price or slippage checks
- paused states
- oracle freshness checks
- epoch / checkpoint / settlement gates
- queue, nonce, or replay protection
- accounting reconciliation

Then ask whether the safeguard actually blocks the described exploit path.

Do not downgrade merely because a safeguard exists by name. Downgrade or reject only when the safeguard constrains the exact harmful state transition.

## Gate 5: Exploitability And Likelihood

For H/M, require a realistic path under current code, but do not require certainty or a public mempool race if the protocol flow naturally exposes the state.

Usually H/M-compatible:
- permissionless or economically rational attacker actions
- sandwich / timing opportunities around public settlement, mint, burn, claim, rebalance, bridge, or liquidation flows
- stale-state windows where a normal keeper / admin / user action finalizes the harmful state
- missing postcondition checks after external protocols or asynchronous flows
- accounting divergence that compounds or blocks core flows

Usually Low / QA or Invalid:
- requires privileged role compromise
- requires trusted admin/operator to act maliciously or contrary to docs
- requires an irrational attacker to lose material value only to grief
- requires unsupported deployment choices
- requires a future integration not present in current code/scope

## Gate 6: Impact Classification

High is appropriate when realistic exploitation can cause:
- direct theft or loss of user funds
- large-scale value extraction from protocol/users
- permanent or near-permanent lock of funds
- systemic insolvency or unbacked accounting
- repeatable extraction from a core asset pool

Medium is appropriate when realistic exploitation can cause:
- material but bounded fund loss
- material DoS or liveness failure of a core workflow
- incorrect accounting that blocks withdrawals, redemptions, bridging, settlement, or claims
- unauthorized use or bypass that creates meaningful financial or protocol risk
- loss socialized across users, vault, pool, strategy, or protocol accounting

Low / QA is appropriate when:
- no meaningful asset or core workflow impact
- temporary nuisance with easy self-recovery
- purely informational, event, UI, or documentation mismatch
- theoretical concern without a realistic path
- only minor dust or rounding loss that cannot scale

## Gate 7: Trusted Roles, Governance, And User Error

Trusted roles are trusted unless the benchmark scope says otherwise.

Reject or downgrade if the issue only exists because a trusted role:
- acts maliciously
- ignores documented responsibilities
- picks obviously bad parameters
- deploys incorrectly
- withholds expected maintenance forever

Do not reject if:
- a trusted role acts according to spec and the code still permits a harmful state
- a normal role-triggered workflow exposes a permissionless exploit path
- the bug is missing validation, missing postcondition checks, stale accounting, or unsafe sequencing in a normal privileged workflow

User error is not H/M unless the protocol is supposed to protect users from that exact mistake and the code failure creates material loss beyond ordinary misuse.

## Gate 8: By-Design And Documentation

If docs explicitly describe the behavior as intended and the behavior does not violate another invariant or H/M security expectation, reject or downgrade.

Do not accept "by design" as a defense when:
- docs are silent or ambiguous
- behavior contradicts the protocol's accounting model
- the claimed design creates unbounded or material loss
- the design depends on unsupported assumptions not stated in benchmark scope

## Gate 9: Final Decision Rules

Mark `Valid` with `High` or `Medium` only when all are true:
- bug exists in current in-scope code
- exploit or failure path is realistic
- impact is materially H/M
- not excluded by configured scope, known issues, unsupported behavior, or V12/prior-finding overlap
- evidence is grounded in code, docs, or concrete exploit reasoning

Mark `Valid` with `Low / QA` when:
- bug exists, but impact or likelihood does not reach H/M
- bug is real but operationally minor
- issue is best handled as hardening, documentation, gas, events, or minor edge-case handling

Mark `Invalid` when:
- bug does not exist
- claim is impossible, speculative, unsupported, out of scope, duplicate of V12/prior finding, or depends on trusted-role/user/deployment error

Mark `Needs Review` only when:
- the code and docs leave a real unresolved ambiguity
- the ambiguity could plausibly flip H/M validity
- you can identify the exact missing fact needed

## Output Guidance

In each finding decision, make the reason specific:
- cite the relevant function / file
- state the broken invariant if valid
- state the exact blocker if invalid or Low / QA
- distinguish bug existence from severity
- if V12 / prior-finding overlap is involved, state why the candidate is materially distinct or why it is not

Avoid vague phrases like:
- "may be possible"
- "could potentially"
- "needs more review"
- "seems mitigated"
- "similar to another issue"

Prefer concise, code-grounded conclusions.
