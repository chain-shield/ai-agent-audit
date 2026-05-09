# H-31 R8 Finding Report Review

Status: Ready
Benchmark: `thegraph`
Finding: `H-31`
Report reviewed: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/finding-reports/v2/thegraph-run-001/H-31-Unsigned-receiverDestination-lets-the-data-service.md`
Report edits: none

## Review Notes

- Read the R8 input, R7 report, R6 PoC verification unit, mandatory The Graph bounty artifacts, affected source files, and the verified PoC test file.
- Severity matches the round 3 Immunefi classification: `Critical`. The report names the exact in-scope row: `critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)`.
- Eligibility is supported by the bounty artifacts: `GraphTallyCollector`, `PaymentsEscrow`, `GraphPayments`, `SubgraphService`, `L2GraphToken`, and `HorizonStaking` are in-scope Arbitrum assets. The exploit is a local-fork smart-contract payment custody issue and does not rely on phishing, leaked keys, compromised credentials, privileged roles, governance, public-chain testing, or user mistakes.
- Source evidence matches the local commit `52b5356d3efea1508396b7f46a18854fb7ae9112`: `GraphTallyCollector._collect()` decodes unsigned `receiverDestination`, `_encodeRAV()` omits it from the signed digest, `PaymentsEscrow.collect()` forwards it, `GraphPayments.collect()` pays it, and `SubgraphService` separately tracks provider-controlled payment destinations.
- The report PoC matches the verified test file byte-for-byte: `graphprotocol-contracts/packages/horizon/test/UnsignedRavPayoutDestinationPoC.t.sol`.
- The secret Gist PoC bundle is linked in the report, is unlisted/secret, contains `README.md`, `.env.example`, and `UnsignedRavPayoutDestinationPoC.t.sol`, and the Gist PoC file SHA-256 matches the verified local PoC file: `7400e147ccfd38ef5178290b28d121ba089cddf04a52dde89c0d6c96bbcfbed8`.
- Re-ran the PoC from `graphprotocol-contracts/packages/horizon` using the report command and `ARBITRUM_RPC_URL` on fork block `460062146`. Result: `1 passed; 0 failed; 0 skipped`.
- The report includes a `Proof of Code` section with explicit `graphprotocol-contracts` / `packages/horizon` repo-package placement, a portable package-local `Save as:` path, a relative `Run:` command using an RPC environment variable placeholder, and an `Expected result:` statement with concrete assertions.
- Prose word count excluding fenced code blocks is `952`, under the 1,000-word cap even before excluding links and commands.

No unresolved scope, severity, PoC, reproducibility, or submission-quality issue remains.
