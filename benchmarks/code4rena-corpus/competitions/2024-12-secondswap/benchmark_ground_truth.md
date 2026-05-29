# Benchmark Ground Truth: SecondSwap

## Accepted H/M Findings

# Accepted H/M Findings: SecondSwap

# [H-01] SecondSwap_Marketplace vesting listing order affects how much the vesting buyers can claim at a given step

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

SecondSwap_Marketplace vesting listing order affects how much the vesting buyers can claim at a given step Submitted by 0xloscar01, also found by 0xaudron, 0xc0ffEE, 0xc0ffEE, 0xEkko, 0xgremlincat, 0xNirix, 0xrex, 4rdiii, Agontuk, anchabadze, BenRai, BenRai, curly, foufrix, jkk812812, jkk812812, jsonDoge, jsonDoge, KupiaSec, KupiaSec, Kyosi, macart224, NexusAudits, nslavchev, Sabit, seerether, shaflow2, sl1, web3km, and y0ng0p3 When a vesting is listed, the vesting is transferred to the SecondSwap_VestingManager contract. With no previous listings, the contract “inherits” the stepsClaimed from the listed vesting:

- https://github.com/code-423n4/2024-12-secondswap/blob/main/contracts/SecondSwap_StepVesting.sol#L288-L290
@> if ( _vestings [ _beneficiary ].

totalAmount == 0 ) { _vestings [ _beneficiary ] = Vesting ({ @> stepsClaimed:

_stepsClaimed,...

Suppose the stepsClaimed amount is positive. In that case, further listing allocations will be mixed with the previous one, meaning the “inherited” stepsClaimed amount will be present in the listings transferred from the SecondSwap_VestingManager contract to users with no allocation that buy listings through SecondSwap_Marketplace::spotPurchase.

This condition creates two scenarios that affect how much the user can claim:

Assuming for both scenarios that there are no listings yet for a given vesting plan.

Scenario 1:

First listing has no claimedSteps Second listing has claimedSteps Since the first listing has no claimedSteps, users with no previous vestings allocation can buy any of the listings and their listing won’t have claimed steps, allowing them to claim immediately after their purchase.

Scenario 2:

First listing has claimedSteps Second listing has no claimedSteps Due to the first listing having a positive claimedSteps amount, users with no previous vesting allocations will have their vestings inherit the claimedSteps, meaning they won’t be able to claim if they are on the current step corresponding to claimedSteps.

## Recommended mitigation steps

Add a virtual total amount to the manager contract on each vesting plan deployed.

TechticalRAM (SecondSwap) confirmed

# [H-02] transferVesting creates an incorrect vesting for new users when they purchase a vesting, because stepsClaimed is the same for all sales, allowing an attacker to prematurely unlock too many tokens

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

transferVesting creates an incorrect vesting for new users when they purchase a vesting, because stepsClaimed is the same for all sales, allowing an attacker to prematurely unlock too many tokens Submitted by TheSchnilch, also found by 056Security, 0xastronatey, 0xc0ffEE, 0xDanielC, 0xDemon, 0xhuh2005, 0xHurley, 0xIconart, 0xlookman, 0xloscar01, 0xlucky, 0xluk3, 0xNirix, 0xNirix, 0xpetern, 0xRiO, 0xSolus, 4B, 4rk4rk, Abdessamed, Abhan, Amarnath, anonymousjoe, aster, aua_oo7, Bigsam, Breeje, BroRUok, BugPull, bugvorus, c0pp3rscr3w3r, chaduke, ChainSentry, chaos304, chupinexx, CipherShieldGlobal, ctmotox2, curly, Daniel526, DanielArmstrong, DharkArtz, dreamcoder, Drynooo, EaglesSecurity, ElKu, eLSeR17, elvin-a-block, escrow, escrow, eta, farismaulana, Flare, focusoor, frndz0ne, fyamf, Gosho, Hama, heylien, Hris, ITCruiser, itsabinashb, ivanov, jkk812812, jsonDoge, ka14ar, knight18695, KupiaSec, levi_104, lightoasis, lightoasis, m4k2, mahdifa, newspacexyz, NHristov, nikhil840096, nslavchev, ogKapten, oualidpro, parishill24, peanuts, Pheonix, Prosperity, queen, Rampage, ro1sharkm, rouhsamad, rouhsamad, saikumar279, Samueltroydomi, Saurabh_Singh, shaflow2, shiazinho, Shinobi, silver_eth, sl1, slavina, SmartAuditPro, smbv-1923, spuriousdragon, TheFabled, trailongoswami, tusharr1411, Uddercover,

udo, Vasquez, waydou, YouCrossTheLineAlfie, YouCrossTheLineAlfie, Z3R0, zhanmingjing, and zzebra83

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L139
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L232
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L288-L295
If a user sells their vesting on the marketplace, it will be transferred with transferVesting to the address of the VestingManager (see first GitHub-Link).

This means that all tokens sold are stored on the address of the VestingManager in the StepVesting contract. However, it is possible that all these sold vestings have different numbers of stepsClaimed. The problem is that the vesting of the VestingManager always stores only one value for stepsClaimed, which is the one taken from the first vesting that is sold.

After that, stepsClaimed cannot change because the VestingManager cannot claim. Only when the totalAmount of the vesting reaches 0, meaning when everything has been sold and there are no more listings, will a new value for stepsClaimed be set at the next listing. If a new user who doesn’t have a vesting yet buys one, they would adopt the wrong value for stepsClaimed (see second and third GitHub links).

It is quite likely that stepsClaimed is 0, as probably something was sold right at the beginning and the value hasn’t changed since then. This then leads to the user being able to directly claim a part of the tokens without waiting.

## Recommended mitigation steps

A mapping should be created where the stepsClaimed for each listing are stored so that they can be transferred correctly to the buyer.

TechticalRAM (SecondSwap) confirmed

# [H-03] In transferVesting , the grantorVesting.releaseRate is calculated incorrectly, which leads to the sender being able to unlock more tokens than were initially locked.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

transferVesting, the grantorVesting.releaseRate is calculated incorrectly, which leads to the sender being able to unlock more tokens than were initially locked.

Submitted by TheSchnilch, also found by 0xpetern, 0xStalin, ABAIKUNANBAEV, BenRai, BugPull, ChainProof, dhank, EPSec, gesha17, KupiaSec, and Rhaydden

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L230
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L178-L182
Users can sell their vestings on the marketplace. For this, the portion of the vesting that a user wants to sell is transferred to the address of the vesting contract until another user purchases the vesting.

Since this alters the seller’s vesting, the releaseRate must be recalculated. Currently, it is calculated as follows:

grantorVesting.releaseRate = grantorVesting.totalAmount / numOfSteps;.

The problem here is that it does not take into account how much of the grantorVesting.totalAmount has already been claimed. This means that the releaseRate ends up allowing the user to claim some of the tokens already claimed again.

It is important that the claiming of the stolen rewards must be done before the complete locking period ends, because otherwise the claimable function will only give the user the tokens they have not yet claimed (see second GitHub link). This would not work, as the attacker has already claimed everything by that point and the bug just works when releaseRate is used to calculate rewards.

This bug could also cause some users who were legitimately waiting for their tokens to no longer receive any, as they have been stolen and are now unavailable. It could also violate the invariant that no more than the maxSellPercent is ever sold, as this bug could allow an attacker to unlock more than the maxSellPercent.

## Recommended mitigation steps

When calculating the release rate for the seller, the steps already claimed and the amount already claimed should be taken into account:

grantorVesting.releaseRate = (grantorVesting.totalAmount - grantorVesting.amountClaimed) /(numOfSteps -grantorVesting.stepsClaimed); calvinx (SecondSwap) confirmed Medium Risk Findings (20)

# [M-01] Incorrect listing type validation bypasses enforcement of minimum purchase amount

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by fyamf, also found by 0xastronatey, 0xKann, 0xPSB, 4rk4rk, ABAIKUNANBAEV, Abdessamed, AshishLach, aster, BajagaSec, BenRai, Bloqarl, bugvorus, DanielArmstrong, Drynooo, Fitro, honey-k12, ITCruiser, itsabinashb, kimnoic, KupiaSec, lightoasis, Olami978355, oualidpro, peanuts, pontifex, pulse, queen, Sabit, Sabit, shaflow2, tusharr1411, and wickie0x

- https://github.com/code-423n4/2024-12-secondswap/blob/main/contracts/SecondSwap_Marketplace.sol#L253
Incorrect validation of the listing type allows bypassing the enforcement of _minPurchaseAmt being within the range of 0 to _amount.

## Recommended mitigation steps

The validation logic should be updated as follows:

require( - _listingType != ListingType.SINGLE || (_minPurchaseAmt > 0 && _minPurchaseAmt <= _amount), + _listingType == ListingType.SINGLE || (_minPurchaseAmt > 0 && _minPurchaseAmt <= _amount), "SS_Marketplace: Minimum Purchase Amount cannot be more than listing amount" );

- https://github.com/code-423n4/2024-12-secondswap/blob/main/contracts/SecondSwap_Marketplace.sol#L253
calvinx (SecondSwap) commented:

This looks like a valid issue. Please put as a Medium issue.

# [M-02] Listing potential can not be purchased with discounted price

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by 0xc0ffEE, also found by 0xastronatey, 0xIconart, 0xNirix, 0XRolko, agadzhalov, AshishLach, BenRai, c0pp3rscr3w3r, ChainProof, ChainSentry, CrazyMoose, farismaulana, itsabinashb, IvanAlexandur, montecristo, mrMorningstar, Olami978355, queen, Sabit, safie, Shinobi, the_code_doctor, TheFabled, trailongoswami, X0sauce, zanderbyte, and zzebra83

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_Marketplace.sol#L459-L471
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_Marketplace.sol#L413-L422
In the function SecondSwap_Marketplace::spotPurchase(), depending on the listing’s discount type, the price is computed accordingly:

function _getDiscountedPrice ( Listing storage listing, uint256 _amount ) private view returns ( uint256 ) { uint256 discountedPrice = listing.

pricePerUnit; if ( listing.

discountType == DiscountType.

LINEAR ) { discountedPrice = ( discountedPrice * ( BASE - (( _amount * listing.

discountPct ) / listing.

total ))) / BASE; } else if ( listing.

discountType == DiscountType.

FIX ) { discountedPrice = ( discountedPrice * ( BASE - listing.

discountPct )) / BASE; } return discountedPrice; } And then the baseAmount that the buyer needs to pay is calculated from the discounted price. Although there is a check to enforce listing value is not too small with the original price, but it still can be too small with the discounted price because indeed the discounted price is lower than the original price. So in that case, the buyer won’t be able to purchase listed sale.

function _handleTransfers ( Listing storage listing, uint256 _amount, uint256 discountedPrice, uint256 bfee, uint256 sfee, address _referral ) private returns ( uint256 buyerFeeTotal, uint256 sellerFeeTotal, uint256 referralFeeCost ) { @> uint256 baseAmount = ( _amount * discountedPrice ) / uint256 ( 10 ** ( IERC20Extended ( address ( IVestingManager ( IMarketplaceSetting ( marketplaceSetting ).

vestingManager ()).

getVestingTokenAddress ( listing.

vestingPlan ) ).

decimals () ) ); // 3.1. Rounding issue leads to total drain of vesting entries @> require ( baseAmount > 0, "SS_Marketplace: Amount too little" ); // 3.1. Rounding issue leads to total drain of vesting entries...

Example with a simple vulnerable path:

Alice lists vesting with amount = 1e15 (assume the token has 18 decimals), currency is USDT, price = 1200 ($0.0012), with fixed discount = 20% and the listing type is Single.

Bob tries to purchase Alice’s sale, but the transaction fails because baseAmount is 0 in this case:

baseAmount = 1e15 * 1200 * 80% / 1e18 = 0.

Impacts:

Users are potentially unable to purchase discounted vestings

## Recommended mitigation steps

Consider updating the check for baseAmount in function listVesting() to take discount into account.

TechticalRAM (SecondSwap) confirmed

# [M-03] Missing option to remove tokens from the isTokenSupport mapping can result in huge financial loss for users and the protocol

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

isTokenSupport mapping can result in huge financial loss for users and the protocol Submitted by BenRai, also found by 0xAkira, 0xrex, 0xStalin, BajagaSec, Bryan_Conquer, Taiger, and zanderbyte

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_Marketplace.sol#L205-L218
Because there is no option for the admin to remove a token from the isTokenSupport mapping, a depeg of a whitelisted token can lead to significant financial loss for users and the protocol.

## Recommended Mitigation Steps

Add an option for the admin to remove currencies from the whitelist. This way, no new listings can be created with a depegged currency. To protect the vestings already listed with the bad currency, make sure to check if the currency used for a listing is still on the whitelist before executing a sale. This way, sellers of the impacted listings are protected from selling their vestings for worthless currency and can delist their listings once they become aware of the depeg.

Koolex (judge) commented:

Validator’s comment:

The supported tokens are under protocol team’s review, we can expect that most widely used token such as ETH/USDC/USDC to be included as currency token. Though it’s a good idea to have a quit design, QA is proper to this issue.

My view after further evaluation: Since all ERC20 tokens are supported, depegged currency risk is still there, even for USDC which actually dropped under 1$ about a year ago. Therefore, at this point, I believe this can be Medium.

calvinx (SecondSwap) commented:

In our initial design, we will only use liquid stables, i.e. USDT and USDC. In a depeg scenario, we can freeze listing and recommend sellers to remove their listings. We will include a fix.

# [M-04] Creator of one vesting plan can affect vesting plans created by other users.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by sl1, also found by 0xDemon, 0xGondar, 0xKann, ABAIKUNANBAEV, Abhan, boredpukar, ctmotox2, ctmotox2, curly, dhank, EPSec, fyamf, fyamf, m4k2, peanuts, pulse, rspadi, Shinobi, spuriousdragon, yuza101, and zanderbyte

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingDeployer.sol#L141-L144
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingDeployer.sol#L176-L183
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingDeployer.sol#L218-L231
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingDeployer.sol#L193-L206
Vesting plans are created by token issuers in the VestingDeployer contract. When a vesting plan is created, a new StepVesting contract is deployed.

SecondSwap_VestingDeployer.sol#L119-L129 address newVesting = address ( new SecondSwap_StepVesting ( msg.

sender, manager, IERC20 ( tokenAddress ), startTime, endTime, steps, address ( this ) ); This contract address will be used by token issuers to manage their vesting plans. However, currently it’s possible that a creator of one vesting plan can affect vesting plans created by other users.

Token issuers are set by the admin in the setTokenOwner() function.

SecondSwap_VestingDeployer.sol#L141-L144 function setTokenOwner ( address token, address _owner ) external onlyAdmin { require ( _tokenOwner [ _owner ] == address ( 0 ), "SS_VestingDeployer: Existing token have owner" ); _tokenOwner [ _owner ] = token; } As can be seen, it’s possible for a token to have multiple owners, as the mapping used is owner => token instead of token => owner. Now if one token has multiple owners and there are 2 vesting plans, creator of vesting plan A can influence vesting plan B and vice versa.

createVesting() and createVestings() functions check that token that is linked to the msg.sender is the same as the token of the vesting plan, which in this case will be true regardless of the fact that msg.sender is not the creator of said vesting plan.

SecondSwap_VestingDeployer.sol#L176-L183 require ( _tokenOwner [ msg.

sender ] == address ( SecondSwap_StepVesting ( _stepVesting ).

token ()), "SS_VestingDeployer: caller is not the token owner" ); But when _createVesting() function of the StepVesting contract will be invoked, it will transfer funds not from the msg.sender but from the actual creator of the vesting plan.

SecondSwap_StepVesting.sol#L306-L308 if (!

_isInternal ) { token.

safeTransferFrom ( tokenIssuer, address ( this ), _totalAmount ); } transferVesting() function is also vulnerable to that issue because it uses the same check as createVesting().

SecondSwap_VestingDeployer.sol#L218-L228 function transferVesting ( address _grantor, address _beneficiary, uint256 _amount, address _stepVesting, string memory _transactionId ) external { require ( _tokenOwner [ msg.

sender ] == address ( SecondSwap_StepVesting ( _stepVesting ).

token ()), "SS_VestingDeployer: caller is not the token owner" );

## Impact

Creator of one vesting plan can influence vesting plans created by other users.

## Recommended Mitigation

Store the address of the vesting plan’s creator in a mapping(address vestingPlan => address creator) and check if the msg.sender is the actual creator of the vesting plan.

Koolex (judge) commented:

But when _createVesting() function of the StepVesting contract will be invoked, it will transfer funds not from the msg.sender but from the actual creator of the vesting plan.

I believe this can be Medium since the impact is high as functionality is broken (not working as intended), and a possible loss of funds + a low likelihood.

# [M-05] Price Granularity Limited by Payment Token Decimals: Cannot List Tokens Cheaper than 0.000001 USDT

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by 0xNirix, also found by KupiaSec

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_Marketplace.sol#L256
The SecondSwap marketplace enforces a minimum price floor based on the payment token’s smallest unit which will commonly be USDT. This creates a limitation where tokens cannot be listed for less than 0.000001 USDT (or equivalent smallest unit of other 6 decimal payment tokens).

Root Cause:

pricePerUnit must be greater than 0 pricePerUnit represents price in payment token’s smallest units for 1 vesting token.

For USDT (6 decimals), minimum pricePerUnit is 1 (0.000001 USDT) Prices lower than this cannot be represented Impact:

Cannot list very low-value tokens at their true market price Could prevent legitimate trading of extremely low-value tokens This limitation will be particularly impactful as memecoins with such low values are fairly common.

bobwong (SecondSwap) acknowledged

# [M-06] Underflow in claimable DOSing claim Function

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

claimable DOSing claim Function Submitted by BugPull, also found by 0xrex, 0XRolko, BugPull, Drynooo, KupiaSec, montecristo, Ryonen, SmartAuditPro, and TheFabled

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L172-L181
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L196-L199
A user who buys vesting tokens after fully claiming their allocation at the end of the vesting period will be unable to claim the newly acquired tokens.

in some cases due to rounding issues the - stepsClaimed > numOfSteps In claimable function claimableSteps is calculated as follow:

174:@> uint256 claimableSteps = currentStep - vesting.

stepsClaimed; For that the user cannot claim newly purchased amounts.

## Impact

The claim function becomes unavailable.

The funds are stuck.

Users who purchase additional amounts are unable to claim their tokens.

## Recommended Mitigation Steps

Apply the following correction to the claimable function:

-- uint256 claimableSteps = currentStep - vesting.stepsClaimed; ++ uint256 claimableSteps = currentStep > vesting.stepsClaimed ? 0: currentStep - vesting.stepsClaimed; Koolex (judge) commented:

As per the sponsor, it is a very rare case. Based on this and the input provided, this is a Medium severity. Setting this as the main submission.

# [M-07] buyFee And sellFee Should Be Known Before Purchase

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

buyFee And sellFee Should Be Known Before Purchase Submitted by EPSec, also found by 0xhuh2005, BenRai, KupiaSec, rouhsamad, sl1, and typicalHuman

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_Marketplace.sol#L240
The platform allows the buyFee and sellFee parameters for a vesting plan to be modified after a listing is created. This creates a significant issue in terms of transparency and predictability for users engaging in transactions.

Impact on Users Uncertainty for Buyers and Sellers:

Both buyers and sellers cannot reliably determine the exact fees associated with a transaction until the spotPurchase function is executed. This lack of transparency diminishes user confidence in the platform.

Financial Discrepancies for Sellers:

Sellers may receive less revenue than anticipated if the seller fee ( sellFee ) is increased after the listing is created. This directly impacts their earnings and could lead to dissatisfaction or distrust in the platform’s fee structure.

## Recommended mitigation steps

Consider two additional parameters to be added for the listing:

(uint256 bfee, uint256 sfee) = _getFees(_vestingPlan); listings[_vestingPlan][listingId] = Listing({ seller: msg.sender, total: _amount, balance: _amount, pricePerUnit: _price, listingType: _listingType, discountType: _discountType, discountPct: _discountPct, listTime: block.timestamp, whitelist: whitelistAddress, currency: _currency, minPurchaseAmt: _minPurchaseAmt, status: Status.LIST, vestingPlan: _vestingPlan, + buyerFee: bfee, + sellerFee: sfee }); emit Listed(_vestingPlan, listingId); } Also make the changes to the Listing struct and spotPurchase to use the correct fees.

TechticalRAM (SecondSwap) acknowledged calvinx (SecondSwap) commented:

We will show this in UI as fees can be negotiated and set later Koolex (judge) commented:

I think the issue here is, no protection for the user in case fee changed.

# [M-08] Outdated penalty fee gets charged if the penalty fee has changed since listing

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by 0xrex, also found by 0xAkira and aua_oo7

- https://github.com/code-423n4/2024-12-secondswap/blob/main/contracts/SecondSwap_Marketplace.sol#L360
Minimum listing duration is currently set at 2 mins, at which point a listing cancellation will no longer incur the fee when unlisted less than 2 minutes since it got listed. However, users can be charged an outdated fee which is more or less the initial fee they expected to pay.

## Recommended mitigation steps

Having a cache of the fee stored in the listing struct of the listing ID would be sufficient to figure out which fee to charge them.

Koolex (judge) commented:

This should be Medium since the user should be protected from such cases. The user should be able to make an informed decision with a protection mechanism in place. Other than that, the responsibility is on the user.

# [M-09] Users can prevent being reallocated by listing to marketplace

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by 0xc0ffEE, also found by attentioniayn, ChainProof, falconhoof, rouhsamad, sl1, and TheKhans

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L216-L235
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L139
The token issuer has the ability to change vesting allocation. However, a user can prevent his vesting from being allocated by listing his vesting to the marketplace.

The function SecondSwap_StepVesting::transferVesting() can be used by token issuer to transfer vesting, effectively reallocating vestings. By listing the vesting to marketplace, seller’s allocated amount is sent to VestingManager contract, which can make the token issuer unable to reallocate his vesting directly (due to available amount check). Indeed, if the token issuer decides to reallocate that wanted amount from VestingManager, then this can cause the marketplace to be insolvent.

function transferVesting ( address _grantor, address _beneficiary, uint256 _amount ) external { require ( msg.

sender == tokenIssuer || msg.

sender == manager || msg.

sender == vestingDeployer, "SS_StepVesting: unauthorized" ); require ( _beneficiary != address ( 0 ), "SS_StepVesting: beneficiary is zero" ); require ( _amount > 0, "SS_StepVesting: amount is zero" ); Vesting storage grantorVesting = _vestings [ _grantor ]; @> require ( grantorVesting.

totalAmount - grantorVesting.

amountClaimed >= _amount, "SS_StepVesting: insufficient balance" ); // 3.8. Claimed amount not checked in transferVesting function @> grantorVesting.

totalAmount -= _amount; grantorVesting.

releaseRate = grantorVesting.

totalAmount / numOfSteps; _createVesting ( _beneficiary, _amount, grantorVesting.

stepsClaimed, true ); emit VestingTransferred ( _grantor, _beneficiary, _amount ); } function listVesting ( address seller, address plan, uint256 amount ) external onlyMarketplace { require ( vestingSettings [ plan ].

sellable, "vesting not sellable" ); require ( SecondSwap_Vesting ( plan ).

available ( seller ) >= amount, "SS_VestingManager: insufficient availablility" ); Allocation storage userAllocation = allocations [ seller ][ plan ]; uint256 sellLimit = userAllocation.

bought; uint256 currentAlloc = SecondSwap_Vesting ( plan ).

total ( seller ); if ( currentAlloc + userAllocation.

sold > userAllocation.

bought ) { sellLimit += (( currentAlloc + userAllocation.

sold - userAllocation.

bought ) * vestingSettings [ plan ].

maxSellPercent ) / BASE; } userAllocation.

sold += amount; require ( userAllocation.

sold <= sellLimit, "SS_VestingManager: cannot list more than max sell percent" ); @> SecondSwap_Vesting ( plan ).

transferVesting ( seller, address ( this ), amount ); } Note that: This attack vector can be done by front-running, since the codebase is deployed to Ethereum.

Impacts:

Token issuer can not reallocate as expected. At least, the total allocated amount can not be reallocated, depending on the max sell percent.

## Recommended mitigation steps

Consider adding trusted role to unlist from marketplace, so that the reallocation can be handled completely.

bobwong (SecondSwap) acknowledged

# [M-10] Tokens that have already been vested can be transferred from a user.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by sl1

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L216-L235
As stated by the contest page, token issuer must be able to reallocate vesting allocations from one user to another. It can be done via transferVesting() function of the StepVesting contract.

SecondSwap_StepVesting.sol#L224-L232 require ( grantorVesting.

totalAmount - grantorVesting.

amountClaimed >= _amount, "SS_StepVesting: insufficient balance" ); grantorVesting.

totalAmount -= _amount; grantorVesting.

releaseRate = grantorVesting.

totalAmount / numOfSteps; _createVesting ( _beneficiary, _amount, grantorVesting.

stepsClaimed, true ); As can be seen, the function ensures that amount transferred is not greater than amount of tokens to vest left after some of them have been claimed.

However, it does not account for tokens that have already been vested, but remain unclaimed by a user. The moment tokens are vested, they cease to be part of the vesting process because the conditions for their release have already been met. Vested but unclaimed tokens are effectively owned by the beneficiary, but remain unclaimed. This essentially allows token issuer to transfer tokens owned by the user instead of reallocation a part of the vesting schedule.

## Impact

Token issuer has an ability to transfer out tokens owner by users instead of reallocation vesting schedule.

## Recommended Mitigation

transferVesting() should account for tokens that have already been vested but remain unclaimed.

TechticalRAM (SecondSwap) acknowledged

# [M-11] maxSellPercent can be buypassed by selling previously bought vestings at a later time

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

maxSellPercent can be buypassed by selling previously bought vestings at a later time Submitted by BenRai, also found by ABAIKUNANBAEV, Agontuk, AshishLach, attentioniayn, chupinexx, EaglesSecurity, escrow, franfran20, gesha17, itsabinashb, jsonDoge, NexusAudits, nikhil840096, nitinaimshigh, parishill24, Rhaydden, Samueltroydomi, Saurabh_Singh, typicalHuman, typicalHuman, wickie0x, and zzebra83

- https://github.com/code-423n4/2024-12-secondswap/blob/b9497bcf5100046a169276cb6b351ebc0eddc2cc/contracts/SecondSwap_VestingManager.sol#L127-L134
Because the maxSellPercent can be bypassed by selling previously bought vestings at a later time, the core functionality to limit the amount of unvested tokens which can be sold is broken.

## Recommended Mitigation Steps

To prevent more locked tokens to be sellable than specified in ´maxSellPercent´ consider adding a stepsBought to the Allocation struct to be able to adjust the bought value according to the steps already claimed by the user:

struct Allocation { uint256 bought; + uint256 stepsBought; uint256 sold; } The stepsBought value would be adjusted each time a user buys or sells a vesting and would be set to the current stepsClaimed value of the buyer. In addition, for sells, the bought amount would also need to be reduced by the already claimed amount.

Buy a vesting:

Buyer buys 100 tokens and the stepsBought value is set to his current stepsClaimed value. This way we know for which steps the bought value will be claimable. e.g stepsBought is 5 => bought value was allocated to step 6 to 10 Sell a vesting:

Buyer claims 2 more periods and wants to sell tokens The bought part of the sellLimit is determined by calculating the bought amount for each step and reducing the original bought amount by the steps already claimed:

uint256 boughtAmountPerStep = userAllocation.

bought / ( stepsBought - SecondSwap_Vesting ( plan ).

numOfSteps ()); ` uint256 claimedStepsSinceBuy = SecondSwap_Vesting(plan)._vestings(seller).stepsClaimed – stepsBought; uint256 sellLimit = userAllocation.bought – (boughtAmountPerStep * claimedStepsSinceBuy) For this to work:

The bought amount must be reduced to the calculated sellLimit We need to sell bought allocations first before selling own allocations. Therefore the bought amount must be reduced by the amount which should be sold. Only when the bought amount reaches 0, the sold amount should be increased.

The result would be that the sold amount represents only the amount a user sold of his initial allocation. In addition the Allocation struct also needs a stepsSold variable which can be used to adjust/reduce the sold amount according to the claimed steps of the seller/buyer.

TechticalRAM (SecondSwap) confirmed

# [M-12] Unauthorized increase of maxSellPercent

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

maxSellPercent Submitted by BenRai, also found by NexusAudits and seerether

- https://github.com/code-423n4/2024-12-secondswap/blob/b9497bcf5100046a169276cb6b351ebc0eddc2cc/contracts/SecondSwap_VestingManager.sol#L194-L198
- https://github.com/code-423n4/2024-12-secondswap/blob/b9497bcf5100046a169276cb6b351ebc0eddc2cc/contracts/SecondSwap_VestingManager.sol#L179
This issue allows the maxSellPercent to be increased to 20% against the will of the token issuer. This will result in users being able to sell tokens even when the issuer intended to prevent selling.

## Recommended Mitigation Steps

To mitigate this issue, a new variable initiated should be added to the vesting settings. This variable will track whether the vesting has been initialized. The setSellable function should only update the maxSellPercent if initiated is false which should only be when the vesting is initial created.

bobwong (SecondSwap) confirmed

# [M-13] MarketPlace Change In Vesting Manager, Leads To Loss Of Previous MarketPlace Listing

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by franfran20, also found by 0xLasadie, BenRai, BenRai, EPSec, and franfran20

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L204
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L121
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L149
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L161
When interacting with the MarketPlace contract and vesting listings, the MarketPlace contract calls the VestingManager contract (using the address gotten from the MarketplaceSetting contract) which calls the StepVesting contract itself to transfer vestings from one address to another.

The VestingManager contract contains the setMarketplace function which is in place in case the MarketPlace contract needs to be changed and redeployed instead of an upgrade (upgrades to the MarketPlace contract occur through the proxy admin, so this function is to change the proxy entirely). When a new MarketPlace contract is set, all previous listings in the marketplace remain stuck, unlistable or inaccessible by the user who listed them, leading to loss of vested assets, simply because the VestingManager is no longer connected to that instance of the marketplace.

## Recommended mitigation steps

Provide a way to allow after a change in the marketplace contract, the user to be able to remove their vested listings and transfer it back to their address from the previous marketplace.

TechticalRAM (SecondSwap) acknowledged

# [M-14] Incorrect referral fee calculations

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by zanderbyte, also found by 0xlucky, 0xNirix, 0XRolko, 0xStalin, Abhan, Agontuk, aster, BajagaSec, BenRai, c0pp3rscr3w3r, ChainSentry, DanielArmstrong, DharkArtz, Drynooo, elvin-a-block, EPSec, franfran20, Gaurav2811, Gaurav2811, Gosho, inh3l, JustUzair, JustUzair, KupiaSec, Lamsy, macart224, macart224, newspacexyz, nslavchev, ogKapten, oualidpro, oualidpro, Rampage, Rhaydden, rspadi, Ryonen, Sabit, sl1, smbv-1923, TheFabled, TheKhans, Uddercover, udo, udo, wickie0x, xKeywordx, y0ng0p3, YouCrossTheLineAlfie, and zia_d_k

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_Marketplace.sol#L480-#L483
When a purchase of listed tokens is performed, the caller can provide a referral address. This address should receive a referral fee as a reward for introducing users to the project. According to the dev team, the referral payment will be done off-chain.

However, in the current implementation, the referralFeeCost calculations are incorrect, which results in much higher fees (ou to 90% of buyersFeeTotal ) for the referral than expected.

## Recommended mitigation steps

The referralFeeCost should be calculated as a percentage of buyerFeeTotal.

The correct calculation should be:

referralFeeCost = - buyerFeeTotal - - (baseAmount * bfee * IMarketplaceSetting(marketplaceSetting).referralFee()) / - (BASE * BASE); + buyerFeeTotal * IMarketplaceSetting(marketplaceSetting).referralFee() / BASE This will result in referralFee = 2.5e6 (exactly 10% of buyerFeeTotal )

# [M-15] Missing sellable check in completePurchase will cause a user to buy a token marked as unsellable by S2ADMIN if it was listed beforehand

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by Bigsam, also found by attentioniayn, Benterkiii, farismaulana, foufrix, hubble, knight18695, and spuriousdragon

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L167-L186
- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_VestingManager.sol#L161-L164
A token marked sellable can be purchased because of the absence of the sellable check when completing a spot purchase.

## Recommended mitigation steps

Add a sellable check in the completePurchase function has done in the listVesting function TechticalRAM (SecondSwap) confirmed

# [M-16] Possible DoS scenario when transferring vests to another address

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by Shubham, also found by ABAIKUNANBAEV

- https://github.com/code-423n4/2024-12-secondswap/blob/main/contracts/SecondSwap_StepVesting.sol#L225
Vestings can be transferred to another address by a trusted authority. All the necessary parameters are recalculated like the totalAmount & releaseRate for the current owner for the vesting.

However it is possible that a call to transfer the vesting might be frontrun where the owner of the original vesting claims their token resulting in an overall revert.

## Recommended mitigation steps

A way would be to pause claiming of tokens when transferring to avoid this issue & unpause later.

calvinx (SecondSwap) commented:

This is a low likelihood scenario.

Koolex (judge) commented:

Due to the low likelihood, this could be low/med.

Clarify in one or two statements what is the impact, and why would you think it is high?

Otherwise, this will be set as low.

ABAIKUNANBAEV (warden) commented:

@Koolex I don’t believe that it’s a high - I think it’s a med as it was stated by the protocol that vesting grantor has to be able to freely transfer the vestings - in this scenario, he can clearly face DoS And it’s a very high probability as users can use the strategy of not claiming the funds to then DoS the grantor Koolex (judge) commented:

Taking into account the input above, this is a valid Medium

# [M-17] Rounding error in stepDuration calculations.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by sl1, also found by 056Security, 056Security, 0xEkko, 0xrex, agadzhalov, codertjay, Drynooo, Fon, gesha17, IzuMan, ka14ar, KiteWeb3, macart224, montecristo, NexusAudits, NexusAudits, oualidpro, pulse, rouhsamad, TheKhans, Viquetour, yuza101, and Z3R0

- https://github.com/code-423n4/2024-12-secondswap/blob/214849c3517eb26b31fe194bceae65cb0f52d2c0/contracts/SecondSwap_StepVesting.sol#L133
When deploying a vesting plan, token issuer can specify the end time of the schedule and number of steps over which tokens should be released.

StepVesting calculates the duration of each distinctive step by dividing the duration of the schedule by number of steps.

SecondSwap_StepVesting.sol#L131-L133 endTime = _endTime; numOfSteps = _numOfSteps; stepDuration = ( _endTime - _startTime ) / _numOfSteps; However, currently it’s possible for calculations to round down, which could lead to multiple problems.

First, consider a scenario where calculations of stepDuration round down to 0. This will result in inability to claim funds from the StepVesting contract. In order to claim tokens from the vesting schedule, a user must call claim() function of the contract, which in turn will call claimable() to get the amount of tokens currently available for claim.

SecondSwap_StepVesting.sol#L193-L194 function claim () external { ( uint256 claimableAmount, uint256 claimableSteps ) = claimable ( msg.

sender ); When claimable() is invoked, it will try to calculate current step by dividing elapsed time by duration of the step, which will revert as solidity does not support division by 0.

SecondSwap_StepVesting.sol#L173 uint256 currentStep = elapsedTime / stepDuration; Secondly, because of the rounding, it’s possible that vesting will be completed earlier than the schedule. Consider a 3 month vesting plan which equals 7884000 seconds and the number of steps is 1 000 000. The step duration will be calculated as 7884000 / 1000000 = 7.884, which will round down to 7. Now the actual time it will take to complete the schedule is 7 * 1000000 = 7000000 seconds, which is 81 days, meaning that vesting is completed roughly 10 days earlier than the schedule.

## Impact

DoS of the claim() function of the StepVesting contract and premature ending of the vesting schedule.

## Recommended Mitigation

When calculating stepDuration revert when _endTime - _startTime is not perfectly divisible by _numOfSteps.

sl1 (warden) commented:

Hey Koolex, thank you for judging!

The only reason why I submitted this finding is because contest page mentioned that sponsor’s concerns were: “What would happen if the amount of locked tokens, duration or number of cycles are on the reaches extremes?“.

And even though I don’t necessarily think that 1 000 000 steps is a super extreme value, but even if we consider it as such, given the attack ideas section of the contest page, I believe this issue should be valid.

Koolex (judge) commented:

@sl1 First, consider a scenario where calculations of stepDuration round down to 0.

Can you explain the possibility of such scenario? what would be the setting by token issuer?

Also, can you give other possibilities with less steps and less vesting plan?

At this moment, it’s duped to F-121. Although, this might be selected as main.

Note: F-121 is S-867 at this moment.

sl1 (warden) commented:

For stepDuration to round down to 0, the result of a calculation should be a 0.999 value or anything less. For example, if the duration of the vesting schedule is 86400 and number of steps is 86401, the calculations will round down to 0. Personally, I think this is less likely than the second scenario provided, so I want to focus on that more.

When calculating stepDuration we use this formula:

(_endTime - _startTime) / _numOfSteps;. Here, the maximum “loss” in seconds can be at most 1 second per step (since at most we can round down from 0.99). If the duration is 190 000 and num of steps is 100 000, the result value will be 1 second per steps (instead of 1.9 seconds per step), which means that a vesting will be completed 0.9 * 100 000 seconds earlier, which in this case is roughly 1 day. The same is true for any other scenario where calculations round down, the only difference would be the magnitude of the issue as there could be a rounding from a lower value, such as 1.4 rounding down to 1, the vesting in this case would be completed 0.4 * 100 000 seconds earlier.

I think I don’t have much to add further to the discussion and will leave the decision up to you, thank you!

0xrex (warden) commented:

Hi @Koolex, I would like to stress that both scenarios 1 & 2, pointed out by @sl1 are both very likely to occur in the same weight. I have already left a simple explainer for scenario 1 where the stepDuration does round to 0, but I’ll just grab it from the original report and chunk it here to follow up:

jvotoken protocol owner gets whitelisted by the SecondSwap team to deploy a vesting contract for the jvo token jvotoken protocol owner calls deployVesting with the following startTime & endTime: startTime = 1733902691, endTime = 1733902692.

jvotoken protocol then proceeds to vest 100e18 of jvo tokens to user A. user A’s releaseRate will be 100e18 / 24 = 4,166,666,666,666,666,666 The entire duration of the vesting elapses. User calls claim. currentTime returned would be the endTime 1733902692. elapsedTime would be 1 because (1733902692 - 1733902691). and in the next line, the function would revert because of a division by 0.

function deployVesting ( address tokenAddress, uint256 startTime, uint256 endTime, uint256 steps, string memory vestingId ) external {...

// @audit not enough validation @> require ( startTime < endTime, "SS_VestingDeployer: start time must be before end time" ); require ( steps > 0, "SS_VestingDeployer: steps must be greater than 0" ); require ( manager != address ( 0 ), "SS_VestingDeployer: manager not set" ); address newVesting = address ( new SecondSwap_StepVesting ( msg.

sender, // jvo token protocol manager, IERC20 ( tokenAddress ), // jvo token startTime, // 1733902691 endTime, // 1733902692 steps, // 24 address ( this ) ); IVestingManager ( manager ).

setSellable ( newVesting, true ); emit VestingDeployed ( tokenAddress, newVesting, vestingId ); } function claimable ( address _beneficiary ) public view returns ( uint256, uint256 ) { Vesting memory vesting = _vestings [ _beneficiary ]; if ( vesting.

totalAmount == 0 ) { return ( 0, 0 ); } @> uint256 currentTime = Math.

min ( block.

timestamp, endTime ); if ( currentTime < startTime ) { return ( 0, 0 ); } uint256 elapsedTime = currentTime - startTime; // 1 @> uint256 currentStep = elapsedTime / stepDuration; // 1 / 0 uint256 claimableSteps = currentStep - vesting.

stepsClaimed; uint256 claimableAmount; if ( vesting.

stepsClaimed + claimableSteps >= numOfSteps ) { //[BUG FIX] user can buy more than they are allocated claimableAmount = vesting.

totalAmount - vesting.

amountClaimed; return ( claimableAmount, claimableSteps ); } claimableAmount = vesting.

releaseRate * claimableSteps; return ( claimableAmount, claimableSteps ); } Thus, I believe @sl1 is right and these issue sets should be a higher severity than LOW.

Koolex (judge) commented:

Based on the input above, this is a valid Med. This also will be the main submission.

# [M-18] Unlisting a vesting after seller has claimed additional steps locks tokens which should have been claimable already

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by BenRai

- https://github.com/code-423n4/2024-12-secondswap/blob/b9497bcf5100046a169276cb6b351ebc0eddc2cc/contracts/SecondSwap_VestingManager.sol#L149-L152
Because unlisted vestings are equally distributed to the unclaimed steps of the seller, if a seller unlists a vesting after he claimed additional steps, tokens which should have been claimable already are locked again and the unlocking schedule is disrupted.

## Recommended Mitigation Steps

To prevent locking tokens which should be claimable when a vesting is unlisted, consider adding the lastStepsClaimedSeller variable listing info indicating the last step the seller has claimed when creating the listing:

struct Listing { address seller; uint256 total; uint256 balance; uint256 pricePerUnit; ListingType listingType; DiscountType discountType; uint256 discountPct; uint256 listTime; + uint256 lastStepsClaimedSeller; address whitelist; uint256 minPurchaseAmt; Status status; address currency; address vestingPlan; } Also, an additional info about claimable tokens should be added to the vesting information:

struct Vesting { uint256 stepsClaimed; uint256 amountClaimed; uint256 releaseRate; uint256 totalAmount; + uint256 claimableAmount; } When the seller unlists a vesting, his last stepsClaimed is compared to lastStepsClaimedSeller of the listing. If they differ, they are handled like this:

An amount of refunded tokens proportional to the steps claimed since the vesting was listed are allocated to claimableAmount of the seller and can be claimed immediately since they have been unlocked already:

claimableAmount = refundedTokens * ( stepsClaimed - lastStepsClaimedSeller ) / numOfSteps releaseRate of the seller is adjusted using the remaining refunded tokens.

calvinx (SecondSwap) commented:

The token’s should be claimable and not locked. This is a valid issue.

Koolex (judge) commented:

Based on above, a valid Med.

# [M-19] Large number of steps in a vesting may lead to loss of beneficiary funds or uneven vesting distribution

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by gesha17

- https://github.com/code-423n4/2024-12-secondswap/blob/main/contracts/SecondSwap_StepVesting.sol#L298
Some token owners may create StepVesting contracts with very large numbers of steps, to simulate a continous vesting. This can lead to two edge cases for the releaseRate of a beneficiary:

Consider a scenario where a vesting is created that has less tokens than there are number of steps in the plan. Then the releaseRate would be calculated to 0, so users will essentially lose their vestings. The severity of this would depend on how many steps there are and how much the vesting is. Suppose there are 100000000 steps in a vesting plan with duration 1 year. A user receives a vesting for 90e6 USDC. So the releaseRate will be calculated as 90e6/100000000 = 0.9, which is rounded down to 0. So the user will lose his tokens.

This can also lead to a situation where the release rate becomes 1 and a large amount of the tokens get vested at the very last step, creating a very uneven vesting distribution. A requirement is that the number of steps is more than tokenAmount/2. So if the amount of steps is say 10000000 USDC or 10e6 USDC, the number of steps has to be more than 5e6 USDC for the bug to occur. So likelihood is very low.

This is only a real concern with tokens that have very low decimals, which are in scope as per the competition page.

Protocol function is impacted as beneficiaries will not get their liquidity released correctly, so their funds will essentially be temporarily locked.

## Recommended mitigation steps

Mitigation is non-trivial as it would require partially changing protocol design - instead of calculating release rate, calculate release amount based on how many steps are passed in the claimable() function.

calvinx (SecondSwap) commented:

This is a low likelihood scenario.

Koolex (judge) commented:

Due to the low likelihood, this could be low/med.

Clarify in one or two statements what is the impact, and why would you think it is high?

Otherwise, this will be set as low.

gesha17 (warden) commented:

Hello Judge, The impact is high because users will basically lose their vested tokens since releaseRate is rounded down to 0. I don’t think likelihood can be set to low either because a malicious user can abuse this bug to create listings with carefully selected amount such that releaseRate rounds down to 0. An honest user would buy the vestings but then find out that in fact they bought nothing.

Koolex (judge) commented:

Based on the input above, this is a valid medium.

# [M-20] maxSellPercent will be broken when a vesting is delisted after a seller has claimed additional steps

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Finding ID:** M-20
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-secondswap
- **Source snapshot:** competitions/2024-12-secondswap/final_report.html

Submitted by BenRai

- https://github.com/code-423n4/2024-12-secondswap/blob/b9497bcf5100046a169276cb6b351ebc0eddc2cc/contracts/SecondSwap_VestingManager.sol#L149-L152
- https://github.com/code-423n4/2024-12-secondswap/blob/b9497bcf5100046a169276cb6b351ebc0eddc2cc/contracts/SecondSwap_VestingManager.sol#L130-L134
When a listed vesting is delisted after the seller has claimed additional steps, the full listed amount is deducted from the sold value of the seller’s Allocation data. This allows him to sell tokens which should have already been unlocked and claimable which breaks a core functionality of the protocol, namely the sell limit intended by the value set for maxSellPercent.

## Recommended Mitigation Steps

To prevent sellers being able to sell more tokens than specified in ´maxSellPercent´ after unlisting a vesting, consider adding a stepsClaimedSeller vriable to the Listing struct indicating the last step the seller has claimed when he created the listing:

struct Listing { address seller; + uint256 stepsClaimedSeller; uint256 total; uint256 balance; uint256 pricePerUnit; ListingType listingType; DiscountType discountType; uint256 discountPct; uint256 listTime; address whitelist; uint256 minPurchaseAmt; Status status; address currency; address vestingPlan; } In addition, the variable amountClaimable needs to be added to the Vesting struct:

struct Vesting { uint256 stepsClaimed; uint256 amountClaimed; + uint256 amountClaimable; uint256 releaseRate; uint256 totalAmount; } Once the seller unlists a listing, the value of stepsClaimedSeller is compared to the seller’s current stepsClaimed. If the seller has claimed additional steps since he initially listed the vesting, the number of steps is calculated and a proportional amount of the refunded tokens is added to amountClaimable making them claimable instantly:

amountToAdd = refundedAmount * (stepsClaimed – stepsClaimedSeller) / totalSteps To ensure the calculation of available tokens stays accurate, the same amount needs to be added to amountClaimed.

The remaining amount of refunded tokens is deducted from the sold amount and is available for sale again.

When claiming tokens the amount saved in amountClaimable is added to the amount transfered to the user and amountClaimable is set to 0.

sl1 (warden) commented:

I’m sorry for the late comment, but I think both this issue and S-140 are more fit for the medium severity, as assets are not at risk/no loss of funds (which is a criteria for medium severity in C4 docs), the only impact is user being able to sell more tokens.

calvinx (SecondSwap) commented:

This looks like a medium issue, please include this. We will explore for ways to do 1 fix for both issues.

Koolex (judge) commented:

Considering the input above from the wardens and the sponsor, this is a valid Medium.

## Rejected Primary Findings

# Rejected Primary Findings: SecondSwap

# Changing minListingDuration after a listing Is created alters unlisting penalties for ongoing listings

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-125
- **Submitter:** 0xastronatey
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-125
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-125.txt

## Brief Summary

The marketplace contract uses the current value of minListingDuration from MarketplaceSetting contract at the time of unlisting, rather than the value that was in place when the listing was originally created. This means if minListingDuration is modified mid-way, sellers face new and unexpected conditions when they try to unlist. They might avoid or incur penalties they never anticipated, simply because the rules changed after they listed their tokens. if ((listing.listTime + IMarketplaceSetting(marketplaceSetting).minListingDuration()) > block.timestamp) { //.. Apply penalty fee } Here, minListingDuration() is fetched dynamically at unlisting time rather than using a stored value from the...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Potential Reentrancy Due to External Calls Before State Updates

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-80
- **Submitter:** TheFabled
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-80
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-80.txt

## Brief Summary

In the spotPurchase function, the contract makes external token transfers before updating critical state variables, such as listing.balance and listing.status: _handleTransfers(listing, _amount, discountedPrice, bfee, sfee, _referral); // Update listing status listing.balance -= _amount; listing.status = listing.balance == 0 ? Status.SOLDOUT : Status.LIST; Issue: If a malicious token contract is used, it could re-enter the spotPurchase function during the external token transfer calls (safeTransferFrom and safeTransfer). This could potentially allow an attacker to manipulate the state variables in unintended ways. Impact: Reentrancy Attack: An attacker might exploit this to execute function...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Lost of `SecondSwap::claim` rewards when Arbitrum L2 sequencer is down

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-67
- **Submitter:** KiteWeb3
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-67
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-67.txt

## Brief Summary

The chains the protocol will be deployed on is defined in the "Question" section in the readme file and are: Ethereum, Base, zkSync, Arbitrum. On the Arbitrum chain when the sequencer is down the state changes can still happen on L2 by passing them from L1 through the Delayed Inbox. During periods when the Arbitrum sequencer is unavailable, transactions are rerouted through the Delayed Inbox, which applies address aliasing to the sender's address. When the sequencer is down on Arbitrum, the msg.sender of a transaction from the Delayed Inbox is aliased in this way: L2_Alias = L1_Contract_Address + 0x1111000000000000000000000000000000001111 If the user that initiates a claim during the sequen...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Missing functionality in `SecondSwap_Marketplace` could harm seller leading in loss of funds and protocol leading in missing profit

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-61
- **Submitter:** mrMorningstar
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-61
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-61.txt

## Brief Summary

If some seller wants to sell their tokens they need first to be listed via listVesting that looks like this: function listVesting( address _vestingPlan, uint256 _amount, uint256 _price, uint256 _discountPct, ListingType _listingType, DiscountType _discountType, uint256 _maxWhitelist, address _currency, uint256 _minPurchaseAmt, bool _isPrivate ) external isFreeze { require( _listingType != ListingType.SINGLE || (_minPurchaseAmt > 0 && _minPurchaseAmt <= _amount), "SS_Marketplace: Minimum Purchase Amount cannot be more than listing amount" ); require(_price > 0, "SS_Marketplace: Price must be greater than 0"); require( (_discountType != DiscountType.NO && _discountPct > 0) || (_discountType =...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Inability to Sell Remaining Tokens Due to Base Amount Check

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-71
- **Submitter:** macart224
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-71
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-71.txt

## Brief Summary

The current implementation of the baseAmount calculation in the listVesting function can lead to a situation where a user is unable to sell their last portion of a listing when the listing type is set to partial. Specifically, if the remaining amount of tokens is too small, it may not satisfy the condition baseAmount > 0, causing the transaction to always revert. This can prevent buyers from purchasing the remaining tokens. The relevant

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# unlistVesting() is susceptible to frontrunning leading to loss of funds to the seller.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-27
- **Submitter:** dhank
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-27
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-27.txt

## Brief Summary

When a user calls unlistVesting( _vestingPlan, _listingId) before a minListingDuration, he should also pay a fixed penaltyFee in USDT. That means user X calls this functions only if he is aware that reclaiming all the vesting tokens they listed for sale is worth more than the penaltyfee. But an attacker can frontrun the above execution by calling the spotPurchasing from the X's _listingId leaving only 1 token remaining in the _listingId. (if attacker takes the entire amount ,listingId is marked as soldOut and the attcker cannot explait since the unlistingexecution reverts) So when the X's unlistVesting() takes place he is actually paying the penaltyFee for 1 vesting token , which puts the X...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Missing CoinAdded event negates transparency and tracking of supported tokens

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-66
- **Submitter:** Mushow
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-66
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-66.txt

## Brief Summary

The addCoin function in the SS_Marketplace contract does not emit the CoinAdded event when a new token is added as a supported payment currency. While the CoinAdded event was previously included in the code as a comment, it was forgotten to be uncommented and properly implemented in the final version. As a result, there is no explicit on-chain record of newly supported tokens. Without the CoinAdded event, users and developers cannot track or verify which tokens have been added as supported currencies. This significantly reduces transparency and usability, forcing users to manually query the isTokenSupport mapping or analyze all past transactions to identify supported tokens. Impact Negates...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Lack of proper support for tokens with decimals that are less than 6

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-109
- **Submitter:** 056Security
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-109
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-109.txt

## Brief Summary

From the contest README, the protocol has stated that they will handle tokens with decimals that are less than 6. This is not properly handled, as such tokens could lead to the vesting plan's releaseRate to be 0 due to how Solidity rounds down.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Incorrect Error Message in `setMaxWhitelist` Function

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-88
- **Submitter:** Brene
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-88
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-88.txt

## Brief Summary

The error message in the setMaxWhitelist function is misleading. It currently states, "SS_Whitelist: amount cannot be lesser that the current whitelist amount," which inaccurately describes the condition being checked. The condition actually compares _maxWhitelist to maxWhitelist, not totalWhitelist. Impact Incorrect error messages can lead to confusion for developers and users, making it difficult to understand the nature of the error and how to resolve it.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Unbounded `maxSellPercent` Value May Cause Incorrect Sell Limit Calculations

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-112
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-112
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-112.txt

## Brief Summary

The vestingSettings[plan].maxSellPercent variable lacks validation to ensure it falls within a reasonable range, such as 0-100%. An unbounded or incorrectly set maxSellPercent could result in erroneous sell limit calculations, allowing users to bypass expected restrictions or unintentionally restrict valid transactions. Impact: If maxSellPercent exceeds the intended range, users may sell more than allowed, violating the vesting plan’s constraints. If maxSellPercent is negative or zero due to a misconfiguration, valid sales could be blocked. Steps to Reproduce: Deploy the contract and configure a maxSellPercent outside the expected range (e.g., 200%). Attempt to sell tokens exceeding userAll...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Sellers can not sell all his vesting tokens which are bought from marketplace

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-34
- **Submitter:** 0xc0ffEE
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-34
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-34.txt

## Brief Summary

Users who are allocated will have the sell limit according to the allocated amount. Besides, users who are not allocated, can be able to sell all the tokens bought from marketplace. However, in case an user's allocation is decreased, the users will be unable to sell the bought vesting tokens. Here, the root cause is the discrepancy between total allocation and sold/bought amount tracked by vesting manager, such that when reallocation happens, only the user's total allocation changed but the sold/bought amount stays unchanged. This can cause user's next sells apply the wrong sell limit. function listVesting(address seller, address plan, uint256 amount) external onlyMarketplace { require(vest...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Giving the `manager` access to `createVesting()` and `createVestings()` can lead to unintended consequences if the `VestingManger` contract is updated

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-142
- **Submitter:** BenRai
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-142
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-142.txt

## Brief Summary

The vestingManager currently has access to the functions createVesting() and createVestings() even though in the current implementation the vestingManger never calls those functions. But since the vestingManager contract is upgradable, there might be an upgrade which allows normal users to make arbitrary calls from the vestingManager. If this is the case, a normal user would be able to create his own vestings since the tokens for vestings are always taken from the tokenIssuer.

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Transfer vestings without fee payment through listing and purchasing with high discount and low price

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-4
- **Submitter:** fyamf
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-4
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-4.txt

## Brief Summary

Users are able to bypass fee payment when transferring vestings to another address by listing a vesting with maximum discount and minimum price.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Inconsistent Fee Validation Logic

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-68
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-68
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-68.txt

## Brief Summary

The following require statement contains a logical inconsistency between the condition and the accompanying error message: require(_fee > -1 && _fee <= 5000, "SS_VestingManager: Seller fee cannot be less than 0"); The condition _fee > -1 is equivalent to _fee >= 0. However, the error message "Seller fee cannot be less than 0" suggests that _fee must indeed be non-negative (greater than or equal to 0). This creates confusion for both developers and auditors since _fee can technically be any value greater than -1, including negative values like -1 (if _fee is a decimal type). Impact This inconsistency can lead to: Misleading Error Messages: Developers may misinterpret the actual requirements...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Using the `currentAllocation` to calculate the `sellLimit` breaks the functionality of the ´maxSellPercent´ variable

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-147
- **Submitter:** BenRai
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-147
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-147.txt

## Brief Summary

Because the currentAllocation is used to calculate the amount of unvested tokens a user can sell, after a while all unvested tokens will be sellable breaking the purpose of the ´maxSellPercent´ variable. This will increase the potential supply of listed vestings over time which will result in a lower marked price (assuming the same demand but higher supply). This will lead to substantial financial loss for users wanting to sell their listing since they will need to sell for a lower price.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Blockchain re-org may cause users to mistakenly purchase tokens at a higher price.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-17
- **Submitter:** shaflow2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-17
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-17.txt

## Brief Summary

The spotPurchase function uses the plan address and ID to distinguish different orders, but it lacks user protection against spending with the highest slippage control. If a blockchain reorganization occurs, it may swap the order IDs, causing users to mistakenly purchase tokens at a higher price.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Lack of deadline checks in spotPurchase allows miners to manipulate execution timing for profit

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-129
- **Submitter:** bumbleb33
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-129
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-129.txt

## Brief Summary

Description: The spotPurchase function in SecondSwap_Marketplace.sol lacks a deadline parameter and slippage protection checks. This allows miners to intentionally delay transaction execution until market conditions are unfavorable for the buyer. When a user submits a purchase transaction, miners can: See the transaction in the mempool Hold the transaction without including it in a block Wait until the price moves unfavorably for the user Include the transaction when it will result in the worst possible execution price This is particularly dangerous because: The marketplace deals with vested tokens which may have volatile prices There's no way for users to specify maximum acceptable slippage

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Bypassing penalty fee by purchasing own listed vesting

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-3
- **Submitter:** fyamf
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-3
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-3.txt

## Brief Summary

When a user unlists a vesting early, they are required to pay a penalty fee. However, the user can bypass this penalty fee by purchasing their own vesting. In this scenario, instead of paying the penalty fee (which defaults to 10 ether in USDT), the user only needs to pay the buyer and seller fees (both set at a default rate of 2.5%).

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Excessive Constraints in `setMaxWhitelist` Function Result in Potential Loss of Functionality

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-89
- **Submitter:** y51r
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-89
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-89.txt

## Brief Summary

The setMaxWhitelist function in the contract is overly restrictive due to redundant require statements. Specifically, the condition _maxWhitelist > maxWhitelist imposes unnecessary constraints on reducing the maxWhitelist value. The issue arises because: The _maxWhitelist > totalWhitelist check is already sufficient to ensure the whitelist capacity cannot be set lower than the number of currently whitelisted users, preserving functionality. The additional _maxWhitelist > maxWhitelist constraint prevents the lotOwner from reducing the maxWhitelist value after it has been increased. If the lotOwner sets an excessively high value (e.g., uint256.max), they lose the ability to adjust the value d...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `VestingTransferred` event can be griefed with duplicate / forged transaction Ids leading off-chain data corruption

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-133
- **Submitter:** YouCrossTheLineAlfie
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-133
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-133.txt

## Brief Summary

The SecondSwap_VestingDeployer::transferVesting function can be used by the token issuer for transferring vesting. This function uses a transactionId as an unique identifier, which is passed down in the emitted event as well function transferVesting( address _grantor, address _beneficiary, uint256 _amount, address _stepVesting, string memory _transactionId <@ - // Used by off-chain mechanism for reconciliation ) external { The issue lies with the way this function is being permissioned require( _tokenOwner[msg.sender] == address(SecondSwap_StepVesting(_stepVesting).token()), <@ - // This allows a malicious _stepVesting contract to be passed "SS_VestingDeployer: caller is not the token owner"

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Admin Can Prevent Users from Unlisting Their Vestings by Arbitrarily Increasing `penaltyFee`

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-52
- **Submitter:** y51r
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-52
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-52.txt

## Brief Summary

The setPenaltyFee function in the contract allows the admin to update the penaltyFee value. This fee is charged to users when they attempt to unlist their vestings before the minimum listing duration has elapsed. However, the function does not enforce an upper bound on the penaltyFee, allowing the admin to set it to an excessively high value. This issue creates a Denial of Service (DoS) scenario for users attempting to unlist their vestings before the minimum listing duration. Specifically: High Penalty Fee Enforcement: If the admin sets an unreasonably high penaltyFee, users will be unable to meet the requirement to pay the fee, preventing them from unlisting their vestings. Exploitation o...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Receiver can be same as sender when spot purchasing a Listing

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-111
- **Submitter:** AshishLach
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-111
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-111.txt

## Brief Summary

When placing listings using spotPurchase() function, protocol doesn't check whether receiver is not the same as the sender Need to keep this require statement in spotPurchase() function require(msg.sender != listing.seller, " seller cant be same as receiver") Write a detailed description of the root cause and impact(s) of this finding. While no major attack path was identified, this could potentially enable attacks based on learnings from previous protocols. Therefore, it is advisable to keep this check in place

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# deployVesting Allows Creation of Vesting Schedules with Past Start Times Enabling Immediate Token Claims

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-77
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-77
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-77.txt

## Brief Summary

A vulnerability has been identified in the vesting schedule deployment mechanism of SecondSwap. The deployVesting function in SecondSwap_VestingDeployer lacks proper temporal validation, allowing the creation of vesting schedules that begin in the past. While the function validates that the start time precedes the end time, it fails to validate against the current blockchain timestamp. The vulnerability stems from the interaction between SecondSwap_VestingDeployer and SecondSwap_StepVesting. When a vesting schedule is deployed with a past start time, the claimable() function in SecondSwap_StepVesting calculates token availability based on elapsed time since the start. This calculation doesn...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Admin of the marketplace can prevent users from claiming their allocations.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-18
- **Submitter:** sl1
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-18
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-18.txt

## Brief Summary

As stated by the contest page, admin of the protocol is expected to only be able to pause and unpause the marketplace, but shouldn't be able to prevent users from claiming their vesting allocations. However, currently if user a has vesting that is listed on the marketplace and decides to unlist and claim it, they won't be able to do so while the market is paused due to unlistVesting() function having isFreeze modifier SecondSwap_Marketplace.sol#L339 function unlistVesting( address _vestingPlan, uint256 _listingId ) external isFreeze { Impact Admin of the marketplace can prevent users from claiming their vesting allocations.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Wrongly Set Penalty Fee

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-14
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-14
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-14.txt

## Brief Summary

In SecondSwap_MarketplaceSetting the penaltyFee is set to 10 ether. This is 10e18. However, the penalty fee is paid with the USDT token. USDT token on all supported chains is a token with 6 decimals. Setting the penaltyFee to 10 ether will set it to 10e18 which is 10000000000000 USDT ~ 10000000000000 $. The penaltyFee can be changed, however, this mistake renders a big part of the protocol useless until that is done by the owner. What is more, if a user theoretically has this amount of USDT and is using the given contract with an approval set to an equal or bigger number, they will lose funds thinking that the fee should be much less.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Incomplete Function Existence Check

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-82
- **Submitter:** TheFabled
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-82
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-82.txt

## Brief Summary

The doesFunctionExist function aims to check if a target contract has a specific function. However, the implementation may not reliably determine the existence of the function. function doesFunctionExist(address target, string memory functionSignature) public view returns (bool) { bytes4 selector = bytes4(keccak256(bytes(functionSignature))); (bool success, ) = target.staticcall(abi.encodeWithSelector(selector)); return success; } Issue: This method may return true even if the function does not exist but a fallback function is present. Additionally, it does not account for function input parameters. Impact: False Positives: The contract might assume a function exists when it does not, leadi...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Linear discount is not calculated correctly

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-21
- **Submitter:** 0xrex
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-21
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-21.txt

## Brief Summary

Token sellers can set up discounts for tokens being sold such as 10%. This would mean that the users would pay 10% less overall for the token sale. For the case of the linear discount, using the current calculation, the actual discount linearly will be 5% max not 10.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Creating a vesting for a user that is past the last claim step will lock the tokens

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-146
- **Submitter:** gesha17
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-146
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-146.txt

## Brief Summary

When creating a vesting, the amount that is being assigned to the benificiary is calculated based on how many steps are left. If the beneficiary has claimed past the last step, then the releaseRate is set to 0. This means that any new tokens that are assigned to this beneficiary will essentially become locked. This can happen by chance or by an honest mistake E.g. a token owner decides to grant some extra tokens to a benificiary at the last step, seeing the beneficiary has not yet claimed his tokens, but the beneficiary decides to claim his tokens just before the owners transaction executes, so the tokens become locked. This can be somewhat mitigated by the beneficiary - he can make a listi...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Token issuer referral reward manipulation through combined direct and market Transfers

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-143
- **Submitter:** Sabit
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-143
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-143.txt

## Brief Summary

The tokenIssuer can be used as a referrer in marketplace purchases, even after directly transferring tokens to the same buyer. This creates an unintended profit opportunity through referral rewards that undermines the marketplace's referral incentive system. The vulnerability stems from insufficient referral validation in the spotPurchase function: function spotPurchase(address _vestingPlan, uint256 _listingId, uint256 _amount, address _referral) external isFreeze { // Current validation only checks if buyer is not referrer _validatePurchase(listing, _amount, _referral); } function _validatePurchase(Listing storage listing, uint256 _amount, address _referral) private view { require(msg.send...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Rounding Issues Leading to Potential Exploits

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-101
- **Submitter:** TheFabled
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-101
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-101.txt

## Brief Summary

There are calculations involving division that could result in integer rounding errors, potentially leading to zero values or inaccurate amounts. Example in _handleTransfers: uint256 baseAmount = (_amount * discountedPrice) / uint256(10 ** (IERC20Extended(...).decimals())); require(baseAmount > 0, "SS_Marketplace: Amount too little"); Issue: If discountedPrice is low or the token decimals are large, baseAmount could be calculated as zero due to integer division truncation. This could allow purchasers to buy tokens for zero payment or cause a denial of service. Impact: Economic Loss: Sellers might lose funds if tokens are sold for less than intended. Denial of Service: Transactions might fai...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# token like cUSDCv3 can be stolen from the SecondSwap_StepVesting contract

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-118
- **Submitter:** Fon
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-118
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-118.txt

## Brief Summary

when creating vesting, the _totalAmount is transferred with token.safeTransferFrom(tokenIssuer, address(this), _totalAmount); however, no check confirms that the contract balance increases by that amount. for tokens like cUSDCv3 where transferring type(uint256).max just transfers the total balance of the account this can lead to accounting errors and stolen funds

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient
