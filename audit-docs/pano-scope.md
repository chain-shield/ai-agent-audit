
## Publicly known issues - ALL OUT OF SCOPE

_Anything included in this section and its subsection is considered a publicly known issue and is therefore is OUT OF SCOPE_

**System & Token Limitations**

- Transfers of ERC1155 SFPM tokens has been disabled.
- Construction helper functions (prefixed with add) in the TokenId library and other types do not perform extensive input validation. Passing invalid or nonsensical inputs into these functions or attempting to overwrite already filled slots may yield unexpected or invalid results. This is by design, so it is expected that users of these functions will validate the inputs beforehand.
- Tokens with a supply exceeding 2^127 - 1 are not supported.
- If one token on a pool is broken/does not meet listed criteria/is malicious there are no guarantees as to the security of the other token in that pool, as long as other pools with two legitimate and compliant tokens are not affected.

**Oracle & Price Manipulation**

- Price/oracle manipulation that is not atomic or requires attackers to hold a price across more than one block is not in scope -i.e., to manipulate the internal exponential moving averages (EMAs), you need to set the manipulated price and then keep it there for at least 1 minute until it can be updated again.
- Attacks that stem from the EMA oracles being extremely stale compared to the market price within its period (currently between 2-30 minutes)
- As a general rule, only price manipulation issues that can be triggered by manipulating the price atomically from a normal pool/oracle state are valid

**Protocol Parameters**

- The constants VEGOID, EMA_PERIODS, MAX_TICKS_DELTA, MAX_TWAP_DELTA_LIQUIDATION, MAX_SPREAD, BP_DECREASE_BUFFER, MAX_CLAMP_DELTA, NOTIONAL_FEE, PREMIUM_FEE, PROTOCOL_SPLIT, BUILDER_SPLIT, SELLER_COLLATERAL_RATIO, BUYER_COLLATERAL_RATIO, MAINT_MARGIN_RATE, FORCE_EXERCISE_COST, TARGET_POOL_UTIL, SATURATED_POOL_UTIL, MAX_OPEN_LEGS, and the IRM parameters (CURVE_STEEPNESS, TARGET_UTILIZATION, etc.) are all parameters and subject to change, but within reasonable levels.

**Premium & Liquidation Issues**

- Given a small enough pool and low seller diversity, premium manipulation by swapping back and forth in Uniswap is a known risk. As long as it's not possible to do it between two of your own accounts profitably and doesn't cause protocol loss, that's acceptable
- It's known that liquidators sometimes have a limited capacity to force liquidations to execute at a less favorable price and extract some additional profit from that. This is acceptable even if it causes some amount of unnecessary protocol loss.
- It's possible to leverage the rounding direction to artificially inflate the total gross premium and significantly decrease the rate of premium option sellers earn/are able to withdraw (but not the premium buyers pay) in the future (only significant for very-low-decimal pools, since this must be done one token at a time).
- It's also possible for options buyers to avoid paying premium by calling settleLongPremium if the amount of premium owed is sufficiently small.
- Premium accumulation can become permanently capped if the accumulator exceeds the maximum value; this can happen if a low amount of liquidity earns a large amount of (token) fees

**Gas & Execution Limitations**

- The liquidator may not be able to execute a liquidation if MAX_POSITIONS is too high for the deployed chain due to an insufficient gas limit. This parameter is not final and will be adjusted by deployed chain such that the most expensive liquidation is well within a safe margin of the gas limit.
- It's expected that liquidators may have to sell options, perform force exercises, and deposit collateral to perform some liquidations. In some situations, the liquidation may not be profitable.
- In some situations (stale oracle tick), force exercised users will be worse off than if they had burnt their position.

**Share Supply Issues**

