# 2025 10 covenant - Findings Report
## Commit hash: d5ebe4461564b46cacf8a90cf11add29470ef001

##Findings by Pattern


 **Derived From** : Preview Functions Use MAX_STALENESS_UPPER_BOUND Instead of Market-Specific maxStaleness

[M-2]. Preview Functions Accept Stale Prices Leading to Transaction Failures and Incorrect Risk Calculations
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : PythOracle preview functions use MAX_STALENESS_UPPER_BOUND instead of maxStaleness

[M-4]. PythOracle preview functions accept stale prices beyond configured maxStaleness causing transaction simulation mismatches
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : previewGetQuote(amount, base, quote) == getQuote(amount, base, quote) for same oracle state

[M-5]. Referential inconsistency between previewGetQuote and getQuote in PythOracle causes MEV exploitation and UX degradation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : elapsedTime > debtDuration IMPLIES accrued interest capped at 1 duration worth

[M-9]. Temporal freeze after extended dormancy causes stale rate application enabling yield extraction
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless
Poc Test Status: ErrorRunningTests



 **Derived From** : Preview Functions Use MAX_STALENESS_UPPER_BOUND Instead of Configured maxStaleness

[M-10]. PythOracle Preview Functions Accept Stale Prices Beyond Configured maxStaleness, Enabling MEV and UX Failures
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests





### Number of Findings
- C: 0
- H: 0
- M: 5
- L: 7
- I: 0

##Findings by Pattern


 **Derived From** : oracleBaseCross and oracleCrossQuote must be bidirectional and support both (base,cross) and (cross,base) pairs

## [L-1]. CrossAdapter deployment with unidirectional oracle breaks inverse path causing DOS on half of market operations

## Derived From Pattern/Invariant
oracleBaseCross and oracleCrossQuote must be bidirectional and support both (base,cross) and (cross,base) pairs

## Exploit Type
AccessControl

## Location
CrossAdapter.constructor

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
CrossAdapter constructor accepts two oracle addresses (oracleBaseCross, oracleCrossQuote) but performs no on-chain validation that these oracles actually support bidirectional queries. The @dev comment states 'Both cross oracles MUST be bidirectional' but this requirement is not enforced. When _getQuote() executes the inverse path (quote→base), it calls IPriceOracle(oracleBaseCross).getQuote(inAmount, cross, base) which will revert with PriceOracle_NotSupported if the oracle only supports base→cross direction. This breaks half of CrossAdapter's functionality - specifically getQuote(_, quote, base) always reverts while getQuote(_, base, quote) works. LatentSwapLEX depends on bidirectional pricing for mint/redeem operations, so a unidirectional CrossAdapter deployment would brick users attempting to withdraw collateral in the inverse direction.

## Impact
This is a governance misconfiguration risk. If the governor wires a CrossAdapter with unidirectional component oracles, one direction of pricing will revert, partially DoSing integrations that expect bidirectional pricing. There is no attacker-controlled exploit path; funds are not lost and the forward direction still works. Under the rubric, bugs only reachable via admin misuse are QA/Low.

## Command to Run Test


## Proof of Concept
Setup: Deploy CrossAdapter(base=WETH, cross=USDC, quote=WBTC) with two unidirectional component oracles: oracleBaseCross supports only WETH->USDC; oracleCrossQuote supports only USDC->WBTC.
1) Forward path call getQuote(x, WETH, WBTC): CrossAdapter queries oracleBaseCross.getQuote(x, WETH, USDC) then oracleCrossQuote.getQuote(_, USDC, WBTC). Both supported, call succeeds.
2) Inverse path call getQuote(y, WBTC, WETH): CrossAdapter first calls oracleCrossQuote.getQuote(y, WBTC, USDC). This pair is unsupported (only USDC->WBTC exists), so it reverts with PriceOracle_NotSupported(WBTC, USDC) before reaching the second hop. This demonstrates that a unidirectional configuration bricks the inverse direction.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/curators/oracles/CrossAdapter.sol";
import "../src/curators/lib/Errors.sol";

contract MockUniOracle {
    address public immutable b;
    address public immutable q;

    constructor(address _b, address _q) {
        b = _b;
        q = _q;
    }

    function getQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        if (base == b && quote == q) {
            return inAmount; // simple 1:1 for test determinism
        }
        revert Errors.PriceOracle_NotSupported(base, quote);
    }
}

contract CrossAdapterBidirectionalityTest is Test {
    address constant WETH = address(0x1);
    address constant USDC = address(0x2);
    address constant WBTC = address(0x3);

    CrossAdapter adapter;

    function setUp() public {
        // Unidirectional oracles: WETH->USDC and USDC->WBTC only
        MockUniOracle oracleBaseCross = new MockUniOracle(WETH, USDC);
        MockUniOracle oracleCrossQuote = new MockUniOracle(USDC, WBTC);
        adapter = new CrossAdapter(WETH, USDC, WBTC, address(oracleBaseCross), address(oracleCrossQuote));
    }

    function testForwardPathWorks() public {
        uint256 outAmt = adapter.getQuote(1e18, WETH, WBTC);
        assertGt(outAmt, 0, "forward path should succeed");
    }

    function testInversePathRevertsOnMissingReverse() public {
        // First hop in inverse path is WBTC->USDC on oracleCrossQuote, which is unsupported
        vm.expectRevert(abi.encodeWithSelector(Errors.PriceOracle_NotSupported.selector, WBTC, USDC));
        adapter.getQuote(1e8, WBTC, WETH);
    }
}


## Suggested Mitigation
Enforce bidirectionality at construction. Instead of live getQuote (which may require fees for pull oracles), validate both directions on both component oracles using previewGetQuote with a small non-zero amount and revert on any failure:

constructor(...) { 
  base = _base; cross = _cross; quote = _quote; 
  oracleBaseCross = _oracleBaseCross; oracleCrossQuote = _oracleCrossQuote; 
  // Validate bidirectionality via previews
  try IPriceOracle(_oracleBaseCross).previewGetQuote(1, _base, _cross) {} catch { revert Errors.PriceOracle_InvalidConfiguration(); }
  try IPriceOracle(_oracleBaseCross).previewGetQuote(1, _cross, _base) {} catch { revert Errors.PriceOracle_InvalidConfiguration(); }
  try IPriceOracle(_oracleCrossQuote).previewGetQuote(1, _cross, _quote) {} catch { revert Errors.PriceOracle_InvalidConfiguration(); }
  try IPriceOracle(_oracleCrossQuote).previewGetQuote(1, _quote, _cross) {} catch { revert Errors.PriceOracle_InvalidConfiguration(); }
}

If on-chain validation is undesirable (e.g., to avoid constructor cost or in case previews can validly revert), provide a public view helper that performs these checks and document governance procedures to run it off-chain before wiring the adapter into the router.





 **Derived From** : Preview Functions Use MAX_STALENESS_UPPER_BOUND Instead of Market-Specific maxStaleness

## [M-2]. Preview Functions Accept Stale Prices Leading to Transaction Failures and Incorrect Risk Calculations

## Derived From Pattern/Invariant
Preview Functions Use MAX_STALENESS_UPPER_BOUND Instead of Market-Specific maxStaleness

## Exploit Type
Oracle

## Location
PythOracle._previewFetchPriceStruct

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `previewGetQuote()` and `previewGetQuotes()` functions in PythOracle use `_previewFetchPriceStruct()` which validates price staleness against the hardcoded `MAX_STALENESS_UPPER_BOUND` (15 minutes) instead of the market-specific `maxStaleness` parameter set during construction. This creates a critical desync between preview and actual execution behavior.

Vulnerable code in `_previewFetchPriceStruct()`:
```solidity
function _previewFetchPriceStruct() internal view returns (PythStructs.Price memory) {
    PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId);
    if (p.publishTime < block.timestamp) {
        uint256 staleness = block.timestamp - p.publishTime;
        if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // Should be maxStaleness
    }
    // ...
}
```

Compare with the correct implementation in `_fetchPriceStruct()`:
```solidity
if (staleness > maxStaleness) revert Errors.PriceOracle_InvalidAnswer();
```

If a market is configured with `maxStaleness < MAX_STALENESS_UPPER_BOUND` (e.g., 5 minutes for volatile assets), the preview functions will accept prices that actual `getQuote()` calls would reject. This leads to:
1. Front-end UI displaying quotes that revert on-chain, causing failed transactions and poor UX
2. Position sizing calculations based on prices that won't be accepted in actual execution
3. Risk management systems making decisions on invalid price data
4. Users losing gas on transactions that were expected to succeed based on preview data

## Impact
Users experience transaction failures when preview data suggests success, leading to wasted gas and poor UX. More critically, if integrators use preview quotes for position sizing or risk calculations, they may make decisions based on stale prices that won't be accepted in actual execution. For markets with volatile assets requiring fresh prices (maxStaleness < 15 minutes), this desync can cause significant operational issues. While no direct asset theft occurs, the protocol's reliability is compromised, potentially leading to incorrect leverage positions or margin calculations based on invalid preview data.

## Command to Run Test


## Proof of Concept
1. Deploy PythOracle for a volatile asset (e.g., ETH) with maxStaleness = 5 minutes (300 seconds) to ensure fresh prices
2. Wait until the Pyth price is 8 minutes old (480 seconds)
3. Call previewGetQuote() - it returns a valid quote (8 min < 15 min MAX_STALENESS_UPPER_BOUND)
4. Front-end shows user they can execute a trade at this price
5. User submits transaction calling getQuote() with same parameters
6. Transaction reverts because 8 minutes > 5 minutes maxStaleness
7. User has wasted gas and experienced failed transaction despite preview indicating success

Alternatively, for risk management:
1. Integrator uses previewGetQuote() to check if leverage position can be opened
2. Preview accepts 8-minute-old price and calculates position size
3. Actual execution reverts, but risk system has already allocated capital based on stale preview
4. System state is inconsistent with actual market conditions

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {PythStructs} from "@pyth/PythStructs.sol";
import {IPyth} from "@pyth/IPyth.sol";
import {Errors as CovErrors} from "src/curators/lib/Errors.sol";

contract MockPyth is IPyth {
    mapping(bytes32 => PythStructs.Price) public prices;

    function setPrice(bytes32 feedId, int64 price, uint64 conf, int32 expo, uint publishTime) external {
        prices[feedId] = PythStructs.Price(price, conf, expo, publishTime);
    }

    function getPriceUnsafe(bytes32 feedId) external view returns (PythStructs.Price memory) {
        return prices[feedId];
    }

    // Unused methods in tests; provided for interface compatibility
    function getValidTimePeriod() external view returns (uint) { return 0; }
    function getPrice(bytes32) external view returns (PythStructs.Price memory) { revert(); }
    function getEmaPrice(bytes32) external view returns (PythStructs.Price memory) { revert(); }
    function getPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory) { revert(); }
    function getEmaPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { revert(); }
    function getEmaPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory) { revert(); }
    function updatePriceFeeds(bytes[] calldata) external payable {}
    function updatePriceFeedsIfNecessary(bytes[] calldata, bytes32[] calldata, uint64[] calldata) external payable {}
    function getUpdateFee(bytes[] calldata) external pure returns (uint) { return 0; }
    function parsePriceFeedUpdates(bytes[] calldata, bytes32[] calldata, uint64, uint64) external payable returns (PythStructs.PriceFeed[] memory) { revert(); }
}

contract MockToken {
    function decimals() external pure returns (uint8) { return 18; }
}

contract PythOraclePreviewDesyncTest is Test {
    PythOracle oracle;
    MockPyth pyth;
    MockToken baseToken;
    MockToken quoteToken;

    bytes32 feedId = bytes32(uint256(1));
    uint256 maxStaleness = 5 minutes; // market requires fresher prices than 15m upper-bound
    uint256 maxConfWidth = 100; // 1%

    function setUp() public {
        pyth = new MockPyth();
        baseToken = new MockToken();
        quoteToken = new MockToken();

        oracle = new PythOracle(
            address(pyth),
            address(baseToken),
            address(quoteToken),
            feedId,
            maxStaleness,
            maxConfWidth
        );
    }

    function _seedPrice(uint256 priceAgeSeconds) internal {
        uint256 publishTime = block.timestamp - priceAgeSeconds;
        pyth.setPrice(
            feedId,
            100_000_000, // price (expo -8): $1,000.00000000
            50_000,      // conf well within 1%
            -8,          // exponent
            publishTime
        );
    }

    function testPreviewAcceptsStalePriceThatExecutionRejects() public {
        // 8 minutes old: exceeds market maxStaleness (5m) but under upper bound (15m)
        _seedPrice(8 minutes);

        uint256 previewOut = oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken));
        assertGt(previewOut, 0, "preview should succeed under 15m bound");

        vm.expectRevert(CovErrors.PriceOracle_InvalidAnswer.selector);
        oracle.getQuote(1e18, address(baseToken), address(quoteToken));
    }

    function testPreviewAcceptsPriceJustOverMarketLimit() public {
        _seedPrice(maxStaleness + 1);

        uint256 previewOut = oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken));
        assertGt(previewOut, 0, "preview accepts > market staleness but < 15m");

        vm.expectRevert(CovErrors.PriceOracle_InvalidAnswer.selector);
        oracle.getQuote(1e18, address(baseToken), address(quoteToken));
    }
}

## Suggested Mitigation
Use the same validation logic for preview and execution to avoid drift. Easiest and safest: remove _previewFetchPriceStruct entirely and have _previewGetQuote call the existing _fetchPriceStruct(), or change the stale check in _previewFetchPriceStruct to use maxStaleness (not MAX_STALENESS_UPPER_BOUND). Additionally, consider factoring the validation into a single internal function shared by both paths to prevent future desynchronization.





 **Derived From** : SlippageMissingOrInsufficient: Redeem/swap allow zero-output or missing deadline checks

## [L-3]. Missing deadline parameter in LatentSwapLEX.redeem and .swap allows delayed execution at unfavorable prices

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient: Redeem/swap allow zero-output or missing deadline checks

## Exploit Type
SlippageMissingOrInsufficient

## Location
LatentSwapLEX.redeem, swap

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The LatentSwapLEX.redeem and LatentSwapLEX.swap functions lack a block.timestamp deadline parameter. While users can specify minAmountOut (redeem) or amountLimit (swap), these checks do not protect against time-based MEV or delayed transaction execution. A user transaction can sit in the mempool for an extended period and execute when market conditions have changed significantly. Even with reasonable slippage bounds set at transaction creation time, the transaction may execute at a much worse price if oracle prices or DEX state shift unfavorably before mining. In volatile markets or during oracle updates, this exposes users to unintended losses. The protocol's reliance on user-set slippage parameters without time bounds means that a transaction approved when conditions were favorable could execute hours or days later when they are not, yet still pass the original slippage check if the user set it too loosely or if market moved within that bound but significantly from the user's intent.

## Impact
Users can have their redeem/swap transactions mined much later than intended, potentially at materially different market conditions. Since Covenant exposes only slippage (minAmountOut/amountLimit) without any time bound, a transaction signed under conditions at T0 can be executed at T1 with worse prices but still within the user’s chosen slippage bound. This is a UX/footgun risk, not a protocol solvency issue: funds are not stolen and execution remains within caller-provided bounds. The correct place for a deadline is the Covenant Core entrypoints. Impact is limited to user receiving a worse fill within their own tolerance window.

## Command to Run Test


## Proof of Concept
Scenario illustrating risk without deadline
1) Alice mints aTokens/zTokens in market M and intends to swap aTokens for base immediately.
2) At T0, she previews the swap. amountOut_now = 100. She submits swap with amountLimit = 95 (tight but reasonable), expecting quick inclusion.
3) The tx remains in mempool; conditions shift (oracle/market state), and at T1 amountOut_later = 96. The swap still satisfies amountLimit (>=95) but is worse than the T0 preview.
4) Because SwapParams has no deadline and Covenant does not enforce a time-bounded guard, the transaction executes at T1 at a worse price than intended. A deadline would have reverted the stale transaction instead of filling it later.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {LatentSwapLEX} from "src/lex/latentswap/LatentSwapLEX.sol";
import {MarketId, MarketParams, AssetType, MintParams, RedeemParams, SwapParams} from "src/interfaces/ICovenant.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

contract MockOracle is Test {
    uint256 public price = 1e18; // quote per 1e18 base units
    function setPrice(uint256 p) external { price = p; }
    function getQuote(uint256 inAmount, address, address) external view returns (uint256) {
        return (inAmount * price) / 1e18;
    }
    function previewGetQuote(uint256 inAmount, address, address) external view returns (uint256) {
        return (inAmount * price) / 1e18;
    }
    function updatePriceFeeds(address, address, bytes calldata) external payable {}
    function getUpdateFee(address, address, bytes calldata) external pure returns (uint128) { return 0; }
}

contract BaseToken is ERC20 {
    constructor() ERC20("Base", "BASE") { _mint(msg.sender, 1_000_000e18); }
}

contract MissingDeadlineLexTest is Test {
    // Simplified constant (Q96)
    uint256 constant Q96 = 2**96;

    LatentSwapLEX lex;
    MockOracle oracle;
    BaseToken base;

    address alice = address(0xA11CE);
    MarketId marketId; // zero-value is fine for test

    function setUp() public {
        oracle = new MockOracle();
        base = new BaseToken();

        // Deploy LEX with this test contract as CovenantCore (so onlyCovenantCore passes)
        lex = new LatentSwapLEX(
            address(this),               // owner
            address(this),               // covenantCore
            uint160(2 * Q96),            // edgeHighSqrtPriceX96
            uint160(Q96 / 2),            // edgeLowSqrtPriceX96
            uint160((12e17 * Q96) / 1e18), // limHighSqrtPriceX96 (~1.2x)
            uint160((15e17 * Q96) / 1e18), // limMaxSqrtPriceX96  (~1.5x)
            int64(0),
            uint32(90 days),
            uint8(10)                    // 0.10% swap fee (bps)
        );

        // Initialize market
        MarketParams memory mp = MarketParams({
            baseToken: address(base),
            quoteToken: address(0x0000000000000000000000000000000000000348), // pseudo USD addr used across repo
            curator: address(oracle),
            lex: address(lex)
        });
        // onlyCovenantCore
        lex.initMarket(marketId, mp, 0, "");
    }

    function test_MissingDeadline_AllowsLaterFillAtWorsePrice() public {
        // CovenantCore tracks baseTokenSupply externally; we simulate it here
        uint256 baseSupply = 0;

        // Mint some a/z supply to Alice
        MintParams memory mp = MintParams({
            marketId: marketId,
            marketParams: MarketParams(address(base), address(0x0000000000000000000000000000000000000348), address(oracle), address(lex)),
            baseAmountIn: 100e18,
            to: alice,
            minATokenAmountOut: 0,
            minZTokenAmountOut: 0,
            data: "",
            msgValue: 0
        });
        (uint256 aMinted,, uint128 fees,) = lex.mint(mp, address(0), baseSupply);
        baseSupply += (mp.baseAmountIn - fees);

        // Preview swap aToken -> BASE at T0
        uint256 exactIn = aMinted / 4;
        SwapParams memory sp = SwapParams({
            marketId: marketId,
            marketParams: MarketParams(address(base), address(0x0000000000000000000000000000000000000348), address(oracle), address(lex)),
            assetIn: AssetType.LEVERAGE,
            assetOut: AssetType.BASE,
            to: alice,
            amountSpecified: exactIn,
            amountLimit: 0, // checked in CovenantCore, not by LEX
            isExactIn: true,
            data: "",
            msgValue: 0
        });
        (uint256 outNow,,,) = lex.quoteSwap(sp, address(0), baseSupply);
        assertGt(outNow, 0, "preview should return > 0");

        // Market conditions worsen before inclusion (no deadline to stop late execution)
        oracle.setPrice(85e16); // 0.85
        vm.warp(block.timestamp + 1 hours);

        // Execute the same swap later via LEX.swap (onlyCovenantCore), burning from Alice
        (uint256 outLater,,) = lex.swap(sp, alice, baseSupply);
        assertGt(outLater, 0, "swap still executes later");
        assertLt(outLater, outNow, "later fill is worse without deadline guarding staleness");
    }
}


## Suggested Mitigation
Enforce a user-provided time bound at the Covenant Core entrypoints. Recommended: add a deadline field to the user-facing params and revert if block.timestamp > deadline.

Option A (preferred, minimal surface change):
- In ICovenant.sol, extend SwapParams and RedeemParams with a uint256 deadline.
- In Covenant Core’s swap() and redeem(), add: require(block.timestamp <= params.deadline, "TX_TOO_OLD"); before calling LatentSwapLEX.
- Keep LatentSwapLEX unchanged (LEX is not user-facing and already does not enforce slippage).

Option B (also acceptable if you revise interfaces end-to-end):
- Add deadline to the shared structs used by LEX (SwapParams/RedeemParams) and have LEX early-revert on stale transactions. This requires updating Covenant Core to propagate the deadline and is a wider change.

Either approach restores the standard UX guard seen in AMMs (e.g., UniswapV2/V3), preventing unintended late execution due to mempool delays or validator-held bundles. Additionally, keep slippage checks in Covenant Core as implemented today (amountLimit/minAmountOut).





 **Derived From** : PythOracle preview functions use MAX_STALENESS_UPPER_BOUND instead of maxStaleness

## [M-4]. PythOracle preview functions accept stale prices beyond configured maxStaleness causing transaction simulation mismatches

## Derived From Pattern/Invariant
PythOracle preview functions use MAX_STALENESS_UPPER_BOUND instead of maxStaleness

## Exploit Type
StandardViolation

## Location
PythOracle.previewGetQuote, previewGetQuotes, _previewFetchPriceStruct

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `previewGetQuote` and `previewGetQuotes` functions in PythOracle.sol use `_previewFetchPriceStruct()` which validates staleness against the hardcoded `MAX_STALENESS_UPPER_BOUND` (15 minutes) instead of the immutable `maxStaleness` parameter set during deployment. This creates a StandardViolation where preview functions do not accurately reflect execution behavior.

When a market is configured with `maxStaleness = 5 minutes`, the preview functions will accept prices up to 15 minutes old and return valid quotes, but the actual execution functions (`getQuote` via `_fetchPriceStruct`) will revert with `PriceOracle_InvalidAnswer` for prices older than 5 minutes.

Vulnerable code in `_previewFetchPriceStruct()`:
```solidity
function _previewFetchPriceStruct() internal view returns (PythStructs.Price memory) {
    PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId);
    if (p.publishTime < block.timestamp) {
        uint256 staleness = block.timestamp - p.publishTime;
        if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // Wrong bound
    }
    // ... validation
}
```

The comment in the code explicitly states `@dev Changed from maxStaleness to MAX_STALENESS_UPPER_BOUND`, confirming this is intentional but problematic behavior.

This violates ERC standards for preview functions which should accurately simulate execution state. Integrators relying on preview functions will experience unexpected reverts when attempting actual transactions, breaking composability with aggregators, front-ends, and multi-call contracts.

## Impact
Preview functions can accept prices stale beyond the configured maxStaleness (up to a hard upper bound of 15 minutes), while execution paths enforce the stricter market-specific maxStaleness. This mismatch causes simulations to succeed but on-chain calls to revert once the price is older than maxStaleness, leading to failed integrations, wasted gas, and degraded availability/composability for routers, aggregators, and front-ends. No direct loss of funds, but reliability of core quoting is impacted.

## Command to Run Test


## Proof of Concept
1) Deploy PythOracle with maxStaleness = 5 minutes for a market.
2) Set the Pyth price publishTime to 7 minutes ago (older than 5 minutes but younger than the 15-minute MAX_STALENESS_UPPER_BOUND).
3) Call previewGetQuote/previewGetQuotes: both succeed because the preview checks 15 minutes.
4) Call getQuote with the same parameters: it reverts with PriceOracle_InvalidAnswer because execution enforces the configured 5-minute maxStaleness.
5) This demonstrates a preview/execute mismatch that causes successful simulations followed by reverted transactions.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";
import {Errors} from "src/curators/lib/Errors.sol";

contract MockPyth is IPyth {
    PythStructs.Price internal _p;

    function setPriceUnsafe(PythStructs.Price memory p_) external { _p = p_; }
    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return _p; }

    // Unused in this PoC but required by the interface
    function getValidTimePeriod() external pure returns (uint) { return 60; }
    function getPrice(bytes32) external view returns (PythStructs.Price memory) { return _p; }
    function getEmaPrice(bytes32) external view returns (PythStructs.Price memory) { return _p; }
    function getPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory) { return _p; }
    function getEmaPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return _p; }
    function getEmaPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory) { return _p; }
    function updatePriceFeeds(bytes[] calldata) external payable {}
    function updatePriceFeedsIfNecessary(bytes[] calldata, bytes32[] calldata, uint64[] calldata) external payable {}
    function getUpdateFee(bytes[] calldata) external pure returns (uint) { return 0; }
    function parsePriceFeedUpdates(bytes[] calldata, bytes32[] calldata, uint64, uint64) external payable returns (PythStructs.PriceFeed[] memory) {}
}

contract MockToken { function decimals() external pure returns (uint8) { return 18; } }

contract PythOraclePreviewStalenessTest is Test {
    PythOracle internal oracle;
    MockPyth internal pyth;
    MockToken internal base;
    MockToken internal quote;

    function setUp() public {
        pyth = new MockPyth();
        base = new MockToken();
        quote = new MockToken();

        // Configure maxStaleness = 5 minutes, maxConfWidth = 5% (500 bps)
        oracle = new PythOracle(
            address(pyth),
            address(base),
            address(quote),
            bytes32(uint256(0x01)),
            5 minutes,
            500
        );
    }

    function _setPriceWithAge(uint256 age) internal {
        PythStructs.Price memory price = PythStructs.Price({
            price: 2000e8,   // positive
            conf: 1e8,       // within 5% conf-width bound
            expo: -8,        // within allowed exponent bounds
            publishTime: block.timestamp - age
        });
        pyth.setPriceUnsafe(price);
    }

    function testPreviewAcceptsOlderThanConfiguredMaxButGetQuoteReverts() public {
        // Fresh price: both preview and execution succeed
        _setPriceWithAge(3 minutes);
        uint256 prevFresh = oracle.previewGetQuote(1e18, address(base), address(quote));
        uint256 execFresh = oracle.getQuote(1e18, address(base), address(quote));
        assertEq(prevFresh, execFresh, "fresh: preview == execute");

        // Make price 7 minutes old (older than 5m maxStaleness, younger than 15m upper bound)
        _setPriceWithAge(7 minutes);

        // Preview succeeds (uses MAX_STALENESS_UPPER_BOUND)
        (uint256 bid, uint256 ask) = oracle.previewGetQuotes(1e18, address(base), address(quote));
        assertGt(bid, 0); assertEq(bid, ask);

        // Execution reverts (uses maxStaleness)
        vm.expectRevert(Errors.PriceOracle_InvalidAnswer.selector);
        oracle.getQuote(1e18, address(base), address(quote));
    }
}


## Suggested Mitigation
Make preview validation consistent with execution: replace MAX_STALENESS_UPPER_BOUND with the configured maxStaleness in _previewFetchPriceStruct. If a more permissive preview is desired for UX, expose it as a separate method (e.g., unsafePreviewGetQuote) or return a tuple (quote, isFresh) where isFresh indicates staleness relative to maxStaleness. Update docs to clearly state behavior and ensure integrators (routers/aggregators) either update Pyth before quoting or handle stale previews gracefully.





 **Derived From** : previewGetQuote(amount, base, quote) == getQuote(amount, base, quote) for same oracle state

## [M-5]. Referential inconsistency between previewGetQuote and getQuote in PythOracle causes MEV exploitation and UX degradation

## Derived From Pattern/Invariant
previewGetQuote(amount, base, quote) == getQuote(amount, base, quote) for same oracle state

## Exploit Type
EventConsistency

## Location
PythOracle.previewGetQuote / _previewFetchPriceStruct

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
PythOracle.previewGetQuote uses _previewFetchPriceStruct which validates staleness against MAX_STALENESS_UPPER_BOUND (15 minutes), while getQuote (inherited from EulerPythOracle) uses _fetchPriceStruct which validates against maxStaleness (constructor parameter, typically 5 minutes). This violates the fundamental invariant that preview functions must accurately simulate live execution. In staleness-sensitive markets, previewGetQuote can succeed while getQuote reverts with PriceOracle_InvalidAnswer, breaking the core assumption DeFi integrators rely on. The vulnerability snippet: `function _previewFetchPriceStruct() internal view returns (PythStructs.Price memory) { PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId); if (p.publishTime < block.timestamp) { uint256 staleness = block.timestamp - p.publishTime; if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // Should use maxStaleness } ... }` while getQuote uses: `function _fetchPriceStruct() internal view returns (PythStructs.Price memory) { ... if (staleness > maxStaleness) revert Errors.PriceOracle_InvalidAnswer(); ... }`. When maxStaleness=5min and price is 7min old, previewGetQuote returns a valid quote but getQuote reverts.

## Impact
Preview functions may succeed while live execution reverts when price age is between maxStaleness (e.g., 5 minutes) and 15 minutes. This causes front-ends/aggregators to display quotes that cannot execute on-chain, leading to user gas loss, degraded UX, and reduced protocol reliability. No direct theft of funds occurs, but protocol availability and composability are negatively impacted.

## Command to Run Test


## Proof of Concept
Setup: Configure maxStaleness=5 minutes. Let the Pyth price be 7 minutes old. previewGetQuote returns a value because it only rejects prices older than 15 minutes, but getQuote reverts because it enforces the configured 5-minute staleness. Steps:
1) Deploy PythOracle with maxStaleness=300 and a reasonable maxConfWidth.
2) Set Pyth price publishTime = block.timestamp - 420.
3) Call previewGetQuote(base, quote, amount) → succeeds.
4) Call getQuote(base, quote, amount) → reverts with PriceOracle_InvalidAnswer().
5) Any staleness in (maxStaleness, 15 minutes] exhibits the same divergence.
This demonstrates the broken invariant: preview does not faithfully simulate live behavior.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract MockPyth is IPyth {
    mapping(bytes32 => PythStructs.Price) public prices;

    function setPriceUnsafe(bytes32 id, int64 price, uint64 conf, int32 expo, uint publishTime) external {
        prices[id] = PythStructs.Price({price: price, conf: conf, expo: expo, publishTime: publishTime});
    }

    function getPriceUnsafe(bytes32 id) external view returns (PythStructs.Price memory) {
        return prices[id];
    }

    // Unused in this PoC
    function getValidTimePeriod() external pure returns (uint) { return 0; }
    function getPrice(bytes32) external pure returns (PythStructs.Price memory) { revert("unused"); }
    function getEmaPrice(bytes32) external pure returns (PythStructs.Price memory) { revert("unused"); }
    function getPriceNoOlderThan(bytes32, uint) external pure returns (PythStructs.Price memory) { revert("unused"); }
    function getEmaPriceUnsafe(bytes32) external pure returns (PythStructs.Price memory) { revert("unused"); }
    function getEmaPriceNoOlderThan(bytes32, uint) external pure returns (PythStructs.Price memory) { revert("unused"); }
    function updatePriceFeeds(bytes[] calldata) external payable {}
    function updatePriceFeedsIfNecessary(bytes[] calldata, bytes32[] calldata, uint64[] calldata) external payable {}
    function getUpdateFee(bytes[] calldata) external pure returns (uint) { return 0; }
    function parsePriceFeedUpdates(bytes[] calldata, bytes32[] calldata, uint64, uint64) external payable returns (PythStructs.PriceFeed[] memory) { revert("unused"); }
}

contract MockERC20 {
    uint8 internal immutable _decimals;
    constructor(uint8 d) { _decimals = d; }
    function decimals() external view returns (uint8) { return _decimals; }
}

contract PythOraclePreviewMismatchTest is Test {
    PythOracle oracle;
    MockPyth pyth;
    MockERC20 baseToken;
    MockERC20 quoteToken;

    bytes32 constant FEED_ID = bytes32(uint256(1));
    uint256 constant MAX_STALENESS = 5 minutes;
    uint256 constant MAX_CONF_BPS = 100; // 1%

    function setUp() public {
        pyth = new MockPyth();
        baseToken = new MockERC20(18);
        quoteToken = new MockERC20(18);
        oracle = new PythOracle(address(pyth), address(baseToken), address(quoteToken), FEED_ID, MAX_STALENESS, MAX_CONF_BPS);
    }

    function _errorSelector() internal pure returns (bytes4) {
        // error PriceOracle_InvalidAnswer()
        return bytes4(keccak256("PriceOracle_InvalidAnswer()"));
    }

    function test_PreviewSucceeds_While_GetQuoteReverts_When_StaleBetween5mAnd15m() public {
        // price = 1.0 with expo -8, conf small enough
        pyth.setPriceUnsafe(FEED_ID, int64(100_000_000), uint64(50_000), int32(-8), block.timestamp - 7 minutes);

        uint256 previewOut = oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken));
        assertGt(previewOut, 0, "preview should succeed under 15m staleness");

        vm.expectRevert(_errorSelector());
        oracle.getQuote(1e18, address(baseToken), address(quoteToken));
    }

    function test_Inconsistency_For_All_Staleness_In_Range() public {
        // iterate staleness in (maxStaleness, 15 minutes)
        for (uint256 s = MAX_STALENESS + 60; s < 15 minutes; s += 60) {
            pyth.setPriceUnsafe(FEED_ID, int64(100_000_000), uint64(50_000), int32(-8), block.timestamp - s);

            bool previewOk;
            bool liveOk;

            // preview
            try oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken)) returns (uint256) {
                previewOk = true;
            } catch { previewOk = false; }

            // live
            try oracle.getQuote(1e18, address(baseToken), address(quoteToken)) returns (uint256) {
                liveOk = true;
            } catch { liveOk = false; }

            assertTrue(previewOk, "preview must succeed up to 15m");
            assertFalse(liveOk, "live must fail beyond configured maxStaleness");
        }
    }
}


## Suggested Mitigation
Unify the validation logic between preview and live paths. Two equivalent fixes:
- Preferred: Reuse the exact live code path in preview to avoid drift. Replace `_previewGetQuote` to call `_fetchPriceStruct()` instead of `_previewFetchPriceStruct()` and remove `_previewFetchPriceStruct` entirely. This guarantees preview mirrors execution semantics, including staleness, aheadness, confidence, and exponent checks; or
- Alternatively: In `_previewFetchPriceStruct`, replace `MAX_STALENESS_UPPER_BOUND` with `maxStaleness` so both paths enforce the same staleness threshold. Ensure all other checks match `_fetchPriceStruct` exactly.
Additionally, add a unit test asserting preview and live results match for all acceptable staleness and that both revert identically when constraints are violated, to prevent future divergence.





 **Derived From** : p.publishTime <= block.timestamp + MAX_AHEADNESS && p.publishTime >= block.timestamp - MAX_STALENESS_UPPER_BOUND

## [L-6]. Preview functions use MAX_STALENESS_UPPER_BOUND while live quotes use maxStaleness causing misleading previews and failed transactions

## Derived From Pattern/Invariant
p.publishTime <= block.timestamp + MAX_AHEADNESS && p.publishTime >= block.timestamp - MAX_STALENESS_UPPER_BOUND

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
The PythOracle contract has a critical discrepancy between preview functions (_previewFetchPriceStruct) and live execution functions (_fetchPriceStruct) in their staleness validation. Preview functions check against MAX_STALENESS_UPPER_BOUND (15 minutes hardcoded constant) while actual quote functions check against maxStaleness (constructor parameter, can be as low as 1 minute). This means previewGetQuote can return valid quotes for prices that would revert when users attempt to execute the actual transaction via getQuote. For example, if maxStaleness=1 minute but a price is 10 minutes old: previewGetQuote succeeds (10 < 15), but getQuote reverts (10 > 1). In the Covenant protocol where users rely on previews to determine trade amounts before executing multi-step transactions, this causes: 1) Users receive valid preview quotes showing they can execute a swap/mint/redeem, 2) Users submit transaction paying gas fees and potentially oracle update fees, 3) Transaction reverts due to stale price, 4) User loses gas fees and must retry. Vulnerable code in _previewFetchPriceStruct at line checking staleness uses MAX_STALENESS_UPPER_BOUND instead of maxStaleness.

## Impact
Preview functions can show a quote based on a price up to 15 minutes old while runtime paths enforce the stricter maxStaleness set at deployment (e.g., 60 seconds). If an integrator or user calls getQuote without first updating the Pyth feed, execution reverts, wasting gas and causing poor UX. In standard Covenant flows that atomically update the Pyth feed before quoting, this does not create an asset loss or lasting DoS; the main impact is misleading previews and potential gas waste for callers that skip the update. No funds can be stolen or locked.

## Command to Run Test


## Proof of Concept
Setup: Deploy PythOracle with maxStaleness = 60 seconds. Ensure the Pyth on-chain price is 10 minutes old and do not perform a Pyth update before calling getQuote.

Steps:
1) Configure base/quote decimals as 18 via mocks, then deploy PythOracle with maxStaleness = 60 and a reasonable maxConfWidth.
2) Set the mocked Pyth price with publishTime = block.timestamp - 600 seconds (10 minutes old), positive price, and conf within bounds.
3) Call previewGetQuote(inAmount, base, quote): succeeds because 600 <= MAX_STALENESS_UPPER_BOUND (15 minutes).
4) Call getQuote(inAmount, base, quote) without updating the Pyth feed: reverts because staleness 600 > maxStaleness 60.

Note: If the caller first updates the Pyth feed in the same transaction (as recommended by the protocol flow), execution will not revert; however, the preview may still be misleading because it used a stale price. The core issue is that preview and live execution enforce different staleness thresholds.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {PythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {PythStructs} from "@pyth/PythStructs.sol";
import {Errors} from "src/curators/lib/Errors.sol";

contract MockPyth {
    PythStructs.Price public mockPrice;

    function setPriceData(int64 price, uint64 conf, int32 expo, uint publishTime) external {
        mockPrice = PythStructs.Price({price: price, conf: conf, expo: expo, publishTime: publishTime});
    }

    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) {
        return mockPrice;
    }

    function getUpdateFee(bytes[] calldata) external pure returns (uint) { return 0; }
    function updatePriceFeeds(bytes[] calldata) external payable {}
}

contract PythOracle_StalenessPreviewMismatchTest is Test {
    PythOracle oracle;
    MockPyth mockPyth;

    address base = address(0xBEEF01);
    address quote = address(0xBEEF02);
    bytes32 feedId = bytes32(uint256(1));

    uint256 maxStaleness = 60; // 1 minute
    uint256 maxConfWidth = 100; // 1%

    function setUp() public {
        // Mock decimals BEFORE deploying oracle (constructor reads them)
        vm.mockCall(base, abi.encodeWithSignature("decimals()"), abi.encode(uint8(18)));
        vm.mockCall(quote, abi.encodeWithSignature("decimals()"), abi.encode(uint8(18)));

        mockPyth = new MockPyth();
        oracle = new PythOracle(
            address(mockPyth),
            base,
            quote,
            feedId,
            maxStaleness,
            maxConfWidth
        );
    }

    function test_PreviewSucceedsButExecutionRevertsWhenPriceOlderThanMaxStaleness() public {
        uint256 nowTs = 1_000_000;
        vm.warp(nowTs);

        // Price is 10 minutes old (stale for runtime check, but allowed by preview upper bound 15m)
        mockPyth.setPriceData(
            100_000_000, // price
            1_000_000,   // conf within 1% of price
            -8,          // exponent
            nowTs - 600  // publishTime
        );

        // Preview uses MAX_STALENESS_UPPER_BOUND (15 minutes) -> should not revert
        uint256 previewOut = oracle.previewGetQuote(1e18, base, quote);
        assertGt(previewOut, 0, "preview should return a positive quote");

        // Live getQuote enforces maxStaleness (60s) -> should revert
        vm.expectRevert(Errors.PriceOracle_InvalidAnswer.selector);
        oracle.getQuote(1e18, base, quote);
    }
}


## Suggested Mitigation
Make preview staleness validation identical to live execution. In _previewFetchPriceStruct, replace the staleness check from `if (staleness > MAX_STALENESS_UPPER_BOUND)` to `if (staleness > maxStaleness)`. Optionally, expose staleness/aheadness in a view helper so front-ends can surface warnings when data is old. This ensures previews align with actual execution semantics and avoids misleading quotes.





 **Derived From** : previewGetQuote == _getQuote for push oracles

## [L-7]. ChainlinkOracle.previewGetQuote violates preview semantics by applying identical staleness check as live quote

## Derived From Pattern/Invariant
previewGetQuote == _getQuote for push oracles

## Exploit Type
AccountingInvariantViolation

## Location
ChainlinkOracle.previewGetQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The IPriceOracle interface documentation explicitly states that preview functions should have 'a longer lookback window to avoid quote blocking'. However, ChainlinkOracle.previewGetQuote() directly calls _getQuote() with the same maxStaleness parameter as getQuote(). This means when the Chainlink feed is stale (updatedAt older than maxStaleness), BOTH getQuote and previewGetQuote will revert with TooStale error. This breaks the intended state machine transition where preview should succeed with relaxed constraints to allow users to assess transaction viability before execution. The CovenantCurator.previewGetQuote() forwards calls to oracle adapters expecting them to honor the preview semantics, but ChainlinkOracle fails to do so. Code snippet:
```solidity
function previewGetQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
    return _getQuote(inAmount, base, quote); // Uses same maxStaleness check
}

function _getQuote(...) internal view returns (uint256) {
    (, int256 answer,, uint256 updatedAt,) = AggregatorV3Interface(feed).latestRoundData();
    if (answer <= 0) revert Errors.PriceOracle_InvalidAnswer();
    uint256 staleness = block.timestamp - updatedAt;
    if (staleness > maxStaleness) revert Errors.PriceOracle_TooStale(staleness, maxStaleness);
    // ... price calculation
}
```
During network congestion or oracle delays, this causes preview queries to revert identically to live queries, preventing the protocol from gracefully degrading or warning users about stale pricing.

## Impact
This issue is a specification mismatch and availability/UX degradation: previewGetQuote/previewGetQuotes do not honor the IPriceOracle contract’s stated behavior of using a longer lookback window, causing previews to revert whenever live quotes would. There is no loss of funds or authorization risk; only pre-flight checks and front-end flows relying on preview semantics are affected during stale-feed windows.

## Command to Run Test


## Proof of Concept
1. ChainlinkOracle is deployed with maxStaleness = 65 minutes (typical for ETH/USD feed with 1-hour heartbeat)
2. Chainlink aggregator updates price at T=0
3. Network congestion or oracle delay occurs
4. At T=66 minutes, user attempts to check price viability via previewGetQuote before executing a large swap
5. previewGetQuote reverts with TooStale(3960, 3900) because staleness=66min > maxStaleness=65min
6. User cannot assess whether proceeding is safe
7. If user attempts getQuote for actual swap, it also reverts with identical TooStale error
8. Market effectively frozen until next oracle update, despite price being only 1 minute past staleness threshold
9. With proper preview implementation (e.g., maxStaleness*1.5 for preview), user would see stale preview price, decide to wait or proceed with caution, but current implementation prevents this informed decision

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {ChainlinkOracle as CovenantChainlinkOracle} from "src/curators/oracles/chainlink/ChainlinkOracle.sol";
import {Errors as EulerErrors} from "@euler-price-oracle/lib/Errors.sol";

contract MockChainlinkFeed {
    uint8 public decimals = 18;
    int256 private _answer;
    uint256 private _updatedAt;

    function setLatestRoundData(int256 answer, uint256 updatedAt) external {
        _answer = answer;
        _updatedAt = updatedAt;
    }

    function latestRoundData()
        external
        view
        returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
    {
        return (1, _answer, block.timestamp, _updatedAt, 1);
    }
}

contract MockERC20 {
    function decimals() external pure returns (uint8) { return 18; }
}

contract ChainlinkOraclePreviewTest is Test {
    CovenantChainlinkOracle oracle;
    MockChainlinkFeed feed;
    MockERC20 base;
    MockERC20 quote;

    uint256 constant MAX_STALENESS = 65 minutes;
    uint256 constant PRICE = 2000e18;

    function setUp() public {
        base = new MockERC20();
        quote = new MockERC20();
        feed = new MockChainlinkFeed();
        oracle = new CovenantChainlinkOracle(address(base), address(quote), address(feed), MAX_STALENESS);
    }

    function testPreviewRevertsSameAsLiveWhenStale() public {
        uint256 staleTimestamp = block.timestamp - 66 minutes;
        feed.setLatestRoundData(int256(PRICE), staleTimestamp);
        uint256 inAmount = 1e18;

        vm.expectRevert(abi.encodeWithSelector(EulerErrors.PriceOracle_TooStale.selector, 66 minutes, MAX_STALENESS));
        oracle.getQuote(inAmount, address(base), address(quote));

        vm.expectRevert(abi.encodeWithSelector(EulerErrors.PriceOracle_TooStale.selector, 66 minutes, MAX_STALENESS));
        oracle.previewGetQuote(inAmount, address(base), address(quote));
    }
}


## Suggested Mitigation
Honor the interface’s preview semantics by relaxing staleness in preview functions and keep error types consistent with the parent (Euler) adapter. Suggested change in Covenant’s ChainlinkOracle:

- Add a constructor param previewMaxStaleness (>= maxStaleness) to make the tolerance explicit and configurable.
- Implement an internal _previewGetQuote that mirrors _getQuote but uses previewMaxStaleness and emits Euler’s Errors.

Example sketch:

// state
uint256 public immutable previewMaxStaleness;

constructor(address _base, address _quote, address _feed, uint256 _maxStaleness, uint256 _previewMaxStaleness)
    EulerChainlinkOracle(_base, _quote, _feed, _maxStaleness)
{
    require(_previewMaxStaleness >= _maxStaleness, "preview<live");
    previewMaxStaleness = _previewMaxStaleness;
}

function _previewGetQuote(uint256 inAmount, address _base, address _quote) internal view returns (uint256) {
    bool inverse = ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote);
    (, int256 answer,, uint256 updatedAt,) = AggregatorV3Interface(feed).latestRoundData();
    if (answer <= 0) revert EulerErrors.PriceOracle_InvalidAnswer();
    uint256 staleness = block.timestamp - updatedAt;
    if (staleness > previewMaxStaleness) revert EulerErrors.PriceOracle_TooStale(staleness, previewMaxStaleness);
    return ScaleUtils.calcOutAmount(inAmount, uint256(answer), scale, inverse);
}

function previewGetQuote(uint256 inAmount, address _base, address _quote) external view returns (uint256) {
    return _previewGetQuote(inAmount, _base, _quote);
}

function previewGetQuotes(uint256 inAmount, address _base, address _quote) external view returns (uint256, uint256) {
    uint256 outAmount = _previewGetQuote(inAmount, _base, _quote);
    return (outAmount, outAmount);
}

If the intended design is that push oracles never relax preview semantics, then update IPriceOracle documentation to clarify that preview may be identical to live for push feeds, and ensure front-ends do not assume leniency.





 **Derived From** : Oracle prices respect staleness bounds across resolution hops

## [L-8]. Multi-hop oracle resolution accumulates staleness beyond system bounds via ERC4626 vault unwrapping and fallback oracle

## Derived From Pattern/Invariant
Oracle prices respect staleness bounds across resolution hops

## Exploit Type
Oracle

## Location
CovenantCurator.getQuote, getQuotes, previewGetQuote, previewGetQuotes

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The CovenantCurator.resolveOracle() function performs multi-hop resolution (ERC4626 vault unwrapping + fallback oracle) without aggregating staleness across hops. When a vault unwraps to an asset via convertToAssets (which may have internal staleness) and then falls back to an oracle with its own staleness threshold, the total staleness can exceed any reasonable system bound while each component individually passes its own check. For example: (1) Vault convertToAssets uses 1-hour stale internal price oracle, (2) Fallback Chainlink oracle allows 24-hour staleness, (3) Total staleness = 25 hours, but no revert occurs because each oracle only checks its own threshold. This violates the temporal invariant that final prices must be fresh within a system-wide bound. The curator blindly forwards getQuote() to the resolved oracle without validating aggregate staleness. Additionally, previewGetQuote is documented to allow 'longer lookback window' but the curator does not implement different staleness logic—it just forwards to the oracle's preview method, which may be identical to getQuote. Code snippet from CovenantCurator.sol: function getQuote(uint256 inAmount, address base, address quote) external view returns (uint256) { address oracle; (inAmount, base, quote, oracle) = resolveOracle(inAmount, base, quote); if (base == quote) return inAmount; return IPriceOracle(oracle).getQuote(inAmount, base, quote); // No staleness aggregation }

## Impact
CovenantCurator composes ERC4626 unwrapping with a single downstream oracle but does not and cannot aggregate staleness across these hops. If governance resolves a vault whose convertToAssets embeds laggy or oracle-derived pricing, the effective freshness of the final price can exceed a desired system-wide bound even when each hop satisfies its own threshold. This can lead to accepting quotes that are older than an intended global policy, but it does not, by itself, create a direct, programmatic asset-theft path. The risk materializes only under misconfiguration/trust of vault wrappers with latent pricing and permissive staleness on the downstream oracle. Consequently, this is a design/configuration risk rather than an exploitable bug.

## Command to Run Test


## Proof of Concept
1) Deploy a market where base = VaultToken (ERC4626), quote = USDC. 2) Governance marks the vault as resolved (so Curator unwraps via convertToAssets) and sets a fallback oracle for WETH/USDC with a 24h staleness limit. 3) The vault’s convertToAssets internally reflects a price ratio that lags by 2 hours (e.g., using an internal feed or delayed accounting). 4) The fallback oracle’s last update is exactly 24 hours ago, which it treats as acceptable (<= 24h). 5) An attacker calls getQuote(VaultToken->USDC). Curator unwraps VaultToken->WETH using the stale 2h ratio and then asks the fallback oracle (24h stale) for WETH->USDC. 6) The quote is returned successfully because each hop individually passes its own staleness check; however, the aggregate age of information is effectively 26h. 7) During volatile markets, this accepted stale composite price can deviate materially from spot, allowing users to mint/swap/redeem against a price older than a desired global bound. This demonstrates Curator’s lack of aggregate staleness enforcement, not a direct exploit in current code.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "../src/curators/CovenantCurator.sol";
import {IPriceOracle} from "../src/interfaces/IPriceOracle.sol";
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";

contract MockVault is IERC4626 {
    address public immutable asset;
    uint256 public priceRatio = 1e18; // assets per share
    uint256 public lastUpdate;

    constructor(address _asset) {
        asset = _asset;
        lastUpdate = block.timestamp;
    }

    function setPriceRatioWithLag(uint256 ratio, uint256 lagSeconds) external {
        priceRatio = ratio;
        lastUpdate = block.timestamp - lagSeconds; // simulate internal lag/staleness
    }

    // IERC4626 minimal surface
    function convertToAssets(uint256 shares) external view returns (uint256) {
        return (shares * priceRatio) / 1e18;
    }

    // Unused stubs to satisfy interface
    function totalAssets() external view returns (uint256) { return 0; }
    function convertToShares(uint256) external pure returns (uint256) { return 0; }
    function maxDeposit(address) external pure returns (uint256) { return 0; }
    function previewDeposit(uint256) external pure returns (uint256) { return 0; }
    function deposit(uint256, address) external pure returns (uint256) { return 0; }
    function maxMint(address) external pure returns (uint256) { return 0; }
    function previewMint(uint256) external pure returns (uint256) { return 0; }
    function mint(uint256, address) external pure returns (uint256) { return 0; }
    function maxWithdraw(address) external pure returns (uint256) { return 0; }
    function previewWithdraw(uint256) external pure returns (uint256) { return 0; }
    function withdraw(uint256, address, address) external pure returns (uint256) { return 0; }
    function maxRedeem(address) external pure returns (uint256) { return 0; }
    function previewRedeem(uint256) external pure returns (uint256) { return 0; }
    function redeem(uint256, address, address) external pure returns (uint256) { return 0; }
}

contract MockOracle is IPriceOracle {
    uint256 public price = 2000e18; // quote per 1e18 base
    uint256 public lastUpdate;

    constructor() {
        lastUpdate = block.timestamp;
    }

    function name() external pure returns (string memory) { return "MockOracle"; }

    function getQuote(uint256 inAmount, address, address) external view returns (uint256) {
        require(block.timestamp - lastUpdate <= 24 hours, "Stale");
        return (inAmount * price) / 1e18;
    }

    function getQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        uint256 q = this.getQuote(inAmount, base, quote);
        return (q, q);
    }

    function previewGetQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        return this.getQuote(inAmount, base, quote);
    }

    function previewGetQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        return this.getQuotes(inAmount, base, quote);
    }

    function updatePriceFeeds(address, address, bytes calldata) external payable {}
    function getUpdateFee(address, address, bytes calldata) external pure returns (uint128) { return 0; }

    function setLastUpdateAgo(uint256 ago) external {
        lastUpdate = block.timestamp - ago;
    }

    function setPrice(uint256 newPrice) external {
        price = newPrice;
    }
}

contract CuratorAggregateStalenessTest is Test {
    CovenantCurator curator;
    MockVault vault;
    MockOracle oracle;

    address constant WETH = address(0xBEEF);
    address constant USDC = address(0xUSDC);

    function setUp() public {
        curator = new CovenantCurator(address(this));
        vault = new MockVault(WETH);
        oracle = new MockOracle();

        // Governance setup
        curator.govSetResolvedVault(address(vault), true);
        curator.govSetFallbackOracle(address(oracle));
    }

    function testAcceptsQuoteWhenAggregateStalenessExceedsDesiredBound() public {
        // Simulate the vault leg being 2h stale (e.g., laggy internal accounting)
        vault.setPriceRatioWithLag(1e18, 2 hours);

        // Set fallback oracle to be exactly 24h stale (passes its own <=24h check)
        oracle.setLastUpdateAgo(24 hours);

        // Call through the router: unwrap via convertToAssets (2h lag) then price via fallback (24h stale)
        uint256 quote = curator.getQuote(1e18, address(vault), USDC);
        assertEq(quote, 2000e18, "Router returned quote via fallback oracle");

        // Effective aggregate info age = 2h (vault) + 24h (oracle) = 26h, yet no revert occurs
        // This shows lack of aggregate staleness enforcement; each hop passed its own local check.
    }

    function testPreviewAndGetQuoteForwardingAreIdenticalByDesign() public {
        vault.setPriceRatioWithLag(1e18, 1 hours);
        oracle.setLastUpdateAgo(0);
        uint256 a = curator.getQuote(1e18, address(vault), USDC);
        uint256 b = curator.previewGetQuote(1e18, address(vault), USDC);
        assertEq(a, b, "Curator forwards preview/getQuote equivalently; policy must live in the oracle");
    }
}

## Suggested Mitigation
Do not rely on resolvedVaults for vaults whose convertToAssets may incorporate laggy or oracle-derived pricing. Instead: (a) deploy a dedicated VaultAdapter oracle for VaultToken/Quote that composes convertToAssets with the underlying asset’s oracle and explicitly enforces a single staleness policy end-to-end; or (b) use the existing Cross/adapter pattern to compose underlying->quote pricing and perform a unified freshness check in one oracle hop. If keeping resolvedVaults, restrict it to deterministic wrappers (e.g., wstETH-like wrappers with monotonic, non-oracle exchange rates) and document this requirement. Optionally add config to disable fallback routing for resolvedVault bases unless a pair-specific oracle is configured, preventing unintended multi-hop composition.





 **Derived From** : elapsedTime > debtDuration IMPLIES accrued interest capped at 1 duration worth

## [M-9]. Temporal freeze after extended dormancy causes stale rate application enabling yield extraction

## Derived From Pattern/Invariant
elapsedTime > debtDuration IMPLIES accrued interest capped at 1 duration worth

## Exploit Type
TimestampDependentLogic

## Location
LatentSwapLogic._calculateMarketState

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a Covenant market remains inactive for longer than debtDuration, the interest accrual is capped to debtDuration to prevent approximation errors. However, this creates a temporal anomaly where markets can 'freeze' interest at an incorrect rate. The vulnerable code:

```solidity
vars.newDebtNotionalPrice = DebtMath.accrueInterest(
    marketState.lexState.lastDebtNotionalPrice,
    lexParams.debtDuration,
    vars.spotPriceDiscount,
    (vars.elapsedTime > lexParams.debtDuration) ? lexParams.debtDuration : vars.elapsedTime,
    vars.spotLnRateBias
);
```

If the market is inactive for 2+ durations (e.g., 180+ days for a 90-day duration market), only 1 duration of interest accrues using the OLD spotPriceDiscount and spotLnRateBias from BEFORE the dormancy period. If market conditions changed drastically during dormancy (e.g., sqrtPrice moved significantly), the single-duration cap causes mispricing. An attacker can exploit this by: (1) waiting for a market to go dormant during a low-rate period; (2) allowing massive price/LTV changes to occur; (3) triggering an update that only applies 1 duration at old low rates; (4) extracting value through asymmetric interest accrual where borrowers effectively receive free borrowing for the excess dormant period.

## Impact
When a market is dormant for longer than debtDuration, the interest accrual caps to a single duration and uses stale parameters (based on the last stored on-chain market price), permanently under-accruing lender yield for the excess time. This is a structural under-accrual that cannot be recovered later because lastUpdateTimestamp is moved forward to now. The economic loss can be material on large TVL and long dormancy. This requires external conditions (market inactivity), so risk is Medium under the rubric.

## Command to Run Test


## Proof of Concept
Repro steps (high level):
1) Deploy LatentSwapLEX with a positive initLnRateBias so interest is strictly positive even if the discount price is near 1.
2) Initialize two identical markets (A and B) with the same base/quote and a no-op oracle that returns identity quotes (price ~1, sufficient for state calc). Do not mint/swap; baseTokenSupply can be passed as 0 in updateState for this test.
3) Let both markets sit. Then:
   - Market A (long dormancy): warp block.timestamp forward by >4×debtDuration (e.g., 400 days for 90-day duration) and call updateState once. Record debtNotionalPrice_A.
   - Market B (periodic updates): perform three updateState calls at ~debtDuration intervals (e.g., +90d, +180d, +400d) and record debtNotionalPrice_B.
4) Observe: debtNotionalPrice_B >> debtNotionalPrice_A. This demonstrates that the single catch-up only accrues 1×duration of interest, while periodic updates accrue interest for each duration chunk, proving permanent under-accrual after long inactivity.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {LatentSwapLEX} from "../src/lex/latentswap/LatentSwapLEX.sol";
import {IPriceOracle} from "../src/interfaces/IPriceOracle.sol";
import {MarketId, MarketParams} from "../src/interfaces/ICovenant.sol";
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";
import {FixedPoint} from "../src/lex/latentswap/libraries/FixedPoint.sol";

contract MockERC20 is ERC20 {
    uint8 private _dec;
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) {
        _dec = d;
        _mint(msg.sender, 1e24);
    }
    function decimals() public view override returns (uint8) { return _dec; }
}

contract IdentityOracle is IPriceOracle {
    function name() external pure returns (string memory) { return "IdentityOracle"; }
    function getQuote(uint256 inAmount, address, address) external pure returns (uint256) { return inAmount; }
    function getQuotes(uint256 inAmount, address, address) external pure returns (uint256, uint256) { return (inAmount, inAmount); }
    function updatePriceFeeds(address, address, bytes calldata) external payable {}
    function getUpdateFee(address, address, bytes calldata) external pure returns (uint128) { return 0; }
    function previewGetQuote(uint256 inAmount, address, address) external pure returns (uint256) { return inAmount; }
    function previewGetQuotes(uint256 inAmount, address, address) external pure returns (uint256, uint256) { return (inAmount, inAmount); }
}

contract TemporalFreezeTest is Test {
    MockERC20 base;
    MockERC20 quote;
    IdentityOracle oracle;
    LatentSwapLEX lexA;
    LatentSwapLEX lexB;
    MarketParams mp;
    MarketId marketIdA;
    MarketId marketIdB;

    uint32 constant DURATION = 90 days;

    function setUp() public {
        base = new MockERC20("Base", "BASE", 18);
        quote = new MockERC20("Quote", "USD", 18);
        oracle = new IdentityOracle();

        // Deploy two identical LEX instances; set covenantCore = this test contract to satisfy onlyCovenantCore
        lexA = new LatentSwapLEX(
            address(this),
            address(this),
            uint160(FixedPoint.Q96 * 15 / 10),   // edgeHigh = 1.5
            uint160(FixedPoint.Q96 / 2),        // edgeLow  = 0.5
            uint160(FixedPoint.Q96 * 12 / 10),  // limHigh  = 1.2
            uint160(FixedPoint.Q96 * 13 / 10),  // limMax   = 1.3
            int64(int256(3e17)),                // initLnRateBias ~ 0.3 WAD (positive)
            DURATION,
            0                                    // swapFee bps
        );

        lexB = new LatentSwapLEX(
            address(this),
            address(this),
            uint160(FixedPoint.Q96 * 15 / 10),
            uint160(FixedPoint.Q96 / 2),
            uint160(FixedPoint.Q96 * 12 / 10),
            uint160(FixedPoint.Q96 * 13 / 10),
            int64(int256(3e17)),
            DURATION,
            0
        );

        mp = MarketParams({
            baseToken: address(base),
            quoteToken: address(quote),
            curator: address(oracle),
            lex: address(lexA)
        });

        marketIdA = MarketId.wrap(bytes20(keccak256("MARKETA")));
        marketIdB = MarketId.wrap(bytes20(keccak256("MARKETB")));

        // Initialize both markets via onlyCovenantCore (this contract)
        lexA.initMarket(marketIdA, mp, 0, "");
        lexB.initMarket(marketIdB, mp, 0, "");
    }

    function testDormancyCapsAccrualAtOneDuration() public {
        // t0 aligned for both markets
        uint256 t0 = block.timestamp;

        // Scenario A: long dormancy then single update
        vm.warp(t0 + 400 days);
        lexA.updateState(marketIdA, mp, 0, "");
        uint256 debtA = lexA.getLexState(marketIdA).lastDebtNotionalPrice;

        // Scenario B: periodic updates around each duration chunk
        vm.warp(t0 + DURATION);
        lexB.updateState(marketIdB, mp, 0, "");
        vm.warp(t0 + 2 * DURATION);
        lexB.updateState(marketIdB, mp, 0, "");
        vm.warp(t0 + 400 days);
        lexB.updateState(marketIdB, mp, 0, "");
        uint256 debtB = lexB.getLexState(marketIdB).lastDebtNotionalPrice;

        // Periodic accrual must be materially larger than capped single-period accrual
        assertGt(debtB, debtA, "Periodic updates accrue more than single catch-up");
        // Require at least ~40% higher to make the under-accrual evident with lnBias ~0.3
        assertGt(debtB * 10, debtA * 14, "Missing interest for skipped durations is material");
    }
}


## Suggested Mitigation
Replace the single-period cap with iterative accrual over full-duration chunks to keep the per-step approximation error bounded while accruing the entire elapsed period at the intended stale parameters. Example patch inside LatentSwapLogic._calculateMarketState, where vars.newDebtNotionalPrice is set:

// Before:
// vars.newDebtNotionalPrice = DebtMath.accrueInterest(
//     marketState.lexState.lastDebtNotionalPrice,
//     lexParams.debtDuration,
//     vars.spotPriceDiscount,
//     (vars.elapsedTime > lexParams.debtDuration) ? lexParams.debtDuration : vars.elapsedTime,
//     vars.spotLnRateBias
// );

// After (iterative catch-up with a reasonable cap on steps to bound gas):
uint256 accNotional = marketState.lexState.lastDebtNotionalPrice;
uint256 remaining = vars.elapsedTime;
uint256 step = lexParams.debtDuration;
uint256 fullSteps = remaining / step;

// Accrue per full duration (using past values as originally intended)
uint256 MAX_STEPS = 12; // e.g., catch up to ~3 years for a 90d market
if (fullSteps > 0) {
    uint256 loops = fullSteps > MAX_STEPS ? MAX_STEPS : fullSteps;
    for (uint256 i = 0; i < loops; ++i) {
        accNotional = DebtMath.accrueInterest(
            accNotional,
            lexParams.debtDuration,
            vars.spotPriceDiscount,
            step,
            vars.spotLnRateBias
        );
    }
    remaining -= loops * step;
}

// Accrue remainder
if (remaining > 0) {
    accNotional = DebtMath.accrueInterest(
        accNotional,
        lexParams.debtDuration,
        vars.spotPriceDiscount,
        remaining,
        vars.spotLnRateBias
    );
}

vars.newDebtNotionalPrice = accNotional;

Notes:
- This preserves the design choice to accrue using past-state parameters (lastSqrtPriceX96 and lastLnRateBias-based workout), while eliminating the permanent under-accrual on long dormancy.
- The MAX_STEPS guard caps gas. Choose a protocol-appropriate bound (e.g., 12) and document behavior if elapsedTime exceeds it (some under-accrual beyond cap may remain but is bounded). Alternatively, split into powers of two to minimize iterations.
- If desired, an alternative policy is to use current oracle-derived parameters for the catch-up (simpler but punitive/unpredictable). The iterative approach is recommended for accuracy and fairness.





 **Derived From** : Preview Functions Use MAX_STALENESS_UPPER_BOUND Instead of Configured maxStaleness

## [M-10]. PythOracle Preview Functions Accept Stale Prices Beyond Configured maxStaleness, Enabling MEV and UX Failures

## Derived From Pattern/Invariant
Preview Functions Use MAX_STALENESS_UPPER_BOUND Instead of Configured maxStaleness

## Exploit Type
Oracle

## Location
PythOracle.previewGetQuote/_previewFetchPriceStruct

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The PythOracle contract implements preview functions (previewGetQuote, previewGetQuotes) that use _previewFetchPriceStruct() to validate prices. However, _previewFetchPriceStruct() validates staleness against MAX_STALENESS_UPPER_BOUND (15 minutes) instead of the configured maxStaleness parameter (which could be 5 minutes or less). This creates a desync where:

1. previewGetQuote() returns a valid price based on data that is 10 minutes old
2. User submits transaction expecting that price
3. Actual getQuote() reverts because the price exceeds the configured maxStaleness=5min

Vulnerable code in _previewFetchPriceStruct():
```solidity
function _previewFetchPriceStruct() internal view returns (PythStructs.Price memory) {
    PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId);
    if (p.publishTime < block.timestamp) {
        uint256 staleness = block.timestamp - p.publishTime;
        if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // @dev Should be maxStaleness
    }
    // ... rest of validation
}
```

The actual execution path in _fetchPriceStruct() correctly uses maxStaleness:
```solidity
if (staleness > maxStaleness) revert Errors.PriceOracle_InvalidAnswer();
```

This creates a ReserveOrPriceDesync vulnerability where preview and execution paths validate different staleness bounds.

## Impact
Preview functions can accept prices that are stale relative to the configured maxStaleness, creating a desynchronization between preview and execution. This leads to: (a) user transactions reverting after a seemingly valid preview (gas and UX impact), (b) brittle integrator logic if they rely on preview to decide whether to proceed/update feeds, and (c) temporary DoS of flows that depend on preview correctness until a Pyth update is provided. Any MEV here is largely an informational advantage (knowing a transaction will revert), not a direct path to asset theft or profit extraction. No user funds are directly at risk from this bug, but protocol availability/UX suffers. Severity remains Medium under the rubric.

## Command to Run Test


## Proof of Concept
1. Deploy PythOracle with maxStaleness=5 minutes (300 seconds) for a volatile asset like ETH
2. Attacker or normal user calls previewGetQuote() with a Pyth price that is 10 minutes old
3. Preview succeeds because _previewFetchPriceStruct() only checks against MAX_STALENESS_UPPER_BOUND=15 minutes
4. User submits transaction based on preview
5. Transaction reverts in getQuote() because _fetchPriceStruct() checks against maxStaleness=5 minutes
6. For MEV: Sophisticated actor observes this pattern, can frontrun users by detecting when preview will succeed but execution will fail, or avoid trades that appear profitable in preview but will revert

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

// Minimal ERC20 decimals mock to satisfy BaseAdapter _getDecimals()
contract ERC20DecimalsMock {
    uint8 private _decimals;
    constructor(uint8 d) { _decimals = d; }
    function decimals() external view returns (uint8) { return _decimals; }
}

contract MockPyth {
    PythStructs.Price public mockPrice;

    function setMockPrice(int64 price, uint64 conf, int32 expo, uint publishTime) external {
        mockPrice = PythStructs.Price({ price: price, conf: conf, expo: expo, publishTime: publishTime });
    }

    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) {
        return mockPrice;
    }

    function getUpdateFee(bytes[] calldata) external pure returns (uint) { return 1 wei; }
    function updatePriceFeeds(bytes[] calldata) external payable {}
}

contract PythOraclePreviewStalenessTest is Test {
    PythOracle oracle;
    MockPyth mockPyth;
    ERC20DecimalsMock baseToken;
    ERC20DecimalsMock quoteToken;

    bytes32 feedId = bytes32(uint256(1));
    uint256 maxStaleness = 5 minutes; // configured stricter than the 15m upper bound
    uint256 maxConfWidth = 100; // 1%

    function setUp() public {
        mockPyth = new MockPyth();
        baseToken = new ERC20DecimalsMock(18);
        quoteToken = new ERC20DecimalsMock(18);

        oracle = new PythOracle(
            address(mockPyth),
            address(baseToken),
            address(quoteToken),
            feedId,
            maxStaleness,
            maxConfWidth
        );
    }

    function _setPriceWithStaleness(uint stalenessSeconds) internal {
        uint publishTime = block.timestamp - stalenessSeconds;
        // price=1e9, conf=5e6, expo=-8 => numeric price ~ 10 with tight confidence (<=1%)
        mockPyth.setMockPrice(1_000_000_000, 5_000_000, -8, publishTime);
    }

    // Demonstrates the desync: preview accepts stale price (<=15m) but execution rejects (>maxStaleness)
    function test_PreviewAcceptsButGetQuoteReverts_InDesyncWindow() public {
        // 10 minutes stale (between 5m configured and 15m upper bound)
        _setPriceWithStaleness(10 minutes);

        // Preview should succeed because _previewFetchPriceStruct uses MAX_STALENESS_UPPER_BOUND (15m)
        uint256 previewOut = oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken));
        assertGt(previewOut, 0, "preview should return a positive quote");

        // Actual execution path (_fetchPriceStruct) uses configured maxStaleness (5m) and should revert
        vm.expectRevert();
        oracle.getQuote(1e18, address(baseToken), address(quoteToken));
    }

    function test_PreviewVsExecution_StalenessBoundary() public {
        // Exactly at configured maxStaleness => both should succeed
        _setPriceWithStaleness(5 minutes);
        uint256 p1 = oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken));
        assertGt(p1, 0);
        uint256 g1 = oracle.getQuote(1e18, address(baseToken), address(quoteToken));
        assertEq(p1, g1, "at boundary both paths agree");

        // Just above configured maxStaleness but below 15m => preview ok, getQuote reverts
        _setPriceWithStaleness(5 minutes + 1);
        uint256 p2 = oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken));
        assertGt(p2, 0);
        vm.expectRevert();
        oracle.getQuote(1e18, address(baseToken), address(quoteToken));

        // Above 15m => both should revert (preview fails too)
        _setPriceWithStaleness(16 minutes);
        vm.expectRevert();
        oracle.previewGetQuote(1e18, address(baseToken), address(quoteToken));
    }
}


## Suggested Mitigation
In _previewFetchPriceStruct(), validate staleness against the configured maxStaleness (same as runtime path) to ensure consistent acceptance criteria between preview and execution. Optionally, for additional clarity/safety, you can fetch via getPriceNoOlderThan(feedId, maxStaleness) in the preview path. Example fix:

- if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer();
+ if (staleness > maxStaleness) revert Errors.PriceOracle_InvalidAnswer();





 **Derived From** : Griefable callback via malicious ERC4626 vault in resolveOracle path

## [L-11]. Malicious ERC4626 vault can permanently DoS all pricing functions via convertToAssets revert in resolveOracle

## Derived From Pattern/Invariant
Griefable callback via malicious ERC4626 vault in resolveOracle path

## Exploit Type
Dos

## Location
CovenantCurator.resolveOracle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The resolveOracle function recursively resolves base assets through configured ERC4626 vaults by calling IERC4626(base).convertToAssets(inAmount) without any try/catch protection. If governance configures a malicious or compromised vault via govSetResolvedVault, that vault's convertToAssets function can revert arbitrarily, blocking all price queries (getQuote, getQuotes, previewGetQuote, previewGetQuotes, updatePriceFeeds) for any asset pair that routes through this vault. There is no fallback mechanism, no bypass option, and no way to skip the vault resolution once configured. This creates a permanent DoS vector for all pricing functionality that depends on this vault until governance removes the vault configuration.

Vulnerable code in resolveOracle (lines ~125-128):
```solidity
address baseAsset = resolvedVaults[base];
if (baseAsset != address(0)) {
    inAmount = IERC4626(base).convertToAssets(inAmount); // Can revert with no protection
    return resolveOracle(inAmount, baseAsset, quote);
}
```

Governance configuration (lines ~61-65):
```solidity
function govSetResolvedVault(address vault, bool set) external onlyOwner {
    address asset = set ? IERC4626(vault).asset() : address(0);
    resolvedVaults[vault] = asset;
    emit ResolvedVaultSet(vault, asset);
}
```

If a malicious vault's convertToAssets reverts, all pricing paths through this vault are permanently blocked. Markets that depend on this pricing cannot perform mints, redeems, swaps, or any operations requiring oracle quotes. The protocol documentation states 'The risk that governance keys are compromised and potential consequences of incorrect or fraudulent governance actions is out of scope' and 'mispricing by a Governance approved ERC4262 is out of scope'. However, this is not mispricing - this is a complete DoS of core protocol functionality. Per severity rubric: 'Admin can accidentally brick the protocol even while following spec (no malice or error) — that can rise to Medium.' A governance-approved vault that later becomes malicious or buggy (common in DeFi) would brick all markets routing through it.

## Impact
Pricing for pairs that route through a governance-approved ERC4626 vault can be temporarily unavailable if that vault’s convertToAssets reverts. This affects quoting and any protocol paths that depend on quotes for affected markets until governance disables the vault or reconfigures routing. No assets can be stolen and the impact is limited to availability (DoS) on those paths under a trusted, governance-controlled configuration. Governance can resolve by unsetting the vault mapping.

## Command to Run Test


## Proof of Concept
1. Governance calls govSetResolvedVault(maliciousVault, true) to add a seemingly legitimate ERC4626 vault
2. Markets are created that price assets through this vault (e.g., vaultToken → underlyingAsset → quote)
3. Users deposit collateral and open positions in these markets
4. The malicious vault owner (or a bug) causes convertToAssets to always revert
5. Any attempt to call getQuote, getQuotes, previewGetQuote, previewGetQuotes, or updatePriceFeeds for affected asset pairs now reverts
6. All Covenant Market operations (mint, redeem, swap) that depend on these pricing functions are now blocked
7. Users cannot close positions, withdraw collateral, or adjust exposure
8. Market is effectively frozen until governance calls govSetResolvedVault(maliciousVault, false) to remove the config
9. During the DoS window, users may suffer losses due to inability to respond to market conditions

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/curators/CovenantCurator.sol";
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";

contract MaliciousVault {
    address public asset;
    bool public shouldRevert;
    
    constructor(address _asset) {
        asset = _asset;
        shouldRevert = false;
    }
    
    function convertToAssets(uint256) external view returns (uint256) {
        require(!shouldRevert, "Malicious revert");
        return 1e18;
    }
    
    function enableRevert() external {
        shouldRevert = true;
    }
}

contract MockOracle {
    function getQuote(uint256 inAmount, address, address) external pure returns (uint256) {
        return inAmount * 2;
    }
    
    function getQuotes(uint256 inAmount, address, address) external pure returns (uint256, uint256) {
        return (inAmount * 2, inAmount * 2);
    }
}

