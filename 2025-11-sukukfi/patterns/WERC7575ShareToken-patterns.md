## Verified Patterns Found: 30

## Verified Patterns Found in following Categories:

- ForcedAssetVsStrictEquality
- GriefableCallbacks
- ERC4626SharePriceMismatch
- SlippageMissingOrInsufficient
- ReadOnlyReentrancy
- FeeOnTransferAssumption
- NonStandardERC20Behavior
- AccountingInvariantViolation
- ReserveOrPriceDesync
- UnsafeRecipient
- PermitMisuse
- AllowanceRace
- StandardViolation
- StateGrowthOrStorageBloat
- AccessControlOrAuthByPass
- PricePrecisionOrRoundingError
- UnprotectedPauseOrStop



## Summary of Patterns

Non-standard Permit implementation breaks EIP-2612 compatibility

KYC Bypass in Batch Transfers

Unregistration Griefing via Active Request Bloat

Non-Standard ERC20 Transfer Requirement

Lack of exit mechanism for Claimable Deposits if KYC is revoked

ERC20 Allowance Race Condition

Inconsistent rBalance updates in rBatchTransfers hide losses

Missing slippage protection in investment management functions

Precision loss in redeem allows burning shares for zero assets

Protocol Incompatible with Fee-On-Transfer Tokens

Unprotected Redemptions (Missing Pause)

Donation Attack via ShareToken Balance Manipulation

Cross-Asset Depeg Vulnerability in Multi-Asset Share Token

Incompatibility with Fee-On-Transfer Tokens

Investment Valuation Unit Mismatch

Broken ERC-20/2612 Integration via Restricted Transfers and Permits

System-Wide DoS via Single Faulty Vault (Griefable Callback)

Missing Slippage Protection in Investment Functions

Storage Layout Risk via Non-Upgradeable ReentrancyGuard

Strict balance equality check breaks compatibility with fee-on-transfer tokens

System-Wide DoS via Faulty Vault and Unregister Lock

Cross-Vault Fund Draining via Infinite Approval

ReadOnlyReentrancy via totalAssets Inflation

Rounding Error in Async Vault Withdraw Inflates Share Price

Unsafe Controller Recipient in Async Deposit

Potential Reserved Asset Accounting Inconsistency

Missing Slippage Protection in Async Deposit/Redeem

Investment returns trapped in illiquid rBalance due to implementation mismatch with spec

adjustrBalance desyncs totalSupply causing underflow

Missing Slippage Protection in Investment Operations

## Patterns



 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: WERC7575ShareToken.permit

 ### Title
Non-standard Permit implementation breaks EIP-2612 compatibility
 ### Description/Code Snippet
The `permit` function in `WERC7575ShareToken` enforces a non-standard logic where self-allowance (`owner == spender`) requires a signature from a `_validator` rather than the `owner`. This matches the PermitMisuse pattern as it deviates from the EIP-2612 specification, causing standard wallet integrations and dApps that rely on standard `permit` signatures to fail when users attempt to approve themselves (which is forced by the contract's `approve` override).
 ### Static Signals
owner == spender checks validator signer, deviates from EIP-2612
 ### Assets at Risk
user interoperability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: WERC7575ShareToken.batchTransfers

 ### Title
KYC Bypass in Batch Transfers
 ### Description/Code Snippet
The WERC7575ShareToken is designed to strictly enforce KYC compliance for all token holders. While individual `transfer`, `transferFrom`, and `mint` functions explicitly check `isKycVerified[recipient]`, the `batchTransfers` and `rBatchTransfers` functions update balances directly via internal storage (`_balances`) without performing any KYC checks on the recipients (`creditors`). This allows the Validator to transfer tokens to non-KYC'd addresses, violating the protocol's core compliance invariant.
 ### Static Signals
_balances[account.owner] += amount, missing isKycVerified check
 ### Assets at Risk
Regulatory compliance
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Unregistration Griefing via Active Request Bloat
 ### Description/Code Snippet
The `unregisterVault` function in `ShareTokenUpgradeable` requires the target vault to have zero active deposit requesters (`metrics.activeDepositRequestersCount != 0`). A malicious user can initiate a minimal deposit request via `requestDeposit` and refuse to claim the shares after fulfillment. This keeps the user in the `activeDepositRequesters` set indefinitely. Since the Owner cannot force-claim or clear these requests, a single user can permanently prevent the `unregisterVault` operation, griefing the protocol's maintenance capabilities.
 ### Static Signals
condition activeDepositRequestersCount != 0, no force-remove mechanism, user-controlled state persistence
 ### Assets at Risk
governance capability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: NonStandardERC20Behavior

 ### Relevant Function/Location: WERC7575ShareToken.transfer

 ### Title
Non-Standard ERC20 Transfer Requirement
 ### Description/Code Snippet
WERC7575ShareToken enforces a non-standard requirement where 'transfer' calls fail unless the sender has explicitly set a self-allowance via the validator-controlled permit function. Standard ERC20 integrations (wallets, DEXs) assuming 'transfer' works with sufficient balance will fail.
 ### Static Signals
_spendAllowance(msg.sender, msg.sender, value), approve blocks self-approval
 ### Assets at Risk
user accessibility
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.sol.deposit

 ### Title
Lack of exit mechanism for Claimable Deposits if KYC is revoked
 ### Description/Code Snippet
The `deposit` function (used to claim shares after fulfillment) transfers shares to the receiver. The `ShareToken` enforces KYC verification on transfers. If a user's KYC status is revoked after their deposit request is fulfilled (Claimable state) but before they claim the shares, the `deposit` transaction will revert. Since `cancelDepositRequest` only supports requests in the Pending state, the user has no mechanism to cancel the request or retrieve their assets, resulting in permanently stuck funds.
 ### Static Signals
transfer reverts on non-KYC, cancel restricted to pending, no escape path for claimable
 ### Assets at Risk
user assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AllowanceRace

 ### Relevant Function/Location: ShareTokenUpgradeable.approve

 ### Title
ERC20 Allowance Race Condition
 ### Description/Code Snippet
The `ShareTokenUpgradeable` inherits standard `ERC20Upgradeable` behavior for `approve`. This allows a spender to front-run an allowance change transaction (e.g., Alice changes Bob's allowance from 100 to 50, Bob spends 100 then 50).
 ### Static Signals
inherits ERC20Upgradeable, approve implementation is standard
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Inconsistent rBalance updates in rBatchTransfers hide losses
 ### Description/Code Snippet
The `rBatchTransfers` function in `WERC7575ShareToken` updates `_rBalances` (Restricted/Invested Balance) asymmetrically: debits increase `_rBalances` fully, but credits decrease `_rBalances` only until 0 (silent truncation). If `rBatchTransfers` is used to process settlement returns where a loss occurred (returned amount < invested amount), and the account's `_rBalance` is not sufficient to absorb the full credit (e.g. due to previous cycles), the reduction is capped. This leaves `_rBalances` higher than the actual remaining invested capital. Since `ShareTokenUpgradeable` uses `rBalance` (via `getInvestedAssets`) to calculate `totalNormalizedAssets` for share pricing, this 'phantom' rBalance inflates the share price, allowing users to redeem shares for more assets than exist, effectively socializing the loss or draining the vault.
 ### Static Signals
_rBalances[account.owner] = 0, metric used for valuation
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.sol.withdrawFromInvestment

 ### Title
Missing slippage protection in investment management functions
 ### Description/Code Snippet
The functions `investAssets` and `withdrawFromInvestment` interact with an external `investmentVault` (IERC7575). `investAssets` calls `deposit` on the external vault without specifying a minimum amount of shares to receive, accepting any exchange rate. Similarly, `withdrawFromInvestment` calculates the number of shares to burn (`minShares`) using `previewWithdraw(amount)` in the same transaction. Using a spot price/preview for the minimum bound offers no protection against slippage or price manipulation (e.g. sandwich attacks) if the external vault has variable pricing.
 ### Static Signals
previewWithdraw used for minShares, deposit result ignored/unchecked, no minOut parameter
 ### Assets at Risk
vault assets, investment shares
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: ERC7575VaultUpgradeable.sol.redeem

 ### Title
Precision loss in redeem allows burning shares for zero assets
 ### Description/Code Snippet
In the `redeem` function, the asset amount is calculated as `shares.mulDiv(availableAssets, availableShares, Math.Rounding.Floor)`. If `shares * availableAssets` is less than `availableShares` (which can occur with high share prices or small redemption amounts), the result is 0 assets. The function proceeds to burn the user's shares via `ShareTokenUpgradeable($.shareToken).burn` but transfers 0 assets, causing a complete loss of value for the user.
 ### Static Signals
mulDiv with Floor rounding, no zero output check, burns input shares
 ### Assets at Risk
user shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: SafeTokenTransfers.safeTransferFrom

 ### Title
Protocol Incompatible with Fee-On-Transfer Tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library enforces strict equality between the transferred amount and the balance increase (`balanceAfter != balanceBefore + amount`). This causes all core vault functions (`requestDeposit`, `investAssets`, etc.) to revert if the underlying asset is a Fee-On-Transfer token or a Rebasing token where the balance change does not exactly match the transfer amount. This creates a Denial of Service risk if a supported asset enables fees.
 ### Static Signals
balanceAfter != balanceBefore + amount, revert TransferAmountMismatch
 ### Assets at Risk
availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnprotectedPauseOrStop

 ### Relevant Function/Location: ERC7575VaultUpgradeable.redeem

 ### Title
Unprotected Redemptions (Missing Pause)
 ### Description/Code Snippet
The vault includes `setVaultActive` to toggle `isActive`, but this flag is only checked in `requestDeposit`. Critical outflow functions `requestRedeem`, `redeem`, and `withdraw` do not check `isActive` or `paused`. In an emergency (e.g., asset depeg), the owner cannot pause withdrawals.
 ### Static Signals
requestDeposit checks isActive, redeem does not check isActive, no whenNotPaused modifier
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Donation Attack via ShareToken Balance Manipulation
 ### Description/Code Snippet
The `ShareTokenUpgradeable` calculates `totalNormalizedAssets` by summing vault assets and `investmentShareToken.balanceOf(address(this))`. Since standard ERC20 transfers cannot be blocked, an attacker can donate `investmentShareToken` directly to the contract. This artificially inflates `totalNormalizedAssets` without minting new shares, skewing the exchange rate (`shares = assets * Supply / TotalAssets`). While virtual shares (1e6) exist, they may be insufficient to prevent manipulation if the donation is large, grieving new depositors.
 ### Static Signals
IERC20(investmentShareToken).balanceOf(address(this)), shares = Math.mulDiv(normalizedAssets, circulatingSupply, totalNormalizedAssets...)
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ShareTokenUpgradeable.convertNormalizedAssetsToShares

 ### Title
Cross-Asset Depeg Vulnerability in Multi-Asset Share Token
 ### Description/Code Snippet
The `ShareTokenUpgradeable` calculates share price by aggregating normalized assets from all registered vaults, implicitly assuming a fixed 1:1 value ratio between all underlying assets (e.g., USDC, USDT, DAI). Since shares are fungible and minted based on this aggregated value, a depeg of any single supported asset allows arbitrageurs to deposit the depegged asset and redeem for valuable assets, draining the protocol's reserves of the stronger assets.
 ### Static Signals
aggregates assets from multiple vaults, assumes 1:1 peg for all assets, fungible shares for different assets
 ### Assets at Risk
vault assets (USDC, DAI, etc.)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: SafeTokenTransfers.safeTransfer, safeTransferFrom

 ### Title
Incompatibility with Fee-On-Transfer Tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library explicitly reverts if the received amount does not match the transferred amount (`balanceAfter != balanceBefore + amount`). This is used in `ERC7575VaultUpgradeable` for deposits. If the underlying asset imposes transfer fees (or is upgradeable and adds fees later), the vault will become unusable as all deposits will revert. While this protects internal accounting, it creates a rigid incompatibility with a class of ERC20 tokens.
 ### Static Signals
balanceAfter != balanceBefore + amount, revert TransferAmountMismatch
 ### Assets at Risk
availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ShareTokenUpgradeable._calculateInvestmentAssets

 ### Title
Investment Valuation Unit Mismatch
 ### Description/Code Snippet
ShareTokenUpgradeable sums investment vault SHARES directly with other vaults' ASSETS to calculate totalNormalizedAssets. This assumes a strict 1:1 price ratio. If the investmentVault is a yield-bearing ERC4626 (price > 1) or suffers losses (price < 1), the total asset calculation is incorrect, leading to mispriced shares in the top-level vault.
 ### Static Signals
totalNormalizedAssets += _calculateInvestmentAssets(), balanceOf(address(this)) used as assets
 ### Assets at Risk
vault assets, investor funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.permit, approve, transfer, transferFrom

 ### Title
Broken ERC-20/2612 Integration via Restricted Transfers and Permits
 ### Description/Code Snippet
The WERC7575ShareToken implementation violates ERC-20 and ERC-2612 standards by reverting `approve()` on self-approval and requiring a validator-signed permit for `transfer()`/`transferFrom()` (specifically enforcing self-allowance). The `permit()` function also deviates by requiring a validator signature instead of the owner's signature for self-allowance. This breaks compatibility with standard wallets, DEXs, and dApps that expect standard behavior, causing broken integrations and effective DoS for users without custom tooling.
 ### Static Signals
approve reverts on self-approval, transfer requires allowance, permit requires validator signature for self-owner
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
System-Wide DoS via Single Faulty Vault (Griefable Callback)
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract aggregates state from all registered vaults in `getCirculatingSupplyAndAssets` to calculate the global share price. This function iterates through all registered vaults and calls `getClaimableSharesAndNormalizedAssets`. If a single vault in the system reverts (e.g., due to a paused asset, bug, or malicious upgrade), the entire share token system halts, as `convertNormalizedAssetsToShares` will revert for ALL vaults. This couples the availability of the entire multi-asset system to its weakest component.
 ### Static Signals
loop over external calls, no try/catch, critical system dependency on external components
 ### Assets at Risk
system availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets, withdrawFromInvestment

 ### Title
Missing Slippage Protection in Investment Functions
 ### Description/Code Snippet
The functions `investAssets` and `withdrawFromInvestment` in `ERC7575VaultUpgradeable` interact with an external `investmentVault` to deposit assets or redeem shares. These operations determine the exchange rate at execution time without any user-defined or administrator-defined minimum output parameters (`minShares` or `minAssets`). If the `investmentVault` experiences price volatility, manipulation, or is upgraded to a vault with variable rates, the protocol may suffer loss of value due to slippage.
 ### Static Signals
deposit(amount, shareToken) without minOut, redeem(minShares) where minShares calculated from preview
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.N/A

 ### Title
Storage Layout Risk via Non-Upgradeable ReentrancyGuard
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` contract inherits from the standard, non-upgradeable `ReentrancyGuard` implementation from OpenZeppelin, rather than `ReentrancyGuardUpgradeable`. While OpenZeppelin v5.0+ uses namespaced storage to mitigate layout collisions, using the non-upgradeable version in a proxy context skips the constructor initialization (setting status to `NOT_ENTERED`), leaving the storage slot uninitialized (0). This forces the first `nonReentrant` call to perform a more expensive zero-to-non-zero state change and deviates from standard upgradeable contract best practices.
 ### Static Signals
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol", is Initializable, ReentrancyGuard
 ### Assets at Risk
Gas optimization, Future upgrade compatibility
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: SafeTokenTransfers.safeTransfer

 ### Title
Strict balance equality check breaks compatibility with fee-on-transfer tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library enforces a strict equality check (`balanceAfter == balanceBefore + amount`) after token transfers. This pattern, ForcedAssetVsStrictEquality, causes the vault to revert and lock funds if the underlying asset is a fee-on-transfer token (or if USDT toggles fees on). While the protocol mentions USDT support, this strict check makes it incompatible with USDT's potential fee feature.
 ### Static Signals
balanceAfter != balanceBefore + amount revert
 ### Assets at Risk
vault availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
System-Wide DoS via Faulty Vault and Unregister Lock
 ### Description/Code Snippet
The `ShareTokenUpgradeable` iterates over all registered vaults in `getCirculatingSupplyAndAssets` to calculate the share price (used by all vaults for deposits/redeems). If a single vault reverts (e.g., due to a buggy upgrade, paused state, or external asset failure affecting `balanceOf`), all operations across the entire multi-asset system will fail. Crucially, the `unregisterVault` function also calls `getVaultMetrics` on the vault and explicitly reverts if that call fails, creating a deadlock where a broken vault bricks the system and cannot be removed.
 ### Static Signals
getCirculatingSupplyAndAssets iterates all vaults, unregisterVault calls vault.getVaultMetrics, try/catch in unregisterVault reverts on failure
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: ShareTokenUpgradeable._configureVaultInvestmentSettings

 ### Title
Cross-Vault Fund Draining via Infinite Approval
 ### Description/Code Snippet
The ShareTokenUpgradeable contract aggregates investment shares (InvestmentShareToken) for all registered vaults in the multi-asset system. In `_configureVaultInvestmentSettings`, it grants unlimited allowance (`type(uint256).max`) to *every* registered vault to spend these shared InvestmentShareTokens. This breaks asset isolation: a single compromised or malicious vault can drain the entire investment pool belonging to all other vaults by calling `transferFrom` on the InvestmentShareToken, bypassing the intended compartmentalization of the multi-asset architecture.
 ### Static Signals
IERC20(investmentShareToken).approve(vaultAddress, type(uint256).max)
 ### Assets at Risk
All invested assets across all vaults
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
ReadOnlyReentrancy via totalAssets Inflation
 ### Description/Code Snippet
ERC7575VaultUpgradeable.totalAssets() relies on balanceOf(address(this)). In requestDeposit, assets are transferred in before the totalPendingDepositAssets state is updated (Pull-Then-Update). If the asset token has transfer hooks (ERC777/ERC1363), a callback can observe an inflated totalAssets() (balance increased, reserved liability not yet increased). This inflates the share price read by external systems or other vaults during the callback.
 ### Static Signals
balanceOf(this), external call before state update, totalAssets used for pricing
 ### Assets at Risk
share price integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Rounding Error in Async Vault Withdraw Inflates Share Price
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.withdraw`, the calculation of shares to burn uses `Math.Rounding.Floor`: `shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`. If a user withdraws a small amount of assets such that `assets * availableShares < availableAssets`, the result is 0 shares. The user receives assets, but `totalClaimableRedeemShares` is not reduced (or reduced less than proportionally). Since `ShareTokenUpgradeable` calculates `circulatingSupply` by subtracting `totalClaimableRedeemShares` from `totalSupply`, retaining a high `totalClaimableRedeemShares` artificially lowers the `circulatingSupply`. This inflates the global share price (`totalAssets / circulatingSupply`), allowing an attacker to redeem other shares at an inflated value, draining the protocol.
 ### Static Signals
rounding bias always favors caller, withdrawal rounds down shares
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Unsafe Controller Recipient in Async Deposit
 ### Description/Code Snippet
The `requestDeposit` function in `ERC7575VaultUpgradeable` takes a `controller` address argument which determines who has the authority to claim the resulting shares. There is no validation that this `controller` address is non-zero. If a user accidentally passes `address(0)` as the controller (e.g. via a frontend error or misunderstanding of the param), the deposited assets will be processed, but the resulting shares will be claimable only by the zero address, effectively burning the user's deposit and locking the assets in the vault permanently.
 ### Static Signals
no zero-address guard, value transfer to argument address
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Potential Reserved Asset Accounting Inconsistency
 ### Description/Code Snippet
The `totalAssets()` calculation subtracts `reservedAssets`, which includes pending deposits, claimable redeems, and cancelled deposits, but excludes `claimableDepositAssets`. While logically consistent with minted shares, the provided documentation explicitly flags 'claimableDeposit was added without conversion' as a bug to be fixed, implying strict reservation was intended but is missing or implemented differently.
 ### Static Signals
reservedAssets calculation, excludes totalClaimableDepositAssets
 ### Assets at Risk
vault accounting integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Missing Slippage Protection in Async Deposit/Redeem
 ### Description/Code Snippet
The `requestDeposit` and `requestRedeem` functions in `ERC7575VaultUpgradeable` initiate asynchronous operations where assets or shares are locked until a privileged Investment Manager calls `fulfillDeposit` or `fulfillRedeem`. The exchange rate is determined at the moment of fulfillment using the current vault state (`_convertToShares` / `_convertToAssets`). However, these request functions do not accept a minimum output parameter (`minShares` for deposit, `minAssets` for redeem) or a deadline. This exposes users to unlimited slippage if the share price changes unfavorably between the request and the fulfillment, which could occur due to market fluctuations, `adjustrBalance` updates, or intentional timing by the manager.
 ### Static Signals
amountOutMin=0 or missing, price fetched at execution without user-specified floor, deadline omitted
 ### Assets at Risk
user deposits, user redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Investment returns trapped in illiquid rBalance due to implementation mismatch with spec
 ### Description/Code Snippet
The `adjustrBalance` function in `WERC7575ShareToken` is intended to distribute investment returns. According to the protocol documentation (Scenario 1), when a profit is realized (`amountr > amounti`), the principal should be unlocked (`_rBalances -= amounti`) and the total return credited to the liquid balance (`_balances += amountr`). 

However, the code implementation at lines 773-776 does not update `_balances` at all. Instead, it adds the profit to `_rBalances`: `_rBalances[account] += difference`. 

Since `_rBalances` cannot be transferred, burned, or redeemed by the user (only `_balances` can be used for these operations), the investment profit is effectively trapped in an illiquid state. This contradicts the documentation and breaks the yield realization flow for the Investment Layer.
 ### Static Signals
_rBalances += difference, missing _balances update
 ### Assets at Risk
investment yield
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
adjustrBalance desyncs totalSupply causing underflow
 ### Description/Code Snippet
The `adjustrBalance` function in `WERC7575ShareToken` directly modifies `_balances` to reflect profit or loss from settlements but fails to update `_totalSupply`. When profit is injected (`amountr > amounti`), `_balances` increases while `_totalSupply` remains unchanged, breaking the invariant `sum(balances) == totalSupply`. When the beneficiary (e.g., `ShareTokenUpgradeable`) later redeems these shares, `_burn` is called. `_burn` subtracts the amount from `_totalSupply` inside an `unchecked` block (inherited from `WERC7575ShareToken._update`). Since the amount burned can exceed the tracked `_totalSupply` (due to the previously injected profit), `_totalSupply` underflows to a near-infinite value. This corrupts the token state and may break integrations relying on `totalSupply`.
 ### Static Signals
_balances[account] += amountr, no _totalSupply update, unchecked { _totalSupply -= value }
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Missing Slippage Protection in Investment Operations
 ### Description/Code Snippet
The `investAssets` and `withdrawFromInvestment` functions in `ERC7575VaultUpgradeable` interact with an external investment vault (ERC7575/ERC4626) to deposit and redeem funds. `investAssets` calls `deposit` without a minimum share output parameter. `withdrawFromInvestment` calls `redeem` using a share amount calculated from `previewWithdraw` in the same transaction, without enforcing a minimum asset output `minAmountOut`. This exposes the vault to slippage, sandwich attacks, or manipulation of the external vault's exchange rate during the transaction, potentially causing loss of principal during investment or divestment.
 ### Static Signals
amountOutMin=0, payout calculated at execution time without minimum bound, price fetched at execution without user-specified floor
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

