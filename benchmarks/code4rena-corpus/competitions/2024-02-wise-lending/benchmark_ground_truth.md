# Benchmark Ground Truth: Wise Lending 

## Accepted H/M Findings

# Accepted H/M Findings: Wise Lending 

# [H-01] Exploitation of the receive Function to Steal Funds

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by 0xCiphky, also found by t0x1c The WiseLending contract incorporates a reentrancy guard through its syncPool modifier, specifically within the _syncPoolBeforeCodeExecution function. This guard is meant to prevent reentrancy during external calls, such as in the withdrawExactAmountETH function, which processes ETH withdrawals for users.

However, there is currently a way to reset this guard, allowing for potential reentrant attacks during external calls. The WiseLending contract includes a receive function designed to automatically redirect all ETH sent directly to it (apart from transactions from the WETH address) to a specified master address.

To forward the ETH the _sendValue function is used, here the sendingProgress variable (which is used for reentrancy checks) is set to true to denote the start of the transfer process and subsequently reset to false following the completion of the call.

function _sendValue ( address _recipient, uint256 _amount ) internal { if ( address ( this ).

balance < _amount ) { revert AmountTooSmall (); } sendingProgress = true; ( bool success, ) = payable ( _recipient ).

call { value:

_amount }( "" ); sendingProgress = false; if ( success == false ) { revert SendValueFailed (); } As a result, an attacker could bypass an active reentrancy guard by initiating the receive function, effectively resetting the sendingProgress variable. This action clears the way for an attacker to re-enter any function within the contract, even those protected by the reentrancy guard.

Having bypassed the reentrancy protection, let’s see how this vulnerability could be leveraged to steal funds from the contract.

The withdrawExactAmountETH function allows users to withdraw their deposited shares from the protocol and receive ETH, this function also contains a healthStateCheck to ensure post withdrawal a users position is still in a healthy state. Note that this health check is done after the external call that pays out the user ETH, this will be important later on.

The protocol also implements a paybackBadDebtForToken function that allows users to pay off any other users bad debt and receive a 5% incentive for doing so.

To understand how this can be exploited, consider the following example:

User A deposits 1 ETH into the protocol.

User A borrows 0.5 ETH.

User A calls withdrawExactAmountETH to withdraw 1 ETH.

User A reenters the contract through the external call.

User A resets the reentrancy guard with a direct transfer of 0.001 ETH to the WiseLending contract.

Next, User A calls the paybackBadDebtForToken function to settle their own 0.5 ETH loan, which, due to the withdrawal, is now classified as bad debt. This not only clears the debt but also secures 0.5 ETH plus an additional incentive for User A.

With the bad debt cleared, the healthStateCheck within the withdrawal function is successfully passed.

Consequently, User A manages to retrieve their initial 1 ETH deposit and gain an additional 0.5 ETH (plus the incentive for paying off bad debt).

## Impact

This vulnerability allows an attacker to illicitly withdraw funds from the contract through the outlined method. Additionally, the exploit could also work using the contract’s liquidation process instead.

Tools Used Foundry Recommendation Edit the _sendValue function to include a reentrancy check. This ensures that the reentrancy guard is first checked, preventing attackers from exploiting this function as a reentry point. This will also not disrupt transfers from the WETH address as those don’t go through the _sendValue function.

function _sendValue ( address _recipient, uint256 _amount ) internal { if ( address ( this ).

balance < _amount ) { revert AmountTooSmall (); } _checkReentrancy (); //add here sendingProgress = true; ( bool success, ) = payable ( _recipient ).

call { value:

_amount }( "" ); sendingProgress = false; if ( success == false ) { revert SendValueFailed (); } vonMangoldt (Wise Lending) commented via duplicate issue #40:

Good catch but we don’t consider it a high since no userFunds relevant state variables are changed after sending the value. And since such a call would encapsulate the borrowrate check at the end, everything works as planned and it cannot be used to block funds or extract value or drain funds or anything. Still good to add a reentrancy check to receive function just in case.

UPDATE EDIT: Ok, you also submitted a stealing of funds POC we will take a look.

vonMangoldt (Wise Lending) commented via duplicate issue #40:

Seems like this doesn’t endanger users. A user is able to borrow beyond his allowed limit. It’s basically just a flashloan without permission?

Wise Lending commented:

Mitigated here.

# [H-02] User can erase their position debt for free

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by Dup1337

- https://github.com/code-423n4/2024-02-wise-lending/blob/79186b243d8553e66358c05497e5ccfd9488b5e2/contracts/FeeManager/FeeManager.sol#L816-L866
- https://github.com/code-423n4/2024-02-wise-lending/blob/79186b243d8553e66358c05497e5ccfd9488b5e2/contracts/MainHelper.sol#L667-L727

## Vulnerability details

When the pool token stops being used in the position, the _removePositionData function is called. However, it assumes that poolToken that is passed as a parameter always exists in user token array, which is not always the case. In the case of function FeeManager.paybackBadDebtNoReward(), which indirectly calls _removePositionData, insufficient validation doesn’t check if repay token is in user array, which results in zeroing out information about user debt.

## Impact

Free elimination of user debt.

## Recommended Mitigation Steps

Add verification if the token that is passed to _removePositionData() exists in user tokens. If not, revert the transaction.

## Assessed type

Invalid Validation vonMangoldt (Wise Lending) commented:

Double checking line of reasoning fails when user deposits large amount and then borrows.

Prepare big position that will have be destined to have positive badDebt. For sake of the argument, let’s assume it’s $1M worth of ETH.

Prepare very small position that will not be incentivized to be liquidated by liquidators, just to achieve non-zero badDebt. This can be done for example before significant price update transaction from Chainlink. Then take $1M worth of ETH flashloan and put this as collateral to position, borrowing as much as possible.

Call FeeManager.paybackBadDebtNoReward() on the position with desired position nftId, USDC token address and 0 shares as input params.

Because there is non-zero bad debt, the check will pass, and and the logic will finally reach MainHelper._corePayback(). Because repay is 0 shares, diminishing position size in USDC token will not underflow, and position token will be tried to be removed:

Comment:

Ok say big position has nftId = 1.

Ok say small position has nftId = 2.

nftId 2 now takes more collateral and borrows max: then calls paybackBadDebtNoReward with nftId 2.

But since collateral has been deposited and borrowed within non liquidation range (healthstate check active remember), This line here:

updatePositionCurrentBadDebt( _nftId ); in the beginning will set badDebtPosition[_nft] to 0 meaning it will exit after this line:

if (badDebtPosition[_nftId] == 0) { return 0; } and no harm done.

deliriusz (warden) commented:

@Trust - I have provided the

# [H-03] Incorrect bad debt accounting can lead to a state where the claimFeesBeneficial function is permanently bricked and no new incentives can be distributed, potentially locking pending and future protocol fees in the FeeManager contract

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

claimFeesBeneficial function is permanently bricked and no new incentives can be distributed, potentially locking pending and future protocol fees in the FeeManager contract Submitted by JCN, also found by serial-coder, 0xStalin, and Draiakoo Protocol fees can be collected from the WiseLending contract and sent to the FeeManager contract via the permissionless FeeManager::claimWiseFees function. During this call, incentives will only be distributed for incentive owners if totalBadDebtETH (global bad debt) is equal to 0:

FeeManager::claimWiseFees 663:

if ( totalBadDebtETH == 0 ) { // @audit: incentives only distributed if there is no global bad debt 664:

665:

tokenAmount = _distributeIncentives ( // @audit: distributes incentives for `incentive owners` via `gatheredIncentiveToken` mapping 666:

tokenAmount, 667:

_poolToken, 668:

underlyingTokenAddress 669: ); 670: } The fees sent to the FeeManager are then able to be claimed by beneficials via the FeeManager::claimFeesBeneficial function or by incentive owners via the FeeManager::claimIncentives function (if incentives have been distributed to the owners):

FeeManager::claimIncentives 284:

function claimIncentives ( 285:

address _feeToken 286: ) 287:

public 288: { 289:

uint256 amount = gatheredIncentiveToken [ msg.

sender ][ _feeToken ]; // @audit: mapping incremented in _distributeIncentives function 290:

291:

if ( amount == 0 ) { 292:

revert NoIncentive (); 293: } However, beneficials are only able to claim fees if there is currently no global bad debt in the system ( totalBadDebtETH == 0 ).

FeeManager::claimFeesBeneficial 689:

function claimFeesBeneficial ( 690:

address _feeToken, 691:

uint256 _amount 692: ) 693:

external 694: { 695:

address caller = msg.

sender; 696:

697:

if ( totalBadDebtETH > 0 ) { // @audit: can't claim fees when there is bad debt 698:

revert ExistingBadDebt (); 699: } Below I will explain how the bad debt accounting logic used during partial liquidations can result in a state where totalBadDebtETH is permanently greater than 0. When this occurs, beneficials will no longer be able to claim fees via the FeeManager::claimFeesBeneficial function and new incentives will no longer be distributed when fees are permissionlessly collected via the FeeManager::claimWiseFees function.

When a position is partially liquidated, the WiseSecurity::checkBadDebtLiquidation function is executed to check if the position has created bad debt, i.e. if the position’s overall borrow value is greater than the overall (unweighted) collateral value. If the post liquidation state of the position created bad debt, then the bad debt is recorded in a global and position-specific state:

WiseSecurity::checkBadDebtLiquidation 405:

function checkBadDebtLiquidation ( 406:

uint256 _nftId 407: ) 408:

external 409:

onlyWiseLending 410: { 411:

uint256 bareCollateral = overallETHCollateralsBare ( 412:

_nftId 413: ); 414:

415:

uint256 totalBorrow = overallETHBorrowBare ( 416:

_nftId 417: ); 418:

419:

if ( totalBorrow < bareCollateral ) { // @audit: LTV < 100% 420:

return; 421: } 422:

423:

unchecked { 424:

uint256 diff = totalBorrow 425: - bareCollateral; 426:

427:

FEE_MANAGER.

increaseTotalBadDebtLiquidation ( // @audit: global state, totalBadDebtETH += diff 428:

diff 429: ); 430:

431:

FEE_MANAGER.

setBadDebtUserLiquidation ( // @audit: position state, badDebtPosition[_nftId] = diff 432:

_nftId, 433:

diff 434: ); 435: } 436: } FeeManagerHeper.sol 77:

function _setBadDebtPosition ( 78:

uint256 _nftId, 79:

uint256 _amount 80: ) 81:

internal 82: { 83:

badDebtPosition [ _nftId ] = _amount; // @audit: position bad debt set 84: } 85:

86:

/** 87: * @dev Internal increase function for global bad debt amount.

88: */ 89:

function _increaseTotalBadDebt ( 90:

uint256 _amount 91: ) 92:

internal 93: { 94:

totalBadDebtETH += _amount; // @audit: total bad debt incremented As we can see above, the method by which the global and position’s state is updated is not consistent (total debt increases, but position’s debt is set to recent debt). Since liquidations can be partial, a position with bad debt can undergo multiple partial liquidations and each time the totalBadDebtETH will be incremented. However, the badDebtPosition for the position will only be updated with the most recent bad debt that was recorded during the last partial liquidation. Note that due to the condition on line 419 of WiseSecurity::checkBadDebtLiquidation, the badDebtPosition will be reset to 0 when totalBorrow == bareCollateral

( LTV == 100% ). However, in this case, any previously recorded bad debt for the position will not be deducted from the totalBadDebtETH. Lets consider two examples:

Scenario 1: Due to a market crash, a position’s LTV goes above 100%. The position gets partially liquidated, incrementing totalBadDebtETH by x (bad debt from 1st liquidation) and setting badDebtPosition[_nftId] to x. The position gets partially liquidated again, this time incrementing totalBadDebtETH by y (bad debt from 2nd liquidation) and setting badDebtPosition[_nftId] to y. The resulting state:

totalBadDebtETH == x + y badDebtPosition[_nftId] == y Scenario 2: Due to a market crash, a position’s LTV goes above 100%. The position gets partially liquidated, incrementing totalBadDebtETH by x and setting badDebtPosition[_nftId] to x. The position gets partially liquidated again, but this time the totalBorrow is equal to bareCollateral ( LTV == 100% ) and thus no bad debt is created. Due to the condition on line 419, totalBadDebtETH will be incremented by 0, but badDebtPosition[_nftId] will be reset to 0. The resulting state:

totalBadDebtETH == x badDebtPosition[_nftId] == 0 Note: Scenario 1 is more likely to occur since Scenario 2 requires the additional partial liquidation to result in an LTV of exactly 100% for the position.

As we can see, partial liquidations can lead to totalBadDebtETH being artificially inflated with respect to the actual bad debt created by a position.

When bad debt is created, it is able to be paid back via the FeeManager::paybackBadDebtForToken or FeeManager::paybackBadDebtNoReward functions. However, the maximum amount of bad debt that can be deducted during these calls is capped at the bad debt recorded for the position specified ( badDebtPosition[_nftId] ). Therefore, the excess “fake” bad debt can not be deducted from totalBadDebtETH, resulting in totalBadDebtETH being permanently greater than 0.

Below is the logic that deducts the bad debt created by a position when it is paid off via one of the payback functions mentioned above:

FeeManagerHelper::_updateUserBadDebt 170:

unchecked { 171:

uint256 newBadDebt = currentBorrowETH 172: - currentCollateralBareETH; 173:

174:

_setBadDebtPosition ( // @audit: badDebtPosition[_nftId] = newBadDebt 175:

_nftId, 176:

newBadDebt 177: ); 178:

179:

newBadDebt > currentBadDebt // @audit: totalBadDebtETH updated with respect to change in badDebtPosition 180: ?

_increaseTotalBadDebt ( newBadDebt - currentBadDebt ) 181::

_decreaseTotalBadDebt ( currentBadDebt - newBadDebt ); The above code is invoked in the FeeManagerHelper::updatePositionCurrentBadDebt function, which is in turn invoked during both of the payback functions previously mentioned. You will notice that the above code properly takes into account the change in the bad debt of the position in question. I.e. if the badDebtPosition[_nftId] decreased (after being paid back), then the totalBadDebtETH will decrease as well. Therefore, the totalBadDebtETH can only be deducted by at most the current bad debt of a position. Returning to the previous example in Scenario 1, this means that totalBadDebtETH would remain equal to x, since only y amount of bad debt can be paid back.

## Impact

In the event a position creates bad debt, partial liquidations of that position can lead to the global totalBadDebtETH state variable being artificially inflated. This additional “fake debt” can not be deducted from the global state when the actual bad debt of the position is paid back. Thus, the FeeManager::claimFeesBeneficial function will be permanently DOS-ed, preventing any beneficials from claiming fees in the FeeManager contract. Additionally, no new incentives are able to be distributed to incentive owners in this state. However, protocol fees can still be collected in this state via the permissionless FeeManager::claimWiseFees function, and since incentive owners and beneficials are the only entities able to claim these fees, this can lead to fees being permanently locked in the

FeeManager contract.

Justification for Medium Severity Although not directly affecting end users, the function of claiming beneficial fees and distributing new incentives will be permanently bricked. To make matters worse, anyone can continue to collect fees via the permissionless FeeManager::claimWiseFees function, which will essentially “burn” any pending or future fees by locking them in the FeeManager (assuming all previously gathered incentives have been claimed). This value is, therefore, leaked from the protocol every time additional fees are collected in this state.

Once this state is reached, any pending or future fees should ideally be left in the WiseLending contract, providing value back to the users instead of allowing that value to be unnecessarily “burned”. However, the permissionless nature of the FeeManager::claimWiseFees function allows bad actors to further grief the protocol during this state by continuing to collect fees.

Note: Once this state is reached, and WiseLending is made aware of the implications, all fees (for all pools) can be set to 0 by the master address. This would ensure that no future fees are sent to the FeeManager. However, this does not stop pending fees from being collected. Additionally, a true decentralized system (such as a DAO) would likely have some latency between proposing such a change (decreasing fee value) and executing that change. Therefore, any fees distributed during that period can be collected.

## Recommended Mitigation Steps

I would recommend updating totalBadDebtETH with the difference of the previous and new bad debt of a position in the WiseSecurity::checkBadDebtLiquidation function, similar to how it is done in the FeeManagerHelper::_updateUserBadDebt internal function.

Example implementation:

diff --git a/./WiseSecurity/WiseSecurity.sol b/./WiseSecurity/WiseSecurity.sol index d2cfb24..75a34e8 100644 --- a/./WiseSecurity/WiseSecurity.sol +++ b/./WiseSecurity/WiseSecurity.sol @@ -424,14 +424,22 @@ contract WiseSecurity is WiseSecurityHelper, ApprovalHelper { uint256 diff = totalBorrow - bareCollateral; - FEE_MANAGER.increaseTotalBadDebtLiquidation( - diff - ); + uint256 currentBadDebt = FEE_MANAGER.badDebtPosition(_nftId); FEE_MANAGER.setBadDebtUserLiquidation( _nftId, diff ); + + if (diff > currentBadDebt) { + FEE_MANAGER.increaseTotalBadDebtLiquidation( + diff - currentBadDebt + ); + } else { + FEE_MANAGER.decreaseTotalBadDebtLiquidation( + currentBadDebt - diff + ); + } } Trust (judge) increased severity to High

vonMangoldt (Wise Lending) commented via duplicate issue #243:

This doesn’t lead to loss of user funds though. Hence, it should be downgraded since one could just migrate and redeploy after discovering that. Otherwise good find.

Foon256 (Wise Lending) commented via duplicate issue #243:

Would agree with that! This is a good insight, but users’ funds are never at risk. This is related to the feeManager and the fees taken from the protocol. Therefore, a Medium issue.

Alex the Entreprenerd (Appellate Court judge) commented:

Summary of the issue When a market accrues bad debt, which can be inflated due to an accounting error, fees and incentives will no longer be distributed.

Note: The discussion had quite a bit of back and forth, for this reason the whole conversation is pasted below:

## Discussion

Alex the Entreprenerd (Appellate Court lead judge) commented:

This seems to be tied to a specific interpretation of this discussion we’ve had around loss of yield as high.

hickuphh3 (judge 2) commented:

Fees would be considered as matured yield? Given that it extends beyond the protocol to beneficials and incentive owners, I’m leaning towards a high more than a medium.

Alex the Entreprenerd (Appellate Court lead judge) commented:

Yes it would be considered matured. I don’t have an opinion on this report yet and will follow up later today with my notes.

Not fully made up my mind but here’s a couple of points:

For Medium: Loss of Yield -> There is no loss of principal so Med seems fine.

For High: The contract is not losing yield in some case, the contract is losing 100% of all yield. The contract is no longer serving it’s purpose.

External Conditions: Bad debt must be formed. Bad debt handling is part of the system design, so assuming this can happen is fair, and starting from a scenario in which this can happen is also fair.

That said, in reality, this may never happen.

My main point for downgrading is that while the contract is losing all of the yield, nothing beside that is impacted, not fully sure on this one.

LSDan (judge 3) commented:

I’m aligned with high on this one. Even though the conditions that lead to it are rare and there are arguably external conditions in some scenarios, there is a direct loss of funds and the functional loss of a contract’s purpose. Once this situation occurs, there is no clean way back from it.

Alex the Entreprenerd (Appellate Court lead judge) commented:

I think this is the issue where we will have some contention. I think the Sponsor interpretation is important to keep in mind as it’s pretty rational. I would like to think about it a bit more.

Alex the Entreprenerd (Appellate Court lead judge) commented:

I’m leaning towards Med on this report, I think the Sponsors POV is valid.

There is an accounting error, it would not cause permanent loss of funds. It would be mitigated by deprecating the market and creating a new one.

My main argument is that if this was live, this would trigger a re-deploy but it would not trigger any white hat rescue operation, as funds would be safe.

hickuphh3 (judge 2) commented:

#74: When we stick to the c4a rules to which we agreed, all the loss of fees are no user funds and therefore, should be treated differently.

The core argument for Medium severity is that fees are a secondary concern.

This goes against the supreme court decision where fees shouldn’t be treated as 2nd class citizens here.

Loss of fees should be regarded as an impact similar to any other loss of capital. Loss of real amounts depends on specific conditions and likelihood considerations.

Likelihood: Requirement of bad debt formation. Once there is, funds (fees) are permanently bricked.

There is an accounting error, it would not cause permanent loss of funds; it would be mitigated by deprecating the market and creating a new one.

The funds you are referring to are user funds? Separately, I don’t see how it would mitigate the bricking once it happens.

Alex the Entreprenerd (Appellate Court lead judge) commented:

I don’t think that the ruling means that loss of fees should be treated as high at all times.

The main argument is that the broken accounting doesn’t create a state that is not recoverable:

Some fees are lost.

User deprecates market (raises interests or pauses).

Deployes new Market.

System resumes functioning as intended.

My main argument is that this would not cause a War Room, it would cause a deprecation that the system can handle.

hickuphh3 (judge 2) commented:

In what cases/scenarios would loss of fees be high then? Most, if not all, won’t have a war room for protocol fees.

The reason I would consider to justify downgrading is the low likelihood of the external requirement of bad debt formation + >= 2 partial liquidations.

I would dissent and argue for high severity.

Permanent loss of unclaimed fees.

Blast radius: affects not just the protocol, but incentive owners and beneficiaries.

Had the fees gone only to the protocol, I’d lean a bit more towards Medium.

Is WiseLending immutable in a poolToken instance?

What contracts would have to be re-deployed?

Alex the Entreprenerd (Appellate Court lead judge) commented:

Liquidation premium being denied could be a valid High loss of yield, loss of gas for refunds when the system entire goal is that (e.g. keepers, voting on Nouns).

Alex the Entreprenerd’s (Appellate Court lead judge) Input The finding shows how in the specific case of liquidations with bad debt, a market will stop accounting for fees.

2 aggravating circumstances seem to be:

Inability to pause and replace each market.

The Math for bad debt is also wrong, leading to the inability to fix the bug.

This would still cause a loss of fees for a certain period of time, as the admin would eventually be able to set the market fees to either a state that would cause users to stop using it or 0 as a means to stop the loss.

I think that the accounting mistake is notable, and I understand the reasoning for raising severity.

That said, because we have to judge by impact of the finding, I believe Medium Severity to be most appropriate.

hickuphh3’s (judge 2) Input I maintain my stance for High severity for the reasons I stated above:

Permanent loss of unclaimed fees.

Impact on protocol ecosystem: beneficiaries and incentive owners.

LSDan’s (judge 3) Input I’m still of the opinion that High is most appropriate here. The impact is significant enough that raising the severity beyond medium makes sense.

Deliberation The severity is kept at High Severity, with a non-unanimous verdict.

Additional Context by the Lead Judge I recommend monitoring how this decision influences future decisions on severities, especially when it comes to a percentage loss of yield, an attacker having the button to cause a loss of yield, against this instance which is the permanent inability for the contract to record a gain of yield.

Wise Lending commented:

Mitigated here.

# [H-04] Liquidators can pay less than required to completely liquidate the private collateral balance of an uncollateralized position

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by nonseodion When a user deposits in the WiseLending contract he can make a private deposit (pure) which allows his deposits not to be used as collateral or a normal deposit. He can also set his position to be collateralized or uncollateralized. If a position is collateralized, the normal deposit can be used as collateral and vice-versa.

When a user uncollateralizes his position, he can only use his private deposit as collateral. If the position becomes liquidatable, it means the private deposit can no longer cover the amount borrowed. In the call to getFullCollateralETH() below only the private collateral is returned immediately as full collateral if it is uncollateralized.

WiseSecurityHelper.sol#L198-L208 ethCollateral = _getTokensInEth ( _poolToken, WISE_LENDING.

getPureCollateralAmount ( _nftId, _poolToken ) ); ❌ if ( _isUncollateralized ( _nftId, _poolToken ) == true ) { return ethCollateral; } In a liquidation, the amount to be liquidated is expressed as a percentage of the full collateral. In an uncollateralized position, the full collateral is the private collateral. The calculateWishPercentage() call calculates this percentage.

WiseSecurityHelper.sol#L760-L786 function calculateWishPercentage ( uint256 _nftId, address _receiveToken, uint256 _paybackETH, uint256 _maxFeeETH, uint256 _baseRewardLiquidation ) external view returns ( uint256 ) { uint256 feeETH = _checkMaxFee ( _paybackETH, _baseRewardLiquidation, _maxFeeETH ); uint256 numerator = ( feeETH + _paybackETH ) * PRECISION_FACTOR_E18; uint256 denominator = getFullCollateralETH ( _nftId, _receiveToken ); return numerator / denominator + 1; } The amount to be liquidated, i.e. the amount the liquidator receives, is calculated in _calculateReceiveAmount() using the percentage from calculateWishPercentage() and applied to the position’s pure collateral first in line 557 below.

It calculates the percentage of the user’s normal balance to be reduced in line 569 without checking if it is uncollateralized. If the amount it gets, i.e.

potentialPureExtraCashout, is greater than zero and less than the current private balance ( pureCollateral ) in line 576, it is reduced from the private balance.

WiseCore.sol#L564-L586 556:

if ( pureCollateralAmount [ _nftId ][ _receiveTokens ] > 0 ) { 557:

receiveAmount = _withdrawPureCollateralLiquidation ( 558:

_nftId, 559:

_receiveTokens, 560:

_removePercentage 561: ); 562: } 563:

564:

uint256 potentialPureExtraCashout; 565:

uint256 userShares = userLendingData [ _nftId ][ _receiveTokens ].

shares; 566:

uint256 pureCollateral = pureCollateralAmount [ _nftId ][ _receiveTokens ]; 567:

568:

if ( pureCollateral > 0 && userShares > 0 ) { 569:

potentialPureExtraCashout = _calculatePotentialPureExtraCashout ( 570:

userShares, 571:

_receiveTokens, 572:

_removePercentage 573: ); 574: } 575:

576:

if ( potentialPureExtraCashout > 0 && potentialPureExtraCashout <= pureCollateral ) { 577:

_decreasePositionMappingValue ( 578:

pureCollateralAmount, 579:

_nftId, 580:

_receiveTokens, 581:

potentialPureExtraCashout 582: ); 583:

584:

_decreaseTotalBareToken ( 585:

_receiveTokens, 586:

potentialPureExtraCashout 587: ); 588:

589:

return receiveAmount + potentialPureExtraCashout; 590: } 591:

The issue is the implementation applies the percentage meant for only the private collateral to both the normal and private collateral. It should reduce only the private collateral, but may also reduce the public collateral and send it to the liquidator.

Here’s how a malicious liquidator can profit and steal user funds:

User deposits $100 worth of WETH in his private balance and $100 worth of WETH in his normal balance.

He uncollateralizes his position and borrows $70 worth of WBTC.

If the price of WBTC he borrowed goes up to $100, he can be liquidated.

Assuming no liquidation fees, the liquidator pays $50 WBTC to liquidate $50 WETH (50%) from the user’s private balance leaving $50.

The 50% is applied to the user’s public balance giving $50. This is also deducted from the private balance leaving $0 in the private balance.

The liquidator ends up paying only $50 to earn $50 extra.

A liquidator can set it up to drain the private collateral balance and only pay for a portion of the liquidation. The user ends up losing funds and the protocol’s bad debt increases.

## Impact

This vulnerability allows the liquidator to steal the user’s balance and pay for only a portion of the shares. It has these effects:

The user loses funds.

The amount of bad debt in the protocol is increased.

## Recommended Mitigation Steps

To ensure the code does not also consider the normal balance at all we can check if the position is uncollateralized early. Currently, this check is done but is done too late in the _calculateReceiveAmount() function. We can fix it by moving the check.

WiseCore.sol#L560-L594 + if ( userLendingData [ _nftId ][ _receiveTokens ].

unCollateralized == true ) { + return receiveAmount; + } + uint256 potentialPureExtraCashout; uint256 userShares = userLendingData [ _nftId ][ _receiveTokens ].

shares; uint256 pureCollateral = pureCollateralAmount [ _nftId ][ _receiveTokens ];...

- if ( userLendingData [ _nftId ][ _receiveTokens ].

unCollateralized == true ) { - return receiveAmount; - } - return _withdrawOrAllocateSharesLiquidation ( _nftId, _nftIdLiquidator,

## Assessed type

Invalid Validation vm06007 (Wise Lending) commented:

Edge case - if user is about to be liquidated, I think they will make things collateralized to avoid liquidation. Either way we would like to see this as Medium. Fix is already applied. This is also something that’s been explored in the hats.finance competition; hence, 564-566 lines came from there etc.

Trust (judge) commented:

# [H-05] Wrong use of nftID to check if a PowerFarm position is an Aave position

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

nftID to check if a PowerFarm position is an Aave position Submitted by nonseodion, also found by 0xStalin When a PowerFarm position is created its keyId is used as a key in the isAave mapping to indicate if it is an Aave position or not. The keyId is the index of the Power Farm NFT linked with the position.

PendlePowerManager.sol#L129-L130 isAave [ keyId ] = _isAave; The keyId is also linked with another nftId. This other nftId is used to hold the keyId s Power Farm position in the WiseLending contract. They are linked together in the farmingKeys mapping of the MinterReserver contract.

MinterReserver.sol#L88C1-L91C46 uint256 keyId = _getNextReserveKey (); reservedKeys [ _userAddress ] = keyId; farmingKeys [ keyId ] = _wiseLendingNFT; The issue is the check if a position is an Aave position is done using the WiseLending nftId instead of the Power Farm’s keyId. This occurs five times in the code:

It is used in getLiveDebtRatio() to know the pool token borrowed so the borrowShares can be retrieved.

PendlePowerFarm.sol#L64-L70 uint256 borrowShares = isAave [ _nftId ] ?

_getPositionBorrowSharesAave ( _nftId ):

_getPositionBorrowShares ( _nftId ); It is used in _manuallyPaybackShares() to know the pool token to pay back.

PendlePowerFarm.sol#L127-L129 if ( isAave [ _nftId ] == true ) { poolAddress = AAVE_WETH_ADDRESS; } It is used in checkDebtRatio() to know the pool token borrowed so the borrowShares can be retrieved.

PendlePowerFarmMathLogic.sol#L396-L402 ❌ uint256 borrowShares = isAave [ _nftId ] ?

_getPositionBorrowSharesAave ( _nftId ):

_getPositionBorrowShares ( _nftId ); It is used in _coreLiquidation() to select a token to payback and to know the pool token borrowed so the borrowShares can be retrieved.

PendlePowerFarmLeverageLogic.sol#L575-L590 ❌ address paybackToken = isAave [ _nftId ] == true ?

AAVE_WETH_ADDRESS:

WETH_ADDRESS; paybackAmount = WISE_LENDING.

paybackAmount ( paybackToken, _shareAmountToPay ); ❌ uint256 cutoffShares = isAave [ _nftId ] == true ?

_getPositionBorrowSharesAave ( _nftId ) * FIVTY_PERCENT / PRECISION_FACTOR_E18:

_getPositionBorrowShares ( _nftId ) * FIVTY_PERCENT / PRECISION_FACTOR_E18; These have the following effects:

For getLiveDebtRatio(), users would get zero when they try to retrieve their debt ratio.

For _manuallyPaybackShares() users won’t be able to pay back their shares manually from the PowerFarm contract since it’ll fetch zero shares as borrow shares.

The last two instances are used in liquidation and allow a malicious user to have a position that can’t be liquidated even though it is eligible for liquidation. The malicious user can:

Create an Aave PowerFarm position.

The position becomes eligible for liquidation after some price changes.

Liquidators cannot liquidate the position because the call to _coreLiquidation first calls checkDebtRatio() which uses the wrong borrowShares to calculate the debt ratio and returns true. Thus, causing a revert.

PendlePowerFarmLeverageLogic.sol#L571C1-L574C1 if ( _checkDebtRatio ( _nftId ) == true ) { revert DebtRatioTooLow (); }

## Impact

Malicious users can open positions that can’t get liquidated.

Users can’t pay back manually when it is an Aave position.

getLiveDebtRatio() returns zero always when it is an Aave position.

## Recommended Mitigation Steps

Consider checking if a position is an Aave position using the keyId of the position.

PendlePowerFarm.sol#L64-L70 - uint256 borrowShares = isAave [ _nftId ] + uint256 borrowShares = isAave [ keyId ] ?

_getPositionBorrowSharesAave ( _nftId ):

_getPositionBorrowShares ( _nftId ); PendlePowerFarm.sol#L127-L129 - if ( isAave [ _nftId ] == true ) { + if ( isAave [ keyId ] == true ) { poolAddress = AAVE_WETH_ADDRESS; } PendlePowerFarmMathLogic.sol#L396-L402 - uint256 borrowShares = isAave [ _nftId ] + uint256 borrowShares = isAave [ keyId ] ?

_getPositionBorrowSharesAave ( _nftId ):

_getPositionBorrowShares ( _nftId ); PendlePowerFarmLeverageLogic.sol#L575-L590 - address paybackToken = isAave [ _nftId ] == true + address paybackToken = isAave [ keyId ] == true ?

AAVE_WETH_ADDRESS:

WETH_ADDRESS; paybackAmount = WISE_LENDING.

paybackAmount ( paybackToken, _shareAmountToPay ); - uint256 cutoffShares = isAave [ _nftId ] == true + uint256 cutoffShares = isAave [ keyId ] == true ?

_getPositionBorrowSharesAave ( _nftId ) * FIVTY_PERCENT / PRECISION_FACTOR_E18:

_getPositionBorrowShares ( _nftId ) * FIVTY_PERCENT / PRECISION_FACTOR_E18; vm06007 (Wise Lending) commented:

While this is marked as High, we would like to bring this to a Medium, as there is still a way to liquidate such nft/user should that even occur (depending if there’s AavePool present etc).

@vonMangoldt can provide details on how this can be mitigated and user can still be liquidated even if they would have a chance to enter with the wrong isAave flag.

Wise Lending commented:

Mitigated here.

Medium Risk Findings (17)

# [M-01] Exiting a farm on mainnet assumes a peg of 1:1 when swapping stETH for ETH

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

1:1 when swapping stETH for ETH Submitted by 0xStalin When stETH depegs from ETH, the swaps on Curve will revert due to requesting a higher amountOut than what the curves pool will give.

## Recommended Mitigation Steps

The recommendation would be to implement a mitigation similar to the one implemented on the referenced issues. Basically, fetch the current price of stETH from a Chainlink Oracle and multiply the minOutAmount by the current price of stETH. In this way, the minOutAmount that is sent to the Curves exchange will now be within the correct limits based on the current price of stETH.

Also, make sure to multiply the ethValueBefore by the current price of stETH (only when exiting farms on mainnet). In this way, both amounts, ethValueAfter and ethValueBefore will be computed based on the current price of stETH, allowing the slippage to validate that no ethValue was lost during the process of removing liquidity from pendle, redeeming the sy tokens and swapping on curves. In the end, both, ethValueAfter and ethValueBefore will represent the ethValue based on the stETH price.

## Assessed type

Context vonMangoldt (Wise Lending) commented:

Since we have ethValueBefore and after we could also set it to 0 and no harm done, thus invalid.

Trust (judge) commented:

@vonMangoldt - Would need additional description for the issue described is not actually effective, as from my analysis I can’t find a counter-example.

vm06007 (Wise Lending) commented:

Mitigated here.

Additionally, the team decided to use PendleRouter ( here ) instead of Curve moving forward during farm exit.

# [M-02] Chainlink Oracles may return stale prices or may be unusable when aggregator roundId is less than 50

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

roundId is less than 50 Submitted by nonseodion WiseLending calibrates Oracles to get a heartbeat which it uses for checking the staleness of prices returned from the Oracle.

To calibrate, it fetches between 3-50 inclusive historical prices and picks the second largest update time among those prices. It calls _getIterationCount() to know the number of historical prices it’ll use. If the current _latestAggregatorRoundId is less than 50 ( MAX_ROUND_COUNT ) it uses _latestAggregatorRoundId else it uses 50.

OracleHelper.sol#L665-L667 res = _latestAggregatorRoundId < MAX_ROUND_COUNT ? _latestAggregatorRoundId: MAX_ROUND_COUNT; The issue with the snippet above is that _latestAggregatorRoundId will always be greater than 50 so the number of historical prices it uses will always be 50.

It’s always greater than 50 because it is fetched from the aggregator’s proxy contract. The roundId s returned from the proxy are a combination of the current aggregator’s roundId and phaseId. Check Chainlink docs for more info.

getLatestRoundId() returns the roundId.

OracleHelper.sol#L708-L715 ( roundId, ) = priceFeed [ _tokenAddress ].

latestRoundData (); The roundId returned is used in the _recalibratePreview() function below to get previous roundIds. The iterationCount as we already mentioned will always be 50.

OracleHelper.sol#L620C1-L630C15 uint80 i = 1; uint256 currentDiff; uint256 currentBiggest; uint256 currentSecondBiggest; ❌ while (i < iterationCount) { uint256 currentTimestamp = _getRoundTimestamp( _tokenAddress, ❌ latestRoundId - i ); The problem with the above call is that the argument, latestRoundId-1 above may not have valid data for some rounds. So calls to the Chainlink Oracle for those rounds will revert.

This may occur because of the way proxy roundIds work.

Example:

If the proxy returns 0x40000000000000010 as roundId.

The phaseId is 4 ( roundId >> 64 ).

The aggregator roundId is 16 (uint64( roundId )).

After 16 iterations in _recalibratePreview(), the latestRoundId will have a value of 0x40000000000000000. When the price feed is called, with this roundId, it will revert because it does not exist.

Check Chainlink docs for more info.

Thus, if the aggregator roundId derived from the proxy roundId is less than 50, _recalibratePreview() will revert. The caller will have to wait until it is greater than 50.

## Impact

For new price feeds, OracleHub won’t be able to set a heartbeat until the aggregator roundId is greater than 50. So the new price feed would be unusable for that period.

For price feeds that already have a heartbeat, they won’t be able to recalibrate during the period the aggregator roundId is less than 50. Which may allow the price feed return stale prices.

In both cases above the max amount of time user can wait for is 50x the official Chainlink heartbeat for the price feed, i.e.

price feed heartbeat * 50.

For the BTC/ETH price feed this would be 50 days ( 24 hours * 50 ).

## Recommended Mitigation Steps

Consider deriving the aggregator roundId from the proxy roundId and using that instead of the proxy roundId.

OracleHelper.sol#L665-L667 - res = _latestAggregatorRoundId < MAX_ROUND_COUNT - ?

_latestAggregatorRoundId + res = uint64 ( _latestAggregatorRoundId ) < MAX_ROUND_COUNT + ?

uint64 ( _latestAggregatorRoundId ):

MAX_ROUND_COUNT; vonMangoldt (Wise Lending) commented:

I don’t understand the recommended mitigation steps. Its just typecasting uint64 to a uint80.

Trust (judge) commented:

Referring to this, it is clear the roundId needs to be trimmed to uint64.

0xStalin (warden) commented:

@Trust - I was checking and came to the realization that the Chainlink Documentation only explains how the proxy queries the getRoundData on the Aggregator.

However, all this logic is implemented on the Proxy itself, the AggregatorProxy.getRoundData() is on charge of trimming the roundId down to uint64 before querying the aggregator, but the proxy itself receives the roundId as an uint80. Thus, the WiseOracle contract is correctly integrated with the Chainlink contract. All the required logic to query the data from the aggregator is contained within the chainlink proxy, any contract interacting with the proxy doesn’t need to trim the received roundId.

Based on the Chainlink contracts and the way how WiseOracle queries the getRoundData(), I think there is not any issue at all. The integration looks perfectly fine, and there is no need to implement any changes. If the roundId were to be trimmed to uint64, it would disrupt the queries and it would be querying a total different values; therefore, this report looks to be invalid.

Trust (judge) commented:

I believe the root cause of the issue is correct. It is not safe to use latestRoundId - i in calculations, as the latestRoundId is crafted of a phaseID and a uint64 counter. By decreasing by a flat amount, we could find ourselves querying for an incorrect phaseID. Essentially, it is a logical overflow not detected because it occurs in internal data of a larger data type (uint80).

0xStalin (warden) commented:

I see, then the root cause seems to be correct, but is the recommendation incomplete? If so, what would it be the correct way to mitigate this issue?

At this point, I’m not arguing the validity of the report, but rather want to understand what needs to be done to fully mitigate the root cause of this issue.

Wise Lending commented:

Mitigated here.

# [M-03] First depositor inflation attack in PendlePowerFarmToken

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

PendlePowerFarmToken Submitted by SBSecurity, also found by 0xCiphky In certain scenarios, shares of a subsequent depositor can be heavily reduced, losing a large amount of his deposited funds, the attacker can increase the right side of the previewMintShares by adding rewards for compounding.

That way victim can lose 6e17 of his assets for a deposit of 1e18.

## Recommended Mitigation Steps

Since there will be many PowerFarmTokens deployed, there is no way for team to perform the first deposit for all of them. Possible mitigation will be to have minimum deposit amount for the first depositor in the PendlePowerToken, which will increase the cost of the attack exponentially. There will not be enough reward tokens making the exchangeRewardsForCompoundingWithIncentive revert, due to insufficient amount. Or, could mint proper amount of tokens in the initialize function.

vm06007 (Wise Lending) commented:

Since there will be many PowerFarmTokens deployed, there is no way for team to perform the first deposit for all of them.

I think this assumption is wrong here, team deploys each token and farm by admin - one by one, and each time a new token/farm is created, team can perform first deposit or necessary step. It is not a public function to create these that team cannot handle something like that or to say “there is no way to perform the first deposit for all of them”. That’s just blown off and far fetched.

vm06007 (Wise Lending) commented:

Mitigated here.

Additional notes: before any farm is publicly available, admin creating farms can ensure no further supplier to the farm would experience any loss due to described far-fetched scenario in this “finding”.

# [M-04] Withdrawing uncollateralized deposits is possible even though the position is in liquidation mode

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by 0xStalin Users can withdraw uncollateralized deposits even though their position is liquidable, as opposed to the README. If the position is in liquidation mode, users should use their uncollateralized deposits to avoid liquidation instead of removing them.

## Recommended Mitigation Steps

If the protocol wants to enforce that users use their uncollateralized deposits to avoid liquidations when the positions are liquidable, don’t set to true the powerFarmCheck when doing withdrawals for uncollateralized deposits. Allow the WiseLending._healthStateCheck() function to validate if the position is indeed in liquidation mode by using the weightedCollateral instead of the bareCollateral value.

WiseSecurity.sol:

function checksWithdraw(..

)..

{...

- if (_isUncollateralized(_nftId, _poolToken) == true) { - return true; - }...

}

## Assessed type

Context vonMangoldt (Wise Lending) commented:

That’s no issue since the additional check is reflecting a higher %. We will keep it like that.

Wise Lending commented:

Mitigated here.

# [M-05] The protocol allows borrowing small positions that can create bad debt

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by serial-coder, also found by serial-coder and SBSecurity The WiseLending protocol allows users to borrow small positions. Even if the protocol has a minimum deposit (collateral) amount check to mitigate the small borrowing position from creating bad debt, this protection can be bypassed.

With a small borrowing position, there is no incentive for a liquidator to liquidate the position, as the liquidation profit may not cover the liquidation cost (gas). As a result, small liquidable positions will not be liquidated, leaving bad debt to the protocol.

## Recommended Mitigation Steps

Implement the minimum borrowing amount check to limit the minimum size of borrowing positions.

vonMangoldt (Wise Lending) commented via duplicate issue #277:

Can also be circumvented by just paying back after borrowing. Doesn’t really add any value, in my opinion.

Trust (judge) commented:

The bug class is valid, in my honest opinion; as either liquidator will liquidate at a loss or protocol will be losing money to protect from bad debt over time.

Wise Lending commented:

Mitigated here.

# [M-06] Off-by-one bug prevents the _compareMinMax() from detecting Chainlink aggregators’ circuit-breaking events

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

_compareMinMax() from detecting Chainlink aggregators’ circuit-breaking events Submitted by serial-coder The WiseLending protocol implements the OracleHelper::_compareMinMax() to detect the circuit-breaking events of Chainlink aggregators when an asset price goes outside of pre-determined min/max values. For instance, in case of a significant price drop (e.g., LUNA crash), the asset price reported by the Chainlink price feed will continue to be at the pre-determined minAnswer instead of the actual price.

The _compareMinMax() ’s objective is to prevent such a crash event that would allow a user to borrow other assets with the wrongly reported asset price. For more, refer to the case of Venus Protocol and Blizz Finance in the crash of LUNA.

However, the current implementation of the _compareMinMax() got an off-by-one bug that would prevent the function from detecting the mentioned Chainlink aggregators’ circuit-breaking events. In other words, the function will not revert the transaction if the flash crash event occurs as expected.

## Recommended Mitigation Steps

Add the cases _answer == minAnswer and _answer == maxAnswer like the snippet below:

function _compareMinMax( IAggregator _tokenAggregator, int192 _answer ) internal view { int192 maxAnswer = _tokenAggregator.maxAnswer(); int192 minAnswer = _tokenAggregator.minAnswer(); - if (_answer > maxAnswer || _answer < minAnswer) { + if (_answer >= maxAnswer || _answer <= minAnswer) { revert OracleIsDead(); }

## Assessed type

Oracle Trust (judge) decreased severity to Low serial-coder (warden) commented:

@Trust - This is a valid medium issue as the _compareMinMax() was implemented incorrectly.

Specifically, the _compareMinMax() does not include the edge cases _answer == minAnswer and _answer == maxAnswer. So, the function cannot detect the minAnswer or maxAnswer as expected.

To elaborate, for example, the minAnswer the aggregator can report for the LINK (one of the tokens in scope) is 100000000000000 ( 10 ** 14 ). Suppose, in the event of a flash crash, the price of the LINK token drops significantly below the minAnswer (below 10 ** 14 ).

However, the price feed will continue to report the pre-determined minAnswer (not the actual price). Since the _compareMinMax() does not include the case _answer == minAnswer, it cannot detect this flash crash event.

In other words, the least reported price (i.e., _answer ) will be the pre-determined minAnswer, not the actual price. Thus, the _compareMinMax() will never enter the if case and revert the transaction as expected because the _answer will never be less than the aggregator’s minAnswer.

function _compareMinMax ( IAggregator _tokenAggregator, int192 _answer ) internal view { int192 maxAnswer = _tokenAggregator.

maxAnswer (); int192 minAnswer = _tokenAggregator.

minAnswer (); @> if ( _answer > maxAnswer || _answer < minAnswer ) { //@audit -- The least reported price (i.e., `_answer`) will be the `minAnswer`. So, the function will never enter the " if " case revert OracleIsDead (); } An example reference is here. As you can see, their recommendation includes the edge cases _answer == minAnswer and _answer == maxAnswer.

Note: to view the provided image, please see the original comment here.

Further on the TWAP Oracle:

Someone may argue that the protocol has the TWAP.

I want to point out that the TWAP setup for each price feed is only optional. If the TWAP is not set, the price deviation comparison mechanism between the TWAP’s price and Chainlink’s price will not be triggered. For this reason, this issue deserves a Medium severity.

Trust (judge) increased severity to Medium and commented:

The impact demonstrated is an incorrect validation check, which in the case maxAnswer/minAnswer are used by the feed, will make detecting black swans fail. As the team assumes those protections are in place, Medium severity is appropriate.

00xSEV (warden) commented:

@Trust - Could you double-check that it’s a valid issue? If we follow the link from the Chainlink docs and check the code here and here:

* @ param _minAnswer lowest answer the median of a report is allowed to be * @ param _maxAnswer highest answer the median of a report is allowed to be require ( minAnswer <= median && median <= maxAnswer, "median is out of min-max range" ); It means that if median == minAnswer or median == maxAnswer it still a valid value and we don’t have to revert.

serial-coder (warden) commented:

@00xSEV - The least reported price will be the pre-determined minAnswer. If you do not include the case ==, the _compareMinMax() will never enter the if case and revert the transaction. You cannot detect the black swan events without it.

The reported price will never be less than the minAnswer. (Without the ==, the if condition will always be false ).

function _compareMinMax ( IAggregator _tokenAggregator, int192 _answer ) internal view { int192 maxAnswer = _tokenAggregator.

maxAnswer (); int192 minAnswer = _tokenAggregator.

minAnswer (); @> if ( _answer > maxAnswer || _answer < minAnswer ) { //@audit -- The least reported price (i.e., `_answer`) will be the `minAnswer`. So, the function will never enter the " if " case revert OracleIsDead (); } Wise Lending commented:

Mitigated here.

# [M-07] Unchecked return value bug on TransferHelper::_safeTransferFrom()

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

TransferHelper::_safeTransferFrom() Submitted by serial-coder, also found by 0x11singh99, josephdara, Jorgect, unix515, Mrxstrange, nonseodion, and Rhaydden Currently, the WiseLending protocol supports several ERC-20 tokens and will also support more tokens in the future:

ERC20 in scope: WETH, WBTC, LINK, DAI, WstETH, sDAI, USDC, USDT, WISE and may others in the future. (Also corresponding Aave tokens if existing).

On some ERC-20 tokens, the transferFrom() will return false on failure instead of reverting a transaction. I noticed that the TransferHelper::_safeTransferFrom(), which is used throughout the protocol, is vulnerable to detecting the token transfer failure if the transferFrom() returns false due to the unchecked return value bug.

The following lists the functions directly invoking the vulnerable _safeTransferFrom():

WiseCore::_coreLiquidation() [`WiseLending::depositExactAmount()]`(

- https://github.com/code-423n4/2024-02-wise-lending/blob/79186b243d8553e66358c05497e5ccfd9488b5e2/contracts/WiseLending.sol#L476-L481
) WiseLending::solelyDeposit() WiseLending::paybackExactAmount() WiseLending::paybackExactShares() FeeManager::paybackBadDebtForToken() FeeManager::paybackBadDebtNoReward() PendlePowerFarm::_manuallyPaybackShares() PendlePowerManager::enterFarm() PendlePowerFarmController::exchangeRewardsForCompoundingWithIncentive() PendlePowerFarmController::exchangeLpFeesForPendleWithIncentive() PendlePowerFarmController::lockPendle() PendlePowerFarmToken::addCompoundRewards() PendlePowerFarmToken::depositExactAmount() AaveHub::depositExactAmount() AaveHub::paybackExactAmount() AaveHub::paybackExactShares() Due to the unchecked return value bug, users or attackers can exploit these protocol’s functions without supplying a token (please refer to the

## Impact

section for a complete list of the functions directly invoking the vulnerable _safeTransferFrom().

The following briefly analyzes 2 of the 17 functions that would affect the exploitation as examples:

Due to the unchecked return value bug, the WiseCore::_coreLiquidation() cannot be aware of the token transfer failure. Thus, a rogue liquidator can steal collateral from the target liquidable position without supplying any debt token ( tokenToPayback ).

On the WiseLending::depositExactAmount(), a rogue depositor can increase their collateral without spending any token.

// FILE: https://github.com/code-423n4/2024-02-wise-lending/blob/79186b243d8553e66358c05497e5ccfd9488b5e2/contracts/WiseCore.sol function _coreLiquidation ( CoreLiquidationStruct memory _data ) internal returns ( uint256 receiveAmount ) {...

@ 5 _safeTransferFrom ( //@audit -- Liquidator can steal collateral (_receiveToken) from the target liquidable position @ 5 _data.

tokenToPayback, @ 5 _data.

caller, @ 5 address ( this ), @ 5 _data.

paybackAmount @ 5 );...

}...

// FILE: https://github.com/code-423n4/2024-02-wise-lending/blob/79186b243d8553e66358c05497e5ccfd9488b5e2/contracts/WiseLending.sol function depositExactAmount ( uint256 _nftId, address _poolToken, uint256 _amount ) public syncPool ( _poolToken ) returns ( uint256 ) { uint256 shareAmount = _handleDeposit ( msg.

sender, _nftId, _poolToken, _amount ); @ 6 _safeTransferFrom ( //@audit -- Depositor can increase their collateral without supplying any token @ 6 _poolToken, @ 6 msg.

sender, @ 6 address ( this ), @ 6 _amount @ 6 ); return shareAmount; } @5 -- Liquidator can steal collateral (_receiveToken) from the target liquidable position: see here.

@6 -- Depositor can increase their collateral without supplying any token: see here.

## Recommended Mitigation Steps

Improve the _safeTransferFrom() by checking the return value from the _callOptionalReturn() and reverting a transaction if it is false.

function _safeTransferFrom( address _token, address _from, address _to, uint256 _value ) internal { - _callOptionalReturn( + bool success = _callOptionalReturn( _token, abi.encodeWithSelector( IERC20.transferFrom.selector, _from, _to, _value ) ); + require(success, "Token transfer failed"); }

## Assessed type

Invalid Validation vm06007 (Wise Lending) commented:

This then leads to some tokens unusable (like USDT, for example) and this topic was already discussed severely during hats competition where I can send links to findings, so should be scraped in my opinion. Also sufficient checks are already done in _callOptionalReturn directly.

vm06007 (Wise Lending) commented:

Mitigated here.

Additionally, WiseLending is not meant to be used with corrupted tokens that have unsupported transfer/transferFrom implementations.

# [M-08] Borrowers can DoS liquidations by repaying as little as 1 share.

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by 0xStalin, also found by Dup1337 and Jorgect Liquidations can be DoSed which increments the risk of bad debt being generated on position.

## Recommended Mitigation Steps

In the WiseSecurityHelper.checkMaxShares() function, if the _shareAmountToPay exceeds maxShares, don’t revert; re-adjust the number of shares that can be liquidated. Return the final value of _shareAmountToPay and forward it back to the WiseLending.liquidatePartiallyFromTokens() function. Then, use the final value of shareAmountToPay to compute the exact amount of tokens to be repaid in the WiseLending.liquidatePartiallyFromTokens() function.

WiseSecurity.sol:

function checksLiquidation(...

uint256 _shareAmountToPay ) external view + returns (uint256) {...

- checkMaxShares( + return checkMaxShares( _nftIdLiquidate, _tokenToPayback, borrowETHTotal, unweightedCollateralETH, _shareAmountToPay ); } WiseSecurityHelper.sol:

function checkMaxShares(...

uint256 _shareAmountToPay ) public view + returns (uint256) {...

uint256 maxShares = checkBadDebtThreshold(_borrowETHTotal, _unweightedCollateralETH) ? totalSharesUser: totalSharesUser * MAX_LIQUIDATION_50 / PRECISION_FACTOR_E18; if (_shareAmountToPay <= maxShares) { - return; + return _shareAmountToPay; + } else { + return maxShares; + } - revert TooManyShares(); } WiseLending.sol:

function liquidatePartiallyFromTokens(...

uint256 _shareAmountToPay )...

{...

- data.shareAmountToPay = _shareAmountToPay;...

//@audit-info => First, determine the maximum amount of liquidable shares + data.shareAmountToPay = WISE_SECURITY.checksLiquidation( + _nftId, + _paybackToken, + _shareAmountToPay + ); //@audit-info => Then, compute the exact amount of tokens required to liquidate the final amount of liquidable shares data.paybackAmount = paybackAmount( _paybackToken, - _shareAmountToPay + data.shareAmountToPay );...

- WISE_SECURITY.checksLiquidation( - _nftId, - _paybackToken, - _shareAmountToPay - );...

}

## Assessed type

Context vonMangoldt (Wise Lending) commented:

Instead of wasting gas by frontrunning a newbie liquidator they could also make money and liquidate themselves. On Arbitrum it’s not possible at all. If liquidators don’t use tools like flashbots or eden, it’s their own fault. No reason to do anything here, in my opinion. Dismissed.

Trust (judge) commented:

Whether the option for liquidators to bypass hacks through private mempools is grounds for reducing severity of their abuse is not a trivial question and would best be standardized in the Supreme Court.

I’m taking a stance that this cheap hack is annoying enough for liquidators to be worth Medium severity. Note that if private mempool is in-scope, attacker can use it as well and insert the annoying TX at start of block.

vm06007 (Wise Lending) commented:

Mitigated here.

Mentioned issue does not seem to be in interest of the borrower, since borrower is more incentivized to perform self-liquidation to earn than preventing others, which becomes a race on who will liquidate first making liquidation a more desired outcome for both parties, no intention to gate the minimum payback threshold by admin.

# [M-09] Liquidating chaining can be achieved by liquidating token collateral with the highest collateralFactor

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

collateralFactor Submitted by Draiakoo The liquidation mechanism is intended as follows:

If a user has more borrowed value than weighted collateral, but it does not surpass the 89% of the unweighted collateral, he can be liquidated up to 50% of his borrowed shares.

When the borrowed amount surpass the 89% of the unweighted collateral, then it is considered bad debt and the position can be fully liquidated This feature is programmed here However, since the liquidator can select which collateral he will receive, he can intentionally liquidate the highest collateralFactor tokens in order to make the overall position’s collateralFactor to go down and being able to keep liquidating the other tokens. Since the intended maximum amount to liquidate when the position has no bad debt is 50%, if a user can intentionally create a sequence of liquidation that leads to a greater percentage it can be considered a high impact vulnerability.

## Recommended Mitigation Steps

This issue can be easily solved by forcing all liquidations to be done with the lowest collateralFactor tokens first. As shown in the written
## Assessed type

Error vonMangoldt (Wise Lending) commented:

This is not an issue since the liquidation incentive usually is lower than the difference in percentage between 100 and collateral factor. So paying back in my described scenario always makes the position more healthy. High as a description is overblown! Also, such a force could lead liquidators to be forced to loose money on a specific scenario before being able to access a profitable liquidation endangering the protocol.

Trust (judge) decreased severity to Medium and commented:

Cascading liquidations are a dangerous situation and I agree with the warden there are no built-in safety mechanisms around it in the liquidation routines (health is monotonically increasing or it is now healthy).

However, the warden had to modify the collateral factors to demonstrate the issue in a PoC. It seems hard to determine whether by natural course of action, such a scenario would occur.

According to the sponsor’s remarks, the liquidation incentive *usually* is lower than the difference in percentage between 100 and collateral factor.

This has not convinced me it could not occur by chance at some point. In case it does, it leads to higher than expected liquidation penalties. Weighing all the circumstances, I believe Medium to be appropriate.

Wise Lending commented:

Mitigated here.

# [M-10] Lack of update when modifying pool fee

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by 0xCiphky The FeeManager contract allows the master address to modify the pool fee. This can be done to a single pool using the setPoolFee function or multiple pools at once using the setPoolFeeBulk function. This fee is used in the the syncPool modifier, specifically the _updatePseudoTotalAmounts function which updates the interest amounts of the borrow and lending pools.

function setPoolFee ( address _poolToken, uint256 _newFee ) external onlyMaster { _checkValue ( _newFee ); WISE_LENDING.

setPoolFee ( _poolToken, _newFee ); emit PoolFeeChanged ( _poolToken, _newFee, block.

timestamp ); } The issue is that the setPoolFee function modifies the pool fee without invoking the syncPool modifier beforehand. Consequently, the next sync operation incorrectly applies the updated pool fee to the period between the previous call and the change in the pool fee. Although the impact of changing the fee for a single pool may be minimal, using the setPoolFeeBulk function to alter fees for multiple pools could have a bigger impact.

## Impact

Depending on whether the pool fee is increased or decreased, the protocol or its users may end up paying additional fees or receiving reduced fees.

Likelihood: Low. This situation arises solely in instances where there is a change in the pool fee.

Recommendation Add the following code to update fees accurately before implementing changes:

function setPoolFee ( address _poolToken, uint256 _newFee ) external onlyMaster { WISE_LENDING.

syncManually ( _poolToken ); //add here _checkValue ( _newFee ); WISE_LENDING.

setPoolFee ( _poolToken, _newFee ); emit PoolFeeChanged ( _poolToken, _newFee, block.

timestamp ); }

## Assessed type

Context vm06007 (Wise Lending) commented:

Admin can also call this manually ( syncManually ) directly on the contract after changing fee.

Trust (judge) commented:

@vm06007 - This is true; but for it to reduce severity, we would need to see indication that likelihood of this happening (ergo, that the issue is known) is high.

Foon256 (Wise Lending) commented:

@Trust - But this is clearly a centralization issue and is OOS. Or what do I miss here?

Trust (judge) commented:

Not every flaw in a privileged function can be viewed as a centralization issue. Warden demonstrated a plausible way where an HONEST admin interaction leads to incorrect fee allocation, which I consider to be in scope.

vm06007 (Wise Lending) commented:

@Trust - It seems “honest” is something caps-locked here so we better unfold this:

If admin decides not to call this, there is no issue.

If admin decides to call this and knows what to do then, there is no issue.

If admin decides to call this and does not know what to do only then it is an issue, so it falls under a category where it does not matter the intention of the admin (honest or dishonest can’t really be a thing here). It is more of a question will admin make a mistake when calling that function without follow up, or admin does not make a mistake and makes correct calls on sync as well.

Also it is in admins interest to make correct calls as expected. If admin makes a mistake then it is same topic of “admin error” no matter the intention.

Trust (judge) commented:

Post-Judging QA is over, so unfortunately, we cannot consider any more arguments (in this submission or any other). I don’t see how we can be confident admin knows to follow up this call correctly. If they are not aware of the issue in the report, that’s the meaning of an honest mistake in my mind.

Wise Lending commented:

Mitigated here.

# [M-11] PendlePowerManager is incompatible with PendleRouterV3

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

PendlePowerManager is incompatible with PendleRouterV3 Submitted by NentoR

- https://github.com/pendle-finance/pendle-core-v2-public/blob/main/contracts/router/ActionAddRemoveLiqV3.sol#L166-L172
- https://github.com/pendle-finance/pendle-core-v2-public/blob/main/contracts/router/ActionAddRemoveLiqV3.sol#L444-L449
- https://github.com/code-423n4/2024-02-wise-lending/blob/main/contracts/PowerFarms/PendlePowerFarm/PendlePowerFarmLeverageLogic.sol#L467-L482
- https://github.com/code-423n4/2024-02-wise-lending/blob/main/contracts/PowerFarms/PendlePowerFarm/PendlePowerFarmLeverageLogic.sol#L165-L171

## Impact

Will cause revert when attempting to open a farm position.

## Recommended Mitigation Steps

Support only the latest router (V3) or add conditional checks to use the respective selector for each router version.

Trust (judge) commented:

By definition a codebase can never be guaranteed to be compatible with the latest version. Requesting warden to provide evidence lack of integration with V3 achieves M+ severity.

NentoR (warden) commented:

@Trust - The reason why I reported this was because the sponsor told me they expect compatibility with both. The problem is that an older version of pendle router is used here and the calls for addLiquiditySingleSy() and removeLiquiditySingleSy() are basically incompatible with the new one and will revert because of a missing parameter. To understand the rationale behind this submission, let’s examine the deployed contracts:

Current router version (taken from test files).

PendleRouterV3 (latest).

Here’s the code for both on Deth.net:

Old router and New one.

The functions can be found in ActionAddRemoveLiq and ActionAddRemoveLiqV3 Old one:

/// @dev swaps SY to PT, then adds liquidity function _addLiquiditySingleSy ( address receiver, address market, IPYieldToken YT, uint256 netSyIn, uint256 minLpOut, ApproxParams calldata guessPtReceivedFromSy ) internal returns ( uint256 netLpOut, uint256 netSyFee ) { MarketState memory state = IPMarket ( market ).

readState ( address ( this )); //...

} New one:

function addLiquiditySingleToken ( address receiver, address market, uint256 minLpOut, ApproxParams calldata guessPtReceivedFromSy, TokenInput calldata input, LimitOrderData calldata limit ) external payable returns ( uint256 netLpOut, uint256 netSyFee, uint256 netSyInterm ) { ( IStandardizedYield SY,, IPYieldToken YT ) = IPMarket ( market ).

readTokens (); netSyInterm = _mintSyFromToken ( _entry_addLiquiditySingleSy ( market, limit ), address ( SY ), 1, input ); ( netLpOut, netSyFee ) = _addLiquiditySingleSy ( receiver, market, SY, YT, netSyInterm, minLpOut, guessPtReceivedFromSy, limit ); //...

} function _addLiquiditySingleSy ( address receiver, address market, IStandardizedYield SY, IPYieldToken YT, uint256 netSyIn, uint256 minLpOut, ApproxParams calldata guessPtReceivedFromSy, LimitOrderData calldata limit ) internal returns ( uint256 netLpOut, uint256 netSyFee ) { uint256 netSyLeft = netSyIn; uint256 netPtReceived; if (!

_isEmptyLimit ( limit )) { ( netSyLeft, netPtReceived, netSyFee, ) = _fillLimit ( market, SY, netSyLeft, limit ); _transferOut ( address ( SY ), market, netSyLeft ); } ( uint256 netPtOutMarket,, ) = _readMarket ( market ).

approxSwapSyToAddLiquidity ( YT.

newIndex (), netSyLeft, netPtReceived, block.

timestamp, guessPtReceivedFromSy ); //...

} _readMarket() comes from ActionBase, here’s it’s definition:

function _readMarket ( address market ) internal view returns ( MarketState memory ) { return IPMarket ( market ).

readState ( address ( this )); } You can see that both call readState() from the underlying Pendle market. Here you can find all market deployments:

- https://docs.pendle.finance/Developers/Deployments/Arbitrum#markets
I’ll use the first one for the example. Arbiscan:

- https://arbiscan.io/address/0x7D49E5Adc0EAAD9C027857767638613253eF125f
Deth.net:

- https://arbiscan.deth.net/address/0x7D49E5Adc0EAAD9C027857767638613253eF125f
If you look at the definion of readState(), you’ll see the following:

function readState ( address router ) public view returns ( MarketState memory market ) { market.

totalPt = _storage.

totalPt; market.

totalSy = _storage.

totalSy; market.

totalLp = totalSupply ().

Int (); ( market.

treasury, market.

lnFeeRateRoot, market.

reserveFeePercent ) = IPMarketFactory ( factory ).

getMarketConfig ( router ); market.

scalarRoot = scalarRoot; market.

expiry = expiry; market.

lastLnImpliedRate = _storage.

lastLnImpliedRate; } readState() on all markets reaches out to the factory contract it was deployed with to grab the market configuration. And there are two versions:

MarketFactory and MarketFactoryV3. Again, both can be found in the docs but here are the links:

MarketFactory and MarketFactory V3.

You can take a market from the docs and query Market::isValidMarket(). Using the first one, the old factory will return true whereas the new one, false.

So basically what this all means is that the protocol won’t be able to use new markets. Pendle can start phasing out the old ones and migrating them over. The fix is rather easy on the protocol’s end, they just need to account for the newly added parameter and can support both, if they wish, using a conditional check.

Trust (judge) commented:

@vonMangoldt - can you confirm the warden’s claims around your intentions?

Trust (judge) commented:

Without sponsor’s take the warden’s claim that V3 should be compatible with the design is accepted.

Wise Lending commented:

Mitigated here.

# [M-12] PendlePowerFarmToken:: totalLpAssetsToDistribute may lead to temporary DOS due to price growth check being skipped during deposit

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

PendlePowerFarmToken:: totalLpAssetsToDistribute may lead to temporary DOS due to price growth check being skipped during deposit Submitted by NentoR, also found by NentoR and 00xSEV

- https://github.com/code-423n4/2024-02-wise-lending/blob/main/contracts/PowerFarms/PendlePowerFarmController/PendlePowerFarmToken.sol#L98-L130
- https://github.com/code-423n4/2024-02-wise-lending/blob/main/contracts/PowerFarms/PendlePowerFarmController/PendlePowerFarmToken.sol#L502-L524
- https://github.com/code-423n4/2024-02-wise-lending/blob/main/contracts/PowerFarms/PendlePowerFarmController/PendlePowerFarmToken.sol#L89-L95

## Impact

Random actors, malicious or not, can DOS the Pendle PowerFarm vault by sending rewards to it through PendlePowerFarmToken::addCompoundRewards() or PendlePowerFarmController::exchangeRewardsForCompoundingWithIncentive(). This results in all users being unable to access their positions or open new ones for a set amount of time imposed by PendlePowerFarmToken::_validateSharePriceGrowth().

## Recommended Mitigation Steps

I recommend turning the deposited rewards into shares at the time of calling addCompoundRewards(), so we can get _validateSharePriceGrowth() to validate it immediately and revert on the spot if needed. This will prevent the DOS and share price growth manipulation.

## Assessed type

DoS Trust (judge) increased severity to High and commented:

High is reasonable for temporary freeze of funds.

vm06007 (Wise Lending) commented:

High is reasonable for temporary freeze of funds.

That is literally definition of a Medium - temporary freeze of the funds. High is permanent freeze of funds. So dismiss the high.

vm06007 (Wise Lending) commented:

- https://docs.code4rena.com/awarding/judging-criteria/severity-categorization
2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

3 — High: Assets can be stolen/lost/compromised directly (or indirectly if there is a valid attack path that does not have hand-wavy hypotheticals.

Trust (judge) commented:

This is on the upper end of temporary FoF attacks, it also lasts for an extended duration. I would rule High on much weaker versions of this finding, not even close to downgrade-worthy.

Alex the Entreprenerd (Appellate Court judge) commented:

Summary of the issue Due to a security check on LP value growth, a donation can cause the temporary inability to withdraw from a Farm Contract

## Discussion

Alex the Entreprenerd (Appellate Court lead judge) commented:

Facts: The Dos has a cost, that grows with supply. The Dos can be performed permissionlessly, at any time.

I’m unclear as to whether this is tied to liquidation risk, which should raise the severity.

hickuphh3 (judge 2) commented:

From what I see, it shouldn’t affect liquidations or protocol health, only causing forced hodl with guaranteed returns. Hence, leaning towards Medium more than High.

Alex the Entreprenerd (Appellate Court lead judge) commented:

Facts: Information contained in the report mentions exclusively an inability to withdraw (funds stuck). Other facts above still apply.

I agree that Medium Severity seems the most appropriate because:

More supply = higher cost.

Dos is temporary.

Unclear/Missing usage to DOS a key feature.

LSDan (judge 3) commented:

I’m also of the opinion that Medium is more appropriate here.

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

Assets are not at direct risk. The minute the DOS stops the assets are once again available. Further, the cost to DOS the platform is not trivial. As you both pointed out, the more assets being frozen, the higher the cost. The attacker is donating to the users they are DOSing, leaving the user inconvenienced, but richer at the attacker’s expense.

Deliberation The severity is downgraded to Medium unanimously.

Additional Context by the Lead Judge Had the Report shown a way to weaponize this to halt liquidations, we would have had a easier time pushing for a higher severity.

However, given the Report only limiting itself to a temporary inability to withdraw, with no loss of principal, the decision was straightforward.

Trust (judge) commented:

I remain convinced that a permissionless, non-theoretical temporary FoF for non-negligible length (certainly days/weeks as shown) meets High severity threshold.

Impact breaks a core invariant which states users can always pull out their funds. Users making monetary decisions under that assumption can certainly face loss of principal (e.g. cannot pull out to repay a due-loan, etc).

Wise Lending commented:

Mitigated here.

Note, this finding was downgraded to Medium by C4 staff in reference to the Appellate Court decision.

# [M-13] Incorrect calculation of lending shares in _withdrawOrAllocateSharesLiquidation can lead to revert and failure to liquidate

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

_withdrawOrAllocateSharesLiquidation can lead to revert and failure to liquidate Submitted by WoolCentaur, also found by SBSecurity and AM When liquidating a user, _withdrawOrAllocateSharesLiquidation() can be called which checks if a pool is large enough to pay out the liquidator, and if not, then the liquidator is allocated shares that can be used to withdraw at a later time when the pool is large enough. There are two scenarios in which the pool will not be large enough to pay out the liquidator:

The obvious scenario where the pool simply doesn’t contain enough tokens to cover the withdraw amount.

So many tokens are borrowed out of the pool that there isn’t enough available to pay out the liquidator.

Note: If there are some tokens available but not enough to cover the entire withdraw amount, then those tokens are transferred, dropping the total pool to 0, and the rest are allocated as shares.

However, if these scenarios were ever to arise, the _withdrawOrAllocateSharesLiquidation() function would not work as expected. This is due to how it calculates the lending shares ( totalPoolInShares ) of the total pool:

- https://github.com/code-423n4/2024-02-wise-lending/blob/79186b243d8553e66358c05497e5ccfd9488b5e2/contracts/WiseCore.sol#L493
When liquidating, it will calculate the totalPoolInShares like this (with amount being the total pool):

lendingShares = (totalDepositShares * amount) / pseudoTotalPool - 1; However, calculating the lending shares this way will cause the _compareSharePrices() to revert on the syncPool modifier. Specifically the check on _validateParameter( _lendSharePriceBefore, lendSharePriceAfter ); This is because _maxSharePrice is passed in as false to calculateLendingShares(), which subtracts one.

uint256 totalPoolInShares = calculateLendingShares( { _poolToken: _poolToken, _amount: totalPoolToken, _maxSharePrice: false } );

- https://github.com/code-423n4/2024-02-wise-lending/blob/79186b243d8553e66358c05497e5ccfd9488b5e2/contracts/WiseCore.sol#L497
However, because we are withdrawing, it should be set to true so that it adds one. In all other areas where calculateLendingShares() is being used, the functions that focus on withdrawing have _maxSharePrice: true and the depositing functions have _maxSharePrice: false:

withdrawOnBehalfExactAmount on WiseLending.sol.

_preparationsWithdraw on MainHelper.sol.

_handleDeposit on WiseCore.sol.

When _maxSharePrice is set to true, the _validateParameter() check will pass.

Even if the _validateParameter() check did not exist, if _maxSharePrice is left as-is (set to false), any subsequent liquidations through _withdrawOrAllocateSharesLiquidation() would revert with panic due to an underflow. This is because (as stated above), liquidating when the pool is not large enough to pay out the liquidator will drop the total pool to 0 and issue the remainder as shares. Therefore, any subsequent calls to liquidate when the total pool is 0 will underflow:

//amount is totalPoolToken which in this scenario = 0; shares = amount * totalDepositShares / pseudoTotalPool - 1

## Impact

When a particular receiving token is desired and _withdrawOrAllocateSharesLiquidation() is called, the liquidation will always revert if the total pool is not large enough to cover the withdraw amount. This defeats the purpose of _withdrawOrAllocateSharesLiquidation(), as stated in the NatSpec:

/** * @dev Internal math function for liquidation logic * which checks if pool has enough token to pay out * liquidator. If not, liquidator get corresponding * shares for later withdraw.

*/ This will lead to frustrated users who desire a particular receiving token. The level of frustration will be even higher if the reason this function reverts is because the total pool is borrowed out. This would lead to a very high APY, thus a much higher desire to receive the particular token. Additionally, the borrowers in these particular situations will not be liquidated even though they should be. Ultimately, this will lead to loss of faith in the protocol.

However, this can be easily avoided by just passing in a different receiving token that is large enough to payout the withdraw amount, which is why this is a medium level issue.

## Recommended Mitigation Steps

Change the _maxSharePrice under calculateLendingShares on _withdrawOrAllocateSharesLiquidation() to true.

## Assessed type

Error vm06007 (Wise Lending) commented via duplicate issue #238:

That seems to be like a desired functionality by design and expected behavior. @vonMangoldt can confirm.

vonMangoldt (Wise Lending) commented via duplicate issue #238:

This looks right from my first looking into it. Just curious why it didn’t DOS in our javascript tests. Probably the percentage roundings (etc.) need to be aligned for that behaviour.

Trust (judge) commented:

Selected as best because of good POC + well balanced severity rationalization.

Wise Lending commented:

Mitigated here.

# [M-14] Current heartbeat implementation may lead to a prolonged DoS for Chainlink Oracles

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

Submitted by 00xSEV Currently, if there are 50 fast updates followed by no updates, a Chainlink Oracle will be considered dead, even though it’s normal behavior. Chainlink Oracles update either after some time has passed or upon a price change.

## Vulnerability Details

How it will revert:

There is a _chainLinkIsDead function that returns true if the last update took longer than the heartbeat. See here.

unchecked { upd = block.

timestamp < upd ?

block.

timestamp:

block.

timestamp - upd; return upd > heartBeat [ _tokenAddress ]; } It’s essentially called on every request to the Chainlink Oracle, before the actual call, to ensure the price is up to date.

How does recalibrate work?

heartBeat is updated when recalibrate / recalibrateBulk is called. Anyone can call them. Both of these functions call _recalibrate. See here.

heartBeat [ _tokenAddress ] = _recalibratePreview ( _tokenAddress ); In _recalibratePreview, we see that currentSecondBiggest is returned, representing the second-largest difference between updates. Thus, heartBeat is set to the second-largest time difference between two consecutive updates in the last 50 updates.

iterationCount is capped by MAX_ROUND_COUNT in _getIterationCount, which is set to 50.

How do Chainlink updates work ?

Aggregators receive updates from the Oracle network only when the Deviation Threshold or Heartbeat Threshold triggers an update during an aggregation round. The first condition that is met triggers an update to the data.

Deviation Threshold: A new aggregation round starts when a node identifies that the off-chain values deviate by more than the defined deviation threshold from the onchain value. Individual nodes monitor one or more data providers for each feed.

Heartbeat Threshold: A new aggregation round starts after a specified amount of time from the last update.

If you check “Show more details” here, you can see that for most feeds, deviation is set to 1-2% and heartbeat is 86400s = 24 hours. However, some feeds are set to 0.5% or even less.

If there’s a period of high volatility followed by no volatility, it’s possible that heartBeat in Wise will be set to a low value. Consequently, the Chainlink feed will be considered dead after a short period of no updates.

E.g., if there are 50 updates, once every minute, followed by 10 hours of no updates, and then no updates for an additional 24 hours, the heartBeat will be set to 1 minute in Wise. Consequently, the Oracle will be considered dead after 1 minute of no updates. This means it will be considered dead for the initial 10 hours, then considered alive for 1 minute, and then considered dead again for the following 24 hours.

Examples demonstrating similar events in the wild can be seen in this Dune dashboard.

## Impact

The Chainlink Oracle is considered dead for a substantial amount of time, affecting liquidations, deposits, withdrawals, and all other functions using this Oracle.

The attacker can disable the entire market that uses the Oracle by calling recalibrate. This can lead to bad debt (the price changes rapidly, but the Oracle still reverts after the first update), griefing (users cannot use the protocol), etc.

It can be even worse if combined with block stuffing or when the gas price is too high and Chainlink does not update. The updates stop coming as often as usual, and the feed is considered dead long enough to accrue bad debt. For example, if the last 50 updates occurred every minute, a sudden spike in demand for block space could make updates come only once an hour, preventing liquidations for 1-2 hours.

## Recommended Mitigation Steps

Consider setting Chainlink’s native heartbeat instead. Also consider adding access control to recalibrate functions and only calling it when it will not lead to DoS.

## Assessed type

Oracle vonMangoldt (Wise Lending) commented:

It takes second highest out of last 50 rounds if you recalibrate. If it takes forever to update for chainlink it means there is no volatility. So dismissed.

Trust (judge) commented:

Warden discussed a potential scenario when the Oracle would be considered dead after just one minute of inactivity.

vm06007 (Wise Lending) commented:

@Trust - it depends on the expected heartbeat, if it’s one minute, and latest data does not come within that frame then Oracle SHOULD be considered dead.

Example:

recalibrate() looks for second longest time gap between latest 50 (or 500 depending on chain) rounds, by analyzing timegaps between reported prices in last 50/500 rounds contract chooses appropriate expected timeframe when Oracle needs to answer before considered dead. If the time frame is too short this is only because that what was picked up from latest round data and should be honored (unlike this finding).

Note that it can be recalibrated to increase the expected time if needed.

Wise Lending commented:

Mitigated here.

# [M-15] Precision loss in the calculation of the fee amounts and fee shares inside the _preparePool function of the MainHelper contract

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

_preparePool function of the MainHelper contract Submitted by Matin Low fee amounts and fee shares are calculated in the preparation part due to the precision loss.

## Recommended Mitigation Steps

Consider modifying the fee shares calculation to prevent such precision loss and prioritize the multiplication over division:

uint256 amountInterest = bareIncrease / PRECISION_FACTOR_YEAR; uint256 feeAmount = bareIncrease * globalPoolData [ _poolToken ].

poolFee / ( PRECISION_FACTOR_E18 * PRECISION_FACTOR_YEAR );

## Assessed type

Math Wise Lending commented:

Mitigated here.

# [M-16] A user can lose more value than he specifies in the spread when he enters a PowerFarm

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

PowerFarm Submitted by nonseodion When a user enters or exits a PowerFarm, he specifies an allowed spread. The spread specifies the minimum value of the position allowed at the end of the transaction.

E.g. a spread of 105% on a position with a value of $1000 ensures the value does not fall below $950.

1000 * (200-105) / 100 = 950 When a user opens a position on Arbitrum, the ENTRY_ASSET is converted to WETH on UniswapV3. The second argument in line 434 specifies the minimum value for the swap. The spread is applied to this argument. Hence, the _depositAmount in line 432 cannot go less than what the spread allows.

PendlePowerFarmLeverageLogic.sol#L423-L440 426:

uint256 reverseAllowedSpread = 2 427: * PRECISION_FACTOR_E18 428: - _allowedSpread; 429:

430:

if ( block.

chainid == ARB_CHAIN_ID ) { 431:

432:

_depositAmount = _getTokensUniV3 ( 433:

_depositAmount, 434:

_getEthInTokens ( 435:

ENTRY_ASSET, 436:

_depositAmount 437: ) 438: * reverseAllowedSpread 439: / PRECISION_FACTOR_E18, 440:

WETH_ADDRESS, 441:

ENTRY_ASSET 442: ); 443: } The value at the end of the transaction is checked in line 508 below to ensure it does not go below the allowed spread. The ethValueBefore is calculated from _depositAmount in line 489. Note that if the swap on Uniswap occurred the _depositAmount may already be the minimum value. The ethValueAfter is scaled with the allowed spread in line 501.

PendlePowerFarmLeverageLogic.sol#L485-L505 489:

uint256 ethValueBefore = _getTokensInETH ( 490:

ENTRY_ASSET, 491:

_depositAmount 492: ); 493:

494: ( 495:

uint256 receivedShares 496:, 497: ) = IPendleChild ( PENDLE_CHILD ).

depositExactAmount ( 498:

netLpOut 499: ); 500:

// @audit-issue the calculation for ethValueAfter below is incorrect 501:

uint256 ethValueAfter = _getTokensInETH ( 502:

PENDLE_CHILD, 503:

receivedShares 504: ) 505: * _allowedSpread 506: / PRECISION_FACTOR_E18; 507:

// @audit-issue ethValueBefore on Arbitrum uses the depositAmount that allowedSpread has already been applied 508:

if ( ethValueAfter < ethValueBefore ) { 509:

revert TooMuchValueLost (); 510: } Thus the comparison in line 508 may compare ethValueAfter with the minimum value of the spread instead of the value of the initial deposit. With this implementation, if the spread for a $1000 transaction is 105%, the minimum value after the transaction becomes $902.5.

(1000 * 95 / 100) * (95/100) = 902.5. The user can lose $47.5 ( 950-902.5 ).

Note: In the implementation, the value at the end is scaled up instead of the value at the beginning being scaled down like the examples show. Hence, the actual minimum value is lesser i.e.

~$904.76 ( 950/1.05 ). This is a different bug.

## Recommended Mitigation Steps

Consider storing the actual deposit and using it to calculate ethValueBefore.

PendlePowerFarmLeverageLogic.sol#L423-L488 uint256 reverseAllowedSpread = 2 * PRECISION_FACTOR_E18 - _allowedSpread; + unit256 actualDeposit = _depositAmount; if ( block.

chainid == ARB_CHAIN_ID ) { _depositAmount = _getTokensUniV3 ( _depositAmount, _getEthInTokens ( ENTRY_ASSET, _depositAmount ) * reverseAllowedSpread / PRECISION_FACTOR_E18, WETH_ADDRESS, ENTRY_ASSET ); }...

uint256 ethValueBefore = _getTokensInETH ( ENTRY_ASSET, - _depositAmount +.

actualDeposit );

## Assessed type

Uniswap Wise Lending commented:

Mitigated here.

# [M-17] User’s attempt to deposit & withdraw reverts due to the calculation style inside _calculateShares()

- **Contest:** Wise Lending 
- **Slug:** 2024-02-wise-lending
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-wise-lending
- **Source snapshot:** competitions/2024-02-wise-lending/final_report.html

_calculateShares() Submitted by t0x1c, also found by DanielArmstrong Scenario 1:

The following flow of events (one among many) causes a revert:

Alice calls depositExactAmountETH() to deposit 1 ether. This executes successfully, as expected.

Bob calls depositExactAmountETH() to deposit 1.5 ether (or 0.5 ether or 1 ether or 2 ether or any other value). This reverts unexpectedly.

In case Bob was attempting to make this deposit to rescue his soon-to-become or already bad debt and to avoid liquidation, this revert will delay his attempt which could well be enough for him to be liquidated by any liquidator, causing loss of funds for Bob. Here’s a concrete example with numbers:

Bob calls depositExactAmountETH() to deposit 1 ether. This executes successfully, as expected.

Bob calls borrowExactAmountETH() to borrow 0.7 ether. This executes successfully, as expected.

Bob can see that price is soon going to spiral downward and cause a bad debt. He plans to deposit some additional collateral to safeguard himself. He calls depositExactAmountETH() again to deposit 0.5 ether. This reverts unexpectedly.

Prices go down and he is liquidated.

Scenario 2:

A similar revert occurs when the following flow of events occur:

Alice calls depositExactAmountETH() to deposit 10 ether. This executes successfully, as expected.

Bob calls withdrawExactAmountETH() to withdraw 10 ether (or 10 ether - 1 or 10 ether - 1000 or 9.5 ether or 9.1 ether ). This reverts unexpectedly.

Bob is not able to withdraw his entire deposit. If he leaves behind 1 ether and withdraws only 9 ether, then he does not face a revert.

In both of the above cases, eventually the revert is caused by the validation failure on L234-L237 due to the check inside _validateParameter():

File:

contracts / WiseLending.

sol 210:

function _compareSharePrices ( 211:

address _poolToken, 212:

uint256 _lendSharePriceBefore, 213:

uint256 _borrowSharePriceBefore 214: ) 215:

private 216:

view 217: { 218: ( 219:

uint256 lendSharePriceAfter, 220:

uint256 borrowSharePriceAfter 221: ) = _getSharePrice ( 222:

_poolToken 223: ); 224:

225:

uint256 currentSharePriceMax = _getCurrentSharePriceMax ( 226:

_poolToken 227: ); 228:

229:

_validateParameter ( 230:

_lendSharePriceBefore, 231:

lendSharePriceAfter 232: ); 233:

234: @---> _validateParameter ( 235: @---> lendSharePriceAfter, 236: @---> currentSharePriceMax 237: ); 238:

239:

_validateParameter ( 240:

_borrowSharePriceBefore, 241:

currentSharePriceMax 242: ); 243:

244:

_validateParameter ( 245:

borrowSharePriceAfter, 246:

_borrowSharePriceBefore 247: ); 248: } Root Cause _compareSharePrices() is called by _syncPoolAfterCodeExecution() which is executed due to the syncPool modifier attached to depositExactAmountETH().

Before _syncPoolAfterCodeExecution() in the above step is executed, the following internal calls are made by depositExactAmountETH():

The _handleDeposit() function is called on L407 which in-turn calls calculateLendingShares() on L115.

The calculateLendingShares() function now calls _calculateShares() on L26.

_calculateShares() decreases the calculated shares by 1 which is represented by the variable lendingPoolData[_poolToken].totalDepositShares inside _getSharePrice().

The _getSharePrice() functions uses this lendingPoolData[_poolToken].totalDepositShares variable in the denominator on L185-187 and in many cases, returns an increased value (in this case it evaluates to 1000000000000000001 ) which is captured in the variable lendSharePriceAfter inside _compareSharePrices().

Circling back to our first step, this causes the validation to fail on L234-L237 inside _compareSharePrices() since the lendSharePriceAfter is now greater than currentSharePriceMax i.e.

1000000000000000001 > 1000000000000000000. Hence, the transaction reverts.

The reduction by 1 inside _calculateShares() is done by the protocol in its own favour to safeguard itself. The lendingPoolData[_poolToken].pseudoTotalPool; however, is never modified. This mismatch eventually reaches a significant divergence, and is the root cause of these reverts.

See:

The last comment inside the

## Recommended Mitigation Steps

section later in the report.

File:

contracts / WiseLending.

sol 97:

modifier syncPool ( 98:

address _poolToken 99: ) { 100: ( 101:

uint256 lendSharePrice, 102:

uint256 borrowSharePrice 103: ) = _syncPoolBeforeCodeExecution ( 104:

_poolToken 105: ); 106:

107:

_; 108:

109: @---> _syncPoolAfterCodeExecution ( 110:

_poolToken, 111:

lendSharePrice, 112:

borrowSharePrice 113: ); 114: } File:

contracts / WiseLending.

sol 308:

function _syncPoolAfterCodeExecution ( 309:

address _poolToken, 310:

uint256 _lendSharePriceBefore, 311:

uint256 _borrowSharePriceBefore 312: ) 313:

private 314: { 315:

_newBorrowRate ( 316:

_poolToken 317: ); 318:

319: @---> _compareSharePrices ( 320:

_poolToken, 321:

_lendSharePriceBefore, 322:

_borrowSharePriceBefore 323: ); 324: } File:

contracts / WiseLending.

sol 388:

function depositExactAmountETH ( 389:

uint256 _nftId 390: ) 391:

external 392:

payable 393:

syncPool ( WETH_ADDRESS ) 394:

returns ( uint256 ) 395: { 396: @---> return _depositExactAmountETH ( 397:

_nftId 398: ); 399: } 400:

401:

function _depositExactAmountETH ( 402:

uint256 _nftId 403: ) 404:

private 405:

returns ( uint256 ) 406: { 407: @---> uint256 shareAmount = _handleDeposit ( 408:

msg.

sender, 409:

_nftId, 410:

WETH_ADDRESS, 411:

msg.

value 412: ); 413:

414:

_wrapETH ( 415:

msg.

value 416: ); 417:

418:

return shareAmount; 419: } File:

contracts / WiseCore.

sol 106:

function _handleDeposit ( 107:

address _caller, 108:

uint256 _nftId, 109:

address _poolToken, 110:

uint256 _amount 111: ) 112:

internal 113:

returns ( uint256 ) 114: { 115: @---> uint256 shareAmount = calculateLendingShares ( 116: { 117:

_poolToken:

_poolToken, 118:

_amount:

_amount, 119:

@---> _maxSharePrice:

false 120: } 121: ); 122:

File:

contracts / MainHelper.

sol 17:

function calculateLendingShares ( 18:

address _poolToken, 19:

uint256 _amount, 20:

bool _maxSharePrice 21: ) 22:

public 23:

view 24:

returns ( uint256 ) 25: { 26: @---> return _calculateShares ( 27:

lendingPoolData [ _poolToken ].

totalDepositShares * _amount, 28:

lendingPoolData [ _poolToken ].

pseudoTotalPool, 29:

_maxSharePrice 30: ); 31: } 32:

33:

function _calculateShares ( 34:

uint256 _product, 35:

uint256 _pseudo, 36:

bool _maxSharePrice 37: ) 38:

private 39:

pure 40:

returns ( uint256 ) 41: { 42:

return _maxSharePrice == true 43: ?

_product / _pseudo + 1 44: @--->:

_product / _pseudo - 1; 45: } File:

contracts / WiseLending.

sol 210:

function _compareSharePrices ( 211:

address _poolToken, 212:

uint256 _lendSharePriceBefore, 213:

uint256 _borrowSharePriceBefore 214: ) 215:

private 216:

view 217: { 218: ( 219: @---> uint256 lendSharePriceAfter, 220:

uint256 borrowSharePriceAfter 221: @---> ) = _getSharePrice ( 222:

_poolToken 223: ); 224:

225:

uint256 currentSharePriceMax = _getCurrentSharePriceMax ( 226:

_poolToken 227: ); 228:

229:

_validateParameter ( 230:

_lendSharePriceBefore, 231:

lendSharePriceAfter 232: ); 233:

234:

_validateParameter ( 235: @---> lendSharePriceAfter, 236:

currentSharePriceMax 237: ); 238:

239:

_validateParameter ( 240:

_borrowSharePriceBefore, 241:

currentSharePriceMax 242: ); 243:

244:

_validateParameter ( 245:

borrowSharePriceAfter, 246:

_borrowSharePriceBefore 247: ); 248: } File:

contracts / WiseLending.

sol 165:

function _getSharePrice ( 166:

address _poolToken 167: ) 168:

private 169:

view 170:

returns ( 171:

uint256, 172:

uint256 173: ) 174: { 175:

uint256 borrowSharePrice = borrowPoolData [ _poolToken ].

pseudoTotalBorrowAmount 176: * PRECISION_FACTOR_E18 177: / borrowPoolData [ _poolToken ].

totalBorrowShares; 178:

179:

_validateParameter ( 180:

MIN_BORROW_SHARE_PRICE, 181:

borrowSharePrice 182: ); 183:

184:

return ( 185:

lendingPoolData [ _poolToken ].

pseudoTotalPool 186: * PRECISION_FACTOR_E18 187: @---> / lendingPoolData [ _poolToken ].

totalDepositShares, 188:

borrowSharePrice 189: ); 190: }

## Recommended Mitigation Steps

Since the reduction by 1 inside _calculateShares() is being done to round-down in favour of the protocol, removing that without a deeper analysis could prove to be risky as it may open up other attack vectors. Still, two points come to mind which can be explored:

Option1:

Reducing lendingPoolData[_poolToken].pseudoTotalPool too would keep it in sync with lendingPoolData[_poolToken].totalDepositShares and will avoid the current issue.

Option2:

Not reducing it by 1 seems to solve the immediate problem at hand (needs further impact analysis):

File: contracts/MainHelper.sol 33: function _calculateShares( 34: uint256 _product, 35: uint256 _pseudo, 36: bool _maxSharePrice 37: ) 38: private 39: pure 40: returns (uint256) 41: { 42: return _maxSharePrice == true 43: ? _product / _pseudo + 1 - 44:: _product / _pseudo - 1; + 44:: _product / _pseudo; 45: }

## Assessed type

Math vm06007 (Wise Lending) commented:

If you remove -1 then it opens other attacks, so it is not justified suggestion. To qualify this for a finding I will let @vonMangoldt give his opinion for these details.

Trust (judge) decreased severity to Medium and commented:

High quality submission. Likelihood is Low/Med, Impact is Med/High, so Medium is appropriate.

t0x1c (warden) commented:

Likelihood is Low/Med @Trust - Could you please expand on your reasoning behind this? As I mentioned in the report, the frequency/likelihood at which this happens currently is very high which I supported by various random examples.

To strengthen my case, here are additional example flows of events which cause a revert for Bob when he tries to save his collateral by depositing additional amount. I have even added more actors so that it can mimic a real world scenario even more closely than before.

I am also supplementing Scenario 1 and Scenario 2 (below) of the table with coded PoCs (very similar to the one I already provided in my report), just in case it helps to run it and see the scenario in action. Due to these reasons, I believe the vulnerability should qualify as a High. Requesting you to re-assess.

# Action1 Action2 Action3 Action4 Action5 Scenario 1 Alice deposits 2 ether Bob deposits 2 ether Carol deposits 1 ether Bob borrows 1 ether Bob deposits 0.5 ether (reverts) Scenario 2 Alice deposits 2 ether Bob deposits 3 ether Carol deposits 2.5 ether Bob borrows 2 ether Bob deposits 1 ether (reverts) function test_t0x1c_MultipleDepositsCombinations_Revert_With_Borrow_Scenario1 () public { address alice = makeAddr ( "Alice" ); address bob = makeAddr ( "Bob" ); address carol = makeAddr ( "Carol" ); vm.

deal ( alice, 10 ether ); vm.

deal ( bob, 10 ether ); vm.

deal ( carol, 10 ether ); console.

log ( "Action1" ); vm.

prank ( alice ); uint256 nftId = POSITION_NFTS_INSTANCE.

mintPosition (); vm.

prank ( alice ); LENDING_INSTANCE.

depositExactAmountETH {value:

2 ether }( nftId ); console.

log ( "Action2" ); vm.

prank ( bob ); uint256 nftId_Bob = POSITION_NFTS_INSTANCE.

mintPosition (); vm.

prank ( bob ); LENDING_INSTANCE.

depositExactAmountETH {value:

2 ether }( nftId_Bob ); console.

log ( "Action3" ); vm.

prank ( carol ); nftId = POSITION_NFTS_INSTANCE.

mintPosition (); vm.

prank ( carol ); LENDING_INSTANCE.

depositExactAmountETH {value:

1 ether }( nftId ); console.

log ( "Action4" ); vm.

prank ( bob ); LENDING_INSTANCE.

borrowExactAmountETH ( nftId_Bob, 1 ether ); console.

log ( "Action5" ); vm.

prank ( bob ); LENDING_INSTANCE.

depositExactAmountETH {value:

0.5 ether }( nftId_Bob ); // @audit: REVERTS incorrectly; Bob can't deposit additional collateral to save himself } and function test_t0x1c_MultipleDepositsCombinations_Revert_With_Borrow_Scenario2 () public { address alice = makeAddr ( "Alice" ); address bob = makeAddr ( "Bob" ); address carol = makeAddr ( "Carol" ); vm.

deal ( alice, 10 ether ); vm.

deal ( bob, 20 ether ); vm.

deal ( carol, 10 ether ); console.

log ( "Action1" ); vm.

prank ( alice ); uint256 nftId = POSITION_NFTS_INSTANCE.

mintPosition (); vm.

prank ( alice ); LENDING_INSTANCE.

depositExactAmountETH {value:

2 ether }( nftId ); console.

log ( "Action2" ); vm.

prank ( bob ); uint256 nftId_Bob = POSITION_NFTS_INSTANCE.

mintPosition (); vm.

prank ( bob ); LENDING_INSTANCE.

depositExactAmountETH {value:

3 ether }( nftId_Bob ); console.

log ( "Action3" ); vm.

prank ( carol ); nftId = POSITION_NFTS_INSTANCE.

mintPosition (); vm.

prank ( carol ); LENDING_INSTANCE.

depositExactAmountETH {value:

2.5 ether }( nftId ); console.

log ( "Action4" ); vm.

prank ( bob ); LENDING_INSTANCE.

borrowExactAmountETH ( nftId_Bob, 2 ether ); console.

log ( "Action5" ); vm.

prank ( bob ); LENDING_INSTANCE.

depositExactAmountETH {value:

1 ether }( nftId_Bob ); // @audit: REVERTS incorrectly; Bob can't deposit additional collateral to save himself } Trust (judge) commented:

There is a huge number of combinations which would cause a revert, but considering the vast space of uint256 that number is actually small. Eventually, the issue is called by a rounding which is off by one, and is easily fixed in a repeat transaction. High would be an overstatement.

Wise Lending commented:

Mitigated here.

## Rejected Primary Findings

# Rejected Primary Findings: Wise Lending 

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
