# Rejected Primary Findings: eBTC Zap Router

# USER CAN INFLATE HIS COLLATERAL TO A HIGH VALUE TO STEAL PROTOCOL FUNDS

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-136
- **Submitter:** 0xweebad
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/136
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-136.md

## Brief Summary

1. In the call to `[openCdp()]` external function of the <EbtcLeverageZapRouter> contract, there's a collateral value based on the margin Amount that is transferred to the ZapRouterBase contract from the user. This represents the user's collateral put up to open the cdp... See #L181 of `EbtcLeverageZapRouter::openCdp` function. + The contract then calls the internal [_openCdp] function, with the fifth param as this margin collateral value put up 2. In the internal [_openCdp()] , the function does not do anything with the user's put up collateral.. + In fact, the put-up collateral isnot used anywhere within this function.. it's absolutely neglected.. + `# instead of setting the openCdpForOpe...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# ATTACKER CAN WEAPONISE WRONG VALIDATION IN requireSingularMarginChange() function TO CLEAR HIS DEBT WITHOUT REPAYMENT USING adjustCdp function AND HENCE INCREASE HIS ICR TO BE ABLE TO BORROW MORE

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-179
- **Submitter:** 0xweebad
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/179
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-179.md

## Brief Summary

There is a wrong validation in the `requireSingularMarginChange` function in the EbtcLeverageZapRouter contract. It only checks that either of the 2 parameters should be 0. However, it fails to check also that both of the 2 parameters cannot be 0, that is, one of the parameters must be 0, while the other is a non-zero. And due to its usage in `EbtcLeverageZapRouter::_adjustCdp` with respect to <marginIncrease> and <marginDecrease> calculation, Attacker can weaponize this vulnerability to clear his debt to the minimum debt (1000) without any repayment, and hence increase his position's ICR as a result. Vulnerability Details 1. In the external `[adjustCdp]` function, if AdjustCdpParams.stEthM...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Closing/Adjusting CDPs are done with a minute leakage in accounting and funds

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-21
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/21
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-21.md

## Brief Summary

look at https://github.com/code-423n4/2024-06-badger/blob/9173558ee1ac8a78a7ae0a39b97b50ff0dd9e0f8/ebtc-zap-router/src/ZapRouterBase.sol#L72-L105 Vulnerability details Impact As hinted under _Proof of Concept_, since we are sure of a 1-2 wei cut off from from the transfers, [the cut-off amount could be higher](https://docs.lido.fi/guides/lido-tokens-integration-guide/#1-2-wei-corner-case), then this leads to a break in protocol's accounting in regards to the amount of funds attached to a cdp position that gets closed, since not all the funds would be transferred to the user, i.e a minute loss of funds for users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Misinterpretation of Liquidator Rewards in Smart Contract Logic

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-97
- **Submitter:** BlockSafe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/97
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-97.md

## Brief Summary

Alice could potentially benefit from a liquidation event without having the required minimum net stETH balance.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Two user touchpoint routers not sharing a global reentrancy guard opens the door for cross-contract reentrancy attack

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-160
- **Submitter:** Chad0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/160
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-160.md

## Brief Summary

In the `EbtcLeverageZapRouter` contract, from the functions of `_openCdp()`, `_closeCdp()` and `_adjustCdp()` we can see they are using `nonReentrant`, this means the protocol is designed as not allowing reentrance into the methods above. However, this `nonReentrant` only works for its residing contract. Since this protocol has two separate user touchpoint router contracts (`EbtcLeverageZapRouter` and `EbtcZapRouter`), hence, a malicious user can still perform a reentrancy-ish operation which is supposed to be forbidden by the protocol. I'll demonstrate how it is done in the

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# The router may not be able to operate due to insufficient funds.

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-128
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/128
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-128.md

## Brief Summary

This will cause the router to become unusable.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# lack checks `_stETHReiceived`

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-11
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/11
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-11.md

## Brief Summary

it is possible that `_stETHBalBefore` will be greater than stEth.balanceOf(address(this)) after the _convertWstEthToStETH function is called, although under normal circumstances this should not happen. This could occur due to a decrease in the stETH balance of the contract during the unwrapping process, potentially due to external factors or contract interactions that are not accounted for within the scope of the `_convertWstEthToStETH()`itself. If there are other interactions with the stETH balance of the contract that occur concurrently (e.g., other functions interacting with the stETH balance, or state changes within the stETH or wstETH contracts that affect balances unexpectedly), it co...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `stETHMarginAmount` can be lower than the loan amount

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-186
- **Submitter:** John_Femi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/186
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-186.md

## Brief Summary

In the `openCdp` function, the `_stEthMarginAmount` is used to determine the collateral to use to open the position and the `_transferInitialStETHFromCaller` gets the collateral, and the `_openCdp` internal function is called, but as we see, there is no verification of margin collateral enough for opening position. This allows a user to put correct values to open a position on eBTCZapLeverageRouter except putting a smaller amount of marginLeverage collateral to open a larger sized position on his account than allowed. This can in turn cause bad debt and losses for the protocol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# exchangeData can be maliciously used for arbitraty calls, even when DEXes are hardcoded and trusted like 1inch and 0x

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-177
- **Submitter:** MrValioBg
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/177
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-177.md

## Brief Summary

We could leverage the ability that we could control the function signature & parameters that will be called on the 1inch or 0x DEXes to make arbitrary calls. In LeverageZapRouterBase we use set the `swaps[0].calldataForSwap `to the `_tradeData.exchangeData`, which we pass as a parameter when we call either openCDP/adjustCDP/closeCDP. DEX is hardcoded, but we can adjust exchangeData. Impact One very easy way to reproduce this behavior is on the closeCDP position to increase the debt/amount in exchangeData (Check

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_21_group

# Discrepancy Between Documentation and Implementation in closeCdp Function Regarding Collateral Return Type

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-31
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/31
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-31.md

## Brief Summary

The function `closeCdp` is documented as follows: [EbtcLeverageZapRouter.sol#L250-L251](https://github.com/code-423n4/2024-06-badger/blob/9173558ee1ac8a78a7ae0a39b97b50ff0dd9e0f8/ebtc-zap-router/src/EbtcLeverageZapRouter.sol#L250-L251) The documentation states that the original collateral (stETH) is returned to the CDP owner regardless of the asset type used to open or adjust the CDP. However, the implementation explicitly sets the `_useWstETH` parameter to false when calling the `_closeCdp` function: [EbtcLeverageZapRouter.sol#L260](https://github.com/code-423n4/2024-06-badger/blob/9173558ee1ac8a78a7ae0a39b97b50ff0dd9e0f8/ebtc-zap-router/src/EbtcLeverageZapRouter.sol#L260) > This setting i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Misleading Comment and Incorrect Logic in adjustCdpWithWrappedEth Function Handling WETH vs WstETH

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-32
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/32
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-32.md

## Brief Summary

The `adjustCdpWithWrappedEth` function has a misleading comment and incorrect logic. The function `adjustCdpWithWrappedEth` in the EbtcLeverageZapRouter.sol is documented to "increase collateral with wrapped Ether" and manage debt of a CDP. However, the function's implementation converts the `_params.stEthMarginBalance` to stETH using `_convertWrappedEthToStETH`, which suggests that it is handling wrapped stETH (WstETH) instead of wrapped Ether (WETH). This inconsistency can lead to confusion and errors, as the function name and documentation suggest that it should handle WETH, but the logic is set up for WstETH. This could result in incorrect asset handling and unexpected behavior when use...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Any Ether sent beyond the specified _ethMarginBalance is lost.

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-33
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/33
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-33.md

## Brief Summary

The `openCdpWithEth` is marked as `payable` and thus can accept Ether transactions. However, the function does not handle the received Ether (`msg.value`) effectively. It converts only the `_ethMarginBalance` parameter to stETH, but does not account for any additional Ether sent with the transaction that exceeds this parameter value. This oversight means that any excess Ether sent to this function (beyond the `_ethMarginBalance` specified) is not refunded or stored, leading to a loss of funds for the user. Impact Any Ether sent beyond the specified `_ethMarginBalance` is lost.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# doOperation Function Always Reverts, Rendering It Unusable in LeverageZapRouterBase.sol

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-34
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/34
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-34.md

## Brief Summary

The primary impact is that the function is effectively unusable. Since it reverts every time it is called, no operations can be performed using this function. This could be a significant limitation if the function is intended to handle important operations such as managing positions or executing trades. The `doOperation` function in the `LeverageZapRouterBase.sol` contract is marked as external, which suggests that it should be callable by other contracts or externally. However, the presence of the `revert("disabled");` statement within this function makes it unusable, as it will always revert any transaction that attempts to invoke it. [LeverageZapRouterBase.sol#L65-L74](https://github.com...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# Missing Success Check in _openCdpForCallback Leads to Potential Fee Transfer Without CDP Creation

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-36
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/36
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-36.md

## Brief Summary

`_openCdpForCallback` does not check the success of the `openCdpFor` operation before proceeding to transfer the fee. This can lead to scenarios where the fee is transferred even though the CDP was not successfully opened, potentially leading to loss of funds or incorrect fee allocation. [LeverageZapRouterBase.sol#L280-L288](https://github.com/code-423n4/2024-06-badger/blob/9173558ee1ac8a78a7ae0a39b97b50ff0dd9e0f8/ebtc-zap-router/src/LeverageZapRouterBase.sol#L280-L288) You can see, the function transfers the fee immediately after attempting to open a CDP without verifying that the operation was successful.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The conversion path from ETH to stETH can be suboptimal in `ZapRouterBase` and cause loss of funds when depositing

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-55
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/55
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-55.md

## Brief Summary

In most cases, stETH is cheaper than ETH. See Chainlink's Oracle here: https://data.chain.link/feeds/arbitrum/mainnet/steth-eth As at the time of this report, `STETH / ETH` Ξ `0.999940765` The conversion path using the `_depositRawEthIntoLido`, `_convertWrappedEthToStETH`, and `_convertRawEthToStETH` functions in ZapRouterBase can be suboptimal if the protocol could buy stETH directly from a DEX pool at a cheaper price. These functions always wrap ETH to stETH by depositing it directly into the Lido contract at a 1:1 ratio, potentially incurring a loss on each deposit if the market price of stETH is lower.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_15_group

# Fee calculation in `LeverageZapRouterBase` could lead to excessive fees being charged and inaccurate CDP debt

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-69
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/69
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-69.md

## Brief Summary

This can result in users paying higher fees than intended and having their CDPs reflect a higher debt than the actual eBTC received.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_09_group

# Bad Actor can Completely Sweep Off StEth in Contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-87
- **Submitter:** Topmark
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/87
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-87.md

## Brief Summary

Bad Actor can Completely Sweep Off StEth balance in Contract

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# an absence of a repayment verification

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-82
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/82
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-82.md

## Brief Summary

The onFlashLoan function is responsible for handling the callback from a flash loan. While it verifies the initiator and the caller, and it does not include a check to ensure that the total repayment borrowed amount plus fee is made before the function is completes and this missing in verification is allows the function to return success FLASH_LOAN_SUCCESS even if the loan is underpaid, creating a potential for financial loss. Impact the issue can result the loss of funds to the lender due to incomplete flash loan repayments.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Add Zero Address Check and Optional Event Emission in `_doSwapChecks`

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-29
- **Submitter:** Zaykov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/29
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-29.md

## Brief Summary

The `_doSwapChecks` function currently lacks a check for the zero address, which can lead to interactions with an invalid ERC20 token address. This could cause unexpected failures or potential security vulnerabilities. Additionally, the function could benefit from emitting events to improve traceability and debugging.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Vulnerability in `_doSwap` Function of `LeverageMacroBase` Contract Enables Unauthorized Token Transfers

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-30
- **Submitter:** Zaykov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/30
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-30.md

## Brief Summary

The `LeverageMacroBas`e contract's `_doSwap` function contains a critical vulnerability that allows arbitrary external addresses to be called due to inadequacies in the `_ensureNotSystem` function. This oversight could potentially enable malicious actors to transfer tokens out of the contract. Impact This vulnerability can lead to a loss of funds from the contract as malicious actors can exploit the arbitrary external call to transfer tokens out of the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Incorrect Parameter Order in Struct Initialization Leading to Financial Miscalculations

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-99
- **Submitter:** Zaykov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/99
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-99.md

## Brief Summary

Incorrect order of parameters when initializing the `AdjustCdpOperation` struct within the `_adjustCdpOperation` function. Specifically, the `_stEthBalanceDecrease` and `_stEthBalanceIncrease` parameters are swapped. This misassignment can have several critical impacts: 1. The incorrect assignment of parameters can lead to erroneous financial calculations related to collateral and debt changes. This could result in the improper adjustment of collateral or debt balances, leading to potential financial losses or imbalances in the system. 2. If the logic relying on these parameters expects specific values, the misassignment can cause operations to behave unpredictably. For example, a decrease...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential manipulation of CDP queue position through unvalidated upperHint and lowerHint parameters

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-52
- **Submitter:** Zims
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/52
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-52.md

## Brief Summary

The `adjustCdp` function in the `EbtcLeverageZapRouter` contract allows users to manipulate their CDP's position in the sorted list through the `upperHint` and `lowerHint` parameters without proper validation. This could lead to unfair advantages in liquidation order or other priority-based operations, potentially impacting the fairness and intended functionality of the protocol. Specifically: 1. Users could potentially position their CDPs more favorably in the queue, affecting liquidation order. 2. This manipulation could result in some users gaining unfair advantages in terms of liquidation risk or redemption priority. 3. The system's intended order of CDPs could be disrupted, leading to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# [M-1] If any eBTC or stETH are accidentally locked the first user to openCDP will gain all of them

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-134
- **Submitter:** agadzhalov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/134
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-134.md

## Brief Summary

By design there should not be any locked tokens in LeverageZapRouter (`EbtcLeverageZapRouter`) contract. But there's no guarantee for this because as we know anyone can directly send these tokens to the contract and lock them, in such case the first user to open CDP position when executing `_sweepEbtc` or `_sweepStEth` will gain ALL of the tokens not just the ones intended for him. Also this approach is in contrast with protocol's design before transferring any tokens to check "balanceBefore" like here https://github.com/code-423n4/2024-06-badger/blob/main/ebtc-zap-router/src/EbtcLeverageZapRouter.sol#L453 (just one example) Impact User can gain more eBTC and/or stETH tokens than intended.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# [H-1] User can open CDP position passing **unallowed** tokens in `TradeData.exchangeData`

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-167
- **Submitter:** agadzhalov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/167
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-167.md

## Brief Summary

There's no validation for tokens and selectors that are passed in `TradeData.exchangeData` this means that any user who opens or adjusts CDP positions can pass different tokens for swapping at the end of the transaction. Details By design when opening or adjusting CDP positions the protocol must execute swap using DEX (1inch or 0x). The allowed swaps must be either `eBTC -> stETH` when increasing debt or `stETH -> eBTC` when decreasing debt. Only `eBTC` and `stETH` must be allowed. But in the `Mock1Inch.sol` which is used for mocking DEX like 1inch, the `swap` method is adapted for the needs of the protocol. Meaning that if you try to swap tokens different than eBTC and stETH it will revert...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# Use call instead of transfer

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-137
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/137
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-137.md

## Brief Summary

Detailed description of the impact of this finding. In _transferStEthToCaller transfer() is used for native ETH withdrawal. The transfer() and send() functions forward a fixed amount of 2300 gas. Historically, it has often been recommended to use these functions for value transfers to guard against reentrancy attacks. However, the gas cost of EVM instructions may change significantly during hard forks which may break already deployed contract systems that make fixed assumptions about gas costs. For example. EIP 1884 broke several existing smart contracts due to a cost increase of the SLOAD instruction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unlocked Pragma

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-130
- **Submitter:** black-wolf
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/130
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-130.md

## Brief Summary

Location: ebtc-zap-router/src/EbtcLeverageZapRouter.sol L:2 ebtc-zap-router/src/interface/IEbtcLeverageZapRouter.sol L:2 Impact Deployment with Old Compiler Versions: There is a risk of deploying the contract with an older compiler version, which might have known and unresolved bugs. Testing Inconsistency: The contract might be tested with a specific compiler version, but deployment with a different version can lead to unexpected behaviors and inconsistencies. Security Vulnerabilities: Bugs in older compiler versions can introduce security vulnerabilities into the contract, compromising its integrity and safety.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ZapRouterBase._permitPositionManagerApproval() will bypass an expired permit.

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-54
- **Submitter:** chaduke
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/54
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-54.md

## Brief Summary

Detailed description of the impact of this finding. will bypass an expired permit. The try-catch clause for help prevent the permit front-running DOS attack, however, the catch body is empty. As a result, will bypass an expired permit.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_05_group

# Lack of Slippage Protections for Asset Conversions in ZapRouterBase

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-188
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/188
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-188.md

## Brief Summary

When performing token swaps or conversions, it's crucial to implement slippage protection to guard against unexpected price movements and front-running attacks. This is typically done by specifying a minimum amount of tokens to receive from the conversion. Without such protection, users may receive significantly fewer tokens than expected if the market price shifts unfavorably during the transaction's execution. Vulnerability Details In the ZapRouterBase contract, there are several functions that convert between different forms of ETH-based collateral (ETH, WETH, stETH, wstETH). However, these conversion functions lack checks for minimum received amounts, which could lead to potential losse...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Fixed Hash Value in LeverageMacroBase contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-117
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/117
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-117.md

## Brief Summary

Detailed description of the impact of this finding. Vulnerable line 37 of code The constant FLASH_LOAN_SUCCESS is defined using a fixed string and the keccak256 hashing function. The value of this constant can be predicted by anyone who knows the string "ERC3156FlashBorrower.onFlashLoan". While this pattern is common and often used to create unique identifiers, the security implications depend on how this constant is utilised in the contract. If the contract relies on FLASH_LOAN_SUCCESS being secret or unpredictable, an attacker could exploit this predictability. For example, if this constant is used to authorise specific actions or validate critical transactions, an attacker could use the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Parameter Handling in _closeCdpOperation function within LeverageZapRouterBase contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-25
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/25
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-25.md

## Brief Summary

Detailed description of the impact of this finding. Improper CDP closure can lead to financial loss and incorrect state.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Incorrect Data Decoding in decodeFLData function within LeverageMacroBase contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-28
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/28
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-28.md

## Brief Summary

Detailed description of the impact of this finding. The decodeFLData function decodes flash loan data but does not handle potential decoding errors, which can lead to incorrect or unexpected data processing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Excessive Approval Amount in LeverageZapRouterBase Contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-71
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/71
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-71.md

## Brief Summary

Detailed description of the impact of this finding. The constructor of the LeverageZapRouterBase contract sets the approval amount to the maximum value (uint256.max) for several tokens. This is done using the IERC20/ERC20 interface, which grants unlimited approval to the specified addresses. This practice is risky as it could allow the approved addresses to withdraw an unlimited amount of tokens, potentially leading to significant financial losses if those addresses are compromised or malicious. By approving the maximum amount (uint256.max), the contract exposes itself to potential abuse, where the approved entity can withdraw an unlimited amount of tokens from the contract. This could resu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unprotected Ether Withdrawal Found (transfer/send) in the ZapRouterBase contract affecting the EbtcLeverageZapRouter contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-81
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/81
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-81.md

## Brief Summary

Detailed description of the impact of this finding. Unprotected Ether withdrawal due to missing or insufficient access controls can allow malicious parties to withdraw Ether from the contract account. This bug may be caused by unintentionally exposing initialization functions or by incorrectly naming a constructor function, causing the constructor code to be included in the runtime bytecode and callable by anyone to re-initialize the contract. Unauthorized withdrawals can lead to significant financial loss and compromise the integrity and security of the smart contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_53_group

# Weak sources of randomness from chain attributes in LeverageMacroBase contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-84
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/84
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-84.md

## Brief Summary

Detailed description of the impact of this finding. The ability to generate random numbers is crucial for applications such as gambling DApps, where pseudo-random number generators are used to pick winners. However, using block attributes like block.number for randomness is insecure because miners can influence these attributes. A miner could manipulate these values to their advantage, especially in high-stakes scenarios, leading to unfair outcomes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Block values used as a proxy for time found in LeverageMacroBase contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-85
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/85
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-85.md

## Brief Summary

Detailed description of the impact of this finding. The use of block.number as a proxy for time can lead to unpredictable and insecure behaviour in smart contracts. Block numbers and timestamps are not precise and can be manipulated to some extent by miners. This makes them unsuitable for time-sensitive operations. Instead, using a reliable oracle to fetch time values ensures greater accuracy and security.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# Lack of Explanation on Emitting Events for Revert Reasons on the EbtcLeverageZapRouter contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-9
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/9
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-9.md

## Brief Summary

Detailed description of the impact of this finding. The functions in the EbtcLeverageZapRouter contract are currently emitting revert messages without sufficient explanation. This can make debugging and error tracing difficult for developers and users. Specifically, the functions adjustCdp, adjustCdpWithWrappedEth, and adjustCdpWithWstEth are reverting without providing detailed context about the reasons for the failure.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Not real closing cdp

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-127
- **Submitter:** djanerch
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/127
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-127.md

## Brief Summary

The `_closeCdpOperation` function is intended to close CDP (Collateralized Debt Position) positions. However, it does not correctly handle the close operation within the `_handleOperation` function.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# User can send eth to router

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-42
- **Submitter:** djanerch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/42
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-42.md

## Brief Summary

The `receive` function is designed to block ETH transfers from users except for the wrappedETH address. However, there is a way for users to send ETH with the current implementation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Owner can't change `zapFeeBPS` and `zapFeeReceiver`

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-44
- **Submitter:** djanerch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/44
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-44.md

## Brief Summary

The `LeverageZapRouterBase` contract currently does not provide the owner with the ability to modify the `zapFeeBPS` and `zapFeeReceiver` parameters. This inflexibility poses a significant risk to the contract's usability and adaptability. If there is a need to update these parameters in response to changing market conditions or organizational changes, the contract will become inoperative, leading to potential financial and operational issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Fixed liquidation reward will cause bad debt

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-72
- **Submitter:** djanerch
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/72
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-72.md

## Brief Summary

A fixed liquidation reward system can lead to larger debts going unliquidated. For instance, no one would liquidate a 20 ETH debt for a 0.2 ETH reward when they can liquidate a 4 ETH debt for the same reward. Also most liquidators are bots which calculate profit percentages and will avoid liquidating a 20 ETH debt for a 0.2 ETH reward, which yields only a 1% profit.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential for token dust accumulation violates contract invariant

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-61
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/61
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-61.md

## Brief Summary

[sweepToCaller()](https://github.com/code-423n4/2024-06-badger/blob/9173558ee1ac8a78a7ae0a39b97b50ff0dd9e0f8/ebtc-protocol/packages/contracts/contracts/LeverageMacroBase.sol#L247-L263) function designed to transfer all eBTC and collateral tokens to the caller after operations. However, due to potential rounding errors/dust amounts, small quantities of tokens may remain in the contract. This violates the stated _invariant that the ZapRouter contract should not hold any tokens after each operation_, aside from rounding. The `sweepToCaller()` function uses `balanceOf()` and `sharesOf()` to determine the amounts to transfer: While this approach attempts to transfer all tokens, it doesn't accoun...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# `doOperation` Marked External But Always Reverts

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-12
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/12
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-12.md

## Brief Summary

`doOperation` function is marked as `external`, but it always reverts with the message "disabled". This means that the function is effectively unusable and cannot be called by any external contract or user. The comment above the `revert` statement suggests that the intention was to prevent the owner from doing arbitrary calls. However, by reverting unconditionally, it prevents anyone from using the `doOperation` function, not just the owner.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# `excessivelySafeCall` will not work with protocol that returns err-code instead of reverting

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-133
- **Submitter:** jesjupyter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/133
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-133.md

## Brief Summary

The function `excessivelySafeCall` is used to call external contract and get if the transaction is successful or not. However, in the `_doSwap` call, the the `_maxCopy` is set as `0`. This means that the returned value will all be threw away. The check `success` actually doesn't work with protocol (like Balancer V2) that returns `ERROR-CODE` or `False` instead of reverting.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# A user could sandwich the oracle update by opening and closing `cdp` to gain profits

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-154
- **Submitter:** jesjupyter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/154
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-154.md

## Brief Summary

The system appears to function as a leveraged trading mechanism for STETH/EBTC, specifically designed for long positions. This is achieved through the use of Lending and Flashloan functionalities. Additionally, the system utilizes an oracle to track the BTC/stETH price, which plays a crucial role in its operations. Under certain market conditions, such as high volatility, a user could sandwich the oracle's action to update price by strategically opening and closing collateralized debt positions (CDPs) if they find it profitable. This could potentially lead to significant financial gains for the user at the expense of the system's integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Mistakenly-sent stETH/eBTC could be used by the next user to openCdp/closeCdp

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-155
- **Submitter:** jesjupyter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/155
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-155.md

## Brief Summary

If a user mistakenly sends `stETH/eBTC` to the contract, these funds could be utilized by the next user during the `openCdp`/`closeCdp` process. This might involve using the funds for swapping or flashloan payback, potentially compromising the intended operation and fairness of the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# Unsafe downcasting in _getOwnerAddress function can be exploited to cause DoS

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-175
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/175
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-175.md

## Brief Summary

The `_getOwnerAddress` function within the `ZapRouterBase` contract contains an unsafe downcast that can lead to unexpected behaviour and potential security vulnerabilities. Unexpected errors or malfunctions if the address is used for authorization checks. Denial-of-Service (DoS) attacks if the contract gets stuck due to invalid addresses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Sunnyboy95 - Security Enhancements and Best Practices Implementation for `EbtcLeverageZapRouter` Contract

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-88
- **Submitter:** sunnyboy95
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/88
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-88.md

## Brief Summary

Impact of Security Issues in EbtcLeverageZapRouter Contract ## Failing to address the security issues and best practices in the `EbtcLeverageZapRouter` contract can lead to significant financial and operational consequences. Potential impacts include: * Financial Loss: Vulnerabilities such as reentrancy attacks, unchecked external calls, integer overflow/underflow, improper input validation, and flash loan attacks can be exploited by malicious actors, leading to substantial financial losses for both users and the contract itself. * Operational Disruption: Exploits and attacks can cause significant downtime and operational disruptions, affecting the reliability and functionality of the contr...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Protocol supports stETH but doesn't consider its unique transfer logic which would lead to not only a DOS of the depositing/withdrawal channel for this collateral token but also a flaw in multiple other core protocol logic

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-161
- **Submitter:** unRekt
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/161
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-161.md

## Brief Summary

eBTCZap__ Protocol supports `stETH` but doesn't consider its unique transfer logic. As per the comment we can pass any arbitrary token to this function. If the token used is `stEth` in the above function, then we should consider that `stEth` is a special token as per lido's official docs, we can see that there is a special section that talks about it's unique concept, i.e the "1-2 wei corner case" here is the [link]( https://docs.lido.fi/guides/lido-tokens-integration-guide/#1-2-wei-corner-case). `transferShares` is used in few functions of the contract, but the other functions doesn't use `transferShares` (shared below) which can lead to a vulnerability. The probability of issue appearing...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `try/catch` doesn't catch every error which may lead to silent failure

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-163
- **Submitter:** unRekt
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/163
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-163.md

## Brief Summary

In the `_permitPositionManagerApproval` function they are using try n catch but try n catch but not matching the results properly. If the try block is successful but it returns something wrong it wont get caught. Also from the comment of the function we can understand that catch block is important to prevent from frontrunning attacks, but the catch block is empty. Related references:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The leverage router doesn't support users to pay eBTC debt back with their own assets

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Submission:** V-43
- **Submitter:** y4y
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-badger-validation/issues/43
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/submissions/raw/V-43.md

## Brief Summary

Users who opens a position with leverage router with some self-provided collaterals may not be able to fully get his collaterals back when closing a position.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_58_group
