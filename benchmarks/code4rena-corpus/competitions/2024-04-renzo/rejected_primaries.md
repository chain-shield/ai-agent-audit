# Rejected Primary Findings: Renzo

# xRenzoDeposit::_recoverBridgeFee() always assumes the native has a wrapper, which may not be the case

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1050
- **Submitter:** 0x73696d616f
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1050
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1050.md

## Brief Summary

Admin is not able to withdraw fees from `xRenzoDeposit` deposits, losing these funds forever.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, unknown, :robot:_primary, :robot:_25_group

# `ezETH` Transfer Restrictions Bypassed for Zero Address Transfers

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-855
- **Submitter:** 0xAadi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/855
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-855.md

## Brief Summary

The `EzEthToken` contract's `_beforeTokenTransfer` function does not prevent transfers to the zero address when the contract is paused, potentially allowing for unintended token burns and bypassing transfer restrictions. Impact This issue could lead to the permanent loss of tokens if users inadvertently transfer to the zero address while the contract is paused. The impact is heightened by the expectation that all non-authorized transfers should be halted during a pause, which is not currently enforced for transfers to the zero address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_142_group

# Missing validation in __XERC20_init function of XERC20 contract

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-566
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/566
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-566.md

## Brief Summary

The `__XERC20_init` function in the `XERC20` contract is responsible for initializing the contract configuration, including setting the token name, symbol, and factory address. However, there is lack of validation for the `_factory` address parameter and the use of an unsafe function `_transferOwnership` instead of transferOwnership. As `_transferOwnership` is called instead of `transferOwnership`, it doesn't validate the address sent to it. So it becomes `__XERC20_init` responsibility to do the verification before sending the address. Because there is no function for changing or setting the `FACTORY` once it is set. If it is set to zero address or incorrect one then protocol can either go...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_51_group

# Potential Risk of Exceeding Maximum Limits in XERC20:_calculateNewCurrentLimit

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-576
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/576
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-576.md

## Brief Summary

The `XERC20:_calculateNewCurrentLimit` function is responsible for determining the new current limit `_newCurrentLimit` based on the updated maximum limit and current limit values. However, in the else condition if the calculation `_currentLimit + _difference` results in a value more than max limit? If the calculated new current limit exceeds the maximum limit defined by the contract, it may lead to inconsistencies in limit management, contract state, or token operations, resulting in protocol limits breaching. Allowing the current limit to exceed the maximum limit poses security risks, such as potential exploitation by attackers to bypass limit restrictions, manipulate token balances, or p...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# OperatorDelegator::stakeEth may be front-run by malicious operator to steal ETH

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-838
- **Submitter:** 0xblackskull
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/838
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-838.md

## Brief Summary

Delegated staking protocols may be exposed to a [known vulnerability](https://ethresear.ch/t/deposit-contract-exploit/6528), where a malicious operator front-runs a staker’s deposit call to the chain deposit contract and provides a different withdrawal credentials. The front-running vulnerability exposes the contract to the risk of unauthorized Ether transfers and manipulation of contract state by malicious actors. This could result in financial loss and undermine the integrity of the staking process.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_38_group

# `xRenzoBridge#sendPrice()` does not correctly query Connext's `xcall()`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-107
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/107
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-107.md

## Brief Summary

Protocol's core functionality is flawed when considering it's integration with Connext, cause when the `PRICE_FEED_SENDER` is calling `Connext#xcall()` it instead passes in the `relayerFee` as is if it's a native value instead of passing it as an argument to the function, which then causes the context in Connext to not know that the `xcall` with the `relayerFee` integration is actually the one being queried breaking the integration.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_74_group

# xRenzoDeposit::getBridgeFeeShare() doesn't calculate correct fee on amounts greater than 32ETH

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-100
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/100
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-100.md

## Brief Summary

`xRenzoDeposit::getBridgeFeeShare()` only calculates the bridge fee up to a deposit of `sweepBatchSize` which will be some value above `32 ETH`, this means that any large deposits will not be charged the full bridging fee, losing funds for the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_24_group

# XERC20Lockbox::withdraw() can withdraw ERC20 or Native, however dev comment state otherwise

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-223
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/223
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-223.md

## Brief Summary

`XERC20Lockbox::withdraw()` states that the function allows for the withdrawal of `ERC20` tokens, however the function also allows for the withdrawal of native currency if `IS_NATIVE` is `true`. Meaning the code has been incorrectly implemented, leading to incorrect behaviour of allowing natitve withdrawals.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_32_group

# OperatorDelegator::completeQueuedWithdrawal() checks wrong withdrawal length

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-24
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/24
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-24.md

## Brief Summary

`OperatorDelegator::completeQueuedWithdrawal()` checks the incorrect `withdrawal` array length. Currently it checks `withdrawal.strategies.length` but it should be checking `withdrawal.shares.length` as this is the array that is accessed within the function's loop. This can cause an out-of-bounds revert when accessing `queuedShares[address(tokens[i])] -= withdrawal.shares[i];` as it is possible for `withdrawal.strategies` and `withdrawal.shares` to be of different lengths.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_54_group

# XERC20::mint() is callable by anyone, however comments state otherwise

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-90
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/90
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-90.md

## Brief Summary

`XERC20::mint()` dev comments for the function state that the function >Can only be called by a bridge However `XERC20::mint()` has no access control, and when following to `XERC20::_mintWithCaller()` it can be seen that anyone with minting allowance can call `mint()` successfully. This deviates from the expected behaviour from the comments on the function. It can also be seen within the repo that `RestakeManager::deposit()` also calls this function for staking, which is not a bridge.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_117_group

# RenzoOracleL2::getMintRate() reverts if price is less than 1 ETH, which can happen due to market forces

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-96
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/96
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-96.md

## Brief Summary

`RenzoOracleL2::getMintRate()` retrieves the price of `ezETH` in `ETH`, however if the price of `ezETH` is less than `1 ETH` the function will revert. The price of `1 ezETH` can be less than `1 ETH` due to market forces. At the time of writing, the [chainlink ezeth-eth](https://data.chain.link/feeds/ethereum/mainnet/ezeth-eth) oracle states that `1 ezETH` is worth `0.9868 Ether`. Meaning in the current code, the function would revert incorrectly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_43_group

# MEV opportunity when refunding gas through `_refundGas()`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-792
- **Submitter:** BlockSails
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/792
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-792.md

## Brief Summary

The function `_refundGas` calculates the gas refund based on `tx.gasprice`, which can be manipulated by validators or in cooperation with a malicious user. A validator or a user in collaboration with a validator could set an arbitrarily high `gasprice` for their transaction, leading to an inflated gas refund from the contract's balance. This could drain a decent amount out of the contract's funds if the balance is sufficient.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Chainlink oracle lookup division using hardcoded value instead of `pricefeed.decimals` may cause incorrect token look-up

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-801
- **Submitter:** BlockSails
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/801
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-801.md

## Brief Summary

The `lookupTokenValue` function divides the price by a hard-coded scale-factor of `10e18` instead of getting the actual scale factor through Chainlink's `pricefeed.decimals` function. If any token is introduced that deviates from this scaling factor, it will cause incorrect price look-ups.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_55_group

# Did Not Approve To Zero First

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-760
- **Submitter:** FastChecker
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/760
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-760.md

## Brief Summary

A number of features within the protocol will not work if the approve function reverts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Lack of Proper Input Validation in recoverNative() and recoverERC20() can lead to lost of funds for users

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1020
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1020
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1020.md

## Brief Summary

In the contraxt xRenzoBridge.sol, the `recoverNative` and `recoverERC20` functions do not perform proper validation of the receiver address (`_to`). If `_to` is set to a zero address (`0x0`), the tokens or Ether will be lost because they are sent to an address from which recovery is impossible (burning funds) Impact: Lost of funds for users

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_63_group

# Usage of depreciated safeApprove() function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1030
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1030
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1030.md

## Brief Summary

In the contract xRenzoDeposit.sol, the function _trade() and sweep() are using OpenZeppelin’s safeApprove() which has been documented as (1) Deprecated because of approve-like race condition and (2) To be used only for initial setting of allowance (current allowance == 0) or resetting to 0 because it reverts otherwise.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_88_group

# Inadequate Decimals Check

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1032
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1032
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1032.md

## Brief Summary

The contract assumes that all tokens interfaced with have 18 decimal places. However, it does not enforce this expectation thoroughly. While the contract does check that the tokens involved in the initialization process (`initialize` function) comply with this assumption, it doesn't enforce this check elsewhere (e.g., when dealing with tokens received in other functions). Although this assumption might be considered reasonable within the Ethereum ecosystem since most tokens use 18 decimal places, there are execptions such as USDT that only have 6. If a token with mismatched decimal places is used with this contract, unexpected behavior such as incorrect token balances and transactions could...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_80_group

# Potential overflow issue in RenzoOracleL2.sol::getMintRate()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1041
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1041
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1041.md

## Brief Summary

The function `getMintRate` reads the price from an oracle and scales it to a value with 18 decimal places. This operation to adjust the decimals involves a multiplication of the price with `10 ** (18 - oracle.decimals())`, which could cause numerical overflow if not properly safeguarded. An overflow occurs when an operation tries to create a number that is outside the maximum limit that can be held by the data type. In Solidity, a `uint256` has a maximum limit of 2^256 - 1, and any calculation that exceeds this value will result in an overflow and the value will loop around to zero.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Unrestricted Ownership Transfer

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1051
- **Submitter:** JC
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1051
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1051.md

## Brief Summary

The contract `OptimismMintableXERC20Factory` is the factory that deploys a new `OptimismMintableXERC20` token. However, it contains a dangerous and unrestricted function that allows the caller to transfer the ownership of the newly created contract. In this function Ownership of the newly created `OptimismMintableXERC20` token contract is transferred to the caller of the `_deployOptimismMintableXERC20` function, which can be any account. If an attacker can control the owner of the token contract, they can manipulate its behavior to their advantage and potentially do malicious activities such as pulling all the tokens to their account, regulating transactions, and other harmful actions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_84_group

# DepositQueue.sol:: Lack of Input Sanitization in Stake Function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1056
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1056
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1056.md

## Brief Summary

The contract DepositQueue.sol lacks proper input sanitization in the function `stakeEthFromQueueMulti`, where arrays of input values are used. The contract trusts the caller to provide appropriate inputs, which can lead to unintended behavior if incorrect data is supplied. In the function `stakeEthFromQueueMulti`, the contract processes multiple calls to `stakeEthInOperatorDelegator` by iterating through the arrays of `operatorDelegators`, `pubkeys`, `signatures`, and `depositDataRoots` and passing each index's values to the stake function. However, it contains no validation to confirm whether the supplied data is appropriate, leading to potential risks if the data is incorrect or malicious.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# DepositQueue.sol:: Arbitrary Spending of Tokens

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1057
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1057
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1057.md

## Brief Summary

The contract DepositQueue.sol does not sufficiently validate the `_asset` address in the `fillERC20withdrawBuffer` function leading to a potential vulnerability where any troublesome token (e.g a token having re-entrancy in its `transferFrom` function) could be spent arbitrarily.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# Use address.call() instead of address.send() to avoid denial of service

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-675
- **Submitter:** Kaysoft
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/675
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-675.md

## Brief Summary

Denial of service for receipients with fallback/receive functions that consumes more than 2300 gas.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_53_group

# Centralization risk in `DepositQueue`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-826
- **Submitter:** MaslarovK
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/826
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-826.md

## Brief Summary

The `DepositQueue` allows the trusted role to set the fee up to 100%, potentially increasing the centralization risk Centralization risk should be a big concern and prevented at all costs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_119_group

# Failure to initialize after disabling the initializer

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1027
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1027
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1027.md

## Brief Summary

In the optimism contract, they were planning to use the initialize imported contract but they didn't

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# Deposit and DepositETH needs some validations for protection from slippage attack

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-424
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/424
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-424.md

## Brief Summary

When making swaps, some validation needs to be checked to protect the contract from being sandwiched or prone to slippage attacks. Every deposit in both deposit and depositETH made will be prone to slippage attacks because of no check for that.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_90_group

# Initialization functions can be front-run

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-952
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/952
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-952.md

## Brief Summary

Several implementation contracts have initialize functions that can be front-run, allowing an attacker to incorrectly initialize the contracts. If the front-running of one of these functions is not detected immediately, an attacker may be able to steal funds at a later time. Attacker Eve has studied the next version of the renzo protocol and identified several parameters of initialization functions that, if set to certain values, will allow her to steal funds from the protocol. She sets up a script to automatically watch the mempool and front-run the initialize functions of the next renzo protocol deployment. Bob, a developer, deploys the next version of the 88mph protocol. Eve’s script fro...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_116_group

# `RestakeManager::depositETH()` always assumes 1:1 peg with ETH

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-589
- **Submitter:** NentoR
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/589
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-589.md

## Brief Summary

`RestakeManager::deposit()` and `RestakeManager::depositETH()` calculate amounts to be minted differently. The first one uses the price of the deposited asset whereas the second one the amount of ether sent. This can lead to incorrect accounting when the price of `ezETH` is not aligned with `ETH`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_101_group

# Ambiguous abi encoding in OptimismMintableXERC20 factory leading to salt collision and deployment DOS for same user deployment

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-800
- **Submitter:** ReadyPlayer2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/800
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-800.md

## Brief Summary

Ambiguous abi encoding of the salt in the _deployOptimismMintableXERC20 function leads to a hash calculation collision, which in turn leads to a collision in create3 contract deployment for the same user. This will be a major problem for users that wish to use the factory to deploy multiple tokens that could most likely have ambiguous names and symbols as a hash collision will also mean contract deployment collision.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_223_group

# TimelockController's `_minDelay` can be set to an arbitrarily high value, potentially rendering the timelock unusable.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-105
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/105
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-105.md

## Brief Summary

If the `_minDelay` is set to a very high value, it could make the timelock unusable, as any new operation would require an extremely long delay before it can be executed. This could lead to a situation where the controlled contract becomes stuck and unable to perform important administrative actions, such as updating critical parameters or addresses.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# TimelockController could be used to allow anyone execute proposals

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-547
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/547
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-547.md

## Brief Summary

The `TimelockController` contract allows the zero address (`address(0)`) to be granted the `EXECUTOR_ROLE`. If this occurs, either intentionally or by mistake, any address can execute operations without explicitly having the `EXECUTOR_ROLE` assigned to them. This could lead to unauthorized execution of sensitive operations, potentially compromising the security of the contract and any dependent systems.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# CCIP router cannot be updated

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-220
- **Submitter:** RootKit0xCE
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/220
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-220.md

## Brief Summary

CCIP Router addresses cannot be updated in [Receiver](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Bridge/L2/PriceFeed/CCIPReceiver.sol#L14) On contracts that inherit from CCIPReceiver, router addresses need to be updateable. Chainlink may update the router addresses as they did before. This issue introduces a single point of failure that is outside of the protocol's control. [an example from Chainlink](https://github.com/smartcontractkit/ccip-tic-tac-toe/blob/main/contracts/TTTDemo.sol#L81-L83) this example is created by Chainlink [CCIP Tic Tac Toa Example](https://docs.chain.link/ccip/examples#ccip-tic-tac-toe) [Chainlink documents noticing users about router address up...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_150_group

# WithdrawQueue's lack of expiration can be used to force the withdrawal more from EigenLayer

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-226
- **Submitter:** SBSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/226
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-226.md

## Brief Summary

Withdraw requests lack expiration and can stay forever if the user doesn’t call `claim()`, causing the withdraw buffer to be lower which forces withdraws from EigenLayer.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_92_group

# setFeeConfig allows setting feeAddress to zero address when feeBasisPoints is 0

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-326
- **Submitter:** Sabit
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/326
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-326.md

## Brief Summary

setFeeConfig allows setting feeAddress to zero address when feeBasisPoints is 0

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_208_group

# [H-06] `xRenzoDeposit::deposit` - Lack of Replay Attack Protection could lead to Draining of Funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-179
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/179
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-179.md

## Brief Summary

The `deposit` function in `xRenzoDeposit.sol` does not have any mechanisms in place to prevent the replay of user signatures across different chains. This vulnerability could allow an attacker to steal funds by replaying a victim's signature on a different chain. This vulnerability allows an attacker to directly steal user funds by replaying the victim's failed transactions. The financial impact can be significant, as an attacker can potentially steal large amounts of user funds by exploiting this vulnerability across multiple victims.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_94_group

# [H-10] `xRenzoBridge::recoverERC20` allows admins to Rug-Pull an Arbitrary Amount of ERC20 tokens from Users, leaving the protocol insolvent

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-183
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/183
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-183.md

## Brief Summary

The `recoverERC20` function in `xRenzoBridge` provides admins with the ability to rugpull as much ERC20 tokens as they want. If they decide to rug-pull, they can, leaving the protocol totally insolvent.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_164_group

# [M-5] `OperatorDelegator::queueWithdrawals` - Unbounded Loop can lead to Denial of Service (reuploaded)

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-273
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/273
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-273.md

## Brief Summary

(I am reuploading this vulnerability because I accidently withdrew the previous one when I went to update it.) The `queueWithdrawals` function in the `OperatorDelegator` contract contains an unbounded loop that iterates through the entire array of tokens and token amounts. If the array is large, it can consume a significant amount of gas, which can lead to a Denial-of-Service (DoS) condition. An Admin can provide a large array of tokens and/or token amounts when calling the `queueWithdrawals` function, causing the transaction to consume an excessive amount of gas. This can result in the transaction exceeding the block gas limit and reverting, making the contract unusable or blocking other t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_01_group

# [H-14] `ConnextReceiver::xReceive` is vulnerable to cross-chain replay attacks, allowing attackers to steal funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-282
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/282
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-282.md

## Brief Summary

the `xReceive` function in the `ConnextReceiver.sol` contract does not have any mechanism to validate the origin of the Connext message, leaving it vulnerable to replay attacks across different chains. An attacker could intercept a valid Connext message on one chain and replay it on a different chain, resulting in unauthorized actions, such as updating the price feed for the attackers benefit or triggering other sensitive operations. This could lead to the loss of user funds and compromise the integrity of the cross-chain functionality.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_151_group

# [M-17] `xRenzoDeposit::sweep()`: Ignoring Return Values can lead to loss of user funds.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-318
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/318
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-318.md

## Brief Summary

The `xRenzoDeposit::sweep()` function ignores the return value of the `connext.xcall()` call, which sends a cross-chain message to the destination chain. Ignoring the return value of the `connext.xcall()` call means that the function will continue executing even if the message sending fails. This could lead to the following issues: Inconsistent Protocol State: If the cross-chain message fails to be delivered, the protocol's state may become inconsistent, as the `xRenzoDeposit` contract would have processed the sweep operation locally, but the corresponding action may not have been executed on the destination chain. Loss of User Funds: If the cross-chain message fails, the user's deposited f...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_132_group

# [M-24] `withdrawQueue::withdraw` - Lack of Slippage Protection creates an opportunity for attackers to Sandwich Attack users, leading to financial losses for the users.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-542
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/542
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-542.md

## Brief Summary

The `withdraw` function in the `withdrawQueue` contract allows users to request a withdrawal of their ezETH tokens. However, the function lacks any form of slippage protection, exposing users to potential sandwich attacks. An attacker can monitor the mempool for pending withdraw transactions, execute their own transactions before and after the victim's transaction, and manipulate the redemption amount in their favor. The absence of slippage protection in the `withdraw` function can lead to users receiving less tokens than expected during the withdrawal process. Attackers can exploit this vulnerability to extract value from users' withdrawals by strategically placing their own transactions b...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_113_group

# [M-23] `RenzoOracle::lookupTokenValue` - Lack of validation for rollup sequencer leading to stale prices and indirect fund risk

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-567
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/567
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-567.md

## Brief Summary

The `lookupTokenValue` function in the `RenzoOracle` contract does not include any explicit validation to check if the rollup sequencer is running. If the rollup sequencer goes offline, it could lead to stale prices being used by the protocol, potentially resulting in mispricing of assets and indirect risk to funds. If the rollup sequencer is offline and stale prices are used, it can lead to mispricing of assets and indirect risk to funds. The impact of this vulnerability can manifest in several ways: Mispricing of assets: If the protocol relies on stale prices, it may lead to incorrect valuation of assets, causing users to make suboptimal trading or investment decisions. Indirect financial...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_112_group

# The Invariant That a Token's TVL in the Protocol Be Within the Set Limit Can Be Broken by an Admin Action

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-236
- **Submitter:** Tendency
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/236
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-236.md

## Brief Summary

[DepositQueue::SweepERC20](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Deposits/DepositQueue.sol#L254-L277) function is to be called by an admin to sweep stuck tokens in the `DepositQueue` to the set token's Eigen layer strategy manager. Here is the call path: ` RestakeManager::depositTokenRewardsFromProtocol --> OperatorDelegator::deposit --> StrategyManager::depositIntoStrategy ` The problem here is that, the system currently uses a limit system that intends to limit the total value locked, and each collateral token's total value locked to an admin set value. + To Illustrate: If for example the tvl limit for stETH has been set to 500...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_115_group

# Fee-loss is incurred during `xezETH` minting

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-103
- **Submitter:** Tigerfrake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/103
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-103.md

## Brief Summary

`Fee-loss` arises from the incorrect handling of the `bridgeFee` deduction in the `_deposit()` function. Impact The protocol gets no `bridgeFee` at all as the whole `_amountIn` is traded for `nextWETH` for the user. In other words, the user doesn't pay any `bridgeFee` for their deposited tokens.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_41_group

# RewardHandler::forwardRewards() is not payable

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-99
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/99
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-99.md

## Brief Summary

`RewardHandler::forwardRewards()` lacks `payable` modifier. If `value` is not zero, it could always revert. Impact `RewardHandler::forwardRewards()` cannot work if `value != 0`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_118_group

# Potential Front-Running Vulnerability in Lockbox Deployment

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-383
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/383
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-383.md

## Brief Summary

The predictability of the _salt used in the _deployLockbox function could potentially expose the XERC20Factory contract to front-running attacks. In this attack, a malicious actor could anticipate the outcome of the function call, pre-compute the address of the lockbox, and interact with it before the legitimate transaction is confirmed. This could lead to unauthorized access or manipulation of the deployed lockbox contract, compromising the integrity and security of the ptotocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_195_group

# in RenzoOracleL2 , wrong check will suscept the contract to wrong calculations

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-966
- **Submitter:** WildSniper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/966
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-966.md

## Brief Summary

in RenzoOracleL2 , wrong check will suscept the contract to wrong calculations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_140_group

# `burn` function in OptimismMintableXERC20 contract should not burn token from `_from` but from `msg.sender`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-190
- **Submitter:** ZanyBonzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/190
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-190.md

## Brief Summary

The `burn` function in OptimismMintableXERC20.sol burns from `_from` and not `msg.sender` which causes that malicious users can burn any tokens in the contract or from other users that have any unspent allowance in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# RestakeManager - calculateTVLs() returns the empty `operatorDelegatorTokenTVLs` array

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-892
- **Submitter:** ak1
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/892
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-892.md

## Brief Summary

Since the `calculateTVLs()` returns the empty `operatorDelegatorTokenTVLs`, it affects the RestakeManager's deposit logic. Especially, the following check would be bypassed since the value is zero always. [RestakeManager.sol#L528-L530](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/RestakeManager.sol#L528-L530)

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_222_group

# LockboxAdapterBlast : bridgeTo could be used to drain the contract balance by sending the large _extraData

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-925
- **Submitter:** ak1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/925
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-925.md

## Brief Summary

The balance of [LockboxAdapterBlast](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Bridge/Connext/integration/LockboxAdapterBlast.sol#L56-L95) could be drained by specifying large extra data.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_85_group

# Griefing attack against the `depositIntoStrategy` function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-723
- **Submitter:** alphacipher
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/723
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-723.md

## Brief Summary

The griefing attack delays the staking process, potentially causing the node to miss out on rewards or incur additional transaction costs. It also disrupts the smooth operation of the staking system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# RewardHandler's ETH balance is not accounted in TVL, enabling sandwich attacks on pending rewards

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-168
- **Submitter:** aslanbek
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/168
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-168.md

## Brief Summary

`RestakeManager#calculateTVLs` is used to retrieve total value of assets in the system, denominated in ETH. However, its logic does not include RewardHandler's balance, making the ezETH price smaller than it should be (until rewards are forwarded), and enabling sandwich attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_216_group

# Inconsistent use of upgradable versions in oppenzeppelin Imports

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-141
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/141
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-141.md

## Brief Summary

The `XERC20Lockbox` contract imports non-upgradeable contracts from `openzeppelin`, which may limit the contract's upgradability and compatibility with upgradeable systems. While the contract itself inherits from `Initializable`, ensuring proper initialization, the non-upgradeable imports could hinder seamless upgrades and integration with other upgradeable contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_188_group

# Missing Import of Initializable in Upgradeable Contract

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-142
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/142
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-142.md

## Brief Summary

The `OperatorDelegator` contract upgradeability is hindered due to the absence of the `Initializable` contract import. This prevents the proper use of the initializer function, which is essential for initializing upgradeable contracts. Without proper initialization, the contract may exhibit unexpected behavior during deployment or upgrade, potentially leading to security vulnerabilities and instability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_237_group

# Timelock Controller does not add all supported interfaces in supportsInterface()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-412
- **Submitter:** b0g0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/412
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-412.md

## Brief Summary

Timelock Controller contract implements both the `onERC721Received` & `onERC1155Received` methods to handle safeTransfer calls to it. However the `supportsInterface()` function looks like this: Only the `IERC1155Receiver` interface is defined, while the `IERC721Receiver` is missing. This breaks the contract composability and prevents other contract calling the function from verifying that the contract supports the interface for receiving ERC721 tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_214_group

# Storage collision can brick token minting on Optimism

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-574
- **Submitter:** b0g0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/574
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-574.md

## Brief Summary

`OptimismMintableXERC20` storage variables will be overridden in case its parent contract `XERC20` is upgraded. Vulnerability details In order to better handle tokens bridging to different L2s, the protocol employs [the XERC20 standard, designed to make the process easier and more reliable](https://hackmd.io/@arjunbhuptani/xerc20-bridge-spec). [The `XERC20.sol` contract](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Bridge/xERC20/contracts/XERC20.sol#L16) has been modified to use the upgradeability pattern of OpenZeppelin. Additionally a separate version has been created to be used on Optimism L2, called `OptimismMintableXERC20`. It inhe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_158_group

# "_deposit" will not work for all tokens in "XERC20Lockbox"

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-414
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/414
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-414.md

## Brief Summary

Detailed description of the impact of this finding. here we are not tracking the native token in XERC20Lockbox.sol. in deposit function when our token is "NATIVE" then we do not know whether we are depositing token or not.There are no way of tracking the NATIVE token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_73_group

# In WithdrawQueue, an address could not claim its assets if it got blacklisted during the cooldown period.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-154
- **Submitter:** blutorque
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/154
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-154.md

## Brief Summary

Renzo does support a blacklist token, e.g., wBETH. If a user is added to the blacklist during the cooldown period, they will not be able to claim wBETH, as it is transferred back to the same blacklisted address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# `RenzoOracleL2.getMinRate()`: Lack of sequencer check

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-847
- **Submitter:** carlitox477
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/847
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-847.md

## Brief Summary

Chainlink recommends that all Optimistic L2 oracles consult the Sequencer Uptime Feed to ensure that the sequencer is live before trusting the data returned by the oracle. This check is not implemented in `RenzoOracleL2.getMinRate()` When utilizing Chainlink in L2 chains like Arbitrum, it's important to ensure that the prices provided are not falsely perceived as fresh, even when the sequencer is down. Impact Use of stale price in case of sequencer down in L2 chains like arbitrum

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# Return values of `approve()` not checked

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-288
- **Submitter:** codeslide
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/288
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-288.md

## Brief Summary

Not all IERC20 implementations `revert()` when there is a failure in `approve()`. The function signature has a `boolean` return value and the function indicates an error by returning `false` instead of reverting. By not checking the return value, operations that should have failed may potentially go through without actually approving anything.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_152_group

# `xRenzoDeposit::deposit` calculation for converting `nextWETH` to `xezETH` is incorrect

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-276
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/276
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-276.md

## Brief Summary

`xRenzoDeposit::deposit` allows users to deposit tokens in exchange for `xezETH`. Due to an error regarding conversion from `nextWETH` to `xezETH` during deposit, users will be minted an incorrect amount of `xezETH`, likely much less than actually owed.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# `WithdrawQueue::claim` can be sandwich attacked

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-418
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/418
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-418.md

## Brief Summary

`WithdrawQueue::claim` allows users to claim their withdrawal request after `coolDownPeriod` has passed. The user's `ezETH` is burned and they are sent `collateralToken` asset. An attacker can front-run this transaction and call `RestakeManager::deposit` to mint them `ezETH` for collateral. When the user's `claim` is executed, the attacker can call `WithdrawQueue::withdraw` to burn their `ezETH` for the collateral, effectively sandwich attacking the call and profiting.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_79_group

# Users can control the price of ezETH through donation attack

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-431
- **Submitter:** cu5t0mpeo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/431
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-431.md

## Brief Summary

The price of ezETH is susceptible to manipulation, which can result in user losses or prevent deposits to L2.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# When the feeBasisPoints value is 10000, the sweepERC20 function cannot be called.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-435
- **Submitter:** cu5t0mpeo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/435
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-435.md

## Brief Summary

The sweepERC20 function will not run properly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_98_group

# Missing Access control to destination chain, which may cause lost of funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-366
- **Submitter:** eeshenggoh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/366
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-366.md

## Brief Summary

The `sendPrice` & `xReceive` functions is used to send the price feed and take all collateral and deposit it into Renzo to the L1. Both functions however are NOT protected by sending funds to wrong destination whitelisted chain. Impact Funds will be lost when calling the function with unauthorized destination chain

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Potential for draining the contract's funds by repeatedly calling the `_execute` function with different addresses.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-891
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/891
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-891.md

## Brief Summary

[target.call{value: value}(data)](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/TimelockController.sol#L369) line is sending value amount of Ether to the address specified by the target variable, which can be any address. This is considered a security risk because an attacker could potentially drain the contract's funds by calling this function repeatedly with different addresses. Vulnerability Details In the line [(bool success, ) = target.call{ value: value }(data);](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/TimelockController.sol#L369). This line sends value amount of Ether to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# RewardHandler's receive function reduces MEV yield

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-783
- **Submitter:** guhu95
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/783
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-783.md

## Brief Summary

MEV bribes (direct `block.coinbase` payments) that execute [the `RewardHandler`'s `receive`](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Rewards/RewardHandler.sol#L52-L54) function [call the `DepositQueue`](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Rewards/RewardHandler.sol#L12-L13), and in its [`receive` function](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Deposits/DepositQueue.sol#L158-L183), it calls `feeAddress`, and [`WithdrawQueue`'s `getBufferDeficit` and `fillEthWithdrawBuffer`](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Deposits/DepositQueue.sol#L296-L302), and [increments `totalEarned`](https://git...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_65_group

# Missing Initialization of ReentrancyGuardUpgradeable In WIthdrawQueue Would Brick All The Reentrancy Protection And Break The Code Consistency

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-974
- **Submitter:** ihtishamsudo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/974
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-974.md

## Brief Summary

[WithdrawQueue](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L11) contract inherits [PausableUpgradeable](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L13) & [ReentrancyGuardUpgradeable](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L14) upgradable contracts and it's invoking only [PausableUpgradeable](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L13) initializer in its...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_233_group

# Collateral Token not included in List for Minting ezETH can be used for staking in the Operator Delegator

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1019
- **Submitter:** inzinko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1019
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1019.md

## Brief Summary

When The Protocol receives rewards for staking it is sent to the `DepositQueue`, and it is used to stake back in the operator delegator, but the problem here is that the rewards sent to the contract are different ERC20 tokens that may have strategies on the eigen layer, but may not have being added to the collateral token list in the `RestakeManager`, which means Temporary DOS for any process that involves those tokens on the `RestakeManager`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_169_group

# Lack of Input Validation in sendPrice Function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-304
- **Submitter:** kaveyjoe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/304
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-304.md

## Brief Summary

The sendPrice function takes two parameters: _destinationParam and _connextDestinationParam. These are arrays containing the details of the destination chains and the addresses of the corresponding receivers on those chains. The function does not validate the contents of these arrays before processing them, which means that any data passed into the function is used as-is. Impact If the arrays contain malformed or malicious data, this could result in failed transactions, incorrect exchange rates being sent, or exploitation of the contract's functionality. This could undermine the integrity of the price feed and potentially lead to financial losses or reputational damage.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_131_group

# Malicious proxyAdmin can upgrade xerc20 token implementation and steal fund from user.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-770
- **Submitter:** ladboy233
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/770
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-770.md

## Brief Summary

Malicious proxyAdmin can upgrade xerc20 token implementation and steal fund from user.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_174_group

# Withdrawal request is not handled correctly upon claiming

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-945
- **Submitter:** m_Rassska
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/945
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-945.md

## Brief Summary

* In order to keep track of the user's withdrawal requests, the mapping is used, where the key is a user, and the value - `withdrawRequestIndex`, which itself holds a request. After claiming the request, it should be removed from the mapping to avoid a double claim. Currently, it's done in the following way: * However, the system assumes that the `withdrawRequests[msg.sender][withdrawRequests[msg.sender].length - 1]` will retrieve the last requested withdrawal, which is not true. In fact, it might be an empty slot, since the `withdrawRequestIndex` is not tied to a specific user. Impact * There is a possibility to accidentally remove two separate requests by only claiming one of them.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# OperatorDelegator._refundGas() does not refund to the admin i.e. `onlyNativeEthRestakeAdmin`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-703
- **Submitter:** mussucal
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/703
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-703.md

## Brief Summary

All protocol ETH rewards go back to `depositQueue` to be restaked without paying back the gas costs to admin.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_03_group

# Failed ERC20 transfer inside `claim()` results in permanent loss of funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-54
- **Submitter:** t0x1c
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/54
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-54.md

## Brief Summary

Although this issue has been mentioned in the [automated findings](https://github.com/code-423n4/2024-04-renzo/blob/main/4naly3er-report.md#m-9-return-values-of-transfertransferfrom-not-checked), I believe it warrants a clear mention here due to the impact being loss of funds with no way for the user to retry the transaction. <br> The `claim()` function [uses the transfer()](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Withdraw/WithdrawQueue.sol#L305) function from the [OZ IERC20 interface](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/IERC20.sol#L34-L41) but never checks it's return value to see if it executed successfully or no...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_91_group

# It is not possible to completely remove an operator delegator from restaking manager without changing ezETH exchange rate

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-137
- **Submitter:** tapir
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/137
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-137.md

## Brief Summary

Completely removing an operator delegator can be impossible because if the completed withdrawal amount is bigger than the deficit then the excess will be redeposited which in case of a migration or off boarding an operator delegator this behaviour would not be correct. The excess being redeposited to the operator delegator makes the restaking manager admin removing the operator not possible. If the admin does that regardless, the excess TVL will also be scraped hence, the TVL will change and exchange rate will drop significantly.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Access control implemented in WithdrawQueue's `fillEthWithdrawBuffer method is Inconsistent with Natspec

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-161
- **Submitter:** umarkhatab_465
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/161
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-161.md

## Brief Summary

The DocString/Natspec of the method `fillEthWithdrawBuffer` states that it is access controlled by the Restake manager - only restake manager is able to call it with access control imposed by `` but the modifier used is `onlyDepositQueue` which ensures only the deposit Queue contract will be able to call this method . But this check given in Natspec is not really enforced

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_67_group

# Missing token vaules in withdrawqueue when checking `collateralTokenTvlLimits`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-292
- **Submitter:** zhaojohnson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/292
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-292.md

## Brief Summary

Collateral tokens' value might exceed the collateral's limit `collateralTokenTvlLimits`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_78_group

# Improper share price calculation in calculateRedeemAmount()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-320
- **Submitter:** zhaojohnson
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/320
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-320.md

## Brief Summary

Depositors may earn more or less profit than expected. Even some profits will be locked in contract and nobody can claim them.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Possible claim() failure because of out of gas.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-321
- **Submitter:** zhaojohnson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/321
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-321.md

## Brief Summary

Users may claim failure because of out of gas and users' funds are locked in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# Deposit fees are taken two times in `xRenzoDeposit` even without trade

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-322
- **Submitter:** zigtur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/322
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-322.md

## Brief Summary

Fees are taken twice from users, bypassing the 1% limit for fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_177_group
