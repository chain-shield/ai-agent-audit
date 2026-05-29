# Rejected Primary Findings: Reserve Core

# Use of the deprecated `safeApprove` function of openzeppelin containing security vulnerabilities.

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-175
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/175
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-175.md

## Brief Summary

The `BackingManager::grantRTokenAllowance` and the `RevenurTrader::_distributeTokenToBuy` functions make use of `safeApprove()` function. The original ERC20 token `approval` function is susceptible to front-running attacks, where malicious actors can modify quotas after seeing a transaction, but before it is mined. The `safeApprove` function inherited this vulnerability and added unnecessary complexity, making it an additional insecurity. This is why both functions have been deprecated.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Reentrancy Vulnerability in `AssetRegistry::refresh`

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-37
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/37
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-37.md

## Brief Summary

Denial Of Services. A malicious user can overload the system by calling function `AssetRegistry::refresh` again and again to the point where the system becomes unusable for other users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_31_group

# rToken can avoid paying any fees by setting DAOFeeRegistry to a different contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-197
- **Submitter:** 0x52
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/197
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-197.md

## Brief Summary

rToken can avoid paying any DAO fees at all

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Minimum Amount Post-Check Leads to Failed Redemptions

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-100
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/100
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-100.md

## Brief Summary

`redeemCustom()` enables users to redeem their RToken balance for a custom basket of underlying assets in specific proportions. there is an issue with the post-checks performed at the end of the `redeemCustom()` function. The check for the received `amounts` of the expected output ERC20 tokens against the specified minimum `amounts` is incorrect leading to users receiving fewer tokens than expected during consecutive redemptions. Impact If a user attempts to perform consecutive redemptions using the same `minAmounts` values, the second redemption may fail even if the user has sufficient RToken balance. This is because the `minAmounts` array is not updated between consecutive calls, causing...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# Lack of Balance Check In RToken's melt() Enables Over-Burning

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-101
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/101
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-101.md

## Brief Summary

The protocol aims to maintain the stability and value of RToken through various mechanisms, including the ability to mint and burn tokens. The RToken contract in RToken.sol defines the core functionality of RToken, including the `melt()` function, which allows the furnace contract to burn a specified amount of RTokens from the caller's account balance. However, The function lacks check to ensure that the caller has a sufficient balance to perform the melting operation. This oversight allows a caller to burn more RTokens than they actually own, leading to an incorrect reduction in the total supply and an inconsistent state of the system. Impact * An attacker can exploit this vulnerability to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_27_group

# Inconsistent asset registry could cause system instability, leading to unexpected behavior or even system failures.

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-108
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/108
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-108.md

## Brief Summary

`refresh()` function (lines 58-69). This function iterates through the registered assets and calls each asset's `refresh()` function to update its state. If an asset's `refresh()` function throws an exception or fails, it could disrupt the iteration and cause the asset list size to change unexpectedly. This violates the expected behavior of the `refresh()` function, which should maintain a consistent asset list size before and after the refresh process. Impact * If the asset list size changes unexpectedly, it could result in an inconsistent state of the asset registry. This may cause discrepancies between the actual assets and the recorded assets in the registry. * The asset registry's inco...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Inconsistent Last Refresh Timestamp Due to Order of Operations in AssetRegistry.refresh()

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-109
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/109
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-109.md

## Brief Summary

There is an issue with the order of operations in the `refresh()` function `lastRefresh` variable, which stores the timestamp of the last successful refresh, which is updated after calling [basketHandler.trackStatus()](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/BasketHandler.sol#L175-L191). This can lead to inconsistencies if `basketHandler.trackStatus()` reverts due to an exception. [AssetRegistry.refresh()](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/AssetRegistry.sol#L58-L69) Impact If `basketHandler.trackStatus()` reverts, the `lastRefresh` variable will not be upda...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# 'compromiseBasketsNeeded' function in the 'rebalance' function is never called

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-152
- **Submitter:** 13u9
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/152
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-152.md

## Brief Summary

The rebalance function plays a crucial role in restoring balance when the system is undercollateralized. This function updates the asset registry, checks the collateral status, and if necessary, initiates trades to replenish the deficient collateral. If the trades are not sufficient to fully restore collateralization, the compromiseBasketsNeeded function is called to adjust the required collateral to match the actual collateral held. The compromiseBasketsNeeded function ensures the system's safety in an undercollateralized state by taking necessary measures. If this function is not called, the system remains undercollateralized, increasing the associated risks.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# in the Fixed.sol the testSqrt(uint192) function failed when trying to calculate the square root of 1 (the input value x was 1)

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-92
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/92
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-92.md

## Brief Summary

echidna test logs show that the testSqrt(uint192) function failed when trying to calculate the square root of 1 (the input value x was 1): Input value (x): 1 Calculated square root (result): 1000000000 Square of result: 1000000000000000000 Square of (result + 1): 1000000002000000001 Analyzing the Output Square Root Calculation: 1. The calculated square root (result) is 1000000000, which corresponds to 1e9. This seems incorrect because the square root of 1 should be 1, not 1e9. Square of Result: 1. The square of the calculated result is 1000000000000000000, which corresponds to 1e18. This is what we would expect if the input was 1e18 (not 1). Square of (Result + 1): The square of result + 1...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Validation of `portions` Sum May Lead to Incorrect Redemptions (`RTokenP1::redeemCustom`)

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-48
- **Submitter:** Agontuk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/48
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-48.md

## Brief Summary

The [`redeemCustom` function](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/RToken.sol#L253-L344) in the `RTokenP1` contract is responsible for redeeming tokens based on custom basket portions. This function checks that the sum of the `portions` array equals `FIX_ONE` to ensure the portions add up to 1. However, the current implementation sums the `portions` using a `uint256` sum, which may not correctly handle the precision of `FIX_ONE` (a fixed-point number). This can lead to incorrect validation of the `portions` sum, potentially allowing for incorrect redemptions. The [`redeemCustom` function](https://github.com/code-423n4/2024-...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# Malicious users can manipulate redemption values through portion sum exploitation (`BasketHandlerP1::quoteCustomRedemption`)

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-64
- **Submitter:** Agontuk
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/64
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-64.md

## Brief Summary

The `BasketHandlerP1` contract implements a custom redemption mechanism through its [`quoteCustomRedemption()` function.](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/BasketHandler.sol#L521-L602) This function allows users to redeem tokens based on a linear combination of historical basket configurations. It takes two key parameters: `basketNonces` (representing historical basket states) and `portions` (representing the proportion of each basket to be used in the redemption). The function iterates through the provided `basketNonces`, calculating a linear combination of historical baskets based on the corresponding `portions`. Howev...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# RevenueTraderP1 will miss potential rewards due to lack of claimRewards (RevenueTraderP1::manageTokens)

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-74
- **Submitter:** Agontuk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/74
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-74.md

## Brief Summary

The `RevenueTraderP1` contract is designed to manage and trade tokens, converting asset balances to a single target asset for distribution. A key function in this process is [`manageTokens()`](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/RevenueTrader.sol#L109-L184), which handles the management and trading of multiple ERC20 tokens. However, this function lacks a crucial step: claiming rewards before managing or trading the tokens. In the current implementation, [`manageTokens()`](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/RevenueTrader.sol#L109-L184) refreshes the asset...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect removal of asset from mapping in `unregister` function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-148
- **Submitter:** Akay
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/148
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-148.md

## Brief Summary

The `unregister` function does not properly remove the asset from the `assets` mapping. Instead, it sets the value to `IAsset(address(0))`, which can lead to potential issues when checking for the existence of the asset in the mapping. This can cause logical errors in the behaviour of the contract, as the key will still exist in the mapping but with a zero address value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# If the current protocol is Under-Collateralized, users’ Funds may be Frozen.

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-121
- **Submitter:** FastChecker
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/121
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-121.md

## Brief Summary

If the current protocol is under-collateralized, users’ funds may be Frozen due to missing of the bounding handling for each withdrawal in `RToken.sol#redeemTo()` function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Array.sol:: Inefficient Array Operations

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-224
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/224
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-224.md

## Brief Summary

The contract Array.sol uses two functions to check if an array of ERC20 token addresses has unique values. The first function utilizes a nested loop to check each address with every other address in the array. This results in quadratic time complexity (O(n^2)), which may cause inefficient execution with large arrays and may even risk exceeding Ethereum block gas limit. The second function assumes the input array to be sorted in ascending order and checks for uniqueness in linear time (O(n)). While this is more efficient in comparison to the first function, it brings an additional constraint that the array should be in a sorted state before checking for uniqueness. If the ordering of element...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Input Validation Method Controls in Permit.sol

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-232
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/232
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-232.md

## Brief Summary

The function requireSignature() in the contract Pernit.sol does not properly validate the input that the contract receives. Having insufficient checks of the `owner` variable means that there is potential for incorrect or malicious data to be processed by the contract. This validates whether the provided `owner` address is a contract or not using the `isContract` call. If it's a contract, it uses the `isValidSignature` call (which is standard defined in ERC1271) to check if the signature is valid. But if the `owner` address is not a contract (which would typically indicate an EOA address), then it checks if the signature is valid using the `isValidSignatureNow` function. There's a security...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Insufficient Input Validation in the `toLower` function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-239
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/239
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-239.md

## Brief Summary

The function `toLower` in the contract String.sol has a potential vulnerability because it lacks proper input validation. It only accounts for basic ASCII uppercase characters (A-Z) and does not appropriately handle multi-byte characters like 'Ö', 'À', or 'Æ'. It only changes the case for ASCII characters, making it lowercase.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient Gas Validation in AssetRegistry.sol

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-253
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/253
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-253.md

## Brief Summary

The function `_reserveGas()` in the contract AssetRegistry.sol is responsible for ensuring that appropriate gas is available when executing contract methods. The function aggregates a fixed amount of gas buffering value and a quantity-related variable amount of gas which is reserved for invoking external contracts. Though the developer has taken into consideration gas buffer before invoking the external contracts, the contract's gas estimation may not be entirely accurate and could result in unintentional contract operations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# BackingManager.sol:: Reentrancy Vulnerability in the method settleTrade

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-255
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/255
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-255.md

## Brief Summary

The contract BackingManager.sol may potentially contain a reentrancy vulnerability due to the lack of usage of reentrancy guards in the `settleTrade` method. If the `settleTrade` method is called by another contract, it calls the `rebalance` function, which in turn makes a call to an external contract. An attacker can potentially manipulate the state changes at this moment, facilitating a reentrancy attack.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# Nonexistent Basket Nonce Check in BasketHandler.sol

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-259
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/259
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-259.md

## Brief Summary

An important consideration when dealing with nonces in smart contracts (which are necessary for keeping a track of changes or transactions in a system) is verifying their validity. In BasketHandler.sol, there doesn't appear to be a sufficient check for nonexistent basket nonces. Specifically, in the `getHistoricalBasket` function, the basket nonce is used to retrieve a historical basket based on it. However, there is no check to determine if a basket with that nonce doesn't exist, which could lead to an unexpected behavior or errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Uncontrolled Recursion Vulnerability In the `_switchBasket` function can result in DOS

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-262
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/262
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-262.md

## Brief Summary

In several aspects, the contract BasketHandler.sol use of recursive calls or iterations that could potentially run indefinitely or until the system reaches its gas limit, resulting in Denial of Service or other unwanted system performance impediments. Appropriate measures must be taken into consideration to prevent such occurrences.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `settleTrade` allow race condition because the lack of access control

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-132
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/132
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-132.md

## Brief Summary

`settleTrade` function is being called by the `DutchTrade` auction contract when someone make a bid but also can be called in the `backingManager` bassicly deleting the trade and sending the token to 0 address which is a waste of money of loss of money for the project. The problem is that this allow attacker to front run the `bid` tx settle the trade in the `backingManager` manager diracly making the project loss money. [[Link]](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/plugins/trading/DutchTrade.sol#L222) Note that this is always true when the `block.timestamp > endTime` Impact Reserve protocol could loss money because in case tha...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `payoutRewards` function can be called directly in the stRSR possibly altering the normal behavior of the protocol.

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-49
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/49
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-49.md

## Brief Summary

`payoutRewards` Assign reward payouts to the staker pool modifying the stakeRSR and payoutLastPaid variables as appropriate, given the current value of rsrRewards(). This function is been used in the `forwardRevenue` function and `seizeRSR` in the StRSR token As we can see the _payoutRewards is modifying the `stakeRate`, `payoutLastPaid` and `rsrRewardsAtLastPayout`, those are important variables used in the StRSR contract. Also we can see in the first arrow above that `_payoutRewards` can only be called once by timestamp. The problem is that an attacker can always front run the `seizeRSR`, `stake`, and `unstake` calling `payoutRewards()` making `stakeRate`, `payoutLastPaid` and `rsrRewards...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# melt function can be called by anyone front running some important action of the system

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-50
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/50
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-50.md

## Brief Summary

`melt function` Performs any melting that has vested since last call. Also it control the payout balance: [[Link]](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/Furnace.sol#L65) As you can see in the arrow above this function can only be called once per timestamp, this open the door for attackers front running this function calling it before some important action is triggered. We can see the `_distributeTokenToBuy` function in the RevenueTrader: [[Link]](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/RevenueTrader.sol#L189C4-L197C6) The `distributor` then sent the tokens to t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_34_group

# Incorrect rounding direction in `_scaleDown` Rtoken

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-52
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/52
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-52.md

## Brief Summary

we all now that solidity does not support float values does why division always round to the lower decimals. projects also have to round in favor of the project. In this case of reserve protocol there is a instance that can be exploited. Let see the `_scaleDown` function which is called by the redeems functions: [[Link]](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/RToken.sol#L508C5-L516C6) As you can see the `amtBaskets` is rounding down this allow an attacker to burn a low amount of tokens but not decrement the basket need. An attacker can do the next scenario: 1. issue `rtokens` increasing the `basketsNeeded`. 2, redeem a low am...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Attacker can `unstake` and ` cancelUnstake` manipulating some variables in `StRSR`

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-79
- **Submitter:** Jorgect
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/79
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-79.md

## Brief Summary

The `StRSR` is putting users in a quote is the user want to `withdraw`, this prevent an attacker to `deposit` and `withdraw` in the same timestamp and perform some manipulations front running a tx and then backruning. The problem is that users can call `unstake` frontrunning a tx and then backrun with `cancelUnstake` to perform manipulation. See impact Impact Attacker first have to `stake`, then he can call freely `unstake` and `cancelUnstake` manipulating some important state variable as `totalDrafts`, `draftRSR` and the most important `stakeRSR` with that been said all functions that use this variables can be front runed doing some damage. See the next scenario: 1. Attacker `stake` (if us...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `price` in the `BasketHandler` could be major than the FIX_MAX

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-98
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/98
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-98.md

## Brief Summary

As you can see in the first arrow the high256 could not be major than the FIX_MAX. The problem is that in case that the value is high256 less a little amount with the premium fee could surpass the maximum amount(see second arrow above). Impact The `price` function could return a high value higher than the possible value. Tool Used Manual

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_66_group

# Large Transfer Failure in StRSRP1 Contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-85
- **Submitter:** JuggerNaut63
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/85
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-85.md

## Brief Summary

The contract fails to handle large transfers correctly for some ERC20 tokens (e.g., UNI, COMP) when the transfer amount exceeds uint96. This can lead to failed transactions in critical functions such as staking, withdrawing, and seizing RSR, potentially causing a denial of service for users with large balances.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Uneven token distribution in `RToken` redemption process

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-112
- **Submitter:** K42
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/112
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-112.md

## Brief Summary

So there is a bug is in [redeemCustom()](https://github.com/code-423n4/2024-07-reserve/blob/main/contracts/p1/RToken.sol#L253) function of the [RToken.sol](https://github.com/code-423n4/2024-07-reserve/blob/main/contracts/p1/RToken.sol) contract. Current logic is not correctly distributing the underlying tokens during the redemption process, currently distributing vastly different amounts of tokens being returned to the user. Impact - Users redeeming `RTokens` receive drastically different amounts of underlying tokens, with some tokens being severely underrepresented. - Financial losses for users redeeming `RTokens`. - It currently undermines the fundamental principle of the `RToken` system...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Over-Issuance of `RTokens` because no effective allowance checks

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-95
- **Submitter:** K42
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/95
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-95.md

## Brief Summary

[issueTo()](https://github.com/code-423n4/2024-07-reserve/blob/main/contracts/p1/RToken.sol#L105) of [RToken.sol](https://github.com/code-423n4/2024-07-reserve/blob/main/contracts/p1/RToken.sol#L105). This function allows for the minting of `RTokens` without assertively checking or updating token allowances before transferring the tokens from the issuer to the backing manager. I place this as a medium risk given `notIssuancePausedOrFrozen` modifier allows for pausing issuance, but still the bug needs attention and is not hard to mitigate. Impact So the issue is that it allows users to mint `RTokens` without effective allowance checks, opening: 1. Opening up unauthorized minting of `RTokens`...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# issue() 's availability will be impacted due to incorrect conditional in issuancePremium()

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-134
- **Submitter:** Kalyan-Singh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/134
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-134.md

## Brief Summary

issue() either reverts or forces users to take more damage than desired.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# MinTradeVolume is subtracted more times than it should

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-135
- **Submitter:** Kalyan-Singh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/135
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-135.md

## Brief Summary

wrong calculation of UoABottom which leads to wrong basketRange leading to wrong buy and sell amounts for rebalance

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Freezing of unclaimed yield due to insufficient checks in forwardRevenue() of BackingManager

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-156
- **Submitter:** Kalyan-Singh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/156
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-156.md

## Brief Summary

Freezing of unclaimed yield due to insufficient checks in forwardRevenue() of BackingManager

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# WETH compatibility in Arbitrum

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-80
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/80
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-80.md

## Brief Summary

Users can't bid with WETH tokens in arbitrum

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Integer Overflow in _price Function of Dutch Auction Due to Unchecked bestPrice Limits

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-233
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/233
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-233.md

## Brief Summary

The DutchTrade contract implements a Dutch auction mechanism for token trading. The `_price` function is crucial as it determines the current price of the auction at any given timestamp. The `_price` function has a potential integer overflow in the geometric decay calculation during the first 20% of the auction period. In the first phase of the auction (0-20% progression), the function uses a geometric decay formula: The problem arises because there's no upper bound check on `bestPrice`. The function itself notes: However, this reversion is not explicitly handled, and there's no check to prevent such high values of `bestPrice`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted Access to Critical State Update

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-236
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/236
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-236.md

## Brief Summary

The `cacheComponents` function in the DistributorP1 contract is designed to update critical contract references after an upgrade. However, it lacks access control, potentially allowing unauthorized parties to manipulate the contract's state. The function `cacheComponents` is declared as `public`, meaning any external actor can call it. This function updates critical contract references, including: - RSR and RToken addresses - Furnace and StRSR contract addresses - RToken and RSR trader addresses These references are crucial for the correct functioning of the Distributor, particularly in the `distribute` function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_37_group

# Missing Total Distribution Validation in _setDistribution Function of DistributorP1 Contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-238
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/238
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-238.md

## Brief Summary

The `DistributorP1` contract manages the distribution of revenue (RSR and RToken) to various destinations. The `_setDistribution` function is a core internal function used to set or update these distributions. The `_setDistribution` function lacks a crucial check to ensure that the total distribution across all destinations remains valid after each update. While the function checks individual distribution shares against `MAX_DISTRIBUTION`, it doesn't verify that the sum of all distributions remains within acceptable bounds after an update. This could lead to a situation where the total distribution exceeds 100% or falls below the required minimum.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_41_group

# Potential Loss of Fees Due to Uninitialized Fee Recipient

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-243
- **Submitter:** NexusAudits
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/243
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-243.md

## Brief Summary

The constructor allows the `feeRecipient` to be set to the zero address (address(0)) without any checks. If this occurs, fees will be sent to the zero address until the owner manually updates the fee recipient, resulting in permanent loss of funds.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Loss of Small Surplus Amounts in Revenue Distribution

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-251
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/251
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-251.md

## Brief Summary

The `forwardRevenue` function in the `BackingManagerP1` contract is responsible for distributing surplus assets to RToken and RSR holders. It calculates the excess amount of each asset and attempts to distribute it proportionally. The current implementation may fail to distribute small surplus amounts due to integer division rounding down to zero, potentially leading to asset accumulation in the contract over time. In the distribution logic, `tokensPerShare` is calculated as: When `delta` (the surplus amount) is smaller than `(totals.rTokenTotal + totals.rsrTotal)`, `tokensPerShare` becomes zero due to integer division. This results in skipping the distribution for that asset entirely. Impa...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_65_group

# Potential DoS Attack in BackingManagerP1's rebalance Function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-252
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/252
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-252.md

## Brief Summary

The `rebalance` function in the BackingManagerP1 contract is responsible for maintaining proper collateralization of the RToken system. It can execute trades to adjust collateral levels and is designed to be called periodically. The function includes a check to prevent multiple rebalances in the same block: However, this check can be exploited by a malicious actor to prevent legitimate rebalancing attempts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# MEV strategy on `stRSR::seizeRSR` without subject to staking risk during rebalances

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-202
- **Submitter:** ParaTroopers
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/202
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-202.md

## Brief Summary

Staking at a low stake rate and unstaking at a high stake rate in the same block results in the extarction of value. Although the withdraw of funds is done after `availableAt`, the staking is not subjected to risk.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# BaskedHandler.sol init will revert when upgrading contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-55
- **Submitter:** ParaTroopers
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/55
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-55.md

## Brief Summary

The upgrade from version *3.4.0* to *4.0.0*, will always revert for BasketHandler.sol! Description The `init` function in BasketHandler.sol version *4.0.0* introduces a new storage variable *enableIssuancePremium* But then it uses an `initializer` modifier which will revert for a BasketHandler proxy that was already initialized!

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unable to start the gnosis auction

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-123
- **Submitter:** PrasadLak
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/123
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-123.md

## Brief Summary

When a collateral token defaults, the protocol needs to quickly and efficiently recapitalize to ensure the stability and redeemability of the RToken. This is done by selling off the defaulted collateral through auctions and using the proceeds to purchase predefined emergency collateral. On that occasion gnosis auction model is used to auction the default collateral. But when initiating gnoiss auction its reverted . Due to gnosis auction model is failed to start , protocol is going to a insolvent state. Impact Its unable to start the gnosis auction due to revert `init` calling transaction. So Protocol is going to insolvent state due to unable to initiate the gnosis auction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# DOS stakeAndDelegate function due to not to write _checkpoints during minting

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-144
- **Submitter:** PrasadLak
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/144
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-144.md

## Brief Summary

`stakeAndDelegate` is implemented to stake RSR and delegate simultaneously.But when staking its not implemented a method to write _checkpoints balance so it caused DOS when [_delegate](https://github.com/code-423n4/2024-07-reserve/blob/main/contracts/p1/StRSRVotes.sol#L192C8-L197C42). Impact Due to no to add the balance to `_checkpoints` during the stake RSR , delegation is reverted duet to underflow.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# delegatee able to use signer signature for replay (Cross chain signature replay)

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-176
- **Submitter:** PrasadLak
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/176
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-176.md

## Brief Summary

The `delegateBySig` function is part of a contract that allows for delegation of voting power using off-chain signed messages. But here malicious delegatee able to re use the signer signature for other chain also. Past occurrences of this issue: https://solodit.xyz/issues/cross-chain-replay-attack-vulnerability-in-beanstalks-l2contractmigrationfacet-codehawks-beanstalk-the-finale-git Impact A malicious user could exploit the vulnerability by reusing the signer's signature to transfer voting power to themselves, potentially leading to a governance attack.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# SHORT_FREEZER role revocation occurs before freeze application

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-126
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/126
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-126.md

## Brief Summary

The `freezeShort()` function in the `Auth.sol` contract currently revokes the SHORT_FREEZER role before applying the system freeze. This implementation deviates from the intention described in the docs: 1. If the freeze application fails after the role revocation, the system will be left in a vulnerable state without the SHORT_FREEZER's ability to take action. 2. The premature loss of the SHORT_FREEZER role reduces the system's resilience and capacity to respond swiftly to emergencies.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Redemption quantities not adjusted for mempool time leading to potential over-redemption

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-139
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/139
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-139.md

## Brief Summary

Users may receive more collateral tokens than expected during redemptions if the transaction stays in the mempool for an extended period

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect return values for ERC20 functions.

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-87
- **Submitter:** Ruandevos
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/87
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-87.md

## Brief Summary

Detailed description of the impact of this finding. Some older or non-standard ERC20 tokens don’t return the expected boolean values for functions like approve, transfer, and transferFrom. This is an issue because, starting from Solidity 0.4.22, these functions are supposed to return a true or false indicating whether the operation was successful. If a token doesn’t return this value, any contract built with Solidity 0.4.22 or later that tries to interact with these tokens might fail. This could lead to transactions not going through, which might disrupt your processes or even cause financial losses if it’s not caught early.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_82_group

# Governance Resets Draft Records Even When DraftRate is Safe or Healthy Which Could Cause Loss of Funds or DOS

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-145
- **Submitter:** SUPERMAN_I4G
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/145
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-145.md

## Brief Summary

The `resetStakes()` function is designed as a safety mechanism to reset all stakes and advance the era when the `stakeRate` is deemed unsafe. However, the function also resets the draft era by calling `beginDraftEra()` even when the `draftRate` is healthy. This action negatively impacts users who are withdrawing their `RSR` (already unstaked), as it forces a reset of their draft records. Consequently, users who have requested unstaking or are ready to fully withdraw their `RSR` (`availableAt` passed) will face losses, as their draft records will be reset/wiped out.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Defaulted collaterals can be added to the prime basket via `BasketHandler._setPrimeBasket` function, thus leading to sub-optimal collateral compositions for baskets

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-218
- **Submitter:** Shield
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/218
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-218.md

## Brief Summary

The `lack of a CollateralStatus.SOUND status check` in the `BasketHandler._setPrimeBasket` function could potentially lead to an issue where defaulted collaterals are included in the prime basket, resulting in an imbalance between the `totalWeight` and `goodWeight` calculations in the `BasketLib.nextBasket` function. Following is a detailed analysis of the issue: In the `BasketHandler._setPrimeBasket` function, there is `no explicit` check to ensure that the collaterals being added to the `prime basket` are in the `CollateralStatus.SOUND state`. The function only checks if the collateral is registered `(assetRegistry.toAsset(erc20s[i]).isCollateral())` and if the target amount is within the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Oracle error introduced in the `issuancePremium`, could result in erroneous prices for the auctions at a particular timestamp

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-225
- **Submitter:** Shield
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/225
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-225.md

## Brief Summary

In the `BackingManager.rebalance` function the `RecollateralizationLib.prepareRecollateralizationTrade` function is called. In the `prepareRecollateralizationTrade` function the `RecollateralizationLib.nextTradePair` function is called to determine the trading parameters for the `new trade auction` which is going to be initiated to rebalance the basket. The `RecollateralizationLib.nextTradePair` function calls the `Asset.price()` function of each of the registered assets in the `reserve protocol` as shown below: The issue here is if multiple `RTokenCollaterals` are registered in the `reserve protocol` then the `nextTradePair` function will call the `price()` function of each of those `RToke...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# `BasketHandler.quantityUnsafe` function does not verify if the `erc20` is registered in the `AssetRegistry` thus leading to erroneous auction trade execution

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-230
- **Submitter:** Shield
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/230
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-230.md

## Brief Summary

In the `BasketHandler.quantityUnsafe` function it is assumed that the asset is correct which means the passed in erc20 is correctly mapped to the asset. But there is no check in place to ensure that the `erc20` is registered in the `AssetRegistry`. Prior to calling the `BasketHandler._quantity` function by respective transactions, it is verified that the `erc20` is registered in the `AssetRegistry.toColl` as shown below: But it is not verified that the `erc20` is registered in the `AssetRegistry` while calling the `BasketHandler.quantityUnsafe` function as shown below: Hence an unregistered `erc20` token could be used in the `BackingManager`. The `BackingManager.tradingContext` function cal...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Meta transactions are not supported in the `DutchTrade` contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-235
- **Submitter:** Shield
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/235
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-235.md

## Brief Summary

Through out the important contracts of the `reserve` protocol the `meta transactions` are supported by inheriting from the `OZ ContextUpgradeable`. We can see that in the `BasketHandler.sol` and `BackingManager` contracts. The `DutchTrade` is an important contract in the `reserve` protocol since it enables the functionality to `initiate auctions`, `initiate bids` and `settle the auctions`. The issue here is this contract uses `msg.sender` thus not supporting the `metaTransactions`. Hence there is a discrepancy in how the `reserve` protocol supports the `metaTransactions`. This could lead to broken operations in the `reserve protocol` when the transactions are executed via a `relayer` for me...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Error Handling in RevenueTraderP1() ::settleTrade

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-170
- **Submitter:** Sid_Sisodia
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/170
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-170.md

## Brief Summary

In the settleTrade function, the try-catch block surrounding the call to this.distributeTokenToBuy() silently ignores errors with non-empty error data. This could lead to undetected failures in token distribution, causing financial discrepancies or operational issues if the function does not execute as expected. Impact Although the issue may not directly lead to a security vulnerability, it could cause operational inefficiencies and lead to unexpected behavior in the system, undermining the contract’s reliability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_36_group

# Overflow in Fixed.sol::mul() Function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-172
- **Submitter:** Sid_Sisodia
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/172
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-172.md

## Brief Summary

The mul() function is designed to perform multiplication on uint256 values. However, it lacks appropriate overflow checks, which makes it susceptible to arithmetic overflow. This occurs when the product of two large numbers exceeds the maximum value that can be stored in a uint256 variable (2^256 - 1). When this happens, the function may produce incorrect results by wrapping around the value or causing the transaction to revert unexpectedly Impact The overflow vulnerability in the mul() function poses a severe risk to the contract's integrity and security. Depending on the function's role within the contract, this could result in: Loss of Funds: Incorrect calculations due to overflow can le...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Lack of Access Control in AssetRegistryP1 Contract :: unregister(IAsset asset)

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-183
- **Submitter:** Sid_Sisodia
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/183
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-183.md

## Brief Summary

The unregister function allows for the removal of an asset from the registry. This function is critical because it can disrupt the functionality of dependent contracts if an asset is removed unexpectedly or maliciously. The function currently lacks any access control, meaning any external user can call it. Impact Without access control, a malicious user could unregister critical assets, leading to the breakdown of functionality in dependent contracts and causing significant disruptions to the system. This could result in financial losses and operational failures. Exploit Scenario: An attacker identifies a key asset in the registry that is integral to the system’s operations. The attacker ca...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_80_group

# Reentrancy Concerns with External Calls in refreshBasket() function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-192
- **Submitter:** Sid_Sisodia
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/192
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-192.md

## Brief Summary

The refreshBasket() function within the BasketHandlerP1 contract calls the external function assetRegistry.refresh(), which could interact with other contracts or components. Following this, the function _switchBasket() is called, which also potentially interacts with external components. If any of these external calls result in reentrant calls back into the BasketHandlerP1 contract, and if state changes occur before these external calls are finalized, there could be a vulnerability that allows reentrancy attacks. Impact If an attacker successfully exploits this vulnerability, they could manipulate the state of the contract, potentially bypassing key contract logic or triggering unintended...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Incorrect Throttling in `currentlyAvailable` Function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-256
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/256
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-256.md

## Brief Summary

The `currentlyAvailable` function in `Throttle.sol` contains a logic error that incorrectly caps the available amount to the hourly limit, even when more than an hour has passed since the last update. This bug prevents the accumulation of unused capacity over extended periods, leading to unintended throttling of actions that should be allowed. The problematic code is in the currentlyAvailable function: The last line incorrectly caps the available amount to the hourly limit, regardless of how much time has passed. Impact - Loss of Accumulated Availability: If more than one hour passes between updates, the system fails to account for the full amount of tokens that should be available. This ef...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Double Registration of Proxy Tokens in Asset Registry

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-258
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/258
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-258.md

## Brief Summary

The AssetRegistry contract does not have a mechanism to detect or prevent the registration of double-entry point tokens (also known as proxy tokens). This could allow the same underlying asset to be registered multiple times with different addresses, potentially leading to incorrect accounting and vulnerabilities in the protocol's economic model. The current implementation checks for duplicates based solely on the ERC20 token address, which is insufficient for detecting proxy tokens that can have multiple entry points representing the same underlying asset. Impact - Double Counting: Assets could be counted twice in the system, inflating the perceived total value of registered assets. - Inco...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Systemic Vulnerability Due to Rigid Asset Dependency in AssetRegistry

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-261
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/261
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-261.md

## Brief Summary

The AssetRegistry contract lacks robust mechanisms to handle scenarios where registered collateral assets become compromised or malfunction. This vulnerability stems from the assumption that all registered assets will consistently behave as expected throughout their lifecycle. However, various events such as token upgrades, hacks, or contract self-destructs can invalidate this assumption, potentially leading to a system-wide failure. The core of this vulnerability lies in three main areas: - Asset Registration Process: The current implementation lacks comprehensive validation and contingency measures for newly registered assets. - Refresh Mechanism: The system-wide refresh function fails to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# The `IAsset::erc20` function lacks implementation

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-247
- **Submitter:** Spomaria
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/247
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-247.md

## Brief Summary

The `IAsset::erc20` function has its function definition in the `IAsset` interface which is supposed to return `IERC20Metadata`. In simple terms, the function is designed to add additional features to the `IERC20` interface such as the ability to get the `name`, `symbol` and `decimals` of an ERC20 token. In addition, if a token is `IERC20Metadata`, it means the `name`, `symbol` and `decimals` of the token can be viewed using the additional functions defined in the `IERC20Metadata` interface. Vulnerability Details The vulnerability lies in the fact that 1. though the `IAsset::erc20` function is defined in the `IAsset` interface, its implementation is not defined before being called. For inst...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_43_group

# No check for sequencer uptime in `DutchTrade` can lead to dutch auctions failing or executing at bad prices

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-189
- **Submitter:** Ward
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/189
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-189.md

## Brief Summary

The `DutchTrade` contract implements a wholesale dutch auction via a 4-piecewise falling-price mechansim. However, there is no check for sequencer uptime, which could lead to auctions failing or executing at unfavorable prices. Current deployment parameters set `dutchAuctionLength` to [15 minutes](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/docs/deployment-variables.md?plain=1#L116) by default in L2s and accept the "Reasonable range" as 100 to 3600 seconds. This could have serious consequences in the event of a network outage. Network outages and large reorgs happen with relative frequency. For instance, Arbitrum suffered a [4 hour-long outage...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_33_group

# It is impossible to use `cancel` in a meaningful context

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-260
- **Submitter:** Ward
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/260
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-260.md

## Brief Summary

The `Governance::cancel()` function overrides the `cancel` implementation from OpenZeppelin's `Governor.sol`. However, it is impossible to use `cancel` in a meaningful context because the require statement in `cancel` requires `startedInSameEra` to return false for the function to execute. The critical problem here is that for a proposal to be queued or executed, `startedInSameEra` needs to return true, meaning it is impossible to `cancel` any proposal that is queueable or executable, and we cannot bypass this require with any role, making it completely impossible. If it is not the same era, `queue` and `execute` will not work anyway and *anyone can run cancel*. The sponsor has also tested...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# in `BackingManagerP1::rebalance` Wrong Require will lead to non-intended behavior

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-160
- **Submitter:** WildSniper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/160
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-160.md

## Brief Summary

Unintended behavior and unchained trades as commented by the protocol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# Invariant check

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-217
- **Submitter:** bakwas
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/217
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-217.md

## Brief Summary

Failing to ensure that tradingDelay and backingBuffer are set within valid ranges can lead to improper functioning of the contract. This could result in delayed trades or insufficient collateral buffers, potentially destabilizing the RToken. Specifically: An excessively high tradingDelay might prevent timely execution of trades, leading to delays in rebalancing and potential undercollateralization. An excessively high backingBuffer could impose unnecessarily high collateral requirements, reducing capital efficiency and possibly affecting the protocol's liquidity.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Distribution Logic: Ensure distributor.distribute Handles Large Transfers and Potential Failures Gracefully

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-222
- **Submitter:** bakwas
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/222
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-222.md

## Brief Summary

Loss of Funds: If the distributor.distribute function cannot handle large token transfers properly, it could lead to the loss of funds, where the tokens might remain stuck in the contract or cause a transaction to fail altogether. Failed Transactions: A failure in distributor.distribute could result in the entire transaction reverting, which could disrupt the revenue distribution process and lock funds within the contract. Unintended Behavior: Without proper error handling, any issues in the distribution process could lead to unexpected behaviors, potentially affecting the contract's intended operations.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# wrong implement of "_register"

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-146
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/146
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-146.md

## Brief Summary

Detailed description of the impact of this finding. wrong implement of _register.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# no _payoutRewards() in withdraw function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-196
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/196
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-196.md

## Brief Summary

Detailed description of the impact of this finding. No _payoutRewards() in withdraw.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# wrong calclation of totalStakes in mintStakes

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-198
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/198
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-198.md

## Brief Summary

Detailed description of the impact of this finding. wrong calculation of totalStakes in mintStakes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# NO check for the price in manageTokens

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-201
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/201
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-201.md

## Brief Summary

Detailed description of the impact of this finding. There is no price check in manageTokens. There is no check for buyLow and there is no check for sellLow and sellHigh.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No access control in deployRTokenAsset

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-213
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/213
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-213.md

## Brief Summary

Detailed description of the impact of this finding. anyone can call deployRTokenAsset and deploy.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Execution Halt Due to Multiple Calls in a Single Transaction in the setRatio function on Furnace contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-105
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/105
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-105.md

## Brief Summary

Detailed description of the impact of this finding. Contract: FurnaceP1 Function name: setRatio(uint192) PC address: 2499 Estimated Gas Usage: 30644 - 164314 Multiple calls are executed in the same transaction The failure of a call within the setRatio(uint192) function can halt the transaction, preventing subsequent operations from executing. This could lead to: • Disrupted contract logic, affecting the stability and reliability of the contract. • Potential exploitation by attackers to prevent the contract from executing as intended. • Financial loss or disruption of contract services. This call is executed following another call within the same transaction. It is possible that the call nev...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Dependence on Predictable Environment Variable in the delegateBySig function on the StRSRVotes contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-129
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/129
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-129.md

## Brief Summary

Detailed description of the impact of this finding. Contract: StRSRP1Votes Function Name: delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32) PC Address: 12248 Estimated Gas Usage: 1427 - 1522 The delegateBySig function relies on the block.timestamp environment variable to determine whether a signature has expired: Using block.timestamp to enforce expiration can be problematic because it is a predictable and manipulatable value. A malicious miner could manipulate the timestamp of a block within a reasonable range to allow a transaction to pass or fail, depending on their incentive. This introduces a trust assumption on miners, which could lead to unexpected behaviours or security v...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_101_group

# Improper State Transition Handling on the bid function in the DutchTrade contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-138
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/138
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-138.md

## Brief Summary

Detailed description of the impact of this finding. Contract: DutchTrade Function name: bid() PC address: 16558 Estimated Gas Usage: 2026 - 2971 The bid() function in the DutchTrade contract contains a critical vulnerability related to improper state transition handling, which could be exploited to disrupt the auction process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_102_group

# Unchecked Return Values on the Deploy function in the Deployer contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-96
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/96
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-96.md

## Brief Summary

Detailed description of the impact of this finding. The deploy function does not check the return values of the init function calls. If any of these calls fail silently, the system could be deployed in a partially uninitialised or broken state. This could lead to undefined behaviour and potential security vulnerabilities.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unprotected settleTrade function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-27
- **Submitter:** ebbieaden
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/27
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-27.md

## Brief Summary

The `function settleTrade()` is called `external` and unprotected making it vulnerable to unauthorized access and manipulation allowing attacker's to disrupt the functionality of the contract and drain tokens

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_42_group

# RToken Upgrade Vulnerability

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-194
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/194
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-194.md

## Brief Summary

The MainP1 contract's RToken upgrade process has a flaw where can bypass the version check, enabling them to upgrade the RToken component without updating the version. Root Cause The version check performed in the `upgradeRTokenTo` function of the MainP1 contract (Main.sol:111-150). Specifically: [Main.sol#L117](https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/Main.sol#L117) This line checks if the current version of the main contract matches the provided `versionHash`. If it doesn't match, the upgrade process is halted. However, if the `versionHash` passed to the function is the same as the current version, the upgrade will proceed wi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_117_group

# when decimal is 6, ``quantities`` will be zero

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-131
- **Submitter:** laksmana
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/131
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-131.md

## Brief Summary

The result of ``quantities[i]`` will be an underflow, when ``erc20s[i])).decimals()`` is 6 and the rounding is ``FLOOR``. So, a function that triggers ``BasketHandler#quote`` then the decimal token is 6 and the rounding is ``FLOOR``, will underflow.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# no access control on `_authorizeUpgrade`

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-210
- **Submitter:** mgf15
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/210
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-210.md

## Brief Summary

`MainP1` contract inherits from `UUPSUpgradeable` and overrides the `_authorizeUpgrade()` function. However, without adding access control to this function, anyone can call it to set the implementation contract. This lack of restriction allows an attacker to set a malicious contract as the implementation, which can then use the `selfdestruct()` function to destroy the proxy contract. This is possible because the implementation is called using delegatecall from the proxy.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# wrong check in basketlib.sol

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-191
- **Submitter:** nikhilx0111
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/191
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-191.md

## Brief Summary

Total Weight: Represents the cumulative weight of all assets in the basket It’s used to determine how much weight is actually achieved with the assets present in the basket. Good Weights: This is the total weight of assets that are considered good (valid and non-zero) according to the docs and comments During the basket selection process, if the total weight is less than good weights (required target weight) backup weights are considered. however due to a vulnerability when the total weights is less than good weights in the backup weights wont be considered The condition totalWeights[i].lte(goodWeights[i]) checks whether the total weight for the target is less than or equal to the weight pr...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# rsr stakers in draft can avoid getting slashed

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-250
- **Submitter:** nikhilx0111
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/250
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-250.md

## Brief Summary

Stakers who stake RSR can have their tokens seized by the protocol. To prevent RSR stakers from bypassing slashing, users must wait until the withdrawal delay period is over before they can withdraw their staked RSR. When users unstake their RSR, it is moved to `rsrdraft`. The problem arises when there is an ongoing slashing: RSR in `staked.rsr` is seized first. A user who is aware of the ongoing slashing and has their RSR in `rsrdraft` can potentially bypass the slashing.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Access Control in Withdraw Function

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-110
- **Submitter:** nour99
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/110
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-110.md

## Brief Summary

The withdraw function in the smart contract lacks proper access control mechanisms. This function is marked as external, allowing any external account or contract to call it. However, there is no verification to ensure that only the account owner or authorized parties can initiate a withdrawal. This oversight could potentially allow unauthorized users to withdraw funds from any account. Impact The absence of access control in the withdraw function poses a critical security risk. An attacker could exploit this vulnerability to: - Withdraw funds from any account without authorization. - Drain the contract of its assets. - Disrupt the normal operation of the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# malicious deployer can use deprecated versions

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-155
- **Submitter:** pashap9990
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/155
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-155.md

## Brief Summary

Owners can deprecate old versions if there is a critical bug in old versions that can hurt users ,if a version have been deprecated ,deployers still can use that version

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Leak of value due to reversed rounding in req struct newBatchAuction function in Broker.sol contract

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-185
- **Submitter:** rzizah
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/185
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-185.md

## Brief Summary

reversed rounding in **Broker.sol** contract in **newBatchAuction** function leading to leakage of value

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# AssetPluginRegistry.sol

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-137
- **Submitter:** umangjainshah
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/137
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-137.md

## Brief Summary

// SPDX-License-Identifier: BlueOak-1.0.0 pragma solidity 0.8.19; import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol"; import { VersionRegistry } from "./VersionRegistry.sol"; import { RoleRegistry } from "./RoleRegistry.sol"; import { ReentrancyGuard } from "@openzeppelin/contracts/security/ReentrancyGuard.sol"; /** * @title Asset Plugin Registry * @notice A tiny contract for tracking asset plugins */ contract AssetPluginRegistry is ReentrancyGuard { VersionRegistry public immutable versionRegistry; RoleRegistry public immutable roleRegistry; // versionHash => asset => isValid mapping(bytes32 => mapping(address => bool)) private _isValidAsset; mapping(address => bool) publ...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reward distribution will fail when there are backlisted users.

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-184
- **Submitter:** zraxx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/184
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-184.md

## Brief Summary

Reward distribution will fail, which leads to the DOSes of the core functions. Details The distribute function is used to distribute rewards to all users, and safeTransferFrom is used to distribute ERC20 tokens. However, this does not take into account that if one of the users is on the blacklist, safeTransferFrom will fail, causing the entire function to be reverted.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Function manageTokens can be front-runned, causing user calls to fail

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Submission:** V-187
- **Submitter:** zraxx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-reserve-validation/issues/187
- **Source snapshot:** competitions/2024-07-reserve-core/submissions/raw/V-187.md

## Brief Summary

Function manageTokens can be front-runned, causing user calls to fail. Details Function manageTokens is used to trade in exchange for tokenToBuy. However, malicious users can front-run it, causing normal users' calls to fail. Firstly, `_distributeTokenToBuy` will revert when the reward amount is zero. Secondly, when the `trades[erc20]` is not zero, it also will revert. So, malicious can expliot it to make legitimate users' transactions fail.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group
