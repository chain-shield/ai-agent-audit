# 2026-04-monetrix Three-Shot Stage 1 Scope Screen

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2026-04-monetrix/report/audit-report-openai.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/2026-04-monetrix`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/monetrix-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/monetrix-scope.md`
- `/Users/apmfree/Desktop/Audit/2026-04-monetrix/README.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| M-1 | Observable yield injection can be sandwiched by late sUSDM depositors to capture existing stakers' yield | Keep | High | in-scope | Similar to V12 late-staker sniping but severity differs from V12 High |
| M-2 | Non-atomic hedge batches can leave MonetrixVault with one-sided spot or perp exposure | Keep | High | in-scope | Hedge and CoreWriter execution behavior is in scope and not a known issue |
| M-3 | Unsolicited USDC can become distributable yield through YieldEscrow live-balance accounting | Keep | High | in-scope | Similar to V12 live-balance issue but severity differs from V12 High |
| H-4 | Live supply APR cap lets late stakers front-run settlement and capture yield accrued before they joined | Keep | High | in-scope | Distinct live-supply APR cap root from V12 pending-yield sniping |
| M-5 | Multisig keeperBridge records principal that Vault cannot bridge back for redemptions | Keep | High | in-scope | Configured bridge accounting path with different impact than V12 multisig-backing item |
| M-6 | Multisig bridge target inflates Vault principal that the Vault cannot bridge back | Keep | High | in-scope | Configured bridge accounting path with different impact than V12 multisig-backing item |
| M-7 | Virtual share scaling in sUSDM._decimalsOffset can trap material injected yield after dust initialization | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-8 | ERC4626 virtual shares can permanently strand injected yield when sUSDM supply is dust-sized | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-9 | Redemption claims are first-come-first-served, letting later requests drain scarce escrow liquidity before earlier redeemers | Keep | High | in-scope | Redemption coverage under partial liquidity is in scope and has no V12 match |
| M-10 | Dust sUSDM supply can route almost all future user yield into ERC4626 virtual shares | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-11 | Misconfigured ERC4626 decimals offset can strand injected yield in sUSDM virtual shares | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-12 | Dust first stake lets virtual shares capture and strand injected sUSDM yield | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| H-13 | Dust first stake captures half of first sUSDM yield injection through virtual-share mis-scaling | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-14 | Hedge execution emits success without verifying both spot and perp legs filled | Keep | High | in-scope | Hedge and CoreWriter execution behavior is in scope and not a known issue |
| M-15 | Supplying to BLP registers accountant slots even when the HyperCore supply action is silently dropped | Keep | High | in-scope | BLP supply registration and precompile read behavior are in scope |
| M-16 | Late stakers can frontrun sUSDM.injectYield to capture yield earned before they joined | Keep | High | in-scope | Similar to V12 late-staker sniping but severity differs from V12 High |
| M-17 | Late sUSDM deposits can free-ride on already accrued yield injected through sUSDM.injectYield | Keep | High | in-scope | Similar to V12 late-staker sniping but severity differs from V12 High |
| M-18 | Low initial sUSDM supply lets injected yield be trapped in ERC4626 virtual shares | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-19 | L1 bridge-back accounting can be reduced using held or supplied balances that cannot satisfy SEND_ASSET | Keep | High | in-scope | Bridge-back accounting and HyperCore balance semantics are in scope |
| M-20 | Dust mint can lock nearly all injected yield in sUSDM virtual shares | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-21 | Hedge execution emits success without proving both spot and perp legs filled | Keep | High | in-scope | Hedge and CoreWriter execution behavior is in scope and not a known issue |
| M-22 | Independent hedge legs can leave MonetrixVault unhedged when one HyperCore order is dropped or partially filled | Keep | High | in-scope | Hedge and CoreWriter execution behavior is in scope and not a known issue |
| M-23 | PM supplied USDC is treated as bridgeable spot balance, allowing bridge-back accounting to decrease without funds arriving | Keep | High | in-scope | Bridge-back accounting and HyperCore balance semantics are in scope |
| M-24 | Held HyperCore USDC can falsely reduce outstanding principal in MonetrixVault.bridgePrincipalFromL1 | Keep | High | in-scope | Bridge-back accounting and HyperCore balance semantics are in scope |
| M-25 | Dust first stake strands injected yield in sUSDM virtual shares during cooldown | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-26 | Low-supply sUSDM deposits let virtual shares capture and strand injected yield | Keep | High | in-scope | Similar to V12 virtual-share dust item but severity differs from V12 Low |
| M-27 | Late deposits can frontrun sUSDM.injectYield and capture yield accrued before they joined | Keep | High | in-scope | Similar to V12 late-staker sniping but severity differs from V12 High |
| M-28 | Supplied USDC is counted as bridgeable, allowing silent bridge drops to corrupt principal accounting | Keep | High | in-scope | Bridge-back accounting and HyperCore balance semantics are in scope |
| M-29 | L1 bridge-back counts held HyperCore USDC as spendable and can permanently understate outstanding principal | Keep | High | in-scope | Bridge-back accounting and HyperCore balance semantics are in scope |
| H-30 | Dust sUSDM stake bypasses empty-vault reroute and captures pending user yield in MonetrixVault.distributeYield | Keep | High | in-scope | Differs in impact and severity from V12 virtual-share dust item |
| M-31 | Late sUSDM deposits can capture yield settled before the shares existed | Keep | High | in-scope | Similar to V12 late-staker sniping but severity differs from V12 High |
| M-32 | Donation over sUSDM injection cap can permanently DoS MonetrixVault.distributeYield | Keep | High | in-scope | Similar to V12 live-balance issue but severity differs from V12 High |
| H-33 | Uninitialized USDM proxy can be taken over to bind attacker as vault and mint unlimited USDM | Exclude | High | deployment-or-setup | Requires uninitialized proxy deployment or initialization mistake excluded by prompt |
| M-34 | HyperCore oracle prices are accepted without freshness or deviation checks when declaring distributable yield | Keep | Medium | in-scope | Oracle and precompile price semantics are in scope with no known or V12 blocker |
| M-35 | HyperCore oracle prices are accepted without freshness or deviation bounds, enabling false surplus settlement | Keep | Medium | in-scope | Oracle and precompile price semantics are in scope with no known or V12 blocker |
| M-36 | First claimant can drain scarce RedeemEscrow liquidity and force later redemption claims to revert | Keep | High | in-scope | Redemption coverage under partial liquidity is in scope and has no V12 match |
