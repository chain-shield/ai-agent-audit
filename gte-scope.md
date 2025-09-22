### Index Price Manipulation

**NOTE**: ALL PRIVILEGED ROLES ARE TRUSTED, ONLY FOCUS ON PERMISSIONLESS EXPLOITS

FOT (Fee on transfer) issue are considered Low/informational; do NOT Report.

Any vulnerabilities stemming from manipulation of the Index Price that is yielded by Oracles and other off-chain resources will be **considered INVALID** for the purposes of the contest; for all intents and purposes, the Index Price is considered valid and secure. 

Tampering of these resources as well as organic market manipulation of the Index Price are also considered out-of-scope. 

However, _price **history** manipulation of the Index Price (or any other price type within the system) is IN-SCOPE._

Additionally, manipulation of the platform-specific construct called the Mark Price is in-scope as well.

### Previously Identified Issues

Any submissions that have been identified in the Zellic audits are considered out-of-scope for the purposes of this contest. Additionally, any submissions applicable to the perpetual CLOB system that were identified in the Code4rena CLOB contest are also considered out-of-scope. 

#### KNOWN issues from Zellic audits, DO NOT REPORT
- Incorrect position update order relative to isLiquidatable check in process- MakerFill permits fund theft 12
- The isLong flag for a position is not reset to false during liquidation, preventing users from opening long positions 14
- The _gtlHook is not triggered when a GTL limit order is fully filled, resulting in inflated GTL totalAssets
- Stale price pointer after expired-order removal
- Time-weighted average price is manipulatable
- Incorrect shortcut used in isLiquidatable
- A malicious actor could block a market if minLimitOrderAmountInBase is set too low
- Incorrect ZeroCostTrade revert condition 
- Unhandled edge case in the twap function of PriceHistoryLib
- The queueWithdraw function lacks a zero-value check for the shares parameter 
- Inflated funding rate during first settlement 
- Incorrect condition check in setMinLimitOrderAmountInBase 
- Lack of explicit asset validation 
- Unsafe cast from int256 to uint256 
- Inaccessible market-setting functions via delegate call
- Centralization risk — owner-controlled deposits to insurance fund using user allowances
- Inconsistency between processWithdraws and previewWithdraw
- Attacker can manipulate the initial price in the Uniswap pool
- Purchase token with zero USDC repeat
- Remove launchpadSell function from MegaRouterFacet
- MegaRouterFacet does not validate that clob is trusted
- The Uniswap router contract still retains an approval after an unsuccessful swap 
- Usage of msg.sender.transfer() function
- Malicios user can spam orders that expire immediately or cancel them immediately to wipe out legit orders from a book with orders only on one side

### Files out of scope

| ------------ |
| [contracts/account-manager/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/contracts/account-manager) |
| [contracts/clob/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/contracts/clob) |
| [contracts/launchpad/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/contracts/launchpad/interfaces)  |
| [contracts/perps/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/contracts/perps/interfaces) |
| [contracts/router/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/contracts/router) |
| [contracts/utils/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/contracts/utils) |
| [script/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/script) |
| [test/\*\*.\*\*](https://github.com/code-423n4/2025-08-gte-perps/tree/main/test) |


## All trusted roles in the protocol

All administrative roles issued within the system are considered trusted and behaving within acceptable bounds.

A user that has been assigned as the operator of another is considered trusted.

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| `ADMIN_ROLE`                          | Can simulate any other role and has total rights over the system               |
| `LIQUIDATOR_ROLE`                             | Can liquidate, deleverage, and delist close through the `LiquidatorPanel`                      |
| `BACKSTOP_LIQUIDATOR_ROLE` | Can issue a backstop liquidation through the `LiquidatorPanel` |
