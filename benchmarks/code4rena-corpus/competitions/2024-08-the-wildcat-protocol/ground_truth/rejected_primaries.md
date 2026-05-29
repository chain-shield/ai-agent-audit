# Rejected Primary Findings: The Wildcat Protocol

# Using multiple versions of Solidity in one project can lead to several issues

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-51
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/51
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-51.md

## Brief Summary

- Incompatibility Between Contracts - Increased Complexity in Maintenance - Different Compiler Behaviors can provoke conflicts between libraries and interfaces used within contracts

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use of adress(0) as sentinel adress in unsetCredential() could cause incompatibility issues with future Ethereum upgrades eg (EIP-1559, serenity)

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-186
- **Submitter:** 0xfeMMANUEL
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/186
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-186.md

## Brief Summary

the use of address(0) as sentinel account for the status.lastProvider in the unsetCredential(), raises isuses of code compatibility in future upgrades(eg, EIP-1559,serenity etc) as its use could affect contract behaviours or in worst case break the code,also the upcoming ethereum upgrade introduces a new transaction pricing mechanism, using address(0) might not be compatible with future gas pricing changes. Also future upgrades might introduce new wallet arhitectures, potentially altering the behavior of adress(0)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Improper logic implementation on `FixedTermLoanHooks::revokeRole` function when a provider whose role has been removed they can still revoke a lender.

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-179
- **Submitter:** 0xsagetony
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/179
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-179.md

## Brief Summary

System flaw, unauthorized role providers will still take part in revoking lender's roles.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Role Encoding/Decoding Vulnerability in RoleProvider.sol

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-114
- **Submitter:** 14Kattel
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/114
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-114.md

## Brief Summary

The `encodeRoleProvider` and `decodeRoleProvider` functions in the `RoleProvider` implementation use low-level bitwise operations to manage critical provider data (`timeToLive`, `providerAddress`, and `pullProviderIndex`). An incorrect shift amount in these operations can lead to inaccurate encoding or decoding of the `providerAddress`. This could result in assigning roles to unintended addresses, potentially allowing unauthorized entities to gain access to privileged roles. Impact - **Role Mismanagement**: The incorrect decoding of `providerAddress` can allow unintended addresses to receive roles. This could lead to severe permission errors, enabling unauthorized users to act with privileg...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_36_group

# mproper Handling of Constructor Arguments in LibStoredInitCode Library Leads to Incorrect Contract Initialization

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-117
- **Submitter:** 14Kattel
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/117
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-117.md

## Brief Summary

The `LibStoredInitCode` library’s `create2WithStoredInitCode` and `create2WithStoredInitCodeCD` functions improperly handle constructor arguments during contract deployment. This vulnerability can result in contracts not being initialized correctly, potentially leading to critical security flaws such as unauthorized access, broken logic, or even complete denial of service (DoS) if initialization is essential for the contract’s functionality. Impact The vulnerability occurs when the library incorrectly passes constructor arguments during contract deployment, which can cause the following: - **Uninitialized Contracts**: Contracts may not receive the expected initialization data, leading to cr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Escrow Sanction Override Bug: Critical Vulnerability and Compliance Risks

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-45
- **Submitter:** 8olidity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/45
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-45.md

## Brief Summary

A bug exists in the createEscrow function where `sanctionOverrides[borrower][escrowContract]` is erroneously set to true instead of `sanctionOverrides[borrower][account]`. This misidentification causes sanctions to be overridden for the escrow contract address rather than the intended account. As a result, the Smart Lending Escrow system fails to apply the intended exemptions for sanctioned accounts, which may compromise the system’s functionality.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Lack of validation for addresses returned by `tmpEscrowParams` in `WildcatSanctionsEscrow` contract

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-329
- **Submitter:** Akay
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/329
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-329.md

## Brief Summary

Lack of validation for addresses returned by `tmpEscrowParams` in `WildcatSanctionsEscrow` contract

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Borrowers would lose a lot of funds if market is intentionally/unintentionally frequently updated

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-273
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/273
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-273.md

## Brief Summary

Gains for lenders at the expense of the borrowers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# isKnownLenderOnMarket is not set to false whenever a user withdraws all his scaled balance.

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-103
- **Submitter:** Bigsam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/103
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-103.md

## Brief Summary

A user remains in the "known" state even after withdrawing their entire balance, which violates the intended behavior of the system. According to the design, a user should only be considered "known" if they pass the necessary checks, including having a non-zero balance. The current implementation allows a "known" user to withdraw all their balance and still remain marked as a known lender.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unrestricted Token Transfer Vulnerability in rescueTokens Function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-322
- **Submitter:** Bryan_Conquer
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/322
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-322.md

## Brief Summary

The rescueTokens function allows the borrower to transfer the entire balance of any token from the contract, except for the main asset. However, the function does not check the amount or context in which tokens are being transferred. This results in the following critical issues: No Limit on Token Amounts: The function transfers all of the specified token's balance in the contract without any limit or restriction on the amount. Any auxiliary tokens (e.g., governance tokens, reward tokens) could be completely drained by the borrower, even if they are essential for the contract's operation. Risk of Mistaken or Malicious Token Transfer: Since no additional validation is performed on the token...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# Inconsistent Use of `block.timestamp` in Multi-chain Deployments

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-101
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/101
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-101.md

## Brief Summary

This inconsistency could cause credential expiration checks to fail or pass erroneously. As a result, users may receive or retain roles they are not eligible for, or their valid roles could be unjustly revoked. The misalignment between L1 and L2 timestamp mechanisms may lead to unfair access control or incorrect permissions in a multi-chain deployment.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_01_group

# Compatibility Issue Due to Unsupported PUSH0 Opcode

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-102
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/102
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-102.md

## Brief Summary

The primary impact of this issue is that the contract may fail to deploy on networks like Polygon where the PUSH0 opcode is unsupported. This failure can prevent the contract from being created and utilized, leading to potential disruptions in the intended functionality of the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unauthorized Approval of Token Allowances

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-105
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/105
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-105.md

## Brief Summary

The primary impact of this issue is that any user can set an allowance for any address, effectively allowing unauthorized third parties to withdraw tokens from an account. For example, if Alice accidentally or maliciously approves a `spender` address controlled by Bob, Bob can then use the `transferFrom` function to withdraw tokens from Alice's account, potentially draining it of tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_43_group

# Withdrawal Queue Can Remain Unfulfilled After Liquidity is Burnt

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-107
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/107
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-107.md

## Brief Summary

Users who attempt to withdraw their funds after liquidity is burnt will have their withdrawals queued but will not be able to access their funds, potentially indefinitely. This can lead to frustration among users and a loss of trust in the protocol, as it gives the false impression that withdrawals are still possible when in reality, there are no available assets to fulfill these requests.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Address Collision and Stale Initialization Code Vulnerability in Market Deployment Using `LibStoredInitCode`

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-66
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/66
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-66.md

## Brief Summary

The primary impact of this vulnerability is the potential deployment of a market contract with stale or incorrect logic. If the stored initialization code is reused without being updated, it could result in deploying a market that does not behave as expected or that contains outdated parameters. Additionally, using the same salt across deployments can lead to address collisions, causing markets to be deployed at the same address, which could overwrite or conflict with existing markets.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Premature Removal of Registered Controller After Recent Registration

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-71
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/71
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-71.md

## Brief Summary

A recently registered controller can be removed before it has a chance to perform its intended operations, which could disrupt the system's functionality. If a legitimate controller is prematurely removed, critical actions it was supposed to execute might not occur, leading to protocol inefficiencies or unexpected behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_114_group

# totalSupply Overestimates Circulating Supply

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-23
- **Submitter:** ETHworker
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/23
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-23.md

## Brief Summary

The `totalSupply` function in `MarketStateLib` does not subtract `scaledPendingWithdrawals` when calculating the total circulating supply of market tokens. This results in the function overestimating the actual supply by the amount of pending withdrawals that have not been processed yet. As a result, functions that rely on `totalSupply` such as `maximumDeposit`, `borrowableAssets`, and `liquidityRequired` will also overestimate the available liquidity in the market. This could allow a malicious borrower to withdraw excess funds, potentially causing losses for lenders. Impact A malicious borrower could exploit the inflated `totalSupply` value to: 1. Deposit more than the intended maximum, as...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Improper Handling of Loop-Called Methods and MultiCall Contracts in Spherex Guarded Methods Can Cause UnIntended Reverts

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-187
- **Submitter:** Fortis_audits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/187
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-187.md

## Brief Summary

The WildcatMarket contracts utilizes the Spherex `sphereXGuardExternal` protection for certain critical methods. However, when these methods are invoked within loops (or via MultiCall contracts), the Spherex protection requires an explicit allowance for every possible loop iteration or combination of called methods. This design leads to false positives in Transaction Flow (TF) and Control Flow (CF) protection modes, causing unintended reverts for integrators and users of the Wildcat protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The user may lose their funds when calling the WildcatMarketWithdrawals.executeWithdrawal() and WildcatMarketWithdrawals.executeWithdrawals() functions

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-151
- **Submitter:** Inspecktor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/151
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-151.md

## Brief Summary

The user may lose their funds when calling the WildcatMarketWithdrawals.executeWithdrawal() and WildcatMarketWithdrawals.executeWithdrawals() functions

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# Possible DOS when calling HooksFactory.deployHooksInstance()

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-152
- **Submitter:** Inspecktor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/152
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-152.md

## Brief Summary

Possible DOS when calling HooksFactory.deployHooksInstance()

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Unable to call HooksFactory.deployMarket() on assets with Non string metadata Name/Symbol

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-153
- **Submitter:** Inspecktor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/153
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-153.md

## Brief Summary

Unable to call HooksFactory.deployMarket() on assets with Non string metadata Name/Symbol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# safetransfer does not check the codesize of the token address, which may lead to fund loss

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-323
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/323
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-323.md

## Brief Summary

safetransfer does not check the codesize of the token address, which may lead to fund loss

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reentrancy Vulnerability in createEscrow()

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-324
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/324
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-324.md

## Brief Summary

Reentrancy Vulnerability in createEscrow()

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# Borrower Lockout in WildcatMarketController

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-280
- **Submitter:** K42
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/280
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-280.md

## Brief Summary

Between `WildcatMarketController` and `WildcatArchController`. In the `registerMarket` function of `WildcatArchController` and the `deployMarket` function of `WildcatMarketController`. In `WildcatArchController.sol`: In `WildcatMarketController.sol`: Impact Bug allows a controller to register markets for borrowers who are no longer approved in the system. While there is access control preventing non-borrowers from directly deploying markets, the missed check in `registerMarket` means that a controller associated with a removed borrower can continue to register markets. Opening a vector to unauthorized market creation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Potential Storage Collision in Withdrawal Status Mapping Due to Type Mismatch

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-281
- **Submitter:** Nexarion
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/281
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-281.md

## Brief Summary

The mismatch between the key types in the `accountStatuses` mapping under the struct `WithdrawData` and the `expiry` parameter type could lead to unexpected behavior, including: 1. Storage collisions 2. Incorrect withdrawal status retrievals 3. Potential loss or corruption of user withdrawal data

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# Interest Rate Manipulation Due to Incremental Reductions Bypass Reserve Ratio Protection

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-286
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/286
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-286.md

## Brief Summary

The `onSetAnnualInterestAndReserveRatioBips` function in the `MarketConstraintHooks` contract is designed to handle changes to the annual interest rate and potentially adjust the reserve ratio. However, there's a vulnerability that could allow manipulation of the reserve ratio. The function doesn't properly handle cases where the interest rate is repeatedly reduced by small amounts. This can lead to a scenario where the reserve ratio is incrementally increased without triggering the intended protective mechanisms.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# validateCredential Reverts on Successful Validation

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-296
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/296
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-296.md

## Brief Summary

The Wildcat protocol utilizes hooks for customized market logic, with AccessControlHooks being a prime example. This contract manages lender access and allows external role providers to validate credentials. The _tryValidateCredential function in AccessControlHooks is crucial for this validation process, using calldata to interact with external providers. The _tryValidateCredential function in AccessControlHooks is designed to interact with external role providers to validate lender credentials. However, a flaw in its implementation causes it to revert even when a credential is successfully validated by the provider. This occurs because the function unconditionally throws an error if the ex...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Inconsistent State After Market Closure

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-303
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/303
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-303.md

## Brief Summary

The Wildcat protocol facilitates the creation of lending markets with varying parameters, like interest rates and withdrawal terms. These markets employ a mechanism to update their state, incorporating accrued interest, fees, and handling withdrawals. A crucial function in this process is _writeState, which updates the stored market state and ensures consistency between the actual and recorded states. A critical oversight exists in the _writeState function related to market closure. When a market is closed, the protocol aims to halt further interest accrual and potentially redistribute remaining assets. However, the _writeState function doesn't prevent state updates even after closure, pote...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_80_group

# Missing Sanction Check During Market Origination

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-308
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/308
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-308.md

## Brief Summary

The Wildcat protocol, a decentralized lending platform, prioritizes security and regulatory compliance. It incorporates a Sanctions Sentinel to enforce sanctions, preventing sanctioned addresses from participating in markets. While the protocol implements checks at various interaction points, a crucial oversight exists during the initial market creation process. This omission could potentially allow sanctioned borrowers to establish markets, undermining the protocol's integrity and compliance. Details The Wildcat protocol utilizes the WildcatSanctionsSentinel contract to manage sanctions. This contract determines whether an address is flagged by Chainalysis and allows borrowers to override...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_90_group

# Unchecked Array Access in getRegisteredBorrowers Function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-134
- **Submitter:** Okazaki
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/134
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-134.md

## Brief Summary

Unchecked Array Access in `WildcatArchController::getRegisteredBorrowers` Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unbounded `extraData` in Hook System Enables Gas Griefing

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-139
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/139
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-139.md

## Brief Summary

The Wildcat protocol's hook system allows for arbitrary `extraData` to be appended to certain function calls. This `extraData` is not directly handled by the market functions, but is instead extracted and processed by the corresponding hook functions. Crucially, the size of this `extraData` is not bounded, potentially allowing malicious actors to create transactions with extremely high gas costs. The following functions in the Wildcat protocol allow for arbitrary `extraData` to be appended to their calldata: - `deposit` and `depositUpTo`: These functions allow for extra data. They call `_depositUpTo` which uses the `hooks.onDeposit` function that processes extra data. - `queueWithdrawal` an...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_118_group

# Interest can be accrued twice in the same transaction

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-94
- **Submitter:** Takarez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/94
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-94.md

## Brief Summary

Interest, along with protocol fees, can be accrued twice within the same transaction. Description When a `lender` intends to deposit into the market, they call the [depositUpTo](https://github.com/code-423n4/2024-08-wildcat/blob/fe746cc0fbedc4447a981a50e6ba4c95f98b9fe1/src/market/WildcatMarket.sol#L55) function. During this process, the `_getUpdatedState()` function is invoked to retrieve the current state of the market: This call returns the `MarketState` after accruing interest, if any has accumulated. The `_getUpdatedState` function first checks for any expired withdrawals and then attempts to accrue the [interest](https://github.com/code-423n4/2024-08-wildcat/blob/fe746cc0fbedc4447a981a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Evade Sanction Escrow by transferring tokens to another address

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-191
- **Submitter:** ayeslick
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/191
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-191.md

## Brief Summary

The `_transfer()` doesn’t check if the sender or receiver is sanctioned. Neither does the onTransfer hook which enforces access control based on lender credentials but doesn’t check for sanctions. Sanctions are only checked when someone tries to withdraw using the `_executeWithdrawal()`. If the account is sanctioned, their assets go to escrow. Otherwise, the assets go straight to the account. This oversight could allow sanctioned lenders to transfer tokens to non-sanctioned addresses. This means a sanctioned lender can avoid escrow by transferring their tokens to an address that isn’t sanctioned, allowing them to withdraw funds without any issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Borrower Can Update Market Parameters After Withdrawal Initiation Leading to Incorrect Asset Claims

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-192
- **Submitter:** ayeslick
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/192
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-192.md

## Brief Summary

The borrower has the ability to change market parameters such as annualInterestBips (annual interest rate) and reserveRatioBips (reserve ratio) at any time using the setAnnualInterestAndReserveRatioBips() in WildcatMarketConfig.sol. If the borrower changes these parameters after lenders have initiated withdrawals but before the withdrawals are processed, this can lead to discrepancies between the burned tokens and the assets claimable. This behavior can result in users receiving less (or more) assets than they expected based on the calculations at the time of withdrawal initiation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_99_group

# no access control on pushProtocolFeeBipsUpdates

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-172
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/172
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-172.md

## Brief Summary

no access control on pushProtocolFeeBipsUpdates

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# deployMarket and deployMarketAndHooks functions could potentially be front-run

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-174
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/174
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-174.md

## Brief Summary

deployMarket and deployMarketAndHooks functions could potentially be front-run

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Potential for DOS

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-189
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/189
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-189.md

## Brief Summary

Potential for DOS

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential for Front-Running

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-190
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/190
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-190.md

## Brief Summary

Potential for Front-Running

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# check for zero address in market

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-209
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/209
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-209.md

## Brief Summary

check for zero address in market

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# should check the creation if the address code size is non-zero

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-232
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/232
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-232.md

## Brief Summary

should check the creation if the address code size is non-zero

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_64_group

# No check for from and msg.sender are different in transferfrom

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-257
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/257
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-257.md

## Brief Summary

No check for from and msg.sender are different in transferfrom

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# wrong implementation of registerBorrower.

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-312
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/312
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-312.md

## Brief Summary

wrong implementation of registerBorrower.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe casting from uint256 to uint32 could cause setCredential function in LenderStatus library to give wrong lastApprovalTimestamp value/Overflow.

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-24
- **Submitter:** codexNature
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/24
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-24.md

## Brief Summary

**Description:** In the `LenderStatus::setCredential` function, the `timestamp` parameter is cast to `uint256` and then used to update the user's status in `lastApprovalTimestamp`, which is cast to `uint32`. `LenderStatus.sol#L50-L55` **Impact:** Users credentials will not be set because the `lastApprovalTimestamp` property of `LenderStatus` struct will always return zero/overflow.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Closing a market does not trigger `onSetAnnualInterestAndReserveRatioBips` hook.

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-125
- **Submitter:** deadrxsezzz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/125
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-125.md

## Brief Summary

Closing a market does not trigger `onSetAnnualInterestAndReserveRatioBips` hook.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# FixedTermLoanHooks has DoS With Block Gas Limit in grantRoles Function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-115
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/115
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-115.md

## Brief Summary

Detailed description of the impact of this finding. Issue Type: Denial of Service (DoS) The function grantRoles uses an unbounded for loop to iterate over the accounts array, granting roles to multiple addresses. If the accounts array is too large, it could lead to a situation where the gas limit for the block is exceeded, causing a denial of service (DoS) for this function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_33_group

# LenderStatus has Unencrypted Private Data On-Chain

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-144
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/144
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-144.md

## Brief Summary

Detailed description of the impact of this finding. In the LenderStatus struct, the contract stores information such as the lastApprovalTimestamp, lastProvider, and canRefresh status. While this data is not sensitive like private keys or personal identifiers, it may still be considered sensitive depending on the application. Storing such information on-chain makes it publicly accessible, which could expose the lender’s status and interaction history to anyone. This could potentially be exploited for profiling users or identifying patterns in their on-chain activity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# WildcatArchController contract has Dependence on Predictable Environment Variable in Inherited completeOwnershipHandover Function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-21
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/21
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-21.md

## Brief Summary

Detailed description of the impact of this finding. Title: WildcatArchController contract has Dependence on Predictable Environment Variable in Inherited completeOwnershipHandover Function • Severity: Medium • Impact: WildcatArchController contract has Dependence on Predictable Environment Variable in Inherited completeOwnershipHandover Function. • Status: Unresolved • File: WildcatArchController.sol • Lines Affected: 5 WildcatArchController contract has Dependence on Predictable Environment Variable in Inherited completeOwnershipHandover Function. Issue Title: Dependence on Predictable Environment Variable in Inherited Function Contract: WildcatArchController Function Name: completeOwnersh...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# ReentrancyGuard has Storage Collision - Write to Arbitrary Storage Location

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-214
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/214
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-214.md

## Brief Summary

ReentrancyGuard has Storage Collision - Write to Arbitrary Storage Location

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# LibStoredInitCode has Denial of Service (DoS) with Block Gas Limit

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-237
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/237
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-237.md

## Brief Summary

LibStoredInitCode has Denial of Service (DoS) with Block Gas Limit

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_92_group

# AccessControlHooks contract has Unchecked Function Call in status.setCredential in _setCredentialAndEmitAccessGranted function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-32
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/32
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-32.md

## Brief Summary

Detailed description of the impact of this finding. Title: AccessControlHooks contract has Unchecked Function Call in status.setCredential in _setCredentialAndEmitAccessGranted function. Severity: Medium Impact: Incorrect Credential Updates Issue Type: Unchecked Function Call Status: Unmitigated Functions: _setCredentialAndEmitAccessGranted() & _grantRole(). In the function _setCredentialAndEmitAccessGranted, the line status.setCredential(provider, credentialTimestamp) calls the setCredential function, which updates the lender’s status. This call is unchecked, meaning there is no validation of its success, and the contract proceeds without ensuring the status update is correct. Unchecked fu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_30_group

# WildcatSanctionsSentinel has Transaction Order Dependence Sanction Overrides in overrideSanction function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-83
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/83
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-83.md

## Brief Summary

Detailed description of the impact of this finding. The sanction override mechanism in overrideSanction() could be subject to transaction-order dependence (ToD). A borrower could attempt to front-run another transaction and maliciously override a sanction or remove an override, leading to unexpected behavior. This could allow an attacker to manipulate the sanctions list in their favor by taking advantage of the order in which transactions are mined.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_21_group

# AccessControlHooks has DoS With Block Gas Limit Potential issue when looping through a large number of pull providers in getLenderStatus function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-85
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/85
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-85.md

## Brief Summary

Detailed description of the impact of this finding. The contract loops through all the pull providers in the _loopTryGetCredential function, which could become a gas-intensive operation if there are a large number of role providers. This can lead to a denial of service (DoS) attack if the gas limit is exceeded when trying to check credentials, effectively preventing credential validation or access control checks from completing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_52_group

# AccessControlHooks has Unchecked Call Return Value in _tryGetCredential the staticcall return value is not checked properly

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-88
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/88
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-88.md

## Brief Summary

Detailed description of the impact of this finding. The call to getLenderStatus function which is via the _tryGetCredential function, a staticcall is made to check a provider’s credential for a user. However, the return value of the staticcall is not properly checked, meaning if the staticcall fails, the contract proceeds as if it succeeded. This could lead to incorrect access being granted or denied based on incomplete or incorrect data. Issue Type: Unchecked Call Return Value

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_42_group

# Invalidation Looping Logic

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-314
- **Submitter:** eth_sol
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/314
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-314.md

## Brief Summary

Invalidation Looping Logic

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Hardcoded Protocol Fee Cap in validateFees Function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-159
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/159
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-159.md

## Brief Summary

The validateFees function in the HooksFactory.sol contract contains a hardcoded protocol fee cap of 1000. Hardcoding such values limits flexibility and can create maintenance challenges if the fee cap needs to be adjusted in the future. Vulnerability Details: Within the validateFees function, the protocol fee cap is set as a hardcoded value of 1000. This use of a "magic number" makes the code less flexible and harder to maintain. A constant or dynamically configurable parameter would allow easier adjustments and clearer understanding of the fee limit. Impact: Having this hardcoded value reduces the contract's adaptability. If the protocol evolves and the fee cap needs to be changed, the con...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Risk in AccessControlHooks Due to Single Borrower Control in Critical Functions

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-162
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/162
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-162.md

## Brief Summary

In the AccessControlHooks.sol contract, the onlyBorrower modifier is used for critical functions. If the borrower's account is compromised, it could lead to significant risks since sensitive functions like blockFromDeposits and addRoleProvider rely solely on this account's control. Vulnerability Details: The onlyBorrower modifier restricts critical operations to a single account. If this account is compromised, an attacker can block/unblock deposits or modify roles, impacting key aspects of the market. The borrower typically has a single account, so there's no additional protection beyond this. Impact: Compromising the borrower account could give an attacker full control over critical funct...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_72_group

# Lack of Circuit Breaker in WildcatMarketBase.sol for Emergency Halt

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-195
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/195
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-195.md

## Brief Summary

WildcatMarketBase.sol does not implement a circuit breaker, exposing the contract to unprotected execution in case of emergencies. This base contract is used across all Wildcat markets, increasing the risk of widespread issues during critical failures. ________________________________________ Vulnerability Details: The absence of a circuit breaker pattern means the contract cannot be paused in emergencies. This could allow potentially harmful operations to continue even if a bug or exploit is discovered, putting user funds at risk. ________________________________________ Impact: Without a circuit breaker, all Wildcat markets are vulnerable to attacks or bugs that could exploit ongoing oper...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Access Control in depositUpTo Function in WildcatMarket.sol

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-211
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/211
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-211.md

## Brief Summary

The depositUpTo function in WildcatMarket.sol lacks proper access control. This function allows external users to deposit assets and mint market tokens, but does not have sufficient access control (such as onlyOwner, onlyBorrower, or similar) to restrict who can call this function. Vulnerability Details The depositUpTo function does not use any access control modifiers, and the sphereXGuardExternal modifier's role is unclear. As a result, unauthorized users may potentially deposit assets and mint tokens without proper permissions. function depositUpTo( uint256 amount ) external virtual sphereXGuardExternal returns (uint256 /* actualAmount */) { return _depositUpTo(amount); } Impact Without...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Non-Configurational Functions in WildcatMarketConfig: nukeFromOrbit Should Be Moved to a Security-Specific Contract

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-97
- **Submitter:** ferdaozdemirsonmez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/97
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-97.md

## Brief Summary

The WildcatMarketConfig contract currently contains functions, such as nukeFromOrbit, that are not related to configuration but rather to security and account management. These functions are out of place in a configuration contract and should be relocated to a more appropriate contract, such as WildcatMarketSecurity, if necessary. Additionally, the WildcatMarketConfig contract’s inheritance from WildcatMarketBase seems unnecessary and introduces confusion. Vulnerability Details The WildcatMarketConfig contract is designed to handle configuration settings like total supply, interest rates, and reserve ratios. However, the presence of non-configurational functions like nukeFromOrbit, which in...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe casting in '_calculateTemporaryReserveRatioBips()' function from 'uint256' to 'uint16' leads to loss of precision and data

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-55
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/55
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-55.md

## Brief Summary

'originalReserveRatioBips' is cast to 'uint16', meaning that if it exceeds 65,535 bips (which is 655.35%), it will be truncated. 'boundRelativeDiff' is similarly cast, so if the value of boundRelativeDiff exceeds 65,535, it will also be truncated. If the values of the reserve ratios or the differences in interest rates can reasonably be expected to exceed 65,535, this downcasting can introduce significant risks to the accuracy and correctness of the function's logic.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unprotected Initializer allows arbitrary contract deployment and could be exploited

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-56
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/56
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-56.md

## Brief Summary

Unprotected initializer in 'deployInitCode', 'createWithStoredInitCode', 'create2WithStoredInitCode', and their overloads. None of these functions implement access control, which means any contract or address interacting with this library could potentially call these functions and deploy contracts. This is dangerous in contexts where arbitrary contract deployment should be restricted. Without proper access control, these deployment functions pose significant risks to the security and stability of the entire contract. It is critical to ensure that only authorized parties can invoke them to prevent malicious deployments, resource exhaustion, or unintended interactions with other contracts.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Nelwly encoded `RoleProvider` in `AccessControlHooks::addRoleProvider` can never be a `pullProvider`

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-126
- **Submitter:** josephxander
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/126
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-126.md

## Brief Summary

The `AccessControlHooks::addRoleProvider` allows a `borrower` to encode a new `RoleProvider` `providerAddress` to its market. The conditions to encode a new `providerAddress` if invalid (`if (provider.isNull()) {`) performs a redundant check to affirm that the `providerAddress` is a `pullProvider`. `bool isPullProvider = IRoleProvider(providerAddress).isPullProvider()` The only way this can happen is if the method `RoleProvider::setPullProviderIndex` is called on this new `providerAddress` should it be decided that it should be a `pullProvider`. And it is set to `internal` visibility and not implemented anywhere else.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect input data size in `WildcatMarketBase::_createEscrowForUnderlyingAsset` will interrupt the creation of new escrow contract for sanctioned address

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-137
- **Submitter:** josephxander
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/137
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-137.md

## Brief Summary

In `WildcatMarketBase::_createEscrowForUnderlyingAsset` a yul `call()` is made to the `sentinelAddress` in order to create an escrow for a sanctioned address. This yul `call()` comprises of 7 parameters. inputDataOffset and inputDataSize: Specify where in memory the input data is stored and its length respectively. outputDataOffset and outputDataSize: Specify where in memory to store the returndata and its length respectively. The issue here is with the `inputDataSize` provided in the `WildcatMarketBase::_createEscrowForUnderlyingAsset`. A hardcoded `0x64` is used here. This means the total input data is `100 bytes`. However this might not be. 4 parameters make up the input data: `function...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Keyspace Collision Between Account and Escrow Contract in Sanction Overrides Mapping could lead to Unintended behaviour

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-260
- **Submitter:** la-arana-inteligente
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/260
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-260.md

## Brief Summary

Keyspace Collision Between Account and Escrow Contract in Sanction Overrides Mapping could lead to Unintended behaviour

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential memory corruption

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-219
- **Submitter:** noured23
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/219
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-219.md

## Brief Summary

Potential memory corruption

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_45_group

# Malformed External Calls

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-256
- **Submitter:** noured23
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/256
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-256.md

## Brief Summary

Malformed External Calls

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Incorrect Error Handling in Assembly

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-261
- **Submitter:** noured23
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/261
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-261.md

## Brief Summary

Incorrect Error Handling in Assembly

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reentrancy Guard Prevent Internal Calls Between Protected Functions

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-274
- **Submitter:** noured23
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/274
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-274.md

## Brief Summary

- Developers cannot have protected functions call other protected functions internally without causing a revert. - Developers may end up duplicating code to avoid internal calls, leading to less maintainable and more error-prone contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# Bug Report: Insufficient Input Validation in `WildcatMarket.sol`

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-299
- **Submitter:** obingo76
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/299
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-299.md

## Brief Summary

Bug Report: Insufficient Input Validation in `WildcatMarket.sol`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# OOG(out of gas) and Dos in registerBorrower function

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-249
- **Submitter:** pepoc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/249
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-249.md

## Brief Summary

The registerBorrower function, because it lacks a limit on the number of borrowers that can be added, when called in a loop for a large number of borrowers, can consume a significant amount of gas, potentially exceeding the block gas limit. This can lead to DoS attacks and MemoryOOG.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Persistent Contract State and Address After Block Reorganization by "create" opcode (_deployHooksInstance function)

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-266
- **Submitter:** pepoc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/266
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-266.md

## Brief Summary

Persistent Contract State and Address After Block Reorganization by "create" opcode (_deployHooksInstance function)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Borrower Can Cause Intentional Out-of-Gas Reverts to Block Withdrawals of Users Transferred Debt Tokens

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-220
- **Submitter:** rabTAI
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/220
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-220.md

## Brief Summary

Borrower Can Cause Intentional Out-of-Gas Reverts to Block Withdrawals of Users Transferred Debt Tokens

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Borrower Can Intentionally Avoid Delinquency Fees by Closing Market Early

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Submission:** V-222
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-wildcat-validation/issues/222
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/submissions/raw/V-222.md

## Brief Summary

Borrower Can Intentionally Avoid Delinquency Fees by Closing Market Early

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_58_group
