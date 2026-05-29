# Benchmark Ground Truth: Gondi Invitational

## Accepted H/M Findings

# Accepted H/M Findings: Gondi Invitational

# [H-01] Merging tranches could make _loanTermination() accounting incorrect

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

_loanTermination() accounting incorrect Submitted by minhquanym, also found by bin2chen In the Pool contract, when a loan is repaid or liquidated, a call to the Pool is made for accounting. The _loanTermination() function is eventually invoked. This function uses the loanId to determine the withdrawal queue to which the loan belongs. If the loan was issued after the last queue, it belongs entirely to the pool, and _outstandingValues is updated. If not, it updates the queue accounting, queue outstanding values, getTotalReceived and getAvailableToWithdraw.

function _loanTermination (...

) private { uint256 pendingIndex = _pendingQueueIndex; uint256 totalQueues = getMaxTotalWithdrawalQueues + 1; uint256 idx; /// @dev oldest queue is the one after pendingIndex uint256 i; for ( i = 1; i < totalQueues;) { idx = ( pendingIndex + i ) % totalQueues; if ( getLastLoanId [ idx ][ _loanContract ] >= _loanId ) { break; } unchecked { ++ i; } /// @dev We iterated through all queues and never broke, meaning it was issued after the newest one.

if ( i == totalQueues ) { _outstandingValues = _updateOutstandingValuesOnTermination ( _outstandingValues, _principalAmount, _apr, _interestEarned ); return; } else { uint256 pendingToQueue = _received.

mulDivDown ( PRINCIPAL_PRECISION - _queueAccounting [ idx ].

netPoolFraction, PRINCIPAL_PRECISION ); getTotalReceived [ idx ] += _received; getAvailableToWithdraw += pendingToQueue; _queueOutstandingValues [ idx ] = _updateOutstandingValuesOnTermination ( _queueOutstandingValues [ idx ], _principalAmount, _apr, _interestEarned ); } However, the mergeTranches() function is permissionless and only requires the merged tranches to be contiguous. Once tranches are merged, the loanId of the new tranche changes, which can lead to incorrect accounting in the Pool.

## Recommended Mitigation Steps

Limit the ability to call mergeTranches() directly to lenders only.

0xend (Gondi) confirmed Gondi mitigated:

Only tranche lender can call mergeTranches so it assumes the responsibility.

Status:

Mitigation confirmed. Full details in reports from minhquanym and bin2chen.

# [H-02] Division before multiplication could lead to users losing 50% in WithdrawalQueue

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

WithdrawalQueue Submitted by minhquanym In the _getAvailable() function, the calculation performs division before multiplication, which could result in precision loss. The consequence is that users may not be able to withdraw the amount they should receive, leaving some funds locked in the WithdrawalQueue.

// @audit division before multiplication function _getAvailable ( uint256 _tokenId ) private view returns ( uint256 ) { return getShares [ _tokenId ] * _getWithdrawablePerShare () - getWithdrawn [ _tokenId ]; } /// @notice Get the amount that can be withdrawn per share.

function _getWithdrawablePerShare () private view returns ( uint256 ) { return ( _totalWithdrawn + _asset.

balanceOf ( address ( this ))) / getTotalShares; }

## Recommended Mitigation Steps

Change the order of calculation to multiply before division.

## Assessed type

Math 0xend (Gondi) confirmed Gondi mitigated:

Change order in multiplication/division as suggested.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-03] Function distribute() lacks access control allowing anyone to spam and disrupt the pool’s accounting

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

distribute() lacks access control allowing anyone to spam and disrupt the pool’s accounting Submitted by minhquanym, also found by zhaojie The LiquidationDistributor contract manages the distribution of funds after a liquidation auction is settled. It distributes the received funds to the lenders of the loan. If the lender has implemented the LoanManager interface, it will also call loanLiquidation() on the lender’s address. The Pool, when loanLiquidation() is called, will conduct an accounting process to ensure that the received funds are fairly distributed to the depositors.

function loanLiquidation ( uint256 _loanId, uint256 _principalAmount, uint256 _apr, uint256, uint256 _protocolFee, uint256 _received, uint256 _startTime ) external override onlyAcceptedCallers { uint256 netApr = _netApr ( _apr, _protocolFee ); uint256 interestEarned = _principalAmount.

getInterest ( netApr, block.

timestamp - _startTime ); uint256 fees = IFeeManager ( getFeeManager ).

processFees ( _received, 0 ); getCollectedFees += fees; // @audit Accounting logic _loanTermination ( msg.

sender, _loanId, _principalAmount, netApr, interestEarned, _received - fees ); } However, the distribute() function lacks access control. Consequently, an attacker could directly call it with malicious data, leading to incorrect accounting in the Pool.

## Recommended Mitigation Steps

Only allow Loan contracts to call the distribute() function.

## Assessed type

Access Control 0xend (Gondi) confirmed Gondi mitigated:

Added caller check to avoid anyone calling distribute.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-04] Function refinanceFromLoanExecutionData() does not check executionData.tokenId == loan.nftCollateralTokenId

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

refinanceFromLoanExecutionData() does not check executionData.tokenId == loan.nftCollateralTokenId Submitted by minhquanym, also found by oakcobalt and bin2chen The refinanceFromLoanExecutionData() function is used to refinance a loan from LoanExecutionData. It allows borrowers to use outstanding offers for new loans to refinance their current loan. This function essentially combines two actions: it processes the repayment for the previous loan and then emits a new loan.

The key difference is that the same NFT is used as collateral for the new loan, so it does not need to be transferred out of the protocol and then transferred back in. However, there is no check to ensure that the NFT id of the old loan matches the NFT id of the new execution data.

Therefore, the new loan may have a collateral NFT that does not match the NFT that the lender requested in their offers.

/// @dev We first process the incoming offers so borrower gets the capital. After that, we process repayments.

/// NFT doesn't need to be transferred (it was already in escrow) ( uint256 newLoanId, uint256 [] memory offerIds, Loan memory loan, uint256 totalFee ) = _processOffersFromExecutionData ( borrower, executionData.

principalReceiver, principalAddress, nftCollateralAddress, executionData.

tokenId, // @audit No check if matched with loan.nftCollateralTokenId executionData.

duration, offerExecution );

## Recommended Mitigation Steps

Add a check to ensure that executionData.tokenId is equal to loan.nftCollateralTokenId.

## Assessed type

Invalid Validation 0xend (Gondi) confirmed via duplicate Issue #14 Gondi mitigated:

Added tokenIdCheck.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-05] triggerFee is stolen from other auctions during settleWithBuyout()

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

triggerFee is stolen from other auctions during settleWithBuyout() Submitted by minhquanym, also found by bin2chen The function settleWithBuyout() is used to settle an auction with a buyout from the main lender. This lender needs to repay all other lenders and will receive the NFT collateral. Near the end of the function, the triggerFee is also paid to the auction originator. However, the funds used to pay this fee are taken directly from the contract balance, even though the main lender doesn’t transfer these funds into the contract.

function settleWithBuyout (...

) external nonReentrant {...

// @note Repay other lenders ERC20 asset = ERC20 ( _auction.

asset ); uint256 totalOwed; for ( uint256 i; i < _loan.

tranche.

length;) {...

} IMultiSourceLoan ( _auction.

loanAddress ).

loanLiquidated ( _auction.

loanId, _loan ); // @audit There is no fund in this contract to pay triggerFee asset.

safeTransfer ( _auction.

originator, totalOwed.

mulDivDown ( _auction.

triggerFee, _BPS ));...

} As a result, if the auction contract balance is insufficient to cover the fee, the function will simply revert and prevent the main lender from buying out. In other cases where multiple auctions are running in parallel, the triggerFee will be deducted from the other auctions. This could lead to the last auctions being unable to settle due to insufficient balance.

## Recommended Mitigation Steps

Consider using safeTransferFrom() to pay the triggerFee from the sender’s address, rather than using safeTransfer() to pay the triggerFee from the contract balance.

0xend (Gondi) confirmed 0xA5DF (judge) commented:

Sustaining high severity because this is going to cause a loss of principal to other auctions.

Gondi mitigated:

Change to safeTransferFrom buyer.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-06] Function settleWithBuyout() does not call LoanManager.loanLiquidation() during a buyout

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

settleWithBuyout() does not call LoanManager.loanLiquidation() during a buyout Submitted by minhquanym, also found by bin2chen Lenders in the Gondi protocol could be EOA and Gondi Pool. Gondi Pool, an ERC4626, allows anyone to deposit funds and earn yield from lending on Gondi. Gondi Pool implemented the LoanManager interfaces, which include the validateOffer(), loanRepayment(), and loanLiquidation() functions. The functions loanRepayment() and loanLiquidation() are called when a borrower repays the loan or the loan is liquidated, i.e., when the Pool receives funds back from MultiSourceLoan. Both functions is used to update the queue accounting and the outstanding values of the Pool.

ERC20 asset = ERC20 ( _auction.

asset ); uint256 totalOwed; // @audit Repay lender but not call LoanManager.loanLiquidation() for ( uint256 i; i < _loan.

tranche.

length;) { if ( i != largestTrancheIdx ) { IMultiSourceLoan.

Tranche calldata thisTranche = _loan.

tranche [ i ]; uint256 owed = thisTranche.

principalAmount + thisTranche.

accruedInterest + thisTranche.

principalAmount.

getInterest ( thisTranche.

aprBps, block.

timestamp - thisTranche.

startTime ); totalOwed += owed; asset.

safeTransferFrom ( msg.

sender, thisTranche.

lender, owed ); } unchecked { ++ i; } IMultiSourceLoan ( _auction.

loanAddress ).

loanLiquidated ( _auction.

loanId, _loan ); In the settleWithBuyout() function, the main lender buys out the loan by repaying all other lenders directly. However, loanLiquidation() is not called, leading to incorrect accounting in the Pool.

## Recommended Mitigation Steps

Consider checking and calling loanLiquidation() in settleWithBuyout() to ensure accurate accounting in the pool.

0xend (Gondi) confirmed and commented:

Changing interest paid to use the end of the loan (this appears in another issue since this delta in time otherwise breaks the maxSeniorRepayment concept).

Gondi mitigated:

Added loanLiquidation call.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-07] deployWithdrawalQueue() need to clear _queueAccounting[lastQueueIndex]

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

deployWithdrawalQueue() need to clear _queueAccounting[lastQueueIndex] Submitted by bin2chen In deployWithdrawalQueue(), only clears _queueOutstandingValues[lastQueueIndex] and _outstandingValues, but doesn’t clear _queueAccounting[lastQueueIndex].

