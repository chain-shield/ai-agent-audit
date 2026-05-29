# Accepted H/M Findings: Size

# [H-01] When sellCreditMarket() is called to sell credit for a specific cash amount, the protocol might receive a lower swapping fee than expected

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

sellCreditMarket() is called to sell credit for a specific cash amount, the protocol might receive a lower swapping fee than expected Submitted by 0xpiken, also found by 3n0ch ( 1, 2 ), 0xAlix2, Honour, gd, asui, DanielArmstrong, carlos__alegre, zarkk01, Shield ( 1, 2 ), mt030d, Brenzee, 0xStalin, dhank, KupiaSec, Kalogerone, 0xJoyBoy03, bin2chen, pkqs90, ether_sky, trachev, samuraii77, m4k2, and LinKenji

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/AccountingLibrary.sol#L249
- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/AccountingLibrary.sol#L256

## Impact

The protocol might receive a lower fee than expected when sellCreditMarket() is called to sell credit for a specific cash amount.

## Recommended Mitigation Steps

Correct the swap fee calculation:

function getCreditAmountIn( State storage state, uint256 cashAmountOut, uint256 maxCashAmountOut, uint256 maxCredit, uint256 ratePerTenor, uint256 tenor ) internal view returns (uint256 creditAmountIn, uint256 fees) { uint256 swapFeePercent = getSwapFeePercent(state, tenor); uint256 maxCashAmountOutFragmentation = 0; if (maxCashAmountOut >= state.feeConfig.fragmentationFee) { maxCashAmountOutFragmentation = maxCashAmountOut - state.feeConfig.fragmentationFee; } // slither-disable-next-line incorrect-equality if (cashAmountOut == maxCashAmountOut) { // no credit fractionalization creditAmountIn = maxCredit; - fees = Math.mulDivUp(cashAmountOut, swapFeePercent, PERCENT); + fees = Math.mulDivUp(cashAmountOut, swapFeePercent, PERCENT - swapFeePercent);

} else if (cashAmountOut < maxCashAmountOutFragmentation) { // credit fractionalization creditAmountIn = Math.mulDivUp( cashAmountOut + state.feeConfig.fragmentationFee, PERCENT + ratePerTenor, PERCENT - swapFeePercent ); - fees = Math.mulDivUp(cashAmountOut, swapFeePercent, PERCENT) + state.feeConfig.fragmentationFee; + fees = Math.mulDivUp(cashAmountOut + state.feeConfig.fragmentationFee, swapFeePercent, PERCENT - swapFeePercent) + state.feeConfig.fragmentationFee; } else { // for maxCashAmountOutFragmentation < amountOut < maxCashAmountOut we are in an inconsistent situation // where charging the swap fee would require to sell a credit that exceeds the max possible credit revert Errors.NOT_ENOUGH_CASH(maxCashAmountOutFragmentation, cashAmountOut);

}

## Assessed type

Math aviggiano (Size) confirmed and commented via duplicate Issue #213:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/137.

hansfriese (judge) increased severity to High MotokoKusanagi-aka-Major (Size) commented:

This issue is an incorrect implementation of the swap fees formula described in the technical documentation, resulting in less fee collection in the credit swapping process.

In our opinion, this should not be categorized as a “High” issue since it only negatively affects the protocol owners, because in the current version, there is no mechanism for fees redistribution, so no user category (lenders / credit buyers, borrowers / credit sellers, and liquidators) is negatively affected. In fact, the user who is subject to fee payment would have benefited from this issue since he would have ended up paying lower fees than expected.

# [H-02] Risk of overpayment due to race condition between repay and liquidateWithReplacement transactions

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

repay and liquidateWithReplacement transactions Submitted by elhaj, also found by KupiaSec, mt030d, and VAD37

- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/Size.sol#L198-L201
- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/Size.sol#L229-L244

## Impact

Likelihood: Medium - Users and liquidation bots are likely to call their respective functions around the same time.

## Impact: High - Users end up repaying more than double their future value.

## Recommended Mitigation Steps

To prevent unintended debt repayment due to borrower changes, users should specify both the borrower address and the debt position ID when repaying. This ensures users have control over who they are repaying for, avoiding scenarios where they might repay another borrower’s debt due to a recent liquidateWithReplacement transaction.

Make this changes in repay lib:

struct RepayParams { uint256 debtPositionId; + address borrower; } function validateRepay(State storage state, RepayParams calldata params) external view { // validate debtPositionId if (state.getLoanStatus(params.debtPositionId) == LoanStatus.REPAID) { revert Errors.LOAN_ALREADY_REPAID(params.debtPositionId); } + if (state.getDebtPosition(params.debtPositionId).borrower != params.borrower) revert("invalid borrower"); // validate msg.sender // N/A }

## Assessed type

Timing aviggiano (Size) acknowledged and commented:

Feedback from the team:

This looks valid since it does not look like a user mistake, it is a kind of MEV. In practice, I think it can happen rarely since for this to happen we need this to be true:

When a user notices that their position is liquidatable (probably through a UI notification), there is a high chance they will repay their debt to avoid liquidation, In my honest opinion, there is a high chance they deposit more collateral or compensate to avoid liquidation, since repaying early would mean losing all the interest; so not a great way to do that (even though in some cases it could be the only chance). Technically valid, but practically low impact.

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/141.

hansfriese (judge) commented:

I will keep as High because the impact is critical and the likelihood is not rare.

0xStalin (warden) commented:

This is an interesting case, but, a high severity seems to be overgenerous because of the pre-conditions that needs to exist for this to become a problem.

There needs to be a borrower for the APR and Tenor of the liquidated Position.

The borrower being liquidated needs to attempt to repay the same position being liquidated (Borrower can have multiple debt positions and can attempt to repay any of the others, not necessarily the exact position being liquidated and replaced).

The liquidate and replacement transaction must be executed BEFORE than the repay tx.

@hansfriese - because of the multiple on-chain conditions, this seems to fit more of a medium severity.

hansfriese (judge) commented:

I agree that it requires some prerequisites. However, with a liquidatable position, it is not that strong assumption that those 2 functions could be called simultaneously. According to the C4 severity categorization, High is more appropriate due to the direct fund loss and the likelihood of this scenario occurring.

MotokoKusanagi-aka-Major (Size) commented:

This issue arises from a potential race condition involving two function calls regarding a debt position eligible for liquidation:

A borrower calling repay() to avoid the liquidation and The protocol backend calling liquidateWithReplacement() to liquidate the position with replacement (this is a permissioned method).

Since there was no check regarding the ownership of that debt, any address was allowed to repay the debt of any other address so if the liquidateWithReplacement() was executed first, the borrower would have ended up being liquidated and repaying someone else debt, because of the replacement.

The impact is certainly high but the likelihood of this issue happening in our opinion has been largely overestimated, and is for all practical purposes, non-existent.

The reason is that a borrower eligible for liquidation has no incentive to repay loans to avoid liquidation since he would forfeit the interest for the remaining lifetime of the loan. In that case, what makes more sense is for them to increase their collateral ratio by Depositing more collateral and Compensating some of their debt with eligible credit The race between 1 and 2 with liquidateWithReplacement() is not associated with any issue.

The only case where the borrower can make sense to repay his loan early is when the forfeited interests would be negligible, meaning at the end of the loan lifecycle. However, in that case, the replacement is extremely unlikely to happen since it would require a borrower willing to borrow passively for a very short tenor. This is very unlikely to happen as a borrower would need some time to react to his limit borrow order being filled and quickly decide what to do with the borrowed money, before this new loan goes overdue and he becomes eligible for liquidation.

On top of this, the liquidateWithReplacement() method is permissioned and we have a negligible economic incentive in running it vs a standard liquidation. This is because the latter is unaffected by the issue mentioned above since the value the protocol captures from running a liquidation with replacement is proportional to the remaining loan lifetime. It would therefore be a net loss to use liquidateWithReplacement() instead of liquidate() for a loan very close to maturity as we would gain almost nothing while potentially significantly affecting our users.

# [H-03] The collateral remainder cap is incorrectly calculated during liquidation

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by ether_sky, also found by hyh, samuraii77, m4k2, KupiaSec, stakog, AMOW, jsmi, and Bob When a position is overdue, it can be liquidated even if the owner has a sufficient collateral ratio. Some rewards are sent to the liquidator as an incentive and some fees are assigned to the protocol. However, the calculation of the protocol fee is flawed due to an incorrect collateral remainder cap calculation.

Imagine the collateral ratio for liquidation is set at 130%. The collateral equivalent to the debt should be paid to the protocol, and the excess collateral (30%) can be the remainder cap. Currently, the entire 130% is treated as the remainder cap, leading to an unfair situation where a user with a 150% collateral ratio experiences more loss than a user with a 140% CR.

This discourages users with higher CRs, who generally contribute to the protocol’s health. If higher CRs result in greater losses, it disincentivizes users from providing sufficient collateral to the protocol.

## Recommended Mitigation Steps

function executeLiquidate(State storage state, LiquidateParams calldata params) external returns (uint256 liquidatorProfitCollateralToken) { if (assignedCollateral > debtInCollateralToken) { uint256 liquidatorReward = Math.min( assignedCollateral - debtInCollateralToken, Math.mulDivUp(debtPosition.futureValue, state.feeConfig.liquidationRewardPercent, PERCENT) ); liquidatorProfitCollateralToken = debtInCollateralToken + liquidatorReward; uint256 collateralRemainder = assignedCollateral - liquidatorProfitCollateralToken; - uint256 cllateralRemainderCap = Math.mulDivDown(debtInCollateralToken, state.riskConfig.crLiquidation, PERCENT); + uint256 cllateralRemainderCap = Math.mulDivDown(debtInCollateralToken, state.riskConfig.crLiquidation - PERCENT, PERCENT);

}

## Assessed type

Math aviggiano (Size) confirmed via duplicate Issue #140 hansfriese (judge) increased severity to High and commented:

Marking this report as primary with the detailed POC.

MotokoKusanagi-aka-Major (Size) commented:

We acknowledge the severity of these issues being high since they impact the users negatively and are not unlikely to happen, but we want to share a bit more context of how these bugs have been introduced into the code H-03 and H-04 are two incorrect implementations of formulas correctly specified in the tech documentation, much like H-01. Note that this means that three of the four high-severity issues were simply incorrect implementations of formulas.

Incorrect implementations of this sort are usually easily caught by simple unit tests relying on pre-computed expected values so it is uncommon to find them in audit reports–and indeed numerous wardens also caught these bugs during the C4 competition–so we want to explain why this happened to avoid incorrect conclusions about our development process.

One finding of the Spearbit Audit (the one before the C4) required us to redesign and reimplement the fee mechanism entirely.

The changes required were quite significant and we had a tight deadline since we had already defined C4 start date and the launch date. This required us to be very judicious with our time and focus on the actual implementation and be light on tests to avoid having to reschedule the C4 competition.

However, since what ultimately matters the most for us is the security of our code, we worked on the tests in parallel with the competition and we were able to identify these bugs even before they were also discovered by the C4 competitors.

In conclusion, we were aware the strategy we had chosen would have increased the risk of having more bugs found during the competition–we did not optimize for receiving “high grades” from the competition (even though overall the “grades” we received seem pretty much aligned with the ones of other protocols). We chose the strategy to be able to meet the deadlines for the competition, and ultimately it did not come at the cost of less code security.

See also Issue #21.

# [H-04] Users won’t liquidate positions because the logic used to calculate the liquidator’s profit is incorrect

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by ether_sky, also found by serial-coder, samuraii77, muellerberndt, 0xAlix2, 3n0ch, nfmelendez, BenRai, Zkillua, supersizer0x, ayden, gd, 0xBugSlayer, radin100, DanielArmstrong, max10afternoon, Bigsam, iamandreiski, asui, 0xanmol, Honour, 0xStalin, Jorgect, Nave765, zanderbyte, carlos__alegre, Nyx, Decipher100Eyes, inzinko ( 1, 2 ), dhank, Infect3d, adeolu, ParthMandale, 0xarno, 0xpiken, hezze, elhaj, trachev, BajagaSec, silver_eth, Nihavent, MidgarAudits, zarkk01, hyh, Tychai0s, alix40, 0xRstStn, Brenzee, KupiaSec, Kalogerone, 0xJoyBoy03, ilchovski, 0xRobocop, stakog, bin2chen, AlexCzm, AMOW, pkqs90, VAD37, Sentryx, and said

Liquidation is crucial, and all liquidatable positions must be liquidated to maintain the protocol’s solvency. However, because the liquidator’s profit is calculated incorrectly, users are not incentivized to liquidate unhealthy positions. This issue has a significant impact on the protocol.

## Recommended Mitigation Steps

Use the converted value to ETH:

function executeLiquidate(State storage state, LiquidateParams calldata params) external returns (uint256 liquidatorProfitCollateralToken) { DebtPosition storage debtPosition = state.getDebtPosition(params.debtPositionId); LoanStatus loanStatus = state.getLoanStatus(params.debtPositionId); uint256 collateralRatio = state.collateralRatio(debtPosition.borrower); uint256 collateralProtocolPercent = state.isUserUnderwater(debtPosition.borrower) ? state.feeConfig.collateralProtocolPercent: state.feeConfig.overdueCollateralProtocolPercent; uint256 assignedCollateral = state.getDebtPositionAssignedCollateral(debtPosition); uint256 debtInCollateralToken = state.debtTokenAmountToCollateralTokenAmount(debtPosition.futureValue);

uint256 protocolProfitCollateralToken = 0; if (assignedCollateral > debtInCollateralToken) { uint256 liquidatorReward = Math.min( assignedCollateral - debtInCollateralToken, - Math.mulDivUp(debtPosition.futureValue, state.feeConfig.liquidationRewardPercent, PERCENT) + Math.mulDivUp(debtInCollateralToken, state.feeConfig.liquidationRewardPercent, PERCENT) ); liquidatorProfitCollateralToken = debtInCollateralToken + liquidatorReward;...

} else { liquidatorProfitCollateralToken = assignedCollateral; }

## Assessed type

Math aviggiano (Size) confirmed and commented:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/118
MotokoKusanagi-aka-Major (Size) commented:

We acknowledge the severity of these issues being high since they impact the users negatively and are not unlikely to happen, but we want to share a bit more context of how these bugs have been introduced into the code H-03 and H-04 are two incorrect implementations of formulas correctly specified in the tech documentation, much like H-01. Note that this means that three of the four high-severity issues were simply incorrect implementations of formulas.

Incorrect implementations of this sort are usually easily caught by simple unit tests relying on pre-computed expected values so it is uncommon to find them in audit reports–and indeed numerous wardens also caught these bugs during the C4 competition–so we want to explain why this happened to avoid incorrect conclusions about our development process.

One finding of the Spearbit Audit (the one before the C4) required us to redesign and reimplement the fee mechanism entirely.

The changes required were quite significant and we had a tight deadline since we had already defined C4 start date and the launch date. This required us to be very judicious with our time and focus on the actual implementation and be light on tests to avoid having to reschedule the C4 competition.

However, since what ultimately matters the most for us is the security of our code, we worked on the tests in parallel with the competition and we were able to identify these bugs even before they were also discovered by the C4 competitors.

In conclusion, we were aware the strategy we had chosen would have increased the risk of having more bugs found during the competition–we did not optimize for receiving “high grades” from the competition (even though overall the “grades” we received seem pretty much aligned with the ones of other protocols). We chose the strategy to be able to meet the deadlines for the competition, and ultimately it did not come at the cost of less code security.

See also Issue #70.

Medium Risk Findings (13)

# [M-01] Multicall does not work as intended

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by prapandey031, also found by Inspex, samuraii77, muellerberndt, 3n0ch, Jorgect, Honour, nnez, MidgarAudits, elhaj, alix40, 0xRobocop, 0xrafaelnicolau, mt030d, inzinko, rscodes, 0xpiken, hezze, evmboi32, zarkk01, trachev, KupiaSec, stakog, BoltzmannBrain, bin2chen, pkqs90, shaflow2, VAD37, Sentryx, ether_sky, said, and SpicyMeatball

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/Multicall.sol#L29
- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/Multicall.sol#L37

## Impact

The multicall(bytes[] calldata _data) function in the Size.sol contract does not work as intended. The intention of the multicall(bytes[] calldata _data) function is to allow users to access multiple functionalities of the Size protocol, such as a (deposit and repay) pair, by a single transaction to Size.sol:

- https://github.com/code-423n4/2024-06-size/blob/main/src/Size.sol#L142
function multicall ( bytes [] calldata _data ) public payable override ( IMulticall ) whenNotPaused returns ( bytes [] memory results ) { results = state.

multicall ( _data ); } The multicall function allows batch processing of multiple interactions with the protocol in a single transaction. This also allows users to take actions that would otherwise be denied due to deposit limits. One of these actions is a (deposit and repay) pair.

Let’s say a credit-debt pair exists. Assume that the tenor of the debt is 1 year and the future value is 100ke6 USDC. Let’s say the borrower decides to repay the loan just 1 day before the maturity ends. During this 1 year, the total supply of borrowAToken had increased so much that the total supply of borrowAToken was just 10e6 USDC worth below the cap (that is, just below the cap) at the time when the borrower decided to repay the loan.

To repay a loan, Size requires the user to have sufficient borrowAToken:

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/actions/Repay.sol#L49
function executeRepay ( State storage state, RepayParams calldata params ) external { DebtPosition storage debtPosition = state.

getDebtPosition ( params.

debtPositionId ); state.

data.

borrowAToken.

transferFrom ( msg.

sender, address ( this ), debtPosition.

futureValue ); debtPosition.

liquidityIndexAtRepayment = state.

data.

borrowAToken.

liquidityIndex (); state.

repayDebt ( params.

debtPositionId, debtPosition.

futureValue ); emit Events.

Repay ( params.

debtPositionId ); } The user achieves this by first depositing the required amount of underlying borrow tokens (here, USDC) and then calling the repay(RepayParams calldata params) function:

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/DepositTokenLibrary.sol#L49
function depositUnderlyingBorrowTokenToVariablePool ( State storage state, address from, address to, uint256 amount ) external { state.

data.

underlyingBorrowToken.

safeTransferFrom ( from, address ( this ), amount ); IAToken aToken = IAToken ( state.

data.

variablePool.

getReserveData ( address ( state.

data.

underlyingBorrowToken )).

aTokenAddress ); uint256 scaledBalanceBefore = aToken.

scaledBalanceOf ( address ( this )); state.

data.

underlyingBorrowToken.

forceApprove ( address ( state.

data.

variablePool ), amount ); state.

data.

variablePool.

supply ( address ( state.

data.

underlyingBorrowToken ), amount, address ( this ), 0 ); uint256 scaledAmount = aToken.

scaledBalanceOf ( address ( this )) - scaledBalanceBefore; state.

data.

borrowAToken.

mintScaled ( to, scaledAmount ); } Now, in our case, when the borrower decides to deposit 100ke6 USDC (at max if it would have some existing borrowAToken ), he would not be able to do so (as the cap would be hit by depositing just 10e6 USDC). The situation is that the tenor is about to end (1 day left) and the borrower is not able to repay, not because he does not have money, but because borrowAToken ’s total supply cap does not allow him to deposit enough USDC.

To mitigate this, Size provides the multicall function, that bypasses the deposit limit and allows users to carry out such actions (LOC-80 in Deposit.sol):

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/actions/Deposit.sol#L80
function executeDeposit ( State storage state, DepositParams calldata params ) public { address from = msg.

sender; uint256 amount = params.

amount; if ( msg.

value > 0 ) { // do not trust msg.value (see `Multicall.sol`) amount = address ( this ).

balance; // slither-disable-next-line arbitrary-send-eth state.

data.

weth.

deposit {value:

amount }(); state.

data.

weth.

forceApprove ( address ( this ), amount ); from = address ( this ); } if ( params.

token == address ( state.

data.

underlyingBorrowToken )) { state.

depositUnderlyingBorrowTokenToVariablePool ( from, params.

to, amount ); // borrow aToken cap is not validated in multicall, // since users must be able to deposit more tokens to repay debt if (!

state.

data.

isMulticall ) { state.

validateBorrowATokenCap (); } else { state.

depositUnderlyingCollateralToken ( from, params.

to, amount ); } emit Events.

Deposit ( params.

token, params.

to, amount ); } Let’s say a user uses the multicall function for a (deposit and repay) pair action. The multicall function checks for an invariant that restricts users from depositing more borrowATokens than required to repay the loan by restricting:

increase in borrowAToken supply <= decrease in debtToken supply

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/CapsLibrary.sol#L19
function validateBorrowATokenIncreaseLteDebtTokenDecrease ( State storage state, uint256 borrowATokenSupplyBefore, uint256 debtTokenSupplyBefore, uint256 borrowATokenSupplyAfter, uint256 debtTokenSupplyAfter ) external view { // If the supply is above the cap if ( borrowATokenSupplyAfter > state.

riskConfig.

borrowATokenCap ) { uint256 borrowATokenSupplyIncrease = borrowATokenSupplyAfter > borrowATokenSupplyBefore ?

borrowATokenSupplyAfter - borrowATokenSupplyBefore:

0; uint256 debtATokenSupplyDecrease = debtTokenSupplyBefore > debtTokenSupplyAfter ?

debtTokenSupplyBefore - debtTokenSupplyAfter:

0; // and the supply increase is greater than the debt reduction if ( borrowATokenSupplyIncrease > debtATokenSupplyDecrease ) { // revert revert Errors.

BORROW_ATOKEN_INCREASE_EXCEEDS_DEBT_TOKEN_DECREASE ( borrowATokenSupplyIncrease, debtATokenSupplyDecrease ); } // otherwise, it means the debt reduction was greater than the inflow of cash: do not revert } // otherwise, the supply is below the cap: do not revert } The problem is, this is the exact invariant that is broken. The PoC below explains how in detail.

Impact:

An invariant, which should not break, is broken. This point sets the impact of this issue to be Medium.

Likelihood:

Any user would call the Multicall function (since it is not access-controlled) and can bypass the deposit limit as well as the restriction: increase in borrowAToken supply <= decrease in debtToken supply. This makes the likelihood high.

The final severity comes to be Medium. Moreover, the multicall functionality does not work as intended, affecting the availability of the correct intended version of multicall.

## Recommended Mitigation Steps

Apply the following in multicall(State storage state, bytes[] calldata data) function:

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/Multicall.sol#L26
function multicall(State storage state, bytes[] calldata data) internal returns (bytes[] memory results) { state.data.isMulticall = true; - uint256 borrowATokenSupplyBefore = state.data.borrowAToken.balanceOf(address(this)); + uint256 borrowATokenSupplyBefore = state.data.borrowAToken.totalSupply(); uint256 debtTokenSupplyBefore = state.data.debtToken.totalSupply(); results = new bytes[](data.length); for (uint256 i = 0; i < data.length; i++) { results[i] = Address.functionDelegateCall(address(this), data[i]); } - uint256 borrowATokenSupplyBefore = state.data.borrowAToken.balanceOf(address(this)); + uint256 borrowATokenSupplyBefore = state.data.borrowAToken.totalSupply(); uint256 debtTokenSupplyAfter = state.data.debtToken.totalSupply();

state.validateBorrowATokenIncreaseLteDebtTokenDecrease( borrowATokenSupplyBefore, debtTokenSupplyBefore, borrowATokenSupplyAfter, debtTokenSupplyAfter ); state.data.isMulticall = false; }

## Assessed type

Invalid Validation aviggiano (Size) confirmed and commented via duplicate Issue #144:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/126.

Note: For full discussion, see here.

# [M-02] Users can not to buy/sell minimum credit allowed due to exactAmountIn condition

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by iam_emptyset, also found by ether_sky, 0x04bytes, samuraii77, Jorgect, radin100 ( 1, 2 ), DanielArmstrong, 0xStalin, 0xanmol, carlos__alegre, inzinko, dhank, Infect3d ( 1, 2 ), hyh, 0xarno, elhaj, zarkk01, trachev, KupiaSec, 0xRobocop, ilchovski, 0xJoyBoy03, BoltzmannBrain, pkqs90, and said

- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/BuyCreditMarket.sol#L91
- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/SellCreditMarket.sol#L93

## Impact

The minimum credit user can buy or sell can be gotten from state.riskConfig.minimumCreditBorrowAToken which was set as 5e6. However, in buyCreditMarket and sellCreditMarket user can not buy or sell minimun credit allowed or small amount above it due to the exactAmountIn condition he chose.

In buyCreditMarket:

when user set params.exactAmountIn = true, The params.amount value will be cash he want to use to buy credit. Since the params.amount value is cash it can be set less than state.riskConfig.minimumCreditBorrowAToken (5e6) as long as when the value get converted to credit it will reach the state.riskConfig.minimumCreditBorrowAToken value. But, unfortunately, in validateBuyCreditMarket() whenever params.amount is less than state.riskConfig.minimumCreditBorrowAToken the transaction will revert due to below code present in the function.

if ( params.

amount < state.

riskConfig.

minimumCreditBorrowAToken ) { revert Errors.

CREDIT_LOWER_THAN_MINIMUM_CREDIT ( params.

amount, state.

riskConfig.

minimumCreditBorrowAToken ); } For example, the cash user can use to buy minimum credit allowed ( 5e6 ) can be calculated as in the code below, And the value should be less than the minimum credit allowed ( 5e6 ).

uint256 minimumCash = Math.

mulDivUp ( 5e6, PERCENT, PERCENT + ratePerTenor ); In sellCreditMarket:

when user set params.exactAmountIn = false, The params.amount will be the exact cash he want to receive. But he will also not be able to set the params.amount value to be less than state.riskConfig.minimumCreditBorrowAToken value due to the below code present in validateSellCreditMarket().

if (params.amount < state.riskConfig.minimumCreditBorrowAToken) { revert Errors.CREDIT_LOWER_THAN_MINIMUM_CREDIT(params.amount, state.riskConfig.minimumCreditBorrowAToken); }

## Recommended Mitigation Steps

In validateBuyCreditMarket() and validateSellCreditMarket() for BuyCreditMarket and SellCreditMarket libraries respectively, there is a need of considering the condition of params.exactAmountIn value. The check should be implemented as shown below.

For BuyCreditMarket library:

inside validateBuyCreditMarket() replace below code:

if ( params.

amount < state.

riskConfig.

minimumCreditBorrowAToken ) { // @audit 5e6 USDC revert Errors.

CREDIT_LOWER_THAN_MINIMUM_CREDIT ( params.

amount, state.

riskConfig.

minimumCreditBorrowAToken ); } With the below code:

uint256 ratePerTenor = borrowOffer.

getRatePerTenor ( VariablePoolBorrowRateParams ({ variablePoolBorrowRate:

state.

oracle.

variablePoolBorrowRate, variablePoolBorrowRateUpdatedAt:

state.

oracle.

variablePoolBorrowRateUpdatedAt, variablePoolBorrowRateStaleRateInterval:

state.

oracle.

variablePoolBorrowRateStaleRateInterval }), tenor ); if ( params.

exactAmountIn && params.

amount < Math.

mulDivUp ( state.

riskConfig.

minimumCreditBorrowAToken, PERCENT, PERCENT + ratePerTenor )) { revert Errors.

CREDIT_LOWER_THAN_MINIMUM_CREDIT ( params.

amount, state.

riskConfig.

minimumCreditBorrowAToken ); } else if (!

params.

exactAmountIn && params.

amount < state.

riskConfig.

minimumCreditBorrowAToken ) { revert Errors.

CREDIT_LOWER_THAN_MINIMUM_CREDIT ( params.

amount, state.

riskConfig.

minimumCreditBorrowAToken ); } For SellCreditMarket library:

inside validateSellCreditMarket() replace below code:

if ( params.

amount < state.

riskConfig.

minimumCreditBorrowAToken ) { // @audit 5e6 USDC revert Errors.

CREDIT_LOWER_THAN_MINIMUM_CREDIT ( params.

amount, state.

riskConfig.

minimumCreditBorrowAToken ); } With the below code:

uint256 ratePerTenor = loanOffer.

getRatePerTenor ( VariablePoolBorrowRateParams ({ variablePoolBorrowRate:

state.

oracle.

variablePoolBorrowRate, variablePoolBorrowRateUpdatedAt:

state.

oracle.

variablePoolBorrowRateUpdatedAt, variablePoolBorrowRateStaleRateInterval:

state.

oracle.

variablePoolBorrowRateStaleRateInterval }), tenor ); if ( params.

exactAmountIn && params.

amount < state.

riskConfig.

minimumCreditBorrowAToken ) { revert Errors.

CREDIT_LOWER_THAN_MINIMUM_CREDIT ( params.

amount, state.

riskConfig.

minimumCreditBorrowAToken ); } else if (!

params.

exactAmountIn && params.

amount < Math.

mulDivUp ( state.

riskConfig.

minimumCreditBorrowAToken, PERCENT, PERCENT + ratePerTenor )) { revert Errors.

CREDIT_LOWER_THAN_MINIMUM_CREDIT ( params.

amount, state.

riskConfig.

minimumCreditBorrowAToken ); }

## Assessed type

Invalid Validation aviggiano (Size) confirmed and commented:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/125

# [M-03] Size uses wrong source to query available liquidity on Aave, resulting in borrow and lend operations being bricked upon mainnet deployment

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by JCN, also found by mt030d, 0xpiken, silver_eth, grearlake, elhaj, zarkk01, trachev, ilchovski, bin2chen, VAD37, and SpicyMeatball Size queries the underlyingBorrowToken balance of the variablePool during borrowing and lending actions in order to check available liquidity. If there is not enough available liquidity for the new borrower to withdraw the borrowed assets, the function call will revert:

Size::buyCreditMarket 178:

function buyCreditMarket ( BuyCreditMarketParams calldata params ) external payable override ( ISize ) whenNotPaused {...

184:

state.

validateVariablePoolHasEnoughLiquidity ( amount ); // @audit: liquidity check Size::sellCreditMarket 188:

function sellCreditMarket ( SellCreditMarketParams memory params ) external payable override ( ISize ) whenNotPaused {...

194:

state.

validateVariablePoolHasEnoughLiquidity ( amount ); // @audit: liquidity check CapsLibrary::validateVariablePoolHasEnoughLiquidity 67:

function validateVariablePoolHasEnoughLiquidity ( State storage state, uint256 amount ) public view { 68:

uint256 liquidity = state.

data.

underlyingBorrowToken.

balanceOf ( address ( state.

data.

variablePool )); // @audit: ATokens hold liquidity, not variablePool 69:

if ( liquidity < amount ) { 70:

revert Errors.

NOT_ENOUGH_BORROW_ATOKEN_LIQUIDITY ( liquidity, amount ); 71: } However, Aave actual holds all liquidity in the AToken contracts, not the Pool contracts. Therefore, the liquidity check above will always fail when the variablePool references a live instance of an actual Aave Pool. This is due to the fact that the variablePool balance will be 0 (does not hold any liquidity), causing borrowing and lending actions to revert on line 70 in CapsLibrary.sol.

The below code snippets are taken from out of scope contracts, but are shown to further explain why the test suite did not detect this vulnerability:

SupplyLogic::executeSupply 52:

function executeSupply (...

67:

IERC20 ( params.

asset ).

safeTransferFrom ( msg.

sender, reserveCache.

aTokenAddress, params.

amount ); As shown above, the actual implementation of the Aave pool will transfer supplied assets to the aTokenAddress on supplies. The Aave pool implementation will then transfer the assets from the AToken to the recipient during withdraws, since the liquidity is stored in the AToken contract:

SupplyLogic::executeWithdraw 106:

function executeWithdraw (...

139:

IAToken ( reserveCache.

aTokenAddress ).

burn ( 140:

msg.

sender, 141:

params.

to, 142:

amountToWithdraw, 143:

reserveCache.

nextLiquidityIndex 144: ); AToken::burn 96:

function burn ( 97:

address from, 98:

address receiverOfUnderlying, 99:

uint256 amount, 100:

uint256 index 101: ) external virtual override onlyPool { 102:

_burnScaled ( from, receiverOfUnderlying, amount, index ); 103:

if ( receiverOfUnderlying != address ( this )) { 104:

IERC20 ( _underlyingAsset ).

safeTransfer ( receiverOfUnderlying, amount ); However, Size uses a PoolMock contract for their test suite, which implements logic that differs from actual Aave Pool contracts:

PoolMock.sol#L67-L78 67:

function supply ( address asset, uint256 amount, address onBehalfOf, uint16 ) external { 68:

Data memory data = datas [ asset ]; 69:

IERC20Metadata ( asset ).

transferFrom ( msg.

sender, address ( this ), amount ); // @audit: underlying transferred from supplier to PoolMock contract 70:

data.

aToken.

mint ( address ( this ), onBehalfOf, amount, data.

reserveIndex ); 71: } 72:

73:

function withdraw ( address asset, uint256 amount, address to ) external returns ( uint256 ) { 74:

Data memory data = datas [ asset ]; 75:

data.

aToken.

burn ( msg.

sender, address ( data.

aToken ), amount, data.

reserveIndex ); // @audit: no assets transferred from AToken 76:

IERC20Metadata ( asset ).

safeTransfer ( to, amount ); // @audit: underlying transferred from PoolMock contract to supplier 77:

return amount; 78: } As shown above, the PoolMock contract transfers all supplied assets to the PoolMock contract itself, not the AToken contract. This differs from how actual Aave Pools handle liquidity on live networks.

## Impact

Borrow ( sellCreditMarket ) and lend ( buyCreditMarket ) operations will be bricked when Size is deployed and integrates with Aave on a live network. This report is labeled as High due to the fact that the main functionalities of this lending protocol will be unusable upon deployment.

## Recommended Mitigation

When checking available liquidity on Aave, Size should query the underlyingBorrowToken balance of the AToken instead of the variablePool:

diff --git a/./src/libraries/CapsLibrary.sol b/./src/libraries/CapsLibrary.sol index 7e35e90..b7ce7a1 100644 --- a/./src/libraries/CapsLibrary.sol +++ b/./src/libraries/CapsLibrary.sol @@ -65,9 +65,11 @@ library CapsLibrary { /// @param state The state struct /// @param amount The amount of cash to withdraw function validateVariablePoolHasEnoughLiquidity(State storage state, uint256 amount) public view { - uint256 liquidity = state.data.underlyingBorrowToken.balanceOf(address(state.data.variablePool)); + address aToken = state.data.variablePool.getReserveData(address(state.data.underlyingBorrowToken)).aTokenAddress; + uint256 liquidity = state.data.underlyingBorrowToken.balanceOf(aToken); if (liquidity < amount) {

revert Errors.NOT_ENOUGH_BORROW_ATOKEN_LIQUIDITY(liquidity, amount); } aviggiano (Size) confirmed and commented:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/127
samuraii77 (warden) commented:

Hi, the severity of this issue should be a medium. Please take a look at what a high severity issue should be:

Assets can be stolen/lost/compromised directly (or indirectly if there is a valid attack path that does not have hand-wavy hypotheticals).

This issue does not cause such an issue. In fact, it can be completely mitigated by redeploying the contract or upgrading the contract (if it is upgradeable, I am not sure if it was) whenever the issue is noticed which would be extremely quickly as it would occur on the first deposit of USDC essentially causing nothing else than some wasted time and headache.

hansfriese (judge) decreased severity to Medium and commented:

I agree. Medium is more appropriate as there is no direct fund loss.

# [M-04] Inadequate checks to confirm the correct status of the sequence/ sequencerUptimeFeed in PriceFeed.getPrice() contract

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

sequencerUptimeFeed in PriceFeed.getPrice() contract Submitted by adeolu, also found by serial-coder The PriceFeed contract has sequencerUptimeFeed checks in place to assert if the sequencer on an L2 is running but these checks are not implemented correctly. The chainlink docs say that sequencerUptimeFeed can return a 0 value for startedAt if it is called during an “invalid round”.

startedAt:

This timestamp indicates when the sequencer changed status. This timestamp returns 0 if a round is invalid. When the sequencer comes back up after an outage, wait for the GRACE_PERIOD_TIME to pass before accepting answers from the data feed. Subtract startedAt from block.timestamp and revert the request if the result is less than the GRACE_PERIOD_TIME.

If the sequencer is up and the GRACE_PERIOD_TIME has passed, the function retrieves the latest answer from the data feed using the dataFeed object.

Please note that an “invalid round” is described to mean there was a problem updating the sequencer’s status, possibly due to network issues or problems with data from oracles, and is shown by a startedAt time of 0 and answer is 0. Further explanation can be seen as given by an official chainlink engineer as seen here in the chainlink public discord Note: to view the provided image, please see the original submission here.

This makes the implemented check below in the PriceFeed.getPrice() to be useless if its called in an invalid round.

if ( block.

timestamp - startedAt <= GRACE_PERIOD_TIME ) { revert Errors.

GRACE_PERIOD_NOT_OVER (); } As startedAt will be 0, the arithmetic operation block.timestamp - startedAt will result in a value greater than GRACE_PERIOD_TIME (which is hardcoded to be 3600). I.e., block.timestamp = 1719739032, so 1719739032 - 0 = 1719739032 which is bigger than 3600. The code won’t revert.

Imagine a case where a round starts, at the beginning startedAt is recorded to be 0, and answer, the initial status is set to be 0. Note that docs say that if answer = 0, sequencer is up, if equals to 1, sequencer is down. But in this case here, answer and startedAt can be 0 initially, until after all data is gotten from oracles and update is confirmed then the values are reset to the correct values that show the correct status of the sequencer.

From these explanations and information, it can be seen that startedAt value is a second value that should be used in the check for if a sequencer is down/up or correctly updated. The checks in PriceFeed.getPrice() will allow for successfull calls in an invalid round because reverts don’t happen if answer == 0 and startedAt == 0 thus defeating the purpose of having a sequencerFeed check to ascertain the status of the sequencerFeed on L2 (i.e., if it is up/down/active or if its status is actually confirmed to be either).

- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/oracle/PriceFeed.sol#L68C1-L76C14
if ( answer == 1 ) { // sequencer is down revert Errors.

SEQUENCER_DOWN (); } if ( block.

timestamp - startedAt <= GRACE_PERIOD_TIME ) { // time since up revert Errors.

GRACE_PERIOD_NOT_OVER (); }

## Impact

Inadequate checks to confirm the correct status of the sequencer/ sequencerUptimeFeed in PriceFeed.getPrice() contract will cause getPrice() to not revert even when the sequencer uptime feed is not updated or is called in an invalid round.

## Recommended Mitigation Steps

Add a check that reverts if startedAt is returned as 0.

## Assessed type

Oracle aviggiano (Size) confirmed and commented:

This looks valid. It’s weird that the Chainlink sample code does not implement the check suggested by the documentation, as the warden points out.

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/140.
CC:

- https://github.com/smartcontractkit/documentation/pull/1995.

# [M-05] Users may incur an unexpected fragmentation fee in the compensate() call

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

compensate() call Submitted by mt030d, also found by pkqs90

- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/Compensate.sol#L116
- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/Compensate.sol#L136
- https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/Compensate.sol#L146-L155

## Impact

The CompensateParams struct lacks a field indicating the minimum amount the user is willing to compensate in the compensate() call. Consequently, users might pay an unexpected fragmentation fee during the compensate() call.

In scenarios where a borrower has a credit position that can be used to compensate the debt ( creditPositionToCompensateId ), they could search the market for a credit position ( creditPositionWithDebtToRepayId ) that relates to the debt and has the same credit amount as creditPositionToCompensateId. This way, in the compensate() call, the borrower would not fragment the creditPositionToCompensateId and avoid paying the fragmentation fee.

However, unexpectedly to the borrower, they might still pay the fragmentation fee if the compensate() transaction is executed after a buyCreditMarket() or sellCreditMarket() transaction that decreases the credit in creditPositionWithDebtToRepayId.

uint256 amountToCompensate = Math.min(params.amount, creditPositionWithDebtToRepay.credit); uint256 exiterCreditRemaining = creditPositionToCompensate.credit - amountToCompensate; if (exiterCreditRemaining > 0) { // charge the fragmentation fee in collateral tokens, capped by the user balance uint256 fragmentationFeeInCollateral = Math.min( state.debtTokenAmountToCollateralTokenAmount(state.feeConfig.fragmentationFee), state.data.collateralToken.balanceOf(msg.sender) ); state.data.collateralToken.transferFrom( msg.sender, state.feeConfig.feeRecipient, fragmentationFeeInCollateral ); } In this case, the amountToCompensate will be less than the credit in creditPositionToCompensateId, and the user will have to pay an unforeseen fragmentation fee.

This situation can occur in normal user flows. Moreover, a malicious user could exploit this vulnerability and front-running a borrower’s compensate() transaction to cause the borrower to pay an unforeseen fragmentation fee.

## Recommended Mitigation Steps

Allow the user to input a minAmount parameter in the compensate() function to specify the minimum amount they are willing to use. Then, handle this parameter within the compensate() function.

aviggiano (Size) acknowledged and commented:

Additional feedback from the team:

I think this is technically valid since, although frontrunning can happen, we can implement measures in the protocol to prevent users from being unexpectedly charged fees. For example, setting slippage protection. compensate could have a parameter allowing the user to specify “revert if a fee is charged”.

Yes, it is another instance of having a proper Anti-MEV system because the same “issue” is true on the other side, meaning somebody trying to buy a credit that gets compensated entirely before his buyCreditMarket() can be executed but on buy and sell we already Anti-MEV params. That’s kind of minor since no attack that benefits anybody, but the protocol, can be run here so up to you.

I think the potential issue of users paying for fees unexpectedly is not as important as the UX problem of operations failing because of the order they are applied to the orderbook, as we are aware. So adding a new parameter to the function doesn’t hurt but doesn’t solve the problem either.

hansfriese (judge) commented:

Will keep as Medium since it might occur during normal interactions.

# [M-06] Neither sellCreditMarket() nor compensate() checks whether the credit position to be sold is allowed for sale

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

sellCreditMarket() nor compensate() checks whether the credit position to be sold is allowed for sale Submitted by 0xpiken, also found by adeolu, hyh, and Shield

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/actions/SellCreditMarket.sol#L51-L122
- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/actions/Compensate.sol#L42-L101

## Impact

A credit buyer could be forced to buy a not for sale credit position with no way to sell it passively, potentially suffering a loss due to missing an arbitrage opportunity.

## Recommended Mitigation Steps

No one should be allowed to sell their not for sale credit positions or use them to compensate their debts:

function validateSellCreditMarket(State storage state, SellCreditMarketParams calldata params) external view { LoanOffer memory loanOffer = state.data.users[params.lender].loanOffer; uint256 tenor; // validate msg.sender // N/A // validate lender if (loanOffer.isNull()) { revert Errors.INVALID_LOAN_OFFER(params.lender); } // validate creditPositionId if (params.creditPositionId == RESERVED_ID) { tenor = params.tenor; // validate tenor if (tenor < state.riskConfig.minTenor || tenor > state.riskConfig.maxTenor) { revert Errors.TENOR_OUT_OF_RANGE(tenor, state.riskConfig.minTenor, state.riskConfig.maxTenor); } } else { CreditPosition storage creditPosition = state.getCreditPosition(params.creditPositionId);

DebtPosition storage debtPosition = state.getDebtPositionByCreditPositionId(params.creditPositionId); if (msg.sender != creditPosition.lender) { revert Errors.BORROWER_IS_NOT_LENDER(msg.sender, creditPosition.lender); } if (!state.isCreditPositionTransferrable(params.creditPositionId)) { revert Errors.CREDIT_POSITION_NOT_TRANSFERRABLE( params.creditPositionId, state.getLoanStatus(params.creditPositionId), state.collateralRatio(debtPosition.borrower) ); } + User storage user = state.data.users[creditPosition.lender]; + if (user.allCreditPositionsForSaleDisabled || !creditPosition.forSale) { + revert Errors.CREDIT_NOT_FOR_SALE(params.creditPositionId); + } tenor = debtPosition.dueDate - block.timestamp; // positive since the credit position is transferrable, so the loan must be ACTIVE

// validate amount if (params.amount > creditPosition.credit) { revert Errors.NOT_ENOUGH_CREDIT(params.amount, creditPosition.credit); } // validate amount if (params.amount < state.riskConfig.minimumCreditBorrowAToken) { revert Errors.CREDIT_LOWER_THAN_MINIMUM_CREDIT(params.amount, state.riskConfig.minimumCreditBorrowAToken); } // validate tenor if (block.timestamp + tenor > loanOffer.maxDueDate) { revert Errors.DUE_DATE_GREATER_THAN_MAX_DUE_DATE(block.timestamp + tenor, loanOffer.maxDueDate); } // validate deadline if (params.deadline < block.timestamp) { revert Errors.PAST_DEADLINE(params.deadline); } // validate maxAPR uint256 apr = loanOffer.getAPRByTenor( VariablePoolBorrowRateParams({ variablePoolBorrowRate: state.oracle.variablePoolBorrowRate,

variablePoolBorrowRateUpdatedAt: state.oracle.variablePoolBorrowRateUpdatedAt, variablePoolBorrowRateStaleRateInterval: state.oracle.variablePoolBorrowRateStaleRateInterval }), tenor ); if (apr > params.maxAPR) { revert Errors.APR_GREATER_THAN_MAX_APR(apr, params.maxAPR); } // validate exactAmountIn // N/A } function validateCompensate(State storage state, CompensateParams calldata params) external view { CreditPosition storage creditPositionWithDebtToRepay = state.getCreditPosition(params.creditPositionWithDebtToRepayId); DebtPosition storage debtPositionToRepay = state.getDebtPositionByCreditPositionId(params.creditPositionWithDebtToRepayId); uint256 amountToCompensate = Math.min(params.amount, creditPositionWithDebtToRepay.credit);

// validate creditPositionWithDebtToRepayId if (state.getLoanStatus(params.creditPositionWithDebtToRepayId) != LoanStatus.ACTIVE) { revert Errors.LOAN_NOT_ACTIVE(params.creditPositionWithDebtToRepayId); } // validate creditPositionToCompensateId if (params.creditPositionToCompensateId == RESERVED_ID) { uint256 tenor = debtPositionToRepay.dueDate - block.timestamp; // validate tenor if (tenor < state.riskConfig.minTenor || tenor > state.riskConfig.maxTenor) { revert Errors.TENOR_OUT_OF_RANGE(tenor, state.riskConfig.minTenor, state.riskConfig.maxTenor); } } else { CreditPosition storage creditPositionToCompensate = state.getCreditPosition(params.creditPositionToCompensateId); DebtPosition storage debtPositionToCompensate =

state.getDebtPositionByCreditPositionId(params.creditPositionToCompensateId); if (!state.isCreditPositionTransferrable(params.creditPositionToCompensateId)) { revert Errors.CREDIT_POSITION_NOT_TRANSFERRABLE( params.creditPositionToCompensateId, state.getLoanStatus(params.creditPositionToCompensateId), state.collateralRatio(debtPositionToCompensate.borrower) ); } + User storage user = state.data.users[creditPosition.lender]; + if (user.allCreditPositionsForSaleDisabled || !creditPosition.forSale) { + revert Errors.CREDIT_NOT_FOR_SALE(params.creditPositionId); + } if ( debtPositionToRepay.dueDate < state.getDebtPositionByCreditPositionId(params.creditPositionToCompensateId).dueDate ) { revert Errors.DUE_DATE_NOT_COMPATIBLE(

params.creditPositionWithDebtToRepayId, params.creditPositionToCompensateId ); } if (creditPositionToCompensate.lender != debtPositionToRepay.borrower) { revert Errors.INVALID_LENDER(creditPositionToCompensate.lender); } if (params.creditPositionToCompensateId == params.creditPositionWithDebtToRepayId) { revert Errors.INVALID_CREDIT_POSITION_ID(params.creditPositionToCompensateId); } amountToCompensate = Math.min(amountToCompensate, creditPositionToCompensate.credit); } // validate msg.sender if (msg.sender != debtPositionToRepay.borrower) { revert Errors.COMPENSATOR_IS_NOT_BORROWER(msg.sender, debtPositionToRepay.borrower); } // validate amount if (amountToCompensate == 0) { revert Errors.NULL_AMOUNT();

}

## Assessed type

Context aviggiano (Size) confirmed and commented:

This was not very well specified in our documentation, so it might be an issue.

On the one hand, not enforcing forSale for credit owners is good because we do not want to force users to do 2 actions if they are the ones selling them. On the other hand, passive buyers can end up with credits that can’t be resold.

Additional feedback from the team:

I think we should reset the variable upon sale.

So I think this issue is technically valid.

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/130.

hansfriese (judge) commented:

The root cause is that sellCreditMarket() and compensate() don’t check forSale flag of a credit position. ( creditPositionToCompensateId in compensate() ).

There are two impacts:

The new lender would have a credit position of forSale = false In case of a full sale. ( #184, #335 ) The credit position’s forSale would be changed from false to true in case of a partial sale. ( #228 ) Will consider them as duplicates because they have the same root cause and can be mitigated by resetting the flag consistently upon sale.

Note: For full discussion, see here.

# [M-07] Credit can be sold forcibly as forSale setting can be ignored via Compensate

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

forSale setting can be ignored via Compensate Submitted by hyh, also found by 0xAlix2, almurhasan, BenRai, 0xStalin, and bin2chen Any credit position can be forcibly sold with the help of its borrower. This will be executed only when have expected profit; for example, when lender’s curve is not null and is above the market it is profitable to buy credit from them (lend to them at above market rates), but they might block it with the lack of free collateral and the forSale = false flag, which can be overridden by the corresponding borrower of this credit position. The impact of this forced sale is proportional to interest rate volatility and can be substantial. There are no additional prerequisites for the setup.

## Recommended Mitigation Steps

Consider passing the flag to createDebtAndCreditPositions() indicating forSale flag to be set, which be passed from the existing credit in Compensate.sol#L139, so it won’t be changed.

aviggiano (Size) confirmed and commented:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/130.

hansfriese (judge) decreased severity to Medium and commented:

Valid finding.

Medium is more appropriate due to the below reasons.

Buying the lender’s credit position doesn’t mean any loss to him as it’s sold with his borrow offer. (It’s just an unintended behavior.) The lender can prevent this by setting allCreditPositionsForSaleDisabled = false.

# [M-08] Sandwich attack on loan fulfillment will temporarily prevent users from accessing their borrowed funds

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by gesha17, also found by grearlake, samuraii77, mt030d, KupiaSec, and VAD37 The protocol does not remove borrowed token liquidity from the aave pool once a loan is initiated. It only checks if there is enough available liquidity via the validateVariablePoolHasEnoughLiquidity() function. A user is expected to withdraw their funds after the loan is created. This means that a malicious user can sandwich loans that exceed the available liquidity in aave by depositing liquidity in a frontrun transaction and removing it in a backrun transaction. This way a user that takes out a loan will not be able to remove their funds until there is enough available liquidity in the pool.

Also note that liquidity may become unavailable via other factors, like external users removing liquidity from aave.

## Recommended Mitigation Steps

Remove the liquidity from the pool when the loan is created and still let the user pull their funds in a separate transaction to remain compliant with pull over push pattern.

aviggiano (Size) acknowledged and commented:

We acknowledge this issue, but we understand that MEV attacks like frontrunning, sandwich attacks, etc., do not belong to the protocol layer. For example, the on-chain Uniswap logic can’t do anything against sandwich attacks.

Also, this can be mitigated with a multicall.

I am not sure how Code4rena judges will weigh in on this issue, but since it was not explicitly stated on the list of Known Limitations, we have decided to make it more explicit in our documentation here.

hansfriese (judge) commented:

Will keep as Medium since it accurately highlights the irrationality of the current validation process.

# [M-09] Borrower is not able to compensate his lenders if he is underwater

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by ilchovski, also found by 0xAlix2, alix40, dhank, zzebra83, ether_sky, and pkqs90 A borrower of a loan could have other credit positions with borrowers that have healthy CRs which he could use to compensate his lenders to avoid liquidations and improve his CR. However, he is not able to do this via compensate() and he is forced to sell his credit positions via sellCreditMarket() or sellCreditLimit(). The complications of this are described in the example below.

## Recommended Mitigation Steps

The CR check at the end of compensate is important because fragmentation fees could occur and lower the CR and make it under the healthy threshold in some specific situations.

What I propose as a solution is measuring the CR before and after the compensate() logic and it should revert if the CR is becoming worse.

This way Bob will be able to improve his CR by using his credit positions even if he has to compensate multiple times before becoming healthy again.

aviggiano (Size) confirmed and commented:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/129

# [M-10] withdraw() users may can’t withdraw underlyingBorrowToken properly

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

withdraw() users may can’t withdraw underlyingBorrowToken properly Submitted by bin2chen, also found by 0xAlix2, 3n0ch, samuraii77, ayden, inzinko, zarkk01, elhaj, Bob, alix40, and ether_sky We can withdraw underlyingCollateralToken and underlyingBorrowToken by withdraw():

function withdraw ( WithdrawParams calldata params ) external payable override ( ISize ) whenNotPaused { state.

validateWithdraw ( params ); state.

executeWithdraw ( params ); @> state.

validateUserIsNotBelowOpeningLimitBorrowCR ( msg.

sender ); } function executeWithdraw ( State storage state, WithdrawParams calldata params ) public { uint256 amount; if ( params.

token == address ( state.

data.

underlyingBorrowToken )) { amount = Math.

min ( params.

amount, state.

data.

borrowAToken.

balanceOf ( msg.

sender )); if ( amount > 0 ) { @> state.

withdrawUnderlyingTokenFromVariablePool ( msg.

sender, params.

to, amount ); } else { amount = Math.

min ( params.

amount, state.

data.

collateralToken.

balanceOf ( msg.

sender )); if ( amount > 0 ) { state.

withdrawUnderlyingCollateralToken ( msg.

sender, params.

to, amount ); } emit Events.

Withdraw ( params.

token, params.

to, amount ); } From the code above we know that whether we take underlyingCollateralToken or underlyingBorrowToken will check validateUserIsNotBelowOpeningLimitBorrowCR() ==> collateralRatio() > openingLimitBorrowCR.

This makes sense for taking underlyingCollateralToken, but not for taking underlyingBorrowToken.

Taking the underlyingBorrowToken does not affect the collateralRatio.

The user has already borrowed the funds (with interest accrued and collateralized), it is the user’s asset, and should be able to be withdrawn at will, even if it may be liquidated.

openingLimitBorrowCR is still far from being liquidated, and should not restrict the user from withdrawing the borrowed token.

## Impact

If the token is already borrowed, just stored in Size and not yet taken, but due to a slight price fluctuation, validateUserIsNotBelowOpeningLimitBorrowCR() fails but is still far from being liquidated, the user may not be able to take the borrowed token, resulting in the possibility that the user may not be able to withdraw the borrowed funds.

## Recommended Mitigation

function withdraw(WithdrawParams calldata params) external payable override(ISize) whenNotPaused { state.validateWithdraw(params); state.executeWithdraw(params); + if (params.token != address(state.data.underlyingBorrowToken) { + state.validateUserIsNotBelowOpeningLimitBorrowCR(msg.sender); + } }

## Assessed type

Context aviggiano (Size) confirmed and commented:

This is a valid bug and the report is good.

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/124.

samuraii77 (warden) commented:

Hi, this issue should be of high severity. The likelihood is very high, imagine a user borrows $1000 and gives $1500 as collateral; his collateral ratio is now 1.5e18. If the price of ETH drops by just 1 cent (or even less), his borrowed funds will be locked. The impact is also high as well.

hansfriese (judge) commented:

I still believe Medium is appropriate for the following reasons:

The user can withdraw the borrowed funds at the time of borrowing using a multicall.

The user can still use the borrowed funds to repay their loan.

There is no fund loss; the funds are simply locked for a while.

# [M-11] LiquidateWithReplacement does not charge swap fees on the borrower

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

LiquidateWithReplacement does not charge swap fees on the borrower Submitted by pkqs90, also found by 0xRobocop, BenRai, Brenzee, 0xStalin, alix40, 3n0ch, Honour, silver_eth, trachev, Nyx, KupiaSec, ether_sky, and VAD37 When a user places a credit sell limit order, it may be either:

Bought by other users.

Bought by the LiquidateWithReplacement action.

However, for case number 2, it does not charge a swap fee for the borrower, which contradicts to the swap fee definition.

Bug Description Let’s first see how LiquidateWithReplacement works. There are two steps:

Liquidation: Liquidator sends the amount of debt debt.futureValue to the protocol, and receive collateral tokens as liquidation reward.

Buy credit: Use the original lender’s debt.futureValue as credit to buy credit from the borrower. This way the original lender can still get the same amount of credit at the same dueDate. The liquidator can then take away the difference between debt.futureValue and the amount sent to the borrower.

Note that step 2 is the same as buying credit using the BuyCreditMarket function.

However, according to the docs, all cash and credit swaps should charge swap fees on the borrower. The issue here is the swap in LiquidateWithReplacement does not charge swap fees.

Protocol fee; charged to the recipient of USDC on cash->credit and credit->cash swaps. Prorated based on credit tenor (the fee is annualized).

function executeLiquidateWithReplacement ( State storage state, LiquidateWithReplacementParams calldata params ) external returns ( uint256 issuanceValue, uint256 liquidatorProfitCollateralToken, uint256 liquidatorProfitBorrowToken ) { emit Events.

LiquidateWithReplacement ( params.

debtPositionId, params.

borrower, params.

minimumCollateralProfit ); DebtPosition storage debtPosition = state.

getDebtPosition ( params.

debtPositionId ); DebtPosition memory debtPositionCopy = debtPosition; BorrowOffer storage borrowOffer = state.

data.

users [ params.

borrower ].

borrowOffer; uint256 tenor = debtPositionCopy.

dueDate - block.

timestamp; liquidatorProfitCollateralToken = state.

executeLiquidate ( LiquidateParams ({ debtPositionId:

params.

debtPositionId, minimumCollateralProfit:

params.

minimumCollateralProfit }) ); uint256 ratePerTenor = borrowOffer.

getRatePerTenor ( VariablePoolBorrowRateParams ({ variablePoolBorrowRate:

state.

oracle.

variablePoolBorrowRate, variablePoolBorrowRateUpdatedAt:

state.

oracle.

variablePoolBorrowRateUpdatedAt, variablePoolBorrowRateStaleRateInterval:

state.

oracle.

variablePoolBorrowRateStaleRateInterval }), tenor ); > issuanceValue = Math.

mulDivDown ( debtPositionCopy.

futureValue, PERCENT, PERCENT + ratePerTenor ); liquidatorProfitBorrowToken = debtPositionCopy.

futureValue - issuanceValue; debtPosition.

borrower = params.

borrower; debtPosition.

futureValue = debtPositionCopy.

futureValue; debtPosition.

liquidityIndexAtRepayment = 0; emit Events.

UpdateDebtPosition ( params.

debtPositionId, debtPosition.

borrower, debtPosition.

futureValue, debtPosition.

liquidityIndexAtRepayment ); state.

data.

debtToken.

mint ( params.

borrower, debtPosition.

futureValue ); > state.

data.

borrowAToken.

transferFrom ( address ( this ), params.

borrower, issuanceValue ); state.

data.

borrowAToken.

transferFrom ( address ( this ), state.

feeConfig.

feeRecipient, liquidatorProfitBorrowToken ); }

## Recommended Mitigation Steps

Also charge swap fees during executeLiquidateWithReplacement.

aviggiano (Size) acknowledged and commented:

Feedback from the team:

Technically valid observation.

Whether we fix it or not is a product decision.

Technically this is a valid one since it is a credit for cash operation, since the chosen borrower sells his credit for cash.

Just imagine what would happen if that credit sell limit order would have been filled by a credit buy market order: the swap fee would have been charged, and liquidateWithReplacement() is essentially not different from doing a credit buy limit order filling a credit sell limit order.

From a product perspective it is not clear if it was decided to treat this in a special way, i.e., not to charge the swap fee on the credit seller, as it should be, as a form of incentive since it is highly desirable from the protocol perspective to have many such offers as the protocol earns a lot from a liquidation with replacement.

The liquidateWithReplacement() leads to the same outcome of the original loan for the lender and it should be indistinguishable from a standard credit sell limit order being filled for the borrower.

So regarding the latter, for consistency, the borrower should pay the swap fee; however, it is fair to observe the protocol already makes a big profit from liquidateWithReplacement() since it takes the full time value of money that otherwise in case of a standard liquidation would have been earned entirely by the lender, and since atm liquidateWithReplacement() atm has some limitations (see the auditors remarks about the problem of running liquidateWithReplacement() on large loans, due to the inability of fractionalize debt) then it could make sense to incentivize it.

I say “could” because that’s a product decision, not relevant for the protocol accounting.

hansfriese (judge) commented:

Thanks for the detailed feedback. Will keep as a valid Medium.

# [M-12] executeBuyCreditMarket returns the wrong amount of cash and overestimates the amount that needs to be checked in the variable pool

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

executeBuyCreditMarket returns the wrong amount of cash and overestimates the amount that needs to be checked in the variable pool Submitted by said, also found by ether_sky, zhaojohnson, Honour, samuraii77, inzinko, trachev, alix40, hyh, and KupiaSec When executeBuyCreditMarket is called and returns the cash amount, it will return the cash amount without deducting the fee. This results in overestimating the value that needs to be validated by validateVariablePoolHasEnoughLiquidity.

## Recommended Mitigation Steps

Update the returned cashAmountIn to cashAmountIn - fees at the end of executeBuyCreditMarket.

function executeBuyCreditMarket(State storage state, BuyCreditMarketParams memory params) external >>> returns (uint256 cashAmountIn) { //...

uint256 creditAmountOut; uint256 fees; if (params.exactAmountIn) { cashAmountIn = params.amount; (creditAmountOut, fees) = state.getCreditAmountOut({ cashAmountIn: cashAmountIn, maxCashAmountIn: params.creditPositionId == RESERVED_ID ? cashAmountIn: Math.mulDivUp(creditPosition.credit, PERCENT, PERCENT + ratePerTenor), maxCredit: params.creditPositionId == RESERVED_ID ? Math.mulDivDown(cashAmountIn, PERCENT + ratePerTenor, PERCENT): creditPosition.credit, ratePerTenor: ratePerTenor, tenor: tenor }); } else { creditAmountOut = params.amount; (cashAmountIn, fees) = state.getCashAmountIn({ creditAmountOut: creditAmountOut, maxCredit: params.creditPositionId == RESERVED_ID ? creditAmountOut: creditPosition.credit,

ratePerTenor: ratePerTenor, tenor: tenor }); } //...

state.data.borrowAToken.transferFrom(msg.sender, borrower, cashAmountIn - fees); state.data.borrowAToken.transferFrom(msg.sender, state.feeConfig.feeRecipient, fees); + cashAmountIn = cashAmountIn - fees; }

## Assessed type

Invalid Validation aviggiano (Size) confirmed and commented:

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/133.

This is arguably a Low -severity issue since the likelihood is Low Aave being illiquid.

Order reverting by a fee amount, which is of the order of a percentage point on the dollar.

Impact is Low (order reverting), but maybe a smaller order could succeed.

hansfriese (judge) commented:

After consideration, keeping it as a valid Medium to remain consistent with #152.

Note: For full discussion, see here.

# [M-13] Fragmentation fee is not taken if user compensates with newly created position

- **Contest:** Size
- **Slug:** 2024-06-size
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-size
- **Source snapshot:** competitions/2024-06-size/final_report.html

Submitted by SpicyMeatball, also found by 0xRobocop, 0xAlix2, samuraii77, DanielArmstrong, Rhaydden, ubl4nk, alix40, 0xStalin, asui, zanderbyte, carlos__alegre, Honour, lanrebayode77, Infect3d, elhaj, Nihavent, Bigsam, trachev, 0xRstStn, Jorgect, hyh, serial-coder, KupiaSec, shaflow2, pkqs90, Sentryx, said, and zzebra83

- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/actions/Compensate.sol#L119-L125
- https://github.com/code-423n4/2024-06-size/blob/main/src/libraries/actions/Compensate.sol#L136

## Impact

User can split his debt credit using the compensate function without paying the fragmentation fee.

## Recommended Mitigation Steps

+ uint256 initialCredit = params.creditPositionToCompensateId == RESERVED_ID ? creditPositionWithDebtToRepay.credit: creditPositionToCompensate.credit; + uint256 exiterCreditRemaining = initialCredit - amountToCompensate; aviggiano (Size) confirmed and commented:

This report correctly identifies the root cause and correctly provides a mitigation. This could be the primary issue.

Fixed in

- https://github.com/SizeCredit/size-solidity/pull/120.
