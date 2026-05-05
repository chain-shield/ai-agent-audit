
 ------------ ## PROTOCOL OVERVIEW ------------ 

# SSV Network Protocol Summary

SSV Network is a Distributed Validator Technology protocol for Ethereum staking. Its goal is to let a validator owner avoid dependence on a single operator by assigning validator duties to a cluster of multiple independent operators. In the scope provided here, the Solidity contracts are not the full validator-execution system; they are the onchain registry, coordination, and accounting layer that supports validator and operator relationships.

## What The In-Scope Contracts Are

### `SSVNetwork.sol`
This is the primary state-changing contract for the protocol’s onchain network layer. Based on the provided context, it is the contract integrations use to interact with validator and operator management. The documentation strongly suggests this contract is responsible for the core write-paths around:

- validator onboarding into the SSV network
- operator selection and cluster formation
- cluster and validator accounting
- operator fee handling
- funding, balance tracking, and potential liquidation logic
- permission checks for who can register, update, remove, or withdraw

The prompt does not include function bodies or ABI details, so those mechanics cannot be stated as facts beyond this architectural role.

### `SSVNetworkViews.sol`
This is the companion read-only contract. Its role is to expose protocol state for offchain systems such as frontends, SDKs, automation, monitoring, or operator-selection tools. Even though it does not mutate state, it is security-relevant because external integrations may rely on it for funding decisions, liquidation awareness, or validator/operator status.

## Protocol Purpose

The protocol sits between two major domains:

1. Ethereum staking itself, where a validator must still be activated with a separate 32 ETH deposit into Ethereum’s Deposit Contract.
2. SSV’s operator network, where validator duties are distributed across multiple operators rather than entrusted to one party.

The in-scope contracts appear to provide the onchain source of truth for which validators are associated with which operators, what balances support that relationship, and how operator compensation is accounted for.

In other words, these contracts are not the validator client and do not themselves perform consensus duties. They instead provide the economic and registry substrate that lets offchain operators coordinate around a validator in a structured way.

## Main Actors

### Stakers / Validator Owners
These users want a validator operated through SSV’s distributed model. They likely register validator-related data, choose operators, and fund the protocol-side accounting needed to keep operator service active. They also remain responsible for the separate 32 ETH Ethereum deposit flow.

### Operators
Operators run infrastructure that performs validator duties as part of a cluster. The contracts likely record operator registration and fee parameters, and they likely define how operators become associated with validators and how they are compensated.

### Operator Clusters
A cluster is the logical grouping of operators assigned to a validator or set of validators. Cluster correctness is a core security property because errors in identity, membership, or accounting could misroute fees, break liveness assumptions, or let an attacker alter who is considered responsible for a validator.

### Governance / DAO-Related Entities
The documentation mentions DAO treasury and foundation addresses, but does not define their exact permissions over the scoped contracts. Any admin or governance authority present in code is therefore a major trust boundary that must be validated directly from Solidity.

### External Ethereum Infrastructure
The Ethereum Deposit Contract is a required dependency for validator activation, but it is outside the SSV contract scope. The prompt also identifies SSV Token and cSSV Token as related ecosystem contracts that may matter if the scoped contracts transfer or account for them.

## How The Protocol Likely Works

From the supplied documentation, the protocol can be understood as an onchain coordination and payment layer around distributed validator operations.

### 1. Validator Onboarding
A validator owner decides to use SSV Network and assigns the validator to a set of operators. The validator’s actual Ethereum activation is still done separately through the Deposit Contract with 32 ETH. The SSV contracts likely store the validator’s SSV-side registration, cluster membership, and funding state.

### 2. Operator Selection And Cluster Formation
The validator is associated with a cluster of operators. The contracts likely encode some canonical notion of cluster identity based on operator membership. That identity is important because accounting, fee accrual, liquidation status, and ownership checks may all depend on it.

### 3. Funding And Fee Accounting
Operators earn fees for servicing validators. The protocol likely maintains balances that are prepaid or reserved to cover operator compensation over time. A core part of the design is therefore expected to be internal accounting that tracks:

- balances attributable to validator owners or clusters
- fee obligations owed to operators
- state transitions when balances are topped up, consumed, or withdrawn
- conditions under which underfunded entities become liquidatable or inactive

The exact fee formula is not described in the prompt, so only the architectural intent can be summarized.

### 4. Read Paths For Integrations
External systems likely query `SSVNetworkViews` to learn whether a validator or cluster is healthy, funded, active, liquidatable, or otherwise usable. This contract probably aggregates or formats state stored in `SSVNetwork` into integration-friendly outputs.

### 5. Potential Rewards / Treasury Interaction
The broader ecosystem includes token and treasury contracts, plus a rewards distributor. The prompt does not confirm whether the scoped contracts actively call them, but they remain relevant external dependencies if referenced by the in-scope code.

## Value And Accounting Model

The provided material indicates that SSV token economics are part of the network’s operational model. Even without function-level detail, the accounting design appears to revolve around these ideas:

- validator owners fund participation in the network
- operators receive fees for providing service
- internal balances must stay synchronized with actual token balances held or referenced by the protocol
- state may depend on whether balances are sufficient to maintain active operation

This means the protocol is economically sensitive even if it is not directly custodying ETH staking deposits. If the accounting layer is wrong, users could lose funds, operators could be underpaid or overpaid, clusters could be liquidated incorrectly, or integrations could make unsafe operational decisions.

## Trust Boundaries

Several trust boundaries are explicit in the supplied context.

### Onchain vs Offchain
The contracts can store registry data and economic state, but they cannot by themselves guarantee correct validator duty execution unless there is explicit reporting, incentive, or slashing logic. Offchain operator performance is therefore partly outside direct EVM enforcement.

### User-Supplied Inputs
Validator owners may supply validator metadata, operator selections, cluster information, or onboarding parameters. Those inputs must be treated as adversarial unless validated.

### Operator-Supplied Inputs
Operators may register themselves, set fees, or expose metadata. Their inputs are also adversarial unless constrained.

### Governance / Admin Power
Any owner, DAO, treasury, or admin role can materially alter the protocol’s trust model if it can pause, seize, reconfigure, whitelist, or otherwise bypass normal accounting and permission rules.

### External Token Behavior
If `SSVNetwork` interacts with SSV or cSSV tokens, safety depends on handling ERC20 edge cases correctly and not assuming ideal token behavior without guards.

### View Consumers
Because frontends and automation may depend on `SSVNetworkViews`, inaccurate or stale view outputs can create indirect security risk even without direct state mutation.

## Security-Critical Properties

The prompt identifies the main assumptions that must hold for the protocol to work safely. These are effectively the protocol’s security invariants at a high level:

- cluster membership and cluster identity must be correct and unforgeable
- validator ownership and authorization must be enforced on every sensitive path
- deposits, balances, fees, and withdrawals must remain internally consistent
- fee calculations must be deterministic and resistant to drift or abuse
- liquidation or inactivity logic must trigger only under correct conditions
- views must reflect mutating-state reality accurately enough for integrations
- governance powers, if any, must be explicit and bounded by the trust model
- token interactions must not break on non-ideal ERC20 behavior
- alternate registration or permission paths must not bypass intended controls

These properties matter because the protocol is effectively a registry-plus-accounting system. In that class of protocol, many critical failures come not from exotic cryptography but from bad authorization, broken accounting, inconsistent read/write logic, or unsafe lifecycle transitions.

## Security Posture Of The In-Scope Surface

From the prompt alone, the most important risk areas are:

- authorization around validator and cluster management
- operator fee and balance accounting
- liquidation or underfunding state transitions
- consistency between `SSVNetwork` state and `SSVNetworkViews` outputs
- interactions with SSV/cSSV or other external contracts
- any governance or privileged override capabilities

This is especially important because external systems may automate around these contracts. A flaw in write logic can directly affect balances or availability, while a flaw in view logic can indirectly cause damaging operational mistakes.

## What This Protocol Is, In One Sentence

SSV Network’s scoped Solidity layer is the onchain registry and accounting backbone that lets Ethereum validator owners assign validators to distributed operator clusters, track the economic relationship between them, and expose that state to integrations.

## Practical Mental Model

A concise way to think about the two scoped contracts is:

- `SSVNetwork.sol`: the state machine and ledger
- `SSVNetworkViews.sol`: the read interface for external consumers

Together, they represent the protocol’s authoritative onchain coordination layer for distributed validator operations, while the actual validator performance and Ethereum consensus duties occur offchain or in external Ethereum infrastructure.

## Scope Limitation

This summary is constrained to the prompt content. The supplied documentation explicitly says that function-level mechanics, role definitions, fee formulas, lifecycle details, and exact invariants were not included and must be confirmed from the Solidity implementation itself. So this summary should be treated as an architectural explanation of the protocol, not a verified line-by-line description of contract behavior.


 ------------ ## Main List of Files in Project ------------ 

contracts/SSVNetwork.sol
contracts/SSVNetworkViews.sol


 ------------ ## DOCUMENTATION: ------------ 

 ### ssv-network-docs.md

# SSV Network Audit Context

Artifact prefix: `ssv-network`  
Repository: `ssv-network`  
Commit: `9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9`

## In-Scope Contracts

- `./contracts/SSVNetwork.sol`
- `./contracts/SSVNetworkViews.sol`

## Protocol Summary

SSV Network is a Distributed Validator Technology (DVT) network for Ethereum validators. Its stated purpose is to provide reusable staking infrastructure that decentralizes validator operations across multiple non-trusting operator nodes. Validator duties are performed by operator clusters on behalf of stakers, reducing reliance on a single validator operator.

