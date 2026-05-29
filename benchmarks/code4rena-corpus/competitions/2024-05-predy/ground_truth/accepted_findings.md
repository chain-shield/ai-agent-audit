# Accepted H/M Findings: Predy

# [H-01] Reallocation depends on the slot0 price, which can be manipulated

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

slot0 price, which can be manipulated Submitted by ayden, also found by ZanyBonzy, emmac002, Tigerfrake, BlockSafe, 0xhere2learn, Yunaci, air_0x, and Takarez Anyone can invoke the reallocate function to reallocate the LP position to be within the desired range. Whether reallocation is necessary depends on the slot0 price. However, the slot0 price can be manipulated, potentially leading to the LP position being out of the current range and resulting in a loss of yield for the protocol.

## Recommended Mitigation Steps

It’s recommend to use TWAP price instead of slot0 price to get the current price.

## Assessed type

Invalid Validation 0xsomeone (judge) increased severity to High and commented:

The submission and its duplicates detail how the re-allocation of the pool’s positions will occur via a permissionless mechanism that will rely on the spot square root price evaluation of the Uniswap V3 pair.

This interaction is insecure, as a user can manipulate the square root price of a pair momentarily to an unfavorable price point, force a re-allocation to occur, and then capitalize on the re-allocated liquidity in an extreme tick range that would effectively result in a loss for the pool.

I believe this vulnerability merits a high-risk rating as it can be trivially exploited and would result in financial loss for a pair’s liquidity providers. To note, a re-allocation will also incur fees associated with the swaps performed by the Perp mechanism thereby permitting multiple re-allocations to continuously drain the funds involved in a particular pairId position exacerbating the effects of the issue described.

Tripathi (warden) commented:

Root cause of the above and issue #157 is the same, which is manipulation of the slot0 price. Even though both issues explain different impacts, they originate from the same root cause, so they should be grouped together.

For example, if an oracle gives an incorrect price, there could be multiple impacts like liquidation failure, withdrawal pause, or deposit pause; but I don’t think we can create different sets for each impact. Here, the root cause is the same, and both issue can be mitigated by preventing manipulation of the slot0 price 0xsomeone (judge) commented:

@Tripathi - The root cause is not manipulation of the slot0 data point, but rather how the slot0 data point is consumed in a different way across two distinct contracts. Given that different code segments give birth to the vulnerability, different impact is observed on each one, and that mitigation of one would not resolve the other (as we cannot change the Uniswap code), the submissions will remain distinct.

Tripathi (warden) commented:

We agree that the impact is different, but the affected code is the same and is implemented multiple times.

It is similar to using transfer instead of safetransfer. In some places, it will lead to high impact and in others, it will lead to less impact. Again, we will have to change every transfer function to safetransfer (As mitigation of one would not resolve the other), but it used to be considered the same group of issues.

The root cause is not manipulation of the slot0 data point, but rather how the slot0 data point is consumed in a different way across two distinct contracts The root cause is indeed using the spot price from the DEX, which can be manipulated by external factors.

0xsomeone (judge) commented:

It is important to understand that not all vulnerabilities are equal and just because some code is the same across the codebase does not necessarily mean it is a problem. As an example, a re-entrancy vulnerability stemming from the same external call at different instances of the codebase would be treated distinctly as the re-entrancy itself is not the problem; how the data points in the contract are processed is.

In this instance, the slot0 has multiple data points and the vulnerabilities require a different approach to being resolved. Simply implementing a TWAP for the re-allocation mechanism is incorrect as that would make the re-allocation system permanently inefficient. Additionally, the re-allocation slot0 vulnerability also utilizes the current tick which is incorrect as well.

Fixing the currentSqrtPrice to use a TWAP would be insufficient as the current tick would be used as well. To properly address this vulnerability, a potential re-design might be warranted which would require a privileged role to perform re-allocations, re-allocations to be authorized via a signature, and other such secondary security measures that would prevent both the malleable tick and square root price from being used.

The less severe issue #157 will solely utilize the square root price data point, and using a TWAP in that case is safe and a sound approach.

While I appreciate your diligent review of this submission and its sibling #157, I will maintain them as separate due to a reasonable alleviation for one exhibit being inapplicable for the other. To sum up:

Different code segments and contracts are involved One exhibit uses 1 data point while the other uses 2 data points A TWAP solution would be applicable to one and a different solution would need to be applied to the other syuhei176 (Predy) confirmed Note: For full discussion, see here.

# [H-02] Liquidators can bypass remaining negative margin check and leave the loss to the protocol

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

Submitted by nnez, also found by Eeyore, pkqs90, and zhaojohnson Liquidators can profit from protocol insolvency Description At the end of liquidation process, in the case of no remaining position, liquidate function checks for remaining margin of the liquidating vault. If there is positive remaining amount, it proceeds to send those margin back to vault’s recipient.

On contrary, if there is negative remaining amount, liquidator must pay (compensate) for those negative amounts.

This mechanism exists to protect against protocol insolvency in the case that vault’s margin cannot cover the loss from vault’s position.

Liquidation incentives Liquidators close vault’s position, settle the trade and profit from slippage tolerance, set by the pool’s owner. For instance, if the slippage tolerance is set at 5% and let’s say liquidator is closing this long position:

amountBase: 1 ETH amountQuote: -3,000 USDC entryValue: 3,000 USDC currentPrice: 2,500 USDC Liquidator will get 1 ETH to sell for 2,500 USDC and have to return only 2500 * 0.95 = 2,375 USDC. The remaining short USDC will be deducted from vault’s margin, in this case, 3000 - 2375 = 625 USDC.

Considering this mechanism, liquidators will always profit from the slippage.

Vulnerability If we take a look at the snippet of the code responsible for the aforementioned protection mechanism.

LiquidationLogic.sol#L89-L108:

if (!hasPosition) { int256 remainingMargin = vault.margin; if (remainingMargin > 0) { if (vault.recipient != address(0)) { // Send the remaining margin to the recipient.

vault.margin = 0; sentMarginAmount = uint256(remainingMargin); ERC20(pairStatus.quotePool.token).safeTransfer(vault.recipient, sentMarginAmount); } } else if (remainingMargin < 0) { vault.margin = 0; // To prevent the liquidator from unfairly profiting through arbitrage trades in the AMM and passing losses onto the protocol, // any losses that cannot be covered by the vault must be compensated by the liquidator ERC20(pairStatus.quotePool.token).safeTransferFrom(msg.sender, address(this), uint256(-remainingMargin)); } The liquidation function only checks for negative margin if and only if the position is fully closed from liquidation. Therefore, if the position is not fully closed (99.99% closed) and left with negative margin, liquidators don’t have to compensate for the loss.

Considering the case where liquidation of a full position would result in negative margin, liquidators should only be incentivized to partially close a position to the point that vault’s margin becomes zero because that would be their maximum profit.

However, with the logic flaw mentioned, liquidators can increase their profit by closing nearly full position (99.99% closed), effectively leaving the loss to the protocol.

## Recommended Mitigation

If the remaining margin in the vault is equal or less than zero, then there is no need to check whether there is still an open position; because it is already effectively closed (no margin left). Therefore, a check for negative margin should be moved out from if(!hasPosition) block.

Suggested fix if (!hasPosition) { int256 remainingMargin = vault.margin; if (remainingMargin > 0) { if (vault.recipient != address(0)) { // Send the remaining margin to the recipient.

vault.margin = 0; sentMarginAmount = uint256(remainingMargin); ERC20(pairStatus.quotePool.token).safeTransfer(vault.recipient, sentMarginAmount); } else{ if (remainingMargin < 0) { vault.margin = 0; // To prevent the liquidator from unfairly profiting through arbitrage trades in the AMM and passing losses onto the protocol, // any losses that cannot be covered by the vault must be compensated by the liquidator ERC20(pairStatus.quotePool.token).safeTransferFrom(msg.sender, address(this), uint256(-remainingMargin)); } Rationale for Medium severity Value leaks from protocol (insolvency) but it only happens in certain situations, i.e., price drops sharply.

syuhei176 (Predy) confirmed via duplicate Issue #9 0xsomeone (judge) increased severity to High and commented:

The Warden has demonstrated how the stop-loss mechanism of the protocol is improperly applied only when a vault is closed in its entirety, permitting liquidators to close a vault whilst leaving a minuscule remainder within it to bypass the mechanism and thus profit at the expense of the protocol.

I believe a high severity rating is appropriate given that the vulnerability can be reliably exploited and will further deteriorate a negative market event with tangible monetary impact.

# [H-03] One pair can steal another pair’s Uniswap liquidity during reallocate() call if both pairs operate on the same Uniswap pool and both have the same upper and lower tick during reallocation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

reallocate() call if both pairs operate on the same Uniswap pool and both have the same upper and lower tick during reallocation Submitted by Eeyore, also found by nnez During the reallocation of a pair, the Predy protocol does not verify that the liquidity in the Uniswap pool between the upper and lower tick belongs exclusively to that pair. Instead, it takes the entire liquidity from the range that the pair currently operates within.

In a scenario where a trusted operator creates two pairs for the same Uniswap pool, perhaps to have different quote tokens as margin tokens, there is a possibility that both pairs will have the same upper and lower tick setup.

In such a situation, if a user trades gamma on the first pair, and later the price moves outside the threshold of the second pair, if anyone performs a reallocation on that second pair, even if there were no gamma trades on the second pair, the second pair will steal liquidity from the user’s open gamma position.

This will cause all accounting within the protocol for these two pairs to be compromised.

## Impact

Internal protocol accounting will be disrupted, potentially making it impossible to close or liquidate positions properly.

## Recommended Mitigation Steps

Perform internal accounting of the liquidity mined within each pair and allow reallocation of only that amount of liquidity during the reallocate() function call.

Collect fees from the range proportionally to the liquidity held by each pair.

## Assessed type

Uniswap syuhei176 (Predy) confirmed and commented:

A great find.

0xsomeone (judge) commented:

The Warden and its duplicate have demonstrated that the accounting system of the Predy pool will be compromised in case overlapping Uniswap V3 tick ranges are utilized in distinct pair instances with the same tokens.

The severity of the submission has been aptly demonstrated by the PoC provided, and a trusted party is involved within it. Per the audit’s description, the trusted operator role should only be able to add new pairs to the system, and being able to maliciously take advantage of this permission to the effect described by the PoC is sufficient in rendering this submission a valid HM vulnerability.

In reality, the operator will add pairs requested by normal users (per the duplicate submission’s contents) and a pair configuration with the same range and token pair but a different quote token would appear innocuous and could easily be presumed as a valid action.

# [H-04] Liquidation incorrectly tries to transfer token from Market instead of liquidator if remainingMargin is negative

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

remainingMargin is negative Submitted by pkqs90 Liquidation cannot be executed if there is bad debt (vault position is negative) after liquidation.

Bug Description See the liquidation flowchart here.

The liquidator would call a Market contract, which triggers the liquidation logic in PredyPool. The tokens would then be swapped accordingly using the SettlementCallbackLib.

The issue is during the end of liquidation, if the position is entirely wiped out (e.g.

hasPosition == false in the following code), the code checks whether there is still margin left in the vault.

If there is still margin left, the remaining margin would be sent to the vault.recipient address.

If there is bad debt (negative margin value), the liquidator must pay for this.

For case 2, the code tries to transfer token from msg.sender to PredyPool. However, the msg.sender is the Market protocol, and not the liquidator. This means liquidating a vault with negative margin is impossible, and this bad debt will never be cleared. What’s worse, the lending fees would still accumulate for this vault, and the bad debt keeps getting larger.

function liquidate ( uint256 vaultId, uint256 closeRatio, GlobalDataLibrary.GlobalData storage globalData, bytes memory settlementData ) external returns (IPredyPool.TradeResult memory tradeResult ) {...

if (!

hasPosition ) { int256 remainingMargin = vault.

margin; if ( remainingMargin > 0 ) { if ( vault.

recipient != address ( 0 )) { // Send the remaining margin to the recipient.

vault.

margin = 0; sentMarginAmount = uint256 ( remainingMargin ); ERC20 ( pairStatus.

quotePool.

token ).

safeTransfer ( vault.

recipient, sentMarginAmount ); } else if ( remainingMargin < 0 ) { vault.

margin = 0; > // To prevent the liquidator from unfairly profiting through arbitrage trades in the AMM and passing losses onto the protocol, > // any losses that cannot be covered by the vault must be compensated by the liquidator > ERC20 ( pairStatus.

quotePool.

token ).

safeTransferFrom ( msg.

sender, address ( this ), uint256 (- remainingMargin )); }...

}

## Recommended Mitigation Steps

Transfer quoteTokens from the liquidator instead of the Market protocol.

## Assessed type

Token-Transfer 0xsomeone (judge) commented:

The code outlined represents the intended design as the caller’s funds (i.e., liquidators) must be used, and the markets are responsible for acquiring the relevant funds from their respective callers.

pkqs90 (warden) commented:

@0xsomeone - Let me first describe my understanding of how liquidation works:

Liquidator calls PerpMarket (the execLiquidationCall() function in BaseMarket) to start a liquidation PerpMarket calls PredyPool’s execLiquidationCall() PredyPool callsback PerpMarket SettlementCallbackLib to handle token swap. Take SettlementCallbackLib sell() as an example. The sender here is the user’s address, instead of the PerpMarket’s. Tokens are swapped between user and PredyPool:

if ( settlementParams.

contractAddress == address ( 0 )) { // direct fill uint256 quoteAmount = sellAmount * price / Constants.

Q96; predyPool.

take ( false, sender, sellAmount ); ERC20 ( quoteToken ).

safeTransferFrom ( sender, address ( predyPool ), quoteAmount ); return; } After swap, PredyPool checks the swap price is in a valid range by SlippageLib.checkPrice().

If the liquidation leads to negative margin, someone should pay the bad debt.

else if ( remainingMargin < 0 ) { vault.

margin = 0; > // To prevent the liquidator from unfairly profiting through arbitrage trades in the AMM and passing losses onto the protocol, > // any losses that cannot be covered by the vault must be compensated by the liquidator > ERC20 ( pairStatus.

quotePool.

token ).

safeTransferFrom ( msg.

sender, address ( this ), uint256 (- remainingMargin )); } In step 5, it transfers tokens from PerpMarket instead of from the user. However, PerpMarket doesn’t have token allowance to PredyPool, so the transfer would always fail. Secondly, I did not find documentation saying markets are responsible acquiring the relevant funds from their respective callers.

So maybe I’m missing something here. I also did not find any unit tests related to such scenario.

syuhei176 (Predy) confirmed and commented:

I missed this report. As @pkqs90 mentioned, regarding non-performing loans, they are compensated from the market contract within the liquidate function’s safeTransferFrom. The processing on the market contract side has not yet been implemented. This should have been pointed out beginning the audit.

This issue is the liquidationCall version of Issue #26. Therefore, this report is valid.

0xsomeone (judge) commented:

I agree with the facts shared here, and this exhibit will be re-opened as a valid high-risk submission. To note, I had mentioned that rulings might change after PJQA and Sponsor input for all exhibits that were marked as unsatisfactory during the validation round and would be revisited after appropriate feedback has been gathered.

Eeyore (warden) commented:

@0xsomeone - I agree there is an issue in the BaseMarket contract and all the contracts that inherit from it, but it does not affect the ability to liquidate any position.

Every position can be liquidated directly by calling PreayPool.execLiquidationCall() with the same parameters as in Base.execLiquidationCall() there is no blocker for liquidators to build its own solution to support IHooks.predySettlementCallback(), and the position will be liquidated even when the vault position is negative.

Therefore, I disagree with categorizing this issue as High.

0xsomeone (judge) commented:

I will maintain the high severity rating for this submission due to simply pointing out an egregious error in the code which, per relevant SC rulings, falls under the definition of a “high-risk” vulnerability.

While direct interaction may be possible, that never was the intended use of the markets and their being effectively inoperable in the regard described by the exhibit is a significant flaw.

Note: For full discussion, see here.

Medium Risk Findings (8)

# [M-01] Liquidity manipulation is possible when trading

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

Submitted by Yunaci, also found by Rhaydden, crypticdefense, Naresh, kodyvim, emmac002, erictee, WinSec, ZanyBonzy, Sparrow, julian_avantgarde, lightoasis, Bauchibred, and Sathish9098 The Trade contract lacks any checks against the retrieved price. It doesn’t utilize TWAP. Consequently, an attacker can manipulate the spot price ( slot0 ) through a flash loan attack. This manipulation can lead to highly inaccurate trade results and potential loss of funds for the users.

## Recommended Mitigation Steps

Consider using the TWAP price instead of the spot price in order to prevent price manipulation.

syuhei176 (Predy) acknowledged and commented via duplicate Issue #74:

To ensure that the price of the target Uniswap pool has not been manipulated during Squart transaction, we need to check slot0. Therefore, a getSqrtPrice function is necessary.

0xsomeone (judge) decreased severity to Medium 0xsomeone (judge) commented via duplicate Issue #74:

The submission and its duplicates detail how it is dangerous to rely on the slot0 to provide proper data for a particular trade.

There is a particular code segment of the system that will utilize the slot0 square root price at face value, and that is the execution of a Trade::swap with a net-zero totalBaseAmount.

Liquidations and Gamma Market trade executions will properly apply slippage checks, but direct interaction with the PredyPool will not. Given that the allowlist can be disabled for a particular pairId, it infers that users would eventually be able to directly interact with the PredyPool and this would be unsafe to do so in case of a net-neutral trade interaction.

I believe a severity of medium is appropriate as trades will be affected, and at the very least the interaction by the user will utilize improper asset ratios when executed. The interactions within the Perp contract involved are complex, and I welcome the Sponsor to refute my understanding of the codebase at this point.

# [M-02] updateIRMParams does not call applyInterestForToken before updating irmParams which leads to incorrect calculation of interest rate for subsequent trades.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

updateIRMParams does not call applyInterestForToken before updating irmParams which leads to incorrect calculation of interest rate for subsequent trades.

Submitted by WinSec, also found by Tigerfrake, 0xhere2learn, TECHFUND-inc ( 1, 2 ), SpicyMeatball, and zhaojohnson

- https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/libraries/logic/AddPairLogic.sol#L128
- https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/libraries/logic/AddPairLogic.sol#L96

## Impact

Incorrect interest rate value will be calculated for future trades as the update to irmParams does not update the lastUpdatedAt or calculate the interest rate before updating the value of irmParams. This also results in the incorrect calculation of totalProtocolFees

## Recommended Mitigation Steps

While updating the params, the applyInterestForToken function should be called first for the current period to calculate the interestRate from block.timestamp to lastUpdateTimestamp, and then the updated value of irmParams can be used for durations after this block.timestamp.

The same should be done for the updateFeeRatio function.

syuhei176 (Predy) confirmed via duplicate Issue #12 0xsomeone (judge) commented:

The submission and its duplicates have demonstrated how an update of the protocol’s IRM parameters is not preceded by an interest update, permitting the updated variables to retroactively apply to the time elapsed since the last interest rate update incorrectly.

I believe a medium-risk severity rating is appropriate given that all IRM reconfigurations will lead to interest rates being improperly tracked in the system.

# [M-03] Incorrect price for negative ticks due to lack of rounding down

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

Submitted by josephdara, also found by Rhaydden ( 1, 2 ), Tigerfrake, Naresh, Bauchibred, ayden, ZanyBonzy, SBSecurity, kodyvim, WinSec, Kaysoft, Giorgio, jolah1, 0xhashiman, 0xabhay, Sparrow, 0xhere2learn, and grearlake The function callUniswapObserve is used to get twap price tick using IUniswapV3PoolOracle.observe.selector which is then used to calculate the int24 tick.

The problem is that in case if (tickCumulatives[1] - tickCumulatives[0]) is negative, the tick should be rounded down as it’s done in the OracleLibrary from uniswap.

As result, in case if (tickCumulatives[1] - tickCumulatives[0]) is negative and (tickCumulatives[1] - tickCumulatives[0]) % secondsAgo != 0, then returned tick will be bigger then it should be, hence incorrect prices would be used.

## Recommended Mitigation Steps

Round down the int24 tick:

if ( tickCumulativesDelta < 0 && ( tickCumulativesDelta % secondsAgo != 0 )) tick --;

## Assessed type

Math syuhei176 (Predy) confirmed via duplicate Issue #65 0xsomeone (judge) commented:

The Warden and its peers have demonstrated that the DEX price feed calculation does not round properly, resulting in a deviation of as much as one tick which, depending on the spacing of the pool, can be significant.

As such, I believe a medium risk rating is appropriate for this submission.

# [M-04] Chainlink’s latestRoundData might return stale or incorrect results

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

latestRoundData might return stale or incorrect results Submitted by 0xb0k0, also found by Tigerfrake, Bigsam, MSaptarshi, JC, Kaysoft, unix515, Pelz, 0xHash, Norah ( 1, 2 ), Bauchibred, biakia, nnez, mt030d, ayden, Neo_Granicen, 0xAkira, golu, atoko ( 1, 2 ), emmac002, WinSec, y0ng0p3, web3km, steadyman, 0xMilenov, Naresh, josephdara, Eeyore, lydia_m_t, Abhan, 0xabhay, Tripathi, dyoff, pkqs90, shaflow2, SpicyMeatball, Sathish9098, forgebyola, Sparrow, ZanyBonzy, and 0xlucky In the PriceFeed contract, the protocol uses a ChainLink aggregator to fetch the latestRoundData(), but there is no check if the return value indicates stale data. The only check present is for the

quoteAnswer to be > 0; however, this alone is not sufficient.

function getSqrtPrice () external view returns ( uint256 sqrtPrice ) { @> (, int256 quoteAnswer,,,) = AggregatorV3Interface ( _quotePriceFeed ).

latestRoundData (); // missing additional checks IPyth.

Price memory basePrice = IPyth ( _pyth ).

getPriceNoOlderThan ( _priceId, VALID_TIME_PERIOD ); require ( basePrice.

expo == - 8, "INVALID_EXP" ); require ( quoteAnswer > 0 && basePrice.

price > 0 ); uint256 price = uint256 ( int256 ( basePrice.

price )) * Constants.

Q96 / uint256 ( quoteAnswer ); price = price * Constants.

Q96 / _decimalsDiff; sqrtPrice = FixedPointMathLib.

sqrt ( price ); } The protocol mentions that:

Attacks that stem from the TWAP being extremely stale compared to the market price within its period (currently 30 minutes) are a known risk. As a general rule, only price manipulation issues that can be triggered by manipulating the price atomically from a normal pool or oracle state are valid.

However, this stale period check is only currently applied to the Pyth integration, where the ChainLink feed is not considered for stale data.

This could lead to stale prices according to the Chainlink documentation here.

This discrepancy could have the protocol produce incorrect values for very important functions in different places across the system, such as GammaTradeMarket, PositionCalculator, LiquidationLogic, etc.

## Recommended Mitigation Steps

Consider adding missing checks for stale data:

@@ -43,7 +43,10 @@ contract PriceFeed { /// @notice This function returns the square root of the baseToken price quoted in quoteToken.

function getSqrtPrice() external view returns (uint256 sqrtPrice) { - (, int256 quoteAnswer,,,) = AggregatorV3Interface(_quotePriceFeed).latestRoundData(); + (uint80 quoteRoundID, int256 quoteAnswer,, uint256 quoteTimestamp, uint80 quoteAnsweredInRound) = + AggregatorV3Interface(_quotePriceFeed).latestRoundData(); + require(quoteAnsweredInRound >= quoteRoundID, "Stale price!"); + require(quoteTimestamp != 0, "Round not complete!"); + require(block.timestamp - quoteTimestamp <= VALID_TIME_PERIOD);

## Assessed type

Oracle syuhei176 (Predy) confirmed 0xsomeone (judge) commented:

The Warden has demonstrated how the Chainlink oracle employed by the system does not impose any staleness check, permitting misbehavior in the Chainlink system to not be detected by the system and the system to continue utilizing a stale price, similar to the Luna flash crash.

I believe a medium-risk rating is appropriate given the low likelihood of such an event but the devastating consequences it could result in.

A subset of the duplicates have been penalized for not properly justifying why the staleness check should be applied, for advising an incorrect alleviation (i.e., round ID based), and/or for being of lower quality than acceptable.

# [M-05] Possible DoS When calling GammaTradeMarket::_removePosition will cause user position to not be able to get liquidated

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

GammaTradeMarket::_removePosition will cause user position to not be able to get liquidated Submitted by web3km, also found by crypticdefense, nnez, 3n0ch, and pkqs90

- https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/gamma/ArrayLib.sol#L20-L32
- https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/gamma/GammaTradeMarket.sol#L146-L149

## Impact

Griefing/DOS attack is possible when, a malicious user creates many very small positions, which could cause excessive gas consumed and even transactions reverted when other users are trying to liquidate any of the user’s positions.

## Recommended Mitigation Steps

Consider using OZ’s EnumerableSet Library, which will allow to add/remove items easily without the need to loop over every item to find the one that needs to be removed.

## Assessed type

DoS syuhei176 (Predy) confirmed 0xsomeone (judge) commented via duplicate Issue #59:

A user can maliciously craft their positions in a way that would breach the block gas limit if a user were to iterate through all of them, thereby causing fillers to be unable to fulfill trades for the account and liquidators to be unable to iterate a user’s positions.

Given that liquidations can be affected as well, the present vulnerability permits a user to craft their position array in such a way that would prevent their liquidation. This misbehavior would usually result in a high-risk severity rating, however, as some submissions noted, this Denial-of-Service can be circumvented via partial liquidations up to the maximum accuracy possible which would not render it a viable strategy. As such, a medium-risk rating is better suited for both functionalities impacted.

# [M-06] Vaults can become immune from liquidation by setting vault.recipient to a blacklisted quote token address

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

vault.recipient to a blacklisted quote token address Submitted by LuarSec, also found by Tigerfrake, Kaysoft, Takarez, 0xhere2learn, ayden, SBSecurity, erictee, DPS, Joshuajee, dyoff, 0xMilenov, forgebyola, pkqs90, gumgumzum, shaflow2, SpicyMeatball, zhaojohnson, MSaptarshi, MrCrowNFT, jolah1, WinSec, EaglesSecurity, lydia_m_t, and chista0x

- https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/PredyPool.sol#L289
- https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/libraries/logic/LiquidationLogic.sol#L99

## Impact

Vault owners can set vault.recipient via the external PredyPool.updateRecepient function to set the address that will receive the vault’s remaining positive margin when the vault’s position is liquidated, denominated in the quote token:

//File:///src/PredyPool.sol function updateRecepient ( uint256 vaultId, address recipient ) external onlyVaultOwner ( vaultId ) { DataType.

Vault storage vault = globalData.

vaults [ vaultId ]; vault.

recipient = recipient; emit RecepientUpdated ( vaultId, recipient ); } A malicious vault owner can set vault.recipient to a blacklisted/prohibited address for the quote token, such as 0x0E6b8E34dC115a2848F585851AF23D99D09b8463, which is blacklisted in Arbitrum’s USDC contract. If the remaining margin is positive, the safeTransfer operation on line 97 in LiquidationLogic.liquidate may revert, as is the case with USDC:

//File:///src/libraries/logic/LiquidationLogic.sol function liquidate ( uint256 vaultId, uint256 closeRatio, GlobalDataLibrary.GlobalData storage globalData, bytes memory settlementData ) external returns (IPredyPool.TradeResult memory tradeResult ) {...

if (!

hasPosition ) { int256 remainingMargin = vault.

margin; if ( remainingMargin > 0 ) { if ( vault.

recipient != address ( 0 )) { // Send the remaining margin to the recipient.

vault.

margin = 0; sentMarginAmount = uint256 ( remainingMargin ); ERC20 ( pairStatus.

quotePool.

token ).

safeTransfer ( vault.

recipient, sentMarginAmount ); }...

} As a new vault can be created when making a trade, any malicious user can maintain their own vault for a trade. The owner of a vault at risk of liquidation with a positive vault.margin can effectively “switch off” liquidations at will if the quote token reverts when sending tokens to blacklisted addresses, preventing the entire liquidation operation from succeeding.

This results in unsafe unliquidatable vault positions remaining in the protocol, putting both the protocol and its users at risk. This can be abused in several ways:

Malicious vault owners can frontrun liquidation attempts with their own calls to PredyPool.updateRecepient to set vault.recipient to a blacklisted quote token address, optionally reverting the change after the liquidation attempt fails.

Malicious market maintainers may be able to game the PredyPool contract by automatically setting vault recipients to blacklisted addresses at the expense of liquidators, the protocol, and other markets.

Malicious vault owners can deliberately maintain unhealthy liquidation-proof positions with the goal of driving up the protocol’s total margin.

## Recommended Mitigation Steps

Blacklisted addresses can be checked against supporting quote tokens before they are set to vault.recipient in PredyPool.updateRecepient. In the case of USDC, this would involve calling the USDC contract’s isBlacklisted method for supplied addresses. Ensure that this does introduce any callback/reentrancy issues by also making PredyPool.updateRecepient nonreentrant.

A similar check can be done prior to the safeTransfer on line 97 in LiquidationLogic.liquidate.

To prevent abuse of this issue via frontrunning, require that a vault’s position is healthy in PredyPool.updateRecepient before vault.recipient can be changed, so long as this does not interfere with any calls to PredyPool.updateRecepient made by existing market contracts.

## Assessed type

Token-Transfer 0xsomeone (judge) decreased severity to Medium and commented:

The submission and its duplicates describe a way that can prevent a user’s position from being liquidated by setting a receiver that has been blacklisted in one of the tokens officially supported by the system per the audit’s README.

This vulnerability is significant, as it would prevent any liquidation that results in a positive margin from being carried out given that a user can change the vault’s recipient at will to a blacklisted user. I believe a medium-risk rating is appropriate given that the Denial-of-Service is impermanent and liquidations can be performed at a zero or negative margin; however, I believe it is an interesting vulnerability that properly applies to the tokens officially supported in the system.

To note, any submission that does not identify the way liquidations are affected will be penalized by 25% (i.e., rewarded 75%).

syuhei176 (Predy) confirmed

# [M-07] Reallocation incorrectly sends the exceed quoteTokens to Market contract instead of reallocator

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

quoteTokens to Market contract instead of reallocator Submitted by pkqs90 Loss of funds ( quoteToken ) for reallocator if the swapped quoteTokens exceeds requirement during reallocation.

Bug Description See the reallocation flowchart here.

The reallocator would call a Market contract, which triggers the reallocation logic in PredyPool. Then tokens would then be swapped accordingly using the SettlementCallbackLib.

For baseToken, there is a check that the diff in baseToken (before/after the swap) must be equal to what is required for reallocation. For quoteToken, if the swap generates more quoteToken than what is required (e.g. the exceedsQuote > 0 case in the following code), the quoteTokens should be transferred back to the initial reallocator, since it was the reallocator who was performing the swap.

However, the quoteTokens are transferred back to msg.sender, which is the Market contract, and would cause a loss of funds for the reallocator.

function reallocate (GlobalDataLibrary.GlobalData storage globalData, uint256 pairId, bytes memory settlementData ) external returns ( bool isRangeChanged ) {...

{ int256 deltaPositionBase; int256 deltaPositionQuote; ( relocationOccurred, isRangeChanged, deltaPositionBase, deltaPositionQuote ) = Perp.

reallocate ( pairStatus, pairStatus.

sqrtAssetStatus ); if ( deltaPositionBase != 0 ) { globalData.

initializeLock ( pairId ); globalData.

callSettlementCallback ( settlementData, deltaPositionBase ); ( int256 settledQuoteAmount, int256 settledBaseAmount ) = globalData.

finalizeLock (); int256 exceedsQuote = settledQuoteAmount + deltaPositionQuote; if ( exceedsQuote < 0 ) { revert IPredyPool.

QuoteTokenNotSettled (); } if ( settledBaseAmount + deltaPositionBase != 0 ) { revert IPredyPool.

BaseTokenNotSettled (); } if ( exceedsQuote > 0 ) { > ERC20 ( pairStatus.

quotePool.

token ).

safeTransfer ( msg.

sender, uint256 ( exceedsQuote )); }...

}...

}

## Recommended Mitigation Steps

Send the tokens to the reallocator.

## Assessed type

Token-Transfer 0xsomeone (judge) decreased severity to Low and commented:

The reallocation function is indeed invoked by the BaseMarket and such an execution path would result in funds being incorrectly sent to the Market instead of the reallocators directly. Given that reallocations can occur directly, I believe that this is better suited as a QA (Low) risk as it arises under very specific circumstances and can be circumvented. Some markets also appear to be using balance of measurements and thus are more than likely to automatically “consume” those funds instead of having them remain locked. I will invite the Sponsor to visit this submission nonetheless.

pkqs90 (warden) commented:

@0xsomeone - maybe I’m missing something important here, but to my current understanding, the reallocation function must be called to change the range for squart, and this does not happen directly during regular trade/swaps.

Also, the funds sent to the Market would be unretrievable since there is no function to do so. That’s why I placed it with a high severity issue.

syuhei176 (Predy) confirmed 0xsomeone (judge) increased severity to Medium and commented:

This report lacked Sponsor input due to being invalidated during the validation round. I explicitly requested the Sponsor to provide input to this and several other exhibits so as to provide a fine-tuned judgment, and my earlier judgment was based on validation.

@pkqs90 - I agree with what you shared; however, I believe a medium-risk rating is better appropriate as the fund loss involved in this exhibit versus #27 is significantly lower and arises under specific conditions.

# [M-08] PriceFeed does not return to the correct price for quote pairs

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-predy
- **Source snapshot:** competitions/2024-05-predy/final_report.html

Submitted by gumgumzum

- https://github.com/code-423n4/2024-05-predy/blob/main/src/PriceFeed.sol#L29-L59
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/PositionCalculator.sol#L141-L149
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/PositionCalculator.sol#L83
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/PositionCalculator.sol#L70
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/PositionCalculator.sol#L41
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/PositionCalculator.sol#L56
- https://github.com/code-423n4/2024-05-predy/blob/main/src/markets/perp/PerpMarketV1.sol#L227
- https://github.com/code-423n4/2024-05-predy/blob/main/src/markets/perp/PerpMarketV1.sol#L114-L115
- https://github.com/code-423n4/2024-05-predy/blob/main/src/markets/gamma/GammaTradeMarket.sol#L237
- https://github.com/code-423n4/2024-05-predy/blob/main/src/markets/gamma/GammaTradeMarket.sol#L286
- https://github.com/code-423n4/2024-05-predy/blob/main/src/markets/gamma/GammaTradeMarket.sol#L346
- https://github.com/code-423n4/2024-05-predy/blob/main/src/PredyPool.sol#L352-L354
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/logic/TradeLogic.sol#L55-L56
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/logic/TradeLogic.sol#L47-L48
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/logic/ReaderLogic.sol#L35-L36
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/logic/LiquidationLogic.sol#L75-L76
- https://github.com/code-423n4/2024-05-predy/blob/main/src/libraries/logic/LiquidationLogic.sol#L141-L142

## Impact

Discrepancy in PositionCalculator@getSqrtIndexPrice (cascading down the line to anywhere it is used) when using a PriceFeed or not.

This makes parts of the system unusable for quote pairs (e.g., when selling, position is not safe in PerpMarket since the vault value ends up being very low and the vault margin being negative) and may lead to other problems if at first a PriceFeed is not used but then is updated later since the computed state will change after the update.

Root Cause contract PriceFeed { address private immutable _quotePriceFeed; address private immutable _pyth; uint256 private immutable _decimalsDiff; // <==== Audit bytes32 private immutable _priceId; uint256 private constant VALID_TIME_PERIOD = 5 * 60; constructor ( address quotePrice, address pyth, bytes32 priceId, uint256 decimalsDiff ) { // <==== Audit _quotePriceFeed = quotePrice; _pyth = pyth; _priceId = priceId; _decimalsDiff = decimalsDiff; // <==== Audit } /// @notice This function returns the square root of the baseToken price quoted in quoteToken.

function getSqrtPrice () external view returns ( uint256 sqrtPrice ) { (, int256 quoteAnswer,,,) = AggregatorV3Interface ( _quotePriceFeed ).

latestRoundData (); IPyth.

Price memory basePrice = IPyth ( _pyth ).

getPriceNoOlderThan ( _priceId, VALID_TIME_PERIOD ); require ( basePrice.

expo == - 8, "INVALID_EXP" ); require ( quoteAnswer > 0 && basePrice.

price > 0 ); uint256 price = uint256 ( int256 ( basePrice.

price )) * Constants.

Q96 / uint256 ( quoteAnswer ); price = price * Constants.

Q96 / _decimalsDiff; // <==== Audit sqrtPrice = FixedPointMathLib.

sqrt ( price ); } It’s not possible to create a PriceFeed with a fraction as decimalsDiff. For base pairs, it’s not an issue, but for quote pairs it makes it impossible to get the correct price.

For example, for the USDC/WETH, the base pair PriceFeed would have a 1e12 decimalsDiff but the quote pair PriceFeed should have a 1e12 decimalsDiff which is not possible (the lower you can go is 1e0 ).

Test pragma solidity ^ 0.8.

0; import { Test } from "forge-std/Test.sol"; import { ERC20 } from "@solmate/src/tokens/ERC20.sol"; import { IUniswapV3Factory } from "@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol"; import { AddPairLogic, PredyPool, Perp, IPredyPool } from "../../src/PredyPool.sol"; import { PriceFeed } from "../../src/PriceFeed.sol"; import { InterestRateModel } from "../../src/libraries/InterestRateModel.sol"; contract PriceFeedDiscrepancyTest is Test { PredyPool predyPool; uint128 internal constant RISK_RATIO = 109544511; uint128 internal constant BASE_MIN_COLLATERAL_WITH_DEBT = 2000; address admin = vm.

addr ( 100 ); ERC20 USDC = ERC20 ( 0xaf88d065e77c8cC2239327C5EDb3A432268e5831 ); ERC20 WETH = ERC20 ( 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1 ); bytes32 USDC_PYTH_PRICE_ID = 0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a; bytes32 WETH_PYTH_PRICE_ID = 0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace; address USDC_CHAINLINK_PRICE_FEED = 0x50834F3163758fcC1Df9973b6e91f0F0F0434aD3; address WETH_CHAINLINK_PRICE_FEED = 0x639Fe6ab55C921f74e7fac1ee960C0B6293ba612; IUniswapV3Factory uniswapFactory = IUniswapV3Factory ( 0x1F98431c8aD98523631AE4a59f267346ea31F984 ); address pyth = 0xff1a0f4744e8582DF1aE09D5611b887B6a12925C; function setUp () public { vm.

label ( admin, "ADMIN" ); vm.

label ( address ( USDC ), "USDC" ); vm.

label ( address ( WETH ), "WETH" ); vm.

createSelectFork ( vm.

envString ( "ALCHEMY_RPC_URL" )); vm.

startPrank ( admin ); predyPool = new PredyPool (); predyPool.

initialize ( address ( uniswapFactory )); } function testSqrtIndexPrice () public { vm.

startPrank ( admin ); PriceFeed quotePriceFeed = new PriceFeed ( WETH_CHAINLINK_PRICE_FEED, pyth, USDC_PYTH_PRICE_ID, 1e0 ); PriceFeed basePriceFeed = new PriceFeed ( USDC_CHAINLINK_PRICE_FEED, pyth, WETH_PYTH_PRICE_ID, 1e12 ); address pool = uniswapFactory.

getPool ( address ( USDC ), address ( WETH ), 500 ); uint256 quotePair = registerPair ( pool, address ( WETH ), address ( 0 ), true ); uint256 basePair = registerPair ( pool, address ( USDC ), address ( 0 ), true ); uint256 quoteIndexPriceBefore = predyPool.

getSqrtIndexPrice ( quotePair ); uint256 baseIndexPriceBefore = predyPool.

getSqrtIndexPrice ( basePair ); predyPool.

updatePriceOracle ( quotePair, address ( quotePriceFeed )); predyPool.

updatePriceOracle ( basePair, address ( basePriceFeed )); assertApproxEqRel ( baseIndexPriceBefore, predyPool.

getSqrtIndexPrice ( basePair ), 1e16 ); assertApproxEqRel ( quoteIndexPriceBefore, predyPool.

getSqrtIndexPrice ( quotePair ), 1e16 ); } function registerPair ( address pool, address quoteToken, address priceFeed, bool isWhitelistEnabled ) internal returns ( uint256 ) { InterestRateModel.

IRMParams memory irmParams = InterestRateModel.

IRMParams ( 1e16, 9 * 1e17, 5 * 1e17, 1e18 ); return predyPool.

registerPair ( AddPairLogic.

AddPairParams ( quoteToken, admin, pool, priceFeed, isWhitelistEnabled, 0, Perp.

AssetRiskParams ( RISK_RATIO, BASE_MIN_COLLATERAL_WITH_DEBT, 1000, 500, 1005000, 1050000 ), irmParams, irmParams ) ); } Results ❯ forge test --match-contract PriceFeedDiscrepancyTest -vv [⠢] Compiling...

No files changed, compilation skipped Ran 1 test for test/fork/PriceFeedDiscrepancy.t.sol:PriceFeedDiscrepancyTest [FAIL. Reason: assertion failed] testSqrtIndexPrice() (gas: 4948278) Logs:

Error: a ~= b not satisfied [uint] Left: 1333136901050186247877690170972068 Right: 1331884368549106242288122906 Max % Delta: 1.000000000000000000 % Delta: 100093942.135387808257073200 Suite result: FAILED. 0 passed; 1 failed; 0 skipped; finished in 17.83ms (4.19ms CPU time) Ran 1 test suite in 342.31ms (17.83ms CPU time): 0 tests passed, 1 failed, 0 skipped (1 total tests) Failing tests:

Encountered 1 failing test in test/fork/PriceFeedDiscrepancy.t.sol:PriceFeedDiscrepancyTest [FAIL. Reason: assertion failed] testSqrtIndexPrice() (gas: 4948278) Encountered a total of 1 failing tests, 0 tests succeeded

## Recommended Mitigation Steps

diff -- git a / src / PriceFeed.

sol b / src / PriceFeed.

sol index a8cd985..

bd0152d 100644 --- a / src / PriceFeed.

sol +++ b / src / PriceFeed.

sol @@ - 9, 13 + 9, 13 @@ import { Constants } from "./libraries/Constants.sol"; contract PriceFeedFactory { address private immutable _pyth; - event PriceFeedCreated ( address quotePrice, bytes32 priceId, uint256 decimalsDiff, address priceFeed ); + event PriceFeedCreated ( address quotePrice, bytes32 priceId, int256 decimalsDiff, address priceFeed ); constructor ( address pyth ) { _pyth = pyth; } - function createPriceFeed ( address quotePrice, bytes32 priceId, uint256 decimalsDiff ) external returns ( address ) { + function createPriceFeed ( address quotePrice, bytes32 priceId, int256 decimalsDiff ) external returns ( address ) { PriceFeed priceFeed = new PriceFeed ( quotePrice, _pyth, priceId, decimalsDiff ); emit PriceFeedCreated ( quotePrice, priceId, decimalsDiff, address ( priceFeed )); @@ - 29, 12 + 29, 12 @@ contract PriceFeedFactory { contract PriceFeed { address private immutable _quotePriceFeed; address private immutable _pyth; - uint256 private immutable _decimalsDiff; + int256 private immutable _decimalsDiff; bytes32 private immutable _priceId; uint256 private constant VALID_TIME_PERIOD = 5 * 60; - constructor ( address quotePrice, address pyth, bytes32 priceId, uint256 decimalsDiff ) { + constructor ( address quotePrice, address pyth, bytes32 priceId, int256 decimalsDiff ) { _quotePriceFeed = quotePrice; _pyth = pyth; _priceId = priceId; @@ - 52, 7 + 52, 12 @@ contract PriceFeed { require ( quoteAnswer > 0 && basePrice.

price > 0 ); uint256 price = uint256 ( int256 ( basePrice.

price )) * Constants.

Q96 / uint256 ( quoteAnswer ); - price = price * Constants.

Q96 / _decimalsDiff; + if ( _decimalsDiff > 0 ) { + price = price * Constants.

Q96 / 10 ** uint256 ( _decimalsDiff ); + } else { + price = price * Constants.

Q96 * 10 ** uint256 (- _decimalsDiff ); + } sqrtPrice = FixedPointMathLib.

sqrt ( price ); }

## Assessed type

Oracle syuhei176 (Predy) disputed and commented:

I don’t understand the issue you’re pointing out. The PriceFeed represents the square root of the price of the baseToken in terms of the quoteToken, and it’s not necessary to have separate prices for both the baseToken and the quoteToken.

As for the recommended mitigation steps, it’s using (- _decimalsDiff ) in the condition where _decimalsDiff is negative, and it seems doing the same thing for both conditions.

0xsomeone (judge) commented:

The Warden states that the price normalization step in the PriceFeed contract does not account for a “negative” decimal difference (i.e., requiring a multiplication instead of a division).

I do not believe the described vulnerability holds true as the oracle can always be created in the “positive” decimal direction, and the square root encoded price can be decoded and utilized in the inverse direction. As such, a PriceFeed configured for the WETH/USDC direction can be used in the USDC/WETH direction without necessitating a different PriceFeed deployment.

gumgumzum (warden) commented:

@0xsomeone - That is unfortunately incorrect due to how PositionCalculator@getSqrtIndexPrice works:

function getSqrtIndexPrice (DataType.PairStatus memory pairStatus ) internal view returns ( uint256 sqrtPriceX96 ) { if ( pairStatus.

priceFeed != address ( 0 )) { return PriceFeed ( pairStatus.

priceFeed ).

getSqrtPrice (); } else { return UniHelper.

convertSqrtPrice ( UniHelper.

getSqrtTWAP ( pairStatus.

sqrtAssetStatus.

uniswapPool ), pairStatus.

isQuoteZero ); } When using a PriceFeed, it directly returns the price returned from the PriceFeed regardless of whether the pair is a base or a quote pair. Otherwise, it uses the Uniswap Sqrt TWAP and converts it based on whether the pair is a base pair or a quote pair.

So it’s not possible to use the same PriceFeed for both a base and quote pair otherwise PositionCalculator@getSqrtIndexPrice would return the same price for both pairs which is wrong.
