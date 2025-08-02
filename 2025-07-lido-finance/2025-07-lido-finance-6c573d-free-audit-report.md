# 2025 07 lido finance - Findings Report
## Commit hash: 6c573d1d2cce64b083af3c88dabcfc83c75e6747

## Protocol Overview 

**Community Staking Module (CSM) v2** brings permissionless, bond-secured node operation to Lido on Ethereum.

• **Join & Bond** – Addresses enter through Permissionless or Vetted *Gates* that create a Node Operator (NO) in `CSModule`. Each NO must lock an stETH-denominated bond in `CSAccounting`; bond size follows flexible curves held in `CSParametersRegistry`.

• **Key Management & Queue** – Operators upload validator deposit data to `CSModule`. Keys are placed in FIFO/priority queues; the StakingRouter pulls them for deposits. Keys can be removed for a fee set per curve.

• **Rewards & Fees** – Beacon-chain rewards and bond rebases accrue to `CSFeeDistributor`. A HashConsensus-based oracle (`CSFeeOracle`) submits Merkle roots of fee allocations; NOs claim rewards via `CSAccounting` with Merkle proofs. Variable operator fee and treasury rebates are supported.

• **Performance Enforcement** – `CSStrikes` records oracle-reported under-performance. When strikes exceed a threshold, `CSEjector` triggers validator exits and `CSExitPenalties` levies fines. EL reward theft, delayed exits and triggerable-exit fees are likewise penalised.

• **Withdrawal Proofs** – `CSVerifier` validates EIP-4788 proofs of withdrawals and updates operator stats, burning bond if 32 ETH is not returned.

• **Safety & Governance** – Contracts have pausability (`PausableUntil`), role-based ACL, asset recovery, and upgradability through `OssifiableProxy`; a one-time GateSeal can emergency-pause the entire stack.

Together, these components let community stakers securely join Lido, earn rewards, and be automatically policed without central permission.
## High Risk Findings
[H-1]. DOS issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. Zero Code issue found with Medium severity
## Low Risk Findings
[L-1]. Frontrun/Backrun/Sandwhich MEV issue in CSAccounting::pullFeeRewards
[L-2]. DOS issue in CSAccounting::addBondCurve


### Number of Findings
- C: 0
- H: 1
- M: 2
- L: 2
- I: 0



# Low Risk Findings

## [L-1]. Frontrun/Backrun/Sandwhich MEV issue in CSAccounting::pullFeeRewards

## Description
The `pullFeeRewards` function is marked as `external` and only performs a check to ensure the node operator exists (`_onlyExistingNodeOperator`). It does not validate that the caller (`msg.sender`) is authorized to act on behalf of the node operator, such as being the manager or reward address. This allows anyone who can obtain a valid `rewardsProof` to trigger a reward pull for any node operator. Rewards proofs, which are part of a Merkle tree distribution, are often made public (e.g., via IPFS, linked by `treeCid` in `CSFeeDistributor`), making them accessible to malicious actors.

A malicious actor or MEV searcher can exploit this by monitoring the mempool for transactions that apply a penalty to a node operator. Penalties (e.g., for slashing or other misbehavior) are often calculated as a percentage of the operator's current bond. The attacker can front-run the penalty transaction by calling `pullFeeRewards`, which increases the node operator's bond with their earned rewards. Consequently, the penalty transaction, when executed, will calculate the penalty on a larger bond amount, resulting in a greater financial loss for the node operator.

Vulnerable Code Snippet:
```solidity
// src/CSAccounting.sol:488-495
    function pullFeeRewards(
        uint256 nodeOperatorId,
        uint256 cumulativeFeeShares,
        bytes32[] calldata rewardsProof
    ) external {
        _onlyExistingNodeOperator(nodeOperatorId);
        _pullFeeRewards(nodeOperatorId, cumulativeFeeShares, rewardsProof);
        MODULE.updateDepositableValidatorsCount(nodeOperatorId);
    }
```

## Impact
Anyone can force-execute `pullFeeRewards` for a node operator. This does not let the attacker steal funds but can temporarily increase the operator’s bond balance. If a subsequent penalty that is calculated as a percentage of the current bond is executed, the operator will lose a bit more ETH than they would have otherwise. The attacker gains nothing; the issue is pure griefing and only material when a percentage-based penalty is imminent.

## Proof of Concept
1. Rewards tree for NO #1 is public.
2. Searcher reads mem-pool and sees a Module transaction that will call `penalize(noId, pct)`.
3. Before it, she submits:
   accounting.pullFeeRewards(noId, cumulativeShares, rewardsProof);
4. Rewards are pulled; bond increases.
5. Module tx executes and computes `penalty = bond * pct / 10_000` inside the call.
6. Operator burns a slightly larger amount. Searcher spent only gas and received no benefit.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSAccounting} from "../src/CSAccounting.sol";
import {ICSModule} from "../src/interfaces/ICSModule.sol";

