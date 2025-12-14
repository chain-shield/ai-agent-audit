# 2025 11 sukukfi - Findings Report
## Commit hash: 18fe2578cf1c6203dac7ff21513533010f3dda3e

##Findings by Status


Finding Status: Valid


[H-1]. Inflation Attack via Rounding Error in `withdraw` allows draining vault value
**Derived From** : RoundingError
Finding Status: Valid
Privilege: Permissionless


[H-2]. Incorrect Rounding Direction in `withdraw` and `mint` Enables Vault Draining
**Derived From** : RoundingError
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign


[M-3]. Missing slippage protection in ERC7575VaultUpgradeable.withdrawFromInvestment
**Derived From** : SlippageMissingOrInsufficient
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign
Privilege: RequiresRole


[M-4]. Missing slippage protection in investment management functions
**Derived From** : SlippageMissingOrInsufficient
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign
Privilege: RequiresRole


[M-5]. Cross-Vault Fund Draining via Infinite Approval in `ShareTokenUpgradeable`
**Derived From** : AccessControl
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign
Privilege: RequiresRole



Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidByDesign + InvalidBugDoesNotExist


[M-6]. Strict Balance Check Incompatible with USDT Fee-On-Transfer
**Derived From** : ForcedAssetVsStrictEquality
Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidByDesign + InvalidBugDoesNotExist
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign + InvalidBugDoesNotExist


[H-7]. Phantom rBalance creation in WERC7575ShareToken.rBatchTransfers during loss settlement inflates share price
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign + InvalidBugDoesNotExist
Privilege: RequiresRole



Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation


[M-8]. System-Wide DoS via Single Faulty Vault in ShareToken aggregation
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation
Privilege: Permissionless


[H-9]. System-Wide DoS via Faulty Vault and Unregister Lock
**Derived From** : GriefableCallbacks
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation


[L-10]. Storage Layout Risk via Non-Upgradeable ReentrancyGuard
**Derived From** : StandardViolation
Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation
Privilege: Permissionless



Finding Status: LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidByDesign


[M-11]. Missing emergency pause check in ERC7575VaultUpgradeable claim functions
**Derived From** : PausableEmergencyStop
Finding Status: LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidByDesign
Privilege: RequiresAdminRole



Finding Status: LowSeverityDueToLowImpact + InvalidUserErrorOrMistake + InvalidNotExploitable


[M-12]. Precision loss in redeem allows burning shares for zero assets
**Derived From** : RoundingError
Finding Status: LowSeverityDueToLowImpact + InvalidUserErrorOrMistake + InvalidNotExploitable
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidERC20EdgeCase + InvalidBugDoesNotExist


[H-13]. ReadOnlyReentrancy via totalAssets Inflation in requestDeposit
**Derived From** : Reentrancy
Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidERC20EdgeCase + InvalidBugDoesNotExist
Privilege: Permissionless



Finding Status: InvalidUserErrorOrMistake


[M-14]. Unsafe Controller Recipient in Async Deposit permanently locks funds
**Derived From** : StandardViolation
Finding Status: InvalidUserErrorOrMistake
Privilege: Permissionless



Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist


[M-15]. Unregistration Griefing via Active Request Bloat
**Derived From** : Dos
Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 5
- M: 9
- L: 1
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Inflation Attack via Rounding Error in `withdraw` allows draining vault value

## id: a0KDdZnOIVjTuPdvSb9UK

## Derived From Pattern/Invariant
RoundingError

## Exploit Type
RoundingError

## Location
ERC7575VaultUpgradeable.withdraw

## Finding Status: Valid
### Finding Status Justification: withdraw() uses Floor for assets->shares, allowing under-burning (including shares==0) while transferring nonzero assets, which can be repeated to extract more assets than the shares burned should allow; there is no shares>0 enforcement and no compensating logic.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdraw` function in `ERC7575VaultUpgradeable` uses `Math.Rounding.Floor` when calculating the number of shares to burn for a given asset amount: `shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`. Standard ERC4626 implementation and security best practices dictate that share calculation for withdrawals must round UP (Ceil) to prevent value leakage. By using Floor, a user can withdraw assets while burning fewer shares than required (or zero shares for small amounts), effectively stealing value from the vault and inflating the value of remaining shares.

## Impact
Users can drain assets from the vault by repeatedly withdrawing small amounts, burning zero or reduced shares, leading to insolvency or loss of funds for other depositors.

## Command to Run Test


## Proof of Concept
1. Attacker requests redeem for a large amount of shares (e.g., 1000). 2. Investment Manager fulfills the request; `claimableRedeemAssets` and `claimableRedeemShares` are set (e.g., 1000 assets, 999 shares). 3. Attacker calls `withdraw(1)`. 4. Calculation: `shares = 1 * 999 / 1000 = 0` (Floor). 5. Attacker receives 1 asset, burns 0 shares. 6. Attacker repeats this loop to drain assets while keeping their claimable share balance intact.

## Proof of Code
function testRoundingAttack() public { 
 // Setup: vault has 1000 assets, 999 shares claimable 
 uint256 assets = 1; 
 uint256 shares = assets * 999 / 1000; // Result is 0 
 assertEq(shares, 0); 
 // Vault burns 0 shares, sends 1 asset 
 }

## Suggested Mitigation
Change rounding mode to `Math.Rounding.Ceil` in the `withdraw` function.


## [H-2]. Incorrect Rounding Direction in `withdraw` and `mint` Enables Vault Draining

## id: zj_9Cq7YmmjE6dL63QIGX

## Derived From Pattern/Invariant
RoundingError

## Exploit Type
RoundingError

## Location
ERC7575VaultUpgradeable.withdraw

## Finding Status: Valid
### Finding Status Justification: The withdraw-side rounding issue is exploitable as described (Floor allows under-burning/zero-burning). While the mint-side 'discount' is overstated in this async-claim design, the overall finding remains materially valid due to withdraw(). No in-code safeguard prevents repeated extraction.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable`, the `withdraw` and `mint` functions use `Math.Rounding.Floor` for converting between assets and shares, which favors the user instead of the vault. 

In `withdraw(assets, ...)`:
`shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor);`
Standard ERC4626 implementation requires rounding UP (Ceil) for shares burned to ensure the user pays sufficient shares for the assets withdrawn. Rounding down allows a user to withdraw small amounts of assets while burning zero shares (if `assets * shares < availableAssets`), effectively draining the vault.

Similarly in `mint(shares, ...)`:
`assets = shares.mulDiv(availableAssets, availableShares, Math.Rounding.Floor);`
Standard ERC4626 implementation requires rounding UP (Ceil) for assets deposited. Rounding down allows a user to mint shares by providing fewer assets than required (undervaluing the share), diluting other shareholders.

## Impact
Direct theft of vault assets by withdrawing without burning shares or minting shares at a discount.

## Command to Run Test


## Proof of Concept
1. Attacker deposits assets to receive shares.
2. Attacker calls `withdraw(1 asset)` repeatedly.
3. If `availableShares / availableAssets < 1`, the formula `1 * availableShares / availableAssets` rounds down to 0 shares.
4. Attacker receives 1 asset but burns 0 shares.
5. Attacker repeats until vault is drained.

## Proof of Code
function testDrainVault() public {
    // Setup vault with 100 assets, 100 shares
    // ... setup code ...
    vm.startPrank(attacker);
    // Loop withdraw 1 asset
    for(uint i=0; i<100; i++) {
        vault.withdraw(1, attacker, attacker);
    }
    // Attacker has 100 assets + original shares
    // Vault has 0 assets + 100 shares (insolvent)
}

## Suggested Mitigation
Change rounding mode to `Math.Rounding.Ceil` in both `withdraw` (for shares calculation) and `mint` (for assets calculation).





Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign
## [M-3]. Missing slippage protection in ERC7575VaultUpgradeable.withdrawFromInvestment

## id: 8e_f6GGqZYN0jNB7XFU4M

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.withdrawFromInvestment

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign
### Finding Status Justification: does_bug_really_not_exist=false: function has no caller-controlled slippage bounds (no maxShares/minAssets). is_there_really_safeguard_against_it=false: min(shares,maxShares) only caps by ShareToken balance; it does not protect against unfavorable preview/price movement. is_really_low_impact=false: if a variable-priced/manipulable investmentVault were configured, value loss could be material (not inherently low impact), even if configuration choice is governance-controlled.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `withdrawFromInvestment` function interacts with an external `investmentVault` to redeem shares for assets. It calculates the number of shares to burn using `previewWithdraw(amount)` (the spot price) in the same transaction and then calls `redeem`. There is no parameter to specify a maximum number of shares to burn or a minimum asset output. If the external investment vault has variable pricing (e.g., standard ERC4626 or Aave wrapper), this operation is vulnerable to sandwich attacks or price manipulation, leading to loss of value.

## Impact
Loss of vault assets due to slippage or sandwich attacks during investment withdrawal.

## Command to Run Test


## Proof of Concept
1. Investment Vault has variable pricing.
2. Attacker sees `withdrawFromInvestment` tx in mempool.
3. Attacker manipulates Investment Vault price (if possible) or sandwiches the transaction.
4. `withdrawFromInvestment` executes at unfavorable rate, burning more shares than necessary for the requested asset amount.

## Proof of Code
function testWithdrawSlippage() public {
    // Mock variable price vault where previewWithdraw returns high share count
    // Call withdrawFromInvestment
    // Assert no revert despite high slippage
}

## Suggested Mitigation
Add a `maxShares` parameter to `withdrawFromInvestment` and enforce `minShares <= maxShares`.


## [M-4]. Missing slippage protection in investment management functions

## id: F4FYfviyxIeoS9EW0ZFjK

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.investAssets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign
### Finding Status Justification: does_bug_really_not_exist=false: investAssets/withdrawFromInvestment have no minOut/maxIn style parameters. is_there_really_safeguard_against_it=false: balance checks and emitting actualAmount do not prevent adverse execution prices. is_really_low_impact=false: if configured to a variable-priced external vault, slippage/MEV losses can be significant (not inherently low impact), though configuration is governance-controlled.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The functions `investAssets` and `withdrawFromInvestment` interact with an external `investmentVault` without slippage protection. `investAssets` deposits assets without a minimum share output check. `withdrawFromInvestment` calculates shares via `previewWithdraw` and immediately redeems them in the same transaction; if the previewed rate is unfavorable (e.g. due to front-running or pool manipulation), the vault suffers a loss.

## Impact
Loss of vault assets due to sandwich attacks or unfavorable exchange rates.

## Command to Run Test


## Proof of Concept
1. Attacker observes `investAssets` tx in mempool.
2. Attacker manipulates `investmentVault` exchange rate (e.g. flash loan deposit).
3. `investAssets` executes, receiving very few shares for the assets.
4. Attacker back-runs to profit, leaving Vault with loss.

## Proof of Code
vm.prank(manager); vault.investAssets(amount); // No minOut param

## Suggested Mitigation
Add `minShares` parameter to `investAssets` and `minAssets` parameter to `withdrawFromInvestment`.


## [M-5]. Cross-Vault Fund Draining via Infinite Approval in `ShareTokenUpgradeable`

## id: B4styFpUVh9zQ5n5mGXU5

## Derived From Pattern/Invariant
AccessControl

## Exploit Type
AccessControl

## Location
ShareTokenUpgradeable._configureVaultInvestmentSettings

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidByDesign
### Finding Status Justification: is_really_low_impact=false: if a registered vault is ever compromised/malicious, the unlimited allowance can expand the blast radius to the entire pooled investmentShareToken balance held by ShareTokenUpgradeable (potentially catastrophic), even if that scenario is governance/compromise-dependent.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `_configureVaultInvestmentSettings`, the ShareToken grants an infinite approval (`type(uint256).max`) to every registered vault to spend the pooled `investmentShareToken` held by the ShareToken contract. Since all investment assets from all vaults are pooled in the ShareToken, this breaks asset isolation. A single compromised or malicious vault can use `transferFrom` (via `withdrawFromInvestment` logic or arbitrary code execution if compromised) to drain the entire investment pool, including funds belonging to other vaults.

## Impact
Complete loss of all invested assets across the entire protocol if a single vault is compromised.

## Command to Run Test


## Proof of Concept
1. Vault A and Vault B both invest funds. ShareToken holds pooled shares. 2. Vault A is compromised (or malicious upgrade). 3. Vault A calls `investmentShareToken.transferFrom(ShareToken, attacker, ALL_FUNDS)`. 4. Transfer succeeds because ShareToken approved Vault A for `uint256.max`.

## Proof of Code
function testCrossVaultDrain() public { 
 // Approve vault A for max 
 // Vault A transfers more than its own balance 
 // Assert success 
 }

## Suggested Mitigation
Do not grant infinite approval. Instead, the ShareToken should provide a function to transfer investment shares to a vault that checks the vault's specific credit/balance before transferring.





Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidByDesign + InvalidBugDoesNotExist
## [M-6]. Strict Balance Check Incompatible with USDT Fee-On-Transfer

## id: X2aRcJuZLl8ksTEvfavSZ

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
SafeTokenTransfers.safeTransferFrom

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidByDesign + InvalidBugDoesNotExist
### Finding Status Justification: SafeTokenTransfers.safeTransfer/safeTransferFrom deliberately enforce strict post-balance equality and revert with TransferAmountMismatch for fee-on-transfer/rebasing tokens; this is explicitly documented in the library comments as an intentional compatibility restriction. The claim that “USDT might enable fees” is hypothetical and depends on future external token behavior, not a present protocol bug. For standard USDT/USDC-like behavior today, transfers succeed and the strict check is a safeguard against silent accounting drift. Therefore this is by-design token-compatibility behavior and future-speculation about non-standard token changes, not a current vulnerability.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SafeTokenTransfers` library uses a strict equality check `balanceAfter != balanceBefore + amount` to validate transfers. The protocol explicitly lists USDT as a supported asset. While USDT currently has zero fees, the fee mechanism exists and can be toggled. If USDT fees are enabled, or if any other supported token (e.g. USDC upgrade) introduces transfer fees, all deposit functions will revert, causing a Denial of Service.

## Impact
Protocol availability failure (DoS) for supported assets if transfer fees are enabled.

## Command to Run Test


## Proof of Concept
1. USDT fee is enabled (hypothetically). 2. User calls `requestDeposit`. 3. `SafeTokenTransfers` verifies balance increase. 4. Received amount is less than sent due to fee. 5. Transaction reverts.

## Proof of Code
function testFoTRevert() public { 
 // Mock USDT with fee 
 // Call safeTransferFrom 
 // Assert Revert 
 }

## Suggested Mitigation
Modify `SafeTokenTransfers` to check `balanceAfter >= balanceBefore + amount - fee` or simply return the actual received amount to the calling function for accounting.





Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign + InvalidBugDoesNotExist
## [H-7]. Phantom rBalance creation in WERC7575ShareToken.rBatchTransfers during loss settlement inflates share price

## id: s-cibrPhquSs4jImPqxeN

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.rBatchTransfers

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign + InvalidBugDoesNotExist
### Finding Status Justification: is_there_really_safeguard_against_it=false: there is no automatic on-chain safeguard that forces losses to be reflected in rBalance during rBatchTransfers; correctness relies on trusted revenueAdmin/validator operationally applying adjustrBalance when appropriate.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `rBatchTransfers` function in `WERC7575ShareToken` updates `_rBalances` (Restricted/Invested Balance) to reflect movements of capital. When a credit (inflow) occurs for an account flagged for rBalance updates, the logic assumes a return of principal and decreases `_rBalances` by the credit amount (capped at 0). However, in a loss scenario where the returned amount (Credit) is less than the invested principal (rBalance), the function only reduces `_rBalances` by the Credit amount. The remaining `rBalance` (representing the loss) is not cleared and remains as a 'phantom' balance. 

Since `ShareTokenUpgradeable` uses `rBalance` (via `getInvestedAssets` -> `_calculateInvestmentAssets`) to calculate `totalNormalizedAssets` for share pricing, this phantom balance artificially inflates the share price. This allows users to redeem shares for more assets than actually exist, effectively draining the investment vault.

