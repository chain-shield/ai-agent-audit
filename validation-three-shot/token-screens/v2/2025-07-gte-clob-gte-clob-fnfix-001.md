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
| M-1 | Broken same-price order links let a maker cancel the tail order and brick the best price level | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-2 | Tail cancellation corrupts same-price order queue and bricks matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-3 | Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-4 | Expired top-of-book orders require unbounded cleanup before any matching can proceed | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-5 | Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts | Keep | High | supported-behavior | Integer zero-cost check over normal trade amounts, no token blocker. |
| L-6 | Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders | Keep | High | supported-behavior | Post-only and expiry ordering is CLOB logic, no token blocker. |
| M-8 | Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-9 | Appended order cancellation corrupts a price level and makes older resting orders unmatchable | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-10 | Tail cancellation corrupts same-price FIFO queue and blocks matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-11 | Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-12 | Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-13 | Expired order cleanup can be inflated to brick matching at the top of book | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-14 | Expired order flood causes unbounded CLOB matching gas and can brick router fill routes | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-15 | Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args | Keep | High | supported-behavior | Amendment timestamp handling is CLOB state logic, no token blocker. |
| M-16 | Fee-on-transfer deposits overcredit AccountManager balances and can make withdrawals insolvent | Exclude | High | unsupported-token-semantics | Depends on fee-on-transfer or deflationary asset behavior not supported by docs. |
| M-17 | Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| H-18 | Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-19 | Unbounded expired-order cleanup lets makers gas-DoS fills at the best price | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-20 | Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-21 | Same-price order cancellation corrupts BookLib limits and blocks matching at the best price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-22 | Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-23 | Zero-quote dust orders can permanently block one side of a CLOB market | Keep | High | supported-behavior | Price and lot rounding is CLOB math, no unsupported-token dependency. |
| M-24 | Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-25 | Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth | Keep | High | supported-behavior | Price-level cap behavior is CLOB logic, no token blocker. |
| M-27 | Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide | Keep | High | supported-behavior | Price-level pruning is CLOB logic, no token blocker. |
| L-28 | Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies | Keep | High | supported-behavior | Amendment timestamp handling is CLOB state logic, no token blocker. |
| H-29 | Same-price order append corrupts CLOB queue and can brick matching at the best price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-30 | Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains | Keep | High | supported-behavior | Chain guard behavior is not token-related. |
| M-31 | Expired best-price order queues create unbounded matching work and can gas-brick market progress | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-33 | Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-34 | Nominal router deposits over-credit accounts for fee-on-transfer market tokens | Exclude | High | unsupported-token-semantics | Depends on fee-on-transfer market-token behavior not supported by docs. |
| M-35 | getNextOrders skips lower bid levels by always walking to the next bigger price | Keep | High | supported-behavior | Pagination traversal is order-book logic, no token blocker. |
| L-36 | Per-fill fee flooring lets traders split orders to avoid maker and taker fees | Keep | High | supported-behavior | Fee-flooring issue is accounting math, no unsupported-token dependency. |
| M-37 | Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-38 | Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-39 | Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-40 | Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-41 | Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-42 | Nominal router deposits overcredit AccountManager for fee-on-transfer market tokens | Exclude | High | unsupported-token-semantics | Depends on fee-on-transfer market-token behavior not supported by docs. |
| M-43 | Authorized tick-size update lets zero-quote asks permanently block buy-side matching | Keep | High | supported-behavior | Tick and minimum-order settings are protocol parameters, not token semantics. |
| M-44 | Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-45 | Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode | Keep | High | supported-behavior | Chain guard behavior is not token-related. |
| H-46 | Canceling an appended order corrupts the FIFO list and bricks matching at that price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-47 | GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized | Keep | High | supported-behavior | Chain guard behavior is not token-related. |
| M-48 | Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-49 | Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-50 | DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-51 | Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-52 | GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains | Keep | High | supported-behavior | Chain guard behavior is not token-related. |
| M-53 | Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-54 | Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders | Keep | High | supported-behavior | Integer zero-cost check over normal trade amounts, no token blocker. |
| M-55 | BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-56 | Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts | Keep | High | supported-behavior | Bitwise predicate can fail for positive integer amounts, no token blocker. |
| L-57 | Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks | Keep | High | supported-behavior | Integer zero-cost check over normal trade amounts, no token blocker. |
| M-58 | Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching | Keep | High | supported-behavior | Order-book link corruption uses standard base and quote assets, no token blocker. |
| M-59 | Unlimited orders per price level can make CLOB matching and settlement exceed block gas | Keep | High | supported-behavior | Same-price order flooding uses standard orders and assets, no token blocker. |
| M-60 | Unbounded expired order clearing can brick CLOB matching at the top of book | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-61 | Unbounded expired top-of-book cleanup can make matching exceed gas and block fills | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-62 | Bitwise zero-cost check rejects valid nonzero CLOB limit fills | Keep | High | supported-behavior | Integer zero-cost check over normal trade amounts, no token blocker. |
| M-63 | Unbounded maker credit settlement lets large fills exceed gas and DoS order matching | Keep | High | supported-behavior | Settlement batching issue uses normal maker credits, no token blocker. |
| M-64 | Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-65 | Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-66 | Expired top-of-book orders can gas-DoS CLOB fills and routed trades | Keep | High | supported-behavior | Expiry cleanup gas path uses standard orders and assets, no token blocker. |
| M-67 | Unlimited same-price dust orders can make CLOB fills exceed practical gas limits | Keep | High | supported-behavior | Same-price order flooding uses standard orders and assets, no token blocker. |
| H-68 | Reentrant token can withdraw pre-existing AccountManager liquidity during deposit before funding the new credit | Exclude | High | unsupported-token-semantics | Depends on callback or malicious token transfer behavior not supported by docs. |
| M-69 | Fee-on-transfer deposits credit more internal balance than AccountManager actually receives | Exclude | High | unsupported-token-semantics | Depends on fee-on-transfer or rebasing asset behavior not supported by docs. |
| L-70 | creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting | Keep | High | supported-behavior | Event and accounting path uses normal settlement, no token blocker. |
| M-71 | Failed fee transfer permanently clears unclaimed fees in AccountManager.collectFees | Exclude | High | unsupported-erc20-return | Depends on false-success or no-op ERC20 transfer behavior not supported by docs. |
| M-72 | Manager setting updates can admit zero-quote orders that brick one side of a CLOB market | Keep | High | supported-behavior | Tick and minimum-order settings are protocol parameters, not token semantics. |
