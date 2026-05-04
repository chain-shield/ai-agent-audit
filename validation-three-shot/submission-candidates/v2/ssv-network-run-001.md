# ssv-network Three-Shot Submission Candidates run-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/ssv-network-run-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/ssv-network-run-001.md`

## Canonicalization Summary

- Candidate H/M findings before R4: `2`
- Kept after R4 canonicalization: `2`
- Dropped by R4 cleanup: `0`
- Dropped finding ids: `-`

## Root Cause Groups

- `cluster-hash-frontrun-griefing`: M-9
- `liquidation-threshold-rounding`: C-5

## Submission Candidates

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
