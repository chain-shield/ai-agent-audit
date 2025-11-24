## Verified Patterns Found: 22

## Verified Patterns Found in following Categories:

- SlippageMissingOrInsufficient
- StaleOracleAcceptance
- StorageCollisionOrSelectorClash
- FeeOnTransferAssumption
- PricePrecisionOrRoundingError
- StandardViolation
- ReserveOrPriceDesync
- UnboundedLoops
- FlashLoanEconomicManipulation



## Summary of Patterns

QuoteDataFetcher reverts on high liquidity Stableswap pools due to unsafe int128 cast

MEVCapture extension forces pool-wide fee penalty based on block-start tick

Critical Quote Data Corruption Due to Improper Assembly Sign Extension

Stale Oracle State Acceptance in QuoteFetcher

Router Slippage Check Incorrect for Exact Output Swaps

MEVCapture extension may cause DoS for standard swaps via `CORE.swap`

Flash Loan Manipulation of Revenue Buybacks

Unsafe Downcasting in RevenueBuybacks

Storage Collision in Oracle Extension via Unhashed Slots

TokenWrapper Transient Balance Loss

Missing slippage protection in Positions liquidity withdrawal

Fee-on-Transfer tokens cause protocol insolvency in Orders/RevenueBuybacks

Incentives Contract Insolvent with Fee-on-Transfer Tokens

Missing Slippage Protection in Positions Withdrawal

Fee-On-Transfer Token Incompatibility in RevenueBuybacks

Unbounded loop in tick data fetching facilitates Gas-Based DoS

Fee-on-transfer tokens cause DoS in RevenueBuybacks

Incorrect unpacking of signed integers in assembly causes data corruption

Unbounded loop in TWAMM virtual order execution

Unbounded Loop in TWAMM Virtual Order Execution

Incorrect sign extension in assembly logic corrupts negative ticks and liquidity deltas

QuoteDataFetcher returns incomplete tick data due to incorrect skipAhead calculation

## Patterns



 ### Issue Type: StandardViolation

 ### Relevant Function/Location: QuoteDataFetcher.getQuoteData

 ### Title
QuoteDataFetcher reverts on high liquidity Stableswap pools due to unsafe int128 cast
 ### Description/Code Snippet
In `getQuoteData`, when handling Stableswap pools (non-concentrated), the contract attempts to cast the pool's `uint128 liquidity` to `int128` to populate the `TickDelta` struct: `liquidityDelta: int128(liquidity)`. Since `liquidity` can legally exceed `type(int128).max` (up to `type(uint128).max`), this cast will revert for pools with sufficiently high liquidity. This creates a denial of service for the QuoteDataFetcher lens for such pools, breaking UI and integration components that rely on it.
 ### Static Signals
int128(liquidity), TickDelta
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEVCapture extension forces pool-wide fee penalty based on block-start tick
 ### Description/Code Snippet
The `MEVCapture` extension calculates a variable swap fee based on the deviation of the current tick from `tickLast`. However, `tickLast` is only updated to the current tick during the first transaction of a block (when `lastUpdateTime` differs from `block.timestamp`). For all subsequent transactions in the same block, `tickLast` remains fixed at the block's starting value. This means if an early transaction (or flash loan) moves the tick significantly, all subsequent users in the block are charged fees based on the deviation from the start of the block, rather than the tick prior to their swap. This allows for griefing attacks where an attacker moves the price to force high fees on victims, and penalizes retail users during periods of high volatility.
 ### Static Signals
lastUpdateTime check, tickLast persistence
 ### Assets at Risk
user funds (excessive fees)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: QuoteDataFetcher._getInitializedTicksInRange

 ### Title
Critical Quote Data Corruption Due to Improper Assembly Sign Extension
 ### Description/Code Snippet
In `QuoteDataFetcher._getInitializedTicksInRange`, assembly is used to pack and unpack `int32 tick` and `int128 liquidityDelta`. The unpacking logic uses `shr` (logical right shift) instead of `sar` (arithmetic right shift) for `tickNumber`, and fails to `signextend` the `liquidityDelta`. For negative ticks or liquidity deltas (which are common), this destroys the sign bits, turning small negative numbers (e.g., -1) into massive positive integers. This corrupts the returned `QuoteData`, causing off-chain quotes to calculate wildly incorrect prices and liquidity depths, leading users to submit transactions with invalid slippage bounds or causing DoS of the quoting infrastructure.
 ### Static Signals
shr(128, packed), no signextend, int32 tickNumber, int128 liquidityDelta
 ### Assets at Risk
User funds via bad execution
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: QuoteDataFetcher.getQuoteData

 ### Title
Stale Oracle State Acceptance in QuoteFetcher
 ### Description/Code Snippet
