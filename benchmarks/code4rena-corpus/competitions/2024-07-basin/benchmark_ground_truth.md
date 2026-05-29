# Benchmark Ground Truth: Basin

## Accepted H/M Findings

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

## Rejected Primary Findings

# Rejected Primary Findings: Basin

# The `__ReentrancyGuard_init()` function should be called before any other initializations in the `init` function to ensure that reentrancy protection is established as early as possible.

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-115
- **Submitter:** 0xvd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/115
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-115.md

## Brief Summary

The current implementation of the `init` function initializes the reentrancy guard after other initializations, which leaves a window where reentrancy attacks could occur. By prioritizing the initialization of the reentrancy guard, you can ensure that the contract is protected from reentrancy attacks from the start.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in Stable2.sol

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-79
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/79
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-79.md

## Brief Summary

The contract Stable2.sol does not validate the length of input arrays in several functions which take in array parameters. This exposes the contract functions to potential out-of-bounds issues. There is no check to verify if the `reserves` array is of the expected length, which could cause unexpected errors or misbehaviors in the contract. Moreover, the smart contract does not perform any validation on address inputs before proceeding with execution. This might expose the contract to potential invocation of the zero address, which may result in unexpected contract behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Lack of Input Sanitization in Stable2LUT1.sol

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-82
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/82
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-82.md

## Brief Summary

The `getRatiosFromPriceLiquidity` and `getRatiosFromPriceSwap` functions do not have any checks to ensure that the input provided is valid. This could lead to unexpected behaviour if an invalid value is passed as an argument to the function. Moreover, the use of nested if-else conditions for price ranges is not the most efficient way for price level determination and makes the code harder to read and maintain.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Unrestricted Function Access Leading to Potential Manipulation and DoS Vulnerabilities

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-16
- **Submitter:** JuggerNaut63
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/16
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-16.md

## Brief Summary

Unrestricted Function Access Leading to Potential Manipulation and DoS Vulnerabilities

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# High Complexity in Price Lookup Functions Due to Deep Nesting

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-17
- **Submitter:** JuggerNaut63
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/17
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-17.md

## Brief Summary

- The deeply nested if-else structure makes the code hard to maintain and update, increasing the risk of introducing errors. - The complexity reduces code readability, making it difficult for developers to understand and debug. - Deep nesting can lead to high gas costs, potentially making transactions more expensive.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Arbitrary from passed to transferFrom (lor safeTransferFrom)

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-37
- **Submitter:** MFaizal14
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/37
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-37.md

## Brief Summary

Passing an arbitrary from address to transferFrom (or safeTransferFrom) can lead to loss of funds, because anyone can transfer tokens from the from address if an approval is made.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Uninitialized local

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-9
- **Submitter:** MFaizal14
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/9
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-9.md

## Brief Summary

is a local variable never initialized

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No Storage Gap For Upgradeable Contracts (child as well as parent contracts)

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-94
- **Submitter:** Shubham
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/94
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-94.md

## Brief Summary

In the Automated Finings, the issue is mentioned ([Link](https://github.com/code-423n4/2024-07-basin/blob/main/4naly3er-report.md#l-10-upgradeable-contract-is-missing-a-__gap50-storage-variable-to-allow-for-new-storage-variables-in-later-versions)) but it only states a part of the bug which would still lead to problems. Upgradeability involves **inheritance** but the inherited contract do not have a storage gap, not even the contract that is inheriting has.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# precision loss due to division before multiplicaton

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-122
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/122
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-122.md

## Brief Summary

Detailed description of the impact of this finding. precision loss due to division before multiplication in getBandC.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# High Gas Consumption and Potential Out-of-Gas in calcLpTokenSupply and calcReserve Functions

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-31
- **Submitter:** black-wolf
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/31
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-31.md

## Brief Summary

1-`High Gas Costs`: Long execution times of these functions can lead to very high gas costs, which is undesirable for users. 2-`Out-of-Gas Risk`: If the computations are too lengthy, the contract might run out of gas, causing the transaction to fail and disrupting contract operations. 3-`Unstable Performance‍‍‍‍‍`: Complex and lengthy computations can lead to unstable contract performance and reduce user trust in the system.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# In the ``Stable2`` contract ``calcLpTokenSupply‍‍`` function, there is a potential integer overflow/underflow vulnerability

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-32
- **Submitter:** black-wolf
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/32
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-32.md

## Brief Summary

In the `calcLpTokenSupply` function, there is a potential integer overflow/underflow vulnerability due to arithmetic operations involving large numbers. Integer overflow/underflow can lead to unexpected results, such as incorrect calculations or unintended contract behavior. This issue occurs in the loop where arithmetic operations are performed on `lpTokenSupply`, which can lead to values exceeding the maximum or minimum limits of the `uint256` type. Impact If an integer overflow or underflow occurs, the calculations within the `calcLpTokenSupply` function may produce incorrect results. This can lead to incorrect `lpTokenSupply` values, which in turn may affect the overall functionality of...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# Unchecked External Call on `calcRate` function on `Stable2` contract

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-33
- **Submitter:** black-wolf
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/33
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-33.md

## Brief Summary

Issue Type: Unchecked External Call Affected Contract: Stable2 Description The `calcRate` function calls an external contract to fetch data. If this external contract is compromised or behaves unexpectedly, it could return incorrect data or fail, potentially causing unintended behavior in the `calcRate` function. This is due to the unchecked nature of the external call. Impact An unchecked external call can lead to the contract relying on potentially faulty or malicious data from the external contract. This could compromise the integrity of the contract's operations, leading to incorrect results or vulnerabilities. Recommendations To mitigate this issue: 1-`Check Results from External Calls...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Division by Zero Error in updateReserve Function

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-123
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/123
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-123.md

## Brief Summary

The `updateReserve` function is responsible for calculating the step size to update the reserve value based on the target price and the current price. It uses the difference between the `pd.lutData.highPrice` and `pd.lutData.lowPrice` to determine the step size. If the `pd.lutData.highPrice` and `pd.lutData.lowPrice` are equal, this means the target price is exactly between the high and low prices in the lookup table. In this case, the expression `(pd.lutData.highPrice - pd.lutData.lowPrice)` will be zero, which will lead to a division by zero error when calculating the step size. Code Impact If the `updateReserve` function encounters a division by zero error, it will revert the entire tran...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Unprotected initializer in 'initNoWellToken' function allowing unauthorized reinitialization of the contract

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-92
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/92
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-92.md

## Brief Summary

The 'initNoWellToken' function is unprotected, posing a high-severity risk by allowing unauthorized reinitialization of the contract, which could result in significant adverse consequences.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Contract contains payable functions but no withdraw/sweep function

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-75
- **Submitter:** jauvany
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/75
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-75.md

## Brief Summary

In smart contract development, particularly for Ethereum, having payable functions without a corresponding withdraw or sweep function can lead to potential issues. Payable functions allow the contract to receive Ether, but without a mechanism to withdraw these funds, the Ether can become locked within the contract indefinitely. This situation might be intentional in some cases (like a burn function), but generally, it’s a design oversight. A withdraw or sweep function is necessary to transfer Ether out of the contract to a specific address, typically the owner's or a designated recipient. Without this, the contract lacks flexibility in managing its funds, potentially leading to lost or inac...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reserve address validation is incorrect; can lead to Division by Zero Error

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-110
- **Submitter:** lonelyprince
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/110
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-110.md

## Brief Summary

A non-zero reserve value when used inside calcLpTokenSupply function can lead to division by Zero Error.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# Gas Griefing through Unbounded Loops in WellUpgradeable Contract

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-15
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/15
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-15.md

## Brief Summary

The vulnerability is identified in the init function, which performs a nested loop to check for duplicate tokens. The WellUpgradeable contract is vulnerable to gas griefing due to an inefficient duplicate token check in its init function. This function uses a nested loop with a time complexity of O(n)^2 to identify duplicate tokens, causing the gas consumption to increase quadratically with the number of tokens. This can lead to excessive gas usage and potential denial of service when the number of tokens is large, as transactions may exceed the block gas limit and fail. Impact: Denial of service due to gas limit exhaustion, making the contract unusable when initializing with a large number...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary
