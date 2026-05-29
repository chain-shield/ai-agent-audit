# Rejected Primary Findings: Munchables

# Use call instead of transfer

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-282
- **Submitter:** 0xleadwizard
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/282
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-282.md

## Brief Summary

Using call instead of transfer is better and less error-prone as there is gas limit of 2300 with transfer and no-check for success.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# PriceFeed can continually launch the wrong proposals, preventing correct proposals from being launched

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-49
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/49
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-49.md

## Brief Summary

Token prices may be maliciously delayed, resulting in erroneous prices.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_22_group

# Setting `DISAPPROVE_THRESHOLD` to 0 results in the inability to pass proposals even when `APPROVE_THRESHOLD` is reached.

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-163
- **Submitter:** Eeyore
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/163
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-163.md

## Brief Summary

In the `setUSDThresholds()` function, an Admin can update the `APPROVE_THRESHOLD` and `DISAPPROVE_THRESHOLD` storage values that determine the thresholds after which a proposal is approved or disapproved. Unfortunately, setting the `DISAPPROVE_THRESHOLD` to 0 has the effect that none of the proposals will pass, leading to a temporary denial of service (DoS) situation if all current PriceFeed roles vote in favor of the proposal by calling the `approveUSDPrice()` function. There is a check in the `_execUSDPriceUpdate()` internal function: This condition will always be false when `DISAPPROVE_THRESHOLD` is 0, preventing any proposal from being passed no matter the number of approval votes. The...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_24_group

# Inaccurate Calculation of Weighted Value When Lock Duration Expires but Tokens Remain Unlocked

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-126
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/126
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-126.md

## Brief Summary

The current implementation inaccurately calculates the weighted value of locked tokens when the lock duration has expired but the user has not yet unlocked them. This discrepancy can lead to misleading representations of the user's locked token holdings, affecting decision-making processes based on the reported values. the `getLockedWeightedValue` function checks to ensure that the player quantity is > 0 as below the issue with that is before unlocking the tokens users who locked have a quantity > 0 even if the lock duration is passed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_18_group

# Lack of Quantity Check in lock and lockOnBehalf Functions.

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-575
- **Submitter:** aua_oo7
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/575
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-575.md

## Brief Summary

Description:In the `_lock` function which is call by `lock` and `lockOnBehalf`, there's no explicit check to ensure that the `_quantity` parameter passed to the function is greater than zero all `lock` , `lockOnBehalf` and `_lock` doesn't check it . This means that if `_quantity` is mistakenly set to zero, it would still execute the lock operation without actually locking any tokens. If an attacker were to exploit this vulnerability by calling `lockOnBehalf` or `lock` with a `_quantity` of zero, they could effectively bypass the intended token locking mechanism. This could lead to loss of funds if users rely on this function to securely lock their tokens. **Mitigation Steps:** Add a check a...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_44_group

# Token Owner tokens will be lost if the owner uses wrong OnBehalf Address

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-614
- **Submitter:** d4r3d3v1l
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/614
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-614.md

## Brief Summary

Loss of tokens for the Owner who called the `lockOnBehalf` function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_16_group

# Incomplete mapping deletion in the disapproveUSDPrice function

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-33
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/33
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-33.md

## Brief Summary

Detailed description of the impact of this finding. Location: The function `LockManager.disapproveUSDPrice(uint256)` in `src/managers/LockManager.sol` (lines 210-242) deletes the `ILockManager.USDUpdateProposal` structure in `src/interfaces/ILockManager.sol` (lines 60-69), which contains a mapping: - Deletion occurs at `src/managers/LockManager.sol` line 238. Description: Deleting a structure that contains a mapping does not remove the mapping itself, as per the [Solidity documentation](https://solidity.readthedocs.io/en/latest/types.html##delete). Residual data from the mapping may pose a security risk and could potentially be exploited to compromise the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_41_group

# Use of payable.transfer() Instead of .call for sending ether

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-225
- **Submitter:** kartik_giri_47538
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/225
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-225.md

## Brief Summary

The contract uses payable.transfer() to send Ether, which has a hard gas limit of 2300 gas. This can cause the transaction to fail if the recipient's fallback or receive function requires more than 2300 gas, or if there is a change in gas costs in the Ethereum network. **Impact:** If the recipient has a complex fallback or receive function that requires more than 2300 gas than tx can revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_28_group

# Locking native tokens using a non-EOA account makes funds unrecoverable

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-527
- **Submitter:** niser93
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/527
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-527.md

## Brief Summary

The [lock() method](https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L296-L309) allows to lock funds and mint Munchables NFT. The [lockOnBehalf() method](https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L274-L294) allows to lock funds on behalf of a player. If a player uses a smart contract without the `receive/fallback` methods instead of an EOA to lock his/her native tokens or decides to lock on behalf of a smart contract address, instead of an EOA, he/she will not be able to unlock his/her funds after the unlockTime period. This means that his/her funds will be stuck indefinitely inside the LockManager contract...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary, :robot:_01_group

# setLockDuration doesn't have unlockTimeLatest controls

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-425
- **Submitter:** tonisives
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/425
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-425.md

## Brief Summary

User's tokens can are frozen for extended periods of time without their possible knowledge or control.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_53_group
