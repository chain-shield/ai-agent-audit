# Rejected Primary Findings: Karak Restaking

# Asymmetric upgrade between `nodeImpl` and `slashStore`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-297
- **Submitter:** 0x18a6
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/297
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-297.md

## Brief Summary

The asymmetric upgrade between `nodeImpl` and `slashStore` introduce incompatibility issues. This design flaw could lead to inconsistent slashing processes and limited protocol evolution, ultimately affecting the reliability and security of the system. A native vault is different from the regular vault by the means of `self.nodeImpl` and `self.slashStore`: - `nodeImpl` is upgradable: - `slashStore` is not upgradable. - Both interact in `_transferToSlashStore` which handles the transfer of slashed assets to the `slashStore`. Both should be able upgradable.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `_getParentBlockRoot` not compliant with EIP-4788 integration guidelines

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-300
- **Submitter:** 0x18a6
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/300
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-300.md

## Brief Summary

This is the [code assessment of the EIP-4788 contract](https://cdn.prod.website-files.com/65d35b01a4034b72499019e8/664203f92deb52f6d21723cb_ChainSecurity_Ethereum_Foundation_EIP-4788_Contract_Audit_compressed.pdf) Quoting 8.1.2 Integration Guidelines for Developers: > The calldata has to be __exactly 32 bytes__ and should only contain the timestamp, in big-endian format, which is the EVM default. However, `NativeVault::_getParentBlockRoot` uses `bytes` as calldata for the staticall to `Constants.BEACON_ROOTS_ADDRESS` because `abi.encode` returns `bytes` not `bytes32`, NOT EXACTLY 32 BYTES. This could cause failures in retrieving parent block roots, which are critical for various operations...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Loss of funds due to unchecked slippage in balance updates

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-346
- **Submitter:** 0x18a6
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/346
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-346.md

## Brief Summary

The `NativeVault::_increaseBalance` and `NativeVault::_decreaseBalance` functions, as currently implemented, could potentially lead to loss of funds due to unchecked slippage. Here are the functions: There is several issue with these

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# It may be returned due to inaccurate gas calculation.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-255
- **Submitter:** 0xHash
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/255
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-255.md

## Brief Summary

It may be returned due to inaccurate gas calculation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Mechanism to Remove Unsupported Tokens from SlashingHandler

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-217
- **Submitter:** 20centclub
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/217
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-217.md

## Brief Summary

The `SlashingHandler.sol` contract allows for adding tokens to the list of supported assets via the `addSlashableToken()` function. However, there is no mechanism to remove tokens from this list. This can lead to issues if a token becomes unsupported or problematic, as it cannot be removed from the list of supported assets. This can be problematic if a token becomes obsolete, faces security vulnerabilities, or needs to be deprecated for any reason. Impact The inability to remove tokens from the list of supported assets poses a risk of maintaining support for problematic or deprecated tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Comment Regarding _depositToken Parameter Leading to Potential Misuse

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-222
- **Submitter:** 20centclub
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/222
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-222.md

## Brief Summary

In the `NativeVault.sol` the docs for the `_depositToken` parameter in the `initialize()` function incorrectly states that it can be set to `address(0)`. However, the implementation explicitly checks for and reverts if `_depositToken` is set to `address(0)`, causing a discrepancy between the code and its documentation. Impact This discrepancy between the docs and the actual implementation can lead to confusion and potential misuse of the function. Developers relying on the comments might pass `address(0)` for `_depositToken`, resulting in unexpected reverts and disruption of the contract's initialization process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing check for validator slashed status

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-246
- **Submitter:** 20centclub
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/246
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-246.md

## Brief Summary

The code currently lacks a check for whether a validator has been [slashed](https://ethresear.ch/t/slashing-proofoor-on-chain-slashed-validator-proofs/19421). This omission could allow a slashed validator to be treated as active or valid, potentially leading to incorrect behavior in the system. Specifically, a slashed validator may be erroneously considered for withdrawals or staking operations, which could undermine the protocol's integrity and security. Impact The issue is exacerbated by the fact that the exit epoch is not set immediately upon slashing ([here](https://consensys.io/blog/understanding-slashing-in-ethereum-staking-its-importance-and-consequences)), allowing slashed validator...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Initializable is not inherited by `NativeVault`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-183
- **Submitter:** 4B
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/183
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-183.md

## Brief Summary

`_disableInitializer()`, `initializer()` and other important components won't work as expected. This will lead to loss of protection on the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Improper Validation can lead to smart contract wallets/multisig to register as DSS

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-299
- **Submitter:** Atharv
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/299
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-299.md

## Brief Summary

In `Core.sol`, the function `registerDSS()` is designed to register a Decentralized Staking System (DSS) with the platform. Currently, this function checks if the caller is a smart contract by checking the code length at the caller's address. However, this method is not foolproof. With the widespread use of smart contracts and multisig wallets, it’s possible for malicious attacker to exploit this simple check, registering unauthorized DSS instances and potentially compromising the system’s integrity. Code Impact This vulnerability could allow unauthorized entities to register as DSS, potentially leading to security breaches or manipulation within the system. Malicious actors could exploit t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_37_group

# Some tokens can never get withdrawn due to the current implementation of `NativeNode#withdraw()`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-139
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/139
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-139.md

## Brief Summary

As hinted under _Proof of Concept_, some tokens have a broken functionality attached to them when trying to [withdraw the slashed assets](https://github.com/code-423n4/2024-07-karak/blob/ab18e1f6c03e118158369527baa2487b2b4616b1/src/NativeNode.sol#L36), which is because the current functionality only allows for the withdrawal of native ETH via `sendValue()`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# Manager of Core can be the 0 address.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-223
- **Submitter:** DrCaligari
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/223
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-223.md

## Brief Summary

The initialize function in the Core contract does not check if the _manager address is the zero address (address(0)). This oversight can leave the contract without a valid manager, resulting in the loss of critical administrative control, such as managing, pausing, unpausing, and upgrading functionalities. This poses a severe risk to contract security. On first look it seems that _vetoCommittee is not validated for address(0) but it is later validated in the init method.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# The user can call the registerOperatorToDSS() function when the contract is paused by governance

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-194
- **Submitter:** Inspecktor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/194
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-194.md

## Brief Summary

The Core.sol contract provides the ability for a user with the MANAGER_ROLE or owner role to pause the contract: The Core.registerOperatorToDSS() function has a whenFunctionNotPaused modifier. When the contract is paused, the user cannot call the Core.registerOperatorToDSS() function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Use safeTransfer/safeTransferFrom consistently instead of transfer/transferFrom

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-195
- **Submitter:** Inspecktor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/195
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-195.md

## Brief Summary

It is good to add a require() statement that checks the return value of token transfers or to use something like OpenZeppelin’s safeTransfer/safeTransferFrom unless one is sure the given token reverts in case of a failure. Failure to do so will cause silent failures of transfers and affect token accounting in contract. This may result in the contract losing funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Invalid Initialization of 'initialize' Function in Contract

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-47
- **Submitter:** JuggerNaut63
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/47
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-47.md

## Brief Summary

This vulnerability is high impact as the *'SlashingHandler'* contract will not be able to function properly if initialized with invalid parameters. Without a supported asset, the 'handleSlashing' function will not be able to perform asset slashing, meaning the contract cannot perform its main logic. Additionally, this could open up other potential risks if the contract is forced to work under invalid circumstances, such as the occurrence of transaction failures or other vulnerabilities that have not yet been identified.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Improper Validation of _depositToken in Initialization

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-62
- **Submitter:** JuggerNaut63
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/62
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-62.md

## Brief Summary

Improper Validation of _depositToken in Initialization

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Misleading Hook Call Logic Due to Incorrect Negation of toStake Parameter

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-291
- **Submitter:** LonelyWolfDemon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/291
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-291.md

## Brief Summary

The `requestUpdateVaultStakeInDSS` function allows an operator to either stake or unstake a vault to/from a DSS (Decentralized Staking System). The StakeUpdateRequest struct includes a boolean field, toStake, which indicates whether the operator intends to stake (true) or unstake (false) the vault. Within the `requestUpdateVaultStakeInDSS` function, the callHookIfInterfaceImplemented is invoked to handle hooks if implemented by the DSS. One of the parameters for this function, ignoreFailure, is set to the negation of `requestStakeUpdate.toStake` This means that if the operation is a stake (toStake is true), ignoreFailure is set to false, and if the operation is an unstake (toStake is false)...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# No specific support for certain ERC20 tokens

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-110
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/110
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-110.md

## Brief Summary

Operators with vaults with these set of assets(Pausable tokens) can avoid getting slashed

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The Return Value of the `callHookIfInterfaceImplemented` Function is not Validated, This can Mess up the Core Storage Variables

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-281
- **Submitter:** Mike_Bello90
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/281
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-281.md

## Brief Summary

The `callHookIfInterfaceImplemented` function returns a boolean to confirm whether the call to the DSS contract hooks was successful, but this variable is not checked in the functions that call the `callHookIfInterfaceImplemented` function, when the `ignoreFailure` flag is true for these calls, a failure in the DSS hook call will not revert the transaction, this can cause Core storage variables to be updated with incorrect values that may differ from the possible state in the DSS contract causing conflicts between the states of the core and the DSS contracts and probably messing up the state variables of the operators, slashing and vaults for the DSS in the `core` contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# The `activeValidatorCount` variable should be also updated when the validator is force to exit the Beacon Chain

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-302
- **Submitter:** Mike_Bello90
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/302
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-302.md

## Brief Summary

There is an edge case that can cause a decrease in the value of the `activeValidatorCount` variable that is not being considered in the current code, this will cause an incorrect state in the `activeValidatorCount` variable in the nativeVault.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users may lose assets through withdrawals if the withdrawable Assets are less than the assets the user wishes to withdraw

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-342
- **Submitter:** Mike_Bello90
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/342
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-342.md

## Brief Summary

When a User tries to withdraw assets from a Native Vault he may lose assets if the `withdrawableAssets` are less than the assets the user is attempting to withdraw.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_43_group

# The `cancelSlashing` function should reset all the variables modified by the `requestSlashing` function

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-350
- **Submitter:** Mike_Bello90
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/350
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-350.md

## Brief Summary

The `cancelSlashing` function should reset all the variables modified by the `requestSlashing` function, so everything goes back to normal after canceling a slash event.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# SupportsInterface call doesn't comply with the ERC165 standard.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-239
- **Submitter:** MrValioBg
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/239
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-239.md

## Brief Summary

The HookLib.sol is responsible for calling hooks on the DSS contract. Before calling one, it uses the ERC165 to validate that the DSS contract supports the function selector that will be called as a hook. However, the implementation does not follow Ethereum's [ERC165 standard.](https://eips.ethereum.org/EIPS/eip-165) Vulnerability Details In the details of the Karak audit page on CodeArena, it states that: The ERC165 standard states that the call to the supportsInterface method should be a `STATICCALL` for security, ensuring that the call won't modify the state. However, in the current implementation in HookLib.sol, the performLowLevelCallAndLimitReturnData uses `CALL` instead of `STATICCAL...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Old validator withdraw proof can be used due to missing validation, which would result in inaccurate totalAssets

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-317
- **Submitter:** MrValioBg
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/317
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-317.md

## Brief Summary

DSSes that use Operators utilizing NativeVault are subject to the risk of having a stale amount of restaked Ethereum due to missing validation of the proof's timestamp. Vulnerability Details In NativeVault's implementation, a proof of restaked ETH for a particular NativeNode is submitted following the initiation of a snapshot. Subsequently, a 7-day window is available to submit this proof using the validateSnapshotProofs() function: However, it currently only checks if the Merkle proof is created after snapshot.parentBeaconBlockRoot (i.e., after last snapshot that has been has started). This validation does not prevent the submission of proofs that, although generated after the snapshot's s...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_56_group

# newNodeImplementation in changeNodeImplementation can affect critical functions

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-184
- **Submitter:** Mwendwa
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/184
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-184.md

## Brief Summary

The function in the contract updates the address in the contract's state to a new implementation address provided as . This change affects how the NativeNodes operate by pointing them to a new implementation. This means that any future and pending interactions with the NativeNodes will be directed to the new implementation, potentially altering the behavior of these nodes depending on the changes in the new implementation code. If in between the node implementation is changed and lets say function removed then it will affect pending txns

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_03_group

# Incorrect conversion from uint256 to int256 In the NativeVaultLib.sol::validateSnapshotProof

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-187
- **Submitter:** Mwendwa
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/187
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-187.md

## Brief Summary

the function calculates as the difference between and , both of which are uint256. The calculation is performed using int256 to handle the possibility of a negative difference.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Incorrect return value in *getNextWithdrawNonce()*

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-156
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/156
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-156.md

## Brief Summary

The getNextWithdrawNonce function does not perform as expected based on its name. Instead of returning the next nonce value for a given `nodeOwner`, it returns the current nonce value from the `nodeOwnerToWithdrawNonce` mapping.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Existence Check in `getNodeOwner` Function

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-157
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/157
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-157.md

## Brief Summary

The `getNodeOwner` function does not perform an existence check to ensure that the provided node address corresponds to a valid owner as other functions like [getValidatorDetails](), [lastSnapshotTimestamp](), [currentSnapshotTimestamp]()[startSnapshot](), [validateSnapshotProofs](), [validateWithdrawalCredentials](), [validateExpiredSnapshot](), [startWithdrawal](), [withdrawableWei](), [activeValidatorCount]().

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Outdated(Expired) Merkle proof generated can be reused to validate a balanceProof and balanceContainer

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-293
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/293
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-293.md

## Brief Summary

The `validateSnapshotProofs` function is used to validate balance proofs for all active validators associated with a specific node owner. However, the function does not check if the `lastSnapshotTimestamp` is still active, meaning it does not validate whether the snapshot has expired before validating new proofs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Duplicate Detection in CommonUtils Library

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-303
- **Submitter:** Nexarion
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/303
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-303.md

## Brief Summary

The `hasDuplicates` function in the `CommonUtils` library fails to correctly identify duplicates in all cases, potentially leading to false negatives. The root cause is that the function assumes duplicates will always be adjacent after sorting, which is not guaranteed due to the sorting algorithm's implementation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_63_group

# Potential Overflow in Bit Shift Amount Calculation

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-332
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/332
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-332.md

## Brief Summary

The calculation of `bitShiftAmount` could potentially overflow if `validatorIndex` is very large, as it's multiplied by 64: While `validatorIndex` is defined as a `uint40`, which might seem to limit its maximum value, it's crucial to consider that Solidity performs integer promotion to `uint256` for arithmetic operations. This means that if `validatorIndex` were to somehow contain a value larger than `2^40 - 1`, the multiplication could lead to unexpected results. Impact If exploited, this overflow could lead to incorrect balance calculations, potentially allowing for manipulation of validator balances. In a proof-of-stake system, this could have significant consequences for validator rewar...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient validation leads to overwriting Existing Slash Handlers

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-339
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/339
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-339.md

## Brief Summary

The validation in the `allowlistAssets` function is not implemented correctly which can lead to serious issues This validation is insufficient because: 1. It only checks that the asset and slashing handler addresses are not zero addresses. 2. It doesn't verify if the asset is already allowlisted or if it's being reassigned a different slashing handler. 3. Most importantly, it lacks a crucial check that is present in the `validateVaultConfigs` function: The `validateVaultConfigs` function checks if an asset has a slashing handler assigned, which implies that it's allowlisted. However, the `allowlistAssets` function doesn't perform any check to see if the asset is already allowlisted or if it...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Precision Loss in Effective Balance Conversion

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-340
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/340
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-340.md

## Brief Summary

In the `BeaconProofsLib` contract, there's a potential for precision loss when converting the effective balance from gwei to wei, particularly for large balance values. **Current Implementation**: issue While this conversion is generally accurate, it may lead to precision loss for very large balance values due to the limitations of uint64 when converted to wei (which requires higher precision). Impact - Slight inaccuracies in reported validator balances - Possible under or over-estimation of total stake - Potential discrepancies in calculations relying on precise balance values Suggested Solution 1. Consider using a higher precision type (like uint256) throughout the balance calculation pro...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can be unfairly slashed

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-279
- **Submitter:** Nyx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/279
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-279.md

## Brief Summary

Users can be unfairly slashed

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# The function Core::pause does not fulfill its purpose

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-124
- **Submitter:** PedroDowsers
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/124
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-124.md

## Brief Summary

If the contract owner needs to pause a function or all functions due to an attack, the Core::pause function will not be able to pause the function(s).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Operators can get away without being slashed

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-259
- **Submitter:** Pheonix
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/259
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-259.md

## Brief Summary

Operators could get away without being slashed due to rounding error while calculating `EarmarkedStakes`

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Should Skip Instead Of Revert If Balance Is Already Proven

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-54
- **Submitter:** Prestige
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/54
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-54.md

## Brief Summary

Since anyone can submit balance proofs, if a balance proof has already been submitted the function `validateSnapshotProofs` should simply skip that balance proof instead of reverting the whole transaction. Reverting can cause problems when different array lengths of balance proofs are submitted by different users, making submitting the proofs a convoluted process. Also this process is able to be grieved Say a user wants to submit 20 balance proofs, a troll can submit just 1, making the original users entire transaction fail, forcing them to review the transaction and alter the array and try again, where they can be dossed again

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `merkleizeSha256` won't work as expected

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-4
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/4
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-4.md

## Brief Summary

`merkleizeSha256` function in `MerkleProofs.sol` does not validate that the input array length is a power of two, which is a critical `precondition` for correct operation. This could cause incorrect merkleization results if the function is called by any other function with an input that doesn't meet this requirement. This will cause invalid proofs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Incorrect balance extraction in `validateBalance` due to error in bit shift operation

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-5
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/5
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-5.md

## Brief Summary

This will result in incorrect balance calculations for most validator indices, leading to severe misrepresentations of validator balances throughout the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# Incorrect `Pause/Unpause` logic in pauser contract leading to unintended state changes which contradicts the `ReadMe`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-7
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/7
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-7.md

## Brief Summary

The current implementation of the `_pause` and `_unpause` functions can lead to unintended state changes. This could result in accidentally pausing or unpausing functions that were not meant to be affected, potentially disrupting the intended functionality of the contract and any contracts that inherit from it.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_73_group

# Incorrect Use of LibClone.predictDeterministicAddressERC1967BeaconProxy in Smart Contract

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-359
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/359
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-359.md

## Brief Summary

The `CoreLib` library incorrectly utilizes the `LibClone.predictDeterministicAddressERC1967BeaconProxy` function during the creation of new vaults. Specifically, the function is provided with `address(this)` as the salt parameter instead of a uniquely generated salt, leading to inaccurate predictions of the new vault's address. Vulnerability Detail The vulnerability arises in the `createVault` function within the `CoreLib` library. The function aims to predict the address of a new vault using the `LibClone.predictDeterministicAddressERC1967BeaconProxy` function. However, the salt parameter passed to this function is mistakenly set to `address(this)`, which represents the address of the `Cor...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Malicious DSS can't be removed/penalized as there's no way to do that.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-73
- **Submitter:** Stormreckson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/73
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-73.md

## Brief Summary

A Dss service can be created by anyone and have operators registered to them, the DSS can also set a slashable percentage up to 100%, to slash operators when needed. However when the Dss becomes malicious and slashes operators maliciously, this DSS can't be removed or penalized Impact Malicious DSS can harm the protocol reputation by slashing operators maliciously, and as Dss can't be removed/penalized this will harm the protocols reputation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Anyone can initialize without any restrictions

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-232
- **Submitter:** Taiger
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/232
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-232.md

## Brief Summary

The problem occurs in the initialize function of the Core, Vault, NativeNode, NativeVault, and SlashStore contracts, which can be called by anyone without any restrictions. This vulnerability allows malicious actors to initialize contracts with arbitrary values ​​before the legitimate owner. As a result, the attacker can control key roles and parameters, overall. The following effects may result: 1. Unauthorized access: The attacker can set himself as the owner, manager, and veto committee, gaining full control of the contract. 2. Loss of funds: After controlling the contract, the attacker can manipulate the vault and assets, which may lead to theft or mismanagement of funds. 3. Operational...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Undefined _beforeWithdraw method causes contract rollback

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-236
- **Submitter:** Taiger
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/236
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-236.md

## Brief Summary

The `_decreaseBalance` method in the NativeVault contract calls the _beforeWithdraw method, which is responsible for reducing the user's balance by burning the corresponding share and updating the total assets and re-staked ETH. However, the method calls `_beforeWithdraw`, which is not defined in the contract or the project. This may lead to multiple potential security risks, including but not limited to: - **Contract rollback**: Since the `_beforeWithdraw` method is undefined, when the `_decreaseBalance` method tries to call `_beforeWithdraw`, the contract will trigger a rollback. This is because calling an undefined method in Solidity will cause the contract execution to fail, thereby rol...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# The slashed assets cannot be the actually slashed amount

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-173
- **Submitter:** Takarez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/173
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-173.md

## Brief Summary

1. The slashed amount can be less than intended amount, the amount can be less more in case of a substantial increase in assets. 2. Meaning, a vault operator can maliciously deposit ( asked validators to) in order to reduced the amount substantially and ask them to withdraw queued their deposits after the actual slashing ( ETH out of node) happened.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_27_group

# Vaults with active slashing that are live long enough time can be bricked

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-70
- **Submitter:** Tointer
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/70
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-70.md

## Brief Summary

If vault will slash users too much times, deposits will slowly stop working. In this state, there exists possibility for malicious actor to brick the vault entirely. Problem Each slashing is removing assets from ERC4626 vault, making each asset convert to more shares. If at the start 1 asset = 1 share, after 50% slashing 1 asset would be equal to 2 shares. Since slashing amounts are usually represented as % of the assets, this ratio is growing exponentially. And, since vaults are not used to distribute rewards, there is only one way in which this ratio can change. As a result, shares `totalSupply` can grow very close to the maximum value of uint256 For example, it would take around 50 90% s...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_54_group

# Anyone can allowlist an asset via `CoreLib`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-218
- **Submitter:** Vancelot
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/218
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-218.md

## Brief Summary

The access control of [allowlistAssets](https://github.com/code-423n4/2024-07-karak/blob/main/src/Core.sol#L85-L91) in [Core](https://github.com/code-423n4/2024-07-karak/blob/main/src/Core.sol) can be bypassed if a user creates a smart contract and calls the function directly from `CoreLib`. This allows anyone to add or remove assets from the `allowlist`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `Vault` initialization is impossible if symbol is non-string

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-242
- **Submitter:** Vancelot
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/242
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-242.md

## Brief Summary

Vaults won't be initalized with a underlying token, whose symbol is a type of `bytes32` instead of `string`, as it would lead to a revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect State Update Order in `startWithdrawl` Function

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-66
- **Submitter:** Zaykov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/66
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-66.md

## Brief Summary

The `startWithdrawl` function in the `NativeVault` contract contains a issue where the state update occurs before checking if the snapshot is expired. This will leat to an inconsistent state if the snapshot is ineed expired, as the function will revert withoud undoing the state change to `nodeOwnerToWithdrawAmount`. This isse could result in incorrect withdrawal limits for users and affect the contract's overall state integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# access conttrol policy for secured pausing and unpausing functions

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-298
- **Submitter:** _1stephen
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/298
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-298.md

## Brief Summary

Impact- by making the pauser functioality ownable , only an authorized user can pause or unpause this contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Mallisious actor can deploy vaults until operator's number of vaults for reaches the maximum.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-204
- **Submitter:** almantare
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/204
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-204.md

## Brief Summary

`Since anyone can deploy vaults for any operator` there is a scenario where an attacker deploys vaults with same `depositToken`(allowed) for an operator until the number of vaults reaches `Constants.MAX_VAULTS_PER_OPERATOR`, after which a vault with a different `depositToken` cannot be added.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# h-03 Vulnerable Admin/Owner Methods Without Emitting Events or Implementing Timelocks arabgodx

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-14
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/14
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-14.md

## Brief Summary

1. several methods within smart contracts that require admin or owner permissions but do not emit events, follow a two-step process with a mandatory time window in between, or implemented a time-lock Vulnerability Detail 1. `changeNodeImplementation` allowed the owner or manager to changing the implementation of all NativeNode, this impact the entire protocol if a malicious or incorrect implementation 2. admin could set an invalid address or a malicious contract as the new node implementation, causing all nodes to malfunction or behave unexpectedly Impact 1. high impact and medium likelihood 2. identified methods have significant control for the protocol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_36_group

# h-04 Withdrawal Function Allowing Arbitrary Fund Transfers

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-17
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/17
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-17.md

## Brief Summary

1. owner of `SlashStore` can withdraw any amount of ETH to any address without a time-lock or a two-step process, posing a risk of unauthorized or malicious fund transfers. Vulnerability Detail 1. `withdraw` function allowed the owner to transfer funds without any restrictions, time-lock, or a event emission, making it vulnerable to the attacks Impact 1. High impact and medium likelihood owasp

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_66_group

# callHookIfInterfaceImplemented should revert when call failed or interface isn't implemented

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-151
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/151
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-151.md

## Brief Summary

when [HookLib.sol::callHookIfInterfaceImplemented](https://github.com/code-423n4/2024-07-karak/blob/main/src/entities/HookLib.sol#L78-L103) is call failed or interface isn't implemented the function return false instead of revert. The issue arise when `ignoreFailure` is set to false.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of __gap Variable

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-159
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/159
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-159.md

## Brief Summary

None of the vaults protocol contracts include a __gap variable. Without this variable, it is not possible to add any new variables to the inherited contracts without causing storage slot issues. Specifically, if variables are added to an inherited contract, the storage slots of all subsequent variables in the contract will shift by the number of variables added. Such a shift would likely break the contract

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_87_group

# non empty proof is required in the verifyInclusionKeccak.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-284
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/284
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-284.md

## Brief Summary

Detailed description of the impact of this finding. The methods verifyInclusionKeccak(proof, root, leaf, index) will always return true if proof.length < 32 (e.g. empty proof) and leaf == root. Although this might be intended behaviour, I see no use case for empty proofs and would require non-empty proofs at the library level. As of now, the user of the library is responsible to enforce non-zero proofs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# not zero implementation will revert in "validateVaultConfigs"

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-356
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/356
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-356.md

## Brief Summary

Detailed description of the impact of this finding. if implementation == address(0) then it will not revert but if implementation is not zero then it will revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# "Possibility of losing funds due to failure to check for address `0`"

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-109
- **Submitter:** black-wolf
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/109
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-109.md

## Brief Summary

Sending Ether to a zero address leads to an irretrievable loss of funds, as there is no way to recover them. This issue can be highly problematic and damaging for users and smart contracts. Financial Insecurity: If a contract sends funds by default without checking the destination address, it can lead to financial misuse and significantly impact the financial security of the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# Slashing possible even though the Vaults are paused

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-102
- **Submitter:** bronze_pickaxe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/102
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-102.md

## Brief Summary

The `Core` can `pause()` and `unpause()` the Vaults by calling: If, for example, `pause()` gets called, any function with this modifier will not work inside `Vault.sol`: Note that, almost all external functions will **not** work while being in a paused state due to the usage of the `whenFunctionNotPaused()` modifier: However, the `whenFunctionNotPaused` is **not** implemented on `Vault.slashAssets()`: This means that slashing can still happen, even in a paused state. This is especially worrisome since all other external functions inside `Vault.sol` will not work. Withdrawals, for example, will not work due to `whenFunctionNotPaused`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ignoring the fail condition of request slashing and finalize slashing methods might prevent time sensitive requests to fail being dealt with

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-328
- **Submitter:** c0pp3rscr3w3r
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/328
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-328.md

## Brief Summary

Time sensitive slashing for the Operator wouldn't be possible for the DSS, as the finalize slashing method has to wait for 2 days before the next request slashing is approved

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing increment for `node.activeValidatorCount` in `NativeVault::validateWithdrawalCredentials()`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-241
- **Submitter:** cholakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/241
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-241.md

## Brief Summary

It is impossible to update the validator's balance in future snapshots as the number of active validators for all native nodes will always remain at 0.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `withdrawalMap[withdrawalKey]` will not get resetted after executing the withdrawal in `NativeVault::finishWithdrawal()`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-244
- **Submitter:** cholakov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/244
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-244.md

## Brief Summary

Due to the incorrect resetting of `withdrawalMap[withdrawalKey]`, a malicious user can repeatedly call [`NativeVault::finishWithdrawal()`](https://github.com/code-423n4/2024-07-karak/blob/main/src/NativeVault.sol#L262) with the same `withdrawalKey` to withdraw all their assets, effectively bypassing the `MIN_WITHDRAWAL_DELAY`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# DSS is still registered to the operator even though the interfaces are not implemented in the DSS or the low level call failed.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-352
- **Submitter:** dhank
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/352
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-352.md

## Brief Summary

DSS is still registered to the operator even though the interfaces are not implemented in the DSS or the low level call failed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Anyone can finish redeem and unstake user funds without they want it

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-56
- **Submitter:** djanerch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/56
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-56.md

## Brief Summary

This issue can lead to early unstaking, causing users to earn fewer rewards.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `ExtSloads` Function Returns Empty Array, Preventing `Querier` Contract from Fetching Vaults Queued for Withdrawal

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-105
- **Submitter:** eta
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/105
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-105.md

## Brief Summary

The `extSloads` function in the `ExtSloads.sol` contract returns an empty array, which causes the `fetchVaultsQueuedForExit` function in the `Querier.sol` contract to receive an empty `results` array when calling `extSloads`. Consequently, this prevents `fetchVaultsQueuedForExit` from returning the vault addresses queued for withdrawal.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Remove `bytes32(0)` Elements before Iteration in `fetchVaultsQueuedForExit`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-106
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/106
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-106.md

## Brief Summary

In the `Querier.sol` contract, the `fetchVaultsQueuedForExit` function first retrieves `slots`, and then iterates through the elements in `slots` (including `bytes32(0)` elements) using the `core.extSloads(slots)` function to get `results`. Afterward, the `bytes32(0)` elements are removed from `results`. This process causes `core.extSloads(slots)` to perform unnecessary iterations over `bytes32(0)` elements.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Type mismatch in the `validateQueuedWithdrawal` function of `VaultLib.sol` contract

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-120
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/120
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-120.md

## Brief Summary

In the `VaultLib.sol` contract, the `validateQueuedWithdrawal` function checks the condition `qdWithdrawal.start + Constants.MIN_WITHDRAWAL_DELAY > block.timestamp`. The issue is that `qdWithdrawal.start` is a `uint96`, while `Constants.MIN_WITHDRAWAL_DELAY` is a `uint256`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Handling of Edge Case in Operator Registration

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-97
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/97
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-97.md

## Brief Summary

The `registerOperatorToDSS()` does not correctly handle this [edge case], leading to unexpected behavior during operator registration. When an operator attempts to register with a DSS that they are already registered with. The current implementation does not check for this scenario, potentially allowing multiple registrations of the same operator to the same DSS. The `Core` contract manages the registration of operators to various DSS instances. The `registerOperatorToDSS()` function ([lines 97-107](https://github.com/code-423n4/2024-07-karak/blob/f5e52fdcb4c20c4318d532a9f08f7876e9afb321/src/Core.sol#L97-L107)) is designed to allow operators to register with a DSS, enabling them to allocate...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_01_group

# DoS Vulnerability in function fetchVaultsQueuedForExit

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-153
- **Submitter:** golomp
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/153
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-153.md

## Brief Summary

An attacker could add a large number of vaults, making the fetchVaultsQueuedForExit function too expensive to execute. If the number of vaults is large, the loops iterating over these arrays (for (uint256 i = 0; i < stakedVaults.length; i++)) can become very expensive in terms of gas. Each additional vault in stakedVaults increases the gas cost of executing the function. If the gas required to execute the function exceeds the block gas limit, the function will fail, causing a DoS condition.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A DSS can selectively revert and block a non-reverting protocol operations affecting the integration with karak protocol

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-266
- **Submitter:** imare
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/266
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-266.md

## Brief Summary

`DSS`s must conform to ERC165 interface call and in doing so enables hooks that are used by the karak protocol for various operations. By reverting on ERC165 operations a DSS can prevent karak protocol operations like slashing, operator removal, or vault validation from working even when karak protocol could safely ignore this hooks reverting.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# Reentrancy Vulnerability in Vault Withdrawal Function

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-87
- **Submitter:** johnthebaptist
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/87
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-87.md

## Brief Summary

Reentrancy vulnerability in the withdrawal process allows malicious actors to drain funds from the vault. This can lead to significant loss of user funds and compromise the entire system's integrity. Attackers can repeatedly withdraw funds before the contract state is updated, potentially emptying the vault.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# finalizedSlashing() should also include whenFunctionNotPaused(Constants.PAUSE_CORE_CANCEL_SLASHING)

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-44
- **Submitter:** lanrebayode77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/44
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-44.md

## Brief Summary

should also include to prevent finalization of a slashing process when veto committee cannot cancel the slashing process if need be. When slashing is requested, it is required that slashing veto window has elapsed, before it can be finalized. This window allows veto committee to cancel the slashing if there is need to, but the function has a specific pause modifier, which means when the function is paused, veto cannot cancel and time keeps counting, if is called after the window despite the cancel function being paused, slashing process will be finalized without giving veto committee the opportunity to cancel.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Pausing a native's node withdraws, will also pause _startSnapshot for that node's owner

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-129
- **Submitter:** max10afternoon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/129
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-129.md

## Brief Summary

Pausing a native's node withdraws, will also pause _startSnapshot for that node's owner. As proposed by the sponsor in the [attack ideas (where to focus for bugs)](): the pause function of the [NativeNode](https://github.com/code-423n4/2024-07-karak/blob/f5e52fdcb4c20c4318d532a9f08f7876e9afb321/src/NativeNode.sol#L17) contract, will also pause the snapshot functionality of the [NativeVault](https://github.com/code-423n4/2024-07-karak/blob/f5e52fdcb4c20c4318d532a9f08f7876e9afb321/src/NativeVault.sol#L25) contract. Also although the manager role (who can call the pause functionalities) is a trusted role in the protocol, this is an unexpected behaviour, meaning that even a "correct" and in goo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# NativeVault: Users can queue a withdraw request, that can be finalized only by them. Making it possible to fron run slashing event with a withdraw

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-137
- **Submitter:** max10afternoon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/137
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-137.md

## Brief Summary

In order to prevent users to front run slashing events with a withdraw, requests to redeem the shares for the underlying assets, have to be queued for 9 days. After the queue time has expired anyone can finalize the withdraw request, making sure that at all time there are no withdraw request that could be exercised, to front run a slash. A malicious user can create a withdraw request that can only be finalized by them, enabling them to have a mature withdraw request ready to be completed at all time. Which can be used to front run slashing events

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# using ownable instead of ownable upgradeable can malfunction onlyowner functions

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-318
- **Submitter:** nikhilx0111
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/318
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-318.md

## Brief Summary

based on context vault.sol is designed to be a upgradeable proxy contract Currently, the implementation uses a non-upgradeable version of the Ownable rather than the upgradeable version In a non-upgradeable Ownable library, the deployer is set as the default owner in the constructor. However, for proxy-based upgradeable contracts, constructors cannot be used. As a result, when the contract is deployed as a proxy, there will be no designated owner note the same issue is present in nativenode.sol and slashinghandler.sol impact As a result, all the onlyOwner functions will be inaccessible.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# manipulated call

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-179
- **Submitter:** nnamdi0482
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/179
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-179.md

## Brief Summary

Detailed description of the impact of this finding. Both destination and calldata could be manipulated The call could be fully manipulated (arbitrary call) through OperatorContract.executeCall(address,bytes)

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# initializer could be frontrun

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-237
- **Submitter:** nnamdi0482
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/237
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-237.md

## Brief Summary

Detailed description of the impact of this finding. initializers could be front-run, allowing an attacker to either set their own values, take ownership of the contract, and in the best case forcing a re-deployment

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_102_group

# Gas optimisation

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-310
- **Submitter:** obingo76
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/310
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-310.md

## Brief Summary

The current implementation of the Merkle library contains several gas inefficiencies: Repeated array length access in loops Suboptimal increment operations Long error messages in require statements Unnecessary memory usage These inefficiencies lead to higher gas costs, which can be particularly significant in functions that are called frequently or process large amounts of data.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Manager EOA may steal assets meant for slashing

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-358
- **Submitter:** perseus
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/358
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-358.md

## Brief Summary

All assets which are meant to be slashed by DSS can be misused by the rogue or compromised Manager. In addition, attacker which has previously compromised Manager EOA and has this capability can use intentionally to break security rules/invariants of particular DSS.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Possible DOS (out-of-gas) on loops.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-231
- **Submitter:** peyodp
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/231
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-231.md

## Brief Summary

It is possible to get an out-of-gas issue while iterating the for loop. Please take a look at [this link](https://github.com/wissalHaji/solidity-coding-advices/blob/master/best-practices/be-careful-with-loops.md).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# User with `MANAGER_ROLE` is able to call `NativeNode::changeNodeImplementation`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-122
- **Submitter:** radin100
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/122
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-122.md

## Brief Summary

`MANAGER_ROLE` role gives more rights than intended

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# Missing checks for `address(0)` when assigning values to address state variables

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-162
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/162
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-162.md

## Brief Summary

The Querier contract has a constructor that initializes the core state variable with a provided address. However, the constructor does not include a check to prevent the initialization with a zero address (address(0)). Vulnerable Code: constructor(address coreAddress) { core = ICore(coreAddress); } Issue: If the coreAddress is set to address(0), it may lead to unexpected behavior and security vulnerabilities. In many cases, smart contracts use zero addresses as placeholders or indicators of invalid states, which can lead to issues such as: Unintended Behavior: Functions that rely on the core address might fail or behave unexpectedly when it is zero. Security Risks: Contracts interacting wit...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# isContract could DOS registerDss() for specific DSS

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-305
- **Submitter:** stanchev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/305
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-305.md

## Brief Summary

Caller can register DSS through the registerDss() function, however DSS is checked with the isContract modifier. The isContract function that uses EXTCODESIZE was discovered to be bypassable. The function will return false if it is invoked from a contract’s constructor, because the contract has not been deployed yet.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Native Node Implementation address should be whitelisted

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-127
- **Submitter:** stuart_the_minion
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/127
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-127.md

## Brief Summary

Malicious operators may deploy the native vaults with their own native node implementation addresses. With these customized native nodes, they can run bad actions, such as stealing node owners' withdrawn ETHs that must be sent to the owners' intended addresses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Insufficient timestamp validation in `validateWithdrawalCredentials` will lead to erroneous user balances, allowing validators to steal rewards

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-220
- **Submitter:** trachev
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/220
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-220.md

## Brief Summary

The timestamp validation of `beaconStateRootProof.timestamp` in `validateWithdrawalCredentials` is insufficient and may cause validators to be minted more shares than intended.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# `HookLib` gas validation is incorrect, causing incorrect reverts in `callHookIfInterfaceImplemented`

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-225
- **Submitter:** trachev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/225
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-225.md

## Brief Summary

In `HookLib.sol` `callHookIfInterfaceImplemented` and `callHook` validate that there is enough gas for the function call to be successful. The issue is that the functions make wrong calculations, which may cause the execution to revert even though, it would have been successful, or not revert when it should have failed. In the first case this may prevent users from updating their stake in DSS, initiating and finalizing slashing and configuring operators' registration to DSS. In the second case this will allow malicious operators to bypass the validation, enabling them to execute the [H-02] attack from the Karak Pro League audit: https://code4rena.com/reports/2024-06-karak-pro-league#h-02-ma...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_09_group

# No checks present in,`startRedeem` function to ensure `beneficiary` actually has the amount of `shares` entered, can lead to silent failure of the function

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-208
- **Submitter:** unRekt
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/208
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-208.md

## Brief Summary

The `startRedeem` of `Vault.sol` doesn't check if `beneficiary` has the amount of `shares` they are trying to redeem which can lead to silent failure of transaction in case, amount of `shares` entered is not present with `beneficiary`. The `startRedeem` function has `address(0)` and `shares == 0` check to ensure `beneficiary` can't be 0 address and amount of shares entered can't be 0. Here - But it doesn't check if the beneficiary has the amount of shares they are trying to redeem. In case the amount of shares entered are not present with beneficiary, `transferFrom` will fail - As it returns a `boolean` according to [OZ docs](https://docs.openzeppelin.com/contracts/4.x/api/token/erc20#IERC2...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The `if (totalAssetsToSlash > self.totalAssets)` check of `NativeVault::slashAssets` function can empty asset pool if wrong amount is entered even by accident.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-209
- **Submitter:** unRekt
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/209
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-209.md

## Brief Summary

The `if (totalAssetsToSlash > self.totalAssets)` check can empty asset pool if wrong amount is entered even by accident. In `slashAssets` function of `NativeVault` contract, the check performed - So, if the owner mistakenly inputs wrong value of `totalAssetsToSlash` greater than `self.totalAssets` then as per the next line - `self.totalAssets ` =0, as `self.totalAssets=self.totalAssets- totalAssetsToSlash` and `totalAssetsToSlash` = `self.totalAssets`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Check for `minSharesOut` is done after assets are already deposited, breaks functionality.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-210
- **Submitter:** unRekt
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/210
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-210.md

## Brief Summary

In `deposit` function of `Vault.sol` check for `minSharesOut` is done after the assets are already transferred, which makes the check useless as the assets are already deposited In `deposit` function of `Vault.sol` In this line `shares = super.deposit(assets, to);` the shares are already deposited. Then the check is performed in the next line ` if (shares < minSharesOut) revert NotEnoughShares();` The check on `minSharesOut` has no value, and one can deposit if `0<shares<minSharesOut`;

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing proper modifier for the functtion [Core::registerOperatorToDSS] allowing anyone to be able to call this function, registering an Operator against their will

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-258
- **Submitter:** willycode20
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/258
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-258.md

## Brief Summary

`operator = msg.sender` This line simply assigns the address that initiated the transaction (i.e., msg.sender) to the variable named operator. It's a way to identify the caller of the function within the function's scope. Whether this caller is considered an "operator" and granted the privileges or responsibilities associated with being an operator depends on additional logic within the function or the contract. However, the additional logic to implement this intende check is not provided in he execution of the function. Malicious actors could exploit the lack of checks to register an operators to a DSS against the operator will. Also, this actor can register this particular operator to mul...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Discrepancy betweeen expectedNewAddr and actual(vault) will always be a thing since expectedNewAddr is calculated wrongly, therefore creation of vault using `deployVaults` which eventually calls the internal function `createVault` likely to revert.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-286
- **Submitter:** willycode20
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/286
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-286.md

## Brief Summary

Observing the `createVault` function below which is called from the `deployVaults` function. If the condition inside the if statement evaluates to true, meaning the addresses do not match, the function will execute the revert statement. In the context of the `LibClone.predictDeterministicAddressERC1967BeaconProxy` function call, passing `address(this)` twice as arguments suggests a misunderstanding or miscommunication in the documentation or usage of the library function. Typically, in Ethereum smart contracts, especially those involving proxy patterns or similar architectural designs, the beacon address and the deployer address serve distinct roles and should not be confused with each othe...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# [H-1] Lack of checks allows a `DSS` contract to register as `Operator` to itself

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-138
- **Submitter:** xKeywordx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/138
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-138.md

## Brief Summary

**Description:** According to the contest page "`Operators` perform tasks for the `DSS` in exchange for rewards, and the `DSS` has the ability to `slash` the funds that `Operators` have delegated". Furthermore, a `DSS` has the ability to `Jail` an `Operator` if they have "valid reasons" not to accept a certain `Operator`. I also went through the official docs of Karak and studied the architecture of the protocol of both [v1](https://docs.karak.network/protocol/v1/overview) and [v2](https://docs.karak.network/protocol/v2/dss) and I didn't find any instance where the sponsor mentions that this is deemed as an intended design decision. Judging based on the architecture of the protocol, the nat...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# [M-2] `NativeVault::finishWithdrawal` updates state after external call and opens up the possibility for cross-function reentrancy calls.

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-150
- **Submitter:** xKeywordx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/150
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-150.md

## Brief Summary

[M-2] `NativeVault::finishWithdrawal` updates the state after an external call and opens up the possibility for cross-function reentrancy calls. **Description:** The `NativeVault` is supposed to work with native Ether, see Karak docs [here](https://docs.karak.network/protocol/v2/native-restaking#full-withdrawals). This implies that there will be instances where the `receiver` of the native ETH transfer from the `finishWithdrawal(...)` function call will be a smart contract that can have a `receive()` function which gets to execute arbitrary logic. The issue arises from the fact that the `NativeVault::finishWithdrawal` function updates the `node.withdrawableCreditedNodeETH` variable and dele...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `NativeNode` does not have a receive function

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Submission:** V-247
- **Submitter:** y4y
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-karak-validation/issues/247
- **Source snapshot:** competitions/2024-07-karak-restaking/submissions/raw/V-247.md

## Brief Summary

As no Ethers can be sent to the `NativeNode` contract, the `withdraw` function does not work.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group
