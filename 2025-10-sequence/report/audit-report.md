# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : noChainId flag enables cross-chain replay of transaction signatures

[M-1]. BaseSig.recover permits chain-agnostic EIP-712 for transactions, enabling cross-chain replay when wallet address matches
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Documentation highlights domain-separated, chain-specific signatures as an invariant, but also documents a noChainId mode and recommends chain-agnostic behavior for configuration updates. It is unclear if allowing noChainId for transaction payloads is an intentional feature or an oversight. Given this ambiguity and external deployment/config assumptions (factory address parity across chains), confidence is not maximal.
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : noChainId flag enables cross-chain replay of transaction signatures

## [M-1]. BaseSig.recover permits chain-agnostic EIP-712 for transactions, enabling cross-chain replay when wallet address matches

### Finding Severity Justification: Setting the noChainId bit in the top-level signature allows hashing transactions with chainId=0, making the EIP-712 digest chain-agnostic. If a wallet address is identical across chains and the nonce on the target chain is unused, a valid transaction signature can be replayed cross-chain, potentially moving real funds. Impact can be large, but exploitation requires external conditions (same wallet address across chains) and the signer intentionally using the noChainId signature type for a transaction. Hence, realistic but conditional: Medium.
## Derived From Pattern/Invariant
noChainId flag enables cross-chain replay of transaction signatures

## Exploit Type
SignatureReplay

## Location
BaseSig.recover

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Documentation highlights domain-separated, chain-specific signatures as an invariant, but also documents a noChainId mode and recommends chain-agnostic behavior for configuration updates. It is unclear if allowing noChainId for transaction payloads is an intentional feature or an oversight. Given this ambiguity and external deployment/config assumptions (factory address parity across chains), confidence is not maximal.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
BaseSig.recover sets _payload.noChainId for any payload when the top-level signatureFlag has bit 0x02 set, without restricting it to off-chain messages or digests. Consequently, Payload.domainSeparator() binds chainId=0 instead of the current chain for transaction payloads as well. Vulnerable excerpts:

In BaseSig.recover():
  // If the signature type is 10 we do a no chain id signature
  _payload.noChainId = signatureFlag & 0x02 == 0x02;
  ...
  opHash = _payload.hash();

In Payload.domainSeparator():
  return keccak256(abi.encode(
    EIP712_DOMAIN_TYPEHASH,
    EIP712_DOMAIN_NAME_SEQUENCE,
    EIP712_DOMAIN_VERSION_SEQUENCE,
    _noChainId ? uint256(0) : uint256(block.chainid),
    _wallet
  ));

Because no guard enforces that 'noChainId' remains limited to KIND_MESSAGE/KIND_DIGEST, transaction payloads can be signed with chainId=0. If the wallet address is identical across chains and the nonce on the target chain is still unused, observing a valid signature on chain A allows a permissionless replay on chain B to perform the same wallet action (e.g., token transfer).

## Impact
Cross-chain replay of a signed transaction drains funds on a second chain when the deterministic wallet address matches and the corresponding nonce space is unused on that chain.

## Command to Run Test


## Proof of Concept
1) Victim signs a transaction payload (KIND_TRANSACTIONS) with a top-level signature flag setting bit 0x02 (noChainId), producing an EIP-712 digest with chainId=0.
2) Attacker copies the signed calldata from chain A.
3) On chain B, where the same wallet address exists (deterministic deployment) and the relevant nonce is unused, attacker submits the same payload/signature.
4) Since the opHash is identical (chainId=0) and verifyingContract is the same, BaseSig.recover validates the signature and the wallet executes the same call(s), transferring funds on chain B.

The following test demonstrates that a transaction signature flagged with noChainId is independent of block.chainid (i.e., chain-agnostic), proving replay feasibility if the wallet address and nonce context permit it.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Payload} from "src/modules/Payload.sol";
import {BaseSig} from "src/modules/auth/BaseSig.sol";

contract SigHarness {
  using Payload for Payload.Decoded;

  function computeDigest(Payload.Decoded memory payload, bool noChainId) external view returns (bytes32) {
    payload.noChainId = noChainId;
    return payload.hash();
  }

  function recoverTx(
    Payload.Decoded memory payload,
    bytes calldata signature
  ) external view returns (uint256 threshold, uint256 weight, bytes32 imageHash, uint256 checkpoint, bytes32 opHash) {
    return BaseSig.recover(payload, signature, false, address(0));
  }
}

contract NoChainIdReplayTest is Test {
  using Payload for Payload.Decoded;

  SigHarness h;
  uint256 signerPk;
  address signer;

  function setUp() public {
    h = new SigHarness();
    signerPk = 0xB0B; // test key
    signer = vm.addr(signerPk);
  }

  function test_NoChainId_AllowsTxSigCrossChainReplay() public {
    // Build a minimal transaction payload
    Payload.Decoded memory p;
    p.kind = Payload.KIND_TRANSACTIONS;
    p.space = 0;
    p.nonce = 0;
    p.calls = new Payload.Call[](1); // one empty call; defaults are fine for hashing

    // Compute digest with noChainId=true bound to harness address
    bytes32 digestNoChain = h.computeDigest(p, true);

    // Sign digest (EIP-712)
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digestNoChain);
    bytes32 yParityAndS = s;
    if (v == 28) {
      yParityAndS = bytes32(uint256(s) | (1 << 255));
    }

    // Top-level signature format:
    // signatureFlag: 0x02 (noChainId), checkpointSize=0, thresholdSize=1-byte
    // checkpoint bytes omitted (size=0)
    // threshold = 1 (1 byte)
    // branch item: FLAG_SIGNATURE_HASH (0x0) with weight=1 => firstByte = 0x01
    bytes memory sig = abi.encodePacked(
      bytes1(0x02),         // signatureFlag: noChainId set
      bytes1(uint8(0x01)),  // threshold = 1
      bytes1(uint8(0x01)),  // branch item header: FLAG_SIGNATURE_HASH with weight = 1
      r,
      yParityAndS
    );

    // Recover on current chain
    (uint256 threshold, uint256 weight, , , bytes32 opHashA) = h.recoverTx(p, sig);
    assertEq(threshold, 1);
    assertEq(weight, 1);
    assertEq(opHashA, digestNoChain);

    // Change chain id and recover again: digest remains identical (chain-agnostic)
    uint256 before = block.chainid;
    vm.chainId(before + 1);
    (, , , , bytes32 opHashB) = h.recoverTx(p, sig);
    assertEq(opHashB, digestNoChain);

    // Restore chain id
    vm.chainId(before);
  }
}


## Suggested Mitigation
Gate the noChainId flag by payload kind. Only allow _payload.noChainId = true for KIND_MESSAGE and KIND_DIGEST. For transaction payloads (KIND_TRANSACTIONS) and config updates (KIND_CONFIG_UPDATE), ignore bit 0x02 and always bind to the current chainId. Alternatively, add a configurable allowlist that defaults to disallowing chain-agnostic transaction signatures. Example fix: if (_payload.kind != Payload.KIND_MESSAGE && _payload.kind != Payload.KIND_DIGEST) { _payload.noChainId = false; } prior to hashing.



