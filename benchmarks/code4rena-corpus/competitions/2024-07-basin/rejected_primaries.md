# Rejected Primary Findings: Basin

# The `__ReentrancyGuard_init()` function should be called before any other initializations in the `init` function to ensure that reentrancy protection is established as early as possible.

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-115
- **Submitter:** 0xvd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/115
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-115.md

## Brief Summary

The current implementation of the `init` function initializes the reentrancy guard after other initializations, which leaves a window where reentrancy attacks could occur. By prioritizing the initialization of the reentrancy guard, you can ensure that the contract is protected from reentrancy attacks from the start.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in Stable2.sol

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-79
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/79
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-79.md

## Brief Summary

The contract Stable2.sol does not validate the length of input arrays in several functions which take in array parameters. This exposes the contract functions to potential out-of-bounds issues. There is no check to verify if the `reserves` array is of the expected length, which could cause unexpected errors or misbehaviors in the contract. Moreover, the smart contract does not perform any validation on address inputs before proceeding with execution. This might expose the contract to potential invocation of the zero address, which may result in unexpected contract behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Lack of Input Sanitization in Stable2LUT1.sol

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-82
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/82
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-82.md

## Brief Summary

The `getRatiosFromPriceLiquidity` and `getRatiosFromPriceSwap` functions do not have any checks to ensure that the input provided is valid. This could lead to unexpected behaviour if an invalid value is passed as an argument to the function. Moreover, the use of nested if-else conditions for price ranges is not the most efficient way for price level determination and makes the code harder to read and maintain.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Unrestricted Function Access Leading to Potential Manipulation and DoS Vulnerabilities

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-16
- **Submitter:** JuggerNaut63
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/16
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-16.md

## Brief Summary

Unrestricted Function Access Leading to Potential Manipulation and DoS Vulnerabilities

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# High Complexity in Price Lookup Functions Due to Deep Nesting

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-17
- **Submitter:** JuggerNaut63
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/17
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-17.md

## Brief Summary

- The deeply nested if-else structure makes the code hard to maintain and update, increasing the risk of introducing errors. - The complexity reduces code readability, making it difficult for developers to understand and debug. - Deep nesting can lead to high gas costs, potentially making transactions more expensive.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Arbitrary from passed to transferFrom (lor safeTransferFrom)

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-37
- **Submitter:** MFaizal14
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/37
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-37.md

## Brief Summary

Passing an arbitrary from address to transferFrom (or safeTransferFrom) can lead to loss of funds, because anyone can transfer tokens from the from address if an approval is made.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Uninitialized local

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-9
- **Submitter:** MFaizal14
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/9
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-9.md

## Brief Summary

is a local variable never initialized

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No Storage Gap For Upgradeable Contracts (child as well as parent contracts)

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-94
- **Submitter:** Shubham
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/94
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-94.md

## Brief Summary

In the Automated Finings, the issue is mentioned ([Link](https://github.com/code-423n4/2024-07-basin/blob/main/4naly3er-report.md#l-10-upgradeable-contract-is-missing-a-__gap50-storage-variable-to-allow-for-new-storage-variables-in-later-versions)) but it only states a part of the bug which would still lead to problems. Upgradeability involves **inheritance** but the inherited contract do not have a storage gap, not even the contract that is inheriting has.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# precision loss due to division before multiplicaton

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-122
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/122
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-122.md

## Brief Summary

Detailed description of the impact of this finding. precision loss due to division before multiplication in getBandC.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# High Gas Consumption and Potential Out-of-Gas in calcLpTokenSupply and calcReserve Functions

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-31
- **Submitter:** black-wolf
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/31
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-31.md

## Brief Summary

1-`High Gas Costs`: Long execution times of these functions can lead to very high gas costs, which is undesirable for users. 2-`Out-of-Gas Risk`: If the computations are too lengthy, the contract might run out of gas, causing the transaction to fail and disrupting contract operations. 3-`Unstable Performance‍‍‍‍‍`: Complex and lengthy computations can lead to unstable contract performance and reduce user trust in the system.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# In the ``Stable2`` contract ``calcLpTokenSupply‍‍`` function, there is a potential integer overflow/underflow vulnerability

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-32
- **Submitter:** black-wolf
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/32
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-32.md

## Brief Summary

In the `calcLpTokenSupply` function, there is a potential integer overflow/underflow vulnerability due to arithmetic operations involving large numbers. Integer overflow/underflow can lead to unexpected results, such as incorrect calculations or unintended contract behavior. This issue occurs in the loop where arithmetic operations are performed on `lpTokenSupply`, which can lead to values exceeding the maximum or minimum limits of the `uint256` type. Impact If an integer overflow or underflow occurs, the calculations within the `calcLpTokenSupply` function may produce incorrect results. This can lead to incorrect `lpTokenSupply` values, which in turn may affect the overall functionality of...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# Unchecked External Call on `calcRate` function on `Stable2` contract

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-33
- **Submitter:** black-wolf
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/33
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-33.md

## Brief Summary

Issue Type: Unchecked External Call Affected Contract: Stable2 Description The `calcRate` function calls an external contract to fetch data. If this external contract is compromised or behaves unexpectedly, it could return incorrect data or fail, potentially causing unintended behavior in the `calcRate` function. This is due to the unchecked nature of the external call. Impact An unchecked external call can lead to the contract relying on potentially faulty or malicious data from the external contract. This could compromise the integrity of the contract's operations, leading to incorrect results or vulnerabilities. Recommendations To mitigate this issue: 1-`Check Results from External Calls...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Division by Zero Error in updateReserve Function

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-123
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/123
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-123.md

## Brief Summary

The `updateReserve` function is responsible for calculating the step size to update the reserve value based on the target price and the current price. It uses the difference between the `pd.lutData.highPrice` and `pd.lutData.lowPrice` to determine the step size. If the `pd.lutData.highPrice` and `pd.lutData.lowPrice` are equal, this means the target price is exactly between the high and low prices in the lookup table. In this case, the expression `(pd.lutData.highPrice - pd.lutData.lowPrice)` will be zero, which will lead to a division by zero error when calculating the step size. Code Impact If the `updateReserve` function encounters a division by zero error, it will revert the entire tran...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Unprotected initializer in 'initNoWellToken' function allowing unauthorized reinitialization of the contract

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-92
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/92
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-92.md

## Brief Summary

The 'initNoWellToken' function is unprotected, posing a high-severity risk by allowing unauthorized reinitialization of the contract, which could result in significant adverse consequences.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Contract contains payable functions but no withdraw/sweep function

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-75
- **Submitter:** jauvany
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/75
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-75.md

## Brief Summary

In smart contract development, particularly for Ethereum, having payable functions without a corresponding withdraw or sweep function can lead to potential issues. Payable functions allow the contract to receive Ether, but without a mechanism to withdraw these funds, the Ether can become locked within the contract indefinitely. This situation might be intentional in some cases (like a burn function), but generally, it’s a design oversight. A withdraw or sweep function is necessary to transfer Ether out of the contract to a specific address, typically the owner's or a designated recipient. Without this, the contract lacks flexibility in managing its funds, potentially leading to lost or inac...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reserve address validation is incorrect; can lead to Division by Zero Error

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-110
- **Submitter:** lonelyprince
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/110
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-110.md

## Brief Summary

A non-zero reserve value when used inside calcLpTokenSupply function can lead to division by Zero Error.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# Gas Griefing through Unbounded Loops in WellUpgradeable Contract

- **Contest:** Basin
- **Slug:** 2024-07-basin
- **Submission:** V-15
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-basin-validation/issues/15
- **Source snapshot:** competitions/2024-07-basin/submissions/raw/V-15.md

## Brief Summary

The vulnerability is identified in the init function, which performs a nested loop to check for duplicate tokens. The WellUpgradeable contract is vulnerable to gas griefing due to an inefficient duplicate token check in its init function. This function uses a nested loop with a time complexity of O(n)^2 to identify duplicate tokens, causing the gas consumption to increase quadratically with the number of tokens. This can lead to excessive gas usage and potential denial of service when the number of tokens is large, as transactions may exceed the block gas limit and fail. Impact: Denial of service due to gas limit exhaustion, making the contract unusable when initializing with a large number...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary
