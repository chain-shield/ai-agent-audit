# thegraph Three-Shot Validation run-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/thegraph/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/thegraph-023b7fdc8558`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`
Stage 1 scope screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/scope-screens/v2/thegraph-run-001.md`
Stage 2 bounty exploitability screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/token-screens/v2/thegraph-run-001.md`
Stage 3 final validation run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/stage3-runs/v2/thegraph-run-001.md`

## Assembly Summary

- Excluded at stage 1 (scope / known issue): `78`
- Excluded at stage 2 (bounty exploitability): `54`
- Fully validated at stage 3: `32`

## Per-Finding Validation

### M-1 / `00u1WdlXYlFoY8snsn4ub`
- Finding Title: GraphTokenLock release after revocation can overcount released plus revoked amounts and brick accounting views
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Stated impact is accounting view corruption and misclassification, not a listed direct fund theft or loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-2 / `vBDR8Rh3hlgRDaWWbljg8`
- Finding Title: Second undelegation postpones already-unbonding delegated tokens in StakingExtension._undelegate
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Temporary withdrawal delay is not one of the program's listed smart-contract impacts.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-3 / `cXd7qTaJeVtidLXoFraQK`
- Finding Title: Just-in-time mint before L2Curation.collect lets attacker siphon pending curation fees
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `jit-curation-fee-capture`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged mint-burn path, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, exploit depends on observable authorized collect and profitable MEV ordering`
- Detailed Reason: The reserve-share timing issue is real in L2Curation, but strict Immunefi submission readiness requires a concrete path to significant (>$1M) direct user-fund loss. This finding gives a plausible MEV fee-sniping mechanism but not a deployed high-value pending collection, capital/tax profitability analysis, or runnable fork path showing the threshold can be met, so it should not be marked Valid yet.
- Code Evidence: `graphprotocol-contracts/packages/contracts/contracts/l2/curation/L2Curation.sol` has `collect()` add `_tokens` directly to `curationPool.tokens`, while public `mint()` immediately mints GCS and `burn()` prices redemption through `signalToTokens()` over the current reserve and supply.

### H-4 / `NapF48O8-6MPuxGVZfffp`
- Finding Title: Locked stake verifier whitelist bypass in HorizonStaking.provision enables slash-based extraction to unallowed verifier
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `locked-verifier-bypass`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `requires authorized locked-wallet/service-provider action, uses slashing path, no unprivileged theft, no exact listed impact`
- Detailed Reason: The missing whitelist check on the generic provisioning path is code-supported, but the described extraction requires the locked wallet/service provider or its authorized operator to intentionally create the provision and an attacker verifier to slash it. That is a lock-bypass/slashing scenario driven by an authorized participant, not unprivileged direct theft under The Graph's listed Immunefi impact rows.
- Code Evidence: `HorizonStaking.provision()` calls `_createProvision()` without `_allowedLockedVerifiers`, while `provisionLocked()` checks that mapping; `slash()` lets the verifier transfer `tokensVerifier`, but only after an authorized provision exists.

### M-5 / `Yn8KDMMJW4OWEtnzs5BOt`
- Finding Title: Bearer vouchers let mempool copiers consume allocations before legitimate redeemMany batches
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Copied vouchers cause redemption timing and batch DoS, not a listed fund theft or loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-6 / `fTpb9jQyiAjg-2_bUzEFQ`
- Finding Title: Missing slippage bounds on L1GNS curation lifecycle burns lets MEV extract curator GRT
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `zero-slippage-gns-rollover`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged MEV leg, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, profitable sandwich feasibility not proven on deployed pools`
- Detailed Reason: The zero-minimum lifecycle burns are real and can expose owner-triggered migrations or deprecations to adverse curation execution. However, submission readiness depends on showing a concrete high-value subgraph and an executable sandwich that extracts over $1M after liquidity, tax, and curve constraints; without that, the impact mapping remains ambiguous.
- Code Evidence: `L1GNS.sendSubgraphToL2()` burns all `vSignal` with `minOut = 0`, and inherited `GNS.deprecateSubgraph()` and `GNS.publishNewVersion()` also call `curation().burn(..., 0)` with `publishNewVersion()` reminting via `curation.mint(..., 0)`.

### H-7 / `o8vp1anUo3H8qXm2HGs6x`
- Finding Title: Missing L1 sender allowlist lets any bridged deposit trigger L2GraphTokenGateway callhooks
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `safeguard-blocks`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: L1 gateway outboundTransfer enforces callhookAllowlist before any L2 finalize call with nonempty data can be created.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-8 / `Ec3H-wdfN6sIYqX0DfzoU`
- Finding Title: Live stake read lets disputed indexer reduce slashable stake before dispute acceptance
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Avoiding or shrinking a slash is deterrence loss, not listed user fund theft or loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-9 / `pE0GLsNwqCEwLvMTg5u0D`
- Finding Title: Query attestations can be replayed by new fishermen to repeatedly slash the same indexer
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Duplicate disputes can be filed but repeated slashing still requires arbitrator acceptance of copied evidence.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-10 / `tsLs6ez0cONVmhrdc7NwK`
- Finding Title: EpochManager.setEpochLength can retroactively advance epochs and allow runEpoch twice in one original epoch interval
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires governor parameter action and the direct effect is functional duplicate epoch execution.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-11 / `O-td7UjPoXlyloTjZXbt2`
- Finding Title: Wallet owner can replace manager and let beneficiary bypass vesting through fallback token transfers
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends on wallet owner manager replacement or owner-beneficiary collusion to bypass the wallet's own lock.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-12 / `ebboW6ut29b_kdFrSR_SO`
- Finding Title: Late delegators can capture historical staking rewards by joining before reward distribution
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `reward-dilution impact is not an exact listed row, >$1M direct user-fund theft not demonstrated`
- Detailed Reason: The live-share reward accounting is supported by the staking code, but this is a reward distribution/dilution issue rather than a clearly listed Immunefi impact for The Graph. The report does not establish significant direct user funds stolen from protocol smart contracts, so it should be rejected under the strict bounty rubric.
- Code Evidence: `StakingExtension._delegate()` mints shares immediately from current `pool.tokens/pool.shares`, and `Staking._collectDelegationQueryRewards()` plus `_collectDelegationIndexingRewards()` add rewards directly into the live pool.

### H-13 / `dvukhFdlwmZfqc3LuaZ7m`
- Finding Title: Late curators can sandwich Curation.collect to steal pending query-fee reserves from existing signal holders
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `jit-curation-fee-capture`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged mint-burn path, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, exploit depends on observable staking collect and profitable MEV ordering`
- Detailed Reason: L1 Curation does let a public minter receive immediately active signal before `collect()` grows reserves, then redeem after the reserve increase. The missing piece for a Valid Immunefi classification is concrete proof that this can steal significant (>$1M) user funds on deployed state, not just a theoretical pending-fee sandwich.
- Code Evidence: `Curation.collect()` only verifies `msg.sender == staking()` and adds fees to `curationPool.tokens`; `mint()` mints GCS before the reserve update and `burn()` redeems against `signalToTokens()` using the current pool reserve.

### M-14 / `HLVsPKpZUNaGNnSq345K5`
- Finding Title: Unchecked periods can unlock all managed GRT before endTime or make active locks unreleasable
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Requires a manager-created malformed lock schedule or beneficiary-owned lock rather than an unprivileged path against another current asset.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-15 / `wvp6vEeJaisgMYf53gBbg`
- Finding Title: Replayable query attestations allow duplicate fishermen to slash the same indexer repeatedly
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Copied attestation disputes do not slash by themselves and need arbitrator approval for each duplicate.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-16 / `6idoXnThGpSBJ-EPa3F8R`
- Finding Title: Governance-accepted 100% delegation tax bricks StakingExtension.delegate and redelegation flows
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted governance setting a boundary parameter and results in functional delegation DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-17 / `2xX-swYSsVN8VGmGmdKrT`
- Finding Title: Late delegators can sandwich collect and transferDelegationToL2 to siphon historical delegation rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `reward-dilution impact is not an exact listed row, bridge step does not create direct >$1M theft evidence`
- Detailed Reason: The L2 transfer path can convert live delegation shares after rewards have inflated `pool.tokens`, but the asserted harm is capture of historical delegation rewards rather than a listed direct theft impact. The extra bridge step does not remove the need to prove significant user funds stolen directly from protocol smart contracts.
- Code Evidence: `StakingExtension._delegate()` issues immediate shares, `Staking._collectDelegationQueryRewards()` adds rewards to live pool reserves, and `L1Staking._transferDelegationToL2()` computes `tokensToSend = delegation.shares * pool.tokens / pool.shares`.

### M-18 / `RLFXAUvWIHTZ_fnu6BpJ1`
- Finding Title: Delegator reward split uses live delegation pool state at collection time instead of accrual-time snapshots
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `unlisted reward-accounting impact, no demonstrated significant direct user-fund theft`
- Detailed Reason: The code distributes delegation rewards to the current pool rather than an accrual-time snapshot, but the bounty does not list medium reward-accounting dilution as a payable smart contract impact. Without a concrete >$1M direct theft path, this should not be submitted.
- Code Evidence: `Staking._collectDelegationIndexingRewards()` calculates the delegator share from live `pool.indexingRewardCut` and adds `delegationRewards` to live `pool.tokens`; no per-allocation or per-delegator snapshot is visible.

### H-19 / `pVM1_spOMfdp34-4q1AeQ`
- Finding Title: Late delegators can sandwich reward collection and bridge historical delegation rewards from L1Staking
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `reward capture is not an exact listed impact, >$1M direct theft not proven`
- Detailed Reason: The same live delegation-share issue is present, and the L2 transfer can remove the attacker's position after reward collection. Even so, the described loss is historical reward dilution and does not clearly satisfy The Graph's listed direct user-fund theft rows.
- Code Evidence: `StakingExtension._delegate()` mints immediately active shares, reward collection adds delegator rewards to `pool.tokens`, and `L1Staking._transferDelegationToL2()` bridges value computed from the inflated pool ratio.

### H-20 / `d9fHpGdoTBa4PKUJ90D_c`
- Finding Title: Domainless AllocationExchange voucher signatures can be replayed across exchange deployments to double-collect GRT
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `future-speculation`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Cross-deployment replay needs another funded compatible exchange with the same signer and no current target is shown.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-21 / `Vm7DYifsPbVOBHMz0xaxO`
- Finding Title: Disputed indexer can reduce live stake before acceptance to shrink or avoid slashing
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Slash reduction or avoidance is not a listed direct user fund theft or loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-22 / `fOOcoBnp5dXZARRW7F0mX`
- Finding Title: Collector can front-run matured PaymentsEscrow withdrawals and drain thawed escrow funds
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `escrow-thaw-available-balance`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, direct escrow balance at issue, potential listed critical row`
- Checklist Gates Failed: `attacker collector role and valid-claim model ambiguous, >$1M theft path not demonstrated`
- Detailed Reason: `PaymentsEscrow.collect()` can spend funds that `getBalance()` treats as thawing/unavailable, so the accounting concern is real. The submission risk is that exploitation appears to require being the tuple collector or having a valid collection route for that escrow, and the report does not yet prove a non-privileged attacker can drain over $1M rather than collect an otherwise authorized payment.
- Code Evidence: `PaymentsEscrow.thaw()` stores `tokensThawing` and `thawEndTimestamp`, `withdraw()` later pays the thawed amount, but `collect()` checks only `account.balance >= tokens`, subtracts from `balance`, and merely caps `tokensThawing` afterward.

### H-23 / `dUiQBJS_ifmyMZDJJg8P3`
- Finding Title: Same query attestation can be replayed by different fishermen to slash an indexer multiple times
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Multiple fishermen can open duplicate disputes but repeated harmful slashing depends on trusted arbitrator acceptance.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-24 / `_z9eHAOqQxc7tw62VOdvg`
- Finding Title: Stale l2Done is not reset on repeated L1 subgraph receive, permanently blocking L2 finalization
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `safeguard-blocks`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: L1GNS marks subgraphTransferredToL2 and prevents a second L1 send for the same subgraph ID.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### L-25 / `euxUG3JwToko9Ql-mZLP-`
- Finding Title: mint() accepts positive deposits that mint zero GCS in existing curated pools
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `configuration-nonissue`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires caller accepting zero output and is a user slippage footgun rather than a listed high impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-26 / `YcK3rO_6x_sv1qcuaa11M`
- Finding Title: Removed token destinations retain max allowance and can still pull locked wallet GRT
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Stale allowance exists only for previously approved protocol destinations and no unprivileged drain call is shown.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-27 / `g3DEuqLAr2RTGnVZ_SI4R`
- Finding Title: L1-transferred wallets can be initialized with invalid beneficiary or schedule and permanently lock bridged GRT
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `safeguard-blocks`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: L1 transfer tool derives wallet data from a validated lock and enforces a nonzero L2 beneficiary before bridging.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-28 / `fPN4offgE0-J1DJnGShk5`
- Finding Title: Router path can burn approved L2 GRT from arbitrary users and withdraw to attacker-controlled L1 recipient
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `safeguard-blocks`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Direct gateway withdrawals burn msg.sender and spoofed from values require the trusted Arbitrum router path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-29 / `HS7ij8NcVKVH9YjUejaMW`
- Finding Title: Live thawingPeriod changes let a provider and verifier extend existing delegators' exit lock in HorizonStaking._undelegate
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires service provider and verifier parameter action and the effect is extended withdrawal delay.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-30 / `SEb97mpjm7qIw65UJ78Cm`
- Finding Title: Removed token destinations retain max allowance and can drain GraphTokenLockWallet funds
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Removed destinations may retain allowance but exploitation requires a former destination contract exposing a usable pull path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-31 / `B1QFkkzj9DYXaowCh7Vf_`
- Finding Title: Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payout-destination`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: The RAV binds the payer, service provider, and data service but not the service provider's payout destination. A malicious data service named in a valid high-value RAV can call the in-scope collector directly with an attacker-controlled destination, consuming the aggregate and causing escrowed GRT to be paid away from the service provider; for balances above $1M this maps cleanly to the Critical direct-theft row.
- Code Evidence: `GraphTallyCollector._collect()` decodes `receiverDestination` from caller data and forwards it to `PaymentsEscrow.collect()`, while `_encodeRAV()` hashes only RAV fields and omits the destination; `SubgraphService.setPaymentsDestination()` shows the intended provider-controlled destination is separate from caller calldata.

### H-32 / `tM-CoEYZDT2wfiQ960uyv`
- Finding Title: RAVs omit paymentType, allowing collection from the wrong escrow bucket and consuming the intended claim
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: PaymentsEscrow balances are not keyed by paymentType and SubgraphService uses a fixed QueryFee type.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-33 / `6Dmb5MHowy4X9YIC_cTN6`
- Finding Title: Unchecked L1 subgraph ID aliasing overflows into the native L2 ID namespace
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Namespace collision and bridge lifecycle DoS do not clearly map to a listed fund theft or loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-34 / `jwIdvdMcGop_qgNSOYd8r`
- Finding Title: Duplicate query attestation disputes can replay the same evidence to slash an indexer multiple times
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Evidence replay only becomes harmful if the arbitrator accepts duplicate disputes.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-35 / `1Oi8TW8IXsPcmpzNCzrE4`
- Finding Title: GraphTokenLock.release ignores revokedAmount and can make released plus revoked exceed managedAmount
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Stated impact is accounting corruption and downstream view DoS, not a listed direct fund loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-36 / `DqituiIkPqFk-9Vz0uuJA`
- Finding Title: Truncated period duration allows GraphTokenLock beneficiaries to release all managed GRT before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The early unlock requires a malformed schedule created by the lock manager or already controlled by the beneficiary.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-37 / `YLx4Szz9XO4l7crTSUkEG`
- Finding Title: Late delegators can front-run fee collection in L1Staking.collect to capture historical delegation rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `reward-dilution impact not listed, significant direct user-fund theft not demonstrated`
- Detailed Reason: The collect path uses the current delegation pool when assigning query-fee rewards, but the impact remains late-entry reward dilution. The Graph bounty does not list this medium-style accounting issue unless it is proven to cause significant direct theft of user funds, which the finding does not do.
- Code Evidence: `Staking.collect()` calls `_collectDelegationQueryRewards()` after query rebate calculation, and that helper reads live `pool.queryFeeCut` and adds the delegator share into current `pool.tokens`.

### H-38 / `pOfr5KMT_5lFxRkf9mVXT`
- Finding Title: Fee sniping in L2Curation.collect lets same-transaction minters capture already-accrued query fees
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `jit-curation-fee-capture`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged mint-burn path, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, collect observability and profitability not proven`
- Detailed Reason: The mechanics match L2Curation's live reserve accounting, but a strict Immunefi Valid decision needs evidence of a significant deployed loss opportunity. This remains a plausible High economic-attack candidate pending a fork PoC or state analysis showing over $1M can be captured.
- Code Evidence: `L2Curation.collect()` increases `curationPool.tokens`; `mint()` immediately mints GCS priced before that increase; `burn()` then uses `signalToTokens()` against the enlarged reserve and unchanged total-signal eligibility model.

### H-39 / `h_vAbk2vdvEPQbccEZdHw`
- Finding Title: Zero-minimum L2GNS.publishNewVersion can burn curator backing and leave all name signal unredeemable
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `zero-vsignal-rounding-lock`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `fund-freeze/unredeemability impact not listed, owner-triggered action, no direct theft row`
- Detailed Reason: L2GNS can accept a zero-output remint while leaving `nSignal` outstanding, but the resulting unredeemability or value destruction does not match any listed The Graph Immunefi impact row. The program does not include permanent freezing or griefing rows for this bounty, so this is not submission-ready.
- Code Evidence: `L2GNS.publishNewVersion()` burns old `vSignal` with `minOut = 0` and remints with `minSignal = 0`; `GNS.nSignalToVSignal()` can then map nonzero `nSignal` to zero `vSignal`, and `L2Curation.burn()` rejects zero signal.

### H-40 / `umjoT_mS7MXkSxZMLIy-Z`
- Finding Title: Removed or old token destinations retain unlimited wallet allowance after revokeProtocol
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Allowance remains only on prior protocol destinations and the finding does not show an unprivileged transferFrom path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-41 / `NcpuLMyKuo880l0y6oqmP`
- Finding Title: Unchecked subgraph ID alias math can wrap L1 IDs into L2 native namespace
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Claimed namespace and bridge accounting corruption lacks a direct listed fund theft or loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-42 / `tjEsNRrIhxiRFNmDx7oi6`
- Finding Title: Inherited burn paths let holders reduce L2 GRT supply outside the bridge
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Holder or approved-spender burning breaks bridge accounting but does not itself show listed fund theft or loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-43 / `oA5-8p36HSkX_wVoUwknn`
- Finding Title: Denied subgraphs reclaim pre-denial rewards already snapshotted for active allocations
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Reward loss starts from a subgraph availability oracle denial and there is no independent attacker action.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-44 / `i1VPtkROwgKUtZ8kYUL4T`
- Finding Title: Permissionless stale allocation close can front-run indexer POI close and permanently forfeit rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Permissionless stale allocation close after maxAllocationEpochs is an explicit code path for stale allocations.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-45 / `GhuG5LBZRxCkptR7_jsC-`
- Finding Title: Zero-slippage curation burns let MEV force losses during L1GNS lifecycle migrations
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `zero-slippage-gns-rollover`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged MEV leg, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M loss not demonstrated, sandwich feasibility not validated on deployed state`
- Detailed Reason: The lifecycle functions really do force whole-position curation trades with zero minimums, but the finding still needs a concrete high-value migration/deprecation target and executable MEV analysis before it clearly satisfies the >$1M Immunefi impact criterion.
- Code Evidence: `GNS.publishNewVersion()` and `GNS.deprecateSubgraph()` use `curation.burn(..., 0)`, while `L1GNS.sendSubgraphToL2()` does the same before splitting migration proceeds.

### H-46 / `5SkoGbKdpm5OAUzjcRC-u`
- Finding Title: Removed token destinations retain max allowance and can drain GraphTokenLockWallet balances
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: A stale allowance alone is not a concrete drain without a callable former destination pull path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-47 / `oOnSznkq-ikH8HEjjDvBw`
- Finding Title: Removed token destinations keep max wallet allowances and can drain locked GRT after revocation
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Former destination allowance persists but exploitability depends on behavior of a removed trusted destination.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-48 / `hPmA41X7Gv-5BNaMRzCSj`
- Finding Title: Late delegator can front-run SubgraphService.collect and capture historical indexing rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `reward distribution impact not listed, direct >$1M user-fund theft not demonstrated`
- Detailed Reason: Horizon reward settlement reads the current delegation pool and can reward late shares, but the asserted loss is allocation reward dilution. The Graph's bounty rows do not cover this as a standalone medium/high reward-accounting issue without proof of significant direct user funds stolen.
- Code Evidence: `SubgraphService._collectIndexingRewards()` calls `_presentPoi()`, `AllocationManager._distributeIndexingRewards()` sends the live delegator cut to `HorizonStaking.addToDelegationPool()`, and `HorizonStaking.delegate()` creates immediately active shares.

### L-49 / `_0HWg2QfaUadZlFaexEyd`
- Finding Title: Related conflict dispute is marked Drawn without DisputeDrawn event in DisputeManager.drawDispute
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `external-system-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Impact is off-chain monitor or indexer desynchronization with no listed on-chain fund loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-50 / `Quc09KbutHkD5n-3xHiP5`
- Finding Title: Pre-curation frontrun DoS blocks L1GNS.publishNewVersion for target deployments
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Subgraph version censorship is functional DoS and does not map to a listed fund theft or loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-51 / `s3Jcahv1Z1ZKtUXwnOqA1`
- Finding Title: Dust pre-curation can permanently block GNS.publishNewVersion for a target deployment
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Blocking a chosen deployment hash is lifecycle DoS, not a listed direct smart-contract fund impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-52 / `LvSXBtfhMrPaWyLPL5LsI`
- Finding Title: Zero-min curation remint lets MEV sandwich GNS upgrades and extract curator value
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `zero-slippage-gns-rollover`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged MEV leg, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, profitable curation sandwich not proven`
- Detailed Reason: The old-deployment burn and new-deployment mint both use zero minimums, so a price manipulation around an owner upgrade is plausible. It remains Needs Review because the report does not yet prove a concrete deployed subgraph and manipulation path causing the required significant loss.
- Code Evidence: `GNS.publishNewVersion()` calls `curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0)` and then `curation.mint(_subgraphDeploymentID, tokensWithTax, 0)`.

### M-53 / `sy0gIS3xnXU4griUAX-Vq`
- Finding Title: Late delegators can front-run reward distribution and capture historical delegation rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `unlisted reward-accounting/dilution impact, no significant direct user-fund theft proof`
- Detailed Reason: Immediate delegation shares and live reward settlement are present, but this is the same historical reward capture issue. Under the strict The Graph rubric, it lacks an exact listed impact row and should not be submitted.
- Code Evidence: `StakingExtension.delegate()` pulls tokens and calls `_delegate()`, `_delegate()` mints shares immediately, and both staking reward helpers add newly distributed rewards to the current delegation pool.

### L-54 / `VWv7fUzI7itheEB82gMzD`
- Finding Title: HorizonStakingBase.getThawedTokens uses provision thawing state for delegation requests and can revert or misreport
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `external-system-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Impact is incorrect getter data for consumers and integrations, not a listed direct fund loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-55 / `ICHWsJrCQfEZOJnrkIIwm`
- Finding Title: Inherited burn functions let non-gateway callers reduce L2 supply outside bridgeBurn
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Voluntary or allowance-based token burning breaks bridge observability but is not a listed theft impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-56 / `74fz6Sq8txN88eCpDtx-d`
- Finding Title: Floor-rounded periods allow GraphTokenLock to release more than managedAmount before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Requires a non-divisible manager-created lock schedule and beneficiary control of that lock.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-57 / `tGL--FFG0yJrpcnUf20b8`
- Finding Title: Removed token destinations retain max allowances and can keep pulling locked GRT
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The removed destination must itself be able and willing to pull wallet tokens, which is not an unprivileged path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### L-58 / `frbAkGX2vz5p8pc9DDwzX`
- Finding Title: Existing Curation pools allow split dust mints to avoid curation tax due to floor rounding
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Dust-level tax rounding leakage is not a listed high or critical smart-contract impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-59 / `LW2vZUmlgJ660REGRS4O7`
- Finding Title: Just-in-time minting before Curation.collect lets attackers siphon pending query-fee reserves
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `jit-curation-fee-capture`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged mint-burn path, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, MEV profitability not proven`
- Detailed Reason: The L1 curation pool accounting allows same-window participation in collected fees, but the bounty requires significant direct user-fund loss. A submission would need deployed-state proof of a large pending fee collection and a profitable sandwich after tax and curve effects.
- Code Evidence: `Curation.collect()` increases `curationPool.tokens`; `Curation.mint()` mints GCS before reserve collection and `Curation.burn()` redeems signal against the post-collection reserve.

### M-60 / `xkzIZMiF7Ae_QNJbXsyRp`
- Finding Title: Unchecked alias arithmetic lets large L1 subgraph IDs wrap into the native L2 namespace
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Alias namespace corruption and possible transfer blocking do not clearly produce a listed fund impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-61 / `HQ5iIK4DLrc3zk9gdqq70`
- Finding Title: Just-in-time curation around Curation.collect lets MEV searchers steal query-fee reserve increases from existing curators
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `jit-curation-fee-capture`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged mint-burn path, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M threshold not shown, depends on observable authorized collection`
- Detailed Reason: The exploit pattern is supported by the Curation functions, but the report stops short of proving the Immunefi threshold or a reliable on-chain ordering opportunity. It is a candidate for further PoC work, not a Valid classification yet.
- Code Evidence: `Curation.collect()` is staking-only but only adds fees to pool reserves; public `mint()` and `burn()` use the live reserve and total GCS supply without a holder snapshot or cooldown.

### H-62 / `N_YWYOluJ5Prxux82Jm8I`
- Finding Title: Removed token destinations retain max allowances and can drain locked GraphTokenLockWallet funds
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Stale allowance persistence lacks a concrete public drain route through a removed destination.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-63 / `-E2VVCOUdoDBYwj_3QMN8`
- Finding Title: Dust stake front-run can brick an indexer's full L2 stake migration
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Public-mempool migration DoS and delay are not listed smart-contract fund theft or loss impacts.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-64 / `qRZ1FOv_Ar0vYaE-Yi6Xt`
- Finding Title: Invalid periods greater than duration bricks release and revoke during the lock schedule
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged lock creation with invalid schedule and the primary effect is temporary lock DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-65 / `33t6_voi7uHow1Olnjxad`
- Finding Title: Surplus tokens can be recorded as released after revoke, corrupting lock accounting
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Stated impact is accounting corruption and stuck future surplus, not a listed direct theft impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-66 / `jx8r2WRqMP3oMfMoqVngf`
- Finding Title: Flooring period duration allows full release before GraphTokenLockWallet.endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Early release needs a vulnerable schedule on a beneficiary-controlled lock rather than an external attacker path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-67 / `fzIAS0dOBY7zovwu_m82R`
- Finding Title: Query dispute evidence can be replayed by different fishermen to create multiple slashable disputes
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Repeated slashing from replayed evidence requires arbitrator acceptance of each duplicated dispute.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-68 / `1s4uC3ge2yUKETEUlxwC-`
- Finding Title: Replayable query attestations allow duplicate dispute rewards and repeated indexer slashing
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Duplicate dispute creation is possible but damaging resolution is controlled by the arbitrator.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-69 / `J92V_dG3IhhhL_JbTdGGR`
- Finding Title: Stale allocator blockAppliedTo ignored allows paused issuance to accrue in RewardsManager.updateAccRewardsPerSignal
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `future-speculation`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Exploit requires a configured stale or paused issuance allocator state not shown as current.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-70 / `GI6ghDWZ0DbbxkmP2EEdc`
- Finding Title: Lifecycle curation burns use zero slippage bounds, allowing MEV to extract curator value during upgrades or L2 migration
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `zero-slippage-gns-rollover`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged MEV leg, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M loss not demonstrated, profitable sandwich not proven`
- Detailed Reason: The code provides no minimum outputs for whole-position lifecycle burns, making adverse execution plausible. It should remain Needs Review until a fork PoC or deployed-state analysis demonstrates significant extractable curator value.
- Code Evidence: `GNS.publishNewVersion()`, `GNS.deprecateSubgraph()`, and `L1GNS.sendSubgraphToL2()` all route subgraph curation backing through `curation.burn(..., 0)`.

### M-71 / `-MTK0MGT9nO7ufWzFQ7V1`
- Finding Title: Live thawingPeriod changes can trap existing HorizonStaking delegators in a longer withdrawal delay
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires service provider and verifier parameter changes and results in withdrawal-delay extension.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-72 / `OJlNnyFbndyJ-aX8sExbp`
- Finding Title: Fee collect can be front-run to capture already-accrued curation fees in L2Curation.collect
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `jit-curation-fee-capture`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged mint-burn path, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, collection-ordering feasibility not proven`
- Detailed Reason: L2Curation's accounting allows a new GCS holder to participate in the next reserve increase, but the strict bounty question is whether this can currently steal significant user funds. The finding needs a high-value deployed target and profitable ordering proof before becoming submission-ready.
- Code Evidence: `L2Curation.collect()` adds fees to `pools[id].tokens`; `mint()` and `burn()` are public and `signalToTokens()` returns a pro-rata share of current pool tokens.

### H-73 / `jdzPwaYmWyMGmgtPKXrKl`
- Finding Title: Verifier reward path lets colluding verifier bypass HorizonStaking.slash thawing and extract provisioned GRT
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires colluding service provider and verifier using slash authority to bypass their own thawing restriction.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-74 / `o8ZxQ3YqGTr7FIb-YG_xl`
- Finding Title: Missing DisputeDrawn event for related conflict in DisputeManager.drawDispute desynchronizes dispute status indexers
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `external-system-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Event-consistency impact is off-chain processor desynchronization with no listed on-chain fund loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-75 / `dHE718r2of_6A8TKmSTns`
- Finding Title: Indexing dispute rewards can be stolen by frontrunning the first dispute for an allocation
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Fisherman reward race does not show listed direct loss of existing user funds.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-76 / `vhAujFKcOkl5LKwh5jTaQ`
- Finding Title: GraphTokenLockWallet.revokeProtocol cannot clear allowances for removed or old-manager destinations
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: revokeProtocol only clears current destinations, but no unprivileged pull path from old destinations is demonstrated.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-77 / `8W0iju22G-llzxhSwodlI`
- Finding Title: Same query attestation can be replayed by different fishermen to create multiple slashable disputes
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The code permits duplicate dispute IDs by fisherman but slashing still depends on arbitrator action.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### L-78 / `oM0G6My7r2aiLmLG54PMj`
- Finding Title: Inherited burn functions let L2GraphToken supply be destroyed outside bridgeBurn
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Token burning and bridge accounting drift do not by themselves show a listed user fund theft impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-79 / `gq6Q-0OujrjCdB2zidwVn`
- Finding Title: Indexer can change delegation reward cuts before settlement to redirect pending delegator rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-cut-settlement`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `normal indexer parameter authority, reward-share impact not listed, no unprivileged direct theft`
- Detailed Reason: An indexer setting its own delegation parameters is an intended participant permission, and the resulting pending-reward shift is not an exact listed bounty impact. Without a privilege bypass or significant direct user-fund theft, this should not be submitted.
- Code Evidence: `Staking.setDelegationParameters()` writes the caller indexer's live `indexingRewardCut` and `queryFeeCut`; later `_collectDelegationIndexingRewards()` and `_collectDelegationQueryRewards()` read those current values at settlement time.

### H-80 / `Ok1F_QWC5REA7Vu257cHi`
- Finding Title: RewardsManager accrues and mints rewards from stale paused issuance allocator rates
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `future-speculation`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The stale-rate mint path depends on a future or unproven paused allocator configuration.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-81 / `Iz7slvVRgZWDP-GJRd3Sz`
- Finding Title: Live curation state lets MEV redirect query-fee curation cut in SubgraphService._collectQueryFees
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `live-curation-status-payment-split`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged curation state change, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M direct loss not demonstrated, end-to-end collect/curation MEV path not proven`
- Detailed Reason: `SubgraphService` reads curation status at settlement time and can route a curation cut based on live state. The finding is plausible but still needs proof that an unprivileged searcher can redirect significant escrowed query fees to itself on deployed state rather than just change a payment split in a small or hypothetical case.
- Code Evidence: `SubgraphService._collectQueryFees()` passes `_curation().isCurated(subgraphDeploymentId) ? curationFeesCut : 0` into GraphTally data, then forwards received curator tokens to `_curation().collect()`.

### H-82 / `IzJRwjfGP8MLrXVsH-zpM`
- Finding Title: L2 token lock owner can replace the wallet manager and approve an arbitrary spender to drain locked GRT before maturity
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends on the lock owner replacing the wallet manager to bypass the wallet's own maturity gate.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-83 / `Cg-P2nt4RDg2y-BXltUrN`
- Finding Title: Floored periodDuration lets availableAmount exceed managedAmount before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Exceeding managedAmount requires a malformed schedule on a controlled lock and not an unprivileged victim path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### L-84 / `TNMAz3cX9t88fcgV4atgG`
- Finding Title: Delegation minimum is checked before tax, allowing active delegated positions below MINIMUM_DELEGATION
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Dust delegation accounting inconsistency is not a listed high or critical impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-85 / `ByZDTLDCFrgVnsuMdV4fD`
- Finding Title: Removed GraphTokenLockWallet token destinations keep unlimited allowance and can drain wallets after revocation
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Unlimited stale approvals are only to prior token destinations and the report lacks a public drain call.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-86 / `BGltva0zz65tEFYsF1FFj`
- Finding Title: Locked GRT can be provisioned to unallowed verifiers through HorizonStaking.provision and escaped via verifier slash rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `locked-verifier-bypass`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `requires authorized locked-wallet/service-provider action, slashing-based extraction, no unprivileged direct theft`
- Detailed Reason: This is the same whitelist bypass as H-4. The normal `provision()` path omits the locked-verifier check, but exploitation requires the locked wallet or its authorized operator to route its own locked stake to the attacker verifier and then use slashing rewards, which is excluded from a strict unprivileged direct-theft classification.
- Code Evidence: `HorizonStaking.provision()` calls `_createProvision()` directly; `provisionLocked()` requires `_allowedLockedVerifiers[verifier]`; `slash()` can transfer a verifier reward up to `prov.maxVerifierCut` after that provision exists.

### M-87 / `krGg_RX12KcU0dWwCrjDk`
- Finding Title: Partial pause bypass in StakingExtension.withdrawDelegated allows redelegation while delegation is stopped
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Pause-control bypass and increased exposure during incidents do not directly match a listed fund impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-88 / `lHrTkJ7wGETXjfF9G4yA6`
- Finding Title: Live thawingPeriod changes can extend existing delegators' withdrawal lock in HorizonStaking._undelegate
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires provider and verifier parameter changes and causes temporary withdrawal-lock extension.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-89 / `QVFoe8ghW2UZrsBc4YTzn`
- Finding Title: Stale L2 mint allowance can be frontrun to over-mint GRT and brick future L2 withdrawals
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `future-speculation`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The race requires a pending governance allowance decrease and a large executable withdrawal during that transition.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-90 / `NrfGQTmHOKIM9zqRbWg9m`
- Finding Title: Late delegators can join immediately before reward distribution and capture historical rewards
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `unlisted reward-dilution impact, no significant direct user-fund theft proof`
- Detailed Reason: The staking code does not snapshot delegation shares for reward accrual, but that alone is not one of The Graph's listed Immunefi impacts. This remains a non-submission reward-accounting issue absent a demonstrated >$1M direct theft path.
- Code Evidence: `StakingExtension._delegate()` immediately increases `pool.tokens` and `pool.shares`; `Staking._collectDelegationIndexingRewards()` and `_collectDelegationQueryRewards()` later add rewards into the same live pool.

### M-91 / `sdJ9UQByrjzzpc_0uxVhw`
- Finding Title: GraphTokenLock.release can count surplus as released managed tokens after revoke and break outstanding accounting
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Stated impact is invariant break and stuck surplus accounting, not a listed direct fund theft impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-92 / `vv3Gaw6yuhQyR6NSIc3Z0`
- Finding Title: Wallet owner can replace manager to approve a malicious token destination and drain locked GRT before vesting
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends on wallet owner replacing manager and authorizing a malicious destination for its own lock.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-93 / `dtq_DbgTj3v6jxNtmLscr`
- Finding Title: Removed token destinations keep unlimited wallet allowances because revokeProtocol only revokes current manager destinations
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Old-manager destination approvals may remain but exploitation depends on a removed destination exposing token pulls.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-94 / `ABhhch6RW3Cehd24Jeacd`
- Finding Title: Same-epoch allocations can be force-closed when maxPOIStaleness is zero or shorter than an epoch
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Premature allocation closure and service interruption are not listed direct fund theft or loss impacts.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-95 / `n3McbS5rJ865jkkMvndtV`
- Finding Title: Permissionless stale closure can terminate too-young SubgraphService allocations before first reward eligibility
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Allocation liveness DoS before reward eligibility does not clearly map to a listed fund impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-96 / `7RSC0BFXcdj5oyRxWspZQ`
- Finding Title: Late delegators can sandwich reward distribution and steal accrued delegation rewards in L1Staking pools
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `live-delegation-reward-shares`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists`
- Checklist Gates Failed: `reward capture not exact listed impact, >$1M direct theft not demonstrated`
- Detailed Reason: The public delegation and live pool reward distribution behavior exists, but the described outcome is redistribution of accrued delegation rewards. The Graph bounty does not pay this unless it is tied to significant direct user-fund theft, which is not established.
- Code Evidence: `StakingExtension._delegate()` computes shares from the live pool ratio, and `Staking._collectDelegationIndexingRewards()` plus `_collectDelegationQueryRewards()` credit rewards to the current pool reserve.

### M-97 / `tM0v7fZVgdvWcCC7BRCAL`
- Finding Title: Permissionless stake donations can front-run and DoS full L1 stake migration or unstake
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Stake migration or exit DoS is not one of the program's listed smart-contract impacts.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-98 / `_vLiS7aapNC9UBoxSffUM`
- Finding Title: Delegators can bypass the unbonding period after an indexer only partially migrated to L2
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Early withdrawal timing bypass does not itself show listed user fund theft or loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-99 / `j1POF82OvPKxWP90AphWq`
- Finding Title: Partial L2 stake transfer lets later non-bridge exit unlock delegations early
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Bypassing unbonding delay is a lockup-timing issue, not a listed direct fund loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-100 / `lYR8UbIMMGs8m8zLoAVTA`
- Finding Title: Post-revocation surplus lets GraphTokenLock.release over-increment releasedAmount and brick accounting views
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Reported impact is accounting-view DoS and surplus accounting breakage, not listed direct theft.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-101 / `cU-ypYxVo_gtLnb-Qkh8S`
- Finding Title: Zero-slippage curation rollover in GNS.publishNewVersion lets MEV extract curator value during upgrades
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `zero-slippage-gns-rollover`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged MEV leg, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M loss not demonstrated, deployed sandwich PoC missing`
- Detailed Reason: The inherited GNS upgrade path has zero slippage protection on both curation legs. It remains Needs Review because a Valid Immunefi classification requires showing a current high-value upgrade target and a manipulation path capable of crossing the significant-loss threshold.
- Code Evidence: `GNS.publishNewVersion()` burns all previous `vSignal` with `curation.burn(..., 0)` and remints with `curation.mint(..., 0)` while keeping name signal supply constant.

### M-102 / `m9HcS0JWZ5HI883rmR4iX`
- Finding Title: Pre-curation front-run permanently blocks GNS.publishNewVersion for the owner's target deployment
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Subgraph upgrade censorship is functional DoS and not a listed direct fund impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-103 / `cEalADCsJqmFziDLjjMdp`
- Finding Title: Zero-signal subgraphs cannot be migrated to L2 due to division by zero in L1GNS.sendSubgraphToL2
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Zero-signal migration DoS does not show listed direct user fund theft or loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-104 / `iZX9MsnPKZcfo1sxrLMde`
- Finding Title: L1GNS.sendSubgraphToL2 burns all curation signal with minOut=0, allowing MEV to extract migration value
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `zero-slippage-gns-rollover`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged MEV leg, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M migration loss not demonstrated, profitable curve manipulation not proven`
- Detailed Reason: `sendSubgraphToL2()` accepts any curation burn output, which can expose migration proceeds to MEV. It should not be marked Valid until a deployed-state PoC demonstrates significant extractable value under realistic curve and transaction-ordering constraints.
- Code Evidence: `L1GNS.sendSubgraphToL2()` calls `curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0)` and derives both `tokensForL2` and `withdrawableGRT` from that unbounded return value.

### M-105 / `p62u_-6ezJSlPmZHC1TP-`
- Finding Title: Indexing dispute slot can be consumed before valid evidence exists, permanently blocking later disputes
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Blocking later disputes and slashing is deterrence failure, not listed direct user fund loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-106 / `eG8JLb2VxlkBf7r8b3cNH`
- Finding Title: Indexing dispute reward can be stolen by frontrunning the allocation dispute slot
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: First-inclusion race for a fisherman reward is not listed direct loss of existing user funds.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-107 / `JhZOjmiCqmhtI8rAf45RI`
- Finding Title: Truncated period duration lets beneficiaries release locked GRT before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Bad period parameters must be created by the token-lock manager or already exist on the beneficiary's own wallet.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-108 / `Kv09RfcoY_LzDQYWDwDrh`
- Finding Title: GraphTokenLock releases all locked GRT before endTime when duration is not aligned with periods
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Early release is only reachable for a beneficiary-controlled lock with a non-aligned schedule.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-109 / `4s7e5qtLEM13jNTbwNvJh`
- Finding Title: GraphPayments.collect can strand GRT when dataService or receiverDestination is GraphPayments
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `configuration-nonissue`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires self-routing payout parameters to GraphPayments and is a configuration footgun.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-110 / `YjdZsvlnh_K-EsSgIcb0d`
- Finding Title: MEV delegation frontrun can siphon GraphPayments.collect receiver proceeds into an attacker-owned pool
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `live-graphpayments-delegation-split`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged delegation path, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M receiver loss not demonstrated, payment/delegation cut assumptions need fork validation`
- Detailed Reason: `GraphPayments.collect()` computes the receiver/delegation split from live HorizonStaking state and offers no bound to the payer or receiver, so late delegation can plausibly redirect part of a payment. The finding still needs end-to-end proof that this is not intended delegation economics and can currently steal significant receiver proceeds from deployed protocol contracts.
- Code Evidence: `GraphPayments.collect()` reads `getDelegationPool(receiver, dataService)` and `getDelegationFeeCut()` during execution, then sends `tokensDelegationPool` to `HorizonStaking.addToDelegationPool()` before paying the remaining receiver destination.