function deployWithdrawalQueue () external nonReentrant {...

/// @dev We move outstanding values from the pool to the queue that was just deployed.

_queueOutstandingValues [ pendingQueueIndex ] = _outstandingValues; /// @dev We clear values of the new pending queue.

delete _queueOutstandingValues [ lastQueueIndex ]; delete _outstandingValues; @> //@audit miss delete _queueAccounting[lastQueueIndex] _updateLoanLastIds (); @> _pendingQueueIndex = lastQueueIndex; // Cannot underflow because the sum of all withdrawals is never larger than totalSupply.

unchecked { totalSupply -= sharesPendingWithdrawal; } After this method, anyone calling queueClaimAll() will use this stale data _queueAccounting[lastQueueIndex].

queueClaimAll() -> _queueClaimAll(_pendingQueueIndex) -> _updatePendingWithdrawalWithQueue(_pendingQueueIndex) function _updatePendingWithdrawalWithQueue ( uint256 _idx, uint256 _cachedPendingQueueIndex, uint256 [] memory _pendingWithdrawal ) private returns ( uint256 [] memory ) { uint256 totalReceived = getTotalReceived [ _idx ]; uint256 totalQueues = getMaxTotalWithdrawalQueues + 1; /// @dev Nothing to be returned if ( totalReceived == 0 ) { return _pendingWithdrawal; } getTotalReceived [ _idx ] = 0; /// @dev We go from idx to newer queues. Each getTotalReceived is the total /// returned from loans for that queue. All future queues/pool also have a piece of it.

/// X_i: Total received for queue `i` /// X_1 = Received * shares_1 / totalShares_1 /// X_2 = (Received - (X_1)) * shares_2 / totalShares_2...

/// Remainder goes to the pool.

for ( uint256 i; i < totalQueues;) { uint256 secondIdx = ( _idx + i ) % totalQueues; @> QueueAccounting memory queueAccounting = _queueAccounting [ secondIdx ]; if ( queueAccounting.

thisQueueFraction == 0 ) { unchecked { ++ i; } continue; } /// @dev We looped around.

@> if ( secondIdx == _cachedPendingQueueIndex + 1 ) { break; } uint256 pendingForQueue = totalReceived.

mulDivDown ( queueAccounting.

thisQueueFraction, PRINCIPAL_PRECISION ); totalReceived -= pendingForQueue; _pendingWithdrawal [ secondIdx ] = pendingForQueue; unchecked { ++ i; } return _pendingWithdrawal; }

## Impact

Not clearing _queueAccounting[lastQueueIndex] when executing queueClaimAll() will use this stale data to distribute totalReceived.

## Recommended Mitigation

function deployWithdrawalQueue() external nonReentrant {...

/// @dev We move outstaning values from the pool to the queue that was just deployed.

_queueOutstandingValues[pendingQueueIndex] = _outstandingValues; /// @dev We clear values of the new pending queue.

delete _queueOutstandingValues[lastQueueIndex]; + delete _queueAccounting[lastQueueIndex] delete _outstandingValues; _updateLoanLastIds(); _pendingQueueIndex = lastQueueIndex; // Cannot underflow because the sum of all withdrawals is never larger than totalSupply.

unchecked { totalSupply -= sharesPendingWithdrawal; }

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Clear state vars.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-08] Incorrect circular array check in _updatePendingWithdrawalWithQueue flow, causing received funds to be added to the wrong queues

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

_updatePendingWithdrawalWithQueue flow, causing received funds to be added to the wrong queues Submitted by oakcobalt In Pool.sol, queueClaimAll flow will transfer received funds (returned funds from loans) for each queue to newer queues.

Received funds for a given queue are intended to be distributed to newer queues:

/// @dev We go from idx to newer queues. Each getTotalReceived is the total /// returned from loans for that queue. All future queues/pool also have a piece of it.

/// X_i: Total received for queue `i` /// X_1 = Received * shares_1 / totalShares_1 /// X_2 = (Received - (X_1)) * shares_2 / totalShares_2...

/// Remainder goes to the pool.

This logic is implemented in _updatePendingWithdrawalWithQueue(). Due to queue arrays are circular, the array index never exceeds getMaxTotalWithdrawalQueues and will restart from 0.

% totalQueues should be used when checking array indexes in most cases. However, in the queue index for-loop, if (secondIdx == _cachedPendingQueueIndex + 1) {break;} is used to break the loop instead of secondIdx == (_cachedPendingQueueIndex + 1)%totalQueues.

This is problematic in some cases:

When _cachedPendingQueueIndex < getMaxTotalWithdrawalQueues.

_updatePendingWithdrawalWithQueue() will always skip the oldest queue when distributing getTotalReceived[_idx] funds.

In _queueClaimAll(), the first for-loop start with the oldestQueueIndex ( _cachedPendingQueueIndex + 1) % totalQueues + 0)%totalQueues ). When ( _cachedPendingQueueIndex + 1 ) % totalQueues== _cachedPendingQueueIndex + 1, this first iteration will always result in a break in the second for-loop, where secondIdx == oldestQueueIndex == _cachedPendingQueueIndex + 1.

As a result, any received funds from the oldesQueueIndex ( getTotalReceived\[oldestQueueIndex\] ) will not be distributed and directly deleted ( getTotalReceived[_idx] = 0; ).

When _cachedPendingQueueIndex == getMaxTotalWithdrawalQueues The second for-loop will never break, because secondIdx < _cachedPendingQueueIndex + 1.

for (uint256 i; i < totalQueues;) will always run getMaxTotalWithdrawalQueues+1 times. This will result in received funds from any queues being distributed to both newer queues and older queues.

## Recommended Mitigation Steps

Based on my understanding, this should be if (i≠0 && secondIdx == (_cachedPendingQueueIndex + 1)%totalQueues) { break;}

## Assessed type

Error 0xA5DF (judge) commented:

causing received funds to be added to the wrong queues What are the consequences of that? I’ll might consider high severity if this leads to frozen funds.

0xend (Gondi) confirmed and commented:

Conversation continued over discord. There’s a bug here I believe (a high severity one), the condition for breaking the loop should be secondIdx == _cachedPendingQueueIndex.

0xA5DF (judge) increased severity to High Gondi mitigated:

Need to break 1 before.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-09] Incorrect accounting of _pendingWithdrawal in queueClaiming flow

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-09
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

_pendingWithdrawal in queueClaiming flow Submitted by oakcobalt, also found by bin2chen Incorrect accounting of _pendingWithdrawal in queueClaiming flow, funds received from a previous queue index will be lost.

## Recommended Mitigation Steps

Change into _pendingWithdrawal[secondIdx] + = pendingForQueue;.

## Assessed type

Error 0xend (Gondi) confirmed Gondi mitigated:

Missing +.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-10] The attackers front-running repayloans so that the debt cannot be repaid

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-10
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

repayloans so that the debt cannot be repaid Submitted by zhaojie, also found by minhquanym The attackers make it impossible for borrowers to repay their debts, and the collateral is liquidated when the debts mature.

## Recommended Mitigation Steps

Do not delete _loanId.

## Assessed type

DoS 0xA5DF (judge) commented:

I have some doubts about severity, since this requires too many resources from the attacker (see here ), and the addNewTranche() requires the lender’s signature (and when using mergeTranches() alone the attacker would eventually run out of tranches to merge).

0xend (Gondi) confirmed and commented:

I think this is low (agree with judge for those reasons).

0xA5DF (judge) decreased severity to Low and commented:

I think there are too many limitations on this one, and the motivation for the attacker isn’t very high - they’re not going to get the entire principal from this.

0xend (Gondi) commented:

Given the limit on tranches the attacker can only run this a handful of times.

zhaojie (warden) commented:

I think it’s a high risk, because anyone can be an attacker, so Lender can be an attacker.

If the lender does not want the borrower to repay the debt, the lender can use addNewTranche/mergeTranches and to attack repayLoans and make the borrower’s loan impossible to repay, especially when the loan is about to expire. This causes the borrower’s NFT to be loss, so it would have a high impact.

When _liquidateLoan, if _canClaim == true, the borrower can get the NFT directly:

function _liquidateLoan....{....

if ( _canClaim ) { ERC721 ( _loan.

nftCollateralAddress ).

transferFrom ( address ( this ), _loan.

tranche [ 0 ].

lender, _loan.

nftCollateralTokenId ); emit LoanForeclosed ( _loanId ); liquidated = true; }....

} function liquidateLoan ( uint256 _loanId, Loan calldata _loan )... {.....

( bool liquidated, bytes memory liquidation ) = _liquidateLoan ( _loanId, _loan, _loan.

tranche.

length == 1 && !

getLoanManagerRegistry.

isLoanManager ( _loan.

tranche [ 0 ].

lender ) );......

} An attacker/lender can use mergeTranches to make _loan.tranche.length == 1. The key issue is that loanId will be reset.

0xA5DF (judge) increased severity to High and commented:

You’re right that the lender has a high motivation to execute this attack. You’re also right that when the borrower attempts to repay close to the expiry time this attack becomes feasible.

While some conditions are required in order for this to work, it still seems pretty likely to happen. Due to those reasons I’m reinstating high severity.

Side note: I think that a better mitigation would be to not allow functions that change the loanID to run near the expiry time.

0xend (Gondi) commented:

No specific PR here since it’s addressed when limiting addNewTranche to only be able to be called by the borrower and checking in refinancePartial that there’s at least one tranche being refinanced. This ends up limiting the number of times a loan can be locked by the lender (tranches are locked for some time after a refinance for future ones).

# [H-11] Incorrect protocol fee implementation results in outstandingValues to be mis-accounted in Pool.sol

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-11
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

outstandingValues to be mis-accounted in Pool.sol Submitted by oakcobalt The vulnerability is that LiquidationDistributer::_handleLoanMangerCall hardcodes 0 as protocol fee when calling LoanManager(_tranche.lender).loanLiquidation().

//src/lib/LiquidationDistributor.sol function _handleLoanManagerCall (IMultiSourceLoan.Tranche calldata _tranche, uint256 _sent ) private { if ( getLoanManagerRegistry.

isLoanManager ( _tranche.

lender )) { LoanManager ( _tranche.

lender ).

loanLiquidation ( _tranche.

loanId, _tranche.

principalAmount, _tranche.

aprBps, _tranche.

accruedInterest, |> 0, //@audit this should be the actual protocol fee fraction _sent, _tranche.

startTime ); } _handleLoanManagerCall() will be called as part of the flow to distribute proceeds from a liquidation.

When protocol fee is hardcoded 0, in the Pool::loanliquidation call, netApr will not account for protocol fee fraction which will inflate the _apr used to offset _outstandingValues.sumApr, a state variable that accounts for the total annual apr of outstanding loans.

//src/lib/pools/Pool.sol OutstandingValues memory __outstandingValues, uint256 _principalAmount, uint256 _apr, uint256 _interestEarned ) private view returns ( OutstandingValues memory ) {...

//@audit inflated _apr will offset __outstandingValues.sumApr to an incorrect lower value, causing accounting error |> __outstandingValues.

sumApr -= uint128 ( _apr * _principalAmount );...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/pools/Pool.sol#L751
For comparison, when a loan is created ( pool::validateOffer ), the actual protocol fee ( protocolFee.fraction ) will be passed, and __outstandingValues.sumApr will be added with the post-fee apr value, instead of the before-fee apr.

State accounting __outstandingValues will be incorrect, all flows that consume __outstandingValues.sumApr when calculating interests will be affected.

## Recommended Mitigation Steps

User _loan.protocolFee instead of 0.

0xend (Gondi) confirmed and commented:

Messes up with accounting, I think this is a high one.

Gondi mitigated:

Passing protocol fee.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-12] addNewTranche() no authorization from borrower

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-12
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

addNewTranche() no authorization from borrower Submitted by bin2chen, also found by oakcobalt, minhquanym, and zhaojie For addNewTranche(), the code implementation is as follows： function addNewTranche ( RenegotiationOffer calldata _renegotiationOffer, Loan memory _loan, bytes calldata _renegotiationOfferSignature ) external nonReentrant returns ( uint256, Loan memory ) { uint256 loanId = _renegotiationOffer.

loanId; _baseLoanChecks ( loanId, _loan ); _baseRenegotiationChecks ( _renegotiationOffer, _loan ); @> _checkSignature ( _renegotiationOffer.

lender, _renegotiationOffer.

hash (), _renegotiationOfferSignature ); if ( _loan.

tranche.

length == getMaxTranches ) { revert TooManyTranchesError (); } uint256 newLoanId = _getAndSetNewLoanId (); Loan memory loanWithTranche = _addNewTranche ( newLoanId, _loan, _renegotiationOffer ); _loans [ newLoanId ] = loanWithTranche.

hash (); delete _loans [ loanId ]; ERC20 ( _loan.

principalAddress ).

safeTransferFrom ( _renegotiationOffer.

lender, _loan.

borrower, _renegotiationOffer.

principalAmount - _renegotiationOffer.

fee ); if ( _renegotiationOffer.

fee > 0 ) { /// @dev Cached ProtocolFee memory protocolFee = _protocolFee; ERC20 ( _loan.

principalAddress ).

safeTransferFrom ( _renegotiationOffer.

lender, protocolFee.

recipient, _renegotiationOffer.

fee.

mulDivUp ( protocolFee.

fraction, _PRECISION ) ); } emit LoanRefinanced ( _renegotiationOffer.

renegotiationId, loanId, newLoanId, loanWithTranche, _renegotiationOffer.

fee ); return ( newLoanId, loanWithTranche ); } Currently only the signature of the lender is checked, not the authorization of the borrower. Then, any lender can add tranche to any loan by:

Specifying a very high apr.

Specifying any _renegotiationOffer.fee; for example: set _renegotiationOffer.fee==_renegotiationOffer.principalAmount.

This doesn’t make sense for borrower. It is recommended that only the borrower performs this method.

## Impact

lender can be specified to generate a malicious tranche to compromise borrower.

## Recommended Mitigation

function addNewTranche( RenegotiationOffer calldata _renegotiationOffer, Loan memory _loan, bytes calldata _renegotiationOfferSignature ) external nonReentrant returns (uint256, Loan memory) { uint256 loanId = _renegotiationOffer.loanId; + if (msg.sender != _loan.borrower) { + revert InvalidCallerError(); + } _baseLoanChecks(loanId, _loan); _baseRenegotiationChecks(_renegotiationOffer, _loan); _checkSignature(_renegotiationOffer.lender, _renegotiationOffer.hash(), _renegotiationOfferSignature); if (_loan.tranche.length == getMaxTranches) { revert TooManyTranchesError(); }

## Assessed type

Context 0xend (Gondi) confirmed via duplicate Issue #52 Gondi mitigated:

Added caller check.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-13] _processOffersFromExecutionData() lack of check executionData.duration<=offer.duration

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-13
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

_processOffersFromExecutionData() lack of check executionData.duration<=offer.duration Submitted by bin2chen emitLoan() only limits offer.duration != 0. There’s no limit in executionData.duration<=offer.duration.

emitLoan() -> _processOffersFromExecutionData() -> _validateOfferExecution() function _validateOfferExecution ( OfferExecution calldata _offerExecution, uint256 _tokenId, address _lender, address _offerer, bytes calldata _lenderOfferSignature, uint256 _feeFraction, uint256 _totalAmount ) private {...

@> if ( offer.

duration == 0 ) { revert ZeroDurationError (); } if ( offer.

aprBps == 0 ) { revert ZeroInterestError (); } if (( offer.

capacity > 0 ) && ( _used [ _offerer ][ offer.

offerId ] + _offerExecution.

amount > offer.

capacity )) { revert MaxCapacityExceededError (); } _checkValidators ( _offerExecution.

offer, _tokenId ); } If the executionData.duration time is not limited, it can lead to far exceeding the borrowing time offer.duration. If the lender is a LoanManager, when repayLoan() it can also exceed the maximum pendingQueues, leading to accounting issues.

## Impact

Far exceeding the borrowing time than offer.duration. If lender is LoanManager also exceeds max pendingQueues, causing bookkeeping issues.

## Recommended Mitigation

Check executionData.duration<=offer[n].duration.

## Assessed type

Context 0xend (Gondi) confirmed 0xA5DF (judge) commented:

Sustaining high due to accounting issues.

Gondi mitigated:

Added duration check.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-14] mergeTranches() / refinancePartial() lack of nonReentrant

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-14
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

mergeTranches() / refinancePartial() lack of nonReentrant Submitted by bin2chen In mergeTranches(), the method’s code implementation is as follows:

function mergeTranches ( uint256 _loanId, Loan memory _loan, uint256 _minTranche, uint256 _maxTranche ) external returns ( uint256, Loan memory ) { _baseLoanChecks ( _loanId, _loan ); uint256 loanId = _getAndSetNewLoanId (); Loan memory loanMergedTranches = _mergeTranches ( loanId, _loan, _minTranche, _maxTranche ); _loans [ loanId ] = loanMergedTranches.

hash (); delete _loans [ _loanId ]; emit TranchesMerged ( loanMergedTranches, _minTranche, _maxTranche ); return ( loanId, loanMergedTranches ); } As shown above, this method lacks reentrancy protection, which could allow reentrancy attacks to manipulate the _loans[].

Example: Suppose _loans[1] = {NFT = 1} Alice calls refinanceFromLoanExecutionData(_loans\[1],LoanExecutionData).

LoanExecutionData.ExecutionData.OfferExecution.LoanOffer.OfferValidator\[0].validator = CustomContract => for callback.

refinanceFromLoanExecutionData() -> _processOffersFromExecutionData() -> _validateOfferExecution() -> _checkValidators() -> IOfferValidator(CustomContract).validateOffer().

In IOfferValidator(CustomContract).validateOffer(), call MultiSourceLoan.mergeTranches(_loans[1]) -> pass without nonReentrant.

_loans\[3] = newLoan.hash() Return to refinanceFromLoanExecutionData(), will execute:

_loans\[2] = newOtherLoan.hash().

There will be _loans[2] and _loans[3], both containing NFT=1. Note: Both Loans ‘s lender are all himself:

The user can repayLoan(_loans[2]) and get the NFT back.

Use the NFT to borrow other people’s funds, e.g. to generate _loans[100].

repayLoan(_loans[3]), get NFT back.

## Recommended Mitigation

Add nonReentrant:

function mergeTranches(uint256 _loanId, Loan memory _loan, uint256 _minTranche, uint256 _maxTranche) external + nonReentrant returns (uint256, Loan memory) { _baseLoanChecks(_loanId, _loan); uint256 loanId = _getAndSetNewLoanId(); function refinancePartial(RenegotiationOffer calldata _renegotiationOffer, Loan memory _loan) external + nonReentrant returns (uint256, Loan memory) {

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Added nonReentrant.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-15] _baseLoanChecks() check errors for expire

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-15
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

_baseLoanChecks() check errors for expire Submitted by bin2chen, also found by zhaojie and minhquanym _baseLoanChecks() is used to check whether Loan has expired:

function _baseLoanChecks ( uint256 _loanId, Loan memory _loan ) private view { if ( _loan.

hash () != _loans [ _loanId ]) { revert InvalidLoanError ( _loanId ); } @> if ( _loan.

startTime + _loan.

duration < block.

timestamp ) { revert LoanExpiredError (); } The expiration checks in liquidation are as follows:

function _liquidateLoan ( uint256 _loanId, IMultiSourceLoan.Loan calldata _loan, bool _canClaim ) internal returns ( bool liquidated, bytes memory liquidation ) {...

uint256 expirationTime = _loan.

startTime + _loan.

duration; @> if ( expirationTime > block.

timestamp ) { revert LoanNotDueError ( expirationTime ); } This way, both checks pass when block.timestamp == _loan.startTime + _loan.duration.

This leads to the problem that a malicious attacker can perform the following steps when block.timestamp == _loan.startTime + _loan.duration:

Alice calls liquidateLoan ( loandId = 1) -> success.

LoanLiquidator generates an auction.

_loans\[loandId = 1] is still valid, and will only be cleared when the auction is over.

Alice call addNewTranche ( loandId = 1) -> success.

_baseLoanChecks ( loandId = 1) will pass.

delete _loans\[1]; _loans\[2] = newLoan.hash().

Bidding ends, call loanLiquidated(loandId = 1) will fail, because _loans[1] has been cleared.

## Impact

Maliciously disrupting the end of the bidding, causing the NFT/funds to be locked.

## Recommended Mitigation

function _baseLoanChecks(uint256 _loanId, Loan memory _loan) private view { if (_loan.hash() != _loans[_loanId]) { revert InvalidLoanError(_loanId); } - if (_loan.startTime + _loan.duration < block.timestamp) { + if (_loan.startTime + _loan.duration <= block.timestamp) { revert LoanExpiredError(); }

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Strict -> <=.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-16] validateOffer() reentry to manipulate exchangeRate

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-16
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

validateOffer() reentry to manipulate exchangeRate Submitted by bin2chen The current mechanism of validateOffer() is to first book _outstandingValues to increase, but assets.balanceOf(address(this)) doesn’t decrease immediately.

function validateOffer ( bytes calldata _offer, uint256 _protocolFee ) external override onlyAcceptedCallers {..

/// @dev Since the balance of the pool includes capital that is waiting to be claimed by the queues, /// we need to check if the pool has enough capital to fund the loan.

/// If that's not the case, and the principal is larger than the currentBalance, then we need to reallocate /// part of it.

if ( principalAmount > undeployedAssets ) { revert InsufficientAssetsError (); } else if ( principalAmount > currentBalance ) { IBaseInterestAllocator ( getBaseInterestAllocator ).

reallocate ( currentBalance, principalAmount - currentBalance, true ); } @> /// @dev If the txn doesn't revert, we can assume the loan was executed.

@> _outstandingValues = _getNewLoanAccounting ( principalAmount, _netApr ( apr, _protocolFee )); } I.e.: After this method is called, _getUndeployedAssets() is unchanged, but _getTotalOutstandingValue() is increased, so totalAssets() is increased, but totalSupply is unchanged, so exchangeRate is get bigger.

Originally, it was expected that after that, the Pool balance would be transferred at MultiSourceLoan, so _getUndeployedAssets() becomes smaller and exchangeRate returns to normal. But if it’s possible to do callback malicious logic before MultiSourceLoan transfers away the Pool balance, it’s possible to take advantage of this exchangeRate that becomes larger.

Example: Suppose _getUndeployedAssets() = 1000, _getTotalOutstandingValue() = 1000 and totalSupply = 2000.

So:

totalAssets() = 2000 exchangeRate = 1:1 Alice calls MultiSourceLoan.emitLoan().

offer.lender = pool.

offer.principalAmount = 500.

offer.validators = CustomContract -> for callback.

emitLoan() -> Pool.validateOffer().

_getUndeployedAssets() = 1000 (no change).

_getTotalOutstandingValue() = 1000 + 500 = 1500 (more 500).

totalAssets() = 2500.

exchangeRate = 1.25: 1.

emitLoan() -> _checkValidators() -> CustomContract.validateOffer() In CustomContract.validateOffer() call pool.redeem (shares) use exchangeRate = 1.25: 1 to get more assets.

emitLoan() -> asset.safeTransferFrom (pool, receiver, 500).

_getUndeployedAssets() = 500.

exchangeRate Expect to return to normal.

## Impact

Manipulating the exchangeRate to redeem additional assets.

## Recommended Mitigation

In validateOffer(), restrict offer.validators to be an empty array to avoid callbacks.

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

validateOffer changed to view so validators cannot change state.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [H-17] refinanceFull / addNewTranche reusing a lender’s signature leads to unintended behavior

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** H-17
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

refinanceFull / addNewTranche reusing a lender’s signature leads to unintended behavior Submitted by bin2chen

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/loans/MultiSourceLoan.sol#L358
- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/loans/MultiSourceLoan.sol#L194

## Vulnerability details

In MultiSourceLoan, refinanceFull() and addNewTranche() use the same signature.

function refinanceFull ( RenegotiationOffer calldata _renegotiationOffer, Loan memory _loan, bytes calldata _renegotiationOfferSignature ) external nonReentrant returns ( uint256, Loan memory ) {...

if ( lenderInitiated ) { if ( _isLoanLocked ( _loan.

startTime, _loan.

startTime + _loan.

duration )) { revert LoanLockedError (); } _checkStrictlyBetter ( _renegotiationOffer.

principalAmount, _loan.

principalAmount, _renegotiationOffer.

duration + block.

timestamp, _loan.

duration + _loan.

startTime, _renegotiationOffer.

aprBps, totalAnnualInterest / _loan.

principalAmount, _renegotiationOffer.

fee ); } else if ( msg.

sender != _loan.

borrower ) { revert InvalidCallerError (); } else { /// @notice Borrowers clears interest @> _checkSignature ( _renegotiationOffer.

lender, _renegotiationOffer.

hash (), _renegotiationOfferSignature ); netNewLender -= totalAccruedInterest; totalAccruedInterest = 0; } function addNewTranche ( RenegotiationOffer calldata _renegotiationOffer, Loan memory _loan, bytes calldata _renegotiationOfferSignature ) external nonReentrant returns ( uint256, Loan memory ) {...

uint256 loanId = _renegotiationOffer.

loanId; _baseLoanChecks ( loanId, _loan ); _baseRenegotiationChecks ( _renegotiationOffer, _loan ); @> _checkSignature ( _renegotiationOffer.

lender, _renegotiationOffer.

hash (), _renegotiationOfferSignature ); if ( _loan.

tranche.

length == getMaxTranches ) { revert TooManyTranchesError (); } So when lender signs RenegotiationOffer, it is meant to replace tranche, i.e. execute refinanceFull(). But a malicious user can use this sign and front-run execute addNewTranche().

addNewTranche() doesn’t limit the RenegotiationOffer too much. The newly generated Loan will be approximately twice the total amount borrowed, and the risk of borrowing against the lender will increase dramatically.

## Impact

Maliciously using the signature of refinanceFull() to execute addNewTranche() will result in approximately double the borrowed amount, and the risk of borrowing will increase dramatically.

## Recommended Mitigation

In RenegotiationOffer, add a type field to differentiate between signatures.

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Check trancheIndex to differentiate between refiFull / addNewTranche.

Status:

Unmitigated. Full details in reports from minhquanym, bin2chen and oakcobalt, and also included in the

# [M-01] Invalid maxTranches check can result in maxTranche cap to be exceeded

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

maxTranches check can result in maxTranche cap to be exceeded Submitted by oakcobalt, also found by minhquanym and bin2chen In src/lib/loans/MultiSourceLoan.sol, there’s max cap for the number of tranches in a loan as defined as getMaxTranches. This cap can be exceeded.

There are two main vulnerabilities:

getMaxTranches is not checked in some key flows where tranches can be added. These including emitLoan(), refinancePartial() (-> _addTrancheFromPartial() ), refinanceFromLoanExecutionData().

Where getMaxTranches is checked, the check is invalid. Only _loan.tranche.length == getMaxTranches is checked. But combined with (1), when number of tranches exceeds getMaxTranches in other flows, this check is invalid.

//src/lib/loans/MultiSourceLoan.sol function addNewTranche ( RenegotiationOffer calldata _renegotiationOffer, Loan memory _loan, bytes calldata _renegotiationOfferSignature ) external nonReentrant returns ( uint256, Loan memory ) {...

//@audit change to _loan.tranch.length >= getMaxTranches |> if ( _loan.

tranche.

length == getMaxTranches ) { revert TooManyTranchesError (); }...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/loans/MultiSourceLoan.sol#L359

## Recommended Mitigation Steps

Add missing checks on getMaxTranches for all flows that might add tranches. In addNewTranche, change into _loan.tranch.length >= getMaxTranches.

0xend (Gondi) confirmed Gondi mitigated:

Check total tranches + min amount per tranche.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-02] A malicious user can take on a loan using an existing borrower’s collateral in refinanceFromLoanExecutionData()

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

refinanceFromLoanExecutionData() Submitted by oakcobalt In MultiSourceLoan.sol, refinanceFromLoanExecutionData() doesn’t check whether _loan.borrower == _loanExecutionData.borrower, which is open rooms for exploits.

BorrowerB (malicious) can sign a _loanExecutionData offer and initiate a refinanceFromLoanExecutionData() call with Borrower A’s loan. Borrower B will use Borrower A’s collateral for his loan.

There are (2) vulnerabilities here:

_validateExecutionData will not check whether _loan.borrower == _executionData.borrower. In addition, it will directly bypass the check on executionData ’s borrower signature as long as msg.sender!=_loan.borrower.

refinanceFromLoanExecutionData() doesn’t check whether the new loanExecutiondata ( _loanExecutionData ) has the same nft tokenId ( executionData.tokenId ) as the existing loan ( _loan.nftCollateralTokenId ).

As a result, if _loanExecutionData.borrower (Borrower B) initiates refinanceFromLoanExecutionData() call, the following would happen:

msg.sender != _loan.borrower (Borrwer A), this bypass _validateExecutionData ’s signature check. Also, it will not revert because no checks on address _borrower ( loan.borrower==_loanExecutionData.borrower;.

There is no check on _loanExecutionData.tokenId. As long as _loan and _loanExecutionData has the same principalAddress and the same nftCollateralAddress, _processOffersFromExecutionData() will succeed in transferring principal loans to Borrower B.

As long as the _loan.borrower (Borrower A) has the funds for repayment. (Note: Borrower A might have approved MultiSourceLoan.sol for asset transfer if they are ready for repayments.), the tx will succeed and Borrower A’s collateral will continually be locked for Borrower B’s new loan.

The above steps can also happen in a front-running scenario, where Borrower B sees that Borrower A approves MultiSourceLoan.sol for loan repayments and front-run Borrower A’s repayment with a new loan.

## Recommended Mitigation Steps

In _validateExecutionData, consider adding checks to ensure address _borrower == _executionData.borrower.

0xend (Gondi) confirmed Note: For full discussion, see here.

Gondi mitigated:

Checking signature from the existing borrower.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-03] Function addNewTranche() should use protocolFee from Loan struct

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

addNewTranche() should use protocolFee from Loan struct Submitted by minhquanym The protocol fee value is recorded and stored in the Loan struct when a new loan is issued. However, when adding a new tranche, the function uses the current value of protocolFee.fraction instead of the value stored in the Loan struct. This could result in inconsistencies in fee collection, as the protocol fee value might be updated by the admin, while the value stored in the Loan struct remains unchanged.

if ( _renegotiationOffer.

fee > 0 ) { /// @dev Cached ProtocolFee memory protocolFee = _protocolFee; ERC20 ( _loan.

principalAddress ).

safeTransferFrom ( _renegotiationOffer.

lender, protocolFee.

recipient, _renegotiationOffer.

fee.

mulDivUp ( protocolFee.

fraction, _PRECISION ) // @audit Use protocolFee from Loan instead ); }

## Recommended Mitigation Steps

Consider using _loan.protocolFee instead of protocolFee.fraction in the addNewTranche() function.

0xend (Gondi) confirmed Gondi mitigated:

addNewTranche uses protocolFee from struct.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-04] Function Pool.validateOffer() does not work correctly in case principalAmount > currentBalance

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

Pool.validateOffer() does not work correctly in case principalAmount > currentBalance Submitted by minhquanym, also found by oakcobalt and bin2chen In the Pool contract, undeployed funds could be deposited to Aave or Lido to earn base yield. When an offer of Pool is accepted from MultiSourceLoan, the function validateOffer() is called to validate the terms and also to pull the undeployed funds back in case the contract balance is insufficient.

if ( principalAmount > undeployedAssets ) { revert InsufficientAssetsError (); } else if ( principalAmount > currentBalance ) { IBaseInterestAllocator ( getBaseInterestAllocator ).

reallocate ( currentBalance, principalAmount - currentBalance, true // @audit Incorrect ); } However, the input params of reallocate() are incorrect, resulting in the contract balance might still be insufficient for the loan after calling the function.

## Recommended Mitigation Steps

Call reallocate(0, principalAmount - currentBalance, true) instead.

0xA5DF (judge) decreased severity to Medium and commented:

If I understand correctly, the impact is DoS that occurs only under certain conditions; in that case, I think severity should be Medium.

0xend (Gondi) confirmed and commented:

Agree on the issue. I think it’s Medium, not High.

Gondi mitigated:

Changed to reallocate ( currentBalance, principalAmount, t r ue) instead of proposed solution (same result) to be compliant with the interface.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-05] Collected fees are never transferred out of Pool contract

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

Submitted by minhquanym, also found by oakcobalt and bin2chen Lenders in the Gondi protocol could be EOA or Gondi Pool. The Gondi Pool, an ERC4626, allows anyone to deposit funds and earn yield from lending on Gondi. When a loan is repaid or liquidated, the pool deducts a fee from the received amount before adding the rest to the pool balance. As shown in the loanRepayment() function, the fees are calculated by calling processFees() and then added to getCollectedFees. After that, the accounting function _loanTermination() is called with the amount being received - fees.

However, this fee is credited to getCollectedFees but never transferred out of the pool. As a result, these funds remain locked in the contract indefinitely.

function loanRepayment ( uint256 _loanId, uint256 _principalAmount, uint256 _apr, uint256, uint256 _protocolFee, uint256 _startTime ) external override onlyAcceptedCallers { uint256 netApr = _netApr ( _apr, _protocolFee ); uint256 interestEarned = _principalAmount.

getInterest ( netApr, block.

timestamp - _startTime ); uint256 received = _principalAmount + interestEarned; uint256 fees = IFeeManager ( getFeeManager ).

processFees ( _principalAmount, interestEarned ); getCollectedFees += fees; // @audit getCollectedFees is never transfer out _loanTermination ( msg.

sender, _loanId, _principalAmount, netApr, interestEarned, received - fees ); }

## Recommended Mitigation Steps

Add a function to collect the credited fees getCollectedFees from the pool in the FeeManager contract.

0xend (Gondi) confirmed and commented:

Not sure if it’s High; tend to think as high as those that would compromise user’s assets. Definitely an issue though.

0xA5DF (judge) decreased severity to Medium and commented:

Marking as med as fees falls under the definition ‘leak of value’.

Gondi mitigated:

Added collectFees method.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-06] Anyone can remove existing term without queueing through setTerms()

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

setTerms() Submitted by minhquanym In PoolOfferHandler, new terms require a two-step process for setting ( setTerms() and confirmTerms() ). The setTerm() function is onlyOwner, but the confirmTerms() function can be called by anyone. This function uses the provided input __terms from the caller to execute the logic. This could enable an attacker to remove all existing terms, even if the owner does not intend to do so (without pending through the setTerms() function).

function confirmTerms ( TermsKey [] calldata _termKeys, Terms [] calldata __terms ) external { if ( block.

timestamp - pendingTermsSetTime < NEW_TERMS_WAITING_TIME ) { revert TooSoonError (); } for ( uint256 i = 0; i < __terms.

length; i ++) { if ( _termKeys [ i ].

duration > getMaxDuration ) { revert InvalidDurationError (); } uint256 pendingAprPremium = _pendingTerms [ _termKeys [ i ].

collection ][ _termKeys [ i ].

duration ][ _termKeys [ i ].

maxSeniorRepayment ][ __terms [ i ].

principalAmount ]; // @audit Can be used to remove terms without pending through setTerm() if ( pendingAprPremium != __terms [ i ].

aprPremium ) { revert InvalidTermsError (); } _terms [ _termKeys [ i ].

collection ][ _termKeys [ i ].

duration ][ _termKeys [ i ].

maxSeniorRepayment ][ __terms [ i ].

principalAmount ] = __terms [ i ].

aprPremium; delete _pendingTerms [ _termKeys [ i ].

collection ][ _termKeys [ i ].

duration ][ _termKeys [ i ].

maxSeniorRepayment ][ __terms [ i ].

principalAmount ]; } pendingTermsSetTime = type ( uint256 ).

max; emit TermsSet ( _termKeys, __terms ); }

## Recommended Mitigation Steps

Only allow the owner to call confirmTerms().

## Assessed type

Invalid Validation 0xend (Gondi) confirmed Gondi mitigated:

Terms must be passed in the confirm as well.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-07] Attacker can front-run and pass in empty terms, making it impossible to confirmTerms()

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

confirmTerms() Submitted by minhquanym, also found by zhaojie and bin2chen In PoolOfferHandler, setting new terms requires two steps ( setTerms() and confirmTerms() ). The setTerm() function is onlyOwner, but anyone can call the confirmTerms() function. At the end of the confirmTerms() function, pendingTermsSetTime is set to type(uint256).max, preventing the function from being called again.

Since confirmTerms() uses the caller’s input to execute the setup, an attacker could input empty __terms to prevent the owner from setting up new terms.

## Recommended Mitigation Steps

Record the __terms list in the setTerms() function to confirm terms instead of using input from caller.

## Assessed type

DoS 0xend (Gondi) confirmed Gondi mitigated:

Terms must be passed in the confirm as well.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-08] Borrower signature could be reused in emitLoan()

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

emitLoan() Submitted by minhquanym The function emitLoan() is used to issue a new loan. This function could be called directly by the borrower or by a random address if the borrower has signed the LoanExecutionData.

function emitLoan ( LoanExecutionData calldata _loanExecutionData ) external nonReentrant returns ( uint256, Loan memory ) { address borrower = _loanExecutionData.

borrower; ExecutionData calldata executionData = _loanExecutionData.

executionData; ( address principalAddress, address nftCollateralAddress ) = _getAddressesFromExecutionData ( executionData ); OfferExecution [] calldata offerExecution = executionData.

offerExecution; // @audit Check borrower signature or borrower is caller _validateExecutionData ( _loanExecutionData, borrower );...

} function _validateExecutionData ( LoanExecutionData calldata _executionData, address _borrower ) private view { if ( msg.

sender != _borrower ) { _checkSignature ( _executionData.

borrower, _executionData.

executionData.

hash (), _executionData.

borrowerOfferSignature ); } if ( block.

timestamp > _executionData.

executionData.

expirationTime ) { revert ExpiredOfferError ( _executionData.

executionData.

expirationTime ); } However, there isn’t a check to ensure the signature for the same LoanExecutionData can’t be used to execute emitLoan() more than once. As a result, if the borrower repays the loan, an attacker could call emitLoan() again to initiate a new loan.

## Recommended Mitigation Steps

Add a nonce to ensure a signature cannot be reused.

0xend (Gondi) acknowledged and commented:

This could be the case but don’t think is high given the struct has an expiration time to contemplate this. Borrower can avoid any issue by setting this to be close to the execution. Loans lasts for months, this will be at most a few hours.

To avoid an extra write+read, I’d probably make it very clear to the consumer of the smart contract.

0xA5DF (judge) decreased severity to Medium and commented:

Marking as Medium due to sponsor comment. This requires some external conditions which don’t seem to be easily satisfied.

0xend (Gondi) commented:

For the replay to happens:

Borrower needs to set an expiration longer than the intended time in which they’ll repay the loan.

All lender offers must not expire before the loan is repaid AND have capacity.

Borrower has no reason to set this variable longer than block.timestamp + small delta (the time it remains in the mempool).

Gondi mitigated:

Borrower should always set block.timestamp + small time delta as expiration to control when the loan can be started.

Status:

Mitigation confirmed. Full details in reports from oakcobalt and bin2chen.

# [M-09] Inconsistent accounting of undeployedAssets might result in undesired optimal range in the pool

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

undeployedAssets might result in undesired optimal range in the pool Submitted by oakcobalt, also found by bin2chen undeployedAssets is calculated inconsistently. Currently in _getUndeployedAssets() the protocol collected fees are subtracted; however, in validateOffer, the protocol collected fees are not subtracted.

_getUndeployedAssets(): This is called in deployWithdrawalQueue() to calculate proRata liquid assets to the queue.contractAddress.

function _getUndeployedAssets () private view returns ( uint256 ) { return asset.

balanceOf ( address ( this )) + IBaseInterestAllocator ( getBaseInterestAllocator ).

getAssetsAllocated () |> - getAvailableToWithdraw - getCollectedFees; } uint256 undeployedAssets: This is manually calculated in validateOffer flow, which is used check whether the pool has enough undeployed Assets to cover loan.principalAmount.

function validateOffer ( bytes calldata _offer, uint256 _protocolFee ) external override onlyAcceptedCallers {...

uint256 currentBalance = asset.

balanceOf ( address ( this )) - getAvailableToWithdraw; uint256 baseRateBalance = IBaseInterestAllocator ( getBaseInterestAllocator ).

getAssetsAllocated (); //@audit getCollectedFees is not subtracted |> uint256 undeployedAssets = currentBalance + baseRateBalance; ( uint256 principalAmount, uint256 apr ) = IPoolOfferHandler ( getUnderwriter ).

validateOffer ( IBaseInterestAllocator ( getBaseInterestAllocator ).

getBaseAprWithUpdate (), _offer );...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/pools/Pool.sol#L398
Note that in (2), undeployedAssets are inflated because getCollectedFees are fees protocol collected from liquidation/repayment flows and shouldn’t be considered as liquid assets to cover the loan principal amount.

_reallocate(): This also manually calculate total undeployedAssets amount, but again didn’t account for getCollectedFees.

_reaalocate() balances optimal target idle assets ratio by checking currentBalance / total ratio. Here, currentBalance should be additionally subtracted by getCollectedFees because fees are set aside and shouldn’t be considered idle. This affects optimal range check.

function _reallocate () private returns ( uint256, uint256 ) { /// @dev Balance that is idle and belongs to the pool (not waiting to be claimed) uint256 currentBalance = asset.

balanceOf ( address ( this )) - getAvailableToWithdraw; if ( currentBalance == 0 ) { revert AllocationAlreadyOptimalError (); } uint256 baseRateBalance = IBaseInterestAllocator ( getBaseInterestAllocator ).

getAssetsAllocated (); uint256 total = currentBalance + baseRateBalance; uint256 fraction = currentBalance.

mulDivDown ( PRINCIPAL_PRECISION, total );...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/pools/Pool.sol#L572
Inconsistent accounting in various flows may result in incorrect checks or undesirable optimal ranges.

## Recommended Mitigation Steps

Account for getCollectedFees in (2) and (3), noted above.

0xend (Gondi) confirmed Gondi mitigated:

Missing collected fees in accounting.

Status:

Mitigation confirmed. Full details in reports from oakcobalt and minhquanym.

# [M-10] Any liquidators can pretend to be a loan contract to validate offers, due to insufficient validation

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

Submitted by oakcobalt Accepted callers in a loan manager (e.g.

Pool.sol ) can be either liquidators or loan contracts.

And only loan contracts should validate offers during a loan creation. However, the current access control check is insufficient in pool::validateOffer, which allows liquidators to pretend to be a loan contract, and directly modify storage ( __outstandingValues ) bypassing additional checks and accounting in a loan contract.

//src/lib/pools/Pool.sol //@audit onlyAcceptedCallers only doesn't ensure caller is a loan contract |> function validateOffer ( bytes calldata _offer, uint256 _protocolFee ) external override onlyAcceptedCallers { if (!

isActive ) { revert PoolStatusError (); }...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/pools/Pool.sol#L392
Current access control check ( onlyAcceptedCallers ) only ensures caller is accepted caller but doesn’t verify the caller is a loan contract ( _isLoanContract(caller)``==true ).

//src/lib/loans/LoanManager.sol modifier onlyAcceptedCallers () { if (!

_acceptedCallers.

contains ( msg.

sender )) { revert CallerNotAccepted (); } _; } When a liquidator calls validateOffer, they can provide a fabricated bytes calldata _offer and uint256 _protocolFee bypassing additional checks and state accounting in a loan contract. For example, in MultiSourceLoan.sol- emitLoan, extra checks are implemented on LoanExecutionData to verify borrower and lender signatures and offer expiration timestamp as well as transfer collateral NFT tokens and recoding loan to storage. All of the above can be skipped if a liquidator directly call validateOffer and modify __outstandingValues without token transfer.

