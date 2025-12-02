
## PROTOCOL OVERVIEW:

## iTRY Tokenization & Staking Protocol – Technical Overview

### 1. Vision & High-level Design
The iTRY protocol tokenises a real-world Turkish Lira money-market fund (DLF) and brings it on-chain in three layers:
1. **DLF (Digital Liquidity Fund)** – ERC-20 that mirrors off-chain fund shares (audited elsewhere).
2. **iTRY** – a 1-for-1 fully-backed Turkish-Lira stablecoin minted by depositing DLF at its Net-Asset-Value (NAV).
3. **wiTRY** – an ERC-4626 vault that stakes iTRY; holders automatically accrue daily yield produced by the underlying MMF without inflating the share price.

The system is multi-chain from day-one using LayerZero v2:
• Ethereum Mainnet acts as the **hub** where real collateral lives; adapters lock native iTRY/wiTRY and issue OFT representations on spoke chains (e.g. MegaETH).
• On spokes, canonical OFT contracts mint/burn to mirror the hub supply and enforce the same compliance rules (KYC whitelist & blacklist).

### 2. Core Solidity Components
| Layer | Contract | Purpose |
|-------|----------|---------|
| **Core** | **iTryIssuer.sol** | Mints & redeems iTRY against DLF, routes collateral to the FastAccessVault buffer, charges configurable basis-point fees, mints daily yield, manages whitelist & oracle. |
|         | **FastAccessVault.sol** | Holds a target % of total DLF so redemptions can settle instantly.  When buffer > target it remits excess to an off-chain custodian, when < target it emits a TopUpNeeded event.  Only the Issuer may withdraw. |
| **Tokens** | **iTry.sol** | Upgradeable ERC-20 with role based minting, 3-state transfer gate (disabled / whitelist-only / fully open) and blacklist/whitelist lists. |
|           | **StakediTry(.sol, Cooldown, FastRedeem, Crosschain)** | Layered ERC-4626 implementation: base vault + optional cooldown module (3 days) + optional fast-redeem path (fee to treasury) + extra functions that authorised "Composer" contracts use when interacting from other chains. |
|           | **iTrySilo.sol** | Custody contract that holds iTRY withdrawn during an unstake cooldown so main vault accounting stays clean. Only the vault may pull funds back out. |
| **Cross-chain** | **iTryTokenOFTAdapter / wiTryOFTAdapter** | Hub-side wrappers that lock native tokens and speak LayerZero OFT. |
|                 | **iTryTokenOFT / wiTryOFT** | Spoke-side OFTs that mint/burn under control of the LayerZero messaging library while reproducing blacklist/whitelist logic. |
|                 | **VaultComposerSync / wiTryVaultComposer** | Generic & wiTRY-specific LayerZero "Composers" that deposit/redeem in the vault and then bridge resulting tokens in one atomic message. wiTryComposer overrides redemption to start a cooldown instead of immediate withdraw. |
|                 | **UnstakeMessenger.sol** | Lets a spoke-chain user finish their cooldown by pinging the hub and paying the hub→spoke gas in a single UX flow. |
| **Periphery** | **YieldForwarder.sol** | Receives freshly minted iTRY (protocol yield) and forwards it to a treasury when anyone calls `processNewYield()`. |
|               | **RedstoneNAVFeed.sol** | Simple test oracle implementing `IOracle` – production deployment will be replaced by a secured Redstone price feed. |
| **Shared libs** | `SingleAdminAccessControl`, `SigUtils`, `CommonErrors` etc. | Gas-optimised utilities and one-admin wrapper around OZ AccessControl preventing accidental loss of the sole admin key.

### 3. Mint, Yield & Redemption Flow
1. **Mint iTRY**
   a. User must possess `WHITELISTED_USER_ROLE` (KYC).
   b. User calls `mintITRY(dlf, minOut)` on Issuer.
   c. Issuer pulls DLF from user, sends it to FastAccessVault (fee portion → Treasury), queries NAV oracle and mints iTRY net of fee.
   d. `_totalIssuedITry` and `_totalDLFUnderCustody` book-keeping is updated.

2. **Daily Yield-capture**
   • When NAV increases the off-chain fund is worth more; Issuer compares NAV×DLF under custody with `totalIssued`.  The delta is minted as NEW iTRY and sent to `YieldForwarder`, which in turn forwards (immediately or on demand) to a distribution treasury.  wiTRY holders benefit because Rewarder feeds those tokens into the vault via `transferInRewards()` which linearly vests over a configurable period.

3. **Redemption**
   • User calls `redeemITRY(iTry, minDLF)`.
   • Issuer burns caller’s iTRY and tries to pull DLF from FastAccessVault.  If vault has enough liquidity, transfer is immediate (same-tx).  If not, Issuer emits `DelayedRedemptionRequested` and off-chain custodian delivers DLF later; user is notified off-chain.

4. **Staking / Unstaking**
   • Deposit: user sends iTRY to `StakediTry` (or composer on spoke) and receives wiTRY shares.
   • Unstake with cooldown: user (or composer) calls `cooldownAssets/Shares`, which withdraws assets into the Silo and records `cooldownEnds`.  After ≥3 days they ping `unstake()` (hub) or `unstakeThroughComposer()` (spoke) to pull iTRY back.
   • **Fast-redeem**: if enabled, user may bypass cooldown via `fastRedeem/Withdraw`; vault burns shares, charges fee (0.01–20 %) to treasury, returns iTRY instantly.  wiTryVaultComposer exposes the same on spoke chains.

### 4. Compliance & Governance
• **Single-admin model** – every contract that inherits `SingleAdminAccessControl` is guaranteed to have exactly one DEFAULT_ADMIN at any time, rotated through a two-step `transferAdmin/acceptAdmin` process that is safer than OZ’s direct transfer.
• **Blacklist / Whitelist** – implemented in both hub tokens and spoke OFTs; transfers, mint, burn and unstake paths call `_beforeTokenTransfer` to enforce.  Admin can also "confiscate" balances of fully black-listed addresses via `redistributeLockedAmount()`.
• **Fee Parameters** – mint-fee and redemption-fee are capped constants (½ % & 0.5 % by default) and only changeable by DEFAULT_ADMIN.  Fast-redeem fee range is clamped (1–2000 bps) to prevent griefing.
• **Reentrancy Guard** – critical external flows (`mint`, `redeem`, vault deposit/withdraw, composers, YieldForwarder, rescue functions) carry `nonReentrant`.

### 5. Cross-chain Security Considerations
1. **Lock/Mint pattern** – hub adapters lock canonical tokens; spoke OFTs can only mint when a proven LayerZero message arrives.
2. **Trusted Remote** – all OApps store a `peers` mapping keyed by endpoint-IDs; messages from unknown peers revert.
3. **Composer atomicity** – `lzCompose()` refunds tokens to the sender on any failure, avoiding stuck funds.
4. **Return-trip gas** – Unstake flow asks the user to pre-pay hub→spoke gas; helper functions `quoteUnstakeWithBuffer()` and dynamic refund logic minimise UX friction while ensuring the hub can always answer.

### 6. Dependency & Library Versions
• Solidity 0.8.20 (via-ir optimised, Cancun EVM target)
• OpenZeppelin 4.9.2 (upgradeable + core)
• LayerZero-v2 packages 2.0.2
• Forge-std 1.10.0 for testing
• Redstone Oracles 0.9.x (for production NAV feed)

### 7. Upgradeability & Extensibility
iTRY token is **UUPS-upgradeable**; all other contracts are immutable/proxy-less for simplicity.  The design leaves future upgrade hooks:
• Replace mock oracle with Redstone price feed.
• Automate daily yield minting (currently manual Exec role).
• Add more collateral types (issuer already parameterised).
• Deploy additional spoke chains by deploying OFT contracts and registering peers.

### 8. Risk & Trust Model Summary
1. **Centralised admin & custodian risk** – Admin controls mint/burn & whitelist; custodian must honestly hold the off-chain DLF and honour delayed redemptions.
2. **Oracle risk** – Incorrect NAV price directly affects mint/redemption parity; governance can swap the oracle address.
3. **Bridge risk** – LayerZero endpoints must remain honest and liveness is required for cross-chain fungibility, but collateral is always on the hub chain so compromise cannot inflate supply.
4. **Contract risk** – Code is forked from audited Ethena & LayerZero repos with minimal business-logic deltas; new components (Issuer, Vault, Composer) undergo audit in this scope.

### 9. Gas & Economic Optimisations
• `CommonErrors` & custom errors instead of revert strings.
• `SingleAdminAccessControl` removes dynamic admin arrays.
• Low-level `transfer` checks aggregated in helper library.
• All cross-chain send functions allow fee quotes so front-ends can accurately fund native gas and avoid over-provision.

### 10. Conclusion
The iTRY protocol brings a regulated, yield-bearing Turkish-Lira stablecoin on-chain with:
• Fully collateralised backing (DLF shares).
• Modular staking vault with cooldown & fast-redeem.
• Liquidity buffer for low-latency redemptions.
• Multi-chain reach via LayerZero v2 while central collateral remains safe on Ethereum.
• Strong, single-admin based compliance controls suited for RWA issuance.
Everything is built on audited primitives (Ethena, LayerZero, OpenZeppelin) with concise, well-isolated custom logic—facilitating both security review and future feature additions.


## Main List of Files in Project

src/protocol/FastAccessVault.sol
src/protocol/YieldForwarder.sol
src/protocol/iTryIssuer.sol
src/token/iTRY/crosschain/iTryTokenOFT.sol
src/token/iTRY/crosschain/iTryTokenOFTAdapter.sol
src/token/iTRY/iTry.sol
src/token/wiTRY/StakediTry.sol
src/token/wiTRY/StakediTryCooldown.sol
src/token/wiTRY/StakediTryCrosschain.sol
src/token/wiTRY/StakediTryFastRedeem.sol
src/token/wiTRY/crosschain/UnstakeMessenger.sol
src/token/wiTRY/crosschain/wiTryOFT.sol
src/token/wiTRY/crosschain/wiTryOFTAdapter.sol
src/token/wiTRY/crosschain/wiTryVaultComposer.sol
src/token/wiTRY/iTrySilo.sol


 ## DOCUMENTATION: 

 ### brix-docs.md

iTRY Contracts Audit Scope Overview
Introduction
This document provides an overview of the audit scope for the iTRY tokenization protocol. The protocol enables the minting and redemption of iTRY tokens (and their staked counterpart wiTRY), which are backed by Digital Liquidity Fund (DLF) tokens representing shares of a traditional fund investing in Turkish Money Market Funds (MMF). The protocol includes cross-chain functionality via LayerZero integration.

Project Overview
System Overview
The iTRY protocol creates a Turkish Lira stablecoin ecosystem backed by real-world money market fund assets. The system consists of three interconnected tokens:
DLF (Digital Liquidity Fund): Tokenized shares of a fund that invests in TRY (Turkish Lira)-denominated Money Market funds. The DLF token implementation is audited separately and out of scope for the current audit.
iTRY: A Turkish Lira stablecoin backed 1:1 by the NAV (Net Asset Value) of DLF tokens. Users supply DLF to mint iTRY based on the current Net Asset Value of the underlying money market fund.
wiTRY: The yield-bearing "staked" version of iTRY. By staking iTRY into wiTRY, holders receive yield generated by the underlying money market funds.

Core Mechanics
Minting Flow: Users deposit DLF tokens to mint iTRY at the current NAV rate. The deposited DLF is routed through a Fast Access Vault (acting as a liquidity buffer pool) before being sent to a custodian. The minted iTRY can then be staked to receive wiTRY.
Yield Distribution: At the end of each market day, the NAV price is updated to reflect the money market fund's earnings. Since this value increase affects all DLF tokens equally, the issuer mints new iTRY corresponding to the DLF tokens under custody—ensuring the circulating iTRY supply always matches the custodied DLF value. The newly minted iTRY is distributed to the staking contract and selected partners according to holding percentages and business agreements. This process will be manual initially and automated in future iterations.
Redemption Flow: When redeeming iTRY for DLF, the Fast Access Vault provides immediate liquidity when available. If vault liquidity is insufficient, the system emits an event for custodian-managed redemption.
Unstaking: Redeeming wiTRY for iTRY requires a 3-day cooldown period. Users can bypass this cooldown using a "fastWithdraw" option that charges an additional fee.
Cross-chain Architecture: Both iTRY and wiTRY are multichain tokens, initially deployed on Mainnet and MegaETH. On Mainnet (hub chain), the system uses OFT Adapters to wrap the native tokens. On spoke chains, standard OFT token implementations mirror the compliance features of the hub chain. A Composer contract manages cross-chain deposits and withdrawals into/from the vault.

Forks from Audited Codebases
The smart contracts are based on previously audited and battle-tested implementations:

iTRY Token: Forked from Ethena's USDtb contract
wiTRY Token: Forked from Ethena's StakedUSDeV2 ERC4626 vault
LayerZero Integration: Based on LayerZero's official OFT implementation, including Composer contracts
These contracts have been adapted primarily through naming/comment changes and Solidity version updates, along with selective feature additions to meet specific protocol requirements. This approach leverages the security assurances of the original audited codebases while customizing functionality for this use case. Diff analyses can be found below.

Protocol Architecture Overview
The iTRY protocol consists of three main components:

Token Layer: iTRY (ERC20) and wiTRY (ERC4626 staked vault)
Cross-chain Layer: LayerZero integration for hub and spoke chain deployments
Core Protocol Layer: Issuance, redemption, and liquidity management contracts

Component Purposes and Functionality
Token Contracts
iTRY Token Contract: An ERC20 token with comprehensive access controls, forked from Ethena's USDtb. The contract implements blacklisting and whitelisting capabilities through configurable transfer states. It includes role-based minting and burning functions.
wiTRY Token Contract: An ERC4626-compliant staking vault for iTRY tokens, forked from Ethena's StakedUSDeV2. It implements a cooldown period on unstake, under which a "fast-track" unstaking is possible for a fee. It also includes functions specifically implemented to allow for cross-chain deposits and withdrawals.

LayerZero Integration
Hub Chain (Mainnet):
iTRY OFT Adapter: Wraps the iTRY token for cross-chain transfers using LayerZero's OFT adapter pattern.
wiTRY OFT Adapter: Wraps the wiTRY token for cross-chain transfers using LayerZero's OFT adapter pattern.

Spoke Chains (MegaETH and future chains):
iTRY OFT: Standard OFT token implementation that mirrors mainnet iTRY compliance features (whitelist/blacklist)
wiTRY OFT: Standard OFT token implementation that mirrors mainnet wiTRY compliance features (whitelist/blacklist)
UnstakeMessenger: Utility contract to handle the crosschain unstaking of assets that have already finished cooldown.

Composer Contract: Forked from LayerZero's official Composer implementation with modified withdrawal mechanism. Instead of immediate withdrawal on _redeemAndSend, it triggers the start of a cooldown, requiring users to manually withdraw after the cooldown period. Also allows calling the fastWithdrawal() functions. It modifies the standard handleCompose() function to make it virtual.

Core Protocol Contracts
iTRY Issuer Contract: Manages the minting and redemption of iTRY tokens in exchange for DLF tokens. Queries oracle for exchange rates, mints corresponding iTRY amounts, and routes DLF to the FastAccessVault. On redemption, attempts to withdraw from FastAccessVault; if insufficient liquidity, emits an event for custodian-managed redemption. Implements whitelist-only minting and redemptions. Also responsible for minting new iTRY during daily yield distribution to maintain parity between circulating iTRY and custodied DLF value.
FastAccessVault Contract: Holds DLF tokens to service redemption requests. Maintains a target percentage of total DLF under custody (with configurable minimum). Includes rebalance function that transfers excess funds to custodian wallet or requests top-up when below target threshold. Acts as a liquidity buffer to speed up redemptions without requiring direct custodian interaction.

Periphery Contracts
Oracle Contract (Out of scope / Still under development): Implements price feed integration for NAV (Net Asset Value) pricing. Exposes a price() function queryable by the iTRY Issuer for exchange rate determination.
YieldForwarder Contract: Receives yield generated by the protocol and forwards it to a designated treasury address on request.

Key Features
Role-based access control with emergency functions
Blacklist/whitelist enforcement for compliance
Cross-chain token bridging via LayerZero OFT standard
Oracle-based dynamic pricing for minting/redemption (updated daily at market close)
Liquidity management with automatic rebalancing
3-day cooldown mechanism for unstaking with fast withdrawal option
Support for multiple collateral types
ERC4626-compliant staking vault

Modifications from Original Codebases
iTRY Token Modifications (based on diff against Ethena's USDtb):
Solidity Version: Updated from 0.8.26 to 0.8.20
Naming Changes: Rebranded from USDtb to iTRY throughout contract
Variable Naming: Updated variable names to reflect iTRY terminology
wiTRY Token Modifications (based on diff against Ethena's stakedUSDE):
Solidity Version: Updated from 0.8.19 to 0.8.20
Naming Changes: Rebranded from stakedUSDE to wiTRY throughout contract
Variable Naming: Updated variable names to reflect wiTRY terminology
Modfiable Reward Vesting Period: Added functions to make the rewards vesting period mutable.
LayerZero Integration Modifications:
OFT Adapters: Standard LayerZero OFT adapter implementation with no additional modifications beyond configuration
Spoke Chain OFTs: Implemented compliance features (whitelist/blacklist) mirroring hub chain token contracts

Composer Contract:
Modified Withdrawal: Overridden _redeemAndSend functionality to trigger cooldown instead of immediate withdrawal
Cooldown System: Added 3-day cooldown period requiring manual withdrawal after delay
Fast Withdrawal: Added the option to call the new fast withdrawal functions (with fee)



 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.10.0",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/openzeppelin-contracts-upgradeable/contracts/package.json

{
  "name": "@openzeppelin/contracts-upgradeable",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.2",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts-upgradeable/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.2",
  "files": [
    "/contracts/**/*.sol",
    "/build/contracts/*.json",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-foundry-upgrades/package.json

{
  "name": "@openzeppelin/foundry-upgrades",
  "version": "0.4.0",
  "description": "Foundry library for deploying and managing upgradeable contracts",
  "license": "MIT",
  "files": [
    "src/**/*"
  ],

### lib/openzeppelin-foundry-upgrades/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true
}

### lib/solidity-bytes-utils/package.json

{
  "name": "solidity-bytes-utils",
  "version": "0.8.2",
  "description": "Solidity bytes tightly packed arrays utility library.",
  "main": "truffle.js",
  "repository": {
    "type": "git",
    "url": "git@github.com:GNSPS/solidity-bytes-utils.git"

### lib/LayerZero-v2/package.json

{
  "name": "@layerzerolabs/layerzero-v2",
  "version": "2.0.2",
  "private": true,
  "workspaces": [
    "packages/**"
  ],
  "scripts": {

### lib/LayerZero-v2/packages/layerzero-v2/sui/contracts/package.json

{
  "name": "@layerzerolabs/layerzero-v2-sui",
  "private": true,
  "license": "LZBL-1.2",
  "scripts": {
    "build": "./build_and_test.mjs compile \"$@\" && $npm_execpath postBuild",
    "postBuild": "find . \\( -path \"*/debug_info/package.json\" -o -path \"*/debug_info/*/package.json\" \\) -type f -delete",
    "test": "./build_and_test.mjs test \"$@\" && $npm_execpath postBuild"

### lib/LayerZero-v2/packages/layerzero-v2/solana/programs/package.json

{
  "name": "@layerzerolabs/layerzero-v2-solana",
  "private": true,
  "license": "BUSL-1.1",
  "files": [
    "target/deploy",
    "target/idl"
  ],

### lib/LayerZero-v2/packages/layerzero-v2/ton/package.json

{
  "name": "@layerzerolabs/layerzero-v2-ton",
  "private": true,
  "license": "LZBL-1.2",
  "scripts": {
    "build": "$npm_execpath clean-prebuild && $npm_execpath blueprint build --all",
    "clean-prebuild": "rimraf target",
    "test": "$npm_execpath jest --verbose"

### lib/LayerZero-v2/packages/layerzero-v2/evm/oapp/package.json

{
  "name": "@layerzerolabs/lz-evm-oapp-v2",
  "private": true,
  "license": "MIT",
  "scripts": {
    "clean": "rimraf cache out",
    "build": "forge build",
    "test": "forge test"

### lib/LayerZero-v2/packages/layerzero-v2/evm/protocol/package.json

{
  "name": "@layerzerolabs/lz-evm-protocol-v2",
  "private": true,
  "license": "LZBL-1.2",
  "scripts": {
    "clean": "rimraf cache out",
    "build": "forge build",
    "test": "forge test"

### lib/LayerZero-v2/packages/layerzero-v2/evm/messagelib/package.json

{
  "name": "@layerzerolabs/lz-evm-messagelib-v2",
  "private": true,
  "license": "LZBL-1.2",
  "scripts": {
    "clean": "rimraf cache out",
    "build": "forge build",
    "test": "forge test"

### lib/redstone-oracles-monorepo/package.json

{
  "name": "@redstone-finance/oracles-monorepo",
  "version": "0.0.2",
  "description": "",
  "scripts": {
    "global:turbo": "cd $INIT_CWD && turbo",
    "build:all": "yarn turbo build",
    "typecheck:all": "yarn turbo typecheck",

### lib/redstone-oracles-monorepo/packages/casper-connector/package.json

{
  "name": "@redstone-finance/casper-connector",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/on-chain-relayer/package.json

{
  "name": "@redstone-finance/on-chain-relayer",
  "version": "0.9.0",
  "types": "dist/src/index.d.ts",
  "main": "dist/src/index.js",
  "exports": {
    ".": "./dist/src/index.js",
    "./src/run-relayer": {

### lib/redstone-oracles-monorepo/packages/evm-adapters/package.json

{
  "name": "@redstone-finance/evm-adapters",
  "version": "0.9.0",
  "types": "dist/src/index.d.ts",
  "main": "dist/src/index.js",
  "exports": {
    ".": "./dist/src/index.js",
    "./artifacts/*": "./artifacts/*",

### lib/redstone-oracles-monorepo/packages/stylus-connector/package.json

{
  "name": "@redstone-finance/stylus-connector",
  "private": true,
  "version": "0.9.0",
  "description": "A tool to inject RedStone data into Stylus compatible smart contracts",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",

### lib/redstone-oracles-monorepo/packages/fuel-connector/package.json

{
  "name": "@redstone-finance/fuel-connector",
  "version": "0.9.0",
  "description": "A tool to inject RedStone data into Fuel compatible smart contracts",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",

### lib/redstone-oracles-monorepo/packages/multichain-kit/package.json

{
  "name": "@redstone-finance/multichain-kit",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/starknet-connector/package.json

{
  "name": "@redstone-finance/starknet-connector",
  "version": "0.9.0",
  "description": "A tool to inject RedStone data into Starknet compatible smart contracts",
  "license": "MIT",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",

### lib/redstone-oracles-monorepo/packages/healthcheck/package.json

{
  "name": "@redstone-finance/healthcheck",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/gelato-relayer/package.json

{
  "name": "@redstone-finance/gelato-relayer",
  "version": "0.9.0",
  "description": "RedStone Relayer based on Gelato Web3 Functions",
  "repository": "https://github.com/redstone-finance/redstone-oracles-monorepo",
  "scripts": {
    "clean": "yarn global:ts-hardhat-clean",
    "build": " yarn global:tsc -p tsconfig.build.json",

### lib/redstone-oracles-monorepo/packages/agents/package.json

{
  "name": "@redstone-finance/agents",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/utils/package.json

{
  "name": "@redstone-finance/utils",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/remote-config/package.json

{
  "name": "@redstone-finance/remote-config",
  "version": "0.8.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/stellar-connector/package.json

{
  "name": "@redstone-finance/stellar-connector",
  "version": "0.9.0",
  "description": "A tool to inject RedStone data into Stellar compatible smart contracts",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",

### lib/redstone-oracles-monorepo/packages/protocol/package.json

{
  "name": "@redstone-finance/protocol",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/erc7412/package.json

{
  "name": "@redstone-finance/erc7412",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/mqtt5-client/package.json

{
  "name": "@redstone-finance/mqtt5-client",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/internal-utils/package.json

{
  "name": "@redstone-finance/internal-utils",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/sdk/package.json

{
  "name": "@redstone-finance/sdk",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/radix-connector/package.json

{
  "name": "@redstone-finance/radix-connector",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/chain-agnostic-oracle-tests/package.json

{
  "name": "@redstone-finance/chain-agnostic-oracle-tests",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/evm-multicall/package.json

{
  "name": "@redstone-finance/evm-multicall",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/canton-connector/package.json

{
  "name": "@redstone-finance/canton-connector",
  "version": "0.9.0",
  "private": true,
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",

### lib/redstone-oracles-monorepo/packages/rpc-providers/package.json

{
  "name": "@redstone-finance/rpc-providers",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/http-client/package.json

{
  "name": "@redstone-finance/http-client",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/sui-connector/package.json

{
  "name": "@redstone-finance/sui-connector",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "scripts": {
    "clean": "yarn global:ts-clean",
    "build": "yarn global:tsc -p tsconfig.build.json",

### lib/redstone-oracles-monorepo/packages/ton-connector/package.json

{
  "name": "@redstone-finance/ton-connector",
  "description": "A tool to inject RedStone data into TON compatible smart contracts",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",

### lib/redstone-oracles-monorepo/packages/eth-contracts/package.json

{
  "name": "@redstone-finance/eth-contracts",
  "version": "0.9.0",
  "description": "RedStone ethereum contracts",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "private": true,
  "license": "MIT",

### lib/redstone-oracles-monorepo/packages/solana-connector/package.json

{
  "name": "@redstone-finance/solana-connector",
  "version": "0.9.0",
  "description": "A tool to inject RedStone data into Solana compatible smart contracts",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",

### lib/redstone-oracles-monorepo/packages/cache-service/package.json

{
  "name": "@redstone-finance/cache-service",
  "version": "0.9.0",
  "main": "dist/src/main.js",
  "exports": "./dist/src/main.js",
  "types": "dist/src/main.d.ts",
  "description": "",
  "author": "",

### lib/redstone-oracles-monorepo/packages/chain-configs/package.json

{
  "name": "@redstone-finance/chain-configs",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/relayer-remote-config/package.json

{
  "name": "@redstone-finance/relayer-remote-config",
  "version": "0.9.0",
  "description": "Configuration files for the relayers module",
  "scripts": {
    "lint": "yarn global:prettier --check .",
    "lint:fix": "yarn global:prettier --write ."
  },

### lib/redstone-oracles-monorepo/packages/move-connector/package.json

{
  "name": "@redstone-finance/move-connector",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "scripts": {
    "clean": "yarn global:ts-clean",
    "build": "yarn global:tsc -p tsconfig.build.json",

### lib/redstone-oracles-monorepo/packages/chain-orchestrator/package.json

{
  "name": "@redstone-finance/chain-orchestrator",
  "version": "0.9.0",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "license": "MIT",
  "scripts": {

### lib/redstone-oracles-monorepo/packages/on-chain-relayer-common/package.json

{
  "name": "@redstone-finance/on-chain-relayer-common",
  "description": "This package cannot contain any node.js specific imports or dependencies",
  "version": "0.9.0",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "exports": "./dist/index.js",
  "license": "MIT",

### lib/redstone-oracles-monorepo/packages/evm-connector/package.json

{
  "name": "@redstone-finance/evm-connector",
  "version": "0.9.0",
  "description": "A tool to inject RedStone data into EVM compatible smart contracts",
  "main": "dist/src/index.js",
  "exports": "./dist/src/index.js",
  "license": "MIT",
  "scripts": {

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.2",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.2",
  "files": [
    "/contracts/**/*.sol",
    "/build/contracts/*.json",
    "!/contracts/mocks/**/*"


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]

# Deterministic deployment settings for CREATE2
solc = "0.8.20"  # Pin exact Solidity version to ensure consistent bytecode
evm_version = "cancun"  # Target EVM version
bytecode_hash = "none"  # Exclude metadata hash from bytecode for deterministic addresses
cbor_metadata = false  # Disable CBOR metadata encoding
always_use_create_2_factory = true  # Always use standard CREATE2 deployer (0x4e59b44847b379578588920cA78FbF26c0B4956C)

# Optimizer settings
optimizer = true
optimizer_runs = 200
via_ir = true
ffi = true  # Needed for LayerZero tests
sparse_mode = true

# File system permissions for deployment scripts
fs_permissions = [{ access = "read-write", path = "./broadcast" }]


# Explicit cache settings for consistent incremental builds
cache = true
cache_path = "cache"
force = false  # Don't force recompile everything

[lint]
exclude_lints = [
 "screaming-snake-case-immutable",
 "mixed-case-variable",
 "mixed-case-function",
 "unused-import",
 "unaliased-plain-import"
]

# Fast profile for development - no via_ir, no scripts
[profile.fast]
src = "src"
out = "out"
libs = ["lib"]
optimizer = true
optimizer_runs = 200
via_ir = false
sparse_mode = true

# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options


### remappings.txt

@oz-upgradeable/=lib/openzeppelin-contracts-upgradeable/contracts/
@oz/=lib/openzeppelin-contracts/contracts/
@openzeppelin/contracts-upgradeable/=lib/openzeppelin-contracts-upgradeable/contracts/
@openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/
@layerzerolabs/lz-evm-oapp-v2/contracts/=lib/LayerZero-v2/packages/layerzero-v2/evm/oapp/contracts/
@layerzerolabs/lz-evm-protocol-v2/contracts/=lib/LayerZero-v2/packages/layerzero-v2/evm/protocol/contracts/
@layerzerolabs/lz-evm-messagelib-v2/contracts/=lib/LayerZero-v2/packages/layerzero-v2/evm/messagelib/contracts/
@redstone-finance/evm-connector/=lib/redstone-oracles-monorepo/packages/evm-connector/
solidity-bytes-utils/=lib/solidity-bytes-utils/
forge-std/=lib/forge-std/src/


