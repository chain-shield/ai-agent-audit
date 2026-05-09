# C-3 Round 8 Finding Report Review

Status: Fixed And Ready
Benchmark: `ens-contracts`
Finding: `C-3`
Report reviewed: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/finding-reports/v2/ens-contracts-run-002/C-3-Expired-eth-name-re-registration-without-resolver.md`

## Review Result

The report is Immunefi bounty submission-ready after one small PoC-packaging edit. I added an `Additional files:` note clarifying that no new files are required beyond the test file and that the PoC uses the repository's existing `OwnedResolver` artifact.

The severity matches the round 3 classification and the generated ENS Immunefi rubric: `Critical`. The `Bounty Criteria Match` names the exact in-scope impact row: `critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)`.

The report frames an unprivileged previous registrant attacker and explicitly rules out malicious or mistaken trusted roles, leaked keys, compromised credentials, social engineering, deployment mistakes, and user approval misuse. The affected source files are in scope, and the report ties the exploit to current code paths in `ETHRegistrarController.register`, `BaseRegistrarImplementation._register`, and `ENSRegistry` record storage.

## PoC Verification

The embedded report PoC is byte-for-byte identical to the verified PoC test file:

`test/ExpiredNameNoResolverReRegistrationPoC.test.ts`

I ran the PoC from the ENS repository root with `MAINNET_RPC_URL` loaded from the audit workspace environment:

`bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts`

Result: passed. Vitest reported `1` test file passed and `1` test passed.

The report contains a portable `Save as:` path under top-level `test/`, a portable `Run:` command with an environment-variable placeholder, and an `Expected result:` section that names the key success criteria. The PoC is a local Anvil mainnet-fork simulation only and does not broadcast live transactions or mutate live protocol state.

## Checklist Notes

- Mandatory artifacts reviewed: R8 input, R7 report, R6 PoC verification unit, R5 PoC run summary, ENS docs, bounty rules, severity rubric, PoC runtime, scope markdown, scope text, and ENS README.
- Affected source reviewed: `ETHRegistrarController.sol`, `BaseRegistrarImplementation.sol`, `ENSRegistry.sol`, and `OwnedResolver.sol`.
- Report evidence includes function-level GitHub links, selective snippets, root cause, attack preconditions, exploit walkthrough, impact/severity mapping, recommended fix, references, and concrete state assertions.
- No unresolved eligibility, scope, severity, or PoC concerns remain.
