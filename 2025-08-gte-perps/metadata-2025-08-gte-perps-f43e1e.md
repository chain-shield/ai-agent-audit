
## PROTOCOL OVERVIEW:

# GTE Protocol – End-to-End DeFi & Derivatives Trading Stack

> This document gives a security-oriented yet developer-friendly walkthrough (≈3 500 words) of the GTE on-chain trading stack whose codebase you just reviewed.  It covers **what each subsystem is for, how the pieces stitch together, and the life-cycle of tokens and collateral from genesis on the Launchpad all the way to perpetual-futures settlement and liquidity-pool exit**.

---

## 1. Architectural Big-Picture

GTE aspires to replicate the full CEX experience **entirely on-chain** while preserving DeFi composability:

* **Permissionless Launchpad** – projects or individuals create a new ERC-20 (`LaunchToken`) and bootstrap liquidity through a bonding-curve sale.  No pre-mine; 80 % of supply is sold on the curve, the remaining 20 % auto–seeds an AMM pool when the curve depletes.
* **Classic AMM layer (Uniswap V2 fork)** – as soon as the bonding-curve finishes, liquidity is parked in a customised `GTELaunchpadV2Pair` whose fees feed the launchpad’s `Distributor`.
* **Spot CLOB layer** – a Central-Limit-Order-Book (`CLOB`) for the freshly launched token (and every other supported spot pair) allows tight-spread price discovery once depth is adequate.
* **Perpetual Futures CLOB layer** – the flagship; high-throughput, price-time-priority matching with cross-/isolated-margin, full liquidations, insurance fund, maker/taker fee tiers, etc.
* **Best-Price Router** – `GTERouter` glues everything together; a user can chain hops (spot order-book fills, AMM swaps, launchpad buylinks) in a single tx.

All contracts are deployed on **MegaETH** – an EVM-compatible L2 boasting 100 k TPS and sub-10 ms latency, which is crucial for order-book viability.

```
┌──────────────────────────┐        ┌────────────────────────┐
│  ░░  User Wallet / dApp  │        │  Admin / Governance    │
└────────────┬─────────────┘        └───────────┬────────────┘
             │                                   │
             ▼                                   ▼
      ┌──────────────┐                ┌───────────────────┐
      │  GTERouter   │◄──────────────►│  AdminPanel       │
      └───────┬──────┘                └────────▲──────────┘
              │                                │
      (spot & perps ops)              (market params, fees)
              ▼                                │
 ┌─────────────────────────┐           ┌───────┴─────────┐
 │ AccountManager (spot)   │◄──────────┤  FeeManager     │
 └─────────┬───────────────┘           └─────────────────┘
           │ deposit/withdraw                      ▲
           ▼                                       │ fee accrual
 ┌─────────────────────────┐                      │
 │    PerpManager          │◄─────────────────────┘
 └─────────┬───────────────┘  collateral / margin
           │                                      
           ▼
 ┌─────────────────────────┐    ┌────────────────────────┐
 │  ClearingHouse          │    │  InsuranceFund         │
 └─────────────────────────┘    └────────────────────────┘
```  

---

## 2. Launchpad Flow (token genesis ➜ liquid AMM)

### 2.1 Players & Contracts

* **`Launchpad`** – orchestrator; takes an ETH fee, deploys the ERC-20, wires bonding-curve, Distributor, LP Vault.
* **`SimpleBondingCurve`** – keeps virtual **quote/base** reserves; sells tokens at an increasing price.
* **`LaunchToken`** – ERC-20 with transfer-lock until `unlock()`.  Each transfer before unlock adjusts *staking shares* that feed Distributor rewards.
* **`GTELaunchpadV2Pair` & Factory** – Uniswap V2 pair with a hard-wired *launchpad-fee share* siphoned to the Distributor.
* **`Distributor`** – stakes, tracks per-account shares via `RewardsTracker` and pays out both base & quote rewards.

### 2.2 Life-cycle

1. **`launch()`** (anyone): provides token name/metadata and pays `launchFee` ⇒
   * Launchpad deploys `LaunchToken`, mints 100 % supply to itself.
   * Splits supply: **80 %** flagged as *bonding supply* (escrowed in curve), **20 %** earmarked for LP.
   * Calls `SimpleBondingCurve.initializeCurve()` to set initial virtual reserves.
2. **Buyers call `buy()` through Launchpad** – Launchpad pulls USDC (or other `quoteAsset`) from the buyer’s *spot* balance (`AccountManager`) and calls `SimpleBondingCurve.buy()` to determine price.  Purchased tokens are *still locked* (cannot transfer except via Router) but accrue staking shares in Distributor.
3. **Graduation** – When the cumulative amount sold reaches the 80 % bonding supply:
   * Launchpad deploys a `GTELaunchpadV2Pair` via Factory.
   * Adds **20 %** of token supply + proportional quote liquidity from its own quote balance to seed LP.
   * Remaining quote or base from the triggering trade is swapped inside the new pair so the taker receives the exact requested output.
   * Calls `endRewards` on Distributor & pair, unlocking transfers.
4. **Post-Launch Trading** – Token is immediately tradable through
   * the AMM pair (Uniswap path),
   * spot CLOB once accountManager registers enough depth (`CLOBManager.createMarket`),
   * perps when AdminPanel whitelists it as a *market asset*.

### 2.3 Security/Pitfalls

* **No team prefunding** – 100 % of supply starts controlled by Launchpad; only bonding-curve buyers & LP receive tokens before unlock.
* **Price manipulation window** – Between unlock and deep CLOB listing the AMM could be shallow; the project relies on the pair’s swap fee + graduated depth to mitigate.
* **Distributor escrow** – `totalPendingRewards` ensures Admin can’t “skim” user-owed rewards.

---

## 3. Spot Layer (Account custody + CLOB)

### 3.1 Custodial Model – `AccountManager`

The protocol is **custodial** at the smart-contract level: users deposit ERC-20s which live under `AccountManager`.  Operator roles (granted via `OperatorPanel`) allow:

* deposit / withdrawal (spot),
* router deposit/withdraw (so multi-hop routes need only one approval),
* perps collateral transfers to `PerpManager`.

All balance-changing externals emit an **event nonce**, enabling indexers to checkpoint state.

### 3.2 Central Limit Order Book – `CLOB`

* **Storage layout** is library driven (`BookLib`, `RedBlackTree`) with one struct per market containing red-black trees for bids/asks, linked-list limits, and the `orders` mapping.
* **Order id** encodes `(owner address | 96-bit client id)` – prevents collisions across users.
* **Matching** happens *inside the tx that places the order* (no external cranker).  The taker loop iterates ask/bid tree until the incoming amount is filled or price condition breaks.
* **Transient maker credits** – while matching, quote/base deltas owed to makers are written to a transient storage area; they are bulk-settled via `AccountManager.settleIncomingOrder` once the taker side finishes.  This guarantees **gas safety** (constant refund complexity) and avoids re-entrancy on ERC-20 callbacks.
* **Fee path** – maker & taker fees fetched from `FeeManager` (packed `PackedFeeRatesLib`) and immediately credited to `AccountManager`.*

### 3.3 Admin / Governance knobs – `CLOBManager`

Owner or addresses with role **`MARKET_MANAGER`** can deploy new spot markets through a Beacon-proxy saving bytecode.  Other special roles:

* `MAX_LIMIT_WHITELISTER` – bypass per-tx limit order caps (for market makers).
* `EXPIRED_ORDER_CLEARER` – gas-efficient batch cancellation to prevent DoS with unfillable dust orders.

---

## 4. Perpetual Futures Layer

### 4.1 Storage Topology (`StorageLib`)

Instead of monolithic contract storage, every logical module gets its own **EIP-7201 style slot** – e.g. `MARKET_SETTINGS_SLOT`, `BOOK_SETTINGS_SLOT`, etc.  All libraries (`MarketLib`, `CollateralManager`, `InsuranceFundLib`) receive a storage pointer via `StorageLib.*load*()` which means:

