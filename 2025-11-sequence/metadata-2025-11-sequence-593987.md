
## PROTOCOL OVERVIEW:

## Trails Protocol – Detailed Technical Overview

*(~3 250 words)*

### 1. What is Trails?
Trails is a non-custodial transaction-orchestration layer that lets any wallet execute **one-click, cross-chain, cross-token flows**. A user expresses an *intent* (e.g. “mint NFT on Base using USDC I hold on Optimism”), signs **one confirmation**, and Trails automatically discovers the optimal bridge → swap → contract-call pipeline, routes the liquidity, pays gas (optionally in ERC-20s), and returns the final result. Under the hood Trails stitches together best-in-class bridges (CCTP, Relay, LiFi, …), DEXs, and relayers while inheriting the security and flexibility of the [Sequence v3] account-abstraction stack.

Key goals:
1. Remove chain & liquidity fragmentation (“Ethereum works as one”).
2. Preserve self-custody – user funds never touch Trails.
3. Be wallet-agnostic – works for EOAs, smart-wallets, ERC-7702 wallets, embedded in-app wallets, etc.
4. Provide an extensible, pluggable architecture so new bridges / DEXs can be added without redeploying integrators.

### 2. End-to-End Flow (from UX to chain)
1. **Define Intent** – dApp builds a JSON payload describing the desired outcome (token, amount, destination chain, calldata for a target contract…).
2. **Trails Inspects** – SDK checks balances, allowances, gas availability, and decides if orchestration is needed.
   • Same-chain & already funded? → Pass-through (single normal tx).
   • Otherwise → Continue orchestrating.
3. **Select Route** – Solver engine scores candidate routes across supported bridges / DEXs; UI shows cost, ETA, fees. User can override.
4. **Orchestrate Payment** – Trails turns the chosen route into a concrete execution plan: swap → bridge → execute.
5. **Execute Transaction** – User signs once; relayers post the required transactions on every chain; intents verify proofs & enforce atomicity.

Behind the scenes multiple contracts & chains can be touched, but the user signs exactly one bundle and receives a single UX confirmation.

### 3. Core On-chain Components
Trails uses a minimal set of **stateless** contracts that live at deterministic addresses (ERC-2470 factory). The contracts never retain ownership or upgradeability controls; their only purpose is validation & fund movement within the user’s own wallet-context.

#### 3.1 TrailsRouter (src/TrailsRouter.sol)
• Acts as the *brain* executed **inside the wallet** via `delegatecall` (Sequence delegated-extension module). 
• Provides helpers to
  – run a restricted **Multicall3.aggregate3Value** bundle atomically (no `allowFailure=true`).
  – pull assets from the calling user (`pullAndExecute`, `pullAmountAndExecute`).
  – inject dynamic amounts into calldata (gas-less ERC-20 approvals + placeholder-replacement) so a downstream DEX/bridge receives the exact balance.
  – sweep or refund residual balances once the operation succeeds (`sweep`, `refundAndSweep`).
• Enforces safety:
  – Multicall selector gated, disallows failures.
  – All sweeping / refund functions are `onlyDelegatecall`, meaning they *must* execute from the wallet context, never from the router contract directly.
  – Optional `validateOpHashAndSweep` checks a **success sentinel** (see 3.3) before sweeping to solver/relayer, ensuring they are paid only after successful completion.

#### 3.2 TrailsRouterShim (src/TrailsRouterShim.sol)
• A 65-line lightweight **shim** that forwards delegated extension calls to a fixed `TrailsRouter` address and writes a *success sentinel* for a given `opHash`.
• Immutable `ROUTER` is set in the constructor – no upgrade path → reduces supply-chain risk.
• Also inherits `DelegatecallGuard` so it can only run when embedded via delegatecall.

#### 3.3 TrailsSentinelLib (src/libraries/TrailsSentinelLib.sol)
• Pure library that deterministically maps `opHash` → storage slot (`keccak256("trails.sentinel"||opHash)`).
• A value of `1` in that slot = operation finished; it is read by `TrailsRouter` before sweeping funds to relayers.

#### 3.4 TrailsIntentEntrypoint (src/TrailsIntentEntrypoint.sol)
Handles **ERC-20 deposits** into a freshly computed *intent address* (a counterfactual Sequence account). Features:
• Supports EIP-2612 permit flow (`depositToIntentWithPermit`) or standard allowance (`depositToIntent`).
• Uses EIP-712 typed data to bind chainId, token, amount, fee, feeCollector, user-nonce. Intent is invalid after `deadline`.
• Maintains two anti-replay maps:
  – `nonces[user]` (monotonic).
  – `usedIntents[digest]` (one-shot per payload).
• Transfers the `amount` directly from the user to `intentAddress`; an optional `feeAmount` is transferred to `feeCollector`.
• No owner, no treasury, no lingering funds – token flows 100 % user → intent / feeCollector.

#### 3.5 DelegatecallGuard (src/guards/DelegatecallGuard.sol)
A 10-line utility inheritable contract that stores `_SELF` and reverts unless `address(this) != _SELF`, i.e. **must be delegatecalled**. Used by Router/Shim to guarantee they are running in wallet storage context.

### 4. Sequence v3 Integration & Intent Addresses
Sequence v3 AA wallets allow execution if the caller provides a **Merkle proof** that a given `(module, digest)` leaf exists in the wallet’s configuration tree. Trails exploits that by creating a **counterfactual wallet** where one leaf authorises the shim contract + a specific `opHash`. Until the user signs, the wallet doesn’t exist; once signed, relayers can deploy it at a deterministic `intentAddress` and feed proofs for both origin & destination chains. Because the leaf binds the calldata hash, relayers cannot tamper – they either post the exact bytes or the wallet rejects.

Result: even an EOA user gains AA superpowers (batched call, paying gas in ERC-20, cross-chain proofs) without upgrading their existing wallet.

### 5. Cross-Chain Atomicity & Relayers
1. Relayers listen for signed intents off-chain.
2. They fund gas on the **origin** chain and execute the router shim in the user’s wallet via Sequence delegated call.
3. The shim writes success sentinel for `opHash` and possibly emits events / messages to destination chains.
4. Other relayers pick up those events, execute counterpart transactions on **destination** chain(s).
5. Finally, after `validateOpHashAndSweep` passes, the solver’s fee is swept to them – guaranteeing they are only paid after end-to-end success.
Failure at any stage → wallet keeps funds, sentinel is not written, relayers can’t sweep.

### 6. Gas Abstraction – “Pay with Any Token”
Because the router runs in the wallet context and can pull arbitrary ERC-20s, Trails can subsidise the native gas using a relayer, then immediately reimburse itself in e.g. USDC via `refundAndSweep`. Optional **permit** saves an approval click. This solves the classic “bridged but no gas” problem for new chains and makes dApp tokens more useful.

### 7. Security Review
1. **Non-custodial** – contracts do not hold balances except transiently inside the user’s own wallet storage.
2. **Deterministic addresses** – deployed via ERC-2470 Factory, so identical bytecode on all chains. Reduces risk of supply-chain attack via different artifacts.
3. **No owners / upgraders** – all critical fields are `immutable`; no proxy pattern.
4. **Delegatecall guard** – prevents attackers from calling `sweep` directly on the router contract.
5. **Sentinel gating** – relayers only paid after success bit set, preventing DoS by malicious relayer.
6. **EIP-712 + nonces + usedIntents** – strong replay and signature validity checks for deposits.
7. **Audits** – built on audited Sequence v3 & third-party audits were conducted (links in docs).

### 8. Developer Integration Path
1. Import the JS/TS SDK.
2. Ask user for *intent* payload and signature (SDK handles EIP-712 typing & wallet prompts).
3. Submit the signed payload to Sequence API → receives proposed routes.
4. Present route(s) in UI; once user accepts, forward intent to relayer network (or run your own relayer).
5. Listen to events on origin chain (`IntentDeposit`, Sequence execution logs) for completion status.

No backend changes needed when Trails adds new bridges – it’s automatically surfaced through the solver layer.

### 9. Contract Reference Cheat-Sheet
| Contract | Key Responsibility | Requires delegatecall? | Holds funds? |
|----------|-------------------|------------------------|--------------|
| TrailsRouter | Execute multicall, pull/inject balances, sweep/refund | Yes (critical paths) | Only while inside wallet |
| TrailsRouterShim | Minimal dispatcher writing success sentinel | Yes | No |
| TrailsIntentEntrypoint | Validate EIP-712 & move ERC-20 from user to intent | No | No |
| TrailsSentinelLib | Pure library (slot calc) | N/A | No |
| DelegatecallGuard | Guard modifier | N/A | No |

### 10. Why the Design Matters
• **AA without waiting for ERC-4337 / 7702 adoption** – Trails delivers account-abstraction UX today for any EOA.
• **Solver competition** – Intents don’t bind to a single relayer; anyone can satisfy them, improving price & reliability.
• **Extensibility** – adding a new bridge is off-chain; on-chain code remains untouched because router just executes arbitrary calldata.
• **Cost efficiency** – Batching via Multicall3 + gas-token reimbursement minimises user cost.

### 11. Future Roadmap
1. Support for non-EVM chains (e.g. Solana, Cosmos) by adding new intent leaf types & cross-chain proof bridges.
2. Privacy features (eg. shielded intents, ZK proofs of solvency).
3. MEV-resistant order-flow auctions so solvers compete fairly without leaking routes.
4. Integrate paymaster-style sponsored gas so some flows become **truly gasless** from the user perspective.

---

### 12. Glossary
• **Intent** – declarative description of the desired outcome without prescribing how to achieve it.
• **Solver** – off-chain agent that bids to fulfil an intent using its own liquidity & gas.
• **Relayer** – entity that submits on-chain transactions, optionally fronting gas.
• **Sentinel** – storage slot toggled to record success; protects post-exec fund movement.
• **Shim** – small dispatcher used as Sequence module; forwards calls to router.

---

### 13. Conclusion
Trails marries the power of Sequence account-abstraction with a lean, stateless set of Solidity helpers to deliver seamless, one-click cross-chain UX. The contracts are small, immutable, and purpose-built: a guarded router that only runs *inside* the user’s wallet, a permit-aware ERC-20 deposit entrypoint, and a sentinel system to guarantee solver payments. Everything else – routing logic, bridge selection, gas sponsorship – lives off-chain in the SDK & solver network, giving the protocol flexibility to evolve without further on-chain migrations.

The result is an ecosystem-level “transaction bus” where any wallet, on any chain, can unlock complex flows with the simplicity of a single confirmation.


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


