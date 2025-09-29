# Lazy Summer Protocol

## Lazy Vaults

Lazy Vaults (also called `Fleets`) are the primary user-facing component of the Summer Protocol. Each Lazy Vault represents a Fleet - a set of coordinated contracts that include a Fleet Commander, ARKs, and RAFT.

A Fleet exists for each deposit token type (DAI, USDC, ETH) and multiple Fleets may exist per token to accommodate different risk preferences. Every Fleet is supported by the RAFT contract which farms reward tokens and swaps farmed tokens to the underlying Fleet deposit token - boosting Fleet yields at no cost to the user.

At launch, Fleet ARKs support basic lending and staking positions. Over time, advanced ARKs will be introduced to Fleets (subject to governance approval). A concrete example would be an ARK that supports the underlying yield-looping stack that is already available on Summer.fi today.

## Key Components

### Fleet Commander

Controls asset allocation and manages user deposits/withdrawals. Issues shares to depositors representing their ownership of the vault's assets.

### ARKs

Specialized contracts that implement yield strategies through lending, staking, and other DeFi mechanisms. Each Fleet has multiple ARKs plus a Buffer ARK holding token for quick withdrawals.

### RAFT

Automatically harvests rewards from protocols, converts them to the vault's deposit token, and reinvests them to compound yields.

# Summer Governance Contracts

This package contains the core governance contracts for the Summer protocol.

## Main Contracts

### SummerGovernor.sol

`SummerGovernor` is the main governance contract for the Summer protocol. It extends various OpenZeppelin governance modules and includes custom functionality such as whitelisting and voting decay.

Key features:
- Cross-chain proposal execution
- Whitelisting system for proposers
- Integration with LayerZero for cross-chain messaging
- Custom voting power calculation with decay

### SummerToken.sol

`SummerToken` is the governance token for the Summer protocol. It extends OpenZeppelin's ERC20 implementation and includes additional features.

Key features:
- ERC20 with voting capabilities
- Integration with LayerZero's OFT (Omnichain Fungible Token)
- Built-in vesting wallet creation

### SummerVestingWallet.sol

`SummerVestingWallet` is a custom vesting wallet implementation for the Summer protocol.

Key features:
- Two vesting schedules: 6-month cliff and 2-year quarterly vesting
- Built on top of OpenZeppelin's VestingWallet

## Understanding Decayed Voting Power

The Summer protocol implements a unique voting decay mechanism where voting power gradually decreases over time unless actively managed. Here's how it works:

### Core **Concepts**

1. **Decay Rate**: A configurable rate (between 1% and 50% per year / TBD) at which voting power decreases
2. **Decay-Free Window**: An initial period (between 1 day and 180 days / TBD) where no decay occurs
3. **Delegation Chain**: A maximum of 2 levels of delegation are allowed before voting power is zeroed out
4. **Checkpoints**: Records of voting power at specific points in time

### Example Scenarios

#### Scenario 1: Basic Decay

Alice holds 1000 SUMMER tokens and self-delegates:
```
T+0:   Alice self-delegates
       Initial voting power = 1000
       Decay checkpoint created with factor = 1.0 (WAD)

T+30d: Still within decay-free window
       Voting power = 1000 (no decay)

T+395d: After 1 year (including decay-free window)
       Voting power ≈ 900 (10% decay applied)
```

#### Scenario 2: Delegation Chain

Bob has 1000 SUMMER tokens and delegates through a chain:
```
T+0:   Bob delegates to Charlie
       Charlie delegates to Alice
       Valid chain (depth = 2)
       Bob's tokens contribute to Alice's voting power

T+0:   If Alice then delegates to Dave
       Chain becomes: Bob -> Charlie -> Alice -> Dave
       Voting power = 0 (exceeds MAX_DELEGATION_DEPTH)
```

#### Scenario 3: Resetting Decay

Charlie wants to reset their decay factor:
```
1. Create new wallet
2. Transfer tokens to new wallet
3. Delegate from new wallet
   - New decay checkpoint created
   - Decay factor reset to WAD (1.0)
   - New decay-free window begins
```

### Best Practices

1. **Active Management**
   - Regularly update your delegation to refresh decay factors
   - Consider re-delegating before important votes

2. **Delegation Strategy**
   - Keep delegation chains short (max 2 levels)
   - Monitor your current decay factor using `getDecayFactor()`

