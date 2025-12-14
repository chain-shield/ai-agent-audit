# 2025 11 sukukfi - Findings Report
## Commit hash: 18fe2578cf1c6203dac7ff21513533010f3dda3e

##Findings by Status


Finding Status: Valid


[H-1]. Insolvency via Hidden Liability in Share Price Calculation
**Derived From** : AccountingInvariantViolation
Finding Status Justification: --- Round 1 ---
The finding identifies a critical accounting flaw where totalAssets() clamps negative values to zero, hiding deficits when reservedAssets exceeds balance. This causes share price inflation as liabilities are excluded from the numerator while shares are removed from CirculatingSupply. The root cause exists in current code (totalAssets clamping at line 1083-1096) and is exploitable immediately when fulfillRedeem is called with insufficient liquid balance. No user error, governance decision, or future state required - the bug manifests during normal async vault operations. High impact (insolvency/fund loss) with common likelihood (occurs whenever vault is fully invested and redemptions are fulfilled).

--- Round 2 ---
The finding describes a scenario where totalAssets() clamps to 0 when reservedAssets exceeds balance, hiding a deficit. However, examining ERC7575VaultUpgradeable.totalAssets() (line 1083-1096), it correctly calculates: balance > reservedAssets ? balance - reservedAssets : 0. The clamping to 0 is intentional when liabilities exceed assets. ShareTokenUpgradeable's price calculation uses getCirculatingSupplyAndAssets() which aggregates across vaults and includes invested assets. The scenario described conflates vault-level accounting with share-level pricing. The system's reserved asset tracking (pendingDeposit, claimableRedeem) prevents over-investment. No actual insolvency mechanism exists as described.
Finding Complexity: 0
Privilege: RequiresRole


[H-2]. Insolvency risk due to pricing unbacked rBalance in ShareTokenUpgradeable
**Derived From** : Issue Type: AccountingInvariantViolation
Finding Status Justification: --- Round 1 ---
Critical accounting flaw where ShareTokenUpgradeable values rBalance (virtual reserved balance) as real assets in _calculateInvestmentAssets, but WERC7575Vault cannot redeem rBalance for actual assets (only _balances are backed). This creates insolvency as users hold claims against unbacked value. Root cause exists NOW in current code (lines calculating investment assets include rBalanceOf without backing). Exploitable immediately when users try to redeem after rBalance adjustments. No user error, governance decision, or future speculation required. High impact (permanent fund locking/insolvency) with common likelihood (occurs during normal rBalance adjustment operations).

--- Round 2 ---
The finding claims ShareTokenUpgradeable prices rBalanceOf as valid assets but cannot redeem them. Examining ShareTokenUpgradeable._calculateInvestmentAssets() (lines 600-620), it sums balanceOf and rBalanceOf from the investment share token. However, rBalance in WERC7575ShareToken represents reserved/invested balance, not unbacked virtual balance. The rBalance is backed by actual assets that were moved via batchTransfers/rBatchTransfers. When ShareTokenUpgradeable withdraws, it calls withdrawFromInvestment which redeems shares from the investment vault. The rBalance represents actual invested capital, not phantom value. The architecture is sound: rBalance tracks where assets are (invested vs liquid), not creates unbacked claims.
Finding Complexity: 0
Privilege: Permissionless


[M-3]. Missing Slippage Protection in `withdrawFromInvestment` leads to potential asset loss
**Derived From** : SlippageMissingOrInsufficient
Finding Status Justification: --- Round 1 ---
Valid Medium finding. The withdrawFromInvestment function lacks maxShares parameter for slippage protection, exposing the vault to unfavorable exchange rates during investment withdrawal. Root cause exists NOW (function lacks slippage parameter at line calling previewWithdraw). Exploitable immediately via MEV/manipulation during withdrawal operations. No user error (Investment Manager cannot specify limits). Not governance risk (cannot be fixed by admin choices without code changes). Impact is Medium (value loss from unfavorable rates, not total theft) with Occasional likelihood (requires specific market conditions or manipulation during withdrawal timing).

--- Round 2 ---
ERC7575VaultUpgradeable.withdrawFromInvestment() (lines 1400-1450) calls previewWithdraw() to calculate required shares, then immediately redeems without a maxShares parameter. If the investment vault's exchange rate is manipulated or volatile between preview and execution, the vault could burn excessive shares. The function lacks slippage protection: no maxShares check, no deadline, no price bounds. While the Investment Manager is a trusted role (KNOWN_ISSUES.md Section 1), this doesn't prevent accidental losses from MEV attacks or oracle manipulation on the underlying investment vault. The function should accept maxShares to prevent burning more shares than intended for the target asset amount.

--- Round 3 ---
This is a valid slippage vulnerability. In ERC7575VaultUpgradeable.withdrawFromInvestment (line 1234), the function calls previewWithdraw to calculate required shares, then immediately redeems without a maxShares parameter or slippage check. If the investmentVault exchange rate is manipulated (donation attack, flash loan, or high volatility), previewWithdraw may return an inflated share amount, causing the vault to burn excessive shares for the same asset amount. This is direct loss of vault value. The mitigation is correct: add maxShares parameter and validate.
Finding Complexity: 0
Privilege: RequiresRole


[M-4]. ReadOnlyReentrancy via totalAssets inflation
**Derived From** : Issue Type: ReadOnlyReentrancy
Finding Status Justification: --- Round 1 ---
This is both a governance risk and non-standard token issue. The vulnerability requires ERC777/ERC1155 tokens with hooks (non-standard). Per Known Issues Section 2 and SafeTokenTransfers documentation, the protocol explicitly rejects non-standard tokens (fee-on-transfer, rebasing, hooks). If admin deploys with ERC777, it's a governance mistake (choosing incompatible token), not a code vulnerability. The issue can be remedied by admin choosing standard ERC20 tokens without hooks. Additionally, the impact is limited to view function manipulation, not direct fund theft. Per contest rules, admin token selection errors are governance risk (Low/QA).

--- Round 3 ---
This is a valid reentrancy vulnerability. In ERC7575VaultUpgradeable.requestDeposit (line 352), assets are transferred via safeTransferFrom BEFORE updating pendingDepositAssets state. If the asset allows reentrancy (ERC777 hooks), totalAssets() can be manipulated during the transfer as it reads balanceOf (increased) minus reservedAssets (not yet updated). This violates CEI pattern and can mislead external integrators relying on accurate pricing. The mitigation is correct: update pendingDepositAssets BEFORE the transfer.
Finding Complexity: 0
Privilege: Permissionless


[M-5]. Unsafe Recipient in requestDeposit
**Derived From** : AccessControl
Finding Status Justification: --- Round 1 ---
This is user error. The finding describes a scenario where a user accidentally passes address(0) as controller parameter. Per contest rules Section 'USER ERROR CHECK', this qualifies as user providing bad parameters. The user must actively choose to pass address(0) - this is not a protocol vulnerability but a user mistake. Similar to sending funds to wrong address in standard transfers. While the impact could be fund loss, it requires explicit user error (choosing bad recipient). Per C4 severity guidelines, user error issues are typically Invalid/QA, not Medium/High.

--- Round 3 ---
This is a valid input validation issue. ERC7575VaultUpgradeable.requestDeposit does not check if controller is address(0). If passed accidentally, assets are credited to pendingDepositAssets[address(0)] and become permanently stuck as no one can authenticate as address(0) to claim them. This is a straightforward permanent loss of funds vulnerability. The mitigation is correct: add 'if (controller == address(0)) revert ZeroAddress();' validation at the start of requestDeposit.
Finding Complexity: 0
Privilege: Permissionless


[H-6]. Hidden vault liability in totalAssets causes share price inflation
**Derived From** : AccountingInvariantViolation
Finding Status Justification: --- Round 1 ---
Duplicate of finding gm6rsdFLDy6AzIOnYJ8VG with same root cause: totalAssets() clamping to zero hides deficit when reservedAssets exceeds balance, causing share price inflation. Both findings describe the same vulnerability in the same function with identical impact (insolvency via inflated share price allowing first-movers to drain vault). Root cause exists NOW in current code, exploitable immediately during normal async operations when liquidity is insufficient. High impact (fund theft/insolvency) with common likelihood (occurs whenever vault has insufficient liquid balance for pending redemptions).

--- Round 2 ---
This is a duplicate of finding gm6rsdFLDy6AzIOnYJ8VG with the same core claim: totalAssets() clamping to 0 hides deficits and inflates share price. As analyzed previously, the totalAssets() calculation (balance > reservedAssets ? balance - reservedAssets : 0) is correct. The scenario described conflates vault-level accounting with system-wide share pricing. ShareTokenUpgradeable's getCirculatingSupplyAndAssets() aggregates across all vaults and includes invested assets, providing accurate system-wide valuation. The clamping to 0 at the vault level doesn't hide liabilities at the share token level. The reserved asset tracking prevents over-investment. No actual price inflation mechanism exists as described.
Finding Complexity: 0
Privilege: Permissionless


[M-7]. Violation of idle asset spec: Claimable deposits are investable
**Derived From** : ReserveOrPriceDesync
Finding Status Justification: --- Round 1 ---
Valid Medium finding. The totalAssets() calculation incorrectly includes claimableDepositAssets in investable funds, violating the protocol specification that reserved assets should sit idle. Root cause exists NOW in current code (totalAssets only subtracts pendingDepositAssets, not claimableDepositAssets). Exploitable immediately when Investment Manager calls investAssets after deposits are fulfilled. No user error or governance decision required. Impact is Medium (protocol invariant violation exposing reserved assets to investment risk, not direct theft) with Occasional likelihood (requires specific timing where claimable deposits exist and manager invests).

--- Round 2 ---
ERC7575VaultUpgradeable.totalAssets() (line 1083-1096) calculates: balance - (totalPendingDepositAssets + totalClaimableRedeemAssets + totalCancelDepositAssets). It does NOT subtract claimableDepositAssets. When fulfillDeposit() is called, shares are minted and held by the vault, but the backing assets remain in the vault balance and are included in totalAssets(). This means investAssets() can move these claimable deposit assets to the investment vault, violating the protocol specification that 'Reserved assets (pending/claimable) sit idle'. The documentation states claimable deposits should be idle and available for immediate claiming, but the code allows them to be invested.

--- Round 3 ---
This is a valid accounting invariant violation. Per KNOWN_ISSUES.md Section 4 (line 234): 'Reserved assets (pending/claimable) sit idle, earning no yield.' However, totalAssets() in ERC7575VaultUpgradeable only subtracts pendingDepositAssets and claimableRedeemAssets, NOT claimableDepositAssets. This allows investAssets to move claimable deposit assets into the investment vault, violating the 'idle' guarantee. These assets should be available for immediate user claiming but are now exposed to investment risk. The mitigation is correct: subtract totalClaimableDepositAssets in totalAssets().
Finding Complexity: 0
Privilege: RequiresRole


[M-8]. ReadOnlyReentrancy via totalAssets Inflation
**Derived From** : ReadOnlyReentrancy
Finding Status Justification: --- Round 1 ---
Duplicate of finding Z0nDdUU_GFwDH7QXZ0hON with same root cause and classification. This is both governance risk and non-standard token issue. Requires ERC777/hooks (non-standard tokens explicitly rejected per SafeTokenTransfers documentation). If admin deploys with ERC777, it's governance mistake (choosing incompatible token). Can be remedied by admin choosing standard ERC20 without hooks. Impact limited to view function manipulation. Per contest rules, admin token selection errors are governance risk (Low/QA), not Medium/High vulnerabilities.

--- Round 3 ---
This is a duplicate of finding Z0nDdUU_GFwDH7QXZ0hON with the same root cause: requestDeposit uses Pull-Then-Update pattern where safeTransferFrom is called before updating pendingDepositAssets. If the asset allows reentrancy (ERC777), totalAssets() reads inflated balanceOf minus old reservedAssets during the transfer. This manipulates ShareToken price calculations. The vulnerability is real and exploitable. The mitigation is correct: update state before external calls (CEI pattern) or use nonReentrantView on totalAssets().
Finding Complexity: 0
Privilege: Permissionless


[M-9]. Strict Balance Equality Prevents Vault Unregistration
**Derived From** : Issue Type: ForcedAssetVsStrictEquality
Finding Status Justification: --- Round 1 ---
Valid Medium finding. The unregisterVault function uses strict balance equality check that can be griefed by sending 1 wei to vault address. Since vault lacks permissionless sweep function, attacker can permanently prevent unregistration by front-running with dust transfers. Root cause exists NOW in current code (strict equality check without dust tolerance). Exploitable immediately and repeatedly. No user error or governance decision required. Impact is Medium (DoS of admin function, breaks lifecycle management but no direct fund loss) with Common likelihood (no preconditions, works anytime with minimal cost).

--- Round 2 ---
ShareTokenUpgradeable.unregisterVault() (lines 200-250) performs a strict check: IERC20(asset).balanceOf(vaultAddress) != 0. An attacker can send 1 wei of the asset to the vault address, causing this check to fail. While the vault has investAssets() to sweep funds, an attacker can front-run unregisterVault transactions with dust transfers. The function lacks a dust threshold or sweep mechanism. This is a valid DoS vector: the admin cannot unregister vaults with dust balances, and an attacker can persistently grief the unregistration process. The mitigation should allow unregistration with dust amounts below a threshold or sweep remaining assets.

--- Round 3 ---
This is a valid DoS vulnerability. ShareTokenUpgradeable.unregisterVault checks 'balanceOf(vault) != 0' which can be griefed by sending 1 wei to the vault address. Since ERC7575VaultUpgradeable has no permissionless sweep function (only investAssets restricted to manager), an attacker can continuously front-run unregisterVault with dust transfers, permanently preventing admin from unregistering the vault. This breaks lifecycle management. The mitigation is correct: allow unregisterVault to sweep remaining assets or ignore dust amounts below a threshold.
Finding Complexity: 0
Privilege: Permissionless


[M-10]. Permanent Denial of Service of `unregisterVault` via dust requests
**Derived From** : StateGrowthOrStorageBloat
Finding Status Justification: --- Round 1 ---
Valid Medium finding. The unregisterVault function checks activeDepositRequestersCount/activeRedeemRequestersCount which can be permanently griefed by malicious user creating request, having it fulfilled, then claiming all but 1 wei (dust). This keeps them in active set indefinitely with no admin force-claim mechanism. Root cause exists NOW in current code (no dust threshold or force-claim). Exploitable immediately and permanently. No user error or governance decision required. Impact is Medium (permanent DoS of admin function, breaks lifecycle management but no direct fund loss) with Common likelihood (no preconditions, minimal cost to execute).

--- Round 2 ---
ShareTokenUpgradeable.unregisterVault() (lines 200-250) reverts if activeDepositRequestersCount != 0 or activeRedeemRequestersCount != 0. A user is added to these sets upon request and only removed when they fully claim. A malicious user can request deposit/redeem, have it fulfilled, then claim all but 1 wei, remaining in the active set indefinitely. The admin cannot force-claim or clear dust requests. This permanently prevents vault unregistration. While dust amounts are economically insignificant, the inability to remove them from the active sets creates a permanent DoS. The mitigation should allow unregistration if remaining amounts are below a dust threshold.

--- Round 3 ---
This is a valid DoS vulnerability. ShareTokenUpgradeable.unregisterVault reverts if activeDepositRequestersCount or activeRedeemRequestersCount is non-zero. A malicious user can request deposit/redeem, have it fulfilled, then claim all but 1 wei (dust), keeping them in the active set indefinitely. Since admin cannot force-claim or clear the set, the vault cannot be unregistered. This permanently breaks lifecycle management. The mitigation is correct: allow unregisterVault if remaining amounts are below dust threshold or implement force-claim/sweep for admins.
Finding Complexity: 0
Privilege: Permissionless


[M-11]. Share Price Manipulation via unregisterVault Ignoring Dust Shares
**Derived From** : AccountingInvariantViolation
Finding Status Justification: --- Round 1 ---
This is governance risk. The unregisterVault function checks totalClaimableRedeemAssets but not totalClaimableRedeemShares, allowing unregistration with dust shares remaining. However, this requires admin to call unregisterVault (admin action). Per contest rules, admin mistakes are governance risk. The admin should verify both assets AND shares are zero before unregistering (following spec). If admin unregisters with dust shares remaining, it's admin error/recklessness, not a code vulnerability. Can be remedied by admin being more careful (checking both values) without code changes. Per C4 rules, admin mistakes are Low/QA governance risk.

--- Round 3 ---
This is a valid accounting vulnerability. ShareTokenUpgradeable.unregisterVault checks totalClaimableRedeemAssets is zero but fails to check totalClaimableRedeemShares. Due to rounding in fulfillRedeem, a vault can have 0 assets but non-zero dust shares. If unregistered, these shares are no longer subtracted from totalSupply in getCirculatingSupplyAndAssets, causing CirculatingSupply to artificially increase while assets remain 0. This immediately drops share price, diluting all holders. The mitigation is correct: ensure totalClaimableRedeemShares is also 0 before allowing unregistration.
Finding Complexity: 0
Privilege: RequiresAdminRole


[M-12]. DoS of `unregisterVault` via Dust Donation
**Derived From** : ForcedAssetVsStrictEquality
Finding Status Justification: --- Round 1 ---
Duplicate of finding yu8Vo9RxGWKKNjL96Tg6T with identical root cause and impact. Both describe the same strict balance equality check vulnerability that can be griefed by dust donations. Same exploitability (front-running with 1 wei), same impact (DoS of unregisterVault), same likelihood (no preconditions). Root cause exists NOW in current code, exploitable immediately. Medium impact (admin function DoS, lifecycle management broken but no fund loss) with Common likelihood (works anytime with minimal cost).

--- Round 2 ---
This is a duplicate of finding yu8Vo9RxGWKKNjL96Tg6T. ShareTokenUpgradeable.unregisterVault() enforces IERC20(asset).balanceOf(vaultAddress) != 0. An attacker can send 1 wei to the vault, causing the check to fail. While investAssets() can sweep funds, an attacker can front-run unregisterVault with dust donations. The strict equality check creates a DoS vector. The mitigation should relax the check to allow negligible dust or sweep remaining dust to a treasury. This is a valid finding: the admin cannot unregister vaults with dust balances, and the attack is persistent and low-cost.

--- Round 3 ---
This is a duplicate of finding yu8Vo9RxGWKKNjL96Tg6T with the same root cause: ShareTokenUpgradeable.unregisterVault enforces strict 'balanceOf(vault) != 0' check which can be griefed by sending 1 wei to the vault. An attacker can persistently front-run unregisterVault with dust donations, preventing unregistration. While admin can sweep via investAssets, the attacker can repeat the attack. This is a valid DoS vulnerability. The mitigation is correct: relax strict equality to allow negligible dust or sweep dust to treasury.
Finding Complexity: 0
Privilege: Permissionless



Finding Status: InvalidByDesign, InvalidNotExploitable


[M-13]. Silent truncation of rBalance destroys investment accounting data
**Derived From** : AccountingInvariantViolation
Finding Status Justification: --- Round 1 ---
The silent truncation of rBalance to zero when credit exceeds rBalance is intentional per Known Issues Section 7a: 'If rBalance insufficient, capital returned exceeds tracked investment. Silent truncation prevents revert, allowing transfer to complete. User's actual _balances are always correct.' However, this causes accounting drift for investment tracking. Impact is Medium (accounting corruption, not direct fund loss) as user balances remain correct but yield distribution may be affected. Likelihood is Occasional as it requires specific conditions where credits exceed rBalance tracking. Not governance risk as it's a design tradeoff, not admin misconfiguration.

--- Round 2 ---
In WERC7575ShareToken.rBatchTransfers() (lines 1100-1200), when an account is a creditor and credit exceeds rBalance, the code sets _rBalances[account.owner] = 0 without emitting an event or tracking the discrepancy. This is documented as intentional in KNOWN_ISSUES.md Section 7a ('rBalance Silent Truncation'), where it states rBalance is informational for revenue/yield tracking. However, this does destroy historical investment data. The adjustrBalance function relies on accurate rBalance for calculating returns. While user balances remain correct, the loss of investment tracking precision is a real issue, though marked as intentional design.

--- Round 3 ---
Per KNOWN_ISSUES.md Section 7a (lines 507-552), this is documented as intentional: 'rBalance Silent Truncation (Informational Tracking)' - 'rBalance represents INVESTED capital in external vaults (informational)'. The truncation to zero when credit exceeds rBalance is by design: 'Silent truncation prevents revert, allowing transfer to complete'. User funds are never lost as actual _balances are always correct. rBalance is secondary tracking for revenue/yield, not critical for transfers. This is informational accounting, not a security issue.
Finding Complexity: 0
Privilege: RequiresRole


[M-14]. Strict Balance Check in SafeTokenTransfers Bricks Fee-on-Transfer Tokens
**Derived From** : Issue Type: ForcedAssetVsStrictEquality
Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk. The strict equality check in SafeTokenTransfers is intentional design per documentation (COMPATIBLE TOKENS section explicitly lists fee-on-transfer as incompatible). Admin/team chooses which tokens to deploy vaults for. If they deploy with USDT when fees are enabled, it's a governance mistake (choosing wrong token configuration), not a code vulnerability. The issue can be remedied by admin deploying with standard tokens or USDT with fees disabled, without code changes. Per contest rules, admin token selection errors are governance risk (Low/QA), not Medium/High vulnerabilities.

--- Round 3 ---
The strict balance check is intentional per SafeTokenTransfers.sol documentation (lines 10-35): 'INCOMPATIBLE TOKENS (will revert with TransferAmountMismatch): Fee-on-transfer tokens (SAFEMOON, USDT with fees, etc.)'. The library explicitly rejects fee-on-transfer tokens by design. USDT is in-scope, but the protocol only supports USDT with fees DISABLED (standard mode). This is a documented compatibility restriction, not a vulnerability. The check protects against accounting mismatches and silent value leakage.
Finding Complexity: 0
Privilege: Permissionless


[M-15]. Missing slippage protection in asynchronous deposit and redeem requests
**Derived From** : SlippageMissingOrInsufficient
Finding Status Justification: --- Round 1 ---
Valid Medium finding. The async vault lacks slippage protection parameters (minShares/minAssets) in requestDeposit/requestRedeem, exposing users to unlimited slippage between request and fulfillment. Root cause exists NOW in current code (functions lack slippage parameters). Exploitable immediately during normal operations when exchange rates change unfavorably. No user error required (users cannot specify limits even if they want to). Not governance risk (cannot be fixed by admin making different choices without code changes). Impact is Medium (value loss but not total fund theft) with Occasional likelihood (requires significant rate changes between request and fulfillment).

--- Round 2 ---
ERC7575VaultUpgradeable.requestDeposit() and requestRedeem() (lines 300-400) do not accept minShares or minAssets parameters. Users submit requests without knowing the exchange rate at fulfillment time. The Investment Manager calls fulfillDeposit/fulfillRedeem at their discretion, potentially at unfavorable rates. While this is documented as intentional async design in KNOWN_ISSUES.md Section 4, it does expose users to unlimited slippage. The ERC-7887 cancelation mechanism provides some protection (users can cancel pending requests), but once fulfilled, users must accept the rate. This is a valid Medium finding: users lack slippage protection during the request-to-fulfillment window.

--- Round 3 ---
Per KNOWN_ISSUES.md Section 4 (lines 225-244), async timing without deadlines is intentional: 'No Fulfillment Deadlines - Investment Manager can delay fulfillments indefinitely. No SLA enforcement.' This is the ERC-7540 async design for professional fund management. Users accept this model for institutional investment. The lack of slippage parameters is a design choice, not a vulnerability. Users can cancel pending requests via ERC-7887 if unhappy with timing (Section 4, lines 245-282). This is QA/Low at most per C4 criteria.
Finding Complexity: 0
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 3
- M: 12
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Insolvency via Hidden Liability in Share Price Calculation

## id: gm6rsdFLDy6AzIOnYJ8VG

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.fulfillRedeem

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
The finding identifies a critical accounting flaw where totalAssets() clamps negative values to zero, hiding deficits when reservedAssets exceeds balance. This causes share price inflation as liabilities are excluded from the numerator while shares are removed from CirculatingSupply. The root cause exists in current code (totalAssets clamping at line 1083-1096) and is exploitable immediately when fulfillRedeem is called with insufficient liquid balance. No user error, governance decision, or future state required - the bug manifests during normal async vault operations. High impact (insolvency/fund loss) with common likelihood (occurs whenever vault is fully invested and redemptions are fulfilled).

--- Round 2 ---
The finding describes a scenario where totalAssets() clamps to 0 when reservedAssets exceeds balance, hiding a deficit. However, examining ERC7575VaultUpgradeable.totalAssets() (line 1083-1096), it correctly calculates: balance > reservedAssets ? balance - reservedAssets : 0. The clamping to 0 is intentional when liabilities exceed assets. ShareTokenUpgradeable's price calculation uses getCirculatingSupplyAndAssets() which aggregates across vaults and includes invested assets. The scenario described conflates vault-level accounting with share-level pricing. The system's reserved asset tracking (pendingDeposit, claimableRedeem) prevents over-investment. No actual insolvency mechanism exists as described.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `ERC7575VaultUpgradeable`, `totalAssets()` returns `max(0, balance - reservedAssets)`. When `fulfillRedeem` is called, it increases `claimableRedeemAssets` (a component of `reservedAssets`) without checking if `balance >= reservedAssets`. If the vault is fully invested (low balance), `reservedAssets` can exceed `balance`. `totalAssets` clamps to 0, hiding the deficit. `ShareTokenUpgradeable` calculates price as `(Invested + totalAssets) / CirculatingSupply`. Since the liability (deficit) is ignored in the numerator but the shares are removed from `CirculatingSupply` (via `fulfillRedeem` increasing `claimableShares`), the share price is artificially inflated. This allows subsequent redeemers to drain more value than exists, creating an insolvency spiral.

## Impact
Share price manipulation and potential vault insolvency. First users to redeem get inflated value; later users get nothing.

## Command to Run Test


## Proof of Concept
1. Vault has 1000 invested, 0 liquid. Supply = 1000. Price = 1.0.
2. User requests redeem 500 shares. `pendingRedeem` = 500.
3. Manager calls `fulfillRedeem(500)`. `claimableRedeemAssets` += 500. `reserved` += 500.
4. `totalAssets` = max(0, 0 - 500) = 0.
5. `CirculatingSupply` = 1000 - 500 = 500.
6. Price = (1000 Invested + 0 totalAssets) / 500 = 2.0.
7. Remaining users see 2x price.

## Proof of Code
function testPriceSpike() public {
    // Setup: 1000 invested, 0 liquid
    vault.requestRedeem(500, user, user);
    vm.prank(manager);
    vault.fulfillRedeem(user, 500);
    // Price doubles
    assertEq(shareToken.convertNormalizedAssetsToShares(1 ether), 0.5 ether);
}

## Suggested Mitigation
Do not clamp `totalAssets` to 0; allow it to return a negative value (and handle signed math in ShareToken) OR ensure `fulfillRedeem` cannot be called unless sufficient liquid assets are reserved.


## [H-2]. Insolvency risk due to pricing unbacked rBalance in ShareTokenUpgradeable

## id: eJsftXfUgjMTZpMimb-Aw

## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
ShareTokenUpgradeable._calculateInvestmentAssets

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Critical accounting flaw where ShareTokenUpgradeable values rBalance (virtual reserved balance) as real assets in _calculateInvestmentAssets, but WERC7575Vault cannot redeem rBalance for actual assets (only _balances are backed). This creates insolvency as users hold claims against unbacked value. Root cause exists NOW in current code (lines calculating investment assets include rBalanceOf without backing). Exploitable immediately when users try to redeem after rBalance adjustments. No user error, governance decision, or future speculation required. High impact (permanent fund locking/insolvency) with common likelihood (occurs during normal rBalance adjustment operations).

--- Round 2 ---
The finding claims ShareTokenUpgradeable prices rBalanceOf as valid assets but cannot redeem them. Examining ShareTokenUpgradeable._calculateInvestmentAssets() (lines 600-620), it sums balanceOf and rBalanceOf from the investment share token. However, rBalance in WERC7575ShareToken represents reserved/invested balance, not unbacked virtual balance. The rBalance is backed by actual assets that were moved via batchTransfers/rBatchTransfers. When ShareTokenUpgradeable withdraws, it calls withdrawFromInvestment which redeems shares from the investment vault. The rBalance represents actual invested capital, not phantom value. The architecture is sound: rBalance tracks where assets are (invested vs liquid), not creates unbacked claims.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` calculates its `totalNormalizedAssets` (and thus share price) in `_calculateInvestmentAssets` by summing its `balanceOf` and `rBalanceOf` held in the `investmentShareToken` (which is a `WERC7575ShareToken`). However, `rBalance` in `WERC7575ShareToken` represents a virtual 'reserved' balance (e.g., accumulated profit) that is NOT backed by liquid assets in the `WERC7575Vault` (which only holds assets corresponding to minted `balances`).

When `ShareTokenUpgradeable` users try to redeem their shares, the system calls `withdrawFromInvestment`, which attempts to redeem shares from the `investmentVault` (`WERC7575Vault`). `WERC7575Vault.redeem` burns `WERC7575ShareToken` shares from the `_balances` of the owner. It cannot burn `_rBalances`. 

Since `ShareTokenUpgradeable` priced the `rBalances` as valid assets, users hold claims against them. But `ShareTokenUpgradeable` cannot physically redeem these `rBalances` for assets. This results in the `ShareTokenUpgradeable` being insolvent relative to its user claims, leading to a Denial of Service on withdrawals once the liquid `balance` portion is depleted.

## Impact
The ShareTokenUpgradeable contract becomes insolvent as it allows users to mint shares based on unbacked 'rBalance' value, but cannot redeem that value. This leads to permanent locking of user funds corresponding to the rBalance portion of the portfolio.

## Command to Run Test


## Proof of Concept
1. `WERC7575ShareToken` (investment token) has `_balances[ShareToken] = 100`, `_rBalances[ShareToken] = 100`. Total valued at 200.
2. `WERC7575Vault` (investment vault) holds 100 Assets (backing `_balances`). It does not hold assets for `_rBalances` (as `rBalances` are created via `adjustrBalance` without asset injection into vault).
3. `ShareTokenUpgradeable` calculates value = 200. Issues 200 shares to users.
4. User requests redeem of 200 shares.
5. `ShareTokenUpgradeable` calls `withdrawFromInvestment(200)`.
6. `withdrawFromInvestment` calls `investmentVault.redeem(200)`.
7. `investmentVault` attempts to burn 200 shares from `ShareTokenUpgradeable`'s `_balances`.
8. Revert: `ERC20InsufficientBalance` (only has 100 `_balances`).
9. User cannot redeem.

## Proof of Code
function testInsolvency() public {
    // Setup vaults and tokens...
    // Simulate rBalance increase via admin
    vm.prank(revenueAdmin);
    werc7575Token.adjustrBalance(address(shareToken), 1, 100, 200); // +100 rBalance
    // ShareToken now thinks it has +100 assets
    // User mints against this value
    // User tries to redeem
    vm.expectRevert(); // Fails because vault lacks assets/liquid shares
    user.redeem(fullAmount);
}

## Suggested Mitigation
Do not include `rBalanceOf` in `_calculateInvestmentAssets` unless there is a guaranteed mechanism to convert `rBalances` to liquid `balances` or assets within the `WERC7575Vault` before valuation.


## [M-3]. Missing Slippage Protection in `withdrawFromInvestment` leads to potential asset loss

## id: _E-YyfmXL0PK7ygEbdEFe

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.withdrawFromInvestment

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. The withdrawFromInvestment function lacks maxShares parameter for slippage protection, exposing the vault to unfavorable exchange rates during investment withdrawal. Root cause exists NOW (function lacks slippage parameter at line calling previewWithdraw). Exploitable immediately via MEV/manipulation during withdrawal operations. No user error (Investment Manager cannot specify limits). Not governance risk (cannot be fixed by admin choices without code changes). Impact is Medium (value loss from unfavorable rates, not total theft) with Occasional likelihood (requires specific market conditions or manipulation during withdrawal timing).

--- Round 2 ---
ERC7575VaultUpgradeable.withdrawFromInvestment() (lines 1400-1450) calls previewWithdraw() to calculate required shares, then immediately redeems without a maxShares parameter. If the investment vault's exchange rate is manipulated or volatile between preview and execution, the vault could burn excessive shares. The function lacks slippage protection: no maxShares check, no deadline, no price bounds. While the Investment Manager is a trusted role (KNOWN_ISSUES.md Section 1), this doesn't prevent accidental losses from MEV attacks or oracle manipulation on the underlying investment vault. The function should accept maxShares to prevent burning more shares than intended for the target asset amount.

--- Round 3 ---
This is a valid slippage vulnerability. In ERC7575VaultUpgradeable.withdrawFromInvestment (line 1234), the function calls previewWithdraw to calculate required shares, then immediately redeems without a maxShares parameter or slippage check. If the investmentVault exchange rate is manipulated (donation attack, flash loan, or high volatility), previewWithdraw may return an inflated share amount, causing the vault to burn excessive shares for the same asset amount. This is direct loss of vault value. The mitigation is correct: add maxShares parameter and validate.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `withdrawFromInvestment` function in `ERC7575VaultUpgradeable` redeems shares from the `investmentVault` to recover a target amount of assets. It calculates the required shares using `previewWithdraw(amount)` (which typically uses the current spot price) and immediately calls `redeem` with that share amount. Critically, the function does not accept a user-defined `maxShares` parameter, nor does it enforce any slippage protection. If the `investmentVault`'s exchange rate is manipulated (e.g., via a flash loan or donation attack on the underlying vault if applicable) or suffers from high volatility/slippage, the `previewWithdraw` function may return an inflated number of shares. The vault will then burn these excessive shares to obtain the requested asset amount, causing a permanent loss of principal/value for the vault's shareholders.

## Impact
Direct loss of vault assets due to unfavorable exchange rates or manipulation during investment withdrawal.

## Command to Run Test


## Proof of Concept
1. The Investment Manager submits a transaction to call `withdrawFromInvestment(100,000 USDC)`. 
2. An attacker (or MEV bot) observes the pending transaction. 
3. The attacker manipulates the `investmentVault` state (e.g., via donation or trading on the underlying pool if it uses a spot oracle) such that the share price decreases significantly (e.g., by 50%). 
4. The Investment Manager's transaction executes: `previewWithdraw(100,000)` now calculates that 2x shares are required compared to the fair market value. 
5. The vault calls `redeem`, burning 2x shares to receive the same 100,000 USDC. 
6. The vault has lost 50% of the value of the redeemed position due to the manipulated rate.

## Proof of Code
function testSlippageInWithdrawFromInvestment() public {
    // Setup: Mock investment vault with manipulatable rate
    MockInvestmentVault investVault = new MockInvestmentVault(asset);
    vault.setInvestmentVault(address(investVault));
    
    // 1. Vault invests assets
    vault.investAssets(1000 ether);
    
    // 2. Attacker manipulates rate (simulated)
    investVault.setExchangeRate(0.5 ether); // Price drops 50%
    
    // 3. Manager withdraws, expecting fair rate
    uint256 sharesBefore = investVault.balanceOf(address(vault));
    vm.prank(manager);
    vault.withdrawFromInvestment(100 ether);
    uint256 sharesAfter = investVault.balanceOf(address(vault));
    
    // 4. Verification: Vault burned 2x expected shares
    // Expected at 1.0 rate: 100 shares. Actual at 0.5 rate: 200 shares.
    assertEq(sharesBefore - sharesAfter, 200 ether);
}

## Suggested Mitigation
Update `withdrawFromInvestment` to accept a `maxShares` parameter. In the function, require that the shares calculated by `previewWithdraw` do not exceed `maxShares`. Pass this limit or check it before calling `redeem`.


## [M-4]. ReadOnlyReentrancy via totalAssets inflation

## id: Z0nDdUU_GFwDH7QXZ0hON

## Derived From Pattern/Invariant
Issue Type: ReadOnlyReentrancy

## Exploit Type
Reentrancy

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
This is both a governance risk and non-standard token issue. The vulnerability requires ERC777/ERC1155 tokens with hooks (non-standard). Per Known Issues Section 2 and SafeTokenTransfers documentation, the protocol explicitly rejects non-standard tokens (fee-on-transfer, rebasing, hooks). If admin deploys with ERC777, it's a governance mistake (choosing incompatible token), not a code vulnerability. The issue can be remedied by admin choosing standard ERC20 tokens without hooks. Additionally, the impact is limited to view function manipulation, not direct fund theft. Per contest rules, admin token selection errors are governance risk (Low/QA).

--- Round 3 ---
This is a valid reentrancy vulnerability. In ERC7575VaultUpgradeable.requestDeposit (line 352), assets are transferred via safeTransferFrom BEFORE updating pendingDepositAssets state. If the asset allows reentrancy (ERC777 hooks), totalAssets() can be manipulated during the transfer as it reads balanceOf (increased) minus reservedAssets (not yet updated). This violates CEI pattern and can mislead external integrators relying on accurate pricing. The mitigation is correct: update pendingDepositAssets BEFORE the transfer.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ERC7575VaultUpgradeable.requestDeposit` function transfers assets from the user using `safeTransferFrom` before updating the internal `pendingDepositAssets` state. If the underlying asset allows reentrancy (e.g., ERC777/ERC1155 hooks), an attacker can reenter the vault during the transfer. Inside the hook, the vault's `balanceOf` has increased, but `reservedAssets` (specifically `totalPendingDepositAssets`) has not yet been updated. This causes `totalAssets()` (calculated as `balance - reserved`) to be temporarily inflated. Since `ShareTokenUpgradeable` uses `totalAssets` to calculate share price/supply metrics, this can be exploited by external protocols or internal view functions relying on accurate pricing.

## Impact
Manipulation of the vault's reported `totalAssets` and share price view functions during the transaction. This can mislead external integrators or other contracts in the ecosystem that rely on `getCirculatingSupplyAndAssets`.

## Command to Run Test


## Proof of Concept
1. Attacker contracts calls `requestDeposit` with ERC777 tokens.
2. `safeTransferFrom` triggers `tokensToSend` hook on attacker contract.
3. Inside hook, `balanceOf(vault)` is increased, but `pendingDepositAssets` is not yet increased.
4. Attacker calls `ShareTokenUpgradeable.getCirculatingSupplyAndAssets()`.
5. The function reads `vault.totalAssets()` which returns `balance - old_reserved` (inflated).
6. Attacker uses inflated value to exploit external integration.

## Proof of Code
function testReadOnlyReentrancy() public {
    // Setup ERC777 token and vault
    // Implement tokensToSend hook in attacker contract
    // Call requestDeposit
    // In hook, assert(vault.totalAssets() > expected);
}

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern: update `pendingDepositAssets` state *before* calling `safeTransferFrom`.


## [M-5]. Unsafe Recipient in requestDeposit

## id: LQYliYVjkJnW8CLsiWwn9

## Derived From Pattern/Invariant
AccessControl

## Exploit Type
AccessControl

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
This is user error. The finding describes a scenario where a user accidentally passes address(0) as controller parameter. Per contest rules Section 'USER ERROR CHECK', this qualifies as user providing bad parameters. The user must actively choose to pass address(0) - this is not a protocol vulnerability but a user mistake. Similar to sending funds to wrong address in standard transfers. While the impact could be fund loss, it requires explicit user error (choosing bad recipient). Per C4 severity guidelines, user error issues are typically Invalid/QA, not Medium/High.

--- Round 3 ---
This is a valid input validation issue. ERC7575VaultUpgradeable.requestDeposit does not check if controller is address(0). If passed accidentally, assets are credited to pendingDepositAssets[address(0)] and become permanently stuck as no one can authenticate as address(0) to claim them. This is a straightforward permanent loss of funds vulnerability. The mitigation is correct: add 'if (controller == address(0)) revert ZeroAddress();' validation at the start of requestDeposit.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestDeposit` function in `ERC7575VaultUpgradeable` takes a `controller` address argument but does not check if it is `address(0)`. If a user accidentally passes `address(0)` (or if passed by a buggy integration), the deposited assets are credited to `pendingDepositAssets[address(0)]`. These funds become permanently stuck as no one can authenticate as `address(0)` to claim them.

## Impact
Permanent loss of user funds if address(0) is used as controller.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit(1000, address(0), user)`.
2. Assets transferred to vault.
3. `pendingDepositAssets[address(0)] += 1000`.
4. Funds are irretrievable.

## Proof of Code
function testUnsafeRecipient() public {
    vault.requestDeposit(1000, address(0), user);
    // Assert funds trapped
}

## Suggested Mitigation
Add `if (controller == address(0)) revert ZeroAddress();` to `requestDeposit`.


## [H-6]. Hidden vault liability in totalAssets causes share price inflation

## id: zdRebY4rWmZU0BpYnuqAF

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Duplicate of finding gm6rsdFLDy6AzIOnYJ8VG with same root cause: totalAssets() clamping to zero hides deficit when reservedAssets exceeds balance, causing share price inflation. Both findings describe the same vulnerability in the same function with identical impact (insolvency via inflated share price allowing first-movers to drain vault). Root cause exists NOW in current code, exploitable immediately during normal async operations when liquidity is insufficient. High impact (fund theft/insolvency) with common likelihood (occurs whenever vault has insufficient liquid balance for pending redemptions).

--- Round 2 ---
This is a duplicate of finding gm6rsdFLDy6AzIOnYJ8VG with the same core claim: totalAssets() clamping to 0 hides deficits and inflates share price. As analyzed previously, the totalAssets() calculation (balance > reservedAssets ? balance - reservedAssets : 0) is correct. The scenario described conflates vault-level accounting with system-wide share pricing. ShareTokenUpgradeable's getCirculatingSupplyAndAssets() aggregates across all vaults and includes invested assets, providing accurate system-wide valuation. The clamping to 0 at the vault level doesn't hide liabilities at the share token level. The reserved asset tracking prevents over-investment. No actual price inflation mechanism exists as described.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.totalAssets()`, the function returns `balance > reservedAssets ? balance - reservedAssets : 0`. This clamping to zero hides the deficit when `reservedAssets` (liabilities) exceed the vault's physical `balance`. 

`ShareTokenUpgradeable` calculates the share price as `(InvestedAssets + totalAssets()) / CirculatingSupply`. When `totalAssets()` clamps to 0 despite a deficit (e.g., due to pending redemptions exceeding liquid cash), the negative value of the liability is ignored. This artificially inflates the share price. Users redeeming at this time exit at an inflated value, stealing collateral from remaining shareholders and deepening the insolvency.

## Impact
Share price overestimation during liquidity crunches allows first-movers to withdraw more than their fair share, exacerbating insolvency for remaining users.

## Command to Run Test


## Proof of Concept
1. Vault has 100 USDC balance, 100 Invested. Supply 200. Price 1.0.
2. 150 USDC worth of redemptions are requested and fulfilled (`claimableRedeemAssets` = 150).
3. `reservedAssets` = 150. `balance` = 100. Deficit = -50.
4. `totalAssets()` returns 0 (clamped).
5. Price calculation: `(100 Invested + 0) / (200 - 150 Supply)`? No, `claimableRedeemShares` are removed from `CirculatingSupply`.
6. Let's say 150 shares pending redeem. Supply = 50.
7. Price = `100 / 50` = 2.0 USDC/share.
8. Real value should be `(100 Invested + 100 Balance - 150 Liability) / 50` = `50 / 50` = 1.0.
9. Remaining users redeem at 2.0, draining the vault instantly.

## Proof of Code
function testHiddenLiability() public {
    // Setup deficit scenario
    // Check share price
    // Assert price is inflated
}

## Suggested Mitigation
Allow `totalAssets` to return a signed integer or handle the deficit in the share price calculation by subtracting the net liability from the invested assets before division.


## [M-7]. Violation of idle asset spec: Claimable deposits are investable

## id: i0o8t73s1P-GPhMxU1jhA

## Derived From Pattern/Invariant
ReserveOrPriceDesync

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. The totalAssets() calculation incorrectly includes claimableDepositAssets in investable funds, violating the protocol specification that reserved assets should sit idle. Root cause exists NOW in current code (totalAssets only subtracts pendingDepositAssets, not claimableDepositAssets). Exploitable immediately when Investment Manager calls investAssets after deposits are fulfilled. No user error or governance decision required. Impact is Medium (protocol invariant violation exposing reserved assets to investment risk, not direct theft) with Occasional likelihood (requires specific timing where claimable deposits exist and manager invests).

--- Round 2 ---
ERC7575VaultUpgradeable.totalAssets() (line 1083-1096) calculates: balance - (totalPendingDepositAssets + totalClaimableRedeemAssets + totalCancelDepositAssets). It does NOT subtract claimableDepositAssets. When fulfillDeposit() is called, shares are minted and held by the vault, but the backing assets remain in the vault balance and are included in totalAssets(). This means investAssets() can move these claimable deposit assets to the investment vault, violating the protocol specification that 'Reserved assets (pending/claimable) sit idle'. The documentation states claimable deposits should be idle and available for immediate claiming, but the code allows them to be invested.

--- Round 3 ---
This is a valid accounting invariant violation. Per KNOWN_ISSUES.md Section 4 (line 234): 'Reserved assets (pending/claimable) sit idle, earning no yield.' However, totalAssets() in ERC7575VaultUpgradeable only subtracts pendingDepositAssets and claimableRedeemAssets, NOT claimableDepositAssets. This allows investAssets to move claimable deposit assets into the investment vault, violating the 'idle' guarantee. These assets should be available for immediate user claiming but are now exposed to investment risk. The mitigation is correct: subtract totalClaimableDepositAssets in totalAssets().
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `totalAssets()` function includes `claimableDepositAssets` (assets corresponding to fulfilled deposits that haven't been claimed yet) because it only subtracts `pendingDepositAssets`, `claimableRedeemAssets`, and `cancelDepositAssets`. Consequently, `investAssets` (which relies on `totalAssets` to determine investable liquidity) can move these claimable assets into the investment vault. This violates the protocol specification that 'Reserved assets (pending/claimable) sit idle'. Furthermore, since shares are minted upon fulfillment (increasing supply), putting the backing assets to risk (investment) rather than holding them idle contradicts the 'idle' guarantee intended for user withdrawals.

## Impact
Violation of protocol invariant regarding reserved assets. Assets that should be available for immediate user claiming are exposed to investment risk.

## Command to Run Test


## Proof of Concept
1. User requests deposit. Manager fulfills. `claimableDepositAssets` increases.
2. `totalAssets()` includes these assets.
3. Manager calls `investAssets(totalAssets())`.
4. The claimable assets are moved to investment vault.
5. User calls `deposit` (claim shares). They get shares, but the assets backing them are now invested, contrary to the 'idle' guarantee.

## Proof of Code
function testInvestClaimable() public {
    // Fulfill deposit
    // Check totalAssets includes claimable
    // Invest all assets
    // Assert claimable assets were moved
}

## Suggested Mitigation
Subtract `totalClaimableDepositAssets` in `totalAssets()` or explicitly exclude them in `investAssets` logic.


## [M-8]. ReadOnlyReentrancy via totalAssets Inflation

## id: UeT2qFlsgCInXuAx1LsQe

## Derived From Pattern/Invariant
ReadOnlyReentrancy

## Exploit Type
Reentrancy

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Duplicate of finding Z0nDdUU_GFwDH7QXZ0hON with same root cause and classification. This is both governance risk and non-standard token issue. Requires ERC777/hooks (non-standard tokens explicitly rejected per SafeTokenTransfers documentation). If admin deploys with ERC777, it's governance mistake (choosing incompatible token). Can be remedied by admin choosing standard ERC20 without hooks. Impact limited to view function manipulation. Per contest rules, admin token selection errors are governance risk (Low/QA), not Medium/High vulnerabilities.

--- Round 3 ---
This is a duplicate of finding Z0nDdUU_GFwDH7QXZ0hON with the same root cause: requestDeposit uses Pull-Then-Update pattern where safeTransferFrom is called before updating pendingDepositAssets. If the asset allows reentrancy (ERC777), totalAssets() reads inflated balanceOf minus old reservedAssets during the transfer. This manipulates ShareToken price calculations. The vulnerability is real and exploitable. The mitigation is correct: update state before external calls (CEI pattern) or use nonReentrantView on totalAssets().
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalAssets` function calculates the vault's value as `balanceOf(asset) - reservedAssets`. In `requestDeposit`, assets are transferred into the vault via `safeTransferFrom` (Pull-Then-Update) before the `reservedAssets` (specifically `pendingDepositAssets`) are updated. If the asset token allows reentrancy (e.g. ERC777 or via hooks), an attacker can reenter during the transfer. Inside the hook, `balanceOf` has increased but `reservedAssets` has not, causing `totalAssets` to be temporarily inflated. This inflates the ShareToken price (calculated via `getCirculatingSupplyAndAssets`) which can be exploited by external protocols or other system functions relying on the price.

## Impact
Manipulation of ShareToken price used by external integrations.

## Command to Run Test


## Proof of Concept
1. Attacker contract calls `requestDeposit` with ERC777-like asset.
2. `transferFrom` triggers `tokensToSend` hook.
3. Hook calls `ShareToken.convertNormalizedAssetsToShares`.
4. ShareToken reads `vault.totalAssets()`.
5. `totalAssets` sees new balance but old `reservedAssets` -> Inflated value.
6. Attacker profits from inflated price calculation.

## Proof of Code
function testReadOnlyReentrancy() public {
    // Implement hook
    // Call requestDeposit
    // Assert price > normal inside hook
}

## Suggested Mitigation
Update state variables (e.g. `pendingDepositAssets`) before transferring assets (Checks-Effects-Interactions) or use `nonReentrantView` on `totalAssets`.


## [M-9]. Strict Balance Equality Prevents Vault Unregistration

## id: yu8Vo9RxGWKKNjL96Tg6T

## Derived From Pattern/Invariant
Issue Type: ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. The unregisterVault function uses strict balance equality check that can be griefed by sending 1 wei to vault address. Since vault lacks permissionless sweep function, attacker can permanently prevent unregistration by front-running with dust transfers. Root cause exists NOW in current code (strict equality check without dust tolerance). Exploitable immediately and repeatedly. No user error or governance decision required. Impact is Medium (DoS of admin function, breaks lifecycle management but no direct fund loss) with Common likelihood (no preconditions, works anytime with minimal cost).

--- Round 2 ---
ShareTokenUpgradeable.unregisterVault() (lines 200-250) performs a strict check: IERC20(asset).balanceOf(vaultAddress) != 0. An attacker can send 1 wei of the asset to the vault address, causing this check to fail. While the vault has investAssets() to sweep funds, an attacker can front-run unregisterVault transactions with dust transfers. The function lacks a dust threshold or sweep mechanism. This is a valid DoS vector: the admin cannot unregister vaults with dust balances, and an attacker can persistently grief the unregistration process. The mitigation should allow unregistration with dust amounts below a threshold or sweep remaining assets.

--- Round 3 ---
This is a valid DoS vulnerability. ShareTokenUpgradeable.unregisterVault checks 'balanceOf(vault) != 0' which can be griefed by sending 1 wei to the vault address. Since ERC7575VaultUpgradeable has no permissionless sweep function (only investAssets restricted to manager), an attacker can continuously front-run unregisterVault with dust transfers, permanently preventing admin from unregistering the vault. This breaks lifecycle management. The mitigation is correct: allow unregisterVault to sweep remaining assets or ignore dust amounts below a threshold.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `unregisterVault` function in `ShareTokenUpgradeable` performs a strict check `IERC20(asset).balanceOf(vaultAddress) != 0` to ensure the vault is empty before unregistering. This check can be griefed by an attacker sending 1 wei of the asset to the vault address. Since the `ERC7575VaultUpgradeable` does not have a permissionless sweep function (only `investAssets` which is restricted to the manager), an attacker can continuously front-run the `unregisterVault` transaction with a dust transfer, permanently preventing the admin from unregistering the vault.

## Impact
Denial of Service on the `unregisterVault` admin function. Prevents proper lifecycle management and cleanup of deprecated vaults.

## Command to Run Test


## Proof of Concept
1. Admin prepares to call `unregisterVault(asset)`.
2. Attacker monitors mempool.
3. Attacker sends 1 wei of `asset` to `vaultAddress`.
4. Admin transaction executes: `balanceOf(vault) is 1`, reverts with `CannotUnregisterVaultAssetBalance`.
5. Admin clears balance via Investment Manager. Attacker sends 1 wei again.
6. Cycle repeats.

## Proof of Code
function testUnregisterGrief() public {
    vm.prank(attacker);
    token.transfer(address(vault), 1);
    vm.prank(owner);
    vm.expectRevert(IERC7575Errors.CannotUnregisterVaultAssetBalance.selector);
    shareToken.unregisterVault(address(token));
}

## Suggested Mitigation
Instead of checking `balanceOf != 0`, allow `unregisterVault` to sweep remaining assets to the admin or a recovery address, or ignore dust amounts.


## [M-10]. Permanent Denial of Service of `unregisterVault` via dust requests

## id: t0brb92GcHYxjHKiR9A2U

## Derived From Pattern/Invariant
StateGrowthOrStorageBloat

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. The unregisterVault function checks activeDepositRequestersCount/activeRedeemRequestersCount which can be permanently griefed by malicious user creating request, having it fulfilled, then claiming all but 1 wei (dust). This keeps them in active set indefinitely with no admin force-claim mechanism. Root cause exists NOW in current code (no dust threshold or force-claim). Exploitable immediately and permanently. No user error or governance decision required. Impact is Medium (permanent DoS of admin function, breaks lifecycle management but no direct fund loss) with Common likelihood (no preconditions, minimal cost to execute).

--- Round 2 ---
ShareTokenUpgradeable.unregisterVault() (lines 200-250) reverts if activeDepositRequestersCount != 0 or activeRedeemRequestersCount != 0. A user is added to these sets upon request and only removed when they fully claim. A malicious user can request deposit/redeem, have it fulfilled, then claim all but 1 wei, remaining in the active set indefinitely. The admin cannot force-claim or clear dust requests. This permanently prevents vault unregistration. While dust amounts are economically insignificant, the inability to remove them from the active sets creates a permanent DoS. The mitigation should allow unregistration if remaining amounts are below a dust threshold.

--- Round 3 ---
This is a valid DoS vulnerability. ShareTokenUpgradeable.unregisterVault reverts if activeDepositRequestersCount or activeRedeemRequestersCount is non-zero. A malicious user can request deposit/redeem, have it fulfilled, then claim all but 1 wei (dust), keeping them in the active set indefinitely. Since admin cannot force-claim or clear the set, the vault cannot be unregistered. This permanently breaks lifecycle management. The mitigation is correct: allow unregisterVault if remaining amounts are below dust threshold or implement force-claim/sweep for admins.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `unregisterVault` function in `ShareTokenUpgradeable` reverts if `activeDepositRequestersCount != 0` or `activeRedeemRequestersCount != 0`. A user is added to these sets upon request and only removed when they fully claim their assets/shares. A malicious user can initiate a request, have it fulfilled, and then claim all but 1 wei (dust). This keeps them in the active set indefinitely. Since the admin cannot force a claim or clear this set, the vault cannot be unregistered.

## Impact
Protocol administrators are permanently prevented from unregistering vaults, breaking the lifecycle management of the multi-asset system.

## Command to Run Test


## Proof of Concept
1. Attacker calls `requestDeposit(1000)`.
2. Manager fulfills.
3. Attacker calls `deposit(999)` (claims most, leaves 1).
4. `activeDepositRequesters` still contains attacker.
5. Admin calls `unregisterVault`. Reverts due to `CannotUnregisterVaultActiveDepositRequesters`.

## Proof of Code
function testUnregisterDoS() public {
    // Setup vault
    // User requests and fulfills
    // User claims partial
    // Admin tries to unregister -> Revert
}

## Suggested Mitigation
Allow `unregisterVault` to proceed if remaining amounts are below a dust threshold, or implement a force-claim/sweep mechanism for admins to clear dust requests.


## [M-11]. Share Price Manipulation via unregisterVault Ignoring Dust Shares

## id: -hyEI_kJYVeGbO_8Qy0mR

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
This is governance risk. The unregisterVault function checks totalClaimableRedeemAssets but not totalClaimableRedeemShares, allowing unregistration with dust shares remaining. However, this requires admin to call unregisterVault (admin action). Per contest rules, admin mistakes are governance risk. The admin should verify both assets AND shares are zero before unregistering (following spec). If admin unregisters with dust shares remaining, it's admin error/recklessness, not a code vulnerability. Can be remedied by admin being more careful (checking both values) without code changes. Per C4 rules, admin mistakes are Low/QA governance risk.

--- Round 3 ---
This is a valid accounting vulnerability. ShareTokenUpgradeable.unregisterVault checks totalClaimableRedeemAssets is zero but fails to check totalClaimableRedeemShares. Due to rounding in fulfillRedeem, a vault can have 0 assets but non-zero dust shares. If unregistered, these shares are no longer subtracted from totalSupply in getCirculatingSupplyAndAssets, causing CirculatingSupply to artificially increase while assets remain 0. This immediately drops share price, diluting all holders. The mitigation is correct: ensure totalClaimableRedeemShares is also 0 before allowing unregistration.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ShareTokenUpgradeable.unregisterVault` function allows the owner to remove a vault from the system if `metrics.totalClaimableRedeemAssets` is zero. However, it fails to check `metrics.totalClaimableRedeemShares`. Due to rounding down in `fulfillRedeem`, a vault can be left with 0 claimable assets but a non-zero amount of claimable shares (dust). If such a vault is unregistered, these shares are no longer subtracted from the `totalSupply` in `getCirculatingSupplyAndAssets`. This causes the calculated `CirculatingSupply` to artificially increase (since the deduction is removed), while the assets (0) remain 0. This results in an immediate drop in the share price, diluting all holders.

## Impact
Permanent loss of share value for all holders due to incorrect supply calculation.

## Command to Run Test


## Proof of Concept
1. Vault has dust shares in claimableRedeem (e.g., 100 wei shares, 0 assets).
2. Owner calls `unregisterVault` (succeeds as assets=0).
3. `getCirculatingSupplyAndAssets` no longer subtracts the 100 shares from supply.
4. `CirculatingSupply` increases by 100.
5. Price = Assets / (OldSupply + 100) -> Price drops.

## Proof of Code
function testUnregisterPriceDrop() public {
    // Setup vault with dust shares
    shareToken.unregisterVault(asset);
    // Check price drop
}

## Suggested Mitigation
Ensure `metrics.totalClaimableRedeemShares` is also 0 before allowing vault unregistration.


## [M-12]. DoS of `unregisterVault` via Dust Donation

## id: 7vBtMrczmc7UY7kNEOGkY

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Duplicate of finding yu8Vo9RxGWKKNjL96Tg6T with identical root cause and impact. Both describe the same strict balance equality check vulnerability that can be griefed by dust donations. Same exploitability (front-running with 1 wei), same impact (DoS of unregisterVault), same likelihood (no preconditions). Root cause exists NOW in current code, exploitable immediately. Medium impact (admin function DoS, lifecycle management broken but no fund loss) with Common likelihood (works anytime with minimal cost).

--- Round 2 ---
This is a duplicate of finding yu8Vo9RxGWKKNjL96Tg6T. ShareTokenUpgradeable.unregisterVault() enforces IERC20(asset).balanceOf(vaultAddress) != 0. An attacker can send 1 wei to the vault, causing the check to fail. While investAssets() can sweep funds, an attacker can front-run unregisterVault with dust donations. The strict equality check creates a DoS vector. The mitigation should relax the check to allow negligible dust or sweep remaining dust to a treasury. This is a valid finding: the admin cannot unregister vaults with dust balances, and the attack is persistent and low-cost.

--- Round 3 ---
This is a duplicate of finding yu8Vo9RxGWKKNjL96Tg6T with the same root cause: ShareTokenUpgradeable.unregisterVault enforces strict 'balanceOf(vault) != 0' check which can be griefed by sending 1 wei to the vault. An attacker can persistently front-run unregisterVault with dust donations, preventing unregistration. While admin can sweep via investAssets, the attacker can repeat the attack. This is a valid DoS vulnerability. The mitigation is correct: relax strict equality to allow negligible dust or sweep dust to treasury.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`ShareTokenUpgradeable.unregisterVault` enforces a strict check that the vault's asset balance is exactly zero (`IERC20(asset).balanceOf(vaultAddress) != 0`). An attacker can send a minimal amount (1 wei) of the asset directly to the vault address. This causes the check to fail and the transaction to revert. While the admin can attempt to sweep funds via `investAssets`, an attacker can persistently front-run the `unregisterVault` transaction with dust donations, effectively preventing unregistration.

## Impact
Denial of Service of the vault unregistration functionality.

## Command to Run Test


## Proof of Concept
1. Admin prepares to call `unregisterVault`.
2. Attacker monitors mempool and sends 1 wei of `asset` to the vault.
3. Admin's transaction executes, checks `balanceOf(vault) != 0`, and reverts with `CannotUnregisterVaultAssetBalance`.
4. Admin sweeps funds and retries; attacker repeats step 2.

## Proof of Code
function test_dosBalance() public {
    // ... setup empty vault ...
    deal(asset, address(vault), 1);
    vm.prank(owner);
    vm.expectRevert(IERC7575Errors.CannotUnregisterVaultAssetBalance.selector);
    shareToken.unregisterVault(asset);
}

## Suggested Mitigation
Relax the strict equality check to allow for negligible dust, or allow the `unregisterVault` function to sweep remaining dust to a treasury/admin address.





Finding Status: InvalidByDesign, InvalidNotExploitable
## [M-13]. Silent truncation of rBalance destroys investment accounting data

## id: qCdS3PckwlmFNtnZVKjMK

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.rBatchTransfers

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
The silent truncation of rBalance to zero when credit exceeds rBalance is intentional per Known Issues Section 7a: 'If rBalance insufficient, capital returned exceeds tracked investment. Silent truncation prevents revert, allowing transfer to complete. User's actual _balances are always correct.' However, this causes accounting drift for investment tracking. Impact is Medium (accounting corruption, not direct fund loss) as user balances remain correct but yield distribution may be affected. Likelihood is Occasional as it requires specific conditions where credits exceed rBalance tracking. Not governance risk as it's a design tradeoff, not admin misconfiguration.

--- Round 2 ---
In WERC7575ShareToken.rBatchTransfers() (lines 1100-1200), when an account is a creditor and credit exceeds rBalance, the code sets _rBalances[account.owner] = 0 without emitting an event or tracking the discrepancy. This is documented as intentional in KNOWN_ISSUES.md Section 7a ('rBalance Silent Truncation'), where it states rBalance is informational for revenue/yield tracking. However, this does destroy historical investment data. The adjustrBalance function relies on accurate rBalance for calculating returns. While user balances remain correct, the loss of investment tracking precision is a real issue, though marked as intentional design.

--- Round 3 ---
Per KNOWN_ISSUES.md Section 7a (lines 507-552), this is documented as intentional: 'rBalance Silent Truncation (Informational Tracking)' - 'rBalance represents INVESTED capital in external vaults (informational)'. The truncation to zero when credit exceeds rBalance is by design: 'Silent truncation prevents revert, allowing transfer to complete'. User funds are never lost as actual _balances are always correct. rBalance is secondary tracking for revenue/yield, not critical for transfers. This is informational accounting, not a security issue.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `WERC7575ShareToken.rBatchTransfers`, if an account acts as a creditor (receives liquid funds) and the credit amount exceeds their current `rBalance` (invested funds), the `rBalance` is silently truncated to 0 via `_rBalances[account.owner] = 0`. This destroys the historical investment data. While `rBalance` is informational, `adjustrBalance` relies on accurate accounting of invested amounts (`amounti`) to calculate returns. Zeroing out `rBalance` disrupts this tracking.

## Impact
Loss of precision in investment accounting, potentially leading to incorrect yield distribution or inability to track principal for specific accounts.

## Command to Run Test


## Proof of Concept
1. Account has 50 `rBalance`.
2. Account receives 100 payment in `rBatchTransfers` (credit).
3. `rBalance` logic: 50 < 100, so `rBalance` set to 0.
4. Information that 50 was invested is lost/cleared.

## Proof of Code
function testTruncation() public {
    // Setup rBalance 50
    // Exec rBatchTransfers credit 100
    // Assert rBalance is 0
}

## Suggested Mitigation
Emit an event when truncation occurs or track the negative `rBalance` (over-redemption) separately if strictly necessary for accounting.


## [M-14]. Strict Balance Check in SafeTokenTransfers Bricks Fee-on-Transfer Tokens

## id: i6nDG6pgYtUs2Yoze4zZC

## Derived From Pattern/Invariant
Issue Type: ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
SafeTokenTransfers.safeTransferFrom

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
This is a governance/centralization risk. The strict equality check in SafeTokenTransfers is intentional design per documentation (COMPATIBLE TOKENS section explicitly lists fee-on-transfer as incompatible). Admin/team chooses which tokens to deploy vaults for. If they deploy with USDT when fees are enabled, it's a governance mistake (choosing wrong token configuration), not a code vulnerability. The issue can be remedied by admin deploying with standard tokens or USDT with fees disabled, without code changes. Per contest rules, admin token selection errors are governance risk (Low/QA), not Medium/High vulnerabilities.

--- Round 3 ---
The strict balance check is intentional per SafeTokenTransfers.sol documentation (lines 10-35): 'INCOMPATIBLE TOKENS (will revert with TransferAmountMismatch): Fee-on-transfer tokens (SAFEMOON, USDT with fees, etc.)'. The library explicitly rejects fee-on-transfer tokens by design. USDT is in-scope, but the protocol only supports USDT with fees DISABLED (standard mode). This is a documented compatibility restriction, not a vulnerability. The check protects against accounting mismatches and silent value leakage.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SafeTokenTransfers` library used by `ERC7575VaultUpgradeable` enforces a strict equality check: `if (balanceAfter != balanceBefore + amount) revert TransferAmountMismatch();`. This logic is incompatible with any token that does not credit the exact transferred amount to the recipient's balance. USDT is explicitly in-scope and has a configurable fee-on-transfer mechanism. If this fee is enabled (or for any other supported token with similar mechanics), all deposits and withdrawals will revert, bricking the vault.

## Impact
Denial of Service for deposits and withdrawals if the underlying asset is a Fee-on-Transfer token (like USDT with fee enabled). User funds could be stuck if fees are turned on after deposits are made.

## Command to Run Test


## Proof of Concept
1. Deploy vault with a Fee-on-Transfer token (or mock USDT with fee on).
2. User calls `requestDeposit(100)`.
3. Token transfers 100, but vault receives 99 due to fee.
4. `SafeTokenTransfers` checks `balanceAfter - balanceBefore (99) == amount (100)`.
5. Check fails, transaction reverts.
6. Vault is unusable.

## Proof of Code
function testFoTBrick() public {
    // Mock FoT token
    fotToken.setFee(1); 
    vm.prank(user);
    vm.expectRevert(SafeTokenTransfers.TransferAmountMismatch.selector);
    vault.requestDeposit(100, user, user);
}

## Suggested Mitigation
Modify `SafeTokenTransfers` to calculate the actual received amount (`balanceAfter - balanceBefore`) and return it, rather than enforcing strict equality. Update vault logic to use this actual amount for accounting.


## [M-15]. Missing slippage protection in asynchronous deposit and redeem requests

## id: LTj48UooCvz6hNoDhP-1B

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
Valid Medium finding. The async vault lacks slippage protection parameters (minShares/minAssets) in requestDeposit/requestRedeem, exposing users to unlimited slippage between request and fulfillment. Root cause exists NOW in current code (functions lack slippage parameters). Exploitable immediately during normal operations when exchange rates change unfavorably. No user error required (users cannot specify limits even if they want to). Not governance risk (cannot be fixed by admin making different choices without code changes). Impact is Medium (value loss but not total fund theft) with Occasional likelihood (requires significant rate changes between request and fulfillment).

--- Round 2 ---
ERC7575VaultUpgradeable.requestDeposit() and requestRedeem() (lines 300-400) do not accept minShares or minAssets parameters. Users submit requests without knowing the exchange rate at fulfillment time. The Investment Manager calls fulfillDeposit/fulfillRedeem at their discretion, potentially at unfavorable rates. While this is documented as intentional async design in KNOWN_ISSUES.md Section 4, it does expose users to unlimited slippage. The ERC-7887 cancelation mechanism provides some protection (users can cancel pending requests), but once fulfilled, users must accept the rate. This is a valid Medium finding: users lack slippage protection during the request-to-fulfillment window.

--- Round 3 ---
Per KNOWN_ISSUES.md Section 4 (lines 225-244), async timing without deadlines is intentional: 'No Fulfillment Deadlines - Investment Manager can delay fulfillments indefinitely. No SLA enforcement.' This is the ERC-7540 async design for professional fund management. Users accept this model for institutional investment. The lack of slippage parameters is a design choice, not a vulnerability. Users can cancel pending requests via ERC-7887 if unhappy with timing (Section 4, lines 245-282). This is QA/Low at most per C4 criteria.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestDeposit` and `requestRedeem` functions in `ERC7575VaultUpgradeable` initiate asynchronous operations but do not allow users to specify a minimum amount of shares to mint (for deposits) or minimum assets to receive (for redeems). Since the exchange rate is determined at the time of fulfillment (by the Investment Manager) rather than the request time, users are exposed to unlimited slippage if the share price changes unfavorably between request and fulfillment.

## Impact
Users may receive significantly fewer shares or assets than expected due to market fluctuations or front-running of the fulfillment transaction.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit(1000 USDC)` when price is 1.0 (Expects 1000 shares).
2. Market moves or `adjustrBalance` creates loss, Price becomes 2.0 (or reverse logic for shares).
3. Manager calls `fulfillDeposit`. User gets 500 shares.
4. User has no ability to revert based on the bad rate.

## Proof of Code
function testNoSlippage() public {
    // Request deposit
    // Change share price drastically
    // Fulfill deposit
    // Assert user received unfavorable amount
}

## Suggested Mitigation
Add `minShares` parameter to `requestDeposit` and `minAssets` parameter to `requestRedeem`. Store these constraints and verify them during `fulfillDeposit/Redeem` or allow user cancellation if constraints are not met.



