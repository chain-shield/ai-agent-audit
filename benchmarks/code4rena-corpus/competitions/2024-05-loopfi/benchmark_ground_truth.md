# Benchmark Ground Truth: LoopFi

## Accepted H/M Findings

# Accepted H/M Findings: LoopFi

# [H-01] Availability of deposit invariant can be bypassed

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-loopfi
- **Source snapshot:** competitions/2024-05-loopfi/final_report.html

Submitted by 0xnev, also found by Krace, 0xrex ( 1, 2 ), gumgumzum, novamanbg, Greed, y4y, DMoore, 0x04bytes, 0xBugSlayer, sldtyenj12, yovchev_yoan, 0xJoyBoy03 ( 1, 2 ), sandy, Evo, Kirkeelee, Sajjad, d3e4, samuraii77, Pechenite, TheFabled, Rhaydden ( 1, 2 ), web3er, ZanyBonzy, SBSecurity, nfmelendez, Topmark, XDZIBECX, Bigsam, shaflow2, 0xSecuri, petarP1998, bbl4de, _karanel, and btk

## Impact

One of the main invariants stated in the audit is the following:

Deposits are active up to the lpETH contract and lpETHVault contract are set However, there are currently two ways this invariant can be broken, allowing users to gain lpETH without explicitly locking tokens before contracts are set.

Sandwich a call to convertAllETH by front-running to directly donate ETH and then back-running to claim lpETH via multiple lock positions Bundle a transaction of donation and claiming with a previously locked position of wrapped LRT This bypasses the onlyBeforeDate(loopActivation) modifier included in all lock functions. It also potentially allows no cap in lpETH minted and also discourages users from locking ETH in the PrelaunchPoints.sol contract before contract addresses are set (i.e.

loopActivation is assigned).

## Recommended Mitigation Steps

For scenario 1, set claimedAmount to amount of ETH bought from LRT swap (such as buyAmount for uniV3).

For scenario 2, no fix is likely required since it would require users to risk their funds to be transferred by other users due to inflated totalLpETH: totalSupply ratio. If not, you can consider setting totalLpETH to totalSupply and allow admin to retrieve any additional ETH donated.

0xd4n1el (Loop) confirmed Koolex (judge) increased severity to High

## Rejected Primary Findings

# Rejected Primary Findings: LoopFi

# The ETH is forever locked i the contract after the `convertAllETH` function is called

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-268
- **Submitter:** 0xBugSlayer
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/268
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-268.md

## Brief Summary

The ETH is forever locked in the contract after the `convertAllETH` function is called

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_09_group

# Users may not withdraw their token when emergency mode is on

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-164
- **Submitter:** 0xJoyBoy03
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/164
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-164.md

## Brief Summary

When users attempt to withdraw tokens in emergency mode under specific conditions related to `block.timestamp`, a transaction may revert due to frontrunners altering the transaction order after the emergency mode is deactivated. This occurs because the `withdraw` function's conditions rely on the `block.timestamp` value, which frontrunners can manipulate during transaction sequencing. Impact This issue affects users attempting to withdraw tokens during an emergency mode period, potentially resulting in failed transactions when frontrunners manipulate transaction orders.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_14_group

# In one case Users can't lock any tokens

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-169
- **Submitter:** 0xJoyBoy03
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/169
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-169.md

## Brief Summary

The user wants to lock their token by calling any lock functions, which leads to the `_processLock` internal function. However, before their transaction is confirmed, the admin executes the `setLoopAddresses` function, changing the `loopActivation` state. This change allows for the potential reordering of transactions by frontrunners, resulting in the user's transaction being reverted. Impact Users miss their chance to lock their tokens due to the potential reordering of transactions by frontrunners, leading to the user's transaction being reverted.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_56_group

# User can withdraw without losing staking points !

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-78
- **Submitter:** 0xMosh
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/78
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-78.md

## Brief Summary

One of the major purpose of `PrelaunchPoints` is to account how many staking points will users get in terms of locking assets in the protocol .On locking , users get staking points and on withdrawal points are removed/reduced . This is acheived by tracking event emissions on the chain by a backend on locking and withdrawal . This is updated in every one hour . However, event emission on `withdraw` function can be skipped by reentering into the function . Thus a user can withdraw funds without losing his accumulated staking points .

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_05_group

# Reentrancy During Token Swap in `_claim` function

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-322
- **Submitter:** 0xbhumii
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/322
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-322.md

## Brief Summary

The `_claim` function calls `_fillQuote` to swap the claimed token for ETH using the provided `_swapCallData`. However, there's no reentrancy protection mechanism in place, leaving the contract vulnerable to a reentrant attack during the swap. A malicious attacker could exploit this by setting up a fallback function that calls `_claim` again after the initial token swap. This reentrant call could manipulate the contract's state before the `lpETH.deposit` (or `lpETH.safeTransfer` for ETH claims) completes, potentially allowing the attacker to: Claim more tokens than they are entitled to by manipulating the balance mapping. Disrupt the claim process for other users by interfering with state v...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Unchecked Zero-Amount Approval in _claim Function

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-323
- **Submitter:** 0xbhumii
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/323
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-323.md

## Brief Summary

In `_fillQuote`, the line `require(_sellToken.approve(exchangeProxy, _amount))` approves the exchangeProxy contract to transfer `_amount` of the `claimed token`. However, there's no check to ensure `_amount` is greater than `zero`. An attacker could exploit this by crafting a transaction that calls claim with a very small claimed token amount (e.g., 0.00000001 tokens). Even though this amount might not be enough for a successful swap, the contract would still attempt to approve the exchangeProxy to transfer it. Impact Potential loss of user tokens in the future: While the immediate impact might be negligible due to the tiny amount, the attacker's token remains approved for the exchangeProxy...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# The _claim function uses external data from 0x for swaps and needs to be validated or malicious users could exploit this to direct funds incorrectly.

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-21
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/21
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-21.md

## Brief Summary

The _claim function uses external data from 0x for swaps, if not properly validated or if there's a vulnerability in how 0x API data is handled (e.g., incorrect or manipulated outputToken), malicious users could exploit this to direct funds incorrectly. To enhance the security of the _claim function and protect it from potential exploitation via maliciously crafted 0x protocol calldata, the validation process needs to be more rigorous. Here are specific steps and code modifications to improve security: 1. Strict Validation of 0x API Data Enhance the _validateData function to include additional checks on the 0x API data, such as ensuring the swap data's integrity, correct output tokens, and...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# No deadline check for the UniswapV3 call can result in transactions being maliciously executed

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-209
- **Submitter:** Cryptor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/209
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-209.md

## Brief Summary

No deadline check for the UniswapV3 call can result in transactions being maliciously executed

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_67_group

# Attacker can overload the backend by sending too many events in _processLock

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-389
- **Submitter:** Evo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/389
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-389.md

## Brief Summary

The protocol's backend will be DoSed by flooding it with a huge amount of events coming from [emit events](https://github.com/code-423n4/2024-05-loop/blob/0dc8467ccff27230e7c0530b619524cc8401e22a/src/PrelaunchPoints.sol#L197) in _processLock method.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Users may suffer loss of funds if wrapped LRT tokens are removed from the token whitelist.

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-317
- **Submitter:** FastChecker
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/317
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-317.md

## Brief Summary

Users may suffer loss of funds if wrapped LRT tokens are removed from the token whitelist.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# Mismatch on event for swap and claimedAmount

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-118
- **Submitter:** John_Femi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/118
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-118.md

## Brief Summary

I am working under the premise that the backend actively monitors the events emitted by the smart contract, utilizing them for a variety of purposes as outlined in the documentation. A potential security concern arises when a malicious actor manipulates the events by introducing a discrepancy between the `Claimed` and `SwappedTokens` events. This manipulation can be achieved by sending a minimal amount of ETH to the contract moments before a user initiates a claim transaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Referral can be spammed

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-89
- **Submitter:** John_Femi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/89
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-89.md

## Brief Summary

Referral code in bytes should be taken with a high same level of security to avoid being spammed. The use of referral to gain extra points could allow a bunch of scenarios to play out.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_41_group

# WETH deposits can't be claimed or withdrawn if claim() or withdraw() is called with WETH address

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-378
- **Submitter:** Kirkeelee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/378
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-378.md

## Brief Summary

When users lock WETH, their deposits are stored as ETH in the function. If they try to claim their lpETH with the WETH address as an input, they will get the error "NothingToClaim". However, they can claim if they provide ETH as the token address. Latter can only be the case if the user knows the contract code or there is a public disclosure by the protocol. Also, users can't withdraw since will always be 0.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Missing slippage protection in `PrelaunchPoints.sol#claim()` function as it execute on chain sawapping

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-397
- **Submitter:** Pechenite
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/397
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-397.md

## Brief Summary

The `PrelaunchPoints` contract within the LoopFi protocol is designed to manage the claiming process where users can convert their locked tokens to another form or currency through on-chain swaps using external DeFi protocols such as Uniswap. The `claim()` function is responsible for this conversion and transfer of assets. However, there is a crucial oversight in the design—lack of slippage control—which exposes users to potential financial risks due to unfavorable rate movements during the transaction execution. The absence of slippage protection means that when users initiate a claim, the actual amount of tokens or assets received after the swap could significantly differ from expected va...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_12_group

# Lack of validation for referral address

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-301
- **Submitter:** Sabit
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/301
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-301.md

## Brief Summary

A user can intentionally provides their own address as the _referral parameter. This could lead to incorrect referral tracking and reward distribution.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# Unauthorized Activation Risk: Emergency Mode Access Control Flaw

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-75
- **Submitter:** Serpent0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/75
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-75.md

## Brief Summary

The presence of the setEmergencyMode function without adequate safeguards allows for the toggling of emergency mode by an authorized entity. The specific impact of this function being misused includes: Change in Contract Behavior: The contract may behave differently when in emergency mode, which could affect all users interacting with it. Alteration of Normal Operations: If emergency mode changes the contract’s operations, such as enabling otherwise restricted actions, it could disrupt normal contract functionality. Direct Influence on User Transactions: Depending on the contract’s logic, entering or exiting emergency mode could directly impact user transactions, potentially leading to unex...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# Incorrect Token Balance Handling Due to Unaccounted Transfer Fees

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-349
- **Submitter:** Tychai0s
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/349
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-349.md

## Brief Summary

The function does not account for possible transfer fees deducted by some ERC20 tokens, which can result in incorrect balance updates within the contract. This discrepancy can lead to financial inaccuracies, affecting the integrity of the contract's token management.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# There is no way to recover the ETH form this protocol

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-335
- **Submitter:** albertwh1te
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/335
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-335.md

## Brief Summary

In this protocol, the user can deposit the LRT token and eth. However, the owner can only rescue the erc20 token from protocol. There is no way to rescue the ETH from this protocol. There should be one more function for the owner to rescue the ETH from the protocol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# There is missing zero address check for at `setOwner`

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-337
- **Submitter:** albertwh1te
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/337
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-337.md

## Brief Summary

The absence of a zero address check in the setOwner function when the owner calls it can result in a critical scenario where user funds may become permanently locked within the protocol.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Users are unable to claim fractions of their ETH

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-340
- **Submitter:** albertwh1te
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/340
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-340.md

## Brief Summary

Users are unable to claim fractions of their ETH. When attempting to claim a percentage of their ETH holdings, the entire deposited ETH amount will be claimed, resulting in unexpected behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_62_group

# Potential Precision Issue in Token Claim Calculation

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-186
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/186
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-186.md

## Brief Summary

The current implementation of the token claim calculation in the contract may lead to precision issues, particularly for tokens with low decimal values. This could result in users receiving incorrect amounts of tokens when claiming.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_24_group

# Staking Functionality Flow

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-188
- **Submitter:** atoko
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/188
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-188.md

## Brief Summary

The `claimAndStake` function in the contract allows users to claim their vested lpETH and stake them in a Loop vault for extra rewards. However, there is a flow in the staking process where the staked tokens are incorrectly attributed to the user's address instead of the vault contract address.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_70_group

# zero token transfer can fail.

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-146
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/146
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-146.md

## Brief Summary

Detailed description of the impact of this finding. Some ERC20 tokens do not allow zero value transfers, reverting such attempts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# Users will unfairly lose all their points during an emergency withdraw

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-304
- **Submitter:** btk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/304
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-304.md

## Brief Summary

When the owner start an emergency withdraw, LRT's stakers will unfairly lose all their points.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# Claims of lpETH will not start immediately after conversion

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-366
- **Submitter:** btk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/366
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-366.md

## Brief Summary

Users will have to wait ~1 block after conversion to claim their lpETH breaking the intended behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Centralization risks in critical functionality

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-355
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/355
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-355.md

## Brief Summary

The PrelaunchPoints contract has several centralization risks due to the owner having too much control over key protocol parameters and functionality. This introduces risks that the owner could abuse their privileges in ways that negatively impact users. For instance: 1. `setOwner` - Allows the owner to set a new owner address. This could be abused to transfer ownership to a malicious address. 2. `setLoopAddresses` - Allows the owner to set the `lpETH` and `lpETHVault` addresses. If called with malicious contract addresses, this could allow the owner to steal user funds when they claim or stake lpETH. 3. `allowToken` - Allows the owner to add support for new tokens to be locked. If a malici...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# 0x API Dependancy creates a single point of failure

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-356
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/356
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-356.md

## Brief Summary

The PrelaunchPoints contract has a dependency on the 0x API for token swaps when users claim their lpETH rewards. This creates a single point of failure where if the 0x API is down, unavailable, or the response format changes, users will not be able to claim their rewards. This could lead to bad user experience and loss of trust in the protocol. Issue Description The `claim` and `claimAndStake` functions in PrelaunchPoints allow users to claim their lpETH rewards by swapping their locked tokens for ETH through the 0x API, and then depositing that ETH into the lpETH contract. These functions rely on the `_data` parameter which is expected to be calldata obtained from the 0x API representing...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_68_group

# Missing ending/auto-relock logic

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-117
- **Submitter:** ctmotox2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/117
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-117.md

## Brief Summary

Expired lockers can dilute the staking rewards. Ending/renewing lock logic is missing in withdraw function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_37_group

# `convertAllETH` can be DOS'ed

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-326
- **Submitter:** deliriusz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/326
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-326.md

## Brief Summary

`convertAllETH` can be DOS'ed

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# User can submit claims with percentage larger than 100 and steal funds

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-387
- **Submitter:** gesha17
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/387
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-387.md

## Brief Summary

A user can submit a call to claim with _percentage over 100, meaning he will receive more funds than his claim. The _percentage is uint8, meaning it can go up to 255. There is no check on the percentage being under 100.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# first to claim the token will receive all the balance of the contract

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-278
- **Submitter:** karsar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/278
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-278.md

## Brief Summary

In the ``_claim`` function the claimedAmount is set to ``address(this).balance ``instead of the ``userClaim`` which a user can get the whole balance of contract instead of the userStake. Impact loss of funds for the users

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# User can withdraw less funds or zero

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-260
- **Submitter:** kgothatso
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/260
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-260.md

## Brief Summary

when a user calls the `claim` function to get their `ETH` they deposits they can get less or nothing of their deposits. This breaks the invariant test and users can get DOS if they try to withdraw `ETH`.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_85_group

# Potential Security Vulnerabilities in Data Decoding Functions

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-250
- **Submitter:** mtimbol
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/250
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-250.md

## Brief Summary

The `_decodeUniswapV3Data` and `_decodeTransformERC20Data` functions are responsible for decoding data from the 0x API when either UniswapV3 and TransformERC20 is used. If the input data is not properly validated, it could lead to serious security vulnerabilities. Malicious actors could potentially manipulate the decoded values by crafting the input data in a specific way, which could lead to unexpected behavior in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# There isnt a way to mint WETH token because of `WETH.withdraw(amount)`

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-294
- **Submitter:** niki
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/294
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-294.md

## Brief Summary

Users cannot lock WETH tokens in the protocol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# PrelaunchPoints should have two step ownership transfer

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-227
- **Submitter:** nisedo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/227
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-227.md

## Brief Summary

The `owner` address carries numerous important abilities for the system: - `convertAllETH()` - `setLoopAddresses()` - `allowToken()` - `setEmergencyMode()` - `recoverERC20()` - `setOwner()` However, the `setOwner()` function allows the owner address to be errantly transferred to the wrong address as it does not use a two-step transfer process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Withdrawal Invariant is not correctly implemented, so withdrawal is available up to `startClaimDate` but not during 7 days after `loopActivation` is set.

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-246
- **Submitter:** petro_1912
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/246
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-246.md

## Brief Summary

Withdrawal invariant is not working as stated in `Main Invariant` section of the protocol document, it can be result in users' misunderstanding. - `Withdrawals are only active on emergency mode or during 7 days after loopActivation is set.` If owner delays executing `convertAllEth` due to some problems, users may think there is an error in the protocol and can withdraw their locked funds breaking the invariant.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# PrelaunchPoints contract will not work with LRT rebasing tokens

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-405
- **Submitter:** radev_sw
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/405
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-405.md

## Brief Summary

The PrelaunchPoints contract, part of the LoopFi protocol, is designed to handle various user interactions, including staking, claiming, and managing digital assets within a DeFi ecosystem. The contract assumes a standard behavior of ERC-20 tokens for these operations. However the contract does not accommodate the unique characteristics of rebasing tokens, such as those whose supply dynamically adjusts, causing individual balances to rebase up or down periodically without any transfer transactions. The lack of support for rebasing tokens can lead to several significant issues: - Incorrect Token Balances - Users holding rebasing tokens might see their balances in the PrelaunchPoints contract...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_83_group

# Lack Of Slippage Control When Swapping Through Uniswap/TransformERC20

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-291
- **Submitter:** sakshamguruji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/291
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-291.md

## Brief Summary

A user's claim amount might be way lesser than what it was supposed to be since there is no slippage protection while claiming.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# msg.sender of type address by default is not explicitly cast to address payable when expected to receive funds

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-344
- **Submitter:** turvy_fuzz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/344
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-344.md

## Brief Summary

msg.sender of type address by default is not explicitly cast to address payable when expected to receive funds, which could cause a revert, leading to unintended behavior and possibly loss of funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# User funds getting locked attack vector

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-106
- **Submitter:** umarkhatab_465
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/106
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-106.md

## Brief Summary

One of the main concerns of the protocol as stated in contest readme is user funds should not be locked for undesirable times However that is possible in the claim function when the total amount of tokens >>> total lp eth totalSupply > totalLpETH More concretely if the user has the balance x to withdraw then if x*totalLpEth < totalSupply then the user's tokens will be locked for a long time until a lot of other depositors start claiming their tokens through claim effectively reducing the totalSupply

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_27_group

# The protocol allows locking eth and tokens for zero address as a receiver

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-170
- **Submitter:** umarkhatab_465
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/170
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-170.md

## Brief Summary

lack of input validation on receiver parameter in `lockEthFor` and `lockFor` allows locking ETH and Tokens for zero address which will be stuck forever after the claim.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# Accidently sent tokens which are supported by the protocol will get stuck forever

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-233
- **Submitter:** umarkhatab_465
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/233
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-233.md

## Brief Summary

The protocol only allows the tokens to be recovered by the Authorized address if the token to be recovered is not one of these - lpETH Token - Supported/Allowed tokens i.e isAllowed[token] =true However, if such tokens are sent accidentally to the contract, they can never be recovered due to this check and lack of any other way to the tokens out.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_82_group

# DoS in claim and ClaimAndStake functions for next 82 years

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-97
- **Submitter:** umarkhatab_465
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/97
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-97.md

## Brief Summary

The `claim` and `claimAndStake` methods are broken , and will lead to denial of service for the next 82 years because the `startClaimDate` is set to `82 years` from now and these functions are only executed `after the startClaimDate has passed`

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Inconsistent Total Supply Calculation in Withdraw Function

- **Contest:** LoopFi
- **Slug:** 2024-05-loopfi
- **Submission:** V-94
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-loop-validation/issues/94
- **Source snapshot:** competitions/2024-05-loopfi/submissions/raw/V-94.md

## Brief Summary

The impact of this finding is a discrepancy between the totalSupply of the contract and the actual amount of WETH stored in the contract. When users lock WETH tokens, the totalSupply increases accordingly. However, when users withdraw their WETH tokens, the totalSupply is not decreased. This inconsistency could lead to incorrect calculations elsewhere in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_45_group
