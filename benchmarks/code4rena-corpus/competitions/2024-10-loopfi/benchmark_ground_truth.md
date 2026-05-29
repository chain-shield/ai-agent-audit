# Benchmark Ground Truth: LoopFi

## Accepted H/M Findings

# Accepted H/M Findings: LoopFi

# [H-01] Rewards might be lost due to the error that _updateRewardIndex() might advance lastBalance without advancing index for a token

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-loopfi
- **Source snapshot:** competitions/2024-10-loopfi/final_report.html

_updateRewardIndex() might advance lastBalance without advancing index for a token Submitted by chaduke, also found by Evo The function _updateRewardIndex() is used to update the lastBalance and index of each reward token. This function will be called when a user deposits, withdraws collateral or claims rewards.

However, the function might not advance index when accrued.divDown(totalShares) = 0. This might happen when totalShares is too big and accrued is too small. One case is that the number of decimals for the reward token is too small.

- https://github.com/code-423n4/2024-10-loopfi/blob/d219f0132005b00a68f505edc22b34f9a8b49766/src/pendle-rewards/RewardManager.sol#L74
For example, the USDC token only has 6 decimals.

Suppose accrued = $100 = 100*10**6, and totalShares = 200M = 200 * 10** 6 * 10**18; then we have accrued.divDown(totalShares) = 0.

Furthermore, if function _updateRewardIndex() is called more frequently, either because a malicious user keeps calling getRewards() (the gas fee is low on Arbitrum) or simply because the community is large so there is a high chance that for each block (per 12 seconds on Ethereum), there is someone who calls a withdraw / deposit / getRewards function. As a result, accrued could be small, leading to accrued.divDown(totalShares) = 0. Meanwhile, _updateRewardIndex() always advances lastBalance when accrued !=0:

- https://github.com/code-423n4/2024-10-loopfi/blob/d219f0132005b00a68f505edc22b34f9a8b49766/src/pendle-rewards/RewardManager.sol#L78
This means the accrued rewards are lost! Nobody will receive the rewards since index has not changed.

More importantly, due to the rounding down error for accrued.divDown(totalShares), there is always a slight loss for the rewards, which is accumulative over time.

## Recommended Mitigation Steps

The fix is simple. Calculate deltaIndex = accrued.divDown(totalShares) and advance lastBalance by deltaIndex.mulDown(totalShares). In this way, index and lastBalance will always advance in the same pace; in particular if index does not advance, then lastBalance will not advance either. The rounding down error is eliminated too since the lastBalance will not be accrued but by deltaIndex.mulDown(totalShares).

## Assessed type

Math 0xtj24 (LoopFi) confirmed 0xAlix2 (warden) commented:

@Koolex - I agree that this is an issue; however, the audit docs states that the ERC20s that are used by the protocol are WETH and PendleLPs which are both 18 decimals.

ERC20 used by the protocol | WETH, PendleLPs But I’m not sure if that should be considered valid in this context.

Koolex (judge) commented:

There is another issue here.

malicious user keeps calling getRewards()

# [H-02] CDPVault.sol#liquidatePositionBadDebt() doesn’t correctly handle profit and loss

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-loopfi
- **Source snapshot:** competitions/2024-10-loopfi/final_report.html

CDPVault.sol#liquidatePositionBadDebt() doesn’t correctly handle profit and loss Submitted by pkqs90, also found by 0xAlix2

- https://github.com/code-423n4/2024-10-loopfi/blob/main/src/CDPVault.sol#L702
- https://github.com/code-423n4/2024-10-loopfi/blob/main/src/PoolV3.sol#L593

## Impact

When liquidating bad debt, the profit and loss is not correctly handled. This will cause incorrect accounting to lpETH stakers.

Bug Description Note: This is based on the 2024-07 Loopfi audit H-12 issue. This protocol team applied a fix, but the fix is incomplete.

There are two issues that needs to be fixed in the new codebase:

The profit that is passed in pool.repayCreditAccount(debtData.debt, profit, loss); should actually use debtData.accruedInterest. This is because we should first “assume” full debt and interest is paid off, and calculate the loss part independently.

The loss is correctly calculated in PoolV3#repayCreditAccount, but the if-else branch is incorrectly implemented. Currently, it can’t handle the case where both profit and loss is non-zero. This would cause a issue that the loss will not be accounted, and will ultimately cause loss to lpETH holders (loss will be implicitly added to the users who hold lpETH) instead of lpETH stakers.

The second fix was also suggested in the original issue, but it isn’t applied.

CDPVault.sol:

takeCollateral = position.

collateral; repayAmount = wmul ( takeCollateral, discountedPrice ); uint256 loss = calcTotalDebt ( debtData ) - repayAmount; uint256 profit; if ( repayAmount > debtData.

debt ) { @> profit = repayAmount - debtData.

debt; }...

@> pool.

repayCreditAccount ( debtData.

debt, profit, loss ); // U:[CM-11] // transfer the collateral amount from the vault to the liquidator token.

safeTransfer ( msg.

sender, takeCollateral ); PoolV3.sol:

function repayCreditAccount ( uint256 repaidAmount, uint256 profit, uint256 loss ) external override creditManagerOnly // U:[LP-2C] whenNotPaused // U:[LP-2A] nonReentrant // U:[LP-2B] {...

if ( profit > 0 ) { _mint ( treasury, _convertToShares ( profit )); // U:[LP-14B] @> } else if ( loss > 0 ) { address treasury_ = treasury; uint256 sharesInTreasury = balanceOf ( treasury_ ); uint256 sharesToBurn = _convertToShares ( loss ); if ( sharesToBurn > sharesInTreasury ) { unchecked { emit IncurUncoveredLoss ({ creditManager:

msg.

sender, loss:

_convertToAssets ( sharesToBurn - sharesInTreasury ) }); // U:[LP-14D] } sharesToBurn = sharesInTreasury; } _burn ( treasury_, sharesToBurn ); // U:[LP-14C,14D] }...

}

## Recommended Mitigation Steps

In CDPVault, change to pool.repayCreditAccount(debtData.debt, debtData.accruedInterest, loss).

In PoolV3:

if ( profit > 0 ) { _mint ( treasury, convertToShares ( profit )); // U:[LP-14B] + } + if ( loss > 0 ) - } else if ( loss > 0 ) {...

} 0xtj24 (LoopFi) confirmed Koolex (judge) commented:

Why Profit should be debtData.accruedInterest ?

For the second part, could you please provide a case where profit and loss are non-zero in PJQA?

pkqs90 (warden) commented:

@Koolex - Here’s an example scenario:

User originally taken out a debt of 100, and interest grows to 50, so debtData.debt = 100, debtData.accruedInterest = 50, calcTotalDebt(debtData) = 150).

User collateral is only 100, and after multiplying discountPrice, the repayAmount is only 90. Bad debt occurs.

loss = calcTotalDebt(debtData) - repayAmount is equal to 150 - 90 = 60.

Since repayAmount < debtData.debt, we would have profit = 0.

This means for PoolV3#repayCreditAccount, 60 shares would be burned from the treasury, while instead it should be 10 (because original debt was 100, repaid is 90, 100 - 90 = 10 ).

You can also see that if repayAmount was 101, we would calculate profit = 1, and in PoolV3#repayCreditAccount we would mint 1 share instead. This means there is a 61 ( 1 - (-60) = 61 ) gap in treasury shares when the repaid amount diff is only 11 ( 101 - 90 = 11 ), which does not make any sense.

function calcTotalDebt ( DebtData memory debtData ) internal pure returns ( uint256 ) { return debtData.

debt + debtData.

accruedInterest; //+ debtData.accruedFees; } function liquidatePositionBadDebt ( address owner, uint256 repayAmount ) external whenNotPaused {...

takeCollateral = position.

collateral; repayAmount = wmul ( takeCollateral, discountedPrice ); @> uint256 loss = calcTotalDebt ( debtData ) - repayAmount; uint256 profit; if ( repayAmount > debtData.

debt ) { @> profit = repayAmount - debtData.

debt; }...

@> pool.

repayCreditAccount ( debtData.

debt, profit, loss ); // U:[CM-11] // transfer the collateral amount from the vault to the liquidator token.

safeTransfer ( msg.

sender, takeCollateral ); } PoolV3.sol:

function repayCreditAccount ( uint256 repaidAmount, uint256 profit, uint256 loss ) external override creditManagerOnly // U:[LP-2C] whenNotPaused // U:[LP-2A] nonReentrant // U:[LP-2B] { uint128 repaidAmountU128 = repaidAmount.

toUint128 (); DebtParams storage cmDebt = _creditManagerDebt [ msg.

sender ]; uint128 cmBorrowed = cmDebt.

borrowed; if ( cmBorrowed == 0 ) { revert CallerNotCreditManagerException (); // U:[LP-2C,14A] } if ( profit > 0 ) { _mint ( treasury, _convertToShares ( profit )); // U:[LP-14B] } else if ( loss > 0 ) { address treasury_ = treasury; uint256 sharesInTreasury = balanceOf ( treasury_ ); uint256 sharesToBurn = _convertToShares ( loss ); if ( sharesToBurn > sharesInTreasury ) { unchecked { emit IncurUncoveredLoss ({ creditManager:

msg.

sender, loss:

_convertToAssets ( sharesToBurn - sharesInTreasury ) }); // U:[LP-14D] } sharesToBurn = sharesInTreasury; } _burn ( treasury_, sharesToBurn ); // U:[LP-14C,14D] }...

} Koolex (judge) commented:

@pkqs90 - Could you please point out the incomplete fix? This is important, since if there is no indication that the sponsor intended to fix it, it would be out of scope (according to this announcement ).

pkqs90 (warden) commented:

@Koolex - The 2024-07 code had pool.repayCreditAccount(debtData.debt, 0, loss);

- https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L624, and was later fixed to pool.repayCreditAccount(debtData.debt, profit, loss);

- https://github.com/code-423n4/2024-10-loopfi/blob/main/src/CDPVault.sol#L702.

The suggested fix was also mentioned the original report for H-12.

Medium Risk Findings (5)

# [M-01] Invalid handling of flash loan fees in PositionAction::onCreditFlashLoan , forcing it to always revert

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-loopfi
- **Source snapshot:** competitions/2024-10-loopfi/final_report.html

PositionAction::onCreditFlashLoan, forcing it to always revert Submitted by 0xAlix2, also found by pkqs90 ( 1, 2 ) When users take a flash loan, an amount is sent to the receiver, and then some action takes place, after that action that sent amount is expected to be paid and some fee. Users can also call PositionAction::decreaseLever through a proxy, to “Decrease the leverage of a position by taking out a credit flash loan to withdraw and sell collateral”, after it is called, the flash loan lender sends the credit and calls onCreditFlashLoan, which handles all that logic.

This was reported in Issue 524, and a fix has been implemented. However, the fix is incomplete, and the onCreditFlashLoan will always revert when the fees are >0.

The fix added includes adding fees to the approval amounts:

underlyingToken.

forceApprove ( address ( leverParams.

vault ), subDebt + fee ); underlyingToken.

forceApprove ( address ( flashlender ), subDebt + fee ); That fix still misses a point; the amount coming from the flash lender is constant, and that amount will be used to repay a position’s debt. The issue here is that all the amount is being used to repay without accounting for the extra fees that the flash lender will be requesting.

This causes PositionAction::onCreditFlashLoan to always revert.

## Recommended Mitigation Steps

function onCreditFlashLoan( address /*initiator*/, uint256 /*amount*/, uint256 fee, bytes calldata data ) external returns (bytes32) {...

// sub collateral and debt ICDPVault(leverParams.vault).modifyCollateralAndDebt( leverParams.position, address(this), 0, - -toInt256(subDebt) + -toInt256(subDebt - fee) );...

return CALLBACK_SUCCESS_CREDIT; }

## Assessed type

DoS amarcu (LoopFi) confirmed

# [M-02] Invalid handling of risdual amount in PositionAction::onCreditFlashLoan , forcing it to revert

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-loopfi
- **Source snapshot:** competitions/2024-10-loopfi/final_report.html

PositionAction::onCreditFlashLoan, forcing it to revert Submitted by 0xAlix2 Users can call PositionAction::decreaseLever through a proxy, to “Decrease the leverage of a position by taking out a credit flash loan to withdraw and sell collateral”. After it is called, the flash loan lender sends the credit and calls onCreditFlashLoan, which handles all that logic. When doing so, users are supposed to swap their collateral withdrawn into debt tokens so that the flash loan can be repaid.

The protocol tries to handle the residual amount from the swap ( swapped - paid debt ), by trying to repay extra debt for the designated position, using:

if ( residualAmount > 0 ) { underlyingToken.

forceApprove ( address ( leverParams.

vault ), residualAmount ); ICDPVault ( leverParams.

vault ).

modifyCollateralAndDebt ( leverParams.

position, address ( this ), address ( this ), 0, - toInt256 ( residualAmount ) ); } However, this is invalid for 2 main reasons:

This is trying to repay extra debt than what the user is trying to, which is passed in the primarySwap.amount.

If the user tries to repay his whole debt using decreaseLever the TX will revert, as it’ll try to repay some nonexistent debt.

## Recommended Mitigation Steps

Rather than using the residual amount to repay excess debt (that might not even exist), transfer it to the designated residual recipient. Alternatively, check if the user still has any remaining debt. If they do, use the residual to repay it; otherwise, transfer the residual amount to the recipient.

## Assessed type

DoS amarcu (LoopFi) confirmed and commented:

The error in the failing test is because of a faulty setup. This will happen if the flashlender contract is not added to the poolv3 as a credit manager.

0xAlix2 (warden) commented:

@Koolex - I believe there’s confusion here, the error here isn’t because “the flashlender contract is not added to the poolv3 as a credit manager”, please let me explain:

In PoolV3.sol, if you search for CallerNotCreditManagerException you can find 2 occurrences, the first is here:

