# 2025-07-gte-clob Three-Shot Validation gte-clob-clean-r1r2-10-inv3-actor2-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2025-07-gte-clob/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`
Stage 1 scope screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/scope-screens/v2/2025-07-gte-clob-gte-clob-clean-r1r2-10-inv3-actor2-001.md`
Stage 2 unsupported token screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/token-screens/v2/2025-07-gte-clob-gte-clob-clean-r1r2-10-inv3-actor2-001.md`
Stage 3 final validation run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/stage3-runs/v2/2025-07-gte-clob-gte-clob-clean-r1r2-10-inv3-actor2-001.md`

## Assembly Summary

- Excluded at stage 1 (scope / known issue): `5`
- Excluded at stage 2 (unsupported token): `6`
- Fully validated at stage 3: `62`

## Per-Finding Validation

### M-1 / `U2Hhi3vbiZqDpjxJh6hmv`
- Finding Title: Broken same-price order links let a maker cancel the tail order and brick the best price level
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The report proves a live in-scope CLOB invariant break: appended same-price orders are persisted before their predecessor pointer is assigned, so a permissionless maker can cancel its own tail and leave a nonempty limit with zero head/tail pointers. This can block matching at a best price until another actor repairs state by cancellation, which is a material market-liveness DoS but not a High-severity theft or permanent fund lock.
- Code Evidence: `contracts/clob/types/Book.sol` stores `self.orders[order.id] = order` in `_updateBookPostOrder` before `_updateLimitPostOrder` mutates only the memory `order.prevOrderId`; `_updateLimitRemoveOrder` then uses zero prev/next to clear limit pointers. `contracts/clob/CLOB.sol` matching reads `ds.orders[limit.headOrder]` and reverts through `ZeroCostTrade` when the corrupted head is order zero.

### M-2 / `ntTDyuzQFbVoG65j59b0Y`
- Finding Title: Tail cancellation corrupts same-price order queue and bricks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The same-price tail-cancel path is reachable through ordinary post/cancel actions and is not blocked by operator checks because the attacker owns the canceled order. The result is a stranded real order and an indexed price that matching cannot consume, which is a material CLOB liveness failure, while recovery by the stranded maker keeps it below High.
- Code Evidence: `contracts/clob/types/Book.sol` writes the order before persisting `prevOrderId`, then `_updateLimitRemoveOrder` treats the stored tail as the head. `contracts/clob/CLOB.sol` `_matchIncomingBid` and `_matchIncomingAsk` start from the corrupted limit head and the fill processors revert when no nonzero trade occurs.

### M-3 / `OWYDJ_ICDupdBAfo1R4VA`
- Finding Title: Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Expired best asks are removed inline during matching with no per-call cap or standalone cleanup route, so an attacker can accumulate an expired prefix that every crossing buy must clear in one transaction. If cleanup exceeds gas, the transaction reverts and cleanup is rolled back, creating a credible permissionless market-side DoS under the sponsor's order-flooding concern.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingBid` loops while the best ask crosses and calls `_removeExpiredAsk` before any amount is filled; `_removeExpiredAsk` removes the order and records a refund. `_settleIncomingOrder` then forwards all transient maker credits to `AccountManager.settleIncomingOrder`, which loops over them.

### M-4 / `qeqdYm-KXE7yOpIWJSlb9`
- Finding Title: Expired top-of-book orders require unbounded cleanup before any matching can proceed
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The bug exists on both sides of the book: matching removes expired head orders synchronously and keeps looping until a live order or empty book is reached. The safeguards named in the docs, including `maxLimitsPerTx` and price-level caps, do not bound accumulated same-price expired orders across transactions, so the finding reaches Medium liveness impact.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingBid` and `_matchIncomingAsk` call `_removeExpiredAsk`/`_removeExpiredBid` inside unbounded while loops. `contracts/clob/types/Book.sol` only enforces per-order minimum amount, lot size, and per-transaction limit placement; it has no per-price FIFO length cap.

### M-5 / `Fj0qTYNYHzzhvXQcrR0IJ`
- Finding Title: Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The limit-order processors use a bitwise intersection as a zero-cost predicate, so positive base and quote deltas with disjoint bits revert even though the trade is economically valid. Because this can be induced by normal resting liquidity and blocks the marketable limit-order path, it is a Medium liveness bug rather than an unsupported-token or documentation issue.
- Code Evidence: `contracts/clob/CLOB.sol` `_processLimitBidOrder` checks `baseTokenAmountReceived & quoteTokenAmountSent == 0`, and `_processLimitAskOrder` mirrors this with sent/received amounts. The fill-order processors use explicit zero checks, showing the erroneous predicate is specific to crossing limit orders.

### L-6 / `cOtLAQEI7uTkuEyPPoAZh`
- Finding Title: Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `post-only-stale-expiry`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, TrustedRoles`
- Checklist Gates Failed: `ImpactH/M`
- Detailed Reason: The stale top-of-book check is real, but this finding only shows post-only order rejection before lazy expiry cleanup. It does not prove direct asset loss or a durable core workflow DoS on its own because non-post-only matching can clear expired orders, so the severity is Low/QA.
- Code Evidence: `contracts/clob/CLOB.sol` `_executeBidLimitOrder` and `_executeAskLimitOrder` perform the `POST_ONLY` crossing check against `getBestAskPrice`/`getBestBidPrice` before calling `_matchIncomingBid` or `_matchIncomingAsk`, where expired orders are actually removed.

### M-7 / `_aisuPcW4RFZC8k1ecQz0`
- Finding Title: Live lot-size updates can make existing top-of-book orders unmatchable
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `configuration-nonissue`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends only on trusted manager-only lot-size configuration changing live order assumptions.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`.

### M-8 / `SNePL7SCfZJ5uSjAhF0yD`
- Finding Title: Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The claimed corruption follows directly from the production linked-list update order and is exploitable by a maker that posts behind an existing order at the same price and cancels its own tail. This can make best-price liquidity unreachable by matching, giving a material but recoverable trading DoS.
- Code Evidence: `contracts/clob/types/Book.sol` persists appended orders before their `prevOrderId` is set and later removes orders based on stored prev/next links. `contracts/clob/CLOB.sol` cancel calls `ds.removeOrderFromBook`, and matching later starts from the corrupted `limit.headOrder`.

### M-9 / `3QpPKCdYzCkBLpAcsNNz6`
- Finding Title: Appended order cancellation corrupts a price level and makes older resting orders unmatchable
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: This finding independently proves the same live order-link invariant failure. The attacker uses normal maker permissions only, and the older resting order becomes unreachable while the price remains indexed, which is a Medium order-book liveness impact.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateBookPostOrder`, `_updateLimitPostOrder`, and `_updateLimitRemoveOrder` produce the zero-prev tail removal state. `contracts/clob/CLOB.sol` `_processFillBidOrder`/`_processFillAskOrder` revert when the corrupted best price yields no nonzero fill.

### M-10 / `ulpg6Or9uGZvpzvldYeKZ`
- Finding Title: Tail cancellation corrupts same-price FIFO queue and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The report's tail-cancel sequence is feasible in current scoped code and does not depend on any privileged role, unsupported asset behavior, or report-local duplication. It blocks matching through a populated price level, so Medium is appropriate.
- Code Evidence: `contracts/clob/types/Book.sol` stores `self.orders[order.id]` before setting the predecessor link and deletes/removes based on stale stored links. `contracts/clob/CLOB.sol` cancel and match functions use those book helpers without a linked-list integrity repair.

### M-11 / `Xst74MzxPAZmmiNOcO__7`
- Finding Title: Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The same-price FIFO corruption is a current code bug with a direct permissionless path. It can brick best-price matching and strand other makers' orders until cancellation, which is material liveness harm but not High-severity value extraction.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateLimitPostOrder` only updates the memory `order.prevOrderId`; `_updateLimitRemoveOrder` then clears head/tail from zero prev/next. `contracts/clob/CLOB.sol` matching cannot reach the stranded order.

### M-12 / `cXOP9qEWAQE0Z6MJUC2lz`
- Finding Title: Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding's root cause and exploit path are supported by the code: stored appended orders have no persisted predecessor, and canceling such an order corrupts limit pointers. The impact is a cheap permissionless DoS of matching at that price, enough for Medium.
- Code Evidence: `contracts/clob/types/Book.sol` lines around add/remove order update the tree, limit, and order mappings without persisting the appended predecessor. `contracts/clob/CLOB.sol` uses those book pointers in `_matchIncomingBid` and `_matchIncomingAsk`.

### M-13 / `NQf3g55GIedsx5L-68UxR`
- Finding Title: Expired order cleanup can be inflated to brick matching at the top of book
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The expired-order cleanup and maker-credit settlement work both scale with attacker-created book state and are not paginated. Since ordinary makers can create the expired queue and later fills must process it before reaching live liquidity, this is a real Medium gas DoS.
- Code Evidence: `contracts/clob/CLOB.sol` removes expired best orders in `_matchIncomingBid`/`_matchIncomingAsk`, records refunds through `TransientMakerData`, and calls `_settleIncomingOrder`. `contracts/account-manager/AccountManager.sol` loops over every `params.makerCredits` entry.

### M-14 / `KhAD_M0uCSUU4P5Rw577p`
- Finding Title: Expired order flood causes unbounded CLOB matching gas and can brick router fill routes
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The core expired-order flood is valid, and applying it to router fills is also valid because router CLOB hops call the same `postFillOrder` matching path. A permissionless expired prefix can make direct and routed fills exceed gas with cleanup rollback, producing material liveness impact.
- Code Evidence: `contracts/router/GTERouter.sol` `_executeClobPostFillOrder` builds a `FILL_OR_KILL` fill and calls `ICLOB.postFillOrder`; `contracts/clob/CLOB.sol` then performs unbounded expired-head cleanup in the matching loops.

### M-15 / `LOsUmoQXuZsyjF_Gi48ug`
- Finding Title: Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `amend-expiry-not-persisted`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, TrustedRoles`
- Checklist Gates Failed: `ImpactH/M`
- Detailed Reason: The same-price amend path really validates and emits the requested timestamp without persisting it, but the demonstrated harm is mainly maker self-impact, missed execution, and misleading off-chain state. Expiry refunds the order rather than stealing funds or causing a broad liveness failure, so it is Low/QA.
- Code Evidence: `contracts/clob/CLOB.sol` `_processAmend` validates `args.cancelTimestamp` and emits `OrderAmended`, while `_executeAmendAmount` updates only amount/open interest and never writes `order.cancelTimestamp`.

### M-16 / `-bEdt8ojcXBsHOvXu74wP`
- Finding Title: Fee-on-transfer deposits overcredit AccountManager balances and can make withdrawals insolvent
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `unsupported-token`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior.
- Code Evidence: Excluded during the stage 2 unsupported-token screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`, and the stage 1 scope screen.

### M-17 / `UeiY1mitQGuD0qzyGErYn`
- Finding Title: Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The appended-order cancellation path is a live linked-list corruption bug and can be executed by any maker against a shared price level. It produces a best-price matching DoS, satisfying Medium impact.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateBookPostOrder` and `_updateLimitPostOrder` leave stored appended orders with `prevOrderId == 0`; `_updateLimitRemoveOrder` then clears limit pointers from that stale state. `contracts/clob/CLOB.sol` matching consumes the corrupted head pointer.

### H-18 / `6Uh2u3bVS4waCHjG-fjsj`
- Finding Title: Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding is valid, but the demonstrated impact is a recoverable trading DoS at a corrupted price level rather than direct theft, systemic insolvency, or permanent fund lock. It should be kept as Medium despite the report's High label.
- Code Evidence: `contracts/clob/types/Book.sol` persists appended orders before setting `prevOrderId` and removes based on stale links. `contracts/clob/CLOB.sol` fill processors revert `ZeroCostTrade` when matching at the corrupted limit yields zero traded amounts.

### M-19 / `m2sqyBVvdOeZDLv7wxPl1`
- Finding Title: Unbounded expired-order cleanup lets makers gas-DoS fills at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Expired best-price orders are lazily removed only during matching and can accumulate across normal maker transactions. Because a failed fill reverts all cleanup, this creates a practical and material gas DoS for takers without relying on trusted-role abuse.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingBid` and `_matchIncomingAsk` remove expired orders inline and continue without reducing incoming amount; there is no bounded cleanup function in the in-scope CLOB or Book code.

### M-20 / `NipRYiPzXhZll_OuCOzOd`
- Finding Title: Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Router CLOB fills inherit the same unbounded expired-order cleanup failure as direct fills. Since `executeRoute` uses CLOB `FILL_OR_KILL` hops and has no cleanup cap, expired top-of-book queues can materially brick routed trading.
- Code Evidence: `contracts/router/GTERouter.sol` `_executeClobPostFillOrder` calls `ICLOB(market).postFillOrder`; `contracts/clob/CLOB.sol` matching loops perform unbounded expired-order removals before reaching live liquidity.

### M-21 / `AB3Otlvf037zOTvgUcwhh`
- Finding Title: Same-price order cancellation corrupts BookLib limits and blocks matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The BookLib limit corruption is proven by the storage/memory ordering and is reachable by a maker canceling its own appended order. The resulting unmatchable best price is a material order-book liveness issue.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateBookPostOrder` stores the order, `_updateLimitPostOrder` fails to persist the new predecessor, and `_updateLimitRemoveOrder` updates head/tail using stale zero links. `contracts/clob/CLOB.sol` matching then reads order zero.

### M-22 / `Y8bBwzzHDvU11PZtarwlE`
- Finding Title: Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: This is a real same-price queue corruption with a normal permissionless path. The corrupted limit can remain indexed with no reachable head, blocking matching through that price and meeting Medium liveness impact.
- Code Evidence: `contracts/clob/types/Book.sol` add/remove helpers do not preserve the appended order's predecessor in storage. `contracts/clob/CLOB.sol` `cancel` invokes that removal path and fill processing later fails on the corrupted limit.

### M-23 / `9DMzwZBvN1i2QQTvRs_0Q`
- Finding Title: Zero-quote dust orders can permanently block one side of a CLOB market
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `zero-quote-dust`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The code can leave a sub-minimum resting remainder whose quote value floors to zero, and later matching can compute nonzero base with zero quote, revert `ZeroCostTrade`, and roll back removal. This is materially distinct from the public fill-rounding note because it leaves a persistent best-price blocker rather than harmless dust.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingOrder` reduces `makerOrder.amount` on partial fills and computes quote with `getQuoteTokenAmount`; fill processors then revert if either aggregate leg is zero. `contracts/clob/types/Book.sol` floors quote amount as `baseAmount * price / baseSize`.

### M-24 / `2Cf1SI_P2vh8pQZjuiEjt`
- Finding Title: Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The report accurately traces a live storage-link bug in scoped BookLib code. An appended order can be canceled by its owner and corrupt a populated limit, blocking fills through that price, so the finding is valid Medium.
- Code Evidence: `contracts/clob/types/Book.sol` writes orders before setting the predecessor and later updates limit head/tail from stale links. `contracts/clob/CLOB.sol` matching at a corrupted best price produces zero fill and reverts through the fill processor.

### M-25 / `RdhlzdOHuMq96aCAhrX5A`
- Finding Title: Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `price-level-cap-eviction`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The book-full eviction removes only one order at the worst price, so a multi-order worst limit remains in the tree and the next inserted price pushes the side beyond `maxNumLimitsPerSide`. This directly undermines the sponsor-highlighted anti-flooding cap and is permissionless, making Medium appropriate.
- Code Evidence: `contracts/clob/CLOB.sol` `_executeBidLimitOrder` and `_executeAskLimitOrder` check `tree.size() == maxNumLimitsPerSide` and call `_removeNonCompetitiveOrder` on only the worst tail order before inserting the new order. `contracts/clob/types/Book.sol` removes the tree node only when `limit.numOrders == 1`.

### M-26 / `7oFfTRZTnC8m9HiDpKPa1`
- Finding Title: Lot-size increases can strand existing orders below the new lot and block matching
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `configuration-nonissue`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends only on trusted manager-only lot-size configuration changing live order assumptions.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`.

### M-27 / `WSYqCqHZyebvGG35AyqJv`
- Finding Title: Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `price-level-cap-eviction`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The pruning logic does not guarantee a price level is removed before inserting a new more-competitive price. Once the side exceeds the cap, the equality guard stops pruning, so this is a real bypass of an in-scope order-flooding safeguard.
- Code Evidence: `contracts/clob/CLOB.sol` uses `if (ds.askTree.size() == maxNumLimitsPerSide)` and removes `ds.askLimits[maxAskPrice].tailOrder`; `contracts/clob/types/Book.sol` keeps the tree price when multiple orders remain at that limit.

### L-28 / `zEdopivUepNxTT_LKsrN7`
- Finding Title: Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `amend-expiry-not-persisted`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, TrustedRoles`
- Checklist Gates Failed: `ImpactH/M`
- Detailed Reason: The stale timestamp bug exists, but the finding's practical harm is early refund/missed maker execution and event-state mismatch. Without direct fund loss or broader market liveness failure, this remains Low/QA.
- Code Evidence: `contracts/clob/CLOB.sol` `_processAmend` accepts the new timestamp and `_executeAmendAmount` only updates `order.amount`; the matching loops later call `isExpired` on the unchanged stored timestamp.

### H-29 / `fc-HXKUWbDnoz-SIy46sV`
- Finding Title: Same-price order append corrupts CLOB queue and can brick matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The queue corruption is real and permissionless, but the impact is best-price liveness degradation and stranded orders rather than direct theft or permanent systemic lock. The finding should remain valid but assessed as Medium.
- Code Evidence: `contracts/clob/types/Book.sol` appends by updating the old tail's `nextOrderId` while failing to persist the new tail's `prevOrderId`. `contracts/clob/CLOB.sol` matching reads the zero head after cancellation and fill processing reverts.

### M-30 / `EANPmE7_tnXWUY7__k2Xm`
- Finding Title: Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-transient-guard`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, ImpactMedium`
- Checklist Gates Failed: `-`
- Detailed Reason: The in-scope router inherits Solady's transient guard without overriding the mainnet-only mode, so guarded router functions revert before their bodies on `chainid != 1` from the uninitialized storage fallback. The mandatory docs do not confine deployment to Ethereum mainnet, so this remains a current code-level router availability risk, with Medium confidence because deployment-chain expectations are not explicit.
- Code Evidence: `contracts/router/GTERouter.sol` applies `nonReentrant` to `executeRoute`, `launchpadBuy`, and `launchpadSell` and never initializes or overrides the guard. `lib/solady/src/utils/ReentrancyGuardTransient.sol` defaults `_useTransientReentrancyGuardOnlyOnMainnet()` to true and reverts on non-mainnet when `sload(slot)` is zero.

### M-31 / `-uTFZlqtkstf85f3lJ1Ox`
- Finding Title: Expired best-price order queues create unbounded matching work and can gas-brick market progress
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The expired best-price queue can be grown through normal order placement and forces later matching to perform all cleanup in one transaction. No configured safeguard bounds cumulative same-price expired order count, so this is a valid Medium DoS.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingBid`/`_matchIncomingAsk` call expiry removal inside unbounded loops; `contracts/clob/types/Book.sol` enforces per-transaction placement limits but no lifetime queue bound.

### M-32 / `0tg5m2yofCOr3Vy6kSVNm`
- Finding Title: Live lot-size changes can make existing top-of-book orders unmatchable
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `configuration-nonissue`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends only on trusted manager-only lot-size configuration changing live order assumptions.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`.

### M-33 / `WJeKSESLnz563ldIydVzn`
- Finding Title: Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding's specific same-price tail-cancel corruption exists in current BookLib code and reaches a practical matching DoS. It requires only normal maker actions and is not excluded by known public issues.
- Code Evidence: `contracts/clob/types/Book.sol` persists the appended order with zero predecessor and clears limit pointers on removal. `contracts/clob/CLOB.sol` fill matching starts at those pointers and cannot reach the stranded live order.

### M-34 / `rdcQt1XKFsNqvAV7390mj`
- Finding Title: Nominal router deposits over-credit accounts for fee-on-transfer market tokens
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `unsupported-token`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior.
- Code Evidence: Excluded during the stage 2 unsupported-token screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`, and the stage 1 scope screen.

### M-35 / `XKtuQBa7uy7q6duOwgi1U`
- Finding Title: getNextOrders skips lower bid levels by always walking to the next bigger price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `bid-pagination-direction`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, TrustedRoles`
- Checklist Gates Failed: `ImpactH/M`
- Detailed Reason: `getNextOrders` is wrong for bid-side price priority, but it is a view/enumeration bug. Without evidence that on-chain settlement or asset movement depends on this view, the impact is off-chain visibility and integration correctness, so it is Low/QA rather than H/M.
- Code Evidence: `contracts/clob/types/Book.sol` `getNextOrders` always calls `getNextBiggestPrice` after the end of a price-level list, while `getOrdersPaginated` correctly branches to `getNextSmallestPrice` for BUY orders.

### L-36 / `VTB6di026hozl6UG7W92S`
- Finding Title: Per-fill fee flooring lets traders split orders to avoid maker and taker fees
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `fee-rounding`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, TrustedRoles`
- Checklist Gates Failed: `ImpactH/M`
- Detailed Reason: Fee flooring per fill is mechanically real, but the report does not prove material H/M loss under the configured market parameters rather than ordinary integer rounding and fee/min-order calibration. It is best treated as Low/QA fee-accounting hardening.
- Code Evidence: `contracts/clob/types/FeeData.sol` computes taker and maker fees with `amount.fullMulDiv(feeRate, FEE_SCALING)`, which floors each call. `contracts/account-manager/AccountManager.sol` applies maker fees per `MakerCredit` inside the settlement loop.

### M-37 / `9E90WTZyPojpmbspEf0t0`
- Finding Title: Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The router claim is supported because routed CLOB fills use the same direct CLOB matching function and inherit its unbounded expired-order cleanup. A permissionless expired queue can make router routes revert or exceed gas before reaching live liquidity.
- Code Evidence: `contracts/router/GTERouter.sol` `_executeClobPostFillOrder` calls `postFillOrder`; `contracts/clob/CLOB.sol` `_matchIncomingBid`/`_matchIncomingAsk` remove expired orders inline and rollback on revert.

### M-38 / `hRZ--CyWJFFnL-AMLE9Fb`
- Finding Title: Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The price-level pointer corruption exists and can be induced by canceling an attacker's own appended order. This materially blocks matching through the corrupted price level and is valid Medium.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateLimitPostOrder` does not persist the new tail's predecessor, and `_updateLimitRemoveOrder` uses the zero predecessor to clear head/tail. `contracts/clob/CLOB.sol` consumes those pointers during matching.

### M-39 / `153JZItTsyyl20QBa0ugC`
- Finding Title: Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The report describes a real, permissionless linked-list corruption in the order book. It blocks crossing orders at the affected price until state is repaired, which is material liveness harm and not merely a duplicate screen in this raw validation round.
- Code Evidence: `contracts/clob/types/Book.sol` add/remove order logic leaves appended orders with stale zero prev links. `contracts/clob/CLOB.sol` `_matchIncomingBid`/`_matchIncomingAsk` load order zero from the corrupted limit head and fail to trade.

### M-40 / `nqPYgqKnbPRtrs3G-Y_1c`
- Finding Title: Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Canceling a stored appended order with zero predecessor can leave a nonempty, indexed limit with no valid head/tail. This is a concrete CLOB fill DoS reachable through public calls and warrants Medium.
- Code Evidence: `contracts/clob/types/Book.sol` stores orders before assigning appended `prevOrderId` and removes based on stored prev/next. `contracts/clob/CLOB.sol` cancel calls that removal and matching later fails at the corrupted price.

### M-41 / `_6-2VSOzJzLq5VgATL8RN`
- Finding Title: Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding independently demonstrates a valid same-price queue corruption with ordinary maker actions. The resulting unmatchable price level is a material but recoverable DoS.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateBookPostOrder`, `_updateLimitPostOrder`, and `_updateLimitRemoveOrder` create and expose the stale-link state. `contracts/clob/CLOB.sol` matching and fill zero checks turn that state into a failed fill.

### M-42 / `NmnsJOJu7S9-cuJppTnev`
- Finding Title: Nominal router deposits overcredit AccountManager for fee-on-transfer market tokens
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `unsupported-token`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior.
- Code Evidence: Excluded during the stage 2 unsupported-token screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`, and the stage 1 scope screen.

### M-43 / `MsApEnZhb8xJbreCI0cFX`
- Finding Title: Authorized tick-size update lets zero-quote asks permanently block buy-side matching
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `configuration-nonissue`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Harm depends on a trusted tick-size setter choosing an unsafe market parameter.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`.

### M-44 / `UQjLUNPmihJvNMUkkW81r`
- Finding Title: Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The attack uses a normal maker post/cancel sequence and exploits a real memory-vs-storage link bug. It can block best-price matching, giving Medium liveness impact.
- Code Evidence: `contracts/clob/types/Book.sol` only updates the prior tail's `nextOrderId` in storage and not the appended order's `prevOrderId`. `contracts/clob/CLOB.sol` relies on the corrupted `Limit` head/tail for matching.

### M-45 / `ejeE9u68HPOend0p-7kCj`
- Finding Title: Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-transient-guard`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, ImpactMedium`
- Checklist Gates Failed: `-`
- Detailed Reason: The guarded router flows revert before executing on non-mainnet chain IDs because the inherited storage fallback expects a nonzero sentinel that GTERouter never initializes. This is a material router availability failure on such deployments; confidence is Medium because the benchmark docs do not state a single deployment chain.
- Code Evidence: `contracts/router/GTERouter.sol` uses `nonReentrant` on router route and launchpad entrypoints. `lib/solady/src/utils/ReentrancyGuardTransient.sol` branches to the storage fallback for `chainid != 1` and reverts if the slot is zero.

### H-46 / `luMc8bQJZFJe5uuP0Lnl5`
- Finding Title: Canceling an appended order corrupts the FIFO list and bricks matching at that price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The FIFO corruption is a valid production bug, but the supported impact is a recoverable matching DoS rather than High-severity theft or permanent lock. It should be preserved as Medium.
- Code Evidence: `contracts/clob/types/Book.sol` writes the appended order before setting the predecessor and then removes based on stale zero links. `contracts/clob/CLOB.sol` matching reads the null head and cannot progress through the corrupted price.

### M-47 / `C4opETU5jlu8R9w5AyDv0`
- Finding Title: GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-transient-guard`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, ImpactMedium`
- Checklist Gates Failed: `-`
- Detailed Reason: The code-level failure is present: a fresh GTERouter on `chainid != 1` cannot enter its guarded functions because the inherited guard treats zero storage as reentered. With no mandatory docs limiting the router to Ethereum mainnet, this remains a Medium availability risk, though chain-deployment assumptions keep confidence at Medium.
- Code Evidence: `contracts/router/GTERouter.sol` does not override `_useTransientReentrancyGuardOnlyOnMainnet` or initialize the guard slot. `lib/solady/src/utils/ReentrancyGuardTransient.sol` defaults that hook to true and uses the reverting storage fallback on non-mainnet.

### M-48 / `vWmsLvCofS-GL6ZvlABOI`
- Finding Title: Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The populated-limit invariant is broken by normal same-price append and cancel operations. When the affected price is best bid or ask, matching is blocked, making this a valid Medium DoS.
- Code Evidence: `contracts/clob/types/Book.sol` leaves appended order predecessors unset in storage and later clears limit pointers on removal. `contracts/clob/CLOB.sol` fill matching uses those pointers and reverts if no nonzero trade occurs.

### M-49 / `dknnew5I8_c7ttT34fiMV`
- Finding Title: Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Expired queue cleanup and settlement are both attacker-state-dependent and unbounded in one fill. Since failed gas execution reverts cleanup, the queue can persistently block fills, satisfying Medium.
- Code Evidence: `contracts/clob/CLOB.sol` loops through expired head orders and turns removals into transient maker credits; `contracts/account-manager/AccountManager.sol` iterates all maker credits during settlement.

### M-50 / `zGQXAkrWqG1uci8hIrcI-`
- Finding Title: DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: BookLib's append order stores the new order too early and exposes a direct same-price tail-cancel DoS. The result is a corrupted, indexed limit that matching cannot traverse, which is Medium liveness harm.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateBookPostOrder` stores the new order and `_updateLimitPostOrder` mutates only the memory predecessor. `contracts/clob/CLOB.sol` cancel and matching paths use the corrupted stored links.

### M-51 / `AvI34sCVIJRryIjqtw_cN`
- Finding Title: Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The queue corruption is live in scoped code, permissionless, and capable of blocking best-price fills. It remains valid in raw validation regardless of same-family neighbors.
- Code Evidence: `contracts/clob/types/Book.sol` add/remove order helpers fail to maintain appended prev links. `contracts/clob/CLOB.sol` `_executeCancel` removes attacker orders and the matching loops later consume the corrupted head.

### M-52 / `S7in-uafMZq650UhBgUK0`
- Finding Title: GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `router-transient-guard`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, ImpactMedium`
- Checklist Gates Failed: `-`
- Detailed Reason: The router's guarded functions are bricked on non-mainnet chain IDs by the inherited guard's uninitialized storage fallback. The issue is a real code path and affects core route availability where such deployments are expected; confidence is Medium because expected deployment chain is not explicit in the benchmark docs.
- Code Evidence: `contracts/router/GTERouter.sol` `executeRoute`, `launchpadBuy`, and `launchpadSell` use `nonReentrant`; `lib/solady/src/utils/ReentrancyGuardTransient.sol` reverts on non-mainnet when `sload(_REENTRANCY_GUARD_SLOT)` is zero.

### M-53 / `jiTlrD6mS8stMUieQXaAV`
- Finding Title: Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The expired-order gas DoS applies directly to `executeRoute` because a CLOB hop calls the same unbounded fill path. Permissionless expired queues can therefore brick a routed market side and rollback cleanup on failure.
- Code Evidence: `contracts/router/GTERouter.sol` `_executeAllHops` dispatches CLOB hops to `_executeClobPostFillOrder`, which calls CLOB `postFillOrder`. `contracts/clob/CLOB.sol` handles expired orders inside unbounded matching loops.

### M-54 / `CI1pMro9PUu-pjukfJxxj`
- Finding Title: Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The bitwise predicate can reject a valid crossing limit order where both traded legs are positive. A maker can choose amount/price pairs that trigger this, blocking a core order path, so the finding is valid Medium.
- Code Evidence: `contracts/clob/CLOB.sol` `_processLimitBidOrder` and `_processLimitAskOrder` use `&` in the `ZeroCostTrade` guard after matching. The fill path uses logical zero checks and can execute the same economic liquidity.

### M-55 / `jUMknNEuqkC60UN4wp2td`
- Finding Title: BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding identifies the correct root cause: appended orders are stored without a persisted predecessor. Canceling or removing that order corrupts a nonempty price level, causing a practical matching DoS.
- Code Evidence: `contracts/clob/types/Book.sol` `_updateBookPostOrder` stores the order before `_updateLimitPostOrder` assigns `order.prevOrderId` in memory only. `contracts/clob/CLOB.sol` later relies on those links in cancel and matching flows.

### M-56 / `862phQLIcBWjMjtUglV8a`
- Finding Title: Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Positive base and quote amounts can have no common set bits, and the current limit-order guard treats that as zero cost. This blocks normal marketable limit orders against selected liquidity, giving Medium liveness impact.
- Code Evidence: `contracts/clob/CLOB.sol` `_processLimitBidOrder` and `_processLimitAskOrder` both contain the bitwise zero-cost predicate immediately after `_executeBidLimitOrder`/`_executeAskLimitOrder` return matched deltas.

### L-57 / `2hWPwrRLWp-9Cy5L-441x`
- Finding Title: Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Although submitted as Low, the bug can block valid marketable limit-order execution with positive value on both legs. Because the CLOB's limit-order route is a core user-facing trading path, the impact reaches Medium.
- Code Evidence: `contracts/clob/CLOB.sol` checks `(base & quote) == 0` rather than `base == 0 || quote == 0` in both limit processors. Matching computes valid deltas before the erroneous revert rolls the transaction back.

### M-58 / `3r67ZUXco89RLvFbEmeiB`
- Finding Title: Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-prev-pointer`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The shared-price tail cancellation path corrupts the linked list in current code and can be triggered by a normal maker. It blocks matching through the affected price, so it is valid Medium.
- Code Evidence: `contracts/clob/types/Book.sol` fails to persist the appended predecessor and clears limit pointers on removal. `contracts/clob/CLOB.sol` `postFillOrder` then cannot progress through the corrupted best price.

