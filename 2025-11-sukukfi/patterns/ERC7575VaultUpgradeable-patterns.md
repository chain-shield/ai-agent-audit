## Verified Patterns Found: 25

## Verified Patterns Found in following Categories:

- GovernanceDelegationFlaw
- SlippageMissingOrInsufficient
- GriefableCallbacks
- AccessControlOrAuthByPass
- Reentrancy
- StandardViolation
- FlashLoanEconomicManipulation
- UnsafeRecipient
- AccountingInvariantViolation
- ForcedAssetVsStrictEquality
- StateGrowthOrStorageBloat
- PrecisionDriftAccumulation
- FeeOnTransferAssumption
- ReadOnlyReentrancy



## Summary of Patterns

Share Price Manipulation via Donation

SlippageMissingOrInsufficient in withdrawFromInvestment

Unsafe Recipient in Deposit Request

StateGrowthOrStorageBloat causing DoS on Unregister

Missing Zero Address Check for Recipient

Share Price Manipulation via Rounding in withdraw()

Potential Asset Lock in Cancelation Flow

Usage of Non-Upgradeable ReentrancyGuard in Upgradeable Contract

Standard Violation: ERC-4626 Max Methods Repurposed

rBalance Truncation Breaks Accounting Invariant

Forced Asset Balance Blocks Vault Unregistration

Batch Revenue Adjustment Denial of Service

Yield Trapped in rBalance due to Logic Mismatch

Unauthorized Redemption via WERC7575Vault.redeem

Governance Delegation Flaw: Cross-Vault Operator Scope

Permanent DoS of Vault Registry via Forced Donation

System-Wide DoS via Single Vault Failure in ShareToken Loop

Reentrancy: State Update after External Transfer

Incompatibility with Fee-on-Transfer Tokens

ReadOnlyReentrancy in requestDeposit via Token Hook

Unsafe Asset Transfer to Zero Address

Standard Violation: ERC-4626 Preview Functions Revert

Accounting Invariant Violation: Reserved Asset Calculation Mismatch

Missing Slippage Protection in investAssets

No Slippage Protection for Async Requests

## Patterns



 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Share Price Manipulation via Donation
 ### Description/Code Snippet
The ShareToken's price (calculated via `getCirculatingSupplyAndAssets` which relies on `ERC7575VaultUpgradeable.totalAssets`) can be manipulated by donating assets to the vault. `totalAssets()` uses the spot `balanceOf(address(this))`. An attacker can flash-donate assets to inflate the share price within a transaction. While the vault itself protects against extraction via slippage, this manipulable price exposes external protocols (lending markets, etc.) relying on this ShareToken as an oracle to economic attacks.
 ### Static Signals
uses totalSupply/totalAssets in same tx as deposit/withdraw, share price derived from manipulable pool state
 ### Assets at Risk
external protocol funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdrawFromInvestment

 ### Title
SlippageMissingOrInsufficient in withdrawFromInvestment
 ### Description/Code Snippet
The withdrawFromInvestment function redeems shares from an external investment vault to obtain a target amount of assets. It calculates the shares to burn using previewWithdraw (based on current state) and executes redeem without a maximum shares limit. If the share price is manipulated (lowered) before execution, the vault may burn significantly more shares than necessary to retrieve the assets.
 ### Static Signals
shares = IERC7575($.investmentVault).previewWithdraw(amount);, IERC7575($.investmentVault).redeem(minShares, ...), no maxShares parameter
 ### Assets at Risk
vault shares in investment vault
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Unsafe Recipient in Deposit Request
 ### Description/Code Snippet
The `requestDeposit` function allows setting the `controller` to `address(0)`. If this happens, the assets are transferred to the vault and added to `pendingDepositAssets[address(0)]`. However, the `deposit` (claim) function requires `msg.sender` to be the controller (impossible for address(0)) or an approved operator (impossible to approve for address(0)). Consequently, assets deposited with `controller = address(0)` are permanently locked.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: ERC7575VaultUpgradeable.unregisterVault

 ### Title
StateGrowthOrStorageBloat causing DoS on Unregister
 ### Description/Code Snippet
The `unregisterVault` function reverts if `activeDepositRequestersCount != 0`. Users are added to `activeDepositRequesters` upon calling `requestDeposit` but are only removed in `deposit` (claim) when fully claimed. An attacker can spam small deposit requests and never claim them, permanently populating `activeDepositRequesters` and preventing the admin from ever unregistering the vault. The admin lacks a mechanism to force-claim or prune these requests.
 ### Static Signals
mapping enumerations via arrays, append-only arrays with no pruning
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Missing Zero Address Check for Recipient
 ### Description/Code Snippet
The `redeem` and `withdraw` functions in `ERC7575VaultUpgradeable` do not explicitly check if the `receiver` address is non-zero before attempting a transfer. While `SafeTokenTransfers` calls the token's transfer function, some ERC20 tokens may allow transfers to the zero address (effectively burning), leading to permanent loss of user funds if the zero address is passed accidentally.
 ### Static Signals
receiver argument not checked against address(0)
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Share Price Manipulation via Rounding in withdraw()
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.withdraw`, the calculation of shares to burn uses `Math.Rounding.Floor`. An attacker can repeatedly call `withdraw` with small asset amounts such that the calculated `shares` amount rounds down to zero. This allows the attacker to withdraw assets without burning the corresponding `claimableRedeemShares`. This action reduces the vault's `totalAssets` (and thus `totalNormalizedAssets`) while keeping `totalClaimableRedeemShares` (which are subtracted from supply) constant, or effectively keeping the circulating supply constant while draining assets. This leads to a decrease in the global share price, enabling profit via arbitrage in other vaults or effectively stealing value from other share holders.
 ### Static Signals
shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor), if (shares > 0) ShareToken.burn
 ### Assets at Risk
vault assets, share price integrity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ERC7575VaultUpgradeable.cancelDepositRequest

 ### Title
Potential Asset Lock in Cancelation Flow
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, calling `cancelDepositRequest` adds the controller to `controllersWithPendingDepositCancelations`, which blocks any further `requestDeposit` calls via the `DepositCancelationPending` check. This state persists until `claimCancelDepositRequest` is called. If the Investment Manager fulfills the cancelation but the user (or their operator) fails to claim it, or if the Investment Manager delays fulfillment indefinitely, the user is permanently blocked from making new deposits. This creates a state-dependent Denial of Service vector where a user can become locked out of the protocol's deposit functionality.
 ### Static Signals
controllersWithPendingDepositCancelations.add, revert DepositCancelationPending
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.N/A

 ### Title
Usage of Non-Upgradeable ReentrancyGuard in Upgradeable Contract
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` contract inherits from the standard, non-upgradeable `@openzeppelin/contracts/utils/ReentrancyGuard.sol` instead of `ReentrancyGuardUpgradeable`. 

In the non-upgradeable version, the constructor initializes the status to `NOT_ENTERED` (1). However, in an upgradeable proxy context, constructors are not executed, leaving the storage slot uninitialized (0). While the current implementation of `nonReentrant` coincidentally works with 0 (since 0 != `ENTERED` (2)), this relies on implementation details that may change and violates the standard proxy initialization pattern, potentially leading to locked functions if the library logic changes.
 ### Static Signals
import .../utils/ReentrancyGuard.sol, is Initializable, no __ReentrancyGuard_init
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.maxDeposit

 ### Title
Standard Violation: ERC-4626 Max Methods Repurposed
 ### Description/Code Snippet
The `maxDeposit` and `maxMint` functions are implemented to return `claimableDepositAssets` and `claimableDepositShares` respectively. In standard ERC-4626, these functions should report the maximum amount a user can *newly* deposit. Repurposing them to report *claimable* amounts for the async flow violates the standard's semantic meaning, potentially causing integrators to believe the vault is full (max=0) or allowing incorrect logic execution.
 ### Static Signals
maxDeposit returns claimable assets, Semantic mismatch with ERC4626
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
rBalance Truncation Breaks Accounting Invariant
 ### Description/Code Snippet
In `WERC7575ShareToken.rBatchTransfers`, if a user receives a credit that exceeds their current `_rBalances` (invested balance), the function silently truncates `_rBalances` to zero. This violates the accounting invariant expected by `adjustrBalance`, which assumes `_rBalances` accurately tracks invested capital. If `adjustrBalance` is subsequently called with `amounti` (original investment) greater than the truncated `_rBalances`, the subtraction `_rBalances[account] -= amounti` will underflow and revert, effectively bricking the yield distribution mechanism for that account.
 ### Static Signals
_rBalances[account.owner] = 0, if (rbalance < amount)
 ### Assets at Risk
yield distribution capability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Forced Asset Balance Blocks Vault Unregistration
 ### Description/Code Snippet
The `ShareTokenUpgradeable.unregisterVault` function strictly requires `IERC20(asset).balanceOf(vaultAddress) == 0`. An attacker can send 1 wei of the asset to the vault contract (force-feeding). While `investAssets` can theoretically sweep funds, it requires the investment vault to accept dust and the investment flow to be fully functional. If the investment vault is paused or has limits, the manager cannot empty the vault to exactly zero, causing `unregisterVault` to revert permanently. This allows a griefer to prevent the unregistration of deprecated or compromised vaults.
 ### Static Signals
IERC20(asset).balanceOf(vaultAddress) != 0
 ### Assets at Risk
protocol administration
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Batch Revenue Adjustment Denial of Service
 ### Description/Code Snippet
In `WERC7575ShareToken.adjustrBalance`, the function iterates over a batch of accounts to apply yield adjustments (profit or loss). It checks `if (currentRBalance < difference) revert RBalanceAdjustmentTooLarge()`. If a user has exited their position via `rBatchTransfers` (which reduces `_rBalances` to 0 upon withdrawal), their `rBalance` will be 0. If the revenue admin subsequently attempts to apply a loss adjustment for a batch that includes this user, the transaction will revert due to underflow check. This allows a single user's exit to block the revenue recording for an entire batch of users, causing a Denial of Service for the administrative accounting function.
 ### Static Signals
currentRBalance < difference, revert RBalanceAdjustmentTooLarge
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Yield Trapped in rBalance due to Logic Mismatch
 ### Description/Code Snippet
The `adjustrBalance` function in `WERC7575ShareToken.sol` (lines 503-538) is intended to record investment returns. According to protocol documentation (Scenario 1), this function should settle investments by moving funds from `_rBalances` (reserved) to `_balances` (liquid). However, the implementation only updates `_rBalances` (increments for profit, decrements for loss) and leaves `_balances` unchanged. 

As a result, 'returned' yield (`amountr`) remains trapped in the illiquid `_rBalances` state. Since the `burn` function used during redemption only spends from `_balances` (lines 280-282), investors cannot withdraw this yield via standard flows. This creates a state where the system records valuation gains (increasing share price) but holds no liquid assets to service the resulting redemptions, requiring manual intervention via `rBatchTransfers` to resolve.
 ### Static Signals
_rBalances[account] += difference, no _balances update, documentation mismatch
 ### Assets at Risk
yield, investment returns
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: WERC7575Vault.redeem

 ### Title
Unauthorized Redemption via WERC7575Vault.redeem
 ### Description/Code Snippet
The `redeem` function in `WERC7575Vault` allows any caller to redeem shares belonging to an arbitrary `owner` address, provided that `owner` has set a self-allowance (which is a required operational state for `WERC7575ShareToken` users). The function calls `_withdraw`, which calls `_shareToken.spendSelfAllowance(owner, shares)`. This strictly checks `allowance[owner][owner]` but fails to verify if `msg.sender` is the `owner` or has a specific allowance from the `owner` (`allowance[owner][msg.sender]`). An attacker can drain the assets of any user (including the Investment Layer's `ShareTokenUpgradeable`) who has an active self-allowance.
 ### Static Signals
public function with owner parameter, missing check for msg.sender == owner, missing _spendAllowance(owner, msg.sender, ...)
 ### Assets at Risk
Vault assets (USDC, etc.), Invested capital from ShareTokenUpgradeable
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GovernanceDelegationFlaw

 ### Relevant Function/Location: ERC7575VaultUpgradeable.setOperator

 ### Title
Governance Delegation Flaw: Cross-Vault Operator Scope
 ### Description/Code Snippet
The `setOperator` function delegates operator status to the shared `ShareToken` contract. Because multiple vaults (e.g., USDC Vault, DAI Vault) share the same `ShareToken`, setting an operator on one vault grants that operator privileges across *all* vaults in the system for that user. A user intending to delegate access only for a specific asset vault inadvertently exposes their positions in all other asset vaults to the same operator.
 ### Static Signals
Delegation to shared central registry, Scope of operator exceeds individual vault
 ### Assets at Risk
User assets in other vaults sharing the same ShareToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: WERC7575ShareToken.unregisterVault

 ### Title
Permanent DoS of Vault Registry via Forced Donation
 ### Description/Code Snippet
In `WERC7575ShareToken.unregisterVault`, the function enforces a strict equality check `totalAssets != 0` via the vault interface before allowing unregistration. The corresponding `WERC7575Vault` implementation returns `asset.balanceOf(address(this))` for `totalAssets()` and lacks any mechanism to sweep or remove excess assets (shares can only be burned to withdraw, so if share supply is 0, assets cannot be withdrawn). An attacker can transfer 1 wei of the asset directly to the vault, making `totalAssets() > 0` permanently. This causes `unregisterVault` to always revert. Since the registry size is strictly limited to 10 vaults (`MAX_VAULTS_PER_SHARE_TOKEN`), an attacker can brick all available slots, permanently preventing the protocol from managing or replacing vaults.
 ### Static Signals
if (totalAssets != 0) revert, balanceOf(address(this)), no sweep/rescue function
 ### Assets at Risk
Protocol Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
System-Wide DoS via Single Vault Failure in ShareToken Loop
 ### Description/Code Snippet
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults to aggregate `getClaimableSharesAndNormalizedAssets`. This function calls `totalAssets()` on each vault, which in turn calls `balanceOf(address(this))` on the underlying asset. If a single underlying asset reverts (e.g., paused USDC, broken proxy, or malicious token) or if one vault malfunctions, the entire loop reverts. This function is critical for share price calculation (`convertNormalizedAssetsToShares`), meaning a single failure bricks deposit, redemption, and fulfillment operations for ALL vaults in the multi-asset system.
 ### Static Signals
loop over external calls, no try/catch inside loop, critical system function depends on all external components
 ### Assets at Risk
Protocol Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Reentrancy: State Update after External Transfer
 ### Description/Code Snippet
In `requestDeposit`, assets are pulled from the user via `SafeTokenTransfers.safeTransferFrom` (external call) *before* the internal state `$.pendingDepositAssets` is updated. While the `nonReentrant` modifier prevents re-entering the function itself, this violation of the Checks-Effects-Interactions pattern could allow reentrancy into view functions or other unprotected parts of the system where the balance has increased but the pending accounting has not.
 ### Static Signals
External call before state update, Violation of CEI pattern
 ### Assets at Risk
Vault state consistency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: SafeTokenTransfers.safeTransferFrom

 ### Title
Incompatibility with Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library explicitly checks that the balance increase in the recipient equals the transferred amount and reverts if they do not match. This makes the vault incompatible with Fee-on-Transfer tokens, causing denial of service for deposit operations if such an asset is used. While this protects internal accounting, it strictly assumes 1:1 transfer semantics.
 ### Static Signals
balanceAfter != balanceBefore + amount, revert TransferAmountMismatch
 ### Assets at Risk
availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
ReadOnlyReentrancy in requestDeposit via Token Hook
 ### Description/Code Snippet
The `requestDeposit` function transfers assets from the user before updating the `pendingDepositAssets` state. If the asset token has a transfer hook (e.g., ERC777), the hook executes after the vault's balance has increased but before `reservedAssets` (which includes `pendingDepositAssets`) has been updated. During this window, `totalAssets()` (calculated as `balance - reserved`) returns an artificially inflated value. If an external protocol calls `convertToShares` or `totalAssets` during this hook (e.g., via a callback), it will receive an incorrect share price.
 ### Static Signals
safeTransferFrom before state update, totalAssets relies on balanceOf
 ### Assets at Risk
integrating protocols, share price consistency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.redeem

 ### Title
Unsafe Asset Transfer to Zero Address
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, the functions `redeem`, `withdraw`, and `claimCancelDepositRequest` transfer assets to a user-specified `receiver` using `SafeTokenTransfers.safeTransfer`. There is no check to ensure `receiver` is not `address(0)`. If the underlying asset token allows transfers to the zero address (burning), user funds will be irretrievably lost if `address(0)` is accidentally passed as the receiver.
 ### Static Signals
no zero-address guard, transfer to receiver parameter
 ### Assets at Risk
User assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.previewDeposit

 ### Title
Standard Violation: ERC-4626 Preview Functions Revert
 ### Description/Code Snippet
The contract claims full ERC-4626 compliance but implements `previewDeposit`, `previewMint`, `previewWithdraw`, and `previewRedeem` to always revert with `AsyncFlow()`. The ERC-4626 specification requires these functions to return a simulation of the effects of the operation (as close to exact as possible), not to revert. This breakage prevents standard integrations (routers, aggregators) from estimating outcomes or validating operations.
 ### Static Signals
revert AsyncFlow(), preview function does not return value
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Accounting Invariant Violation: Reserved Asset Calculation Mismatch
 ### Description/Code Snippet
The `totalAssets` function calculates `reservedAssets` by summing pending deposits, claimable redeems, and cancelations, but explicitly omits `totalClaimableDepositAssets` (or converted shares). The protocol documentation states the Reserved Asset Calc should sum `pendingDeposit + claimableDeposit (converted) + pendingRedeem` to guarantee idle liquidity. By omitting claimable deposits from the reservation, these assets are treated as 'available for investment' via `investAssets`. This exposes users' unclaimed deposits to investment risk and potential liquidity shortages immediately, contradicting the documented invariant.
 ### Static Signals
reservedAssets calculation excludes totalClaimableDeposit, Spec vs implementation mismatch in reserved assets
 ### Assets at Risk
User deposits waiting to be claimed
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Missing Slippage Protection in investAssets
 ### Description/Code Snippet
The `investAssets` function in `ERC7575VaultUpgradeable` deposits vault assets into an external `investmentVault` without specifying a minimum acceptable share output (`minShares`). If the investment vault's exchange rate is manipulated or unfavorable at the time of execution, the vault may receive fewer shares than expected, resulting in value loss for the protocol.
 ### Static Signals
external call to deposit without min output param, amountOutMin=0 or missing
 ### Assets at Risk
Vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
No Slippage Protection for Async Requests
 ### Description/Code Snippet
The asynchronous `requestDeposit` and `requestRedeem` functions in `ERC7575VaultUpgradeable` do not allow users to specify slippage bounds (e.g., `minShares` or `minAssets`). The exchange rate is determined later during `fulfillDeposit`/`fulfillRedeem` by the Investment Manager. Since there is no deadline or guaranteed rate, users are exposed to unlimited slippage risk between request and fulfillment.
 ### Static Signals
amountOutMin=0 or missing, price fetched at execution without user-specified floor
 ### Assets at Risk
User deposits, User redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

