# ssv-network Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ssv-network/report/audit-report.md`
Finding count: `4`

## Findings

### H-1 / `QG_ICWJZysYLIuVfb6oiu`
- Finding title: Permissionless fee sync can permanently strand staking ETH reward dust in SSVStaking._syncFees
- Report lines: 75-150
```md
## [H-1]. Permissionless fee sync can permanently strand staking ETH reward dust in SSVStaking._syncFees

## id: QG_ICWJZysYLIuVfb6oiu

## Derived From Pattern/Invariant
PrecisionDriftAccumulation

## Exploit Type
RoundingError

## Location
SSVStaking.syncFees

## Finding Status: Valid
### Finding Status Justification: The accounting path exists. SSVNetwork.syncFees delegates to SSVStaking.syncFees, which is external and permissionless. _syncFees computes current = sp.networkTotalEarnings(), sets sp.ethDaoBalance = current, computes packedNewFees = current - previous, and when totalStaked is nonzero increases accEthPerShare by floor((newFeesWei * PRECISION) / totalStaked). It then sets s.stakingEthPoolBalance = current regardless of whether the per-share increment was zero or had a remainder. No remainder accumulator or allocation reconciliation is present. Therefore small fee increments can be marked as synced while not increasing any staker's claimable rewards, and later syncs start from the advanced pool balance. nonReentrant is irrelevant to this precision-loss path. The issue is in production delegatecall behavior, is permissionless, does not require user error or privileged abuse, and is not speculative.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
SSVNetwork.syncFees delegatecalls into SSVStaking._syncFees, where the full newly observed ETH reward amount is marked as synced even when the per-share increment rounds down to zero or leaves a remainder. The vulnerable logic is: `s.accEthPerShare += uint128((newFeesWei * PRECISION) / totalStaked);` followed by `s.stakingEthPoolBalance = current;`. Any remainder from `(newFeesWei * PRECISION) / totalStaked` is not carried forward, but `stakingEthPoolBalance` is still advanced to `current`, so the undistributed ETH is treated as already allocated and is never included in a later sync. A permissionless caller can repeatedly call syncFees when incremental DAO/network earnings are small relative to cSSV totalSupply, permanently stranding reward dust that should have accrued to stakers.

## Impact
Permanent freezing of unclaimed ETH yield inside the staking accounting. The affected rewards remain included in stakingEthPoolBalance/ethDaoBalance but no staker receives corresponding accrued rewards, matching the program's permanent freezing of unclaimed yield impact.

## Proof of Concept
1. Ensure cSSV totalSupply is high enough that a small newly accrued ETH fee increment produces `(newFeesWei * 1e18) / totalStaked == 0`. 2. Any EOA calls SSVNetwork.syncFees(). 3. _syncFees sets stakingEthPoolBalance to the new current network earnings but accEthPerShare does not increase. 4. preview/settlement for all stakers returns zero additional claimable ETH for that increment. 5. Later syncs start from the already-advanced stakingEthPoolBalance, so the skipped amount is never redistributed.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";

contract StakingRoundingHarness {
    uint256 constant PRECISION = 1e18;
    uint256 public stakingEthPoolBalance;
    uint256 public accEthPerShare;

    function sync(uint256 currentNetworkEarningsWei, uint256 totalStaked) external {
        uint256 previous = stakingEthPoolBalance;
        if (currentNetworkEarningsWei <= previous) {
            stakingEthPoolBalance = currentNetworkEarningsWei;
            return;
        }
        uint256 newFeesWei = currentNetworkEarningsWei - previous;
        if (totalStaked != 0) {
            accEthPerShare += (newFeesWei * PRECISION) / totalStaked;
        }
        stakingEthPoolBalance = currentNetworkEarningsWei;
    }

    function pending(uint256 balance, uint256 userIndex) external view returns (uint256) {
        return (balance * (accEthPerShare - userIndex)) / PRECISION;
    }
}

contract SSVStakingSyncRoundingPoC is Test {
    function testSmallFeeSyncIsMarkedSyncedButNoStakerCanClaimIt() external {
        StakingRoundingHarness h = new StakingRoundingHarness();
        uint256 totalStaked = 1_000_000 ether;
        uint256 newFeesWei = 900_000;

        h.sync(newFeesWei, totalStaked);

        assertEq(h.accEthPerShare(), 0);
        assertEq(h.pending(totalStaked, 0), 0);
        assertEq(h.stakingEthPoolBalance(), newFeesWei);
        assertGt(h.stakingEthPoolBalance(), h.pending(totalStaked, 0));
    }
}

## Suggested Mitigation
Carry undistributed reward precision forward instead of advancing the pool over unallocated fees. Store a `rewardRemainderScaled` value and add it to the next `newFeesWei * PRECISION` calculation, or only increase stakingEthPoolBalance by the amount actually represented in accEthPerShare.
```

### C-5 / `xV5HkuxJmnZDvpMrER57I`
- Finding title: Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB
- Report lines: 401-507
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

### H-7 / `zzWCpNrNBYw-Cdvv_tgOw`
- Finding title: Frequent EB updates undercharge cluster fees by repeatedly flooring vUnit fee math
- Report lines: 575-640
```md
## [H-7]. Frequent EB updates undercharge cluster fees by repeatedly flooring vUnit fee math

## id: zzWCpNrNBYw-Cdvv_tgOw

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
RoundingError

## Location
SSVClusters.updateClusterBalance

## Finding Status: Valid
### Finding Status Justification: The root cause is present in SSVClusters.updateClusterBalance. For ETH clusters, _applyClusterFeeUpdates computes idxNet and idxOp deltas, then floors both (idxNet * units) / BPS_DENOMINATOR and (idxOp * units) / BPS_DENOMINATOR before multiplying by ETH_DEDUCTED_DIGITS and deducting from cluster.balance. No per-cluster remainder is stored, so when vUnits is not an exact multiple of 10000, splitting the same total index growth across multiple valid EB updates can discard fractional fees on each update. The minBlocksBetweenUpdates and latest-root/proof checks throttle and authenticate updates, but they do not preserve fractional fee debt and therefore are not a complete safeguard. The update function is publicly callable with valid current roots and proofs; relying on normal oracle root production is not privileged abuse by the attacker. The issue exists in current production code and is not merely future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
SSVClusters.updateClusterBalance charges accrued operator and network fees through _applyClusterFeeUpdates. The fee conversion floors each update: `uint128 networkFeeUnits = (idxNet * units) / BPS_DENOMINATOR; uint128 operatorFeeUnits = (idxOp * units) / BPS_DENOMINATOR; uint256 totalFees = (uint256(networkFeeUnits) + uint256(operatorFeeUnits)) * ETH_DEDUCTED_DIGITS;`. For clusters whose effective-balance vUnits are not an exact multiple of BPS_DENOMINATOR, splitting the same index growth across many permissionless EB updates discards the fractional fee component each time. The cluster owner can execute valid latest-root EB updates as often as allowed, causing less ETH to be debited from the cluster than if the same interval were accounted once, and later withdraw/self-liquidate the retained balance.

## Impact
The DAO/operators/stakers receive less unclaimed ETH fee yield than owed, while the cluster owner retains the undercharged ETH in the cluster balance. This is systematic value extraction from fee recipients through precision drift.

## Proof of Concept
1. A cluster has an explicit effective balance above baseline, e.g. vUnits = 15000 for a 48 ETH-equivalent validator, so fee math requires division by 10000. 2. Valid EB roots are committed over time. 3. The cluster owner or any caller repeatedly calls updateClusterBalance for small index deltas. 4. Each call floors `(idxDelta * vUnits) / 10000`, discarding the fractional component. 5. Over many updates, total charged fees are lower than a single update over the same total index delta, leaving extra ETH withdrawable by the cluster owner.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";

contract EBChargeHarness {
    uint256 constant BPS_DENOMINATOR = 10_000;
    uint256 constant ETH_DEDUCTED_DIGITS = 100_000;

    function charge(uint256 indexDelta, uint256 vUnits) public pure returns (uint256) {
        return ((indexDelta * vUnits) / BPS_DENOMINATOR) * ETH_DEDUCTED_DIGITS;
    }

    function chargeSplit(uint256 periods, uint256 indexDeltaPerPeriod, uint256 vUnits) external pure returns (uint256 total) {
        for (uint256 i; i < periods; ++i) {
            total += charge(indexDeltaPerPeriod, vUnits);
        }
    }
}

contract SSVClusterEBRoundingPoC is Test {
    function testSplitEBAccountingUnderchargesFees() external {
        EBChargeHarness h = new EBChargeHarness();
        uint256 vUnits = 15_000;
        uint256 periods = 1_000;
        uint256 oneShot = h.charge(periods, vUnits);
        uint256 split = h.chargeSplit(periods, 1, vUnits);

        assertGt(oneShot, split);
        assertEq(oneShot - split, 50_000_000);
    }
}

## Suggested Mitigation
Do not discard fractional vUnit fees on every EB update. Store per-cluster fee remainders for network and operator fee units and add them to the next calculation, or keep cluster indexes/fees in a higher-precision accumulator so splitting updates cannot reduce total charged fees.
```

### M-9 / `JDUeqSWcB7YKK1PeXVxfX`
- Finding title: Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state
- Report lines: 712-834
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
