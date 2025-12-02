# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern
gpt-5.1 "low"  3X - $10


 **Derived From** : StandardViolation

 ### Relevant Function/Location: BaseAuth.signatureValidation / isValidSignature / ERC4337v07.validateUserOp

 ### Title
Invalid signatures can revert ERC1271 and ERC4337 flows instead of returning failure codes

[L-1]. Invalid signatures revert in BaseAuth / ERC4337v07, violating ERC-1271 and ERC-4337 semantics and enabling DoS
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : StateGrowthOrStorageBloat

 ### Relevant Function/Location: Recovery.queuePayload

 ### Title
Recovery module unbounded queuedPayloadHashes array causes storage bloat

[L-2]. Recovery.queuePayload appends to queuedPayloadHashes without any pruning, causing unbounded storage growth
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole



 **Derived From** : Checkpointer callback can grief all signatures using that configuration

[L-3]. Malicious or broken checkpointer can permanently DoS all operations for affected wallet configurations
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresRole
[L-4]. Untrusted ICheckpointer.snapshotFor callbacks can permanently grief signature validation for affected configs
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresRole


### Number of Findings
- C: 0
- H: 0
- M: 0
- L: 4
- I: 0

##Findings by Pattern


 **Derived From** : StandardViolation

 ### Relevant Function/Location: BaseAuth.signatureValidation / isValidSignature / ERC4337v07.validateUserOp

 ### Title
Invalid signatures can revert ERC1271 and ERC4337 flows instead of returning failure codes

## [L-1]. Invalid signatures revert in BaseAuth / ERC4337v07, violating ERC-1271 and ERC-4337 semantics and enabling DoS

### Finding Severity Justification: The report correctly identifies that BaseAuth.signatureValidation reverts on common invalid-signature paths (expired/wrong-caller static signatures, insufficient weight), which bubbles through isValidSignature and validateUserOp. This is a standards-compliance issue (ERC-1271 prefers returning 0x00000000 on failure). However, practical impact is limited: ERC-4337 bundlers/EntryPoint treat a revert during validation as an invalid operation, not a protocol-wide DoS, and it only affects the provided operation, not other users or future valid ops. Thus, while it can inconvenience integrators expecting non-reverting ERC-1271, it does not risk assets or create a realistic DoS beyond the failing op.
## Derived From Pattern/Invariant
StandardViolation

 ### Relevant Function/Location: BaseAuth.signatureValidation / isValidSignature / ERC4337v07.validateUserOp

 ### Title
Invalid signatures can revert ERC1271 and ERC4337 flows instead of returning failure codes

## Exploit Type
StandardViolation

## Location
BaseAuth / ERC4337v07.isValidSignature / validateUserOp

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The wallet uses BaseAuth.signatureValidation() for both ERC-1271 (isValidSignature) and ERC-4337 (validateUserOp via this.isValidSignature). In multiple invalid-signature cases, signatureValidation **reverts** instead of returning a failure code, violating both ERC-1271 and ERC-4337 behavioral expectations and enabling denial-of-service on signature validation.

Relevant code:

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

function isValidSignature(bytes32 _hash, bytes calldata _signature) external view returns (bytes4) {
  Payload.Decoded memory payload = Payload.fromDigest(_hash);
  (bool isValid,) = signatureValidation(payload, _signature);
  if (!isValid) {
    return bytes4(0);
  }
  return IERC1271_MAGIC_VALUE_HASH;
}
```

For static signatures (bit 0x80 set) and for low signature weight, signatureValidation reverts via:
- InvalidStaticSignatureExpired(opHash, timestamp)
- InvalidStaticSignatureWrongCaller(opHash, msg.sender, addr)
- InvalidSignatureWeight(threshold, weight)

These reverts are **not** caught in isValidSignature or ERC4337v07.validateUserOp:

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
``

Under ERC-1271, invalid signatures should return 0x00000000, **not revert**. Under ERC-4337, validateUserOp is expected to return a validationData error code (e.g., SIG_VALIDATION_FAILED) for invalid signatures, **not revert**. Because validateUserOp calls this.isValidSignature without try/catch, any revert bubbles up and reverts validateUserOp itself.

Impact:
- Any external caller can pass a malformed or expired static signature to isValidSignature, causing an unexpected revert instead of a 0 return value, breaking integrations that assume ERC-1271 semantics.
- For ERC-4337, a UserOperation with such a signature causes validateUserOp to revert instead of returning SIG_VALIDATION_FAILED. Bundlers/EntryPoint simulators expect a non-reverting validation; reverting can break simulation and allow griefing or DoS for the account, as a malicious signature can repeatedly cause validateUserOp to revert.

This matches the StandardViolation pattern: standard-required interfaces ERC-1271 and ERC-4337 are implemented in a way that can revert on normal invalid-signature conditions instead of returning failure codes.

## Impact
Standards-compliance and integration risk: multiple normal invalid-signature paths in BaseAuth.signatureValidation (e.g., static-sig flag with no stored entry or expired/wrong-caller, and low signature weight) revert instead of returning a failure code. For ERC-1271, callers expect 0x00000000 on failure; reverts can break composability for dApps/libraries that probe signature validity. For ERC-4337, validateUserOp currently bubbles these reverts, causing the user operation to be rejected during simulation/validation; bundlers will treat such ops as invalid, but this does not create a broader DoS or asset risk—only the provided operation fails. Severity is low but correctness and UX are impacted.

## Command to Run Test


## Proof of Concept
Observation 1 (ERC-1271): Any external caller can trigger a revert from isValidSignature by passing a signature whose first byte has bit7 set (0x80), even when no static signature was configured. This exercises the static-signature branch and reverts with InvalidStaticSignatureExpired since the stored timestamp defaults to 0.

Steps:
- Deploy Stage2Module with any entryPoint.
- Call isValidSignature(digest, hex"80").
- The call reverts with InvalidStaticSignatureExpired (timestamp=0) instead of returning 0x00000000.

Observation 2 (ERC-4337): A PackedUserOperation with signature starting with 0x80 triggers the same revert through validateUserOp. Since validateUserOp calls this.isValidSignature(...) without try/catch, the revert bubbles up and validateUserOp itself reverts instead of returning SIG_VALIDATION_FAILED.

Steps:
- Deploy Stage2Module with entryPoint E.
- As E, call validateUserOp with a PackedUserOperation whose signature is hex"80" (other fields can be zeroed). Provide any userOpHash.
- validateUserOp reverts (via isValidSignature) rather than returning a non-zero validationData code.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Stage2Module} from "src/Stage2Module.sol";
import {IAccount, PackedUserOperation} from "src/modules/interfaces/IAccount.sol";

contract SignatureRevertSpec is Test {
    Stage2Module wallet;
    address ep = address(0x1234);

    function setUp() public {
        wallet = new Stage2Module(ep);
    }

    function test_isValidSignature_reverts_on_static_flag() public {
        // Any digest; the specific value is irrelevant to trigger the static-sig path
        bytes32 digest = bytes32(uint256(0x42));
        // First byte 0x80 selects the static-signature path
        bytes memory sig = hex"80";

        vm.expectRevert();
        wallet.isValidSignature(digest, sig);
    }

    function test_validateUserOp_bubbles_revert() public {
        // Minimal PackedUserOperation; only signature matters for this test
        PackedUserOperation memory uo;
        uo.sender = address(wallet);
        uo.signature = hex"80"; // triggers static-sig branch -> revert inside isValidSignature

        vm.expectRevert();
        vm.prank(ep); // must be called by the configured entrypoint
        wallet.validateUserOp(uo, bytes32(uint256(1)), 0);
    }
}


## Suggested Mitigation
Make signature validation non-reverting for ordinary invalid cases and defensively swallow residual reverts at the call sites:

- In BaseAuth.signatureValidation, normalize expected invalid states to return (false, opHash) instead of reverting:
  - If static-sig path finds timestamp == 0 (no stored entry), expired timestamp, or wrong caller -> return (false, opHash).
  - If recovered weight < threshold -> return (false, opHash).
  Keep exceptional situations (e.g., malformed flags, snapshot misuse) as revert conditions in BaseSig.

- In ERC4337v07.validateUserOp, wrap the external call to this.isValidSignature in try/catch so any unexpected revert is converted to SIG_VALIDATION_FAILED:
  try this.isValidSignature(userOpHash, userOp.signature) returns (bytes4 magic) {
      if (magic != IERC1271_MAGIC_VALUE_HASH) return SIG_VALIDATION_FAILED;
  } catch { return SIG_VALIDATION_FAILED; }

- In BaseAuth.isValidSignature, similarly guard against internal errors by delegating to a small external view helper and catching reverts:
  - Add an external-view helper, e.g., function _trySignatureValidation(Payload.Decoded calldata p, bytes calldata s) external view returns (bool) { (bool ok,) = signatureValidation(p, s); return ok; }
  - Implement isValidSignature using try/catch on this._trySignatureValidation(...); on catch return 0x00000000.

These changes align behavior with ERC-1271 expectations, prevent revert bubbling in ERC-4337 flows, and preserve revert semantics only for exceptional/unexpected conditions.





 **Derived From** : StateGrowthOrStorageBloat

 ### Relevant Function/Location: Recovery.queuePayload

 ### Title
Recovery module unbounded queuedPayloadHashes array causes storage bloat

## [L-2]. Recovery.queuePayload appends to queuedPayloadHashes without any pruning, causing unbounded storage growth

### Finding Severity Justification: The issue correctly identifies an append-only array (queuedPayloadHashes) with no pruning, leading to unbounded storage growth. However, only authorized recovery signers can add entries, assets are not at risk, and no in-contract iteration over this array exists that could cause gas-based DoS. The impact is limited to on-chain state bloat and potential gas overhead for the caller, which aligns with QA/Low per C4 rubric.
## Derived From Pattern/Invariant
StateGrowthOrStorageBloat

 ### Relevant Function/Location: Recovery.queuePayload

 ### Title
Recovery module unbounded queuedPayloadHashes array causes storage bloat

## Exploit Type
Dos

## Location
Recovery.queuePayload

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The Recovery sapient signer tracks queued recovery payloads per (wallet, signer) in two structures:
- timestampForQueuedPayload[wallet][signer][payloadHash] => uint256
- queuedPayloadHashes[wallet][signer] => bytes32[]

queuePayload appends to the queuedPayloadHashes array and never removes entries:

```solidity
mapping(address => mapping(address => mapping(bytes32 => uint256))) public timestampForQueuedPayload;

mapping(address => mapping(address => bytes32[])) public queuedPayloadHashes;

function queuePayload(
  address _wallet,
  address _signer,
  Payload.Decoded calldata _payload,
  bytes calldata _signature
) external {
  if (!isValidSignature(_wallet, _signer, _payload, _signature)) {
    revert InvalidSignature(_wallet, _signer, _payload, _signature);
  }

  bytes32 payloadHash = Payload.hashFor(_payload, _wallet);
  if (timestampForQueuedPayload[_wallet][_signer][payloadHash] != 0) {
    revert AlreadyQueued(_wallet, _signer, payloadHash);
  }

  timestampForQueuedPayload[_wallet][_signer][payloadHash] = block.timestamp;
  queuedPayloadHashes[_wallet][_signer].push(payloadHash);

  emit NewQueuedPayload(_wallet, _signer, payloadHash, block.timestamp);
}
```

There is no function anywhere in Recovery that removes elements from queuedPayloadHashes or compacts the array. queuedPayloadHashes[_wallet][_signer] grows monotonically for every distinct payload queued by that signer for that wallet, regardless of whether it is later executed or becomes irrelevant.

Implications:
- Over time, an actively used recovery signer (or a griefing attacker who can validly sign many distinct payloads) can cause queuedPayloadHashes[_wallet][_signer].length to grow arbitrarily large.
- Each new payload increases the contract’s storage footprint and increases gas costs for any future operations that touch those storage slots (cold/warm access pattern, refunds reduced).
- If in the future any function iterates over queuedPayloadHashes (or off-chain tools rely on it), the unbounded length could cause gas-based DoS or make those interactions prohibitively expensive.

This matches the StateGrowthOrStorageBloat pattern: an append-only array with no pruning leads to unbounded state growth and potential long-term gas/availability issues.

## Impact
Unbounded on-chain state growth per (wallet, signer) due to an append-only queuedPayloadHashes array. Only a signer (or ERC1271 signer contract) that can produce valid signatures for the wallet can enqueue, so this is not an unprivileged attack. The current Recovery module does not iterate this array on-chain, so there is no immediate gas-based DoS in existing logic; the impact is persistent storage bloat and higher cumulative costs for writes that create new slots. If future features (or other contracts) iterate this array, calls could become prohibitively expensive. Off-chain tooling that enumerates the array will also face scalability issues.

## Command to Run Test


## Proof of Concept
PoC steps (ECDSA path):
- Pick a private key pk and derive signer = vm.addr(pk).
- Construct distinct Payload.Decoded values (e.g., Payload.fromMessage(abi.encode(i)) for i in 1..N) so each produces a different recoveryPayloadHash.
- For each payload, compute rPayloadHash = Recovery.recoveryPayloadHash(wallet, payload) and produce an ERC-2098 compact signature over rPayloadHash using pk.
- Call queuePayload(wallet, signer, payload, sig). Each call succeeds and pushes the payload hash into queuedPayloadHashes[wallet][signer].
- After N iterations, totalQueuedPayloads(wallet, signer) == N, demonstrating unbounded growth with no pruning.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Recovery} from "src/extensions/recovery/Recovery.sol";
import {Payload} from "src/modules/Payload.sol";

contract RecoveryGrowthTest is Test {
    Recovery rec;
    address wallet = address(0xBEEF);

    function setUp() public {
        rec = new Recovery();
    }

    function _signCompact(bytes32 digest, uint256 pk) internal view returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        uint256 yParity = (v == 28) ? 1 : 0;
        bytes32 yParityAndS = bytes32((uint256(s) & ((1 << 255) - 1)) | (yParity << 255));
        return abi.encodePacked(r, yParityAndS);
    }

    function test_unbounded_growth_of_queuedPayloadHashes() public {
        // EOA signer with known private key
        uint256 pk = 0xA11CE;
        address signer = vm.addr(pk);

        uint256 N = 50; // can be arbitrarily large
        for (uint256 i = 1; i <= N; i++) {
            Payload.Decoded memory p = Payload.fromMessage(abi.encode(i));
            bytes32 rPayloadHash = rec.recoveryPayloadHash(wallet, p);
            bytes memory sig = _signCompact(rPayloadHash, pk);
            rec.queuePayload(wallet, signer, p, sig);
        }

        assertEq(rec.totalQueuedPayloads(wallet, signer), N, "queuedPayloadHashes grows without pruning");
    }
}


## Suggested Mitigation
Best: remove queuedPayloadHashes entirely and rely on NewQueuedPayload events plus the timestampForQueuedPayload mapping for verification. This eliminates on-chain enumeration and prevents storage bloat while preserving required functionality.

If enumeration is required on-chain, introduce bounds and pruning:
- Cap per-(wallet,signer) entries (e.g., MAX_QUEUED_PER_SIGNER) and revert when reached.
- Add explicit pruning/sweeping functions callable by the wallet or signer, e.g., delete specific payloadHash entries (swap-and-pop) or prune entries older than a time window.

Note: recoverSapientSignatureCompact is view and cannot mutate state, so automatic pruning upon consumption cannot be performed there; pruning must be an explicit, state-changing function or the array should be removed.





 **Derived From** : Checkpointer callback can grief all signatures using that configuration

## [L-3]. Malicious or broken checkpointer can permanently DoS all operations for affected wallet configurations

### Finding Severity Justification: Impact is availability-only (DoS of signature validation) and hinges on a trusted, user-chosen external dependency (the checkpointer). No funds or authorization can be stolen; operations are blocked if the configured checkpointer reverts. Under Code4rena rules, this is a centralization/trust assumption and thus QA/Low.
## Derived From Pattern/Invariant
Checkpointer callback can grief all signatures using that configuration

## Exploit Type
Dos

## Location
BaseSig.recover

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The signature recovery pipeline calls into an external `ICheckpointer` contract during every non‑static signature validation when the top‑level signature flag has the checkpointer bit set and no explicit `_checkpointer` was passed.

Relevant code in `BaseSig.recover`:

```solidity
function recover(
  Payload.Decoded memory _payload,
  bytes calldata _signature,
  bool _ignoreCheckpointer,
  address _checkpointer
) internal view returns (...) {
  // First byte is the signature flag
  (uint256 signatureFlag, uint256 rindex) = _signature.readFirstUint8();

  Snapshot memory snapshot;

  // Recover the imageHash checkpointer if any
  // but checkpointer passed as argument takes precedence
  if (signatureFlag & 0x40 == 0x40 && _checkpointer == address(0)) {
    (_checkpointer, rindex) = _signature.readAddress(rindex);

    if (!_ignoreCheckpointer) {
      uint256 checkpointerDataSize;
      (checkpointerDataSize, rindex) = _signature.readUint24(rindex);

      bytes memory checkpointerData = _signature[rindex:rindex + checkpointerDataSize];

      snapshot = ICheckpointer(_checkpointer).snapshotFor(address(this), checkpointerData);

      rindex += checkpointerDataSize;
    }
  }
  ...
}
```

This call is made:
- For any signature whose first byte sets bit 6 (`0x40`) and where `_checkpointer` argument is zero (the normal path from `BaseAuth.signatureValidation`).
- Before any threshold / signer‑tree validation, and without a `try/catch`.
- With all remaining gas forwarded and no bounds on return‑data size.

`BaseAuth.signatureValidation` always calls `BaseSig.recover` with `_ignoreCheckpointer=false` and `_checkpointer=address(0)` in the dynamic‑signature path:

```solidity
(threshold, weight, imageHash,, opHash) = BaseSig.recover(_payload, _signature, false, address(0));
```

As a result, if the configured checkpointer contract:
- Reverts in `snapshotFor()` (e.g. due to a bug, bad upgrade, or deliberate attack), or
- Consumes excessive gas or attacks via return‑bombing,

then *every* signature that references it will cause `BaseSig.recover` to revert, which bubbles up through `BaseAuth.signatureValidation` into all higher‑level entrypoints (`Calls.execute`, `ERC4337v07.validateUserOp` via ERC‑1271, etc.). The wallet becomes unable to validate any non‑static signature under that configuration, effectively bricking all operations.

Because the checkpointer address is encoded into the configuration Merkle tree and thus into `imageHash`, all wallets using that configuration on a chain share the same single point of failure. A misbehaving or compromised checkpointer can therefore DoS all affected wallets without any action from external attackers beyond supplying a signature that sets bit 6 in the flag (which is required for normal use of a checkpointer anyway).

This matches the **GriefableCallbacks / Dos** pattern: a mandatory external callback in the core auth path can revert or consume gas to permanently prevent use of a configuration.

## Impact
If the configured checkpointer contract becomes faulty or malicious, all dynamic signatures that rely on it will revert during validation, preventing affected wallets from executing any operations (including configuration upgrades) under that configuration. This can brick wallets or force users to migrate to new wallets, with loss of availability and potentially requiring asset migrations.

## Command to Run Test


## Proof of Concept
1. Assume a wallet configuration includes a checkpointer leaf whose address is `MaliciousCheckpointer`.
2. A user signs a normal (non‑static) Sequence payload producing a signature whose first byte has bit 6 set (`0x40`), encoding the checkpointer address and some arbitrary proof data.
3. `Calls.execute` is invoked with this payload and signature. In the dynamic signature path, `BaseAuth.signatureValidation` calls `BaseSig.recover(decoded, signature, false, address(0))`.
4. `BaseSig.recover` sees `signatureFlag & 0x40 != 0` and `_checkpointer == address(0)`, so it reads the checkpointer address from the signature and then calls `ICheckpointer(_checkpointer).snapshotFor(address(this), proof)`.
5. The `MaliciousCheckpointer.snapshotFor` implementation simply reverts (or runs heavy computation until gas is exhausted).
6. This revert bubbles up, causing `signatureValidation` to revert and in turn reverting `execute` / `validateUserOp` for all such signatures.
7. Because the checkpointer is part of the configuration, *every* signature using that configuration and bit 6 set will hit the same revert. The owner cannot execute any further calls that require dynamic signatures under that config, including any upgrade logic that would remove or change the checkpointer.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {BaseAuth} from "src/modules/auth/BaseAuth.sol";
import {Payload} from "src/modules/Payload.sol";
import {ICheckpointer, Snapshot} from "src/modules/interfaces/ICheckpointer.sol";

contract MaliciousCheckpointer is ICheckpointer {
  function snapshotFor(address, bytes calldata) external view override returns (Snapshot memory) {
    revert("snap-revert");
  }
}

contract AuthHarness is BaseAuth {
  function _isValidImage(bytes32) internal view override returns (bool) {
    return true;
  }
  function _updateImageHash(bytes32) internal override {}
  function validate(Payload.Decoded calldata payload, bytes calldata sig) external view {
    signatureValidation(payload, sig);
  }
}

contract CheckpointerRevertTest is Test {
  AuthHarness auth;
  MaliciousCheckpointer cp;

  function setUp() public {
    auth = new AuthHarness();
    cp = new MaliciousCheckpointer();
  }

  function test_CheckpointerRevert_GriefsValidation() public {
    // Minimal payload; revert happens before hashing/signature tree processing
    Payload.Decoded memory payload;
    payload.kind = Payload.KIND_MESSAGE;
    payload.message = bytes("test");

    // signatureFlag = 0x40 (checkpointer enabled), checkpoint size=0, threshold size=1 byte
    // Encoded as: [flag][checkpointer address][uint24 dataSize=0][threshold=1]
    bytes memory sig = abi.encodePacked(
      bytes1(0x40),                 // bit6 set => checkpointer present
      address(cp),                  // checkpointer address
      bytes3(0),                    // checkpointer data size = 0
      bytes1(uint8(0x01))           // threshold = 1 (checkpoint size=0 => no bytes)
      // no branch items; unreachable due to revert prior to parsing
    );

    // Expect the revert reason from MaliciousCheckpointer
    vm.expectRevert("snap-revert");
    auth.validate(payload, sig);
  }
}


## Suggested Mitigation
Do not silently ignore checkpointer failures, as that weakens the core invariant that snapshots must be respected when a checkpointer is configured. Instead:
- Preserve security invariants while improving operability:
  1) Wrap the snapshotFor call in try/catch but rethrow a dedicated error (e.g., CheckpointerCallFailed(address cp, bytes reason)) so failures are diagnosable without masking them.
  2) Provide an explicit, owner-controlled escape hatch that does not rely on the checkpointer path:
     - Pre-provision a static-signature authorization (bit7 path) for a narrowly scoped self-call that updates the imageHash to a configuration with checkpointer = address(0). Static signatures are evaluated before any checkpointer call, so this path remains usable even if the checkpointer bricks validation.
     - Alternatively, add a dedicated onlySelf function (e.g., disableCheckpointer()) callable via a pre-authorized hardcoded subdigest leaf, to flip the checkpointer leaf to zero in storage. Ensure this function is only invocable through a pre-committed static/hardcoded authorization to avoid bypassing normal signature validation.
- Operational hardening:
  3) Enforce a maximum checkpointerDataSize and cap gas forwarded to the checkpointer via inline assembly staticcall to reduce gas grief.
  4) Document that the checkpointer must implement a non-reverting, explicit disable mode (return Snapshot with imageHash=0) and thorough upgrade/testing procedures.
This approach avoids availability DoS without compromising the safety guarantees the checkpointer is intended to enforce.


## [L-4]. Untrusted ICheckpointer.snapshotFor callbacks can permanently grief signature validation for affected configs

### Finding Severity Justification: The described DoS is real: if a configuration uses a checkpointer whose snapshotFor reverts, signature validation reverts and the wallet becomes unusable under that imageHash. However, the checkpointer address is an explicit, privileged configuration element selected by the wallet owners. This is an operational/governance risk stemming from trusting a bad third-party module rather than a flaw enabling external attackers to brick wallets. Impact can be high for affected users, but exploitation requires owners to opt into a faulty/malicious checkpointer, so per Code4rena rubric this fits QA/Low.
## Derived From Pattern/Invariant
Checkpointer callback can grief all signatures using that configuration

## Exploit Type
Dos

## Location
BaseSig.recover

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The core signature recovery library `BaseSig.recover` supports an optional checkpointer module that is invoked based on the top-level `signatureFlag` bit 6:

```solidity
// BaseSig.recover
if (signatureFlag & 0x40 == 0x40 && _checkpointer == address(0)) {
  (_checkpointer, rindex) = _signature.readAddress(rindex);

  if (!_ignoreCheckpointer) {
    uint256 checkpointerDataSize;
    (checkpointerDataSize, rindex) = _signature.readUint24(rindex);

    bytes memory checkpointerData = _signature[rindex:rindex + checkpointerDataSize];

    snapshot = ICheckpointer(_checkpointer).snapshotFor(address(this), checkpointerData);

    rindex += checkpointerDataSize;
  }
}
```

Key points:
- If the checkpointer bit is set and `_checkpointer` is zero on entry, the function reads a dynamic address from the signature and assigns it to `_checkpointer`.
- Unless `_ignoreCheckpointer` is true (which is only used internally when processing chained segments), it then reads a length-prefixed `checkpointerData` slice and calls `ICheckpointer(_checkpointer).snapshotFor(address(this), checkpointerData)`.
- This is an **unbounded external view call** to an arbitrary address controlled by the wallet configuration. There is no `try/catch`, no gas stipend control (other than the ambient gas), and no limit on return data.

If the configured checkpointer is buggy or malicious, it can:
- Always revert in `snapshotFor`, causing any signature that references it to revert during validation.
- Consume excessive gas or return huge data, leading to griefing via gas or return-data expansion.

Because the checkpointer address is part of the imageHash configuration, any configuration that uses a misbehaving checkpointer becomes **unusable**: all signatures that include the checkpointer flag will revert during validation before reaching the signer tree logic. There is no in-protocol way for a wallet already stuck on such a configuration to bypass the call: you need a new configuration that removes or replaces the checkpointer, but updating configuration also requires valid signatures under the current one, which are blocked by the malfunctioning callback.


## Impact
A wallet configuration that enables a misbehaving checkpointer (either buggy or malicious) can be permanently DoS’d: every signature that uses that configuration will revert during BaseSig.recover when calling ICheckpointer.snapshotFor. This blocks execute, ERC-1271 isValidSignature, and ERC-4337 validateUserOp, effectively freezing funds under that imageHash. The risk is primarily operational/governance because owners must opt into the faulty checkpointer, but impact to affected users can be total loss of control on the affected chain until an out-of-band recovery path is available.

## Command to Run Test


## Proof of Concept
How to deterministically trigger the DoS via a malicious checkpointer:

1) Attacker model: Owners or integrator configure the wallet to use a checkpointer address C (part of the imageHash). If C later reverts in snapshotFor, any signature using that config DoS’s validation.
2) Deploy a checkpointer contract C where snapshotFor always reverts.
3) Build a signature whose first byte sets bit6 (0x40), encoding the checkpointer address C and a 0-length checkpointerData. Minimal encoding that already causes the call:
   - signature[0] = 0x40
   - signature[1..20] = bytes20(C)
   - signature[21..23] = bytes3(0x000000)  // 3-byte size = 0
   No further bytes are required to trigger the external call.
4) Call wallet.isValidSignature(hash, signature). BaseSig.recover reads the 0x40 flag, loads C, sees !_ignoreCheckpointer, reads size=0, and immediately calls ICheckpointer(C).snapshotFor(address(this), ""). C reverts, bubbling up and reverting validation.
5) The same revert occurs for Calls.execute (during signatureValidation) and ERC-4337 validateUserOp (which internally uses isValidSignature). The wallet is unable to validate any signature for that configuration, preventing configuration updates that would remove/replace the checkpointer.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ICheckpointer, Snapshot} from "src/modules/interfaces/ICheckpointer.sol";
import {Stage2Module} from "src/Stage2Module.sol";

contract MaliciousCheckpointer is ICheckpointer {
    function snapshotFor(address, bytes calldata) external pure override returns (Snapshot memory) {
        revert("grief");
    }
}

contract CheckpointerGriefTest is Test {
    Stage2Module wallet;
    MaliciousCheckpointer cp;

    function setUp() public {
        wallet = new Stage2Module(address(0));
        cp = new MaliciousCheckpointer();
    }

    function test_CheckpointerRevert_DOSesSignatureValidation() public {
        // signature layout (minimal to trigger snapshotFor):
        // [0]: 0x40 (bit6 set => checkpointer present)
        // [1..20]: checkpointer address
        // [21..23]: uint24 checkpointerDataSize = 0
        bytes memory sig = abi.encodePacked(bytes1(0x40), bytes20(address(cp)), bytes3(0));

        bytes32 fakeHash = keccak256("op");
        vm.expectRevert("grief");
        wallet.isValidSignature(fakeHash, sig);
    }
}


## Suggested Mitigation
Do not ignore a reverting checkpointer; that would silently bypass the snapshot enforcement and weaken security. Instead:

- Contract-level hardening:
  - Wrap the snapshotFor call in try/catch and on catch revert with a dedicated error (e.g., CheckpointerCallFailed(address)) so failures are explicit and diagnosable, but do not proceed as if disabled.
  - Optionally bound gas via a low-level call with a reasonable stipend to mitigate gas griefing without altering semantics.

- Safe escape hatch (liveness without weakening normal semantics):
  - Add a narrowly-scoped recovery path that allows replacing/removing the checkpointer only via a chained signature that performs a CONFIG_UPDATE and strictly increases the checkpoint. This path should be the only case where a failed snapshot call is allowed to continue. Enforce that the final imageHash’s checkpointer leaf is either address(0) or a whitelisted/audited address. Any non-config-update payload must still revert on snapshot failure.

- Operational controls:
  - Maintain a whitelist/registry of audited checkpointers and restrict configurations to use only approved addresses.
  - Strongly specify in ICheckpointer that snapshotFor must not revert on bad proofs; it should return Snapshot({imageHash: 0, checkpoint: 0}) for invalid data. Enforce this in audits for third-party checkpointers.
  - Encourage wallets enabling a checkpointer to pre-provision an emergency config-update mechanism (e.g., a pre-set static signature that authorizes only updateImageHash) so owners can replace a failing checkpointer without relying on it during emergencies.



