# ssv-network Round 4 Canonicalization Input

Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/ssv-network-run-001.md`
Candidate count: `2`

This file contains only findings currently marked `Valid` with reportable severity for the configured validation profile.

## Candidates

## C-5 / `xV5HkuxJmnZDvpMrER57I`
- Finding title: Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB
- Report lines: 401-507

### Original Report Block
```md
## [C-5]. Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB

## id: xV5HkuxJmnZDvpMrER57I

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError / PrecisionDriftAccumulation

## Exploit Type
RoundingError

## Location
SSVClusters.liquidate

## Finding Status: Valid
### Finding Status Justification: The described root cause is present. The ETH balance update path in ClusterLib.updateBalanceWithEB computes network fee and operator fee using separate divisions by BPS_DENOMINATOR, while isLiquidatableWithEB and isLiquidatableWithVUnits add burnRate and networkFee first and divide once. For vUnits not divisible by 10000 and nonzero operator/network fee components, floor(a*x/d)+floor(b*x/d) can be less than floor((a+b)*x/d), so the liquidation threshold can be higher than the fee-consumption amount used to update cluster.balance. SSVClusters.liquidate is permissionless for third parties when the cluster passes isLiquidatableWithEB; it updates operators and cluster balance, then transfers the remaining cluster balance to msg.sender in _executeLiquidation. No complete safeguard canonicalizes the formulas or prevents liquidation inside this rounding window. nonReentrant does not address the arithmetic mismatch, and Solidity 0.8 overflow checks are irrelevant. The path is reachable through the production SSVNetwork.liquidate delegate path and concerns ETH cluster accounting for the in-scope network asset. There is no documentation in the supplied material explicitly accepting premature liquidation from this rounding inconsistency. Exploitation does not require a privileged actor, compromised credentials, or victim-only misuse; it requires a normal public liquidate call against a cluster whose current balance falls in the rounding gap. The issue exists in today's code and does not depend on future integrations or upgrades.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
ETH cluster balance decay floors operator and network fee components separately, but liquidation threshold calculation first adds the rates and then floors once. For vUnits not divisible by BPS_DENOMINATOR, the combined calculation can be larger than the actual balance decay formula, so a cluster can be treated as liquidatable while its balance is still above the separately-accounted threshold. Vulnerable liquidation check:

uint128 rate = burnRate + networkFee;
uint256 thresholdUnits = (uint256(minimumBlocksBeforeLiquidation) * rate * units) / BPS_DENOMINATOR;
uint256 liquidationThreshold = thresholdUnits * ETH_DEDUCTED_DIGITS;
return cluster.balance < liquidationThreshold;

Actual balance update uses separate floors:

uint128 networkFeeUnits = (idxNet * units) / BPS_DENOMINATOR;
uint128 usageUnits = (idxOp * units) / BPS_DENOMINATOR + networkFeeUnits;

A permissionless liquidator can call SSVNetwork.liquidate for a victim ETH cluster in this rounding window and receive the entire remaining cluster balance through _executeLiquidation even though the cluster is not yet liquidatable under the balance-consumption formula.

## Impact
Direct theft of the victim cluster's remaining ETH balance by a third-party liquidator in the rounding window, plus premature deactivation/removal of the validator cluster. Under the Immunefi impact rows this maps to direct theft of user funds, though feasibility depends on finding or timing a cluster near the collateral boundary with non-10000-multiple vUnits.

## Proof of Concept
1. A cluster has explicit effective-balance tracking where vUnits is not divisible by 10000, for example 15001.
2. Both operator fee and network fee are nonzero.
3. The actual balance decay path floors operator and network fee components separately, producing a lower threshold.
4. The liquidation path combines the fee rates before division, producing a higher threshold.
5. While the cluster balance lies between those two thresholds, an attacker calls liquidate(clusterOwner, operatorIds, cluster).
6. The contract accepts the cluster as liquidatable and transfers the full remaining cluster.balance to the attacker.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import "../contracts/libraries/ClusterLib.sol";
import "../contracts/interfaces/ISSVNetworkCore.sol";
import {PackedETH, ETH_DEDUCTED_DIGITS, BPS_DENOMINATOR} from "../contracts/libraries/SSVCoreTypes.sol";

contract ClusterLibRoundingHarness {
    using ClusterLib for ISSVNetworkCore.Cluster;

    function vulnerableLiquidatable(
        uint256 balance,
        uint64 vUnits,
        uint64 burnRate,
        uint64 networkFee
    ) external pure returns (bool) {
        ISSVNetworkCore.Cluster memory cluster = ISSVNetworkCore.Cluster({
            validatorCount: 1,
            networkFeeIndex: 0,
            index: 0,
            active: true,
            balance: balance
        });
        return cluster.isLiquidatableWithVUnits(vUnits, burnRate, networkFee, 1, PackedETH.wrap(0));
    }

    function separatelyFlooredThreshold(
        uint64 vUnits,
        uint64 burnRate,
        uint64 networkFee
    ) external pure returns (uint256) {
        uint256 operatorUnits = (uint256(burnRate) * vUnits) / BPS_DENOMINATOR;
        uint256 networkUnits = (uint256(networkFee) * vUnits) / BPS_DENOMINATOR;
        return (operatorUnits + networkUnits) * ETH_DEDUCTED_DIGITS;
    }
}

contract PrematureLiquidationRoundingTest is Test {
    function testCombinedRoundingMakesClusterLiquidatableEarly() external {
        ClusterLibRoundingHarness h = new ClusterLibRoundingHarness();

        uint64 vUnits = 15_001;
        uint64 operatorFee = 1;
        uint64 networkFee = 1;
        uint256 victimBalance = 250_000;

        uint256 expectedThreshold = h.separatelyFlooredThreshold(vUnits, operatorFee, networkFee);
        assertEq(expectedThreshold, 200_000);
        assertGt(victimBalance, expectedThreshold);
        assertFalse(victimBalance < expectedThreshold);

        assertTrue(h.vulnerableLiquidatable(victimBalance, vUnits, operatorFee, networkFee));
    }
}


## Suggested Mitigation
Use one canonical fee-accrual formula for both balance decay and liquidation. In isLiquidatableWithEB and isLiquidatableWithVUnits, floor operator and network components separately before summing, or refactor liquidation to call the same helper used by updateBalanceWithEB/_applyClusterFeeUpdates. Add boundary tests where vUnits % BPS_DENOMINATOR != 0 and both fee components are nonzero.
```

### Current Validated Block
### C-5 / `xV5HkuxJmnZDvpMrER57I`
- Finding Title: Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `liquidation-threshold-rounding`
- Bounty Criteria Match: critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- Checklist Gates Passed: `in-scope asset, unprivileged liquidate path, possible listed impact, code path exists, cannot concretely falsify before PoC`
- Checklist Gates Failed: `mainnet-state fork PoC still required for final submission confidence`
- Detailed Reason: The code does use separate flooring for accrued balance decay and combined flooring for liquidation threshold math, and a liquidation accepted only because of that boundary would transfer the remaining cluster balance to the liquidator. The main counterargument is that the repository specification documents the combined `(burnRate + networkFee)` threshold, but that does not concretely falsify the finding because the live balance-decay path uses a different rounding surface and the liquidator payout is the full remaining cluster balance, not merely the rounding delta. Promote this to the submission-candidate path so R5/R6 can require a mainnet-fork PoC to prove or falsify serious permissionless damage.
- Code Evidence: `SSVClusters.liquidate` updates cluster balance and then checks `isLiquidatableWithEB` before `_executeLiquidation` transfers `balanceLiquidatable` to the caller in `contracts/modules/SSVClusters.sol:31` and `contracts/modules/SSVClusters.sol:603`. `ClusterLib.isLiquidatableWithEB` combines rates before division in `contracts/libraries/ClusterLib.sol:67`, while `ClusterLib.updateBalanceWithEB` floors network and operator fee components separately in `contracts/libraries/ClusterLib.sol:306`; the documented threshold formula also uses the combined rate in `docs/SPEC.md:949`.

## M-9 / `JDUeqSWcB7YKK1PeXVxfX`
- Finding title: Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state
- Report lines: 712-834

### Original Report Block
```md
## [M-9]. Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state

## id: JDUeqSWcB7YKK1PeXVxfX

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
FrontrunMev

## Location
SSVClusters via SSVNetwork.deposit

## Finding Status: Valid
### Finding Status Justification: The described path exists in the provided production code. SSVNetwork.deposit delegates to SSVClusters.deposit, which validates the caller-supplied cluster hash, requires only the ETH cluster version, adds msg.value to cluster.balance, and stores the new cluster hash. There is no minimum msg.value, no liquidation-health check, and no owner restriction. SSVClusters.liquidate first calls validateHashedCluster on the supplied Cluster, so a prior 1 wei deposit changes the stored hash and makes the liquidator's stale calldata revert before liquidation logic can proceed. nonReentrant on liquidate does not stop mempool frontrunning, and deposit has no complete safeguard for this exact stale-state invalidation. The path is in analyzed production SSVNetwork/SSVClusters behavior, does not require privileged access, does not depend on victim misuse, and is exploitable today whenever liquidation calldata is public.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Cluster state-changing calls rely on the caller supplying the exact currently stored Cluster struct. `liquidate()` first validates the supplied struct against storage, then computes whether it is liquidatable. However `deposit()` is permissionless for any `clusterOwner`, accepts any `msg.value`, performs no minimum deposit or liquidation-health check, and rewrites the stored cluster hash after only adding the deposit amount:

```solidity
function deposit(address clusterOwner, uint64[] calldata operatorIds, Cluster memory cluster) external payable override {
    (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
    ClusterLib.validateClusterVersion(version, VERSION_ETH);

    cluster.balance += msg.value;
    s.ethClusters[hashedCluster] = cluster.hashClusterData();
}
```

A cluster owner, or any third party, can frontrun a pending liquidation with a dust deposit that is too small to make the cluster healthy. This changes only `cluster.balance` in the stored hash, so the pending liquidator transaction reverts during `cluster.validateHashedCluster(...)` before liquidation can proceed. The attacker can repeat this against public liquidation transactions to cheaply delay liquidation and force liquidators to waste gas while the cluster remains underfunded.

## Impact
Medium griefing/theft-of-gas impact: liquidators can be repeatedly forced to pay for reverting liquidation transactions, and undercollateralized clusters can remain active longer than intended if the owner keeps invalidating public liquidation calldata with dust deposits.

## Proof of Concept
1. A cluster becomes externally liquidatable using stored `cluster` state with balance `B`.
2. An honest liquidator submits `liquidate(clusterOwner, operatorIds, cluster)` using the currently valid cluster struct.
3. The cluster owner observes the transaction and frontruns it with `deposit(clusterOwner, operatorIds, cluster)` and `msg.value = 1 wei`.
4. `deposit()` stores the same cluster with balance `B + 1` even though the cluster is still liquidatable.
5. The liquidator transaction now calls `validateHashedCluster(...)` with the stale balance `B` and reverts.
6. The owner repeats the dust-deposit frontrun for each public liquidation attempt, paying tiny value to preserve liveness and impose gas losses on liquidators.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import "forge-std/Test.sol";

contract ClusterHashModel {
    struct Cluster { uint32 validatorCount; uint64 networkFeeIndex; uint64 index; bool active; uint256 balance; }

    mapping(bytes32 => bytes32) public ethClusters;

    function clusterId(address owner, uint64[] memory operatorIds) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(owner, operatorIds));
    }

    function hashClusterData(Cluster memory c) public pure returns (bytes32) {
        return keccak256(abi.encode(c.validatorCount, c.networkFeeIndex, c.index, c.active, c.balance));
    }

    function seed(address owner, uint64[] memory operatorIds, Cluster memory c) external {
        ethClusters[clusterId(owner, operatorIds)] = hashClusterData(c);
    }

    function deposit(address owner, uint64[] memory operatorIds, Cluster memory c) external payable {
        bytes32 id = clusterId(owner, operatorIds);
        require(ethClusters[id] == hashClusterData(c), "IncorrectClusterState");
        c.balance += msg.value;
        ethClusters[id] = hashClusterData(c);
    }

    function liquidate(address owner, uint64[] memory operatorIds, Cluster memory c) external {
        bytes32 id = clusterId(owner, operatorIds);
        require(ethClusters[id] == hashClusterData(c), "IncorrectClusterState");
        require(c.active, "inactive");
        require(c.balance < 1 ether, "not liquidatable");
        c.active = false;
        c.balance = 0;
        ethClusters[id] = hashClusterData(c);
    }
}

contract DustDepositLiquidationGriefPoC is Test {
    function testDustDepositInvalidatesPendingLiquidationCalldata() external {
        ClusterHashModel model = new ClusterHashModel();
        address owner = address(0xA11CE);
        address liquidator = address(0xB0B);
        address griefer = owner;

        uint64[] memory operators = new uint64[](4);
        operators[0] = 1; operators[1] = 2; operators[2] = 3; operators[3] = 4;

        ClusterHashModel.Cluster memory stale = ClusterHashModel.Cluster({
            validatorCount: 1,
            networkFeeIndex: 0,
            index: 0,
            active: true,
            balance: 0.01 ether
        });

        model.seed(owner, operators, stale);

        vm.deal(griefer, 1 wei);
        vm.prank(griefer);
        model.deposit{value: 1 wei}(owner, operators, stale);

        assertEq(model.ethClusters(model.clusterId(owner, operators)) != model.hashClusterData(stale), true);

        vm.prank(liquidator);
        vm.expectRevert(bytes("IncorrectClusterState"));
        model.liquidate(owner, operators, stale);
    }
}

## Suggested Mitigation
Do not let arbitrary dust deposits invalidate liquidation calldata unless they actually cure liquidation. Before storing a deposit, update accrued fees and require that an active liquidatable cluster becomes non-liquidatable after the top-up, or add a liquidation path that validates immutable cluster identity fields while reading the current stored balance. A practical fix is to store mutable cluster fields separately instead of only a hash, then let `liquidate()` operate on current storage rather than requiring an exact balance snapshot from calldata.
```

### Current Validated Block
### M-9 / `JDUeqSWcB7YKK1PeXVxfX`
- Finding Title: Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `state-hash-frontrun-griefing`
- Bounty Criteria Match: medium (smart contract): Theft of gas; medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- Checklist Gates Passed: `in-scope asset, unprivileged attacker, listed medium impact, code path exists, no privileged role needed`
- Checklist Gates Failed: `-`
- Detailed Reason: A public liquidation can be frontrun by any address that submits a tiny ETH `deposit` against the same current cluster state, changing only the stored cluster hash and causing the pending liquidation to revert before it reaches the liquidatability check. The attacker does not need privileged access or oracle control, and repeated dust deposits can waste liquidator gas and delay liquidation of an unhealthy cluster. This maps to the program's Medium griefing and theft-of-gas rows, not to Critical theft or freezing of funds.
- Code Evidence: `SSVNetwork.deposit` and `SSVNetwork.liquidate` delegate to the in-scope cluster module in `contracts/SSVNetwork.sol:271` and `contracts/SSVNetwork.sol:294`. `SSVClusters.deposit` accepts any caller, validates the supplied cluster hash, adds `msg.value`, and stores the new hash without a minimum deposit or health check in `contracts/modules/SSVClusters.sol:186`; `SSVClusters.liquidate` first calls the same hash validation in `contracts/modules/SSVClusters.sol:31`, and `ClusterLib.validateHashedCluster` reverts on stale cluster data in `contracts/libraries/ClusterLib.sol:131`.

