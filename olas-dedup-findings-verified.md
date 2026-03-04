# 2026-01-olas - Deduplicated Findings Report
## Commit hash: 9e059a602c29b79f898718058ffc88c0d1d39609

## Summary (filtered: ERC20 edge-case + user-error + privileged-governance-risk + strict external-dependency speculation removed)
- **Total Unique Valid Findings**: 57
- **High Severity (Valid)**: 30 unique findings
- **Medium Severity (Valid)**: 27 unique findings

### Number of Findings (After Deduplication)
- H: 30 unique (from 66 High findings with status **Valid**)
- M: 27 unique (from 61 Medium findings with status **Valid**)

**Total Valid Findings: 57 unique findings**
**Total Findings in Source Report: 205 (H-1 to M-205, including 6 Low severity)**

### Remaining Non-Valid Findings (Need Further Review)
- **LowSeverityDueToLowImpact (need further review)**: 9 findings (M-164, M-166, L-167, M-171, M-172, L-173, M-174, M-175, M-177)
- **InvalidByDesign (need further review)**: 1 finding (M-178)
- **LowSeverityDueToRareLikelihood (need further review)**: 8 findings (M-179, M-185, H-186, M-187, M-188, M-189, M-194, H-195)
- **InvalidNotExploitable (need further review)**: 1 finding (H-201)
- **InvalidOutOfScope (need further review)**: 1 finding (M-205)

**Note**: In the **source report**, findings with status **Valid** span IDs 1-127; findings 128-205 are all marked as non-Valid (need further review). This file is a filtered copy where all ERC20 edge-case findings, user-error-dependent findings, privileged-governance-risk findings, and strict external-dependency speculation findings have been removed. The status breakdown above reflects what remains in this filtered file.

---

## High Severity Findings

### [H-1]. Permanent DoS of Staking via Malicious Service Multisig Upgrade (Gas Griefing)
**Pattern**: Dos
**Duplicates**: H-36
**Status**: Valid
**Privilege**: Permissionless

### [H-4]. Token-Secured Services bypass Slashing mechanism due to decoupled bond accounting
**Pattern**: AccountingInvariantViolation
**Duplicates**: H-53
**Status**: Valid
**Privilege**: Permissionless

### [H-5]. Drained service slashed funds are permanently locked in Treasury due to missing accounting update
**Pattern**: AccountingInvariantViolation
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresAdminRole

### [H-7]. UniswapPriceOracle.getPrice returns inverted price (Quote/Base vs Base/Quote)
**Pattern**: Oracle
**Duplicates**: H-73, H-75
**Status**: Valid
**Privilege**: Permissionless

### [H-8]. Contract non-functional due to missing setters for allowlist configuration
**Pattern**: DoS / Initialization Missing
**Duplicates**: H-19, H-28, H-29, H-83
**Status**: Valid
**Privilege**: Permissionless

### [H-12]. DoS in ProcessBridgedDataGnosis due to unsafe assembly casting in VerifyBridgedData
**Pattern**: Unsafe assembly type casting
**Duplicates**: H-15, H-24, H-25, H-26, H-42, H-47, H-56, H-67, H-79, H-82, H-106
**Status**: Valid
**Privilege**: Permissionless

### [H-23]. StakingBase validation bypass via Proxy codehash check allows reward theft
**Pattern**: AccessControlOrAuthByPass
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-35]. Sybil Attack on `IDF` Calculation via `numNewOwners` Manipulation
**Pattern**: Oracle
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-37]. Bypass of Liveness Check via Malicious Multisig Implementation allowing Reward Theft
**Pattern**: AuthByPass
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-66]. Service Multisig Hijacking via Unchecked Data Payload in RecoveryModule.create
**Pattern**: AuthByPass
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-45]. Accounting corruption via reentrancy in _withdraw function
**Pattern**: CEIViolation
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-49]. DoS and fund loss via permissionless `collectFees` burning/sweeping tokens
**Pattern**: Attack is rational because attacker profits from delay/failure
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-55]. Operators can bypass token bond requirement in ServiceManager by communicating directly with ServiceRegistry
**Pattern**: IncentiveMisalignmentOrGameTheory
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-58]. Service Owner can trap operator bonds by changing token address in update()
**Pattern**: GlobalParamMidFlowManipulation
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-59]. Lack of slippage protection in V3 buyback enables value extraction via sandwich attacks
**Pattern**: FlashLoanEconomicManipulation
**Duplicates**: H-61, H-69, M-71, H-72, H-85, M-87, H-90, H-93, H-95, M-97, M-100, H-118, M-119
**Status**: Valid
**Privilege**: Permissionless

### [H-64]. UniswapPriceOracle.validatePrice fails to enforce TWAP validation allowing price manipulation
**Pattern**: OracleUsingDEXorTWAP
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-88]. Pool-age/staleness check bypass via incorrect `observationIndex` usage in `checkPoolAndGetCenterPrice`
**Pattern**: OracleUsingDEXorTWAP
**Duplicates**: H-124
**Status**: Valid
**Privilege**: Permissionless

### [H-89]. Oracle check fails open on TWAP failure allowing 100% price manipulation on young pools
**Pattern**: OracleUsingDEXorTWAP
**Duplicates**: H-86, H-125
**Status**: Valid
**Privilege**: Permissionless

### [H-120]. Excessive hardcoded `MAX_ALLOWED_DEVIATION` (10%) enables MEV extraction while passing deviation checks
**Pattern**: OracleUsingDEXorTWAP
**Duplicates**: M-127
**Status**: Valid
**Privilege**: Permissionless

### [H-126]. Broken Price Deviation Check in checkPoolAndGetCenterPrice due to Variable Shadowing
**Pattern**: Oracle Price Manipulation
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-70]. Broken Uniswap V3 integration due to incorrect IRouterV3 interface definition
**Pattern**: Standard Violation
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-84]. Permanent DoS of Tokenomics checkpointing via front-running donations
**Pattern**: GovernanceFrontrunDoS
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-103]. GuardCM fails to restrict calls to non-privileged addresses enabling arbitrary execution
**Pattern**: AccessControlOrAuthByPass
**Duplicates**: M-104, H-105
**Status**: Valid
**Privilege**: RequiresRole

### [H-107]. Operator Whitelist bypass in registerAgentsWithSignature
**Pattern**: AccessControlOrAuthByPass
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [H-109]. Service Owner can drain Operator funds by manipulating bond requirements before executing `registerAgentsWithSignature`
**Pattern**: ConfigFootgun
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [H-110]. DoS in recoverAccess due to quadratic memory expansion in loop
**Pattern**: GasGriefBlockLimit
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-111]. Permanent DoS of recoverAccess via uncooperative operator preventing state transition
**Pattern**: Incentives / Game Theory (Governance / Griefing DoS)
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [H-112]. Malicious agents can permanently block recovery and slashing by disabling RecoveryModule
**Pattern**: IncentiveMisalignmentOrGameTheory
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [H-113]. Permanent DoS of RecoveryModule via Safe Owners List Inflation
**Pattern**: Dos
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [H-114]. Permanent DoS of Oracle updates due to strict slippage check against high-inertia lifetime average
**Pattern**: Dos
**Duplicates**: H-116, H-117
**Status**: Valid
**Privilege**: Permissionless

---

## Medium Severity Findings

### [M-6]. Missing Deadline in Operator Signatures
**Pattern**: PermitDeadlineBypass
**Duplicates**: M-11
**Status**: Valid
**Privilege**: RequiresRole (Note: M-11 was marked Permissionless in the original report)

### [M-14]. `changeRanges` causes persistent DoS and misroutes funds for single-sided positions
**Pattern**: Dos
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [M-16]. DoS of Service Redeployment via malicious Operator in unbond
**Pattern**: PullorPushPaymentbugs
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-17]. Missing meaningful deadline + floating slippage bounds in liquidity management functions
**Pattern**: SlippageMissingOrInsufficient
**Duplicates**: M-60, M-78, M-122, L-168, M-169, M-170
**Status**: Valid
**Privilege**: RequiresRole

### [M-18]. NeighborhoodScanner chooses wrong optimization mode for unbalanced amounts causing capital inefficiency
**Pattern**: Optimization Logic Error
**Duplicates**: M-48
**Status**: Valid
**Privilege**: Permissionless

### [M-22]. GuardCM `_verifySchedule` ignores ETH value allowing potential Timelock drainage
**Pattern**: AccessControlOrAuthByPass
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [M-27]. Evicted services bypass reward forfeiture specification
**Pattern**: MaturityorGatingByPass
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-30]. Lack of try-catch in `redeem` causes permanent stuck funds for reverting targets
**Pattern**: GriefableCallbacks
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-80]. DoS of L2 Dispenser via Reverting Staking Target in Batch
**Pattern**: ExternalCallAfterStateChange
**Duplicates**: M-81
**Status**: Valid
**Privilege**: Permissionless

### [M-33]. Permissionless `transfer` enables DoS of Buyback mechanism
**Pattern**: AccessControl
**Duplicates**: M-94
**Status**: Valid
**Privilege**: Permissionless

