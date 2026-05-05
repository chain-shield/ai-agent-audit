
 ------------ ## PROTOCOL OVERVIEW ------------ 

# The Graph Protocol Smart Contract Summary

The scoped contracts describe the on-chain core of **The Graph**, a protocol for indexing, organizing, and serving blockchain data. At a product level, The Graph supports subgraphs, streaming data pipelines, and query infrastructure. At a smart contract level, the protocol uses **GRT** as its economic asset and coordinates staking, rewards, curation, subgraph ownership, payments, disputes, token distribution, governance, and L1/L2 bridging between Ethereum and Arbitrum.

## What the protocol is

The Graph’s contracts form an economic and administrative system around decentralized data indexing and query services. The protocol appears to split responsibilities across several major domains:

- **Token layer**: `GraphToken.sol` and `L2GraphToken.sol` represent GRT on Ethereum and Arbitrum.
- **Governance and upgrades**: `Controller.sol`, `Governed.sol`, and `GraphProxyAdmin.sol` define privileged control and upgrade surfaces.
- **Staking and rewards**: `L1Staking.sol`, `StakingExtension.sol`, `HorizonStaking.sol`, and `RewardsManager.sol` manage stake-backed participation and incentives.
- **Subgraph discovery and curation**: `L1GNS.sol`, `L2GNS.sol`, `SubgraphNFT.sol`, `Curation.sol`, and `L2Curation.sol` handle subgraph identity, ownership, and signal.
- **Disputes and epochs**: `DisputeManager.sol`, `EpochManager.sol`, `SubgraphService.sol`, and related interfaces govern timing and challenge flows.
- **Payments**: `AllocationExchange.sol`, `GraphPayments.sol`, `PaymentsEscrow.sol`, and `GraphTallyCollector.sol` support settlement and escrow.
- **Token distribution**: `GraphTokenLock.sol`, `GraphTokenLockManager.sol`, `GraphTokenLockWallet.sol`, and L1/L2 lock managers enforce locked-token flows.
- **Cross-chain infrastructure**: `BridgeEscrow.sol`, `L1GraphTokenGateway.sol`, `L2GraphTokenGateway.sol`, and Arbitrum bridge interfaces manage L1/L2 value transfer.

Taken together, the contracts suggest a protocol where participants stake or delegate GRT, signal on useful subgraphs, earn rewards for providing services, settle payments for protocol usage, and move assets and positions across Ethereum and Arbitrum.

## Main architecture and how it works

### 1. GRT as the base asset

GRT is the core unit of value across the protocol. The main value-bearing surfaces called out in scope are:

- liquid token balances on L1 and L2,
- staked or delegated balances,
- rewards distributions,
- curated capital or shares,
- escrowed payments,
- locked distribution balances,
- bridge escrow and gateway custody.

This means most protocol-critical invariants reduce to **conservation, authorization, and one-time claimability of GRT** as it moves through different subsystems.

### 2. Governance and shared control plane

`Governed.sol` and `Controller.sol` indicate a common governance-controlled configuration layer. This likely provides a registry or authority mechanism used by the rest of the protocol. `GraphProxyAdmin.sol` shows that at least part of the system is upgradeable and centrally administered through a proxy-admin pattern.

In practice, this means governance probably controls:

- protocol parameter updates,
- contract references and dependency wiring,
- upgrade execution,
- privileged maintenance operations,
- possibly pausing or migration actions.

This governance layer is a central trust boundary because mistakes or overbroad permissions here could affect all downstream modules.

### 3. Staking, delegation, and rewards

The staking subsystem appears in both legacy/core and newer Horizon-era forms:

- `L1Staking.sol`
- `StakingExtension.sol`
- `HorizonStaking.sol`
- `RewardsManager.sol`
- `IHorizonStakingMain.sol`

From the provided documentation, delegation is part of the protocol’s security model. The likely flow is:

1. Participants stake GRT to back protocol activity.
2. Delegators attach capital to service providers or indexers.
3. Rewards accrue over time based on protocol rules.
4. Rewards are distributed through `RewardsManager` or related staking modules.
5. Disputes, epoch state, or service results may affect reward or stake outcomes.

`StakingExtension.sol` suggests extension or migration behavior layered on top of base staking. `HorizonStaking.sol` indicates a newer staking system on Arbitrum tied to the Horizon architecture.

The key accounting model here is probably share- or balance-based bookkeeping across deposits, undelegations, withdrawals, reward distribution, and possibly slashing or dispute-linked penalties. Since the provided docs do not define formulas, the most important inferred property is that stake and rewards must remain internally consistent across epoch transitions and migrations.

### 4. Epoch-based timing

`EpochManager.sol` suggests that important protocol actions are grouped into discrete time windows. In a protocol like this, epochs commonly determine:

- reward accrual windows,
- activation/deactivation timing,
- challenge windows,
- settlement eligibility,
- snapshot-based accounting.

This means timing is not incidental; it is a first-class coordination primitive. Any contract depending on current versus previous epoch state must preserve correct behavior at boundaries, especially around late claims, stale state, and off-by-one errors.

### 5. Subgraph discovery, ownership, and curation

The Graph’s user-facing product centers on **subgraphs**, and the smart contracts appear to support their on-chain identity and signaling layers.

Relevant contracts:

- `L1GNS.sol`
- `L2GNS.sol`
- `SubgraphNFT.sol`
- `Curation.sol`
- `L2Curation.sol`

A reasonable high-level flow from the provided context is:

1. A subgraph is published or registered through a Graph Name Service-style system.
2. Ownership or control may be represented through `SubgraphNFT.sol`.
3. Participants signal value or demand for a subgraph through curation.
4. L1 and L2 variants mirror or split these responsibilities across Ethereum and Arbitrum.

`Curation.sol` and `L2Curation.sol` likely manage some form of curation share or signal accounting. The docs explicitly warn that bonding-curve or mint/burn mechanics may exist here, even though formulas are not included. That makes curation an economic subsystem rather than a simple metadata registry.

`L1GNS.sol` and `L2GNS.sol` imply that subgraph identity, naming, or publication exists across chains. The major design requirement is consistency: ownership, identifiers, and associated curation state must not desynchronize in ways that redirect permissions or value.

### 6. Disputes and service validation

The presence of both `disputes/DisputeManager.sol` and `packages/subgraph-service/contracts/DisputeManager.sol`, alongside `SubgraphService.sol`, indicates that dispute resolution is an important part of protocol enforcement.

This suggests a system where service quality, behavior, or settlement outcomes can be challenged. The likely lifecycle is:

1. A participant provides protocol service.
2. A result, payment claim, or service event is recorded or relied upon.
3. Another participant may challenge or dispute that behavior.
4. The dispute manager resolves the dispute according to protocol rules.
5. The outcome may affect stake, payments, rewards, or service eligibility.

Because there are multiple dispute-related contracts, there may be legacy and Horizon-specific dispute paths, or service-specific dispute layers. The provided context does not define arbitrators, evidence, appeals, or penalties, so those are important unknowns. Still, architecturally, disputes appear to be the mechanism that converts protocol rules into enforceable economic consequences.

### 7. Payments and escrow

The payments subsystem is scoped through:

- `AllocationExchange.sol`
- `GraphPayments.sol`
- `PaymentsEscrow.sol`
- `GraphTallyCollector.sol`

This strongly suggests that protocol usage can generate payable obligations, which are then settled through an escrow-based flow rather than direct, trustless immediate transfers alone.

A likely model is:

1. Funds are deposited or reserved in an escrow contract.
2. A service or allocation-related event generates a claimable entitlement.
3. A collector, tally mechanism, or authorized participant submits settlement data.
4. Escrow releases funds to the correct party.

`AllocationExchange.sol` suggests that payments may be attached to allocations or resource commitments in the protocol. `GraphTallyCollector.sol` implies some off-chain or semi-off-chain aggregation layer whose results are finalized on-chain. That introduces an important trust boundary: the chain may enforce accounting and authorization, while the source of the tally may originate elsewhere.

The critical protocol property here is that liabilities in escrow must never exceed available funds, and claims must be protected against replay, duplication, stale authorization, or forged approvals.

### 8. Token distribution and locked wallets

The token-distribution subsystem is especially important because Immunefi explicitly lists `GraphTokenLockWallet` as an in-scope deployed asset.

Relevant contracts include:

- `GraphTokenLock.sol`
- `GraphTokenLockManager.sol`
- `GraphTokenLockWallet.sol`
- `IGraphTokenLockManager.sol`
- `L1GraphTokenLockTransferTool.sol`
- `L2GraphTokenLockManager.sol`

The role of these contracts is to manage **restricted or vesting-style GRT balances**. The likely model is:

1. Tokens are placed under lock terms.
2. A lock wallet or manager enforces beneficiary rights and timing rules.
3. Locked positions may be transferred, migrated, or mirrored across L1/L2.
4. Beneficiaries eventually access unlocked balances according to contract logic.

This is a highly sensitive custody surface because any bug here could allow:

- premature unlocks,
- unauthorized withdrawal,
- duplicated claims,
- manager overreach,
- stranded balances,
- broken preservation of restrictions during cross-chain movement.

The presence of both legacy `graphprotocol-token-distribution` and monorepo `packages/token-distribution` code suggests that token distribution has evolved over time, and versioning matters when reasoning about deployment behavior.

### 9. L1/L2 bridging and escrow

Cross-chain movement between Ethereum and Arbitrum is handled by:

- `BridgeEscrow.sol`
- `L1GraphTokenGateway.sol`
- `L2GraphTokenGateway.sol`
- `L2GraphToken.sol`
- `IBridge.sol`
- `ITokenGateway.sol`

This subsystem likely works as follows:

1. On L1, GRT is deposited into a gateway or escrow.
2. A bridge message is sent to Arbitrum.
3. On L2, the corresponding amount is minted or released.
4. The reverse path burns or escrows on L2 and releases on L1.

`BridgeEscrow.sol` is the custody anchor on Ethereum. `L1GraphTokenGateway.sol` and `L2GraphTokenGateway.sol` are the chain-specific controllers. `L2GraphToken.sol` represents the bridged token on Arbitrum.

The core invariant is **asset conservation across domains**: every L2 representation must correspond to a legitimate L1 escrowed or deposited amount, and every release must correspond to a valid burn or authenticated message. Since the system integrates with Arbitrum bridge interfaces, it also inherits correctness requirements around counterpart validation, retry handling, replay resistance, and failed-message behavior.

## Versioning and implementation context

The prompt provides useful package-level signals:

- `@graphprotocol/contracts` is version `7.3.0` and uses `@openzeppelin/contracts` `3.4.2`.
- `@graphprotocol/token-distribution` appears in two lines of development, including older `1.2.0` and newer monorepo `3.0.0` package metadata.
- `@graphprotocol/horizon` is version `1.1.1` and uses OpenZeppelin `^5.0.2`.
- `@graphprotocol/subgraph-service` is version `1.1.1` and also uses OpenZeppelin `^5.0.2`.
- Interface tooling is configured for Solidity `0.8.27` and `0.7.6`.

This implies the protocol is not a single-era codebase. It contains:

- older contracts and patterns tied to OpenZeppelin 3.x and Solidity 0.7-era compatibility,
- newer Horizon and subgraph-service contracts tied to OpenZeppelin 5.x and modern Solidity toolchains,
- legacy and current deployment tracks that likely coexist across Ethereum and Arbitrum.

That matters because protocol behavior may differ materially across modules even if they serve related purposes.

## End-to-end protocol picture

From the provided material, the broad protocol workflow looks like this:

1. **GRT exists on Ethereum and Arbitrum** and can be bridged between chains.
2. **Participants lock, stake, or delegate GRT** to take part in protocol economics.
3. **Subgraphs are registered and owned** through name-service and NFT-like primitives.
4. **Curators signal toward subgraphs**, likely affecting discoverability or economic incentives.
5. **Service providers perform work** related to indexing or query infrastructure.
6. **Epochs define timing boundaries** for accounting, rewards, and state transitions.
7. **Rewards are distributed** through staking and rewards modules.
8. **Payments are escrowed and settled** through payment contracts and tally/collector mechanisms.
9. **Disputes enforce rules** when service behavior, settlement, or protocol obligations are contested.
10. **Governance and proxy administration** retain protocol-wide authority to configure and upgrade the system.

## Trust boundaries and security-critical assumptions

The prompt makes clear that the largest trust boundaries are:

- governance and proxy admins,
- lock managers and token lock wallet controllers,
- bridge gateway counterparts,
- payment collectors and tally submitters,
- dispute resolvers or service authorities,
- staking/reward administrators.

In practical terms, the protocol depends on strong correctness in these areas:

- no unauthorized minting, release, or withdrawal of GRT,
- no duplication across bridge, lock, staking, escrow, or reward flows,
- no loss of synchronization between L1 and L2 state where mirrored behavior exists,
- no replay of claims, disputes, signatures, or cross-chain messages,
- no upgrade/admin path that silently expands fund-moving authority beyond what the protocol intends.

## What is known versus unknown from the provided context

The provided prompt is strong on architecture and scope, but it intentionally does **not** define many precise mechanics. Unknowns that must come from code include:

- exact staking and delegation formulas,
- reward issuance math,
- slashing or dispute penalty rules,
- curation pricing or share logic,
- subgraph lifecycle permissions,
- payment voucher or tally formats,
- bridge finality and retry semantics,
- token lock schedules and beneficiary rights,
- upgrade initialization and storage compatibility details.

So the safest protocol-level conclusion is this:

**The Graph’s smart contracts form a multi-module GRT-based coordination system for decentralized indexing and query infrastructure, spanning governance, staking, subgraph identity, curation, disputes, payments, token distribution, and L1/L2 bridging.** Its design is economically dense and cross-chain, with the most important invariants centered on GRT conservation, role-restricted state transitions, correct epoch and dispute handling, and consistent identity/accounting across Ethereum and Arbitrum.


 ------------ ## Main List of Files in Project ------------ 

graphprotocol-contracts/packages/contracts/contracts/curation/Curation.sol
graphprotocol-contracts/packages/contracts/contracts/discovery/L1GNS.sol
graphprotocol-contracts/packages/contracts/contracts/discovery/SubgraphNFT.sol
graphprotocol-contracts/packages/contracts/contracts/disputes/DisputeManager.sol
graphprotocol-contracts/packages/contracts/contracts/epochs/EpochManager.sol
graphprotocol-contracts/packages/contracts/contracts/gateway/BridgeEscrow.sol
graphprotocol-contracts/packages/contracts/contracts/gateway/L1GraphTokenGateway.sol
graphprotocol-contracts/packages/contracts/contracts/governance/Controller.sol
graphprotocol-contracts/packages/contracts/contracts/governance/Governed.sol
graphprotocol-contracts/packages/contracts/contracts/l2/curation/L2Curation.sol
graphprotocol-contracts/packages/contracts/contracts/l2/discovery/L2GNS.sol
graphprotocol-contracts/packages/contracts/contracts/l2/gateway/L2GraphTokenGateway.sol
graphprotocol-contracts/packages/contracts/contracts/l2/token/L2GraphToken.sol
graphprotocol-contracts/packages/contracts/contracts/payments/AllocationExchange.sol
graphprotocol-contracts/packages/contracts/contracts/rewards/RewardsManager.sol
graphprotocol-contracts/packages/contracts/contracts/staking/L1Staking.sol
graphprotocol-contracts/packages/contracts/contracts/staking/StakingExtension.sol
graphprotocol-contracts/packages/contracts/contracts/token/GraphToken.sol
graphprotocol-contracts/packages/contracts/contracts/upgrades/GraphProxyAdmin.sol
graphprotocol-contracts/packages/horizon/contracts/payments/GraphPayments.sol
graphprotocol-contracts/packages/horizon/contracts/payments/PaymentsEscrow.sol
graphprotocol-contracts/packages/horizon/contracts/payments/collectors/GraphTallyCollector.sol
graphprotocol-contracts/packages/horizon/contracts/staking/HorizonStaking.sol
graphprotocol-contracts/packages/interfaces/contracts/contracts/arbitrum/IBridge.sol
graphprotocol-contracts/packages/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol
graphprotocol-contracts/packages/interfaces/contracts/horizon/internal/IHorizonStakingMain.sol
graphprotocol-contracts/packages/interfaces/contracts/subgraph-service/ISubgraphService.sol
graphprotocol-contracts/packages/subgraph-service/contracts/DisputeManager.sol
graphprotocol-contracts/packages/subgraph-service/contracts/SubgraphService.sol
graphprotocol-contracts/packages/token-distribution/contracts/GraphTokenLock.sol
graphprotocol-contracts/packages/token-distribution/contracts/GraphTokenLockManager.sol
graphprotocol-contracts/packages/token-distribution/contracts/GraphTokenLockWallet.sol
graphprotocol-contracts/packages/token-distribution/contracts/IGraphTokenLockManager.sol
graphprotocol-contracts/packages/token-distribution/contracts/L2GraphTokenLockManager.sol
graphprotocol-token-distribution/contracts/GraphTokenLock.sol
graphprotocol-token-distribution/contracts/GraphTokenLockManager.sol
graphprotocol-token-distribution/contracts/GraphTokenLockWallet.sol
graphprotocol-token-distribution/contracts/IGraphTokenLockManager.sol
graphprotocol-token-distribution/contracts/L1GraphTokenLockTransferTool.sol
graphprotocol-token-distribution/contracts/L2GraphTokenLockManager.sol
graphprotocol-token-distribution/contracts/arbitrum/ITokenGateway.sol


 ------------ ## DOCUMENTATION: ------------ 

 ### thegraph-docs.md

# The Graph Protocol Audit Context

## Scope Basis

This context is limited to the Smart Contract bounty material and the provided docs excerpt. Web/App assets, generic site content, marketing material, setup instructions, changelogs, and low-signal links were intentionally excluded.

Primary sources used:

- Immunefi The Graph program information: `https://immunefi.com/bug-bounty/thegraph/information/`
- Immunefi The Graph resources: `https://immunefi.com/bug-bounty/thegraph/resources/`
- The Graph docs entry page: `https://thegraph.com/docs`
- Machine-readable in-scope contract list from the provided context bundle

## Protocol Overview

The Graph is a blockchain data protocol for extracting, processing, indexing, streaming, and querying blockchain data. The docs describe it as a blockchain data solution powering applications, analytics, and AI across 80+ chains. Its main products include:

- **Subgraphs**: open APIs for extracting, processing, and querying smart contract data.
- **Substreams**: real-time and historical data streaming with parallel execution.
- **Token API**: token data queries with native MCP support.
- **Graph Node**: indexing blockchain data and serving GraphQL queries.
- **Firehose**: extracting blockchain data into flat files to speed synchronization.

For smart contract audit purposes, the provided scope indicates the on-chain protocol includes GRT token contracts, staking, curation, subgraph discovery/name-service components, disputes, rewards, payments, token distribution locks, governance/controller contracts, proxy administration, and L1/L2 bridge gateways.

## In-Scope Contract Architecture

The in-scope Solidity assets are grouped by function below. This grouping is derived from contract paths and names in the machine-readable scope.

### Token and Token Distribution

Relevant scope:

- `GraphToken.sol`
- `L2GraphToken.sol`
- `GraphTokenLock.sol`
- `GraphTokenLockManager.sol`
- `GraphTokenLockWallet.sol`
- `IGraphTokenLockManager.sol`
- `L1GraphTokenLockTransferTool.sol`
- `L2GraphTokenLockManager.sol`

The bounty overview specifically identifies **GraphTokenLockWallet** under Token Distribution on Ethereum mainnet. The token distribution contracts are security-critical because they likely control locked GRT balances, wallet access, lock management, and movement of locked positions across L1/L2. The provided material does not include vesting schedules, unlock formulas, admin permissions, or transfer rules, so those must be read from code.

Security review focus:

- Locked-token accounting and release conditions.
- Authorization for lock managers, lock wallets, and transfer tooling.
- Whether locked balances can be prematurely withdrawn, duplicated, bypassed, or stranded.
- Cross-chain lock transfer assumptions between L1 and L2 managers.
- Upgrade or manager privilege paths that can affect beneficiary balances.

### Governance, Controller, and Upgrade Administration

Relevant scope:

- `Controller.sol`
- `Governed.sol`
- `GraphProxyAdmin.sol`

The scoped governance contracts form a trust boundary around protocol configuration and upgrades. `Controller` and `Governed` suggest shared governance-controlled access patterns, while `GraphProxyAdmin` indicates proxy-admin functionality for upgradeable contracts.

Security review focus:

- Which addresses can set protocol parameters, replace contract references, pause flows, or upgrade implementations.
- Whether governance/admin actions can directly move user funds or only configure protocol behavior.
- Initialization and re-initialization protection.
- Proxy admin ownership, implementation compatibility, and storage layout safety.
- Trust assumptions around privileged roles versus permissionless actors.

### Staking and Rewards

Relevant scope:

- `L1Staking.sol`
- `StakingExtension.sol`
- `HorizonStaking.sol`
- `RewardsManager.sol`
- `IHorizonStakingMain.sol`

The docs include a video guide reference describing delegation as a form of staking that helps secure The Graph. The scoped contracts indicate staking exists on L1 and in the Horizon system, with rewards managed separately.

Security review focus:

- Stake accounting, delegation accounting, reward accrual, and reward withdrawal.
- Slashing or dispute-linked effects on stake, if present in code.
- Role separation among indexers, delegators, service providers, reward managers, and governance.
- Epoch-dependent reward calculations and edge cases around partial periods.
- Migration or extension behavior between `L1Staking`, `StakingExtension`, and `HorizonStaking`.

The provided docs excerpt does not define exact staking formulas, delegation shares, reward curves, thawing periods, or slashing conditions.

### Curation and Subgraph Discovery

Relevant scope:

- `Curation.sol`
- `L2Curation.sol`
- `L1GNS.sol`
- `L2GNS.sol`
- `SubgraphNFT.sol`

The Graph docs describe Subgraphs as open APIs used to extract, process, and query blockchain data. The scoped discovery and curation contracts indicate on-chain support for identifying subgraphs, representing subgraph ownership or identity, and signaling value or demand around subgraphs.

Security review focus:

- Curation share accounting and any bonding-curve or mint/burn mechanics present in code.
- Subgraph publication, ownership, transfer, and NFT lifecycle.
- L1/L2 consistency for subgraph names, identifiers, and curation state.
- Permission checks for publishing, updating, deprecating, or transferring subgraphs.
- Failure modes where curation funds, subgraph ownership, or names become inconsistent across layers.

Exact GNS mechanics and curation formulas are not included in the provided source material and must be verified from code.

### Disputes, Epochs, and Service Validation

Relevant scope:

- `disputes/DisputeManager.sol`
- `epochs/EpochManager.sol`
- `subgraph-service/DisputeManager.sol`
- `SubgraphService.sol`
- `ISubgraphService.sol`

The scoped contracts show two dispute-manager locations and an epoch manager. These are likely important for validating service behavior, timing protocol actions, and applying dispute outcomes. The docs excerpt does not describe dispute lifecycle, evidence requirements, arbitrator roles, slashing, or appeal mechanics.

Security review focus:

- Who can open, accept, reject, escalate, or resolve disputes.
- Whether dispute outcomes can affect stake, payments, rewards, allocations, or service rights.
- Epoch boundary assumptions, off-by-one timing, and stale-epoch handling.
- Duplicate or replayed disputes across managers or chains.
- Trust boundaries between on-chain dispute state and off-chain indexing/query behavior.

### Payments and Escrow

Relevant scope:

- `AllocationExchange.sol`
- `GraphPayments.sol`
- `PaymentsEscrow.sol`
- `GraphTallyCollector.sol`

The scoped payment contracts indicate a protocol payment system with escrow and collection. `AllocationExchange` suggests payments may be tied to allocations, while `GraphTallyCollector` suggests an external or semi-external tally/collection component. The provided docs excerpt does not explain exact fee flows, voucher formats, signatures, or settlement conditions.

Security review focus:

- Deposit, escrow, release, collection, refund, and withdrawal accounting.
- Authorization for collectors and payees.
- Replay protection for payment claims, receipts, signatures, or tallies.
- Token transfer assumptions, allowance handling, and rounding.
- Whether service delivery is enforced on-chain or relies on off-chain attestations.

### L1/L2 Gateways and Bridge Escrow

Relevant scope:

- `BridgeEscrow.sol`
- `L1GraphTokenGateway.sol`
- `L2GraphTokenGateway.sol`
- `IBridge.sol`
- `ITokenGateway.sol`
- Arbitrum gateway interfaces

The in-scope gateway contracts and Arbitrum interfaces show that GRT movement between L1 and L2 is part of the audit surface. The Immunefi bounty pays rewards in GRT on Arbitrum, and scoped contracts include both L1 and L2 token/gateway components.

Security review focus:

- Token escrow/mint/burn accounting across L1 and L2.
- Bridge message authentication and allowed counterpart validation.
- Replay, retryable-ticket, failed-message, or duplicate-finalization behavior.
- Asset conservation across `BridgeEscrow`, L1 gateway, L2 gateway, and L2 token.
- Upgrade/admin privileges over bridge endpoints and escrowed funds.
- Assumptions inherited from Arbitrum bridge interfaces and gateway behavior.

The provided source material does not document bridge parameters, canonical gateway addresses, or finality assumptions.

## Main Protocol Flows to Audit

### 1. Subgraph Lifecycle

A high-level docs-backed flow is: developers publish data APIs called Subgraphs; Graph Node indexes blockchain data and serves queries; users and applications query that indexed data. The scoped contracts imply on-chain lifecycle components for subgraph discovery/name service, subgraph NFT ownership, and curation.

Audit-relevant questions:

- Can only authorized owners update or transfer subgraph identities?
- Can subgraph identity or curation state diverge between L1 and L2?
- Can metadata/name records be hijacked, replayed, or permanently locked?

### 2. Staking, Delegation, and Rewards

The docs identify delegation as staking that helps secure The Graph. Scoped contracts include L1 staking, Horizon staking, staking extensions, and rewards management.

Audit-relevant questions:

- Are stake/delegation shares conserved across deposits, withdrawals, migrations, and reward events?
- Can reward calculations be manipulated through epoch timing, allocation state, or rounding?
- Do dispute outcomes correctly affect stake or rewards if the code intends that behavior?

### 3. Payment Settlement and Escrow

Scoped payment contracts indicate escrowed payments, allocation exchange, and tally-based collection.

Audit-relevant questions:

- Are escrow balances conserved per payer/payee/service/allocation?
- Are claims protected against replay, double collection, invalid signatures, and stale authorization?
- Can a trusted collector or governance role drain funds beyond its intended authority?

### 4. Cross-Chain Token Movement

Scoped L1/L2 gateways, bridge escrow, Arbitrum bridge interfaces, and L2 GRT token contracts define a major value-flow boundary.

Audit-relevant questions:

- Does every L2 mint or release correspond to an authenticated L1 deposit or escrow event?
- Can failed or retried bridge messages create duplicate credits?
- Are gateway counterparts and token addresses immutable or safely governable?

### 5. Token Locking and Distribution

The Immunefi program overview highlights `GraphTokenLockWallet` under Token Distribution on Ethereum mainnet. Scoped token distribution contracts appear in both `graphprotocol-contracts` and `graphprotocol-token-distribution` paths.

Audit-relevant questions:

- Can a beneficiary, manager, or admin bypass lock terms?
- Can locked GRT be transferred cross-chain without preserving restrictions?
- Are lock wallet approvals, recoveries, and manager actions tightly scoped?

## Accounting and Value Flow

Primary value-bearing asset: **GRT**.

Observable value-flow surfaces from scope:

- GRT token balances and L2 GRT representations.
- Staked or delegated GRT in staking contracts.
- Rewards distributed through `RewardsManager` and possibly Horizon staking flows.
- Curated value or shares through L1/L2 curation contracts.
- Escrowed payments through payment and bridge escrow contracts.
- Locked token balances in token distribution contracts.
- Cross-chain GRT custody between L1 bridge escrow/gateway and L2 token/gateway contracts.

Core accounting invariants to verify from code:

- Token conservation across bridge escrow, L1 gateway, L2 gateway, and L2 token supply.
- No unauthorized increase, premature release, or duplicate claim of locked GRT.
- Staking shares, delegation balances, and rewards remain internally consistent after deposits, withdrawals, slashing/disputes, migrations, and epoch transitions.
- Payment escrow liabilities never exceed held funds and cannot be claimed twice.
- Curation shares and subgraph ownership state cannot be desynchronized in ways that misdirect funds or permissions.

## External Integrations

Documented or scoped integrations:

- **Arbitrum bridge/gateway interfaces**: `IBridge.sol`, `ITokenGateway.sol`, and Arbitrum-specific gateway interfaces are in scope.
- **Graph Node / indexing stack**: The docs describe Graph Node as indexing blockchain data and serving GraphQL queries, but Graph Node itself is not listed as a scoped smart contract asset.
- **Firehose/Substreams**: The docs describe these as blockchain data extraction/streaming systems. They are relevant to protocol context but are not directly described as Solidity trust roots in the provided material.
- **The Graph docs and smart contract READMEs**: Immunefi resources identify smart contract package documentation and overall docs as protocol documentation sources.

Security assumptions to check:

- On-chain contracts should not assume untrusted off-chain indexing/query output is authoritative unless validated by protocol rules.
- Bridge contracts rely on correct authentication of cross-chain messages and configured counterpart contracts.
- Payment collectors or tally systems may introduce off-chain trust assumptions; exact mechanics are not documented in the provided source material.

## Trust Boundaries and Privileged Roles

The provided source material does not enumerate trusted roles, but scoped contract names imply several trust boundaries that must be mapped from code:

- Governance/admin roles in `Controller`, `Governed`, and `GraphProxyAdmin`.
- Gateway/bridge counterparts and bridge escrow authority.
- Lock managers and token lock wallet controllers.
- Staking/rewards administrators or protocol parameter setters.
- Dispute resolvers, arbitrators, service managers, or challenge authorities if present.
- Payment collectors such as `GraphTallyCollector` and any authorized escrow release actors.

Audit priority should be given to paths where privileged roles can affect custody, minting, burning, escrow release, reward issuance, lock release, slashing, dispute outcomes, or upgradeability.

## Security-Relevant Assumptions and Unknowns

The provided docs are entry-level and do not include detailed mechanics for most smart contracts. The following must be established from code and package READMEs/audits before final severity judgment:

- Exact GRT token supply and mint/burn permissions.
- Staking, delegation, reward, allocation, and slashing formulas.
- Curation pricing/share formulas and subgraph lifecycle rules.
- Dispute lifecycle, evidence model, resolvers, and economic consequences.
- Payment claim formats, collector trust model, escrow release rules, and replay protection.
- Bridge counterpart configuration, canonical gateway addresses, finality assumptions, and failed-message handling.
- Token lock schedules, managers, wallet permissions, recoverability, and L1/L2 transfer behavior.
- Upgradeability model, timelocks, multisigs, initialization state, and storage-layout constraints.

## Immunefi Program Notes

Smart contract rewards:

- Critical: `$15,000` to `$50,000`
- High: `$5,000` to `$15,000`
- Reward token: `GRT`
- Reward network: `Arbitrum`
- Proof of Concept: required
- Primacy: program rules have primacy as stated in the Immunefi source

Known audits / prior reviews listed by Immunefi:

- OpenZeppelin Horizon audits: 2024-05-31, 2025-02-28, 2025-04-30
- Trust Horizon audit: 2024-10-31
- Trust contracts audit: 2023-01-31
- OpenZeppelin contracts audit: 2024-01-31
- ConsenSys Diligence contracts audit: 2022-02-28
- OpenZeppelin issuance audit: 2025-11-30, noted by Immunefi resources as possibly including upgrades not yet deployed or not reward-eligible until listed as in-scope

Prohibited activities from Immunefi source:

- Testing on mainnet or public testnet deployed code; use local forks only.
- Testing with pricing oracles or third-party smart contracts.
- Phishing or social engineering against employees or customers.
- Testing third-party systems, applications, websites, SSO providers, advertising networks, or browser extensions.
- Denial-of-service attacks against project assets.
- Automated service testing that generates significant traffic.
- Public disclosure of an unpatched vulnerability in an embargoed bounty.
- Any other actions prohibited by Immunefi rules.

## Auditor Checklist

Prioritize review of:

- GRT custody and conservation across token, staking, escrow, locks, rewards, and bridge flows.
- Privileged role authority and upgrade paths.
- Cross-chain gateway authentication and replay/failure handling.
- Lock wallet and token distribution bypasses.
- Payment escrow claim validity and double-spend prevention.
- Epoch-dependent accounting, stale state, and boundary conditions.
- Curation and subgraph ownership state transitions.
- Dispute effects on stake, rewards, payments, or service state.

Because the supplied docs are high-level, contract code and package-level READMEs/audits should be treated as necessary follow-up sources for exact mechanics.

### thegraph-immunefi-bounty-rules.md

# Immunefi Bounty Rules - The Graph

Use this file as mandatory context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/thegraph/information/
- Scope: https://immunefi.com/bug-bounty/thegraph/scope/
- Resources: https://immunefi.com/bug-bounty/thegraph/resources/

## Program Requirements

- Proof of Concept: required
- Primacy: primacy_of_rules
- Rewards token: GRT
- Rewards token network: Arbitrum
- Maximum bounty: $50000

## Assets In Scope

> Smart Contract category only. Web & App assets are intentionally excluded.

- smart contract: https://etherscan.io/address/0xbE5e630383b5BAEcF0Db7b15C50d410edD5A2255
  - Description: GraphTokenLockWallet (Token Distribution - https://github.com/graphprotocol/token-distribution ) (Ethereum mainnet)
- smart contract: https://arbiscan.io/address/0x971B9d3d0Ae3ECa029CAB5eA1fB0F72c85e6a525
  - Description: RewardsManager (Arbitrum)
- smart contract: https://etherscan.io/address/0xF55041E37E12cD407ad00CE2910B8269B01263b9
  - Description: L1Staking (Ethereum Mainnet)
- smart contract: https://etherscan.io/address/0xc944E90C64B2c07662A292be6244BDf05Cda44a7
  - Description: GraphToken (GRT) - Ethereum Mainnet
- smart contract: https://etherscan.io/address/0xaDcA0dd4729c8BA3aCf3E99F3A9f471EF37b6825
  - Description: L1GNS (Ethereum Mainnet)
- smart contract: https://etherscan.io/address/0x9Ac758AB77733b4150A901ebd659cbF8cB93ED66
  - Description: RewardsManager (Ethereum Mainnet)
- smart contract: https://etherscan.io/address/0x97307b963662cCA2f7eD50e38dCC555dfFc4FB0b
  - Description: DisputeManager (Ethereum Mainnet)
- smart contract: https://etherscan.io/address/0x8FE00a685Bcb3B2cc296ff6FfEaB10acA4CE1538
  - Description: Curation (Ethereum Mainnet)
- smart contract: https://etherscan.io/address/0x8017B9AF3F199CC6b08A48DA3859410F20bbea72
  - Description: BillingConnector - Ethereum Mainnet
- smart contract: https://etherscan.io/address/0x74Db79268e63302d3FC69FB5a7627F7454a41732
  - Description: Governor - Ethereum Mainnet
- smart contract: https://etherscan.io/address/0x4Dbd4fc535Ac27206064B68FfCf827b0A60BAB3f
  - Description: ArbitrumInbox - Ethereum Mainnet
- smart contract: https://etherscan.io/address/0x36aFF7001294daE4C2ED4fDEfC478a00De77F090
  - Description: BridgeEscrow - Ethereum Mainnet
- smart contract: https://etherscan.io/address/0x01cDC91B0A9bA741903aA3699BF4CE31d6C5cC06
  - Description: L1GraphTokenGateway - Ethereum Mainnet
- smart contract: https://arbiscan.io/address/0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E
  - Description: PaymentsEscrow - Arbitrum One
- smart contract: https://arbiscan.io/address/0xec9A7fb6CbC2E41926127929c2dcE6e9c5D33Bec
  - Description: L2GNS - Arbitrum One
- smart contract: https://arbiscan.io/address/0xc9E1Aa57223aDD21cC88A03088aF552f1ea8A34a
  - Description: BanxaWrapper - Arbitrum One
- smart contract: https://arbiscan.io/address/0xb2Bb92d0DE618878E438b55D5846cfecD9301105
  - Description: SubgraphService - Arbitrum One
- smart contract: https://arbiscan.io/address/0x993F00C98D1678371a7b261Ed0E0D4b6F42d9aEE
  - Description: AllocationExchange (Arbitrum)
- smart contract: https://arbiscan.io/address/0x9623063377AD1B27544C965cCd7342f7EA7e88C7
  - Description: L2GraphToken (GRT) - Arbitrum One
- smart contract: https://arbiscan.io/address/0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e
  - Description: GraphTallyCollector - Arbitrum One
- smart contract: https://arbiscan.io/address/0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D
  - Description: GraphPayments - Arbitrum One
- smart contract: https://arbiscan.io/address/0x76C00F71F4dACE63fd83eC80dBc8c30a88B2891c
  - Description: Collector - Arbitrum One
- smart contract: https://arbiscan.io/address/0x65E1a5e8946e7E87d9774f5288f41c30a99fD302
  - Description: L2GraphTokenGateway - Arbitrum One
- smart contract: https://arbiscan.io/address/0x5A843145c43d328B9bB7a4401d94918f131bB281
  - Description: EpochManager - Arbitrum One
- smart contract: https://arbiscan.io/address/0x3FbD54f0cc17b7aE649008dEEA12ed7D2622B23f
  - Description: SubgraphNFT - Arbitrum One
- smart contract: https://arbiscan.io/address/0x3bE385576d7C282070Ad91BF94366de9f9ba3571
  - Description: StakingExtension (Arbitrum)
- smart contract: https://arbiscan.io/address/0x2FE023a575449AcB698648eD21276293Fa176f96
  - Description: DisputeManager - Arbitrum One
- smart contract: https://arbiscan.io/address/0x2983936aC20202a6555993448E0d5654AC8Ca5fd
  - Description: GraphProxyAdmin - Arbitrum One
- smart contract: https://arbiscan.io/address/0x270Ea4ea9e8A699f8fE54515E3Bb2c418952623b
  - Description: Governor - Arbitrum One
- smart contract: https://arbiscan.io/address/0x22d78fb4bc72e191C765807f8891B5e1785C8014
  - Description: L2Curation - Arbitrum One
- smart contract: https://arbiscan.io/address/0x1B07D3344188908Fb6DEcEac381f3eE63C48477a
  - Description: Billing - Arbitrum One
- smart contract: https://arbiscan.io/address/0x0Ab2B043138352413Bb02e67E626a70320E3BD46
  - Description: LegacyDisputeManager (Arbitrum)
- smart contract: https://arbiscan.io/address/0x0a8491544221dd212964fbb96487467291b2C97e
  - Description: Controller - Arbitrum One
- smart contract: https://arbiscan.io/address/0x072884c745c0A23144753335776c99BE22588f8A
  - Description: LegacyServiceRegistry (Arbitrum)
- smart contract: https://arbiscan.io/address/0x00669A4CF01450B64E8A2A20E9b1FCB71E61eF03
  - Description: HorizonStaking - Arbitrum One

## Impacts In Scope

- high (smart contract): Private information being stolen
- high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- high (smart contract): A bug that could cause network participants to be impersonated and unwanted actions being taken (eg., User funds being stolen directly from the protocol smart contracts)
- critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)

## Out Of Scope And Exclusions

### Smart Contract Out Of Scope

- Incorrect data supplied by third party oracles
  - Not to exclude oracle manipulation/flash loan attacks
- Impacts requiring basic economic and governance attacks (e.g. 51% attack)
- Lack of liquidity impacts
- Impacts from Sybil attacks
- Impacts involving centralization risks

### General Out Of Scope

- Impacts requiring attacks that the reporter has already exploited themselves, leading to damage
- Impacts caused by attacks requiring access to leaked keys/credentials
- Impacts caused by attacks requiring access to privileged addresses (including, but not limited to: governance and strategist contracts) without additional modifications to the privileges attributed
- Impacts relying on attacks involving the depegging of an external stablecoin where the attacker does not directly cause the depegging due to a bug in code
- Mentions of secrets, access tokens, API keys, private keys, etc. in Github will be considered out of scope without proof that they are in-use in production
- Best practice recommendations
- Feature requests
- Impacts on test files and configuration files unless stated otherwise in the bug bounty program
- Impacts requiring phishing or other social engineering attacks against project's employees and/or customers

### Prohibited Activities

- Any testing on mainnet or public testnet deployed code; all testing should be done on local-forks of either public testnet or mainnet
- Any testing with pricing oracles or third-party smart contracts
- Attempting phishing or other social engineering attacks against our employees and/or customers
- Any testing with third-party systems and applications (e.g. browser extensions) as well as websites (e.g. SSO providers, advertising networks)
- Any denial of service attacks that are executed against project assets
- Automated testing of services that generates significant amounts of traffic
- Public disclosure of an unpatched vulnerability in an embargoed bounty
- [Any other actions prohibited by the Immunefi Rules](https://immunefi.com/rules/)

## Audit And Documentation Exclusions

- OpenZeppelin (2025-02-28T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/horizon/audits
- OpenZeppelin (2024-05-31T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/horizon/audits
- Trust (2023-01-31T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/contracts/audits
- OpenZeppelin (2025-04-30T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/horizon/audits
- OpenZeppelin (2024-01-31T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/contracts/audits
- ConsenSys Diligence (2022-02-28T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/contracts/audits
- OpenZeppelin (2025-11-30T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/issuance/audits
- Trust (2024-10-31T00:00:00.000Z): https://github.com/graphprotocol/contracts/tree/main/packages/horizon/audits

## Stage 1 Eligibility Rules

- A finding must affect an in-scope asset, unless the exact category and severity are covered by the program's Primacy of Impact rules.
- A finding must produce an impact listed in the program's Impacts in Scope.
- Exclude known issues, prior audit findings, documented accepted risks, closed duplicate reports, and program-specific OOS cases.
- Exclude cases requiring privileged access, leaked credentials, social engineering, malicious or mistaken trusted roles, deployment mistakes, test/mock files, public disclosure, or third-party-only failures.
- Do not perform final exploitability or severity scoring in stage 1; keep only when there is no decisive eligibility blocker.

## Link Handling

- Do not follow Immunefi navigation, marketing, login, social, newsletter, or platform-help links during validation.
- Use only the Source URLs above, in-scope explorer links, codebase links, documentation links, and prior-audit links when live verification is necessary.



### thegraph-immunefi-severity-rubric.md

# Immunefi Severity Rubric - The Graph

Use this file as mandatory severity context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/thegraph/information/
- Scope: https://immunefi.com/bug-bounty/thegraph/scope/
- Resources: https://immunefi.com/bug-bounty/thegraph/resources/
- Immunefi severity system v2.3: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-3/

## Program-Specific Severity Source Of Truth

- Primacy: primacy_of_rules
- Proof of Concept: required

## Impacts In Scope

- high (smart contract): Private information being stolen
- high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- high (smart contract): A bug that could cause network participants to be impersonated and unwanted actions being taken (eg., User funds being stolen directly from the protocol smart contracts)
- critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)

## Rewards By Threat Level

- critical (smart contract) [range]: min $15000, max $50000
- high (smart contract) [range]: min $5000, max $15000

## Platform Smart Contract Severity Summary

Always prefer the program's exact impact rows above. Use this summary only to interpret the referenced Immunefi severity system.

### v2.3 Smart Contract Summary

- Critical: direct theft of funds or NFTs, permanent freezing, protocol insolvency, governance result manipulation, unauthorized NFT minting, manipulable RNG abuse, or NFT representation alteration when listed by the program.
- High: theft or permanent freezing of unclaimed yield/royalties, temporary freezing of funds/NFTs, or other High rows listed by the program.
- Medium: griefing, block stuffing, gas theft, unbounded gas, or liveness failures only when listed by the program.
- Low/Insight: lower-impact failures only when listed and rewarded by the program.

## Immunefi Severity Decision Rules

- Stage 3 must match the finding to an exact program impact row and severity.
- Apply Primacy of Impact only for the category and severity levels explicitly covered by this bounty.
- Under Primacy of Rules, both the impacted asset and impact must be in scope.
- Downgrade or reject findings requiring privileged access, leaked keys, malicious trusted roles, unusual user mistakes, unrealistic repeated interactions, or external-only failures.
- Feasibility limitations can affect payout and confidence; they should not replace the program's listed impact rows.
- Mark ambiguous, medium-only, best-practice-only, or weak-evidence findings as `Invalid` or `Needs Review`, not submission-ready.
- PoC policy is recorded here for later PoC stages only; stage 3 should not require an already-created PoC.

## PoC Policy For Later Stages Only

- Prefer a runnable local mainnet fork PoC when the affected in-scope asset is deployed on mainnet.
- Use the generated Immunefi PoC runtime artifact to select deployed addresses, networks, and RPC env var names.
- Use a local public-testnet fork only when the affected in-scope asset itself is a public-testnet deployment, or when no matching mainnet deployment exists but a relevant in-scope public-testnet deployment does.
- Do not use local non-fork tests as the primary proof for deployed-asset findings.
- Never broadcast live transactions, mutate live protocol state, steal funds, freeze funds, manipulate live governance, or cause real harm, even for a tiny amount.

## Link Handling

- Do not follow Immunefi navigation, marketing, login, social, newsletter, or platform-help links during validation.
- Use only the Source URLs above, in-scope explorer links, codebase links, documentation links, and prior-audit links when live verification is necessary.



### thegraph-immunefi-poc-runtime.md

# Immunefi PoC Runtime - The Graph

Use this file as mandatory context for Immunefi R5 PoC generation and R6 PoC verification.

## Fork Preference

- Fork PoCs allowed: `true`
- Fork PoCs preferred: `true`
- Prefer a mainnet fork PoC whenever the finding touches deployed in-scope mainnet contracts and a matching RPC env var is available.
- Use a public-testnet fork only when the in-scope asset itself is a public-testnet deployment, or when no matching mainnet deployment exists but a relevant in-scope public-testnet deployment does.
- Do not use a local non-fork test as the primary proof for an Immunefi deployed-asset finding.
- Never invent RPC URLs, deployed addresses, networks, or block numbers.

## In-Scope Deployed Contracts

| Description | Type | Address | Network | RPC Env Var | Env Available | Explorer |
| --- | --- | --- | --- | --- | --- | --- |
| HorizonStaking - Arbitrum One | smart_contract | `0x00669A4CF01450B64E8A2A20E9b1FCB71E61eF03` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x00669A4CF01450B64E8A2A20E9b1FCB71E61eF03 |
| LegacyServiceRegistry (Arbitrum) | smart_contract | `0x072884c745c0A23144753335776c99BE22588f8A` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x072884c745c0A23144753335776c99BE22588f8A |
| LegacyDisputeManager (Arbitrum) | smart_contract | `0x0Ab2B043138352413Bb02e67E626a70320E3BD46` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x0Ab2B043138352413Bb02e67E626a70320E3BD46 |
| Controller - Arbitrum One | smart_contract | `0x0a8491544221dd212964fbb96487467291b2C97e` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x0a8491544221dd212964fbb96487467291b2C97e |
| Billing - Arbitrum One | smart_contract | `0x1B07D3344188908Fb6DEcEac381f3eE63C48477a` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x1B07D3344188908Fb6DEcEac381f3eE63C48477a |
| L2Curation - Arbitrum One | smart_contract | `0x22d78fb4bc72e191C765807f8891B5e1785C8014` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x22d78fb4bc72e191C765807f8891B5e1785C8014 |
| Governor - Arbitrum One | smart_contract | `0x270Ea4ea9e8A699f8fE54515E3Bb2c418952623b` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x270Ea4ea9e8A699f8fE54515E3Bb2c418952623b |
| GraphProxyAdmin - Arbitrum One | smart_contract | `0x2983936aC20202a6555993448E0d5654AC8Ca5fd` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x2983936aC20202a6555993448E0d5654AC8Ca5fd |
| DisputeManager - Arbitrum One | smart_contract | `0x2FE023a575449AcB698648eD21276293Fa176f96` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x2FE023a575449AcB698648eD21276293Fa176f96 |
| SubgraphNFT - Arbitrum One | smart_contract | `0x3FbD54f0cc17b7aE649008dEEA12ed7D2622B23f` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x3FbD54f0cc17b7aE649008dEEA12ed7D2622B23f |
| StakingExtension (Arbitrum) | smart_contract | `0x3bE385576d7C282070Ad91BF94366de9f9ba3571` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x3bE385576d7C282070Ad91BF94366de9f9ba3571 |
| EpochManager - Arbitrum One | smart_contract | `0x5A843145c43d328B9bB7a4401d94918f131bB281` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x5A843145c43d328B9bB7a4401d94918f131bB281 |
| L2GraphTokenGateway - Arbitrum One | smart_contract | `0x65E1a5e8946e7E87d9774f5288f41c30a99fD302` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x65E1a5e8946e7E87d9774f5288f41c30a99fD302 |
| Collector - Arbitrum One | smart_contract | `0x76C00F71F4dACE63fd83eC80dBc8c30a88B2891c` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x76C00F71F4dACE63fd83eC80dBc8c30a88B2891c |
| GraphPayments - Arbitrum One | smart_contract | `0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D |
| GraphTallyCollector - Arbitrum One | smart_contract | `0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e |
| L2GraphToken (GRT) - Arbitrum One | smart_contract | `0x9623063377AD1B27544C965cCd7342f7EA7e88C7` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x9623063377AD1B27544C965cCd7342f7EA7e88C7 |
| RewardsManager (Arbitrum) | smart_contract | `0x971B9d3d0Ae3ECa029CAB5eA1fB0F72c85e6a525` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x971B9d3d0Ae3ECa029CAB5eA1fB0F72c85e6a525 |
| AllocationExchange (Arbitrum) | smart_contract | `0x993F00C98D1678371a7b261Ed0E0D4b6F42d9aEE` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0x993F00C98D1678371a7b261Ed0E0D4b6F42d9aEE |
| SubgraphService - Arbitrum One | smart_contract | `0xb2Bb92d0DE618878E438b55D5846cfecD9301105` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0xb2Bb92d0DE618878E438b55D5846cfecD9301105 |
| BanxaWrapper - Arbitrum One | smart_contract | `0xc9E1Aa57223aDD21cC88A03088aF552f1ea8A34a` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0xc9E1Aa57223aDD21cC88A03088aF552f1ea8A34a |
| L2GNS - Arbitrum One | smart_contract | `0xec9A7fb6CbC2E41926127929c2dcE6e9c5D33Bec` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0xec9A7fb6CbC2E41926127929c2dcE6e9c5D33Bec |
| PaymentsEscrow - Arbitrum One | smart_contract | `0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E` | `arbitrum-mainnet` | `ARBITRUM_RPC_URL` | `true` | https://arbiscan.io/address/0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E |
| L1GraphTokenGateway - Ethereum Mainnet | smart_contract | `0x01cDC91B0A9bA741903aA3699BF4CE31d6C5cC06` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x01cDC91B0A9bA741903aA3699BF4CE31d6C5cC06 |
| BridgeEscrow - Ethereum Mainnet | smart_contract | `0x36aFF7001294daE4C2ED4fDEfC478a00De77F090` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x36aFF7001294daE4C2ED4fDEfC478a00De77F090 |
| ArbitrumInbox - Ethereum Mainnet | smart_contract | `0x4Dbd4fc535Ac27206064B68FfCf827b0A60BAB3f` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x4Dbd4fc535Ac27206064B68FfCf827b0A60BAB3f |
| Governor - Ethereum Mainnet | smart_contract | `0x74Db79268e63302d3FC69FB5a7627F7454a41732` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x74Db79268e63302d3FC69FB5a7627F7454a41732 |
| BillingConnector - Ethereum Mainnet | smart_contract | `0x8017B9AF3F199CC6b08A48DA3859410F20bbea72` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x8017B9AF3F199CC6b08A48DA3859410F20bbea72 |
| Curation (Ethereum Mainnet) | smart_contract | `0x8FE00a685Bcb3B2cc296ff6FfEaB10acA4CE1538` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x8FE00a685Bcb3B2cc296ff6FfEaB10acA4CE1538 |
| DisputeManager (Ethereum Mainnet) | smart_contract | `0x97307b963662cCA2f7eD50e38dCC555dfFc4FB0b` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x97307b963662cCA2f7eD50e38dCC555dfFc4FB0b |
| RewardsManager (Ethereum Mainnet) | smart_contract | `0x9Ac758AB77733b4150A901ebd659cbF8cB93ED66` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x9Ac758AB77733b4150A901ebd659cbF8cB93ED66 |
| L1Staking (Ethereum Mainnet) | smart_contract | `0xF55041E37E12cD407ad00CE2910B8269B01263b9` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xF55041E37E12cD407ad00CE2910B8269B01263b9 |
| L1GNS (Ethereum Mainnet) | smart_contract | `0xaDcA0dd4729c8BA3aCf3E99F3A9f471EF37b6825` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xaDcA0dd4729c8BA3aCf3E99F3A9f471EF37b6825 |
| GraphTokenLockWallet (Token Distribution - https://github.com/graphprotocol/token-distribution ) (Ethereum mainnet) | smart_contract | `0xbE5e630383b5BAEcF0Db7b15C50d410edD5A2255` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xbE5e630383b5BAEcF0Db7b15C50d410edD5A2255 |
| GraphToken (GRT) - Ethereum Mainnet | smart_contract | `0xc944E90C64B2c07662A292be6244BDf05Cda44a7` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xc944E90C64B2c07662A292be6244BDf05Cda44a7 |

## Network RPC Availability

| Network | Kind | RPC Env Var | Env Available | Asset Count |
| --- | --- | --- | --- | --- |
| `arbitrum-mainnet` | `mainnet` | `ARBITRUM_RPC_URL` | `true` | `23` |
| `ethereum-mainnet` | `mainnet` | `MAINNET_RPC_URL` | `true` | `12` |

## Foundry Command Templates

Use env vars, not raw URLs:

```bash
set -a; source "<AI_AGENT_AUDIT_ROOT>/.env"; set +a; forge test --match-test <testName> --fork-url "$MAINNET_RPC_URL"
set -a; source "<AI_AGENT_AUDIT_ROOT>/.env"; set +a; forge test --match-path test/<PoCFile>.t.sol --fork-url "$ARBITRUM_RPC_URL"
```

## Safety Rules

- Fork PoCs must be local simulations only.
- Do not broadcast transactions.
- Do not use live private keys or live privileged accounts.
- Do not mutate live mainnet or public-testnet protocol state.
- Do not steal, freeze, transfer, or manipulate real assets, even tiny amounts.



 ------------ ## PACKAGE.JSON HEADERS OF LIB PACKAGES ------------ 

 *Note*: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 
 ------------ ## CONFIG FILES ------------ 

 *Note*: Check for important package version info.

 ### graphprotocol-token-distribution/.env.sample

MNEMONIC=
ETHERSCAN_API_KEY=
INFURA_KEY=


### graphprotocol-token-distribution/package.json

{
  "name": "@graphprotocol/token-distribution",
  "version": "1.2.0",
  "description": "Graph Token Distribution",
  "main": "index.js",
  "scripts": {
    "prepublishOnly": "scripts/prepublish",
    "build": "scripts/build",
    "clean": "rm -rf build/ cache/ dist/ && hardhat clean",
    "compile": "hardhat compile --show-stack-traces",
    "deploy": "yarn run build && hardhat deploy",
    "test": "scripts/test",
    "test:gas": "RUN_EVM=true REPORT_GAS=true scripts/test",
    "test:coverage": "scripts/coverage",
    "lint": "yarn run lint:ts && yarn run lint:sol",
    "lint:fix": "yarn run lint:ts:fix && yarn run lint:sol:fix",
    "lint:ts": "eslint '*/**/*.{js,ts}'",
    "lint:ts:fix": "eslint '*/**/*.{js,ts}' --fix",
    "lint:sol": "solhint './contracts/**/*.sol'",
    "lint:sol:fix": "yarn prettier:sol && solhint --fix './contracts/**/*.sol'",
    "prettier": "yarn run prettier:ts && yarn run prettier:sol",
    "prettier:ts": "prettier --write 'test/**/*.ts'",
    "prettier:sol": "prettier --write 'contracts/**/*.sol'",
    "security": "scripts/security",
    "flatten": "scripts/flatten",
    "typechain": "hardhat typechain",
    "verify": "hardhat verify",
    "size": "hardhat size-contracts"
  },
  "files": [
    "dist/**/*",
    "README.md",
    "LICENSE"
  ],
  "author": "The Graph Team",
  "license": "MIT",
  "devDependencies": {
    "@ethersproject/experimental": "^5.0.7",
    "@graphprotocol/client-cli": "^2.0.2",
    "@graphprotocol/contracts": "^5.0.0",
    "@nomiclabs/hardhat-ethers": "^2.0.0",
    "@nomiclabs/hardhat-etherscan": "^3.1.7",
    "@nomiclabs/hardhat-waffle": "^2.0.0",
    "@openzeppelin/contracts": "^3.3.0-solc-0.7",
    "@openzeppelin/contracts-upgradeable": "3.4.2",
    "@openzeppelin/hardhat-upgrades": "^1.22.1",
    "@typechain/ethers-v5": "^7.0.0",
    "@typechain/hardhat": "^2.0.0",
    "@types/mocha": "^9.1.0",
    "@types/node": "^20.4.2",
    "@typescript-eslint/eslint-plugin": "^5.20.0",
    "@typescript-eslint/parser": "^5.20.0",
    "chai": "^4.2.0",
    "coingecko-api": "^1.0.10",
    "consola": "^2.15.0",
    "dotenv": "^16.0.0",
    "eslint": "^8.13.0",
    "eslint-config-prettier": "^8.5.0",
    "eslint-config-standard": "^16.0.3",
    "eslint-plugin-import": "^2.22.0",
    "eslint-plugin-mocha-no-only": "^1.1.1",
    "eslint-plugin-node": "^11.1.0",
    "eslint-plugin-prettier": "^4.0.0",
    "eslint-plugin-promise": "^6.0.0",
    "eslint-plugin-standard": "5.0.0",
    "ethereum-waffle": "^3.1.1",
    "ethers": "^5.0.18",
    "graphql": "^16.5.0",
    "hardhat": "^2.6.1",
    "hardhat-abi-exporter": "^2.0.1",
    "hardhat-contract-sizer": "^2.0.1",
    "hardhat-deploy": "^0.7.0-beta.9",
    "hardhat-gas-reporter": "^1.0.1",
    "inquirer": "8.0.0",
    "p-queue": "^6.6.2",
    "prettier": "^2.1.1",
    "prettier-plugin-solidity": "^1.0.0-alpha.56",
    "solhint": "^3.3.7",
    "solhint-plugin-prettier": "^0.0.5",
    "ts-node": "^10.9.1",
    "typechain": "^5.0.0",
    "typescript": "^4.0.2"
  },
  "dependencies": {}
}


### graphprotocol-contracts/packages/subgraph-service/foundry.toml

[profile.default]
src = 'contracts'
out = 'build'
libs = ["node_modules"]
test = 'test'
cache_path  = 'cache_forge'
fs_permissions = [{ access = "read", path = "./"}]
optimizer = true
optimizer_runs = 100
via_ir = true
evm_version = 'cancun'

# Exclude test files from coverage reports
no_match_coverage = "(^test/|/mocks/)"

# Lint configuration
[lint]
exclude_lints = ["mixed-case-function", "mixed-case-variable"]


### graphprotocol-contracts/packages/subgraph-service/package.json

{
  "name": "@graphprotocol/subgraph-service",
  "version": "1.1.1",
  "publishConfig": {
    "access": "public"
  },
  "description": "Data service contracts for Graph Horizon subgraph indexing",
  "author": "Edge & Node",
  "license": "GPL-2.0-or-later",
  "types": "typechain-types/index.ts",
  "exports": {
    "./artifacts/*": "./build/contracts/*",
    "./addresses*": "./addresses*"
  },
  "files": [
    "build/contracts/**/*",
    "typechain-types/**/*",
    "README.md"
  ],
  "scripts": {
    "lint": "pnpm lint:ts; pnpm lint:sol; pnpm lint:forge; pnpm lint:md; pnpm lint:json",
    "lint:ts": "eslint --fix --cache '**/*.{js,ts,cjs,mjs,jsx,tsx}'; prettier -w --cache --log-level warn '**/*.{js,ts,cjs,mjs,jsx,tsx}'",
    "lint:sol": "solhint --fix --noPrompt --noPoster 'contracts/**/*.sol'; prettier -w --cache --log-level warn '**/*.sol'",
    "lint:forge": "forge lint",
    "lint:md": "markdownlint --fix --ignore-path ../../.gitignore '**/*.md'; prettier -w --cache --log-level warn '**/*.md'",
    "lint:json": "prettier -w --cache --log-level warn '**/*.json'",
    "clean": "rm -rf build dist cache cache_forge typechain-types",
    "build": "pnpm build:dep && pnpm build:self",
    "build:dep": "pnpm --filter '@graphprotocol/subgraph-service^...' run build:self",
    "build:self": "hardhat compile --quiet",
    "test": "pnpm build && pnpm test:self",
    "test:self": "forge test",
    "test:deployment": "SECURE_ACCOUNTS_DISABLE_PROVIDER=true hardhat test test/deployment/*.ts",
    "test:integration": "./scripts/integration",
    "test:coverage": "pnpm build && pnpm test:coverage:self",
    "test:coverage:self": "mkdir -p coverage && forge coverage --report lcov --report-file coverage/lcov.info",
    "prepublishOnly": "pnpm run build"
  },
  "devDependencies": {
    "@graphprotocol/contracts": "workspace:^",
    "@graphprotocol/horizon": "workspace:^",
    "@graphprotocol/interfaces": "workspace:^",
    "@graphprotocol/toolshed": "workspace:^",
    "@nomicfoundation/hardhat-chai-matchers": "^2.0.0",
    "@nomicfoundation/hardhat-ethers": "catalog:",
    "@nomicfoundation/hardhat-foundry": "^1.1.1",
    "@nomicfoundation/hardhat-ignition": "^0.15.9",
    "@nomicfoundation/hardhat-ignition-ethers": "^0.15.9",
    "@nomicfoundation/hardhat-network-helpers": "^1.0.0",
    "@nomicfoundation/hardhat-toolbox": "^4.0.0",
    "@nomicfoundation/hardhat-verify": "^2.0.10",
    "@nomicfoundation/ignition-core": "^0.15.9",
    "@openzeppelin/contracts": "^5.0.2",
    "@openzeppelin/contracts-upgradeable": "^5.0.2",
    "@openzeppelin/foundry-upgrades": "0.4.0",
    "@tenderly/hardhat-tenderly": "^1.11.0",
    "@typechain/ethers-v6": "^0.5.0",
    "@typechain/hardhat": "^9.0.0",
    "@types/chai": "^4.2.0",
    "@types/mocha": ">=9.1.0",
    "@types/node": ">=16.0.0",
    "chai": "^4.2.0",
    "eslint": "catalog:",
    "ethers": "catalog:",
    "forge-std": "catalog:",
    "glob": "^11.0.1",
    "hardhat": "catalog:",
    "hardhat-contract-sizer": "^2.10.0",
    "hardhat-gas-reporter": "^1.0.8",
    "hardhat-graph-protocol": "workspace:^",
    "hardhat-secure-accounts": "^1.0.5",
    "json5": "^2.2.3",
    "lint-staged": "catalog:",
    "prettier": "catalog:",
    "prettier-plugin-solidity": "catalog:",
    "solhint": "catalog:",
    "solidity-coverage": "^0.8.0",
    "solidity-docgen": "^0.6.0-beta.36",
    "ts-node": ">=8.0.0",
    "typechain": "^8.3.0",
    "typescript": "catalog:"
  },
  "lint-staged": {
    "contracts/**/*.sol": [
      "pnpm lint:sol"
    ],
    "**/*.ts": [
      "pnpm lint:ts"
    ],
    "**/*.js": [
      "pnpm lint:ts"
    ],
    "**/*.json": [
      "pnpm lint:ts"
    ]
  }
}


### graphprotocol-contracts/packages/subgraph-service/remappings.txt

@openzeppelin/=node_modules/@openzeppelin/
@graphprotocol/=node_modules/@graphprotocol/
forge-std/=node_modules/forge-std/src/


### graphprotocol-contracts/packages/contracts/package.json

{
  "name": "@graphprotocol/contracts",
  "version": "7.3.0",
  "publishConfig": {
    "access": "public"
  },
  "description": "Contracts for the Graph Protocol",
  "main": "index.js",
  "repository": {
    "type": "git",
    "url": "git+https://github.com/graphprotocol/contracts",
    "directory": "packages/contracts"
  },
  "author": "Edge & Node",
  "license": "GPL-2.0-or-later",
  "bugs": {
    "url": "https://github.com/graphprotocol/contracts/issues"
  },
  "homepage": "https://github.com/graphprotocol/contracts#readme",
  "types": "index.d.ts",
  "scripts": {
    "prepack": "pnpm build",
    "clean": "rm -rf artifacts/ cache/ types/ abis/ build/ dist/ coverage/ test/node_modules/",
    "build": "pnpm build:self",
    "build:self": "pnpm compile",
    "compile": "hardhat compile --quiet",
    "test": "pnpm --filter @graphprotocol/contracts-tests test",
    "test:coverage": "pnpm --filter @graphprotocol/contracts-tests run test:coverage",
    "deploy": "pnpm predeploy && pnpm build",
    "deploy-localhost": "pnpm build",
    "predeploy": "scripts/predeploy",
    "lint": "pnpm lint:ts; pnpm lint:sol; pnpm lint:md; pnpm lint:json",
    "lint:ts": "eslint '**/*.{js,ts,cjs,mjs,jsx,tsx}' --fix --cache; prettier -w --cache --log-level warn '**/*.{js,ts,cjs,mjs,jsx,tsx}'",
    "lint:sol": "solhint --fix --noPrompt --noPoster 'contracts/**/*.sol'; prettier -w --cache --log-level warn 'contracts/**/*.sol'",
    "lint:md": "markdownlint --fix --ignore-path ../../.gitignore '**/*.md'; prettier -w --cache --log-level warn '**/*.md'",
    "lint:json": "prettier -w --cache --log-level warn '**/*.json'",
    "analyze": "scripts/analyze",
    "myth": "scripts/myth",
    "flatten": "scripts/flatten && scripts/clean",
    "typechain": "hardhat typechain",
    "verify": "hardhat verify",
    "size": "hardhat size-contracts"
  },
  "files": [
    "artifacts/**/*",
    "types/**/*",
    "contracts/**/*",
    "README.md",
    "addresses.json",
    "index.js",
    "index.d.ts"
  ],
  "devDependencies": {
    "@arbitrum/sdk": "~3.1.13",
    "@defi-wonderland/smock": "^2.4.1",
    "@ethersproject/abi": "^5.8.0",
    "@ethersproject/abstract-provider": "^5.8.0",
    "@ethersproject/abstract-signer": "^5.8.0",
    "@ethersproject/bytes": "^5.8.0",
    "@ethersproject/providers": "^5.8.0",
    "@graphprotocol/common-ts": "^1.8.3",
    "@graphprotocol/interfaces": "workspace:^",
    "@nomicfoundation/hardhat-network-helpers": "^1.0.0",
    "@nomicfoundation/hardhat-verify": "2.1.1",
    "@nomiclabs/hardhat-ethers": "^2.2.3",
    "@nomiclabs/hardhat-etherscan": "^3.1.0",
    "@nomiclabs/hardhat-waffle": "^2.0.6",
    "@openzeppelin/contracts": "3.4.2",
    "@openzeppelin/contracts-upgradeable": "3.4.2",
    "@openzeppelin/hardhat-upgrades": "^1.22.1",
    "@typechain/ethers-v5": "^10.2.1",
    "@typechain/hardhat": "^6.1.2",
    "@types/chai": "^4.2.0",
    "@types/mocha": ">=9.1.0",
    "@types/node": "^20.17.50",
    "@types/sinon-chai": "^3.2.12",
    "arbos-precompiles": "^1.0.2",
    "chai": "^4.2.0",
    "dotenv": "^16.5.0",
    "eslint": "catalog:",
    "ethereum-waffle": "^4.0.10",
    "ethers": "^5.7.2",
    "form-data": "^4.0.0",
    "glob": "catalog:",
    "graphql": "^16.11.0",
    "graphql-tag": "^2.12.4",
    "hardhat": "catalog:",
    "hardhat-abi-exporter": "^2.11.0",
    "hardhat-contract-sizer": "catalog:",
    "hardhat-gas-reporter": "catalog:",
    "hardhat-ignore-warnings": "catalog:",
    "hardhat-storage-layout": "catalog:",
    "prettier": "catalog:",
    "prettier-plugin-solidity": "catalog:",
    "solhint": "catalog:",
    "solidity-coverage": "^0.8.16",
    "ts-node": "^10.9.2",
    "typechain": "^8.3.2",
    "typescript": "catalog:",
    "winston": "^3.3.3",
    "yaml": "^1.10.2",
    "yargs": "^17.0.0"
  },
  "exports": {
    ".": {
      "types": "./index.d.ts",
      "default": "./index.js"
    },
    "./artifacts/*": "./artifacts/*",
    "./types": "./types/index.ts",
    "./types/*": "./types/*"
  }
}


### graphprotocol-contracts/packages/contracts/task/package.json

{
  "name": "@graphprotocol/contracts-task",
  "version": "1.0.3",
  "private": true,
  "description": "Task utilities for @graphprotocol/contracts",
  "author": "Edge & Node",
  "license": "GPL-2.0-or-later",
  "main": "src/index.ts",
  "types": "src/index.ts",
  "exports": {
    ".": {
      "default": "./src/index.ts",
      "types": "./src/index.ts"
    }
  },
  "scripts": {
    "build": "tsc --build",
    "clean": "rm -rf build types",
    "deploy": "hardhat migrate",
    "deploy-localhost": "hardhat migrate --force --skip-confirmation --disable-secure-accounts --network localhost --graph-config config/graph.localhost.yml --address-book addresses-local.json",
    "verify": "hardhat verify",
    "lint": "pnpm lint:ts; pnpm lint:json",
    "lint:ts": "eslint '**/*.{js,ts,cjs,mjs,jsx,tsx}' --fix --cache; prettier -w --cache --log-level warn '**/*.{js,ts,cjs,mjs,jsx,tsx}'",
    "lint:json": "prettier -w --cache --log-level warn '**/*.json'"
  },
  "dependencies": {
    "@graphprotocol/contracts": "workspace:^",
    "@graphprotocol/sdk": "0.6.0",
    "axios": "^1.9.0",
    "console-table-printer": "^2.14.1"
  },
  "devDependencies": {
    "@arbitrum/sdk": "~3.1.13",
    "@ethersproject/abi": "^5.8.0",
    "@ethersproject/abstract-provider": "^5.8.0",
    "@ethersproject/abstract-signer": "^5.8.0",
    "@ethersproject/bytes": "^5.8.0",
    "@ethersproject/providers": "^5.8.0",
    "@graphprotocol/common-ts": "^1.8.3",
    "@nomicfoundation/hardhat-network-helpers": "^1.0.0",
    "@nomiclabs/hardhat-ethers": "^2.2.3",
    "@nomiclabs/hardhat-etherscan": "^3.1.0",
    "@openzeppelin/contracts": "3.4.2",
    "@openzeppelin/contracts-upgradeable": "3.4.2",
    "@openzeppelin/hardhat-upgrades": "^1.22.1",
    "@typechain/ethers-v5": "^10.2.1",
    "@typechain/hardhat": "^6.1.2",
    "@types/glob": "^8.1.0",
    "@types/node": "^20.17.50",
    "arbos-precompiles": "^1.0.2",
    "dotenv": "^16.5.0",
    "eslint": "catalog:",
    "ethers": "^5.7.0",
    "form-data": "^4.0.0",
    "glob": "catalog:",
    "graphql": "^16.11.0",
    "graphql-tag": "^2.12.4",
    "hardhat": "catalog:",
    "hardhat-abi-exporter": "^2.11.0",
    "hardhat-contract-sizer": "^2.10.0",
    "hardhat-gas-reporter": "^1.0.8",
    "hardhat-storage-layout": "^0.1.7",
    "prettier": "catalog:",
    "prettier-plugin-solidity": "catalog:",
    "ts-node": "^10.9.2",
    "typechain": "^8.3.2",
    "typescript": "catalog:",
    "winston": "^3.3.3",
    "yaml": "^1.10.2",
    "yaml-lint": "catalog:",
    "yargs": "^17.0.0"
  }
}


### graphprotocol-contracts/packages/token-distribution/.env.sample

MNEMONIC=
ETHERSCAN_API_KEY=
INFURA_KEY=
STUDIO_API_KEY=


### graphprotocol-contracts/packages/token-distribution/package.json

{
  "name": "@graphprotocol/token-distribution",
  "version": "3.0.0",
  "private": true,
  "description": "Graph Token Distribution",
  "author": "Edge & Node",
  "license": "MIT",
  "main": "index.js",
  "scripts": {
    "prepublishOnly": "scripts/prepublish",
    "build": "pnpm build:dep && pnpm build:self",
    "build:dep": "pnpm --filter '@graphprotocol/token-distribution^...' run build:self",
    "build:self": "node scripts/build.js",
    "build:legacy": "scripts/build",
    "extract": "node scripts/extract-graphclient.js",
    "clean": "rm -rf build/ cache/ dist/ .graphclient/ reports/ types/",
    "clean:extracted": "rm -rf .graphclient-extracted/",
    "clean:all": "pnpm clean && pnpm clean:extracted",
    "compile": "hardhat compile --quiet",
    "deploy": "pnpm run build && hardhat deploy",
    "test": "pnpm build && pnpm test:self",
    "test:self": "scripts/test",
    "test:gas": "RUN_EVM=true REPORT_GAS=true scripts/test",
    "test:coverage:broken": "pnpm build && scripts/coverage",
    "lint": "pnpm lint:ts; pnpm lint:sol; pnpm lint:md; pnpm lint:json",
    "lint:ts": "eslint '**/*.{js,ts,cjs,mjs,jsx,tsx}' --fix --cache; prettier -w --cache --log-level warn --ignore-path ../../.prettierignore '**/*.{js,ts,cjs,mjs,jsx,tsx}'",
    "lint:sol": "solhint --fix --noPrompt --noPoster 'contracts/**/*.sol'; prettier -w --cache --log-level warn 'contracts/**/*.sol'",
    "lint:md": "markdownlint --fix --ignore-path ../../.gitignore '**/*.md'; prettier -w --cache --log-level warn '**/*.md'",
    "lint:json": "prettier -w --cache --log-level warn --ignore-path ../../.prettierignore '**/*.json'",
    "security": "scripts/security",
    "flatten": "scripts/flatten",
    "typechain": "hardhat typechain",
    "verify": "hardhat verify",
    "size": "hardhat size-contracts"
  },
  "files": [
    "dist/**/*",
    "README.md",
    "LICENSE"
  ],
  "devDependencies": {
    "@ethersproject/abi": "^5.7.0",
    "@ethersproject/bytes": "^5.7.0",
    "@ethersproject/experimental": "^5.0.7",
    "@ethersproject/hardware-wallets": "^5.7.0",
    "@ethersproject/providers": "^5.7.0",
    "@graphprotocol/client-cli": "^2.2.22",
    "@graphprotocol/contracts": "workspace:^",
    "@graphprotocol/interfaces": "workspace:^",
    "@graphql-yoga/plugin-persisted-operations": "^3.13.5",
    "@nomiclabs/hardhat-ethers": "^2.2.3",
    "@nomiclabs/hardhat-etherscan": "^3.1.0",
    "@nomiclabs/hardhat-waffle": "^2.0.6",
    "@openzeppelin/contracts": "3.4.2",
    "@openzeppelin/contracts-upgradeable": "3.4.2",
    "@openzeppelin/hardhat-upgrades": "^1.22.1",
    "@typechain/ethers-v5": "^10.2.1",
    "@typechain/hardhat": "^6.1.6",
    "@types/mocha": "^9.1.0",
    "@types/node": "^20.17.50",
    "@types/sinon-chai": "^3.2.12",
    "chai": "^4.2.0",
    "coingecko-api": "^1.0.10",
    "consola": "^2.15.0",
    "dotenv": "^16.0.0",
    "eslint": "catalog:",
    "ethereum-waffle": "^4.0.10",
    "ethers": "^5.7.2",
    "graphql": "^16.5.0",
    "graphql-yoga": "^5.13.4",
    "hardhat": "catalog:",
    "hardhat-abi-exporter": "^2.0.1",
    "hardhat-contract-sizer": "^2.0.1",
    "hardhat-deploy": "^0.7.0-beta.9",
    "hardhat-gas-reporter": "^1.0.1",
    "hardhat-ignore-warnings": "catalog:",
    "inquirer": "8.0.0",
    "lodash": "^4.17.21",
    "markdownlint-cli": "0.45.0",
    "p-queue": "^6.6.2",
    "prettier": "catalog:",
    "prettier-plugin-solidity": "catalog:",
    "solhint": "catalog:",
    "solidity-coverage": "^0.8.16",
    "ts-node": "^10.9.2",
    "typechain": "^8.3.0",
    "typescript": "catalog:",
    "typescript-eslint": "catalog:"
  },
  "dependencies": {
    "ajv": "^8.17.1"
  }
}


### graphprotocol-contracts/packages/horizon/foundry.toml

[profile.default]
src = 'contracts'
out = 'build'
libs = ["node_modules"]
test = 'test'
cache_path  = 'cache_forge'
fs_permissions = [{ access = "read", path = "./"}]
optimizer = true
optimizer_runs = 100

# Exclude test files from coverage reports
no_match_coverage = "(^test/|/mocks/)"

# Lint configuration
[lint]
ignore = ["contracts/mocks/imports.sol"]
exclude_lints = ["mixed-case-function", "mixed-case-variable"]


### graphprotocol-contracts/packages/horizon/package.json

{
  "name": "@graphprotocol/horizon",
  "version": "1.1.1",
  "publishConfig": {
    "access": "public"
  },
  "description": "Graph Horizon - Next generation Graph Protocol contracts",
  "author": "Edge & Node",
  "license": "GPL-2.0-or-later",
  "types": "typechain-types/index.ts",
  "exports": {
    "./artifacts/*": "./build/contracts/*",
    "./addresses*": "./addresses*",
    "./ignition": "./ignition/modules/index.ts",
    "./tasks/*": "./tasks/*"
  },
  "files": [
    "build/contracts/**/*",
    "typechain-types/**/*",
    "README.md"
  ],
  "scripts": {
    "lint": "pnpm lint:ts; pnpm lint:sol; pnpm lint:forge; pnpm lint:md; pnpm lint:json",
    "lint:ts": "eslint --fix --cache '**/*.{js,ts,cjs,mjs,jsx,tsx}'; prettier -w --cache --log-level warn '**/*.{js,ts,cjs,mjs,jsx,tsx}'",
    "lint:sol": "solhint --fix --noPrompt --noPoster 'contracts/**/*.sol'; prettier -w --cache --log-level warn '**/*.sol'",
    "lint:forge": "forge lint",
    "lint:md": "markdownlint --fix --ignore-path ../../.gitignore '**/*.md'; prettier -w --cache --log-level warn '**/*.md'",
    "lint:json": "prettier -w --cache --log-level warn '**/*.json'",
    "clean": "rm -rf build dist cache cache_forge typechain-types",
    "build": "pnpm build:dep && pnpm build:self",
    "build:dep": "pnpm --filter '@graphprotocol/horizon^...' run build:self",
    "build:self": "hardhat compile --quiet",
    "test": "pnpm build && pnpm test:self",
    "test:self": "forge test",
    "test:deployment": "SECURE_ACCOUNTS_DISABLE_PROVIDER=true hardhat test test/deployment/*.ts",
    "test:integration": "./scripts/integration",
    "test:coverage": "pnpm build && pnpm test:coverage:self",
    "test:coverage:self": "mkdir -p coverage && forge coverage --report lcov --report-file coverage/lcov.info",
    "prepublishOnly": "pnpm run build"
  },
  "devDependencies": {
    "@graphprotocol/contracts": "workspace:^",
    "@graphprotocol/interfaces": "workspace:^",
    "@graphprotocol/toolshed": "workspace:^",
    "@nomicfoundation/hardhat-chai-matchers": "^2.0.0",
    "@nomicfoundation/hardhat-ethers": "catalog:",
    "@nomicfoundation/hardhat-foundry": "^1.1.1",
    "@nomicfoundation/hardhat-ignition": "^0.15.9",
    "@nomicfoundation/hardhat-ignition-ethers": "^0.15.9",
    "@nomicfoundation/hardhat-network-helpers": "^1.0.0",
    "@nomicfoundation/hardhat-toolbox": "^4.0.0",
    "@nomicfoundation/hardhat-verify": "^2.1.1",
    "@nomicfoundation/ignition-core": "^0.15.9",
    "@openzeppelin/contracts": "^5.0.2",
    "@openzeppelin/contracts-upgradeable": "^5.0.2",
    "@openzeppelin/foundry-upgrades": "0.4.0",
    "@tenderly/hardhat-tenderly": "^1.11.0",
    "@typechain/ethers-v6": "^0.5.0",
    "@typechain/hardhat": "^9.0.0",
    "@types/chai": "^4.2.0",
    "@types/mocha": ">=9.1.0",
    "@types/node": ">=16.0.0",
    "chai": "^4.2.0",
    "eslint": "catalog:",
    "ethers": "catalog:",
    "forge-std": "catalog:",
    "glob": "^11.0.1",
    "hardhat": "catalog:",
    "hardhat-contract-sizer": "^2.10.0",
    "hardhat-gas-reporter": "^1.0.8",
    "hardhat-graph-protocol": "workspace:^",
    "hardhat-secure-accounts": "^1.0.5",
    "lint-staged": "catalog:",
    "prettier": "catalog:",
    "prettier-plugin-solidity": "catalog:",
    "solhint": "catalog:",
    "solidity-coverage": "^0.8.0",
    "ts-node": ">=8.0.0",
    "typechain": "^8.3.0",
    "typescript": "catalog:"
  },
  "lint-staged": {
    "contracts/**/*.sol": [
      "pnpm lint:sol"
    ],
    "**/*.ts": [
      "pnpm lint:ts"
    ],
    "**/*.js": [
      "pnpm lint:ts"
    ],
    "**/*.json": [
      "pnpm lint:ts"
    ]
  }
}


### graphprotocol-contracts/packages/horizon/remappings.txt

@openzeppelin/=node_modules/@openzeppelin/
@graphprotocol/=node_modules/@graphprotocol/
forge-std/=node_modules/forge-std/src/


### graphprotocol-contracts/packages/interfaces/dist/hardhat.config.js

"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
require("@nomicfoundation/hardhat-toolbox");
require("hardhat-ignore-warnings");
const config = {
    solidity: {
        compilers: [{ version: '0.8.27' }, { version: '0.7.6' }],
    },
    typechain: {
        outDir: 'types',
    },
    warnings: {
        'contracts/token-distribution/IGraphTokenLockWallet.sol': {
            default: 'off',
        },
        'contracts/toolshed/IGraphTokenLockWalletToolshed.sol': {
            default: 'off',
        },
    },
};
module.exports = config;
//# sourceMappingURL=hardhat.config.js.map

### graphprotocol-contracts/packages/interfaces/package.json

{
  "name": "@graphprotocol/interfaces",
  "version": "0.6.6",
  "publishConfig": {
    "access": "public"
  },
  "description": "Contract interfaces for The Graph protocol",
  "repository": {
    "type": "git",
    "url": "git+https://github.com/graphprotocol/contracts",
    "directory": "packages/interfaces"
  },
  "main": "./dist/src/index.js",
  "types": "./dist/src/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/src/index.d.ts",
      "default": "./dist/src/index.js"
    },
    "./types": {
      "types": "./dist/types/index.d.ts",
      "default": "./dist/types/index.js"
    },
    "./types-v5": {
      "types": "./dist/types-v5/index.d.ts",
      "default": "./dist/types-v5/index.js"
    },
    "./wagmi": {
      "types": "./dist/wagmi/generated.d.ts",
      "default": "./dist/wagmi/generated.js"
    },
    "./utils": {
      "types": "./dist/src/utils.d.ts",
      "default": "./dist/src/utils.js"
    },
    "./artifacts/*": "./artifacts/*",
    "./contracts/*": "./contracts/*"
  },
  "files": [
    "artifacts/**/*",
    "dist/**/*",
    "contracts/**/*",
    "types/**/*",
    "types-v5/**/*",
    "wagmi/**/*",
    "README.md"
  ],
  "author": "Edge & Node",
  "license": "GPL-2.0-or-later",
  "scripts": {
    "clean": "rm -rf dist dist-v5 cache artifacts types types-v5 wagmi",
    "lint": "pnpm lint:ts; pnpm lint:sol; pnpm lint:md; pnpm lint:json",
    "lint:ts": "eslint --fix --cache '**/*.{js,ts,cjs,mjs,jsx,tsx}'; prettier -w --cache --log-level warn '**/*.{js,ts,cjs,mjs,jsx,tsx}'",
    "lint:sol": "solhint --fix --noPrompt --noPoster 'contracts/**/*.sol'; prettier -w --cache --log-level warn 'contracts/**/*.sol'",
    "lint:md": "markdownlint --fix --ignore-path ../../.gitignore '**/*.md'; prettier -w --cache --log-level warn '**/*.md'",
    "lint:json": "prettier -w --cache --log-level warn '**/*.json'",
    "format": "prettier -w --cache --log-level warn '**/*.{js,ts,cjs,mjs,jsx,tsx,json,md,yaml,yml}'",
    "build": "pnpm build:self",
    "build:self": "scripts/build.sh",
    "build:clean": "pnpm clean && pnpm build",
    "watch": "tsc --watch",
    "prepublishOnly": "pnpm run build"
  },
  "devDependencies": {
    "@ethersproject/abi": "5.7.0",
    "@ethersproject/providers": "5.7.2",
    "@nomicfoundation/hardhat-ethers": "^3.0.0",
    "@nomicfoundation/hardhat-toolbox": "^4.0.0",
    "@nomicfoundation/hardhat-verify": "^2.0.0",
    "@openzeppelin/contracts": "3.4.2",
    "@openzeppelin/contracts-upgradeable": "3.4.2",
    "@typechain/ethers-v5": "^10.2.1",
    "@wagmi/cli": "^2.3.1",
    "ethers": "catalog:",
    "ethers-v5": "npm:ethers@5.7.2",
    "hardhat": "catalog:",
    "hardhat-ignore-warnings": "catalog:",
    "markdownlint-cli": "catalog:",
    "prettier": "catalog:",
    "prettier-plugin-solidity": "catalog:",
    "solhint": "catalog:",
    "ts-node": "catalog:",
    "typechain": "^8.3.2",
    "viem": "^2.31.7"
  }
}


