
## PROTOCOL OVERVIEW:

Ekubo Protocol is a next–generation automated-market-maker (AMM) built around three design pillars: 1) a singleton core that holds every liquidity pool in one contract, 2) an aggressive “till” (deferred-transfer) accounting model that minimises token transfers, and 3) a powerful, plug-and-play extensions system that lets anyone bolt new pool logic—TWAMM, MEV-aware fees, on-chain oracles, airdrop incentives—onto the same liquidity without fragmenting UI or aggregator support.

----------------------------------------------------
High-level architecture
----------------------------------------------------

1. Core.sol – the heart of the system.  
   • Custodies **all** pool assets.  
   • Maintains canonical state: price (sqrtRatio, tick), liquidity, fees-per-liquidity, tick bitmaps, positions, and per-user saved balances.  
   • Implements swaps (swap_6269342730), liquidity updates, fee collection, and extension hooks.  
   • Is itself upgrade-less; new functionality comes through registered extensions.

2. FlashAccountant.sol – a transient, flash-loan-style accountant that guards each user interaction.  
   • A call acquires a lock; during the lock the accountant tracks per-token debt in transient storage.  
   • Real ERC-20 / ETH transfers happen only once, when debt settles, so a user can touch many pools and extensions yet pay each token gas cost only once.  
   • If any debt remains on exit the whole tx reverts, guaranteeing Core is solvent.

3. BaseLocker / BaseForwardee – thin wrappers that let higher-level contracts (Routers, Extensions, TWAMM Orders, TokenWrapper, …) execute their logic *inside* an accountant lock or have the accountant forward arbitrary calldata while maintaining accounting guarantees.

4. Extensions – standalone contracts that declare, at compile time, which Core hook call-points they wish to receive. Core verifies those call-points on registration and then executes them before/after the corresponding action.  
   • Oracle.sol  – keeps a ring-buffer of time-weighted price/liquidity snapshots used for TWAP, analytics, risk management and cross-pair quoting.  
   • TWAMM.sol  – adds long-term, time-weighted market orders. It intercepts swaps and position changes to execute virtual orders, tracks pool-wide sale rates, and stores per-order state in deterministic slots.  
   • MEVCapture.sol – levies an additional dynamic fee proportional to the tick movement realised inside the block, diverting the extra value to protocol revenue.  
   Extensions are permission-less: anyone can deploy and register a new one, provided its bytecode advertises a non-colliding set of call-points.

5. Higher-level UX contracts
   • Router.sol – multi-hop swap router that bundles token transfers via FlashAccountant, supports ETH, enforces slippage, and can batch many swaps.  
   • MEVCaptureRouter.sol – same interface as Router, but auto-detects MEV pools and routes them through the MEVCapture extension path.  
   • Positions.sol (+ BasePositions) – NFT-based LP positions with protocol fee logic baked in. Liquidity providers mint or transfer NFTs that map 1:1 to Core positions; withdrawal incurs a small protocol fee equal to the pool swap fee.  
   • Orders.sol – NFT container around TWAMM orders. Lets users permissionlessly spin up long-term DCA orders, adjust sale rate, and collect proceeds.  
   • TokenWrapper (+ Factory) – time-locked wrappers (e.g. gEKUBO-2026Q1) that integrate with till accounting; unwrap is prohibited until a preset timestamp.  
   • Incentives.sol – Merkle airdrop distributor; manages many drops in one mapping-less layout.  
   • RevenueBuybacks.sol – trust-minimal contract that converts the protocol’s revenue into BUY_TOKEN via TWAMM orders.  
   • PositionsOwner.sol – forwards protocol fees from Positions to RevenueBuybacks and maintains admin ownership of the Positions contract.

6. Lens contracts – completely read-only helpers for off-chain UIs and analytics.  They use low-level storage-slot math to read Core/extension state without extra gas on-chain.  Examples include CoreDataFetcher, QuoteDataFetcher, TWAMMDataFetcher, IncentivesDataFetcher, and ERC7726 (price oracle adapter).

----------------------------------------------------
Till pattern & saved balances
----------------------------------------------------

Traditional AMMs push/pull tokens on every swap or liquidity change. Ekubo defers those transfers: each locker maintains *saved balances* inside Core storage (mapping key: locker → token pair → salt). After a swap the locker’s debt for token0/token1 is updated, but the actual ERC-20 transfer only happens once, when the accountant settles at the end of the user’s batched interaction. Pros:
•  Amortises gas over many actions.  
•  Allows power users (keepers, aggregators) to keep float inside Ekubo, avoiding transfers entirely.  
•  Makes multi-hop routing almost as cheap as a single swap.

----------------------------------------------------
Pool model
----------------------------------------------------
•  Concentrated liquidity identical in spirit to Uniswap V3, but with **100× smaller tick spacing (0.0001 bp)**, making capital 100× more efficient.  
•  All pools live under one contract; poolId is a hash of (token0, token1, fee, tickSpacing, stableFlag, …).  
•  Tick bitmaps, liquidity nets/deltas, global fees-per-liquidity, and per-position checkpoints are packed into deterministic storage slots computed by CoreStorageLayout to save keccak costs.
•  Withdrawal fee: when liquidity is burned the user’s principal is shaved by the pool’s swap fee; this naturally aligns LPs with low-fee, highly concentrated markets.

----------------------------------------------------
Swap execution path
----------------------------------------------------
1. Caller invokes Router.swap (or MEVCaptureRouter.swap).  
2. Router grabs accountant lock; accountant stores current locker address (Router) in transient storage.  
3. Router forwards to Core.swap, passing along PoolKey, amount, price limit, and skipAhead hint.  
4. Core.swap:  
   a. Runs any registered extension.beforeSwap hooks.  
   b. Traverses ticks until price limit hit, applying fee, updating liquidity, global fees, and bitmap.  
   c. Runs extension.afterSwap hooks.  
   d. Returns PoolBalanceUpdate (Δtoken0, Δtoken1) so Router can know net owed.  