### [M-38]. Burner.burn() lacks idempotency enabling front-running DoS/griefing of keepers
**Pattern**: DoubleExecutionOrReplay
**Duplicates**: M-39
**Status**: Valid
**Privilege**: Permissionless

### [M-41]. Non-compliant EIP-1271 implementation prevents smart contract operators
**Pattern**: StandardViolation
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-44]. DoS of Service Deployment via Sentinel Address Registration
**Pattern**: StandardViolation
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-46]. Rewards DoS and Loss via Push Payment Griefing in `_withdraw`
**Pattern**: PullorPushPaymentbugs
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-50]. Front-running `collectFees` Causes Revert and Blocks Governance/Maintenance Batches
**Pattern**: CheapGriefingOrDosProfit
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-52]. Permissionless `collectFees` enables griefing of fee compounding strategy in `changeRanges`
**Pattern**: IncentiveMisalignmentOrGameTheory
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-54]. Spot-price based slippage bounds in liquidity operations enable sandwich attacks
**Pattern**: SlippageMissingOrInsufficient
**Duplicates**: M-51, M-77, H-76
**Status**: Valid
**Privilege**: RequiresRole

### [M-57]. DoS in `create` due to front-running of deterministic deployment
**Pattern**: Deterministic deployment DoS via front-run pre-creation
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-62]. Arithmetic Underflow or Oracle DoS due to Unit Mismatch in maxSlippage
**Pattern**: IntegerMath
**Duplicates**: M-121
**Status**: Valid
**Privilege**: Permissionless

### [M-63]. DoS in `convertToV3` due to Token Ordering Mismatch
**Pattern**: StandardViolation
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [M-65]. DoS in UniswapPriceOracle via same-block interaction
**Pattern**: Dos
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-68]. Stale multisig authorization in IdentityRegistryBridger allows evicted service owners to act on behalf of agents
**Pattern**: AccessControlOrAuthByPass
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

### [M-101]. Permanent DoS of `linkServiceIdAgentIds` automation via failing service registration
**Pattern**: Head of Line Blocking / Denial of Service
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-102]. IdentityRegistryBridger.linkServiceIdAgentIds fails to update stale multisig mappings causing DoS
**Pattern**: UnincentivizedMaintenanceOrKeeperlessProgress
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-108]. Indefinite signature validity and missing revocation mechanism allows service owners to bond operators against their will
**Pattern**: IncentiveMisalignmentOrGameTheory
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-115]. Flawed cumulative moving average math in updatePrice leads to price divergence
**Pattern**: PricePrecisionOrRoundingError
**Duplicates**: None
**Status**: Valid
**Privilege**: Permissionless

### [M-123]. Liquidity Provision at Manipulated Price via convertToV3
**Pattern**: FlashLoanEconomicManipulation
**Duplicates**: None
**Status**: Valid
**Privilege**: RequiresRole

---

## Potentially Invalid/Out of Scope Findings (Need Further Review)

### Findings 128-205 Analysis:
All findings from 128-205 are marked with various "Invalid" or "LowSeverity" statuses in the original report. These require further review to confirm their validity.

### [M-164]. Proxy fallback exceeds 2300 gas stipend causing DoS on ETH transfers
**Pattern**: Dos
**Duplicates**: None
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless

### [M-166]. ETH Dust Permanently Locked in Treasury due to Donation Truncation
**Pattern**: RoundingError
**Duplicates**: L-165
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless

### [L-167]. DoS of fee collection via front-running due to strict zero-value check
**Pattern**: IncentiveGameTheory
**Duplicates**: None
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless

### [M-177]. Lack of user-specified slippage or deadline bounds exposes protocol to market movements
**Pattern**: SlippageMissingOrInsufficient
**Duplicates**: None
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: RequiresRole

### [M-171]. DoS of valid SEND_MESSAGE payloads due to incorrect minimum data length check in ProcessBridgedDataWormhole
**Pattern**: Dos
**Duplicates**: None
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless

### [M-172]. Authorization bypass in `validationRequest` allows unauthorized calls for Agent ID 0
**Pattern**: AuthByPass
**Duplicates**: None
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless

### [L-173]. Swapped Arguments in Error definitions lead to misleading revert logs
**Pattern**: Custom
**Duplicates**: None
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless

### [M-174]. Permanent skipping of undeployed services in `linkServiceIdAgentIds` enables Griefing
**Pattern**: Dos
**Duplicates**: None
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless

### [M-175]. Silent failure in recoverAccess due to unchecked return value
**Pattern**: UncheckedLowLevelCallResults
**Duplicates**: M-176
**Status**: LowSeverityDueToLowImpact (need further review)
**Privilege**: Permissionless (Note: M-176 was marked RequiresRole in the original report)

### [M-178]. Minority Censorship via Permissionless Proposal Front-running
**Pattern**: GovernanceFrontrunDoS
**Duplicates**: None
**Status**: InvalidByDesign (need further review)
**Privilege**: Permissionless

### [M-179]. Supply Cap logic violates monotonicity invariant causing cap decrease at Year 10
**Pattern**: AccountingInvariantViolation
**Duplicates**: M-180
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: Permissionless

### [M-185]. Protocol actions DoS due to ZeroValue check in _checkTokensAndRemoveLiquidityV2
**Pattern**: Dos
**Duplicates**: None
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: Permissionless

### [H-186]. Missing Source Chain ID Verification in `_receiveMessage`
**Pattern**: CrossChainMessageSpoofing
**Duplicates**: None
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: Permissionless

### [M-187]. Inflation cap violation due to effectiveBond reset in updateInflationPerSecondAndFractions
**Pattern**: GlobalParamMidFlowManipulation
**Duplicates**: None
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: RequiresAdminRole

### [M-188]. Refunded staking incentives are permanently erased during inflation update
**Pattern**: AccountingInvariantViolation
**Duplicates**: None
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: RequiresAdminRole

### [M-189]. Authorization bypass due to selector aliasing on short calldata
**Pattern**: Zero-selector allowlist edge-case exploiter
**Duplicates**: M-190, M-191
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: Permissionless

### [M-194]. TWAP formula causes asymmetric price adaptation (Ratchet Effect)
**Pattern**: Oracle
**Duplicates**: None
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: Permissionless

### [H-195]. Incorrect TWAP calculation reduces historical weight on price increases enabling manipulation
**Pattern**: PricePrecisionOrRoundingError
**Duplicates**: None
**Status**: LowSeverityDueToRareLikelihood (need further review)
**Privilege**: Permissionless

### [H-201]. Validation bypass in `_verifyBridgedData` due to dirty high bits in assembly reads
**Pattern**: UnsafeAssembyTypeCasts
**Duplicates**: None
**Status**: InvalidNotExploitable (need further review)
**Privilege**: Permissionless
**Note**: This may be a duplicate of H-12 group if it's actually exploitable

### [M-205]. Tokenomics.checkpoint fails to reduce effectiveBond when actual inflation is lower than estimated
**Pattern**: AccountingInvariantViolation
**Duplicates**: None
**Status**: InvalidOutOfScope (need further review)
**Privilege**: Permissionless

---

## Deduplication Summary
 
### Major Duplicate Groups (includes invalid/OOS triage overrides):
Note: Some groups below include duplicates that were marked as potentially invalid/out of scope in the original report (need further review). Those are still grouped here when the underlying root cause appears identical.
1. **Slippage Protection Missing / Ineffective (Buyback swaps)** (15 findings → 1 unique): H-59, H-61, H-69, M-71, H-72, H-85, H-86, M-87, H-90, H-93, H-95, M-97, M-100, H-118, M-119 - **OUT OF SCOPE - found in V12**
2. **Unsafe Assembly Casting/Dirty Bits** (12 findings → 1 unique): H-12, H-15, H-24, H-25, H-26, H-42, H-47, H-56, H-67, H-79, H-82, H-106 **INVALID** future speculation, solidity 0.8.30 compiler handles this case

-> 3. **Slippage Protection Missing / Ineffective (Liquidity operations)** (3 findings → 1 unique): M-54, M-51, H-76 - **VALID - NO V12**

4. **Missing Deadline + Floating Slippage in Liquidity Ops** (8 findings → 1 unique): M-17, M-60, M-77, M-78, M-122, L-168, M-169, M-170 **DUP**
5. **TWAP/Oracle Manipulation** (6 findings → 4 unique): H-64, H-88, H-89, H-124, H-125, H-126 **OUT OF SCOPE - found in V12**
6. **Missing Configuration Setters** (5 findings → 1 unique): H-8, H-19, H-28, H-29, H-83 ** INVALID by design**
7. **Oracle Price Inversion** (3 findings → 1 unique): H-7, H-73, H-75 **Low/QA**
8. **GuardCM Access Control Bypass** (3 findings → 1 unique): H-103, M-104, H-105 **INVALID - governance risk** out of scope
 9. **Oracle DoS due to TWAP Math** (3 findings → 1 unique): H-114, H-116, H-117 **OUT OF SCOPE - found in V12**

-> 10. **Reverting Target DoS** (2 findings → 1 unique): M-80, M-81 **Valid - NO V12**

11. **Hardcoded Price Deviation Tolerance** (2 findings → 1 unique): H-120, M-127 **Dup**

-> 12. **Gas Griefing DoS** (2 findings → 1 unique): H-1, H-36 **Valid - NO V12**
-> 13. **Token Bond Bypass** (2 findings → 1 unique): H-4, H-53 **Valid - NO V12**

14. **Signature Deadline Missing** (2 findings → 1 unique): M-6, M-11 **Invalid - user error**
15. **Permissionless Transfer DoS** (2 findings → 1 unique): M-33, M-94 **Invalid - by design**
16. **Optimization Logic Error** (2 findings → 1 unique): M-18, M-48 **Invalid - admin avoidable**

### New Unique Findings from 51-100 (Valid Status):
20. **H-107**: Operator Whitelist bypass **Low/QA**
21. **H-109**: Service Owner bond manipulation **Invalid - user error**
22. **H-110**: DoS via quadratic memory expansion **Invalid - attack precondition impossible**
23. **H-111**: DoS via uncooperative operator **Invalid - user error**
24. **H-112**: Recovery blocking via module disable **Invalid - by design**
25. **H-113**: DoS via Safe owners inflation **dup of invalid H-110**
26. **M-101**: DoS via failing service registration **out of scope**
27. **M-102**: Stale multisig mapping DoS **out of scope**
28. **M-108**: Indefinite signature validity **Low/QA - user error**
29. **M-115**: Flawed cumulative average math **dup of H-114**
30. **M-123**: Liquidity provision at manipulated price **Dup of H-125**

### Potentially Invalid/Out of Scope Findings from 101-150 (Need Further Review):
31. **M-179**: Supply cap becomes non-monotonic at year 10 (+ M-180) **InvalidFutureSpeculation**
32. **M-185**: ZeroValue check DoS **Low/QA**
33. **H-186**: Missing chain ID verification **Future speculation**
34. **M-187**: Inflation cap violation **Low/QA**
35. **M-188**: Refunded incentives erasure **Low/QA**
36. **M-189**: Selector aliasing bypass (+ M-190, M-191) **Low/QA**
37. **M-194**: TWAP ratchet effect **dup of H-194**
38. **H-195**: TWAP calculation manipulation **Low/QA** 
39. **H-201**: Assembly validation bypass (possibly duplicate of H-12) **Dup of H-12**
40. **M-205**: EffectiveBond reduction failure **Low/QA**

### Findings Marked Invalid/Out of Scope (128-205):
- **LowSeverityDueToLowImpact (need further review)** (9 findings): **M-164, M-166, L-167, M-171, M-172, L-173, M-174, M-175, M-177**
- **InvalidByDesign (need further review)** (1 finding): M-178
- **LowSeverityDueToRareLikelihood (need further review)** (8 findings): **M-179, M-185, H-186, M-187, M-188, M-189, M-194, H-195**
- **InvalidNotExploitable (need further review)** (1 finding): H-201
- **InvalidOutOfScope (need further review)** (1 finding): M-205

---

## Statistics - FILTERED DEDUPLICATION (All 205 Findings, with triage removals)

### Valid Findings (IDs 1-127 in the source report):
- **Original valid findings**: 127 findings
- **Unique valid findings remaining in this filtered file**: 57
- **Removed during triage (additional unique valid groups)**: 2 (M-3/M-21, M-31/M-32)
- **High severity (Valid)**: 66 → 30 unique
- **Medium severity (Valid)**: 61 → 27 unique

### Non-Valid Findings (IDs 128-205, need further review):
- **Count**: 78 findings
- **Status labels**: see “Findings by Status” sections at top (all marked “need further review”)

### Overall Statistics (Complete Report 1-205):
- **Total findings in report**: 205 (79 High, 120 Medium, 6 Low)
- **Valid findings**: 127 (IDs 1-127)
- **Non-Valid / potentially invalid / out of scope (need further review)**: 78 (IDs 128-205)
- **Unique valid findings after dedup (remaining in this filtered file)**: 57

### Deduplication Efficiency (Valid findings only):
- **Valid findings (remaining here)**: 127 → 57 unique (~55% reduction, includes triage removals)
- **Largest valid duplicate group**: Buyback slippage protection (15 findings → 1 unique)
- **Second largest (valid)**: Unsafe assembly (12 findings → 1 unique)

