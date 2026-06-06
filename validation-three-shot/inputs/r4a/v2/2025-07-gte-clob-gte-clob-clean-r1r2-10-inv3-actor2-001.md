# 2025-07-gte-clob Round 4a V12 Sweep Input

Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/2025-07-gte-clob-gte-clob-clean-r1r2-10-inv3-actor2-001.md`
Candidate count: `8`

This file contains only post-R4 submission candidates. R4a must decide whether each candidate is
materially distinct from the configured V12 / prior-findings context.

## Candidates

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
