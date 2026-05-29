# Benchmark Ground Truth: DittoETH

## Accepted H/M Findings

# Accepted H/M Findings: DittoETH

# [H-01] A successfully disputed redemption proposal has still increased the redemption fee base rate; exploit to depeg dUSD

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by d3e4 Redemptions may not be incentivized to increase the value of dUSD. Furthermore, this may be deliberately induced to prevent a falling dUSD from repegging, ultimately causing dUSD to fully depeg and crash. The motivations for this attack could be anything from pure spite (perhaps from a competitor) Root cause and summary The most fundamental root cause of this issue and exploit is that the base rate in the redemption fee is increased when a redemption is proposed, but not restored (decreased) if the redemption is successfully disputed.

When dUSD is trading below the dollar, redemptions are needed to restore the peg. The redemption fee regulates the incentive to redeem dUSD. So if the redemption fee is increased, without contributing to the peg restoration by decreasing the supply of dUSD, the peg restoration mechanism is impeded and sufficient redemptions will not happen. A sufficiently high redemption fee will make any redemption a loss for the redeemer, which means that the peg will not be restored.

This issue becomes viable as a direct exploit because of multiple factors reducing its cost to the attacker:

The attacker can avoid paying the penalty by disputing his own proposal (using a different account).

The redemption fee contains an added base fee of 0.5% which reduces the required amount to propose, on which the fee is paid.

A. The total redemption fee decreases, down to almost a half, if the proposed amount is split over several redemption proposals.

B. By splitting the total proposed amount the attacker only needs to escrow dUSD for one such part (at a time), which minimizes his own loss on his devalued dUSD.

The redemption fee is proportional to the collateral that would be redeemed, which is capped to the short record’s collateral balance; whereas the base rate is increased based on the amount proposed, which is not reduced accordingly. This can only be leveraged if there exists an undercollateralized short record. But if there is, this can be leveraged to make this exploit essentially for free for about 48 hours.

With the possible exception of 1 above, all of the above are issues in their own right, especially 4; which is therefore, also reported separately. The impact of 2 and 3 is subject to the model used for how (quantitatively) redemptions restore the peg,; therefore, is discussed below rather than reported separately.

## Recommended Mitigation Steps

Make sure the base rate is unaffected by a proposal which is successfully disputed. It might be tricky to account for the decay if attempting to later subtract when disputing. A possible solution might be to simply calculate and apply the new rate only when a proposal is successfully claimed.

ditto-eth (DittoETH) confirmed

# [H-02] An attacker can cancel other people’s short orders

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by 00xSEV LibSRUtil.transferShortRecord does not check the owner of nft.shortOrderId before canceling it.

shortOrderId s are reused after the order is canceled.

An attacker can create a state for an NFT where the short record’s status is SR.PartialFill, but nft.shortOrderId is already reassigned to another user, yet still set in the attacker’s NFT. This will make the system think that it needs to cancel nft.shortOrderId, even though it belongs to another user already and does not reference the original order.

## Vulnerability Details

- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/libraries/LibSRUtil.sol#L134-L136
if ( short.

status == SR.

PartialFill ) { LibOrders.

cancelShort ( asset, nft.

shortOrderId ); } For the example, we will use ids that would be used by a new asset (starting ids, 2 for shortRecord and 100 for short order).

Steps for the attacker, the simplest attack (see test1

## Impact

The attacker has full control over which short orders appear on the order book; they can censor orders, manipulate the price by canceling orders with a low price.

It’s possible to reduce other’s rewards by canceling shorts just before the rewards are due (Token yield is provided only after some time on the order book, see here ).

The platform is unusable because short orders can be canceled at any time arbitrarily by an attacker, or several attackers/bad actors, who do not like some short orders.

## Recommended Mitigation Steps

Consider checking the owner of the short order in transferShortRecord before cancelling it.

Consider burning the nft when short record is deleted.

## Assessed type

Invalid Validation ditto-eth (DittoETH) confirmed

# [H-03] Users can mint DUSD with less collateral than required, which gives them free DUSD and may open a liquidatable position

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by nonseodion, also found by serial-coder Users can mint more DUSD than the value of collateral they provided for the Short Order when they cancel it.

Note: Each number below is a step and some steps jump to other steps.

When a user cancels a short order, the following happens:

If the short’s associated short record has a status of SR.Closed, i.e. it hasn’t been matched, it is deleted and the process moves to step 5.

LibOrders.sol#L898-L902 if ( shortRecord.

status == SR.

Closed ) { // @dev creating shortOrder automatically creates a closed shortRecord which also sets a shortRecordId // @dev cancelling an unmatched order needs to also handle/recycle the shortRecordId ❌ LibShortRecord.

deleteShortRecord ( asset, shorter, shortRecordId ); } else { Else, if the short record is partially filled or filled, it is checked if it has less than the minimum erc debt ( minShortErc ).

LibOrders.sol#L904 if ( shortRecord.

ercDebt < minShortErc ) { If it has less debt than minShortErc, the short record is filled up to the minShortErc and its status is changed to SR.FullyFilled and the process moves to step 5.

LibOrders.sol#L905C1-L923 // @dev prevents leaving behind a partially filled SR is under minShortErc // @dev if the corresponding short is cancelled, then the partially filled SR's debt will == minShortErc uint88 debtDiff = minShortErc - shortRecord.

ercDebt; { STypes.

Vault storage Vault = s.

vault [ vault ]; uint88 collateralDiff = shortOrder.

price.

mulU88 ( debtDiff ).

mulU88 ( LibOrders.

convertCR ( shortOrder.

shortOrderCR )); ❌ LibShortRecord.

fillShortRecord ( asset, shorter, shortRecordId, ❌ SR.

FullyFilled, collateralDiff, debtDiff, Asset.

ercDebtRate, Vault.

dethYieldRate ); Else, if it has more debt than minShortErc, the status of the short record is changed to SR.FullyFilled.

LibOrders.sol#L934 shortRecord.

status = SR.

FullyFilled; Finally the short order itself is canceled.

LibOrders.sol#L951 cancelOrder ( s.

shorts, asset, id ); The issue arises in step 3 where it tries to fill the short record up to the minShortErc. To fill the Short Record, it first gets the amount of DUSD needed in line 928 of the code snippet below as debtDiff. The collateral needed to mint the DUSD is calculated in line 932.

There are two issues with the calculation in line 932:

It uses the shortOrderCR to calculate the collateral needed. If the short order’s collateral ratio is less than 1 ether then the value of the collateral calculated is less than the value of DUSD that eventually gets minted.

It uses the short order’s price shortOrder.price to calculate the needed collateral. If this price is less than the current price of DUSD in ETH value, the collateral calculated is less than what is required. But if this price is higher than the current price, the user uses more ETH to mint the DUSD.

The short record is filled in line 934, and the collateral needed is removed from the ETH the user initially supplied when he created the short order in line 950. Note that the user (i.e. the shorter) gets the debtDiff (i.e. DUSD minted) in line 953.

LibOrders.sol#L907-L938 928:

uint88 debtDiff = minShortErc - shortRecord.

ercDebt; 929: { 930:

STypes.

Vault storage Vault = s.

vault [ vault ]; 931:

932:

uint88 collateralDiff = shortOrder.

price.

mulU88 ( debtDiff ).

mulU88 ( LibOrders.

convertCR ( shortOrder.

shortOrderCR )); 933:

934:

LibShortRecord.

fillShortRecord ( 935:

asset, 936:

shorter, 937:

shortRecordId, 938:

SR.

FullyFilled, 939:

collateralDiff, 940:

debtDiff, 941:

Asset.

ercDebtRate, 942:

Vault.

dethYieldRate 943: ); 944:

945:

Vault.

dethCollateral += collateralDiff; 946:

Asset.

dethCollateral += collateralDiff; 947:

Asset.

ercDebt += debtDiff; 948:

949:

// @dev update the eth refund amount 950:

eth -= collateralDiff; 951: } 952:

// @dev virtually mint the increased debt 953:

s.

assetUser [ asset ][ shorter ].

ercEscrowed += debtDiff; A malicious user can exploit this by following these steps:

Create a short order on an asset that lets the user provide less than 100% capital.

Ensure that the order only gets partially filled before it is added to the market.

Cancel the order to mint DUSD for only a part of the collateral and get the minted DUSD.

This will allow him to mint more DUSD than the value of the collateral he provided. The Short Record he leaves is also immediately liquidatable.

## Impact

The issues above has the following impacts:

Users can mint more DUSD than the collateral they provide.

Users can mint DUSD at a lesser price than the current ETH price.

A user can open a position that is immediately liquidatable if he does any of the two actions above.

Users can also mint DUSD at a higher price than the current ETH price letting them experience a loss.

## Recommended Mitigation Steps

Consider using the initialCR of the asset if the short order’s CR is lesser and consider using the current oracle price instead of the short order’s price when it was created.

It is also possible that the ETH calculated exceeds the ETH the user provided when he created the Short Order. The sponsor can also consider sourcing more ETH from the user’s escrowed ETH to enable him to cancel when this occurs.

LibOrders.sol#L911-L938 - uint88 collateralDiff = shortOrder.

price.

mulU88 ( debtDiff ).

mulU88 ( LibOrders.

convertCR ( shortOrder.

shortOrderCR )); + uint16 cr = shortOrder.

shortOrderCR < s.

asset [ asset ].

initialCR ?

s.

asset [ asset ].

initialCR:

shortOrder.

shortOrderCR; + uint80 price = LibOracle.

getSavedOrSpotOraclePrice ( asset ); + uint88 collateralDiff = price.

mulU88 ( debtDiff ).

mulU88 ( LibOrders.

convertCR ( cr )); LibShortRecord.

fillShortRecord ( asset, shorter, shortRecordId, SR.

FullyFilled, collateralDiff, debtDiff, Asset.

ercDebtRate, Vault.

dethYieldRate ); Vault.

dethCollateral += collateralDiff; Asset.

dethCollateral += collateralDiff; Asset.

ercDebt += debtDiff; // @dev update the eth refund amount + if ( eth < collateralDiff ) revert Errors.

InsufficientCollateral (); eth -= collateralDiff; ditto-eth (DittoETH) confirmed

# [H-04] Partially filled Short Records created without a short order cannot be liquidated and exited

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by nonseodion, also found by 0xbepresent When a Short Order is being created it tries to fill its Short Record. If it fills the Short Record, the Short Record is given a filled status ( SR.FullyFilled ) and the Short Order isn’t added to the market. But if it doesn’t fill the Short Record, it is given a partially filled status ( SR.PartiallyFilled ) and the remaining part of the Short Order is added to the market.

The issue is the current implementation doesn’t add the Short Order to the market every time the Short Record is partially filled. It does this in the sellMatchAlgo() function loop when it tries to match bids.

LibOrders.sol#L591-L597 ❌ matchIncomingSell ( asset, incomingAsk, matchTotal ); ❌ if ( incomingAsk.

ercAmount.

mul ( incomingAsk.

price ) >= minAskEth ) { addSellOrder ( incomingAsk, asset, orderHintArray ); } s.

bids [ asset ][ C.

HEAD ].

nextId = C.

TAIL; return; When the Short Order is being matched in the sellMatchAlgo() loop, it encounters the check in the if statement above. If the value of the erc remaining in the short is less than minAskEth it is not added to the market. The Short Record is already given the SR.PartiallyFilled status before the check.

When this happens, the Short Record is created with no associated Short Order. This prevents the user from exiting the Short Record and a liquidator from liquidating the position if it ever becomes liquidatable. These actions revert with InvalidShortOrder() error in the following portion of the code.

Exiting When a user tries to exit using any of the exit functions, he has to pass a Short Order id.

ExitShortFacet.sol#L41 function exitShortWallet ( address asset, uint8 id, uint88 buybackAmount, ❌ uint16 shortOrderId ) ExitShortFacet.sol#L87 function exitShortErcEscrowed ( address asset, uint8 id, uint88 buybackAmount, ❌ uint16 shortOrderId ) ExitShortFacet.sol#L142 function exitShort ( address asset, uint8 id, uint88 buybackAmount, uint80 price, uint16 [] memory shortHintArray, ❌ uint16 shortOrderId ) Since there is no valid Short Order Id, if he passes any value it reverts when the user tries to exit. Because the id needs to be associated with the shortRecord and still be owned by him to pass the checks.

exitShort() function calls checkCancelShortOrder() which will revert in the check below.

LibSRUtil.sol#L57 if ( shortOrder.

shortRecordId != shortRecordId || shortOrder.

addr != shorter ) revert Errors.

InvalidShortOrder (); For exitShortWallet() and exitShortErcEscrowed(), they revert in the check below when they call checkShortMinErc().

LibSRUtil.sol#L84 if ( shortOrder.

shortRecordId != shortRecordId || shortOrder.

addr != shorter ) revert Errors.

InvalidShortOrder (); Liquidation The primary and secondary liquidation calls require a Short Order Id.

Primary Liquidation call:

PrimaryLiquidationFacet.sol#L47 function liquidate ( address asset, address shorter, uint8 id, uint16 [] memory shortHintArray, ❌ uint16 shortOrderId ) Secondary Liquidation call:

SecondaryLiquidationFacet.sol#L39 function liquidateSecondary ( address asset, ❌ MTypes.BatchLiquidation[] memory batches, uint88 liquidateAmount, bool isWallet ) BatchLiquidation struct:

struct BatchLiquidation { address shorter; uint8 shortId; ❌ uint16 shortOrderId; } The liquidate() function reverts in its call to checkCancelShortOrder(). The check below causes the revert, because the id passed by the liquidator needs to be associated with the Short Record and still be owned by the user being liquidated to pass the check.

LibSRUtil.sol#L57 if (shortOrder.shortRecordId != shortRecordId || shortOrder.addr != shorter) revert Errors.InvalidShortOrder(); The liquidateSecondary() function uses a loop to complete batch liquidation. In the loop, it first does the check below on each batch element.

SecondaryLiquidationFacet.sol#L69-L80 bool shortUnderMin; if ( m.

isPartialFill ) { // Check attached shortOrder ercAmount left since SR will be fully liquidated STypes.

Order storage shortOrder = s.

shorts [ m.

asset ][ m.

shortOrderId ]; ❌ shortUnderMin = shortOrder.

ercAmount < minShortErc; ❌ if ( shortUnderMin ) { // Skip instead of reverting for invalid shortOrder ❌ if ( shortOrder.

shortRecordId != m.

short.

id || shortOrder.

addr != m.

shorter ) { continue; } The loop skips liquidating if the Short Record’s debt is below the minimum i.e.

shortUnderMin is true for the passed shortOrder and shortOrder.shortRecordId != m.short.id || shortOrder.addr != m.shorter evaluates to true since the short order isn’t attached to the Short Record.

It reverts in the check below.

liquidateAmount is the amount the liquidator wants to liquidate and liquidateAmountLeft is the amount not liquidated. If only the bad Short Record is in the batch, it reverts. If other Short Records in the batch get liquidated it doesn’t revert.

SecondaryLiquidationFacet.sol#L124 if ( liquidateAmount == liquidateAmountLeft ) revert Errors.

SecondaryLiquidationNoValidShorts (); Note: Secondary Liquidation can still be done in some scenarios check the POC section for more details.

Apart from the DOS effects above, the issue also lets a user create a Short Record with an erc amount below the minShortErc

## Impact

The issue above has the following effects:

Users cannot exit a Short Record Position.

Primary Liquidation cannot be done on the Short Record.

Secondary Liquidation may not be possible on the Short Record.

Allows a user to create a Short Record below minShortErc.

## Recommended Mitigation Steps

Consider setting ercAmount of the incomingAsk to zero in the sellMatchAlgo() function. This will allow the matchIncomingSell() call to set the Short Record to a Fully Filled state.

LibOrders.sol#L590-L598 if ( startingId == C.

TAIL ) { - matchIncomingSell ( asset, incomingAsk, matchTotal ); if ( incomingAsk.

ercAmount.

mul ( incomingAsk.

price ) >= minAskEth ) { addSellOrder ( incomingAsk, asset, orderHintArray ); } + incomingAsk.

ercAmount = 0; + matchIncomingSell ( asset, incomingAsk, matchTotal ); s.

bids [ asset ][ C.

HEAD ].

nextId = C.

TAIL; return; }

## Assessed type

DoS ditto-eth (DittoETH) confirmed and commented:

# [H-05] Flawed if check causes inaccurate tracking of the protocol’s ercDebt and collateral

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

ercDebt and collateral Submitted by samuraii77, also found by samuraii77 and serial-coder A flawed if check using && instead of || in RedemptionFacet::claimRemainingCollateral() leads to a break of one of the core protocol invariants. The total collateral and ercDebt of an asset should always equal the total collateral and ercDebt of all shortRecords combined. However, this will not be the case if the scenario explained below takes place. This results in the protocol holding inaccurate values of their ercDebt and collateral which are extremely important values used for very important calculations across the entire protocol.

## Recommended Mitigation Steps

Use || instead of && + if (claimProposal.shorter != msg.sender || claimProposal.shortId != id) revert Errors.CanOnlyClaimYourShort(); - if (claimProposal.shorter != msg.sender && claimProposal.shortId != id) revert Errors.CanOnlyClaimYourShort();

## Assessed type

Invalid Validation ditto-eth (DittoETH) confirmed

# [H-06] Closing a SR during a wrong redemption proposal leads to loss of funds

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by Cosine, also found by klau5

- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/facets/RedemptionFacet.sol#L267-L268
- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/libraries/AppStorage.sol#L92

## Impact

When a user creates a redemption proposal with the proposeRedemption function the user has to provide a list of the short records (SRs) with the lowest collateral ratios (CR) in the system ascending.

To prevent users from creating proposals with a wrong SR list, anyone is allowed to dispute proposals with the disputeRedemption function. This function allows the disputer to prove that a SR with a lower CR was not included in the proposal and for doing so the disputer receives a penalty fee from the proposer.

If between these flows of creating a wrong proposal and disputing it a SR is closed (liquidation, exiting, transfer, …) the collateral is added to the closed SR and can not be recovered.

## Recommended Mitigation Steps

Opening up the SR again if it’s closed would be a solution, but it could probably be misused to avoid liquidations. Therefore, carefully think about the implications of changes in this context.

## Assessed type

Context ditto-eth (DittoETH) confirmed

# [H-07] Valid redemption proposals can be disputed by decreasing collateral

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by Cosine, also found by ilchovski and klau5

- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/facets/RedemptionFacet.sol#L259
- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/facets/ShortRecordFacet.sol#L81-L104

## Impact

When a user creates a redemption proposal with the proposeRedemption function the user has to provide a list of the short records (SRs) with the lowest collateral ratios (CR) in the system ascending.

To prevent users from creating proposals with a wrong SR list, anyone is allowed to dispute proposals with the disputeRedemption function. This function allows the disputer to prove that a SR with a lower CR was not included in the proposal and for doing so the disputer receives a penalty fee from the proposer. Therefore, if an attacker can dispute a valid redemption proposal, the attacker can steal funds from a proposer.

To avoid malicious disputers the system invented a DISPUTE_REDEMPTION_BUFFER that should prevent users from disputing with a SR that was created/modified <= 1 hour before the redemption proposal was created:

if ( disputeCR < incorrectProposal.

CR && disputeSR.

updatedAt + C.

DISPUTE_REDEMPTION_BUFFER <= redeemerAssetUser.

timeProposed ) But not every function that modifies a SR updates the updatedAt param. This enables the possibility for an attacker to dispute a valid redemption proposal by modifying a SR after the proposal so that the proposer does not have the chance to create a correct proposal.

The decreaseCollateral function does not update the updatedAt param and therefore, the following attack path is enabled:

initialCR of the given asset is set to 1.7 (as in the docs) and the max redemption CR is 2 (constant).

User creates a valid redemption proposal where the SRs have a CR above the initialCR.

The attacker owns a SR with a CR above the ones in the proposal.

The attacker decreases the CR of the own SR to the initialCR, disputes the redemption to receive the penalty fee, and increases the CR back up in one transaction.

## Recommended Mitigation Steps

Update the updatedAt param when decreasing collateral, or do now allow redemption proposals of SRs above the initialCR (as decreasing below that is not possible).

## Assessed type

Context ditto-eth (DittoETH) confirmed Medium Risk Findings (9)

# [M-01] The shortOrder verification bug on the RedemptionFacet::proposeRedemption() allows an attacker to leave a small shortOrder on the order book, leading to the protocol’s bad debt

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

shortOrder verification bug on the RedemptionFacet::proposeRedemption() allows an attacker to leave a small shortOrder on the order book, leading to the protocol’s bad debt Submitted by serial-coder, also found by unix515 The BidOrdersFacet::bidMatchAlgo() allows a shortOrder to be partially matched and leave its ercAmount * price < minAskEth due to the DUST_FACTOR constant (as long as its corresponding shortRecord is maintaining enough ercDebt + shortOrder ’s ercAmount to keep the position >= the minShortErc threshold).

The redemption process enables redeemers to redeem their ercEscrowed for ethCollateral on target shortRecords. If a shortRecord was partially filled, the RedemptionFacet::proposeRedemption() must guarantee that the corresponding shortOrder maintains the ercAmount >= minShortErc. In other words, if the shortOrder ’s ercAmount is less than the minShortErc threshold, the shortOrder must be canceled from the order book. Otherwise, the shortRecord position will be less than the minShortErc threshold when the order is matched again.

Subsequently, the small shortRecord ( short position) will not incentivize liquidators to liquidate it even if it is liquidable, leaving bad debt to the protocol.

I discovered that the proposeRedemption() improperly verifies the proposer (redeemer)‘s inputted shortOrderId param, allowing an attacker to specify the shortOrderId param to another shortOrder ’s id that does not correspond to the target redeeming shortRecord.

The vulnerability can bypass the minShortErc threshold verification process on the shortOrder corresponding to the processing shortRecord, eventually allowing an attacker to leave a small shortRecord position that disincentivizes liquidators from liquidating the position. Furthermore, the small shortRecord also disables the redemption mechanism from redeeming it for ethCollateral.

## Recommended Mitigation Steps

To fix the vulnerability, move out the shortOrder verification check and execute it immediately after loading the shortOrder from storage.

function proposeRedemption( address asset, MTypes.ProposalInput[] calldata proposalInput, uint88 redemptionAmount, uint88 maxRedemptionFee ) external isNotFrozen(asset) nonReentrant {...

for (uint8 i = 0; i < proposalInput.length; i++) { p.shorter = proposalInput[i].shorter; p.shortId = proposalInput[i].shortId; p.shortOrderId = proposalInput[i].shortOrderId; STypes.ShortRecord storage currentSR = s.shortRecords[p.asset][p.shorter][p.shortId];...

STypes.Order storage shortOrder = s.shorts[asset][p.shortOrderId]; + if (shortOrder.shortRecordId != p.shortId || shortOrder.addr != p.shorter) revert Errors.InvalidShortOrder(); if (currentSR.status == SR.PartialFill && shortOrder.ercAmount < minShortErc) { - if (shortOrder.shortRecordId != p.shortId || shortOrder.addr != p.shorter) revert Errors.InvalidShortOrder(); LibOrders.cancelShort(asset, p.shortOrderId); }...

}...

}

## Assessed type

Invalid Validation raymondfam (lookout) commented:

Inadequate/unstructured proof to support the intended code refactoring.

serial-coder (warden) commented:

Please have a second look at the following from the

# [M-02] Can manipulate the C.SHORT_STARTING_ID ShortRecord of the TAPP

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

C.SHORT_STARTING_ID ShortRecord of the TAPP Submitted by klau5

- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/facets/PrimaryLiquidationFacet.sol#L244-L247
- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/libraries/LibSRUtil.sol#L124

## Impact

Attackers can make it so that risky debts are not liquidated, and unliquidated risky debts can accumulate over the long term.

## Recommended Mitigation Steps

Prevents ShortRecord NFT from being sent to TAPPs.

ditto-eth (DittoETH) confirmed and commented:

Great find, solution seems to work too.

# [M-03] The colRedeemed variable is wrongly retrieved in LibBytes::readProposalData function

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

colRedeemed variable is wrongly retrieved in LibBytes::readProposalData function Submitted by Bube, also found by maxim371 and Rhaydden The LibBytes::readProposalData function uses inline assembly for efficient data extraction from a byte array. The colRedeemed variable, which represents an 11-byte value within the ProposalData structure, is intended to be extracted by applying a mask to isolate the relevant bytes. However, the current implementation incorrectly uses the add operation. That leads to retrieve incorrect value of colRedeemed variable:

function readProposalData ( address SSTORE2Pointer, uint8 slateLength ) internal view returns (MTypes.ProposalData[] memory ) { bytes memory slate = SSTORE2.

read ( SSTORE2Pointer ); // ProposalData is 51 bytes require ( slate.

length % 51 == 0, "Invalid data length" ); MTypes.

ProposalData [] memory data = new MTypes.

ProposalData []( slateLength ); for ( uint256 i = 0; i < slateLength; i ++) { // 32 offset for array length, multiply by each ProposalData uint256 offset = i * 51 + 32; address shorter; // bytes20 uint8 shortId; // bytes1 uint64 CR; // bytes8 uint88 ercDebtRedeemed; // bytes11 uint88 colRedeemed; // bytes11 assembly { // mload works 32 bytes at a time let fullWord:= mload ( add ( slate, offset )) // read 20 bytes shorter:= shr ( 96, fullWord ) // 0x60 = 96 (256-160) // read 8 bytes shortId:= and ( 0xff, shr ( 88, fullWord )) // 0x58 = 88 (96-8), mask of bytes1 = 0xff * 1 // read 64 bytes CR:= and ( 0xffffffffffffffff, shr ( 24, fullWord )) // 0x18 = 24 (88-64), mask of bytes8 = 0xff * 8

fullWord:= mload ( add ( slate, add ( offset, 29 ))) // (29 offset) // read 88 bytes ercDebtRedeemed:= shr ( 168, fullWord ) // (256-88 = 168) // read 88 bytes @> colRedeemed:= add ( 0xffffffffffffffffffffff, shr ( 80, fullWord )) // (256-88-88 = 80), mask of bytes11 = 0xff * 11 } data [ i ] = MTypes.

ProposalData ({ shorter:

shorter, shortId:

shortId, CR:

CR, ercDebtRedeemed:

ercDebtRedeemed, colRedeemed:

colRedeemed }); } return data; } The add operation would incorrectly add the mask to the shifted value, potentially resulting in an incorrect value for colRedeemed. The correct operation should use and to apply the mask and isolate the 11-byte colRedeemed value.

The RedemptionFacet contract calls the LibBytes::readProposalData function and uses colRedeemed variable in claimRedemption function.

## Recommended Mitigation Steps

Replace the add operation with an and operation to correctly apply the mask:

colRedeemed:= and(0xffffffffffffffffffffff, shr(80, fullWord)).

ditto-eth (DittoETH) confirmed

# [M-04] transferShortRecord : Can transfer a newly created ShortRecord using a previously minted NFT

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

transferShortRecord: Can transfer a newly created ShortRecord using a previously minted NFT Submitted by klau5 Can move the newly created ShortRecord using the NFT that was minted in the past.

## Recommended Mitigation Steps

function transferShortRecord(address from, address to, uint40 tokenId) internal { AppStorage storage s = appStorage(); STypes.NFT storage nft = s.nftMapping[tokenId]; address asset = s.assetMapping[nft.assetId]; STypes.ShortRecord storage short = s.shortRecords[asset][from][nft.shortRecordId]; if (short.status == SR.Closed) revert Errors.OriginalShortRecordCancelled(); if (short.ercDebt == 0) revert Errors.OriginalShortRecordRedeemed(); + if (short.tokenId != tokenId) revert Errors.NotValidNFT(); // @dev shortOrderId is already validated in mintNFT if (short.status == SR.PartialFill) { LibOrders.cancelShort(asset, nft.shortOrderId); } short.tokenId = 0; LibShortRecord.deleteShortRecord(asset, from, nft.shortRecordId);

LibBridgeRouter.transferBridgeCredit(asset, from, to, short.collateral); uint8 id = LibShortRecord.createShortRecord( asset, to, SR.FullyFilled, short.collateral, short.ercDebt, short.ercDebtRate, short.dethYieldRate, tokenId ); nft.owner = to; nft.shortRecordId = id; nft.shortOrderId = 0; }

## Assessed type

Invalid Validation ditto-eth (DittoETH) confirmed, but disagreed with severity and commented:

This scenario is technically possible, but only happens because of user error. I recommend low because a series of mistakes on the user’s part would have to occur for this scenario:

User gives approval to an attacker.

User closes their SR before the SR is transferred. Why would the user give approval to another address to transfer their SR but then not wait for the transfer to happen?

And if for some reason the user does not want to transfer anymore they can revoke approval.

klau5 (warden) commented:

User gives approval to an attacker.

User closes their SR before the SR is transferred. Why would the user give approval to another address to transfer their SR but then not wait for the transfer to happen?

This can be problematic not only in attacks, but also in general situations. For example, a user can approve to NFT marketplace contract in order to sell their NFT. A scenario is possible where a user approves an NFT for sale -> the SR is closed -> a new SR is created with the same ID -> and the NFT is sold.

A user will think that it doesn’t matter if the old NFT is sold because the closed SR will move. A user doesn’t realize that the new SR will move when the old NFT is sold.

And if for some reason the user does not want to transfer anymore they can revoke approval.

NFT should be minted independently per SR and should not affect each other. The current implementation implicitly points to multiple SRs with a single NFT. In a strict implementation, the past NFT should point to a closed SR, but due to optimizations, it doesn’t.

Revoking the approval of past SR NFTs to prevent the current SR from moving is not the right solution because it’s violate the individuality of NFT and SR, and user would not think that they should revoke past approval to prevent to moving new SR.

hansfriese (judge) decreased severity to Medium and commented:

Thanks for your detailed comments. I agree that managing a new SR using prior approval is not a recommended approach. I think Medium is more appropriate for the external requirements.

# [M-05] oracleCircuitBreaker : Not checking if price information of asset is stale

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

oracleCircuitBreaker: Not checking if price information of asset is stale Submitted by klau5, also found by 0xSecuri, Infect3d, Bauchibred, falconhoof, serial-coder, nonseodion, and Bigsam

- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/libraries/LibOracle.sol#L125-L126
- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/libraries/LibOracle.sol#L60

## Recommended Mitigation Steps

Set the chainlinkStaleLimit for each asset and check if the price information is not outdated.

function getOraclePrice(address asset) internal view returns (uint256) { AppStorage storage s = appStorage(); AggregatorV3Interface baseOracle = AggregatorV3Interface(s.baseOracle); uint256 protocolPrice = getPrice(asset); AggregatorV3Interface oracle = AggregatorV3Interface(s.asset[asset].oracle); if (address(oracle) == address(0)) revert Errors.InvalidAsset(); try baseOracle.latestRoundData() returns (uint80 baseRoundID, int256 basePrice, uint256, uint256 baseTimeStamp, uint80) {...

} catch { if (oracle == baseOracle) { return twapCircuitBreaker(); } else { // prettier-ignore ( uint80 roundID, int256 price, /*uint256 startedAt*/, uint256 timeStamp, /*uint80 answeredInRound*/ ) = oracle.latestRoundData(); - if (roundID == 0 || price == 0 || timeStamp > block.timestamp) revert Errors.InvalidPrice(); + if (roundID == 0 || price == 0 || timeStamp > block.timestamp || block.timestamp > chainlinkStaleLimit[asset] + timeStamp) revert Errors.InvalidPrice(); uint256 twapInv = twapCircuitBreaker(); uint256 priceInEth = uint256(price * C.BASE_ORACLE_DECIMALS).mul(twapInv); return priceInEth; } function oracleCircuitBreaker( uint80 roundId, uint80 baseRoundId, int256 chainlinkPrice,

int256 baseChainlinkPrice, uint256 timeStamp, - uint256 baseTimeStamp + uint256 baseTimeStamp, + address asset ) private view { bool invalidFetchData = roundId == 0 || timeStamp == 0 || timeStamp > block.timestamp || chainlinkPrice <= 0 - || baseRoundId == 0 || baseTimeStamp == 0 || baseTimeStamp > block.timestamp || baseChainlinkPrice <= 0; + || baseRoundId == 0 || baseTimeStamp == 0 || baseTimeStamp > block.timestamp || baseChainlinkPrice <= 0 || block.timestamp > chainlinkStaleLimit[asset] + timeStamp; if (invalidFetchData) revert Errors.InvalidPrice(); }

## Assessed type

Oracle ditto-eth (DittoETH) confirmed and commented via duplicate Issue #252:

Agree, this seems like a valid issue. Returned base oracle price could be stale.

hansfriese (judge) commented:

I will maintain this as a primary issue since it encompasses both concerns. Additionally, I will allocate partial credit to other findings explaining only one case.

FYI, it should check if price <= 0 instead of price == 0 in the catch block.

# [M-06] ShortOrders can be created with ercAmount == minAskEth/2 , increasing the gas costs for matching large orders and disincentivizing liquidators from liquidating them

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

ShortOrders can be created with ercAmount == minAskEth/2, increasing the gas costs for matching large orders and disincentivizing liquidators from liquidating them Submitted by nonseodion, also found by 0xbepresent A user can create a ShortOrder with ercAmount == minAskEth/2 by matching the ShortOrder with a bid that fills it up to minAskEth/2 and exiting the ShortRecord leaving only minAskEth/2 in the ShortOrder with an empty ShortRecord.

Description This issue makes use of two properties in the codebase.

A ShortOrder can be matched such that only minAskEth/2 remains in the ShortOrder.

A ShortOrder on the market can be matched by an incoming bid. This match is done in the call to bidMatchAlgo() function. Which tries to fill the bid with asks or shorts in the market.

BidOrdersFacet.sol#L106-L111 if ( incomingBid.

price >= lowestSell.

price && ( lowestSell.

orderType == O.

LimitAsk || lowestSell.

orderType == O.

LimitShort )) { // @dev if match and match price is gt.5% to saved oracle in either direction, update startingShortId LibOrders.

updateOracleAndStartingShortViaThreshold ( asset, LibOracle.

getPrice ( asset ), incomingBid, shortHintArray ); b.

shortHintId = b.

shortId = Asset.

startingShortId; b.

oraclePrice = LibOracle.

getPrice ( asset ); ❌ return bidMatchAlgo ( asset, incomingBid, orderHintArray, b ); In the call to bidMatchAlgo(), it goes through a loop and on each iteration, it first tries to match lowestSell in line 155 below.

lowestSell is the current lowest bid or ask. After matching it compares the ercAmount in the bid and lowest sell.

If the ercAmount in the lowest sell exceeds that in the bid, the bid is filled and executes the else statement in line 179 below. If the amount left in the lowest sell is >= LibAsset.minAskEth(asset).mul(C.DUST_FACTOR), b.dustShortId and b.dustAskId are set to zero in line 191 below. This ensures the lowest sell isn’t deleted in the call to matchIncomingBid() function.

BidOrdersFacet.sol#L155-L197 155:

matchlowestSell ( asset, lowestSell, incomingBid, matchTotal ); 156:

if ( incomingBid.

ercAmount > lowestSell.

ercAmount ) { 157:

incomingBid.

ercAmount -= lowestSell.

ercAmount; 158:

lowestSell.

ercAmount = 0; 159:

if ( lowestSell.

isShort ()) { 160:

b.

matchedShortId = lowestSell.

id; 161:

b.

prevShortId = lowestSell.

prevId; 162:

LibOrders.

matchOrder ( s.

shorts, asset, lowestSell.

id ); 163:

_shortDirectionHandler ( asset, lowestSell, incomingBid, b ); 164: } else { 165:

b.

matchedAskId = lowestSell.

id; 166:

LibOrders.

matchOrder ( s.

asks, asset, lowestSell.

id ); 167:

b.

askId = lowestSell.

nextId; 168: } 169: } else { 170:

if ( incomingBid.

ercAmount == lowestSell.

ercAmount ) { 171:

if ( lowestSell.

isShort ()) { 172:

b.

matchedShortId = lowestSell.

id; 173:

b.

prevShortId = lowestSell.

prevId; 174:

LibOrders.

matchOrder ( s.

shorts, asset, lowestSell.

id ); 175: } else { 176:

b.

matchedAskId = lowestSell.

id; 177:

LibOrders.

matchOrder ( s.

asks, asset, lowestSell.

id ); 178: } 179: } else { 180:

lowestSell.

ercAmount -= incomingBid.

ercAmount; 181:

if ( lowestSell.

isShort ()) { 182:

b.

dustShortId = lowestSell.

id; 183:

STypes.

Order storage lowestShort = s.

shorts [ asset ][ lowestSell.

id ]; 184:

lowestShort.

ercAmount = lowestSell.

ercAmount; 185: } else { 186:

b.

dustAskId = lowestSell.

id; 187:

s.

asks [ asset ][ lowestSell.

id ].

ercAmount = lowestSell.

ercAmount; 188: } 189:

// Check reduced dust threshold for existing limit orders 190:

if ( lowestSell.

ercAmount.

mul ( lowestSell.

price ) >= LibAsset.

minAskEth ( asset ).

mul ( C.

DUST_FACTOR )) { 191:

b.

dustShortId = b.

dustAskId = 0; 192: } 193: } 194:

incomingBid.

ercAmount = 0; 195:

return matchIncomingBid ( asset, incomingBid, matchTotal, b ); 196: } 197: } else { If the amount in lowestSell is less than LibAsset.minAskEth(asset).mul(C.DUST_FACTOR), matchIncomingBid() deletes in the code section below.

BidOrdersFacet.sol#L292-L296 if ( b.

dustAskId > 0 ) { IDiamond ( payable ( address ( this ))).

_cancelAsk ( asset, b.

dustAskId ); } else if ( b.

dustShortId > 0 ) { IDiamond ( payable ( address ( this ))).

_cancelShort ( asset, b.

dustShortId ); } LibAsset.minAskEth(asset).mul(C.DUST_FACTOR) translates to minAskEth/2 because C.DUST_FACTOR is a constant and is 0.5 ether. So if a ShortOrder has minAskEth/2 it won’t be deleted.

A ShortRecord can be exited with the id of a cancelled ShortOrder that still points to it.

When a call is made to the exitShortWallet() or exitShortErcEscrowed() function, shortOrderId is passed which is expected to be the order id of the ShortRecord currently being exited.

exitShortWallet() function calls checkShortMinErc() and passes the shortOrderId.

ExitShortFacet.sol#L67-L73 LibSRUtil.

checkShortMinErc ({ asset:

asset, initialStatus:

initialStatus, shortOrderId:

shortOrderId, shortRecordId:

id, shorter:

msg.

sender }); In the call to checkShortMinErc(), the shortOrderId is verified in line 94 below by checking if the shortOrder currently points to ShortRecord or if its address points to the shorter, i.e. the owner of the ShortRecord.

LibSRUtil.sol#L81-L99 091:

if ( initialStatus == SR.

PartialFill ) { 092:

// Verify shortOrder 093:

STypes.

Order storage shortOrder = s.

shorts [ asset ][ shortOrderId ]; 094:

if ( shortOrder.

shortRecordId != shortRecordId || shortOrder.

addr != shorter ) revert Errors.

InvalidShortOrder (); 095:

096:

if ( shortRecord.

status == SR.

Closed ) { 097:

// Check remaining shortOrder 098:

if ( shortOrder.

ercAmount < minShortErc ) { 099:

// @dev The resulting SR will not have PartialFill status after cancel 100:

LibOrders.

cancelShort ( asset, shortOrderId ); 101:

isCancelled = true; 102: } 103: } else { 104:

// Check remaining shortOrder and remaining shortRecord 105:

if ( shortOrder.

ercAmount + shortRecord.

ercDebt < minShortErc ) revert Errors.

CannotLeaveDustAmount (); 106: } 107: } else if ( shortRecord.

status != SR.

Closed && shortRecord.

ercDebt < minShortErc ) { 108:

revert Errors.

CannotLeaveDustAmount (); 109: } Note: it doesn’t check if the ShortOrder is cancelled. This means we can use a cancelled ShortOrder that points to the ShortRecord and is owned by the owner of the ShortRecord being exited.

A malicious user can use these two properties of the codebase to create a ShortOrder that has minAskEth/2 as ercAmount and 0 debt in its ShortRecord by following these setps:

Create 1 shortOrder with address A and another with address B. They shouldn’t get matched.

Cancel address A’s shortOrder than cancel address B’s. This order is strict and is to ensure Address B’s ShortOrder is reused before address A’s.

Create another ShortOrder with address A of 3000 DUSD that doesn’t get matched. This short will reuse the id of the ShortRecord associated with its first ShortOrder but will reuse address B’s ShortOrder as its ShortOrder. This leaves address A’s former ShortOrder cancelled but still pointing to its ShortRecord.

Create a Bid of 3000 - minAskEth/2 DUSD to match the ShortOrder and leave minAskEth/2 in the ShortOrder. This also lets the ShortOrder have a PartiallyFilled status to pass the check on line 91 above.

Exit the ShortOrder with 3000 - minAskEth/2 DUSD as buybackAmount and the former id of address A’s cancelled ShortOrder. The buybackAmount is the amount of debt (DUSD) to pay back. So we’ll be paying everything back.

After paying back we’ll have an empty ShortRecord and the ShortOrder will have an ercAmount of minAskEth/2.

If the minAskEth is small, we’ll end up creating small ShortOrder s in the market.

ShortOrders like this may make a transaction trying to fill a large order run out of gas and may disincentivize liquidators from liquidating them if they become liquidatable.

## Impact

The issue above has the following effects:

Large orders may run out of gas if a malicious user puts many orders with minAskEth/2 erc amounts on the market.

Liquidators are disincentivized from liquidating them because of their small amounts.

## Recommended Mitigation Steps

Consider checking if the ShortOrder being validated in checkShortMinErc() is cancelled.

LibSRUtil.sol#L84 - if ( shortOrder.

shortRecordId != shortRecordId || shortOrder.

addr != shorter ) revert Errors.

InvalidShortOrder (); + if ( shortOrder.

shortRecordId != shortRecordId || shortOrder.

addr != shorter || shortOrder.

orderType == O.

Cancelled ) revert Errors.

InvalidShortOrder ();

## Assessed type

DoS ditto-eth (DittoETH) confirmed and commented:

I think the new check should be shortOrder.orderType != O.LimitShort, but otherwise great find.

hansfriese (judge) decreased severity to Medium and commented:

Medium is more appropriate as there is no direct fund loss.

# [M-07] Using cached price to create a proposal reduce the efficacity of redemptions for asset peg

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by Infect3d, also found by klau5, XDZIBECX, falconhoof, Evo, ilchovski, LinKenji, foxb868, and nonseodion One of the conditions for updating oracle prices in Ditto is when an action related to shorts is executed. This is important because, in the event of high volatility, shorts must be closed out before bad debt occurs in the protocol. While liquidations does update the oracle before processing the short, this is not the case for redemptions.

Redemptions, as liquidations, play a central role in Ditto. By allowing users to redeem shorts with poor collateralization for a 1:1 exchange rate of the asset, Ditto is able to maintain a stable peg for its pegged asset. For this reason, it is important to reduce the pricing delay for the redeems, as much as for the liquidations.

File:

contracts \ facets \ RedemptionFacet.

sol 56:

function proposeRedemption ( 57:

address asset, 58:

MTypes.

ProposalInput [] calldata proposalInput, 59:

uint88 redemptionAmount, 60:

uint88 maxRedemptionFee 61: ) external isNotFrozen ( asset ) nonReentrant {...:

//...

74:

75:⚠ p.

oraclePrice = LibOracle.

getPrice ( p.

asset ); //<@ getting cached price 76:

77:

bytes memory slate; 78:

for ( uint8 i = 0; i < proposalInput.

length; i ++) { 79:

p.

shorter = proposalInput [ i ].

shorter; 80:

p.

shortId = proposalInput [ i ].

shortId; 81:

p.

shortOrderId = proposalInput [ i ].

shortOrderId; 82:

// @dev Setting this above _onlyValidShortRecord to allow skipping 83:

STypes.

ShortRecord storage currentSR = s.

shortRecords [ p.

asset ][ p.

shorter ][ p.

shortId ]; 84:

85:

/// Evaluate proposed shortRecord 86:

87:

if (!

validRedemptionSR ( currentSR, msg.

sender, p.

shorter, minShortErc )) continue; 88:

89:

currentSR.

updateErcDebt ( p.

asset ); 90:⚠ p.

currentCR = currentSR.

getCollateralRatio ( p.

oraclePrice ); //<@ Collateral ratio calculated using cached price 91:

92:

// @dev Skip if proposal is not sorted correctly or if above redemption threshold 93:❌ if ( p.

previousCR > p.

currentCR || p.

currentCR >= C.

MAX_REDEMPTION_CR ) continue; //<@

## Impact

If the cached price is not reflective of the current market price, the protocol may either overvalue or undervalue the collateral backing shorts. The usage of cached prices in the proposeRedemption function, as opposed to real-time or recently updated prices will affect the effectiveness of the redemption process in maintaining the asset’s peg in periods of high volatility.

## Recommended Mitigation Steps

Do not use cached price for redemptions, but rather an updated price through LibOracle::getSavedOrSpotOraclePrice, for example.

## Assessed type

Oracle raymondfam (lookout) commented:

Oracle price is cached to 15m, mostly done to allow the hint system to work. Will let sponsor to assess the severity of the issue.

ditto-eth (DittoETH) confirmed

# [M-08] If a redemption has N disputable shorts, it is possible to dispute N-1 times the redemption to maximize the penalty

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

N disputable shorts, it is possible to dispute N-1 times the redemption to maximize the penalty Submitted by Infect3d Ditto is a decentralized CDP protocol allowing users to get stablecoins (called dUSD here) against LST (Liquid Staking Tokens like stETH or rETH) collateral. Depositing LST grant dETH, a shares representing the value deposited in ETH. Users can then open different kind of positions: bids, asks or shorts, each one having its own orderbook.

Ditto implemented a mechanism called Redemption, idea borrowed from Liquity. Redemption allows anyone to redeem dUSD for a value of exactly 1 USD by calling out shorts with very bad CR (collateral ratio) and closing them. This helps the protocol to keep a healthy global market CR, shorters to not be liquidated, and dUSD holders to redeem dUSD with no loss in case of dUSD trading below 1 USD.

Users can propose multiple shorts to be redeemed at once if they meet certain conditions ( see Constraints in doc ). The shorts must be sorted from lowest CR to highest CR, and all CRs must be below a CR of 2. If a proposal do not properly follow these rules, anyone can dispute the proposal by showing a proof-short: all proposal shorts with a CR higher than the proof become invalid.

For each invalid short, a penalty is applied to the redeemer (and awarded to the disputer) The penalty calculation is based on the CR difference between the disputeCR (proof) and the the currentProposal.CR (lowest invalid CR):

Note: please see scenario in warden’s original submission.

The issue is located in this part of the mechanism and is pretty easy to understand. Let’s imagine 4 shorts in a list of shorts: … < CR1 < CR2 < CR3 < CR4 < … CR1 = 1.1, CR2 = 1.2, CR3 = 1.3, CR4 = 1.4, and all have an ercDebt of 1 ETH.

Redeemer propose [CR2, CR3, CR4] to redeem.

Disputer sees that there is in fact CR1 that is lower than all the proposed shorts.

Disputer could dispute the proposal by giving CR1 as a proof, and CR2 as the invalid CR, which would also invalid all higher CRs in the proposal. This would give (let’s take callerFeePct value from tests, 0.005 or 0.5%):

penaltyPct = min( max(0.005, (1.2 - 1.1)/1.2 ), 0.33) = min( max(0.005, 0.091), 0.33) = 0.091.

penaltyAmt = (1 + 1 + 1)ETH * 0.091 = 3 * 0.091 = 0.273 ETH.

The vulnerability lies in the fact that the disputer can dispute this proposal 3 times:

1st: dispute CR4 with CR1.

2nd: dispute CR3 with CR1.

3rd: dispute CR2 with CR1.

By doing this, the disputer will get penalty applied 3 times, and in some case, the total penalty using this trick will be higher than the penalty when disputing the whole proposal at once,

For CR1:

penaltyPct = min( max(0.005, (1.4 - 1.1)/1.4 ), 0.33) = 0.214.

penaltyAmt = 1 ETH * 0.214 = 0.214 ETH.

For CR2:

penaltyPct = min( max(0.005, (1.3 - 1.1)/1.3 ), 0.33) = 0.142.

penaltyAmt = 1 ETH * 0.214 = 0.142 ETH.

For CR3:

penaltyPct = min( max(0.005, (1.2 - 1.1)/1.2 ), 0.33) = 0.091.

penaltyAmt = 1 ETH * 0.091 = 0.091 ETH.

sum of penaltyAmt = 0.447 ETH.

We can see here how more beneficial it to adopt the second strategy.

## Impact

Higher penalty than expected for redeemer, higher reward for disputer.

## Recommended Mitigation Steps

If a user gives N as the incorrectIndex in the disputed proposal, knowing that the proposal is sorted from lowest to highest CR, ensure that proposal[N-1].CR <= disputeCR (revert in that case).

## Assessed type

Math ditto-eth (DittoETH) confirmed, but disagreed with severity and commented:

Really good find. Looks like the formula can be exploited to give a higher fee than intended. That said, I’m leaning more towards low/medium because the only effect is a higher fee for the disputer, incorrect proposals should happen very rarely from an honest user, and this doesn’t affect the redemption from working properly.

hansfriese (judge) decreased severity to Medium and commented:

I agree Medium is more appropriate.

# [M-09] Valid redemption proposals can be disputed when bad debt occurs by applying it to a SR outside of the proposal

- **Contest:** DittoETH
- **Slug:** 2024-03-dittoeth
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-dittoeth
- **Source snapshot:** competitions/2024-03-dittoeth/final_report.html

Submitted by Cosine, also found by 0xbepresent

- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/facets/RedemptionFacet.sol#L259
- https://github.com/code-423n4/2024-03-dittoeth/blob/91faf46078bb6fe8ce9f55bcb717e5d2d302d22e/contracts/libraries/LibSRUtil.sol#L151-L162

## Impact

When bad debt occurs in the system it is socialized among all short records by increasing the ercDebtRate. At the next interaction with this short the updateErcDebt function is called to apply the portion of bad debt to the short:

function updateErcDebt (STypes.ShortRecord storage short, address asset ) internal { AppStorage storage s = appStorage (); // Distribute ercDebt uint64 ercDebtRate = s.

asset [ asset ].

ercDebtRate; uint88 ercDebt = short.

ercDebt.

mulU88 ( ercDebtRate - short.

ercDebtRate ); if ( ercDebt > 0 ) { short.

ercDebt += ercDebt; short.

ercDebtRate = ercDebtRate; } As we can see the updateErcDebt function has the mentioned properties. It increases the debt and therefore, influences the CR without updating the updatedAt param. This enables the following attack path:

User creates a valid redemption proposal.

Bad debt occurs in the system.

The attacker applies the bad debt to a short record that is not part of the proposal and by doing so falls below the CR of any of the SRs in the proposal.

Attacker disputes the redemption proposal to receive the penalty fee.

The updateErcDebt function is internal, but the proposeRedemption function can be used to apply it on any SR.

## Recommended Mitigation Steps

Update the updatedAt parameter every time the updateErcDebt function is called, or/and call updateErcDebt in the disputeRedemption function on the SR at the incorrectIndex before comparing the CRs.

## Assessed type

Context ditto-eth (DittoETH) confirmed

## Rejected Primary Findings

# Rejected Primary Findings: DittoETH

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
