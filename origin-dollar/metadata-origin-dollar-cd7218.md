
 ------------ ## PROTOCOL OVERVIEW ------------ 

# Origin Dollar Protocol Summary

Origin Dollar, as described by the provided context, is a multi-product yield protocol centered on tokenized vault assets, strategy-based capital deployment, and governed upgradeable smart contracts. Its Solidity surface includes yield-bearing tokens such as `OUSD`, `OETH`, `OSonic`, and wrapped variants like `WOETH` and `WOSonic`; vault systems for asset custody and accounting; strategy modules that deploy capital into external venues; dripper contracts for paced yield distribution; and governance/proxy infrastructure for administration and upgrades.

## What The Protocol Is

At a high level, the protocol appears to issue user-facing assets whose value is backed by assets held in vaults and deployed into strategies. The vaults act as the canonical accounting layer, while strategies are the execution layer that attempts to earn yield from external protocols such as Curve, Morpho, ERC-4626-style vaults, native staking systems, and cross-chain bridging/messaging infrastructure.

The provided Immunefi context makes clear that the most important in-scope production focus is the **Curve OETH+WETH AMO Strategy**, though the repository scope is broader and includes OUSD, OETH, Sonic, Base, staking, and cross-chain components.

## Main Components

### 1. Tokens

The token layer includes:

- `OUSD`
- `OETH`
- `OETHBase`
- `OETHPlume`
- `OSonic`
- `WOETH`
- `WOSonic`

These appear to represent the protocol’s user-facing assets across different environments and wrapping models. The wrapped variants are especially important because they likely convert between a rebasing representation and a non-rebasing share-style representation. That means protocol safety depends not just on total asset value, but also on correct conversion math between underlying balances and wrapped shares.

### 2. Vaults

The vault system is split into multiple contracts:

- `VaultCore`
- `VaultAdmin`
- `VaultStorage`
- `OETHVault`
- `OETHBaseVault`
- `OUSDVault`
- `OSVault`

From the context, vaults are the center of truth for deposits, redemptions, asset accounting, strategy allocation, and solvency. The split between core/admin/storage suggests a modular architecture where user logic, admin controls, and state layout are separated. Asset-specific vaults imply product specialization while still following a common vault pattern.

The vault is the most important economic component because it is where total assets, token liabilities, strategy balances, and realized losses must reconcile correctly.

### 3. Strategies

The strategy layer includes:

- `CurveAMOStrategy`
- `Generalized4626Strategy`
- `MorphoV2Strategy`
- `BridgedWOETHStrategy`
- `CrossChainMasterStrategy`
- `CrossChainRemoteStrategy`
- `SonicStakingStrategy`
- `CompoundingStakingSSVStrategy`
- `CompoundingStakingView`

These strategies likely receive capital from vaults, interact with external systems, and later return principal plus yield. Each strategy represents a distinct risk domain:

- **Curve AMO**: pool accounting, slippage, LP valuation, gauge/reward flow
- **Morpho V2**: lending/borrowing or vault accounting correctness
- **ERC-4626 generalized strategy**: share-price manipulation and preview/redeem mismatch risk
- **Cross-chain strategies**: in-flight accounting, replay protection, remote trust assumptions
- **Bridged asset strategies**: wrapped/bridged token consistency and bridge state correctness
- **Staking strategies**: validator, reward, withdrawal-delay, and slashing exposure

### 4. Governance And Roles

The control plane includes:

- `Governable`
- `Strategizable`
- vault admin paths
- timelock/governor assets listed in the bounty context

This suggests privileged roles can manage strategies, configure system behavior, harvest rewards, perform upgrades, and potentially move capital. These are core trust boundaries. The protocol’s security model depends heavily on whether these roles are constrained by timelocks, multisigs, or limited permissions.

### 5. Upgradeability And Deployment

The scope contains multiple proxy contracts:

- `InitializeGovernedUpgradeabilityProxy`
- `InitializeGovernedUpgradeabilityProxy2`
- `BaseProxies`
- `Proxies`
- `SonicProxies`
- `CrossChainStrategyProxy`

This indicates the protocol is upgradeable and likely deployed across multiple chains with product-specific deployment patterns. The presence of initializer-based governance proxies means deployment correctness, initializer one-time safety, storage layout continuity, and admin authority are fundamental to system integrity.

### 6. Yield Distribution

The protocol includes:

- `Dripper`
- `FixedRateDripper`

These contracts likely smooth the release of harvested rewards over time instead of applying them immediately. That makes them part of the value-distribution pipeline rather than the value-generation pipeline. They matter because accounting may differ between assets already earned and assets already recognized in user balances.

## How The Protocol Works

## User And Vault Flow

The context implies the following base lifecycle:

1. A user or authorized actor interacts with a protocol token or vault.
2. Assets are deposited into a vault or redeemed from it.
3. The vault tracks total asset value and corresponding liabilities.
4. The vault allocates some assets into one or more strategies.
5. Strategies deploy those assets into external systems to earn yield.
6. Yield, rewards, or recovered funds return to the vault directly or indirectly.
7. Some rewards may be buffered and streamed through dripper contracts.
8. Token balances, rebases, or wrapper exchange rates reflect the new accounting state.

Even without line-by-line code, the architecture clearly depends on vault accounting remaining coherent as funds move between idle balances, deployed strategies, claimable rewards, wrapped token forms, and possibly remote chains.

## OETH / OUSD / Sonic Product Model

The repository suggests the protocol is not a single-token system. Instead, it appears to operate multiple product lines:

