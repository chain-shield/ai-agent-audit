## Verified Patterns Found: 31

## Verified Patterns Found in following Categories:

- PrecisionDriftAccumulation
- UnboundedLoops
- UnsafeRecipient
- SlippageMissingOrInsufficient
- BeaconOrFactoryAuthorityDrift
- StandardViolation
- ReadOnlyReentrancy
- FeeOnTransferAssumption
- AllowanceRace
- ReserveOrPriceDesync
- AccountingInvariantViolation
- ForcedAssetVsStrictEquality
- AccessControlOrAuthByPass
- FlashLoanEconomicManipulation
- StateGrowthOrStorageBloat
- ERC4626SharePriceMismatch



## Summary of Patterns

Accounting Invariant Violation in rBalance Adjustment

Slippage Missing in Investment Withdrawal

Strict Balance Equality Prevents Vault Unregistration

ReadOnlyReentrancy via totalAssets inflation

Strict Balance Check in SafeTokenTransfers Bricks Fee-on-Transfer Tokens

Silent Truncation of rBalance

Read-Only Reentrancy in Share Price

Quadratic Gas Cost in Transfer Consolidation

Centralized Transfer Control via Validator

Precision Loss in rBalance Tracking

Reserved Assets Accounting Bug (ReserveOrPriceDesync)

Share price manipulation via unregisterVault ignoring dust shares

Insolvency via Hidden Liability in Share Price Calculation

Reserved Assets Invariant Violation

Strict Balance Equality Check Incompatible with Fee-on-Transfer Tokens

Unsafe Recipient in Redeem and Withdraw

Insolvency risk due to pricing unbacked rBalance in ShareTokenUpgradeable

Rounding direction favors user in withdraw and mint functions

Allowance Race Condition in Settlement Token

ERC-20 Approve Blocks Self-Approval

Investment Valuation Mismatch in Share Price

Incompatibility with Fee-on-Transfer Tokens

Vault Unregistration DOS via Dust in Active Requesters Set

Incorrect Rounding Direction in Mint Claims

Investment Manager Authority Drift

Flash Loan Manipulation in Investment Withdrawal

Missing Slippage Protection in Investment Operations

ERC-20 Standard Violation Breaking Integrations

Standard Violation: ERC4626 preview functions revert

Slippage Missing in Investment Allocation

Missing Slippage Protection in Async Deposit/Redeem Requests

## Patterns



 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Accounting Invariant Violation in rBalance Adjustment
 ### Description/Code Snippet
In `WERC7575ShareToken.adjustrBalance`, the function modifies `_rBalances` to reflect investment profits (`amountr > amounti`) by adding the difference directly to the user's `_rBalances` without minting new tokens or updating `_totalSupply`. This breaks the invariant `totalSupply == sum(balances) + sum(rBalances)`. Since `_rBalances` can be converted to `_balances` (via `rBatchTransfers`) and subsequently burned, users can burn more tokens than exist in `_totalSupply`, eventually causing `_totalSupply` to underflow and revert, leading to a permanent Denial of Service on burning and withdrawals.
 ### Static Signals
balance tracking references different token than actual holdings, totalSupply != sum(balances)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdrawFromInvestment

 ### Title
Slippage Missing in Investment Withdrawal
 ### Description/Code Snippet
The `withdrawFromInvestment` function redeems shares from the `investmentVault` to recover a target amount of assets. It calculates the required shares using `previewWithdraw` and calls `redeem`, but does not enforce a minimum asset output (`minAssets`). If the investment vault's exchange rate fluctuates unfavorably between the preview and execution (e.g., due to front-running or oracle updates), the vault may burn shares while receiving fewer assets than the requested `amount`, causing loss of principal.
 ### Static Signals
redeem called without minOutput, actualAmount calculation implies uncertainty
 ### Assets at Risk
invested capital
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Strict Balance Equality Prevents Vault Unregistration
 ### Description/Code Snippet
The `unregisterVault` function in `ShareTokenUpgradeable` enforces a strict check `IERC20(asset).balanceOf(vaultAddress) != 0` to ensure the vault is empty. An attacker can forcefully send (donate) 1 wei of the asset to the vault, causing this check to fail and preventing the vault from being unregistered. While the Investment Manager can theoretically sweep funds, exact clearing to 0 requires precise manipulation which can be griefed continuously.
 ### Static Signals
balanceOf(address) != 0, strict equality check on balance
 ### Assets at Risk
none (DoS of admin function)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
ReadOnlyReentrancy via totalAssets inflation
 ### Description/Code Snippet
The `totalAssets` function calculates the vault's value as `balanceOf(asset) - reservedAssets`. In `requestDeposit`, assets are transferred into the vault via `safeTransferFrom` before the `reservedAssets` (specifically `totalPendingDepositAssets`) are updated. If the asset token allows reentrancy (e.g. ERC777 tokensToSend/Received hooks), an attacker can enter a read-only context where `balanceOf` has increased but `reservedAssets` has not, inflating `totalAssets`. This inflates the ShareToken price (calculated via `getCirculatingSupplyAndAssets` -> `totalAssets`) which can be exploited by external protocols using the ShareToken as collateral or in AMMs.
 ### Static Signals
balanceOf called, state update after external call, view function reads inconsistent state
 ### Assets at Risk
yield, external protocol funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: SafeTokenTransfers.safeTransfer, safeTransferFrom

 ### Title
Strict Balance Check in SafeTokenTransfers Bricks Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library enforces a strict equality check `balanceAfter == balanceBefore + amount`. This pattern causes all deposit/withdrawal/transfer functions to revert if the underlying asset is a Fee-on-Transfer token or has non-standard transfer behavior (e.g. rebasing). While the library comments warn about this, the system's reliance on this strict check makes it vulnerable to bricking if such a token is introduced or upgraded (e.g. USDC proxy upgrade).
 ### Static Signals
if (balanceAfter != balanceBefore + amount) revert TransferAmountMismatch();
 ### Assets at Risk
User deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Silent Truncation of rBalance
 ### Description/Code Snippet
In `rBatchTransfers`, if an account's credit exceeds its `rBalance` and the update flag is set, `rBalance` is silently truncated to 0 instead of underflowing or tracking negative investment. While `rBalance` is documented as informational, strict reliance on it for investment accounting could be disrupted by this loss of precision.
 ### Static Signals
_rBalances[account.owner] = 0, silent truncation
 ### Assets at Risk
investment accounting data
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Read-Only Reentrancy in Share Price
 ### Description/Code Snippet
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function (used for share pricing) relies on `ERC7575VaultUpgradeable.totalAssets()`, which reads the underlying asset's `balanceOf(address(this))`. If the underlying asset allows reentrancy (e.g., ERC777), an attacker can enter a read-only context during a token transfer hook where the balance is updated but the vault's internal accounting (`pendingDepositAssets`) is not yet updated. This causes the share price to be temporarily skewed, potentially allowing exploitation of third-party protocols relying on this price.
 ### Static Signals
balanceOf used in view, no nonReentrantView
 ### Assets at Risk
third-party integrations
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: WERC7575ShareToken.consolidateTransfers

 ### Title
Quadratic Gas Cost in Transfer Consolidation
 ### Description/Code Snippet
`WERC7575ShareToken.consolidateTransfers` employs a nested loop O(N^2) structure to aggregate accounts. With `MAX_BATCH_SIZE = 100`, the inner loop can iterate up to 200 times, leading to ~20,000 iterations. While bounded, this quadratic complexity in a memory-heavy operation can cause excessive gas consumption, potentially leading to DoS of the settlement functionality on networks with tighter gas limits.
 ### Static Signals
nested loops, loop bound depends on input array length
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: WERC7575ShareToken.transfer

 ### Title
Centralized Transfer Control via Validator
 ### Description/Code Snippet
The `WERC7575ShareToken` transfer logic creates a dependency on the Validator role for every transfer (via the self-allowance `permit` requirement). If the Validator goes offline or acts maliciously, all token transfers are frozen. This effectively delegates control of all user funds to the Validator.
 ### Static Signals
_spendAllowance(from, from, value), permit validator check
 ### Assets at Risk
all user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Precision Loss in rBalance Tracking
 ### Description/Code Snippet
In `WERC7575ShareToken.rBatchTransfers`, if a credit amount exceeds an account's current `rBalance`, the `rBalance` is silently truncated to zero (`_rBalances[account.owner] = 0`). This truncation destroys information about the invested capital. Subsequent calls to `adjustrBalance` (which rely on historical invested amounts) or `cancelrBalanceAdjustment` may operate on incorrect or underflowing balances, breaking the accounting for yield distribution.
 ### Static Signals
_rBalances[account.owner] = 0, silent truncation
 ### Assets at Risk
yield accounting
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Reserved Assets Accounting Bug (ReserveOrPriceDesync)
 ### Description/Code Snippet
The `totalAssets()` calculation in `ERC7575VaultUpgradeable` excludes `totalPendingDepositAssets`, `totalClaimableRedeemAssets`, and `totalCancelDepositAssets`, but fails to exclude `totalClaimableDepositAssets` (assets backing shares that have been minted but not yet claimed). This implies these assets are considered 'available' and can be moved to the investment vault via `investAssets()`. This directly violates the protocol specification stating 'Reserved assets (pending/claimable) sit idle, earning no yield' and can lead to over-investment and unintended risk exposure for users in the claimable state.
 ### Static Signals
reservedAssets calculation misses a component, totalAssets used for investment availability
 ### Assets at Risk
claimableDepositAssets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Share price manipulation via unregisterVault ignoring dust shares
 ### Description/Code Snippet
The `ShareTokenUpgradeable.unregisterVault` function allows the owner to remove a vault from the system if `metrics.totalClaimableRedeemAssets` is zero. However, it fails to check `metrics.totalClaimableRedeemShares`. Due to rounding down in `fulfillRedeem` (which uses `Math.Rounding.Floor` for asset conversion), a vault can be left with 0 claimable assets but a non-zero amount of claimable shares (dust). If such a vault is unregistered, these shares are no longer subtracted from the `totalSupply` in `getCirculatingSupplyAndAssets`, causing the calculated circulating supply to artificially increase. This results in an immediate drop in the share price (`Assets / Supply`), effectively diluting existing share value.
 ### Static Signals
unregisterVault, totalClaimableRedeemAssets != 0, missing share check
 ### Assets at Risk
share value
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillRedeem

 ### Title
Insolvency via Hidden Liability in Share Price Calculation
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, `totalAssets()` returns `max(0, balance - reservedAssets)`. When `fulfillRedeem` is called while assets are invested (low liquid balance), `reservedAssets` (liabilities) can exceed `balance`. `totalAssets` clamps to 0, hiding the deficit. `ShareTokenUpgradeable` calculates price as `(Invested + totalAssets) / circulatingSupply`. Since the liability is ignored in the numerator but the shares are removed from the denominator (via `circulatingSupply`), the share price is artificially inflated (e.g., doubles if 50% redeemed), allowing subsequent redeemers to drain more value than exists.
 ### Static Signals
balance > reservedAssets ? balance - reservedAssets : 0, fulfillRedeem does not check balance
 ### Assets at Risk
vault assets, invested assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Reserved Assets Invariant Violation
 ### Description/Code Snippet
The `totalAssets` function calculates `reservedAssets` to determine investable capital. The implementation sums `totalPendingDepositAssets`, `totalClaimableRedeemAssets`, and `totalCancelDepositAssets`, but explicitly excludes `totalPendingRedeemShares` (converted to assets). This contradicts the protocol documentation, which states that pending redemptions are summed 'to guarantee enough idle liquidity'. By failing to reserve assets for pending redemptions, the Investment Manager can over-invest liquid funds, potentially causing `fulfillRedeem` or subsequent user claims to fail due to lack of liquidity.
 ### Static Signals
reservedAssets calculation missing pendingRedeem, mismatch with documentation invariants
 ### Assets at Risk
user redemption liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: SafeTokenTransfers.safeTransferFrom

 ### Title
Strict Balance Equality Check Incompatible with Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library used by `ERC7575VaultUpgradeable` enforces a strict check that the recipient's balance increase exactly equals the transfer amount (`balanceAfter != balanceBefore + amount`). This creates a denial of service for any underlying asset that implements transfer fees (e.g., USDT/USDC if fees are enabled), as the check will always revert. This bricks the vault for such tokens.
 ### Static Signals
require(balanceAfter == balanceBefore + amount)
 ### Assets at Risk
user funds (frozen)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.redeem

 ### Title
Unsafe Recipient in Redeem and Withdraw
 ### Description/Code Snippet
The `redeem` and `withdraw` functions in `ERC7575VaultUpgradeable` transfer assets to a user-specified `receiver` without checking if it is `address(0)`. While `SafeTokenTransfers` handles fee-on-transfer checks, it relies on `SafeERC20`, which may not revert on zero-address transfers for all token implementations. This could lead to permanent loss of assets if `address(0)` is passed accidentally.
 ### Static Signals
no zero-address guard, transfer to receiver
 ### Assets at Risk
user assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ShareTokenUpgradeable._calculateInvestmentAssets

 ### Title
Insolvency risk due to pricing unbacked rBalance in ShareTokenUpgradeable
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract calculates its total assets (and thus share price) in `_calculateInvestmentAssets` by summing its `balanceOf` and `rBalanceOf` in the investment share token. However, the `WERC7575ShareToken` (the investment token) separates these balances: `_balances` are backed by assets in the synchronous vault, while `_rBalances` are virtual/reserved balances created via administrative adjustments or transfers and are NOT backed by liquid assets in the `WERC7575Vault`. Since `ShareTokenUpgradeable` allows users to redeem shares based on a price that includes this unbacked `rBalance`, but can only physically withdraw assets corresponding to its `balance` via `withdrawFromInvestment`, the vault risks insolvency and DoS of redemptions if the `rBalance` component constitutes a significant portion of the valuation.
 ### Static Signals
balanceOf + rBalanceOf, withdrawFromInvestment, valuation mismatch
 ### Assets at Risk
user funds, vault solvency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Rounding direction favors user in withdraw and mint functions
 ### Description/Code Snippet
In ERC7575VaultUpgradeable, both withdraw() and mint() use Math.Rounding.Floor when converting between assets and shares for claims. In withdraw(), shares are calculated using Floor, allowing users to burn fewer shares than required. In mint(), assets to consume are calculated using Floor, allowing users to spend fewer claimable assets than required. Both leak value from the vault.
 ### Static Signals
Math.Rounding.Floor used in asset-to-share calc during withdrawal, Math.Rounding.Floor used in share-to-asset calc during minting
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AllowanceRace

 ### Relevant Function/Location: WERC7575ShareToken.approve

 ### Title
Allowance Race Condition in Settlement Token
 ### Description/Code Snippet
The `WERC7575ShareToken` overrides `approve` to block self-approval but retains the standard ERC-20 approval race condition for third-party spenders. If a carrier attempts to change an operator's allowance from a non-zero value to another non-zero value, the operator can front-run the transaction to spend the old allowance before the new one is set, effectively spending `old + new` allowance.
 ### Static Signals
changes allowance from X to Y without zeroing
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.approve

 ### Title
ERC-20 Approve Blocks Self-Approval
 ### Description/Code Snippet
WERC7575ShareToken overrides `approve` to revert if `msg.sender == spender`. This prevents users from setting their own allowance, enforcing the requirement to use the Validator-signed `permit` flow for self-allowance. This is a deviation from the ERC-20 standard which allows owners to approve themselves.
 ### Static Signals
msg.sender == spender revert, ERC20 deviation
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ShareTokenUpgradeable._calculateInvestmentAssets

 ### Title
Investment Valuation Mismatch in Share Price
 ### Description/Code Snippet
The ShareTokenUpgradeable calculates `totalNormalizedAssets` by adding `_calculateInvestmentAssets()`, which sums the `balanceOf` and `rBalanceOf` held in the `investmentShareToken`. This logic assumes a strict 1:1 exchange rate between investment shares and assets. If the configured `investmentShareToken` is a floating-price ERC4626 vault (e.g. another ShareTokenUpgradeable with yield), the system incorrectly values it at 1:1, ignoring accumulated yield or losses. This leads to incorrect share pricing for the top-level vault.
 ### Static Signals
totalNormalizedAssets += _calculateInvestmentAssets(), balanceOf(address(this)), rBalanceOf(address(this)), assumes 1:1 price
 ### Assets at Risk
yield, principal
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Incompatibility with Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library used by `ERC7575VaultUpgradeable` explicitly reverts if the received balance delta does not exactly match the transferred amount. This renders the vault incompatible with tokens that have transfer fees (e.g., USDT if fees are enabled) or deflationary mechanisms, causing a Denial of Service for deposits with such assets.
 ### Static Signals
balanceAfter != balanceBefore + amount, revert TransferAmountMismatch
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Vault Unregistration DOS via Dust in Active Requesters Set
 ### Description/Code Snippet
The `unregisterVault` function in `ShareTokenUpgradeable` reverts if the vault has any active deposit requesters (`metrics.activeDepositRequestersCount != 0`). A user is removed from `activeDepositRequesters` only when they fully claim their deposit in `deposit`/`mint`. A malicious user can initiate a deposit, have it fulfilled, and then intentionally leave a dust amount (1 wei) unclaimed. This keeps them in the `activeDepositRequesters` set indefinitely. Since the admin cannot force a claim or clear this set, the vault cannot be unregistered, causing a Denial of Service on the unregistration lifecycle.
 ### Static Signals
state enumeration via unbounded array traversal, append-only arrays with no pruning, condition depends on user-controlled state
 ### Assets at Risk
protocol availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ERC7575VaultUpgradeable.mint

 ### Title
Incorrect Rounding Direction in Mint Claims
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.mint`, when a user claims shares from a fulfilled deposit, the amount of claimable assets to deduct is calculated using `Math.Rounding.Floor` (`assets = shares.mulDiv(availableAssets, availableShares, Math.Rounding.Floor)`). This favors the user by deducting slightly fewer assets than the proportional value of the shares, effectively leaving 'dust' assets in the `claimableDepositAssets` mapping which are then socialized or lost. Standard ERC4626 implementation requires `Ceil` rounding when calculating assets input for a fixed share output to favor the protocol/vault.
 ### Static Signals
rounding bias favors caller, mint uses Floor rounding for assets
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: ERC7575VaultUpgradeable.setInvestmentManager

 ### Title
Investment Manager Authority Drift
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` contract allows its local owner to set the `investmentManager` via `setInvestmentManager`, potentially desynchronizing it from the centralized `ShareTokenUpgradeable` registry. The documentation mandates a 'Centralized Investment Architecture' where a single manager controls all vaults and updates propagate automatically. This implementation allows a specific vault's authority to drift from the global configuration, breaking the centralized management invariant.
 ### Static Signals
local admin override of global registry, drift from factory/registry configuration
 ### Assets at Risk
vault management control
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdrawFromInvestment

 ### Title
Flash Loan Manipulation in Investment Withdrawal
 ### Description/Code Snippet
The `withdrawFromInvestment` function calculates the shares to burn using `investmentVault.previewWithdraw(amount)`. If `investmentVault` is a standard ERC4626 vault, `previewWithdraw` returns the amount of shares based on the current spot exchange rate. An attacker can manipulate the investment vault's total assets (e.g., via flash loan donation) to skew the exchange rate, causing `withdrawFromInvestment` to burn an excessive number of investment shares for the requested assets, resulting in value loss for the protocol.
 ### Static Signals
previewWithdraw(amount), redeem(shares), spot price usage
 ### Assets at Risk
invested assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets, withdrawFromInvestment

 ### Title
Missing Slippage Protection in Investment Operations
 ### Description/Code Snippet
The `investAssets` and `withdrawFromInvestment` functions in `ERC7575VaultUpgradeable` execute cross-contract deposits and redemptions with the `investmentVault` without specifying a minimum output amount (`minShares` or `minAssets`). The code assumes a fixed exchange rate. If the `investmentVault` (an interface-based external contract) were to have a variable exchange rate or be upgraded to one, or if the `shareToken` configuration creates a mismatch, value could be lost during these operations due to lack of slippage bounds.
 ### Static Signals
deposit(amount, $.shareToken), redeem(minShares, address(this), shareToken_), no minAmountOut parameter
 ### Assets at Risk
Vault Assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.transfer

 ### Title
ERC-20 Standard Violation Breaking Integrations
 ### Description/Code Snippet
`WERC7575ShareToken` modifies `transfer` and `transferFrom` to require the sender to have a self-allowance (set via validator-signed `permit`). Standard ERC-20 `transfer` calls will revert. Additionally, `approve` reverts if `msg.sender == spender`, preventing users from setting their own allowance. This deviation from the ERC-20 standard breaks compatibility with wallets, DEXs, and other protocols expecting standard behavior.
 ### Static Signals
_spendAllowance(msg.sender, msg.sender, value), if (msg.sender == spender) revert
 ### Assets at Risk
user funds (stuck)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.previewDeposit

 ### Title
Standard Violation: ERC4626 preview functions revert
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` contract implements `IERC7575` which extends `IERC4626`. However, the view functions `previewDeposit`, `previewMint`, `previewWithdraw`, and `previewRedeem` are implemented to strictly revert with `AsyncFlow()`. While this reflects the asynchronous nature of the vault, it violates the ERC-4626 standard which requires these functions to return a simulation of the effects of an exchange. This deviation breaks integration with standard ERC-4626 tooling, routers, and other protocols that rely on these functions for off-chain estimation or on-chain composition.
 ### Static Signals
revert AsyncFlow(), preview function reverts
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Slippage Missing in Investment Allocation
 ### Description/Code Snippet
The `investAssets` function in `ERC7575VaultUpgradeable` deposits assets into an external `investmentVault` without specifying a minimum amount of shares to receive. If the `investmentVault` has a variable exchange rate or is manipulated before the transaction, the vault could receive significantly fewer shares than expected, causing value loss for all share token holders.
 ### Static Signals
IERC7575($.investmentVault).deposit(amount, $.shareToken), no minShares parameter
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Missing Slippage Protection in Async Deposit/Redeem Requests
 ### Description/Code Snippet
The `requestDeposit` and `requestRedeem` functions initiate asynchronous operations where assets or shares are transferred immediately, but the exchange rate is determined later upon fulfillment by the Investment Manager. There are no parameters for users to specify a minimum amount of shares to receive (deposit) or assets to receive (redeem), nor a deadline. This exposes users to unlimited slippage if the share price changes unfavorably between the request and the fulfillment.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound, price fetched at execution without user-specified floor
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

