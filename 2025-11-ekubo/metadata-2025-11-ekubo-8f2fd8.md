
## PROTOCOL OVERVIEW:

# Ekubo Protocol – Architectural & Functional Overview

Ekubo is a next–generation Automated-Market-Maker (AMM) that combines a singleton architecture, ultra-fine-grained concentrated liquidity, and a permission-less extension system to deliver best-in-class trade execution and capital efficiency while remaining fully composable.  
Below is a holistic, developer–oriented explanation of how the system is put together, how value flows through it, and how each major contract cohort collaborates to create the final user experience.

---
## 1. Architectural Principles

1. **Singleton Core** – All pools live inside a single contract (`Core.sol`).  Liquidity, positions, fee accounting and token balances are shared in one deterministic storage layout, removing the need for separate pool deployments and allowing cross-pool routing without extra `call` overhead.

2. **“Till” Pattern & Flash Accountant** – All user token transfers are **deferred** to the very end of the transaction.  During execution, each externally-owned account or contract that interacts with Core is treated as a “Locker”; its inflows/outflows are tracked in signed transient debt variables held by `FlashAccountant.sol`.  When the accountant’s lock closes the debts must net to zero, at which point at most the *minimal* number of ERC-20 / ETH transfers are performed.  This saves gas, enables multi-pool batching, and side-steps mis-behaving ERC-20 tokens.

3. **Super-Concentrated Liquidity** – Liquidity providers choose a price range using ticks that are 100× smaller (1/10 000 th of a tick) than most AMMs.  More granular ticks mean that $1 in Ekubo provides the same depth as ~$100 in a conventional 0.01-tick AMM.

4. **Extensions** – Anyone can permission-lessly deploy contracts that hook into well-defined call-points inside Core (before/after swap, position update, fee collect, etc.).  Examples shipped in this repository:
   * **Oracle** – manipulation-resistant on-chain TWAP snapshots.
   * **TWAMM** – long-term time-weighted orders (DCA).
   * **MEV Capture** – extracts an additional dynamic fee based on tick movement to subsidise protocol buybacks.

   Because extensions run *inside* Core’s accounting they inherit the till pattern automatically and never custody user funds.

5. **Governance Minimalism** – The canonical contracts have *no mutable admin controls* over user funds.  Governance (held by Ekubo DAO) can only withdraw the protocol revenue that Core or Positions accrued, primarily to recycle it into buybacks.  Immutable parameters such as swap fee, withdrawal fee denominator, or MEV extension address are hard-coded at construction.

---
## 2. Token Flow in a Typical Swap

1. **User/Router Interaction**  
   A caller invokes `Router.swap` or any of its multi-hop variants.  The router immediately opens a **lock** on the `FlashAccountant`, becoming the *current Locker*.

2. **Deferred Accounting**  
   Using either a direct Core call or a forwarded extension call (`_swap`), the router performs the swap logic.  No ERC-20 transfers happen; instead
   * Core updates the pool’s √price, tick, liquidity, and global per-token *fee-per-liquidity* counters.
   * The router’s signed *debts* vs each token are mutated inside transient storage via `ACCOUNTANT.updateDebt`.

3. **Lock Closure**  
   When the router exits, `FlashAccountant` verifies that all token debts are settled (zero) and executes **at most one `transfer` or `transferFrom` per token** to resolve the net difference.  If the user supplied ETH, any remainder is automatically refunded by the router’s `refundNativeToken` helper.

4. **Result**  
   The caller receives the bought asset, and gas consumption is close to the theoretical minimum because: zero token transfers during the swap path + one transfer at the very end.

---
## 3. Liquidity Provisioning & NFTs

*Liquidity positions are ERC-721s* managed by `Positions.sol` (concrete) / `BasePositions.sol` (abstract).  An LP can:

• `mintAndDeposit` → mint NFT, maybe initialise pool, deposit amounts.  
• `deposit` / `withdraw` → adjust liquidity.  
• `collectFees` → withdraw the fees the position earned.

Protocol-level fees are applied in two ways:
1. **Swap Protocol Fee** – A fraction (`SWAP_PROTOCOL_FEE_X64`) of every swap fee is skimmed before distribution to LPs.
2. **Withdrawal Fee** – When liquidity is removed, a small cut equal to the pool’s swap fee / `WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR` is taken from principal.

Collected protocol fees accumulate in Core’s till balances owned by the `Positions` contract itself.  `PositionsOwner` is a thin helper that can withdraw those balances and forward them directly into the `RevenueBuybacks` strategy.

---
## 4. Extensions In-Depth

### 4.1 Oracle
Produces cumulative `tick` and `seconds-per-liquidity` snapshots for the canonical `WETH/token` 0-fee, full-range pools.  Snapshots are stored in per-token circular buffers (ring-buffers), and the public functions let anyone extrapolate TWAPs without needing to touch Core state.  This data feeds on-chain pricing adapters (`ERC7726.sol`) and the analytics lens (`PriceFetcher.sol`).  The snapshot insertions happen opportunistically in three Core hooks:
* before pool initialisation
* before every swap
* before every position update

### 4.2 TWAMM (Time Weighted Automated Market Maker)
Implements Uniswap-v3-style long-term orders:
• **Order State** – saleRate, soldAmount, remainingQty, purchasedQty.  
• **Per-Pool State** – global sale rates for token0/token1, last execution time.  
On every relevant Core entry point the extension computes and settles *virtual trades* that would have occurred since the last sync (`_executeVirtualOrdersFromWithinLock`).  Users interact through two facades:
1. `Orders.sol` NFT manager (user-friendly).  
2. `TWAMMLib` helpers (contracts & lenses).

### 4.3 MEV Capture
Blocks direct calls to `Core.swap`; the router must forward via `handleForwardData`.  On each swap it charges extra fees proportional to the *absolute tick delta* realised in that swap – effectively making MEV takers pay more when they move the price.  The additional fees are credited to Core as if they were usual swap fees, funneling the value into protocol revenue.

---
## 5. Governance & Revenue Loop

1. Swap / withdrawal / MEV fees accumulate inside Core.
2. `PositionsOwner.withdrawAndRoll` pulls them into `RevenueBuybacks`.
3. `RevenueBuybacks` runs an *infinite TWAMM DCA* that converts every revenue token into the designated *buyback token* (`EKUBO`) by creating/rolling orders in the `Orders` contract.
4. Anyone can permission-lessly call `roll` and `collect` to keep the order evergreen and send purchased EKUBO to `owner()` (the DAO timelock).

No contract in the pipeline can steal user funds; only the excess fees that were **already** owed to governance are ever moved.

---
## 6. Lens & Data Fetchers

Because storage is highly packed and often accessed through raw assembly, a suite of read-only helper contracts makes it trivial for indexers or front-ends to query protocol state:

• `CoreDataFetcher` – fetch pool price, liquidity, positions, saved balances.  
• `QuoteDataFetcher` – pull deep tick-range liquidity and produce order-book-like quote data.  
• `TWAMMDataFetcher` – aggregate TWAMM pool + sale-rate deltas and can trigger syncs.  
• `IncentivesDataFetcher` – read Merkle airdrop claim status.  
• `TokenDataFetcher` – batch balances/allowances for wallets.

---
## 7. Security & Trust Boundaries

1. **User Funds** live only in `Core` (principal & fees) or the `FlashAccountant` *during* a lock.
2. **Extensions** cannot access arbitrary storage; they only mutate specific slots allotted to them and must go through call-points registered at deployment time.
3. **Re-entrancy** is prevented because Core relies on the *till* pattern – no external token transfers happen mid-operation, so no external code is executed while Core’s invariants are half-updated.
4. **Upgradeability** – all primary contracts are immutable; upgrades occur only by deploying new versions and migrating liquidity voluntarily.
5. **Time Locks & Governance** – the only mutable lever is protocol revenue withdrawal, controlled by Ekubo DAO timelock.

---
## 8. Contracts Cohorts & Responsibilities

| Layer | Main Contracts | Responsibility |
|-------|----------------|----------------|
| Core Ledger | `Core.sol`, `FlashAccountant.sol`, `ExposedStorage.sol` | custody, swaps, fee accounting, till pattern |
| LP NFTs | `Positions.sol`, `BasePositions.sol`, `PositionsOwner.sol` | mint/burn positions, compute protocol fees |
| Extensions | `Oracle.sol`, `TWAMM.sol`, `MEVCapture.sol` (+ libs) | plug-in behaviours |
| Routers & UX | `Router.sol`, `MEVCaptureRouter.sol`, `TokenWrapper*.sol` | high-level user flows |
| Revenue Loop | `RevenueBuybacks.sol`, `Orders.sol` | turn fees → buybacks |
| Peripheral | `Incentives.sol`, lenses in /lens, maths in /math | airdrops, analytics, helpers |

---
## 9. Example End-to-End Flow

1. **Alice** wants to market-make ETH/USDC between 2 100 and 2 200 USDC per ETH.  
2. She calls `Positions.mintAndDeposit`, which underneath:
   * Maybe initialises the pool (single SSTORE setup).
   * Locks the accountant, pushes ETH & USDC deltas, updates Core ticks & liquidity.
   * Mints her an ERC-721 representing the position.
3. **Bob** swaps 10 000 USDC for ETH through `Router.swap`.  The router batches his trade, as described earlier, against Alice’s liquidity and maybe many other pools.
4. Swap fees accumulate: LP share → claimable by Alice; protocol share → Core’s till balance.
5. At epoch end, anyone calls `PositionsOwner.withdrawAndRoll` which:
   * Withdraws protocol fees (USDC) from Core.
   * Sends them to `RevenueBuybacks`.
   * `RevenueBuybacks.roll` creates/extends a TWAMM order selling USDC to buy EKUBO over, say, 7 days.
6. After 7 days someone calls `RevenueBuybacks.collect`; the purchased EKUBO is forwarded to the DAO treasury.  Cycle restarts.

---
## 10. Lines of Defense Summary

1. **No un-vetted delegate-calls** – Extensions are *separate* contracts; Core only performs normal `call`s.
2. **Immutable critical params** – Swap fee, withdrawal fee, MEV fee logic hard-coded.
3. **Lock-based zero-transfer core** – eliminates observable re-entrancy vectors.
4. **Ring-buffer snapshot Oracle** – provides manipulaton-resistant pricing for on-chain consumers.
5. **Permissionless Upkeep** – Anyone can trigger Oracle snapshot, TWAMM sync, Revenue roll ensuring liveness even if governance is asleep.

---
## 11. Gas & Storage Optimisations

* Via-IR + 9 999 999 optimizer runs; EVM version `osaka`.
* Custom maths libraries (`sqrtRatio`, `exp2`, `liquidity`, …) written in unchecked assembly for minimal gas.
* Deterministic storage slots calculated in libraries (`CoreStorageLayout`, `TWAMMStorageLayout`) to avoid `keccak256` inside hot paths.
* CREATE2 factories (`TokenWrapperFactory`) for cheap predictable deployments.

---
## 12. Conclusion

Ekubo marries the *capital efficiency* of extremely tight concentrated liquidity with the *gas efficiency* of a singleton core and till-based deferred token transfers.  Its open extension model empowers developers to ship entirely new pool behaviours without fragmenting liquidity or sacrificing execution quality, while governance stays narrowly scoped to revenue handling.  For integrators the protocol presents an unusually low-level but **predictable** surface: one Core address, one Accountant address, deterministic storage, and a suite of helpful read-only lenses.  This makes Ekubo a compelling settlement layer for any on-chain application that needs deep liquidity with minimal gas overhead.



## Main List of Files in Project

src/Core.sol
src/Incentives.sol
src/MEVCaptureRouter.sol
src/Orders.sol
src/Positions.sol
src/PositionsOwner.sol
src/RevenueBuybacks.sol
src/Router.sol
src/TokenWrapper.sol
src/TokenWrapperFactory.sol
src/base/BaseExtension.sol
src/base/BaseForwardee.sol
src/base/BaseLocker.sol
src/base/BaseNonfungibleToken.sol
src/base/BasePositions.sol
src/base/ExposedStorage.sol
src/base/FlashAccountant.sol
src/base/PayableMulticallable.sol
src/base/UsesCore.sol
src/extensions/MEVCapture.sol
src/extensions/Oracle.sol
src/extensions/TWAMM.sol
src/interfaces/IBaseNonfungibleToken.sol
src/interfaces/ICore.sol
src/interfaces/IExposedStorage.sol
src/interfaces/IFlashAccountant.sol
src/interfaces/IIncentives.sol
src/interfaces/IOrders.sol
src/interfaces/IPositions.sol
src/interfaces/IRevenueBuybacks.sol
src/interfaces/extensions/IMEVCapture.sol
src/interfaces/extensions/IOracle.sol
src/interfaces/extensions/ITWAMM.sol
src/lens/CoreDataFetcher.sol
src/lens/ERC7726.sol
src/lens/IncentivesDataFetcher.sol
src/lens/PriceFetcher.sol
src/lens/QuoteDataFetcher.sol
src/lens/TWAMMDataFetcher.sol
src/lens/TokenDataFetcher.sol
src/libraries/CoreLib.sol
src/libraries/CoreStorageLayout.sol
src/libraries/ExposedStorageLib.sol
src/libraries/ExtensionCallPointsLib.sol
src/libraries/FlashAccountantLib.sol
src/libraries/IncentivesLib.sol
src/libraries/OracleLib.sol
src/libraries/RevenueBuybacksLib.sol
src/libraries/TWAMMLib.sol
src/libraries/TWAMMStorageLayout.sol
src/libraries/TimeDescriptor.sol
src/math/constants.sol
src/math/delta.sol
src/math/exp2.sol
src/math/fee.sol
src/math/isPriceIncreasing.sol
src/math/liquidity.sol
src/math/sqrtRatio.sol
src/math/tickBitmap.sol
src/math/ticks.sol
src/math/time.sol
src/math/timeBitmap.sol
src/math/twamm.sol
src/types/bitmap.sol
src/types/buybacksState.sol
src/types/callPoints.sol
src/types/claimKey.sol
src/types/counts.sol
src/types/dropKey.sol
src/types/dropState.sol
src/types/feesPerLiquidity.sol
src/types/locker.sol
src/types/mevCapturePoolState.sol
src/types/observation.sol
src/types/orderConfig.sol
src/types/orderId.sol
src/types/orderKey.sol
src/types/orderState.sol
src/types/poolBalanceUpdate.sol
src/types/poolConfig.sol
src/types/poolId.sol
src/types/poolKey.sol
src/types/poolState.sol
src/types/position.sol
src/types/positionId.sol
src/types/snapshot.sol
src/types/sqrtRatio.sol
src/types/storageSlot.sol
src/types/swapParameters.sol
src/types/tickInfo.sol
src/types/timeInfo.sol
src/types/twammPoolState.sol


 ## DOCUMENTATION: 

 ### ekubo-docs.md

# Introduction

Ekubo Protocol delivers the best pricing using *super-concentrated* liquidity, a singleton architecture, and extensions. The Ekubo protocol vision is to provide a balance between the best swap execution and liquidity provider returns. The contracts are relentlessly optimized to be able to provide the most capital efficient liquidity ever at the lowest cost.

# Features

## Gas efficiency

Ekubo uses the ["till" pattern](https://docs.ekubo.org/integration-guides/till-pattern) and a singleton design to provide the cheapest trades against many pools all featuring concentrated liquidity. That means all the pools are managed in a single contract, and when you swap against a pool or update a position on Ekubo Protocol, token transfers are deferred until the end of the transaction. In fact, you don't have to transfer tokens at all--advanced swappers could save those tokens in Ekubo Protocol for later, avoiding expensive token transfers and undesirable token behavior altogether.

The result is that you can execute many actions across many pools and only make the minimum number of required token transfers. The highly optimized and capital efficient design and ruthlessly optimized contracts enables Ekubo protocol to provide the best execution net of gas.

## Concentrated liquidity

Concentrated liquidity allows market makers to [provide liquidity](https://docs.ekubo.org/user-guides/add-liquidity) within a specified price range. Each liquidity provider chooses the exact parameters of their position, but all positions in a pool are aggregated from a swapper's perspective. As a result, swappers get better pricing because liquidity providers can leverage up within a price range, *or* earn yield on unused capital elsewhere.

Ekubo Protocol uses ticks 100x smaller than the competitors at 1/100th of a basis point. This allows $1,000 in liquidity in EKUBO to work as well as $100k in the next best AMM protocol.

## Extensions

Extensions allow third party developers to permissionlessly create new kinds of pools on Ekubo that integrate into the same ecosystem of aggregators and interfaces built on top of Ekubo. These pools can implement new features such as oracles, or additional order types like limit orders or TWAMM orders. Read more about extensions [here](https://docs.ekubo.org/integration-guides/extensions).

## Withdrawal fee

When you withdraw liquidity from Ekubo, you pay a fee equal to the swap fee of the selected pool from your principal. Because this fee is taken on principal:

* It *decreases* as a percentage of *liquidity* as capital efficiency increases
* It *decreases* as a percentage of `principal + fees` as fees are earned over time

Thus, the fee incentivizes **all of** liquidity concentration, passive liquidity and low fees.

{% hint style="info" %}
This fee is collected by the protocol, able to be withdrawn by the protocol's current `owner`.&#x20;

The owner of the protocol is [Ekubo Governance](https://docs.ekubo.org/user-guides/governance). As of January 2025, Ekubo Governance currently directs all protocol revenue towards [EKUBO buybacks](https://app.ekubo.org/governance/revenue-buybacks).




 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.9.7",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/solady/package.json

{
  "name": "solady",
  "license": "MIT",
  "version": "0.1.22",
  "description": "Optimized Solidity snippets.",
  "files": [
    "src/**/*.sol",
    "js/**/*"


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]
verbosity = 3
no_match_contract = "SlowTest"
evm_version = 'osaka'
# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options

optimizer = true
optimizer_runs = 9999999
via_ir = true
ignored_error_codes = [2394, 6321, 3860, 5574]

[invariant]
fail_on_revert = true
runs = 1

[profile.ci]
verbosity = 3

# [profile.ci.fuzz]
# runs = 100000
# max_test_rejects = 500000

# [profile.ci.invariant]
# runs = 128

[profile.superfuzz]
verbosity = 3

[profile.superfuzz.fuzz]
runs = 1000000
max_test_rejects = 1000000

[profile.superfuzz.invariant]
runs = 1024

[profile.all_ticks]
no_match_contract = "n/a"
verbosity = 0
gas_limit = "18446744073709551615"
match_test = "test_all_tick_values"
match_contract = "SlowTestAllTicksTest"

### remappings.txt

forge-std/=lib/forge-std/src/
solady/=lib/solady/src/