- **OUSD-related** vault and strategies
- **OETH-related** vault and AMO / wrapped token infrastructure
- **Sonic-related** vault, token, wrapper, staking, dripper, and proxy infrastructure
- **Base / cross-chain extensions** for bridged or remote assets

That implies a shared architectural pattern repeated across chains or assets, with each deployment adapting the core vault-strategy-token model to its target environment.

## Curve OETH+WETH AMO Flow

The most emphasized strategy in the documentation is the **Curve OETH+WETH AMO Strategy**. Based on the scoped files and context, its lifecycle likely involves some combination of:

1. Receiving vault-managed OETH and/or WETH.
2. Adding liquidity to a Curve pool.
3. Possibly staking LP positions in a Curve gauge.
4. Accruing trading fees and/or incentive rewards.
5. Harvesting rewards through Curve/Minter/Merkl-like integrations.
6. Reporting its current asset value back to the vault.
7. Withdrawing liquidity when the vault needs funds or wants to rebalance.

This strategy is especially sensitive because AMO logic often affects both yield generation and peg maintenance. If LP positions are overvalued, manipulable, or not readily withdrawable at the reported price, the vault can appear solvent when it is not. If withdrawals or reward paths are mis-sequenced, users may bear hidden losses or the system may misprice OETH exposure.

## ERC-4626 / Share-Based Flow

The `Generalized4626Strategy` and vault interface files indicate support for external share-based vaults. In such systems, the local strategy likely deposits an asset into an external vault and receives shares in return. The local protocol must then translate those external shares back into recoverable asset value when reporting to the vault.

This is a classic accounting boundary. The strategy must handle:

- deposit-to-share conversion
- redeem-to-asset conversion
- possible donations affecting share price
- possible liquidity shortfalls on withdrawal
- differences between preview functions and actual execution

If those assumptions fail, the local vault can mint or redeem against bad pricing.

## Morpho V2 Flow

The presence of `MorphoV2Strategy`, `IMorphoV2Adapter`, and `IVaultV2` indicates integration with Morpho-style vault infrastructure. Conceptually, the vault allocates capital to the Morpho strategy, which then interacts with the external Morpho system and later reports balance or withdrawable value back.

This creates similar accounting risks to ERC-4626 integrations, but with additional dependency on adapter logic, protocol-specific redemption rules, and possibly asynchronous or utilization-sensitive liquidity.

## Cross-Chain Flow

The cross-chain surface includes:

- `AbstractCCTPIntegrator`
- `CrossChainMasterStrategy`
- `CrossChainRemoteStrategy`
- `CrossChainStrategyProxy`
- `ICCTP`
- `BridgedWOETHStrategy`

This strongly suggests a model where one chain acts as a controlling or canonical side and another chain hosts a remote execution strategy. Funds may be bridged, messages may be transmitted, and remote positions may be tracked from a master strategy.

A likely conceptual flow is:

1. A source-chain vault allocates funds to a master strategy.
2. The master strategy initiates a bridge or message flow via CCTP-like infrastructure.
3. A remote strategy on another chain receives funds or instructions.
4. The remote strategy deploys capital into local opportunities.
5. Asset balances and accounting must still be reflected correctly on the source side.
6. When funds are recalled, another message/bridge path returns value.

This design introduces a major distinction between **economic ownership** and **immediate possession**. Funds in transit, delayed, failed, or pending finality may still be counted somewhere. That means the protocol must define how in-flight assets affect total assets, redeemability, and solvency.

## Staking Flow

The staking-related contracts suggest a strategy family that deploys capital into native staking or SSV infrastructure. These systems usually include delayed withdrawals, reward accrual outside immediate token balances, and slashing risk. In protocol terms, the vault likely treats the staking strategy as yield-generating capital, while the strategy must convert validator/reward state into a recoverable asset estimate.

That estimate is inherently more judgmental than holding an idle ERC-20 balance. The protocol therefore depends on careful handling of pending rewards, inactive validators, queued exits, and potentially slashed balances.

## Dripper Flow

The dripper contracts likely buffer rewards and release them gradually. Functionally, that means there are at least two layers of value recognition:

- value already received by the protocol
- value already distributed into the user-facing accounting path

A fixed-rate dripper implies governance or an operator can set how quickly excess rewards become visible. This can be operationally useful, but it creates a need for precise accounting around undistributed balances, elapsed time, sweep permissions, and rate changes.

## External Integrations

The documented interfaces show the protocol touches multiple third-party systems:

- Curve StableSwap NG
- Curve gauges and minter
- Merkl reward distribution
- Morpho V2
- ERC-4626-like vaults
- CCTP bridging/messaging
- staking and validator infrastructure

This means the protocol is not only a vault issuer, but also an allocator and wrapper around external state machines. In practice, that makes its safety depend on two things:

1. Its own internal accounting discipline.
2. How conservatively it models the external systems it integrates with.

## Core Security And Economic Model

From the context, the main invariant the protocol needs to preserve is straightforward:

**user-facing liabilities must not exceed recoverable protocol assets.**

Everything else is a refinement of that rule. The main supporting invariants likely include:

- vault total assets should track real recoverable value
- strategy-reported balances should not exceed what can actually be withdrawn or realized
- wrapped/share conversions should preserve proportional value
- losses should be recognized instead of hidden
- cross-chain funds in transit should be treated consistently
- reward dripping should not double count or leak value
- proxy upgrades should preserve state and authority assumptions

## Why This Protocol Is Complex

This is not a simple single-vault ERC-20 project. The provided scope shows several overlapping complexity layers:

- multiple product lines (`OUSD`, `OETH`, `OSonic`)
- multiple chains (Ethereum, Base, Sonic)
- upgradeable proxies
- wrapped and rebasing token representations
- active market operations through Curve AMOs
- generalized external vault strategies
- cross-chain master/remote strategy architecture
- delayed yield-release mechanics via drippers

That combination means the protocol’s main challenge is consistent valuation and authority control across many different execution environments.

## Audit-Relevant Risk Concentrations

Based on the provided context, the highest-risk areas are:

- **Vault accounting**: whether total assets and liabilities remain coherent.
- **Curve AMO valuation**: whether LP positions and rewards are counted safely.
- **Strategy reporting**: whether reported balances reflect recoverable value rather than optimistic nominal value.
- **Wrapped token math**: whether share conversions match economic reality.
- **Cross-chain accounting**: whether bridged and in-flight funds are treated consistently.
- **Governance and strategist privileges**: whether trusted operations can alter critical state safely.
- **Upgradeability**: whether initializer/admin/storage assumptions can break the system.
- **Dripper recognition timing**: whether undistributed rewards are accounted for exactly once.

## Overall Interpretation

From the material provided, Origin Dollar is best understood as a governed, upgradeable yield protocol that issues protocol-branded assets backed by vault-managed capital. Vaults hold and account for assets, strategies deploy them into external systems, drippers smooth reward realization, wrappers adapt token behavior for different use cases, and governance controls upgrades and operational parameters. The system spans multiple chains and products, with the Curve OETH+WETH AMO strategy being a particularly important focus because it directly combines liquidity management, external pool accounting, and protocol solvency assumptions.

The protocol’s correctness therefore depends less on any single formula and more on whether all of these modules preserve a shared economic truth: every issued token or wrapped claim must remain backed by conservatively valued, recoverable assets across vaults, strategies, rewards, and cross-chain positions.


 ------------ ## Main List of Files in Project ------------ 

contracts/contracts/beacon/BeaconProofs.sol
contracts/contracts/governance/Governable.sol
contracts/contracts/governance/Strategizable.sol
contracts/contracts/harvest/Dripper.sol
contracts/contracts/harvest/FixedRateDripper.sol
contracts/contracts/interfaces/IBasicToken.sol
contracts/contracts/interfaces/ICurveLiquidityGaugeV6.sol
contracts/contracts/interfaces/ICurveMinter.sol
contracts/contracts/interfaces/ICurveStableSwapNG.sol
contracts/contracts/interfaces/IMerkl.sol
contracts/contracts/interfaces/IStrategy.sol
contracts/contracts/interfaces/IVault.sol
contracts/contracts/interfaces/cctp/ICCTP.sol
contracts/contracts/interfaces/morpho/IMorphoV2Adapter.sol
contracts/contracts/interfaces/morpho/IVaultV2.sol
contracts/contracts/proxies/BaseProxies.sol
contracts/contracts/proxies/InitializeGovernedUpgradeabilityProxy.sol
contracts/contracts/proxies/InitializeGovernedUpgradeabilityProxy2.sol
contracts/contracts/proxies/Proxies.sol
contracts/contracts/proxies/SonicProxies.sol
contracts/contracts/proxies/create2/CrossChainStrategyProxy.sol
contracts/contracts/strategies/BridgedWOETHStrategy.sol
contracts/contracts/strategies/CurveAMOStrategy.sol
contracts/contracts/strategies/Generalized4626Strategy.sol
contracts/contracts/strategies/MorphoV2Strategy.sol
contracts/contracts/strategies/NativeStaking/CompoundingStakingSSVStrategy.sol
contracts/contracts/strategies/NativeStaking/CompoundingStakingView.sol
contracts/contracts/strategies/crosschain/AbstractCCTPIntegrator.sol
contracts/contracts/strategies/crosschain/CrossChainMasterStrategy.sol
contracts/contracts/strategies/crosschain/CrossChainRemoteStrategy.sol
contracts/contracts/strategies/sonic/SonicStakingStrategy.sol
contracts/contracts/token/OETH.sol
contracts/contracts/token/OETHBase.sol
contracts/contracts/token/OETHPlume.sol
contracts/contracts/token/OSonic.sol
contracts/contracts/token/OUSD.sol
contracts/contracts/token/WOETH.sol
contracts/contracts/token/WOSonic.sol
contracts/contracts/utils/Helpers.sol
contracts/contracts/utils/Initializable.sol
contracts/contracts/utils/StableMath.sol
contracts/contracts/vault/OETHBaseVault.sol
contracts/contracts/vault/OETHVault.sol
contracts/contracts/vault/OSVault.sol
contracts/contracts/vault/OUSDVault.sol
contracts/contracts/vault/VaultAdmin.sol
contracts/contracts/vault/VaultCore.sol
contracts/contracts/vault/VaultStorage.sol
contracts/lib/rooster/openzeppelin-custom/contracts/utils/math/Math.sol
contracts/lib/rooster/openzeppelin-custom/contracts/utils/math/SafeCast.sol
contracts/lib/rooster/v2-common/libraries/Math.sol


 ------------ ## DOCUMENTATION: ------------ 

 ### origin-dollar-docs.md

# Origin Dollar Audit Context

## Source Boundaries

This document is derived only from the provided context bundle for repository `origin-dollar` at commit `cd7218c2b070a52470b2621c3ce0ce12378ba700` and the Immunefi Origin Protocol bounty entry/resources excerpts. Web & App assets, impacts, resources, and findings are intentionally excluded.

The Immunefi program overview identifies the smart-contract bounty focus as the **Curve OETH+WETH (AMO) Strategy**. No additional protocol documentation links were present in the provided Immunefi Resources excerpt.

## Protocol Overview

Origin Protocol’s in-scope smart-contract surface centers on Origin yield-bearing token and vault contracts, strategy contracts, governance/upgradeability support, and external integration interfaces. The bounty overview specifically names the **Curve OETH+WETH AMO Strategy**, indicating that the audit should pay close attention to how OETH/WETH liquidity and accounting are handled through Curve-related strategy code.

The in-scope repository paths include:

- Token contracts: `OETH`, `OETHBase`, `OETHPlume`, `OSonic`, `OUSD`, `WOETH`, `WOSonic`.
- Vault contracts: `VaultCore`, `VaultAdmin`, `VaultStorage`, `OETHVault`, `OETHBaseVault`, `OUSDVault`, `OSVault`.
- Strategy contracts: `CurveAMOStrategy`, `Generalized4626Strategy`, `MorphoV2Strategy`, `BridgedWOETHStrategy`, `CrossChainMasterStrategy`, `CrossChainRemoteStrategy`, `SonicStakingStrategy`, and native staking SSV strategy/view contracts.
- Harvest/dripping contracts: `Dripper`, `FixedRateDripper`.
- Governance and role boundaries: `Governable`, `Strategizable`.
- Proxy/upgradeability contracts: `InitializeGovernedUpgradeabilityProxy`, `InitializeGovernedUpgradeabilityProxy2`, base/protocol proxy files, Sonic proxies, and a CREATE2 cross-chain strategy proxy.
- External integration interfaces: Curve gauge/minter/stableswap, Merkl, CCTP, Morpho, ERC-4626-like vaults, and generic strategy/vault interfaces.

## Architecture

The scoped system is organized around vaults, tokens, strategies, governance controls, and upgradeable proxies.

Vault contracts are the central accounting layer. Based on the scoped files, vault logic is split across core execution, admin controls, storage, and asset-specific vault implementations for OETH, OUSD, and OSonic. Security review should treat vault accounting as the canonical source for deposits, withdrawals, strategy allocation, asset valuation, and any user-facing mint/redeem behavior.

Token contracts define the user-facing assets and wrappers. `WOETH` and `WOSonic` indicate wrapped representations of rebasing or yield-bearing assets. Review should distinguish between underlying token balances, wrapped share balances, and any conversion math between them.

Strategy contracts are the capital deployment layer. They appear to connect vault-held assets to external protocols or cross-chain destinations. The bounty overview singles out the Curve OETH+WETH AMO Strategy, while the broader scope also includes Morpho, ERC-4626, CCTP cross-chain, bridged WOETH, Sonic staking, and SSV-native staking strategy code.

Governance and strategist contracts define privileged control planes. `Governable` and `Strategizable` should be reviewed as trust boundaries for who can upgrade contracts, configure strategies, move funds, harvest rewards, pause flows, or alter accounting assumptions.

Proxy contracts indicate upgradeable deployment patterns. Review should include initializer safety, storage layout compatibility, admin authority, implementation switching, and any differences between proxy variants.

## Main Flows

### Vault Asset Flow

The likely primary flow is:

1. Users or privileged actors interact with vault/token contracts.
2. Vault contracts account for supported assets and liabilities.
3. Vaults allocate capital to strategies through strategy interfaces.
4. Strategies interact with external protocols or cross-chain systems.
5. Harvested rewards or yield return to the vault or are distributed over time through dripper contracts.
6. Token supply or wrapper exchange rates reflect the resulting accounting state.

The exact mechanics must be verified in code; the provided documentation does not specify deposit, mint, redeem, withdrawal, or rebase formulas.

### Curve OETH+WETH AMO Flow

The Immunefi program overview identifies **Curve OETH+WETH (AMO) Strategy** as the program focus. The scoped files include `CurveAMOStrategy` plus Curve interfaces for liquidity gauge, minter, and StableSwap NG.

Security-relevant review areas:

- How OETH and WETH balances are valued before and after Curve pool interactions.
- Whether pool share accounting can be manipulated by donations, imbalance, slippage, stale balances, or virtual price assumptions.
- How deposits, withdrawals, liquidity additions/removals, gauge staking, and reward minting are sequenced.
- Whether AMO activity can affect OETH peg, vault solvency, or reported asset value.
- Whether Curve rewards are harvested, converted, or dripped in a way that can be front-run or mis-accounted.

### Strategy Allocation Flow

Strategies are likely managed by vault/admin or strategist roles. The in-scope interfaces include `IStrategy` and `IVault`, and strategy implementations cover Curve, Morpho V2, generalized ERC-4626 vaults, native staking, cross-chain movement, and bridged assets.

Review should trace:

- Strategy deposit and withdrawal entry points.
- Who can call allocation, withdrawal, harvest, and emergency functions.
- How each strategy reports assets back to the vault.
- Whether reported assets include pending rewards, staked positions, bridged assets, or claimable balances.
- Whether strategy losses are recognized immediately and consistently.

### Harvest and Dripping Flow

`Dripper` and `FixedRateDripper` suggest yield or rewards may be released gradually rather than immediately. Security review should inspect:

- Source of funds sent to drippers.
- Release-rate configuration authority.
- Accounting treatment of undistributed balances.
- Rounding behavior over time.
- Whether changing rates or sweeping funds can create value leakage.

### Cross-Chain Flow

The scope includes `AbstractCCTPIntegrator`, `CrossChainMasterStrategy`, `CrossChainRemoteStrategy`, `CrossChainStrategyProxy`, CCTP interfaces, and bridged WOETH strategy code. This creates a cross-chain trust boundary.

Review should focus on:

- Message authentication and domain validation.
- Replay protection.
- Finality and failure handling.
- Accounting for funds in transit.
- Authority split between master and remote strategies.
- CREATE2 deployment assumptions for cross-chain strategy proxies.

## Accounting and Value Flow

The highest-risk accounting surfaces are vault solvency, token supply, wrapper conversion rates, strategy asset reporting, and external-position valuation.

Key accounting questions for audit:

- Does the vault’s total asset calculation match actual recoverable value?
- Are strategy-reported assets independently verifiable or dependent on manipulable external state?
- Can a strategy report assets that are illiquid, bridged, pending, slashed, or claimable but not immediately withdrawable?
- Are rebasing token balances and wrapped share balances converted consistently?
- Are rounding losses assigned intentionally and bounded?
- Can a privileged role move assets between vault, strategy, dripper, and external protocol without preserving accounting invariants?
- Are losses socialized correctly across token holders?

The provided source material does not define formal invariants. Auditors should derive them from code, especially around vault total assets, token supply, wrapper exchange rates, and strategy debt/asset accounting.

## External Integrations

The scoped interfaces show integrations with:

- Curve StableSwap NG, liquidity gauge, and minter.
- Morpho V2 adapter and vault interfaces.
- ERC-4626-style vaults through generalized strategy code.
- Merkl rewards.
- CCTP cross-chain messaging/bridging.
- Native staking and SSV-related strategy contracts.
- Sonic-specific token, vault, staking, and proxy contracts.

Security-relevant assumptions for these integrations:

- External protocol accounting may be manipulable within a transaction or across low-liquidity periods.
- Reward contracts may have claim timing, authorization, or token-behavior assumptions.
- Cross-chain messages introduce finality, replay, domain, and liveness risks.
- ERC-4626 integrations require careful handling of share price manipulation, donation attacks, preview/redeem mismatch, and withdrawal liquidity.
- Staking strategies may expose validator, slashing, withdrawal-queue, or delayed-finality risks.

The Immunefi prohibited activities restrict testing with pricing oracles or third-party smart contracts and require testing only on local forks of public testnet or mainnet. Audit work should therefore model third-party interactions locally and avoid live-system testing.

## Trust Boundaries

Primary trust boundaries visible from scope:

- Governance: upgrade control, admin configuration, and privileged protocol changes.
- Strategist roles: strategy movement and operational permissions.
- Proxy admins/initializers: implementation control and storage safety.
- External protocols: Curve, Morpho, Merkl, CCTP, ERC-4626 vaults, staking systems, and Sonic-specific infrastructure.
- Cross-chain counterpart contracts: master/remote strategy trust and message authenticity.
- Reward and dripper mechanisms: delayed value distribution and configurable release rates.

Privileged-role review should determine whether compromised governance or strategist authority is considered in scope, whether timelocks or multisigs are assumed, and whether role misuse can directly cause user fund loss.

## Security-Relevant Assumptions

The provided source material supports only limited protocol-level assumptions. The following should be validated directly against code:

- Vault accounting is expected to remain solvent across strategy deposits, withdrawals, harvests, and losses.
- Strategy asset reports should reflect recoverable value, not merely nominal balances.
- AMO interactions with Curve should not allow pool manipulation to mint, redeem, withdraw, or report value incorrectly.
- Wrapped-token conversions should preserve value across rebasing and non-rebasing representations.
- Cross-chain strategy accounting should handle in-flight funds, failed messages, replay attempts, and remote-chain delays.
- Upgradeable proxies should be initialized exactly once and preserve storage compatibility.
- Dripper release mechanics should not allow accelerated, repeated, or misdirected reward distribution.

## Immunefi Bounty Context

Program: Origin Protocol smart-contract bounty.

Program overview focus: Curve OETH+WETH (AMO) Strategy.

Rewards:

- Critical smart-contract impact: up to `1,000,000` OUSD.
- High smart-contract impact: fixed `15,000` OUSD.
- Reward token: OUSD on Ethereum.

Proof of Concept: required.

Primacy: Immunefi program rules state `primacy_of_rules`.

Prohibited activities:

- No testing on mainnet or public testnet deployed code; testing must be on local forks of public testnet or mainnet.
- No testing with pricing oracles or third-party smart contracts.
- No phishing or social engineering against employees or customers.
- No testing of third-party systems, applications, browser extensions, websites, SSO providers, advertising networks, or similar systems.
- No denial-of-service attacks against project assets.
- No automated testing that generates significant traffic.
- No public disclosure of an unpatched vulnerability in an embargoed bounty.
- Immunefi Rules also apply.

## Omitted Items

- Web & App scope, impacts, resources, and findings: intentionally excluded because this document is for Solidity smart-contract audit context only.
- Thirty-three low-signal, binary, directory, or over-budget links noted by context discovery: omitted because their contents were not provided and they were pre-classified as low-signal or unsuitable.
- Immunefi Resources documentation links: no documentation links were present in the provided Resources excerpt.
- Marketing, setup instructions, changelogs, and generic contest rules: omitted unless reflected in the provided Immunefi bounty constraints above.

## Source Notes

- Immunefi Information, `https://immunefi.com/bug-bounty/originprotocol/information/`: used for program overview, reward structure, PoC requirement, primacy, reward token/network, and prohibited activities.
- Immunefi Resources, `https://immunefi.com/bug-bounty/originprotocol/resources/`: checked as primary protocol documentation per instruction, but the provided excerpt did not include documentation links.
- Machine-readable scope from the context bundle: used to identify in-scope contracts, architecture areas, and integration surfaces. Behavioral descriptions derived from filenames/interfaces are intentionally framed as audit hypotheses to verify in code, not as confirmed protocol facts.

### origin-dollar-immunefi-bounty-rules.md

# Immunefi Bounty Rules - Origin Protocol

Use this file as mandatory context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/originprotocol/information/
- Scope: https://immunefi.com/bug-bounty/originprotocol/scope/
- Resources: https://immunefi.com/bug-bounty/originprotocol/resources/

## Program Requirements

- Proof of Concept: required
- Primacy: primacy_of_rules
- Rewards token: OUSD
- Rewards token network: Ethereum
- Maximum bounty: $1000000

## Assets In Scope

> Smart Contract category only. Web & App assets are intentionally excluded.

- smart contract: https://etherscan.io/address/0xba0e352AB5c13861C26e4E773e7a833C3A223FE6
  - Description: Curve OETH+WETH (AMO) Strategy
- smart contract: https://etherscan.io/address/0x3643cafA6eF3dd7Fcc2ADaD1cabf708075AFFf6e
  - Description: Morpho OUSD v2 Strategy
- smart contract: https://etherscan.io/address/0x26a02ec47ACC2A3442b757F45E0A82B8e993Ce11
  - Description: Curve USDC AMO Strategy
- smart contract: https://etherscan.io/address/0x85b78aca6deae198fbf201c82daf6ca21942acc6#code
  - Description: ARM (stETH/WETH)
- smart contract: https://basescan.org/address/0xF611cC500eEE7E4e4763A05FE623E2363c86d2Af
  - Description: AerodromeAMOStrategyProxy
- smart contract: https://sonicscan.org/address/0x31a91336414d3b955e494e7d485a6b06b55fc8fb#code
  - Description: Origin Timelock
- smart contract: https://etherscan.io/address/0xe75d77b1865ae93c7eaa3040b038d7aa7bc02f70
  - Description: Vault
- smart contract (Primacy of Impact placeholder): https://immunefi.com
  - Description: Primacy of Impact
- smart contract: https://sonicscan.org/address/0xb1e25689D55734FD3ffFc939c4C3Eb52DFf8A794#code
  - Description: Origin Sonic
- smart contract: https://etherscan.io/address/0xaF04828Ed923216c77dC22a2fc8E077FDaDAA87d
  - Description: CompoundingStakingSSVStrategyProxy
- smart contract: https://etherscan.io/address/0x7609c88e5880e934dd3a75bcfef44e31b1badb8b#code
  - Description: OGN rewards
- smart contract: https://etherscan.io/address/0x63898b3b6ef3d39332082178656e9862bee45c57
  - Description: xOGN
- smart contract: https://etherscan.io/address/0x501804B374EF06fa9C427476147ac09F1551B9A0
  - Description: OGN Staking
- smart contract: https://sonicscan.org/address/0xa3c0eCA00D2B76b4d1F170b0AB3FdeA16C180186#code
  - Description: Origin Sonic Vault
- smart contract: https://basescan.org/address/0x80c864704DD06C3693ed5179190786EE38ACf835
  - Description: BridgedWOETHStrategy
- smart contract: https://sonicscan.org/address/0x596B0401479f6DfE1cAF8c12838311FeE742B95c#code
  - Description: Sonic Staking Strategy
- smart contract: https://etherscan.io/address/0xDcEe70654261AF21C44c093C300eD3Bb97b78192
  - Description: wOETH Token
- smart contract: https://basescan.org/address/0xf817cb3092179083c48c014688d98b72fb61464f
  - Description: Timelock
- smart contract: https://sonicscan.org/address/0x9F0dF7799f6FDAd409300080cfF680f5A23df4b1#code
  - Description: Wrapped Origin Sonic
- smart contract: https://etherscan.io/address/0x1D3Fbd4d129Ddd2372EA85c5Fa00b2682081c9EC
  - Description: Governor / Timelock
- smart contract: https://basescan.org/address/0xdbfefd2e8460a6ee4955a68582f85708baea60a3#code
  - Description: SuperOETHb Token
- smart contract: https://etherscan.io/address/0xEDf495F92c2eBdEE8B797E9C503aA7A3302A9c88
  - Description: CompoundingStakingStrategyView
- smart contract: https://etherscan.io/address/0xc4444C5D9e7C1a5A0a01c5E4b11692d589DcAF22
  - Description: BeaconProofs
- smart contract: https://basescan.org/address/0xdbfefd2e8460a6ee4955a68582f85708baea60a3#code
  - Description: Wrapped SuperOETHb
- smart contract: https://etherscan.io/address/0x95c347d6214614a780847b8aaf4f96eb84f4da6d
  - Description: Migrator
- smart contract: https://etherscan.io/address/0x39254033945AA2E4809Cc2977E7087BEE48bd7Ab
  - Description: OETH Vault
- smart contract: https://sonicscan.org/address/0x5b72992e9CDe8C07CE7C8217eB014EC7fD281f03#code
  - Description: Origin Sonic Dripper
- smart contract: https://etherscan.io/address/0x2A8e1E676Ec238d8A992307B495b45B3fEAa5e86
  - Description: OUSD
- smart contract: https://basescan.org/address/0x98a0cbef61bd2d21435f433be4cd42b56b38cc93
  - Description: SuperOETHb Vault
- smart contract: https://etherscan.io/address/0xB1d624fc40824683e2bFBEfd19eB208DbBE00866
  - Description: OUSD Morpho V2 CrossChain Master Strategy
- smart contract: https://basescan.org/address/0xB1d624fc40824683e2bFBEfd19eB208DbBE00866
  - Description: OUSD Morpho V2 CrossChain Remote Strategy

## Impacts In Scope

- critical (smart contract): Any governance voting result manipulation
- critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- critical (smart contract): Permanent freezing of funds
- critical (smart contract): Protocol insolvency
- high (smart contract): Theft of unclaimed yield
- high (smart contract): Permanent freezing of unclaimed yield
- high (smart contract): Temporary freezing of funds

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

- No structured audit links were extracted. Read Resources directly.

## Stage 1 Eligibility Rules

- A finding must affect an in-scope asset, unless the exact category and severity are covered by the program's Primacy of Impact rules.
- A finding must produce an impact listed in the program's Impacts in Scope.
- Exclude known issues, prior audit findings, documented accepted risks, closed duplicate reports, and program-specific OOS cases.
- Exclude cases requiring privileged access, leaked credentials, social engineering, malicious or mistaken trusted roles, deployment mistakes, test/mock files, public disclosure, or third-party-only failures.
- Do not perform final exploitability or severity scoring in stage 1; keep only when there is no decisive eligibility blocker.

## Link Handling

- Do not follow Immunefi navigation, marketing, login, social, newsletter, or platform-help links during validation.
- Use only the Source URLs above, in-scope explorer links, codebase links, documentation links, and prior-audit links when live verification is necessary.



### origin-dollar-immunefi-severity-rubric.md

# Immunefi Severity Rubric - Origin Protocol

Use this file as mandatory severity context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/originprotocol/information/
- Scope: https://immunefi.com/bug-bounty/originprotocol/scope/
- Resources: https://immunefi.com/bug-bounty/originprotocol/resources/
- Immunefi severity system v2.2: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-2/

## Program-Specific Severity Source Of Truth

- Primacy: primacy_of_rules
- Proof of Concept: required

## Impacts In Scope

- critical (smart contract): Any governance voting result manipulation
- critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- critical (smart contract): Permanent freezing of funds
- critical (smart contract): Protocol insolvency
- high (smart contract): Theft of unclaimed yield
- high (smart contract): Permanent freezing of unclaimed yield
- high (smart contract): Temporary freezing of funds

## Rewards By Threat Level

- critical (smart contract) [up_to]: max $1000000
- high (smart contract) [fixed]: fixed $15000

## Platform Smart Contract Severity Summary

Always prefer the program's exact impact rows above. Use this summary only to interpret the referenced Immunefi severity system.

### v2.2 Smart Contract Summary

- Critical and High are still impact-first, but some program pages using v2.2 add stricter profitability, freezing, or end-effect clauses.
- Always use the program's listed impact rows and reward body when v2.2 text conflicts with generic platform summaries.
- Treat project-specific exclusions for non-standard tokens, malicious integrations, and trusted components as mandatory.

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



### origin-dollar-immunefi-poc-runtime.md

# Immunefi PoC Runtime - Origin Protocol

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
| BridgedWOETHStrategy | smart_contract | `0x80c864704DD06C3693ed5179190786EE38ACf835` | `base-mainnet` | `BASE_RPC_URL` | `true` | https://basescan.org/address/0x80c864704DD06C3693ed5179190786EE38ACf835 |
| SuperOETHb Vault | smart_contract | `0x98a0cbef61bd2d21435f433be4cd42b56b38cc93` | `base-mainnet` | `BASE_RPC_URL` | `true` | https://basescan.org/address/0x98a0cbef61bd2d21435f433be4cd42b56b38cc93 |
| OUSD Morpho V2 CrossChain Remote Strategy | smart_contract | `0xB1d624fc40824683e2bFBEfd19eB208DbBE00866` | `base-mainnet` | `BASE_RPC_URL` | `true` | https://basescan.org/address/0xB1d624fc40824683e2bFBEfd19eB208DbBE00866 |
| AerodromeAMOStrategyProxy | smart_contract | `0xF611cC500eEE7E4e4763A05FE623E2363c86d2Af` | `base-mainnet` | `BASE_RPC_URL` | `true` | https://basescan.org/address/0xF611cC500eEE7E4e4763A05FE623E2363c86d2Af |
| SuperOETHb Token | smart_contract | `0xdbfefd2e8460a6ee4955a68582f85708baea60a3` | `base-mainnet` | `BASE_RPC_URL` | `true` | https://basescan.org/address/0xdbfefd2e8460a6ee4955a68582f85708baea60a3#code |
| Timelock | smart_contract | `0xf817cb3092179083c48c014688d98b72fb61464f` | `base-mainnet` | `BASE_RPC_URL` | `true` | https://basescan.org/address/0xf817cb3092179083c48c014688d98b72fb61464f |
| Governor / Timelock | smart_contract | `0x1D3Fbd4d129Ddd2372EA85c5Fa00b2682081c9EC` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x1D3Fbd4d129Ddd2372EA85c5Fa00b2682081c9EC |
| Curve USDC AMO Strategy | smart_contract | `0x26a02ec47ACC2A3442b757F45E0A82B8e993Ce11` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x26a02ec47ACC2A3442b757F45E0A82B8e993Ce11 |
| OUSD | smart_contract | `0x2A8e1E676Ec238d8A992307B495b45B3fEAa5e86` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x2A8e1E676Ec238d8A992307B495b45B3fEAa5e86 |
| Morpho OUSD v2 Strategy | smart_contract | `0x3643cafA6eF3dd7Fcc2ADaD1cabf708075AFFf6e` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x3643cafA6eF3dd7Fcc2ADaD1cabf708075AFFf6e |
| OETH Vault | smart_contract | `0x39254033945AA2E4809Cc2977E7087BEE48bd7Ab` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x39254033945AA2E4809Cc2977E7087BEE48bd7Ab |
| OGN Staking | smart_contract | `0x501804B374EF06fa9C427476147ac09F1551B9A0` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x501804B374EF06fa9C427476147ac09F1551B9A0 |
| xOGN | smart_contract | `0x63898b3b6ef3d39332082178656e9862bee45c57` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x63898b3b6ef3d39332082178656e9862bee45c57 |
| OGN rewards | smart_contract | `0x7609c88e5880e934dd3a75bcfef44e31b1badb8b` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x7609c88e5880e934dd3a75bcfef44e31b1badb8b#code |
| ARM (stETH/WETH) | smart_contract | `0x85b78aca6deae198fbf201c82daf6ca21942acc6` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x85b78aca6deae198fbf201c82daf6ca21942acc6#code |
| Migrator | smart_contract | `0x95c347d6214614a780847b8aaf4f96eb84f4da6d` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x95c347d6214614a780847b8aaf4f96eb84f4da6d |
| OUSD Morpho V2 CrossChain Master Strategy | smart_contract | `0xB1d624fc40824683e2bFBEfd19eB208DbBE00866` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xB1d624fc40824683e2bFBEfd19eB208DbBE00866 |
| wOETH Token | smart_contract | `0xDcEe70654261AF21C44c093C300eD3Bb97b78192` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xDcEe70654261AF21C44c093C300eD3Bb97b78192 |
| CompoundingStakingStrategyView | smart_contract | `0xEDf495F92c2eBdEE8B797E9C503aA7A3302A9c88` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xEDf495F92c2eBdEE8B797E9C503aA7A3302A9c88 |
| CompoundingStakingSSVStrategyProxy | smart_contract | `0xaF04828Ed923216c77dC22a2fc8E077FDaDAA87d` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xaF04828Ed923216c77dC22a2fc8E077FDaDAA87d |
| Curve OETH+WETH (AMO) Strategy | smart_contract | `0xba0e352AB5c13861C26e4E773e7a833C3A223FE6` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xba0e352AB5c13861C26e4E773e7a833C3A223FE6 |
| BeaconProofs | smart_contract | `0xc4444C5D9e7C1a5A0a01c5E4b11692d589DcAF22` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xc4444C5D9e7C1a5A0a01c5E4b11692d589DcAF22 |
| Vault | smart_contract | `0xe75d77b1865ae93c7eaa3040b038d7aa7bc02f70` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xe75d77b1865ae93c7eaa3040b038d7aa7bc02f70 |
| Origin Timelock | smart_contract | `0x31a91336414d3b955e494e7d485a6b06b55fc8fb` | `sonic-mainnet` | `SONIC_RPC_URL` | `true` | https://sonicscan.org/address/0x31a91336414d3b955e494e7d485a6b06b55fc8fb#code |
| Sonic Staking Strategy | smart_contract | `0x596B0401479f6DfE1cAF8c12838311FeE742B95c` | `sonic-mainnet` | `SONIC_RPC_URL` | `true` | https://sonicscan.org/address/0x596B0401479f6DfE1cAF8c12838311FeE742B95c#code |
| Origin Sonic Dripper | smart_contract | `0x5b72992e9CDe8C07CE7C8217eB014EC7fD281f03` | `sonic-mainnet` | `SONIC_RPC_URL` | `true` | https://sonicscan.org/address/0x5b72992e9CDe8C07CE7C8217eB014EC7fD281f03#code |
| Wrapped Origin Sonic | smart_contract | `0x9F0dF7799f6FDAd409300080cfF680f5A23df4b1` | `sonic-mainnet` | `SONIC_RPC_URL` | `true` | https://sonicscan.org/address/0x9F0dF7799f6FDAd409300080cfF680f5A23df4b1#code |
| Origin Sonic Vault | smart_contract | `0xa3c0eCA00D2B76b4d1F170b0AB3FdeA16C180186` | `sonic-mainnet` | `SONIC_RPC_URL` | `true` | https://sonicscan.org/address/0xa3c0eCA00D2B76b4d1F170b0AB3FdeA16C180186#code |
| Origin Sonic | smart_contract | `0xb1e25689D55734FD3ffFc939c4C3Eb52DFf8A794` | `sonic-mainnet` | `SONIC_RPC_URL` | `true` | https://sonicscan.org/address/0xb1e25689D55734FD3ffFc939c4C3Eb52DFf8A794#code |

## Network RPC Availability

| Network | Kind | RPC Env Var | Env Available | Asset Count |
| --- | --- | --- | --- | --- |
| `base-mainnet` | `mainnet` | `BASE_RPC_URL` | `true` | `6` |
| `ethereum-mainnet` | `mainnet` | `MAINNET_RPC_URL` | `true` | `17` |
| `sonic-mainnet` | `mainnet` | `SONIC_RPC_URL` | `true` | `6` |

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

 ### package.json

{
  "name": "origin-dollar",
  "license": "MIT",
  "homepage": "https://github.com/originprotocol/origin-dollar",
  "scripts": {
    "build": "(cd contracts && NODE_ENV=development pnpm install && npx hardhat compile)",
    "start": "(cd contracts && pnpm run node)",
    "test": "exit 0",
    "prepare": "husky"
  },
  "engines": {
    "node": "20"
  },
  "devDependencies": {
    "danger": "^11.2.8",
    "husky": "^9.0.11"
  }
}


