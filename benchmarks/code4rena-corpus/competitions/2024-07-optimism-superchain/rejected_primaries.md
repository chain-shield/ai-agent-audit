# Rejected Primary Findings: Optimism Superchain

# Attacker can reverse chess clock by attacking a defense

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-73
- **Submitter:** Bac0nj
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/73
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-73.md

## Brief Summary

Attack and Defense for a claim both use the same clock. Attacker can take advantage of this to reverse chess clock. Impact Malicious attacker can gain more time while shorten the others', make the game resolved incorrectly. Vulnerability Detail 1. Here's the chain of attack: `0(root claimer) <- 1(attacker) <- 2(root claimer) <- 3(attacker) <- 4(root claimer)`, 2. When the time to challenge 2 is about to end, honest player Bob come to support root by creating a defense 5 to 2. 3. Attacker then attack Bob's defense 5, thus reverse the chess clock.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Create a game with big MAX_GAME_DEPTH could cost too much to win

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-74
- **Submitter:** Bac0nj
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/74
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-74.md

## Brief Summary

If a game is created with big MAX_GAME_DEPTH and attacker keep attacking, it would cost too much for a honest player to win the game. Vulnerability Detail For a honest party to win a game, player should keep attacking malicious players when they make a claim. If the MAX_GAME_DEPTH is too big, player should put lots of bonds to get to the final step.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Current method of hashing the leaves make it vulnerable to the second preimage attack

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-54
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/54
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-54.md

## Brief Summary

Preimage proposals are susceptible to the second preimage attacks

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `FaultDisputeGame` claims could be unresolvable in some edge cases

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-62
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/62
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-62.md

## Brief Summary

An unwanted state would be reached where we would have an unresolvable claim.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# The `getChallengerDuration` function lacks handling the case when the parent indexer does not exist

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-66
- **Submitter:** DanielTan_MetaTrust
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/66
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-66.md

## Brief Summary

The vulnerability exists in the `getChallengerDuration` function, which calculates the remaining time for a challenger to respond to a claim based on the parent’s clock. The specific issue arises when the parent indexer does not exist, and the function uses the default value of `type(uint32).max`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Immutable ABSOLUTE_PRESTATE forces frequent redeployments and allows proving invalid withdrawals

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-109
- **Submitter:** Dup1337
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/109
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-109.md

## Brief Summary

Immutable ABSOLUTE_PRESTATE forces freqent redeployments and allows proving invalid withdrawals in case of block number since `ABSOLUTE_PRESTATE` is bigger than `1 << SPLIT_INDEX`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Redundancy

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-58
- **Submitter:** Kavin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/58
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-58.md

## Brief Summary

There is redundancy in the client initialization process where the same URL is used to initialize multiple types of clients (ethclient).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted Access to resolveClaim Function Allows Anyone to Modify Status Value

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-25
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/25
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-25.md

## Brief Summary

* The `resolveClaim()` function initially checks the status. If it is not in the `IN_PROGRESS` stage, the function call will revert. However, the status value can be changed by anyone simply by calling the `resolve()` function, as there are no access control modifiers or user requirement checks. This lack of control can affect the contract's function process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# L1 re-orgs could cause `Position` to be different, resulting in `move()` being called on a different node

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-42
- **Submitter:** RI_trollers
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/42
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-42.md

## Brief Summary

`FaultDisputeGame.sol`'s `move()` requires participants to provide the index(in `claimData`) of the parent claim. After that, it checks that `Claim _disputed` provided by the user is equals to the `parent.claim` which was introduced after another re-org related bug was found in the previous optimism sherlock audit. However, <ins>the function fails to check that the `Position` of the node remains the same</ins>, which makes it susceptible to a different re-org attack. Proof of code Let's consider an oridinary binary tree. <ins>**Sequence Walkthrough:**<ins> It can be assumed in the code that `Alice` is the honest challenger while `Miner` is the malicious actor. 1. `Alice` disagrees with the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Bit Manipulation in `setCountered` Function

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-121
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/121
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-121.md

## Brief Summary

The `setCountered` function in the `CannonTypes.sol` does not correctly clear the relevant bits before setting the countered flag. The other setter functions (`setTimestamp`, `setPartOffset`, etc.) follow a consistent pattern of first clearing the target bits and then setting the new value. This pattern ensures that the new value is correctly applied without residual bits from previous values. The `setCountered` function should follow the same pattern to maintain consistency and correctness. It also employs a mask designed for a 64-bit segment instead of targeting the intended single bit. The operation attempts to extract the entire 64-bit segment, instead of isolating the 255th bit, which...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Loop Condition in `findlatestgames`

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-122
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/122
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-122.md

## Brief Summary

The vulnerability in the `findLatestGames` function stems from an incorrect loop condition that prevents the function from iterating over the intended range of indices in the `_disputeGameList`. Specifically, the loop is designed to search backward from the index `_start` down to 0, but the condition `i >= 0 && i <= _start` ensures that the loop either does not execute at all (if `_start` is not 0) or executes just once (if `_start` is 0) The loop condition consists of two parts combined with a logical AND operator (`&&`): - `i >= 0`: This condition checks if the current index `i` is greater than or equal to 0. Since `i` is initialized to `_start`, and `_start` is presumably a positive inte...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A defense can be made against the root claim of the execution trace bisection subgames

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-123
- **Submitter:** Udsen
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/123
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-123.md

## Brief Summary

The `FaultDisputeGame.move` function is used as the generic move function for both `attack` and `defend` moves. In the function execution there is following `Invariant` which is secured. INVARIANT: A defense can never be made against the root claim of either the output root game or any of the execution trace bisection subgames. The implementation of the above `Invariant` is given as follows: As per the `invariant` it is stated that the `defense` can never be made against the root claim of the execution trace bisection subgames. The root claim of the execution trace subgames is the `SPLIT_DEPTH + 1` which is one level below the `SPLIT_DEPTH`, but in the above logic implementation of the inva...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Re-Entrancy issue in claimCredit Function

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-114
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/114
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-114.md

## Brief Summary

The claimCredit function is to allow users to claim their credited balance from the contract. The function is correctly sets the user's credit balance to 0 before making an external call to the WETH contract to withdraw the corresponding amount. the attacker can re-enter the claimCredit function during the withdrawal process and claim additional credit before the initial call completes cause there is a miss of a re-entrancy guard the function. the function is sets the credit balance to 0 before making the external call, it does not prevent re-entrancy during the WETH.withdraw call. Impact the issue can lead to loss of funds , and it's allowing an attacker to drain the entire balance of the...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# clone can be manupilated

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-129
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/129
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-129.md

## Brief Summary

Detailed description of the impact of this finding. clone can be manupliated as we are taking all the input from the user.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Attacker can challenge a game with correct root and win by spoofing one of the local preimages

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-119
- **Submitter:** niroh
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/119
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-119.md

## Brief Summary

When the game reaches MAX_DEPTH the last claim in the game can be challanged with a step. If it's an honest challanger's turn in the game, they submit a step transaction with state data that matches the prestate commitment. If the state data indicates that the step requires a local preimage, they need to also call addLocalData before they call Step, so that the data is available for the Mips Step function. There is however a scenario where a malicious user can cause a game with a valid root to fail (while gaining all bonds) using a loophole in the preimage system:

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Inverted Logic in resolveClaim Function May Cause Unresolved Disputes and Financial Losses

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-115
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/115
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-115.md

## Brief Summary

The line in question is intended to prevent the resolution of a subgame that has already been resolved. However, the logic appears inverted, which could lead to incorrect behavior. Potential Issues Inverted Logic: The condition checks if the subgame is not resolved This should instead check if the subgame is resolved. The revert message ``ClaimAlreadyResolved()`` is misleading because the condition checks for unresolved subgames. Impact of this bug can prevent the correct resolution of subgames, leading to unresolved disputes and potential loss of funds or incorrect game outcomes.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# The bisection root should be at SPLIT_DEPTH + 1, not SPLIT_DEPTH - 1

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-59
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/59
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-59.md

## Brief Summary

The bisection root should be at SPLIT_DEPTH + 1, not SPLIT_DEPTH - 1

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# A Fault Dispute Game can be played solo.

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-61
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/61
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-61.md

## Brief Summary

The purpose of a back-and-forth is not established.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A claimant with a valid claim will lose their bond and will be considered if there is a valid defence but no attack against their claim

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-116
- **Submitter:** silver_eth
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/116
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-116.md

## Brief Summary

A valid claim could be considered invalid

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_21_group

# Unchecked External Calls

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-86
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/86
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-86.md

## Brief Summary

**Unchecked External Call in `claimCredit`**: - The external call to `WETH.withdraw` in the `claimCredit` function is not checked for success. **Unchecked External Call**: - The contract makes an unchecked external call to `ANCHOR_STATE_REGISTRY.tryUpdateAnchorState()` which may fail silently. - **Mitigation**: Ensure the success of the external call using proper error handling mechanisms. Suggested Improvements **External Call Validation**:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Blockhash Function Limitation

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-88
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/88
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-88.md

## Brief Summary

The `blockhash` function only provides access to block hashes of the most recent 256 blocks. **Impact:** If the dispute game creation relies on a block hash older than 256 blocks, the `blockhash` function will return zero, leading to potential failures or incorrect initializations. **Solution:** Add a check to ensure that the parent block hash is not zero.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Constructor Misuse

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-89
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/89
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-89.md

## Brief Summary

The constructor calls the `initialize` function with `address(0)`, potentially leaving the contract in an uninitialized state. **Impact:** If the contract is deployed without proper initialization, it may be vulnerable to unauthorized access or other unintended behaviors. **Solution:** Separate the constructor logic from the initialization logic, ensuring the constructor disables initializers and proper initialization is handled separately.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# Incorrect Use of Assembly for Array Management

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-90
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/90
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-90.md

## Brief Summary

**Incorrect Use of Assembly for Array Management** **Issue:** The use of inline assembly for array manipulation in `findLatestGames` is complex and prone to errors. **Impact:** Improper use of assembly can lead to unexpected behavior, memory corruption, or inefficient gas usage. **Solution:** Use native Solidity constructs to handle dynamic arrays, making the code more readable and safer.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-91
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/91
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-91.md

## Brief Summary

Functions like `setImplementation` and `setInitBond` do not validate the provided `_gameType` and `_impl`. **Impact:** If invalid inputs are provided, it could lead to incorrect state updates and unintended behavior. **Solution:** Add checks to validate the inputs before performing state updates.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# `load**` functions don't check key exists and don't have access control

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-46
- **Submitter:** twcctop
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/46
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-46.md

## Brief Summary

anyone can call `load**` functions and modify the data, the fucntion `readPreimage` may get wrong data.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# the proof data of function `squeezeLPP` can be used to call `challengeLPP`

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-50
- **Submitter:** twcctop
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/50
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-50.md

## Brief Summary

when try to finalize prososal, malicious user is possible to front run get proof data and call `challengeLPP` to get profit

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group
