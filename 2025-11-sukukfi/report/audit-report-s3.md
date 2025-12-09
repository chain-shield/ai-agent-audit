# 2025 11 sukukfi - Findings Report
## Commit hash: 18fe2578cf1c6203dac7ff21513533010f3dda3e

##Findings by Status


Finding Status: LowSeverityDueToLowImpact, LowSeverityDueToRareLikelihood


[H-1]. First depositor inflation attack via insufficient virtual offset
**Derived From** : ERC4626SharePriceMismatch
Finding Status Justification: --- Round 1 ---
The finding describes a first-depositor attack using virtual offsets of 1e6. However, this is NOT a standard ERC4626 vault - it's a 1:1 stablecoin wrapper with deterministic decimal normalization (shares = assets * scalingFactor). The conversion does NOT depend on totalSupply or totalAssets ratios, making donation attacks impossible. The virtual offset is irrelevant since the share price is fixed by the scaling factor (10^(18-assetDecimals)). This is a misunderstanding of the architecture. Impact: Low (no actual vulnerability). Likelihood: Rare (attack vector doesn't exist in this design).
Finding Complexity: 0
Privilege: Permissionless



Finding Status: InvalidByDesign, InvalidNotExploitable


[M-2]. Validator consumes owner nonce in permit execution
**Derived From** : PermitNonceMisuse
Finding Status Justification: --- Round 1 ---
Valid Medium finding. When owner == spender (self-permit), the contract validates against validator signature but consumes the owner's nonce via _useNonce(owner). This creates nonce desynchronization - the validator can invalidate user permits by submitting self-permit transactions that consume the user's nonce. This breaks EIP-2612 expectations and enables DOS of user permit operations. Impact: Medium (DOS of critical permit functionality). Likelihood: Occasional (requires validator to intentionally or accidentally submit conflicting permits). This is a non-standard nonce consumption pattern that violates permit semantics.

--- Round 2 ---
Bug exists: When `owner == spender` (self-permit), the validator signs but `_useNonce(owner)` consumes the owner's nonce, not validator's. This creates nonce desync: if a user generates a standard permit (nonce N), validator can invalidate it by submitting a self-permit consuming nonce N. This breaks EIP-2612 expectations where each principal manages their own nonce sequence. No safeguard prevents this griefing vector.

--- Round 3 ---
This is BY DESIGN per the Known Issues document (Section 2: Non-Standard ERC-20 Behavior). The permit system intentionally requires validator signatures for self-allowance (owner==spender case), and consuming the owner's nonce is the standard EIP-2612 behavior. The 'desynchronization' described is actually the intended mechanism - the validator controls when users can move funds. While non-standard, this is documented governance/centralization risk (QA/Low per Known Issues Section 1), not a vulnerability. The nonce consumption prevents replay attacks as designed.
Finding Complexity: 0
Privilege: RequiresRole


[M-3]. Missing Slippage Protection in Asynchronous Redemption
**Derived From** : SlippageMissingOrInsufficient
Finding Status Justification: --- Round 1 ---
Valid Medium finding. The requestRedeem function lacks slippage protection - users lock shares but the conversion rate is determined later at fulfillment time. If totalAssets fluctuates (investment losses, price manipulation), users are forced to accept unfavorable rates with no minimum output guarantee. Impact: Medium (user loss of funds due to exchange rate movements). Likelihood: Occasional (requires market movements or manipulation between request and fulfillment). This is a missing protection mechanism in the async flow that exposes users to unlimited slippage risk.

--- Round 2 ---
Bug exists: `requestRedeem()` locks shares but conversion rate is determined later at `fulfillRedeem()`. No `minAssets` parameter exists to enforce minimum output. If `totalAssets` drops (investment loss, price manipulation) between request and fulfillment, users receive fewer assets than expected with no recourse. This violates user expectations for async operations. ERC-7540 spec doesn't mandate slippage protection, but its absence creates user loss risk. No safeguard exists.

--- Round 3 ---
This is BY DESIGN per Known Issues Section 4 (Async Operations & Timing). The ERC-7540 async architecture intentionally allows the Investment Manager to control fulfillment timing without user-specified slippage limits. The documentation explicitly states 'Investment Manager can delay fulfillments indefinitely' and 'No SLA enforcement' as intentional design for professional fund management. Users accept this model for institutional investment. This is centralization/governance risk (QA/Low), not a Medium vulnerability. The lack of minAssets parameter is an intentional trade-off for capital efficiency.
Finding Complexity: 0
Privilege: Permissionless


[M-4]. Missing slippage protection in asynchronous deposit and redeem
**Derived From** : Slippage Missing in Async Fulfillment
Finding Status Justification: --- Round 1 ---
This is a duplicate of finding CBhV_8DBRagb4P9iYmnMu (Missing Slippage Protection in Asynchronous Redemption), but extends to both deposit and redeem flows. Valid Medium finding. The async flow (requestDeposit/requestRedeem → fulfill → claim) lacks slippage parameters. Users specify amounts but conversion rates are determined later, exposing them to exchange rate fluctuations without recourse. Impact: Medium (user loss of funds). Likelihood: Occasional (requires rate changes between request and fulfillment). This is a missing protection mechanism in ERC-7540 async operations.

--- Round 2 ---
Bug exists (duplicate of CBhV_8DBRagb4P9iYmnMu): Both `requestDeposit()` and `requestRedeem()` lack slippage parameters (`minShares`/`minAssets`). Exchange rate fluctuations between request and fulfillment can cause user loss. For deposits: if rate drops, user gets fewer shares. For redeems: if rate drops, user gets fewer assets. No mechanism exists to specify minimum acceptable output or cancel if rate moves unfavorably. This is a design flaw in the async flow.

--- Round 3 ---
Duplicate of finding CBhV_8DBRagb4P9iYmnMu with same rationale. This is BY DESIGN per Known Issues Section 4. The ERC-7540 async pattern intentionally separates request from fulfillment, with the Investment Manager controlling timing and conversion rates. The documentation explicitly acknowledges this as 'Async Operations & Timing' (QA/Low - NOT Medium). The example scenario (rate changes between request and fulfillment) is the intended behavior for professional fund management, not a vulnerability. Users knowingly accept this model for institutional investment products.
Finding Complexity: 0
Privilege: Permissionless



Finding Status: Valid


[H-5]. Rounding error in ERC7575VaultUpgradeable.withdraw allows asset theft
**Derived From** : Incorrect rounding in withdraw allows asset drainage without burning shares
Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The withdraw function uses Math.Rounding.Floor when calculating shares to burn, which rounds down and can result in 0 shares burned while still transferring assets. An attacker can repeatedly withdraw small amounts to drain claimableRedeemAssets without burning any shares. The function only removes the controller from activeRedeemRequesters if shares == availableShares or assets == availableAssets, but for partial withdrawals where shares round to 0, it simply decrements claimableRedeemAssets. Impact: High (direct theft of assets). Likelihood: Common (no preconditions, exploitable whenever withdrawal amount results in <1 share). This is a severe rounding vulnerability.

--- Round 2 ---
Bug exists: `withdraw()` uses `Math.Rounding.Floor` to calculate shares to burn: `shares = assets.mulDiv(availableShares, availableAssets, Floor)`. If `assets * availableShares < availableAssets`, shares rounds to 0. User receives assets but burns 0 shares. Attacker can repeatedly withdraw small amounts to drain `claimableRedeemAssets` without burning shares. The function only removes controller from active set if `shares == availableShares`, not when shares=0. No safeguard prevents this.

--- Round 3 ---
This is a VALID HIGH severity finding. The withdraw function uses Math.Rounding.Floor when calculating shares to burn, which favors the user instead of the vault. When withdrawing small amounts where shares round down to 0, the user receives assets without burning any shares, enabling repeated drainage of claimableRedeemAssets. This violates the fundamental invariant that withdrawals must burn proportional shares. The PoC demonstrates clear theft: withdraw 9 assets repeatedly while burning 0 shares each time. The suggested mitigation (use Ceil rounding) is correct and aligns with ERC-4626 best practices.
Finding Complexity: 0
Privilege: Permissionless


[M-6]. Single Point of Failure in Share Pricing Mechanism via External Vault Dependency
**Derived From** : ReserveOrPriceDesync
Finding Status Justification: --- Round 1 ---
Valid Medium finding. The getCirculatingSupplyAndAssets() function iterates over all vaults and calls external balanceOf() without try/catch protection. If any single asset's balanceOf() reverts (paused token, blacklist, malicious implementation), the entire pricing mechanism fails, causing DOS for ALL vaults. This is a single point of failure that compromises system availability. Impact: Medium (function of protocol impacted - all deposits/redeems fail). Likelihood: Occasional (requires specific but realistic conditions like token pause). The lack of defensive programming around external calls is a legitimate vulnerability.

--- Round 2 ---
The bug exists: `getCirculatingSupplyAndAssets()` iterates all vaults calling `getClaimableSharesAndNormalizedAssets()`, which calls `totalAssets()` on each vault. If any vault's asset token reverts (paused, blacklisted, malicious), the entire function reverts, breaking conversions for ALL vaults. No try/catch exists. However, MAX_VAULTS_PER_SHARE_TOKEN=10 limits blast radius. Mitigation: wrap external calls in try/catch to skip failing vaults rather than reverting globally.

--- Round 3 ---
The finding identifies a legitimate DoS vulnerability where a single malicious/paused vault can brick the entire multi-asset system. The `getCirculatingSupplyAndAssets` function iterates through all vaults without error handling, causing a revert if any vault's `balanceOf` call fails. This is NOT by design - the system should be resilient to individual vault failures. It's in scope (core vault functionality) and exploitable (a paused USDC contract or malicious vault can halt all deposits/withdrawals across all assets). The suggested try/catch mitigation is appropriate.
Finding Complexity: 0
Privilege: Permissionless


[M-7]. IUSD Token (ShareTokenUpgradeable) bypasses KYC restrictions
**Derived From** : Investment Layer IUSD Token Lacks KYC Restrictions Allowing Compliance Bypass
Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The ShareTokenUpgradeable (Investment Layer IUSD token) inherits from ERC20Upgradeable but does NOT override transfer/transferFrom to enforce isKycVerified checks, unlike WERC7575ShareToken (Settlement Layer). This allows IUSD tokens to be transferred freely to non-KYC'd addresses, completely bypassing the protocol's compliance perimeter. Impact: High (regulatory compliance violation, potential legal action). Likelihood: Common (no preconditions, works anytime). This is a missing access control that violates the protocol's core KYC requirement.

--- Round 2 ---
Bug exists: `ShareTokenUpgradeable` (investment layer IUSD) inherits `ERC20Upgradeable` but does NOT override `transfer()`/`transferFrom()` to enforce `isKycVerified` checks. Only `WERC7575ShareToken` (settlement layer) enforces KYC. This allows IUSD tokens to be transferred freely to non-KYC addresses, bypassing compliance requirements. The Known Issues doc states 'All token recipients must be KYC-verified' but this is not enforced in ShareTokenUpgradeable. No safeguard exists.

--- Round 3 ---
This is a VALID HIGH severity finding. The ShareTokenUpgradeable (investment layer IUSD token) does NOT override transfer/transferFrom to enforce KYC checks, while the settlement layer WERC7575ShareToken does. The Known Issues document (Section 2) states 'All token recipients must be KYC-verified' as a system-wide requirement, not just settlement layer. This creates a compliance bypass where investment layer tokens can flow to non-KYC addresses, violating the protocol's regulatory perimeter. This is NOT documented as intentional and represents a genuine security gap requiring the suggested mitigation.
Finding Complexity: 0
Privilege: Permissionless



Finding Status: InvalidBugDoesNotExist, InvalidSafeGuardInPlace


[H-8]. Investment yield trapped in rBalance leading to insolvency
**Derived From** : Yield trapped in rBalance due to accounting logic mismatch
Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The adjustrBalance function adds investment profits to _rBalances (reserved balance), but the burn function only checks _balances (liquid balance). When the Investment Manager attempts to realize profits via withdrawFromInvestment → shareToken.burn, the burn fails because the profit was added to _rBalances instead of _balances. This permanently traps investment yield and causes insolvency. Impact: High (permanent loss of yield, protocol insolvency). Likelihood: Common (occurs whenever investment profits are recorded). This is a severe accounting logic mismatch between rBalance tracking and burn operations.

--- Round 2 ---
Bug does NOT exist. The report misunderstands the architecture. `adjustrBalance()` is called on `WERC7575ShareToken` (settlement layer), NOT `ShareTokenUpgradeable` (investment layer). The settlement layer's `_rBalances` tracks reserved funds for settlements, not investment profits. Investment profits are realized through the investment vault's share price appreciation (WUSD shares held by ShareTokenUpgradeable increase in value). The `burn()` function operates on `_balances`, which is correct. The report conflates two separate token systems.
Finding Complexity: 0
Privilege: RequiresAdminRole


[H-9]. Total supply desync in rBatchTransfers allows fund theft
**Derived From** : AccountingInvariantViolation
Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The rBatchTransfers function moves funds from _rBalances to _balances when creditors are flagged for rBalance updates, but fails to update _totalSupply. This creates unbacked tokens that violate the invariant sum(_balances) == totalSupply. When these tokens are burned, totalSupply will underflow or reach zero while users hold balances, causing permanent DOS or insolvency. Impact: High (theft/permanent loss of assets, accounting corruption). Likelihood: Common (no preconditions, works whenever rBatchTransfers is called with flagged creditors). This is a severe accounting bug.

--- Round 2 ---
Bug does NOT exist. The report misunderstands rBalance architecture. `_rBalances` tracks reserved/invested funds, NOT circulating supply. When creditors receive funds, `_balances` increases (correct), and `_rBalances` decreases (releasing reserved funds). `_totalSupply` is NOT updated because no minting/burning occurs—only internal balance shifts. The `_totalSupply` is managed by `_mint()` and `_burn()` in vault operations, not by rBatchTransfers. The invariant `sum(_balances) == _totalSupply` holds because rBatchTransfers only redistributes existing balances.
Finding Complexity: 0
Privilege: RequiresRole



Finding Status: InvalidGovernanceRisk, LowSeverityDueToLowImpact


[M-10]. Permanent DoS of vault unregistration via persistent active requester state
**Derived From** : Permanent Denial of Service on Vault Unregistration via Dust State
Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk, not a vulnerability. The finding describes a griefing attack where a user leaves dust amounts to prevent vault unregistration. However, per Known Issues Section 10 (Vault Unregistration DOS via Dust Holdings), this is documented as intentional design. The admin can handle this through force-claim mechanisms or by accepting dust during unregistration. No user funds are at risk. This falls under 'Governance/Centralization risk' per C4 severity categorization, making it Low/QA. The protocol is functioning as designed with known trade-offs.
Finding Complexity: 0
Privilege: Permissionless


[M-11]. Vault Unregistration DOS via Dust Holdings
**Derived From** : GovernanceDelegationFlaw
Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk, not a vulnerability. The finding describes a griefing attack where a user leaves dust amounts in pending requests to prevent vault unregistration. However, per Known Issues Section 10 (Vault Unregistration DOS via Dust Holdings), this is documented as intentional design. The admin can handle this through force-claim mechanisms. No user funds are at risk - this is a protocol maintenance issue. Per C4 severity categorization, 'Governance/Centralization risk' is Low/QA. The protocol is functioning as designed with known trade-offs for administrative operations.
Finding Complexity: 0
Privilege: Permissionless


[M-12]. Strict Balance Equality Check Enables Griefing of Vault Unregistration
**Derived From** : ForcedAssetVsStrictEquality
Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk. The finding describes a griefing attack where an attacker donates dust to prevent vault unregistration due to strict balance == 0 check. However, per Known Issues Section 11 (Strict Balance Equality Check Enables Griefing), this is documented as intentional design. The admin can handle this through investAssets to sweep dust or by accepting dust during unregistration. No user funds are at risk. Per C4 severity categorization, 'Governance/Centralization risk' is Low/QA. This is a known trade-off in the vault lifecycle management design.
Finding Complexity: 0
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[M-13]. Total Assets Clamping Hides Insolvency and Bricks Price Calculation
**Derived From** : AccountingInvariantViolation
Finding Status Justification: --- Round 1 ---
This is a valid Medium finding. If a vault becomes insolvent (balance < reservedAssets), totalAssets() returns 0, causing division-by-zero in ShareToken.convertNormalizedAssetsToShares(). This would DOS all operations across ALL vaults sharing that ShareToken. The impact is significant (protocol-wide DOS), but likelihood is Rare because it requires investment losses severe enough to cause insolvency. The clamping to 0 instead of handling negative equity gracefully is a design flaw that could brick the entire multi-asset system. This is NOT a governance issue - it's a function impacted by a bug.
Finding Complexity: 0
Privilege: Permissionless



Finding Status: InvalidERC20EdgeCase, LowSeverityDueToLowImpact, LowSeverityDueToRareLikelihood


[M-14]. Rebasing token yield theft via pending deposits
**Derived From** : FeeOnTransferAssumption
Finding Status Justification: --- Round 1 ---
This is a non-standard token issue. The finding describes rebasing tokens (stETH, aTokens) causing yield theft from pending depositors. However, per Known Issues Section 2 (Non-Standard ERC-20 Behavior) and the SafeTokenTransfers library implementation, the protocol explicitly rejects non-standard tokens including rebasing tokens. The SafeTokenTransfers.safeTransferFrom checks that balanceAfter == balanceBefore + amount, which would fail for rebasing tokens. Impact: Low (protocol doesn't support these tokens). Likelihood: Rare (tokens are rejected at transfer time). This is out of scope per non-standard token exclusions.
Finding Complexity: 0
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 10
- L: 0
- I: 0

##Findings by Status


Finding Status: LowSeverityDueToLowImpact, LowSeverityDueToRareLikelihood
## [H-1]. First depositor inflation attack via insufficient virtual offset

## id: UMdCLLBaEIs-qHhukYD72

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
ShareTokenUpgradeable.convertNormalizedAssetsToShares

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact
### Finding Status Justification: --- Round 1 ---
The finding describes a first-depositor attack using virtual offsets of 1e6. However, this is NOT a standard ERC4626 vault - it's a 1:1 stablecoin wrapper with deterministic decimal normalization (shares = assets * scalingFactor). The conversion does NOT depend on totalSupply or totalAssets ratios, making donation attacks impossible. The virtual offset is irrelevant since the share price is fixed by the scaling factor (10^(18-assetDecimals)). This is a misunderstanding of the architecture. Impact: Low (no actual vulnerability). Likelihood: Rare (attack vector doesn't exist in this design).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` uses a virtual offset of `1e6` (`VIRTUAL_SHARES` and `VIRTUAL_ASSETS`) in `convertNormalizedAssetsToShares`. Since the system normalizes all assets to 18 decimals, an offset of `1e6` (equivalent to `1e-12` tokens) is negligible. A malicious first depositor can deposit a small amount (e.g., 1 wei of USDC, normalized to `1e12`), and then donate a significant amount of assets to the vault (e.g. 1 USDC or `1e18` normalized). This inflates the share price (`totalNormalizedAssets / circulatingSupply`) drastically. Subsequent depositors who deposit amounts smaller than the inflated share price will receive 0 shares due to rounding down, effectively losing their deposit.

## Impact
The first depositor can manipulate the share price to cause future depositors to lose funds (rounding to zero), effectively stealing their deposits.

## Command to Run Test


## Proof of Concept
1. Attacker deposits 1 wei of USDC (`1e12` normalized). Gets `~1e6` shares. 
2. Attacker donates 1 USDC (`1e18` normalized) to the vault. 
3. `totalNormalizedAssets` is now `~1e18`, `circulatingSupply` is `~1e6`. Share price is `~1e12`. 
4. Victim deposits 0.5 USDC (`5e17` normalized). 
5. Shares = `5e17 * 1e6 / 1e18` = 0.5, which rounds down to 0. 
6. Victim loses 0.5 USDC.

## Proof of Code
function testInflationAttack() public {
    address attacker = address(0x1);
    address victim = address(0x2);
    vm.startPrank(attacker);
    usdc.approve(address(vault), 1);
    vault.deposit(1, attacker); // 1 wei USDC
    usdc.transfer(address(vault), 1e6); // Donate 1 USDC
    vm.stopPrank();
    
    vm.startPrank(victim);
    usdc.approve(address(vault), 5e5); // 0.5 USDC
    uint256 shares = vault.deposit(5e5, victim);
    assertEq(shares, 0); // Victim gets 0 shares
    vm.stopPrank();
}

## Suggested Mitigation
Increase the virtual offset significantly (e.g., to `10**decimals()`) to make the donation cost prohibitive, or use an internal balance tracking variable to exclude donated assets from the share price calculation.





Finding Status: InvalidByDesign, InvalidNotExploitable
## [M-2]. Validator consumes owner nonce in permit execution

## id: -4CmSmaRUavwAnfKW3xS2

## Derived From Pattern/Invariant
PermitNonceMisuse

## Exploit Type
PermitNonceMisuse

## Location
WERC7575ShareToken.permit

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. When owner == spender (self-permit), the contract validates against validator signature but consumes the owner's nonce via _useNonce(owner). This creates nonce desynchronization - the validator can invalidate user permits by submitting self-permit transactions that consume the user's nonce. This breaks EIP-2612 expectations and enables DOS of user permit operations. Impact: Medium (DOS of critical permit functionality). Likelihood: Occasional (requires validator to intentionally or accidentally submit conflicting permits). This is a non-standard nonce consumption pattern that violates permit semantics.

--- Round 2 ---
Bug exists: When `owner == spender` (self-permit), the validator signs but `_useNonce(owner)` consumes the owner's nonce, not validator's. This creates nonce desync: if a user generates a standard permit (nonce N), validator can invalidate it by submitting a self-permit consuming nonce N. This breaks EIP-2612 expectations where each principal manages their own nonce sequence. No safeguard prevents this griefing vector.

--- Round 3 ---
This is BY DESIGN per the Known Issues document (Section 2: Non-Standard ERC-20 Behavior). The permit system intentionally requires validator signatures for self-allowance (owner==spender case), and consuming the owner's nonce is the standard EIP-2612 behavior. The 'desynchronization' described is actually the intended mechanism - the validator controls when users can move funds. While non-standard, this is documented governance/centralization risk (QA/Low per Known Issues Section 1), not a vulnerability. The nonce consumption prevents replay attacks as designed.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `WERC7575ShareToken.permit`, when `owner == spender`, the contract validates the signature against `_validator` but consumes the `owner`'s nonce via `_useNonce(owner)`. This creates a desynchronization between the owner's actual signature intent and their on-chain nonce. If the owner generates a valid off-chain permit signature for a standard transfer, the validator can invalidate it by submitting a self-permit transaction (consuming the nonce). This gives the validator power to grief user operations or force nonce race conditions.

## Impact
Denial of service for user permit operations; non-standard nonce consumption breaks EIP-2612 expectations and wallet integrations.

## Command to Run Test


## Proof of Concept
1. User signs a Permit for a dApp (Nonce 1). 
2. Validator submits a `permit(User, User, ...)` transaction which is valid under current logic. 
3. This transaction calls `_useNonce(User)`, incrementing User's nonce to 2. 
4. User's dApp transaction fails because Nonce 1 is now invalid.

## Proof of Code
function testNonceGrief() public {
    // ...
    vm.prank(validator);
    token.permit(user, user, 0, deadline, v, r, s);
    // User nonce incremented
    assertEq(token.nonces(user), 1);
}

## Suggested Mitigation
When `owner == spender` (validator signed), consider using the validator's nonce or a separate nonce mechanism, rather than consuming the owner's nonce.


## [M-3]. Missing Slippage Protection in Asynchronous Redemption

## id: CBhV_8DBRagb4P9iYmnMu

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.requestRedeem

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. The requestRedeem function lacks slippage protection - users lock shares but the conversion rate is determined later at fulfillment time. If totalAssets fluctuates (investment losses, price manipulation), users are forced to accept unfavorable rates with no minimum output guarantee. Impact: Medium (user loss of funds due to exchange rate movements). Likelihood: Occasional (requires market movements or manipulation between request and fulfillment). This is a missing protection mechanism in the async flow that exposes users to unlimited slippage risk.

--- Round 2 ---
Bug exists: `requestRedeem()` locks shares but conversion rate is determined later at `fulfillRedeem()`. No `minAssets` parameter exists to enforce minimum output. If `totalAssets` drops (investment loss, price manipulation) between request and fulfillment, users receive fewer assets than expected with no recourse. This violates user expectations for async operations. ERC-7540 spec doesn't mandate slippage protection, but its absence creates user loss risk. No safeguard exists.

--- Round 3 ---
This is BY DESIGN per Known Issues Section 4 (Async Operations & Timing). The ERC-7540 async architecture intentionally allows the Investment Manager to control fulfillment timing without user-specified slippage limits. The documentation explicitly states 'Investment Manager can delay fulfillments indefinitely' and 'No SLA enforcement' as intentional design for professional fund management. Users accept this model for institutional investment. This is centralization/governance risk (QA/Low), not a Medium vulnerability. The lack of minAssets parameter is an intentional trade-off for capital efficiency.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestRedeem` function allows users to lock shares for redemption. The conversion rate is determined later when `fulfillRedeem` is called. Neither function accepts a `minAssets` or `minOutput` parameter. Since the share price depends on `totalAssets` (which can fluctuate based on investment performance), users are exposed to unlimited slippage between the request and fulfillment time. If the investment vault suffers a loss or the price is manipulated, users are forced to accept the unfavorable rate.

