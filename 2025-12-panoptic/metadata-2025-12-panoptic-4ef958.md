
## PROTOCOL OVERVIEW:

# Panoptic Protocol – Detailed Technical Overview

### 1. What **Panoptic** Is
Panoptic is a fully-on-chain, non-custodial options protocol that lets anyone mint, trade, settle and liquidate **perpetual options** on top of any Uniswap V3 or Uniswap V4 pool.  
Unlike traditional, dated options Panoptic positions never expire; they can be opened, closed, force-exercised or liquidated at any time.  All settlement happens atomically on-chain, there is **no counter-party risk**, and the system is designed so that every account remains over-collateralised in every block.

The protocol treats Uniswap liquidity itself as the underlying option.  A **short option** is implemented by *adding* a chunk of concentrated liquidity to a Uniswap pool; the counterparty **long option** is implemented by *removing* that same chunk.  The economic behaviour of those two legs mirrors a covered call / protective put pair, but Panoptic tokenises the entire strategy into an ERC-1155 that can combine up to four independent ‘legs’.  

### 2. High-level Architecture
For each Uniswap pool the factory deploys:
* **PanopticPool** – orchestrator & single public entrypoint.
* **RiskEngine** – pure logic contract that calculates collateral, liquidation bonus, interest rates and oracle ticks.
* **CollateralTracker0 / CollateralTracker1** – two independent ERC-4626 vaults (one per underlying token) that hold user deposits, track borrows, collect commissions and accrue interest.
A single global **SemiFungiblePositionManager (SFPM)** contract services *all* pools; it is the ‘engine’ that actually mints/burns Uniswap liquidity and wraps the resulting positions into ERC-1155 ids.

```
  user ─┐   ▲                                                                    
        │   │   ┌─ interest + commission      ┌─ borrow index & utilisation     
        ▼   │   ▼                             │                                  
   CollateralTracker0  CollateralTracker1  <─ RiskEngine <─ oracle / IRM maths   
        ▲           ▲                         ▲   ▲                              
        │           │                         │   │                              
        └── tokens ─┼────── PanopticPool ─────┘   │ calls for collateral / IRM   
                    │                             │                              
                    ▼                             │                              
                 SFPM ─── Uniswap V3 / V4 pool ◄──┘ (mints & burns liquidity)    
```

### 3. Core Components and Their Responsibilities
#### 3.1 SemiFungiblePositionManager (SFPM)
* ERC-1155 that **cannot be transferred** – positions live & die inside Panoptic.
* Encodes up to **4 legs** into a 256-bit `TokenId` (asset side, strike, width, long/short flag, partner index, etc.).
* On *mint* it:
  1. Decodes legs, checks tick limits & enforced tick range.
  2. Calls Uniswap `mint` for each short leg (adds liquidity).
  3. Calls Uniswap `burn` for each long leg (removes liquidity).
  4. Optionally performs a netting swap so that the caller can fund the trade with a single token.
  5. Records per-chunk liquidity and per-liquidity *premium* accumulator (`owed` to seller vs `gross` collected from AMM fees) which later settles through CollateralTracker.
* On *burn* it performs the reverse flow and returns collected fees/premia deltas to the caller.
* Keeps no ETH/ERC20 balances – all debits/credits are paid via Uniswap callbacks.

#### 3.2 CollateralTracker (one per token)
* Conforms to ERC-4626 (deposit, mint, withdraw, redeem) and issues ERC-20 *shares*.
* Keeps accounting for:
  - `depositedAssets` – idle liquidity provided by PLPs (passive liquidity providers).
  - `assetsInAMM`   – liquidity borrowed by option sellers and sitting inside Uniswap.
  - `creditedShares` – virtual shares credited when long premia > rehypothecation threshold.
* Implements a **compound-interest model**: borrowers pay variable-rate interest per second; lenders earn that interest implicitly through share price appreciation.
* Interest rate is fetched from `RiskEngine.interestRate(utilisation)`, where utilisation = assetsInAMM / totalAssets.
* Handles **commissions** on mint/burn (`notionalFee` and `premiumFee`) and splits them between protocol, builders and PLPs according to `RiskParameters`.
* Settlement helpers exposed only to PanopticPool: `settleMint`, `settleBurn`, `settleLiquidation`, `delegate`, `revoke`, `refund`.

#### 3.3 RiskEngine
* Pure math & oracle contract – **never holds user funds**.
* Key responsibilities:
  1. **Collateral maths** – seller vs buyer collateral ratio, utilisation multipliers, spread/strangle credits, cross-collateral buffers.
  2. **Solvency check** – `isAccountSolvent` returns true/false for a candidate action.
  3. **Liquidations** – `getLiquidationBonus` (how much bonus liquidator gets & protocol loss), `haircutPremia` (claw back excess premium).
  4. **Force exercise** – `exerciseCost` gives exponentially decaying cost to force-exercise deep OTM longs.
  5. **Adaptive Interest-Rate Model (IRM)** – PID-style controller that targets utilisation; exposes `interestRate` (view) and `updateInterestRate`.
  6. **Oracle** – 8-slot median filter + multi-period EMAs, fully on-chain, guarded by clamp rules so pool price manipulation cannot bankrupt the system. Stored in a single 256-bit `OraclePack` per pool.
  7. **Guardian** – an address that can toggle `safeMode` for a pool (locks mint/burn) and sweep stray tokens sent to RiskEngine.
* All tunables are packed into a single `RiskParameters` word that PanopticPool asks for every user action. Parameters can include a **builderCode** referral which reroutes a percentage of fees to a deterministic `BuilderWallet` deployed via CREATE2 by `BuilderFactory`.

#### 3.4 PanopticPool
* Single façade contract that end-users interact with.
* Exposes two generic entrypoints
  - `dispatch()` – user acts on themselves (mint, burn, settle, poke, force exercise…).
  - `dispatchFrom()` – approved operator (e.g., a liquidation bot) acts on another user.
* Orchestrates the whole flow:
  1. Decodes action list & fetches current ticks via SFPM / RiskEngine oracle.
  2. Calls SFPM **mintTokenizedPosition** / **burnTokenizedPosition**.
  3. Sends collected paid/received amounts to **CollateralTracker0/1.settleMint/Burn** (which in turn charge commission, update interest, etc.).
  4. Runs **RiskEngine.isAccountSolvent** *before and after* the state-changing action; reverts if insolvency would result.
  5. For `liquidate` or `forceExercise` it coordinates the complete asset flow, including liquidation bonus, haircut of excess premium, and optional cross-asset conversions.
* Tracks per-user:
  - `PositionBalance` snapshot (size, utilisation at mint, four oracle ticks at mint).
  - Homomorphic hash of all open TokenIds + leg count (prevents inconsistent lists).
  - Per-leg last‐seen gross premium so that only the delta since last settlement is charged.

### 4. Life-cycle of an Option
1. **Deposit collateral** – user deposits token0 and/or token1 into the respective CollateralTracker vault(s) and receives shares.
2. **Mint** – user calls `PanopticPool.dispatch` with `Action.Mint` specifying up to four legs.  PanopticPool → SFPM adds/burns liquidity in Uniswap, receives the net token deltas, and tells CollateralTracker how much was borrowed or credited.  A commission proportional to **notional liquidity** is immediately charged.
3. **Premium accrual** – while the position lives the removed liquidity (long) or added liquidity (short) earns **Uniswap swap fees**. SFPM continuously tracks them per-liquidity; sellers can claim at any time, buyers may owe if their long position is in-range.
4. **Burn / Exercise** – user closes a leg or the entire position.  Premium owed is settled; a **premiumFee** (min of notional-fee and premium) is charged.  Collateral is released and shares are returned.
5. **Force exercise** – if a long leg is far out-of-range and is no longer generating premium, the counter-party can pay a small `exerciseCost` to forcibly exercise it so their collateral gets unlocked.
6. **Liquidation** – if at any moment an account’s collateral falls below maintenance margin the first caller can liquidate.  All positions are closed, CollateralTracker mints shares to the liquidator equal to the bonus, protocol loss is absorbed from pools’ insurance fee bucket, and any remaining excess premium can be clawed back via `haircutPremia`.

