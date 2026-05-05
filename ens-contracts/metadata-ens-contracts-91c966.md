
 ------------ ## PROTOCOL OVERVIEW ------------ 

# ENS Contracts Protocol Summary

## What This Protocol Is

`ens-contracts` is the onchain core of the Ethereum Name Service (ENS), a decentralized naming system that maps human-readable names to blockchain identities, addresses, resolvers, and related metadata. In practical terms, it turns names like `.eth` domains and imported DNS names into programmable assets and lookup targets.

The protocol is built around a few core ideas:

- A **registry** is the canonical source of ownership and configuration for ENS names.
- **Registrars** issue and renew names in specific namespaces, most importantly `.eth`.
- **Resolvers** answer questions about a name, such as which address it points to.
- **Reverse resolution** maps addresses back to human-readable names.
- The **Name Wrapper** tokenizes names and adds restriction logic.
- **DNSSEC integration** allows DNS names to be proven and imported into ENS.
- **Universal/offchain resolution** supports richer resolution flows, including CCIP-read.

This makes ENS both an identity layer and a naming/ownership system. The contracts in scope cover issuance, renewal, delegation, metadata resolution, reverse records, DNS-backed claims, and governance-sensitive root control.

## Main Architecture

### 1. Registry Layer

The ENS registry is the protocol’s root authority. It tracks, for each namehash node:

- owner/controller
- resolver address
- TTL/config-like metadata
- subnode delegation relationships

Relevant contracts:

- `ENS.sol`
- `ENSRegistry.sol`
- `Root.sol`
- `Ownable.sol`
- `RootSecurityController.sol`

This layer defines who can control a name and which resolver is used to answer lookups. If registry ownership changes, effective control over the name usually changes with it.

### 2. Registrar Layer

Registrars manage specific namespaces and lifecycle rules. In this scope, the `.eth` registrar subsystem is central.

Relevant contracts:

- `BaseRegistrarImplementation.sol`
- `ETHRegistrarController.sol`
- `ExponentialPremiumPriceOracle.sol`
- `IPriceOracle.sol`
- `RegistrarSecurityController.sol`
- `StaticBulkRenewal.sol`

This layer handles:

- registration
- renewal
- expiry/grace-period logic
- payment collection
- premium pricing for recently expired names
- batch renewal flows

### 3. Resolver Layer

Resolvers provide the actual data a name resolves to.

Relevant contracts:

- `PublicResolver.sol`
- `Resolver.sol`
- `ExtendedDNSResolver.sol`
- `IAddrResolver.sol`
- `IAddressResolver.sol`
- `IExtendedResolver.sol`
- `INameResolver.sol`
- `UniversalResolver.sol`

A resolver can serve records such as:

- addresses
- reverse/name records
- DNS-style records
- extended/offchain responses

The registry says *which* resolver is authoritative for a name; the resolver provides the *content*.

### 4. Name Wrapper Layer

The Name Wrapper adds an NFT-like ownership and restriction model on top of ENS names.

Relevant contracts:

- `INameWrapper.sol`
- `NameWrapper.sol`
- `StaticMetadataService.sol`

Wrapped names carry extra state, including permission restrictions commonly described as **fuses**. This creates a second control plane beyond raw registry ownership.

### 5. Reverse Resolution Layer

Reverse records map an address back to a name for identity display.

Relevant contracts:

- `DefaultReverseRegistrar.sol`
- `IStandaloneReverseRegistrar.sol`
- `ReverseRegistrar.sol`
- `AbstractReverseResolver.sol`
- `ChainReverseResolver.sol`
- `DefaultReverseResolver.sol`
- `INameReverser.sol`

This is important because many apps display the reverse name as the user’s identity.

### 6. DNSSEC / DNS Import Layer

ENS can import or resolve DNS names when ownership is proven through DNSSEC.

Relevant contracts:

- `DNSRegistrar.sol`
- `OffchainDNSResolver.sol`
- `SimplePublicSuffixList.sol`
- `TLDPublicSuffixList.sol`
- `DNSSECImpl.sol`
- `SHA1.sol`
- `P256SHA256Algorithm.sol`
- `RSASHA1Algorithm.sol`
- `RSASHA256Algorithm.sol`
- `SHA1Digest.sol`
- `SHA256Digest.sol`

This subsystem verifies DNSSEC proof material and applies public suffix boundary logic to determine whether a DNS name can be represented in ENS.

### 7. CCIP-Read / Gateway Layer

Relevant contracts:

- `GatewayProvider.sol`
- `IGatewayProvider.sol`
- `IExtendedResolver.sol`

This supports offchain and cross-chain resolution patterns. It extends ENS beyond pure onchain storage by allowing verifiable offchain responses.

## How It Works

## Name Registration Flow

A typical `.eth` registration flow is:

1. A user interacts with `ETHRegistrarController`.
2. The controller checks availability and obtains pricing from the oracle.
3. The user pays the required fee for a chosen duration.
4. `BaseRegistrarImplementation` records ownership/expiry for the label.
5. ENS registry state and resolver settings may be configured.
6. The name may optionally be wrapped in `NameWrapper`.

The pricing system can include a premium for recently expired names, using `ExponentialPremiumPriceOracle`.

## Renewal and Expiry

ENS `.eth` names are time-bound assets. Renewal extends expiration. Expiry changes who can legitimately control or reclaim a name.

Important protocol expectations:

- the correct name must be extended by the correct duration
- payment must match current oracle pricing
- wrapper and registrar expiry state should remain synchronized
- stale control should not survive expiry through registry/resolver/wrapper inconsistencies

`StaticBulkRenewal.sol` allows renewing multiple names in one operation, which makes aggregate pricing and partial-failure handling important.

## Resolution Flow

A standard resolution path is:

1. A client computes the ENS node for a name.
2. The ENS registry is queried for that node’s resolver.
3. The resolver is called for the desired record type.
4. The returned data is interpreted by the client.

For more advanced resolution:

- `UniversalResolver.sol` composes registry and resolver lookup into a client-friendly path.
- extended resolvers can provide richer or nonstandard responses.
- CCIP-read can route part of the lookup offchain.

## Reverse Resolution Flow

A reverse record flow is roughly:

1. An address owner or authorized path calls a reverse registrar.
2. A reverse node is assigned/configured.
3. A reverse resolver stores the primary ENS name.
4. Apps query the reverse record to display the address’s claimed identity.

This is useful for UX, but reverse ownership alone is not the same as forward ownership verification.

## Name Wrapping Flow

Wrapping converts a name into a wrapped representation with additional permission logic.

This can involve:

- converting control from plain registry ownership to wrapped-token ownership
- setting fuse restrictions
- syncing expiry and control assumptions with registrar state
- managing subnames under wrapper-specific rules

The wrapper is one of the most security-sensitive parts of the system because it adds state and restrictions that must remain consistent with the base ENS registry and registrar lifecycle.

## DNS Name Import / DNSSEC Flow

For DNS-backed names:

1. A DNS name and proof chain are provided.
2. DNSSEC contracts validate signatures and digest chains.
3. Public suffix list logic determines valid registration boundaries.
4. If valid, ENS-side claim or resolution logic can proceed.

This subsystem depends on correct cryptographic verification, correct label parsing, and correct treatment of delegation and proof validity windows.

## Value Flow

The main economic flows in scope are:

