# 2025-07-gte-clob Three-Shot Submission Candidates gte-clob-fnfix-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/2025-07-gte-clob-gte-clob-fnfix-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/2025-07-gte-clob-gte-clob-fnfix-001.md`

## Canonicalization Summary

- Candidate reportable findings before R4: `51`
- Kept after R4 canonicalization: `9`
- Dropped by R4 cleanup: `42`
- Dropped finding ids: `M-1, M-2, M-3, M-4, M-5, M-8, M-9, M-10, M-11, M-12, M-13, M-14, M-17, H-18, M-19, M-20, M-21, M-22, M-24, M-27, H-29, M-31, M-33, M-37, M-38, M-39, M-40, M-41, M-44, H-46, M-48, M-50, M-51, M-53, M-54, M-56, L-57, M-58, M-60, M-61, M-64, M-67`

## Root Cause Groups

- `bitwise-zero-cost`: M-5 -> M-62, M-54 -> M-62, M-56 -> M-62, L-57 -> M-62, M-62
- `expired-top-cleanup-gas`: M-3 -> M-66, M-4 -> M-66, M-14 -> M-66, M-19 -> M-66, M-20 -> M-66, M-31 -> M-66, M-37 -> M-66, M-53 -> M-66, M-60 -> M-66, M-61 -> M-66, M-64 -> M-66, M-66
- `expired-top-cleanup-revert`: M-65
- `expired-top-cleanup-settlement-gas`: M-13 -> M-49, M-49
- `maker-credit-settlement-gas`: M-63
- `max-price-level-eviction`: M-25, M-27 -> M-25
- `same-price-order-count-gas`: M-59, M-67 -> M-59
- `same-price-prev-pointer`: M-1 -> M-55, M-2 -> M-55, M-8 -> M-55, M-9 -> M-55, M-10 -> M-55, M-11 -> M-55, M-12 -> M-55, M-17 -> M-55, H-18 -> M-55, M-21 -> M-55, M-22 -> M-55, M-24 -> M-55, H-29 -> M-55, M-33 -> M-55, M-38 -> M-55, M-39 -> M-55, M-40 -> M-55, M-41 -> M-55, M-44 -> M-55, H-46 -> M-55, M-48 -> M-55, M-50 -> M-55, M-51 -> M-55, M-55, M-58 -> M-55
- `zero-quote-dust`: M-23

## R4a V12 Sweep Summary

- Input candidates before R4a: `9`
- Kept after R4a V12 sweep: `9`
- Excluded as V12 / prior-finding overlap: `0`
- Excluded finding ids: `-`
- V12 sweep screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-sweeps/v2/2025-07-gte-clob-gte-clob-fnfix-001.md`

## Submission Candidates

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
