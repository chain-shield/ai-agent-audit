## Verified Patterns Found: 26

## Verified Patterns Found in following Categories:

- AccountingInvariantViolation
- FlashLoanEconomicManipulation
- ForcedAssetVsStrictEquality
- PrecisionDriftAccumulation
- StateGrowthOrStorageBloat
- ReserveOrPriceDesync
- ERC4626SharePriceMismatch
- MaturityorGatingByPass
- UnsafeRecipient
- BeaconOrFactoryAuthorityDrift
- ReadOnlyReentrancy
- SlippageMissingOrInsufficient
- StandardViolation
- GriefableCallbacks



## Summary of Patterns

Cross-Vault Inflation/Donation Attack via Shared Share Token

Use of non-upgradeable ReentrancyGuard in upgradeable contracts

Missing slippage protection in async requests and investment operations

Missing slippage protection in investment withdrawal

Missing slippage protection in fulfillDeposit enables sandwich attacks via donation

Storage collision risk due to non-upgradeable ReentrancyGuard in upgradeable contract

ERC4626 Standard Violation in Deposit Flow

Permanent DoS of unregisterVault via Dust Donation

Read-Only Reentrancy via totalAssets Manipulation

Non-Standard ERC20 Implementation Breaks Wallet and DeFi Integrations

Non-standard ERC20 behavior blocks self-approval

Missing slippage protection in investAssets exposes vault to value loss

Strict balance equality check enables DoS

Dust Accounts Permanently Block Vault Unregistration

Incorrect rounding direction in withdraw and mint functions

Share Inflation Attack due to Insufficient Virtual Offset

Batch Fulfillment DoS via Front-Run Cancelation

Incorrect Asset Valuation in ShareTokenUpgradeable

Unsafe recipient address in claim functions

Investment Withdrawal Deadlock due to Self-Allowance Restriction

Yield leakage via 1:1 Investment Share assumption

Assets Lost if Withdrawn to Zero Address

Yield realization logic in adjustrBalance contradicts documentation

Silent Truncation of Restricted Balance in rBatchTransfers

Async Deposit/Redeem Missing Slippage Protection

Missing slippage protection in investment operations

## Patterns



 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Cross-Vault Inflation/Donation Attack via Shared Share Token
 ### Description/Code Snippet
The `ShareTokenUpgradeable` calculates the share price globally by aggregating total assets and total supply across all registered vaults (`getCirculatingSupplyAndAssets`). An attacker can donate assets to an empty or low-liquidity vault (e.g., a newly registered one) to inflate the global `totalNormalizedAssets` without minting new shares. This artificially increases the share price for *all* vaults sharing the token. While 'virtual shares' mitigate the divide-by-zero case, a large donation to one vault affects the exchange rate of others, potentially allowing value extraction or griefing of pending requests in other vaults.
 ### Static Signals
aggregates state from multiple vaults, totalNormalizedAssets calculated from balance, share price affects all vaults
 ### Assets at Risk
vault assets, shareholder value
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.N/A

 ### Title
Use of non-upgradeable ReentrancyGuard in upgradeable contracts
 ### Description/Code Snippet
Both `ERC7575VaultUpgradeable` and `ShareTokenUpgradeable` import and inherit the non-upgradeable `ReentrancyGuard` from OpenZeppelin instead of `ReentrancyGuardUpgradeable`. In upgradeable contracts using namespaced storage (ERC-7201), non-upgradeable storage variables (like `_status` in `ReentrancyGuard`) occupy slot 0. This breaks the intended storage layout safety of UUPS proxies and poses a high risk of storage collision if future upgrades introduce variables or mixins that also utilize slot 0 or expect namespaced storage.
 ### Static Signals
import .../ReentrancyGuard.sol, is ReentrancyGuard
 ### Assets at Risk
vault state, governance
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit, requestRedeem, investAssets, withdrawFromInvestment

 ### Title
Missing slippage protection in async requests and investment operations
 ### Description/Code Snippet
