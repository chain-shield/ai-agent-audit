## Verified Patterns Found: 12

## Verified Patterns Found in following Categories:

- FlashLoanEconomicManipulation
- StandardViolation
- SlippageMissingOrInsufficient
- AccessControlOrAuthByPass
- FeeOnTransferAssumption
- UnsafeRecipient
- AccountingInvariantViolation
- UnboundedLoops



## Summary of Patterns

MEVCapture Fee Logic Enables Griefing via Cumulative Tick Deviation

Storage Collision in Incentives allows corrupting arbitrary DropState via malicious index

Missing Slippage Protection in Liquidity Withdrawal

Unsafe Recipient in Position Withdrawal

Missing Max Input Bound in Fund Funding

TokenWrapper fails to mint tokens to user during wrapping

Storage Corruption via Unbounded Claim Index

Unbounded Loop in TWAMM Virtual Order Execution

Airdrop insolvency via Fee-on-Transfer token incompatibility

Unbounded funding cost in `fund` exposes donors to slippage/front-running

Accounting mismatch for Fee-on-Transfer tokens leading to stuck funds and DoS

Fee-on-Transfer tokens break accounting invariant in singleton Incentives contract

## Patterns



 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.sol.handleForwardData

 ### Title
MEVCapture Fee Logic Enables Griefing via Cumulative Tick Deviation
 ### Description/Code Snippet
The `MEVCapture` extension calculates an additional swap fee based on the absolute deviation of the post-swap tick from `tickLast` (`abs(stateAfter.tick() - tickLast)`). `tickLast` is updated to the current tick *only* when the block timestamp changes (`lastUpdateTime != currentTime`). Within a single block, `tickLast` remains anchored to the tick at the start of the block (or first interaction). 

This creates a vulnerability where a user executing a swap later in the block pays a fee proportional to the *total* price movement of the block, not just their own impact. An attacker can manipulate the price (move tick from T0 to T1000) paying a fee on 1000 ticks. A subsequent victim swapping slightly (T1000 to T1001) will be charged a fee based on 1001 ticks, effectively paying for the attacker's movement again. The attacker can then reverse their trade (T1001 to T0) paying zero fees (delta is 0), effectively griefing the victim with exorbitant fees.
 ### Static Signals
branches on pool.balanceOf()/getReserves(), share price derived from manipulable pool state
 ### Assets at Risk
user funds (excessive fees)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Incentives.claim

 ### Title
Storage Collision in Incentives allows corrupting arbitrary DropState via malicious index
 ### Description/Code Snippet
In `Incentives.sol`, the `claim` function calls `IncentivesLib.claimIndexToStorageIndex` to determine the storage slot for the claimed bitmap. The slot is calculated as `id + 1 + (index >> 8)`. Since `index` is a user-controlled parameter (verified only against the Merkle root provided by the drop creator) and is not checked for size, an attacker can create a malicious drop with a specific large `index` such that the calculated `bitmapSlot` wraps around the storage space and aliases the `DropState` slot of a victim drop (or their own). The `claim` function then toggles a bit in that slot. This allows an attacker to corrupt the `funded` or `claimed` values of any drop. By flipping the most significant bit of `funded` on their own drop, an attacker can artificially inflate their funded balance and drain all matching tokens from the Incentives contract via `refund`.
 ### Static Signals
sstore(bitmapSlot, bitmap), unchecked index usage in storage calculation, no bounds check on index
 ### Assets at Risk
tokens held in Incentives contract
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.sol.withdraw

 ### Title
Missing Slippage Protection in Liquidity Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` calculates the amounts of `token0` and `token1` to return to the user based on the provided `liquidity` and the current pool tick (price). However, it does not accept user-supplied `amount0Min` and `amount1Min` parameters to enforce minimum output amounts. This omission exposes liquidity providers to sandwich attacks or high volatility where the pool price is manipulated or shifts unfavorably before the transaction executes, causing the withdrawal to return an unexpected and potentially unfavorable ratio of assets compared to the user's expectation.
 ### Static Signals
payout calculated at execution time without minimum bound, withdraw() converts shares to assets at current rate without minimum
 ### Assets at Risk
user liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: BasePositions.sol.withdraw

 ### Title
Unsafe Recipient in Position Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` accepts a `recipient` address argument but does not check if it is `address(0)`. If a user incorrectly passes the zero address, the withdrawn tokens are sent to `address(0)` via the `FlashAccountant`'s `withdrawTwo` function (which calls the token transfer functions), effectively burning the funds.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Incentives.sol.fund

 ### Title
Missing Max Input Bound in Fund Funding
 ### Description/Code Snippet
The `fund` function calculates the amount of tokens to pull from the caller as `minimum - currentFunded` at execution time. There is no parameter to limit the maximum amount transferred. If `currentFunded` decreases (e.g., due to a front-run `refund` call by the owner) between transaction signing and execution, the `fundedAmount` will increase, causing the caller to pay significantly more than intended to reach the target `minimum`.
 ### Static Signals
payout calculated at execution time without minimum bound, input calculated from state without maximum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper fails to mint tokens to user during wrapping
 ### Description/Code Snippet
The `TokenWrapper` contract is designed to wrap an underlying token into a time-locked token. However, in `handleForwardData`, while the contract correctly updates the Core's `savedBalances` to reflect the deposit of the underlying token, it fails to credit the `_balanceOf` mapping for the user (or the `original` locker) with the newly created wrapper tokens. Since `_balanceOf` is the only source of truth for the ERC20 functionality of `TokenWrapper` and it is never increased, users deposit funds but receive no wrapper tokens in return, and all subsequent calls to `transfer` or `approve` will fail due to zero balance.
 ### Static Signals
missing balance update, transfer logic inconsistent with minting
 ### Assets at Risk
User funds deposited into TokenWrapper
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Incentives.claim

 ### Title
Storage Corruption via Unbounded Claim Index
 ### Description/Code Snippet
The `claim` function derives the storage slot for the claim bitmap using `id + 1 + (c.index >> 8)` without checking if `c.index` is within a reasonable range. An attacker can craft a Merkle tree with a very large `c.index` such that the calculated `bitmapSlot` wraps around or aliases the storage slot of another Drop (`DropState`) or the contract's own `id` slot. This allows the attacker to toggle bits in the `DropState` (e.g., modifying `funded` or `claimed` amounts), enabling theft of funds from the contract.
 ### Static Signals
bitmapSlot = StorageSlot.wrap(bytes32(uint256(id) + 1 + word)), no check on c.index
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM.sol._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
In `TWAMM.sol`, the function `_executeVirtualOrdersFromWithinLock` uses a `while` loop to iterate from `realLastVirtualOrderExecutionTime` to `block.timestamp`. It processes time intervals defined by the initialization bitmap. If a pool has not been interacted with for a significant period, or if an attacker populates the bitmap with highly granular intervals (depending on `isTimeValid` constraints), the number of iterations required to catch up to the current block may consume more gas than the block limit. This would result in a Denial of Service (DoS), rendering the pool permanently unusable as any subsequent interaction attempts to execute the expensive backlog and fails.
 ### Static Signals
iteration count grows with contract state, no gas limit checks in loop body, loop bound depends on attacker-controlled value
 ### Assets at Risk
liquidity pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.sol.fund

 ### Title
Airdrop insolvency via Fee-on-Transfer token incompatibility
 ### Description/Code Snippet
The `fund` function in `Incentives.sol` calculates the `fundedAmount` to pull from the user as `minimum - currentFunded` and executes a `safeTransferFrom` for exactly this amount. It then updates the internal `funded` accounting variable by the same amount. However, it fails to verify the actual amount of tokens received by the contract. 

If the token charges a fee on transfer, the `Incentives` contract will receive less than the accounting record indicates. Since the contract holds pooled assets for all drops, this creates a discrepancy. For a specific drop, the `getRemaining()` function (used in `claim` and `refund`) will report more tokens available than actually exist for that drop. This leads to two negative outcomes: 
1. The last users to claim (or the owner trying to refund) will face a revert due to insufficient balance in the contract.
2. In a multi-drop scenario sharing the same token, a drop with a fee-on-transfer token effectively subsidizes its deficit by draining the balance belonging to other valid drops, causing insolvency for honest participants.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk
tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Incentives.fund

 ### Title
Unbounded funding cost in `fund` exposes donors to slippage/front-running
 ### Description/Code Snippet
The `fund` function calculates the amount to transfer (`fundedAmount`) based on `minimum - currentFunded`. It does not accept a `maxAmount` parameter. If the drop's `funded` amount decreases (e.g., via the Owner calling `refund`) before the transaction executes, the caller will be forced to pay significantly more than expected. This allows a Drop Owner to front-run a donor's 'top-up' transaction to force them to fully replenish the drop.
 ### Static Signals
payout calculated at execution time without minimum bound, amountOutMin=0 or missing
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Accounting mismatch for Fee-on-Transfer tokens leading to stuck funds and DoS
 ### Description/Code Snippet
The `fund` function calculates the `fundedAmount` as the difference between the requested `minimum` and the current recorded funding, then transfers this exact amount using `SafeTransferLib.safeTransferFrom`. It immediately updates the internal `dropState` assuming the contract received the full `fundedAmount`. However, if the asset is a Fee-on-Transfer (FOT) token, the contract actually receives `fundedAmount - fee`. This creates a discrepancy where the internal accounting (`funded`) exceeds the actual token balance available. 

Consequently, `claim` functions will eventually revert for the last claimants due to insufficient contract balance (despite passing the internal `remaining` check), causing a Denial of Service. Furthermore, the `refund` function attempts to sweep the entire `remaining` calculated amount to the owner; this will consistently revert because the contract lacks the full balance, causing the owner's remaining funds to be permanently stuck (unless they top up the difference manually).
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check, accounting based on transfer parameter
 ### Assets at Risk
Owner funds (stuck), Unclaimed airdrop rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Incentives.fund

 ### Title
Fee-on-Transfer tokens break accounting invariant in singleton Incentives contract
 ### Description/Code Snippet
The `fund` function calculates the amount to transfer as `minimum - currentFunded` and updates `dropState.funded` by this exact amount. However, it uses `safeTransferFrom` which does not check the actual balance increase. If a Fee-on-Transfer token is used, the contract receives less than the state records. Since `Incentives` is a singleton holding shared balances for all drops, a deficit in one drop allows its owner/users to drain tokens belonging to other drops sharing the same token via `refund` or `claim`.
 ### Static Signals
balance tracking references different token than actual holdings, accounting state not updated when underlying asset is swapped/upgraded
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