5. Router accumulates per-hop deltas, possibly chains further hops, then exits accountant lock.  
6. Accountant sees that router owes X of tokenA, receives Y of tokenB, nets pre-existing saved balances, performs *at most one* ERC20/ETH transfer per distinct token, and finally zeroes all debts.  
7. Any leftover native ETH in Router is refunded via PayableMulticallable.refundNativeToken.

----------------------------------------------------
Extensions in action – TWAMM walkthrough
----------------------------------------------------
•  TWAMM extension registers for: afterInitializePool (to assert full-range), beforeSwap, beforeUpdatePosition, beforeCollectFees, and a custom locked callback ID.  
•  A user creates a TWAMM order through Orders.mintAndIncreaseSellAmount → accountant.lock → forward to TWAMM.handleForwardData (callType 0).  
•  TWAMM stores order state in deterministic slot, updates pool-wide sale rates and time-point bitmaps.  
•  On every subsequent pool activity, Core invokes TWAMM.beforeSwap / beforeUpdatePosition … which calls _executeVirtualOrdersFromWithinLock.  That function:  
   1. Scans the future-time bitmap, stepping through 30-minute bins until current block time.  
   2. Calculates amount_to_swap = saleRate * Δt per side.  
   3. Calls Core.swap *again*, but with opposite tokens, to faithfully execute the virtual trade inside the same lock (no external transfers!).  
   4. Updates reward-rate accumulators so each order’s purchased amount can be computed lazily.  
•  When the order owner later calls collectProceeds, Orders forwards to TWAMM.handleForwardData (callType 1), which computes purchasedAmount = saleRate × (rewardRateInside − snapshot) and credits it to the locker’s saved balance; accountant withdraws it to the recipient.

----------------------------------------------------
MEV Capture extension
----------------------------------------------------
Goal: extract part of the price impact value the swapper leaves on-chain and route it to protocol revenue.

Algorithm per swap:  
1. Swap *must* be invoked via MEVCaptureRouter which uses Core.forward (BaseForwardee) so swaps arrive inside MEVCapture.handleForwardData.  
2. MEVCapture loads lastUpdateTime and tickLast from its per-pool slot.  
3. feeExtra = poolFee × |Δtick| / tickSpacing, capped at uint64.  
4. For exact-in swaps the swapper gets fewer output tokens; for exact-out they must pay more input.  
5. The extension calls CORE.accumulateAsFees so the extra is added to protocol fees, not LP fees.  
6. Once per block anyone (or other hooks) can call accumulatePoolFees to forcibly settle the block’s delta if no swap touches the pool.

----------------------------------------------------
Security & trust assumptions
----------------------------------------------------
•  **No upgradeability** – Core and extensions are immutable once deployed.   
•  **No protocol admin over user funds** – governance touches only protocol-fee withdrawals and extension registration (which cannot steal funds because Core guards invariants and extensions never receive custody).   
•  **FlashAccountant debt-zero invariant** – protects against re-entrancy and partial settlement.   
•  **Deterministic storage layouts** avoid collisions among many extensions reading Core storage.

----------------------------------------------------
Gas design highlights
----------------------------------------------------
1. *Singleton pools* + *till pattern* eliminate `transferFrom` per pool per swap.
2. Storage is laid out linearly off poolId so computing a slot costs a couple of arithmetic ops instead of keccak256.
3. Extensions flag which hooks they need via bit-encoded call-points; Core can JIT decide to call them or skip (saves two dynamic calls per hook per extension).
4. Low-level assembly throughout (balance packing, unchecked loops, direct EVM opcodes for logs) keeps runtime bytecode lean and gas cheap.

----------------------------------------------------
Economic model & governance knobs
----------------------------------------------------
•  Swap protocol fee (x/2^64) and withdrawal fee denominator are baked into Positions at deploy—immutable, no rug risk.  
•  Governance (Ekubo DAO) currently directs all protocol revenue, accumulated as FeesPerLiquidity inside pools, to an on-chain buyback via RevenueBuybacks → TWAMM → BUY_TOKEN.  
•  Time-locked TokenWrappers (e.g., gEKUBO-2026Q1) enable governance vesting or bond products without writing bespoke staking logic.

----------------------------------------------------
Developer ergonomics
----------------------------------------------------
•  Lens contracts expose all state off-chain with **zero extra gas** on main contracts.  
•  Library helpers (CoreLib, OracleLib, TWAMMLib, IncentivesLib) give integrators pure/view calls to compute deltas without touching storage twice.  
•  foundry.toml is configured for `via_ir = true` and `optimizer_runs = 9,999,999`, targeting the Osaka EVM to squeeze every last gas unit.

----------------------------------------------------
Conclusion
----------------------------------------------------
Ekubo combines Uniswap-V3-style concentrated liquidity with Uniswap-v4-style singleton pools and an extensibility model reminiscent of hooks—yet it is live today, not a draft. Its aggressive gas optimisation and till accounting make it the cheapest place to trade or LP, while extensions like TWAMM, Oracle, and MEVCapture show how new functionality can be layered on without forking liquidity. Users benefit from better pricing, LPs earn more due to higher capital efficiency, and developers gain a modular platform to innovate on exotic orders, dynamic fees, oracles, lending integrations, and beyond.


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

solc = './bin/solc-macos'
optimizer = true
optimizer_runs = 9999999
via_ir = true
ignored_error_codes = [2394, 6321, 3860, 5574]

[invariant]
fail_on_revert = true
runs = 1

[profile.ci]
verbosity = 3
solc = '0.8.31-pre.1'

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


