
 ------------ ## PROTOCOL OVERVIEW ------------ 

# Ethena `bbp-public-assets` Protocol Summary

## What The Protocol Is
Ethena is a synthetic dollar protocol centered on `USDe`, an ERC20-style dollar-denominated asset that is described as crypto-backed rather than fiat-backed. The broader protocol aims to maintain dollar-like value through a combination of off-chain backing assets, short derivatives hedges, and on-chain minting, redemption, and staking logic. Within this repository, the core on-chain system includes:

- `USDe` as the main synthetic dollar token
- `StakedUSDe` / `StakedUSDeV2` as staking wrappers for earning protocol revenue
- `EthenaMinting` as the permissioned mint/redeem gateway
- `USDeSilo` as a custody/escrow-style component used in staking or cooldown flows
- `StakingRewardsDistributor` for USDe-to-sUSDe rewards distribution
- `EthenaLPStaking` for LP staking and rewards accounting
- `ENA` as a separate in-scope token
- `SingleAdminAccessControl` as the central privileged access layer

At a high level, the protocol separates economic backing from on-chain token accounting:

1. Off-chain or external systems hold collateral, manage custody, and maintain hedges.
2. On-chain contracts account for token issuance, redemptions, staking shares, reward flows, and administrative permissions.

That separation is important: these contracts do not by themselves create price stability. Instead, they encode the token, staking, and operational controls that represent claims within Ethena’s broader synthetic-dollar system.

## Core Design

### USDe
`USDe` is the main synthetic dollar token. It is intended to be broadly transferable and usable across DeFi. Users can acquire it in two different ways:

- **Permissionlessly on external AMMs** by swapping assets like USDC or USDT for USDe
- **Directly from the protocol** through permissioned minting, available only to approved market makers

The documentation makes clear that direct protocol minting and redemption are not open to all users. Instead, approved counterparties that pass KYC/KYB can mint USDe by delivering accepted reserve assets, or redeem USDe by burning it in exchange for backing assets.

This means `USDe` serves two roles simultaneously:

- A freely transferable token in public markets
- A controlled liability issued and redeemed by a permissioned gateway

### sUSDe / Staked USDe
`StakedUSDe` and `StakedUSDeV2` represent the staking layer. Users deposit USDe and receive a staked claim that accrues protocol revenue. Economically, this behaves like a share-based savings product:

- Users deposit `USDe`
- They receive a staked position, typically representing proportional ownership of pooled assets plus future rewards
- Rewards increase the value claim of stakers over time
- Unstaking may involve cooldown, escrow, or silo mechanics before final withdrawal

The staking system is therefore one of the protocol’s most sensitive accounting surfaces, because it must correctly preserve proportional ownership across deposits, rewards, cooldowns, and withdrawals.

### ENA
`ENA` is an additional in-scope token contract. The prompt does not provide detailed mechanics for governance, utility, or emissions, so it should be treated as a scoped token component rather than a fully documented subsystem. It is relevant because it may interact with staking, rewards, or protocol administration elsewhere in the system design.

## How The Protocol Works

## 1. Permissioned Mint And Redeem
`EthenaMinting.sol` is the central contract for controlled issuance and redemption of `USDe`.

### Mint flow
The intended mint path is:

1. An approved market maker submits a mint request.
2. The requester transfers accepted reserve assets.
3. The protocol validates authorization and order parameters.
4. `USDe` is minted to the authorized recipient.

### Redeem flow
The intended redeem path is:

1. An approved market maker submits a redemption request.
2. The requester provides or burns `USDe`.
3. The protocol validates authorization and order parameters.
4. Backing assets are released to the approved recipient.

Because mint/redeem is permissioned, the main security model is not open-price discovery on-chain, but **authorization correctness**. The contract must ensure that only valid counterparties can:

- Use the mint or redeem path
- Use only approved assets
- Reuse no expired or replayed orders
- Receive no more value than intended
- Stay within configured operational limits

This also means off-chain order construction, signer trust, and custody routing are part of the effective security boundary even if not fully visible in Solidity.

## 2. External Market Liquidity
Ordinary users are expected to access USDe mainly through secondary markets such as AMM pools. This makes the protocol hybrid:

- Primary issuance/redemption is restricted
- Secondary market acquisition/disposal is permissionless

From a design perspective, this allows market makers to arbitrage between protocol mint/redeem flows and public market prices. The protocol itself does not appear, from the provided material, to rely on a fully on-chain peg module. Instead, peg behavior is partly delegated to market structure and arbitrage incentives.

## 3. Staking And Yield Distribution
The staking layer lets users convert `USDe` into a staked claim that earns protocol revenue.

The basic model is:

1. A user deposits USDe into the staking contract.
2. The contract issues a staked balance or shares.
3. Revenue or rewards are periodically added.
4. Existing stakers benefit proportionally.
5. A user later initiates unstake/withdrawal, potentially via cooldown or silo.

The inclusion of both `StakedUSDe` and `StakedUSDeV2` suggests either an upgrade iteration or a second version of staking logic. The documentation does not specify the exact delta, but auditors should assume version-to-version behavior may matter for migrations, accounting consistency, and inherited storage or role assumptions.

## 4. Cooldown / Silo Mechanics
`USDeSilo.sol` is described as part of staking and withdrawal flows. Based on the provided context, it likely holds escrowed or delayed-withdrawal balances during cooldown periods.

Its likely role in the system is:

- Receive USDe associated with pending unstake operations
- Segregate assets that are no longer actively earning but not yet withdrawable
- Release them only to the correct beneficiary when conditions are met

That makes `USDeSilo` an important ownership and state-transition component. Any error here could cause double-claims, stuck funds, or misattribution of withdrawal rights.

## 5. Rewards Distribution
`StakingRewardsDistributor.sol` appears to be the distributor for rewards flowing from USDe into the staking system. Its job is likely to move reward assets into the staked pool or otherwise update reward state so stakers receive yield.

Security-critical properties for this component include:

- Rewards are only distributed from valid sources
- Distribution timing cannot be abused to dilute or over-reward certain users
- Reward transfers cannot be stolen, stranded, or redirected
- Accounting remains consistent even if transfers happen at awkward times

This is especially important in share-based systems, where deposits just before a reward event or withdrawals just after it can create timing edge cases.

## 6. LP Staking
`EthenaLPStaking.sol` is a separate staking subsystem for liquidity provider positions. Although the prompt does not describe its full mechanics, it is clearly a reward/accounting contract that likely tracks:

- Staked LP token balances
- Reward accrual rates or allocations
- User claimable rewards
- Emergency or administrative flows

LP staking contracts often have different risks from simple token staking because the deposited asset may itself be a receipt token representing underlying pool exposure.

## Trust Model And Roles
A major characteristic of this protocol is that it is **not trust-minimized end-to-end**. It depends on privileged operational roles and off-chain systems.

### Centralized administration
`SingleAdminAccessControl.sol` indicates a centralized admin model. The exact role graph is not provided, but the repository clearly expects privileged actors to manage system-critical functions such as:

- Granting or revoking access
- Configuring mint/redeem permissions
- Managing staking or reward components
- Adjusting emergency settings
- Potentially pausing or rerouting sensitive operations

### Approved counterparties
Direct mint/redeem is limited to approved market-making entities. This is a core trust boundary. The protocol assumes these actors are intentionally authorized, but the contracts must still prevent:

- Unauthorized access
- Privilege escalation
- Order tampering
- Replay or nonce reuse
- Circumvention of expiry or rate limits

### External operations
The documentation also assumes trusted off-chain infrastructure for:

- Custody of backing assets
- Execution and maintenance of delta hedges
- Market maker onboarding and compliance
- Potential order signing or approval workflows

So the repository implements only the on-chain leg of a larger financial system.

## Economic Model
The economic idea behind Ethena is that `USDe` is backed by a combination of crypto spot assets and offsetting derivative positions. Liquid stablecoins may also be used to improve hedging efficiency or operational flexibility. In simplified terms:

- Spot crypto creates asset exposure
- Short futures reduce directional price risk
- Stablecoin reserves may supplement liquidity and execution
- On-chain USDe supply represents tokenized liabilities against that managed backing structure

This means the protocol’s on-chain contracts are tightly coupled to solvency assumptions that are partly external. On-chain correctness alone is not enough for a robust peg, but on-chain errors could still directly cause over-issuance, over-redemption, or loss of user funds.

## Main Security-Critical Invariants
From the provided materials, the most important protocol invariants are:

### Mint/redeem invariants
- USDe should only be minted through authorized flows.
- Mint amounts should correspond to accepted reserve-asset intake.
- Redemptions should only release authorized backing value.
- Replay, stale orders, or malformed approvals must not create duplicate mint/redeem outcomes.

### Staking invariants
- Staking share issuance must preserve fair proportional ownership.
- Rewards must accrue to the correct holders.
- Deposits, rewards, cooldown balances, and withdrawals must not be double-counted.
- Users should not be able to inflate their claim through donations, timing, or rounding edge cases.

### Silo / withdrawal invariants
- Escrowed or cooling-down funds remain attributable to the correct user.
- Release conditions are enforced exactly once.
- Pending withdrawals cannot be stolen or made permanently inaccessible by accounting mistakes.

### Access-control invariants
- Only intended privileged roles can invoke admin functions.
- Admin transitions cannot leave the system in an unusable or hijackable state.
- No role should implicitly gain authority over unrelated assets or flows unless explicitly intended.

### Reward-distribution invariants
- Reward funding cannot be drained or redirected.
- Late or early entrants cannot capture disproportionate rewards unless designed.
- Reward mechanics should remain robust against non-standard ERC20 behavior.

## External Dependency Assumptions
The protocol interacts with assets and systems outside its direct control. The prompt explicitly highlights the need to account for:

- Non-standard ERC20 tokens
- Fee-on-transfer behavior
- Rebasing behavior
- Transfer return-value inconsistencies
- Frozen or blacklisted stablecoins
- Depeg scenarios in external stable assets
- WETH9 wrapping/unwrapping assumptions

These assumptions matter most in minting, redemption, and reward funding, where asset movement and accounting must stay exact despite token quirks.

## Repository Scope By Functional Area
The scoped contracts cluster into the following functional categories:

- **Token layer:** `USDe.sol`, `ENA.sol`
- **Staking layer:** `StakedUSDe.sol`, `StakedUSDeV2.sol`, `EthenaLPStaking.sol`
- **Mint/redeem gateway:** `EthenaMinting.sol`
- **Reward and escrow support:** `StakingRewardsDistributor.sol`, `USDeSilo.sol`
- **Permissions:** `SingleAdminAccessControl.sol`
- **Interfaces:** definitions for minting, staking, cooldown, access control, silo, USDe, ENA, and WETH9

Taken together, these implement the on-chain operational core of Ethena’s public-asset system.

## Practical Audit Interpretation
From a security-review standpoint, this is best understood as a **permissioned issuance system plus public token circulation plus share-based yield accounting**.

The highest-risk areas are therefore:

- Mint/redeem authorization and replay safety
- Supply integrity of `USDe`
- Share-accounting correctness for `sUSDe`
- Reward timing and distribution correctness
- Cooldown/silo ownership logic
- Centralized access control and admin transitions
- Handling of unusual ERC20 behavior in reserve/reward assets

The protocol is not a simple stablecoin, pure vault, or pure staking system. It combines all three patterns:

- A centrally controlled mint/redeem engine
- A freely circulating synthetic dollar token
- A vault-like staking product that distributes protocol revenue

That hybrid structure is what defines both its functionality and its main attack surface.

## Bottom Line
Ethena’s `bbp-public-assets` repository implements the on-chain contracts for a synthetic dollar ecosystem built around `USDe`. Approved counterparties can mint and redeem through `EthenaMinting`, while ordinary users mainly interact through public markets and staking products. `StakedUSDe` and related support contracts turn USDe into a yield-bearing position, while `SingleAdminAccessControl` and distributor/silo contracts coordinate privilege, rewards, and delayed withdrawals.

Operationally, the protocol depends on both strict on-chain accounting and trusted off-chain reserve, hedge, and custody processes. The Solidity system’s main job is to ensure that token issuance, redemption, staking shares, rewards, and withdrawals remain internally consistent and only executable by the intended parties under the intended conditions.


 ------------ ## Main List of Files in Project ------------ 

contracts/contracts/ENA.sol
contracts/contracts/EthenaLPStaking.sol
contracts/contracts/EthenaMinting.sol
contracts/contracts/SingleAdminAccessControl.sol
contracts/contracts/StakedUSDe.sol
contracts/contracts/StakedUSDeV2.sol
contracts/contracts/StakingRewardsDistributor.sol
contracts/contracts/USDe.sol
contracts/contracts/USDeSilo.sol
contracts/contracts/interfaces/IENADefinitions.sol
contracts/contracts/interfaces/IEthenaLPStakingDefinitions.sol
contracts/contracts/interfaces/IEthenaMinting.sol
contracts/contracts/interfaces/ISingleAdminAccessControl.sol
contracts/contracts/interfaces/IStakedUSDe.sol
contracts/contracts/interfaces/IStakedUSDeCooldown.sol
contracts/contracts/interfaces/IStakingRewardsDistributor.sol
contracts/contracts/interfaces/IUSDe.sol
contracts/contracts/interfaces/IUSDeDefinitions.sol
contracts/contracts/interfaces/IUSDeSiloDefinitions.sol
contracts/contracts/interfaces/IWETH9.sol


 ------------ ## DOCUMENTATION: ------------ 

 ### bbp-public-assets-docs.md

# bbp-public-assets Protocol Documentation

## Scope And Sources

This document summarizes protocol mechanics relevant to a Solidity security audit of Ethena's `bbp-public-assets` repository at commit `f3e56d5f06bfef82367d5d5b561398e91d5bebc1`.

Primary sources used:

- Immunefi Ethena program overview and resources: `https://immunefi.com/bug-bounty/ethena/information/`, `https://immunefi.com/bug-bounty/ethena/resources/`
- Ethena documentation root: `https://ethena-labs.gitbook.io/ethena-labs`
- Ethena key addresses page: `https://docs.ethena.fi/solution-design/key-addresses`

Web/App bounty material was intentionally excluded; this document only covers Smart Contract scope and protocol documentation useful for contract review.

## Protocol Overview

Ethena is a synthetic dollar protocol built on Ethereum. Its main asset, `USDe`, is described as a synthetic dollar rather than a fiat-backed stablecoin. It is backed by crypto assets and corresponding short futures positions, with additional support from liquid stablecoins such as USDC and USDT.

The protocol also offers `sUSDe`, a staked USDe savings asset. Users can stake USDe to receive rewards derived from protocol revenue, subject to jurisdictional restrictions stated in the documentation.

The protocol's core stability mechanism is delta hedging: Ethena holds Bitcoin, Ethereum, and other governance-approved spot assets, then offsets their price exposure using perpetual or deliverable futures contracts. The intent is to maintain a relatively stable dollar value by combining spot collateral value with derivatives positions. The documentation states USDe is fully backed, subject to risk scenarios that may result in loss of backing.

## In-Scope Smart Contracts

Machine-readable smart contract scope includes:

- `contracts/contracts/USDe.sol`
- `contracts/contracts/ENA.sol`
- `contracts/contracts/StakedUSDe.sol`
- `contracts/contracts/StakedUSDeV2.sol`
- `contracts/contracts/EthenaMinting.sol`
- `contracts/contracts/EthenaLPStaking.sol`
- `contracts/contracts/StakingRewardsDistributor.sol`
- `contracts/contracts/USDeSilo.sol`
- `contracts/contracts/SingleAdminAccessControl.sol`
- Interfaces for USDe, ENA, Ethena minting, staking, cooldown, rewards distribution, access control, USDe silo, and WETH9.

The Immunefi program identifies `USDe.sol` in the program overview and links the `ethena-labs/bbp-public-assets` GitHub repository as the program codebase.

## Main Assets

### USDe

`USDe` is the protocol's synthetic dollar token. It is designed to be composable across CeFi and DeFi. Users may acquire or dispose of USDe permissionlessly through external AMM pools, including pools using assets such as USDT or USDC.

Security-relevant assumptions:

- USDe is not a fiat stablecoin and depends on correct backing, hedging, custody, and redemption operations.
- External markets may affect peg behavior and arbitrage flows, but the provided source material does not specify a complete on-chain peg controller.
- Direct minting and redemption are restricted to approved market-making counterparties that clear KYC/KYB checks.

### sUSDe

`sUSDe` represents staked USDe. Users stake USDe and may receive protocol revenue rewards. The documentation notes staking availability is restricted to permitted jurisdictions.

Security-relevant assumptions:

- Correct share accounting between USDe deposits, sUSDe balances, rewards, and withdrawals is critical.
- Reward distribution and staking flows are core value-transfer paths.
- Jurisdictional restrictions are documented as product constraints; the provided material does not state how they are enforced on-chain.

### ENA

`ENA` is included in scope, and the key addresses page lists ENA and staked ENA-related token contracts. The provided excerpts do not describe ENA governance or staking mechanics in detail, so auditors should avoid assuming additional ENA behavior beyond the scoped contracts.

## Main Flows

### Permissionless Acquisition And Disposal

Users can acquire or dispose of USDe through external AMM pools using assets such as USDT or USDC. These flows are external-market interactions rather than direct protocol mint/redeem flows.

Audit relevance:

- On-chain contracts may receive USDe sourced externally, so accounting should not assume USDe only enters through direct minting.
- External AMMs are integrations and liquidity venues, not trusted backing sources in the provided documentation.

### Direct Minting

Approved market-making counterparties can directly mint USDe by transferring accepted reserve assets and receiving USDe. This requires KYC/KYB approval according to the documentation.

Audit relevance:

- `EthenaMinting.sol` is a central trust boundary for reserve-asset intake and USDe issuance.
- Authorization, order validation, replay protection, asset allowlists, price assumptions, custody routing, and mint limits are high-value review areas.
- The source material does not provide exact accepted reserve assets or signing rules; those must be derived from code.

### Direct Redemption

Approved market-making counterparties can burn USDe and receive backing assets, subject to KYC/KYB approval.

Audit relevance:

- Redemption must preserve solvency and avoid over-withdrawal of backing assets.
- Burn-before-transfer ordering, asset accounting, role permissions, replay resistance, and failure handling are security-critical.
- Any off-chain approval/signature system becomes part of the trust boundary if used by `EthenaMinting.sol`.

### Staking And Unstaking

Users can stake USDe to receive rewards from protocol revenue. The in-scope contracts include `StakedUSDe.sol`, `StakedUSDeV2.sol`, `USDeSilo.sol`, and `StakingRewardsDistributor.sol`.

Audit relevance:

- Staking likely introduces share/accounting invariants between USDe held by staking contracts, sUSDe supply, reward transfers, cooldown or silo balances, and withdrawals.
- Reward distribution should be checked for front-running, donation/inflation effects, rounding loss, stale accounting, and privileged manipulation.
- Cooldown or silo mechanics, if present in code, should be reviewed as delayed-withdrawal trust boundaries.

### LP Staking

`EthenaLPStaking.sol` is in scope. The provided documentation excerpt does not describe LP staking mechanics beyond key-address references. Auditors should treat this as an in-scope reward/accounting system whose detailed behavior must be established from code.

## Accounting And Value Flow

The protocol value model has two layers:

1. Off-chain or externally held backing and hedging: crypto spot assets, futures positions, and liquid stablecoins support USDe's synthetic dollar value.
2. On-chain token and staking accounting: USDe mint/burn, sUSDe staking shares, reward distribution, silo/cooldown balances, LP staking balances, and ENA token behavior.

Important accounting relationships to verify in code:

- USDe minting should correspond to authorized receipt or allocation of accepted reserve assets.
- USDe redemption should burn the correct amount and release no more backing value than authorized.
- Staked USDe share issuance and redemption should preserve proportional ownership and avoid inflation via donations, rounding, or timing.
- Reward distributor transfers should not let privileged or early actors extract more than intended.
- Silo/cooldown balances should remain claimable by the correct users and should not be double-counted.
- Access-control changes should not bypass operational limits or create unexpected mint, burn, transfer, or reward authority.

The documentation says liquid stables may improve hedging efficiency and can act as a safeguard when funding rates and futures basis are suboptimal. This is a protocol-level economic assumption, not a complete on-chain invariant in the provided source material.

## External Integrations

Known integrations or external dependencies from the provided documentation:

- External AMM pools where users can acquire or dispose of USDe with assets such as USDT or USDC.
- Perpetual and deliverable futures venues used for delta hedging.
- Backing assets including Bitcoin, Ethereum, governance-approved spot assets, and liquid stables such as USDC and USDT.
- Custody and backing infrastructure referenced by the documentation navigation, but not detailed in the provided excerpts.
- WETH9 interface is included in scope.

Audit relevance:

- The smart contracts should be reviewed for assumptions about ERC20 behavior, token decimals, transfer return values, blacklisting/freezing, rebasing, fee-on-transfer, and non-standard stablecoin behavior.
- Futures and custody systems appear to be external to the in-scope Solidity code but are part of the protocol solvency model.
- Testing with pricing oracles or third-party smart contracts is prohibited by the Immunefi rules for this program; local-fork-only constraints apply.

## Trust Boundaries And Privileged Roles

The in-scope `SingleAdminAccessControl.sol` indicates a centralized access-control component. The docs also describe direct mint/redeem as limited to approved market-making counterparties.

Security-relevant trust boundaries:

- Admin or role holders that can mint, burn, pause, configure assets, manage staking/rewards, or update privileged addresses.
- Approved counterparties allowed to use direct mint/redeem flows.
- Off-chain systems that approve counterparties, construct mint/redeem orders, manage hedges, or custody backing assets.
- External liquidity venues used by ordinary users to buy or sell USDe.
- Reward distribution authority for sUSDe and LP staking.

Auditors should distinguish between intended trusted roles and exploitable privilege escalation. Governance or centralization risks are normally not valid unless they enable an impact beyond the documented trust model.

## Key Mainnet Addresses From Documentation

The key-addresses page lists these Ethereum mainnet core contracts:

- Mint and Redeem Contract V1: `0x2cc440b721d2cafd6d64908d6d8c4acc57f8afc3`
- Mint and Redeem Contract V2: `0xe3490297a08d6fC8Da46Edb7B6142E4F461b62D3`
- Staking / sUSDe Contract: `0x9d39a5de30e57443bff2a8307a4256c8797a3497`
- USDe Token Contract: `0x4c9edd5852cd905f086c759e8383e09bff1e68b3`
- USDe to sUSDe Staking Rewards Distributor: `0xf2fa332bd83149c66b09b45670bce64746c6b439`
- ENA Token Contract: `0x57e114B691Db790C35207b2e685D4A43181e6061`
- sENA Token Contract: `0x8bE3460A480c80728a8C4D7a5D5303c85ba7B3b9`
- rsENA Token Contract: `0xc65433845ecd16688eda196497fa9130d6c47bd8`

The same page also states Ethena tokens exist on many other chains, including TON, Aptos, Solana, Mantle, Blast, Arbitrum, Optimism, Base, Zircuit, ZKSync, BNB, Linea, Manta, Scroll, Fraxtal, Mode, XLayer, Metis, Kava, Morph, and Swell. Cross-chain token addresses are not included here because the audit scope is the Solidity repository listed above, and the excerpted table is incomplete.

## Security-Relevant Assumptions

- USDe stability depends on delta hedging and backing management, not fiat redemption alone.
- Solvency depends on correct reserve asset handling, derivatives hedging, liquid stable allocation, custody availability, and operational controls.
- Direct mint/redeem is permissioned for approved market makers; access control and signature/order validation are therefore core security surfaces.
- sUSDe security depends on correct staking share accounting, reward distribution, and withdrawal/cooldown mechanics.
- External AMM liquidity supports permissionless access but introduces market and integration risk outside the direct mint/redeem path.
- Liquid stable exposure may introduce third-party token risks such as freezing, blacklisting, non-standard transfer behavior, or depeg scenarios.
- The provided source material references broader risks, custody, oracle, hedging, reserve fund, and audits pages, but their contents were not included in the prompt and should not be inferred.

## Immunefi Smart Contract Program Rules

Program overview:

- Maximum bounty: `$3,000,000`
- Reward token: USDC on Ethereum
- Proof of Concept: required
- Primacy rule: primacy of impact

Smart contract rewards:

- Critical: `$100,000` to `$3,000,000`
- High: `$10,000` to `$75,000`
- Medium: fixed `$10,000`
- Low: fixed `$2,500`

Prohibited activities:

- Testing on mainnet or public testnet deployed code; testing must be done on local forks of public testnet or mainnet.
- Testing with pricing oracles or third-party smart contracts.
- Phishing or social engineering against employees or customers.
- Testing third-party systems and applications, including browser extensions, SSO providers, advertising networks, and websites.
- Denial-of-service attacks against project assets.
- Automated service testing that generates significant traffic.
- Public disclosure of an unpatched vulnerability in an embargoed bounty.
- Any other actions prohibited by Immunefi rules.

## Auditor Focus Areas

- `EthenaMinting.sol`: authorization, order structure, asset validation, replay protection, nonce handling, expiry, custody routing, mint/redeem limits, slippage or price assumptions, and emergency controls.
- `USDe.sol`: mint/burn authority, role separation, ERC20 behavior, supply invariants, upgrade or admin assumptions if any.
- `StakedUSDe.sol` and `StakedUSDeV2.sol`: share accounting, reward accrual, cooldown/withdrawal behavior, donation and rounding attacks, first-depositor behavior, and migration/version differences.
- `StakingRewardsDistributor.sol`: reward funding, distribution timing, privileged withdrawals, front-running, and accounting drift.
- `USDeSilo.sol`: ownership of escrowed or cooldown funds, release conditions, double-claim prevention, and access control.
- `EthenaLPStaking.sol`: stake accounting, reward allocation, emergency withdrawal behavior, and external token assumptions.
- `SingleAdminAccessControl.sol`: admin transfer, role grants/revocations, single-admin failure modes, and privilege escalation paths.
- Interfaces: confirm implementation assumptions against non-standard ERC20/WETH behaviors where assets cross trust boundaries.

### bbp-public-assets-immunefi-bounty-rules.md

# Immunefi Bounty Rules - Ethena

Use this file as mandatory context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/ethena/information/
- Scope: https://immunefi.com/bug-bounty/ethena/scope/
- Resources: https://immunefi.com/bug-bounty/ethena/resources/

## Program Requirements

- Proof of Concept: required
- Primacy: primacy_of_impact
- Rewards token: USDC
- Rewards token network: Ethereum
- Maximum bounty: $3000000

## Assets In Scope

> Smart Contract category only. Web & App assets are intentionally excluded.

- smart contract: https://etherscan.io/address/0x4c9edd5852cd905f086c759e8383e09bff1e68b3
  - Description: USDe.sol
- smart contract: https://etherscan.io/address/0xe3490297a08d6fC8Da46Edb7B6142E4F461b62D3#code
  - Description: EthenaMinting.sol V2
- smart contract: https://etherscan.io/address/0x9d39a5de30e57443bff2a8307a4256c8797a3497
  - Description: StakedUSDe.sol
- smart contract: https://etherscan.io/address/0x9d39a5de30e57443bff2a8307a4256c8797a3497
  - Description: StakedUSDeV2.sol
- smart contract: https://etherscan.io/address/0x7FC7c91D556B400AFa565013E3F32055a0713425
  - Description: USDeSilo.sol
- smart contract: https://etherscan.io/address/0x2cc440b721d2cafd6d64908d6d8c4acc57f8afc3
  - Description: SingleAdminAccessControl.sol