- It is feasible for the share supply of the CollateralTracker to approach 2**256 - 1 (given the token supply constraints, this can happen through repeated protocol-loss-causing liquidations), which can cause various reverts and overflows. Generally, issues with an extremely high share supply as a precondition (delegation reverts due to user's balance being too high, other DoS caused by overflows in calculations with share supply or balances, etc.) are not valid unless that share supply can be created through means other than repeated liquidations/high protocol loss.

**Constructor Assumptions**

- For the purposes of this competition, assume the constructor arguments to the RiskEngine are: 10_000_000, 10_000_000, address(0), address(0)

**Out of Scope**

- Front-running via insufficient slippage specification is not in scope

### Additional Findings from Nethermind pre-contest - ALL OUT OF SCOPE

1. **Double Penalty / Index Update**

   Users pay the interest penalty even when using phantom shares for interest payment. In force exercise scenarios, the user pays in `delegate(...)` and their borrow index is also updated in `_accrueInterest`. In the regular penalty case, the index is not updated.

2. **Masking Insolvency Magnitude**

   While the else cases in `_getMargin(...)` correctly resolve the staleness issue, the if statement (where interest owed > balance) masks the true deficit magnitude. Since `_getMargin(...)` is used in `isAccountSolvent(...)`, setting interest (requirement) to the balance value hides the actual funds shortage.
   
   **Example:** Alice owes 100 interest with a balance of 20. Setting interest to 20 and balance to 0 shows a deficit of 20 instead of the actual deficit of 80.

3. **Broken Bonus Calculations**

   The if statement logic in `_getMargin(...)` breaks bonus calculations by hiding the true deficit. The values of bonus cross and threshold cross are calculated based on the masked deficit rather than the actual shortage.

4. **Orphan Shares in Delegate/Revoke**

   The `delegate(...)` to `revoke(...)` interaction creates shares not owned by anyone, breaking the supply invariant. In force exercise scenarios:

   1. User starts with balance X.
   2. **Delegate:** User balance inflates to inflation + X, then decrements by X due to insufficient interest payment. Balance = inflation.
   3. **Settle Burn / Accrue Interest:** Y shares are burned to cover interest (sufficient phantom balance). Total supply decreases by Y. User balance = inflation - Y.
   4. **Revoke:** Since inflation > balance, user balance is zeroed and total supply is restored by adding Y (inflation - (inflation - Y)).
   5. **Result:** Net change in total supply is 0 (-Y burn +Y restore). The original X shares remain in total supply but are owned by no one.


### Files in scope

**Note:** The nSLoC counts in the following table have been automatically generated and may differ depending on the definition of what a "significant" line of code represents. As such, they should be considered indicative rather than absolute representations of the lines involved in each contract.

| File | nSLOC |
| ---- | ----- |
| [/contracts/PanopticPool.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/PanopticPool.sol) | 1183 |
| [/contracts/RiskEngine.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/RiskEngine.sol) | 1294 |
| [/contracts/types/OraclePack.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/types/OraclePack.sol) | 291 |
| [/contracts/types/MarketState.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/types/MarketState.sol) | 65 |
| [/contracts/types/PoolData.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/types/PoolData.sol) | 42 |
| [/contracts/types/RiskParameters.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/types/RiskParameters.sol) | 72 |
| [/contracts/SemiFungiblePositionManager.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/SemiFungiblePositionManager.sol) | 673 |
| [/contracts/SemiFungiblePositionManagerV4.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/SemiFungiblePositionManagerV4.sol) | 631 |
| [/contracts/CollateralTracker.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/CollateralTracker.sol) | 863 |
| [/contracts/libraries/PanopticMath.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/libraries/PanopticMath.sol) | 369 |
| [/contracts/libraries/Math.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/libraries/Math.sol) | 641 |
| [/contracts/types/TokenId.sol](https://github.com/code-423n4/2025-12-panoptic/blob/main/contracts/types/TokenId.sol) | 232 |


# Additional context - WHERE TO FOCUS

## Areas of concern (where to focus for bugs)

The factory contract and usage of libraries by external integrators is relatively unimportant -- wardens should focus their efforts on the security of the SFPM, PanopticPool, RiskEngine, and CollateralTracker

# Main invariants

## SFPM (SemiFungiblePositionManager)

- Fees collected from Uniswap during any given operation should not exceed the amount of fees earned by the liquidity owned by the user performing the operation.
- Fees paid to a given user should not exceed the amount of fees earned by the liquidity owned by that user.

## CollateralTracker

### Asset Accounting

- `totalAssets()` must equal `s_depositedAssets + s_assetsInAMM + unrealizedGlobalInterest` at all times
- `totalSupply()` must equal `_internalSupply + s_creditedShares` at all times
- `s_depositedAssets` should never underflow below 1 (the initial virtual asset)
- The share price (`totalAssets() / totalSupply()`) must be non-decreasing over time (except for rounding in favor of the protocol and during liquidations with protocol loss)

### Interest Accrual

- The global `borrowIndex` must be monotonically increasing over time and start at 1e18 (WAD)
- For any user with `netBorrows > 0`, their `userBorrowIndex` must be ≤ the current global `borrowIndex`
- Interest owed by a user must equal: `netBorrows * (currentBorrowIndex - userBorrowIndex) / userBorrowIndex`
- `unrealizedGlobalInterest` must never exceed the sum of all individual users' interest owed
- After `_accrueInterest()`, the user's `userBorrowIndex` must equal the current global `borrowIndex` (unless insolvent and unable to pay)

### Credited Shares

- `s_creditedShares` represents shares for long positions and can only increase when positions are created and decrease when closed
- When a long position is closed, if `creditDelta > s_creditedShares`, the difference must be paid by the option owner as a rounding haircut
- The rounding haircut from Uniswap position management should not exceed a few wei per position

### Deposits and Withdrawals

- Users with open positions (`numberOfLegs > 0`) cannot transfer shares via `transfer()` or `transferFrom()`
- Users with open positions can only withdraw if they provide `positionIdList` and remain solvent after withdrawal
- Deposits must not exceed `type(uint104).max` (2^104 - 1)
- Withdrawals must leave at least 1 asset in `s_depositedAssets` (cannot fully drain the pool)

## RiskEngine

### Solvency Checks

- An account is solvent if and only if: `balance0 + convert(scaledSurplus1) >= maintReq0` AND `balance1 + convert(scaledSurplus0) >= maintReq1` (where conversion direction depends on price)
- The maintenance requirement includes: position collateral requirements + accrued interest owed + long premia owed - short premia owed - credit amounts
- Cross-collateralization uses a `crossBufferRatio` that scales based on pool utilization (higher utilization = more conservative buffer)
- Solvency must be checked at the oracle tick, not the current tick, to prevent manipulation

### Collateral Requirements

- Collateral requirements must scale with pool utilization (higher utilization = higher requirements)
- The "global utilization" used for margin calculations is the maximum utilization across all of a user's positions at the time they were minted

### Liquidation Bonuses

- Liquidation bonus = `min(collateralBalance / 2, required - collateralBalance)`
- The bonus is computed cross-collaterally using both tokens converted to the same denomination
- If the liquidatee has insufficient balance in one token, excess balance in the other token can be used to mitigate protocol loss through token conversion
- If premium was paid to sellers during liquidation, it must be clawed back (haircut) if it would cause protocol loss to exceed the remaining collateral

### Premium Haircutting

- If `collateralRemaining < 0` (protocol loss exists), premium paid to sellers during the liquidation must be proportionally clawed back
- The haircut is applied per-leg based on the ratio of protocol loss to premium paid
- After haircutting, the adjusted bonus must not result in protocol loss exceeding the initial collateral balance

### Force Exercise Costs

- Base force exercise cost = 1.024% (`FORCE_EXERCISE_COST = 102_400 / 10_000_000`) of notional for in-range positions
- Cost for out-of-range positions = 1 bps (`ONE_BPS = 1000 / 10_000_000`)
- The cost must account for token deltas between current and oracle prices for all long legs
- Only long legs (not short legs) contribute to force exercise costs

### Interest Rate Model

- Interest rate must be bounded: `MIN_RATE_AT_TARGET ≤ rate ≤ MAX_RATE_AT_TARGET`
- Rate adjusts continuously based on utilization error: `targetUtilization - currentUtilization`
- The adaptive rate (`rateAtTarget`) compounds at speed `ADJUSTMENT_SPEED * error` per second
- Initial `rateAtTarget` = 4% APR (`INITIAL_RATE_AT_TARGET`)
- Target utilization = 66.67% (`TARGET_UTILIZATION = 2/3 in WAD`)

### Oracle Safety

- The maximum allowed delta between fast and slow oracle ticks = 953 ticks (~10% price move)
- During liquidation, current tick must be within 513 ticks of the TWAP (~5% deviation)
- If oracle deltas exceed thresholds, the protocol enters safe mode using more conservative price estimates
- The median tick buffer can only be updated if sufficient time has elapsed since the last observation

## PanopticPool

### Entry Points

- All user actions must originate from `dispatch()` or `dispatchFrom()` entry points
- `dispatchFrom()` requires the caller to have operator approval from the account owner
- Each dispatch call can execute exactly multiple action (mint, burn, mint, mint, burn  etc.) and allows transiently insolvent states

### Position Management

- Users can mint up to `MAX_OPEN_LEGS = 33` position legs total across all their positions
- Commission is split between protocol and builder (if builder code present) according to `PROTOCOL_SPLIT` and `BUILDER_SPLIT`
- Option sellers must pay back exact `shortAmounts` of tokens when positions are burned
- Option buyers must add back exact liquidity amounts that were removed when positions are burned

### Solvency Requirements

- Users must remain solvent (per RiskEngine) after any mint, burn, or withdrawal operation
- Solvency is checked at the fast oracle tick, or at several oracle ticks if they diverge beyond the threshold

### Premium Settlement

- Option sellers should not receive unsettled premium (premium not yet collected from Uniswap or paid by long holders)
- Sellers' share of settled premium must be proportional to their share of liquidity in the chunk
- Premium distribution ratio: `min(settled / owed, 1)` applies to all sellers in a chunk
- Long premium can be settled against solvent buyers to force payment

### Liquidations

- Liquidations can only occur when `RiskEngine.isAccountSolvent()` returns false at the oracle tick
- The liquidator must close all positions held by the liquidatee
- Liquidation bonus paid to liquidator must not exceed the liquidatee's pre-liquidation collateral balance
- If the liquidation results in protocol loss, shares are minted to the liquidator to cover the difference
- It is acceptable for protocol loss to occur even if the liquidatee has residual token balance, if that balance is insufficient when converted

### Premium Haircutting Invariant

- If premium is paid to sellers during a liquidation AND protocol loss exists after the liquidation, the premium must be haircut (clawed back) to reduce protocol loss
- After haircutting, protocol loss must be minimized but may still be positive if premium clawback is insufficient

### Position Size Limits

- Individual position sizes are limited by the available liquidity in the Uniswap pool
- The maximum spread ratio (removed/net liquidity) is capped at `MAX_SPREAD = 90_000 / 100_000 = 90%`
- Position sizes must not cause integer overflows in any token amount or liquidity calculations

## Cross-Contract Invariants

### Oracle Consistency

- The oracle tick used for solvency checks must come from PanopticPool's oracle management
- All operations in a single transaction must use consistent oracle tick(s)
- The oracle must account for volatility via EMA and median filters to prevent manipulation

## All trusted roles in the protocol

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Guardian                          | Can only interact with the protocol through the **RiskEngine.sol** smart contract to: <br>- Lock/unlock the pool (sets safeMode to 3)<br>- Collect the outstanding protocol fees in the RiskEngine<br>- The owner of BuilderFactory and can deploy new builderWallets                  |
