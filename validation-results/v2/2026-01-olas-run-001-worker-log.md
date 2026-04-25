# 2026-01-olas Worker Log run-001

Prompt version: `v2`
Benchmark: `2026-01-olas`
Run id: `run-001`
Raw run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-runs/v2/2026-01-olas-run-001.md`
Scored result: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-results/v2/2026-01-olas-run-001.md`

This file records orchestrator-visible worker lifecycle events and the exact raw finding block appended by each completed raw worker.

## 2026-04-24T19:43:00Z `completed` `raw-finding` `H-1`
- Report ID: `jWuI7tcEvGKNrRHhPxhvQ`
- Title: Permanent DoS of Staking via Malicious Service Multisig Upgrade (Gas Griefing)
- Summary: Marked invalid as publicly known test-folder gas-griefing scenario with no cleanly demonstrated in-scope upgrade path beyond vm.store simulation.

## 2026-04-24T19:45:45Z `completed` `raw-finding` `M-2`
- Report ID: `6VIQqFECY8GRDFYyOO5Mb`
- Title: DoS with USDT and other non-compliant ERC20s due to incompatible IToken interface
- Summary: Marked valid Low/QA: strict approve semantics create a real USDT-style compatibility issue, but the benchmarked path is owner-only and not shown to require USDT support.

## 2026-04-24T19:46:44Z `completed` `raw-finding` `M-3`
- Report ID: `cHTZfN14l5r_MIayF8Ra2`
- Title: Permanent DoS / Fund Lock due to immutable bridge address and lack of rescue mechanism
- Summary: Marked invalid as an immutable external dependency / operational bridge-outage scenario rather than a concrete attacker-driven protocol bug.

## 2026-04-24T19:50:07Z `completed` `raw-finding` `H-4`
- Report ID: `DL1JhZIFCapJ9HHQq5mgo`
- Title: Token-Secured Services bypass Slashing mechanism due to decoupled bond accounting
- Summary: Marked invalid: token-bond slashing exists in ServiceRegistryTokenUtility, and the reported bypass is a publicly known test-folder scenario using the wrong slashing path.

## 2026-04-24T19:51:19Z `completed` `raw-finding` `H-5`
- Report ID: `d_xbCK4coBPU3zLTfnKAV`
- Title: Drained service slashed funds are permanently locked in Treasury due to missing accounting update
- Summary: Marked valid Medium: Treasury drains real ETH from ServiceRegistry but never updates ETHOwned or ETHFromServices, so withdrawn slashed funds become untracked and stuck.

## 2026-04-24T19:52:02Z `completed` `raw-finding` `M-6`
- Report ID: `I0qedlcD_ewvQf5AkEez-`
- Title: Missing Deadline in Operator Signatures
- Summary: Marked invalid: signatures are intentionally service-owner-bound and nonce-scoped, so missing deadline is a stale-authorization UX tradeoff rather than a concrete replay bug.

## 2026-04-24T19:54:24Z `completed` `raw-finding` `H-7`
- Report ID: `f1WVAhp-n1jMWcGo_hpiA`
- Title: UniswapPriceOracle.getPrice returns inverted price (Quote/Base vs Base/Quote)
- Summary: Marked invalid: the oracle orientation is consistent with the Balancer oracle and downstream use is relative slippage checking, not absolute valuation.

## 2026-04-24T19:55:42Z `completed` `raw-finding` `H-8`
- Report ID: `n1A5RFZkv7pfuahxMDoXy`
- Title: Contract non-functional due to missing setters for allowlist configuration
- Summary: Marked invalid: bridge verifiers are designed to run via GuardCM delegatecall and read GuardCM’s configured allowlist storage.

## 2026-04-24T19:59:12Z `completed` `raw-finding` `H-9`
- Report ID: `xiIyanPs6cnpJX0o9weoy`
- Title: DoS on USDT interactions due to incompatible ERC20 interface in approve calls
- Summary: Marked valid Low/QA: strict approve semantics create a real USDT-style compatibility issue, but the scoped LiquidityManager deployment is owner-only and benchmark evidence only supports OLAS/WETH usage, not an H/M USDT DoS.

## 2026-04-24T20:00:30Z `completed` `raw-finding` `M-10`
- Report ID: `NPHnqoRRjrDXL__imdksF`
- Title: DoS with USDT due to incorrect approve interface
- Summary: Marked valid Low/QA: strict approve semantics create a real USDT-style compatibility issue in liquidity and buyback paths, but the scoped setup only demonstrates owner-configured OLAS/WETH usage rather than an H/M USDT production DoS.

## 2026-04-24T20:01:32Z `completed` `raw-finding` `M-11`
- Report ID: `WFQStm95ZAXZro6Pm_23W`
- Title: Infinite signature validity and missing revocation in ServiceManager
- Summary: Marked invalid: signatures are service-owner-bound and nonce-scoped, stale token-secured registrations can still be blocked by revoking operator allowance, and stale unbond execution returns funds to the operator.

## 2026-04-24T20:04:02Z `completed` `raw-finding` `H-12`
- Report ID: `fmYSYKHqWM3K2OSAbkZwq`
- Title: DoS in ProcessBridgedDataGnosis due to unsafe assembly casting in VerifyBridgedData
- Summary: Marked invalid: the claimed dirty-bits DoS from uint32 payloadLength did not reproduce, and the Gnosis wrapper only forwards into the shared verifier logic.

