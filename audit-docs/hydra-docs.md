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

