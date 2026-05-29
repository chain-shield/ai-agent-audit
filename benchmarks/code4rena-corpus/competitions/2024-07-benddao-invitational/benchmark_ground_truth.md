# Benchmark Ground Truth: BendDAO Invitational

## Accepted H/M Findings

# Accepted H/M Findings: BendDAO Invitational

# [H-01] Mismatch between yield amount deposited in shares calculation and getAccountYieldBalance()

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

getAccountYieldBalance() Submitted by 0x73696d616f, also found by bin2chen ( 1, 2 ) Mismatch between the return amount in all yield Etherfi and Lido assets and the getAccountYieldBalance() call causes the yield amount of each staked nft to the same YieldAccount to be incorrect, leading to the possibility of withdrawing the excess for some nfts while being closer to liquidation on others

## Recommended Mitigation Steps

On YieldStakingBase::deposit() the claimed amount should be the assets deposited (roughly equal to the borrowed amount minus rounding errors).

thorseldon (BendDAO) confirmed and commented:

Fixed here and here.

# [H-02] isolateRepay() lack of check onBehalf == nftOwner

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

isolateRepay() lack of check onBehalf == nftOwner Submitted by bin2chen When calling isolateRepay() we can specify nftTokenIds and onBehalf. The current implementation just restricts onBehalf!=address(0) and does not restrict onBehalf == nftOwner.

function validateIsolateRepayBasic ( InputTypes.ExecuteIsolateRepayParams memory inputParams, DataTypes.PoolData storage poolData, DataTypes.AssetData storage debtAssetData, DataTypes.AssetData storage nftAssetData ) internal view { validatePoolBasic ( poolData ); validateAssetBasic ( debtAssetData ); require ( debtAssetData.

assetType == Constants.

ASSET_TYPE_ERC20, Errors.

ASSET_TYPE_NOT_ERC20 ); validateAssetBasic ( nftAssetData ); require ( nftAssetData.

assetType == Constants.

ASSET_TYPE_ERC721, Errors.

ASSET_TYPE_NOT_ERC721 ); @> require ( inputParams.

onBehalf != address ( 0 ), Errors.

INVALID_ONBEHALF_ADDRESS ); require ( inputParams.

nftTokenIds.

length > 0, Errors.

INVALID_ID_LIST ); require ( inputParams.

nftTokenIds.

length == inputParams.

amounts.

length, Errors.

INCONSISTENT_PARAMS_LENGTH ); validateArrayDuplicateUInt256 ( inputParams.

nftTokenIds ); for ( uint256 i = 0; i < inputParams.

amounts.

length; i ++) { require ( inputParams.

amounts [ i ] > 0, Errors.

INVALID_AMOUNT ); } This way, we can maliciously specify onBehalf!=nftOwner and when the method is executed userScaledIsolateBorrow[onBehalf] will be maliciously reduced. When the victim makes a real repayment or is liquidated, it will not be repaid or liquidated due to insufficient userScaledIsolateBorrow[onBehalf] resulting in an underflow.

Example:

Alice depositERC721(NFT_1).

Bob depositERC721(NFT_2).

Alice isolateBorrow(NFT_1, 100).

userScaledIsolateBorrow[alice] = 100 loanData[NFT_1].scaledAmount = 100 Bob isolateBorrow(NFT_2, 100) userScaledIsolateBorrow[bob] = 100 loanData[NFT_2].scaledAmount = 100 Alice malicious repay, isolateRepay(NFT_1,amounts = 1, onBehalf = bob) userScaledIsolateBorrow[bob] = 99 When NFT_2 be liquidated, isolateLiquidate(NFT_2) userScaledIsolateBorrow[bob] = loanData[NFT_2].scaledAmount = 99 - 100 =====> underflow.

Note:

isolateRedeem() is similar.

## Impact

Maliciously prevent being liquidated or repaid.

## Recommended Mitigation

limit onBehalf==owner:

function validateIsolateRepayBasic( InputTypes.ExecuteIsolateRepayParams memory inputParams, DataTypes.PoolData storage poolData, DataTypes.AssetData storage debtAssetData, DataTypes.AssetData storage nftAssetData ) internal view { validatePoolBasic(poolData); validateAssetBasic(debtAssetData); require(debtAssetData.assetType == Constants.ASSET_TYPE_ERC20, Errors.ASSET_TYPE_NOT_ERC20); validateAssetBasic(nftAssetData); require(nftAssetData.assetType == Constants.ASSET_TYPE_ERC721, Errors.ASSET_TYPE_NOT_ERC721); require(inputParams.onBehalf != address(0), Errors.INVALID_ONBEHALF_ADDRESS); require(inputParams.nftTokenIds.length > 0, Errors.INVALID_ID_LIST); require(inputParams.nftTokenIds.length == inputParams.amounts.length, Errors.INCONSISTENT_PARAMS_LENGTH);

validateArrayDuplicateUInt256(inputParams.nftTokenIds); for (uint256 i = 0; i < inputParams.amounts.length; i++) { require(inputParams.amounts[i] > 0, Errors.INVALID_AMOUNT); + DataTypes.ERC721TokenData storage tokenData = VaultLogic.erc721GetTokenData( + nftAssetData, + inputParams.nftTokenIds[i] + ); + require(tokenData.owner == inputParams.onBehalf, Errors.ISOLATE_LOAN_OWNER_NOT_MATCH); }

## Assessed type

Context thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [H-03] erc721DecreaseIsolateSupplyOnLiquidate() missing clear lockerAddr

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

erc721DecreaseIsolateSupplyOnLiquidate() missing clear lockerAddr Submitted by bin2chen, also found by oakcobalt When isolateLiquidate(supplyAsCollateral=false) is executed, finally erc721DecreaseIsolateSupplyOnLiquidate() will be executed and the NFT will be transferred to the user.

function erc721DecreaseIsolateSupplyOnLiquidate ( DataTypes.AssetData storage assetData, uint256 [] memory tokenIds ) internal { for ( uint256 i = 0; i < tokenIds.

length; i ++) { DataTypes.

ERC721TokenData storage tokenData = assetData.

erc721TokenData [ tokenIds [ i ]]; require ( tokenData.

supplyMode == Constants.

SUPPLY_MODE_ISOLATE, Errors.

INVALID_SUPPLY_MODE ); assetData.

userScaledIsolateSupply [ tokenData.

owner ] -= 1; tokenData.

owner = address ( 0 ); tokenData.

supplyMode = 0; @> //missing tokenData.lockerAddr = address(0); } assetData.

totalScaledIsolateSupply -= tokenIds.

length; } We know from the above code that this method does not clear the tokenData.lockerAddr, so now tokenData is:

erc721TokenData[NFT_1].owner = 0 erc721TokenData[NFT_1].supplyMode = 0 erc721TokenData[NFT_1].lockerAddr = address(poolManager) User Alice has NFT_1; then Alice executes BVault.depositERC721(NFT_1, supplyMode = SUPPLY_MODE_CROSS) will succeed, deposit() does not check lockerAddr.

So tokenData becomes:

erc721TokenData[NFT_1].owner = Alice erc721TokenData[NFT_1].supplyMode = SUPPLY_MODE_CROSS erc721TokenData[NFT_1].lockerAddr = address(poolManager) - not changed.

After that the user’s NFT_ 1 will be locked because withdrawERC721() -> validateWithdrawERC721() will check that lockerAddr must be address(0):

function validateWithdrawERC721 (...

for ( uint256 i = 0; i < inputParams.tokenIds.length; i ++) { DataTypes.

ERC721TokenData storage tokenData = VaultLogic.

erc721GetTokenData ( assetData, inputParams.

tokenIds [ i ]); require ( tokenData.

owner == inputParams.

onBehalf, Errors.

INVALID_CALLER ); require ( tokenData.

supplyMode == inputParams.

supplyMode, Errors.

INVALID_SUPPLY_MODE ); @> require ( tokenData.

lockerAddr == address ( 0 ), Errors.

ASSET_ALREADY_LOCKED_IN_USE ); } Other Isolate methods cannot be operated either.

Note:

erc721DecreaseIsolateSupply() is similar.

## Impact

Unable to retrieve NFT.

## Recommended Mitigation

function erc721DecreaseIsolateSupplyOnLiquidate( DataTypes.AssetData storage assetData, uint256[] memory tokenIds ) internal { for (uint256 i = 0; i < tokenIds.length; i++) { DataTypes.ERC721TokenData storage tokenData = assetData.erc721TokenData[tokenIds[i]]; require(tokenData.supplyMode == Constants.SUPPLY_MODE_ISOLATE, Errors.INVALID_SUPPLY_MODE); assetData.userScaledIsolateSupply[tokenData.owner] -= 1; tokenData.owner = address(0); tokenData.supplyMode = 0; + tokenData.lockerAddr = address(0); } assetData.totalScaledIsolateSupply -= tokenIds.length; } function erc721DecreaseIsolateSupply( DataTypes.AssetData storage assetData, address user, uint256[] memory tokenIds ) internal { for (uint256 i = 0; i < tokenIds.length; i++) {

DataTypes.ERC721TokenData storage tokenData = assetData.erc721TokenData[tokenIds[i]]; require(tokenData.supplyMode == Constants.SUPPLY_MODE_ISOLATE, Errors.INVALID_SUPPLY_MODE); tokenData.owner = address(0); tokenData.supplyMode = 0; + tokenData.lockerAddr = address(0); } assetData.totalScaledIsolateSupply -= tokenIds.length; assetData.userScaledIsolateSupply[user] -= tokenIds.length; }

## Assessed type

Context thorseldon (BendDAO) confirmed and commented:

Fixed here.

0xTheC0der (judge) commented:

Other Isolate methods cannot be operated either.

Points out other instances.

# [H-04] Revert due to underflow error, leading to lock of the liquidated NFT

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by Ch_301, also found by bin2chen and oakcobalt

- https://github.com/code-423n4/2024-07-benddao/blob/main/src/libraries/logic/IsolateLogic.sol#L464
- https://github.com/code-423n4/2024-07-benddao/blob/main/src/libraries/logic/IsolateLogic.sol#L269
Description The IsolateLogic.sol#executeIsolateAuction() is triggered by liquidators to launch a new auction or just participate in an existing one for isolated lending. Each NFT token has a predefined auction duration nftAssetData.auctionDuration, the MAX duration is 7 days. In the period of (e.g.) 7 days, the borrowAmount will keep increasing due to the interest:

File:

GenericLogic.

sol # calculateNftLoanLiquidatePrice () 279:

vars.

borrowAmount = nftLoanData.

scaledAmount.

rayMul ( vars.

normalizedIndex ); If the liquidator is the first bidder, the bid price must be greater than borrowed debt and the liquidate price. Otherwise, the liquidator will just try to outbid the last bidder, by at least 1%.

At the end of executeIsolateAuction(), it will transfer the underlying asset from the liquidator to escrow and increase the value of assetData.totalBidAmout by the bid amount from the liquidator (Note: remember the last point).

File:

IsolateLogic.

sol # executeIsolateAuction () 269:

VaultLogic.

erc20TransferInBidAmount ( debtAssetData, params.

msgSender, vars.

totalBidAmount ); After 7 days, the liquidator who wins the auction will invoke IsolateLogic.sol#executeIsolateLiquidate() to implement the liquidation. It will calculate the borrowAmount and check for any remaining or extra amount.

File:

IsolateLogic.

sol # executeIsolateLiquidate () 427:

vars.

borrowAmount = loanData.

scaledAmount.

rayMul ( vars.

normalizedIndex ); Then, because the bid is already in the pool, it will sub-call to VaultLogic.sol#erc20TransferOutBidAmountToLiqudity() to increase liquidity in the pool assetData.availableLiquidity and decrease the assetData.totalBidAmout value.

The issue is here, is about how much totalBidAmout has decreased. In the below code snap is passing totalBorrowAmount to erc20TransferOutBidAmountToLiqudity():

File:

IsolateLogic.

sol # executeIsolateLiquidate () 464:

VaultLogic.

erc20TransferOutBidAmountToLiqudity ( debtAssetData, vars.

totalBorrowAmount ); The vars.totalBorrowAmount is just the value of vars.borrowAmount (in case of one NFT in the liquidation). As we mentioned above, the borrowed amount will keep increasing over time and it will exceed the saved value in assetData.totalBidAmout (check the above note). This will lead the transaction to revert:

File:

VaultLogic.

sol 503:

function erc20TransferOutBidAmountToLiqudity (DataTypes.AssetData storage assetData, uint amount ) internal { 504:

assetData.

totalBidAmout -= amount; 505:

assetData.

availableLiquidity += amount; 506: } There are multiple scenarios to reach this point (revert):

The auction ends with only one bidder (the first bidder wins it), so the rest of the time (until the auction ends) the borrowed amount will keep accumulating interest.

The auction ends, but the borrowed amount exceeds the winner’s bid price (due to accumulated interest).

## Impact

The liquidator wins the auction but he can’t withdraw/unlock the NFT, due to an Arithmetic over/underflow and fails when reducing the assetData.totalBidAmout value in erc20TransferOutBidAmountToLiqudity().

## Recommended Mitigation Steps

Just decrease assetData.totalBidAmout by vars.totalBidAmount:

File: IsolateLogic.sol#executeIsolateLiquidate() - 464: VaultLogic.erc20TransferOutBidAmountToLiqudity(debtAssetData, vars.totalBorrowAmount); + 464: VaultLogic.erc20TransferOutBidAmountToLiqudity(debtAssetData, vars.totalBorrowAmount, vars.totalBidAmount); File: VaultLogic.sol - 503: function erc20TransferOutBidAmountToLiqudity(DataTypes.AssetData storage assetData, uint amount) internal { + 503: function erc20TransferOutBidAmountToLiqudity(DataTypes.AssetData storage assetData, uint amount, unit bidAmount) internal { - 504: assetData.totalBidAmout -= amount; + 504: assetData.totalBidAmout -= bidAmount; 505: assetData.availableLiquidity += amount; 506: }

## Assessed type

Math thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [H-05] Bad debt is never handled which places insolvency risks on BendDAO

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by 0x73696d616f Bad debt is never handled, which may happen whenever the collateral asset price crashes or the debt value spikes. Whenever this happens ERC20 positions will be partially liquidated and ERC721 positions will not be profitable for liquidation, which means the debt will not be repaid. When this happens, the protocol will not have enough liquidity to fulfill withdrawals, DoSing them forever or until the price comes back up again.

## Recommended Mitigation Steps

On liquidation, place a mechanism to handle the creation of bad debt. This could be implemented as a reserve or by decreasing the supply index or similar, so the bad debt is redistributed by the suppliers.

thorseldon (BendDAO) disputed and commented:

The bad debt should be handled by the DAO Treasury and the protocol income.

0x73696d616f (warden) commented:

@0xTheC0der - There was no mention of the treasury handling bad debt at the time of the audit. As per the report, the bad debt is not handled because:

ERC20 tokens clear debt up to the available collateral, so if debt > collateral, this leftover debt will never be handled.

ERC721 tokens are not profitable for liquidation (separate issue here ).

Also, for ERC20 tokens, the treasury and protocol income can not cover the bad debt because it is not possible to clear the extra debt. It will revert here as the user has no collateral left. So the only option for the protocol to clear this bad debt would be gifting the user collateral to liquidate him, which is a very weird flow.

0xTheC0der (judge) commented:

Thanks for following up with more info after I initially closed due to insufficient proof.

The Warden has shown that the protocol seems to have no graceful way of handling bad debt position in case of collateral value crashes which are common in the DeFi space.

Also, the source of truth (codebase & README ) do not outline how these situations could be handled by the DAO treasury.

thorseldon (BendDAO) commented:

@0xTheC0der, @0x73696d616f - For the bad debt, DAO treasury should actively repay the debt using crossRepayERC20. I don’t know why reopen this finding? Most of the lending pool will has bad debt need to covered by DAO treasury or protocol income.

I think security audit should focus on the vulnerabilities in the code, not question the service model design.

0x73696d616f (warden) commented:

@thorseldon - it wasn’t publicly mentioned at the time of the audit you would be taking losses and handling bad debt directly, so the finding is in scope.

# [H-06] Users cannot unstake from YiedlETHStakingEtherfi.sol, because YieldAccount.sol is incompatible with ether.fi’s WithdrawRequestNFT.sol

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by oakcobalt, also found by 0x73696d616f and bin2chen In YiedlETHStakingEtherfi::stake, users yieldBorrow WETH, convert it to ETH, and stake in Ether.fi’s LiquidityPool. Ether.fi’s LiquidityPool will mint eETH (protocol token) to the user’s yieldAccount.

Users will unstake() by requesting withdrawal of staked ETH from ether.fi’s WithdrawRequestNFT::requestWithdraw.

The problem is WithdrawRequestNFT::requestWithdraw will _safeMint an NFT token to the user’s yieldAccount as proof of request withdrawal. But the user’s yieldAccount is not compatible with _safeMint because the implementation YieldAcount.sol doesn’t have onERC721Received method required by _safeMint.

//src/yield/YieldAccount.sol...

//@audit YieldAcount doesn't inherit openzeppelin's ERC721Holder.sol, neither does it implement onERC721Received. Not compatible with safeMint method required by ether.fi's WithdrawRequestNFT.

contract YieldAccount is IYieldAccount, Initializable {...

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/yield/YieldAccount.sol#L15
Flows:

YieldStakingBase::unstake → YieldEthStakingEtherfi::protocolRequestWithdrawal → liquidityPool::requestWithdraw → WithdrawRequestNFT::requestWithdraw → _safeMint ( recipient, requestId ) function _safeMint ( address to, uint256 tokenId, bytes memory data ) internal virtual { _mint ( to, tokenId ); require ( _checkOnERC721Received ( address ( 0 ), to, tokenId, data ), "ERC721: transfer to non ERC721Receiver implementer" ); }

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC721/ERC721.sol#L315
Because yieldAcount.sol is cloned for every user and atomically used as caller address for ether.fi interactions, no one can unstake from YiedlETHStakingEtherfi.sol. User funds, yield and nft collaterals will be locked. When users’ positions become unhealthy (e.g., nft token price drops), yieldBorrow debts compounding can also put the protocol at risks.

Note: In the unit test file for test/yield/YieldEthStakingEtherfi.t.sol, unit tests passed only because a MockEtherfiWithdrawRequestNFT.sol is used that uses _mint() instead of _safeMint().

## Recommended Mitigation Steps

Add onERC721Received support in YieldAccount.sol.

thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [H-07] Anyone can get the NFT collateral token after an Auction without bidding due to missing check on msg.sender

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

msg.sender Submitted by oakcobalt, also found by bin2chen and 0x73696d616f

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/IsolateLogic.sol#L477
In IsolateLogic.sol, liquidation of an isolate loan can only be placed after the auction period is passed with bidding. The problem is that there is a missing check on msg.sender in isolate liquidation flow, which allows anyone to take the collateral NFT token.

//src/libraries/logic/IsolateLogic.sol function executeIsolateLiquidate (InputTypes.ExecuteIsolateLiquidateParams memory params ) internal {...

if ( params.

supplyAsCollateral ) { //@audit due to no check on msg.sender, anyone calls isolateLiquidate will get the collateral nft |> VaultLogic.

erc721TransferIsolateSupplyOnLiquidate ( nftAssetData, params.

msgSender, params.

nftTokenIds ); } else { VaultLogic.

erc721DecreaseIsolateSupplyOnLiquidate ( nftAssetData, params.

nftTokenIds ); |> VaultLogic.

erc721TransferOutLiquidity ( nftAssetData, params.

msgSender, params.

nftTokenIds ); }...

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/IsolateLogic.sol#L473
Flows:

IsolateLiquidation::isolateLiquidate -> IsolateLogic.executeIsolateLiquidate(). Note that msg.sender is passed from isolateLiquidate() to the end of executeIsolateLiquidate() without any checks.

## Recommended Mitigation Steps

Add check msg.Sender is the lastBidder.

## Assessed type

Access Control thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [H-08] The bot won’t be able to unstake or repay risky positions in the yield contract

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by SpicyMeatball, also found by oakcobalt and bin2chen

- https://github.com/code-423n4/2024-07-benddao/blob/main/src/yield/YieldStakingBase.sol#L312
- https://github.com/code-423n4/2024-07-benddao/blob/main/src/yield/YieldStakingBase.sol#L384

## Impact

When botAdmin attempts to unstake and repay risky positions, wrong account address will be used, causing the transaction to fail.

## Recommended Mitigation Steps

Allow specifying user address in unstake and repay functions:

+ function _unstake(uint32 poolId, address nft, address user, uint256 tokenId, uint256 unstakeFine) internal virtual { UnstakeLocalVars memory vars; + vars.yieldAccout = IYieldAccount(yieldAccounts[user]); + require(user == msg.sender || botAdmin == msg.sender, Errors.INVALID_CALLER); require(address(vars.yieldAccout) != address(0), Errors.YIELD_ACCOUNT_NOT_EXIST); thorseldon (BendDAO) confirmed and commented:

Fixed here.

Medium Risk Findings (20)

# [M-01] PriceOracle has invalid checks on price staleness

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by oakcobalt, also found by Ch_301, bin2chen, and SpicyMeatball There are two checks on price staleness in PriceOracle::getAssetPriceFromChainlink, but both checks are invalid.

updatedAt != 0 - In chainlink aggregator, the price is updated at a set heartbeat and a threshold of deviation.

updatedAt should be used to check if the answer is within the heartbeat or acceptable time limits. See doc.

answeredInRound >= roundId - answeredInRound is deprecated and shouldn’t be used. See doc.

//src/PriceOracle.sol /// @notice Query the price of asset from chainlink oracle function getAssetPriceFromChainlink ( address asset ) public view returns ( uint256 ) { AggregatorV2V3Interface sourceAgg = assetChainlinkAggregators [ asset ]; require ( address ( sourceAgg ) != address ( 0 ), Errors.

ASSET_AGGREGATOR_NOT_EXIST ); ( uint80 roundId, int256 answer,, uint256 updatedAt, uint80 answeredInRound ) = sourceAgg.

latestRoundData (); require ( answer > 0, Errors.

ASSET_PRICE_IS_ZERO ); |> require ( updatedAt != 0, Errors.

ORACLE_PRICE_IS_STALE ); |> require ( answeredInRound >= roundId, Errors.

ORACLE_PRICE_IS_STALE ); return uint256 ( answer ); }

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/PriceOracle.sol#L124-L125

## Recommended Mitigation Steps

Consider using asset-specific heartbeat (e.g., ETH/USD has 1 hour heartbeat) and check against ( block.timestamp - updatedAt ).

## Assessed type

Oracle thorseldon (BendDAO) acknowledged and commented:

Checking the stale interval or grace period of oracle price, it’s maybe better do this as suggested, but it’s hard to set or predict the appropriate interval time.

We suggest adjust the severity level to informative.

0xTheC0der (judge) commented:

Historically, findings like this which can have a price impact were awarded with Medium severity on C4.

# [M-02] Risk of mass liquidation after pool/asset pause and unpause, due to borrow interest compounding implementation

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by oakcobalt Current pausing features allow a pool to be globally paused (all assets borrow, repay, liquidation pausing) and asset pausing (specific asset borrow, repay, liquidation pausing). See validatePoolBasic() and validateAssetBasic() which are checked in borrow, repay, and will revert if the pool or the asset is paused.

The problem is the borrow interest compounding implementation ( InterestLogic::_updateBorrowIndex ) will not factor in the duration of pausing when a user cannot repay a loan.

//src/libraries/logic/InterestLogic.sol function _updateBorrowIndex (DataTypes.AssetData storage assetData, DataTypes.GroupData storage groupData ) internal { // borrow index only gets updated if there is any variable debt.

// groupData.borrowRate != 0 is not a correct validation, // because a positive base variable rate can be stored on // groupData.borrowRate, but the index should not increase if (( groupData.

totalScaledCrossBorrow != 0 ) || ( groupData.

totalScaledIsolateBorrow != 0 )) { //@audit Case: asset paused and unpaused, assetData.lastUpdateTimestamp will be a timestamp before paused, interests compound during pausing |> uint256 cumulatedBorrowInterest = MathUtils.

calculateCompoundedInterest ( groupData.

borrowRate, assetData.

lastUpdateTimestamp ); uint256 nextBorrowIndex = cumulatedBorrowInterest.

rayMul ( groupData.

borrowIndex ); groupData.

borrowIndex = nextBorrowIndex.

toUint128 (); }

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/InterestLogic.sol#L281
Interests continuously compound with no repayment allowed. When unpaused, many accounts are vulnerable to immediate liquidation out of users’ control.

## Recommended Mitigation Steps

Consider storing pause/unpause timestamp to allow skipping borrow interests compounding for the duration of the pause.

thorseldon (BendDAO) acknowledged and commented:

It’s our service design. Pause & unpause should be only used for emergency case, the accrued interest should be very small.

We suggest adjust the severity level to Informative.

0xTheC0der (judge) commented:

Although pause durations are intended to be small, they can be arbitrarily long and put users at risk on unpausing due to compounding in the meantime. This could be mitigated on contract level; therefore, leaning towards maintaining Medium severity.

# [M-03] If an isolated borrower/bidder is blacklisted by the debt token, risk of DOS liquidation/auction of the corresponding loan

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by oakcobalt, also found by bin2chen and Ch_301 If a debt asset has a blacklisting feature (e.g., USDC), and an isolated borrower/bidder is blacklisted by the asset token, isolated liquidation/auction of the corresponding loan can be DOSsed.

When a user is blacklisted by an erc20 token, the asset cannot be transferred in or out of the blacklisted user’s account.

A bidder is blacklisted after they placed the bid for an isolated loan auction through IsolateLogic::executeIsolateAuction.

When the next bidder outbids by calling IsolateLogic::executeIsolateAuction, the previous bidder’s bid amount is returned. Because previous bidder is blacklisted after placing the bid, VaultLogic.erc20TransferOutBidAmount(debtAssetData, vars.oldLastBidder, vars.oldBidAmount) will revert the auction tx. At this point, no one can bid on the loan.

//src/libraries/logic/IsolateLogic.sol function executeIsolateAuction (InputTypes.ExecuteIsolateAuctionParams memory params ) internal {...

// transfer last bid amount to previous bidder from escrow if (( vars.

oldLastBidder != address ( 0 )) && ( vars.

oldBidAmount > 0 )) { VaultLogic.

erc20TransferOutBidAmount ( debtAssetData, vars.

oldLastBidder, vars.

oldBidAmount ); }...

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/IsolateLogic.sol#L262
The borrower is blacklisted by the debt token.

After the auction ends, when the liquidator calls executeIsolateLiquidate(), if the last bid amount is greater than total debt, the extra amount (last bid amount - total debt) has to be transferred to the borrower in the current implementation.

//src/libraries/logic/IsolateLogic.sol function executeIsolateLiquidate (InputTypes.ExecuteIsolateLiquidateParams memory params ) internal {...

// transfer remain amount to borrower if ( vars.

remainBidAmounts [ vars.

nidx ] > 0 ) { VaultLogic.

erc20TransferOutBidAmount ( debtAssetData, tokenData.

owner, vars.

remainBidAmounts [ vars.

nidx ]); }...

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/IsolateLogic.sol#L444
However, VaultLogic.erc20TransferOutBidAmount(debtAssetData, tokenData.owner, vars.remainBidAmounts[vars.nidx]) will revert due to the original borrower ( tokenData.owner ) is blacklisted.

## Recommended Mitigation Steps

In the isolate auction or liquidation flow, consider using try-catch block to handle VaultLogic.erc20TransferOutBidAmount.

thorseldon (BendDAO) acknowledged and commented:

We suggest adjust the severity level to Informative.

0xTheC0der (judge) commented:

According to the README, which is a source of truth in this audit, Pausability and Blocklists (blacklisting) of ERC20 tokens are in scope.

Therefore, I have to treat this as a viable risk and cannot justify a downgrade of the issue, even if not immediately relevant in the vast majority of practical cases.

# [M-04] Unhandled request invalidation by the owner of Etherfi will lead to stuck debt

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by 0x73696d616f The owner of Etherfi has the ability to invalidate withdrawal requests, which will DoS YieldEthStakingEtherfi::protocolClaimWithdraw() without ever repaying the debt. Additionally, Etherfi has a mechanism to manually claim the invalidated withdrawal, which will leave the withdraw request in a DoSed state in YieldEthStakingEtherfi.

## Recommended Mitigation Steps

Create a mechanism to handle invalidated withdrawals or withdrawals that have been claimed by the owner.

thorseldon (BendDAO) acknowledged and commented:

After carefully reading the WithdrawRequestNFT code, we decided that we could only contact Etherfi and manually handle this special case of InvalidRequest according to the actual situation. Only after understanding the details can we give a reasonable solution. We can trust that Etherfi will not maliciously invalidate requests.

We suggest adjust the severity level to Informative.

# [M-05] wrapNativeTokenInWallet() always reverts on Arbitrum

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

wrapNativeTokenInWallet() always reverts on Arbitrum Submitted by bin2chen VaultLogic.wrapNativeTokenInWallet() is used in a number of places It’s mainly used to store msg.value into WETH and then transfer it to msg.sender.

function wrapNativeTokenInWallet ( address wrappedNativeToken, address user, uint256 amount ) internal { require ( amount > 0, Errors.

INVALID_AMOUNT ); IWETH ( wrappedNativeToken ).

deposit {value:

amount }(); @> bool success = IWETH ( wrappedNativeToken ).

transferFrom ( address ( this ), user, amount ); require ( success, Errors.

TOKEN_TRANSFER_FAILED ); } The token transfer is done using the transferFrom method. This works fine on most chains (Ethereum, Optimism, Polygon, BSC) which use the standard WETH9 contract that handles the case src == msg.sender:

WETH9.

sol if ( src != msg.

sender && allowance [ src ][ msg.

sender ] != uint (- 1 )) { require ( allowance [ src ][ msg.

sender ] >= wad ); allowance [ src ][ msg.

sender ] -= wad; } The problem is that the WETH implementation on Arbitrum uses a different contract, and does not have this src == msg.sender handling.

## Impact

wrapNativeTokenInWallet() revert, BVault/CrossLending...

contracts that use this method don’t execute correctly.

## Recommended Mitigation

