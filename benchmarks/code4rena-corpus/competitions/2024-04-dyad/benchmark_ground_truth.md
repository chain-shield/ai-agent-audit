# Benchmark Ground Truth: DYAD

## Accepted H/M Findings

# Accepted H/M Findings: DYAD

# [H-01] Design flaw and mismanagement in vault licensing leads to double counting in collateral ratios and positions collateralized entirely with kerosine

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by Maroutis, also found by falconhoof, pontifex ( 1, 2 ), 0xleadwizard, Hueber, grearlake, josephdara, 0xlemon, gumgumzum, Emmanuel, ych18, Al-Qa-qa, adam-idarrha ( 1, 2 ), CodeWasp, Honour ( 1, 2 ), 0xabhay ( 1, 2 ), oakcobalt ( 1, 2 ), SBSecurity ( 1, 2 ), Infect3d, SpicyMeatball, Limbooo ( 1, 2 ), AM, LeoGold, Daniel526, Giorgio, KupiaSec, Circolors ( 1, 2 ), neocrao, 0xtankr, The-Seraphs, 0xShitgem, poslednaya, ke1caM, ljj ( 1, 2 ), TheSchnilch ( 1, 2 ), bhilare_, n4nika ( 1, 2 ), btk, iamandreiski ( 1, 2, 3 ), VAD37 ( 1, 2 ), 0xnilay, Aamir, carlitox477, n0kto, petro_1912, Abdessamed, itsabinashb, shaflow2, web3km ( 1, 2 ), dimulski, ZanyBonzy, shikhar229169 ( 1, 2 ), T1MOH ( 1, 2 ), zigtur, Topmark, Krace, cinderblock, Egis_Security, AlexCzm ( 1, 2 ), 3docSec, PoeAudits, TheSavageTeddy, kennedy1030, 0x486776, and zhaojohnson ( 1, 2 )

- https://github.com/code-423n4/2024-04-dyad/blob/cd48c684a58158de444b24854ffd8f07d046c31b/script/deploy/Deploy.V2.s.sol#L64-L65
- https://github.com/code-423n4/2024-04-dyad/blob/cd48c684a58158de444b24854ffd8f07d046c31b/script/deploy/Deploy.V2.s.sol#L95-L96

## Impact

Due to a design flaw in the protocol’s management of vault licensing and the deploy script, a significant risk exists where collateral can be counted twice in the calculation of the Collateralization Ratio. This occurs because WETH vaults are incorrectly licensed to both the KeroseneManager and VaultLicenser, allowing users to register the same asset and their NFT ID in both Kerosene vaults and normal vaults. This can allow users to exploit this to greatly inflate their CR calculations, misleading the protocol into considering a position more secure than it truly is, which can prevent necessary liquidations and pose significant risk to the protocol.

However, a user can also register his ID and the keroseneVault as a normal vault because the script calls the licensing function for the kerosineVaults using the VaultLicenser rather than the kerosineManager. This can lead to positions entirely collateralized with kerosene token. Which is not what protocol intends to do and is very risky as the kerosene token is endogenous and has a manipulable asset price.

## Recommended Mitigation Steps

The design of the licensing part needs to be re-thinked. The issue here is that the vaults mapping of the KerosineManager contract which is constructed via the method KerosineManager::add is the same mapping that is used by the UnboundedKerosineVault::assetPrice function. You can consider creating two separate mappings.

One used only for the price calculation in the UnboundedKerosineVaultassetPrice contract which would only include the classic vaults (weth …); another mapping used for the licensing part which would include the kerosene vaults.

Let’s assume these were implemented, we have now two mappings. The DeployV2 should change as follows:

kerosineManager.

addVaultForOracleCalculation ( address ( ethVault )); kerosineManager.

addVaultForOracleCalculation ( address ( wstEth )); kerosineManager.

add ( address ( unboundedKerosineVault )); kerosineManager.

add ( address ( boundedKerosineVault )); Assuming the addVaultForOracleCalculation feeds a mapping that will be used by UnboundedKerosineVault::assetPrice while add doesn’t.

shafu0x (DYAD) acknowledged Note: For full discussion, see here.

# [H-02] Inability to perform partial liquidations allows huge positions to accrue bad debt in the system

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by MrPotatoMagic, also found by Maroutis, peanuts, ArmedGoose, d3e4, OMEN, NentoR, 0xtankr, SpicyMeatball, KYP, Shubham, dimulski, Giorgio, Sabit, Egis_Security, and T1MOH The liquidate() function allows liquidators to burn DYAD on behalf of an DNft id and receive collateral in return.

The issue is that the current functionality only allows burning of the whole DYAD amount minted by the DNft id. This means that partial liquidations cannot be performed and prevents liquidators from liquidating DYAD minted by whales that hold huge positions in the system. Since the liquidations cannot be performed unless the liquidator can match up to the collateral deposited and DYAD minted by the whale, the system will be undercollaterized causing bad debt to accrue.

The effect of this issue will increase as more such positions exist in the system that cannot be liquidated by the liquidators.

## Recommended Mitigation Steps

Implement a mechanism to allow liquidators to partially liquidate positions. This would also require refactoring the collateral paid out to them based on the amount they cover.

shafu0x (DYAD) commented:

Hmm, but can’t this be solved by flash loaning DYAD?

0xMax1 (DYAD) commented:

Not if loan exceeds market liquidity. Partial liquidations is a feature we should implement.

shafu0x (DYAD) confirmed

# [H-03] Attacker can make 0 value deposit() calls to deny user from redeeming or withdrawing collateral

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

0 value deposit() calls to deny user from redeeming or withdrawing collateral Submitted by MrPotatoMagic, also found by 0xAkira, SBSecurity, 0xblack_bird ( 1, 2 ), dinkras, xyz, falconhoof, Tychai0s, josephdara, 0x175, DedOhWale, 0xloscar01, 0xlemon, djxploit, turvy_fuzz, kartik_giri_47538, sashik_eth, Honour, Pechenite, koo, grearlake, NentoR, Dinesh11G, Dots, KupiaSec, imare, Circolors, pep7siup, Mrxstrange, web3km, forgebyola, alix40, 0xtankr, 0xDemon, Ryonen, Imp, ke1caM, poslednaya, itsabinashb, Cryptor, asui, steadyman, DPS, VAD37, ljj, btk, TheFabled, c0pp3rscr3w3r, niser93, DMoore, d_tony7470, blutorque, 0x77, adam-idarrha, ZanyBonzy, Vasquez, Angry_Mustache_Man, Jorgect, shaflow2, valentin_s2304, zigtur, Sabit, 0xAsen, kennedy1030, caglankaan, GalloDaSballo, Giorgio, dimulski, T1MOH, 3docSec, AlexCzm, 0xabhay, 4rdiii, PoeAudits, WildSniper, ptsanev, BiasedMerc, y4y, TheSavageTeddy, carrotsmuggler, Abdessamed, ubl4nk, zhaojohnson, 0x486776, lionking927, and Krace Function deposit() uses modifier isValidDNft() instead of isDNftOwner(), which allows anyone to call deposit() on behalf of any DNft id.

## Impact

Attacker can make user devoid of withdrawing or redeeming collateral by making 0 value deposit() calls. Other than denying users from temporarily withdrawing their collateral, this is also an issue since it could force users into liquidations when they try to take preventative measures on their collateral ratio (especially through redeemDyad() ) in high collateral price volatility situations. If successful, the attacker could then perform the liquidation to profit from the situation.

Attacker can make user devoid of removing vault due to id2asset > 0 by depositing 1 wei of collateral.

## Recommended Mitigation Steps

Use modifier isDNftOwner() instead of isValidDNft() on function deposit().

## Assessed type

Invalid Validation shafu0x (DYAD) confirmed and commented via duplicate Issue #489:

Good find! We should restrict it to only owner.

# [H-04] Attacker can frontrun user’s withdrawals to make them revert without costs

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by Limbooo, also found by ahmedaghadi, pontifex, Evo, MiniGlome, favelanky, Infect3d, ArmedGoose, AM, SpicyMeatball, 0xleadwizard, HChang26, TheSchnilch, Jorgect, and 0xabhay User’s withdrawals will be prevented from success and an attacker can keep it up without a cost by using a fake vault and a fake token.

## Recommended Mitigation

Consider limiting anyone with any token vaults to update idToBlockOfLastDeposit. One of these mitigations can be used:

Prevent anyone to deposit to unowned dNft token.

Allow to only depositing using licensed vaults, so if the attacker try to front-runs he will lose some real tokens.

Since this used to protect against flash loans, no need to use it with all token vaults. This should be used only with vaults that can be used to mint DYAD. So, we can check if the deposit included in the vaultLicenser and keroseneManager licenser, we need to update the idToBlockOfLastDeposit. Here is a git diff for this fix:

diff --git a/src/core/VaultManagerV2.sol b/src/core/VaultManagerV2.sol index fc574a8..73dbb6b 100644 --- a/src/core/VaultManagerV2.sol +++ b/src/core/VaultManagerV2.sol @@ -124,7 +124,8 @@ contract VaultManagerV2 is IVaultManager, Initializable { external isValidDNft(id) { - idToBlockOfLastDeposit[id] = block.number; + if (vaultLicenser.isLicensed(vault) || keroseneManager.isLicensed(vault)) + idToBlockOfLastDeposit[id] = block.number; Vault _vault = Vault(vault); _vault.asset().safeTransferFrom(msg.sender, address(vault), amount); _vault.deposit(id, amount);

## Assessed type

Invalid Validation Koolex (judge) increased severity to High shafu0x (DYAD) confirmed

# [H-05] Unable to withdraw Kerosene from vaultmanagerv2::withdraw as it expects a vault.oracle() method which is missing in Kerosene vaults

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

vaultmanagerv2::withdraw as it expects a vault.oracle() method which is missing in Kerosene vaults Submitted by Circolors, also found by dinkras, ahmedaghadi, 0x175, Evo, 0xfox, d3e4, Al-Qa-qa, 0xlemon, Mahmud, Honour, sashik_eth, SBSecurity, amaron, TheSchnilch, Infect3d, ducanh2706, Limbooo, 3th, 0xShitgem, ke1caM, ljj, bhilare_, iamandreiski, Josh4324 ( 1, 2 ), 0xSecuri, bbl4de, Aamir, btk, alix40, 0xnilay, steadyman, shaflow2, cinderblock, AlexCzm, y4y, Egis_Security, web3km, 0xAlix2 ( 1, 2 ), 0x486776, itsabinashb, carrotsmuggler, dimulski, and 4rdiii VaultManagerV2 has one withdraw function responsible for withdrawing both exogenous collateral (weth/wsteth) and endogenous collateral (Kerosene). However, the function expects the

vault passed as an argument to have an oracle method. This is the case for Vault contracts, but not the case for the BoundedKerosineVault or UnboundedKerosineVault contracts. This means that whenever a user attempts to withdraw Kerosene deposited into the contract the call will revert, meaning the Kerosene remains stuck in the contract permanently.

## Recommended Mitigation

Given that the value of exogenous and endogenous collateral is calculated differently it is necessary to handle withdrawal of exogenous collateral and Kerosene differently. It would avoid added complexity to the function logic to have two different withdraw and withdrawKerosene functions.

shafu0x (DYAD) confirmed and commented:

Good find. This is correct.

Note: For full discussion, see here.

# [H-06] User can get their Kerosene stuck because of an invalid check on withdraw

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by 0xAlix2, also found by Aamir, oakcobalt ( 1, 2, 3 ), MrPotatoMagic ( 1, 2 ), Evo, favelanky, 0xlemon, 0xfox, TheSchnilch, Honour, SpicyMeatball, koo, Limbooo, KupiaSec, alix40, forgebyola, ke1caM, Jorgect, bhilare_, Dudex_2004, 0xnev, petro_1912, FastChecker, Abdessamed, shikhar229169, Egis_Security, kennedy1030, and 3docSec The protocol allows users to deposit both Kerosene and non-Kerosene collateral, to mint Dyad users should have an equal value of non-Kerosene (exogenous) collateral. So users should have 100% non-Kerosene and the rest could be Kerosene collateral.

However, in VaultManagerV2::withdraw, the protocol allows users to withdraw Kerosene and non-Kerosene collateral, by just passing the corresponding vault. When withdrawing it checks if the (non-Kerosene value - withdraw value) is less than the minted Dyad, if so it reverts. This is also checked when withdrawing Kerosene collateral, which is wrong as it’s comparing non-Kerosene value with Kerosene value.

Ultimately, this blocks users from withdrawing their Kerosene collateral, even if they should be able to. Let’s take an example, a user has $100 non-Kerosene and $100 Kerosene collateral, and you have 100 Dyad minted, that’s a 200% ratio. If he tries to withdraw $1 Kerosene, the TX will revert, because getNonKeroseneValue(id) = 100 - value = 1 < Dyad minted = 100, which again is a wrong check.

## Recommended Mitigation Steps

Differentiate between Kerosene and non-Kerosene USD values when withdrawing either of them.

## Assessed type

Invalid Validation shafu0x (DYAD) confirmed and commented:

This is correct. Good find!

# [H-07] Missing enough exogenous collateral check in VaultManagerV2::liquidate makes the liquidation revert even if (DYAD Minted > Non Kerosene Value)

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

VaultManagerV2::liquidate makes the liquidation revert even if (DYAD Minted > Non Kerosene Value) Submitted by shikhar229169, also found by Honour, Circolors, Maroutis, KupiaSec, 3th, ke1caM, 0xfox, 0xSecuri, Sancybars, Strausses, kennedy1030, Stormreckson, and 0x486776 The vulnerability is present in the VaultManagerV2::liquidate function where it doesn’t have any check for whether the vault has enough exogenous collateral leading to no liquidation even if the non-kerosene value is less than DYAD minted.

It is intended that the position of user’s DNFT has enough exogenous collateral and should be 150% overcollateralized. But there will arise a case when a position is no doubt 150% collateralized by the support of kerosene value, but the non-kerosene value is reduced below the DYAD token minted. As a result of which the vault doesn’t have enough required exogenous collateral. But due to the missing check for enough non-kerosene value collateral in liquidate function, the liquidation will never happen and reverts.

## Impact

The position will never be liquidated, for the above discussed case. As a result of which the (DYAD Minted > Non Kerosene Value), making the value of DYAD to fall.

## Recommended Mitigation Steps

Update the line 214 in VaultManagerV2.sol:

- if (cr >= MIN_COLLATERIZATION_RATIO) revert CrTooHigh(); + if (cr >= MIN_COLLATERIZATION_RATIO && getNonKeroseneValue(id) >= dyad.mintedDyad(address(this), id)) revert CrTooHigh();

## Assessed type

Context 0xMax1 (DYAD) commented:

Kerosene should be included in the transfer upon liquidation.

shafu0x (DYAD) confirmed Koolex (judge) commented:

@shafu0x - Could you please help with the severity assessment here?

The impact is obviously high, However, I’m not sure about the likelihood.

Here is an example, the user added extra kerosene (i.e. above 50%):

A user added 110% value of non-kerosene + 60% value of kerosene => CR 170%.

Non-kerosene value drops below 100%, let’s say 90%.

Now, we have 90% value of non-kerosene + 60% value of kerosene => CR 150%.

Liquidation is reverting.

Another scenario, would be that the kerosene added by the user is equal or less than 50%, and the price of kerosene goes up enough to cover 150% as CR.

So, likelihood depends on two events:

Amount of kerosene has to be above 50%.

Price of kerosene going up.

Given the info above, would you say the likelihood is low, medium or high?

shafu0x (DYAD) commented:

As @0xMax1 mentioned, we fix this by also moving kerosene in the case of a liquidation. I think this should fix it.

Koolex (judge) commented:

The liquidation will revert if CR >= 1.5 but the non-kerosene collateral value is less than 100%. So, minted dyad is not backed by (at least) 100% of non-kerosene collateral.

After discussing the likelihood of this with the sponsor, I believe the issue stands as high.

adam-idarrha (warden) commented:

This issue should be invalid. The issue is about the state where positions have exogenous collateral less than the DYAD minted, suggesting such positions should be liquidated. However, the protocol specifications do not mandate liquidation under these circumstances if the position remains overall sufficiently collateralized with kerosene.

T1MOH (warden) commented:

Protocol docs state:

If a Note’s collateral value in USD drops below 150% of its DYAD minted balance, it faces liquidation.

There is no mention that position is subject to liquidation when exogenous collateral < DYAD minted. Yes there is that check in withdraw() and mintDyad(), but from reviewer’s perspective it is no more than sanity check. Because bypassing this check as it is doesn’t harm protocol due to the nature of Kerosine price (it has valuation of surplus collateral). That’s because if someone has say 90% of exogenous collateral and 60% of Kerosine, then somebody else has more than 100% in exogenous collateral and protocol is totally fine. As noted before, Kerosine has value only if there is surplus collateral in protocol. I think submission is invalid.

McToady (warden) commented:

The core invariant of the protocol is TVL (which is measured as the non Kerosene collateral) > DYAD total supply Positions being able to fall below a 1:1 exogenous collateral to dyad minted ratio allows for the protocol to break it’s core invariant meaning this is a valid issue regardless of the sponsors intentions.

As pointed out in issue #1027, as the value of the collateral in the protocol is all significantly correlated (to the price of ETH), a significant drop in ETH price can cause a large % of open positions to drop below this 1:1 ratio; meaning it becomes very likely this core invariant is broken.

Abdessamed (warden) commented:

The confusion around this issue is that it didn’t succeed in combining two findings. There are two invariants related to minting/liquidating DYAD:

When minting DYAD, the ratio 1:1 of exogenous value with dyad minted should hold.

When liquidating DYAD, the 150% ratio of USD collateral value (not exogenous collaterals only) should hold.

This issue does not highlight the first invariant. It only talks about liquidation and it assumes that if the USD value of exogenous collateral drops below a 1:1 ratio, liquidation should happen, while this is a wrong assumption.

Liquidation should happen if the total collateral USD value (including both exogenous and kerosene tokens) drops below 150% However, the first invariant (when minting DYAD, a 1:1 ratio should hold) is a valid finding as it demonstrates a core invariant break, but it is not highlighted in this issue.

McToady (warden) commented:

The core invariant of the whole protocol is that total TVL does not drop below total Dyad minted. Being able to liquidate positions that have fallen below the 1:1 ratio is merely a mitigation to protect the protocol from this happening.

Given there is no option to post collateral in non-ETH sources (other stable coins), if ETH price were to drop & TVL to fall below DYAD minted it would lead to the stablecoin to depegging as it would no longer be backed 1:1. Being able to liquidate positions with less than a 1:1 ratio is more a mitigation to protect the health of the entire protocol rather than a bug in itself.

Koolex (judge) commented:

This issue stays as a valid high since the core invariant in the protocol can be broken leading to DYAD’s depeg.

Note: For full discussion, see here.

# [H-08] Users can get their Kerosene stuck until TVL becomes greater than Dyad’s supply

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by 0xAlix2, also found by NentoR, Abdessamed ( 1, 2 ), DarkTower, 0xlucky, CodeWasp, sashik_eth ( 1, 2, 3 ), Egis_Security, Maroutis, TheFabled, 0xabhay ( 1, 2 ), itsabinashb ( 1, 2 ), Infect3d, windhustler, btk, Limbooo, KupiaSec, SpicyMeatball ( 1, 2 ), imare, Circolors, gumgumzum, web3km, n4nika, 0xtankr, cu5t0mpeo, Ryonen, ke1caM, oakcobalt, TheSchnilch, XDZIBECX, steadyman, VAD37, shaflow2, lian886, iamandreiski, dimulski, 0x486776 ( 1, 2 ), Giorgio, T1MOH, kennedy1030, TheSavageTeddy, carrotsmuggler, zhaojohnson ( 1, 2 ), and Krace The protocol expects users to migrate their collateral from V1 vaults to V2 vaults, this significantly increases the TVL of the protocol’s V2. At the same time, the Kerosene price depends on the TVL, in

UnboundedKerosineVault::assetPrice the numerator of the equation is:

uint256 numerator = tvl - dyad.totalSupply(); This will always revert until the TVL becomes > Dyad’s supply, which is around 600k. So when users deposit Kerosene in either Kerosene vaults their Kerosene will temporarily get stuck in there.

## Recommended Mitigation Steps

This is a bit tricky, but I think the most straightforward and logical solution would be to block the usage of the Kerosene vaults (just keep them unlicensed) until enough users migrate their positions from V1, i.e. the TVL reaches the Dyad’s total supply.

## Assessed type

Under/Overflow shafu0x (DYAD) confirmed and commented:

Yes, it should only check for dyad minted from v1.

# [H-09] Kerosene collateral is not being moved on liquidation, exposing liquidators to loss

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-09
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by 0xAlix2, also found by falconhoof, 0x175, pontifex, DedOhWale, Emmanuel, Honour, Myrault, vahdrak1, SBSecurity, sashik_eth, koo, Vasquez, miaowu, Giorgio, Maroutis, Stefanov, KupiaSec, Aamir, Circolors, 3th, ducanh2706, Jorgect, ke1caM, ljj, VAD37, 0xnev, shikhar229169, lian886, adam-idarrha, iamandreiski, alix40, Angry_Mustache_Man, AlexCzm, kennedy1030, 3docSec, Abdessamed, 0x486776, and T1MOH When a position’s collateral ratio drops below 150%, it is subject to liquidation. Upon liquidation, the liquidator burns a quantity of DYAD equal to the target Note’s DYAD minted balance, and in return receives an equivalent value plus a 20% bonus of the liquidated position’s collateral. If the collateral ratio is

< 100%, all the position’s collateral should be moved to the liquidator, this logic is done in VaultManagerV2::liquidate.

However, that function is only moving the non-Kerosene collateral to the liquidator, which is wrong. All collateral including Kerosene should be moved to the liquidator in the case of full liquidation.

This will affect both the liquidated and liquidator positions:

Liquidator position will be exposed to loss, as he’ll pay some Dyad and won’t get enough collateral in return.

Liquidated position will end up with some collateral after being fully liquidated, where it should end up with 0 collateral of both types.

## Recommended Mitigation Steps

Add the following to VaultManagerV2::liquidate:

uint256 numberOfKeroseneVaults = vaultsKerosene[id].length(); for (uint256 i = 0; i < numberOfKeroseneVaults; i++) { Vault vault = Vault(vaultsKerosene[id].at(i)); uint256 collateral = vault.id2asset(id).mulWadUp(liquidationAssetShare); vault.move(id, to, collateral); }

## Assessed type

Error shafu0x (DYAD) confirmed

# [H-10] Flash loan protection mechanism can be bypassed via self-liquidations

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** H-10
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by carrotsmuggler, also found by ZanyBonzy, TheSavageTeddy, adam-idarrha, Emmanuel, Al-Qa-qa, alix40, TheFabled, and lian886

- https://github.com/code-423n4/2024-04-dyad/blob/44becc2f09c3a75bd548d5ec756a8e88a345e826/src/core/Vault.kerosine.sol#L47-L59
- https://github.com/code-423n4/2024-04-dyad/blob/44becc2f09c3a75bd548d5ec756a8e88a345e826/src/core/VaultManagerV2.sol#L225-L226

## Impact

The protocol implements a flash-loan manipulation protection mechanism with the idToBlockOfLastDeposit variable. This values is set to the current block number during a deposit, and is checked during a withdrawal. If the system detects a deposit and withdrawal in the same block, the system reverts the transaction.

//function deposit idToBlockOfLastDeposit [ id ] = block.

number; //function withdraw if ( idToBlockOfLastDeposit [ id ] == block.

number ) revert DepositedInSameBlock (); The issue is that there is another way to move funds around: liquidations. This calls the move function to transfer around the balances, and does not update the idToBlockOfLastDeposit of the receiving account.

function liquidate ( uint id, uint to ) { //...

vault.

move ( id, to, collateral ); //...

} So, a user can:

Take out a flashloan. Deposit funds into a vault A. Mint dyad.

Manipulate the price of kerosene to trigger a liquidation.

Liquidate themselves and send their collateral to vault B.

Withdraw from vault B in the same block.

Pay off their flashloans.

The step 2 involves manipulating the price of kerosene, which affects their collateralization ratio. This has been discussed in a separate issue, and mainly states that if the user mints more dyad against free collateral in the system, or if any user takes out free collateral in the system, the price of kerosene will fall.

The flaw being discussed in this report is that the flash loan protection mechanism can be bypassed. This is different from the price manipulation issue and is thus a separate issue. Since this bypasses one of the primary safeguards in the system, this is a high severity issue.

## Recommended Mitigation Steps

The flashloan protection can be bypassed. MEV liquidation bots rely on flashloans to carryout liquidations, so there isn’t a very good way to prevent this attack vector. Making the price of kerosene less manipulatable is a good way to lower this attack chance. However, the system will still be open to flashloan deposits via liquidations.

Incorporating a mint fee will also help mitigate this vector, since the attacker will have a higher cost to manipulate the system.

shafu0x (DYAD) confirmed Koolex (judge) decreased severity to Medium and commented:

@carrotsmuggler - the attack assumes that kerosine is used within the CR. Could you please clarify how the attacker would acquire this big amount of kerosine?

carrotsmuggler (warden) commented:

@Koolex - kerosene can just be bought off of DEXs and other open markets. In this attack, kerosene is not being flashloaned. Kerosene is just required as an initial investment. USDC is flashloaned, and dyad tokens are minted against that up to a CR of 1.5, which drops to 1.0 once the value of kerosene drops.

The point of the issue is to show that the flashloan protection can be bypassed, which is being done here by utilising multiple accounts.

Koolex (judge) commented:

@carrotsmuggler - I’m requesting a PoC (with code) in order to be able to evaluate the severity better. At the moment, the attack is too expensive since the attacker should hold a big amount of Kerosene, which practically difficult since Kerosene is being distributed over 10 years. Unless there is a demonstrated impact on the protocol, this would be a QA.

carrotsmuggler (warden) commented:

The problem this issue addresses, is that the flash loan protection can be bypassed. For that, a POC is taken from the issue #537 showing that self liquidation can be used to flash funds, manipulate the system, and then take them out in the same transaction.

However, the main point of contention here seems to be the impact. Flashloans in general don’t do anything a well funded attacker cannot do on their own, and not an exploit on their own. However, they can be used to exacerbate an existing problem by anyone, well funded or not.

To eradicate this vector, and to make the system less manipulatable, the devs had put in certain restrictions. This issue shows that these restrictions are insufficient. So users can use flashloans and thus a near infinite amount of funds to manipulate the system. This was reported since the devs had explicitly put up a counter to this.

Since this breaks the safeguards put in place by the devs and makes the system more easily manipulatable, I believe this is of medium severity. This can be used in #67, but should not be a duplicate. This can also be abused to mint positions at 100% CR instead of 150% with a very large volume by anyone due to the flashloans, which makes the system way more unstable. Even small changes in price at that condition will be enough to cause bad debt to the system then.

// SPDX-License-Identifier: MIT pragma solidity = 0.8.

17; import "forge-std/Test.sol"; import "forge-std/console.sol"; import { DeployBase, Contracts } from "../script/deploy/DeployBase.s.sol"; import { Parameters } from "../src/params/Parameters.sol"; import { DNft } from "../src/core/DNft.sol"; import { Dyad } from "../src/core/Dyad.sol"; import { Licenser } from "../src/core/Licenser.sol"; import { VaultManagerV2 } from "../src/core/VaultManagerV2.sol"; import { Vault } from "../src/core/Vault.sol"; import { OracleMock } from "./OracleMock.sol"; import { ERC20Mock } from "./ERC20Mock.sol"; import { IAggregatorV3 } from "../src/interfaces/IAggregatorV3.sol"; import { ERC20 } from "@solmate/src/tokens/ERC20.sol"; import { KerosineManager

} from "../src/core/KerosineManager.sol"; import { UnboundedKerosineVault } from "../src/core/Vault.kerosine.unbounded.sol"; import { BoundedKerosineVault } from "../src/core/Vault.kerosine.bounded.sol"; import { Kerosine } from "../src/staking/Kerosine.sol"; import { KerosineDenominator } from "../src/staking/KerosineDenominator.sol"; contract VaultManagerV2Test is Test, Parameters { DNft dNft; Licenser vaultManagerLicenser; Licenser vaultLicenser; Dyad dyad; VaultManagerV2 vaultManagerV2; // weth Vault wethVault; ERC20Mock weth; OracleMock wethOracle; //Kerosine Kerosine kerosine; UnboundedKerosineVault unboundedKerosineVault; KerosineManager kerosineManager; KerosineDenominator

kerosineDenominator; //users address user1; address user2; function setUp () public { dNft = new DNft (); weth = new ERC20Mock ( "WETH-TEST", "WETHT" ); wethOracle = new OracleMock ( 3000e8 ); vaultManagerLicenser = new Licenser (); vaultLicenser = new Licenser (); dyad = new Dyad ( vaultManagerLicenser ); //vault Manager V2 vaultManagerV2 = new VaultManagerV2 ( dNft, dyad, vaultLicenser ); //vault wethVault = new Vault ( vaultManagerV2, ERC20 ( address ( weth )), IAggregatorV3 ( address ( wethOracle )) ); //kerosineManager kerosineManager = new KerosineManager (); kerosineManager.

add ( address ( wethVault )); vaultManagerV2.

setKeroseneManager ( kerosineManager ); //kerosine token kerosine = new Kerosine (); //Unbounded KerosineVault unboundedKerosineVault = new UnboundedKerosineVault ( vaultManagerV2, kerosine, dyad, kerosineManager ); //kerosineDenominator kerosineDenominator = new KerosineDenominator ( kerosine ); unboundedKerosineVault.

setDenominator ( kerosineDenominator ); //Licenser add vault vaultLicenser.

add ( address ( wethVault )); vaultLicenser.

add ( address ( unboundedKerosineVault )); //vaultManagerLicenser add manager vaultManagerLicenser.

add ( address ( vaultManagerV2 )); } function testFlashLoanAttackUsingLiquidateSimulation () public { wethOracle.

setPrice ( 1000e8 ); //1 The attacker prepares two NFTs,some collateral and some Kerosene Token.

uint id = mintDNft (); uint id_for_liquidator = mintDNft (); weth.

mint ( address ( this ), 1e18 ); //2 deposit all the non-Kerosene collateral in the vault with One NFT like id=1 vaultManagerV2.

add ( id_for_liquidator, address ( wethVault )); weth.

approve ( address ( vaultManagerV2 ), 1e18 ); vaultManagerV2.

deposit ( id_for_liquidator, address ( wethVault ), 1e18 ); //3 In the next blocknumber, flashloan non-Kerosene collateral from Lending like Aave, vm.

roll ( block.

number + 1 ); weth.

mint ( address ( this ), 1e18 ); //Simulation borrow 1 weth //deposit all the borrowed flashloan non-Kerosene collateral and Kerosene Token in the vault with One NFT like id=0 vaultManagerV2.

add ( id, address ( wethVault )); weth.

approve ( address ( vaultManagerV2 ), 1e18 ); vaultManagerV2.

deposit ( id, address ( wethVault ), 1e18 ); vaultManagerV2.

addKerosene ( id, address ( unboundedKerosineVault )); kerosine.

approve ( address ( vaultManagerV2 ), 1000_000_000e18 ); vaultManagerV2.

deposit ( id, address ( unboundedKerosineVault ), 1000_000_000e18 ); //Mint the max number Dyad you can vaultManagerV2.

mintDyad ( id, 1000e18, address ( this )); uint256 cr = vaultManagerV2.

collatRatio ( id ); //2e18 assertEq ( cr, 2e18 ); //withdraw using id_for_liquidator, manipulate the Kerosene price vaultManagerV2.

withdraw ( id_for_liquidator, address ( wethVault ), 1e18, address ( this )); cr = vaultManagerV2.

collatRatio ( id ); //1e18 assertEq ( cr, 1e18 ); //liquidate vaultManagerV2.

liquidate ( id, id_for_liquidator ); //withdraw the vault which is move from id using id_for_liquidator vaultManagerV2.

withdraw ( id_for_liquidator, address ( wethVault ), 1e18, address ( this )); console.

log ( "weth balance is ", weth.

balanceOf ( address ( this ))/ 1e18 ); } function mintDNft () public returns ( uint ) { return dNft.

mintNft {value:

1 ether }( address ( this )); } function deposit ( ERC20Mock token, uint id, address vault, uint amount ) public { vaultManagerV2.

add ( id, vault ); token.

mint ( address ( this ), amount ); token.

approve ( address ( vaultManagerV2 ), amount ); vaultManagerV2.

deposit ( id, address ( vault ), amount ); } receive () external payable {} function onERC721Received ( address, address, uint256, bytes calldata ) external pure returns ( bytes4 ) { return 0x150b7a02; } adam-idarrha (warden) commented:

For the impact the sponsors stated quite clearly from the DYAD code4rena audit page that the main point of migrating from vaultManagerV1 to V2 is the need for a flashloan protection mechanism, and the impact of the bypass is the ability to manipulate kerosene price which could lead to mass liquidations as discussed in separate issues:

Attack ideas (where to focus for bugs).

Manipulation of Kerosene Price.

Flash Loan attacks.

Migration.

The goal is to migrate from VaultManager to VaultManagerV2. The main reason is the need for a flash loan protection which makes it harder to manipulate the deterministic Kerosene price.

Koolex (judge) increased severity to High and commented:

After reading all the comments above, I believe this should be a valid high due to the following reasons:

Flash loan protection can be bypassed, since this didn’t exist in V1, it seems to me, it is a major change in V2.

Price manipulation impact is demonstrated above, which is caused by utilising Flash loans.

Flash loan attacks mentioned under Attack ideas of the audit page, obviously, the sponsor is interested in breaking this validation put in place.

Not a dup of #67 67’s attack is less accessible unlike with Flash loans where anyone can perform it.

Furthermore, 67 isn’t necessarily to be performed as an attack, the event could occur naturally when whales intend to withdraw funds.

Note: For full discussion, see here.

Medium Risk Findings (9)

# [M-01] Liquidation bonus logic is wrong

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by SBSecurity, also found by peanuts, Emmanuel, carlitox477, Stefanov, AlexCzm, carrotsmuggler, d3e4, and grearlake When a liquidator liquidates a user, he will pay his debt and must receive the debt + 20% bonus in form of collateral (from the user). But now the 20% bonus is based on the user’s ( collateral - debt ), which removes the entire incentive for liquidation.

## Recommended Mitigation Steps

The bonus should be based on the burned user debt and then must send the liquidator the percentage of liquidated user collateral equal to the burned debt + 20% bonus.

This is an example implementation, which gives the desired 20% bonus from the right amount, but need to be tested for further issues.

function liquidate( uint id, uint to ) external isValidDNft(id) isValidDNft(to) { uint cr = collatRatio(id); + uint userCollateral = getTotalUsdValue(id); if (cr >= MIN_COLLATERIZATION_RATIO) revert CrTooHigh(); dyad.burn(id, msg.sender, dyad.mintedDyad(address(this), id)); - uint cappedCr = cr < 1e18 ? 1e18: cr; + uint liquidationEquityShare = 0; + uint liquidationAssetShare = 1e18; + if (cr >= 1.2e18) { + liquidationEquityShare = (dyad.mintedDyad(address(this), id)).mulWadDown(LIQUIDATION_REWARD); + liquidationAssetShare = (dyad.mintedDyad(address(this), id) + liquidationEquityShare).divWadDown(userCollateral); + } - uint liquidationEquityShare = (cappedCr - 1e18).mulWadDown(LIQUIDATION_REWARD);

- uint liquidationAssetShare = (liquidationEquityShare + 1e18).divWadDown(cappedCr); uint numberOfVaults = vaults[id].length(); for (uint i = 0; i < numberOfVaults; i++) { Vault vault = Vault(vaults[id].at(i)); uint collateral = vault.id2asset(id).mulWadUp(liquidationAssetShare); vault.move(id, to, collateral); } emit Liquidate(id, msg.sender, to); }

## Assessed type

Math shafu0x (DYAD) acknowledged and commented via duplicate Issue #75:

Technically true, but we are not gonna change that.

Koolex (judge) decreased severity to Medium

# [M-02] No incentive to liquidate when CR <= 1 as asset received < dyad burned

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

CR <= 1 as asset received < dyad burned Submitted by Infect3d, also found by ZanyBonzy, ArmedGoose, 0xleadwizard, peanuts, Myrault, vahdrak1, jesjupyter, SBSecurity, miaowu, OMEN, ke1caM, Bauchibred, Abdessamed, 0xnilay, HChang26, Bigsam, iamandreiski, 0xAlix2, alix40, atoko, GalloDaSballo, T1MOH, and 0x486776 Right now there are no incentives to liquidate a position with a CR<1, as the liquidator will have to burn the full borrowed amount, and will get the full collateral.

But a CR<1 means collateral is worth less than borrowed amount. So this is a clear loss for the liquidator, meaning no one will liquidate the position.

File:

src / core / VaultManagerV2.

sol 205:

function liquidate ( 206:

uint id, //The ID of the dNFT to be liquidated.

207:

uint to //The address where the collateral will be sent 208: )...:

//... some code...

215:❌ dyad.

burn ( id, msg.sender, dyad.mintedDyad( address ( this ), id )); //<@audit: caller need to burn full borrowed amount 216:

217:

uint cappedCr = cr < 1e18 ?

1e18:

cr; /// == max(1e18, cr) 218:

uint liquidationEquityShare = ( cappedCr - 1e18 ).

mulWadDown ( LIQUIDATION_REWARD ); /// if cr<1, this is equal 0 219:

uint liquidationAssetShare = ( liquidationEquityShare + 1e18 ).

divWadDown ( cappedCr ); /// if cr<1, this is equal 1e18 220:

221:

uint numberOfVaults = vaults [ id ].

length (); 222:

for ( uint i = 0; i < numberOfVaults; i ++) { 223:

Vault vault = Vault ( vaults [ id ].

at ( i )); 224:

uint collateral = vault.

id2asset ( id ).

mulWadUp ( liquidationAssetShare ); 225:

vault.

move ( id, to, collateral ); 226: } 227:

emit Liquidate ( id, msg.

sender, to ); 228: }

## Impact

If CR<1, position will not be liquidated, protocol possibly incurring worse bad debt than this could have been.

## Recommended Mitigation Steps

Refactor the liquidation calculation, to allow liquidator to repay the debt and still get a reward out of this.

## Assessed type

Context shafu0x (DYAD) disputed and commented:

If the CR < 100, where should the reward come from?

Koolex (judge) decreased severity to Low and commented:

Looks like a systematic risk. If CR drops below 100 before it gets liquidated, there is a much bigger problem.

Infect3d (warden) commented:

I don’t think this is an unsolvable systemic issue, and this would be a mistake to not implement mechanism to take care underwater loans.

This issue, and the remediations are well established in CDP protocols:

If a loan CR is <100%, then it must be cut down as soon as possible to limit the even bigger potential losses.

To do so, the operation must be profitable for the liquidators so that they act quickly (which is not the case right now) This will indeed create a bad debt into to protocol, but here’s come the remediation.

That’s why protocols like MakerDAO or other CDP build an “insurance fund” (usually through fees, that can come from liquiations) which role is to absorb bad debt in such events.

shafu0x (DYAD) commented:

We will not implement something like an “insurance fund”. Do you have another mitigation option?

Infect3d (warden) commented:

@shafu0x - I believe the other possibilities would be based on a debt socialization:

Allow liquidator to burn only the DYAD equivalent of collateral.

Award Kerosene from the 1B total supply as a bonus to cover missing part of collateral.

But I feel like a fund/balance, generated by fees taken from “good” liquidation, to cover for these rare situations is still the best way to go.

Koolex (judge) increased severity to Medium and commented:

Upgrading this to Medium due to the following:

The absence of a mechanism to allow liquidating bad debts ( CR < 1 ) creates a potential risk for the protocol.

I couldn’t find in the documentation/code that it is an accepted risk based on an intended design.

award Kerosene from the 1B total supply as a bonus to cover missing part of collateral This seems to be a good starting point. LP will be incentivised to liquidate, in return, they receive Kerosene. Kerosene then can be sold in the secondary market or re-used to mint DYAD.

However, consequences of this approach should be taken into account and addressed; for example, circulating Kerosene will increase which will influence the price.

# [M-03] setUnboundedKerosineVault not called during deployment, causing reverts when querying for Kerosene value after adding it as a Kerosene vault

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

setUnboundedKerosineVault not called during deployment, causing reverts when querying for Kerosene value after adding it as a Kerosene vault Submitted by Circolors, also found by SBSecurity, Al-Qa-qa, Infect3d, AamirMK, gumgumzum, 0xtankr, VAD37, Strausses, FastChecker, T1MOH, and carrotsmuggler

- https://github.com/code-423n4/2024-04-dyad/blob/cd48c684a58158de444b24854ffd8f07d046c31b/script/deploy/Deploy.V2.s.sol#L78-L82
- https://github.com/code-423n4/2024-04-dyad/blob/cd48c684a58158de444b24854ffd8f07d046c31b/src/core/Vault.kerosine.bounded.sol#L23-L30
- https://github.com/code-423n4/2024-04-dyad/blob/cd48c684a58158de444b24854ffd8f07d046c31b/README.md?plain=1#L66-L68
Root Cause The setUnboundedKerosineVault function was never called during deployment, nor was it planned to be called post deployment.

## Impact

Without setting the unboundedKerosineVault, any attempt to get the asset price of a dNFT that has uses the bounded Kerosene vault will result in a revert.

Note Regarding Vault Licenser VaultManagerV2 ’s addKerosene() erroneously does a vault license check using keroseneManager.isLicensed(vault) at VaultManagerV2.sol#L88 making it impossible to add Kerosene vaults to Notes.

As clarified by the sponsor in this video DYAD V2- Kerosene - Code4rena Audit #2, the vaults in keroseneManager are intended to be used for kerosene value calculation and kerosene vaults are not supposed to be added there. We updated the relevant Kerosene vault license checks to use vaultLicenser.isLicensed(vault) instead as it is aligned with the deployment script at Deploy.V2.s.sol#L95 since unboundedKerosineVault is added as a licensed vault with vaultLicenser.add(address(unboundedKerosineVault)) The two following code changes were made to VaultManagerV2.sol so that the unbounded kerosene vault can be added as a kerosene vault without further changes in the vaults, the vault manager, or the deployment script.

VaultManagerV2.sol#L88 From:

if (!

keroseneManager.

isLicensed ( vault )) revert VaultNotLicensed (); To:

if (!

vaultLicenser.

isLicensed ( vault )) revert VaultNotLicensed (); and VaultManagerV2.sol#L280 From:

if ( keroseneManager.

isLicensed ( address ( vault ))) { To:

if ( vaultLicenser.

isLicensed ( address ( vault ))) {

## Recommended Mitigation Steps

Set the unboundedKerosineVault during deployment.

Changes to DeployV2 Call the setUnboundedKerosineVault function during deployment after deploying the bounded Kerosene vault at Deploy.V2.s.sol#L78-L82:

boundedKerosineVault.

setUnboundedKerosineVault ( unboundedKerosineVault ); shafu0x (DYAD) acknowledged and commented:

Doesn’t necessarily need to be called in the deployment script.

McToady (warden) commented:

While the sponsor comment is true, the documentation in the audit’s README explicitly states:

The whole migration is described in Deploy.V2.s.sol. The only transaction that needs to be done by the multi-sig after the deployment is licensing the new Vault Manager.

This finding shows that this is, in fact, not the case and that the comments in the provided documentation suggest the team were unaware this would be an issue. Given the deploy script is within the scope of the audit, I believe this issue is a valid finding. If this issue had not been raised and the protocol had deployed as they previously outlined, users who deposit would have their funds stuck (due to both the withdraw and mintDyad functions reverting) until the DYAD team themselves worked out what the issue was and called the necessary function.

Koolex (judge) commented:

The statement in README is about the migration from VaultManager to VaultManagerV2.

users who deposit would have their funds stuck (due to both the withdraw and mintDyad functions reverting).

Not sure how the funds will be stuck if the UnboundedKerosineVault is not set.

UnboundedKerosineVault is used in BoundedKerosineVault to retrieve the price. So, BoundedKerosineVault will not function till this is set. Furthermore, withdraw is disallowed in BoundedKerosineVault.

McToady (warden) commented:

@Koolex - Funds will be stuck because the value of all a users collateral is checked on withdraw (not just the collateral they’re attempting to withdraw) in the collatRatio(id) call. Therefore, if they have added the bounded kerosene vault to their vaults, mapping the withdraw function will revert when attempting to calculate the value of their bounded kerosene.

Koolex (judge) commented:

After reviewing README and the comments above again, because of:

There is an impact (clarified already by the warden) that will make the protocol not function.

The statement in README.

The whole migration is described in Deploy.V2.s.sol. The only transaction that needs to be done by the multi-sig after the deployment is licensing the new Vault Manager.

# [M-04] Liquidating positions with bounded Kerosen could be unprofitable for liquidators

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by alix40, also found by dimulski, eta, Giorgio, ljj, and zhaojohnson Bounded Kerosene is not attractive for liquidation. First of all bounded kerosene has twice the value of unbounded kerosene. For example, if 1 Kerosene is worth 10 usd in the unbounded vault then 0.5 Kerosen will be worth the same amount in a bounded vault.

Twice the valuation is justified for users and Liquidity Providers. The problem however, is if we take the perspective of a liquidator. If he liquidates a position that has a significant percentage of the position value in bounded Kerosene Vault. The liquidator will receive the amount locked in the bounded kerosene vault, which he can’t withdraw; so the liquidator actually wouldn’t be necessary be able to swap the kerosene against stable coins to make a profit, for example. He will also receive half the amount of kerosene if this was an unbounded kerosene Vault. This might not be clear, so we encourage the reader to see the following example to understand the need to rework liquidations for bounded kerosene vaults.

Please note that there is a bug in the liquidate() function and Kerosene Collateral are not sent to the liquidator. As I have confirmed with the sponsor, the protocol is intended to send the Kerosene Tokens to the liquidators alongside the seized Eth.

## Assessed type

Context shafu0x (DYAD) commented:

Great find and description. Making liquidated kerosene unbounded is a good idea.

Koolex (judge) decreased severity to Medium alix40 (warden) commented:

@Koolex - I still think this is a high severity bug and for the following reasons:

Impact:

High -> Unprofitable liquidations always have a high impact.

Likeliness:

High -> Bounded Kerosene is a huge part of the new protocol update, and users are incentivized (2x Valuation) to lock their Kerosene Tokens in bounded Vaults; which would lead to a high likeliness of positions with bounded Kerosene Vaults in them becoming eligible for liquidation.

McToady (warden) commented:

I’m unsure how this issue is in scope given it first requires speculating on how the sponsor will mitigate the underlying issue (that neither Kerosene type is handled during liquidations).

In fact, KeroseneVault (inherited by bounded & unbounded vaults) has a move function which assumedly was supposed to be used to move Kerosene on liquidation meaning the liquidators would still receive bounded Kerosene.

function move ( uint from, uint to, uint amount ) external onlyVaultManager { id2asset [ from ] -= amount; id2asset [ to ] += amount; emit Move ( from, to, amount ); } The profitability of taking on BoundedKerosene is a decision to be made by the liquidator, and given that the liquidator is expected to also own a Note nft rather than a traditional liquidation bot, it’s likely they would also value owning bounded Kerosene.

The issue basically boils down to the opinion that “liquidators don’t want bounded Kerosene” and I don’t think is our job to decide whether that’s the case or not.

Additionally, the recommended mitigation of unlocking bounded Kerosene completely defeats the purpose of having bounded/unbounded kerosene and users would be able to enjoy the benefits of 2x valued Kerosene as collateral, and then liquidate themselves when they wished to withdraw their “bounded” Kerosene.

Koolex (judge) commented:

Leaving as Medium severity since it is explicitly known (by docs, code) that bounded can’t be withdrawn. One could argue that, liquidators could be aware of this and choose not to liquidate. Furthermore, liquidators could liquidate to accumulate bounded kerosene to utilize in the protocol for enhancing the CR. So, it is not completely unprofitable.

shafu0x (DYAD) acknowledged Note: For full discussion, see here.

# [M-05] No incentive to liquidate small positions could result in protocol going underwater

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by dimulski, also found by Al-Qa-qa, 0xleadwizard, pontifex, MrPotatoMagic, 0xlemon, igdbase, AvantGard, xiao, SBSecurity, Ocean_Sky, grearlake, Ryonen, Maroutis, OMEN, bhilare_, SpicyMeatball, Bauchibred, fandonov, Cryptor, darksnow, Aamir, web3km, Stefanov, atoko, Giorgio ( 1, 2 ), Sabit, Egis_Security, WildSniper, DarkTower, iamandreiski, T1MOH, and Tigerfrake The DYAD protocol allows users to deposit as little as 1 WEI via the deposit() function; however, in order to mint the DYAD token the protocol requires user to have a collateral ratio of 150% or above. Liquidators liquidate users for the profit they can make. Currently, the DYAD protocol awards the value of the DYAD token burned (1 DYAD token is always equal to

$1 when calculated in the liquidate function) + 20% of the collateral left to the liquidator.

function liquidate ( uint id, uint to ) external isValidDNft ( id ) isValidDNft ( to ) { uint cr = collatRatio ( id ); if ( cr >= MIN_COLLATERIZATION_RATIO ) revert CrTooHigh (); dyad.

burn ( id, msg.

sender, dyad.

mintedDyad ( address ( this ), id )); uint cappedCr = cr < 1e18 ?

1e18:

cr; uint liquidationEquityShare = ( cappedCr - 1e18 ).

mulWadDown ( LIQUIDATION_REWARD ); uint liquidationAssetShare = ( liquidationEquityShare + 1e18 ).

divWadDown ( cappedCr ); uint numberOfVaults = vaults [ id ].

length (); for ( uint i = 0; i < numberOfVaults; i ++) { Vault vault = Vault ( vaults [ id ].

at ( i )); uint collateral = vault.

id2asset ( id ).

mulWadUp ( liquidationAssetShare ); vault.

move ( id, to, collateral ); } emit Liquidate ( id, msg.

sender, to ); } If there is no profit to be made than there will be no one to call the liquidate function. Consider the following example:

User A deposits collateral worth $75, and mint 50 DYAD tokens equal to $50. The collateral ratio is 75/50 = 150%.

The price of the provided collateral drops, and now user A collateral is worth $70, 70/50 = 140% collateral ratio. The position should be liquidated now, so the protocol doesn’t become insolvent.

We get the following calculation:

cr & cappedCr = 1.4e18.

liquidationEquityShare = (1.4e18 - 1e18) * 0.2e18 = 80000000000000000 = 0.08e18.

liquidationAssetShare = (0.08e18 + 1e18) / 1.4e18 = 771428571428571428 ≈ 0.77e18.

The total amount of collateral the liquidator will receive is 70 * 0.77 = $53.9.

The dollar amount he spent for the DYAD token is $50 (assuming no depegs) so the profit for the liquidator will be $53.9 - $50 = $3.9.

The protocol will be deployed on Ethereum where gas is expensive. Because the reward the liquidator will receive is low, after gas costs taking into account that most liquidators are bots, and they will have to acquire the DYAD token on an exchange, experience some slippage, swapping fees, and additional gas cost, the total cost to liquidate small positions outweighs the potential profit of liquidators. In the end, these low value accounts will never get liquidated, leaving the protocol with bad debt and can even cause the protocol to go underwater.

Depending on the gas prices at the time of liquidation (liquidity at DEXes can also be taken into account, as less liquidity leads to bigger slippage) positions in the range of $150 - $200 can be unprofitable for liquidators. This attack can be beneficial to a whale, large competitor, or a group of people actively working together to bring down the stable coin. The incentive for them is there if they have shorted the DYAD token with substantial amount of money. The short gains will outweigh the losses they incur by opening said positions to grief the protocol. Also this is crypto, there have been numerous instances of prices dropping rapidly, and usually at that time gas prices are much higher compared to normal market conditions.

Note: The liquidation mechanism is extremely inefficient, as it requires bots to have a note in order to be able to liquidate a position; however, this is a separate issue, as even the inefficiency of the liquidation mechanism is fixed, small positions still won’t be profitable enough for liquidators.

## Recommended Mitigation Steps

Consider setting a minimum amount that users have to deposit before they can mint DYAD stable coin. Minimum amount of $500-600 should be good enough.

## Assessed type

Context shafu0x (DYAD) acknowledged and commented via duplicate Issue #1258:

This is a known issue. We normally would just liquidate small position ourselves.

# [M-06] Attacker can frontrun to prevent vaults from being removed from the dNFT owner’s position

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by TheSavageTeddy, also found by 0x175, dimulski, MrPotatoMagic, alix40, ArmedGoose ( 1, 2 ), SBSecurity, VAD37, AamirMK, josephdara, ljj, turvy_fuzz, CaeraDenoir, sashik_eth, grearlake, KYP, okolicodes, sil3th, 0xnev, n0kto, d_tony7470, adam-idarrha, Jorgect, SovaSlava, BiasedMerc, Egis_Security ( 1, 2, 3 ), AlexCzm, PoeAudits, carrotsmuggler, and 0x486776 When removing a vault from a dNFT position, the vault must have no assets for that dNFT.

function remove ( uint id, address vault ) external isDNftOwner ( id ) { if ( Vault ( vault ).

id2asset ( id ) > 0 ) revert VaultHasAssets ();...

}

- https://github.com/code-423n4/2024-04-dyad/blob/main/src/core/VaultManagerV2.sol#L94-L104
function removeKerosene ( uint id, address vault ) external isDNftOwner ( id ) { if ( Vault ( vault ).

id2asset ( id ) > 0 ) revert VaultHasAssets (); }

- https://github.com/code-423n4/2024-04-dyad/blob/main/src/core/VaultManagerV2.sol#L106-L116
However, since anyone can deposit into a dNFT, anyone can prevent a vault from being removed from a dNFT position by observing the call to remove() in the mempool, and frontrunning the transaction by depositing dust amounts to the dNFT.

## Impact

Anyone can stop a vault from being removed from a dNFT position, at almost no cost.

If the victim has reached the max vault limit, they must remove a vault before adding a new one to their dNFT position. Therefore, this vulnerability may force them to mint a new dNFT to use new vaults.

## Recommended Mitigation Steps

Allow dNFT owners to remove vaults from their dNFT positions even if it has assets, but have a clear warning that doing so may reduce their collateral and cause liquidation.

shafu0x (DYAD) confirmed

# [M-07] VaultManagerV2.sol::burnDyad function is missing an isDNftOwner modifier, allowing a user to burn another user’s minted DYAD

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

VaultManagerV2.sol::burnDyad function is missing an isDNftOwner modifier, allowing a user to burn another user’s minted DYAD Submitted by Pataroff, also found by MrPotatoMagic, Evo, SBSecurity, Jorgect, ljj, T1MOH, Egis_Security, and carrotsmuggler VaultManagerV2.sol has a function burnDyad that allows a DNft owner to burn his minted DYAD tokens.

function burnDyad ( uint256 id, uint256 amount ) external isValidDNft ( id ) { dyad.

burn ( id, msg.

sender, amount ); emit BurnDyad ( id, amount, msg.

sender ); } However, the function does not check if the DNft id that is passed to it and the isValidDNft modifier belongs to msg.sender, allowing any DNft owner to burn any other DNft owner minted DYAD by calling the burnDyad function with the other user’s DNft id.

## Impact

A user can prevent an open position from being liquidated by calling VaultManagerV2::burnDyad to burn his own DYAD balance, while retaining his DYAD debt, effectively creating bad debt that cannot be liquidated nor redeemed.

Moreover, by specifying a different DNft id from their own when calling VaultManagerV2::burnDyad, the user can clear DYAD debt from another position while retaining the DYAD balance associated with it, effectively tricking the protocol in allowing him to mint more DYAD as the position no longer has DYAD debt.

Recommended Mitigation:

Add isDNftOwner modifier to VaultManagerV2.sol::burnDyad to check if the passed DNft id belongs to msg.sender, preventing the function caller from being able to burn another user’s minted DYAD.

function burnDyad(uint256 id, uint256 amount) external isValidDNft(id) + isDNftOwner(id) { dyad.burn(id, msg.sender, amount); emit BurnDyad(id, amount, msg.sender); }

## Assessed type

DoS Koolex (judge) decreased severity to Medium shafu0x (DYAD) confirmed and commented via duplicate Issue #74:

Yeah, burn dyad should only be done by the owner.

# [M-08] Incorrect deployment/missing contract will break functionality

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by carrotsmuggler, also found by tchkvsky, pontifex, falconhoof, Hueber, Tychai0s, Evo, 0xlemon, d3e4, Mahmud, sashik_eth ( 1, 2 ), 0xleadwizard, SBSecurity, oakcobalt, Circolors, 3th, 0xSecuri, 0xtankr, alix40, ke1caM ( 1, 2 ), n4nika, TheSchnilch, btk, bbl4de, steadyman, Egis_Security, ducanh2706, lian886 ( 1, 2 ), zhuying, Aamir, TheSavageTeddy, itsabinashb ( 1, 2 ), 0xAlix2, Bauchibred, and Abdessamed The kerosene manager is the contract responsible for managing kerosene prices. In the current state it has broken functionality due to the design. The KeroseneManager contract contains a list of vaults.

When we look at UnboundedKerosineVault contract, we see what those vaults are used for:

function assetPrice () public view override returns ( uint ) { //...

address [] memory vaults = kerosineManager.

getVaults (); uint numberOfVaults = vaults.

length; for ( uint i = 0; i < numberOfVaults; i ++) { Vault vault = Vault ( vaults [ i ]); //...

* vault.

assetPrice () * 1e18 //...

} The KeroseneManager contract is expected to have a list of the backing vaults. This list is then queried for the individual assetPrice(). Crucially, the KeroseneManager contract will not have the vault which takes kerosene as the asset. This is because calling assetPrice on a vault handling kerosene will make it go into an infinite loop.

So this section of the code expects the KeroseneManager contract to only contain a list of the vaults which has exo collateral.

Now, let’s look at VaultManagerV2.sol contract’s getKeroseneValue function. This function is supposed to return the value of kerosene tokens a user has deposited. It should do this by querying all the vaults, which takes kerosene as the deposit.

function getKeroseneValue ( uint id ) public view returns ( uint ) { uint numberOfVaults = vaultsKerosene [ id ].

length (); for ( uint i = 0; i < numberOfVaults; i ++) { Vault vault = Vault ( vaultsKerosene [ id ].

at ( i )); if ( keroseneManager.

isLicensed ( address ( vault ))) { usdValue = vault.

getUsdValue ( id ); } //...

} So the vaultsKerosene should hold the vaults which take kerosene as its deposit. Then, the code calls keroseneManager.isLicensed(address(vault). The isLicensed function only checks in the same vaults array in the kerosene manager.

function isLicensed ( address vault ) return vaults.

contains ( vault ); So, this expects the KeroseneManager to also contain the kerosene accepting vaults as well!

We have shown that in UnboundedKerosineVault, the KeroseneManager contract is expected to have only the backing vaults, not the kerosene deposit vaults itself, or it will enter an infinite loop. We have also shown that in VaultManagerV2, the KeroseneManager contract is expected to have the kerosene deposit vaults as well, or it will not be able to pass the isLicensed check.

Both the above statements cannot be true at the same time. This is a design flaw. The KeroseneManager contract, if it contains the kerosene deposit vaults will break the kerosene pricing mechanism, and if it does not, it will break the manager contract. Thus, the current design is flawed and will break the functionality.

If we look at the deployment script, we see that the kerosene manager only has the backing vaults:

kerosineManager.

add ( address ( ethVault )); kerosineManager.

add ( address ( wstEth )); This means the manager’s isLicensed call will fail for vaults which accept kerosene.

## Recommended Mitigation Steps

Store the kerosene vaults info in the Licenser contract. Then change the keroseneManager.isLicensed call to vaultLicenser.isLicensed in the manager contract.

## Assessed type

Error shafu0x (DYAD) disputed and commented:

Kerosene Manager only uses vaults with exogenous collateral.

Koolex (judge) commented:

The getKeroseneValue() function should return the value of vaults that have kerosene tokens. However, as you mentioned, Kerosene Manager only uses vaults with exogenous collateral. So, those vaults won’t be licensed by Kerosene Manager as they don’t hold any kerosene tokens.

From Deploy V2 script:

KerosineManager kerosineManager = new KerosineManager (); kerosineManager.

add ( address ( ethVault )); kerosineManager.

add ( address ( wstEth )); This result in, getKeroseneValue will always return zero.

Instead of this condition:

if (keroseneManager.isLicensed(address(vault))) {.

We should probably have this:

if (!keroseneManager.isLicensed(address(vault)) and vaultLicenser.isLicensed(address(vault))).

This checks that the vault is a kerosene, since it is licensed by Licenser and not licensed by Kerosene Manager.

shafu0x (DYAD) confirmed and commented:

Ahh, ok now it makes sense. Thanks for the clarification. This is a correct find.

# [M-09] Value of kerosene can be manipulated to force liquidate users

- **Contest:** DYAD
- **Slug:** 2024-04-dyad
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-dyad
- **Source snapshot:** competitions/2024-04-dyad/final_report.html

Submitted by carrotsmuggler, also found by peanuts, pontifex, adam-idarrha, d3e4, Al-Qa-qa, amaron ( 1, 2 ), 0xblack_bird, foxb868, Infect3d, windhustler, 0xSecuri, jesjupyter, forgebyola, Ryonen, AM, KupiaSec, wangxx2026, SpicyMeatball, nnez, itsabinashb, cu5t0mpeo, zhuying, T1MOH, VAD37, darksnow ( 1, 2 ), Dudex_2004, Jorgect, 0xnev, AlexCzm, 0xAlix2, GalloDaSballo, and Egis_Security The value of kerosene tokens is calculated according to the following formula:

Note: Please see scenario in warden’s original submission.

The price of kerosene changes based on user actions. If a user has a large amount of tokens in their vault, that adds to the TVL. Now if a user mints a bunch of dyad tokens with their deposit as collateral, or if a removes a bunch of their unused collateral, the price of kerosene tokens will drop instantaneously.

When calculating the collateralization ratio of a user’s position, the price of kerosene is essential. This is used in the getKeroseneValue function.

function getKeroseneValue ( uint id ) public view returns ( uint ) { uint totalUsdValue; uint numberOfVaults = vaultsKerosene [ id ].

length (); for ( uint i = 0; i < numberOfVaults; i ++) { Vault vault = Vault ( vaultsKerosene [ id ].

at ( i )); uint usdValue; if ( keroseneManager.

isLicensed ( address ( vault ))) { usdValue = vault.

getUsdValue ( id ); } totalUsdValue += usdValue; } return totalUsdValue; } function getTotalUsdValue ( uint id ) public view returns ( uint ) { return getNonKeroseneValue ( id ) + getKeroseneValue ( id ); } As seen above, the totalUsdValue depends on the getKeroseneValue function, which calls getUsdValue on the kerosene vaults. So instantaneously dropping the price of kerosene tokens will also instantaneously drop the totalUSDValue of the user’s position, decreasing their collateralization ratio. This can be used to force users into liquidation, if their final ratio drops below the liquidation threshold.

Since this allows any user to affect the collateralization ratio of other user’s positions, this is a high severity issue.

## Recommended Mitigation Steps

Add a TWAP mechanism to calculate the price of kerosene. The price of kerosene being manipulatable instantaneously is a huge risk. By using a TWAP mechanism, the price of kerosene will be more stable, and users will have more time to react to changes in the price of kerosene.

## Assessed type

Oracle 0xMax1 (DYAD) commented:

Kerosene can be very volatile. Especially in the early days. Users should act accordingly.

shafu0x (DYAD) acknowledged Koolex (judge) decreased severity to Medium and commented:

Relying solely on the spot price makes users positions subject to liquidation which is a known issue in DeFi. Users should act to prevent their position from being liquidated, However, in this scenario, it is not possible since the price is manipulated in one block. Downgrading to medium since the price math is stated by the protocol.

## Rejected Primary Findings

# Rejected Primary Findings: DYAD

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
