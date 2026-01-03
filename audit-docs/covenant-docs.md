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
