# ssv-network Three-Shot Stage 3 run-001

Status: Completed
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ssv-network/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-immunefi-bounty-rules.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-immunefi-severity-rubric.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-immunefi-poc-runtime.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-scope.txt`
- `/Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network/README.md`

## Per-Finding Validation

### H-1 / `QG_ICWJZysYLIuVfb6oiu`
- Finding Title: Permissionless fee sync can permanently strand staking ETH reward dust in SSVStaking._syncFees
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `staking-reward-rounding`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, unprivileged caller, code path exists`
- Checklist Gates Failed: `non-dust impact, exact bounty row, submission materiality`
- Detailed Reason: The accumulator rounding behavior exists, but the claimed impact is explicitly dust-scale reward loss rather than a clear non-dust permanent freezing of unclaimed yield. The bounty has a High row for permanent freezing of unclaimed yield, but the round-3 rejection rules exclude dust impacts, and this path only strands sub-share precision increments while the protocol already uses 100,000-wei ETH packing and documented reward truncation. This should not be submitted as an Immunefi bounty finding.
- Code Evidence: `SSVNetwork.syncFees` delegates to the staking module, and `SSVStaking._syncFees` sets `sp.ethDaoBalance`, computes `newFeesWei`, floors `(newFeesWei * PRECISION) / totalStaked`, then advances `stakingEthPoolBalance` to `current` in `contracts/SSVNetwork.sol:205` and `contracts/modules/SSVStaking.sol:183`. ETH accounting is packed at `ETH_DEDUCTED_DIGITS = 100_000` in `contracts/libraries/SSVCoreTypes.sol:19` and `contracts/libraries/SSVPackedLib.sol:59`.

### C-5 / `xV5HkuxJmnZDvpMrER57I`
- Finding Title: Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `liquidation-threshold-rounding`
- Bounty Criteria Match: critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- Checklist Gates Passed: `in-scope asset, unprivileged liquidate path, possible listed impact, code path exists, cannot concretely falsify before PoC`
- Checklist Gates Failed: `mainnet-state fork PoC still required for final submission confidence`
- Detailed Reason: The code does use separate flooring for accrued balance decay and combined flooring for liquidation threshold math, and a liquidation accepted only because of that boundary would transfer the remaining cluster balance to the liquidator. The main counterargument is that the repository specification documents the combined `(burnRate + networkFee)` threshold, but that does not concretely falsify the finding because the live balance-decay path uses a different rounding surface and the liquidator payout is the full remaining cluster balance, not merely the rounding delta. Promote this to the submission-candidate path so R5/R6 can require a mainnet-fork PoC to prove or falsify serious permissionless damage.
- Code Evidence: `SSVClusters.liquidate` updates cluster balance and then checks `isLiquidatableWithEB` before `_executeLiquidation` transfers `balanceLiquidatable` to the caller in `contracts/modules/SSVClusters.sol:31` and `contracts/modules/SSVClusters.sol:603`. `ClusterLib.isLiquidatableWithEB` combines rates before division in `contracts/libraries/ClusterLib.sol:67`, while `ClusterLib.updateBalanceWithEB` floors network and operator fee components separately in `contracts/libraries/ClusterLib.sol:306`; the documented threshold formula also uses the combined rate in `docs/SPEC.md:949`.

### H-7 / `zzWCpNrNBYw-Cdvv_tgOw`
- Finding Title: Frequent EB updates undercharge cluster fees by repeatedly flooring vUnit fee math
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `eb-fee-rounding`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, permissionless update call, code path exists`
- Checklist Gates Failed: `non-dust impact, attacker-controlled oracle cadence, exact bounty row`
- Detailed Reason: The repeated-flooring behavior exists, but each valid EB update can discard less than one packed ETH unit per fee component, and execution still requires a committed latest oracle root and valid proof. That is a dust-scale undercharge of operator/network fees, not a clear theft of funds, theft of unclaimed yield, permanent freezing, insolvency, gas theft, or other listed Immunefi impact. Under the strict round-3 rules, dust and weak materiality must be rejected.
- Code Evidence: `SSVClusters.updateClusterBalance` requires committed latest roots, Merkle proof validation, update-frequency checks, and EB bounds before applying ETH accounting in `contracts/modules/SSVClusters.sol:349`. `_applyClusterFeeUpdates` separately floors `(idxNet * units) / BPS_DENOMINATOR` and `(idxOp * units) / BPS_DENOMINATOR`, then multiplies by `ETH_DEDUCTED_DIGITS` in `contracts/modules/SSVClusters.sol:462`; `ETH_DEDUCTED_DIGITS` is 100,000 wei in `contracts/libraries/SSVCoreTypes.sol:19`.

### M-9 / `JDUeqSWcB7YKK1PeXVxfX`
- Finding Title: Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `state-hash-frontrun-griefing`
- Bounty Criteria Match: medium (smart contract): Theft of gas; medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- Checklist Gates Passed: `in-scope asset, unprivileged attacker, listed medium impact, code path exists, no privileged role needed`
- Checklist Gates Failed: `-`
- Detailed Reason: A public liquidation can be frontrun by any address that submits a tiny ETH `deposit` against the same current cluster state, changing only the stored cluster hash and causing the pending liquidation to revert before it reaches the liquidatability check. The attacker does not need privileged access or oracle control, and repeated dust deposits can waste liquidator gas and delay liquidation of an unhealthy cluster. This maps to the program's Medium griefing and theft-of-gas rows, not to Critical theft or freezing of funds.
- Code Evidence: `SSVNetwork.deposit` and `SSVNetwork.liquidate` delegate to the in-scope cluster module in `contracts/SSVNetwork.sol:271` and `contracts/SSVNetwork.sol:294`. `SSVClusters.deposit` accepts any caller, validates the supplied cluster hash, adds `msg.value`, and stores the new hash without a minimum deposit or health check in `contracts/modules/SSVClusters.sol:186`; `SSVClusters.liquidate` first calls the same hash validation in `contracts/modules/SSVClusters.sol:31`, and `ClusterLib.validateHashedCluster` reverts on stale cluster data in `contracts/libraries/ClusterLib.sol:131`.