function wrapNativeTokenInWallet(address wrappedNativeToken, address user, uint256 amount) internal { require(amount > 0, Errors.INVALID_AMOUNT); IWETH(wrappedNativeToken).deposit{value: amount}(); - bool success = IWETH(wrappedNativeToken).transferFrom(address(this), user, amount); + bool success = IWETH(wrappedNativeToken).transfer(user, amount); require(success, Errors.TOKEN_TRANSFER_FAILED); }

## Assessed type

Context thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-06] YieldEthStakingLido lacks a limit on the max stake amount, which may result in the unstake exceeding MAX_STETH_WITHDRAWAL_AMOUNT , resulting in the token not being retrieved

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

YieldEthStakingLido lacks a limit on the max stake amount, which may result in the unstake exceeding MAX_STETH_WITHDRAWAL_AMOUNT, resulting in the token not being retrieved Submitted by bin2chen, also found by oakcobalt In YieldEthStakingLido, the main processes are as follows:

stake() -> stEth balance increase unstake () -> unstETH.requestWithdrawals(allShares) repay() -> unstETH.claimWithdrawal() The issue is the second step, unstETH is with a maximum withdrawal limit:

MAX_STETH_WITHDRAWAL_AMOUNT contract YieldEthStakingLido is YieldStakingBase {...

function protocolRequestWithdrawal ( YieldStakeData storage sd ) internal virtual override { IYieldAccount yieldAccount = IYieldAccount ( yieldAccounts [ msg.

sender ]); uint256 [] memory requestAmounts = new uint256 []( 1 ); requestAmounts [ 0 ] = sd.

withdrawAmount; bytes memory result = yieldAccount.

execute ( address ( unstETH ), @> abi.

encodeWithSelector ( IUnstETH.

requestWithdrawals.

selector, requestAmounts, address ( yieldAccount )) ); uint256 [] memory withdrawReqIds = abi.

decode ( result, ( uint256 [])); require ( withdrawReqIds.

length > 0 && withdrawReqIds [ 0 ] > 0, Errors.

YIELD_ETH_WITHDRAW_FAILED ); sd.

withdrawReqId = withdrawReqIds [ 0 ]; }

- https://docs.lido.fi/contracts/withdrawal-queue-erc721/#requestwithdrawals
each amount in _amounts must be greater than or equal to MIN_STETH_WITHDRAWAL_AMOUNT and lower than or equal to MAX_STETH_WITHDRAWAL_AMOUNT.

Current configuration here.

/// @notice maximum amount of stETH that is possible to withdraw by a single request /// Prevents accumulating too much funds per single request fulfillment in the future.

/// @dev To withdraw larger amounts, it's recommended to split it to several requests uint256 public constant MAX_STETH_WITHDRAWAL_AMOUNT = 1000 * 1e18; lido suggests that if it exceeds this value, it needs to be taken in batches, but YieldEthStakingLido.sol can only be taken at once. If the stake amount exceeds this value, it will not be possible to unstake() and the token will be locked in the contract.

Assumption:

leverageFactor = 50000, eth price = $2500 As long as the value of the nft is greater than 1000 * $2500 / 5 = $500,000 it will be possible to stake() more than MAX_STETH_WITHDRAWAL_AMOUNT. After that, when the user performs an unstake() it will fail, causing the token to be locked.

## Impact

Excessive amount of stake will result in failure to unstake().

## Recommended Mitigation

YieldStakingBase add method checkStakeAmount. Each sub contract override implements its own maximum amount limit, YieldEthStakingLido suggests a maximum of MAX_STETH_WITHDRAWAL_AMOUNT/2.

## Assessed type

Context 0xTheC0der (judge) decreased severity to Medium and commented:

Downgrading to Medium due to reduced likelihood of such great stake amounts.

thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-07] executeYieldBorrowERC20() checking yieldCap is wrong

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

executeYieldBorrowERC20() checking yieldCap is wrong Submitted by bin2chen, also found by oakcobalt In executeYieldBorrowERC20(), we will check whether stakerAddr exceeds yieldCap.

function executeYieldBorrowERC20 (InputTypes.ExecuteYieldBorrowERC20Params memory params ) internal {...

ValidateLogic.

validateYieldBorrowERC20 ( params, poolData, assetData, groupData ); @> vars.

totalSupply = VaultLogic.

erc20GetTotalCrossSupply ( assetData, groupData.

borrowIndex ); // check asset level yield cap limit vars.

totalBorrow = VaultLogic.

erc20GetTotalCrossBorrowInGroup ( groupData, groupData.

borrowIndex ); require ( vars.

totalBorrow + params.

amount ) <= vars.

totalSupply.

percentMul ( assetData.

yieldCap ), Errors.

YIELD_EXCEED_ASSET_CAP_LIMIT ); // check staker level yield cap limit vars.

stakerBorrow = VaultLogic.

erc20GetUserCrossBorrowInGroup ( groupData, vars.

stakerAddr, groupData.

borrowIndex ); require ( vars.

stakerBorrow + params.

amount ) <= vars.

totalSupply.

percentMul ( ymData.

yieldCap ), Errors.

YIELD_EXCEED_STAKER_CAP_LIMIT ); The code above, calculating totalSupply using totalSupply = totalScaledCrossSupply * groupData.borrowIndex to calculate supply is wrong. The correct way to calculate supply is to use:

assetData.supplyIndex.

## Impact

With an incorrect value for totalSupply, the check for yieldCap will be inaccurate, leading to a security risk.

## Recommended Mitigation

function executeYieldBorrowERC20(InputTypes.ExecuteYieldBorrowERC20Params memory params) internal {...

ValidateLogic.validateYieldBorrowERC20(params, poolData, assetData, groupData); - vars.totalSupply = VaultLogic.erc20GetTotalCrossSupply(assetData, groupData.borrowIndex); + vars.totalSupply = VaultLogic.erc20GetTotalCrossSupply(assetData, assetData.supplyIndex); // check asset level yield cap limit vars.totalBorrow = VaultLogic.erc20GetTotalCrossBorrowInGroup(groupData, groupData.borrowIndex);

## Assessed type

Context thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-08] Fee-on-Transfer tokens cause problems in multiple places

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by Ch_301 The protocol is designed to ensure it is compatible with fee-on-transfer tokens (Fee-on-transfer is in scope). For example, the erc20TransferInLiquidity() function checks the balance before and after, and calculates the difference between these values to measure the tokens received File:

VaultLogic.

sol 448:

function erc20TransferInBidAmount (DataTypes.AssetData storage assetData, address from, uint256 amount ) internal { 449:

address asset = assetData.

underlyingAsset; 450:

uint256 poolSizeBefore = IERC20Upgradeable ( asset ).

balanceOf ( address ( this )); 451:

452:

assetData.

totalBidAmout += amount; 453:

454:

IERC20Upgradeable ( asset ).

safeTransferFrom ( from, address ( this ), amount ); 455:

456:

uint256 poolSizeAfter = IERC20Upgradeable ( asset ).

balanceOf ( address ( this )); 457:

require ( poolSizeAfter == ( poolSizeBefore + amount ), Errors.

INVALID_TRANSFER_AMOUNT ); 458: } But at the end of the logic, it has a required check if the contract received the exact intended amount these two features are not compatible If the asset is Fee-on-transfer tokens the requirement will keep revert Example:

Contract balanceOf is 2000 of Fee-on-transfer tokens.

Contract transfer from user amount == 1000.

Fee is collected, so the contract received only 950 tokens, but the require will check 2000 + 1000 == 2950 and revert.

## Impact

Any function/feature that calls:

erc20TransferInLiquidity() erc20TransferOutLiquidity() erc20TransferBetweenWallets() erc20TransferOutBidAmount() erc20TransferInBidAmount() in VaultLogic.sol will revert if the asset is Fee-on-transfer tokens.