The `QuoteDataFetcher` retrieves `CORE.poolState` directly. For pools utilizing extensions (like TWAMM or Oracle), the Core state is only updated lazily when interactions occur (via hooks). The fetcher does not simulate these hooks or check for staleness. As a result, it returns stale price and tick data for these pools. Users relying on this data for quotes will calculate `minOut` based on outdated state, leading to reverted transactions (if price moved against them) or unintended positive slippage (if price moved in favor), but fundamentally bypassing accurate slippage protection.
 ### Static Signals
CORE.poolState(poolId), no extension validation, no freshness check
 ### Assets at Risk
User funds via slippage
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router._swap

 ### Title
Router Slippage Check Incorrect for Exact Output Swaps
 ### Description/Code Snippet
In `Router.sol`, the `_swap` function calculates `amountCalculated` as the negative of the pool's delta for the unspecified token. For Exact Output swaps (where `amount` is negative), `amountCalculated` represents the *input* amount as a negative value (e.g., -Input). The slippage check `if (amountCalculated < calculatedAmountThreshold) revert` expects the user to provide a negative threshold to represent 'Max Input'. If a user or UI provides a standard positive 'Max Input' (e.g., 100), the check becomes `-Input < 100` which is always true, causing the swap to revert even if the price is good. Users are forced to provide negative thresholds or `type(int256).min`, the latter of which disables slippage protection entirely.
 ### Static Signals
amountCalculated < calculatedAmountThreshold, revert SlippageCheckFailed
 ### Assets at Risk
User funds (slippage)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension may cause DoS for standard swaps via `CORE.swap`
 ### Description/Code Snippet
The `MEVCapture` extension registers for the `beforeSwap` hook but implements it to unconditionally revert with `SwapMustHappenThroughForward()`. When a pool uses this extension, `CORE.swap` will invoke `beforeSwap`. Unless the `ExtensionCallPointsLib` (not fully visible but implied) contains specific logic to skip the hook when the locker is the extension itself, any attempt to use the standard `CORE.swap` entry point for these pools will revert. This forces all interactions to go through the `forward` mechanism, potentially breaking integrations that rely on the standard `CORE.swap` interface.
 ### Static Signals
revert in beforeSwap hook, beforeSwap: true in call points
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Flash Loan Manipulation of Revenue Buybacks
 ### Description/Code Snippet
The `RevenueBuybacks.roll()` function calculates the size of the TWAMM buyback order based on the contract's current spot balance (`balanceOf(token)` or `address(this).balance`). An attacker can manipulate this via a flash loan or direct transfer to the contract immediately before calling `roll()`. This dramatically increases the `saleRate` of the TWAMM order, creating artificial sell pressure on the revenue token and buy pressure on the `BUY_TOKEN`, which can be exploited for market manipulation or griefing.
 ### Static Signals
branches on pool.balanceOf()/getReserves(), atomic swap → read price → execute logic pattern
 ### Assets at Risk
Protocol Revenue, BUY_TOKEN market price
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: RevenueBuybacks.sol.roll

 ### Title
Unsafe Downcasting in RevenueBuybacks
 ### Description/Code Snippet
In `RevenueBuybacks.sol`, the `roll` function calculates `amountToSpend` using the contract's full token balance (`uint256`), but casts it to `uint128` when calling `ORDERS.increaseSellAmount`. If the revenue token has a total supply or balance exceeding `type(uint128).max` (approx 3.4e38, possible with hyper-inflationary tokens), this cast will silently truncate the high-order bits in Solidity 0.8 (via explicit conversion). This results in the buyback order selling a significantly smaller amount than intended, leaving the remaining tokens stuck or requiring multiple calls.
 ### Static Signals
uint128(amountToSpend), unsafe explicit casting
 ### Assets at Risk
revenue tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: Oracle.maybeInsertSnapshot

 ### Title
Storage Collision in Oracle Extension via Unhashed Slots
 ### Description/Code Snippet
The Oracle extension uses manual assembly to determine storage slots for `Counts` and `Snapshots` without hashing the keys. Specifically, it uses `sload(token)` (slot = token address) for `Counts` and `sstore((token << 32) | index)` for `Snapshots`. This creates a collision overlap if `tokenA = (tokenB << 32) + index`, where `tokenA` and `tokenB` are valid token addresses. While exploiting this requires generating specific addresses, it violates secure storage patterns and could allow one token's oracle data to corrupt another's.
 ### Static Signals
manual assembly slots without namespace, sstore(or(shl(32, token), index), snapshot)
 ### Assets at Risk
Oracle integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
TokenWrapper Transient Balance Loss
 ### Description/Code Snippet
