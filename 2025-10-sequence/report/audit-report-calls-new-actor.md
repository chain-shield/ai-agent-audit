# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : ERC-4337 Bundler Griefing via Unreported Static Signature Expiry

[M-1]. ERC-4337 `validateUserOp` returns infinite validity for time-bounded signatures, enabling Bundler griefing
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Static-signature authorized caller for specific opHash

[M-2]. Static signatures persist across wallet configuration updates allowing unauthorized access
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : Actor Name: Wallet signers meeting multisig threshold | Abuse Title: Simulator Contract Enables Unauthenticated Wallet Execution

[H-3]. Unauthenticated Execution via Simulator Module Implementation Upgrade
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresAdminRole


### Number of Findings
- C: 0
- H: 1
- M: 2
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : ERC-4337 Bundler Griefing via Unreported Static Signature Expiry

## [M-1]. ERC-4337 `validateUserOp` returns infinite validity for time-bounded signatures, enabling Bundler griefing

### Finding Severity Justification: The finding identifies a violation of the ERC-4337 specification regarding time-bound validation. By returning 0 (infinite validity) instead of the actual expiration timestamp in `validateUserOp`, the wallet misleads Bundlers. This results in Bundlers relaying transactions that pass simulation but fail on-chain (revert) once the timestamp is exceeded, causing financial loss (gas wastage) to the Bundlers. This constitutes a griefing vector and strictly breaks protocol compliance.
## Derived From Pattern/Invariant
ERC-4337 Bundler Griefing via Unreported Static Signature Expiry

## Exploit Type
TimelockEdgeCase