- `.eth` registration fees
- `.eth` renewal fees
- expiry premium payments
- bulk renewal aggregate payments

These payments are security-critical because failure can cause:

- undercharging or overcharging
- incorrect refunds
- frozen registration fees
- name control mismatches after failed accounting

## Trust Boundaries

The protocol has several strong trust and control boundaries:

- **Registry ownership**: determines who controls a node.
- **Root/DAO/admin roles**: can have top-level namespace power.
- **Registrar/controller roles**: affect name issuance and lifecycle.
- **Resolver operators**: control what data names resolve to.
- **Name Wrapper authority**: governs fuse logic and wrapped control.
- **Gateway providers**: affect offchain lookup routing.
- **DNSSEC algorithm/digest configuration**: anchors DNS proof validity.
- **Public suffix list maintainers**: affect DNS boundary rules.

The provided documentation explicitly notes known governance-related risks around a malicious DAO upgrading the Name Wrapper or reducing expirations. Those are accepted known-issue context, not novel mechanics.

## Security-Critical Design Themes

Several invariants define whether the protocol behaves correctly:

- only authorized parties should be able to change name ownership, resolver, or records
- registration and renewal payments should match oracle output
- expiry transitions should be consistent across registrar, registry, and wrapper state
- wrapped-name restrictions should not be bypassable through direct registry operations or race conditions
- reverse records should not let unauthorized actors spoof identity
- offchain resolution must not let gateways or resolvers spoof unrelated names
- DNSSEC proofs must be validated precisely, including algorithm and boundary handling

## Known High-Risk Areas From Context

Based on the supplied materials, the highest-sensitivity areas are:

- `.eth` registration, renewal, premium, and expiry edge cases
- Name Wrapper fuse authorization, race conditions, and expiry sync
- registry and resolver authorization invariants
- root and governance control paths
- DNSSEC proof validation and public suffix boundary correctness
- CCIP-read/offchain response validation
- reverse resolution authorization and spoof resistance
- migration helper one-time-use and replay safety

## Practical Mental Model

A concise way to understand ENS is:

- the **registry** says who controls a name and where to resolve it
- the **registrar** determines how names are issued and renewed
- the **resolver** answers what a name means
- the **reverse system** answers which name an address claims
- the **wrapper** adds NFT-style control restrictions
- the **DNSSEC subsystem** bridges DNS into ENS
- the **universal/offchain layer** makes resolution more flexible across environments

Overall, `ens-contracts` is a modular naming and identity protocol whose correctness depends on tight synchronization between ownership, expiry, authorization, and resolution across several interlocking subsystems.


 ------------ ## Main List of Files in Project ------------ 

contracts/ccipRead/GatewayProvider.sol
contracts/ccipRead/IGatewayProvider.sol
contracts/dnsregistrar/DNSRegistrar.sol
contracts/dnsregistrar/OffchainDNSResolver.sol
contracts/dnsregistrar/SimplePublicSuffixList.sol
contracts/dnsregistrar/TLDPublicSuffixList.sol
contracts/dnssec-oracle/DNSSECImpl.sol
contracts/dnssec-oracle/SHA1.sol
contracts/dnssec-oracle/algorithms/P256SHA256Algorithm.sol
contracts/dnssec-oracle/algorithms/RSASHA1Algorithm.sol
contracts/dnssec-oracle/algorithms/RSASHA256Algorithm.sol
contracts/dnssec-oracle/digests/SHA1Digest.sol
contracts/dnssec-oracle/digests/SHA256Digest.sol
contracts/ethregistrar/BaseRegistrarImplementation.sol
contracts/ethregistrar/ETHRegistrarController.sol
contracts/ethregistrar/ExponentialPremiumPriceOracle.sol
contracts/ethregistrar/IPriceOracle.sol
contracts/ethregistrar/RegistrarSecurityController.sol
contracts/ethregistrar/StaticBulkRenewal.sol
contracts/registry/ENS.sol
contracts/registry/ENSRegistry.sol
contracts/resolvers/PublicResolver.sol
contracts/resolvers/Resolver.sol
contracts/resolvers/profiles/ExtendedDNSResolver.sol
contracts/resolvers/profiles/IAddrResolver.sol
contracts/resolvers/profiles/IAddressResolver.sol
contracts/resolvers/profiles/IExtendedResolver.sol
contracts/resolvers/profiles/INameResolver.sol
contracts/reverseRegistrar/DefaultReverseRegistrar.sol
contracts/reverseRegistrar/IStandaloneReverseRegistrar.sol
contracts/reverseRegistrar/ReverseRegistrar.sol
contracts/reverseResolver/AbstractReverseResolver.sol
contracts/reverseResolver/ChainReverseResolver.sol
contracts/reverseResolver/DefaultReverseResolver.sol
contracts/reverseResolver/INameReverser.sol
contracts/root/Ownable.sol
contracts/root/Root.sol
contracts/root/RootSecurityController.sol
contracts/universalResolver/UniversalResolver.sol
contracts/utils/MigrationHelper.sol
contracts/wrapper/INameWrapper.sol
contracts/wrapper/NameWrapper.sol
contracts/wrapper/StaticMetadataService.sol


 ------------ ## DOCUMENTATION: ------------ 

 ### ens-contracts-docs.md

# ENS Contracts Audit Context

Repository: `ens-contracts`  
Commit: `91c966febd7b55494269df830fc6775f040b927b`  
Scope: smart contracts only, from the provided machine-readable scope.

## Source Boundaries

Primary protocol documentation sources used:

- Immunefi ENS program overview: bounty criteria, known audits, prohibited testing constraints.
- Immunefi ENS resources: official codebase/docs links and known issues.
- ENS Documentation entry page: documentation map for protocol components, registries, resolvers, CCIP-read, Name Wrapper, and DAO/governance docs.

The supplied documentation content is an entry-level index, not full page text. The mechanics below are therefore limited to facts supported by the source bundle, contract names in scope, and generally implied component boundaries from the scoped contracts and documentation index. Where details are not in the supplied material, they are not expanded.

## Protocol Overview

ENS is a decentralized naming system for self-sovereign identity. It maps human-readable names to onchain and offchain resources through a registry/resolver architecture.

At a high level:

- The ENS registry is the authoritative source for ownership/control of name nodes and resolver configuration.
- Registrars manage issuance and renewal of names under specific namespaces such as `.eth`, reverse records, and DNS-imported names.
- Resolvers store or serve records such as addresses, names, text-like profile data, extended resolution data, and cross-chain/offchain lookup data.
- The Name Wrapper adds tokenized ownership and permission constraints over ENS names.
- DNSSEC and DNS registrar components allow DNS names to be proven and claimed using DNSSEC-validated records.
- Universal resolution components compose registry and resolver lookups for client-facing name resolution.

## In-Scope Contract Areas

### Registry and Root Control

Relevant scope:

- `contracts/registry/ENS.sol`
- `contracts/registry/ENSRegistry.sol`
- `contracts/root/Ownable.sol`
- `contracts/root/Root.sol`
- `contracts/root/RootSecurityController.sol`

Security-relevant mechanics:

- The registry is the core authority for name ownership and resolver assignment.
- Ownership of a node determines who can modify records or delegate subnodes, subject to registry and wrapper rules.
- Root contracts govern top-level namespace control and administrative permissions.
- Root-level permissions are a high-trust boundary because compromise or malicious governance action can affect large parts of the namespace.

Audit focus:

- Authorization checks for setting owners, subnode owners, resolvers, TTLs, and operators.
- Whether root/security-controller controls can unexpectedly pause, block, transfer, or mutate ownership.
- Upgrade/governance paths that can alter registry or wrapper assumptions.
- Invariants around node ownership: only authorized parties should be able to change ownership, resolver, or delegated control.

### `.eth` Registrar and Pricing

Relevant scope:

- `contracts/ethregistrar/BaseRegistrarImplementation.sol`
- `contracts/ethregistrar/ETHRegistrarController.sol`
- `contracts/ethregistrar/ExponentialPremiumPriceOracle.sol`
- `contracts/ethregistrar/IPriceOracle.sol`
- `contracts/ethregistrar/RegistrarSecurityController.sol`
- `contracts/ethregistrar/StaticBulkRenewal.sol`

Security-relevant mechanics:

- The `.eth` registrar manages registration and renewal of `.eth` names.
- The controller mediates user-facing registration flows.
- The price oracle determines registration/renewal pricing and premium decay behavior.
- Bulk renewal enables renewal operations across multiple names.
- Registrar security controls may constrain registration behavior.

Accounting/value flow:

- Users pay for registrations and renewals based on oracle pricing.
- Expiration and renewal accounting are central to name ownership continuity.
- Premium pricing introduces time-dependent value flow, especially after name expiry or release.

Audit focus:

- Registration commitment/reveal or anti-front-running assumptions if present in implementation.
- Correct payment calculation, refund handling, fee forwarding, and edge cases around overpayment/underpayment.
- Expiry boundary conditions: grace periods, renewal after expiry, premium decay, and reclaim behavior.
- Bulk renewal correctness across duplicate names, expired names, failed renewals, and price changes.
- Controller authorization and interaction with `BaseRegistrarImplementation`.

### Name Wrapper

Relevant scope:

- `contracts/wrapper/INameWrapper.sol`
- `contracts/wrapper/NameWrapper.sol`
- `contracts/wrapper/StaticMetadataService.sol`

Security-relevant mechanics:

- The Name Wrapper tokenizes ENS names and applies permission constraints, commonly referred to as fuses in ENS documentation and known-issue descriptions.
- Wrapped names introduce additional state and restrictions beyond raw registry ownership.
- Metadata service supports token metadata representation.

Known issues from Immunefi resources:

- A malicious DAO can steal names using an upgrade to the NameWrapper.
- A malicious DAO can reduce the expiration of names.
- A NameWrapper race condition can allow fuses to be set inappropriately.

Audit focus:

- Fuse-setting authorization and ordering.
- Expiry synchronization between wrapper state, registrar state, and registry ownership.
- Race conditions between wrapping, unwrapping, transferring, renewing, and setting restrictions.
- Whether wrapped-name constraints remain enforceable across resolver changes, subnode operations, and registrar interactions.
- Governance or upgrade authority over wrapper behavior, especially because known issues explicitly identify malicious DAO upgrade risk.

### Resolvers and Universal Resolution

Relevant scope:

- `contracts/resolvers/PublicResolver.sol`
- `contracts/resolvers/Resolver.sol`
- `contracts/resolvers/profiles/ExtendedDNSResolver.sol`
- `contracts/resolvers/profiles/IAddrResolver.sol`
- `contracts/resolvers/profiles/IAddressResolver.sol`
- `contracts/resolvers/profiles/IExtendedResolver.sol`
- `contracts/resolvers/profiles/INameResolver.sol`
- `contracts/universalResolver/UniversalResolver.sol`

Security-relevant mechanics:

- Resolvers provide records for ENS names, including address and name records.
- Public resolver contracts are shared infrastructure and must enforce record-level authorization.
- Extended resolver interfaces support more flexible resolution flows.
- Universal resolution combines registry lookup and resolver calls into a single resolution path.

Audit focus:

- Authorization for setting resolver records.
- Resolver behavior when registry ownership changes.
- Interface support and fallback behavior.
- Universal resolver handling of missing resolvers, malformed names, revert data, and resolver-controlled callbacks.
- Record integrity: unauthorized users should not be able to set or spoof address/name records for names they do not control.

### CCIP-Read / Offchain Resolution

Relevant scope:

- `contracts/ccipRead/GatewayProvider.sol`
- `contracts/ccipRead/IGatewayProvider.sol`
- `contracts/resolvers/profiles/IExtendedResolver.sol`

Security-relevant mechanics:

- ENS documentation includes cross-chain resolvers and CCIP-read as official resolver topics.
- Gateway provider contracts represent a trust and availability boundary for offchain lookup routes.
- Offchain responses must be validated according to resolver expectations; gateway availability and correctness are not equivalent to onchain authority.

Audit focus:

- Gateway configuration authority.
- Whether offchain lookup data can be spoofed, replayed, or routed to malicious gateways.
- Failure modes when gateways are unavailable or return malformed data.
- Resolver assumptions around chain IDs, sender validation, and response verification if present in implementation.

### Reverse Resolution

Relevant scope:

- `contracts/reverseRegistrar/DefaultReverseRegistrar.sol`
- `contracts/reverseRegistrar/IStandaloneReverseRegistrar.sol`
- `contracts/reverseRegistrar/ReverseRegistrar.sol`
- `contracts/reverseResolver/AbstractReverseResolver.sol`
- `contracts/reverseResolver/ChainReverseResolver.sol`
- `contracts/reverseResolver/DefaultReverseResolver.sol`
- `contracts/reverseResolver/INameReverser.sol`

Security-relevant mechanics:

- Reverse resolution maps an address or chain-specific address context back to a primary ENS name.
- Reverse records are identity-critical because applications may display them as user-facing identity.
- Reverse registrar and resolver contracts control who can set or resolve primary-name records.

Audit focus:

- Authorization for setting reverse records on behalf of an address or contract.
- Spoofing risks where a name points to an address but reverse records imply a different identity.
- Cross-chain reverse resolution assumptions in `ChainReverseResolver`.
- Default resolver/registrar configuration and upgrade or replacement authority.

### DNS Registrar and DNSSEC Oracle

Relevant scope:

- `contracts/dnsregistrar/DNSRegistrar.sol`
- `contracts/dnsregistrar/OffchainDNSResolver.sol`
- `contracts/dnsregistrar/SimplePublicSuffixList.sol`
- `contracts/dnsregistrar/TLDPublicSuffixList.sol`
- `contracts/dnssec-oracle/DNSSECImpl.sol`
- `contracts/dnssec-oracle/SHA1.sol`
- `contracts/dnssec-oracle/algorithms/P256SHA256Algorithm.sol`
- `contracts/dnssec-oracle/algorithms/RSASHA1Algorithm.sol`
- `contracts/dnssec-oracle/algorithms/RSASHA256Algorithm.sol`
- `contracts/dnssec-oracle/digests/SHA1Digest.sol`
- `contracts/dnssec-oracle/digests/SHA256Digest.sol`

Security-relevant mechanics:

- DNS registrar components support importing or resolving DNS names using DNSSEC proof material.
- DNSSEC oracle components validate DNSSEC signatures and digests through configured algorithms.
- Public suffix list contracts help determine valid registrable DNS boundaries.
- Offchain DNS resolver components introduce offchain data availability and correctness assumptions.

