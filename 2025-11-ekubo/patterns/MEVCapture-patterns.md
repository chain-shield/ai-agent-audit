## Verified Patterns Found: 16

## Verified Patterns Found in following Categories:

- TimelockEdgeCase
- SlippageMissingOrInsufficient
- FeeAccountingDrift
- AccessControlOrAuthByPass
- StandardViolation
- FlashLoanEconomicManipulation
- FeeOnTransferAssumption
- UnsafeRecipient
- UnboundedLoops
- AccountingInvariantViolation



## Summary of Patterns

MEV Capture Fee Bypass via Trade Splitting

Missing slippage protection in liquidity withdrawal

Incentives claim function potentially allows theft via malleable ClaimKey

Deposit lacks explicit amount slippage protection

Missing zero-address check for recipient in fee collection and withdrawal

Missing transaction deadline in deposit

Unsafe recipient allows burning funds

TWAMM virtual order execution allows DoS via dense time initialization

Unsafe cast to int128 in fee accounting causes DoS for large amounts

MEVCapture extension locks pools due to beforeSwap revert loop

Incompatibility with Fee-On-Transfer tokens

DoS via Unbounded Loop in TWAMM Virtual Order Execution

Burning Position NFT permanently locks underlying liquidity

Incompatibility with Fee-on-Transfer Tokens

UnsafeRecipient in withdraw and collectFees

Missing slippage protection and deadline in position withdrawal

## Patterns



 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture Fee Bypass via Trade Splitting
 ### Description/Code Snippet
The `MEVCapture` extension calculates the MEV fee based on the absolute tick displacement from the start of the block to the end of the current swap (`abs(stateAfter.tick() - tickLast)`). Since the fee rate scales linearly with displacement (up to a cap) and applies to the entire swap amount, splitting a large trade into multiple smaller trades allows a trader to pay significantly lower fees. The earlier swaps in the sequence pay fees based on smaller displacements, bypassing the intended progressive fee curve designed to capture LVR/MEV.
 ### Static Signals
fee taken before scaling normalization, caller skims dust each claim via rounding
 ### Assets at Risk
Protocol Revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing slippage protection in liquidity withdrawal
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows users to burn liquidity for tokens but does not accept `amount0Min` or `amount1Min` parameters. The amounts of `token0` and `token1` returned depend on the pool's current price (tick). An attacker can sandwich the withdrawal transaction, manipulating the price to ensure the user receives a less desirable ratio or value of tokens than expected.
 ### Static Signals
withdraw(..., liquidity), no minAmount params, returns (amount0, amount1)
 ### Assets at Risk
User liquidity/withdrawn tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Incentives.claim

 ### Title
Incentives claim function potentially allows theft via malleable ClaimKey
 ### Description/Code Snippet
The `claim` function in `Incentives.sol` takes a `ClaimKey` struct (containing `account`, `amount`, etc.) and a merkle proof. It reconstructs the leaf using `c.toClaimId()`. If `toClaimId` does not include `c.account` in the hash generation, an attacker can observe a valid proof for a victim and submit the same proof with a modified `ClaimKey` where `account` is the attacker's address, stealing the claim.
 ### Static Signals
struct passed to leaf generation, transfer to address in struct, merkle proof verification
 ### Assets at Risk
incentive tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: BasePositions.deposit

 ### Title
Deposit lacks explicit amount slippage protection
 ### Description/Code Snippet
The `deposit` function relies solely on `minLiquidity` and `maxAmount` constraints. It does not accept `amount0Min` or `amount1Min` parameters. If a user provides loose `maxAmount` caps to ensure execution, an attacker can manipulate the pool price (sandwich attack) to force the deposit to consume a skewed ratio of tokens (e.g., 100% token0, 0% token1) while still satisfying the `minLiquidity` threshold. This exposes users to immediate impermanent loss/bad entry price, as they cannot enforce a minimum valid ratio or amount for each token individually.
 ### Static Signals
missing amount0Min param, missing amount1Min param, slippage check only on liquidity
 ### Assets at Risk
User deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: BasePositions.collectFees

 ### Title
Missing zero-address check for recipient in fee collection and withdrawal
 ### Description/Code Snippet
The `collectFees` and `withdraw` functions in `BasePositions` accept a `recipient` address argument but do not validate that it is non-zero. If a user or frontend accidentally passes `address(0)` (the default value for uninitialized address variables), the funds will be withdrawn from the pool and effectively burned (transferred to the zero address), resulting in permanent loss of funds.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.deposit

 ### Title
Missing transaction deadline in deposit
 ### Description/Code Snippet
The `deposit` function allows users to add liquidity to a position. While it includes `minLiquidity` for slippage protection regarding the minted amount, it lacks a `deadline` timestamp parameter. Transactions without a deadline can be held by validators/miners and executed at a later time when market conditions or the pool price have shifted unfavorably against the user's intent.
 ### Static Signals
deadline omitted
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Unsafe recipient allows burning funds
 ### Description/Code Snippet
The `withdraw` and `collectFees` functions in `Positions.sol`, as well as `swap` in `Router.sol`, accept a `recipient` address argument but fail to validate that it is not `address(0)`. If a user accidentally passes the zero address, the protocol will execute transfers to `address(0)`, permanently burning the tokens.
 ### Static Signals
address recipient, no zero address check
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TimelockEdgeCase

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM virtual order execution allows DoS via dense time initialization
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through initialized time buckets from the last execution time to the current block timestamp. An attacker can cheaply create many orders with sequentially increasing end times (e.g., every second or minute), densely populating the initialized times bitmap. This forces the `while` loop in `_executeVirtualOrdersFromWithinLock` to perform excessive iterations, potentially exceeding the block gas limit and causing a Denial of Service for the pool.
 ### Static Signals
