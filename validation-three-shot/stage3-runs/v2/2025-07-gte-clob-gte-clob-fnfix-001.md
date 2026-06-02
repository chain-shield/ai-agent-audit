# 2025-07-gte-clob Three-Shot Stage 3 gte-clob-fnfix-001

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

## Per-Finding Validation

### M-1 / `U2Hhi3vbiZqDpjxJh6hmv`
- Finding Title: Broken same-price order links let a maker cancel the tail order and brick the best price level
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The report proves a live in-scope linked-list corruption. A normal maker can append at an existing price and cancel its own tail order; because the stored tail has no predecessor, the remaining live order becomes unreachable while the price remains indexed. This is a permissionless market-availability failure, but the impact is best assessed as Medium rather than High because it bricks a price/side until affected makers cancel or the book is repaired, without direct theft.
- Code Evidence: `contracts/clob/types/Book.sol` stores `self.orders[order.id] = order` in `_updateBookPostOrder` before `_updateLimitPostOrder` assigns `order.prevOrderId` only in memory; `_updateLimitRemoveOrder` then sets `headOrder` and `tailOrder` from zero links. `contracts/clob/CLOB.sol` `_matchIncomingBid` / `_matchIncomingAsk` read `orders[limit.headOrder]`, hit order zero, and fill processing reverts through `ZeroCostTrade`.

### M-2 / `ntTDyuzQFbVoG65j59b0Y`
- Finding Title: Tail cancellation corrupts same-price order queue and bricks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The same-price append and tail-cancel path exists exactly as described and is reachable by ordinary order placement and cancellation. The corrupted limit keeps a positive `numOrders` and tree membership but loses its head/tail pointers, so taker fills cannot consume the older order. This is a material CLOB liveness bug and satisfies Medium severity.
- Code Evidence: `BookLib.addOrderToBook` calls `_updateBookPostOrder` before `_updateLimitPostOrder`, leaving appended orders in storage with `prevOrderId == 0`. `BookLib.removeOrderFromBook` uses `_updateLimitRemoveOrder`, and `CLOB._processFillBidOrder` / `_processFillAskOrder` revert when the corrupted best price produces zero matched amounts.

### M-3 / `OWYDJ_ICDupdBAfo1R4VA`
- Finding Title: Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Expired top-of-book asks are removed only inside the matching loop and with no iteration cap or standalone bounded cleanup route. A permissionless maker can accumulate many valid expiring orders over multiple transactions; later buyers must clear that prefix in one transaction, and out-of-gas reverts all cleanup. The benchmark docs explicitly prioritize order-flooding and note no admin cancel, so this is a Medium liveness issue.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingBid` loops while the ask crosses, calls `_removeExpiredAsk`, then continues without reducing the incoming amount. `BookLib.incrementLimitsPlaced` only throttles per transaction, while `maxNumLimitsPerSide` bounds price levels rather than FIFO depth at one price.

### M-4 / `qeqdYm-KXE7yOpIWJSlb9`
- Finding Title: Expired top-of-book orders require unbounded cleanup before any matching can proceed
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Both bid and ask matching remove expired head orders inline without a per-call limit. Because same-price order count can grow across transactions, an attacker can make every crossing taker process an unbounded expired queue before live liquidity. This is not just rare timing; it is a credible permissionless state-growth DoS against core fills.
- Code Evidence: `CLOB._matchIncomingBid` and `_matchIncomingAsk` call `_removeExpiredAsk` / `_removeExpiredBid` inside unbounded `while` loops. Expired removals call `BookLib.removeOrderFromBook`, and failed fills or out-of-gas revert the entire cleanup.

### M-5 / `Fj0qTYNYHzzhvXQcrR0IJ`
- Finding Title: Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost-predicate`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The limit-order zero-cost guard is a bitwise overlap test, not a logical zero-side test. Positive base and quote amounts with disjoint binary bits can revert after a real match, blocking valid marketable limit orders against crafted or naturally occurring liquidity. This affects an in-scope core order path and is Medium.
- Code Evidence: `contracts/clob/CLOB.sol` `_processLimitBidOrder` checks `baseTokenAmountReceived & quoteTokenAmountSent == 0`, and `_processLimitAskOrder` mirrors it. The fill-order processors use explicit `== 0 || == 0` checks, confirming the limit-order predicate is the divergent path.

### L-6 / `cOtLAQEI7uTkuEyPPoAZh`
- Finding Title: Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `post-only-stale-expiry-gate`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause`
- Checklist Gates Failed: `H/M materiality`
- Detailed Reason: The stale-expired-order gate exists: post-only checks use the raw best bid/ask before lazy expiry cleanup. The impact is limited to post-only quoting, because non-post-only limit paths can clear expired orders while posting, and no direct fund loss is shown. This is a real correctness issue but does not reach H/M materiality on its own.
- Code Evidence: `CLOB._executeBidLimitOrder` checks `ds.getBestAskPrice() <= newOrder.price` before `_matchIncomingBid`; `_executeAskLimitOrder` checks `ds.getBestBidPrice() >= newOrder.price` before `_matchIncomingAsk`. Expiry is only handled later in the matching loops through `OrderLib.isExpired`.

### M-8 / `SNePL7SCfZJ5uSjAhF0yD`
- Finding Title: Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding identifies the same live storage-linking bug: appended orders are persisted before their predecessor pointer is assigned, and tail cancellation clears a nonempty limit's links. A permissionless maker can strand older liquidity and make fills at the indexed best price revert. This is a Medium availability failure.
- Code Evidence: `BookLib._updateBookPostOrder` writes the order to `orders`, `_updateLimitPostOrder` mutates only the memory `order.prevOrderId`, and `_updateLimitRemoveOrder` later treats the stored tail as a head. `CLOB._matchIncomingBid` / `_matchIncomingAsk` then load a null order from `limit.headOrder`.

### M-9 / `3QpPKCdYzCkBLpAcsNNz6`
- Finding Title: Appended order cancellation corrupts a price level and makes older resting orders unmatchable
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The described appended-order cancellation corrupts an in-scope price level and leaves an older order stored but unreachable from the limit head. The attacker only needs normal maker permissions over its own order, and the existing safeguards do not validate linked-list integrity. This is Medium due to material matching DoS without direct asset theft.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateLimitPostOrder` updates `tailOrder.nextOrderId` in storage but not the new order's `prevOrderId` in storage. Cancellation through `CLOB.cancel` reaches `BookLib.removeOrderFromBook`, leaving `numOrders > 0` with zero head/tail pointers.

### M-10 / `ulpg6Or9uGZvpzvldYeKZ`
- Finding Title: Tail cancellation corrupts same-price FIFO queue and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The FIFO queue corruption is real and permissionless. Once the appended tail is canceled, the affected price level remains in the tree but matching reads order zero instead of the still-live order. That breaks core fills at the best price and is a Medium DoS.
- Code Evidence: `BookLib._updateBookPostOrder` line of control stores the order before link completion; `BookLib._updateLimitRemoveOrder` sets `limit.headOrder = next` and `limit.tailOrder = prev` using zero links. `CLOB._processFillBidOrder` and `_processFillAskOrder` reject the resulting zero-cost fill.

### M-11 / `Xst74MzxPAZmmiNOcO__7`
- Finding Title: Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The queue corruption path exists for normal post and cancel calls. The remaining maker order is still locked in storage and open interest, but matching begins at a null head and cannot consume it. The impact is a material, permissionless top-of-book DoS, assessed as Medium.
- Code Evidence: `BookLib.addOrderToBook` delegates to `_updateBookPostOrder` then `_updateLimitPostOrder`; the latter does not persist the new order's predecessor. `CLOB.cancel` invokes `_executeCancel`, which calls `ds.removeOrderFromBook(order)` on that malformed stored order.

### M-12 / `cXOP9qEWAQE0Z6MJUC2lz`
- Finding Title: Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding proves the same independent root on its own code path: same-price append stores an incomplete order, and canceling that tail zeroes a nonempty limit. The attack is cheap because the attacker cancels and recovers its own order while stranding existing liquidity. This reaches Medium market-availability impact.
- Code Evidence: `BookLib._updateBookPostOrder` persists the order before `order.prevOrderId = tailOrder.id`; `_updateLimitRemoveOrder` then clears head/tail when both stored links are zero. `CLOB._matchIncomingBid` / `_matchIncomingAsk` break on zero `baseDelta`, and fill processing reverts.

### M-13 / `NQf3g55GIedsx5L-68UxR`
- Finding Title: Expired order cleanup can be inflated to brick matching at the top of book
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Cleanup of expired top-of-book orders is state-derived and unbounded, and each expired removal can add maker refund credits that later have to be materialized in settlement. The attacker can accumulate the queue through normal minimum-size orders over time, so the path is practical and in scope. This is Medium because it can block core matching progress.
- Code Evidence: `CLOB._matchIncomingBid` / `_matchIncomingAsk` remove expired orders inline; `_removeExpiredAsk` / `_removeExpiredBid` add transient maker credits. `_settleIncomingOrder` converts all transient makers into an array, and `AccountManager.settleIncomingOrder` loops through every maker credit.

### M-14 / `KhAD_M0uCSUU4P5Rw577p`
- Finding Title: Expired order flood causes unbounded CLOB matching gas and can brick router fill routes
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The CLOB matching loop has no bound on expired head removals, and router CLOB fills use the same CLOB fill path. A permissionless maker can therefore make direct fills and routed CLOB fills consume unbounded cleanup before live matching. The docs identify order-flooding as a sponsor concern, so this is a Medium liveness bug.
- Code Evidence: `CLOB._matchIncomingBid` and `_matchIncomingAsk` perform unbounded expired removal. `contracts/router/GTERouter.sol` `_executeClobPostFillOrder` calls `ICLOB.postFillOrder` with a `FILL_OR_KILL` fill and no cleanup cap.

