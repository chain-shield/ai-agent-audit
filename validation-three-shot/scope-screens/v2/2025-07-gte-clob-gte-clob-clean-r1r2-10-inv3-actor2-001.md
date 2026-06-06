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
| M-1 / U2Hhi3vbiZqDpjxJh6hmv | Broken same-price order links let a maker cancel the tail order and brick the best price level | Keep | High | in-scope | BookLib and CLOB are scoped and permissionless order actions are not listed as known issues. |
| M-2 / ntTDyuzQFbVoG65j59b0Y | Tail cancellation corrupts same-price order queue and bricks matching at that price | Keep | High | in-scope | BookLib and CLOB cancel paths are scoped and not covered by the public known issues. |
| M-3 / OWYDJ_ICDupdBAfo1R4VA | Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks | Keep | High | in-scope | CLOB matching is scoped and expired-order cleanup DoS is not declared known or out of scope. |
| M-4 / qeqdYm-KXE7yOpIWJSlb9 | Expired top-of-book orders require unbounded cleanup before any matching can proceed | Keep | High | in-scope | CLOB bid and ask matching are scoped and this is distinct from the disclosed fill-rounding issue. |
| M-5 / Fj0qTYNYHzzhvXQcrR0IJ | Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts | Keep | High | in-scope | Scoped CLOB limit matching and not the disclosed CLOB fill dust behavior. |
| L-6 / cOtLAQEI7uTkuEyPPoAZh | Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders | Keep | High | in-scope | Scoped CLOB post-only gate and no known-issue blocker found. |
| M-7 / _aisuPcW4RFZC8k1ecQz0 | Live lot-size updates can make existing top-of-book orders unmatchable | Exclude | Medium | configuration-nonissue | Depends only on trusted manager-only lot-size configuration changing live order assumptions. |
| M-8 / SNePL7SCfZJ5uSjAhF0yD | Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable | Keep | High | in-scope | BookLib and CLOB order cancellation are scoped and not listed as public known issues. |
| M-9 / 3QpPKCdYzCkBLpAcsNNz6 | Appended order cancellation corrupts a price level and makes older resting orders unmatchable | Keep | High | in-scope | Scoped BookLib order queue behavior with permissionless maker actions. |
| M-10 / ulpg6Or9uGZvpzvldYeKZ | Tail cancellation corrupts same-price FIFO queue and blocks matching at that price | Keep | High | in-scope | Scoped BookLib and CLOB cancel path with no scope or known-issue exclusion. |
| M-11 / Xst74MzxPAZmmiNOcO__7 | Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price | Keep | High | in-scope | Scoped same-price FIFO behavior and permissionless order flow. |
| M-12 / cXOP9qEWAQE0Z6MJUC2lz | Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price | Keep | High | in-scope | Scoped BookLib limit pointer behavior and no public-known overlap. |
| M-13 / NQf3g55GIedsx5L-68UxR | Expired order cleanup can be inflated to brick matching at the top of book | Keep | High | in-scope | CLOB and AccountManager settlement paths are scoped and expired cleanup is not a known issue. |
| M-14 / KhAD_M0uCSUU4P5Rw577p | Expired order flood causes unbounded CLOB matching gas and can brick router fill routes | Keep | High | in-scope | CLOB matching and GTERouter fill routes are scoped. |
| M-15 / LOsUmoQXuZsyjF_Gi48ug | Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args | Keep | High | in-scope | Scoped CLOB amend path and distinct from the public transient max-limit amend issue. |
| M-16 / -bEdt8ojcXBsHOvXu74wP | Fee-on-transfer deposits overcredit AccountManager balances and can make withdrawals insolvent | Keep | Medium | in-scope | AccountManager deposits are scoped and benchmark docs do not classify fee-on-transfer behavior as out of scope for round 1. |
| M-17 / UeiY1mitQGuD0qzyGErYn | Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price | Keep | High | in-scope | Scoped BookLib append and removal behavior with no known-issue blocker. |
| H-18 / 6Uh2u3bVS4waCHjG-fjsj | Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching | Keep | High | in-scope | CLOB and BookLib are scoped and the permissionless cancel path is not excluded. |
| M-19 / m2sqyBVvdOeZDLv7wxPl1 | Unbounded expired-order cleanup lets makers gas-DoS fills at the best price | Keep | High | in-scope | Scoped CLOB matching and not a public known issue. |
| M-20 / NipRYiPzXhZll_OuCOzOd | Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity | Keep | High | in-scope | GTERouter and CLOB fill paths are scoped and no scope exclusion applies. |
| M-21 / AB3Otlvf037zOTvgUcwhh | Same-price order cancellation corrupts BookLib limits and blocks matching at the best price | Keep | High | in-scope | Scoped BookLib limit removal behavior with no known-issue overlap. |
| M-22 / Y8bBwzzHDvU11PZtarwlE | Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price | Keep | High | in-scope | Scoped CLOB cancellation and order queue behavior. |
| M-23 / 9DMzwZBvN1i2QQTvRs_0Q | Zero-quote dust orders can permanently block one side of a CLOB market | Keep | Medium | in-scope | Scoped CLOB matching and alleged blocking impact is materially distinct from mere disclosed fill dust. |
| M-24 / 2Cf1SI_P2vh8pQZjuiEjt | Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price | Keep | High | in-scope | BookLib add and remove paths are scoped and not benchmark-declared known. |
| M-25 / RdhlzdOHuMq96aCAhrX5A | Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth | Keep | High | in-scope | Scoped CLOB limit eviction and order-flooding controls are sponsor review priorities. |
| M-26 / 7oFfTRZTnC8m9HiDpKPa1 | Lot-size increases can strand existing orders below the new lot and block matching | Exclude | Medium | configuration-nonissue | Depends only on trusted manager-only lot-size configuration changing live order assumptions. |
| M-27 / WSYqCqHZyebvGG35AyqJv | Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide | Keep | High | in-scope | Scoped CLOB order-flooding control behavior and no known-issue exclusion. |
| L-28 / zEdopivUepNxTT_LKsrN7 | Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies | Keep | High | in-scope | Scoped amend behavior and distinct from the disclosed transient limit amend issue. |
| H-29 / fc-HXKUWbDnoz-SIy46sV | Same-price order append corrupts CLOB queue and can brick matching at the best price | Keep | High | in-scope | Scoped BookLib queue behavior with permissionless maker actions. |
| M-30 / EANPmE7_tnXWUY7__k2Xm | Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains | Keep | Medium | in-scope | GTERouter is scoped and docs do not declare non-mainnet guard behavior out of scope. |
| M-31 / -uTFZlqtkstf85f3lJ1Ox | Expired best-price order queues create unbounded matching work and can gas-brick market progress | Keep | High | in-scope | Scoped CLOB matching and expired order queues are not known public issues. |
| M-32 / 0tg5m2yofCOr3Vy6kSVNm | Live lot-size changes can make existing top-of-book orders unmatchable | Exclude | Medium | configuration-nonissue | Depends only on trusted manager-only lot-size configuration changing live order assumptions. |
| M-33 / WJeKSESLnz563ldIydVzn | Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price | Keep | High | in-scope | Scoped CLOB and BookLib cancellation behavior with no known-issue blocker. |
| M-34 / rdcQt1XKFsNqvAV7390mj | Nominal router deposits over-credit accounts for fee-on-transfer market tokens | Keep | Medium | in-scope | GTERouter and AccountManager deposits are scoped and token-behavior screening is not a round 1 exclusion from docs. |
| M-35 / XKtuQBa7uy7q6duOwgi1U | getNextOrders skips lower bid levels by always walking to the next bigger price | Keep | High | in-scope | BookLib and limit lens style views are scoped and not declared documentation-only. |
| L-36 / VTB6di026hozl6UG7W92S | Per-fill fee flooring lets traders split orders to avoid maker and taker fees | Keep | High | in-scope | FeeData is scoped and this fee-flooring claim is not the disclosed CLOB fill dust issue. |
| M-37 / 9E90WTZyPojpmbspEf0t0 | Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas | Keep | High | in-scope | GTERouter and CLOB matching paths are scoped and not public known issues. |
| M-38 / hRZ--CyWJFFnL-AMLE9Fb | Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price | Keep | High | in-scope | Scoped CLOB cancellation and BookLib queue behavior. |
| M-39 / 153JZItTsyyl20QBa0ugC | Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price | Keep | High | in-scope | Scoped BookLib and CLOB matching behavior with no known-issue blocker. |
| M-40 / nqPYgqKnbPRtrs3G-Y_1c | Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price | Keep | High | in-scope | Scoped CLOB post and cancel path and not a benchmark-declared exclusion. |
| M-41 / _6-2VSOzJzLq5VgATL8RN | Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price | Keep | High | in-scope | Scoped BookLib removal behavior and permissionless order actions. |
| M-42 / NmnsJOJu7S9-cuJppTnev | Nominal router deposits overcredit AccountManager for fee-on-transfer market tokens | Keep | Medium | in-scope | Router and AccountManager deposit paths are scoped and docs do not make fee-on-transfer behavior a round 1 blocker. |
| M-43 / MsApEnZhb8xJbreCI0cFX | Authorized tick-size update lets zero-quote asks permanently block buy-side matching | Exclude | High | configuration-nonissue | Harm depends on a trusted tick-size setter choosing an unsafe market parameter. |
| M-44 / UQjLUNPmihJvNMUkkW81r | Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching | Keep | High | in-scope | Scoped BookLib and CLOB cancellation behavior with no public-known overlap. |
| M-45 / ejeE9u68HPOend0p-7kCj | Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode | Keep | Medium | in-scope | GTERouter is scoped and non-mainnet behavior is not benchmark-declared out of scope. |
| H-46 / luMc8bQJZFJe5uuP0Lnl5 | Canceling an appended order corrupts the FIFO list and bricks matching at that price | Keep | High | in-scope | Scoped BookLib FIFO behavior and permissionless CLOB cancellation. |
| M-47 / C4opETU5jlu8R9w5AyDv0 | GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized | Keep | Medium | in-scope | GTERouter is scoped and docs do not classify chain-specific guard behavior as deployment-only. |
| M-48 / vWmsLvCofS-GL6ZvlABOI | Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask | Keep | High | in-scope | Scoped BookLib and CLOB matching behavior with no known-issue exclusion. |
| M-49 / dknnew5I8_c7ttT34fiMV | Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills | Keep | High | in-scope | CLOB and AccountManager settlement paths are scoped and expired cleanup is not known. |
| M-50 / zGQXAkrWqG1uci8hIrcI- | DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder | Keep | High | in-scope | Scoped BookLib append behavior and no public-known overlap. |
| M-51 / AvI34sCVIJRryIjqtw_cN | Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price | Keep | High | in-scope | Scoped CLOB cancel path and permissionless maker action. |
| M-52 / S7in-uafMZq650UhBgUK0 | GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains | Keep | Medium | in-scope | GTERouter is scoped and benchmark docs do not declare non-mainnet deployment behavior out of scope. |
| M-53 / jiTlrD6mS8stMUieQXaAV | Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side | Keep | High | in-scope | GTERouter executeRoute and CLOB matching are scoped. |
| M-54 / CI1pMro9PUu-pjukfJxxj | Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders | Keep | High | in-scope | Scoped CLOB limit order path and not the public fill-rounding known issue. |
| M-55 / jUMknNEuqkC60UN4wp2td | BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching | Keep | High | in-scope | Scoped BookLib append behavior with no known-issue blocker. |
| M-56 / 862phQLIcBWjMjtUglV8a | Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts | Keep | High | in-scope | Scoped CLOB limit matching and no scope exclusion applies. |
| L-57 / 2hWPwrRLWp-9Cy5L-441x | Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks | Keep | High | in-scope | Scoped CLOB limit matching and distinct from disclosed fill dust. |
| M-58 / 3r67ZUXco89RLvFbEmeiB | Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching | Keep | High | in-scope | Scoped CLOB post and cancel path with no known-issue blocker. |
| M-59 / 8w065WL0FE9FUGvl4vSXj | Unlimited orders per price level can make CLOB matching and settlement exceed block gas | Keep | High | in-scope | Scoped CLOB matching and AccountManager settlement behavior. |
| M-60 / lIFEP3RQX99oSBh6CEhyT | Unbounded expired order clearing can brick CLOB matching at the top of book | Keep | High | in-scope | Scoped CLOB matching and expired cleanup is not public known. |
| M-61 / 8efy2NB9lB-h6l5RBXVFp | Unbounded expired top-of-book cleanup can make matching exceed gas and block fills | Keep | High | in-scope | Scoped CLOB matching path and no known-issue exclusion. |
| M-62 / wG0yKv8bfAxS8cfQ-jBgy | Bitwise zero-cost check rejects valid nonzero CLOB limit fills | Keep | High | in-scope | Scoped CLOB limit matching and no public-known overlap. |
| M-63 / rOuvmt_HeIakChKQ1M8uP | Unbounded maker credit settlement lets large fills exceed gas and DoS order matching | Keep | High | in-scope | AccountManager settlement and CLOB maker credit flow are scoped. |
| M-64 / -0eZ9DmvhCPURywmjaWdQ | Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes | Keep | High | in-scope | CLOB fills and GTERouter CLOB routes are scoped and not known issues. |
| M-65 / 7NpX1q1QAy0pvHbmh_VvF | Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup | Keep | High | in-scope | Scoped CLOB cleanup and fill behavior with no scope blocker. |
| M-66 / CvvbhB2_m67scVPlc4Knw | Expired top-of-book orders can gas-DoS CLOB fills and routed trades | Keep | High | in-scope | CLOB matching and routed trades through GTERouter are scoped. |
| M-67 / V7oC8eZirDDRKdSOpU5_S | Unlimited same-price dust orders can make CLOB fills exceed practical gas limits | Keep | High | in-scope | Scoped order-count and settlement gas behavior not declared known. |
| H-68 / oBFwqzifOb7uy-Lkb1tK- | Reentrant token can withdraw pre-existing AccountManager liquidity during deposit before funding the new credit | Keep | Medium | in-scope | AccountManager deposit is scoped and token-behavior screening is not a round 1 exclusion from docs. |
| M-69 / ihRh47bUNlVaErxYytyq1 | Fee-on-transfer deposits credit more internal balance than AccountManager actually receives | Keep | Medium | in-scope | AccountManager deposit is scoped and docs do not classify fee-on-transfer assets as out of scope for this screen. |
| L-70 / MQILBNE2yqCT59JTgYiZQ | creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting | Keep | High | in-scope | AccountManager credit paths are scoped and this is not a documentation-only scope issue. |
| M-71 / DzJ-BBAE-EMiZlNn20JUJ | Failed fee transfer permanently clears unclaimed fees in AccountManager.collectFees | Keep | Medium | in-scope | Fee collection path is scoped and trusted role acts through normal collection rather than malicious misuse. |
| M-72 / 2F-o7dt6Wwd3vQcFKEZQf | Manager setting updates can admit zero-quote orders that brick one side of a CLOB market | Exclude | High | configuration-nonissue | Harm depends on trusted setters choosing an unsafe tick or minimum-order configuration. |
| M-73 / 0pUyX3n9XiEdRjh5tEN-Z | Live fee-tier reads let routine tier changes alter already-resting order settlement | Keep | Medium | in-scope | Fee-tier update path is scoped and no benchmark doc marks live fee reads as known or out of scope. |
