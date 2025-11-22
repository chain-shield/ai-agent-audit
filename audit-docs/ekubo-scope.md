ALL PRIVILEGED ROLES in protocol are TRUSTED.  
- PRIVILEGED ACTORS CANNOT ACT MALICIOUSLY.
- MISTAKES by PRIVILEGED ACTORS are considered 
governance risk (NOT security vulnerability).

**NOTE**: This protocol uses solidity version `0.8.31`

## Publicly known issues

_Anything included in this section is considered a publicly known issue and is therefore ineligible for awards._

### Compiler Vulnerabilities

**NOTE**: Any vulnerabilities that pertain to the experimental nature of the `0.8.31` pre-release candidate and the project's toolkits are considered out-of-scope for the purposes of this contest.

### Non-Standard EIP-20 Assets

Tokens that have non-standard behavior e.g. allow for arbitrary calls may not be used safely in the system.

Token balances are only expected to change due to calls to `transfer` or `transferFrom`.

Any issues related to non-standard tokens should only affect the pools that use the token, i.e. those pools can never become insolvent in the other token due to non-standard behavior in one token.

### Extension Freezing Power

The extensions in scope of the audit are **not** expected to be able to freeze a pool and lock deposited user capital.

Third-party extensions, however, can freeze a pool and lock deposited user capital. This is considered an acceptable risk.

### TWAMM Guarantees

TWAMM order execution quality is dependent on the liquidity in the pool and orders on the other side of the pool. 

If any of the following conditions are true:

- Liquidity in the pool is low
- The other side has not placed orders
- Blocks are not produced for a period of time

The user may receive a bad price from the TWAMM. This is a known risk; the TWAMM order execution price is not guaranteed.

# Overview

Ekubo Protocol delivers the best pricing using super-concentrated liquidity, a singleton architecture, and extensions. The Ekubo protocol vision is to provide a balance between the best swap execution and liquidity provider returns. The contracts are relentlessly optimized to be able to provide the most capital efficient liquidity ever at the lowest cost.

## Links

- **Previous audits:**  
  - [Current Ethereum Version Audits](https://docs.ekubo.org/integration-guides/reference/audits#ethereum)
  - [Riley Holterhus Audit Report](https://github.com/code-423n4/2025-11-ekubo/blob/main/audits/Ekubo-Riley-Holterhus-Audit.pdf)
- **Documentation:** https://docs.ekubo.org/
- **Website:** https://ekubo.org/
- **X/Twitter:** https://x.com/EkuboProtocol

### Files out of scope

| File         |
| ------------ |
| [test/\*\*.\*\*](https://github.com/code-423n4/2025-11-ekubo/tree/main/test) |
| Totals: 68 |

*For a machine-readable version, see [out_of_scope.txt](https://github.com/code-423n4/2025-11-ekubo/blob/main/out_of_scope.txt)*

# Additional context

## Areas of concern (where to focus for bugs)

### Assembly Block Usage

We use a custom storage layout and also regularly use stack values without cleaning bits and make extensive use of assembly for optimization. All assembly blocks should be treated as suspect and inputs to functions that are used in assembly should be checked that they are always cleaned beforehand if not cleaned in the function. The ABDK audit points out many cases where we assume the unused bits in narrow types (e.g. the most significant 160 bits in a uint96) are cleaned.

## Main invariants

The sum of all swap deltas, position update deltas, and position fee collection should never at any time result in a pool with a balance less than zero of either token0 or token1.

All positions should be able to be withdrawn at any time (except for positions using third-party extensions; the extensions in the repository should never block withdrawal within the block gas limit).

The codebase contains extensive unit and fuzzing test suites; many of these include invariants that should be upheld by the system.

## All trusted roles in the protocol


| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| `Positions` Owner                          | Can change metadata and claim protocol fees               |
| `RevenueBuybacks` Owner                             | Can configure buyback rules and withdraw leftover tokens                       |
| `BaseNonfungibleToken` Owner | Can set metadata of the NFT |