The `requestDeposit` and `requestRedeem` functions lock assets/shares in a pending state without a minimum return parameter (`minShares` or `minAssets`), exposing users to unfavorable exchange rate fluctuations between request and fulfillment. Additionally, `investAssets` and `withdrawFromInvestment` execute trades with the investment vault without minimum output bounds, exposing the vault to sandwich attacks.
 ### Static Signals
requestDeposit(..., assets), investAssets(..., amount), minOut missing
 ### Assets at Risk
user funds, vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdrawFromInvestment

 ### Title
Missing slippage protection in investment withdrawal
 ### Description/Code Snippet
The withdrawFromInvestment function converts a requested asset amount to shares using 'previewWithdraw' (current rate) and immediately redeems those shares. It lacks a minimum output parameter to ensure the actual assets received match the requested amount, exposing the vault to slippage or sandwich attacks on the underlying investment vault during execution.
 ### Static Signals
previewWithdraw used to calculate shares for redeem, redeem call without minAssets check, output calculated from balance change without floor
 ### Assets at Risk
invested assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillDeposit

 ### Title
Missing slippage protection in fulfillDeposit enables sandwich attacks via donation
 ### Description/Code Snippet
The `fulfillDeposit` function converts pending assets to shares using the current exchange rate (`totalAssets` / `totalSupply`). This rate is calculated dynamically based on the vault's asset balance. An attacker can manipulate this rate by donating assets to the vault (increasing `totalAssets` without minting shares) right before the Investment Manager calls `fulfillDeposit`. Since `fulfillDeposit` lacks a `minShares` or `maxAssets` parameter to enforce slippage limits, the user receives significantly fewer shares than expected, effectively stealing value from their deposit.
 ### Static Signals
_convertToShares(assets, Math.Rounding.Floor), no minShares parameter, spot price usage
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: ERC7575VaultUpgradeable.N/A

 ### Title
Storage collision risk due to non-upgradeable ReentrancyGuard in upgradeable contract
 ### Description/Code Snippet
ERC7575VaultUpgradeable inherits from the non-upgradeable OpenZeppelin ReentrancyGuard ('import {ReentrancyGuard} ...'). This causes the '_status' variable to reside at storage slot 0, conflicting with the namespaced storage pattern used elsewhere (ERC-7201) and posing a high risk of storage collision during future upgrades if state variables are added or if the inheritance chain changes.
 ### Static Signals
inherits ReentrancyGuard instead of ReentrancyGuardUpgradeable, ERC7575VaultUpgradeable is Initializable, ReentrancyGuard
 ### Assets at Risk
vault state integrity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.deposit

 ### Title
ERC4626 Standard Violation in Deposit Flow
 ### Description/Code Snippet
The contract claims to be an ERC4626 vault (`IERC7575` inherits `IERC4626`) but the `deposit(uint256 assets, address receiver)` function deviates significantly from the standard. Instead of initiating a deposit (transferring assets and minting shares), it attempts to **claim** shares from a previously fulfilled request via `deposit(assets, receiver, receiver)`. This reverts if the user has no claimable shares, breaking integrations (e.g., routers, adapters) that rely on the standard synchronous `deposit` behavior defined in ERC4626.
 ### Static Signals
deposit calls internal claim logic, reverts on valid assets but no prior request
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Permanent DoS of unregisterVault via Dust Donation
 ### Description/Code Snippet
The `unregisterVault` function strictly enforces `IERC20(asset).balanceOf(vaultAddress) == 0`. An attacker can send 1 wei of the asset to the vault (direct transfer), making the balance non-zero permanently (as there is no sweep function). This reverts `unregisterVault`, preventing the removal of the vault. Since `registerVault` caps the number of vaults at 10 (`MAX_VAULTS_PER_SHARE_TOKEN`), an attacker can brick all available slots, permanently preventing the protocol from adding new assets.
 ### Static Signals
balanceOf(vaultAddress) != 0, revert CannotUnregisterVaultAssetBalance
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Read-Only Reentrancy via totalAssets Manipulation
 ### Description/Code Snippet
The `totalAssets()` function in `ERC7575VaultUpgradeable` relies on `IERC20(asset).balanceOf(address(this))` minus reserved assets. In `requestDeposit`, funds are pulled via `safeTransferFrom` *before* the internal accounting `totalPendingDepositAssets` is updated. If the underlying asset supports transfer hooks (e.g., ERC777 `tokensToSend`, or similar extensions), a malicious sender can re-enter the system during the hook. At this point, the vault's balance has increased, but the `reservedAssets` (pending deposits) have not yet been incremented. Consequently, `totalAssets()` reports an inflated value. Since `ShareTokenUpgradeable` calculates the global share price based on `totalAssets()` of all vaults, this transient inflation manipulates the exchange rate. External protocols using the ShareToken as collateral or an oracle could be exploited during this inconsistent state.
 ### Static Signals
balanceOf(this) used in totalAssets, state update after transferFrom, no reentrancy guard on view functions
 ### Assets at Risk
External protocol funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.transfer, approve

 ### Title
Non-Standard ERC20 Implementation Breaks Wallet and DeFi Integrations
 ### Description/Code Snippet
The `WERC7575ShareToken` deliberately deviates from the ERC20 standard in `approve` (reverting if `spender == msg.sender`) and `transfer` (requiring pre-existing self-allowance via validator permit). This breaks compatibility with all standard ERC20 wallets (MetaMask, Ledger) and DeFi protocols (Uniswap, Aave) that assume `transfer` relies solely on balance ownership. While documented as intentional, this creates a 'Stuck Funds' risk for users attempting to use standard interfaces.
 ### Static Signals
revert ERC20InvalidSpender, requires self-allowance permit
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.approve

 ### Title
Non-standard ERC20 behavior blocks self-approval
 ### Description/Code Snippet
The `approve` function in `WERC7575ShareToken` explicitly reverts if `msg.sender == spender`. This deviation from the ERC-20 standard breaks composability with protocols and tools that rely on self-approval patterns (e.g., certain routers or account abstraction layers) or generic ERC-20 wrappers, potentially causing integration failures.
 ### Static Signals
msg.sender == spender revert
 ### Assets at Risk
integration compatibility
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Missing slippage protection in investAssets exposes vault to value loss
 ### Description/Code Snippet
The `investAssets` function deposits vault assets into an external `investmentVault` (an `IERC7575` interface) and receives shares in return. The function does not accept a `minShares` parameter to enforce a minimum exchange rate. If the `investmentVault` uses a floating exchange rate (e.g., based on `totalAssets`), an attacker could manipulate the rate via donation or flash loan front-running, causing the `investAssets` call to return fewer investment shares than expected, leading to a loss of vault value.
 ### Static Signals
investmentVault.deposit(amount), no minShares parameter
 ### Assets at Risk
vault treasury
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: SafeTokenTransfers.safeTransferFrom

 ### Title
Strict balance equality check enables DoS
 ### Description/Code Snippet
The `SafeTokenTransfers` library enforces `balanceAfter == balanceBefore + amount` during transfers. This reverts if the balance change is not exact. An attacker can cause a DoS on deposits or withdrawals if the underlying asset allows transfer hooks (e.g., ERC777, ERC677) by sending dust to the recipient during the transfer callback, triggering the strict equality check failure.
 ### Static Signals
balanceAfter != balanceBefore + amount, revert TransferAmountMismatch()
 ### Assets at Risk
availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: ERC7575VaultUpgradeable.deposit

 ### Title
Dust Accounts Permanently Block Vault Unregistration
 ### Description/Code Snippet
The `ShareTokenUpgradeable.unregisterVault` function strictly requires `activeDepositRequestersCount == 0`. In `ERC7575VaultUpgradeable`, a requester is removed from this set only when they fully claim their assets. A malicious user can make a valid deposit (meeting minimums), have it fulfilled, and then claim all but 1 wei of assets. This keeps them in the `activeDepositRequesters` set indefinitely with negligible cost, permanently preventing the `unregisterVault` operation.
 ### Static Signals
state enumeration via unbounded array traversal, append-only arrays with no pruning
 ### Assets at Risk
protocol maintainability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw, mint

 ### Title
Incorrect rounding direction in withdraw and mint functions
 ### Description/Code Snippet
The `withdraw` function calculates the shares to burn using `Math.Rounding.Floor` instead of `Ceil`, allowing users to withdraw small amounts of assets for 0 shares (free withdrawal). Similarly, the `mint` function calculates the assets to take using `Math.Rounding.Floor` instead of `Ceil`, allowing users to mint shares for slightly fewer assets than the exchange rate dictates.
 ### Static Signals
shares = assets.mulDiv(..., Math.Rounding.Floor), assets = shares.mulDiv(..., Math.Rounding.Floor)
 ### Assets at Risk
assets, shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ShareTokenUpgradeable.convertNormalizedAssetsToShares

 ### Title
Share Inflation Attack due to Insufficient Virtual Offset
 ### Description/Code Snippet
The `convertNormalizedAssetsToShares` function adds `VIRTUAL_ASSETS` (1e6) to `totalNormalizedAssets` (18 decimals) to prevent inflation attacks. However, 1e6 is negligible compared to the 18-decimal normalized scale (e.g., 1 USDC = 1e18 normalized). This insufficient offset allows an attacker to drain a vault to dust, donate assets to inflate the share price massively, and cause subsequent depositors (victims) to receive zero shares due to rounding, effectively stealing their deposits.
 ### Static Signals
circulatingSupply += VIRTUAL_SHARES, totalNormalizedAssets += VIRTUAL_ASSETS, Math.mulDiv
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillDeposits

 ### Title
Batch Fulfillment DoS via Front-Run Cancelation
 ### Description/Code Snippet
The `fulfillDeposits` function allows the Investment Manager to process multiple deposit requests in a single transaction. However, it calls `fulfillDeposit`, which strictly reverts if `assets > pendingDepositAssets[controller]`. A malicious user can submit a deposit request and, upon observing a pending `fulfillDeposits` transaction in the mempool, call `cancelDepositRequest` to set their pending assets to zero (or reduce them). This causes `fulfillDeposit` to revert, thereby causing the entire batch transaction to fail. This allows a single user to grief the Investment Manager and block efficient batch processing.
 ### Static Signals
loop calling potentially reverting function, no try/catch inside loop, state change via front-running
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ShareTokenUpgradeable._calculateInvestmentAssets

 ### Title
Incorrect Asset Valuation in ShareTokenUpgradeable
 ### Description/Code Snippet
In `ShareTokenUpgradeable._calculateInvestmentAssets`, the protocol calculates the value of invested assets by reading `IERC20(investmentShareToken).balanceOf(address(this))`. This assumes a 1:1 exchange rate between investment shares and underlying assets. If `investmentVault` is a standard ERC4626 vault (or any vault where share price != 1, e.g., due to yield accumulation not using rebasing), the protocol will under-report total assets, causing an incorrect (deflated) share price for the Upgradeable Vaults. This leads to value loss for depositors.
 ### Static Signals
balanceOf used as assets, missing convertToAssets
 ### Assets at Risk
vault share price, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.deposit, mint, redeem, withdraw

 ### Title
Unsafe recipient address in claim functions
 ### Description/Code Snippet
The `deposit`, `mint`, `redeem`, and `withdraw` functions accept a `receiver` address but do not validate that it is non-zero. If `address(0)` is provided, shares or assets will be sent to the zero address and permanently lost.
 ### Static Signals
receiver argument not checked against address(0)
 ### Assets at Risk
assets, shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdrawFromInvestment

 ### Title
Investment Withdrawal Deadlock due to Self-Allowance Restriction
 ### Description/Code Snippet
The investment layer (`ERC7575VaultUpgradeable`) is designed to invest assets into the settlement layer (`WERC7575Vault`). However, `WERC7575ShareToken` (the settlement share token) enforces a non-standard security model where `approve(self, amount)` reverts, and self-allowance must be set via `permit()` signed by a validator. `ERC7575VaultUpgradeable` (and its `ShareTokenUpgradeable` owner) has no mechanism to collect a validator signature and call `permit()` on the investment share token. Consequently, `withdrawFromInvestment` will revert when `WERC7575Vault` calls `spendSelfAllowance`, permanently locking invested funds.
 ### Static Signals
spendSelfAllowance, approve(msg.sender) revert, missing permit call
 ### Assets at Risk
treasury, invested assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Yield leakage via 1:1 Investment Share assumption
 ### Description/Code Snippet
ShareTokenUpgradeable sums `balanceOf(investmentShareToken)` directly into `totalNormalizedAssets`, assuming 1 investment share equals 1 normalized asset. If the investment vault is a standard compounding ERC4626, its share price increases over time. The protocol ignores this appreciation, undervaluing the share price and allowing arbitrageurs to steal yield by depositing before `withdrawFromInvestment` (which realizes the value) and redeeming after.
 ### Static Signals
totalNormalizedAssets += _calculateInvestmentAssets(), uses balanceOf for share value, no exchange rate synchronization
 ### Assets at Risk
yield, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Assets Lost if Withdrawn to Zero Address
 ### Description/Code Snippet
The `withdraw` and `redeem` functions in `ERC7575VaultUpgradeable` (and `WERC7575Vault`) do not validate that the `receiver` address is non-zero. They rely on `SafeTokenTransfers.safeTransfer` (wrapping `SafeERC20`). While most ERC20 tokens revert on transfer to address(0), some do not (or implement burn semantics). If such an asset is used, calling `withdraw` with `receiver = address(0)` will burn the shares and the assets without reverting, leading to permanent fund loss.
 ### Static Signals
no zero-address check, safeTransfer to user input
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Yield realization logic in adjustrBalance contradicts documentation
 ### Description/Code Snippet
The `adjustrBalance` function in `WERC7575ShareToken` adds profit to `_rBalances` (Restricted) instead of `_balances` (Liquid) as described in the system documentation. The docs state: `_rBalances[account] -= amounti; _balances[account] += amountr;` (moving principal + profit to liquid). The code implements: `_rBalances[account] += (amountr - amounti)`. This keeps the profit locked in a restricted state, preventing the Investment Manager from withdrawing/realizing the yield without manual Validator intervention via `rBatchTransfers`, creating a risk of fund lockup.
 ### Static Signals
_rBalances[account] += difference, amountr > amounti
 ### Assets at Risk
yield
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Silent Truncation of Restricted Balance in rBatchTransfers
 ### Description/Code Snippet
In `rBatchTransfers`, if an account receives a credit (net inflow) and is flagged for rBalance update, the code decreases `_rBalances`. If `_rBalances[account] < amount`, it silently truncates `_rBalances` to 0. This violates the invariant that `rBalance` tracks invested capital, potentially corrupting future yield distribution calculations in `adjustrBalance` which rely on accurate rBalance tracking.
 ### Static Signals
_rBalances[account.owner] = 0, unchecked, rBalanceFlags
 ### Assets at Risk
yield/rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Async Deposit/Redeem Missing Slippage Protection
 ### Description/Code Snippet
The `requestDeposit` and `requestRedeem` functions initiate asynchronous operations where the exchange rate is determined later upon fulfillment by the Investment Manager. However, these functions lack a `minShares` or `minAssets` parameter to bound the acceptable exchange rate. Users are exposed to unlimited slippage if the share price changes unfavorably between the request and the fulfillment.
 ### Static Signals
amountOutMin=0 or missing, price fetched at execution without user-specified floor
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Missing slippage protection in investment operations
 ### Description/Code Snippet
`investAssets` calls `deposit` on an external vault without a `minShares` check. `withdrawFromInvestment` calculates shares via `previewWithdraw` and immediately calls `redeem` without a `maxShares` or slippage bound. Both functions are vulnerable to sandwich attacks on the external investment vault, potentially causing loss of principal during deployment or withdrawal.
 ### Static Signals
deposit(amount, ...), redeem(shares, ...), no minAmountOut/minShares parameter
 ### Assets at Risk
vault funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole

