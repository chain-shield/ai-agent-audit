# bbp-public-assets Three-Shot Stage 3 run-001

Status: Completed
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/bbp-public-assets/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-bounty-rules.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-severity-rubric.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-immunefi-poc-runtime.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/bbp-public-assets/bbp-public-assets-scope.txt`
- `/Users/apmfree/Desktop/Audit/bbp-public-assets-f3e56d/bbp-public-assets/README.md`

## Per-Finding Validation

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
