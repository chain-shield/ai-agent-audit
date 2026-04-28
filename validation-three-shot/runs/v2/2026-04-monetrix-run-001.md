# 2026-04-monetrix Three-Shot Validation run-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2026-04-monetrix/report/audit-report-openai.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/2026-04-monetrix`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`
Stage 1 scope screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/scope-screens/v2/2026-04-monetrix-run-001.md`
Stage 2 token screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/token-screens/v2/2026-04-monetrix-run-001.md`
Stage 3 final validation run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/stage3-runs/v2/2026-04-monetrix-run-001.md`

## Assembly Summary

- Excluded at stage 1 (scope / known issue): `1`
- Excluded at stage 2 (unsupported token): `0`
- Fully validated at stage 3: `35`

## Per-Finding Validation

### M-1 / `MQPNhFyzi1LA8-UlwLy-c`
- Finding Title: Observable yield injection can be sandwiched by late sUSDM depositors to capture existing stakers' yield
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `yield-checkpoint-free-rider`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding describes a live, permissionless late-joiner path: sUSDM shares are minted at the pre-injection exchange rate, while yield is credited to all holders at `injectYield` execution time with no stake-age check, snapshot, reward debt, or pending-yield inclusion in deposit pricing. It overlaps the V12 late-staker item, but the submitted severity is Medium rather than V12 High, so it is not excluded by the provided duplicate rule. The impact is material yield dilution/extraction from existing stakers, but this specific report proves loss of yield rather than principal-scale insolvency, so Medium is the best final severity.
- Code Evidence: `src/tokens/sUSDM.sol::deposit` simply delegates to ERC4626 at lines 122-124, while `injectYield` only checks amount, per-injection cap, and nonzero total supply before transferring USDM into `totalAssets` at lines 234-245. `src/core/MonetrixVault.sol::distributeYield` mints and injects the live user share at lines 377-399 without an eligibility snapshot.

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

### M-3 / `zhpa5luow_LNZA3sBdQxN`
- Finding Title: Unsolicited USDC can become distributable yield through YieldEscrow live-balance accounting
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `live-yield-escrow-balance`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: Yield distribution uses the escrow's raw USDC balance as the source of truth, so unsolicited or stale USDC in `YieldEscrow` is indistinguishable from Accountant-approved settled yield. `onlyVault` protects who can pull funds but not what amount is recognized as yield, and ordinary ERC20 transfers into the escrow cannot be rejected. This is related to the V12 live-balance item, but the submitted severity is Medium rather than High; the demonstrated impact is corrupted/unapproved yield distribution rather than direct theft from the donor, so Medium is appropriate.
- Code Evidence: `src/core/MonetrixVault.sol::distributeYield` reads `IYieldEscrow(yieldEscrow).balance()` and pulls exactly that `totalYield` at lines 377-383. `src/core/YieldEscrow.sol::balance` returns `usdc.balanceOf(address(this))` at lines 44-45, and `pullForDistribution` only checks live balance before transferring at lines 37-41.

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

### M-6 / `0whGLNpUqbf2aQHrfzF4U`
- Finding Title: Multisig bridge target inflates Vault principal that the Vault cannot bridge back
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `multisig-principal-returnability`
- Checklist Gates Passed: `Scope, V12 root-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The report's path exists: bridging to the configured multisig target still increases the Vault's global `outstandingL1Principal`, but bridge-back operations are executed from the Vault's L1 identity. The issue is not the V12 arbitrary-multisig-backing item, because the impact here is returnability of principal for redemption funding rather than false backing selection. The consequence is a material liveness/accounting failure under expected multisig-bridge operation, which supports Medium severity.
- Code Evidence: `src/core/MonetrixVault.sol::keeperBridge` allows `BridgeTarget.Multisig` and records the amount globally at lines 223-233. `bridgePrincipalFromL1` and `emergencyBridgePrincipalFromL1` both reduce `outstandingL1Principal` before using `_sendL1Bridge`, and `_sendL1Bridge` checks only the Vault account's L1 USDC availability at lines 237-244 and 530-540.

### M-7 / `p_4jfP5ct6TDbca9VeaQR`
- Finding Title: Virtual share scaling in sUSDM._decimalsOffset can trap material injected yield after dust initialization
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: sUSDM reports 6 decimals and also returns a 6-decimal ERC4626 offset, creating 1e6 virtual shares while real supply can be dust-sized. A dust initializer can make `totalSupply() > 0`, allowing a later material `injectYield`; when all real shares cool down, ERC4626 conversion leaves the virtual-share portion unclaimed in sUSDM. The V12 item with the same general root was Low, but this report demonstrates material yield capture/stranding up to the injection cap, so the final severity is High.
- Code Evidence: `src/tokens/sUSDM.sol` sets `totalAssets` to the live USDM balance at lines 102-104, `_decimalsOffset()` to 6 at lines 106-108, and `decimals()` to 6 at lines 110-112. `injectYield` only requires nonzero total supply at lines 234-240, while `cooldownShares` computes `assets = convertToAssets(shares)` before burning at lines 160-172.

### M-8 / `cFiYburwlcqePVE8VHj0P`
- Finding Title: ERC4626 virtual shares can permanently strand injected yield when sUSDM supply is dust-sized
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The dust-supply state is reachable because sUSDM has no minimum deposit or minimum real-supply threshold, and `injectYield` treats any positive supply as eligible. With the 1e6 virtual-share denominator, a dust real supply can capture only part of the injected assets and leave the rest permanently in the sUSDM contract after all real shares are burned. Since the amount can be non-dust and bounded only by configured injection limits, this is a material permanent asset-loss path and should be High.
- Code Evidence: `src/tokens/sUSDM.sol::deposit` and `mint` are open ERC4626 entry points at lines 122-127, `_decimalsOffset` and `decimals` both return 6 at lines 106-112, `injectYield` checks only `totalSupply() > 0` at lines 234-240, and `cooldownShares` escrows only `convertToAssets(shares)` at lines 160-172.

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

### M-10 / `-vYBRbQ82_TvmIC7yT7Lt`
- Finding Title: Dust sUSDM supply can route almost all future user yield into ERC4626 virtual shares
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The zero-supply reroute in `distributeYield` only handles exactly zero supply; a dust sUSDM position bypasses it. Because the ERC4626 virtual-share offset is large relative to the dust real supply, much of a subsequent user-share injection is economically assigned to virtual shares and can remain stuck after real shares are burned. The path is permissionless, live under low-supply/launch conditions, and can strand material yield, so High severity is warranted.
- Code Evidence: `src/core/MonetrixVault.sol::distributeYield` reroutes only when `susdm.totalSupply() == 0` at lines 388-390 and otherwise mints/injects `userShare` at lines 395-399. `src/tokens/sUSDM.sol` combines 6 share decimals with `_decimalsOffset() == 6` at lines 106-112 and accepts yield for any positive supply at lines 234-240.

### M-11 / `-jKZwssyjJFqqwQ_Bu9Ih`
- Finding Title: Misconfigured ERC4626 decimals offset can strand injected yield in sUSDM virtual shares
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The root cause is present in the current code: sUSDM forces a 6-decimal share token while retaining a 6-decimal ERC4626 virtual-share offset. This permits very small real supply to satisfy the nonzero-supply yield condition, after which cooldown conversion can burn all real shares while leaving virtual-share-attributed assets trapped. Existing pause, reentrancy, and max-injection checks do not address the accounting mismatch, and material injected yield can be permanently stranded, making the final severity High.
- Code Evidence: `src/tokens/sUSDM.sol::totalAssets`, `_decimalsOffset`, and `decimals` are at lines 102-112. `injectYield` relies on `totalSupply() > 0` at lines 234-240, and `cooldownShares` calculates assets with `convertToAssets` before burning and escrowing at lines 160-172.

### M-12 / `2YnBC8DBhUCnBF2rx0PZk`
- Finding Title: Dust first stake lets virtual shares capture and strand injected sUSDM yield
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: A 1-base-unit first stake can mint real shares comparable to OZ's virtual-share constant because sUSDM's share decimals and offset are both 6. Later, `injectYield` accepts large yield solely because supply is nonzero, and cooling down all real shares transfers only the real-share fraction to escrow. The remaining USDM has no standard withdrawal path once total supply is zero, so this is a material permanent-loss path rather than cosmetic rounding.
- Code Evidence: `src/tokens/sUSDM.sol` exposes unrestricted ERC4626 deposit/mint at lines 122-127, sets `_decimalsOffset()` and `decimals()` at lines 106-112, and allows `injectYield` on any positive supply at lines 234-240. `cooldownShares` burns after computing a virtual-share-influenced asset amount at lines 160-172.

### H-13 / `562sw6kPJi1oG_WnvmLn0`
- Finding Title: Dust first stake captures half of first sUSDM yield injection through virtual-share mis-scaling
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: This finding gives the clearest exploit instance of the virtual-share bug: a dust initial stake creates real shares equal to the virtual-share constant, so the first material injection is split between the attacker's real shares and unowned virtual shares. The attacker can cooldown and claim a large fraction of the injected user yield while the rest remains stranded. Because this can redirect and permanently lock non-dust protocol yield during an expected first-distribution scenario, High severity is appropriate.
- Code Evidence: `src/tokens/sUSDM.sol::_decimalsOffset` and `decimals` both return 6 at lines 106-112, `injectYield` only gates on positive `totalSupply` at lines 234-240, and `cooldownShares` uses `convertToAssets` then burns real shares and escrows only the computed assets at lines 160-172. `src/core/MonetrixVault.sol::distributeYield` bypasses the empty-supply reroute once supply is nonzero at lines 388-399.

### M-14 / `VGXewSiUj8BHLOJxaVuhf`
- Finding Title: Hedge execution emits success without verifying both spot and perp legs filled
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `hypercore-async-hedge-postcondition`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The reported issue exists independently of duplicate handling: hedge open and close paths emit success after dispatching raw HyperCore actions, but do not verify matched fills or record a pending/failed batch. Asset-pair whitelist checks do not bind execution success. A one-sided spot or perp fill can create directional exposure until manual repair, causing material backing risk but generally requiring venue/order conditions, so Medium is the right severity.
- Code Evidence: `src/core/MonetrixVault.sol::executeHedge` sends the spot and perp orders separately and emits `HedgeExecuted` at lines 256-275. `closeHedge` similarly sends sell-spot and close-perp actions and emits at lines 278-284, with raw action encoding in `src/core/ActionEncoder.sol::_sendLimitOrder` at lines 96-110.

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

### M-16 / `6XZ6R35K-AS-mqKcn6SrS`
- Finding Title: Late stakers can frontrun sUSDM.injectYield to capture yield earned before they joined
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `yield-checkpoint-free-rider`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The code has no mechanism to distinguish shares held during yield accrual from shares minted immediately before `injectYield`. A mempool observer can deposit at the stale pre-injection exchange rate, receive live shares, and then cooldown after the injection to lock in part of the transferred yield. The harm is material dilution of existing staker yield and is in scope despite the V12 severity mismatch, but this report proves yield theft rather than principal loss, so Medium is the final severity.
- Code Evidence: `src/tokens/sUSDM.sol::deposit` delegates directly to ERC4626 at lines 122-124, `injectYield` transfers USDM into `totalAssets` for all current shares at lines 234-245, and `cooldownShares` lets the late holder convert shares to an increased USDM claim at lines 160-172.

### M-17 / `-bu-ZQL12LJrtjVvaoqvr`
- Finding Title: Late sUSDM deposits can free-ride on already accrued yield injected through sUSDM.injectYield
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `yield-checkpoint-free-rider`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: This finding is the same live checkpointing flaw expressed through `deposit` and `injectYield`: pending/accrued yield is not priced into new shares before injection, so a late depositor participates pro rata in yield accrued before their deposit. No existing stake-age, snapshot, or reward-index safeguard binds the path. The exploit is realistic with observable distribution transactions and sufficient capital, and the impact is Medium yield extraction from long-term stakers.
- Code Evidence: `src/tokens/sUSDM.sol::deposit` has no pending-yield adjustment at lines 122-124, `injectYield` adds USDM to total assets and records the post-injection cumulative rate at lines 234-245, and `claimUnstake` later releases the increased claim from escrow after cooldown at lines 219-229.

### M-18 / `nJ-m0Tzzb9sfJAB2xVy3J`
- Finding Title: Low initial sUSDM supply lets injected yield be trapped in ERC4626 virtual shares
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: Low initial supply is a realistic launch/low-liquidity state, and the code treats any positive supply as sufficient to receive a material yield injection. Because real supply can be similar to or much smaller than the virtual-share constant, cooldown conversion after injection can leave a large virtual-share portion inside sUSDM with no owner. The value at risk is bounded by per-injection configuration but can be material, making this a High severity permanent-loss/capture path.
- Code Evidence: `src/tokens/sUSDM.sol::_decimalsOffset` and `decimals` create the mismatch at lines 106-112. `injectYield` checks only the configured maximum and `totalSupply() > 0` at lines 234-240, while `cooldownShares` transfers only `convertToAssets(shares)` into escrow before burning real supply at lines 160-172.

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

### M-20 / `auD8e5eF4-wAW6yYUUeGZ`
- Finding Title: Dust mint can lock nearly all injected yield in sUSDM virtual shares
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The mint variant is reachable because `mint` is open and uncapped except by pause, and a tiny real share supply can satisfy the nonzero-supply guard. With virtual shares dominating real supply, a subsequent material injection can become almost entirely unclaimable by real holders once the dust shares are cooled down and burned. The attacker may mainly grief/strand rather than capture all value, but permanent locking of material protocol yield is enough for High severity.
- Code Evidence: `src/tokens/sUSDM.sol::mint` delegates to ERC4626 at lines 126-127, `_decimalsOffset` creates 1e6 virtual shares at lines 106-108, `injectYield` accepts any positive supply at lines 234-240, and `cooldownShares` burns real shares after escrowing only the virtual-share-adjusted asset amount at lines 160-172.

### M-21 / `9BEHf5Q0oJXlBT_zGCxNt`
- Finding Title: Hedge execution emits success without proving both spot and perp legs filled
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `hypercore-async-hedge-postcondition`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The contract does not prove that both legs of a hedge have been accepted and filled after dispatching the orders. The success event is therefore not tied to the actual L1 position state, and a dropped/resting/partial leg can leave the protocol with naked exposure until off-chain detection and repair. This is a live runtime postcondition failure in a core hedge path and merits Medium severity.
- Code Evidence: `src/core/MonetrixVault.sol::executeHedge` calls the two leg-sending helpers and immediately emits at lines 256-275. `closeHedge` repeats the same no-readback pattern at lines 278-284, and `src/core/ActionEncoder.sol::_sendLimitOrder` only constructs and sends a raw action at lines 96-110.

### M-22 / `n-cuygU2v7GVv19-anjrm`
- Finding Title: Independent hedge legs can leave MonetrixVault unhedged when one HyperCore order is dropped or partially filled
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `hypercore-async-hedge-postcondition`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: Open and close hedges are not atomic at the contract level: each leg is a separate HyperCore action and the Vault stores no pending, failed, or fill-size state. Since the protocol's solvency model depends on matched spot/perp exposure, a one-leg fill under normal orderbook conditions can create material market exposure before manual repair. The finding is therefore valid Medium.
- Code Evidence: `src/core/MonetrixVault.sol::executeHedge` sends `sendBuySpot` then `sendShortPerp` at lines 264-265 and emits success at line 275. `closeHedge` sends `sendSellSpot` then `sendClosePerp` at lines 281-284; neither path reads `PrecompileReader` balances or positions afterward.

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

### M-25 / `1V-Z9xUIIGsxmEi58p3vR`
- Finding Title: Dust first stake strands injected yield in sUSDM virtual shares during cooldown
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The cooldown path is exactly where the virtual-share loss becomes permanent: after a dust first stake and material yield injection, `convertToAssets(totalSupply())` returns only the real-share portion, then all real shares are burned. Any remaining USDM in sUSDM is not represented by real supply or pending claims. Since this can affect non-dust injected yield under live low-supply conditions, it is a valid High.
- Code Evidence: `src/tokens/sUSDM.sol::cooldownShares` calculates `assets = convertToAssets(shares)` at line 167, burns at line 170, and escrows only that amount at lines 171-172. The underlying virtual-share mismatch is set at lines 106-112, and `injectYield` accepts any positive total supply at lines 234-240.

### M-26 / `UVYDK0igzip7Vve9a3yJa`
- Finding Title: Low-supply sUSDM deposits let virtual shares capture and strand injected yield
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `erc4626-virtual-share-misscaling`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The reported low-supply exploit follows directly from the ERC4626 configuration: virtual shares can be comparable to or dominate real shares, yet the Vault treats positive real supply as enough to distribute user yield. After cooldown, only the real-share portion becomes a claim and the rest can be permanently stranded. This is a live permissionless path with material value at risk, so High is the final severity.
- Code Evidence: `src/tokens/sUSDM.sol` defines the 6-decimal offset/decimals mismatch at lines 106-112, accepts yield with only a `totalSupply() > 0` check at lines 234-240, and escrows virtual-share-adjusted assets during `cooldownShares` at lines 160-172. `src/core/MonetrixVault.sol::distributeYield` only reroutes when supply is exactly zero at lines 388-390.

### M-27 / `MMwYCWdiyDhSE3uCKAhFF`
- Finding Title: Late deposits can frontrun sUSDM.injectYield and capture yield accrued before they joined
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `yield-checkpoint-free-rider`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: New sUSDM shares minted immediately before `injectYield` are treated the same as long-held shares because there is no snapshot or minimum stake age. A permissionless late depositor can therefore capture part of settled or in-motion yield that economically belongs to prior stakers. This is the V12 family with a different reported severity, and on its own proof it supports Medium yield-dilution severity.
- Code Evidence: `src/tokens/sUSDM.sol::deposit` lacks pending-yield accounting at lines 122-124, `injectYield` transfers USDM into the vault balance at lines 234-240, and `cooldownShares` allows the late holder to lock in the post-injection exchange rate at lines 160-172.

### M-28 / `lLLeOnUDm_UY6WAA2ne10`
- Finding Title: Supplied USDC is counted as bridgeable, allowing silent bridge drops to corrupt principal accounting
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-back-supplied-balance-mismatch`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge-back code conflates PM supplied USDC with free spot USDC available for `ACTION_SEND_ASSET`. Since `outstandingL1Principal` is decremented before confirmed bridge effect and there is no postcondition or pending bridge reconciliation, a local precheck can pass on non-spendable supplied balance while no EVM-side funds arrive. This is a material accounting/redemption liveness bug, not unsupported-token behavior, and should be Medium.
- Code Evidence: `src/core/MonetrixVault.sol::_sendL1Bridge` adds supplied balance to the bridge availability calculation at lines 530-540, and `bridgePrincipalFromL1` reduces principal before that send at lines 237-244. `src/core/ActionEncoder.sol::sendBridgeToL1` emits a `SEND_ASSET` action that spends spot USDC at lines 177-191.

### M-29 / `koX1oBSwdWmmvHkEW9rhM`
- Finding Title: L1 bridge-back counts held HyperCore USDC as spendable and can permanently understate outstanding principal
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-back-held-balance-mismatch`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: Held L1 spot USDC can be included in `SpotBalance.total` while unavailable for a send action. The Vault does not subtract `hold`, then reduces `outstandingL1Principal` before observing whether the bridge completed. That can strand principal behind an understated accounting cap and block normal redemption funding, which is a realistic Medium issue under expected hedge/order workflows.
- Code Evidence: `src/core/PrecompileReader.sol::spotBalance` exposes the separate `hold` field at lines 45-52. `src/core/MonetrixVault.sol::_sendL1Bridge` checks only `total` at lines 530-540, and `bridgePrincipalFromL1` consumes the principal counter first at lines 237-244.

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

### M-31 / `IjVPRntddtUJaBZS16kcA`
- Finding Title: Late sUSDM deposits can capture yield settled before the shares existed
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `settled-yield-checkpoint-free-rider`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: `settle` moves approved yield into `YieldEscrow`, but sUSDM `totalAssets` does not include that pending escrow balance and deposits remain open until `distributeYield`. A late depositor can buy shares after settlement but before injection and share in yield settled before the shares existed. This is the V12 pending-yield-sniping root with a different severity; the demonstrated impact is material yield dilution, so Medium is appropriate.
- Code Evidence: `src/core/MonetrixVault.sol::settle` calls the Accountant and transfers `proposedYield` to `yieldEscrow` at lines 364-374. `distributeYield` later pulls and injects the live balance at lines 377-399, while `src/tokens/sUSDM.sol::deposit` has no checkpoint against settled-but-not-injected yield at lines 122-124.

### M-32 / `CBjsOJXof775M4MhOn7_o`
- Finding Title: Donation over sUSDM injection cap can permanently DoS MonetrixVault.distributeYield
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `live-escrow-balance-distribution-dos`
- Checklist Gates Passed: `Scope, V12 severity-difference, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: Because `distributeYield` processes the full live escrow balance, a permissionless USDC donation can push the derived `userShare` above `sUSDM.config().maxYieldPerInjection()`. The subsequent `injectYield` revert rolls back the whole distribution, and there is no partial distribution, accounted-yield ledger, or excess sweep path in the hot path. The attack has a real cost and mainly DoSes yield distribution rather than stealing principal, so Medium is the final severity.
- Code Evidence: `src/core/MonetrixVault.sol::distributeYield` reads the full `YieldEscrow.balance()`, pulls that amount, computes `userShare`, and calls `susdm.injectYield(userShare)` at lines 377-399. `src/core/YieldEscrow.sol::balance` is just the raw USDC balance at lines 44-45, and `src/tokens/sUSDM.sol::injectYield` reverts when `usdmAmount > config.maxYieldPerInjection()` at lines 234-237.

### H-33 / `ZmoUk6IifETznZEslhFnX`
- Finding Title: Uninitialized USDM proxy can be taken over to bind attacker as vault and mint unlimited USDM
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `deployment-or-setup`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Requires uninitialized proxy deployment or initialization mistake excluded by prompt
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/monetrix-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/monetrix-scope.md`, `/Users/apmfree/Desktop/Audit/2026-04-monetrix/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`.

### M-34 / `jdQ8Il4Cv1_wuMYDXr8CX`
- Finding Title: HyperCore oracle prices are accepted without freshness or deviation checks when declaring distributable yield
- Decision: Invalid
- Confidence: Med
- Bug Exists: No
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-hypercore-oracle-assumption`
- Checklist Gates Passed: `Scope, Supported Behavior`
- Checklist Gates Failed: `Live Runtime Exposure, Trusted External Failure, Evidence Quality`
- Detailed Reason: The code does rely on HyperCore's canonical oracle price without an additional heartbeat/deviation layer, but the report's harmful path requires the trusted HyperCore oracle/precompile to return stale or inflated data. The benchmark docs frame Hyperliquid/HyperCore as a core dependency and the validation prompt excludes issues that depend on a trusted external component failing or becoming malicious unless the protocol itself violates a live runtime invariant. Without evidence of an available freshness signal or current code path that can be manipulated under normal operation, this is not a valid H/M protocol bug.
- Code Evidence: `src/core/PrecompileReader.sol::oraclePx` checks call success, response length, and nonzero price at lines 65-72. `src/core/MonetrixAccountant.sol::_readL1Backing` uses those reader paths to value spot/supplied assets at lines 137-171, but the report does not show a protocol-controlled stale-price input or bypass beyond trusting the HyperCore precompile.

### M-35 / `jO-u5ALuLN4PJgk_lcDxg`
- Finding Title: HyperCore oracle prices are accepted without freshness or deviation bounds, enabling false surplus settlement
- Decision: Invalid
- Confidence: Med
- Bug Exists: No
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-hypercore-oracle-assumption`
- Checklist Gates Passed: `Scope, Supported Behavior`
- Checklist Gates Failed: `Live Runtime Exposure, Trusted External Failure, Evidence Quality`
- Detailed Reason: This finding again depends on HyperCore's own oracle returning a stale or wrong value. The absence of a secondary freshness/deviation check is visible, but the report does not establish that such metadata exists or that an attacker can cause the canonical precompile to misprice assets under current expected operation. Since the exploit requires trusted venue/oracle failure rather than a protocol-side validation bypass, it should be rejected under the live-runtime and trusted-external-component gates.
- Code Evidence: `src/core/PrecompileReader.sol::oraclePx` only validates successful nonzero precompile output at lines 65-72, and `src/core/MonetrixAccountant.sol::surplus` and `settleDailyPnL` consume backing values for settlement at lines 180-221. The missing piece is a code-grounded path to stale or manipulated `oraclePx` output within Monetrix's control.

### M-36 / `4L3Ikdc-lTfCMKnCPhROR`
- Finding Title: First claimant can drain scarce RedeemEscrow liquidity and force later redemption claims to revert
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `redemption-pro-rata-missing`
- Checklist Gates Passed: `Scope, Supported Behavior, User Error, Live Runtime Exposure, Safeguards, Impact, Likelihood, Evidence`
- Checklist Gates Failed: `-`
- Detailed Reason: `RedeemEscrow.payOut` settles each claim at full face value if the live escrow balance covers that individual claim, with no FIFO cursor, batch accounting, or pro-rata handling during shortfall. The protocol documents redemption accounting and bank-run coverage as important invariants, and partial funding is an expected stress state; first-mover claiming can therefore materially deny other matured claimants available liquidity. The remaining obligation is still tracked, so this is Medium liveness/fairness harm rather than High theft.
- Code Evidence: `src/core/RedeemEscrow.sol::payOut` only checks the current balance against the single payout amount, then decrements `totalOwed` and transfers at lines 47-52. `src/core/MonetrixVault.sol::claimRedeem` performs ownership/cooldown checks and calls `payOut` without ordering checks at lines 198-212.
