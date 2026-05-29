# Benchmark Ground Truth: BendDAO Invitational

## Accepted H/M Findings

# Accepted H/M Findings: BendDAO Invitational

# [H-01] executeIsolateLiquidate() totalBidAmout/availableLiquidity incorrect accounting

- **Contest:** BendDAO Invitational
- **Slug:** 2024-12-benddao-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-benddao-invitational
- **Source snapshot:** competitions/2024-12-benddao-invitational/final_report.html

executeIsolateLiquidate() totalBidAmout/availableLiquidity incorrect accounting Submitted by bin2chen, also found by oakcobalt ( 1, 2 ) and SpicyMeatball ( 1, 2 ) Code reference:

IsolateLogic.sol#L499 When executeIsolateLiquidate() is executed, it will account totalBidAmout/availableLiquidity.

The main accounting code is as follows:

function executeIsolateLiquidate (InputTypes.ExecuteIsolateLiquidateParams memory params ) internal {...

InterestLogic.

updateInterestRates ( poolData, debtAssetData, vars.

totalBorrowAmount, 0 ); if ( vars.

totalExtraAmount > 0 ) { // transfer underlying asset from caller to pool @> VaultLogic.

erc20TransferInLiquidity ( debtAssetData, params.

msgSender, vars.

totalExtraAmount ); } // bid already in pool and now repay the borrow but need to increase liquidity @> VaultLogic.

erc20TransferOutBidAmountToLiqudity ( debtAssetData, vars.

totalBorrowAmount ); // transfer erc721 to winning bidder if ( params.

supplyAsCollateral ) { VaultLogic.

erc721TransferIsolateSupplyOnLiquidate ( nftAssetData, vars.

winningBidder, params.

nftTokenIds, true ); } else { VaultLogic.

erc721DecreaseIsolateSupplyOnLiquidate ( nftAssetData, params.

nftTokenIds ); VaultLogic.

erc721TransferOutLiquidity ( nftAssetData, vars.

winningBidder, params.

nftTokenIds ); } function erc20TransferInLiquidity (DataTypes.AssetData storage assetData, address from, uint256 amount ) internal { address asset = assetData.

underlyingAsset; uint256 poolSizeBefore = IERC20Upgradeable ( asset ).

balanceOf ( address ( this )); @> assetData.

availableLiquidity += amount; IERC20Upgradeable ( asset ).

safeTransferFrom ( from, address ( this ), amount ); uint256 poolSizeAfter = IERC20Upgradeable ( asset ).

balanceOf ( address ( this )); require ( poolSizeAfter == ( poolSizeBefore + amount ), Errors.

INVALID_TRANSFER_AMOUNT ); } function erc20TransferOutBidAmountToLiqudity (DataTypes.AssetData storage assetData, uint amount ) internal { require ( assetData.

totalBidAmout >= amount, Errors.

ASSET_INSUFFICIENT_BIDAMOUNT ); @> assetData.

totalBidAmout -= amount; @> assetData.

availableLiquidity += amount; } We know from the above code that the current formula is as follows:

availableLiquidity += (totalBorrowAmount + totalExtraAmount) totalBidAmout -= totalBorrowAmount Both of these accounting errors. TotalExtraAmount is calculated twice.totalBorrowAmount already contains totalExtraAmount.

Example:

Suppose: total Borrow Amount = 100, Actual Bid Amout = 80 So: total Extra Amount = 20 but in the current algorithm:

availableLiquidity += (totalBorrowAmount + totalExtraAmount) = 100 + 20 = 120 totalBidAmout -= totalBorrowAmount = 100 The correct value is:

availableLiquidity += totalBorrowAmount = 100 totalBidAmout -= (totalBorrowAmount - totalExtraAmount) = (100 - 20) = 80

## Impact

availableLiquidity adds extra totalExtraAmount, resulting in inaccurate borrowRate.

totalBidAmout underflow, which makes it impossible to liquidate.

## Recommended Mitigation