## 2026-04-24T20:04:51Z `completed` `raw-finding` `M-13`
- Report ID: `gxRokS-Fs7ExSdykZqntv`
- Title: Incompatible ERC20 interface causes DoS and stuck funds for USDT on Mainnet
- Summary: Marked valid Low/QA: strict ERC20 approve/transfer semantics create a real USDT-style compatibility issue in the Balancer buyback burner, but the scoped setup only demonstrates OLAS/WETH rather than an H/M USDT production path.

## 2026-04-24T20:07:49Z `completed` `raw-finding` `M-14`
- Report ID: `ieN07-aeT52ezbToLzBpo`
- Title: changeRanges causes persistent DoS and misroutes funds for single-sided positions
- Summary: Marked valid Medium: changeRanges can exit into a single-sided state, sweep funds to treasury, and leave a stale pool->position mapping that makes future changeRanges/decreaseLiquidity revert until manual recovery.

## 2026-04-24T20:08:56Z `completed` `raw-finding` `H-15`
- Report ID: `YR9XLGUR-OjmKgTXmj0Zd`
- Title: Critical DoS in BridgeMessenger due to unsafe assembly bitmasking in _processData
- Summary: Marked invalid: the claimed dirty-bits DoS in BridgeMessenger did not reproduce, and the narrow uint96/uint32 assignments decode cleanly in the shared packed-message parser.

## 2026-04-24T20:11:25Z `completed` `raw-finding` `M-16`
- Report ID: `qXm3skKOM1buPaPPeXVrA`
- Title: DoS of Service Redeployment via malicious Operator in unbond
- Summary: Marked invalid: unbond refund reverts are real, but the scenario depends on a malicious or incompatible operator address in the user registration flow, which the benchmark excludes as registration misuse / operator misconfiguration.

## 2026-04-24T20:12:12Z `completed` `raw-finding` `M-17`
- Report ID: `i-gmsNtIwpOl7nzzrZ3QV`
- Title: Missing transaction expiration check in liquidity management functions
- Summary: Marked valid Low/QA: liquidity operations use execution-time block.timestamp as the deadline, so expiry protection is ineffective even though the flows remain owner-driven and slippage-bounded.

## 2026-04-24T20:13:12Z `completed` `raw-finding` `M-18`
- Report ID: `2keXmUkz03Q3OUWgVbuNJ`
- Title: NeighborhoodScanner chooses wrong optimization mode for unbalanced amounts causing capital inefficiency
- Summary: Marked invalid: the report disputes an optimization heuristic, but both scanner branches re-cap liquidity against token budgets and then maximize utilization locally, so no concrete security bug is established.

## 2026-04-24T20:14:01Z `completed` `raw-finding` `H-19`
- Report ID: `GHMSoOJ2HCch-nPtHNErS`
- Title: Missing configuration setters for mapAllowedTargetSelectorChainIds renders verification unusable
- Summary: Marked invalid: the allowlist is configured on GuardCM and read through delegatecall, so the verifier is not bricked by lacking a local setter.

## 2026-04-24T20:14:27Z `completed` `raw-finding` `M-20`
- Report ID: `jYEv6OQfADRnQlFLeHda6`
- Title: Incompatibility with USDT due to strict ERC20 interface return check causing DOS
- Summary: Marked valid Low/QA: the Balancer buyback burner has a real USDT-style ERC20 compatibility issue, but the scoped setup still only demonstrates OLAS/WETH rather than an H/M USDT production path.

## 2026-04-24T20:18:23Z `completed` `raw-finding`
- Summary: Marked invalid: missing rescue in Bridge2BurnerGnosis is an external bridge outage / trust-assumption issue, not a permissionless exploit in the scoped Olas code.

## 2026-04-24T20:20:51Z `completed` `raw-finding`
- Summary: Marked valid medium: GuardCM ignores scheduled ETH value, and the scoped Wormhole bridge path makes that omission a concrete expansion of CM authority over Timelock-held ETH.

## 2026-04-24T20:23:10Z `completed` `raw-finding`
- Summary: Marked valid medium: staking and activity checks trust only the Safe proxy shell hash, while the whitelisted same-address multisig path does not pin the delegated singleton implementation.

## 2026-04-24T20:25:43Z `completed` `raw-finding`
- Summary: Marked invalid: BridgeMessenger dirty-bits DoS claim does not reproduce; the typed uint96/uint32 assembly assignments decode cleanly on the live path.

## 2026-04-24T20:26:15Z `completed` `raw-finding`
- Summary: Marked invalid: duplicate BridgeMessenger dirty-bits DoS claim still does not reproduce; the live uint96/uint32 typed assignments decode cleanly.

## 2026-04-24T20:28:18Z `completed` `raw-finding`
- Summary: Marked invalid: another duplicate BridgeMessenger dirty-bits DoS claim does not reproduce; the typed decode path remains clean in local reproduction.

## 2026-04-24T20:29:47Z `completed` `raw-finding`
- Summary: Marked invalid: eviction does not auto-forfeit accrued rewards in code, but the canonical benchmark docs do not specify that forfeiture invariant and the repo exposes forcedUnstake as the explicit no-reward path.

## 2026-04-24T20:30:19Z `completed` `raw-finding`
- Summary: Marked invalid: ProcessBridgedDataWormhole reads GuardCM allowlist storage through delegatecall, so the missing local setter does not brick the verifier.

## 2026-04-24T20:32:16Z `completed` `raw-finding`
- Summary: Marked invalid: duplicate ProcessBridgedDataWormhole setter claim misunderstands the GuardCM delegatecall/shared-storage design.

## 2026-04-24T20:35:25Z `completed` `raw-finding`
- Summary: Marked invalid: redeem() can fail on a reverting target, but the benchmark docs and contract maintenance/migration flows make this a recoverable operational condition rather than permanent stuck funds.

## 2026-04-24T20:37:14Z `completed` `raw-finding`
- Summary: Marked invalid: Balancer validatePrice has no explicit max-age check, but the report does not prove stale unsafe acceptance because execution and validation both use the same live pool spot, and the PoC never changes that on-chain price source.

## 2026-04-24T20:37:51Z `completed` `raw-finding`
- Summary: Marked invalid: duplicate Balancer stale-oracle claim still does not prove unsafe acceptance, because validation and execution both depend on the same live pool spot and any harmful path would require a separate concrete manipulation attack.

## 2026-04-24T20:38:58Z `completed` `raw-finding`
- Summary: Marked invalid: BuyBackBurner.transfer is intentionally permissionless but only routes funds to protocol-controlled treasury or bridge2Burner, while buyBack itself is also permissionless, so this is not an attacker-controlled DoS path.

## 2026-04-24T20:42:32Z `completed` `raw-finding`
- Summary: Marked valid low/QA: BuyBackBurner token calls are genuinely incompatible with void-return ERC20s like USDT, but the current scoped buyback routes are standard WETH/OLAS, so this is a low-severity supported-asset compatibility issue rather than an H/M exploit.

## 2026-04-24T20:46:17Z `completed` `raw-finding`
- Summary: Marked valid low/QA: numNewOwners is genuinely sybil-sensitive and does feed next-epoch IDF/bond payouts, but the effect is explicitly capped by epsilonRate and the benchmark scope already acknowledges small-donation incentive gaming as a design tradeoff.

## 2026-04-24T20:51:34Z `completed` `raw-finding`
- Summary: Marked invalid: the gas-griefing checkpoint path depends on staking a deliberately malicious service multisig, and the benchmark's optimistic multisig design notes treat that malicious setup as outside protocol responsibility rather than an independent H/M staking flaw.

## 2026-04-24T20:53:43Z `completed` `raw-finding`
- Summary: Marked invalid: StakingBase only checks proxy codehash, but this report never proves a scoped path from the whitelisted service-deployment flow to an arbitrary fake master copy, and the benchmark's optimistic Safe-creation notes treat intentionally malicious multisig setup as outside protocol responsibility.

## 2026-04-24T20:54:44Z `completed` `raw-finding`
- Summary: Marked invalid: Burner.burn is intentionally permissionless and the first caller already achieves the desired burn, so a later zero-balance revert only wastes a redundant caller's gas rather than causing a real protocol DoS.

## 2026-04-24T20:55:16Z `completed` `raw-finding`
- Summary: Marked invalid: this is the same burner-front-run griefing overclaim as M-38, because the first permissionless caller already achieves the intended burn and the only remaining effect is wasted gas for a redundant follower.

## 2026-04-24T20:56:08Z `completed` `raw-finding`
- Summary: Marked valid low/QA: the liquidity-manager approve pattern is genuinely incompatible with USDT-style void-return or zero-first tokens, but the benchmarked routes are standard WETH/OLAS flows, so this is a compatibility issue rather than a practical H/M exploit.

## 2026-04-24T20:59:32Z `completed` `raw-finding`
- Summary: Marked valid low/QA: ServiceManager advertises contract-operator support, but OperatorSignedHashes only accepts a bespoke 65-byte v=4 format with the validator address encoded into r, so generic EIP-1271 wallets are not broadly interoperable.

## 2026-04-24T21:01:19Z `completed` `raw-finding`
- Summary: Marked invalid: VerifyBridgedData does load payloadLength in assembly, but later high-level uses of the uint32 are cleaned before allocation/use, so the claimed gigantic dirty-bits memory allocation DoS is not established.

## 2026-04-24T21:02:15Z `completed` `raw-finding`
- Summary: Marked valid low/QA: Treasury uses bool-return transfer/transferFrom interfaces and is genuinely incompatible with USDT-style void-return tokens, but the benchmarked treasury/depository flows use standard enabled assets, so this is a compatibility issue rather than a practical H/M exploit.

## 2026-04-24T21:07:02Z `completed` `raw-finding`
- Summary: M-44 invalid: sentinel registration is excluded misconfigured registration / misuse of registration logic

## 2026-04-24T21:10:04Z `completed` `raw-finding`
- Summary: H-45 valid medium: cross-service reentrancy can overwrite stale global balance in native staking path

## 2026-04-24T21:14:46Z `completed` `raw-finding`
- Summary: M-46 valid low: native-token push payments let malicious operator contract grief claim/unstake

## 2026-04-24T21:15:38Z `completed` `raw-finding`
- Summary: H-47 invalid: optimism path inherits same cleaned uint32 payloadLength semantics as shared verifier

## 2026-04-24T21:16:54Z `completed` `raw-finding`
- Summary: M-48 invalid: scanner branch selection matches documented token0-vs-token1-limited search design

## 2026-04-24T21:19:53Z `completed` `raw-finding`
- Summary: H-49 valid medium: permissionless collectFees can sweep prefunded balances via full-balance utility handling

## 2026-04-24T21:20:54Z `completed` `raw-finding`
- Summary: M-50 invalid: second collectFees revert lacks proven protocol harm beyond off-chain batch assumptions

## 2026-04-24T21:22:11Z `completed` `raw-finding`
- Summary: M-51 valid medium: slippage mins are spot-based and TWAP deviation check is neutralized by overwritten sqrt price

## 2026-04-24T21:26:49Z `completed` `raw-finding`
- Summary: M-52 invalid: early permissionless fee realization changes strategy timing but does not divert protocol-owned fees

## 2026-04-24T21:29:56Z `completed` `raw-finding`
- Summary: H-53 appended as Invalid (wrong-slashing-entrypoint); token services are slashable via ServiceRegistryTokenUtility and utility refunds honor reduced balances.

## 2026-04-24T21:31:16Z `completed` `raw-finding`
- Summary: M-54 appended as Valid Low/QA; real spot-price slippage issue, but not root-cause independent from the broader liquidity slippage flaw.

## 2026-04-24T21:32:03Z `completed` `raw-finding`
- Summary: H-55 appended as Invalid; ServiceRegistry.registerAgents is manager-gated, so operators cannot bypass ServiceManager and register with only the 1 wei wrapper.

## 2026-04-24T21:35:47Z `completed` `raw-finding`
- Summary: H-56 appended as Invalid; compiler cleanup of uint32/uint96 after assembly prevents the claimed dirty-bits DoS in _verifyBridgedData.

## 2026-04-24T21:36:59Z `completed` `raw-finding`
- Summary: M-57 appended as Valid Low/QA; deterministic Safe CREATE2 deployment can be frontrun for gas griefing, but the Safe remains configured for the same owners and the service owner can retry with a new nonce.

## 2026-04-24T21:37:49Z `completed` `raw-finding`
- Summary: H-58 appended as Invalid; token changes during update are blocked to PreRegistration, so the owner cannot swap bond tokens while operator bonds still exist.

## 2026-04-24T21:40:23Z `completed` `raw-finding`
- Summary: H-59 appended as Valid Medium; the V3 buyback path spends protocol inventory with amountOutMinimum=1 and no post-swap validation, allowing sandwich-driven value extraction.

## 2026-04-24T21:41:13Z `completed` `raw-finding`
- Summary: M-60 appended as Invalid; block.timestamp deadline and execution-time pricing describe stale-intent UX, not a distinct H/M exploit without adversarial manipulation.

## 2026-04-24T21:41:51Z `completed` `raw-finding`
- Summary: H-61 appended as Valid Low/QA; the Uniswap V3 buyback path repeats the same near-zero minOut sandwich issue already captured for the other V3 buyback implementation.

## 2026-04-24T21:44:06Z `completed` `raw-finding`
- Summary: M-62 appended as Invalid; buyback maxSlippage and the paired V2 oracles consistently use percent-style units, so the reported WAD-vs-percent underflow mismatch does not exist.

## 2026-04-24T21:48:02Z `completed` `raw-finding`
- Summary: M-63 invalid: no demonstrated non-canonical token-order DoS in benchmarked convertToV3 paths

## 2026-04-24T21:49:10Z `completed` `raw-finding`
- Summary: H-64 valid: UniswapPriceOracle TWAP check collapses to spot price and bypasses slippage validation

## 2026-04-24T21:49:47Z `completed` `raw-finding`
- Summary: M-65 valid low/QA: same-block pair updates can grief UniswapPriceOracle consumers for one block

## 2026-04-24T21:53:14Z `completed` `raw-finding`
- Summary: H-66 invalid: recovery module binds decoded serviceId to current agent set, and recoverable services have zero instances

## 2026-04-24T21:54:13Z `completed` `raw-finding`
- Summary: H-67 invalid: Solidity cleans uint32 payloadLength before allocation, so the bridge verifier OOG claim does not hold

## 2026-04-24T21:55:14Z `completed` `raw-finding`
- Summary: M-68 valid low/QA: IdentityRegistryBridger can retain stale multisig authorization until public sync updates the cache

## 2026-04-24T21:56:08Z `completed` `raw-finding`
- Summary: H-69 valid low/QA: duplicate V3 buyback min-out-disabled / sandwichable execution-price bug

## 2026-04-24T21:56:59Z `completed` `raw-finding`
- Summary: H-70 invalid: the repo intentionally uses the 7-field router interface against the configured 0x68b346 router variant

## 2026-04-24T21:59:46Z `completed` `raw-finding`
- Summary: M-71 valid low/QA: overbroad report, but its actionable substance is the same V3 buyback min-out-disabled bug family

## 2026-04-24T22:00:22Z `completed` `raw-finding`
- Summary: H-72 valid low/QA: duplicate V3 buyback slippage / missing min-out enforcement bug family

## 2026-04-24T22:01:12Z `completed` `raw-finding`
- Summary: H-73 invalid: oracle quote orientation may be counterintuitive, but downstream code only uses relative comparisons so no H/M impact is shown

## 2026-04-24T22:04:45Z `completed` `raw-finding`
- Summary: M-74 valid low/QA: BalancerPriceOracle lacks decimals normalization, but the benchmark only wires this oracle to WETH so the issue remains configuration-scoped

## 2026-04-24T22:05:32Z `completed` `raw-finding`
- Summary: H-75 valid low/QA: decimals bug is real, but the inversion claim is non-impactful and the benchmark only wires BalancerPriceOracle to WETH

## 2026-04-24T22:07:43Z `completed` `raw-finding`
- Summary: H-76 valid low/QA: stale or failed TWAP checks do fall back to spot, but this is a duplicate slice of the broader liquidity-manager slippage bug

## 2026-04-24T22:08:24Z `completed` `raw-finding`
- Summary: M-77 invalid: natural market movement before inclusion is stale-intent risk, not a standalone security exploit

## 2026-04-24T22:08:51Z `completed` `raw-finding`
- Summary: M-78 invalid: block.timestamp deadline is a stale-intent design choice, not a distinct security exploit

## 2026-04-24T22:12:12Z `completed` `raw-finding`
- Summary: H-79 invalid: BridgeMessenger parsing offsets are intentional and Solidity cleans uint96/uint32 before the claimed balance and allocation uses

## 2026-04-24T22:14:50Z `completed` `raw-finding`
- Summary: M-80 invalid: the L2 dispenser only deposits into factory-verified staking instances, and the report does not establish an attacker-controlled reverting target path

## 2026-04-24T22:21:28Z `completed` `raw-finding`
- Summary: M-81 invalid: batch-wide deposit revert is only theoretical here because invalid targets are already isolated and the report does not show a verified OLAS staking instance that can realistically revert on deposit()

## 2026-04-24T22:22:48Z `completed` `raw-finding`
- Summary: H-82 invalid: the Polygon verifier follows the claimed assembly pattern, but the benchmark’s own valid polygonPayload passes through processBridgeData(), so the alleged dirty-bits OOG brick is not real

## 2026-04-24T22:23:52Z `completed` `raw-finding`
- Summary: H-83 invalid: the Polygon verifier uses GuardCM storage via delegatecall, and GuardCM already exposes setTargetSelectorChainIds() to populate the allowlist before bridge verification runs

## 2026-04-24T22:27:45Z `completed` `raw-finding`
- Summary: H-84 valid low/QA: same-block donation front-running can grief checkpoint() attempts, but it is an intentional flash-loan guard with per-attempt donation cost, not a permanent H/M protocol freeze

## 2026-04-24T22:28:47Z `completed` `raw-finding`
- Summary: H-85 invalid: this is another framing of the same V3 buyback slippage bug already captured earlier, not a root-cause-independent new H/M finding

## 2026-04-24T22:29:35Z `completed` `raw-finding`
- Summary: H-86 invalid: this fresh-pool fail-open framing still collapses into the same V3 buyback slippage root cause already captured earlier rather than creating a new independent H/M issue

## 2026-04-24T22:33:59Z `completed` `raw-finding`
- Summary: M-87 valid medium: V2 buyback stacks its pre-swap oracle tolerance and post-swap spot-relative bound, while the Balancer execution path still swaps with limit=0, leaving a distinct sandwichable value-extraction surface on protocol inventory

## 2026-04-24T22:34:54Z `completed` `raw-finding`
- Summary: H-88 valid low/QA: the fresh-pool observe() fail-open path in checkPoolAndGetCenterPrice() is real, but it is the same already-counted liquidity-manager spot-price slippage family rather than a root-cause-independent new H/M issue

## 2026-04-24T22:35:26Z `completed` `raw-finding`
- Summary: H-89 valid low/QA: the explicit staticcall fail-open in checkPoolAndGetCenterPrice() is real, but it is the same already-counted liquidity-manager spot-price slippage family rather than a distinct H/M bug

## 2026-04-24T22:36:31Z `completed` `raw-finding`
- Summary: H-90 valid low/QA: the report points at the same already-counted V2 buyback sandwichable slippage family, but its post-swap TWAP explanation is imprecise because getPrice() reads live spot rather than a frozen TWAP

## 2026-04-24T22:41:33Z `completed` `raw-finding`
- Summary: H-91 valid low/QA: the strict bool-returning approve/transfer incompatibility with USDT-style tokens is real, but it is the same already-counted unsupported-token compatibility family rather than a new H/M bug

## 2026-04-24T22:42:25Z `completed` `raw-finding`
- Summary: M-92 valid low/QA: this is another duplicate of the same strict bool-returning USDT/non-compliant-token compatibility family already counted earlier in the buyback burner

## 2026-04-24T22:52:32Z `completed` `raw-finding`
- Summary: H-93 valid low/QA: this is the same already-counted V3 buyback min-out-disabled / execution-price family, restated as an MEV theft scenario rather than a new H/M root cause

## 2026-04-24T22:53:08Z `completed` `raw-finding`
- Summary: M-94 invalid: this is the same permissionless protocol-routing overclaim already rejected earlier, since transfer() only routes funds between treasury and bridge2Burner while buyBack() itself is permissionless

## 2026-04-24T22:53:38Z `completed` `raw-finding`
- Summary: H-95 valid low/QA: this is another duplicate of the same already-counted V3 buyback min-out-disabled / execution-price family, framed through low-liquidity pool conditions

## 2026-04-24T22:54:19Z `completed` `raw-finding`
- Summary: M-96 valid low/QA: this is another duplicate of the same strict bool-returning USDT/non-standard-token compatibility family already counted in the buyback burner

## 2026-04-24T22:56:35Z `completed` `raw-finding`
- Summary: M-97 valid low/QA: this bundled the already-counted V2 and V3 buyback execution-bound weaknesses into one composite slippage narrative rather than adding a new independent root cause

## 2026-04-24T22:57:16Z `completed` `raw-finding`
- Summary: M-98 valid low/QA: this is another duplicate of the strict-ABI USDT/non-standard-token approval family, with the usual zero-first allowance nuance but no new benchmarked H/M path

## 2026-04-24T22:57:51Z `completed` `raw-finding`
- Summary: M-99 valid low/QA: this is another duplicate of the same USDT/non-standard-token strict-ABI compatibility family already counted in the Balancer buyback burner

## 2026-04-24T23:00:11Z `completed` `raw-finding`
- Summary: M-100 valid low/QA: this bundled the already-counted Uniswap V2 and V3 buyback execution-bound weaknesses into one composite sandwich/slippage narrative rather than a new independent root cause

## 2026-04-24T23:02:07Z `completed` `raw-finding`
- Summary: M-101 invalid: the automation cursor only gets stuck if an external registry call consistently reverts, and the report does not show a realistic benchmark-valid service path that creates that permanent failure

## 2026-04-24T23:02:46Z `completed` `raw-finding`
- Summary: M-102 valid low/QA: this is the same already-counted stale multisig/agent mapping desync, framed as the new multisig being blocked rather than the old multisig staying authorized

## 2026-04-24T23:07:09Z `completed` `raw-finding`
- Summary: H-103 invalid: GuardCM intentionally allows direct non-timelock calls, and the benchmark tests explicitly exercise guarded token and ETH transfers so this behavior is disclosed and out of scope

## 2026-04-24T23:07:58Z `completed` `raw-finding`
- Summary: M-104 invalid: the CM is intentionally given timelock proposer, executor, and canceller roles, and the GuardCM tests rely on guarded timelock.execute calls as normal behavior

## 2026-04-24T23:08:29Z `completed` `raw-finding`
- Summary: H-105 invalid: this just combines the two already-disclosed GuardCM behaviors that the benchmark tests and deployment docs treat as intended CM authority

## 2026-04-24T23:11:53Z `completed` `raw-finding`
- Summary: H-106 valid low/QA: the unmasked payloadLength read is a real VerifyBridgedData dirty-bits parser bug, but it is part of the already-repeated/disclosed bridge-verifier assembly cluster rather than a fresh High

## 2026-04-24T23:13:34Z `completed` `raw-finding`
- Summary: H-107 valid low/QA: registerAgentsWithSignature really skips the operator whitelist, but exploiting it still requires both the service owner and an operator signature for the exact registration payload

## 2026-04-24T23:14:03Z `completed` `raw-finding`
- Summary: M-108 valid low/QA: operator signatures and approvals really have no deadline or revocation path, but the risk depends on the operator having already authorized the exact action and often still leaving allowance or balance available

## 2026-04-24T23:15:15Z `completed` `raw-finding`
- Summary: H-109 valid medium: registerAgentsWithSignature does not bind bond terms, so a service owner can update service bond requirements before consuming an old operator signature and charge the now-higher current bond

## 2026-04-24T23:20:26Z `completed` `raw-finding`
- Summary: H-110 valid low/QA: recoverAccess really builds multisend data with quadratic bytes.concat growth, but the exploit requires already-authorized Safe owners to deliberately bloat the owner set before recovery

## 2026-04-24T23:22:23Z `completed` `raw-finding`
- Summary: H-111 valid medium: recoverAccess only unlocks in PreRegistration, and a contract operator that rejects ETH can make unbond revert forever and keep the service stuck in TerminatedBonded

## 2026-04-24T23:24:16Z `completed` `raw-finding`
- Summary: H-112 valid low/QA: Safe owners can remove the recovery module and break later recoverAccess, but the report overstates this as a slashing bypass because slash is only callable while the service is still Deployed

## 2026-04-24T23:24:55Z `completed` `raw-finding`
- Summary: H-113 valid low/QA duplicate: this is the same Safe-owner inflation plus quadratic bytes.concat recovery-DoS mechanism already captured in H-110, not a new independent High

## 2026-04-24T23:28:33Z `completed` `raw-finding`
- Summary: H-114 valid low/QA: BalancerPriceOracle really has a fail-closed lifetime-average liveness weakness, but the report overstates it as an ordinary permanent brick because failed updates still mutate cumulativePrice and the repo config uses 50% oracle slippage

## 2026-04-24T23:29:25Z `completed` `raw-finding`
- Summary: M-115 valid low/QA: BalancerPriceOracle really uses cumulativePrice divided by a mutable averagePrice as a time proxy, causing average drift, but the benchmarked 50% slippage setup keeps this as a low-severity correctness issue

## 2026-04-24T23:30:02Z `completed` `raw-finding`
- Summary: H-116 valid low/QA duplicate: this combines the same Balancer oracle math-drift and fail-closed liveness issues already captured in H-114 and M-115, rather than adding a new High-severity root cause

## 2026-04-24T23:30:40Z `completed` `raw-finding`
- Summary: H-117 valid low/QA duplicate: this is the same Balancer oracle lifetime-average plus biased-math plus fail-closed liveness cluster already captured in H-114, M-115, and H-116

## 2026-04-24T23:35:44Z `completed` `raw-finding`
- Summary: H-118 valid low/QA duplicate: the V3 buyback path really lacks a post-trade price floor and swaps with amountOutMinimum 1 after only a pre-swap TWAP/spot check, but the dangerous pool must be owner-whitelisted so this is the same low-severity V3 slippage family rather than a new High

## 2026-04-24T23:37:33Z `completed` `raw-finding`
- Summary: M-119 valid low/QA duplicate: the V2 buyback path really stacks a pre-trade oracle tolerance with a post-trade check against pre-swap spot while swapping with zero min-out, but this is another low-severity slippage-guardrail issue in an owner-configured buyback path rather than a fresh Medium

## 2026-04-24T23:38:47Z `completed` `raw-finding`
- Summary: H-120 valid low/QA duplicate: the V3 buyback path ignores BuyBackBurner.maxSlippage and relies on a LiquidityManager pre-check whose deviation math is effectively non-binding, but this still collapses into the same low-severity V3 buyback slippage/min-out family rather than a fresh High

## 2026-04-24T23:39:56Z `completed` `raw-finding`
- Summary: M-121 invalid: the report imports a WAD-style slippage assumption from the V3 LiquidityManager path, but the V2 buyback/oracle wiring in this benchmark clearly uses plain percentage values like 50, so the claimed unit-mismatch underflow does not reflect the actual code path

## 2026-04-24T23:42:53Z `completed` `raw-finding`
- Summary: M-122 invalid: these liquidity operations are owner-only and compute min-amount bounds from on-chain state at execution time, so deadline=block.timestamp is at most a missing UX parameter, not an unlimited-slippage exploit path

## 2026-04-24T23:45:16Z `completed` `raw-finding`
- Summary: M-123 invalid: the report relies on the already-public LiquidityManager slippage/check bug family documented in the tokenomics test folder, which README marks out of scope, and it also overstates convertToV3 by claiming the optimizer directly uses manipulable spot rather than the TWAP-derived center price on the normal path

## 2026-04-24T23:45:57Z `completed` `raw-finding`
- Summary: H-124 invalid: the mature-pools explanation is technically wrong because the code reads the latest observation index, and the real fail-open/deviation issue it points at is already publicly documented in the tokenomics test folder, which README marks out of scope

## 2026-04-24T23:46:36Z `completed` `raw-finding`
- Summary: H-125 invalid: checkPoolAndGetCenterPrice really fails open to spot when the TWAP staticcall fails, but that exact oracle-guard bypass is already publicly documented in the tokenomics test folder, which README marks out of scope

## 2026-04-24T23:48:29Z `completed` `raw-finding`
- Summary: H-126 invalid: the variable-overwrite bug in checkPoolAndGetCenterPrice is real, but the exact broken-deviation logic is already publicly documented in the tokenomics test folder, which README marks out of scope

## 2026-04-24T23:49:03Z `completed` `raw-finding`
- Summary: M-127 invalid: this is another restatement of the already-public LiquidityManager slippage/deviation bug family from the tokenomics test folder, and rephrasing it as a hardcoded 10% tolerance does not make it a new in-scope Medium

## 2026-04-24T23:49:57Z `completed` `raw-finding`
- Summary: M-128 valid low/QA: BuyBackBurner uses raw IERC20 approve and transfer calls with bool-return interfaces, so returnless legacy tokens like USDT can revert buyback approvals and token sweeps, causing a real low-severity compatibility/stuck-funds issue

## 2026-04-24T23:50:49Z `completed` `raw-finding`
- Summary: M-129 valid low/QA: LiquidityManagerCore repeatedly uses raw non-zero approve calls to the position manager without a reset-to-zero pattern, so USDT-like tokens can revert later liquidity operations once residual allowance remains

## 2026-04-24T23:51:25Z `completed` `raw-finding`
- Summary: M-130 valid low/QA duplicate: BuyBackBurnerUniswap and BuyBackBurner use raw typed approve and transfer calls that expect bool returns, so returnless legacy tokens like USDT can break buybacks and sweeps as a real low-severity compatibility issue

## 2026-04-24T23:53:34Z `completed` `raw-finding`
- Summary: M-131 valid low/QA duplicate: LiquidityManagerCore.convertToV3 uses raw non-zero approve calls to the position manager without a reset-to-zero pattern, so USDT-like tokens can revert later pair-management operations once residual allowance remains

## 2026-04-24T23:54:27Z `completed` `raw-finding`
- Summary: M-132 valid low/QA: WormholeRelayerTimelock.transferTokens uses a raw typed approve call to the token bridge, so USDT-like non-standard tokens can revert bridging as a real low-severity compatibility issue

## 2026-04-24T23:56:38Z `completed` `raw-finding`
- Summary: H-133 valid low/QA duplicate: WormholeRelayerTimelock uses a raw typed approve call before bridging, so USDT-like tokens can leave funds stuck as a real low-severity compatibility issue rather than a new High

## 2026-04-24T23:57:01Z `completed` `raw-finding`
- Summary: M-134 valid low/QA duplicate: LiquidityManagerCore repeatedly uses raw non-zero approve calls to the position manager without a zero-reset pattern, so USDT-like tokens can brick later pool-management operations as a real low-severity compatibility issue

## 2026-04-24T23:57:44Z `completed` `raw-finding`
- Summary: M-135 valid low/QA duplicate: BuyBackBurnerUniswap uses raw bool-returning approve and sweep transfer calls, so USDT-like returnless tokens remain a real low-severity compatibility issue rather than a new Medium

## 2026-04-24T23:58:20Z `completed` `raw-finding`
- Summary: M-136 valid low/QA duplicate: BuyBackBurnerUniswap’s strict bool-returning approve path leaves USDT-like returnless tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-24T23:58:55Z `completed` `raw-finding`
- Summary: M-137 valid low/QA duplicate: BuyBackBurner’s strict transfer interface and the Uniswap burner’s raw approve calls keep USDT-like tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-24T23:59:40Z `completed` `raw-finding`
- Summary: M-138 valid low/QA duplicate: BuyBackBurner’s strict transfer interface keeps USDT-like returnless tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-25T00:02:02Z `completed` `raw-finding`
- Summary: M-139 valid low/QA duplicate: BuyBackBurner.transfer uses a strict bool-returning ERC20 transfer path, so USDT-like returnless tokens remain a real low-severity compatibility issue rather than a new Medium

## 2026-04-25T00:02:37Z `completed` `raw-finding`
- Summary: M-140 valid low/QA duplicate: BuyBackBurnerUniswap’s strict approve path and the shared sweep transfer path keep USDT-like returnless tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-25T00:03:09Z `completed` `raw-finding`
- Summary: H-141 valid low/QA duplicate: the buyback stack’s strict approve and transfer interfaces keep USDT-like returnless tokens as a real low-severity compatibility issue rather than a new High

## 2026-04-25T00:05:05Z `completed` `raw-finding`
- Summary: M-142 valid low/QA duplicate: LiquidityManagerCore’s strict bool-returning IToken approve path keeps USDT-like returnless tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-25T00:05:36Z `completed` `raw-finding`
- Summary: M-143 valid low/QA duplicate: LiquidityManagerCore combines strict bool-returning approve calls with raw non-zero allowance updates, keeping USDT-like tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-25T00:06:06Z `completed` `raw-finding`
- Summary: M-144 valid low/QA duplicate: LiquidityManagerCore’s missing zero-reset on repeated approve calls keeps USDT-like tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-25T00:08:00Z `completed` `raw-finding`
- Summary: M-145 valid low/QA duplicate: LiquidityManagerCore’s unsafe repeated approve pattern keeps USDT-like tokens as a real low-severity compatibility issue rather than a new Medium

## 2026-04-25T00:10:08Z `completed` `raw-finding`
- Summary: M-146 invalid speculative: IdentityRegistryBridger can overwrite cached multisig-to-agent mappings, but the claimed shared-multisig collision is not grounded in the real service deployment and same-address redeploy flow

## 2026-04-25T00:11:13Z `completed` `raw-finding`
- Summary: M-147 invalid speculative: VerifyData does alias chain IDs modulo 2^64, but the contract explicitly assumes a uint64 chain-id domain and the report does not ground a live >2^64 collision in this benchmark

## 2026-04-25T00:13:45Z `completed` `raw-finding`
- Summary: M-148 invalid: ServiceManager.update does not check the returned bool, but ServiceRegistry.update has no real false-return branch and instead reverts on failure, so the claimed state split is unreachable

## 2026-04-25T00:14:14Z `completed` `raw-finding`
- Summary: M-149 invalid speculative: VerifyData’s chain-id aliasing is the same out-of-domain uint64 packing issue as M-147 and is not grounded in a live benchmark-relevant collision

## 2026-04-25T00:18:51Z `completed` `raw-finding`
- Summary: L-150 invalid speculative: unchecked ERC20 return values exist, but the benchmarked buyback path is wired to standard OLAS and WETH rather than arbitrary false-return tokens

## 2026-04-25T00:21:13Z `completed` `raw-finding`
- Summary: M-151 invalid: the benchmarked UniswapPriceOracle is not a stored-TWAP oracle, so the missing updatePrice call does not create stale-price state

## 2026-04-25T00:22:02Z `completed` `raw-finding`
- Summary: M-152 invalid: IdentityRegistryBridgerProxy initializes atomically in its constructor, so the reported front-run initialize race does not exist on the benchmarked path

## 2026-04-25T00:24:10Z `completed` `raw-finding`
- Summary: H-153 invalid governance risk: GuardCM pause-after-defeated is an intended emergency power exercised by the privileged community multisig rather than an unprivileged bypass
