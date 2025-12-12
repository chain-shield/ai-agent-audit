## Verified Patterns Found: 23

## Verified Patterns Found in following Categories:

- PricePrecisionOrRoundingError
- ERC4626SharePriceMismatch
- PrecisionDriftAccumulation
- PermitFrontRun
- StandardViolation
- SlippageMissingOrInsufficient
- BeaconOrFactoryAuthorityDrift
- ForcedAssetVsStrictEquality
- UnsafeRecipient
- GriefableCallbacks
- AccountingInvariantViolation
- ReadOnlyReentrancy
- EIP1271ByPass
- UnboundedLoops
- FlashLoanEconomicManipulation



## Summary of Patterns

Permit Front-Running DoS on Restricted Token Transfers

Denial of Service via Unbounded Loop in Share Pricing

Unsafe Controller Recipient in Request Deposit

ShareTokenUpgradeable claims ERC20Permit support but logic is missing

Flash Loan Economic Manipulation via Donation

SlippageMissingOrInsufficient in Async Deposit/Redeem

Read-Only Reentrancy in `requestDeposit` via `totalAssets` Inflation

Incorrect Share Calculation in fulfillDeposit Causes Value Loss

UUPS Upgradeability Brick due to Missing Proxiable Interface

WERC7575ShareToken permit does not support EIP-1271

Missing Slippage Protection in Async Redeem Fulfillment

Severe Value Loss in Mint due to Scaling Factor Rounding

Unsafe Recipient in Cancelation Claims

Vault Unregistration Blocked by User State (Strict Equality)

rBatchTransfers Can Inflate Invested Asset Value

System-wide DoS via Single Vault Failure (Lack of Fault Isolation)

Rounding Error in Withdraw and Mint Leads to Insolvency

Non-Standard ERC-20 Implementation violates Transfer and Approve Invariants

Rounding direction mismatch in ERC7575VaultUpgradeable mint/withdraw allows share dilution

Unsafe asset transfer to zero address in redeem function

ERC7575VaultUpgradeable totalAssets excludes claimable deposit assets

Missing Slippage Protection for Async Deposits

Missing Slippage Protection in Async Deposit Fulfillment

## Patterns



 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: WERC7575ShareToken.permit

 ### Title
Permit Front-Running DoS on Restricted Token Transfers
 ### Description/Code Snippet
The `WERC7575ShareToken` relies on `permit` to grant self-allowance (`owner` to `owner`) which is a prerequisite for any `transfer`. Since `permit` signatures are broadcast in the mempool (or via Multicall), an attacker can front-run the `permit` call with the same signature but higher gas. This consumes the nonce, causing the original user's batch transaction (e.g., `permit` followed by `transfer` in a Multicall) to revert due to nonce mismatch, effectively denying service for transfers.
 ### Static Signals
permit function public, nonces used, signature revealed in mempool
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Denial of Service via Unbounded Loop in Share Pricing
 ### Description/Code Snippet
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults to aggregate assets and supply. It calls `getClaimableSharesAndNormalizedAssets` on each vault, which performs external calls (`totalAssets` -> `balanceOf`). If a single vault becomes unresponsive (e.g., asset contract pauses or reverts), the loop will revert, freezing the entire Share Token system's pricing and conversion logic.
 ### Static Signals
for (uint256 i = 0; i < length; i++), external call in loop
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Unsafe Controller Recipient in Request Deposit
 ### Description/Code Snippet
The `requestDeposit` function allows the `controller` parameter to be `address(0)` without validation. If a user inadvertently sets the controller to the zero address, the assets are transferred to the vault and added to `pendingDepositAssets[address(0)]`. These assets can be fulfilled into shares, but they can never be claimed via `deposit()` or `mint()` because those functions require `msg.sender` to be the controller (which cannot be `address(0)`) or an approved operator (which cannot be set for `address(0)`). This leads to permanent loss of funds.
 ### Static Signals
no check for controller != address(0), deposit/mint require msg.sender == controller or isOperator
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ShareTokenUpgradeable.permit

 ### Title
ShareTokenUpgradeable claims ERC20Permit support but logic is missing
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract inherits `IERC20Permit` (via `IERC7575ShareExtended` or docs description) and is expected to support permit functionality for the investment layer. However, the contract `ShareTokenUpgradeable` does not inherit `ERC20PermitUpgradeable` nor `EIP712Upgradeable`, and does not implement the `permit` function. This results in a missing feature that is claimed in documentation and interfaces, potentially breaking integrations that rely on gasless approvals.
 ### Static Signals
Missing implementation of permit, Does not inherit ERC20PermitUpgradeable, Contract docs claim IERC20Permit compliance
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillDeposit

 ### Title
Flash Loan Economic Manipulation via Donation
 ### Description/Code Snippet
The `fulfillDeposit` function calculates shares based on the vault's `totalAssets()`, which relies on a spot `balanceOf` reading. An attacker can donate assets to the vault (or any vault in the system) to artificially inflate `totalAssets`, causing `totalNormalizedAssets` to spike. This decreases the share price calculated in `convertNormalizedAssetsToShares`, resulting in the depositor receiving significantly fewer shares than expected. There is no slippage protection (`minShares`) in `fulfillDeposit`.
 ### Static Signals
branches on pool.balanceOf(), share price derived from manipulable pool state, no multi-block observation or TWAP
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit / requestRedeem

 ### Title
SlippageMissingOrInsufficient in Async Deposit/Redeem
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` implements an asynchronous deposit/redeem flow (ERC7540) where users lock assets/shares via `requestDeposit`/`requestRedeem`, but the conversion rate is determined later during `fulfillDeposit`/`fulfillRedeem` executed by the Investment Manager. The request functions (`requestDeposit`, `requestRedeem`) lack `minShares` or `minAssets` parameters, and the fulfillment functions use the spot rate at execution time. This exposes users to unrestricted price slippage between the time of request and fulfillment, which could be significant if the vault's share price changes (e.g., due to losses, large yield events, or manipulation) while requests are pending.
 ### Static Signals
requestDeposit returns requestId without minShares, requestRedeem returns requestId without minAssets, fulfillDeposit uses current exchange rate
 ### Assets at Risk
user deposits, user redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Read-Only Reentrancy in `requestDeposit` via `totalAssets` Inflation
 ### Description/Code Snippet
The `requestDeposit` function transfers assets from the user using `SafeTokenTransfers.safeTransferFrom` before updating the `pendingDepositAssets` and `totalPendingDepositAssets` state variables. `totalAssets()` calculates the vault's value as `balanceOf(asset) - reservedAssets` (where `reservedAssets` includes `totalPendingDepositAssets`). 

During the asset transfer (if the asset has callbacks like ERC777/ERC677), the vault's balance increases but `reservedAssets` has not yet increased. This causes `totalAssets()` to return an inflated value. Since `ShareToken` relies on `totalAssets()` for price calculation (via `convertNormalizedAssetsToShares`), the share price is temporarily inflated during the callback. An attacker can exploit this if the `ShareToken` is used as an oracle or collateral in other protocols.
 ### Static Signals
untrusted call before all updates, view function reads balanceOf/totalSupply during external call, no reentrancy guard on state-reading functions
 ### Assets at Risk
vault share price integrity, external protocol integrations
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillDeposit

 ### Title
Incorrect Share Calculation in fulfillDeposit Causes Value Loss
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.fulfillDeposit`, `totalPendingDepositAssets` is decremented before `_convertToShares` is called. Decrementing pending assets reduces `reservedAssets`, which immediately increases the vault's `totalAssets()` (net available). The share calculation then uses this increased `totalAssets` value against the old share supply (since shares are minted afterwards), resulting in an artificially high share price. This causes the depositor to receive fewer shares than entitled, effectively donating value to existing shareholders.
 ### Static Signals
state update before conversion, totalPendingDepositAssets -= assets, _convertToShares(assets)
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: ERC7575VaultUpgradeable.upgradeTo

 ### Title
UUPS Upgradeability Brick due to Missing Proxiable Interface
 ### Description/Code Snippet
The contracts `ERC7575VaultUpgradeable` and `ShareTokenUpgradeable` implement UUPS-style upgrades via `ERC1967Utils.upgradeToAndCall` but do not inherit `UUPSUpgradeable` or implement `IERC1822Proxiable` (specifically `proxiableUUID`). When `upgradeToAndCall` is invoked, it validates the new implementation by calling `proxiableUUID()`. If the new implementation is a patched version of the current code (which lacks this function), the upgrade will revert, effectively bricking the upgradeability of the system.
 ### Static Signals
upgradeTo called without UUPSUpgradeable inheritance, ERC1967Utils used manually in upgradeTo, missing proxiableUUID
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: EIP1271ByPass

 ### Relevant Function/Location: WERC7575ShareToken.permit

 ### Title
WERC7575ShareToken permit does not support EIP-1271
 ### Description/Code Snippet
The `WERC7575ShareToken` contract implements `permit` using `ECDSA.recover` and strictly enforces `signer == owner`. This prevents smart contract wallets (like Gnosis Safe or other multisigs) from using the permit functionality, as they cannot produce an EOA signature that recovers to their address. EIP-1271 `isValidSignature` checks are missing.
 ### Static Signals
ECDSA.recover used without isValidSignature check, signer != owner revert
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillRedeem

 ### Title
Missing Slippage Protection in Async Redeem Fulfillment
 ### Description/Code Snippet
The `fulfillRedeem` function converts pending shares to assets using the current share price via `_convertToAssets`. There is no parameter for `minAssets` to enforce slippage protection. Users who request a redemption are exposed to price fluctuations (e.g., investment losses) between the request and fulfillment. Users receive the asset value at the time of fulfillment, which may be lower than anticipated.
 ### Static Signals
amountOutMin=0 or missing, price fetched at execution without user-specified floor, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds (value loss due to slippage)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575Vault.mint

 ### Title
Severe Value Loss in Mint due to Scaling Factor Rounding
 ### Description/Code Snippet
In `WERC7575Vault.mint`, the function calculates required assets using `previewMint` (which rounds up) and then mints the *exact* requested shares. If the asset has fewer decimals than the share token (e.g. USDC 6 vs 18), the scaling factor is large (10^12). Requesting 1 share (wei) requires 1 unit of asset (wei) due to rounding. However, 1 unit of asset is worth 10^12 shares. The user pays 1 asset and receives only 1 share, losing 99.99% of the deposit value. This violates ERC4626 value exchange expectations.
 ### Static Signals
preview/view functions modify state (breaks simulations), wrong totalSupply/balance invariants
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.claimCancelDepositRequest

 ### Title
Unsafe Recipient in Cancelation Claims
 ### Description/Code Snippet
The functions `claimCancelDepositRequest` and `claimCancelRedeemRequest` transfer assets or shares to a `receiver` address provided by the caller. There is no explicit check to ensure `receiver` is not `address(0)`. While standard ERC20 implementations revert on transfer to zero address, non-standard tokens (or future upgrades to the asset) might allow it, leading to permanent loss of funds.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Vault Unregistration Blocked by User State (Strict Equality)
 ### Description/Code Snippet
The `unregisterVault` function in `ShareTokenUpgradeable` enforces strict equality checks on vault state before allowing unregistration. Specifically, it requires `metrics.activeDepositRequestersCount == 0`, `metrics.totalPendingDepositAssets == 0`, and `IERC20(asset).balanceOf(vaultAddress) == 0`. A single malicious user can permanently block the unregistration of a vault by maintaining a small 'dust' deposit request (pending or fulfilled-but-unclaimed). Since the admin cannot force a user to claim or cancel their request, the vault cannot be removed from the system, creating a griefing vector against protocol governance.
 ### Static Signals
require(metrics.activeDepositRequestersCount == 0), require(balanceOf(vault) == 0)
 ### Assets at Risk
protocol governance
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
rBatchTransfers Can Inflate Invested Asset Value
 ### Description/Code Snippet
In `WERC7575ShareToken.rBatchTransfers`, if a creditor's `rBalance` (restricted/invested balance) is less than the credit amount being received, `rBalance` is silently truncated to 0, while the full credit amount is added to their liquid `_balances`. This causes the aggregate sum of `_balances + _rBalances` to increase for the system. Since `ShareTokenUpgradeable` calculates `investedAssets` based on this sum, this mechanism allows the validator to artificially inflate the total value of the investment layer, potentially manipulating share prices.
 ### Static Signals
rBalance truncation, sum(balances) invariant violation
 ### Assets at Risk
investment valuation
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
System-wide DoS via Single Vault Failure (Lack of Fault Isolation)
 ### Description/Code Snippet
The `ShareTokenUpgradeable` aggregates state from all registered vaults to calculate the exchange rate in `getCirculatingSupplyAndAssets`. It iterates over all vaults, calling `getClaimableSharesAndNormalizedAssets`, which in turn calls `totalAssets()` on the vault. `totalAssets()` relies on `IERC20(asset).balanceOf(address(this))`. 

If a single underlying asset reverts `balanceOf` (e.g., USDC is paused, or a token upgrade breaks compatibility), or if a vault reverts for any other reason, the loop in `getCirculatingSupplyAndAssets` fails. This causes `convertNormalizedAssetsToShares` to revert, blocking `fulfillDeposit` and `fulfillRedeem` for **all** vaults in the system, not just the faulty one.

Crucially, the recovery mechanism `unregisterVault` is also blocked. It attempts to call `getVaultMetrics` (which calls `totalAssets`) to verify safety. The `try/catch` block in `unregisterVault` catches the revert but throws `CannotUnregisterActiveVault`, mistakenly treating the revert as an active state. This creates a permanent deadlock where the Owner cannot remove the broken vault, and the entire protocol remains frozen.
 ### Static Signals
external call in loop without failure isolation, try/catch around external hook, loop over all registered vaults
 ### Assets at Risk
All funds in the system (frozen)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Rounding Error in Withdraw and Mint Leads to Insolvency
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, the `withdraw` function calculates `shares` to burn using `Math.Rounding.Floor` instead of `Ceil`. Similarly, the `mint` function calculates `assets` to consume using `Floor` instead of `Ceil`. This incorrect rounding favors the user, allowing them to withdraw more assets than their shares are worth or mint shares for fewer assets than required, draining the vault.
 ### Static Signals
Math.Rounding.Floor used in withdraw share calculation, Math.Rounding.Floor used in mint asset calculation
 ### Assets at Risk
assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.transfer, transferFrom, approve

 ### Title
Non-Standard ERC-20 Implementation violates Transfer and Approve Invariants
 ### Description/Code Snippet
The WERC7575ShareToken explicitly violates the ERC-20 standard by restricting `transfer` and `transferFrom` operations to require a validator-signed permit (self-allowance) and blocking `approve(self)`. While intentional for compliance, this breaks composability with standard DeFi protocols (DEXs, Lending) and wallets that expect standard ERC-20 behavior.
 ### Static Signals
revert ERC20InvalidSpender, _spendAllowance(msg.sender, msg.sender, value), if (!isKycVerified[to]) revert KycRequired
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Rounding direction mismatch in ERC7575VaultUpgradeable mint/withdraw allows share dilution
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, the `withdraw` and `mint` functions calculate the input amount (shares to burn or assets to consume) using `Math.Rounding.Floor`. Standard ERC4626 implementation requires inputs to be rounded UP (Ceil) to prevent dust exploitation. 

In `withdraw(assets)`, the shares to burn are calculated as `assets * (shares/assets)` rounded DOWN. If the ratio allows, a user can withdraw 1 wei of assets for 0 shares (if `shares < 1`). By repeating this in a loop, a user can drain their entire `claimableRedeemAssets` balance while burning zero or very few shares. 

Since the shares held by the vault (pending redemption) are not burned, the global `totalSupply` remains artificially high while assets leave the system. This dilutes the share price `(TotalAssets / TotalSupply)` for all other shareholders, effectively stealing value from the collective.
 ### Static Signals
Math.Rounding.Floor used in input calculation, withdraw calls mulDiv with Floor, mint calls mulDiv with Floor
 ### Assets at Risk
shareholder value, vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.redeem

 ### Title
Unsafe asset transfer to zero address in redeem function
 ### Description/Code Snippet
The `redeem` function in `ERC7575VaultUpgradeable` transfers assets to the specified `receiver` address using `SafeTokenTransfers.safeTransfer`, but does not validate that `receiver` is non-zero. If the underlying asset token contract does not revert on transfers to `address(0)` (e.g. some ERC20 implementations or if it handles it as a burn), a user accidentally passing the zero address will permanently lose their redeemed assets.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
user redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
ERC7575VaultUpgradeable totalAssets excludes claimable deposit assets
 ### Description/Code Snippet
The `totalAssets()` function in `ERC7575VaultUpgradeable` subtracts `totalPendingDepositAssets`, `totalClaimableRedeemAssets`, and `totalCancelDepositAssets`, but fails to subtract assets corresponding to `claimableDepositShares` (assets that have been fulfilled but not yet claimed by users). While shares have been minted to the vault for these deposits, counting the backing assets as 'available' in `totalAssets` while the corresponding shares are also part of the total supply (held by the vault) creates an accounting inconsistency where assets are double-counted as both 'backing new shares' and 'available for investment' or 'backing existing shares'.
 ### Static Signals
totalAssets logic excludes some reserved assets but not all, claimableDepositAssets not subtracted
 ### Assets at Risk
Vault Assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Missing Slippage Protection for Async Deposits
 ### Description/Code Snippet
The `requestDeposit` function initiates an asynchronous deposit but does not allow the user to specify a minimum amount of shares (`minShares`). Since fulfillment occurs in a separate transaction by an Investment Manager and the share price depends on the global state of the `ShareTokenUpgradeable` (which aggregates multiple vaults), the exchange rate can fluctuate unfavorably or be manipulated between request and fulfillment. Users are exposed to unlimited slippage.
 ### Static Signals
no minOut parameter, async fulfillment
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillDeposit

 ### Title
Missing Slippage Protection in Async Deposit Fulfillment
 ### Description/Code Snippet
The `fulfillDeposit` function converts pending assets to shares using the current share price via `_convertToShares`. There is no parameter for `minShares` or a deadline to enforce slippage protection. Users who request a deposit are exposed to price fluctuations between the time of their request and the time the Investment Manager executes the fulfillment. If the share price increases significantly (e.g., due to investment gains or manipulation), users receive fewer shares than expected.
 ### Static Signals
amountOutMin=0 or missing, price fetched at execution without user-specified floor, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds (value loss due to slippage)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

