## Verified Patterns Found: 27

## Verified Patterns Found in following Categories:

- StandardViolation
- PermitFrontRun
- PrecisionDriftAccumulation
- SlippageMissingOrInsufficient
- ERC4626SharePriceMismatch
- UnsafeRecipient
- FlashLoanEconomicManipulation
- StateGrowthOrStorageBloat
- ReserveOrPriceDesync
- ReadOnlyReentrancy
- AccessControlOrAuthByPass
- GriefableCallbacks
- AllowanceRace
- ForcedAssetVsStrictEquality
- StorageCollisionOrSelectorClash
- GovernanceDelegationFlaw
- AccountingInvariantViolation
- PricePrecisionOrRoundingError



## Summary of Patterns

Non-Standard ERC20 Behavior Breaks Wallet and DEX Integrations

Griefing Unregistration via Forced Asset Donation

Phantom Liquidity via rBalance Pricing Mismatch

Incorrect Rounding Direction in Claim Functions Favoring User

Inflation of Restricted Balance (rBalance) via Batch Transfers

Precision Loss and Share Inflation via Donation Attack

Standard ERC20 Allowance Race Condition

Missing Slippage Protection in investAssets and withdrawFromInvestment

Strict balance equality check causes DoS with fee tokens

Read-Only Reentrancy in `requestDeposit` via Pull-Then-Credit

ERC4626 Preview Functions Revert Breaking Standard Composability

Broken ERC-20 Integrations due to Non-Standard Behavior

Storage Collision Risk via Non-Upgradeable ReentrancyGuard

Inflation Attack Vulnerability due to Insufficient Virtual Offset

Missing Zero Address Check in Cancelation Claim Functions

Permit Front-Running DoS

Slippage in Investment Withdrawal

Registry exhaustion DoS via dust blocking unregisterVault

Investment Accounting Corruption via Malicious Flags

Missing Slippage Protection in Async Request Functions

Access Control Bypass in Investment ShareToken (Missing KYC/Permit)

DoS of Monitoring via Unbounded Active Requesters Set

Accounting Invariant Violation in WERC7575ShareToken

System-wide DoS via Unregistrable Broken Vault

Loss Avoidance via rBalance Underflow Protection

TotalSupply desynchronization in rBalance adjustments

Async Deposit Missing Slippage Protection

## Patterns



 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.transfer

 ### Title
Non-Standard ERC20 Behavior Breaks Wallet and DEX Integrations
 ### Description/Code Snippet
The `WERC7575ShareToken` deviates from the ERC20 standard by reverting on self-approval in `approve()` and requiring a pre-existing self-allowance (settable only via a validator-signed `permit`) for `transfer()` to succeed. This design intentionally prevents standard wallets (Metamask, Ledger) and DeFi protocols (Uniswap, lending pools) from interacting with the token, as they expect standard `approve` and `transfer` mechanics.
 ### Static Signals
_spendAllowance(msg.sender, msg.sender, value), if (msg.sender == spender) revert
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Griefing Unregistration via Forced Asset Donation
 ### Description/Code Snippet
ShareTokenUpgradeable.unregisterVault enforces `IERC20(asset).balanceOf(vaultAddress) == 0`. An attacker can send 1 wei of asset to the vault. If the vault is inactive or has no mechanism to sweep dust (which usually requires burning shares), the unregistration is permanently blocked (DoS).
 ### Static Signals
balanceOf(vault) != 0 revert, strict equality check on balance
 ### Assets at Risk
governance operations
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: ShareTokenUpgradeable.getInvestedAssets

 ### Title
Phantom Liquidity via rBalance Pricing Mismatch
 ### Description/Code Snippet
ShareTokenUpgradeable includes `rBalance` (unrealized profit) in `getInvestedAssets`, inflating the share price. However, `withdrawFromInvestment` only redeems `balanceOf` (realized assets) from the underlying WERC7575Vault. If `rBalance` constitutes a significant portion of value, the vault reports a high share price but lacks redeemable liquidity to back it. Users redeeming during this window exit with value taken from other users' principal, leading to potential insolvency.
 ### Static Signals
rBalanceOf included in asset calc, redeem only burns balanceOf, no check for rBalance realizability
 ### Assets at Risk
user deposits, vault liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.mint, withdraw

 ### Title
Incorrect Rounding Direction in Claim Functions Favoring User
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, the `mint` and `withdraw` functions (used to claim fulfilled requests) calculate the required input amount (`assets` for mint, `shares` for withdraw) using `Math.Rounding.Floor`. Standard ERC4626 implementation guidelines dictate that input amounts required from the user should be rounded UP (Ceil) to prevent dust leakage and precision drift that favors the user at the expense of the vault. Using Floor allows a user to pay slightly less assets or burn slightly fewer shares than the precise calculated value.
 ### Static Signals
Math.Rounding.Floor, shares.mulDiv(..., Floor), assets.mulDiv(..., Floor)
 ### Assets at Risk
assets, shares
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Inflation of Restricted Balance (rBalance) via Batch Transfers
 ### Description/Code Snippet
In `WERC7575ShareToken`, the `rBatchTransfers` function updates `_rBalances` (Restricted/Reserved Balances) based on transfer flows. If a creditor receives funds but has insufficient `_rBalances` to reduce, the reduction is truncated to zero (`_rBalances[account.owner] = 0`). However, the debtor's `_rBalances` is always increased by the full amount. This asymmetry allows the total global `_rBalances` to increase (inflate) during transfers where the creditor has low or zero restricted balance. Since `_rBalances` tracks invested capital and is used for yield distribution checks (via `adjustrBalance`), this inflation could disrupt the accounting of invested vs. liquid capital.
 ### Static Signals
accounting state not updated when underlying asset is swapped/upgraded, conservation/binding/monotonic invariants break
 ### Assets at Risk
accounting state
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ShareTokenUpgradeable.convertNormalizedAssetsToShares

 ### Title
Precision Loss and Share Inflation via Donation Attack
 ### Description/Code Snippet
The `ShareTokenUpgradeable` uses `VIRTUAL_SHARES` and `VIRTUAL_ASSETS` constants of `1e6` to protect against inflation attacks. For 18-decimal assets, `1e6` (1 million wei) is an insignificant amount (1e-12 tokens). An attacker can deposit a small amount, then donate a large amount (e.g., 100e18) to the vault. This inflates the `totalNormalizedAssets` / `circulatingSupply` ratio massively. Subsequent user deposits will suffer severe precision loss (e.g., depositing 1e18 assets might result in only 1e6 shares). This allows a griefing or theft attack where user funds are devalued due to rounding/precision loss.
 ### Static Signals
VIRTUAL_SHARES = 1e6, VIRTUAL_ASSETS = 1e6, Math.mulDiv
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AllowanceRace

 ### Relevant Function/Location: WERC7575ShareToken.approve

 ### Title
Standard ERC20 Allowance Race Condition
 ### Description/Code Snippet
The `WERC7575ShareToken` inherits from `ERC20` and overrides `approve` only to block self-approval. It retains the standard ERC20 `approve` behavior for third parties, which is susceptible to the well-known front-running race condition. If an owner changes a spender's allowance from A to B, the spender can front-run the transaction to spend A, and then spend B, effectively spending A+B. While `increaseAllowance` and `decreaseAllowance` exist in OpenZeppelin's implementation, the vulnerable `approve` function remains exposed.
 ### Static Signals
super.approve(spender, value), no check for current allowance 0
 ### Assets at Risk
User share tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.investAssets

 ### Title
Missing Slippage Protection in investAssets and withdrawFromInvestment
 ### Description/Code Snippet
The `ERC7575VaultUpgradeable` functions `investAssets` and `withdrawFromInvestment` execute deposits and redemptions against an external `investmentVault` without any minimum output parameters (`minShares` or `minAssets`).

Code snippet:
`shares = IERC7575($.investmentVault).deposit(amount, $.shareToken);`
`IERC7575($.investmentVault).redeem(minShares, address(this), shareToken_);`

If the investment vault's exchange rate fluctuates or is manipulated, the vault could receive significantly fewer shares/assets than expected, causing loss of value for investors. While the Investment Manager is trusted, the lack of on-chain slippage bounds is a vulnerability pattern.
 ### Static Signals
deposit(amount, receiver) without minShares, redeem(shares, ...) without minAssets
 ### Assets at Risk
Vault Assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: SafeTokenTransfers.safeTransfer

 ### Title
Strict balance equality check causes DoS with fee tokens
 ### Description/Code Snippet
The `SafeTokenTransfers` library enforces strict equality (`balanceAfter == balanceBefore + amount`) on transfers. While this prevents Fee-on-Transfer token issues, it creates a denial of service vulnerability if the underlying asset (e.g. USDC) enables fees or if a future upgrade introduces transfer hooks that consume dust, permanently bricking vault deposits and withdrawals.
 ### Static Signals
balanceAfter != balanceBefore + amount, revert TransferAmountMismatch()
 ### Assets at Risk
vault availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Read-Only Reentrancy in `requestDeposit` via Pull-Then-Credit
 ### Description/Code Snippet
The `requestDeposit` function transfers assets from the user (Pull) before updating the `pendingDepositAssets` state variable (Credit). If the asset token allows callbacks (e.g., ERC777, ERC677), the vault's `balanceOf` will increase during the callback, but the `reservedAssets` deduction (via `totalPendingDepositAssets`) will not have been updated yet. This causes `totalAssets()` to temporarily inflate. Since `ShareTokenUpgradeable` uses `totalAssets()` to calculate the share price, any external protocol reading the price during this callback window will receive a manipulated value.
 ### Static Signals
safeTransferFrom before state update, totalAssets reads balanceOf, view function reads manipulable state
 ### Assets at Risk
vault share price, external protocol funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7575VaultUpgradeable.previewDeposit

 ### Title
ERC4626 Preview Functions Revert Breaking Standard Composability
 ### Description/Code Snippet
The contract implements `IERC7575` which inherits from ERC4626, but the preview functions (`previewDeposit`, `previewMint`, `previewWithdraw`, `previewRedeem`) are overridden to revert with `AsyncFlow()`. The ERC4626 specification states these functions 'MUST return' values. Reverting breaks integration with standard ERC4626 routers and aggregators that rely on these views for simulations.
 ### Static Signals
revert AsyncFlow(), override returns (uint256)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: WERC7575ShareToken.transfer, approve

 ### Title
Broken ERC-20 Integrations due to Non-Standard Behavior
 ### Description/Code Snippet
The `WERC7575ShareToken` intentionally violates the ERC-20 standard in two critical ways: 1) `transfer` and `transferFrom` require the sender to have a valid self-allowance (set via `permit`), effectively blocking standard transfers. 2) `approve` reverts if `msg.sender == spender`, blocking self-approvals. This breaks integration with standard wallets, DEXs, and other protocols that expect standard ERC-20 behavior, potentially causing funds to be stuck or operations to fail unexpectedly.
 ### Static Signals
_spendAllowance(msg.sender, msg.sender, value), if (msg.sender == spender) revert
 ### Assets at Risk
user accessibility
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: ERC7575VaultUpgradeable.inheritance

 ### Title
Storage Collision Risk via Non-Upgradeable ReentrancyGuard
 ### Description/Code Snippet
The upgradeable contract `ERC7575VaultUpgradeable` inherits the non-upgradeable `ReentrancyGuard` from OpenZeppelin. Non-upgradeable contracts use storage slot 0 for the `_status` variable, which can collide with `Initializable` or other logic in the proxy or future upgrades. Additionally, `ReentrancyGuard` relies on a constructor to initialize `_status` to `_NOT_ENTERED` (1). In a proxy context, the constructor does not run for the proxy storage, leaving `_status` as 0. While `0 != _ENTERED (2)` allows the lock to function, this is an unsafe pattern that corrupts storage slot 0 and creates collision risks.
 ### Static Signals
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol", contract ERC7575VaultUpgradeable is ... ReentrancyGuard
 ### Assets at Risk
storage integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: ShareTokenUpgradeable.convertNormalizedAssetsToShares

 ### Title
Inflation Attack Vulnerability due to Insufficient Virtual Offset
 ### Description/Code Snippet
In `ShareTokenUpgradeable`, the `VIRTUAL_ASSETS` constant is set to `1e6`. For 18-decimal normalized assets, this represents `1e-12` units, which is negligible. An attacker can donate assets to the `ERC7575VaultUpgradeable`, inflating `totalNormalizedAssets` massively while `circulatingSupply` remains small (or 1e6 virtual). Subsequent deposits by victims will suffer severe rounding loss (rounding to zero) in `convertNormalizedAssetsToShares` because the virtual offset is too small to stabilize the exchange rate against the donation.
 ### Static Signals
VIRTUAL_ASSETS = 1e6, totalNormalizedAssets += VIRTUAL_ASSETS
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.claimCancelDepositRequest, claimCancelRedeemRequest

 ### Title
Missing Zero Address Check in Cancelation Claim Functions
 ### Description/Code Snippet
The functions `claimCancelDepositRequest` and `claimCancelRedeemRequest` accept a `receiver` address argument but do not explicit check if it is `address(0)`. While they use `SafeTokenTransfers`, not all ERC20 tokens revert on transfer to the zero address. If a user accidentally passes the zero address (or if a frontend defaults to it), funds could be permanently lost.
 ### Static Signals
no zero-address guard, input receiver address
 ### Assets at Risk
canceled assets, canceled shares
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: WERC7575ShareToken.permit

 ### Title
Permit Front-Running DoS
 ### Description/Code Snippet
WERC7575ShareToken implements `permit` which can be front-run. An attacker can extract the signature from the mempool and submit it first. This consumes the nonce and sets the approval. If the user's original transaction relied on the `permit` call succeeding (e.g., in a multicall or smart contract flow), the transaction will revert, causing Denial of Service.
 ### Static Signals
permit implemented, nonce used, no front-run protection
 ### Assets at Risk
user transactions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.withdrawFromInvestment

 ### Title
Slippage in Investment Withdrawal
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable.withdrawFromInvestment`, the function calculates `minShares` based on a current `previewWithdraw` call to the investment vault, and then executes `redeem` using that value. If the investment vault's state changes between the preview and execution (e.g., rate change, fee update), or if the preview is manipulated (read-only reentrancy if applicable), the withdrawal may suffer unbounded slippage. The function lacks a user-supplied `minAmountOut` parameter.
 ### Static Signals
previewWithdraw, redeem(minShares), no user min param
 ### Assets at Risk
invested assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
Registry exhaustion DoS via dust blocking unregisterVault
 ### Description/Code Snippet
The `unregisterVault` function in `ShareTokenUpgradeable` strictly reverts if the vault holds any asset balance (`IERC20(asset).balanceOf(vaultAddress) != 0`). A malicious user can transfer a negligible amount of assets (dust) to the vault directly. Since `investAssets` (the only mechanism to move funds out besides withdrawals) may fail on dust amounts due to minimum deposit limits in the target investment vault, the Owner may be unable to clear the vault's balance. Because `MAX_VAULTS_PER_SHARE_TOKEN` is strictly limited to 10, an attacker can permanently occupy all registry slots by dusting vaults, preventing the protocol from registering new asset classes.
 ### Static Signals
balanceOf(vaultAddress) != 0, revert CannotUnregisterVaultAssetBalance, MAX_VAULTS_PER_SHARE_TOKEN = 10
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GovernanceDelegationFlaw

 ### Relevant Function/Location: WERC7575ShareToken.rBatchTransfers

 ### Title
Investment Accounting Corruption via Malicious Flags
 ### Description/Code Snippet
The `rBatchTransfers` function in `WERC7575ShareToken` relies on `rBalanceFlags` passed by the caller (Validator) to determine which accounts receive `rBalance` (invested capital) updates. The contract blindly applies these flags without on-chain validation of their consistency. A compromised Validator or a bug in the off-chain flag computation can submit valid balance transfers with malicious flags, corrupting the `_rBalances` mapping. This de-syncs the investment accounting from the actual asset flows, potentially leading to incorrect yield distribution or loss of yield tracking.
 ### Static Signals
((rBalanceFlags >> i) & 1) == 1, no flag validation
 ### Assets at Risk
investment accounting, yield
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit, requestRedeem

 ### Title
Missing Slippage Protection in Async Request Functions
 ### Description/Code Snippet
The asynchronous request functions `requestDeposit` and `requestRedeem` allow users to lock assets or shares for future fulfillment by an Investment Manager. However, these functions lack parameters to specify a minimum amount of shares (for deposit) or assets (for redeem) to receive, nor do they accept a deadline. Users are exposed to unlimited price slippage between the request time and the fulfillment time, with no on-chain guarantee of the execution rate.
 ### Static Signals
no minAmountOut param, no deadline param
 ### Assets at Risk
user deposits, user redemptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: ShareTokenUpgradeable.transfer, transferFrom

 ### Title
Access Control Bypass in Investment ShareToken (Missing KYC/Permit)
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract, used for the investment layer, fails to implement the strict access controls present in the settlement layer (`WERC7575ShareToken`). Specifically, it lacks: 1) The `isKycVerified` check on transfers, allowing non-KYC'd addresses to hold shares. 2) The `permit`-based self-allowance requirement for transfers (blocking `approve` / requiring `_spendAllowance(owner, owner)`). 

This deviates from the system's compliance documentation which states 'Every recipient must be KYC-verified' and 'Transfer requires self-allowance'. Users can use standard `approve()` and `transfer()` to bypass validator authorization, undermining the regulatory compliance model.
 ### Static Signals
missing isKycVerified check, approve not overridden to revert, transfer does not check self-allowance
 ### Assets at Risk
Regulatory Compliance, Restricted Investment Shares
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: ERC7575VaultUpgradeable.getActiveDepositRequesters

 ### Title
DoS of Monitoring via Unbounded Active Requesters Set
 ### Description/Code Snippet
In `ERC7575VaultUpgradeable`, the `activeDepositRequesters` EnumerableSet grows indefinitely as unique controllers request deposits. The function `getActiveDepositRequesters` retrieves all values but explicitly reverts if the length exceeds 100 (`TooManyRequesters`). An attacker can generate 101 requests from different addresses, permanently disabling this monitoring function and potentially breaking off-chain integrations dependent on it.
 ### Static Signals
length() > 100, revert TooManyRequesters
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Accounting Invariant Violation in WERC7575ShareToken
 ### Description/Code Snippet
The `WERC7575ShareToken` contract violates the ERC20 invariant `totalSupply == sum(balances)`. The function `adjustrBalance` modifies user `_balances` directly to distribute profit (minting) or loss (burning) without updating `_totalSupply`. Similarly, `rBatchTransfers` moves amounts between `_balances` and `_rBalances`, changing the sum of liquid balances without updating `_totalSupply`. This causes the `totalSupply()` view to desynchronize from the actual circulating liquid supply, potentially breaking external integrations and tracking tools.
 ### Static Signals
totalSupply != sum(balances), balance update without totalSupply update
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: ShareTokenUpgradeable.unregisterVault

 ### Title
System-wide DoS via Unregistrable Broken Vault
 ### Description/Code Snippet
The `ShareTokenUpgradeable` contract iterates over all registered vaults in `getCirculatingSupplyAndAssets`, which is called by `convertNormalizedAssetsToShares` and `convertSharesToNormalizedAssets`. These conversion functions are critical dependencies for `deposit`, `mint`, `withdraw`, and `redeem` in *all* `ERC7575VaultUpgradeable` contracts. If a single registered vault enters a broken state where `getVaultMetrics()` or `getClaimableSharesAndNormalizedAssets()` reverts (e.g., due to a buggy upgrade, a pause mechanism that reverts views, or a logic error), the entire multi-asset system freezes. Critically, the `unregisterVault` function strictly requires `getVaultMetrics()` to succeed to verify safety conditions. This creates a deadlock: a broken vault bricks the system, but the system cannot unregister the broken vault because it is broken.
 ### Static Signals
try IVaultMetrics(vaultAddress).getVaultMetrics(), revert CannotUnregisterActiveVault(), loop over _assetToVault
 ### Assets at Risk
All assets in all vaults
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
Loss Avoidance via rBalance Underflow Protection
 ### Description/Code Snippet
In `WERC7575ShareToken`, the `adjustrBalance` function attempts to record investment losses by decreasing `_rBalances[account]`. It explicitly reverts if `currentRBalance < difference`. However, `rBatchTransfers` allows users to withdraw their invested capital (reducing `_rBalances` to 0 via silent truncation). A user can front-run a loss adjustment by withdrawing their funds via `rBatchTransfers`, resetting their `_rBalances` to 0. When the Revenue Admin subsequently calls `adjustrBalance` to apply the loss, it reverts due to underflow/check, causing the transaction to fail. The user effectively avoids the loss, and the admin is blocked from reconciling the state.
 ### Static Signals
_rBalances[account] -= difference, currentRBalance < difference, revert RBalanceAdjustmentTooLarge
 ### Assets at Risk
yield, principal
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: WERC7575ShareToken.adjustrBalance

 ### Title
TotalSupply desynchronization in rBalance adjustments
 ### Description/Code Snippet
The `adjustrBalance` function modifies `_balances` to reflect investment profit/loss but fails to update `_totalSupply`. This breaks the ERC20 invariant `totalSupply == sum(balances)`, causing the reported supply to drift from the actual circulating supply. In profit scenarios (`amountr > amounti`), `_balances` increases without a corresponding `totalSupply` increase.
 ### Static Signals
_balances[account] += amountr, no _update call, no _totalSupply modification
 ### Assets at Risk
protocol accounting
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: ERC7575VaultUpgradeable.requestDeposit

 ### Title
Async Deposit Missing Slippage Protection
 ### Description/Code Snippet
The async deposit flow (`requestDeposit` -> `fulfillDeposit`) determines the amount of shares minted at the time of fulfillment based on the current exchange rate. There is no `minShares` parameter in `requestDeposit` to bound slippage. If the share price increases significantly (e.g., due to rBalance adjustment) between request and fulfillment, the user receives fewer shares than expected.
 ### Static Signals
no minShares parameter, payout calculated at execution time
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