function executeIsolateLiquidate(InputTypes.ExecuteIsolateLiquidateParams memory params) internal {...

InterestLogic.updateInterestRates(poolData, debtAssetData, vars.totalBorrowAmount, 0); if (vars.totalExtraAmount > 0) { // transfer underlying asset from caller to pool VaultLogic.erc20TransferInLiquidity(debtAssetData, params.msgSender, vars.totalExtraAmount); } // bid already in pool and now repay the borrow but need to increase liquidity - VaultLogic.erc20TransferOutBidAmountToLiqudity(debtAssetData, vars.totalBorrowAmount); + VaultLogic.erc20TransferOutBidAmountToLiqudity(debtAssetData, vars.totalBorrowAmount - vars.totalExtraAmount ); thorseldon (BendDAO) confirmed and commented:

Fixed in commit 18a4b84

# [H-02] YieldWUSDStaking.repay() may incorrectly over-refund remainAmount to user

- **Contest:** BendDAO Invitational
- **Slug:** 2024-12-benddao-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-benddao-invitational
- **Source snapshot:** competitions/2024-12-benddao-invitational/final_report.html

YieldWUSDStaking.repay() may incorrectly over-refund remainAmount to user Submitted by bin2chen, also found by oakcobalt and SpicyMeatball Code reference:

YieldWUSDStaking.sol#L468 In YieldWUSDStaking.repay(), we will calculate the amount that needs to be refunded to the user remainAmount:

function _repay ( uint32 poolId, address nft, uint256 tokenId ) internal virtual {...

// compute fine value if ( vars.

remainAmount >= sd.

unstakeFine ) { vars.

remainAmount = vars.

remainAmount - sd.

unstakeFine; } else { vars.

extraAmount = vars.

extraAmount + ( sd.

unstakeFine - vars.

remainAmount ); @> // missing clear vars.remainAmount = 0 } @> sd.

remainYieldAmount = vars.

remainAmount;...

// send remain funds to owner if ( sd.

remainYieldAmount > 0 ) { @> underlyingAsset.

safeTransfer ( vars.

nftOwner, sd.

remainYieldAmount ); sd.

remainYieldAmount = 0; } The problem is that when use call repay(), if vars.remainAmount < sd.unstakeFine, it doesn’t clear vars.remainAmount to 0, which results in a refund to the user, which should be treated as unstakeFine.

For example: unstakeFine = 10, remainAmount = 5 As currently calculated extraAmount = (unstakeFine - remainAmount ) = 10 - 5 = 5 remainYieldAmount = 5 =====> ( should 0)

## Impact

Over-refund remainAmount to user.

At the same time, it will also cause the contract balance to be insufficient (after collectFeeToTreasury () ) and others will not be able to repay.

## Recommended Mitigation

function _repay(uint32 poolId, address nft, uint256 tokenId) internal virtual {...

// compute fine value if (vars.remainAmount >= sd.unstakeFine) { vars.remainAmount = vars.remainAmount - sd.unstakeFine; } else { vars.extraAmount = vars.extraAmount + (sd.unstakeFine - vars.remainAmount); + if(msg.sender != botAdmin) { + vars.remainAmount = 0; + } } sd.remainYieldAmount = vars.remainAmount;...

// send remain funds to owner if (sd.remainYieldAmount > 0) { underlyingAsset.safeTransfer(vars.nftOwner, sd.remainYieldAmount); sd.remainYieldAmount = 0; } thorseldon (BendDAO) confirmed and commented:

Fixed in commit 75055c9 Medium Risk Findings (4)

# [M-01] Staked assets can be locked in Lido, due to vulnerable check in YieldEthStakingLido::protocolDeposit

- **Contest:** BendDAO Invitational
- **Slug:** 2024-12-benddao-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-benddao-invitational
- **Source snapshot:** competitions/2024-12-benddao-invitational/final_report.html

YieldEthStakingLido::protocolDeposit Submitted by oakcobalt Code reference:

YieldEthStakingLido.sol#L85

## Finding description and impact

In Lido’s WithdrawRequest(WithdrawalQueue.sol) implementation, only a withdraw request with stETH amount within MIN_STETH_WITHDRAWAL_AMOUNT, MAX_STETH_WITHDRAWAL_AMOUNT ( [100, 1000ether]) can be processed.

If the yieldAmount exceeds 1000ether, the withdrawal request has to be split into several requests.

The problem is YiedEthstaingLido::protocolDeposit has a vulnerable check on MAX_STETH_WITHDRAWAL_AMOUNT, and will still allow a position to stake more than MAX_STETH_WITHDRAWAL_AMOUNT overtime.

Since YieldEthStakingLido.sol doesn’t allow splitting withdrawal of a staking position(yieldStakeData), any staking position exceeding MAX_STETH_WITHDRAWAL_AMOUNT cannot be unstaked. Funds will be locked in Lido.

## Impact

Unable to unstake, NFT locked and ETH locked in Lido.

## Recommended mitigation steps

Consider moving the max cap check (total yield amount + borrow) into YieldStakingBase::_stake. The actual check parameter can be pulled from YieldEthStakingLido.sol. For example, //src/yield/YieldStakingBase.sol function _stake(uint32 poolId, address nft, uint256 tokenId, uint256 borrowAmount) internal virtual {...

vars.totalYieldBeforeDeposit = getAccountTotalUnstakedYield(address(vars.yieldAccout)); + checkMaxStakingCap(vars.totalYieldBeforeDeposit + borrowAmount); vars.yieldAmount = protocolDeposit(sd, borrowAmount);...

//src/yield/lido/YieldEthStakingLido.sol function checkMaxStakingCap(uint256 amount ) public pure override{ require(amount >= MIN_STETH_WITHDRAWAL_AMOUNT, Errors.YIELD_ETH_LT_MIN_AMOUNT); require(amount <= MAX_STETH_WITHDRAWAL_AMOUNT, Errors.YIELD_ETH_GT_MAX_AMOUNT);...

thorseldon (BendDAO) acknowledged and commented:

As we implement the new YieldWUSDStaking contract, we already know the increment stake issues which should not be supported in any yield contract.

But the fix is pending as our frontend only support only 1 stake per NFT for now.

# [M-02] YieldStakingBase::collectFeeToTreasury will attempt to transfer funds that are not available, resulting in repetitive revert and potential DOS

- **Contest:** BendDAO Invitational
- **Slug:** 2024-12-benddao-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-benddao-invitational
- **Source snapshot:** competitions/2024-12-benddao-invitational/final_report.html

YieldStakingBase::collectFeeToTreasury will attempt to transfer funds that are not available, resulting in repetitive revert and potential DOS Submitted by oakcobalt, also found by SpicyMeatball Code references:

YieldStakingBase.sol#L171 YieldStakingBase.sol#L357 YieldStakingBase::collectFeeToTreasury will attempt to transfer funds that are not available, resulting in repetitive revert.

## Impact

collectFeeToTreasury might face repetitive revert or even DOS. protocol funds can be locked.

## Recommended mitigation steps

Consider only incrementing totalUnstakFine in _repay. The increment amount is the actual funds that are claimed or covered by the user.

thorseldon (BendDAO) acknowledged

# [M-03] Missing price staleness check for WEETH_AGGREGATOR

- **Contest:** BendDAO Invitational
- **Slug:** 2024-12-benddao-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-benddao-invitational
- **Source snapshot:** competitions/2024-12-benddao-invitational/final_report.html

WEETH_AGGREGATOR Submitted by oakcobalt Code reference:

EETHPriceAdapter.sol#L115 EETHPriceAdapter::latestRoundata() is called by PriceOracle::getAssetPriceFromChainlink. It provides price of eETH in baseCurrency(USD) by using two chainlink oracles - ETH/USD( BASE_AGGREGATOR ), WETH/ETH ( WEETH_AGGREGATOR ).

The problem is the flow of getAssetPriceFromChainlink -> EETHPriceAdapter::latestRoundata() would only have price staleness check for BASE_AGGREGATOR oracle, but no price staleness check for WEETH_AGGREGATOR oracle.

Missing price staleness check for one of the two oracle could result in a invalid eETH/USD price.

## Recommended mitigation steps

Consider adding staleness check for WEETH_AGGREGATOR in EETHPriceAdapter::latestRoundata.

thorseldon (BendDAO) acknowledged 0xTheC0der (judge) commented:

Close to M-2 “Use of deprecated chainlink function:

latestAnswer() ” from the

# [M-04] YieldStakingBase.stake() cannot append borrowAmount

- **Contest:** BendDAO Invitational
- **Slug:** 2024-12-benddao-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-benddao-invitational
- **Source snapshot:** competitions/2024-12-benddao-invitational/final_report.html

YieldStakingBase.stake() cannot append borrowAmount Submitted by bin2chen Code reference:

YieldLogic.sol#L150 In this PR, yieldSetERC721TokenData() adds the following restrictions:

function executeYieldSetERC721TokenData (InputTypes.ExecuteYieldSetERC721TokenDataParams memory params ) internal {...

if ( params.

isLock ) { require ( ymData.

yieldCap > 0, Errors.

YIELD_EXCEED_STAKER_CAP_LIMIT ); @> require ( tokenData.

lockerAddr == address ( 0 ), Errors.

ASSET_ALREADY_LOCKED_IN_USE ); VaultLogic.

erc721SetTokenLockerAddr ( nftAssetData, params.

tokenId, lockerAddr ); } else { require ( tokenData.

lockerAddr == lockerAddr, Errors.

YIELD_TOKEN_LOCKED_BY_OTHER ); VaultLogic.

erc721SetTokenLockerAddr ( nftAssetData, params.

tokenId, address ( 0 )); } This causes yieldSetERC721TokenData() to be called only once. This is fine for YieldWUSDStaking.sol. But for the other YieldStakingBase.sol, there is a problem, because it is not possible to increase borrowing again (Health Factor is still enough) as before.

In YieldStakingBase.stake() function _stake ( uint32 poolId, address nft, uint256 tokenId, uint256 borrowAmount ) internal virtual {...

YieldStakeData storage sd = stakeDatas [ nft ][ tokenId ]; if ( sd.

yieldAccount == address ( 0 )) { require ( vars.

nftLockerAddr == address ( 0 ), Errors.

YIELD_ETH_NFT_ALREADY_USED ); vars.

totalDebtAmount = borrowAmount; sd.

yieldAccount = address ( vars.

yieldAccout ); sd.

poolId = poolId; sd.

state = Constants.

YIELD_STATUS_ACTIVE; } else { require ( vars.

nftLockerAddr == address ( this ), Errors.

YIELD_ETH_NFT_NOT_USED_BY_ME ); require ( sd.

state == Constants.

YIELD_STATUS_ACTIVE, Errors.

YIELD_ETH_STATUS_NOT_ACTIVE ); require ( sd.

poolId == poolId, Errors.

YIELD_ETH_POOL_NOT_SAME ); vars.

totalDebtAmount = convertToDebtAssets ( poolId, sd.

debtShare ) + borrowAmount; }....

@> poolYield.

yieldSetERC721TokenData ( poolId, nft, tokenId, true, address ( underlyingAsset )); // check hf uint256 hf = calculateHealthFactor ( nft, nc, sd ); require ( hf >= nc.

unstakeHeathFactor, Errors.

YIELD_ETH_HEATH_FACTOR_TOO_LOW ); emit Stake ( msg.

sender, nft, tokenId, borrowAmount ); }

## Impact

Users can’t borrow additional funds and have to pay back the loan first, losing a certain amount of handling fee.

## Recommended Mitigation

Two possible modifications _stake() does not call poolYield.yieldSetERC721TokenData() if appending.

function _stake(uint32 poolId, address nft, uint256 tokenId, uint256 borrowAmount) internal virtual {...

require(vars.nftLockerAddr == address(this), Errors.YIELD_ETH_NFT_NOT_USED_BY_ME); require(sd.state == Constants.YIELD_STATUS_ACTIVE, Errors.YIELD_ETH_STATUS_NOT_ACTIVE); require(sd.poolId == poolId, Errors.YIELD_ETH_POOL_NOT_SAME); vars.totalDebtAmount = convertToDebtAssets(poolId, sd.debtShare) + borrowAmount; }....

poolYield.yieldSetERC721TokenData(poolId, nft, tokenId, true, address(underlyingAsset)); if (vars.nftLockerAddr != address(this)) poolYield.yieldSetERC721TokenData(poolId, nft, tokenId, true, address(underlyingAsset)); } 2. `yieldSetERC721TokenData()` to allow the current `nftLockerAddr` to execute.

```diff function executeYieldSetERC721TokenData(InputTypes.ExecuteYieldSetERC721TokenDataParams memory params) internal {...

if (params.isLock) { require(ymData.yieldCap > 0, Errors.YIELD_EXCEED_STAKER_CAP_LIMIT); - require(tokenData.lockerAddr == address(0), Errors.ASSET_ALREADY_LOCKED_IN_USE); + require(tokenData.lockerAddr == address(0) || tokenData.lockerAddr == lockerAddr), Errors.ASSET_ALREADY_LOCKED_IN_USE); VaultLogic.erc721SetTokenLockerAddr(nftAssetData, params.tokenId, lockerAddr); } else { require(tokenData.lockerAddr == lockerAddr, Errors.YIELD_TOKEN_LOCKED_BY_OTHER); VaultLogic.erc721SetTokenLockerAddr(nftAssetData, params.tokenId, address(0)); } It is recommended to choose option (2).

thorseldon (BendDAO) confirmed and commented:

Fixed in commit eef87bf

## Rejected Primary Findings

# Rejected Primary Findings: BendDAO Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** captured_from_authenticated_browser
