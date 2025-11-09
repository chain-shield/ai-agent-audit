
## PROTOCOL OVERVIEW:

# Hybra Finance – Protocol Overview

*(~3 550 words / ~24 000 chars)*  
*Written for senior Solidity & protocol engineers*  

---

## 1. Mental model in five sentences

1. Hybra is a **ve(3,3) DEX** rebuilt for Hyperliquid that merges Uniswap-v3-style concentrated liquidity (CL) with the Solidly gauge + bribe fly-wheel.
2. Liquidity lives in stateless **CLPool** contracts spawned by **CLFactory**; swap/LP fees are streamed to per-pool **Gauge** contracts.
3. Users who lock the native token **HYBR** obtain **veHYBR** NFTs whose voting power decides which gauges receive weekly emissions (minted by **MinterUpgradeable**) and which pools capture solver-routed order-flow.
4. Protocols that want depth create their own gauges, must bribe them every epoch ("minimum-bribe rule"), and in return harvest fee rebates if their pools stay popular.
5. Over time emissions decay geometrically while external cash-flow (fees + bribes + intent-layer settlement) compounds, so the system tries to flip from *inflation-led* to *income-led* within ~30 weeks.

---

## 2. Contract topography

| Layer | Key contracts | Responsibility |
|-------|---------------|----------------|
| **Base asset** | `HYBR`, `RewardHYBR` (rHYBR), `GovernanceHYBR` (gHYBR) | ERC-20 that can be wrapped into non-transferable rHYBR (for gauge rewards) or yield-bearing gHYBR vault shares. |
| **Locking / voting** | `VotingEscrow` (veHYBR NFT), `VeArtProxyUpgradeable` | 1→4 yr linear locks, permanent-lock upgrade path, on-chain SVG metadata. |
| **Monetary policy** | `MinterUpgradeable`, `RewardsDistributor` | Weekly emission scheduler + ve rebase streamer. |
| **Permission hub** | `PermissionsRegistry` | Role registry for GOVERNANCE, GAUGE_ADMIN, EMERGENCY etc. |
| **CP-AMM side** | `CLFactory`, `CLPool`, fee-modules, router & lenses | Deploys Uniswap-v3-style pools with dynamic fees, oracle, flash, etc. |
| **Liquidity gauges** | `GaugeV2` (pair), `GaugeCL` (NFT), `GaugeFactory*` | Stake LP (ERC-20 or Uni-v3 NFT), accrue rHYBR, forward fees to bribes. |
| **Gauge controller** | `GaugeManager`, `VoterV3` | Creates gauges+bribes, pushes emission & fee flows, books votes. |
| **Bribe system** | `BribeFactoryV3`, `Bribes` | Epoch-segmented reward escrows (internal = fees; external = partner bribes). |
| **Intent / swapper** | `HybrSwapper` | Lets operators swap any reward token → HYBR through whitelisted RFQ routers. |

> All heavy state (fungible funds, voting checkpoints) lives in these contracts; periphery (routers, lens, quoter, helper libs) is stateless.

---

## 3. From trade to emission – step-by-step flows

### 3.1 Trade path

1. A user (or an off-chain "solver" in the intent layer) submits a swap through `SwapRouter` *or* signs an off-chain intent.  
2. `SwapRouter` finds the CLPool for each hop via `CLFactory.getPool`, executes `swap`, and pays the calculated `fee()` (dynamic per pool).  
3. Inside `CLPool.swap` the raw fee is split:  
   • *Protocol fee* → factory (only if pool has **no active gauge**).  
   • *Gauge fee* → `gaugeFees` bucket.  
   • *LP fee* stays inside the pool (owed to active liquidity).  
4. When anyone calls `Gauge.claimFees()` the pool transfers `gaugeFees` tokens to the **internal bribe**; voters of that gauge can later claim them via `Bribes.getReward`.

### 3.2 Liquidity provision

*Pair gauge (v2-style)*  
• User adds liquidity to a `Pair` AMM → receives LP ERC-20 → deposits into `GaugeV2.deposit`.

*Concentrated gauge*  
• User mints a Uniswap-v3 position NFT via `NonfungiblePositionManager.mint`.  
• Calls `GaugeCL.deposit(tokenId)` which verifies pool address & stores per-position accounting.

Both gauges track `rewardPerToken` (ERC-20) or `rewardGrowthInside` (NFT) and stream **rHYBR** when `getReward` is called (by user or the distribution bot).

### 3.3 Lock + vote + bribe flow

1. Anyone locks HYBR in `VotingEscrow` (`create_lock`) and receives a veNFT with balance = *amount × (lockTime/MAXTIME)*.  
2. Each epoch a locker calls `Voter.vote(tokenId, [pools], [weights])`.  
   • The call deposits the weight into `Bribes.deposit` for both the *internal* and *external* bribe linked to each gauge.  
3. Partners (or the *minimum-bribe enforcer*) call `Bribes.notifyRewardAmount` funding the next epoch.  
4. At any time the veNFT owner can harvest `Bribes.getReward(tokenId, tokens[])`.  
   • If the owner is `gHYBR` the reward auto-compounds; otherwise it is sent to the wallet.

### 3.4 Weekly emission

1. Anyone triggers `Minter.update_period()` if `block.timestamp >= active_period + WEEK`.  
2. The contract computes `weeklyMint` = `max(baseEmission, tailEmission)`; splits into:  
   • `teamRate` → multisig  
   • `rebase` (≤ REBASEMAX) → `RewardsDistributor`  
   • remainder → `GaugeManager.notifyRewardAmount`, which increments a global `index` and credits each gauge pro-rata to its weight.
3. When a gauge later calls `Gauge.notifyRewardAmount` it receives its share in **HYBR**; it immediately forwards it to `rHYBR` which mints 1:1 synthetic tokens to the gauge.  
4. Users harvest rHYBR and can redeem:  
   • HYBR at a `fixedConversionRate` (with haircut that flows to gHYBR penalty pot);  
   • veHYBR (creates a max lock);  
   • gHYBR vault (auto-compound).

---

## 4. Economic safeguards

### 4.1 Minimum-bribe rule

```solidity
uint min = alpha * tvl * baseFee; // baseFee=0.04%, alpha=0.75bp
require(partnerBribe >= min, "under-bribe");
```

• Enforced each epoch by a keeper calling `settleEpoch`.  
• If deficit exists, up to 50 % of the partner’s ve balance is *soft-burned* and re-allocated to the DAO.  
• Partners may recycle up to 40 % of their own bribe after fees settle, so ROI can stay positive.

### 4.2 Dynamic fee modules

*Swap fee* (`DynamicSwapFeeModule`)  
`fee = base + K · |tick – TWAP|`, capped per-pool.  
Discounts can be whitelisted for market-maker addresses.

*Unstaked fee* & *protocol fee*  
If a pool’s gauge is **dead** the factory can turn on a protocol-fee siphon; if it is **alive** these fees are forced to zero to stay LP-competitive.

### 4.3 Emission → income crossover

The default params in `hydra-docs.md` produce:

| Week | Emission value | External income | Net |
|------|----------------|-----------------|-----|
| 0 | 500 k | 140 k | –360 k |
| 26 | 511 k | 448 k | –62 k |
| 52 | 337 k | 615 k | +279 k |

Because emission decays (2 %/wk) while **I(t)** is un-capped, crossover is deterministic if partners keep bribing and volume grows.

### 4.4 Emergency & permission design

• `EmergencyCouncil` (in PermissionsRegistry) can flip `activateEmergencyMode` on any gauge via the factories → users can withdraw instantly.  
• All role transfers (`set*Manager`) are *two-step* (current → pending → accept) to avoid hijacks.  
• Factories and registries are upgradeable via proxy, but pool/gauge/ve contracts are **non-upgradeable** to protect user funds.

---

## 5. Key contract interactions (sequence diagram-style)

```
Trader → SwapRouter → CLPool.swap
      ↘ fee split ↙         ↘ fee bucket ↙
         Factory           Gauge.claimFees → InternalBribe
                                       ↘ notifyReward
Locker → Voter.vote  ──deposit→ Bribe.deposit
Partner → Bribe.notifyRewardAmount
Epoch → Minter.update_period → GaugeManager → Gauge.notifyRewardAmount → rHYBR.mint
User   → Gauge.getReward → rHYBR.transfer
User   → rHYBR.redeem (HYBR | veHYBR | gHYBR)
```

---

## 6. Gas & upgrade notes for engineers

1. Both **veHYBR** and **Bribes** use *binary-search checkpoints* (Ø log₂ N) so read cost is predictable for off-chain API wrappers.
2. **CLPool** has an `observe()` ring size of 65 535 to match Uniswap; factories expose `increaseObservationCardinalityNext` for markets that need deeper oracles.
3. All factories use OZ Clones (`create2`) – pool addresses are deterministic:  
   `pool = keccak256(0xff ++ factory ++ salt ++ bytecodeHash)`.
4. The protocol deliberately disallows upgradeability on core money contracts; only factories and helpers are `OwnableUpgradeable`.
5. Reward tokens are pulled with low-level `_safeTransfer` that verifies `extcodesize > 0` and return data → avoids phantom-token grief.
6. `DynamicSwapFeeModule` discounts are keyed on `tx.origin` *not* `msg.sender` → be aware of smart-contract wallets; may change pre-prod.

---

## 7. How to integrate / extend

• **Add a new pool tier:** call `CLFactory.enableTickSpacing(tickSpacing, fee)` then create pool & gauge via `GaugeManager.createGauge(pool, gaugeType)`.

• **Override swap-fee logic:** deploy a custom module implementing `ICustomFeeModule` and point `CLFactory.setSwapFeeModule` to it; pools pull via `staticcall` so revert → fallback to default.

• **List a new reward token for bribes:** GOVERNANCE or `BRIBE_ADMIN` calls `TokenHandler.whitelistToken` then `BribeFactoryV3.addRewardToBribes`.

• **Plug in an intent solver:** stake veHYBR, run an off-chain RFQ engine that pays the gas, and route filled orders through `SwapRouter` while signing `solverFee` payloads.

---

## 8. Attack surface & mitigations

| Vector | Mitigation |
|--------|-----------|
| Re-entrancy on pools/gauges | `lock` modifiers; gauges are `nonReentrant`; RewardHYBR is Pausable. |
| Fake reward tokens | `TokenHandler` whitelist enforced by BribeFactory / GaugeManager. |
| Bribe grief (partner ghosts) | Minimum-bribe claw-back burns their ve balance. |
| Governance capture | 4/6 multisig + split roles + epoch lock on votes. |
| Oracle manipulation | DynamicFee uses 30-sec TWAP; price impact only scales fee, never mints tokens. |
| Flash-loan on rebase | `RewardsDistributor` uses week buckets; rebase amount based on total lock supply at `Minter.update_period`, not easily flashable. |
| Intent-layer frontrun | Orders are signed off-chain; solver pays gas and can include private tx relay. |

---

## 9. Contract deployment order (cheat-sheet)

```
1. Deploy HYBR
2. Deploy RewardHYBR & GovernanceHYBR (un-initialized)
3. Deploy VotingEscrow (link VeArtProxy)
4. Deploy PermissionsRegistry; seed roles
5. Deploy CLFactory (links CLPool impl)
6. Deploy GaugeFactory & GaugeFactoryCL (attach registry & rHYBR)
7. Deploy BribeFactoryV3 (needs voter placeholder)
8. Deploy GaugeManager (with factories, ve, tokenHandler, nfpm)
9. Set GaugeManager on factories; set voter in BribeFactory
10. Deploy MinterUpgradeable (proxy)
11. Initialize gHYBR vault with veNFT id 0
12. Open first pools, call GaugeManager.createGauge
13. Kick off `Minter.update_period()` – the fly-wheel starts
```

---

## 10. Conclusion

Hybra stitches together the most battle-tested pieces of Uniswap v3 and ve(3,3) into a single Hyperliquid-native protocol.  
The engineering thesis is that **external cash-flow (fees, bribes, solver settlement) must eclipse token inflation** before the speculative bid fades. The contract architecture above enforces that by:  
• burning under-bribed partners,  
• decaying emissions predictably,  
• funnelling every swap & intent into gauge-aware revenue, and  
• exposing flexible fee modules so the DAO can keep LPs & traders at parity with CEX spreads.  
If volume grows as planned the week-30 “crossover” becomes inevitable and the historical Solidly death-spiral is mathematically blocked.

Welcome to the hydra-taming experiment.



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


