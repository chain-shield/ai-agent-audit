## Verified Patterns Found: 23

## Verified Patterns Found in following Categories:

- SlippageMissingOrInsufficient
- GovernanceDelegationFlaw
- PermitMisuse
- PricePrecisionOrRoundingError
- AccountingInvariantViolation
- AccessControlOrAuthByPass
- ReserveOrPriceDesync
- ERC4626SharePriceMismatch
- StateGrowthOrStorageBloat
- StandardViolation
- FeeOnTransferAssumption
- ForcedAssetVsStrictEquality



## Summary of Patterns

Strict Balance Equality Check Enables Griefing of Vault Unregistration

Vault Unregistration DoS via Dust Holdings

Precision Loss in Redemption Fulfillment

ERC-4626 Standard Violation in Preview Functions

Single Point of Failure in Share Pricing Mechanism

Non-Standard ERC20 Implementation Breaks Composability

Permit Signer Validation Flaw in ShareToken

Insufficient virtual offset allows first depositor inflation attack

Silent rBalance Truncation Corrupts Investment Accounting

Missing Slippage Protection in Asynchronous Redemption

Strict Balance Check Incompatible with Fee-on-Transfer Tokens

Permanent Denial of Service on Vault Unregistration via Dust State

Unbacked Share Price Inflation via adjustrBalance

FeeOnTransferAssumption (Rebasing Tokens)

Investment Layer IUSD Token Lacks KYC Restrictions Allowing Compliance Bypass

Slippage Missing in Async Fulfillment

Incorrect rounding in withdraw allows asset drainage without burning shares

SlippageMissingOrInsufficient in Async Deposit Flow

Accounting Mismatch in Reserved Asset Calculation

Total Assets Clamping Hides Insolvency and Breaks Price

Missing Slippage Protection in Investment Execution

Yield trapped in rBalance due to accounting logic mismatch

Accounting Invariant Violation in Yield Adjustment

## Patterns



 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Strict Balance Equality Check Enables Griefing of Vault Unregistration
 ### Description/Code Snippet
The `unregisterVault` function enforces a strict check `IERC20(asset).balanceOf(vaultAddress) != 0` to ensure the vault is empty. An attacker can perform a direct transfer of 1 wei of the asset to the vault (`donation`). If the investment vault is not configured (or was removed), the admin has no mechanism to sweep this dust (as `investAssets` requires a valid investment vault). This forces the unregistration to revert, griefing the protocol's lifecycle management.
 ### Static Signals
require(address(this).balance == 0), strict equality check
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GovernanceDelegationFlaw

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Vault Unregistration DoS via Dust Holdings
 ### Description/Code Snippet
A malicious user can permanently block the unregistration of a vault by maintaining a small pending cancelation or unclaimed deposit/redemption. `ShareTokenUpgradeable.unregisterVault` (lines 198-245) strictly requires the target vault to have zero total pending, claimable, and cancel assets to prevent user fund loss. However, there is no mechanism for the admin to force-claim, refund, or sweep these specific user funds. An attacker can deposit a minimal amount in all registered vaults and refuse to claim or finalize cancelations. Combined with the hard cap of 10 vaults (`MAX_VAULTS_PER_SHARE_TOKEN` line 52) and the inability to add new vaults once the limit is reached (`registerVault` lines 147-150), this allows an attacker to permanently lock the protocol's asset list, preventing the onboarding of new assets.
 ### Static Signals
MAX_VAULTS_PER_SHARE_TOKEN, revert CannotUnregisterVaultAssetBalance, revert CannotUnregisterVaultPendingDeposits
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillRedeem

 ### Title
Precision Loss in Redemption Fulfillment
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, `fulfillRedeem` converts shares to assets using `Math.Rounding.Floor`. For assets with low decimals (e.g., USDC, 6 decimals) and shares with 18 decimals, `scalingFactor` is large (1e12). Redeeming `shares < 1e12` results in 0 assets, but the shares are marked for burning in `claimableRedeemShares`. When the user calls `redeem`, they burn shares but receive 0 assets. This creates a systematic precision loss for dust amounts or non-aligned share amounts.
 ### Static Signals
division with large scaling factor, rounding down to zero, state update with zero value
 ### Assets at Risk
user shares
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.previewDeposit

 ### Title
ERC-4626 Standard Violation in Preview Functions
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` contract implements ERC-4626 but causes all preview functions (`previewDeposit`, `previewMint`, etc.) to revert with `AsyncFlow()`. The ERC-4626 standard requires these functions to return a simulation of the transaction effects. Reverting breaks compatibility with on-chain aggregators, routers, and off-chain tools that rely on the standard ERC-4626 interface for price and outcome estimation.
 ### Static Signals
revert AsyncFlow()
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ShareTokenUpgradeable.getCirculatingSupplyAndAssets

 ### Title
Single Point of Failure in Share Pricing Mechanism
 ### Description/Code Snippet
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults to calculate total system assets. If a single registered vault becomes dysfunctional (e.g., reverts on `getClaimableSharesAndNormalizedAssets` due to a bug, upgrade failure, or pause state), the call will revert. This causes the shared pricing functions `convertToShares` and `convertToAssets` to fail globally, effectively bricking deposit and redemption operations for ALL vaults in the system.
 ### Static Signals
loop over all vaults, external call in loop, no try/catch
 ### Assets at Risk
system availability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.transfer

 ### Title
Non-Standard ERC20 Implementation Breaks Composability
 ### Description/Code Snippet
The WERC7575ShareToken implementation intentionally breaks ERC-20 standard compliance by requiring a validator-signed permit for `transfer` operations and disabling self-approval via `approve`. Specifically, `transfer` calls `_spendAllowance(msg.sender, msg.sender, value)`, which reverts unless a self-allowance has been set via `permit`. Furthermore, `approve` reverts if `spender == msg.sender`. This design breaks compatibility with standard wallets, DEXs, and other protocols that rely on the standard ERC-20 interface, causing transactions to revert unexpectedly.
 ### Static Signals
_spendAllowance(msg.sender, msg.sender, value), if (msg.sender == spender) revert ERC20InvalidSpender
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: WERC7575ShareToken.permit

 ### Title
Permit Signer Validation Flaw in ShareToken
 ### Description/Code Snippet
In `WERC7575ShareToken.permit`, when `owner == spender`, the contract enforces `signer == _validator` instead of `signer == owner`. While this implements the intended 'self-allowance via validator' logic, it deviates from the EIP-2612 standard which expects the owner to sign. This non-standard behavior breaks integrations with wallets and dApps that generate standard permit signatures signed by the owner. Additionally, `_useNonce(owner)` consumes the owner's nonce even though the validator signed it, potentially leading to nonce synchronization issues if the owner also signs standard permits.
 ### Static Signals
no deadline check, nonces reused or not incremented
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ShareTokenUpgradeable.convertNormalizedAssetsToShares

 ### Title
Insufficient virtual offset allows first depositor inflation attack
 ### Description/Code Snippet
The `ShareTokenUpgradeable` uses `VIRTUAL_ASSETS` and `VIRTUAL_SHARES` constants set to `1e6` to prevent inflation attacks. However, the system normalizes all assets to 18 decimals (`1e18`). A virtual offset of `1e6` (equivalent to `1e-12` tokens) is negligible compared to the 18-decimal precision. A malicious first depositor can deposit a small amount (e.g., 1 wei of USDC, normalized to `1e12`), then donate a large amount of assets to the vault to inflate the share price excessively, causing subsequent depositors to lose value due to rounding.
 ### Static Signals
no virtual shares/assets to prevent inflation attack, first depositor can manipulate share price
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Silent rBalance Truncation Corrupts Investment Accounting
 ### Description/Code Snippet
In `rBatchTransfers`, the logic silently truncates an account's `_rBalances` (reserved balance) to zero if the amount to be deducted exceeds the current rBalance (`if (rbalance < amount) _rBalances[...] = 0`). This destroys the invariant of tracked invested capital versus returned capital. If `adjustrBalance` or `cancelrBalanceAdjustment` is called later, the accounting will be incorrect because the historical investment basis has been lost. This can lead to incorrect yield distribution or inability to reconcile investment positions.
 ### Static Signals
_rBalances[account.owner] = 0, if (rbalance < amount)
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestRedeem

 ### Title
Missing Slippage Protection in Asynchronous Redemption
 ### Description/Code Snippet
The `requestRedeem` function allows users to queue shares for redemption, and `fulfillRedeem` converts these shares to assets using the exchange rate at the time of fulfillment. However, there is no `minAssets` parameter in `requestRedeem` or `fulfillRedeem` to enforce a minimum acceptable output. Because the fulfillment is asynchronous and controlled by the Investment Manager, users are exposed to unlimited downward price volatility (slippage or investment losses) between the time of their request and the time of execution.
 ### Static Signals
no minAmountOut parameter in liquidation/redemption, payout calculated at execution time without minimum bound
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: SafeTokenTransfers.safeTransfer

 ### Title
Strict Balance Check Incompatible with Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library enforces a strict invariant that `balanceAfter == balanceBefore + amount`. This check will revert for any Fee-on-Transfer tokens (e.g., USDT with fees enabled) or rebase tokens where the received amount is less than the transferred amount. If the protocol supports such assets, this validation will cause a Denial of Service for all deposits and transfers involving that asset.
 ### Static Signals
if (balanceAfter != balanceBefore + amount) revert TransferAmountMismatch
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: ERC7575VaultUpgradeable.unregisterVault

 ### Title
Permanent Denial of Service on Vault Unregistration via Dust State
 ### Description/Code Snippet
The `unregisterVault` function strictly requires `metrics.activeDepositRequestersCount == 0`. A user is only removed from `activeDepositRequesters` when they fully claim their deposit (`availableAssets == assets` in `deposit`). An attacker can request a deposit, wait for fulfillment, and then claim all but 1 wei of the assets. This keeps the attacker in the `activeDepositRequesters` set indefinitely. Since the admin cannot force-claim or evict a user, this permanently prevents the `unregisterVault` function from executing.
 ### Static Signals
no pruning mechanism, strict equality check on state size
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Unbacked Share Price Inflation via adjustrBalance
 ### Description/Code Snippet
The `adjustrBalance` function in `WERC7575ShareToken` increases `_rBalances` for accounts based on reported profits (`amountr > amounti`) without requiring a corresponding increase in liquid `_balances`. 

`ShareTokenUpgradeable` calculates the share price (via `convertNormalizedAssetsToShares`) using `totalNormalizedAssets`, which includes `rBalance` (via `getInvestedAssets`). 

By increasing `rBalance` without backing assets, `adjustrBalance` inflates the share price. Since the underlying `_balances` do not increase, the vault lacks the liquid assets to fulfill redemptions at this inflated price, leading to insolvency and loss of funds for last-exiters.
 ### Static Signals
_rBalances[account] +=, no transfer from _balances
 ### Assets at Risk
vault solvency, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
FeeOnTransferAssumption (Rebasing Tokens)
 ### Description/Code Snippet
The `requestDeposit` function tracks deposits using a static `pendingDepositAssets` mapping. If the underlying asset is a rebasing token (e.g., stETH, aTokens) that changes balance over time without a transfer hook, the vault's `balanceOf` will diverge from the sum of `reservedAssets` and invested assets. The yield or loss generated by the pending assets will be misattributed to the general pool (existing shareholders) via `totalAssets()` instead of the pending depositor, effectively stealing yield or socializing losses for the pending amounts.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, no handling for rebasing tokens
 ### Assets at Risk
yield, user principal
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: ShareTokenUpgradeable.transfer / transferFrom

 ### Title
Investment Layer IUSD Token Lacks KYC Restrictions Allowing Compliance Bypass
 ### Description/Code Snippet
The protocol documentation emphasizes regulatory compliance and KYC/AML enforcement. The Settlement Layer token (`WERC7575ShareToken`) correctly overrides `transfer` and `transferFrom` to enforce `isKycVerified[user]`. However, the Investment Layer share token (`ShareTokenUpgradeable`), which represents `IUSD`, inherits `ERC20Upgradeable` but does not override transfer functions to check KYC status. This allows non-KYC verified actors to acquire, hold, and transfer IUSD tokens permissionlessly. Since IUSD represents a claim on assets that are invested into the regulated Settlement Layer, this bypasses the protocol's compliance perimeter.
 ### Static Signals
inherits ERC20Upgradeable, no override for transfer/transferFrom, no isKycVerified check, intended regulatory compliance
 ### Assets at Risk
Regulatory Compliance, Reputation
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.fulfillDeposit

 ### Title
Slippage Missing in Async Fulfillment
 ### Description/Code Snippet
The `fulfillDeposit` and `fulfillRedeem` functions in `ERC7575VaultUpgradeable` execute share/asset conversions at the current spot rate without any slippage protection (e.g., `minShares` or `minAssets`). Since the exchange rate depends on `totalAssets` (which includes balances in external vaults and the `InvestmentShareToken`), it can fluctuate or be manipulated between the user's request and the manager's fulfillment. This exposes users to significant value loss if the rate moves unfavorably.
 ### Static Signals
payout calculated at execution time without minimum bound, price fetched at execution without user-specified floor
 ### Assets at Risk
user deposits, user redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdraw

 ### Title
Incorrect rounding in withdraw allows asset drainage without burning shares
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.withdraw`, the share calculation uses `Math.Rounding.Floor` (`shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`). This rounding direction favors the caller instead of the vault. An attacker can repeatedly withdraw small amounts of assets such that the calculated `shares` to burn rounds down to 0, allowing them to drain the `claimableRedeemAssets` without burning any `claimableRedeemShares`. This devalues the share token for other holders.
 ### Static Signals
rounding bias always favors caller
 ### Assets at Risk
claimableRedeemAssets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
SlippageMissingOrInsufficient in Async Deposit Flow
 ### Description/Code Snippet
The `requestDeposit` function in `ERC7575VaultUpgradeable` initiates an asynchronous deposit by transferring assets to the vault, but it lacks an `amountOutMin` or `minShares` parameter. The actual share minting occurs later in `fulfillDeposit` based on the exchange rate at that time. Users have no guarantee of the exchange rate they will receive and cannot enforce a minimum slippage bound, exposing them to rate changes or manipulation between the request and fulfillment.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Accounting Mismatch in Reserved Asset Calculation
 ### Description/Code Snippet
The `totalAssets()` function in `ERC7575VaultUpgradeable` calculates available assets by subtracting `reservedAssets` from the balance. `reservedAssets` includes `pendingDepositAssets`, `claimableRedeemAssets`, and `cancelDepositAssets`, but omits `claimableDepositAssets`. This allows the Investment Manager to invest assets that correspond to fulfilled but unclaimed deposits via `investAssets()`. This contradicts the documentation/invariants which state reserved assets (including claimable) should sit idle and not be invested to ensure liquidity and correct yield commitment.
 ### Static Signals
reservedAssets calculation omits claimableDepositAssets, investAssets relies on totalAssets
 ### Assets at Risk
claimable deposit assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.totalAssets

 ### Title
Total Assets Clamping Hides Insolvency and Breaks Price
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.totalAssets()`, the function returns `0` if `balance < reservedAssets` due to the check `balance > reservedAssets ? balance - reservedAssets : 0`. `reservedAssets` includes `claimableRedeemAssets` which are physically present in the balance. If the vault suffers a loss such that `balance` falls below `reservedAssets`, `totalAssets` reports 0. This causes the share price calculation (`Supply / TotalAssets`) in `ShareTokenUpgradeable` to revert (div by zero) or return an incorrect infinite price, bricking the protocol and potentially enabling share price manipulation exploits if the balance fluctuates near the threshold.
 ### Static Signals
balance tracking references different token than actual holdings, collateral swapped but totalCollateral unchanged
 ### Assets at Risk
vault funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Missing Slippage Protection in Investment Execution
 ### Description/Code Snippet
The `investAssets` function in `ERC7575VaultUpgradeable` deposits assets into an external `investmentVault` without a minimum share output parameter (`minShares`). If the external vault has variable exchange rates, slippage, or is manipulated, the vault may receive fewer shares than expected, leading to value loss for the protocol.
 ### Static Signals
external call to deposit without min output, no slippage param
 ### Assets at Risk
vault assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Yield trapped in rBalance due to accounting logic mismatch
 ### Description/Code Snippet
The `WERC7575ShareToken.adjustrBalance` function updates `_rBalances` to reflect investment profit but does not move these profits to `_balances`. Conversely, `WERC7575Vault.redeem` and `_burn` only operate on `_balances`. As a result, when the Investment Manager calls `withdrawFromInvestment` to realize yield, the `redeem` operation is capped by the principal (`balanceOf`), leaving the profit (`rBalance`) trapped and unwithdrawable. This contradicts the system's documented yield generation flow.
 ### Static Signals
accounting state not updated when underlying asset is swapped/upgraded, balance tracking references different token than actual holdings
 ### Assets at Risk
investment yield
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Accounting Invariant Violation in Yield Adjustment
 ### Description/Code Snippet
In `WERC7575ShareToken.adjustrBalance`, the function updates `_rBalances` (reserved balance) and `_balances` (liquid balance) to reflect investment yield, but it fails to update `_totalSupply`. While the provided code snippet only explicitly shows the `_rBalances` update, the logic implies profit distribution. If `_balances` are increased (as implied by documentation/logic for withdrawal) without a corresponding `_totalSupply` increase, subsequent withdrawals (which call `burn` and decrease `_totalSupply`) will eventually cause `_totalSupply` to underflow and revert, permanently locking the contract.
 ### Static Signals
totalSupply != sum(balances), burnFrom doesn't reduce totalSupply
 ### Assets at Risk
all token liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

