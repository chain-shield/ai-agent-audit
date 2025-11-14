# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : AccountingInvariantViolation

[L-1]. SUBDIGEST leaf sets weight to MAX, then unchecked additions wrap to 0, breaking valid pre-authorized signatures
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 0
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : AccountingInvariantViolation

## [L-1]. SUBDIGEST leaf sets weight to MAX, then unchecked additions wrap to 0, breaking valid pre-authorized signatures

### Finding Severity Justification: The overflow of the weight accumulator after setting it to type(uint256).max in the SUBDIGEST path is real and can cause a valid pre-authorized digest to fail threshold checks if additional weight-contributing leaves are included after it. However, this only impacts the specific signature instance provided by the caller and does not create an asset loss or persistent DoS against the wallet. Callers can trivially avoid it by placing SUBDIGEST as the only weight-contributing leaf (and using only NODEs otherwise) or by ordering so no subsequent additions occur. It does not enable theft, privilege escalation, or permanent bricking; at worst it causes a particular transaction to revert.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
BaseSig.recoverBranch

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In BaseSig.recoverBranch, when encountering FLAG_SUBDIGEST (0x05) and the provided 32-byte value equals opHash, the code sets weight = type(uint256).max inside an unchecked block. Subsequent branches (e.g., FLAG_SIGNATURE_HASH, FLAG_BRANCH) unconditionally do weight += ... within the same unchecked scope. Any further addition after MAX causes modulo-2^256 wraparound, potentially dropping weight to a small number (e.g., 0). This violates the monotonicity of weight accumulation and can DoS otherwise valid pre-authorized operations that rely on SUBDIGEST granting sufficient weight. Vulnerable snippet: 

// Subdigest (0x05)
if (flag == FLAG_SUBDIGEST) {
  (hardcoded, rindex) = _signature.readBytes32(rindex);
  if (hardcoded == _opHash) {
    weight = type(uint256).max;
  }
  ...
}
...
// Signature hash (0x00) adds weight
weight += addrWeight;

Because execution doesn't early-return or saturate after MAX, an attacker can append a trivial signature leaf to wrap weight to 0, making a pre-authorized digest fail threshold checks.

## Impact
A SUBDIGEST (or ANY_ADDRESS_SUBDIGEST) match sets weight to uint256.max inside an unchecked block. If subsequent items in the same branch add weight (e.g., ECDSA/1271/Sapient leaves), the unchecked addition wraps and collapses weight (often to 0), making an otherwise valid pre-authorized digest fail the threshold check. This is a functional DoS on that specific signature encoding. It is not an external attack where an adversary can append arbitrary leaves, because doing so would change the imageHash and be rejected. The issue materializes when the wallet’s configuration orders SUBDIGEST before other weight-contributing leaves, or when the signature encoder opts to represent later signer leaves as weight-contributing (FLAG_SIGNATURE_*), instead of non-contributing (FLAG_ADDRESS). Severity remains low since callers can avoid it via encoding choices, and no assets or privileges are at risk.

## Command to Run Test


## Proof of Concept
1) Prepare any payload and compute its opHash for the target wallet (using the same EIP-712 domain the contract will use). 2) Build a non-chained signature with threshold=1 whose branch encodes: (a) a SUBDIGEST leaf that equals opHash (setting weight = MAX), followed by (b) any weight-contributing leaf (e.g., FLAG_SIGNATURE_HASH with weight=1) that recovers a valid signer. 3) During recovery, the code performs unchecked addition after weight was set to MAX, causing wraparound to 0. 4) As a result, weight < threshold and the signature fails, despite the SUBDIGEST being present. Note: This is a self-induced failure that only occurs if the configuration/signature places SUBDIGEST before other weight-adding leaves, or if the encoder chooses to add weight after SUBDIGEST; it cannot be forced by an external party without breaking the imageHash.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {Stage2Module} from "src/Stage2Module.sol";
import {Payload} from "src/modules/Payload.sol";

contract SubdigestOverflowTest is Test {
  using Payload for Payload.Decoded;

  Stage2Module wallet;
  uint256 pk = 0xA11CE;
  address signer;

  function setUp() public {
    wallet = new Stage2Module(address(0));
    signer = vm.addr(pk);
  }

  // helper: build compact ERC-2098 sig bytes (r, yParityAndS)
  function toCompact(uint8 v, bytes32 r, bytes32 s) internal pure returns (bytes memory) {
    uint256 yParity = uint256(v) - 27;
    uint256 sNum = uint256(s);
    if (yParity == 1) {
      sNum |= (1 << 255);
    } else {
      sNum &= (~(1 << 255));
    }
    bytes32 yParityAndS = bytes32(sNum);
    return abi.encodePacked(r, yParityAndS);
  }

  function test_SubdigestWeightWrapsToZero() public {
    // Build a minimal payload (transactions kind, empty calls)
    Payload.Decoded memory p;
    p.kind = Payload.KIND_TRANSACTIONS;
    p.space = 0;
    p.nonce = 0;

    // Compute opHash for this wallet
    bytes32 opHash = p.hashFor(address(wallet));

    // Non-chained signature, no checkpointer, checkpointSize=0, thresholdSize=1
    bytes1 signatureFlag = 0x00; // normal
    bytes1 thresholdByte = 0x01; // threshold = 1

    // (1) SUBDIGEST leaf: flag=0x5 -> firstByte=0x50, then opHash
    bytes memory subdigestLeaf = abi.encodePacked(bytes1(0x50), opHash);

    // (2) ECDSA leaf: flag=0x0, weight nibble=1 -> firstByte=0x01, then compact sig of opHash by signer
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, opHash);
    bytes memory compact = toCompact(v, r, s);
    bytes memory ecdsaLeaf = abi.encodePacked(bytes1(0x01), compact);

    // Branch = SUBDIGEST then a weight-contributing leaf → triggers MAX + 1 wrap
    bytes memory branch = abi.encodePacked(subdigestLeaf, ecdsaLeaf);
    bytes memory sig = abi.encodePacked(signatureFlag, thresholdByte, branch);

    (uint256 threshold, uint256 weight,,,,) = wallet.recoverPartialSignature(p, sig);

    // threshold=1, but weight wrapped to 0 after MAX + 1 in unchecked block
    assertEq(threshold, 1);
    assertEq(weight, 0);
  }
}


## Suggested Mitigation
Implement saturating addition so that once weight reaches uint256.max it never changes again. Avoid early-returns because the remaining leaves are still needed to reconstruct the imageHash. For every addition site (FLAG_SIGNATURE_HASH, FLAG_SIGNATURE_ETH_SIGN, FLAG_SIGNATURE_ERC1271, FLAG_SIGNATURE_SAPIENT, FLAG_SIGNATURE_SAPIENT_COMPACT, and the sum from FLAG_BRANCH and FLAG_NESTED when applicable), replace `weight += add` with a saturating add, e.g.: if (weight != type(uint256).max) { uint256 remaining = type(uint256).max - weight; if (add >= remaining) { weight = type(uint256).max; } else { weight += add; } } Also apply the same logic after a SUBDIGEST or ANY_ADDRESS_SUBDIGEST match (weight already set to max). This preserves correctness without altering the merkle root computation.



