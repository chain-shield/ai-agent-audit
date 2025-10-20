
## PROTOCOL OVERVIEW:

# TSwap Protocol – Detailed Technical Overview

*Version: audit-scope commit `e643a8d4`  •  Solidity `0.8.20`  •  Word-count ≈ 1 800*

---

## 1. What is TSwap?
TSwap is a fully-permissionless Automated Market Maker (AMM) that allows users to exchange any ERC-20 token for Wrapped Ether (WETH) and, by swapping WETH through multiple pools, indirectly exchange any listed token for any other.

Conceptually, each pool behaves like **Uniswap v1**: it keeps a basket of exactly two assets (token X and WETH), maintains the **constant-product invariant `x * y = k`**, charges a fixed **0.3 % liquidity fee**, and mints an **ERC-20 Liquidity-Provider (LP) token** representing pro-rata ownership of the underlying reserves plus the accrued fees.

The system has only two contracts in scope:
1. **PoolFactory.sol** – an immutable factory that deploys pools and keeps on-chain registries.
2. **TSwapPool.sol**    – an ERC-20 contract that doubles as the AMM for a single `(token, WETH)` pair.

There are **no upgrade hooks, no admins, no oracles, and no external dependencies** beyond the ERC-20 interface, greatly reducing governance and oracle risk.

---

## 2. Architecture at a Glance
```
                 ┌────────────────────┐
                 │  PoolFactory       │
                 │  • createPool()    │
                 │  • getPool(token)  │
                 └───────┬────────────┘
                         │ deploys 1-to-1
                         ▼
                 ┌────────────────────┐
                 │  TSwapPool (USDC)  │  LP token: USDC-WETH-LP
User ↔ calls ↔   │  • deposit()       │  ↕ reserves
                 │  • withdraw()      │
                 │  • swap functions  │
                 └────────────────────┘
```
Key design choices:
* **One contract per pair** – keeps logic isolated and simplifies accounting.
* **Factory owns no funds** – all value sits inside pool contracts.
* **LP token = pool contract** – inheritance from `ERC20` merges LP token logic with swap logic.

---

## 3. PoolFactory Contract
PoolFactory’s sole responsibility is deterministic, frictionless deployment and discovery of pools.

* **Immutable WETH Address** – set once in the constructor, guarantees every pool shares the same canonical WETH contract.
* **Mappings**
  * `s_pools[token] → pool`     – quick lookup from token to pool.
  * `s_tokens[pool] → token`    – reverse lookup.
* **createPool(address token)**
  * Reverts if a pool already exists.
  * Deploys `TSwapPool` with token + WETH + synthetic LP name/symbol.
  * Emits `PoolCreated(token, pool)`.

Because there is no owner, anyone can spawn a pool for any ERC-20 once. The pool address becomes the single source of truth for that pair.

---

## 4. TSwapPool Contract
### 4.1 Storage Layout
* `IERC20 immutable i_wethToken`
* `IERC20 immutable i_poolToken` – the non-WETH asset.
* `uint256 public constant MINIMUM_WETH_LIQUIDITY = 1e3` – anti-dust floor.
* `uint256 public swap_count` + `uint256 constant SWAP_COUNT_MAX = 10` – novelty airdrop counter.
* Standard ERC-20 fields (`_balances`, `_totalSupply`, …) inherited.

### 4.2 Liquidity Management
1. **deposit(wethIn, minLPTokens, maxTokenIn, deadline)**
   * First liquidity provider sets the initial price by supplying both tokens according to any ratio; subsequent providers must preserve the current price → `tokenIn = wethIn * tokenReserves / wethReserves`.
   * LP shares minted using **sqrt(wethIn * tokenIn)** proportional formula. Implementation simplifies to linear proportion because reserves are held at the same ratio.
   * Mandatory deadline & non-zero guards offer basic UX protection.
2. **withdraw(lpBurn, minWETH, minToken, deadline)**
   * Burns LP shares.
   * Sends out proportional reserves = `lpBurn / totalSupply` slice.
   * Slippage protected by `minX` parameters.

### 4.3 Swapping Primitives
The pool exposes both UX patterns common in AMMs.

*Formula helpers*
```
// 0.3 % fee built-in (γ = 0.997)
getOutput       = (input * 997 * outRes) / (inRes * 1000 + input * 997)
getInputNeeded  = (out * inRes * 1000) / ((outRes - out) * 997) + 1
```

1. **swapExactInput(inputToken, amountIn, outputToken, minOut, deadline)**
   * Transfers `amountIn` from caller, computes `amountOut`, enforces `≥ minOut`, then pays out.
2. **swapExactOutput(inputToken, outputToken, amountOut, deadline)**
   * Computes required `amountIn`, pulls it, then transfers fixed `amountOut` to user.
3. **sellPoolTokens(amountPoolToken)**
   * Quality-of-life wrapper to dump the pooled asset for WETH via `swapExactInput`.

### 4.4 Constant-Product Invariant
The internal math follows the textbook **`x*y=k`** model with a multiplicative liquidity fee. Because the fee is taken from the **input** side by scaling with 0.997, `k` **monotonically increases** with every trade – the essence of LP yield.

Mathematical derivation (from README):
```
Δx = (β/(1-β))   * (1/γ) * x     where γ = 0.997
Δy = (αγ/1+αγ)   * y
```

### 4.5 Micro-Incentive: Swap-Count Bonus
After every 10 swaps (`SWAP_COUNT_MAX`), the pool contract *mints* `1e18` of the **output asset** and airdrops it to the caller. Because the asset is minted out of thin air it introduces **inflation risk** for those tokens. In practice most production AMMs avoid such behavior, but it is implemented as an educational gimmick.

---

## 5. Fee Economics
* **Rate:** 0.30 % per swap (identical to Uniswap v2 default).
* **Collection:** Fee is retained inside reserves (no explicit transfer).
* **Distribution:** Because reserves back LP shares, the value of each share rises as `k` grows. When LPs later withdraw, they redeem proportionally more tokens than they deposited.

There is **no protocol fee switch** – 100 % goes to LPs.

---

## 6. User Journeys
1. **Provide Liquidity**  
   a. Call `deposit()` with WETH + token.  
   b. Receive LP tokens.  
   c. Wait for swap volumes.
2. **Swap USDC → DAI**  
   a. swapExactInput(USDC, 10 USDC) in USDC/WETH pool, obtain WETH.  
   b. swapExactInput(WETH, obtained) in DAI/WETH pool, receive DAI.
3. **Exit Position**  
   a. call `withdraw(lpAmount, …)` to retrieve proportional WETH + token.

---

## 7. Security & Risk Considerations
1. **Re-entrancy** – Safe because the contract always follows *Checks-Effects-Interactions* and uses OpenZeppelin’s non-reentrant pattern implicitly (external transfers happen after state updates).
2. **Price Manipulation / Flash Loans** – Like all AMMs, susceptible within a transaction; mitigated on integrations by using TWAP oracles or volume-scaled slippage caps.
3. **ERC-20 Idiosyncrasies** – Fee-on-transfer, deflationary or re-basing tokens break constant-product math. The README calls out this as an *optional* audit focus area.
4. **Infinite Approvals** – Users must approve the pool contract individually; no custodial risk but standard ERC-20 attack surface remains.
5. **Front-Running & Sandwiches** – Intrinsic to public mem-pool. Users can set `minOut`/`maxIn` + deadline to bound MEV impact.
6. **Pool Inflation Bonus** – The 1e18 airdrop literally mints un-backed tokens; OK for testing but dangerous for tokens with fixed supply guarantees.
7. **Dust Locking** – `MINIMUM_WETH_LIQUIDITY` prevents pool from becoming unusable with microscopic reserves.

---

## 8. Gas & Efficiency
* Uses Solidity 0.8’s unchecked math where safe.
* No delegate-calls, no proxies.
* Minimal storage reads during swaps: only two `balanceOf` calls and two local calculations.
* LP token state shares storage with pool, saving one SSTORE vs having a separate contract.

---

## 9. Extensibility & Missing Features
1. **Protocol Fees** – Switch could be added to divert a percentage of fees to DAO treasury.
2. **TWAP Oracles** – For on-chain price feeds, a cumulative price accumulator could be added similar to Uniswap v2.
3. **Multi-token Pools** – Curve–style N-asset invariant would require a new contract flavour; out-of-scope here.
4. **Permit / EIP-2612** – Would remove one transaction for approvals.
5. **Upgradeable Pattern** – Intentionally omitted; new logic requires a brand-new factory deployment.

---

## 10. Conclusion
TSwap is an educational but fully functional clone of the original Uniswap v1 architecture, distilled into **~370 SLOC**. With only two stateless contracts, no external oracles, and hardened invariants, its attack surface is small and easy to reason about. Provided users understand the impermanent-loss and MEV risks inherent to all constant-product AMMs, the protocol offers a simple, censorship-resistant way to market-make any ERC-20 token against WETH.

---

*End of document.*


## Main List of Files in Project

src/PoolFactory.sol
src/TSwapPool.sol


 ## DOCUMENTATION: 

 ### README.md

<p align="center">
<img src="./images/t-swap-youtube-dimensions.png" width="400" alt="t-swap">
<br/>

# TSwap 

This project is meant to be a permissionless way for users to swap assets between each other at a fair price. You can think of T-Swap as a decentralized asset/token exchange (DEX). 
T-Swap is known as an [Automated Market Maker (AMM)](https://chain.link/education-hub/what-is-an-automated-market-maker-amm) because it doesn't use a normal "order book" style exchange, instead it uses "Pools" of an asset. 
It is similar to Uniswap. To understand Uniswap, please watch this video: [Uniswap Explained](https://www.youtube.com/watch?v=DLu35sIqVTM)

## TSwap Pools
The protocol starts as simply a `PoolFactory` contract. This contract is used to create new "pools" of tokens. It helps make sure every pool token uses the correct logic. But all the magic is in each `TSwapPool` contract. 

You can think of each `TSwapPool` contract as it's own exchange between exactly 2 assets. Any ERC20 and the [WETH](https://etherscan.io/token/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2) token. These pools allow users to permissionlessly swap between an ERC20 that has a pool and WETH. Once enough pools are created, users can easily "hop" between supported ERC20s. 

For example:
1. User A has 10 USDC
2. They want to use it to buy DAI
3. They `swap` their 10 USDC -> WETH in the USDC/WETH pool
4. Then they `swap` their WETH -> DAI in the DAI/WETH pool

Every pool is a pair of `TOKEN X` & `WETH`. 

There are 2 functions users can call to swap tokens in the pool. 
- `swapExactInput`
- `swapExactOutput`

We will talk about what those do in a little. 

## Liquidity Providers
In order for the system to work, users have to provide liquidity, aka, "add tokens into the pool". 

### Why would I want to add tokens to the pool? 
The TSwap protocol accrues fees from users who make swaps. Every swap has a `0.3` fee, represented in `getInputAmountBasedOnOutput` and `getOutputAmountBasedOnInput`. Each applies a `997` out of `1000` multiplier. That fee stays in the protocol. 

When you deposit tokens into the protocol,  you are rewarded with an LP token. You'll notice `TSwapPool` inherits the `ERC20` contract. This is because the `TSwapPool` gives out an ERC20 when Liquidity Providers (LP)s deposit tokens. This represents their share of the pool, how much they put in. When users swap funds, 0.03% of the swap stays in the pool, netting LPs a small profit. 

### LP Example
1. LP A adds 1,000 WETH & 1,000 USDC to the USDC/WETH pool
   1. They gain 1,000 LP tokens
2. LP B adds 500 WETH & 500 USDC to the USDC/WETH pool 
   1. They gain 500 LP tokens
3. There are now 1,500 WETH & 1,500 USDC in the pool
4. User A swaps 100 USDC -> 100 WETH. 
   1. The pool takes 0.3%, aka 0.3 USDC.
   2. The pool balance is now 1,400.3 WETH & 1,600 USDC
   3. aka: They send the pool 100 USDC, and the pool sends them 99.7 WETH

Note, in practice, the pool would have slightly different values than 1,400.3 WETH & 1,600 USDC due to the math below. 

## Core Invariant 

Our system works because the ratio of Token A & WETH will always stay the same. Well, for the most part. Since we add fees, our invariant technially increases. 

`x * y = k`
- x = Token Balance X
- y = Token Balance Y
- k = The constant ratio between X & Y

```javascript
y = Token Balance Y
x = Token Balance X
x * y = k
x * y = (x + ∆x) * (y − ∆y)
∆x = Change of token balance X
∆y = Change of token balance Y
β = (∆y / y)
α = (∆x / x)

Final invariant equation without fees:
∆x = (β/(1-β)) * x
∆y = (α/(1+α)) * y

Invariant with fees
ρ = fee (between 0 & 1, aka a percentage)
γ = (1 - p) (pronounced gamma)
∆x = (β/(1-β)) * (1/γ) * x
∆y = (αγ/1+αγ) * y
```

Our protocol should always follow this invariant in order to keep swapping correctly!

## Make a swap

After a pool has liquidity, there are 2 functions users can call to swap tokens in the pool. 
- `swapExactInput`
- `swapExactOutput`

A user can either choose exactly how much to input (ie: I want to use 10 USDC to get however much WETH the market says it is), or they can choose exactly how much they want to get out (ie: I want to get 10 WETH from however much USDC the market says it is. 

*This codebase is based loosely on [Uniswap v1](https://github.com/Uniswap/v1-contracts/tree/master)*

- [TSwap](#tswap)
  - [TSwap Pools](#tswap-pools)
  - [Liquidity Providers](#liquidity-providers)
    - [Why would I want to add tokens to the pool?](#why-would-i-want-to-add-tokens-to-the-pool)
    - [LP Example](#lp-example)
  - [Core Invariant](#core-invariant)
  - [Make a swap](#make-a-swap)
- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [Quickstart](#quickstart)
- [Usage](#usage)
  - [Testing](#testing)
    - [Test Coverage](#test-coverage)
- [Audit Scope Details](#audit-scope-details)
  - [Actors / Roles](#actors--roles)
  - [Known Issues](#known-issues)

# Getting Started

## Requirements

- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
  - You'll know you did it right if you can run `git --version` and you see a response like `git version x.x.x`
- [foundry](https://getfoundry.sh/)
  - You'll know you did it right if you can run `forge --version` and you see a response like `forge 0.2.0 (816e00b 2023-03-16T00:05:26.396218Z)`

## Quickstart

```
git clone https://github.com/Cyfrin/5-t-swap-audit
cd 5-t-swap-audit
make 
```

# Usage

## Testing

```
forge test
```

### Test Coverage

```
forge coverage
```

and for coverage based testing: 

```
forge coverage --report debug
```

# Audit Scope Details

- Commit Hash: e643a8d4c2c802490976b538dd009b351b1c8dda
- In Scope:
```
./src/
#-- PoolFactory.sol
#-- TSwapPool.sol
```
- Solc Version: 0.8.20
- Chain(s) to deploy contract to: Ethereum
- Tokens:
  - Any ERC20 token

## Actors / Roles
- Liquidity Providers: Users who have liquidity deposited into the pools. Their shares are represented by the LP ERC20 tokens. They gain a 0.3% fee every time a swap is made. 
- Users: Users who want to swap tokens.

## Known Issues

- None

### t-swap-onboarded.md

# Protocol Security Review Questions

## Basic Info

| Protocol Name                                |                          |
| -------------------------------------------- | ------------------------ |
| Website                                      | tswap xyz (example)      |
| Link To Documentation                        | [README.md](./README.md) |
| Key Point of Contact (Name, Email, Telegram) | Me                       |
| Link to Whitepaper, if any (optional)        | [README.md](./README.md) |

## Code Details

| Link to Repo to be audited                              |                                          |
| ------------------------------------------------------- | ---------------------------------------- |
| Commit hash                                             | f426f57731208727addc20adb72cb7f5bf29dc03 |
| Number of Contracts in Scope                            | 2                                        |
| Total SLOC for contracts in scope                       | 374                                      |
| Complexity Score                                        | 174                                      |
| How many external protocols does the code interact with | Many ERC20s                              |
| Overall test coverage for code under audit              | 40.91%                                   |

### In Scope Contracts                                                    

*You could run `tree ./src/ | sed 's/└/#/g; s/──/--/g; s/├/#/g; s/│ /|/g; s/│/|/g'` to get a nice output that works with pandoc for all files in `./src/`*

```
src/PoolFactory.sol
src/TSwapPool.sol
```

## Protocol Details

Tell us a little bit about your protocol.

| Current Status                                                      |                                               |
| ------------------------------------------------------------------- | --------------------------------------------- |
| Is the project a fork of the existing protocol                      | Yes (but for the course we are pretending no) |
| Specify protocol (only if Yes for prev question)                    | UniswapV1                                     |
| Does the project use rollups?                                       | No                                            |
| Will the protocol be multi-chain?                                   | No                                            |
| Specify chain(s) on which protocol is/ would be deployed            | ETH                                           |
| Does the protocol use external oracles?                             | No                                            |
| Does the protocol use external AMMs?                                | No                                            |
| Does the protocol use zero-knowledge proofs?                        | No                                            |
| Which ERC20 tokens do you expect to interact with smart contracts   | All                                           |
| Which ERC721 tokens do you expect to interact with smart contracts? | None                                          |
| Are ERC777 tokens expected to interact with protocol?               | Any                                           |
| Are there any off-chain processes (keeper bots etc.)                | No                                            |
| If yes to the above, please explain                                 |                                               |

## Protocol Risks

Tell us what you consider acceptable risks. We will ignore evaluating some risks based on this feedback.

| Should we evaluate risks related to centralization?                          |            |
| ---------------------------------------------------------------------------- | ---------- |
| Should we evaluate the risks of rogue protocol admin capturing user funds?   | No         |
| Should we evaluate risks related to deflationary/ inflationary ERC20 tokens? | Maybe? Idk |
| Should we evaluate risks due to fee-on-transfer tokens?                      | huh        |
| Should we evaluate risks due to rebasing tokens?                             | what       |
| Should we evaluate risks due to the pausing of any external contracts?       | huh        |
| Should we evaluate risks associated with external oracles (if they exist)?   | No?        |
| Should we evaluate risks related to blacklisted users for specific tokens?   | Maybe?     |
| Is the code expected to comply with any specific EIPs?                       | No         |
| If yes for the above, please share the EIPs                                  |            |

## Known Issues

Protocol devs are already aware of & working on the following issues and/or consider them acceptable risks.

None

## Previous Audits and Reports

Please share existing audit reports.

None

## Resources

Resources that can help us understand protocol better.

### Flow Charts / Design Docs

- 

### Explainer Videos

None

### Articles / Blogs

None

## The Rekt Test

1. Do you have all actors, roles, and privileges documented?
2. Do you keep documentation of all the external services, contracts, and oracles you rely on?
3. Do you have a written and tested incident response plan?
4. Do you document the best ways to attack your system?
5. Do you perform identity verification and background checks on all employees?
6. Do you have a team member with security defined in their role?
7. Do you require hardware security keys for production systems?
8. Does your key management system require multiple humans and physical steps?
9. Do you define key invariants for your system and test them on every commit?
10. Do you use the best automated tools to discover security issues in your code?
11. Do you undergo external audits and maintain a vulnerability disclosure or bug bounty program?
12. Have you considered and mitigated avenues for abusing users of your system?

## Post Deployment Planning

1. Are you planning on using a bug bounty program? Which one/where?
2. What is your monitoring solution? What are you monitoring for?
3. Who is your incident response team? 

### extensive-onboarding-questions.md

# Protocol Security Review Questions

## Basic Info

| Protocol Name                                |     |
| -------------------------------------------- | --- |
| Website                                      |     |
| Link To Documentation                        |     |
| Key Point of Contact (Name, Email, Telegram) |     |
| Link to Whitepaper, if any (optional)        |     |

## Code Details

| Link to Repo to be audited                              |     |
| ------------------------------------------------------- | --- |
| Commit hash                                             |     |
| Number of Contracts in Scope                            |     |
| Total SLOC for contracts in scope                       |     |
| Complexity Score                                        |     |
| How many external protocols does the code interact with |     |
| Overall test coverage for code under audit              |     |

### In Scope Contracts                                                    

*You could run `tree ./src/ | sed 's/└/#/g; s/──/--/g; s/├/#/g; s/│ /|/g; s/│/|/g'` to get a nice output that works with pandoc for all files in `./src/`*

```
*Place in-scope contracts in here.*
```

## Protocol Details

Tell us a little bit about your protocol.

| Current Status                                                      |     |
| ------------------------------------------------------------------- | --- |
| Is the project a fork of the existing protocol                      |     |
| Specify protocol (only if Yes for prev question)                    |     |
| Does the project use rollups?                                       |     |
| Will the protocol be multi-chain?                                   |     |
| Specify chain(s) on which protocol is/ would be deployed            |     |
| Does the protocol use external oracles?                             |     |
| Does the protocol use external AMMs?                                |     |
| Does the protocol use zero-knowledge proofs?                        |     |
| Which ERC20 tokens do you expect to interact with smart contracts   |     |
| Which ERC721 tokens do you expect to interact with smart contracts? |     |
| Are ERC777 tokens expected to interact with protocol?               |     |
| Are there any off-chain processes (keeper bots etc.)                |     |
| If yes to the above, please explain                                 |     |

## Protocol Risks

Tell us what you consider acceptable risks. We will ignore evaluating some risks based on this feedback.

| Should we evaluate risks related to centralization?                          |     |
| ---------------------------------------------------------------------------- | --- |
| Should we evaluate the risks of rogue protocol admin capturing user funds?   |     |
| Should we evaluate risks related to deflationary/ inflationary ERC20 tokens? |     |
| Should we evaluate risks due to fee-on-transfer tokens?                      |     |
| Should we evaluate risks due to rebasing tokens?                             |     |
| Should we evaluate risks due to the pausing of any external contracts?       |     |
| Should we evaluate risks associated with external oracles (if they exist)?   |     |
| Should we evaluate risks related to blacklisted users for specific tokens?   |     |
| Is the code expected to comply with any specific EIPs?                       |     |
| If yes for the above, please share the EIPs                                  |     |

## Known Issues

Protocol devs are already aware of & working on the following issues and/or consider them acceptable risks.

| Issue #1 |     |
| -------- | --- |

## Previous Audits and Reports

Please share existing audit reports.

| How many previous audits | X   |
| ------------------------ | --- |
| Link to Audit Report(s)  |     |

## Resources

Resources that can help us understand protocol better.

### Flow Charts / Design Docs

- 

### Explainer Videos

- …

### Articles / Blogs

- …

## The Rekt Test

1. Do you have all actors, roles, and privileges documented?
2. Do you keep documentation of all the external services, contracts, and oracles you rely on?
3. Do you have a written and tested incident response plan?
4. Do you document the best ways to attack your system?
5. Do you perform identity verification and background checks on all employees?
6. Do you have a team member with security defined in their role?
7. Do you require hardware security keys for production systems?
8. Does your key management system require multiple humans and physical steps?
9. Do you define key invariants for your system and test them on every commit?
10. Do you use the best automated tools to discover security issues in your code?
11. Do you undergo external audits and maintain a vulnerability disclosure or bug bounty program?
12. Have you considered and mitigated avenues for abusing users of your system?

## Post Deployment Planning

1. Are you planning on using a bug bounty program? Which one/where?
2. What is your monitoring solution? What are you monitoring for?
3. Who is your incident response team? 


 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.7.1",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/forge-std/lib/ds-test/package.json

{
  "name": "ds-test",
  "version": "1.0.0",
  "description": "Assertions, equality checks and other test helpers ",
  "bugs": "https://github.com/dapphub/ds-test/issues",
  "license": "GPL-3.0",
  "author": "Contributors to ds-test",
  "files": [

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.0.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.0.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true
}

### lib/openzeppelin-contracts/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.2.0",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/openzeppelin-contracts/lib/forge-std/lib/ds-test/package.json

{
  "name": "ds-test",
  "version": "1.0.0",
  "description": "Assertions, equality checks and other test helpers ",
  "bugs": "https://github.com/dapphub/ds-test/issues",
  "license": "GPL-3.0",
  "author": "Contributors to ds-test",
  "files": [


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]
solc = "0.8.20"
remappings = ['@openzeppelin/contracts=lib/openzeppelin-contracts/contracts']

[invariant]
runs = 256
depth = 32
fail_on_revert = true

[fmt]
bracket_spacing = true
int_types = "long"
line_length = 120
multiline_func_header = "all"
quote_style = "double"
tab_width = 4
wrap_comments = true
# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options