For this audit, the Solidity focus is the onchain registry/accounting layer around SSV Network operations, not the full offchain validator client or Ethereum consensus-layer behavior. The source material identifies `SSVNetwork` as the main network contract and `SSVNetworkViews` as the companion views contract.

Sources: Immunefi program overview; SSV docs home; SSV smart-contract docs.

## Main Actors

- **Stakers / validator owners:** Use SSV Network to distribute Ethereum validator operations across operators. The docs imply they must separately fund the Ethereum validator deposit with 32 ETH through the Ethereum Deposit Contract.
- **Operators:** Node operators that participate in clusters and perform validator operations. They earn fees for securing validator operations.
- **Operator clusters:** Groups of operators assigned to operate validators on behalf of stakers.
- **SSV DAO / governance-related entities:** The docs list an SSV DAO Treasury and SSV DAO Foundation. The source material does not specify their permissions over the in-scope contracts.
- **SSV Grants Committee:** Relevant to bounty administration and disclosure approval, not necessarily protocol execution.
- **Ethereum Deposit Contract:** External Ethereum staking dependency used to activate validators with 32 ETH. It is not an SSV contract.

## Core Architecture

### `SSVNetwork.sol`

`SSVNetwork` is the primary smart contract named by the official docs and the Immunefi scope. Based on available documentation, it represents the onchain SSV Network contract used by integrations to interact with the protocol’s validator/operator system. The fetched docs do not include function-level mechanics for this contract, so auditor assumptions about registration, cluster accounting, fee settlement, liquidation, or permission checks must be verified directly against the code.

Mainnet deployment listed by docs:

- `SSV Network`: `0xDD9BC35aE942eF0cFa76930954a156B3fF30a4E1`
- ENS: `ssvnetwork.eth`

### `SSVNetworkViews.sol`

`SSVNetworkViews` is the official views contract. It likely exposes read-only protocol state for integrations and frontends, but the provided documentation does not describe its functions. Treat it as security-relevant because stale, inconsistent, or misleading view outputs can affect integrations, offchain automation, accounting estimates, and liquidation or funding decisions if external systems rely on them.

Mainnet deployment listed by docs:

- `SSV Network Views`: `0xafE830B6Ee262ba11cce5F32fDCd760FFE6a66e4`
- ENS: `views.ssvnetwork.eth`

### Token and Treasury Components

The smart-contract docs list these related mainnet contracts:

- `SSV Token`: `0x9D65fF81a3c488d585bBfb0Bfe3c7707c7917f54`, ENS `ssv.ssvnetwork.eth`
- `cSSV Token`: `0xe018D31F120A637828F46aFD6c64EC099d960546`, ENS `cssv.ssvnetwork.eth`
- `Mainnet Rewards Distributor`: `0xe16d6138b1d2ad4fd6603acdb329ad1a6cd26d9f`, ENS `v1-imp.ssvnetwork.eth`
- `SSV DAO Treasury`: `0xb35096b074fdb9bBac63E3AdaE0Bbde512B2E6b6`, ENS `treasury.ssvnetwork.eth`
- `SSV DAO Foundation`: `0xeC29418bc30FED20dE85706F32c7D77Da0be7afB`, ENS `foundation.ssvnetwork.eth`

Only `SSVNetwork.sol` and `SSVNetworkViews.sol` are in the machine-readable scope. Related token, rewards, treasury, and foundation contracts are still relevant as external dependencies or privileged addresses if referenced by scoped code.

## Main Flows To Understand In Code

The provided documentation is high level and does not enumerate contract methods. The following flows are security-review priorities inferred from the documented protocol role, and must be validated against the in-scope Solidity implementation:

- **Validator onboarding:** Stakers integrate with SSV Network to onboard validators and distribute validator operation across operators. Separately, validator activation requires sending 32 ETH to the Ethereum Deposit Contract.
- **Operator selection / cluster formation:** Validators are operated by clusters of operator nodes. Security depends on correct cluster identity, operator membership, and any constraints around who may register, update, or remove cluster/operator data.
- **Fee and balance accounting:** Operators earn fees for securing validator operations. The in-scope contract likely tracks balances, fees, or cluster funding; exact mechanics are not described in the docs and require code review.
- **Views / integration reads:** `SSVNetworkViews` may expose data used by SDKs, apps, or automation. Read-path correctness matters if downstream systems make funding, liquidation, onboarding, or operator-selection decisions based on it.
- **Rewards and treasury interactions:** Docs list a rewards distributor and DAO treasury, but do not define whether they interact with the scoped contracts.

## Accounting and Value Flow

Assets and value-bearing flows identified from the source material:

- **SSV token:** Bounty rewards are paid in SSV, and the SSV token is listed as a mainnet contract. The docs imply SSV is part of the network economy, including operator compensation, but the fetched docs do not provide exact onchain fee formulas.
- **cSSV token:** Listed in official smart-contract docs, but mechanics are not explained in the provided source material.
- **ETH validator deposits:** Ethereum validators require 32 ETH sent to the Ethereum Deposit Contract. This is an external activation flow, not shown as handled by SSV contracts in the provided docs.
- **Operator fees:** Operators earn fees for validator operations. The source material does not specify whether fees are prepaid, streamed, charged per block/epoch, liquidated on underfunding, or distributed through a separate rewards distributor.
- **Bounty vault:** Immunefi lists a public vault address and SSV-denominated rewards. This is bounty infrastructure, not protocol accounting.

Security review should derive exact value conservation rules from code: token transfers in/out, internal balances, fee accrual, cluster liquidation/exit behavior, rounding, precision, withdrawal authority, and invariants between stored balances and actual token balances.

## External Integrations and Dependencies

- **Ethereum Deposit Contract:** Official docs state 32 ETH must be sent to activate a validator. Mainnet and Hoodi testnet address shown in docs: `0x00000000219ab540356cBB839Cbe05303d7705Fa`. The docs advise verifying against Ethereum official documentation and Launchpad guidance because phishing risk is material.
- **SSV offchain operator network:** The protocol relies on operator nodes performing validator duties. Solidity can register/account/coordinate, but offchain liveness and correct duty performance are outside direct EVM enforcement unless explicitly represented in contract logic.
- **SSV SDK, API, Explorer, Web App:** Official docs reference these integration surfaces, but they are outside the machine-readable Solidity scope. Their reliance on views or contract events can still influence threat modeling.
- **Mainnet Rewards Distributor / DAO addresses:** Listed by docs. Treat as external contracts/accounts if referenced by scoped code.
- **Token contracts:** `SSV Token` and `cSSV Token` are listed by docs. If scoped contracts transfer or account for these tokens, token behavior, approvals, and transfer return handling are relevant.

## Trust Boundaries

- **Onchain vs offchain:** Solidity cannot by itself ensure that operators perform validator duties correctly unless the contract includes an enforceable reporting/slashing/accounting mechanism. The source material only states that non-trusting operators operate validators.
- **Staker-controlled inputs:** Validator onboarding data, operator selections, keyshares, or cluster metadata should be treated as untrusted unless validated by contract logic. The fetched docs did not include keyshare format details.
- **Operator-controlled inputs:** Operator registration, fee settings, or metadata should be treated as adversarial unless constrained.
- **Governance / DAO / treasury:** The docs identify DAO treasury/foundation addresses but do not specify authority. Any owner/admin/governance roles in code are high-priority trust assumptions.
- **External contract calls:** Token transfers, reward distributor calls, whitelist contracts, deposit-contract assumptions, or upgrade/admin hooks are trust-boundary points requiring code validation.
- **View consumers:** `SSVNetworkViews` is a trust boundary for offchain users. Incorrect reads can lead to unsafe external decisions even if they do not directly mutate state.

## Security-Relevant Assumptions To Validate

The documentation supports only high-level assumptions. The following must be confirmed from Solidity and tests:

- Operator clusters are correctly formed, uniquely identified, and cannot be hijacked or misaccounted.
- Validator ownership/authorization checks prevent unauthorized registration, removal, fee withdrawal, liquidation, or metadata updates.
- Internal balances and token balances stay synchronized across deposits, withdrawals, fee accrual, and operator payments.
- Fee calculation is deterministic, bounded, and resistant to rounding drift, stale parameters, or griefing.
- View functions return state consistent with mutating functions and cannot mislead integrations about balances, cluster health, liquidation state, or operator status.
- Admin or DAO-controlled parameters cannot unexpectedly confiscate funds, bypass accounting, or brick user operations unless documented as trusted governance behavior.
- External token interactions are safe for the actual SSV/cSSV token behavior and do not assume nonstandard ERC20 behavior unless guarded.
- Any whitelisting, permissioning, or operator registration path cannot be bypassed through alternate entry points.
- Failure of offchain operators is either outside the contract threat model or explicitly handled through code-level incentives/penalties.

## Bounty and Known-Issue Context

Immunefi program terms relevant to audit triage:

- Maximum smart-contract bounty: `$250,000`.
- Critical smart-contract rewards: 10% of directly affected funds, minimum `$50,000`, maximum `$250,000`.
- High, Medium, Low fixed rewards: `$30,000`, `$10,000`, `$1,500`.
- Rewards are denominated in USD but paid in SSV on Ethereum.
- Proof of Concept is required for Critical, High, Medium, and Low smart-contract submissions.
- Primacy: `primacy_of_impact`.
- Repeatable attacks are rewarded only for the initial attack vector.
- Testing on mainnet or public testnet deployed code is prohibited; testing should use local forks.
- Testing with pricing oracles, third-party smart contracts, third-party systems/apps, or traffic-heavy automation is prohibited by the program text.
- Public disclosure requires SSV DAO Grants Committee authorization.

Known audits / exclusions listed by Immunefi:

- Quantstamp audit dated `2024-07-03`: `https://github.com/ssvlabs/ssv-network/tree/main/contracts/audits`
- Quantstamp audit dated `2026-02-25`: `https://github.com/ssvlabs/ssv-network/tree/mainnet-v2.0.0/contracts/audits`
- Vulnerabilities already identified in these audit reports, documented on `docs.ssv.network`, or documented within any branch of the specified repository are ineligible for reward according to the Immunefi program text.

## Source Notes

- Immunefi Information: `https://immunefi.com/bug-bounty/ssvnetwork/information/`
- Immunefi Resources: `https://immunefi.com/bug-bounty/ssvnetwork/resources/`
- SSV Smart Contracts docs: `https://docs.ssv.network/developers/smart-contracts`
- SSV Documentation home: `https://docs.ssv.network`

## Documentation Gaps

The fetched source bundle did not include detailed `SSVNetwork` or `SSVNetworkViews` function documentation, operator/cluster accounting formulas, role definitions, lifecycle state machines, or explicit invariants. Those details should be extracted from Solidity, ABI docs, tests, prior audit reports, and any deeper official contract pages if available in the audit workspace.

### ssv-network-immunefi-bounty-rules.md

# Immunefi Bounty Rules - SSV Network

Use this file as mandatory context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/ssvnetwork/information/
- Scope: https://immunefi.com/bug-bounty/ssvnetwork/scope/
- Resources: https://immunefi.com/bug-bounty/ssvnetwork/resources/

## Program Requirements

- Proof of Concept: required
- Primacy: primacy_of_impact
- Rewards token: SSV
- Rewards token network: Ethereum
- Maximum bounty: $250000

## Assets In Scope

- smart contract: https://etherscan.io/address/0xDD9BC35aE942eF0cFa76930954a156B3fF30a4E1
  - Description: SSV Network
- smart contract: https://etherscan.io/address/0xafE830B6Ee262ba11cce5F32fDCd760FFE6a66e4
  - Description: SSV Network View
- smart contract (Primacy of Impact placeholder): https://immunefi.com
  - Description: Primacy of Impact

## Impacts In Scope

- low (smart contract): Contract fails to deliver promised returns, but doesn't lose value
- high (smart contract): Theft of unclaimed yield
- high (smart contract): Permanent freezing of unclaimed yield
- high (smart contract): Temporary freezing of funds
- medium (smart contract): Block stuffing
- medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- medium (smart contract): Theft of gas
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

- Quantstamp (2024-07-03T00:00:00.000Z): https://github.com/ssvlabs/ssv-network/tree/main/contracts/audits
- Quantstamp (2026-02-25T00:00:00.000Z): https://github.com/ssvlabs/ssv-network/tree/mainnet-v2.0.0/contracts/audits

## Stage 1 Eligibility Rules

- A finding must affect an in-scope asset, unless the exact category and severity are covered by the program's Primacy of Impact rules.
- A finding must produce an impact listed in the program's Impacts in Scope.
- Exclude known issues, prior audit findings, documented accepted risks, closed duplicate reports, and program-specific OOS cases.
- Exclude cases requiring privileged access, leaked credentials, social engineering, malicious or mistaken trusted roles, deployment mistakes, test/mock files, public disclosure, or third-party-only failures.
- Do not perform final exploitability or severity scoring in stage 1; keep only when there is no decisive eligibility blocker.

## Link Handling

- Do not follow Immunefi navigation, marketing, login, social, newsletter, or platform-help links during validation.
- Use only the Source URLs above, in-scope explorer links, codebase links, documentation links, and prior-audit links when live verification is necessary.



### ssv-network-immunefi-severity-rubric.md

# Immunefi Severity Rubric - SSV Network

Use this file as mandatory severity context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/ssvnetwork/information/
- Scope: https://immunefi.com/bug-bounty/ssvnetwork/scope/
- Resources: https://immunefi.com/bug-bounty/ssvnetwork/resources/
- Immunefi severity system: not detected from page payload; use program impact rows first and verify the live bounty page before submission.

## Program-Specific Severity Source Of Truth

- Primacy: primacy_of_impact
- Proof of Concept: required

## Impacts In Scope

- low (smart contract): Contract fails to deliver promised returns, but doesn't lose value
- high (smart contract): Theft of unclaimed yield
- high (smart contract): Permanent freezing of unclaimed yield
- high (smart contract): Temporary freezing of funds
- medium (smart contract): Block stuffing
- medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- medium (smart contract): Theft of gas
- critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- critical (smart contract): Permanent freezing of funds
- critical (smart contract): Protocol insolvency

## Rewards By Threat Level

- critical (smart contract) [range]: min $50000, max $250000
- high (smart contract) [fixed]: fixed $30000
- medium (smart contract) [fixed]: fixed $10000
- low (smart contract) [fixed]: fixed $1500

## Platform Smart Contract Severity Summary

Always prefer the program's exact impact rows above. Use this summary only to interpret the referenced Immunefi severity system.

### v2.3 Smart Contract Summary

- Critical: direct theft of funds or NFTs, permanent freezing, protocol insolvency, governance result manipulation, unauthorized NFT minting, manipulable RNG abuse, or NFT representation alteration when listed by the program.
- High: theft or permanent freezing of unclaimed yield/royalties, temporary freezing of funds/NFTs, or other High rows listed by the program.
- Medium: griefing, block stuffing, gas theft, unbounded gas, or liveness failures only when listed by the program.
- Low/Insight: lower-impact failures only when listed and rewarded by the program.

The page did not expose a precise severity-system version in structured data; verify the live bounty page before final submission.

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



### ssv-network-immunefi-poc-runtime.md

# Immunefi PoC Runtime - SSV Network

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
| SSV Network | smart_contract | `0xDD9BC35aE942eF0cFa76930954a156B3fF30a4E1` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xDD9BC35aE942eF0cFa76930954a156B3fF30a4E1 |
| SSV Network View | smart_contract | `0xafE830B6Ee262ba11cce5F32fDCd760FFE6a66e4` | `ethereum-mainnet` | `MAINNET_RPC_URL` | `true` | https://etherscan.io/address/0xafE830B6Ee262ba11cce5F32fDCd760FFE6a66e4 |

## Network RPC Availability

| Network | Kind | RPC Env Var | Env Available | Asset Count |
| --- | --- | --- | --- | --- |
| `ethereum-mainnet` | `mainnet` | `MAINNET_RPC_URL` | `true` | `2` |

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

 ### node_modules/@types/mocha/package.json

{
    "name": "@types/mocha",
    "version": "10.0.10",
    "description": "TypeScript definitions for mocha",
    "homepage": "https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/mocha",
    "license": "MIT",
    "contributors": [
        {

### node_modules/@types/chai/package.json

{
    "name": "@types/chai",
    "version": "4.3.20",
    "description": "TypeScript definitions for chai",
    "homepage": "https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/chai",
    "license": "MIT",
    "contributors": [
        {

### node_modules/@types/node/package.json

{
    "name": "@types/node",
    "version": "22.19.2",
    "description": "TypeScript definitions for node",
    "homepage": "https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/node",
    "license": "MIT",
    "contributors": [
        {

### node_modules/@types/chai-as-promised/package.json

{
    "name": "@types/chai-as-promised",
    "version": "8.0.2",
    "description": "TypeScript definitions for chai-as-promised",
    "homepage": "https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/chai-as-promised",
    "license": "MIT",
    "contributors": [
        {

### node_modules/mocha/package.json

{
  "name": "mocha",
  "version": "11.7.5",
  "type": "commonjs",
  "description": "simple, flexible, fun test framework",
  "keywords": [
    "mocha",
    "test",

### node_modules/dotenv/package.json

{
  "name": "dotenv",
  "version": "17.2.3",
  "description": "Loads environment variables from .env file",
  "main": "lib/main.js",
  "types": "lib/main.d.ts",
  "exports": {
    ".": {

### node_modules/ethers/lib.commonjs/package.json

{
  "browser": {
    "./crypto/crypto.js": "./crypto/crypto-browser.js",
    "./providers/provider-ipcsocket.js": "./providers/provider-ipcsocket-browser.js",
    "./providers/ws.js": "./providers/ws-browser.js",
    "./utils/base64.js": "./utils/base64-browser.js",
    "./utils/geturl.js": "./utils/geturl-browser.js",
    "./wordlists/wordlists.js": "./wordlists/wordlists-browser.js"

### node_modules/ethers/package.json

{
  "author": "Richard Moore <me@ricmoo.com>",
  "browser": {
    "./lib.esm/crypto/crypto.js": "./lib.esm/crypto/crypto-browser.js",
    "./lib.esm/providers/provider-ipcsocket.js": "./lib.esm/providers/provider-ipcsocket-browser.js",
    "./lib.esm/providers/ws.js": "./lib.esm/providers/ws-browser.js",
    "./lib.esm/utils/base64.js": "./lib.esm/utils/base64-browser.js",
    "./lib.esm/utils/geturl.js": "./lib.esm/utils/geturl-browser.js",

### node_modules/ethers/lib.esm/package.json

{
  "browser": {
    "./crypto/crypto.js": "./crypto/crypto-browser.js",
    "./providers/provider-ipcsocket.js": "./providers/provider-ipcsocket-browser.js",
    "./providers/ws.js": "./providers/ws-browser.js",
    "./utils/base64.js": "./utils/base64-browser.js",
    "./utils/geturl.js": "./utils/geturl-browser.js",
    "./wordlists/wordlists.js": "./wordlists/wordlists-browser.js"

### node_modules/chai/package.json

{
  "author": "Jake Luer <jake@alogicalparadox.com>",
  "name": "chai",
  "type": "module",
  "description": "BDD/TDD assertion library for node.js and the browser. Test framework agnostic.",
  "keywords": [
    "test",
    "assertion",

### node_modules/tsx/package.json

{
  "name": "tsx",
  "version": "4.21.0",
  "description": "TypeScript Execute (tsx): Node.js enhanced with esbuild to run TypeScript & ESM files",
  "keywords": [
    "cli",
    "runtime",
    "node",

### node_modules/solhint/package.json

{
  "name": "solhint",
  "version": "5.2.0",
  "description": "Solidity Code Linter",
  "main": "lib/index.js",
  "keywords": [
    "solidity",
    "linter",

### node_modules/@nomicfoundation/hardhat-ignition/package.json

{
  "name": "@nomicfoundation/hardhat-ignition",
  "version": "3.0.6",
  "description": "Hardhat Ignition is a declarative system for deploying smart contracts on Ethereum. It enables you to define smart contract instances you want to deploy, and any operation you want to run on them. By taking over the deployment and execution, Hardhat Ignition lets you focus on your project instead of getting caught up in the deployment details.",
  "homepage": "https://hardhat.org",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/@nomicfoundation/hardhat-ethers-chai-matchers/package.json

{
  "name": "@nomicfoundation/hardhat-ethers-chai-matchers",
  "version": "3.0.2",
  "description": "Hardhat utils for testing",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat-ethers-chai-matchers",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/@nomicfoundation/hardhat-toolbox-mocha-ethers/package.json

{
  "name": "@nomicfoundation/hardhat-toolbox-mocha-ethers",
  "version": "3.0.2",
  "description": "Nomic Foundation's recommended bundle of Hardhat plugins",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat-toolbox-mocha-ethers",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/@nomicfoundation/hardhat-ethers/package.json

{
  "name": "@nomicfoundation/hardhat-ethers",
  "version": "4.0.3",
  "description": "Hardhat plugin for ethers",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat-ethers",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/@nomicfoundation/hardhat-verify/package.json

{
  "name": "@nomicfoundation/hardhat-verify",
  "version": "3.0.8",
  "description": "Hardhat plugin for verifying contracts",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat-verify",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/hardhat/package.json

{
  "name": "hardhat",
  "version": "3.1.0",
  "description": "Hardhat is an extensible developer tool that helps smart contract developers increase productivity by reliably bringing together the tools they want.",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/hardhat/templates/hardhat-3/03-minimal/package.json

{
  "name": "template-minimal",
  "private": true,
  "version": "0.0.1",
  "description": "A minimal Hardhat project",
  "type": "module",
  "devDependencies": {
    "hardhat": "workspace:^3.1.0",

### node_modules/hardhat/templates/hardhat-3/02-mocha-ethers/package.json

{
  "name": "template-mocha-ethers",
  "private": true,
  "version": "0.0.1",
  "description": "A TypeScript Hardhat project using Mocha and Ethers.js",
  "type": "module",
  "devDependencies": {
    "hardhat": "workspace:^3.1.0",

### node_modules/hardhat/templates/hardhat-3/01-node-test-runner-viem/package.json

{
  "name": "template-node-test-runner-viem",
  "private": true,
  "version": "0.0.1",
  "description": "A TypeScript Hardhat project using Node Test Runner and Viem",
  "type": "module",
  "devDependencies": {
    "hardhat": "workspace:^3.1.0",

### node_modules/hardhat/templates/hardhat-2/05-empty-hardhat-config-js/package.json

{
  "name": "template-v2-empty-hardhat-config-js",
  "private": true,
  "version": "0.0.1",
  "description": "An empty config file (hardhat.config.js)",
  "devDependencies": {
    "hardhat": "^2.14.0"
  }

### node_modules/hardhat/templates/hardhat-2/03-mocha-ethers-ts/package.json

{
  "name": "template-v2-mocha-ethers-ts",
  "private": true,
  "version": "0.0.1",
  "description": "A Typescript project using Mocha and Ethers.js",
  "devDependencies": {
    "@nomicfoundation/hardhat-chai-matchers": "^2.0.0",
    "@nomicfoundation/hardhat-ethers": "^3.0.0",

### node_modules/hardhat/templates/hardhat-2/04-mocha-viem-ts/package.json

{
  "name": "template-v2-mocha-viem-ts",
  "private": true,
  "version": "0.0.1",
  "description": "A Typescript project using Mocha and Viem",
  "devDependencies": {
    "@nomicfoundation/hardhat-ignition": "^0.15.0",
    "@nomicfoundation/hardhat-ignition-viem": "^0.15.0",

### node_modules/hardhat/templates/hardhat-2/02-mocha-ethers-js-esm/package.json

{
  "name": "template-v2-mocha-ethers-js-esm",
  "private": true,
  "version": "0.0.1",
  "description": "A Javascript project using Mocha and Ethers.js (ESM)",
  "type": "module",
  "devDependencies": {
    "@nomicfoundation/hardhat-chai-matchers": "^2.0.0",

### node_modules/hardhat/templates/hardhat-2/01-mocha-ethers-js/package.json

{
  "name": "template-v2-mocha-ethers-js",
  "private": true,
  "version": "0.0.1",
  "description": "A Javascript project using Mocha and Ethers.js",
  "devDependencies": {
    "@nomicfoundation/hardhat-chai-matchers": "^2.0.0",
    "@nomicfoundation/hardhat-ethers": "^3.0.0",

### node_modules/@openzeppelin/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.6",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### node_modules/@openzeppelin/contracts-upgradeable/package.json

{
  "name": "@openzeppelin/contracts-upgradeable",
  "description": "Secure Smart Contract library for Solidity",
  "version": "4.9.6",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"


 ------------ ## CONFIG FILES ------------ 

 *Note*: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "contracts"
test = "test"
out = "out"
libs = ["node_modules"]
auto_detect_solc = true
via_ir = true
optimizer = true
optimizer_runs = 10000
evm_version = "cancun"

remappings = [
    "@openzeppelin/=node_modules/@openzeppelin/"
]

[rpc_endpoints]
hoodi = "https://hoodi.infura.io/v3/{INFURA_KEY}"


### package.json

{
  "name": "ssv-network",
  "version": "2.0.0",
  "description": "Solidity smart contracts for the SSV Network",
  "author": "SSV.Network",
  "type": "module",
  "repository": {
    "type": "git",
    "url": "https://github.com/ssvlabs/ssv-network.git"
  },
  "license": "MIT",
  "keywords": [
    "ssv",
    "ssv.network",
    "solidity",
    "staking"
  ],
  "files": [
    "contracts/**/*.sol",
    "!contracts/**/deprecated/**",
    "!contracts/**/mocks/**",
    "!contracts/**/test/**",
    "!contracts/**/upgrades/**",
    "abis/CSSVToken.json",
    "abis/ISSVStaking.json",
    "abis/SSVClusters.json",
    "abis/SSVDAO.json",
    "abis/SSVNetwork.json",
    "abis/SSVNetworkViews.json",
    "abis/SSVOperators.json",
    "abis/SSVOperatorsWhitelist.json",
    "abis/SSVStaking.json",
    "abis/SSVToken.json",
    "abis/SSVValidators.json",
    "abis/SSVViews.json",
    "docs/",
    "README.md",
    "RELEASE_NOTES.md",
    "LICENSE",
    "CHANGELOG.md"
  ],
  "scripts": {
    "build": "npx hardhat compile",
    "test": "npx hardhat test",
    "test:gas": "npx hardhat test --gas-stats",
    "test:unit": "npx hardhat test test/unit/**/*.test.ts",
    "test:unit:gas": "npx hardhat test test/unit/**/*.test.ts --gas-stats",
    "test:integration": "npx hardhat test test/integration/*.test.ts",
    "test:e2e": "npx hardhat test test/e2e/**/*.test.ts",
    "test:integration:gas": "npx hardhat test test/integration/*.test.ts --gas-stats",
    "test-forked": "FORK_TESTING_ENABLED=true npx hardhat test test/forked/**/*.test.ts",
    "gas:report": "REPORT_GAS=true npx hardhat test",
    "gas:compare": "npx tsx scripts/gas-compare.ts",
    "gas:ci": "REPORT_GAS=true NO_GAS_ENFORCE=1 npx hardhat test && npx tsx scripts/gas-compare.ts",
    "lint": "eslint . --ext .ts",
    "lint:fix": "eslint --fix . --ext .ts",
    "solidity-coverage": "NO_GAS_ENFORCE=1 npx hardhat test --coverage",
    "slither": "npx hardhat compile --force && slither . --hardhat-ignore-compile --solc-remaps @openzeppelin=node_modules/@openzeppelin --filter-paths \"contracts/test/\" --exclude-informational --exclude-dependencies",
    "size-contracts": "npx hardhat size-contracts"
  },
  "devDependencies": {
    "@nomicfoundation/hardhat-ethers": "^4.0.3",
    "@nomicfoundation/hardhat-ethers-chai-matchers": "^3.0.2",
    "@nomicfoundation/hardhat-ignition": "^3.0.6",
    "@nomicfoundation/hardhat-toolbox-mocha-ethers": "^3.0.2",
    "@nomicfoundation/hardhat-verify": "^3.0.8",
    "@openzeppelin/contracts": "^4.9.6",
    "@openzeppelin/contracts-upgradeable": "^4.9.6",
    "@types/chai": "^4.3.20",
    "@types/chai-as-promised": "^8.0.2",
    "@types/mocha": "^10.0.10",
    "@types/node": "^22.19.2",
    "chai": "^5.3.3",
    "dotenv": "^17.2.3",
    "ethers": "^6.16.0",
    "hardhat": "^3.1.0",
    "mocha": "^11.7.5",
    "solhint": "^5.0.0",
    "tsx": "^4.19.0"
  }
}


