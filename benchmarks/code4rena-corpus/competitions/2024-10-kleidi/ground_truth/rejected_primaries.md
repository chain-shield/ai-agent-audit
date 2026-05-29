# Rejected Primary Findings: Kleidi

# maximum delay for timelocked operations can be bypassed

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-76
- **Submitter:** 0x37
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/76
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-76.md

## Brief Summary

maximum delay for timelocked operations can be bypassed

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Out-of-Gas Risk from Gas Gulf in Timelock's Calldata Check Logic

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-221
- **Submitter:** 0xSecuri
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/221
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-221.md

## Brief Summary

The implications of this issue are significant. High gas consumption not only leads to increased operational costs for users but also poses a risk of out-of-gas errors, which could render the contract unusable in certain situations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# executeRecovery safe problem

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-91
- **Submitter:** 0xWeakSheep
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/91
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-91.md

## Brief Summary

The `executeRecovery` function contains a significant vulnerability. Although `executeRecovery` uses `tstore` and `tload` for operations to retrieve whether the corresponding storage slot has the corresponding address, the function called via `calldata` does not check if the `safe` address is the same as the current contract. This leads to security risks where the function can be manipulated repeatedly to modify the contents of the transient storage, thereby facilitating an attack.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Execution of expired operations, potentially violating governance rules.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-60
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/60
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-60.md

## Brief Summary

Execution of expired operations, potentially violating governance rules.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Inconsistent state where the expiration period is shorter than the minimum delay, allowing operations to expire before they're eligible for execution

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-92
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/92
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-92.md

## Brief Summary

Inconsistent state where the expiration period is shorter than the minimum delay, allowing operations to expire before they're eligible for execution

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# Once a recovered safe's keys are compromised, other chains can be enforced to change their owners to the compromised ones

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-281
- **Submitter:** Allarious
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/281
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-281.md

## Brief Summary

In case different chains are using the same recovery spell, certain attack vectors are possible: (1) In case one of the chains has undergone a recovery that other chains haven't, and the new owners are compromised, the wallets on other chains can be enforced to change to the compromised keys. (2) Once the recovery spell on one of the chains has been created, those can be created on other chains as well and start the timer on `recoveryInitiated + delay` Impact The precondition on this issue is that wallets on different chains use the same set of revcovery spells. Since the protocol can not enforce the different recovery spells, once the first set of keys on all chains is compromised, if diff...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_36_group

# Setting high expirationPeriod bricks contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-81
- **Submitter:** Boy2000
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/81
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-81.md

## Brief Summary

kledi wallet proposals have an expiration date/period, after which they can not be executed. The owner of the wallet can modify it be calling `updateExpirationPeriod`: The only check is that the `newPeriod` is greater than `MIN_DELAY`. However there is no upper bound check. A wallet owner who has not audited the code (very likely) may assume that setting a high/max `expirationPeriod` will just mean that the proposals never expire. The code uses this value in both `isOperationExpired` and `isOperationReady`: `timestamp` is added to `expirationPeriod`, and since the latter is not bound, it will overflow and revert. As a result it would be impossible to execute any proposals. A recovery spell...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing `onlyOwner` Modifier for `_grantGuardian` Function

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-149
- **Submitter:** Brene
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/149
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-149.md

## Brief Summary

Missing `onlyOwner` Modifier for `_grantGuardian` Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Non-Standard Signature Usage in InstanceDeployer Contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-152
- **Submitter:** Brene
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/152
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-152.md

## Brief Summary

Non-Standard Signature Usage in InstanceDeployer Contract

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of index validation in `getSliceBytesHash` function

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-158
- **Submitter:** Brene
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/158
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-158.md

## Brief Summary

Lack of index validation in `getSliceBytesHash` function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Fund Recovery Mechanism for Expired Proposals in Timelock Contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-160
- **Submitter:** Brene
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/160
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-160.md

## Brief Summary

Lack of Fund Recovery Mechanism for Expired Proposals in Timelock Contract

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent Delay Configuration in `RecoverySpell` Contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-223
- **Submitter:** Brene
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/223
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-223.md

## Brief Summary

Inconsistent Delay Configuration in `RecoverySpell` Contract

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_38_group

# Use of Undefined EVM Opcodes in RecoverySpell.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-106
- **Submitter:** ChainSentry
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/106
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-106.md

## Brief Summary

A critical vulnerability has been identified in the `RecoverySpell.sol` contract of the Kleidi Wallet project. The contract utilizes non-standard EVM opcodes `tstore` and `tload` within assembly blocks. These opcodes are not recognized by the Ethereum Virtual Machine (EVM), leading to compilation failures and rendering the contract non-deployable and non-functional. This flaw not only disrupts the intended recovery mechanisms but also poses severe security risks if attempted to be exploited through unconventional means. Vulnerability Details

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_102_group

# Hardcoded salt which can replay attacks across different chains or environments

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-174
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/174
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-174.md

## Brief Summary

Hardcoded salt which can replay attacks across different chains or environments

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Access Control that can lead to unauthorized Deployment of contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-179
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/179
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-179.md

## Brief Summary

Lack of Access Control that can lead to unauthorized Deployment of contract

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Immutable Bytecode Check Issue can lead to erroneous assumptions about deployed contracts.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-187
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/187
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-187.md

## Brief Summary

Immutable Bytecode Check Issue can lead to erroneous assumptions about deployed contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Static Address Assumptions can cause interaction with outdated or incorrect contracts in the future.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-189
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/189
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-189.md

## Brief Summary

Static Address Assumptions can cause interaction with outdated or incorrect contracts in the future.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# _removeCalldataCheck function call may revert. Because the same dataHashe may cause the add function to revert

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-117
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/117
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-117.md

## Brief Summary

_removeCalldataCheck function call may revert. Because the same dataHashe may cause the add function to revert

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Malicious pauseGuardian can DOS protocol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-52
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/52
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-52.md

## Brief Summary

Malicious pauseGuardian can DOS protocol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Duplicate owner addresses bypass expected owner threshold, reducing Safe security

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-68
- **Submitter:** ETHworker
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/68
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-68.md

## Brief Summary

Duplicate owner addresses bypass expected owner threshold, reducing Safe security

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `Timelock.sol` is vulnerable to ABI smuggling when executing calls to other contracts that take raw bytes as arguments

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-271
- **Submitter:** Fon
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/271
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-271.md

## Brief Summary

`Timelock.sol` is vulnerable to ABI smuggling when executing calls to other contracts that take raw bytes as arguments

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_18_group

# no check on the call value allows a malicious HOT_SIGNER_ROLE to potentially steal ETH

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-273
- **Submitter:** Fon
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/273
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-273.md

## Brief Summary

no check on the call value allows a malicious HOT_SIGNER_ROLE to potentially steal ETH

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_70_group

# Lack of payable when deploying contracts

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-212
- **Submitter:** Guardians
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/212
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-212.md

## Brief Summary

The `deploy` function in the `SystemDeploy` contract lacks the `payable` modifier, preventing it from receiving native tokens (Ether) during contract creation. This is critical if any of the deployed contracts require Ether in their constructors. Without the ability to receive Ether, the deployment will fail when contracts attempt to access the funds needed for their initialization logic, resulting in potential system failures and inability to properly deploy essential components like the `TimelockFactory` or `Guard`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The duplicate owner check fails in 'createRecoverySpell'

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-255
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/255
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-255.md

## Brief Summary

The duplicate owner check fails in 'createRecoverySpell'

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Unchecked External Call in Timelock::_execute

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-256
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/256
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-256.md

## Brief Summary

Unchecked External Call in Timelock::_execute

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Unsecured Usage of `CREATE2` and `SALT` in TimelockFactory::createTimelock

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-258
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/258
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-258.md

## Brief Summary

Unsecured Usage of `CREATE2` and `SALT` in TimelockFactory::createTimelock

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# Insufficient Input Validation in 'createTimelock'

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-259
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/259
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-259.md

## Brief Summary

Insufficient Input Validation in 'createTimelock'

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Insecure Address Pre-computation in AddressCalculation.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-260
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/260
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-260.md

## Brief Summary

Insecure Address Pre-computation in AddressCalculation.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incomplete Input Validation in Create2Helper.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-262
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/262
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-262.md

## Brief Summary

Incomplete Input Validation in Create2Helper.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Setter Functions Inside the InstanceDeployer, Will Lead to Deploying the Same Contract Again Incase of Wrong Address Assignment to a State

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-156
- **Submitter:** JustAWanderKid
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/156
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-156.md

## Brief Summary

inside the `InstanceDeployer`, there's no setter function, in order to change addresses stored in each state incase of incorrect address assignment when deploying the `InstsanceDeployer`. also they're marked as `immutable`. this leads to `InstanceDeployer` being unsuable and useless in that particular scenario where, deployer assigns incorrect addresses to only one of the state variables. and then deployer will be forced to deploy new `InstanceDeployer` and pay additional gas for no reason. Impact If Invalid addresses are provided during deployment of `InstanceDeployer`, the `safe` and `Timelock` deployment process which relies on these states (such as safe proxy creation, timelock, multica...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Calldata Removal Related Functions, Should Inherit `onlySafe` modifier instead of `onlyTimelock`, so Safe can Remove Unsafe Calldatas Immediately

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-164
- **Submitter:** JustAWanderKid
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/164
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-164.md

## Brief Summary

calldata removal related functions like [removeCalldataCheck()](https://github.com/solidity-labs-io/kleidi/blob/0d72b6cb5725c1380212dc76257da96fcfacf22f/src/Timelock.sol#L857-L869), [removeCalldataCheckDatahash()](https://github.com/solidity-labs-io/kleidi/blob/0d72b6cb5725c1380212dc76257da96fcfacf22f/src/Timelock.sol#L871-L934) and [removeAllCalldataChecks()](https://github.com/solidity-labs-io/kleidi/blob/0d72b6cb5725c1380212dc76257da96fcfacf22f/src/Timelock.sol#L936-L952), do not currently inherit the `onlySafe` modifier, instead they inherit `onlyTimelock` modifier. This means that, if a malicious calldata is uncautiously added by safe owners through timelock, the Safe owners must wait...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Invariant that safe can only call itself with empty calldata can be broken

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-195
- **Submitter:** KlosMitSoss
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/195
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-195.md

## Brief Summary

When a transaction by the safe is made, it is checked by `checkTransaction()` in `Guard.sol` that it is no self or delegate call so that no new modules, upgrades, owners or fallback handlers can be added or removed by the transaction. It only allows self or delegate calls if the calldata is empty. However, compromised signers of the safe could still make the safe call itself with non-empty calldata by using `Safe.handlePayment()`. This means the invariant that the safe can only call itself with empty calldata is broken. If the safe is configured in such a way that its security relies on that invariant (for example with fallback handler) then it is possible that an attack can be performed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# hash collisions can occur because abi.encodePacked() is used with hashing operations of keccak256()

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-166
- **Submitter:** Moyinmaala
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/166
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-166.md

## Brief Summary

hash collision most likely will occur because `abi.encodePacked()` is used with hashing operations of `keccak256()` in create2helper.sol hash collisions are bound to occur because the parameters passed in `function calculateCreate2Address` for abi-encoding `creationCode, constructorParams` are both dynamic types. same issue with function `calculateCreate2Address` In solidity documentation it is advised against to use abi.encodePacked with dynamic parameters

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_45_group

# address.code.length can return false-positives

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-269
- **Submitter:** Moyinmaala
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/269
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-269.md

## Brief Summary

in `AddressCalculation.sol`, `.code.length` is used to check that the recovery spell address,safe and timelock instance have no bytecode, to prevent deploying to an address that already has a contract. however there are instances where there may be a contract at the said address. OpenZeppelin gives some insight to this possible implications 1. Contracts in Construction as noted in the OpenZeppelin will return 0 on `address.code.length` for contracts that are currently being constructed.A malicious actor contract can watch the mempool during when the original contract transaction is being deployed and plan the process of deployment of his malicious contract such that `address.code.length` wi...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Return Value Check in _revokeRole(HOT_SIGNER_ROLE, deprecatedHotSigner)

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-121
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/121
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-121.md

## Brief Summary

The `revokeHotSigner` function fails to check the `bool` return value from OpenZeppelin's `_revokeRole` function, which signals whether the role was successfully revoked. Ignoring this return value can result in unnoticed role revocation failures, impacting security.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Access Control Issue in Role Management Functions

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-124
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/124
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-124.md

## Brief Summary

The `grantRole`, `revokeRole`, and `renounceRole` functions in the contract are vulnerable to improper access control. These functions allow any user to grant or revoke roles for other accounts without restriction, except for the `DEFAULT_ADMIN_ROLE`. This opens up potential for privilege escalation or role manipulation by unauthorized users. Affected code Vulnerability: - **grantRole**: Anyone can grant roles to any account. - **revokeRole**: Anyone can revoke roles from any account. - **renounceRole**: Anyone can renounce roles for other accounts. Recommended Fix: Restrict role management actions to specific authorized accounts:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# Hardcoded 365-Day Check Fails to Account for Leap Years in Recovery Delay Validation"

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-241
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/241
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-241.md

## Brief Summary

The `_paramChecks` function in the `RecoverySpellFactory` contract incorrectly validates the maximum allowed `delay` for recovery operations. It assumes a year always consists of 365 days, failing to account for leap years. This can lead to unexpected transaction reverts when attempting to set a delay that includes February 29th in a leap year. This bug affects the createRecoverySpell and calculateAddress functions, as they rely on _paramChecks for parameter validation. Steps to Reproduce 1. Use the `createRecoverySpell` function in a leap year (e.g., 2024). 2. Provide a `delay` value that equates to 366 days or more (including February 29th). 3. The transaction will revert with the error "...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Non-Standard EIP-712 Implementation in RecoverySpell Contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-243
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/243
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-243.md

## Brief Summary

The RecoverySpell contract implements EIP-712 signature verification using manual message construction instead of OpenZeppelin's standard `MessageHashUtils` utility. While functionally correct, this divergence from best practices could lead to maintainability issues and potential vulnerabilities if the manual implementation is modified. The current implementation works correctly but deviates from security best practices in a critical security component. Vulnerable Code Impact 1. Harder to audit and verify correctness of EIP-712 implementation 2. Manual byte packing could lead to vulnerabilities if modified 3. Less gas efficient than OpenZeppelin's optimized implementation 4. Maintenance ove...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Signature Replay Attack Possible Due to Missing Nonce in Recovery Authorization Process

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-244
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/244
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-244.md

## Brief Summary

The RecoverySpell contract uses ECDSA signatures to authorize Safe recovery, requiring `recoveryThreshold` signatures from new owners. The signatures are collected off-chain and submitted in a single transaction via `executeRecovery`. Vulnerability Details The contract doesn't protect against frontrunning of signatures. Since signatures are collected off-chain and don't include nonces or timestamps, they remain valid indefinitely once the delay period has passed. A malicious actor could: 1. Collect valid signatures for one set of recovery parameters 2. Wait for the delay period 3. Front-run another recovery attempt with the previously collected signatures Vulnerable

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Block Timestamp Manipulation Enables Early Recovery Execution by Circumventing Delay Period

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-245
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/245
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-245.md

## Brief Summary

The RecoverySpell contract uses `block.timestamp` for two critical timing checks: 1. Setting initial recovery time: `recoveryInitiated = block.timestamp` 2. Enforcing delay period: `block.timestamp > recoveryInitiated + delay` Vulnerability Details Miners/validators can manipulate `block.timestamp` within a certain range (typically up to 15 seconds in Ethereum). In the RecoverySpell contract, this could affect the delay enforcement mechanism. Vulnerable Code Impact 1. Delay Period Manipulation: - Miners could slightly reduce the effective delay period - For short delays (e.g., hours), the manipulation could be significant percentage-wise - Critical for emergency response timing 2. Timestamp...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# Inefficient gas consumption in `calculateAddress` function due to O(n²) duplicate owner validation, potentially leading to transaction failures for large owner sets

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-248
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/248
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-248.md

## Brief Summary

The `calculateAddress` function is a critical component used to predict RecoverySpell contract addresses before deployment. This function is often called on-chain as part of larger transactions or integration flows. The current implementation uses a nested loop for duplicate checking: Gas consumption grows quadratically with the number of owners: - 5 owners: ~1,500 gas - 10 owners: ~6,000 gas - 50 owners: ~150,000 gas - 100 owners: ~600,000 gas Impact 1. Gas Inefficiency: - Excessive gas consumption for larger owner sets - Higher transaction costs for users - Potential for transaction failures due to block gas limits 2. Integration Risks: - Smart contracts integrating with RecoverySpellFact...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Recovery Spell Signature Verification Bypass Through EOA Self-Destruct

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-277
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/277
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-277.md

## Brief Summary

The RecoverySpell contract uses transient storage (`tstore`/`tload`) to track valid signers during signature verification. The contract assumes storage values of 0 indicate invalid/used signatures. However, this creates a vulnerability due to how EOA (Externally Owned Account) self-destruct and contract deployment work in Ethereum. The signature verification logic: The vulnerability arises because: 1. The contract uses transient storage value of 1 to mark valid signers 2. It checks `tload(address)` to verify signatures 3. A value of 0 is considered invalid/used 4. But crucially, transient storage for EOAs can be manipulated through self-destruct and redeployment Impact - Allows bypass of si...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# Unauthorized Safe Recovery Due to Missing Recovery Initiation Controls

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-279
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/279
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-279.md

## Brief Summary

The RecoverySpell system is designed to provide social recovery for Gnosis Safes. However, there's a critical issue in how recovery is initiated. The recovery process is automatically initiated in the constructor of RecoverySpell: Since RecoverySpellFactory is permissionless and allows anyone to create a RecoverySpell, this means: 1. Anyone can start a recovery process for any Safe 2. The delay timer starts immediately upon creation 3. There's no verification that the initiator has any relationship with the Safe Impact - Malicious actors can spam recovery attempts for any Safe - Each recovery attempt starts a delay timer that, once expired, could allow changing the Safe's owners - Creates D...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unnecessary Contract Deployments Before Prerequisite Check Leads to Gas Waste

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-280
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/280
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-280.md

## Brief Summary

The `deploy()` function creates unnecessary gas waste by allowing multiple contract deployments to succeed before failing on missing prerequisites for the `InstanceDeployer`. This causes maximum gas consumption while guaranteeing deployment failure in certain conditions. The current deployment sequence deploys contracts in this order: Gas Wastage 1. **Contract Creation Opcodes** - Each successful deployment consumes gas for CREATE2 operations - Storage writes for contract bytecode - Contract initialization costs 2. **Storage Operations** - Address registration in mapping - Storage slots updated for each successful deployment 3. **Failed Transaction** - All gas used by successful deployments...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect pause state determination in `paused()` function

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-51
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/51
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-51.md

## Brief Summary

Incorrect pause state determination in `paused()` function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# execute function can be still called after reschedule a proposal made.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-122
- **Submitter:** Satyam_Sharma
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/122
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-122.md

## Brief Summary

firstly let us explain how reschedule and its internal operations are working. schedule function internally make a call to _schedule function, which is responsible to schedule the timestamp for a proposal or a transactions by setting timestamps[id] mapping to block.timestamp + delay. And now in the execute function which can be called by anyone by providing target address, id, payload and a salt, which than make a internal call to _execute function that firstly removes the id from the _liveProposals and checks if the operation is ready to execute and make a call to _execute function to transfer value to the target address thereafter it make a call to _aftercall function which sets timestamp...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Wrong check performed under _addCalldataCheck, which makes the function to execute even if the start and end indexes are same.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-123
- **Submitter:** Satyam_Sharma
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/123
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-123.md

## Brief Summary

_addCalldataCheck function get called for adding a calldata check, the issue lies in the if statement under _addCalldataCheck , which is only check if length is 1. the above check might get easily bypassed unwillingly and of no use, when the _calldataList.length is greater than 1, which makes the check for calldataChecks at 0 index for start and end Index completely skip that could make the operation under _addCalldataCheck to occur when start and end Index are same for calldataChecks. The issue is likely to occur because their could be more than 1 Indexes that are already get registered and making it more open to completely skip this check.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Executing Recovery Operations on Certain EVM Chains will Fail

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-109
- **Submitter:** Takarez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/109
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-109.md

## Brief Summary

Executing a recovery on `zkSync` or Avalanche's `C-Chain` would be impossible due to the lack of support for the `tload` and `tstore` opcodes, which is used in functions like [executeRecovery](https://github.com/code-423n4/2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/RecoverySpell.sol#L165). Without these opcodes being supported on these chains, executing recovery operations would fail, rendering the function ineffective on these platforms. Description According to the [README](https://github.com/code-423n4/2024-10-kleidi#:~:text=The%20system%20only%20works%20on%20EVM%20compatible%20chains%2C%20does%20not%20work%20on%20chains%20that%20have%20not%20undergone%20the%20Shang...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential `EIP712` violation in multiple cases

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-10
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/10
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-10.md

## Brief Summary

---> Non-compliance with `EIP712` can cause problems with integrators and potentially lead to denial of service.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_53_group

# Self-call check can be bypassed in `checkTransaction()` via smart contract wallets

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-147
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/147
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-147.md

## Brief Summary

This allows calls where `to` is the original `EOA`'s address to bypass the check, as the condition `to == msg.sender` is no longer met.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Caller may use Ether held in contract without sending their own

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-23
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/23
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-23.md

## Brief Summary

The `executeWhitelisted()` and `executeWhitelistedBatch()` functions do not validate the `msg.value` to ensure that the `ether` sent with the call is the `ether` the caller intends to use. Because the contract can hold `ether` from other users (via the `receive()` function), a caller could execute these functions without providing any ether of their own. This would allow them to consume ether already held in the contract from previous users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_77_group

# Missing `calls3` validation in `executeRecovery()`

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-35
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/35
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-35.md

## Brief Summary

- Without validating that `calls3` is fully populated, the function may execute fewer calls than intended. This could prevent crucial updates or changes from being applied, potentially leaving the system in an incorrect or insecure state. - Errors resulting from this oversight may go unnoticed during execution, leading to silent failures where the recovery process does not complete as expected.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Validation on `instance` parameters in `createSystemInstance()`

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-43
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/43
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-43.md

## Brief Summary

The lack of validation on these parameters introduces potential vulnerabilities: - The absence of checks on `owner addresses` could allow the inclusion of the `zero address`, which could disrupt the normal operation of owner-based security mechanisms.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect access Control in `revokeHotSigner()` prevents `Timelock` execution

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-9
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/9
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-9.md

## Brief Summary

The `revokeHotSigner()` function is intended to be callable by both the `safe` and the `timelock` contracts, allowing them to `revoke` a hot signer's role. However, due to a restrictive modifier (`onlySafe`), only the `safe` can invoke this function, while attempts by the `timelock` will fail. This misconfiguration prevents the `timelock` from executing its intended role, which could lead to governance or administrative delays or failures in the revocation of signers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_35_group

# Error in Extraction of Function Signature

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-227
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/227
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-227.md

## Brief Summary

Improper extraction of function signatures in the BytesHelper contract would incorrectly interpret function selectors, it would inadvertently call the wrong function and fail to execute as intended. Description The code above shows how getFunctionSignature(...) function is implemented in the BytesHelper contract, it can be noted from the pointer above that the toSlice was casted into bytes4 in order to extract the function signature, function signatures are derived from the first four bytes of the keccak256 hash of the function's prototype. However it is a mistake to directly cast a bytes array to bytes4, which does not properly extract the intended bytes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# There is Missing Fallback for Pause Guardian Assignment Results in Unmanageable Unpaused State

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-143
- **Submitter:** XDZIBECX
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/143
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-143.md

## Brief Summary

The cause of the issue is occur In the pause function, When the function is called, the pause guardian is set to address(0), and there is no fallback is present to automatically assign a new guardian, and this is resulting that the system is being unable to pause again after the current pause expires in this line on the contract : pauseGuardian = address(0); Here this line is removes the pause guardian after a pause is initiated, and there is no mechanism is in place to reassign a new guardian. Without external intervention as result the system will become unmanageable and remains in an unpaused state indefinitely once the pause duration ends. impact The issue is can make the contract unpau...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Kleidi wallet cannot be created if user's safe version is 1,5+

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-29
- **Submitter:** ZanyBonzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/29
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-29.md

## Brief Summary

Kleidi wallet cannot be created if user's safe version is 1,5+

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# One single address can grief all other recipients.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-257
- **Submitter:** arman
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/257
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-257.md

## Brief Summary

- https://github.com/code-423n4/2024-10-kleidi/blob/main/src/Timelock.sol#L642 lets someone to send native ether to multiple addresses at once. But if one of the recipients is a contract that can't receive ether or intentionally reverts upon receiving ether the whole tx fails and no recipient gets paid. Impact * One single recipient can grief all other recipients. * Loss of gas since the whole tx will fail.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Attacker has the ability to steal funds from the timelock contract.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-270
- **Submitter:** arman
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/270
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-270.md

## Brief Summary

- https://github.com/code-423n4/2024-10-kleidi/blob/main/src/Timelock.sol#L594 - https://github.com/code-423n4/2024-10-kleidi/blob/main/src/Timelock.sol#L626 Timelock contract has a function which means anyone can send native ether to it. Both and are , functions. Tx senders need to declare how much fund they would like to distribute among recipients through function arguments. But during function call none of the above 2 functions checks how much fund the sender declared to distribute and how much fund they actually sent to the contract. As long as there is fund available in the timelock contract a tx sender is able to distribute funds among recipients without sending a single wei. An atta...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_48_group

# eth can be lost in "Timelock"

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-101
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/101
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-101.md

## Brief Summary

eth can be lost in "Timelock"

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# "calculateCreate2Address" is vulnerable to a birthday attack

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-57
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/57
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-57.md

## Brief Summary

"calculateCreate2Address" is vulnerable to a birthday attack

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Possible loss of funds because the `_execute` function allow valid calls to be made to addresses without code

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-66
- **Submitter:** catellatech
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/66
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-66.md

## Brief Summary

The internal function `_execute` in the **Timelock** contract allows executing calls to arbitrary addresses using low-level calls. However, there is no prior check to verify whether the `target` address has any contract code implemented. This may allow valid calls to be made to addresses without code (such as EOAs), which could result in loss of funds or undesired behavior. The vulnerability affects the `execute`, `executeBatch`, `executeWhitelisted`, and `executeWhitelistedBatch` functions. - https://github.com/code-423n4/2024-10-kleidi/blob/main/src/Timelock.sol#L1017-L1026 In the docs the team clarify that Timelock contract `holds all funds` and requires a delay before a transaction can...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Reliance on block.timestamp for Critical Contract Logic on ConfigurablePause contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-252
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/252
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-252.md

## Brief Summary

Reliance on block.timestamp for Critical Contract Logic on ConfigurablePause contract

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Access Control on createTimelock Function

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-261
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/261
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-261.md

## Brief Summary

Missing Access Control on createTimelock Function

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# Inconsistent Batch and Individual Operation Scheduling in Timelock Contract

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-39
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/39
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-39.md

## Brief Summary

Inconsistent Batch and Individual Operation Scheduling in Timelock Contract

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Users could execute operations before the intended timelock period has passed, bypassing the security measures put in place by the timelock mechanism.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-44
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/44
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-44.md

## Brief Summary

Operations can be executed before their intended timelock period has fully elapsed. This issue stems from the order of operations in the `execute` function, where a proposal is removed from the `_liveProposals` set before its readiness is checked. Due to a race condition in the `execute` function. The function first removes the proposal from `_liveProposals` and then checks if the operation is ready. This creates a window where multiple transactions can attempt to execute the same operation, potentially leading to premature execution. The issue arises because the `_liveProposals.remove(id)` operation is performed before checking if the operation is ready. This can lead to a scenario where a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# An attacker can exploit reentrancy during the proposal build process to call _simulateActions multiple times, causing unexpected proposal execution flow.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-230
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/230
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-230.md

## Brief Summary

Proposal State Manipulation: Reentrancy within _startBuild and _endBuild enables an attacker to manipulate the proposal lifecycle, resulting in repeated or reordered actions. Bypass of Validation: The ignored return in _simulateActions causes any failed actions to appear successful, enabling an attacker to bypass security checks and introduce inconsistent multisig execution states. Critical Proposal Execution Vulnerability: This attack chain compromises the proposal’s integrity, allowing unauthorized or incorrect actions to pass as valid in the multisig process.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reentrancy in InstanceDeployer.createSystemInstance() Causing Failed Safe Creation Reporting

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-232
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/232
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-232.md

## Brief Summary

A reentrancy attack could result in inconsistent or incorrect event reporting, leaving failed system instance creations undetected or incorrectly reported, which can affect system auditability and integrity.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_95_group

# Reentrancy in RecoverySpell.executeRecovery() Leading to Inconsistent Recovery

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-233
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/233
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-233.md

## Brief Summary

A reentrancy attack could disrupt the recovery process, leading to an incomplete or incorrect recovery, which may enable unauthorized access or interfere with the recovery mechanism.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Privilege Escalation in Timelock allows hot signers to execute administrative functions through whitelisted calls, bypassing timelock governance controls.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-171
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/171
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-171.md

## Brief Summary

[src/Timelock.sol#715-L725](https://github.com/code-423n4/2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/Timelock.sol#L715-L726) [src/Timelock.sol#1034](https://github.com/code-423n4/2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/Timelock.sol#L1034) The issue exists in the interaction between three key components: 1. Hot signer role permissions 2. `Calldata` whitelisting mechanism 3. Administrative function access controls > If administrative functions can be whitelisted, it breaks the core security assumption that these functions must go through timelock governance.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Race condition where operations can be cleaned up multiple times, breaking core timelock functionality and security assumptions.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-172
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/172
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-172.md

## Brief Summary

Cleanup operation modifies critical state variables (`_liveProposals`) after making external calls, violating the checks-effects-interactions pattern. The `isOperationExpired()` check and subsequent state modifications are not atomic, creating a window for reentrancy. An attacker can reenter during the cleanup process before state variables are updated, potentially executing multiple cleanups on the same operation. The vulnerability point [src/Timelock.sol#L671-L677](https://github.com/code-423n4/2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/Timelock.sol#L671-L677) Attack Sequence: 1. Attacker initiates cleanup for an expired operation 2. During `isOperationExpired` check...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Value Accumulation Leading to Arithmetic Overflow

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-175
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/175
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-175.md

## Brief Summary

Timelock contract fails to track and validate cumulative value transfers across execution paths, enabling potential arithmetic overflow attacks through multiple transaction executions, as the contract executes value transfers without maintaining or validating the total accumulated value. In `_execute()`, each call transfers value independently, without considering previous transfers. This design allows the total transferred value to silently overflow when multiple transactions are executed sequentially or in batch. [src/Timelock.sol#L1021-L1026](https://github.com/code-423n4/2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/Timelock.sol#L1021-L1026) [src/Timelock.sol#L589-L60...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Large batch sizes can lead to DoS attacks by consuming excessive gas

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-181
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/181
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-181.md

## Brief Summary

Unbounded Batch Operations stems from the lack of size validation in batch operations. The contract allows arbitrarily large arrays to be processed in `scheduleBatch()` and `executeBatch()`. Each operation in these batches requires gas for execution and event emission, scaling linearly with batch size. The contract fails to enforce limits on batch operation sizes, allowing 1. Creation of excessively large batches that may exceed block gas limits 2. Potential DoS attacks through gas exhaustion 3. Governance disruption via failed transactions Scenario 1. Alice (malicious safe owner) creates a batch with 1000 operations 2. Bob (legitimate user) has a time-sensitive proposal queued after Alice's

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_42_group

# Cancelled Operations Can Be Re-executed Due To Timestamp Invalidation Issue

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-186
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/186
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-186.md

## Brief Summary

1. The proposal is removed from `_liveProposals` 2. `timestamps[id]` is deleted (set to 0) 3. A subsequent execution could still pass the `isOperationReady()` check if the timestamps and expiration period align correctly The vulnerability stems from improper invalidation of cancelled operations. When an operation is cancelled, its timestamp is set to 0 using `delete`. However, the `isOperationReady()` check can still pass with a timestamp of 0 due to how the arithmetic conditions are structured, allowing cancelled operations to be re-executed if timed correctly. The issue occurs in the following sequence 1. Safe schedules an operation, setting: * `timestamps[id]` = future timestamp * `_live...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_09_group

# MEV bots can extract value by manipulating transaction ordering

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-191
- **Submitter:** foxb868
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/191
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-191.md

## Brief Summary

Whitelisted execution mechanism allows multiple hot signers to execute the same transaction, creating opportunities for MEV extraction through transaction ordering manipulation, occurs because `executeWhitelisted()` provides no ordering guarantees between different hot signers executing the same whitelisted transaction. When multiple hot signers attempt to execute identical transactions in the same block, their ordering can be manipulated by MEV bots through gas price competition. [src/Timelock.sol#executeWhitelisted](https://github.com/code-423n4/2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/Timelock.sol#L715-L726) [src/Timelock.sol#_execute](https://github.com/code-423n...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Uninitialized Storage Pointer in 2024-10-kleidi/src/ConfigurablePause.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-142
- **Submitter:** ihuntpackets
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/142
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-142.md

## Brief Summary

Uninitialized Storage Pointer in 2024-10-kleidi/src/ConfigurablePause.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Out of Bounds Read in 2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/BytesHelper.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-176
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/176
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-176.md

## Brief Summary

Out of Bounds Read in 2024-10-kleidi/blob/ab89bcb443249e1524496b694ddb19e298dca799/src/BytesHelper.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Unprotected Self-Destruct in 2024-10-kleidi/src/TimelockFactory.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-192
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/192
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-192.md

## Brief Summary

Unprotected Self-Destruct in 2024-10-kleidi/src/TimelockFactory.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_96_group

# Delegatecall Injection in InstanceDeployer.sol leads to Arbitrary Code Execution

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-206
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/206
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-206.md

## Brief Summary

Delegatecall Injection in InstanceDeployer.sol leads to Arbitrary Code Execution

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Critical bug in InstanceDeployer blocks system instance creation, rendering the Kleidi system unusable

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-177
- **Submitter:** krisp
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/177
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-177.md

## Brief Summary

The **InstanceDeployer** contract is a crucial component of the Kleidi protocol, responsible for creating new wallet instances. However, a critical bug in the `createSystemInstance` function prevents the successful deployment of system instances. This issue lies in the construction of the signature used in the Safe's `execTransaction` function, which fails due to incorrect encoding, thereby blocking instance creation. Without functional instance creation, the Kleidi system is rendered unusable, as it heavily relies on this process to onboard users and create secure wallets. Vulnerability Details The issue originates from the way the signature is crafted in the `createSystemInstance` functio...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_33_group

# aad

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-8
- **Submitter:** kumb1d1pm
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/8
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-8.md

## Brief Summary

aad

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Front-running on execute function

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-224
- **Submitter:** linemi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/224
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-224.md

## Brief Summary

The `execute` function is susceptible to front-running. In this function, users can submit transaction details such as target, value, payload, and salt in cleartext calldata. Since these parameters are visible on the blockchain before transaction execution, a malicious actor could monitor pending transactions in the mempool, intercepting and replicating them with potentially modified parameters (such as redirecting target to an address they control or modifying value). This vulnerability can be particularly problematic in high-stakes environments, where the function interacts with valuable assets or executes critical contract logic. Attackers could manipulate transaction ordering by paying...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# eip712 domain separator does not include chainID

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-116
- **Submitter:** maxim371
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/116
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-116.md

## Brief Summary

eip712 domain separator does not include chainID

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Transaction Cancellation Bypass in Guard.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-264
- **Submitter:** maxim371
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/264
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-264.md

## Brief Summary

Transaction Cancellation Bypass in Guard.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_54_group

# Timelock Salt Derivation Issue in addresscalculation.sol

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-272
- **Submitter:** maxim371
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/272
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-272.md

## Brief Summary

Timelock Salt Derivation Issue in addresscalculation.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# Scheduled operations will take more delay time on Pausing Guardian.

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-178
- **Submitter:** sil3th
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/178
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-178.md

## Brief Summary

Scheduled operations will take more delay time on Pausing Guardian.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect onlyTimelock modifier restricts function calls to the contract itself

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-110
- **Submitter:** xiao
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/110
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-110.md

## Brief Summary

Incorrect onlyTimelock modifier restricts function calls to the contract itself

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Potential Address Collision in create2 Deployment due to Missing chainId in Salt Calculation

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-184
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/184
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-184.md

## Brief Summary

Potential Address Collision in create2 Deployment due to Missing chainId in Salt Calculation

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# Incorrect Handling of Function Selector Extraction in getFunctionSignatur

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-193
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/193
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-193.md

## Brief Summary

Incorrect Handling of Function Selector Extraction in getFunctionSignatur

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Length Limitation in sliceBytes Function

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-194
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/194
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-194.md

## Brief Summary

Lack of Input Length Limitation in sliceBytes Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The supportsInterface function of the Timelock contract does not comply with ERC721 and ERC1155, destroying composability

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-94
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/94
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-94.md

## Brief Summary

The supportsInterface function of the Timelock contract does not comply with ERC721 and ERC1155, destroying composability

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_37_group

# should delete timestamps[id] when clean up an expired timelock action

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Submission:** V-2
- **Submitter:** zhanmingjing
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-kleidi-validation/issues/2
- **Source snapshot:** competitions/2024-10-kleidi/submissions/raw/V-2.md

## Brief Summary

should delete timestamps[id] when clean up an expired timelock action

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_20_group
