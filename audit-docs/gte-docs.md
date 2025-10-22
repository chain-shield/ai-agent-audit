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