- smart contract: https://etherscan.io/address/0x8707f238936c12c309bfc2B9959C35828AcFc512
  - Description: EthenaLPStaking.sol. Present on both Ethereum and Mantle: https://explorer.mantle.xyz/address/0xf2fa332bD83149c66b09B45670bCe64746C6b439?tab=contract
- smart contract: https://etherscan.io/address/0xf2fa332bd83149c66b09b45670bce64746c6b439#tokentxns
  - Description: StakingRewardsDistributor.sol
- smart contract: https://etherscan.io/address/0x57e114B691Db790C35207b2e685D4A43181e6061
  - Description: ENA.sol
- smart contract (Primacy of Impact placeholder): https://immunefi.com/
  - Description: Primacy of Impact
- smart contract: https://etherscan.io/address/0x58538e6a46e07434d7e7375bc268d3cb839c0133
  - Description: ENAOFTAdapter.sol
- smart contract: https://etherscan.io/address/0x5d3a1ff2b6bab83b63cd9ad0787074081a52ef34
  - Description: USDeOFTAdapter.sol
- smart contract: https://etherscan.io/address/0x211cc4dd073734da055fbf44a2b4667d5e5fe5d2
  - Description: StakedUSDeOFTAdapter.sol
- smart contract: https://explorer.mantle.xyz/address/0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34
  - Description: USDeOFT.sol - Present on the following chains: Mantle, Arbitrum One, Manta Pacific, Optimism, BNB, Kava, Scroll, Mode, Metis, Fraxtal, Linea, and some others updated here: https://docs.ethena.fi/solution-design/key-addresses
- smart contract: https://explorer.mantle.xyz/address/0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2
  - Description: StakedUSDeOFT.sol - Present on the following chains: Mantle, Arbitrum One, Manta Pacific, Optimism, BNB, Kava, Scroll, Mode, Metis, Fraxtal, Linea, and some others updated here: https://docs.ethena.fi/solution-design/key-a
- smart contract: https://explorer.mantle.xyz/address/0x58538e6A46E07434d7E7375Bc268D3cb839C0133
  - Description: ENAOFT.sol - Present on the following chains: Mantle, Arbitrum One, Manta Pacific, Optimism, BNB, Kava, Scroll, Mode, Metis, Fraxtal, Linea, and some others updated here: https://docs.ethena.fi/solution-design/key-addresses
- smart contract: https://etherscan.io/address/0x8bE3460A480c80728a8C4D7a5D5303c85ba7B3b9#code
  - Description: StakedENA.sol
- smart contract: https://etherscan.io/address/0xc139190f447e929f090edeb554d95abb8b18ac1c#code
  - Description: USDtb.sol
- smart contract: https://etherscan.io/address/0xa3DDBf92077b850E29C4805Df0a2459Ae048416a
  - Description: USDtbMinting.sol
- smart contract: https://tonviewer.com/EQAIb6KmdfdDR7CN1GBqVJuP25iCnLKCvBlJ07Evuu2dzP5f
  - Description: USDe minter contract on TON
- smart contract: https://tonviewer.com/EQDQ5UUyPHrLcQJlPAczd_fjxn8SLrlNQwolBznxCdSlfQwr
  - Description: tsUSDe minter contract on TON
- smart contract: https://tonviewer.com/EQChGuD1u0e7KUWHH5FaYh_ygcLXhsdG2nSHPXHW8qqnpZXW
  - Description: tsUSDe vault on TON
- smart contract: https://tonviewer.com/EQAjpnYUX43uNjL3IqrFA5LyLPC0vo9iTgOCeab1AF-2aYcq
  - Description: USDe OFT contract on TON

## Impacts In Scope

- low (smart contract): Contract fails to deliver promised returns, but doesn't lose value
- high (smart contract): Theft of unclaimed yield
- high (smart contract): Theft of unclaimed royalties
- high (smart contract): Permanent freezing of unclaimed yield
- high (smart contract): Permanent freezing of unclaimed royalties
- high (smart contract): Temporary freezing of funds
- medium (smart contract): Smart contract unable to operate due to lack of token funds
- medium (smart contract): Block stuffing
- medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- medium (smart contract): Theft of gas
- medium (smart contract): Unbounded gas consumption
- critical (smart contract): Manipulation of governance voting result deviating from voted outcome and resulting in a direct change from intended effect of original results
- critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- critical (smart contract): Permanent freezing of funds
- critical (smart contract): Protocol insolvency

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



### bbp-public-assets-immunefi-severity-rubric.md

# Immunefi Severity Rubric - Ethena

Use this file as mandatory severity context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/ethena/information/
- Scope: https://immunefi.com/bug-bounty/ethena/scope/
- Resources: https://immunefi.com/bug-bounty/ethena/resources/
- Immunefi severity system v2.3: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-3/

## Program-Specific Severity Source Of Truth

- Primacy: primacy_of_impact
- Proof of Concept: required