## Recommended Mitigation Steps

In Pool::validateOffer, consider adding a check to ensure _isLoanContract(msg.sender)``==true.

## Assessed type

Invalid Validation 0xend (Gondi) acknowledged and commented:

This is a low one given these contracts must all live within our ecosystem and be whitelisted. Given there’s already that trust assumption, not doing that check to save some gas on an extra state read.

0xA5DF (judge) commented:

This is a low one given these contracts must all live within our ecosystem and be whitelisted.

That makes sense. However, I’m judging this based on the info that was present to the wardens in the README and docs. Given that that trust assumption wasn’t noted there I’m going to sustain Medium severity.

Gondi mitigated:

Check loanContract.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-11] AuctionLoanLiquidator#placeBid can be DoS

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

AuctionLoanLiquidator#placeBid can be DoS Submitted by zhaojie, also found by bin2chen The attacker performs a DoS attack on the Bid function, causing other users to be unable to participate and eventually obtaining the NFT at a low price.

## Recommended Mitigation Steps

function placeBid(.....){ + require (_bid > MIN_BID); }

## Assessed type

DoS 0xA5DF (judge) commented:

Given that auction duration is 3 days front-running every single tx in that timeframe isn’t going to be easy for the attacker. Considering Medium.

0xend (Gondi) confirmed and commented:

We have a check that for an auction to be settled it also requires the last bid to be at least 10’ old (to avoid someone sniping at the very end). Given this, someone doing this attack, would actually have to continue going for an indiscriminate amount of time since honest players would be able to bid and extend it.

I think this is Low/Medium.

0xA5DF (judge) decreased severity to Medium and commented:

The attack seems very unlikely, but given that the impact can be quite high, I’m sustaining Medium severity.

0xend (Gondi) commented:

- https://github.com/pixeldaogg/florida-contracts/pull/377
Starting with a min based on principle to make sure it’s meaningful.

Gondi mitigated:

There’s a min bid now. This + the min improvement invalidates DoS.

Status:

Mitigation confirmed. Full details in reports from oakcobalt and minhquanym.

# [M-12] Pool.getMinTimeBetweenWithdrawalQueues current calculations may not be sufficient

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

Pool.getMinTimeBetweenWithdrawalQueues current calculations may not be sufficient Submitted by bin2chen getMinTimeBetweenWithdrawalQueues is very important for Pool. If getMinTimeBetweenWithdrawalQueues is too small, pendingQueues will be overwritten too early, and when Loan pays off, it won’t be able to find the corresponding queues.

So we will calculate getMinTimeBetweenWithdrawalQueues by MaxDuration + _LOAN_BUFFER_TIME to make sure it won’t be overwritten too early.

Currently:

_LOAN_BUFFER_TIME = 7 days Similarly:

LiquidationHandler.MAX_AUCTION_DURATION = 7 days So getMinTimeBetweenWithdrawalQueues is sufficient if the bidding is done within the time period. However, less consideration is given to the presence of AuctionLoanLiquidator._MIN_NO_ACTION_MARGIN and the bidding can be delayed.

function placeBid ( address _nftAddress, uint256 _tokenId, Auction memory _auction, uint256 _bid ) external nonReentrant returns ( Auction memory ) {...

uint256 currentTime = block.

timestamp; uint96 expiration = _auction.

startTime + _auction.

duration; uint96 withMargin = _auction.

lastBidTime + _MIN_NO_ACTION_MARGIN; @> uint96 max = withMargin > expiration ?

withMargin:

expiration; if ( max < currentTime && currentHighestBid > 0 ) { revert AuctionOverError ( max ); } If the bidding is intense, it may be delayed > _MIN_NO_ACTION_MARGIN =10 minutes, or even longer. So getMinTimeBetweenWithdrawalQueues may not be enough. Suggest adding an extra day:

MaxDuration + _LOAN_BUFFER_TIME + 3 days.

## Impact

getMinTimeBetweenWithdrawalQueues is not large enough causing pendingQueues to be overwritten prematurely.

## Recommended Mitigation

contract Pool is ERC4626, InputChecker, IPool, IPoolWithWithdrawalQueues, LoanManager, ReentrancyGuard {...

/// @dev Used in case loans might have a liquidation, then the extension is upper bounded by maxDuration + liq time.

- uint256 private constant _LOAN_BUFFER_TIME = 7 days; + uint256 private constant _LOAN_BUFFER_TIME = 10 days;

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Limit auction extensions.

Status:

Unmitigated. Full details in reports from bin2chen, and also included in the

# [M-13] confirmBaseInterestAllocator() change BaseInterestAllocator may pay large getReallocationBonus

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

confirmBaseInterestAllocator() change BaseInterestAllocator may pay large getReallocationBonus Submitted by bin2chen owner can submit getPendingBaseInterestAllocator first, and then anyone can enable it by confirmBaseInterestAllocator().

function confirmBaseInterestAllocator ( address _newBaseInterestAllocator ) external { address cachedAllocator = getBaseInterestAllocator; if ( cachedAllocator != address ( 0 )) { if ( getPendingBaseInterestAllocatorSetTime + UPDATE_WAITING_TIME > block.

timestamp ) { revert TooSoonError (); } if ( getPendingBaseInterestAllocator != _newBaseInterestAllocator ) { revert InvalidInputError (); } @> IBaseInterestAllocator ( cachedAllocator ).

transferAll (); asset.

approve ( cachedAllocator, 0 ); } asset.

approve ( _newBaseInterestAllocator, type ( uint256 ).

max ); getBaseInterestAllocator = _newBaseInterestAllocator; getPendingBaseInterestAllocator = address ( 0 ); getPendingBaseInterestAllocatorSetTime = type ( uint256 ).

max; emit BaseInterestAllocatorSet ( _newBaseInterestAllocator ); } The current logic is:

Take all the balance of the old BaseInterestAllocator and put it in Pool.

Change getBaseInterestAllocator to the new BaseInterestAllocator.

If the old BaseInterestAllocator already has a large balance, the balance of the Pool will increase dramatically. Subsequent users executing reallocate() will get a big bonus getReallocationBonus.

function reallocate () external nonReentrant returns ( uint256 ) { ( uint256 currentBalance, uint256 targetIdle ) = _reallocate (); uint256 delta = currentBalance > targetIdle ?

currentBalance - targetIdle:

targetIdle - currentBalance; @> uint256 shares = delta.

mulDivDown ( totalSupply * getReallocationBonus, totalAssets () * _BPS ); _mint ( msg.

sender, shares ); emit Reallocated ( delta, shares ); return shares; } Assuming old BaseInterestAllocator balance: 1 M:

shares = 1 M * (1 - optimalIdleRange.mid) * totalSupply * getReallocationBonus / totalAssets()

## Impact

After change, BaseInterestAllocator may pay large getReallocationBonus.

## Recommended Mitigation

Execute _reallocate() in the confirmBaseInterestAllocator() method without paying any getReallocationBonus.

function confirmBaseInterestAllocator(address _newBaseInterestAllocator) external { address cachedAllocator = getBaseInterestAllocator; if (cachedAllocator != address(0)) { if (getPendingBaseInterestAllocatorSetTime + UPDATE_WAITING_TIME > block.timestamp) { revert TooSoonError(); } if (getPendingBaseInterestAllocator != _newBaseInterestAllocator) { revert InvalidInputError(); } IBaseInterestAllocator(cachedAllocator).transferAll(); asset.approve(cachedAllocator, 0); } asset.approve(_newBaseInterestAllocator, type(uint256).max); getBaseInterestAllocator = _newBaseInterestAllocator; getPendingBaseInterestAllocator = address(0); getPendingBaseInterestAllocatorSetTime = type(uint256).max; + if (cachedAllocator != address(0)) {

+ _reallocate(); + } emit BaseInterestAllocatorSet(_newBaseInterestAllocator); }

## Assessed type

Context 0xend (Gondi) confirmed 0xA5DF (judge) decreased severity to Medium and commented:

Marking as Medium since it’ll only take the fee part (which is only a small percentage I guess). I guess also the allocator isn’t going to change very often.

Gondi mitigated:

Proactively reallocate (we got rid of the bonus though).

Status:

Mitigation confirmed. Full details in reports from oakcobalt and bin2chen.

# [M-14] loanLiquidation() calculation of interest is not accurate

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

loanLiquidation() calculation of interest is not accurate Submitted by bin2chen loanLiquidation(): The calculated interest codes are as follows： function loanLiquidation ( uint256 _loanId, uint256 _principalAmount, uint256 _apr, uint256, uint256 _protocolFee, uint256 _received, uint256 _startTime ) external override onlyAcceptedCallers { uint256 netApr = _netApr ( _apr, _protocolFee ); uint256 interestEarned = _principalAmount.

getInterest ( netApr, block.

timestamp - _startTime ); @> uint256 fees = IFeeManager ( getFeeManager ).

processFees ( _received, 0 ); getCollectedFees += fees; _loanTermination ( msg.

sender, _loanId, _principalAmount, netApr, interestEarned, _received - fees ); }...

contract FeeManager is IFeeManager, TwoStepOwned {...

function processFees ( uint256 _principal, uint256 _interest ) external view returns ( uint256 ) { /// @dev cached Fees memory __fees = _fees; return _principal.

mulDivDown ( __fees.

managementFee, PRECISION ) + _interest.

mulDivDown ( __fees.

performanceFee, PRECISION ); } The above code takes all of _received as the principal and gives it to IFeeManager to calculate. But the amount received from the bidding is not always less than the principal, it may be more than the principal, and this part should be calculated as interest.

## Impact

When received is greater than the principal, fees is not correct.

## Recommended Mitigation

function loanLiquidation( uint256 _loanId, uint256 _principalAmount, uint256 _apr, uint256, uint256 _protocolFee, uint256 _received, uint256 _startTime ) external override onlyAcceptedCallers { uint256 netApr = _netApr(_apr, _protocolFee); uint256 interestEarned = _principalAmount.getInterest(netApr, block.timestamp - _startTime); - uint256 fees = IFeeManager(getFeeManager).processFees(_received, 0); + uint256 fees; + if (_received > _principalAmount) { + fees = IFeeManager(getFeeManager).processFees(_principalAmount, _received - _principalAmount); + }else { + fees = IFeeManager(getFeeManager).processFees(_received, 0); + } getCollectedFees += fees; _loanTermination(msg.sender, _loanId, _principalAmount, netApr, interestEarned, _received - fees);

}

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Corrected calculation of fees as suggested.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-15] confirmUnderwriter() need to recalculate getMinTimeBetweenWithdrawalQueues

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

confirmUnderwriter() need to recalculate getMinTimeBetweenWithdrawalQueues Submitted by bin2chen getMinTimeBetweenWithdrawalQueues is very important for Pool. If getMinTimeBetweenWithdrawalQueues is too small, pendingQueues will be overwritten too early, and when Loan pays off, it won’t be able to find the corresponding queues.

So we will calculate getMinTimeBetweenWithdrawalQueues by MaxDuration + _LOAN_BUFFER_TIME to make sure it won’t be overwritten too early.

constructor ( address _feeManager, address _offerHandler, uint256 _waitingTimeBetweenUpdates, OptimalIdleRange memory _optimalIdleRange, uint256 _maxTotalWithdrawalQueues, uint256 _reallocationBonus, ERC20 _asset, string memory _name, string memory _symbol ) ERC4626 ( _asset, _name, _symbol ) LoanManager ( tx.

origin, _offerHandler, _waitingTimeBetweenUpdates ) {....

@> getMinTimeBetweenWithdrawalQueues = ( IPoolOfferHandler ( _offerHandler ).

getMaxDuration () + _LOAN_BUFFER_TIME ).

mulDivUp ( 1, _maxTotalWithdrawalQueues ); But switching the new getUnderwriter/_offerHandler doesn’t recalculate the getMinTimeBetweenWithdrawalQueues.

function confirmUnderwriter ( address __underwriter ) external onlyOwner { if ( getPendingUnderwriterSetTime + UPDATE_WAITING_TIME > block.

timestamp ) { revert TooSoonError (); } if ( getPendingUnderwriter != __underwriter ) { revert InvalidInputError (); } @> getUnderwriter = __underwriter; getPendingUnderwriter = address ( 0 ); getPendingUnderwriterSetTime = type ( uint256 ).

max; emit UnderwriterSet ( __underwriter ); } This may break the expectation of getMinTimeBetweenWithdrawalQueues, and the new getUnderwriter.getMaxDuration is larger than the old one; which may cause pendingQueues to be overwritten prematurely.

## Impact

The new getUnderwriter.getMaxDuration is larger than the old one, which may cause pendingQueues to be overwritten prematurely.

## Recommended Mitigation

Pool overrides confirmUnderwriter() with an additional recalculation of getMinTimeBetweenWithdrawalQueues and must not be smaller than the old one, to avoid premature overwriting of the previous one.

contract Pool is ERC4626, InputChecker, IPool, IPoolWithWithdrawalQueues, LoanManager, ReentrancyGuard { - uint256 public immutable getMinTimeBetweenWithdrawalQueues; + uint256 public getMinTimeBetweenWithdrawalQueues;...

+ function confirmUnderwriter(address __underwriter) external override onlyOwner { + super.confirmUnderwriter(__underwriter); + uint256 newMinTime = (IPoolOfferHandler(__underwriter).getMaxDuration() + _LOAN_BUFFER_TIME) +.mulDivUp(1, _maxTotalWithdrawalQueues); + require(newMinTime >= getMinTimeBetweenWithdrawalQueues,"invalid"); + getMinTimeBetweenWithdrawalQueues = newMinTime; + }

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Added check ( maxDuration cannot be longer).

Status:

Mitigation confirmed. Full details in reports from oakcobalt, bin2chen and minhquanym.

# [M-16] distribute() uses the wrong end time to break maxSeniorRepayment ’s expectations

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

distribute() uses the wrong end time to break maxSeniorRepayment ’s expectations Submitted by bin2chen When the bid amount is not enough, the lender will be repaid in order of tranche[]. In order to minimize the risk, the user can specify maxSeniorRepayment to avoid the risk to some extent, and put himself in a position of higher repayment priority. At the same time emitLoan() checks maxSeniorRepayment for the emitLoan().

emitLoan() -> _processOffersFromExecutionData() -> _checkOffer() function _processOffersFromExecutionData ( address _borrower, address _principalReceiver, address _principalAddress, address _nftCollateralAddress, uint256 _tokenId, uint256 _duration, OfferExecution [] calldata _offerExecution ) private returns ( uint256, uint256 [] memory, Loan memory, uint256 ) {...

uint256 amount = thisOfferExecution.

amount; address lender = offer.

lender; /// @dev Please note that we can now have many tranches with same `loanId`.

tranche [ i ] = Tranche ( loanId, totalAmount, amount, lender, 0, block.

timestamp, offer.

aprBps ); totalAmount += amount; @> totalAmountWithMaxInterest += amount + amount.

getInterest ( offer.

aprBps, _duration );...

function _checkOffer ( LoanOffer calldata _offer, address _principalAddress, address _nftCollateralAddress, uint256 _amountWithInterestAhead ) private pure { if ( _offer.

principalAddress != _principalAddress || _offer.

nftCollateralAddress != _nftCollateralAddress ) { revert InvalidAddressesError (); } @> if ( _amountWithInterestAhead > _offer.

maxSeniorRepayment ) { revert InvalidTrancheError (); } totalAmountWithMaxInterest is computed using loan._duration. But when the bidding ends and the distribution is done in LiquidationDistributor.distribute(), the current time is used to calculate Interest.

function distribute ( uint256 _proceeds, IMultiSourceLoan.Loan calldata _loan ) external { uint256 [] memory owedPerTranche = new uint256 []( _loan.

tranche.

length ); uint256 totalPrincipalAndPaidInterestOwed = _loan.

principalAmount; uint256 totalPendingInterestOwed = 0; for ( uint256 i = 0; i < _loan.

tranche.

length;) { IMultiSourceLoan.

Tranche calldata thisTranche = _loan.

tranche [ i ]; uint256 pendingInterest = @> thisTranche.

principalAmount.

getInterest ( thisTranche.

aprBps, block.

timestamp - thisTranche.

startTime ); totalPrincipalAndPaidInterestOwed += thisTranche.

accruedInterest; totalPendingInterestOwed += pendingInterest; owedPerTranche [ i ] += thisTranche.

principalAmount + thisTranche.

accruedInterest + pendingInterest; unchecked { ++ i; } Because bidding takes a certain amount of time ( ~3-7 days ), using block.timestamp - thisTranche.startTime will be larger than expected! Correctly should use: ( loan.startTime + loan.duration - thisTranche.startTime ) to calculate the interest.

This leads to the problem that if there are not enough funds, the front lender will get a larger repayment than expected, breaking the back lender ’s initial expectation of maxSeniorRepayment.

## Impact

If there are not enough funds, the initial expectation of maxSeniorRepayment may be broken.

## Recommended Mitigation

function distribute(uint256 _proceeds, IMultiSourceLoan.Loan calldata _loan) external { uint256[] memory owedPerTranche = new uint256[](_loan.tranche.length); uint256 totalPrincipalAndPaidInterestOwed = _loan.principalAmount; uint256 totalPendingInterestOwed = 0; + uint256 loanExpireTime = _loan.startTime + _loan.duration; for (uint256 i = 0; i < _loan.tranche.length;) { IMultiSourceLoan.Tranche calldata thisTranche = _loan.tranche[i]; uint256 pendingInterest = - thisTranche.principalAmount.getInterest(thisTranche.aprBps, block.timestamp - thisTranche.startTime); + thisTranche.principalAmount.getInterest(thisTranche.aprBps, loanExpireTime - thisTranche.startTime); totalPrincipalAndPaidInterestOwed += thisTranche.accruedInterest;

totalPendingInterestOwed += pendingInterest; owedPerTranche[i] += thisTranche.principalAmount + thisTranche.accruedInterest + pendingInterest; unchecked { ++i; }

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Changed to loan end time (instead of current timestamp).

Status:

Mitigation confirmed. Full details in reports from oakcobalt, bin2chen and minhquanym.

# [M-17] loan.hash() does not contain protocolFee

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

loan.hash() does not contain protocolFee Submitted by bin2chen, also found by oakcobalt and minhquanym The current IMultiSourceLoan.loop.hash() does not contain protocolFee:

function emitLoan ( LoanExecutionData calldata _loanExecutionData ) external nonReentrant returns ( uint256, Loan memory ) {...

@> _loans [ loanId ] = loan.

hash (); emit LoanEmitted ( loanId, offerIds, loan, totalFee ); return ( loanId, loan ); } function hash (IMultiSourceLoan.Loan memory _loan ) internal pure returns ( bytes32 ) { bytes memory trancheHashes; for ( uint256 i; i < _loan.

tranche.

length;) { trancheHashes = abi.

encodePacked ( trancheHashes, _hashTranche ( _loan.

tranche [ i ])); unchecked { ++ i; } return keccak256 ( abi.

encode ( _MULTI_SOURCE_LOAN_HASH, _loan.

borrower, _loan.

nftCollateralTokenId, _loan.

nftCollateralAddress, _loan.

principalAddress, _loan.

principalAmount, _loan.

startTime, _loan.

duration, @> //@audit miss protocolFee keccak256 ( trancheHashes ) ); } struct Loan { address borrower; uint256 nftCollateralTokenId; address nftCollateralAddress; address principalAddress; uint256 principalAmount; uint256 startTime; uint256 duration; Tranche [] tranche; @> uint256 protocolFee; } Then, you can specify protocolFee arbitrarily in many methods, but the _baseLoanChecks() security check doesn’t revert.

function _baseLoanChecks ( uint256 _loanId, Loan memory _loan ) private view { @> if ( _loan.

hash () != _loans [ _loanId ]) { revert InvalidLoanError ( _loanId ); } if ( _loan.

startTime + _loan.

duration < block.

timestamp ) { revert LoanExpiredError (); } Example:

repayLoan(loadn.protocolFee=0) to escape fees and cause a LoanManager accounting error.

refinancePartial()/refinanceFull() can also specify the wrong fees to skip the fees.

## Impact

The loan hash does not contain a protocolFee, leading to an arbitrary protocolFee that can be specified to escape fees or cause an accounting error.

## Recommended Mitigation

function hash(IMultiSourceLoan.Loan memory _loan) internal pure returns (bytes32) { bytes memory trancheHashes; for (uint256 i; i < _loan.tranche.length;) { trancheHashes = abi.encodePacked(trancheHashes, _hashTranche(_loan.tranche[i])); unchecked { ++i; } return keccak256( abi.encode( _MULTI_SOURCE_LOAN_HASH, _loan.borrower, _loan.nftCollateralTokenId, _loan.nftCollateralAddress, _loan.principalAddress, _loan.principalAmount, _loan.startTime, _loan.duration, keccak256(trancheHashes), + _loan.protocolFee ) ); }

## Assessed type

Context 0xend (Gondi) confirmed 0xA5DF (judge) decreased severity to Medium and commented:

Avoiding fees is just a Medium. For the accounting error, I’ll need more proof that this can lead to something significant to mark this as High.

Gondi mitigated:

Added field in hash.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-18] distribute() when can’t repay all lenders, may lack of notification to LoanManager for accounting

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

distribute() when can’t repay all lenders, may lack of notification to LoanManager for accounting Submitted by bin2chen The LiquidationDistributor is used to distribute funds after an auction. When the auction amount is insufficient, lenders are repaid in sequence.

function distribute ( uint256 _proceeds, IMultiSourceLoan.Loan calldata _loan ) external {...

if ( _proceeds > totalPrincipalAndPaidInterestOwed + totalPendingInterestOwed ) { for ( uint256 i = 0; i < _loan.

tranche.

length;) { IMultiSourceLoan.

Tranche calldata thisTranche = _loan.

tranche [ i ]; _handleTrancheExcess ( _loan.

principalAddress, thisTranche, msg.

sender, _proceeds, totalPrincipalAndPaidInterestOwed + totalPendingInterestOwed ); unchecked { ++ i; } else { @> for ( uint256 i = 0; i < _loan.

tranche.

length && _proceeds > 0;) { IMultiSourceLoan.

Tranche calldata thisTranche = _loan.

tranche [ i ]; _proceeds = _handleTrancheInsufficient ( _loan.

principalAddress, thisTranche, msg.

sender, _proceeds, owedPerTranche [ i ] ); unchecked { ++ i; } The code snippet above introduces a condition _proceeds > 0 to terminate the loop when there’s no remaining balance in _proceeds, thereby preventing further execution of _handleTrancheInsufficient().

However, this approach creates an issue. If the subsequent lender is a LoanManager, it won’t be notified for accounting via _handleTrancheInsufficient() -> _handleLoanManagerCall() -> LoanManager(_tranche.lender).loanLiquidation().

Although no funds can be repaid, accounting is still necessary to notice and prevent incorrect accounting, causing inaccuracies in totalAssets() and continued accumulation of interest. This outstanding debt should be shared among current users and prevent it from persisting as bad debt.

## Impact

Failure to notify LoanManager.loanLiquidation() may result in accounting inaccuracies.

## Recommended Mitigation

function distribute(uint256 _proceeds, IMultiSourceLoan.Loan calldata _loan) external {...

} else { - for (uint256 i = 0; i < _loan.tranche.length && _proceeds > 0;) { + for (uint256 i = 0; i < _loan.tranche.length;) { IMultiSourceLoan.Tranche calldata thisTranche = _loan.tranche[i]; _proceeds = _handleTrancheInsufficient( _loan.principalAddress, thisTranche, msg.sender, _proceeds, owedPerTranche[i] ); unchecked { ++i; }

## Assessed type

Context 0xend (Gondi) confirmed Gondi mitigated:

Always call loanManager (even if 0 proceeds).

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-19] Bidders might lose funds due to possible racing condition between settleWithBuyout and placeBid

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

settleWithBuyout and placeBid Submitted by oakcobalt, also found by oakcobalt In AuctionWithBuyoutLoanLiquidator.sol, settleWithBuyout and placeBid are allowed at an overlapping timestamp ( _auction.startTime + _timeForMainLenderToBuy ). This allows settleWithBuyout and placeBid to be settled at the same block.

When placeBid tx settles at _auction.startTime + _timeForMainLenderToBuy before settleWithBuyout tx, the bidder will lose their funds. Because settleWithBuyout will always assume no bids are placed, it will directly transfer out the collateral NFT token and delete the auction data from storage.

function settleWithBuyout ( address _nftAddress, uint256 _tokenId, Auction calldata _auction, IMultiSourceLoan.Loan calldata _loan ) external nonReentrant {...

uint256 timeLimit = _auction.

startTime + _timeForMainLenderToBuy; |> if ( timeLimit < block.

timestamp ) { revert OptionToBuyExpiredError ( timeLimit ); }...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/AuctionWithBuyoutLoanLiquidator.sol#L63C1-L66C10
function _placeBidChecks ( address _nftAddress, uint256 _tokenId, Auction memory _auction, uint256 _bid ) internal view override {...

uint256 timeLimit = _auction.

startTime + _timeForMainLenderToBuy; |> if ( timeLimit > block.

timestamp ) { revert OptionToBuyStilValidError ( timeLimit ); }

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/AuctionWithBuyoutLoanLiquidator.sol#L129

## Recommended Mitigation Steps

Consider to only allow buyout strictly before the timeLimit if (timeLimit <= block.timestamp) {//revert.

0xend (Gondi) confirmed Gondi mitigated:

Strict to >=.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

# [M-20] Hardcoded incorrect getLidoData timestamp, resulting in incorrect base point Apr. Loans can be validated with a substantially low baseRate interest

- **Contest:** Gondi Invitational
- **Slug:** 2024-04-gondi-invitational
- **Finding ID:** M-20
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-gondi-invitational
- **Source snapshot:** competitions/2024-04-gondi-invitational/final_report.html

getLidoData timestamp, resulting in incorrect base point Apr. Loans can be validated with a substantially low baseRate interest Submitted by oakcobalt In LidoEthBaseInterestAllocator.sol, getLidoData is initialized with an incorrect timestamp, causing subsequent baseApr to be incorrect.

In constructor, getLidoData is initialized with 0 timestamp. This should be block.timestamp instead. As a result, baseApr updates will be much lower:

//src/lib/pools/LidoEthBaseInterestAllocator.sol struct LidoData { uint96 lastTs; uint144 shareRate; uint16 aprBps; }...

constructor ( address _pool, address payable __curvePool, address payable __weth, address __lido, uint256 _currentBaseAprBps, uint96 _lidoUpdateTolerance ) Owned ( tx.

origin ) {...

//@audit 0-> block.timestamp |> getLidoData = LidoData ( 0, uint144 ( _currentShareRate ()), uint16 ( _currentBaseAprBps ));...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/pools/LidoEthBaseInterestAllocator.sol#L62
For example, in _updateLidoValue(), _lidoData.aprBps is calculated based on delta shareRate, divided by delta timespan. ( _BPS * _SECONDS_PER_YEAR * (shareRate - _lidoData.shareRate) / _lidoData.shareRate/ (block.timestamp - _lidoData.lastTs) ) shareRate and _lidoData.shareRate are associated with current timestamp, and deployment timestamp respectively. But timespan would be ( block.timestamp - 0 ). This deflated _lidoData.aprBps value, which is used to validate Loan offers in Pool.sol during loan initiation.

In PoolOfferHandler.sol, this allows loan offers with substantially low aprBps to pass the minimal apr check.

//src/lib/pools/PoolOfferHandler.sol function validateOffer ( uint256 _baseRate, bytes calldata _offer ) external view override returns ( uint256 principalAmount, uint256 aprBps ) {...

if ( offerExecution.

offer.

aprBps < _baseRate + aprPremium || aprPremium == 0 ) { revert InvalidAprError (); }...

- https://github.com/code-423n4/2024-04-gondi/blob/b9863d73c08fcdd2337dc80a8b5e0917e18b036c/src/lib/pools/PoolOfferHandler.sol#L165
Loans with invalid aprs can be created.

## Recommended Mitigation Steps

Use block.timestamp to initialize getLidoData.

## Assessed type

Error 0xend (Gondi) confirmed 0xA5DF (judge) decreased severity to Medium and commented:

Marking as Medium since this only impacts interest and it’s limited to the initial phase of the contract.

Gondi mitigated:

Set right value for getLidoData timestamp.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, minhquanym and bin2chen.

## Rejected Primary Findings

# Rejected Primary Findings: Gondi Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
