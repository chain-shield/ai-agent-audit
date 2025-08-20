# 2025 04 virtuals protocol - Findings Report
## Commit hash: 28e93273daec5a9c73c438e216dde04c084be452

## Protocol Overview 

**Virtuals Protocol** is a modular Solidity framework for launching and operating on-chain "Virtual Agents"—self-contained micro-ecosystems with their own token, DAO, NFT identity and revenue flow.  

1. Native Currency  
   • `Virtual` (ERC20) is the base token. Voting power is locked into non-transferable `veVirtualToken`, which feeds system-wide governance via `VirtualProtocolDAO` and fast-tracked `VirtualGenesisDAO`.  

2. Agent Creation  
   • Anyone stakes Virtual into `AgentFactoryV*` and submits an application.  
   • When a DAO proposal passes, the factory clones templates (`AgentToken`, `AgentDAO`, `AgentVeToken`) and mints an `AgentNftV2` that represents the persona. Optional Token-Bound Accounts (ERC-6551) are created for smart-wallet utility.  

3. Operations inside an Agent  
   • Holders stake the agent’s ERC20 into its veToken to gain voting power.  
   • Builders mint `ContributionNft` proposals; once accepted they mature into `ServiceNft`, updating the agent’s impact score.  
   • Daily income is funneled to `AgentRewardV2/V3`, which splits rewards among protocol, stakers, validators, model & dataset owners.  

4. DeFi & Treasury  
   • Swaps, liquidity and bonding-curve issuance use `FFactory`, `FRouter`, `FPair`, `Bonding` and taxation helpers (`AgentTax`, `BondingTax`, `LPRefund`).  

5. Auxiliary Tools  
   • `Airdrop`, `TokenSaver`, `EloCalculator`, bridging contracts, and upgradeable admin utilities round out the stack.  

Together these pieces let communities spin up fully-governed, revenue-sharing virtual personas with minimal code and maximum composability.
## High Risk Findings
[H-1]. Slippage Missing Or Insufficient issue found with High severity
[H-2]. Access Control issue found with High severity
[H-3]. Auth Bypass issue found with High severity
[H-4]. Access Control issue found with High severity
[H-5]. Accounting Invariant Violation issue found with High severity
[H-6]. Access Control issue found with High severity
[H-7]. Access Control issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. Access Control issue found with Medium severity
[M-3]. DOS issue found with Medium severity


### Number of Findings
- C: 0
- H: 7
- M: 3
- L: 0
- I: 0



