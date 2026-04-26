# Validation Prompt Changelog

## Reset on 2026-04-25

- Discarded the `v2` / `v3` prompt lineage and the derived `v4` draft after recall regressed across iterations.
- Reset prompt evolution back to `v1` as the active baseline.
- Future prompt creation is now explicitly recall-first: maximize approved-root H/M recall while keeping H/M precision near or above the 50% guardrail.

## v1 -> v2

- Added an explicit evidence hierarchy and durable `Core Validation Principles` section so workers separate bug existence, root-cause independence, and severity instead of collapsing them.
- Added a mandatory scope-first stage that tells workers to read `README.md`, `olas-scope.md`, `olas-docs.md`, `v12-findings.md`, and `v12-checklist.md` before any deeper validation work.
- Strengthened the prompt so client-declared scope, known issues, and V12 duplicates are filtered before bug-existence or severity analysis.
- Reframed unsupported ERC20 / non-standard token behavior as out of scope unless the benchmark docs explicitly declare support for that exact token class or semantic.
- Added a root-cause independence gate that rejects only non-independent downstream symptoms or alternate patch sites while preserving same-root articulations that still prove the live broken invariant.
- Reframed safeguards, likelihood, governance, and by-design handling so real runtime H/M issues are not downgraded merely because exploitation needs realistic timing, expected admin / keeper workflows, or nominal-but-non-binding protections.
- Added stronger positive H/M guidance for fail-open pricing or oracle guards, missing execution-time output checks, stale authorization, missing value binding, and core accounting / liveness / reconciliation failures.
- Kept precision guardrails against unsupported non-standard asset behavior, speculative future failures, deployment / setup mistakes, documentation-only issues, and economically irrational nuisance griefing.
- Removed report-local deduping from raw validation: V12 / client-known duplicates are still filtered at scope time, but same-family report findings are now evaluated independently and left for a later cleanup pass.
