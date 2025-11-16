# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : noChainId signatures enable cross‑chain replay of wallet operations

[M-1]. noChainId signatures allow the same wallet operation to be replayed on every chain where the wallet exists
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresRole



 **Derived From** : Any-address subdigest not bound to wallet enables cross-wallet signature replay

[L-2]. Any-address subdigest signatures are not bound to a specific wallet, enabling cross-wallet and cross-chain replay
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : noChainId signatures enable cross‑chain replay of wallet operations

## [M-1]. noChainId signatures allow the same wallet operation to be replayed on every chain where the wallet exists

### Finding Severity Justification: The behavior is a genuine security weakening: when the noChainId flag is used, the EIP‑712 domain hard‑codes chainId=0, so the same signed payload and signature can be valid on every chain where an identical wallet exists. This breaks the stated invariant that signatures are network-specific and enables a realistic multi-chain replay: one authorization can be reused once per chain to drain comparable assets wherever the wallet holds funds. However, exploitation requires either the wallet owner or an integrating frontend to explicitly produce such a noChainId signature; the default and recommended flow is chain-bound signatures. It is thus a significant but configuration-dependent attack vector, fitting a Medium impact/likelihood profile rather than guaranteed catastrophic loss for ordinary users.
## Derived From Pattern/Invariant
noChainId signatures enable cross‑chain replay of wallet operations

## Exploit Type
SignatureReplay

## Location
BaseSig.recover

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
BaseSig.recover supports a "noChainId" mode that deliberately omits the chainId from the EIP‑712 domain. When bit 1 of the top‑level signature flag is set (0x02), recover mutates the payload and sets `_payload.noChainId = true` before hashing:

```solidity
// BaseSig.recover
// If the signature type is 10 we do a no chain id signature
_payload.noChainId = signatureFlag & 0x02 == 0x02;
...
opHash = _payload.hash();
```

`Payload.hash()` then builds the domain with `chainId = 0` when `_decoded.noChainId` is true:

```solidity
function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32) {
  return keccak256(
    abi.encode(
      EIP712_DOMAIN_TYPEHASH,
      EIP712_DOMAIN_NAME_SEQUENCE,
      EIP712_DOMAIN_VERSION_SEQUENCE,
      _noChainId ? uint256(0) : uint256(block.chainid),
      _wallet
    )
  );
}

function hash(Decoded memory _decoded) internal view returns (bytes32) {
  bytes32 domain = domainSeparator(_decoded.noChainId, address(this));
  bytes32 structHash = toEIP712(_decoded);
  return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
}
```

As a result, for a given wallet address and payload (including nonce/space), `opHash` is *identical on all chains* whenever `noChainId` is enabled. Sequence wallets are designed to exist at the same address on multiple chains. Each chain maintains its own nonce storage, so the same signed payload can be executed once per chain.

This means that if a signer (or integrating frontend) produces a high‑value transaction with the `noChainId` flag set, a malicious relayer can replay that single signature on *every chain where the wallet is deployed* and where it has funds. There is no on‑chain mechanism to bind such signatures to a specific chain or to prevent per‑chain replays; the only replay protection (the nonce) is local to each chain.

This directly contradicts the stated invariant "Domain‑Separated Signatures" which claims that signatures are network‑specific and cannot be replayed cross‑chain. In `noChainId` mode, that invariant no longer holds, but this is not enforced or surfaced at the contract level.

## Impact
A single `noChainId` signature authorizing a wallet operation (e.g., ERC20 transfer or upgrade) can be reused to perform the same operation once per chain where the wallet exists and holds funds. This enables multi‑chain double‑spend of the wallet's assets: the owner signs once, but the attacker/relayer can drain equivalent funds on each chain.

## Command to Run Test


## Proof of Concept
1. Assume a Sequence wallet `W` is deployed on chains A and B at the same address, with identical configuration. Each chain has independent ERC20 balances and nonces.
2. The wallet owner uses a frontend which (intentionally or accidentally) constructs a payload `P` of kind `KIND_TRANSACTIONS` that transfers all of W's ERC20 balance on chain A to `attackerEOA`, and encodes the top‑level signature flag with bit1 set (`signatureFlag & 0x02 != 0`), enabling `noChainId`.
3. The owner signs `P` off‑chain with their private key, producing ECDSA signature `S`. Because of `noChainId`, the EIP‑712 hash `opHash = Payload.hash(P)` uses chainId = 0 (and `verifyingContract = W`), so `opHash` is independent of the actual chain.
4. The attacker (or any relayer) submits `(P,S)` to `W.execute` on chain A. `BaseSig.recover` sets `_payload.noChainId = true`, computes `opHash`, verifies the signature, and the wallet consumes nonce space/nonce for chain A and transfers ERC20 from W(A) to `attackerEOA`.
5. The attacker reuses the *same* `(P,S)` on chain B, calling `W.execute` on chain B. Since `domainSeparator` uses `chainId = 0`, `opHash` is identical; `ecrecover` succeeds, and the wallet consumes nonce for chain B and again transfers the ERC20 balance from W(B) to `attackerEOA`.
6. The same replay can be repeated on every other chain where `W` exists at the same address and has funds, causing repeated withdrawal or state changes even though the user only signed once.

The Foundry test below demonstrates that a `noChainId` signature produces the same `opHash` and validates successfully when `block.chainid` is changed, which is the core condition enabling this cross‑chain replay.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {BaseSig} from "../src/modules/auth/BaseSig.sol";
import {Payload} from "../src/modules/Payload.sol";

contract SigReplayTest is Test {
  using Payload for Payload.Decoded;

  SigHelper helper1;

  function setUp() public {
    helper1 = new SigHelper();
  }

  function test_NoChainIdSignatureReplayAcrossChainIds() public {
    // Build a simple MESSAGE payload
    Payload.Decoded memory payload;
    payload.kind = Payload.KIND_MESSAGE;
    payload.message = bytes("hello world");

    // Compute the EIP-712 hash with noChainId=true (domain uses chainId=0)
    bytes32 opHash = helper1.hashNoChainId(payload);

    // Sign that hash with some EOA
    uint256 pk = 0xA11CE;
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, opHash);

    (bytes32 rOut, bytes32 yParityAndS) = _toCompact(v, r, s);

    // Build a BaseSig top-level signature:
    // - signatureFlag = 0x02 : noChainId=1, no checkpointer, not chained, checkpointSize=0, thresholdSize=1 byte
    // - threshold = 1
    // - one branch item: FLAG_SIGNATURE_HASH (0x0) with weight=1 (low nibble=1), then 64-byte compact signature
    bytes memory sig = abi.encodePacked(
      bytes1(0x02), // signatureFlag: noChainId
      bytes1(0x01), // threshold = 1
      bytes1(0x01), // branch firstByte: flag=0 (SIG_HASH), weight=1
      rOut,
      yParityAndS
    );

    // Simulate chain A
    vm.chainId(1);
    (uint256 thr1, uint256 w1, , , bytes32 opHash1) = helper1.recoverSig(payload, sig);

    // Simulate chain B
    vm.chainId(2);
    (uint256 thr2, uint256 w2, , , bytes32 opHash2) = helper1.recoverSig(payload, sig);

    // Threshold and weight are satisfied on both chains
    assertEq(thr1, 1);
    assertEq(w1, 1);
    assertEq(thr2, 1);
    assertEq(w2, 1);

    // Crucially, opHash is identical even though chainId changed,
    // proving the signature is chain-agnostic in noChainId mode.
    assertEq(opHash1, opHash2, "opHash should be identical across chainIds when noChainId is set");
  }

  function _toCompact(uint8 v, bytes32 r, bytes32 s) internal pure returns (bytes32 rOut, bytes32 yParityAndS) {
    rOut = r;
    uint256 yParity;
    if (v == 27) yParity = 0;
    else if (v == 28) yParity = 1;
    else revert("bad v");

    uint256 sInt = uint256(s);
    require(sInt >> 255 == 0, "s too large");

    yParityAndS = bytes32((yParity << 255) | sInt);
  }
}

