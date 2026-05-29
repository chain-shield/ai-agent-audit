# Accepted H/M Findings: Basin

# [H-01] WellUpgradeable can be upgraded by anyone

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-basin
- **Source snapshot:** competitions/2024-07-basin/final_report.html

WellUpgradeable can be upgraded by anyone Submitted by zanderbyte, also found by Egis_Security ( 1, 2 ), 0xSecuri ( 1, 2 ), debo, Walter ( 1, 2 ), unnamed, mgf15, Honour, 0x11singh99, Flare, ZanyBonzy, shaflow2, Mrxstrange, NoOne, John_Femi, and 0xvd WellUpgradeable is an upgradeable version of the Well contract, inheriting from OpenZeppelin’s UUPSUpgradeable and OwnableUpgradeable contracts. According to OpenZeppelin’s documentation for UUPSUpgradeable.sol ( here and here ), the internal _authorizeUpgrade function must be overridden to include access restriction, typically using the onlyOwner modifier. This must be done to prevent unauthorized users from upgrading the contract to a potentially malicious implementation.

However, in the current implementation the _authorizeUpgrade function is overridden with custom logic but lacks the onlyOwner modifier. As a result, the upgradeTo and upgradeToAndCall methods can be invoked by any address, allowing anyone to upgrade the contract, leading to the deployment of malicious code and compromise the integrity of the contract.

## Recommended mitigation steps

Add the onlyOwner modifier to the _authorizeUpgrade function in WellUpgradeable.sol to restrict upgrade permissions:

+ function _authorizeUpgrade(address newImplmentation) internal view override onlyOwner { // verify the function is called through a delegatecall.

require(address(this) != ___self, "Function must be called through delegatecall"); // verify the function is called through an active proxy bored by an aquifer.

address aquifer = aquifer(); address activeProxy = IAquifer(aquifer).wellImplementation(_getImplementation()); require(activeProxy == ___self, "Function must be called through active proxy bored by an aquifer"); // verify the new implementation is a well bored by an aquifier.

require( IAquifer(aquifer).wellImplementation(newImplmentation) != address(0), "New implementation must be a well implementation" ); // verify the new impelmentation is a valid ERC-1967 impelmentation.

require( UUPSUpgradeable(newImplmentation).proxiableUUID() == _IMPLEMENTATION_SLOT, "New implementation must be a valid ERC-1967 implementation" ); }

## Assessed type

Access Control nickkatsios (Basin) confirmed and commented via duplicate Issue #18:

The modifier was mistakenly omitted here and should definitely be added.

0xsomeone (judge) commented:

The Warden and its duplicates have properly identified that the upgrade methodology of a WellUpgradeable is insecure, permitting any contract to be upgraded to an arbitrary implementation. Specifically, the upgrade authorization mechanism will ensure that:

The call was routed through the proxy.

The new implementation complies with the EIP-1967 standard.

The new implementation has been registered in the Aquifier.

Given that well registration on the Aquifier is unrestricted as seen here, it is possible to practically upgrade any well to any implementation.

I believe a high-risk assessment is valid as this represents a critical security issue that can directly lead to total fund loss for all wells deployed in the system.

# [H-02] Incorrectly assigned decimal1 parameter upon decoding

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-basin
- **Source snapshot:** competitions/2024-07-basin/final_report.html

decimal1 parameter upon decoding Submitted by ZanyBonzy, also found by SAQ, trachev, Agontuk, rare_one, Rhaydden, TheFabled, Akay, FastChecker, d4r3d3v1l, Mrxstrange, zanderbyte, macart224, Honour, Bauchibred, stuart_the_minion, johnthebaptist, willycode20, Nikki, 0xAadi, psb01, Flare, 0xRiO, 0xvd, alexzoid, KupiaSec, ArcaneAgent, and dreamcoder Impact is high as lots of functions depend on the decodeWellData, and as such will be fed incorrect data about token1 ’s decimals. This can lead to potential overvaluation or undervaluation of the token’s amount and prices, depending on the context with which its used. It also ignores the fact that decimals1 can return a 0 value and as such will work with that for scaling, rather than scaling it to 18. This also depends on the

decimal0 being 0;

## Recommended Mitigation Steps

Change the check:

//...

- if (decimal0 == 0) { + if (decimal1 == 0) { decimal1 = 18; //...

}

## Assessed type

en/de-code nickkatsios (Basin) confirmed and commented:

This is valid and needs to be corrected. Good catch!

0xsomeone (judge) increased severity to High and commented:

The Warden has outlined an issue in the way decimals are handled when decoding immutable well data and specifically a problem that will arise when the decimal0 configuration is 0 whilst the decimal1 configuration is a value other than 18 and 0.

I believe that a high-risk rating is better suited for this submission as the decimals utilized throughout the swapping calculations are imperative to the AMM’s safe operation.

Medium Risk Findings (2)

# [M-01] For extreme ratios, getRatiosFromPriceSwap will return data for which is impossible to converge into a reserve

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-basin
- **Source snapshot:** competitions/2024-07-basin/final_report.html

getRatiosFromPriceSwap will return data for which is impossible to converge into a reserve Submitted by Egis_Security The Basin team has implemented look up table to fetch pre-calculated values, which are close to the targetPrice to decrease the complexity of calcReserveAtRatioSwap and calcReserveAtRatioLiquidity functions.

Stable2LUT1 has a function getRatiosFromPriceSwap, which returns PriceData struct based on provided price.

We can see that in case of super extreme price, the function will revert with LUT: Invalid price, but there is also support for extreme prices, which we assume should work correctly, when such situation occurs.

We will investigate an edge case, which is present when we enter if (price < 0.213318e6) and price is above 0.001083e6. In such situations, the function will return the following struct:

PriceData( 0.213318e6, // highPrice 0.188693329162796575e18, 2.410556040105746423e18, 0.001083e6, // lowPrice 0.005263157894736842e18, 10.522774272309483479e18, 1e18 ); If you notice, we have a large gap between highPrice and lowPrice and more precisely. The following results in large jump in reserves calculation when updateReserve which leads to skipping the target price sequentially, which moves away pd.currentPrice until we exit the for loop, which returns 0.

Here is an estimation of the impact based on the PoC which is below:

For a ratio ~ 1:4.6 of the price of the tokens:

We have a targetPrice of 212104 (0.21).

We end up with reserves for a price 44957 (0.04), which is ~ X5 less than the requested amount.

As result we receive 3623929482258792273, instead of 2421185918213441156, which is a big difference.

Note that we don’t return the compromised data, but this is the calculated value on the last iteration.

## Recommended Mitigation Steps

Recalculate the PriceData for the given case and consider narrowing down the price diff.

## Assessed type

Invalid Validation Brean0 (Basin) confirmed 0xsomeone (judge) decreased severity to Low and commented:

The Warden has identified a potential edge case in the pre-computed values yielded by the Stable2LUT1::getRatiosFromPriceSwap function where the discrepancy between the upper and lower price bounds is significant and adversely affects reserve calculations.

I believe a medium-risk assessment is better suited for this particular vulnerability due to its low likelihood of manifesting in a production environment. Specifically, the PoC will directly interact with the Stable2 contract even though the ratios passed into it stem from different calculations in the MultiFlowPump contract.

In order to properly accept the medium-risk assessment, I invite the wardens of the team to supplement a PoC that demonstrates the vulnerability manifesting in a production environment through a higher-level call to the Basin system. As the issue stands, it lacks sufficient impact although the actual flaw is adequately described.

For the above reason, I will consider this submission as QA (L) pending substantiation by the Warden team.

nmirchev8 (warden) commented:

@0xsomeone - Here are my arguments regarding why the issue should be a valid Medium:

We can both agree that the scope of this competition is very abstract and 90% of the functions are “view”. As auditors, we should ensure that the code in scope behaves as expected. The following issue is bound in the limited scope of this competition and can be summarized as “break core functionality”. When the scope is abstract, we should assume that every case in the provided code may be executed.

The only precondition for the vulnerability to occur is to have a ratio of ~ 1:4.6, which is realistic in de-peg situations for stablecoins.

Here is a proof from sponsor that currently there is no production environment for the corresponding contracts:

Note: to view the provided image, please see the original submission here.

nickkatsios (Basin) commented:

We would tend to agree with classifying this issue as Medium severity. While the likelihood of it occurring in a real production environment is low, it effectively renders the well function unusable. According to the severity documentation, which states that “assets are not at direct risk, but the protocol’s function or availability could be impacted,” this issue clearly falls within that category.

Instead of recalculating the data points for the lookup table, the issue was resolved by improving the step size, ensuring that the maxStep size can never exceed the j reserve. You can see the complete fix in this PR.

0xsomeone (judge) increased severity to Medium and commented:

Hey @nmirchev8 and @nickkatsios - After evaluating all relevant information as well as discussing it with the Sponsor, I believe that the submission should be accepted as a medium-risk vulnerability due to relying on hypotheticals yet demonstrating the flaw clearly.

Submission #22 was resolved using the same approach as this one and thus the root cause is the large gap that exists in the step size. As such, the issues will remain group rendering this submission to be treated as a “single” one.

# [M-02] In Stable2LUT1::getRatiosFromPriceLiquidity , in extreme cases, updateReserve will start breaking

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-basin
- **Source snapshot:** competitions/2024-07-basin/final_report.html

Stable2LUT1::getRatiosFromPriceLiquidity, in extreme cases, updateReserve will start breaking Submitted by Egis_Security This is one of the edge cases in getRatiosFromPriceLiquidity:

if ( price < 0.001083e6 ) { revert ( "LUT: Invalid price" ); } else { return PriceData ( 0.27702e6, 0, 9.646293093274934449e18, 0.001083e6, 0, 2000e18, 1e18 ); } The range where the price can be is very large, between 0.27702e6 ( highPrice ) and 0.001083e6 ( lowPrice ). If we are closer to the lowPrice, then pd.currentPrice is set to pd.lutData.lowPrice:

if ( pd.

lutData.

highPrice - pd.

targetPrice > pd.

targetPrice - pd.

lutData.

lowPrice ) { // targetPrice is closer to lowPrice.

scaledReserves [ j ] = scaledReserves [ i ] * pd.

lutData.

lowPriceJ / pd.

lutData.

precision; // set current price to lowPrice.

pd.

currentPrice = pd.

lutData.

lowPrice; } In this case, the current price is much smaller than the target price and because of this, updateReserve will have to do a very large correction of the reserve in order to converge on the target price.

Because of these large corrections, the reserve will become so small, that the next time the reserve has to be updated, the amount that it has to be reduced by will be larger than the reserve itself, resulting in a panic underflow and bricking the function.

This revert happens because of two reasons:

Because of the large range that the price can be in, thus using such a small lowPrice as pd.currentPrice makes the difference between pd.targetPrice so large that the corrections become very large and underflow before convergence can be achieved.

lowPriceJ is extremely large. In this case it’s 2000e18, which makes the step size very large and thus, the code will attempt extreme corrections of the reserve, resulting in a revert.

It’s important to note, that even if lowPriceJ is significantly reduced, the function won’t revert, but it will never converge on a price and this can only be fixed by reducing the gap between highPrice and lowPrice effectively reducing the range for that specific price.

We wanted to showcase both issues and that even if the underflow ( lowPriceJ issue) is resolved that the estimated ranges still are too large, thus making the price impossible to converge.

The below tests showcase both the original code (underflow) and how reducing lowPriceJ doesn’t fix the real problem.

## Recommended Mitigation Steps

To completely fix both issues, lowPriceJ must be lowered and the estimated range must be narrowed down.

## Assessed type

DoS Brean0 (Basin) confirmed 0xsomeone (judge) decreased severity to Low and commented:

The Warden outlines a misbehavior that arises from the edge case price configuration outlined in Issue #25. I consider both issues to stem from the extreme difference between the upper and lower price bounds, as eliminating this gap would resolve both issues and thus be considered the root cause of both.

This particular submission once again relies on direct interaction with the Stable2LUT1 contract and does not adequately demonstrate the issue in a production environment. A demonstration of either this one or exhibit #25 will be considered acceptable to consider them acceptable per my comment on #25.

deth (warden) commented:

@0xsomeone - Here are our arguments why this is a valid Medium and it is not a duplicate of #25:

Why this is valid:

The case may be an edge case, but it is in the Stable2LUT1 and must be considered.

The PoC provided is the only one that can be provided as of now, as currently Beanstalk (the integrator of the function) doesn’t have any production ready code built around the Stable2 contract (the caller of getRatiosFromPriceLiquidity ). Thus, we cannot provide a production ready PoC since there is nothing to base it off of.

The issue should be considered a Medium, as it breaks core functionality completely, making any call to getRatiosFromPriceLiquidity unusable, which can have a large impact on all integrators that use/will use the function. Thus, with a low likelihood and High impact, we believe this a valid Medium.

Why this isn’t a duplicate:

The root between the two issues are different. One occurs in getRatiosFromPriceLiquidity and the other in getRatiosFromPriceSwap. If #25 is fixed this issue will still persist and vice-versa.

nickkatsios (Basin) commented:

We would tend to agree with classifying this issue as Medium severity. While the likelihood of it occurring in a real production environment is low, it effectively renders the well function unusable. According to the severity documentation, which states that “assets are not at direct risk, but the protocol’s function or availability could be impacted,” this issue clearly falls within that category.

Instead of recalculating the data points for the lookup table, both issues were resolved by improving the step size, ensuring that the maxStep size can never exceed the j reserve. You can see the complete fix in this PR.

0xsomeone (judge) increased severity to Medium and commented:

See response in the primary submission Issue #25.

deth (warden) commented:

@0xsomeone - I don’t understand, how is the root cause is the same? If #25 is fixed, #22 persists and vice-versa. I understand that main driving force behind both issues is the same, but fixing one of them doesn’t fix the other, you can see the sponsors applied fixes to both functions independently.

0xsomeone (judge) commented:

@deth - While discussing with the Sponsor, we concluded that both issues stem from the extreme step sizes on the lower bounds of the respective curves. However, you are indeed correct in that the actual ratios are yielded by distinct functions and thus there is no actual correlation between the two submissions (i.e., it is not possible to fix both simultaneously with a single code change).

As such, I have proceeded with splitting this submission as a unique entry given that from a C4 perspective the issues do not strictly share a root cause. I greatly appreciate the due diligence!
