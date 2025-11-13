
## PROTOCOL OVERVIEW:

## Trails Protocol – Detailed Technical Summary

### 1. What is Trails?
Trails is a cross-chain transaction orchestration layer that turns a multi-step, multi-chain operation (swaps, bridges, contract calls) into a single **intent** that a user approves once.  Under the hood it stitches together the user’s existing wallet (EOA or smart wallet), liquidity venues (DEXs, RFQ makers, bridges such as CCTPv2, LiFi, Relay) and a network of relayers/solvers.  The end result is a *1-click* experience where “Ethereum works as one”, meaning assets and dApps on different chains interact as if they were on the same chain.

Trails is built on top of Sequence v3 account-abstraction contracts.  Those contracts allow arbitrary call graphs to be authorised via Merkle proofs, which is the key building block that lets Trails collapse an entire cross-chain saga into a single transaction hash that the user signs.

### 2. High-Level Flow
```
User ➜ defines intent (pay X to contract Y on chain Z, in token T)
       ▼
Trails SDK ➜ queries balances, fees, bridges, DEX quotes
       ▼
Solver layer ➜ proposes optimal route (bridge A, swap B, …)
       ▼
Relayers ➜ submit Merkle proofs on origin + dest chains
       ▼
Intent contracts (Sequence) ➜ verify proofs & execute calls
       ▼
Final state reached (funds delivered / tx completed)
```
A single EIP-712 signature from the user authorises *only* the calldata encoded in the intent.  Relayers have zero latitude to modify parameters; they simply fund gas and surface proofs.

### 3. Contract Breakdown
1. **TrailsIntentEntrypoint.sol** – stateless verifier & deposit helper
   • Verifies off-chain signed intents (EIP-712) and prevents replay (usedIntents mapping + per-user nonces).  
   • Optionally calls `permit()` so users can pay gas & deposit with any EIP-2612 token.  
   • Transfers user funds (deposit + optional fee) directly to the *intent contract address* and fee collector; the entrypoint never escrows assets.

2. **TrailsRouter.sol** – execution & utility router
   • Wraps Multicall3 (aggregate3Value) with extra validation (disallows `allowFailure=true`) and value forwarding.  
   • Pulls funds from caller (`pullAndExecute`, `pullAmountAndExecute`) or injects its own balance into downstream calls (`injectAndCall`).  
   • Provides sweeping utilities that can only be used *via delegatecall* inside a Sequence wallet, guaranteeing they operate on the wallet’s storage/balance, not the Router contract itself.  
   • Emits sentinels into storage (via `TrailsSentinelLib`) so later steps can verify that a prior operation succeeded before sweeping leftovers.

3. **TrailsRouterShim.sol** – thin Sequence *Delegated Extension*
   • Stores immutable `ROUTER` address.  
   • Exposes `handleSequenceDelegateCall` which the Sequence kernel routes to when the intent tree demands it.  It simply decodes the forwarded payload, delegate-calls the Router and writes the SUCCESS sentinel.

4. **DelegatecallGuard.sol** – utility inherited by Router & Shim ensuring certain functions *must* be executed via `delegatecall`.  This prevents misuse on the implementation contract directly.

5. **TrailsSentinelLib.sol** – pure library that defines a namespace constant and the numeric `SUCCESS_VALUE` (`hex"0001"`).  Given an `opHash` it deterministically computes the storage slot where that op’s success flag must be stored (`keccak256(namespace || opHash)`).  In practice, the shim writes this flag and the Router checks it before sweeping.

6. **Interfaces** – `ITrailsIntentEntrypoint`, `ITrailsRouter`, `ITrailsRouterShim`, `IMulticall3` provide ABI surface for external tooling and for the Sequence kernel at runtime.

### 4. How Execution Works In Practice
1. **Intent Creation**: the dApp (via Trails SDK) builds a Merkle tree where each leaf is a `handleSequenceDelegateCall` to the RouterShim with calldata that eventually ends up executing `router.execute(aggregate3Value)` or one of the other helper functions.
2. **User Signature**: user signs the root hash (EIP-712 typed data).  The signed digest plus auxiliary params (amounts, deadlines, fee, etc.) is handed to `depositToIntent(WithPermit)`.
3. **Fund Deposit**: the entrypoint transfers funds to the *counterfactual* intent address (derived exactly like a Sequence wallet address would be).  At this moment the intent contract has enough balance to cover the first leg (gas + bridged token).
4. **Relayer Action**: off-chain relayers monitor mempool / API, see that the intent address is now funded, and submit the Merkle proofs on the origin chain.  Each proof eventually gets routed by the Sequence kernel into the Shim which calls the Router and sets the sentinel.
5. **On-Chain Calls**: the Router executes the validated multicall (swaps, bridge sends, etc.).  When done it performs `sweep()` so that any dust or bridged tokens are moved to the next bridge contract or to the destination address.
6. **Destination Chain**: another relayer repeats the process on the destination chain.  Finally the Router validates that the origin opHash succeeded (`validateOpHashAndSweep`), claiming the bridged funds and delivering them to the user / dApp contract.

The entire process either completes fully or reverts; because every leg records a sentinel, later legs can programmatically check and abort if a precursor failed.

### 5. Security Design
• **Non-custodial** – neither the entrypoint nor the Router/Shim ever hold user funds long-term.  Assets live in the intent (Sequence wallet) contract owned by the user’s key material.  
• **Delegate-only functions** – `onlyDelegatecall` ensures sweep/refund functions can *only* touch the caller’s storage context, not the Router contract itself.  
• **Replay protection** – intents are unique (`usedIntents` mapping) and per-user `nonce` increments.  
• **Selective authorisation** – the Merkle tree encodes exact calldata; relayers cannot replace parameters or routes.  Any mismatch causes on-chain reversion.  
• **Audited base layer** – built on Sequence v3 which has undergone multiple audits; Trails contracts are smaller extensions on top.

### 6. Gas & Token Flexibility
Because deposits are done through `depositToIntentWithPermit`, the protocol supports paying gas in *any* EIP-2612 token (USDC, USDT, etc.).  The Router’s injection helpers automatically approve & pass that same token down to bridges/DEXs.  This removes the common *no-native-gas* UX hurdle when landing on a new chain.

### 7. Upgrade / Deployment Model
All contracts are deployed as singletons via ERC-2470 factory, giving deterministic addresses across chains.  The Router/Shim are *stateless* (except immutable addresses) so they don’t need upgradeability proxies.  Any future upgrade would involve deploying a new version and referencing it in newly generated intents while old intents continue to use the previous binaries (immutable & safe).

### 8. How to Integrate as a dApp Developer
1. Import the Trails SDK (npm) and call `sdk.createIntent({...})` with your desired target chain, token, and calldata.  
2. Present the generated typed-data to the user for signing.  
3. Submit the signature + parameters to `TrailsIntentEntrypoint.depositToIntentWithPermit` (or `.depositToIntent`).  
4. Listen to the SDK’s `status()` stream or on-chain events; once the intent is settled you receive a final `TransactionCompleted` callback.

No bridging/DEX code is required on the dApp side; you describe *what* you want, not *how* to get there.

### 9. File Relationship Diagram
```
                   ┌────────────────────┐
                   │User Wallet / SDK   │
                   └────────┬───────────┘
                            │EIP-712 Sig
┌───────────────────────────▼────────────────────────────┐
│ TrailsIntentEntrypoint (deposit & verify)              │
└───────────────────────────┬────────────────────────────┘
                            │funds + calldata
             Counterfactual Intent (Sequence v3 wallet)
                            │delegatecalls
┌───────────────────────────▼────────────────────────────┐
│ TrailsRouterShim  (Sequence DelegatedExtension)        │
└───────────────────────────┬────────────────────────────┘
                            │delegatecall
┌───────────────────────────▼────────────────────────────┐
│ TrailsRouter (exec + sweep + inject)                   │
└──────────────────────────┬─────────────────────────────┘
                           │ERC20 transfers / native
                 External Protocols (DEX, Bridge, etc.)
```

### 10. Conclusion
Trails abstracts away the complexity of fragmented liquidity and disparate gas tokens by combining:
• Sequence v3 AA for powerful intent execution;
• A stateless Router/Shim pair for safe multicall orchestration;
• A deposit entrypoint that supports any EIP-2612 token;
• Off-chain solvers/relayers that supply gas and compete on best route.

The end-user gets a single confirmation, developers integrate a single endpoint, and under the hood an extensible, trust-minimised system achieves atomic cross-chain settlement.



## Main List of Files in Project

src/TrailsIntentEntrypoint.sol
src/TrailsRouter.sol
src/TrailsRouterShim.sol
src/guards/DelegatecallGuard.sol
src/interfaces/IMulticall3.sol
src/interfaces/ITrailsIntentEntrypoint.sol
src/interfaces/ITrailsRouter.sol
src/interfaces/ITrailsRouterShim.sol
src/libraries/TrailsSentinelLib.sol


 ## DOCUMENTATION: 

 ### sequence2-docs.md

# How Trails Works

> Core architecture and design of the Trails transaction rails protocol

Trails offers developers a solution to enable 1-click crypto experiences via any wallet, token and chain. Trails aims to make "Ethereum work as one" by solving the widespread chain and liquidity fragmentation problems by providing a multi-chain transaction orchestration layer on top of wallets that unifies all chains and automates token routing, all with a single end-user confirmation (1-click).

## Overview

Trails automatically determines the optimal path for cross-chain transaction execution, whether it's a simple single-chain transaction or a complex multi-step orchestration involving swaps, bridges, and executions across different blockchains.

Trails is built on the [Sequence](https://docs.sequence.xyz) v3 stack combining the powers of account abstraction, real-time indexing, and gasless transaction execution into an intent-powered chain abstraction solution. Trails is designed to work with any wallet, including a simple EOA wallet, a powerful smart wallet or an in-app embedded wallet (ie. MetaMask, Wallet Connect, Privy, Coinbase Smart Wallet, etc). For more details under-the-hood, please see the [protocol architecture](/architecture/protocol) section.

The protocol is designed to be extensible, with plans to support non-EVM chains in the future.

## Step-by-Step Flow

<Steps>
  <Step title="Define Intent">
    **Create a payment transaction for any chain in any token**

    Users begin by defining their payment intent - whether it's a mint, deposit, swap, or any other transaction type. This intent can be created for any supported blockchain using any supported token.
  </Step>

  <Step title="Trails Inspects">
    **Determines if orchestration is needed**

    Trails's intelligent orchestration engine analyzes the defined intent to determine the optimal execution path:

    * **Pass Through**: If the transaction can succeed without orchestration (same chain, sufficient balance, etc.), it passes through directly
    * **Continue Orchestrating**: If cross-chain operations or token swaps are needed, the system continues to the orchestration process

    <Note>
      This inspection happens in milliseconds and considers factors like:

      * Available token balances
      * Gas requirements
      * Chain compatibility
      * Protocol availability
    </Note>
  </Step>

  <Step title="Select Route">
    **Trails suggests optimal routing options**

    When orchestration is required, Trails analyzes available routes across chains and presents the optimal options:

    * **Automated Suggestions**: The system suggests the most efficient token and chain combinations
    * **User Customization**: Users can review and customize the suggested route
    * **Cost Analysis**: Each route option includes gas costs, fees, and estimated execution time

    <Tip>
      Users maintain full control over the routing decision while benefiting from Trails's optimization algorithms.
    </Tip>
  </Step>

  <Step title="Orchestrate Payment">
    **Intelligently combine operations for the selected route**

    Once the route is confirmed, Trails orchestrates the necessary operations:

    * **Swap Operations**: Convert tokens to the required format
    * **Bridge Transactions**: Move assets between chains when needed
    * **Execution Preparation**: Set up the final transaction parameters
  </Step>

  <Step title="Execute Transaction">
    **User confirms and completes the end-to-end flow**

    The final step puts control back in the user's hands:

    * **Wallet Confirmation**: User reviews and confirms the transaction through their connected wallet
    * **End-to-End Execution**: Trails executes the complete orchestrated flow
    * **Transaction Completion**: The system handles all intermediate steps and delivers the final result

    <Info>
      Users only need to sign once, but Trails may execute multiple transactions behind the scenes to complete the orchestrated payment.
    </Info>
  </Step>
</Steps>

## Key Benefits

<CardGroup cols={2}>
  <Card title="Simplified UX" icon="hand-sparkles">
    Users interact with a simple interface while Trails handles complex cross-chain orchestration
  </Card>

  <Card title="Optimal Routing" icon="route">
    Intelligent algorithms find the most efficient paths across chains and protocols
  </Card>

  <Card title="Cost Effective" icon="dollar-sign">
    Minimizes fees and gas costs through smart routing and batching
  </Card>

  <Card title="Secure" icon="shield-check">
    Non-custodial architecture ensures users maintain control of their assets
  </Card>
</CardGroup>

## Technical Architecture

The Trails protocol leverages several key components:

* **Intent Recognition**: Parses and understands user payment intentions
* **Route Optimization**: Calculates optimal paths across supported chains
* **Protocol Integration**: Interfaces with DEXs, bridges, and other protocols
* **Transaction Orchestration**: Coordinates complex multi-step operations
* **Execution Engine**: Handles the actual transaction execution and monitoring

## Integration with External Protocols & Liquidity Networks

* **Trails as Orchestrator (Glue Layer)**: Trails does not custody funds or act as a bridge/DEX itself. It orchestrates flows across existing liquidity networks and chains, acting as the glue that connects them so they work seamlessly as one.
* **Relayers & Cross-Chain Routers**: Trails composes best-in-class relayers and routers (e.g., Relay, LiFi, custom routers) to execute cross-chain legs when required.
* **Liquidity Providers (LPs) & DEXs**: Trails taps into AMMs, RFQ makers, and LPs to source liquidity and pricing, selecting the optimal venue per step of the route, providing the best price and gas efficiency possible for the end-user.
* **Pluggable Architecture**: New protocols, bridges, and networks can be added without changing integrator code. Trails evaluates options at runtime and selects the safest, most efficient path, giving the developer full control over the flow.

<Note>
  Trails is responsible for orchestrating the flow, not holding user assets. It coordinates protocols so users can transact with a single click (one transaction on the original chain), while assets move through underlying networks.
</Note>

## Security & Validation

* **Built on Sequence v3**: Trails is built on top of Sequence's audited v3 wallet contracts, leveraging mature account abstraction and battle-tested controls.
* **Intent-Scoped Authorization**: Intent addresses are only used to trustlessly authorize the specific transaction calldata the user has explicitly permitted—nothing more.
* **User “sudo” Control**: The user always retains control over their account and funds. In emergencies, they can take direct action at any time.
* **Validation & Refunds**: Calls are simulated and expected to succeed. If a step cannot complete, Trails handles refunds and unwind flows automatically where applicable.
* **Non-Custodial by Design**: Trails never takes custody; execution happens via the user's wallet permissions, preserving full control and auditability.

# Protocol Architecture

> Core architecture and design of the Trails protocol

## Trails end-to-end Flow

Trails enables seamless cross-chain transactions through a coordinated system of intent contracts, relayers, and solvers. Below is the complete flow from user initiation to transaction completion.

<a href="https://pub-dc89be64e317442b832bec22d3d12052.r2.dev/trails-flow.svg" target="_blank">(click here to view diagram in full screen)</a>

![Trails end-to-end flow](https://pub-dc89be64e317442b832bec22d3d12052.r2.dev/trails-flow.svg)

Additionally, you can think of Trails as a pluggable transaction adapter that can execute any arbitrary transactions across multiple chains. See below.

![Trails adapter](https://pub-dc89be64e317442b832bec22d3d12052.r2.dev/trails-adapter.svg)

## Under the Hood

Under the hood, Trails intents are counterfactual instantiations of [Sequence v3 account abstraction](https://github.com/0xsequence/wallet-contracts-v3) contracts. The Sequence v3 account abstraction contracts introduce a very flexible and novel execution model powered by merkle trees. The contract execution configuration is represented as a merkle tree that includes signers, a digest, a module, or a combination of all. The execution of the intent is triggered by a single transaction to the intent address, where the Trails relayers observe intent address and post merkle proofs onchain of the encoded sub-transactions on origin and destination chains. Please review the "Protocol Flow Overview" section above for more details.

TLDR: Trails successfully layers account abstraction transaction capabilities to primitive EOA wallets (even EOA wallets without ERC7702). This is the magic of the Trails design in how it can construct a single transaction with a single end-user confirmation that spans multiple chains and transactions. Trails also works with smart wallets directly, or ERC7702 wallets with some simplifications. The architectural goal of Trails is to always be able to construct a direct cross-chain/cross-token route for even the most common denominator of wallets, such as an EOA without ERC7702.

Trails is architected as a trustless system that works on top of existing bridging / filler / solver infrastructure. For bridging / filling, Trails currently has integrated CCTPv2, Relay and Lifi.

## Key Components

### 🏗️ Architecture Layers

1. **User Interaction Layer**: Handles user requests and wallet interactions
2. **SDK Layer**: Manages balance queries, token selection, and orchestration
3. **Solver Layer**: Provides optimal routing solutions for cross-chain transfers
4. **Chain Layer**: Coordinates relayer operations across multiple blockchains
5. **Execution Layer**: Executes intent contracts and completes transactions

### 🔄 Protocol Flow

The protocol follows a systematic approach:

1. **Initiation**: User initiates a cross-chain transaction through their app
2. **Discovery**: SDK queries available token balances across all supported chains
3. **Selection**: User selects preferred payment token from any available chain
4. **Solution**: Sequence API provides optimal bridge/swapsolution
5. **Execution**: Coordinated execution across origin and destination chains
6. **Completion**: Intent contracts fulfill the transaction requirements

### 🎯 Intent-Based Architecture

The protocol leverages intent contracts that:

* Define transaction requirements without specifying execution paths
* Enable flexible solver competition for optimal routes
* Provide atomicity guarantees across chain boundaries
* Support complex multi-hop operations seamlessly

### 🌉 Cross-Chain Coordination

Relayers on each chain work in concert to:

* Monitor intent contract states
* Execute transactions with cryptographic proofs
* Ensure atomic completion or rollback
* Maintain consistency across chain boundaries

### 📨 Role of the Relayer (How is this trustless?)

* Relayers cannot alter the transaction calldata or intent contents in any way; they can only submit the pre-committed, verifiable calls that match the intent's merkle commitments.
* Their responsibility is limited to subsidizing gas and successfully landing the transaction on the underlying chain on the user's behalf.
* Any deviation from the committed parameters is rejected on-chain by the intent contracts, ensuring the relayer cannot manipulate execution.

# Pricing

> Core architecture and design of the Trails transaction rails protocol

## Fee Model

The protocol is free to integrate for any app. The fee for Trails is paid by the end-user which includes:

* Gas fees on the origin and destination chains
* Included fees for liquidity providers or bridges
* Trails protocol fee which scales significantly down with the amount of volume (in USD) transacted

# Pay Gas with Any Token

> Users can pay gas in any permit-compatible, non-native token

## Pay Gas with non-native tokens

Trails is able to support paying gas in any permit-compatible (EIP-2612) token which includes popular options and many stablecoins such as USDC and USDT.

This prevents such scenarios when bridging to a new chain and not having any gas tokens to actually interact with apps. Additionally, Trails is able to bring more utility for your token seamlessly by enabling it as a end to end payment & gas option.

During the Trails flow, a user will be by default prompted with the native gas token, however can select other options. This will initially require an approval for the token, then every subsequent transaction will be a single confirmation.

# Trails Contracts

> Trails protocol smart contracts and security audits

## Overview

Trails is built on a robust set of smart contracts deployed across multiple chains. All contracts are deployed via ERC-2470 Singleton Factory for deterministic addresses, ensuring consistency across the ecosystem.

## Contracts Repository

The complete source code for all Trails smart contracts is available on GitHub:

### Key Contracts

The Trails protocol consists of several core smart contracts:

* **TrailsRouter** - Main routing contract for intent execution and token operations
* **TrailsIntentEntrypoint** - Entry point for intent deposits with permit support
* **TrailsRouterShim** - Execution wrapper with sentinel tracking for success/failure states

All contracts leverage Sequence v3 account abstraction for flexible, secure cross-chain operations.

## Security Audits

Security is a top priority for Trails. All smart contracts have undergone comprehensive third-party security audits:


For a technical overview, see the [Protocol Architecture](/architecture/protocol) documentation.




 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/evm-cctp-contracts/docs/package.json

{
  "name": "docs",
  "version": "1.0.0",
  "description": "CCTP Quickstart",
  "main": "index.js",
  "scripts": {
    "start": "node index.js"
  },

### lib/evm-cctp-contracts/package.json

{
  "name": "evm-cctp-contracts",
  "license": "Apache-2.0",
  "scripts": {
    "lint": "solhint '**/*.sol'"
  },
  "devDependencies": {
    "solhint": "^3.3.7"

### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.11.0",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/contracts/package.json

{
  "name": "lifi-contracts",
  "version": "0.1.0",
  "description": "LI.FI is a cross-chain bridge aggregation protocol",
  "engines": {
    "node": ">= 12.18.0"
  },
  "type": "module",

### lib/wallet-contracts-v3/package.json

{
  "devDependencies": {
    "lefthook": "^1.6.1",
    "prettier": "^3.2.5"
  },
  "scripts": {
    "postinstall": "lefthook install",
    "format:prettier": "prettier --write \"**/*.{js,jsx,ts,tsx,json,md,yml,yaml}\"",

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.4.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.4.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true,
  "dependencies": {
    "minimatch": "^3.1.2"
  }
}


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]

solc_version = "0.8.30"
via-ir = true
optimizer = true
optimizer_runs = 4294967295

[lint]
exclude_lints = ["unsafe-cheatcode"]


### remappings.txt

@/=src/
test/=test/

@openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/
forge-std/=lib/forge-std/src/
lifi-contracts/=lib/contracts/src/
erc2470-libs/=lib/erc2470-libs/
wallet-contracts-v3/=lib/wallet-contracts-v3/src/
tstorish/=lib/tstorish/src/