### H-111 / `u2iIZVIBB1bS5kZQBCykf`
- Finding Title: Zero-slippage version upgrade in L1GNS.publishNewVersion exposes all name curators to curation sandwich loss
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `zero-slippage-gns-rollover`
- Bounty Criteria Match: high (smart contract): An economic attack other than a basic 51% governance attack that could cause significant (>$1M) User funds to be lost or stolen directly from the protocol smart contracts
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged MEV leg, potential listed economic-impact row`
- Checklist Gates Failed: `>$1M impact not demonstrated, deployed sandwich PoC missing`
- Detailed Reason: The L1GNS inherited upgrade flow accepts zero minimums for the old-deployment burn and new-deployment mint, but the report does not yet prove the significant-loss criterion on deployed state. This should be reviewed further rather than submitted as Valid now.
- Code Evidence: `GNS.publishNewVersion()`, inherited by L1GNS, executes `curation.burn(..., 0)` followed by `curation.mint(..., 0)` and overwrites `subgraphData.vSignal` while `nSignal` remains constant.

### M-112 / `d3Pjzqnm0MTIWlQDNBNv-`
- Finding Title: Permissionless pre-curation can permanently block L1GNS.publishNewVersion for the intended deployment
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Version-upgrade censorship is not a listed direct smart-contract fund impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-113 / `_2pqZ29B1sJKnCNR6GyZn`
- Finding Title: Removed token destinations retain unlimited lock-wallet allowances after GraphTokenLockWallet.revokeProtocol
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Removed destination allowance remains but no concrete unprivileged drain mechanism is shown.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-114 / `1q6drsqs_SUrW2AdJ047I`
- Finding Title: Rounded period duration lets GraphTokenLock.release unlock all GRT before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The schedule rounding issue needs an affected beneficiary-controlled lock and no current external attacker path is shown.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-115 / `ehVpD-cV2madnpbmgU4kx`
- Finding Title: RAV signatures omit payment parameters allowing data service to redirect or skim collections
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payment-params`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: The signed RAV omits payment-affecting execution parameters, while `GraphTallyCollector` accepts those parameters from the data service caller and forwards them into escrow settlement. A malicious data service with a valid high-value RAV can set a 100% data-service cut and/or attacker payout destination, causing escrowed GRT to be stolen or misdirected; this is direct theft from an in-scope protocol smart contract when value exceeds $1M.
- Code Evidence: `GraphTallyCollector._collect()` decodes `dataServiceCut` and `receiverDestination` from `_data` and forwards `_paymentType` to `PaymentsEscrow.collect()`, while `GraphTallyCollector._encodeRAV()` hashes only `collectionId`, `payer`, `serviceProvider`, `dataService`, `timestampNs`, `valueAggregate`, and `metadata`; `GraphPayments.collect()` then pays the chosen cut/destination.

### H-116 / `VQKcGszzCjO6o-P5XLfDl`
- Finding Title: Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payout-destination`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: This is the receiver-destination subset of H-115 and is submission-ready on severity: the service provider identity is signed, but the actual payout address is not. A malicious data service can consume a valid aggregate and send the provider side to an attacker-controlled address, directly stealing escrowed GRT if the RAV/escrow value is significant.
- Code Evidence: `GraphTallyCollector._collect()` sets `receiver = signedRAV.rav.serviceProvider` but forwards caller-supplied `receiverDestination`; `_encodeRAV()` omits that field, and `GraphPayments.collect()` transfers remaining receiver proceeds to `receiverDestination` when it is nonzero.

### H-117 / `dc08N1iIOjknfncyCT9i2`
- Finding Title: RAV omits paymentType so GraphTallyCollector.collect can debit an unauthorized escrow bucket
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Payment type is not an escrow-account key and the normal SubgraphService path fixes QueryFee.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-118 / `3Jw5fNBgFZQIZXBWZOabx`
- Finding Title: Removed token destinations retain max allowance from token lock wallets
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Removed destinations retain approval but the finding does not show a callable spender path for an attacker.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-119 / `OASc81Cvao4n37EZtIodX`
- Finding Title: approveProtocol grants unlimited spending from revocable wallets despite protocol forwarding being disabled
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The wallet fallback does not categorically block revocable protocol calls and approved destinations still need callable pull logic.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-120 / `X0itUNIgi-JAjkzgLz1ZK`
- Finding Title: Revocable GraphTokenLockWallet can grant max token allowances and bypass locked-token transfer restrictions
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: approveProtocol alone does not move funds and the claimed bypass needs a destination contract that can pull wallet tokens.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-121 / `CQeJC-wExbcB_Xu6RzxSp`
- Finding Title: Rounded periodDuration can unlock all managed tokens before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: A beneficiary can only exploit a malformed schedule already created for that lock.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-122 / `2yrAtCL-ES-15uAfRlAEc`
- Finding Title: Rounded period duration unlocks all managed GRT before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The rounded schedule bug needs a vulnerable lock configuration and beneficiary access.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-123 / `ilToR6Ub4dn4RnDMJlOtI`
- Finding Title: Release after revoke can overcount surplus as scheduled GRT and underflow outstanding accounting
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Stated impact is surplus misclassification and accounting underflow, not a listed direct fund loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-124 / `lngDuit-0EI66fFHVws3t`
- Finding Title: Floored period duration can unlock the full lock balance before endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Full early release is limited to beneficiary-controlled locks with non-divisible schedules.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-125 / `cWVy3mtuEgtyKiYnTckyD`
- Finding Title: Surplus tokens can be counted as scheduled releases and make totalOutstandingAmount underflow
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Reported impact is accounting invariant break and view reverts, not a listed direct theft impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-126 / `dADqX6pseO-paTjLOPfd4`
- Finding Title: Non-divisible vesting periods let GraphTokenLock.release account and transfer more than managedAmount at endTime
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `privileged-precondition`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The endTime surplus release path requires beneficiary control and surplus in a malformed lock.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### C-127 / `tVCBeXMEBH5V6SCGf3URD`
- Finding Title: L1 wallet key squatting in L2GraphTokenLockManager.onTokenTransfer redirects later bridged lock funds
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `safeguard-blocks`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: L1 transfer tool sets l1Address to msg.sender, so an attacker cannot squat another L1 wallet key through the bridge.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-128 / `gDOfk3hDkW8rDnfKAsSqF`
- Finding Title: L2GraphTokenLockManager.onTokenTransfer lets bridged top-ups become pre-vesting surplus withdrawals
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Repeated lock deposits up to managedAmount do not create surplus and excess bridged tokens were already surplus on L1.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-129 / `Np7jVa0FkXPAmRDrKn-RO`
- Finding Title: Overfunded first L1 receipt creates immediate surplus in L2GraphTokenLockManager.onTokenTransfer
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: An amount above managedAmount is surplus rather than locked value and is withdrawable by the beneficiary by design.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-130 / `w8d_zw8BUDInxrq3a3Hak`
- Finding Title: Malformed L1 wallet data can initialize an L2 lock with endTime zero and release all locked GRT immediately
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `safeguard-blocks`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Bridge data is produced by L1 transfer tool from validated wallet fields, so endTime zero cannot be supplied normally.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-131 / `DM2-8G7j0aYaT8tU_ynnC`
- Finding Title: First bridge receipt can create immediately withdrawable surplus when amount exceeds managedAmount
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: First receipt surplus requires overfunding beyond managedAmount, which is not locked value stolen from another party.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-132 / `ILknBlfB8zW9j7GVsOuTE`
- Finding Title: AllocationExchange vouchers can be replayed across exchange deployments because signatures omit domain separation
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `future-speculation`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Domain replay needs another funded compatible AllocationExchange and matching authority not shown in current scope.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-133 / `x_vOtbCGV6Wcmp18hRMQd`
- Finding Title: Reauthorized signers resurrect stale vouchers because vouchers have no expiry or signer epoch
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `future-speculation`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Old vouchers revive only if governance later reauthorizes the same signer and stale vouchers exist unredeemed.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### M-134 / `nCwpdPsUcY2UVXUMAyKMh`
- Finding Title: AllocationExchange rejects documented Ethereum signed vouchers and accepts raw digests instead
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Signature-format mismatch is payment-settlement DoS, not a listed direct fund theft or loss impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-135 / `ByclhPFtjt77sfRBpEU6l`
- Finding Title: Bearer vouchers can be frontrun because AllocationExchange does not bind redemptions to the intended caller
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Vouchers are bearer by design and frontrunning only triggers the same Staking.collect to the signed allocation.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### H-136 / `gN3VVdG4Rhc4aEudFrjFy`
- Finding Title: Replayable AllocationExchange vouchers can drain funded deployments or revived signer epochs
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `future-speculation`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The replay scenarios require another compatible deployment or future signer reauthorization, not a current unprivileged path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`, and the stage 1 scope screen.

### L-137 / `Kh6-cI1bpC2qDbdBkKmKA`
- Finding Title: Authorized token selectors let beneficiaries bypass lock schedules through fallback forwarding
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted manager admin to authorize unsafe token selectors.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-138 / `18R6YuY2OOUXXDjxIYBuy`
- Finding Title: Manager can fund minimal proxies whose masterCopy has no executable code
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted manager owner to set a no-code master copy and fund unusable proxies.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-139 / `8pY7cdVd4449l4qS8uONz`
- Finding Title: Zero period duration bricks release and revoke during active lock schedules
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged invalid lock schedule creation and results in active-window release DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-140 / `BonWd42IiJTSCnuNvTt7b`
- Finding Title: Inherited minter role lets L2GraphToken supply be minted outside the bridge
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires a configured non-gateway minter role, which is privileged access.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-141 / `gQyL8CVcSsjv_Nex0mMD0`
- Finding Title: Division-by-zero schedule bricks GraphTokenLockWallet releases when periods exceed duration
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged creation of an invalid lock schedule and causes temporary release DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-142 / `uFZfooDyfYFfSCTHrAQDq`
- Finding Title: GraphTokenLockManager can deploy and fund unusable proxies when masterCopy has no code
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted manager owner setting a bad masterCopy.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-143 / `9Q_fWFi-oNqueSDsZdHdH`
- Finding Title: Zero period duration DoSes revocation and release during the full vesting window
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends on accepted invalid schedule setup by privileged lock roles.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### H-144 / `8kMMYiBCkFq88yaCWsttI`
- Finding Title: Authorized provision operator can re-register an indexer and redirect SubgraphService rewards to an attacker payment destination
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires malicious or compromised authorized provision operator access.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-145 / `V1Y52aZlyMmnUmdeghPEi`
- Finding Title: GraphTokenLock active schedule functions revert when periods exceed duration
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends on privileged or role-controlled invalid schedule setup and causes custody DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-146 / `C7Kw-_9H62nDaSE5z9HOO`
- Finding Title: Invalid period count can brick active GraphTokenLock releases and revocations
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged malformed lock creation and causes temporary freeze.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-147 / `I4edxIRUaIYDUphW3Qx2l`
- Finding Title: Initializer accepts schedules with zero periodDuration, breaking release and revoke during the active window
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged invalid schedule initialization and results in active-window DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-148 / `Nmnjdcf3bctp8i9rgiUoi`
- Finding Title: GraphTokenLock schedules with periods greater than duration brick release paths until endTime
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires owner or manager controlled malformed lock creation.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-149 / `0SVeziUG9ox6dvPkTZRxJ`
- Finding Title: Authorized token transfer selector lets beneficiaries bypass the lock schedule through fallback forwarding
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged manager allowlist configuration of unsafe token transfer selectors.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-150 / `CuyXqVpz5cARhVC0xvGUY`
- Finding Title: Non-contract masterCopy lets manager fund unusable minimal proxies
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted manager owner setting a non-contract masterCopy.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-151 / `_xEF4gfOWYe92b0eM_58j`
- Finding Title: Authorized token transfer selectors let beneficiaries bypass lock release schedule
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged manager configuration to authorize raw token transfer or approval selectors.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-152 / `lbkf7hb4jCCpkaL2Fl1_v`
- Finding Title: Locks with periods greater than duration revert schedule and release functions
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged lock creation with periods greater than duration.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-153 / `OyjjjybTLwykOvm4_lPz5`
- Finding Title: Invalid periods greater than duration brick active GraphTokenLock release and revoke paths
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires owner or manager controlled malformed schedule setup.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-154 / `CWBfshE4WhMIUcG8wpf6-`
- Finding Title: Non-contract masterCopy can create funded no-op proxies that strand managed GRT
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted owner configuration of a no-code implementation.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-155 / `FJYxqrUIyJm3074O84czj`
- Finding Title: Governance-accepted 100% curation tax disables new public curation and burns existing-pool deposits for zero signal
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted governor setting 100 percent curation tax.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-156 / `PFUTQNSd9LwIqsZSGdu7z`
- Finding Title: Invalid period configuration makes active locks divide by zero and blocks releases until endTime
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged invalid schedule initialization.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-157 / `g_tI0b9meutRyfrDxHjoC`
- Finding Title: GraphTokenLock schedules with periods greater than duration revert during release and revoke calculations
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires owner or manager controlled malformed lock schedule.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-158 / `9juKsdihssjpdAyWtLiB1`
- Finding Title: GraphTokenLock schedules with periods greater than duration revert until endTime
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged schedule creation with zero period duration.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-159 / `NxXWOE1Y0mdPCGhTgzIGr`
- Finding Title: Zero periodDuration bricks active GraphTokenLock releases and revocations when periods exceed duration
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires trusted role creating a malformed lock schedule.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-160 / `IgtuPlhBeBya-D_ZFBl5X`
- Finding Title: GraphTokenLock schedules with periods greater than duration revert during active release window
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Depends on role-controlled malformed schedule setup and causes temporary lock DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-161 / `-o4btWto8SniNvUeJ7TBg`
- Finding Title: Invalid periods parameter can brick active GraphTokenLockWallet releases with division by zero
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires owner or manager controlled invalid schedule creation.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-162 / `KWKKleZuh8CB9Ht91kuyy`
- Finding Title: Invalid period count can divide by zero and brick GraphTokenLock release schedules
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires manager owner or admin creating a malformed wallet.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### L-163 / `237WhvPdUNGlJuTgfk9dX`
- Finding Title: Invalid period count can brick active token lock schedule functions in GraphTokenLock
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires privileged invalid period configuration and causes temporary active-schedule DoS.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.

### M-164 / `KY4DD7oqc-P5mYc1RIKRS`
- Finding Title: Shortening thawing periods can strand already-mature thaw requests behind older unexpired requests
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `known-issue-or-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: HorizonStaking comments explicitly document traversal stopping before later expired thaw requests after shortening.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`.
