
## PROTOCOL OVERVIEW:

# Panoptic Protocol – End-to-End Overview

Panoptic is a permissionless, non-custodial options protocol that lets anyone create, trade, and settle **perpetual American-style options** on top of any Uniswap V3 or V4 liquidity pool.  Rather than building a new AMM from scratch, Panoptic re-uses Uniswap’s concentrated-liquidity engine and wraps it with a thin but powerful layer of accounting, collateral management, and risk control.  The result behaves like a fully-collateralised options clearing-house that settles in a single Ethereum transaction, has no counter-party risk, and retains the deep liquidity and price discovery of the underlying Uniswap pool.

---

## 1.  Core Idea

*  **Options = Liquidity Chunks.**  A Uniswap liquidity position that is _added_ (short option) or _removed_ (long option) from the pool inside a narrow price band produces cash-flows identical to writing or buying an options contract.  By allowing users to add _and_ remove liquidity inside the same ERC-1155 token, Panoptic can represent multi-leg strategies such as spreads, strangles, or condors in a single `tokenId`.
*  **Perpetual & Instant Settlement.**  Positions live on-chain as ERC-1155 IDs.  They can be minted, burned, force-exercised, or liquidated at any block.  When a user’s account loses solvency the protocol has everything it needs on chain to liquidate immediately—no oracles, order books, or counterparties required.
*  **Under-Collateralised Sellers, Over-Collateralised Protocol.**  Liquidity posted by passive vault depositors (Panoptic Liquidity Providers, **PLPs**) is re-hypothecated to short-option writers.  Borrow rates grow with utilisation and are paid back to PLPs.  A sophisticated Risk Engine guarantees that the vaults themselves are always net-positive so the system remains fully collateralised even if individual sellers default.

---

## 2.  Contract Topology

There is **one semi-fungible engine (`SemiFungiblePositionManager`, SFPM)** that speaks directly to Uniswap pools and an isolated **option clearing-house (`PanopticPool`)** per Uniswap pair.  Each pool instance owns:

*  **RiskEngine** – math-only brain for collateral, solvency, liquidation bonus, adaptive interest rate, price oracle and safe-mode logic.  Holds no funds.
*  **CollateralTracker 0 / 1** – ERC-4626 vaults (one per underlying asset) that custody user deposits, track shares, pay interest/commissions, and supply borrowed liquidity to shorts.
*  **PanopticPool** – orchestrator.  All user interactions go through the `dispatch()` entry point which fans-out to SFPM, CollateralTrackers, and the RiskEngine.

```
User ↔ PanopticPool ↔ SFPM      (Uniswap pool interaction)
     ↘            ↘
      ↘            CollateralTracker0 (token0 vault)
       RiskEngine
        ↘
         CollateralTracker1 (token1 vault)
```

---

## 3.  SemiFungiblePositionManager (SFPM)

*  **ERC-1155 Tokens as Strategies.**  A 256-bit `tokenId` packs up to four legs (strike, width, long/short flag, token type, ratio, cross-references, etc.).
*  **Mint & Burn.**  On mint, SFPM calls `uniswapV3Pool.mint()` (or the v4 unlock flow) to add/burn liquidity, collects fees, and sends any in-the-money token delta back to the caller.
*  **Swaps for Netting.**  If a multi-leg position moves only one token in aggregate, SFPM performs an internal swap so that the user can open the position with a single asset.
*  **No Transfers.**  `safeTransferFrom` is disabled → positions are non-transferable.  This keeps accounting simple: holdings = owner’s risk.
*  **Public Good.**  Although Panoptic relies on SFPM, any Uniswap LP can use it as a gas-optimised replacement for the canonical NonFungiblePositionManager.

---

## 4.  CollateralTracker (ERC-4626 Vault)

Each tracker is a share vault for **one** of the pair tokens.

