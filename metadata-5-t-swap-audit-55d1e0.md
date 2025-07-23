
## SUMMARY OF FILE: 5-t-swap-audit/src/TSwapPool.sol
### TSwapPool Contract Summary

**Contract Overview:**
`TSwapPool` is an ERC20 token contract managing a pool that facilitates swapping between WETH and another pool token. The contract allows liquidity providers to deposit and withdraw liquidity, and users to swap tokens.

**Contract Details:**
- **Errors:** Defined for issues such as deadline expiry, deposit limitations, and invalid tokens.
- **State Variables:** Include immutable tokens (`i_wethToken`, `i_poolToken`), constants for minimum deposit, and swap count.
- **Modifiers:** `revertIfDeadlinePassed` checks if a deadline is unmet, `revertIfZero` ensures amounts are non-zero.

**Functions Overview:**
- **Constructor:** Initializes with addresses for pool and WETH tokens and ERC20 details.
  ```solidity
  constructor(address poolToken, address wethToken, string memory liquidityTokenName, string memory liquidityTokenSymbol)
  ```
- **Deposit:** Adds liquidity subject to constraints like minimum deposit and maximum pool tokens.
  ```solidity
  function deposit(uint256 wethToDeposit, uint256 minimumLiquidityTokensToMint, uint256 maximumPoolTokensToDeposit, uint64 deadline)
  ```
- **_addLiquidityMintAndTransfer:** Mints liquidity tokens and transfers deposits.
  ```solidity
  function _addLiquidityMintAndTransfer(uint256 wethToDeposit, uint256 poolTokensToDeposit, uint256 liquidityTokensToMint)
  ```
- **Withdraw:** Removes liquidity, ensuring withdrawal meets specified minimums.
  ```solidity
  function withdraw(uint256 liquidityTokensToBurn, uint256 minWethToWithdraw, uint256 minPoolTokensToWithdraw, uint64 deadline)
  ```
- **Get Pricing Methods:** Calculate swap pricing and required inputs/outputs.
- **Swap Methods:** Execute swaps via `swapExactInput` or `swapExactOutput`, ensuring token validity.
  ```solidity
  function swapExactInput(IERC20 inputToken, uint256 inputAmount, IERC20 outputToken, uint256 minOutputAmount, uint64 deadline)
  ```
  ```solidity
  function swapExactOutput(IERC20 inputToken, IERC20 outputToken, uint256 outputAmount, uint64 deadline)
  ```
- **Sell Pool Tokens:** Sell pool tokens for WETH.
  ```solidity
  function sellPoolTokens(uint256 poolTokenAmount)
  ```
- **Utility Functions:** Include methods to get token prices, current reserves, and total liquidity.

**State Variables:**
- `IERC20 private immutable i_wethToken;` An immutable reference to the WETH token contract.
- `IERC20 private immutable i_poolToken;` An immutable reference to the pool token contract.
- `uint256 private constant MINIMUM_WETH_LIQUIDITY = 1_000_000_000;` A constant specifying the minimum liquidity deposit for WETH.
- `uint256 private swap_count = 0;` Tracks the number of swaps, resets after reaching `SWAP_COUNT_MAX`.
- `uint256 private constant SWAP_COUNT_MAX = 10;` A constant to determine when to give incentives after a certain number of swaps.


## SUMMARY OF FILE: 5-t-swap-audit/src/PoolFactory.sol
### PoolFactory Contract
The PoolFactory contract is a smart contract that manages the creation and tracking of TSwapPool contracts that are associated with ERC20 tokens. Its main functionalities include creating new TSwapPools and providing mapping between tokens and pools.

#### State Variables
- **s_pools**: `mapping(address token => address pool)` - Maps an ERC20 token address to its respective TSwapPool address.
- **s_tokens**: `mapping(address pool => address token)` - Maps a TSwapPool address back to its respective ERC20 token.
- **i_wethToken**: `address` - An immutable variable storing the WETH token address.

#### Constructor
- **constructor(address wethToken)**: Initializes the contract with the address of the WETH token.

#### Functions
- **createPool(address tokenAddress) external returns (address)**: Checks if a pool for the given token already exists. If not, it creates a new TSwapPool with a generated liquidity token name and symbol, and adds it to the mappings.
- **getPool(address tokenAddress) external view returns (address)**: Returns the TSwapPool address linked to the specified token.
- **getToken(address pool) external view returns (address)**: Retrieves the token address for a specified TSwapPool address.
- **getWethToken() external view returns (address)**: Returns the configured WETH token address.


## Main List of Files in Project

script/DeployTSwap.t.sol
src/PoolFactory.sol
src/TSwapPool.sol
test/unit/PoolFactoryTest.t.sol
test/unit/TSwapPool.t.sol

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

