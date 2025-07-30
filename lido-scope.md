# Do NOT report Automated Findings / Publicly Known Issues / Security Considerations

## Reported Findings must be novel and NOT included in any of the Publicly Known Issues / Security Considerations listed below


> **Note for C4 wardens:**  
> Anything included in this **Automated Findings / Publicly Known Issues** section is considered a publicly known issue and is **ineligible for awards** - Do NOT report.

---

## Assumptions

- It is assumed that all of the roles in the contracts are assigned correctly concerning the permissions granted.
- It is assumed that **EL Rewards Stealing** penalty is reported timely before Node Operators exit their validators and claim back bond tokens.


## All Trusted Roles in the Protocol

➡️ [More on roles here](https://hackmd.io/@lido/csm-v2-spec#Roles-to-actors-mapping)

| Role                      | Description |
|---------------------------|-------------|
| **Lido DAO**              | Admin functions, smart contract upgrades |
| **CSM Committee multisig**| EL Rewards Stealing penalty reporting and cancellation, Bond curve set, emergency contracts pause via GateSeal, end referral season in VettedGate |

---

## Additional Assumptions

- It is assumed that Oracles deliver valid **Merkle Trees** for rewards distribution and strikes.
- Bond tokens are stored in the form of **stETH**. Hence, it is assumed that Node Operators accept all of the risks associated with stETH holding.

➡️ [More on the Known Issues and Security Considerations](https://hackmd.io/@lido/csm-v2-spec#Security-considerations)

---

## Overview

**Lido Community Staking Module (CSM)** is a permissionless module allowing community stakers to operate Ethereum validators with lower entry costs.

- Stakers provide **stETH bonds**, serving as security collateral.
- Stakers receive rewards in the form of bond rebase and staking rewards (including execution layer rewards).
- Rewards are **socialized across Lido’s staking modules**.

# Security Considerations -- NOT ADMISSIBLE AS FINDINGS

## Bond Exposure to Negative stETH Rebase
- The bond stored in **stETH** inherits all stETH features, including the possibility of a negative rebase.  
- The effective bond amount (in ETH) will decrease if a negative rebase occurs.  
- **Risks:**
  - Large Node Operators may end up with unbonded keys.
  - The effective bond could fall below the required amount.
  - Node Operators may lose part of their rewards.

---

## Malicious Oracles Can Steal All Unclaimed CSM Rewards
- A **single updatable Merkle tree** approach to rewards distribution allows malicious Oracles to collude and:
  - Submit a Merkle tree version allocating all rewards to a single Node Operator (created by the attackers).
- **Worst-case:** All unclaimed rewards stored on the CSM contract are claimed by one malicious Node Operator.

---

## Malicious Oracles Can Assign Inappropriate Strikes to CSM Validators
- Malicious Oracles could submit a Merkle tree indicating an inappropriate number of strikes for a Node Operator.
- **Worst-case:**  
  - Some validators are incorrectly ejected.  
  - Node Operators' bonds are penalized.  
- This risk is heightened if the Oracles use the permissionless validator ejection method before **CSEjector.sol** is paused via GateSeal.

---

## TE Fees Might Be Confiscated After Voluntary Exit
- With **EIP-7002 Triggerable Withdrawals (TE)**:
  - TE requests can still be made for validators that have voluntarily exited.
  - CL ignores it, but EL accepts the request, collects a fee, and doesn’t report status.
- **Edge Case:**
  - If a validator exits after `allowedExitDelay`, and TE is triggered:
    - CSM cannot detect TE was unnecessary.
    - TE fee is confiscated from the Node Operator bond.
- **Assumption:** This can only occur due to a bug in the Lido DAO-maintained TE bot.  
- **Mitigation:** If a bug causes this, the TE fee can be reimbursed from the Lido treasury via DAO decision.

---

## Possible 'Resell' of Beneficial Node Operator Types
- With Node Operator types and **VettedGate** benefits:
  - There is a risk of selling Node Operator ownership or private keys to claim benefits.
- **Mitigation:**
  - Require the eligible address to be the ultimate owner.
  - Limit benefits for custom Node Operator types to reduce abuse.

---

# Known Issues - DO NOT REPORT

## Permissionless Withdrawal Reporting Vulnerability
- To distinguish partial vs. full withdrawals for permissionless reporting, the following condition is used (similar to Rocket Pool):

```solidity
if (!witness.slashed && gweiToWei(witness.amount) < 8 ether) {
    revert PartialWitdrawal();
}
````

* **Attack Vector:**

  1. Wait for a full validator withdrawal and sweep.
  2. Ensure no proof is provided for at least one sweep cycle (\~8 days with 1M validators).
  3. Deposit 1 ETH (slashed) or 8 ETH (non-slashed).
  4. Wait for sweep.
  5. Provide proof of the last withdrawal.

* **Impact:**

  * Node Operator bond penalized for 32 ETH minus the additional deposit.
  * All ETH remains in the protocol.
  * Only consequence: bond accounting inconsistency.

---

## Resolution

* **No protocol loss** and high cost of attack (1 or 8 ETH).
* **Mitigation:** Acknowledge the possibility and be prepared to propose a DAO vote to correct bond accounting if it occurs.



### [](https://hackmd.io/@lido/csm-v2-spec#Bond-exposure-to-negative-stETH-rebase "Bond-exposure-to-negative-stETH-rebase")Bond exposure to negative stETH rebase

The bond stored in stETH inevitably inherits all stETH features, including the possibility of a negative rebase. The effective bond amount (counted in ETH) will decrease in case of a negative rebase. This can lead to the case when a relatively large Node Operator might end up with unbonded keys. Also, it might result in an effective bond being lower than the bond required. Hence, Node Operators will lose part of their rewards.

### [](https://hackmd.io/@lido/csm-v2-spec#Malicious-Oracles-can-steal-all-unclaimed-CSM-rewards "Malicious-Oracles-can-steal-all-unclaimed-CSM-rewards")Malicious Oracles can steal all unclaimed CSM rewards

A single updatable Merkle tree approach to the rewards distribution allows malicious Oracles to collude and submit a version of the Merkle tree, indicating that all rewards should be allocated to a single Node Operator (previously created by malicious actors). The worst-case scenario is when all unclaimed rewards stored on the CSM contract will be available for claim by a single Node Operator.

### [](https://hackmd.io/@lido/csm-v2-spec#Malicious-Oracles-can-assign-an-inappropriate-number-of-strikes-to-the-CSM-validators "Malicious-Oracles-can-assign-an-inappropriate-number-of-strikes-to-the-CSM-validators")Malicious Oracles can assign an inappropriate number of strikes to the CSM validators

A single updatable Merkle tree approach to the strikes allows malicious Oracles to collude and submit a version of the Merkle tree, indicating an inappropriate number of strikes for the CSM Node Operator. In the worst-case scenario, some validators might get ejected due to that should the Oracles also utilize the permissionless method for the validator ejection due to strikes before the `CSEjector.sol` will be paused using `GateSeal`. The impact in this case is a certain number of validators being inappropriately ejected and the corresponding Node Operators bond will be penalized.

### [](https://hackmd.io/@lido/csm-v2-spec#TE-fees-might-be-confiscated-even-if-the-validator-had-exited-voluntarily-after-allowedExitDelay "TE-fees-might-be-confiscated-even-if-the-validator-had-exited-voluntarily-after-allowedExitDelay")TE fees might be confiscated even if the validator had exited voluntarily after `allowedExitDelay`

The current design of the Triggerable Withdrawals ([EIP-7002](https://eips.ethereum.org/EIPS/eip-7002)) allows to request Triggerable Withdrawal (Triggerable Exit) even for the validators that were already exited. Even though the actual request will be [ignored](https://github.com/ethereum/consensus-specs/blob/7bf43d1bc4fdb91059f0e6f4f7f0f3349b144950/specs/electra/beacon-chain.md#execution-layer-withdrawal-requests) on CL, EL request will be accepted, fee will be taken, and no information about the status of the request will be provided on EL.

This results in a small edge case when the validator was not exited within `allowedExitDelay`, then exited voluntarily, and then TE was invoked for the validator with the fee reported to CSM. In this case, CSM will not be able to detect that TE was not actually required and will confiscate the TE fee from the Node Operator bond.

Since there is no motivation for the external actor to request TE for Lido validators, it is assumed that this edge case can only occur due to the bug in the TE bot developed and maintained by Lido DAO contributors. Should there be a bug in the bot code, the TE fee can always be transferred back to the Node Operator's bond from the Lido treasury based on the Lido DAO decision.

### [](https://hackmd.io/@lido/csm-v2-spec#Possible-resell-of-the-beneficial-Node-Operator-types "Possible-resell-of-the-beneficial-Node-Operator-types")Possible 'resell' of the beneficial Node Operator types

With the introduction of Node Operator types and the ability to claim beneficial type using `VettedGate` (applies to both addition to the vetted list and referral program participation), it becomes more likely that Node Operators will consider a 'resell' of the benefits. This is partially mitigated by the requirement that the eligible address be the ultimate owner of the Node Operator. Hence, it is assumed that Node Operators will think twice before giving away Node Operator ownership to a third party for claiming benefits, since the bond will be at risk in this case. However, this does not fully mitigate the issue. The private key of the eligible address can still be sold. Given that, it is proposed to limit benefits for the custom Node Operator types to avoid massive abuse.

** DO NOT REPORT ABOVE KNOWN ISSUES **
