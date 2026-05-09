# thegraph Three-Shot Stage 3 run-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/thegraph/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/thegraph-023b7fdc8558`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-bounty-rules.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-severity-rubric.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-immunefi-poc-runtime.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/thegraph/thegraph-scope.txt`

## Per-Finding Validation

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
