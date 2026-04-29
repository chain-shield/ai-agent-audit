# 2026-04-monetrix Three-Shot Submission Candidates run-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/2026-04-monetrix-run-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/2026-04-monetrix-run-001.md`

## Canonicalization Summary

- Candidate H/M findings before R4: `33`
- Kept after R4 canonicalization: `9`
- Dropped by R4 cleanup: `24`
- Dropped finding ids: `M-1, M-3, M-6, M-7, M-8, M-10, M-11, M-12, H-13, M-14, M-16, M-17, M-18, M-20, M-21, M-22, M-25, M-26, M-27, M-28, M-29, M-31, M-32, M-36`

## Root Cause Groups

- `blp-supply-registration-postcondition`: M-15
- `bridge-back-free-balance-mismatch`: M-19
- `bridge-back-held-balance-mismatch`: M-24, M-29 -> 4YjJddXWcRv9EM5yYCoju
- `bridge-back-supplied-balance-mismatch`: M-23, M-28 -> ixjYBbG7rWa4eVh74Lapg
- `empty-supply-reroute-bypass`: M-10 -> -, H-30
- `live-escrow-balance-distribution-dos`: M-32 -> -
- `live-inject-yield-sniping`: M-1 -> -, M-16 -> MQPNhFyzi1LA8-UlwLy-c, M-17 -> MQPNhFyzi1LA8-UlwLy-c, M-27 -> MQPNhFyzi1LA8-UlwLy-c
- `live-supply-cap-sniping`: H-4
- `live-yield-escrow-balance`: M-3 -> -
- `multisig-principal-returnability`: M-5, M-6 -> vzb5HZneiQ6_eF3s_2gUG
- `non-atomic-hedge-postcondition`: M-2, M-14 -> tRgLb0L5rgUEVRSxwghNr, M-21 -> tRgLb0L5rgUEVRSxwghNr, M-22 -> tRgLb0L5rgUEVRSxwghNr
- `redeem-queue-ordering`: M-9, M-36 -> mzmK3kHt4VHqATCWaq0vz
- `settled-yield-pre-injection-sniping`: M-31 -> -
- `virtual-share-dust-first-stake`: M-7 -> 562sw6kPJi1oG_WnvmLn0, M-8 -> 562sw6kPJi1oG_WnvmLn0, M-11 -> 562sw6kPJi1oG_WnvmLn0, M-12 -> 562sw6kPJi1oG_WnvmLn0, H-13 -> -, M-18 -> 562sw6kPJi1oG_WnvmLn0, M-25 -> 562sw6kPJi1oG_WnvmLn0, M-26 -> 562sw6kPJi1oG_WnvmLn0
- `virtual-share-dust-mint-stranding`: M-20 -> -

## Submission Candidates

### M-2 / `tRgLb0L5rgUEVRSxwghNr`
- Finding Title: Non-atomic hedge batches can leave MonetrixVault with one-sided spot or perp exposure
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `hypercore-async-hedge-postcondition`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The hedge flow submits two independent HyperCore orders and emits success without proving that the spot and perp legs filled in matched size. This is not merely operator compromise or arbitrary bad params: even expected operator workflows can leave a live one-sided exposure when an external order rests, partially fills, or is dropped, and the whitelist only binds asset pairing. The repair function is a later manual remediation path, not an execution-time postcondition, so the issue is a realistic Medium accounting/market-exposure bug.
- Code Evidence: `src/core/MonetrixVault.sol::executeHedge` validates the pair, calls `ActionEncoder.sendBuySpot` and `sendShortPerp`, then emits `HedgeExecuted` at lines 256-275 with no readback. `closeHedge` has the same pattern at lines 278-284, and `src/core/ActionEncoder.sol::_sendLimitOrder` only dispatches raw limit-order actions at lines 96-110.

### H-4 / `i9H5Zq0-WT7HizyybWMol`
- Finding Title: Live supply APR cap lets late stakers front-run settlement and capture yield accrued before they joined
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `live-supply-yield-cap`
- Checklist Gates Passed: `Scope, V12 root-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The settlement APR cap is computed from the live USDM supply at settlement time, while sUSDM distribution uses the live holder set at injection time. A late depositor can increase `USDM.totalSupply()` without increasing old surplus, expanding the cap for previously accrued yield, then stake before distribution and receive a pro-rata share of that old yield. This is a stronger path than simple injection sniping because the attacker can also influence how much accrued surplus becomes distributable; when applied at protocol scale it can materially redirect accrued yield, supporting High severity.
- Code Evidence: `src/core/MonetrixVault.sol::deposit` mints USDM 1:1 and increases live supply at lines 168-180. `src/core/MonetrixAccountant.sol::settleDailyPnL` computes the annualized cap from `usdm.totalSupply()` at lines 217-221, and `src/core/MonetrixVault.sol::distributeYield` injects yield into the live sUSDM supply at lines 377-399.

### M-5 / `vzb5HZneiQ6_eF3s_2gUG`
- Finding Title: Multisig keeperBridge records principal that Vault cannot bridge back for redemptions
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `multisig-principal-returnability`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: `keeperBridge` supports a configured `Multisig` target but records the bridged amount in a single Vault principal counter. The normal bridge-back path later attempts to source USDC from the Vault contract's own L1 account, not from `multisigVault`, so principal sent to the multisig can be counted as outstanding returnable principal while the Vault cannot bridge it back for redemptions. This is a configured protocol path rather than arbitrary operator theft, and it can materially delay redemption funding until out-of-band recovery, so Medium is justified.
- Code Evidence: `src/core/MonetrixVault.sol::keeperBridge` chooses `multisigVault` as recipient when enabled and increments `outstandingL1Principal` at lines 223-233. `bridgePrincipalFromL1` decrements the same counter and calls `_sendL1Bridge` at lines 237-244, while `_sendL1Bridge` checks `PrecompileReader.spotBalance(address(this), usdcToken)` at lines 530-540 and never reads the multisig account.

### M-9 / `mzmK3kHt4VHqATCWaq0vz`
- Finding Title: Redemption claims are first-come-first-served, letting later requests drain scarce escrow liquidity before earlier redeemers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `redemption-fifo-missing`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: Redeem requests are assigned monotonically increasing IDs and stored as a queue-like structure, but `claimRedeem` permits any matured owner to claim if the shared escrow currently has enough balance for that single request. Under the benchmark's stated concern for bridge/redemption behavior during bank runs, partial funding turns redemption access into a gas/ordering race and can delay earlier redeemers despite mature claims. The obligation is not erased, so this is not direct theft, but it is a material liveness/fairness failure of a core workflow and fits Medium.
- Code Evidence: `src/core/MonetrixVault.sol::requestRedeem` increments `nextRedeemId` and records requests at lines 183-195. `claimRedeem` checks only ownership and cooldown before calling `RedeemEscrow.payOut` at lines 198-212, while `src/core/RedeemEscrow.sol::payOut` only checks current balance for the individual amount at lines 47-52.

### M-15 / `mXm88s3ZwJpaIYOi_uq0x`
- Finding Title: Supplying to BLP registers accountant slots even when the HyperCore supply action is silently dropped
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `blp-supply-registration-postcondition`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: `supplyToBlp` registers the supplied slot with the Accountant immediately after dispatching the HyperCore supply action, without verifying that the BLP supply actually activated. The Accountant intentionally performs strict supplied-balance reads for registered slots, so a stale or unactivated registration can break backing/surplus views and settlement until cleanup. `removeSuppliedEntry` is a post-incident repair mechanism, not a safeguard that prevents the bad registration; the impact is a material accounting/liveness failure, so Medium is appropriate.
- Code Evidence: `src/core/MonetrixVault.sol::supplyToBlp` calls `ActionEncoder.sendSupply` and then `MonetrixAccountant.notifyVaultSupply` at lines 332-345. `src/core/MonetrixAccountant.sol::totalBackingSigned` reads registered supplied slots in `_readL1Backing` at lines 117-157, and `src/core/ActionEncoder.sol::sendSupply` only sends the raw action at lines 138-147.

### M-19 / `wEkO1pypNFhqLehk7ie0D`
- Finding Title: L1 bridge-back accounting can be reduced using held or supplied balances that cannot satisfy SEND_ASSET
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-back-free-balance-mismatch`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: Bridge-back accounting is reduced before the protocol can verify that the L1 `SEND_ASSET` action actually delivered EVM-side USDC. The local availability check is incomplete because it uses spot `total` without excluding `hold`, and when PM is enabled it also counts supplied 0x811 balances that are not free spot USDC for `SEND_ASSET`. A silent venue-side drop can therefore understate outstanding principal and leave redemptions unfunded, which is a material Medium accounting/liveness issue.
- Code Evidence: `src/core/MonetrixVault.sol::bridgePrincipalFromL1` decrements `outstandingL1Principal` before calling `_sendL1Bridge` at lines 237-244. `_sendL1Bridge` checks `PrecompileReader.spotBalance(address(this), usdcToken).total` and optionally `suppliedBalance` at lines 530-540, while `src/core/PrecompileReader.sol::spotBalance` exposes both `total` and `hold` at lines 45-52.

### M-23 / `ixjYBbG7rWa4eVh74Lapg`
- Finding Title: PM supplied USDC is treated as bridgeable spot balance, allowing bridge-back accounting to decrease without funds arriving
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-back-supplied-balance-mismatch`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: When PM is enabled, `_sendL1Bridge` counts supplied USDC as if it were immediately spendable spot USDC for an `ACTION_SEND_ASSET` bridge. The code does not withdraw that supplied balance first and reduces `outstandingL1Principal` before any finality/readback. If the bridge action is silently dropped because the free spot balance is insufficient, principal accounting is corrupted and redemptions can remain unfunded, making this a Medium severity issue.
- Code Evidence: `src/core/MonetrixVault.sol::_sendL1Bridge` adds `PrecompileReader.suppliedBalance(address(this), usdcToken)` to spot `total` when `pmEnabled` at lines 530-540. `bridgePrincipalFromL1` decrements the principal counter before `_sendL1Bridge` at lines 237-244, and `src/core/ActionEncoder.sol::sendBridgeToL1` sends `ACTION_SEND_ASSET` at lines 177-191.

### M-24 / `4YjJddXWcRv9EM5yYCoju`
- Finding Title: Held HyperCore USDC can falsely reduce outstanding principal in MonetrixVault.bridgePrincipalFromL1
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-back-held-balance-mismatch`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge-back precheck relies on spot `total`, but HyperCore spot balances also include a `hold` field for funds locked by orders. Held funds cannot necessarily satisfy `SEND_ASSET`, so the local require can pass while the L1 action does not deliver USDC. Because the principal counter is decremented first and there is no reconciliation, the bug can materially understate recoverable principal and delay redemptions; Medium severity is appropriate.
- Code Evidence: `src/core/PrecompileReader.sol::spotBalance` decodes `total` and `hold` at lines 45-52. `src/core/MonetrixVault.sol::_sendL1Bridge` uses only `spotBalance(...).total` at lines 530-540, and `bridgePrincipalFromL1` reduces `outstandingL1Principal` before dispatching the bridge action at lines 237-244.

### H-30 / `zoFAMm4hl3fNnvAJpmz6w`
- Finding Title: Dust sUSDM stake bypasses empty-vault reroute and captures pending user yield in MonetrixVault.distributeYield
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `empty-supply-reroute-bypass`
- Checklist Gates Passed: `Scope, V12 impact-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: `distributeYield` attempts to prevent empty-vault yield capture by rerouting user yield only when sUSDM supply is exactly zero. A permissionless small stake makes supply nonzero before distribution, so the user share is minted and injected into sUSDM instead of routed to foundation; depending on stake size and virtual-share dilution, the attacker can capture or strand a material part of the pending user yield. This differs from pure virtual-share dust griefing because it bypasses the explicit empty-supply safeguard and redirects a live distribution, supporting High severity.
- Code Evidence: `src/core/MonetrixVault.sol::distributeYield` zeros `userShare` only for `susdm.totalSupply() == 0` at lines 388-390 and then injects any positive user share at lines 395-399. `src/tokens/sUSDM.sol::injectYield` accepts the transfer for any positive supply at lines 234-240, with the virtual-share mismatch at lines 106-112.