contract SigHelper {
  using Payload for Payload.Decoded;

  function recoverSig(
    Payload.Decoded memory payload,
    bytes calldata signature
  ) external view returns (uint256 threshold, uint256 weight, bytes32 imageHash, uint256 checkpoint, bytes32 opHash) {
    (threshold, weight, imageHash, checkpoint, opHash) = BaseSig.recover(payload, signature, false, address(0));
  }

  function hashNoChainId(Payload.Decoded memory payload) external view returns (bytes32) {
    payload.noChainId = true;
    return payload.hash();
  }
}


## Suggested Mitigation
At the contract level, restrict or hard‑scope the `noChainId` mode so it cannot be used for arbitrary transaction payloads:

1. In `BaseSig.recover` or `BaseAuth.signatureValidation`, reject `noChainId` signatures for `KIND_TRANSACTIONS` payloads and only allow them for `KIND_CONFIG_UPDATE` (or explicitly whitelisted payload kinds) where cross‑chain reuse is intended.
2. Alternatively, extend the domain for `noChainId` signatures with an explicit `targetChainId` field inside the `Payload` struct and include that in `toEIP712`. This preserves cross‑chain configurability while still binding a signature to the chain where it should be executed.
3. As an additional defense, wallets could expose a configuration bit that completely disables `noChainId` signatures for user transaction flows, enforced on‑chain, so applications cannot inadvertently opt into chain‑agnostic signatures.





 **Derived From** : Any-address subdigest not bound to wallet enables cross-wallet signature replay

## [L-2]. Any-address subdigest signatures are not bound to a specific wallet, enabling cross-wallet and cross-chain replay

### Finding Severity Justification: The behavior described is real: FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST intentionally computes the digest over a domain with verifyingContract = address(0) (and optionally chainId = 0 when noChainId is set). This means a hardcoded any‑address subdigest can be reused across multiple wallets that share the same configuration and across chains when noChainId is used. However, the impact is limited to external integrations that incorrectly treat a naked digest as a globally unique proof of a single wallet’s identity. On‑chain, the wallet itself still binds normal signatures to its own address and chain; any‑address subdigests are explicitly documented as counterfactual and require deliberate inclusion as leaves in the configuration tree. There is no direct loss or theft of assets from the Sequence protocol itself, only potential misuse by third-party systems that ignore the standard (walletAddress, digest) binding. This aligns with a documentation / misuse risk rather than a direct protocol vulnerability, thus Low.
## Derived From Pattern/Invariant
Any-address subdigest not bound to wallet enables cross-wallet signature replay

## Exploit Type
SignatureReplay

## Location
BaseSig.recoverBranch

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Within `BaseSig.recoverBranch`, the `FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST` path intentionally computes a subdigest against a domain where `verifyingContract` is `address(0)` instead of the actual wallet address. The code is:

```solidity
// Signature Any address subdigest (0x08)
// similar to subdigest, but allows for counter-factual payloads
if (flag == FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST) {
  // A hardcoded always accepted digest
  // it pushes the weight to the maximum
  bytes32 hardcoded;
  (hardcoded, rindex) = _signature.readBytes32(rindex);
  bytes32 anyAddressOpHash = _payload.hashFor(address(0));
  if (hardcoded == anyAddressOpHash) {
    weight = type(uint256).max;
  }

  bytes32 node = _leafForAnyAddressSubdigest(hardcoded);
  root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
  continue;
}
```

The helper computes the comparison hash as:

```solidity
function hashFor(Decoded memory _decoded, address _wallet) internal view returns (bytes32) {
  bytes32 domain = domainSeparator(_decoded.noChainId, _wallet);
  bytes32 structHash = toEIP712(_decoded);
  return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
}

function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32) {
  return keccak256(
    abi.encode(
      EIP712_DOMAIN_TYPEHASH,
      EIP712_DOMAIN_NAME_SEQUENCE,
      EIP712_DOMAIN_VERSION_SEQUENCE,
      _noChainId ? uint256(0) : uint256(block.chainid),
      _wallet
    )
  );
}
```

Crucially, for ANY‑ADDRESS subdigests the domain is built with `_wallet = address(0)`, so:

* The EIP‑712 domain for the subdigest is **not bound to the actual wallet address**.
* If `_decoded.noChainId` is set (via the top‑level signature flag), the domain also uses `chainId = 0`, making the digest **chain‑agnostic** as well.

The leaf committed into the configuration Merkle tree is:

```solidity
function _leafForAnyAddressSubdigest(bytes32 _anyAddressSubdigest) internal pure returns (bytes32) {
  return keccak256(abi.encodePacked("Sequence any address subdigest:\n", _anyAddressSubdigest));
}
```

Consequences:

* Any two wallets that upgrade to an identical configuration (`imageHash`) containing the same ANY‑ADDRESS subdigest leaf for a given payload will accept the **same hardcoded digest**.
* Because the digest is computed with `verifyingContract = address(0)` and optionally `chainId = 0`, the same 32‑byte value is valid across all wallets sharing that configuration and across all chains.
* For `KIND_DIGEST` / ERC‑1271 flows (where `BaseAuth.isValidSignature` wraps a digest into a `Payload` and passes it through `BaseSig.recover`), there is **no nonce** and no per‑wallet binding on the digest path. A pre‑authorized ANY‑ADDRESS subdigest can therefore act as a globally reusable proof for that digest on any wallet whose configuration includes the corresponding leaf.

This breaks the usual invariant that signatures are bound to a specific contract address (wallet) and optionally a specific chain. Integrations that assume "only this wallet can ever produce a valid signature for this digest" can be surprised: as soon as another wallet adopts the same configuration (or is deliberately configured that way), the same ANY‑ADDRESS subdigest signature becomes valid there as well.

## Impact
Any-address subdigest signatures are intentionally defined over a domain with `verifyingContract = address(0)` and optionally `chainId = 0` (when `noChainId` is set). As a result, a given ANY-ADDRESS subdigest leaf corresponds to the *same* 32-byte digest for all wallets and all chains that share the same payload and `noChainId` setting. If multiple wallets deliberately adopt an identical configuration (same `imageHash`) containing that leaf, then a single hardcoded ANY-ADDRESS subdigest can satisfy the threshold on each of those wallets, including in ERC-1271 / `KIND_DIGEST` flows where there is no additional nonce binding. This does not by itself let an attacker bypass Sequence’s on-chain authorization model, because inclusion of such leaves is explicit and opt-in per configuration, and the wallet still enforces its configured `imageHash` and thresholds. The risk arises for integrators that incorrectly treat a bare digest as a globally unique proof of a single wallet’s identity (rather than binding authorizations to `(walletAddress, digest)` and respecting the documented semantics of ANY-ADDRESS subdigests). In those mis-integration scenarios, the same pre-authorized digest may be reused across wallets or chains that share the configuration, enabling cross-wallet logical impersonation or double-claiming at the application layer, but not direct theft of funds from correctly implemented on-chain contracts.

## Command to Run Test


## Proof of Concept
This PoC focuses on the ERC-1271 / KIND_DIGEST path, and on the fact that ANY-ADDRESS subdigests are computed with `verifyingContract = address(0)` and optionally `chainId = 0`.

High-level steps:

1. Off-chain, construct a digest `D` that some external system will treat as an authorization token. For example, `D = keccak256("airdrop-claim:user-123")`.
2. Build a `Payload.Decoded` of kind `KIND_DIGEST` with `digest = D` and `noChainId = true`. Let this be `P`.
3. Compute the ANY-ADDRESS subdigest for this payload using the Sequence EIP-712 rules but with `verifyingContract = address(0)`:
   - `P.noChainId = true`
   - `P_hash_any = Payload.hashFor(P, address(0))`
   Because `noChainId = true`, the domain separator uses `chainId = 0`. Because `_wallet = address(0)`, `verifyingContract` is also 0. Thus `P_hash_any` is independent of both wallet address and chain.
4. Two Sequence wallets, `W1` and `W2`, are configured (either at deployment or via chained config updates) to share the same `imageHash` that includes an ANY-ADDRESS subdigest leaf for this exact value:
   - the tree contains `_leafForAnyAddressSubdigest(P_hash_any)` as one of its leaves;
   - top-level threshold is chosen such that this leaf’s `weight = type(uint256).max` is sufficient on its own.
   Since the `imageHash` is identical, both wallets enforce the same configuration.
5. Now construct a signature `S` for `P` that consists solely of this ANY-ADDRESS subdigest branch item:
   - set the top-level `signatureFlag` to have `noChainId` bit set and a non-chained, non-checkpointer signature (e.g., `0x02`, then a zero checkpoint, threshold=1);
   - in the branch, emit one item with flag `FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST` and append the 32-byte `P_hash_any`.
   This `S` can be constructed by *any* party who knows `P_hash_any`; no private key is needed.
6. On chain X, an external contract or off-chain backend calls `W1.isValidSignature(D, S)`. Inside `BaseAuth.isValidSignature`:
   - the digest `D` is wrapped as a `KIND_DIGEST` payload `P` and passed into `BaseSig.recover`;
   - because the top-level flag sets `noChainId`, `P.noChainId = true`;
   - in `recoverBranch`, the ANY-ADDRESS branch checks `hardcoded == _payload.hashFor(address(0))`, i.e. `hardcoded == P_hash_any`;
   - this comparison succeeds and sets `weight = type(uint256).max`, and because the Merkle proof matches the configured `imageHash`, the signature is accepted and `isValidSignature` returns the ERC-1271 magic value.
7. On chain Y (possibly different from X), repeat the same call against wallet `W2`: `W2.isValidSignature(D, S)` with the *same* `D` and `S`.
   - The payload constructed internally is identical (`KIND_DIGEST`, `digest = D`, `noChainId = true`);
   - `Payload.hashFor(P, address(0))` again returns the same `P_hash_any` because the domain uses `chainId = 0` and `verifyingContract = address(0)`;
   - the ANY-ADDRESS branch again matches and grants max weight, and the same `imageHash` reconstruction succeeds since `W2` uses the same config.
8. From the perspective of any integration that keys authorizations *only* by `D` (for example, "if we see any wallet return ERC-1271 valid for digest D, we consider D consumed"), this means the logical approval represented by `D` can be replayed across `W1` and `W2`. If `D` was intended to be bound uniquely to `W1`, this assumption is violated: any wallet that adopts the same configuration leaf for `P_hash_any` can produce a valid ERC-1271 response for the same digest.

This demonstrates that ANY-ADDRESS subdigests are not bound to a particular wallet or chain, and that a single hardcoded ANY-ADDRESS subdigest can be reused across multiple wallets and chains that share the same configuration, specifically in the `KIND_DIGEST` / ERC-1271 validation path.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {BaseSig} from "../src/modules/auth/BaseSig.sol";
import {Payload} from "../src/modules/Payload.sol";

contract AnyAddressSubdigestReplayTest is Test {
  using Payload for Payload.Decoded;

  SigHelper helper1;
  SigHelper helper2;

  function setUp() public {
    helper1 = new SigHelper();
    helper2 = new SigHelper();
  }

  function test_anyAddressSubdigest_sameDigestAcceptedByTwoHelpers() public {
    // Construct a KIND_DIGEST payload around some arbitrary digest D
    bytes32 D = keccak256("airdrop-claim:user-123");

    Payload.Decoded memory payload;
    payload.kind = Payload.KIND_DIGEST;
    payload.digest = D;
    payload.noChainId = true; // we want chain-agnostic behavior

    // Compute the ANY-ADDRESS subdigest with noChainId=true and wallet=address(0)
    // Because chainId and verifyingContract are fixed to 0, this value is
    // independent of the eventual wallet contract and the current chain.
    bytes32 anyAddrDigest = helper1.anyAddressOpHash(payload);

    // Build a minimal signature that uses ONLY the ANY-ADDRESS subdigest leaf.
    // Top-level signatureFlag layout we choose:
    // - bit0 (chained) = 0 (normal)
    // - bit1 (noChainId) = 1
    // - bits4..2 (checkpoint size) = 0 (0 bytes => checkpoint = 0)
    // - bit5 (threshold size) = 0 (1 byte threshold)
    // - bit6 (checkpointer) = 0
    // So signatureFlag = 0x02.
    // After that:
    // - checkpoint: size=0 => encoded as 0
    // - threshold: 1 byte, value = 1
    // Branch:
    //   firstByte: top nibble 0x8 (FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST), free nibble ignored => 0x80
    //   then 32-byte hardcoded anyAddrDigest
    bytes memory sig = abi.encodePacked(
      bytes1(0x02),       // signatureFlag: noChainId
      bytes1(0x00),       // checkpoint, size=0 => value 0
      bytes1(0x01),       // threshold = 1
      bytes1(0x80),       // branch item: flag=8 (ANY_ADDRESS_SUBDIGEST)
      anyAddrDigest       // hardcoded subdigest
    );

    // Fix chainId to some value; because payload.noChainId=true, it should not matter
    vm.chainId(1);

    // Recover through helper1 and helper2 using the same payload and signature
    (uint256 thrA, uint256 wA, bytes32 imageHashA,, bytes32 opHashA) = helper1.recoverSig(payload, sig);
    (uint256 thrB, uint256 wB, bytes32 imageHashB,, bytes32 opHashB) = helper2.recoverSig(payload, sig);

    // opHash is computed with domainSeparator(_decoded.noChainId, address(this)),
    // so it is bound to the helper contract address and will differ across helpers.
    assertTrue(opHashA != opHashB, "Per-helper opHash should differ (bound to address(this))");

    // However, the ANY-ADDRESS subdigest branch is evaluated against hashFor(address(0)),
    // which is identical in both helpers. The resulting weight and reconstructed root
    // are thus the same across helpers that share the same logical configuration.
    assertEq(thrA, 1);
    assertEq(thrB, 1);
    assertEq(wA, type(uint256).max);
    assertEq(wB, type(uint256).max);
    assertEq(imageHashA, imageHashB, "Same ANY-ADDRESS subdigest signature yields same imageHash across different helpers");
  }
}

contract SigHelper {
  using Payload for Payload.Decoded;

  function recoverSig(
    Payload.Decoded memory payload,
    bytes calldata signature
  ) external view returns (uint256 threshold, uint256 weight, bytes32 imageHash, uint256 checkpoint, bytes32 opHash) {
    (threshold, weight, imageHash, checkpoint, opHash) = BaseSig.recover(payload, signature, false, address(0));
  }

  function anyAddressOpHash(Payload.Decoded memory payload) external view returns (bytes32) {
    // Domain for ANY-ADDRESS is (noChainId ? 0 : chainId, verifyingContract = address(0))
    // Caller must set payload.noChainId as desired before calling.
    return payload.hashFor(address(0));
  }
}


## Suggested Mitigation
Because ANY-ADDRESS subdigests are explicitly designed for counterfactual use, changing them to be bound to a specific wallet address would break documented functionality. Instead:

1. **Restrict where ANY-ADDRESS subdigests can apply:**
   - Consider disallowing `FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST` for generic `KIND_DIGEST` payloads that are surfaced via ERC-1271, or gate it behind an explicit configuration flag so a wallet must opt in to using ANY-ADDRESS leaves for digest-based validation.
   - Alternatively, require that ANY-ADDRESS subdigests only be used for narrowly scoped payload kinds (e.g., specific `KIND_MESSAGE` or `KIND_CONFIG_UPDATE` variants) where cross-wallet reuse is acceptable and documented.

2. **Encourage per-wallet salting where appropriate:**
   - For high-value off-chain authorizations that should not be shareable, recommend that integrators include a wallet-specific salt (e.g., the wallet address or another per-wallet identifier) inside the data they hash into the `KIND_DIGEST` or `KIND_MESSAGE` payload, so that the resulting digest is inherently wallet-bound even if validated via an ANY-ADDRESS subdigest.

3. **Strengthen integrator guidance and API contracts:**
   - Clearly document that ANY-ADDRESS subdigests are *not* globally unique proofs of a single wallet’s identity and can be valid for any wallet that adopts the same configuration leaf.
   - Recommend that integrators never key authorizations solely by digest for Sequence wallets; instead they should bind permissions to `(walletAddress, digest)` and, where feasible, inspect configuration metadata to detect whether a wallet’s config includes ANY-ADDRESS subdigest leaves.
   - In SDKs and higher-level APIs, expose helpers that distinguish between signatures that relied on ANY-ADDRESS leaves vs "normal" wallet-bound signatures, so applications can apply stricter policies or reject the former in sensitive flows.

These measures preserve the intended counterfactual behavior of ANY-ADDRESS subdigests while reducing the likelihood that they will be misused as globally unique, wallet-bound proofs by external systems.