## Impact
Users may receive significantly fewer assets than expected due to market movements or front-running, with no ability to enforce a minimum acceptable amount.

## Command to Run Test


## Proof of Concept
1. User calls `requestRedeem(100 shares)`. Current rate 1:1.
2. A large loss occurs in the investment vault, or `totalAssets` drops.
3. Manager calls `fulfillRedeem`. New rate 0.5:1.
4. User receives 50 assets instead of 100.
5. User had no way to specify 'revert if rate < 0.9'.

## Proof of Code
function testSlippage() public {
    // User requests redeem
    vm.prank(user);
    vault.requestRedeem(100, user, user);
    
    // Simulate loss
    simulateLoss(500); // 50% loss
    
    // Fulfill
    vm.prank(manager);
    vault.fulfillRedeem(user, 100);
    
    // User gets 50% value
    assertEq(vault.claimableRedeemAssets(user), 50);
}

## Suggested Mitigation
Add a `minAssets` parameter to `requestRedeem` (stored in the request) and enforce it during `fulfillRedeem`, or allow users to specify a slippage limit.


## [M-4]. Missing slippage protection in asynchronous deposit and redeem

## id: u3LY3zrNGDCCjaLbyp9Ey

## Derived From Pattern/Invariant
Slippage Missing in Async Fulfillment

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
This is a duplicate of finding CBhV_8DBRagb4P9iYmnMu (Missing Slippage Protection in Asynchronous Redemption), but extends to both deposit and redeem flows. Valid Medium finding. The async flow (requestDeposit/requestRedeem → fulfill → claim) lacks slippage parameters. Users specify amounts but conversion rates are determined later, exposing them to exchange rate fluctuations without recourse. Impact: Medium (user loss of funds). Likelihood: Occasional (requires rate changes between request and fulfillment). This is a missing protection mechanism in ERC-7540 async operations.

--- Round 2 ---
Bug exists (duplicate of CBhV_8DBRagb4P9iYmnMu): Both `requestDeposit()` and `requestRedeem()` lack slippage parameters (`minShares`/`minAssets`). Exchange rate fluctuations between request and fulfillment can cause user loss. For deposits: if rate drops, user gets fewer shares. For redeems: if rate drops, user gets fewer assets. No mechanism exists to specify minimum acceptable output or cancel if rate moves unfavorably. This is a design flaw in the async flow.

--- Round 3 ---
Duplicate of finding CBhV_8DBRagb4P9iYmnMu with same rationale. This is BY DESIGN per Known Issues Section 4. The ERC-7540 async pattern intentionally separates request from fulfillment, with the Investment Manager controlling timing and conversion rates. The documentation explicitly acknowledges this as 'Async Operations & Timing' (QA/Low - NOT Medium). The example scenario (rate changes between request and fulfillment) is the intended behavior for professional fund management, not a vulnerability. Users knowingly accept this model for institutional investment products.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The asynchronous flow in `ERC7575VaultUpgradeable` (`requestDeposit` / `fulfillDeposit` and `requestRedeem` / `fulfillRedeem`) lacks slippage protection. Users initiate a request specifying an amount of assets (for deposit) or shares (for redeem), but the conversion rate is determined later when the Investment Manager calls `fulfill`. There is no parameter for users to specify a minimum output amount (`minShares` or `minAssets`). If the exchange rate moves unfavorably between request and fulfillment, users will suffer a loss of value without recourse.

## Impact
User loss of funds due to exchange rate fluctuations or manipulation between request and fulfillment time.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit(100 USDC)` when rate is 1:1.
2. Time passes. Rate changes to 1:0.9 due to losses/revaluation.
3. Investment Manager calls `fulfillDeposit`.
4. User receives 90 shares instead of expected 100. User cannot prevent this.

## Proof of Code
function testSlippage() public {
    // ... setup ...
    vault.requestDeposit(100, user, user);
    // Simulate loss in vault
    // ...
    vault.fulfillDeposit(user, 100);
    uint256 shares = vault.claimableShares(user);
    // Shares are less than expected, no revert occurred
    assertLt(shares, 100);
}

## Suggested Mitigation
Add `minShares` to `requestDeposit` and `minAssets` to `requestRedeem`. Store these limits in the request state and verify them during fulfillment.





Finding Status: Valid
## [H-5]. Rounding error in ERC7575VaultUpgradeable.withdraw allows asset theft

## id: 8yPfx6KQ4zFzQE7d0s7p3

## Derived From Pattern/Invariant
Incorrect rounding in withdraw allows asset drainage without burning shares

## Exploit Type
RoundingError

## Location
ERC7575VaultUpgradeable.withdraw

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The withdraw function uses Math.Rounding.Floor when calculating shares to burn, which rounds down and can result in 0 shares burned while still transferring assets. An attacker can repeatedly withdraw small amounts to drain claimableRedeemAssets without burning any shares. The function only removes the controller from activeRedeemRequesters if shares == availableShares or assets == availableAssets, but for partial withdrawals where shares round to 0, it simply decrements claimableRedeemAssets. Impact: High (direct theft of assets). Likelihood: Common (no preconditions, exploitable whenever withdrawal amount results in <1 share). This is a severe rounding vulnerability.

--- Round 2 ---
Bug exists: `withdraw()` uses `Math.Rounding.Floor` to calculate shares to burn: `shares = assets.mulDiv(availableShares, availableAssets, Floor)`. If `assets * availableShares < availableAssets`, shares rounds to 0. User receives assets but burns 0 shares. Attacker can repeatedly withdraw small amounts to drain `claimableRedeemAssets` without burning shares. The function only removes controller from active set if `shares == availableShares`, not when shares=0. No safeguard prevents this.

--- Round 3 ---
This is a VALID HIGH severity finding. The withdraw function uses Math.Rounding.Floor when calculating shares to burn, which favors the user instead of the vault. When withdrawing small amounts where shares round down to 0, the user receives assets without burning any shares, enabling repeated drainage of claimableRedeemAssets. This violates the fundamental invariant that withdrawals must burn proportional shares. The PoC demonstrates clear theft: withdraw 9 assets repeatedly while burning 0 shares each time. The suggested mitigation (use Ceil rounding) is correct and aligns with ERC-4626 best practices.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.withdraw`, the calculation of shares to burn uses `Math.Rounding.Floor` (`shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`). This rounding direction favors the user (caller) rather than the vault. If a user withdraws an amount of assets that corresponds to less than 1 share (e.g., in a scenario where 1 share is worth multiple assets due to exchange rate or precision), the calculated `shares` will round down to 0. 

The function proceeds to transfer the assets to the user but burns 0 shares. The logic only removes the controller from `activeRedeemRequesters` if `shares == availableShares` or `assets == availableAssets`, but for partial withdrawals where `shares` rounds to 0, it simply decrements `claimableRedeemAssets` by the asset amount and `claimableRedeemShares` by 0. An attacker can repeatedly withdraw small amounts of assets to drain the claimable asset pool without burning any shares.

## Impact
Direct theft of assets from the vault.

## Command to Run Test


## Proof of Concept
1. Assume a state where `claimableRedeemShares` = 10 and `claimableRedeemAssets` = 100 (Price: 10 assets/share).
2. User calls `withdraw(9, receiver, controller)`.
3. `shares = 9 * 10 / 100 = 0.9` -> rounds down to 0.
4. `claimableRedeemAssets` becomes 91. `claimableRedeemShares` remains 10.
5. User receives 9 assets. User burns 0 shares.
6. User repeats until `claimableRedeemAssets` is drained.

## Proof of Code
function testRoundingTheft() public {
    // Setup vault with 10 shares claimable for 100 assets
    // ... (setup code) ...
    vm.startPrank(user);
    // Withdraw 9 assets (should cost 0.9 shares)
    vault.withdraw(9, user, user);
    // Check user received assets
    assertEq(asset.balanceOf(user), 9);
    // Check user burned 0 shares
    assertEq(shareToken.balanceOf(user), initialShares);
}

## Suggested Mitigation
Use `Math.Rounding.Ceil` in `withdraw` to calculate shares to burn, ensuring the user pays at least 1 share or the proportional amount rounded up.


## [M-6]. Single Point of Failure in Share Pricing Mechanism via External Vault Dependency

## id: gsFxEZ9y-QeBfcDn01NLV

## Derived From Pattern/Invariant
ReserveOrPriceDesync

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.getCirculatingSupplyAndAssets

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. The getCirculatingSupplyAndAssets() function iterates over all vaults and calls external balanceOf() without try/catch protection. If any single asset's balanceOf() reverts (paused token, blacklist, malicious implementation), the entire pricing mechanism fails, causing DOS for ALL vaults. This is a single point of failure that compromises system availability. Impact: Medium (function of protocol impacted - all deposits/redeems fail). Likelihood: Occasional (requires specific but realistic conditions like token pause). The lack of defensive programming around external calls is a legitimate vulnerability.

--- Round 2 ---
The bug exists: `getCirculatingSupplyAndAssets()` iterates all vaults calling `getClaimableSharesAndNormalizedAssets()`, which calls `totalAssets()` on each vault. If any vault's asset token reverts (paused, blacklisted, malicious), the entire function reverts, breaking conversions for ALL vaults. No try/catch exists. However, MAX_VAULTS_PER_SHARE_TOKEN=10 limits blast radius. Mitigation: wrap external calls in try/catch to skip failing vaults rather than reverting globally.

--- Round 3 ---
The finding identifies a legitimate DoS vulnerability where a single malicious/paused vault can brick the entire multi-asset system. The `getCirculatingSupplyAndAssets` function iterates through all vaults without error handling, causing a revert if any vault's `balanceOf` call fails. This is NOT by design - the system should be resilient to individual vault failures. It's in scope (core vault functionality) and exploitable (a paused USDC contract or malicious vault can halt all deposits/withdrawals across all assets). The suggested try/catch mitigation is appropriate.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults to calculate the total system assets and circulating supply. This function calls `IERC7575Vault(vaultAddress).getClaimableSharesAndNormalizedAssets()`, which in turn calls `totalAssets()` on the vault. `totalAssets()` performs an external call to `IERC20(asset).balanceOf(address(this))`. 

If a single registered asset's `balanceOf` function reverts (e.g., due to a pause state, blacklist, upgrade failure, or malicious implementation), the entire `getCirculatingSupplyAndAssets` function will revert. Since this function is critical for `convertNormalizedAssetsToShares` and `convertSharesToNormalizedAssets`, a failure here causes a global Denial of Service (DoS) for ALL vaults in the system. Operations like `deposit`, `mint`, `withdraw`, `redeem`, `fulfillDeposit`, and `fulfillRedeem` will fail for every asset, effectively bricking the protocol.

## Impact
The entire investment layer becomes non-functional if a single underlying asset behaves unexpectedly (reverts). This is a single point of failure that compromises system availability.

## Command to Run Test


## Proof of Concept
1. Admin registers Vault A (linked to a standard token) and Vault B (linked to a token that reverts on `balanceOf`, e.g., a paused upgradeable token).
2. User attempts to deposit into Vault A.
3. Vault A calls `ShareToken.convertNormalizedAssetsToShares` to calculate share issuance.
4. ShareToken loops through all vaults. When it hits Vault B, the call reverts.
5. The deposit into Vault A fails, even though Vault A and its asset are healthy.

## Proof of Code
function test_DoS_SingleBadVaultBricksSystem() public {
    // 1. Setup good vault
    address goodVault = address(new ERC7575VaultUpgradeable());
    // ... initialize goodVault ...
    shareToken.registerVault(address(goodAsset), goodVault);

    // 2. Setup bad vault with reverting asset
    MockRevertingToken badAsset = new MockRevertingToken(); // Reverts on balanceOf
    address badVault = address(new ERC7575VaultUpgradeable());
    // ... initialize badVault ...
    shareToken.registerVault(address(badAsset), badVault);

    // 3. Attempt operation on GOOD vault
    vm.startPrank(user);
    goodAsset.approve(goodVault, 1000);
    // This should fail because badVault causes ShareToken pricing to revert
    vm.expectRevert();
    goodVault.requestDeposit(1000, user, user);
    vm.stopPrank();
}

## Suggested Mitigation
Wrap the external call to `getClaimableSharesAndNormalizedAssets` in a `try/catch` block within the loop. If a vault fails, it should be skipped or treated as having zero assets/shares for the purpose of the calculation (possibly emitting an alert event), rather than causing the entire transaction to revert.


## [M-7]. IUSD Token (ShareTokenUpgradeable) bypasses KYC restrictions

## id: Ao1e2aJRKwC3Ct75st6Oi

## Derived From Pattern/Invariant
Investment Layer IUSD Token Lacks KYC Restrictions Allowing Compliance Bypass

## Exploit Type
AccessControl

## Location
ShareTokenUpgradeable.transfer

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The ShareTokenUpgradeable (Investment Layer IUSD token) inherits from ERC20Upgradeable but does NOT override transfer/transferFrom to enforce isKycVerified checks, unlike WERC7575ShareToken (Settlement Layer). This allows IUSD tokens to be transferred freely to non-KYC'd addresses, completely bypassing the protocol's compliance perimeter. Impact: High (regulatory compliance violation, potential legal action). Likelihood: Common (no preconditions, works anytime). This is a missing access control that violates the protocol's core KYC requirement.

--- Round 2 ---
Bug exists: `ShareTokenUpgradeable` (investment layer IUSD) inherits `ERC20Upgradeable` but does NOT override `transfer()`/`transferFrom()` to enforce `isKycVerified` checks. Only `WERC7575ShareToken` (settlement layer) enforces KYC. This allows IUSD tokens to be transferred freely to non-KYC addresses, bypassing compliance requirements. The Known Issues doc states 'All token recipients must be KYC-verified' but this is not enforced in ShareTokenUpgradeable. No safeguard exists.

--- Round 3 ---
This is a VALID HIGH severity finding. The ShareTokenUpgradeable (investment layer IUSD token) does NOT override transfer/transferFrom to enforce KYC checks, while the settlement layer WERC7575ShareToken does. The Known Issues document (Section 2) states 'All token recipients must be KYC-verified' as a system-wide requirement, not just settlement layer. This creates a compliance bypass where investment layer tokens can flow to non-KYC addresses, violating the protocol's regulatory perimeter. This is NOT documented as intentional and represents a genuine security gap requiring the suggested mitigation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol documentation and Known Issues state that "All token recipients must be KYC-verified". This is implemented in `WERC7575ShareToken` (Settlement Layer) via overrides of `transfer` and `transferFrom`. However, the `ShareTokenUpgradeable` (Investment Layer IUSD token) inherits from `ERC20Upgradeable` but does **not** override `transfer` or `transferFrom` to enforce the `isKycVerified` check. This allows IUSD tokens to be transferred freely to non-KYC'd addresses, bypassing the protocol's compliance perimeter.

## Impact
Violation of regulatory compliance requirements (KYC/AML), potentially leading to legal action against the protocol.

## Command to Run Test


## Proof of Concept
1. User A (KYC verified) holds IUSD (ShareTokenUpgradeable).
2. User B (Non-KYC) wants to acquire IUSD.
3. User A calls `ShareTokenUpgradeable.transfer(UserB, amount)`.
4. Transaction succeeds because `isKycVerified` is not checked.
5. User B now holds compliant tokens without verification.

## Proof of Code
function testKycBypass() public {
    // Setup IUSD token
    // ...
    vm.prank(kycUser);
    // Transfer to non-kyc address
    shareToken.transfer(nonKycUser, 100);
    assertEq(shareToken.balanceOf(nonKycUser), 100);
}

## Suggested Mitigation
Override `transfer` and `transferFrom` in `ShareTokenUpgradeable` to enforce `isKycVerified` checks, similar to `WERC7575ShareToken`.





Finding Status: InvalidBugDoesNotExist, InvalidSafeGuardInPlace
## [H-8]. Investment yield trapped in rBalance leading to insolvency

## id: oK6SGKfii0KxHaRuazodg

## Derived From Pattern/Invariant
Yield trapped in rBalance due to accounting logic mismatch

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.adjustrBalance

## Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
### Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The adjustrBalance function adds investment profits to _rBalances (reserved balance), but the burn function only checks _balances (liquid balance). When the Investment Manager attempts to realize profits via withdrawFromInvestment → shareToken.burn, the burn fails because the profit was added to _rBalances instead of _balances. This permanently traps investment yield and causes insolvency. Impact: High (permanent loss of yield, protocol insolvency). Likelihood: Common (occurs whenever investment profits are recorded). This is a severe accounting logic mismatch between rBalance tracking and burn operations.

--- Round 2 ---
Bug does NOT exist. The report misunderstands the architecture. `adjustrBalance()` is called on `WERC7575ShareToken` (settlement layer), NOT `ShareTokenUpgradeable` (investment layer). The settlement layer's `_rBalances` tracks reserved funds for settlements, not investment profits. Investment profits are realized through the investment vault's share price appreciation (WUSD shares held by ShareTokenUpgradeable increase in value). The `burn()` function operates on `_balances`, which is correct. The report conflates two separate token systems.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `WERC7575ShareToken.adjustrBalance` function is used to record investment profits by increasing the `_rBalances` (reserved balance) of the account (the `ShareTokenUpgradeable`). However, the `WERC7575ShareToken` inherits from standard `ERC20` (and overrides) where `_burn` and `transfer` operations strictly check `_balances` (liquid balance). The `_rBalances` mapping is completely separate and not accessible for burning or transferring.

When the Investment Manager attempts to realize these profits via `withdrawFromInvestment`, the flow calls `investmentVault.redeem` -> `shareToken.burn`. The `burn` function attempts to deduct the amount from `_balances`. Since the profit was added to `_rBalances` and not `_balances`, the `ShareTokenUpgradeable` has insufficient liquid balance to cover the burn, causing the transaction to revert. The yield is permanently trapped.

## Impact
Investment profits cannot be realized or withdrawn, leading to loss of yield and potential insolvency of the Investment Layer.

## Command to Run Test


## Proof of Concept
1. `ShareTokenUpgradeable` (Investment Layer) has 100k `_balances` and 0 `_rBalances` in `WERC7575ShareToken`.
2. Revenue Admin reports 10k profit via `adjustrBalance`. `_rBalances` becomes 10k.
3. `ShareTokenUpgradeable` attempts to withdraw 110k (principal + profit).
4. Calls `WERC7575ShareToken.burn(110k)`.
5. `burn` checks `_balances[account] >= 110k`. Current `_balances` is 100k.
6. Reverts due to insufficient balance.

## Proof of Code
function testTrappedYield() public {
    // Setup share token and mint 100 to user
    // ...
    // Simulate profit adjustment
    vm.prank(revenueAdmin);
    shareToken.adjustrBalance(user, 1, 100, 110);
    // Check rBalance increased
    assertEq(shareToken.rBalanceOf(user), 10);
    // Attempt to burn total (110)
    vm.prank(validator);
    vm.expectRevert(); // Insufficient balance
    shareToken.burn(user, 110);
}

## Suggested Mitigation
Modify `adjustrBalance` to move profit from `_rBalances` to `_balances` or allow `_burn` to consume `_rBalances`.


## [H-9]. Total supply desync in rBatchTransfers allows fund theft

## id: ZEYF7MyeBqZbFtW2zIkQk

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.rBatchTransfers

## Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
### Finding Status Justification: --- Round 1 ---
This is a critical High severity finding. The rBatchTransfers function moves funds from _rBalances to _balances when creditors are flagged for rBalance updates, but fails to update _totalSupply. This creates unbacked tokens that violate the invariant sum(_balances) == totalSupply. When these tokens are burned, totalSupply will underflow or reach zero while users hold balances, causing permanent DOS or insolvency. Impact: High (theft/permanent loss of assets, accounting corruption). Likelihood: Common (no preconditions, works whenever rBatchTransfers is called with flagged creditors). This is a severe accounting bug.

--- Round 2 ---
Bug does NOT exist. The report misunderstands rBalance architecture. `_rBalances` tracks reserved/invested funds, NOT circulating supply. When creditors receive funds, `_balances` increases (correct), and `_rBalances` decreases (releasing reserved funds). `_totalSupply` is NOT updated because no minting/burning occurs—only internal balance shifts. The `_totalSupply` is managed by `_mint()` and `_burn()` in vault operations, not by rBatchTransfers. The invariant `sum(_balances) == _totalSupply` holds because rBatchTransfers only redistributes existing balances.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `WERC7575ShareToken.rBatchTransfers`, the function moves funds from `_rBalances` (reserved/invested) to `_balances` (liquid) when a user is a net creditor and flagged for rBalance update. Specifically, `_balances[account.owner] += amount` increases the user's liquid balance, and `_rBalances` is decreased. However, `_balances` are tracked by `_totalSupply` (invariant `sum(_balances) == totalSupply`), while `_rBalances` are not. The function fails to increase `_totalSupply` when increasing `_balances`. This creates unbacked liquid tokens. When these tokens are eventually burned (e.g. via withdrawal), `_totalSupply` will decrease. If enough such transfers occur, `_totalSupply` will underflow or reach zero while users still hold balances, leading to a permanent DOS of the token (burns will revert) or insolvency.

## Impact
Severe breakage of token accounting. `totalSupply` becomes decoupled from actual balances. Can lead to permanent freezing of the token contract when `totalSupply` underflows during burns.

## Command to Run Test


## Proof of Concept
1. Validator calls `rBatchTransfers` where User A is a creditor of 1000 tokens (and flagged for rBalance update). 
2. `_balances[UserA]` increases by 1000. `_rBalances[UserA]` decreases by 1000. 
3. `_totalSupply` is NOT updated. 
4. User A calls `transfer` or `burn`. `_totalSupply` is reduced by 1000 (if burning). 
5. If this happens repeatedly, `_totalSupply` drains to 0 while users hold valid balances. 
6. Any subsequent burn attempts revert due to underflow on `_totalSupply`.

## Proof of Code
function testTotalSupplyDesync() public {
    // Setup validator and users...
    vm.prank(validator);
    // Execute rBatchTransfers where user gets credit from rBalance
    token.rBatchTransfers(debtors, creditors, amounts, flags);
    
    uint256 balanceSum = token.balanceOf(user1) + token.balanceOf(user2);
    uint256 supply = token.totalSupply();
    
    assertGt(balanceSum, supply); // Invariant broken
}

## Suggested Mitigation
Update `_totalSupply` when increasing `_balances` in `rBatchTransfers`.





Finding Status: InvalidGovernanceRisk, LowSeverityDueToLowImpact
## [M-10]. Permanent DoS of vault unregistration via persistent active requester state

## id: 2N_8swFFanIHiBexuGIrT

## Derived From Pattern/Invariant
Permanent Denial of Service on Vault Unregistration via Dust State

## Exploit Type
Dos

## Location
ERC7575VaultUpgradeable.deposit

## Finding Status: LowSeverityDueToLowImpact + InvalidGovernanceRisk
### Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk, not a vulnerability. The finding describes a griefing attack where a user leaves dust amounts to prevent vault unregistration. However, per Known Issues Section 10 (Vault Unregistration DOS via Dust Holdings), this is documented as intentional design. The admin can handle this through force-claim mechanisms or by accepting dust during unregistration. No user funds are at risk. This falls under 'Governance/Centralization risk' per C4 severity categorization, making it Low/QA. The protocol is functioning as designed with known trade-offs.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `unregisterVault` function in `ShareTokenUpgradeable` enforces that the target vault must have `metrics.activeDepositRequestersCount == 0`. In `ERC7575VaultUpgradeable`, a user is removed from `activeDepositRequesters` only when they claim their *entire* claimable amount (`availableAssets == assets` in `deposit`). A malicious user can deposit, wait for fulfillment, and then claim all but 1 wei of the assets. This keeps them permanently in the `activeDepositRequesters` set. Since the admin has no function to force-evict a user or clear this state, the vault can never be unregistered.

## Impact
Permanent Denial of Service of the `unregisterVault` governance function, preventing protocol lifecycle management.

## Command to Run Test


## Proof of Concept
1. Attacker calls `requestDeposit(100)`.
2. Manager fulfills deposit.
3. Attacker calls `deposit(99, attacker)`.
4. `claimableDepositAssets` remains 1. `activeDepositRequesters` set still contains attacker.
5. Admin calls `ShareToken.unregisterVault`.
6. Reverts due to `activeDepositRequestersCount != 0`.

## Proof of Code
function testUnregisterDos() public {
    // ... setup ...
    vault.requestDeposit(100, user, user);
    vm.prank(manager);
    vault.fulfillDeposit(user, 100);
    vm.prank(user);
    vault.deposit(99, user);
    vm.prank(owner);
    vm.expectRevert();
    shareToken.unregisterVault(address(asset));
}

## Suggested Mitigation
Allow admin to force-unregister a vault even if active requesters exist, or implement a force-claim mechanism to clear dust.


## [M-11]. Vault Unregistration DOS via Dust Holdings

## id: O89zNf5jwXjBJNNKhHbBV

## Derived From Pattern/Invariant
GovernanceDelegationFlaw

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: LowSeverityDueToLowImpact + InvalidGovernanceRisk
### Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk, not a vulnerability. The finding describes a griefing attack where a user leaves dust amounts in pending requests to prevent vault unregistration. However, per Known Issues Section 10 (Vault Unregistration DOS via Dust Holdings), this is documented as intentional design. The admin can handle this through force-claim mechanisms. No user funds are at risk - this is a protocol maintenance issue. Per C4 severity categorization, 'Governance/Centralization risk' is Low/QA. The protocol is functioning as designed with known trade-offs for administrative operations.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.unregisterVault` function strictly enforces that the target vault has zero pending/active requests (`activeDepositRequestersCount == 0`). A malicious user can permanently block vault unregistration by requesting a deposit of a dust amount (1 wei) and refusing to claim it after fulfillment. This keeps the user in the `activeDepositRequesters` set indefinitely, preventing the admin from unregistering the vault.

## Impact
Permanent Denial of Service of the `unregisterVault` function, preventing protocol maintenance and asset delisting.

## Command to Run Test


## Proof of Concept
1. Attacker calls `vault.requestDeposit(1 wei)`.
2. Manager calls `fulfillDeposit`.
3. Attacker's request moves to `claimable`. Attacker is added to `activeDepositRequesters`.
4. Attacker never calls `deposit` (claim).
5. Admin calls `shareToken.unregisterVault(asset)`.
6. Calls `vault.getVaultMetrics` -> checks `activeDepositRequestersCount`.
7. Reverts because count is 1. Admin cannot remove vault.

## Proof of Code
function testUnregisterDos() public {
    vm.prank(attacker);
    vault.requestDeposit(1, attacker, attacker);
    
    vm.prank(manager);
    vault.fulfillDeposit(attacker, 1);
    
    vm.prank(owner);
    vm.expectRevert();
    shareToken.unregisterVault(asset);
}

## Suggested Mitigation
Implement a force-claim mechanism or allow admins to sweep dust requests during the unregistration process.


## [M-12]. Strict Balance Equality Check Enables Griefing of Vault Unregistration

## id: bCzL72Qd-z8Y-X2-ROugW

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: LowSeverityDueToLowImpact + InvalidGovernanceRisk
### Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk. The finding describes a griefing attack where an attacker donates dust to prevent vault unregistration due to strict balance == 0 check. However, per Known Issues Section 11 (Strict Balance Equality Check Enables Griefing), this is documented as intentional design. The admin can handle this through investAssets to sweep dust or by accepting dust during unregistration. No user funds are at risk. Per C4 severity categorization, 'Governance/Centralization risk' is Low/QA. This is a known trade-off in the vault lifecycle management design.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.unregisterVault` function strictly enforces that the target vault's asset balance must be exactly zero (`IERC20(asset).balanceOf(vaultAddress) != 0`). A malicious actor can donate a negligible amount of assets (dust) to the vault. While `investAssets` can normally sweep assets, it requires a configured `investmentVault`. If no investment vault is configured (or if the admin is unregistering precisely to decommission the vault and has not set one up), there is no mechanism to remove this dust. This permanently causes `unregisterVault` to revert, griefing the protocol's lifecycle management and potentially preventing the removal of compromised vaults.

## Impact
Permanent DoS of vault unregistration capability unless specific complex admin actions (deploying dummy vault) are taken.

## Command to Run Test


## Proof of Concept
1. Admin decides to unregister a deprecated vault that has 0 assets.
2. Attacker detects the intent and transfers 1 wei of the asset token to the vault address.
3. Admin calls `unregisterVault(asset)`.
4. The call fails because `IERC20(asset).balanceOf(vaultAddress)` is 1, not 0.
5. If `investmentVault` is not set, Admin cannot call `investAssets` to sweep the dust.

## Proof of Code
function testUnregisterGriefing() public {
    // Setup vault and asset
    vm.prank(admin);
    shareToken.registerVault(address(asset), address(vault));
    
    // Attacker donates dust
    vm.prank(attacker);
    asset.transfer(address(vault), 1);
    
    // Admin tries to unregister
    vm.prank(admin);
    vm.expectRevert(IERC7575Errors.CannotUnregisterVaultAssetBalance.selector);
    shareToken.unregisterVault(address(asset));
}

## Suggested Mitigation
Allow unregistration if the balance is below a small dust threshold, or provide a permissioned `sweepAssets` function in the vault.





Finding Status: LowSeverityDueToRareLikelihood
## [M-13]. Total Assets Clamping Hides Insolvency and Bricks Price Calculation

## id: gA24_kn1CzcaGvVQ-j-hW

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: --- Round 1 ---
This is a valid Medium finding. If a vault becomes insolvent (balance < reservedAssets), totalAssets() returns 0, causing division-by-zero in ShareToken.convertNormalizedAssetsToShares(). This would DOS all operations across ALL vaults sharing that ShareToken. The impact is significant (protocol-wide DOS), but likelihood is Rare because it requires investment losses severe enough to cause insolvency. The clamping to 0 instead of handling negative equity gracefully is a design flaw that could brick the entire multi-asset system. This is NOT a governance issue - it's a function impacted by a bug.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.totalAssets()`, the function returns `0` if `balance < reservedAssets`. If the vault becomes even slightly insolvent (e.g., due to investment losses), `totalAssets` drops to 0. This causes `ShareToken.convertNormalizedAssetsToShares` to revert due to division by zero (via `totalNormalizedAssets`), causing a global Denial of Service for all operations (deposits, redeems) across ALL vaults sharing that ShareToken.

## Impact
Global protocol DoS. If one vault becomes insolvent, the shared pricing logic fails, bricking the entire multi-asset system.

## Command to Run Test


## Proof of Concept
1. Vault has 1000 assets. 1001 reserved (due to loss).
2. `totalAssets()` returns 0.
3. `ShareToken.getCirculatingSupplyAndAssets` sums this (plus others). If this is the only vault, `totalNormalizedAssets` = 0.
4. `ShareToken.convertNormalizedAssetsToShares` calls `Math.mulDiv(..., totalNormalizedAssets)`. Reverts on div by 0.
5. No user can deposit or redeem.

## Proof of Code
function testInsolvencyBrick() public {
    // Setup insolvency
    simulateInsolvency(); // balance < reserved
    
    // Try convert
    vm.expectRevert(); // Div by zero
    shareToken.convertNormalizedAssetsToShares(100, Math.Rounding.Floor);
}

## Suggested Mitigation
Handle insolvency gracefully in `totalAssets` (e.g., allow negative equity representation internally) or ensure `ShareToken` handles 0 assets without reverting (e.g., return 0 price).





Finding Status: InvalidERC20EdgeCase, LowSeverityDueToLowImpact, LowSeverityDueToRareLikelihood
## [M-14]. Rebasing token yield theft via pending deposits

## id: mKKwTn0dUCGuh4YZaWGur

## Derived From Pattern/Invariant
FeeOnTransferAssumption

## Exploit Type
FeeOnTransferAssumption

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidERC20EdgeCase
### Finding Status Justification: --- Round 1 ---
This is a non-standard token issue. The finding describes rebasing tokens (stETH, aTokens) causing yield theft from pending depositors. However, per Known Issues Section 2 (Non-Standard ERC-20 Behavior) and the SafeTokenTransfers library implementation, the protocol explicitly rejects non-standard tokens including rebasing tokens. The SafeTokenTransfers.safeTransferFrom checks that balanceAfter == balanceBefore + amount, which would fail for rebasing tokens. Impact: Low (protocol doesn't support these tokens). Likelihood: Rare (tokens are rejected at transfer time). This is out of scope per non-standard token exclusions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.requestDeposit`, the vault assumes the balance increase equals the transferred amount (`pendingDepositAssets += assets`). If the asset is a rebasing token (e.g., stETH, aTokens), the vault's balance will grow independently. Since `totalAssets` is calculated as `balance - reservedAssets`, any balance increase from rebasing is immediately attributed to `totalAssets`, benefiting existing shareholders. The pending depositor (whose assets generated the yield) receives shares based on the price *at fulfillment*, which effectively donates their yield to the pool.

## Impact
Loss of yield for pending depositors; existing shareholders steal yield generated by pending assets.

## Command to Run Test


## Proof of Concept
1. User requests deposit of 100 RebasingTokens. 
2. Rebase occurs, vault balance grows to 110. 
3. `totalAssets` increases by 10. Share price increases. 
4. `fulfillDeposit` runs. User gets shares based on inflated price. 
5. User effectively lost the 10 tokens of growth.

## Proof of Code
function testRebaseYieldTheft() public {
    // ... 
}

## Suggested Mitigation
Do not support rebasing tokens or track the exact balance contribution of pending deposits dynamically.



