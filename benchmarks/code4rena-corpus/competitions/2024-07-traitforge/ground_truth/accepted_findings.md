# Accepted H/M Findings: TraitForge

# [H-01] Wrong minting logic based on total token count across generations

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by TopStar, also found by Rhaydden ( 1, 2 ), dimulski, 0xlemon, pep7siup, aldarion, gajiknownnothing, zhaojohnson, Fitro, eta, 0x3b, dontonka, p0wd3r, pipidu83, almurhasan, 0xDazai, ogKapten, samuraii77, 0xAleko, Ruhum, jesjupyter, stanchev, MinhTriet, avoloder, persik228, Abdessamed, Shubham, AvantGard, Akay, FastChecker, 0xHash, federodes, 0xlookman, y4y, peanuts, zhanmingjing, anonymousjoe, erike1, PENGUN, pfapostol ( 1, 2 ), onthehunt11, Autosaida, LeFy, yaioxy, hakunamatata, zeroProtocol, MrValioBg, blackVul, PASCAL, VulnViper, cryptomoon, binary, Daniel_eth, 0xcontrol, Bac0nj, sl1, vinica_boy, Kunhah, 3n0ch, SharpPeaks, inzinko, 0x0bserver, jeremie, DigiSafe, dvrkzy, Abhan, Nihavent, Udsen, 0xPwned, mashbust, shikhar229169, smbv-1923, Bob, Xcrypt, LonelyWolfDemon, frodoBaggins, gkrastenov, Shahil_Hussain, dhank, Topmark, rbserver, ArsenLupin, KupiaSec, gesha17, klau5, ilchovski, ke1caM, 0xrex, shaka, nnez, and 0xJoyBoy03 TraitForgeNft::mintWithBudget function is similar to mintToken, but allows users to mint multiple tokens in a single transaction if they have a budget exceeding the minting price for one token. However, _tokenIds tracks the total number of tokens ever minted, not just the tokens in the current generation.

## Impact

In the current implementation, _tokenIds is used to control the minting process. The check while (budgetLeft >= mintPrice && _tokenIds < maxTokensPerGen) ensures that minting will stop when current generation minted tokens reaches maxTokensPerGen. Instead of checking the number of tokens minted in the current generation, the function incorrectly checks the total number of tokens minted across all generations ( _tokenIds ).

## Recommended Mitigation Steps

Use generationMintCounts[currentGeneration] instead of _tokenIds:

- while (budgetLeft >= mintPrice && _tokenIds < maxTokensPerGen) { + while (budgetLeft >= mintPrice && generationMintCounts[currentGeneration] <= maxTokensPerGen) {

## Assessed type

Invalid Validation TForge1 (TraitForge) confirmed

# [H-02] Griefing attack on seller’s airdrop benefits

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by ZanyBonzy, also found by Trooper, 0x3b, Sabit, King_ ( 1, 2 ), _karanel, sl1, Shubham, 0xR360, inzinko, 0x0bserver, PetarTolev, AvantGard, and 0xJoyBoy03

- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/TraitForgeNft/TraitForgeNft.sol#L47
- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/TraitForgeNft/TraitForgeNft.sol#L148
- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/TraitForgeNft/TraitForgeNft.sol#L294
- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/TraitForgeNft/TraitForgeNft.sol#L329

## Impact

TraitForge NFTs can be transferred and sold, but the new owner can burn or most likely nuke the token to reduce the the initial owner’s airdrop benefits.

## Recommended Mitigation Steps

Recommend introducing a check in the burn function that skips reducing user amount if the caller is the NukeFund contract or not the initial owner.

TForge1 (TraitForge) confirmed and commented:

Great find. We want to keep only minters/forgers as airdrop receivers, but yes, if they transfer or sell and that next person nukes, the initialOwner will lose entropy from their airdrop amount. This will be a real issue.

Koolex (judge) increased severity to High KupiaSec (warden) commented:

I think this issue is not valid.

The documentation regarding the airdrop states that:

In the event that a player ‘nukes’ an entity, the entropy allocated by that entity is removed.

This implies that the incentive should be removed when an entity is nuked. Even if the new entity holder performs the nuke, the entropy allocated to that entity should be removed. Since the incentive assigned to that entity does not transfer during ownership changes, the original owner remains responsible for its burning.

Therefore, I believe this should be considered either a design choice or potentially a user error.

dimulski (warden) commented:

Hey @Koolex thanks for the swift judging! The Airdrop contract is not within the scope of this audit. The TraitForge contract doesn’t inherit it, it just interacts with it in certain conditions - if the airdrop is started. Since all the logic that may cause some potential losses for the initial owner is in the Airdrop.sol contract, this issue is clearly out of scope and thus invalid.

ZanyBonzy (warden) commented:

@KupiaSec - from the docs Everytime someone mints directly or forges to mint, they are recorded in the AirDrop Contract, alongside the entropy of the entity they minted. If they nuke, that allocation given by the entity is removed.

Keyword here is “they” meaning the person doing the minting or the forging. In this case, the nuker is the buyer (or someone that the token is transferred to), someone who didn’t do the minting nor the forging, and as a result, their actions shouldn’t have to affect the original minter.

@dimulski - I’d suggest reading the issue carefully again. The root cause is in the TraitForge/NukeFund contract, the fix is also in the NukeFund contract. The airdrop points just happen to be rewards being lost which is the impact.

dimulski (warden) commented:

There are no points without the Airdrop contract, so what exactly should be fixed if the contract where the point distribution logic and their potential value is determined, is out of scope? All the interactions with that contract should be considered OOS. This is not a USDC or some other ERC20 contract, the value of these points comes from the Airdrop contract. Feel free to check the SC verdict on this type of situations.

Koolex (judge) commented:

Given that:

The bug is in scope since mapping(uint256 => address) public initialOwners; is used to track minters and forgers. Without tracking, no one will get airdrops even if the airdrop contract is ready and implemented.

Loss of funds to initial owners.

I believe the issue stands as is.

# [H-03] Incorrect percentage calculation in NukeFund and EntityForging when taxCut is changed from default value

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

taxCut is changed from default value Submitted by 0xDarko, also found by Fitro, 0XRolko ( 1, 2 ), Abdessamed, 0xDazai, namx05, samuraii77, cryptomoon, perseus, dvrkzy, and 0xAadi

- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/EntityForging/EntityForging.sol#L146
- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/NukeFund/NukeFund.sol#L41

## Impact

In both NukeFund::receive and EntityForging::forgeWithListed, if the owner changes taxCut from its default value of 10, the percentage calculations for fee distribution become severely inaccurate. This flaw can lead to:

Incorrect distribution of funds between developers, users, and the protocol.

Potential for significant financial losses or unintended gains.

Undermining of the protocol’s economic model.

Loss of user trust if discrepancies are noticed.

The root cause is the use of simple division instead of proper percentage calculation:

// NukeFund uint256 devShare = msg.value / taxCut; // Calculate developer's share (10%) uint256 remainingFund = msg.value - devShare; // Calculate remaining funds to add to the fund // EntityForging uint256 devFee = forgingFee / taxCut; uint256 forgerShare = forgingFee - devFee; Some mathematical examples:

When taxCut = 10, 1/10 = 0.1 = 10% (correct).

When taxCut = 5, 1/5 = 0.2 = 20% (intended 5%, actually 20%).

When taxCut = 20, 1/20 = 0.05 = 5% (intended 20%, actually 5%).

## Recommended Mitigation Steps

Implement a basis point (BPS) system for precise percentage calculations. This means now that with a BPS value of 10_000 you would represent 10% as 1000 and 5% in our tests as 500.

Add the following changes:

At the top of EntityForging and NukeFund where state variables reside, add the following BPS state variable and change default taxCut of 10% to correspond to the BPS values:

+ uint256 private constant BPS = 10_000; - uint256 public taxCut = 10; + uint256 public taxCut = 10_000; Update the calculations in EntityForging and NukeFund where taxCut is used.

// NukeFund - uint256 devShare = msg.value / taxCut; // Calculate developer's share (10%) + uint256 devShare = (msg.value * taxCut) / BPS; // EntityForging - uint256 devFee = forgingFee / taxCut; + uint256 devShare = (msg.value * taxCut) / BPS; Also consider adding a boundary setTaxCut to make sure taxCut isn’t greater than BPS:

function setTaxCut(uint256 _taxCut) external onlyOwner { + require(_taxCut <= BPS, "Tax cut cannot exceed 100%"); taxCut = _taxCut; }

## Assessed type

Math TForge1 (TraitForge) confirmed ZdravkoHr (warden) commented:

@Koolex - I think this issue depends on assumptions that were not confirmed when the audit was running.

What we see in the codebase is a taxCut variable that represent a part of the whole. Nowhere it was said that this cut has to be in percents. In fact, the only place where percents are mentioned, is the second link attached to the report that says 10%. And really, when you divide by 10, you get 10%.

jesjupyter (warden) commented:

I think this issue is pretty easy to solve so it’s actually not a valid issue. Here, if we want 5% rate, we just set taxcut to 20. If we want 20%, set it to 5. Nothing will be changed, so this issue looks like an admin input error.

samuraii77 (warden) commented:

The sponsor has confirmed this one so your assumptions are incorrect. It is supposed to be a %.

Abdessamed (warden) commented:

@ZdravkoHr Nowhere it was said that this cut has to be in percents All examples in the docs are referred to a %, not a share.

In fact, the only place where percents are mentioned, is the second link attached to the report that says 10%. And really, when you divide by 10, you get 10%.

There is an explicit setTaxCut function on every contract that demonstrates tax fees can change.

@jesjupyter - What you are proposing is just a workaround. Semantically, the tax represents a fee percentage and that’s how it is expected by admins to work when calling setTaxCut.

Kalogerone (warden) commented:

The setTaxCut function is onlyOwner protected:

function setTaxCut ( uint256 _taxCut ) external onlyOwner { taxCut = _taxCut; } All owner actions are trusted. The owners can still set the taxCut to their desired value, so nothing is really broken. Nowhere it was mentioned that setting taxCut to 20 should mean 20%.

TRUSTED function and still works anyway, probably a low/info.

Koolex (judge) commented:

It is clearly intended to be a percentage. We can see this from the comment in the code:

// Fallback function to receive ETH and update fund balance receive() external payable { uint256 devShare = msg.value / taxCut; // Calculate developer's share (10%) uint256 remainingFund = msg.value - devShare; // Calculate remaining funds to add to the fund contracts/NukeFund/NukeFund.sol:L38-L43 This is not an admin issue, but the code is not working as intended (i.e., percentage based calculations).

KupiaSec (warden) commented:

@Koolex - I don’t understand why it is clearly intended to be a percentage in the following code. I believe it is meant to be a denominator, as 1/10 equals 10%.

uint256 public taxCut = 10; [...] uint256 devShare = msg.

value / taxCut; // Calculate developer's share (10%) This is the code from a audit in which I participated.

// 20% of last users receive rewards uint256 Amount; if ( share == 0 && block.

timestamp >= time ) { share = totalSupply () / 5; } Perhaps it would be better to use taxcut as a percentage, but that’s a design choice rather than an issue.

# [H-04] Number of entities in generation can surpass the 10k number

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by hakunamatata, also found by zhaojohnson ( 1, 2 ), waydou, 0x3b, _karanel, oxwhite, emerald7017, Abdessamed, 0xDazai, dontonka, Tigerfrake, _thanos1, x0t0wt1w, jesjupyter, almurhasan, Shubham, D_Auditor, MinhTriet, Ruhum, stanchev, FastChecker, 0xHash, 0xlookman, ABAIKUNANBAEV, samuraii77, LeFy, Tomas0707, erike1, onthehunt11, Trooper, ogKapten, pep7siup, EaglesSecurity, shikhar229169, VulnViper, cryptomoon, CyberscopeInterns, kutugu, vinica_boy, inzinko, BajagaSec, nikhil840096, 0x0bserver, franfran20, yaioxy, DigiSafe, Nihavent, perseus, Udsen, smbv-1923, Coinymous, Abhan, AvantGard, Night, Shahil_Hussain, den-sosnowsky, LonelyWolfDemon, amaron, bhavya0911, dhank, denzi_, turvy_fuzz, rbserver, 0rpse, KupiaSec ( 1, 2 ), klau5, al88nsk, Zac, ilchovski, 0xJoyBoy03, dimulski, ke1caM, shaka, shaflow2, nnez, and ZdravkoHr Increment generation function sets the count of the entites of the new generation to 0, which could lead to incorrect calculations of the entities in new generation. What can happen is that users will forge entities that will belong to the next generation, and then via the mintToken function the generation will be incremented and genMintCount will be reset for the next generation, which means that all forged entities will not be counted.

## Impact

The main invariant of the protocol is broken, thus potentially breaking the economy of the game.

## Recommended Mitigation Steps

Do not reset the genMintCount of the generation.

TForge1 (TraitForge) confirmed

# [H-05] The maximum number of generations is infinite

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by Daniel_eth, also found by Rhaydden ( 1, 2 ), hl_, LSHFGJ, iam_emptyset, Fitro, dontonka, TopStar, Abdessamed, 0xDazai, Agontuk, 0x3b, Shubham, 0xb0k0, Ruhum, avoloder, 0xlemon, AvantGard, Tigerfrake, 0xAleko, ABAIKUNANBAEV, FastChecker, 0xHash, anonymousjoe, y4y, JustUzair, samuraii77, onthehunt11, Autosaida, Trident-Audits, yaioxy, LeFy, EaglesSecurity, MrValioBg, PASCAL, shikhar229169, pfapostol, Tonchi, AuditGuy, 3n0ch, 0x0bserver, dobrevaleri, inzinko, kartik_giri_47538, dyoff, Topmark, DemoreX, abdulsamijay, rbserver, JanuaryPersimmon2024, KupiaSec, Kalogerone, 0xJoyBoy03, Zac, ilchovski, ke1caM, nnez, 0xrex, and shaka

In ‘TraitForgeNft’ there should be a maximum number of generations (it should be capped at 10), but instead, users can mint infinite generations and there is not a limit.

## Recommended Mitigation Steps

In TraiForgeNft::\_incrementGeneration, add a check in order to do not allow the the mint of the NFT’s above the maximum generation.

Instead of doing this:

function _incrementGeneration () private { require ( generationMintCounts [ currentGeneration ] >= maxTokensPerGen, 'Generation limit not yet reached' ); currentGeneration ++; generationMintCounts [ currentGeneration ] = 0; priceIncrement = priceIncrement + priceIncrementByGen; entropyGenerator.

initializeAlphaIndices (); emit GenerationIncremented ( currentGeneration ); } Consider doing this:

function _incrementGeneration () private { require ( generationMintCounts [ currentGeneration ] >= maxTokensPerGen, 'Generation limit not yet reached' ); currentGeneration ++; require ( currentGeneration <= maxGeneration, 'Maximum generation reached' ); generationMintCounts [ currentGeneration ] = 0; priceIncrement = priceIncrement + priceIncrementByGen; entropyGenerator.

initializeAlphaIndices (); emit GenerationIncremented ( currentGeneration ); } TForge1 (TraitForge) confirmed

# [H-06] mintToken() , mintWithBudget() , and forge() in the TraitForgeNft contract will fail due to a wrong modifier used in EntropyGenerator.initializeAlphaIndices()

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

mintToken(), mintWithBudget(), and forge() in the TraitForgeNft contract will fail due to a wrong modifier used in EntropyGenerator.initializeAlphaIndices() Submitted by 0xAadi, also found by LogBytes, Fitro, Abdessamed, avoloder, Tomas0707, synackrst, dontonka, _karanel, lrivo, Autosaida, ZanyBonzy, samuraii77, x0t0wt1w, rndquu, Auditor_Nate, blackVul ( 1, 2 ), brevis, 0xAleko, zeroProtocol, MinhTriet, 0xb0k0, 0xHelium, AvantGard, Trooper, Stoicov, 0xcontrol, EdMarcavage, yaioxy, anonymousjoe, federodes, Decap, desaperh, ogKapten, LSHFGJ, PENGUN, 0xDarko, onthehunt11, Trident-Audits, ABAIKUNANBAEV, hakunamatata, MrValioBg, 0xlemon, LeFy, kingnull, PetarTolev, binary, Undefined, Bac0nj, Erko, Daniel_eth, pep7siup, vinica_boy, SharpPeaks, yixxas, kutugu, KaligoAudits, inzinko, BajagaSec, franfran20, jeremie, 0x0bserver, dobrevaleri, DigiSafe, Nihavent, mashbust, McToady, King_, Pataroff, eierina, Shahil_Hussain, bhavya0911, amaron, hail_the_lord, ArsenLupin, KupiaSec, gesha17, zxriptor, klau5, ilchovski, Zac, 0xrex, Kalogerone, dimulski, and ZdravkoHr The EntropyGenerator contract has an issue where the initializeAlphaIndices() function uses the wrong modifier. This function is supposed to be called by the TraitForgeNft contract, but it currently uses the onlyOwner modifier instead of onlyAllowedCaller.

## Impact

The initializeAlphaIndices() function will not be callable by the TraitForgeNft contract as intended. This could lead to failures in the expected functionality of the system, particularly in scenarios where the indices need to be initialized or updated by the TraitForgeNft contract while performing minting or forging. That means this vulnerability will cause DoS on mintToken(), mintWithBudget() and forge().

## Recommended Mitigation Steps

Replace the onlyOwner modifier with the onlyAllowedCaller modifier in the initializeAlphaIndices() function to ensure it can be called by the TraitForgeNft contract.

- function initializeAlphaIndices() public whenNotPaused onlyOwner { + function initializeAlphaIndices() public whenNotPaused onlyAllowedCaller {

## Assessed type

Access Control TForge1 (TraitForge) confirmed Medium Risk Findings (19)

# [M-01] Potential uninitialized entropySlots reading in getNextEntropy , causing 0 entropy mint

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

entropySlots reading in getNextEntropy, causing 0 entropy mint Submitted by pep7siup, also found by anonymousjoe, boredpukar, amaron, shaflow2, and d3e4 The getNextEntropy function can be called at any time without waiting for the write entropy batches process to finish. This could lead to the function returning an uninitialized entropy value of 000000, resulting in users losing funds to mint useless tokens and not being eligible for future airdrops as they get 0 shares. This vulnerability can severely impact the users’ trust and the protocol’s functionality.

## Recommended Mitigation Steps

Only allow getNextEntropy call if currentSlotIndex < lastInitializedIndex to ensure that the entropy slots are properly initialized:

function getNextEntropy() public onlyAllowedCaller returns (uint256) {...

+ require(currentSlotIndex < lastInitializedIndex, 'Slot not initialized'); uint256 entropy = getEntropy(currentSlotIndex, currentNumberIndex); Abdessamed (warden) commented:

@Koolex Sorry for dropping a comment now, but this is a completely invalid issue. The entropySlots will be initialized just after the deployment. This is confirmed by the deploy script.

Koolex (judge) commented:

Please check this for more context.

Abdessamed (warden) commented:

@Koolex - The validity of this issue is based on the timeframe after deployment where the entropies are not initialized, as stated by the sponsor:

This is a valid issue as there could be a timeframe where there is no initialised entropy. Better yet, we should put a batch in constructor to be sure of mitigation.

However, this information was not accessible during the audit and the deployment script I pointed out above contradicts this fact and demonstrates that entropies will be initiated just after the deployment.

# [M-02] Funds can be locked indefinitely in NukeFund.sol

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by Sabit, also found by 0xDemon, KupiaSec, and dimulski

- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/NukeFund/NukeFund.sol#L40-L61

## Impact

Permanent loss of funds.

Dependency on user action.

## Recommended Mitigation Steps

Add a function that allows the contract owner to withdraw funds in case of emergencies or when all NFTs are burned.

## Assessed type

Context Koolex (judge) decreased severity to Medium

# [M-03] A dev will lose rewards if after claiming his rewards he mints an NFT

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by 0xlemon, also found by yixxas

- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/DevFund/DevFund.sol#L69
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/DevFund/DevFund.sol#L74
Summary A developer may lose rewards if the receive function is triggered during the reward claim process from DevFund, resulting in their info.rewardDebt being higher than expected. This scenario can occur when a developer claims their rewards and then immediately mints an NFT with the funds, but the airdrop hasn’t yet started, they stand to lose those rewards. This happens because the system updates the user’s rewardDebt after the callback.

## Vulnerability Details

When claiming the dev’s rewards are calculated as follows:

uint256 pending = info.pendingRewards + (totalRewardDebt - info.rewardDebt) * info.weight; Then, we do a call to that dev to transfer his funds and only after that call do we update his reward debt to the total reward debt.

This can create a flow in which the developer loses a portion of his rewards. For example, if the dev has a contract that receives his rewards and wants to use that money to then buy an NFT, as he believes in the protocol and wants to support it, he would have a receive function like this:

receive () external payable { traitForgeNFtContract.

mintWithBudget { value:

address ( this ).

balance }( "" ); } However, in this scenario the DevFund::receive function will be triggered (if the airdrop hasn’t started) which increases totalRewardDebt and only after that the dev’s info is updated to the totalRewardDebt, which means that the user will later receive less rewards because info.rewardDebt will include the new rewards from the received minting fees but the dev didn’t claim them.

## Recommended Mitigation Steps

Follow the CEI pattern and transfer the dev’s funds only after updating all the state variables in DevFund::claim().

TForge1 (TraitForge) commented:

If I understand it correctly then it’s not an issue. I don’t see why myself or anybody else will do this. Especially using a sequencer based tx system like base. This is not an issue to me, not sure about everybody else.

Koolex (judge) commented:

@TForge1 - In this scenario, the sequencer is irrelevant. This is on EVM level and applicable on all chains.

To summarize this for better assessment:

Scenario:

A developer claims rewards from DevFund.

The developer’s contract immediately uses these funds to mint NFTs.

If the airdrop hasn’t started, this minting triggers the DevFund’s receive() function.

The totalRewardDebt is increased before the developer’s info.rewardDebt is updated.

This results in the developer’s info.rewardDebt being set higher than it should be, causing them to lose out on some rewards.

Impact: Developers may lose rewards.

Root cause: The issue occurs because the contract updates the user’s rewardDebt after transferring funds to the developer.

Note from the warden: This scenario is considered likely because developers are expected to want to support and participate in the protocol actively.

For clearer picture of the technical flow, here is a diagram:

sequenceDiagram participant DeveloperContract participant DevFund participant NFTContract DeveloperContract->>DevFund: claim() rewards activate DevFund DevFund->>DeveloperContract: transfer rewards activate DeveloperContract Note over DeveloperContract: receive() function triggered DeveloperContract->>NFTContract: mintWithBudget() NFTContract->>DevFund: receive() (minting fees) DevFund->>DevFund: Update totalRewardDebt deactivate DeveloperContract DevFund->>DevFund: Update developer's info.rewardDebt deactivate DevFund Note over DevFund: Developer's info.rewardDebt now higher than expected @0xlemon - Please confirm or correct me if I’m wrong.

TForge1 (TraitForge) commented:

Ok, fair enough. I guess it’s an issue then. not sure if I’ll do this but if I do then it can be an issue.

samuraii77 (warden) commented:

@Koolex, so we are assuming that the protocol would have a receive() function to mint NFTs automatically? Why would that be a likely thing to happen? It was never mentioned that they would be minting NFTs and reenter into their own contract, that can’t possibly be more than a QA. It classifies as a few different things - out of scope and user error, not only is it user error but it is actually owner error and owners are supposed to be much more knowledgeable than users.

0xlemon (warden) commented:

As it can be seen by the sponsor’s reply, they were not aware of this bug so it would be an owner error only if they knew the bug existed.

I don’t see why the devs wouldn’t do such thing to support their protocol by using the claimed rewards to mint NFTs. You can see that the sponsor said “not sure if I’ll do this but if I do then it can be an issue”. This means the devs can use this method.

How is it out of scope when the root cause is in the DevFund - a contract that is in-scope?

Koolex (judge) commented:

@TForge1 - There is a middle solution, implement a sweep function under restrictive condition; for instance, when there is no NFT to be nuked any longer. This way, rug-pull is not possible. You could also add to it a timeframe that has to pass first as a delay before sweeping even if all NFTs are nuked.

# [M-04] Lack of slippage protection in dynamic pricing mint function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by OMEN, also found by p0wd3r, KupiaSec, gesha17, and kutugu The mintWithBudget function uses a dynamic pricing mechanism without implementing slippage protection. Given the specific pricing parameters, this can lead to users paying more than anticipated for their NFTs, especially for larger mints or mints occurring later in a generation. Specific impacts include:

Unexpected cost increases for users, particularly for multi-NFT mints.

Potential for users to receive fewer NFTs than expected if prices rise during minting.

Possible transaction failures if the price increases beyond the user’s budget, wasting gas.

## Recommended Mitigation Steps

Add slippage protection.

## Assessed type

Context

# [M-05] Incorrect check against golden entropy value in the first two batches

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by Abdessamed, also found by DemoreX, anonymousjoe, 0xlemon, jesjupyter, Udsen, rbserver, 0rpse, KupiaSec, ZdravkoHr, and Kalogerone Entropy is stored in uint256 slots, with each slot able to contain 13 concatenated entropies. Entropies are generated in three passes, and there is a golden entropy value 999999 that should not be generated in the first two passes, as specified in the documentation:

There is a certain entropy, “999999” which is referred to as “the Golden God”, since it has perfect parameters and will exceed all other entities if played correctly.

The Golden God is scanned for and is kept out of the first 2 passes, but is deliberately set in the final pass (at some random point).

The issue is that writeEntropyBatch1 and writeEntropyBatch2 functions do not correctly check for the golden entropy:

function writeEntropyBatch1 () public { require ( lastInitializedIndex < batchSize1, 'Batch 1 already initialized.' ); uint256 endIndex = lastInitializedIndex + batchSize1; // calculate the end index for the batch unchecked { for ( uint256 i = lastInitializedIndex; i < endIndex; i ++) { uint256 pseudoRandomValue = uint256 ( keccak256 ( abi.

encodePacked ( block.

number, i )) ) % uint256 ( 10 ) ** 78; // generate a pseudo-random value using block number and index require ( pseudoRandomValue != 999999, 'Invalid value, retry.' ); // @audit INCORRECT check entropySlots [ i ] = pseudoRandomValue; // store the value in the slots array } lastInitializedIndex = endIndex; } The function (same for writeEntropyBatch2 ) attempts to ensure that no golden entropy is generated using the following check:

require(pseudoRandomValue != 999999, 'Invalid value, retry.');. However, this check is incorrect because pseudoRandomValue does NOT represent a single entropy value but represents the entire slot value, which contains 13 entropies. For example, if the pseudo-randomly generated slot value is........999999123345, the check will pass while it should not because the second entropy is a golden one.

## Recommended Mitigation Steps

The current check incorrectly validates the entire slot value. Consider updating the check to validate each of the 13 entropies within the slot value.

## Assessed type

Invalid Validation

# [M-06] TraitForgeNft: Generations without a golden god are possible

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by Trooper, also found by 0x3b ( 1, 2 ), Decap, cryptomoon, ABAIKUNANBAEV, desaperh, eta, nikhil840096, CyberscopeInterns, AuditGuy, shikhar229169, sl1, the_hunter, Nihavent ( 1, 2 ), hail_the_lord, anonymousjoe, Zac, ke1caM, and Kalogerone The golden god is the special entropy of 999,999. This entropy is set at slotIndexSelectionPoint and numberIndexSelectionPoint here. These are in the third part of the entropy list. When forging two NFTs it increases the current generations generationMintCounts. This means that less NFTs of that generation can be minted, as there are only 10,000 NFTs per generation. Therefore, the golden god might not be reached and no golden god for that generation exists.

## Recommended Mitigation Steps

There is always a chance that no golden god is minted, as the NFT contract in the current form can’t predict when users use mint or forge. In theory, the users could call forge for the last 100s or 1000s NFTs.

Consider reverting on the last forge if the golden god is not yet minted and add a additional condition when minting, that the last NFT of a generation is guaranteed to be the golden god if not yet minted.

TForge1 (TraitForge) confirmed via duplicate Issue #220

# [M-07] Discrepancy between nfts minted, price of nft when a generation changes & position of _incrementGeneration() inside _mintInternal() and _mintNewEntity()

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

_incrementGeneration() inside _mintInternal() and _mintNewEntity() Submitted by Shubham, also found by ArsenLupin, ogKapten, zhaojohnson, Tomas0707, aldarion, dontonka, 0x3b ( 1, 2 ), eLSeR17, p0wd3r, Agontuk ( 1, 2 ), jesjupyter, samuraii77, emerald7017, Ruhum, LSHFGJ, FastChecker, 0xHash, anonymousjoe, TECHFUND-inc, Trident-Audits, hakunamatata, avoloder, Abdessamed, MrValioBg, VulnViper ( 1, 2, 3 ), PNS, perseus, shikhar229169, Dots, valy001, inzinko, 0x0bserver, dobrevaleri, Udsen, ABAIKUNANBAEV, eierina, StraawHaat, valentin_s2304, dimah7, gkrastenov, LeFy, dhank, denzi_, King_, rbserver, KupiaSec, Ryonen, 0xJoyBoy03, ZdravkoHr, 4rdiii (

1, 2 ), dimulski, and 0xrex

- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/TraitForgeNft/TraitForgeNft.sol#L280-L283
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/TraitForgeNft/TraitForgeNft.sol#L227-L232

## Vulnerability details

The report covers two bugs:

User pays the maximum price of the last generation when minting the 1st nft of the next generation.

Wrong positioning of _incrementGeneration() will lead to the user paying the starting price of 1st nft of generation 1 even when the nft being minted is the 1st nft of a different generation.

## Recommended Mitigation Steps

Fixing the bugs would require changes in the calculateMintPrice() and _mintInternal(). However, the protocol will not be able to claim the the last calculated price as intended by the whitepaper but it would then align with the number of nfts minted & the price the user pays.

File:

TraitForgeNft.

sol function calculateMintPrice () public view returns ( uint256 ) { + if ( generationMintCounts [ currentGeneration ] > 0 ) { uint256 currentGenMintCount = generationMintCounts [ currentGeneration ]; uint256 priceIncrease = priceIncrement * currentGenMintCount; uint256 price = startPrice + priceIncrease; return price; } + else { + uint256 price = startPrice + (( currentGeneration - 1 ) * priceIncrementByGen ); + return price; } File:

TraitForgeNft.

sol function _mintInternal ( address to, uint256 mintPrice ) internal { - if ( generationMintCounts [ currentGeneration ] >= maxTokensPerGen ) { - _incrementGeneration (); - } _tokenIds ++; uint256 newItemId = _tokenIds; _mint ( to, newItemId ); uint256 entropyValue = entropyGenerator.

getNextEntropy (); tokenCreationTimestamps [ newItemId ] = block.

timestamp; tokenEntropy [ newItemId ] = entropyValue; tokenGenerations [ newItemId ] = currentGeneration; generationMintCounts [ currentGeneration ]++; initialOwners [ newItemId ] = to; + if ( generationMintCounts [ currentGeneration ] >= maxTokensPerGen ) { + _incrementGeneration (); + }...

## Assessed type

Error TForge1 (TraitForge) confirmed via duplicate Issue #210 Koolex (judge) decreased severity to Medium

# [M-08] Lack of ability to make an some external function calls makes the DAO stage unreachable

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by pfapostol, also found by 0xlemon and yixxas Switching to DAO phase in NukeFund is not possible. This vulnerability is on the boundary of scope because it makes it impossible to use the out-of-scope contract but the problem is in the contact inside the scope that is the owner of this external contract.

The TraitForgeNft contract is written in such a way that it can add and remove users from the airdrop. For this the NFT contract must have ownership over the airdrop contract.

Because of this, an additional function is created in TraitForgeNft to start the airdrop while other functions that can only be called by the owner ( setTraitToken, allowDaoFund ) are absent in TraitForgeNft contract. Which makes these functions impossible to call.

The first function may not be necessary (deployer can call it at deploy), but the second function must have its own handle in the TraitForgeNft contract since this function is used by NukeFund and can only be called after the start of the airdrop; that is, after the transfer of ownership to the TraitForgeNft contract.

## Recommended Mitigation Steps

Add delegate handle to TraitForgeNft:

diff --git a/contracts/TraitForgeNft/TraitForgeNft.sol b/contracts/TraitForgeNft/TraitForgeNft.sol index b933d74..14b2f99 100644 --- a/contracts/TraitForgeNft/TraitForgeNft.sol +++ b/contracts/TraitForgeNft/TraitForgeNft.sol @@ -86,6 +86,11 @@ contract TraitForgeNft is ITraitForgeNft, ERC721Enumerable, ReentrancyGuard, Own airdropContract.startAirdrop(amount); //@external-logic } + function allowDaoFund() external whenNotPaused onlyOwner { + airdropContract.allowDaoFund(); + } + function setStartPrice(uint256 _startPrice) external onlyOwner { startPrice = _startPrice; }

## Assessed type

Access Control pfapostol (warden) commented:

I think this issue was marked as invalid because validators mistakenly flagged it as a duplicate of Issue 214, but this is a completely different issue. Issue #214 addresses the normal behavior by reporting that “the TraitForgeNft cannot call subUserAmount and addUserAmount because they are ownable”, but this is incorrect since the ownership is transferred to TraitForgeNft.

Since the ownership is transferred to TraitForgeNft, allowDaoFund cannot be called (not implemented); which results in the failure to switch to the DAO stage of the project.

samuraii77 (warden) commented:

@Koolex - I just want to mention that the Airdrop contract is OOS and any issues related to improper access control set on it should not be valid at all.

Koolex (judge) commented:

I believe the issue is valid. Owner of Airdrop is TraitForgeNft and the bug prevents from entering the DAO stage. The root cause and the fix is in TraitForgeNft, therefore it is in scope.

Note: For full discussion, see here.

# [M-09] Golden God tokens can be minted twice per generation

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Golden God tokens can be minted twice per generation Submitted by waydou, also found by 0xb0k0, Decap, robertodf99, Autosaida, 0x3b, Tomas0707 ( 1, 2 ), _karanel, Trooper, rndquu, D_Auditor, MKVsentry, samuraii77, unnamed, yaioxy, ogKapten, anonymousjoe, peanuts, y4y, LeFy, MrValioBg, shikhar229169 ( 1, 2 ), artilugio0, Phoenix_x, 0xR360, cryptomoon, Coinymous, pfapostol, 0xPwned, Yunaci, Nihavent, 3n0ch, Xcrypt, Quaternion2691, perseus, Udsen, LonelyWolfDemon, den-sosnowsky, Shahil_Hussain, nnez, Zac, ke1caM, shaka, 4rdiii, and Kalogerone The current implementation of the EntropyGenerator contract allows for the possibility of having two “Golden God” tokens (tokens with an entropy value of 999999) per generation. This duplicity undermines the intended uniqueness and rarity of the Golden God token, potentially disrupting the game’s economy and fairness. The predictability of one Golden God token, combined with the chance of a second one, could lead to exploitation and loss of player trust.

Recommended Mitigation Steps:

Add a boolean to check if a golden god token has already been issued.

## Assessed type

Context TForge1 (TraitForge) acknowledged and commented:

Although it is technically an issue, it is definitely not high risk. Low at most as it doesn’t affect the game that much, just doesn’t make the golden god as cool.

Koolex (judge) decreased severity to Medium and commented:

Based on the given criteria, This would be categorized as Medium.

Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements

- https://docs.code4rena.com/awarding/judging-criteria/severity-categorization
Reasoning:

Assets are not at direct risk, which rules out a High severity categorization.

The function of the protocol is impacted, as the uniqueness and rarity of the “Golden God” token are compromised.

The issue could impact the game fairness, which are core aspects of the protocol’s intended function.

Abdessamed (warden) commented:

This is an intended behavior and a design choice. It is explicitly mentioned in the docs that Golden entropy can be selected at random slots and indexes:

The getEntropy function calculates an entropy value based on a given slot and number index. It first verifies that the provided slot index is within bounds. If the slot and number indices match the predefined selection points, a special value of 999999 is returned. Otherwise, it computes the position for slicing the entropy value from the selected slot in the entropySlots array.

Koolex (judge) commented:

@Abdessamed - I don’t see where it mentions two Gods are allowed (even implicitly).

Abdessamed (warden) commented:

@Koolex - Golden entropy is generated on the third batch, so it exists on the entropySlots for every generation. The documentation I linked explicitly says that when the function getEntropy is called, it can return 999999 directly without consulting the entropySlots state which makes the fact that golden entropies can be found twice per generation.

Note: For full discussion, see here.

# [M-10] Imprecise token age calculation results in an incorrect nuke factor, causing users to claim the wrong amount

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by Matin, also found by preslavxyz, JuggerNaut63, samuraii77, the_hunter, stanchev, swapnaliss, inzinko, Udsen, nikhil840096, and Abhan The function calculateAge() computes the age of a token ID based on the current and creation time difference. This token age is then used to determine the nuke factor to calculate the possible claim amount.

The problem here is that when calculating the age of a token, it first divides the time difference to 1 day in seconds, and then multiplies the result to the other variables.

function calculateAge ( uint256 tokenId ) public view returns ( uint256 ) { require ( nftContract.

ownerOf ( tokenId ) != address ( 0 ), 'Token does not exist' ); uint256 daysOld = ( block.

timestamp - nftContract.

getTokenCreationTimestamp ( tokenId )) / 60 / 60 / 24; uint256 perfomanceFactor = nftContract.

getTokenEntropy ( tokenId ) % 10; uint256 age = ( daysOld * perfomanceFactor * MAX_DENOMINATOR * ageMultiplier ) / 365; // add 5 digits for decimals return age; } With having this pattern, users will face significant losses. Especially, in the case of small differences (less than 1 day), the daysOld variable becomes 0, making the age zero respectively.

## Recommended Mitigation Steps

Consider multiplying the numerator variables first and then divide by the denominator:

function calculateAge(uint256 tokenId) public view returns (uint256) { require(nftContract.ownerOf(tokenId) != address(0), 'Token does not exist'); - uint256 daysOld = (block.timestamp - - nftContract.getTokenCreationTimestamp(tokenId)) / - 60 / - 24; uint256 perfomanceFactor = nftContract.getTokenEntropy(tokenId) % 10; - uint256 age = (daysOld * + uint256 age = (perfomanceFactor * MAX_DENOMINATOR * + (block.timestamp - nftContract.getTokenCreationTimestamp(tokenId)) * ageMultiplier) / (60 * 60 * 24 * 365); // add 5 digits for decimals return age; }

## Assessed type

Math TForge1 (TraitForge) acknowledged and commented:

Age multiplier is marked as an invalid value, and should not have been used as I marked several times in the public channel (I forgot to remove from codebase which is my fault). The default for age multiplier should be 0.

The difference of nuke factor % between each day is 0.006% which is not much 2.5% / 365. Although this is still a valid finding as it could be more accurate, I personally would mark as low risk.

Koolex (judge) commented:

The issue as demonstrated is valid with a Medium risk. For fairness, since it is a part of the actual code, I will keep it as it is (Medium). If there is something that I may have missed here that may change the risk level, please let me know. Will consider inputs from all parties in post-judging QA, as usual.

dimulski (warden) commented:

@Koolex - I believe this issue is invalid, the sponsor was asked several times about the ageMultiplier value in the public chat as this value can’t be found anywhere in the tests or deployments scripts, and was adamant that this value won’t be used.

From the code implementation, it is clear that the age multiplier should increase only once whole days are passed not seconds or minutes. I believe this issue is invalid, or QA at best. Given the fact that the sponsor has mentioned that ageMultiplier won’t be used multiple times in the public channel it will be extremely unfair to other wardens to validate this issue.

Kalogerone (warden) commented:

I agree with the comment above and want to add some additional info as to why this issue seems like an invalid, at best QA. Quoting from the issue report:

With having this pattern, users will face significant losses. Especially, in the case of small differences (less than 1 day), the daysOld variable becomes 0, making the age zero respectively.

The daysOld variable is how many days old an NFT is. I don’t understand why it shouldn’t be 0 if an NFT is minted less than 24 hours ago. It’s not an hoursOld variable. What matters to the protocol is how many full days old an NFT is. There is no precision loss here and the variable works as intended.

Also, the issue above in the PoC is setting the ageMultiplier variable to something different that 1 to make the percentage of the error seem big ( ageMultiplier = 150000 ). With the correct ageMultiplier = 1, the calculation of the daysOld variable is at most 23 hours and 59 minutes behind. Which is not really behind because, again, the variable is supposed to calculate the full days of the NFT’s existence. The sponsors have mentioned multiple times that the ageMultiplier should be set to 1 for testing.

Koolex (judge) commented:

After further review, I will keep it as it is, since it is a part of the actual code; which I believe it should take priority over the public chat. I understand that this might not be favourable to some of you, but I have to do this, to keep the game fair.

# [M-11] Duplicate NFT generation via repeated forging with the same parent

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by avoloder, also found by _karanel, stanchev, Trident-Audits, 0xb0k0, Tamer, yaioxy, CyberscopeInterns, AuditGuy, 0xvd, n3smaro, AvantGard, Shahil_Hussain, and ZdravkoHr

- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/EntityForging/EntityForging.sol#L102-L144
- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/TraitForgeNft/TraitForgeNft.sol#L153-L179

## Impact

It is possible to get the same NFT (same generation, same entropy) when the forging is done with the same parentId multiple times. Although the number of times this action will be feasible is limited with the forging potential of the token it can still occur at least once.

Scenario:

User A lists his/her NFT for forging.

User B forges his/her NFT with the NFT from User A.

The new NFT is created with the combined properties of the NFT1 and NFT2.

User A lists his/her NFT for forging again.

User B forges his/her NFT with the NFT from User A again.

The NFT will be of the same generation and will have the same entropy as the one forged before.

## Recommended Mitigation Steps

You could use a mapping(bytes32 => bool) forgedPairs to store token ids that have been forged before. An additional function to generate the unique key for id pairs would be necessary, for example:

keccak256(abi.encodePacked(id1 < id2 ? id1: id2, id1 < id2 ? id2: id1)); This type of hashing would ensure that you treat two pairs in reverse order as the same, since (entropy1 + entropy2) / 2 is the same as (entropy2 + entropy1) / 2.

You could then add require statement to check if the pair has already been used before. Don’t forget to add the pairs to the mapping after they have been forged.

## Assessed type

Error TForge1 (TraitForge) confirmed dimulski (warden) commented:

@Koolex - This is not a real issue, nowhere in the docs or in the code is it even slightly mentioned that minting with NFTs with the same parents is forbidden. There is no randomness at all in the project, as the entropies for all NFTs can be calculated before the user mints them, or forges. The entropy is used in different game mechanics but having the same entropy for NFTs is guaranteed for NFTs from different generations. I don’t see how this is an issue, it doesn’t lead to any lost funds, doesn’t break any protocol invariants in the slightest, and there is no broken functionality. This is at best some design recommendation.

samuraii77 (warden) commented:

I agree, I also wanted to mention that, it’s never mentioned that parents should not merge with each other more than once and it’s in no way problematic.

As a matter of fact, it is completely expected for parents to merge more than once, those who have a brother or sister will probably agree with me.

Koolex (judge) commented:

Not every implementation in the code must be mentioned in the docs and the sponsor confirmed the issue. This confirm that it wasn’t intended since we don’t have in the docs or the code what states otherwise.

dimulski (warden) commented:

@Koolex - I disagree with your decision. The only way this qualifies as a medium is if it was stated as an invariant of the protocol. The documentation was a google doc file, which the sponsor edited multiple times during the audit, and still didn’t include this. The fact that this is undesired behavior was not disclosed in any public way before or during the audit. Given that this is an NFT game and developers can come up with whatever logic they want, they have to disclose all of their decisions before or during the audit.

Lastly, I would like to ask since when whether the sponsors confirms something or not, decides whether an issue is valid or not. This issue doesn’t cover a single thing from the requirements for a medium issue.

Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

Someone could have reported that NFT with id 55 and NFT with id 77 shouldn’t be forged if there is no full moon, and since the sponsor didn’t explicitly say these NFTs can forge even without a full moon, should this also be considered a medium issue?

Abdessamed (warden) commented:

@Koolex - This submission was more like a speculation of something that might be wrong. The documentation was very clear about how forging should happen and all its requirements (forging role, number of forging times per generation). The codebase was very clear on the forging process. If we ask the warden who submitted the report how he knows that this is an issue, he wouldn’t have a clear response since he has no reference to go to that proves this is indeed an issue. We feel like this was intended behavior and the sponsor decided to change the forging behavior after reading this submission, so it was more like a feature suggestion.

Koolex (judge) commented:

From README Core Mechanics Entities: NFTs with unique traits and parameters impacting the game.

Same generation and same entropy goes against this Mechanic. Therefore, I still see it an issue.

# [M-12] NFTs mature too slowly under default settings.

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by sl1, also found by 0x3b, peanuts, anonymousjoe, samuraii77, shikhar229169, hakunamatata, MrValioBg, cryptomoon, perseus, vinica_boy, 0x0bserver, onthehunt11, ABAIKUNANBAEV, persik228, dyoff, DemoreX, rbserver, KupiaSec, ZdravkoHr, ke1caM, hl_, Kalogerone, and dimulski

- https://github.com/code-423n4/2024-07-traitforge/blob/279b2887e3d38bc219a05d332cbcb0655b2dc644/contracts/NukeFund/NukeFund.sol#L20
Description The Nuke Fund accumulates ETH from new mints and economic activity. After a 3-day maturity period, anyone can nuke their entity to claim a share of the ETH in the Fund. Every entity has a parameter, called initialNukeFactor, set on mint which represents how much of the Fund can be claimed on nuke. The maximum total nukeFactor is 50%, expressed as 50_000.

The calculations of finalNukeFactor consist of adjustedAge, defaultNukeFactorIncrease and initialNukeFactor.

NukeFund.sol#L145-L148 uint256 initialNukeFactor = entropy / 40; // calculate initalNukeFactor based on entropy, 5 digits uint256 finalNukeFactor = (( adjustedAge * defaultNukeFactorIncrease ) / MAX_DENOMINATOR ) + initialNukeFactor; The age is calculated via calculateAge() function.

NukeFund.sol#L121-L131 uint256 daysOld = ( block.

timestamp - nftContract.

getTokenCreationTimestamp ( tokenId )) / 60 / 60 / 24; uint256 perfomanceFactor = nftContract.

getTokenEntropy ( tokenId ) % 10; uint256 age = ( daysOld * perfomanceFactor * MAX_DENOMINATOR * ageMultiplier ) / 365; // add 5 digits for decimals The idea behind age calculations is quite simple: we calculate how many days have passed since token creation and convert that into years.

However, the default value of the nukeFactorIncrease set to 250 is extremely low. While whitepaper mentions that the fastest maturing NFT should mature (reach nukeFactor of 50_000 ) in around 30 days and the slowest in 600 days, in reality it would take 4050 days for the best possible NFT to fully mature.

To showcase this let’s do some math:

To mature NFT must reach finalNukeFactor of 50_000, the best possible NFT would have 25000 initialNukeFactor and performanceFactor of 9.

defaultNukeFactor is set to 250 finalNukeFactor = adjustedAge * defaultNukeFactorIncrease + initialNukeFactor 50000 = x * 250 + 25000 25000 = 250x x = 100 adjustedAge = daysOld * performanceFactor / 365 100 = x * 9 / 365 9x = 100 * 365 x = 365000 / 9 = 4055.

For the best NFT to mature fully in 30 days, the defaultNukeFactor would need to be 135 times bigger, more precisely 33750.

## Impact

NFTs mature extremely slowly with default settings, to the point where performanceFactor of an NFT does not play any role and finalNukeFactor is determined solely by the initialNukeFactor.

## Recommended Mitigation

defaultNukeFactorIncrease variable should be set to reasonable value, that will correctly reflect the speed at which NFTs should mature. Judging by the docs, its value should be set to at least 33750.

TForge1 (TraitForge) confirmed

# [M-13] Pause and unpause functions are inaccessible

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by Ward, also found by klau5, atoko, MSaptarshi, _thanos1, LogBytes, 0xAleko, c4whitehat, Autosaida, lrivo, TECHFUND-inc, dic0de, Tigerfrake, zhaojohnson, gajiknownnothing, Abdessamed, UAARRR, oxwhite, p0wd3r, ZanyBonzy, PetarTolev, kodyvim, samuraii77, Mahi_Vasisth, jesjupyter, 0xcontrol, persik228, blackVul, peanuts, Matin, zarkk01, zeroProtocol, yaioxy, avoloder, 0xDarko, ABAIKUNANBAEV, 0xAkira, boraichodrunkenmaster, Bac0nj, anonymousjoe, Stoicov, JustUzair, aldarion, _karanel, Shubham, unnamed, 0xjarix, King_, 0xlemon, ak1, Trident-Audits, MKVsentry, kingnull, dontonka, shui, nervouspika, VulnViper, Chad0, 0xDemon, binary, Yunaci, mgf15, sl1, pep7siup, cryptomoon, yixxas, 4B, kutugu, dimah7, BajagaSec, korok, ast3ros, kartik_giri_47538, dobrevaleri, rekxor, dyoff, 0x0bserver, 0xAadi, Pataroff, Xcrypt, LonelyWolfDemon, Shahil_Hussain, arman, hail_the_lord, bhavya0911, ArsenLupin, KupiaSec, Ryonen, hl_, ilchovski, Stefanov, ke1caM, dimulski, ZdravkoHr, and Kalogerone

- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/TraitForgeNft/TraitForgeNft.sol#L19
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/NukeFund/NukeFund.sol#L11
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntropyGenerator/EntropyGenerator.sol#L9
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntityTrading/EntityTrading.sol#L11
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/DevFund/DevFund.sol#L9
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntityForging/EntityForging.sol#L10

## Impact

All in-scope contracts of TraitForge inherit from the Pausable contract of OpenZeppelin, a feature designed to allow the pausing and unpausing of contract functionalities in emergency situations or for maintenance. However, (parent) contracts do not expose the _pause() and _unpause() functions and the Pausable contract contains only internal pausing/unpausing functions.

As a result, despite inheriting the pausability feature, administrators are unable to utilize these critical controls to pause or resume the contract’s operations when needed, potentially leading to issues during periods requiring immediate intervention.

## Recommended Mitigation Steps

To leverage the full capabilities of the Pausable inheritance and enhance the contract’s operational security and flexibility, it is recommended to expose the pause and unpause functions in the parent contracts. These should be accessible by the contract owner or other authorized roles, ensuring they can respond effectively to operational needs or emergencies:

/** * @dev Pauses all functions affected by `whenNotPaused`.

*/ function pause () public onlyOwner { _pause (); } /** * @dev Unpauses all functions affected by `whenNotPaused`.

*/ function unpause () public onlyOwner { _unpause (); }

## Assessed type

Library TForge1 (TraitForge) confirmed

# [M-14] Forger Entities can forge more times than intended

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by AvantGard, also found by Tigerfrake, gajiknownnothing, zhaojohnson, robertodf99, Abdessamed, _karanel, 0x3b, p0wd3r, ZanyBonzy, almurhasan, Agontuk, oxwhite, samuraii77, D_Auditor, MinhTriet, peanuts, stanchev, Tamer, 0xb0k0, air_0x, anonymousjoe, Autosaida, MrValioBg, pfapostol, shikhar229169, hakunamatata, LeFy, dontonka, shui, VulnViper, jesjupyter, sl1, rndquu, Coinymous, cryptomoon, 0xvd, 0xlemon, onthehunt11, 0xPwned, Xcrypt, dyoff, nervouspika, perseus, Nihavent, LonelyWolfDemon, den-sosnowsky, Udsen, inzinko, dhank, hail_the_lord, turvy_fuzz, rbserver, yotov721, KupiaSec, al88nsk, Ryonen, klau5, nnez, ke1caM, shaflow2, shaka, ZdravkoHr, Beosin, and dimulski

- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntityForging/EntityForging.sol#L87-L90
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntityForging/EntityForging.sol#L133

## Vulnerability details

As per the docs:

Entities’ Forge Potential is determined by the [5] digit in entropy Each entity may have the ability to forge 0-9 times depending on its Forge Potential.

This means an entity with forgePotential of 0 is infertile, an entity with forgePotential = 1 can forge 1 time an year, an entity with forgePotential = 2 can forge 2 times an year and so on.

The number of times an entity forges is tracked in the forgingCounts mapping. This mapping is incremented for both the forgerTokenId and mergerTokenId everytime a successful forging happens thru forgeWithListed.

The forgingCounts[] of mergerTokenIds is handled correctly. The increment of forgingCounts[] precedes the condition check:

forgingCounts[mergerTokenId]++; require(mergerForgePotential > 0 && forgingCounts[mergerTokenId] <= mergerForgePotential, 'forgePotential insufficient' ); However, Forger Entities forgerTokenId are able to exceed the forging limit due to increment of forgingCounts[] happening after the condition check. The check is made during the listForForging call, but the forgingCounts[] mapping is only incremented during forgeWithListed call. This allows the ‘Forger Entities’ to list one more time than intended.

This results in each Forger entity with non-zero forgingPotential having the ability to forge 2-10 times, instead of 1-9 times intended by the protocol.

## Impact

As long as Forger Entities have a non-zero forgingPotential, the Entity is able to exceed the limit intended by forgingPotential.

## Recommended Mitigation Steps

This can be fixed in different ways. The simplest way is to use the < instead of the <= in the listForForging condition check.

forgingCounts[tokenId] < forgePotential require( forgePotential > 0 && forgingCounts[tokenId] < forgePotential, 'Entity has reached its forging limit' );

## Assessed type

Invalid Validation TForge1 (TraitForge) confirmed

# [M-15] Users’ ability to nuke will be DoSed for three days after putting NFTs up for sale and canceling the sale

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by 0rpse, also found by 0xb0k0, smbv-1923, Abhan, mansa11, and anonymousjoe Users will not be able to nuke their NFTs for three days after canceling sale of their NFTs.

## Recommended Mitigation Steps

Add necessary checks to _beforeTokenTransfer so that:

If a transfer is happening to EntityTrading contract.

If a transfer is happening from EntityTrading to sale’s original listing account lastTokenTransferredTimestamp is not reset.

## Assessed type

DoS Koolex (judge) decreased severity to Low 0rpse (warden) commented:

This issue effectively locks users out of accessing the nuke functionality for three days after canceling a sale, which will be unexpected and detrimental, especially if users intended to use the nuke feature shortly after the cancellation. Given that the rewards are tied to the NFT’s attributes and the state of the nuke fund, a delay could mean the difference between a lucrative transaction and a missed opportunity.

The nuking function is a key part of the utility and value proposition of the NFT. By introducing an unintended delay through the sale cancellation process, the contract effectively diminishes the NFT’s usability and could potentially harm the user’s financial interests. This is not just a minor inconvenience; it can materially impact the user’s ability to interact with their assets in a way that is consistent with their expectations and strategic planning.

Given these factors, I think the severity should be Medium.

Koolex (judge) commented:

This seems to be a design choice. Otherwise, the fix can be exploited to the locking duration.

0rpse (warden) commented:

@Koolex- When I talked to the sponsors about the issue, they answered that this is just something that people will have to accept because there is no way to stop this without ruining the maturity period mitigation, I don’t think it is a design choice but a flaw.

Otherwise, the fix can be exploited to the locking duration.

I don’t understand how the fix can be exploited; lastTokenTransferredTimestamp is not going to be updated only when a transfer is happening to trading contract, or from the trading contract to the original listing account. This is logically the same thing as transferring from address A to address A, which they have done here.

Koolex (judge) commented:

When I talked to the sponsors about the issue, they answered that this is just something that people will have to accept because there is no way to stop this without ruining the maturity period mitigation, I don’t think it is a design choice but a flaw.

Isn’t this a design choice since the trade-off is would be ruining the maturity period mitigation?

Does this mean if the user list and cancel twice, they have to wait 3 days again? The code seems to apply this. I can see your concern here.

0rpse (warden) commented:

Isn’t this a design choice since the trade-off is would be ruining the maturity period mitigation?

No, _beforeTokenTransfer should simply check if a user is canceling his/her sale listing and should not update the timestamp (also when a user is listing), if any other kinds of transfers occur, timestamp update will still take place, there is no ruining of maturity period mitigation.

Does this mean if the user list and cancel twice, they have to wait 3 days again? The code seems to apply this. I can see your concern here.

If I am not understanding this question please clarify. The code as it stands will block a user’s ability to nuke his NFT for three days after canceling his sale. We can easily imagine a scenario where:

Alice puts her NFT for sale.

After some time, Alice notices nuke-fund has gathered a lot of funds and if she nukes her NFT she will make a nice profit.

Alice cancels the sale and tries to nuke, but at this point she will be locked out of the nuke function for 3 days because of the current implementation.

As Alice is logically the owner of her NFT throughout the sale process, these transfers should not block her ability to nuke her own NFT.

Koolex (judge) commented:

My question is, is this repetitive?

Example: Alice puts her NFT for sale, then cancels. Now she has to wait for 3 days, right? Let’s say she waits for 1 days, since nuke is not possible, she lists her NFT and then canceled. Will she wait again for 3 days?

0rpse (warden) commented:

Alice puts her NFT for sale, then cancels. Now she has to wait for 3 days, right?

Yes.

Let’s say she waits for 1 days, since nuke is not possible, she lists here NFT and then canceled. Will she wait again for 3 days?

Yes, timestamp will be updated again, and she has to wait for another 3 days from the moment of cancellation.

TForge1 (TraitForge) commented:

Yeah, I’d say this is an issue. Maybe low as it’s not detrimental, but can impact UX. We can definitely check if its just being canceled by the original owner and make sure it doesn’t lock it for 3 days. Valid, in my opinion.

Koolex (judge) commented:

After careful review, I believe this issue hits the mark of Medium severity.

Reasoning:

The problem persists where canceling an NFT sale extends the lock period, preventing immediate nuking.

After canceling an NFT sale, users face a dilemma: they can’t relist the NFT (as this resets the 3-day waiting period) nor can they nuke it. This restriction is harmful to the user and could negatively impact the game’s economic dynamics.

The frequency of listing and canceling NFTs is likely high due to competitive price adjustments. This issue affects many active participants, as they’re all temporarily barred from nuking their NFTs, amplifying the overall impact.

It appears to have a straightforward resolution that doesn’t conflict with any fundamental game design principles.

# [M-16] There is no slippage check in the nuke() function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

nuke() function Submitted by KupiaSec, also found by burnerelu, gajiknownnothing, Decap, Rhaydden, p0wd3r, 0x3b, samuraii77, peanuts, DemoreX, Abdessamed, desaperh, jesjupyter, franfran20, vinica_boy, 0xvd, amaron, slvDev, Joshuajee, cozymfer, phoenixV110, gkrastenov, ilchovski ( 1, 2 ), 0xrex, Kalogerone, ke1caM, shaflow2, shaka, dimulski, and hl_ NFT owners may receive significantly less nuke funds than they expected.

## Recommended Mitigation Steps

It is recommended to implement a minimum claim amount check in the nuke() function.

TForge1 (TraitForge) confirmed and commented:

Although base-chain has a private mempool, meaning MEV and Front-Running is not possible. There is a possibility of the mempool privacy changing, which will make this a valid issue. But it is impossible, now, judges can decide what they want to do with this. But possibility of mempool going public means this is still a risk.

Koolex (judge) increased severity to High and commented:

Valid issue.

Reasoning:

Even without intentional front-running, transaction ordering can still have impacts similar to front-running.

While users can’t see other pending transactions, market conditions can still change between transaction submission and execution As far as I know, Transactions on Base are generally ordered on a first-come, first-served basis by the sequencer. Since, there is no guarantee that the first submitted TX will be processed first (due to multiple factors including network latency), the risk still exists.

ABAIKUNANBAEV (warden) commented:

I think this should downgraded to medium or QA for the same reasoning as described in Issue #166:

The mempool is private meaning the probability is extremely low.

Front-running issues will always exist and it’s the same reasoning as with the auctions: “If I front-run you, you will have to pay the bigger price”. I guess it’s just the nature of the game.

This certainly has a risk but the problem is that the game allows “the fastest” to win and let’s say the suggested mitigation is introduced and Alice gets minNukeAmount. Her tx will revert multiple times before that due to deviation from minNukeAmount and it’s not likely that she will get a much bigger amount as the competition will be extremely high for such amounts of ETH. There will be just the same situation if she was front-runned with the absence of slippage. The example with 2.5 ETH and 5 ETH seems highly unrealistic, as such amounts from NukeFund in real scenario will be taken almost right away.

Abdessamed (warden) commented:

@ABAIKUNANBAEV - You are mistaking two things:

The slippage here is different from #166. The slippage here can cause significant loss for users (up to 50% of what they expected) as outlined by the PoC: the user received 2.5 ETH instead of 5 ETH, we are talking about a 2.5 ETH difference which is about $6k.

There doesn’t have to be front-running for this issue to happen as explained in another duplicate Issue #753, the warden incorrectly mentioned this in his report.

If Alice submits her transaction before Bob, according to the fastest rule, Alice’s transaction should be executed first. But still, Bob’s transaction can be executed before Alice due to several reasons like network congestion, etc.; something that players can not control.

ABAIKUNANBAEV (warden) commented:

Yes, and as I outlined, such minAmount mitigation for this issue is senseless as, at the start of the game, everybody will try to claim and here “the fastest wins” rule comes into play. The fastest here will get the most amount of money. You make a logical mistake by saying that users here “lose the funds” as they don’t lose anything - they only own a tokenId that makes them eligible to claim at most 50% of the funds. By saying they lose funds - they didn’t lose, they just did not get amount that they could potentially get - big difference. I personally think it’s at most QA.

dimulski (warden) commented:

I disagree that users don’t lose the funds. If they don’t receive what they expect, they have burned their NFT with which later on they can claim what they desire. There is definitely a loss of funds, as you have to pay for the NFT; either mint it, forge it or buy it on an open market.

ABAIKUNANBAEV (warden) commented:

They don’t own any funds to lose them, it’s just opportunity for everybody.

Koolex (judge) decreased severity to Medium and commented:

I believe it is fair to downgrade it to Medium for similar reasons explained here.

# [M-17] Incorrect isApprovedForAll check in the NukeFund.nuke() function

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

isApprovedForAll check in the NukeFund.nuke() function Submitted by KupiaSec, also found by Bauchibred and ZanyBonzy Approved users can’t nuke the owner’s NFT.

## Recommended Mitigation Steps

It is recommended to change the code as follows:

function nuke(uint256 tokenId) public whenNotPaused nonReentrant { require( nftContract.isApprovedOrOwner(msg.sender, tokenId), 'ERC721: caller is not token owner or approved' ); require( nftContract.getApproved(tokenId) == address(this) || - nftContract.isApprovedForAll(msg.sender, address(this)), + nftContract.isApprovedForAll(nftContract.ownerOf(tokenId), address(this)), 'Contract must be approved to transfer the NFT.' ); [...] }

## Assessed type

Access Control KupiaSec (warden) commented:

This report clearly demonstrates the presence of a bug.

Scenario Consider the following scenario:

Alice approved the NukeFund contract as an operator.(Alice called setApprovalForAll(nukeFund, true) ).

Alice is the owner of an entity.

Alice gave permission to Bob for the entity. (Alice called approve(Bob's address, the entity's tokenId) ).

Bob called nuke(the entity's tokenId).

In the scenario described above, it should be allowed for Bob to call nuke(the entity's tokenId). However, Bob’s call will be reverted due to the bug highlighted in the report.

nftContract.getApproved(tokenId) returns Bob, not address(this). Additionally, msg.sender at L160 is Bob who have not approved the NukeFund contract as an operator. Consequently, the nuke() function will be reverted unexpectedly.

So, msg.sender at L160 should be replaced by nftContract.ownerOf(tokenId):.

require ( nftContract.

getApproved ( tokenId ) == address ( this ) || 160:

nftContract.

isApprovedForAll ( msg.

sender, address ( this )), 'Contract must be approved to transfer the NFT.' ); This unexpected reversal also takes place when Alice approves Bob as an operator in step 3 of the scenario. Therefore, I believe this issue is valid.

# [M-18] Excess ETH from forgingFee can get stuck in EntityForging under certain situations

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

forgingFee can get stuck in EntityForging under certain situations Submitted by nnez, also found by Bauchibred, 0x3b, p0wd3r, samuraii77, FastChecker, 0xHash, pep7siup, broccolirob, and dontonka ETH being permanently locked in the contract under certain situations.

Description The forgeWithListed function contains a vulnerability in the fee checking mechanism. The line that checks the forging fee allows users to send more ETH than the required amount. Normally, users would want to send the exact amount of the forging fee to avoid overpayment. However, if the forging fee is somehow reduced between the time a user initiates a transaction and when it’s executed, the excess ETH becomes permanently locked in the contract.

See:

EntityForging.sol#L102-L175:

uint256 forgingFee = _forgerListingInfo.

fee; require ( msg.

value >= forgingFee, 'Insufficient fee for forging' ); //... (later in the function) uint256 devFee = forgingFee / taxCut; uint256 forgerShare = forgingFee - devFee; address payable forgerOwner = payable ( nftContract.

ownerOf ( forgerTokenId )); //... (fee distribution) ( bool success, ) = nukeFundAddress.

call { value:

devFee }( '' ); require ( success, 'Failed to send to NukeFund' ); ( bool success_forge, ) = forgerOwner.

call { value:

forgerShare }( '' ); require ( success_forge, 'Failed to send to Forge Owner' ); The >= check in the require statement allows for overpayment, but the contract only distributes the exact forgingFee amount, leaving excess ETH locked in the contract.

Here is an example of a scenario that could get the fund stuck from allowing overpayment:

Bob lists his forger entity with a fee of 0.1 ETH.

Alice initiates a transaction to forge with Bob’s entity, sending exactly 0.1 ETH.

Alice’s transaction is delayed or stuck due to gas price fluctuations or other network conditions.

Before Alice’s transaction is executed, Bob reduces his forging fee to 0.08 ETH.

Alice’s transaction is finally executed after the fee reduction.

The contract accepts Alice’s 0.1 ETH, sends 0.08 ETH to Bob, and 0.02 ETH remains locked in the contract.

[Bob's Entity (Forger)] | Listing: Fee = 0.1 ETH | Alice sends 0.1 ETH | ------------------------------ | Transaction Delayed | ------------------------------ | Bob reduces fee to 0.08 ETH | ----------------------------- | Alice's Transaction | | Executed After Delay | ----------------------------- | Contract Receives 0.1 ETH | ----------------------------- | Sends 0.08 ETH to Bob | | 0.02 ETH Locked in Contract| ----------------------------- Rationale for Severity The severity is set to Medium because:

It can cause financial losses to users, but only in limited scenarios.

The impact is restricted to the difference between the original and reduced forging fee.

Recommended Mitigations There are two available options:

Only allow paying an exact fee.

Allow overpayment but send excess amount back to msg.sender.

# [M-19] Each generation should have 1 “Golden God” NFT, but there could be 0

- **Contest:** TraitForge
- **Slug:** 2024-07-traitforge
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-traitforge
- **Source snapshot:** competitions/2024-07-traitforge/final_report.html

Submitted by Kalogerone, also found by samuraii77, anonymousjoe, Decap, sl1, Shahil_Hussain, and yotov721

- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntropyGenerator/EntropyGenerator.sol#L206
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/TraitForgeNft/TraitForgeNft.sol#L345
- https://github.com/code-423n4/2024-07-traitforge/blob/main/contracts/EntropyGenerator/EntropyGenerator.sol#L10

## Vulnerability Details

The location of the “Golden God” NFT with entropy of 999999 is calculated by the initializeAlphaIndices function in the EntropyGenerator contract.

function initializeAlphaIndices () public whenNotPaused onlyOwner { uint256 hashValue = uint256 ( keccak256 ( abi.

encodePacked ( blockhash ( block.

number - 1 ), block.

timestamp ))); uint256 slotIndexSelection = ( hashValue % 258 ) + 512; uint256 numberIndexSelection = hashValue % 13; slotIndexSelectionPoint = slotIndexSelection; numberIndexSelectionPoint = numberIndexSelection; } It is confirmed by the devs/sponsors that TraitForgeNft contract is the only allowed caller of the initializeAlphaIndices. As we can see, the function is protected by a modifier and when the generation is getting incremented, TraitForgeNft is the caller.

TraitForgeNft.

sol function _incrementGeneration () private { require ( generationMintCounts [ currentGeneration ] >= maxTokensPerGen, "Generation limit not yet reached" ); currentGeneration ++; generationMintCounts [ currentGeneration ] = 0; priceIncrement = priceIncrement + priceIncrementByGen; @> entropyGenerator.

initializeAlphaIndices (); emit GenerationIncremented ( currentGeneration ); } Each generation is supposed to contain 1 “Golden God” NFT. According to the TraitForge docs:

Entropy is set into uint256 slots, with each slot able to contain a 78 digit number - or 13 concatenated entropies. Thus 770 slots are needed to contain 10,000 entropies.

Entropies are stored in the following array:

uint256 [ 770 ] private entropySlots; // Array to store entropy values We have 770 slots with 13 entropies each, which means 770 * 13 = 10,010 total entropies generated.

The last 10 entropies are skipped when the generation ends, since there is a max mint value of 10,000 NFTs per generation and when it is reached, it increments to the next generation. There is a chance that the 999999 entropy is placed in the last entropy slot, in the last 10 entropies of that slot and is lost and not mintable. Since the generation has incremented by the TraitForgeNft contract, the action is not recoverable.

## Impact

There is a chance that the “Golden God” NFT is completely lost and not mintable for a generation. This goes against the docs and the expectations of the game.

## Recommended Mitigation Steps

Implement a check that ensures that the 999999 entropy is not set out of bounds.

function initializeAlphaIndices() public whenNotPaused onlyOwner { uint256 hashValue = uint256(keccak256(abi.encodePacked(blockhash(block.number - 1), block.timestamp))); uint256 slotIndexSelection = (hashValue % 258) + 512; uint256 numberIndexSelection = hashValue % 13; slotIndexSelectionPoint = slotIndexSelection; numberIndexSelectionPoint = numberIndexSelection; + if (slotIndexSelection == 769 && numberIndexSelection > 3) { + initializeAlphaIndices(); + } }

## Assessed type

Math Kalogerone (warden) commented:

@Koolex - I believe this issue is placed in the wrong group/duplicate of Issue #656. Issue #656 describes how there can be 0 Golden God NFT due to the forging, while my issue doesn’t involve forging at all. My issue describes how there can be 0 Golden God NFT even in the first generation which forging doesn’t affect, simply if the Golden God NFT slot gets placed in the last 10 slots during the entropy generation.

EDIT: My issue can even happen in the 1st generation while Issue #656 can’t.

Koolex (judge) commented:

@Kalogerone - Could you please provide a PoC for this?

Kalogerone (warden) commented:

@Koolex - I have created a foundry test file for my PoC so I could fuzz block numbers, because the location of the Golden God entropy relies on the block number, as shown in the report. I have created a tests.t.sol file inside the test folder.

## Vulnerability Details

section for more detailed information on the reason why.