## Recommended Mitigation Steps

When fee-on-transfer tokens should be supported, you need to check the actual balance differences but don’t expect to receive the exact amount. If they are not supported, this should be documented.

## Assessed type

ERC20 thorseldon (BendDAO) disputed and commented:

We suggest adjust the severity level to Informative.

For now we don’t support the token fee-on-transfer. Maybe it’s will be optimised in future if we really need to support those tokens.

0xTheC0der (judge) commented:

I fully understand your point. However, the audit README states that Fee on transfer ERC20s are in scope. The audit README is part of the source of truth for wardens during this audit and I have to judge according to that. Therefore, I cannot legitimately downgrade this finding even if it’s not of relevance.

# [M-09] User are forced to borrow again in order to unlock their NFTs from IsolateLending.sol

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

IsolateLending.sol Submitted by Ch_301 Isolated Margin Lending the BendDAO protocol will calculate the health factor based on a single collateral (one NFT), the liquidation of collateral will be achieved by selling the NFT through an auction mechanism.

However, the borrower who gets liquidated (his NFT is in the auction state) still able to repay his loan and close the auction, by triggering IsolateLiquidation.sol#isolateRedeem(). Each asset has a redemption threshold redeemThreshold to define the minimum debt repayment amount by the borrower.

File:

IsolateLogic.

sol # executeIsolateRedeem () vars.

redeemAmounts [ vars.

nidx ] = vars.

borrowAmount.

percentMul ( nftAssetData.

redeemThreshold ); The redeemThreshold is updateable from Configurator.sol#setAssetAuctionParams() with a max value of 10_000. In case the user calls IsolateLiquidation.sol#isolateRedeem() and nftAssetData.redeemThreshold is 10_000 ( MAX_REDEEM_THRESHOLD ). This will be set scaledAmount to zero.

File:

IsolateLogic.

sol # executeIsolateRedeem () loanData.

scaledAmount -= vars.

amountScaled; This means the debt is fully paid; however, The loanData.loanStatus will still be in active state Constants.LOAN_STATUS_ACTIVE.

The erc721TokenData.LockerAddr is not updated to address(0).

So, the user is not able to withdraw his NFT from BVault.sol or even update the loan state and LockerAddr values through IsolateLending.sol#isolateRepay() due to this check Note: The only way the user can unlock his NFT is by borrowing again. This will increase loanData.scaledAmount again to greater than zero, and then he needs to repay that loan IsolateLending.sol#isolateRepay() to update the erc721TokenData.LockerAddr to address(0). This will expose the user to the unnecessary risk of being liquidated again.

## Impact

Users are forced to borrow again to unlock their NFTs after Isolate Redeem.

## Recommended Mitigation Steps

In case loanData.scaledAmount == 0 call VaultLogic.erc721SetTokenLockerAddr(nftAssetData, params.nftTokenIds[vars.nidx], address(0));.

thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-10] isolateRedeem() revert in case Revert-on-zero-value-transfers tokens

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

isolateRedeem() revert in case Revert-on-zero-value-transfers tokens Submitted by Ch_301 One of the features of this protocol is that the borrower can redeem his loan (under the Isolate Lending ) after his loan goes into auction state (before the end of the auction) by simply invoking IsolateLiquidation.sol#isolateRedeem(). However, in order to keep the liquidators incentivized to launch the auction for any bad debt, the borrower could get forced to pay them some fee (called bidFine ).

The bidFine is defined by two factors bidFineFactor and minBidFineFactor, both of them are updatable from Configurator.sol#setAssetAuctionParams(), In case admin set them to zero, when the borrower tries to redeem his loan this logic from IsolateLogic.sol#executeIsolateRedeem():

File:

IsolateLogic.

sol # executeIsolateRedeem () 328: (, vars.

bidFines [ vars.

nidx ]) = GenericLogic.

calculateNftLoanBidFine ( 329:

debtAssetData, 330:

debtGroupData, 331:

nftAssetData, 332:

loanData, 333:

vars.

priceOracle 334: ); will set the value of vars.bidFines[vars.nidx] to zero. After that, the flow will enter this IF block to transfer the bidFine to the liquidator who launched the auction.

File:

IsolateLogic.

sol # executeIsolateRedeem () 346:

if ( loanData.

firstBidder != address ( 0 )) { 347:

// transfer bid fine from borrower to the first bidder 348:

VaultLogic.

erc20TransferBetweenWallets ( 349:

params.

asset, 350:

params.

msgSender, 351:

loanData.

firstBidder, 352:

vars.

bidFines [ vars.

nidx ] 353: ); 354: } In case params.asset is one of Revert-on-Zero-Value-Transfers tokens (in scope), the transaction will revert because it is trying to transfer zero value vars.bidFines[vars.nidx] == 0.

## Impact

The borrower will never be able to redeem his loan after his loan goes into auction state.

## Recommended Mitigation Steps

File: IsolateLogic.sol#executeIsolateRedeem() - 346: if (loanData.firstBidder != address(0)) { + 346: if (loanData.firstBidder != address(0) && vars.bidFines[vars.nidx] > 0) { 347: // transfer bid fine from borrower to the first bidder 348: VaultLogic.erc20TransferBetweenWallets( 349: params.asset, 350: params.msgSender, 351: loanData.firstBidder, 352: vars.bidFines[vars.nidx] 353: ); 354: }

## Assessed type

ERC20 thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-11] Updating asset collateral params can lead to liquidate borrowers arbitrarily

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by Ch_301, also found by Ch_301 One of the key concepts in this protocol is Cross Lending when the contract will calculate the health-factor of the account. If it is unhealthy, the liquidator can repay the debt on behalf of the borrower and take their collateral assets at a certain discount price.

The protocol has two main factors to calculate the health-factor for users; the collateralFactor and liquidationThreshold values are unique for each pool and asset. Also, the PoolAdmin can update them at any time by triggering Configurator.sol#setAssetCollateralParams().

The responsibility for computing health-factor is GenericLogic.sol#calculateUserAccountData(), which calculates the user data across the reserves. However, the current logic uses liquidationThreshold value also to check if the asset will not be used as collateral (but still, users can supply/lend it to be borrowed and earn interest).

File:

GenericLogic.

sol # calculateUserAccountData () if ( currentAssetData.

liquidationThreshold != 0 ) { /***/ result.

totalCollateralInBaseCurrency += vars.

userBalanceInBaseCurrency; /***/ } The issue is the PoolAdmin can call Configurator.sol#setAssetCollateralParams() at any time to disable accepting an asset as collateral by updating collateralFactor and liquidationThreshold to zero value. So, users suddenly will get liquidated directly by MEV bots.

## Impact

The PoolAdmin can update the asset collateral params. This will leave borrowers with bad debt and allow the liquidators to liquidate all the loans.

MEV bots will just back-run the setAssetCollateralParams() transaction to liquidate users; they have no chance to update their positions.

## Recommended Mitigation Steps

In case the PoolAdmin decides to no longer accept an asset as collateral, users who are using this asset as collateral should have a period of time to update their loans. You can use a buffer period or lock time to do this critical update.

thorseldon (BendDAO) disputed 0xTheC0der (judge) commented:

Seems like a valid concern. @thorseldon, could you please provide more context concerning your dispute?

After a second thought, this seems to fall below Centralisation Risk on Contracts which has Owner or Administrator according to the README.

0xTheC0der (judge) commented:

After a third thought, this is exceeding centralization risks, since it’s never a good time to change asset collateral params. The params should be cached for ongoing borrows.

thorseldon (BendDAO) commented:

This a parameter configuration which controlled by the DAO governance, any parameters like liquidation threshold should be carefully discussed and reviewed the community and dev team.