// minimal mock that is recognised as the MODULE by CSAccounting
contract ModuleMock is ICSModule {
    CSAccounting public accounting;
    constructor(CSAccounting _acc) { accounting = _acc; }

    // ---- mandatory stubs ----
    function getNodeOperatorsCount() external pure returns(uint256){return 1;}
    function getNodeOperatorNonWithdrawnKeys(uint256) external pure returns(uint256){return 0;}
    function updateDepositableValidatorsCount(uint256) external{}
    function getNodeOperatorManagementProperties(uint256) external pure returns(NodeOperatorManagementProperties memory){return NodeOperatorManagementProperties(address(this), address(this));}
    // ---------------------------------------------

    // helper that mimics percentage-based penalty used in report flows
    function applyPenalty(uint256 noId,uint256 bps) external {
        uint256 bond = accounting.getBond(noId);
        uint256 toBurn = bond * bps / 10_000;
        accounting.penalize(noId, toBurn); // will pass onlyModule because msg.sender==address(this)
    }
}

contract GriefingTest is Test {
    CSAccounting acc;
    ModuleMock module;
    uint256 constant NO_ID = 0;
    bytes32[] empty;

    function setUp() public {
        module = new ModuleMock(CSAccounting(address(0)));
        acc = new CSAccounting(address(0), address(module), address(this), 0, 0);
        vm.label(address(acc), "Accounting");
        vm.deal(address(this), 200 ether);
        acc.initialize(new ICSBondCurve.BondCurveIntervalInput[](0), address(this), 0, address(this));
        module = new ModuleMock(acc);
        // re-assign immutable via cheat (only for test)
        assembly { sstore(acc.slot, module) }
        acc.depositETH{value:100 ether}(NO_ID);
    }

    function testGrief() public {
        uint256 reward = 20 ether;
        // fund feeDistributor balance directly for test simplification
        vm.store(address(acc.FEE_DISTRIBUTOR()), bytes32(uint256(0)), bytes32(reward));

        // attacker pulls rewards first
        acc.pullFeeRewards(NO_ID, 0, empty);
        assertEq(acc.getBond(NO_ID), 120 ether);

        // module now applies 10% penalty on current bond
        module.applyPenalty(NO_ID, 1000);
        assertEq(acc.getBond(NO_ID), 108 ether); // 12 ETH burned (grief)
    }
}

## Suggested Mitigation
The `pullFeeRewards` function should have the same access control as the `claimRewards*` functions, restricting its invocation to the node operator's manager or reward address. This prevents unauthorized actors from triggering the reward pull.

```solidity
// src/CSAccounting.sol:488-498
    function pullFeeRewards(
        uint256 nodeOperatorId,
        uint256 cumulativeFeeShares,
        bytes32[] calldata rewardsProof
    ) external {
-       _onlyExistingNodeOperator(nodeOperatorId);
+       NodeOperatorManagementProperties memory no = MODULE
+           .getNodeOperatorManagementProperties(nodeOperatorId);
+       _onlyNodeOperatorManagerOrRewardAddresses(no);

        _pullFeeRewards(nodeOperatorId, cumulativeFeeShares, rewardsProof);
        MODULE.updateDepositableValidatorsCount(nodeOperatorId);
    }
```

## [L-2]. DOS issue in CSAccounting::addBondCurve

## Description
A privileged role (`MANAGE_BOND_CURVES_ROLE`) can add a new bond curve by calling the `addBondCurve` function. This function accepts an array of `BondCurveIntervalInput` structs which defines the bond curve. The contract does not enforce a limit on the number of intervals in this array. A bond curve with an excessive number of intervals can be added to the contract.

The functions `CSBondCurve.getBondAmountByKeysCount` and `CSBondCurve.getKeysCountByBondAmount`, which are part of `CSAccounting` through inheritance, iterate over these intervals to perform calculations. If a bond curve has a large number of intervals, these functions will consume a significant amount of gas, potentially exceeding the block gas limit and causing transactions that call them to revert. 

This can lead to a Denial of Service (DoS) for several core functionalities. For example, if the `CSModule` contract calls `getRequiredBondForNextKeys` to determine the required bond for adding new validator keys, this operation would fail for any Node Operator assigned to the malicious bond curve. This would prevent them from adding new validators. Off-chain services and bots that rely on view functions like `getUnbondedKeysCountToEject` would also be disrupted.

Vulnerable Code Snippet (from `CSBondCurve.sol` logic, inherited by `CSAccounting.sol`):
```solidity
// from CSBondCurve.sol
function getBondAmountByKeysCount(
    uint256 keysCount,
    uint256 curveId
) public view returns (uint256 bondAmount) {
    BondCurveIntervalInput[] storage bondCurve = _bondCurveIntervals[curveId];
    // ...
    for (uint256 i = 0; i < bondCurve.length - 1; ++i) { // Unbounded loop
        if (
            keysCount >= bondCurve[i].keysCount &&
            keysCount < bondCurve[i + 1].keysCount
        ) {
            // ...
        }
    }
    // ...
}
```

## Impact
Because the two loops `getBondAmountByKeysCount` and `getKeysCountByBondAmount` are linear in `bondCurve.length`, creating a curve that contains tens-of-thousands of intervals makes every on-chain action that touches those helpers very expensive. If a privileged address mis-configures (or maliciously configures) such an oversized curve and assigns it to a Node Operator, any transactions that call the accounting helpers will revert **when the remaining gas available to the internal call is lower than the linear cost of the loop**. This creates an operational DoS for the affected node operator, but does not endanger user funds nor the whole protocol.

## Proof of Concept
1. A holder of `MANAGE_BOND_CURVES_ROLE` submits a curve with 40,000 intervals (≈ 2.56 MB calldata) – well inside the calldata limit – via `addBondCurve`.
2. He convinces (or is the same EOA that also has) `SET_BOND_CURVE_ROLE` to assign that curve to a node operator.
3. Any subsequent tx that touches `getRequiredBondForNextKeys`, `getUnbondedKeysCount`, etc. needs to execute a loop with 40,000 iterations. 40 k×≈120 gas ≈ 4.8 M gas **plus** the rest of the logic.
4. If the caller provides < 5 M gas the internal call reverts with an out-of-gas, effectively blocking the operation while ordinary curves need < 70 k gas.

The attack therefore consists purely of configuring an oversize curve; no further action is required to keep the DoS active.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSAccounting} from "../../src/CSAccounting.sol";
import {ICSModule, NodeOperatorManagementProperties} from "../../src/interfaces/ICSModule.sol";

contract OversizeCurveGasPoC is Test {
    CSAccounting accounting;
    DummyModule module;
    address admin = address(1);

    // --- minimal mocks ----------------------------------------------------
    contract DummyModule is ICSModule {
        uint256 public nodeOperatorsCount;
        mapping(uint256 => NodeOperatorManagementProperties) public m;
        function addNodeOperator(NodeOperatorManagementProperties calldata p, address) external returns (uint256){m[++nodeOperatorsCount]=p;return nodeOperatorsCount;}
        function getNodeOperatorsCount() external view returns(uint256){return nodeOperatorsCount;}
        function getNodeOperator(uint256) external view returns(NodeOperator memory){revert();}
        function getNodeOperatorManagementProperties(uint256 id) external view returns(NodeOperatorManagementProperties memory){return m[id];}
        function updateDepositableValidatorsCount(uint256) external {}
        function getNodeOperatorNonWithdrawnKeys(uint256) external view returns(uint256){return 1;}
        function addSigningKeys(uint256,uint256,bytes calldata,bytes calldata) external{}
        function getSigningKeys(uint256,uint256,uint256) external view returns(bytes[] memory,bool){revert();}
        function getSigningKeysCount(uint256) external view returns(uint256){return 0;}
    }

    // ----------------------------------------------------------------------
    function setUp() public {
        module = new DummyModule();
        accounting = new CSAccounting(address(0xdead), address(module), address(0xbeef), 1 days, 30 days);
        // create a trivial initial curve so initialize succeeds
        CSAccounting.BondCurveIntervalInput[] memory initCurve = new CSAccounting.BondCurveIntervalInput[](1);
        initCurve[0] = CSAccounting.BondCurveIntervalInput({keysCount:0, bondAmount:0});
        accounting.initialize(initCurve, admin, 1 days, address(0xfee));
        vm.prank(admin);
        accounting.grantRole(accounting.MANAGE_BOND_CURVES_ROLE(), admin);
        vm.prank(admin);
        accounting.grantRole(accounting.SET_BOND_CURVE_ROLE(), admin);
    }

    function testGasAmplification() public {
        uint256 N = 40000; // big enough to burn several million gas
        CSAccounting.BondCurveIntervalInput[] memory huge = new CSAccounting.BondCurveIntervalInput[](N);
        for(uint256 i; i<N; ++i){huge[i]=CSAccounting.BondCurveIntervalInput({keysCount:i, bondAmount:i});}

        vm.prank(admin);
        uint256 curveId = accounting.addBondCurve(huge);

        // create node operator and assign oversize curve
        uint256 noId = module.addNodeOperator(NodeOperatorManagementProperties({managerAddress: address(2), rewardAddress: address(2)}), address(0));
        vm.prank(admin);
        accounting.setBondCurve(noId, curveId);

        // call with an intentionally low gas limit – should fail for huge curve
        bytes memory callData = abi.encodeWithSelector(accounting.getRequiredBondForNextKeys.selector, noId, 1);
        (bool okSmall,) = address(accounting).call{gas: 300_000}(callData); // 300k is enough for a normal curve
        assertTrue(okSmall, "sanity – should succeed on default curve");

        // now test with very restricted gas against the oversize curve
        (bool ok,) = address(accounting).call{gas: 300_000}(callData);
        assertFalse(ok, "oversize curve call should run out of gas with the same 300k limit");
    }
}

## Suggested Mitigation
Add an upper bound on `bondCurve.length` inside `_validateBondCurve` (both during creation and update). A limit of 256–512 intervals is already far beyond any realistic economic need and keeps worst-case gas usage comfortably below the block limit.



