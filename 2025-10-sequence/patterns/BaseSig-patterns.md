## Verified Patterns Found: 9

## Verified Patterns Found in following Categories:

- ReplayAcrossForksOrL2s
- ChainIdorDomainDrift
- UncheckedLowLevelCallResults
- StandardViolation



## Summary of Patterns

noChainId flag allows cross-chain replay of signatures

ERC-4337 validateUserOp reverts instead of returning SIG_VALIDATION_FAILED

Optional noChainId domain allows cross-chain signature replay

Unhandled behaviorOnError==3 makes failed low-level calls appear successful

Optional noChainId / any-address subdigest allow cross-chain/cross-wallet replays if misused

noChainId and any-address subdigest allow cross-chain and cross-wallet replay

ERC1271 / ERC4337 validation reverts instead of returning failure codes

ERC1271 implementation reverts instead of returning 0x00000000 on invalid signatures

Optional noChainId mode allows multi-chain signature replay

## Patterns



 ### Issue Type: ChainIdorDomainDrift

 ### Relevant Function/Location: BaseSig.recover

 ### Title
noChainId flag allows cross-chain replay of signatures
 ### Description/Code Snippet
The signature scheme supports a `noChainId` mode that zeroes out the chain id in the EIP‑712 domain separator, making signatures valid across forks and sibling L2s.

In `Payload.domainSeparator`:
```solidity
function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32 _domainSeparator) {
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
In `BaseSig.recover`, the low-level signature flag controls this bit:
```solidity
// If the signature type is 10 we do a no chain id signature
_payload.noChainId = signatureFlag & 0x02 == 0x02;
...
opHash = _payload.hash();
```
When bit1 of the signature flag is set, `_payload.noChainId` becomes true and `hash()` uses `chainId = 0` for all EIP‑712 operations. This means the same signed payload+signature can be re-used on any chain where the wallet address and configuration exist, because nothing in the domain separates the chains. There is also no per‑domain (e.g., chain) nonce.

This is an explicit feature (see docs: support for counter-factual / multi-chain state), but from a security-pattern perspective it matches **ChainId or Domain Drift / ReplayAcrossForksOrL2s**: if a user unintentionally signs with `noChainId` (e.g. due to misconfigured client), a relayer or attacker could replay the same transaction on multiple networks where the wallet exists. The contract does not provide any in-protocol guardrail against this; safe use is entirely off-chain.

Relevant locations:
- `Payload.domainSeparator`
- `Payload.hash` / `hashFor`
- `BaseSig.recover` setting `_payload.noChainId` based on the signature flag.
 ### Static Signals
domainSeparator uses _noChainId ? 0 : block.chainid, signatureFlag bit1 toggles _payload.noChainId, no per-domain nonce when chainId is 0
 ### Assets at Risk
wallet funds across multiple chains, cross-chain state / config updates
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC4337v07.validateUserOp

 ### Title
ERC-4337 validateUserOp reverts instead of returning SIG_VALIDATION_FAILED
 ### Description/Code Snippet
ERC4337v07.validateUserOp is supposed to return a uint256 validationData, where a non-zero value indicates signature failure. However, it calls this.isValidSignature(...) and lets any revert bubble up, instead of catching it and returning SIG_VALIDATION_FAILED. Because BaseAuth.signatureValidation can revert for various invalid-signature cases, an attacker can craft signatures that cause the entire validateUserOp call to revert instead of returning a failure code.

Code excerpt:

```solidity
function validateUserOp(
  PackedUserOperation calldata userOp,
  bytes32 userOpHash,
  uint256 missingAccountFunds
) external returns (uint256 validationData) {
  ...
  if (missingAccountFunds != 0) {
    IEntryPoint(entrypoint).depositTo{ value: missingAccountFunds }(address(this));
  }

  if (this.isValidSignature(userOpHash, userOp.signature) != IERC1271_MAGIC_VALUE_HASH) {
    return SIG_VALIDATION_FAILED;
  }

  return 0;
}
```

As shown above, there is no try/catch around isValidSignature. BaseAuth.isValidSignature delegates to signatureValidation, which can revert with:
- InvalidStaticSignatureExpired
- InvalidStaticSignatureWrongCaller
- InvalidSignatureWeight

Thus, instead of returning SIG_VALIDATION_FAILED, validateUserOp may revert outright. This breaks the ERC-4337 expectation that validateUserOp should be a pure validation function returning a status code, and opens DoS patterns where a maliciously-formed static signature causes all user operations for that wallet to revert during simulation.

This is a StandardViolation of ERC-4337’s validateUserOp semantics and directly tied to signature validation behavior.
 ### Static Signals
validateUserOp calls external isValidSignature without try/catch, BaseAuth.signatureValidation can revert on invalid signatures, ERC4337 expects SIG_VALIDATION_FAILED return not revert
 ### Assets at Risk
user operations for affected wallet, relayer / bundler liveness, protocol UX / availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReplayAcrossForksOrL2s

 ### Relevant Function/Location: Payload.domainSeparator

 ### Title
Optional noChainId domain allows cross-chain signature replay
 ### Description/Code Snippet
The payload hashing logic supports a "no chain id" mode where the EIP-712 domain binds only to the wallet address and uses chainId == 0. This is toggled by the signature flag bit 0x02 in BaseSig.recover, which sets _payload.noChainId. When this bit is used, signatures can be replayed across chains and forks where the same wallet address exists.

Relevant pieces:

```solidity
// Payload.sol
function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32 _domainSeparator) {
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

```solidity
// BaseSig.recover
// If the signature type is 10 we do a no chain id signature
_payload.noChainId = signatureFlag & 0x02 == 0x02;
```

If a signer chooses the "no chain id" mode, then the same payload and signature validate on any chain where the wallet exists, because the domain separator uses chainId = 0 rather than binding to the actual chain. This matches the ReplayAcrossForksOrL2s / ChainIdorDomainDrift pattern: signatures are not domain-separated by chain and can be replayed across forks or sibling L2s.

This is apparently intentional for certain subdigest / counter-factual flows, but it is still a potentially dangerous pattern: if used incorrectly, users or integrators might expect chain-specific signatures and instead end up with cross-chain replayability.
 ### Static Signals
optional _noChainId flag, domainSeparator uses chainId 0 when _noChainId=true, BaseSig.recover sets _payload.noChainId based on signatureFlag
 ### Assets at Risk
wallet funds across chains, cross-chain actions, off-chain signed authorizations
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UncheckedLowLevelCallResults

 ### Relevant Function/Location: Calls._execute

 ### Title
Unhandled behaviorOnError==3 makes failed low-level calls appear successful
 ### Description/Code Snippet
In `Calls._execute`, each user-specified call is executed via low-level `LibOptim.call` / `LibOptim.delegatecall`, and the result is handled according to `call.behaviorOnError`:

```solidity
bool success;
if (call.delegateCall) {
  (success) = LibOptim.delegatecall(...);
} else {
  (success) = LibOptim.call(...);
}

if (!success) {
  if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) {
    errorFlag = true;
    emit CallFailed(_opHash, i, LibOptim.returnData());
    continue;
  }

  if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) {
    revert Reverted(_decoded, i, LibOptim.returnData());
  }

  if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) {
    emit CallAborted(_opHash, i, LibOptim.returnData());
    break;
  }
}

emit CallSucceeded(_opHash, i);
```

`behaviorOnError` is decoded from untrusted payload flags as `(flags & 0xC0) >> 6`, so it can take values 0–3. The code only handles 0,1,2 (IGNORE, REVERT, ABORT). If a caller sets the top two bits of `flags` to `11` (`behaviorOnError == 3`), the `if (!success)` block does not match any branch, so execution falls through and emits `CallSucceeded` even though `success == false`.

This is a classic unchecked low-level call result pattern: a failed external call is treated as a success for this reserved error mode. Consequences include:
- Silent failure of external operations (token transfers, protocol interactions) while the wallet logs a success event for that index.
- `errorFlag` remains `false`, so a subsequent `onlyFallback` call that was meant to run only after an error will be skipped.
- Higher-level tooling or off-chain services that rely on `CallSucceeded`/`CallFailed` semantics can be misled about what actually executed, potentially causing inconsistent assumptions about state or security policies.

Because the flags are fully controlled by the signed payload creator, this is primarily an integrity/consistency bug inside the wallet execution engine rather than a direct privilege escalation. However, it fits the UncheckedLowLevelCallResults pattern: a low-level call failure is not handled for one reachable configuration of control flags and is instead reported as success.

 ### Static Signals
behaviorOnError decoded from 2 flag bits (0..3), no branch for behaviorOnError == 3, low-level call failure can fall through to CallSucceeded
 ### Assets at Risk
user funds, wallet execution integrity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ReplayAcrossForksOrL2s

 ### Relevant Function/Location: BaseSig / Payload.recover / recoverBranch / domainSeparator / hash / hashFor

 ### Title
Optional noChainId / any-address subdigest allow cross-chain/cross-wallet replays if misused
 ### Description/Code Snippet
The signature scheme intentionally supports a 'no chain id' mode and counter-factual 'any-address' subdigests. While this is a documented feature, it matches the ReplayAcrossForksOrL2s / ChainIdorDomainDrift pattern: if wallets or signers accidentally use these modes where they intend chain-bound or wallet-bound signatures, the same signature can become valid across multiple chains or wallet instances.

Relevant components:

1) Payload.domainSeparator

```solidity
function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32 _domainSeparator) {
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

2) Payload.hash / hashFor

```solidity
function hash(Decoded memory _decoded) internal view returns (bytes32) {
  bytes32 domain = domainSeparator(_decoded.noChainId, address(this));
  bytes32 structHash = toEIP712(_decoded);
  return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
}

function hashFor(Decoded memory _decoded, address _wallet) internal view returns (bytes32) {
  bytes32 domain = domainSeparator(_decoded.noChainId, _wallet);
  bytes32 structHash = toEIP712(_decoded);
  return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
}
```

3) BaseSig.recover – noChainId flag controlled by signatureFlag

```solidity
function recover(
  Payload.Decoded memory _payload,
  bytes calldata _signature,
  bool _ignoreCheckpointer,
  address _checkpointer
) internal view returns (...) {
  (uint256 signatureFlag, uint256 rindex) = _signature.readFirstUint8();
  ...
  // If the signature type is 10 we do a no chain id signature
  _payload.noChainId = signatureFlag & 0x02 == 0x02;
  ...
  opHash = _payload.hash();
  (weight, imageHash) = recoverBranch(_payload, opHash, _signature[rindex:]);
  ...
}
```

If bit 1 of the top-level signatureFlag is set, the payload’s domain separator uses chainId=0. That opHash will be identical across chains (for the same payload) and can therefore be replayed on any EVM network where the wallet configuration matches.

4) BaseSig.recoverBranch – FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST (8)

```solidity
if (flag == FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST) {
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

Here, the opHash is computed with domainSeparator(_payload.noChainId, wallet=address(0)). This is explicitly counter-factual: the same digest can be valid before deployment, and for any wallet that later chooses to honour that leaf. However, this also means that the same subdigest is not bound to a specific wallet address and can theoretically be reused in other wallets that happen to include the same leaf.

Pattern implications:

* If signers mistakenly set the noChainId bit in contexts where they expect signatures to be chain-specific (for example, when interacting with a single L2 or in cross-chain setups), the same signature can authorize actions on multiple chains.
* If any-address subdigests are used without careful coordination, the same pre-authorized digest could be valid across multiple wallets that embed that leaf, enabling a form of cross-wallet replay of that specific operation.

The protocol documentation acknowledges and leverages these features (e.g., for counter-factual deployments), so this may be acceptable by design. Still, it matches the generic ReplayAcrossForksOrL2s / ChainIdorDomainDrift pattern: domain separation can be intentionally disabled, and a generic digest over address(0) is used, so misconfiguration or misuse by integrators can lead to replayability across forks or sibling L2s.
 ### Static Signals
optional noChainId flag in domainSeparator, hashFor(address(0)) used for any-address subdigest, signer-controlled signatureFlag bit for chainId binding
 ### Assets at Risk
wallet actions across chains, counter-factual pre-authorized operations, cross-chain / cross-wallet replay safety
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReplayAcrossForksOrL2s

 ### Relevant Function/Location: BaseSig.recover / recoverBranch

 ### Title
noChainId and any-address subdigest allow cross-chain and cross-wallet replay
 ### Description/Code Snippet
The signature system explicitly supports a "noChainId" mode and an "any-address" hardcoded subdigest mode that together enable signatures to be replayed across chains and across multiple wallets sharing the same imageHash.

1) noChainId flag in BaseSig / Payload
- In BaseSig.recover, when the signature’s top-level flag has bit1 set (`signatureFlag & 0x02 == 0x02`), we set:
```solidity
_payload.noChainId = signatureFlag & 0x02 == 0x02;
```
- Payload.hash and Payload.hashFor then compute the EIP-712 domain separator with:
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
```
- When `_noChainId == true`, the domain binds `chainId = 0` instead of the real chainid. As a result, the same signature over a given Payload will validate on any chain where the wallet has the same imageHash and nonce space, since the opHash is identical regardless of chain fork or L2.

2) Any-address static subdigest (FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST)
- In BaseSig.recoverBranch, flag 0x08 is defined as "Signature Any address subdigest":
```solidity
if (flag == FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST) {
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
- The hardcoded digest is computed against `address(0)` in the domain separator. The configuration stores this hardcoded digest as a Merkle leaf via `_leafForAnyAddressSubdigest(hardcoded)`. Once such a leaf is in the imageHash, any wallet that uses the *same* imageHash can grant MAX weight whenever a payload’s `hashFor(address(0))` equals that hardcoded value.
- Because `hashFor` uses `_wallet` only in the domain separator and this mode forces `_wallet = address(0)`, the same hardcoded digest applies to **all wallets** sharing the same configuration (imageHash), independent of their actual deployed address.

3) Replay scenarios
- Cross-chain replay: If signers choose the noChainId flag for a transaction payload, opHash is the same on every EVM chain. If the wallet address and configuration (imageHash) exist on multiple chains (as Sequence explicitly supports), the exact same signature can be used to execute the same payload on each chain. This can be leveraged to unintentionally or maliciously spend assets on multiple networks with a single off-chain approval.
- Cross-wallet replay with any-address subdigest: A hardcoded any-address subdigest leaf encodes a particular `_payload.hashFor(address(0))`. Any wallet that upgrades to a configuration whose imageHash includes that leaf will accept *any* call whose payload hashes to that value as having `weight = MAX_UINT`. Since the digest is computed for `wallet = address(0)`, it is not bound to a specific wallet address. If multiple wallets converge to the same imageHash (e.g., a popular configuration template), a single hardcoded any-address subdigest can be used to authorize identical payloads across all those wallets, even if only one wallet was originally intended.

These features may be intentional for counter-factual or multi-chain flows, but from a security perspective they break the standard domain separation invariant that signatures should be bound to a specific chain and contract. This falls under the ReplayAcrossForksOrL2s / ChainIdorDomainDrift pattern: signatures or hardcoded digests are valid across forks, L2s, and multiple wallet instances, enabling replay beyond the original intended domain.

Static signals:
- `_payload.noChainId` toggled by a signature flag and used to set `chainId = 0` in the domain separator.
- `hashFor(_decoded, address(0))` used inside the any-address subdigest branch instead of the actual wallet address.
- `_leafForAnyAddressSubdigest` binds only the digest, not the wallet, so any wallet with the same imageHash and that leaf accepts the same hardcoded digest.

Assets/impact: incorrect or unintended multi-chain / multi-wallet replays can cause:
- repeated execution of the same transaction on several chains,
- use of a pre-authorized any-address digest across multiple wallets sharing the same configuration,
leading to duplicate asset transfers, protocol interactions, or configuration changes.
 ### Static Signals
domainSeparator uses chainId = 0 when _noChainId is true, anyAddressOpHash = _payload.hashFor(address(0)), hardcoded digests not bound to specific wallet address
 ### Assets at Risk
user funds, cross-chain state consistency, multi-wallet configuration safety
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: BaseAuth / ERC4337v07.isValidSignature / signatureValidation / validateUserOp

 ### Title
ERC1271 / ERC4337 validation reverts instead of returning failure codes
 ### Description/Code Snippet
The BaseAuth / ERC4337 integration deviates from ERC-1271 and ERC-4337 expectations by reverting on several signature failure paths instead of returning a failure code, which can break integrations or cause unintended DoS.

Key code paths:

1) BaseAuth.isValidSignature

```solidity
function isValidSignature(bytes32 _hash, bytes calldata _signature) external view returns (bytes4) {
  Payload.Decoded memory payload = Payload.fromDigest(_hash);

  (bool isValid,) = signatureValidation(payload, _signature);
  if (!isValid) {
    return bytes4(0);
  }

  return IERC1271_MAGIC_VALUE_HASH;
}
```

isValidSignature delegates to signatureValidation:

```solidity
function signatureValidation(
  Payload.Decoded memory _payload,
  bytes calldata _signature
) internal view virtual returns (bool isValid, bytes32 opHash) {
  bytes1 signatureFlag = _signature[0];

  if (signatureFlag & 0x80 == 0x80) {
    opHash = _payload.hash();

    (address addr, uint256 timestamp) = _getStaticSignature(opHash);
    if (timestamp <= block.timestamp) {
      revert InvalidStaticSignatureExpired(opHash, timestamp);
    }

    if (addr != address(0) && addr != msg.sender) {
      revert InvalidStaticSignatureWrongCaller(opHash, msg.sender, addr);
    }

    return (true, opHash);
  }

  uint256 threshold;
  uint256 weight;
  bytes32 imageHash;

  (threshold, weight, imageHash,, opHash) = BaseSig.recover(_payload, _signature, false, address(0));

  if (weight < threshold) {
    revert InvalidSignatureWeight(threshold, weight);
  }

  isValid = _isValidImage(imageHash);
}
```

For static signatures, expired signatures or wrong callers revert via InvalidStaticSignatureExpired / InvalidStaticSignatureWrongCaller; for dynamic signatures, insufficient weight reverts via InvalidSignatureWeight. isValidSignature only maps a `false` return to `bytes4(0)`, and does not catch these reverts.

Per ERC-1271, invalid signatures are expected to *return* 0x00000000, not revert. Reverting can break generic callers that expect a simple return-based validity check and may treat reverts as hard errors rather than ‘invalid signature’.

2) ERC4337v07.validateUserOp

```solidity
function validateUserOp(
  PackedUserOperation calldata userOp,
  bytes32 userOpHash,
  uint256 missingAccountFunds
) external returns (uint256 validationData) {
  ...
  if (this.isValidSignature(userOpHash, userOp.signature) != IERC1271_MAGIC_VALUE_HASH) {
    return SIG_VALIDATION_FAILED;
  }

  return 0;
}
```

validateUserOp makes an external call to this.isValidSignature. If any of the revert conditions in signatureValidation are triggered (expired static signature, wrong static signer, signature weight too low), the whole validateUserOp call reverts instead of returning SIG_VALIDATION_FAILED. Under ERC-4337, validateUserOp is supposed to *return* a failure code on invalid signatures; reverting can cause unexpected behavior or DoS.

Pattern match: This is a standards compliance issue around signature validation rather than direct fund theft, but it can break off-chain components (bundlers, relayers, generic ERC-1271 integrators) that rely on consistent return semantics.
 ### Static Signals
IERC1271 implementation reverts on invalid signatures, ERC4337 validateUserOp does not catch ERC1271 reverts, signatureValidation uses custom errors instead of returning false
 ### Assets at Risk
wallet usability, ERC4337 operations, integrations relying on ERC1271
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: BaseAuth.isValidSignature

 ### Title
ERC1271 implementation reverts instead of returning 0x00000000 on invalid signatures
 ### Description/Code Snippet
The wallet’s ERC‑1271 entrypoint `BaseAuth.isValidSignature` does not follow the ERC‑1271 failure semantics and instead bubbles up reverts from internal validation, which can break or DoS integrations that expect a simple magic-value / 0x00000000 result.

Relevant code:

```solidity
function isValidSignature(bytes32 _hash, bytes calldata _signature) external view returns (bytes4) {
    Payload.Decoded memory payload = Payload.fromDigest(_hash);

    (bool isValid,) = signatureValidation(payload, _signature);
    if (!isValid) {
      return bytes4(0);
    }

    return IERC1271_MAGIC_VALUE_HASH;
}
```

`signatureValidation` mixes return-based and revert-based failure paths:

```solidity
function signatureValidation(
  Payload.Decoded memory _payload,
  bytes calldata _signature
) internal view returns (bool isValid, bytes32 opHash) {
  // static-signature flag
  bytes1 signatureFlag = _signature[0];

  if (signatureFlag & 0x80 == 0x80) {
    opHash = _payload.hash();

    (address addr, uint256 timestamp) = _getStaticSignature(opHash);
    if (timestamp <= block.timestamp) {
      revert InvalidStaticSignatureExpired(opHash, timestamp);
    }

    if (addr != address(0) && addr != msg.sender) {
      revert InvalidStaticSignatureWrongCaller(opHash, msg.sender, addr);
    }

    return (true, opHash);
  }

  // dynamic signatures
  uint256 threshold;
  uint256 weight;
  bytes32 imageHash;

  (threshold, weight, imageHash,, opHash) = BaseSig.recover(_payload, _signature, false, address(0));

  // Validate the weight
  if (weight < threshold) {
    revert InvalidSignatureWeight(threshold, weight);
  }

  isValid = _isValidImage(imageHash);
}
```

Failure modes:
- Expired static signatures -> `InvalidStaticSignatureExpired` (revert).
- Static signature from wrong caller -> `InvalidStaticSignatureWrongCaller` (revert).
- Dynamic signatures where `weight < threshold` -> `InvalidSignatureWeight` (revert).
- Only the final imageHash mismatch returns `isValid = false` (leading to `0x00000000`).

Under ERC‑1271 the canonical behavior for invalid signatures is to **return** `0x00000000`, not to revert. Here, many common invalid cases instead revert. This violates the standard and can:
- cause 3rd‑party protocols that `staticcall` `isValidSignature` and expect a return value to revert unexpectedly,
- break ERC‑4337 flow (`validateUserOp` calls `this.isValidSignature`, so these reverts bubble up instead of returning the configured failure code),
- let an attacker construct signatures that deliberately trigger a revert path (e.g., expired static signatures), creating DoS conditions instead of clean failures.

This is a standards-level signature validation bug: the implementation does not consistently use the magic value for failure, which can break integrations relying on ERC‑1271’s specified behavior.
 ### Static Signals
ERC1271 implementation bubbles up custom errors, invalid signatures sometimes revert instead of returning 0x00000000, signatureValidation uses revert paths for common invalid cases
 ### Assets at Risk
wallet funds, integrations depending on ERC1271, ERC4337 user operations
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReplayAcrossForksOrL2s

 ### Relevant Function/Location: BaseSig.recover

 ### Title
Optional noChainId mode allows multi-chain signature replay
 ### Description/Code Snippet
The signature format supports a `noChainId` mode where the EIP-712 domain separator is computed with `chainId = 0`. This is wired via the global signature flag in `BaseSig.recover`:

```solidity
// First byte is the signature flag
(uint256 signatureFlag, uint256 rindex) = _signature.readFirstUint8();
...
// If the signature type is 10 we do a no chain id signature
_payload.noChainId = signatureFlag & 0x02 == 0x02;
```

Then, the payload hash used for signature verification is derived by:

```solidity
function hash(Decoded memory _decoded) internal view returns (bytes32) {
  bytes32 domain = domainSeparator(_decoded.noChainId, address(this));
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

When the `noChainId` flag is set in the signature, the domain binds only to the wallet address but **not** to the actual `block.chainid`. As a consequence, the *same* signed payload + signature pair can be replayed on any chain where the same wallet address exists (Sequence wallets are designed to be multi-chain), because the domain separators are identical (`chainId = 0` on all chains).

This is an intentional feature for some flows (e.g., counter-factual / any‑chain subdigests), but from a vulnerability-pattern perspective it matches a classic **cross-chain replay** risk: any integration that assumes all signatures are chain‑bound may be vulnerable if signers or tools mistakenly enable `noChainId` for operations that should be chain-specific (payments, approvals, etc.).

The core contracts do not add any secondary anti‑replay guard (such as a per-chain nonce or domain‑scoped nonce) when `noChainId` is used, so correctness depends entirely on off‑chain tooling and signer discipline.
 ### Static Signals
optional noChainId mode, domain separator can use chainId=0, no additional per-chain nonce or domain separation when noChainId=true
 ### Assets at Risk
wallet funds, dApp-specific authorizations, session operations across chains
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