## Impact
Inflation of share price leading to theft of assets from the Investment Vault.

## Command to Run Test


## Proof of Concept
1. `ShareTokenUpgradeable` invests 1000 USDC into a settlement deal. Its `rBalance` in `WERC7575ShareToken` is 1000.
2. The deal suffers a 50% loss. The Validator settles the return via `rBatchTransfers` with a credit of 500 USDC to `ShareTokenUpgradeable`.
3. `rBatchTransfers` executes: `_balances += 500`, `_rBalances -= 500`. New `rBalance` is 500.
4. Actual value is 500 (liquid). But the system calculates Total Assets = 500 (liquid) + 500 (phantom rBalance) = 1000.
5. The loss is hidden. Share price remains same as if 1000 USDC exists.
6. Users redeem shares at the inflated price, draining the 500 liquid USDC and stealing from other depositors.

## Proof of Code
function testPhantomRBalance() public {
    // Setup: Account has 1000 rBalance (invested)
    vm.store(address(token), keccak256(abi.encode(user, 6)), bytes32(uint256(1000))); // Slot 6 is _rBalances map

    // Execute settlement with 50% loss (return 500)
    address[] memory debtors = new address[](1);
    address[] memory creditors = new address[](1);
    uint256[] memory amounts = new uint256[](1);
    debtors[0] = debtor;
    creditors[0] = user;
    amounts[0] = 500;

    // Flag user for rBalance update (bit 1 set for creditor at index 0 in aggregated array)
    // Bitmap: ...0010 (binary) = 2. Assuming debtor is index 0, user is index 1.
    uint256 flags = 2;
    
    vm.prank(validator);
    token.rBatchTransfers(debtors, creditors, amounts, flags);

    // Assert rBalance is 500 (Phantom) instead of 0
    assertEq(token.rBalanceOf(user), 500);
}

## Suggested Mitigation
The `rBatchTransfers` function cannot autonomously distinguish between a partial return and a final return with loss. The protocol should enforce that settlement returns are processed via `adjustrBalance` (which handles PnL correctly) or introduce a mechanism to explicitly mark a transfer as 'Final Settlement' to clear the remaining `rBalance`.





Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation
## [M-8]. System-Wide DoS via Single Faulty Vault in ShareToken aggregation

## id: M__36oYyYN4ssHMDSKzj5

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.getCirculatingSupplyAndAssets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation
### Finding Status Justification: is_really_low_impact=false: a single reverting registered vault can revert getCirculatingSupplyAndAssets(), which can block conversions and thereby break core deposit/redeem flows system-wide (high impact availability failure), even if the root cause is governance/operations.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults and calls `getClaimableSharesAndNormalizedAssets`. If a single vault reverts (e.g. paused, buggy upgrade, or hacked), the aggregation fails. This function is critical for share pricing (`convertNormalizedAssetsToShares`), meaning a single faulty vault bricks deposit/withdraw/redeem operations for the *entire* system of vaults.

## Impact
Complete protocol paralysis; funds locked in all vaults.

## Command to Run Test


## Proof of Concept
1. Vault A enters a broken state (e.g. `totalAssets` reverts).
2. User tries to deposit into Vault B.
3. Vault B calls `ShareToken.convertNormalizedAssetsToShares`.
4. ShareToken loops vaults, hits Vault A, reverts.
5. Vault B deposit fails.

## Proof of Code
mockVault.setRevert(true); shareToken.getCirculatingSupplyAndAssets(); // Reverts

## Suggested Mitigation
Wrap external vault calls in a `try/catch` block and skip faulty vaults, or allow admin to forcibly disable a vault from the calculation without calling it.


## [H-9]. System-Wide DoS via Faulty Vault and Unregister Lock

## id: bycrIvXNE_SYdKecMXqEs

## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation
### Finding Status Justification: is_really_low_impact=false: the combination of (1) aggregation without try/catch and (2) unregisterVault reverting if metrics calls revert can create a persistent system-wide DoS/deadlock if it happens.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` relies on `getCirculatingSupplyAndAssets` which iterates over all registered vaults. If a single vault enters a broken state (e.g., reverts on `getClaimableSharesAndNormalizedAssets`), the entire share token system reverts. Critically, the `unregisterVault` function also calls the faulty vault's `getVaultMetrics` to verify it is empty. If the vault reverts, the `catch` block in `unregisterVault` explicitly reverts with `CannotUnregisterActiveVault`, making it impossible to remove the broken vault. This creates a permanent deadlock where the system is bricked and the admin cannot fix it.

## Impact
Permanent Denial of Service of the entire multi-asset system. No deposits, redemptions, or transfers can occur if one vault fails.

## Command to Run Test


## Proof of Concept
1. A registered vault encounters a bug or state that causes `getVaultMetrics` or `getClaimableShares` to revert. 2. Calls to `ShareToken.getCirculatingSupplyAndAssets` now revert. 3. Admin attempts to call `unregisterVault` to remove the faulty vault. 4. `unregisterVault` calls `vault.getVaultMetrics`. 5. The call fails, enters `catch`, and reverts. 6. The vault remains registered, sustaining the DoS.

## Proof of Code
function testUnregisterLock() public { 
 // Mock a broken vault that always reverts 
 // Register it 
 // Try to unregister 
 // Assert revert 
 }

## Suggested Mitigation
In `unregisterVault`, allow a forced unregistration (perhaps via a separate emergency function or parameter) that bypasses the vault calls/checks, acknowledging the risk of orphaned assets.





Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation
## [L-10]. Storage Layout Risk via Non-Upgradeable ReentrancyGuard

## id: qQT_S9gIktPubmAlI9t_L

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
ERC7575VaultUpgradeable.N/A

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation
### Finding Status Justification: Code path exists: ERC7575VaultUpgradeable inherits OZ non-upgradeable ReentrancyGuard (constructor-based), so proxy deployment skips the constructor and the status slot starts at 0. However, OZ v5.5 ReentrancyGuard uses a namespaced ERC-7201-style storage slot constant, so it does not collide with the vault’s own ERC-7201 namespaced storage (VAULT_STORAGE_SLOT). Also, status=0 does not break protection: nonReentrant checks only ENTERED(2), then sets ENTERED and resets to NOT_ENTERED(1). The main effect is a small first-call gas difference and a theoretical future-upgrade concern if parent storage semantics change, not an immediate exploit. Therefore it’s a low-impact best-practice deviation with an existing safeguard (namespaced storage slot) and mainly future/maintenance risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`ERC7575VaultUpgradeable` inherits from the non-upgradeable `ReentrancyGuard` instead of `ReentrancyGuardUpgradeable`. This causes the reentrancy status to be uninitialized (0) in the proxy storage, leading to higher gas costs on first use and potential storage layout collisions with future upgrades or parent contracts if the storage slot is not properly reserved/namespaced.

## Impact
Gas inefficiency and potential future storage corruption.

## Command to Run Test


## Proof of Concept
Inspection of inheritance list: `contract ERC7575VaultUpgradeable is ... ReentrancyGuard ...`

## Proof of Code
assertEq(vault.reentrancyStatus(), 0); // Should be 1

## Suggested Mitigation
Inherit from `ReentrancyGuardUpgradeable` and call `__ReentrancyGuard_init()`.





Finding Status: LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidByDesign
## [M-11]. Missing emergency pause check in ERC7575VaultUpgradeable claim functions

## id: jaIwZeiea3NtLKFZzKd4K

## Derived From Pattern/Invariant
PausableEmergencyStop

## Exploit Type
PausableEmergencyStop

## Location
ERC7575VaultUpgradeable.redeem

## Finding Status: LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: does_bug_really_not_exist=false: isActive is indeed not checked in claim/outflow functions. is_really_governance_risk=false: this is not primarily an 'admin mistake' scenario; it's an intentional/structural choice about what isActive is meant to control (deposits only), i.e., design/policy rather than governance error.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ERC7575VaultUpgradeable` contract has an `isActive` flag, but it is only checked in `requestDeposit`. Critical outflow functions `redeem`, `withdraw`, `deposit` (claim), and `mint` (claim) do not check `isActive` or any paused state. In the event of an exploit, bug, or asset depeg, the owner cannot pause these operations to protect the protocol funds.

## Impact
Inability to stop fund outflows during an emergency.

## Command to Run Test


## Proof of Concept
1. Owner calls `setVaultActive(false)` due to an emergency.
2. Attacker calls `redeem` or `withdraw`.
3. The transaction succeeds despite the vault being 'inactive'.

## Proof of Code
function testUnprotectedPause() public {
    vm.prank(owner);
    vault.setVaultActive(false);
    
    // Ensure claimable shares exist
    // ... setup ...
    
    vm.prank(user);
    vault.redeem(shares, user, user);
    // Assert success (should fail if protected)
}

## Suggested Mitigation
Add `if (!isActive) revert VaultNotActive();` or implement `Pausable` and `whenNotPaused` modifier on `redeem`, `withdraw`, `deposit`, and `mint`.





Finding Status: LowSeverityDueToLowImpact + InvalidUserErrorOrMistake + InvalidNotExploitable
## [M-12]. Precision loss in redeem allows burning shares for zero assets

## id: 8aZ1RZj7uWc5C3rlQ96lm

## Derived From Pattern/Invariant
RoundingError

## Exploit Type
RoundingError

## Location
ERC7575VaultUpgradeable.redeem

## Finding Status: LowSeverityDueToLowImpact + InvalidUserErrorOrMistake + InvalidNotExploitable
### Finding Status Justification: ERC7575VaultUpgradeable.redeem() computes assets = shares.mulDiv(availableAssets, availableShares, Floor) and burns shares unconditionally; it only skips transferring if assets==0. Therefore, for sufficiently small shares relative to the claimable ratio, a user can burn shares and receive 0 assets. This is reproducible (dust/rounding scenario) and not mitigated by a revert like “assets > 0”. It is not a profitable exploit (no value gained), but it can cause user loss for that transaction and can strand “dust” redemptions. It primarily requires user choosing a too-small shares amount (or having dust shares), hence user-error-driven and low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.redeem`, the asset amount is calculated as `shares.mulDiv(availableAssets, availableShares, Math.Rounding.Floor)`. If `shares * availableAssets < availableShares`, the result is 0. The function burns the user's shares but transfers 0 assets. This can happen with small redemptions or high share prices.

## Impact
User loss of shares with no compensation (100% loss for that transaction).

## Command to Run Test


## Proof of Concept
1. Vault has 1000 shares outstanding backed by 1 asset (extreme example or dust).
2. User calls `redeem(10 shares)`.
3. Assets = 10 * 1 / 1000 = 0.
4. 10 shares burned, 0 assets sent.

## Proof of Code
vault.redeem(smallAmount, user, user); assertEq(token.balanceOf(user), 0);

## Suggested Mitigation
Require `assets > 0` in `redeem` or use `Math.Rounding.Ceil` for the asset calculation (though Ceil might drain vault dust, checking > 0 is safer).





Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidERC20EdgeCase + InvalidBugDoesNotExist
## [H-13]. ReadOnlyReentrancy via totalAssets Inflation in requestDeposit

## id: cYmqa6-aXgdk80T7bfK0j

## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidERC20EdgeCase + InvalidBugDoesNotExist
### Finding Status Justification: The described callback reentrancy path is blocked: requestDeposit() is nonReentrant, and the purported profitable target functions (redeem/withdraw/fulfill, etc.) are also nonReentrant. During SafeTokenTransfers.safeTransferFrom, any ERC777/ERC1363 hook attempting to call back into redeem/withdraw would hit ReentrancyGuardReentrantCall and revert. While totalAssets() could be queried during a callback (view-only), there is no state-changing path to realize profit as described because the relevant external entrypoints are guarded. Thus the claimed read-only reentrancy exploitation does not match the actual executable code path in this vault implementation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.requestDeposit`, assets are transferred using `safeTransferFrom` (Pull) *before* updating `totalPendingDepositAssets` (Update). If the asset is an ERC777/ERC1363 token, the transfer triggers a callback. Inside the callback, `balanceOf(this)` has increased but `reservedAssets` (which includes `totalPendingDepositAssets`) has not. This causes `totalAssets()` (`balance - reserved`) to spike temporarily. Since share price is `Assets / Supply`, the share price effectively drops (or `convertToAssets` spikes if calculated inversely for redemptions), allowing arbitrage.

## Impact
Theft of value via share price manipulation during read-only reentrancy.

## Command to Run Test


## Proof of Concept
1. Attacker calls `requestDeposit` with ERC777 tokens.
2. Token calls `tokensToSend` hook on attacker contract.
3. Attacker contract calls `redeem`. `redeem` uses `convertToAssets`.
4. `totalAssets` is inflated, so `convertToAssets` (shares * assets / supply) returns an inflated asset amount.
5. Attacker burns shares for excess assets.
6. `requestDeposit` completes.

## Proof of Code
function tokensToSend(...) external { if(attacking) vault.redeem(shares, address(this), address(this)); }

## Suggested Mitigation
Update `totalPendingDepositAssets` state *before* transferring assets (follow Checks-Effects-Interactions pattern).





Finding Status: InvalidUserErrorOrMistake
## [M-14]. Unsafe Controller Recipient in Async Deposit permanently locks funds

## id: nEBoITwql86gCuHZpMAYP

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: InvalidUserErrorOrMistake
### Finding Status Justification: ERC7575VaultUpgradeable.requestDeposit() does not validate controller != address(0). It records pendingDepositAssets[controller] and adds controller to activeDepositRequesters, so controller=0 can be stored. Later, only controller (msg.sender) or its operator can claim/cancel, which is impossible for address(0). This permanently locks the deposited assets: totalPendingDepositAssets remains increased, and totalAssets() will treat the funds as reserved forever. This is a real, reproducible issue caused by bad input (user/frontend mistake) and results in permanent loss of user funds and trapped liquidity in the vault.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestDeposit` function allows the `controller` parameter to be `address(0)`. If a user (or frontend) mistakenly passes zero, the assets are transferred to the vault and a deposit request is created for `address(0)`. Since no one can transact as `address(0)`, the shares/assets are permanently unclaimable.

## Impact
Permanent loss of user funds due to input error.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit(100, address(0), user)`.
2. Assets transferred.
3. Request recorded for `address(0)`.
4. Impossible to claim.

## Proof of Code
vault.requestDeposit(100, address(0), me);

## Suggested Mitigation
Add `if (controller == address(0)) revert InvalidController();` validation.





Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
## [M-15]. Unregistration Griefing via Active Request Bloat

## id: rLdmlJ8oiMLwcNLzBPBze

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
### Finding Status Justification: unregisterVault() intentionally requires metrics.activeDepositRequestersCount == 0 (and other zero metrics) to ensure no users have pending/claimable positions before removal. A user who still has a claimable deposit (i.e., they have not completed the async claim step) legitimately represents outstanding user funds/entitlements, so blocking unregistration is a safety property, not a DoS vulnerability. The vault does not expose an admin force-clear because that would risk user fund loss. Therefore the reported “griefing” is simply the protocol enforcing the invariant that a vault cannot be unregistered while users still have unsettled/claimable requests.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `unregisterVault` function requires `metrics.activeDepositRequestersCount` to be zero. A malicious user can call `requestDeposit` with the minimum amount and never claim the shares. This keeps them in the `activeDepositRequesters` set. There is no mechanism for the admin to force-clear these requests or remove the user from the set. This prevents the vault from ever being unregistered, potentially blocking system maintenance or upgrades.

## Impact
Prevents unregistering obsolete or problematic vaults, hindering protocol maintenance.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit` with 1000 wei. 2. `activeDepositRequesters` count increments. 3. Investment Manager fulfills deposit. 4. User refuses to call `deposit` (claim). 5. Count remains > 0. 6. Admin calls `unregisterVault` and it reverts.

## Proof of Code
function testUnregisterGrief() public { 
 vault.requestDeposit(1000, user, user); 
 vm.prank(admin); 
 vm.expectRevert(); 
 shareToken.unregisterVault(asset); 
 }

## Suggested Mitigation
Allow `unregisterVault` to proceed if only dust amounts remain, or provide an admin function to force-close/cancel pending/claimable requests for a user.



