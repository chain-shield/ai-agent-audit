# Benchmark Ground Truth: Althea Liquid Infrastructure

## Accepted H/M Findings

# Accepted H/M Findings: Althea Liquid Infrastructure

# [H-01] Holders array can be manipulated by transferring or burning with amount 0, stealing rewards or bricking certain functions

- **Contest:** Althea Liquid Infrastructure
- **Slug:** 2024-02-althea-liquid-infrastructure
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-althea-liquid-infrastructure
- **Source snapshot:** competitions/2024-02-althea-liquid-infrastructure/final_report.html

Submitted by BowTiedOriole, also found by 0xJoyBoy03 ( 1, 2 ), pontifex, 0x0bserver, psb01, ReadyPlayer2, rouhsamad, CodeWasp, DanielArmstrong, n0kto, 0xpiken, Krace ( 1, 2 ), nuthan2x ( 1, 2 ), spark, kinda_very_good ( 1, 2 ), 0xlamide, pynschon, gesha17, MrPotatoMagic, Brenzee, Honour, DarkTower, turvy_fuzz, Fitro, shaflow2, Babylen, slippopz, d3e4, matejdb ( 1, 2 ), web3pwn, max10afternoon ( 1, 2 ), 0xAadi ( 1, 2 ), JohnSmith, miaowu, Myrault, krikolkk, TheSavageTeddy ( 1, 2 ), SovaSlava, atoko, Breeje, cryptphi, 0xlemon, SpicyMeatball, Tigerfrake, parlayan_yildizlar_takimi, csanuragjain, petro_1912, zhaojohnson, peanuts, and Fassi_Security Lines of code

- https://github.com/code-423n4/2024-02-althea-liquid-infrastructure/blob/main/liquid-infrastructure/contracts/LiquidInfrastructureERC20.sol#L214-L231

## Impact

LiquidInfrastructureERC20._beforeTokenTransfer() checks if the to address has a balance of 0, and if so, adds the address to the holders array.

LiquidInfrastructureERC20#L142-145 bool exists = ( this.

balanceOf ( to ) != 0 ); if (!

exists ) { holders.

push ( to ); } However, the ERC20 contract allows for transferring and burning with amount = 0, enabling users to manipulate the holders array.

An approved user that has yet to receive tokens can initiate a transfer from another address to themself with an amount of 0. This enables them to add their address to the holders array multiple times. Then, LiquidInfrastructureERC20.distribute() will loop through the user multiple times and give the user more rewards than it should.

for ( i = nextDistributionRecipient; i < limit; i ++) { address recipient = holders [ i ]; if ( isApprovedHolder ( recipient )) { uint256 [] memory receipts = new uint256 []( distributableERC20s.

length ); for ( uint j = 0; j < distributableERC20s.

length; j ++) { IERC20 toDistribute = IERC20 ( distributableERC20s [ j ]); uint256 entitlement = erc20EntitlementPerUnit [ j ] * this.

balanceOf ( recipient ); if ( toDistribute.

transfer ( recipient, entitlement )) { receipts [ j ] = entitlement; } emit Distribution ( recipient, distributableERC20s, receipts ); } This also enables any user to call burn with an amount of 0, which will push the zero address to the holders array causing it to become very large and prevent LiquidInfrastructureERC20.distributeToAllHolders() from executing.

## Recommended Mitigation Steps

Adjust the logic in _beforeTokenTransfer to ignore burns, transfers where the amount is 0, and transfers where the recipient already has a positive balance.

function _beforeTokenTransfer( address from, address to, uint256 amount ) internal virtual override { require(!LockedForDistribution, "distribution in progress"); if (!(to == address(0))) { require( isApprovedHolder(to), "receiver not approved to hold the token" ); } if (from == address(0) || to == address(0)) { _beforeMintOrBurn(); } - bool exists = (this.balanceOf(to) != 0); - if (!exists) { + if (to != address(0) && balanceOf(to) == 0 && amount > 0) holders.push(to); }

## Assessed type

Token Transfer ChristianBorst (Althea) confirmed and commented:

This is a significant issue since it is a DoS attack vector and can cause miscalculation of entitlements. I also think the report is very clear in outlining the issue.

Medium Risk Findings (4)

# [M-01] LiquidInfrastructureERC20.sol disapproved holders keep part of the supply, diluting approved holders revenue.

- **Contest:** Althea Liquid Infrastructure
- **Slug:** 2024-02-althea-liquid-infrastructure
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-althea-liquid-infrastructure
- **Source snapshot:** competitions/2024-02-althea-liquid-infrastructure/final_report.html

LiquidInfrastructureERC20.sol disapproved holders keep part of the supply, diluting approved holders revenue.

Submitted by 0xloscar01, also found by Breeje, Limbooo, rouhsamad, n0kto, 0xAadi, jesjupyter ( 1, 2 ), 0xpiken, peanuts, thank_you, zhaojohnson, pkqs90, Tendency, max10afternoon, matejdb, BowTiedOriole, Fassi_Security, aslanbek, SpicyMeatball, ZanyBonzy, JohnSmith, Topmark, and atoko Lines of code

- https://github.com/code-423n4/2024-02-althea-liquid-infrastructure/blob/main/liquid-infrastructure/contracts/LiquidInfrastructureERC20.sol#L222

## Impact

When LiquidInfrastructureERC20 owner disapproves a holder, it prevents the holder from receiving any revenue from the contract. However, the disapproved holder will still keep part of the supply, diluting the revenue of the approved holders.

The dilution is a result of the calculation of the entitlements per token held, which is based on the division of the ERC20 balances held by the LiquidInfrastructureERC20 contract and the total supply of the LiquidInfrastructureERC20 token.

- https://github.com/code-423n4/2024-02-althea-liquid-infrastructure/blob/main/liquid-infrastructure/contracts/LiquidInfrastructureERC20.sol#L257-L281
function _beginDistribution () internal { require ( !

LockedForDistribution, "cannot begin distribution when already locked" ); LockedForDistribution = true; // clear the previous entitlements, if any if ( erc20EntitlementPerUnit.

length > 0 ) { delete erc20EntitlementPerUnit; } // Calculate the entitlement per token held uint256 supply = this.

totalSupply (); for ( uint i = 0; i < distributableERC20s.

length; i ++) { uint256 balance = IERC20 ( distributableERC20s [ i ]).

balanceOf ( address ( this ) ); @> uint256 entitlement = balance / supply; erc20EntitlementPerUnit.

push ( entitlement ); } nextDistributionRecipient = 0; emit DistributionStarted (); }

## Recommended Mitigation Steps

To prevent dilution of revenue for approved holders, consider implementing a mechanism to burn the tokens of disapproved holders upon disapproval.

In the event that disapproved holders are intended to retain their tokens for potential reapproval in the future, track the balance of the LiquidInfrastructureERC20 tokens held by the holder at the time of their disapproval. Subsequently, burn the tokens. Upon reapproval, mint the same amount of tokens to the holder, ensuring they regain their previous token balance.

ChristianBorst (Althea) confirmed and commented:

This is a good suggestion.

0xA5DF (judge) commented:

Maintaining medium severity (despite some dupes suggesting high) since this would only affect part of the revenue (% of disapproved supply) and the funds aren’t lost forever - they’re kept in the contract and will be distributed in the next round.

SovaSlava (warden) commented:

I think this is not a valid issue, because function disapproveHolder() is intended to limit the user’s receipt of rewards and this is intended.

To completely remove a user from the project and prevent him from receiving rewards, the owner must burn the user’s tokens. The disapproveHolder function is not enough for this and the owner must know this.

Owner could just call function burnFromAndDistribute() and disapproveHolder() in one tx.

sl1 (warden) commented:

Owner could just call function burnFromAndDistribute() and disapproveHolder() in one tx.

Just as a note, burnFromAndDistribute() requires allowance, so it will be impossible for owner to just burn user’s tokens.

0xA5DF (judge) commented:

The sponsor confirmed this is not the intended design.

The current design doesn’t actually reserve the reward for the disapproved holder, the remaining balance would be distributed in the next cycle to all holders equally.

Maintaining medium severity.

# [M-02] Malicious users can prevent holders from claiming their rewards during a reward cycle by skipping it.

- **Contest:** Althea Liquid Infrastructure
- **Slug:** 2024-02-althea-liquid-infrastructure
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-althea-liquid-infrastructure
- **Source snapshot:** competitions/2024-02-althea-liquid-infrastructure/final_report.html

Submitted by Fassi_Security, also found by max10afternoon, 0xJoyBoy03, and web3pwn The Sponsors noted via discord communication:

"The most likely configuration would be roughly weekly or monthly, based on average block times." "Althea-L1 will have a mempool,..." This is important to mention because this issue would not be significant if the minimum time between reward distribution was, for example, 100 blocks; and since there is a mempool, front-running is possible.

The project holds liquid NFTs, which accumulate rewards. These rewards are used to reward the erc20 token holders. These rewards are transferred to the liquiderc20 contract by using the two following functions:

function withdrawFromAllManagedNFTs () public { withdrawFromManagedNFTs ( ManagedNFTs.

length ); } function withdrawFromManagedNFTs ( uint256 numWithdrawals ) public { require (!

LockedForDistribution, "cannot withdraw during distribution" ); if ( nextWithdrawal == 0 ) { emit WithdrawalStarted (); } uint256 limit = Math.

min ( numWithdrawals + nextWithdrawal, ManagedNFTs.

length ); uint256 i; for ( i = nextWithdrawal; i < limit; i ++) { LiquidInfrastructureNFT withdrawFrom = LiquidInfrastructureNFT ( ManagedNFTs [ i ] ); ( address [] memory withdrawERC20s, ) = withdrawFrom.

getThresholds (); withdrawFrom.

withdrawBalancesTo ( withdrawERC20s, address ( this )); emit Withdrawal ( address ( withdrawFrom )); } nextWithdrawal = i; if ( nextWithdrawal == ManagedNFTs.

length ) { nextWithdrawal = 0; emit WithdrawalFinished (); } However, the problem here is the following check inside withdrawFromManagedNFTs:

require (!

LockedForDistribution, "cannot withdraw during distribution" ); Even when there are 0 rewards in the current contract, a malicious user can still call distribute(1) to start the distribution process and to set the LockedForDistribution boolean to true.

This results in no one being able to call withdrawFromManagedNFTs to get the rewards inside the erc20 contract to distribute, which results in the next reward cycle being after block.number + MinDistributionPeriod.

## Recommended Mitigation Steps

Do not start a distribution cycle if there are no rewards that can be paid out to the approved holders and keep track of the rewards currently held in the liquidNFT. Only start reward cycles when this amount that is held in the liquidNFT is sent to the liquidERC20. This prevents malicious users from sending 1 wei of rewardTokens to the liquidERC20 to maliciously start a distribution cycle.

ChristianBorst (Althea) acknowledged and commented via duplicate issue #594:

I think this is a valid suggestion but does not pose a risk to the protocol. This could enable a malicious owner (which is already a highly trusted role) to avoid distributing revenue to holders before minting new tokens.

There is no restriction on distribution frequency, a user could call distributeToAllHolders() multiple times every single block without withdrawing rewards from the ManagedNFTs if they want to pay the gas to do so. Even if a user decides to pay all that gas, nothing is preventing any other accounts from withdrawing rewards from the ManagedNFTs and performing yet another distribution to actually distribute the revenue to the holders.

I think that there is an argument to restrict owner misbehavior by forcing withdrawal before distribution, but the recommended mitigation strategy would introduce a new DoS vector to the protocol.

0xA5DF (judge) commented via duplicate issue #594:

There is no restriction on distribution frequency, a user could call distributeToAllHolders() multiple times every single block.

There is actually a restriction, you have to wait MinDistributionPeriod before calling it again ( code ).

Issue #119, is more clear and the mitigation makes more sense (ensuring withdrawFromManagedNFTs() was completed before starting distribution). Given that the minimum period is going to be about a week or a month I think this is a valid medium issue.

0xA5DF (judge) decreased severity to Medium and commented:

Marking as medium, as this is mostly only a temporarily grief of funds, and this would only happen if nobody calls withdrawFromManagedNFTs().

bronze_pickaxe (warden) commented:

@0xA5DF - We agree this should be of medium severity because it’s a temporary grief of funds.

As you correctly mentioned in your comment, this only happens if nobody calls withdrawFromManagedNFTs. Therefore, we ask you to consider de-duping the following dupes because they both describe another attack path, starting from someone calling withdrawFromManagedNFTs:

#594 #227 #594 and #227 both describe the following attack path in the wrong:

calling withdrawFromManagedNFTs if only one NFT distributes the reward, both these issues are invalid since they will still be fully distributed.

calling distribute() This is not the same as our report since the attack path that leads to skipping a reward cycle with no distribution is:

distribute() withdrawFromManagedNFTs There is no mention of calling distribute() before withdrawFromManagedNFTs.

#218 correctly identified the attack vector of calling withdrawFromManagedNFTs and distribute() in the wrong order, being: 1.

distribute() 2.

withdrawFromManagedNFTs This report correctly mentioned the attack path, unlike the other two.

0xA5DF (judge) commented:

I agree that they don’t fully identify the impact and don’t describe the issue well, but it seems like they’re touching on it:

Holders must wait for another MinDistributionPeriod (30 days) to receive their shares from other assets of ManagedNFTs. their assets will freeze for another 30 days.

This leads to a scenario where it the user might trigger withdrawFromManagedNFTs for a portion of managed NFTs, and then attacker calls distribute which block another call to withdrawFromManagedNFTs from finishing its work. That way only partial amount of rewards will be distributed since withdrawFromManagedNFTs did not pull all rewards.

I’ll give them partial credit.

# [M-03] Distribution can be bricked, and double claims by a few holders are possible when owner calls LiquidInfrastructureERC20::setDistributableERC20s

- **Contest:** Althea Liquid Infrastructure
- **Slug:** 2024-02-althea-liquid-infrastructure
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-althea-liquid-infrastructure
- **Source snapshot:** competitions/2024-02-althea-liquid-infrastructure/final_report.html

LiquidInfrastructureERC20::setDistributableERC20s Submitted by nuthan2x, also found by SpicyMeatball, 0x0bserver, AM, agadzhalov, Limbooo, offside0011, DanielArmstrong, aslanbek, jesjupyter, kartik_giri_47538, turvy_fuzz, Krace, CaeraDenoir, JrNet, zhaojohnson, KmanOfficial, pkqs90, imare, max10afternoon, SovaSlava, TheSavageTeddy, d3e4, Meera, atoko, juancito ( 1, 2 ), Kirkeelee, ziyou-, csanuragjain, kutugu, and xchen1130 Lines of code

- https://github.com/code-423n4/2024-02-althea-liquid-infrastructure/blob/bd6ee47162368e1999a0a5b8b17b701347cf9a7d/liquid-infrastructure/contracts/LiquidInfrastructureERC20.sol#L222

## Impact

Double claim, DOS by bricking distribution, few holders can lose some rewards due to missing validation of distribution timing when owner is calling LiquidInfrastructureERC20::setDistributableERC20s.

function setDistributableERC20s ( address [] memory _distributableERC20s ) public onlyOwner { distributableERC20s = _distributableERC20s; } This issue will be impacted the following ways:

When a new token is accepted by any NFT it should be added as a desirable token, or a new managerNFT with a new token, then, LiquidInfrastructureERC20::setDistributableERC20s has to be called to distribute the rewards.

So when the owner calls LiquidInfrastructureERC20::setDistributableERC20s which adds a new token as desired token:

Bricking distribution to holders: Check the POC for proof. When a new token is added, we can expect revert when:

The number of desirable tokens length array is increased.

An attacker can frontrun and distribute to only one holder, so erc20EntitlementPerUnit is now changed.

Now, the actual owner tx is processed, which increases/decreases the size of distributableERC20s array.

But now, distribution is not possible because the Out of Bound revert on the distribution function because distributableERC20s array is changed, but erc20EntitlementPerUnit array size is same before the distributableERC20s array is modified.

for ( uint j = 0; j < distributableERC20s.

length; j ++) { uint256 entitlement = erc20EntitlementPerUnit [ j ] * this.

balanceOf ( recipient ); if ( IERC20 ( distributableERC20s [ j ]).

transfer ( recipient, entitlement )) { receipts [ j ] = entitlement; } Double claim by a holder: so some holder can DOS by:

A last holder of the 10 holders array frontruns this owner tx ( setDistributableERC20s ) and calls LiquidInfrastructureERC20::distribute with 90% of the holders count as params, so all the rewards of old desirable tokens will be distributed to 9 holders.

Now, after the owners action, backruns it to distribute to the last holder, which will also receive new token as rewards.

The previous holders can also claim their share in the next distribution round, but the last holder also can claim which makes double claim possible, which takes a cut from all other holders.

If owner calls LiquidInfrastructureERC20::setDistributableERC20s, which removes some tokens, then any token balance in the contract will be lost until the owner re-adds the token, and it can be distributed again.

All the issues can be countered if the LiquidInfrastructureERC20::setDistributableERC20s is validated to make changes happen only after any potential distributions

## Recommended Mitigation Steps

Modify LiquidInfrastructureERC20::setDistributableERC20s to only change the desirable tokens after a potential distribution for fair reward sharing.

function setDistributableERC20s( address[] memory _distributableERC20s ) public onlyOwner { + require(!_isPastMinDistributionPeriod(), "set only just after distribution"); distributableERC20s = _distributableERC20s; }

## Assessed type

DoS ChristianBorst (Althea) confirmed via duplicate issue #260 0xA5DF (judge) decreased severity to Medium and commented:

Changing to medium since setDistributableERC20s() is a function rarely called. This would only be relevant if we’re after minimum distribution period has passed since the last distribution.

MrPotatoMagic (warden) commented:

@0xA5DF - I think this issue should be invalid.

This issue arises solely due to owner misbehaviour/error, which is a centralization risk and should be included in analysis reports. Changing the configuration of distributable ERC20 tokens in the middle of the distribution period is the owner’s fault and not expected behaviour.

0xA5DF (judge) commented:

The submission talks about a scenario where the owner sent out the tx when there was no distribution going on and a malicious actor front runs it. This is a likely scenario and isn’t due to an error on the admin side (the admin can prevent this by running distribution first, but there’s no way for the average admin to guess that running this tx without distributing first would cause any issues). Therefore, I’m maintaining medium severity.

# [M-04] Withdrawal from NFTs can be temporarily blocked

- **Contest:** Althea Liquid Infrastructure
- **Slug:** 2024-02-althea-liquid-infrastructure
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-althea-liquid-infrastructure
- **Source snapshot:** competitions/2024-02-althea-liquid-infrastructure/final_report.html

Submitted by SpicyMeatball, also found by SpicyMeatball, rokinot, PumpkingWok, rouhsamad, CaeraDenoir, nuthan2x, web3pwn, SovaSlava, Krace, d3e4, Meera, Breeje ( 1, 2 ), juancito, BowTiedOriole, JohnSmith, and kutugu If nextWithdrawal > ManagedNFTs.length, the contract won’t be able to withdraw revenue from managed NFTs, because nextWithdrawal can’t reset.

uint256 limit = Math.

min ( numWithdrawals + nextWithdrawal, ManagedNFTs.

length ); uint256 i; >> for ( i = nextWithdrawal; i < limit; i ++) { LiquidInfrastructureNFT withdrawFrom = LiquidInfrastructureNFT ( ManagedNFTs [ i ] ); ( address [] memory withdrawERC20s, ) = withdrawFrom.

getThresholds (); withdrawFrom.

withdrawBalancesTo ( withdrawERC20s, address ( this )); emit Withdrawal ( address ( withdrawFrom )); } nextWithdrawal = i; >> if ( nextWithdrawal == ManagedNFTs.

length ) { nextWithdrawal = 0; emit WithdrawalFinished (); }

## Recommended Mitigation Steps

Consider modifying the check here to:

+ if (nextWithdrawal >= ManagedNFTs.length) { nextWithdrawal = 0; emit WithdrawalFinished(); }

## Assessed type

DoS ChristianBorst (Althea) confirmed via duplicate issue #130

## Rejected Primary Findings

# Rejected Primary Findings: Althea Liquid Infrastructure

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
