**NOTE**: ALL privileged roles are TRUSTED. Any Finding that requires role is Low/Information UNLESS Impact is HIGH.

### Arbitrage Opportunities

The system behaves like a DEX (where collateral, leverage and yield tokens can be swapped for each other).

We care that there are no arbitrage opportunities (more value taken out than going in) under a circular set of trades upto a `10e20` precision. 

However, we are ok if `value_out` is bigger than `value_in` for smaller amounts (e.g, if `10e21` of a token goes in, then `10e21 + 1` can come out).

### Administrative Risks

Admin risks (if admin is taken over, or if admin misconfigures oracles / latentswap limits, etc) are out of scope. 

### Fee Handling

The protocol is ok with small miscalculations or underaccruals of fees (that are in favour of the protocol instead of the users). Specifically, there are design choices to accrue small amounts probabilistically that could be manipulated by validators, but we chose to accept this risk. Fees that accrue for users are in scope.

There are some edge cases for the last user leaving a market where fees can be front-run. As documented in code, this risk is out scope.

### Covenant + LatentSwap Governance

Once a market is created, the only thing governance can do is pause/unpause the market, and change mint/redeem caps. The risk that governance keys are compromised and markets are paused (and held hostage) is out of scope, or the risk that caps are removed.

The risk of governances keys being compromised and potential consequences of incorrect or fraudulent governance actions is out of scope. This includes invalid actions by governance (e.g. creating a market with a non-working oracle, etc).

### Curator (oracle) Governance

Covenant governance can approve a curator (oracle router) and subsequently this cannot be changed for a market.  However, Curator governance can change where the router points to, potentially affecting market behaviour.  The risk of curators misconfiguring a live market is out of scope.

### Oracle Misbehaviours

The Covenant protocol assumes the price from the Oracle (as governed by curators) can be trusted, and does not add additional checks. The oracle contracts themselves are a direct copy of those created for Euler, specifically for Chainlink (push) + Pyth (pull). The DEX effectively prices around the oracle price, and hence DEX actions can compensate (up-to a point) for oracle mispricing or lags. Given this, volatile assets (e.g., ETH, BTC, MON) are priced through pull feeds from Pyth, similar to a Perp DEX (Chainlink streams being implemented but not in scope for this audit), whereas more stable assets (e.g. USDC / USDT) being priced through Chainlink.  The Covenant system itself is semi-permissionless, so we do not check or stop curators from using slow oracles for volatile assets (governance risk).  These markets can be created, but users will quickly see that other markets are more liquid and have less slippage vs these markets and gravitate to those.

In addition, if approved by Curator governance, ERC4262 prices will be used as a price source, and mispricing by a Governance approved ERC4262 is out of scope.

### Large Market Size

We expect Covenant markets to be based on token assets with reasonable FDVs and total token mint amounts, with reasonable pricing from Oracles. We have built some protections for markets that are really big - e.g., where the FDV is bigger than 2^256 when measured in quote token decimal units. Or when markets become big over time. Specifically, we are using saturating multiplications at times which allow markets not to lock - but change market functionality in the following ways: 1) yield tokens might not accrued more yield after a point (>1000 years for most valid markets) 2) it will not be possible to mint new leverage tokens or yield tokens because their amount is over 2^256, etc. In these situations, we want users to be able to unwind their positions (and stop using that market instance) - but acknowledge that pricing might be off and interest might not be accruing in some edge cases.

### Market Parameterization

Market parameters are tested with limits upon creation, and only markets within these limits are in scope.

### Irregular ERC20s

Irregular ERC20s are out of scope (e.g, fee-on transfer, rebasing).  However, older ERC20s with slightly different return interfaces (e.g, bytes32 for name, no success bool for transfers) are in scope.  E.g, DAI, USDT, SNX.

### In-Code Comments

Any in-code comments supercede the functionality outlined in the project's documentation, and may reflect additional design choices that have not been explicitly outlined in this chapter and will be considered out of scope.

# Overview

Covenant enables markets for the use and funding of leverage against any collateral asset, using the collateral itself as liquidity. This facilitates the permissionless creation of structured products and unlocks a 10x increase in DeFi liquidity.

## Links

- **Previous audits:**  https://github.com/pashov/audits/blob/master/team/pdf/Covenant-security-review_2025-08-18.pdf
- **Documentation:** https://docs.covenant.finance
- **Website:** https://covenant.finance/
- **X/Twitter:** https://x.com/covenantFi

---

# Scope

### Files out of scope

| File         |
| ------------ |
| [script/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/script) |
| [src/curators/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/curators/interfaces) |
| [src/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/interfaces) |
| [src/lex/latentswap/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/lex/latentswap/interfaces) |
| [src/periphery/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/periphery) |
| [test/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/test) |
| Totals: 51 |


# Additional context

## Areas of concern (where to focus for bugs)

### LatentSwap

- Is the LatentSwap invariant upheld at all times for atomic transactions?
- Can anyone mint / redeem / swap more tokens than allowed to (and extract value from the system, up-to a 10^-20 precision)?
- Can one user bring in a large amount of base assets (up-to allowed limits), and through a series of mint / swap / redeem actions extract value from the system (up-to a 10^-20 precision)?

### Covenant

- Does the Covenant contract keep base assets between markets siloed and independent of each other (i.e., one market cannot take out base assets from another)?
- Can a malformed ERC20 or oracle external contract re-enter the market to extract value?
- Can a Covenant market be bricked (in situations where the collateral FDV is in an appropriate range)?

## Main invariants

### LatentSwap Invariant

This invariant sets the relationship between the notional value of yield tokens, the value of leverage tokens, and the value of base assets. This is based on the following two formulas:

$$
\left(\frac{L}{\sqrt{P_a}} - V_{yield}\right )\left(L\sqrt{P_b} - V_{leverage}\right)=L^2
$$
where
$$
L = V_c \frac{\sqrt{P_a P_b}}{\sqrt{P_b}-\sqrt{P_a}}
$$

Where $P_b$, $P_a$ are the min / max price of the market (price edges). $V_c$ is the collateral value, $V_{yield}$ the yield coin notional value, and $V_{leverage}$ the leverage coin value.  The formula above is very similar to a Uniswap V3 concentrated liquidity invariant between two prices $p_a$ and $p_b$.

When initialized, markets are position at target LTV and have a price of 1.  In this instance, $V_{collateral} = V_{yield} + V_{leverage}$.  However, this equality is not an invariant, it is just a spot case of the previous equation. What does hold however, is that $V_{collateral} <= V_{yield} + V_{leverage}$. 


## All trusted roles in the protocol

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Covenant Governance                          | Can pause markets, and add trusted LEX markets and curators (oracles)                |
| Covenant Pause                             | Per market, this address can pause the market (and transfer this permission)                       |
| LEX Governance                             | Can set some market parameters, including mint / redeem limits                       |
| Curator Governance                             | Can add / modify / remove oracles                       |

