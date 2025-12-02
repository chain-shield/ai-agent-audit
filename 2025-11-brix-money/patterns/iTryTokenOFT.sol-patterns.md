## Verified Patterns Found: 24

## Verified Patterns Found in following Categories:

- StandardViolation
- FeeOnTransferAssumption
- SlippageMissingOrInsufficient
- AccessControlOrAuthByPass
- GriefableCallbacks
- UnsafeRecipient
- ConfigFootgun
- GovernanceDelegationFlaw
- ExternalCallAfterStateChange
- AccountingInvariantViolation
- MaturityorGatingByPass



## Summary of Patterns

Whitelist Bypass in iTryTokenOFT Cross-chain Transfers

Cross-chain Cooldown Reset DoS Attack

StakediTry Reward Distribution DoS

Cross-chain Unstake State Desynchronization

Unstake Logic Lockup via Griefing Callback

Inconsistent Cooldown Bypass in Crosschain Logic forces wait during emergency release

iTrySilo lacks rescue mechanism for untracked or accidental transfers

Griefing of withdrawal time bounds via rolling cooldown extension

Stuck Funds in Silo due to Incomplete Confiscation Logic

maxWithdraw/maxRedeem Violate ERC4626 Spec During Cooldown

Slippage protection missing in fee-based fast redemption

Blacklisted user assets permanently locked in iTrySilo

Unsafe ERC20 transfer in iTrySilo.withdraw ignores return value

Admin Redistribution to Rewards Blocked by Active Vesting

Cross-chain Cooldown Reset Griefing

Blacklist Bypass via Cross-chain Unstake

Cross-Chain DoS via Unsafe Blacklist Redirection

Missing rescue mechanism in iTrySilo causes stuck funds

Confiscation Blocked by MinShares Invariant

Slippage Missing in Fast Redeem/Withdraw

ERC4626 Standard Violation in maxWithdraw/maxRedeem

Bypass of Restricted Role logic in unstake() allows restricted users to withdraw

Fee-On-Transfer/Rebasing Token Accounting Mismatch

Missing Slippage Protection in Staking/Redemption

## Patterns



 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTryTokenOFT.sol._beforeTokenTransfer

 ### Title
Whitelist Bypass in iTryTokenOFT Cross-chain Transfers
 ### Description/Code Snippet
In `iTryTokenOFT._beforeTokenTransfer`, the logic for `TransferState.WHITELIST_ENABLED` allows the `minter` (LayerZero Endpoint) to mint or burn tokens as long as the user is not blacklisted, ignoring the whitelist check. This allows non-whitelisted users to bridge tokens in and out (mint/redeem), violating the protocol invariant that only whitelisted users can transact in this state.
 ### Static Signals
msg.sender == minter, !blacklisted[from], no whitelisted[from] check in minter branch
 ### Assets at Risk
Compliance/Regulatory State
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Cross-chain Cooldown Reset DoS Attack
 ### Description/Code Snippet
In `StakediTryCrosschain.sol`, `cooldownSharesByComposer` and `cooldownAssetsByComposer` unconditionally overwrite the `cooldownEnd` timestamp for a target `redeemer` to `block.timestamp + cooldownDuration`. 

If the system allows users to initiate cross-chain stakes to arbitrary recipients (a common feature in LayerZero/OFT integrations), an attacker can send a dust amount (1 wei) to a victim who is near the end of their unstaking cooldown. This action resets the victim's `cooldownEnd`, effectively locking their funds for another full cooldown cycle. Repeating this attack prevents the victim from ever withdrawing.
 ### Static Signals
cooldowns[redeemer].cooldownEnd = uint104(block.timestamp) + cooldownDuration, onlyRole(COMPOSER_ROLE)
 ### Assets at Risk
user funds (DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: StakediTry.sol._updateVestingAmount

 ### Title
StakediTry Reward Distribution DoS
 ### Description/Code Snippet
The `_updateVestingAmount` function in `StakediTry` reverts with `StillVesting` if `getUnvestedAmount() > 0`. This enforcement prevents the addition of new rewards until the previous vesting period (configurable up to 30 days) has fully elapsed. This contradicts the documented goal of daily yield distribution if the vesting period is set to a standard duration (e.g., 30 days), as it forces a discrete 'stop-and-go' reward cycle rather than continuous streaming.
 ### Static Signals
if (getUnvestedAmount() > 0) revert
 ### Assets at Risk
Yield Distribution
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: StakediTryCrosschain.sol.unstakeThroughComposer

 ### Title
Cross-chain Unstake State Desynchronization
 ### Description/Code Snippet
In `StakediTryCrosschain`, the cooldown state is shared between the cross-chain composer flow and direct Hub chain interactions. A user can initiate a cross-chain unstake (locking assets in Silo), wait for the cooldown, and then claim the assets directly on the Hub chain via `unstake()`. When the cross-chain automation later calls `unstakeThroughComposer`, the assets are already claimed (amount is 0), leading to a zero-value transfer or potential bridge message failure, breaking the expected Spoke-Hub-Spoke asset flow.
 ### Static Signals
cooldowns[receiver], silo.withdraw(msg.sender, assets)
 ### Assets at Risk
Cross-chain Asset Consistency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: StakediTryV2.unstake

 ### Title
Unstake Logic Lockup via Griefing Callback
 ### Description/Code Snippet
The `unstake` function in `StakediTryV2` (and `unstakeThroughComposer` in `StakediTryCrosschain`) relies on `silo.withdraw` successfully transferring assets to the receiver. If the `iTry` token transfer reverts (e.g., if the user is blacklisted by the token contract after starting cooldown), the entire `unstake` transaction reverts. Since the user's shares were burned upon entering cooldown, and there is no administrative function to rescue funds from the `iTrySilo` or re-credit the cooldown, the user's funds become permanently locked in the Silo.
 ### Static Signals
no try/catch around external hook, withdrawal blocked because recipient's receive() reverts
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: StakediTryCrosschain.unstakeThroughComposer

 ### Title
Inconsistent Cooldown Bypass in Crosschain Logic forces wait during emergency release
 ### Description/Code Snippet
The `StakediTryV2` contract allows users to `unstake` immediately if `cooldownDuration` is set to 0 by the admin (e.g., in an emergency). However, the `StakediTryCrosschain` contract's `unstakeThroughComposer` function strictly checks `block.timestamp >= userCooldown.cooldownEnd` without checking for the `cooldownDuration == 0` override. This creates an inconsistency where cross-chain users (via the Composer) are forced to wait out the full original cooldown period while direct users can exit immediately.
 ### Static Signals
missing override check, inconsistent gating logic
 ### Assets at Risk
silo assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: iTrySilo.N/A

 ### Title
iTrySilo lacks rescue mechanism for untracked or accidental transfers
 ### Description/Code Snippet
The `iTrySilo` contract holds the underlying `iTry` assets but strictly limits withdrawals to the amounts tracked by the `StakediTry` vault's `cooldowns` mapping via the `withdraw` function. There is no `rescueTokens` function (unlike the main vault) and no mechanism to sweep excess `iTry`. If `iTry` tokens are sent directly to the Silo address by mistake, or if the token balance drifts from the accounting (e.g., due to rounding or weird token behavior), those funds are permanently locked.
 ### Static Signals
no rescueTokens, immutable state, balance tracking references different source
 ### Assets at Risk
iTry tokens
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Griefing of withdrawal time bounds via rolling cooldown extension
 ### Description/Code Snippet
The `cooldownSharesByComposer` function in `StakediTryCrosschain` allows a trusted composer (triggered by a cross-chain message) to add shares to a user's existing cooldown position. This action resets the `cooldownEnd` timestamp for the entire accrued amount to `block.timestamp + duration`. An attacker can exploit this by periodically triggering small cross-chain deposits to a victim's address, perpetually extending their cooldown and effectively locking their funds (Denial of Service).
 ### Static Signals
value transfers executed without meaningful time bounds
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTry.redistributeLockedAmount

 ### Title
Stuck Funds in Silo due to Incomplete Confiscation Logic
 ### Description/Code Snippet
The `redistributeLockedAmount` function allows the admin to confiscate and redistribute the liquid shares (`balanceOf`) of a blacklisted user. However, it does not account for assets already moved to the `iTrySilo` during the cooldown process. If a blacklisted user has a pending cooldown, their shares are already burned, and their assets are locked in the Silo. Since `unstake` reverts for blacklisted users and `redistributeLockedAmount` only affects share balances, these assets become permanently stuck in the Silo with no mechanism for the admin to seize or rescue them.
 ### Static Signals
burns shares only, ignores cooldowns mapping, no silo rescue function
 ### Assets at Risk
siloed assets
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.sol.maxWithdraw

 ### Title
maxWithdraw/maxRedeem Violate ERC4626 Spec During Cooldown
 ### Description/Code Snippet
The `StakediTryV2` contract disables standard `withdraw` and `redeem` when `cooldownDuration > 0`, but fails to override `maxWithdraw` and `maxRedeem` to return 0 in this state. This violates the ERC4626 specification which requires these view functions to reflect the actual withdrawable amount (0 if paused), potentially causing external integrators or routers to revert during execution.
 ### Static Signals
withdraw reverts but maxWithdraw returns balance
 ### Assets at Risk
Integrator Funds/Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryFastRedeem.fastRedeem

 ### Title
Slippage protection missing in fee-based fast redemption
 ### Description/Code Snippet
The `fastRedeem` and `fastWithdraw` functions in `StakediTryFastRedeem` (and their cross-chain counterparts in `StakediTryCrosschain`) allow users to redeem iTRY instantly by paying a fee. This fee is mutable (up to 20%) and the exchange rate is variable (due to reward vesting). These functions lack `minAssetsOut` or `maxSharesIn` parameters, exposing users to significant value loss if the fee is raised or the exchange rate fluctuates unfavorably before the transaction is executed.
 ### Static Signals
payout calculated at execution time without minimum bound, no minAmountOut parameter in liquidation/redemption
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTrySilo.withdraw

 ### Title
Blacklisted user assets permanently locked in iTrySilo
 ### Description/Code Snippet
When a user initiates a cooldown, their underlying `iTry` assets are transferred to the `iTrySilo`. If the user is subsequently blacklisted by the protocol (via `iTry` token blacklist), the `unstake` function will revert because `iTry.transfer` blocks transfers to blacklisted addresses. Unlike `iTry` tokens or `wiTry` shares which can be confiscated/redistributed by the Admin via `redistributeLockedAmount`, the Admin has no mechanism to withdraw or redistribute the specific raw assets sitting in `iTrySilo` corresponding to a blacklisted user's cooldown. These funds become permanently locked.
 ### Static Signals
withdraw restricted to staking vault, token transfer subject to blacklist, no admin rescue for specific siloed funds
 ### Assets at Risk
siloed iTry tokens
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ExternalCallAfterStateChange

 ### Relevant Function/Location: iTrySilo.withdraw

 ### Title
Unsafe ERC20 transfer in iTrySilo.withdraw ignores return value
 ### Description/Code Snippet
The `iTrySilo.withdraw` function uses `iTry.transfer(to, amount)` instead of `iTry.safeTransfer(to, amount)`, despite importing and declaring `using SafeERC20 for IERC20`. The `iTry` token is upgradeable. If the token implementation is upgraded to one that returns `false` on failure (instead of reverting) or does not return a boolean (like USDT), the transfer may fail silently. In `StakediTryV2.unstake`, the user's debt (`cooldowns[user].underlyingAmount`) is zeroed out before calling `silo.withdraw`. If the transfer fails silently, the user loses their claim to the assets without receiving them.
 ### Static Signals
using SafeERC20 for IERC20, call to .transfer() instead of .safeTransfer()
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: GovernanceDelegationFlaw

 ### Relevant Function/Location: StakediTry.redistributeLockedAmount

 ### Title
Admin Redistribution to Rewards Blocked by Active Vesting
 ### Description/Code Snippet
The `redistributeLockedAmount` function allows the admin to confiscate funds from blacklisted users. If `to == address(0)`, the funds are intended to be added to the reward vesting stream via `_updateVestingAmount`. 

However, `_updateVestingAmount` reverts with `StillVesting()` if `getUnvestedAmount() > 0`. In an active protocol where rewards are distributed frequently, vesting will likely always be active. This creates a logic conflict where the admin cannot confiscate funds to the reward pool without waiting for the entire vesting period to expire naturally.
 ### Static Signals
if (to == address(0)) { _updateVestingAmount(iTryToVest); }, if (getUnvestedAmount() > 0) revert StillVesting();
 ### Assets at Risk
confiscated funds (stuck)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Cross-chain Cooldown Reset Griefing
 ### Description/Code Snippet
The `StakediTryCrosschain` contract allows a trusted `composer` (triggered by a cross-chain message) to initiate or add to a cooldown for any `redeemer` address via `cooldownSharesByComposer`. If the cross-chain messaging layer allows a sender to specify the `redeemer` (destination address) or if an attacker can manipulate the message payload, they can send a negligible amount of assets to an existing staker's cooldown. The `_startComposerCooldown` function blindly overwrites `cooldowns[redeemer].cooldownEnd` to `block.timestamp + cooldownDuration`. This resets the maturity timer for the victim, enabling a perpetual Denial of Service on their withdrawal by repeating the attack just before maturity.
 ### Static Signals
cooldownEnd = block.timestamp + cooldownDuration, input address redeemer, no check for existing cooldown maturity
 ### Assets at Risk
user funds (locked indefinitely)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTryCrosschain.unstakeThroughComposer

 ### Title
Blacklist Bypass via Cross-chain Unstake
 ### Description/Code Snippet
The `unstakeThroughComposer` function in `StakediTryCrosschain` allows the Composer to withdraw assets from the `iTrySilo` on behalf of a `receiver`. It calls `silo.withdraw(msg.sender, assets)` where `msg.sender` is the Composer. The underlying `iTry` token transfer checks the blacklist status of `from` (Silo) and `to` (Composer), but not the `receiver` (the actual user). Consequently, a user who is blacklisted on the Hub chain (but not on the Spoke chain) can bypass the transfer restriction by initiating an unstake via the cross-chain composer, effectively exfiltrating their funds despite the `FULL_RESTRICTED_STAKER_ROLE`.
 ### Static Signals
silo.withdraw(msg.sender), no check for FULL_RESTRICTED_STAKER_ROLE on receiver
 ### Assets at Risk
confiscated funds, compliance integrity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
Cross-Chain DoS via Unsafe Blacklist Redirection
 ### Description/Code Snippet
In `wiTryOFT.sol`, the `_credit` function intercepts incoming cross-chain transfers to blacklisted addresses and redirects them to `owner()`. If `owner()` is set to `address(0)` (renounced ownership) or is a contract that rejects transfers (e.g. no fallback), the `_mint` operation will revert. In LayerZero v2, a reverting `lzReceive` handler can block the message channel (depending on ordered nonce configuration), effectively causing a Denial of Service for all bridge users.
 ### Static Signals
no zero-address guard on owner(), redirects transfer to owner()
 ### Assets at Risk
bridge availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTrySilo.N/A

 ### Title
Missing rescue mechanism in iTrySilo causes stuck funds
 ### Description/Code Snippet
The `iTrySilo` contract lacks a `rescueTokens` function or any mechanism for the `STAKING_VAULT` to withdraw excess funds. If users accidentally transfer `iTry` directly to the Silo, or if the protocol sends excess funds, these tokens become permanently locked. While `StakediTryV2` has a `rescueTokens` function, it interacts with the vault's balance, not the Silo's. Since the Silo's balance is strictly tied to `cooldowns` logic in `StakediTryV2`, any deviation (excess balance) creates an unresolvable accounting invariant violation where funds exist but cannot be retrieved.
 ### Static Signals
no rescueTokens function, no withdrawExcess function
 ### Assets at Risk
stuck iTry tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: StakediTry.redistributeLockedAmount

 ### Title
Confiscation Blocked by MinShares Invariant
 ### Description/Code Snippet
The `redistributeLockedAmount` function in `StakediTry.sol` is designed to confiscate funds from restricted users. It calls `_burn` followed immediately by `_checkMinShares`. If the burn reduces the `totalSupply` to a value between 0 and `MIN_SHARES` (1 ether), `_checkMinShares` reverts. This creates an edge case where the admin cannot confiscate funds if the remaining valid supply is small (dust), effectively causing a DoS on the compliance mechanism.
 ### Static Signals
accounting state check between burn and mint
 ### Assets at Risk
governance/compliance capability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryFastRedeem.fastRedeem, fastWithdraw

 ### Title
Slippage Missing in Fast Redeem/Withdraw
 ### Description/Code Snippet
The `fastRedeem` and `fastWithdraw` functions in `StakediTryFastRedeem.sol` allow users to exchange shares for assets (and vice versa) instantly by paying a fee. However, these functions calculate the payout and fee at execution time based on the current state without accepting user-defined slippage bounds (`minAssetsOut` or `maxSharesIn`). A user submitting a transaction can be front-run by a fee increase (up to 20%) or suffer from share price manipulation/volatility, resulting in a significant loss of value.
 ### Static Signals
no minAmountOut parameter, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.maxWithdraw

 ### Title
ERC4626 Standard Violation in maxWithdraw/maxRedeem
 ### Description/Code Snippet
The `StakediTryV2` contract overrides `withdraw` and `redeem` to revert when `cooldownDuration > 0` (via `ensureCooldownOff`), effectively disabling immediate withdrawals. However, it does not override `maxWithdraw` or `maxRedeem`. 

Consequently, these view functions continue to return the user's full balance as withdrawable, violating the ERC-4626 specification which states `maxWithdraw` must factor in global and user-specific limits (including pauses or disabled states). This breaks integration with routers and adapters that check `maxWithdraw` before attempting execution.
 ### Static Signals
withdraw(...) ensureCooldownOff, no override function maxWithdraw
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: StakediTryV2.unstake

 ### Title
Bypass of Restricted Role logic in unstake() allows restricted users to withdraw
 ### Description/Code Snippet
The `unstake` function in `StakediTryV2` does not check if the caller has the `FULL_RESTRICTED_STAKER_ROLE`. While this role is checked in `_deposit` and `_withdraw`, `unstake` allows a user who has already entered the cooldown period to release their funds. Furthermore, the `redistributeLockedAmount` admin function only burns `wiTry` shares (`balanceOf`) and does not account for or seize pending assets in the `cooldowns` mapping. A user can front-run a restriction by calling `cooldownAssets`, waiting for the blacklist, and then successfully calling `unstake` to exit with funds, bypassing the intended freeze.
 ### Static Signals
role check missing, state clearing does not check role, redistribution logic misses cooldown state
 ### Assets at Risk
silo assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: StakediTry.transferInRewards

 ### Title
Fee-On-Transfer/Rebasing Token Accounting Mismatch
 ### Description/Code Snippet
The `transferInRewards` function updates the internal `vestingAmount` by the input `amount` before transferring tokens. It assumes the vault receives exactly `amount`. If `iTry` (which is upgradeable) implements a transfer fee or rebalancing mechanism, the vault's `balanceOf` will increase by less than `amount`, while `vestingAmount` increases by `amount`. This causes `totalAssets()` (balance - unvested) to underflow or artificially decrease, damaging the share price for all stakers.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, uses input amount instead of post-transfer delta
 ### Assets at Risk
vault shares, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryV2.cooldownAssets

 ### Title
Missing Slippage Protection in Staking/Redemption
 ### Description/Code Snippet
The `cooldownAssets`/`cooldownShares` functions in `StakediTryV2` and the `fastRedeemThroughComposer`/`fastWithdrawThroughComposer` functions in `StakediTryCrosschain` execute asset-to-share conversions without accepting minimum output or maximum input parameters. This exposes users (and cross-chain composers) to slippage from exchange rate fluctuations caused by reward vesting, yield updates, or market volatility between transaction submission and execution.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

