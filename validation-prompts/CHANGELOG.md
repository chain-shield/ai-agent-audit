# Validation Prompt Changelog

## v1 -> v2

- Added an explicit evidence hierarchy and a durable `Core Validation Principles` section so workers separate bug existence, root-cause quality, and severity instead of collapsing them into one decision.
- Removed the permissive "when in doubt -> valid" posture and replaced it with a balanced tie-breaker: uncertain bug existence becomes `Invalid`, while real but not independently proven H/M issues become `Low / QA` or `Needs Review`.
- Added a dedicated root-cause independence gate to suppress derivative symptoms, alternate patch sites, and weaker restatements of deeper bugs that should not count as separate H/M findings.
- Tightened supported-asset and supported-integration handling so non-standard token behavior and unsupported external semantics do not enter the H/M bucket by default.
- Tightened likelihood and governance guidance so economically irrational griefing, deployment/setup mistakes, and trusted-component assumptions are filtered more reliably.
- Clarified that nominal safeguards do not count if they are fail-open, stale, incomplete, on the wrong path, or too weak to materially bound the exposure.
- Clarified that realistic timing, sequencing, front-running, and market conditions do not automatically make a valid H/M issue low-likelihood.
- Strengthened severity calibration so real but bounded, derivative, or operationally minor issues stay `Low / QA`, while real fail-open pricing, liveness, authorization, and accounting failures are not downgraded merely because they are conditional.