3. **Decay Protection**
   - Use the decay-free window strategically
   - Create new wallets for fresh decay factors when needed
   - Consider splitting holdings across multiple addresses to manage decay risk

### Technical Details

- Decay is calculated using either linear or exponential functions
- Historical voting power uses current decay factors (not historical ones)
- Checkpoints are created on:
  - Initial delegation
  - Token transfers
  - Decay factor updates
  - Delegation changes

### Important Notes

1. **Irreversible**: Decay cannot be reversed without using a new wallet
2. **Compound Effect**: Long delegation chains can result in zero voting power
3. **Checkpoint Impact**: All voting power calculations use the current decay factor, even for historical queries

### Understanding Delegation vs Decay Inheritance

A critical distinction exists between how voting power and decay factors are handled in delegation chains:

#### Voting Power Flow
```
Alice (1000 SUMMER) -> Bob -> Charlie
Result: Charlie has 1000 voting power
        Bob has 0 voting power
        Alice has 0 voting power
```

#### Decay Factor Inheritance (Reverse Flow)
```
Alice (holder) -> Bob -> Charlie (final delegate)
Result: Alice's decay factor = Charlie's decay factor
        If Charlie is inactive, Alice's rewards decay
        If Bob changes delegate, Alice's decay follows the new chain
```

### Why This Matters

1. **Voting Power**
   - Flows forward through the delegation chain
   - Only the final delegate can use the voting power
   - Previous delegates in the chain have zero voting power

2. **Decay Factor**
   - Inherited backwards through the delegation chain
   - Token holders (like Alice) are affected by their delegate's inactivity
   - Used for both voting power AND rewards calculation
   - Encourages choosing active delegates

### Example Scenario

```
T+0:   Alice holds 1000 SUMMER
       Delegates to Bob, who delegates to Charlie
       All decay factors = 1.0 (WAD)

T+60d: Charlie becomes inactive
       Charlie's decay factor begins decreasing
       Alice's rewards also decay, even though she's active
       Because: Alice inherits Charlie's decay factor

T+90d: Bob changes delegate to Dave (active user)
       Alice's decay factor now follows Dave
       Alice's rewards begin to recover
```

### Strategic Implications

1. **For Token Holders**
   - Monitor your delegate's activity
   - Your rewards depend on your delegate's engagement
   - Consider direct delegation to active participants

2. **For Delegates**
   - Maintain regular activity to prevent decay
   - Communicate delegation changes to your delegators
   - Understand you affect your delegators' reward rates


## Preliminary Documentation (for auditors)

### Overview (concise)

Governance v2 is a hub-and-satellite model. Voting happens exclusively on the hub chain using xSUMR (a non-transferable ERC20Votes token minted 1:1 for staked SUMR or vesting wallet/s/). Approved proposals are executed via a timelock on the hub and can be relayed cross-chain to satellites via LayerZero. Satellites do not accept proposals or votes; they only queue and execute hub-approved operations after the configured delay.

xSUMR mint/burn is restricted to authorized staking modules managed by governance. Guardians (tracked by `accessManager`) can propose below threshold and have specific cancellation and pause privileges, enforced by `SummerTimelockController` and `StakedSummerToken`.

### Staking weights and penalties (summary)

- **Weight multipliers (rewards only)**:
  - Weighted stake uses a quadratic time factor: `weighted = amount * (1 + 7e-16 * t^2)` where `t` is lockup seconds (capped at 3 years).
  - Rewards accounting uses weighted balances (the rewards `totalSupply` equals the sum of weighted stakes).
  - Governance voting power is based on xSUMR balance (1:1 minted for staked SUMR or vesting balances), not the weighted amount.
- **Penalty on early unstake**:
  - If penalties disabled → 0; if lockup expired → 0.
  - If remaining lockup < 110 days → flat 2%; else linear up to 20% at 3 years: `penaltyPct = 20% * (remaining / 3y)`.
  - Penalty applies to the amount being unstaked; penalty is transferred to `treasury()`, remainder to user.
- **Buckets & caps**:
  - Lockups are grouped into buckets (NoLockup, ShortTerm, 2w–3m, 3–6m, 6–12m, 1–2y, 2–3y) with governor-configurable caps; All buckets are disabled by default (cap = 0).
  - A user has a single portfolio; index 0 aggregates NoLockup; up to 1000 stakes; full portfolio can be migrated to a fresh target via `transferStakes(to)`.

### Actors and Roles (who can do what)

- **SUMR holder**: Holds the base token. Can stake into `SummerStaking` or stake via vesting wallets using `SummerVestingWalletsEscrow` to receive xSUMR.
- **xSUMR holder (voter)**: Has voting power on the hub chain. Can propose if voting power ≥ proposal threshold; can always vote on hub.
- **Proposer (threshold-based)**: Any xSUMR holder with voting power ≥ proposal threshold can propose on the hub.
- **Guardian (via `accessManager`)**: Can propose even below threshold on the hub; can cancel certain queued ops via timelock rules; can pause/unpause xSUMR alongside governor.
- **Governor (the governance process + addresses with governor role)**: Adds/removes staking modules on xSUMR, manages vesting factory allowlist in the escrow, toggles pause, can perform emergency minter role actions in xSUMR, and controls configuration via proposals executed through the timelock.
- **Timelock (`SummerTimelockController`)**: Schedules and executes approved operations after a delay. On satellites, anyone permitted by timelock executors can execute after delay.
- **Staking module (`SummerStaking`)**: When authorized by xSUMR, can `mint` and `burnFrom` xSUMR corresponding to stake/unstake flows; cannot transfer xSUMR between users.
- **Vesting wallet owner**: Can stake/unstake from approved vesting wallet factories via `SummerVestingWalletsEscrow` (escrow must already own the vesting wallet during staking period).
- **Protocol Access Manager (`IProtocolAccessManager`)**: Source of truth for guardianship and roles; queried by the governor and timelock for authorization decisions.
- **LayerZero endpoint**: Cross-chain transport used by the governor to distribute finalized proposals to satellites. Governor accepts ETH only from the endpoint or the timelock.

### Contract relationships (high level)

- `SummerGovernorV2`
  - Uses xSUMR (ERC20Votes) as the voting token (no decay in v2)
  - Owns/schedules through `SummerTimelockController`
  - Queries `accessManager` to check guardian status
  - Sends proposals to other chains via LayerZero OApp; satellites only queue received proposals
  - Hub-only for propose/vote/execute/cancel; satellites cannot run these (queue only)

- `StakedSummerToken` (xSUMR)
  - Non-transferable; only mint/burn flows
  - Governor adds/removes staking modules which receive `MINTER_ROLE` and `BURNER_ROLE`
  - `burnFrom` requires owner or `BURNER_ROLE` plus allowance; direct `grantRole`/`revokeRole` are disabled
  - Pausable by guardian/governor

- `SummerStaking`
  - Main staking with lockups (0–3y), weighted staking, penalties, and bucket caps
  - Mints/burns xSUMR 1:1 on stake/unstake via `WrappedStakingToken`
  - Weighted total supply drives rewards accounting

- `SummerVestingWalletsEscrow`
  - Allows staking from vesting wallets owned by the escrow
  - Governor manages allowed vesting factories
  - Mints/burns xSUMR equal to vesting wallet SUMR balance and tracks released amounts during stake

- `SummerTimelockController`
  - Enforces delay and specialized cancellation rules (governors vs guardians; guardian-expiry operations restricted)
  - On satellites, executes queued operations after delay without hub voting (execution path is via timelock)

### Hub/Satellite Governance and Voting Flow

1. Stake to obtain votes
   - Users lock SUMR in `SummerStaking` (or via `SummerVestingWalletsEscrow`) and receive non-transferable xSUMR.
2. Propose (hub-only)
   - Any address with votes ≥ proposal threshold can propose on the hub; guardians can propose even below threshold.
3. Vote (hub-only)
   - xSUMR holders vote; quorum and counting follow OpenZeppelin Governor modules.
4. Queue and execute on hub
   - Successful proposals are queued and then executed through `SummerTimelockController` after the delay.
5. Distribute cross-chain
   - `SummerGovernorV2.sendProposalToTargetChain()` is called on the hub to broadcast the finalized proposal to target chains via LayerZero.
6. Satellite behavior
   - Satellite governors receive the message and `_queueCrossChainProposal(...)` schedules operations in the local timelock. Propose/vote/execute/cancel remain disabled on satellites. After the delay, the satellite timelock executes (per its executor permissions).
7. Safeguards
   - Governor/guardian can pause xSUMR; guardianship checked via `accessManager`; governor accepts ETH only from LayerZero endpoint or the timelock.

### Testing and Coverage (audited scope)

- Build/tests for this package:

```bash
pnpm -F @summerfi/earn-gov-contracts build
pnpm -F @summerfi/earn-gov-contracts test
pnpm -F @summerfi/earn-gov-contracts coverage
pnpm -F @summerfi/earn-gov-contracts coverage:report
```

- Expectations:
  - Tests: 100% passing. Each test should include at least one failure path (e.g., `expectRevert`).
  - Coverage: >80% lines/branches across audited contracts (`SummerStaking.sol`, `SummerVestingWalletsEscrow.sol`, `SummerGovernorV2.sol`, `StakedSummerToken.sol`).
  - Clean environment instructions are in the repo root `README.md`.

### What changed vs Governance v1

- Voting decay is removed in v2. Governance token remains xSUMR (ERC20Votes), but without time-based decay mechanics. Hub/satellite architecture and guardian model are preserved, simplifying analysis and operations.

## Executive Summary for Auditors

This audit covers **new contracts** that extend previously audited functionality. The focus is on:
- **StakedSummerToken.sol** - Governance token with controlled minting
- **SummerGovernorV2.sol** - Governance without voting decay (vs V1 with decay)
- **SummerVestingWalletsEscrow.sol** - MVP staking bridge for vesting wallets
- **SummerStaking.sol** - Advanced staking with lockup periods and weighted rewards

## Key Architectural Changes

### 1. StakedSummerToken.sol - Governance Token Design

**Purpose**: This is the **governance token** (xSUMR) that represents staked SUMMER tokens with voting power.

**Critical Security Features**:
- **Controlled Minting**: Only `MINTER_ROLE` holders can mint
- **Role Management**: Direct `grantRole`/`revokeRole` are **disabled**; only governor can add/remove staking modules or (emergency) grant/revoke minter role
- **Non-transferable**: xSUMR disables user-to-user transfers. Only mint (from address(0)) and burn (to address(0)) are allowed
- **Pausable**: Governor/Guardian can pause, which blocks mint/burn while paused
**Multiple Staking Modules Support**:
- **Before**: Single `stakingModule` address with direct role assignment
- **After**: Multiple staking modules can be added/removed dynamically

**Role Assignment Flow**:
```solidity
addStakingModule() → 
  - Grants MINTER_ROLE to new staking module
  - Grants BURNER_ROLE to new staking module
  - Emits StakingModuleAdded event

removeStakingModule() → 
  - Revokes MINTER_ROLE from staking module
  - Revokes BURNER_ROLE from staking module  
  - Emits StakingModuleRemoved event

// Emergency (governor-only, not part of normal flow)
grantMinterRole(_minter)
revokeMinterRole(_minter)
```

### **Security Implications**

**Enhanced Flexibility**:
- ✅ Multiple staking contracts can mint xSUMR tokens
- ✅ Each staking module has independent minting/burning authority
- ✅ Governor can add/remove staking modules without redeployment

**Risk Considerations**:
- 🔴 **Multiple Minters**: More attack surface - each staking module can mint
- 🔴 **Role Proliferation**: Each added module gets both MINTER and BURNER roles
- 🔴 **Governance Control**: Only governor can add/remove modules

**Authorization Nuances**:
- Burning uses `burnFrom(from, amount)` and enforces standard ERC20 allowances. Having `BURNER_ROLE` does not bypass allowances unless burning own balance. This reduces blast radius of a compromised burner.

**Audit Focus Areas**:
- Role escalation prevention (direct role granting disabled)
- Staking module address validation
- Minting/burning authorization flow (BURNER_ROLE + allowance model)
- Pause semantics impact on mint/burn and governance snapshots

### 2. SummerGovernorV2.sol vs SummerGovernor.sol

**Key Difference**: **V2 removes voting decay functionality**

| Feature | V1 (Audited) | V2 (New) |
|---------|---------------|-----------|
| Voting Decay | ✅ `DecayController` | ❌ Removed |
| `updateDecay` modifier | ✅ Present | ❌ Removed |
| Cross-chain messaging | ✅ LayerZero | ✅ LayerZero |
| Guardian system | ✅ Active | ✅ Active |

**Why This Matters**: V2 simplifies governance by removing time-based voting power decay.

**Hub/Satellite Model**:
- Proposals, votes, execute, cancel: restricted to the hub chain via `onlyHubChain`
- Cross-chain distribution: `sendProposalToTargetChain()` (hub-only) sends to satellites using LayerZero OApp
- Satellite chains queue received proposals via `_queueCrossChainProposal` guarded by `onlySatelliteChain`
- Contract `receive()` accepts ETH only from LayerZero endpoint or the timelock; others revert (`GovernorDisabledDeposit`)

**Thresholds and Guardians**:
- `MIN_PROPOSAL_THRESHOLD = 1,000e18`, `MAX_PROPOSAL_THRESHOLD = 100,000e18` (validated at construction)
- Proposers below threshold can still propose if they are active guardians (`isActiveGuardian` via `accessManager`)

### 3. SummerVestingWalletsEscrow.sol - MVP Staking Bridge

**Purpose**: Temporary staking solution that allows users to stake from vesting wallets.

**Critical Flows**:
```solidity
stakeVesting(address[] factories) →
  - Requires each `factory` is enabled
  - For each factory: resolve `vestingWallets(user)` (initial owner), require nonzero and escrow already owns it
  - Mint xSUMR equal to current SUMR balance in that vesting wallet
  - Track staked amount and the `released(token)` snapshot per factory -> cases where `release()` has been called permisionlessly while staked

unstakeVesting(address[] factories) →
  - For each factory: compute SUMR released while staked and forward to the user (if any)
  - Transfer vesting wallet ownership back to the user
  - Burn recorded xSUMR
  - remove `released` and `balance` tracking for factory/user pair - to enable consequent stakes
```

**Security Considerations**:
- **Ownership Expectations**: Escrow must already own the vesting wallets to stake from them; this contract does not transfer ownership during stake
- **Factory Validation**: Only pre-approved vesting factories allowed
- **Granular Operations**: Users may stake/unstake per-Factory by passing a list of factories

**Audit Focus Areas**:
- Vesting wallet ownership management
- Factory whitelist validation
- Token release calculations during staking

**User/Operator API**:
- `addVestingFactory(address)` / `removeVestingFactory(address)`: governor-only
- `rescueWallet(wallet, newOwner)`: governor-only safety valve
- `rescueToken(token, to)`: governor-only
- Views: `vestingFactories()`, `getVestingFactory(index)`, `userStakedVestingFactories(user)`, `getUserStakedVestingFactory(user, index)`

**Events**:
- `StakedVestingWallet(user, factory, balance, releasedAtStake)`
- `UnstakedVestingWallet(user, factory, balance, releasedAtUnstake)`

### 4. SummerStaking.sol - Advanced Staking with Lockups

**Purpose**: Main staking contract with lockup periods, weighted rewards, and bucket-based caps.

**Key Features**:
- **Lockup Periods**: 0 to 3 years (0 = no lockup via a dedicated aggregated stake at index 0)
- **Weighted Staking**: Longer lockups = higher reward multipliers (quadratic in time)
- **Bucket System**: Configurable caps per lockup duration
- **Penalty System**: Early unstaking incurs time-based penalties (governor can toggle penalties on/off)
- **Stake Portfolio Management**: One portfolio per address; index 0 aggregates no-lockup stake; up to 1000 stakes; full-portfolio transfer supported via `transferStakes(to)` to a fresh target
- **Default Caps**:
  - `NoLockup`: cap = 0 (by default - to be initialized by governance)
  - `ShortTerm` (1 sec – 14 days): cap = 0 (by default - to be initialized by governance)
  - `TwoWeeksToThreeMonths` (>14 days – 90 days): cap = 0 (by default - to be initialized by governance) 
  - `ThreeToSixMonths` (>90 – 180 days): cap = 0 (by default - to be initialized by governance) 
  - `SixToTwelveMonths` (>180 – 365 days): cap = 0 (by default - to be initialized by governance) 
  - `OneToTwoYears` (>365 – 730 days): cap = 0 (by default - to be initialized by governance) 
  - `TwoToThreeYears` (>730 – 1095 days): cap = 0 (by default - to be initialized by governance)


**Critical Calculations**:
```ts
// Weighted stake formula (UD60x18 fixed-point; time in seconds)
// WEIGHTED_STAKE_BASE = 1.0
// WEIGHTED_STAKE_COEFFICIENT = 7e-16
weightedAmount = amount * (1 + 7e-16 * time^2)

// Penalty calculation  
// If penalties disabled → 0
// If lockup expired → 0
// If timeRemaining < 110 days → 2% flat (according to the `formula` the fee is 2% at 9460800 seconds - 109.5days - but it was rounded up)
// Else → 20% * (timeRemaining / 3 years)
penaltyPct = remainingLockup < 110 days ? 2% : 20% * (remainingLockupTime / 3 years);
penaltyAmount = penaltyPct * amount;
```

<img width="2208" height="1370" alt="image" src="https://github.com/user-attachments/assets/9d52f110-42a5-405d-9555-99458c5bcedb" />


**Rewards Accounting**:
- The `totalSupply` used for rewards is actually the weighted total supply. Because the base rewards manager requires `rewardPerToken()` to use `totalSupply`, this staking contract accounts `totalSupply` as the sum of weighted amounts to ensure correct reward distribution.

**Token Flows**:
- On stake: transfer SUMR → contract, approve and deposit into `WrappedStakingToken`, mint xSUMR 1:1 to receiver
- On unstake: burn xSUMR, withdraw wrapped SUMR; if a penalty applies, send penalty to `treasury()` and remainder to user

**Security Model**:
- **Bucket Caps**: Governor-controlled limits per lockup duration
- **Penalty Enforcement**: Penalties sent to treasury, not burned
- **Wrapped Token Integration**: Uses `WrappedStakingToken` for internal accounting

**Audit Focus Areas**:
- Weighted stake calculation precision
- Bucket cap enforcement
- Penalty calculation accuracy
- Wrapped token integration security

**User/Operator API**:
- Stake: `stakeLockup(amount, lockupPeriod)`; `stakeLockupOnBehalf(receiver, amount, lockupPeriod)`
- Unstake: `unstakeLockup(stakeIndex, amount)`
- Transfer portfolio: `transferStakes(to)` (to must have no stakes, no xSUMR, and no reward markers)
- Admin: `updateLockupBucketCap(bucket, newCap)`, `updatePenaltyEnabled(bool)`, `rescueToken(token, to)`
- Views: `getUserStakesCount(user)`, `getUserStake(user, index)`, `weightedBalanceOf(user)`, bucket getters
- Disabled (reverts): `stake()`, `unstake()`, `exit()`, `stakeOnBehalfOf()`, `unstakeAndWithdrawOnBehalfOf()`

## Integration Points & Dependencies

### Staking Flow
```
User → SummerStaking.stakeLockup() → 
  - Transfers SUMMER tokens
  - Wraps via WrappedStakingToken
  - Mints xSUMR via StakedSummerToken.mint()
  - Updates weighted balances for rewards
```

### Governance Flow
```
xSUMR holders → SummerGovernorV2.propose() → 
  - Cross-chain proposal distribution
  - Timelock execution
  - No voting decay (unlike V1)
  - Propose/vote/execute/cancel only on hub chain
```

### Vesting Integration
```
Vesting Wallets → SummerVestingWalletsEscrow → 
  - Vesting wallet ownership must be transferred to escrow prior to staking
  - xSUMR minting/burning
  - Release tracking during staking
  - Per-factory granular stake/unstake with validation
```

## Critical Security Considerations

1. **Role Management**: StakedSummerToken disables direct role granting - only governor can manage
2. **Staking Module Control**: The staking module has both minting and burning authority
3. **Vesting Wallet Ownership**: Escrow takes temporary ownership - ensure proper return
4. **Weighted Calculations**: `weightedAmount = amount * (1 + 7e-16 * t^2)` and penalties include a fixed 2% floor near expiry; verify precision and edge cases
5. **Bucket Caps**: Governor-controlled limits that could affect staking economics; ShortTerm bucket disabled by default
6. **Cross-chain Governance**: V2 maintains LayerZero integration; enforce hub/satellite constraints
7. **Pause Behavior**: Pausing xSUMR halts mint/burn; consider operational runbooks

## Previous Audit Coverage

**Already Audited**:
- `StakingRewardsManagerBase.sol` - Base reward distribution logic
- `SummerGovernor.sol` - V1 with voting decay

**New Audit Scope**:
- Role management changes in StakedSummerToken
- Removal of voting decay in V2
- Vesting wallet escrow mechanics
- Advanced staking with lockups and penalties

The contracts build upon proven patterns but introduce new complexity around lockup periods, weighted rewards, and vesting wallet management that requires careful review.

---

## Known Issues and Limitations

### Rewards duration immutability in StakingRewardsManagerBase

In `StakingRewardsManagerBase`, calling `notifyRewardAmount(rewardToken, reward, newRewardsDuration)` for an already-configured reward token will revert if `newRewardsDuration` differs from the stored `rewardsDuration` for that token. This is enforced to prevent mid-stream schedule changes:

```405:409:packages/rewards-contracts/src/contracts/StakingRewardsManagerBase.sol
// For existing reward tokens, check if current period is complete
if (_isRewardToken(rewardToken)) {
    if (newRewardsDuration != rewardTokenData.rewardsDuration) {
        revert CannotChangeRewardsDuration();
    }
}
```

- To change a reward token's `rewardsDuration`, wait until the current reward period finishes, then call `setRewardsDuration(rewardToken, newDuration)`. Passing a different duration via `notifyRewardAmount` will fail for existing tokens.
- Operational implication: integrations that rotate reward programs must either reuse the same duration or sequence a `setRewardsDuration` after the prior period ends. Attempting to extend/shorten during an active period is intentionally disallowed.
- There is a mismatch between comment and code

## Appendix: Quick API Reference (for implementers and auditors)

### StakedSummerToken (xSUMR)
- Transfers: disabled (only mint/burn allowed)
- Roles: `MINTER_ROLE`, `BURNER_ROLE`
- Governance: `addStakingModule(address)`, `removeStakingModule(address)`, `pause()`, `unpause()`
- Emergency: `grantMinterRole(address)`, `revokeMinterRole(address)` (direct `grantRole`/`revokeRole` are disabled and revert)
- Mint/Burn: `mint(to, amount)` (minter only), `burn(amount)`, `burnFrom(from, amount)` (owner or burner + allowance)

### SummerStaking
- Stake: `stakeLockup(amount, lockupPeriod)`; `stakeLockupOnBehalf(receiver, amount, lockupPeriod)`
- Unstake: `unstakeLockup(stakeIndex, amount)`
- Transfer: `transferStakes(to)` (strict preconditions)
- Admin: `updateLockupBucketCap(bucket, cap)`, `updatePenaltyEnabled(bool)`, `rescueToken(token, to)`
- Views: stake getters, bucket getters, `weightedBalanceOf(user)`
- Disabled: `stake()`, `unstake()`, `exit()`, `stakeOnBehalfOf()`, `unstakeAndWithdrawOnBehalfOf()`

### SummerVestingWalletsEscrow
- Configure factories (gov): `addVestingFactory(address)`, `removeVestingFactory(address)`
- User flows: `stakeVesting(address[] factories)`, `unstakeVesting(address[] factories)`
- Safety: `rescueWallet(wallet, newOwner)`, `rescueToken(token, to)` (gov)
- Views: `vestingFactories()`, `getVestingFactory(i)`, `userStakedVestingFactories(user)`, `getUserStakedVestingFactory(user, i)`

### SummerGovernorV2
- Hub-only: `propose(...)`, `castVote(proposalId, support)`, `execute(...)`, `cancel(...)`, `sendProposalToTargetChain(...)`
- Satellite: queue via cross-chain receive
- Params: voting delay/period, quorum fraction, proposal threshold validated within `[1,000; 100,000] SUMR`


## Previous audits (with overlapping scope):
- `StakingRewardsManagerBase.sol`,`ProtocolAccessManaged.sol`,`ConfigurationManaged.sol` [REPORT](https://cdn.prod.website-files.com/65d35b01a4034b72499019e8/68c01d6c3692197b1ecda495_ChainSecurity_Summer_fi_Summer_Earn_Protocol_audit.pdf)
- `StakingRewardsManagerBase.sol`, `SummerGovernor.sol` [REPORT](https://github.com/Prototech-Labs/published-work/blob/main/18012025%20Prototech-SummerFi-Report.pdf)


