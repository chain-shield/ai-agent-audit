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

{% hint style="info" %}
This fee is collected by the protocol, able to be withdrawn by the protocol's current `owner`.&#x20;

The owner of the protocol is [Ekubo Governance](https://docs.ekubo.org/user-guides/governance). As of January 2025, Ekubo Governance currently directs all protocol revenue towards [EKUBO buybacks](https://app.ekubo.org/governance/revenue-buybacks).

