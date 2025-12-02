## Verified Patterns Found: 21

## Verified Patterns Found in following Categories:

- UnboundedLoops
- StandardViolation
- TWAPWindowPinningOrLowLiquidity
- SlippageMissingOrInsufficient
- BeaconOrFactoryAuthorityDrift
- MulticallCrossPathReentrancy
- AccountingInvariantViolation
- PricePrecisionOrRoundingError
- FeeOnTransferAssumption
- BlockhashOrPRNGWeakness



## Summary of Patterns

Weak PRNG in mint() using gas() and prevrandao() causes Token ID collisions

Orders Contract Relies on Missing `CORE.updateSaleRate` Function

TWAMM Recursive Infinite Loop Denial of Service

MEVCapture extension causes permanent DoS on swaps due to unconditional revert in beforeSwap hook

Mutable Extensions Can Drain User Funds via Saved Balances

Unsafe downcast in TWAMM virtual order execution allows logic inversion

Missing Limit Price for TWAMM Orders

Oracle extrapolateSnapshot allows history rewriting via spot price manipulation

TWAMM Proceeds Trapped due to Locker Mismatch

Positions.withdraw lacks slippage protection parameters

Unsafe minting allows NFTs to be stuck in non-receiver contracts

SlippageMissingOrInsufficient in Orders.collectProceeds

TWAMM execution loop is unbounded allowing gas-based DoS

PricePrecisionOrRoundingError in TWAMM Order Creation

Potential Data Truncation in `PoolState` Packing

MEVCapture Extension Denial of Service via Recursive Hook Revert

tokenURI violation of EIP-721 standard regarding non-existent tokens

SlippageMissingOrInsufficient in Positions.withdraw

FeeOnTransferAssumption in Positions and Orders

SlippageMissingOrInsufficient in Positions.withdraw

DoS of TWAMM Pools via Unbounded Loop over Time Checkpoints

## Patterns



 ### Issue Type: BlockhashOrPRNGWeakness

 ### Relevant Function/Location: BaseNonfungibleToken.mint

 ### Title
Weak PRNG in mint() using gas() and prevrandao() causes Token ID collisions
 ### Description/Code Snippet
The `mint()` function generates a token ID salt using `keccak256(0, 64)` where memory contains `prevrandao()` and `gas()`. `prevrandao()` is constant within a block, and `gas()` (remaining gas) is predictable and manipulable by the sender. If a user sends multiple mint transactions in the same block with identical gas parameters (or if an attacker manipulates gas limits), the generated salt will be identical. This results in the same Token ID being generated. Since `_mint` reverts on duplicate IDs, this leads to transaction failures (DoS) and wasted gas for users attempting to mint multiple tokens rapidly.
 ### Static Signals
prevrandao() used for randomness, gas() used for randomness, keccak256 of block difficulty and gas
 ### Assets at Risk
User Gas
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Orders.handleLockData

 ### Title
Orders Contract Relies on Missing `CORE.updateSaleRate` Function
 ### Description/Code Snippet
The `Orders` contract attempts to call `CORE.updateSaleRate` in `handleLockData` (lines 120, 128, etc.). However, the provided `Core.sol` contract does not implement `updateSaleRate`, nor is it part of the `ICore` interface definitions visible in standard paths (it handles `swap`, `updatePosition`, etc.). Unless `Core` implements a fallback or the provided code snippet is incomplete, this call will revert, bricking the `Orders` functionality for increasing/decreasing sale rates.
 ### Static Signals
call to non-existent function, CORE.updateSaleRate
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MulticallCrossPathReentrancy

 ### Relevant Function/Location: TWAMM.beforeSwap

 ### Title
TWAMM Recursive Infinite Loop Denial of Service
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders by calling `CORE.swap`. `CORE.swap` unconditionally calls the `beforeSwap` hook of the registered extension. Since the TWAMM pool registers itself as the extension, `TWAMM.beforeSwap` is triggered again. The `TWAMM` logic only updates `realLastVirtualOrderExecutionTime` in storage *after* the swap completes. Consequently, the re-entrant call to `_executeVirtualOrdersFromWithinLock` sees the old timestamp, determining that orders need execution, and calls `CORE.swap` again. This creates an infinite recursion loop that causes an Out of Gas error, bricking swap functionality for any pool using the TWAMM extension.
 ### Static Signals
public multicall executes arbitrary function list, state flags/locals reused across calls in same tx, assumes call ordering; no global reentrancy sentinel
 ### Assets at Risk
pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension causes permanent DoS on swaps due to unconditional revert in beforeSwap hook
 ### Description/Code Snippet
The `MEVCapture` extension registers the `beforeSwap` hook (`beforeSwap: true`) but implements the function to unconditionally `revert SwapMustHappenThroughForward()`. Even when a user correctly routes through `MEVCaptureRouter` (which calls `MEVCapture.handleForwardData` -> `Core.swap`), `Core.swap` invokes the registered `beforeSwap` hook on the extension. Since the hook always reverts, no swaps can ever complete for pools using this extension.
 ### Static Signals
revert SwapMustHappenThroughForward() in beforeSwap, beforeSwap: true in CallPoints
 ### Assets at Risk
User funds (locked in broken pool)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: Core.updateSavedBalances

 ### Title
Mutable Extensions Can Drain User Funds via Saved Balances
 ### Description/Code Snippet
The `Core` contract allows extensions to define hooks (e.g., `beforeSwap`) that execute while the User or Router is the active Locker. `Core.updateSavedBalances` allows modifying the *current* Locker's balance. If a pool uses an extension that is upgradable (e.g., a proxy) or malicious, the extension can implement a hook that calls `updateSavedBalances` with a negative delta, transferring or burning the User's funds without authorization. This `BeaconOrFactoryAuthorityDrift` allows a compromised extension authority to hijack user funds during interaction.
 ### Static Signals
updateSavedBalances uses _requireLocker(), Extensions invoked during active swap lock, No immutability check on extensions
 ### Assets at Risk
User deposits, Swap inputs, Router balances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unsafe downcast in TWAMM virtual order execution allows logic inversion
 ### Description/Code Snippet
In `TWAMM.sol`, the `_executeVirtualOrdersFromWithinLock` function calculates the amount to swap as `saleRate * timeElapsed`. Since `saleRate` is `uint112` and `timeElapsed` is `uint32`, the product can reach ~2^144, exceeding the `int128` maximum (~2^127). The code performs an unsafe cast `int128(uint128(amount))`, which wraps large positive values to negative numbers. `Core.swap` interprets a negative amount as an `exactOutput` swap (buying) rather than `exactInput` (selling). This inverts the intended trade direction, causing the virtual order to consume liquidity it should be providing.
 ### Static Signals
int128(uint128(amount)), CORE.swap call with calculated amount
 ### Assets at Risk
Pool liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Orders.mintAndIncreaseSellAmount

 ### Title
Missing Limit Price for TWAMM Orders
 ### Description/Code Snippet
The `Orders` contract allows users to create Time-Weighted Average Market Maker (TWAMM) orders via `mintAndIncreaseSellAmount`. While users can specify a `maxSaleRate` to limit the speed of selling, there is no parameter to set a minimum output price (limit price) or minimum total output amount. If the pool price crashes or is manipulated to a very low value during the order's duration, the TWAMM mechanism will continue to sell the user's assets at the unfavorable price, leading to significant value loss.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound, price fetched at execution without user-specified floor
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.extrapolateSnapshot

 ### Title
Oracle extrapolateSnapshot allows history rewriting via spot price manipulation
 ### Description/Code Snippet
The `extrapolateSnapshot` function in `Oracle.sol` calculates TWAP cumulatives by taking the last snapshot and adding the contribution of the time elapsed since then using the *current* spot price (`CORE.poolState(poolId).tick()`). If the last snapshot is old (due to low pool activity), an attacker can manipulate the current spot price within a transaction or block and query the oracle. The oracle will incorrectly apply this manipulated price to the entire duration since the last snapshot, enabling the attacker to drastically skew the reported TWAP.
 ### Static Signals
price fetched at execution without user-specified floor, twapWindow < 10–30 minutes (or unbounded long window)
 ### Assets at Risk
protocols relying on oracle
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TWAMM.handleForwardData

 ### Title
TWAMM Proceeds Trapped due to Locker Mismatch
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders by calling `CORE.lock` on itself, making the extension the active Locker. Consequently, swap proceeds from virtual orders are credited to `TWAMM`'s `savedBalances` in Core. However, when a user calls `Orders.collectProceeds`, the `Orders` contract becomes the active Locker. `TWAMM`'s `handleForwardData` then attempts to withdraw proceeds by calling `CORE.updateSavedBalances` with a negative delta against the `Orders` locker. Since `Core` strictly isolates balances by Locker and `Orders` has no balance (proceeds are in `TWAMM`), this operation underflows/reverts, permanently trapping user funds in the extension.
 ### Static Signals
CORE.updateSavedBalances called with negative delta, Virtual orders executed under Extension locker, Withdrawal attempted under User/Orders locker
 ### Assets at Risk
User swap proceeds, Accrued rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.sol.withdraw

 ### Title
Positions.withdraw lacks slippage protection parameters
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` (inherited from `BasePositions`) allows users to remove liquidity from a position. It accepts the amount of `liquidity` to remove but does not accept `minAmount0` or `minAmount1` arguments to bound the output token amounts. The function calculates the resulting amounts based on the pool's current tick/price. If the pool price changes significantly (due to volatility or manipulation) between the transaction submission and execution, the user may suffer execution at an unfavorable price, receiving a different ratio of assets than expected without the transaction reverting. This exposes users to sandwich attacks and impermanent loss realization.
 ### Static Signals
no minAmount parameters, amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: BaseNonfungibleToken.mint

 ### Title
Unsafe minting allows NFTs to be stuck in non-receiver contracts
 ### Description/Code Snippet
The `mint` functions call Solady's `_mint` internally, which does not invoke the `onERC721Received` hook on the recipient. If a smart contract that does not support ERC721 handling calls `mint()` (e.g., a generic router or multisig without hooks), the resulting NFT will be permanently locked in that contract. EIP-721 convention favors `safeMint` to prevent such asset loss.
 ### Static Signals
_mint called instead of _safeMint, no onERC721Received check
 ### Assets at Risk
User NFTs
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Orders.collectProceeds

 ### Title
SlippageMissingOrInsufficient in Orders.collectProceeds
 ### Description/Code Snippet
The `collectProceeds` function in `Orders.sol` collects the results of a TWAMM order. Crucially, calling this function can trigger the execution of pending virtual orders via the `TWAMM` extension (e.g., if the pool hasn't been touched in a while). The virtual orders execute swaps against the pool's *current* spot liquidity and price. 

If the pool price is manipulated immediately before this call (e.g., in a sandwich attack), the pending virtual orders will execute at a highly unfavorable price. The user then collects the degraded proceeds. The function lacks a `minProceeds` parameter to allow users to protect against poor execution of the pending batch.
 ### Static Signals
payout calculated at execution time without minimum bound, price fetched at execution without user-specified floor
 ### Assets at Risk
User TWAMM order proceeds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM.sol._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM execution loop is unbounded allowing gas-based DoS
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through time intervals from the `lastVirtualOrderExecutionTime` to `block.timestamp` using a `while` loop. The loop advances time using `searchForNextInitializedTime`, which finds the next interval with active order transitions. An attacker can create a sequence of orders that initialize many consecutive time intervals (e.g., every valid interval for a long duration). If the pool remains inactive for a significant period, the next user interaction (swap or position update) triggers the loop to process all pending intervals. If the accumulated gas cost of processing these intervals exceeds the block gas limit, the pool becomes permanently unusable (bricked).
 ### Static Signals
loops over user-controlled arrays/sets, iteration count grows with contract state, no gas limit checks in loop body
 ### Assets at Risk
pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: Orders.increaseSellAmount

 ### Title
PricePrecisionOrRoundingError in TWAMM Order Creation
 ### Description/Code Snippet
In `Orders.sol`, `increaseSellAmount` calculates the `saleRate` using integer division: `saleRate = amount / duration`. 

If `amount` is small relative to `duration` (e.g., low-decimal tokens like USDC sold over a long period), the `saleRate` is truncated. The remainder `amount % duration` is effectively lost because the order only sells `saleRate * duration` tokens over its lifetime. 

The `Orders` contract locks the full `amount` from the user, but the `TWAMM` extension only accounts for the truncated sales. The `collectProceeds` function relies on `rewardRate` logic and does not appear to refund the initial rounding error (the unsold dust) to the user, leading to a permanent loss of funds for the user.
 ### Static Signals
divide before multiply, precision loss
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Core.writePoolState

 ### Title
Potential Data Truncation in `PoolState` Packing
 ### Description/Code Snippet
In `Core.sol`, `PoolState` is wrapped/unwrapped from a `bytes32` storage slot (e.g., `PoolState.wrap(CoreStorageLayout.poolStateSlot(poolId).load())`). The `PoolState` struct ostensibly contains `SqrtRatio` (typically 160 bits), `Tick` (24 bits), and `Liquidity` (128 bits). The sum (312 bits) exceeds the 256-bit capacity of a single slot. Unless `SqrtRatio` or `Liquidity` use reduced precision types (e.g., `uint96` or `uint64`) not visible in the provided snippets, writing `PoolState` to a single slot will result in data truncation and severe accounting invariant violations.
 ### Static Signals
struct > 256 bits packed in bytes32, load() reads single slot
 ### Assets at Risk
All pool assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEVCapture Extension Denial of Service via Recursive Hook Revert
 ### Description/Code Snippet
The `MEVCapture` extension enforces that swaps must occur via the forwarder by reverting in its `beforeSwap` hook with `SwapMustHappenThroughForward()`. However, the `handleForwardData` function, which implements the forwarder logic, calls `CORE.swap`. `CORE.swap` unconditionally triggers the `beforeSwap` hook of the registered extension (which is `MEVCapture`). This creates a recursive loop where the valid path (`handleForwardData`) calls `CORE.swap`, which calls `beforeSwap`, which reverts. Consequently, any swap on a pool using the `MEVCapture` extension will always revert, rendering the extension unusable.
 ### Static Signals
revert in beforeSwap, CORE.swap calls beforeSwap, handleForwardData calls CORE.swap
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: BaseNonfungibleToken.tokenURI

 ### Title
tokenURI violation of EIP-721 standard regarding non-existent tokens
 ### Description/Code Snippet
The `tokenURI` function in `BaseNonfungibleToken` overrides the standard implementation but fails to check if the token `id` exists before returning the URI string. EIP-721 explicitly states that `tokenURI` must throw if `_tokenId` is not a valid NFT. This spec deviation can mislead off-chain indexers, marketplaces, or integrations into displaying metadata for unminted or burned tokens.
 ### Static Signals
override tokenURI, missing _exists(id) check, returns string for any input
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
SlippageMissingOrInsufficient in Positions.withdraw
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` allows users to remove liquidity from a position. It calculates the amounts of `token0` and `token1` to return based on the pool's current `sqrtRatio` (price) and the position's tick range. However, the function does not accept any `minAmount0` or `minAmount1` parameters to enforce minimum return values. 

If the pool price is manipulated (e.g. via a sandwich attack) or moves significantly before the transaction executes, the ratio of assets returned to the user can shift drastically (e.g., receiving 100% of one token and 0% of the other, when a mix was expected). This exposes users to loss of value or undesirable asset composition without a mechanism to revert the transaction.
 ### Static Signals
withdraw() converts shares to assets at current rate without minimum, no minAmount parameters
 ### Assets at Risk
User liquidity (token0/token1)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Positions.deposit

 ### Title
FeeOnTransferAssumption in Positions and Orders
 ### Description/Code Snippet
The `Positions` and `Orders` contracts calculate token amounts internally (e.g., `liquidityDeltaToAmountDelta` or `amount` from input) and assume that transferring this amount from the user via `FlashAccountant` (`payFrom` / `payTwoFrom`) results in the protocol receiving exactly that amount. 

If a Fee-On-Transfer (FOT) token is used, the `FlashAccountant` will receive less than the tracked debt. If the Accountant strictly enforces debt solvency (balance change == debt change), interactions will revert, making the pool unusable for FOT tokens. If checks are loose, the protocol `Core` will credit the user with more tokens than it actually holds, leading to insolvency.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk
liquidity pool solvency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
SlippageMissingOrInsufficient in Positions.withdraw
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` (and `BasePositions.sol`) executes a liquidity withdrawal and optionally collects fees without allowing the caller to specify minimum return amounts (`amount0Min`, `amount1Min`). 

`Positions.withdraw` calls `CORE.updatePosition`, which calculates the delta amounts based on the current pool price (sqrtRatio). If the pool price is manipulated (e.g., via MEV) or volatile in the same block, the user may receive an unfavorable ratio of tokens (high impermanent loss realized) without a revert mechanism. 

Code location: `BasePositions.sol` function `withdraw`.
 ### Static Signals
no minAmountOut parameter in liquidation/redemption, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
DoS of TWAMM Pools via Unbounded Loop over Time Checkpoints
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders by iterating strictly chronologically through initialized time checkpoints in `_executeVirtualOrdersFromWithinLock`. The loop continues `while (time != block.timestamp)`, calling `searchForNextInitializedTime` and processing state updates for each checkpoint. An attacker can mint many orders with sequential/granular end-times (e.g., every minute for a week), densely populating the `poolInitializedTimesBitmap`. When time advances, the next interaction with the pool (swap or position update) triggers this loop. If the number of accumulated checkpoints is sufficiently large, the gas cost of the loop will exceed the block gas limit, causing the transaction to revert. Since the processing is mandatory and sequential, the pool becomes permanently frozen (DoS), as no user can process the pending virtual orders within a single block.
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime, iteration count grows with contract state
 ### Assets at Risk
Pool liquidity (frozen), User funds in orders (stuck)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