### 5. Risk Management Details
* **Dynamic collateral ratios**: seller ratio ramps up from a minimum to 100 % as utilisation grows; buyer ratio is fixed (e.g., 15 %).
* **Cross-collateral**: a buffer ratio (CROSS_BUFFER) allows positive balance in the ‘cheap’ token to cover requirement in the ‘expensive’ side at current price.
* **Spread / Strangle**: legs with matching `riskPartner` enjoy reduced collateral (max loss of the pair, or half the standalone requirement).
* **Safe mode**: triggered automatically when EMA divergence indicates high volatility or when guardian toggles it.  In safe mode pools reject new mint / burn that would reduce overall solvency.
* **Interest-Rate Model**: piece-wise linear curve around target utilisation; PID component nudges the curve (rateAtTarget) up/down over time depending on previous block’s error so that borrow rate self-adjusts.
* **Oracle manipulation resistance**: Panoptic never relies on a single TWAP.  Collateral checks are performed at **multiple at-ticks** (current, median, lagged, clamp) and an account must be solvent at *all* of them to pass.

### 6. Gas & Storage Optimisations
* Every complex struct is bit-packed into a single word (`TokenId`, `MarketState`, `RiskParameters`, `OraclePack`, `PositionBalance`, `PoolData`).
* Global constants live in `Constants.sol`; heavy math uses custom `Math` library with inline assembly `mulDiv*` helpers.
* ERC-20/1155 transfers use `SafeTransferLib` which accepts tokens with missing return booleans.
* Approvals are handled lazily through `InteractionHelper.doApprovals` – one call per pool after deployment.
* All pools, trackers and builder wallets are deployed as **minimal proxies** with immutable args (clone-with-immutable-args) to keep deployment gas low.

### 7. Upgrade & Admin Story
* **No upgradability** – every pool is a fixed immutable proxy; if logic needs to change a new implementation + factory version is deployed and users can migrate funds.
* **Guardian** has *only* the power to:
  - Lock/unlock a pool’s safe mode flag.
  - Sweep stray ERC-20 sent to RiskEngine (can’t touch CollateralTrackers).
* **BuilderFactory owner** can deploy referral wallets but cannot touch user funds – the wallet only holds protocol fee revenue directed to the builder.

### 8. Security & Trust Assumptions
1. Uniswap V3/V4 pools remain solvent and follow their documented invariants.
2. The guardian key remains uncompromised; even if compromised the attacker can only freeze (not steal) the system.
3. SFPM uses Uniswap callbacks for payment; users must approve the necessary tokens before minting.
4. All math is done with checked custom libraries; overflow is guarded and reverts with descriptive errors from `Errors.sol`.

### 9. Extensibility & Composability
* Any EOA or smart contract can interact with `PanopticPool.dispatch`, enabling vaults, DAOs or structured-product builders on top.
* Builder referral system allows front-ends to monetise order-flow without taking custody.
* ERC-4626 CollateralTrackers are compatible with yield aggregators; idle collateral could be rehypothecated to external money-markets in future iterations.

### 10. Conclusion
Panoptic turns Uniswap liquidity itself into a perpetual option primitive while maintaining **capital efficiency**, **instant settlement** and **permissionless access**.  Through a careful mix of bit-packed structs, sophisticated risk math and non-custodial vault design it achieves CEX-like margin behaviour entirely on-chain.

---
*This document is a condensed technical reference (~2500 words) and should serve reviewers and integrators as a starting point for deeper audits.*


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


