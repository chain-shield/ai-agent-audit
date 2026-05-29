# Rejected Primary Findings: TraitForge

# Skewed NukeFactor Calculations Disadvantage Players in Later Generations

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1418
- **Submitter:** 0x0bserver
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1418
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1418.md

## Brief Summary

NukeFactor calculations are significantly skewed for tokens in the later generations, particularly affecting the 10th generation. This results in two out of the four main player strategies, Hold to Age and Forge till Infertility, becoming ineffective. Consequently, forging as a whole loses its value in the 10th generation. Players with high `performanceFactor` (pF) will either nuke their entity or sell it shortly after minting, leading to a disparity in the value of entities based on their pF. For tokens in the 10th generation: 1. **High `performanceFactor` Players:** - Players with high pFs will either nuke or sell their entities as soon as possible (typically after 3 days), due to the ent...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The game will not be able to convert from dev owner to DAO owned

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-160
- **Submitter:** 0x3b
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/160
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-160.md

## Brief Summary

After the airdrop the DAO won't receive any profits from Nuking of NFTs and it's code won't function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_158_group

# Users can use another address to merge heir tokens

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-207
- **Submitter:** 0x3b
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/207
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-207.md

## Brief Summary

Users would be able to merge their own NFTs, by just transferring them to another address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can farm airdrop points

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-224
- **Submitter:** 0x3b
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/224
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-224.md

## Brief Summary

Users can farm airdrop points by using two good NFTs and forging them as much as possible. This allows them to accumulate a large number of airdrop points for a very low cost.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Changing contract inside TraitForgeNft will mess up the whole system

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-296
- **Submitter:** 0x3b
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/296
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-296.md

## Brief Summary

Changing `entropyGenerator` or `nukeFundAddress`, and so on... inside TraitForgeNft will mess up the whole system

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# A player's hold time is reset to zero when placing and cancelling a listing for sale

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1292
- **Submitter:** 0xAkira
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1292
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1292.md

## Brief Summary

When a player places his token for sale he gives it to the contract, this is done for a secure transaction. But if the player changes his mind about selling his token, the contract will return the token to the owner and the player will have to wait 3 days again for his token to become eligible for nuking.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `TraitForge:mintToken()` can be intentionally reverted if the generated `entropyValue` is unfavorable.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1073
- **Submitter:** 0xGreyWolf
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1073
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1073.md

## Brief Summary

Entropy in the game defines each entity's traits and characteristics. It determines crucial gameplay attributes, and the randomness of the generation is crucial to the integrity of the game. However, the user can choose to revert the minting process if the entropy value is unfavorable to him/her. **- DETAILS -** The problem is that the `entropyValue` is determined and set within the same transaction. If a user employs a smart contract wallet, they can intentionally revert the transaction if the `entropyValue` is not favorable and then attempt to mint again until they achieve the desired result. Here's the [`TraitForge::mintToken()`](https://github.com/code-423n4/2024-07-traitforge/blob/279b...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_198_group

# Non-existent input validation on `setOneYearInDays` will break protocol invariants

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1016
- **Submitter:** 0xLeveler
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1016
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1016.md

## Brief Summary

An invariant of the game is for `forgeCounts` (the number of times a forge token can `breed` before becoming infertile) of forge NFTs to be reset in `_resetForgeCountsIfNeeded` after `oneYear`, this is hardcoded in storage to `365` days. This is also an low trust setter that can rug users, among other things.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing amount validation while transferring 'nukeFundContribution'

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-558
- **Submitter:** 0xMilenov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/558
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-558.md

## Brief Summary

- The protocol loses expected revenue from the tax cut, which is intended to be contributed to the NukeFund. - Users can artificially inflate NFT prices across generations without any financial penalty, undermining the protocol's economic model. Summary The `EntityTrading` contract allows users to list and buy NFTs. However, there is a vulnerability that allows users to repeatedly buy their own NFTs at **low prices, resulting in zero contributions to the protocol's NukeFund.** This can lead to financial loss for the protocol, as users can manipulate the NFT prices across generations without paying the intended tax cut. Vulnerability Details The issue lies in the calculation of the `nukeFund...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_68_group

# Possibility of sending zero 'devShare'

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-821
- **Submitter:** 0xMilenov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/821
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-821.md

## Brief Summary

The developers may not receive their fair share of the contributions, leading to potential misalignment of incentives. Description The `NukeFund` contract has a vulnerability in the receive function, where the developer's share (devShare) of the received funds is calculated without ensuring it is greater than zero. This can lead to avoid contributing to the developer fund or to the DAO, resulting in a potential loss for the protocol. The calculation uses integer division, which truncates the result towards zero. If the `msg.value` is less than `taxCut`, the contribution to the developer's fund becomes zero. **Exploit Scenario** 1. Send ETH to Contract: A user sends a very small amount of ET...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# The total eth expected from minting gen 1 is way higher than expected

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1059
- **Submitter:** 0xR360
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1059
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1059.md

## Brief Summary

According to the docs, the amount expected of eth to get from gen 1 is 1,275 ETH. The real amount is close to 1274.8775 ETH

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ETH can get stuck on the DevFund

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-812
- **Submitter:** 0xR360
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/812
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-812.md

## Brief Summary

Eth can get stuck on the DevFund. This happens when a dev with assigned reward doesnt claim and is removed from the DevFund Explanation The DevFund allow devs to claim a portion proportional of their weight. Dev can only get rewards sent AFTER they are added. If the following conditions are met then ETH can get stuck on the contract: - dev is added - dev has rewards that can be claimed - dev is removed before claiming rewards Since no devs can claim other devs rewards, no one can withdraw it. And since devs can only claim rewards that are created after they are added, adding the dev again wont work.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Entropy is 0 when `writeEntropyBatch` functions are not initialized.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-542
- **Submitter:** 0x_6a70
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/542
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-542.md

## Brief Summary

High as when entropy is 0, NFT's minted become useless and users will basically have useless NFT's that they cannot use in the protocol, or nuke to return some of the spent funds.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# Misalignment Between Token ID and Generation in Forged Entities

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1052
- **Submitter:** 0xvd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1052
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1052.md

## Brief Summary

In the `forgeWithListed` function, the new forged entity is assigned the next available token ID, while its generation is calculated based on its parent entities. This can lead to scenarios where token IDs and generations don't align intuitively. Impact User Confusion: Players may expect token IDs to correlate with generations, leading to misunderstandings about entity lineage and age.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unnecessary and Always False Condition Check in Entropy Generation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1069
- **Submitter:** 0xvd
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1069
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1069.md

## Brief Summary

In the `writeEntropyBatch1`, `writeEntropyBatch2` and `writeEntropyBatch3` functions, there's a require statement that checks if pseudoRandomValue is equal to 999999: However, `pseudoRandomValue` is a 78-digit number, making it practically impossible for this condition to ever be true. Impact Increased Gas Costs: The unnecessary require statement is executed in a loop, consuming additional gas for each iteration. This leads to higher transaction costs for users. This is also the reason why setting entropy values cannot be set during contract initialization in a single transaction and has to be done in 3 batches due to contract & block size limits.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_90_group

# Potential Stale Listings in listForForging Function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1177
- **Submitter:** 0xvd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1177
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1177.md

## Brief Summary

The listForForging function in EntityForging.sol allows users to list their tokens for forging. However, these listings can become stale or outdated if not actively managed. Without a mechanism to clean up or validate these listings periodically, the contract can accumulate outdated data, leading to inefficiencies and a cluttered marketplace. Impact If the listings become outdated, users will have to manually cancel them by calling cancelListingForForging function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Entropy Values Visible On-Chain

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-926
- **Submitter:** 0xvd
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/926
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-926.md

## Brief Summary

The EntropyGenerator contract stores all generated entropy values in private state variables: These values are visible to anyone who can read the blockchain state. Impact The visibility of entropy values on-chain defeats the purpose of randomness in the system. This could lead to: Predictability of outcomes in any system relying on this entropy. Unfair advantages for users who can read and analyze the blockchain data. Potential for front-running or other exploits in connected systems. This vulnerability undermines the core functionality of the randomness mechanism and any features dependent on it.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_232_group

# Lack of Access Control for Batch Writing of Entropy

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-938
- **Submitter:** 0xvd
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/938
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-938.md

## Brief Summary

The functions `writeEntropyBatch1`, `writeEntropyBatch2`, and `writeEntropyBatch3` are public and can be called by anyone, potentially allowing manipulation of entropy values. The `EntropyGenerator` contract contains three public functions for writing entropy batches: These functions are publicly accessible, allowing any address to call them and potentially manipulate the entropy values. Impact The lack of access control on these critical functions could lead to: Unauthorized manipulation of entropy values, compromising the randomness of the system. Potential front-running attacks where malicious actors could predict or influence the entropy values. Disruption of the intended initialization...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# `Forge` function has re-entrancy

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-732
- **Submitter:** 0xweb3boy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/732
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-732.md

## Brief Summary

A forger can keep on forging any amount of new entity.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `forgeWithListed()` function is downcasting uint256 to uint8 which can lead to silent underflow

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-736
- **Submitter:** 0xweb3boy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/736
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-736.md

## Brief Summary

`mergerEntropy` being greater than 255 will silently overflow.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_132_group

# The admin setting a new airdrop contract will causes the `TFGNFT`s are not burnable

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1129
- **Submitter:** 3n0ch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1129
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1129.md

## Brief Summary

The admin can set a new airdrop contract by calling `TraitForgeNft#setAirdropContract`, but the `userInfo` in `Airdrop` is not carried over to the new contract. This will cause DoS on `TraitForgeNft#burn` as `airdropContract.subUserAmount()` could revert if `userInfo[user]` is less than `amount` A `TFGNFT`, which is not burnable, is also can not be used to nuke in `NukeFund#nuke`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Approval checks

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1158
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1158
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1158.md

## Brief Summary

`EntityTrading::listNFTForSale` [approval](https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/EntityTrading/EntityTrading.sol#L47-L51) check will not work

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `EntityForging::listingCount` does not reduce when canceling

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-913
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/913
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-913.md

## Brief Summary

This will lead to a wrong `listingCount` value and further complications

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# The slots are only initialized for one generation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1210
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1210
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1210.md

## Brief Summary

In `EntropyGenerator` smart contract, the slots with 78 digits number (13 concatenated entropies) should be initialized for each generation (total number of 10). And as there are the total of 10000 tokens per generation, 770 slots are therefore needed. The problem is that there is no any initialization of slots and new entropies for the next generations - it's only done for the first one.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_45_group

# Inconsistency when calculating token age time

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-671
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/671
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-671.md

## Brief Summary

In the current implementation of `NukeFund` contract, the token age is determined in `calculateAge()` and `canTokenBeNuked()` functions. However, in both cases different functions are used which should not be the case and can be considered as unexpected behavior and inconsistency.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# _beforeTokenTransfer() hook deviates from ERC721 implementation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-723
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/723
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-723.md

## Brief Summary

Currently `TraitForgetNft` contract implements ERC721 functionality. In particular, it's `_beforeTokenTransfer()` hook. The problem is that its implementation deviates from the necessary one and deviates from the standard itself as well.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Users can list tokens inside of EntityTrading and then burn the token before it's bought

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-724
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/724
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-724.md

## Brief Summary

The current functionality of `TraitForgeNft` allows users to burn their tokenIds. The only requirement is that the caller should be the owner or approved address. There is also `_beforeTokenTransfer()` hook that should check whether the tokenId is not listed in `EntityForging` contract. And if so, delist it. The problem is that the hook only checks for `EntityForging` contract and forgets about `EntityTrading` where users can trade their tokens as well. This allows a user to list his token and then DoS a user who bought it by simply burning the token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_183_group

# `onlyOwner` cannot be used as Ownable is not properly initialized

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-728
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/728
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-728.md

## Brief Summary

The contract `EntropyGenerator` uses `onlyOwner` modifier on multiple occasions but the problem is that the OZ `Ownable` contract is not initialized correctly - in particular, the owner is not initialized and the ownership over the contract is not transferred to him.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Multiple Nuke Exploitable Forging with Same Parent IDs

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-986
- **Submitter:** AuditGuy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/986
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-986.md

## Brief Summary

A vulnerability exists in the `TraitForgeNft` contract. Forging NFTs with the same parent ID can be exploited by malicious users to trigger multiple invocations of the `nuke` function, leading to repeated withdrawals from the `NukeFund.sol` contract. This poses a significant risk of draining the `NukeFund` and disrupting the game's economic balance.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_229_group

# The owner of forger listed for forge can deliberately cause players try to forge with it fail to do so

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-966
- **Submitter:** Bac0nj
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/966
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-966.md

## Brief Summary

The owner of forger can use a contract as proxy and then list forger for forge. So they can deliberately DoS those who wants to forge with their listed forgers (maybe attracted by the low cost and good properties).

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# An NFT's owner could be unfairly parted away from their tokens

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-301
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/301
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-301.md

## Brief Summary

Evidently when the protocol is unpaused there is no assurance that the owner has ample time to cancel their intentions on forging which could cause them to lose out on their tokens, also this bug case can also happen due to the chain being down considering deployment is going to be on an optimistic rollup the sequencer could be down which could also cause for the owner not to be able to cancel.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_65_group

# Entropy generation deviates from the whitepaper

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-302
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/302
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-302.md

## Brief Summary

The contract uses `block.number` and an index for entropy generation, while the [whitepaper](https://docs.google.com/document/d/1pihtkKyyxobFWdaNU4YfAy56Q7WIMbFJjSHUAfRm6BA/edit#heading=h.19r6v8uax1s2) states that **"Genesis entropy is seeded using blockhash"**. This inconsistency could lead to less secure/random entropy generation and also shows how not the intended functionality is being followed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# When merging NFT, user can bypass mintPrice corresponding to new NFT.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-569
- **Submitter:** FastChecker
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/569
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-569.md

## Brief Summary

When user calls `forgeWithListed()` to mint new NFT, user bypass mint price of new NFT.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_220_group

# [H-1] Reward Manipulation via Weight Update in `DevFund::updateDev`

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-375
- **Submitter:** GuireWire
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/375
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-375.md

## Brief Summary

The `updateDev` function in `DevFund.sol` allows the owner to arbitrarily manipulate user rewards by temporarily increasing a user's weight, potentially leading to unfair reward distribution and drainage of contract funds. **Vulnerability Details:** The `updateDev` function allows the contract owner to change a user's weight without restrictions. This can be exploited by: - Temporarily increasing a user's weight - Allowing rewards to accumulate at the higher rate - Decreasing the weight back to its original value - The accumulated rewards are not adjusted when the weight is decreased, allowing users to retain unfairly earned rewards. **Impact:** This vulnerability can lead to: - Unfair dist...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Risk of lost funds due to lack of zero-address check in functions

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1415
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1415
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1415.md

## Brief Summary

In the contract DevFund.sol, the function 'safeRewardTransfer' is missing a check to ensure that the to argument does not equal the zero address. As a result, this function could transfer funds to the zero address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_184_group

# Integer Division Exposes Incorrect Values

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1439
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1439
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1439.md

## Brief Summary

The smart contract makes use of integer division that may not provide the expected results. In Solidity, integer division truncates the fraction. As such, any integer division operation can drop significant digits that could lead to monetary loss or manipulation of the system. In the forgeWithListed() function, devFee is calculated as forgingFee / taxCut. Given the integer division, it could round down and miscalculate the fees. This can either lead to a smaller fee being extracted than expected or a larger forgerShare being credited.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Re-entrancy Vulnerability in the nuke function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1461
- **Submitter:** JC
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1461
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1461.md

## Brief Summary

The "nuke" function of the NukeFund contract is vulnerable to re-entrancy attacks. Whilst there is nonReentrant modifier in place, it is possible to perform potential re-entrancy attacks due to external calls, specifically the call which sends Ether to the msg.sender. A re-entrancy attack can occur when a contract calls external contracts and makes state changes after the external call. If the called contract is malicious and implements a fallback function, it can re-enter the caller contract before the state changes have been committed, leading to potential exploits. In this function, an amount of fund is deducted and a token is burnt after making an external call to transfer Ether. If the...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_63_group

# Unexpected Revenue Distribution

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1469
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1469
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1469.md

## Brief Summary

The contract NukeFund.solhas a potential revenue distribution issue. The contract's fallback function is intended to split received Ether appropriately, with a percentage going to a developer's address and the remainder going into other operations. However, when the `airdropContract.daoFundAllowed()` evaluates to true, only the developer's share is sent to the DAO address and no portion of the ether sent is given to the contract owner.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use of Loops without Check of Gas Limits leading to potential DoS

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1483
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1483
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1483.md

## Brief Summary

Blockchains have a block gas limit to prevent DoS attacks, and in Solidity, loops consume gas. Specifically in Ethereum, if a function's gas usage exceeds the transaction's gas limit, it will fail to execute, and the gas is lost. The `mintWithBudget` function uses a loop to continuously mint tokens until the budget is exhausted or the max token limit per generation reached. However, it's not considering the worst-case scenario where the function execution could be halted due to the exceeding Ethereum block gas limit, leading to potential Denial of Service (DoS).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_172_group

# [M-01] State variables not being updated in `cancelListing` function in `EntityTrading` contract, causing misinformation about the NFTs in the protocol

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1013
- **Submitter:** KaligoAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1013
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1013.md

## Brief Summary

The function `cancelListing` is used for removing an NFT from the TraitForge marketplace, where people can list their NFT for sale. The NFT can be listed with `listNFTForSale` function. When a user lists their NFT for sale, the state variables `listingCount` and `listings` are being updated in the contract. However, when a user removes their NFT by calling the `cancelListing` function, these 2 variables are not being updated, leaving misinformation about an NFT on the marketplace. Impact State variables not being updated when they should be, leaving misinformation about NFTs in the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# When minting NFTs with a budget a lot of gas is used transferring ETH, which results in minting less NFTs than when transfer is optimized

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-483
- **Submitter:** LogBytes
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/483
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-483.md

## Brief Summary

Whenever a user mints an NFT with a budget, for every iteration the ETH is sent to the NukeFund contract consuming gas. This can be optimized by sending the total mint price in full to the NukeFund contract. Impact The result of sending ETH transfers in a loop results in so much gas that when a user would buy NFTs in different transaction the user ends up with more NFTs than when doing it with a budget.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# arbitrary send-eth

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-555
- **Submitter:** MFaizal14
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/555
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-555.md

## Brief Summary

Unprotected call to a function sending Ether to an arbitrary address. DevFund.safeRewardTransfer(address,uint256) (contracts/DevFund/DevFund.sol#83-92) sends eth to arbitrary user Dangerous calls: - (success) = address(to).call{value: amount}() (contracts/DevFund/DevFund.sol#89)

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_144_group

# In `EntropyGenerator::getEntropy` there is a high chance of `entropy` being zero for an NFT due to precision loss, particularly if a tokens `numberIndex == 0`

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1234
- **Submitter:** McToady
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1234
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1234.md

## Brief Summary

The `getEntropy` function is used for calculating the entropy of a given token when it is minted by a user. It logic for this is shown here: During the calculation of the `entropy` variable the value of `slotValue` is divided by `10 ** (72 - position)` in the event `numberIndex == 0` (and in that case `position` also equals zero), `slotValue` will be divided by a very large number, that number shown here via chisel: Therefore if `slotValue` is smaller than this number it's `paddedEntropy` will always end up being zero. The value of each item in the `entropySlots` array is cset in the various `writeEntropyBatchX` functions in the following manner: There is no check in this logic that the `ps...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_110_group

# writeEntropy functions can be called by anyone, which according to the sponsor is wrong

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-909
- **Submitter:** MrValioBg
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/909
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-909.md

## Brief Summary

**DISCLAIMER**: Correct behaviour was confirmed by the sponsor, in Discord thread: >MrValioBg — Yesterday at 9:08 PM Hey @ManPeach.0x are the writeEntropyBatch1, writeEntropyBatch2, writeEntropyBatch3, supposed to be called by the owner, or by anyone? ManPeach.0x — Today at 3:49 AM Owner Impact Currently, the writeEntropyBatch1, writeEntropyBatch2, and writeEntropyBatch3 can be called by anyone, as they are missing the `onlyOwner` function modifier. However, the desired behavior would be that only the owner will be able to call those, as they decide when entropies are registered.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Call to non-existing contracts returns success

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1063
- **Submitter:** NoOne
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1063
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1063.md

## Brief Summary

Low level calls (`call`, `delegatecall` and `staticcall`) return success if the called contract doesn’t exist (not deployed or destructed) As written in the [solidity documentation](https://docs.soliditylang.org/en/develop/control-structures.html#error-handling-assert- The low-level functions `call`, `delegatecall` and `staticcall` return true as their first return value if the account called is non-existent, as part of the design of the EVM. Account existence must be checked prior to calling if needed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Year Duration Calculation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-395
- **Submitter:** Nyxaris
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/395
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-395.md

## Brief Summary

The contract may not accurately reset forging counts for tokens on leap years, potentially allowing or disallowing additional forging operations based on an incorrect time calculation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_154_group

# Potential Denial of Service in EntityTrading Contract

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-402
- **Submitter:** Nyxaris
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/402
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-402.md

## Brief Summary

The EntityTrading contract contains a vulnerability that can lead to a denial of service condition in the buyNFT function. This vulnerability arises from the implementation of the transferToNukeFund function and its dependency on the nukeFundAddress being set. Technical Details: The nukeFundAddress is not initialized in the constructor. The setNukeFundAddress function allows setting nukeFundAddress to any address, including address(0). The transferToNukeFund function, called within buyNFT, requires nukeFundAddress to be non-zero Impact: If nukeFundAddress is not set or is set to address(0), all NFT purchase transactions will revert. This can lead to a complete denial of service for the core...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_85_group

# Unlimited Minting exploit

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-782
- **Submitter:** OMEN
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/782
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-782.md

## Brief Summary

The mintWithBudget function allows whitelisted users to mint an unlimited number of NFTs, constrained only by their budget and the maxTokensPerGen limit. This lack of per-user minting limits during the whitelist period can lead to several significant issues: 1.Centralization Risk: A small number of wealthy users could acquire a large portion of the total supply, potentially centralizing ownership. 2.Unfair Distribution: Early whitelist participants could mint a disproportionate number of NFTs, leaving few or none for later participants. 3.Price Manipulation: Large mints could rapidly drive up the price, potentially pricing out other whitelisted users. 4.Bot Exploitation: Automated scripts c...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_94_group

# Entropy value does not always have 6 digits, leading to a non-playable Entity.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1293
- **Submitter:** PedroDowsers
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1293
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1293.md

## Brief Summary

Entropy value doesn't always have 6 digits, leading to a non-playable Entity and loss of users fund.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect reward calculation in `updateDev` leads to unfair distribution of rewards

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-15
- **Submitter:** Rhaydden
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/15
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-15.md

## Brief Summary

The `updateDev` function has a logic error that can result in incorrect calculation of pending rewards when a developer's weight is updated. This issue can lead to overpayment or underpayment of rewards, depending on whether the weight is increased or decreased. As a result, the distribution of rewards among developers becomes unfair.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_55_group

# Lack of duplicate listing check in `listNFTForSale` allows multiple active listings per NFT

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-246
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/246
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-246.md

## Brief Summary

The `listNFTForSale` function does not check if an NFT is already listed before creating a new listing. As a result, there may be multiple listings for the same NFT, causing issues when trying to buy or cancel listings.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_80_group

# Potential re-listing issue due to `listedTokenIds` mapping not being updated in `cancelListing` function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-28
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/28
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-28.md

## Brief Summary

The `cancelListing` function does not update the `listedTokenIds` mapping when a listing is canceled. This will cause an issue specifically, the `listedTokenIds[tokenId]` will still point to the old index, which can cause errors or unexpected behavior in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# Owner can frontrun forging to sell worthless NFT

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-423
- **Submitter:** Ruhum
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/423
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-423.md

## Brief Summary

Malicious user can list NFT for sale on third-party marketplaces and list it for forging at the same time. They frontrun the buyer's transaction to forge with that NFT causing its value to decrease. That will cause the buyer to pay more for a less valuable token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_79_group

# Users can bypass NukeFund tax by using third party marketplaces

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-427
- **Submitter:** Ruhum
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/427
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-427.md

## Brief Summary

User can sell NFTs through third-party marketplaces to bypass NukeFund tax set in the EntityTrading contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_112_group

# NukeFund doesn't set ageMultiplier

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-428
- **Submitter:** Ruhum
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/428
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-428.md

## Brief Summary

`NukeFund.ageMultiplier` is set to 0 by default. That means that the NukeFund contract only uses the NFT's `initialNukeFactor` to calculate their share of the funds. Any NFT burned will thus receive less ETH than they should. While this is easily fixed by the admin, they don't seem to have it on their radar. The value isn't set in either the deployment script or the testnet deployment. The loss of funds occurs whenever someone nukes their NFT. So retroactively fixing it won't undo the damage that's been done.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_42_group

# Compromised Approved Spender Can Burn all Owner's NFTs in `TraitForgeNft` contract

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-337
- **Submitter:** Sadiqmuktar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/337
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-337.md

## Brief Summary

In the `TraitForgeNft` contract, any address that has been granted approval for a specific NFT can call the burn function and destroy that NFT. This functionality is facilitated by checking if the caller is either an owner or an approved spender using `_isApprovedOrOwner`. However, this introduces a vulnerability where if an approved spender’s account becomes compromised, they could maliciously burn all NFTs they have approval for without further check and this could lead to the following: 1: Loss of Assets: Owners may lose valuable and irreplaceable NFTs. 2: Security Risk: Compromised accounts with burning privileges can cause irreversible damage. 3: User Trust: Users' trust in platform se...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Front-Running Vulnerability via Flash Loans in `EntityTrading` Contract

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-523
- **Submitter:** Sadiqmuktar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/523
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-523.md

## Brief Summary

The EntityTrading contract allows users to list and purchase NFTs. However, the `BuyNft` function is susceptible to front-running attacks. Attackers can monitor pending transactions and use flash loans to preemptively buy listed NFTs before legitimate buyers' transactions are processed which could lead to the following: 1. Financial Losses for Users: Legitimate buyers may lose out on purchasing desired NFTs or end up paying inflated prices due to artificial market manipulation by attackers. 2. Market Manipulation: The exploit undermines fair market practices by allowing malicious actors to manipulate NFT listings and sales. 3. Erosion of Trust: Such vulnerabilities erode user trust in decen...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Address is not wrap inside `payable` keyword when sending funds.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1406
- **Submitter:** Shahil_Hussain
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1406
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1406.md

## Brief Summary

`msg.sender` is not being wrapped inside `payable` keyword when sending funds resulting in the `msg.sender` not being able to receive funds. `msg.sender` can be both EOA or a contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `Whitelisting` can be bypassed via Token Transfers

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-237
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/237
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-237.md

## Brief Summary

The current implementation of the `TraitForgeNft` contract enforces `whitelisting` only during the `minting` process. However, once a `token` is minted, it can be freely `transferred` to any `address`, including `non-whitelisted` addresses. This creates a loophole where `non-whitelisted` users can indirectly obtain `tokens` by having `whitelisted` users mint and then transfer tokens to them before the `whitelist period` elapses.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_124_group

# Address Type Mismatch for `nukeFundAddress` in `TraitForgeNft` result in DoS.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-69
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/69
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-69.md

## Brief Summary

The `setNukeFundContract()` function expects a `payable address`, but the `nukeFundAddress` variable is declared as a `regular address`. This discrepancy can cause issues when trying to send funds to `nukeFundAddress`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_38_group

# Missing `pseudoRandomValue` in `EntropyGenerator::writeEntropyBatch3()`

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-70
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/70
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-70.md

## Brief Summary

The `pseudoRandomValue` is not validated in the `writeEntropyBatch3()` function, unlike in [`writeEntropyBatch1()`](https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/EntropyGenerator/EntropyGenerator.sol#L56) and [`writeEntropyBatch2()`](https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/EntropyGenerator/EntropyGenerator.sol#L76). Without validation, `pseudoRandomValue` could be set to an `invalid value`, potentially affecting the integrity of the `entropy slots`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_52_group

# Centralization risk, could ruin the protocol.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-988
- **Submitter:** Tonchi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/988
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-988.md

## Brief Summary

This `TraitForge` protocol is intended to be operated by an owner, There are some functions that should always remain decentralized or immutable but the owner can change them either without emitting any event or without anyone knowing about it or the owner can easily manipulate the protocol functionality to his advantage even if there are fair awards. They too can manipulate the owner or there is no need to give so much power to the owner, it should not be the intended behavior of smart contracts. some of those protocol functionalities are given below: - In the `DevFund.sol` contract, There should be a maximum weight in the `DevFund::addDev` function that the owner cannot give more weight t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `writeEntropyBatch*` could be revert because `pseudoRandomValue` could be `999999`.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-834
- **Submitter:** VulnViper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/834
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-834.md

## Brief Summary

In `writeEntropyBatch1` and `writeEntropyBatch2`, `pseudoRandomValue` could be `999999` and as a result, the function will be revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# TraitForgeNft:isForger( ) will always return true for any given token ID

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1121
- **Submitter:** air_0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1121
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1121.md

## Brief Summary

The value of `isForger` should not have a default value of `true` because that would incorrectly indicate that a token is a `forger` regardless of the actual entropy value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The leaf value of proof should not be exactly 64 bytes long prior to hashing

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-247
- **Submitter:** air_0x
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/247
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-247.md

## Brief Summary

When using [MerkleProof.sol](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/MerkleProof.sol) avoid using leaf values that are 64 bytes long prior to hashing. However proof in `onlyWhitelisted` modifier in `TraitForgeNft` contract uses leaf values that are exactly 64 bytes long. An attacker can bypass the merkle-tree proof and exploit the `onlyWhitelisted` modifier to mint tokens.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# EntityTrading.sol:buyNFT( ) does not check if tokenID is valid before purchasing

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-979
- **Submitter:** air_0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/979
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-979.md

## Brief Summary

Before making a purchase , the function [ EntityTrading.sol:buyNFT( )](https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/EntityTrading/EntityTrading.sol#L63) should check if the `tokenId` is valid to prevent loss of buyer funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_101_group

# EntityTrading : `buyNFT` can be purosefully reverted by the seller when the demand for NFT is reached high.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-726
- **Submitter:** ak1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/726
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-726.md

## Brief Summary

Seller can control the NFT selling by using the function `buyNFT` when the ETH is transferred to the seller.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# function writeEntropyBatch1/writeEntropyBatch2/writeEntropyBatch3 will be DOSed in contract EntropyGenerator.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-259
- **Submitter:** almurhasan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/259
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-259.md

## Brief Summary

Those functions will revert due to out gas error when executing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_208_group

# Possible accidental DOS when sending fund to DevFund contract.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1458
- **Submitter:** arman
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1458
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1458.md

## Brief Summary

Sending fund to DevFund contract will always fail if the owner is a contract and always reverts upon receiving ether.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_81_group

# Potential Loss of Funds Due to Incorrect ETH Transfer Order

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-853
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/853
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-853.md

## Brief Summary

The current implementation of the `buyNFT` function allows the seller to receive proceeds from the sale before the NFT is transferred to the buyer. This sequence creates a risk where, if the NFT transfer fails, the seller would still retain the ETH while the buyer would not receive the NFT. This situation results in the buyer losing their funds without receiving the purchased NFT. look at this part The fact that msg.value is checked when the buyNFT is called means that the buyer has already committed their money in and there is no way out. all the computations that require msg.value will definitly run. Priority should not be seller getting the proceeds because they will still get the procee...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insecure Internal Function Access in DevFund.sol

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1027
- **Submitter:** aua_oo7
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1027
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1027.md

## Brief Summary

The `safeRewardTransfer` function in the `DevFund.sol` contract is marked as internal, which means it can be called by any function within the same contract or any contract that inherits from it. This function handles the transfer of rewards to a specified address and does not include explicit access controls or checks to validate the caller's permissions beyond its internal visibility. If a derived contract maliciously or inadvertently calls this function, it could lead to unauthorized or unintended reward distributions, potentially draining the contract's funds. Impact The lack of access controls on the `safeRewardTransfer` function exposes the contract to risks where inherited contracts...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# In all the three functions calculating entropy for different phases, `writeEntropyBatch1`, `writeEntropyBatch2`, `writeEntropyBatch3`, the generated entropy is not checked against 0(null) value.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-469
- **Submitter:** azariah239
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/469
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-469.md

## Brief Summary

The functions that calculate and store entropy for each phases(1-3) lack the checking for 0(nill) values of entropy. If the value for entropy is 0, then the nukeFactor, forgePotential, performanceFactor etc will be 0 leaving it meaningless.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# we are not adding payable into claim function in "DevFund"

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-850
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/850
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-850.md

## Brief Summary

Detailed description of the impact of this finding. we are not adding payable into the claim function.we should be adding payable into the claim function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_143_group

# can get "fetchListings" for zero listingCount.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-876
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/876
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-876.md

## Brief Summary

Detailed description of the impact of this finding. if there is no listingCount is zero but we will get a output.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Not reverting an empty transaction

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1444
- **Submitter:** bhavya0911
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1444
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1444.md

## Brief Summary

This bugs allows for a transaction to go through that does not do any state change, emit events or return any data, wasting users gas in that case.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Future End Time Check in setWhitelistEndTime

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-727
- **Submitter:** boredpukar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/727
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-727.md

## Brief Summary

The [setWhitelistEndTime function](https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/TraitForgeNft/TraitForgeNft.sol#L126) does not validate if the provided end time is in the future. This can allow the owner to set an end time that is already past, effectively ending the whitelist period immediately. This could result in users being unfairly excluded from the whitelist period, impacting the fair distribution of NFTs.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_159_group

# Risk of Unintended Disruption Due to Modifiable Critical Parameters

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-734
- **Submitter:** boredpukar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/734
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-734.md

## Brief Summary

The [EntityForging contract](https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntityForging/EntityForging.sol#L40) allows the owner to modify critical parameters, such as oneYearInDays, taxCut, and minimumListFee, after deployment. Although the setter functions in the contract is regulated by the onlyOwner modifier, which implies a level of trust in the owner, the ability to change these parameters can lead to significant disruptions if not properly managed. The primary concern is the setOneYearInDays function, which allows the owner to reset the annual reset period for forging counts. If set to an unreasonably low value, this can lead to frequent resets, causing tokens...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The GoldenGod ID is Expected to Remain Confidential, However, It can be Determined Right From the Start

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1136
- **Submitter:** brevis
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1136
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1136.md

## Brief Summary

According to the protocol design, obtaining the `GoldenGod` entity gives the user a considerable advantage. Although this entity is “randomly” generated, it is stored in the contract storage. This allows the ID of the `GoldenGod` to be determined in advance once the entropies are generated. A malicious user, armed with this ID, can monitor the IDs of newly created entities and wait until they approach the `GoldenGod`, then strategically mint it. As a result, apart from defeating the game’s purpose of being fair and random, this system flaw can have a detrimental effect on other users' motivation to participate thus killing the protocol’s user engagement. Here is the function in question: As...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Slippage in Reward Distribution Due to Balance Fluctuations

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1477
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1477
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1477.md

## Brief Summary

The `claim()` function in the DevFund contract is vulnerable to a form of slippage where the amount of rewards a developer receives can be less than expected due to changes in the contract's balance between the time of calculation and the time of transfer. Details 1. The `claim()` function calculates pending rewards based on the current `totalRewardDebt`: 2. The actual transfer is performed by the `safeRewardTransfer()` function: 3. The `safeRewardTransfer()` function checks the current balance before transfer: Slippage Scenario 1. A developer calls `claim()`, and `pending` is calculated based on the current `totalRewardDebt`. 2. Before `safeRewardTransfer()` is called, the contract's balan...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_95_group

# wrong condition and calculation in getNextEntropy

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-888
- **Submitter:** cryptomoon
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/888
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-888.md

## Brief Summary

The `getNextEntropy` function has following code to ensure that the value of `currentSlotIndex` remains less than `maxSlotIndex` and value of `currentNumberIndex` remains less than `maxNumberIndex`. However, following condition is false and it is possible to have value of `currentSlotIndex` to be `maxSlotIndex` and `currentNumberIndex` to be `maxNumberIndex`. Due to such invalid values, `mint` will revert and cause DoS on all minting functionality.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_100_group

# No limit on Dev Weight can lead to unfairness in eth distribution

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1392
- **Submitter:** cryptphi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1392
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1392.md

## Brief Summary

There is no limit set for weight that can be assigned to a dev user. This can lead to unfair distribution weight where an owner is biased, thereby causing a dev claiming most of the eth claimable in the DevFund contract For fairness in eth distribution for dev claims, a fixed limit for weight should be set for which weights assigned to a dev would be within the limit predetermined.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A player can generate multiple time 'perfect' entity

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-631
- **Submitter:** desaperh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/631
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-631.md

## Brief Summary

By using two wallets, a player can generate multiple perfect entity with the same caracteristics By optimising performance factor and nuke factor, the attacker could nuke the fund multiple times

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Next minted NFT can be predicted

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-661
- **Submitter:** desaperh
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/661
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-661.md

## Brief Summary

Loss of funds for user. Some bots will have an unfair advantage.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_56_group

# The expected maximum price of NFT per generation is never reached.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1124
- **Submitter:** dobrevaleri
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1124
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1124.md

## Brief Summary

As per the [GitBook](https://github.com/TraitForge/GitBook/blob/main/GamePlay/Generations.md#generations) and the [Whitepaper](https://docs.google.com/document/d/1pihtkKyyxobFWdaNU4YfAy56Q7WIMbFJjSHUAfRm6BA/edit#heading=h.56jevmqnri5f), the final entity price of the first generation should be `0.25 ETH`. As per the communication with the sponsor the starting price of each NFT of each generation should be `0.005 ETH` >ManPeach.0x: 0.005 across all gens However the maximum value that [TraitForgeNft::calculateMintPrice](https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/TraitForgeNft/TraitForgeNft.sol#L227) for 10_000th NFT of the first gen...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_185_group

# Owner is allowed to `set max nuke to any arbitrary` value above 50%

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1006
- **Submitter:** dontonka
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1006
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1006.md

## Brief Summary

There is a `contradictory` element in `NukeFund` which allow the owner to set a max nuke for more than 50% of the NukeFund to be collected by a single nuke which `go against rules` presented to players and seems to warrant `Medium`. One of the key invariant of the game and economy is that a `single nuke cannot claim more than 50%` of the NukeFund contract funds. While the variable `maxAllowedClaimDivisor` controlling this is initialized properly, the owner has a way to `set this value to any arbitrary value` later on, which `doesn't look very legetimate` and is error-prone. I understand the flexibility here, and that the owner is a trusted actor, but this seems to cross the line a bit. Impa...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# `getListedTokenIds` is inaccurate as `listedTokenIds` is not cleaned up when unlisting

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-281
- **Submitter:** dontonka
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/281
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-281.md

## Brief Summary

There is a bug in `EntityForging::getListedTokenIds` when the player is unlisting a forger NFT which still return the previous id as if the NFT would `still be listed` which is innacurate and seems to warrant `Medium` severity. Be aware that this issue is also present in `EntityTrading` but in a more hidden way as there is not official getter, but `EntityTrading::listedTokenIds` is public and will present the same issue as `EntityTrading` is not cleaning it up either. The root cause is due to the fact that `_cancelListingForForging` is `not reseting listedTokenIds` but only listings state variable when unlisting a forger NFT. This oversight doesn't seems to cause major damage in the moment,...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `fetchListings` is inaccurate as returning always an empty first entry

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-282
- **Submitter:** dontonka
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/282
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-282.md

## Brief Summary

There is a bug in `EntityForging::fetchListings` which returns always a `first empty entry` which is innacurate and seems to warrant `Medium` severity. The root cause is due to the fact that `fetchListings` is allocating +1 entry for no good reasons. Impact `fetchListings` is not returning the proper information as always returning an `additional first empty entry`. If the caller rely on the lenght of the array, that could be an issue, as it might think there is always a forger NFT listed when it's not the case.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Forging when `nuke contract is not set` will cause the `dev cut to be burned`

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-289
- **Submitter:** dontonka
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/289
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-289.md

## Brief Summary

There is a bug in `EntityForging::forgeWithListed` when the player is forging. 10% of the forging fees (dev cut) are going to the `Nuke fund contract` but there are scenarios where this might not be set which will essentially burn this value (as sending it to address(0)), which is unexpected and seems to warrant `Medium` severity. This would usually be High severity, but since this is fully controlled by the owner, severity is reduced. The root cause is due to the fact that `forgeWithListed` is simply not checking if nuke fund has been set or not. So the contract could be deployed without calling `setNukeFundAddress` or the owner could actually call it with address(0) and those would result...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_72_group

# `Aging feature` is currently not working and `always zero`

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-397
- **Submitter:** dontonka
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/397
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-397.md

## Brief Summary

There is a bug in `NukeFund::calculateAge` which prevent the `aging feature` from working, which is a key feature of the game, so this seems to warrant `Medium` severity at least. The root cause is due to the fact that `calculateAge` is using the `ageMultiplier` which is not set during the constructor and neither after in tests and neither in the official deployment (OOS - deployTasks.ts), which means it will always be zero, which `disable the aging feature` completely. There is a actually a test `should calculate the age of a token` in the test suite, but unfortunatelly fail to uncover this as the `perfomanceFactor` of the NFT minted is zero (since entropy is 283160) and also the age teste...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_43_group

# Potential Reentrancy in claim Function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1044
- **Submitter:** dreamcoder
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1044
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1044.md

## Brief Summary

uint256 amount = (totalTokenAmount * userInfo[msg.sender]) / totalValue; traitToken.transfer(msg.sender, amount); userInfo[msg.sender] = 0; Issue Even though nonReentrant is used, transferring tokens before updating the state can still be considered a bad practice. It's better to update the state first to follow the "checks-effects-interactions" pattern. Updated Code uint256 amount = (totalTokenAmount * userInfo[msg.sender]) / totalValue; userInfo[msg.sender] = 0; // Update state before the external call traitToken.transfer(msg.sender, amount);

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_41_group

# Use ReentrancyGuard

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1049
- **Submitter:** dreamcoder
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1049
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1049.md

## Brief Summary

Even though the receive function uses an external call to swap ETH for tokens and then calls the burn function, it's important to ensure the contract is protected against reentrancy attacks. However, in this specific example, the likelihood of reentrancy is minimal due to the nature of the operations (interacting with Uniswap and burning tokens). Still, as a best practice, it's worth considering adding ReentrancyGuard to the contract. Improved Code contract DAOFund is ReentrancyGuard { ... receive() external payable nonReentrant { ... } ... }

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Proper Validation:

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1053
- **Submitter:** dreamcoder
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1053
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1053.md

## Brief Summary

uint256 amountPerWeight = msg.value / totalDevWeight; ... } ... } Issue Need to check all input values are validated correctly. Improved code receive() external payable { require(msg.value > 0, 'No ETH sent'); if (totalDevWeight > 0) { uint256 amountPerWeight = msg.value / totalDevWeight; ... } ... }

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Validation for taxcut and maxAllowedClaimDivisor

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1095
- **Submitter:** dreamcoder
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1095
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1095.md

## Brief Summary

In the code the taxcut is not percent. Is just divider So when use setTaxCut function, if tax fee is 0, coder might call setTaxCut(0) by mistake. In this case, as the code didn't validate tax cut, will cause error "divide by 0". Need to validate maxAllowedClaimDivisor, too.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_153_group

# Invalid One Year Values Cause Excessive Forge Resets

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1446
- **Submitter:** elprofesor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1446
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1446.md

## Brief Summary

Entity Forging is a feature that allows users to forge new tokens by combining two existing tokens. This feature makes new generation tokens available to the users forging. Each token that is a forger has a forging count that is reset every year. However, due to a bug in the implementation, if the one year value is set to 0, the forging count can be reset multiple times in the same block, leading to unexpected behavior and potential security issues if miners / block proposers decide to extract value out of this system. The following

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# Miss-configuration

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-470
- **Submitter:** erike1
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/470
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-470.md

## Brief Summary

Posting a vulnerable entry like this without additional documentation or information may lead to misunderstanding its purpose and possible deployment. The impact is severe if u don

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Adding a Minimum Generation Check in TraitForgeNft.sol's `setMaxGeneration` Function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-582
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/582
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-582.md

## Brief Summary

The `setMaxGeneration` function in the `TraitForgeNft.sol` contract only checks if `maxGeneration_ >= currentGeneration` but does not check if `maxGeneration_ >= 10`. This means it is possible to set `maxGeneration_` to a value less than 10, which contradicts the whitepaper's statement: ["There are a total of 10 Generations (extended as the game goes on)"](https://docs.google.com/document/d/1pihtkKyyxobFWdaNU4YfAy56Q7WIMbFJjSHUAfRm6BA/edit).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Token transfer timestamp not updated correctly

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-326
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/326
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-326.md

## Brief Summary

One of the key features of the `TraitForgeNft` contract is tracking the timestamp of the last transfer for each token using the `lastTokenTransferredTimestamp` mapping. The [_beforeTokenTransfer()](https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/TraitForgeNft/TraitForgeNft.sol#L367-L396) function is an internal hook that is called before a token transfer occurs. It is responsible for updating the `lastTokenTransferredTimestamp` and handling the unlisting of tokens from forging if they are being transferred. However, there is an issue with the logic that updates the `lastTokenTransferredTimestamp`. The condition `if (from != to)` is me...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Division by Zero

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1314
- **Submitter:** hackeroid8080
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1314
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1314.md

## Brief Summary

Description: In the receive function, there's a division operation that could result in a division by zero if totalDevWeight is zero. Impact: A division by zero would cause the transaction to revert, leading to unexpected behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# Inconsistent State Updates

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1318
- **Submitter:** hackeroid8080
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1318
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1318.md

## Brief Summary

Description: In the updateDev and removeDev functions, state updates are not adequately protected. If a transaction fails midway, it could result in an inconsistent state. Impact: An inconsistent state could lead to incorrect reward calculations and potential loss of funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Emitting Events After External Calls

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1322
- **Submitter:** hackeroid8080
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1322
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1322.md

## Brief Summary

Description: In the receive function, events are emitted after making an external call, which could lead to out-of-order events if reentrancy occurs. Impact: Out-of-order events can cause issues with event log consistency and make debugging difficult.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Constructor Parameter Validation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1370
- **Submitter:** hackeroid8080
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1370
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1370.md

## Brief Summary

The constructor parameter _traitForgetNft is not validated, potentially allowing an invalid address to be set.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Arithmetic Operations

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1381
- **Submitter:** hackeroid8080
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1381
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1381.md

## Brief Summary

Unchecked arithmetic operations might lead to integer overflow or underflow.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Denial of Service (DoS) due to delete Operation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1427
- **Submitter:** hackeroid8080
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1427
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1427.md

## Brief Summary

Using the delete operation on mappings without a secondary cleanup can leave listedTokenIds and listings in a state where they still reference old or deleted entries, potentially leading to higher gas costs or DoS attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# External Call Without a Gas Budget (Transfer to Seller and Transfer to NukeFund)

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1447
- **Submitter:** hackeroid8080
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1447
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1447.md

## Brief Summary

External Call Without a Gas Budget (Transfer to Seller) External Call Without a Gas Budget (Transfer to NukeFund)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_157_group

# Potential DOS Attack Due to High Gas Consumption with Increasing Whitelisted Users

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1428
- **Submitter:** hail_the_lord
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1428
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1428.md

## Brief Summary

High gas costs can make transactions prohibitively expensive, leading to a DOS attack where users cannot interact with the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insecure Private Entropy Slots (Lack of True Privacy for Critical Data)

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1448
- **Submitter:** hail_the_lord
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1448
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1448.md

## Brief Summary

Despite being marked as private, the data stored in entropySlots is still publicly accessible on the blockchain. Anyone with the right tools can view this data, which may contain sensitive or critical information. Impact On-chain data is publicly accessible, and sensitive information stored in private variables can still be retrieved using various blockchain analysis techniques, potentially compromising the security of the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# Front-Running Vulnerability in Slot and Number Index Selection (Deterministic Selection + Revealed Return Value)

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1455
- **Submitter:** hail_the_lord
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1455
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1455.md

## Brief Summary

It contains a conditional check on slotIndex and numberIndex against their selection points. If both indices match the respective selection points, the function returns a fixed value of 999999. This creates a vulnerability where an attacker can predict and front-run the transaction to exploit this condition, gaining the valuable return of 999999. Impact Attackers can monitor transactions and include their own transactions to exploit the condition, ensuring they gain the valuable return. This can lead to significant financial losses or disruption of the intended functionality of the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Duplicate Entropy in Pseudo-Random Value Generation (Low Probability but Potential Collisions)

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1457
- **Submitter:** hail_the_lord
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1457
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1457.md

## Brief Summary

Duplicate Entropy in Pseudo-Random Value Generation (Low Probability but Potential Collisions)

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Misleading lastInitializedIndex Handling (Incorrect Slot Initialization Count)

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1482
- **Submitter:** hail_the_lord
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1482
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1482.md

## Brief Summary

Future functions relying on lastInitializedIndex might incorrectly assume the state of initialization, leading to potential logic errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of listing update functionality, may allows users sell their Entities at a loss

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1294
- **Submitter:** inzinko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1294
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1294.md

## Brief Summary

An argument can be made that the current implementation of the `EntityTrading` contract, which does not allow for listing update is a inefficient way to handle a marketplace for nfts, this oversight can make users loose funds and sell nft at lower price than they should, some reasons are below - The capacity of a entity increases everyday, with more maturity and performance factor results into higher nuke factor, and also the ability to earn more from the nukeFund, Hence the higher the value of the entity - A whole marketplace can form from the trading platform, where prices being sold by other users may be more and a user might be selling their entity at a loss

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A User Could Avoid Minting Bad `Entropy` Via Reverting in `Refunding`

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-710
- **Submitter:** jesjupyter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/710
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-710.md

## Brief Summary

The refund mechanism in the `mintToken` and `mintWithBudget` functions, which sends excess funds back to the `msg.sender`, can be exploited by malicious users. Specifically, users can intentionally trigger the `refund` process and revert the transaction within the `receive/fallback` function if the generated entropy does not meet their desired criteria. This creates an unfair advantage and can be used to manipulate the minting process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_67_group

# Read only reentrancy in `DevFund::claim` function.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-222
- **Submitter:** kartik_giri_47538
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/222
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-222.md

## Brief Summary

There is read only reentrancy in `DevFund::claim` function because it is not following CEI pattern. And making external calls before updating the `info.pendingRewards`. The callee can perform read only reentrancy by calling `pendingRewards` function in it's `receive` or `fallback` function and can see `pendingRewards` is not being updated. **Impact:** The `info.pendingRewards` is not updated before making any external call which leads to incorrect function returns. It could be combined with other bugs in the contract to cause problems like incorrect reward calculations, unintended behaviors, or even potential loss of funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# Implement Validation Checks for TaxCut & MinimumFee Parameters

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1244
- **Submitter:** mashbust
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1244
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1244.md

## Brief Summary

In the EntityForging contract, the devFee is calculated during the forging process as follows: Here, forgingFee is essentially the _forgerListingInfo.fee. There is no validation check applied on the setter function for taxCut, which is a crucial parameter in the contract. If the admin mistakenly sets taxCut to a value greater than minimumListFee (in terms of ether), it will cause the devFee calculation to produce 0 due to integer division, effectively bypassing the devFee.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted ETH Transfers to NukeFund Contract Allow Malicious Manipulation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1065
- **Submitter:** n3smaro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1065
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1065.md

## Brief Summary

The fallback `receive` function in the `NukeFund` contract is designed to receive ETH and update the `fund` balance. However, it does not restrict or validate the source of the funds being sent. This lack of restriction means that any user can send ETH directly to the contract, thereby altering the `fund` variable. This variable is used in critical calculations, such as determining the `finalNukeFactor` and claim amounts during a nuke action. This functionality deviates from the intended behavior, which is to only accept funds from project-related activities such as minting, nuking, or forging. The ability for unauthorized users to send ETH directly could result in manipulation of the contr...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_59_group

# `nftContract` address cannot be changed in future

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-433
- **Submitter:** namx05
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/433
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-433.md

## Brief Summary

The `nftContract` address is set during the construction of the `EntityTrading` contract and cannot be changed afterward. This presents a significant limitation if there is a need to upgrade or change the NFT contract in the future. If the `nftContract` has vulnerabilities, requires upgrades, or if there is a need to migrate to a new contract, there is no mechanism to update the address in the current `EntityTrading` contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Owner can change the value of `taxCut` without notifying the users.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-435
- **Submitter:** namx05
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/435
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-435.md

## Brief Summary

The `taxCut` variable can be altered by the contract owner using the `setTaxCut()` function without notifying users. Users should be informed about changes to the tax rate prior to its implementation to avoid unexpected financial impacts. Additionally, the contract should emit an event to transparently communicate any modifications to the tax rate, ensuring that users are aware of such changes in a timely manner.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# pess-uni-v2

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-148
- **Submitter:** nnamdi0482
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/148
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-148.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_137_group

# Centralization Risk in DevFund.sol

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1018
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1018
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1018.md

## Brief Summary

Single point of failure; owner has unchecked power over fund distribution.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_17_group

# DevFund Contract: Denial of Service (DoS) Vulnerability

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-976
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/976
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-976.md

## Brief Summary

The receive function in the DevFund contract contains an unbounded operation that iterates over all developers to distribute rewards. As the number of developers increases, the gas cost of this operation grows linearly. This creates a severe vulnerability that could lead to a Denial of Service (DoS) condition, potentially rendering the contract unusable.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_160_group

# Adding an incorrect dev address leads to funds being stuck with no way to recover them

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-768
- **Submitter:** octeezy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/768
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-768.md

## Brief Summary

When an incorrect dev address is added to the dev fund there is no way to recover the funds that have been assigned to it.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Visible `currentNumberIndex` and `currentSlotIndex` Compromise Upcoming Entity's Entropy

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-346
- **Submitter:** ogKapten
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/346
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-346.md

## Brief Summary

The `getNextEntropy` function in the `EntropyGenerator` prevents unauthorized users from knowing the next entity's entropy by making `currentNumberIndex` and `currentSlotIndex` private, thus mitigating selective minting of entities. However, the `currentNumberIndex` and `currentSlotIndex` state variables are publicly accessible through the contract's storage layout. By leveraging these two variables, anyone can determine the next entity's entropy by calling the `getPublicEntropy` function in the `EntropyGenerator` contract, effectively bypassing the intended security measure.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_128_group

# The rewards that devs can receive will be affected by MEV.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-200
- **Submitter:** p0wd3r
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/200
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-200.md

## Brief Summary

The attacker can manipulate the rewards that dev can obtain by modifying the sorting results.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# EntityTrading.sol imports ERC721URIStorage but does not inherit it

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-417
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/417
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-417.md

## Brief Summary

The functions in ERC721URIStorage is not used.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Money flow in receive() fallback function in DevFund is incorrect

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-421
- **Submitter:** peanuts
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/421
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-421.md

## Brief Summary

The owner gets the fund instead of the devs.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_108_group

# Resetting forging count does not help the game as the presence of extremely powerful entities can cause game imbalance

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-498
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/498
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-498.md

## Brief Summary

The more powerful entities with high entropies will be populated, causing game imbalance.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_104_group

# Dev Cannot Claim Income if DevFund Contract is Paused

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-937
- **Submitter:** pep7siup
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/937
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-937.md

## Brief Summary

Devs are unable to claim their income if the contract is paused. This contradicts the [Dev Fund Documentation](https://docs.google.com/document/d/1pihtkKyyxobFWdaNU4YfAy56Q7WIMbFJjSHUAfRm6BA/edit#heading=h.g7dgt2kibh7m) stating that "Devs can withdraw income at any time." By imposing the `whenNotPaused` modifier on the `DevFund:claim` function, the owner can prevent Devs from claiming their eligible income, breaking the main invariant.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_92_group

# State Modifying `getNextEntropy` Function Not Protected by `whenNotPaused`

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-941
- **Submitter:** pep7siup
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/941
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-941.md

## Brief Summary

The `getNextEntropy` function modifies critical state without being protected by the `whenNotPaused` modifier. This means that the function can still be called even when the contract is paused, which can be problematic. In case the governance/owner wants to halt all `EntropyGenerator` related activities, they currently cannot do so.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_205_group

# Function receive() in NukeFund doesn't check the address(0) before sending the Eth

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1140
- **Submitter:** phoenixV110
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1140
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1140.md

## Brief Summary

There is no validation on **devAddress** and **daoAddress** to not be address(0). When the Eth is sent to `NukeFund` if the above mentioned address are not set then the transfer call will lead to burning of Dev/Dao ETH share.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# Malicious Attacker/Node can frontrun and gain the Golden number (999999)

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1342
- **Submitter:** smbv-1923
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1342
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1342.md

## Brief Summary

- As entropy has been generated on chain any attacker would mint NFT whose entropy is 999999. - The number 999999 is considered to be luckiest and would be able to majority of nuke funds while nuking. - Generation of onchain random numbers is risky as malicious attacker can take advantage of it.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_83_group

# Minting can be done by anyone that inherits the TraitForgeNft contract.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-496
- **Submitter:** stanchev
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/496
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-496.md

## Brief Summary

Minting can be done by anyone for free, that inherits the TraitForgeNft contract. Malicious user that is not whitelisted could create a contract that inherits the TraitForgeNft contract calling _mintInternal as much times as he likes. Impact - high (evades whitelisted modifier) Likelihood - high Overall - high

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# mintNewEntity() could be DOS'ed for an unknown amount of time.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-625
- **Submitter:** stanchev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/625
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-625.md

## Brief Summary

There is an edge case where mintNewEntity may be dos'ed because of the structure of the require and if statements in _mintInternal() and mintNewEntity(). In mintNewEntity the require: is at the start of the function. What that means is that if we had a _mintInternal() call beforehand with 99999 generationMintCounts[gen], here we would revert. Why is that, shouldnt mintInternal() increment the generation if it has hit the max. No because in mintInternal the incrementGeneration comes at the start, and then we increment the genCount. Which leaves a window in which the forges are DOS'ed. Yes this fixes with a single mint of a token. But if there are no available whitelisted minters at the momen...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Risk of Integer Overflow in `calculateNukeFactor` Method.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1187
- **Submitter:** swapnaliss
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1187
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1187.md

## Brief Summary

The `calculateNukeFactor` function in the NukeFund contract is susceptible to integer overflow, especially in the computation of `finalNukeFactor`. This risk is pronounced when adjustedAge or `defaultNukeFactorIncrease` are extremely large, which may lead to incorrect nuke factor values and disrupt the nuking mechanism's reliability. Impact 1.Inaccurate calculation of nuke factors for tokens with very high adjusted ages. 2.Potential undervaluation of mature tokens, resulting in inequitable fund distribution during nuking. 3.Opportunity for exploitation by malicious entities who could manipulate the system to their benefit.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_222_group

# Bias in Random Number Generation Due to Modulo Operation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1333
- **Submitter:** swapnaliss
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1333
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1333.md

## Brief Summary

The contract generates random numbers using modulo operations (`%`), which can introduce bias, particularly in smaller ranges. Impact 1.Non-Uniform Distribution: The use of modulo can lead to a biased distribution of generated values. 2.Uneven Trait Generation: Some traits or characteristics may appear more frequently than intended. 3.Exploitation Risk: Savvy players may exploit the bias, potentially disrupting the balance of the game economy.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_166_group

# `mergerEntropy` must be greater than 10 and exactly divisible by 10 otherwise, it can lead to precision loss and `forgeWithListed` function can give unexpected results.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-488
- **Submitter:** tdey
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/488
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-488.md

## Brief Summary

`mergerEntropy` must be greater than 10 and exactly divisible by 10 otherwise, it can lead to precision loss and `forgeWithListed` function of `EntityForging.sol` can give unexpected results and ultimately generate wrong `tokenId`.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# No `0` address check in `forge` & `_mintNewEntity` , `_mint` strictly requires `to` address to not be 0 address by OZ. Can lead to failure of function in case `newOwner` is `address(0)`.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-493
- **Submitter:** tdey
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/493
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-493.md

## Brief Summary

Both `forge` & `_mintNewEntity` doesn't check for `newOwner` to be non-zero. In case `newOwner` is a `address(0)` the function will fail. Vulnerability details: The `forge` function, calls `_mintNewEntity` function in the following line: `_mintNewEntity` function, calls `_mint` function of Openzeppelin in the following line: As per guidelines of Openzeppelin, the `_mint` function strictly requires `to` address to be non-zero. [Openzeppelin Docs](https://docs.openzeppelin.com/contracts/4.x/api/token/erc721#ERC721-_mint-address-uint256-) But in both `forge` & `_mintNewEntity` doesn't check for `newOwner` to be non-zero. In case `newOwner` is a `address(0)` the function will fail.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked ERC20 Token Transfer

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1366
- **Submitter:** theyardmic
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1366
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1366.md

## Brief Summary

The `claim` function in the `Airdrop` contract calls the `transfer` method on the `traitToken` ERC20 token without checking its return value. The `transfer` method is expected to return a boolean indicating whether the transfer operation was successful or not. Impact: Ignoring the return value from `transfer` can lead to undetected failures in token transfers. If the transfer fails (e.g., due to insufficient funds, a frozen account, or other issues), the contract will not be aware of this failure. Consequently, the user's balance in the contract will be set to zero, even though the tokens may not have been transferred successfully. This can result in financial losses and inconsistencies in...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_66_group

# Unchecked Return Value of Uniswap Swap

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1386
- **Submitter:** theyardmic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1386
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1386.md

## Brief Summary

The `receive` function in the `DAOFund` contract calls the `swapExactETHForTokens` method on the Uniswap V2 Router contract but does not check the return value of this call. The `swapExactETHForTokens` method is expected to return an array of token amounts representing the amounts of tokens purchased. Impact: Ignoring the return value from the `swapExactETHForTokens` method can lead to undetected failures in the token swap operation. If the swap fails or does not execute as expected (e.g., due to insufficient liquidity, incorrect path, or other issues), the contract will not be aware of this failure. This can result in ETH being sent without receiving the expected tokens, leading to potenti...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Equality in EntropyGenerator

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1414
- **Submitter:** theyardmic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1414
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1414.md

## Brief Summary

The `EntropyGenerator` contract has instances where strict equality checks are used in a potentially dangerous manner. - The `getEntropy(uint256, uint256)` function uses strict equality to check if `slotIndex` and `numberIndex` match specific selection points: - [slotIndex == slotIndexSelectionPoint && numberIndex == numberIndexSelectionPoint](https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/EntropyGenerator/EntropyGenerator.sol#L171) **Impact:** Using strict equality for selection points or role checks can be exploited if the conditions are predictable or misconfigured. This may lead to unauthorized access or predictable outputs, comp...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Fund Allocation Based on Airdrop Status in Fallback Function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-507
- **Submitter:** unnamed
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/507
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-507.md

## Brief Summary

The fallback function for receiving ETH does not properly handle fund allocation based on the airdrop status. Specifically, if the airdrop has not started, it inappropriately attempts to allocate funds based on the `daoFundAllowed` status, leading to potential fund misallocation. Vulnerability Details The fallback function interacts with the `airdropContract` to determine how to distribute developer's share of the received ETH: 1. **If the airdrop has not started (`!airdropStarted()` returns `true`):** The function attempts to send the developer's share to `devAddress`, which is appropriate. 2. **If the airdrop has started but `daoFundAllowed()` returns `false`:** The function attempts to s...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No Prevention of Multiple Calls in `EntropyGenerator::writeEntropyBatch1/2/3` can Lead to Dos

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-534
- **Submitter:** unnamed
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/534
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-534.md

## Brief Summary

There is no mechanism in place to prevent the writeEntropyBatch1 function from being called multiple times, which could lead to unintended behavior. Vulnerability Details The function is intended to be called only once, but the current implementation does not prevent multiple invocations. While there is a check to ensure that the function has not been initialized before, there are no additional safeguards to prevent multiple executions if called again after initialization. Impact If the function is called repeatedly before the initial execution finishes, it could lead to a denial-of-service (DoS) by exhausting gas or causing the function to fail.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Pausing of the Airdrop.sol contract will incidentally pause minting, `forge()` and `burn()` of the TraitForgeNft.sol contract

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-1000
- **Submitter:** yixxas
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/1000
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-1000.md

## Brief Summary

Pausing of one contract will lead to an unintended pause in certain functionalities of other contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# DAOFund is an easy target for MEV bots to exploit

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-990
- **Submitter:** yixxas
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/990
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-990.md

## Brief Summary

Massive loss of funds due to improper management of interacting with Uniswap router.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_78_group

# Possible dos when `totalDevWeight` is 0

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Submission:** V-61
- **Submitter:** zhaojohnson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-traitforge-validation/issues/61
- **Source snapshot:** competitions/2024-07-traitforge/submissions/raw/V-61.md

## Brief Summary

Some key functions will be dos when `totalDevWeight` in DevFund is 0.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_188_group