function _revertIfCallerNotCreditManager () internal view { if (!

_creditManagerSet.

contains ( msg.

sender )) { revert CallerNotCreditManagerException (); // U:[PQK-4] } This is indeed the case that the sponsor is referencing; however, there’s another occurrence here:

if ( cmBorrowed == 0 ) { revert CallerNotCreditManagerException (); // U:[LP-2C,14A] } This is the case that the report is discussing, where this error is thrown when the borrowed of a position equals 0.

You can easily confirm this by changing the amount passed to amount in the above PoC to a lower value and see that the test doesn’t fail, example below:

function test_leverageDownWrongResidualHandling() public { uint256 depositAmount = 1_000 ether; uint256 borrowAmount = 200 ether; deal(address(token), user, depositAmount); address[] memory assets = new address[](2); assets[0] = address(token); assets[1] = address(underlyingToken); vm.startPrank(user); // User deposits 1k ETH collateral token.approve(address(vault), type(uint256).max); vault.deposit(address(userProxy), depositAmount); // User borrows 200 ETH userProxy.execute( address(positionAction), abi.encodeWithSelector( positionAction.borrow.selector, address(userProxy), address(vault), CreditParams({amount: borrowAmount, creditor: user, auxSwap: emptySwap}) ) ); userProxy.execute( address(positionAction),

abi.encodeWithSelector( positionAction.decreaseLever.selector, LeverParams({ position: address(userProxy), vault: address(vault), collateralToken: address(token), primarySwap: SwapParams({ swapProtocol: SwapProtocol.BALANCER, swapType: SwapType.EXACT_IN, assetIn: address(token), - amount: vault.virtualDebt(address(userProxy)), + amount: vault.virtualDebt(address(userProxy)) / 2, limit: 0, recipient: address(positionAction), residualRecipient: address(positionAction), deadline: block.timestamp, args: abi.encode(weightedPoolIdArray, assets) }), auxSwap: emptySwap, auxAction: emptyPoolActionParams }), 201 ether, address(userProxy) ) ); vm.stopPrank(); } Hence, I believe there’s some confusion here and this is a valid medium and would appreciate if you could take another look.

Koolex (judge) commented:

@amarcu - I have run the PoC and changed the error:

if (cmBorrowed == 0) { revert CallerNotCreditManagerException(); // U:[LP-2C,14A] } to other a different one. When running the PoC, it throws this error. This confirms the error is caused when there is no debt left cmBorrowed == 0.

# [M-03] PositionAction4626.sol#_onWithdraw should withdraw from position CDPVault position instead of address(this)

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-loopfi
- **Source snapshot:** competitions/2024-10-loopfi/final_report.html

PositionAction4626.sol#_onWithdraw should withdraw from position CDPVault position instead of address(this) Submitted by pkqs90 Note: This is based on the 2024-07 Loopfi audit M-35 issue. This protocol team applied a fix, but the fix is incomplete.

Only the bug in the _onDeposit() was fixed, but not the one in _onWithdraw().

PositionAction4626.sol#_onWithdraw does not withdraw from the correct position, it should withdraw from position instead of address(this).

function _onDeposit ( address vault, address position, address src, uint256 amount ) internal override returns ( uint256 ) { address collateral = address ( ICDPVault ( vault ).

token ()); // if the src is not the collateralToken, we need to deposit the underlying into the ERC4626 vault if ( src != collateral ) { address underlying = IERC4626 ( collateral ).

asset (); IERC20 ( underlying ).

forceApprove ( collateral, amount ); amount = IERC4626 ( collateral ).

deposit ( amount, address ( this )); } IERC20 ( collateral ).

forceApprove ( vault, amount ); // @audit-note: This was fixed.

return ICDPVault ( vault ).

deposit ( position, amount ); } function _onWithdraw ( address vault, address /*position*/, address dst, uint256 amount ) internal override returns ( uint256 ) { // @audit-note: This is still a bug.

@> uint256 collateralWithdrawn = ICDPVault ( vault ).

withdraw ( address ( this ), amount ); // if collateral is not the dst token, we need to withdraw the underlying from the ERC4626 vault address collateral = address ( ICDPVault ( vault ).

token ()); if ( dst != collateral ) { collateralWithdrawn = IERC4626 ( collateral ).

redeem ( collateralWithdrawn, address ( this ), address ( this )); } return collateralWithdrawn; }

## Recommended Mitigation Steps

- uint256 collateralWithdrawn = ICDPVault(vault).withdraw(address(this), amount); + uint256 collateralWithdrawn = ICDPVault(vault).withdraw(position, amount); amarcu (LoopFi) confirmed

# [M-04] PositionActionPendle.sol#_onWithdraw does not have slippage parameter minOut set

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-loopfi
- **Source snapshot:** competitions/2024-10-loopfi/final_report.html

PositionActionPendle.sol#_onWithdraw does not have slippage parameter minOut set Submitted by pkqs90, also found by ZanyBonzy and Bauchibred When performing withdraws on PositionActionPendle and exiting Pendle pools, users may lose funds due to not setting slippage.

Bug Description Note: This is a new issue that was introduced by the latest code diff.

The dataflow for withdrawing on PositionActionPendle is:

User withdraws collateral (which is a Pendle token) from CDPVault.

User performs pendle pool exit.

The issue is in step 2; since minOut is set to 0, users may receive less output tokens than expected.

function _onWithdraw ( address vault, address position, address dst, uint256 amount ) internal override returns ( uint256 ) { uint256 collateralWithdrawn = ICDPVault ( vault ).

withdraw ( address ( position ), amount ); address collateralToken = address ( ICDPVault ( vault ).

token ()); if ( dst != collateralToken && dst != address ( 0 )) { PoolActionParams memory poolActionParams = PoolActionParams ({ protocol:

Protocol.

PENDLE, @> minOut:

0, // @audit-bug: No slippage.

recipient:

address ( this ), args:

abi.

encode ( collateralToken, collateralWithdrawn, dst ) }); bytes memory exitData = _delegateCall ( address ( poolAction ), abi.

encodeWithSelector ( poolAction.

exit.

selector, poolActionParams ) ); collateralWithdrawn = abi.

decode ( exitData, ( uint256 )); } return collateralWithdrawn; } Also note that this is similar to the 2024-07 Loopfi audit finding M-39, which also talks about slippage in ERC4626. However, this Pendle withdraw exit pool code is new, and not existant in the last audit. Thus this should be considered a new bug.

## Recommended Mitigation Steps

Allow user to set a minOut parameter for withdraw functions, especially for Pendle position and ERC4626 position.

amarcu (LoopFi) confirmed

# [M-05] PositionAction.sol#onCreditFlashLoan may end up with stuck funds for EXACT_IN primary swaps Disclosures Overview About C4 Code4rena (C4) is an open organization consisting of security researchers, auditors, developers, and individuals with domain expertise in smart contracts.

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-loopfi
- **Source snapshot:** competitions/2024-10-loopfi/final_report.html

PositionAction.sol#onCreditFlashLoan may end up with stuck funds for EXACT_IN primary swaps Submitted by pkqs90 Note: This is a new issue that was introduced by the latest code diff.

When conducting a decreaseLever action, the final swap inside onCreditFlashLoan() is from collateral token to debt token. If the swap is EXACT_IN type, this means all collateral token is used for the swap. The output tokens are then split to two parts:

Repay the flashloan (and fees).

Send back to CDPVault to repay debt.

However, the second part have some issues. Mainly because if the repaid amount is larger than debt amount, the repayed amount will be capped to the debt amount (See CDPVault.sol code below). This means there may be some debt tokens ending up dangling in the PositionAction.sol contract, which the user does not have access to.

To explain a bit more, this is a reasonable scenario, because the amount of collateral tokens used for swap comes from uint256 withdrawnCollateral = _onDecreaseLever(leverParams, subCollateral);, and for PositionAction4626, _onDecreaseLever() supports gathering collateral tokens by exiting from pools (e.g., Balancer). This means it is totally possible that the amount of collateral tokens used for swap values more than the user’s debt in CDPVault.

PositionAction.sol:

function onCreditFlashLoan ( address /*initiator*/, uint256 /*amount*/, uint256 fee, bytes calldata data ) external returns ( bytes32 ) { if ( msg.

sender != address ( flashlender )) revert PositionAction__onCreditFlashLoan__invalidSender (); ( LeverParams memory leverParams, uint256 subCollateral, address residualRecipient ) = abi.

decode ( data, ( LeverParams, uint256, address ) ); uint256 subDebt = leverParams.

primarySwap.

amount; underlyingToken.

forceApprove ( address ( leverParams.

vault ), subDebt + fee ); // sub collateral and debt ICDPVault ( leverParams.

vault ).

modifyCollateralAndDebt ( leverParams.

position, address ( this ), address ( this ), 0, - toInt256 ( subDebt ) ); // withdraw collateral and handle any CDP specific actions @> uint256 withdrawnCollateral = _onDecreaseLever ( leverParams, subCollateral ); if ( leverParams.

primarySwap.

swapType == SwapType.

EXACT_IN ) { leverParams.

primarySwap.

amount = withdrawnCollateral; bytes memory swapData = _delegateCall ( address ( swapAction ), abi.

encodeWithSelector ( swapAction.

swap.

selector, leverParams.

primarySwap ) ); uint256 swapAmountOut = abi.

decode ( swapData, ( uint256 )); uint256 residualAmount = swapAmountOut - subDebt; // sub collateral and debt @> if ( residualAmount > 0 ) { underlyingToken.

forceApprove ( address ( leverParams.

vault ), residualAmount ); ICDPVault ( leverParams.

vault ).

modifyCollateralAndDebt ( leverParams.

position, address ( this ), address ( this ), 0, - toInt256 ( residualAmount ) ); }...

} PositionAction4626.sol:

function _onDecreaseLever ( LeverParams memory leverParams, uint256 subCollateral ) internal override returns ( uint256 tokenOut ) { // withdraw collateral from vault uint256 withdrawnCollateral = ICDPVault ( leverParams.

vault ).

withdraw ( leverParams.

position, subCollateral ); // withdraw collateral from the ERC4626 vault and return underlying assets tokenOut = IERC4626 ( leverParams.

collateralToken ).

redeem ( withdrawnCollateral, address ( this ), address ( this )); if ( leverParams.

auxAction.

args.

length != 0 ) { _delegateCall ( address ( poolAction ), abi.

encodeWithSelector ( poolAction.

exit.

selector, leverParams.

auxAction ) ); tokenOut = IERC20 ( IERC4626 ( leverParams.

collateralToken ).

asset ()).

balanceOf ( address ( this )); } CDPVault.sol:

function modifyCollateralAndDebt ( address owner, address collateralizer, address creditor, int256 deltaCollateral, int256 deltaDebt ) public {...

if ( deltaDebt > 0 ) {...

} else if ( deltaDebt < 0 ) { uint256 debtToDecrease = abs ( deltaDebt ); uint256 maxRepayment = calcTotalDebt ( debtData ); @> if ( debtToDecrease >= maxRepayment ) { debtToDecrease = maxRepayment; deltaDebt = - toInt256 ( debtToDecrease ); } uint256 scaledDebtDecrease = wmul ( debtToDecrease, poolUnderlyingScale ); poolUnderlying.

safeTransferFrom ( creditor, address ( pool ), scaledDebtDecrease ); }

## Recommended Mitigation Steps

Send the residual tokens back to residualRecipient instead of trying to repay debt.

## Assessed type

Token-Transfer amarcu (LoopFi) confirmed and commented:

The scenario is valid if the case where the debt repayment is capped, but not sure about the severity.

Koolex (judge) decreased severity to Medium and commented:

@pkqs90 - Please clarify the likelihood and the following in PJQA:

To explain a bit more, this is a reasonable scenario, because the amount of collateral tokens used for swap comes from uint256 withdrawnCollateral = _onDecreaseLever(leverParams, subCollateral);, and for PositionAction4626, _onDecreaseLever() supports gathering collateral tokens by exiting from pools (e.g., Balancer). This means it is totally possible that the amount of collateral tokens used for swap values more than the user’s debt in CDPVault.

pkqs90 (warden) commented:

@Koolex For PositionAction4626 actions, this issue is more likely to occur because:

Uses a redeem() call for ERC4626 vaults to get collateral tokens. Users cannot know exactly how much tokens are withdrawn beforehand, this is a dynamic value.

If auxAction.args exists, it will perform a pool exit to get collateral tokens. Both balancer and pendle pool exits use a dynamic output value (e.g., Balancer uses EXACT_BPT_IN_FOR_ONE_TOKEN_OUT ) Also considering the general use case:

When conducting collateral token -> debt token swap, the output of swapped out debt token is a dynamic value.

If the position was liquidate-able, and is partially liquidated by other users by accidentally frontrunning, the required repay amount would be smaller than expected.

All above makes it more likely the user over repays his position.

function _onDecreaseLever ( LeverParams memory leverParams, uint256 subCollateral ) internal override returns ( uint256 tokenOut ) { // withdraw collateral from vault uint256 withdrawnCollateral = ICDPVault ( leverParams.

vault ).

withdraw ( leverParams.

position, subCollateral ); // withdraw collateral from the ERC4626 vault and return underlying assets @> tokenOut = IERC4626 ( leverParams.

collateralToken ).

redeem ( withdrawnCollateral, address ( this ), address ( this )); if ( leverParams.

auxAction.

args.

length != 0 ) { @> _delegateCall ( address ( poolAction ), abi.

encodeWithSelector ( poolAction.

exit.

selector, leverParams.

auxAction ) ); tokenOut = IERC20 ( IERC4626 ( leverParams.

collateralToken ).

asset ()).

balanceOf ( address ( this )); } function _balancerExit ( PoolActionParams memory poolActionParams ) internal returns ( uint256 retAmount ) {...

balancerVault.

exitPool ( poolId, address ( this ), payable ( poolActionParams.

recipient ), ExitPoolRequest ({ assets:

assets, minAmountsOut:

minAmountsOut, @> userData:

abi.

encode ( ExitKind.

EXACT_BPT_IN_FOR_ONE_TOKEN_OUT, bptAmount, outIndex ), toInternalBalance:

false }) ); }

## Rejected Primary Findings

# Rejected Primary Findings: LoopFi

# Lack of access control "FUNDS_ADMINISTRATOR_ROLE" in _moveFunds function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-82
- **Submitter:** 0xpetern
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/82
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-82.md

## Brief Summary

The [_moveFunds](https://github.com/code-423n4/2024-10-loopfi/blob/d219f0132005b00a68f505edc22b34f9a8b49766/src/Treasury.sol#L68C5-L73C6) function can be called by any internal function within the contract or derived contracts, which allows unauthorized users to transfer funds from the contract to any specified treasury address. This could lead to significant financial losses, as attackers can exploit this vulnerability to siphon off funds. Here is a comment from the dev But the access control is missing in this critical function. This vulnerability directly impacts the financial security of the contract, potentially allowing attackers to transfer funds without authorization.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# withdraw() in the Locking.sol contract emits an event transferring of 0 tokens, which results in an erroneous event emission

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-21
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/21
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-21.md

## Brief Summary

withdraw() in the Locking.sol contract emits an event transferring of 0 tokens, which results in an erroneous event emission

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Revert on Zero Balance in Treasury Contract's _moveFunds Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-55
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/55
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-55.md

## Brief Summary

Lack of Revert on Zero Balance in Treasury Contract's _moveFunds Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Insufficient permission checks when decreasing collateral or debt

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-17
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/17
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-17.md

## Brief Summary

The current permission checks allow `deltaCollateral < 0` without validating the `collateralizer` permissions, potentially allowing unauthorized users to reduce the collateral. Similarly, `deltaDebt > 0` does not check if the creditor grants permission to increase debt on their behalf. Impact: Unauthorized modification of collateral and debt values could lead to theft or liquidation, which may destabilize the system or lead to unexpected financial losses for users.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Front-running Vulnerability in `_updateBaseInterest()` Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-76
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/76
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-76.md

## Brief Summary

A front-running vulnerability exists in the `_updateBaseInterest()` function due to the improper reliance on `block.timestamp` for determining when to update the `lastBaseInterestUpdate` variable. An attacker can exploit this by calling the function just before the victim, preventing the victim’s transaction from executing the interest update logic. **Affected Function**: PoolV3.sol#L694 **Description**: * The function relies on the condition `if (block.timestamp != lastBaseInterestUpdate_)` to determine whether to update the base interest. However, because `block.timestamp` remains the same within the same block, an attacker can front-run the victim's transaction by calling the function in...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Division by Zero and Withdraw Fee Boundaries in `_amountWithWithdrawalFee`

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-77
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/77
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-77.md

## Brief Summary

The `_amountWithWithdrawalFee` function has two critical issues: 1. **Division by Zero**: When `withdrawFee` equals `PERCENTAGE_FACTOR`, the denominator becomes zero, causing a transaction revert. 2. **Withdraw Fee Boundaries**: A high `withdrawFee` (close to `PERCENTAGE_FACTOR`) results in an abnormally large withdrawal amount, which may lead to unintended behavior. Affected code: PoolV3.sol#L929 **Impact** - **Division by Zero**: Causes transaction failure. - **Boundary Issue**: Allows abnormally large withdrawals, posing a risk to the contract’s balance. **Proposed Fix** 1. Prevent division by zero: 2. Enforce maximum fee limits:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ff-by-One Timestamp Vulnerability in Unstake Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-73
- **Submitter:** PolarizedLight
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/73
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-73.md

## Brief Summary

The unstake function in the StakingLPEth.sol contract uses a problematic comparison operator (>=) when checking against block.timestamp, which could lead to off-by-one errors. Description: In the unstake function, there's a conditional statement that compares block.timestamp to userCooldown.cooldownEnd using the >= operator. This comparison method introduces off-by-one errors due to the discrete nature of block.timestamp updates. Block timestamps are only updated once per block, remaining constant throughout the block's execution. If an operation occurs precisely when the block.timestamp changes, it may result in unexpected behavior. CodeLocation: https://github.com/code-423n4/2024-10-loopf...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Address Comparison Vulnerability in Token Operations

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-74
- **Submitter:** PolarizedLight
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/74
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-74.md

## Brief Summary

Loopfi protocol's code contains critical instances where function parameters of type `address` are directly compared against state variables. This practice can be exploited in the case of proxy tokens or tokens with multiple addresses, leading to security checks being bypassed and potentially resulting in significant financial losses or unauthorized access to protocol functions. Description: In multiple core functions within the contract, there are direct comparisons between an `address` parameter and a state variable (typically `address(underlyingToken)`). While this check aims to ensure operations are performed only on the intended token, it fails to account for the complexities of modern...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Accrued Rewards Reset

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-11
- **Submitter:** black-wolf
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/11
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-11.md

## Brief Summary

Accrued Rewards Reset

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Incorrect Reward Calculation with Initial Index

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-13
- **Submitter:** black-wolf
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/13
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-13.md

## Brief Summary

Incorrect Reward Calculation with Initial Index

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_06_group

# Lack of Slippage Protection in ERC4626 Implementation in PoolV3.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-43
- **Submitter:** catellatech
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/43
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-43.md

## Brief Summary

The provided PoolV3 contract, which implements the ERC4626 standard, lacks explicit slippage protection mechanisms. This omission could potentially lead to unexpected losses for users due to price fluctuations or manipulations during transactions. - https://eips.ethereum.org/EIPS/eip-4626#security-considerations Vulnerability Details The ERC4626 standard [recommends](https://eips.ethereum.org/EIPS/eip-4626#security-considerations) implementing slippage protection for functions that interact with the underlying vault, especially for operations initiated by EOA (Externally Owned Accounts). The current implementation of PoolV3 does not include such protections in its `deposit`, `mint`, `withdr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# CDPVault has getRewards function that is unchecked transfer

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-18
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/18
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-18.md

## Brief Summary

The getRewards function within the CDPVault contract is unprotected. And the getRewards function calls safeTransfer successfully. This getRewards function has no protection on it. And I was able to call the getRewards function from a non-registered address. Vulnerable Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential I/O Flow Issue in Rate Calculation in GaugeV3.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-37
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/37
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-37.md

## Brief Summary

Although the code prevents division by zero with totalVotes == 0, there is still a potential risk of integer overflow during the multiplication and addition of large values for votesCaSide, votesLpSide, qrp.minRate, and qrp.maxRate. In Solidity, casting values to uint256 as shown in this code does not inherently prevent overflow when the numbers involved are large enough. This could result in incorrect rate calculations, leading to unexpected behavior in the system. Potential Impact: The improper calculation of rates could lead to incorrect data being used in voting outcomes, which could impact the overall contract logic. An incorrect rate calculation could skew decision-making based on vot...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Signature Malleability in Permit2 Signature Construction in TransferAction.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-38
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/38
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-38.md

## Brief Summary

Potential Signature Malleability in Permit2 Signature Construction in TransferAction.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Array Access in getSwapToken Function in SwapAction.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-39
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/39
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-39.md

## Brief Summary

The function accesses elements from primarySwapPath without checking its length: token = primarySwapPath[0]; token = primarySwapPath[primarySwapPath.length - 1]; If the primarySwapPath array is empty, this will cause a transaction revert due to an out-of-bounds array access.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ABI Decoding Risk in getSwapToken Function in SwapAction.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-40
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/40
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-40.md

## Brief Summary

ABI Decoding Risk in getSwapToken Function in SwapAction.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Return Values from Virtual Function _updateRewardIndex in RewardManagerAbstract.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-41
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/41
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-41.md

## Brief Summary

The function _updateRewardIndex() is marked as virtual and will be overridden by derived contracts. The return values (tokens and indexes) from _updateRewardIndex() are used directly in the logic without validation. Since these values come from an unknown implementation, they could be faulty or unexpected, such as empty arrays or arrays with mismatched lengths. Potential Risks: Faulty Data Handling: Using unvalidated tokens and indexes arrays could lead to incorrect logic execution, failed transactions, or undefined behavior if the arrays are empty or mismatched.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Timestamp Manipulation in Interest Rate Calculation at QuotasLogic.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-53
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/53
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-53.md

## Brief Summary

The _cumulativeIndexSince() function calculates the interest index using block.timestamp, which can be slightly manipulated by miners. This creates a risk of interest rate manipulation, especially in a financial system where even small changes in time can lead to significant discrepancies over long-term calculations or high-value transactions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Integer Overflow in calcQuotaRevenueChange Function at QuotasLogic.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-54
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/54
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-54.md

## Brief Summary

The calcQuotaRevenueChange() function performs arithmetic with int256 and uint256 values. The multiplication of change (an int256) by rate (cast to int256 from uint256) could lead to an integer overflow or underflow if the result exceeds the bounds of an int256.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Arbitrary Jump in Delegate Call in PositionActionPendle.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-56
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/56
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-56.md

## Brief Summary

The _delegateCall() function is called with basic validation on leverParams.auxAction.args.length, ensuring that the call is not made with empty data. However, there is still a risk of an arbitrary jump if the target address (poolAction) or the data being passed can be manipulated, even with a success/failure check present. While the function handles failed calls properly, it does not fully mitigate the risk of arbitrary code execution or jumps to unintended contract functions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Integer Overflow Risk in PositionAction4626.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-57
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/57
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-57.md

## Brief Summary

There is a potential risk of integer overflow in the following code segment where upFrontAmount is added to addCollateralAmount. Although Solidity 0.8.x provides automatic overflow protection, it is important to ensure that these additions do not exceed safe limits in the context of this contract's logic.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Violation of Checks-Effects-Interactions Pattern in PositionAction4626.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-58
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/58
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-58.md

## Brief Summary

The Checks-Effects-Interactions (CEI) pattern is not followed in the _onDecreaseLever() function, where external calls are made before the state changes are finalized. This leaves the function open to reentrancy attacks, where malicious contracts could exploit the contract's state by reentering it before the final state changes are completed.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# Lack of Role-Based Access Control (RBAC) in Vault Management

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-63
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/63
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-63.md

## Brief Summary

Critical functions like addVault() and removeVault() in the IVaultRegistry contract are missing access control. This allows any external user to add or remove vaults, which could lead to unauthorized vault manipulation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Push Model in moveFunds

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-67
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/67
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-67.md

## Brief Summary

Issue: The moveFunds function uses a push model (Address.sendValue()), which can lead to transaction failures if the receiving address (treasury) is unable to accept Ether.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Use of Push Pattern Instead of Pull for Fund Transfers in PoolV3

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-68
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/68
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-68.md

## Brief Summary

The PoolV3 contract uses the "push" pattern for transferring funds directly to a user in the _withdraw function, such as in the following line: IERC20(underlyingToken).safeTransfer({to: receiver, value: amountToUser}); // U:[LP-8,9] Using the "push" pattern to transfer funds directly to users can lead to potential risks such as reentrancy or failed transfers. It’s generally safer to adopt the "pull over push" pattern, where users can explicitly request to withdraw their funds, reducing the likelihood of vulnerabilities during the transfer process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Circuit Breaker Pattern for Critical Operations

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-70
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/70
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-70.md

## Brief Summary

Description: The contract StakingLPEth handles user funds and critical operations like withdrawals and staking, but it lacks a circuit breaker mechanism that can halt all or certain contract functions in case of emergencies or vulnerabilities. The absence of a circuit breaker pattern poses a risk in situations where vulnerabilities are discovered, or the contract behavior is not functioning as expected. If such an issue arises, the contract cannot be paused to prevent further damage or malicious actions until it is resolved.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can repay more than they owe and lose funds

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-81
- **Submitter:** misbahu
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/81
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-81.md

## Brief Summary

Users can repay more than they owe and lose funds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# BalancerOracle Fails to Validate Token Addresses, Resulting in Incorrect Prices for Any Token

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-84
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/84
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-84.md

## Brief Summary

BalancerOracle Fails to Validate Token Addresses, Resulting in Incorrect Prices for Any Token

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Mismatch in Scaling of deltaCollateral in modifyCollateralAndDebt() Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-85
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/85
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-85.md

## Brief Summary

Mismatch in Scaling of deltaCollateral in modifyCollateralAndDebt() Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Calculation of Liquidation Penalty Leading to Potential Over-Penalization or Under-Penalization

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-86
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/86
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-86.md

## Brief Summary

Inconsistent Calculation of Liquidation Penalty Leading to Potential Over-Penalization or Under-Penalization

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Donation attack vulnerability in `StakingLPEth` Contract

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-25
- **Submitter:** safie
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/25
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-25.md

## Brief Summary

The vulnerability allows attackers to execute donation attacks by circumventing the minimum share constraints. This can lead to the following potential impacts: Asset Loss: Users may unintentionally donate shares to attackers, resulting in a financial loss. Reduced Integrity of the Pool: The value of the pool may be diminished as small shares are systematically drained through coordinated attacks. This vulnerability could significantly undermine the integrity of the staking mechanism, allowing malicious actors to exploit the contract for financial gain through systematic donation attacks. Immediate remediation is advised to prevent potential exploits.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Precision loss vulnerability in division operations with `RAY` and `SECONDS_PER_YEAR` will affect the accuracy of interest accrual or quota calculations in `QuotasLogic.sol`

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-48
- **Submitter:** safie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/48
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-48.md

## Brief Summary

Precision loss vulnerability in division operations with `RAY` and `SECONDS_PER_YEAR` will affect the accuracy of interest accrual or quota calculations in `QuotasLogic.sol`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary
