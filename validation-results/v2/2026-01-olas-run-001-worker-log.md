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
