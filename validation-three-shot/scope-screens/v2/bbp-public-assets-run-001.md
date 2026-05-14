# bbp-public-assets Three-Shot Stage 1 Scope Screen

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
| C-1 | Fee-on-transfer collateral can mint full USDe while custodians receive less collateral | Keep | High | in-scope | EthenaMinting is in scope and claimed over-issuance maps to protocol insolvency with no decisive OOS blocker. |
| M-2 | Nonce bitmap truncates uint256 nonces to 64 bits causing distinct orders to invalidate each other | Keep | Medium | in-scope | EthenaMinting is in scope and order invalidation can map to listed griefing pending deeper exploit review. |
| M-3 | Full-restricted stakers can bypass withdrawal restrictions through StakedUSDeV2.unstake | Exclude | High | impact-out-of-scope | Bypassing a blacklist freeze is not a listed smart contract impact such as theft or freezing of funds. |
| H-4 | Late deposits during reward vesting steal unclaimed yield from existing sUSDe stakers | Keep | High | in-scope | StakedUSDe reward accounting is in scope and the claimed impact maps directly to theft of unclaimed yield. |
| H-5 | Late deposits during reward vesting steal unclaimed yield from existing StakedUSDeV2 stakers | Keep | High | in-scope | StakedUSDeV2 is in scope and the claimed impact maps directly to theft of unclaimed yield. |
| C-6 | Fee-on-transfer collateral redemptions burn full USDe while beneficiary receives less collateral | Keep | High | in-scope | EthenaMinting is in scope and short redemption delivery can map to direct loss of user funds. |
| H-7 | Zero supply during reward vesting lets first new depositor capture all leftover USDe yield | Keep | High | in-scope | StakedUSDeV2 is in scope and the claimed capture of leftover rewards maps to theft of unclaimed yield. |
| H-8 | Late depositors can capture already-funded unvested rewards in StakedUSDe | Keep | High | in-scope | StakedUSDe is in scope and the claimed impact maps directly to theft of unclaimed yield. |
| H-9 | Late stakers can front-run StakedUSDe.transferInRewards to capture unclaimed yield from existing stakers | Keep | High | in-scope | StakedUSDe is in scope and the claimed impact maps directly to theft of unclaimed yield. |
| M-10 | Direct USDe donation to an empty StakedUSDeV2 can brick future deposits through MIN_SHARES | Keep | Medium | in-scope | StakedUSDeV2 is in scope and the claimed deposit DoS can map to listed griefing or smart contract liveness impact. |
| H-11 | Late deposits during reward vesting steal unclaimed yield from existing StakedUSDe holders | Keep | High | in-scope | StakedUSDe is in scope and the claimed impact maps directly to theft of unclaimed yield. |
| M-12 | Dust share holder can block final withdrawals via StakedUSDe._checkMinShares | Keep | High | in-scope | StakedUSDe is in scope and blocked withdrawals can map to listed temporary freezing or griefing impact. |
| H-13 | Last staker can redeem during vesting and brick StakedUSDe deposits with unclaimable rewards stranded | Keep | High | in-scope | StakedUSDe is in scope and stranded rewards plus deposit DoS map to listed freezing of unclaimed yield and liveness impacts. |
| H-14 | Stale minting contract authority lets a removed operator mint USDe from distributor funds after migration | Exclude | High | trusted-role-oos | Exploitation still requires MINTER_ROLE or collusion with a privileged minter on the old minting contract. |
| H-15 | Cooldown changes in EthenaLPStaking.updateStakeParameters retroactively extend active withdrawals | Exclude | High | trusted-role-oos | The freeze depends on the owner changing a delegated cooldown parameter and has no non-privileged attacker path. |
| H-16 | Old minting contract keeps distributor signer and allowances after StakingRewardsDistributor mint-contract rotation | Exclude | High | trusted-role-oos | Exploitation still requires MINTER_ROLE or collusion with a privileged minter on the old minting contract. |
| L-17 | Full-restricted address can bypass staking ban by depositing USDe for an unrestricted receiver | Exclude | High | impact-out-of-scope | The stated bypass undermines restrictions but does not claim a listed theft, freezing, liveness, gas, or return impact. |
