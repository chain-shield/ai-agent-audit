# C-1 Finding Report Review

Status: Fixed And Ready
Benchmark: `ens-contracts`
Finding: `C-1`

## Review Result

The report is Immunefi submission-ready after a minor wording patch. The issue remains valid and maps to the round 3 severity classification: Critical.

## Checks Performed

- Read the R8 C-1 input, R7 report, R6 PoC verification record, ENS bounty rules, severity rubric, PoC runtime, scope artifacts, audit docs, and repository README.
- Reviewed affected source code in `contracts/ethregistrar/BaseRegistrarImplementation.sol` and inherited OpenZeppelin `ERC721.approve`.
- Compared the report PoC against the verified PoC file at `test/ExpiredApprovalRenewalTheftPoC.test.ts`; the code block matches the verified test.
- Ran `bun run test test/ExpiredApprovalRenewalTheftPoC.test.ts` from the ENS repository root. Result: passed, 1 test file and 1 test passed.

## Eligibility And Severity

- Severity matches the required classification: Critical.
- `Bounty Criteria Match` names the exact in-scope Immunefi impact row: `critical (smart contract): Direct theft of any user NFTs, whether at-rest or in-motion, other than unclaimed royalties`.
- The affected contract is in scope: `contracts/ethregistrar/BaseRegistrarImplementation.sol`.
- The attacker does not need leaked keys, compromised credentials, DAO/admin access, governance control, or a malicious trusted role.
- The impact is direct theft of the renewed ENS registrar ERC721 plus unauthorized registry owner reassignment through `reclaim`.
- The finding is not a QA, hardening, best-practice, or speculative report.

## PoC Review

- `Save as:` uses a portable top-level test path: `test/ExpiredApprovalRenewalTheftPoC.test.ts`.
- `Run:` uses a portable relative command: `bun run test test/ExpiredApprovalRenewalTheftPoC.test.ts`.
- Imports are valid from the stated top-level `test/` location.
- The PoC is a non-harmful local Hardhat/Vitest simulation and does not use RPC URLs, private keys, live broadcasts, or live protocol state mutation.

## Report Changes Made

- Clarified that the renewal step models the normal authorized registrar renewal path and that the attacker does not need controller privileges.
- Added an explicit local-only, non-broadcasting PoC safety note.

## Remaining Risk

No unresolved eligibility, scope, severity, or PoC issues remain.
