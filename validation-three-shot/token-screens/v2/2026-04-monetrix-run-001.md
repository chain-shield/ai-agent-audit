# 2026-04-monetrix Three-Shot Stage 2 Unsupported-Token Screen

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
| M-1 | Observable yield injection can be sandwiched by late sUSDM depositors to capture existing stakers' yield | Keep | High | supported-behavior | Uses protocol-native USDM and sUSDM yield injection, not unsupported token behavior. |
| M-2 | Non-atomic hedge batches can leave MonetrixVault with one-sided spot or perp exposure | Keep | High | supported-behavior | Concerns documented HyperCore spot and perp execution, not ERC20 semantics. |
| M-3 | Unsolicited USDC can become distributable yield through YieldEscrow live-balance accounting | Keep | High | supported-behavior | Uses supported USDC transfers and live escrow accounting, not non-standard token behavior. |
| H-4 | Live supply APR cap lets late stakers front-run settlement and capture yield accrued before they joined | Keep | High | supported-behavior | Depends on live USDM supply and sUSDM staking mechanics, not unsupported assets. |
| M-5 | Multisig keeperBridge records principal that Vault cannot bridge back for redemptions | Keep | High | supported-behavior | Uses supported USDC bridge accounting and documented multisig HyperCore routing. |
| M-6 | Multisig bridge target inflates Vault principal that the Vault cannot bridge back | Keep | High | supported-behavior | Uses supported USDC principal accounting across documented L1 accounts. |
| M-7 | Virtual share scaling in sUSDM._decimalsOffset can trap material injected yield after dust initialization | Keep | High | supported-behavior | USDM and sUSDM 6-decimal ERC4626 behavior is protocol-defined and supported. |
| M-8 | ERC4626 virtual shares can permanently strand injected yield when sUSDM supply is dust-sized | Keep | High | supported-behavior | Relies on protocol-defined sUSDM ERC4626 virtual shares, not exotic token decimals. |
| M-9 | Redemption claims are first-come-first-served, letting later requests drain scarce escrow liquidity before earlier redeemers | Keep | High | supported-behavior | Concerns supported USDC redemption escrow ordering, not non-standard token behavior. |
| M-10 | Dust sUSDM supply can route almost all future user yield into ERC4626 virtual shares | Keep | High | supported-behavior | Uses protocol-native sUSDM virtual-share accounting, not unsupported token semantics. |
| M-11 | Misconfigured ERC4626 decimals offset can strand injected yield in sUSDM virtual shares | Keep | High | supported-behavior | Uses documented 6-decimal USDM and protocol sUSDM ERC4626 configuration. |
| M-12 | Dust first stake lets virtual shares capture and strand injected sUSDM yield | Keep | High | supported-behavior | Depends on protocol-defined sUSDM virtual shares and USDM yield injection. |
| H-13 | Dust first stake captures half of first sUSDM yield injection through virtual-share mis-scaling | Keep | High | supported-behavior | Depends on native USDM and sUSDM 6-decimal ERC4626 math, not unsupported tokens. |
| M-14 | Hedge execution emits success without verifying both spot and perp legs filled | Keep | High | supported-behavior | Concerns documented HyperCore spot and perp action semantics. |
| M-15 | Supplying to BLP registers accountant slots even when the HyperCore supply action is silently dropped | Keep | High | supported-behavior | Concerns documented HyperCore supply and accounting semantics, not token quirks. |
| M-16 | Late stakers can frontrun sUSDM.injectYield to capture yield earned before they joined | Keep | High | supported-behavior | Uses protocol-native sUSDM deposit and USDM yield injection behavior. |
| M-17 | Late sUSDM deposits can free-ride on already accrued yield injected through sUSDM.injectYield | Keep | High | supported-behavior | Uses supported USDM and sUSDM accounting, not unsupported token behavior. |
| M-18 | Low initial sUSDM supply lets injected yield be trapped in ERC4626 virtual shares | Keep | High | supported-behavior | Relies on protocol-defined sUSDM ERC4626 virtual-share math. |
| M-19 | L1 bridge-back accounting can be reduced using held or supplied balances that cannot satisfy SEND_ASSET | Keep | High | supported-behavior | Uses documented HyperCore held and PM supplied USDC balance semantics. |
| M-20 | Dust mint can lock nearly all injected yield in sUSDM virtual shares | Keep | High | supported-behavior | Uses protocol-native sUSDM minting and virtual-share accounting. |
| M-21 | Hedge execution emits success without proving both spot and perp legs filled | Keep | High | supported-behavior | Concerns supported HyperCore hedge action semantics, not ERC20 behavior. |
| M-22 | Independent hedge legs can leave MonetrixVault unhedged when one HyperCore order is dropped or partially filled | Keep | High | supported-behavior | Concerns documented HyperCore order execution semantics. |
| M-23 | PM supplied USDC is treated as bridgeable spot balance, allowing bridge-back accounting to decrease without funds arriving | Keep | High | supported-behavior | Uses documented PM supplied USDC and bridge accounting semantics. |
| M-24 | Held HyperCore USDC can falsely reduce outstanding principal in MonetrixVault.bridgePrincipalFromL1 | Keep | High | supported-behavior | Uses documented HyperCore USDC total and hold balance fields. |
| M-25 | Dust first stake strands injected yield in sUSDM virtual shares during cooldown | Keep | High | supported-behavior | Relies on protocol-defined sUSDM cooldown and ERC4626 virtual-share math. |
| M-26 | Low-supply sUSDM deposits let virtual shares capture and strand injected yield | Keep | High | supported-behavior | Uses native sUSDM 6-decimal ERC4626 behavior and USDM yield injection. |
| M-27 | Late deposits can frontrun sUSDM.injectYield and capture yield accrued before they joined | Keep | High | supported-behavior | Depends on supported USDM staking and sUSDM yield injection mechanics. |
| M-28 | Supplied USDC is counted as bridgeable, allowing silent bridge drops to corrupt principal accounting | Keep | High | supported-behavior | Uses documented PM supplied USDC and HyperCore bridge semantics. |
| M-29 | L1 bridge-back counts held HyperCore USDC as spendable and can permanently understate outstanding principal | Keep | High | supported-behavior | Uses documented HyperCore USDC hold semantics, not token quirks. |
| H-30 | Dust sUSDM stake bypasses empty-vault reroute and captures pending user yield in MonetrixVault.distributeYield | Keep | High | supported-behavior | Uses protocol-native sUSDM supply gating and USDM yield distribution. |
| M-31 | Late sUSDM deposits can capture yield settled before the shares existed | Keep | High | supported-behavior | Depends on supported USDC settlement and sUSDM distribution timing. |
| M-32 | Donation over sUSDM injection cap can permanently DoS MonetrixVault.distributeYield | Keep | High | supported-behavior | Uses ordinary supported USDC transfers to YieldEscrow, not non-standard token behavior. |
| M-34 | HyperCore oracle prices are accepted without freshness or deviation checks when declaring distributable yield | Keep | High | supported-behavior | Concerns documented HyperCore oracle valuation of supported backing assets. |
| M-35 | HyperCore oracle prices are accepted without freshness or deviation bounds, enabling false surplus settlement | Keep | High | supported-behavior | Concerns supported HyperCore oracle and backing valuation semantics. |
| M-36 | First claimant can drain scarce RedeemEscrow liquidity and force later redemption claims to revert | Keep | High | supported-behavior | Concerns supported USDC redemption escrow liquidity ordering. |
