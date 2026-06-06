# 2025-07-gte-clob Three-Shot Stage 1 Scope Screen

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
| M-1 | Broken same-price order links let a maker cancel the tail order and brick the best price level | Keep | High | in-scope | Scoped CLOB and BookLib path with permissionless maker actions and no known-issue overlap. |
| M-2 | Tail cancellation corrupts same-price order queue and bricks matching at that price | Keep | High | in-scope | Scoped BookLib and CLOB cancel path with permissionless actions and no V12 context match. |
| M-3 | Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks | Keep | High | in-scope | Scoped CLOB matching cleanup issue and not one of the README public known issues. |
| M-4 | Expired top-of-book orders require unbounded cleanup before any matching can proceed | Keep | High | in-scope | Scoped CLOB matching liveness issue and not excluded by benchmark docs. |
| M-5 | Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts | Keep | High | in-scope | Scoped CLOB limit-order math issue and not the known fill-rounding disclosure. |
| L-6 | Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders | Keep | High | in-scope | Scoped CLOB post-only path and not a benchmark-declared known issue. |
| M-7 | Live lot-size updates can make existing top-of-book orders unmatchable | Exclude | High | deployment-or-setup | Requires a privileged lot-size update path that the scoped CLOBManager does not expose. |
| M-8 | Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable | Keep | High | in-scope | Scoped CLOB and BookLib order-link issue with permissionless post and cancel actions. |
| M-9 | Appended order cancellation corrupts a price level and makes older resting orders unmatchable | Keep | High | in-scope | Scoped BookLib order removal issue and no benchmark known-issue blocker. |
| M-10 | Tail cancellation corrupts same-price FIFO queue and blocks matching at that price | Keep | High | in-scope | Scoped CLOB and BookLib path with permissionless maker actions. |
| M-11 | Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price | Keep | High | in-scope | Scoped same-price queue corruption issue and not V12 or public-known overlap. |
| M-12 | Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price | Keep | High | in-scope | Scoped CLOB and BookLib path and no decisive scope exclusion. |
| M-13 | Expired order cleanup can be inflated to brick matching at the top of book | Keep | High | in-scope | Scoped CLOB and AccountManager cleanup and settlement path. |
| M-14 | Expired order flood causes unbounded CLOB matching gas and can brick router fill routes | Keep | High | in-scope | Scoped CLOB and GTERouter route impact and not a listed known issue. |
| M-15 | Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args | Keep | High | in-scope | Scoped amend timestamp behavior and distinct from the public amend transient-limit issue. |
| M-16 | Fee-on-transfer deposits overcredit AccountManager balances and can make withdrawals insolvent | Keep | Medium | in-scope | Scoped AccountManager deposit path and unsupported-token screening is not a round 1 exclusion here. |
| M-17 | Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price | Keep | High | in-scope | Scoped BookLib append and removal path with permissionless order actions. |
| H-18 | Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching | Keep | High | in-scope | Scoped CLOB and BookLib same-price cancellation issue. |
| M-19 | Unbounded expired-order cleanup lets makers gas-DoS fills at the best price | Keep | High | in-scope | Scoped CLOB matching cleanup issue and not benchmark-declared known. |
| M-20 | Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity | Keep | High | in-scope | Scoped router-to-CLOB fill route and CLOB cleanup path. |
| M-21 | Same-price order cancellation corrupts BookLib limits and blocks matching at the best price | Keep | High | in-scope | Scoped BookLib limit pointer issue and no V12 duplicate context. |
| M-22 | Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price | Keep | High | in-scope | Scoped CLOB and BookLib cancellation path with permissionless maker actions. |
| M-23 | Zero-quote dust orders can permanently block one side of a CLOB market | Keep | Medium | in-scope | Severe zero-quote blocker is materially beyond the README dust-rounding known behavior. |
| M-24 | Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price | Keep | High | in-scope | Scoped BookLib append and remove path with no known-issue exclusion. |
| M-25 | Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth | Keep | High | in-scope | Scoped CLOB anti-flooding path and sponsor focus includes order-flooding controls. |
| M-26 | Lot-size increases can strand existing orders below the new lot and block matching | Exclude | High | deployment-or-setup | Requires a privileged lot-size update path that the scoped CLOBManager does not expose. |
| M-27 | Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide | Keep | High | in-scope | Scoped CLOB price-level pruning issue and not excluded by public known issues. |
| L-28 | Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies | Keep | High | in-scope | Scoped amend timestamp path and distinct from the public amend transient-limit issue. |
| H-29 | Same-price order append corrupts CLOB queue and can brick matching at the best price | Keep | High | in-scope | Scoped BookLib order-link invariant issue with permissionless path. |
| M-30 | Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains | Keep | Medium | in-scope | GTERouter is scoped and the benchmark docs do not declare non-mainnet deployment out of scope. |
| M-31 | Expired best-price order queues create unbounded matching work and can gas-brick market progress | Keep | High | in-scope | Scoped CLOB matching loop issue and no known-issue blocker. |
| M-32 | Live lot-size changes can make existing top-of-book orders unmatchable | Exclude | High | deployment-or-setup | Requires a privileged lot-size update path that the scoped CLOBManager does not expose. |
| M-33 | Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price | Keep | High | in-scope | Scoped BookLib pointer issue with permissionless post and cancel actions. |
| M-34 | Nominal router deposits over-credit accounts for fee-on-transfer market tokens | Keep | Medium | in-scope | Scoped GTERouter and AccountManager deposit path and no benchmark token exclusion applies in round 1. |
| M-35 | getNextOrders skips lower bid levels by always walking to the next bigger price | Keep | High | in-scope | Scoped BookLib view path and no benchmark-declared exclusion. |
| L-36 | Per-fill fee flooring lets traders split orders to avoid maker and taker fees | Keep | High | in-scope | Scoped FeeData fee calculation issue and not public-known fill rounding. |
| M-37 | Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas | Keep | High | in-scope | Scoped GTERouter and CLOB fill route with no known-issue overlap. |
| M-38 | Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price | Keep | High | in-scope | Scoped CLOB cancel and BookLib pointer path. |
| M-39 | Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price | Keep | High | in-scope | Scoped CLOB and BookLib order-link issue. |
| M-40 | Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price | Keep | High | in-scope | Scoped same-price cancellation path with permissionless maker actions. |
| M-41 | Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price | Keep | High | in-scope | Scoped BookLib append and CLOB fill path with no public-known blocker. |
| M-42 | Nominal router deposits overcredit AccountManager for fee-on-transfer market tokens | Keep | Medium | in-scope | Scoped router-mediated deposit path and unsupported-token filtering is not applied at this stage. |
| M-43 | Authorized tick-size update lets zero-quote asks permanently block buy-side matching | Keep | Medium | in-scope | Scoped tick-size setter can be followed by permissionless blocker and docs do not declare it config-only OOS. |
| M-44 | Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching | Keep | High | in-scope | Scoped BookLib and CLOB matching path with permissionless maker actions. |
| M-45 | Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode | Keep | Medium | in-scope | GTERouter is scoped and no benchmark doc excludes non-mainnet guarded flow issues. |
| H-46 | Canceling an appended order corrupts the FIFO list and bricks matching at that price | Keep | High | in-scope | Scoped BookLib FIFO issue and no V12 duplicate context. |
| M-47 | GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized | Keep | Medium | in-scope | Scoped GTERouter behavior and not benchmark-declared deployment-only OOS. |
| M-48 | Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask | Keep | High | in-scope | Scoped same-price queue corruption issue. |
| M-49 | Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills | Keep | High | in-scope | Scoped CLOB and AccountManager settlement path. |
| M-50 | DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder | Keep | High | in-scope | Scoped BookLib order-link issue with permissionless reachability alleged. |
| M-51 | Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price | Keep | High | in-scope | Scoped CLOB cancel and BookLib queue path. |
| M-52 | GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains | Keep | Medium | in-scope | Scoped GTERouter issue and no configured context excludes non-mainnet deployments. |
| M-53 | Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side | Keep | High | in-scope | Scoped GTERouter executeRoute and CLOB cleanup path. |
| M-54 | Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders | Keep | High | in-scope | Scoped CLOB limit-order zero-cost check and not public-known fill dust. |
| M-55 | BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching | Keep | High | in-scope | Scoped BookLib append path with no benchmark known-issue blocker. |
| M-56 | Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts | Keep | High | in-scope | Scoped CLOB limit-order math issue and no V12 overlap. |
| L-57 | Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks | Keep | High | in-scope | Scoped CLOB limit-order predicate issue. |
| M-58 | Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching | Keep | High | in-scope | Scoped BookLib linked-list issue with permissionless order actions. |
| M-59 | Unlimited orders per price level can make CLOB matching and settlement exceed block gas | Keep | High | in-scope | Scoped CLOB matching and AccountManager settlement issue. |
| M-60 | Unbounded expired order clearing can brick CLOB matching at the top of book | Keep | High | in-scope | Scoped CLOB expired-order cleanup issue. |
| M-61 | Unbounded expired top-of-book cleanup can make matching exceed gas and block fills | Keep | High | in-scope | Scoped CLOB matching loop issue and not a public known issue. |
| M-62 | Bitwise zero-cost check rejects valid nonzero CLOB limit fills | Keep | High | in-scope | Scoped CLOB limit-order zero-cost predicate issue. |
| M-63 | Unbounded maker credit settlement lets large fills exceed gas and DoS order matching | Keep | High | in-scope | Scoped AccountManager settlement loop and CLOB maker-credit path. |
| M-64 | Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes | Keep | High | in-scope | Scoped CLOB and router fill route with no known-issue exclusion. |
| M-65 | Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup | Keep | High | in-scope | Scoped CLOB cleanup rollback issue and not benchmark-declared known. |
| M-66 | Expired top-of-book orders can gas-DoS CLOB fills and routed trades | Keep | High | in-scope | Scoped CLOB matching and GTERouter route behavior. |
| M-67 | Unlimited same-price dust orders can make CLOB fills exceed practical gas limits | Keep | High | in-scope | Scoped CLOB same-price order count and settlement work issue. |
| H-68 | Reentrant token can withdraw pre-existing AccountManager liquidity during deposit before funding the new credit | Keep | Medium | in-scope | Scoped AccountManager deposit and withdraw path and unsupported-token screening is deferred. |
| M-69 | Fee-on-transfer deposits credit more internal balance than AccountManager actually receives | Keep | Medium | in-scope | Scoped AccountManager deposit path and no configured docs classify fee-on-transfer as OOS. |
| L-70 | creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting | Keep | Medium | in-scope | Scoped AccountManager behavior and not a documentation-only report under benchmark docs. |
| M-71 | Failed fee transfer permanently clears unclaimed fees in AccountManager.collectFees | Keep | Low | in-scope | Scoped collectFees path and unsupported-token filtering is not a round 1 benchmark exclusion. |
| M-72 | Manager setting updates can admit zero-quote orders that brick one side of a CLOB market | Keep | Medium | in-scope | Scoped tick and min-order setters can precede permissionless blocker and are not declared config-only OOS. |
| M-73 | Live fee-tier reads let routine tier changes alter already-resting order settlement | Exclude | High | trusted-role-oos | Depends on trusted fee-tier setter changing delegated fee parameters with no permissionless obstruction. |
