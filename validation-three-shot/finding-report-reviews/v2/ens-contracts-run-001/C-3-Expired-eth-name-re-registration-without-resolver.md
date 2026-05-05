# C-3 Finding Report Review

Status: Ready
Benchmark: `ens-contracts`
Finding: `C-3`
Report reviewed: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/finding-reports/v2/ens-contracts-run-001/C-3-Expired-eth-name-re-registration-without-resolver.md`

## Review Result

The report is submission-ready. I did not edit the report.

## Checks Performed

- Read the R8 single-finding input, R7 report, R6 PoC verification unit, ENS bounty rules, severity rubric, PoC runtime, scope artifacts, audit docs, source README, affected source code, and the verified PoC test.
- Confirmed severity is `Critical`, matching the round 3 classification and the generated ENS Immunefi rubric.
- Confirmed `Bounty Criteria Match` names the exact in-scope impact row: `critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)`.
- Confirmed the exploit path is unprivileged: the attacker only needs to be the prior legitimate registrant and does not require leaked keys, compromised credentials, or privileged/trusted-role behavior.
- Confirmed affected contracts are in scope: `ETHRegistrarController.sol`, `BaseRegistrarImplementation.sol`, and `ENSRegistry.sol`.
- Confirmed source evidence supports the issue: `ETHRegistrarController.register` uses the no-resolver branch to call only `base.register`; `BaseRegistrarImplementation._register` calls `ens.setSubnodeOwner`; `ENSRegistry.setSubnodeOwner` updates owner state without clearing resolver or TTL.
- Confirmed the report uses function-level GitHub links and concise snippets.
- Compared the embedded report PoC against `test/ExpiredNameNoResolverReRegistrationPoC.test.ts`; it matches the verified test content, with only an extraction-boundary trailing-newline difference.
- Confirmed the report has a portable `Save as: test/ExpiredNameNoResolverReRegistrationPoC.test.ts` path under top-level `test/`.
- Confirmed the report has a portable `Run: bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts` command.
- Confirmed the PoC is a local simulation only, uses no raw RPC URL, does not broadcast live transactions, and does not mutate live protocol state.
- Confirmed the final report contains no local absolute paths, raw RPC URLs, console output, icons, fluff, or custom code wrappers.

## PoC Run

Command run from the ENS repository root:

```bash
bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts
```

Result: passed. Vitest reported `1 passed` test file and `1 passed` test.

## Notes

No unresolved eligibility, scope, severity, PoC, or report-quality issue remains.
