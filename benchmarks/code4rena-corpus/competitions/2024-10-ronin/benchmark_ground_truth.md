# Benchmark Ground Truth: Ronin

## Accepted H/M Findings

# Accepted H/M Findings: Ronin

No accepted High/Medium findings were parsed from a final report in the current corpus.

- **Accepted H/M count:** 0
- **Final report status:** captured
- **Submission status:** captured_from_github_validation_repo

## Rejected Primary Findings

# Rejected Primary Findings: Ronin

# Lack of tokens check in the `KatanaGovernance::createPair` function

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-108
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/108
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-108.md

## Brief Summary

- Invalid Pairs: If the factory creates a pair with the same tokens, the pair would be useless and could confuse both users and the system. - Wasted Gas: The user would spend gas to create a pair that cannot be utilized. - Potential for System Errors: Subsequent functions interacting with such a pair might not handle it properly, leading to errors or unexpected behavior in the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use of the `TransferHelper::safeApprove` method in the `V3Migrator::migrate` function without resetting the allowance to 0.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-114
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/114
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-114.md

## Brief Summary

Loss of Funds: A malicious spender with an unlimited allowance can transfer an unlimited number of tokens, resulting in potential loss of all assets from the token holder’s balance

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Possibility of underflow without revert in the `V2SwapRouter::_v2Swap` function, which can be exploited by a malicious user to steal funds.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-99
- **Submitter:** 0XRolko
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/99
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-99.md

## Brief Summary

Inappropriate behaviour and theft of funds: a malicious user can use this vulnerability to swap very large amounts of Tokens with a minimal initial investment.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Liquidity Burn Allows Excessive Token Withdrawal

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-38
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/38
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-38.md

## Brief Summary

`burn` function in KatanaV3Pool.sol allows users to burn more liquidity than what's available in the pool. The reason is that in the `burn` function, directly proceed to modify the position without verifying if the requested burn amount is valid. The function calculates `amount0` and `amount1` based on the requested burn amount, regardless of whether this amount exceeds the available liquidity. This calculation is done in the [_modifyPosition](https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/core/KatanaV3Pool.sol#L284-L335) function, which doesn't have built-in checks for this scenario. Impact Users could potentially extract more tokens th...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Use of unchecked arithmetic operations

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-39
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/39
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-39.md

## Brief Summary

The `swap` function in KatanaV3Pool.sol is susceptible to flow, particularly in handling the `amountSpecified` parameter and subsequent calculations. This could lead to incorrect swap amounts, manipulated pool balances, and potential loss of funds for users. The is because the `swap` function assumes that `amountSpecified` and related calculations will always remain within the valid range of `int256`, which is not guaranteed. The function performs unchecked arithmetic operations on large integers without proper bounds checking or SafeMath-like protections. For example, in the swap loop: [#L632-L637](https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# Improper Token Address Validation in `createPool` Function

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-5
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/5
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-5.md

## Brief Summary

`createPool` function takes three parameters. `tokenA`, `tokenB`, and `fee`. It is responsible for creating a new pool with the given token pair and fee. However, the function does not validate that both `tokenA` and `tokenB` are non-zero addresses. The bug happens because the function only checks if `token0` is not the zero address after sorting `tokenA` and `tokenB` based on their addresses. It does not check if `token1` is also a non-zero address. This allows the creation of a pool where one of the token addresses is zero. [KatanaV3Factory.sol#L77-L89](https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/core/KatanaV3Factory.sol#L77-L91) To...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# `totalPools` variable will not accurately reflect the number of pools created.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-7
- **Submitter:** 0xbrett8571
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/7
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-7.md

## Brief Summary

There is an issue with the tracking of the total number of pools created. The `createPool` function, which is used to deploy new pools, does not increment the `totalPools` variable. This leads to an inconsistency between the actual number of pools created and the value stored in `totalPools`. The flaw arises from the missing logic to increment `totalPools` after a new pool is successfully created. **Scenario:** 1. Alice calls the `createPool` function with valid parameters to create a new pool. 2. The `createPool` function deploys a new pool contract and updates the `getPool` mappings. 3. However, the `totalPools` variable is not incremented. 4. Bob, another user or an external contract, re...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# `setTreasury` allows setting treasury to the current address

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-8
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/8
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-8.md

## Brief Summary

`setTreasury` function in the KatanaV3Factory contract allows the owner to update the treasury address, which is where protocol fees are sent. The function does not check if the new `_treasury` address is different from the current one. [KatanaV3Factory.sol#L93-L97](https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/core/KatanaV3Factory.sol#L93-L97) Occurs because the function only validates that `_treasury` is not the zero address, but it does not ensure that `_treasury` differs from the current treasury value. This incomplete input validation allows the owner to set treasury to itself, leading to a misleading `TreasuryChanged` event emissi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Change of Router in the Katana governance will cause all Calls from the new router to revert.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-155
- **Submitter:** Bigsam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/155
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-155.md

## Brief Summary

Change of Router in the Katana governance will cause all Calls from the new router to revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Zero Denominator Vulnerability in Fee Protocol Parameter Validation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-221
- **Submitter:** BradMoon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/221
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-221.md

## Brief Summary

Zero Denominator Vulnerability in Fee Protocol Parameter Validation

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_09_group

# Memory Array Length Manipulation Vulnerability in Whitelist Token Management

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-224
- **Submitter:** BradMoon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/224
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-224.md

## Brief Summary

- Stale or expired whitelist entries might be returned - Applications relying on this data could make incorrect authorization decisions - Could lead to unauthorized access or denial of legitimate access

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ETH Transfer exploit via calldata manipulation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-248
- **Submitter:** CipherShieldGlobal
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/248
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-248.md

## Brief Summary

The `AggregateRouter` contract is vulnerable to an malicious fund transfer exploit due to insufficient validation of calldata. Specifically, an attacker can call `AggregateRouter::execute()` with a specific command (`Commands.TRANSFER`) and a fabricated recipient, allowing them to drain funds through the `Payments` contract. Vulnerability Details The `AggregateRouter::execute()` function allows execution of multiple arbitrary commands via the `dispatch` function, which routes them to specific handlers. Attacker can initiate a `TRANSFER` command targeting an arbitrary recipient and set a high value as an amount in the calldata inputs - `value := calldataload(add(inputs.offset, 0x40))`. - The...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Authorization Bypass in Permit Functions could result in unauthorized transfers or allowance abuse.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-187
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/187
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-187.md

## Brief Summary

Authorization Bypass in Permit Functions could result in unauthorized transfers or allowance abuse.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_53_group

# Visibility of internal state variables leading to unauthorized actions

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-191
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/191
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-191.md

## Brief Summary

Visibility of internal state variables leading to unauthorized actions

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# Potential Misuse of Governance Address

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-196
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/196
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-196.md

## Brief Summary

Potential Misuse of Governance Address

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Lack of event emission making debugging and monitoring difficult

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-197
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/197
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-197.md

## Brief Summary

Lack of event emission making debugging and monitoring difficult

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Transaction Expiration leading to potential confusion and financial discrepancies.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-200
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/200
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-200.md

## Brief Summary

Transaction Expiration leading to potential confusion and financial discrepancies.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Error Handling making debugging more difficult

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-202
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/202
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-202.md

## Brief Summary

Error Handling making debugging more difficult

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted command execution could lead to malicious actions by unauthorized actors.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-209
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/209
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-209.md

## Brief Summary

Unrestricted command execution could lead to malicious actions by unauthorized actors.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# ETH Address (address(0)) Flagging Issue could lead to loss of funds

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-222
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/222
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-222.md

## Brief Summary

ETH Address (address(0)) Flagging Issue could lead to loss of funds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Ambiguity with MSG_SENDER and ADDRESS_THIS Flags could lead to unauthorized access, improper transaction routing, or ambiguous transfer flows.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-228
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/228
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-228.md

## Brief Summary

Potential Ambiguity with MSG_SENDER and ADDRESS_THIS Flags could lead to unauthorized access, improper transaction routing, or ambiguous transfer flows.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_45_group

# Potential Misuse of Hardcoded Constants (ADDR_SIZE, V3_FEE_SIZE, V3_POP_OFFSET) which can lead to incorrect logic or data misinterpretations

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-235
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/235
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-235.md

## Brief Summary

Potential Misuse of Hardcoded Constants (ADDR_SIZE, V3_FEE_SIZE, V3_POP_OFFSET) which can lead to incorrect logic or data misinterpretations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Misuse of Constants.CONTRACT_BALANCE as Input can lead to undesired fund flows and contract misbehavior

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-245
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/245
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-245.md

## Brief Summary

Misuse of Constants.CONTRACT_BALANCE as Input can lead to undesired fund flows and contract misbehavior

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_102_group

# Arbitrary Token Transfers without Validation can lead to contract misbehavior or stuck funds.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-247
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/247
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-247.md

## Brief Summary

Arbitrary Token Transfers without Validation can lead to contract misbehavior or stuck funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# A call to pool.collect from `NonFungiblePositionManager.collect` will be returned

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-162
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/162
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-162.md

## Brief Summary

A call to pool.collect from `NonFungiblePositionManager.collect` will be returned

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `migrate()` lack slippage control

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-210
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/210
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-210.md

## Brief Summary

`migrate()` lack slippage control

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# lack safeApprove before call TransferHelper.safeTransferFrom

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-211
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/211
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-211.md

## Brief Summary

lack safeApprove before call TransferHelper.safeTransferFrom

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# no check on the liquidity holder

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-241
- **Submitter:** Hajime
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/241
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-241.md

## Brief Summary

no check on the liquidity holder

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Multiple Pool Creation Enables Liquidity Fragmentation and Market Manipulation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-176
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/176
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-176.md

## Brief Summary

Multiple Pool Creation Enables Liquidity Fragmentation and Market Manipulation

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# positions Function in NonfungiblePositionManager Returns Potentially Invalid Tick Values

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-217
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/217
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-217.md

## Brief Summary

The NonfungiblePositionManager's positions() function returns potentially invalid tick values without validating against current pool requirements, leading to inconsistencies between stored position data and protocol rules. Impact This issue can cause: - Integration contract failures - Incorrect position valuations - Failed trading strategies - Invalid market making ranges - System-wide inconsistencies in position management Description The NonfungiblePositionManager allows positions to exist with tick values that may violate current pool tick spacing requirements. This occurs due to the relationship between three key components: 1. The Factory contract defines valid tick spacings for each...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Public Exposure of Position Data in positions() Enables Precise MEV Extraction

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-218
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/218
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-218.md

## Brief Summary

The NonfungiblePositionManager's positions() function exposes complete economic data about liquidity positions (including exact liquidity amounts and uncollected fees) through a public view function. MEV bots can combine this detailed position data with pending collection/withdrawal transactions in the mempool to execute precise sandwich attacks. By knowing the exact amounts of fees to be collected or liquidity to be withdrawn before these transactions execute, attackers can optimize their front-running strategies with zero uncertainty about position sizes or expected returns. The transparency of position data, while useful for the protocol's operation, creates a deterministic attack vector...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Command Boundary Validation in Dispatcher Contract Can Lead to Unexpected Execution Paths

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-219
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/219
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-219.md

## Brief Summary

The Dispatcher contract's core functionality revolves around processing different command types through its dispatch function. The current implementation has a critical flaw in how it validates and processes command boundaries. In the current implementation, we see this structure: The issue lies in the command validation logic. While there's a lower boundary check (`command < Commands.FIRST_IF_BOUNDARY`), the contract lacks proper upper boundary validation. This creates several potential problems: 1. The command masking operation: While this masks the input, it doesn't prevent invalid commands from being processed in the second branch of the if statement. Consider what happens with differen...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Pool ID Overflow in cachePoolKey Can Cause Mapping Collisions and System Inconsistency

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-220
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/220
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-220.md

## Brief Summary

An overflow vulnerability exists in the cachePoolKey function where _nextPoolId++ can silently overflow when reaching its maximum value (2^80 - 1). When this occurs, the next pool creation would attempt to reuse ID 0, potentially colliding with an existing pool mapping. This can lead to system inconsistency where new pools cannot be created and existing pool mappings could be compromised. The function assigns the current value of _nextPoolId to poolId BEFORE incrementing When _nextPoolId is at max value (2^80 - 1): - The assignment to poolId will work (gets max value) - But the increment operation (_nextPoolId++) will overflow - Next pool creation attempt will use _nextPoolId = 0 Impact - I...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe Parameter Decoding in Dispatcher Contract Leads to Potential Data Corruption

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-223
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/223
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-223.md

## Brief Summary

The Dispatcher contract's current implementation uses an unsafe pattern for parameter decoding that leads to scattered memory access, repeated assembly blocks, and potential stack pressure issues. Code Issues: 1. Manual memory offset calculations are error-prone 2. Stack variables are scattered across function scope 3. Similar assembly blocks are repeated for different commands 4. Parameter validation is mixed with decoding logic 5. Unclear separation between parameter loading and business logic Impact Manual offset calculations in the assembly block can lead to data corruption and incorrect parameter loading. In the current implementation: The danger lies in the manual offset calculations:...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Conditional Oracle Updates in Katana V3 _modifyPosition Enable Stale Data and Price Manipulation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-226
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/226
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-226.md

## Brief Summary

The oracle mechanism in KatanaV3Pool contains a critical design flaw in `_modifyPosition` where oracle updates are conditional on the position's tick range, potentially leading to stale or manipulated price data. The issue lies in how oracle updates are handled during position modifications: The vulnerability manifests in several ways: 1. **Selective Oracle Updates** The oracle is only updated when the current tick falls within the position's range: This creates a scenario where significant liquidity changes outside the current tick range go unrecorded, leading to incomplete historical data. 2. **Oracle Manipulation Vector** An attacker can exploit this by structuring their positions to avo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inefficient Tick Update Mechanism in Katana V3 Causes Excessive Gas Costs for Multi-Range Positions

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-232
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/232
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-232.md

## Brief Summary

The KatanaV3Pool contract's tick update mechanism processes each tick modification individually, leading to excessive gas costs and inefficient state updates when multiple tick changes occur in close succession. This architectural limitation particularly impacts large liquidity providers and multi-range position management. The issue lies in how tick updates are processed in the `_updatePosition` function. Each tick modification requires individual state updates and storage writes. Impact Scenario Consider a liquidity provider managing multiple positions across different price ranges. Each position modification triggers separate tick updates: Each position modification involves: 1. Two sepa...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Rebase Vulnerability in Katana V3 mint Function Can Cause Incorrect Liquidity Provisioning

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-236
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/236
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-236.md

## Brief Summary

The mint function's balance checking mechanism is vulnerable to rebasing tokens, where token balances can change independently of transfers. The current implementation assumes token balances only change through explicit transfers, but rebasing tokens can modify balances automatically between the initial check and final verification, potentially leading to incorrect liquidity provisioning and pool imbalances. In the KatanaV3Pool contract, the handling of token balances assumes traditional ERC20 behavior where balances only change through transfers. Let's examine how this assumption breaks down with rebasing tokens: Let's look at how this interacts with a rebasing token. Here's a simplified r...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_101_group

# Katana V3 collect Function Breaks NFT Security Model, Allows Fee Collection After Transfer

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-238
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/238
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-238.md

## Brief Summary

The Katana V3 Pool's collect function has a significant design inconsistency in its security model. While position creation and burning require interaction through the position manager (enforcing NFT ownership), the collect function allows direct pool interaction. This means users who have transferred their position NFTs can still collect fees directly from the pool, breaking the core security assumption that position NFTs represent complete control over a position. While this doesn't enable direct theft of funds, it undermines protocol composability and could break higher-level DeFi protocols that rely on position NFT ownership as a source of truth for position control. The issue centers a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Memory Exhaustion Vulnerability in Katana V3 Swap Function During Large Swaps

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-239
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/239
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-239.md

## Brief Summary

The KatanaV3Pool's swap function implements a loop-based price traversal system that can potentially consume significant amounts of memory when processing large swaps across multiple price ranges. At the heart of this issue lies the interaction between memory allocation patterns and Ethereum's block gas limits. Let's examine the relevant code structure: The problem manifests in how memory is allocated and managed during swap execution. For each iteration of the while loop, a new `StepComputations` struct is allocated in memory: When a large swap occurs, particularly one that crosses many tick boundaries, the function creates numerous `StepComputations` instances. Consider a scenario where a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_36_group

# Lack of Emergency Pause Mechanism in KatanaV3Pool

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-249
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/249
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-249.md

## Brief Summary

The `KatanaV3Pool` contract lacks an emergency pause mechanism, such as the one provided by OpenZeppelin's `Pausable.sol` contract and its `whenNotPaused` modifier. This omission poses a significant risk to the protocol and its users as it prevents critical functions from being halted in the event of vulnerabilities, exploits, or unforeseen circumstances. Impact Without a pause mechanism, the following scenarios could lead to significant losses or disruption: * **Smart Contract Bugs:** If a critical bug is found in the `mint()`, `swap()`, `burn()`, or `flash()` logic, the contract owner or governance system has no way to stop users from interacting with these functions, potentially leading...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Suboptimal Array Allocation in Mixed Route Quoter

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-262
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/262
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-262.md

## Brief Summary

The `quoteExactInput` function in MixedRouteQuoterV1 allocates arrays sized for all pools in the path, regardless of pool type (V2/V3), leading to unnecessary gas consumption due to unused array slots for V2 pools. The function allocates two arrays to store V3-specific data: - `v3SqrtPriceX96AfterList`: Stores price data after each V3 swap - `v3InitializedTicksCrossedList`: Stores tick crossing data for V3 swaps Current Implementation Impact 1. **Gas Overhead**: - Each unused array slot costs ~20k gas for initialization - For a path with 3 V2 pools and 1 V3 pool, we waste ~60k gas 2. **Memory Inefficiency**: - Allocates 32 bytes per unused slot for `v3SqrtPriceX96AfterList` - Allocates 4 by...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Infinite Loop Risk in MixedRouteQuoter

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-264
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/264
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-264.md

## Brief Summary

The `quoteExactInput` function in MixedRouteQuoterV1 contains a potential infinite loop vulnerability due to insufficient path validation and no maximum iteration protection. The function uses an unbounded while(true) loop to process a path of pools, relying solely on `path.hasMultiplePools()` and path manipulation for termination: Current Implementation Impact 1. **Denial of Service (DoS)**: - Malicious users could craft paths that cause infinite loops - Function could hit block gas limits - Could block other users from using the quoter 2. **Resource Exhaustion**: - Excessive CPU usage on RPC nodes - High gas costs for users - Node operators might blacklist contracts Attack Scenarios Scena...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incomplete Mixed Route Gas Estimation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-267
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/267
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-267.md

## Brief Summary

The `quoteExactInput` function only tracks gas costs for V3 swaps while ignoring V2 swap gas costs, leading to inaccurate gas estimations for mixed routes. This can cause users to receive incomplete or misleading gas estimates for their transactions. The current implementation only accumulates gas estimates for V3 pools: Current Implementation Impact 1. **Inaccurate Gas Estimates**: - Users receive incomplete gas estimates for mixed routes - V2 swap costs are completely ignored - Can lead to transaction failures due to insufficient gas 2. **Economic Impact**: - Users cannot accurately compare route costs - May choose suboptimal routes based on incomplete information - Higher than expected t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# KatanaV3Pool uses custom fees whereas KatanaV2Library.getAmountOut uses hardcoded fees

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-118
- **Submitter:** PrasadLak
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/118
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-118.md

## Brief Summary

Katana v3 allows for flexible fee structures with multiple protocol fee tiers but [KatanaV2Library.getAmountIn and KatanaV2Library.getAmountOut](https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/periphery/libraries/KatanaV2Library.sol#L46C1-L70C4) used hardcoded fees which caused incorrect calculation during swaps. Impact Swap is not executed as expect since KatanaV2Library.getAmountIn and KatanaV2Library.getAmountOut are used hardcoded fees whereas KatanaV3Pools used custom fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# AggregateRouter doesn't refund unspent ETH after swapping

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-159
- **Submitter:** PrasadLak
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/159
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-159.md

## Brief Summary

User not received unspent ETH after swapping.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Front-running Vulnerability in swap Function of KatanaV3Pool.sol

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-97
- **Submitter:** Raihan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/97
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-97.md

## Brief Summary

Front-running Vulnerability in `swap` Function of **KatanaV3Pool.sol** ---

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# Cross-site scripting risk in NFT position descriptor due to insufficient token symbol validation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-123
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/123
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-123.md

## Brief Summary

Cross-site scripting risk in NFT position descriptor due to insufficient token symbol validation

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Number of NFT positions mintable are limited due to the data type used for tracking the next token id

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-12
- **Submitter:** TECHFUND-inc
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/12
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-12.md

## Brief Summary

Number of NFT positions mintable are limited due to the data type used for tracking the next token id

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_92_group

# OOG / unexpected reverts due to incorrect usage of staticcall.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-100
- **Submitter:** Tomasleocadio
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/100
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-100.md

## Brief Summary

Update of token0 (`balance0`) and token1 (`balance1`) is done using a `staticcall` on the pool contract and checking the return value. According to the solidity docs, if a staticcall encounters a state change, it burns up all gas and returns. The `mint` function calls `balance0` and `balance1`, everytime `amount0` and `amount1` are greater than 0 on the pool contract, and returns if it finds a state change. This happens in `KatanaV3Pool::swap` and `KatanaV3Pool::flash` functions to. The issue is that this burns up all the gas sent with the call. According to EIP150, a call gets allocated 63/64 bits of the gas, and the entire 63/64 parts of the gas is burnt up after the staticcall, since the...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Use unchecked in `TickMath.sol ` and `FullMath.sol`

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-55
- **Submitter:** Tomasleocadio
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/55
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-55.md

## Brief Summary

Uniswap math libraries rely on wrapping behaviour for conducting arithmetic operations. Solidity version 0.8.0 introduced checked arithmetic by default where operations that cause an overflow would revert. Since the code was adapted from Uniswap and written in Solidity version 0.7.6, these arithmetic operations should be wrapped in an `unchecked` block.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing access Control over `initialize` function.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-243
- **Submitter:** Tonchi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/243
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-243.md

## Brief Summary

`KatanaV3Factory::initialize` function missing access control over `initialize` function, anyone can manipulate `critical storage` of this protocol, any malicious caller can change `beacon`, `owner`, `treasury` and other storage variables and also a malicious actor can initialize any pool created previously. **Impact** Malicious actor can initialize any pool created previously with unintended parameters, and can change critical storage variables. And anyone can change storage later after initialize.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_107_group

# Beacon is not showed up as a proxy to block explorers or can be set any contract's address as beacon

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-255
- **Submitter:** Tonchi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/255
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-255.md

## Brief Summary

This beacon proxy can only be set as proxy so that block explorer should know that this is a proxy and hence if we set a beacon using contructor of beacon proxy than it checks `"assert(_BEACON_SLOT == bytes32(uint256(keccak256("eip1967.proxy.beacon")) - 1));" `will be true only then beacon is set. But now because _setBeacon function doesn't have any of these checks it is not considered as proxy by block explorers. **Impact** _setBeacon function doesn't have any of these checks it is not considered as proxy by block explorers.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# function `v2SwapExactInput` has a missing deadline parameter

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-158
- **Submitter:** Vancelot
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/158
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-158.md

## Brief Summary

function `v2SwapExactInput` has a missing deadline parameter

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Loss of Tokens Due to Incomplete Collection Check Before Burning Position

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-254
- **Submitter:** YourGuyD3v
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/254
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-254.md

## Brief Summary

An issue was identified in the `collect` function of the `NonfungiblePositionManager` contract. Specifically, a missing check for the owed token amounts `(tokensOwed0 and tokensOwed1)` before deleting the position could result in uncollected tokens being permanently lost. This issue affects the implementation of the `collect` function, where the contract may inadvertently delete a position with outstanding tokens owed, Vulnerability In the `collect` function, the following line of code deletes a position when `liquidity == 0`: However, there is no check to verify if the position’s tokens owed `(tokensOwed0 and tokensOwed1)` have been fully collected. As a result, if a position has zero liqu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Unbounded Token Collection Can Lead to Unexpected Reverts

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-91
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/91
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-91.md

## Brief Summary

* Users attempting to collect maximum tokens will have transactions fail * Position accounting could become inconsistent * Affects liquidity providers trying to exit positions **Trigger Conditions** * User requests maximum uint128 amounts * Position has less tokens than requested * Function reverts instead of returning available amounts The vulnerability creates a path where legitimate collection attempts can fail, impacting user experience and position management functionality.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Downcasting Issue in Protocol Fee Calculation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-125
- **Submitter:** albahaca
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/125
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-125.md

## Brief Summary

In the swap function of the KatanaV3Pool contract, there is a potential downcasting issue in the line: Technical Details: Calculation: delta is computed using `FullMath.mulDiv(step.feeAmount, cache.feeProtocolNum, cache.feeProtocolDen)`, which returns a uint256. Downcasting: The result is cast to uint128 before being added to `state.protocolFee`. Potential Risks: Data Loss: If delta exceeds the maximum value of uint128 (i.e., (2^{128} - 1)), the downcasting operation will truncate the value, leading to incorrect protocol fee accumulation. Financial Impact: This could result in under-collection of protocol fees, impacting revenue and potentially leading to discrepancies in financial reportin...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# check for token in checkPair

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-139
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/139
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-139.md

## Brief Summary

check for token in checkPair

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Common tokens such as WETH9 work differently on chains such a Blast which isn't taken into account during transfer calls

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-142
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/142
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-142.md

## Brief Summary

Common tokens such as WETH9 work differently on chains such a Blast which isn't taken into account during transfer calls

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# eth being struck in AggregateRouter.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-144
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/144
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-144.md

## Brief Summary

eth being struck in AggregateRouter.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Wrong init code hash

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-151
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/151
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-151.md

## Brief Summary

Wrong init code hash

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# wrong implement of nativeCurrencyLabel

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-154
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/154
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-154.md

## Brief Summary

wrong implement of nativeCurrencyLabel

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# use safemint instead of mint.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-166
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/166
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-166.md

## Brief Summary

use safemint instead of mint.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# use memory safe for assembly.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-173
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/173
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-173.md

## Brief Summary

use memory safe for assembly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of protection against multiple initialisations in the constructor of the `KatanaV3Factory` contract

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-71
- **Submitter:** catellatech
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/71
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-71.md

## Brief Summary

In the `KatanaV3Factory` contract, the constructor is commented to suggest that it disables initialisation (// disable initialization), but the proper use of the `_disableInitializers()` function, recommended by OpenZeppelin, is missing. Without this function, the implementation contract is vulnerable to being taken over by an attacker if deployed as part of a proxy setup. **Impact:** 1. **Risk of contract takeover**: Failing to use `_disableInitializers()` leaves the implementation contract exposed to being controlled by an attacker. 2. **Risk of double initialisation**: The current structure could potentially allow multiple initialisations of the contract, depending on how it is deployed...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Authorization Through tx.origin

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-137
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/137
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-137.md

## Brief Summary

Authorization Through tx.origin

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Reentrancy Risk in the migrate Function

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-149
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/149
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-149.md

## Brief Summary

A reentrancy attack could potentially allow an attacker to: • Withdraw more tokens or ETH than they are entitled to, by manipulating the state between operations. • Drain the contract’s funds if combined with other vulnerabilities. Code Reference The reentrancy risk is present in the migrate function, specifically in the following code sections: • IWETH9(WETH9).withdraw(refund0); • TransferHelper.safeTransferETH(msg.sender, refund0); • TransferHelper.safeTransfer(params.token0, msg.sender, refund0); • Similar logic for refund1. Direct link to the code: [GitHub - migrate function](https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/periphery/V...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Liquidity providers may face difficulties when managing their positions, as the inconsistent tick state can affect their ability to add or remove liquidity correctly.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-46
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/46
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-46.md

## Brief Summary

Liquidity providers may face difficulties when managing their positions, as the inconsistent tick state can affect their ability to add or remove liquidity correctly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_110_group

# The `totalFeeGrowth0` and `totalFeeGrowth1` variables are not updated along with their corresponding `feeGrowthGlobal0X128` and `feeGrowthGlobal1X128` variables.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-48
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/48
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-48.md

## Brief Summary

The `swap` function logic correctly updates the `feeGrowthGlobal0X128` and `feeGrowthGlobal1X128` variables based on the swap direction (`zeroForOne`). However, it fails to make the corresponding updates to the `totalFeeGrowth0` and `totalFeeGrowth1` variables. This results in the total fee growth variables falling out of sync with the actual fee growth. See below for the key steps are: 1. The `initialize` function is called with `sqrtPriceX96 = 79228162514264337593543950336`. 2. The `mint` function is called with `recipient = 0`, `tickLower = -887272`, `tickUpper = 887272`, `amount = 1`, and some `data`. 3. The `swap` function is called with `recipient = 1`, `zeroForOne = true`, `amountSpe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect accounting of total fees collected by the pool, leading to discrepancies in financial records and reporting.

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-49
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/49
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-49.md

## Brief Summary

Incorrect accounting of total fees collected by the pool, leading to discrepancies in financial records and reporting.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Missing Authorization Check in Flash Loan Function

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-50
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/50
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-50.md

## Brief Summary

`flash` function does not include a check to verify that the caller (`msg.sender`) is the position manager. As a result, any address can call the function and initiate a flash loan, bypassing the intended access control mechanism. [#L747-L748](https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/core/KatanaV3Pool.sol#L747-L748) It is part of the `IKatanaV3PoolActions` interface, which defines the actions that can be performed on the pool. The function is responsible for initiating flash loans, allowing users to borrow tokens from the pool and repay them within the same transaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Improper Initialization of Observation Array in Oracle Library Leads to Data Integrity Issues

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-94
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/94
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-94.md

## Brief Summary

The Oracle library serves a crucial function in managing a circular buffer of observations that track historical price and liquidity data within an AMM pool. Central to this process is the `write` function, which adds new observations to the buffer. Unfortunately, this function contains a significant flaw in its logic concerning the expansion of the observation array's cardinality, or size. Specifically, when the cardinality is increased, the function fails to properly initialize the newly allocated slots. This oversight can result in uninitialized data being read or gaps in the data, leading to serious implications for data integrity. Vulnerable Code The problem arises when the function de...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# Unsecured Position Handling in NonfungiblePositionManager.sol

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-67
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/67
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-67.md

## Brief Summary

Issue Description: The NonfungiblePositionManager.sol contract allows positions to be minted and transferred, but it lacks adequate protection mechanisms to prevent reentrancy or manipulation of position ownership. Proof of State: The mint and transferFrom functions in this contract do not implement checks to protect against reentrancy attacks, especially when interacting with external contracts during minting or transfers: function mint(...) external {...} function increaseLiquidity(...) external {...} function decreaseLiquidity(...) external {...} Reentrancy could occur if malicious contracts hijack these functions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_44_group

# Lack of Ownership Transfer Control in KatanaV3Factory.sol

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-68
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/68
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-68.md

## Brief Summary

Issue Description: The KatanaV3Factory.sol allows creation and deployment of new pools but lacks clear ownership transfer mechanics for pools it deploys, potentially allowing an unauthorized transfer of ownership. Proof of State: Ownership is initially assigned but lacks mechanisms to securely transfer control or manage permissions on newly created pools: address public owner; function createPool(...) external returns (...) { // No transfer validation }

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Event Emission After Tick Updates in Tick.sol

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-70
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/70
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-70.md

## Brief Summary

In the update function within Tick.sol, there is no event emitted after a tick update. Since this function manages crucial tick updates, such as liquidity changes and fee growth data, the absence of event emission reduces transparency. Without events, it becomes challenging for off-chain monitoring systems or external contracts to track state changes related to liquidity and ticks effectively. This could lead to undetected issues in decentralized finance (DeFi) systems where transparency is crucial for security and functionality. Proof of State: Here is the relevant portion of the update function: function update( mapping(int24 => Tick.Info) storage self, int24 tick, int24 tickCurrent, int1...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Optimizer Bug in Low-Level Calls for Solidity Versions < 0.8.14

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-126
- **Submitter:** igdbase
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/126
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-126.md

## Brief Summary

(https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/periphery/lens/MixedRouteQuoterV1.sol#L53-L76 Vulnerability details Impact The contracts in scope are using low-level calls along with inline assembly in Solidity versions prior to 0.8.14, which introduces an optimizer bug. This bug can lead the optimizer to incorrectly identify certain memory operations as "dead" and remove them. Consequently, later operations that depend on these memory operations may read stale or incorrect values, resulting in unpredictable behavior, state inconsistencies, and potential vulnerabilities, such as improper state transitions or incorrect calculations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Token Order Check in KatanaV3PoolDeployer.deploy()

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-129
- **Submitter:** igdbase
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/129
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-129.md

## Brief Summary

(https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/core/KatanaV3PoolDeployer.sol#L31-L40 Vulnerability details Impact The `KatanaV3PoolDeployer.deploy()` function does not validate the order of token addresses (`token0` and `token1`). If a pool is deployed with the token addresses in the wrong order, it may result in swaps being executed at incorrect prices. This could lead to significant financial losses for users interacting with the pool, as the price calculations depend on the correct ordering of tokens. Additionally, improper token order may disrupt the expected functionality of the liquidity pool, causing further user confusion and ri...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Token Existence Check in `NonfungibleTokenPositionDescriptor.tokenURI()

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-130
- **Submitter:** igdbase
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/130
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-130.md

## Brief Summary

(https://github.com/ronin-chain/katana-v3-contracts/blob/03c80179e04f40d96f06c451ea494bb18f2a58fc/src/periphery/NonfungibleTokenPositionDescriptor.sol#L46-L86 Vulnerability details Impact The `tokenURI()` function in the `NonfungibleTokenPositionDescriptor` contract does not verify the existence of the provided token ID. This oversight allows malicious users to query metadata for non-existent NFTs, potentially impersonating bogus assets. Users attempting to interact with or view the metadata of these non-existent tokens could be misled, risking financial loss or reputational damage. Moreover, this lack of verification violates the ERC-721 standard, which mandates that functions must revert...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Initialization Logic Vulnerability exploitable in katana-v3-contracts-03c80179e04f40d96f06c451ea494bb18f2a58fc/src/core/KatanaV3PoolBeacon.sol

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-179
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/179
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-179.md

## Brief Summary

Initialization Logic Vulnerability exploitable in katana-v3-contracts-03c80179e04f40d96f06c451ea494bb18f2a58fc/src/core/KatanaV3PoolBeacon.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Integer Overflow in /src/periphery/NonfungibleTokenPositionDescriptor.sol

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-251
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/251
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-251.md

## Brief Summary

For context, the vulnerable function in NonfungibleTokenPositionDescriptor.sol is here: The `flipRatio` function calls `tokenRatioPriority` twice, once for each input token (`token0` and `token1`). It then compares the returned values to determine the priority order. However, there is no check on whether `tokenRatioPriority` returns a valid value. If `tokenRatioPriority` returns an unexpected or invalid value (e.g., due to an error in its logic), it leads to incorrect comparisons, and potentially unintended behavior in `flipRatio`. There is a lack of input validation within the `tokenRatioPriority` function, leading to an integer overflow. While the exact impact depends on how `tokenRatioPr...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_27_group

# Gas Limit Vulnerability in src/core/KatanaV3PoolProxy.sol

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-258
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/258
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-258.md

## Brief Summary

For context, there is a gas limit vuln in the following

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Missing Events on Important State Changes

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-26
- **Submitter:** linemi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/26
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-26.md

## Brief Summary

Missing Events on Important State Changes

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Overflow can lead to pool replacement

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-4
- **Submitter:** newspacexyz
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/4
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-4.md

## Brief Summary

In the mint() function of the NonfungiblePositionManager, the unchecked increment of the token ID (_nextId++) poses a potential vulnerability. Specifically, a "dust amount" of tokens (tiny amounts of token0 or token1) could cause a new token ID to be minted without proper validation, potentially overwriting an existing token ID. This could lead to severe consequences such as: 1. Token ID Collisions: If the token ID is incremented without checking its uniqueness, the new mint could overwrite an existing token, leading to a loss of control over existing positions. 2. Loss of Funds: The overwriting of an existing token ID would mean that liquidity providers may lose access to their funds, as t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_92_group

# Fixed addresses for USDC, WETH, and WBTC in `NonfungibleTokenPositionDescriptor` contract can lead to misbehavior in the contract's logic, causing failures or inaccuracies in metadata generation

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-19
- **Submitter:** safie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/19
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-19.md

## Brief Summary

1. The contract will fail to retrieve correct token metadata for USDC, WETH, and WBTC if the contract is deployed on a different chain or if new versions of these tokens are deployed with different addresses. 2. Functions that rely on fetching token data (such as metadata generation or token ratio calculations) will fail or return incorrect results. This could lead to DoS in the functionality that depends on correct token metadata. 3. The contract cannot easily adapt to changes in token contracts or deployments on different chains, making it less robust and more prone to failure. The vulnerability is classified as Medium because while it does not directly affect security (such as unauthoriz...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The MixedQuoteRouter would never be able to quote prices

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-165
- **Submitter:** silver_eth
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/165
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-165.md

## Brief Summary

The MixedQuoteRouter would never be able to quote prices

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Improper Handling of Negative Token Amounts in mint Function (Casting Signed Integers to Unsigned Integers)

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-82
- **Submitter:** skid0016
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/82
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-82.md

## Brief Summary

In the `mint` function of the `KatanaV3Pool`, when liquidity is added to a position, the protocol calculates the amount of `token0` and `token1` owed. These amounts are represented as signed integers (`amount0Int` and `amount1Int`). However, these signed integers are always cast to unsigned integers (`uint256`). If either of these amounts is negative (i.e., the protocol owes tokens to the user), casting the negative values to unsigned integers results in large, incorrect values due to integer underflow. This can lead to incorrect tracking of tokens owed, potentially resulting in mismanagement of balances during the mint process. Impact: The incorrect casting of negative amounts to unsigned...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Payments.sol#pay() should check "value" if (token == Constants.ETH)

- **Contest:** Ronin
- **Slug:** 2024-10-ronin
- **Submission:** V-2
- **Submitter:** zhanmingjing
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ronin-validation/issues/2
- **Source snapshot:** competitions/2024-10-ronin/submissions/raw/V-2.md

## Brief Summary

Payments.sol#pay() should check "value" if (token == Constants.ETH)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary
