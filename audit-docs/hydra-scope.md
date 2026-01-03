## Hybra Finance audit scope

**NOTE**: NO_VOTING_WINDOW is temporarily set to 0 for convenience. However, in the actual on-chain deployment, NO_VOTING_WINDOW will not be zero — there will be a time buffer before and after each epoch flip during which voting is disabled. This ensures that during the Distributor phase, all voting weights remain fixed and cannot be changed.

The issues identified in [Peckshield's September 2025 audit report](https://github.com/peckshield/publications/blob/master/audit_reports/PeckShield-Audit-Report-Hybra-ve33-v1.0.pdf) are considered publicly known issues and are therefore ineligible for awards, including:

1. Possible ERC7702 Incompatibility in Contract Check
2. Voting Delegate Denial-of-Service With Dust Delegates
3. Trust Issue of Admin Keys
4. Improved Dynamic Fee Calculation in `DynamicSwapFeeModule`
5. Improper `estimateAmount0/1()` logic in `SugarHelper`

Additionally, the codebase at hand is a fork of Blackhole. As such, any findings that have been acknowledged in the original Blackhole audit are ineligible for a reward **unless their impact has been increased by the code delta introduced by the Hybra Finance team**.

### Known Limitations

1. CL pools use Solidity 0.7.6, ve33 uses 0.8.13
2. Cross-repo dependency management via bytecode export
3. Integration tests require sequential deployment

## Overview

This repository contains the complete Hybra Finance protocol for Code4rena audit, including:

- **ve33**: Vote-escrowed tokenomics and gauge system
- **cl**: Concentrated liquidity AMM (Uniswap V3 fork)

The projects are meant to be independently compiled, with the `ve33` codebase directly integrating the `cl` contracts and thus depending on them.

The scope of the project involves a subset of the full contract list in this repository, and should be consulted to ensure wardens invest their time and effort solely in the in-scope contracts.

The project at hand is a fork of Blackhole and thus implements the following features:

- Curve's vote-escrow
- Uniswap V3's clAMM pools
- Custom Fee pools
- Enhanced ve(3, 3) token design

## Links

- **Previous audits:**
  - [Code4rena: Original Blackhole Contest](https://code4rena.com/reports/2025-05-blackhole)
  - [Peckshield: September 29, 2025](https://github.com/peckshield/publications/blob/master/audit_reports/PeckShield-Audit-Report-Hybra-ve33-v1.0.pdf)
- **Documentation:** https://hybra-foundation.gitbook.io/hybra-foundation/
- **Website:** https://www.hybra.finance/
- **X/Twitter:** https://x.com/hybrafinance

---

# Scope

The scope of the project involves a subset of the full contract list in this repository, and should be consulted to ensure wardens invest their time and effort solely in the in-scope contracts.

### Files in scope

| Contract                                                                                                                                                         |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [ve33/contracts/GaugeManager.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/GaugeManager.sol)                                 |
| [ve33/contracts/GaugeV2.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/GaugeV2.sol)                                           |
| [ve33/contracts/MinterUpgradeable.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/MinterUpgradeable.sol)                       |
| [ve33/contracts/VoterV3.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/VoterV3.sol)                                           |
| [ve33/contracts/VotingEscrow.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/VotingEscrow.sol)                                 |
| [ve33/contracts/GovernanceHYBR.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/GovernanceHYBR.sol)                             |
| [ve33/contracts/HYBR.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/HYBR.sol)                                                 |
| [ve33/contracts/RewardHYBR.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/RewardHYBR.sol)                                     |
| [ve33/contracts/swapper/HybrSwapper.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/swapper/HybrSwapper.sol)                   |
| [ve33/contracts/CLGauge/GaugeCL.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/CLGauge/GaugeCL.sol)                           |
| [ve33/contracts/CLGauge/GaugeFactoryCL.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/ve33/contracts/CLGauge/GaugeFactoryCL.sol)             |
| [cl/contracts/core/CLFactory.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/cl/contracts/core/CLFactory.sol)                                 |
| [cl/contracts/core/CLPool.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/cl/contracts/core/CLPool.sol)                                       |
| [cl/contracts/core/fees/DynamicSwapFeeModule.sol](https://github.com/code-423n4/2025-10-hybra-finance/blob/main/cl/contracts/core/fees/DynamicSwapFeeModule.sol) |

### Files out of scope

Any files not explicitly included in the aforementioned list of contracts are expected to be out-of-scope.

# Additional context

## Areas of concern (where to focus for bugs)

### 1. CL Gauge

- When users deposit a tokenId into the NFT, does the reward distribution from emissions correctly match the active liquidity range they provide, or could there be a mismatch in reward calculations?

### 2. ve(3,3) Epoch Cycle, Rotation, and Reward Distribution / Rebase

- For the entire ve(3,3) epoch cycle rollover: is the process of reward distribution (Distribute), rebase, etc. functioning correctly? Could there be cases of uneven reward allocation? If the epoch rollover fails, would it cause impact? When claiming rewards across epochs, can users still fully claim all rewards from previous epochs?
- For voters claiming rewards, if rewards have already been distributed but not yet fully claimed, what happens when they extend their lock duration or merge positions — will this affect the reward data?

### 3. RewardHYBR

- During the reward claiming process, when converting rewards into veHYBR, HYBR, or gHYBR, can the claim always be executed successfully under all circumstances?
- Could there be any asset loss or abnormal situations?

### 4. gHYBR

- Are there any potential fund security vulnerabilities within gHYBR?
- For deposits and withdrawals, are fund flows handled correctly? When splitting, slicing, or merging veNFTs within gHYBR, does the system ensure funds remain intact and properly accounted for?

## Main invariants

### 1. Global & Supply

- **[G-1] Token Supply Conservation**
  No token within the protocol may be minted or burned out of thin air: any increase or decrease in supply must originate from clearly defined mint/burn paths (e.g., emissions, penalty burns, treasury mints). All amounts and events must be auditable and traceable.

- **[G-2] Balance Conservation**
  On-chain balances must always be greater than or equal to the sum of all accrued/claimable balances (rewards, bribes, fees). Negative balances or overflows must never occur.

- **[G-3] Time Monotonicity**
  Time-dependent cumulative quantities (e.g., feeGrowth, rewardDebt, emission distributions) must only increase monotonically or decay according to defined rules; they must never retrogress.

---

### 2. Locks & Voting Power (veNFT / Lock / Voting Power)

- **[V-1] Lock Bounds**
  `lock_start < lock_end ≤ lock_start + MAX_LOCK`; MAX_LOCK cannot be bypassed or extended. Early withdrawals before expiry are prohibited unless explicitly documented via penalty or escape-hatch mechanisms.

- **[V-2] Voting Power Decay**
  The derivative of veBalance with respect to time must be non-positive: without relocking or extending the lock, `veBalance(t+Δ) ≤ veBalance(t)`.

- **[V-3] Transfer Rules**
  Transfers, merges, and splits of veNFTs must follow rules:

  - _Merge_: New NFT voting power = result of deterministic merge logic (typically a function of amounts and remaining durations). No artificial increase of voting power is allowed.
  - _Split_: Sum of child NFT weights = parent NFT weight (evaluated at the same timestamp).
  - Restrictions must be consistent and verifiable when NFTs are under active votes or pending rewards/bribes.

- **[V-4] Delegation & Caps**
  Delegation only reassigns accounting, without changing the global voting power. If MAX_DELEGATES or checkpoint limits exist, failed writes are By Design but must never break settlement or compromise fund safety of other accounts.

---

### 3. Voting, Gauges & Epochs

- **[W-1] Weight Sum Conservation**
  Within an epoch, the sum of all gauge weights = the effective total voting power (or its normalized form). A single veNFT’s total vote allocation ≤ its available voting power.

- **[W-2] Epoch Boundary Consistency**
  Weight snapshots take effect strictly at epoch boundaries. Updates in one epoch must not retroactively affect settled allocations from the previous epoch.

- **[W-3] No Overvote / No Reuse**
  The same veNFT cannot have its voting share double-counted within the same window. Unvoting/revoting must follow predictable activation timing.

---

### 4. Emissions & Bribes

- **[E-1] Emission Cap**
  Per-epoch emissions must not exceed the formula or governance-set budget. No hidden inflation is allowed.

- **[E-2] Proportionality**
  Emissions allocated to gauges must be strictly proportional to their effective weight snapshots. Gauges with zero weight must receive zero emissions.

- **[E-3] Bribe Accounting Conservation**
  For each bribe pool, the accounting identity must hold at all times:
  `inflows – claimed – refunds = remaining ≥ 0`. Negative values must never occur.

- **[E-4] Multi-Token & Precision**
  For multi-token rewards/bribes, accounting across different decimals/precisions must follow consistent rounding/dust-handling rules. User value must never be lost.

---

### 5. Rewards & Fees

- **[R-1] Non-negative Claimables**
  Any user/veNFT claimable balance ≥ 0; under no sequence or precision case may it become negative.

- **[R-2] Monotone Accrual**
  Without user interaction, `claimable(t+Δ) ≥ claimable(t)` (or, after claims reduce the balance, subsequent accrual resumes monotonically increasing).

- **[R-3] Accrue-then-Claim**
  Claims must first checkpoint/accrue all pending rewards to ensure “slow users” are not diluted.

---

### 6. Access Control & Governance

- **[A-1] Least Privilege**
  Only governance/multisig may alter global critical parameters (emission rates, fees, whitelists, etc.). Regular users cannot escalate privilege.

- **[A-2] Upgrade/Pause Safety**
  Upgrades, pauses, or resumes must not alter existing entitlements or accrued balances. In paused state, critical invariants must still hold (assets can be safely withdrawn and cannot be siphoned).

---

### 7. Checkpointing

- **[C-1] Order Independence**
  Different valid call orders must not change the final cumulative results (audit guidance: use differential testing against reentrancy/order-dependence).

- **[C-2] Context Isolation**
  The settlement of a single veNFT must not depend on the settlement order of other veNFTs, unless explicitly documented as shared state.

---

### 8. Math & Bounds

- **[M-1] No Over/Underflow**
  All multiplications/divisions must be safe within bounded ranges (e.g., 256-bit/128-bit). Use safe-math utilities such as FullMath/mulDiv.

- **[M-2] Precision & Rounding**
  Explicit rounding strategy (“round down” / “round to nearest”) must be defined. Precision handling must not create exploitable arbitrage or liabilities.

## ALL TRUSTED ROLES IN THE PROTOCOL

Based on PermissionsRegistry contract analysis:

Core Multisigs

1. hybraMultisig (PermissionsRegistry:11)

- Main 4/6 multisig controlling the protocol
- Can add/remove roles, assign roles to addresses, change itself

2. hybraTeamMultisig (PermissionsRegistry:14)

- Team 2/2 multisig
- Can change itself

3. emergencyCouncil (PermissionsRegistry:17)

- Emergency functions control
- Can change itself

Role-Based Access Control

All roles defined in PermissionsRegistry constructor (lines 45-67):

4. GOVERNANCE - High-level governance decisions
5. VOTER_ADMIN - Manage voter contract settings
6. GAUGE_ADMIN - Manage gauge contracts
7. BRIBE_ADMIN - Manage bribe contracts

Additional Trusted Addresses

12. team (MinterUpgradeable:36) - Controls emission parameters and rewards
13. owner - Standard Ownable admin on various contracts"
