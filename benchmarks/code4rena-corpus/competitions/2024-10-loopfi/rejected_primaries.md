# Rejected Primary Findings: LoopFi

# Lack of access control "FUNDS_ADMINISTRATOR_ROLE" in _moveFunds function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-82
- **Submitter:** 0xpetern
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/82
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-82.md

## Brief Summary

The [_moveFunds](https://github.com/code-423n4/2024-10-loopfi/blob/d219f0132005b00a68f505edc22b34f9a8b49766/src/Treasury.sol#L68C5-L73C6) function can be called by any internal function within the contract or derived contracts, which allows unauthorized users to transfer funds from the contract to any specified treasury address. This could lead to significant financial losses, as attackers can exploit this vulnerability to siphon off funds. Here is a comment from the dev But the access control is missing in this critical function. This vulnerability directly impacts the financial security of the contract, potentially allowing attackers to transfer funds without authorization.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# withdraw() in the Locking.sol contract emits an event transferring of 0 tokens, which results in an erroneous event emission

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-21
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/21
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-21.md

## Brief Summary

withdraw() in the Locking.sol contract emits an event transferring of 0 tokens, which results in an erroneous event emission

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Revert on Zero Balance in Treasury Contract's _moveFunds Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-55
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/55
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-55.md

## Brief Summary

Lack of Revert on Zero Balance in Treasury Contract's _moveFunds Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Insufficient permission checks when decreasing collateral or debt

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-17
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/17
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-17.md

## Brief Summary

The current permission checks allow `deltaCollateral < 0` without validating the `collateralizer` permissions, potentially allowing unauthorized users to reduce the collateral. Similarly, `deltaDebt > 0` does not check if the creditor grants permission to increase debt on their behalf. Impact: Unauthorized modification of collateral and debt values could lead to theft or liquidation, which may destabilize the system or lead to unexpected financial losses for users.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Front-running Vulnerability in `_updateBaseInterest()` Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-76
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/76
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-76.md

## Brief Summary

A front-running vulnerability exists in the `_updateBaseInterest()` function due to the improper reliance on `block.timestamp` for determining when to update the `lastBaseInterestUpdate` variable. An attacker can exploit this by calling the function just before the victim, preventing the victim’s transaction from executing the interest update logic. **Affected Function**: PoolV3.sol#L694 **Description**: * The function relies on the condition `if (block.timestamp != lastBaseInterestUpdate_)` to determine whether to update the base interest. However, because `block.timestamp` remains the same within the same block, an attacker can front-run the victim's transaction by calling the function in...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Division by Zero and Withdraw Fee Boundaries in `_amountWithWithdrawalFee`

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-77
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/77
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-77.md

## Brief Summary

The `_amountWithWithdrawalFee` function has two critical issues: 1. **Division by Zero**: When `withdrawFee` equals `PERCENTAGE_FACTOR`, the denominator becomes zero, causing a transaction revert. 2. **Withdraw Fee Boundaries**: A high `withdrawFee` (close to `PERCENTAGE_FACTOR`) results in an abnormally large withdrawal amount, which may lead to unintended behavior. Affected code: PoolV3.sol#L929 **Impact** - **Division by Zero**: Causes transaction failure. - **Boundary Issue**: Allows abnormally large withdrawals, posing a risk to the contract’s balance. **Proposed Fix** 1. Prevent division by zero: 2. Enforce maximum fee limits:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ff-by-One Timestamp Vulnerability in Unstake Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-73
- **Submitter:** PolarizedLight
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/73
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-73.md

## Brief Summary

The unstake function in the StakingLPEth.sol contract uses a problematic comparison operator (>=) when checking against block.timestamp, which could lead to off-by-one errors. Description: In the unstake function, there's a conditional statement that compares block.timestamp to userCooldown.cooldownEnd using the >= operator. This comparison method introduces off-by-one errors due to the discrete nature of block.timestamp updates. Block timestamps are only updated once per block, remaining constant throughout the block's execution. If an operation occurs precisely when the block.timestamp changes, it may result in unexpected behavior. CodeLocation: https://github.com/code-423n4/2024-10-loopf...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Address Comparison Vulnerability in Token Operations

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-74
- **Submitter:** PolarizedLight
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/74
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-74.md

## Brief Summary

Loopfi protocol's code contains critical instances where function parameters of type `address` are directly compared against state variables. This practice can be exploited in the case of proxy tokens or tokens with multiple addresses, leading to security checks being bypassed and potentially resulting in significant financial losses or unauthorized access to protocol functions. Description: In multiple core functions within the contract, there are direct comparisons between an `address` parameter and a state variable (typically `address(underlyingToken)`). While this check aims to ensure operations are performed only on the intended token, it fails to account for the complexities of modern...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Accrued Rewards Reset

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-11
- **Submitter:** black-wolf
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/11
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-11.md

## Brief Summary

Accrued Rewards Reset

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Incorrect Reward Calculation with Initial Index

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-13
- **Submitter:** black-wolf
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/13
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-13.md

## Brief Summary

Incorrect Reward Calculation with Initial Index

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_06_group

# Lack of Slippage Protection in ERC4626 Implementation in PoolV3.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-43
- **Submitter:** catellatech
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/43
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-43.md

## Brief Summary

The provided PoolV3 contract, which implements the ERC4626 standard, lacks explicit slippage protection mechanisms. This omission could potentially lead to unexpected losses for users due to price fluctuations or manipulations during transactions. - https://eips.ethereum.org/EIPS/eip-4626#security-considerations Vulnerability Details The ERC4626 standard [recommends](https://eips.ethereum.org/EIPS/eip-4626#security-considerations) implementing slippage protection for functions that interact with the underlying vault, especially for operations initiated by EOA (Externally Owned Accounts). The current implementation of PoolV3 does not include such protections in its `deposit`, `mint`, `withdr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# CDPVault has getRewards function that is unchecked transfer

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-18
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/18
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-18.md

## Brief Summary

The getRewards function within the CDPVault contract is unprotected. And the getRewards function calls safeTransfer successfully. This getRewards function has no protection on it. And I was able to call the getRewards function from a non-registered address. Vulnerable Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential I/O Flow Issue in Rate Calculation in GaugeV3.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-37
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/37
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-37.md

## Brief Summary

Although the code prevents division by zero with totalVotes == 0, there is still a potential risk of integer overflow during the multiplication and addition of large values for votesCaSide, votesLpSide, qrp.minRate, and qrp.maxRate. In Solidity, casting values to uint256 as shown in this code does not inherently prevent overflow when the numbers involved are large enough. This could result in incorrect rate calculations, leading to unexpected behavior in the system. Potential Impact: The improper calculation of rates could lead to incorrect data being used in voting outcomes, which could impact the overall contract logic. An incorrect rate calculation could skew decision-making based on vot...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Signature Malleability in Permit2 Signature Construction in TransferAction.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-38
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/38
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-38.md

## Brief Summary

Potential Signature Malleability in Permit2 Signature Construction in TransferAction.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Array Access in getSwapToken Function in SwapAction.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-39
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/39
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-39.md

## Brief Summary

The function accesses elements from primarySwapPath without checking its length: token = primarySwapPath[0]; token = primarySwapPath[primarySwapPath.length - 1]; If the primarySwapPath array is empty, this will cause a transaction revert due to an out-of-bounds array access.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ABI Decoding Risk in getSwapToken Function in SwapAction.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-40
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/40
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-40.md

## Brief Summary

ABI Decoding Risk in getSwapToken Function in SwapAction.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Return Values from Virtual Function _updateRewardIndex in RewardManagerAbstract.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-41
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/41
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-41.md

## Brief Summary

The function _updateRewardIndex() is marked as virtual and will be overridden by derived contracts. The return values (tokens and indexes) from _updateRewardIndex() are used directly in the logic without validation. Since these values come from an unknown implementation, they could be faulty or unexpected, such as empty arrays or arrays with mismatched lengths. Potential Risks: Faulty Data Handling: Using unvalidated tokens and indexes arrays could lead to incorrect logic execution, failed transactions, or undefined behavior if the arrays are empty or mismatched.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Timestamp Manipulation in Interest Rate Calculation at QuotasLogic.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-53
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/53
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-53.md

## Brief Summary

The _cumulativeIndexSince() function calculates the interest index using block.timestamp, which can be slightly manipulated by miners. This creates a risk of interest rate manipulation, especially in a financial system where even small changes in time can lead to significant discrepancies over long-term calculations or high-value transactions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Integer Overflow in calcQuotaRevenueChange Function at QuotasLogic.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-54
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/54
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-54.md

## Brief Summary

The calcQuotaRevenueChange() function performs arithmetic with int256 and uint256 values. The multiplication of change (an int256) by rate (cast to int256 from uint256) could lead to an integer overflow or underflow if the result exceeds the bounds of an int256.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Arbitrary Jump in Delegate Call in PositionActionPendle.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-56
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/56
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-56.md

## Brief Summary

The _delegateCall() function is called with basic validation on leverParams.auxAction.args.length, ensuring that the call is not made with empty data. However, there is still a risk of an arbitrary jump if the target address (poolAction) or the data being passed can be manipulated, even with a success/failure check present. While the function handles failed calls properly, it does not fully mitigate the risk of arbitrary code execution or jumps to unintended contract functions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Integer Overflow Risk in PositionAction4626.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-57
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/57
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-57.md

## Brief Summary

There is a potential risk of integer overflow in the following code segment where upFrontAmount is added to addCollateralAmount. Although Solidity 0.8.x provides automatic overflow protection, it is important to ensure that these additions do not exceed safe limits in the context of this contract's logic.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Violation of Checks-Effects-Interactions Pattern in PositionAction4626.sol

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-58
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/58
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-58.md

## Brief Summary

The Checks-Effects-Interactions (CEI) pattern is not followed in the _onDecreaseLever() function, where external calls are made before the state changes are finalized. This leaves the function open to reentrancy attacks, where malicious contracts could exploit the contract's state by reentering it before the final state changes are completed.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# Lack of Role-Based Access Control (RBAC) in Vault Management

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-63
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/63
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-63.md

## Brief Summary

Critical functions like addVault() and removeVault() in the IVaultRegistry contract are missing access control. This allows any external user to add or remove vaults, which could lead to unauthorized vault manipulation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Push Model in moveFunds

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-67
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/67
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-67.md

## Brief Summary

Issue: The moveFunds function uses a push model (Address.sendValue()), which can lead to transaction failures if the receiving address (treasury) is unable to accept Ether.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Use of Push Pattern Instead of Pull for Fund Transfers in PoolV3

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-68
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/68
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-68.md

## Brief Summary

The PoolV3 contract uses the "push" pattern for transferring funds directly to a user in the _withdraw function, such as in the following line: IERC20(underlyingToken).safeTransfer({to: receiver, value: amountToUser}); // U:[LP-8,9] Using the "push" pattern to transfer funds directly to users can lead to potential risks such as reentrancy or failed transfers. It’s generally safer to adopt the "pull over push" pattern, where users can explicitly request to withdraw their funds, reducing the likelihood of vulnerabilities during the transfer process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Circuit Breaker Pattern for Critical Operations

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-70
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/70
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-70.md

## Brief Summary

Description: The contract StakingLPEth handles user funds and critical operations like withdrawals and staking, but it lacks a circuit breaker mechanism that can halt all or certain contract functions in case of emergencies or vulnerabilities. The absence of a circuit breaker pattern poses a risk in situations where vulnerabilities are discovered, or the contract behavior is not functioning as expected. If such an issue arises, the contract cannot be paused to prevent further damage or malicious actions until it is resolved.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can repay more than they owe and lose funds

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-81
- **Submitter:** misbahu
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/81
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-81.md

## Brief Summary

Users can repay more than they owe and lose funds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# BalancerOracle Fails to Validate Token Addresses, Resulting in Incorrect Prices for Any Token

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-84
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/84
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-84.md

## Brief Summary

BalancerOracle Fails to Validate Token Addresses, Resulting in Incorrect Prices for Any Token

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Mismatch in Scaling of deltaCollateral in modifyCollateralAndDebt() Function

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-85
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/85
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-85.md

## Brief Summary

Mismatch in Scaling of deltaCollateral in modifyCollateralAndDebt() Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Calculation of Liquidation Penalty Leading to Potential Over-Penalization or Under-Penalization

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-86
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/86
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-86.md

## Brief Summary

Inconsistent Calculation of Liquidation Penalty Leading to Potential Over-Penalization or Under-Penalization

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Donation attack vulnerability in `StakingLPEth` Contract

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-25
- **Submitter:** safie
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/25
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-25.md

## Brief Summary

The vulnerability allows attackers to execute donation attacks by circumventing the minimum share constraints. This can lead to the following potential impacts: Asset Loss: Users may unintentionally donate shares to attackers, resulting in a financial loss. Reduced Integrity of the Pool: The value of the pool may be diminished as small shares are systematically drained through coordinated attacks. This vulnerability could significantly undermine the integrity of the staking mechanism, allowing malicious actors to exploit the contract for financial gain through systematic donation attacks. Immediate remediation is advised to prevent potential exploits.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Precision loss vulnerability in division operations with `RAY` and `SECONDS_PER_YEAR` will affect the accuracy of interest accrual or quota calculations in `QuotasLogic.sol`

- **Contest:** LoopFi
- **Slug:** 2024-10-loopfi
- **Submission:** V-48
- **Submitter:** safie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-loopfi-validation/issues/48
- **Source snapshot:** competitions/2024-10-loopfi/submissions/raw/V-48.md

## Brief Summary

Precision loss vulnerability in division operations with `RAY` and `SECONDS_PER_YEAR` will affect the accuracy of interest accrual or quota calculations in `QuotasLogic.sol`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary
