Title: FAssets | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/overview

Markdown Content:
FAssets is a trustless, over-collateralized bridge connecting non smart contract networks to Flare. It enables the creation of wrapped tokens (`FAssets`) for assets like BTC, DOGE and XRP. These tokens can participate in Flare's DeFi ecosystem or be redeemed for their original assets.

FAssets are powered by Flare's enshrined data protocols:

*   **[Flare Time Series Oracle (FTSO)](https://dev.flare.network/ftso/overview):** Provides decentralized price feeds.
*   **[Flare Data Connector (FDC)](https://dev.flare.network/fdc/overview):** Verifies offchain actions, such as transactions on other blockchains.

Each FAsset is backed by a mix of collateral, including:

1.   Stablecoin or ETH collateral.
2.   FLR (Flare's native token) or SGB (Songbird's native token) collateral.

Agents and a community-provided collateral pool ensure trustlessness through over-collateralization.

FAsset Workflow[​](https://dev.flare.network/fassets/overview#fasset-workflow "Direct link to FAsset Workflow")
---------------------------------------------------------------------------------------------------------------

Anyone on the Flare blockchain can mint FAssets, which are wrapped versions of original tokens from other blockchains, known as underlying networks. The original tokens from these chains, such as Ripple (XRPL), Dogecoin (DOGE), Bitcoin (BTC), and Litecoin (LTC), are referred to as underlying assets. For example, the FAsset version of Bitcoin is known as FBTC.

### Minting[​](https://dev.flare.network/fassets/overview#minting "Direct link to Minting")

*   A user (minter) selects an agent and pays a fee to reserve collateral.
*   The user sends the underlying asset (e.g., BTC) to the agent.
*   The FDC verifies the transaction.
*   The equivalent FAssets (e.g., FBTC) are minted as ERC-20 tokens on Flare.

### Usage[​](https://dev.flare.network/fassets/overview#usage "Direct link to Usage")

Minted FAssets can be used in DeFi applications on Flare or bridged to other chains.

### Redeeming[​](https://dev.flare.network/fassets/overview#redeeming "Direct link to Redeeming")

Users can redeem FAssets for the original underlying assets at any time.

Key Participants[​](https://dev.flare.network/fassets/overview#key-participants "Direct link to Key Participants")
------------------------------------------------------------------------------------------------------------------

### Agents[​](https://dev.flare.network/fassets/overview#agents "Direct link to Agents")

Agents manage the infrastructure and operations of the FAssets system, including:

*   Holding the underlying assets.
*   Providing collateral for minting and redemption.
*   Redeeming underlying assets for users.

Each agent is verified through governance and uses the following addresses on the native chain:

*   **Work Address:** A hot wallet for executing operations.
*   **Management Address:** A cold wallet for secure administrative actions.

Agents must comply with the **backing factor**, which ensures sufficient collateral is locked to back FAssets.

### Users[​](https://dev.flare.network/fassets/overview#users "Direct link to Users")

Users interact with the system by:

*   **Minting:** Depositing underlying assets to mint FAssets.
*   **Redeeming:** Exchanging FAssets for the original underlying assets.

Eligibility:

*   No restrictions—anyone can mint or redeem FAssets.

### Collateral Providers[​](https://dev.flare.network/fassets/overview#collateral-providers "Direct link to Collateral Providers")

Collateral providers supply native FLR tokens to an agent's collateral pool and earn a share of minting fees as long as their tokens remain locked.

### Liquidators[​](https://dev.flare.network/fassets/overview#liquidators "Direct link to Liquidators")

Liquidators maintain system health by:

*   Burning FAssets in exchange for collateral when an agent's collateral drops below the required minimum.
*   Earning rewards, including premiums on the collateral received.

Eligibility:

*   Open to all—anyone can become a liquidator.

### Challengers[​](https://dev.flare.network/fassets/overview#challengers "Direct link to Challengers")

Challengers monitor agents for illegal transactions that reduce collateral below the backing factor. They:

*   Submit proof of illegal actions to the system.
*   Earn rewards from the agent's vault upon successful challenges.

If an agent is found in violation, they enter **full liquidation**, permanently restricting them from new minting operations.

Core Vault[​](https://dev.flare.network/fassets/overview#core-vault "Direct link to Core Vault")
------------------------------------------------------------------------------------------------

The **Core Vault (CV)** is a specialized FAsset system component that enhances capital efficiency by allowing agents to store underlying assets without requiring additional collateral. Each asset type has its own dedicated Core Vault, which is managed by a multisig account on the underlying network under formal governance oversight.

### Key Features[​](https://dev.flare.network/fassets/overview#key-features "Direct link to Key Features")

*   **Collateral Efficiency:** Agents transferring assets to the CV free up collateral, allowing them to mint additional FAssets or withdraw funds.
*   **Redemption Support:** The CV ensures that underlying assets are available for redemptions, reducing reliance on individual agents.
*   **Security & Governance:** A multisig setup controls the vault, and governance can pause in case of security concerns.

### Core Vault Implementation[​](https://dev.flare.network/fassets/overview#core-vault-implementation "Direct link to Core Vault Implementation")

On networks without smart contracts (e.g., XRP Ledger), the Core Vault is a **multisig account** managed by signers authorized by Flare governance. Movements of funds require multiple signatures and follow formal agreements, not individual agent control.

### Agent vs. Core Vault Ownership[​](https://dev.flare.network/fassets/overview#agent-vs-core-vault-ownership "Direct link to Agent vs. Core Vault Ownership")

*   **Agents:** Hold and control their own underlying assets in wallets as part of collateral.
*   **Core Vault:** Holds pooled assets that no single agent owns; agents can request assets but cannot directly control the vault. This improves capital efficiency and liquidity for the system.

Title: Minting | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/minting

Markdown Content:
Minting FAssets is the process of wrapping underlying tokens from connected blockchains into FAssets to be used on the Flare blockchain. Any user can mint FAssets.

Minting Process[​](https://dev.flare.network/fassets/minting#minting-process "Direct link to Minting Process")
--------------------------------------------------------------------------------------------------------------

This is the summary of the minting process:

### 1. Reserving Collateral[​](https://dev.flare.network/fassets/minting#1-reserving-collateral "Direct link to 1. Reserving Collateral")

The minter chooses an agent from the publicly available [agent list](https://dev.flare.network/fassets/overview#agents). The choice is based on the minting fee or the amount of free collateral, which must be enough to back the amount to be minted.

The minter sends to the Asset Manager contract a collateral reservation transaction (CRT). The CRT includes:

*   The address of the chosen agent.
*   The amount to mint, which must be a positive integer of [lots](https://dev.flare.network/fassets/minting#lots).
*   The [collateral reservation fee (CRF)](https://dev.flare.network/fassets/minting#fees) to compensate for the locked collateral.
*   The executor's address, if the minter is not the executor.
*   The executor's fee, if the minter is not the executor.

The Asset Manager contract locks the agent's collateral in the amount needed to back the whole minting until the underlying payment is proved or disproved. The collateral reservation response is an event issued by the contract, which includes:

*   The agent's address to which the minter must send funds on the underlying chain.
*   The amount to be paid on the underlying chain, which corresponds to the amount to be minted plus the agent's fee.
*   The payment reference, which is a unique 32-byte number the minter must include as a memo in the payment on the underlying chain.
*   The last underlying block and the last underlying timestamp to pay. Valid payments occur either before the last block or before the last timestamp, both inclusive.
*   The executor's address, if the minter is not the executor.
*   The executor's fee, if the minter is not the executor.

The time to pay is measured both in the underlying chain's block numbers and block times because the underlying chain might halt for a long time. In this situation, the block numbers do not increment but the block timestamps do.

### 2. Underlying Payment[​](https://dev.flare.network/fassets/minting#2-underlying-payment "Direct link to 2. Underlying Payment")

After this event is emitted, the minter must pay the full underlying amount plus the fee to the agent on the underlying chain. This payment must be completed within a specified time limit.

### 3. Payment Proof[​](https://dev.flare.network/fassets/minting#3-payment-proof "Direct link to 3. Payment Proof")

Using the [Flare Data Connector](https://dev.flare.network/fdc/overview), the minter or executor proves the payment on Flare network.

### 4. Minting Execution[​](https://dev.flare.network/fassets/minting#4-minting-execution "Direct link to 4. Minting Execution")

After the payment is proved, the minter or executor executes the minting process, which sends FAssets to the minter's account.

When minting is executed, the [minting fee](https://dev.flare.network/fassets/minting#fees) is split between the agent and the pool:

*   The percentage split is set by the agent.
*   The agent's share increases the free balance on the agent's underlying address. The free balance is the part of the balance in an agent's underlying address that the agent can withdraw. It is composed of minting fees, redemption fees, and self-closed FAssets.
*   The pool share gets minted as FAssets and credited to the collateral pool contract.

After minting is complete, the Asset Manager creates a [redemption ticket](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue), which includes the mint amount and the name of the agent backing the minting.

Executor Role[​](https://dev.flare.network/fassets/minting#executor-role "Direct link to Executor Role")
--------------------------------------------------------------------------------------------------------

The execution of the minting process can be performed by an **executor**, an external actor such as a bot or service that monitors pending minting requests. Executors are incentivized to act quickly and correctly, but they hold no special permissions. If they fail to execute in time, the request may expire, and the minting must be restarted.

The executor:

*   Is nominated by the minter and gets paid by the minter.
*   Uses the Flare Data Connector to obtain valid payment proof.
*   Executes the minting with a valid payment proof.

Fees[​](https://dev.flare.network/fassets/minting#fees "Direct link to Fees")
-----------------------------------------------------------------------------

The following fees are paid to mint FAssets:

### Collateral Reservation Fee[​](https://dev.flare.network/fassets/minting#collateral-reservation-fee "Direct link to Collateral Reservation Fee")

The **collateral reservation fee (CRF)** is paid in native tokens by the minter at the same time the [collateral reservation](https://dev.flare.network/fassets/minting#minting-process) is made. The CRF is defined by governance as a percentage of the minted value, and the same fee applies to all agents.

The purpose of the CRF is to compensate the agent and collateral pool token (CPT) holders for the time their collateral is locked during the minting process.

*   If the minter does not pay on the underlying chain, the CRF is distributed to the agent and the pool in the same share as the minting fee.
*   If the minter successfully pays on the underlying chain, the CRF is also distributed to the agent and the pool in the same manner.

For underlying chains where proving payments takes longer, the CRF might be set higher to account for the extended lock-up time. The CRF percentage is defined by governance and may vary based on the performance of the underlying chain.

### Minting Fee[​](https://dev.flare.network/fassets/minting#minting-fee "Direct link to Minting Fee")

The **minting fee** is paid by the minter with the underlying currency as a percentage of the minted amount, and each agent can declare a different fee value. This fee is the main source of revenue for the agent and the CPT holders.

The minting fee is further divided in two shares:

This share remains in the agent's underlying account but is not marked as being in use. The agent can use this balance freely.

This share is minted as FAssets and sent to the [collateral pool](https://dev.flare.network/fassets/collateral#pool-collateral). The percentage of this share is defined by the agent and can be changed by the agent after a delay that provides time for minters to notice the change.

### Executor Fee[​](https://dev.flare.network/fassets/minting#executor-fee "Direct link to Executor Fee")

To incentivize reliable execution of minting requests, an **executor fee** may be included in the system.

The executor is the actor who submits the payment proof to the Asset Manager, finalizing the minting process.

*   The executor fee is paid by the minter when minting is executed.
*   This fee is optional and configurable within the system based on chain-specific governance parameters.
*   If set, the fee is denominated in FLR and transferred directly to the executor's address as part of the execution transaction.
*   Executors compete to be the first to execute minting and collect this fee, providing a decentralized execution layer.

This design ensures timely and reliable minting finalization without relying on a centralized party.

### Minting[​](https://dev.flare.network/fassets/minting#minting "Direct link to Minting")

The FAssets agent verifies the minter after the user completes the collateral reservation and pays the collateral reservation fee. The agent is responsible for confirming or rejecting the minter's status. If the agent does not respond within a certain timeframe, the minter has the option to cancel the reservation and receive a full refund of the collateral reservation fee.

To enable the agent to verify the minter, the collateral reservation must include the address (or multiple addresses, in the case of UTXO chains) from which the payment will be made. If multiple addresses are provided, all of them must be used for the payment.

Users must wait up to 60 seconds before they can cancel their request. If the agent accepts within this time, the user can proceed to mint by depositing the underlying assets. Therefore, it is important for the agent to respond quickly. If the agent does not respond in time, it will depend on whether the user is willing to wait; otherwise, the agent will simply miss the opportunity to mint, but there will be no loss of tokens.

When the agent rejects the minter's request or the minter decides to cancel, the minter will receive a refund of the collateral reservation fee, minus a small percentage (e.g., 5%) that is burned. This burned amount is designed to prevent abuse of the agent by stopping someone from repeatedly reserving collateral from a sanctioned address. If the burned percentage were zero, an attacker could exploit the system without any cost.

Payment Failure[​](https://dev.flare.network/fassets/minting#payment-failure "Direct link to Payment Failure")
--------------------------------------------------------------------------------------------------------------

To finalize the minting, the minter must pay the agent on the underlying chain and prove the payment was received. If the payment is not completed in the time frame defined by the underlying chain block and timestamp, the agent must prove nonpayment to release the locked collateral. After nonpayment is proved, the agent's collateral that was reserved by the [CRT](https://dev.flare.network/fassets/minting#minting-process) is released, and the agent receives the [CRF](https://dev.flare.network/fassets/minting#collateral-reservation-fee).

The [agent's registration process](https://dev.flare.network/fassets/overview#agents) verifies that the agent's underlying address does not purposefully block payments and illegally collects the CRF.

The following example shows proof of nonpayment.

Referenced payment nonexistence attestation type example.

Edge Cases[​](https://dev.flare.network/fassets/minting#edge-cases "Direct link to Edge Cases")
-----------------------------------------------------------------------------------------------

### Unresponsive minter[​](https://dev.flare.network/fassets/minting#unresponsive-minter "Direct link to Unresponsive minter")

After a successful payment, the minter might not provide the payment proof needed to complete the minting process. In this case, the agent can present the payment proof and execute minting at any time. FAssets are still transferred to the minter's account, and the agent's collateral becomes redeemable.

### Expired proof[​](https://dev.flare.network/fassets/minting#expired-proof "Direct link to Expired proof")

Proofs provided by the Flare Data Connector are available for only 24 hours, approximately. If neither the minter nor the agent presents the proof of payment or nonpayment within 24 hours, the regular minting process cannot continue, and the agent's collateral could be locked indefinitely.

In this case, the agent can still recover the collateral by buying it back with native tokens. The recovery is accomplished with the following procedure:

1.   Request the proof from the time when the deposit should have happened. The Flare Data Connector's answer will indicate that payments proofs are no longer available for that time.
2.   Provide the amount of FLR collateral equivalent to the price of the underlying assets that should have been deposited.
3.   Present the proof.

Because a successful deposit cannot be proven, the FAssets system burns the amount of collateral in native tokens provided by the agent. After the burn is complete, the rest of the agent's collateral is released, both from his vault and the collateral pool.

warning

Note that this procedure should be used only in rare cases because providing timely payment or nonpayment proofs is always more advantageous for agents.

Duration of the Minting Process[​](https://dev.flare.network/fassets/minting#duration-of-the-minting-process "Direct link to Duration of the Minting Process")
--------------------------------------------------------------------------------------------------------------------------------------------------------------

The duration of the minting process depends mainly on the speed of the underlying chain. The maximum duration of the process is the sum of:

*   A system-defined maximum time for deposit. It is either a few blocks on the underlying chain or a few minutes, whichever is longer.
*   The underlying chain's finalization time.
*   The Flare Data Connector proof time, which is approximately 3 - 5 minutes, independent of the underlying chain.

On fast chains like XRPL, the maximum total time is less than 10 minutes, while on Bitcoin it is approximately 1.5 hours. For payment failures, the agent needs to wait the maximum time, as defined above, before the nonpayment proof can be retrieved.

Minting Payment Reference[​](https://dev.flare.network/fassets/minting#minting-payment-reference "Direct link to Minting Payment Reference")
--------------------------------------------------------------------------------------------------------------------------------------------

The system generates a unique payment reference at the time of the collateral reservation request. The minter must include the payment reference in a memo field when the underlying payment transaction is made.

The payment reference ensures the payment transaction cannot be used by another entity that might claim to have made the payment on the underlying chain and receive the minted FAssets in return. Additionally, if the payment time expires before payment is done, the agent can prove that no payment with that reference was made.

A similar payment reference for the same purposes is generated for [redemptions](https://dev.flare.network/fassets/redemption).

Redemption Tickets and the Redemption Queue[​](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue "Direct link to Redemption Tickets and the Redemption Queue")
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

For every minting operation, a redemption ticket is created. This ticket references the minted amount and the agent that is backing the minting.

The redemption tickets are ordered in a queue that determines the next agent to be [redeemed](https://dev.flare.network/fassets/redemption) against according to the first in, first out method (FIFO). In other words, the first redemption ticket created will be the first redemption ticket processed. The FIFO queue impartially ensures that all agents have the opportunity to fulfill the duties of their role.

The following example shows how the redemption queue works.

Redemption queue example.

Lots[​](https://dev.flare.network/fassets/minting#lots "Direct link to Lots")
-----------------------------------------------------------------------------

Every minting and redemption must be made in a positive integer of lots. Lots serve the following purposes:

*   They prevent underlying transaction fees from exceeding minting or redemption fees.
*   They restrict large numbers of very small redemption tickets from being submitted, which would increase gas costs.

Therefore, the amount of tokens in a lot (the _lot size_) varies for each underlying chain. For example, on the XRPL chain, a lot can be as small as 10 XRP because transaction fees are low. On the other hand, on the Bitcoin chain, lots might need to be as big as 0.25 BTC or more because transactions are far more expensive.

Over time, the lot size can be updated to reflect price fluctuations of the underlying asset. Only a governance call can update the lot size, and it can be updated only by a limited amount per day.

Dust[​](https://dev.flare.network/fassets/minting#dust "Direct link to Dust")
-----------------------------------------------------------------------------

Some processes generate a fractional number of lots:

*   On minting, part of the minting fee is minted as the FAsset fee to the collateral pool. This value is usually less than 1 lot.
*   When the lot size is changed, redemptions close only a positive integer of lots of each redemption ticket, which leaves the remainder unredeemed.

These amounts, known as dust, cannot be redeemed directly because redemption requires a positive integer of lots.

In such cases, the generated dust is not included in any redemption ticket. Instead, each agent's dust is accumulated until the dust amounts to a whole lot. When that happens, another redemption ticket is automatically created.

Therefore, the dust can be recovered or destroyed in the following ways:

*   If the dust exceeds 1 lot during minting, the part that is a whole multiple of a lot is automatically added to the created redemption ticket.

*   If an agent does not mint any FAssets for a while but the lot size changes and several redemptions occur, enough dust might accumulate to more than 1 lot.

In this case, the part that is a whole multiple of a lot can be converted to a redemption ticket by request. To prevent an inactive agent making FAssets less fungible, this request can be made by any address.

*   Self-closing can work with fractional lots, so it can be used to remove dust.

*   Liquidation can work with fractional lots too, so it can also be used to remove dust.

Self-Minting[​](https://dev.flare.network/fassets/minting#self-minting "Direct link to Self-Minting")
-----------------------------------------------------------------------------------------------------

Agents can also act as minters and mint FAssets from their own vaults. This process is called self-minting and is simpler than regular minting because neither the CRT nor the agent's fee are necessary.

When an agent self-mints FAssets:

*   The agent still needs to pay the amount to mint on the underlying chain and execute the minting.
*   The self-minting operation also adds a [ticket to the redemption queue](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue), alongside tickets added by mints done by other users. All tickets are processed by the FIFO queue.
*   Only the [pool's share of the fee](https://dev.flare.network/fassets/minting#fees) must be paid.

Because self-minting is done without a collateral reservation request, in some cases, a change between the underlying deposit and the execution, such as another collateral reservation, price change which reduces the amount of free [lots](https://dev.flare.network/fassets/minting#lots), or lot-size change, might prohibit the intended number of lots to be minted. If one of these changes occurs, the agent can self-mint a smaller number of lots, even 0 lots, and the remainder of the deposited underlying assets is added to the free underlying balance.

Additionally, when agents create a vault, they can choose not to make it public, so the vault can only be used to self-mint.


Title: Redemption | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/redemption

Markdown Content:
Any holder of FAssets can redeem their FAssets for the underlying original asset. To do so, these holders, known as redeemers, send FAssets to the Asset Manager smart contract, and the redeemed amount is paid with the underlying asset from an agent's address.

Redemption Process[​](https://dev.flare.network/fassets/redemption#redemption-process "Direct link to Redemption Process")
--------------------------------------------------------------------------------------------------------------------------

This is the summary of the redemption process:

1.   The redeemer starts the redemption for a positive integer of lots by issuing a request to the Asset Manager smart contract.

The FAssets system chooses one or more redemption tickets from the front of the [FIFO redemption queue](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue). The number of chosen redemption tickets is capped to avoid high gas consumption. If the redemption amount requires too many tickets, only a partial redemption is done.

2.   The system burns FAssets from the redeemer's account in the amount of the total of the selected redemption tickets. If the redeemer's account does not contain enough FAssets, the redemption fails immediately.

3.   Each chosen ticket belongs to an agent. For every agent participating in the redemption, the system issues an event with the following redemption payment information:

    *   Redeemer's underlying address.

Agents can use the Flare Data Connector to ensure the validity of this address. Otherwise, malicious redeemers could provide an address that systematically blocks payments and exploit the redeeming process to their advantage.

    *   Amount to pay minus the fee that was already subtracted.

    *   [A payment reference](https://dev.flare.network/fassets/minting#minting-payment-reference). This payment reference is different for each agent and each redemption.

    *   The last underlying block and the last underlying timestamp to complete the payment.

4.   Every agent pays the redeemer on the underlying chain and includes the payment reference in the memo field of the payment transaction.

Agents can pay the redemption from their own address they control on the underlying chain. It does not need to be the same address where they receive minting payments.

5.   After the payment is finalized, the agent uses the [FDC](https://dev.flare.network/fdc/overview) to prove the payment and obtain a payment proof.

6.   The agent (or, in Core Vault and certain flows, the **executor**) presents the payment proof to the FAssets system, which issues a **redemption ticket**.

The executor role is responsible for ensuring that the system can finalize redemptions even if the agent is unresponsive or if the flow is managed by a central entity (such as the Core Vault executor).

The redemption ticket is required to:

    *   Prove the payment has occurred.
    *   Trigger burning of the FAssets.
    *   Release the corresponding agent's vault collateral and pool collateral.
    *   Ensure the system tracks the underlying chain balances correctly.

After the collateral is released, it can either back the minting of more FAssets or be withdrawn.

Redemption-Payment Failure[​](https://dev.flare.network/fassets/redemption#redemption-payment-failure "Direct link to Redemption-Payment Failure")
--------------------------------------------------------------------------------------------------------------------------------------------------

Agents have a limited time to pay the redeemer on the underlying chain. The amount of time is defined by the last block and the last timestamp on the underlying chain. If the payment is not made in time, the redeemer has to prove nonpayment to be compensated. After the redeemer presents the nonpayment proof, he is paid with the agent's collateral plus a _redemption default premium_. The premium is intended to encourage the agent to complete redemptions by paying with the underlying asset instead of collateral.

If a payment fails and the failed transaction is recorded on the underlying chain, the agent must submit a proof of failed payment. In this way, the gas costs of the failed transaction can be accounted for by the FAssets system. If the transaction was not recorded, then no gas was spent and reporting is not necessary.

If the agent does not report the failed payment in time, anyone (including the executor) can report the failed payment and receive a reward from the agent's vault.

info

When payment fails because of the redeemer, the agent can obtain a proof of the failed payment from the Flare Data Connector and present it to the FAssets system. The agent's obligation is then fulfilled, and he can keep both the collateral and the underlying.

Two different proofs can be used:

*   Proof of invalid address, due to a wrong syntax or checksum, for example.

*   Proof of blocked payment: Even if the address is valid, it might contain a contract that blocks the payment. This can only happen on underlying networks supporting smart contracts.

The agent must still try to pay and, if the payment is blocked, the agent can request this proof from the Flare Data Connector and present it to the FAssets system.

During step 4 above, if any agent does not pay on the underlying chain, the redeemer completes the following procedure separately for each nonpaying agent:

1.   The redeemer obtains a proof of nonpayment from the Flare Data Connector.
2.   The redeemer presents the nonpayment proofs to the FAssets system, which triggers a redemption failure.
3.   The redeemer is paid with collateral, according to the current price plus a premium.
4.   FAssets are overcollateralized, so, even after paying the redeemer with a premium, a remainder is released. This remainder is derived by the [system-wide collateral ratio settings](https://dev.flare.network/fassets/collateral#system-wide-thresholds) specified by governance.
5.   The underlying assets backing the redeemed FAssets are marked as free and can be withdrawn by the agent later.

Edge Cases[​](https://dev.flare.network/fassets/redemption#edge-cases "Direct link to Edge Cases")
--------------------------------------------------------------------------------------------------

### Unresponsive redeemer[​](https://dev.flare.network/fassets/redemption#unresponsive-redeemer "Direct link to Unresponsive redeemer")

After a redemption nonpayment, the redeemer might not report the failure for some reason. In this case, the agent or the executor can present a nonpayment proof, and the redeemer receives collateral plus a premium. After this operation, the underlying backing collateral and the remaining local collateral are released.

### Unresponsive agent[​](https://dev.flare.network/fassets/redemption#unresponsive-agent "Direct link to Unresponsive agent")

After a successful payment, the agent might not present the payment proof (redemption ticket).

Because the agent has already paid, the redeemer is not affected. However, the system still requires the payment proof to correctly track the agent's balance on the underlying chain. After enough time for the agent to present the proof has elapsed, anyone, including the executor, can present the payment proof and receive collateral from the agent's vault as a reward.

### Expired proof[​](https://dev.flare.network/fassets/redemption#expired-proof "Direct link to Expired proof")

Proofs provided by the Flare Data Connector are available for only 24 hours, approximately. If neither the redeemer, the agent, nor the executor presents the proof of payment or nonpayment within 24 hours, the regular redeeming process cannot continue, and the agent's collateral could be locked indefinitely.

The procedure to recover this collateral is the same as the procedure in the minting case.

Redemption Fee[​](https://dev.flare.network/fassets/redemption#redemption-fee "Direct link to Redemption Fee")
--------------------------------------------------------------------------------------------------------------

The redemption fee is the amount of the underlying asset that the agent can keep for doing the redemption. This fee is meant only to cover the agent's transaction fee on the underlying chain, so it is not shared with the collateral pool. The fee percentage is defined by governance, is the same for all agents, and is typically smaller than the minting fee.

Governance calculates the percentage so that the fee to redeem 1 lot pays for a typical transaction fee on the underlying chain. Therefore, when larger amounts on a single address are redeemed, the agent accrues some extra fees because the underlying fee for small and large transactions is the same. However, when underlying fees are very high, the agent might still lose funds when a redemption for a small amount, such as 1 lot, is made. If this situation occurs frequently, governance will increase the redemption-fee percentage.

Self-redemption[​](https://dev.flare.network/fassets/redemption#self-redemption "Direct link to Self-redemption")
-----------------------------------------------------------------------------------------------------------------

Agents can also act as users and redeem FAssets from their own vaults. This process is called self-redemption or self-closing, and it is simplified because payment on the underlying chain is not required.

As shown in the following process, agents can self-redeem for any reason, including to stop liquidations because it reduces the amount of FAssets the agent is backing.

1.   An agent sends FAssets to their account.
2.   FAssets are burned.
3.   The collateral that was backing those assets is released.
4.   The underlying collateral is released and can be withdrawn from the underlying address later.

The self-redeemed amount is not limited to a positive integer of lots and can be less than 1 lot, which makes self-closing ideal for redeeming an agent's dust.

Title: Collateral | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/collateral

Markdown Content:
FAssets collateral is locked in contracts that ensure the minted FAssets can always be redeemed for the underlying assets they represent or compensated by collateral. Along with Flare's native token, FLR, any governance approved ERC-20 token on the Flare blockchain can be used as collateral.

FAssets collateral ensures the security and redemption of minted FAssets by locking collateral in smart contracts. This guarantees that FAssets can either be redeemed for their underlying assets or compensated by collateral. Collateral can include Flare's native token (FLR) and any governance-approved ERC-20 tokens on the Flare blockchain.

Collateral Types[​](https://dev.flare.network/fassets/collateral#collateral-types "Direct link to Collateral Types")
--------------------------------------------------------------------------------------------------------------------

Two primary types of collateral secure FAssets: **Vault Collateral** and **Pool Collateral**.

Vault collateral is provided exclusively by agents and ensures they perform their duties. Pool collateral is provided by agents and FLR holders who choose to contribute to the pool. It is a safeguard when a sudden drop in the price of the vault collateral makes it insufficient to back the underlying assets.

### Vault Collateral[​](https://dev.flare.network/fassets/collateral#vault-collateral "Direct link to Vault Collateral")

Vault collateral consists of the types of collateral chosen by agents to store in their vault. Flare governance approves the valid types, which are generally stablecoins, such as USDC, USDT, or other highly liquid tokens on the Flare network.

Agents choose one of the types defined by FAssets governance and use it as collateral in their vaults. Agents cannot switch to a different type after a vault is created, but they can create any number of vaults, with different types.

Each collateral type defines an ERC-20 token to use as collateral, a series of [collateral ratios](https://dev.flare.network/fassets/collateral#collateral-ratio), and information to retrieve the asset's price from the FTSO system. Governance reserves the right to add new types or deprecate existing types. If governance deprecates a type, agents must switch to a supported type.

Each vault is associated with a single, unique address on the underlying chain called the agent's underlying address. It receives underlying assets when they are minted into FAssets and sends underlying assets to the redeemer's address when they are redeemed.

When an agent creates a vault, the underlying address is checked for validity using the Flare Data Connector. Otherwise, malicious agents could provide an address that systematically blocks payments and exploit the [minting process](https://dev.flare.network/fassets/minting) to their advantage.

### Pool Collateral[​](https://dev.flare.network/fassets/collateral#pool-collateral "Direct link to Pool Collateral")

When the price of the vault collateral changes in such a way that the vault collateral cannot fully back all the minted FAssets, a [liquidation](https://dev.flare.network/fassets/liquidation) mechanism ensures enough FAssets are burned to restore balance. The pool collateral provides an additional source of backing for situations when the price fluctuates too rapidly for liquidations to correct the imbalance.

Pool collateral is always native FLR tokens or SGB tokens on the Songbird network and can be used as an additional source of collateral for [liquidations](https://dev.flare.network/fassets/liquidation) and [failed redemptions](https://dev.flare.network/fassets/redemption#redemption-payment-failure).

Anyone can participate in the FAssets system by providing native tokens to this pool. In return, providers receive **collateral pool tokens** (CPTs) as proof of the share of native tokens they provided to a specific pool from a specific agent. CPTs are ERC-20 tokens specific to both an agent and a pool.

Providers can redeem their CPTs for FLR, or even transfer or trade them, after a governance-defined time period has elapsed since they entered the pool. This **time lock** is necessary to reduce sandwiching attacks.

Additionally, CPT holders are entitled to a share of any fee the agent earns from minting FAssets using this pool as explained in the next section.

CPT conversion formulae and examples.

Collateral Ratio[​](https://dev.flare.network/fassets/collateral#collateral-ratio "Direct link to Collateral Ratio")
--------------------------------------------------------------------------------------------------------------------

The collateral ratio (CR) is the ratio between the value of all the tokens used as collateral and the total value of the underlying assets held by an agent at any given time. The agent's vault and the collateral pool each has its own unique collateral ratio, which is constantly changing as the value of the underlying assets and the collateral change. These values are obtained using the [FTSO](https://dev.flare.network/ftso/overview).

The following example shows vault and pool CR:

Vault and pool CR

Assume an amount of FAssets currently valued at $1000 USD, backed by $1500 worth of USDC in vault collateral and $2000 worth of FLR in pool collateral.

The resulting vault CR is: $1500$1000=1.5\frac{\text{\$1500}}{\text{\$1000}} = 1.5

The resulting pool CR is: $2000$1000=2\frac{\text{\$2000}}{\text{\$1000}} = 2

Several thresholds are defined for the collateral ratio, and they are used at different times during the FAsset operations. Some are set by the system, and others are set by the agent:

### System-Wide Thresholds[​](https://dev.flare.network/fassets/collateral#system-wide-thresholds "Direct link to System-Wide Thresholds")

The following thresholds are set by the FAssets system's governance and are the same for all agents.

#### Minimal CR[​](https://dev.flare.network/fassets/collateral#minimal-cr "Direct link to Minimal CR")

The lowest collateral ratio the agent vault and the collateral pool must maintain so that enough collateral exists to insure the minted FAssets and to compensate for redemption payments that fail. The minimal CR can be different for each type of collateral.

If an agent's CR remains below the minimal CR for longer than a governance-set amount of time, [liquidations](https://dev.flare.network/fassets/liquidation) can start.

#### Liquidation CR[​](https://dev.flare.network/fassets/collateral#liquidation-cr "Direct link to Liquidation CR")

**Liquidation CR**: An agent's position is unhealthy when the agent's vault CR or pool CR fall below their minimal CR. However, as long as the CR remains above liquidation CR, the CR can briefly fall below the minimal CR.

During this time, the agent can either deposit more collateral or self-close some backed FAssets to improve the position.

However, if the CR falls below the liquidation CR, liquidations can start immediately.

The value of each liquidation CR is approximately 10% less than the minimal CR.

Example liquidation CR

Assume the **minimal CR** is 1.4 and the **liquidation CR** is 1.3.

If the agent's vault CR drops below 1.3, the agent's position can be liquidated immediately. If the agent's vault CR drops below 1.4 but not below 1.3, the agent has some time to amend the position before it can be liquidated.

Adjusted for the collateral pool's minimal CR, the same example applies to the collateral pool.

#### Safety CR[​](https://dev.flare.network/fassets/collateral#safety-cr "Direct link to Safety CR")

If one or both of the collateral types fall below liquidation CR or below the minimum CR for a longer period of time, liquidation occurs. When the offending collateral reaches a healthy CR again, the liquidation stops. To prevent the agent from immediately reverting into liquidation after a small price change, the CR must reach the safety CR before it can start operating normally again and liquidation stops.

Each of the collateral types, the agent's vault and the collateral pool, has its own unique safety CR.

### Agent Thresholds[​](https://dev.flare.network/fassets/collateral#agent-thresholds "Direct link to Agent Thresholds")

The following thresholds are set by each agent according to their own preferences.

#### Minting CR[​](https://dev.flare.network/fassets/collateral#minting-cr "Direct link to Minting CR")

For each mint done by an agent, the maximum amount allowed to be minted is calculated so that the CR for the agent's vault and the CR for the agent's collateral pool after the mint remain higher than the minting CR for each collateral type. To reduce the threat of liquidation, agents should set the minting CR well above the minimal CR to accommodate price fluctuations that might occur before the CR falls below the minimal CR after the mint and minting is no longer possible.

#### Exit CR[​](https://dev.flare.network/fassets/collateral#exit-cr "Direct link to Exit CR")

After a user redeems CPTs, the pool CR must be more than the exit CR. If the pool CR is already below the exit CR, redemption cannot occur. The exit CR is for the collateral pool only.

#### Top-up CR[​](https://dev.flare.network/fassets/collateral#top-up-cr "Direct link to Top-up CR")

To incentivize healthy collateral pools, if the pool CR falls below the top-up CR, anyone can add collateral to the pool and receive [CPTs](https://dev.flare.network/fassets/collateral#pool-collateral) at a reduced price. This [top-up mechanism](https://dev.flare.network/fassets/collateral#top-up) decreases the likelihood of liquidations because of a low amount of pool collateral.

Minting Fees and Debt[​](https://dev.flare.network/fassets/collateral#minting-fees-and-debt "Direct link to Minting Fees and Debt")
-----------------------------------------------------------------------------------------------------------------------------------

As part of the minting process, users pay a [minting fee](https://dev.flare.network/fassets/minting#fees) on the underlying chain. The agent's share of this fee remains on the underlying chain, whereas the pool's share triggers the minting of an equivalent amount of FAssets on the Flare network.

These FAssets coming from the minting fee are added to the collateral pool, where they are shared between collateral providers in proportion to the amount of CPTs that providers have. At any time, providers can claim their due share of the fees in the pool. When providers exit the collateral pool by redeeming their CPTs, any remaining unclaimed fee is automatically transferred to them.

Providers are naturally only entitled to the minting fees accrued after they entered the pool. Therefore, providers entering a pool with preexisting fees are assigned a **fee debt**. The amount of fees a provider can actually withdraw from the pool is calculated by first subtracting their debt from the total amount of fees in the pool. In this way, the amount of fees that a provider can withdraw upon entering a pool is exactly zero.

A provider's fee debt:

| Increases when the provider | Decreases when the provider |
| --- | --- |
| Enters a pool which already has fees in it. | Exits the pool, partially or completely. |
| Withdraws FAsset fees. | Deposits FAssets, paying off part of the debt. |

It is worth noting that:

*   When a provider withdraws fees, their debt increases by the same amount.
*   Since CPTs are ERC-20 tokens, a secondary market for them is expected to develop. If CPTs become more valuable than the FAsset fees they represent, returning the FAssets and paying off part of their fee debt might be more lucrative for providers.

Fee entitlement formulae and examples.

Transferable and Locked CPTs[​](https://dev.flare.network/fassets/collateral#transferable-and-locked-cpts "Direct link to Transferable and Locked CPTs")
--------------------------------------------------------------------------------------------------------------------------------------------------------

CPTs can always be **redeemed** by exiting the pool, but only the portion above the fee debt can be **transferred** to another account; therefore, CPTs held by providers are divided into two types.

### Transferable[​](https://dev.flare.network/fassets/collateral#transferable "Direct link to Transferable")

Tokens whose time lock has expired and are also free of fee debt. These tokens are fungible, and they can be transferred or traded just like any other ERC-20 token.

### Locked[​](https://dev.flare.network/fassets/collateral#locked "Direct link to Locked")

The CPTs serve only as proof of ownership of some of the collateral in the pool, and they cannot be transferred nor traded.

Locked CPTs are one of the following types:

*   **Time-locked**: Tokens whose time lock has not expired must wait to become transferable or redeemable.

*   **Debt-locked**: Tokens corresponding to an amount of fees below the provider's fee debt cannot be transferred because they would need to carry the debt with them. However, they can be [redeemed](https://dev.flare.network/fassets/collateral#cpt-redemption).

As new fees arrive in the pool, some previously debt-locked tokens become transferable.

These CPTs can also become transferable by adding FAssets to the pool, which settles, either partially or completely, the fee debt.

CPT transferability formulae and examples.

CPT Redemption[​](https://dev.flare.network/fassets/collateral#cpt-redemption "Direct link to CPT Redemption")
--------------------------------------------------------------------------------------------------------------

When collateral providers exit the pool by redeeming their CPTs, the FAssets system burns them and returns the appropriate share of the collateral plus the share of [FAsset-minting fees minus any FAsset-fee debt](https://dev.flare.network/fassets/collateral#minting-fees-and-debt).

Providers also have the option to exit the pool partially, by redeeming only some of their CPTs. In this case, they can choose one of the following options to manage their due FAsset fees: withdraw the fees, reduce the fee debt, or both, keeping the current fee-to-debt-ratio.

However, providers can exit, either fully or partially, only when the [collateral ratio CR](https://dev.flare.network/fassets/collateral#collateral-ratio) is high enough. After they exit, the **CR** must be higher than the **exit CR** to prevent their exit from reducing the **CR** to a dangerous level.

Therefore, exits are impossible when the **CR** is below the **exit CR**. In this case, if providers have enough FAssets, they can exit by **self-closing**, which burns enough of their FAssets, plus their fees, to release their collateral.

Providers are mainly compensated in underlying assets for the burned FAssets, depending on the [number of lots](https://dev.flare.network/fassets/minting#lots) of FAssets that need to be redeemed:

*   If more than 1 lot needs to be redeemed, the value of the burned FAssets is redeemed through the standard [redemption process](https://dev.flare.network/fassets/redemption).

*   If less than 1 lot needs to be redeemed, the agent buys the underlying funds from the user using vault collateral, at the price reported by the [FTSO](https://dev.flare.network/ftso/overview) minus a percentage defined by the agent. This purchase by the agent occurs because fees on underlying chains can be expensive, which makes redemption of small quantities too expensive for the agent.

Providers can always request this option instead of receiving underlying tokens. Also, if enough vault collateral is not available, pool collateral is used instead.

warning

In the case where the agent does not redeem in the underlying asset, the FAssets system pays the provider in collateral from the agent's vault because the pool collateral backing the redeemed FAssets is already withdrawn.

When this type of redemption occurs, users might receive less collateral than they would have received if they had made a normal redemption.

Agent Stake[​](https://dev.flare.network/fassets/collateral#agent-stake "Direct link to Agent Stake")
-----------------------------------------------------------------------------------------------------

Agents must have a stake in their collateral pools, which means they must hold the amount of CPTs proportional, by a system-defined constant, to the backed amount of FAssets. The maximum amount of minting is limited by the amount of collateral pool tokens held by the agents. The agents' tokens are locked, which means they cannot be redeemed or transferred, while agents back these FAssets.

When the agent's portion of the collateral pool is below the threshold, new mintings are not allowed. However, this situation does not trigger a liquidation because only the total pool stake matters when collateral needs to be redeemed or a liquidation payment needs to be made.

If an agent's actions force a payment to be made from the collateral pool, the agent's CPTs, valued by the paid native tokens and recalculated by the collateral-pool-price formula, are burned. These actions can cause the agent's CPTs to be burned:

*   When a redemption payment fails, when enough vault collateral to compensate the redeemer is not available, or when the system is set to automatically pay for redemption failures from the collateral pool.
*   Liquidation because the CR of the vault collateral is too low.
*   Full liquidation because of an [agent infraction](https://dev.flare.network/fassets/redemption#redemption-payment-failure) during a transfer on an underlying chain.

Top-up[​](https://dev.flare.network/fassets/collateral#top-up "Direct link to Top-up")
--------------------------------------------------------------------------------------

To reduce the likelihood of liquidations because the pool collateral is too low, the pool can be topped up at a reduced price when the **CR** is above the **top-up CR**. A top-up mechanism for vault collateral is not available. To prevent liquidation, agents can add vault collateral any time.


Title: Core Vault | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/core-vault

Markdown Content:
Overview[​](https://dev.flare.network/fassets/core-vault#overview "Direct link to Overview")
--------------------------------------------------------------------------------------------

The **Core Vault (CV)** is a specialized FAsset system vault that operates on the underlying network. It addresses a critical challenge in cross-chain systems: **protecting user funds from malicious agents while maintaining system scalability.**

### Why the Core Vault?[​](https://dev.flare.network/fassets/core-vault#why-the-core-vault "Direct link to Why the Core Vault?")

*   In **FAssets v1**, every agent had to keep the underlying asset (e.g., XRP) in their own wallet.
*   To prevent theft, agents needed to be heavily overcollateralized - they had to lock up more value than they could ever profit from stealing.
*   This worked, but it limited minting capacity and made the system capital-inefficient.

The **Core Vault** changes this model:

*   Instead of holding XRP in their own wallets, **agents can transfer underlying assets into a shared, insured vault**.
*   The vault is **multisig-controlled** and governed by Flare, so agents cannot unilaterally take the funds.
*   Because theft is structurally prevented, agents no longer need extreme levels of overcollateralization.

The Core Vault prevents agents from running away with XRP while simultaneously lowering collateral requirements and improving system scalability.

### Key Properties[​](https://dev.flare.network/fassets/core-vault#key-properties "Direct link to Key Properties")

*   **Secure by design**: Agents cannot withdraw underlying assets directly.
*   **Capital efficient**: Agents can mint more FAssets with less collateral.
*   **System-wide liquidity**: Assets in the CV form a shared pool available to all agents.
*   **Governance oversight**: Multisig with pause controls and time-bounded fund release.

Introduced in **FAssets v1.1**, the Core Vault enables the system to expand its minting capacity while maintaining the same strong guarantees of safety for users.

Key Features[​](https://dev.flare.network/fassets/core-vault#key-features "Direct link to Key Features")
--------------------------------------------------------------------------------------------------------

### Transfer Capacity[​](https://dev.flare.network/fassets/core-vault#transfer-capacity "Direct link to Transfer Capacity")

To ensure redemption liquidity, a parameter enforces that after transferring assets to the Core Vault, **an agent must still maintain a minimum portion of minting capacity outside the CV**. This prevents agents from moving everything into the vault and leaving the system unbalanced.

### Security Design[​](https://dev.flare.network/fassets/core-vault#security-design "Direct link to Security Design")

*   Each supported asset (XRP, BTC, DOGE, etc.) has its **own dedicated Core Vault**.
*   Each CV is a **multisig account on the underlying chain**, operated under a formal governance agreement with Flare.
*   Funds in the CV **no longer belong to a single agent** - they are pooled, with withdrawals only allowed under system rules.
*   Governance can pause deposits or withdrawals if suspicious activity is detected.

Core Vault Implementation[​](https://dev.flare.network/fassets/core-vault#core-vault-implementation "Direct link to Core Vault Implementation")
-----------------------------------------------------------------------------------------------------------------------------------------------

### Fund Movement on Underlying Networks[​](https://dev.flare.network/fassets/core-vault#fund-movement-on-underlying-networks "Direct link to Fund Movement on Underlying Networks")

The Core Vault operates differently depending on the underlying network:

**For XRP Ledger (XRP CV):**

*   The CV is implemented as a **multisig account** on the XRP Ledger.
*   Only the authorized **multisig signers** can move XRP from the vault.
*   These signers are authorized by Flare governance and operate under formal agreements.
*   All outgoing transactions require multiple signatures from the authorized signers.

### Agent Ownership vs. Core Vault Ownership[​](https://dev.flare.network/fassets/core-vault#agent-ownership-vs-core-vault-ownership "Direct link to Agent Ownership vs. Core Vault Ownership")

There's an important distinction between agent ownership and CV ownership:

#### Agent Ownership (Standard FAssets)[​](https://dev.flare.network/fassets/core-vault#agent-ownership-standard-fassets "Direct link to Agent Ownership (Standard FAssets)")

*   Agents hold underlying assets in their own wallets.
*   These assets belong to the specific agent and are part of their collateral.
*   Agents have direct control over these assets.

#### Core Vault Ownership[​](https://dev.flare.network/fassets/core-vault#core-vault-ownership "Direct link to Core Vault Ownership")

*   Assets transferred to the CV become part of a shared pool.
*   These assets **do not belong to any specific agent** once in the CV.
*   The CV is a system-level reserve that any agent can request assets from.
*   This allows for better capital efficiency and system-wide liquidity.

### Agent Collateral Requirements[​](https://dev.flare.network/fassets/core-vault#agent-collateral-requirements "Direct link to Agent Collateral Requirements")

Agents are required to provide collateral in the form of:

*   **Flare's native token (FLR or SGB)**
*   **USDC/USDT (stablecoins)**

When agents transfer underlying assets to the CV, they can reduce their collateral requirements while maintaining their minting capacity. The CV effectively acts as an **insurance-backed reserve** that boosts efficiency without weakening security.

Operational Workflow[​](https://dev.flare.network/fassets/core-vault#operational-workflow "Direct link to Operational Workflow")
--------------------------------------------------------------------------------------------------------------------------------

### Transferring to Core Vault[​](https://dev.flare.network/fassets/core-vault#transferring-to-core-vault "Direct link to Transferring to Core Vault")

1.   **Agent Transfers Assets**

The agent announces a transfer and sends the underlying asset (e.g., XRP) to the Core Vault (CV) address with a valid payment reference.

2.   **Proof of Payment Submitted**

Anyone (including the agent) submits the proof of the transfer to the FAsset system.

3.   **Verification and Redemption**

After verifying the payment, the system releases the agent's collateral.

info

Transfer requests do not expire. Agents must either complete the transfer or cancel and re-queue it. Inaction leaves collateral locked indefinitely.

### Redemption from Core Vault[​](https://dev.flare.network/fassets/core-vault#redemption-from-core-vault "Direct link to Redemption from Core Vault")

There are two methods to retrieve assets from the CV:

#### Request for Return (Agents Only)[​](https://dev.flare.network/fassets/core-vault#request-for-return-agents-only "Direct link to Request for Return (Agents Only)")

An agent can request assets from the CV through a special minting process, which creates a collateral reservation. Once the request is made, CV operators execute the transfer and submit proof of payment to the asset manager. There is no time limit for the CV to honor the request, but governance ensures timely execution.

#### Direct Redemption (Users)[​](https://dev.flare.network/fassets/core-vault#direct-redemption-users "Direct link to Direct Redemption (Users)")

Approved users can burn FXRP and receive underlying XRP from the CV. This requires KYC approval and a minimum redemption threshold. It is typically processed once per day and has a lower priority than agent return requests. It is helpful for large, less time-sensitive redemptions.

XRP Core Vault Design[​](https://dev.flare.network/fassets/core-vault#xrp-core-vault-design "Direct link to XRP Core Vault Design")
-----------------------------------------------------------------------------------------------------------------------------------

The XRP Core Vault is a multisig account on the XRP Ledger, backed by an audited Flare smart contract that emits transaction instructions for execution.

### Transactions in XRP Core Vault[​](https://dev.flare.network/fassets/core-vault#transactions-in-xrp-core-vault "Direct link to Transactions in XRP Core Vault")

The XRP Core Vault (CV) uses two types of transactions:

*   `Payment`[Transactions](https://xrpl.org/docs/references/protocol/transactions/types/payment): Transfers XRP back to agents upon redemption.
*   `EscrowCreate`[Transactions](https://xrpl.org/docs/references/protocol/transactions/types/escrowcreate): Time-locks XRP to control fund releases and minimize spending risk.

info

The vault is operated manually. Multisig signers validate all outgoing transactions against a pre-approved rule set and sign them only during designated daily windows.

### Daily Security Routine[​](https://dev.flare.network/fassets/core-vault#daily-security-routine "Direct link to Daily Security Routine")

1.   **Escrow Expiry Adds Funds**

An expired escrow (size L) adds XRP to the vault's available pool.

2.   **Withdrawals & Payouts**

Users request withdrawals; multisig operators validate Flare's off-chain instructions and process payouts.

3.   **Re-escrow Excess & Maintain Reserve**

Remaining funds are escrowed in batches (size L), while a minimum reserve (M) is kept in the vault.

Three escrows are created if `remaining_funds = M + 3L`, and M stays in the wallet.

### Security Model[​](https://dev.flare.network/fassets/core-vault#security-model "Direct link to Security Model")

The XRP Core Vault features enhanced security measures for managing daily liquidity, including escrow time-locking and a minimum reserve in multisig setups. It has an emergency pause function and "Red Alert Mode" for urgent threats.

In case of a security issue, all signing is halted pending governance review. Escrow accounts may be unlocked to prevent asset loss, with governance overseeing the restoration of operations. Only one escrow can be unlocked per day, minimizing potential damage.

Summary[​](https://dev.flare.network/fassets/core-vault#summary "Direct link to Summary")
-----------------------------------------------------------------------------------------

The **Core Vault** fundamentally strengthens FAssets:

*   **Prevents loss of underlying assets** by removing agent control over pooled funds.
*   **Lowers collateral burdens**, improving capital efficiency.
*   **Expands minting capacity**, making the system more scalable.
*   **Adds layered security**, combining multisig, escrow, and governance controls.

It is the key to making cross-chain assets on Flare both **secure** and **efficient**.


Title: Collateral | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/collateral

Markdown Content:
FAssets collateral is locked in contracts that ensure the minted FAssets can always be redeemed for the underlying assets they represent or compensated by collateral. Along with Flare's native token, FLR, any governance approved ERC-20 token on the Flare blockchain can be used as collateral.

FAssets collateral ensures the security and redemption of minted FAssets by locking collateral in smart contracts. This guarantees that FAssets can either be redeemed for their underlying assets or compensated by collateral. Collateral can include Flare's native token (FLR) and any governance-approved ERC-20 tokens on the Flare blockchain.

Collateral Types[​](https://dev.flare.network/fassets/collateral#collateral-types "Direct link to Collateral Types")
--------------------------------------------------------------------------------------------------------------------

Two primary types of collateral secure FAssets: **Vault Collateral** and **Pool Collateral**.

Vault collateral is provided exclusively by agents and ensures they perform their duties. Pool collateral is provided by agents and FLR holders who choose to contribute to the pool. It is a safeguard when a sudden drop in the price of the vault collateral makes it insufficient to back the underlying assets.

### Vault Collateral[​](https://dev.flare.network/fassets/collateral#vault-collateral "Direct link to Vault Collateral")

Vault collateral consists of the types of collateral chosen by agents to store in their vault. Flare governance approves the valid types, which are generally stablecoins, such as USDC, USDT, or other highly liquid tokens on the Flare network.

Agents choose one of the types defined by FAssets governance and use it as collateral in their vaults. Agents cannot switch to a different type after a vault is created, but they can create any number of vaults, with different types.

Each collateral type defines an ERC-20 token to use as collateral, a series of [collateral ratios](https://dev.flare.network/fassets/collateral#collateral-ratio), and information to retrieve the asset's price from the FTSO system. Governance reserves the right to add new types or deprecate existing types. If governance deprecates a type, agents must switch to a supported type.

Each vault is associated with a single, unique address on the underlying chain called the agent's underlying address. It receives underlying assets when they are minted into FAssets and sends underlying assets to the redeemer's address when they are redeemed.

When an agent creates a vault, the underlying address is checked for validity using the Flare Data Connector. Otherwise, malicious agents could provide an address that systematically blocks payments and exploit the [minting process](https://dev.flare.network/fassets/minting) to their advantage.

### Pool Collateral[​](https://dev.flare.network/fassets/collateral#pool-collateral "Direct link to Pool Collateral")

When the price of the vault collateral changes in such a way that the vault collateral cannot fully back all the minted FAssets, a [liquidation](https://dev.flare.network/fassets/liquidation) mechanism ensures enough FAssets are burned to restore balance. The pool collateral provides an additional source of backing for situations when the price fluctuates too rapidly for liquidations to correct the imbalance.

Pool collateral is always native FLR tokens or SGB tokens on the Songbird network and can be used as an additional source of collateral for [liquidations](https://dev.flare.network/fassets/liquidation) and [failed redemptions](https://dev.flare.network/fassets/redemption#redemption-payment-failure).

Anyone can participate in the FAssets system by providing native tokens to this pool. In return, providers receive **collateral pool tokens** (CPTs) as proof of the share of native tokens they provided to a specific pool from a specific agent. CPTs are ERC-20 tokens specific to both an agent and a pool.

Providers can redeem their CPTs for FLR, or even transfer or trade them, after a governance-defined time period has elapsed since they entered the pool. This **time lock** is necessary to reduce sandwiching attacks.

Additionally, CPT holders are entitled to a share of any fee the agent earns from minting FAssets using this pool as explained in the next section.

CPT conversion formulae and examples.

Collateral Ratio[​](https://dev.flare.network/fassets/collateral#collateral-ratio "Direct link to Collateral Ratio")
--------------------------------------------------------------------------------------------------------------------

The collateral ratio (CR) is the ratio between the value of all the tokens used as collateral and the total value of the underlying assets held by an agent at any given time. The agent's vault and the collateral pool each has its own unique collateral ratio, which is constantly changing as the value of the underlying assets and the collateral change. These values are obtained using the [FTSO](https://dev.flare.network/ftso/overview).

The following example shows vault and pool CR:

Vault and pool CR

Assume an amount of FAssets currently valued at $1000 USD, backed by $1500 worth of USDC in vault collateral and $2000 worth of FLR in pool collateral.

The resulting vault CR is: $1500$1000=1.5\frac{\text{\$1500}}{\text{\$1000}} = 1.5

The resulting pool CR is: $2000$1000=2\frac{\text{\$2000}}{\text{\$1000}} = 2

Several thresholds are defined for the collateral ratio, and they are used at different times during the FAsset operations. Some are set by the system, and others are set by the agent:

### System-Wide Thresholds[​](https://dev.flare.network/fassets/collateral#system-wide-thresholds "Direct link to System-Wide Thresholds")

The following thresholds are set by the FAssets system's governance and are the same for all agents.

#### Minimal CR[​](https://dev.flare.network/fassets/collateral#minimal-cr "Direct link to Minimal CR")

The lowest collateral ratio the agent vault and the collateral pool must maintain so that enough collateral exists to insure the minted FAssets and to compensate for redemption payments that fail. The minimal CR can be different for each type of collateral.

If an agent's CR remains below the minimal CR for longer than a governance-set amount of time, [liquidations](https://dev.flare.network/fassets/liquidation) can start.

#### Liquidation CR[​](https://dev.flare.network/fassets/collateral#liquidation-cr "Direct link to Liquidation CR")

**Liquidation CR**: An agent's position is unhealthy when the agent's vault CR or pool CR fall below their minimal CR. However, as long as the CR remains above liquidation CR, the CR can briefly fall below the minimal CR.

During this time, the agent can either deposit more collateral or self-close some backed FAssets to improve the position.

However, if the CR falls below the liquidation CR, liquidations can start immediately.

The value of each liquidation CR is approximately 10% less than the minimal CR.

Example liquidation CR

Assume the **minimal CR** is 1.4 and the **liquidation CR** is 1.3.

If the agent's vault CR drops below 1.3, the agent's position can be liquidated immediately. If the agent's vault CR drops below 1.4 but not below 1.3, the agent has some time to amend the position before it can be liquidated.

Adjusted for the collateral pool's minimal CR, the same example applies to the collateral pool.

#### Safety CR[​](https://dev.flare.network/fassets/collateral#safety-cr "Direct link to Safety CR")

If one or both of the collateral types fall below liquidation CR or below the minimum CR for a longer period of time, liquidation occurs. When the offending collateral reaches a healthy CR again, the liquidation stops. To prevent the agent from immediately reverting into liquidation after a small price change, the CR must reach the safety CR before it can start operating normally again and liquidation stops.

Each of the collateral types, the agent's vault and the collateral pool, has its own unique safety CR.

### Agent Thresholds[​](https://dev.flare.network/fassets/collateral#agent-thresholds "Direct link to Agent Thresholds")

The following thresholds are set by each agent according to their own preferences.

#### Minting CR[​](https://dev.flare.network/fassets/collateral#minting-cr "Direct link to Minting CR")

For each mint done by an agent, the maximum amount allowed to be minted is calculated so that the CR for the agent's vault and the CR for the agent's collateral pool after the mint remain higher than the minting CR for each collateral type. To reduce the threat of liquidation, agents should set the minting CR well above the minimal CR to accommodate price fluctuations that might occur before the CR falls below the minimal CR after the mint and minting is no longer possible.

#### Exit CR[​](https://dev.flare.network/fassets/collateral#exit-cr "Direct link to Exit CR")

After a user redeems CPTs, the pool CR must be more than the exit CR. If the pool CR is already below the exit CR, redemption cannot occur. The exit CR is for the collateral pool only.

#### Top-up CR[​](https://dev.flare.network/fassets/collateral#top-up-cr "Direct link to Top-up CR")

To incentivize healthy collateral pools, if the pool CR falls below the top-up CR, anyone can add collateral to the pool and receive [CPTs](https://dev.flare.network/fassets/collateral#pool-collateral) at a reduced price. This [top-up mechanism](https://dev.flare.network/fassets/collateral#top-up) decreases the likelihood of liquidations because of a low amount of pool collateral.

Minting Fees and Debt[​](https://dev.flare.network/fassets/collateral#minting-fees-and-debt "Direct link to Minting Fees and Debt")
-----------------------------------------------------------------------------------------------------------------------------------

As part of the minting process, users pay a [minting fee](https://dev.flare.network/fassets/minting#fees) on the underlying chain. The agent's share of this fee remains on the underlying chain, whereas the pool's share triggers the minting of an equivalent amount of FAssets on the Flare network.

These FAssets coming from the minting fee are added to the collateral pool, where they are shared between collateral providers in proportion to the amount of CPTs that providers have. At any time, providers can claim their due share of the fees in the pool. When providers exit the collateral pool by redeeming their CPTs, any remaining unclaimed fee is automatically transferred to them.

Providers are naturally only entitled to the minting fees accrued after they entered the pool. Therefore, providers entering a pool with preexisting fees are assigned a **fee debt**. The amount of fees a provider can actually withdraw from the pool is calculated by first subtracting their debt from the total amount of fees in the pool. In this way, the amount of fees that a provider can withdraw upon entering a pool is exactly zero.

A provider's fee debt:

| Increases when the provider | Decreases when the provider |
| --- | --- |
| Enters a pool which already has fees in it. | Exits the pool, partially or completely. |
| Withdraws FAsset fees. | Deposits FAssets, paying off part of the debt. |

It is worth noting that:

*   When a provider withdraws fees, their debt increases by the same amount.
*   Since CPTs are ERC-20 tokens, a secondary market for them is expected to develop. If CPTs become more valuable than the FAsset fees they represent, returning the FAssets and paying off part of their fee debt might be more lucrative for providers.

Fee entitlement formulae and examples.

Transferable and Locked CPTs[​](https://dev.flare.network/fassets/collateral#transferable-and-locked-cpts "Direct link to Transferable and Locked CPTs")
--------------------------------------------------------------------------------------------------------------------------------------------------------

CPTs can always be **redeemed** by exiting the pool, but only the portion above the fee debt can be **transferred** to another account; therefore, CPTs held by providers are divided into two types.

### Transferable[​](https://dev.flare.network/fassets/collateral#transferable "Direct link to Transferable")

Tokens whose time lock has expired and are also free of fee debt. These tokens are fungible, and they can be transferred or traded just like any other ERC-20 token.

### Locked[​](https://dev.flare.network/fassets/collateral#locked "Direct link to Locked")

The CPTs serve only as proof of ownership of some of the collateral in the pool, and they cannot be transferred nor traded.

Locked CPTs are one of the following types:

*   **Time-locked**: Tokens whose time lock has not expired must wait to become transferable or redeemable.

*   **Debt-locked**: Tokens corresponding to an amount of fees below the provider's fee debt cannot be transferred because they would need to carry the debt with them. However, they can be [redeemed](https://dev.flare.network/fassets/collateral#cpt-redemption).

As new fees arrive in the pool, some previously debt-locked tokens become transferable.

These CPTs can also become transferable by adding FAssets to the pool, which settles, either partially or completely, the fee debt.

CPT transferability formulae and examples.

CPT Redemption[​](https://dev.flare.network/fassets/collateral#cpt-redemption "Direct link to CPT Redemption")
--------------------------------------------------------------------------------------------------------------

When collateral providers exit the pool by redeeming their CPTs, the FAssets system burns them and returns the appropriate share of the collateral plus the share of [FAsset-minting fees minus any FAsset-fee debt](https://dev.flare.network/fassets/collateral#minting-fees-and-debt).

Providers also have the option to exit the pool partially, by redeeming only some of their CPTs. In this case, they can choose one of the following options to manage their due FAsset fees: withdraw the fees, reduce the fee debt, or both, keeping the current fee-to-debt-ratio.

However, providers can exit, either fully or partially, only when the [collateral ratio CR](https://dev.flare.network/fassets/collateral#collateral-ratio) is high enough. After they exit, the **CR** must be higher than the **exit CR** to prevent their exit from reducing the **CR** to a dangerous level.

Therefore, exits are impossible when the **CR** is below the **exit CR**. In this case, if providers have enough FAssets, they can exit by **self-closing**, which burns enough of their FAssets, plus their fees, to release their collateral.

Providers are mainly compensated in underlying assets for the burned FAssets, depending on the [number of lots](https://dev.flare.network/fassets/minting#lots) of FAssets that need to be redeemed:

*   If more than 1 lot needs to be redeemed, the value of the burned FAssets is redeemed through the standard [redemption process](https://dev.flare.network/fassets/redemption).

*   If less than 1 lot needs to be redeemed, the agent buys the underlying funds from the user using vault collateral, at the price reported by the [FTSO](https://dev.flare.network/ftso/overview) minus a percentage defined by the agent. This purchase by the agent occurs because fees on underlying chains can be expensive, which makes redemption of small quantities too expensive for the agent.

Providers can always request this option instead of receiving underlying tokens. Also, if enough vault collateral is not available, pool collateral is used instead.

warning

In the case where the agent does not redeem in the underlying asset, the FAssets system pays the provider in collateral from the agent's vault because the pool collateral backing the redeemed FAssets is already withdrawn.

When this type of redemption occurs, users might receive less collateral than they would have received if they had made a normal redemption.

Agent Stake[​](https://dev.flare.network/fassets/collateral#agent-stake "Direct link to Agent Stake")
-----------------------------------------------------------------------------------------------------

Agents must have a stake in their collateral pools, which means they must hold the amount of CPTs proportional, by a system-defined constant, to the backed amount of FAssets. The maximum amount of minting is limited by the amount of collateral pool tokens held by the agents. The agents' tokens are locked, which means they cannot be redeemed or transferred, while agents back these FAssets.

When the agent's portion of the collateral pool is below the threshold, new mintings are not allowed. However, this situation does not trigger a liquidation because only the total pool stake matters when collateral needs to be redeemed or a liquidation payment needs to be made.

If an agent's actions force a payment to be made from the collateral pool, the agent's CPTs, valued by the paid native tokens and recalculated by the collateral-pool-price formula, are burned. These actions can cause the agent's CPTs to be burned:

*   When a redemption payment fails, when enough vault collateral to compensate the redeemer is not available, or when the system is set to automatically pay for redemption failures from the collateral pool.
*   Liquidation because the CR of the vault collateral is too low.
*   Full liquidation because of an [agent infraction](https://dev.flare.network/fassets/redemption#redemption-payment-failure) during a transfer on an underlying chain.

Top-up[​](https://dev.flare.network/fassets/collateral#top-up "Direct link to Top-up")
--------------------------------------------------------------------------------------

To reduce the likelihood of liquidations because the pool collateral is too low, the pool can be topped up at a reduced price when the **CR** is above the **top-up CR**. A top-up mechanism for vault collateral is not available. To prevent liquidation, agents can add vault collateral any time.


Title: Operational Parameters | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/operational-parameters

Markdown Content:
This page lists the current values for the most important parameters of the FAssets system on **Songbird Canary-Network** and **Songbird Testnet Coston**. These values are subject to change as the system is further developed and tested.

Asset Manager Operational Parameters[​](https://dev.flare.network/fassets/operational-parameters#asset-manager-operational-parameters "Direct link to Asset Manager Operational Parameters")
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

To get the default agent settings, you need to call the `getSettings` function on the `IAssetManager` interface. Read more about the `IAssetManager` interface [here](https://dev.flare.network/fassets/reference/IAssetManager).

### Minting and Redeeming[​](https://dev.flare.network/fassets/operational-parameters#minting-and-redeeming "Direct link to Minting and Redeeming")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Minting cap** `mintingCapAMG` Total amount of allowed FAssets in circulation. Once reached, no more FAssets can be minted until some are redeemed. This is intended as a security measure. In the final deployment, this cap will be gradually increased and finally removed. | 750k XRP |
| [**Lot size**](https://dev.flare.network/fassets/minting#lots) `lotSizeAMG` Minimum quantity required for minting FAssets. | 10 XRP |
| [**Collateral reservation fee (CRF)**](https://dev.flare.network/fassets/minting#collateral-reservation-fee) `collateralReservationFee` Fee applied when reserving collateral for minting.. | 0.5% |
| [**Redemption fee**](https://dev.flare.network/fassets/redemption#redemption-fee) `redemptionFee` Fee charged during redemption of FAssets. | 0.5% |
| [**Redemption default premium**](https://dev.flare.network/fassets/redemption#redemption-payment-failure) `redemptionDefaultPremium` Premium paid if an agent fails to meet redemption obligations. | 5% |
| **Redemption default premium source** Where does the premium come from when an agent fails to pay the redeemer on time? If the vault CR > 1.1, from the agent's vault. Otherwise, from the agent's vault and the collateral pool. | ✅ |
| **Maximum redemption tickets** `maxRedeemedTickets` Maximum number of tickets redeemed in a single request. | 20 |

### Payment Times[​](https://dev.flare.network/fassets/operational-parameters#payment-times "Direct link to Payment Times")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Underlying blocks for payment** `underlyingBlocksForPayment` The number of underlying blocks during which the minter or agent can pay the underlying value. | 225 |
| **Underlying seconds for payment** `underlyingSecondsForPayment` The minimum time allowed for an agent to pay for a redemption or a minter to pay for minting. | 15 minutes |
| **Average block time** `averageBlockTimeMS` The average time between two successive blocks on the underlying chain. | 4 seconds |
| **Time of proof availability** `attestationWindowSeconds` The amount of time that proofs of payment or nonpayment must be available on the Data Connector. | 1 day |
| **Amount of extra time per redemption** `redemptionPaymentExtensionSeconds` The extra amount of time per redemption granted to an agent when many redemption requests occur in a short period of time. | 45 seconds |

### Collateral Ratios[​](https://dev.flare.network/fassets/operational-parameters#collateral-ratios "Direct link to Collateral Ratios")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Vault Collateral Supported Types** Types of collateral required in the agent's vault. | `USDX` |
| [**Vault Minimal CR**](https://dev.flare.network/fassets/collateral#minimal-cr) `minimalCR` The minimum collateral ratio required to avoid liquidation. | 1.2 |
| [**Vault Collateral Safety CR**](https://dev.flare.network/fassets/collateral#safety-cr) `safetyCR` The collateral ratio required to exit liquidation mode. | 1.3 |
| **Pool Collateral Supported Types** Types of collateral required in the collateral pool. | SGB |
| [**Pool Collateral Pool Minimal CR**](https://dev.flare.network/fassets/collateral#minimal-cr) `minimalCR` The minimum collateral ratio required to avoid liquidation. | 1.5 |
| [**Pool Collateral Call Band CR**](https://dev.flare.network/fassets/collateral#liquidation-cr) `ccbCR` The threshold at which collateral is considered unhealthy but liquidation is delayed. | 1.4 |
| [**Pool Collateral Safety CR**](https://dev.flare.network/fassets/collateral#safety-cr) `safetyCR` The collateral ratio required to exit liquidation mode. | 1.6 |
| **Minting pool holdings required** `mintingPoolHoldingsRequired` The minimum amount of pool tokens an agent must hold to be able to mint, as a percentage of the FAssets the agent is currently backing. | 50% |

### Liquidation[​](https://dev.flare.network/fassets/operational-parameters#liquidation "Direct link to Liquidation")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Liquidation premium** `liquidationPremium` Increases in steps, as time passes. | **Step 1**: 5% **Step 2**: 8% **Step 3**: 12% |
| **Liquidation step time** `liquidationStepTime` Elapsed time before the liquidation premium advances to the next step. | 300 seconds |
| **Liquidation source - Liquidated value** Where do the funds come from to pay for liquidations? | The agent's vault |
| **Liquidation source - Premium** Where do the funds come from to pay for liquidations? | The collateral pool |

### Rewarding[​](https://dev.flare.network/fassets/operational-parameters#rewarding "Direct link to Rewarding")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| [**Challenger reward**](https://dev.flare.network/fassets/overview#challengers) `paymentChallengeReward` After a successful challenge for an illegal operation, the agent goes into full liquidation and the challenger is paid this reward from the agent's vault. | 250 USD converted to vault collateral |
| [**Confirmation by others**](https://dev.flare.network/fassets/redemption#edge-cases) `confirmationByOthersAfter` If an agent or redeemer becomes unresponsive, anybody can confirm payments and non-payments some time after the request was made, and get a reward from the agent's vault. |  |
| **Minimum time** `confirmationByOthersAfter` | 6 hours |
| **Reward** `confirmationByOthersReward` | 50 USD (converted to vault collateral) |

### Time Locks[​](https://dev.flare.network/fassets/operational-parameters#time-locks "Direct link to Time Locks")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Time lock** `withdrawalTimelock` Agent has to announce any collateral withdrawal or vault destruction and then wait this time before executing it. | 1 hour |
| **Maximum governance update frequency** `minUpdateRepeatTime` Minimum amount of time between updates of any governance setting. | 1 day |
| **Token invalidation time** `tokenInvalidationTime` Time between the moment a token is deprecated by governance and it becomes invalid. Agents still using it as vault collateral get liquidated after this time. | 1 day |
| **Agent exit available time lock** `agentExitAvailableTimelock` The time the agent has to wait after announcing exit from the list of publicly available agents and executing the exit. | 3 hours |
| **Agent fee change time lock** `agentFeeChangeTimelock` The time the agent has to wait between announcing and changing the agent fee or the pool share. | 1 hour |
| **Agent minting CR change time lock** `agentMintingCRChangeTimelock` The time the agent has to wait between announcing and changing the minting CR (vault or pool). | 5 minutes |
| **Pool exit and top-up change time lock** `poolExitAndTopupChangeTimelock` The time the agent has to wait between announcing and changing any pool exit and top-up settings. | 1 day |
| **Agent time-locked operation window** `agentTimelockedOperationWindow` Once the above time locks expire, agents have this amount of time to execute the requested operation. | 2 hours |
| **Collateral pool token time lock** `collateralPoolTokenTimelock` Amount of seconds that a user entering the collateral pool must wait before spending (exit or transfer) the obtained pool tokens. | 60 seconds |
| **Minimum diamond-cut time lock** `diamondCutMinTimelockSeconds` Amount of time that must elapse before the system performs a <a href='https://eips.ethereum.org/EIPS/eip-2535' target='_blank'>diamond cut</a>. | 1 hour |

### Emergency Pause[​](https://dev.flare.network/fassets/operational-parameters#emergency-pause "Direct link to Emergency Pause")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Emergency pause** `maxEmergencyPauseDurationSeconds` The maximum time for a pause triggered by governance or some other entity. | 3 days |
| **Emergency pause reset** `emergencyPauseDurationResetAfterSeconds` The amount of time since the last emergency pause. After it has elapsed, the pause duration counter automatically resets. | 1 week |

### Transfer Fees[​](https://dev.flare.network/fassets/operational-parameters#transfer-fees "Direct link to Transfer Fees")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Transfer fee represented as a fraction of one millionth of the transferred amount** `transferFeeMillionths` The fee on FAsset token transfer. Each transfer has this value times the transferred amount deducted from its value. The fees get deposited into epochs that are claimable by agents depending on their minting history. | 0 |
| **Maximum Unexpired Epochs for Transfer Fee Claims** `transferFeeClaimMaxUnexpiredEpochs` The number of epochs to pass before the fees get transferred to new epochs. | 30 |
| **Epoch Duration in Seconds for Transfer Fee Claims** `transferFeeClaimEpochDurationSeconds` Duration of each reward epoch. | 3.5 days |
| **Start Timestamp for First Transfer Fee Claim Epoch** `transferFeeClaimFirstEpochStartTs` The first reward epoch timestamp. | 1733122800 (Mon Dec 02 2024 07:00:00 GMT) |

Default Agent Settings[​](https://dev.flare.network/fassets/operational-parameters#default-agent-settings "Direct link to Default Agent Settings")
--------------------------------------------------------------------------------------------------------------------------------------------------

To get the default agent settings, you need to call the `getAgentInfo` function on the `IAssetManager` interface. Read more about the `IAssetManager` interface [here](https://dev.flare.network/fassets/reference/IAssetManager).

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| [**Minting fee**](https://dev.flare.network/fassets/minting#minting-fee) `feeBIPS` The minting fee is when users (minters) mint FAssets by depositing underlying assets with an agent. | 1% |
| [**Pool share**](https://dev.flare.network/fassets/minting#pool-share) `poolFeeShareBIPS` The pool share fee is the portion of the minting and redemption fees allocated to pool collateral providers. | 30% |
| [**Minting Collateral Ratio - Agent Vault**](https://dev.flare.network/fassets/collateral#minting-cr) `mintingVaultCollateralRatioBIPS` The minting vault collateral ratio is the minimum collateral required to back FAssets, ensuring value protection against under-collateralization. | 1.4 |
| [**Minting Collateral Ratio - Collateral Pool**](https://dev.flare.network/fassets/collateral#minting-cr) `mintingPoolCollateralRatioBIPS` The minting pool collateral ratio ensures the collateral value supports the minted FAssets. | 1.7 |
| [**Exit Collateral Ratio**](https://dev.flare.network/fassets/collateral#exit-cr) `poolExitCollateralRatioBIPS` The pool exit collateral ratio is the minimum collateral ratio agents must maintain when exiting their pool collateral. | 1.6 |
| [**Discount for agent self-close**](https://dev.flare.network/fassets/liquidation#stopping-liquidations) `buyFAssetByAgentFactorBIPS` Applied when agents buy back FAssets during liquidation events, shown as a factor on the Agent UI. | 99% |
| **Redemption Pool Fee Share** `redemptionPoolFeeShare` Percentage of redemption fees paid to the pool to sustain it during high redemption periods. | 30% |

Core Vault[​](https://dev.flare.network/fassets/operational-parameters#core-vault "Direct link to Core Vault")
--------------------------------------------------------------------------------------------------------------

### Core Vault Manager[​](https://dev.flare.network/fassets/operational-parameters#core-vault-manager "Direct link to Core Vault Manager")

To get the Core Vault manager operational parameters you need to use the [`ICoreVaultManager`](https://dev.flare.network/fassets/reference/ICoreVaultManager) interface. Specific functions added to each parameter.

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Escrow amount** [_escrowAmount](https://dev.flare.network/fassets/reference/ICoreVaultManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/ICoreVaultManager#getsettings)`_escrowAmount` The amount of XRP to escrow (setting to 0 disables escrowing). | 150k XRP |
| **Minimal left amount in the multisig** [_minimalAmount](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)`_minimalAmount` The minimal amount that will be left on the multisig after escrowing. | 150k XRP |
| **Escrow expiration time** [_escrowEndTimeSeconds](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)`_escrowEndTimeSeconds` The time of day (UTC) when the escrows expire. Exactly one escrow per day will expire. | 50400 (14:00 UTC) |
| **Max expected fee** [_fee](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)`_fee` Maximum expected fee charged by the chain for a payment | 0.0004 XRP (400 drops) |

### Core Vault Settings[​](https://dev.flare.network/fassets/operational-parameters#core-vault-settings "Direct link to Core Vault Settings")

To get the Core Vault settings you need to use the [`IAssetManager`](https://dev.flare.network/fassets/reference/IAssetManager) interface. Specific functions added to each parameter.

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Minting left on agent's address** [getCoreVaultMinimumAmountLeftBIPS](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaultminimumamountleftbips) Minimum amount of minting left on agent's address after transfer to core vault. Expressed as percentage of agent's minting capacity (calculated from agent's vault and pool collateral). | 15% |
| **Transfer to core vault time** [getCoreVaultTransferTimeExtensionSeconds](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaulttransfertimeextensionseconds) The extra time for an agent's transfer to the core vault, compared to ordinary redemption payment. | 2 hours |
| **Transfer fee to Core Vault** [getCoreVaultTransferTimeExtensionSeconds](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaulttransferfeebips) Fee (in percentage of transfer amount) paid by agent for transfer to the core vault. | 0 |
| **Minimum number of lots for direct redemption** [getCoreVaultMinimumRedeemLots](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaultminimumredeemlots) The minimum number of lots that a direct redemption from core vault can take | 1000 |
| **Redemption fee** [getCoreVaultRedemptionFeeBIPS](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaultredemptionfeebips) Fee (in percentage of redemption amount) paid by the redeemer for direct redemptions from the core vault. | 0 |


Title: FAssets on Songbird | Flare Developer Hub

URL Source: https://dev.flare.network/fassets/songbird

Markdown Content:
FAssets on Songbird | Flare Developer Hub

===============

We use cookies to enhance your browsing experience, serve relevant ads or content, and analyze our traffic. By clicking "Accept All", you consent to our use of cookies.[Read our Privacy Policy.](https://flare.network/privacy-policy/)

Customize Accept all

Customize Consent Preferences

We use cookies to help you navigate efficiently and perform certain functions. You will find detailed information about all cookies under each consent category below.

The cookies that are categorized as "Necessary" are stored on your browser as they are essential for enabling the basic functionalities of the site. ...Show more

Necessary Always Active

Necessary cookies are required to enable the basic features of this site, such as providing secure log-in or adjusting your consent preferences. These cookies do not store any personally identifiable data.

Functional

- [x] 

Functional cookies help perform certain functionalities like sharing the content of the website on social media platforms, collecting feedback, and other third-party features.

Analytics

- [x] 

Analytical cookies are used to understand how visitors interact with the website. These cookies help provide information on metrics such as the number of visitors, bounce rate, traffic source, etc.

Performance

- [x] 

Performance cookies are used to understand and analyze the key performance indexes of the website which helps in delivering a better user experience for the visitors.

Advertisement

- [x] 

Advertisement cookies are used to provide visitors with customized advertisements based on the pages you visited previously and to analyze the effectiveness of the ad campaigns.

Others

- [x] 

Other uncategorized cookies are those that are being analyzed and have not been classified into a category as yet.

 Save My Preferences  Accept all 

 Powered by [](https://www.cookieyes.com/product/cookie-consent/)

[Skip to main content](https://dev.flare.network/fassets/songbird#__docusaurus_skipToContent_fallback)

[**Developer Hub**](https://dev.flare.network/)

[](https://github.com/flare-foundation/developer-hub)

ctrl K

*   [Home](https://dev.flare.network/)
*   [Network](https://dev.flare.network/network/overview) 
*   [FTSOv2](https://dev.flare.network/ftso/overview) 
*   [FDC](https://dev.flare.network/fdc/overview) 
*   [FAssets](https://dev.flare.network/fassets/overview) 
    *   [Minting](https://dev.flare.network/fassets/minting)
    *   [Redemption](https://dev.flare.network/fassets/redemption)
    *   [Collateral](https://dev.flare.network/fassets/collateral)
    *   [Core Vault](https://dev.flare.network/fassets/core-vault)
    *   [Liquidation](https://dev.flare.network/fassets/liquidation)
    *   [Operational Parameters](https://dev.flare.network/fassets/operational-parameters)
    *   [FAssets on Songbird](https://dev.flare.network/fassets/songbird)
    *   [Developer Guides](https://dev.flare.network/fassets/developer-guides) 
    *   [Infrastructure Guides](https://dev.flare.network/fassets/guides) 
    *   [FAssets Reference](https://dev.flare.network/fassets/reference) 

*   [Run a Node](https://dev.flare.network/run-node) 

*   [](https://dev.flare.network/)
*   [FAssets](https://dev.flare.network/fassets/overview)
*   FAssets on Songbird

FAssets on Songbird
===================

The launch of FAssets on Songbird Canary-Network demonstrates system behavior while paving the way for its next deployment on Flare Mainnet. The primary goals of this test are to ensure the system operates as intended, identify edge cases, refine usability and automation, and incentivize whitehat security researchers to uncover potential code errors.

The test on Songbird Canary-Network will have the following characteristics:

| Parameters | Description |
| --- | --- |
| FAsset Sequence | XRP will be tested first, followed by either BTC or DOGE. |
| Agent Whitelisting | FAssets agents must be whitelisted by Flare Foundation to perform their roles. |
| Caps and Losses | Flare Foundation will underwrite up to $300,000 in FAsset issuance to cover any losses resulting from system issues, while imposing a cap of $2 million in issuance per asset. |
| Duration of the Test | Each FAsset will be tested on Songbird for at least 6 weeks until no issues have been found. |
| FAssets Minting dApps | FAssets system users can access the frontend web interface for minting and redeeming: - [`https://fasset.oracle-daemon.com/sgb`](https://fasset.oracle-daemon.com/sgb) - [`https://fassets.au.cc/`](https://fassets.au.cc/) |
| System Integrity and FAsset Pricing | During the Songbird test, restrictions and incentives may cause the FAsset price to deviate from the underlying currency's value. The current focus is on testing system integrity, not price alignment. |
| Vault Collateral | USDX will serve as collateral for FAsset agent vaults. To ensure sufficient support for FAsset issuance and possible liquidations on Songbird, a large amount of USDX has been minted. |

Help improve FAssets

To participate, begin by joining the Flare Network FAssets Songbird [Telegram channel](https://t.me/FlareSupport) or contact [support@flare.network](mailto:support@flare.network).

Open Issue Collector

[Edit this page](https://github.com/flare-foundation/developer-hub/edit/main/docs/fassets/8-songbird.mdx)

[Previous Operational Parameters](https://dev.flare.network/fassets/operational-parameters)[Next Developer Guides](https://dev.flare.network/fassets/developer-guides)

[](https://flare.network/)

[Support](https://flare.network/resources/technical-support)|[Brand Kit](https://drive.google.com/drive/u/1/folders/1mPrtIBb2k88E4f1fguEm3eAXLW74xOry)|[Terms & Conditions](https://flare.network/privacy-policy/)|[UK Disclaimer](https://flare.network/uk-disclaimer)

[](https://github.com/flare-foundation)[](https://www.youtube.com/c/Flare_Networks)[](https://www.linkedin.com/company/flarenetwork/)[](https://discord.com/invite/flarenetwork)[](https://x.com/FlareNetworks)[](https://t.me/FlareNetwork)[](https://forum.flare.network/)

© Flare 2025

RESOURCES

[Whitepapers](https://dev.flare.network/support/whitepapers)[Audits](https://dev.flare.network/support/audits)[FAQs](https://dev.flare.network/support/faqs)[FLR](https://dev.flare.network/support/flr)

EXPLORE

[Flarescan](https://flarescan.com/)[Systems Explorer](https://flare-systems-explorer.flare.network/)[Bug Bounty](https://immunefi.com/bug-bounty/flarenetwork/information/)[Grants](https://flare.network/grants)

GOVERNANCE

[Flare Portal](https://portal.flare.network/)[Governance Proposals](https://proposals.flare.network/)


