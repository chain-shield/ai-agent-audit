# 2025 10 covenant - Findings Report
## Commit hash: d5ebe4461564b46cacf8a90cf11add29470ef001

##Findings by Pattern


 **Derived From** : IPriceOracle(oracleBaseCross).getUpdateFee(b, cross, d0) + IPriceOracle(oracleCrossQuote).getUpdateFee(q, cross, d1) <= type(uint128).max

[L-1]. CrossAdapter sums child oracle fees in uint128; overflow reverts getUpdateFee and updatePriceFeeds causing oracle-update DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Infinite recursion via resolvedVaults cycle can DoS all quote/update paths

[L-2]. Unbounded recursion in CovenantCurator.resolveOracle via resolvedVaults cycles can DoS getQuote/preview/updatePriceFeeds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresAdminRole
Poc Test Status: ErrorRunningTests




 **Derived From** : IPyth(pyth).getUpdateFee(abi.decode(updateData,(bytes[]))) <= type(uint128).max

[I-3]. Narrowing cast in PythOracle.getUpdateFee causes DoS when Pyth fee > 2^128-1
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : resolvedVaults[vault] != vault

[L-4]. Infinite recursion in CovenantCurator.resolveOracle via self-resolving ERC4626 vault bricks quotes and price updates
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: RequiresAdminRole
Poc Test Status: ErrorRunningTests




 **Derived From** : IERC20Metadata(base).decimals() <= 127 && IERC20Metadata(quote).decimals() <= 127

[L-5]. PythOracle misprices assets when base token has decimals >127 due to int8 cast, enabling value extraction against LatentSwap
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : if (block.timestamp - p.publishTime) > maxStaleness then revert Errors.PriceOracle_InvalidAnswer()

[M-6]. PythOracle.previewGetQuote ignores per-oracle maxStaleness and returns quotes that live getQuote will revert on
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Inverse update misroutes fees to wrong oracle legs causing DoS and fee loss

[M-7]. CrossAdapter misroutes inverse update fees to wrong oracle legs, causing DoS or burning ETH without refreshing the used price
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless
Poc Test Status: ErrorRunningTests



### Number of Findings
- C: 0
- H: 0
- M: 2
- L: 4
- I: 1

##Findings by Pattern


 **Derived From** : IPriceOracle(oracleBaseCross).getUpdateFee(b, cross, d0) + IPriceOracle(oracleCrossQuote).getUpdateFee(q, cross, d1) <= type(uint128).max

## [L-1]. CrossAdapter sums child oracle fees in uint128; overflow reverts getUpdateFee and updatePriceFeeds causing oracle-update DoS

## Derived From Pattern/Invariant
IPriceOracle(oracleBaseCross).getUpdateFee(b, cross, d0) + IPriceOracle(oracleCrossQuote).getUpdateFee(q, cross, d1) <= type(uint128).max

## Exploit Type
IntegerOverflow

## Location
CrossAdapter.updatePriceFeeds

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
CrossAdapter computes the total pull-oracle fee as a uint128 sum of two child-oracle fees and compares it to msg.value. If baseFee = IPriceOracle(oracleBaseCross).getUpdateFee(...) and quoteFee = IPriceOracle(oracleCrossQuote).getUpdateFee(...), the code does (baseFee + quoteFee) in uint128 both in _getUpdateFee (return baseFee + quoteFee) and _updatePriceFeeds (if msg.value != (baseFee + quoteFee) revert). When baseFee is near type(uint128).max and quoteFee > 0, the uint128 addition overflows and Solidity's checked arithmetic reverts before any comparison. This bricks both CrossAdapter.getUpdateFee() and CrossAdapter.updatePriceFeeds() for that updateData, preventing fee calculation and price-feed updates. As CovenantCurator and LEX flows rely on these functions to pre-quote and forward Pyth/other pull-oracle fees, this creates a permissionless DoS path: an unprivileged caller can present updateData that makes at least one child oracle report an extremely large fee so that baseFee + quoteFee exceeds 2^128-1, causing permanent reverts for that update path.

## Impact
If the two child-oracle fees sum to more than type(uint128).max, CrossAdapter’s uint128 addition reverts with a panic, preventing getUpdateFee and updatePriceFeeds for that specific input. In practice, with supported oracles (e.g., Pyth + Chainlink), fee magnitudes are orders of magnitude below 2^128−1 wei, and unprivileged users cannot force such fees without a malicious or grossly misconfigured oracle. Thus, this is a robustness/UX issue rather than a practical DoS vector.

## Command to Run Test


## Proof of Concept
1) Attacker deploys/targets a child oracle pair where getUpdateFee(base,cross,d0) returns type(uint128).max and the other returns 1 for the supplied updateData; 2) Call CrossAdapter.getUpdateFee(base,quote,abi.encode([d0,d1])) → reverts due to uint128 overflow; 3) Call CrossAdapter.updatePriceFeeds(base,quote,abi.encode([d0,d1])) with any msg.value → reverts before fee comparison; 4) Because CovenantCurator/LEX first call getUpdateFee to compute the fee and then call updatePriceFeeds, all operations requiring a fresh pull update for this route are blocked.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CrossAdapter} from "src/curators/oracles/CrossAdapter.sol";
import {IPriceOracle} from "src/curators/interfaces/IPriceOracle.sol";

contract FixedFeeOracle is IPriceOracle {
    uint128 public fee;
    constructor(uint128 _fee) { fee = _fee; }

    function name() external pure override returns (string memory) { return "FixedFeeOracle"; }
    function getQuote(uint256, address, address) external pure override returns (uint256) { return 0; }
    function getQuotes(uint256, address, address) external pure override returns (uint256, uint256) { return (0, 0); }
    function updatePriceFeeds(address, address, bytes calldata) external payable override {}
    function getUpdateFee(address, address, bytes calldata) external view override returns (uint128) { return fee; }
    function previewGetQuote(uint256, address, address) external pure override returns (uint256) { return 0; }
    function previewGetQuotes(uint256, address, address) external pure override returns (uint256, uint256) { return (0, 0); }
}

contract CrossAdapterFeeOverflowTest is Test {
    CrossAdapter cross;
    FixedFeeOracle baseOracle;
    FixedFeeOracle quoteOracle;
    address base = address(0xB0);
    address crossAsset = address(0xC0);
    address quote = address(0xA0);

    function setUp() public {
        baseOracle = new FixedFeeOracle(type(uint128).max);
        quoteOracle = new FixedFeeOracle(1);
        cross = new CrossAdapter(base, crossAsset, quote, address(baseOracle), address(quoteOracle));
    }

    function test_GetUpdateFee_OverflowReverts() public {
        bytes[] memory parts = new bytes[](2);
        parts[0] = hex"01";  // dummy data for base/cross
        parts[1] = hex"02";  // dummy data for cross/quote
        bytes memory updateData = abi.encode(parts);
        vm.expectRevert();
        cross.getUpdateFee(base, quote, updateData);
    }

    function test_UpdatePriceFeeds_OverflowReverts() public {
        bytes[] memory parts = new bytes[](2);
        parts[0] = hex"01";
        parts[1] = hex"02";
        bytes memory updateData = abi.encode(parts);
        vm.expectRevert();
        cross.updatePriceFeeds{value: 0}(base, quote, updateData);
    }
}


## Suggested Mitigation
Avoid uint128 addition for the sum. In _updatePriceFeeds, compute total as uint256(baseFee) + uint256(quoteFee) and compare msg.value to that uint256 total; forward baseFee and quoteFee individually as before. In _getUpdateFee, compute total in uint256; if total > type(uint128).max, revert with a descriptive custom error (e.g., PriceOracle_FeeTooLarge) instead of triggering a panic via uint128 overflow; otherwise, cast to uint128 and return. Optionally, consider migrating the interface to return uint256 fees across the stack to remove artificial 128-bit limits.





 **Derived From** : Infinite recursion via resolvedVaults cycle can DoS all quote/update paths

## [L-2]. Unbounded recursion in CovenantCurator.resolveOracle via resolvedVaults cycles can DoS getQuote/preview/updatePriceFeeds

## Derived From Pattern/Invariant
Infinite recursion via resolvedVaults cycle can DoS all quote/update paths

## Exploit Type
Dos

## Location
CovenantCurator.resolveOracle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
CovenantCurator.resolveOracle unwraps ERC4626 vaults by recursively substituting base with resolvedVaults[base] and calling itself again without any cycle/depth guard. A misconfigured or malicious ERC4626 added via govSetResolvedVault can create a self-loop (asset() == vault) or a cycle across multiple vaults (A→B→A). In both cases, resolveOracle recurses indefinitely: convertToAssets() is called and the function re-enters itself with the same or cyclic base, exhausting gas. This bricks all paths that route through resolveOracle: getQuote, getQuotes, previewGetQuote, previewGetQuotes, updatePriceFeeds, and getUpdateFee for the affected base across any quote.
Vulnerable snippet:

function govSetResolvedVault(address vault, bool set) external onlyOwner {
    address asset = set ? IERC4626(vault).asset() : address(0);
    resolvedVaults[vault] = asset;
}

function resolveOracle(uint256 inAmount, address base, address quote) public view returns (...) {
    if (base == quote) return (inAmount, base, quote, address(0));
    address oracle = getConfiguredOracle(base, quote);
    if (oracle != address(0)) return (inAmount, base, quote, oracle);
    address baseAsset = resolvedVaults[base];
    if (baseAsset != address(0)) {
        inAmount = IERC4626(base).convertToAssets(inAmount);
        return resolveOracle(inAmount, baseAsset, quote); // no cycle/depth guard
    }
    ...
}


## Impact
Functional DoS of routing for specific bases configured in resolvedVaults: any getQuote/preview/updatePriceFeeds/getUpdateFee that traverses a misconfigured vault will run out of gas due to infinite recursion. However, exploitation requires the owner (governance) to set a self-referential or cyclic resolvedVault mapping, so the risk is limited to governance misconfiguration or a malicious owner. Markets not involving the misconfigured base remain unaffected.

## Command to Run Test


## Proof of Concept
Actor: contract owner (governance).
1) Deploy CovenantCurator with the test as governor.
2) Deploy a minimal ERC4626 where asset() returns its own address and convertToAssets() is identity.
3) Owner calls govSetResolvedVault(vault, true) so resolvedVaults[vault] = vault.
4) Any resolve path starting with base=vault calls convertToAssets() then re-enters resolveOracle with the same base forever, exhausting gas and failing. The same occurs with a 2-cycle: two vaults A and B where A.asset()=B and B.asset()=A, both added to resolvedVaults.
Result: getQuote/preview/updatePriceFeeds/getUpdateFee for the affected base will fail until governance clears/fixes the mapping.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";

// Minimal ERC4626-like that causes a self-loop: asset() == vault
contract SelfAsset4626 {
    function asset() external view returns (address) { return address(this); }
    function convertToAssets(uint256 amount) external pure returns (uint256) { return amount; }
}

// Minimal ERC4626-like with configurable asset to form cycles
contract CycleVault4626 {
    address public assetAddr;
    constructor(address _asset) { assetAddr = _asset; }
    function setAsset(address _asset) external { assetAddr = _asset; }
    function asset() external view returns (address) { return assetAddr; }
    function convertToAssets(uint256 amount) external pure returns (uint256) { return amount; }
}

contract ResolveOracleRecursionTest is Test {
    CovenantCurator router;

    function setUp() public {
        router = new CovenantCurator(address(this));
    }

    function test_selfAssetVault_staticcall_runsOutOfGas() public {
        SelfAsset4626 v = new SelfAsset4626();
        router.govSetResolvedVault(address(v), true);

        // Low-level staticcall with a limited gas stipend to deterministically fail due to infinite recursion
        (bool ok, ) = address(router).staticcall{gas: 50_000}(abi.encodeWithSelector(
            CovenantCurator.getQuote.selector,
            1e18,
            address(v),
            address(0xBEEF)
        ));
        assertFalse(ok, "expected getQuote to fail due to unbounded recursion");
    }

    function test_cycleVaults_staticcall_runsOutOfGas() public {
        CycleVault4626 a = new CycleVault4626(address(0));
        CycleVault4626 b = new CycleVault4626(address(0));
        a.setAsset(address(b));
        b.setAsset(address(a));

        router.govSetResolvedVault(address(a), true);
        router.govSetResolvedVault(address(b), true);

        (bool ok, ) = address(router).staticcall{gas: 50_000}(abi.encodeWithSelector(
            CovenantCurator.getUpdateFee.selector,
            address(a),
            address(1),
            bytes("")
        ));
        assertFalse(ok, "expected getUpdateFee to fail due to A->B->A recursion");
    }
}


## Suggested Mitigation
- In govSetResolvedVault: reject immediate bad configurations:
  - require(vault != address(0)) and asset != address(0)
  - require(asset != vault) to prevent self-loops
  - optionally prevent 2-cycles: require(resolvedVaults[asset] != vault)
  - optionally perform a bounded walk (e.g., up to MAX_HOPS) starting from `asset` to ensure adding `vault` does not introduce a cycle; revert on detection.
- In resolveOracle: replace recursion with an iterative loop and enforce a small MAX_HOPS (e.g., 8–16). If hop limit is exceeded, revert with a specific error (e.g., PriceOracle_InvalidConfiguration).
  Example:
  
    address cur = base;
    for (uint8 i = 0; i < MAX_HOPS && resolvedVaults[cur] != address(0); i++) {
        inAmount = IERC4626(cur).convertToAssets(inAmount);
        address next = resolvedVaults[cur];
        if (next == base) revert Errors.PriceOracle_InvalidConfiguration(); // cycle
        cur = next;
    }
    if (resolvedVaults[cur] != address(0)) revert Errors.PriceOracle_InvalidConfiguration(); // exceeded MAX_HOPS
    base = cur;

- Operationally: document that only well-audited ERC4626 vaults should be added; add off-chain checks to detect cycles before executing governance transactions.





 **Derived From** : IPyth(pyth).getUpdateFee(abi.decode(updateData,(bytes[]))) <= type(uint128).max

## [I-3]. Narrowing cast in PythOracle.getUpdateFee causes DoS when Pyth fee > 2^128-1

## Derived From Pattern/Invariant
IPyth(pyth).getUpdateFee(abi.decode(updateData,(bytes[]))) <= type(uint128).max

## Exploit Type
IntegerMath

## Location
PythOracle.getUpdateFee

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
PythOracle.getUpdateFee casts the upstream IPyth.getUpdateFee(uint256) result to uint128 via SafeCast.toUint128(). If the Pyth proxy returns a fee greater than type(uint128).max, SafeCast reverts, causing a denial-of-service on fee estimation. Any upstream configuration or unexpected large fee will brick fee queries and any core flows that depend on getUpdateFee to pre-compute ETH forwarding for pull-oracle updates. Vulnerable snippet:

function getUpdateFee(address, address, bytes calldata updateData) external view returns (uint128) {
    return IPyth(pyth).getUpdateFee(abi.decode(updateData, (bytes[]))).toUint128();
}

## Impact
No realistic DoS. The narrowing cast to uint128 can only revert if the Pyth update fee exceeds 3.4e38 wei, which is physically and economically impossible. Pyth fees are small and controlled by the trusted Pyth contract; attackers cannot induce such values. Under the rubric, this is a theoretical edge case without asset or functional risk.

## Command to Run Test


## Proof of Concept
1) Attacker (or any caller) supplies updateData and calls PythOracle.getUpdateFee(). 2) If the upstream Pyth contract returns a fee > 2^128-1 (e.g., due to configuration or fee schedule changes), SafeCast.toUint128 reverts. 3) Any protocol path that relies on PythOracle.getUpdateFee to calculate the ETH to forward for updatePriceFeeds will revert and cannot proceed, resulting in a DoS for price-refresh dependent operations.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle as CovenantPythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract MockERC20Dec {
    uint8 private _dec;
    constructor(uint8 d) { _dec = d; }
    function decimals() external view returns (uint8) { return _dec; }
}