### M-15 / `LOsUmoQXuZsyjF_Gi48ug`
- Finding Title: Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `same-price-amend-stale-expiry`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause`
- Checklist Gates Failed: `H/M materiality`
- Detailed Reason: Same-price/same-side amendments validate a new cancel timestamp and emit it, but storage keeps the old expiry. The bug can make the maker's own order expire earlier than the successful amend indicates. That is a real state inconsistency, but the impact is self-affecting order availability/refund behavior rather than a credible third-party H/M loss or DoS.
- Code Evidence: `CLOB._processAmend` validates `args.cancelTimestamp`, then calls `_executeAmendAmount` when side and price are unchanged. `_executeAmendAmount` updates only amount and open interest; only `_executeAmendNewOrder` writes `newOrder.cancelTimestamp`.

### M-17 / `UeiY1mitQGuD0qzyGErYn`
- Finding Title: Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The report's root cause is present in the production `BookLib` append/remove path. Canceling a stored non-head order with a zero predecessor corrupts the limit pointers and leaves matching stuck at a null order. This is a realistic permissionless DoS of a populated price level and is Medium.
- Code Evidence: `BookLib._updateBookPostOrder` writes storage too early, `_updateLimitPostOrder` fails to persist `prevOrderId`, and `_updateLimitRemoveOrder` uses the malformed links. `CLOB._matchIncomingOrder` returns zero when the maker order is null, leading fill processors to revert.

### H-18 / `6Uh2u3bVS4waCHjG-fjsj`
- Finding Title: Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The corruption exists and is reachable without privileged action, but the demonstrated harm is a market liveness failure at affected price levels rather than direct theft, insolvency, or permanent fund lock. It should therefore be kept as valid Medium, not High.
- Code Evidence: `BookLib._updateLimitPostOrder` sets `order.prevOrderId` only on the memory copy after `self.orders[order.id]` was already written. `BookLib._updateLimitRemoveOrder` clears links from zero prev/next values, and `CLOB._matchIncomingBid` / `_matchIncomingAsk` subsequently operate on `orders[0]`.

### M-19 / `m2sqyBVvdOeZDLv7wxPl1`
- Finding Title: Unbounded expired-order cleanup lets makers gas-DoS fills at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The expired-order cleanup cost is controlled by accumulated book state, not by caller input or a protocol cap. Attackers can create many expiring best-price orders over multiple transactions, and every crossing fill must process them before reaching live liquidity. Failed cleanup rolls back, making this a Medium DoS.
- Code Evidence: `CLOB._matchIncomingBid` and `_matchIncomingAsk` remove expired head orders in unbounded loops and continue. `BookLib.incrementLimitsPlaced` is only a per-transaction throttle and does not cap total same-price order count.

### M-20 / `NipRYiPzXhZll_OuCOzOd`
- Finding Title: Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-expired-cleanup-dos`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The router CLOB fill route delegates into `CLOB.postFillOrder`, so the unbounded expired-order cleanup bug directly affects routed trades. A maker can create an expired top-of-book prefix that causes router fills to exceed gas before any live liquidity is reached. This is a Medium liveness failure for an in-scope router integration.
- Code Evidence: `GTERouter._executeClobPostFillOrder` constructs a `FILL_OR_KILL` `PostFillOrderArgs` and calls `ICLOB.postFillOrder`. `CLOB._matchIncomingBid` / `_matchIncomingAsk` perform all expired cleanup inside the matching transaction with no cap.

### M-21 / `AB3Otlvf037zOTvgUcwhh`
- Finding Title: Same-price order cancellation corrupts BookLib limits and blocks matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The BookLib limit invariant is broken by normal append and cancel operations. Once a tail order with a missing predecessor is removed, the remaining order cannot be reached through the limit head. This is a valid Medium DoS of the best price.
- Code Evidence: `BookLib._updateBookPostOrder` and `_updateLimitPostOrder` split storage write from link assignment incorrectly. `CLOB._executeCancel` calls `ds.removeOrderFromBook(order)`, and later matching uses the corrupted `Limit.headOrder`.

### M-22 / `Y8bBwzzHDvU11PZtarwlE`
- Finding Title: Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The vulnerable ordering is present and no linked-list integrity check repairs it. A permissionless account can append behind an existing order and cancel its own tail, leaving the tree pointed at an unmatchable nonempty limit. This is a Medium matching DoS.
- Code Evidence: In `Book.sol`, `_updateBookPostOrder` writes `self.orders[order.id] = order`, while `_updateLimitPostOrder` later assigns `order.prevOrderId` on memory only. `_updateLimitRemoveOrder` then uses null `prev` and `next` values to zero the limit pointers.

### M-23 / `9DMzwZBvN1i2QQTvRs_0Q`
- Finding Title: Zero-quote dust orders can permanently block one side of a CLOB market
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `zero-quote-dust-remainder`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The code can leave a partially filled resting order below the minimum amount, and at valid low prices that remainder can have nonzero base but zero quote value. A later fill removes it during matching, then reverts as `ZeroCostTrade`, rolling back the removal and leaving the dust at top of book. This is a severe unintended consequence beyond the public rounding note and is a Medium market DoS.
- Code Evidence: `CLOB._matchIncomingOrder` reduces `makerOrder.amount` on partial fill and computes quote with `BookLib.getQuoteTokenAmount`, which floors. `_processFillBidOrder` / `_processFillAskOrder` revert if one aggregate side is zero, rolling back `ds.removeOrderFromBook` for the dust order.

### M-24 / `2Cf1SI_P2vh8pQZjuiEjt`
- Finding Title: Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Canceling an appended order corrupts the price-level pointers for the same reason as the other same-family reports. The exploit is reachable through ordinary post/cancel operations and blocks fills through an indexed price with live liquidity. This is a valid Medium issue.
- Code Evidence: `BookLib._updateBookPostOrder` persists the appended order before its predecessor link exists. `BookLib._updateLimitRemoveOrder` decrements `numOrders` and sets head/tail from zero links, while `CLOB._matchIncomingBid` / `_matchIncomingAsk` later use the zero head.

### M-25 / `RdhlzdOHuMq96aCAhrX5A`
- Finding Title: Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `single-order-cap-eviction`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The full-book eviction removes only one tail order from the worst price. If that price has multiple orders, the price node remains, then the incoming price is inserted and tree size exceeds the configured cap. Because later checks use equality rather than `>=`, the cap can be bypassed indefinitely. This breaks an explicit anti-flooding control and is Medium.
- Code Evidence: `CLOB._executeBidLimitOrder` and `_executeAskLimitOrder` call `_removeNonCompetitiveOrder` when `tree.size() == maxNumLimitsPerSide`; `_removeNonCompetitiveOrder` calls `BookLib.removeOrderFromBook` on a single order. `BookLib._updateLimitRemoveOrder` only removes the tree node when `limit.numOrders == 1`.

### M-27 / `WSYqCqHZyebvGG35AyqJv`
- Finding Title: Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `single-order-cap-eviction`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The report correctly identifies that pruning a full side removes a single worst-price order, not necessarily the worst price level. A multi-order worst level survives, a new level is inserted, and the equality guard stops firing once the cap is exceeded. This undermines the sponsor-highlighted order-flooding limit and is Medium.
- Code Evidence: `CLOB._executeAskLimitOrder` and `_executeBidLimitOrder` use `if (tree.size() == maxNumLimitsPerSide)` and call `_removeNonCompetitiveOrder` once. `BookLib._updateLimitRemoveOrder` leaves the red-black-tree price present whenever more than one order existed at that limit.

### L-28 / `zEdopivUepNxTT_LKsrN7`
- Finding Title: Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `same-price-amend-stale-expiry`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause`
- Checklist Gates Failed: `H/M materiality`
- Detailed Reason: Same-price amendments accept and emit a new cancel timestamp but do not persist it, so the order can expire according to the old timestamp. The issue is real but primarily affects the maker's own order lifecycle and does not demonstrate third-party H/M loss, unbounded DoS, or unauthorized impact.
- Code Evidence: `CLOB._processAmend` validates `args.cancelTimestamp` and emits `OrderAmended`; `_executeAmendAmount` only adjusts amount and open interest. The stored `Order.cancelTimestamp` is only set in `_executeAmendNewOrder` when price or side changes.

### H-29 / `fc-HXKUWbDnoz-SIy46sV`
- Finding Title: Same-price order append corrupts CLOB queue and can brick matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The same-price append bug is mechanically proven, including the tail-cancel path that leaves a nonempty limit with null head/tail links. The issue is permissionless and materially blocks matching, but the demonstrated harm remains availability/stranded-order impact rather than High-severity theft or insolvency. Medium is the appropriate assessment.
- Code Evidence: `BookLib._updateBookPostOrder` stores the order before `_updateLimitPostOrder` assigns `prevOrderId`; `_updateLimitRemoveOrder` uses the zero stored links. `CLOB._matchIncomingBid` and `_matchIncomingAsk` then load a null order and the fill processors revert on zero-cost output.

### M-30 / `EANPmE7_tnXWUY7__k2Xm`
- Finding Title: Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `unsupported-nonmainnet-router-guard`
- Checklist Gates Passed: `Stage0, bug-exists`
- Checklist Gates Failed: `supported-deployment, materiality`
- Detailed Reason: The Solady guard behavior on `chainid != 1` is real, but the configured benchmark docs do not state that this CLOB/router deployment must support non-mainnet chain IDs. Without that deployment requirement, the harm depends on an unsupported deployment choice rather than a current in-scope H/M runtime invariant. This should be excluded under the deployment/support gate.
- Code Evidence: `GTERouter` inherits `ReentrancyGuardTransient` and applies `nonReentrant` to `executeRoute`, `launchpadBuy`, and `launchpadSell`. `lib/solady/src/utils/ReentrancyGuardTransient.sol` defaults `_useTransientReentrancyGuardOnlyOnMainnet()` to true and uses the storage fallback on non-mainnet chains.

### M-31 / `-uTFZlqtkstf85f3lJ1Ox`
- Finding Title: Expired best-price order queues create unbounded matching work and can gas-brick market progress
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The matching loops synchronously clear expired best-price orders and cannot persist progress if the transaction fails. Since same-price order count is unbounded across transactions, a maker can create an expired FIFO prefix that blocks all crossing takers. This is a Medium core workflow DoS.
- Code Evidence: `CLOB._matchIncomingBid` / `_matchIncomingAsk` call `_removeExpiredAsk` / `_removeExpiredBid` in unbounded loops. `CLOB._processFillBidOrder` and `_processFillAskOrder` revert on zero aggregate fills, rolling back any cleanup that did not reach live liquidity.

### M-33 / `WJeKSESLnz563ldIydVzn`
- Finding Title: Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The report accurately traces the storage write before `prevOrderId` assignment and the malformed tail removal. The corrupted nonempty limit remains indexed and blocks matching at that best price. This is a valid Medium liveness issue.
- Code Evidence: `BookLib._updateBookPostOrder` writes `self.orders[order.id] = order`, while `_updateLimitPostOrder` mutates the memory order. `_updateLimitRemoveOrder` then sets `headOrder` and `tailOrder` to zero for the canceled tail, and CLOB matching starts from that zero head.

### M-35 / `XKtuQBa7uy7q6duOwgi1U`
- Finding Title: getNextOrders skips lower bid levels by always walking to the next bigger price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `bid-pagination-direction`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause`
- Checklist Gates Failed: `H/M materiality`
- Detailed Reason: The view bug exists: bid-side priority should walk to the next lower price, but `getNextOrders` always asks for the next bigger price after a limit is exhausted. The impact is incomplete view pagination for integrations or indexers, not an on-chain matching, settlement, withdrawal, or funds-loss failure. It is valid Low/QA only.
- Code Evidence: `BookLib.getNextOrders` calls `self.getNextBiggestPrice(currentOrder.price, currentOrder.side)` for both sides. `BookLib.getOrdersPaginated` contains the side-specific branch using `getNextSmallestPrice` for buys, showing the intended bid direction.

### L-36 / `VTB6di026hozl6UG7W92S`
- Finding Title: Per-fill fee flooring lets traders split orders to avoid maker and taker fees
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `fee-flooring-split`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause`
- Checklist Gates Failed: `H/M materiality`
- Detailed Reason: Fees are floored per taker amount and per maker credit, so splitting can reduce fee collection. The report does not prove a material H/M loss under expected market settings; for normal high-decimal assets the zero-fee threshold is dust-sized, and repeated splitting has execution/gas costs. This is a real precision concern but only Low/QA on the supplied evidence.
- Code Evidence: `FeeDataLib.getTakerFee` and `getMakerFee` return `amount.fullMulDiv(feeRate, FEE_SCALING)` with floor rounding. `AccountManager.settleIncomingOrder` charges maker fees per `MakerCredit` in a loop rather than accumulating at higher precision.

### M-37 / `9E90WTZyPojpmbspEf0t0`
- Finding Title: Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-expired-cleanup-dos`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Router CLOB routes are exposed through in-scope code and inherit the CLOB's unbounded expired-order cleanup. A permissionless expired queue before live liquidity can make routed fills revert or exceed gas, with cleanup rolled back. This is Medium because it blocks a core integration path.
- Code Evidence: `GTERouter._executeAllHops` dispatches `HopType.CLOB_FILL` to `_executeClobPostFillOrder`, which calls `ICLOB.postFillOrder`. `CLOB._matchIncomingBid` / `_matchIncomingAsk` clear expired orders with no pagination or maximum removals.

### M-38 / `hRZ--CyWJFFnL-AMLE9Fb`
- Finding Title: Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The tail cancellation bug is present in BookLib and is reachable through normal maker actions. The affected best price remains indexed but has no reachable head order, so crossing fills fail. This is a Medium DoS.
- Code Evidence: `BookLib._updateBookPostOrder` persists incomplete appended orders; `_updateLimitPostOrder` only persists the old tail's `nextOrderId`; `_updateLimitRemoveOrder` then clears pointers from the stored zero links. `CLOB.postFillOrder` reaches the corrupted limit through the matching loops.

### M-39 / `153JZItTsyyl20QBa0ugC`
- Finding Title: Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The report proves a real broken linked-list invariant that causes crossing fills to hit a null order at an indexed price. The attacker owns only the appended order and needs no trusted role. This is Medium because it materially blocks matching at the corrupted price.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateBookPostOrder`, `_updateLimitPostOrder`, and `_updateLimitRemoveOrder` contain the vulnerable sequence. `contracts/clob/CLOB.sol` `_processFillBidOrder` / `_processFillAskOrder` revert when matching returns zero totals.

### M-40 / `nqPYgqKnbPRtrs3G-Y_1c`
- Finding Title: Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The same-price order corruption is a live production path. After cancellation of the malformed appended order, remaining liquidity is still stored but no longer reachable from the limit. This is a Medium matching DoS.
- Code Evidence: `BookLib.addOrderToBook` writes storage before link completion, and `BookLib.removeOrderFromBook` updates the limit according to the malformed stored links. `CLOB._matchIncomingOrder` receives an empty maker order from `orders[0]` and makes no progress.

### M-41 / `_6-2VSOzJzLq5VgATL8RN`
- Finding Title: Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The appended order's predecessor pointer is not persisted, so removing it as a tail corrupts the queue. Since any maker can post and cancel its own same-price order, the exploit path is credible and permissionless. The resulting price-level DoS is Medium.
- Code Evidence: `BookLib._updateLimitPostOrder` assigns `order.prevOrderId = tailOrder.id` after `BookLib._updateBookPostOrder` has stored the order. `BookLib._updateLimitRemoveOrder` then zeroes the limit pointers, and CLOB matching loads `orders[limit.headOrder]`.

### M-43 / `MsApEnZhb8xJbreCI0cFX`
- Finding Title: Authorized tick-size update lets zero-quote asks permanently block buy-side matching
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-zero-quote-config`
- Checklist Gates Passed: `Stage0, bug-exists, scope`
- Checklist Gates Failed: `trusted-role, configuration-only`
- Detailed Reason: The zero-quote state can exist after a tick-size update, but entering that state depends on a trusted tick-size setter selecting a parameter combination that violates the creation-time notional invariant. The benchmark docs list tick-size setters as trusted roles for this capability, and no permissionless actor first makes an otherwise safe update unapplyable. Under the delegated-authority/configuration gate, this is not a valid H/M finding.
- Code Evidence: `CLOBManager.setTickSizes` forwards to `CLOB.setTickSize`, and `BookLib.setTickSize` only checks `newTickSize >= 1`. `CLOBManager._assertValidSettings` enforces the nonzero-notional product only during market creation, but the harmful update is still a trusted settings action.

### M-44 / `UQjLUNPmihJvNMUkkW81r`
- Finding Title: Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The same-price tail cancel path is real and produces a nonempty indexed limit with null pointers. A maker can trigger it with ordinary public order placement and cancellation, then crossing takers fail at the corrupted price. This is a Medium DoS.
- Code Evidence: `BookLib._updateBookPostOrder` stores before link assignment, `BookLib._updateLimitPostOrder` fails to persist the new `prevOrderId`, and `BookLib._updateLimitRemoveOrder` clears `headOrder` / `tailOrder`. `CLOB._matchIncomingBid` / `_matchIncomingAsk` consume those corrupted pointers.

### M-45 / `ejeE9u68HPOend0p-7kCj`
- Finding Title: Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `unsupported-nonmainnet-router-guard`
- Checklist Gates Passed: `Stage0, bug-exists`
- Checklist Gates Failed: `supported-deployment, materiality`
- Detailed Reason: The guard behavior is correctly traced, but the H/M claim depends on deployment to non-mainnet chain IDs. The configured benchmark docs do not establish non-mainnet/L2 deployment as required expected operation for this scoped Solidity system. Without that support requirement, the issue is deployment-chain dependent and should not be accepted as a current H/M runtime bug.
- Code Evidence: `GTERouter.executeRoute`, `launchpadBuy`, and `launchpadSell` use `nonReentrant`. `lib/solady/src/utils/ReentrancyGuardTransient.sol` reverts on non-mainnet if the fallback storage sentinel is zero, and `GTERouter` does not override `_useTransientReentrancyGuardOnlyOnMainnet`.

### H-46 / `luMc8bQJZFJe5uuP0Lnl5`
- Finding Title: Canceling an appended order corrupts the FIFO list and bricks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The FIFO corruption is valid and permissionless, but the report demonstrates market availability loss rather than High-severity theft, insolvency, or permanent global lock. It should be retained as Medium.
- Code Evidence: `BookLib._updateBookPostOrder` persists the appended order too early, `_updateLimitPostOrder` does not update its stored predecessor, and `_updateLimitRemoveOrder` clears the nonempty limit. `CLOB._matchIncomingBid` / `_matchIncomingAsk` then cannot progress past the null head.

### M-47 / `C4opETU5jlu8R9w5AyDv0`
- Finding Title: GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `unsupported-nonmainnet-router-guard`
- Checklist Gates Passed: `Stage0, bug-exists`
- Checklist Gates Failed: `supported-deployment, materiality`
- Detailed Reason: The code would revert on the Solady storage-fallback branch for non-mainnet chains, but the benchmark materials do not declare non-mainnet deployments as supported or required. The finding therefore rests on an unsupported deployment assumption, not a proven current H/M issue for the configured benchmark.
- Code Evidence: `GTERouter` uses Solady `ReentrancyGuardTransient` without overriding its mode hook. `ReentrancyGuardTransient.nonReentrant` checks `block.chainid == 1`; otherwise it requires a nonzero storage sentinel that `GTERouter` never initializes.

### M-48 / `vWmsLvCofS-GL6ZvlABOI`
- Finding Title: Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The populated-limit invariant is genuinely broken: a price can remain indexed with positive order count but no reachable head. This is reachable through public maker actions and prevents takers from matching live liquidity at the corrupted best bid or ask. The issue is Medium.
- Code Evidence: `BookLib._updateBookPostOrder` and `_updateLimitPostOrder` produce appended orders with stored `prevOrderId == 0`. `BookLib._updateLimitRemoveOrder` then leaves null pointers, and `CLOB._matchIncomingBid` / `_matchIncomingAsk` start from those null pointers.

### M-49 / `dknnew5I8_c7ttT34fiMV`
- Finding Title: Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Expired-order cleanup and maker-credit settlement are both unbounded by attacker-created order count. Since per-transaction placement limits do not cap accumulated same-price orders, a permissionless actor can force crossing fills to do excessive cleanup and settlement work before reaching honest liquidity. This is a Medium DoS.
- Code Evidence: `CLOB._matchIncomingBid` / `_matchIncomingAsk` remove expired orders inline; `_removeExpiredAsk` / `_removeExpiredBid` add maker credits. `TransientMakerData.getMakerCreditsAndClearStorage` and `AccountManager.settleIncomingOrder` iterate over all affected makers.

### M-50 / `zGQXAkrWqG1uci8hIrcI-`
- Finding Title: DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The root cause exists in `BookLib._updateLimitPostOrder` and causes an indexed price to become unmatchable after a tail cancellation. The exploit path is ordinary public posting and cancellation and creates material trading DoS. This is valid Medium.
- Code Evidence: `BookLib._updateBookPostOrder` writes storage before `_updateLimitPostOrder` finishes links. When `CLOB.cancel` removes the malformed order, `BookLib._updateLimitRemoveOrder` clears the limit, and CLOB fill processors later revert on zero matched amounts.

### M-51 / `AvI34sCVIJRryIjqtw_cN`
- Finding Title: Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding independently proves a real same-price queue corruption. The attacker does not need to control the victim order; it only appends and cancels its own order, leaving older liquidity unreachable. This is a Medium market-availability issue.
- Code Evidence: `BookLib.addOrderToBook` persists new orders before setting predecessor links. `BookLib.removeOrderFromBook` relies on those links, and `CLOB._matchIncomingBid` / `_matchIncomingAsk` read the resulting zero `headOrder`.

### M-52 / `S7in-uafMZq650UhBgUK0`
- Finding Title: GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `unsupported-nonmainnet-router-guard`
- Checklist Gates Passed: `Stage0, bug-exists`
- Checklist Gates Failed: `supported-deployment, materiality`
- Detailed Reason: The non-mainnet revert condition exists, but the report does not tie it to a deployment chain supported by the benchmark docs. Under the validation prompt, unsupported deployment assumptions and setup-only failures are not current H/M vulnerabilities. This should be rejected unless separate scope evidence establishes non-mainnet deployment as required.
- Code Evidence: `GTERouter.executeRoute` is guarded by `nonReentrant`, and Solady `ReentrancyGuardTransient` defaults to mainnet-only transient mode with a storage fallback on other chain IDs. `GTERouter` has no initialization of that fallback guard slot.

### M-53 / `jiTlrD6mS8stMUieQXaAV`
- Finding Title: Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-expired-cleanup-dos`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: `executeRoute` CLOB hops have no separate cleanup bound and rely on the vulnerable CLOB fill logic. An expired prefix accumulated over many transactions can make the routed fill exceed gas and roll back cleanup, leaving the side bricked. This is Medium.
- Code Evidence: `GTERouter.executeRoute` calls `_executeAllHops`, then `_executeClobPostFillOrder` for `CLOB_FILL`; that calls `ICLOB.postFillOrder`. The CLOB matching functions clear expired top-of-book orders in unbounded loops.

### M-54 / `CI1pMro9PUu-pjukfJxxj`
- Finding Title: Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost-predicate`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The zero-cost predicate rejects positive matched amounts with no shared set bits, so valid marketable limit orders can fail even when both sides are nonzero. This is not an unsupported token edge; it is integer logic in the in-scope CLOB. The impact on a core order type is Medium.
- Code Evidence: `CLOB._processLimitBidOrder` and `_processLimitAskOrder` use bitwise `&` between base and quote amounts. `BookLib.getQuoteTokenAmount` supplies the floored quote amount, and no later safeguard retries or settles the valid match.

### M-55 / `jUMknNEuqkC60UN4wp2td`
- Finding Title: BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: BookLib really stores appended orders without their predecessor, and later removal of that order corrupts the same-price queue. Normal makers can trigger the path, and the result is blocked matching at an indexed price. This is a valid Medium issue.
- Code Evidence: `BookLib._updateBookPostOrder` stores `self.orders[order.id]`; `_updateLimitPostOrder` later sets only the memory `order.prevOrderId`. `BookLib._updateLimitRemoveOrder` and `CLOB._matchIncomingBid` / `_matchIncomingAsk` demonstrate the resulting corrupted queue and zero-fill failure.

### M-56 / `862phQLIcBWjMjtUglV8a`
- Finding Title: Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost-predicate`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The bitwise guard can revert valid nonzero limit-order matches solely because the base and quote quantities have disjoint binary representations. The path is reachable through normal maker/taker order flow and affects core CLOB limit execution. This is Medium.
- Code Evidence: `CLOB._processLimitBidOrder` checks `(baseTokenAmountReceived & quoteTokenAmountSent) == 0`, and `_processLimitAskOrder` has the symmetric check. The fill-order paths do not use this predicate.

### L-57 / `2hWPwrRLWp-9Cy5L-441x`
- Finding Title: Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost-predicate`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Despite the low label, the report describes the same material limit-order execution bug: valid nonzero matched amounts can revert under the bitwise predicate. This impacts the in-scope marketable limit path and can block users from taking available quotes. The code evidence supports Medium.
- Code Evidence: `CLOB._processLimitBidOrder` and `_processLimitAskOrder` implement the flawed bitwise `ZeroCostTrade` check after matching. `CLOB._matchIncomingOrder` can produce positive base and quote deltas, and no safeguard converts the bitwise predicate into a true zero-side check.

### M-58 / `3r67ZUXco89RLvFbEmeiB`
- Finding Title: Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `appended-order-prev-pointer`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The linked-list corruption is real for shared price levels and is reachable by canceling the attacker's own appended tail. The remaining live order becomes unreachable while the price remains in the tree. This is Medium.
- Code Evidence: `BookLib._updateBookPostOrder` and `_updateLimitPostOrder` leave stored appended orders with zero `prevOrderId`; `_updateLimitRemoveOrder` treats such a tail as a head. CLOB matching then reads the zero head and fails to fill.

### M-59 / `8w065WL0FE9FUGvl4vSXj`
- Finding Title: Unlimited orders per price level can make CLOB matching and settlement exceed block gas
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-order-count-gas`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The code does not cap same-price FIFO depth or maker-credit batch size, so matching a large order through many minimum-size makers can exceed practical gas. This is more economically constrained than the expired-order variant because live orders are executable liquidity, but the sponsor docs specifically identify order-flooding and many-orders-to-clear as a concern. The finding should be kept as Medium for recall.
- Code Evidence: `CLOB._matchIncomingBid` / `_matchIncomingAsk` loop through resting orders until the incoming amount is filled, and `_settleIncomingOrder` passes all makers to `AccountManager.settleIncomingOrder`. `AccountManager.settleIncomingOrder` then loops over `params.makerCredits` without a cap.

### M-60 / `lIFEP3RQX99oSBh6CEhyT`
- Finding Title: Unbounded expired order clearing can brick CLOB matching at the top of book
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Expired head orders are cleared only during matching and with no maximum cleanup count. An expired FIFO prefix can be large enough that every fill reverts and restores the same queue. This is a Medium permissionless DoS of core matching.
- Code Evidence: `CLOB._matchIncomingBid` and `_matchIncomingAsk` call `_removeExpiredAsk` / `_removeExpiredBid` and continue. Cleanup is not paginated, and `CLOB._processFillBidOrder` / `_processFillAskOrder` revert if no nonzero trade is ultimately produced.

### M-61 / `8efy2NB9lB-h6l5RBXVFp`
- Finding Title: Unbounded expired top-of-book cleanup can make matching exceed gas and block fills
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The report proves a live unbounded cleanup path: no per-price order cap, no cleanup batch size, and no separate expire function. A permissionless maker can accumulate expired best-price orders and force later fills to clear them in a single transaction. This is Medium.
- Code Evidence: `CLOB._matchIncomingBid` / `_matchIncomingAsk` are bounded only by book state and incoming amount. `BookLib.incrementLimitsPlaced` does not bound accumulated order count, and expired removals revert if the fill transaction reverts.

### M-62 / `wG0yKv8bfAxS8cfQ-jBgy`
- Finding Title: Bitwise zero-cost check rejects valid nonzero CLOB limit fills
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost-predicate`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The zero-cost predicate is not equivalent to checking either side for zero, and it can reject a positive base/quote match. The bug is in current in-scope CLOB limit processing and requires only normal order flow. This is Medium.
- Code Evidence: `CLOB._processLimitBidOrder` and `_processLimitAskOrder` use the bitwise `&` expression after `_executeBidLimitOrder` / `_executeAskLimitOrder` return matched amounts. Positive values like 1 and 2 can satisfy the revert condition even though neither side is zero.

### M-63 / `rOuvmt_HeIakChKQ1M8uP`
- Finding Title: Unbounded maker credit settlement lets large fills exceed gas and DoS order matching
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `maker-credit-unbounded-settlement`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Maker-credit settlement work scales with the number of distinct makers touched by one fill, and the book does not cap same-price makers accumulated over time. The path is more capital/execution-risk constrained than expired-order cleanup, but it is a real unbounded gas surface in a sponsor-highlighted order-flooding area. Medium is justified.
- Code Evidence: `TransientMakerData.addQuoteToken` / `addBaseToken` add a maker entry the first time a maker is credited in a transaction. `_settleIncomingOrder` calls `getMakerCreditsAndClearStorage`, and `AccountManager.settleIncomingOrder` iterates all maker credits to apply fees and balances.

### M-64 / `-0eZ9DmvhCPURywmjaWdQ`
- Finding Title: Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-expired-cleanup-dos`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Direct CLOB fills and router CLOB_FILL routes share the same unbounded expired-order cleanup. Per-transaction limits and minimum order size do not cap accumulated expired queues, so a permissionless maker can cause repeated fill reverts and block live liquidity. This is Medium.
- Code Evidence: `CLOB.postFillOrder` reaches `_matchIncomingBid` / `_matchIncomingAsk`; `GTERouter._executeClobPostFillOrder` also calls `ICLOB.postFillOrder`. Both rely on the same unbounded expired-head removal logic in `CLOB.sol`.

### M-65 / `7NpX1q1QAy0pvHbmh_VvF`
- Finding Title: Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-reverting-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The cleanup path can revert after removing expired orders when no live trade produces nonzero totals, which rolls back the cleanup and leaves the same expired top-of-book state. This makes expired-order grief persistent unless a successful high-gas trade reaches live liquidity or owners cancel. That is a Medium matching liveness issue.
- Code Evidence: `_matchIncomingBid` / `_matchIncomingAsk` remove expired orders and add maker refunds, but `_processFillBidOrder` / `_processFillAskOrder` revert with `ZeroCostTrade` if total quote or base is zero. The revert restores all removed expired orders.

### M-66 / `CvvbhB2_m67scVPlc4Knw`
- Finding Title: Expired top-of-book orders can gas-DoS CLOB fills and routed trades
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-top-of-book-unbounded-cleanup`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, exploitability, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: The report accurately describes a bounded-gas failure mode where expired head orders must all be removed before a live order can be reached. If the call runs out of gas, cleanup rolls back and repeated victims make no progress. This impacts direct fills and routed CLOB trades and is Medium.
- Code Evidence: `CLOB._matchIncomingBid` and `_matchIncomingAsk` perform expired-order removal inside unbounded while loops. `GTERouter._executeClobPostFillOrder` feeds routed CLOB fills into the same `postFillOrder` path.

### M-67 / `V7oC8eZirDDRKdSOpU5_S`
- Finding Title: Unlimited same-price dust orders can make CLOB fills exceed practical gas limits
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-order-count-gas`
- Checklist Gates Passed: `Stage0, bug-exists, scope, supported-behavior, root-cause, safeguards, materiality`
- Checklist Gates Failed: `-`
- Detailed Reason: Same-price order count is unbounded, and consuming a large FIFO prefix plus settling maker credits can exceed practical gas. The exploit is economically constrained because live orders may be filled, but it targets the sponsor-identified order-flooding property and bypasses the existing controls' intended cap on expensive-to-clear book state. It should remain Medium.
- Code Evidence: `CLOB._matchIncomingBid` / `_matchIncomingAsk` process same-price orders one by one. `TransientMakerData` records distinct makers, and `AccountManager.settleIncomingOrder` loops over every `MakerCredit`; no per-price order cap or max maker count exists.

### L-70 / `MQILBNE2yqCT59JTgYiZQ`
- Finding Title: creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `intentional-no-event-credit`
- Checklist Gates Passed: `Stage0, scope`
- Checklist Gates Failed: `by-design, H/M materiality`
- Detailed Reason: The no-event credit behavior is explicit in the function name and comments, and on-chain balances remain correct. The report only shows possible divergence for event-only indexers or frontends that choose not to query storage. That is not a broken on-chain security invariant and does not satisfy H/M validity.
- Code Evidence: `AccountManager.creditAccountNoEvent` is marked market-only and calls `_creditAccountNoEvent`, which intentionally mutates balances without emitting `AccountCredited`. `CLOB._removeNonCompetitiveOrder` and `AccountManager.settleIncomingOrder` use the no-event path for maker credits/refunds.

### M-72 / `2F-o7dt6Wwd3vQcFKEZQf`
- Finding Title: Manager setting updates can admit zero-quote orders that brick one side of a CLOB market
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-zero-quote-config`
- Checklist Gates Passed: `Stage0, bug-exists, scope`
- Checklist Gates Failed: `trusted-role, configuration-only`
- Detailed Reason: The missing settings invariant is real, but the harmful state is created only after a trusted manager role changes tick size or minimum order parameters into an unsafe combination. The benchmark docs define those setter roles as trusted for market configuration, and this is not a case where permissionless users first obstruct a safe admin update. Under the delegated-authority rules, this is configuration-only and not a valid H/M finding.
- Code Evidence: `CLOBManager._assertValidSettings` checks the nonzero-notional invariant only at market creation. Later `CLOBManager.setTickSizes` / `setMinLimitOrderAmounts` call CLOB setters, and `BookLib.setTickSize` / `setMinLimitOrderAmountInBase` do not re-check the product, but those calls are restricted to trusted roles.