We will use timelock to schedule the configuration updating if the proposal voting is passed.

0xTheC0der (judge) commented:

I understand the centralization concern of this issue is out of scope and changes are carefully reviewed. However, on a contract level:

Even carefully considered changes do directly affect open borrow positions. Therefore, there is never/seldom a good time to change these values.

It is not ensured on a contract level that e.g. liquidation threshold changes can only happen in favor of the users and not against them.

For these reasons, Medium severity seems justified.

# [M-12] No check if Arbitrum/Optimism L2 sequencer is down in Chainlink feeds PriceOracle.sol

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

PriceOracle.sol Submitted by Ch_301, also found by oakcobalt and bin2chen Chainlink recommends that all Optimistic L2 oracles consult the Sequencer Uptime Feed to ensure that the sequencer is live before trusting the data returned by the oracle.

If the Arbitrum Sequencer goes down, oracle data will not be kept up to date, and thus could become stale. However, users are able to continue to interact with the protocol directly through the L1 optimistic rollup contract. You can review Chainlink docs on L2 Sequencer Uptime Feeds for more details on this.

This protocol uses getPriceOracle() everywhere and it sub call to PriceOracle.sol#latestRoundData(). As a result, users may be able to use the protocol while Oracle feeds are stale. This could cause many problems, for example:

A user has a loan with 100 tokens, valued at 1 ETH each, and no borrows.

The Arbitrum sequencer goes down temporarily.

While it’s down, the price of the token falls to 0.5 ETH each.

The current value of the user’s loan is 50 ETH, so the liquidators should be able to liquidate this position because it is unhealthy.

However, due to the stale price, the protocol lets the user keep borrowing.

## Impact

If the Arbitrum sequencer goes down, the protocol will allow users to continue to operate at the previous (stale) prices. This will break all the core functionality in the protocol.

Tools Used Solodit

## Recommended Mitigation Steps

It is recommended to follow the code example of Chainlink here or just add this check:

File:

PriceOracle.

sol function getAssetPriceFromChainlink ( address asset ) public view returns ( uint256 ) { AggregatorV2V3Interface sourceAgg = assetChainlinkAggregators [ asset ]; require ( address ( sourceAgg ) != address ( 0 ), Errors.

ASSET_AGGREGATOR_NOT_EXIST ); if (!

isSequencerActive ()) revert Errors.

L2SequencerUnavailable ();...

} function isSequencerActive () internal view returns ( bool ) { (, int256 answer, uint256 startedAt,,) = sequencer.

latestRoundData (); if ( block.

timestamp - startedAt <= GRACE_PERIOD_TIME || answer == 1 ) return false; return true; }

## Assessed type

Oracle thorseldon (BendDAO) acknowledged and commented:

Checking the stale interval or grace period of oracle price, it’s maybe better do this as suggested, but it’s hard to set or predict the appropriate interval time.

We suggest adjust the severity level to informative.

# [M-13] Major insolvency risk in LiquidationLogic::executeCrossLiquidateERC721() due to not setting a maximum liquidation price

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

LiquidationLogic::executeCrossLiquidateERC721() due to not setting a maximum liquidation price Submitted by 0x73696d616f Whenever the price of a nft suffers a significant drop or the debt asset increases in price, there is a threshold after which liquidations are not profitable at all for the liquidator. Moreover, it does not offer the possibility of partial liquidation to at least partially mitigate this issue.

## Recommended Mitigation Steps

Cap the price per nft of the liquidation to a reasonable value so liquidators can at least repay some of the token ids of the position. This is how it works for ERC20 liquidations, for example, in which the maximum debt to repay is calculated based on the maximum collateral balance sold at a discount.

thorseldon (BendDAO) acknowledged and commented:

In our current service design: We want NFT assets to bear corresponding debts according to their collateral value, and try to avoid the bad debt.

The final bad debt will be covered the DAO treasury which comes from protocol income.

We suggest adjust the severity level to informative, because this belong to service design which can be optimised in the future.

0xTheC0der (judge) commented:

Having the DAO treasury cover bad debt from protocol income is a protocol internal design decisions which doesn’t immediately put users at risk or allows direct theft of funds. Therefore, I agree with the sponsor.

Seems to fall below Questioning the protocol’s business model is unreasonable.

0x73696d616f (warden) commented:

@0xTheC0der - the protocol means they will take unfavourable liquidations to cover for their users. This was not documented so the issue should be valid, as it is a serious risk for BendDao.

As far as the docs and code were concerned (not mentioning treasury intervention to clear bad debt), this is a high severity issue.

0xTheC0der (judge) decreased severity to Medium and commented:

Similar reasoning as in Issue #23. But ERC721 instead of ERC20 instance.

However, Medium severity seems more justified in this case given the discussed impacts in contrast to #23 which discusses impacts more thoroughly (e.g., withdrawal DoS).

# [M-14] Incorrect accounting of utilization, supply/borrow rates due to vulnerable implementation in IsolateLogic::executeIsolateLiquidate

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

IsolateLogic::executeIsolateLiquidate Submitted by oakcobalt When an isolate loan (borrow against a single nft token) is auctioned and liquidated, the liquidator (caller of IsolateLogic::executeIsolateLiquidate ) may need to repay extra amount of debt ( vars.totalExtraAmount ) if the bid amount doesn’t fully cover total debt ( vars.totalBorrowAmount ).

The problem is the above case is not correctly implemented when interests rates are updated ( InterestLogic.updateInterestRates(poolData, debtAssetData, (vars.totalBorrowAmount + vars.totalExtraAmount), 0) ).

In InterestLogic::updateInterestRates, utilization is recalculated based on liquidityAdded and liquidityTaken. In the context of IsolateLogic::executeIsolateLiquidate, the liquidtyAdded for a loan is the repayment of debt ( vars.borrowAmount ) which includes loanData.bidAmount + extraAmount.

//src/libraries/logic/IsolateLogic.sol function executeIsolateLiquidate (InputTypes.ExecuteIsolateLiquidateParams memory params ) internal {...

for ( vars.

nidx = 0; vars.

nidx < params.

nftTokenIds.

length; vars.

nidx ++) {...

// Last bid can not cover borrow amount and liquidator need pay the extra amount if ( loanData.

bidAmount < vars.

borrowAmount ) { |> vars.

extraBorrowAmounts [ vars.

nidx ] = vars.

borrowAmount - loanData.

bidAmount; //@audit-info note: vars.borrowAmount = extraAmount + loanData.bidAmount }...

// update interest rate according latest borrow amount (utilization) //@audit (vars.totalBorrowAmount + vars.totalExtraAmount) is incorrect input, which inflates `liquidityAdded` in updateInterestRates.

|> InterestLogic.

updateInterestRates ( poolData, debtAssetData, ( vars.

totalBorrowAmount + vars.

totalExtraAmount ), 0 ); // bid already in pool and now repay the borrow but need to increase liquidity //@audit-info note: only total debt (vars.totalBorrowAmount) are added to liquidity.

|> VaultLogic.

erc20TransferOutBidAmountToLiqudity ( debtAssetData, vars.

totalBorrowAmount );...

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/IsolateLogic.sol#L461
In InterestLogic::updateInterestRates, ( vars.totalBorrowAmount + vars.totalExtraAmount ) inflates liquidityAdded, which inflates vars.availableLiquidityPlusDebt and deflates utilization rate vars.assetUtilizationRate. This will cause an inflated borrow rate/supply rate based on the interest rate model.

function updateInterestRates ( DataTypes.PoolData storage poolData, DataTypes.AssetData storage assetData, uint256 liquidityAdded, uint256 liquidityTaken ) internal {...

// calculate the total asset supply vars.

availableLiquidityPlusDebt = assetData.

availableLiquidity + |> liquidityAdded - liquidityTaken + vars.

totalAssetDebt; if ( vars.

availableLiquidityPlusDebt > 0 ) { vars.

assetUtilizationRate = vars.

totalAssetDebt.

rayDiv ( vars.

availableLiquidityPlusDebt ); }...

vars.

nextGroupBorrowRate = IInterestRateModel ( loopGroupData.

rateModel ).

calculateGroupBorrowRate ( vars.

loopGroupId, vars.

assetUtilizationRate );...

- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/InterestLogic.sol#L165
Flows:

IsolateLiquidation::isolateLiquidate -> IsolateLogic.executeIsolateLiquidate -> InterestLogic.updateInterestRates

## Recommended Mitigation Steps

Consider changing into InterestLogic.updateInterestRates(poolData, debtAssetData, vars.totalBorrowAmount, 0).

## Assessed type

Error thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-15] It’s impossible to retrieve collected fines from the yield staking contract

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by SpicyMeatball, also found by oakcobalt, 0x73696d616f, and bin2chen

- https://github.com/code-423n4/2024-07-benddao/blob/main/src/yield/YieldStakingBase.sol#L329-L337
- https://github.com/code-423n4/2024-07-benddao/blob/main/src/yield/YieldStakingBase.sol#L407
- https://github.com/code-423n4/2024-07-benddao/blob/main/src/yield/YieldStakingBase.sol#L426

## Impact

No means to retrieve collected fines from the staking contract.

## Recommended Mitigation Steps

Implement a function that allows admin to collect fines from the yield staking contract.

## Assessed type

Token-Transfer thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-16] Borrower can prevent yield position repayment and closure by the bot

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by SpicyMeatball, also found by 0x73696d616f User may revoke the underlying asset approval from the yield contract and prevent the botAdmin from repaying and closing his position. As a result, the debt will not be repaid, leaving the protocol at a loss.

## Recommended Mitigation Steps

It is recommended to separate the logic of debt repayment and NFT unlocking. The _repay function will reduce the debt of the position using the collected yield and unlock the NFT only if extraAmount is zero. In case extraAmount is greater than zero, leave the token locked and allow the NFT owner to unlock it only after he has repaid the remaining debt.

thorseldon (BendDAO) confirmed and commented:

Fixed here.

# [M-17] Changing auction duration will have effect on ongoing auctions

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by SpicyMeatball

- https://github.com/code-423n4/2024-07-benddao/blob/main/src/libraries/logic/ConfigureLogic.sol#L504
- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/IsolateLogic.sol#L320-L321
- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/IsolateLogic.sol#L423-L424

## Impact

Auctions created before the auction duration was changed will have their duration shortened/extended, which may catch their participants off guard.

## Recommended Mitigation Steps

Consider calculating and storing the auction end time instead of the start time:

- loanData.bidStartTimestamp = uint40(block.timestamp); + loanData.auctionEndTimestamp = uint40(block.timestamp) + nftAssetData.auctionDuration;

## Assessed type

Timing thorseldon (BendDAO) disputed and commented:

This is our service design as expected. We will try to avoid the adjust the parameters when some auctions going on. But maybe we want to adjust the auction duration sometimes when there’s auction triggered by some emergency cases; no one can predict the emergency cases in the future, but we keep this flexibility.

0xTheC0der (judge) commented:

Similar to my reasoning in Issue #26, I understand that the centralized aspect is intended, carefully considered and out of scope. However, on contract level these parameter changes can have implications that harm users, therefore Medium severity seems justified.

Nevertheless, I am open for further discussion (also with wardens) whether to consistently downgrade all of those DAO parameter change related issues or keep them.

# [M-18] Updating fee factor may create issues for the protocol

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by SpicyMeatball, also found by oakcobalt

- https://github.com/code-423n4/2024-07-benddao/blob/main/src/libraries/logic/ConfigureLogic.sol#L509-L521
- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/InterestLogic.sol#L248
- https://github.com/code-423n4/2024-07-benddao/blob/117ef61967d4b318fc65170061c9577e674fffa1/src/libraries/logic/InterestLogic.sol#L201-L202

## Impact

When the protocol fee is modified in the Configurator module:

The new fee percentage will be applied to the interest collected before the fee change.

The protocol will use the supply rate that was calculated using the old fee factor.

## Recommended Mitigation Steps

function executeSetAssetProtocolFee(address msgSender, uint32 poolId, address asset, uint16 feeFactor) internal { + InterestLogic.updateInterestIndexs(poolData, assetData); assetData.feeFactor = feeFactor; + InterestLogic.updateInterestRates(poolData, assetData, 0, 0); emit Events.SetAssetProtocolFee(poolId, asset, feeFactor); } thorseldon (BendDAO) confirmed and commented:

Fixed here.

We suggest adjust the severity level to Low Risk or Information, because there’s many TX action will frequently trigger to update the interest index, e.g., borrow/repay. We check the same logic exist in Aave V2 & V3.

0xTheC0der (judge) commented:

Thanks for that additional clarification about the likelihood of this being an issue.

Due to the possibility of fixing this and avoiding even temporary impacts and in consistency with other parameter change relates issues (see Issue #8 and Issue #26 ), I’d like to maintain Medium severity.

# [M-19] Protocol should update interest rate after changing rate model in the configurator module

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

Submitted by SpicyMeatball

- https://github.com/code-423n4/2024-07-benddao/blob/main/src/libraries/logic/ConfigureLogic.sol#L526
- https://github.com/code-423n4/2024-07-benddao/blob/main/src/libraries/logic/ConfigureLogic.sol#L626

## Impact

After updating the interest model, the protocol does not update the interest rate, resulting in interest being calculated at the old model rates for some time.

## Recommended Mitigation Steps

---SNIP--- DataTypes.GroupData storage groupData = assetData.groupLookup[groupId]; groupData.rateModel = rateModel_; + InterestLogic.updateInterestRates(poolData, assetData, 0, 0) emit Events.SetAssetLendingRate(poolId, asset, groupId, rateModel_); } thorseldon (BendDAO) confirmed and commented:

Fixed here.

We suggest adjust the severity level to Low Risk or Information, because there’s many TX action will frequently trigger to update the interest index, e.g., borrow/repay. We check the same logic exist in Aave V2 & V3.

0xTheC0der (judge) commented:

Thanks for adding that clarification!

Similar case as in Issue #7. The same reasoning applies here.

# [M-20] Incorrect unwrapNativeTokenInWallet receiver address

- **Contest:** BendDAO Invitational
- **Slug:** 2024-07-benddao-invitational
- **Finding ID:** M-20
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-benddao-invitational
- **Source snapshot:** competitions/2024-07-benddao-invitational/final_report.html

unwrapNativeTokenInWallet receiver address Submitted by SpicyMeatball, also found by oakcobalt

- https://github.com/code-423n4/2024-07-benddao/blob/main/src/modules/BVault.sol#L74
- https://github.com/code-423n4/2024-07-benddao/blob/main/src/modules/CrossLending.sol#L50
- https://github.com/code-423n4/2024-07-benddao/blob/main/src/modules/IsolateLending.sol#L51

## Impact

If the user specifies native tokens as output assets, the protocol will mistakenly try to unwrap wrapped tokens from the msg.sender instead of the receiver address that was specified in the function call.

## Recommended Mitigation Steps

if (isNative) { - VaultLogic.unwrapNativeTokenInWallet(asset, msgSender, amount); + VaultLogic.unwrapNativeTokenInWallet(asset, receiver, amount); }

## Assessed type

Token-Transfer thorseldon (BendDAO) confirmed and commented:

Fixed here.

## Rejected Primary Findings

# Rejected Primary Findings: BendDAO Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
