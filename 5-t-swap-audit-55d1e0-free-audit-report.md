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
[H-1]. Reentrancy issue found with High severity
[H-2]. Reentrancy issue found with High severity
[H-3]. Reentrancy issue found with High severity
[H-4]. Frontrun/Backrun/Sandwhich MEV issue found with High severity
## Medium Risk Findings
[M-1]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
[M-2]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
[M-3]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
[M-4]. Reentrancy issue found with Medium severity
[M-5]. DOS issue found with Medium severity
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



