# Rejected Primary Findings: Munchables

# Every time a user uses the `unstakeMunchable` function he loses possession of his Munchable he just unstaked

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-145
- **Submitter:** 0XRolko
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/145
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-145.md

## Brief Summary

Every time a user uses the `LandManger::unstakeMunchable` function, as well as unstaking his munchable, he also loses possession of it; this Munchable no longer belongs to him after this function has been executed.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_49_group

# `unstakeMunchable` is pauable

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-41
- **Submitter:** 0x3b
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/41
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-41.md

## Brief Summary

Pausable unstake/withdraws can be dangerous as they restrict users in the case of an emergency, like a hack - when withdraws are most needed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Rounding in `_getNumPlots` disfavors landlords and disincentivizes land ownership (locking tokens)

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-331
- **Submitter:** 0xLeveler
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/331
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-331.md

## Brief Summary

The `_getNumPlots` function in the Munchables `LandManager` contract is used to get the plots of land a landlord owns using the formula `_getNumPlots = lockManager.getLockedWeightedValue(_account) / PRICE_PER_PLOT;` due to rounding down in solidity this disfavors landlords and disincentivizes land ownership.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Incorrect plot range validation

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-175
- **Submitter:** 0xMilenov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/175
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-175.md

## Brief Summary

The incorrect plot range check can cause transactions to fail due to a wrongly implemented validation, while we have both the plot and the NFT. This disrupts the staking process, leading to losing the opportunity for both parties, staker and landlord.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_66_group

# Missing access control in _removeTokenIdFromStakedList

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-130
- **Submitter:** 0xSpacePirate
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/130
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-130.md

## Brief Summary

In `LandManager::_removeTokenIdFromStakedList` is internal and is missing access control, everyone can call the function via a contract which inherits the `LandManager` and passing on the mainAccount address and removing any tokenId they desire (or all of them). High - remove other peoples' tokenIds

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# Missing index check in `LandManager::_farmPlots()` could lead to DoS due to out-of-bonds error

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-307
- **Submitter:** 0xb0k0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/307
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-307.md

## Brief Summary

The `LandManager::_farmPlots(...)` can potentially revert due to `Index out of bounds` error, if the global `REALM_BONUSES` or `RARITY_BONUSES` arrays are not properly set.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# Continuous Plot Hopping for Attribute Farming

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-247
- **Submitter:** Auditor_Nate
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/247
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-247.md

## Brief Summary

The LandManager contract has a vulnerability that allows for "Continuous Plot Hopping for Attribute Farming." This issue is primarily due to the lack of cooldown mechanisms and rate-limiting controls in the functions stakeMunchable and transferToUnoccupiedPlot. Both functions use the forceFarmPlots modifier, which calls the _farmPlots function without adequate checks, allowing the following issues: Rapid Accumulation of In-Game Currency: Users can exploit this vulnerability to accumulate schnibbles (in-game currency) rapidly. By repeatedly calling stakeMunchable and transferToUnoccupiedPlot in quick succession, players can farm schnibbles at an accelerated rate. This undermines the in-game...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# The same user with different accounts can be `landlord` and `renter` which allows `mainAccount(renter)` to get higher rewards always.

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-226
- **Submitter:** Flare
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/226
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-226.md

## Brief Summary

The `LandManager.sol` contract allow users to be a landlord as well as the mainAccount(renter) by just creating a separte accounts for them. However when a user(mainAccount/renter) stakes his munchable for any landlord they both endups with reward when user `unstakeMunchable`. Since we know that the rewards are not transferrable , but instead of this we can still allow mainAccount(renter) to get always higher rewards by the help of landlord's account. Since the landlord is also a same user(staker) the whole control over the `updateTaxRate` function will be on that user. That mean now user can adjust tax rate as he want from the landlord's account. But actually the main problem start with th...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Allowing Unauthorized Metadata Updates

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-373
- **Submitter:** Heaven
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/373
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-373.md

## Brief Summary

The updatePlotMetadata function does not call _getMainAccountRequireRegistered(landlord), which allows subaccounts and users without plots to add fake metadata. This can lead to unauthorized users manipulating plot metadata, potentially disrupting game balance and integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_70_group

# Using memory instead of storage for toilerState in transferToUnoccupiedPlot() and unstakeMunchable() can lead to issues

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-383
- **Submitter:** LeFy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/383
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-383.md

## Brief Summary

In both unstakeMunchable() and transferToUnoccupiedPlot(), using memory to read ToilerState creates a snapshot that does not reflect changes made to the actual state in storage. This inconsistency can lead to incorrect operations and potential vulnerabilities.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Ignored Return Values in LandManager.sol

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-194
- **Submitter:** PinchasChaim
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/194
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-194.md

## Brief Summary

Ignoring return values from external calls can lead to logical errors and unexpected behaviors in the contract execution.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Newly Updated Dirty Toiler is Still Farmed instead of Exemption

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-75
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/75
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-75.md

## Brief Summary

Newly Updated Dirty Toiler is Still Farmed instead of Exempted, thereby giving undue privilege and bonus to a dirty toiler

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_09_group

# Landlord's player data will be updated on different address

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-210
- **Submitter:** Utsav
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/210
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-210.md

## Brief Summary

updatePlayer() takes main account & player as input to update the players mapping in accountManager.sol But in _farmPlots(), for updating landlord's player data it pass landlord address instead of landlord's mainAccount As result there will be 2 players mapping created in accountManager.sol for same user

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# NFT approve is not revoked after unstaking

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-125
- **Submitter:** Zims
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/125
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-125.md

## Brief Summary

The LandManager contract retains approval to transfer NFTs even after they have been unstaked. This persistent approval presents several risks: 1. Unexpected Control: Users may assume that unstaking an NFT removes all permissions, but the LandManager can still transfer their NFT. 2. Increased Attack Surface: If the LandManager contract is compromised in the future, it could affect NFTs that users believe are fully under their control. 3. Reduced NFT Liquidity: Potential buyers on secondary markets may be hesitant to purchase NFTs with existing approvals, potentially impacting the asset's value. 4. User Trust: This behavior contradicts user expectations, potentially eroding trust in the plat...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Potential Loss of Substantial Value in Plot Allocation Due to Integer Division

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-346
- **Submitter:** cheatc0d3
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/346
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-346.md

## Brief Summary

The `getNumPlots` function uses integer division to calculate plot allocation, which can lead to the loss of significant value due to truncation, especially given the high `PRICE_PER_PLOT`.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Precision Loss in Landlord's Share Calculation Due to Integer Division

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-355
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/355
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-355.md

## Brief Summary

The calculation of the landlord's share of schnibbles may result in significant precision loss due to integer division, particularly when dealing with small values of `schnibblesTotal`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Potential Integer Overflow in Renter's Share Calculation

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-356
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/356
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-356.md

## Brief Summary

The calculation of the renter's share of schnibbles is susceptible to integer overflow if the `unfedSchnibbles` accumulates to a very large value over time.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Authorization Issues in 'configUpdated' function allowing unauthorized users changing critical configurations.

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-312
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/312
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-312.md

## Brief Summary

The 'configUpdated' function permits reconfiguration of contract settings and is currently accessible to all users. To prevent unauthorized modifications to critical configurations, access to this function should be restricted.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# It's still possible to create multiple accounts to bypass checks

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-308
- **Submitter:** jesjupyter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/308
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-308.md

## Brief Summary

In the `LandManager contract`, certain constraints are enforced on accounts, such as ensuring that `landlord != mainAccount` and limiting `munchablesStaked[mainAccount].length` to a maximum of 10. However, it's still possible to create multiple accounts to bypass checks: these checks can be easily bypassed by creating and registering another account or by transferring NFTs to another account to perform staking.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# `_removeTokenIdFromStakedList` does not check for duplicate tokenId in the `munchablesStaked[address]` mapping

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-385
- **Submitter:** lonelyprince
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/385
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-385.md

## Brief Summary

If there are duplicate tokenId in the `munchablesStaked` mapping for an account then `_removeTokenIdFromStakedList` function will only remove the first occurence of the tokenId from the list. Ideally it should check for duplicate tokenIds in the array and remove all occurences from the list.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# It is possible to stake munchables to the zero address but they can never be unstaked

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-354
- **Submitter:** m4ttm
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/354
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-354.md

## Brief Summary

Tokens staked to the zero address become stuck and can never be unstaked as it is possible to stake to any landlord even if they haven't registered.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# The portion of finalBonus corresponding to rarity can be negative if the RARITY_BONUSES value is sufficiently high

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-357
- **Submitter:** m4ttm
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/357
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-357.md

## Brief Summary

When farming plots, a bonus is calculated. A portion of the bonus corresponds to the tokens rarity and this is indexed from a uint8 array, cast to int8 then cast to int16. When casting from int8 to uint8, the value will underflow if it is >= 128.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Use safeTransferFrom when transferring munchNFTs

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-270
- **Submitter:** matejdb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/270
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-270.md

## Brief Summary

Using function when transferring NFTs does not verify that the receiver can handle receiving them properly. In a time where more users depend on using smart contracts instead of EOAs this should be implemented properly. Impact Smart contract accounts that not implement receiving NFTs (ERC721) can be stuck and lost.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_24_group

# Users can update plots even if they are not landlord's anymore

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-167
- **Submitter:** mrMorningstar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/167
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-167.md

## Brief Summary

When user lock funds and became landlord his address with according plot metadata is stored in this mapping `plotMetadata`. When updating in [updatePlotMetadata](https://github.com/code-423n4/2024-07-munchables/blame/94cf468aaabf526b7a8319f7eba34014ccebe7b9/src/managers/LandManager.sol#L116) if it is not updated before it will update(create) with current `block.timestamp` and `DEFAULT_TAX_RATE` if it was it will just update `lastUpdated` with current `block.timestamp` as we can see here: The issue is there is no check if user still have funds locked in order to be able to own plots as this is mandatory to be a landlord.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unbounded Loop Vulnerability

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-256
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/256
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-256.md

## Brief Summary

The `_farmPlots` function in the `LandManager` contract contains an unbounded loop that iterates over all staked munchables. This could lead to excessively high gas costs or even cause transactions to fail due to reaching the block gas limit, potentially resulting in a denial of service (DoS) condition.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_45_group

# Timestamp Dependence Vulnerability

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-262
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/262
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-262.md

## Brief Summary

Medium. The `LandManager` contract relies heavily on `block.timestamp` for various calculations, particularly in the `_farmPlots` function. This dependence on block timestamps could potentially be manipulated by miners to a small degree, affecting the precision of farming calculations and potentially leading to unfair advantages or disadvantages for users.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Admin Privileges Allow Potential Misuse in MunchNFT Contract

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-94
- **Submitter:** obingo76
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/94
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-94.md

## Brief Summary

Admins have the ability to blacklist accounts and tokens, which grants them significant control over the contract. This could lead to potential misuse or malicious actions, such as unjustly blacklisting users or manipulating token transfers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Configuration Keys Leading to wrong Initialization of Critical Parameters

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-352
- **Submitter:** ro1sharkm
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/352
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-352.md

## Brief Summary

1. All tax rates and the price per plot could be set to zero, fundamentally breaking the economic model of the system. 2. With MIN_TAX_RATE and MAX_TAX_RATE potentially set to zero, the updateTaxRate() function's checks become meaningless allowing any tax rate to be set. 3. If PRICE_PER_PLOT is zero, users might be able to acquire land for free leading to economic imbalance and potential system abuse. 4. Zero-value BASE_SCHNIBBLE_RATE could disrupt any reward or incentive mechanisms based on this rate.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Not following the Checks-Effects-Interactions pattern

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-84
- **Submitter:** saneryee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/84
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-84.md

## Brief Summary

The `stakeMunchable` function in the `LandManager` contract don't strictly adhere to the Checks-Effects-Interactions (C-E-I) pattern, which may bring potential risks to the contract, such as reentrancy attacks. While the current implementation may not pose immediate security risks, adopting the C-E-I pattern would enhance code clarity, maintainability, and align with smart contract development best practices.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# An attacker can prevent a landlord from updating its tax rate

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-300
- **Submitter:** synackrst
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/300
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-300.md

## Brief Summary

As AccountManager.addSubAccount() can arbitrarily overwrite anyone’s main account, an attacker can prevent a chosen victim from updating their tax rate, which results in unfair game restrictions for the victim.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Return value of `transferFrom` is not checked in `stakeMunchable` function, in case of failure one can still successfully stake munchable.

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-279
- **Submitter:** unRekt
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/279
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-279.md

## Brief Summary

The `stakeMunchable` function, allows user to stake their munchables and earn rewards. The line below, transfers the munchNFT to the contract As we know `transferFrom` returns a `boolean` according to [OZ docs](https://docs.openzeppelin.com/contracts/4.x/api/token/erc20#IERC20-transferFrom-address-address-uint256-). Hence, in case of failure the function won't be notified, or it won't revert. It will continue the staking process. The following part stakes and updates metadata of munchables. Hence, in case of `transferFrom` failure this part would continue successfully. Which should not happen in case of transaction failure.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# `unstakeMunchable` function transfers after `mainAccount` and `tokenId` is deleted, will cause function to fail

- **Contest:** Munchables
- **Slug:** 2024-07-munchables
- **Submission:** V-72
- **Submitter:** unRekt
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-munchables-validation/issues/72
- **Source snapshot:** competitions/2024-07-munchables/submissions/raw/V-72.md

## Brief Summary

`transferFrom` is called on `mainAccount` and `tokenId` after being deleted. In the `unstakeMunchable` function this line ` _removeTokenIdFromStakedList(mainAccount, tokenId)` deletes `mainAccount` and `tokenId` as per the `_removeTokenIdFromStakedList` function We can see here In the next line of the `unstakeMunchable` function - `munchNFT.transferFrom(address(this), mainAccount, tokenId); ` `transferFrom` is called on `mainAccount` and `tokenId` after it is being deleted on the previous line. Which is not feasible. Using a variable after deletion can cause runtime errors and undefined behavior.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary
