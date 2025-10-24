
## PROTOCOL OVERVIEW:

# Covenant Protocol — Technical Overview

## 1  Purpose & High-Level Idea

Covenant is an on-chain credit marketplace that turns *any* ERC-20 collateral (ETH, WBTC, stETH, sUSDe, …) into a continuously-clearing lending market.  
Instead of the classical "pool of stable-coins + interest-rate set by governance + liquidation cascades" design, Covenant:

1. Splits every unit of collateral that is deposited into two fungible ERC-20s:  
   • **Yield Coin** (`zToken`, “debt”) – a fully collateralised, zero-coupon bond whose *price* implies its continuously-compounding APY.  
   • **Leverage Coin** (`aToken`, “equity”) – the residual leveraged exposure to the same collateral that keeps whatever is **left** after the Yield Coin notional is honoured.
2. Makes those two tokens, plus the underlying collateral, swappable along a **concave AMM curve** called **Latent Swap**.  The curve hard-wires the relationship between:
   • Yield-Coin price  ↔  implied funding-rate  ↔  market Loan-to-Value (LTV)
3. As a consequence, supply and demand of leverage *self-balance* – high LTV pushes the debt price down → APY rises → lenders enter; low LTV does the opposite.
4. No liquidations, no expiries, no governance-set rates.

## 2  Contract Architecture

```
           ┌────────────────────────┐
           │  CovenantCurator       │  ← governor wires assets→oracle mapping
           └─────────┬──────────────┘
                     │IPriceOracle interface
   ┌─────────────────┴─────────────────┐
   │           Oracles                 │  (pull or push)
   │  • ChainlinkOracle (push)         │
   │  • PythOracle      (pull)         │
   │  • CrossAdapter    (compose)      │
   └─────────────────┬─────────────────┘
                     │
┌────────────────────┴────────────────────┐
│         Covenant Core (not in repo)     │
│  – user entry for deposit/mint/swap      │
│  – talks to LatentSwapLEX                │
└────────────────────┬────────────────────┘
                     │ILiquidExchangeModel
┌────────────────────┴────────────────────┐
│          LatentSwapLEX.sol              │
│  – AMM math, state, fees                │
│  – deploys SynthToken (a/z)             │
└────────────────────┬────────────────────┘
                     │mint/burn only
┌───────────────┬────┴─────┬──────────────┐
│  SynthToken   │          │             │
│  (ERC20, one  │          │             │
│   per market) │          │             │
└───────────────┘          │
                     read-only helpers
               ┌─────┴──────┐
               │DataProvider│
               └────────────┘
```

### 2.1  CovenantCurator
Oracle router + ERC-4626 un-wrapper.  Owner (governor) maps `(base,quote)` → oracle, adds fallback oracle and vault-to-asset resolution.  Exposes:
• `getQuote / getQuotes` (state changing – may call Pull oracle)  
• `previewGetQuote / previewGetQuotes` (view)  
• `updatePriceFeeds` & `getUpdateFee` – forwards ETH where needed.

### 2.2  Oracle adapters
Common interface from Euler-Vault-Kit.  Implementations shipped:
* **ChainlinkOracle** – push; no fee; preview only.  
* **PythOracle** – pull; validates staleness, confidence, fee forwarding.  
* **CrossAdapter** – composes two oracles through a *cross* asset (e.g. ETH→USD + ETH→BTC gives BTC→USD).

### 2.3  LatentSwapLEX
Single contract that *models every market*.  Immutable parameters chosen once at deployment (price bands, debt duration, swap fee, etc.).  For each new market `initMarket()` is called; this:
* Stores oracle address, quote token, protocol fee.
* Deploys two **SynthToken** contracts (aToken & zToken) with deterministic salts.
* Initialises **LexState** (sqrtPrice, debt notional, supply).

AMM functions exposed to **Covenant Core** only: `mint`, `redeem`, `swap`, `updateState` plus corresponding pure **quote** versions for front-ends.

Internal math lives in `/libraries/*`:  
`LatentMath`, `SqrtPriceMath`, `DebtMath`, `SaturatingMath`, `FixedPoint`, `Uint512`.

### 2.4  SynthToken
Minimal ERC-20 with `lexMint/lexBurn` restricted to the authorising Lex (LatentSwapLEX).  immutable metadata: marketId, synthType (0 = debt, 1 = leverage), decimals.  Transfers are standard ERC-20.

### 2.5  Off-chain & Periphery
* `DataProvider.sol` – view aggregator that fetches market params, LEX state, computes current LTV, debt discount, spot prices, token meta.  Zero storage or owner.
* An (un-included) Covenant Core coordinates user flows, calls LEX then moves collateral tokens.
* sUSDz Yield-Fund (not in repo) pools Z-tokens.

## 3  Core Mechanics in Detail

### 3.1  Market lifecycle
1. Governor whitelists `(base, quote)` oracle on **Curator**.
2. Core `createMarket(base, quote, params)` → `LatentSwapLEX.initMarket()`.  Passes:
   • edge sqrt prices *Pa/Pb* (liquidity band)  
   • debtDuration (e.g. 90 days)  
   • protocolFee bps  
   • swapFee bps  
   • lnRateBias (initial discount so debt starts at target APY)
3. LEX deploys `aToken` + `zToken`; initial sqrtPrice sits mid-band so `LTV = target`.

### 3.2  Depositing collateral → Mint
User deposits *B* units of Base Asset into Core.  Core forwards to LEX `mint` with `MintParams` specifying how much of the output they want as debt vs leverage (or just pro-rata).  LEX:
* Calculates oracle price (may need a Pyth update: collects fee).  
* Computes amountsout along AMM invariant.  
* Mints `aToken`, `zToken` to user addresses.  
* Charges `protocolFee` (portion of swap fee that gets skimmed for DAO, destined for `_covenantCore`).

### 3.3  Redeem
Holder of any of the three assets (`a`, `z`, Base) can convert to another via `redeem` or `swap`:
* `redeem` burns `a` and/or `z` and releases Base.  
* `swap` can transform (Base ↔ z, Base ↔ a, a ↔ z) directly, paying swap fee.

Because the invariant is **concave** (vs XY=K convex), the spot value of `a+z` normally exceeds pool collateral; that spread is what gives the system room to pay fees & oracle costs without breaking redeemability.

### 3.4  Latent Swap Invariant (intuition)
```
(V_L - L/√P_a) · (V_Y - L√P_b) = L² ;    L = √(P_a·P_b) / (√P_b - √P_a)
```
where `V_L` = value of leverage side, `V_Y` = value of debt side, `P_a … P_b` = price band edges.  
For practical minds: the closer the market LTV gets to band edges the steeper the curve, i.e. debt becomes cheaper (price↓, APY↑) when `V_Y` is scarce.

#### Derived relationships
Loan-to-Value:  `LTV = V_Y / V_B`  
Effective leverage for `aToken`:  `Lev = 1 / (1 – LTV)`  
Implied variable rate for `zToken`:  `r = -ln(P_t / D)` where *D* is `debtDuration` constant.

### 3.5  Funding Flow
*Borrower* side (`aToken` holder) implicitly **pays** funding because the notional of `zTokens` grows versus a fixed collateral pile.  No explicit transfer – instead their share of collateral is diluted when they redeem later.  
*Lender* side (`zToken` holder) earns that funding; their token’s redemption value vs par rises over time.

The magnitude is path-independent: if the `zToken` stays priced at `0.95` (5 % discount to par) the APY will be ~5 % / `debtDuration`-in-years continuously compounded.

### 3.6  No Liquidations – Why It Works
Because the market always holds *100 % collateral* and `a` holders are the first-loss tranche, the system cannot become under-collateralised unless Base price goes to 0 instantaneously.  There is never a point where the protocol needs to force sell collateral – it simply allows the `aToken` price to fall (can reach 0 in theory).

