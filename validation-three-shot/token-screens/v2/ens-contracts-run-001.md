# ens-contracts Three-Shot Stage 2 Bounty Exploitability Screen

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
| C-1 / `_qAeE89D1CAdrEK_KI7DX` | Expired ENS names can receive hidden ERC721 approvals that become active again after renewal | Keep | High | current-exploit | OZ approve bypasses expiry-aware ownerOf and a prior operator can set a hidden per-token approval during grace that survives renewal and revocation. |
| M-2 / `51MUeJkjADHOWM6JQl0XX` | Reusable reverse-name signatures let stale relayers overwrite newer DefaultReverseRegistrar records | Keep | Medium | current-exploit | Any holder of a still-valid signature can replay it before expiry and overwrite a newer reverse record because no nonce or consumed digest is checked. |
| C-3 / `J8z405q9aEEOyFyX089YE` | Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution | Keep | High | current-exploit | Zero-resolver registration calls setSubnodeOwner only, leaving the previous resolver live under new ownership and controllable by the former registrant. |
| H-4 / `VBCthqJsNe58AfV8cKuzh` | P256SHA256Algorithm.verify accepts DNSKEY material that is not bound to DNSSEC Algorithm 13 | Exclude | High | safeguard-blocks | DNSSECImpl parses DNSKEY protocol and algorithm before dispatch, so malformed Algorithm 13 headers do not reach P256 verification in the oracle path. |
| M-5 / `6iLweelVLw3pz3C8yl6Jl` | UniversalResolver multicall returns failed CCIP batch lookups as successful resolver results | Exclude | Medium | trusted-role-error | Spoofed successful multicall output requires a malicious or wrong batch gateway response; an unprivileged caller cannot impose those bytes on victims. |
| M-6 / `czakDZoLlxs5lgJdEaovq` | Multicoin addr resolution returns address ABI and rejects valid non-20-byte ENSIP-11 records | Exclude | High | not-exploitable | The ABI bug is real but only affects caller-supplied or DNS-owner-controlled TXT context and gives no unprivileged path against another user or protocol state. |
| M-7 / `6a-XhkrZKMLYM7rhTBUe9` | Stale token approvals survive transfers after CANNOT_APPROVE, letting an old approved address extend subnames | Keep | High | current-exploit | A preexisting token approval survives a CANNOT_APPROVE transfer and remains usable for extendExpiry while the new owner cannot clear it. |
| M-9 / `lVVCkIB8v6NnxPXfbcKu2` | Strict less-than grace-period checks let expired .eth names be transferred at the registrar expiry timestamp | Keep | Medium | current-exploit | Wrapper grace checks use strict less-than, so a stale owner or operator can transfer or mutate at the exact registrar expiry boundary before grace restrictions start. |
| L-10 / `trbxeQBTMBIC-ALIGbQLF` | ETH reverse resolver returns addr.reverse registrar for unrelated reverse namespaces | Exclude | High | not-exploitable | This is a direct view resolver namespace mismatch and does not let an attacker change records or force clients to query the wrong resolver. |
| L-11 / `BDPZ_5wcfnzQUS0E-X5fl` | Odd-length reverse labels resolve as aliases for canonical EVM addresses | Exclude | High | not-exploitable | Odd-length aliases only affect direct crafted view calls and do not create a state change or attacker-controlled impact on another account. |
| M-12 / `DRKsNuQtFB8DiYxxNr00-` | Non-canonical 39-nibble reverse labels resolve to canonical address names in DefaultReverseResolver.resolve | Exclude | High | not-exploitable | Same canonicality issue as L-11; crafted view input can alias an address but no unprivileged harmful state transition follows. |
| M-13 / `B80Sn7wWoAV__-X9emW1F` | Default registrar is returned for unrelated reverse namespaces in DefaultReverseResolver.resolve | Exclude | High | not-exploitable | Registrar discovery is mis-scoped in a view call, but the attacker cannot alter namespace ownership or force a victim resolver route. |
| M-14 / `0MzfSAZHFu-yafYBgn7kd` | Malformed short DNS TXT context reverts ExtendedDNSResolver._findValue and DoSes resolution | Exclude | High | not-exploitable | Malformed context reverts, but the context is caller-supplied or DNS-owner-controlled and cannot be forced onto a victim name without DNS or gateway control. |
| M-15 / `5YV4UNdiG3l7fe3YbslGB` | Ignored TXT records ending at EOF trigger out-of-bounds reads and DoS ExtendedDNSResolver lookups | Exclude | High | not-exploitable | EOF parser reverts require malformed TXT context controlled by the DNS owner or caller, with no concrete unprivileged victim path. |
| M-16 / `juVObwZbLvsqXFJSoaFF9` | Ignored bracketed TXT key at EOF DoS in ExtendedDNSResolver._findValue reverts all matching lookups | Exclude | High | not-exploitable | This duplicates the ignored bracket EOF parser revert and still lacks a path beyond caller-controlled or DNS-owner-controlled context. |
| H-17 / `ykxAX67ZAlhbqeuUQ6hwf` | StaticBulkRenewal.renewAll refunds the entire contract ETH balance to the caller | Exclude | High | weak-poc-path | The global-balance refund can sweep forced ETH, but normal operation leaves no user funds in this non-payable helper and the PoC profit is forced or mistaken ETH. |
| H-18 / `lgI_AYG83YjljxWSCyCrG` | StaticBulkRenewal.renewAll fails at the final grace-period timestamp when premium refund is returned | Exclude | High | not-exploitable | The boundary refund failure is a self-failing helper path at one timestamp, not an attacker-driven freeze of another user's renewal. |
| M-19 / `fupzTRRvDjR5RojlZjHzX` | StaticBulkRenewal.renewAll always calls msg.sender refund and blocks non-receivable contract callers | Exclude | High | not-exploitable | Rejecting contract callers DoS their own bulk renewal because they cannot receive the refund call; no external attacker path is shown. |
| M-20 / `RLL39_tA45x7uvx02N6gb` | Malformed DNSKEY exponent length makes RSASHA256Algorithm.verify revert instead of rejecting the proof | Exclude | High | weak-poc-path | Malformed RSA keys can revert verification, but an attacker only makes their own proof transaction fail absent a relayer or trusted DNS input dependency. |
| H-21 / `Ovl2IoLGc0ESEgrGyixaa` | RSASHA256Algorithm.verify accepts forged signatures when DNSKEY exponent is 1 | Exclude | Medium | weak-poc-path | Exponent-one direct verify succeeds, but exploitation requires a trusted DNSKEY or DS chain for the target zone, which an attacker cannot create. |
| M-22 / `AyWIZjZJ3bf1WXvDs4EXH` | Malformed RSA key material makes RSASHA256Algorithm.verify revert instead of returning false | Exclude | High | weak-poc-path | Same malformed RSA input class as M-20; reverts are paid by the caller and no victim-submitted proof path is concrete. |
| M-23 / `fkK11fgPckx9ByS1Icpau` | Unbounded RSA key and signature sizes allow gas griefing through RSASHA256Algorithm.verify | Exclude | High | not-exploitable | Oversized RSA inputs burn gas in the caller's own verification transaction and no protocol path forces another party to process attacker-sized proofs. |