### M-59 / `8w065WL0FE9FUGvl4vSXj`
- Finding Title: Unlimited orders per price level can make CLOB matching and settlement exceed block gas
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-order-count-gas`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The code does not cap FIFO depth at one price, and matching plus maker-credit settlement scale with the number of attacker-created orders. This is a credible order-flooding liveness issue under the sponsor's stated concerns, though confidence is Medium because attackers must lock valid minimum-sized liquidity and takers can sometimes split execution.
- Code Evidence: `contracts/clob/CLOB.sol` matching loops consume one maker order at a time and `_settleIncomingOrder` exports all transient maker credits. `contracts/account-manager/AccountManager.sol` loops over every maker credit, while `contracts/clob/types/Book.sol` lacks a per-price order-count cap.

### M-60 / `lIFEP3RQX99oSBh6CEhyT`
- Finding Title: Unbounded expired order clearing can brick CLOB matching at the top of book
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Expired top-of-book cleanup is unbounded and rollback-prone, so a permissionless maker can create a persistent gas barrier before live liquidity. This is a material CLOB liveness failure.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingBid`/`_matchIncomingAsk` repeatedly remove expired head orders before matching. No in-scope function provides bounded third-party cleanup.

### M-61 / `8efy2NB9lB-h6l5RBXVFp`
- Finding Title: Unbounded expired top-of-book cleanup can make matching exceed gas and block fills
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The described exploit path is practical: valid orders can expire at the head, later fills must clear them all, and out-of-gas reverts leave them in place. This is valid Medium.
- Code Evidence: `contracts/clob/CLOB.sol` expiry removal is inline in the matching loops; `contracts/clob/types/Book.sol` only restricts per-order and per-transaction placement, not cumulative expired queue length.

### M-62 / `wG0yKv8bfAxS8cfQ-jBgy`
- Finding Title: Bitwise zero-cost check rejects valid nonzero CLOB limit fills
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bitwise-zero-cost`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The zero-cost predicate is a real arithmetic bug in the limit-order path and can reject positive nonzero fills. It blocks core trading for reachable price/amount pairs and is not excluded by scope or unsupported token behavior.
- Code Evidence: `contracts/clob/CLOB.sol` `_processLimitBidOrder` and `_processLimitAskOrder` use the bitwise `&` condition after matching. `contracts/clob/types/Book.sol` amount conversion can naturally produce positive disjoint base/quote values.

### M-63 / `rOuvmt_HeIakChKQ1M8uP`
- Finding Title: Unbounded maker credit settlement lets large fills exceed gas and DoS order matching
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-order-count-gas`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Settlement work scales with distinct makers accumulated in transient maker data, and there is no settlement pagination or maker-count cap. This can make large fills through many small maker orders exceed gas, so it is a credible Medium order-flooding issue, with Medium confidence due to the attacker's liquidity and gas costs.
- Code Evidence: `contracts/clob/types/TransientMakerData.sol` records one maker entry per distinct credited maker and returns the full array. `contracts/account-manager/AccountManager.sol` `settleIncomingOrder` loops over `params.makerCredits` and credits/calculates fees for each.

### M-64 / `-0eZ9DmvhCPURywmjaWdQ`
- Finding Title: Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding combines the valid direct-fill expired cleanup DoS with the router path that delegates to direct CLOB fills. Both are in scope and no safeguard bounds expired queue cleanup.
- Code Evidence: `contracts/clob/CLOB.sol` removes expired head orders in unbounded matching loops; `contracts/router/GTERouter.sol` CLOB_FILL route execution calls `postFillOrder` with no cleanup limit.

### M-65 / `7NpX1q1QAy0pvHbmh_VvF`
- Finding Title: Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The cleanup progress is not durable when the fill reverts, including the no-live-trade `ZeroCostTrade` case. That makes expired top-of-book queues a persistent liveness blocker until enough cleanup can finish in one successful transaction or owners cancel.
- Code Evidence: `contracts/clob/CLOB.sol` removes expired orders and records transient credits before `_processFillBidOrder`/`_processFillAskOrder` may revert `ZeroCostTrade`; a revert rolls back `ds.removeOrderFromBook` and transient settlement.

