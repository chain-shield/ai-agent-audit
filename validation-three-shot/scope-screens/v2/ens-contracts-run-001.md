# ens-contracts Three-Shot Stage 1 Scope Screen

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ens-contracts/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`
- `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| C-1 | Expired ENS names can receive hidden ERC721 approvals that become active again after renewal | Keep | High | in-scope | BaseRegistrarImplementation is scoped and claimed renewed name theft maps to direct user NFT theft. |
| M-2 | Reusable reverse-name signatures let stale relayers overwrite newer DefaultReverseRegistrar records | Keep | High | in-scope | DefaultReverseRegistrar is scoped and temporary reverse-name overwrite can map to Medium griefing. |
| C-3 | Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution | Keep | High | in-scope | ETHRegistrarController and registrar flow are scoped and stale resolver control can map to funds in motion or NFT representation impacts. |
| H-4 | P256SHA256Algorithm.verify accepts DNSKEY material that is not bound to DNSSEC Algorithm 13 | Keep | Medium | in-scope | P256SHA256Algorithm is scoped and forged DNSSEC acceptance can affect DNS-backed ENS control with no decisive OOS blocker. |
| M-5 | UniversalResolver multicall returns failed CCIP batch lookups as successful resolver results | Keep | Medium | in-scope | UniversalResolver flow is scoped and failed lookup success semantics can map to resolver spoofing or griefing. |
| M-6 | Multicoin addr resolution returns address ABI and rejects valid non-20-byte ENSIP-11 records | Keep | High | in-scope | ExtendedDNSResolver is scoped and broken DNS-backed multicoin resolution can map to Medium griefing. |
| M-7 | Stale token approvals survive transfers after CANNOT_APPROVE, letting an old approved address extend subnames | Keep | High | in-scope | NameWrapper is scoped and stale approval authority can grief transferred names without matching the listed known issues exactly. |
| H-8 | Stale or invalid ETH/USD oracle answers can underprice ENS registration and renewal fees | Exclude | High | external-system-oos | The impact depends on stale or invalid data from a third-party ETH/USD pricing oracle, which the bounty excludes. |
| M-9 | Strict less-than grace-period checks let expired .eth names be transferred at the registrar expiry timestamp | Keep | High | in-scope | NameWrapper is scoped and unauthorized boundary-time wrapped-name mutation can map to NFT or griefing impacts. |
| L-10 | ETH reverse resolver returns addr.reverse registrar for unrelated reverse namespaces | Keep | Medium | in-scope | AbstractReverseResolver is scoped and incorrect registrar discovery can plausibly map to Medium resolver griefing. |
| L-11 | Odd-length reverse labels resolve as aliases for canonical EVM addresses | Keep | Medium | in-scope | AbstractReverseResolver is scoped and non-canonical reverse identity aliases can plausibly map to Medium griefing. |
| M-12 | Non-canonical 39-nibble reverse labels resolve to canonical address names in DefaultReverseResolver.resolve | Keep | Medium | in-scope | DefaultReverseResolver is scoped and non-canonical reverse-name acceptance can plausibly map to Medium griefing. |
| M-13 | Default registrar is returned for unrelated reverse namespaces in DefaultReverseResolver.resolve | Keep | Medium | in-scope | DefaultReverseResolver is scoped and cross-namespace registrar discovery can plausibly map to Medium griefing. |
| M-14 | Malformed short DNS TXT context reverts ExtendedDNSResolver._findValue and DoSes resolution | Keep | High | in-scope | ExtendedDNSResolver is scoped and resolver DoS maps to Medium griefing or gas theft. |
| M-15 | Ignored TXT records ending at EOF trigger out-of-bounds reads and DoS ExtendedDNSResolver lookups | Keep | High | in-scope | ExtendedDNSResolver is scoped and malformed TXT context DoS maps to Medium griefing or gas theft. |
| M-16 | Ignored bracketed TXT key at EOF DoS in ExtendedDNSResolver._findValue reverts all matching lookups | Keep | High | in-scope | ExtendedDNSResolver is scoped and resolver lookup reverts can map to Medium griefing or gas theft. |
| H-17 | StaticBulkRenewal.renewAll refunds the entire contract ETH balance to the caller | Keep | High | in-scope | StaticBulkRenewal is scoped and sweeping contract ETH can map to direct theft of user funds or registration funds. |
| H-18 | StaticBulkRenewal.renewAll fails at the final grace-period timestamp when premium refund is returned | Keep | High | in-scope | StaticBulkRenewal is scoped and failed final renewal can map to permanent loss or freezing of a name renewal opportunity. |
| M-19 | StaticBulkRenewal.renewAll always calls msg.sender refund and blocks non-receivable contract callers | Keep | Medium | in-scope | StaticBulkRenewal is scoped and blocking otherwise valid bulk renewals can map to Medium griefing. |
| M-20 | Malformed DNSKEY exponent length makes RSASHA256Algorithm.verify revert instead of rejecting the proof | Keep | High | in-scope | RSASHA256Algorithm is scoped and malformed proof reverts can map to Medium griefing or gas theft. |
| H-21 | RSASHA256Algorithm.verify accepts forged signatures when DNSKEY exponent is 1 | Keep | High | in-scope | RSASHA256Algorithm is scoped and forged DNSSEC acceptance can affect ENS DNS-name control without a stage-1 scope blocker. |
| M-22 | Malformed RSA key material makes RSASHA256Algorithm.verify revert instead of returning false | Keep | High | in-scope | RSASHA256Algorithm is scoped and malformed proof reverts can map to Medium griefing or gas theft. |
| M-23 | Unbounded RSA key and signature sizes allow gas griefing through RSASHA256Algorithm.verify | Keep | High | in-scope | RSASHA256Algorithm is scoped and unbounded verifier cost maps to listed gas theft or unbounded gas impacts. |