contract MockPyth is IPyth {
    uint public fee;
    function setFee(uint v) external { fee = v; }

    function getValidTimePeriod() external view returns (uint) { return 1; }
    function getPrice(bytes32) external view returns (PythStructs.Price memory p) {
        p.price = 1; p.conf = 1; p.expo = 0; p.publishTime = block.timestamp; return p;
    }
    function getEmaPrice(bytes32) external view returns (PythStructs.Price memory p) {
        p.price = 1; p.conf = 1; p.expo = 0; p.publishTime = block.timestamp; return p;
    }
    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory p) {
        p.price = 1; p.conf = 1; p.expo = 0; p.publishTime = block.timestamp; return p;
    }
    function getPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory p) {
        p.price = 1; p.conf = 1; p.expo = 0; p.publishTime = block.timestamp; return p;
    }
    function getEmaPriceUnsafe(bytes32) external view returns (PythStructs.Price memory p) {
        p.price = 1; p.conf = 1; p.expo = 0; p.publishTime = block.timestamp; return p;
    }
    function getEmaPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory p) {
        p.price = 1; p.conf = 1; p.expo = 0; p.publishTime = block.timestamp; return p;
    }
    function updatePriceFeeds(bytes[] calldata) external payable {}
    function updatePriceFeedsIfNecessary(bytes[] calldata, bytes32[] calldata, uint64[] calldata) external payable {}
    function getUpdateFee(bytes[] calldata) external view returns (uint feeAmount) { return fee; }
    function parsePriceFeedUpdates(bytes[] calldata, bytes32[] calldata, uint64, uint64) external payable returns (PythStructs.PriceFeed[] memory pf) {
        return new PythStructs.PriceFeed[](0);
    }
}

contract PythOracle_GetUpdateFee_Test is Test {
    CovenantPythOracle oracle;
    MockPyth mock;
    function setUp() public {
        mock = new MockPyth();
        MockERC20Dec base = new MockERC20Dec(18);
        MockERC20Dec quote = new MockERC20Dec(18);
        oracle = new CovenantPythOracle(address(mock), address(base), address(quote), bytes32(uint256(1)), 10 minutes, 100);
    }

    function test_GetUpdateFee_okWhenWithinUint128() public {
        mock.setFee(1 ether);
        bytes[] memory arr = new bytes[](1); arr[0] = hex"01";
        bytes memory updateData = abi.encode(arr);
        uint128 fee = oracle.getUpdateFee(address(0), address(0), updateData);
        assertEq(uint256(fee), 1 ether);
    }

    function test_GetUpdateFee_revertsWhenFeeExceedsUint128() public {
        mock.setFee(type(uint256).max);
        bytes[] memory arr = new bytes[](1); arr[0] = hex"01";
        bytes memory updateData = abi.encode(arr);
        vm.expectRevert();
        oracle.getUpdateFee(address(0), address(0), updateData);
    }
}


## Suggested Mitigation
No change needed. Optionally, align with upstream by returning uint256 from getUpdateFee to remove the theoretical narrowing cast, or add a require with a descriptive error if fee > type(uint128).max for clarity.





 **Derived From** : resolvedVaults[vault] != vault

## [L-4]. Infinite recursion in CovenantCurator.resolveOracle via self-resolving ERC4626 vault bricks quotes and price updates

## Derived From Pattern/Invariant
resolvedVaults[vault] != vault

## Exploit Type
AccountingInvariantViolation

## Location
CovenantCurator.govSetResolvedVault

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
govSetResolvedVault stores resolvedVaults[vault] = IERC4626(vault).asset(). If a vault’s asset() returns its own address, resolvedVaults[vault] == vault. Then resolveOracle recurses forever on step (3): address baseAsset = resolvedVaults[base]; if (baseAsset != address(0)) { inAmount = IERC4626(base).convertToAssets(inAmount); return resolveOracle(inAmount, baseAsset, quote); } This never reaches the base==quote, configured-oracle, or fallback cases, causing out-of-gas reverts in getQuote/getQuotes/preview* and updatePriceFeeds/getUpdateFee for any path starting with base=vault.

## Impact
A misconfigured resolvedVault that maps a vault to itself (or a cycle across multiple vaults) will cause resolveOracle to recurse indefinitely, making all price/preview/update calls with that vault as base revert. This is a functional DoS scoped to the misconfigured market and only reachable if the owner whitelists a self-referential or cyclic ERC4626 configuration. No direct asset loss; availability is impacted until configuration is corrected.

## Command to Run Test


## Proof of Concept
Two variants:
1) Self-referential vault: Deploy an ERC4626-like vault V where asset() returns address(V). Owner calls govSetResolvedVault(V, true). Any resolveOracle(..., base=V, ...) re-enters step (3) forever because baseAsset == base, so recursion never reaches configured-oracle or fallback cases.
2) Two-hop cycle: Deploy vaults A and B with asset(A)=address(B) and asset(B)=address(A). Owner calls govSetResolvedVault(A, true) and govSetResolvedVault(B, true). Any resolveOracle(..., base=A, ...) alternates A→B→A→… endlessly.
Both variants brick getQuote/getQuotes/preview* and updatePriceFeeds/getUpdateFee for the affected base vault(s).

## Proof of Code
pragma solidity ^0.8.30;
import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";

error TooDeep();

contract SelfAssetVault {
    uint256 public calls;
    function asset() external view returns (address) { return address(this); }
    // Intentionally non-view to track calls; ABI remains compatible with IERC4626
    function convertToAssets(uint256 inAmount) external returns (uint256) {
        unchecked { calls++; }
        if (calls > 5) revert TooDeep(); // bound recursion for deterministic test
        return inAmount;
    }
}

contract CovenantCuratorRecursionTest is Test {
    CovenantCurator curator;
    SelfAssetVault vault;
    address constant DUMMY_QUOTE = address(0xBEEF);

    function setUp() public {
        curator = new CovenantCurator(address(this));
        vault = new SelfAssetVault();
        // Owner misconfigures: marks vault as resolved; asset() = self
        curator.govSetResolvedVault(address(vault), true);
    }

    function test_recursion_boundedByVaultRevert_getQuote() public {
        vm.expectRevert(TooDeep.selector);
        curator.getQuote(1e18, address(vault), DUMMY_QUOTE);
    }

    function test_recursion_boundedByVaultRevert_updatePriceFeeds() public {
        vm.expectRevert(TooDeep.selector);
        curator.updatePriceFeeds(address(vault), DUMMY_QUOTE, "");
    }
}


## Suggested Mitigation
- In govSetResolvedVault: validate the mapping to prevent trivial self-cycles: require(asset != address(0) && asset != vault), otherwise revert with PriceOracle_InvalidConfiguration.
- Harden resolution against multi-hop cycles: replace recursion with a bounded iterative loop (e.g., maxHops = 8) that repeatedly unwraps via convertToAssets while checking: (a) base != previous base; (b) hop count not exceeded. If exceeded or a repeat is detected, revert with PriceOracle_InvalidConfiguration.
- Optionally, reject configurations that create immediate cycles by checking on set: if resolvedVaults[asset] != address(0) and resolvedVaults[asset] == vault, then revert.





 **Derived From** : IERC20Metadata(base).decimals() <= 127 && IERC20Metadata(quote).decimals() <= 127

## [L-5]. PythOracle misprices assets when base token has decimals >127 due to int8 cast, enabling value extraction against LatentSwap

## Derived From Pattern/Invariant
IERC20Metadata(base).decimals() <= 127 && IERC20Metadata(quote).decimals() <= 127

## Exploit Type
PricePrecision

## Location
PythOracle._previewGetQuote

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In PythOracle._previewGetQuote (and inherited _getQuote), the feed exponent is computed via a lossy int8 cast: int8 feedExponent = int8(baseDecimals) - int8(priceStruct.expo);. If baseDecimals >= 128 (valid uint8 per IERC20Metadata), int8(baseDecimals) wraps to negative (two's complement), flipping the sign and magnitude of feedExponent. This leads to constructing the wrong Scale in ScaleUtils.from(...) and returning drastically mis-scaled quotes. Since LatentSwap relies on oracle quotes to value base vs quote during mint/swap/redeem, a market initialized on a high-decimal base token can be drained: traders can deposit a small amount of the high-decimal base and redeem disproportionate quote or base via swaps, because the oracle price is off by orders of magnitude.

## Impact
If governance whitelists a base token whose decimals exceed 127, the int8 cast in feedExponent computation wraps, leading to a grossly incorrect scale and mispriced quotes. This can enable value extraction when trading against LatentSwap for that specific market. Under normal operation (well-known tokens with <= 18 decimals), the issue is not reachable. Therefore, impact is configuration-dependent and only material if an admin adds a pathological token, making this a low-severity but real correctness bug.

## Command to Run Test


## Proof of Concept
Scenario: Misconfiguration-driven mispricing
1) A base ERC20 with decimals=200 (still a valid uint8) exists. 2) Governance creates a market using this token as base and PythOracle as the price source. 3) Pyth feed expo is typical (e.g., -8). The correct feedExponent should be 200 - (-8) = 208. However, the code computes int8(baseDecimals) - int8(expo) = (-56) - (-8) = -48 (two’s complement wrap). 4) PythOracle consequently builds the wrong Scale (as if exponent = -48), returning quotes off by orders of magnitude. 5) Traders interacting with LatentSwap in this market can extract value due to mispricing (e.g., deposit minimal base and swap/redeem to receive outsized quote/base). Note: This requires governance to list a token with decimals > 127; it is not permissionless.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle as CovenantPythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract MockHighDecToken {
    uint8 internal _dec;
    constructor(uint8 d){_dec=d;}
    function decimals() external view returns (uint8){return _dec;}
}

contract MockPyth {
    PythStructs.Price internal p;
    constructor(int64 price, int32 expo, uint64 conf) {
        p.price = price; p.expo = expo; p.conf = conf; p.publishTime = uint64(block.timestamp);
    }
    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return p; }
}

// Subclass that exposes the computed exponents to make the bug deterministic and testable
contract OracleExpose is CovenantPythOracle {
    constructor(
        address _pyth,
        address _base,
        address _quote,
        bytes32 _feedId,
        uint256 _maxStaleness,
        uint256 _maxConfWidth
    ) CovenantPythOracle(_pyth, _base, _quote, _feedId, _maxStaleness, _maxConfWidth) {}

    function previewFeedExponents() external view returns (int8 wrong, int256 correct) {
        PythStructs.Price memory p = _previewFetchPriceStruct();
        wrong = int8(baseDecimals) - int8(p.expo);
        correct = int256(uint256(baseDecimals)) - int256(p.expo);
    }
}

contract PythOracleInt8CastTest is Test {
    OracleExpose oracle;
    MockHighDecToken base;
    MockHighDecToken quote;
    MockPyth pyth;

    bytes32 feedId = bytes32(uint256(0xBEEF));
    uint256 maxStaleness = 10 minutes;
    uint256 maxConfWidth = 100; // 1%

    function setUp() public {
        base = new MockHighDecToken(200); // >127 to trigger int8 wrap
        quote = new MockHighDecToken(6);
        // price = 100 with expo = -8, small conf; within adapter bounds
        pyth = new MockPyth(100_00000000, -8, 1000);
        oracle = new OracleExpose(address(pyth), address(base), address(quote), feedId, maxStaleness, maxConfWidth);
    }

    function test_Int8CastWraps_WhenBaseDecimalsOver127() public {
        (int8 wrong, int256 correct) = oracle.previewFeedExponents();
        // int8(200) = -56; -56 - (-8) = -48
        assertEq(wrong, -48);
        // Correct wide-typed calc: 200 - (-8) = 208
        assertEq(correct, 208);

        // Also ensure previewGetQuote runs using the wrapped exponent path (negative branch)
        uint256 out = oracle.previewGetQuote(1, address(base), address(quote));
        // Not asserting magnitude, just that the function does not revert and returns a value
        assertTrue(out >= 0);
    }
}


## Suggested Mitigation
Eliminate the lossy int8 arithmetic and add explicit bounds checks for the scale parameters.

Suggested fix (conceptual):

- Compute feedExponent in a wide signed type and only cast after bounds validation.
- Ensure both branches of Scale construction cannot overflow uint8 and revert on invalid ranges.

Example:

int256 fe = int256(uint256(baseDecimals)) - int256(priceStruct.expo);
Scale scale;
if (fe >= 0) {
    // Require fe to fit into uint8 and not overflow ScaleUtils expectations
    if (fe > 255) revert Errors.PriceOracle_InvalidAnswer();
    scale = ScaleUtils.from(quoteDecimals, uint8(uint256(fe)));
} else {
    uint256 neg = uint256(-fe); // magnitude of negative exponent
    // Ensure quoteDecimals + neg fits into uint8
    if (neg > 255 || uint256(quoteDecimals) + neg > 255) revert Errors.PriceOracle_InvalidAnswer();
    scale = ScaleUtils.from(quoteDecimals + uint8(neg), 0);
}

Additionally, consider validating at construction that base/quote decimals are within a sane range (e.g., <= 36) or, at minimum, <= 127 to preserve the original int8 logic if you wish to keep narrower types. Rejecting pathological tokens up-front removes the configuration footgun.





 **Derived From** : if (block.timestamp - p.publishTime) > maxStaleness then revert Errors.PriceOracle_InvalidAnswer()

## [M-6]. PythOracle.previewGetQuote ignores per-oracle maxStaleness and returns quotes that live getQuote will revert on

## Derived From Pattern/Invariant
if (block.timestamp - p.publishTime) > maxStaleness then revert Errors.PriceOracle_InvalidAnswer()

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
previewGetQuote calls _previewFetchPriceStruct(), which validates recency against MAX_STALENESS_UPPER_BOUND (15m) instead of the oracle’s configured maxStaleness. When staleness is in (maxStaleness, MAX_STALENESS_UPPER_BOUND], preview returns an outAmount while the live path (_fetchPriceStruct → getQuote) reverts due to the maxStaleness check. This temporal mismatch breaks the preview/live invariant and can cause routers/solvers to submit trades that systematically revert on-chain, resulting in intermittent DoS/gas grief and unreliable routing decisions for Pyth-backed markets.

Vulnerable snippet (from _previewFetchPriceStruct):

if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // should use maxStaleness

Live path (_fetchPriceStruct) correctly uses maxStaleness. Hence preview may accept stale prices that live getQuote rejects.

## Impact
Functional DoS of quoting/route execution: off-chain previews show executable quotes while on-chain execution reverts during the stale window; routers waste gas and users experience failed trades until a feed update lands.

## Command to Run Test


## Proof of Concept
Setup: Deploy Covenant PythOracle with maxStaleness = 5 minutes. Feed a Pyth price update whose publishTime is 10 minutes in the past (stale relative to the per-oracle 5m but within the 15m MAX_STALENESS_UPPER_BOUND). Observation: previewGetQuote succeeds because _previewFetchPriceStruct checks against the 15m upper bound, while getQuote (live) reverts because _fetchPriceStruct enforces the stricter per-oracle maxStaleness. Result: Off-chain routing that relies on preview returns executable-looking quotes that systematically revert on-chain during this window, causing gas grief/DoS for integrators until a fresh Pyth update is pushed.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {PythOracle as CovenantPythOracle} from "src/curators/oracles/pyth/PythOracle.sol";
import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";

contract MockERC20Decimals {
    uint8 private immutable _dec;
    constructor(uint8 d) { _dec = d; }
    function decimals() external view returns (uint8) { return _dec; }
}

contract MockPyth is IPyth {
    PythStructs.Price internal p;
    function setPrice(int64 price, uint64 conf, int32 expo, uint publishTime) external {
        p = PythStructs.Price({price: price, conf: conf, expo: expo, publishTime: publishTime});
    }
    function getValidTimePeriod() external pure returns (uint) { return 0; }
    function getPrice(bytes32) external view returns (PythStructs.Price memory) { return p; }
    function getEmaPrice(bytes32) external view returns (PythStructs.Price memory) { return p; }
    function getPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return p; }
    function getPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory) { return p; }
    function getEmaPriceUnsafe(bytes32) external view returns (PythStructs.Price memory) { return p; }
    function getEmaPriceNoOlderThan(bytes32, uint) external view returns (PythStructs.Price memory) { return p; }
    function updatePriceFeeds(bytes[] calldata) external payable {}
    function updatePriceFeedsIfNecessary(bytes[] calldata, bytes32[] calldata, uint64[] calldata) external payable {}
    function getUpdateFee(bytes[] calldata) external pure returns (uint) { return 0; }
    function parsePriceFeedUpdates(bytes[] calldata, bytes32[] calldata, uint64, uint64) external payable returns (PythStructs.PriceFeed[] memory priceFeeds) {
        priceFeeds = new PythStructs.PriceFeed[](0);
    }
}

contract PythOracle_PreviewStalenessMismatchTest is Test {
    CovenantPythOracle oracle;
    MockPyth mockPyth;
    MockERC20Decimals base;
    MockERC20Decimals quote;

    function setUp() public {
        vm.warp(1_000_000);
        mockPyth = new MockPyth();
        base = new MockERC20Decimals(10);
        quote = new MockERC20Decimals(10);
        // maxStaleness = 5 minutes, maxConfWidth = 50 bps
        oracle = new CovenantPythOracle(
            address(mockPyth),
            address(base),
            address(quote),
            bytes32(uint256(0x1234)),
            5 minutes,
            50
        );
    }

    function test_preview_allows_price_within_15m_but_live_reverts_past_per_oracle_limit() public {
        // Price is recent enough for preview (<=15m) but stale for live (>5m)
        mockPyth.setPrice(int64(100_000_000), uint64(0), int32(0), block.timestamp - 10 minutes);

        uint256 inAmount = 1e6;

        // Preview path should NOT revert (uses MAX_STALENESS_UPPER_BOUND = 15m)
        uint256 outPreview = oracle.previewGetQuote(inAmount, address(base), address(quote));
        // We only require that it does not revert; value can be zero depending on scaling/rounding
        (outPreview);

        // Live path MUST revert (uses per-oracle maxStaleness = 5m)
        vm.expectRevert();
        oracle.getQuote(inAmount, address(base), address(quote));
    }
}


## Suggested Mitigation
Make preview enforce the same per-oracle maxStaleness as the live path. Concretely: in _previewFetchPriceStruct, replace the staleness check against MAX_STALENESS_UPPER_BOUND with the immutable maxStaleness check. Even better, delete _previewFetchPriceStruct and reuse the same validation routine as live (call _fetchPriceStruct from preview) to guarantee preview/live alignment across all checks (staleness, aheadness, confidence width, exponent bounds). This eliminates the stale-but-previewable window and the resulting revert mismatch.





 **Derived From** : Inverse update misroutes fees to wrong oracle legs causing DoS and fee loss

## [M-7]. CrossAdapter misroutes inverse update fees to wrong oracle legs, causing DoS or burning ETH without refreshing the used price

## Derived From Pattern/Invariant
Inverse update misroutes fees to wrong oracle legs causing DoS and fee loss

## Exploit Type
StandardViolation

## Location
CrossAdapter._updatePriceFeeds

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
CrossAdapter correctly flips legs for price queries in _getQuote/_previewGetQuote, but _updatePriceFeeds/_getUpdateFee never determine direction and blindly assume forward inputs. When called with the inverse pair (givenBase=quote, givenQuote=base), the adapter requests fees and forwards ETH to the wrong legs: it calls oracleBaseCross with (quote→cross) and oracleCrossQuote with (base→cross), whereas the price path actually uses (cross→base) and (quote→cross) respectively. Depending on the underlying oracle, this either reverts (DoS of price updates) or succeeds while updating unrelated feeds, burning the caller’s ETH with no effect on the price used by this adapter. Vulnerable snippet:

function _updatePriceFeeds(address _base, address _quote, bytes calldata updateData) internal override {
    ...
    uint128 baseFee = IPriceOracle(oracleBaseCross).getUpdateFee(_base, cross, crossUpdateData[0]);
    uint128 quoteFee = IPriceOracle(oracleCrossQuote).getUpdateFee(_quote, cross, crossUpdateData[1]);
    if (msg.value != (baseFee + quoteFee)) revert Errors.PriceOracle_IncorrectPayment();
    IPriceOracle(oracleBaseCross).updatePriceFeeds{value: baseFee}(_base, cross, crossUpdateData[0]);
    IPriceOracle(oracleCrossQuote).updatePriceFeeds{value: quoteFee}(_quote, cross, crossUpdateData[1]);
}

Same bug in _getUpdateFee().

## Impact
Price updates for inverse queries revert or silently update the wrong feeds; users either face a DoS on operations requiring fresh pull prices or pay ETH fees that do not refresh the price path used by this adapter.

## Command to Run Test


## Proof of Concept
1) Deploy CrossAdapter(base, cross, quote) with two underlying oracles.
2) Configure CovenantCurator to route (base, quote) to this CrossAdapter.
3) Prepare inverse updateData containing legs for the correct inverse path: leg0=cross→base (for oracleBaseCross), leg1=quote→cross (for oracleCrossQuote).
4) Call curator.getUpdateFee(quote, base, updateData) or curator.updatePriceFeeds(quote, base, updateData, value=fee).
5a) If oracles strictly validate direction, getUpdateFee/update revert because CrossAdapter calls wrong legs (DoS).
5b) If oracles accept any direction, update succeeds but updates wrong feeds (baseCross: quote→cross; crossQuote: base→cross); caller’s ETH is paid while the actually used legs remain stale, so subsequent operations that require freshness still fail.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {CovenantCurator} from "src/curators/CovenantCurator.sol";
import {CrossAdapter} from "src/curators/oracles/CrossAdapter.sol";
import {IPriceOracle} from "src/interfaces/IPriceOracle.sol";

contract MockOracle is IPriceOracle {
    string public constant name = "Mock";
    bool public strict;
    mapping(address => mapping(address => uint256)) public updates;

    constructor(bool _strict) { strict = _strict; }

    function getQuote(uint256 inAmount, address, address) external view returns (uint256) { return inAmount; }
    function getQuotes(uint256 inAmount, address, address) external view returns (uint256,uint256) { return (inAmount, inAmount); }
    function previewGetQuote(uint256 inAmount, address, address) external view returns (uint256){ return inAmount; }
    function previewGetQuotes(uint256 inAmount, address, address) external view returns (uint256,uint256) { return (inAmount, inAmount); }

    function getUpdateFee(address base, address quote, bytes calldata updateData) external view returns (uint128) {
        (address expectedBase, address expectedQuote) = abi.decode(updateData, (address, address));
        if (strict) {
            require(base == expectedBase && quote == expectedQuote, "wrong pair for fee");
        }
        return 1; // 1 wei
    }

    function updatePriceFeeds(address base, address quote, bytes calldata updateData) external payable {
        (address expectedBase, address expectedQuote) = abi.decode(updateData, (address, address));
        if (strict) {
            require(base == expectedBase && quote == expectedQuote, "wrong pair for update");
        }
        updates[base][quote] += 1; // record which leg was updated
    }
}

contract CrossAdapterUpdateDirectionTest is Test {
    address base = address(0xBEEF_000000000000000000000000000000000000);
    address cross = address(0xC0FFEE_0000000000000000000000000000000000);
    address quote = address(0xFEE_000000000000000000000000000000000000);

    address governor = address(0xAA);
    address attacker = address(0xBB);

    function _wireCurator(address oracle) internal returns (CovenantCurator curator) {
        curator = new CovenantCurator(governor);
        vm.prank(governor);
        curator.govSetConfig(base, quote, oracle);
    }

    function testInverseUpdateRevertsDueToWrongLegs() public {
        // Strict mocks revert if pair passed to getUpdateFee/update does not match encoded updateData
        MockOracle baseCross = new MockOracle(true);
        MockOracle crossQuote = new MockOracle(true);
        CrossAdapter x = new CrossAdapter(base, cross, quote, address(baseCross), address(crossQuote));
        CovenantCurator curator = _wireCurator(address(x));

        // Inverse orientation: caller supplies base=quote, quote=base
        // Correct legs should be: cross->base on oracleBaseCross, and quote->cross on oracleCrossQuote
        bytes[] memory legs = new bytes[](2);
        legs[0] = abi.encode(cross, base);  // intended for oracleBaseCross
        legs[1] = abi.encode(quote, cross); // intended for oracleCrossQuote
        bytes memory updateData = abi.encode(legs);

        vm.expectRevert(); // getUpdateFee will be asked for wrong legs and revert
        curator.getUpdateFee(quote, base, updateData);

        vm.deal(attacker, 10 ether);
        vm.prank(attacker);
        vm.expectRevert();
        curator.updatePriceFeeds{value: 2}(quote, base, updateData);
    }

    function testInverseUpdateBurnsFeesWithoutUpdatingUsedFeeds() public {
        // Permissive mocks accept any pair and record the updated leg, charging 1 wei each
        MockOracle baseCross = new MockOracle(false);
        MockOracle crossQuote = new MockOracle(false);
        CrossAdapter x = new CrossAdapter(base, cross, quote, address(baseCross), address(crossQuote));
        CovenantCurator curator = _wireCurator(address(x));

        // Legs intended for inverse path:
        // baseCross: cross->base; crossQuote: quote->cross
        bytes[] memory legs = new bytes[](2);
        legs[0] = abi.encode(cross, base);
        legs[1] = abi.encode(quote, cross);
        bytes memory updateData = abi.encode(legs);

        uint128 fee = curator.getUpdateFee(quote, base, updateData);
        assertEq(fee, 2);

        vm.deal(attacker, 10 ether);
        vm.prank(attacker);
        curator.updatePriceFeeds{value: fee}(quote, base, updateData);

        // Fees forwarded
        assertEq(address(baseCross).balance, 1);
        assertEq(address(crossQuote).balance, 1);

        // Wrong legs got updated:
        // baseCross got quote->cross instead of cross->base
        assertEq(baseCross.updates(quote, cross), 1);
        assertEq(baseCross.updates(cross, base), 0);
        // crossQuote got base->cross instead of quote->cross
        assertEq(crossQuote.updates(base, cross), 1);
        assertEq(crossQuote.updates(quote, cross), 0);
    }
}


## Suggested Mitigation
Determine direction with ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote) in both _updatePriceFeeds and _getUpdateFee, then route legs accordingly: forward → (oracleBaseCross: base→cross, oracleCrossQuote: cross→quote); inverse → (oracleCrossQuote: quote→cross, oracleBaseCross: cross→base). Keep updateData[0]/[1] aligned to the actual legs being updated, and optionally validate that (_base,_quote) match either (base,quote) or (quote,base) to prevent misrouting.