### M-66 / `CvvbhB2_m67scVPlc4Knw`
- Finding Title: Expired top-of-book orders can gas-DoS CLOB fills and routed trades
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `expired-cleanup-unbounded`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: The report accurately describes unbounded expired-head cleanup with rollback on gas failure, and it covers both direct fills and router routes. The path is permissionless and materially affects market availability.
- Code Evidence: `contracts/clob/CLOB.sol` `_matchIncomingBid` and `_matchIncomingAsk` remove expired orders inside unbounded loops; `contracts/router/GTERouter.sol` routes CLOB hops through `postFillOrder`.

### M-67 / `V7oC8eZirDDRKdSOpU5_S`
- Finding Title: Unlimited same-price dust orders can make CLOB fills exceed practical gas limits
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `same-price-order-count-gas`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior, RootCause, Safeguards, Exploitability, ImpactMedium, TrustedRoles`
- Checklist Gates Failed: `-`
- Detailed Reason: Same-price FIFO depth is unbounded over time and both matching and maker settlement scale with that depth. This can degrade meaningful top-of-book fills beyond practical gas limits, which is Medium under the benchmark's order-flooding priorities, though confidence is Medium because the attacker supplies valid minimum-size liquidity.
- Code Evidence: `contracts/clob/CLOB.sol` matching consumes resting orders one by one and exports maker credits; `contracts/account-manager/AccountManager.sol` settlement iterates all credits; `contracts/clob/types/Book.sol` has no per-price order cap.

### H-68 / `oBFwqzifOb7uy-Lkb1tK-`
- Finding Title: Reentrant token can withdraw pre-existing AccountManager liquidity during deposit before funding the new credit
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `unsupported-token-semantics`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Claim requires callback or malicious ERC20 transfer semantics; docs do not explicitly support hook-enabled tokens.
- Code Evidence: Excluded during the stage 2 unsupported-token screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`, and the stage 1 scope screen.

### M-69 / `ihRh47bUNlVaErxYytyq1`
- Finding Title: Fee-on-transfer deposits credit more internal balance than AccountManager actually receives
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `unsupported-token`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Claim requires fee-on-transfer market token; docs do not explicitly support fee-on-transfer behavior.
- Code Evidence: Excluded during the stage 2 unsupported-token screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`, and the stage 1 scope screen.

### L-70 / `MQILBNE2yqCT59JTgYiZQ`
- Finding Title: creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `event-only-accounting`
- Checklist Gates Passed: `Stage0, Scope, SupportedBehavior`
- Checklist Gates Failed: `BugExists, ByDesign, ImpactH/M`
- Detailed Reason: The behavior exists but is explicitly named and commented as a no-event credit path, and on-chain balances remain correct. Event-only indexer divergence without functional asset or core workflow harm is not an H/M security bug and is contradicted by the intended `creditAccountNoEvent` interface.
- Code Evidence: `contracts/account-manager/AccountManager.sol` exposes `creditAccountNoEvent` and `_creditAccountNoEvent`, which intentionally mutate `accountTokenBalances` without emitting `AccountCredited`; `settleIncomingOrder` also uses `_creditAccountNoEvent` for maker credits.

### M-71 / `DzJ-BBAE-EMiZlNn20JUJ`
- Finding Title: Failed fee transfer permanently clears unclaimed fees in AccountManager.collectFees
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `unsupported-erc20-return`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Claim requires token that reports transfer success without moving funds; docs do not explicitly support false-success ERC20 semantics.
- Code Evidence: Excluded during the stage 2 unsupported-token screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`, and the stage 1 scope screen.

### M-72 / `2F-o7dt6Wwd3vQcFKEZQf`
- Finding Title: Manager setting updates can admit zero-quote orders that brick one side of a CLOB market
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `configuration-nonissue`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Harm depends on trusted setters choosing an unsafe tick or minimum-order configuration.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-scope.txt`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-07-gte-clob/2025-07-gte-clob-validation.md`, `/Users/apmfree/Desktop/Audit/2025-07-gte-clob-23ee9e/2025-07-gte-clob/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`.

### M-73 / `0pUyX3n9XiEdRjh5tEN-Z`
- Finding Title: Live fee-tier reads let routine tier changes alter already-resting order settlement
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `live-fee-tier`
- Checklist Gates Passed: `Stage0, BugExists, Scope, SupportedBehavior`
- Checklist Gates Failed: `TrustedRole, ImpactH/M`
- Detailed Reason: Settlement does use live fee tiers, but changing account fee tiers is an explicitly delegated capability of the trusted `FEE_TIER_SETTER`/owner path in the benchmark docs. The finding does not prove permissionless state manipulation or unauthorized incremental impact beyond that trusted role's authority, so it is not a valid H/M issue.
- Code Evidence: `contracts/account-manager/AccountManager.sol` `settleIncomingOrder` calls `feeData.getTakerFee` and `getMakerFee` using current tiers, while `setSpotAccountFeeTiers` is callable only by `CLOBManager`. `contracts/clob/CLOBManager.sol` gates `setAccountFeeTiers` with `onlyOwnerOrRoles(CLOBRoles.FEE_TIER_SETTER)`.