## Location
ERC4337v07.validateUserOp

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ERC4337v07.validateUserOp` function is required to return a `validationData` value that packs the time-range validity of the UserOperation (validAfter/validUntil). However, the current implementation hardcodes the return value to `0`, indicating 'valid forever'.

The wallet's underlying authentication logic (`BaseAuth.signatureValidation`) enforces expiration for Static Signatures via `InvalidStaticSignatureExpired`. Because `validateUserOp` relies on `this.isValidSignature`, which returns a boolean/magic value but not the expiration timestamp, `validateUserOp` cannot propagate the actual expiry to the EntryPoint.

This creates a discrepancy where the Bundler believes a UserOp is valid indefinitely (based on `validationData`), but the wallet will revert (or fail) on-chain if the transaction is mined after the static signature's expiry. This leads to Bundlers relaying transactions that are doomed to fail, wasting gas and potentially causing the UserOp to be dropped or the wallet to be throttled.

## Impact
Bundlers waste gas on transactions that fail on-chain due to expiry, despite passing simulation. This violates ERC-4337 specifications and degrades service reliability.

## Command to Run Test


## Proof of Concept
1. User creates a UserOperation with a Static Signature valid until `T + 10 minutes`.
2. Bundler calls `validateUserOp` (via simulation). The wallet verifies the signature is currently valid and returns `0` (valid forever).
3. Bundler accepts the UserOp and queues it.
4. Due to congestion, the UserOp is included in a block at `T + 11 minutes`.
5. On-chain, `validateUserOp` executes. It calls `isValidSignature`, which calls `signatureValidation`. `signatureValidation` checks `timestamp <= block.timestamp`, fails, and reverts (or returns failure).
6. The transaction fails on-chain. The Bundler pays for the gas execution validation, which they expected to be valid indefinitely.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import { Test } from "forge-std/Test.sol";
import { Stage2Module } from "src/Stage2Module.sol";
import { BaseAuth } from "src/modules/auth/BaseAuth.sol";
import { PackedUserOperation } from "src/modules/interfaces/IAccount.sol";

contract ValidateUserOpTest is Test {
    Stage2Module wallet;
    address entryPoint = makeAddr("entryPoint");
    address signer = makeAddr("signer");

    function setUp() public {
        // Deploy wallet with mock entryPoint
        wallet = new Stage2Module(entryPoint);
    }

    function test_validateUserOp_ReportsInfiniteValidity_ButRevertsOnExpiry() public {
        // 1. Setup expiry and opHash
        uint96 expiry = uint96(block.timestamp + 100);
        bytes32 opHash = keccak256("operation_hash");
        
        // 2. Set static signature (must be self-called)
        vm.prank(address(wallet));
        BaseAuth(address(wallet)).setStaticSignature(opHash, signer, expiry);

        // 3. Construct UserOp with static signature flag (0x80)
        PackedUserOperation memory userOp;
        userOp.signature = hex"80";

        // 4. Call validateUserOp (Simulate Bundler check)
        vm.prank(entryPoint);
        uint256 validationData = wallet.validateUserOp(userOp, opHash, 0);

        // 5. Assert validationData reports "valid forever" (validUntil == 0)
        // validationData layout: [authorizer(160)][validUntil(48)][validAfter(48)]
        // 0 at bits 160-207 implies infinite validity.
        uint48 returnedValidUntil = uint48(validationData >> 160);
        assertEq(returnedValidUntil, 0, "validateUserOp incorrectly reports infinite validity");

        // 6. Warp past expiry
        vm.warp(block.timestamp + 101);

        // 7. Verify execution reverts on-chain due to expiry, causing Bundler grief
        vm.prank(entryPoint);
        vm.expectRevert(abi.encodeWithSelector(BaseAuth.InvalidStaticSignatureExpired.selector, opHash, expiry));
        wallet.validateUserOp(userOp, opHash, 0);
    }
}

## Suggested Mitigation
Do not modify the public `isValidSignature` function as it must adhere to the ERC-1271 standard (returning `bytes4`). Instead, modify the internal `signatureValidation` function to return the expiration timestamp (defaulting to `type(uint48).max` for non-expiring signatures). Update `ERC4337v07.validateUserOp` to call this internal function directly (instead of `this.isValidSignature`) to retrieve the timestamp and correctly pack it into the `validUntil` field (bits 160-207) of the returned `validationData`.





 **Derived From** : Static-signature authorized caller for specific opHash

## [M-2]. Static signatures persist across wallet configuration updates allowing unauthorized access

### Finding Severity Justification: The vulnerability allows a removed signer or attacker to maintain persistent access to the wallet via pre-authorized static signatures, effectively bypassing the security guarantees of configuration rotation (key rotation). While it requires the attacker to have had access previously to set the static signature, it constitutes a privilege escalation/persistence issue that violates the core invariant that the wallet's current image hash defines its security state. This could lead to unauthorized fund transfers after a user believes they have secured the wallet by removing a compromised signer.
## Derived From Pattern/Invariant
Static-signature authorized caller for specific opHash

## Exploit Type
AccessControl

## Location
BaseAuth.signatureValidation

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `BaseAuth` contract allows the wallet to authorize specific payloads via static signatures stored in the `STATIC_SIGNATURE_KEY` mapping. These signatures are validated in `signatureValidation` before and independently of the wallet's configuration (`imageHash`). 

When a wallet owner updates their configuration via `updateImageHash` (e.g., to rotate signers after a key compromise or to remove a malicious signer), the `imageHash` is updated, but the `STATIC_SIGNATURE_KEY` mapping is not cleared. 

This creates a vulnerability where a removed signer or an attacker with a previously authorized static signature can continue to execute the authorized operation indefinitely (until expiry), bypassing the signer rotation. This violates the invariant that the wallet's security configuration is defined by its image hash.

## Impact
A removed signer or attacker can maintain persistent access to specific wallet operations even after the wallet configuration has been rotated to revoke their access.

## Command to Run Test


## Proof of Concept
1. **Setup**: Deploy the Wallet with Configuration A (Signer A). 
2. **Authorize**: Signer A executes a self-call transaction via `execute()` to call `setStaticSignature(opHash, address(0), expiry)`. This pre-approves a specific operation `opHash` via a static signature.
3. **Rotate**: The wallet keys are compromised or rotated. The owner performs an `updateImageHash` transaction to switch to Configuration B (Signer B), intending to revoke Signer A's access.
4. **Exploit**: An attacker (or the removed Signer A) submits the transaction corresponding to `opHash` with the static signature flag `0x80`.
5. **Result**: `BaseAuth.signatureValidation` checks the `STATIC_SIGNATURE_KEY` mapping. Since the mapping was not cleared during the image hash update, the static signature is still found and valid. The transaction executes successfully, bypassing the new configuration.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {BaseAuth} from "src/modules/auth/BaseAuth.sol";
import {Payload} from "src/modules/Payload.sol";
import {Storage} from "src/modules/Storage.sol";

contract HarnessAuth is BaseAuth {
    bytes32 public currentImageHash;
    
    function initialize(bytes32 _img) external {
        currentImageHash = _img;
    }

    // Mock implementation of abstract method
    function _isValidImage(bytes32 _imageHash) internal view override returns (bool) {
        return _imageHash == currentImageHash;
    }

    // Mock implementation of abstract method
    function _updateImageHash(bytes32 _imageHash) internal override {
        currentImageHash = _imageHash;
    }

    // Helper to verify signature logic
    function verify(Payload.Decoded memory payload, bytes calldata signature) external view returns (bool) {
        (bool valid, ) = signatureValidation(payload, signature);
        return valid;
    }

    // Simulate internal self-call to set static sig
    function setStaticSig(bytes32 _hash, address _addr, uint96 _timestamp) external {
        _setStaticSignature(_hash, _addr, _timestamp);
    }
    
    // Simulate internal self-call to update image hash (rotate keys)
    function rotateConfig(bytes32 _newImageHash) external {
        _updateImageHash(_newImageHash);
    }
}

contract StaticSigPersistenceTest is Test {
    HarnessAuth auth;
    
    function setUp() public {
        auth = new HarnessAuth();
        auth.initialize(bytes32(uint256(1))); // Config A
    }

    function test_StaticSigPersistsAfterRotation() public {
        // 1. Create a payload and calculate its hash for the wallet
        Payload.Decoded memory payload;
        payload.kind = Payload.KIND_TRANSACTIONS;
        payload.nonce = 123;
        // We use hashFor to match the wallet's domain separator
        bytes32 opHash = Payload.hashFor(payload, address(auth));

        // 2. Authorize it via static signature (simulating Config A authorization)
        // Valid for 1000 seconds
        uint96 expiry = uint96(block.timestamp + 1000);
        auth.setStaticSig(opHash, address(0), expiry);

        // 3. Verify it works initially
        bytes memory staticSig = abi.encodePacked(uint8(0x80)); // Flag 0x80 = Static Signature
        assertTrue(auth.verify(payload, staticSig), "Static sig should be valid under Config A");

        // 4. Rotate wallet configuration to Config B (Simulate removing the signer)
        auth.rotateConfig(bytes32(uint256(2))); 

        // 5. Verify static signature persists
        bool isValid = auth.verify(payload, staticSig);
        
        // Assert: The static signature from Config A is still valid in Config B
        assertTrue(isValid, "VULNERABILITY: Static signature persists after config rotation");
    }
}

## Suggested Mitigation
Introduce a `staticEpoch` to namespace static signatures to the current configuration version. 

1. Define a new storage key constant, e.g., `bytes32 private constant STATIC_EPOCH_KEY = keccak256("org.sequence.module.auth.static.epoch");`.
2. In `_updateImageHash` (override in `BaseAuth` or implement in `StageXAuth`), read the current epoch using `Storage.readBytes32`, increment it, and write it back.
3. Modify `_getStaticSignature` and `_setStaticSignature` to include this epoch in the mapping key. 
   - Current: `readBytes32Map(STATIC_SIGNATURE_KEY, _hash)`
   - New: `readBytes32Map(keccak256(abi.encode(STATIC_SIGNATURE_KEY, currentEpoch)), _hash)`

This ensures that whenever the configuration (imageHash) is updated, the key derivation for static signatures changes, effectively invalidating all signatures from previous epochs.





 **Derived From** : Actor Name: Wallet signers meeting multisig threshold | Abuse Title: Simulator Contract Enables Unauthenticated Wallet Execution

## [H-3]. Unauthenticated Execution via Simulator Module Implementation Upgrade

### Finding Severity Justification: The Simulator contract creates a high-severity risk by combining two factors: 1) It inherits from Stage2Module, technically qualifying it as a valid implementation upgrade target within the system's architecture, and 2) It exposes a completely unauthenticated `simulate` function that executes arbitrary calls without reverting (unlike standard simulators). This creates a protocol-level 'trap' where a plausible admin action (upgrading to a protocol-provided module) results in the immediate ability for any attacker to drain the wallet. The failure of the Simulator to enforce read-only behavior via reversion constitutes a code vulnerability in a scoped contract.
## Derived From Pattern/Invariant
Actor Name: Wallet signers meeting multisig threshold | Abuse Title: Simulator Contract Enables Unauthenticated Wallet Execution

## Exploit Type
AuthByPass

## Location
Simulator.simulate

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Simulator` contract is designed for off-chain simulation of wallet transactions. It inherits from `Stage2Module` (making it a valid implementation candidate) and exposes a public `simulate` function. 
```solidity
  function simulate(
    Payload.Call[] calldata _calls
  ) external returns (Result[] memory results) {
    // ... executes calls ...
      if (call.delegateCall) {
        (success) = LibOptim.delegatecall(...);
      } else {
        (success) = LibOptim.call(...);
      }
    // ...
  }
```
Crucially, `simulate` does not perform any signature verification or `onlySelf` checks. If a wallet admin mistakenly or is tricked into upgrading the wallet implementation to the `Simulator` address (via `updateImplementation`), the `simulate` function becomes publicly accessible on the wallet proxy. An attacker can then call `simulate` to execute arbitrary calls (including transfers or delegatecalls) without any authentication, draining the wallet.

## Impact
Complete loss of wallet funds and unauthorized state modification.

## Command to Run Test


## Proof of Concept
1. Attacker (or unwitting admin) calls `execute` on the Wallet to trigger `updateImplementation(address(Simulator))`.
2. The Wallet proxy now delegates all calls to the `Simulator` logic.
3. Attacker calls `Wallet.simulate(calls)` with a payload that transfers all `USDC` to the attacker.
4. `Simulator.simulate` executes the calls immediately without checking `msg.sender` or signatures.
5. Funds are drained.

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { Simulator } from "src/Simulator.sol";
import { Payload } from "src/modules/Payload.sol";

contract MockToken {
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract SimulatorVulnerabilityTest is Test {
    Simulator simulator;
    MockToken token;
    address attacker = address(0xBAD);
    address walletAddress;

    function setUp() public {
        // Deploy Simulator. In a real attack, the Wallet Proxy's implementation 
        // would be updated to this address.
        simulator = new Simulator(address(0));
        walletAddress = address(simulator);
        
        // Setup funds in the "wallet"
        token = new MockToken();
        token.mint(walletAddress, 100 ether);
    }

    function test_SimulatorBypass() public {
        // 1. Prepare payload to transfer funds to attacker
        Payload.Call[] memory calls = new Payload.Call[](1);
        calls[0] = Payload.Call({
            to: address(token),
            value: 0,
            data: abi.encodeWithSelector(MockToken.transfer.selector, attacker, 100 ether),
            gasLimit: 0,
            delegateCall: false,
            onlyFallback: false,
            behaviorOnError: 0
        });

        // 2. Attacker calls simulate(). 
        // Since Simulator is the implementation, this executes in the wallet's context.
        // No auth checks exist in simulate().
        vm.prank(attacker);
        simulator.simulate(calls);

        // 3. Verify funds were drained
        assertEq(token.balanceOf(attacker), 100 ether);
        assertEq(token.balanceOf(walletAddress), 0);
    }
}

## Suggested Mitigation
Modify the `simulate` function to revert with the results at the end of execution. This allows off-chain tools to retrieve simulation results via `eth_call` (by parsing the revert data) while preventing any on-chain state changes or fund movements if the contract is used as an implementation.



