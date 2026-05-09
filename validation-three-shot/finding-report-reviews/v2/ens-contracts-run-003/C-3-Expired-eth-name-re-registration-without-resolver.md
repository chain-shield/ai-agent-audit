# C-3 Finding Report Review

Status: Fixed And Ready
Benchmark: `ens-contracts`
Finding: `C-3`
Report reviewed: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/finding-reports/v2/ens-contracts-run-003/C-3-Expired-eth-name-re-registration-without-resolver.md`

## Review Result

The report is Immunefi submission-ready after reframing severity from Critical to Medium based on the R9 judge simulation and final sanity review.

## Evidence Reviewed

- R8 single-finding input for `C-3`.
- R7 report for `C-3`.
- R6 verified PoC review unit and the referenced top-level test file `test/ExpiredNameNoResolverReRegistrationPoC.test.ts`.
- Mandatory ENS artifacts: docs, bounty rules, severity rubric, PoC runtime, scope markdown, scope text, and repository README.
- Affected source code in `ETHRegistrarController.register`, `BaseRegistrarImplementation._register`, `ENSRegistry.setSubnodeOwner` / `setRecord` / `setResolver`, and `OwnedResolver`.

## Checklist Notes

- Severity is `Medium`, matching the strongest acceptance path from the R9 judge simulation.
- `Bounty Criteria Match` names the exact in-scope row: `medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)`.
- The exploit path is unprivileged and does not rely on malicious or mistaken trusted roles, leaked keys, compromised credentials, phishing, governance action, or live-network mutation.
- The affected contracts and source paths are in scope, and the PoC uses deployed ENS mainnet addresses on a local fork only.
- The report contains the required executive summary, root cause, preconditions, threat model, walkthrough, impact/severity mapping, recommended fix, references, function-level source links, selective snippets, and a full standalone PoC.
- The report has a portable `Save as:` path under top-level `test/`, a portable `Run:` command using `MAINNET_RPC_URL=<mainnet rpc url>`, and an expected result describing the ownership/resolver assertions.
- The embedded PoC matches the verified PoC test content except for the final trailing newline and proves real state damage: the victim owns the NFT and registry node while the stale attacker-controlled resolver remains active and mutable.
- The report no longer claims permanent NFT alteration or direct theft; it now frames the issue as stale-resolution griefing that the new owner can fix after discovery.
- Local prose count is approximately 816 words excluding code blocks, link-only material, and run commands, so it is under the 1,000-word cap.
- The final report contains no local absolute paths, raw RPC URLs, irrelevant console output, icons, fluff, or custom wrappers.

## PoC Run

From `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts`:

```bash
set -a; source '/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/.env'; set +a; bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts
```

Result: passed.

Key terminal result:

```text
Test Files  1 passed (1)
Tests       1 passed (1)
```