Audit focus:

- DNSSEC proof validation, signature algorithm handling, digest verification, and replay/expiry behavior.
- Correct handling of unsupported, deprecated, or weak algorithms such as SHA1 where applicable to compatibility.
- Public suffix list correctness and update authority.
- Boundary cases for DNS name normalization, label parsing, parent/child delegation, and TLD handling.
- Offchain DNS resolver trust assumptions and failure behavior.

### Migration Utilities

Relevant scope:

- `contracts/utils/MigrationHelper.sol`

Security-relevant mechanics:

- Migration helpers are high-risk because they may temporarily receive broad permissions, assets, or ownership authority.

Audit focus:

- One-time-use assumptions.
- Authorization and replay resistance.
- Whether migration operations can be called after intended completion.
- Preservation of ownership, expiry, resolver, and wrapper invariants during migration.

## Main Flows

### Name Registration

Likely flow for `.eth` names:

1. User interacts with `ETHRegistrarController`.
2. Controller checks availability and pricing through registrar/oracle components.
3. Payment is collected according to registration duration and premium rules.
4. `BaseRegistrarImplementation` records ownership/expiry for the label.
5. ENS registry and resolver records may be configured.
6. Name may be wrapped through `NameWrapper` depending on user flow or implementation.

Security assumptions:

- Price oracle output is correct for the requested name and duration.
- Registrar/controller authorization is enforced.
- Expiry state cannot be bypassed by stale registry or wrapper state.
- Payments cannot be mispriced, trapped, or redirected unexpectedly.

### Renewal and Expiry

Names have duration-based ownership. Renewal extends ownership/expiry, while expiry changes the rights associated with a name.

Security assumptions:

- Renewals update the canonical expiry source and any dependent wrapper state consistently.
- Expired names cannot retain unauthorized control through stale resolver, wrapper, or registry state.
- Premium pricing after expiry cannot be manipulated by boundary timing, bulk operations, or oracle inconsistencies.

### Wrapping and Fuse Restrictions

Wrapping changes the control model from plain registry/registrar ownership to wrapped-token ownership plus permission restrictions.

Security assumptions:

- Fuses/restrictions are set only by authorized parties.
- Restrictions cannot be bypassed by unwrapping, renewal, resolver changes, registry direct calls, or subnode operations.
- Expiry and ownership state remain synchronized between wrapper and registrar.
- Known race-condition risk around inappropriate fuse-setting must be treated as an explicit review target.

### Resolution

Resolution starts with registry lookup for a name’s resolver, then resolver-specific calls return records. Universal resolution composes this path, while extended resolvers and CCIP-read may route resolution offchain.

Security assumptions:

- Resolver records are controlled by the legitimate name owner or approved operator.
- Offchain resolution responses are validated and scoped to the intended name, resolver, chain, and request.
- Malicious resolvers cannot cause UniversalResolver to misreport results for unrelated names.

### Reverse Resolution

Reverse registrar/resolver components map addresses back to ENS names for primary-name display.

Security assumptions:

- Only the address owner, authorized controller, or valid contract path can set the reverse record.
- Applications should not assume reverse records alone prove forward ownership unless forward confirmation is performed.

### DNSSEC-Based DNS Name Import/Resolution

DNS registrar and DNSSEC oracle components verify DNSSEC proof chains and use public suffix rules to determine whether DNS names can be claimed or resolved.

Security assumptions:

- DNSSEC algorithms and digests are implemented correctly.
- Proof validity, expiry, delegation, and denial cases are handled precisely.
- Public suffix list data is correct and cannot be maliciously changed by an unauthorized party.

## Accounting and Value Flow

Primary protocol value flows in scope:

- `.eth` registration payments.
- `.eth` renewal payments.
- Premium pricing for names after expiry or release, through `ExponentialPremiumPriceOracle`.
- Bulk renewal aggregate payments through `StaticBulkRenewal`.

Audit invariants:

- Required payment should match oracle price for name, duration, and premium state.
- Renewals should extend the correct name by the correct duration.
- Overpayment/refund behavior, if implemented, should not allow fund loss, reentrancy, or griefing.
- Funds should be forwarded or retained only according to intended ownership/treasury rules; the supplied docs do not specify the beneficiary.
- Expiry reductions are known as a malicious-DAO risk and should be explicitly reviewed.

## External Integrations

- ENS official documentation: authoritative protocol documentation entry point.
- ENS GitHub organization: official codebase resource from Immunefi.
- DNSSEC: external naming security system used by DNS registrar/oracle contracts.
- Public suffix list: external DNS namespace policy data represented onchain by suffix-list contracts.
- CCIP-read/offchain gateways: offchain lookup infrastructure for extended or cross-chain resolution.
- L2/cross-chain resolution: suggested by ENS documentation topics for cross-chain resolvers and chain reverse resolver scope.

Security-relevant integration assumptions:

- Offchain gateways are availability and data-source dependencies, not trusted onchain authorities unless verified by contract logic.
- DNSSEC trust depends on correct cryptographic verification and correct root/delegation assumptions.
- Cross-chain identity data must be scoped by chain/context to avoid replay or ambiguity.

## Trust Boundaries and Privileged Roles

High-trust actors and boundaries visible from supplied sources:

- ENS DAO/governance: known issues explicitly include malicious DAO upgrade and expiry-reduction risks.
- Root owner/controller roles: govern top-level namespace control.
- Registrar controllers: can mediate name registration and renewal.
- Resolver record owners/operators: can modify records for controlled names.
- NameWrapper upgrade/admin authority: especially sensitive due to known Immunefi issues.
- Public suffix list maintainers/admins if updateable in implementation.
- Gateway providers/admins for CCIP-read/offchain resolution routes.
- DNSSEC oracle algorithm/digest configuration authority if mutable in implementation.

Security review should distinguish between:

- Expected governance/admin power.
- Unexpected privilege escalation.
- Admin actions that are known/accepted centralization risks under Immunefi versus exploitable contract bugs.

## Bounty and Known-Issue Context

Immunefi program facts from supplied source:

- Maximum bounty: `250,000 USDC`.
- Smart contract rewards: critical `10,000-250,000 USDC`, high `25,000-100,000 USDC`, medium fixed `10,000 USDC`.
- Proof of Concept is required.
- Testing on mainnet or public testnet deployed code is prohibited; testing should be on local forks of public testnet or mainnet.
- Testing with pricing oracles or third-party smart contracts is prohibited by the supplied Immunefi overview.
- Denial-of-service attacks against project assets and significant automated traffic are prohibited.

Known audits:

- Code4rena audit dated `2023-04-16`: `https://code4rena.com/reports/2023-04-ens`
- Code4rena audit dated `2022-07-18`: `https://code4rena.com/reports/2022-07-ens`

Known issues from Immunefi resources:

- Malicious DAO can steal names using an upgrade to the NameWrapper.
- Malicious DAO can reduce the expiration of names.
- NameWrapper race condition allowing fuses to be set inappropriately.

## Security Review Priorities

Highest-signal review areas from the supplied scope and documentation:

- Registry ownership and resolver authorization invariants.
- `.eth` registration, renewal, expiry, and premium pricing edge cases.
- NameWrapper fuse/expiry synchronization and known race-condition surface.
- Governance/admin upgrade paths, especially NameWrapper and root/registrar control.
- DNSSEC proof validation and public suffix boundary correctness.
- CCIP-read/offchain resolver validation and gateway trust assumptions.
- Reverse resolution spoofing or unauthorized primary-name assignment.
- Migration helper authority and replay/finalization behavior.

## Auditor Notes

- The supplied ENS docs source is an index page only; deeper protocol pages were not included in the bundle.
- Do not treat marketing phrasing such as self-sovereign identity as a security guarantee.
- Do not classify known malicious-DAO powers as novel vulnerabilities without checking Immunefi known-issue treatment and bounty rules.
- For Immunefi context, this document uses only smart-contract-relevant assets, impacts, rewards, repositories, documentation, audits, and known issues from the supplied program material.

### ens-contracts-immunefi-bounty-rules.md

# Immunefi Bounty Rules - ENS

Use this file as mandatory context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/ens/information/
- Scope: https://immunefi.com/bug-bounty/ens/scope/
- Resources: https://immunefi.com/bug-bounty/ens/resources/

## Program Requirements

- Proof of Concept: required
- Primacy: primacy_of_rules
- Rewards token: USDC
- Rewards token network: Ethereum
- Maximum bounty: $250000

## Assets In Scope

> Smart Contract category only. Web & App assets are intentionally excluded.

- smart contract: https://github.com/ensdomains/ens-contracts/wiki/ENS-Contract-Deployments
  - Description: Smart Contracts
- smart contract (Primacy of Impact placeholder): https://immunefi.com
  - Description: Primacy of Impact

## Impacts In Scope

- critical (smart contract): Direct theft of any user NFTs, whether at-rest or in-motion, other than unclaimed royalties
- high (smart contract): Temporary freezing of NFTs
- medium (smart contract): Smart contract unable to operate due to lack of token funds
- medium (smart contract): Block stuffing
- medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- medium (smart contract): Theft of gas
- medium (smart contract): Unbounded gas consumption
- critical (smart contract): Manipulation of governance voting result deviating from voted outcome and resulting in a direct change from intended effect of original results
- critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- high (smart contract): Temporary freezing of funds
- critical (smart contract): Permanent freezing of funds
- critical (smart contract): Permanent freezing of NFTs
- critical (smart contract): Unauthorized minting of NFTs
- critical (smart contract): Predictable or manipulable RNG that results in abuse of the principal or NFT
- critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)
- critical (smart contract): Protocol insolvency
- high (smart contract): Theft of registration fee
- high (smart contract): Permanent freezing of registration fees
- critical (smart contract): Theft of treasury funds
- critical (smart contract): Permanent freezing of treasury funds
- critical (smart contract): Retrieve sensitive data/files from a running server, such as:   Access tokens

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

### Custom Out Of Scope

- Taking over broken links from sources that are no longer considered active, such as links related to meeting minutes, past events etc., as this content is left up for archival purposes and should not be changed.

- Products funded or maintained by the DAO unless otherwise stated

### Prohibited Activities

- Any testing on mainnet or public testnet deployed code; all testing should be done on local-forks of either public testnet or mainnet
- Any testing with pricing oracles or third-party smart contracts
- Attempting phishing or other social engineering attacks against our employees and/or customers
- Any testing with third-party systems and applications (e.g. browser extensions) as well as websites (e.g. SSO providers, advertising networks)
- Any denial of service attacks that are executed against project assets
- Automated testing of services that generates significant amounts of traffic
- Public disclosure of an unpatched vulnerability in an embargoed bounty
- [Any other actions prohibited by the Immunefi Rules](https://immunefi.com/rules/)

### Known Issues

- Malicious DAO can steal names using an upgrade to the NameWrapper
- Malicious DAO can reduce the expiration of names
- Namewrapper race condition allowing fuses to be set inappropriately

## Audit And Documentation Exclusions

- Code4rena audit (2023-04-16T00:00:00.000Z): https://code4rena.com/reports/2023-04-ens
- Code4rena audit (2022-07-18T00:00:00.000Z): https://code4rena.com/reports/2022-07-ens

## Stage 1 Eligibility Rules

- A finding must affect an in-scope asset, unless the exact category and severity are covered by the program's Primacy of Impact rules.
- A finding must produce an impact listed in the program's Impacts in Scope.
- Exclude known issues, prior audit findings, documented accepted risks, closed duplicate reports, and program-specific OOS cases.
- Exclude cases requiring privileged access, leaked credentials, social engineering, malicious or mistaken trusted roles, deployment mistakes, test/mock files, public disclosure, or third-party-only failures.
- Do not perform final exploitability or severity scoring in stage 1; keep only when there is no decisive eligibility blocker.

## Link Handling

- Do not follow Immunefi navigation, marketing, login, social, newsletter, or platform-help links during validation.
- Use only the Source URLs above, in-scope explorer links, codebase links, documentation links, and prior-audit links when live verification is necessary.



### ens-contracts-immunefi-severity-rubric.md

# Immunefi Severity Rubric - ENS

Use this file as mandatory severity context for `validation_profile: immunefi-bounty`.

## Source URLs

- Information: https://immunefi.com/bug-bounty/ens/information/
- Scope: https://immunefi.com/bug-bounty/ens/scope/
- Resources: https://immunefi.com/bug-bounty/ens/resources/
- Immunefi severity system v2.3: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-3/

## Program-Specific Severity Source Of Truth

- Primacy: primacy_of_rules
- Proof of Concept: required

## Impacts In Scope

- critical (smart contract): Direct theft of any user NFTs, whether at-rest or in-motion, other than unclaimed royalties
- high (smart contract): Temporary freezing of NFTs
- medium (smart contract): Smart contract unable to operate due to lack of token funds
- medium (smart contract): Block stuffing
- medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- medium (smart contract): Theft of gas
- medium (smart contract): Unbounded gas consumption
- critical (smart contract): Manipulation of governance voting result deviating from voted outcome and resulting in a direct change from intended effect of original results
- critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- high (smart contract): Temporary freezing of funds
- critical (smart contract): Permanent freezing of funds
- critical (smart contract): Permanent freezing of NFTs
- critical (smart contract): Unauthorized minting of NFTs
- critical (smart contract): Predictable or manipulable RNG that results in abuse of the principal or NFT
- critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)
- critical (smart contract): Protocol insolvency
- high (smart contract): Theft of registration fee
- high (smart contract): Permanent freezing of registration fees
- critical (smart contract): Theft of treasury funds
- critical (smart contract): Permanent freezing of treasury funds
- critical (smart contract): Retrieve sensitive data/files from a running server, such as:   Access tokens

## Rewards By Threat Level

- critical (smart contract) [range]: min $10000, max $250000
- high (smart contract) [range]: min $25000, max $100000
- medium (smart contract) [fixed]: fixed $10000

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



### ens-contracts-immunefi-poc-runtime.md

# Immunefi PoC Runtime - ENS

Use this file as mandatory context for Immunefi R5 PoC generation and R6 PoC verification.

## Fork Preference

- Fork PoCs allowed: `true`
- Fork PoCs preferred: `true`
- Prefer a mainnet fork PoC whenever the finding touches deployed in-scope mainnet contracts and a matching RPC env var is available.
- Use a public-testnet fork only when the in-scope asset itself is a public-testnet deployment, or when no matching mainnet deployment exists but a relevant in-scope public-testnet deployment does.
- Do not use a local non-fork test as the primary proof for an Immunefi deployed-asset finding.
- Never invent RPC URLs, deployed addresses, networks, or block numbers.

## In-Scope Deployed Contracts

- No EVM deployed contract addresses with recognized explorer networks were extracted from the Immunefi Scope tab.

## Network RPC Availability

- No recognized EVM networks were extracted.

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
    "version": "9.1.1",
    "description": "TypeScript definitions for mocha",
    "homepage": "https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/mocha",
    "license": "MIT",
    "contributors": [
        {

### node_modules/@types/node/package.json

{
    "name": "@types/node",
    "version": "18.19.74",
    "description": "TypeScript definitions for node",
    "homepage": "https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/node",
    "license": "MIT",
    "contributors": [
        {

### node_modules/dns-packet/package.json

{
  "name": "dns-packet",
  "version": "5.6.1",
  "description": "An abstract-encoding compliant module for encoding / decoding DNS packets",
  "author": "Mathias Buus",
  "license": "MIT",
  "repository": "mafintosh/dns-packet",
  "homepage": "https://github.com/mafintosh/dns-packet",

### node_modules/clones-with-immutable-args/package.json

{
  "name": "clones-with-immutable-args",
  "author": "wighawag",
  "license": "BSD",
  "version": "1.1.0",
  "description": "Factory for deploying clones with immutable parameters.",
  "files": [
    "*.sol"

### node_modules/dotenv/package.json

{
  "name": "dotenv",
  "version": "16.4.7",
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

### node_modules/prettier/package.json

{
  "name": "prettier",
  "version": "2.8.8",
  "description": "Prettier is an opinionated code formatter",
  "bin": "./bin-prettier.js",
  "repository": "prettier/prettier",
  "funding": "https://github.com/prettier/prettier?sponsor=1",
  "homepage": "https://prettier.io",

### node_modules/@rocketh/deploy/package.json

{
  "name": "@rocketh/deploy",
  "version": "0.14.0",
  "description": "provide deploy function for rocketh",
  "publishConfig": {
    "access": "public"
  },
  "type": "module",

### node_modules/@rocketh/viem/package.json

{
  "name": "@rocketh/viem",
  "version": "0.14.0",
  "description": "provide viem functionality for rocketh",
  "publishConfig": {
    "access": "public"
  },
  "type": "module",

### node_modules/@rocketh/verifier/package.json

{
  "name": "@rocketh/verifier",
  "version": "0.14.5",
  "description": "submit verification proof to verifier services (blockchain explorer, sourcify...",
  "publishConfig": {
    "access": "public"
  },
  "type": "module",

### node_modules/@rocketh/read-execute/package.json

{
  "name": "@rocketh/read-execute",
  "version": "0.14.0",
  "description": "provide read abd execute functions for rocketh",
  "publishConfig": {
    "access": "public"
  },
  "type": "module",

### node_modules/vitest/package.json

{
  "name": "vitest",
  "type": "module",
  "version": "3.2.4",
  "description": "Next generation testing framework powered by Vite",
  "author": "Anthony Fu <anthonyfu117@hotmail.com>",
  "license": "MIT",
  "funding": "https://opencollective.com/vitest",

### node_modules/typescript/package.json

{
    "name": "typescript",
    "author": "Microsoft Corp.",
    "homepage": "https://www.typescriptlang.org/",
    "version": "5.9.2",
    "license": "Apache-2.0",
    "description": "TypeScript is a language for application scale JavaScript development",
    "keywords": [

### node_modules/abitype/zod/package.json

{
  "type": "module",
  "types": "../dist/types/exports/zod.d.ts",
  "main": "../dist/cjs/exports/zod.js"
}

### node_modules/abitype/abis/package.json

{
  "type": "module",
  "types": "../dist/types/exports/abis.d.ts",
  "main": "../dist/cjs/exports/abis.js"
}

### node_modules/abitype/dist/esm/package.json

{"type":"module","sideEffects":false}

### node_modules/abitype/dist/cjs/package.json

{"type":"commonjs"}

### node_modules/abitype/package.json

{
  "name": "abitype",
  "description": "Strict TypeScript types for Ethereum ABIs",
  "version": "1.0.8",
  "license": "MIT",
  "repository": "wevm/abitype",
  "files": [
    "dist",

### node_modules/ts-node/package.json

{
  "name": "ts-node",
  "version": "10.9.2",
  "description": "TypeScript execution environment and REPL for node.js, with source map support",
  "main": "dist/index.js",
  "exports": {
    ".": "./dist/index.js",
    "./package": "./package.json",

### node_modules/rocketh/package.json

{
  "name": "rocketh",
  "version": "0.14.5",
  "description": "deploy smart contract on ethereum-compatible networks",
  "publishConfig": {
    "access": "public"
  },
  "type": "module",

### node_modules/@viem/anvil/dist/esm/package.json

{"type":"module"}

### node_modules/@viem/anvil/dist/cjs/package.json

{"type":"commonjs"}

### node_modules/@viem/anvil/package.json

{
  "name": "@viem/anvil",
  "version": "0.0.10",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/wagmi-dev/anvil.js.git",
    "directory": "packages/anvil.js"

### node_modules/@nomicfoundation/hardhat-viem/package.json

{
  "name": "@nomicfoundation/hardhat-viem",
  "version": "3.0.0",
  "description": "Hardhat plugin for viem",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat-viem",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/@nomicfoundation/hardhat-keystore/package.json

{
  "name": "@nomicfoundation/hardhat-keystore",
  "version": "3.0.0",
  "description": "A module for managing keystore files that store a map from IDs to encrypted string values.",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat-keystore",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/@nomicfoundation/hardhat-network-helpers/package.json

{
  "name": "@nomicfoundation/hardhat-network-helpers",
  "version": "3.0.0",
  "description": "Hardhat utils for testing",
  "homepage": "https://github.com/nomicfoundation/hardhat/tree/v-next/v-next/hardhat-network-helpers",
  "repository": {
    "type": "git",
    "url": "https://github.com/NomicFoundation/hardhat",

### node_modules/@unruggable/gateways/package.json

{
  "name": "@unruggable/gateways",
  "version": "1.3.0",
  "description": "Trustless Ethereum Multichain CCIP-Read Gateway",
  "publishConfig": {
    "access": "public"
  },
  "keywords": [

### node_modules/hardhat/package.json

{
  "name": "hardhat",
  "version": "3.1.4",
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
    "hardhat": "workspace:^3.1.4",

### node_modules/hardhat/templates/hardhat-3/02-mocha-ethers/package.json

{
  "name": "template-mocha-ethers",
  "private": true,
  "version": "0.0.1",
  "description": "A TypeScript Hardhat project using Mocha and Ethers.js",
  "type": "module",
  "devDependencies": {
    "hardhat": "workspace:^3.1.4",

### node_modules/hardhat/templates/hardhat-3/01-node-test-runner-viem/package.json

{
  "name": "template-node-test-runner-viem",
  "private": true,
  "version": "0.0.1",
  "description": "A TypeScript Hardhat project using Node Test Runner and Viem",
  "type": "module",
  "devDependencies": {
    "hardhat": "workspace:^3.1.4",

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
  "version": "4.9.3",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### node_modules/@openzeppelin/contracts-v5/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.1.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### node_modules/viem/celo/package.json

{
  "type": "module",
  "types": "../_types/celo/index.d.ts",
  "module": "../_esm/celo/index.js",
  "main": "../_cjs/celo/index.js"
}

### node_modules/viem/clients/package.json

{
  "type": "module",
  "types": "../_types/clients/index.d.ts",
  "module": "../_esm/clients/index.js",
  "main": "../_cjs/clients/index.js"
}

### node_modules/viem/experimental/erc7846/package.json

{
  "type": "module",
  "types": "../../_types/experimental/erc7846/index.d.ts",
  "module": "../../_esm/experimental/erc7846/index.js",
  "main": "../../_cjs/experimental/erc7846/index.js"
}

### node_modules/viem/experimental/package.json

{
  "type": "module",
  "types": "../_types/experimental/index.d.ts",
  "module": "../_esm/experimental/index.js",
  "main": "../_cjs/experimental/index.js"
}

### node_modules/viem/experimental/erc7821/package.json

{
  "type": "module",
  "types": "../../_types/experimental/erc7821/index.d.ts",
  "module": "../../_esm/experimental/erc7821/index.js",
  "main": "../../_cjs/experimental/erc7821/index.js"
}

### node_modules/viem/experimental/erc7739/package.json

{
  "type": "module",
  "types": "../../_types/experimental/erc7739/index.d.ts",
  "module": "../../_esm/experimental/erc7739/index.js",
  "main": "../../_cjs/experimental/erc7739/index.js"
}

### node_modules/viem/experimental/erc7895/package.json

{
  "type": "module",
  "types": "../../_types/experimental/erc7895/index.d.ts",
  "module": "../../_esm/experimental/erc7895/index.js",
  "main": "../../_cjs/experimental/erc7895/index.js"
}

### node_modules/viem/ens/package.json

{
  "type": "module",
  "types": "../_types/ens/index.d.ts",
  "module": "../_esm/ens/index.js",
  "main": "../_cjs/ens/index.js"
}

### node_modules/viem/_cjs/package.json

{"type":"commonjs"}

### node_modules/viem/nonce/package.json

{
  "type": "module",
  "types": "../_types/nonce/index.d.ts",
  "module": "../_esm/nonce/index.js",
  "main": "../_cjs/nonce/index.js"
}

### node_modules/viem/account-abstraction/package.json

{
  "type": "module",
  "types": "../_types/account-abstraction/index.d.ts",
  "module": "../_esm/account-abstraction/index.js",
  "main": "../_cjs/account-abstraction/index.js"
}

### node_modules/viem/op-stack/package.json

{
  "type": "module",
  "types": "../_types/op-stack/index.d.ts",
  "module": "../_esm/op-stack/index.js",
  "main": "../_cjs/op-stack/index.js"
}

### node_modules/viem/utils/package.json

{
  "type": "module",
  "types": "../_types/utils/index.d.ts",
  "module": "../_esm/utils/index.js",
  "main": "../_cjs/utils/index.js"
}

### node_modules/viem/package.json

{
  "name": "viem",
  "description": "TypeScript Interface for Ethereum",
  "version": "2.33.3",
  "main": "./_cjs/index.js",
  "module": "./_esm/index.js",
  "types": "./_types/index.d.ts",
  "typings": "./_types/index.d.ts",

### node_modules/viem/accounts/package.json

{
  "type": "module",
  "types": "../_types/accounts/index.d.ts",
  "module": "../_esm/accounts/index.js",
  "main": "../_cjs/accounts/index.js"
}

### node_modules/viem/window/package.json

{
  "type": "module",
  "types": "../_types/window/index.d.ts",
  "module": "../_esm/window/index.js",
  "main": "../_cjs/window/index.js"
}

### node_modules/viem/siwe/package.json

{
  "type": "module",
  "types": "../_types/siwe/index.d.ts",
  "module": "../_esm/siwe/index.js",
  "main": "../_cjs/siwe/index.js"
}

### node_modules/viem/zksync/package.json

{
  "type": "module",
  "types": "../_types/zksync/index.d.ts",
  "module": "../_esm/zksync/index.js",
  "main": "../_cjs/zksync/index.js"
}

### node_modules/viem/actions/package.json

{
  "type": "module",
  "types": "../_types/actions/index.d.ts",
  "module": "../_esm/actions/index.js",
  "main": "../_cjs/actions/index.js"
}

### node_modules/viem/node/package.json

{
  "type": "module",
  "types": "../_types/node/index.d.ts",
  "module": "../_esm/node/index.js",
  "main": "../_cjs/node/index.js"
}

### node_modules/viem/chains/utils/package.json

{
  "type": "module",
  "types": "../../_types/chains/utils.d.ts",
  "module": "../../_esm/chains/utils.js",
  "main": "../../_cjs/chains/utils.js"
}

### node_modules/viem/chains/package.json

{
  "type": "module",
  "types": "../_types/chains/index.d.ts",
  "module": "../_esm/chains/index.js",
  "main": "../_cjs/chains/index.js"
}

### node_modules/viem/_esm/package.json

{"type": "module","sideEffects":false}

### node_modules/viem/linea/package.json

{
  "type": "module",
  "types": "../_types/linea/index.d.ts",
  "module": "../_esm/linea/index.js",
  "main": "../_cjs/linea/index.js"
}

### node_modules/prettier-plugin-solidity/package.json

{
  "name": "prettier-plugin-solidity",
  "version": "1.4.2",
  "description": "A Prettier Plugin for automatically formatting your Solidity code.",
  "type": "module",
  "main": "./src/index.js",
  "browser": "./dist/standalone.cjs",
  "unpkg": "./dist/standalone.cjs",

### node_modules/hardhat-deploy/package.json

{
  "name": "hardhat-deploy",
  "version": "2.0.0-next.35",
  "description": "deployment plugin for hardhat",
  "publishConfig": {
    "access": "public"
  },
  "type": "module",

### node_modules/glob/dist/esm/package.json

{
  "type": "module"
}

### node_modules/glob/dist/commonjs/package.json

{
  "type": "commonjs"
}

### node_modules/glob/package.json

{
  "author": "Isaac Z. Schlueter <i@izs.me> (https://blog.izs.me/)",
  "name": "glob",
  "description": "the most correct and second fastest glob implementation in JavaScript",
  "version": "11.0.3",
  "type": "module",
  "tshy": {
    "main": true,

### node_modules/husky/package.json

{
	"name": "husky",
	"version": "9.1.7",
	"type": "module",
	"description": "Modern native Git hooks",
	"keywords": [
		"git",
		"hooks",

### node_modules/@namestone/ezccip/package.json

{
	"name": "@namestone/ezccip",
	"version": "0.1.1",
	"type": "module",
	"scripts": {
		"test": "node test/all.js",
		"start": "node test/demo.js",
		"build": "node build/make.js"

### node_modules/@safe-global/api-kit/package.json

{
  "name": "@safe-global/api-kit",
  "version": "2.5.11",
  "description": "SDK that facilitates the interaction with the Safe Transaction Service API",
  "main": "dist/src/index.js",
  "typings": "dist/src/index.d.ts",
  "keywords": [
    "Ethereum",

### node_modules/@safe-global/safe-core-sdk-types/package.json

{
  "name": "@safe-global/safe-core-sdk-types",
  "version": "5.1.0",
  "description": "Safe Core SDK types",
  "main": "dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "keywords": [
    "Ethereum",

### node_modules/@safe-global/protocol-kit/package.json

{
  "name": "@safe-global/protocol-kit",
  "version": "5.2.13",
  "description": "SDK that facilitates the interaction with Safe Smart Accounts",
  "main": "dist/src/index.js",
  "types": "dist/src/index.d.ts",
  "keywords": [
    "Ethereum",

### node_modules/@ensdomains/hardhat-chai-matchers-viem/package.json

{
  "name": "@ensdomains/hardhat-chai-matchers-viem",
  "type": "module",
  "version": "0.1.14",
  "module": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "typings": "./dist/index.d.ts",
  "exports": {

### node_modules/@ensdomains/dnsprovejs/package.json

{
  "name": "@ensdomains/dnsprovejs",
  "version": "0.5.1",
  "description": "A library to generate chains of trust proving DNS records via DNSSEC",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "dependencies": {
    "@noble/hashes": "^1.3.2",

### node_modules/@ensdomains/address-encoder/coders/package.json

{
  "types": "../dist/types/coders.d.ts",
  "module": "../dist/esm/coders.js",
  "main": "../dist/cjs/coders.js"
}

### node_modules/@ensdomains/address-encoder/dist/esm/package.json

{"type":"module","sideEffects":false}

### node_modules/@ensdomains/address-encoder/dist/cjs/package.json

{"type":"commonjs"}

### node_modules/@ensdomains/address-encoder/async/package.json

{
  "types": "../dist/types/async.d.ts",
  "module": "../dist/esm/async.js",
  "main": "../dist/cjs/async.js"
}

### node_modules/@ensdomains/address-encoder/utils/package.json

{
  "types": "../dist/types/utils/index.d.ts",
  "module": "../dist/esm/utils/index.js",
  "main": "../dist/cjs/utils/index.js"
}

### node_modules/@ensdomains/address-encoder/coins/package.json

{
  "types": "../dist/types/coins.d.ts",
  "module": "../dist/esm/coins.js",
  "main": "../dist/cjs/coins.js"
}

### node_modules/@ensdomains/address-encoder/package.json

{
  "name": "@ensdomains/address-encoder",
  "description": "Encodes and decodes address formats for various cryptocurrencies",
  "dependencies": {
    "@noble/curves": "^1.2.0",
    "@noble/hashes": "^1.3.2",
    "@scure/base": "^1.1.5"
  },

### node_modules/@ensdomains/solsha1/package.json

{
  "name": "@ensdomains/solsha1",
  "version": "0.0.3",
  "requires": true,
  "lockfileVersion": 1,
  "devDependencies": {
    "@nomiclabs/hardhat-ethers": "^2.0.2",
    "@nomiclabs/hardhat-truffle5": "^2.0.0",

### node_modules/@ensdomains/buffer/package.json

{
  "name": "@ensdomains/buffer",
  "version": "0.1.3",
  "requires": true,
  "lockfileVersion": 1,
  "devDependencies": {
    "@nomiclabs/hardhat-ethers": "^2.0.2",
    "@nomiclabs/hardhat-truffle5": "^2.0.7",


 ------------ ## CONFIG FILES ------------ 

 *Note*: Check for important package version info.

 ### package.json

{
  "name": "@ensdomains/ens-contracts",
  "version": "1.7.0",
  "description": "ENS contracts",
  "type": "module",
  "scripts": {
    "compile": "hardhat compile",
    "test": "bun run compile && NODE_OPTIONS=\"--disable-warning=ExperimentalWarning\" vitest run",
    "test:remote": "TEST_REMOTE=1 bun run test ./test/**/Test*.remote.ts",
    "test:parallel": "bun run test ./test/**/Test*.ts --sequence.concurrent",
    "test:local": "bun run test --network localhost",
    "test:deploy": "bun ./scripts/deploy-test.ts",
    "l2:deploy": "bun run hh deploy --tags l2",
    "interfaces": "bun run compile --quiet && bun ./scripts/interfaces.ts",
    "lint": "bun run hh check",
    "build": "rm -rf ./build/deploy ./build/hardhat.config.js && hardhat compile && tsc",
    "format": "prettier --write .",
    "prepublishOnly": "bun run build",
    "pub": "npm publish --access public",
    "wiki": "bun ./scripts/wiki.ts"
  },
  "files": [
    "build",
    "contracts/**/*.sol",
    "artifacts",
    "deployments"
  ],
  "main": "index.js",
  "devDependencies": {
    "@ensdomains/address-encoder": "1.1.2",
    "@ensdomains/dnsprovejs": "^0.5.1",
    "@ensdomains/hardhat-chai-matchers-viem": "^0.1.14",
    "@namestone/ezccip": "^0.1.1",
    "@nomicfoundation/hardhat-keystore": "^3.0.0",
    "@nomicfoundation/hardhat-network-helpers": "^3.0.0",
    "@nomicfoundation/hardhat-viem": "^3.0.0",
    "@rocketh/deploy": "0.14.0",
    "@rocketh/read-execute": "0.14.0",
    "@rocketh/verifier": "0.14.5",
    "@rocketh/viem": "0.14.0",
    "@safe-global/api-kit": "^2.5.6",
    "@safe-global/protocol-kit": "5.2.13",
    "@safe-global/safe-core-sdk-types": "^5.1.0",
    "@types/mocha": "^9.1.1",
    "@types/node": "^18.0.0",
    "@viem/anvil": "^0.0.10",
    "abitype": "^1.0.8",
    "chai": "^5.1.1",
    "dotenv": "^16.4.5",
    "ethers": "^6.15.0",
    "hardhat": "3.1.4",
    "hardhat-deploy": "^2.0.0-next.35",
    "husky": "^9.1.7",
    "prettier": "^2.6.2",
    "prettier-plugin-solidity": "^1.0.0-beta.24",
    "rocketh": "0.14.5",
    "ts-node": "^10.9.2",
    "typescript": "5.9.2",
    "viem": "2.33.3",
    "vitest": "^3.2.3"
  },
  "dependencies": {
    "@ensdomains/buffer": "^0.1.3",
    "@ensdomains/solsha1": "0.0.3",
    "@openzeppelin/contracts": "4.9.3",
    "@openzeppelin/contracts-v5": "npm:@openzeppelin/contracts@5.1.0",
    "@unruggable/gateways": "^1.3.0",
    "clones-with-immutable-args": "Arachnid/clones-with-immutable-args#feature/create2",
    "dns-packet": "^5.3.0",
    "glob": "^11.0.3"
  },
  "resolutions": {
    "hardhat": "3.1.4"
  },
  "directories": {
    "test": "test"
  },
  "repository": {
    "type": "git",
    "url": "git+https://github.com/ensdomains/ens-contracts.git"
  },
  "author": "",
  "license": "MIT",
  "bugs": {
    "url": "https://github.com/ensdomains/ens-contracts/issues"
  },
  "homepage": "https://github.com/ensdomains/ens-contracts#readme",
  "volta": {
    "node": "24.6.0"
  }
}


### remappings.txt

@unruggable/gateways/=node_modules/@unruggable/gateways/contracts
@openzeppelin/contracts-v5/=node_modules/@openzeppelin/contracts-v5/
@openzeppelin/contracts/=node_modules/@openzeppelin/contracts/
clones-with-immutable-args/=node_modules/clones-with-immutable-args/