### 3.7  Example Walk-Through
1. Market starts with 0 collateral.  Alice deposits 100 ETH valued at $350k.  She chooses to *lend* → she receives 100 zETH priced at 0.95 (implied APY ≈ 5 %).  There are no `aToken`s yet; LTV=0.
2. Bob comes and wants 3x long ETH.  He deposits 50 ETH and swaps 50 ETH → zETH to *borrow* (debt) then swaps the obtained zETH for more `aETH`.  After swap LTV climbs to 75 %; implied APY on zETH rises to 9 % and Bob’s effective leverage is 4×.
3. Carol sees juicy 9 % APY, deposits stables into the sUSDz fund that buys zETH, pushing price up → APY drifts back to 7 %, LTV down to 70 %.
4. ETH rallies +10 %.  `aETH` NAV grows ~4× = +40 %.  zETH principal also grows 7 % annualised but far less than the price move.  Bob realises gains by swapping `aETH` back to ETH and withdrawing collateral.

## 4  Oracle Flow & Pull-Oracle Fees
Pull oracles (Pyth) require the caller to pre-pay the update fee in ETH.  Any state-changing function in LEX that needs a fresh price will:
1. Call Curator `getUpdateFee()` with `updateData`.  
2. Add that fee to `protocolFees` it is about to collect from the user.  
3. Forward the fee in `msg.value` to Curator → Oracle.

Design-choice: *users, not protocol*, pay oracle fees, maintaining solvency.

## 5  Fee Model
• **swapFee** – spread collected by LEX each time the curve is touched (default few bps).  
• **protocolFee** – share of swapFee flows to DAO treasury (config per market).  
• **oracleFee** – external cost, forwarded 1:1.

Fees are minted into `baseToken` supply or withheld from amounts out; Debt & Lev token accounting always reconciles via AMM math.

## 6  Security & Trust Assumptions
1. **Immutables** – LatentSwapLEX core parameters and oracle addresses are fixed per market at creation; no upgradeable pattern, minimal admin.
2. **Governance keys** (Ownable)
   • Curator owner: can change oracle mappings & fallback – oracle risk.  
   • LEX owner: can tweak *noCapLimit* (per-token exemptions) and metadata overrides – low criticality.
3. **No asset custody** in oracle layer; only Core holds collateral.
4. **Math libraries** use 512-bit intermediate products, saturating casts, overflow checks.
5. **Pull oracle fee mismatch** reverts, preventing griefing by malicious `msg.sender` underfunding call.
6. SynthToken mint/burn strictly gated to LEX address.

## 7  Extensibility
• **New collateral** → deploy oracle adapter (or wire existing), call Core.createMarket.  No code change.  
• **Cross asset markets** (e.g., stETH priced in BTC) achievable via CrossAdapter.
• **Yield Funds** can hold baskets of `zToken`s, presenting ERC-4626 front-ends (sUSDz).  Under the hood same redeemability guarantees.

## 8  Repository Quick Stats
Files ≈ 60; main contracts in `src/`.  0.8.30 compiler, Cancun EVM, OZ 5.3.0, Foundry test infra.  Licence BUSL-1.1.  Total repo LOC ≈ 8 k.

---
**In one sentence:** Covenant replaces liquidations and governance-set rates with a mathematically-coupled AMM and a pair of fungible tokens so that leverage demand, lender supply, and funding rates equilibrate themselves on-chain.


## Main List of Files in Project

src/Covenant.sol
src/curators/CovenantCurator.sol
src/curators/lib/Errors.sol
src/curators/oracles/BaseAdapter.sol
src/curators/oracles/CrossAdapter.sol
src/curators/oracles/chainlink/ChainlinkOracle.sol
src/curators/oracles/pyth/PythOracle.sol
src/lex/latentswap/LatentSwapLEX.sol
src/lex/latentswap/libraries/DebtMath.sol
src/lex/latentswap/libraries/FixedPoint.sol
src/lex/latentswap/libraries/LSErrors.sol
src/lex/latentswap/libraries/LatentMath.sol
src/lex/latentswap/libraries/LatentSwapLogic.sol
src/lex/latentswap/libraries/SaturatingMath.sol
src/lex/latentswap/libraries/SqrtPriceMath.sol
src/lex/latentswap/libraries/TokenData.sol
src/lex/latentswap/libraries/Uint512.sol
src/libraries/Errors.sol
src/libraries/Events.sol
src/libraries/MarketParams.sol
src/libraries/MultiCall.sol
src/libraries/NoDelegateCall.sol
src/libraries/SafeMetadata.sol
src/libraries/Utils.sol
src/libraries/ValidationLogic.sol
src/synths/SynthToken.sol


 ## DOCUMENTATION: 

 ### covenant-docs.md

# What is Covenant?

Covenant is a fully on‑chain and permissionless credit marketplace that matches **leverage** **users** (borrowers) with **yield seekers** (lenders).

Leverage has proven strong product-market fit in crypto: traders borrow to take levered long positions on volatile assets, or to run profitable carry trades on yield-bearing assets. Covenant provides a transparent, market-driven way to fund this activity:

1. for any crypto asset;
2. without relying on centrally- or governance-set interest rates;
3. without requiring a stablecoin liquidiy pool; and
4. without liquidations.

The Covenant Protocol coordinates leverage users (borrowers) with margin providers (lenders) for a given Covenant Market (e.g., ETH, WBTC, stETH, sUSDe) using two core instruments which can be minted or acquired:

* **Leverage Coins:** provide leveraged exposure to the Base Asset’s price and (where applicable) its native yield. Holders continuously pay funding to Margin Coin holders at a rate implied by the Margin Coin’s market price.
* **Yield Coins:** fully‑collateralized debt claims minted when a Base Asset is deposited into a Covenant Market. Margin Coins operate like tradeable perpetual, zero-coupon bonds with interest accrued at the rate implied by the Margin Coin’s market price.

These instruments are fully collateralized and redeemable to the Base Asset at all times.

Within each Covenant Market, funding and risk (leverage) are self‑balancing. The Covenant Protocol’s Latent Swap AMM couples Margin Coin price, Margin Coin interest rate, and the Covenant Market’s LTV:

* **When LTV rises** (high leverage demand / low margin supply), Margin Coin prices fall, implied rates rise, and new lenders are drawn in—reducing LTV.
* **When LTV falls** (low leverage demand / abundant margin supply), Margin Coin prices rise, implied rates fall, and lenders withdraw—raising LTV.

This mechanism makes Covenant operate like a perpetual, transparent margin lending exchange, where funding and leverage continuously balance.

To participate, lenders can either hold a specific market’s Yield Coin (i.e. lend to a specific Covenant Market / Base Asset) or, for supported assets, supply to a diversified yield pool, known as the **Covenant sUSDz Yield Fund**, represented by the receipt token **$sUSDz**.


```markdown
# Glossary

A list of terms commonly used in Covenant's documentation and their corresponding descriptions.

| Term | Description |
|------|--------------|
| **Base Asset** | The underlying collateral (e.g., ETH, WBTC, stETH, sUSDe) deposited into a Covenant Market. |
| **Covenant Market** | A self-contained market within the Covenant Protocol that transforms a deposited Base Asset (e.g., ETH, WBTC, stETH, sUSDe) into two fungible claims: Margin Coins and Leverage Coins. <br><br>Each Covenant Market is defined by:<ol><li>a specific Base Asset;</li><li>a price oracle (denominating that asset in a quote unit);</li><li>a debt duration parameter (used in the Perpetual Debt mechanism);</li><li>a min / max market price (concentrating liquidity in the Latent Swap AMM).</li></ol><br>All Yield Coins and Leverage Coins in a given Covenant Market are fully collateralized and redeemable back to the Base Asset at any time.<br><br>Prices and interest rates in each Covenant Market are continuously set by the Latent Swap AMM, which couples Yield Coin price, Yield Coin implied interest rate, and market LTV so that funding and leverage remain self-balancing rather than fixed by governance or managed via forced liquidations. |
| **Covenant Protocol** | The set of smart contracts and mechanisms that power the Covenant credit marketplace. The Covenant Protocol is responsible for:<ul><li>Collateral management: Accepting deposits of Base Assets into Covenant Markets and minting fully collateralized Margin Coins and Leverage Coins.</li><li>Interest rate setting (via Covenant’s Perpetual Debt mechanism): Implementing the Perpetual Debt mechanism, where the price of Margin Coins continuously determines their implied interest rate and value accrual.</li><li>Market clearing (Latent Swap AMM): Operating a convex swap curve that links Margin Coin price, market LTV, and leverage, so that funding and risk are self-balancing without governance-set rates or per-position liquidations.</li><li>Redemptions: Ensuring that Margin Coins and Leverage Coins are always redeemable back to the underlying Base Asset at market rates.</li><li>Optional pooling ($sUSDz): Supporting pooled lending products (e.g., the Covenant sUSDz Margin Fund, $sUSDz) that diversify exposure across multiple Covenant Markets.</li></ul><br>In short, the Covenant Protocol is the on-chain infrastructure that transforms any Base Asset into a continuously clearing lending market, matching leverage users (borrowers) with margin providers (lenders). |
| **Latent Swap** | Covenant’s automated market maker (AMM) that governs swaps between Margin Coins, Leverage Coins, and the underlying Base Asset within a Covenant Market. |
| **Leverage Coin** | Leverage Coins provide leveraged exposure to the Base Asset’s price and (where applicable) native yield. In exchange for this leverage, Leverage Coin holders implicitly pay funding to Margin Coin holders at the APY implied by the Margin Coin price.<br><br>Issuance/pricing is regulated by the Latent Swap invariant so leverage becomes progressively more expensive as LTV rises.<br><br>Leverage Coins are fungible ERC20 claims redeemable to the Base Asset on a per-market basis (not pooled across assets). |
| **Yield Coin** | A fully-collateralized debt minted by a Covenant Market when a Base Asset is deposited.<br><br>A Yield Coin’s interest accrues continuously and is implied by its market price via Covenant’s Perpetual Debt Mechanism.<br><br>Yield Coins are fungible ERC20 redeemable to the Base Asset on a per-market basis (not pooled across collateral assets). |
| **sUSDz Yield Fund** | The Covenant sUSDz Yield Fund is an optional pooled lending product that diversifies lender exposure across multiple Covenant Markets with USD as their quote token.<br><br>Lenders deposit supported assets into the fund and receive $sUSDz, a fungible ERC20 receipt token representing their share of the fund’s NAV.<br><br>The fund allocates capital by holding a portfolio of Yield Coins across different markets, so $sUSDz holders are effectively long a diversified set of yield exposures.<br><br>This structure provides a simple way for lenders to participate in the Covenant marketplace without needing to select and manage exposure to individual Base Asset markets. |
| **sUSDz Margin Fund Receipt Token ($sUSDz)** | Participating lenders deposit supported assets to the sUSDz Margin Fund and receive $sUSDz, a receipt token that tracks their share of the sUSDz Margin Fund’s NAV.<br><br>$sUSDz is a fungible ERC20 claim redeemable against the Base Assets in the sUSDz Margin Fund. |
| **Perpetual Debt** | A financial primitive in the Covenant Protocol that makes Yield Coins work. Yield Coins are designed as a continually refinancing zero-coupon bond (i.e., Perpetual Debt): instead of expiring at a fixed maturity, the Yield Coin’s notional balance automatically grows or shrinks over time at a rate implied by its market price. |
```



# How does Covenant work?

## 1. Covenant Markets

The Covenant Protocol operates Covenant Markets where a deposited Base Asset (e.g., ETH, WBTC, stETH, sUSDe) is split into two fungible claims:

* **Leverage Coins** provide holders with leveraged exposure to the Base Asset’s price and (where applicable) its native yield. Holders continuously pay funding to Yield Coin holders at a rate implied by the Yield Coin’s market price.
* **Yield Coins** are fully‑collateralized debt claims minted when a Base Asset is deposited into a Covenant Market. Yield Coins operate like tradeable perpetual, zero-coupon bonds with interest accrued at the rate implied by the Yield Coin’s market price.

## 2. Latent Swap AMM

Prices and interest in a given Covenant Market are set continuously by a built‑in, concave swap curve (known as the Latent Swap).

Traditional AMMs (like Uniswap) need both sides of a trading pair deposited as liquidity. Covenant is different: the protocol itself controls all Yield Coins, Leverage Coins, and Base Asset collateral. That means it can create a swap invariant that reflects the balance between Leverage Coins and Yield Coins rather than needing external LPs. The goal:

* Allow continuous swaps among Yield Coins, Leverage Coins, and Base Assets.
* Tie swap prices directly to the system’s Loan-to-Value (LTV) and therefore to the implied funding rate.
* Ensure leverage becomes more expensive as LTV rises (and cheaper as LTV falls).

The Latent Swap invariant is a concave curve defined over the *at target* values of Leverage Coints, Yield Coins, and Base Tokens:

\left(V_L - L/\sqrt{P_a}\right)\left(V_Y- L\sqrt{P_b}\right)=L^2

L=\frac{\sqrt{P_aP_b}}{\sqrt{P_b}-\sqrt{P_a}}

Where:

* $$V\_L$$ =  value of **Leverage Coins** in the market ( *at target*)
* $$V\_Y$$ = notional value of **Yield Coins** in the market
* $$V\_B$$ = value of the **Base Asset**
* $$P\_a, P\_b$$ = price band parameters that concentrate liquidity within a defined range

Covenant’s Latent Swap AMM means that Covenant Markets are self‑balancing:

* **When LTV rises** (high leverage demand / low margin supply), Yield Coin prices fall, implied rates rise, and new lenders are drawn in—reducing LTV.
* **When LTV falls** (low leverage demand / abundant margin supply), Yield Coin prices rise, implied rates fall, and lenders withdraw—raising LTV.

LTV for a given market is defined as total value of Yield Coins outstanding in the market divided by total mark‑to‑market value of the Base Asset collateral in that market, i.e.:

$$
LTV = \frac{V\_Y}{V\_B}
$$

## 3. Leverage Coins

Leverage Coins provide leveraged exposure to the Base Asset’s price and (where applicable) native yield. In exchange for this leverage, Leverage Coin holders implicitly pay funding to Yield Coin holders at the APY implied by the Yield Coin price.

The value of all Leverage Coins in a given Covenant Market, B, is (approximately) the residual of the market’s Base Asset collateral value after accounting for the notional value of Margin Coins:

V_L \approx V_B-V_Y

where $$V\_L$$ = total Leverage Coin value,   $$V\_B$$ =  total Base Asset value and $$V\_Y$$ =  total Yield Coin value.

*Note: we say ‘approximately’ above as this accounting equation only holds at the margin. In practice, because Margin Coins and Leverage Coins are priced continuously along the (concave) Latent Swap curve, the spot values of Margin and Leverage Coins combined will usually exceed the total Base Asset value in the collateral pool. The exact relationship among Margin Coin value, Leverage Coin value, and Base Asset collateral value is governed by the Latent Swap invariant, which ensures the market clears consistently.*

The application of Leverage Coins is most easily conceptualized by examining use with different Base Asset types:

1. Volatile, non-yield-bearing assets (e.g. ETH, WBTC); and
2. Yield-bearing assets (e.g. stETH, sUSDe).

| Base Asset Type                                                | Leverage Coin effect                                              | Return drivers                                        |
| -------------------------------------------------------------- | ----------------------------------------------------------------- | ----------------------------------------------------- |
| <ol><li>Volatile non-yield bearing assets (ETH, BTC)</li></ol> | Amplified price exposure                                          | Directional price movement vs funding costs           |
| <ol start="2"><li>Yield-bearing asset (stETH, sUSDe)</li></ol> | Leveraged carry on native yield + price exposure (where relevant) | (Collateral yield × leverage) – funding + price moves |

### 3.1 **Amplified price exposure: Use of Leverage Coins with volatile non-yield bearing Base Assets**&#x20;

Holding a Leverage Coin gives leveraged directional exposure to the Base Asset’s price.

* If the Base Asset goes up +1%, Leverage Coin value rises by more than +1%
* Effective leverage = 1 / (1 – LTV)
* If the Base Asset goes down –1%, the loss is amplified the same way.

Because Leverage Coin holders are implicitly paying funding to Yield Coin holders, they need asset price appreciation to outpace the funding costs (sometimes referred to as *Funding Drag*). The implicit funding cost borne by Leverage Coin holders isn’t a separate payment, but rather a dilution of the Leverage Coin’s share of the Base Asset collateral in a given Covenant Market.

To use a simple example, consider a Covenant Market with $100 in collateral:

* $80 represented by Yield Coins,
* $20 represented by Leverage Coins.

If the implied debt APY is, say, 10%, then over the next year the notional owed to Yield Coin holders will compound upward. Unless the collateral also grows in value (via price appreciation or native yield), that compounding comes out of the Leverage Coin’s slice.

* If collateral stays flat at $100, Yield Coin value increases (say, to $88), leaving only $12 for Leverage Coin holders.
* That $8 erosion is the funding cost being “felt” by Leverage Coin holders.

### 3.2 **Leveraged carry (plus price exposure):** Use of Leverage Coins with yield bearing assets

In this case, Leverage Coins not only track Base Asset price but also capture its native yield with leverage.

For example, if the LTV in a given Covenant Market is 80% (implying 5x effective leverage) Leverage Coin holders in this market will capture the yield (and price movements) of an underlying yield-bearing asset times 5, less the funding paid to Margin Coin holders.

This setup makes Leverage Coins behave like a tokenized carry instrument. Net carry is:

(Collateral Yield × Effective Leverage) − (Funding Cost)

In “cheap funding” environments (low Margin Coin APY), Leverage Coin holders can earn strong positive carry.

In “tight funding” environments (high Margin Coin APY), net carry may be negative, and returns depend more on Base Asset price moves.

Of course, Leverage Coin holders in Covenant Markets with yield bearing Base Assets are also subject to the Funding Drag phenomenon detailed in 2.1.3.1.

## 4. Perpetual Debt (Yield Coins)

Yield Coins in the Covenant Protocol operate according to a financial primitive known as Perpetual Debt. Perpetual Debt is designed as a continually refinancing zero-coupon bond: instead of expiring at a fixed maturity, the Yield Coin’s notional balance automatically grows or shrinks over time at a rate implied by its market price.

The price of a Yield Coin on the market $$P\_t$$ determines its implied funding rate $$r\_t$$ :

$$
r\_t=-\ln{ \left(P\_t / D \right)}
$$

where $$r\_t$$ is the instantaneous rate and D is a “duration” constant. Under this formulation, it is intuitive that lower Margin Coin prices result in higher implied Yield Coin funding rates and higher Yield Coin price result in lower implied Yield Coin funding rates.

Perpetual Debt eliminates fixed maturities and governance-set rates. It allows Covenant to run continuous, liquid credit markets for any Base Asset where funding rates are set entirely by Margin Coin prices.

## 5. The sUSDz Yield Fund

An optional pooled lending product within the Covenant Protocol that allows margin providers (lenders) to gain diversified exposure across multiple Covenant Markets without needing to manage each Base Asset market individually.

* **How it works:** Lenders supply supported assets to the sUSDz Yield Fund. In return, they receive a receipt token ($sUSDz) that represents their share of the fund’s net asset value (NAV).
* **What it holds:** The sUSDz Yield Fund allocates capital across different Covenant Markets by holding a portfolio of Yield Coins denominated in USD. This diversification spreads exposure across many Base Assets and interest-rate environments.&#x20;
* **Why it exists:** For lenders who don’t want to pick a single Base Asset market (DIY approach), the sUSDz Yield Fund provides a simple way to participate in Covenant’s credit marketplace while gaining broader, protocol-level risk and yield exposure.



 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.9.2",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/euler-price-oracle/package.json

{
  "name": "euler-price-oracle",
  "description": "Euler Price Oracles is a library of minimal and immutable oracle adapters. Contracts in this library implement `IPriceOracle`, an opinionated quote-based interface for price oracles. To understand how Price Oracles fit into the [Euler Vault Kit](https://github.com/euler-xyz/euler-vault-kit), read the [EVK whitepaper.](https://docs.euler.finance/euler-vault-kit-white-paper/#price-oracles)",
  "version": "1.0.0",
  "keywords": [],
  "author": "Euler Labs",
  "license": "GPL-2.0-or-later",
  "dependencies": {

### lib/aave-v3-core/package.json

{
  "name": "@aave/core-v3",
  "version": "1.16.2-beta.5",
  "description": "Aave Protocol V3 core smart contracts",
  "files": [
    "contracts",
    "artifacts",
    "types"

### lib/pyth-sdk-solidity/package.json

{
  "name": "@pythnetwork/pyth-sdk-solidity",
  "version": "2.2.0",
  "description": "Read prices from the Pyth oracle",
  "repository": {
    "type": "git",
    "url": "git+https://github.com/pyth-network/pyth-sdk-solidity.git"
  },

### lib/solady/package.json

{
  "name": "solady",
  "license": "MIT",
  "version": "0.1.23",
  "description": "Optimized Solidity snippets.",
  "files": [
    "src/**/*.sol",
    "js/**/*"

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.3.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.3.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true,
  "dependencies": {
    "minimatch": "^3.1.2"
  }
}


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = 'src'
out = 'out'
libs = ['lib']
gas_reports = ["*"]

evm_version = 'cancun'
optimizer = true
optimizer_runs = 3750
fs_permissions = [{ access = "read", path = "./script/"}]

## Sets the concrete solc version to use, this overrides the `auto_detect_solc` value
solc = '0.8.30'
auto_detect_solc = false

## Sets revert-based tests to be executed in a backward compatible manner for recent forge versions
allow_internal_expect_revert = true

[fuzz]
runs = 10000
max_test_rejects = 60000
max_test_case_size = 512 # Optional, adjust input size if needed
fuzz_max_shrink_time = 0 # Unlimited shrinking time
failure_persistence = true

[profile.forkfuzz.fuzz]
runs = 1000
max_test_rejects = 60000

[profile.coverage.fuzz]
runs = 100
max_test_rejects = 60000

[profile.intense.fuzz]
verbosity = 3
runs = 100000
max_test_rejects = 600000

# Remappings in remappings.txt

# See more config options https://github.com/gakonst/foundry/tree/master/config


### package.json

{
    "name": "covenant-liquid-v1",
    "version": "1.0.0",
    "description": "Covenant liquid core protocol",
    "author": "Covenant Labs",
    "license": "BUSL-1.1",
    "scripts": {
        "build": "forge build",
        "clean": "rm -rf out/ cache/",
        "compile": "forge compile",
        "test": "forge test",
        "coverage": "forge coverage",
        "lint": "npm run lint:ts && npm run lint:sol",
        "lint:fix": "npm run lint:ts:fix && npm run lint:sol:fix",
        "lint:ts": "eslint '*/**/*.{js,ts}'",
        "lint:ts:fix": "eslint '*/**/*.{js,ts}' --fix",
        "lint:sol": "solium --dir ./contracts",
        "lint:sol:fix": "solium --dir ./contracts --fix",
        "prettier": "npm run prettier:sol",
        "prettier:sol": "prettier --write 'src/**/*.sol' && prettier --write 'test/**/*.sol' & prettier --write 'script/**/*.sol'"
    },
    "dependencies": {
        "prettier": "^3.4.2"
    }
}


### remappings.txt

@std=lib/forge-std/src/
@clones=lib/clones-with-immutable-args/src/
@chainlink/=lib/chainlink-brownie-contracts/
@openzeppelin/=lib/openzeppelin-contracts/contracts
@solady=lib/solady/src/
@aave/=lib/aave-v3-core/contracts/protocol/
@euler-price-oracle/=lib/euler-price-oracle/src/
aave-v3-core/=lib/aave-v3-core/
forge-std/=lib/forge-std/src/
@openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/
chainlink-brownie-contracts/=lib/chainlink-brownie-contracts/contracts/src/v0.6/vendor/@arbitrum/nitro-contracts/src/
clones-with-immutable-args/=lib/clones-with-immutable-args/src/
ds-test/=lib/clones-with-immutable-args/lib/ds-test/src/
erc4626-tests/=lib/openzeppelin-contracts/lib/erc4626-tests/
halmos-cheatcodes/=lib/openzeppelin-contracts/lib/halmos-cheatcodes/src/
openzeppelin-contracts/=lib/openzeppelin-contracts/
solady/=lib/solady/src/


