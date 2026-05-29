# Accepted H/M Findings: DittoETH Invitational

# [H-01] Attacker can profit from discount fees

- **Contest:** DittoETH Invitational
- **Slug:** 2024-07-dittoeth-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-dittoeth-invitational
- **Source snapshot:** competitions/2024-07-dittoeth-invitational/final_report.html

Submitted by d3e4

## Impact

It is possible that the amount of dUSD minted in discount fees are greater than the discount loss. An attacker can therefore deliberately trigger a fee and, provided he has a large stake in the yDUSD vault, he can claim more dUSD in fees, than lost from the trade.

The root cause is that the discount fee is fixed in proportion only to the entire debt.

# [H-02] DUSD assets can be minted with less ETH collateral than required

- **Contest:** DittoETH Invitational
- **Slug:** 2024-07-dittoeth-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-dittoeth-invitational
- **Source snapshot:** competitions/2024-07-dittoeth-invitational/final_report.html

DUSD assets can be minted with less ETH collateral than required Submitted by serial-coder, also found by 0xbepresent Summary I discovered that the current implementation has not fixed the issue H-03 (Users can mint DUSD with less collateral than required which gives them free DUSD and may open a liquidatable position) raised in the previous C4 audit.

Description To mint the DUSD assets with less collateral than required, a user or attacker executes the OrdersFacet::cancelShort() to cancel the shortOrder with its shortRecord.ercDebt < minShortErc (i.e., shortRecord.status == SR.PartialFill ).

The OrdersFacet::cancelShort() will execute another internal function, LibOrders::cancelShort() ( @1 in the snippet below), to do the short canceling job. If the shortOrder ’s corresponding shortRecord.status == SR.PartialFill and has shortRecord.ercDebt < minShortErc, the steps @2.1 and @2.2 will get through.

Since the shortRecord is less than the minShortErc, the cancelShort() has to fill an ercDebt for more to reach the minShortErc threshold (so that the partially filled shortRecord.ercDebt will == minShortErc ). Specifically, the function has to virtually mint the DUSD assets to increase the ercDebt by spending the shortRecord.collateral (Let’s name it the collateralDiff ) for exchange.

Here, we come to the root cause in @3. To calculate the collateralDiff:

The shortOrder.price is used instead of the current price. Nevertheless, the shortOrder.price can be stale (less or higher than the current price).

The shortOrder.shortOrderCR (i.e., the cr variable in the snippet below) is used, which can be less than 100% CR.

If the shortOrder.price is less than the current price and/or the shortOrder.shortOrderCR is less than 100% CR, the calculated collateralDiff will have a value less than the value of the DUSD assets that get minted ( @4 ).

// FILE: https://github.com/code-423n4/2024-07-dittoeth/blob/main/contracts/facets/OrdersFacet.sol function cancelShort ( address asset, uint16 id ) external onlyValidAsset ( asset ) nonReentrant { STypes.

Order storage short = s.

shorts [ asset ][ id ]; if ( msg.

sender != short.

addr ) revert Errors.

NotOwner (); if ( short.

orderType != O.

LimitShort ) revert Errors.

NotActiveOrder (); //@audit @1 -- Execute the cancelShort() to cancel the shortOrder with // its shortRecord.ercDebt < minShortErc (SR.PartialFill).

@ 1 LibOrders.

cancelShort ( asset, id ); } // FILE: https://github.com/code-423n4/2024-07-dittoeth/blob/main/contracts/libraries/LibOrders.sol function cancelShort ( address asset, uint16 id ) internal {...

if ( shortRecord.

status == SR.

Closed ) {...

@ 2.1 } else { //@audit @2.1 -- shortRecord.status == SR.PartialFill uint256 minShortErc = LibAsset.

minShortErc ( Asset ); @ 2.2 if ( shortRecord.

ercDebt < minShortErc ) { //@audit @2.2 -- shortRecord.ercDebt < minShortErc // @dev prevents leaving behind a partially filled SR under minShortErc // @dev if the corresponding short is cancelled, then the partially filled SR's debt will == minShortErc uint88 debtDiff = uint88 ( minShortErc - shortRecord.

ercDebt ); // @dev(safe-cast) { STypes.

Vault storage Vault = s.

vault [ vault ]; //@audit @3 -- To calculate the collateralDiff:

// 1) The shortOrder.price is used instead of the current price.

// -> The shortOrder.price can be stale (less or higher than the current price).

// // 2) The shortOrder.shortOrderCR (i.e., cr) is used, which can be less than 100% CR.

@ 3 uint88 collateralDiff = shortOrder.

price.

mulU88 ( debtDiff ).

mulU88 ( cr ); LibShortRecord.

fillShortRecord ( asset, shorter, shortRecordId, SR.

FullyFilled, @ 4 collateralDiff, //@audit @4 -- The collateralDiff's value can be less than the value of the DUSD assets that get minted.

debtDiff, Asset.

ercDebtRate, Vault.

dethYieldRate, 0 ); Vault.

dethCollateral += collateralDiff; Asset.

dethCollateral += collateralDiff; Asset.

ercDebt += debtDiff; // @dev update the eth refund amount eth -= collateralDiff; } // @dev virtually mint the increased debt s.

assetUser [ asset ][ shorter ].

ercEscrowed += debtDiff; } else {...

}...

} @1:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/facets/OrdersFacet.sol#L60
@2.1:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/libraries/LibOrders.sol#L944
@2.2:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/libraries/LibOrders.sol#L946
@3:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/libraries/LibOrders.sol#L953
@4:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/libraries/LibOrders.sol#L960

## Impact

Users or attackers can mint the DUSD assets with less ETH collateral than required (i.e., free money). This vulnerability is critical and can lead to the de-pegging of the DUSD token.

## Recommended Mitigation Steps

When calculating the collateralDiff:

Use the current price instead of the shortOrder.price.

If the shortOrder.shortOrderCR < initialCR, use the initialCR as the collateral ratio instead of the shortOrder.shortOrderCR.

Note: I have slightly modified the original recommended code of nonseodion to make it work with the current codebase. Thanks to nonseodion again.

// Credit:

// - Original by: nonseodion // - Modified by: serial-coder function cancelShort(address asset, uint16 id) internal {...

if (shortRecord.status == SR.Closed) {...

} else { uint256 minShortErc = LibAsset.minShortErc(Asset); if (shortRecord.ercDebt < minShortErc) { // @dev prevents leaving behind a partially filled SR under minShortErc // @dev if the corresponding short is cancelled, then the partially filled SR's debt will == minShortErc uint88 debtDiff = uint88(minShortErc - shortRecord.ercDebt); // @dev(safe-cast) { STypes.Vault storage Vault = s.vault[vault]; - uint88 collateralDiff = shortOrder.price.mulU88(debtDiff).mulU88(cr); + uint256 newCR = convertCR( + shortOrder.shortOrderCR < s.asset[asset].initialCR ? s.asset[asset].initialCR: shortOrder.shortOrderCR + ); + uint80 price = uint80(LibOracle.getSavedOrSpotOraclePrice(asset)); + uint88 collateralDiff = price.mulU88(debtDiff).mulU88(newCR);

LibShortRecord.fillShortRecord( asset, shorter, shortRecordId, SR.FullyFilled, collateralDiff, debtDiff, Asset.ercDebtRate, Vault.dethYieldRate, 0 ); Vault.dethCollateral += collateralDiff; Asset.dethCollateral += collateralDiff; Asset.ercDebt += debtDiff; // @dev update the eth refund amount eth -= collateralDiff; } // @dev virtually mint the increased debt s.assetUser[asset][shorter].ercEscrowed += debtDiff; } else {...

}...

} ditto-eth (sponsor) confirmed via duplicate issue #11 Note: see original submission for full discussion.

# [H-03] Incorrect accounting bug of the yDUSD vault leads to total loss of depositors’ DUSD assets

- **Contest:** DittoETH Invitational
- **Slug:** 2024-07-dittoeth-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-dittoeth-invitational
- **Source snapshot:** competitions/2024-07-dittoeth-invitational/final_report.html

yDUSD vault leads to total loss of depositors’ DUSD assets Submitted by serial-coder, also found by d3e4 Summary The current implementation of the yDUSD vault does not properly support the auto-compounding token rewards mechanism by directly minting the DUSD assets to the vault.

Due to an incorrect accounting bug of the vault’s total supply (shares), depositors can lose some deposited DUSD assets (principal) or even all the assets when the Ditto protocol mints DUSD debts to the vault to account for any discounts.

Description When the match price is below the oracle price, the OrdersFacet::_matchIsDiscounted() will be invoked to account for a discount by minting the discount ( newDebt ) ( @1 in the snippet below) to the yDUSD vault.

Assuming the yDUSD vault is empty (i.e., both totalAssets and totalSupply are 0), for simplicity’s sake. This step will increase the vault’s total DUSD assets ( totalAssets — spot balance) without updating the vault’s total supply ( totalSupply — tracked shares).

function _matchIsDiscounted (MTypes.HandleDiscount memory h ) external onlyDiamond {...

if ( pctOfDiscountedDebt > C.

DISCOUNT_THRESHOLD && !

LibTStore.

isForcedBid ()) {...

// @dev Increase global ercDebt to account for the increase debt owed by shorters uint104 newDebt = uint104 ( ercDebtMinusTapp.

mul ( discountPenaltyFee )); Asset.

ercDebt += newDebt; Asset.

ercDebtFee += uint88 ( newDebt ); // should be uint104?

// @dev Mint dUSD to the yDUSD vault for // Note: Does not currently handle mutli-asset @ 1 IERC20 ( h.

asset ).

mint ( s.

yieldVault [ h.

asset ], newDebt ); //@audit @1 -- When the match price is below the oracle price, the _matchIsDiscounted() // will be invoked to account for a discount by minting the discount (newDebt) // to the yDUSD vault.

// // Assuming that the yDUSD vault is empty (i.e., totalAssets and totalSupply are 0), // for simplicity's sake.

// // This step will increase the vault's total DUSD assets (totalAssets -- spot balance) // without updating the vault's total supply (totalSupply -- tracked shares).

} @1:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/facets/OrdersFacet.sol#L178
After @1, when a user deposits their DUSD assets to the yDUSD vault, the ERC4626::previewDeposit() ( @2 in the snippet below) will be executed to calculate the shares based on the deposited assets.

An incorrect accounting bug of the vault’s total supply (tracked shares) occurs in @1 above. In this example, the calculated shares will be 0 since the totalSupply is 0 (if the totalSupply != 0, the calculated shares can be less than expected). For more details, refer to @2.1 below.

Since the calculated shares == 0, the user will receive 0 shares and lose all deposited DUSD assets ( @3 ). Even the slippage protection check ( @4 ) cannot detect the invalid calculation due to the slippage.mul(shares) == 0, and the user’s tracked shares remain unchanged.

(Actually, I discovered another issue regarding the slippage protection check, which will be reported separately.) function deposit ( uint256 assets, address receiver ) public override returns ( uint256 ) { if ( assets > maxDeposit ( receiver )) revert Errors.

ERC4626DepositMoreThanMax (); //@audit @2 -- After @1, when a user deposits their DUSD assets to the yDUSD vault, the previewDeposit() // will be executed to calculate the shares based on the deposited assets.

// // Due to an incorrect accounting bug of the vault's total supply (tracked shares) occurs in @1, // the calculated shares, in this case, will be 0 since totalSupply is 0 (if totalSupply != 0, // the calculated shares can be less than expected). For more details, refer to @2.1.

@ 2 uint256 shares = previewDeposit ( assets ); uint256 oldBalance = balanceOf ( receiver ); //@audit @3 -- Since the calculated shares == 0, the user will receive 0 shares. Thus, they will lose all // deposited DUSD assets.

@ 3 _deposit ( _msgSender (), receiver, assets, shares ); uint256 newBalance = balanceOf ( receiver ); // @dev Slippage is likely irrelevant for this. Merely for preventative purposes uint256 slippage = 0.01 ether; @ 4 if ( newBalance < slippage.

mul ( shares ) + oldBalance ) revert Errors.

ERC4626DepositSlippageExceeded (); //@audit @4 -- Even the slippage protection check above cannot detect the invalid calculation since // the slippage.mul(shares) == 0, and the user's tracked shares remain unchanged.

return shares; } @2:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/tokens/yDUSD.sol#L69
@3:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/tokens/yDUSD.sol#L72
@4:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/tokens/yDUSD.sol#L77
To calculate the user’s shares, the ERC4626::previewDeposit() calls the ERC4626::_convertToShares() ( @2.1 in the snippet below).

The calculated shares will be 0 (due to the rounding down) ( @2.2 ) because the ERC20::totalSupply() will return 0 (the vault’s tracked total shares) while the ERC4626::totalAssets() will return the previously minted discount amount (i.e., the newDebt from @1 ). Refer to the @2.2 for a detailed explanation of the calculation.

// FILE: node_modules/@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol function previewDeposit ( uint256 assets ) public view virtual override returns ( uint256 ) { //@audit @2.1 -- The previewDeposit() calls the _convertToShares() to calculate the shares.

@ 2.1 return _convertToShares ( assets, Math.

Rounding.

Down ); } // FILE: node_modules/@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol function _convertToShares ( uint256 assets, Math.Rounding rounding ) internal view virtual returns ( uint256 ) { //@audit @2.2 -- The calculated shares, in this case, will be 0 (due to the rounding down) // because the totalSupply() will return 0 (the vault's tracked total shares) // while the totalAssets() will return the previously minted discount amount // (i.e., the newDebt from @1).

// // shares = assets * (0 + 10 ** 0) / newDebt // = assets * 1 / newDebt (e.g., assets < newDebt) // = 0 (due to rounding down) @ 2.2 return assets.

mulDiv ( totalSupply () + 10 ** _decimalsOffset (), totalAssets () + 1, rounding ); } @2.1:

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/4fd2f8be339e850c32206342c3f9a1a7bedbb204/contracts/token/ERC20/extensions/ERC4626.sol#L134
@2.2:

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/4fd2f8be339e850c32206342c3f9a1a7bedbb204/contracts/token/ERC20/extensions/ERC4626.sol#L200
As you can see, the ERC20::totalSupply() returns the vault’s tracked total shares (0) ( @2.2.1 in the snippet below), and ERC4626::totalAssets() returns the previously minted discount amount (i.e., the newDebt from @1 ) ( @2.2.2 ).

// FILE: node_modules/@openzeppelin/contracts/token/ERC20/ERC20.sol function totalSupply () public view virtual override returns ( uint256 ) { //@audit @2.2.1 -- The totalSupply() returns the vault's tracked total shares (0).

@ 2.2.

1 return _totalSupply; //@audit -- tracked shares } // FILE: node_modules/@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol function totalAssets () public view virtual override returns ( uint256 ) { //@audit @2.2.2 -- The totalAssets() returns the previously minted discount amount // (i.e., the newDebt from @1).

@ 2.2.

2 return _asset.

balanceOf ( address ( this )); //@audit -- spot balance } @2.2.1:

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/4fd2f8be339e850c32206342c3f9a1a7bedbb204/contracts/token/ERC20/ERC20.sol#L94
@2.2.2:

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/4fd2f8be339e850c32206342c3f9a1a7bedbb204/contracts/token/ERC20/extensions/ERC4626.sol#L99

## Impact

Depositors can lose some deposited DUSD assets (principal) or whole assets when the Ditto protocol mints DUSD debts to the yDUSD vault to account for discounts.

## Recommended Mitigation Steps

Rework the yDUSD vault by applying the concept of a single-sided auto-compounding token rewards mechanism of the xERC4626, which is fully compatible with the ERC4626 and ultimately maintains balances using internal accounting to prevent instantaneous changes in the exchange rate.

@ditto-eth (sponsor) acknowledged Note: see original submission for full discussion.

# [H-04] An attacker can mint free DUSD and liquidate the corresponding Short Record to earn liquidation rewards

- **Contest:** DittoETH Invitational
- **Slug:** 2024-07-dittoeth-invitational
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-dittoeth-invitational
- **Source snapshot:** competitions/2024-07-dittoeth-invitational/final_report.html

