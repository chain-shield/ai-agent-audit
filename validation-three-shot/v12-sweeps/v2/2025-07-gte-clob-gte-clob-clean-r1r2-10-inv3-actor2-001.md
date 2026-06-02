# 2025-07-gte-clob Three-Shot Round 4a V12 Overlap Sweep

Status: In progress
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/2025-07-gte-clob-gte-clob-clean-r1r2-10-inv3-actor2-001.md`
Output filtered submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/2025-07-gte-clob-gte-clob-clean-r1r2-10-inv3-actor2-001.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`
- `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Closest V12 Finding | Material Difference | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- |
| M-1 / `U2Hhi3vbiZqDpjxJh6hmv` | Broken same-price order links let a maker cancel the tail order and brick the best price level | Keep | High | - | Same-price FIFO pointer corruption breaks a live limit and can brick best-price matching until another actor repairs that level. | keep-materially-distinct-root | No benchmark V12 exists, and this is unrelated to the README rounding and amend known issues. |
| M-4 / `qeqdYm-KXE7yOpIWJSlb9` | Expired top-of-book orders require unbounded cleanup before any matching can proceed | Keep | High | - | Matching liveness depends on clearing arbitrary expired head-order queues that can accumulate across transactions. | keep-materially-distinct-impact | No benchmark V12 exists, and the known issues do not cover expired-order cleanup liveness. |
| M-5 / `Fj0qTYNYHzzhvXQcrR0IJ` | Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts | Keep | High | - | Valid crossing limit trades fail because the zero-cost predicate is a bitwise test rather than an actual zero-value check. | keep-materially-distinct-root | No benchmark V12 exists, and this does not overlap the disclosed dust or amend issues. |
| M-23 / `9DMzwZBvN1i2QQTvRs_0Q` | Zero-quote dust orders can permanently block one side of a CLOB market | Keep | High | CLOB fill rounding | The known issue covers current-fill dust handling, while this leaves a resting best-price blocker that can later revert unrelated trades. | keep-materially-distinct-value-flow | This shows a separate future-trade liveness failure, not just the disclosed fill-dust symptom. |
| M-25 / `RdhlzdOHuMq96aCAhrX5A` | Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth | Keep | High | - | The bug breaks the market-wide price-level cap itself, so valid inserts can grow the book past the configured side limit. | keep-materially-distinct-root | Different anti-flooding boundary from the README known issues and from same-price queue growth. |
| M-30 / `EANPmE7_tnXWUY7__k2Xm` | Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains | Exclude | Med | - | none | exclude-compatibility-only | The supplied benchmark context does not establish non-mainnet router support as an intended compatibility target. |
| M-59 / `8w065WL0FE9FUGvl4vSXj` | Unlimited orders per price level can make CLOB matching and settlement exceed block gas | Keep | Med | - | One-price FIFO depth can exhaust matching and maker-credit settlement gas without adding new price levels. | keep-materially-distinct-root | Distinct from price-level-cap bypass because it abuses uncapped per-level order count instead of tree size. |
| M-65 / `7NpX1q1QAy0pvHbmh_VvF` | Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup | Exclude | High | - | none | exclude-weak-materiality | Same expired-cleanup root and same liveness harm as M-4; cleanup rollback is only a variant failure path, not a separate exploit or fix. |