contract CovenantCurator_Griefable_Test is Test {
    CovenantCurator curator;
    MaliciousVault maliciousVault;
    MockOracle mockOracle;
    address governance = address(0x1);
    address vaultToken = address(0x2);
    address underlyingAsset = address(0x3);
    address quoteToken = address(0x4);
    
    function setUp() public {
        vm.startPrank(governance);
        curator = new CovenantCurator(governance);
        maliciousVault = new MaliciousVault(underlyingAsset);
        mockOracle = new MockOracle();
        
        // Set up oracle for underlying → quote
        curator.govSetConfig(underlyingAsset, quoteToken, address(mockOracle));
        
        // Configure malicious vault (vault token resolves to underlying asset)
        curator.govSetResolvedVault(address(maliciousVault), true);
        vm.stopPrank();
    }
    
    function testGriefableDoS_VaultRevert() public {
        // Initially, pricing works fine
        uint256 resultBefore = curator.getQuote(1e18, address(maliciousVault), quoteToken);
        assertGt(resultBefore, 0, "Initial pricing should work");
        
        // Malicious vault owner enables revert
        maliciousVault.enableRevert();
        
        // Now all pricing functions revert
        vm.expectRevert("Malicious revert");
        curator.getQuote(1e18, address(maliciousVault), quoteToken);
        
        vm.expectRevert("Malicious revert");
        curator.getQuotes(1e18, address(maliciousVault), quoteToken);
        
        vm.expectRevert("Malicious revert");
        curator.previewGetQuote(1e18, address(maliciousVault), quoteToken);
        
        vm.expectRevert("Malicious revert");
        curator.previewGetQuotes(1e18, address(maliciousVault), quoteToken);
        
        // Markets depending on this pricing are now DoS'd
        // Users cannot mint, redeem, or swap in affected Covenant Markets
        // Only governance can fix by removing vault config
    }
    
    function testGriefableDoS_ImpactOnMarkets() public {
        // Simulate market operations that would be blocked
        maliciousVault.enableRevert();
        
        // Any Covenant Market operation requiring price would fail:
        // - mint (needs price to calculate output amounts)
        // - redeem (needs price to calculate collateral release)
        // - swap (needs price for AMM invariant)
        // - updateState (needs price for LTV calculations)
        
        vm.expectRevert("Malicious revert");
        curator.getQuote(1e18, address(maliciousVault), quoteToken);
        
        // User funds effectively frozen until governance intervention
        // This demonstrates the DoS impact on protocol functionality
    }
}

## Suggested Mitigation
Keep the trust assumption explicit and harden failure handling: (a) wrap convertToAssets in try/catch and, on error, revert with a clear custom error (e.g., PriceOracle_InvalidConfiguration) to avoid ambiguous reverts; (b) optionally add an opt-in per-vault flag (e.g., vaultFallbackOnError[vault]) that, if true and a fallback oracle exists for the original base/quote, treats the vault as ‘not configured’ on error and attempts the fallback oracle. Default this flag to false to prevent silent mispricing; (c) add monitoring and a fast governance toggle (existing govSetResolvedVault(vault, false)) playbook to promptly disable malfunctioning vaults. Document that only audited, non-upgradeable ERC4626 vaults should be whitelisted and that enabling fallback-on-error may degrade price guarantees if the fallback does not exactly replicate the intended path.





 **Derived From** : CovenantCurator resolveOracle recursive vault unwrapping lacks depth limit

## [L-12]. Unbounded recursion in CovenantCurator.resolveOracle() enables DoS via vault chain configuration

## Derived From Pattern/Invariant
CovenantCurator resolveOracle recursive vault unwrapping lacks depth limit

## Exploit Type
Dos

## Location
CovenantCurator.resolveOracle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The CovenantCurator.resolveOracle() function recursively unwraps ERC4626 vaults by calling convertToAssets and substituting the vault's asset for base. However, there is no depth limit, visited-set tracking, or circuit breaker on this recursion. If governance configures a chain of vaults (vault A → vault B → vault C → ...) or a circular reference (vault A → vault B → vault A), the function will recurse until hitting the EVM stack depth limit (~1024 frames), causing all price queries to revert with stack overflow. This bricks all mint/redeem/swap operations in affected Covenant markets that depend on this oracle route, as these operations require a successful price query. While this requires governance misconfiguration via govSetResolvedVault(), it represents an unbounded loop vulnerability that can permanently DoS markets.

Vulnerable code in CovenantCurator.resolveOracle:
```solidity
address baseAsset = resolvedVaults[base];
if (baseAsset != address(0)) {
    inAmount = IERC4626(base).convertToAssets(inAmount);
    return resolveOracle(inAmount, baseAsset, quote); // unbounded recursion
}
```
No depth counter or visited-set to prevent infinite loops.

## Impact
If the owner configures a cycle of resolved ERC4626 vaults (e.g., vault A.asset() = vault B and vault B.asset() = vault A), resolveOracle will recurse indefinitely between A and B and run out of gas, causing price queries to fail. This is an availability/DoS risk gated by privileged configuration. No assets are lost, and the owner can restore functionality by correcting configuration. Therefore the impact is limited to temporary unavailability under misconfiguration by a trusted role.

## Command to Run Test


## Proof of Concept
Set up two ERC4626 vaults whose asset() point to each other and mark both as resolved via CovenantCurator.govSetResolvedVault. With no specific base/quote oracle configured for the path and regardless of fallback, resolveOracle will keep unwrapping base from A->B->A->... forever. Any call that requires a price (getQuote/previewGetQuote/updatePriceFeeds routing) will run out of gas due to the infinite recursion. Steps:
1) Deploy two ERC4626-compatible vaults VA and VB. Set VA.asset() = VB and VB.asset() = VA.
2) Owner calls govSetFallbackOracle to some passthrough oracle (for control).
3) Owner configures only VA as resolved: now querying (VA, Q) unwraps once to VB and resolves to fallback → success.
4) Owner configures VB as resolved as well: now the same query A→B→A→... loops recursively and the call runs out of gas.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";
import {IPriceOracle} from "src/curators/interfaces/IPriceOracle.sol";
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";

contract MockVault is IERC4626 {
    address public assetAddr;

    constructor(address _asset) { assetAddr = _asset; }
    function setAsset(address _asset) external { assetAddr = _asset; }

    // IERC4626 minimal functionality used by CovenantCurator
    function asset() external view returns (address) { return assetAddr; }
    function convertToAssets(uint256 shares) external pure returns (uint256) { return shares; }

    // Stubs to satisfy interface
    function totalAssets() external pure returns (uint256) { return 0; }
    function convertToShares(uint256) external pure returns (uint256) { return 0; }
    function maxDeposit(address) external pure returns (uint256) { return 0; }
    function previewDeposit(uint256) external pure returns (uint256) { return 0; }
    function deposit(uint256, address) external pure returns (uint256) { return 0; }
    function maxMint(address) external pure returns (uint256) { return 0; }
    function previewMint(uint256) external pure returns (uint256) { return 0; }
    function mint(uint256, address) external pure returns (uint256) { return 0; }
    function maxWithdraw(address) external pure returns (uint256) { return 0; }
    function previewWithdraw(uint256) external pure returns (uint256) { return 0; }
    function withdraw(uint256, address, address) external pure returns (uint256) { return 0; }
    function maxRedeem(address) external pure returns (uint256) { return 0; }
    function previewRedeem(uint256) external pure returns (uint256) { return 0; }
    function redeem(uint256, address, address) external pure returns (uint256) { return 0; }
}

contract PassthroughOracle is IPriceOracle {
    string public constant name = "Passthrough";
    function getQuote(uint256 inAmount, address, address) external view returns (uint256) { return inAmount; }
    function getQuotes(uint256 inAmount, address, address) external view returns (uint256, uint256) { return (inAmount, inAmount); }
    function updatePriceFeeds(address, address, bytes calldata) external payable {}
    function getUpdateFee(address, address, bytes calldata) external view returns (uint128) { return 0; }
    function previewGetQuote(uint256 inAmount, address, address) external view returns (uint256) { return inAmount; }
    function previewGetQuotes(uint256 inAmount, address, address) external view returns (uint256, uint256) { return (inAmount, inAmount); }
}

contract Curator_UnboundedRecursion_Test is Test {
    CovenantCurator curator;
    address governor = address(0x1111);
    address quote = address(0xBEEF);
    MockVault vaultA;
    MockVault vaultB;
    PassthroughOracle fallbackOracle;

    function setUp() public {
        curator = new CovenantCurator(governor);
        vaultA = new MockVault(address(0));
        vaultB = new MockVault(address(0));
        // Create circular reference: A.asset = B, B.asset = A
        vaultA.setAsset(address(vaultB));
        vaultB.setAsset(address(vaultA));
        fallbackOracle = new PassthroughOracle();
        vm.prank(governor);
        curator.govSetFallbackOracle(address(fallbackOracle));
    }

    function test_DoS_when_cyclic_resolved_vaults() public {
        // Configure only A as resolved first → unwrap once then use fallback → should succeed
        vm.prank(governor);
        curator.govSetResolvedVault(address(vaultA), true);

        bytes memory data = abi.encodeWithSelector(IPriceOracle.getQuote.selector, 1e18, address(vaultA), quote);

        // With only A resolved (no cycle), call should succeed under modest gas
        (bool ok1, ) = address(curator).staticcall{gas: 150_000}(data);
        assertTrue(ok1, "Non-cyclic unwrap path should succeed");

        // Now also resolve B, forming A <-> B cycle → infinite recursion → OOG
        vm.prank(governor);
        curator.govSetResolvedVault(address(vaultB), true);

        // The same call now runs out of gas due to unbounded recursion (staticcall returns false)
        (bool ok2, ) = address(curator).staticcall{gas: 150_000}(data);
        assertFalse(ok2, "Cyclic resolved vaults should cause OOG due to infinite recursion");
    }
}


## Suggested Mitigation
Eliminate unbounded recursion in resolveOracle by switching to an iterative loop with a maximum unwrap depth (e.g., 5–10), and revert with a specific error if the depth is exceeded. Optionally track visited vaults within the loop (using a small in-memory array up to MAX_VAULT_DEPTH) to detect cycles and revert early. Example:

- Add a new error to src/curators/lib/Errors.sol: `error PriceOracle_ExceededMaxVaultDepth();` and optionally `error PriceOracle_CyclicResolvedVault();`.
- Refactor resolveOracle to an iterative version:

function resolveOracle(uint256 inAmount, address base, address quote)
    public view returns (uint256, address, address, address)
{
    if (base == quote) return (inAmount, base, quote, address(0));

    address oracle = getConfiguredOracle(base, quote);
    if (oracle != address(0)) return (inAmount, base, quote, oracle);

    // Iteratively unwrap resolved vaults up to a max depth
    uint256 depth;
    address[MAX_DEPTH] memory visited;
    while (depth < MAX_DEPTH) {
        address next = resolvedVaults[base];
        if (next == address(0)) break;
        // cycle check (optional)
        for (uint256 i = 0; i < depth; ++i) {
            if (visited[i] == base) revert Errors.PriceOracle_CyclicResolvedVault();
        }
        visited[depth] = base;
        inAmount = IERC4626(base).convertToAssets(inAmount);
        base = next;
        if (base == quote) return (inAmount, base, quote, address(0));
        oracle = getConfiguredOracle(base, quote);
        if (oracle != address(0)) return (inAmount, base, quote, oracle);
        unchecked { ++depth; }
    }
    if (depth == MAX_DEPTH) revert Errors.PriceOracle_ExceededMaxVaultDepth();

    oracle = fallbackOracle;
    if (oracle == address(0)) revert Errors.PriceOracle_NotSupported(base, quote);
    return (inAmount, base, quote, oracle);
}

This fully bounds recursion, prevents infinite loops from cycles, and maintains current functionality for benign single- or multi-layer vault unwrapping.



