# Rejected Primary Findings: Axelar Network

# `rotateSigners` can get bricked, also stopping any message approvals

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-24
- **Submitter:** 0x3b
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/24
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-24.md

## Brief Summary

[_validateSignatures](https://github.com/code-423n4/2024-08-axelar-network/blob/main/axelar-gmp-sdk-solidity/contracts/governance/BaseWeightedMultisig.sol#L192) has a critical revert that will revert every time and even prevent [rotateSigners](https://github.com/code-423n4/2024-08-axelar-network/blob/main/axelar-gmp-sdk-solidity/contracts/gateway/AxelarAmplifierGateway.sol#L96) to be called. This will brick the whole signers mechanic, stopping singers from approving messages or being rotated.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# Approve races are still possible

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-29
- **Submitter:** 0x3b
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/29
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-29.md

## Brief Summary

Approved users can revert [decreaseAllowance](https://github.com/code-423n4/2024-08-axelar-network/blob/main/interchain-token-service/contracts/interchain-token/ERC20.sol#L123-L126) and prevent their approvers from removing their allowance, essentially causing an approval sandwich.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Passing an arbitrary from address to transferFrom (or safeTransferFrom) can lead to loss of funds, because anyone can transfer tokens from the from address if an approval is made.

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-122
- **Submitter:** 0xflamingo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/122
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-122.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_108_group

# Use abi.encode() instead which will pad items to 32 bytes, which will prevent hash collisions (e.g. abi.encodePacked(0x123,0x456) => 0x123456 => abi.encodePacked(0x1,0x23456), but abi.encode(0x123,0x456) => 0x0...1230...456). Unless there is a compelling reason, abi.encode should be preferred. If there is only one argument to abi.encodePacked() it can often be cast to bytes() or bytes32() instead. If all arguments are strings and or bytes, bytes.concat() should be used instead.

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-123
- **Submitter:** 0xflamingo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/123
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-123.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Instead of separating the logic into a separate function, consider inlining the logic into the calling function. This can reduce the number of function calls and improve readability. Recommendation: No recommendations provided.

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-125
- **Submitter:** 0xflamingo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/125
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-125.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# ITS Hub does not implement AxelarExecutableMsg::Execute interface

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-142
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/142
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-142.md

## Brief Summary

In `execute.rs` of Axelarnet Gateway, it's said that apps are required to implement `AxelarExecutableMsg::Execute` interface. The problem is that in ITS Hub there is no such interface explicitly exposed. This goes in deviation with the spec and causes the problem when executing the message.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# onlyOwner check not working in InterchainTokenFactory.sol and InterchainTokenService.sol contracts

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-93
- **Submitter:** Inspecktor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/93
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-93.md

## Brief Summary

The InterchainTokenFactory.sol and InterchainTokenService.sol contracts use the onlyOwner modifier: This modifier allows you to check that the function can only be called by the specified owner in the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Role-Based Access Control in BaseAmplifierGateway.sol

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-274
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/274
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-274.md

## Brief Summary

The BaseAmplifierGateway contract does not implement role-based access control (RBAC) on its critical functions related to message approval, validation, and execution. This vulnerability allows anyone (including malicious attackers) to interact directly with these functions and potentially issue harmful or compromising the contract's integrity. In this contract, the `callContract`, `validateMessage`, and `_approveMessage` functions are all unprotected, permitting any address to execute these actions without control. For example the function `_approveMessage(Message calldata message)` allows the updating of the state variable `messages` from the `BaseAmplifierGatewayStorage` struct. This fun...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_94_group

# BaseWeightedMultisig.sol :: Missing nonce in `_validateSignatures()` allows signature replay

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-278
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/278
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-278.md

## Brief Summary

_validateSignatures() doesn't include a nonce, thereby allowing signatures to be replayed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Potential DoS By Block Gas Limit in BaseWeightedMultisig.sol

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-280
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/280
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-280.md

## Brief Summary

The `_validateSignatures` function might be subject to a Denial of Service (DoS) attack by the Block Gas Limit. As this function requires both signers and signatures to be sorted for operation, a large number of signers and signatures could make the `for-loop` excessively gas-consuming, resulting in exceeding the block gas limit, making it impractical or impossible in some cases.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_24_group

# Unprotected Function Calls in InterchainTokenFactory.sol allow reentrancy

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-283
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/283
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-283.md

## Brief Summary

The function calls in this contract are not protected against reentrancy attacks. Reentrancy attacks occur when, during the execution of a function, the control is passed to an external contract, which in turn calls the function again, changing data that the function relies upon. In the function `deployInterchainToken`, tokens are first minted and then the minter's address is transferred. The function `token.mint` and `token.transferMintership` are external calls that can be a potential target of reentrancy risk.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Lack of Error Handling in Data Decoding

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-39
- **Submitter:** Jean_Pierre_Polnaref
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/39
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-39.md

## Brief Summary

The absence of error handling in the decoding process can have significant consequences: Contract Instability: If the input data format is incorrect or maliciously crafted, the contract may revert unexpectedly. This can disrupt normal contract operations, leading to potential downtime or instability. Security Risks: Malicious actors could exploit this vulnerability by crafting input data that causes the contract to behave unpredictably or fail, potentially resulting in a denial-of-service attack. The contract's reliability is compromised, which may expose it to further security risks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Gas Optimizations

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-40
- **Submitter:** Jean_Pierre_Polnaref
- **Claimed severity:** Gas
- **Final severity:** Gas
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/40
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-40.md

## Brief Summary

See the markdown file with the details of this report [here](https://github.com/code-423n4/2024-08-axelar-network-validation/blob/main/data/Jean_Pierre_Polnaref-G.md).

## Rejection Reason

GitHub validation labels: bug, G (Gas Optimization), insufficient quality report, :robot:_primary

# addFlowIn used in givetoken and addFlowOut used in takeToken functions would record account of tokens flowing in and flowing out

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-47
- **Submitter:** LogBytes
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/47
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-47.md

## Brief Summary

There would be incorrect accounting of tokens when tokens are given out and taken in.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Potential Storage Slot Collision in Inheritance Chain Risks Data Corruption and Unpredictable Contract Behavior

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-224
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/224
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-224.md

## Brief Summary

InterchainMultisig inherits from BaseWeightedMultisig. This means it inherits all the state variables and functions, including the storage layout defined in the base contract. InterchainMultisig introduces its own storage struct, InterchainMultisigStorage, and uses a similar pattern to calculate its dedicated storage slot (INTERCHAIN_MULTISIG_SLOT). Here's how both BaseWeightedMultisig and InterchainMultisig calculate their storage slots. - BaseWeightedMultisig uses keccak256('BaseWeightedMultisig.Slot') - 1. - InterchainMultisig uses keccak256('InterchainMultisig.Slot') - 1. This approach only works well as long as the contract is not inherited and extended by other contracts that also emp...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Loss of Funds due to Lack of Input Validation

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-247
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/247
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-247.md

## Brief Summary

The callContractWithToken and callContract functions do not perform any validation on the destinationAddress parameter. This allows a malicious user to provide an incorrect or malicious contract address, potentially leading to the irreversible loss of transferred tokens or other assets. Code Impact Loss of Funds: If a user mistakenly or intentionally provides an incorrect destinationAddress, the cross-chain call will interact with an unintended contract. If this contract is malicious or has unexpected behavior, it could lead to the loss of the tokens transferred in the callContractWithToken function. Scenario A user, either by mistake or with malicious intent, provides an incorrect destinat...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Race Condition in ERC20Permit Implementation Allows Double-Spending Through Replay Attacks

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-250
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/250
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-250.md

## Brief Summary

The `permit` function is designed to allow users to approve token spending without sending a transaction, using a signed message instead. This is part of the ERC20Permit standard (EIP-2612). The function increments the nonce after using it in the message digest calculation: This creates a race condition where multiple valid signatures for the same nonce can be created and used.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# Insufficient Enum Validation in TokenManager Deployment Allows Creation of Undefined Types

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-251
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/251
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-251.md

## Brief Summary

The `deployTokenManager` function in the InterchainTokenService contract is used to deploy custom TokenManagers, either locally or on remote chains. The function does not fully validate the `tokenManagerType` parameter against all possible values of the TokenManagerType enum. While the function checks for the NATIVE_INTERCHAIN_TOKEN type, it doesn't ensure that the provided type is within the valid range of the TokenManagerType enum. This could potentially allow the deployment of TokenManagers with undefined types.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_79_group

# Uninitialized State Variable

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-244
- **Submitter:** Pewbhai
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/244
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-244.md

## Brief Summary

The presence of an uninitialized state variable, specifically the 'nameHash' in the provided 'ERC20Permit' contract, poses significant risks to the smart contract's functionality and integrity. If the 'nameHash' variable is accessed before it has been expressly initialized, it will contain its default value of '0x0'.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Risk of Trapped Ether in Core Contracts Due to Lack of Recovery Mechanism

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-135
- **Submitter:** PolarizedLight
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/135
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-135.md

## Brief Summary

While the core contracts of the Axelar protocol are designed to use received Ether for specific purposes (primarily gas payments for cross-chain operations), the lack of a recovery mechanism presents a potential risk. In the event of failed transactions, protocol changes, or unforeseen circumstances, there's a possibility that Ether could become trapped in these contracts without a way to recover it. This could lead to: 1. Potential loss of funds in edge cases where Ether is not fully utilized as intended. 2. Reduced protocol flexibility in handling unexpected scenarios or upgrades. 3. Possible complications in future protocol migrations or upgrades where residual Ether needs to be addresse...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of ERC-1271 Compliance in AxelarAmplifierGateway, BaseWeighted MultiSig and ERC20Permit.

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-136
- **Submitter:** PolarizedLight
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/136
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-136.md

## Brief Summary

Absence of ERC-1271 Support Limits Compatibility with Smart Contract Wallets Issue Description The AxelarAmplifierGateway, BaseWeightedMultisig, and ERC20Permit contracts do not implement the ERC-1271 standard for signature validation. While these contracts offer sophisticated multi-signature and permit functionalities through custom implementations, the absence of ERC-1271 support may significantly limit interoperability with smart contract wallets and certain DeFi protocols that expect ERC-1271 compliance. Impact The lack of ERC-1271 support has the following impacts: 1. Limited Compatibility: Smart contract wallets relying on ERC-1271 for signature validation cannot interact directly wit...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Multicall being marked as payable is seriously dangerous

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-178
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/178
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-178.md

## Brief Summary

Multicall being marked as payable can lead to 2 impact: - user pay more eth than expected those exceed eth stuck in contract - the msg.value is used many times

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# unchecked output of the ECDSA recover function.

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-211
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/211
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-211.md

## Brief Summary

Detailed description of the impact of this finding. ECDSA.recover function return address(0) if the signature provided is invalid.This can result in unintended behaviour.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# acceptOperatorship can be called by any operator

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-240
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/240
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-240.md

## Brief Summary

Detailed description of the impact of this finding. There is no check for the fromOperator is proposed operator or not.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# CREATE3 is not available in the ZkSync Era

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-242
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/242
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-242.md

## Brief Summary

Detailed description of the impact of this finding. As we are deploying into different chains but zksync does not support the create3.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No check on receiver address

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-276
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/276
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-276.md

## Brief Summary

Detailed description of the impact of this finding. There is no verification on recipient address as it can be msg.sender.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Creation of Interchain Tokens is susceptible for Denial of Service

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-59
- **Submitter:** bronze_pickaxe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/59
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-59.md

## Brief Summary

Due to the usage of only a salt when deploying an Interchain Token, malicious users can front-run this creation, leading to Denial of Service. Description `InterchainTokenDeployer.deployInterchainToken()` handles the deployment of Interchain Tokes: The problem here is that, for creation of the Interchain Token, only these two parameters are used: `bytecode` will always be static since it's calculated inside the `deployInterchainToken()` function. `salt` can be provided by the entity calling `deployInterchainToken()`. This opens up the ability for malicious users to DoS the creation of Interchain Tokens. All that the malicious users have to do is copy the `salt` and use their own address as...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# BaseAmplifierGateway contract has Invalid Type for bytes.concat Function named messageToCommandId

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-110
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/110
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-110.md

## Brief Summary

Detailed description of the impact of this finding. The use of incorrect data types in the bytes.concat function leads to a compilation error, preventing the contract from being compiled and deployed. This error stops the contract from functioning as intended and could cause significant issues if the function is critical for contract logic.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# InterchainTokenService contract has Incorrect Type Conversion from bytes calldata slice to bytes4 Leading to Fatal Compilation Error

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-113
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/113
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-113.md

## Brief Summary

Detailed description of the impact of this finding. The code attempts to decode the first 4 bytes from a bytes calldata slice into a bytes4 and then convert that bytes4 to a uint32. However, Solidity does not allow direct conversion from a bytes calldata slice to bytes4, which results in a fatal compilation error. This issue prevents the contract from compiling, leading to deployment failure. This not only stops the smart contract from functioning but also halts any further development and testing. In a production environment, this could delay the release of critical updates or fixes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# TokenManagerProxy contract missing override specifier in Overriding Public State Variables and Functions

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-126
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/126
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-126.md

## Brief Summary

Detailed description of the impact of this finding. ***Prerequisites: Update all contracts' pragma versions from ^0.8.0 to ^0.8.4 in order for the contracts to subsequently compile after the overrides are implemented.*** The TokenManagerProxy contract contains a function getImplementationTypeAndTokenAddress that overrides a function declared in the ITokenManagerProxy interface. However, the overriding function is missing the override keyword, which is required in Solidity 0.8.0 and later to explicitly denote that a function overrides another function in a base contract or interface. This issue is causing compilation errors in this contract. The TokenManagerProxy contract declares public sta...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_78_group

# ERC20Permit contract has Block Timestamp Manipulation Risk

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-129
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/129
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-129.md

## Brief Summary

Detailed description of the impact of this finding. ***Prerequisites:*** 1. For all contracts change pragma version from ^0.8.0 to ^0.8.4 in order for contract to compile successfully and subsequently run successfully after overrides keywords are added where necessary. 2. Add override keyword to the functions in contracts that are also in the parents or interfaces. ***ERC20Permit contract has Block Timestamp Manipulation Risk.*** Mythril Log: Contract: ERC20Permit Function Name: permit(address issuer, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) PC Address: 1477 Estimated Gas Usage: 1605 - 1700 Issue Type: Time Manipulation The permit function of the ERC2...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_39_group

# Lack of Zero Address Check in transferOperatorship Function in the AxelarAmplifierGateway contract

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-22
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/22
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-22.md

## Brief Summary

Detailed description of the impact of this finding. The transferOperatorship function allows the current operator or owner to transfer the operator role to a new address. However, the transferOperatorship function does not verify if the newOperator is not a zero address (address(0)). Assigning the zero address to the operator role can result in the loss of control over the operator-specific functions, potentially leading to governance issues or other critical operational failures.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_23_group

# Allowance Underrun in _spendAllowance Function within InterchainToken contract

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-26
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/26
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-26.md

## Brief Summary

Detailed description of the impact of this finding. ***Prerequisites:*** 1. For all contracts change pragma version from ^0.8.0 to ^0.8.4 in order for contract to compile successfully and subsequently run successfully after overrides keywords are added where necessary. 2. Add override keyword to the functions in contracts that are also in the parents or interfaces. Allowance Under-run in _spendAllowance Function within InterchainToken contract The _spendAllowance function in the InterchainToken contract contains a potential arithmetic error that can lead to incorrect allowance handling. Specifically, the function subtracts the amount from _allowance without ensuring that amount is less than...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_35_group

# Incorrect Solidity Pragma Version Used in the BaseWeightedMultisig contract

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-88
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/88
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-88.md

## Brief Summary

Detailed description of the impact of this finding. The contract is configured with the Solidity pragma version ^0.8.0, which may lead to potential issues, including compatibility problems and missed compiler optimisations available in later versions of the 0.8.x series. Specifically, certain features and syntax, such as the improved handling of revert statements with custom errors, require a compiler version of ^0.8.4 or higher. The error encountered in the contract: This error is related to the use of custom errors in Solidity, which are properly supported starting from version 0.8.4. Thus, upgrading the Solidity version to ^0.8.4 resolves this issue.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# `AxelarAmplifierGateway::rotateSigners` missing repeat signer set validation

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-203
- **Submitter:** fibonacci
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/203
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-203.md

## Brief Summary

`AxelarAmplifierGateway::rotateSigners` implementations assume that repeat signer sets are validated in the `BaseWeightedMultisig::_rotateSigners`. However, this assumption is incorrect, and the validation should actually be performed on the caller (`AxelarAmplifierGateway`) side. Impact There is currently no validation for repeated signer sets. It is possible to set the same set by changing the nonce. A group of malicious signers can set themselves multiple times, bypassing `previousSignersRetention` and gaining control over the gateway.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_75_group

# `InterchainTokenFactory` and `InterchainTokenService` lack of `contractId` validation during upgrade

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-204
- **Submitter:** fibonacci
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/204
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-204.md

## Brief Summary

`InterchainTokenFactory` and `InterchainTokenService` are implemented `IContractIdentifier:: contractId`. This identifier is used during the contract upgrade process to ensure that the new implementation is valid. `InterchainProxy` is used as a proxy for both factory and service implementations and does not set `contractId`. Therefore, `contractId` has a default `Proxy` implementation - `bytes32(0)`, and validation during upgrade is skipped. Impact The `contractId` from `InterchainTokenFactory` and `InterchainTokenService` implementations is unused. `InterchainProxy` can be upgraded to an invalid implementation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Unrestricted Access to `acceptMintership` Function in `Minter` Contract

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-210
- **Submitter:** hunter_w3b
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/210
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-210.md

## Brief Summary

The `acceptMintership` function in the `Minter` contract lacks any access control, which allows any external account to call this function and potentially become the new minter. This issue contrasts with other functions in the contract that use the `onlyRole` modifier to ensure only authorized users can execute them. The absence of such a restriction in the `acceptMintership` function poses a significant risk, as it allows unauthorized users to take control of the contract's minting privileges.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Replay Vulnerability in Trusted Address System

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-286
- **Submitter:** hunter_w3b
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/286
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-286.md

## Brief Summary

The contract's handling of trusted addresses is vulnerable to replay attacks due to the lack of message nonce or timestamp validation. An attacker could potentially intercept a valid message from a trusted address and replay it at a later time or to a different destination chain, leading to unauthorized actions. An attacker could trigger unauthorized actions by replaying messages, such as transferring funds, updating contract state, or interacting with external contracts.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_82_group

# Message Verification Assumptions

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-6
- **Submitter:** implabinash
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/6
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-6.md

## Brief Summary

The comment "because the messages came from the router, we can assume they are already verified" could lead to security complacency, where assumptions about input verification are not actually enforced.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_81_group

# To mitigate this risk, implement validation checks for the address before saving it to the storage.

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-67
- **Submitter:** lee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/67
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-67.md

## Brief Summary

The primary cause of these issues lies in the error handling strategy and the match statement's completeness. The change_context_lazy function is used, but it may not provide sufficient context for debugging. Additionally, the absence of a catch-all case in the match statement can lead to unhandled message types.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Permit doesn't work with DAI because the DAI token's permit signature is different.

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-176
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/176
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-176.md

## Brief Summary

The protocol is expected to work with all types of ERC20 tokens. | Question | Answer | | ---------- | ---------- | | ERC20 used by the protocol | Any (all possible ERC20s)| The protocol aims to be compatible with all types of ERC20 tokens such as dai, but the DAI token's permit signature is different. From the contract at address 0x6B175474E89094C44Da98b954EedeAC495271d0F, we see the permit function: Due to the missing nonce field, DAI, a token which allows permit based interactions, cannot be used with signed messages as an interchain token in the axelar network. Due to the wrong parameters, the permit transactions will revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Premature Native Token Transfer in Contract Deployment

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-277
- **Submitter:** mansa11
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/277
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-277.md

## Brief Summary

The deploy function in the Deployer contract attempts to transfer native tokens to the contract address before the contract is actually deployed, leading to potential loss of funds and deployment failures. Description In the the `Deployer::deploy`, it tries to transfer native tokens (ETH) to the contract address that is about to be deployed. However, this transfer occurs before the actual deployment takes place which means the transfer would fail leading to potential loss of funds The problematic code is: This sequence of operations is incorrect because it tries to send funds to an address that doesn't yet contain a contract. As a result, the transfer will fail, potentially causing the enti...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Access Control and Unbounded Execution in `InterchainMultisig::executeCalls`

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-285
- **Submitter:** mansa11
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/285
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-285.md

## Brief Summary

According to the Natspec, The `executeCalls` function in the `InterchainMultisig` contract is meant to be accessed controlled, However the current implementation lacks the required `onlySigners` modifier, and it also allows execution of unbounded calls with potentially malicious `callData` to a determined `target` address specified by an attacker. Natspec: Description The `executeCalls` function is missing the `onlySigners` modifier as specified in the Natspec comment. This oversight allows any external actor to execute calls, bypassing the intended access control. Additionally, the function permits execution of an unbounded number of calls with arbitrary array length not to mention an atta...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# No matching or checking in `match_gateway` function

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-292
- **Submitter:** minato7namikazi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/292
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-292.md

## Brief Summary

The `match_gateway` function has a logic bug: The bug is that this function doesn't actually perform any matching or checking. It simply loads the gateway address from the config and returns it, regardless of the `ExecuteMsg` passed in. This means that all messages will be considered as coming from the gateway, even if they aren't. The function should be comparing the sender's address (which isn't even passed to this function) with the gateway address from the config. Additionally, the `ExecuteMsg` parameter is unused (denoted by `_`), so the function isn't considering the type of message at all, which it probably should be doing to enforce proper permissions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# fn `route_incoming_messages` doesn't check if message has been approved or is ready

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-294
- **Submitter:** minato7namikazi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/294
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-294.md

## Brief Summary

The logic bug in the `route_incoming_messages` function is in the message verification process. The function checks if each incoming message exists in storage and matches the stored message, but it does not actually verify that the message has been approved or is ready for routing. Specifically: 1. It only checks if the message exists and matches, but not its status. 2. There's no check to ensure the message has been properly verified before routing. 3. It allows routing of messages that may still be pending verification. To fix this, the function should: 1. Check the status of each message (e.g., verified, pending, rejected). 2. Only allow routing of messages that have been fully verified...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# invallid validation totalweight can be equal to threshold which threshold is equal to 0

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-170
- **Submitter:** nnamdi0482
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/170
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-170.md

## Brief Summary

Detailed description of the impact of this finding. the check will not revert if totalWeight is equal to threshold

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No access control in ITS TokenHandler.sol giveToken and takeToken

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-154
- **Submitter:** peanuts
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/154
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-154.md

## Brief Summary

Anybody can call TokenHandler.giveToken() for example to mint tokens for free.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_41_group

# Users can bypass ITS interchain transfer by directly calling `callContract()` in GatewayCaller.sol

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-155
- **Submitter:** peanuts
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/155
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-155.md

## Brief Summary

Users can mint free tokens on the destination chain.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_37_group

# The axelarnet gateway only routes one message per chain at a time

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-76
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/76
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-76.md

## Brief Summary

The axelarnet gateway will not be feasible as a message parser.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# The message can't be executed again once the message can't be executed successfully

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-187
- **Submitter:** pks_
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/187
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-187.md

## Brief Summary

When cosmwasm execute users payload, the call trace: `contract#execute -> execute#execute`: Once the `execute#execute` called, the message status updated firstly: As we can see, the message status is updated to `Executed` and persistent the status in the storage. But if anything wrong after the message status updated, such that `config.chain_name != msg.destination_chain`, which the `msg` is passed by the users, the message status can't be reverted to previous status. You can also see the the detail [here](https://docs.scrt.network/secret-network-documentation/development/development-concepts/secret-contract-fundamentals/secret-contract-cosmwasm-framework/contract-components/storage): So if...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Cross-chain tokens transactions can be replayed after the chain hard-fork

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Submission:** V-188
- **Submitter:** pks_
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-axelar-network-validation/issues/188
- **Source snapshot:** competitions/2024-08-axelar-network/submissions/raw/V-188.md

## Brief Summary

When protocol transfer tokens with calling contract to the destination chain, the call trace `InterchainTokenService.interchainTransfer -> InterchainTokenService._transmitInterchainTransfer -> InterchainTokenService._callContractWithToken`: However, once the chain hard-fork, any users still can transfer the tokens to the destination chain from both source chain and the hard-fork chain without any verification, so that the users can double their tokens in the destination chain. Although the possibility is low, but the effect is enormous. See omni bridge [vulnerability](https://blocksecteam.medium.com/reveal-the-message-replay-attacks-on-ethereumpow-64e4feee991c) when Ethereum’s transition to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_62_group
