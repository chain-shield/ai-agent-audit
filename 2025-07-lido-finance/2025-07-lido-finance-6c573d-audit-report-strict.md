# 2025 07 lido finance - Findings Report
## Commit hash: 6c573d1d2cce64b083af3c88dabcfc83c75e6747

## Protocol Overview 

### Community Staking Module v2  
Lido’s Community Staking Module (CSM) v2 opens Ethereum validation to solo & community stakers while protecting the protocol with an economic bond.

• **Core flow**  
  1. **On-boarding** – PermissionlessGate (open) or VettedGate (whitelist/referral) collect the required bond (ETH/stETH/wstETH) and call CSModule to create a Node Operator (NO) with initial validator keys.  
  2. **Key queue** – CSModule stores keys in FIFO priority queues. StakingRouter pulls `obtainDepositData()` to deposit the next batch.  
  3. **Bond & rewards** – CSAccounting keeps bonds as stETH shares, locks them against penalties, and lets NOs top-up or claim excess.  
  4. **Oracle cycle** – Off-chain Oracle computes performance, fees and strikes. HashConsensus reaches quorum; CSFeeOracle pushes a Merkle root to CSFeeDistributor (fees) and CSStrikes (strikes).  
  5. **Claiming** – NO submits Merkle proof to CSAccounting, which pulls fee shares from CSFeeDistributor and releases claimable bond/rewards.  
  6. **Exits & penalties** – Voluntary or forced exits are executed via CSEjector. CSExitPenalties tracks fines for delays, forced exits or EL-reward theft; locked bond can be burned.

• **Security & Governance** – Fine-grained roles, PausableUntil/GateSeal emergency pause, invariant tests, and upgradable OssifiableProxy deployments safeguard funds and allow DAO governance.

## Medium Risk Findings
[M-1]. DOS issue in CSModule::migrateToPriorityQueue
[M-2]. Oracle issue in CSVerifier::_getHistoricalBlockRootGI


### Number of Findings
- C: 0
- H: 0
- M: 2
- L: 0
- I: 0



# Medium Risk Findings

## [M-1]. DOS issue in CSModule::migrateToPriorityQueue

## Description
The function `migrateToPriorityQueue` is designed to allow node operators to move their queued validator keys from the legacy queue to a priority queue. However, the implementation incorrectly handles the `enqueuedCount` for the node operator. When keys are "migrated", they are added to the priority queue via `_enqueueNodeOperatorKeys`, which increments `no.enqueuedCount`. The original keys are not removed from the legacy queue, and their count is not decremented from `enqueuedCount`. This results in `enqueuedCount` being artificially inflated, as it now double-counts the migrated keys.

The direct consequence of this inflated counter is a Denial of Service for the node operator. The `_enqueueNodeOperatorKeys(uint256 nodeOperatorId)` function, which is responsible for adding any new depositable keys to the queue, contains the check `if (depositable <= enqueued) { return; }`. Due to the inflated `enqueued` value, this condition will likely evaluate to true, even when the operator has new keys that are eligible for deposit (`depositable > actual_enqueued_keys`). This causes the function to return prematurely, preventing the operator from enqueuing new validator keys and thus blocking them from participating further with new validators until their old queue entries are cleared, which can be a lengthy process.

## Impact
Node operators using the `migrateToPriorityQueue` feature will be unable to add new validator keys to the deposit queue. This prevents them from scaling their operations and earning rewards on new validators, effectively pausing their growth within the module. While no funds are lost, it's a significant operational disruption for affected operators.

## Proof of Concept
1. A Node Operator (NO) has 10 depositable keys. They are automatically enqueued in the legacy queue. The NO's state is `depositableValidatorsCount = 10`, `enqueuedCount = 10`.
2. The NO becomes eligible for a priority queue and calls `migrateToPriorityQueue(nodeOperatorId)`. Let's assume `toMigrate` is calculated as 10.
3. `_enqueueNodeOperatorKeys` is called, which adds a new batch of 10 keys to the priority queue and increments the NO's `enqueuedCount`. The state becomes `enqueuedCount = 20`. The old batch of 10 keys remains in the legacy queue.
4. The NO adds more bond and now has 5 more depositable keys. Through the standard mechanism, the NO's `depositableValidatorsCount` becomes 15.
5. The system automatically tries to enqueue these 5 new keys by calling `_enqueueNodeOperatorKeys(nodeOperatorId)` (e.g. via a call to `updateDepositableValidatorsCount`).
6. Inside this function, it checks `if (depositable <= enqueued)`. With `depositable = 15` and `enqueued = 20`, the condition `15 <= 20` is true.
7. The function returns, and the 5 new keys are never enqueued. The NO is prevented from having their new validators deposited.

## Proof of Code
```solidity
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import { Test } from "forge-std/Test";
import { Fixtures } from "../test/helpers/Fixtures.sol";
import { ICSModule, NodeOperator } from "../../src/interfaces/ICSModule.sol";
import { CSModule } from "../../src/CSModule.sol";
import { CSAccounting } from "../../src/CSAccounting.sol";
import { CSParametersRegistry } from "../../src/CSParametersRegistry.sol";

contract MigrateQueueDosTest is Fixtures {
    function setUp() public {
        _setUp();
        // Grant admin role to self for managing NO types
        _grantRole(csm, csm.DEFAULT_ADMIN_ROLE(), address(this));
        vm.prank(address(csm.LIDO_LOCATOR().getRoleHolder(csm.DEFAULT_ADMIN_ROLE())));
        csmAccounting.grantRole(csmAccounting.MANAGE_BOND_CURVES_ROLE(), address(this));
    }

    function test_dos_migrateToPriorityQueue() public {
        // 1. Setup a priority queue for a new NO type (bond curve)
        uint256 priorityCurveId = 1;
        uint32 priority = 1;
        uint32 maxDeposits = 20;
        csmParametersRegistry.setQueueConfig(priorityCurveId, priority, maxDeposits);

        // 2. Create a Node Operator
        uint256 keysCount = 10;
        (uint256 noId, address manager) = _createNodeOperator(
            address(this),
            keysCount,
            NO_TYPE_DEFAULT,
            address(0)
        );

        NodeOperator memory no = csm.getNodeOperator(noId);
        assertEq(no.enqueuedCount, 10, "Initial enqueued count should be 10");
        assertEq(no.depositableValidatorsCount, 10, "Initial depositable count should be 10");

        // 3. Make NO eligible for priority queue by changing its curve id
        csmAccounting.setBondCurve(noId, priorityCurveId);

        // 4. Call migrateToPriorityQueue
        vm.prank(manager);
        csm.migrateToPriorityQueue(noId);

        no = csm.getNodeOperator(noId);
        // enqueuedCount is now inflated
        assertEq(no.enqueuedCount, 20, "Enqueued count should be inflated to 20");
        assertEq(no.depositableValidatorsCount, 10, "Depositable should still be 10");

        // 5. Add more bond and keys
        uint256 newKeysCount = 5;
        uint256 requiredBond = csmAccounting.getRequiredBondForNextKeys(noId, newKeysCount);
        _fundAndApproveSteth(manager, address(csmAccounting), requiredBond);
        bytes memory pubkeys;
        bytes memory signatures;
        (pubkeys, signatures) = _getSigningKeys(newKeysCount, 10);

        // This call will internally trigger _updateDepositableValidatorsCount
        // which in turn triggers _enqueueNodeOperatorKeys
        vm.prank(manager);
        csm.addValidatorKeysStETH(manager, noId, newKeysCount, pubkeys, signatures, EMPTY_PERMIT);

        // 6. Check the state. The new keys should be depositable but not enqueued.
        no = csm.getNodeOperator(noId);
        assertEq(no.totalAddedKeys, 15, "Total added keys should be 15");
        assertEq(no.depositableValidatorsCount, 15, "Depositable should be 15");

        // The critical assertion: enqueuedCount did not increase by 5 for the new keys.
        // It is still 20 from the migration bug. An un-bugged version would result in 15.
        assertEq(no.enqueuedCount, 20, "enqueuedCount is stuck at 20, new keys not enqueued");
    }
}
```

## Suggested Mitigation
The `migrateToPriorityQueue` function should not increment `enqueuedCount`, as the total number of keys across all queues remains the same. A dedicated internal function for migration that does not increment `enqueuedCount` should be used. This resolves the immediate DoS vector. The underlying issue of key duplication in queues is expected to be resolved by the `cleanDepositQueue` mechanism over time.

```diff
// src/CSModule.sol

    function migrateToPriorityQueue(uint256 nodeOperatorId) external {
        // ... checks ...

        uint32 toMigrate = uint32(Math.min(enqueued, maxDeposits - deposited));
-       _enqueueNodeOperatorKeys(nodeOperatorId, priority, toMigrate);
+       _migrateKeysToQueue(nodeOperatorId, priority, toMigrate); // New internal function
        no.usedPriorityQueue = true;
        _incrementModuleNonce();
    }

    // NOTE: If `count` is 0 an empty batch will be created.
    function _enqueueNodeOperatorKeys(
        uint256 nodeOperatorId,
        uint256 queuePriority,
        uint32 count
    ) internal {
        NodeOperator storage no = _nodeOperators[nodeOperatorId];
        no.enqueuedCount += count;
        QueueLib.Queue storage q = _getQueue(queuePriority);
        q.enqueue(nodeOperatorId, count);
        emit BatchEnqueued(queuePriority, nodeOperatorId, count);
    }

+   function _migrateKeysToQueue(
+       uint256 nodeOperatorId,
+       uint256 queuePriority,
+       uint32 count
+   ) internal {
+       // This function enqueues keys without incrementing enqueuedCount,
+       // as it's for migration of already-counted keys.
+       QueueLib.Queue storage q = _getQueue(queuePriority);
+       q.enqueue(nodeOperatorId, count);
+       emit BatchEnqueued(queuePriority, nodeOperatorId, count);
+   }
```

## [M-2]. Oracle issue in CSVerifier::_getHistoricalBlockRootGI

## Description
The `_getHistoricalBlockRootGI` function incorrectly determines which G-Index to use for the `block_roots` field within a `HistoricalSummary` object when verifying a proof that spans across a network fork. It uses the `targetSlot` (the slot of the old block being proven) to decide between `_PREV` and `_CURR` G-Indices for the internal structure of the `HistoricalSummary`. However, it should use the `recentSlot` (the slot of the block whose state is being used for verification). 

All data structures within a given beacon state (at `recentSlot`) conform to that state's fork specification. Therefore, even if a `HistoricalSummary` object contains a root from a `targetSlot` that was pre-fork, the `HistoricalSummary` struct itself, as it exists in the state at `recentSlot`, will have the layout of the `recentSlot`'s fork. 

Using a G-Index from a previous fork (`_PREV`) to navigate a data structure from a new fork (`_CURR`) will lead to verification failure for legitimate proofs or, in a worst-case scenario, could lead to verification against an incorrect part of the state if the G-Index points to other sensitive data.

Vulnerable Code Snippet:
```solidity
// src/CSVerifier.sol:501-506
        gI = gI.concat(
            targetSlot < PIVOT_SLOT
                ? GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV
                : GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR
        ); // historicalSummaries[summaryIndex].blockRoots[0]
```
This logic should depend on `recentSlot` to correctly reflect the structure of the state against which the proof is being verified.

## Impact
This bug will cause legitimate historical withdrawal proofs to fail if they are submitted after a hard fork that alters the layout of the `HistoricalSummary` struct. For example, if a withdrawal happens pre-fork (`targetSlot < PIVOT_SLOT`) and the proof is submitted post-fork (`recentSlot >= PIVOT_SLOT`), the transaction will revert. This results in a Denial of Service for a core function of the contract, preventing the processing of valid withdrawals and potentially locking up information flow about withdrawn validators for an indefinite period until the contract is redeployed with a fix.

## Proof of Concept
1. A hard fork occurs at `PIVOT_SLOT`. This fork changes the layout of the `HistoricalSummary` struct in the Beacon State, making `GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV` and `GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR` different.
2. A validator is withdrawn at `targetSlot`, where `targetSlot < PIVOT_SLOT`.
3. Some time passes. The current beacon slot is `recentSlot`, where `recentSlot >= PIVOT_SLOT`.
4. A user calls `processHistoricalWithdrawalProof` to process the withdrawal. They provide valid proofs against the state at `recentSlot`.
5. The internal call to `_getHistoricalBlockRootGI(recentSlot, targetSlot)` is made.
6. The function correctly uses `recentSlot` to determine the G-Index for the `historical_summaries` list (`GI_FIRST_HISTORICAL_SUMMARY_CURR`).
7. However, it then incorrectly uses `targetSlot` to determine the G-Index for the `block_roots` field within the summary, selecting `GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV`.
8. The `SSZ.verifyProof` function receives an incorrect G-Index. It attempts to navigate the post-fork state structure using a pre-fork data layout, causing the proof verification to fail.
9. The transaction reverts, and the valid withdrawal cannot be processed.

## Proof of Code
```solidity
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSVerifier } from "../../src/CSVerifier.sol";
import { ICSVerifier, GIndices } from "../../src/interfaces/ICSVerifier.sol";
import { Slot } from "../../src/lib/Types.sol";
import { GIndex } from "../../src/lib/GIndex.sol";
import { ICSModule } from "../../src/interfaces/ICSModule.sol";
import { SSZ } from "../../src/lib/SSZ.sol";

// Mock contracts to satisfy dependencies
contract MockModule is ICSModule {
    function getSigningKeys(uint256, uint256, uint256) external view returns (bytes memory) {
        return abi.encodePacked(bytes32(0x01));
    }
    function submitWithdrawals(ValidatorWithdrawalInfo[] calldata) external {}
    // Implement other functions as no-ops
    function addNodeOperator(NodeOperatorManagementProperties calldata, bytes calldata, address) external payable returns (uint256) { return 0; }
    function addSigningKeys(uint256, uint256, bytes calldata, bytes calldata) external payable {}
    function removeSigningKeys(uint256, uint256, uint256) external {}
    function setNodeOperatorAddress(uint256, uint8, address) external {}
    function reportStuckValidators(uint256[] calldata) external {}
    function reportELRewardsStealingPenalty(uint256, bytes calldata, uint256) external {}
    function settleELRewardsStealingPenalty(uint256, bytes calldata) external {}
    function submitWithdrawalProof(uint256, bytes calldata, ICSVerifier.WithdrawalWitness calldata) external {}
    function submitHistoricalWithdrawalProof(uint256, bytes calldata, ICSVerifier.ProvableBeaconBlockHeader calldata, ICSVerifier.WithdrawalWitness calldata) external {}
    function onValidatorsVetted(uint256, uint256, uint256) external {}
    function onValidatorsRemoved(uint256, uint256) external {}
    function getNodeOperator(uint256) external view returns (NodeOperator memory) {}
    function getNodeOperatorsCount() external view returns (uint256) { return 1; }
    function getStakingModuleSummary() external view returns (uint256, uint256, uint256) {}
    function getSigningKey(uint256, uint256) external view returns (bytes memory) {}
}

// A modified version of CSVerifier to make _getHistoricalBlockRootGI public for testing
contract TestableCSVerifier is CSVerifier {
    constructor(
        address withdrawalAddress,
        address module,
        uint64 slotsPerEpoch,
        uint64 slotsPerHistoricalRoot,
        GIndices memory gindices,
        Slot firstSupportedSlot,
        Slot pivotSlot,
        Slot capellaSlot,
        address admin
    ) CSVerifier(withdrawalAddress, module, slotsPerEpoch, slotsPerHistoricalRoot, gindices, firstSupportedSlot, pivotSlot, capellaSlot, admin) {}

    function getHistoricalBlockRootGIPublic(
        Slot recentSlot,
        Slot targetSlot
    ) external view returns (GIndex gI) {
        return _getHistoricalBlockRootGI(recentSlot, targetSlot);
    }
}

contract CSVerifierTest is Test {
    TestableCSVerifier internal verifier;
    GIndices internal gindices;
    Slot internal PIVOT_SLOT = Slot.wrap(100_000);

    function setUp() public {
        // These G-Indices simulate a fork where a field was added before `block_roots` in HistoricalSummary
        // The relative path to block_roots changes from index 1 to index 2.
        gindices.gIFirstHistoricalSummaryPrev = GIndex.from_generalized_index(100);
        gindices.gIFirstHistoricalSummaryCurr = GIndex.from_generalized_index(100); // Base path to summaries list is the same
        gindices.gIFirstBlockRootInSummaryPrev = GIndex.from_generalized_index(SSZ.to_gindex(1, 2)); // depth 1, index 1
        gindices.gIFirstBlockRootInSummaryCurr = GIndex.from_generalized_index(SSZ.to_gindex(2, 2)); // depth 1, index 2

        address admin = address(this);
        address mockModule = address(new MockModule());

        verifier = new TestableCSVerifier(
            address(0x1), 
            mockModule,
            32, 
            8192,
            gindices,
            Slot.wrap(0),
            PIVOT_SLOT, 
            Slot.wrap(0), 
            admin
        );
    }

    function test_PoC_IncorrectHistoricalGIndex() public {
        // Scenario: verify a proof for a pre-fork block (`targetSlot`) using a post-fork state (`recentSlot`)
        Slot recentSlot = Slot.wrap(PIVOT_SLOT.unwrap() + 100);
        Slot targetSlot = Slot.wrap(PIVOT_SLOT.unwrap() - 100);
        uint64 SLOTS_PER_HISTORICAL_ROOT = 8192;

        // --- 1. Calculate the INCORRECT G-Index produced by the contract ---
        GIndex incorrectGI = verifier.getHistoricalBlockRootGIPublic(recentSlot, targetSlot);

        // --- 2. Manually calculate the CORRECT G-Index ---
        uint256 targetSlotShifted = targetSlot.unwrap() - 0; // Assuming CAPELLA_SLOT is 0
        uint256 summaryIndex = targetSlotShifted / SLOTS_PER_HISTORICAL_ROOT;
        uint256 rootIndex = targetSlot.unwrap() % SLOTS_PER_HISTORICAL_ROOT;

        // Base G-Index should be for the CURRENT fork (since recentSlot is post-pivot)
        GIndex gIBase = gindices.gIFirstHistoricalSummaryCurr;
        gIBase = gIBase.shr(summaryIndex);

        // The path within the summary should ALSO be for the CURRENT fork
        GIndex gIConcat = gindices.gIFirstBlockRootInSummaryCurr;

        GIndex correctGI = gIBase.concat(gIConcat).shr(rootIndex);

        // --- 3. Assert that the contract's calculation is wrong ---
        console.log("Incorrect GI:", incorrectGI.unwrap());
        console.log("Correct GI:  ", correctGI.unwrap());

        assertNotEq(
            incorrectGI.unwrap(),
            correctGI.unwrap(),
            "The G-Index calculated by the contract is incorrect"
        );

        // --- 4. Verify what the incorrect GI is equal to ---
        GIndex expectedIncorrectGI = gIBase.concat(gindices.gIFirstBlockRootInSummaryPrev).shr(rootIndex);
        assertEq(
            incorrectGI.unwrap(),
            expectedIncorrectGI.unwrap(),
            "The calculated G-Index matches the buggy logic"
        );
    }
}
```

## Suggested Mitigation
The logic in `_getHistoricalBlockRootGI` should be corrected to use `recentSlot` for determining the G-Index for all parts of the state structure being verified. The selection between `_PREV` and `_CURR` G-Indices for the `block_roots` field should depend on the fork of the state being accessed (`recentSlot`), not the fork of the historical data point (`targetSlot`).

```diff
--- a/src/CSVerifier.sol
+++ b/src/CSVerifier.sol
@@ -498,9 +498,9 @@
 
         gI = gI.shr(summaryIndex); // historicalSummaries[summaryIndex]
         gI = gI.concat(
-            targetSlot < PIVOT_SLOT
+            recentSlot < PIVOT_SLOT
                 ? GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV
                 : GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR
         ); // historicalSummaries[summaryIndex].blockRoots[0]
         gI = gI.shr(rootIndex); // historicalSummaries[summaryIndex].blockRoots[rootIndex]
     }

```



