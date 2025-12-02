## Verified Patterns Found: 25

## Verified Patterns Found in following Categories:

- FeeAccountingDrift
- OracleUsingDEXorTWAP
- AccessControlOrAuthByPass
- FeeOnTransferAssumption
- TimelockEdgeCase
- SlippageMissingOrInsufficient
- MaturityorGatingByPass
- StandardViolation
- ReserveOrPriceDesync
- StaleOracleAcceptance
- AccountingInvariantViolation
- GriefableCallbacks



## Summary of Patterns

Cross-chain minting bypasses Whitelist restrictions causing stuck funds

Missing oracle freshness checks in iTryIssuer

Cross-chain Cooldown Timer Reset Griefing

ERC-4626 `maxWithdraw` violation causes integration DoS

Lack of oracle freshness validation in Issuer

FastRedeem Denial of Service on Small Amounts

Vesting State Blocks Confiscation of Blacklisted Funds

Indefinite DoS of withdrawals via cross-chain cooldown reset

Accounting invariant violation with fee-on-transfer collateral

OracleUsingDEXorTWAP: Missing staleness and validation checks on Redstone oracle

Yield calculation desync with rebasing collateral

Fee Rounding Bias Penalizes Small Redemptions

AccountingInvariantViolation: Yield leakage due to FastAccessVault balance desynchronization

Cross-chain and local cooldown state collision

Admin Confiscation Blocked by Active Vesting

Reward accounting breaks with fee-on-transfer tokens

Incompatible with fee-on-transfer tokens due to unsynced silo accounting

Missing Slippage Protection in Fast Redeem/Withdraw

Missing Slippage Protection in Share-Asset Conversions

StandardViolation: ERC4626 maxWithdraw/maxRedeem do not reflect cooldown state

Cross-chain cooldown reset griefing via Composer

Cooldown duration reset attack via dust deposit

Unsafe Downcasting in Cooldown Accounting

Shared Cooldown State allows Composer to seize Local Stakers' funds

Missing slippage protection in cross-chain fast redeem

## Patterns



 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTryTokenOFT._beforeTokenTransfer

 ### Title
Cross-chain minting bypasses Whitelist restrictions causing stuck funds
 ### Description/Code Snippet
In `iTryTokenOFT.sol`, when `transferState` is `WHITELIST_ENABLED`, normal transfers enforce that sender and receiver are whitelisted. However, the `_beforeTokenTransfer` hook's minting branch (`msg.sender == minter`) only checks `!blacklisted[to]`. This allows a non-whitelisted user to bridge tokens in from another chain. Once received, the user cannot transfer or burn/bridge them out because the `transfer` and `burn` branches enforce the `whitelisted` check, effectively trapping the funds.
 ### Static Signals
role check after state change, inconsistent permission checks
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.mintITRY

 ### Title
Missing oracle freshness checks in iTryIssuer
 ### Description/Code Snippet
The `iTryIssuer` contract relies on `oracle.price()` to determine the NAV of DLF for minting, redeeming, and yield distribution. The code does not validate the freshness of the returned price (e.g., via `updatedAt`, `roundId`, or timestamp checks). 

If the oracle feed becomes stale (due to downtime or lack of updates), the Issuer will continue to mint and redeem iTRY at an outdated price. This allows arbitrageurs to exploit the difference between the stale on-chain price and the real-world NAV, leading to undercollateralization of the stablecoin.
 ### Static Signals
oracle.price() called without timestamp check, no updatedAt/answeredInRound checks
 ### Assets at Risk
treasury, collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: StakediTryCrosschain._startComposerCooldown

 ### Title
Cross-chain Cooldown Timer Reset Griefing
 ### Description/Code Snippet
The `_startComposerCooldown` function unconditionally overwrites `cooldowns[redeemer].cooldownEnd` with `block.timestamp + cooldownDuration` every time it is called. The `wiTryVaultComposer` (the intended caller holding `COMPOSER_ROLE`) acts as a bridge interface that likely allows any user on a source chain to specify an arbitrary `redeemer` address on the hub chain. 

A malicious actor can exploit this by repeatedly bridging 'dust' amounts (e.g., 1 wei of wiTRY) to a target victim's address. Each time the `cooldownSharesByComposer` or `cooldownAssetsByComposer` function is executed by the composer, the victim's cooldown timer is reset to the full duration. By continuously sending these transactions just before the victim's cooldown expires, the attacker can permanently prevent the victim from finalizing their withdrawal via `unstakeThroughComposer`, effectively locking the victim's funds in the `StakediTry` silo indefinitely.
 ### Static Signals
cooldowns[redeemer].cooldownEnd = cooldownEnd, no check for existing cooldown
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.withdraw

 ### Title
ERC-4626 `maxWithdraw` violation causes integration DoS
 ### Description/Code Snippet
The `StakediTryV2` contract overrides `withdraw` to revert when `cooldownDuration > 0` (via `ensureCooldownOff`), but does not override `maxWithdraw` to return 0 in this state. This violates the ERC-4626 standard which mandates `maxWithdraw` return the amount withdrawable in the current block. Integrators and routers relying on `maxWithdraw` to check feasibility will proceed to call `withdraw` and revert, causing Denial of Service.
 ### Static Signals
ensureCooldownOff, maxWithdraw not overridden
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Lack of oracle freshness validation in Issuer
 ### Description/Code Snippet
The `mintFor` and `redeemFor` functions rely on `oracle.price()` to determine the exchange rate between DLF and iTRY. However, the code does not perform any validation on the returned price data (such as checking `updatedAt`, `roundId`, or a freshness timestamp). If the oracle feed stops updating or provides stale data, the protocol may issue unbacked iTRY or allow redemptions at incorrect prices, breaking the 1:1 backing invariant.
 ### Static Signals
no updatedAt/answeredInRound checks on Chainlink, accepts price older than reasonable threshold
 ### Assets at Risk
protocol collateral, iTRY backing
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryFastRedeem._redeemWithFee

 ### Title
FastRedeem Denial of Service on Small Amounts
 ### Description/Code Snippet
In `_redeemWithFee`, `feeShares` is calculated using `previewWithdraw(feeAssets)`, which rounds up. For small redemptions or high share prices, the rounded-up `feeShares` can equal the total `shares` being redeemed, resulting in `netShares = 0`. The subsequent call to `_withdraw` includes a `notZero(shares)` modifier (inherited from `StakediTry`), causing the transaction to revert. This creates a Denial of Service for users attempting to fast-redeem small valid amounts.
 ### Static Signals
previewWithdraw rounding up, subtraction for netShares, notZero modifier
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: StakediTry.redistributeLockedAmount

 ### Title
Vesting State Blocks Confiscation of Blacklisted Funds
 ### Description/Code Snippet
The `redistributeLockedAmount` function in `StakediTry.sol` is designed to allow the admin to confiscate and burn funds from blacklisted users by passing `to = address(0)`. However, this specific code path triggers `_updateVestingAmount(iTryToVest)`, which enforces an invariant that no new vesting can start while previous rewards are still vesting (`getUnvestedAmount() > 0`). Since the protocol is designed to stream rewards continuously, this condition will almost always cause the confiscation attempt to revert, effectively disabling the 'burn' functionality for seized assets.
 ### Static Signals
implementation vs spec mismatch, security functions depend on unrelated yield state
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Indefinite DoS of withdrawals via cross-chain cooldown reset
 ### Description/Code Snippet
In `StakediTryCrosschain.sol`, the function `cooldownSharesByComposer` (called by the trusted Composer upon cross-chain messages) adds assets to a `redeemer`'s cooldown bucket and resets their `cooldownEnd` timestamp to `block.timestamp + cooldownDuration`. Since the cooldown is cumulative, an attacker can trigger a cross-chain message to add a negligible amount of shares (dust) to a victim's pending cooldown, resetting their 3-day timer. Repeating this attack just before the timer expires can permanently lock the victim's funds.
 ### Static Signals
state-dependent iteration, external call in loop, callback success required for core flow
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryIssuer._transferIntoVault

 ### Title
Accounting invariant violation with fee-on-transfer collateral
 ### Description/Code Snippet
The `iTryIssuer` contract tracks `_totalDLFUnderCustody` by adding the `dlfAmount` parameter passed to `mintITRY`. It transfers this amount using `transferFrom`. If the underlying DLF token implements transfer fees, the `FastAccessVault` receives fewer tokens than accounted for. This drift causes `processAccumulatedYield` to overestimate the backing assets, leading to the minting of unbacked iTRY yield (insolvency) or preventing full redemptions.
 ### Static Signals
balance tracking references different token, accounting state not updated when underlying asset is swapped
 ### Assets at Risk
protocol solvency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
OracleUsingDEXorTWAP: Missing staleness and validation checks on Redstone oracle
 ### Description/Code Snippet
The `iTryIssuer` contract retrieves the NAV price via `oracle.price()` in `mintFor`, `redeemFor`, and `processAccumulatedYield`. The code performs no validation on the returned data (e.g., checking for stale timestamps, round IDs, or min/max bounds). If the oracle returns stale data or 0 (bypassable only if >0 check exists but validity is not guaranteed), the protocol could mint iTRY at incorrect prices or distribute incorrect yield.
 ### Static Signals
oracle.price() call without timestamp check, no roundId validation
 ### Assets at Risk
treasury, rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
Yield calculation desync with rebasing collateral
 ### Description/Code Snippet
The `iTryIssuer` tracks the total collateral (`_totalDLFUnderCustody`) strictly based on mint and redeem operations. If the underlying `DLF` token is a rebasing token (common for MMFs) that distributes yield via balance increases, `processAccumulatedYield` will fail to detect this value growth. The `FastAccessVault` will treat the rebased tokens as excess liquidity and sweep them to the custodian, effectively resulting in stakers losing their claim to the yield.
 ### Static Signals
assumes invariant without verifying, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk
Protocol Yield
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: StakediTryFastRedeem._redeemWithFee

 ### Title
Fee Rounding Bias Penalizes Small Redemptions
 ### Description/Code Snippet
In `StakediTryFastRedeem._redeemWithFee`, the protocol calculates the protocol fee in assets and then converts this to shares using `previewWithdraw`. ERC4626 `previewWithdraw` rounds up the number of shares. For small redemptions or scenarios where the share price is high, this rounding forces the user to burn a minimum of 1 share as a fee, even if the calculated fee in assets is worth much less than 1 share. This results in users paying a disproportionately high effective fee (potentially exceeding the configured BPS) which accumulates to the remaining stakers.
 ### Static Signals
fee math (order/rounding) leaks value, consistent floor toward sender/receiver
 ### Assets at Risk
assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
AccountingInvariantViolation: Yield leakage due to FastAccessVault balance desynchronization
 ### Description/Code Snippet
The `iTryIssuer` contract manually tracks `_totalDLFUnderCustody` based on mint/redeem flows. However, `FastAccessVault` (the liquidity buffer) can receive DLF tokens directly (donations or rebasing yield from the underlying asset). Since `_totalDLFUnderCustody` is not updated to reflect these external balance increases, `processAccumulatedYield` (which calculates yield as `(Custody * Price) - Issued`) will underreport or fail to report this yield. Consequently, the `rebalanceFunds` function in `FastAccessVault` will identify the untracked surplus as 'excess' and transfer it to the Custodian, effectively leaking staker yield to the Custodian.
 ### Static Signals
_totalDLFUnderCustody tracked manually, balanceOf(vault) not used in yield calc
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: StakediTryCrosschain.unstakeThroughComposer

 ### Title
Cross-chain and local cooldown state collision
 ### Description/Code Snippet
The `StakediTryCrosschain` contract shares the `cooldowns` mapping (inherited from `StakediTryV2`) between direct user actions and composer-mediated cross-chain actions. If a user initiates cooldowns both locally (on Hub) and via a spoke chain (via `cooldownSharesByComposer`), the amounts are aggregated into a single `underlyingAmount`. 

When `unstakeThroughComposer` (triggered by the bridge) or `unstake` (triggered by the user) is called, the *entire* aggregated amount is withdrawn to the caller (Composer or User). This allows a user to withdraw cross-chain assets locally (breaking the bridge's asset-backing invariant) or allows the bridge to withdraw local assets to the spoke chain without explicit user intent for those specific funds, potentially stranding assets on the source chain where the corresponding burn/lock event never occurred.
 ### Static Signals
cooldowns[redeemer] used in multi-path flow, state flags/locals reused across calls
 ### Assets at Risk
collateral, withdrawals
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: TimelockEdgeCase

 ### Relevant Function/Location: StakediTry.redistributeLockedAmount

 ### Title
Admin Confiscation Blocked by Active Vesting
 ### Description/Code Snippet
The `redistributeLockedAmount` function (in `StakediTry`) allows the admin to burn restricted funds and convert them to rewards by passing `to = address(0)`. This path calls `_updateVestingAmount`, which strictly reverts if `getUnvestedAmount() > 0`. If the protocol is actively streaming rewards (which is the expected state), this admin feature is permanently blocked, preventing the redistribution of confiscated funds.
 ### Static Signals
revert if unvested > 0, admin function dependency
 ### Assets at Risk
confiscated funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: StakediTry.transferInRewards

 ### Title
Reward accounting breaks with fee-on-transfer tokens
 ### Description/Code Snippet
The `transferInRewards` function transfers `amount` of assets and immediately records `amount` in `vestingAmount` without verifying the actual balance increase. If the underlying `asset` has a fee-on-transfer mechanism (now or after an upgrade), the contract's `totalAssets()` calculation (`balanceOf(this) - getUnvestedAmount()`) will become incorrect, potentially underflowing or artificially lowering the share price for all stakers.
 ### Static Signals
safeTransferFrom without balance check, accounting based on input amount
 ### Assets at Risk
vault share price
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: StakediTryCrosschain._startComposerCooldown

 ### Title
Incompatible with fee-on-transfer tokens due to unsynced silo accounting
 ### Description/Code Snippet
In `_startComposerCooldown`, the contract withdraws `assets` to the `silo` and simultaneously credits the `redeemer` with the same `assets` amount in the `cooldowns` mapping. It does not verify the actual amount received by the `silo`. If the underlying `iTRY` token (or a future upgrade) implements transfer fees, the `silo` will receive fewer assets than recorded, causing insolvency when the user later attempts to `unstake`.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check
 ### Assets at Risk
silo solvency
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryFastRedeem.fastRedeem

 ### Title
Missing Slippage Protection in Fast Redeem/Withdraw
 ### Description/Code Snippet
The `fastRedeem` and `fastWithdraw` functions in `StakediTryFastRedeem.sol` allow users to convert between shares and assets (paying a fee) at the current exchange rate. However, these functions lack `minAssetsOut` or `maxSharesIn` parameters. This exposes users to unlimited slippage if the exchange rate changes (e.g., due to a slashing event, large deposit/withdrawal, or manipulation) or if the admin raises the fee (up to 20%) between transaction submission and execution.
 ### Static Signals
no minAmountOut parameter in liquidation/redemption, payout calculated at execution time without minimum bound
 ### Assets at Risk
assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Missing Slippage Protection in Share-Asset Conversions
 ### Description/Code Snippet
The functions `cooldownSharesByComposer` and `fastRedeemThroughComposer` convert shares to assets using `previewRedeem`, while `cooldownAssetsByComposer` and `fastWithdrawThroughComposer` convert assets to shares using `previewWithdraw`. These functions execute at the current spot rate without accepting a user-defined minimum output (assets) or maximum input (shares). Since these functions are intended to be triggered by cross-chain messages via a Composer (introducing significant latency between signing and execution), users are highly exposed to price volatility, sandwich attacks, or state changes that unfavorably alter the exchange rate.
 ### Static Signals
assets = previewRedeem(shares), no minAssets parameter, no deadline
 ### Assets at Risk
assets, shares
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.maxWithdraw

 ### Title
StandardViolation: ERC4626 maxWithdraw/maxRedeem do not reflect cooldown state
 ### Description/Code Snippet
The StakediTryV2 contract restricts withdrawals when `cooldownDuration > 0` via the `ensureCooldownOff` modifier on `withdraw` and `redeem`. However, it inherits the default `maxWithdraw` and `maxRedeem` implementations from `StakediTry` (ERC4626), which return the user's full balance. This violates the EIP-4626 specification, which states that these functions MUST return 0 if the corresponding withdrawal/redemption would revert. This can cause integration failures for routers or UI that rely on the standard to determine availability.
 ### Static Signals
withdraw reverts but maxWithdraw returns > 0, ensureCooldownOff modifier
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Cross-chain cooldown reset griefing via Composer
 ### Description/Code Snippet
The `cooldownSharesByComposer` and `cooldownAssetsByComposer` functions in `StakediTryCrosschain` allow the `COMPOSER_ROLE` (trusted bridge contract) to initiate a cooldown for an arbitrary `redeemer` address. If the Composer contract's logic allows a source-chain user to specify any destination `redeemer`, a malicious user can send minimal assets to an existing staker's address. This action invokes `_startComposerCooldown`, which unconditionally resets the `cooldownEnd` timestamp to `block.timestamp + cooldownDuration`, effectively locking the victim's existing pending withdrawals for another full cooldown period (DoS/Griefing).
 ### Static Signals
role check after state change, Sensitive state-changing function lacks or misconfigures role/ownership checks
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: StakediTryCrosschain._startComposerCooldown

 ### Title
Cooldown duration reset attack via dust deposit
 ### Description/Code Snippet
The `_startComposerCooldown` function updates the `cooldowns[redeemer]` state by adding assets and unconditionally resetting `cooldownEnd` to `block.timestamp + cooldownDuration`. Since this function is triggered by the `COMPOSER_ROLE` (the bridge) based on cross-chain messages, an attacker can bridge dust amounts to a victim's address. This action resets the victim's cooldown timer repeatedly, permanently preventing them from unstaking their principal ('Denial of Service' via state manipulation).
 ### Static Signals
accounting state not updated when underlying asset is swapped/upgraded, index decreases
 ### Assets at Risk
user funds (DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: StakediTryCrosschain._startComposerCooldown

 ### Title
Unsafe Downcasting in Cooldown Accounting
 ### Description/Code Snippet
In `_startComposerCooldown`, the `assets` amount (uint256) is explicitly cast to `uint152` when adding to `cooldowns[redeemer].underlyingAmount`. Solidity 0.8.x does not revert on explicit casting truncation. If a cooldown is initiated for an amount greater than ~5e45 (2^152), the significant bits are discarded. The `_withdraw` function moves the full `assets` amount to the Silo, but the user is only credited with the truncated amount in the cooldown mapping, leading to permanent locking of the difference in the Silo.
 ### Static Signals
uint152(assets), +=
 ### Assets at Risk
user funds, silo balance
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTryCrosschain.unstakeThroughComposer

 ### Title
Shared Cooldown State allows Composer to seize Local Stakers' funds
 ### Description/Code Snippet
The `unstakeThroughComposer` function enables the `COMPOSER_ROLE` to finalize the cooldown of *any* address and receive the assets (presumably to bridge them). However, the `cooldowns` mapping is shared between local users (who call `cooldownAssets`) and cross-chain users. The contract fails to verify that the cooldown being claimed was actually initiated by the Composer. This allows the Composer (or an attacker abusing the Composer) to drain the assets of local users who are in the cooldown phase, effectively bypassing their custody.
 ### Static Signals
onlyRole(COMPOSER_ROLE), silo.withdraw(msg.sender, assets), cooldowns[receiver]
 ### Assets at Risk
silo, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryCrosschain.fastRedeemThroughComposer

 ### Title
Missing slippage protection in cross-chain fast redeem
 ### Description/Code Snippet
The `fastRedeemThroughComposer` function executes a redemption of shares for assets, deducting a mutable fee (`fastRedeemFeeInBPS`) and using the current exchange rate. It lacks a `minAmountOut` parameter to bound the minimum assets received. Since this function is part of a cross-chain flow (via LayerZero), the latency between initiation and execution exposes users to significant slippage risk if the admin changes fees or the NAV fluctuates while the message is in flight.
 ### Static Signals
payout calculated at execution time without minimum bound, no minOut parameter
 ### Assets at Risk
User redemption proceeds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

