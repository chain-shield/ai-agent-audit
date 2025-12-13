# 2025 11 sukukfi - Findings Report
## Commit hash: 18fe2578cf1c6203dac7ff21513533010f3dda3e

##Findings by Status


Finding Status: Valid


[H-1]. System-wide DoS via Single Vault Failure in ShareTokenUpgradeable
**Derived From** : Issue Type: UnboundedLoops
Finding Status: Valid
Privilege: Permissionless


[M-2]. Permit Front-Running DoS on Restricted Token Transfers in WERC7575ShareToken
**Derived From** : Issue Type: PermitFrontRun
Finding Status: Valid
Privilege: Permissionless


[H-3]. System-wide DoS via Single Vault Failure due to unregisterVault Logic
**Derived From** : GriefableCallbacks
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidNotExploitable + InvalidByDesign


[H-4]. rBatchTransfers Can Inflate Invested Asset Value
**Derived From** : Issue Type: AccountingInvariantViolation
Finding Status: InvalidNotExploitable + InvalidByDesign
Privilege: RequiresRole


[M-5]. WERC7575ShareToken permit does not support EIP-1271
**Derived From** : StandardViolation
Finding Status: InvalidNotExploitable + InvalidByDesign
Privilege: Permissionless



Finding Status: NeedsMoreInfo


[H-6]. Severe Value Loss in Mint due to Scaling Factor Rounding (WERC7575Vault)
**Derived From** : Issue Type: AccountingInvariantViolation
Finding Status: NeedsMoreInfo
Privilege: Permissionless


[H-7]. Rounding Error in Withdraw Leads to Vault Drainage
**Derived From** : Issue Type: PricePrecisionOrRoundingError
Finding Status: NeedsMoreInfo
Privilege: Permissionless


[H-8]. Flash Loan / Donation Attack via Spot Price Manipulation
**Derived From** : Issue Type: FlashLoanEconomicManipulation
Finding Status: NeedsMoreInfo
Privilege: Permissionless


[M-9]. Vault Unregistration Blocked by User State
**Derived From** : Issue Type: ForcedAssetVsStrictEquality
Finding Status: NeedsMoreInfo
Privilege: Permissionless



Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist


[H-10]. UUPS Upgradeability Brick due to Missing Proxiable Interface
**Derived From** : Issue Type: BeaconOrFactoryAuthorityDrift
Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
Privilege: RequiresAdminRole


[H-11]. Incorrect Share Calculation in fulfillDeposit Causes Value Loss
**Derived From** : Issue Type: ERC4626SharePriceMismatch
Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
Privilege: RequiresRole


[H-12]. Read-Only Reentrancy in `requestDeposit` allows Theft via Cross-Vault Arbitrage
**Derived From** : Issue Type: ReadOnlyReentrancy
Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
Privilege: Permissionless



Finding Status: LowSeverityDueToLowImpact


[L-13]. ShareTokenUpgradeable claims ERC20Permit support but logic is missing
**Derived From** : StandardViolation
Finding Status: LowSeverityDueToLowImpact
Privilege: Permissionless



Finding Status: InvalidUserErrorOrMistake


[M-14]. Unsafe Controller Recipient in Request Deposit leads to Stuck Funds
**Derived From** : Issue Type: UnsafeRecipient
Finding Status: InvalidUserErrorOrMistake
Privilege: Permissionless



Finding Status: InvalidByDesign


[M-15]. Missing Slippage Protection in Async Operations
**Derived From** : Issue Type: SlippageMissingOrInsufficient
Finding Status: InvalidByDesign
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 9
- M: 5
- L: 1
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. System-wide DoS via Single Vault Failure in ShareTokenUpgradeable

## id: h_QubIJtYfQIBTZpJAk1X

## Derived From Pattern/Invariant
Issue Type: UnboundedLoops

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.getCirculatingSupplyAndAssets

## Finding Status: Valid
### Finding Status Justification: getCirculatingSupplyAndAssets iterates all vaults and makes external calls without try/catch; any single vault revert bricks conversions across the system, and unregisterVault also depends on successful metrics/totalAssets so recovery is blocked.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults to aggregate assets. It calls `getClaimableSharesAndNormalizedAssets` on each vault, which calls `totalAssets()` -> `IERC20(asset).balanceOf(address(this))`. If a single underlying asset reverts (e.g. paused USDC) or a vault reverts, the loop fails. This function is critical for `convertNormalizedAssetsToShares`, used by *all* vaults for deposits/withdrawals. Thus, one broken vault freezes the entire multi-asset system. Furthermore, `unregisterVault` also calls `getVaultMetrics` -> `totalAssets`, so the broken vault cannot be removed.

## Impact
Permanent freeze of all funds in the entire protocol (all vaults). Recovery (unregistration) is also blocked.

## Command to Run Test


## Proof of Concept
1. A vault is registered for Token A (e.g. USDC).
2. Token A pauses (reverts on balanceOf) or upgrades to a broken implementation.
3. Any call to `ShareToken.convertNormalizedAssetsToShares` now reverts because it iterates all vaults including A.
4. Users cannot deposit/withdraw from Vault B (Token B) because it relies on ShareToken conversion.
5. Admin calls `unregisterVault(Token A)` to fix it.
6. `unregisterVault` calls `getVaultMetrics` -> `totalAssets` -> `TokenA.balanceOf` -> Revert.
7. System is permanently bricked.

## Proof of Code
function testSystemDos() public { 
  // Mock a broken asset 
  brokenAsset.setRevertBalanceOf(true); 
  // Try to interact with a healthy vault 
  vm.expectRevert(); 
  healthyVault.deposit(100, user); 
  // Try to unregister 
  vm.expectRevert(); 
  shareToken.unregisterVault(address(brokenAsset)); 
}

## Suggested Mitigation
Wrap external calls in the loop within a try/catch block. If a vault fails, skip it or treat its assets as 0 (for valuation) but do not revert. Also allow `unregisterVault` to force-remove a vault even if `getVaultMetrics` fails.


## [M-2]. Permit Front-Running DoS on Restricted Token Transfers in WERC7575ShareToken

## id: rcIa4miN4DVSIDhPDi3l1

## Derived From Pattern/Invariant
Issue Type: PermitFrontRun

## Exploit Type
Dos

## Location
WERC7575ShareToken.permit

## Finding Status: Valid
### Finding Status Justification: The permit implementation consumes nonces; a copied signature can be front-run to consume the nonce, causing the original tx’s permit step to revert. There is no built-in mitigation (e.g., idempotent permit-if-allowance-already-set).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `WERC7575ShareToken` requires self-allowance (via `permit`) for any `transfer`. Since `permit` signatures are broadcast in the mempool, an attacker can front-run the `permit` call with the same signature but higher gas. This consumes the nonce, causing the original user's batch transaction (e.g., `permit` followed by `transfer`) to revert due to nonce mismatch. This effectively denies service for transfers, which is critical in this permissioned settlement layer.

## Impact
Denial of Service for legitimate user transfers/settlements.

## Command to Run Test


## Proof of Concept
1. User signs a `permit` message to grant themselves allowance.
2. User submits a transaction batch: `[permit(...), transfer(...)]`.
3. Attacker sees the transaction in the mempool.
4. Attacker extracts the `permit` parameters and signature.
5. Attacker submits a `permit` transaction with the same parameters and higher gas price.
6. Attacker's tx is mined first, consuming the user's nonce.
7. User's tx reverts at the `permit` step (InvalidNonce), blocking the `transfer`.

## Proof of Code
function testPermitFrontRun() public { 
  // Setup 
  uint256 privateKey = 0xA11CE; 
  address owner = vm.addr(privateKey); 
  token.mint(owner, 100 ether); 
  
  // Generate signature 
  (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, keccak256(abi.encodePacked(...))); 
  
  // Attacker front-runs 
  token.permit(owner, owner, 100 ether, deadline, v, r, s); 
  
  // User tx fails 
  vm.expectRevert(); 
  token.permit(owner, owner, 100 ether, deadline, v, r, s); 
}

## Suggested Mitigation
In the `permit` function, check if the allowance is already set and the nonce matches; if so, return success instead of reverting. Alternatively, use a try/catch mechanism in the batch execution or submit transfers via a relayer that handles permit failures gracefully.


## [H-3]. System-wide DoS via Single Vault Failure due to unregisterVault Logic

## id: ZlC26VOMQ50Kl8V5EZ6sK

## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
### Finding Status Justification: A reverting vault (e.g., underlying token/vault view function revert) can break ShareTokenUpgradeable aggregation loops and also prevents unregisterVault because unregisterVault requires getVaultMetrics/totalAssets to succeed; the try/catch reverts instead of allowing force-removal.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` calculates `getCirculatingSupplyAndAssets` by iterating over all registered vaults and calling `getClaimableSharesAndNormalizedAssets`, which relies on `totalAssets()`. If a single vault becomes unresponsive (e.g., the underlying asset pauses transfers or `balanceOf`), this loop reverts, breaking share pricing and blocking `fulfillDeposit`/`fulfillRedeem` for ALL vaults. The administrator cannot resolve this by unregistering the faulty vault because `unregisterVault` calls `getVaultMetrics` (which calls `totalAssets`) inside a `try/catch` block that reverts with `CannotUnregisterActiveVault` upon failure. This creates a deadlock where a single broken vault permanently freezes the entire protocol.

## Impact
Permanent freezing of the entire protocol (deposits, redemptions, investments) if a single underlying asset fails or a vault reverts.

## Command to Run Test


## Proof of Concept
1. Deploy ShareToken and Register two vaults (Vault A, Vault B). 2. Vault A's underlying asset (e.g. USDC) pauses or blacklists Vault A, causing `balanceOf(VaultA)` to revert. 3. Calls to `ShareToken.getCirculatingSupplyAndAssets` now revert. 4. Admin attempts to call `unregisterVault(AssetA)`. 5. `unregisterVault` calls `getVaultMetrics`, which calls `totalAssets`, which reverts. 6. The `catch` block catches the revert but throws `CannotUnregisterActiveVault()`. 7. Admin is unable to remove Vault A; system remains frozen.

## Proof of Code
test_dos_unregister

## Suggested Mitigation
Modify `unregisterVault` to allow force-unregistration of broken vaults by bypassing the `getVaultMetrics` check if a specific flag is passed or if the call fails.





Finding Status: InvalidNotExploitable + InvalidByDesign
## [H-4]. rBatchTransfers Can Inflate Invested Asset Value

## id: frBU6eX3yax3Fgh-hvN6z

## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.rBatchTransfers

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: This is intentional design per Known Issues Section 7a. The rBalance truncation to 0 when insufficient is documented as 'silent truncation' for informational tracking. The finding describes rBalance as 'invested' tracking, but the actual balance (_balances) is always correct. The system maintains zero-sum invariant for actual balances. The rBalance is secondary informational tracking for revenue/yield, not critical for fund safety. User funds are never at risk as actual _balances are always correct. This is a design choice for institutional investment tracking, not a vulnerability.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `WERC7575ShareToken.rBatchTransfers`, if an account is credited (net creditor) and flagged for rBalance update, `_rBalances` is reduced. However, if `_rBalances` < `creditAmount`, it is truncated to 0 (`_rBalances[account] = 0`). The account receives the full credit in `_balances` (liquid), but the reduction in `_rBalances` (invested) is capped. This means the system converts 'Invested' tokens to 'Liquid' tokens without fully reducing the 'Invested' counter. This inflates the total system value (Liquid + Invested) and the `ShareTokenUpgradeable`'s calculated `investedAssets`, manipulating the share price upwards.

## Impact
Inflation of share price and accounting mismatch; creation of phantom value.

## Command to Run Test


## Proof of Concept
1. User A has 100 Invested (rBalance).
2. `rBatchTransfers`: User A receives 1000 credit (from User B who pays 1000 Liquid).
3. User A gets +1000 Liquid.
4. User A rBalance should reduce by 1000, but is capped at -100 (set to 0).
5. Net result: System Liquid +0 (A+1000, B-1000). System Invested -100 (should be -1000).
6. Total Assets = Liquid + Invested. Value increased by 900.

## Proof of Code
function testRBatchInflation() public { 
  // Setup scenario 
  token.rBatchTransfers(...); 
  // Check invariants 
  assertGt(token.totalAssets(), initialTotal); 
}

## Suggested Mitigation
Revert if `rBalance` is insufficient to cover the credit, or ensure the credit to liquid balance is limited to the available rBalance reduction.


## [M-5]. WERC7575ShareToken permit does not support EIP-1271

## id: sXUnRRLRrytQCNoe8BERP

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
WERC7575ShareToken.permit

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: This is by design per Known Issues Section 3 'Wallet Incompatibility'. The docs explicitly state 'Standard wallets will fail' and 'Which wallets fail: MetaMask, Trust Wallet, Ledger Live, Any wallet using standard ERC-20 UI'. Smart contract wallets (Safe/Gnosis) are a subset of this known incompatibility. The system is 'Intended for institutional use with custom interface' and 'NOT a Medium: Intended for institutional use with custom interface, No function of protocol impacted - wallets are not part of our protocol'. This is QA/Low at best per Known Issues categorization.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `WERC7575ShareToken.permit` function uses `ECDSA.recover` to validate signatures and explicitly checks `signer == owner` (or validator). It does not implement EIP-1271 `isValidSignature` checks. This prevents smart contract wallets (like Safe) from using the permit functionality, effectively blocking them from the system since transfers require self-allowance via permit.

## Impact
Denial of Service for smart contract wallets (multisigs) which are common for institutional users.

## Command to Run Test


## Proof of Concept
1. User uses a Gnosis Safe.
2. Tries to generate a signature for `permit`.
3. Safe produces EIP-1271 signature.
4. `permit` calls `ECDSA.recover`, which returns a signer address different from the Safe address.
5. `permit` reverts.

## Proof of Code
function testSmartWalletPermitFail() public {
    // Setup mock smart wallet
    // Try to call permit
    // Expect revert
}

## Suggested Mitigation
Add support for EIP-1271 by calling `isValidSignature` if `ECDSA.recover` fails or returns a mismatch.





Finding Status: NeedsMoreInfo
## [H-6]. Severe Value Loss in Mint due to Scaling Factor Rounding (WERC7575Vault)

## id: 9VX0vHezo-lzRq2hVb49h

## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation

## Exploit Type
RoundingError

## Location
WERC7575Vault.mint

## Finding Status: NeedsMoreInfo
### Finding Status Justification: The described dust-mint behavior does exist: previewMint uses Ceil, so minting < scalingFactor share-wei costs at least 1 asset-wei. There is no on-chain minimum-share constraint that prevents this; preview functions merely disclose the cost and are not a safeguard.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `WERC7575Vault.mint`, `assets` are calculated using `_convertToAssets` with `Ceil` rounding. The scaling factor is `10^(18-d)`. For USDC (d=6), factor is 10^12. If a user mints 1 share (1 wei), `assets = 1 * 1 / 1e12` (Ceil) = 1 asset (1 wei USDC). The user pays 1 USDC wei (worth 1e12 shares) but receives only 1 share. This is a 99.999...% loss. The rounding logic forces a full asset unit payment for dust shares.

## Impact
User loses almost 100% of value on small mints; breaks ERC4626 value exchange invariant.

## Command to Run Test


## Proof of Concept
1. User calls `mint(1)` (1 wei share).
2. `previewMint` calculates required assets: `ceil(1 / 1e12) = 1`.
3. User transfers 1 wei USDC.
4. User gets 1 wei Share.
5. 1 wei Share is worth 1e-12 USDC.
6. User paid 1 USDC wei for 1e-12 USDC wei value.

## Proof of Code
function testMintRounding() public { 
  vault.mint(1, user); 
  // User paid 1 asset 
  // User got 1 share 
  // Convert back 
  uint256 assets = vault.convertToAssets(1); 
  assertEq(assets, 0); // Worthless 
}

## Suggested Mitigation
Revisit scaling logic or disallow minting dust amounts below the scaling factor.


## [H-7]. Rounding Error in Withdraw Leads to Vault Drainage

## id: SkpYQEPH5InRnPYP3kGX1

## Derived From Pattern/Invariant
Issue Type: PricePrecisionOrRoundingError

## Exploit Type
RoundingError

## Location
ERC7575VaultUpgradeable.withdraw

## Finding Status: NeedsMoreInfo
### Finding Status Justification: In ERC7575VaultUpgradeable.withdraw(), shares are computed with Floor and there is no check shares>0 before transferring assets; for 18-decimal assets (scalingFactor=1) whenever share price > 1, small asset withdrawals can compute shares=0 and still transfer assets, enabling repeated draining.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.withdraw`, the shares to burn are calculated using `Math.Rounding.Floor`. `shares = assets * supply / totalAssets` (Floor). If `assets` is small enough such that `assets * supply < totalAssets`, `shares` becomes 0. The function transfers assets to the user but burns 0 shares. An attacker can loop this to drain the vault's assets completely without reducing their share balance.

## Impact
Theft of all vault assets by draining them in dust amounts.

## Command to Run Test


## Proof of Concept
1. Vault state: 1000 Assets, 1000 Shares.
2. Attacker calls `withdraw(1)`.
3. `shares = 1 * 1000 / 1000 = 1`. Correct.
4. Wait, assume 1000 Assets, 500 Shares (Price 2).
5. `withdraw(1)`.
6. `shares = 1 * 500 / 1000 = 0.5` -> Floor -> 0.
7. Attacker gets 1 asset, burns 0 shares.
8. Repeat 1000 times -> Drain vault.

## Proof of Code
function testWithdrawRounding() public { 
  // Setup price = 2 
  vault.deposit(1000, user); 
  // Donate to double price 
  token.transfer(vault, 1000); 
  // Withdraw 1 
  uint256 shares = vault.withdraw(1, user, user); 
  assertEq(shares, 0); 
}

## Suggested Mitigation
Use `Math.Rounding.Ceil` for calculating shares to burn in `withdraw`.


## [H-8]. Flash Loan / Donation Attack via Spot Price Manipulation

## id: z8E6vurmiyx5P_J3_n0Bw

## Derived From Pattern/Invariant
Issue Type: FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
ERC7575VaultUpgradeable.fulfillDeposit

## Finding Status: NeedsMoreInfo
### Finding Status Justification: Share pricing uses vault totalAssets() which directly depends on IERC20(asset).balanceOf(vault) minus reserved; direct token donations increase totalAssets and can reduce shares minted in fulfillDeposit. Virtual shares/assets only dampen tiny manipulations and do not prevent large donations; fulfillDeposit has no minShares-style protection.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `fulfillDeposit` function converts assets to shares using the current exchange rate (`totalAssets / totalSupply`). `ShareTokenUpgradeable` aggregates `totalAssets` from all vaults. An attacker can donate assets to any vault in the system to artificially inflate `totalAssets` (and `totalNormalizedAssets`). This spikes the share price. If a user has a pending deposit, the attacker can front-run the `fulfillDeposit` tx with a donation, causing the user to receive significantly fewer shares (slippage) due to the inflated price. The donated assets are distributed among all existing shareholders.

## Impact
Theft of value from depositors (massive slippage/haircut on deposit).

## Command to Run Test


## Proof of Concept
1. User requests deposit of 1000 USDC.
2. Investment Manager submits `fulfillDeposit`.
3. Attacker front-runs with a direct transfer of 1,000,000 USDC to the vault.
4. `totalAssets` spikes 1000x. Share price spikes 1000x.
5. `fulfillDeposit` executes: `shares = 1000 / (HighPrice)`.
6. User gets 0.001x expected shares.
7. Attacker (holding existing shares) benefits from the donation + user's diluted entry.

## Proof of Code
function testDonationAttack() public { 
  // Setup: Attacker has shares 
  // User requests deposit 
  vault.requestDeposit(1000, user, user); 
  // Attacker donates 
  token.transfer(address(vault), 100000); 
  // Fulfill 
  vm.prank(manager); 
  uint256 shares = vault.fulfillDeposit(user, 1000); 
  // Assert shares are tiny 
  assertLt(shares, expectedShares / 100); 
}

## Suggested Mitigation
Implement internal balance tracking (do not rely on `balanceOf` for price) OR enforce strict slippage protection (`minShares`) in `fulfillDeposit`.


## [M-9]. Vault Unregistration Blocked by User State

## id: zKsibdMDDI52e3x-F7igU

## Derived From Pattern/Invariant
Issue Type: ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: NeedsMoreInfo
### Finding Status Justification: This is not primarily admin/privileged-user mistake; any permissionless user can create/leave pending or unclaimed requests to keep active*RequestersCount nonzero and thereby block unregisterVault.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`ShareTokenUpgradeable.unregisterVault` enforces `activeDepositRequestersCount == 0`. A malicious user can make a tiny deposit request and leave it pending (or fulfilled but unclaimed). This increments the counter. The admin cannot force the user to claim or cancel. This permanently blocks the unregistration of the vault, preventing system maintenance or removal of faulty assets.

## Impact
DoS of governance/maintenance (cannot remove vaults).

## Command to Run Test


## Proof of Concept
1. Attacker calls `requestDeposit(dust)`. 
2. `activeDepositRequestersCount` becomes > 0.
3. Admin calls `unregisterVault`.
4. Reverts due to count check.

## Proof of Code
function testBlockUnregister() public { 
  vault.requestDeposit(1, user, user); 
  vm.prank(owner); 
  vm.expectRevert(); 
  shareToken.unregisterVault(asset); 
}

## Suggested Mitigation
Allow admin to force-clear requests or remove vaults regardless of active user count in emergency.





Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
## [H-10]. UUPS Upgradeability Brick due to Missing Proxiable Interface

## id: X7jyq43MdqCQWG3wrvdjB

## Derived From Pattern/Invariant
Issue Type: BeaconOrFactoryAuthorityDrift

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
ERC7575VaultUpgradeable.upgradeTo

## Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
### Finding Status Justification: 
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
`ERC7575VaultUpgradeable` uses `ERC1967Utils.upgradeToAndCall` for upgrades but does not inherit `UUPSUpgradeable` or implement `proxiableUUID`. The standard `ERC1967Utils` upgrade flow checks the *new* implementation for `proxiableUUID` to ensure safety. Since the current contract code (which would likely be the basis for V2) lacks this function, any upgrade to a similar contract will revert, effectively bricking the upgradeability.

## Impact
Contract cannot be upgraded; upgrade functionality is permanently broken.

## Command to Run Test


## Proof of Concept
1. Deploy `ERC7575VaultUpgradeable` (V1).
2. Create V2 (inheriting V1 logic) and deploy.
3. Call `V1.upgradeTo(V2)`.
4. `ERC1967Utils` calls `V2.proxiableUUID()`.
5. Call reverts (function signature not found).
6. Upgrade fails.

## Proof of Code
function testUpgradeFail() public { 
  // Deploy V1 
  // Deploy V2 (copy of V1) 
  vm.expectRevert(); 
  v1.upgradeTo(address(v2)); 
}

## Suggested Mitigation
Inherit `UUPSUpgradeable` in `ERC7575VaultUpgradeable` to implement the required interface.


## [H-11]. Incorrect Share Calculation in fulfillDeposit Causes Value Loss

## id: SmQZmJ3uQWdZHhtjoke5s

## Derived From Pattern/Invariant
Issue Type: ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
ERC7575VaultUpgradeable.fulfillDeposit

## Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
### Finding Status Justification: 
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `fulfillDeposit`, the code decrements `pendingDepositAssets` *before* calling `_convertToShares`. Decrementing pending assets reduces `reservedAssets`, which immediately increases `totalAssets()` (the net available assets). The share calculation then uses this *increased* `totalAssets` (which includes the new deposit) against the *old* share supply (since shares are minted afterwards). This results in an artificially high share price for the conversion. The depositor receives fewer shares than they should, effectively donating value to existing shareholders immediately upon entry.

## Impact
Immediate loss of value for every depositor; value transferred to existing shareholders.

## Command to Run Test


## Proof of Concept
1. Vault has 100 Assets, 100 Shares. Price = 1.
2. User pending deposit: 100 Assets.
3. `fulfillDeposit`: `pending -= 100`. Now `totalAssets` = 100 (old) + 100 (new) = 200.
4. `_convertToShares(100)` uses Price = 200 / 100 = 2.
5. Shares = 100 / 2 = 50.
6. User gets 50 shares for 100 assets. Should have got 100.
7. Final state: 200 Assets, 150 Shares. Price = 1.33.
8. User lost 33% value instantly.

## Proof of Code
function testShareMispricing() public { 
  // Setup vault with 100 assets, 100 shares 
  vault.requestDeposit(100, user, user); 
  vm.prank(manager); 
  uint256 shares = vault.fulfillDeposit(user, 100); 
  // Expected 100 shares (1:1), but gets 50 
  assertEq(shares, 50); 
}

## Suggested Mitigation
Calculate shares *before* updating `pendingDepositAssets` (or ensure `totalAssets` used for conversion excludes the current deposit).


## [H-12]. Read-Only Reentrancy in `requestDeposit` allows Theft via Cross-Vault Arbitrage

## id: kpA9o_xpQCYXS-wmBFmDa

## Derived From Pattern/Invariant
Issue Type: ReadOnlyReentrancy

## Exploit Type
Reentrancy

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: InvalidSafeGuardInPlace + InvalidBugDoesNotExist
### Finding Status Justification: 
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`requestDeposit` calls `safeTransferFrom` (pull assets) *before* updating `pendingDepositAssets`. `totalAssets()` is calculated as `balanceOf(this) - reservedAssets` (where reserved includes pending). During the transfer callback (e.g. ERC777), `balanceOf` has increased but `reservedAssets` has NOT. This causes `totalAssets()` to be temporarily inflated. Since all vaults share the same `ShareToken` pricing, this inflates the global share price. An attacker can exploit this by calling `redeem` on a *second* vault (sharing the ShareToken) during the callback. `redeem` burns shares and pays out assets based on the inflated price (`assets = shares * InflatedPrice`), allowing the attacker to drain the second vault.

## Impact
Theft of assets from connected vaults via price manipulation.

## Command to Run Test


## Proof of Concept
1. Attacker deploys contract receiving ERC777 hook.
2. Attacker calls `requestDeposit` on Vault A.
3. In hook: `VaultA.balance` is up, `reserved` is not. `ShareToken.price` is inflated.
4. In hook: Attacker calls `redeem` on Vault B (which holds Asset B).
5. `redeem` calculates `assets = shares * Price`.
6. Since Price is high, Attacker gets more Asset B for their shares.
7. Attacker drains Vault B.

## Proof of Code
function testReadOnlyReentrancy() public { 
  // Setup Vault A (ERC777) and Vault B (USDC) 
  // Attacker holds shares 
  // Attacker calls requestDeposit(VaultA) 
  // Inside callback: withdraw(VaultB) 
  // Assert Attacker got excess funds 
}

## Suggested Mitigation
Update state variables (`pendingDepositAssets`) *before* transferring assets (CEI pattern).





Finding Status: LowSeverityDueToLowImpact
## [L-13]. ShareTokenUpgradeable claims ERC20Permit support but logic is missing

## id: XQn1BZHArf6vTZi6QfXU_

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
ShareTokenUpgradeable.permit

## Finding Status: LowSeverityDueToLowImpact
### Finding Status Justification: 
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` contract inherits `IERC20Permit` via `IERC7575ShareExtended` and context suggests it should support permit. However, the contract does not inherit `ERC20PermitUpgradeable` nor implement the `permit` function. Integrators relying on the interface description will fail when attempting to use signatures for approvals.

## Impact
Broken integration for systems expecting ERC20Permit compliance as advertised by interfaces.

## Command to Run Test


## Proof of Concept
Attempt to call `permit(...)` on the `ShareTokenUpgradeable` contract. It will revert with fallback/does not exist.

## Proof of Code
function testMissingPermit() public { 
  // Setup 
  vm.expectRevert(); 
  shareToken.permit(user, spender, 100, 0, 0, bytes32(0), bytes32(0)); 
}

## Suggested Mitigation
Inherit `ERC20PermitUpgradeable` and initialize it in the `initialize` function.





Finding Status: InvalidUserErrorOrMistake
## [M-14]. Unsafe Controller Recipient in Request Deposit leads to Stuck Funds

## id: gbCWt9X731KeLQAyI4FCS

## Derived From Pattern/Invariant
Issue Type: UnsafeRecipient

## Exploit Type
UncheckedReturn

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: InvalidUserErrorOrMistake
### Finding Status Justification: 
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestDeposit` function in `ERC7575VaultUpgradeable` allows the `controller` parameter to be `address(0)`. Assets are transferred to the vault and credited to `pendingDepositAssets[address(0)]`. These can be fulfilled into shares for `address(0)`, but can never be claimed because `deposit()`/`mint()` require `msg.sender` to be the controller (impossible for `address(0)`) or an approved operator (impossible to set for `address(0)`).

## Impact
Permanent loss of user funds if they mistakenly pass address(0) as controller.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit(1000, address(0), user)`.
2. 1000 assets transferred to vault.
3. `pendingDepositAssets[address(0)]` = 1000.
4. Investment Manager fulfills deposit -> `claimableDepositShares[address(0)]` = X.
5. No one can call `deposit()` for `address(0)` because `msg.sender != address(0)` and no operator can be set.

## Proof of Code
function testUnsafeRecipient() public { 
  vault.requestDeposit(100, address(0), user); 
  // Assert funds are in pending 
  assertEq(vault.pendingDepositRequest(0, address(0)), 100); 
  // Cannot claim 
  vm.prank(address(0)); 
  vm.expectRevert(); 
  vault.deposit(100, address(0), address(0)); 
}

## Suggested Mitigation
Add `if (controller == address(0)) revert InvalidController();` in `requestDeposit`.





Finding Status: InvalidByDesign
## [M-15]. Missing Slippage Protection in Async Operations

## id: BXev-0O9wScQCb-5CSPKL

## Derived From Pattern/Invariant
Issue Type: SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: InvalidByDesign
### Finding Status Justification: Even if no slippage bounds are an intended UX/design choice, it remains exploitable in practice because share price can be manipulated between request and fulfill (e.g., via asset donations affecting totalAssets), causing users to receive materially fewer shares/assets with no on-chain protection.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`requestDeposit`, `requestRedeem`, `fulfillDeposit`, and `fulfillRedeem` all lack parameters for `minShares` or `minAssets`. Users requesting deposits or redemptions are exposed to unrestricted price slippage between the request time and the fulfillment time. Given the potential for price manipulation (flash loan attacks) or natural volatility, users can suffer significant loss of value.

## Impact
Loss of user value due to slippage.

## Command to Run Test


## Proof of Concept
1. User requests deposit expecting Price 1.
2. Market moves or manipulation occurs -> Price 2.
3. Fulfillment happens.
4. User gets half the shares expected.

## Proof of Code
function testNoSlippage() public { 
  // No param in function signature 
}

## Suggested Mitigation
Add `minShares`/`minAssets` output limits to request and fulfill functions.



