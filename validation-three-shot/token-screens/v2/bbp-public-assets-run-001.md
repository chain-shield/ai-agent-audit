# bbp-public-assets Three-Shot Stage 2 Bounty Exploitability Screen

Status: In progress
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

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| C-1 | Fee-on-transfer collateral can mint full USDe while custodians receive less collateral | Exclude | High | practical-route-unproven | Requires MINTER_ROLE execution and a fee-on-transfer asset already supported; current scope shows no such asset or unprivileged way to add one. |
| M-2 | Nonce bitmap truncates uint256 nonces to 64 bits causing distinct orders to invalidate each other | Exclude | High | practical-route-unproven | A victim's colliding order can only exist if the same benefactor or delegate signed both nonces; no unprivileged attacker can create or consume a target order. |
| H-4 | Late deposits during reward vesting steal unclaimed yield from existing sUSDe stakers | Exclude | High | bug-does-not-exist | Rewards are intentionally streamed through share price and current tests treat mid-vesting deposits as fair; no prior-holder snapshot entitlement exists. |
| H-5 | Late deposits during reward vesting steal unclaimed yield from existing StakedUSDeV2 stakers | Exclude | High | bug-does-not-exist | StakedUSDeV2 inherits the tested share-price vesting model; late deposits share future vesting rather than violating a snapshot entitlement. |
| C-6 | Fee-on-transfer collateral redemptions burn full USDe while beneficiary receives less collateral | Exclude | High | practical-route-unproven | Requires REDEEMER_ROLE execution and a fee-on-transfer collateral already supported; no current unprivileged route to force a victim redemption is shown. |
| H-7 | Zero supply during reward vesting lets first new depositor capture all leftover USDe yield | Exclude | High | weak-poc-path | OZ 4.9.5 virtual asset math does not mint 1:1 into zero-supply positive-asset vaults; meaningful capture would require infeasible deposit size. |
| H-8 | Late depositors can capture already-funded unvested rewards in StakedUSDe | Exclude | High | bug-does-not-exist | The contract deliberately excludes unvested rewards from share price until they vest; tests exercise this as fair mid-vesting accounting. |
| H-9 | Late stakers can front-run StakedUSDe.transferInRewards to capture unclaimed yield from existing stakers | Exclude | High | bug-does-not-exist | A deposit before reward funding is a current shareholder by design, and remaining vesting is shared by current shares under the tested share-price model. |
| M-10 | Direct USDe donation to an empty StakedUSDeV2 can brick future deposits through MIN_SHARES | Keep | High | rare-but-real-state | Zero supply is allowed; a direct USDe transfer makes totalAssets positive, normal deposits mint below MIN_SHARES and asset rescue is blocked. |
| H-11 | Late deposits during reward vesting steal unclaimed yield from existing StakedUSDe holders | Exclude | High | bug-does-not-exist | This duplicates the intended share-price vesting behavior; current code and tests do not define rewards as owed only to holders present at funding. |
| M-12 | Dust share holder can block final withdrawals via StakedUSDe._checkMinShares | Keep | High | current-exploit | A dust holder can make a victim full redeem leave nonzero supply below MIN_SHARES, causing _checkMinShares to revert without privileged access. |
| H-13 | Last staker can redeem during vesting and brick StakedUSDe deposits with unclaimable rewards stranded | Keep | High | rare-but-real-state | The last staker can burn totalSupply to zero while rewards are unvested; later the stranded USDe blocks normal deposits and cannot be rescued. |