The `TokenWrapper` contract implements a `transient` balance for the Core contract to enable efficient internal accounting. However, this transient balance is wiped at the end of every transaction. If a user standard `transfer()`s tokens to the Core address (instead of paying via the Accountant/Router flow), the tokens are added to this transient balance and subsequently burned when the transaction context ends, leading to permanent loss of funds.
 ### Static Signals
preview/view functions modify state, wrong totalSupply/balance invariants
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing slippage protection in Positions liquidity withdrawal
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows users to burn liquidity and receive underlying tokens without specifying minimum output amounts (`amount0Min`, `amount1Min`). Since the ratio of tokens returned depends on the current pool tick, an attacker can manipulate the price (e.g., via a sandwich attack) to force the user to withdraw value in a disadvantageous ratio, causing loss of value.
 ### Static Signals
withdraw function returns amounts but takes no minAmount params, liquidity burn calculated at current tick
 ### Assets at Risk
user liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Orders.sol.handleLockData

 ### Title
Fee-on-Transfer tokens cause protocol insolvency in Orders/RevenueBuybacks
 ### Description/Code Snippet
The `Orders` contract funds TWAMM orders using `ACCOUNTANT.payFrom`, which typically wraps a transfer from the payer. If the `sellToken` (configured in `RevenueBuybacks`) is a Fee-on-Transfer token, the `FlashAccountant` receives less than the credited `amount`. Since the `Orders` logic credits the order with the full `amount` but the physical reserves are lower, this creates a deficit in the Core/Accountant. This can lead to insolvency where subsequent withdrawals fail due to insufficient balance, or the debt check fails if the discrepancy is detected during the lock.
 ### Static Signals
ACCOUNTANT.payFrom, increaseSellAmount, no balance check
 ### Assets at Risk
protocol revenue, liquidity pool assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Incentives Contract Insolvent with Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `Incentives.sol` contract's `fund` function calculates `fundedAmount` based on the `minimum` parameter and then credits the internal `dropState.funded` accounting by that amount after calling `safeTransferFrom`. It does not check the actual balance increase. If a Fee-on-Transfer token is used, the contract receives less than the credited amount. This leads to insolvency where the last claimers will fail to `claim` their tokens because the contract lacks the funds that the accounting says it has.
 ### Static Signals
dropState.setFunded(minimum), safeTransferFrom, no balance check
 ### Assets at Risk
Incentive funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.sol.withdraw

 ### Title
Missing Slippage Protection in Positions Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows users to burn liquidity to receive tokens but does not accept minimum output amount parameters (`minAmount0`, `minAmount1`). The amount of tokens received is calculated based on the pool's current `sqrtRatio` via `liquidityDeltaToAmountDelta`. Since the transaction can be sandwiched or the pool price manipulated before the withdrawal in the same block, users may receive significantly fewer tokens (or a different ratio of tokens) than expected without any reversion capability.
 ### Static Signals
withdraw function lacks minAmount arguments, returns calculated amounts without checking against a threshold
 ### Assets at Risk
user liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: RevenueBuybacks.sol.roll

 ### Title
Fee-On-Transfer Token Incompatibility in RevenueBuybacks
 ### Description/Code Snippet
The `RevenueBuybacks` contract supports arbitrary `buyToken` and revenue tokens, but the integration with `Orders` and `FlashAccountant` fails for fee-on-transfer (FOT) tokens. `roll` attempts to create an order using `balanceOf(address(this))`. `ORDERS.increaseSellAmount` pulls this exact amount to the `FlashAccountant`. If the token applies a fee on transfer, the `FlashAccountant` receives less than the debt amount recorded. Since `FlashAccountant` enforces exact debt settlement (received delta must >= debt), the transaction will revert, making it impossible to process buybacks for FOT revenue tokens.
 ### Static Signals
uses balanceOf for input amount, FlashAccountant enforces exact receipt
 ### Assets at Risk
revenue tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: QuoteDataFetcher._getInitializedTicksInRange

 ### Title
Unbounded loop in tick data fetching facilitates Gas-Based DoS
 ### Description/Code Snippet
The function `_getInitializedTicksInRange` iterates over initialized ticks using `CORE.prevInitializedTick` within a range defined by `minBitmapsSearched`. Since `minBitmapsSearched` is user-controlled and the density of initialized ticks in a pool is unbounded (up to every tick being initialized), a call with a wide range on a dense pool can execute excessive external calls to Core, consuming all available gas and causing the transaction to revert. This effectively denies service to off-chain components relying on `getQuoteData` for pricing.
 ### Static Signals
while (toTick >= fromTick), CORE.prevInitializedTick
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Fee-on-transfer tokens cause DoS in RevenueBuybacks
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function creates orders via `Orders.increaseSellAmount`, which records the full input amount as debt in the `FlashAccountant`. For fee-on-transfer tokens, the actual amount received by the Core is less than the recorded debt (due to the fee). This causes the `DebtsNotZeroed` check in `FlashAccountant` to fail, causing the transaction to revert and preventing buybacks for such tokens.
 ### Static Signals
assumes transfer amount equals received amount, FlashAccountant debt check on balance delta
 ### Assets at Risk
protocol revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: QuoteDataFetcher._getInitializedTicksInRange

 ### Title
Incorrect unpacking of signed integers in assembly causes data corruption
 ### Description/Code Snippet
In `_getInitializedTicksInRange`, the `tickNumber` (int32) and `liquidityDelta` (int128) are packed into a `uint256` and then unpacked using assembly bitwise operations (`shr`, `and`) without sign extension. 

Specifically, `liquidityDelta := and(packed, 0xff...ff)` produces a value with the upper 128 bits zeroed out. For negative liquidity deltas (e.g. -1), this results in a large positive integer (2^128 - 1) instead of the negative value. Similarly, `tickNumber := shr(128, packed)` uses a logical shift, filling the upper bits with zeros, which corrupts negative tick numbers (e.g. -1 becomes a large positive value). 

This causes `getQuoteData` to return wildly incorrect `QuoteData` for any pool with negative ticks or negative liquidity deltas (which is common), leading to incorrect off-chain quotes.
 ### Static Signals
assembly usage, shr on signed type, and masking signed type, missing signextend
 ### Assets at Risk
user funds (via bad quotes)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded loop in TWAMM virtual order execution
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through time intervals from the last execution time to the current block timestamp using `searchForNextInitializedTime`. If an attacker creates many orders with end times spaced closely together (e.g., every second), this loop can exceed the block gas limit when a swap triggers execution, causing a Denial of Service for the pool.
 ### Static Signals
while (time != block.timestamp), state-dependent loop condition, searchForNextInitializedTime
 ### Assets at Risk
pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM.sol._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The function `_executeVirtualOrdersFromWithinLock` in `TWAMM.sol` iterates from `realLastVirtualOrderExecutionTime` to `block.timestamp` using a `while` loop that steps through initialized time intervals. An attacker can create many small TWAMM orders ending at sequential seconds (or minimal time intervals) to densely populate the `poolInitializedTimesBitmapSlot`. If a pool is left untouched for a significant period, the next user interaction (or keeper call) will attempt to iterate through all these intervals, performing gas-intensive `CORE.swap` calls in each step. This can exceed the block gas limit, permanently DoS-ing the pool.
 ### Static Signals
while (time != block.timestamp), loop condition depends on elapsed time, external call (CORE.swap) inside loop
 ### Assets at Risk
pool availability, locked user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: QuoteDataFetcher._getInitializedTicksInRange

 ### Title
Incorrect sign extension in assembly logic corrupts negative ticks and liquidity deltas
 ### Description/Code Snippet
In `QuoteDataFetcher._getInitializedTicksInRange`, the code packs `tick` (int32) and `liquidityDelta` (int128) into a single word and subsequently unpacks them using inline assembly. The unpacking logic uses `shr` (logical shift right) for the tick and `and` for the liquidity delta. These operations preserve the bit patterns but do not perform sign extension to 256 bits. In the EVM, signed integers must be sign-extended (e.g., negative values must have leading 1s up to the 256th bit). By failing to use `signextend`, negative ticks and liquidity deltas result in values with zeroed upper bits, which are interpreted as very large positive integers by consumers. This corrupts the data returned by the fetcher.
 ### Static Signals
shr used on signed integer in assembly, missing signextend opcode, manual packing/unpacking of signed types
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: QuoteDataFetcher._getInitializedTicksInRange

 ### Title
QuoteDataFetcher returns incomplete tick data due to incorrect skipAhead calculation
 ### Description/Code Snippet
In `_getInitializedTicksInRange`, the `skipAhead` parameter passed to `CORE.prevInitializedTick` is calculated as `(toTick - fromTick) / (tickSpacing * 256)`. This value represents the number of bitmap words in the remaining search range. `prevInitializedTick` uses `skipAhead` to skip processing of bitmap words for gas optimization. By passing a large non-zero `skipAhead` derived from the search distance, the function inadvertently instructs Core to skip scanning the very ticks it intends to fetch (those closest to `toTick`). This results in the function returning an incomplete list of initialized ticks, leading to incorrect off-chain quotes and potential slippage or DoS for users relying on this data.
 ### Static Signals
prevInitializedTick, skipAhead calculated from range, uint256(uint32(toTick - fromTick))
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

