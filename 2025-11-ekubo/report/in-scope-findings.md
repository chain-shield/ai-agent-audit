# In-Scope Findings

- H-22 (TWAMM infinite recursion) - ✅ **FULLY VERIFIED** 🎯 (H-23 is duplicate)
- H-28 (TWAMM DoS via dense time) - ✅ **FULLY VERIFIED** 🎯 (H-29 is duplicate)

This file contains findings that are IN SCOPE and NOT already covered in V12 findings.

---

## TRIAGE SUMMARY

**Total Findings in Report:** 33 findings

**Duplicates Identified:**
- M-1 and M-7 (Revenue Buybacks slippage) - DUPLICATE
- M-2, M-3, M-8, M-10 (Missing deadline checks) - DUPLICATE
- M-4, M-5, M-6, M-11 (Missing slippage in withdrawal) - DUPLICATE
- M-14 and M-15 (Unsafe cast to int128) - DUPLICATE
- M-17 and M-20 (Burning NFT locks liquidity) - DUPLICATE
- H-22 and H-23 (TWAMM infinite recursion) - DUPLICATE
- H-28 and H-29 (TWAMM DoS via dense time) - DUPLICATE

**Out of Scope (Per ekubo-scope.md):**
- M-1, M-2, M-3, M-4, M-5, M-6, M-7, M-8, M-10, M-11 (Missing slippage/deadline) - Design choice per scope lines 23-33
- H-9 (TWAMM slippage) - Design choice per scope lines 23-33
- H-12 (MEVCapture slippage bypass) - Design choice per scope lines 23-33
- H-13 (TWAMM one-way slippage) - Design choice per scope lines 23-33
- H-18, H-30 (MEVCapture freezing pools) - Acceptable per scope lines 17-21
- M-24 (Deposit slippage) - Design choice
- M-25 (MEV fee avoidance) - Design choice
- M-31 (MEV fee bypass via splitting) - Design choice
- L-27 (Unsafe recipient) - Duplicate of L-21

**In Scope - Valid Findings:**
- L-14 (Unsafe cast to int128) - VALID (M-15 is duplicate)
- L-16 (MEVCapture dust accumulation) - VALID
- L-21 (Missing zero-address check) - VALID
- H-22 (TWAMM infinite recursion) - ✅ **FULLY VERIFIED** 🎯 (H-23 is duplicate)
- H-28 (TWAMM DoS via dense time) - ✅ **FULLY VERIFIED** 🎯 (H-29 is duplicate)
- L-32 (MEVCapture overflow DoS) - VALID (downgraded from Medium)

**Already in V12:** Verified - None of the valid findings share root causes with V12 issues (detailed analysis below)

---

## ROOT CAUSE COMPARISON WITH V12

### V12 Finding 1: Self-Transfer Balance Invariant Violation (TokenWrapper.transfer)
**Root Cause:** Missing check for `to == msg.sender`
**Our Findings:** ✅ None share this root cause. Our L-21 checks for `address(0)`, not self-transfer.

### V12 Finding 2: Uncollected Fee Loss on Full Liquidity Removal (Core.updatePosition)
**Root Cause:** Fee checkpoint reset to (0,0) without computing/transferring fees when liquidityNext == 0
**Our Findings:** ✅ None share this root cause. Our M-14 is about unsafe casting, not fee loss on removal.

### V12 Finding 3: Unrestricted extraData Writing for Non-Existent Positions (Core.setExtraData)
**Root Cause:** Missing existence/liquidity check before writing extraData
**Our Findings:** ✅ None share this root cause. None of our findings involve extraData manipulation.

### V12 Finding 4: ERC721Receiver Check Bypass in mint Function (BaseNonfungibleToken.mint)
**Root Cause:** Using `_mint` instead of `_safeMint`, bypassing onERC721Received check
**Our Findings:** ⚠️ **POTENTIAL OVERLAP** - Our M-17 is about `burn()` function, not `mint()`. Let me verify...
- **M-17 Root Cause:** Missing liquidity check before allowing NFT burn + unrecoverable salt
- **V12 Root Cause:** Missing receiver capability check on mint
- **Analysis:** ✅ **DIFFERENT ROOT CAUSES** - M-17 is about burn safety, V12 is about mint safety. Different functions, different safety checks.

### V12 Finding 5: Stuck Ether Due to Unused Payable Modifier (BasePositions.withdrawProtocolFees)
**Root Cause:** Payable modifier without handling msg.value + no ETH withdrawal mechanism
**Our Findings:** ✅ None share this root cause. None of our findings involve stuck ETH from payable modifiers.

---

## DETAILED VERIFICATION OF IN-SCOPE FINDINGS

### ✅ M-14: Unsafe cast to int128 in fee accounting
- **Root Cause:** Unsafe cast from uint128 to int128 causing overflow/wraparound
- **V12 Comparison:** Not related to any V12 finding
- **Verdict:** UNIQUE

### ✅ L-16: MEVCapture dust accumulation
- **Root Cause:** Off-by-one error treating persistent storage as transient storage (subtracting 1 wei)
- **V12 Comparison:** Not related to any V12 finding
- **Verdict:** UNIQUE

### ✅ M-17: Burning Position NFT locks liquidity
- **Root Cause:** Missing liquidity/withdrawal check before allowing burn + unrecoverable salt from ephemeral data
- **V12 Comparison:** V12 has mint safety issue, this is burn safety issue - DIFFERENT
- **Verdict:** UNIQUE

### ✅ L-21: Missing zero-address check for recipient
- **Root Cause:** Missing validation for `recipient != address(0)`
- **V12 Comparison:** V12 has self-transfer check issue, this is zero-address check - DIFFERENT
- **Verdict:** UNIQUE

### ✅ M-19: Missing TWAMM function in interface
- **Root Cause:** Interface mismatch - function called but not implemented
- **V12 Comparison:** Not related to any V12 finding
- **Verdict:** UNIQUE

### ✅ H-22: TWAMM infinite recursion DoS
- **Root Cause:** Reentrancy via beforeSwap hook + state update after external call (CEI violation)
- **V12 Comparison:** Not related to any V12 finding
- **Verdict:** UNIQUE

### ✅ M-26: Silent sale rate truncation
- **Root Cause:** Unsafe cast from uint256 to uint112 without overflow check
- **V12 Comparison:** Not related to any V12 finding (different from M-14 which is uint128→int128)
- **Verdict:** UNIQUE

### ✅ H-28: TWAMM DoS via dense time initialization
- **Root Cause:** Unbounded loop through time bitmap + no gas limit check
- **V12 Comparison:** Not related to any V12 finding
- **Verdict:** UNIQUE

### ✅ M-32: MEVCapture overflow DoS
- **Root Cause:** Uncapped fee calculation leading to division by near-zero in amountBeforeFee
- **V12 Comparison:** Not related to any V12 finding
- **Verdict:** UNIQUE

### ✅ H-33: TWAMM rounding loss
- **Root Cause:** Rounding down to zero in `(saleRate * duration) >> 32` with high-frequency execution
- **V12 Comparison:** Not related to any V12 finding
- **Verdict:** UNIQUE

---

## [M-14]. Unsafe cast to int128 in fee accounting causes DoS for large amounts

**Status:** IN SCOPE - VALID
**Not in V12:** Confirmed - V12 does not cover this specific unsafe cast issue
**Severity:** Medium
**Location:** BasePositions.sol.handleLockData

**Description:**
In `BasePositions.handleLockData`, protocol fees (calculated as `uint128`) are explicitly cast to `int128` when calling `CORE.updateSavedBalances`. If the fee amount exceeds `type(int128).max` (approx 1.7e38), the value wraps to a negative number, causing `SavedBalanceOverflow` revert and DoS on withdrawals/fee collection.

**Impact:**
Denial of Service on withdrawals and fee collection for positions with accumulated fees exceeding `type(int128).max`. While this threshold is very high, the protocol is permissionless and should support any ERC20 token including high-supply tokens.

**Suggested Mitigation:**
Cast fee amounts to `int256` instead of `int128`:
```solidity
CORE.updateSavedBalances(
    poolKey.token0, poolKey.token1, bytes32(0),
    int256(uint256(swapProtocolFee0)),
    int256(uint256(swapProtocolFee1))
);
```

---

## [L-16]. Permanent Dust Accumulation due to Off-by-One Error in `loadCoreState`

**Status:** IN SCOPE - VALID
**Not in V12:** Confirmed - V12 does not cover this MEVCapture-specific issue
**Severity:** Low
**Location:** MEVCapture.loadCoreState

**Description:**
In `MEVCapture.loadCoreState`, the function reads fee balances from Core storage and subtracts 1 wei (`sub(fees0, gt(fees0, 0))`). This pattern is for transient storage, but Core's `savedBalances` are stored as raw values in persistent storage. This causes 1 wei to be permanently stuck per pool/token.

**Impact:**
1 wei permanently locked in Core contract for every MEVCapture pool and token combination. Economic loss limited to dust amounts.

**Suggested Mitigation:**
Remove the subtraction instructions in `loadCoreState` assembly block.

---

## [L-21]. Missing zero-address check for recipient in fee collection and withdrawal

**Status:** IN SCOPE - VALID
**Not in V12:** Confirmed - V12 does not cover this specific recipient validation issue
**Severity:** Low
**Location:** BasePositions.sol.collectFees

**Description:**
The `collectFees` and `withdraw` functions accept a `recipient` address but do not validate that it is not `address(0)`. If a user accidentally passes the zero address, funds are permanently burned.

**Impact:**
Permanent loss of user funds due to input error.

**Suggested Mitigation:**
Add validation:
```solidity
if (recipient == address(0)) revert InvalidRecipient();
```

---

## [H-22]. Infinite Recursion DoS in TWAMM Pool Swaps

**Status:** IN SCOPE - VALID (H-23 is duplicate)
**Not in V12:** Confirmed - V12 does not cover this TWAMM recursion issue
**Severity:** High
**Location:** TWAMM.beforeSwap

**Description:**
The `TWAMM` contract's `beforeSwap` hook calls `lockAndExecuteVirtualOrders`, which eventually calls `CORE.swap` to execute virtual orders. This inner swap triggers `TWAMM.beforeSwap` again. Since `TWAMM` does not update its `lastVirtualOrderExecutionTime` state variable until *after* the swap completes (violation of Check-Effects-Interactions), the re-entrant call sees stale state, passes the time check, and enters the execution loop again, causing infinite recursion.

**Impact:**
Permanent Denial of Service for all TWAMM-enabled pools; users cannot swap or execute orders.

**Suggested Mitigation:**
Modify `TWAMM.beforeSwap` to check if the current locker is the TWAMM contract itself:
```solidity
function beforeSwap(Locker locker, PoolKey memory poolKey, SwapParameters) external override(BaseExtension, IExtension) {
    // Prevent recursion: if we are the locker, we are already executing orders
    if (locker.addr() == address(this)) return;
    lockAndExecuteVirtualOrders(poolKey);
}
```

---

## [H-28]. TWAMM virtual order execution allows DoS via dense time initialization ✅ **FULLY VERIFIED** 🎯

**Status:** IN SCOPE - VALID (H-29 is duplicate)
**Not in V12:** Confirmed - V12 does not cover this TWAMM DoS vector
**Severity:** High
**Location:** TWAMM._executeVirtualOrdersFromWithinLock
**Violates Core Invariant:** Extensions in repository should never block withdrawal within block gas limit (ekubo-scope.md lines 65-71)

**Description:**
The `TWAMM` extension executes virtual orders by iterating through initialized time intervals from the last execution time to the current block timestamp. An attacker can cheaply create many orders with sequentially increasing end times (e.g., every 1 second), densely populating the `poolInitializedTimesBitmap`. Since the `while` loop must process all intervals up to the current block timestamp, a sufficiently dense bitmap combined with a period of inactivity can cause the required gas to exceed the block gas limit, permanently freezing the pool.

**Impact:**
Permanent DoS of the liquidity pool; assets cannot be swapped or withdrawn if the loop cost exceeds the block gas limit.

**Suggested Mitigation:**
Modify `_executeVirtualOrdersFromWithinLock` to check remaining gas or limit the number of iterations per call:
```solidity
function _executeVirtualOrdersFromWithinLock(PoolKey memory poolKey, PoolId poolId) internal {
    uint256 time = realLastVirtualOrderExecutionTime;
    uint256 GAS_BUFFER = 100_000;

    while (time != block.timestamp) {
        if (gasleft() < GAS_BUFFER) {
            break;
        }
        // ... existing loop logic ...
    }
}
```

---

## [L-32]. Overflow in fee calculation causes DoS during high volatility

**Status:** IN SCOPE - VALID (downgraded from Medium to Low)
**Not in V12:** Confirmed - V12 does not cover this MEVCapture overflow issue
**Severity:** Low (MEDIUM Impact + RARE Likelihood = LOW)
**Location:** MEVCapture.handleForwardData

**Description:**
In `handleForwardData`, the `additionalFee` is calculated based on tick movement from the block start. If the price moves significantly (e.g., >100 ticks with tickSpacing=1), the fee can approach 100% (2^64 in fixed point). The subsequent call to `amountBeforeFee` calculates `input / (1 - fee)`. When `fee` is close to 1, the divisor is small and the result scales massively, potentially exceeding `uint128` or causing a revert.

**Impact:**
Temporary Denial of Service for pools with tight tick spacing (e.g., 1) during high volatility. For pools with tickSpacing=1 and 1% pool fee, a price movement of ~100 ticks results in an `additionalFee` of >= 100%. This causes `amountBeforeFee` to revert, temporarily preventing swaps that move the price significantly.

**Why Low Severity:**
- **MEDIUM Impact:** Temporary DoS (not permanent), no fund loss, pool recovers when volatility decreases
- **RARE Likelihood:** Requires tickSpacing=1 (uncommon - most pools use 10, 60, 200) + significant price movement (>100 ticks)
- **Per severity matrix:** MEDIUM Impact + RARE Likelihood = LOW

**Suggested Mitigation:**
Cap the `additionalFee` at a safe maximum percentage, such as 50%:
```solidity
uint64 maxAdditionalFee = 0x8000000000000000; // 50%
additionalFee = uint64(FixedPointMathLib.min(maxAdditionalFee, (feeMultiplierX64 * poolFee) >> 64));
```

---