* Upgrade safety – new modules can be added without changing existing structs.
* Gas – direct `sload` addresses in assembly avoid double-mapping lookups.

### 4.2 Collateral & Margin – `CollateralManager`

* Users deposit free USDC collateral (unsigned).  Each sub-account has an **int256 `margin`** which can go negative (bad debt scenario) and is reconciled with `freeCollateral` on trade/funding events.
* **Invariant** highlighted in docs:  
  `Σ freeCollateral + Σ margin + insuranceFund == PerpManager.usdcBalance` – every entrypoint that transfers USDC adjusts exactly one of these three terms.

### 4.3 Markets and Matching – `MarketLib` & `CLOBLib`

* Each perps asset owns **two books**: `STANDARD` and `BACKSTOP` (wider margin, for emergency liquidations).
* **Reduce-Only link** – to guarantee liquidations always succeed, reduce-only orders are tracked in arrays whose size is capped by `MarketSettings.reduceOnlyCap`; posting opposite-direction orders larger than position size is rejected.
* **Funding** – `FundingRateEngine` runs *capped interest + basis spread* model: 
  * Admin sets `interestRate`, clamps, intervals.
  * Oracle provides `indexPrice` (spot) via Admin; `markPrice` is driven by book impact + funding component.
  * On each settlement funding index is added to `cumulativeFundingIndex`; positions realize payment lazily when touched.

### 4.4 Clearing House

`ClearingHouseLib` glues books, margin, fee and funding together.

* `placeOrder` routes to `CLOBLib.placeOrder` then adjusts margin according to maker/taker fills and fees.
* **Cross-margin** – when enabled, `rebalanceAccount` prorates global margin across all assets based on notional weight.  This allows capital efficiency (only one “free collateral pot” per account).
* **Liquidations** – `LiquidatorPanel` checks `isLiquidatable` (maintenance margin ratio) and posts a **reduce-only IOC** into either standard or backstop book.  Post trade:
  * liquidation fee is paid by liquidated margin → insurance fund → liquidator reward.
  * If margin < 0, remainder is *bad debt* satisfied from insurance fund.

### 4.5 Insurance Fund

Simple contractless storage struct controlled by `InsuranceFundLib`.

* **Credited** by trading fees & liquidation fees.
* **Claimed** during bankruptcy (bad debt) events.
* Governance (AdminPanel owner) can top-up or withdraw – but withdrawals will fail if they would violate the core invariant previously stated.

### 4.6 Liquidity Pool Token – `GTL`

To attract passive liquidity for perps, GTE smart-wraps USDC in `GTL` (ERC-4626).  Design choices:

* Shares are non-freely redeemable; users **queue withdrawals** (`queueWithdrawal`) and an admin batch (`processWithdrawals`).
* `PerpManager` registers GTL-owned sub-accounts to compute `totalAssets()`.
* Admins can grant `ADMIN_ROLE` (bitmask) to market-makers so they can process withdrawal queue but can’t steal funds (because shares are burned pro-rata).

---

## 5. Operator-Permissioning Model

Every user can approve **granular operator roles** (bitmask) via `OperatorPanel`:

* Spot roles (`SpotOperatorRoles`): deposit, withdraw, launchpad fill, CLOB order placement etc.
* Perps roles (`PerpsOperatorRoles`): margin ops, leverage set, order placement, etc.

`OperatorHub` is a convenience facade so front-ends need only one contract call to approve both spot & perps operators.

On every sensitive function `onlySenderOrOperator(...)` checks:

```
if (msg.sender == account)               OK
else if (msg.sender == gteRouter)        OK (for UX batch ops)
else   assert operatorRoleBits & requiredRole != 0
```

No operator can escalate privileges: bitmasks are fixed constants shipped in `Enums.sol`.

---

## 6. Governance & Upgradability

* *Governance addresses* (owner / roles) live in **AdminPanel**, **CLOBManager** and **Launchpad** contracts.  Those are `OwnableRoles` upgrade-safe patterns from OZ Upgradeable 5.1.
* Critical contracts (CLOB markets, AccountManager, PerpManager) **are NOT proxies**; they rely on library-driven storage slots.  Any upgrade would deploy new versions and migrate balances at the orchestrator layer.
* `GTELaunchpadV2PairFactory` and `CLOBManager` use **BeaconProxy** or `CREATE2` deterministic addresses so front-ends can predict pool/market addresses before deployment.

---

## 7. External Integrations & Libraries

* **OpenZeppelin Upgradeable 5.1** – ERC-20, 4626, OwnableRoles.
* **Solady 0.0.287** – `RedBlackTreeLib`, `SafeTransferLib`, storage utilities.
* **Uniswap Permit2** – Gas-efficient token approvals inside `GTERouter`.
* **Foundry Forge-Std** – only in tests.

Upgrade paths rely on OZ’s Initializable pattern; constructors call `_disableInitializers()`.

---

## 8. Threat Model Highlights

1. **Economic bad debt** – ensure every code path that reduces margin also decreases open interest or uses insurance fund.
2. **Orderbook DoS** – `maxNumOrders`, per-tx cap and `adminCancelExpiredOrders` mitigate storage-bloat griefing.
3. **Launchpad graduation race** – `_checkGraduation` caps last buyer to remaining bonding supply and then performs AMM swap atomically.
4. **Re-entrancy** – Almost all external-token transfers go through `SafeTransferLib` **after** state mutations; major entrypoints are marked `nonReentrant` where pull-based patterns exist (Launchpad `buy` / `sell`, Router multihop).
5. **Operator hijack** – approval events include `eventNonce` for off-chain indexing; users can monitor rogue approvals in real time.

---

## 9. Typical User Journeys

### 9.1 Launchpad Investor
1. Deposit USDC into `AccountManager` through Router (one Permit2 signature).
2. Call `launchpadBuy()`; tokens credited & staking started.
3. After graduation: claim Distributor rewards, freely transfer tokens or trade on spot/perps.

### 9.2 Perps Trader
1. Deposit USDC collateral; optionally set an operator (e.g. a bot).
2. `PerpManager.placeOrder()` – fills instantly if crossing; leftover posts to book.
3. As funding accrues, `addMargin/removeMargin` or `setPositionLeverage` as risk control.
4. Close or get liquidated – margin / PnL settled back to free collateral; withdraw to spot and then back to L1 if desired.

### 9.3 Passive Liquidity Provider
1. Deposit USDC to `GTL` – receive shares.
2. Wait; `totalAssets()` grows from PM captures → share price up.
3. Queue withdrawal; admin batches; receives USDC + yield.

---

## 10. Conclusion

GTE delivers a **vertically integrated** on-chain trading venue:

* **Fair-launch token issuance** with built-in community rewards.
* **Efficient spot trading** through both AMM and CLOB, achieving price-time priority.
* **High-performance perps** with real central-limit mechanics, robust risk engine and modular governance.
* **Unified routing UX** along with fine-grained operator delegation for automation.

From a security perspective the design splits fund custody into specialised modules (`AccountManager`, `CollateralManager`, `InsuranceFund`) and enforces balance-sheet invariants.  Admin powers are explicit and scoped; the biggest centralisation points are Launchpad owner discretion, AdminPanel parameter tuning, and the withdrawal queue admin in `GTL`.  Reviewers should focus on economic edge-cases (funding, bad-debt, partial liquidations) and storage slot correctness across libraries.



## Main List of Files in Project

contracts/launchpad/BondingCurve.sol
contracts/launchpad/BondingCurves/IBondingCurveMinimal.sol
contracts/launchpad/BondingCurves/SimpleBondingCurve.sol
contracts/launchpad/Distributor.sol
contracts/launchpad/LaunchToken.sol
contracts/launchpad/Launchpad.sol
contracts/launchpad/LaunchpadLPVault.sol
contracts/launchpad/libraries/RewardsTracker.sol
contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol
contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol
contracts/launchpad/uniswap/interfaces/IGTELaunchpadV2Pair.sol
contracts/launchpad/uniswap/interfaces/IUniswapV2Router01.sol
contracts/perps/GTL.sol
contracts/perps/PerpManager.sol
contracts/perps/modules/AdminPanel.sol
contracts/perps/modules/LiquidatorPanel.sol
contracts/perps/modules/ViewPort.sol
contracts/perps/types/BackstopLiquidatorDataLib.sol
contracts/perps/types/Book.sol
contracts/perps/types/CLOBLib.sol
contracts/perps/types/ClearingHouse.sol
contracts/perps/types/CollateralManager.sol
contracts/perps/types/Constants.sol
contracts/perps/types/Enums.sol
contracts/perps/types/FeeManager.sol
contracts/perps/types/FundingRateEngine.sol
contracts/perps/types/InsuranceFund.sol
contracts/perps/types/Market.sol
contracts/perps/types/Order.sol
contracts/perps/types/PackedFeeRatesLib.sol
contracts/perps/types/Position.sol
contracts/perps/types/PriceHistory.sol
contracts/perps/types/StorageLib.sol
contracts/perps/types/Structs.sol


 ## DOCUMENTATION: 

 ### gte-docs.md

# Overview

## Launchpad

Permissionless token launcher and project token launchpad.

### Permissionless Token Launcher

The GTE token launcher is a permissionless system that allows anyone to boostrap liquidity to launch a token on GTE. Launches are fair, meaning that no tokens will be available for purchase by the team beforehand. The flow of launching a new long-tail asset is as follows:

- 80% of the token supply will be traded on a bonding curve, and when a token hits the bonding price, a liquidity pool will automatically be deployed on the GTE AMM seeded with 20% of the supply reserved from the bonding curve.
- After a launched token bonds and gets its own liquidity pool, the token will be immediately tradeable in the DEX aggregator frontend.
- After a token reaches sufficient maturity and market depth, it will be automatically added to the GTE CLOB platform.

### Project Token Launchpad

The GTE Token Launchpad addresses the growing skepticism around CEX listings, which are often expensive and lack transparency in price discovery. Unlike CEXs, GTE partners with projects on MegaETH to launch tokens onchain through our token launchpad and across our trading venues.

The launchpad facilitates the creation of fully onchain token vaults, enabling token sales to the GTE community. Upon a sale, tokens are locked in a stake vault. Users who hold their staked tokens for longer periods receive more tokens at the time of vault unlock. Additionally, GTE receives a portion of the initial supply dedicated to the launchpad.

This process is conducted in a fully compliant manner, with partnerships in place to provide necessary KYC, ensuring protection for both GTE and its users.

## Perps CLOB

GTE onchain Central-Limit Order Book

### What is a CLOB?

The order book is an exchange design that resembles traditional finance. For any given asset pair, an order book maintains a bid and ask side – each one being a list of buy and sell orders, respectively. Each order is placed at a different price level, called a limit, and has an order size, which represents the amount of the trade asset that the order wants to buy or sell. Order books use an algorithmic matching engine to match up buy and sell orders, settling the funds of orders that fulfill each other. Most order books use “price-time priority” for their matching engines, meaning that the highest buy offers and lowest sell offers are settled first, followed by the chronological sequence of orders placed at that limit price.

### Perps
GTE leverages its high-performance infrastructure to offer Central Limit Order Books for both major market types, with perpetual futures being the focus of this contest:

- Perpetual Futures CLOB: For trading derivatives contracts that mimic spot prices without an expiry date, allowing for leverage and hedging strategies.


# GTE Protocol Overview (Security-Oriented Summary)

## Core Components

* **Token Launchpad & Launcher**
  Fully permissionless — enables fair liquidity bootstrapping via bonding curves, automatically spawning an AMM pool and enabling immediate trading.
* **Classic AMM**
  Facilitates price discovery for new and niche tokens.
* **Spot & Perpetual CLOB (on-chain order book)**
  Runs centralized exchange–style matching on-chain using a crankless design — meaning orders are matched seamlessly with high frequency.
* **Best-Price Aggregator**
  Routes trades across AMM, CLOB, and potentially other MegaETH venues to ensure optimal pricing.

All of this is brought together under one roof—launch, price discovery, live trading, perpetuals—in a streamlined, low-friction interface.

## MegaETH: The Foundation

* **EVM-compatible L2** with a real-time sequencer, enabling parallel execution, and integrated with EigenDA for robust data availability.
* Capable of **100,000 TPS** and **single-digit millisecond latency** — setting the stage for on-chain order books that match CEX speeds.
* **Low gas cost** facilitates frequent order cancellation and resubmission without penalty — ideal for market maker strategies and tight spread regimes.
* **Price-time priority matching** mirrors traditional trading fairness models, encouraging pro traders.
* Fully composable within the Ethereum DeFi landscape, thanks to EVM compatibility.

## **MAIN INVARIANTS**
## ************************************************************************
We define the PerpManager's USDC balance as:

$$
\sum^{total\_users}_{i=0}{user\_free\_collateral\_balance[i]} + \sum^{total\_users}_{i=0}{user\_margin\_balance[i]} + insurance\_fund\_balance
$$
## ************************************************************************

## **AREAS OF CONCERN ==> PAY EXTRA ATTENTION TO BELOW TO WHEN BUG HUNTING**

### Economical Vulnerabilities

Economical attacks (e.g. engaging large amounts of tokens to break the platform or profit from it) are considered a valid attack vector; we encourage Wardens to look out for ways to generate "bad debt", e.g. negative equity, in a way that would result in net-positive gains for the attacker, or make the platform illiquid. ADL (Auto-De-Leverage) abuses that result in monetary gains for the attacker as well as any attack that would result in loss of funds for other users, themselves or the Platform's funds, making it illiquid, are of particular interest to us. To note, *fund loss of oneself must result from an inadvertent action to be considered a valid vulnerability and must not arise from deliberate misuse of the platform*.

### Orderbook Denial-of-Service

The Orderbook should be able to process orders at all times; we invite the wardens to look for attacks that would result in a Denial Of Service (e.g. placing an order that cannot be cleared by the ClearingHouse), or that bypasses the limit of orders a user can place in one transaction.


This should always be equal to or less than the PerpManager's USDC Token balance.





 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.9.5",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/openzeppelin-contracts-upgradeable/contracts/package.json

{
  "name": "@openzeppelin/contracts-upgradeable",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.1.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts-upgradeable/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.1.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts-upgradeable/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true
}

### lib/permit2/package.json

{
  "name": "@uniswap/permit2",
  "description": "Low-overhead, next generation token approval/meta-tx system to make token approvals easier, more secure, and more consistent across applications",
  "version": "1.0.0",
  "bugs": "https://github.com/Uniswap/permit2/issues",
  "keywords": [
    "ethereum",
    "permit2",

### lib/solady/package.json

{
  "name": "solady",
  "license": "MIT",
  "version": "0.0.287",
  "description": "Optimized Solidity snippets.",
  "files": [
    "src/**/*.sol",
    "js/**/*"

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.1.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.1.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true
}


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
evm_version = 'cancun'
libs = ["lib"]
optimizer = true
optimizer-runs = 200
out = "out"
solc_version = '0.8.27'
src = "contracts"
test = 'test'
via_ir = false


[rpc_endpoints]
testnet = "${RPC_TESTNET}"


[fmt]
single_line_statement_blocks="single"
multiline_func_header="attributes_first"
contract_new_lines=false
sort_imports=false
override_spacing=true
line_length=120
tab_width=4
int_types="long"
quote_style="double"
number_underscore="thousands"
hex_underscore="remove"
wrap_comments=false
ignore=["script/", "test/"]


### remappings.txt

@openzeppelin=lib/openzeppelin-contracts/contracts
@openzeppelin-contracts-upgradeable=lib/openzeppelin-contracts-upgradeable/contracts
@solady=lib/solady/src/
@permit2=lib/permit2/src/
forge-std=lib/forge-std/src/
@gte-univ2-core=lib/gte-univ2/src/core/
@gte-univ2-periphery=lib/gte-univ2/src/periphery/