while loop over time, bitmap traversal, user-controlled end times
 ### Assets at Risk
pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: BasePositions.handleLockData

 ### Title
Unsafe cast to int128 in fee accounting causes DoS for large amounts
 ### Description/Code Snippet
In `handleLockData` (both `CALL_TYPE_WITHDRAW` and the logic handling `CALL_TYPE_DEPOSIT` via `collectFees`), the contract calculates protocol fees as `uint128`. It then casts these fees to `int128` when calling `CORE.updateSavedBalances`. If the fee amount exceeds `type(int128).max` (approx 1.7e38), the cast wraps to a negative number. Passing a negative delta to `updateSavedBalances` attempts to withdraw from the saved balance instead of depositing the fee. Since the contract likely has insufficient saved balance to cover this 'withdrawal', the Core will revert with `SavedBalanceOverflow`, causing a DoS on withdrawals or fee collections for tokens with very high supplies or decimals.
 ### Static Signals
int128(swapProtocolFee0), int128(withdrawalFee0)
 ### Assets at Risk
Fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension locks pools due to beforeSwap revert loop
 ### Description/Code Snippet
The `MEVCapture` extension implements `beforeSwap` to unconditionally revert with `SwapMustHappenThroughForward`, intending to force users to use `forward`. However, when users correctly use `forward`, the `handleForwardData` function calls `CORE.swap`. `CORE.swap` invokes the registered extension's `beforeSwap` hook. Since `MEVCapture` registers for `beforeSwap`, this creates a circular call path that always reverts, effectively permanently freezing any pool using this extension.
 ### Static Signals
revert in beforeSwap, CORE.swap called in handleForwardData, beforeSwap=true in CallPoints
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: BasePositions.handleLockData

 ### Title
Incompatibility with Fee-On-Transfer tokens
 ### Description/Code Snippet
The `deposit` flow in `BasePositions.sol` (via `handleLockData` and `CORE.updatePosition`) relies on `FlashAccountant` to settle debts. When `CORE` updates a position, it records a debt equal to the liquidity delta amounts. `BasePositions` then calls `ACCOUNTANT.payTwoFrom` to transfer exactly these amounts from the user to the accountant. For Fee-On-Transfer (FoT) tokens, the actual balance increase in the Accountant will be less than the transfer amount. The `FlashAccountant` calculates debt reduction based on the actual balance change. Consequently, the debt will not be fully settled (debt > 0), causing the `lock` function to revert with `DebtsNotZeroed`. This makes the standard deposit flow unusable for FoT tokens.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
DoS via Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function (and by extension `lockAndExecuteVirtualOrders`) iterates through time intervals from the last execution time up to `block.timestamp`. The step size is determined by `searchForNextInitializedTime`, which depends on the density of initialized orders in the time bitmap. An attacker can cost-effectively create orders with start/end times spaced by 1 second (or the minimum tick) to densely populate this bitmap. If the pool is left inactive for a period, the number of iterations required to process the backlog can exceed the block gas limit, causing the pool to permanently revert on all swaps and updates (Denial of Service). There is no mechanism to process the backlog in chunks.
 ### Static Signals
loops over user-controlled arrays/sets, iteration count grows with contract state, no pagination or batching mechanism
 ### Assets at Risk
liquidity pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: BasePositions.burn

 ### Title
Burning Position NFT permanently locks underlying liquidity
 ### Description/Code Snippet
`BasePositions` inherits `burn` from `BaseNonfungibleToken`, which allows the owner to destroy the NFT. The contract does not override `burn` to withdraw liquidity from Core. Since `withdraw` and `collectFees` require authorization via `authorizedForNft(id)` (which checks ownership), burning the NFT removes the ability to access or withdraw the underlying liquidity and accrued fees, resulting in permanent stuck funds.
 ### Static Signals
burn function exposed, no balance check before burn, access control depends on token ownership
 ### Assets at Risk
user deposits, accrued fees
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: BasePositions.deposit

 ### Title
Incompatibility with Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `deposit` flow (via `handleLockData` in `BasePositions` and `FlashAccountant`) calculates the debt based on the `liquidity` minted (expecting 1:1 token amount). It then calls `ACCOUNTANT.payTwoFrom`, which transfers `amount` from the user. For Fee-on-Transfer tokens, the Core receives `amount - fee`. The `FlashAccountant` validates that the received balance delta exactly matches the debt. Since the received amount is less than the debt, the transaction reverts with `DebtsNotZeroed`. This makes the protocol effectively incompatible with Fee-on-Transfer tokens for standard deposits.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, assumes 1:1 token transfers
 ### Assets at Risk
protocol integration
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
UnsafeRecipient in withdraw and collectFees
 ### Description/Code Snippet
The `withdraw` and `collectFees` functions accept a `recipient` address argument and pass it directly to the `FlashAccountant` for token transfer without checking if `recipient` is `address(0)`. If a user mistakenly passes the zero address, the `FlashAccountant` will execute a transfer to the zero address. For many ERC20 tokens and specifically for native ETH (handled via `call`), this results in the permanent burning of the withdrawn funds.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
withdrawn funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
Missing slippage protection and deadline in position withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions` burns a liquidity position to retrieve the underlying tokens. The amount of tokens returned depends on the current pool price (tick). The function signature lacks parameters for minimum output amounts (`amount0Min`, `amount1Min`) and a transaction `deadline`. This exposes users to sandwich attacks where an attacker manipulates the pool price prior to withdrawal to extract value, and allows miners to hold transactions until market conditions are unfavorable.
 ### Static Signals
no minAmountOut parameter in liquidation/redemption, deadline omitted
 ### Assets at Risk
User liquidity positions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

