# Rejected Primary Findings: Chakra

# The cuurent implementation of the bridge is incompatible with Starknet's unique L1 <-> L2 messaging mechanism

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-641
- **Submitter:** 0x18a6
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/641
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-641.md

## Brief Summary

The current implementation of the bridge is incompatible with Starknet's unique L1 <-> L2 messaging mechanism. This incompatibility renders the bridge non-functional on Starknet.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Callback won't be processed if the mode is MintBurn because of incorrect use of _burnFrom

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-247
- **Submitter:** 0xAsen
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/247
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-247.md

## Brief Summary

Callback won't be able to be processed if the mode is MintBurn because of an incorrect use of _burnFrom. This is because when the mode is `MintBurn`, the contract will try to burn tokens from itself via `_burnFrom`. However, even though the contract burns tokens from itself, it still needs allowance.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Malicious managers can add and remove validators allowing them to validate malicious transactions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-460
- **Submitter:** 0xAsen
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/460
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-460.md

## Brief Summary

Malicious managers can add and remove validators allowing them to set themselves as validators and validate malicious transactions. The issue is present in both the cairo and solidity part. The manager role is NOT trusted according to the contest description.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_126_group

# Protocol assume `from_handler` will be fit in `felt252`, this may cause unintended behavior

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-671
- **Submitter:** 0xDemon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/671
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-671.md

## Brief Summary

Protocol assume `from_handler` will be fit in `felt252`, this may cause unintended behaviors, i.e `from_handler` higher than `felt252` this may cause overflow and signature verification failure

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# misspelling of the decode_transfer

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-117
- **Submitter:** 0xEllipticCurve
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/117
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-117.md

## Brief Summary

Inside the function of the ChakraSettlementHandler.sol contract, a functionality is written incorrectly if (payload_type == PayloadType.ERC20) { // Cross chain transfer { // Decode transfer payload ERC20TransferPayload memory transfer_payload = codec .deocde_transfer(msg_payload); <------

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_122_group

# Immediate Effect of Admin Actions on Validator and Handler Requirements Compromises Cross-Chain Transaction Integrity

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-461
- **Submitter:** 0xNirix
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/461
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-461.md

## Brief Summary

The current implementation of the cross-chain settlement system allows for immediate changes like to validator and handler configuration without considering in-flight transactions or synchronization across different blockchains. This can lead to various issues, including unexpected transaction failures and temporarily stuck funds. This issue is present in both solidity and cairo code. This immediate effect fails to account for: **In-flight transactions**: Cross-chain transactions that are already in progress when the changes are made may fail unexpectedly if they no longer meet the new validation or handler requirements. **Cross-chain synchronization**: Different blockchains may process the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing Access Control Initialization

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-612
- **Submitter:** 0xabhay
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/612
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-612.md

## Brief Summary

This bug pertains to the absence of the `__AccessControl_init` in the `BaseSettlement::_Settlement_init` and `BaseSettlementHandler::_Settlement_handler_init` Function, which inherits from `AccessControlUpgradeable`. The lack of this initialization function can impact the proper functioning of access control roles within the contract, particularly affecting the `_grantRole` function. Without the initialization step, there is a risk of unauthorized access or incorrect role assignments, potentially compromising the security and integrity of the contract's access control mechanisms. Vulnerability Details The `BaseSettlement` contract inherits from `AccessControlUpgradeable` but lacks the ` __A...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Missing Length check in function “deocde transfer” in ERC20CodecV1.sol leading to unintended behaviour.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-569
- **Submitter:** 0xpetern
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/569
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-569.md

## Brief Summary

[deocde_transfer](https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/solidity/handler/contracts/ERC20CodecV1.sol#L65C5-L65C29) is used to decode payload and this function is external. As seen from the code, there is no check to ensure that the payload lenght is within the acceptable range. Omitting the length check in decode_transfer function can lead to several potential issues, including: 1. Unexpected Behavior: If the payload is not the expected length, the function may try to access memory beyond the end of the provided data, leading to unexpected results or incorrect decoding. For example, if the payload is shorter than expected, accessing byte s...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Failed to use of ERC20BunableUpgradeable.sol contract in BaseSettlementHandler.sol

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-571
- **Submitter:** 0xpetern
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/571
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-571.md

## Brief Summary

BaseSettlementHandler.sol is an upgradeable contract and should import and make use of openzeppelin upgradeable contracts. It imported other upgradeable contracts but failed to use an upgradeable version of ERC20Bunable.sol which is ERC20BurnableUpgradeable.sol. It imported ERC20Burnable.sol which is not upgradeable. Impact When the protocol upgrades, ERC20Burnable.sol could behave in unintended manner because it is not upgradeable. This can lead to inconsistencies and complications in the behaviour of the protocol From https://docs.openzeppelin.com/upgrades-plugins/1.x/writing-upgradeable, it clearly warns against using OpenZeppelin contract that is not upgradeable as opposed to the upgrad...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No zero address check on burn `to` at `ckr_btc.cairo::burn_from`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-404
- **Submitter:** 0xwhisperingwoods
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/404
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-404.md

## Brief Summary

The `ckr_btc.cairo::burn_from` does not check the zero address in `to`. This can create serious vulnerabilities which can be further exploited. The user might burn their tokens unintentionally, leading to permanent loss. Furthermore, the invariants in contracts that assume that token cannot be burned to a zero address such as `handler_erc20.cairo::receive_cross_chain_callback` and `handler_erc20.cairo::cross_chain_erc20_settlement` could be broken, leading to flawed logical errors elsewhere in the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# Critical Access Control Vulnerability in UUPS Upgrade Mechanism.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-463
- **Submitter:** 14Kattel
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/463
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-463.md

## Brief Summary

The `BaseSettlementHandler` contract contains a critical vulnerability in its upgrade function. The `_authorizeUpgrade` function, which is used to authorize contract upgrades, is protected solely by the `onlyOwner` modifier. This means that if an attacker gains control of the owner's account, they could perform unauthorized upgrades. This flaw compromises the integrity and security of the contract, making it possible for malicious actors to inject malicious code or alter contract behavior. Vulnerability Details The vulnerability is present in the `_authorizeUpgrade` function: Impact The `BaseSettlementHandler.sol` contract has a critical vulnerability related to the upgrade mechanism due to...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unrestricted Minting in ChakraToken.sol

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-465
- **Submitter:** 14Kattel
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/465
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-465.md

## Brief Summary

The ChakraToken contract contains a **medium severity vulnerability** in its minting mechanism. The minting functions (`mint` and `mint_to`) are accessible to any account with the `OPERATOR_ROLE`, and there are **no limits or constraints** on the number of tokens that can be minted. This lack of restriction can lead to significant inflation and dilution of token value, potentially resulting in financial losses for users and stakeholders. Additionally, excessive minting could disrupt dependent systems and trigger unintended consequences. The ability for `OPERATOR_ROLE` holders to mint an unlimited amount of tokens poses a serious risk of market manipulation and abuse.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_77_group

# No events emitted when handlers are being added and removed

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-624
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/624
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-624.md

## Brief Summary

When a handler is added to the whitelist or removed there is no event emission to off-chain entities. But as we can see in similar functions like add_validator, add_operator, add_manager they all emit events after execution. This very necessary in the functioning of this protocol, since there is a lot that goes on across chains.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# invalid addresses can bypass validation check in `addressCast`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-652
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/652
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-652.md

## Brief Summary

Invalid addresses with bytes length/size less than 20 will bypass all validation checks

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# MSTORE is different on zkSync

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-657
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/657
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-657.md

## Brief Summary

On zkSync, MSTORE has a different meaning and as result applies differently as compared to ethereum. this can cause further complications or misinterpretations on these chains

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ChakraSettlement would not work on zkSync due to the nonce ordering

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-162
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/162
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-162.md

## Brief Summary

In the current implementation of `ChakraSettlement` and `ChakraSettlementHandler` contract, there is a `nonce_manager` mapping that's increased every time there is a new cross chain transaction. The problem is that the protocol is supposed to work on zkSync and it may not work due to the problem of nonce ordering.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Owner cannot remove handler from the whitelist on Starknet

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-213
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/213
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-213.md

## Brief Summary

The owner of the handler / settlement contract has to be able to add / remove the owner of the handler. However, on the Starknet chain it cannot be done done so as `remove_handler()` functionality is missing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Message hash is assigned incorrect values when verifying signatures on Starknet

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-440
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/440
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-440.md

## Brief Summary

In the current implementation of the `settlement` contract, `message_hash` is being reassigned inside of `receive_cross_chain_msg()` function. This is an unexpected behavior as eventually incorrect value will be verified.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_59_group

# Issues with Data Types in Event and Struct Definitions for Cross-Chain Transactions in BaseSettlementHandler.sol

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-175
- **Submitter:** Alhakista
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/175
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-175.md

## Brief Summary

This report examines the data types used in the CrossChainLocked event and CreatedCrossChainTx struct in BaseSettlementHandler.sol Solidity contract, highlighting the invalid data. There are critical issues with the usage of uint256 for fields that should be of type address. The fields to and to_token in both the event and struct definitions are incorrectly declared as uint256 when they represent addresses on the destination chain. This creates a scenario where the contract performs in an unexpected manner due to internal errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `SettlementSignatureVerifier` Fails to Validate Smart Contract Wallet Signatures from Validators

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-707
- **Submitter:** Breeje
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/707
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-707.md

## Brief Summary

If a validator uses a smart contract wallet instead of an externally owned account (EOA), the `SettlementSignatureVerifier` fails to validate its signature.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Accepting all ERC20 tokens is not safe

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-187
- **Submitter:** Decap
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/187
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-187.md

## Brief Summary

Accepting all ERC20 by the bridge is not safe. Malicious user can create ERC20 token contract that will be harmful for the protocol. For example it is possible to create ERC20 token that will be very gas expensive when making transfer. This can burn out funds out of contract, cause DoS for other users, block bridge etc. Here is example contract that can simulate malicious ERC20 token

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# There is a problem with the data structure in the inheritance of upgradable contracts

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-99
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/99
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-99.md

## Brief Summary

In contracts such as BaseSettlementHandler, gap variables are not used, nor are designated storage slots used for storage. During subsequent upgrades, variable overwriting may occur.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# The Number Of Required Validators on Starknet Can Be Set To Zero

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-593
- **Submitter:** Respx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/593
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-593.md

## Brief Summary

Although the likelihood of a manager setting the number of required validators to zero is low, the impact would be extreme: it would allow any user to successfully submit any cross chain message to the protocol. This would essentially unlock all tokens held by the relevant handler contract to any user, and also allow any other handlers to be compromised.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Incorrect message ID parsing due to off-by-one error in `decode_message`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-250
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/250
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-250.md

## Brief Summary

The `decode_message` function doesnt correctly parse the `message_id` field from the payload due to an off-by-one error. Because of this, `message_id` will be misaligned, which can lead to incorrect message processing, potential misinterpretation of the message, and downstream errors in any logic that relies on the correct `message_id`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Missing keccak256 when computing the message_hash

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-493
- **Submitter:** SBSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/493
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-493.md

## Brief Summary

`message_hash` is incorrectly computed, which will not allow signature verification.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_129_group

# Integer overflow in mint_to funciton

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-364
- **Submitter:** Sabit
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/364
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-364.md

## Brief Summary

The actual amount minted will be much smaller than intended.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# TokenRoles contract does not implement `_authorizeUpgrade` & `__UUPSUpgradeable_init()`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-730
- **Submitter:** Shubham
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/730
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-730.md

## Brief Summary

`ChakraToken` contract inherits the `TokenRoles` contract & initializes it in the `initialize()`. The ChakraToken contract implements the `_authorizeUpgrade()` but the `TokenRoles` contract neither implements the `__UUPSUpgradeable_init()` nor the `_authorizeUpgrade()` Impact `TokenRoles` contract cannot be upgraded in the future even though it inherits UUPSUpgradeable from oz.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# After all validators are removed, the system can no longer perform effective signature verification.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-622
- **Submitter:** Taiger
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/622
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-622.md

## Brief Summary

The vulnerability exists in the `remove_validator` function, where the removal of a validator is performed without any checks on the remaining number of validators. If all validators are removed from the system, the contract could become non-functional, as it would no longer have any validators available to verify signatures. This would lead to critical security risks because cross-chain transactions would no longer be validated. The absence of validators would leave the system open to attacks and manipulation, or even cause it to cease functioning entirely. Consequences: - **Loss of Signature Validation**: If all validators are removed, the `check_chakra_signatures` function will not be ab...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Cross chain callback will always fail if source `chain_name` is changed.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-342
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/342
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-342.md

## Brief Summary

The bug will cause the cross-chain callback to fail when the `chain_name` is changed at source after a cross-chain tx has been sent.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_22_group

# Missing zero address check for owner in `TokenRoles_transferOwnership`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-554
- **Submitter:** Tonchi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/554
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-554.md

## Brief Summary

This contract is inherited by `ChakraToken.sol` and `ChakraTokenUpgradeTest.sol`. Ownership Transfer to Zero Address, Permanent Loss of Control, Mismanagement of Role. 1. Critical Loss of control: - Ownership loss is one of the most severe vulnerabilities because it permanently disables the contract’s governance and security features. - Without an active owner, no updates, fixes, or upgrades can be made to the contract, which is especially dangerous for upgradeable contracts like this one. - If this is a contract in charge of valuable assets (like tokens or DeFi operations), this can lead to irreversible financial damage. 2. Attack Vector for Malicious Actors: - An attacker or a malicious a...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Calldata Slicing Inefficiency Leading to Contract Disruption and Inaccessibility

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-232
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/232
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-232.md

## Brief Summary

The current implementation of signature verification in SettlementSignatureVerifier contract using calldata slicing introduces a critical inefficiency that could lead to severe problems for the contract’s usability. calldata slicing use in as would be explained in this report results in repeated, unnecessary memory allocations, significantly increasing gas consumption for every signature processed. This inefficiency does more than just increase gas usage. In practice, this excessive gas costs can lead to transaction Reversion and any attempt to verify multiple signatures (e.g., for multi-signature transactions or consensus mechanisms) could exceed the gas limit, causing critical transaction...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_133_group

# An attacker can reuse old message with altered payload

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-271
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/271
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-271.md

## Brief Summary

In the current implementation there is no checks to make sure that the payload used is unique and has not already been used. it is important to verify the uniqueness of the payload when processing cross-chain messages. As a result, the same payload could be reused across different transactions, leading to potential replay attacks. The current txid generation logic omits the `payload_type` and `payload` fields, which can lead to potential issues: The absence of `payload_type` and `payload` in the txid hash makes the system susceptible to replay attacks or message forgery, where an attacker might reuse an old message with altered payloads, bypassing checks and causing unintended consequences....

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# Type Mismatch in from_address Parameter `uint256` instead of `address`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-272
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/272
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-272.md

## Brief Summary

The `from_address` parameter is represented as an address in the function `send_cross_chain_msg` but incorrectly represented as a uint256 instead of an address in the function `receive_cross_chain_msg`. From the comments This means that message_hash will have from_address as type uint256 instead of type address This type mismatch could lead to incorrect address handling, potential signature verification failures, and unexpected behavior within the smart contract. If the from_address is misinterpreted or manipulated, it could open up the contract to potential exploits, such as bypassing access controls or altering transaction outcomes. By doing this our message_hash now contains a `from_addr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_63_group

# Signature Verification missing protection from Replay Attacks

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-273
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/273
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-273.md

## Brief Summary

Currently there is no way the function is protected from replay attacks as there is no nonce or even a deadline. Without a deadline (or expiration time) for signatures in cross-chain message verification or a nonce, an attacker could potentially reuse a valid signature indefinitely. This lack of temporal restriction can open up the system to replay attacks, where old messages are resent to manipulate the system, bypass intended logic, or disrupt operations. It is always a good idea to use deadlines and nonce whenever signatures are involved

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_115_group

# Lack of Message Hash Validation in verifyECDSA Could Allow Forgery or Replay Attacks

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-279
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/279
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-279.md

## Brief Summary

The current implementation of the `verifyECDSA` function does not validate the source or correctness of the `msgHash`. This leaves the system vulnerable to signature forgery and replay attacks. An attacker could potentially submit a valid signature for a malicious or incorrect `msgHash`, leading to unauthorized actions within the system. By not verifying that the `msgHash` corresponds to a legitimate transaction, validators could unknowingly sign off on illegitimate or fraudulent transactions, allowing attackers to execute unintended operations, such as replaying previous valid signatures in a different context, resulting in repeated execution of certain actions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# Off-chain validators(executors) can be griefed on multiple occasion by malicious transaction creators

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-732
- **Submitter:** b0g0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/732
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-732.md

## Brief Summary

Malicious users can create transactions with invalid parameters due to inefficient validation when transactions are created and grief the executors of callbacks. Description Cairo contracts can be exploited for the following reasons: - cross_chain_erc20_settlement - can be called with 0 amount or address(0) for to - send_cross_chain_msg - can provide invalid `payload_type` which also revert execution later Solidity - the to_hadnler is not validated

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# wrong implement of "verifyECDSA"

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-283
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/283
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-283.md

## Brief Summary

Detailed description of the impact of this finding. There is no check of zero address for msgHash.recover(sig).There should be a check for validators address should be not zero.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_53_group

# Too much power given to "manager" Centralization issue

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-604
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/604
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-604.md

## Brief Summary

Detailed description of the impact of this finding. In the "settlement.cairo" we are giving too much power to the "manager".as it can set the number of required_validators_num.it can also add and remove the validator.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Insufficient gas requirement in cross-chain ERC20 settlement

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-252
- **Submitter:** blackpanther
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/252
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-252.md

## Brief Summary

The `cross_chain_erc20_settlement` function, crucial for initiating token transfers from Ethereum to Starknet, lacks a minimum gas requirement setting. This omission can lead to transaction failures in the cross-chain messaging process, potentially resulting in stuck or failed token transfers. Vulnerability Details - **Function**: `cross_chain_erc20_settlement` - **Issue**: No minimum gas requirement set for `send_cross_chain_msg` call - **Required Gas**: Minimum 20,000 wei for L1 to L2 messaging as per [docs](https://book.cairo-lang.org/ch16-04-L1-L2-messaging.html?highlight=msg.value#sending-messages-from-ethereum-to-starknet:~:text=It's%20important%20to%20note%20that%20we,be%20deserializ...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# Potential Overflow in Bitmasking Operations

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-254
- **Submitter:** crown22
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/254
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-254.md

## Brief Summary

The constants in the file used for bit manipulation (MASK_8, TWO_POW_8, etc.) can potentially cause overflows or incorrect handling of large numbers. When used improperly, these constants might result in unexpected values in cryptographic operations or arithmetic manipulations. If such issues occur in a cross-chain context or while calculating balances, it could lead to major inconsistencies, security exploits, or financial losses in the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reentrancy Vulnerability in ERC20 Handler

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-256
- **Submitter:** crown22
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/256
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-256.md

## Brief Summary

Without reentrancy protection, an attacker can exploit ERC20 functions by recursively calling them before the state is fully updated. This could result in repeated transfers or minting of tokens, allowing attackers to drain the contract’s funds or inflate the token supply.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent Message Encoding/Decoding

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-257
- **Submitter:** crown22
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/257
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-257.md

## Brief Summary

Incorrect or inconsistent encoding/decoding of cross-chain messages can lead to manipulated payloads being passed between chains. If a payload is incorrectly decoded or lacks strict validation, an attacker could craft a message that bypasses intended checks, causing incorrect transaction processing or unauthorized transfers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Protection Against Malicious Contract Addresses

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-261
- **Submitter:** crown22
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/261
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-261.md

## Brief Summary

The lack of protection against malicious contract addresses poses a significant security risk, allowing attackers to replace valid contract addresses with malicious ones. This can lead to unintended interactions, such as unauthorized token transfers, reentrancy attacks, or malicious code execution that drains funds from the protocol. Without proper validation, malicious contracts can be introduced into critical functions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Weak Validation of ERC20 Transfer Data

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-262
- **Submitter:** crown22
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/262
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-262.md

## Brief Summary

Weak validation of ERC20 transfer data, particularly for fields like amount, can lead to various attack vectors, including overflow attacks or unintended token transfers. This opens up the potential for malicious users to transfer more tokens than intended or to exploit overflow vulnerabilities to disrupt the contract's logic.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# TokenRoles contract has Incorrect Argument Count in __Ownable_init Call

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-16
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/16
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-16.md

## Brief Summary

Detailed description of the impact of this finding. Title: TokenRoles contract has Incorrect Argument Count in __Ownable_init Call • Severity: Medium • Impact: __Ownable_init() does not accept any arguments, leading to a type error during contract deployment. • Status: Unresolved • File: TokenRoles.sol • Lines Affected: 20 Contract Initialization Failure. The TokenRoles contract attempts to initialize the OwnableUpgradeable contract by calling __Ownable_init(_owner). However, __Ownable_init() does not accept any arguments, leading to a type error during contract deployment.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_50_group

# ChakraSettlementHandler contract has Unchecked transferFrom Return Value in _safe_transfer_from During ERC20 Locking

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-300
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/300
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-300.md

## Brief Summary

Detailed description of the impact of this finding. Title: ChakraSettlementHandler contract has Unchecked transferFrom Return Value in _safe_transfer_from During ERC20 Locking. Contract: ChakraSettlementHandler.sol. Functions: _safe_transfer_from() & _erc20_lock(). The _safe_transfer_from function does not verify the return value of the external IERC20(token).transferFrom(from, to, amount) call. According to the ERC-20 standard, the transferFrom function returns a boolean indicating whether the transfer succeeded. Some ERC-20 tokens may return false instead of reverting on failure. If the contract does not check the return value, it will assume that the transfer was successful even if it fa...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_56_group

# ERC20CodecV1 contract has Typographical Error in Function Name deocde_transfer

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-48
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/48
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-48.md

## Brief Summary

Detailed description of the impact of this finding. Title: ERC20CodecV1 contract has Typographical Error in Function Name deocde_transfer • Severity: Medium • Impact: A typographical error was found in the function name deocde_transfer within the Solidity contract and interface. • Status: Unresolved • File: ERC20CodecV1.sol • Lines Affected: 65-74 Location: • File: /2024-08-chakra/solidity/handler/contracts/ERC20CodecV1.sol • Line: 54 • File: /2024-08-chakra/solidity/handler/contracts/interfaces/IERC20CodecV1.sol • Line: 23 • Solidity File: /2024-08-chakra/solidity/handler/contracts/ERC20CodecV1.sol • Line: 54 • Interface File: /2024-08-chakra/solidity/handler/contracts/interfaces/IERC20Cod...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_40_group

# Unsafe use of `transfer()`/`transfer_from()` across Cairo contracts

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-630
- **Submitter:** devival
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/630
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-630.md

## Brief Summary

The automated bot report does not include the Cairo handler contract, even though the same issue is present in Solidity handler contract. Not all `IERC20` implementations `revert()` when there's a failure in `transfer()`/`transferFrom()`. The function signature has a `boolean` return value and they indicate errors that way instead. By not checking the return value, operations that should have marked as failed, may potentially go through without actually making a payment. Also, some tokens do not implement the ERC20 standard properly, and do not return booleans as the specification requires. Instead, they have no return value. Impact In the context of `cross_chain_erc20_settlement` function...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Missing parameters in txid hash compute

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-727
- **Submitter:** eierina
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/727
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-727.md

## Brief Summary

The parameters `payload_type` and `payload` are not included in the txid hash compute, allowing the `payload` / `payload_type` to be tampered with for a given txid.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# The `verifyECDSA` function doesn't handle the case of an empty signature array correctly. It immediately returns false if there are no signatures, even though the invariant expects it to return true for `sign_type == 0`.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-115
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/115
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-115.md

## Brief Summary

The `SettlementSignatureVerifier` contract suffers from vulnerability that allows any transaction to be validated with an empty signature when the signature type is set to ECDSA (0). This flaw compromises the entire signature verification process, leading to unauthorized transactions being processed as valid. **The impact is severe, as it undermines the fundamental security assumptions of the system, allowing attackers to execute unauthorized settlements or manipulate the state of the contract without proper authentication.**

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_81_group

# The `remove_validator` function does not check if the validator being removed actually exists in the `validators` mapping. It only checks if the caller has the `MANAGER_ROLE`.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-122
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/122
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-122.md

## Brief Summary

The impact of this bug is that it allows managers to arbitrarily decrease the `validator_count` by removing non-existent validators. Unauthorized parties can remove validators, potentially compromising the integrity of the settlement process. **The impact of this issue is twofold:** 1. The `validator_count` can be manipulated by managers, causing it to diverge from the actual number of active validators. 2. The inconsistency in the validator management system may lead to confusion and incorrect assumptions about the state of the validators. While this does not directly lead to a loss of funds, it can cause discrepancies in the validator set and undermine the integrity of the settlement sign...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# nonce is incremented unconditionally even if the `amount` requirement fails. If `amount` is 0, the function will revert due to the failed requirement, but the nonce would have already been incremented.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-66
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/66
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-66.md

## Brief Summary

The `ChakraSettlementHandler` contract is vulnerable in the `cross_chain_erc20_settlement` function where the nonce is incremented unconditionally, even if the settlement fails due to invalid input parameters. **The affected parties are:** * Users who initiate cross-chain ERC20 settlements with invalid parameters (e.g., zero amount). They may lose the ability to retry the settlement with the same nonce. * The protocol itself, as it may have inconsistent nonce values for users, leading to potential replay attacks and incorrect transaction handling.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# Unsafe casting leads to sending tokens to incorrect address

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-358
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/358
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-358.md

## Brief Summary

The function casts a 'uint256' to an address by first converting it to a 'uint160'. 'uint160' is unsafe when you cast from larger types like 'uint256' without ensuring that the value fits within the 160-bit range, as this can result in data truncation and potential vulnerabilities. Casting a 'uint256' to 'uint160' truncates the upper 96 bits (256 - 160 = 96). This means if the original 'uint256' contains important data in those upper 96 bits, it will be lost in the conversion, leading to incorrect results. If that 'uint256' is supposed to be an address but contains extra data, you could end up with an invalid or unintended address. If the input to the cast comes from an untrusted source (su...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_67_group

# Lack of Time-Lock Enforcement in Cross-Chain Settlement Functions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-324
- **Submitter:** igdbase
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/324
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-324.md

## Brief Summary

The Chakra Protocol claims to provide self-custodian staking by allowing users to stake Bitcoin without transferring assets out of their wallets. This is supposed to be achieved through time-lock scripts, which ensure that assets are not exposed to third-party risks. However, the current implementation of key cross-chain settlement functions, including `receive_cross_chain_msg`, `receive_cross_chain_callback`, and `cross_chain_erc20_settlement`, does not enforce any time-locks on assets. As a result, assets that should remain locked for a defined period can be settled or unlocked prematurely. This introduces risks of premature asset release and potential double-spending exploits, underminin...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Status Update in processCrossChainCallback Function

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-418
- **Submitter:** m4k2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/418
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-418.md

## Brief Summary

In the `processCrossChainCallback` function of the ChakraSettlement contract, there's a logical error in updating the status of cross-chain transactions. Even when the `receive_cross_chain_callback` function returns true (indicating success), the status may incorrectly remain as "Pending" instead of being updated to "Success". Impact This vulnerability can lead to several significant issues: 1. Inconsistent Transaction States: Transactions that have been successfully processed may remain in a "Pending" state, leading to confusion and issues with further processing or user interactions. 2. Broken State Machine: The incorrect status update breaks the expected flow of the cross-chain transacti...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_27_group

# Incorrect Interface Dispatcher Usage in Settlement Contract

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-682
- **Submitter:** mansa11
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/682
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-682.md

## Brief Summary

The Settlement contract is attempting to use an undefined IHandlerDispatcher interface at lines 325 and 414, which is not present in the provided interfaces. This causes a mismatch between the contract implementation and the defined interfaces. Description In the Settlement contract `settlement.cairo` - cairo\handler\src\settlement.cairo, at lines `353` and `393` which are both present in the `receive_cross_chain_msg` and `receive_cross_chain_callback` respectively, the code attempts to create a dispatcher for an interface that doesn't exist: However, the `interfaces.cairo` file does not define an `IHandlerDispatcher`. Instead, it provides two relevant interfaces: `IHandler` and `IERC20Hand...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# There is actually no checks that prevents a non ERC20 token being passed

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-512
- **Submitter:** mjcpwns
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/512
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-512.md

## Brief Summary

There is a lack of check for whether an ERC 20 token is actually passed in or another type of token require(isValidPayloadType(payload_type), "Invalid payload type"); This invariant will be broken: The contract only accepts valid payload types (in this case, only ERC20 payloads). To be honest I do not think the impact will be high, maybe the user will lose their token if it is not ERC20 but I do not think it will be a big issue but I report this just in case.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# A block reorg can leave users with extra tokens

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-700
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/700
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-700.md

## Brief Summary

A block reorg can leave users with extra tokens

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# Missing Checks for Address(0)

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-192
- **Submitter:** pwnforce
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/192
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-192.md

## Brief Summary

Following functions miss validation checks for Address(0): 1- ChakraSettlementHandler: initialize function should check address values _owner, _token, _codec, _verifier, and _settlement. 2- ChakraSettlement: initialize function should check address values _owner and _verify_contract: 3- TokenRoles: functions __TokenRoles_init, transferOwnership, and add_operator should check address values _owner, _operator, newOwner, and newOperator.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_107_group

# Missing Checks for Removing Managers in settlement.cairo and ckr_btc.cairo

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-223
- **Submitter:** pwnforce
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/223
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-223.md

## Brief Summary

There are two `remove_manager` functions in settlement.cairo and ckr_btc.cairo: However, there are no checks to ascertain that the old_manager being removed is indeed an existing manager. For example, in functions `remove_validator` and `remove_operator` in settlement.cairo and ckr_btc.cairo respectively, it is checked that the old_validator and old_operator being removed are indeed existing validators and operators: This would result in incorrect state changes, where non-managers could be marked as removed managers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_51_group

# Decimals assigned on mutable variable

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-513
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/513
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-513.md

## Brief Summary

Token decimals is stored on mutable variable which could be modified later since it is an upgradable contract. Changing the number of decimals could cause issues in external applications or token holders who assume a fixed number of decimals. `uint8 set_decimals;`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Decoding and Encoding of Addresses

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-515
- **Submitter:** rabTAI
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/515
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-515.md

## Brief Summary

The contract uses abi.decode with uint256 for decoding addresses (from, to, from_token, to_token). However, Ethereum addresses are 20 bytes (160 bits) long, not 32 bytes. Using uint256 will result in incorrect decoding of addresses, leading to potential security vulnerabilities such as incorrectly decoded addresses, which could result in token transfers being sent to wrong or invalid addresses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# No input validation for Encoding/Decoding Functions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-516
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/516
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-516.md

## Brief Summary

There is no input validation on the encoded or decoded payloads. Without validation, malicious or malformed payloads could result in unexpected behavior, including incorrect decoding or possible denial of service (DoS) attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_108_group

# Lack of Proper Event Emission in Critical Function

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-186
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/186
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-186.md

## Brief Summary

While the function updates the internal state of the contract,the function receive_cross_chain_callback in handler_erc20.cairo contract doesn't emit any events to notify external observers (like applications or other contracts) about the successful settlement. Events are critical for providing transparency and traceability of contract interactions, especially when state-changing operations affect cross-chain transactions and token balances. IMPACTS: Potential for Malicious Activities to Go Undetected: - In the absence of emitted events, an attacker could exploit vulnerabilities or manipulate state variables without being detected. This could be used to obscure malicious activities such as t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# Birthday Attack Vulnerability in `txid` Generation

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-480
- **Submitter:** sivanesh_808
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/480
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-480.md

## Brief Summary

The `txid` (transaction ID) is generated using the `keccak256` hash function, a cryptographic hash function that maps an arbitrary amount of input data into a fixed-length output of 256 bits. While this ensures a unique output for different inputs, the large input space and repetitive parameters in the input to `keccak256` open up the possibility of a **birthday attack**, where two different inputs generate the same hash output. The birthday attack exploits the **birthday paradox**, which reduces the complexity of finding a hash collision from `2^256` to approximately `2^128`, making collisions significantly more feasible with enough attempts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of message ordering may lead to failed transactions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-568
- **Submitter:** tonisives
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/568
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-568.md

## Brief Summary

The `receive_cross_chain_msg` function in the `ChakraSettlement` contract processes cross-chain messages without ensuring that they are executed in the order they were sent. This can lead to potential transaction failures due to dependencies between messages. Impact The lack of enforced message ordering can result in unexpected behavior or failures in dependent transactions. For example, if a later message depends on the successful processing of an earlier message, processing them out of order can cause the later message to fail, leading to potential loss of funds or incorrect state.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_94_group

# Missing check on mode in ChakraSettlementHandler::_Settlement_handler_init function

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-230
- **Submitter:** yudan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/230
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-230.md

## Brief Summary

In ChakraSettlementHandler::cross_chain_erc20_settlement function, it depends on the value of `mode` to determine if token needs to be lock or unlock, and call `_erc20_lock` or `_erc20_unlock` internal function. But there missing check that the mode may not one of value in enum `SettlementMode`, e.g. 5, which exceed the max value of enum `SettlementMode`. In this way, the function will do nothing for user's token, but construct a cross chain request. which make malicious user receive token on the other chain.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_62_group
