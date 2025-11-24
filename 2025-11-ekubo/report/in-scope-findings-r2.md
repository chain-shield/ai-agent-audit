# IN-SCOPE FINDINGS - ROUND 2 (R2)

**H-47, H-50** - Storage Collision in Incentives Contract
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - Unbounded index allows storage collision, can drain contract (H-50 likely duplicate)

**H-51, H-109** - MEVCapture Extension DoS
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - beforeSwap unconditionally reverts, but handleForwardData calls CORE.swap which triggers beforeSwap (H-109 likely duplicate)

**H-89, M-90** - Costless Oracle Manipulation via Zero-Fee Pools
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - Oracle requires fee==0, enabling costless manipulation. Claims "manipulation resistant" but removes economic barrier. No scope disclaimer like TWAMM (M-90 likely duplicate)

**H-91, H-94, H-112, H-132, H-135** - TWAMM Infinite Recursion DoS
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - beforeSwap calls lockAndExecuteVirtualOrders → CORE.swap → beforeSwap (infinite loop). Affects ALL users, not self-DoS. Pool permanently frozen (duplicates likely)

**H-126** - MEV Capture fee evasion via backrun manipulation
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - tickLast only updates on new blocks. Backrun to block-start tick pays 0 MEV fees. Attacker profits: saves backrun fee (e.g., 6%), victim overpays (charged for attacker's frontrun too). Makes sandwiches 2-3x more profitable.

**H-127** - Revenue Extraction via TWAMM Duration Manipulation
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - roll() updates state even with amountToSpend==0. Attacker calls roll() with 0 balance (sets 24h window), waits 23h, calls again (forces 1h execution = 24x sale rate). Attacker profits as arbitrageur: buys buyback token at depressed price during forced high-rate execution, sells at normal price. Protocol loses 20-30% of value, attacker captures profit. Atomic attack possible in single tx.

**M-97** - Strict configuration check causes stuck protocol fees
- **Status:** ✅ VALID - MEDIUM - GOVERNANCE/OPS FRICTION BUT REAL FEE STICKINESS. `PositionsOwner.withdrawAndRoll` currently reverts unless *both* tokens in the pair are configured in `RevenueBuybacks`, despite the comment saying "at least one". In a permissionless AMM it is fully in line with the intended spec for governance to configure *only* the actual revenue token (e.g. WETH/USDC, or a governance token paired with some arbitrary asset) and leave the other side unconfigured; `RevenueBuybacks.configure` is per-token, not per-pair. Under that normal configuration, protocol fees for the configured revenue token become stuck whenever its pool is paired with an unconfigured token, until governance either configures that second token or migrates ownership. Users can’t trigger collection on the healthy side alone, and there is no alternative trustless withdrawal path from `PositionsOwner`, so this passes Gate 3 (stuck protocol revenue) and Gate 4 (occasional but realistic given permissionless pools) as a genuine Medium that should be fixed in code, not just in docs.

**M-98** - Gas limit DoS on fee withdrawal
- **Status:** ✅ VALID - MEDIUM - COUPLED ROLLING CAN DOs HEALTHY TOKEN FEES. `PositionsOwner.withdrawAndRoll` always calls `BUYBACKS.roll(token0)` and then `BUYBACKS.roll(token1)` in a single transaction. If one token's TWAMM state becomes extremely expensive to execute (e.g. backlog of virtual orders), `roll` can revert or OOG and thereby block withdrawal & rolling of fees for the *other* token in the pair as well. There is no per-token escape hatch inside `PositionsOwner`, so protocol fee collection for healthy tokens can be DoSd until governance intervenes (e.g. changing owner/Buybacks), which meets Gate 3 (DoS of core fee collection) and Gate 4 (occasional but plausible under stressed pools) for Medium severity.
## TRIAGE SUMMARY

**Total Findings in R2:** 141 findings
- High: 34
- Medium: 66
- Low: 41

**Triage Status:** IN PROGRESS

---

## SCOPE CRITERIA (from ekubo-scope.md)

### OUT OF SCOPE:
1. **Fee-on-Transfer (FoT) tokens** - Lines 11-15: "Non-standard ERC20 assets (e.g., fee-on-transfer, rebasing) are not supported"
2. **Slippage/Deadline protection** - Lines 23-33: "Missing slippage or deadline protection is not a valid finding"
3. **Extension freezing pools** - Lines 17-21: "Extensions NOT expected to freeze pools" (but in-scope extensions SHOULD NOT freeze)
4. **Third-party extensions** - Only in-repo extensions are in scope
5. **View-only functions** - Lines 35-37: "View-only functions returning incorrect data is QA/Low"
6. **Known issues from V12.md**

### IN SCOPE:
1. Core protocol contracts (Core, BasePositions, Router, etc.)
2. In-repo extensions (TWAMM, MEVCapture, Oracle, Incentives, Orders)
3. Violations of stated invariants (lines 65-71)
4. Actual vulnerabilities causing fund loss, DoS, or accounting corruption

---

## QUICK TRIAGE CATEGORIES

### Category 1: Fee-on-Transfer (FoT) Token Issues - OUT OF SCOPE
**Reason:** Explicitly not supported per scope lines 11-15

- M-2, M-3, M-4 - Router/MEVCaptureRouter FoT incompatibility
- M-6 - Incentives FoT incompatibility
- M-58 - Slippage check with FoT
- M-68, M-69 - FoT insolvency in Orders/RevenueBuybacks/Incentives
- M-96 - Incentives FoT accounting

**Count:** ~8 findings

### Category 2: Slippage/Deadline Protection - OUT OF SCOPE
**Reason:** Explicitly not valid per scope lines 23-33

- M-55, M-56, M-57, M-59, M-60, M-61, M-62, M-63, M-64, M-65, M-66, M-67 - Missing slippage/deadline
- M-113 - Missing transaction expiration
- M-129 - Missing slippage in withdrawal
- M-139, M-140 - Slippage bypass/TWAMM slippage

**Count:** ~16 findings

### Category 3: View-Only Function Issues - QA/LOW (OUT OF SCOPE for Medium/High)
**Reason:** Scope lines 35-37

- L-7, L-85 - CoreDataFetcher zero price for uninitialized pools
- M-24 - QuoteDataFetcher incomplete data
- M-87 - PriceFetcher invalid price
- M-88 - Misleading cross-pair liquidity
- M-107 - QuoteDataFetcher stale data
- M-114 - QuoteData corruption
- L-93 - Liquidity reporting overflow
- L-106 - QuoteDataFetcher DoS
- L-115 - Unbounded loop in QuoteDataFetcher

**Count:** ~9 findings

### Category 4: Standard Violations / QA Issues - OUT OF SCOPE for Medium/High
**Reason:** These are quality/standard compliance, not security vulnerabilities

- L-1 - ERC20 allowance race (known ERC20 issue)
- M-10, L-16, L-19 - NFT mint lacks onERC721Received (V12 already has this)
- M-11, L-28 - TokenWrapper Transfer event violations
- L-15 - StandardViolation in tokenURI
- L-17, L-22 - TokenWrapper assumes optional metadata
- L-25 - TokenWrapperFactory lacks idempotent deployment
- L-84 - TokenWrapperFactory deceptive wrappers
- L-99, M-100 - TokenWrapper metadata issues
- L-101, L-102, L-103 - Lens contract issues

**Count:** ~13 findings

---

## POTENTIAL IN-SCOPE FINDINGS (Requires Deep Analysis)

### HIGH SEVERITY - PRIORITY REVIEW

**H-5** - TokenWrapper functionality broken: Wraps tokens without crediting user balance
- **Status:** ❌ INVALID - Finding itself admits it's invalid; misunderstands flash accountant pattern

**H-12** - Core contract missing critical functions required by Orders contract causing DoS
- **Status:** ❌ DUPLICATE of R1 M-19 - Orders calls CORE.updateSaleRate/collectProceeds but should use TWAMMLib library functions

**H-21** - Router Checks Fixed Output Instead of Calculated Input for Exact Output Swaps
- **Status:** ❌ OUT OF SCOPE - Missing slippage protection (scope lines 23-33)

**H-34** - PoolState packing truncation leads to severe accounting corruption
- **Status:** ❌ INVALID - Wrong bit widths: SqrtRatio is 96 bits (not 160), Tick is 32 bits (not 24), total is 256 bits (not 312)

**H-36, H-44, H-117** - Storage collision in Oracle extension
- **Status:** ❌ THEORETICAL - Finding admits "practical exploitation is infeasible as it requires breaking Keccak256 preimage resistance"

**H-38, H-39** - TWAMM Proceeds Trapped Due to Locker Mismatch
- **Status:** ❌ DUPLICATE of H-12 - Orders calls CORE.collectProceeds which doesn't exist (should use TWAMMLib)

**H-40, H-42, H-45, H-80** - Stuck ETH / Router uses contract balance
- **Status:** ❌ INVALID - Router automatically refunds excess ETH to swapper (lines 138-139), no funds stuck

**H-47, H-50** - Storage Collision in Incentives Contract
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - Unbounded index allows storage collision, can drain contract (H-50 likely duplicate)

**H-51, H-109** - MEVCapture Extension DoS
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - beforeSwap unconditionally reverts, but handleForwardData calls CORE.swap which triggers beforeSwap (H-109 likely duplicate)

**H-54** - Unsafe integer downcast in TWAMM inverts trade direction
- **Status:** - LOW SEVERITY (downgraded) - Bug exists but EXTREMELY RARE: needs saleRate > 1e33 + 9hrs-2days inactivity + no pool activity

**H-71, H-72** - TokenWrapper DoS due to FlashAccountant ABI Mismatch
- **Status:** ❌ INVALID - Misunderstands `using for` directive. TokenWrapper uses FlashAccountantLib which has correct 20-byte assembly encoding

**H-82** - State Corruption via Reentrancy in Core.swap
- **Status:** ❌ INVALID - Bug already fixed. State is read AFTER beforeSwap hook (line 532), not before. Finding admits "fix is already present"

**H-83** - Unchecked access in updateSavedBalances allows draining
- **Status:** ❌ INVALID - _requireLocker() enforces msg.sender == locker (line 56 FlashAccountant). Extensions cannot call it.

**H-89, M-90** - Costless Oracle Manipulation via Zero-Fee Pools
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - Oracle requires fee==0, enabling costless manipulation. Claims "manipulation resistant" but removes economic barrier. No scope disclaimer like TWAMM (M-90 likely duplicate)

**H-91, H-94, H-112, H-132, H-135** - TWAMM Infinite Recursion DoS
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - beforeSwap calls lockAndExecuteVirtualOrders → CORE.swap → beforeSwap (infinite loop). Affects ALL users, not self-DoS. Pool permanently frozen (duplicates likely)

**H-126** - MEV Capture fee evasion via backrun manipulation
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - tickLast only updates on new blocks. Backrun to block-start tick pays 0 MEV fees. Attacker profits: saves backrun fee (e.g., 6%), victim overpays (charged for attacker's frontrun too). Makes sandwiches 2-3x more profitable.

**H-127** - Revenue Extraction via TWAMM Duration Manipulation
- **Status:** ✅ **VALID - HIGH SEVERITY** 🎯 - roll() updates state even with amountToSpend==0. Attacker calls roll() with 0 balance (sets 24h window), waits 23h, calls again (forces 1h execution = 24x sale rate). Attacker profits as arbitrageur: buys buyback token at depressed price during forced high-rate execution, sells at normal price. Protocol loses 20-30% of value, attacker captures profit. Atomic attack possible in single tx.

**H-131, H-133, H-134** - TWAMM Unbounded Loop DoS
- **Status:** LIKELY DUPLICATES of R1 H-28 (already verified)

**H-136** - TWAMM price manipulation via sandwich attacks
- **Status:** ❌ INVALID - OUT OF SCOPE. Uses MIN/MAX_SQRT_RATIO (no slippage protection), enables sandwiches. BUT scope explicitly states "execution price is not guaranteed" and "bad price is a known risk" (lines 23-33). Design choice, not bug.

---

### MEDIUM SEVERITY - PRIORITY REVIEW

**M-8, L-77** - Accounting Drift in MEV Capture due to Incorrect 1-wei Offset
- **Status:** ✅ DUPLICATE of R1 L-16 (already verified in-scope)

**M-9** - ERC7726 Oracle denies service due to insufficient snapshot capacity
- **Status:** **VALID - LOW SEVERITY** (downgraded) - Default capacity=1 insufficient for TWAP. ERC7726 is lens contract (not core), easy fix via expandCapacity() (permissionless), no fund loss. Configuration issue.

**M-13, M-14, L-18, M-95** - Swapped return values in poolTicks
- **Status:** ❌ INVALID - FALSE POSITIVE. CoreLib.poolTicks unpacks correctly: liquidityDelta from lower 128 bits, liquidityNet from upper 128 bits via bytes16() cast. Finding misunderstands bytes16(bytes32) takes UPPER 16 bytes, not lower.

**M-20, M-26** - TokenWrapper Total Supply Invariant Violation via Flash Mint
- **Status:** **VALID - LOW SEVERITY** (downgraded) - Flash mint via withdraw() creates temporary sum(balances) > totalSupply. Transient only (must repay in same tx), debt system enforces settlement. Limited exploits (governance/oracles query post-tx state). Design quirk, not critical.

**M-23** - TokenWrapper updateDebt call reverts due to ABI encoding mismatch
- **Status:** ❌ INVALID - FALSE POSITIVE. TokenWrapper uses `using FlashAccountantLib for *;`, so `CORE.updateDebt(...)` resolves to the library helper, which hand-packs a 20-byte payload (selector + 16-byte int128) exactly as `FlashAccountant.updateDebt` expects. No ABI length mismatch or DoS occurs.

**M-27, M-37** - TokenWrapper zero-address transfer issues
- **Status:** VALID - LOW SEVERITY (user-error–dependent accounting drift). `transfer(to=address(0))` burns wrapper tokens without updating Core `savedBalances`, so `totalSupply` exceeds sum(balances). However, it only occurs when users or integrations explicitly choose the zero address as recipient, so it cannot justify Medium severity and should be treated as a single Low-severity issue for M-27/M-37 combined.

**M-29, L-48, L-52, L-104, L-105, L-116** - Sale rate truncation / rounding in Orders
- **Status:** ❌ INVALID - DUPLICATE of R1 M-26 (sale rate truncation already judged invalid there).

**M-32** - Denial of Service via forced excess payments in FlashAccountant
- **Status:** ❌ INVALID - DUPLICATE / LOW-SEVERITY variant of L-31 (ForcedAssetVsStrictEquality). The 'forced credit' via `FlashAccountant.receive` is real, but it only occurs when an external contract chosen by the locker/user sends ETH back to Core during a lock, i.e. it fails Gate 2 (user/integration error) and is already covered by L-31's strict donation/composability analysis.

**M-35, M-41, M-53** - MEV Capture Fees Diverted/Burned
- **Status:** INVALID - BY DESIGN / DOCS MISMATCH. MEVCapture intentionally routes its extra fees into `CORE.accumulateAsFees`, crediting LP fee buckets instead of a dedicated “protocol revenue” account, and `Core.accumulateAsFees` explicitly burns fees when pool liquidity is zero (per its own comment). This affects how MEV fees are split between LPs and the DAO, but does not create accounting drift, user fund loss, or a security invariant break; it is an economic/configuration choice already covered by other MEVCapture findings (fee bypass, convexity, overflow) rather than a separate Medium-severity bug.

**M-43** - Oracle.expandCapacity overwrites recent history
- **Status:** INVALID - BY DESIGN / NO EXTRA DATA LOSS. Oracle uses a circular buffer where old snapshots are always overwritten as new ones are written; `expandCapacity` only pre-allocates additional slots and gradually increases the usable window as the write index advances. The first writes after expansion behave exactly as they would without expansion (still overwriting the oldest entries), so the call does not introduce new history loss or accounting drift beyond the intended ring-buffer behavior, and tests like `test_expandCapacity_doesNotOverwrite` cover this semantics.

**M-46, M-130** - TokenWrapper transient storage issues
- **Status:** INVALID - USER ERROR / VIEW-ONLY. The `coreBalance` transient pattern is intentional: Core never holds real TokenWrapper balances, total supply is tracked via `CORE.savedBalances`, and `balanceOf(CORE)` is a transient intra-transaction convenience. M-46 is purely an ERC20 accounting/observability quirk (view-only, falling under Category 3: view-function issues → QA/Low), and M-130 requires a user or integrator to send TokenWrapper tokens directly to `address(CORE)` outside the documented Router/forward flows (choosing a bad recipient). Both therefore fail Gate 2 (user error) and the scope/View-only checks, and are treated as design footguns rather than separate Medium-severity vulnerabilities.

**M-49** - DoS on Incentives Refund due to Accounting Mismatch
- **Status:** INVALID - OUT OF SCOPE (FEE-ON-TRANSFER). The refund mismatch only occurs when the drop token is fee-on-transfer/deflationary; for standard ERC20s, `fund` and `refund` remain consistent and cannot lock funds. Ekubo's scope and Gate 6 explicitly exclude fee-on-transfer/rebasing edge cases, and this issue shares the same FoT accounting root cause already captured by M-6/M-69/M-96 ("Fee-on-Transfer tokens break Incentives accounting"), so it is not a separate in-scope Medium DoS finding.

**M-73, M-74, L-75, L-79, L-141** - Public refundNativeToken / roll function theft
- **Status:** INVALID - DUST / ECONOMIC DESIGN & DUPLICATES. All five findings concern dust or small residual ETH left in `Orders`/`RevenueBuybacks` due to TWAMM fixed-point rounding, which is (i) strictly bounded by order duration (≪ 1e-6 ETH for realistic configs) or at most 1 wei, and (ii) either already covered by H-40/H-80 (same `refundNativeToken` pattern on Router/MEVCaptureRouter) or classified as QA/Low dust leakage. Gate 3 impact is dust only, and Gate 11 safeguards plus UIs can always reclaim dust via multicall `refundNativeToken`, so we treat these as economic design trade-offs and documentation/UX issues rather than additional Medium/Lows for access control.

**M-81, M-92** - Swap Output Clamping
- **Status:** INVALID - EXTREME LIMITS / DUPLICATE. Both findings describe the same internal clamp at `type(int128).min` in `Core.swap_6269342730` when theoretical deltas exceed the int128 range. Hitting this requires outputs on the order of >2^127 (~1.7e38 units), which is impossible for any realistically listed token (18 decimals, bounded total supply) and would imply pathological pool liquidity/price far beyond deployment constraints. Moreover, the core issue (swaps at extreme magnitudes) is already captured and mitigated via the reentrancy/looping protections and validation of pool parameters. Under Gate 4 this is RARE-to-unrealistic and under Gate 3 it does not create a practical new extraction vector distinct from the general "don’t use insane token scales" guidance, so we de-duplicate M-81/M-92 as non-actionable edge-case clamping rather than separate Mediums.

**M-90** - Costless Oracle Manipulation (duplicate of H-89)
- **Status:** DUPLICATE

**M-97** - Strict configuration check causes stuck protocol fees
- **Status:** ✅ VALID - GOVERNANCE/OPS FRICTION BUT REAL FEE STICKINESS. `PositionsOwner.withdrawAndRoll` currently reverts unless *both* tokens in the pair are configured in `RevenueBuybacks`, despite the comment saying "at least one". In a permissionless AMM it is fully in line with the intended spec for governance to configure *only* the actual revenue token (e.g. WETH/USDC, or a governance token paired with some arbitrary asset) and leave the other side unconfigured; `RevenueBuybacks.configure` is per-token, not per-pair. Under that normal configuration, protocol fees for the configured revenue token become stuck whenever its pool is paired with an unconfigured token, until governance either configures that second token or migrates ownership. Users can’t trigger collection on the healthy side alone, and there is no alternative trustless withdrawal path from `PositionsOwner`, so this passes Gate 3 (stuck protocol revenue) and Gate 4 (occasional but realistic given permissionless pools) as a genuine Medium that should be fixed in code, not just in docs.

**M-98** - Gas limit DoS on fee withdrawal
- **Status:** ✅ VALID - COUPLED ROLLING CAN DOs HEALTHY TOKEN FEES. `PositionsOwner.withdrawAndRoll` always calls `BUYBACKS.roll(token0)` and then `BUYBACKS.roll(token1)` in a single transaction. If one token's TWAMM state becomes extremely expensive to execute (e.g. backlog of virtual orders), `roll` can revert or OOG and thereby block withdrawal & rolling of fees for the *other* token in the pair as well. There is no per-token escape hatch inside `PositionsOwner`, so protocol fee collection for healthy tokens can be DoSd until governance intervenes (e.g. changing owner/Buybacks), which meets Gate 3 (DoS of core fee collection) and Gate 4 (occasional but plausible under stressed pools) for Medium severity.

**M-108** - Incentives refund blocked by reverting owner
- **Status:** INVALID - USER ERROR / SELF-GRIEF. `refund` only ever sends the remaining drop balance back to `key.owner` via `SafeTransferLib.safeTransfer(key.token, key.owner, refundAmount)`. The only way this can be "blocked by a reverting owner" is if the drop creator themselves chose an `owner` address that cannot receive `key.token` (e.g. a contract that always reverts on `transfer`), in which case they are just making their own refund impossible. Recipients can still claim normally, no third party can be griefed, and there is no protocol-funds impact. Under Gate 2 this is classic user-error (bad recipient choice) and under Gate 3 the impact is limited to the misconfigured owner, so we treat this as a UX/design footgun rather than a Medium-severity vulnerability.

**M-110, M-128** - Oracle defaults to unsafe capacity
- **Status:** DUPLICATE - ORACLE CONFIG / H-89. The fact that `Oracle` initializes new tokens with `capacity = max(1, c.capacity())` means the default ring buffer only holds 1 snapshot until someone calls `expandCapacity`. That default on its own does not violate any documented invariant; production deployments are expected to configure capacity/observation windows via governance or integration. The real security concern — that using too-small capacity/windows can make the Oracle/PriceFetcher manipulable — is already captured in H-89 (and its Medium duplicate M-90), so M-110 and M-128 are simply re-framings of the same root cause rather than independent bugs.

**M-111** - Unbounded funding cost in Incentives.fund
- **Status:** INVALID - USER ERROR / VOLUNTARY OVER-FUNDING. `fund` always pulls tokens from `msg.sender` (`SafeTransferLib.safeTransferFrom(key.token, msg.sender, address(this), fundedAmount)`) and only ever increases the drop's `funded` amount toward the caller-specified `minimum`. An attacker cannot force anyone else to pay or approve; the only way to incur a large "funding cost" is for a sponsor to call `fund` with a very large `minimum` while having enough balance/allowance, i.e. willingly over-funding the drop. Claims and refunds continue to work as designed, there is no DoS or third-party griefing, and Gate 2 classifies this as user error rather than a protocol bug.

**M-122, M-123, M-138** - MEVCapture fee logic issues
- **Status:** INVALID - ECONOMIC DESIGN / FEE POLICY, NOT A BUG. The MEVCapture extension is explicitly a dynamic-fee layer (`contract MEVCapture ... /// @notice Charges additional fees based on the relative size of the priority fee`) that reroutes extra fees into `savedBalances` and then into pool fees via `accumulatePoolFees`. The behaviors raised in M-122/M-123/M-138 (how the extra fee scales with tick movement, saturation at large multipliers, and minor rounding artifacts) follow directly from this design: Router minOut/maxIn checks still bound what users pay, Core accounting remains consistent, and no one can steal protocol/user funds or DoS swaps. Under Gate 3 this is a question of fee schedule economics rather than a Medium/High vulnerability, and under Gate 8 it is documented by design, so we treat these as design/UX discussion points or at most QA, not separate Medium findings.

**M-124** - Oracle TWAP manipulation
- **Status:** DUPLICATE - SAME ZERO-FEE ORACLE MANIPULATION AS H-89/M-90. The attack relies on cheaply pushing the Oracle ETH/token pool's price and then relying on PriceFetcher/ERC7726 using `extrapolateSnapshot` over that window. This is exactly the "costless Oracle manipulation via zero-fee pools" already captured as H-89 (with M-90 marked DUPLICATE): Oracle enforces fee==0, full-range pools, so there is no trading-fee cost to moving the oracle price. M-124 does not introduce a distinct invariant break or consumer, it just restates the same zero-fee-TWAP manipulability, so under Gate 3/Gate 11 we treat it as fully subsumed by H-89/M-90 rather than a new Medium.

**M-125** - TWAMM Price Manipulation
- **Status:** INVALID - OUT OF SCOPE / EXECUTION PRICE NOT GUARANTEED. M-125 argues that TWAMM's long-term order mechanics allow adversaries to manipulate execution price (e.g., via sandwiches or price path shaping). However, this is the same class of "bad execution price" risk already covered and explicitly excluded in scope, and mirrors H-136 which we mark INVALID for the same reason. The protocol never promises slippage-free TWAMM execution, Router parameters still give users standard minOut/maxIn protections, and no accounting invariant, solvency, or liveness property is violated beyond the expected MEV/price-impact tradeoffs of on-chain AMMs. Under the checklist's scope and Gate 3 this is market-structure/MEV design, not a separate Medium-severity bug.

---

### LOW SEVERITY - QUICK REVIEW

**L-30** - NFT ID to Order Key One-to-Many Mapping Violation
- **Status:** NEEDS REVIEW

**L-31** - Strict balance tracking locks donated assets
- **Status:** NEEDS REVIEW - By design?

**L-33, L-76** - Protocol Fee Accounting Drift via Dust Withdrawals
- **Status:** NEEDS REVIEW - Duplicates

**L-70** - Unsafe recipient in FlashAccountant withdrawal
- **Status:** NEEDS REVIEW

**L-78** - RevenueBuybacks Leaks ETH Dust
- **Status:** NEEDS REVIEW

**L-86** - Critical price desync risk (requires admin role)
- **Status:** OUT OF SCOPE - Admin/governance issue

**L-118, L-119, L-120** - Missing zero-address check for recipient
- **Status:** LIKELY DUPLICATE of R1 L-21 (already in-scope)

**L-121** - Protocol fee leakage via rounding
- **Status:** NEEDS REVIEW

**L-137** - Precision Drift in TWAMM Order Accounting
- **Status:** NEEDS REVIEW

---

## SUMMARY OF INITIAL TRIAGE

### Definitely OUT OF SCOPE: ~49 findings
- Fee-on-Transfer: 8 findings
- Slippage/Deadline: 16 findings
- View-only functions: 9 findings
- Standard violations/QA: 13 findings
- **V12 Duplicates: 3 findings** (M-10, L-16, L-19 - NFT mint safety)

### Likely DUPLICATES of R1: ~15 findings
- H-91, H-94, H-112, H-132, H-135 → R1 H-22 (TWAMM recursion)
- H-131, H-133, H-134 → R1 H-28 (TWAMM unbounded loop)
- M-8, L-77 → R1 L-16 (MEVCapture dust)
- M-29, L-48, L-52, L-104, L-105, L-116 → R1 M-26 (sale rate truncation - invalid)
- L-118, L-119, L-120 → R1 L-21 (zero-address check)
- M-10, L-16, L-19 → V12 (NFT mint safety)

### Requires DEEP ANALYSIS: ~80 findings
- High severity: ~20 unique findings
- Medium severity: ~40 unique findings
- Low severity: ~20 unique findings

---

## ✅ V12 ROOT CAUSE COMPARISON - COMPLETE

### V12 Finding 1: Self-Transfer Balance Invariant Violation (TokenWrapper.transfer)
**Root Cause:** Missing `to == msg.sender` check
**R2 Findings:** M-11, M-27, M-37, L-28 (TokenWrapper transfer issues)
**Analysis:** These are about Transfer event parameters and zero-address, NOT self-transfer
**Verdict:** ✅ NO DUPLICATES

### V12 Finding 2: Uncollected Fee Loss on Full Liquidity Removal (Core.updatePosition)
**Root Cause:** Fee checkpoint reset to (0,0) without computing/transferring fees when liquidityNext == 0
**R2 Findings:** L-121 (Protocol fee leakage via rounding)
**Analysis:** L-121 is about rounding in withdrawal, NOT fee checkpoint reset on full removal
**Verdict:** ✅ NO DUPLICATES

### V12 Finding 3: Unrestricted extraData Writing (Core.setExtraData)
**Root Cause:** Missing existence/liquidity check before writing extraData
**R2 Findings:** None found
**Verdict:** ✅ NO DUPLICATES

### V12 Finding 4: ERC721Receiver Check Bypass (BaseNonfungibleToken.mint)
**Root Cause:** Using `_mint` instead of `_safeMint`
**R2 Findings:** ❌ **M-10, L-16, L-19** - All about NFT mint lacking onERC721Received
**Verdict:** ❌ **DUPLICATES - MOVED TO OUT OF SCOPE**

### V12 Finding 5: Stuck Ether (BasePositions.withdrawProtocolFees)
**Root Cause:** Payable modifier without msg.value handling + no ETH withdrawal mechanism
**R2 Findings:** H-40, H-42 (refundNativeToken), H-45, H-80 (Router using contract balance)
**Analysis:** These are about DIFFERENT issues:
- H-40, H-42: Public refundNativeToken allowing theft
- H-45, H-80: Router using contract ETH balance to cover debts
- NOT about payable modifier without handling
**Verdict:** ✅ NO DUPLICATES (different root causes)

---

---

## ✅ V12 COMPARISON COMPLETE

**Total V12 Findings:** 5
**R2 Duplicates Found:** 3 findings (M-10, L-16, L-19)

**Confirmed OUT OF SCOPE (V12 Duplicates):**
- ❌ **M-10** - Positions NFT `mint` lacks `onERC721Received` check → V12 Finding 4
- ❌ **L-16** - Positions NFTs minted to non-receiver contracts → V12 Finding 4
- ❌ **L-19** - Unsafe minting allows NFTs stuck → V12 Finding 4

**All other R2 findings have DIFFERENT root causes from V12.**

---

## UPDATED TRIAGE SUMMARY

### Definitely OUT OF SCOPE: ~49 findings
- Fee-on-Transfer: 8 findings
- Slippage/Deadline: 16 findings
- View-only functions: 9 findings
- Standard violations/QA: 13 findings
- **V12 Duplicates: 3 findings** ✅

### Likely DUPLICATES of R1: ~15 findings
- TWAMM Recursion: 5 findings → R1 H-22
- TWAMM Unbounded Loop: 3 findings → R1 H-28
- MEVCapture Dust: 2 findings → R1 L-16
- Sale Rate Truncation: 6 findings → R1 M-26 (invalid)
- Zero-address check: 3 findings → R1 L-21

### Requires DEEP ANALYSIS: ~74 findings (reduced from 80)
- High severity: ~17 unique findings (removed H-5, H-12, H-21)
- Medium severity: ~40 unique findings
- Low severity: ~17 unique findings

### QUICK FILTERED (H findings):
- ❌ H-5 - Invalid (finding admits it's invalid)
- ❌ H-12 - Duplicate of R1 M-19
- ❌ H-21 - Out of scope (slippage protection)

---

## NEXT STEPS

1. ✅ Check for duplicates against V12.md (COMPLETE)
2. Review HIGH severity findings against R1
3. Apply remaining scope filters
4. Verify root causes for remaining findings
5. Create detailed in-scope findings list

---


