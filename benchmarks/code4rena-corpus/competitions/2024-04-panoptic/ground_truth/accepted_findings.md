# Accepted H/M Findings: Panoptic

# [H-01] SettleLongPremium is incorrectly implemented: premium should be deducted instead of added

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

SettleLongPremium is incorrectly implemented: premium should be deducted instead of added Submitted by pkqs90, also found by bin2chen, Aymen0909, 0xStalin, JecikPo, and DanielArmstrong

- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/PanopticPool.sol#L1621-L1640
- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/CollateralTracker.sol#L1043-L1089
SettleLongPremium is the function intended to settle premiums for long option holders. When called, it should deduct the premium from the option owner’s account, but the current implementation adds the premium instead.

Bug Description Let’s see the code for premium calculation. We can see that accumulatedPremium and s_options[owner][tokenId][legIndex] are premium accumulators for calculating the owed amount of premium, and that accumulatedPremium is a LeftRightUnsigned type, which means it must be positive.

The realizedPremia is also positive, because it is calculated by accumulatedPremium * liquidity.

The issue occurs when calling s_collateralToken.exercise(). The realizedPremia that is passed inside should be negative instead of positive, because negative means user pays premia, and positive means user receives premia. The current implementation is incorrect.

PanopticPool.sol accumulatedPremium = LeftRightUnsigned.

wrap ( 0 ).

toRightSlot ( premiumAccumulator0 ).

toLeftSlot ( premiumAccumulator1 ); // update the premium accumulator for the long position to the latest value // (the entire premia delta will be settled) LeftRightUnsigned premiumAccumulatorsLast = s_options [ owner ][ tokenId ][ legIndex ]; s_options [ owner ][ tokenId ][ legIndex ] = accumulatedPremium; > accumulatedPremium = accumulatedPremium.

sub ( premiumAccumulatorsLast ); } uint256 liquidity = PanopticMath.

getLiquidityChunk ( tokenId, legIndex, s_positionBalance [ owner ][ tokenId ].

rightSlot ()).

liquidity (); unchecked { // update the realized premia > LeftRightSigned realizedPremia = LeftRightSigned >.

wrap ( 0 ) >.

toRightSlot ( int128 ( int256 (( accumulatedPremium.

rightSlot () * liquidity ) / 2 ** 64 ))) >.

toLeftSlot ( int128 ( int256 (( accumulatedPremium.

leftSlot () * liquidity ) / 2 ** 64 ))); // deduct the paid premium tokens from the owner's balance and add them to the cumulative settled token delta s_collateralToken0.

exercise ( owner, 0, 0, 0, realizedPremia.

rightSlot ()); s_collateralToken1.

exercise ( owner, 0, 0, 0, realizedPremia.

leftSlot ()); CollateralTracker.sol function exercise ( address optionOwner, int128 longAmount, int128 shortAmount, int128 swappedAmount, int128 realizedPremium ) external onlyPanopticPool returns ( int128 ) { unchecked { // current available assets belonging to PLPs (updated after settlement) excluding any premium paid int256 updatedAssets = int256 ( uint256 ( s_poolAssets )) - swappedAmount; // add premium to be paid/collected on position close > int256 tokenToPay = - realizedPremium; // if burning ITM and swap occurred, compute tokens to be paid through exercise and add swap fees int256 intrinsicValue = swappedAmount - ( longAmount - shortAmount ); if (( intrinsicValue != 0 ) && (( shortAmount

!= 0 ) || ( longAmount != 0 ))) { // intrinsic value is the amount that need to be exchanged due to burning in-the-money // add the intrinsic value to the tokenToPay tokenToPay += intrinsicValue; } > if ( tokenToPay > 0 ) { // if user must pay tokens, burn them from user balance (revert if balance too small) uint256 sharesToBurn = Math.

mulDivRoundingUp ( uint256 ( tokenToPay ), totalSupply, totalAssets () ); _burn ( optionOwner, sharesToBurn ); > } else if ( tokenToPay < 0 ) { // if user must receive tokens, mint them uint256 sharesToMint = convertToShares ( uint256 (- tokenToPay )); _mint ( optionOwner, sharesToMint ); }

## Recommended Mitigation Steps

Take the negative of realizedPremia before calling s_collateralToken.exercise().

dyedm1 (Panoptic) confirmed via duplicate issue #376 Picodes (judge) commented:

Keeping High severity as funds are at stake.

# [H-02] Overflow in CollateralTracker allows minting shares for free

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

CollateralTracker allows minting shares for free Submitted by 0xLogos

- https://github.com/code-423n4/2024-04-panoptic/blob/833312ebd600665b577fbd9c03ffa0daf250ed24/contracts/CollateralTracker.sol#L478
- https://github.com/code-423n4/2024-04-panoptic/blob/833312ebd600665b577fbd9c03ffa0daf250ed24/contracts/CollateralTracker.sol#L461-L467

## Impact

Malicious actors can mint huge amounts of shares for free and then withdraw all collateral.

## Recommended Mitigation Steps

Remove unchecked block.

function maxMint(address) external view returns (uint maxShares) { return (convertToShares(type(uint104).max) * DECIMALS) / (DECIMALS + COMMISSION_FEE); }

## Assessed type

Under/Overflow dyedm1 (Panoptic) confirmed Medium Risk Findings (9)

# [M-01] PanopticFactory uses spot price when deploying new pools, resulting in liquidity manipulation when minting

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

PanopticFactory uses spot price when deploying new pools, resulting in liquidity manipulation when minting Submitted by DadeKuma, also found by sammy, Dup1337, Bauchibred, jesjupyter, and Vancelot When deployNewPool is called it uses the spot price of the pool, which can be manipulated through a flashloan and thus could return a highly inaccurate result.

The price is used when deciding how much liquidity should be minted for each token, so this can result in an unbalanced pool.

In other parts of the code, this is not an issue as there are oracles that prevent price manipulations, but in case there aren’t any checks to avoid so.

## Recommended Mitigation Steps

Consider using the TWAP price instead of the spot price.

## Assessed type

Uniswap dyedm1 (Panoptic) disputed and commented:

True, but I don’t see negative consequences for this? This function is just a way to add some full-range liquidity to the pool so the entire range can be swapped across, and depending on the tokens we can add very small/large amounts of liquidity anyway (mentioned in the readme: Depending on the token, the amount of funds required for the initial factory deployment may be high or unrealistic) Picodes (judge) commented:

@dyedm1 - assuming the deployer has infinite approvals, can’t we imagine a scenario where, by manipulating the spot pool price, it ends up depositing way too many token1 and getting sandwiched leading to a significant loss?

Said differently, the risk is that currently the deployer has no control over the amount of token1 he will donate and this amount can be manipulated by an attacker.

Picodes (judge) decreased severity to Medium and commented:

This is at most Medium to me considering pool deployers are advanced users and you need to deploy a pool where the manipulation cost is low which should remain exceptional.

dyedm1 (Panoptic) commented:

Yeah I agree this might be less than ideal if you have infinite approvals. Our UI doesn’t do infinite approvals to the factory, but some wallets allow users to edit the approval amount before signing the transaction, so it might be prudent to add slippage checks here (to make the process idiot-proof).

# [M-02] _validatePositionList() does not check for duplicate tokenIds, allowing attackers to bypass solvency checks

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

_validatePositionList() does not check for duplicate tokenIds, allowing attackers to bypass solvency checks Submitted by pkqs90, also found by Udsen, 0xLogos, and bin2chen

- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/PanopticPool.sol#L1367-L1391
- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/PanopticPool.sol#L887-L893

## Impact

The underlying issue is that _validatePositionList() does not check for duplicate tokenIds. Attackers can use this issue to bypass solvency checks, which leads to several impacts:

Users can mint/burn/liquidate/forceExercise when they are insolvent.

Users can settleLongPremium/forceExercise another user when the other user is insolvent.

This also conflicts a main invariant stated in audit readme:

Users should not be allowed to mint/burn options or pay premium if their end state is insolvent.

Bug Description First, let’s see why _validatePositionList does not check for duplicate tokenIds. For a user position hash, the first 8 bits is the length of tokenIds which overflows, last 248 bits is the xor hash. However, we can easily add 256 tokenIds of the same kind to create a duplicate positionHash.

For example:

Hash(key0, key1, key2) == Hash(key0, key1, key2, key0, key0,..., 256 more key0). This way, we can add any tokenId we want while still arriving the same position hash.

PanopticPool.sol function _validatePositionList ( address account, TokenId [] calldata positionIdList, uint256 offset ) internal view { uint256 pLength; uint256 currentHash = s_positionsHash [ account ]; unchecked { pLength = positionIdList.

length - offset; } // note that if pLength == 0 even if a user has existing position(s) the below will fail b/c the fingerprints will mismatch // Check that position hash (the fingerprint of option positions) matches the one stored for the '_account' uint256 fingerprintIncomingList; for ( uint256 i = 0; i < pLength; ) { > fingerprintIncomingList = PanopticMath.

updatePositionsHash ( fingerprintIncomingList, positionIdList [ i ], ADD ); unchecked { ++ i; } // revert if fingerprint for provided '_positionIdList' does not match the one stored for the '_account' if ( fingerprintIncomingList != currentHash ) revert Errors.

InputListFail (); } PanopticMath.sol function updatePositionsHash ( uint256 existingHash, TokenId tokenId, bool addFlag ) internal pure returns ( uint256 ) { // add the XOR`ed hash of the single option position `tokenId` to the `existingHash` // @dev 0 ^ x = x unchecked { // update hash by taking the XOR of the new tokenId uint248 updatedHash = uint248 ( existingHash ) ^ ( uint248 ( uint256 ( keccak256 ( abi.

encode ( tokenId ))))); // increment the top 8 bit if addflag=true, decrement otherwise return addFlag ?

uint256 ( updatedHash ) + ((( existingHash >> 248 ) + 1 ) << 248 ):

uint256 ( updatedHash ) + ((( existingHash >> 248 ) - 1 ) << 248 ); } Then, let’s see how duplicate ids can bypass solvency check. The solvency check is in _validateSolvency(), which is called by all the user interaction functions, such as mint/burn/liquidate/forceExercise/settleLongPremium. This function first checks for a user passed in positionIdList (which we already proved can include duplicates), then calls _checkSolvencyAtTick() to calculate the balanceCross (collateral balance) and thresholdCross (required collateral) for all tokens.

The key is the collateral balance includes the premium that is collected for each of the positions. For most of the positions, the collected premium should be less than required collateral to keep this position open. However, if a position has been open for a long enough time, the fees it accumulated may be larger than required collateral.

For this kind of position, we can duplicate it 256 times (or multiple of 256 times, as long as gas fee is enough), and make our collateral balance grow faster than required collateral. This can make a insolvent account “solvent”, by duplicating the key tokenId multiple times.

function _validateSolvency ( address user, TokenId [] calldata positionIdList, uint256 buffer ) internal view returns ( uint256 medianData ) { // check that the provided positionIdList matches the positions in memory > _validatePositionList ( user, positionIdList, 0 );...

// Check the user's solvency at the fast tick; revert if not solvent bool solventAtFast = _checkSolvencyAtTick ( user, positionIdList, currentTick, fastOracleTick, buffer ); if (!

solventAtFast ) revert Errors.

NotEnoughCollateral (); // If one of the ticks is too stale, we fall back to the more conservative tick, i.e, the user must be solvent at both the fast and slow oracle ticks.

if ( Math.

abs ( int256 ( fastOracleTick ) - slowOracleTick ) > MAX_SLOW_FAST_DELTA ) if (!

_checkSolvencyAtTick ( user, positionIdList, currentTick, slowOracleTick, buffer )) revert Errors.

NotEnoughCollateral (); } function _checkSolvencyAtTick ( address account, TokenId [] calldata positionIdList, int24 currentTick, int24 atTick, uint256 buffer ) internal view returns ( bool ) { ( LeftRightSigned portfolioPremium, uint256 [ 2 ][] memory positionBalanceArray ) = _calculateAccumulatedPremia ( account, positionIdList, COMPUTE_ALL_PREMIA, ONLY_AVAILABLE_PREMIUM, currentTick ); LeftRightUnsigned tokenData0 = s_collateralToken0.

getAccountMarginDetails ( account, atTick, positionBalanceArray, portfolioPremium.

rightSlot () ); LeftRightUnsigned tokenData1 = s_collateralToken1.

getAccountMarginDetails ( account, atTick, positionBalanceArray, portfolioPremium.

leftSlot () ); ( uint256 balanceCross, uint256 thresholdCross ) = _getSolvencyBalances ( tokenData0, tokenData1, Math.

getSqrtRatioAtTick ( atTick ) ); // compare balance and required tokens, can use unsafe div because denominator is always nonzero unchecked { return balanceCross >= Math.

unsafeDivRoundingUp ( thresholdCross * buffer, 10_000 ); } CollateralTracker.sol function getAccountMarginDetails ( address user, int24 currentTick, uint256 [ 2 ][] memory positionBalanceArray, int128 premiumAllPositions ) public view returns ( LeftRightUnsigned tokenData ) { tokenData = _getAccountMargin ( user, currentTick, positionBalanceArray, premiumAllPositions ); } function _getAccountMargin ( address user, int24 atTick, uint256 [ 2 ][] memory positionBalanceArray, int128 premiumAllPositions ) internal view returns ( LeftRightUnsigned tokenData ) { uint256 tokenRequired; // if the account has active options, compute the required collateral to keep account in good health if (

positionBalanceArray.

length > 0 ) { // get all collateral required for the incoming list of positions tokenRequired = _getTotalRequiredCollateral ( atTick, positionBalanceArray ); // If premium is negative (ie. user has to pay for their purchased options), add this long premium to the token requirement if ( premiumAllPositions < 0 ) { unchecked { tokenRequired += uint128 (- premiumAllPositions ); } // if premium is positive (ie. user will receive funds due to selling options), add this premum to the user's balance uint256 netBalance = convertToAssets ( balanceOf [ user ]); if ( premiumAllPositions > 0 ) { unchecked { netBalance += uint256 ( uint128 ( premiumAllPositions )); } // store assetBalance and tokens required in tokenData variable

tokenData = tokenData.

toRightSlot ( netBalance.

toUint128 ()).

toLeftSlot ( tokenRequired.

toUint128 () ); return tokenData; } Now we have shown how to bypass the _validateSolvency(), we can bypass all related checks. Listing them here:

Burn options #1, #2 Mint options #1 Force exercise #1, #2 SettleLongPremium #1

## Recommended Mitigation Steps

Add a check in _validatePositionList that the length is shorter than MAX_POSITIONS (32).

## Assessed type

Invalid Validation dyedm1 (Panoptic) confirmed Picodes (judge) decreased severity to Medium and commented:

I don’t think here “liquidation bots not liquidating the insolvent position during a period of time” is an external requirement considering how critical liquidations are to Panoptic. My reasoning is that even if the loss of funds is not “direct”, being able to properly control when positions can be opened is a key feature, and the fact that you can “increase your leverage” while being insolvent prevents proper risk control.

However, it’s true that this is not strictly speaking a scenario where “assets can be stolen/lost/compromised directly” so I’ll downgrade to Med.

Note: for full discussion, please see the original submission.

# [M-03] CREATE2 address collision during pool deployment allows for complete draining of the pool

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

CREATE2 address collision during pool deployment allows for complete draining of the pool Submitted by Kalogerone

- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/PanopticFactory.sol#L237
- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/0a25c1940ca220686588c4af3ec526f725fe2582/contracts/proxy/Clones.sol#L53
(NOTE: This report is very highly inspired from this past valid report.

Necessary changes have been made to suit the Panoptic Protocol.) The attack consists of two parts: Finding a collision, and actually draining the lending pool. We describe both here:

## Impact

Address collision can cause all tokens of a Panoptic Pool to be drain.

## Recommended Mitigation Steps

Don’t allow the user to control the salt used.

Consider also adding and encoding block.timestamp and block.number combined with the user’s salt. Then the attacker, after they successfully found a hash collision, already has to execute the attack at a fixed block and probably conspire with the sequencer to ensure that also the time is fixed.

dyedm1 (Panoptic) acknowledged and commented:

Technically true, but the cost to do this is enormous (with a likely minimal return, given that deposits would first have to be solicited into that pool), and we can add safeguards on the frontend to prevent this kind of attack.

Picodes (judge) commented:

This report is worth Medium severity to me, considering:

that the attacker could target any pool playing on the salt that the attacker can wait for enough deposits before draining the pool So it fulfills “hypothetical attack path with stated assumptions, but external requirements”.

# [M-04] Incorrect validation during checking liquidity spread

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

Submitted by KupiaSec

## Impact

Because of incorrect validation, it allows option buyers not to pay premium.

## Recommended Mitigation Steps

When checking liquidity spread, it should revert when N is zero and T is positive:

+ if ( netLiquidity == 0 && totalLiquidity > 0 ) revert; if ( netLiquidity == 0 ) return;

## Assessed type

Context dyedm1 (Panoptic) confirmed

# [M-05] Panoptic pool can be non-profitable by specific Uniswap governance

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

Submitted by petro_1912, also found by Joshuajee

- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/CollateralTracker.sol#L247-L251
- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/CollateralTracker.sol#L261-L263
Swap commission is paid on the intrinsic value based on s_ITMSpreadFee in CollateralTracker contract.

If s_ITMSpreadFee is zero, then swap commission can not be paid.

## Recommended Mitigation Steps

Use Uniswap’s DECIMALS (1e6) instead 10_000 and update all code related to DECIMALS.

## Assessed type

Uniswap dyedm1 (Panoptic) confirmed Picodes (judge) commented:

This report shows how the current version of the protocol may not support all Uniswap V3 pools whereas the sponsor’s label suggests it was their intention, so Medium severity seems appropriate under “broken functionality”.

# [M-06] _updateSettlementPostBurn() may not correctly reduce s_grossPremiumLast[chunkKey]

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

_updateSettlementPostBurn() may not correctly reduce s_grossPremiumLast[chunkKey] Submitted by bin2chen s_grossPremiumLast[] definitions are as follows:

/// @dev Per-chunk last value that gives the aggregate amount of premium owed to all sellers when multiplied by the total amount of liquidity totalLiquidity /// totalGrossPremium = totalLiquidity * (grossPremium(perLiquidityX64) - lastGrossPremium(perLiquidityX64)) / 2**64 /// Used to compute the denominator for the fraction of premium available to sellers to collect /// LeftRight - right slot is token0, left slot is token1 mapping(bytes32 chunkKey => LeftRightUnsigned lastGrossPremium) internal s_grossPremiumLast; A critical rule: if there is a change in totalLiquidity, we must recalculate this value.

When Pool.mintOptions(), s_grossPremiumLast[chunkKey] will increase.

Step:

mintOptions() -> _mintInSFPMAndUpdateCollateral() -> _updateSettlementPostMint() function _updateSettlementPostMint ( TokenId tokenId, LeftRightUnsigned [ 4 ] memory collectedByLeg, uint128 positionSize ) internal {...

if ( tokenId.

isLong ( leg ) == 0 ) { LiquidityChunk liquidityChunk = PanopticMath.

getLiquidityChunk ( tokenId, leg, positionSize );...

uint256 [ 2 ] memory grossCurrent; ( grossCurrent [ 0 ], grossCurrent [ 1 ]) = SFPM.

getAccountPremium ( address ( s_univ3pool ), address ( this ), tokenId.

tokenType ( leg ), liquidityChunk.

tickLower (), liquidityChunk.

tickUpper (), type ( int24 ).

max, 0 ); unchecked { // L LeftRightUnsigned grossPremiumLast = s_grossPremiumLast [ chunkKey ]; // R uint256 positionLiquidity = liquidityChunk.

liquidity (); // T (totalLiquidity is (T + R) after minting) uint256 totalLiquidityBefore = totalLiquidity - positionLiquidity; @> s_grossPremiumLast [ chunkKey ] = LeftRightUnsigned.

wrap ( 0 ).

toRightSlot ( uint128 ( grossCurrent [ 0 ] * positionLiquidity + grossPremiumLast.

rightSlot () * totalLiquidityBefore ) / ( totalLiquidity ).

toLeftSlot ( uint128 ( grossCurrent [ 1 ] * positionLiquidity + grossPremiumLast.

leftSlot () * totalLiquidityBefore ) / ( totalLiquidity ) ); } When Pool.burnOptions() ， s_grossPremiumLast[chunkKey] will decrease burnOptions() -> _burnAndHandleExercise() -> _updateSettlementPostBurn() function _updateSettlementPostBurn ( address owner, TokenId tokenId, LeftRightUnsigned [ 4 ] memory collectedByLeg, uint128 positionSize, bool commitLongSettled ) internal returns ( LeftRightSigned realizedPremia, LeftRightSigned [ 4 ] memory premiaByLeg ) {...

uint256 numLegs = tokenId.

countLegs (); uint256 [ 2 ][ 4 ] memory premiumAccumulatorsByLeg; // compute accumulated fees ( premiaByLeg, premiumAccumulatorsByLeg ) = _getPremia ( tokenId, positionSize, owner, COMPUTE_ALL_PREMIA, type ( int24 ).

max ); for ( uint256 leg = 0; leg < numLegs; ) { LeftRightSigned legPremia = premiaByLeg [ leg ]; bytes32 chunkKey = keccak256 ( abi.

encodePacked ( tokenId.

strike ( leg ), tokenId.

width ( leg ), tokenId.

tokenType ( leg )) ); // collected from Uniswap LeftRightUnsigned settledTokens = s_settledTokens [ chunkKey ].

add ( collectedByLeg [ leg ]); @> if ( LeftRightSigned.

unwrap ( legPremia ) != 0 ) { // (will be) paid by long legs if ( tokenId.

isLong ( leg ) == 1 ) {...

} else { uint256 positionLiquidity = PanopticMath.

getLiquidityChunk ( tokenId, leg, positionSize ).

liquidity (); // new totalLiquidity (total sold) = removedLiquidity + netLiquidity (T - R) uint256 totalLiquidity = _getTotalLiquidity ( tokenId, leg ); // T (totalLiquidity is (T - R) after burning) uint256 totalLiquidityBefore = totalLiquidity + positionLiquidity; LeftRightUnsigned grossPremiumLast = s_grossPremiumLast [ chunkKey ]; LeftRightUnsigned availablePremium = _getAvailablePremium ( totalLiquidity + positionLiquidity, settledTokens, grossPremiumLast, LeftRightUnsigned.

wrap ( uint256 ( LeftRightSigned.

unwrap ( legPremia ))), premiumAccumulatorsByLeg [ leg ] ); // subtract settled tokens sent to seller settledTokens = settledTokens.

sub ( availablePremium ); // add available premium to amount that should be settled realizedPremia = realizedPremia.

add ( LeftRightSigned.

wrap ( int256 ( LeftRightUnsigned.

unwrap ( availablePremium ))) );...

unchecked { uint256 [ 2 ][ 4 ] memory _premiumAccumulatorsByLeg = premiumAccumulatorsByLeg; uint256 _leg = leg; // if there's still liquidity, compute the new grossPremiumLast // otherwise, we just reset grossPremiumLast to the current grossPremium @> s_grossPremiumLast [ chunkKey ] = totalLiquidity != 0 ?

LeftRightUnsigned.

wrap ( 0 ).

toRightSlot ( uint128 ( uint256 ( Math.

max ( int256 ( grossPremiumLast.

rightSlot () * totalLiquidityBefore ) - int256 ( _premiumAccumulatorsByLeg [ _leg ][ 0 ] * positionLiquidity )) + int256 ( legPremia.

rightSlot () * 2 ** 64 ), 0 ) ) / totalLiquidity ).

toLeftSlot ( uint128 ( uint256 ( Math.

max ( int256 ( grossPremiumLast.

leftSlot () * totalLiquidityBefore ) - int256 ( _premiumAccumulatorsByLeg [ _leg ][ 1 ] * positionLiquidity )) + int256 ( legPremia.

leftSlot ()) * 2 ** 64, 0 ) ) / totalLiquidity ):

LeftRightUnsigned.

wrap ( 0 ).

toRightSlot ( uint128 ( premiumAccumulatorsByLeg [ _leg ][ 0 ])).

toLeftSlot ( uint128 ( premiumAccumulatorsByLeg [ _leg ][ 1 ])); } // update settled tokens in storage with all local deltas s_settledTokens [ chunkKey ] = settledTokens; unchecked { ++ leg; } The issue lies within _updateSettlementPostBurn(), where it adds a condition that if (LeftRightSigned.unwrap(legPremia) != 0) must be met for s_grossPremiumLast[chunkKey] to decrease.

This results in not recalculating s_grossPremiumLast[chunkKey] even when totalLiquidity changes.

For example, in the same block, if a user executes mintOptions(), s_grossPremiumLast[chunkKey] increases by 50. Immediately after, executing burnOptions() doesn’t decrease s_grossPremiumLast[chunkKey] by 50 because legPremia == 0.

## Impact

Due to the incorrect accounting of s_grossPremiumLast[chunkKey], the calculation in _getAvailablePremium() is also incorrect. This results in inaccuracies in the amount of premium received by the seller when closing their position.

## Recommended Mitigation

Regardless of the value of legPremia, it should recalculate s_grossPremiumLast[chunkKey] when long == 0.

function _updateSettlementPostBurn( address owner, TokenId tokenId, LeftRightUnsigned[4] memory collectedByLeg, uint128 positionSize, bool commitLongSettled ) internal returns (LeftRightSigned realizedPremia, LeftRightSigned[4] memory premiaByLeg) {...

for (uint256 leg = 0; leg < numLegs; ) { LeftRightSigned legPremia = premiaByLeg[leg]; bytes32 chunkKey = keccak256( abi.encodePacked(tokenId.strike(leg), tokenId.width(leg), tokenId.tokenType(leg)) ); // collected from Uniswap LeftRightUnsigned settledTokens = s_settledTokens[chunkKey].add(collectedByLeg[leg]); if (LeftRightSigned.unwrap(legPremia) != 0) { // (will be) paid by long legs if (tokenId.isLong(leg) == 1) {...

} else {....

// subtract settled tokens sent to seller settledTokens = settledTokens.sub(availablePremium); // add available premium to amount that should be settled realizedPremia = realizedPremia.add( LeftRightSigned.wrap(int256(LeftRightUnsigned.unwrap(availablePremium))) ); - unchecked { - uint256[2][4] memory _premiumAccumulatorsByLeg = premiumAccumulatorsByLeg;- - uint256 _leg = leg; - - // if there's still liquidity, compute the new grossPremiumLast - // otherwise, we just reset grossPremiumLast to the current grossPremium - s_grossPremiumLast[chunkKey] = totalLiquidity != 0 - ? LeftRightUnsigned -.wrap(0) -.toRightSlot( - uint128( - uint256( - Math.max( - (int256( - grossPremiumLast.rightSlot() *

- totalLiquidityBefore - ) - - int256( - _premiumAccumulatorsByLeg[_leg][0] * - positionLiquidity - )) + int256(legPremia.rightSlot() * 2 ** 64), - 0 - ) - ) / totalLiquidity - ) -.toLeftSlot( - uint128( - uint256( - Math.max( - (int256( - grossPremiumLast.leftSlot() * - totalLiquidityBefore - ) - - int256( - _premiumAccumulatorsByLeg[_leg][1] * - positionLiquidity - )) + int256(legPremia.leftSlot()) * 2 ** 64, - 0 - ) - ) / totalLiquidity - ) -: LeftRightUnsigned -.wrap(0) -.toRightSlot(uint128(premiumAccumulatorsByLeg[_leg][0])) -.toLeftSlot(uint128(premiumAccumulatorsByLeg[_leg][1])); - } } + if (tokenId.isLong(leg) == 0){ + uint256 positionLiquidity = PanopticMath +.getLiquidityChunk(tokenId, leg, positionSize)

+.liquidity(); + + // new totalLiquidity (total sold) = removedLiquidity + netLiquidity (T - R) + uint256 totalLiquidity = _getTotalLiquidity(tokenId, leg); + // T (totalLiquidity is (T - R) after burning) + uint256 totalLiquidityBefore = totalLiquidity + positionLiquidity; + + LeftRightUnsigned grossPremiumLast = s_grossPremiumLast[chunkKey]; + unchecked { + uint256[2][4] memory _premiumAccumulatorsByLeg = premiumAccumulatorsByLeg; + uint256 _leg = leg; + + // if there's still liquidity, compute the new grossPremiumLast + // otherwise, we just reset grossPremiumLast to the current grossPremium + s_grossPremiumLast[chunkKey] = totalLiquidity != 0 + ? LeftRightUnsigned +.wrap(0) +.toRightSlot(

+ uint128( + uint256( + Math.max( + (int256( + grossPremiumLast.rightSlot() * + totalLiquidityBefore + ) - + int256( + _premiumAccumulatorsByLeg[_leg][0] * + positionLiquidity + )) + int256(legPremia.rightSlot() * 2 ** 64), + 0 + ) + ) / totalLiquidity + ) +.toLeftSlot( + uint128( + uint256( + Math.max( + (int256( + grossPremiumLast.leftSlot() * + totalLiquidityBefore + ) - + int256( + _premiumAccumulatorsByLeg[_leg][1] * + positionLiquidity + )) + int256(legPremia.leftSlot()) * 2 ** 64, + 0 + ) + ) / totalLiquidity + ) +: LeftRightUnsigned +.wrap(0) +.toRightSlot(uint128(premiumAccumulatorsByLeg[_leg][0])) +.toLeftSlot(uint128(premiumAccumulatorsByLeg[_leg][1])); + } } // update settled tokens in storage with all local deltas

s_settledTokens[chunkKey] = settledTokens; unchecked { ++leg; }

## Assessed type

Context dyedm1 (Panoptic) confirmed

# [M-07] When Burning a Tokenized Position validate should be done before flipping the isLong bits in _validateAndForwardToAMM()

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

validate should be done before flipping the isLong bits in _validateAndForwardToAMM() Submitted by 0xdice91

- https://github.com/code-423n4/2024-04-panoptic/blob/833312ebd600665b577fbd9c03ffa0daf250ed24/contracts/types/TokenId.sol#L535-L571
- https://github.com/code-423n4/2024-04-panoptic/blob/833312ebd600665b577fbd9c03ffa0daf250ed24/contracts/SemiFungiblePositionManager.sol#L673-L702
Each leg in a tokenid has a risk partner which is usually its own index but in some cases, it could be another leg (Partner in defined risk position).

In the function validate() if the risk partner of a specific leg is not its own index then some additional checks are done to ensure that they are compatible like:

Ensuring that risk partners are mutual Ensures that risk partners have the same asset.

Ensures that risk partners have the same ratio.

Plus other checks that depend on the legs isLong value compared to that of its risk partner.

function validate ( TokenId self ) internal pure { if ( self.

optionRatio ( 0 ) == 0 ) revert Errors.

InvalidTokenIdParameter ( 1 ); // More Code...

// In the following, we check whether the risk partner of this leg is itself // or another leg in this position.

// Handles case where riskPartner(i) != i ==> leg i has a risk partner that is another leg uint256 riskPartnerIndex = self.

riskPartner ( i ); if ( riskPartnerIndex != i ) { // Ensures that risk partners are mutual if ( self.

riskPartner ( riskPartnerIndex ) != i ) revert Errors.

InvalidTokenIdParameter ( 3 ); // Ensures that risk partners have 1) the same asset, and 2) the same ratio if ( self.

asset ( riskPartnerIndex ) != self.

asset ( i )) || ( self.

optionRatio ( riskPartnerIndex ) != self.

optionRatio ( i )) ) revert Errors.

InvalidTokenIdParameter ( 3 ); // long/short status of associated legs uint256 _isLong = self.

isLong ( i ); uint256 isLongP = self.

isLong ( riskPartnerIndex ); // token type status of associated legs (call/put) uint256 _tokenType = self.

tokenType ( i ); uint256 tokenTypeP = self.

tokenType ( riskPartnerIndex ); // if the position is the same i.e both long calls, short put's etc.

// then this is a regular position, not a defined risk position if (( _isLong == isLongP ) && ( _tokenType == tokenTypeP )) revert Errors.

InvalidTokenIdParameter ( 4 ); // if the two token long-types and the tokenTypes are both different (one is a short call, the other a long put, e.g.), this is a synthetic position // A synthetic long or short is more capital efficient than each leg separated because the long+short premia accumulate proportionally // unlike short stranlges, long strangles also cannot be partnered, because there is no reduction in risk (both legs can earn premia simultaneously) if ((( _isLong != isLongP ) || _isLong == 1 ) && ( _tokenType != tokenTypeP )) revert Errors.

InvalidTokenIdParameter ( 5 ); } // end for loop over legs } In burnTokenizedPosition() the internal function _validateAndForwardToAMM() is called, this function calls Tokenid.flipToBurnToken() which simply flips the isLong bits of all active legs of the tokenid. Then validate() is called which validates a position tokenId and its legs.

/// @param tokenId the option position /// @param positionSize the size of the position to create /// @param tickLimitLow lower limits on potential slippage /// @param tickLimitHigh upper limits on potential slippage /// @param isBurn is equal to false for mints and true for burns /// @return collectedByLeg An array of LeftRight encoded words containing the amount of token0 and token1 collected as fees for each leg /// @return totalMoved the total amount of funds swapped in Uniswap as part of building potential ITM positions function _validateAndForwardToAMM ( TokenId tokenId, uint128 positionSize, int24 tickLimitLow, int24 tickLimitHigh, bool isBurn ) internal returns ( LeftRightUnsigned

[ 4 ] memory collectedByLeg, LeftRightSigned totalMoved ) { // Reverts if positionSize is 0 and user did not own the position before minting/burning if ( positionSize == 0 ) revert Errors.

OptionsBalanceZero (); /// @dev the flipToBurnToken() function flips the isLong bits if ( isBurn ) { tokenId = tokenId.

flipToBurnToken (); } // Validate tokenId tokenId.

validate (); // Extract univ3pool from the poolId map to Uniswap Pool IUniswapV3Pool univ3pool = s_poolContext [ tokenId.

poolId ()].

pool; // Revert if the pool not been previously initialized if ( univ3pool == IUniswapV3Pool ( address ( 0 ))) revert Errors.

UniswapPoolNotInitialized (); // More Code...

} The issue here is that if a leg in the tokenid has its risk partner as another leg (that is, it is not its own risk partner), then flipping the isLong bits may cause one of the checks in validate() to fail and revert as the isLong bits of its risk partner are not changed as well.

Remember that flipping changes the value of the bit from what it was to an opposite value (from 0 to 1 or from 1 to 0).

For example; Let’s say a leg with a different risk partner has isLong() values that are the same but their tokenType() is different, this would easily pass these checks below from validate() but after a flip is done to its isLong bits using flipToBurnToken() it will fail and revert in the second check below.

// if the position is the same i.e both long calls, short put's etc.

// then this is a regular position, not a defined risk position if (( _isLong == isLongP ) && ( _tokenType == tokenTypeP )) revert Errors.

InvalidTokenIdParameter ( 4 ); // if the two token long-types and the tokenTypes are both different (one is a short call, the other a long put, e.g.), this is a synthetic position // A synthetic long or short is more capital efficient than each leg separated because the long+short premia accumulate proportionally // unlike short stranlges, long strangles also cannot be partnered, because there is no reduction in risk (both legs can earn premia simultaneously) if ((( _isLong != isLongP ) || _isLong == 1 ) && ( _tokenType != tokenTypeP )) revert Errors.

InvalidTokenIdParameter ( 5 );

## Impact

This will result in a continuous revert of the function leading to an inability to Burn a Tokenized Position.

## Recommended Mitigation Steps

This whole issue results from the simple fact the risk partners, if different, are not flipped as well. I recommend validating the tokenid before flipping the isLong bits, to ensure any changes caused by flipping will not affect the execution of the function.

/// @param tokenId the option position /// @param positionSize the size of the position to create /// @param tickLimitLow lower limits on potential slippage /// @param tickLimitHigh upper limits on potential slippage /// @param isBurn is equal to false for mints and true for burns /// @return collectedByLeg An array of LeftRight encoded words containing the amount of token0 and token1 collected as fees for each leg /// @return totalMoved the total amount of funds swapped in Uniswap as part of building potential ITM positions function _validateAndForwardToAMM ( TokenId tokenId, uint128 positionSize, int24 tickLimitLow, int24 tickLimitHigh, bool isBurn ) internal returns ( LeftRightUnsigned

[ 4 ] memory collectedByLeg, LeftRightSigned totalMoved ) { // Reverts if positionSize is 0 and user did not own the position before minting/burning if ( positionSize == 0 ) revert Errors.

OptionsBalanceZero (); ++ // Validate tokenId ++ tokenId.

validate (); /// @dev the flipToBurnToken() function flips the isLong bits if ( isBurn ) { tokenId = tokenId.

flipToBurnToken (); } // Extract univ3pool from the poolId map to Uniswap Pool IUniswapV3Pool univ3pool = s_poolContext [ tokenId.

poolId ()].

pool; // Revert if the pool not been previously initialized if ( univ3pool == IUniswapV3Pool ( address ( 0 ))) revert Errors.

UniswapPoolNotInitialized (); // More Code...

} dyedm1 (Panoptic) confirmed Picodes (judge) commented:

Keeping Medium severity under “functionality is broken”.

# [M-08] Wrong leg chunkKey calculation in haircutPremia function

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

chunkKey calculation in haircutPremia function Submitted by Aymen0909, also found by FastChecker, 0xStalin, and DanielArmstrong

- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/libraries/PanopticMath.sol#L878-L884
- https://github.com/code-423n4/2024-04-panoptic/blob/main/contracts/PanopticPool.sol#L1122-L1131
When the user positions are getting liquidated, the PanopticPool::liquidate function will invoke under the hood the PanopticMath::haircutPremia function which will use the user’s long premium to cover the protocol losses.

The PanopticMath::haircutPremia function will also update the storage mapping settledTokens which represent the per-chunk accumulator for tokens owed to options sellers.

The issue that it’s present in the PanopticMath::haircutPremia function is when it runs the for-loop to update settledTokens for each position leg chunk, inside the loop the function uses always the index 0 when calculating the leg chunkKey instead of using the actual leg index leg (which can be 0,1,2,3), this shown in the code snippet below:

function haircutPremia ( address liquidatee, TokenId [] memory positionIdList, LeftRightSigned [ 4 ][] memory premiasByLeg, LeftRightSigned collateralRemaining, CollateralTracker collateral0, CollateralTracker collateral1, uint160 sqrtPriceX96Final, mapping ( bytes32 chunkKey => LeftRightUnsigned settledTokens ) storage settledTokens ) external returns ( int256, int256 ) {...

for ( uint256 i = 0; i < positionIdList.

length; i ++) { TokenId tokenId = positionIdList [ i ]; LeftRightSigned [ 4 ][] memory _premiasByLeg = premiasByLeg; for ( uint256 leg = 0; leg < tokenId.

countLegs (); ++ leg ) { if ( tokenId.

isLong ( leg ) == 1 ) { mapping ( bytes32 chunkKey => LeftRightUnsigned settledTokens ) storage _settledTokens = settledTokens; // calculate amounts to revoke from settled and subtract from haircut req uint256 settled0 = Math.

unsafeDivRoundingUp ( uint128 (- _premiasByLeg [ i ][ leg ].

rightSlot ()) * uint256 ( haircut0 ), uint128 ( longPremium.

rightSlot ()) ); uint256 settled1 = Math.

unsafeDivRoundingUp ( uint128 (- _premiasByLeg [ i ][ leg ].

leftSlot ()) * uint256 ( haircut1 ), uint128 ( longPremium.

leftSlot ()) ); //@audit always calculating the chunkKey of leg 0 bytes32 chunkKey = keccak256 ( abi.

encodePacked ( tokenId.

strike ( 0 ), tokenId.

width ( 0 ), tokenId.

tokenType ( 0 ) ); // The long premium is not commited to storage during the liquidation, so we add the entire adjusted amount // for the haircut directly to the accumulator settled0 = Math.

max ( 0, uint128 (- _premiasByLeg [ i ][ leg ].

rightSlot ()) - settled0 ); settled1 = Math.

max ( 0, uint128 (- _premiasByLeg [ i ][ leg ].

leftSlot ()) - settled1 ); _settledTokens [ chunkKey ] = _settledTokens [ chunkKey ].

add ( LeftRightUnsigned.

wrap ( 0 ).

toRightSlot ( uint128 ( settled0 )).

toLeftSlot ( uint128 ( settled1 ) ); } return ( collateralDelta0, collateralDelta1 ); } This issue means that the settledTokens accumulator will be updated incorrectly and will not include all the legs premium, as for each position only the first leg (index=0) is considered, this will result in funds losses for the options sellers and for the protocol.

## Impact

Wrong leg chunkKey calculation in haircutPremia will cause incorrect update of settledTokens accumulator and will result in funds losses for the options sellers and for the protocol.

Tools Used VS Code

## Recommended Mitigation

Use the correct leg index when calculating the leg chunkKey in haircutPremia function, the correct code should be:

function haircutPremia( address liquidatee, TokenId[] memory positionIdList, LeftRightSigned[4][] memory premiasByLeg, LeftRightSigned collateralRemaining, CollateralTracker collateral0, CollateralTracker collateral1, uint160 sqrtPriceX96Final, mapping(bytes32 chunkKey => LeftRightUnsigned settledTokens) storage settledTokens ) external returns (int256, int256) {...

for (uint256 i = 0; i < positionIdList.length; i++) { TokenId tokenId = positionIdList[i]; LeftRightSigned[4][] memory _premiasByLeg = premiasByLeg; for (uint256 leg = 0; leg < tokenId.countLegs(); ++leg) { if (tokenId.isLong(leg) == 1) { mapping(bytes32 chunkKey => LeftRightUnsigned settledTokens) storage _settledTokens = settledTokens; // calculate amounts to revoke from settled and subtract from haircut req uint256 settled0 = Math.unsafeDivRoundingUp( uint128(-_premiasByLeg[i][leg].rightSlot()) * uint256(haircut0), uint128(longPremium.rightSlot()) ); uint256 settled1 = Math.unsafeDivRoundingUp( uint128(-_premiasByLeg[i][leg].leftSlot()) * uint256(haircut1), uint128(longPremium.leftSlot()) );

bytes32 chunkKey = keccak256( abi.encodePacked( -- tokenId.strike(0), -- tokenId.width(0), -- tokenId.tokenType(0) ++ tokenId.strike(leg), ++ tokenId.width(leg), ++ tokenId.tokenType(leg) ) ); // The long premium is not commited to storage during the liquidation, so we add the entire adjusted amount // for the haircut directly to the accumulator settled0 = Math.max( 0, uint128(-_premiasByLeg[i][leg].rightSlot()) - settled0 ); settled1 = Math.max( 0, uint128(-_premiasByLeg[i][leg].leftSlot()) - settled1 ); _settledTokens[chunkKey] = _settledTokens[chunkKey].add( LeftRightUnsigned.wrap(0).toRightSlot(uint128(settled0)).toLeftSlot( uint128(settled1) ) ); } return (collateralDelta0, collateralDelta1);

}

## Assessed type

Context dyedm1 (Panoptic) confirmed and commented:

I will confirm this because we are fixing it, but I don’t think high severity is justified here. The only impact is that during liquidations, premium paid by the liquidatee for legs other than 0 is effectively haircut/refunded to the liquidatee, which could result in an uneven distribution of the haircut and some premium that could have been safely paid being refunded to the liquidatee. Besides resulting in unfair distribution at liquidation time for sellers in the 2-3-4 chunks, no actor in the protocol actually loses funds (yield for sellers is not realized until it is settled anyway, so if they closed before the liquidation occurred they would get the exact same payment).

Picodes (judge) decreased severity to Medium and commented:

Giving Medium severity as this only concerns unrealized yield and happens during liquidations only.

# [M-09] Removed liquidity can overflow when calling SemiFungiblePositionManager.mintTokenizedPosition function

- **Contest:** Panoptic
- **Slug:** 2024-04-panoptic
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-panoptic
- **Source snapshot:** competitions/2024-04-panoptic/final_report.html

SemiFungiblePositionManager.mintTokenizedPosition function Submitted by rbserver The following mintTokenizedPosition function eventually calls the _createLegInAMM function below.

- https://github.com/code-423n4/2024-04-panoptic/blob/855f58414589e93fa62e34890118164e8ac7822f/contracts/SemiFungiblePositionManager.sol#L504-L527
function mintTokenizedPosition ( TokenId tokenId, uint128 positionSize, int24 slippageTickLimitLow, int24 slippageTickLimitHigh ) external ReentrancyLock (tokenId.poolId()) returns ( LeftRightUnsigned [ 4 ] memory collectedByLeg, LeftRightSigned totalSwapped ) { // create the option position via its ID in this erc1155 _mint ( msg.

sender, TokenId.

unwrap ( tokenId ), positionSize );...

// validate the incoming option position, then forward to the AMM for minting/burning required liquidity chunks ( collectedByLeg, totalSwapped ) = _validateAndForwardToAMM ( tokenId, positionSize, slippageTickLimitLow, slippageTickLimitHigh, MINT ); } The _createLegInAMM function increases the removed liquidity when minting a long position by executing unchecked { removedLiquidity += chunkLiquidity } in which the related code comment states that we can't remove more liquidity than we add in the first place, so this can't overflow. However, minting contracts of the short and long positions repeatedly can actually increase the removed liquidity to overflow uint128, which means that the unchecked

block is unsafe. When such overflow occurs, the accounting for the removed liquidity and liquidity becomes incorrect; in this case, the overflowed removed liquidity becomes less than it should be, and burning such overflowed removed liquidity would increase the liquidity by an amount that is less than it should be.

- https://github.com/code-423n4/2024-04-panoptic/blob/855f58414589e93fa62e34890118164e8ac7822f/contracts/SemiFungiblePositionManager.sol#L958-L1104
function _createLegInAMM ( IUniswapV3Pool univ3pool, TokenId tokenId, uint256 leg, LiquidityChunk liquidityChunk, bool isBurn ) internal returns ( LeftRightSigned moved, LeftRightSigned itmAmounts, LeftRightUnsigned collectedSingleLeg ) {...

uint128 updatedLiquidity; uint256 isLong = tokenId.

isLong ( leg ); LeftRightUnsigned currentLiquidity = s_accountLiquidity [ positionKey ]; //cache {...

uint128 startingLiquidity = currentLiquidity.

rightSlot (); uint128 removedLiquidity = currentLiquidity.

leftSlot (); uint128 chunkLiquidity = liquidityChunk.

liquidity (); if ( isLong == 0 ) { // selling/short: so move from msg.sender *to* uniswap // we're minting more liquidity in uniswap: so add the incoming liquidity chunk to the existing liquidity chunk updatedLiquidity = startingLiquidity + chunkLiquidity; /// @dev If the isLong flag is 0=short but the position was burnt, then this is closing a long position /// @dev so the amount of removed liquidity should decrease.

if ( isBurn ) { removedLiquidity -= chunkLiquidity; } else { // the _leg is long (buying: moving *from* uniswap to msg.sender) // so we seek to move the incoming liquidity chunk *out* of uniswap - but was there sufficient liquidity sitting in uniswap // in the first place?

if ( startingLiquidity < chunkLiquidity ) { // the amount we want to move (liquidityChunk.legLiquidity()) out of uniswap is greater than // what the account that owns the liquidity in uniswap has (startingLiquidity) // we must ensure that an account can only move its own liquidity out of uniswap // so we revert in this case revert Errors.

NotEnoughLiquidity (); } else { // startingLiquidity is >= chunkLiquidity, so no possible underflow unchecked { // we want to move less than what already sits in uniswap, no problem:

updatedLiquidity = startingLiquidity - chunkLiquidity; } /// @dev If the isLong flag is 1=long and the position is minted, then this is opening a long position /// @dev so the amount of removed liquidity should increase.

if (!

isBurn ) { // we can't remove more liquidity than we add in the first place, so this can't overflow unchecked { removedLiquidity += chunkLiquidity; } // update the starting liquidity for this position for next time around s_accountLiquidity [ positionKey ] = LeftRightUnsigned.

wrap ( 0 ).

toLeftSlot ( removedLiquidity ).

toRightSlot ( updatedLiquidity ); }...

}

## Recommended Mitigation Steps

SemiFungiblePositionManager.sol#L1029-L1034 can be updated to the following code:

if (!

isBurn ) { removedLiquidity += chunkLiquidity; }

## Assessed type

Under/Overflow dyedm1 (Panoptic) confirmed
