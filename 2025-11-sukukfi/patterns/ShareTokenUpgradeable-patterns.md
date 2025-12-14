## Verified Patterns Found: 25

## Verified Patterns Found in following Categories:

- UnsafeRecipient
- SlippageMissingOrInsufficient
- StandardViolation
- PrecisionDriftAccumulation
- Reentrancy
- AccessControlOrAuthByPass
- ReadOnlyReentrancy
- ReserveOrPriceDesync
- GriefableCallbacks
- PricePrecisionOrRoundingError
- GovernanceDelegationFlaw
- ForcedAssetVsStrictEquality
- FlashLoanEconomicManipulation
- AccountingInvariantViolation



## Summary of Patterns

Permanent DoS via Broken Vault Dependency

Rounding direction in withdraw allows free asset withdrawals

Unsafe Controller in Deposit/Redeem Requests

rBalance Corruption via Logic Divergence

Slippage Missing in Async Vault Operations

Multi-Asset Reserve Desync Arbitrage

Share Price Manipulation via Donation

Vault Unregistration Griefing via Dust

Share Price Manipulation via Donation (Spot Balance Dependency)

Double Allowance Consumption in WERC7575ShareToken.transferFrom

Governance DoS via Broken Vault Integration

ERC4626 Preview Functions Revert Breaking Standard Integrations

Unsafe UUPS Implementation Bypassing Verification

System-Wide DoS via Griefable Asset Callbacks

Dust donation prevents vault unregistration

Precision Drift Favoring User in mint (Claim Deposit)

Share Price Inflation via Insolvency Clamping in totalAssets

Precision Drift allowing Free Asset Extraction in withdraw

Phantom yield in rBalance causes Investment Layer insolvency

TotalSupply Invariant Violation via Unsafe Recipient in Batch Transfers

Share Price Dilution via Dust Shares Accumulation

Read-Only Reentrancy in Share Price via Asset Callbacks

Read-Only Reentrancy in `totalAssets` via `requestDeposit`

Slippage Missing in Investment Operations

Missing slippage protection in async deposit/redeem requests

## Patterns



 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Permanent DoS via Broken Vault Dependency
 ### Description/Code Snippet
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults to aggregate assets. It calls `vault.getClaimableSharesAndNormalizedAssets()`, which calls `vault.totalAssets()`, which calls `IERC20(asset).balanceOf(vault)`. If a single underlying asset reverts on `balanceOf` (e.g. paused token, malicious upgrade, or hacked adapter), the entire ShareToken accounting reverts. This bricks `convertNormalizedAssetsToShares` and `fulfillDeposit` for ALL vaults. Crucially, `unregisterVault` also performs the same `totalAssets()` check to verify the vault is empty, so the Owner cannot remove the broken vault to restore system functionality. The protocol becomes permanently frozen.
 ### Static Signals
loop over external calls, critical dependency on external token state, unregister function reverts if dependency fails, no try-catch block
 ### Assets at Risk
Availability of all funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Rounding direction in withdraw allows free asset withdrawals
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.sol`, the `withdraw` function calculates the shares to burn using `Math.Rounding.Floor`: `shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`. If `availableAssets > availableShares` (share price > 1 asset), a user can withdraw small amounts of assets such that `assets * availableShares < availableAssets`, resulting in `shares = 0`. The code executes `if (shares > 0) { ... burn ... }` but unconditionally transfers assets `SafeTokenTransfers.safeTransfer($.asset, receiver, assets)`. This allows users to drain assets without burning any shares. Similarly, `mint` uses Floor rounding for asset cost, favoring the user.
 ### Static Signals
Math.Rounding.Floor used for calculating costs/inputs, shares = assets.mulDiv(..., Floor)
 ### Assets at Risk
assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Unsafe Controller in Deposit/Redeem Requests
 ### Description/Code Snippet
The `requestDeposit` and `requestRedeem` functions in `ERC7575VaultUpgradeable` do not validate that the `controller` parameter is non-zero. If a user mistakenly passes `address(0)` as the controller (e.g., thinking it defaults to `msg.sender`), the assets or shares will be transferred to the vault but the resulting request record will be assigned to `address(0)`. Since no one can authenticate as `address(0)` or authorize an operator for it, these funds become permanently stuck in the pending or claimable state, leading to loss of funds.
 ### Static Signals
no zero-address guard, controller parameter used directly in mapping keys, authentication check validates owner but not controller
 ### Assets at Risk
user deposits, user shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GovernanceDelegationFlaw

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
rBalance Corruption via Logic Divergence
 ### Description/Code Snippet
The `rBatchTransfers` function relies on `consolidateTransfers` to aggregate accounts in a specific order that must exactly match the order used by the off-chain `computeRBalanceFlags` helper. The `rBalanceFlags` bitmask is applied to indices derived from this aggregation. While currently identical, the logic is duplicated across `consolidateTransfers` and `_computeRBalanceFlagsInternal`. Any future update or subtle deviation between these two complex aggregation algorithms will cause the bitmask to apply to the wrong accounts, silently corrupting the critical `_rBalances` ledger which tracks investor capital.
 ### Static Signals
logic duplication, complex bitmask mapping, critical state update depends on implicit ordering
 ### Assets at Risk
Investor capital tracking
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit, requestRedeem

 ### Title
Slippage Missing in Async Vault Operations
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` implements ERC-7540 asynchronous flows (`requestDeposit`, `requestRedeem`) where users lock assets/shares, but the exchange rate is determined later at fulfillment time by the Investment Manager. The request functions (`requestDeposit`, `requestRedeem`) lack `minShares` or `minAssets` parameters, and the fulfillment logic (`fulfillDeposit`, `fulfillRedeem`) calculates the payout using the spot rate at execution time without any user-defined bounds. This exposes users to unlimited slippage due to market movements or rate manipulation between the request and fulfillment transactions.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound, no minAmountOut parameter in liquidation/redemption
 ### Assets at Risk
assets, shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestRedeem

 ### Title
Multi-Asset Reserve Desync Arbitrage
 ### Description/Code Snippet
The system allows multiple vaults with different underlying assets (e.g., USDC, DAI) to mint the same fungible `ShareToken`, assuming a fixed 1:1 exchange rate (after decimal normalization). The protocol lacks an oracle to verify real-time market prices. If one underlying asset depegs (e.g., USDC drops to $0.90), the invariant `1 Share = 1 Normalized Unit` breaks. Users can deposit the depegged asset to mint shares at face value and redeem them for a pegged asset (e.g., DAI) from another vault, effectively draining the valuable reserves and diluting other share holders.
 ### Static Signals
assumes invariant without verifying, assumes reserve/price invariants that no longer hold
 ### Assets at Risk
vault reserves
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ShareTokenUpgradeable.convertNormalizedAssetsToShares

 ### Title
Share Price Manipulation via Donation
 ### Description/Code Snippet
The `ShareTokenUpgradeable` calculates the global share price based on `totalNormalizedAssets`, which includes the balance of `investmentShareToken` held by the contract. An attacker can donate `investmentShareToken` assets directly to the `ShareTokenUpgradeable` contract to artificially inflate `totalNormalizedAssets` and thus the share price (`assets/supply`). While a self-donation/redeem cycle is typically zero-sum for the attacker, this manipulation can be used to grief other users (e.g., inflating price right before a `fulfillDeposit` executes, causing the user to receive dust shares) or exploit external systems relying on the share price.
 ### Static Signals
share price derived from manipulable pool state, uses totalSupply/totalAssets in same tx as deposit/withdraw
 ### Assets at Risk
user deposits, share price integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Vault Unregistration Griefing via Dust
 ### Description/Code Snippet
The `unregisterVault` function enforces a strict check that `IERC20(asset).balanceOf(vaultAddress) == 0`. An attacker can send 1 wei of the asset to the vault (direct transfer), causing this check to fail. While the Investment Manager can theoretically move funds via `investAssets`, this requires the vault to be active and the Investment Vault to accept dust (which might fail due to min deposit limits). If the dust cannot be cleared, the vault cannot be unregistered, permanently occupying one of the limited slots (`MAX_VAULTS_PER_SHARE_TOKEN` = 10) and preventing vault rotation.
 ### Static Signals
strict equality check on balance, external user can manipulate balance, admin action blocked by user state
 ### Assets at Risk
Protocol slots, Management capability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Share Price Manipulation via Donation (Spot Balance Dependency)
 ### Description/Code Snippet
The `totalAssets` function in `ERC7575VaultUpgradeable` relies directly on `IERC20(asset).balanceOf(address(this))`. This allows an attacker to manipulate the exchange rate (share price) by donating assets to the vault to inflate `totalAssets`. Since `ShareTokenUpgradeable` aggregates these spot balances to calculate `totalNormalizedAssets` for conversion rates, any function relying on this rate (such as `fulfillDeposit`, `fulfillRedeem`, or external pricing) uses a manipulable spot price. This facilitates flash loan attacks or sandwich attacks against user deposits/redemptions.
 ### Static Signals
totalAssets uses balanceOf(this), no internal accounting balance, share price derived from spot balance
 ### Assets at Risk
user deposits, user redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.transferFrom

 ### Title
Double Allowance Consumption in WERC7575ShareToken.transferFrom
 ### Description/Code Snippet
The `transferFrom` function in `WERC7575ShareToken` explicitly calls `_spendAllowance(from, from, value)` to enforce self-allowance (permit), and then calls `super.transferFrom(from, to, value)`. The parent OpenZeppelin `ERC20.transferFrom` implementation calls `_spendAllowance(from, msg.sender, value)`. When a user calls `transferFrom` on themselves (`msg.sender == from`), `allowance[from][from]` is deducted twice: once in the override and once in the parent function. This creates an accounting invariant violation where a self-transfer costs double the allowance amount.
 ### Static Signals
_spendAllowance(from, from, value), super.transferFrom(from, to, value), msg.sender == from
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GovernanceDelegationFlaw

 ### Relevant Function/Location: ShareTokenUpgradeable.setInvestmentManager

 ### Title
Governance DoS via Broken Vault Integration
 ### Description/Code Snippet
The `setInvestmentManager` function in `ShareTokenUpgradeable` iterates through all registered vaults to update the investment manager. If a single vault enters a broken state where it reverts on this call (or is a malicious contract registered by a compromised owner), the global update function becomes permanently unusable. Furthermore, `unregisterVault` attempts to query `getVaultMetrics` and `balanceOf` on the vault before removal; if these calls revert, the vault cannot be unregistered. This creates a circular dependency where a broken vault cannot be fixed (via manager update) and cannot be removed, causing a permanent Denial of Service to critical governance functions.
 ### Static Signals
external call in loop, try/catch that reverts in catch, global configuration push
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.previewDeposit

 ### Title
ERC4626 Preview Functions Revert Breaking Standard Integrations
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` contract implements `IERC4626` but defines all preview functions (`previewDeposit`, `previewMint`, `previewWithdraw`, `previewRedeem`) to revert with `AsyncFlow()`. While this behavior complies with ERC-7540 for async vaults, it violates the ERC-4626 standard which mandates that preview functions simulate the effects of a transaction. Integrators and routers expecting standard ERC-4626 behavior will fail when interacting with this vault.
 ### Static Signals
revert AsyncFlow(), implementing IERC4626, preview function reverts
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ShareTokenUpgradeable.upgradeTo

 ### Title
Unsafe UUPS Implementation Bypassing Verification
 ### Description/Code Snippet
Both `ShareTokenUpgradeable` and `ERC7575VaultUpgradeable` inherit from `UUPSUpgradeable` but implement a custom `upgradeTo` function that calls `ERC1967Utils.upgradeToAndCall` directly. This bypasses the standard UUPS safety mechanism (normally enforced via `_authorizeUpgrade` and `UUPSUpgradeable`'s version of `upgradeTo`) which verifies that the new implementation contract supports UUPS upgrades (`proxiableUUID` check). Upgrading to a logic contract that lacks this UUID or the upgrade function will permanently brick the proxy, preventing future upgrades.
 ### Static Signals
ERC1967Utils.upgradeToAndCall used directly in public function, missing _authorizeUpgrade override usage, manual upgradeTo implementation in UUPS contract
 ### Assets at Risk
future upgrades
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
System-Wide DoS via Griefable Asset Callbacks
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract aggregates value across all registered vaults in `getCirculatingSupplyAndAssets` by looping through the `assetToVault` list. Inside the loop, it calls `getClaimableSharesAndNormalizedAssets` on each vault, which eventually calls `totalAssets()` and the underlying asset's `balanceOf()`. If a single registered asset reverts (e.g., PAUSED USDC, upgraded token, or malicious token), the entire loop fails. This causes `convertNormalizedAssetsToShares` to revert, which in turn causes `fulfillDeposit` to revert for ALL vaults. A single failing asset denies service to the entire multi-asset system.
 ### Static Signals
external call in loop without failure isolation, loops over user-controlled arrays/sets
 ### Assets at Risk
vault operations
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Dust donation prevents vault unregistration
 ### Description/Code Snippet
The `ShareTokenUpgradeable.unregisterVault` function strictly requires `IERC20(asset).balanceOf(vaultAddress) == 0`. An attacker can send a minimal amount (dust) of the asset to the vault address, causing this check to fail and preventing the owner from unregistering the vault. While the Investment Manager can technically sweep assets via `investAssets`, this creates a griefing vector where unregistration is blocked until specific cleanup actions are taken.
 ### Static Signals
require(address(this).balance == totalFees) or similar strict guard, assumes only contract code changes balances
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.mint

 ### Title
Precision Drift Favoring User in mint (Claim Deposit)
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.mint`, the asset calculation uses `Math.Rounding.Floor`: `assets = shares.mulDiv(availableAssets, availableShares, Math.Rounding.Floor)`. This calculates the assets to consume from the user's claimable bucket to mint a specific number of shares. Rounding down means the user consumes slightly fewer assets than the shares are worth proportionally. This systematically undervalues the assets required, favoring early claimers and leaving dust shares in the bucket with insufficient backing assets for the last claimer.
 ### Static Signals
assets = shares.mulDiv(..., Math.Rounding.Floor), assets calculated from shares using Floor
 ### Assets at Risk
claimableDepositAssets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Share Price Inflation via Insolvency Clamping in totalAssets
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.totalAssets()`, the function returns `balance > reservedAssets ? balance - reservedAssets : 0`. `reservedAssets` includes `totalClaimableRedeemAssets`. If the Investment Manager invests all liquid assets (so `balance` becomes 0) and then fulfills a redemption request (converting pending shares to `claimableRedeemAssets`), `reservedAssets` will exceed `balance`. The function will return 0 due to clamping. However, `ShareTokenUpgradeable` calculates the global share price by summing `totalAssets()` from all vaults (0) and adding `investedAssets` (full amount) while subtracting the redeemed shares from `circulatingSupply`. This effectively counts the assets twice (once in `investedAssets` and implicitly again by not deducting the liability from `totalAssets`), causing `totalNormalizedAssets` to be overstated relative to `circulatingSupply`, leading to an artificial spike in share price.
 ### Static Signals
return balance > reservedAssets ? balance - reservedAssets : 0
 ### Assets at Risk
vault shares
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Precision Drift allowing Free Asset Extraction in withdraw
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.withdraw`, the share calculation uses `Math.Rounding.Floor` when calculating the shares to burn for a requested asset amount: `shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`. If the user requests an asset amount small enough such that `assets * availableShares < availableAssets`, the result `shares` rounds to 0. The function does not check if calculated `shares` is 0 (unlike `deposit` or `mint` which have checks). It proceeds to transfer the assets and burn 0 shares (`if (shares > 0)`). This allows an attacker to drain their claimable assets without burning any shares by making multiple small withdrawals.
 ### Static Signals
shares = assets.mulDiv(..., Math.Rounding.Floor), if (shares > 0) burn, missing check shares != 0
 ### Assets at Risk
claimableRedeemAssets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Phantom yield in rBalance causes Investment Layer insolvency
 ### Description/Code Snippet
The `WERC7575ShareToken` implements an `adjustrBalance` function intended to inject yield/profit into the system by increasing `_rBalances`. The Investment Layer (`ShareTokenUpgradeable`) counts these `rBalance` increases as assets (`_calculateInvestmentAssets`). However, `adjustrBalance` does not mint or transfer actual liquid tokens to `_balances`. Since `WERC7575Vault` redemption burns from `_balances` (standard ERC20 behavior), the Investment Layer cannot redeem this phantom yield. When investors try to exit, the Investment Layer will be insolvent as it holds 'value' in `rBalance` that cannot be converted to underlying assets via redemption.
 ### Static Signals
accounting state not updated when underlying asset is swapped/upgraded, balance tracking references different token than actual holdings
 ### Assets at Risk
rewards, treasury
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.batchTransfers

 ### Title
TotalSupply Invariant Violation via Unsafe Recipient in Batch Transfers
 ### Description/Code Snippet
The `WERC7575ShareToken.batchTransfers` function directly modifies `_balances` based on input arrays without checking for `address(0)` or updating `_totalSupply` (it bypasses the `_update` override). If the validator passes `address(0)` as a creditor (mint effect) or debtor (burn effect), the `_balances[0]` changes but `_totalSupply` remains unchanged. This breaks the invariant `totalSupply == sum(balances)`. While ERC20 transfers usually block `address(0)`, this custom batch function does not validation on the addresses.
 ### Static Signals
_balances[account.owner] += amount, missing address(0) check, bypasses _update/totalSupply logic
 ### Assets at Risk
token accounting integrity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.deposit

 ### Title
Share Price Dilution via Dust Shares Accumulation
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.deposit` and `mint` (claim functions), when a user claims the full amount of `claimableDepositAssets`, the corresponding `claimableDepositShares` record is deleted. However, due to `Math.Rounding.Floor` in the share calculation, the calculated `shares` transferred to the user may be slightly less than the total `availableShares` held by the vault for that deposit. The difference (dust shares) remains in the vault's balance but is removed from the `claimableDepositShares` tracking. 

Since `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` calculates `circulatingSupply` by subtracting `totalClaimableRedeemShares` (but not deposit shares) from `totalSupply`, these dust shares are considered 'circulating'. However, the assets backing them have been fully claimed and removed from the vault. This results in circulating shares with zero backing assets, permanently diluting the share price. Repeated cycles of deposit and full claim will accumulate this drift.
 ### Static Signals
delete $.claimableDepositShares[controller], shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor), IERC20Metadata($.shareToken).transfer(receiver, shares)
 ### Assets at Risk
vault share price
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Read-Only Reentrancy in Share Price via Asset Callbacks
 ### Description/Code Snippet
The `requestDeposit` function in `ERC7575VaultUpgradeable.sol` calls `SafeTokenTransfers.safeTransferFrom`, which transfers assets *before* updating `pendingDepositAssets`. If the underlying asset supports transfer hooks (e.g., ERC-777 or ERC-677), an attacker can trigger code execution after the balance update but before the state update. During this window, `totalAssets()` (calculated as `balance - reserved`) will be artificially inflated because the balance has increased but `reserved` (pending deposits) has not yet. Since `ShareTokenUpgradeable`'s share price calculation relies on `totalAssets()` from all vaults, this inflated value allows the attacker to manipulate the global share price, potentially exploiting external protocols relying on this price or executing arbitrage.
 ### Static Signals
external call before view function stabilizes, view function reads balanceOf/totalSupply during external call
 ### Assets at Risk
share price integrity, external protocol collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Read-Only Reentrancy in `totalAssets` via `requestDeposit`
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.requestDeposit`, assets are transferred from the user using `safeTransferFrom` *before* the `pendingDepositAssets` state is updated. If the asset token allows reentrancy (e.g., ERC777 hooks), an attacker can read `totalAssets()` during the callback. `totalAssets()` calculates `balanceOf(this) - reservedAssets`. Since `balance` has increased but `reservedAssets` (which includes pending deposits) has not yet updated, `totalAssets()` momentarily inflates. This creates a manipulated share price in `convertToShares` and `convertToAssets` (which rely on `totalAssets` via the ShareToken), potentially exploiting external protocols or composable defi integrations relying on this vault as an oracle.
 ### Static Signals
balanceOf(this) used in state-dependent view, CEI violation, external call before state update
 ### Assets at Risk
external protocols
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Slippage Missing in Investment Operations
 ### Description/Code Snippet
The `investAssets` and `withdrawFromInvestment` functions in `ERC7575VaultUpgradeable.sol` interact with external investment vaults (`IERC7575`) without any user-specified or manager-specified slippage protection (minimum shares or minimum assets). If the external vault has a variable exchange rate (e.g., standard ERC-4626) and is manipulated (e.g., via sandwich attack or flash loan) or experiences high volatility, the protocol may receive significantly fewer shares or assets than expected, resulting in loss of value for the vault users.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Missing slippage protection in async deposit/redeem requests
 ### Description/Code Snippet
The functions `requestDeposit` and `requestRedeem` in `ERC7575VaultUpgradeable.sol` initiate an asynchronous flow where assets/shares are transferred to the vault, but the exchange rate is determined later upon fulfillment by the Investment Manager. These functions lack a `minShares` or `minAssets` parameter (slippage protection). If the vault's share price changes unfavorably (e.g., due to investment losses or manipulation) between the request and the fulfillment, users are forced to accept the unfavorable rate without recourse.
 ### Static Signals
requestDeposit returns requestId but takes no minOut, requestRedeem takes no minAssets, async flow without bounded execution price
 ### Assets at Risk
assets, shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

