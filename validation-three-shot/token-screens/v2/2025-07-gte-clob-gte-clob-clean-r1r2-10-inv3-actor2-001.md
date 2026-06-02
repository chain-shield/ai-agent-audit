# 2025-07-gte-clob Three-Shot Stage 2 Unsupported-Token Screen

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2025-07-gte-clob/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`
- `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| M-1 | Broken same-price order links let a maker cancel the tail order and brick the best price level | Keep | High | supported-behavior | Order-book pointer corruption uses standard order flow, not token semantics. |
| M-2 | Tail cancellation corrupts same-price order queue and bricks matching at that price | Keep | High | supported-behavior | Same-price queue corruption uses standard order flow, not token semantics. |
| M-3 | Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks | Keep | High | supported-behavior | Expired-order cleanup gas path uses standard orders, not token semantics. |
| M-4 | Expired top-of-book orders require unbounded cleanup before any matching can proceed | Keep | High | supported-behavior | Expired-order cleanup gas path uses standard orders, not token semantics. |
| M-5 | Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts | Keep | High | supported-behavior | Bitwise zero-cost predicate is protocol integer math, not token behavior. |
| L-6 | Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders | Keep | High | supported-behavior | Post-only expiry gate uses order timestamps, not token semantics. |
| M-8 | Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable | Keep | High | supported-behavior | Price-level link corruption uses standard order flow, not token semantics. |
| M-9 | Appended order cancellation corrupts a price level and makes older resting orders unmatchable | Keep | High | supported-behavior | Appended-order cancellation uses standard order flow, not token semantics. |
| M-10 | Tail cancellation corrupts same-price FIFO queue and blocks matching at that price | Keep | High | supported-behavior | FIFO queue corruption uses standard order flow, not token semantics. |
| M-11 | Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price | Keep | High | supported-behavior | FIFO queue corruption uses standard order flow, not token semantics. |
| M-12 | Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price | Keep | High | supported-behavior | Limit pointer corruption uses standard order flow, not token semantics. |
| M-13 | Expired order cleanup can be inflated to brick matching at the top of book | Keep | High | supported-behavior | Expired-order cleanup gas path uses standard order flow, not token semantics. |
| M-14 | Expired order flood causes unbounded CLOB matching gas and can brick router fill routes | Keep | High | supported-behavior | Router fill gas path uses standard CLOB orders, not token semantics. |
| M-15 | Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args | Keep | High | supported-behavior | Amend timestamp handling uses order state, not token semantics. |
| M-16 | Fee-on-transfer deposits overcredit AccountManager balances and can make withdrawals insolvent | Exclude | High | unsupported-token | Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior. |
| M-17 | Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price | Keep | High | supported-behavior | Price-level link corruption uses standard order flow, not token semantics. |
| H-18 | Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching | Keep | High | supported-behavior | Shared-price cancellation uses standard order flow, not token semantics. |
| M-19 | Unbounded expired-order cleanup lets makers gas-DoS fills at the best price | Keep | High | supported-behavior | Expired-order cleanup gas path uses standard orders, not token semantics. |
| M-20 | Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity | Keep | High | supported-behavior | Router CLOB fill path uses standard orders, not token semantics. |
| M-21 | Same-price order cancellation corrupts BookLib limits and blocks matching at the best price | Keep | High | supported-behavior | BookLib limit corruption uses standard order flow, not token semantics. |
| M-22 | Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price | Keep | High | supported-behavior | Price-level queue corruption uses standard order flow, not token semantics. |
| M-23 | Zero-quote dust orders can permanently block one side of a CLOB market | Keep | High | supported-behavior | Zero-quote dust is lot and rounding behavior called out for review, not unsupported token behavior. |
| M-24 | Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price | Keep | High | supported-behavior | Price-level pointer corruption uses standard order flow, not token semantics. |
| M-25 | Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth | Keep | High | supported-behavior | Price-level cap bypass is order-book state behavior, not token semantics. |
| M-27 | Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide | Keep | High | supported-behavior | Full-book pruning is order-book state behavior, not token semantics. |
| L-28 | Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies | Keep | High | supported-behavior | Amend expiry handling uses order state, not token semantics. |
| H-29 | Same-price order append corrupts CLOB queue and can brick matching at the best price | Keep | High | supported-behavior | Queue corruption uses standard order flow, not token semantics. |
| M-30 | Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains | Keep | High | supported-behavior | Router guard behavior is chain guard execution, not token semantics. |
| M-31 | Expired best-price order queues create unbounded matching work and can gas-brick market progress | Keep | High | supported-behavior | Expired-order queue gas path uses standard orders, not token semantics. |
| M-33 | Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price | Keep | High | supported-behavior | Same-price limit corruption uses standard order flow, not token semantics. |
| M-34 | Nominal router deposits over-credit accounts for fee-on-transfer market tokens | Exclude | High | unsupported-token | Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior. |
| M-35 | getNextOrders skips lower bid levels by always walking to the next bigger price | Keep | High | supported-behavior | View pagination traversal is order-book logic, not token semantics. |
| L-36 | Per-fill fee flooring lets traders split orders to avoid maker and taker fees | Keep | High | supported-behavior | Fee-flooring is protocol fee math, not unsupported token behavior. |
| M-37 | Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas | Keep | High | supported-behavior | Router CLOB fill path uses standard orders, not token semantics. |
| M-38 | Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price | Keep | High | supported-behavior | Price-level pointer corruption uses standard order flow, not token semantics. |
| M-39 | Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price | Keep | High | supported-behavior | Price-level link corruption uses standard order flow, not token semantics. |
| M-40 | Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price | Keep | High | supported-behavior | Limit pointer corruption uses standard order flow, not token semantics. |
| M-41 | Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price | Keep | High | supported-behavior | Queue corruption uses standard order flow, not token semantics. |
| M-42 | Nominal router deposits overcredit AccountManager for fee-on-transfer market tokens | Exclude | High | unsupported-token | Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior. |
| M-44 | Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching | Keep | High | supported-behavior | Price-level link corruption uses standard order flow, not token semantics. |
| M-45 | Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode | Keep | High | supported-behavior | Router guard behavior is chain guard execution, not token semantics. |
| H-46 | Canceling an appended order corrupts the FIFO list and bricks matching at that price | Keep | High | supported-behavior | FIFO list corruption uses standard order flow, not token semantics. |
| M-47 | GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized | Keep | High | supported-behavior | Router guard behavior is chain guard execution, not token semantics. |
| M-48 | Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask | Keep | High | supported-behavior | Price-level corruption uses standard order flow, not token semantics. |
| M-49 | Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills | Keep | High | supported-behavior | Expired-order settlement gas path uses standard orders, not token semantics. |
| M-50 | DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder | Keep | High | supported-behavior | Same-price link corruption uses standard order flow, not token semantics. |
| M-51 | Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price | Keep | High | supported-behavior | Price queue corruption uses standard order flow, not token semantics. |
| M-52 | GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains | Keep | High | supported-behavior | Router guard behavior is chain guard execution, not token semantics. |
| M-53 | Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side | Keep | High | supported-behavior | Router CLOB fill path uses standard orders, not token semantics. |
| M-54 | Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders | Keep | High | supported-behavior | Bitwise zero-cost predicate is protocol integer math, not token behavior. |
| M-55 | BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching | Keep | High | supported-behavior | Appended-order storage bug uses standard order flow, not token semantics. |
| M-56 | Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts | Keep | High | supported-behavior | Bitwise predicate is integer math; zero-decimal PoC is not necessary to the root cause. |
| L-57 | Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks | Keep | High | supported-behavior | Bitwise zero-cost predicate is protocol integer math, not token behavior. |
| M-58 | Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching | Keep | High | supported-behavior | Linked-list corruption uses standard order flow, not token semantics. |
| M-59 | Unlimited orders per price level can make CLOB matching and settlement exceed block gas | Keep | High | supported-behavior | Same-price order count gas path uses standard orders, not token semantics. |
| M-60 | Unbounded expired order clearing can brick CLOB matching at the top of book | Keep | High | supported-behavior | Expired-order clearing uses standard orders, not token semantics. |
| M-61 | Unbounded expired top-of-book cleanup can make matching exceed gas and block fills | Keep | High | supported-behavior | Expired-order cleanup uses standard orders, not token semantics. |
| M-62 | Bitwise zero-cost check rejects valid nonzero CLOB limit fills | Keep | High | supported-behavior | Bitwise zero-cost predicate is protocol integer math, not token behavior. |
| M-63 | Unbounded maker credit settlement lets large fills exceed gas and DoS order matching | Keep | High | supported-behavior | Maker-credit loop uses standard settlement data, not token semantics. |
| M-64 | Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes | Keep | High | supported-behavior | Expired-order router path uses standard orders, not token semantics. |
| M-65 | Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup | Keep | High | supported-behavior | Expired-order reverting cleanup uses standard orders, not token semantics. |
| M-66 | Expired top-of-book orders can gas-DoS CLOB fills and routed trades | Keep | High | supported-behavior | Expired-order routed trade path uses standard orders, not token semantics. |
| M-67 | Unlimited same-price dust orders can make CLOB fills exceed practical gas limits | Keep | High | supported-behavior | Same-price dust order gas path uses standard orders, not token semantics. |
| H-68 | Reentrant token can withdraw pre-existing AccountManager liquidity during deposit before funding the new credit | Exclude | High | unsupported-token-semantics | Claim requires callback or malicious ERC20 transfer semantics; docs do not explicitly support hook-enabled tokens. |
| M-69 | Fee-on-transfer deposits credit more internal balance than AccountManager actually receives | Exclude | High | unsupported-token | Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior. |
| L-70 | creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting | Keep | High | supported-behavior | Event emission mismatch is accounting and indexing behavior, not unsupported token semantics. |
| M-71 | Failed fee transfer permanently clears unclaimed fees in AccountManager.collectFees | Exclude | High | unsupported-erc20-return | Claim requires token that reports transfer success without moving funds; docs do not explicitly support false-success ERC20 semantics. |
| M-73 | Live fee-tier reads let routine tier changes alter already-resting order settlement | Keep | High | supported-behavior | Fee-tier live-read behavior is governance and accounting state, not token semantics. |
