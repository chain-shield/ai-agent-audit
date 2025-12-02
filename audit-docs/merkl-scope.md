## Publicly known issues

_Anything included in this section is considered a publicly known issue and is therefore ineligible for awards._

Merkl operates as a centralized solution. Therefore, any issues related to the contract owner's administrative access should be considered out of scope.
Additionally, the following assumptions underpin the system's security model:

- The dispute resolution mechanism functions as intended
- Active monitoring bots continuously verify reward distributions through Merkl

We acknowledge a known risk: When users designate a smart contract as their reward recipient and pass data to it, their rewards are potentially vulnerable if:

- The recipient contract is misconfigured or malicious
- The user submits incorrect parameters (such as executing a swap without slippage protection)

In such cases, the recipient contract could redirect or capture the user's rewards.

# Overview

Merkl is a DeFi incentive platform that **connects liquidity providers with protocols** looking to boost activity and engagement.

- For Protocols: Launch, manage, and customize growth campaigns to attract liquidity, track user engagement, and distribute incentives without the usual operational burden.

- For Users: Earn rewards or points by participating in incentive campaigns.

At its core, Merkl operates on an offchain engine that processes both onchain and offchain data to compute rewards and points for campaigns.

# Scope

### Files in scope


| File   | nSLOC |
| ------ | ----- |
|[contracts/DistributionCreator.sol](https://github.com/code-423n4/2025-11-merkl/blob/main/contracts/DistributionCreator.sol)| 333 |
|[contracts/Distributor.sol](https://github.com/code-423n4/2025-11-merkl/blob/main/contracts/Distributor.sol)| 271 |
|**Totals**| **604** |

*For a machine-readable version, see [scope.txt](https://github.com/code-423n4/2025-11-merkl/blob/main/scope.txt)*

### Files out of scope

| File         |
| ------------ |
|[contracts/AccessControlManager.sol](https://github.com/code-423n4/2025-11-merkl/blob/main/contracts/AccessControlManager.sol)|
|[contracts/Disputer.sol](https://github.com/code-423n4/2025-11-merkl/blob/main/contracts/Disputer.sol)|
|[contracts/DistributionCreatorWithDistributions.sol](https://github.com/code-423n4/2025-11-merkl/blob/main/contracts/DistributionCreatorWithDistributions.sol)|
|[contracts/ReferralRegistry.sol](https://github.com/code-423n4/2025-11-merkl/blob/main/contracts/ReferralRegistry.sol)|
|[contracts/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/contracts/interfaces)|
|[contracts/mock/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/contracts/mock)|
|[contracts/partners/middleman/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/contracts/partners/middleman)|
|[contracts/partners/tokenWrappers/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/contracts/partners/tokenWrappers)|
|[contracts/struct/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/contracts/struct)|
|[contracts/utils/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/contracts/utils)|
|[scripts/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/scripts)|
|[test/\*\*.\*\*](https://github.com/code-423n4/2025-11-merkl/tree/main/test)|
| Totals: 60 |

# Additional context

## Areas of concern (where to focus for bugs)

Primary Security Concerns:

### 1. Campaign Pre-Deposit Protection

Can an address exploit or access funds pre-deposited by another address without authorization?

### 2. Reward Claim Integrity

Do rewards consistently reach their intended recipient through all claim paths (the claimant's address, a recipient specified in the call parameters, or a user-defined default recipient)?

### 3. Unauthorized Access and Fund Theft

Any scenario enabling unauthorized assumption of user roles or theft of funds constitutes a valid issue, including:

- Pre-deposited funds in the distributionCreator contract
- Idle funds in the distributor contract


## Main invariants

### Guardian Restrictions
The Guardian role must not have the ability to steal user funds or perform actions that result in fund reallocation.

### Reward Finality
Once rewards are earned and recorded in a Merkle root, they are immutably assigned to the recipient. These rewards cannot be revoked or redirected within the current Merkle root, except through:
- An explicit reallocation by authorized parties
- Token recovery executed by the admin address

### Campaign Creator Autonomy

Campaign creators retain full control over:
- End-to-end campaign management
- Their pre-deposited funds
- The ability to recover ownership of their funds at any time
- They can revoke any role or allowance they give at anytime

## All trusted roles in the protocol

The contracts include three main trusted roles:

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Governor                          | - Operated via multisignature wallet<br>- Possesses administrative rights over the distribution contracts and creator                |
| Guardian                             | - May be held by EOAs<br>- Responsible for operational tasks such as whitelisting of tokens and toggling operator permissions                       |
| Updater Address | - Authorized to update Merkle roots<br>- May be EOAs as a dispute period exists as a safeguard to prevent malicious root updates before they are finalized |