## Impacts In Scope

- low (smart contract): Contract fails to deliver promised returns, but doesn't lose value
- high (smart contract): Theft of unclaimed yield
- high (smart contract): Theft of unclaimed royalties
- high (smart contract): Permanent freezing of unclaimed yield
- high (smart contract): Permanent freezing of unclaimed royalties
- high (smart contract): Temporary freezing of funds
- medium (smart contract): Smart contract unable to operate due to lack of token funds
- medium (smart contract): Block stuffing
- medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- medium (smart contract): Theft of gas
- medium (smart contract): Unbounded gas consumption
- critical (smart contract): Manipulation of governance voting result deviating from voted outcome and resulting in a direct change from intended effect of original results
- critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- critical (smart contract): Permanent freezing of funds
- critical (smart contract): Protocol insolvency

## Rewards By Threat Level

- critical (smart contract) [range]: min $100000, max $3000000
- high (smart contract) [range]: min $10000, max $75000
- medium (smart contract) [fixed]: fixed $10000
- low (smart contract) [fixed]: fixed $2500

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



### bbp-public-assets-immunefi-poc-runtime.md

# Immunefi PoC Runtime - Ethena

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
| StakedUSDeOFTAdapter.sol | smart_contract | `0x211cc4dd073734da055fbf44a2b4667d5e5fe5d2` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x211cc4dd073734da055fbf44a2b4667d5e5fe5d2 |
| SingleAdminAccessControl.sol | smart_contract | `0x2cc440b721d2cafd6d64908d6d8c4acc57f8afc3` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x2cc440b721d2cafd6d64908d6d8c4acc57f8afc3 |
| USDe.sol | smart_contract | `0x4c9edd5852cd905f086c759e8383e09bff1e68b3` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x4c9edd5852cd905f086c759e8383e09bff1e68b3 |
| ENA.sol | smart_contract | `0x57e114B691Db790C35207b2e685D4A43181e6061` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x57e114B691Db790C35207b2e685D4A43181e6061 |
| ENAOFTAdapter.sol | smart_contract | `0x58538e6a46e07434d7e7375bc268d3cb839c0133` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x58538e6a46e07434d7e7375bc268d3cb839c0133 |
| USDeOFTAdapter.sol | smart_contract | `0x5d3a1ff2b6bab83b63cd9ad0787074081a52ef34` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x5d3a1ff2b6bab83b63cd9ad0787074081a52ef34 |
| USDeSilo.sol | smart_contract | `0x7FC7c91D556B400AFa565013E3F32055a0713425` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x7FC7c91D556B400AFa565013E3F32055a0713425 |
| EthenaLPStaking.sol. Present on both Ethereum and Mantle: https://explorer.mantle.xyz/address/0xf2fa332bD83149c66b09B45670bCe64746C6b439?tab=contract | smart_contract | `0x8707f238936c12c309bfc2B9959C35828AcFc512` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x8707f238936c12c309bfc2B9959C35828AcFc512 |
| StakedENA.sol | smart_contract | `0x8bE3460A480c80728a8C4D7a5D5303c85ba7B3b9` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x8bE3460A480c80728a8C4D7a5D5303c85ba7B3b9#code |
| StakedUSDe.sol | smart_contract | `0x9d39a5de30e57443bff2a8307a4256c8797a3497` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0x9d39a5de30e57443bff2a8307a4256c8797a3497 |
| USDtbMinting.sol | smart_contract | `0xa3DDBf92077b850E29C4805Df0a2459Ae048416a` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xa3DDBf92077b850E29C4805Df0a2459Ae048416a |
| USDtb.sol | smart_contract | `0xc139190f447e929f090edeb554d95abb8b18ac1c` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xc139190f447e929f090edeb554d95abb8b18ac1c#code |
| EthenaMinting.sol V2 | smart_contract | `0xe3490297a08d6fC8Da46Edb7B6142E4F461b62D3` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xe3490297a08d6fC8Da46Edb7B6142E4F461b62D3#code |
| StakingRewardsDistributor.sol | smart_contract | `0xf2fa332bd83149c66b09b45670bce64746c6b439` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xf2fa332bd83149c66b09b45670bce64746c6b439#tokentxns |

## Network RPC Availability

| Network | Kind | RPC Env Var | Env Available | Asset Count |
| --- | --- | --- | --- | --- |
| `ethereum-mainnet` | `mainnet` | `MAINNET_RPC_URL` | `true` | `14` |

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

 ### contracts/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.7.3",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### contracts/lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.5",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### contracts/lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.5",
  "files": [
    "/contracts/**/*.sol",
    "/build/contracts/*.json",
    "!/contracts/mocks/**/*"


 ------------ ## CONFIG FILES ------------ 

 *Note*: Check for important package version info.

 