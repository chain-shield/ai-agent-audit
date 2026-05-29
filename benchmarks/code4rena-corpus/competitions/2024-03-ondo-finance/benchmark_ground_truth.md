# Benchmark Ground Truth: Ondo Finance

## Accepted H/M Findings

# Accepted H/M Findings: Ondo Finance

# [H-01] OUSGInstantManager will allow excessive OUSG token minting during USDC depeg event

- **Contest:** Ondo Finance
- **Slug:** 2024-03-ondo-finance
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-ondo-finance
- **Source snapshot:** competitions/2024-03-ondo-finance/final_report.html

OUSGInstantManager will allow excessive OUSG token minting during USDC depeg event Submitted by Breeje, also found by Arz, HChang26, and immeas Any user can use mint function in ousgInstantManager contract to mint OUSG tokens by providing USDC token. It calls internal function _mint where the main logic resides.

function _mint ( uint256 usdcAmountIn, address to ) internal returns ( uint256 ousgAmountOut ) { // SNIP: Validation uint256 usdcfees = _getInstantMintFees ( usdcAmountIn ); uint256 usdcAmountAfterFee = usdcAmountIn - usdcfees; // Calculate the mint amount based on mint fees and usdc quantity uint256 ousgPrice = getOUSGPrice (); ousgAmountOut = _getMintAmount ( usdcAmountAfterFee, ousgPrice ); require ( ousgAmountOut > 0, "OUSGInstantManager::_mint: net mint amount can't be zero" ); // SNIP: Transfering USDC ousg.

mint ( to, ousgAmountOut ); } Two important points to understand OUSG Price Stability:

The contract depends on the OUSG price obtained from an oracle, which is heavily constrained (as per Readme) to ensure stability.

OUSG Price - The OUSG price tracks an off chain portfolio of cash equivalents and treasury bills, price changes are heavily constrained in the OUSG Oracle, which uses the change in the price of SHV to set the allowable OUSG price in between updates. We are aware that the SHV price could differ from the OUSG portfolio, so any findings related to this price discrepancy is out of scope. Also, scenarios where the OUSG price increases by many orders of magnitudes are not realistic and consequently not considered valid.

As per RWAOracleRateCheck Oracle, constraints includes:

OUSG price updates restricted to once every 23 hours.

Price deviations limited to a maximum of 1%.

function setPrice ( int256 newPrice ) external onlyRole ( SETTER_ROLE ) { if ( newPrice <= 0 ) { revert InvalidPrice (); } @-> if ( block.

timestamp - priceTimestamp < MIN_PRICE_UPDATE_WINDOW ) { revert PriceUpdateWindowViolation (); } @-> if ( _getPriceChangeBps ( rwaPrice, newPrice ) > MAX_CHANGE_DIFF_BPS ) { revert DeltaDifferenceConstraintViolation (); } // Set new price int256 oldPrice = rwaPrice; rwaPrice = newPrice; priceTimestamp = block.

timestamp; emit RWAPriceSet ( oldPrice, newPrice, block.

timestamp ); } These constraints ensure relative stability of the OUSG price.

Calculation Assumptions:

The calculation of the amount of OUSG tokens to mint assumes a fixed conversion rate of 1 USDC = 1 USD.

Key point: The _getMintAmount function calculates the OUSG amount based on the provided USDC amount and the OUSG price obtained from the oracle (by just upscaling and dividing).

function _getMintAmount ( uint256 usdcAmountIn, uint256 price ) internal view returns ( uint256 ousgAmountOut ) { uint256 amountE36 = _scaleUp ( usdcAmountIn ) * 1e18; ousgAmountOut = amountE36 / price; } Here, there are no validation checks implemented regarding the current USDC price.

Scenario of the issue Consider Alice’s attempt to mint OUSG tokens by providing 100,000 USDC, assuming no minting fees and OUSG price of 105e18 USD. The calculation yields:

100_000e36 / 105e18 which is approximately 95_000e18 or 95_000 OUSG tokens for the 100_000 USDC provided.

However, in the event of a USDC depeg, where USDC ’s value deviates from 1 USD:

The contract’s calculation logic remains unchanged.

Despite the depeg, the OUSG price remains fairly constant (maximum 1% deviation allowed in 23 hours).

This scenario leads to Alice getting close to 95_000 OUSG tokens again for 100_000 USDC provided. But this time, 100_000 USDC can be worth as low as 87_000 USD if we take recent depeg event in March 2023, where USDC price went as low as 87 cents ( reference ).

This way, contract will allow users to mint excessive OUSG tokens during the depeg event.

## Impact

Minting of excessive token in case of USDC depeg.

Tools Used VS Code

## Recommended Mitigation Steps

Ideally, there needs to be an additional Oracle to check current price of USDC and take its price into the consideration when calculation OUSG tokens to mint.

## Assessed type

Context 3docSec (judge) increased severity to High and commented:

Upgraded as High because there is risk of value extraction from the protocol under conditions that can be monitored by an attacker.

cameronclifton (Ondo) confirmed, but disagreed with severity and commented:

After further review, we will be mitigating this by adding a Chainlink USDC / USD oracle to the OUSGInstantManager contract. If the price is lower than what we are comfortable with, all mints and redemptions will be blocked. While we think it is unlikely that we won’t be able to convert USDC->USD 1:1 in our backend systems, we decided to do this out of extreme caution.

Note: For full discussion, see here.

Medium Risk Findings (4)

# [M-01] Integration issue in ousgInstantManager with BUIDL if minUSTokens is set by blackrock

- **Contest:** Ondo Finance
- **Slug:** 2024-03-ondo-finance
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-ondo-finance
- **Source snapshot:** competitions/2024-03-ondo-finance/final_report.html

ousgInstantManager with BUIDL if minUSTokens is set by blackrock Submitted by asui Integration issues with BUIDL, in the case blackrock decides to set a minimum amount of BUIDL tokens that should be held by its holders.

## Recommended Mitigation Steps

Import IDSComplianceConfigurationService or create an interface just for the getMinUSTokens() function and consider replacing the require statement in the _redeemBUIDL function with:

function _redeemBUIDL ( uint256 buidlAmountToRedeem ) internal { require ( buidl.

balanceOf ( address ( this )) - IDSComplianceConfigurationService ( 0x1dc378568cefD4596C5F9f9A14256D8250b56369 ).

getMinUSTokens >= minBUIDLRedeemAmount, "OUSGInstantManager::_redeemBUIDL: Insufficient BUIDL balance" ); The contract will never try to redeem more than its minimum allowed to hold and appropriately reverts with our error message:

” OUSGInstantManager::_redeemBUIDL: Insufficient BUIDL balance ” We get the address 0x1dc378568cefD4596C5F9f9A14256D8250b56369 of the complianceConfigurationService proxy by querying the BUIDL contract in etherscan using the function no.27 getDSService with 256 as the argument.

This minimum amount required may not be set currently but could be set by the admin in the future. So, implementing it now should be more compatible with BUIDL, even if in the future blackrock decides to set it.

## Assessed type

Invalid Validation cameronclifton (Ondo) acknowledged, but disagreed with severity and commented:

We will not mitigate this in the smart contract code. We plan to work with the BUIDL team to better understand the conditions in which minUSTokens will be set.

Note: For full discussion, see here.

# [M-02] Inadequate handling of BUIDL redemption limit in OUSG instant manager

- **Contest:** Ondo Finance
- **Slug:** 2024-03-ondo-finance
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-ondo-finance
- **Source snapshot:** competitions/2024-03-ondo-finance/final_report.html

BUIDL redemption limit in OUSG instant manager Submitted by Limbooo, also found by Bigsam

- https://github.com/code-423n4/2024-03-ondo-finance/blob/78779c30bebfd46e6f416b03066c55d587e8b30b/contracts/ousg/ousgInstantManager.sol#L426-L429
- https://github.com/code-423n4/2024-03-ondo-finance/blob/78779c30bebfd46e6f416b03066c55d587e8b30b/contracts/ousg/ousgInstantManager.sol#L460

## Impact

The OUSG Instant Redemption Manager contract contains an oversight in its redeem function, specifically in the handling of BUIDL redemption limits. This oversight can potentially lead to failed redemption attempts when the redemption balance exceeds the BUIDL balance held by the manager contract while it has a right amount if its concatenated with USDC amount left by another redemption process. The impact of this issue is significant as it affects the usability of the redemption feature and can result in user frustration and loss of trust in the system.

## Recommended Mitigation Steps

To address this issue, the OUSG Instant Redemption Manager contract should implement a mechanism to ensure that redemption requests do not exceed the available BUIDL balance held by the manager contract. This can be achieved by incorporating proper checks and balances in the redemption process, such as verifying the BUIDL balance before processing redemption requests and adjusting the redemption amount accordingly. Additionally, consider an redeem implementation that concatenate the balance of remaining USDC amount with the BUIDL redeemed balance if the corresponding USDC amount or redeem amount of OUSG is more than minBUIDLRedeemAmount.

## Assessed type

Error cameronclifton (Ondo) confirmed and commented:

Due to changing requirements, the contract will now concatenate the USDC amount with BUIDL when performing redemptions. (This should mitigate this already known issue).

Note: For full discussion, see here.

# [M-03] Users can lose access to funds due to minimum withdrawal limits

- **Contest:** Ondo Finance
- **Slug:** 2024-03-ondo-finance
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-ondo-finance
- **Source snapshot:** competitions/2024-03-ondo-finance/final_report.html

Submitted by carrotsmuggler, also found by Breeje, 0xmystery, radev_sw, dvrkzy, and 0xCiphky The InstantManager contract restricts deposits and withdrawals to certain minimum amounts. Users can deposit a minimum of 100k USDC tokens, and withdraw a minimum of 50k USDC tokens.

The issue is that the minimum withdrawal limit can lead to users losing access to part of their funds. Say a user deposits 100k USDC tokens and then later withdraws 60k USDC tokens. Now, the user only has 40k USDC worth holdings in their account, and cannot withdraw the full amount. This is because it falls below the minimum withdrawal limit of 50k USDC tokens. The user is now stuck with 40k USDC tokens in their account, and cannot withdraw them.

The only option the user has is to deposit 100k USDC more, and then withdraw the whole 140k USDC amount. This will incur fees on the extra 100k USDC the user brings as well. Thus this is a Medium severity issue.

## Recommended Mitigation Steps

Allow users to remove all their funds from the contract even if it is below the minimum limit. Since the protocol now uses a more liquid system such as the BUIDL token, this should be possible and should not affect the protocol’s functioning.

3docSec (judge) commented:

I acknowledge this behavior is a design decision. However, I would keep this as a valid Medium for an audit report:

There is an availability impact for users, in a condition that they did not necessarily have to purposely create for themselves.

Users can decide to still withdraw for a loss in fees “for minting more to redeem all”.

The report highlights what I find to be a very reasonable mitigation - which could be the behavior users reasonably expect:

Allow users to remove all their funds from the contract even if it is below the minimum limit.

This mitigation seems feasible and difficult to exploit for systematic, abusive bypasses of minimumRedemptionAmount, because both OUSG and rOUSG have a KYC requirement on token holders.

cameronclifton (Ondo) disputed and commented:

We will not be removing minimum redemption requirement from the smart contract as there are other means in which users can redeem OUSG or rOUSG tokens from Ondo Finance.

Note: For full discussion, see here.

# [M-04] The BURNER cannot burn tokens from accounts not KYC verified due to the check in _beforeTokenTransfer .

- **Contest:** Ondo Finance
- **Slug:** 2024-03-ondo-finance
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-ondo-finance
- **Source snapshot:** competitions/2024-03-ondo-finance/final_report.html

BURNER cannot burn tokens from accounts not KYC verified due to the check in _beforeTokenTransfer.

Submitted by Krace, also found by ni8mare, Limbooo, leegh, ast3ros, radev_sw, Shubham, kartik_giri_47538, Arz, kaden, Tychai0s, yotov721, SpicyMeatball, ZanyBonzy, dvrkzy, and 0xDemon

- https://github.com/code-423n4/2024-03-ondo-finance/blob/be2e9ebca6fca460c5b0253970ab280701a15ca1/contracts/ousg/rOUSG.sol#L586-L606
- https://github.com/code-423n4/2024-03-ondo-finance/blob/be2e9ebca6fca460c5b0253970ab280701a15ca1/contracts/ousg/rOUSG.sol#L624-L640

## Impact

The BURNER_ROLE cannot burn tokens if the target account has been removed from the KYC list.

## Recommended Mitigation Steps

Allow the BURNER to burn tokens without checking the KYC of from address.

diff --git a/contracts/ousg/rOUSG.sol b/contracts/ousg/rOUSG.sol index 29d9112..6809a28 100644 --- a/contracts/ousg/rOUSG.sol +++ b/contracts/ousg/rOUSG.sol @@ -594,7 +594,7 @@ contract ROUSG is require(_getKYCStatus(msg.sender), "rOUSG: 'sender' address not KYC'd"); } - if (from != address(0)) { + if (from != address(0) && !hasRole(BURNER_ROLE, msg.sender)) { // If not minting require(_getKYCStatus(from), "rOUSG: 'from' address not KYC'd"); }

## Assessed type

Invalid Validation 3docSec (judge) commented:

The reasons why I opted to keep this as Medium is:

The possibility of changing a contract implementation (in this case the registry) to a new implementation (that is not in scope) is not something that is generally accepted as a severity mitigation The same finding was already judged as a valid Medium in a previous audit with a different scope (rUSDY), that was not explicitly marked as a known issue in the README cameronclifton (Ondo) disputed and commented:

We will not be addressing this as we have a safe workaround for this exact scenario.

Note: For full discussion, see here.

## Rejected Primary Findings

# Rejected Primary Findings: Ondo Finance

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
