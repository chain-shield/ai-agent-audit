# 2025-07-gte-clob Three-Shot Round 4a V12 Overlap Sweep

Status: Complete
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/2025-07-gte-clob-gte-clob-fnfix-001.md`
Output filtered submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/2025-07-gte-clob-gte-clob-fnfix-001.md`
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
| M-23 | Zero-quote dust orders can permanently block one side of a CLOB market | Keep | High | README known issue: CLOB Fill Rounding | This turns rounding dust into a persistent top-of-book liveness failure rather than a dust-only fill-rounding symptom. | keep-materially-distinct-impact | Distinct market-blocking harm from a live dust remainder. |
| M-25 | Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth | Keep | High | - | This bypasses the configured price-level cap itself, allowing persistent tree growth instead of merely slowing clears. | keep-materially-distinct-root | Separate anti-flooding control break with its own fix path. |
| M-49 | Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills | Keep | High | - | This uses expired head orders plus refund settlement to force attacker-sized cleanup before honest liquidity can trade. | keep-materially-distinct-root | Distinct expired-order cleanup path, not just general many-order pressure. |
| M-55 | BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching | Keep | High | - | This corrupts FIFO pointers at one limit so later normal removals break matching even without flooding or dust. | keep-materially-distinct-root | State-corruption bug with a separate invariant and mitigation. |
| M-59 | Unlimited orders per price level can make CLOB matching and settlement exceed block gas | Keep | Med | - | This uses valid non-expired liquidity, so the gas failure remains even if expired-order cleanup is fixed. | keep-materially-distinct-action | Separate live-order flood path with a different exploit precondition. |
| M-62 | Bitwise zero-cost check rejects valid nonzero CLOB limit fills | Keep | High | README known issue: CLOB Fill Rounding | This is a standalone bitwise predicate bug that rejects positive-positive fills independent of dust rounding. | keep-materially-distinct-root | Different logic error and mitigation from the rounding family. |
| M-63 | Unbounded maker credit settlement lets large fills exceed gas and DoS order matching | Keep | Med | - | This isolates the maker-credit settlement loop as the gas bottleneck even without relying on expired-order cleanup. | keep-materially-distinct-root | Separate settlement-side gas surface with its own exploit step. |
| M-65 | Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup | Keep | High | - | This is a deterministic rollback that preserves expired head orders even without block-gas exhaustion. | keep-materially-distinct-impact | Permanent liveness failure via reverting cleanup, not just gas-heavy progress loss. |
| M-66 | Expired top-of-book orders can gas-DoS CLOB fills and routed trades | Keep | High | - | This targets out-of-gas rollback on active expired-head cleanup and shows the same harm propagates through routed fills. | keep-materially-distinct-value-flow | Distinct direct-plus-routed fill path for the expired-head gas failure. |
