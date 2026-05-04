# legion-protocol-contracts Three-Shot Stage 1 Scope Screen

Status: In progress
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/legion-protocol-contracts/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`
- `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| C-1 | Replayable position-transfer signature can burn a later position and erase the recipient position accounting | Exclude | High | closed-bounty-report | Closed bounty issue #88 covers replayable transfer authorization theft across these position-transfer paths. |
| C-2 | Inherited native ETH release() bypasses LegionLinearVesting cliff and allows premature withdrawal | Exclude | High | closed-bounty-report | Closed bounty issue #80 covers the same native ETH release before cliff in LegionLinearVesting. |
| H-3 | Reusable investment signatures allow uncapped repeated bids in LegionSealedBidAuctionSale.invest | Exclude | High | known-issue-or-oos | Bounty known issues explicitly exclude signature reuse when investing with the same signature. |
| C-4 | Replayable transfer authorization corrupts investor position ownership in LegionFixedPriceSale | Exclude | High | closed-bounty-report | Closed bounty issue #88 specifically covers replayed position-transfer authorization in fixed price sales. |
| C-5 | Position merge can credit a victim's position to another account while burning an unrelated position | Exclude | Medium | trusted-role-oos | Standalone path requires a trusted signer or Legion caller to authorize an inconsistent from and positionId tuple. |
| C-6 | Stale transfer authorizations can merge another user's position and burn the signer's current position | Exclude | High | closed-bounty-report | Closed bounty issue #88 covers missing transfer signature consumption across sale, raise, and position manager paths. |
| H-7 | Stale distributor claim signatures can overclaim and exhaust tokens for later valid claimants | Exclude | High | closed-bounty-report | Closed bounty issue #86 covers LegionTokenDistributor over-claims beyond total allocation causing later claims to fail. |
| H-8 | Stale position-transfer authorization can merge another holder's position and burn an unrelated position | Exclude | High | closed-bounty-report | Closed bounty issue #88 covers stale position-transfer authorization replay and resulting NFT or entitlement theft. |
| C-9 | Replayable position transfer authorization can burn a later investor position and permanently freeze entitlements | Exclude | High | closed-bounty-report | Closed bounty issue #88 covers replayable position-transfer authorization with permanent entitlement loss. |
| C-10 | Replayable position transfer authorizations can re-steal returned investor positions | Exclude | High | closed-bounty-report | Closed bounty issue #88 covers cyclic replay after a position returns to the original signer. |
| H-11 | Fee-on-transfer bid tokens over-credit deposits and can leave later investor refunds insolvent | Exclude | High | known-issue-or-oos | Bounty known issues explicitly exclude lack of support for fee-on-transfer and rebasing tokens. |
| H-12 | Fee-on-transfer askToken makes distributor insolvent and freezes later valid token claims | Exclude | High | known-issue-or-oos | Bounty known issues explicitly exclude lack of support for fee-on-transfer and rebasing tokens. |
