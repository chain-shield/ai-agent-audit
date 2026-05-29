# Benchmark Ground Truth: Curves

## Accepted H/M Findings

# Accepted H/M Findings: Curves

# [H-01] Whitelisted accounts can be forcefully DoSed from buying curveTokens during the presale

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

curveTokens during the presale Submitted by 0xStalin, also found by osmanozdemir1, ahmedaghadi, whoismatthewmc1, nonseodion, deepplus ( 1, 2 ), d3e4, Josephdara_0xTiwa, matejdb ( 1, 2 ), gesha17 ( 1, 2 ), emrekocak, Tychai0s, c3phas, Aymen0909, 0xJaeger, anshujalan, hihen, slylandro_star, TermoHash, ktg, grearlake, Ryonen, yixxas, CDSecurity, lsaudit ( 1, 2 ), santipu_, jesusrod15, 0xc0ffEE, Kong, 0xprinc, Cosine, _eperezok, 0xE1, sl1, ke1caM, 0xLogos, KingNFT, n1punp ( 1, 2, 3 ), danb, lil_eth, UbiquitousComputing, DMoore, lukejohn, klau5, igbinosuneric, jasonxiale, KupiaSec, cats, hals, EV_om, bronze_pickaxe, cccz, Stormreckson, jangle,

Soliditors, y4y, batsanov, rvierdiiev, 0xPluto, and AS

- https://github.com/code-423n4/2024-01-curves/blob/main/contracts/Curves.sol#L328-L336
- https://github.com/code-423n4/2024-01-curves/blob/main/contracts/Curves.sol#L276-L279
Whitelisted accounts can be DoSed from buying curveTokens during the presale by a malicious party, as a result, the user who owns the whitelisted account will be forced to miss the presale and it will be able to acquire the curveTokens only during the open sale using a different account.

## Recommended Mitigation Steps

The most straightforward mitigation to prevent the permanent DoS is to implement logic that allows accounts to get rid of curveTokens from tokenSubjects they don’t want to own.

Make sure to implement the pop() functionality to the ownedCurvesTokenSubjects array when the account doesn’t own any curveToken of a tokenSubject.

This should be implemented in the functions Curves::sellCurvesToken() function & Curves::_transfer() function. Whenever the account’s curvesTokenBalance is updated on any of these two functions, make sure to validate if the post balance is 0, if so, pop the subjectToken ’s address from the ownedCurvesTokenSubjects array of the account.

By allowing users to have control over their ownedCurvesTokenSubjects array, there won’t be any incentive from third parties to attempt to cause a DoS by inflating the users’ ownedCurvesTokenSubjects array, now, each user will be able to clean up their array as they please.

A more elaborated mitigation that will require more changes across the codebase is to use EnumerableSets instead of arrays, and make sure to implement correctly the functions offered by the EnumerableSets, such as.contain(),.delete() and.add().

But in the end, the objective must be the same, allow users to have control over their ownedCurvesTokenSubjects, if they stop owning a curveToken of a certain tokenSubject, remove that address from the ownedCurvesTokenSubjects variable.

alcueca (Judge) commented:

The impact in this report is more severe than in the duplicates; however, the root cause stays the same.

andresaiello (Curves) acknowledged Note: For full discussion, see here.

# [H-02] Unrestricted claiming of fees due to missing balance updates in FeeSplitter *Submitted by EV_om , also found by lukejohn , Tychai0s , visualbits

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

FeeSplitter *Submitted by EV_om, also found by lukejohn, Tychai0s, visualbits, Josephdara_0xTiwa, kuprum, 0x0bserver, peritoflores, Tumelo_Crypto, anshujalan, Kose, nonseodion ( 1, 2 ), Aymen0909, grearlake, Kong, 0xPhantom ( 1, 2 ), jangle, rouhsamad, Soul22 ( 1, 2 ), btk ( 1, 2 ), 0xE1, Ryonen, Varun_05 ( 1, 2 ), 0xmystery, jacopod ( 1, 2, 3 ), wangxx2026 ( 1, 2 ), novodelta, ubermensch ( 1, 2 ), Draiakoo ( 1, 2 ), 0xprinc, santipu_, DarkTower, 0xc0ffEE ( 1, 2 ), petro_1912 ( 1, 2 ), almurhasan, Cosine, 0xNaN, zhaojie ( 1, 2 ), khramov, al88nsk, mahdirostami, Lalanda, iamandreiski ( 1, 2 ), jasonxiale ( 1, 2 ), UbiquitousComputing, pep7siup, osmanozdemir1, 0xLogos, salutemada, oreztker ( 1, 2 ), alexfilippov314, peanuts ( 1, 2 ), SovaSlava, aslanbek, SpicyMeatball, dimulski, BowTiedOriole, Bobface, Soliditors, dopeflamingo, nmirchev8, hals ( 1, 2 ), Krace, klau5, ke1caM, nuthan2x, cccz, pkqs90, AlexCzm ( 1, 2, 3 ), 0xMAKEOUTHILL, ptsanev, Kow, israeladelaja, rvierdiiev, kutugu ( 1, 2 ), DarkTower, peanuts, d3e4, 0x0bserver, Aymen0909, 0xE1, Soul22, and 0xStalin ** The FeeSplitter contract is designed to distribute fees among token holders. It employs an accumulator pattern to distribute rewards over time among users who do not sell or withdraw their tokens as ERC20s. This pattern works by maintaining an accumulator representing the cumulative total of the reward rate over time, and updating it for each user every time their balance changes.

However, the current implementation does not update the accumulator associated with each user during token transfers, deposits, or withdrawals. The onBalanceChange() function, which is responsible for updating the accumulator following changes in a user’s balance, is exclusively called from Curves._transferFees(), which is only called during buy and sell transactions.

This oversight can be easily exploited to drain the FeeSplitter contract of its fees. An attacker could repeatedly transfer the same tokens to new accounts and claim fees every time. This is possible because data.userFeeOffset[account] will be zero for every new account they transfer to, while the claimable rewards are calculated using the current balance returned by the Curves contract.

Since there is no limit to the amount of rewards that may accumulate in the FeeSplitter contract, this can be considered loss of matured yield and is hence classified as high severity.

## Recommended Mitigation Steps

To mitigate this issue, the onBalanceChange(token, account) function should be triggered during all token transfers, deposits, and withdrawals. For token transfers, it should be triggered for both accounts. This will ensure that userFeeOffset is accurately tracked, preventing users from claiming fees multiple times.

raymondfam (Lookout) commented:

The root cause is due to not calling updateFeeCredit() diligently.

andresaiello (Curves) confirmed

# [H-03] Attack to make CurveSubject to be a HoneyPot

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

CurveSubject to be a HoneyPot Submitted by KingNFT, also found by lukejohn, 0xmystery, DimaKush, vnavascues, Matue, adamn000, Kose, 0xDemon ( 1, 2 ), jacopod, 0xStalin, MrPotatoMagic, 42TechLabs, oxTory, nnez, yixxas, ktg, thank_you, ubermensch, opposingmonkey, nonseodion, btk, matejdb, 0xc0ffEE, salutemada, novodelta, zhaojie, _eperezok, sl1, mrudenko, PENGUN, osmanozdemir1, spacelord47, mahdirostami, DarkTower, InAllHonesty, SpicyMeatball, alexfilippov314 ( 1, 2 ), oreztker, Timenov, zhaojohnson, eeshenggoh, SovaSlava, dimulski, UbiquitousComputing, cats, Soliditors, nmirchev8, darksnow, BugzyVonBuggernaut, hals, ke1caM, peanuts, EV_om, cccz, bronze_pickaxe, OMEN, 0xMAKEOUTHILL, aslanbek, israeladelaja, Kow, and haxatron, Any CurveSubjects clould be turned to a HoneyPot by the creator of CurveSubject, which causes that users can only buy but can’t sell the curve tokens any more. Then malicious creators can sell their own tokens at a high price to make profit.

## Recommended Mitigation Steps

Solady’s forceSafeTransferETH() seems suitable for this case:

- https://github.com/Vectorized/solady/blob/61612f187debb7affbe109543556666ef716ef69/src/utils/SafeTransferLib.sol#L117
andresaiello (Curves) confirmed alcueca (Judge) commented:

Keeping as High because with this the DAO controlling the platform would have to do an enormous effort to keep scammers out.

# [H-04] Unauthorized Access to setCurves Function

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

setCurves Function Submitted by parlayan_yildizlar_takimi, also found by Avci, 0x11singh99, visualbits, bigtone, djxploit, 0xMango, Arion, Nachoxt17, m4ttm, Ephraim, GhK3Ndf, spark, karanctf, LeoGold, peritoflores, Matue, dutra, dyoff, vnavascues, Josephdara_0xTiwa, SanketKogekar, Kose, Faith, McToady, santipu_, imare, jangle, Aymen0909, ktg, ivanov, bengyles ( 1, 2 ), Nikki, 0xAadi, kuprum, Zach_166, c0pp3rscr3w3r, 0xhashiman, burhan_khaja, anshujalan, The-Seraphs, mitev, Soul22, ArsenLupin, rouhsamad, DimaKush, tonisives, kodak_rome, adamn000, TermoHash, nazirite, 0xStalin, baice, Ryonen, cu5t0mpeo, whoismatthewmc1, FastChecker, bbl4de, Mike_Bello90, erebus, ether_sky, Draiakoo, opposingmonkey, btk, azanux, 0xSmartContract, XDZIBECX, Varun_05, ahmedaghadi, novodelta, Mj0ln1r, Berring, VigilantE, MrPotatoMagic, 0xc0ffEE, LouisTsai, nonseodion, forkforkdog, codegpt, Kong, DanielArmstrong, kodyvim, 0xprinc, Cosine, salutemada, th13vn, Inspex, popelev, grearlake, zhaojie, _eperezok, jacopod, 0xNaN, PoeAudits, Oxsadeeq, khramov, 0xblackskull, bareli, KHOROAMU, para8956, KingNFT, 0xPhantom, 13u9, sl1, AmitN, wangxx2026, pipidu83, XORs33r, mahdirostami, Prathik3, merlinboii, PENGUN, jasonxiale, pep7siup, lukejohn, spacelord47, danb, dd0x7e8, Lirios, 0xLogos, alexfilippov314, DMoore, iamandreiski, PetarTolev, zhaojohnson, iberry, eeshenggoh, zxriptor, Kaysoft, SovaSlava, oreztker, nmirchev8, UbiquitousComputing, dimulski, BowTiedOriole, jesjupyter, SpicyMeatball, zaevlad, Lef, Bobface, darksnow, KupiaSec, hals, Stormreckson, 0x111, klau5, ke1caM, peanuts, n1punp, cats, EV_om, bronze_pickaxe, cartlex_, rudolph, L0s1, deepplus, Inference, 0xMAKEOUTHILL, lil_eth, polarzero, Timenov, IceBear, nuthan2x, ubl4nk, skyge, AlexCzm, andywer, mrudenko, alexbabits, pipoca, Timeless, y4y, kutugu, Krace, ravikiranweb3, haxatron, and Soliditors The FeeSplitter.sol contract, which is responsible for fee distribution and claiming, contains a significant security vulnerability related to the

setCurves function. This function allows updating the reference to the Curves contract. However, as it currently stands, any user, including a malicious actor, can call setCurves. This vulnerability can be exploited to redirect the contract’s reference to a fake or malicious Curves contract ( FakeCurves.sol ), enabling manipulation of critical calculations used in fee distribution.

The exploit allows an attacker to set arbitrary values for curvesTokenBalance and curvesTokenSupply in the fake Curves contract. By manipulating these values, the attacker can falsely inflate their claimable fees, leading to unauthorized profit at the expense of legitimate token holders.

## Recommended Mitigation Steps

To mitigate this vulnerability, the setCurves function in FeeSplitter.sol should be restricted to be callable only by the owner or a trusted manager. This can be achieved by using the onlyOwner or onlyManager modifier (from the inherited Security.sol contract) in the setCurves function.

The modified setCurves function should look like this:

function setCurves ( Curves curves_ ) public onlyOwner { curves = curves_; } or, if managers are also trusted to perform this action, function setCurves ( Curves curves_ ) public onlyManager { curves = curves_; } andresaiello (Curves) confirmed

# [H-05] Malformed equate statement

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

Submitted by ChaseTheLight Note: This finding was reported via the winning

# [M-01] Protocol and referral fee would be permanently stuck in the Curves contract when selling a token

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

Submitted by anshujalan, also found by lukejohn, zxriptor, ether_sky, ahmedaghadi, 0x0bserver, 0x11singh99, todorc, Aymen0909, c0pp3rscr3w3r, FastChecker, ro1sharkm, tonisives, adeolu, cu5t0mpeo, whoismatthewmc1, Oxsadeeq, Silvermist, Ryonen, ktg, nonseodion, CDSecurity, MrPotatoMagic, santipu_ ( 1, 2 ), grearlake, dimulski, BowTiedOriole ( 1, 2 ), codegpt, Cosine, _eperezok, khramov, rouhsamad, para8956, Topmark, 0xPhantom, XORs33r, aslanbek, sl1, dd0x7e8, mrudenko, mahdirostami, fishgang, alexfilippov314, zhaojohnson, SovaSlava, hals, UbiquitousComputing, SpicyMeatball, KupiaSec, klau5, cats, cccz, DanielArmstrong, developerjordy, Soliditors, deepplus, Inference, nuthan2x, petro_1912, and haxatron During the sale of a token Curves._transferFee subtracts all the fees from the selling price and transfers the remaining to the seller here.

uint256 sellValue = price - protocolFee - subjectFee - referralFee - holderFee; ( bool success1, ) = firstDestination.

call {value:

isBuy ?

buyValue:

sellValue }( "" ); However, the protocolFee taken away is not transferred to the protocolFeeDestination in the remainder of the _transferFee function here. It stays back in the contract with no other of way of retrieval.

Furthermore, referralFee is taken away without checking if a referral address actually exists. In the event that there is no referral defined, the third transfer is never executed here. This leaves the referral fee in the contract, again no retrieval mechanism.

## Recommended Mitigation Steps

The buyValue ( here ) variable already has the logic for jointly handling the protocol and referral fee. It can be given a more generic name, and be transferred to the protocolDestination for both buying and selling transactions.

uint256 protocolShare = referralDefined ? protocolFee: protocolFee + referralFee; uint256 sellValue = price - protocolFee - subjectFee - referralFee - holderFee; (bool success, ) = (feesEconomics.protocolFeeDestination).call{value: protocolShare}(""); if (!success) revert CannotSendFunds(); if(!isBuy) { (bool success1, ) = (msg.sender).call{value: sellValue}(""); if(!success1) revert CannotSendFunds(); } alcueca (Judge) decreased severity to Medium andresaiello (Curves) acknowledged

# [M-02] Theft of holder fees when holderFeePercent was positive and is set to zero

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

holderFeePercent was positive and is set to zero Submitted by santipu_, also found by zhaojie, _eperezok, SpicyMeatball, rvierdiiev, and BowTiedOriole In the Curves protocol, a holder fee is imposed on all buy and sell transactions. This fee is distributed among share holders via the FeeSplitter contract, incentivizing long-term holding over frequent trading. The _transferFees() function plays a crucial role in this process by transferring the holder fee to FeeSplitter.

if ( feesEconomics.

holdersFeePercent > 0 && address ( feeRedistributor ) != address ( 0 )) { feeRedistributor.

onBalanceChange ( curvesTokenSubject, msg.

sender ); feeRedistributor.

addFees {value:

holderFee }( curvesTokenSubject ); } This code ensures that onBalanceChange() and addFees() are called only when holdersFeePercent is non-zero, deeming FeeSplitter unnecessary otherwise.

A critical vulnerability arises when the holderFeePercent, initially set to a positive value, is later changed to zero. In this case, previously collected holder fees remain in the FeeSplitter, awaiting distribution. The absence of onBalanceChange() calls in such scenarios allows new shareholders to unjustly claim these fees, leading to a potential theft of funds.

## Impact

When the holder fee is reduced from a positive value to zero, new shareholders can exploit this to illicitly claim holder fees from FeeSplitter. This theft is not only limited to these new shareholders but can also be perpetuated across multiple addresses, progressively draining the FeeSplitter of its ETH reserves.

## Recommended Mitigation Steps

To prevent this exploitation, it is advised to always call onBalanceChange() in _transferFees(), regardless of the holdersFeePercent status. This ensures continuous and accurate tracking of shareholder balances and rightful fee distribution, even when the holder fee is set to zero.

+ feeRedistributor.onBalanceChange(curvesTokenSubject, msg.sender); if (feesEconomics.holdersFeePercent > 0 && address(feeRedistributor) != address(0)) { - feeRedistributor.onBalanceChange(curvesTokenSubject, msg.sender); feeRedistributor.addFees{value: holderFee}(curvesTokenSubject); } andresaiello (Curves) confirmed, but disagreed with severity and commented:

Not high, because a DAO is responsible for that change, but will fix it.

alcueca (Judge) decreased severity to Medium and commented:

Even after merging #1491 with #247 which is more general, it still doesn’t cover the issue here that unclaimed fees need to be considered if changing the holders fee to zero.

# [M-03] If a user sets their curve token symbol as the default one plus the next token counter instance it will render the whole default naming functionality obsolete

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

Submitted by iamandreiski, also found by UbiquitousComputing, dutra, 0x0bserver, m4ttm, peritoflores, cheatc0d3, imare, Tychai0s, FastChecker, Soul22, kodyvim, deepplus, whoismatthewmc1, nonseodion ( 1, 2 ), Matue, erebus, ether_sky, Draiakoo, LouisTsai, 0xprinc, aslanbek, Cosine, BugzyVonBuggernaut, KupiaSec, ke1caM, jesjupyter, PENGUN, spacelord47, SpicyMeatball, dimulski, ktg, DanielArmstrong, nmirchev8 ( 1, 2 ), KingNFT, Mylifechangefast_eth, SovaSlava, zaevlad, jasonxiale, hals, AlexCzm, 51l3nt, cats, EV_om, ZanyBonzy, ubl4nk, cccz, pkqs90, israeladelaja, jangle, Krace, and haxatron

- https://github.com/code-423n4/2024-01-curves/blob/516aedb7b9a8d341d0d2666c23780d2bd8a9a600/contracts/Curves.sol#L338-L362
- https://github.com/code-423n4/2024-01-curves/blob/516aedb7b9a8d341d0d2666c23780d2bd8a9a600/contracts/Curves.sol#L47
If a malicious user or just an unbeknownst one decides to name/set their curve token symbol as “CURVE N” “N” can/will be substituted to whatever the next number is in the _curvesTokenCounter it can render the whole default naming functionality in the protocol useless as it will be DoS’d indefinitely.

## Recommended Mitigation Steps

Forbid users to include CURVES in their symbol by including checks when manually setting name/symbol or creating a token with name/symbol and reserve these keywords only for the default naming functionality or rethink how the architecture can be changed to make including symbols/names by users mandatory.

andresaiello (Curves) confirmed

# [M-04] Withdrawing with amount = 0 will forcefully set name and symbol to default and disable some functions for token subject

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

= 0 will forcefully set name and symbol to default and disable some functions for token subject Submitted by ktg, also found by ktg, gkrastenov, Nikki, anshujalan, imare, gesha17, burhan_khaja, ether_sky, nonseodion ( 1, 2 ), HChang26, MrPotatoMagic, grearlake, santipu_, matejdb, jangle, th13vn, SovaSlava, para8956, merlinboii, jasonxiale, nuthan2x, Bobface, 0xPluto, y4y, and 0x0bserver Name and symbol is forcefully set to default value before the token subject can do anything.

Disable functions mint, setNameAndSymbol and buyCurvesTokenWithName for token subject

## Recommended Mitigation Steps

I recommend you forbid calling function withdraw with amount = 0.

andresaiello (Curves) acknowledged

# [M-05] Stuck rewards in FeeSplitter contract

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

FeeSplitter contract Submitted by hals, also found by ether_sky, visualbits, m4ttm, and btk Curves protocol incentivize users to buy subjects and hold it for a prolonged time to receive high rewards, and the claimable rewards amount entitled for each user is calculated based on the user subject balance times the difference between the subject cumulativeFeePerToken and the user’s userFeeOffset of that subject, where the user’s userFeeOffset is updated whenever he buys or sells that specific subject:

```javascript function onBalanceChange(address token, address account) public onlyManager { TokenData storage data = tokensData[token]; data.userFeeOffset[account] = data.cumulativeFeePerToken; if (balanceOf(token, account) > 0) userTokens[account].push(token); } ``` The cumulativeFeePerToken is updated by an increment of msg.value / PRECISION, where msg.value represents the holdersFee amount that is deducted from each selling/purchase price of each subject token, and this value is accumulated to be claimed later by that subject holders (via FeeSplitter.addFees function):

```javascript function addFees(address token) public payable onlyManager { uint256 totalSupply_ = totalSupply(token); if (totalSupply_ == 0) revert NoTokenHolders(); TokenData storage data = tokensData[token]; //@audit-issue: the more the totalSupply increases the more stuck rewards (division loss) data.cumulativeFeePerToken += (msg.value * PRECISION) / totalSupply_; } ``` But there’s an issue with this implementation:

There will be always a locked amount in the contract equals to the latest subject’s cumulativeFeePerToken, and this amount will be stuck and not utilized as there’s no way to withdraw them from the splitter contract (no withdraw function to rescue any stuck funds in excess of the contracts’s needs).

These stuck accrued rewards will never be fully claimed as well and will be increasing with each purchase or selling of that subject, because users will be able to claim rewards based on their subject balance times the difference between the subject cumulativeFeePerToken and the user’s userFeeOffset of that subject only.

This issue is presented in the PoC section below, where all subject holders claim their rewards while there’s still a stuck amount of that subject rewards in the contract.

## Recommended Mitigation Steps

Implement a mechanism to utilize these stuck rewards or withdraw them; or modify the current rewarding mechanism to prevent any stuck rewards in the future.

andresaiello (Curves) acknowledged Note: For full discussion, see here.

# [M-06] A subject creator within a single block can claim holder fees without holding due to unprotected reentrancy path

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

Submitted by nuthan2x, also found by nonseodion, DarkTower, spaghetticode_sentinel, 0xNaN, BowTiedOriole, xiao, 0xPhantom, zhaojie, hals, alexfilippov314, aslanbek, nmirchev8, Kow, and kutugu ( 1, 2 )

- https://github.com/code-423n4/2024-01-curves/blob/516aedb7b9a8d341d0d2666c23780d2bd8a9a600/contracts/Curves.sol#L236-L241
- https://gist.github.com/nuthan2x/bb0ecf745abfdc37ce374f6af0d83699#L17
- https://github.com/code-423n4/2024-01-curves/blob/516aedb7b9a8d341d0d2666c23780d2bd8a9a600/contracts/FeeSplitter.sol#L80
A subject creater can keep on claiming holder fees on every buy and sell transaction even when he doesn’t hold the balance. This claiming of holder fees without holding is possible due to a combination of reentrancy and usage of call instead of transfer while transferring the subject fees.

A subject can perform this attack by:

Buy some curves initially.

mint and lock them in curves. The external minted tokens can be used on bridges or AMMs.

Now, when someone buys/sells, subject fee is transferred on _transferFees call.

Using this external call, the subject when receiving the ether, can withdraw tokens from bridge/AMM and then burn those tokens to get the curves. Then use those curves updated balance, to claim the updated holder fees. Then lock those curves again and mint external tokens, then lock them on Bridge/AMM.

For attacker to claim this holder fees unethically, only once every two transactions of buy and sell. Because, the holder fee is updated after sending the subject fees. So previous update of the holders fees can be claimed only next time someone buys/sells.

## Recommended Mitigation Steps

Since this attack is possible for both subjects and referrers, mitigate as recommended below. Use transfer, instead of call on subject/referral fee transfers, so that only 2300 units of gas is alloted when receiving, and is not possible to perform any reentrancy attack.

Or, implement Reentrancy guard from openzeppelin library.

function _transferFees( address curvesTokenSubject, bool isBuy, uint256 price, uint256, uint256 ) internal { (uint256 protocolFee, uint256 subjectFee, uint256 referralFee, uint256 holderFee, ) = getFees(price); { bool referralDefined = referralFeeDestination[curvesTokenSubject] != address(0); { address firstDestination = isBuy ? feesEconomics.protocolFeeDestination: msg.sender; uint256 buyValue = referralDefined ? protocolFee: protocolFee + referralFee; uint256 sellValue = price - protocolFee - subjectFee - referralFee - holderFee; (bool success1, ) = firstDestination.call{value: isBuy ? buyValue: sellValue}(""); if (!success1) revert CannotSendFunds(); } { - (bool success2, ) = curvesTokenSubject.call{value: subjectFee}("");

- if (!success2) revert CannotSendFunds(); + payable(curvesTokenSubject).transfer(subjectFee); } { - (bool success3, ) = referralDefined - ? referralFeeDestination[curvesTokenSubject].call{value: referralFee}("") -: (true, bytes("")); - if (!success3) revert CannotSendFunds(); + if(referralDefined) payable(referralFeeDestination[curvesTokenSubject]).transfer(referralFee); } if (feesEconomics.holdersFeePercent > 0 && address(feeRedistributor) != address(0)) { feeRedistributor.onBalanceChange(curvesTokenSubject, msg.sender); feeRedistributor.addFees{value: holderFee}(curvesTokenSubject); } raymondfam (Lookout) commented:

data.userFeeOffset[account] = data.cumulativeFeePerToken will take care of it when updateFeeCredit() is triggered in claimFees(). You can only do it once.

alcueca (Judge) decreased severity to Medium and commented:

You can do it once per buy/sell cycle, as I understand. Issues related to loss of fees, and not loss of principal, are Medium.

andresaiello (Curves) confirmed

# [M-07] Selling will be bricked if all other tokens are withdrawn to ERC20 token

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

Submitted by BowTiedOriole, also found by d3e4, McToady, slylandro_star, 0xmystery, ether_sky, amaechieth, 0xPhantom, wangxx2026, nuthan2x, zxriptor, Bobface, Krace, hals, cats, EV_om, and AlexCzm

- https://github.com/code-423n4/2024-01-curves/blob/main/contracts/FeeSplitter.sol#L90
- https://github.com/code-423n4/2024-01-curves/blob/main/contracts/FeeSplitter.sol#L43-L46
- https://github.com/code-423n4/2024-01-curves/blob/main/contracts/Curves.sol#L248
FeeSplitter.addFees() reverts if the totalSupply of the token is 0. The total supply is calculated via the formula below:

return ( curves.

curvesTokenSupply ( token ) - curves.

curvesTokenBalance ( token, address ( curves ))) * PRECISION; Consider the following scenario:

Alice buys 1 Curve token to initiate herself as a token subject and withdraws her token to ERC20.

Bob buys 1 Curve token.

Charlie buys 10 Curve tokens and immediately withdraws all of them to the ERC20 contract.

Bob tries to sell 1 Curve token but is not able to because all other tokens are ERC20.

Bob’s transaction will revert because FeeSplitter.totalSupply() will return 0.

## Recommended Mitigation Steps

Check if the Curve contract balance matches the total supply, and if so, send the holder fee to the protocol.

if ( curvesTokenSupply [ curvesTokenSubject ] == curvesTokenBalance [ curvesTokenSubject ][ address ( this )]) { ( bool success, ) = feesEconomics.

protocolFeeDestination.

call {value:

holderFee }( "" ); if (!

success ) revert CannotSendFunds (); } else { feeRedistributor.

onBalanceChange ( curvesTokenSubject, msg.

sender ); feeRedistributor.

addFees {value:

holderFee }( curvesTokenSubject ); } Alternatively, you can call _transferFees prior to updating token balances and supply, but this will require reentrancy guards as well as other adjustments to the fee calculations.

andresaiello (Curves) confirmed

# [M-08] Single token purchase restriction on curve creation enables sniping

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

Submitted by EV_om, also found by mrudenko, McToady, adamn000, M3azad ( 1, 2 ), burhan_khaja, bengyles, matejdb, jesjupyter, 13u9, wangxx2026, pipidu83, erosjohn, eeshenggoh, lukejohn, aslanbek ( 1, 2 ), twcctop, KingNFT, and Daniel526 The getPrice() function in the Curves contract is used to calculate the price of tokens when buying or selling. However, due to the order of arguments in the sum2 calculation within this function, subjects are restricted to buying only one token when initializing their curve.

The issue arises from the expression (supply - 1 + amount), which reverts when supply is greater than 1. However, if the expression was (supply + amount - 1), this issue would not occur. This restriction limits the functionality of the contract and makes it vulnerable to sniping on initialization and first-purchase frontrunning. This issue has spawned a small industry around friend.tech, with different companies offering sniping bots that can make instant automatic first purchases for accounts associated with a sizeable social media presence.

function getPrice ( uint256 supply, uint256 amount ) public pure returns ( uint256 ) { uint256 sum1 = supply == 0 ?

0: (( supply - 1 ) * ( supply ) * ( 2 * ( supply - 1 ) + 1 )) / 6; uint256 sum2 = supply == 0 && amount == 1 ?

0: (( supply - 1 + amount ) * ( supply + amount ) * ( 2 * ( supply - 1 + amount ) + 1 )) / 6;...

} Although, the issue can be mitigated in the Curves implementation by the curve subject opening a presale, it is still present for curves initialized without presale.

## Recommended Mitigation Steps

To resolve this issue, the order of arguments in the sum2 calculation within the getPrice() function should be adjusted. Instead of (supply - 1 + amount), the expression should be (supply + amount - 1). This change will allow subjects to buy more than one token when initializing their curve, enhancing the contract’s functionality and protecting the users.

andresaiello (Curves) disputed via duplicate Issue #44 and commented:

Reverts in the price calculation.

alcueca (Judge) commented:

I’m not convinced that the purpose of the presales functionality is to allow the token subject to create a presale for himself just so that he can buy more than one token (and then not being able to have a presale for other users).

If the token subject would choose to do a regular presale, using the regular community mechanisms from the NFT era, how would they ensure that they don’t get a sniping bot registered for the presale?

The team might have intended to solve the issues with frontrunners, but to me it doesn’t seem they succeeded.

Note: For full discussion, see here.

# [M-09] Curves::_buyCurvesToken() , Excess of Eth received is not refunded back to the user.

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

Curves::_buyCurvesToken(), Excess of Eth received is not refunded back to the user.

Submitted by ravikiranweb3, also found by btk, ether_sky ( 1, 2 ), 0xprinc ( 1, 2 ), pkqs90, cats, cartlex_, nuthan2x, pep7siup ( 1, 2 ), Prathik3, MrPotatoMagic ( 1, 2 ), Kose, peanuts, 0xmystery ( 1, 2 ), Ephraim, m4ttm, spark, 0xStriker, emrekocak, SanketKogekar, Bjorn_bug, LeoGold, 0xlamide ( 1, 2 ), c3phas, imare, PetarTolev, anshujalan, M3azad, spaghetticode_sentinel, Nikki, Timeless, Varun_05, negin, ro1sharkm, Night, tonisives, hihen, Zach_166, kodyvim, cu5t0mpeo, FastChecker, yixxas, Ryonen, nonseodion, ubermensch, Tychai0s, Silvermist, codegpt, grearlake, rouhsamad ( 1, 2 ), Cosine, HChang26, _eperezok, 0xblackskull, para8956, 13u9, Kong, dd0x7e8, iamandreiski, ubl4nk ( 1, 2 ), pipidu83, mahdirostami, merlinboii, dopeflamingo, oreztker, osmanozdemir1, sl1, DarkTower, Kaysoft, UbiquitousComputing, zhaojohnson, 0xSwahili, eeshenggoh, dimulski, SovaSlava, BowTiedOriole, developerjordy, zaevlad, Lef, KmanOfficial, latt1ce, KupiaSec, jasonxiale, hals ( 1, 2 ), Mwendwa, aslanbek, ke1caM, ZanyBonzy, EV_om, cccz, bronze_pickaxe, Oxsadeeq, ktg, twcctop, deepplus, Inference, Soliditors, lil_eth ( 1, 2 ), Mylifechangefast_eth, AlexCzm, haxatron, and alexbabits

- https://github.com/code-423n4/2024-01-curves/blob/516aedb7b9a8d341d0d2666c23780d2bd8a9a600/contracts/Curves.sol#L211-L216
- https://github.com/code-423n4/2024-01-curves/blob/516aedb7b9a8d341d0d2666c23780d2bd8a9a600/contracts/Curves.sol#L263-L280
In the buyCurvesToken(), the logic check for the msg.value to be greater than price + fees. This is fine, since if the price moves after the estimates was provided to the user, the above validation checks if the funds received are sufficient to proceed with the transaction.

But, at the same time, it is equally important to refund any excess eth received which is not being done.

## Recommended Mitigation Steps

uint256 excess = msg.value - (price + totalFee); if excess > 0, refund the amount back to the caller.

andresaiello (Curves) confirmed, but disagreed with severity and commented:

Valid but not high severity. Friend tech does not refund in fact.

alcueca (Judge) decreased severity to Medium

# [M-10] onBalanceChange causes previously unclaimed rewards to be cleared

- **Contest:** Curves
- **Slug:** 2024-01-curves
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-curves
- **Source snapshot:** competitions/2024-01-curves/final_report.html

onBalanceChange causes previously unclaimed rewards to be cleared Submitted by kutugu, also found by 0xMAKEOUTHILL, pkqs90 ( 1, 2 ), zxriptor, visualbits, d3e4, Tychai0s, peritoflores, 0x0bserver, kuprum, xiao, dimulski, McToady, SovaSlava, Aymen0909, anshujalan, Zach_166, rouhsamad, Kose, 0xAadi, nonseodion, nazirite, Soul22, TermoHash, ether_sky ( 1, 2, 3 ), whoismatthewmc1, 0xmystery ( 1, 2, 3 ), HChang26, FastChecker, jacopod, AgileJune, matejdb ( 1, 2 ), ubermensch, Mike_Bello90 ( 1, 2 ), btk, Varun_05, santipu_, 0xPhantom, DarkTower, codegpt, sl1, KingNFT, almurhasan, erosjohn, Cosine, zhaojie, Topmark, dd0x7e8, UbiquitousComputing, Lalanda, mahdirostami, XORs33r, PENGUN, jasonxiale, pep7siup, lukejohn ( 1, 2, 3 ), dopeflamingo, osmanozdemir1, spacelord47, 0xLogos, fishgang ( 1, 2 ), alexfilippov314, SpicyMeatball, DanielArmstrong, zhaojohnson, aslanbek, hals, BowTiedOriole, Bobface, wangxx2026, nmirchev8, L0s1, KupiaSec, Oxsadeeq, BugzyVonBuggernaut, klau5, ke1caM, Soliditors, ZanyBonzy, EV_om, 0x111, cccz, bronze_pickaxe, Stormreckson, twcctop, deepplus, Inference ( 1, 2 ), AlexCzm, alexbabits, Kow, rvierdiiev, AS, Krace, and haxatron ( 1, 2 ) onBalanceChange does not help to claim the previous rewards, but directly resets userFeeOffset, causing the user’s unclaimed rewards to be cleared.

## Recommended Mitigation Steps

onBalanceChange should claim the previous rewards, and then resets userFeeOffset:

diff --git a/contracts/FeeSplitter.sol b/contracts/FeeSplitter.sol index bb24f02..9751902 100644 --- a/contracts/FeeSplitter.sol +++ b/contracts/FeeSplitter.sol @@ -66,8 +66,8 @@ contract FeeSplitter is Security { if (balance > 0) { uint256 owed = (data.cumulativeFeePerToken - data.userFeeOffset[account]) * balance; data.unclaimedFees[account] += owed / PRECISION; - data.userFeeOffset[account] = data.cumulativeFeePerToken; } + data.userFeeOffset[account] = data.cumulativeFeePerToken; } function getClaimableFees(address token, address account) public view returns (uint256) { @@ -94,8 +94,7 @@ contract FeeSplitter is Security { } function onBalanceChange(address token, address account) public onlyManager {

- TokenData storage data = tokensData[token]; - data.userFeeOffset[account] = data.cumulativeFeePerToken; + updateFeeCredit(token, account); if (balanceOf(token, account) > 0) userTokens[account].push(token); } andresaiello (Curves) confirmed alcueca (Judge) decreased severity to Medium and commented:

Loss of fees or rewards is Medium.

## Rejected Primary Findings

# Rejected Primary Findings: Curves

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