Submitted by nonseodion Summary An attacker can use decreaseCollateral() function to reduce the collateral of a Short Recordn (SR) and cancelOrder() to cancel the corresponding Short Order having a collateral ratio < 1. The Short Record ends up having less collateral than debt and is now liquidatable.

Description A user can create a Short Order with a collateral ratio (CR) of less than 1. The protocol ensures that at least minShortErc in the Short Orders Short Record has enough collateral by filling the Short Record with collateral to cover minShortErc before the Short Order is matched or placed on the order book.

In addition, during the match the bid also provides equivalent collateral to the ercDebt it creates. So if the Short Order CR is 0.7 it ends up being 1.7 CR after matching.

By utilising the cancelShort() and decreaseCollateral() functions on a short Order with CR < 1, an attacker can create free DUSD and make the Short Record liquidatable.

The cancelShort() function lets the caller cancel a Short Order. If the ercDebt is smaller than minShortErc as shown in line 953 below, it mints the difference to the shorter (i.e the address that created the Short Order) in line 982. The amount of collateral needed is calculated in line 960 and uses cr.

cr is the collateral ratio and if it is less than 1 then the protocol would be minting DUSD with lesser collateral. But since it has already ensured that minShortErc has enough collateral in the Short Record before the Short Order was created, this shouldn’t be a problem.

LibOrders.sol#L946-L976 953:

if ( shortRecord.

ercDebt < minShortErc ) { 954:

// @dev prevents leaving behind a partially filled SR under minShortErc 955:

// @dev if the corresponding short is cancelled, then the partially filled SR's debt will == minShortErc 956:

uint88 debtDiff = uint88 ( minShortErc - shortRecord.

ercDebt ); // @dev(safe-cast) 957: { 958:

STypes.

Vault storage Vault = s.

vault [ vault ]; 959:

// @audit-issue the collateral could be bad if price moved.

960:

uint88 collateralDiff = shortOrder.

price.

mulU88 ( debtDiff ).

mulU88 ( cr ); 961:

962:

LibShortRecord.

fillShortRecord ( 963:

asset, 964:

shorter, 965:

shortRecordId, 966:

SR.

FullyFilled, 967:

collateralDiff, 968:

debtDiff, 969:

Asset.

ercDebtRate, 970:

Vault.

dethYieldRate, 971:

0 972: ); 973:

974:

Vault.

dethCollateral += collateralDiff; 975:

Asset.

dethCollateral += collateralDiff; 976:

Asset.

ercDebt += debtDiff; 977:

978:

// @dev update the eth refund amount 979:

eth -= collateralDiff; 980: } 981:

// @dev virtually mint the increased debt 982:

s.

assetUser [ asset ][ shorter ].

ercEscrowed += debtDiff; 983: } else { The decreaseCollateral() function lets the caller reduce the collateral he has in the Short Record.

An attacker can use decreaseCollateral() to reduce the collateral of the Short Record before cancelling. Thus, removing the collateral added for minShortErc and the protocol ends up minting DUSD with less Ethereum collateral.

The attacker earns the DUSD minted while providing less collateral value. The Short Record will become liquidatable and the attacker can further liquidate it to earn liquidation rewards.

## Impact

The protocol mints DUSD for free and pays liquidation rewards when the bad debt position is liquidated.

## Recommended Mitigation Steps

Consider adding a check to cancelOrder() to ensure the resulting SR Collateral Ratio is not below the redemption and liquidation collateral ratios.

if ( cRatio < LibOrders.

max ( LibAsset.

liquidationCR ( asset ), C.

MAX_REDEMPTION_CR )) { revert Errors.

ShortBelowCRThreshold (); } ditto-eth (sponsor) confirmed and commented:

Good find, will fix this issue in decreaseCollateral() instead of cancelShort().

Medium Risk Findings (1)

# [M-01] Users can evade the yDUSD vault’s withdrawal timelock mechanism Disclosures Overview About C4 Code4rena (C4) is an open organization consisting of security researchers, auditors, developers, and individuals with domain expertise in smart contracts.

- **Contest:** DittoETH Invitational
- **Slug:** 2024-07-dittoeth-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-dittoeth-invitational
- **Source snapshot:** competitions/2024-07-dittoeth-invitational/final_report.html

yDUSD vault’s withdrawal timelock mechanism Submitted by serial-coder Summary The yDUSD vault’s withdrawal timelock mechanism is vulnerable. An exploiter can withdraw the DUSD assets in their account by using another account’s proposedWithdraw request info, breaking the vault’s core invariant.

Description Assuming Bob is an exploiter, he can evade his account (i.e., owner ) from the yDUSD vault’s withdrawal timelock mechanism by using the proposeWithdraw request info of another account (i.e., msg.sender ) ( @1 in the snippet below), which can be requested beforehand.

This way, he can withdraw his DUSD assets in the owner account from the vault without waiting for the 7-day period, breaking the invariant of the vault’s timelock mechanism.

With the msg.sender ’s proposeWithdraw request info, the 7-day timelock period check ( @2 ) can be passed. After passing the withdrawal timelock check in @2, the DUSD assets in the owner account ( @3 ) can be withdrawn without invoking the yDUSD::proposeWithdraw().

function withdraw ( uint256 assets, address receiver, address owner ) public override returns ( uint256 ) { //@audit @1 -- Bob can evade his account (i.e., owner) from the vault's withdrawal timelock mechanism by using // the proposeWithdraw info of another account (i.e., msg.sender), which can be requested beforehand.

// // This way, he can withdraw his DUSD assets in the 'owner' account from the vault without // waiting for the 7-day period, breaking the invariant of the vault's timelock mechanism.

@ 1 WithdrawStruct storage withdrawal = withdrawals [ msg.

sender ]; uint256 amountProposed = withdrawal.

amountProposed; uint256 timeProposed = withdrawal.

timeProposed; if ( timeProposed == 0 && amountProposed <= 1 ) revert Errors.

ERC4626ProposeWithdrawFirst (); //@audit @2 -- With the msg.sender's proposeWithdraw request info, the 7-day timelock period check can be passed.

@ 2 if ( timeProposed + C.

WITHDRAW_WAIT_TIME > uint40 ( block.

timestamp )) revert Errors.

ERC4626WaitLongerBeforeWithdrawing (); // @dev After 7 days from proposing, a user has 45 days to withdraw // @dev User will need to cancelWithdrawProposal() and proposeWithdraw() again if ( timeProposed + C.

WITHDRAW_WAIT_TIME + C.

MAX_WITHDRAW_TIME <= uint40 ( block.

timestamp )) { revert Errors.

ERC4626MaxWithdrawTimeHasElapsed (); } if ( amountProposed > maxWithdraw ( owner )) revert Errors.

ERC4626WithdrawMoreThanMax (); checkDiscountWindow (); uint256 shares = previewWithdraw ( amountProposed ); IAsset _dusd = IAsset ( dusd ); uint256 oldBalance = _dusd.

balanceOf ( receiver ); //@audit @3 -- After passing the timelock check in @2, the DUSD assets in Bob's owner account can be withdrawn // without invoking the proposeWithdraw().

@ 3 _withdraw ( _msgSender (), receiver, owner, amountProposed, shares ); uint256 newBalance = _dusd.

balanceOf ( receiver ); // @dev Slippage is likely irrelevant for this. Merely for preventative purposes uint256 slippage = 0.01 ether; if ( newBalance < slippage.

mul ( amountProposed ) + oldBalance ) revert Errors.

ERC4626WithdrawSlippageExceeded (); delete withdrawal.

timeProposed; //reset withdrawal (1 to keep slot warm) withdrawal.

amountProposed = 1; return shares; } @1:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/tokens/yDUSD.sol#L85
@2:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/tokens/yDUSD.sol#L91
@3:

- https://github.com/code-423n4/2024-07-dittoeth/blob/ca3c5bf8e13d0df6a2c1f8a9c66ad95bbad35bce/contracts/tokens/yDUSD.sol#L107
To exploit the vulnerability, Bob must approve the shares transfer of the owner account ( @3.1 in the snippet below) for the withdrawer account (i.e., msg.sender ).

After @3.1, the ERC4626::_withdraw() will burn the shares from the owner account ( @3.2 ), and then the DUSD assets in the owner account ( @3.3 ) will eventually be transferred out.

// FILE: node_modules/@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol function _withdraw ( address caller, address receiver, address owner, uint256 assets, uint256 shares ) internal virtual { //@audit @3.1 -- To exploit the vulnerability, Bob must approve the shares transfer of the 'owner' account // for the withdrawer account (i.e., msg.sender).

@ 3.1 if ( caller != owner ) { @ 3.1 _spendAllowance ( owner, caller, shares ); @ 3.1 } // If _asset is ERC777, `transfer` can trigger a reentrancy AFTER the transfer happens through the // `tokensReceived` hook. On the other hand, the `tokensToSend` hook, that is triggered before the transfer, // calls the vault, which is assumed not malicious.

// // Conclusion: we need to do the transfer after the burn so that any reentrancy would happen after the // shares are burned and after the assets are transferred, which is a valid state.

//@audit @3.2 -- The _withdraw() will burn the shares from the 'owner' account.

@ 3.2 _burn ( owner, shares ); //@audit @3.3 -- Then, the DUSD assets in the 'owner' account will eventually be transferred out.

@ 3.3 SafeERC20.

safeTransfer ( _asset, receiver, assets ); emit Withdraw ( caller, receiver, owner, assets, shares ); } @3.1:

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/4fd2f8be339e850c32206342c3f9a1a7bedbb204/contracts/token/ERC20/extensions/ERC4626.sol#L237-L239
@3.2:

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/4fd2f8be339e850c32206342c3f9a1a7bedbb204/contracts/token/ERC20/extensions/ERC4626.sol#L247
@3.3:

- https://github.com/OpenZeppelin/openzeppelin-contracts/blob/4fd2f8be339e850c32206342c3f9a1a7bedbb204/contracts/token/ERC20/extensions/ERC4626.sol#L248

## Impact

Users can withdraw their DUSD assets from the yDUSD vault without waiting for the 7-day period, breaking the vault’s timelock mechanism’s invariant.

## Recommended Mitigation Steps

Add a check for the msg.sender and the inputted owner, and revert a transaction if they are not matched.

function withdraw(uint256 assets, address receiver, address owner) public override returns (uint256) { + if (msg.sender != owner) revert Errors.ERC4626InvalidOwner(); WithdrawStruct storage withdrawal = withdrawals[msg.sender]; uint256 amountProposed = withdrawal.amountProposed; uint256 timeProposed = withdrawal.timeProposed; if (timeProposed == 0 && amountProposed <= 1) revert Errors.ERC4626ProposeWithdrawFirst(); if (timeProposed + C.WITHDRAW_WAIT_TIME > uint40(block.timestamp)) revert Errors.ERC4626WaitLongerBeforeWithdrawing(); // @dev After 7 days from proposing, a user has 45 days to withdraw // @dev User will need to cancelWithdrawProposal() and proposeWithdraw() again if (timeProposed + C.WITHDRAW_WAIT_TIME + C.MAX_WITHDRAW_TIME <= uint40(block.timestamp)) {

revert Errors.ERC4626MaxWithdrawTimeHasElapsed(); } if (amountProposed > maxWithdraw(owner)) revert Errors.ERC4626WithdrawMoreThanMax(); checkDiscountWindow(); uint256 shares = previewWithdraw(amountProposed); IAsset _dusd = IAsset(dusd); uint256 oldBalance = _dusd.balanceOf(receiver); _withdraw(_msgSender(), receiver, owner, amountProposed, shares); uint256 newBalance = _dusd.balanceOf(receiver); // @dev Slippage is likely irrelevant for this. Merely for preventative purposes uint256 slippage = 0.01 ether; if (newBalance < slippage.mul(amountProposed) + oldBalance) revert Errors.ERC4626WithdrawSlippageExceeded(); delete withdrawal.timeProposed; //reset withdrawal (1 to keep slot warm) withdrawal.amountProposed = 1;

return shares; } ditto-eth (sponsor) confirmed hansfriese (judge) commented:

Nice finding!

Medium is appropriate as the withdrawal timelock can be bypassed.
