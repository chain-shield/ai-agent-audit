
## PROTOCOL OVERVIEW:

## Hybra Finance – Protocol Architecture & Mechanics

### 1. 30-second elevator pitch
Hybra Finance is a Uniswap-v3-style concentrated-liquidity DEX coupled to a next-generation **ve(3,3)** token economy.  
It lives on the Hyperliquid EVM roll-up, issues the governance token **HYBR**, and fuses three previously disjoint ideas into one fly-wheel:

1. Capital-efficient CL pools (better prices → more volume).  
2. A **veHYBR** voting escrow that directs weekly HYBR emissions to the pools that earn the most external income (swap fees + partner bribes).  
3. An **intent / solver layer** that lets traders sign gas-less swap intents; solvers who want order-flow must lock veHYBR, so every trade reinforces the voting system.

The result is a system designed to break the classic Solidly “death spiral”. Token inflation decays each week while external, non-inflationary cash-flow scales with usage, so emissions are mathematically guaranteed to be exceeded by fees after ≈30 weeks.

---

### 2. Component map
| Domain | Contract(s) | What it does |
| ------ | ----------- | ------------ |
| **DEX core** | `CLFactory`, `CLPool`, periphery routers | Creates & manages Uniswap-v3-style pools with protocol / unstaked / dynamic fee modules. |
| **Range orders & ranges** | `NonfungiblePositionManager`, `GaugeCL` | LP positions are minted as NFTs and can be staked in gauges. |
| **Classic v2 pools (optional)** | `PairFactory`, `Pair` | Solidly‐style stable & volatile pools for tail assets. |
| **Governance token** | `HYBR` | Plain ERC-20, 500 M hard cap, minted by `MinterUpgradeable`. |
| **Voting escrow** | `VotingEscrow` (veHYBR) | Locks HYBR up to 4 years, NFT-based, supports permanent locks, merge/split, delegation, on-chain SVG art. |
| **Emission engine** | `MinterUpgradeable` | Weekly emission with decay, tail & rebase logic. Mints to team wallet, rebase distributor & gauges. |
| **Reward distributor** | `RewardsDistributor` | Streams weekly rebase to veHYBR lockers. |
| **Gauge system** | `GaugeFactory`, `GaugeFactoryCL`, `GaugeV2`, `GaugeCL`, `GaugeManager` | Deploys gauges, streams emissions, harvests trading fees and forwards them to bribe contracts. Handles emergency pause. |
| **Bribes & fee sharing** | `BribeFactoryV3`, `Bribe`, internal bribes in gauges | Partners deposit ERC-20 bribes per epoch; trading fees from pools are also routed here. ve voters claim pro-rata. |
| **Intent / solver layer** | off-chain + `HybrSwapper` | Traders sign swap intents; solvers fill & pay gas, must stake veHYBR for routing priority. Fees collected flow back to voters. |
| **Auto-compound vault** | `GovernanceHYBR` (gHYBR) | Pooled veHYBR strategy that auto-claims bribes, swaps them to HYBR and re-locks. Users receive liquid gHYBR shares. |
| **Auxiliary** | `PermissionsRegistry`, `TokenHandler`, API helpers, lens contracts | Role registry, token whitelisting, read-only APIs, multicalls, quoting, SVG descriptor, etc. |

---

### 3. Life-cycle of a trade
1. Trader signs an **intent**: _“swap 1 ETH → USDC at ≥3 250, expire in 5 min”_.  
2. Off-chain solver with staked veHYBR wins the auction, executes the optimal path through one or more **CLPools**, pays the gas, and submits the transaction.  
3. In each pool:
   * Swap fee is split into **(a)** staked-LP fee (to the pool’s gauge), **(b)** unstaked-LP fee, **(c)** protocol fee (can be zero) according to `DynamicSwapFeeModule` & `CustomUnstakedFeeModule` settings.
   * The gauge immediately forwards the staked fee to its **internal bribe**.
4. At epoch rollover (weekly):
   * Gauges receive new HYBR emissions from `MinterUpgradeable` via `GaugeManager` according to ve votes.
   * veHYBR voters claim trading-fee bribes (token0 / token1) plus any **external bribes** partners deposited.

Result: every swap generates real, external income that competes for future HYBR emissions.

---

### 4. ve(3,3) flow in detail
1. **Locking**: users lock HYBR for up to 4 years → receive veHYBR NFT with time-decaying voting power.  
   * Permanent locks are supported and can later be converted back to time locks.
2. **Voting**: once per epoch holders call `VoterV3.vote(tokenId, pools[], weights[])`.  
   * Votes are capped (`maxVotingNum`) and gated by an epoch timestamp guard.  
   * Weight deposits into matching internal+external bribe contracts.
3. **Bribes / Fees**: projects must bribe their gauge each epoch with at least `α · TVL · baseFee`. If they under-bribe, a slice of their veNFT is clawed back ☞ no free-riders.  
4. **Emissions**: `MinterUpgradeable` computes weekly emission = max(targetDecay, tailEmis).  
   * Team share (≤5 %), ve rebase (≤20 %), remainder to gauges via `GaugeManager`.
5. **Claim**: lockers can claim (a) fee bribes, (b) external token bribes, (c) rebase HYBR.  
   * Claim can be done in HYBR, veHYBR (auto-relock) or gHYBR via the `RewardHYBR` wrapper which applies a configurable penalty for raw HYBR withdrawals.

---

### 5. Gauge design
There are two gauge flavours:
* **GaugeV2** – for classic v2 LP tokens (`Pair`).
* **GaugeCL** – for Uniswap-v3 NFT positions.

Common properties
* Custodies user LP/NFTs.
* Tracks `rewardRate`, accrues `rewardPerTokenStored`, and lets `GaugeManager` harvest on behalf of users for gas efficiency.
* Forwards pair fees to its **internal bribe** so that fee APR is rewarded to voters, not LPs – aligning LPs with voters via emissions.
* Emergency council can toggle `emergency` mode allowing penalty-free withdrawals but freezing rewards.

Additional for CL
* Uses growth inside tick ranges to calculate rewards per NFT.  
* Integrates with `NonfungiblePositionManager` so users can add/remove liquidity while staked.

---

### 6. Factories & role control
All deployers are upgradeable proxies owned by a **hybraMultisig** (4-of-6).  Authorization is centralised in `PermissionsRegistry` which stores string-based roles:
* `GOVERNANCE` ― protocol decisions (kill / revive gauge, whitelist tokens).
* `GAUGE_ADMIN` ― adjust gauge params, rewarders.
* `BRIBE_ADMIN` ― manage bribe token lists.
* `EMERGENCY_COUNCIL` ― toggle emergencies.
* `GENESIS_MANAGER` ― one-time bootstrap privileges.

Factories consult the registry so governance can grant granular control without deploying new code.

---

### 7. Dynamic fee model
`DynamicSwapFeeModule` makes swap fees reflexive to volatility:
```
fee = baseFee + K · |tick_now – tick_TWAP|;   capped at feeCap
```
* `secondsAgo` sets the TWAP window (default 30 min).  
* Governance can set per-pool `baseFee`, `scalingFactor (K)`, `feeCap`, or even a `ZERO_FEE_INDICATOR` for promotional zero-fee pools.
* VIP order-flow providers can receive address-level discounts (ppm-denominated).

Unstaked liquidity can be charged an extra fee via `CustomUnstakedFeeModule`, encouraging LPs to stake in gauges.

---

### 8. Intent / solver incentive loop
1. Trader submits intent → solver MUST own veHYBR **or** pay a higher protocol fee.  
2. Solvers are therefore economic buyers of HYBR and sustained bribers of gauges they profit from.  
3. All solver-paid swap fees (0.02–0.05 bp) are non-inflationary revenue to the protocol treasury and can be redirected as additional bribes or buy-backs.

---

### 9. Auto-compound vault (gHYBR)
`GovernanceHYBR` turns passive holders into active participants:
* Users deposit HYBR → receive transferrable **gHYBR**.  
* Contract owns a max-locked veNFT (`veTokenId`).
* Operator script (could be same as solver) periodically:
  * Claims rebase & bribes via `claimRewards()`.
  * Swaps non-HYBR rewards to HYBR through `HybrSwapper` (aggregator whitelist).
  * Calls `compound()` to extend lock and increase amount.
* Withdrawals split the master veNFT using `VotingEscrow.multiSplit`, charge a small exit fee (`withdrawFee`), and hand the new veNFT to the user.

This funnels small holders into a single giant ve position, amplifying voting power and reducing gas.

---

### 10. Tokenomics recap
* **Supply:** 500 M HYBR max, minted once by `HYBR.initialMint()` then controlled exclusively by `MinterUpgradeable`.
* **Launch emission:** 500 k HYBR week-0, decay 2 %/wk.  
* **Crossover:** with conservative volume & bribe inputs, external income > emissions by week 30 (see white-paper plot).  
* **Rebase cap:** max 20 % of weekly emission so lockers earn real yield without runaway supply growth.  
* **Team allocation:** capped at 5 % of weekly mint, streamed not vested up-front.

---

### 11. Security & upgradability
* Core CLPool logic is forked from Uniswap v3 (battle-tested) with additional fee-splitting; compiler 0.7.6, optimizer 10 runs + IR.  
* ve/ gauge / bribe system leverages Velodrome v2 patterns on Solidity 0.8.13 with OZ 5.4 libs.  
* All upgradeable contracts use **OpenZeppelin UUPS** proxies with `onlyOwner` (hybraMultisig) upgrade path; immutable pools & factory clones are not upgradeable.  
* Critical arithmetic is unchecked-math-free; re-entrancy guards (`nonReentrant`) wrap all external flows interacting with ERC-20.

---

### 12. Failure & mitigation matrix
| Risk | Mitigation |
| ---- | ---------- |
| Token death-spiral | Emission decay hard-coded; partner bribe floor + solver fees inject external cash. |
| Partners under-bribe | `settleEpoch()` claw-backs veNFT voting power each week they are under the minimum. |
| LPs stay unstaked | `unstakedFee` penalises them → higher APY when staking. |
| Gauge bribery cartel | `maxVotingNum` spreads votes; dynamic fees & solver bidding move volume to best-priced pools regardless. |
| Contract exploits | Re-use audited code (Uni v3, Velodrome), heavy test-suite with Foundry fuzz & Echidna; third-party audit mandatory before main-net. |
| Governance capture | 4-of-6 multisig plus on-chain timelock before critical changes (parameter tweaks, upgrades). |

---

### 13. Development & testing stack
* Solidity 0.8.13 (ve & governance) / 0.7.6 (CL core).  
* Foundry with IR & gas reporting; Hardhat for TypeChain + Etherscan verification.  
* Unit + fuzz tests across both v2 & v3 pools; Echidna invariant fuzz for CLPool.  
* GitHub Actions CI running `forge test -vvv`, slither static analysis, and differential tests against upstream Uniswap v3.

---

### 14. Road-map
1. **Public test-net** with full CL + ve system, gHYBR vault, solver sandbox.  
2. **Audit & contest** (Code4rena).  
3. **Main-net Genesis**: liquidity boot-strap, airdrop claim portal, veHYBR WAR season.  
4. **Intent relayer launch** once Hyperliquid finalises EVM mempool access.  
5. **Strategy vaults** (Gamma, Arrakis integrations) and cross-chain fee streaming.

---

### 15. TL;DR for builders
Hybra offers:
* The most fee-efficient AMM on Hyperliquid (Uni-v3 math + dynamic fees).
* A ve(3,3) design where every swap, bribe and solver bid reinforces HYBR demand.
* A modular, audited code-base: forkable CL core, generic gauges, UUPS upgradability, rich periphery tooling.

If you believe the missing piece for Solidly forks was *external income > inflation* – Hybra is the on-chain experiment that finally tips that equation.



## Main List of Files in Project

cl/contracts/core/CLFactory.sol
cl/contracts/core/CLPool.sol
cl/contracts/core/fees/DynamicSwapFeeModule.sol
ve33/contracts/CLGauge/GaugeCL.sol
ve33/contracts/CLGauge/GaugeFactoryCL.sol
ve33/contracts/GaugeManager.sol
ve33/contracts/GaugeV2.sol
ve33/contracts/GovernanceHYBR.sol
ve33/contracts/HYBR.sol
ve33/contracts/MinterUpgradeable.sol
ve33/contracts/RewardHYBR.sol
ve33/contracts/VoterV3.sol
ve33/contracts/VotingEscrow.sol
ve33/contracts/swapper/HybrSwapper.sol


 ## DOCUMENTATION: 

 ### hydra-docs.md

# Hydra Finance Whitepaper

*A story about taming the ve(3,3) hydra on Hyperliquid*

***

#### Prologue — why we still believe

I watched the first **Solidly** pools hatch on Fantom.\
For one brilliant week it felt like alchemy: fees were paid in real dollars, emissions were paid in future hope, and lockers became tiny printing presses.\
Week two the charts bent.\
Week six they snapped: partners stopped bribing, votes fled to ghost gauges, SOLID bled to zero, and a once-luminous idea spiralled into the textbook *death spiral*.

The idea, however, never died.\
Velodrome, Aerodrome, Thena, Retro, Ramses—each iteration stitched a new safety net onto the ve(3,3) balloon.\
Some slowed the spiral, one or two climbed out of it, none broke the curse completely.

**Hybra** is the name we give to our next attempt.\
It lives on Hyperliquid— a chain that already clears nine-figure volume daily and rewards its traders with **HYPE**, not platitudes.\
If we get the flywheel right here, the hydra may finally breathe clean air.

***

#### 1 We dissect the corpse before we race the horse

**1.1 The three knives that killed past ve(3,3) DEXes**

1. **Front-loaded inflation** – 100 % of supply promised up front → token dumps faster than it can be locked.
2. **Free ride for partners** – protocols receive veNFT, bribe once for show, then harvest fees forever.
3. **Thin or fragmented liquidity** – users pay more slippage than on Uniswap or the native CEX, so volume never roots itself.

Miss any one knife, you wobble; miss all three, you die.\
We decide to dull every blade.

***

#### 2 The pact we strike with the hydra

*Rule 1 Cold-start must bang, not bleed.*\
We copy no one’s playbook; we hybridise them.

*Rule 2 Every dollar of emission must be backed by at least a cent of **external** income.*\
Partner bribes and Foundation bribes are not garnish, they are line-items in the P\&L.

*Rule 3 Traders come first.*\
If Robinhood feels smoother than your DEX, you deserve zero fees.\
So we graft **Uniswap v3 style ranges** on day one, hook in **v4 intents** the day they ship, and pour every marketing dollar into showing traders a visibly better price.

***

#### 3 How the machine works

**3.1 Keeping the heads alive – minimum-bribe rule**

Every epoch (7 days) a partner **must** bribe its gauge at least:

```
minBribe = α · avgTVL_epoch · baseFee
where
    α  = 0.75 ‰   // DAO-tunable, start at 0.00075
    baseFee = 0.04 %            // taker fee schedule
```

If the partner under-bribes, the contract melts part of its veNFT and recycles it into the community pool.

**Pseudocode (Solidity-pseudo)**

```solidity
solidity 
function settleEpoch(address partner) external {
    uint tvl = oracle.tvlOf(partner);
    uint min = alpha * tvl * baseFee;       // 18-dec fixed
    uint paid = bribeVault.claimed(partner, epoch);

    if (paid < min) {
        uint deficit = min - paid;
        uint ratio = deficit * 1e18 / min;  // 0-1e18
        uint burn = ratio * veBalance[partner] / 2; // soft-clawback
        veBalance[partner] -= burn;
        veBalance[DAO]     += burn;
        emit Clawback(partner, burn);
    }
}
```

*Interpretation*

* **Half** of the deficit ratio is burned each epoch; partners can correct course next week.
* Partners may **`recycle()`** up to 40 % of the bribe after fees settle, keeping net APR positive for them while lockers still receive the cash-flow.

> **Result** – grabbing a top-TVL airdrop commits you to a recurring marketing budget. The veNFT becomes a kind of perpetual-license that *rents itself*.

***

#### 4 Traders will actually like this place

| milestone  | feature                                              | why they care                                     |
| ---------- | ---------------------------------------------------- | ------------------------------------------------- |
| **Launch** | Uni v3 ranges, auto-rebalancer, gas-abstracted swaps | 3–6× fee density vs v2 AMMs; one-click LP.        |
| +60 days   | <p>Uni v4 hooks—limit orders, TWAP oracle</p><p></p> | <p>CEX-grade tooling without CEX risk.</p><p></p> |
| +120 days  | Strategy vaults (stable ranges, gamma farming)       | LP becomes deposit-and-chill                      |
| +180 days  | Smart router & off-chain intent relayer              | 15 % better fill on average; no failed Tx rage.   |

#### How Hybra will embed an **intent layer**—and why it matters

**What we add**\
Hybra lets traders sign a simple *intent* (“swap 1 ETH → USDC at ≥ $3 250, expire in 5 min”).\
Off-chain *solvers* compete to fill the order, pay the gas, and settle the result on-chain.\
The router is **gauge-aware**: solvers that stake or lock veHYBRA get priority routing, so execution flow is pulled toward the pools they vote for.

| Stakeholder            | Direct benefit                                                                                                                                                                                                                     | Fly-wheel effect                                                     |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| **Trader**             | <p>• Gas-free, fail-proof swaps.<br>• Solvers aggregate Hybra pools + RFQ quotes + external AMMs, so average price improves and MEV is neutralised.<a href="https://0x.org/post/intents-in-defi?utm_source=chatgpt.com">0x</a></p> | Better UX → higher volume → deeper books.                            |
| **veHYBRA voter / LP** | <p>• 100 % of solver-paid swap fees stream to ve vaults.<br>• Solvers must <strong>stake veHYBRA</strong> to maximise routing priority → constant buy-pressure and bribe demand.</p>                                               | Fee APR rises without extra inflation.                               |
| **Protocol (Hybra)**   | <p>• Takes a 0.02–0.05 bp settlement fee from solvers—<em>non-inflationary</em> income.<br>• Gauge-aware routing auto-recycles volume into the pools that pay the most fees.</p>                                                   | External cash flow > emissions sooner → death-spiral risk minimized. |

**Why it’s differentiated**\
– UniswapX and CoW Swap show that intent + solver architecture can deliver CEX-grade pricing and gasless UX, but they do **not** tie solver incentives to ve voting. Hybra stitches the two together, turning every solver into a ve holder and every trade into a vote-reinforcing event.

In short, the intent layer makes trading cheaper for users, richer for voters, and cash-positive for the protocol—exactly the “someone other than the minter pays” principle we build around.

Marketing is idle if product stinks; we reverse the order.

***

#### 5 *Inflation-led ignition → Income-led sustainability*

**Assumptions)**

```
weekly token emission decay k        = 2 %
token price P(t)            = logistic 1 $ → 2 $ (speed q = 5 %/wk)
week-0 emission tokens       = 500 000
external-income ceiling      = 650 000 USDC / wk
external-income rise speed   = 8 %/wk (mid-point t₀ = 16 wk)
```

***

**5.1 What the curve now does**

* **Week 0-10 – Ignition**\
  Inflation (yellow) hovers around **500-570 k USDC** because price appreciation offsets the 2 % token decay.\
  → liquidity rushes in; veNFT WAR is headline news.
* **Week 11-29 – Hand-off**\
  Fees + bribes (orange) scale with volume; inflation slides as price growth flattens.\
  → the two lines draw together.
* **Week 30 – Crossover**\
  External income overtakes inflation **(\~30 th week)** and never looks back (see dashed line in chart).
* **Week 30+ – Income-led era**\
  Emissions keep decaying geometrically; income keeps compounding with usage.\
  By week 52 the protocol runs a **+278 k USDC weekly surplus**.

***

| Snapshot    | Emission value E(t) | External income I(t) | Surplus I – E |
| ----------- | ------------------- | -------------------- | ------------- |
| **Week 0**  | 500 k               | 140 k                | **–360 k**    |
| **Week 26** | 511 k               | 448 k                | –62 k         |
| **Week 52** | 337 k               | 615 k                | **+279 k**    |

*(all figures USD / week, rounded)*

<figure><img src="https://3034550939-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FV7WY0kfUBQqJkRAW2jcI%2Fuploads%2FbfYKYnrUWGgtPfJfVjgN%2Fimage.png?alt=media&#x26;token=28e112c0-1ac4-4660-8708-ac1aacea90aa" alt=""><figcaption><p>emission &#x26; income chart</p></figcaption></figure>

***

**5.2 Why this matters**

| Phase        | Dominant payer     | Locker psychology                                        | Fragility                                |
| ------------ | ------------------ | -------------------------------------------------------- | ---------------------------------------- |
| Ignition     | Printer            | “Farm the yield, test the UX.”                           | Token price drag (managed by decay cap). |
| Hand-off     | Printer ≈ Outsider | “APR looks balanced—maybe relock.”                       | Keep partner bribe ROI > 1.              |
| Steady-state | Outsider           | “Fees + cash bribes = real yield; inflation just icing.” | Only if volume collapses.                |

Because **E(t)** is capped by deterministic decay while **I(t)** is free to grow with depth and volume, the crossover is **mathematically inevitable** so long as\
`I_max > E₀ · (1-k)^{t_cross}`.\
With the numbers above that holds at t ≈ 30 weeks even under a 50 % price shock.

The plotted curve (see chart) shows a launch that *starts* with attractive inflation but smoothly hands the baton to sustainable external income—exactly the arc a ve(3,3) hydra needs to break the death-spiral cliché.

***

#### Epilogue — an invitation

Solidly showed us how ve(3,3) could soar;\
its fork-sons showed us every way it could crash.\
We believe the missing piece is simple: **make someone other than the minter pay.**\
Bribes are that someone; traders are that someone; the Foundation is that someone.\
If we balance their incentives, the hydra sheds its death spiral and grows into a flywheel.

Lock some veHYBRA, steer the gauges, and take a cut every time the chain you already trade on moves a dollar.\
Let’s prove the hydra can live.

*— The Hybra core crew*




 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### ve33/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.10.0",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### ve33/lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.4.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### ve33/lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.4.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### ve33/lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true,
  "dependencies": {
    "minimatch": "^3.1.2"
  }
}

### ve33/lib/openzeppelin-contracts/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.9.6",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### cl/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.7.6",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### cl/lib/base64/package.json

{
  "name": "base64-sol",
  "version": "1.1.0",
  "description": "base64 implementation in solidity",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },

### cl/lib/solidity-lib/package.json

{
  "name": "@uniswap/lib",
  "version": "4.0.1-alpha",
  "description": "📖 Solidity libraries that are shared across Uniswap contracts",
  "files": [
    "contracts",
    "!contracts/test"
  ],

### cl/lib/ExcessivelySafeCall/package.json

{
  "name": "@nomad-xyz/excessively-safe-call",
  "version": "0.0.1-rc.1",
  "description": "Helps you call untrusted contracts safely",
  "keywords": [
    "nomad",
    "excessively safe call"
  ],

### cl/lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "3.4.2-solc-0.7",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks",

### cl/lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "3.4.2-solc-0.7",
  "files": [
    "/contracts/**/*.sol",
    "/build/contracts/*.json",
    "!/contracts/mocks",


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### ve33/foundry.toml

[profile.default]
src = "contracts"
out = "out"
libs = ["lib", "node_modules"]
test = "test"
script = "script"
# Suppress warnings
ignored_error_codes = [2519, 5667, 2072, 2018]
suppress_warnings = true
# Disable specific lint warnings
ignored_warnings_from = ["*"]
deny_warnings = false

# See more config options https://github.com/foundry-rs/foundry/tree/master/config

remappings = [
    "@openzeppelin/contracts/=node_modules/@openzeppelin/contracts/",
    "@openzeppelin/contracts-upgradeable/=node_modules/@openzeppelin/contracts-upgradeable/",
    "@cryptoalgebra/integral-core/=node_modules/@cryptoalgebra/integral-core/",
    "@cryptoalgebra/integral-periphery/=node_modules/@cryptoalgebra/integral-periphery/",
    "@cryptoalgebra/integral-base-plugin/=node_modules/@cryptoalgebra/integral-base-plugin/",
    "@cryptoalgebra/integral-farming/=node_modules/@cryptoalgebra/integral-farming/",
]

solc_version = "0.8.13"
evm_version = "london"
optimizer = true
optimizer_runs = 200
via_ir = true

# Gas optimization settings
ffi = false
fs_permissions = [
    { access = "read-write", path = "./" },
    { access = "read", path = "../cl/deployments/" }
]

[profile.local]
verbosity = 3
gas_reports = ["*"]

[profile.test]
verbosity = 3
gas_reports = ["*"]

[rpc_endpoints]
local = "http://localhost:8545"
anvil = "http://localhost:8545"
hyper_test="${HYPER_TEST}"
hyper = "${HYPEREVM_RPC_URL}"

[etherscan]
# Add your API keys here

# Lint configuration
[profile.default.lint]
# Disable specific lint rules
ignored = [
    "asm-keccak256",           # Inline assembly for keccak256
    "unaliased-plain-import",   # Plain imports without alias
    "screaming-snake-case-const", # Constant naming convention
    "mixed-case-variable",      # Variable naming convention
    "mixed-case-function",      # Function naming convention
    "unused-import"             # Unused imports
]

### ve33/hardhat.config.js

require("@nomiclabs/hardhat-waffle");
require('@openzeppelin/hardhat-upgrades');
require("@nomiclabs/hardhat-etherscan");
require("@nomiclabs/hardhat-web3");
require("hardhat-contract-sizer");
require("dotenv").config(); 

module.exports = {
  // Latest Solidity version
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
  solidity: {
    compilers: [
      {
        version: "0.8.13",
        settings: {
          optimizer: {
            enabled: true,
            runs: 1,
          },
          metadata: {
              useLiteralContent: true
          }
        },
      },
    ],
  },

  networks: {
    hyper: {
      url: "https://rpc.hyperliquid.xyz/evm",
      chainId: 999,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      gas: 30000000,
      blockGasLimit: 30000000,
      timeout: 1800000
    },
  },

  etherscan: {
    apiKey: `${process.env.APIKEY}`,
  },

  mocha: {
    timeout: 100000000,
  },
  contractSizer: {
    runOnCompile: true
},
};


### ve33/package.json

{
  "dependencies": {
    "@cryptoalgebra/integral-base-plugin": "~1.2.1",
    "@cryptoalgebra/integral-core": "~1.2.1",
    "@cryptoalgebra/integral-farming": "~1.2.1",
    "@cryptoalgebra/integral-periphery": "~1.2.1",
    "@openzeppelin/contracts": "^4.9.6",
    "@openzeppelin/contracts-upgradeable": "^4.8.0"
  }
}


### cl/foundry.toml

[profile.default]
src = "contracts"
test = "test"
out = "out"
libs = ["lib"]
solc_version = "0.7.6"

optimizer = true
optimizer_runs = 10  # Reduced to match Hardhat config and reduce contract size
bytecode_hash = "none"
cbor_metadata = false

fs_permissions = [{ access = "read-write", path = "./"}]

no_match_test = "testEchidna"

# See more config options https://github.com/foundry-rs/foundry/tree/master/config

[fuzz]
runs = 5000

[rpc_endpoints]
base_goerli = "${BASE_GOERLI_RPC_URL}"
base = "${BASE_RPC_URL}"
hyper = "${HYPEREVM_RPC_URL}"
hyper_test="${HYPER_TEST}"
local="http://localhost:8545"

[etherscan]
base_goerli = { key = "${BASE_GOERLI_ETHERSCAN_API_KEY}", url = "${BASE_GOERLI_ETHERSCAN_VERIFIER_URL}" }
base = { key = "${BASE_ETHERSCAN_API_KEY}", url = "${BASE_ETHERSCAN_VERIFIER_URL}" }
hyper = { key = "${HYPEREVM_ETHERSCAN_API_KEY}", url = "${HYPEREVM_ETHERSCAN_VERIFIER_URL}" }
hyper_test = { key = "${HYPEREVM_ETHERSCAN_API_KEY}", url = "${HYPEREVM_ETHERSCAN_VERIFIER_URL}" }

### cl/package.json

{
  "name": "@aerodrome-finance/slipstream",
  "description": "Core smart contracts of CL",
  "license": "BUSL-1.1",
  "publishConfig": {
    "access": "public"
  },
  "version": "1.0.1",
  "homepage": "https://aerodrome.finance",
  "keywords": [
    "uniswap",
    "core",
    "v3"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/aerodrome-finance/slipstream"
  },
  "files": [
    "contracts/core/interfaces",
    "contracts/core/libraries",
    "contracts/periphery/base",
    "contracts/periphery/interfaces",
    "contracts/peripherylibraries",
    "artifacts/contracts/core/CLFactory.sol/CLFactory.json",
    "artifacts/contracts/core/CLPool.sol/CLPool.json",
    "artifacts/contracts/core/interfaces/**/*.json",
    "!artifacts/contracts/core/interfaces/**/*.dbg.json",
    "artifacts/contracts/periphery/**/*.json",
    "!artifacts/contracts/periphery/**/*.dbg.json",
    "!artifacts/contracts/periphery/test/**/*",
    "!artifacts/contracts/periphery/base/**/*"
  ],
  "engines": {
    "node": ">=10"
  },
  "devDependencies": {
  },
  "scripts": {
    "compile": "hardhat compile",
    "test": "hardhat test",
    "test:all": "UPDATE_SNAPSHOT=1 yarn test",
    "format": "prettier --write 'test/**/*.ts'",
    "format:check": "prettier --check 'test/**/*.ts'"
  },
  "dependencies": {
    "@openzeppelin/contracts": "^3.4.2"
  }
}


### cl/remappings.txt

@ensdomains/=node_modules/@ensdomains/
@solidity-parser/=node_modules/solhint/node_modules/@solidity-parser/
ds-test/=lib/forge-std/lib/ds-test/src/
forge-std/=lib/forge-std/src/
hardhat/=node_modules/hardhat/
@openzeppelin/=lib/openzeppelin-contracts/
@nomad-xyz/=lib/ExcessivelySafeCall/
@uniswap/=lib/solidity-lib/
base64-sol/=lib/base64/