*  **Deposits & Withdrawals.**  Anyone can `deposit()` tokens and receive shares.  Outgoing transfers are blocked when the owner has open positions to avoid dangling risk.
*  **Borrow Accounting.**  Shorts borrow vault liquidity.  Borrow index compounds every block at a rate supplied by RiskEngine’s PID-style interest-rate model (min ↔ max APR bounded, target utilisation driven).
*  **Commissions.**  Every mint pays a notional fee; every burn pays a premium fee.  Fees are split between protocol, builders (referrals), and PLPs.
*  **Delegation.**  When a long option has value _above_ the rehypothecation threshold its implicit collateral is materialised as “virtual” shares and delegated to the borrower so that the position can partially self-collateralise.
*  **Liquidation Flow.**  If RiskEngine declares an account insolvent a keeper can liquidate.  Tracker mints shares to the liquidator equal to collateral seized + bonus and socialises any shortfall across vault shareholders.

---

## 5.  RiskEngine

The protocol’s single source of truth for **price, utilisation, and collateral maths**.

1. **Oracle Pack** – internal 8-sample median ring buffer + four EMAs; updated lazily when someone calls `PanopticPool.pokeOracle()` or any price-sensitive action crosses an epoch boundary.
2. **Safe Mode.**  Deviations between EMAs or vs. spot trigger graduated safe-mode levels.  Higher level → higher collateral ratios, lower leverage, restricted operations.  A guardian can also forcibly lock/unlock a pool.
3. **Collateral Requirements.**  Short/long base ratios scale with utilisation and VEGOID parameter.  Multi-leg positions get spread/strangle credits; cross-asset collateral allowed with buffer.
4. **Liquidation Bonus.**  Formula pays liquidator bonus plus possible protocol loss; haircut reins in excess rewards.
5. **Force-Exercise Cost.**  Out-of-range longs that drag on seller capital can be exercised by anyone willing to pay an exponentially decaying fee to the long holder.
6. **Interest-Rate Model.**  PID-like controller moves `rateAtTarget` toward min or max depending on utilisation; borrow index in each vault compounds per-second.
7. **Guardian Utilities.**  Can sweep stray tokens and lock pools but cannot touch user collateral.

RiskEngine never touches tokens; it only **reads** vault state and pool price, then returns pure numbers.

---

## 6.  PanopticPool – The Conductor

`dispatch()` is the single façade through which users open/close positions, settle premia, exercise, or liquidate.  Internally it executes a deterministic flow:

1. **Decode Action Array.**  Each element describes mint/burn/exercise/etc. with limits and size.
2. **Risk Assessment.**  Ask RiskEngine for `safeMode` + collateral ratios → make sure the user will remain solvent after the action.
3. **SFPM Call.**  Mint/burn positions; receive arrays of tokens moved and fees collected.
4. **Vault Settlement.**  Pass token deltas to the right CollateralTracker which handles interest accrual, commission splitting, share mint/burn.
5. **Update Premia Records.**  Per-leg premium accumulators are updated so future burns know how much has already been paid/earned.

Because every leg’s liquidity and every user’s borrow index are tracked on-chain, PanopticPool can always prove solvency with **one static call** (`isAccountSolvent`) which is vital for permissionless liquidation bots.

---

## 7.  User Journeys

### 7.1  Passive PLP

1. `deposit(token0)` or `deposit(token1)` into the appropriate CollateralTracker.
2. Earn interest from option sellers + share of commissions.
3. Withdraw anytime if you hold no open positions and vault utilisation allows.

### 7.2  Writing a Short Put Spread

1. Deposit token0 into its vault as initial collateral.
2. Call `dispatch()` with `mint` instruction specifying two short-liquidity legs: wide OTM put (high ratio) and long narrow ITM put (hedge).  TokenId packs both legs.
3. PanopticPool:
   * Verifies spread is price-bounded, passes ticks to SFPM
   * SFPM mints liquidity, possibly performs a swap so you only pay token0
   * CollateralTracker loans out extra token1 liquidity if needed and records borrow
4. Collect premium continuously as buyers remove your liquidity.
5. Burn later to close; if price crashes beyond max loss, RiskEngine already required full collateral so no social loss.

### 7.3  Buying a Long Call

1. Deposit the quote token as collateral (or borrow it via rehypothecation).
2. `dispatch()` mint with a _negative_ liquidity leg (removing liquidity between [strike,strike+width]).
3. Pay premium up front; position now tracks delta like a call option.
4. If price rallies, intrinsic value accrues automatically because the removed liquidity is worth more.  Seller is paying borrow interest.
5. Exercise at will by burning; or seller may force-exercise if range goes completely out-of-money and pay you a fee.

---

## 8.  Security & Trust Assumptions

*  **No Upgradeability.**  All core contracts are deployed as immutable clones; no proxy patterns.
*  **Minimised Custody.**  Only CollateralTrackers hold ERC-20 balances.  SFPM + PanopticPool hold nothing.  RiskEngine holds nothing.
*  **Guardian Powers.**  Guardian can lock a misbehaving pool (sets safeMode=2) and sweep dust from RiskEngine but cannot move user funds.
*  **Uniswap Battle-Tested Math.**  Liquidity math is reused from Uniswap libraries; price oracle is internal median + EMA to avoid TWAP manipulation.
*  **Comprehensive Error Library.**  Every revert path uses custom errors for tight gas and audit clarity.

---

## 9.  Gas & UX Optimisations

*  **Bit-Packing Libraries.**  Every frequently updated struct (tokenId, market state, oracle pack, balances) is a single `uint256` for cheap SSTORE and calldata.
*  **ERC-1155 Over ERC-721.**  An entire four-leg strategy fits in one `uint256` – far lighter than the original `NonFungiblePositionManager`.
*  **Batchable Multicall.**  Users can bundle approve + deposit + mint in one transaction.
*  **Vanity Factory NFT.**  Anyone who deploys a pool gets an on-chain SVG NFT; the salt mining helper can search for rare addresses.

---

## 10.  Extensibility & Public Goods

*  **SFPM as Stand-Alone Tool.**  Advanced LPs can use it directly to manage complex Uniswap positions without touching options.
*  **Builder Referrals.**  Any address can embed a `builderCode`; RiskEngine resolves it to a deterministic `BuilderWallet` where referral fees accrue.
*  **Chain-Agnostic.**  Contracts rely only on ERC-20, ERC-4626, ERC-1155, and Uniswap V3/V4 interfaces; Foundry config points at Cancun EVM for forward compatibility.

---

## 11.  Summary

Panoptic stitches together Uniswap’s constant-product liquidity math, a high-resolution collateral vault, and a sophisticated but stateless risk engine into a single, permissionless smart-contract system.  Traders can assemble arbitrary option strategies, LPs earn passive yield, and keepers ensure bad debt is liquidated instantly.  Every dollar of risk is accounted for on-chain, making Panoptic the first fully collateralised, counter-party-free options exchange native to DeFi.



## Main List of Files in Project

contracts/CollateralTracker.sol
contracts/PanopticPool.sol
contracts/RiskEngine.sol
contracts/SemiFungiblePositionManager.sol
contracts/SemiFungiblePositionManagerV4.sol
contracts/libraries/Math.sol
contracts/libraries/PanopticMath.sol
contracts/types/MarketState.sol
contracts/types/OraclePack.sol
contracts/types/PoolData.sol
contracts/types/RiskParameters.sol
contracts/types/TokenId.sol


 ## DOCUMENTATION: 

 ### pano-docs.md

# Overview

Panoptic is a permissionless options trading protocol. It enables the trading of perpetual options on top of any [Uniswap V3](https://uniswap.org/) and [Uniswap V4](https://uniswap.org/) pool.

The Panoptic protocol is noncustodial, has no counterparty risk, offers instantaneous settlement, and is designed to remain fully collateralized at all times.

## Core Contracts

### SemiFungiblePositionManager

A gas-efficient alternative to Uniswap's NonFungiblePositionManager that manages complex, multi-leg Uniswap positions encoded in ERC1155 tokenIds, performs swaps allowing users to mint positions with only one type of token, and, most crucially, supports the minting of both typical LP positions where liquidity is added to Uniswap and "long" positions where Uniswap liquidity is burnt. While
the SFPM is enshrined as a core component of the protocol and we consider it to be the "engine" of Panoptic, it is also a public good that we hope savvy Uniswap V3 and V4 LPs will grow to find an essential tool and upgrade for managing their liquidity.

### RiskEngine

The central risk assessment and solvency calculator for the Panoptic Protocol. This contract serves as the mathematical framework for all risk-related calculations and does not hold funds or state regarding user balances. The RiskEngine is responsible for:

- **Collateral Requirements**: Calculating the required collateral for complex option strategies including spreads, strangles, iron condors, and synthetic positions based on position composition and pool utilization
- **Solvency Verification**: Determining whether an account meets the maintenance margin requirements through the `isAccountSolvent` function, accounting for cross-collateralization between token0 and token1
- **Liquidation Parameters**: Computing liquidation bonuses paid to liquidators and protocol loss via `getLiquidationBonus`, factoring in the account's token balances and position requirements
- **Force Exercise Costs**: Calculating the cost to forcefully exercise out-of-range long positions via `exerciseCost`, using an exponentially decaying function based on distance from strike
- **Adaptive Interest Rate Model**: Computing dynamic borrow rates based on pool utilization using a PID controller approach, with rates adjusting between minimum and maximum thresholds to target optimal utilization
- **Oracle Management**: Managing the internal pricing oracle with volatility safeguards, exponential moving averages (EMAs), and median filters to prevent price manipulation
- **Risk Parameters**: Storing and providing access to protocol-wide risk parameters including seller/buyer collateral ratios, commission fees, force exercise costs, and target pool utilization levels
- **Guardian Controls**: Enabling an authorized guardian address to override safe mode settings and lock/unlock pools in emergency situations

The RiskEngine uses sophisticated algorithms including utilization-based multipliers (modulated by the VEGOID parameter), cross-buffer ratios for cross-collateralization, and dynamic collateral requirements that scale with pool utilization to ensure protocol solvency at all times.

### CollateralTracker

An ERC4626 vault where token liquidity from passive Panoptic Liquidity Providers (PLPs) and collateral for option positions are deposited. The CollateralTracker is responsible for:

- **Asset Management**: Tracking deposited assets, assets deployed in the AMM, and credited shares from long positions that exceed the rehypothecation threshold
- **Interest Accrual**: Implementing a compound interest model where borrowers (option sellers) pay interest on borrowed liquidity, with rates determined by the RiskEngine based on pool utilization
- **Commission Handling**: Collecting and distributing commission fees on option minting and burning, splitting fees between the protocol, builders (if a builder code is present), and PLPs
- **Premium Settlement**: Facilitating the payment and receipt of options premia between buyers and sellers, including settled and unsettled premia calculations
- **Balance Operations**: Managing user share balances through deposits, withdrawals, mints, redeems, and the delegation/revocation of virtual shares for active positions
- **Liquidation Settlement**: Handling the settlement of liquidation bonuses by minting shares to liquidators and managing protocol loss when positions are liquidated
- **Collateral Refunds**: Processing refunds between users when positions are closed, force-exercised, or adjusted

Each CollateralTracker maintains its own market state including a global borrow index for compound interest calculations, tracks per-user interest states (net borrows and last interaction snapshots), and coordinates with the RiskEngine to determine appropriate interest rates based on real-time pool utilization.

### PanopticPool

The Panoptic Pool exposes the core functionality of the protocol. If the SFPM is the "engine" of Panoptic, the Panoptic Pool is the "conductor". All interactions with the protocol, be it minting or burning positions, liquidating or force exercising distressed accounts, or just checking position balances and accumulating premiums, originate in this contract. It is responsible for:

- **Position Orchestration**: Coordinating calls to the SFPM to create, modify, and close option positions in Uniswap
- **Premium Tracking**: Tracking user balances and accumulating premia on option positions over time
- **Solvency Checks**: Consulting the RiskEngine to verify account solvency before allowing position changes or withdrawals
- **Settlement Coordination**: Calling the CollateralTracker with the necessary data to settle position changes, including commission payments, interest accrual, and balance updates
- **Risk Validation**: Ensuring all operations comply with the risk parameters and collateral requirements calculated by the RiskEngine

## Architecture & Actors

Each instance of the Panoptic protocol on a Uniswap pool contains:

- One PanopticPool that orchestrates all interactions in the protocol
- One RiskEngine that calculates collateral requirements, verifies solvency, and manages risk parameters
- Two CollateralTrackers, one for each constituent token0/token1 in the Uniswap pool
- A canonical SFPM - the SFPM manages liquidity across every Panoptic Pool

There are five primary roles assumed by actors in this Panoptic Ecosystem:

### Panoptic Liquidity Providers (PLPs)

Users who deposit tokens into one or both CollateralTracker vaults. The liquidity deposited by these users is borrowed by option sellers to create their positions - their liquidity is what enables undercollateralized positions. In return, they receive commission fees on both the notional and intrinsic values of option positions when they are minted, as well as interest payments from
borrowers. Note that options buyers and sellers are PLPs too - they must deposit collateral to open their positions. We consider users who deposit collateral but do not _trade_ on Panoptic to be "passive" PLPs.

### Option Sellers

These users deposit liquidity into the Uniswap pool through Panoptic, making it available for options buyers to remove. This role is similar to providing liquidity directly to Uniswap V3, but offers numerous benefits including advanced tools to manage risky, complex positions and a multiplier on the fees/premia generated by their liquidity when it is removed by option buyers. Option
sellers pay interest to PLPs on borrowed liquidity, with rates dynamically adjusted by the RiskEngine based on pool utilization. Sold option positions on Panoptic have similar payoffs to traditional options.

### Option Buyers

These users remove liquidity added by option sellers from the Uniswap Pool and move the tokens back into Panoptic. The premia they pay to sellers for the privilege is equivalent to the fees that would have been generated by the removed liquidity, plus a spread multiplier based on the portion of available liquidity in their Uniswap liquidity chunk that has been removed or utilized.

### Liquidators

These users are responsible for liquidating distressed accounts that no longer meet the collateral requirements calculated by the RiskEngine. They provide the tokens necessary to close all positions in the distressed account and receive a bonus from the remaining collateral, calculated by the RiskEngine's liquidation bonus formula. Sometimes, they may also need to buy or sell options to
allow lower liquidity positions to be exercised.

### Force Exercisors

These are usually options sellers. They provide the required tokens and forcefully exercise long positions (from option buyers) in out-of-range strikes that are no longer generating premia, so the liquidity from those positions is added back to Uniswap and the sellers can exercise their positions (which involves burning that liquidity). They pay a fee to the exercised user for the
inconvenience, with the fee amount determined by the RiskEngine's `exerciseCost` function.

## Flow

All protocol users first onboard by depositing tokens into one or both CollateralTracker vaults and being issued shares (becoming PLPs in the process). Panoptic's CollateralTracker supports the full ERC4626 interface, making deposits and withdrawals a simple and standardized process. Passive PLPs stop here.

Once they have deposited, all interactions with the protocol are initiated through the PanopticPool's unified entry points:

- `dispatch()` - The primary entry point for users to execute actions on their own behalf
- `dispatchFrom()` - Allows approved operators to execute actions on behalf of another user

These entry points accept encoded action data that specifies the operation to perform, which can include:

- Minting option positions with up to four distinct legs, each encoded in a positionID/tokenID as either short (sold/added) or long (bought/removed) liquidity chunks. The RiskEngine verifies that the account will remain solvent after minting.
- Burning or exercising positions. The RiskEngine ensures collateral requirements are met during the burn process.
- Settling long premium to force solvent option buyers to pay any premium owed to sellers
- Poking the median oracle to insert a new observation into the RiskEngine's internal median ring buffer
- Force exercising out-of-range long positions held by other users, with costs calculated by the RiskEngine
- Liquidating distressed accounts that no longer meet collateral requirements, with bonuses determined by the RiskEngine

This unified dispatch architecture provides a consistent interface for all protocol interactions while allowing the PanopticPool to orchestrate the necessary calls to the SFPM, CollateralTracker, and RiskEngine based on the requested action.



 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/clones-with-immutable-args/package.json

{
  "name": "clones-with-immutable-args",
  "author": "wighawag",
  "license": "BSD",
  "version": "1.1.2",
  "description": "Factory for deploying clones with immutable parameters.",
  "files": [
    "*.sol"

### lib/v4-core/test/js-scripts/package.json

{
  "name": "v4-js-scripts",
  "description": "Scripts for v4 tests",
  "license": "MIT",
  "publishConfig": {
    "access": "restricted"
  },
  "version": "1.0.0",

### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.9.4",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/v3-core/package.json

{
  "name": "@uniswap/v3-core",
  "description": "🦄 Core smart contracts of Uniswap V3",
  "license": "BUSL-1.1",
  "publishConfig": {
    "access": "public"
  },
  "version": "1.0.1-solc-0.8",

### lib/v3-periphery/package.json

{
  "name": "@uniswap/v3-periphery",
  "description": "🎚 Peripheral smart contracts for interacting with Uniswap V3",
  "license": "GPL-2.0-or-later",
  "publishConfig": {
    "access": "public"
  },
  "version": "1.4.2-solc-0.8",

### lib/solmate/package.json

{
  "name": "@rari-capital/solmate",
  "license": "MIT",
  "version": "7.0.0-alpha.3",
  "description": "Modern, opinionated and gas optimized building blocks for smart contract development.",
  "files": [
    "src/**/*.sol"
  ],

### lib/solady/package.json

{
  "name": "solady",
  "license": "MIT",
  "version": "0.0.198",
  "description": "Optimized Solidity snippets.",
  "files": [
    "src/**/*.sol",
    "js/**/*"

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.8.3",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.8.3",
  "files": [
    "/contracts/**/*.sol",
    "/build/contracts/*.json",
    "!/contracts/mocks/**/*"


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = 'contracts'
test = 'test/foundry'
out = 'out'
libs = ['lib']
no_match_path = "contracts/*"
evm_version = 'cancun'
optimizer = true
optimizer_runs = 9_999_999
viaIR = false
gas_limit = 9223372036854775807
eth_rpc_url = ""
fs_permissions = [{ access = "read", path = "./"}]

[profile.prod]
test='DO_NOT_COMPILE'
script="DO_NOT_COMPILE"

[profile.ci_test]
fork_block_number = 18963715

[profile.ci_sizes]
optimizer_runs = 216
test = 'DO_NOT_COMPILE'

[lint]
exclude_lints = ["mixed-case-variable", "mixed-case-function", "unaliased-plain-import", "unsafe-cheatcode", "unsafe-typecast"]

[profile.ci_sizes_ir]
optimizer_runs = 200
test = 'DO_NOT_COMPILE'
viaIR = true

[fuzz]
# temporary workaround while we figure out perf issues + configure different test sets + profiles
runs = 30
max_test_rejects = 9_999_999
[rpc_endpoints]
sepolia = ''


### package.json

{
  "devDependencies": {
    "@commitlint/cli": "^17.1.2",
    "@commitlint/config-conventional": "^17.1.0",
    "husky": "^8.0.0",
    "prettier": "3.0.1",
    "prettier-plugin-solidity": "1.1.3"
  },
  "scripts": {
    "postinstall": "husky install"
  },
  "lint-staged": {
    "*.{css,html,json,jsx,md,sass,scss,ts,tsx,vue,yaml,yml,sol}": "prettier --write"
  }
}


### remappings.txt

forge-std/=lib/forge-std/src/
@openzeppelin/=lib/v4-core/lib/openzeppelin-contracts/
solmate/=lib/solmate/
clones-with-immutable-args/=lib/clones-with-immutable-args/src/
univ3-core/=lib/v3-core/contracts
univ3-periphery/=lib/v3-periphery/contracts
v4-core/=lib/v4-core/src
@contracts/=contracts
@libraries/=contracts/libraries
@base/=contracts/base
@test_periphery/=test/foundry/test_periphery
@tokens/=contracts/tokens
@types/=contracts/types
@scripts/=scripts
@uniswap/=lib


