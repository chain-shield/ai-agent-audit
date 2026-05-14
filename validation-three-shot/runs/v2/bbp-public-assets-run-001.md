# bbp-public-assets Three-Shot Validation run-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/bbp-public-assets/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`
Stage 1 scope screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/scope-screens/v2/bbp-public-assets-run-001.md`
Stage 2 bounty exploitability screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/token-screens/v2/bbp-public-assets-run-001.md`
Stage 3 final validation run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/stage3-runs/v2/bbp-public-assets-run-001.md`

## Assembly Summary

- Excluded at stage 1 (scope / known issue): `5`
- Excluded at stage 2 (bounty exploitability): `9`
- Fully validated at stage 3: `3`

## Per-Finding Validation

### C-1 / `p8uGob3LjBhYLShSbIN0_`
- Finding Title: Fee-on-transfer collateral can mint full USDe while custodians receive less collateral
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `practical-route-unproven`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Requires MINTER_ROLE execution and a fee-on-transfer asset already supported; current scope shows no such asset or unprivileged way to add one.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### M-2 / `mwJxMX3nWLUyQBG68VFI6`
- Finding Title: Nonce bitmap truncates uint256 nonces to 64 bits causing distinct orders to invalidate each other
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `practical-route-unproven`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: A victim's colliding order can only exist if the same benefactor or delegate signed both nonces; no unprivileged attacker can create or consume a target order.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### M-3 / `2brJpjt0oM1-AOnBmJ3c-`
- Finding Title: Full-restricted stakers can bypass withdrawal restrictions through StakedUSDeV2.unstake
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Bypassing a blacklist freeze is not a listed smart contract impact such as theft or freezing of funds.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`.

### H-4 / `ZvancpJ6l3DyWLqr3Xkl0`
- Finding Title: Late deposits during reward vesting steal unclaimed yield from existing sUSDe stakers
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Rewards are intentionally streamed through share price and current tests treat mid-vesting deposits as fair; no prior-holder snapshot entitlement exists.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### H-5 / `EEBhzRa4U33LvA2ZwroR9`
- Finding Title: Late deposits during reward vesting steal unclaimed yield from existing StakedUSDeV2 stakers
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: StakedUSDeV2 inherits the tested share-price vesting model; late deposits share future vesting rather than violating a snapshot entitlement.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### C-6 / `vpr73qOdd3B0JtWI1OE98`
- Finding Title: Fee-on-transfer collateral redemptions burn full USDe while beneficiary receives less collateral
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `practical-route-unproven`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Requires REDEEMER_ROLE execution and a fee-on-transfer collateral already supported; no current unprivileged route to force a victim redemption is shown.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### H-7 / `LKzCcTxhNcEQszVVHD1uh`
- Finding Title: Zero supply during reward vesting lets first new depositor capture all leftover USDe yield
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: OZ 4.9.5 virtual asset math does not mint 1:1 into zero-supply positive-asset vaults; meaningful capture would require infeasible deposit size.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### H-8 / `jzQSoBIDdLn6j4JuheKLN`
- Finding Title: Late depositors can capture already-funded unvested rewards in StakedUSDe
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The contract deliberately excludes unvested rewards from share price until they vest; tests exercise this as fair mid-vesting accounting.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### H-9 / `1TR_tQ8nm7i1y2XiB8tDO`
- Finding Title: Late stakers can front-run StakedUSDe.transferInRewards to capture unclaimed yield from existing stakers
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: A deposit before reward funding is a current shareholder by design, and remaining vesting is shared by current shares under the tested share-price model.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### M-10 / `Rxt5N1widPYf1llRNMoqZ`
- Finding Title: Direct USDe donation to an empty StakedUSDeV2 can brick future deposits through MIN_SHARES
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `erc4626-empty-vault-donation`
- Bounty Criteria Match: medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- Checklist Gates Passed: `in-scope asset, code path exists, permissionless direct transfer possible`
- Checklist Gates Failed: `current exploitability, attacker-controlled prerequisite, deployed-state impact`
- Detailed Reason: The empty-vault donation behavior is real in code, but it is not currently exploitable against the deployed in-scope sUSDe vault by an unprivileged attacker. A read-only mainnet check of `0x9d39a5de30e57443bff2a8307a4256c8797a3497` returned a very large nonzero `totalSupply`, so the cheap 1 USDe donation path only works in a zero-supply or pre-seeding state that the attacker cannot force. Under this bounty's strict rules, a hypothetical future empty-vault liveness grief does not satisfy a submission-ready listed impact.
- Code Evidence: `contracts/contracts/StakedUSDe.sol` computes `totalAssets()` from the raw USDe balance less unvested rewards, blocks rescuing `asset()` in `rescueTokens()`, and runs `_checkMinShares()` after deposits. `contracts/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol` prices initial shares through virtual asset/share conversion, which creates the reported zero-supply donation behavior, but the live vault is not in that state.

### H-11 / `Ta6V-f3SZ4ShmDQ9e51Ko`
- Finding Title: Late deposits during reward vesting steal unclaimed yield from existing StakedUSDe holders
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `bug-does-not-exist`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: This duplicates the intended share-price vesting behavior; current code and tests do not define rewards as owed only to holders present at funding.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`, and the stage 1 scope screen.

### M-12 / `7DRdpVApA8wuFLR28wQOR`
- Finding Title: Dust share holder can block final withdrawals via StakedUSDe._checkMinShares
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `min-share-dust-dos`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, code path exists, permissionless share ownership possible`
- Checklist Gates Failed: `current exploitability, non-dust impact, material listed impact`
- Detailed Reason: The minimum-share check can revert a final exit that would leave a nonzero supply below `MIN_SHARES`, but the submit-ready impact is too narrow and state-dependent. On the current deployed vault, supply is far above `MIN_SHARES`, and an attacker cannot force the system into the final-holder state; even then, the practical residual block is bounded around the minimum-share dust threshold rather than a material temporary freeze of user funds. The user-provided rejection rules explicitly exclude dust and temporary nuisance cases, so this should not be submitted.
- Code Evidence: `contracts/contracts/StakedUSDe.sol` defines `MIN_SHARES = 1 ether`, `_withdraw()` calls `super._withdraw()` and then `_checkMinShares()`, and `_checkMinShares()` reverts only when total supply is nonzero and below `MIN_SHARES`. `contracts/contracts/StakedUSDeV2.sol` routes `cooldownAssets()` and `cooldownShares()` through the same `_withdraw()` path while current direct `withdraw()` and `redeem()` are disabled when cooldown is on.

### H-13 / `U8Fri0BIaSpSUs_mC_jiU`
- Finding Title: Last staker can redeem during vesting and brick StakedUSDe deposits with unclaimable rewards stranded
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `zero-supply-vesting-stranding`
- Bounty Criteria Match: high (smart contract): Permanent freezing of unclaimed yield
- Checklist Gates Passed: `in-scope asset, code path exists, listed impact type`
- Checklist Gates Failed: `current exploitability, attacker-controlled last-staker prerequisite, mainnet-fork PoC path`
- Detailed Reason: This is the strongest of the three mechanically because unvested rewards can be left behind when supply reaches zero, and the claimed impact aligns with the high-severity unclaimed-yield row. It still fails strict round 3 eligibility because the exploit requires the attacker to be the last staker while rewards are vesting, which is not an attacker-controlled condition on the live in-scope vault; current read-only calls show large nonzero supply and active cooldown exits. A local fresh-deployment PoC would not be enough for this deployed-asset bounty, so the finding is risky to submit as-is.
- Code Evidence: `contracts/contracts/StakedUSDe.sol` lets `transferInRewards()` start vesting, excludes `getUnvestedAmount()` from `totalAssets()`, allows `_withdraw()` to reduce supply to exactly zero because `_checkMinShares()` only rejects nonzero sub-minimum supply, and blocks rescuing USDe through `rescueTokens(asset())`. Once vesting finishes, OpenZeppelin ERC4626 conversion prices new deposits against stranded assets with zero real shares, causing zero or sub-minimum shares.

### H-14 / `9vVf32KBgvKp8hMjfbds8`
- Finding Title: Stale minting contract authority lets a removed operator mint USDe from distributor funds after migration
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Exploitation still requires MINTER_ROLE or collusion with a privileged minter on the old minting contract.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`.

### H-15 / `FpCz4HhIbSyW5lCPs_x0S`
- Finding Title: Cooldown changes in EthenaLPStaking.updateStakeParameters retroactively extend active withdrawals
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: The freeze depends on the owner changing a delegated cooldown parameter and has no non-privileged attacker path.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`.

### H-16 / `v0uwBw3mnOJukPFGMStUQ`
- Finding Title: Old minting contract keeps distributor signer and allowances after StakingRewardsDistributor mint-contract rotation
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Exploitation still requires MINTER_ROLE or collusion with a privileged minter on the old minting contract.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`.

### L-17 / `b5GO4Co88EstY_x_w82uD`
- Finding Title: Full-restricted address can bypass staking ban by depositing USDe for an unrestricted receiver
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `impact-out-of-scope`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: The stated bypass undermines restrictions but does not claim a listed theft, freezing, liveness, gas, or return impact.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`, `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`.
