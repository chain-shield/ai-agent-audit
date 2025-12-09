## Verified Patterns Found: 19

## Verified Patterns Found in following Categories:

- AccessControlOrAuthByPass
- ERC4626SharePriceMismatch
- SlippageMissingOrInsufficient
- AccountingInvariantViolation
- FlashLoanEconomicManipulation
- ReadOnlyReentrancy
- StandardViolation
- UnsafeRecipient
- GriefableCallbacks



## Summary of Patterns

ERC4626 rounding mismatch in withdraw function

UnsafeRecipient in claim functions

Inconsistent rBalance tracking leads to yield distribution DoS

DoS in unregisterVault via Griefable Callback

Share Price Manipulation via Asset Donation

Unsafe Burn via Transfer to Zero Address

Missing Slippage Protection in Investment Operations

ERC4626 Preview Functions Revert Violating Standard

System-wide DoS via single broken vault dependency

Missing Slippage Protection in Investment Withdrawal

Silent rBalance Truncation Corrupts Investment Accounting

Unsafe Recipient in Async Claim Functions

Accounting invariant violation in investment valuation

Missing KYC Validation in ShareTokenUpgradeable

ReadOnlyReentrancy via requestDeposit state update ordering

Missing slippage protection in investment transactions

Slippage Missing in Async Deposit/Redeem

Accounting Invariant Violation via Asset Clamping

Total Assets Manipulation via Donation

## Patterns



 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
ERC4626 rounding mismatch in withdraw function
 ### Description/Code Snippet
The `withdraw` function in `ERC7575VaultUpgradeable` uses `Math.Rounding.Floor` when calculating the amount of shares to burn (`shares = assets.mulDiv(..., Math.Rounding.Floor)`). The ERC4626 specification requires `previewWithdraw` (and by extension `withdraw`) to round up (`Ceil`) the shares burned to favor the vault. Using `Floor` allows users to withdraw small amounts of assets (dust) for 0 shares, potentially leaving 'zombie' share entitlements in the bucket that can no longer claim assets.
 ### Static Signals
Math.Rounding.Floor in withdraw shares calculation
 ### Assets at Risk
vault shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
UnsafeRecipient in claim functions
 ### Description/Code Snippet
The `withdraw` and `redeem` functions (used to claim fulfilled requests) accept a `receiver` address but do not validate that it is non-zero. They call `SafeTokenTransfers.safeTransfer`, which delegates to `SafeERC20.safeTransfer`. 

While some ERC20 tokens revert on transfer to address(0), others do not (or simply burn the tokens). If a user or operator accidentally passes `address(0)` as the receiver, the claimed assets could be permanently lost.
 ### Static Signals
no zero-address guard, transfer to user-supplied address
 ### Assets at Risk
claimed assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Inconsistent rBalance tracking leads to yield distribution DoS
 ### Description/Code Snippet
In `WERC7575ShareToken.sol`, the `rBatchTransfers` function (lines 1184-1188) allows `_rBalances` to be silently truncated to zero if a user receives credits exceeding their restricted balance. However, the `adjustrBalance` function (lines 765-776) attempts to subtract the full original investment amount (`amounti`) from `_rBalances` in loss or break-even scenarios. If `_rBalances` was previously truncated, `adjustrBalance` will revert due to underflow. This creates a permanent deadlock where the Revenue Admin is unable to record investment outcomes for any user who has utilized their liquidity via `rBatchTransfers`, effectively breaking the yield distribution mechanism for active users.
 ### Static Signals
_rBalances[account] -= amounti, _rBalances[account.owner] = 0
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
DoS in unregisterVault via Griefable Callback
 ### Description/Code Snippet
The `unregisterVault` function in `ShareTokenUpgradeable` includes a safety check that calls `getVaultMetrics()` on the vault. If this call reverts (e.g. if the vault is paused, broken, upgraded to malicious logic, or self-destructed), the `catch` block explicitly reverts with `CannotUnregisterActiveVault()`. This makes it impossible to unregister a broken vault. Since `getCirculatingSupplyAndAssets` iterates over all registered vaults and calls them, a single broken vault that cannot be unregistered will permanently brick the entire Multi-Asset system (DoS on all deposits/redeems/conversions).
 ### Static Signals
try ... catch { revert ... }, external call in loop, no bypass on callback failure
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Share Price Manipulation via Asset Donation
 ### Description/Code Snippet
The `totalAssets` function in `ERC7575VaultUpgradeable` relies directly on `IERC20(asset).balanceOf(address(this))` minus reserved amounts. It lacks internal accounting for `managedAssets`. An attacker can donate assets (e.g., via flash loan) to the vault to artificially inflate the share price. If the Investment Manager's fulfillment transaction occurs while the balance is inflated (e.g., attacker front-runs the manager), users receive fewer shares for their deposits or incorrect asset amounts for redemptions.
 ### Static Signals
branches on pool.balanceOf(), share price derived from manipulable pool state
 ### Assets at Risk
user shares, user assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: WERC7575ShareToken._update

 ### Title
Unsafe Burn via Transfer to Zero Address
 ### Description/Code Snippet
In `WERC7575ShareToken`, the overridden `_update` function explicitly allows transfers to `address(0)` and treats them as burns (`_totalSupply -= value`). Standard ERC20 implementations usually revert on transfers to the zero address. Since `batchTransfers` and `rBatchTransfers` allow the validator to pass arbitrary creditor addresses, an accidental inclusion of `address(0)` in the creditor array would irreversibly burn debtor funds instead of reverting the transaction.
 ### Static Signals
if (to == address(0)) { _totalSupply -= value; }, no zero-address check in batchTransfers
 ### Assets at Risk
user balances
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets, withdrawFromInvestment

 ### Title
Missing Slippage Protection in Investment Operations
 ### Description/Code Snippet
The `investAssets` and `withdrawFromInvestment` functions interact with an external `investmentVault` (defined as `IERC7575`) to deposit and redeem assets. These functions essentially perform token swaps (Asset <-> Investment Share) but lack minimum output parameters (`minShares` for investing, `minAssets` for withdrawing). If the `investmentVault` operates with a variable exchange rate (e.g., a standard ERC4626 vault with floating supply/assets), these transactions are susceptible to slippage or sandwich attacks where the Investment Manager receives fewer shares or assets than expected.
 ### Static Signals
no minAmountOut parameter, payout calculated at execution time without minimum bound, shares = IERC7575($.investmentVault).deposit(amount, $.shareToken)
 ### Assets at Risk
vault assets, investment shares
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.previewDeposit

 ### Title
ERC4626 Preview Functions Revert Violating Standard
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` implementation causes `previewDeposit`, `previewMint`, `previewWithdraw`, and `previewRedeem` to revert with `AsyncFlow`. This violates the ERC4626 specification, which states these functions MUST return as close to and no more than the exact amount and MUST NOT revert. This deviation breaks integrations with routers, aggregators, and other systems expecting standard ERC4626 behavior.
 ### Static Signals
revert in view function, preview/view functions modify state
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
System-wide DoS via single broken vault dependency
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract iterates over all registered vaults in `getCirculatingSupplyAndAssets()` to calculate global supply metrics. This function calls `getClaimableSharesAndNormalizedAssets()` on each vault. If a single registered vault reverts (e.g. due to a bug, failed upgrade, or malicious behavior), the entire ShareToken's conversion logic (`convertToShares`/`convertToAssets`) reverts. This bricks deposits, redemptions, and withdrawals for *all* vaults in the system. Furthermore, `unregisterVault` also calls the vault (`getVaultMetrics`) and reverts on failure, making it impossible to remove the broken vault and creating a permanent system deadlock.
 ### Static Signals
loops over assetToVault, external call to getClaimableSharesAndNormalizedAssets inside loop, unregisterVault calls external vault function with try/catch that reverts on failure
 ### Assets at Risk
All user funds in all vaults (locked due to DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdrawFromInvestment

 ### Title
Missing Slippage Protection in Investment Withdrawal
 ### Description/Code Snippet
The `withdrawFromInvestment` function in `ERC7575VaultUpgradeable` calculates the share amount to redeem using `previewWithdraw` (which uses Ceil rounding) but executes `redeem` which calculates assets using `previewRedeem` (which uses Floor rounding). It does not accept a minimum asset output parameter. If the external `investmentVault` has a variable exchange rate or experiences slippage/rounding variances, the protocol may receive fewer assets than expected without reverting, leading to value loss for the vault.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Silent rBalance Truncation Corrupts Investment Accounting
 ### Description/Code Snippet
In `WERC7575ShareToken`, the `rBatchTransfers` function silently truncates an account's `_rBalances` (reserved/invested balance) to zero if a credit operation exceeds the current `rBalance`. This creates an accounting invariant violation where the sum of `rBalance` changes does not accurately reflect the movement of invested capital, potentially leading to a disconnect between the recorded invested capital and the actual state.
 ### Static Signals
accounting state not updated when underlying asset is swapped, balance tracking references different token than actual holdings
 ### Assets at Risk
investment accounting data
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.redeem

 ### Title
Unsafe Recipient in Async Claim Functions
 ### Description/Code Snippet
The `redeem` and `withdraw` functions in `ERC7575VaultUpgradeable` transfer assets to the specified `receiver` address using `SafeTokenTransfers.safeTransfer`. However, there is no explicit check that `receiver` is not `address(0)`. While standard ERC20s revert on transfer to zero, some tokens may allow it (effectively burning funds) or handle it unexpectedly. If a user accidentally specifies `address(0)` (or if it defaults to zero in a UI), the claimed assets could be permanently lost.
 ### Static Signals
no zero-address guard, transfer to receiver argument
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Accounting invariant violation in investment valuation
 ### Description/Code Snippet
The `ShareTokenUpgradeable` calculates `totalNormalizedAssets` by adding `_calculateInvestmentAssets()`, which uses the raw `balanceOf` the investment share token (plus `rBalance`). This assumes a 1:1 price ratio between the investment share token and the normalized asset. If the `investmentShareToken` is a standard yield-bearing ERC4626 token where the share price appreciates (price > 1), the system will undervalue the invested assets, causing the `ShareTokenUpgradeable` share price to be incorrect. This allows arbitrage and potential value loss for redeeming users.
 ### Static Signals
totalNormalizedAssets += _calculateInvestmentAssets(), balanceOf(investmentShareToken)
 ### Assets at Risk
share valuation
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: ShareTokenUpgradeable.transfer

 ### Title
Missing KYC Validation in ShareTokenUpgradeable
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract inherits standard ERC20 transfer logic without overriding it to enforce KYC checks (unlike `WERC7575ShareToken`). This allows KYC-verified users to transfer investment shares (IUSD) to non-KYC addresses. These non-KYC addresses can then `requestRedeem` and claim underlying assets, effectively bypassing the protocol's regulatory compliance and gating mechanisms.
 ### Static Signals
no modifier, missing check, inheritance without override
 ### Assets at Risk
compliance status, underlying assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
ReadOnlyReentrancy via requestDeposit state update ordering
 ### Description/Code Snippet
In `requestDeposit`, the contract calls `SafeTokenTransfers.safeTransferFrom` to pull assets before updating the `totalPendingDepositAssets` state variable. `safeTransferFrom` triggers token transfer logic, which for tokens with hooks (like ERC777 sender hooks) or malicious tokens updates the vault's `balanceOf` before the function returns. 

The `totalAssets()` function calculates `balanceOf(this) - totalPendingDepositAssets - ...`. During the hook execution, `balanceOf` is increased but `totalPendingDepositAssets` is not yet updated. This causes `totalAssets()` to be transiently inflated. 

Since `ShareToken` pricing and conversion functions rely on `totalAssets()`, an attacker can exploit this via a read-only reentrancy attack to read an inflated share price or inflated `totalNormalizedAssets` value, potentially exploiting external protocols that rely on this vault's valuation.
 ### Static Signals
external call before state update, totalAssets relies on balanceOf and state variable
 ### Assets at Risk
vault valuation, external protocol funds relying on price
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Missing slippage protection in investment transactions
 ### Description/Code Snippet
The `investAssets` function deposits assets into an external `investmentVault` but fails to validate that the number of shares received meets a minimum threshold. Similarly, `withdrawFromInvestment` calculates shares to redeem using `previewWithdraw` and executes `redeem` without verifying that the actual assets received match the expectation. If the external vault's exchange rate is manipulated (e.g. via donation/inflation attacks) or unfavorable at execution time, the vault may suffer significant loss of value.
 ### Static Signals
shares = deposit(...) without min output check, redeem output not checked against minimum, payout calculated at execution time without minimum bound
 ### Assets at Risk
assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Slippage Missing in Async Deposit/Redeem
 ### Description/Code Snippet
The asynchronous `requestDeposit` and `requestRedeem` functions in `ERC7575VaultUpgradeable` lock user assets/shares without allowing the user to specify a minimum output amount (`minShares` or `minAssets`). The actual conversion rate is determined later when the Investment Manager calls `fulfillDeposit`/`fulfillRedeem`, exposing users to unlimited slippage if the share price changes unfavorably between request and fulfillment.
 ### Static Signals
requestDeposit(...) returns (uint256 requestId), no minShares parameter, fulfillDeposit(...) determines rate
 ### Assets at Risk
user deposits, user redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Accounting Invariant Violation via Asset Clamping
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, `totalAssets()` is calculated as `balance > reservedAssets ? balance - reservedAssets : 0`. `reservedAssets` includes `totalClaimableRedeemAssets`. When the Investment Manager calls `fulfillRedeem`, shares are converted to assets and added to `totalClaimableRedeemAssets`. If the vault's local `balance` is low (due to funds being invested in external vaults), `reservedAssets` can exceed `balance`. The clamping to 0 in `totalAssets()` hides this local liability deficit. 

However, `ShareTokenUpgradeable` calculates `totalNormalizedAssets` by summing `vault.totalAssets()` and `getInvestedAssets()`. When `vault.totalAssets()` clamps to 0 instead of reflecting the negative net position (`balance - reserved`), the global `totalNormalizedAssets` becomes overstated (equal to `InvestedAssets` without deducting the local deficit). This inflates the share price. 

Consequences:
1. Subsequent `fulfillRedeem` calls use the inflated price, promising more assets to redeemers than the vault possesses, draining value from remaining shareholders.
2. New deposits via `requestDeposit` -> `fulfillDeposit` will receive fewer (or zero) shares due to the inflated price, causing immediate loss of value.
 ### Static Signals
balance > reservedAssets ? balance - reservedAssets : 0, totalClaimableRedeemAssets += assets
 ### Assets at Risk
User deposits, Shareholder equity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Total Assets Manipulation via Donation
 ### Description/Code Snippet
The `totalAssets()` function calculates the vault's managed assets based on `IERC20(asset).balanceOf(address(this)) - reservedAssets`. It does not track deposit inflows via strict internal accounting. This allows an attacker to donate assets directly to the vault, artificially inflating `totalAssets`. Since `ShareTokenUpgradeable` relies on `totalAssets` to calculate the global share price, a donation can manipulate the exchange rate for all users. While `VIRTUAL_SHARES` mitigates the profitability of an inflation attack, the ability to externally manipulate the protocol's core accounting and share price invariant remains a vulnerability pattern.
 ### Static Signals
balance tracking references different token than actual holdings, supply != sum(balances)
 ### Assets at Risk
user shares value
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

