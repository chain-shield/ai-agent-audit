## Verified Patterns Found: 20

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption
- PermitFrontRun
- FlashLoanEconomicManipulation
- AccountingInvariantViolation
- PermitOrSignatureReplay
- UnsafeRecipient
- StandardViolation
- PermitMisuse
- GriefableCallbacks
- ReadOnlyReentrancy
- ReserveOrPriceDesync



## Summary of Patterns

DoS via Griefable Distributor Callback

Accounting Invariant Violation allowing Fee Skimming

Malleable signatures in permit allow front-running

DoS on USDT pairs due to unsafe approval reset

Permit signature malleability allows front-running DoS

Trading DoS due to SafeApprove Incompatibility (USDT)

Replayable Permits due to Static Domain Separator

Flash Loan Fee Manipulation via LP Token Transfer during Swap

Accrued Launchpad Fees Theft via mint()

Flash-minting liquidity dilutes launchpad fee revenue

Read-Only Reentrancy in Swap Callback

Reserve Desync via Skim if Distributor Fails to Pull

`endRewardsAccrual` deletes accrued fees without distribution

Read-Only Reentrancy via Rewards Distribution

Skimming of Accrued Fees via Accounting Invariant Violation

Burn Revert Due to Fee Accounting Mismatch

Fee-on-transfer tokens cause Distributor insolvency

USDT/Non-standard Token Approval Revert in Fee Distribution

Launchpad Fee Dilution via Flash-Minted Liquidity Inflation

Denial of Service via Reverting Distributor Callback

## Patterns



 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
DoS via Griefable Distributor Callback
 ### Description/Code Snippet
The `_update` function, which is critical for `swap`, `mint`, `burn`, and `sync`, calls `IDistributor(launchpadFeeDistributor).addRewards`. If the external distributor contract reverts (due to logic error, gas limits, or intentional pausing), the entire Pair contract is Denial-of-Service (DoS) bricked, as all state-changing functions will fail.
 ### Static Signals
call to external contract in critical state update, no try/catch around callback, callback failure causes revert
 ### Assets at Risk
Liquidity Pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.skim

 ### Title
Accounting Invariant Violation allowing Fee Skimming
 ### Description/Code Snippet
In `_update`, `reserve` is updated to `balance - totalLaunchpadFee`, assuming the fee tokens are transferred out. However, `_distributeLaunchpadFees` only approves the tokens; it relies on the Distributor to `transferFrom`. If the Distributor fails to pull the tokens (e.g. due to error or pause), the tokens remain in the contract balance but are excluded from `reserve`. The `skim` function calculates excess as `balance - reserve`, allowing anyone to claim these uncollected fees.
 ### Static Signals
emitted != claimed + unclaimed, accounting state not updated when underlying asset is swapped/upgraded
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: GTELaunchpadV2Pair.permit

 ### Title
Malleable signatures in permit allow front-running
 ### Description/Code Snippet
The `permit` function (inherited from `UniswapV2ERC20`) uses `ecrecover` without validating that the signature's `s` value is in the lower half of the curve (s <= secp256k1n/2). This allows an attacker to observe a valid pending `permit` transaction and submit a front-running transaction with the same `r` and an inverted `s` value. This second signature validates to the same address, consuming the user's nonce and causing the original transaction to revert, creating a Denial of Service vector for gasless approvals.
 ### Static Signals
ecrecover used without requiring s <= secp256k1n/2, nonces incremented on success
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._safeApprove

 ### Title
DoS on USDT pairs due to unsafe approval reset
 ### Description/Code Snippet
The `_safeApprove` function sets the allowance to `value` without first resetting it to 0. Certain tokens like USDT revert if `approve` is called with a non-zero value when the current allowance is already non-zero. If `_distributeLaunchpadFees` executes but the trusted Distributor fails to consume the entire allowance (or if a previous transaction failed to consume it), subsequent calls will revert, bricking the pair for that token.
 ### Static Signals
callback to arbitrary user-controlled address, no try/catch around external hook
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
Permit signature malleability allows front-running DoS
 ### Description/Code Snippet
The `permit` function in `UniswapV2ERC20` (inherited by `GTELaunchpadV2Pair`) uses `ecrecover` without verifying that the `s` value is in the lower half of the secp256k1 curve (`s <= 0x7FFFF...`). This allows a valid signature to be transformed into a second valid signature (using `secp256k1n - s`) for the same deadline and nonce. An attacker can front-run a user's permit transaction with the malleable signature, consuming the nonce and causing the user's original transaction to revert.
 ### Static Signals
ecrecover used without requiring s <= secp256k1n/2, nonces reused or not incremented (if front-run)
 ### Assets at Risk
user gas, transaction validity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: GTELaunchpadV2Pair._distributeLaunchpadFees

 ### Title
Trading DoS due to SafeApprove Incompatibility (USDT)
 ### Description/Code Snippet
The `_distributeLaunchpadFees` function uses `_safeApprove` to approve the `distributor` to spend accrued fees. The standard `_safeApprove` implementation reverts if attempting to approve a non-zero value when the current allowance is already non-zero (to prevent front-running, a behavior enforced by tokens like USDT). If `IDistributor.addRewards` fails to consume the exact full allowance (e.g., due to fee-on-transfer or internal calculation mismatches), a residual allowance remains. Subsequent swaps will then revert when trying to approve the new fee, bricking the pair.
 ### Static Signals
changes allowance from X to Y without zeroing
 ### Assets at Risk
trading_availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitOrSignatureReplay

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
Replayable Permits due to Static Domain Separator
 ### Description/Code Snippet
The `UniswapV2ERC20` contract calculates `DOMAIN_SEPARATOR` in the constructor using the chain ID at deployment time. It does not recompute it if the chain ID changes (e.g., after a hard fork). This allows valid permits signed on one chain to be replayed on a forked chain, potentially allowing unauthorized spending of user assets.
 ### Static Signals
DOMAIN_SEPARATOR immutable, no block.chainid check in permit
 ### Assets at Risk
user funds via permit
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
Flash Loan Fee Manipulation via LP Token Transfer during Swap
 ### Description/Code Snippet
The `_getLaunchpadFees` function calculates fees based on `balanceOf(launchpadLp)`. While `swap` is protected by the `lock` modifier, the inherited `transfer` function for LP tokens is not. An attacker (e.g., the `launchpadLp` owner) can flash-borrow LP tokens, transfer them to the `launchpadLp` address inside the `uniswapV2Call` callback, and inflate the `launchpadLpBal`. This forces the pool to deduct the maximum fee share from reserves, which is then sent to the Distributor (benefiting the attacker). The attacker can then retrieve their LP tokens. This siphons value from other LPs.
 ### Static Signals
branches on balanceOf(launchpadLp), unlocked LP token transfer during locked swap
 ### Assets at Risk
liquidity provider yield
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.mint

 ### Title
Accrued Launchpad Fees Theft via mint()
 ### Description/Code Snippet
The `mint()` function calculates the amounts provided by a user as `balance - reserve`. However, the `_update()` function subtracts accrued launchpad fees from the reserves without immediately removing the tokens from the balance (if within the same block or if distribution is delayed). This breaks the accounting invariant. An attacker can trigger fee accrual (e.g., via `swap` or `sync`) and then immediately call `mint()` without transferring tokens. `mint` will interpret the accrued fees (which sit in `balance` but not `reserve`) as new liquidity provided by the attacker, allowing them to steal the pending rewards.
 ### Static Signals
amount0 = balance0.sub(_reserve0), reserve0 updated with fees subtracted, fees not subtracted from balance in mint calculation
 ### Assets at Risk
accruedLaunchpadFee0, accruedLaunchpadFee1
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair._getLaunchpadFees

 ### Title
Flash-minting liquidity dilutes launchpad fee revenue
 ### Description/Code Snippet
The `_getLaunchpadFees` function calculates the fee diverted to the launchpad based on the ratio of the launchpad's LP balance to the total LP supply (`launchpadLpBal / totalLpBal`). An attacker can flash-mint a massive amount of LP tokens immediately before a swap (and burn them afterwards) to dilute this ratio to near zero. This allows the attacker to bypass the fee diversion mechanism, keeping the swap fees within the pool reserves (which they momentarily own the majority of), thereby depriving the launchpad distributor of its intended revenue.
 ### Static Signals
rewards share depends on manipulable totalSupply, fee calculated using instantaneous spot ratio
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
Read-Only Reentrancy in Swap Callback
 ### Description/Code Snippet
The `swap` function executes the `uniswapV2Call` callback to the recipient before calling `_update` to sync the new reserves. During this callback, the token balances of the contract have changed, but the result of `getReserves()` still reflects the state prior to the swap. If any third-party contract (or the `launchpadFeeDistributor`) relies on `getReserves` or derived prices during this callback, they will receive stale data, enabling read-only reentrancy attacks.
 ### Static Signals
external call before view function stabilizes, uses getReserves/spot price that can change intratx
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Reserve Desync via Skim if Distributor Fails to Pull
 ### Description/Code Snippet
In `_update`, `accruedLaunchpadFee` is deleted before calling `_distributeLaunchpadFees`. The logic subsequently calculates `reserve = balance - totalFee`, assuming the distributor pulls the fee. If the distributor records the reward but fails to pull the tokens (e.g., due to implementation specific behavior), `reserve` is reduced while `balance` remains high. The `skim` function then allows anyone to steal the undistributed fees.
 ### Static Signals
state deleted before external call, reserve invariant assumes external transfer
 ### Assets at Risk
accrued fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.endRewardsAccrual

 ### Title
`endRewardsAccrual` deletes accrued fees without distribution
 ### Description/Code Snippet
The `endRewardsAccrual` function deletes `accruedLaunchpadFee0` and `accruedLaunchpadFee1` and deactivates the rewards pool without first distributing these accrued amounts. In the subsequent `_update` call, `accrued` is treated as 0, causing the reserves (`reserve0`, `reserve1`) to be set to the full token balance (`balance0`, `balance1`). This effectively donates the accumulated uncollected fees back to the pool's liquidity providers (inflating `k`) instead of sending them to the `launchpadFeeDistributor`, resulting in a permanent loss of revenue for the launchpad participants.
 ### Static Signals
accounting state not updated when underlying asset is swapped/upgraded, emitted != claimed + unclaimed
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Read-Only Reentrancy via Rewards Distribution
 ### Description/Code Snippet
In `_update`, the contract calls `_distributeLaunchpadFees` (which makes an external call to the `distributor`) *before* updating `reserve0`, `reserve1`, and `blockTimestampLast`. During this external call, `getReserves()` returns stale values (pre-swap/mint/burn reserves) while `balanceOf()` reflects the new state. If the distributor or any connected system reads the pair's price via `getReserves` during the callback, it acts on invalid data.
 ### Static Signals
external call before view function stabilizes, uses getReserves/spot price that can change intratx
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Skimming of Accrued Fees via Accounting Invariant Violation
 ### Description/Code Snippet
The `_update` function reduces `reserve0` and `reserve1` by the amount of `totalLaunchpadFee` and approves the `distributor` to spend this amount. It assumes the `distributor.addRewards` call will synchronously `transferFrom` these tokens out of the contract. If the distributor implementation does not pull the tokens immediately (or if the transfer fails silently/partially), the tokens remain in the contract's balance but are excluded from the reserve accounting. This creates a discrepancy where `balanceOf(this) > reserve`, allowing any user to steal the untransferred fees by calling `skim()`.
 ### Static Signals
balance tracking references different token than actual holdings, accounting state not updated when underlying asset is swapped/upgraded
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.burn

 ### Title
Burn Revert Due to Fee Accounting Mismatch
 ### Description/Code Snippet
The `burn()` function calculates `amount0 = liquidity * balance0 / totalSupply`. If `balance0` contains accrued fees that haven't been distributed, a user burning a large portion of liquidity will withdraw their share plus a portion of the fees. Subsequently, `_update()` attempts to calculate `reserve0 = balance0 - accruedFees`. If the remaining `balance0` after the burn is less than `accruedFees`, the subtraction will underflow and revert, locking user funds and preventing liquidity removal.
 ### Static Signals
reserve calculation involves subtraction of fees, balance used in burn includes fees
 ### Assets at Risk
User Liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: GTELaunchpadV2Pair._distributeLaunchpadFees

 ### Title
Fee-on-transfer tokens cause Distributor insolvency
 ### Description/Code Snippet
The `_distributeLaunchpadFees` function approves the full `fee0` amount and calls `distributor.addRewards` with that same amount. For Fee-On-Transfer (FoT) tokens, the Distributor receives less than `fee0` during the transfer. However, the Distributor (consuming `RewardsTrackerLib`) typically accounts for rewards based on the input parameter (`fee0`). This discrepancy causes the Distributor's internal accounting to track more tokens than it physically holds, eventually leading to insolvency where the last users cannot claim their rewards.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair._safeApprove

 ### Title
USDT/Non-standard Token Approval Revert in Fee Distribution
 ### Description/Code Snippet
The function `_safeApprove` uses a low-level call to `approve` with a non-zero value. Tokens like USDT revert if `approve` is called with a non-zero value when the current allowance is already non-zero. In `_distributeLaunchpadFees`, `_safeApprove` is called every time fees are distributed. If `IDistributor.addRewards` fails to consume the entire allowance (e.g., due to partial transfers or logic updates), the subsequent call to `_safeApprove` in the next fee accrual will revert, causing a Denial of Service for the entire pair (swaps/mints/burns will fail).
 ### Static Signals
token.call(abi.encodeWithSelector(APPROVE_SELECTOR, to, value)), no approve(0) check
 ### Assets at Risk
liquidity pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: GTELaunchpadV2Pair._getLaunchpadFees

 ### Title
Launchpad Fee Dilution via Flash-Minted Liquidity Inflation
 ### Description/Code Snippet
The `_getLaunchpadFees` function calculates the Launchpad's fee share dynamically based on the ratio of the Launchpad's LP balance (`launchpadLpBal`) to the total LP supply (`totalLpBal`). An attacker can flash-mint a massive amount of LP tokens within a single transaction, inflating `totalSupply` and driving this ratio near zero. This manipulation allows the attacker (who now holds the majority of liquidity) to bypass the intended fee/tax that should go to the Distributor, effectively keeping the full 0.3% swap fee for themselves within the pool reserves.
 ### Static Signals
uses totalSupply/totalAssets in same tx as deposit/withdraw, fee calculated from manipulable pool state
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._distributeLaunchpadFees

 ### Title
Denial of Service via Reverting Distributor Callback
 ### Description/Code Snippet
The `swap`, `mint`, and `burn` functions all trigger `_update`, which subsequently calls `_distributeLaunchpadFees`. This function performs an external call to `IDistributor(distributor).addRewards` to transfer fees. There is no `try/catch` block or gas limit for this call. Consequently, if the `Distributor` contract reverts (due to logic error, pause state, or malicious upgrade), all core trading functionality on the pair is permanently blocked, violating the isolation principle of the AMM.
 ### Static Signals
no try/catch around external hook, external call in critical flow
 ### Assets at Risk
trading_availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

