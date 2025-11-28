# Overview

The WERC7575 smart contract system is the **blockchain settlement layer** within a **multi-tier telecom wholesale voice traffic settlement ecosystem**. It works in conjunction with off-chain platforms (COMMTRADE and WRAPX) and telecom OSS/BSS systems to enable efficient, transparent settlement of inter-carrier voice traffic transactions.

## All trusted roles in the protocol

The roles of the system are as follows:

- Owners
- Validators
- KYC Administrators
- Revenue Administrators
- Investment Managers 

Their privileges are documented in the known issues section of the contest.

# Scope

### Files in scope


| File   | nSLOC |
| ------ | ----- |
|[src/DecimalConstants.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/DecimalConstants.sol)| 5 |
|[src/ERC7575VaultUpgradeable.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/ERC7575VaultUpgradeable.sol)| 737 |
|[src/SafeTokenTransfers.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/SafeTokenTransfers.sol)| 19 |
|[src/ShareTokenUpgradeable.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/ShareTokenUpgradeable.sol)| 243 |
|[src/WERC7575ShareToken.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/WERC7575ShareToken.sol)| 514 |
|[src/WERC7575Vault.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/WERC7575Vault.sol)| 152 |
|**Totals**| **1670** |

*For a machine-readable version, see [scope.txt](https://github.com/code-423n4/2025-11-sukukfi/blob/main/scope.txt)*

### Files out of scope

| File         |
| ------------ |
|[src/ERC20Faucet.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/ERC20Faucet.sol)|
|[src/ERC20Faucet6.sol](https://github.com/code-423n4/2025-11-sukukfi/blob/main/src/ERC20Faucet6.sol)|
|[src/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-11-sukukfi/tree/main/src/interfaces)|
|[test/\*\*.\*\*](https://github.com/code-423n4/2025-11-sukukfi/tree/main/test)|
| Totals: 61 |

*For a machine-readable version, see [out_of_scope.txt](https://github.com/code-423n4/2025-11-sukukfi/blob/main/out_of_scope.txt)*

# Additional context

## Areas of concern (where to focus for bugs)
  1. Batch Settlement Netting (WERC7575ShareToken.batchTransfers) - Validator-controlled, complex netting logic, zero-sum invariant validation, potential for state corruption
  2. Role Access Control - Five distinct roles (Owner, Validator, KYC Admin, Revenue Admin, Investment Manager) with independent permissions; risk of single-point-of-failure key compromise
  3. Reentrancy in Async Flows - External calls in deposit/redeem/investment functions with nonReentrant guards; validate CEI pattern throughout
  4. Dual Allowance Model - Non-standard ERC20 requiring self-allowance + caller allowance; validate both checks are enforced in transfer/transferFrom
  5. Reserved Asset Accounting - Ensure pending/claimable/invested assets are correctly calculated and don't overlap; verify investment layer can't over-allocate
  6. Async State Transitions - Request→Fulfill→Claim flow with cancelations; validate no state-skipping, double-claiming, or permanent blocking
  7. Permit Signature Validation - EIP-712 replay protection, nonce tracking, chain ID inclusion; validate validator signature authenticity
  8. Upgrade Safety - ERC-7201 namespaced storage, gap arrays, no storage reordering; validate upgrade pattern prevents storage collision

## Main invariants

### Settlement Layer

  1. sum(balances) == totalSupply - Token supply conservation
  2. batchTransfers: sum(balance changes) == 0 - Zero-sum settlement
  3. transfer requires self-allowance[user] - Permit enforcement
  4. transferFrom requires both allowances - Dual authorization
  5. Only KYC-verified addresses can receive/hold shares
  6. assetToVault[asset] ↔ vaultToAsset[vault] - One-to-one mapping
  7. Only registered vaults can mint/burn

### Investment Layer

  8. Deposit/Redeem: Pending → Claimable → Claimed (no skipping)
  9. investedAssets + reservedAssets ≤ totalAssets - Reserved protection
  10. convertToShares(convertToAssets(x)) ≈ x - Rounding accuracy

### Global 

  11. No role escalation - access control boundaries enforced
  12. No fund theft - no double-claims, no reentrancy, no bypass



## Publicly known issues

_Anything included in this section is considered a publicly known issue and is therefore ineligible for awards._

# Known Issues & Design Decisions
## **NOTE:** ALL Known Issues and Design Decisions are OUT OF SCOPE

## Purpose

This document lists all **intentional design choices** and **known limitations** in our system, documented **before the audit begins**. Per Code4rena judging criteria, "Known issues from prior audits are generally out-of-scope unless substantively distinct or higher-severity."

**These are NOT bugs. These are documented design decisions made for our institutional use case.**

Wardens: Please do NOT report these as Medium or High findings - they will be marked as Known/Invalid.


## C4 Key Principles Applicable to This Protocol

Per C4 official severity categorization and judging criteria:

### 1. Trustworthy Roles Assumption
**C4 Principle**: "All roles assigned by the system are expected to be trustworthy."

Our system has trusted roles (Owner, Investment Manager, Validator). These roles are intentional, documented, and expected to act in good faith.

### 2. Admin Mistakes Are Invalid
**C4 Guidance**: "Reckless admin mistakes are invalid. Assume calls are previewed."

Reports claiming "admin could accidentally do X" or "owner might mistakenly do Y" are invalid. Admin actions are assumed to be intentional and previewed.

### 3. Centralization = Governance/Centralization Risk Category
**C4 Severity**: "Governance/Centralization risk (including admin privileges)" is explicitly **Low/QA**, NOT Medium.

Any finding about Owner powers, Investment Manager powers, or Validator powers falls under this category.

### 4. Function Incorrect as to Spec
**C4 Severity**: "Function incorrect as to spec" is **Low**, NOT Medium (when assets not at risk).

Our non-standard ERC-20 behavior (permit requirements, dual allowances, KYC) is spec deviation without asset risk.

### 5. Judge Discretion
Judges have final say on validity and severity. This document provides our rationale using C4's own criteria. If a Warden overstates severity, judges may assign lower grades per C4 judging criteria.

---

## Critical: Refund Eligibility

Per our C4 agreement (Exhibit 2):
- **$34,560 USDC refund** if zero valid High AND zero valid Medium findings
- **$0 USDC refund** if ANY valid High OR Medium finding exists

We have extensively documented our design choices below to ensure Wardens understand what is intentional vs. what is a bug.

---

## 1. Centralized Access Control (QA/Low - NOT Medium/High)

### Owner Powers
- Registers/unregisters vaults
- Sets investment manager (propagates to all vaults)
- Upgrades contracts via UUPS
- Pauses/unpauses system
- Configures investment parameters

**Severity: QA/Low** - "Governance/Centralization risk (including admin privileges)" per C4 severity categorization

**Why Intentional**: Built for institutional tokenized assets with regulatory requirements. Clear administrative control required for compliance.

**NOT a Medium**: No "function of protocol impacted" - this IS the intended function. Assets are not "at risk" from admin having admin powers that are documented and expected.

### Investment Manager Powers
- Controls fulfillment timing (no deadlines enforced)
- Decides when to fulfill deposit/redeem requests
- Invests idle assets into external vaults
- Withdraws from investment positions
- Batch fulfillment operations

**Severity: QA/Low** - Centralization/governance risk

**Why Intentional**: Professional fund management model. Capital efficiency requires discretionary timing control.

**NOT a Medium**: This is the intended operation model. No protocol "function impacted" when functioning as designed.

---

## 2. Non-Standard ERC-20 Behavior (QA/Low - NOT Medium/High)

### Transfer Requires Self-Allowance
```solidity
function transfer(address to, uint256 value) public override {
    _spendAllowance(msg.sender, msg.sender, value); // Requires permit
    super.transfer(to, value);
}
```

**Severity: QA/Low** - "Function incorrect as to spec" (deviates from ERC-20 spec)

**Why Intentional**: Regulatory compliance requires validator approval for all transfers.

**NOT a Medium**:
- No assets at risk
- Not a "function of protocol impacted" - this IS the function
- Documented deviation from standard for compliance reasons
- Users knowingly use institutional platform with KYC

### TransferFrom Requires Dual Allowances
```solidity
function transferFrom(address from, address to, uint256 value) public override {
    _spendAllowance(from, from, value);        // Self-allowance
    super.transferFrom(from, to, value);       // Caller allowance
}
```

**Severity: QA/Low** - Spec deviation

**Why Intentional**: Dual-authorization model (platform permission + owner delegation)

**NOT a Medium**: No assets at risk. Intentional security model.

### Approve Blocks Self-Approval
```solidity
function approve(address spender, uint256 value) public override {
    if (msg.sender == spender) revert ERC20InvalidSpender(msg.sender);
    super.approve(spender, value);
}
```

**Severity: QA/Low** - Spec deviation

**Why Intentional**: Forces permit flow through validator for compliance.

**NOT a Medium**: No assets at risk. Intentional enforcement mechanism.

### KYC Requirements
All token recipients must be KYC-verified. Validator controls KYC status.

**Severity: QA/Low** - Centralization/governance risk

**Why Intentional**: Regulatory requirement for institutional tokenized assets.

**NOT a Medium**: This IS the business model. No protocol function impacted.

---

## 3. External Protocol Incompatibilities (INVALID - Out of Scope)

### DEX Incompatibility
- Uniswap, Curve, Balancer pools will fail
- Requires permit signatures DEXs cannot obtain

**Status: INVALID/Out of Scope** - Not designed for DEX integration

**NOT a Medium**: No protocol function impacted. We don't support DEX integration.

### Lending Protocol Incompatibility
- Aave, Compound, Morpho will fail
- Cannot use as collateral

**Status: INVALID/Out of Scope** - Not designed for lending

**NOT a Medium**: No protocol function impacted. We don't support lending.

### Wallet Incompatibility
Standard wallets will fail when users attempt transfers because they don't set up self-allowance first.

**Failure Mechanism**:
```solidity
// What standard wallets do:
token.transfer(recipient, amount);  // ❌ FAILS - no self-allowance

// What's required:
1. validator.permit(user, user, amount, signature);  // Set self-allowance via validator
2. token.transfer(recipient, amount);                // ✓ Now succeeds
```

**Why self-allowance is required**:
- Self-allowance is the mechanism used by the validator to ensure funds committed for a settlement are not moved before settlement occurs
- Validator signature required to grant self-allowance prevents users from moving funds that are already allocated to pending settlements
- This is a compliance control, not a bug

**Which wallets fail**:
- MetaMask "Send" button - calls `transfer()` directly without self-allowance setup
- Trust Wallet - calls `transfer()` directly without self-allowance setup
- Ledger Live - standard ERC-20 UI doesn't support permit flow
- Any wallet using standard ERC-20 transfer UI

**Why they fail**: Standard ERC-20 wallets don't know about the permit requirement and call `transfer()` directly, which reverts due to missing self-allowance.

**Status: INVALID/Out of Scope** - Not designed for standard wallet integration. Requires custom UI with permit flow.

**NOT a Medium**:
- Intended for institutional use with custom interface
- No "function of protocol impacted" - wallets are not part of our protocol
- Not a compatibility we claim to support

---

## 4. Async Operations & Timing (QA/Low - NOT Medium)

### No Fulfillment Deadlines
Investment Manager can delay fulfillments indefinitely. No SLA enforcement.

**Severity: QA/Low** - Governance/centralization

**Why Intentional**: ERC-7540 async design for professional fund management. Capital efficiency requires flexible timing.

**NOT a Medium**:
- Assets not "at risk" - they're held securely in pending state
- This IS the protocol function (async by design)
- Users accept this model for institutional investment

### Reserved Assets Not Invested
Reserved assets (pending/claimable) sit idle, earning no yield.

**Severity: QA/Low** or INVALID - Design choice

**Why Intentional**:
- Safety buffer ensuring liquidity for pending operations
- Cannot invest funds that are pending fulfillment or claimable (they're not yet/already owned by users)
- We only commit (promise yields/shares) on capital that will be invested
- This PROTECTS APY - if we invested reserves and committed on all capital, it would dilute returns

**NOT a Medium**:
- No assets at risk - intentional safety mechanism
- This is NOT "loss of unmatured yield" - reserved assets aren't earning yield because users don't own them yet (pending) or are exiting (claimable)
- Per C4: unmatured yield loss capped at Medium, but this isn't yield loss - it's correct reserve management
- APY is actually HIGHER because we only commit on invested capital, not on uninvested reserves
- If we wrongly invested reserves or committed on them, it would LOWER APY by diluting invested capital with idle reserves

### Request Cancellation Allowed
Users (or their approved operators) can cancel **pending** deposit/redeem requests and reclaim their assets/shares.

**Functions**:
- `cancelDepositRequest(address controller)` - Returns assets to user
- `cancelRedeemRequest(address controller)` - Returns shares to user

**Severity: QA/Low** or INVALID - Intentional user protection feature

**Why Intentional**:
- User protection - allows exit if Investment Manager delays too long
- **Custom extension beyond ERC-7540** (ERC-7540 deliberately excludes cancellation, deferring to future EIP)
- Design choice to provide user escape mechanism
- Only controller or their approved operator can cancel (access control working as intended)
- Only PENDING requests can be cancelled (not CLAIMABLE/fulfilled ones)

**NOT a Medium**:
- This IS the intended function - user has control over their pending requests
- No assets at risk - assets/shares returned to rightful owner
- Access control is intentional (controller or operator only)
- Not a "missing access control" - it's controlled access working correctly

**What WOULD Be a Bug** (High/Medium severity):
- ✅ Anyone can cancel other users' requests (broken access control)
- ✅ Cancellation doesn't return assets/shares (loss of funds)
- ✅ Can cancel CLAIMABLE requests (allowing double-claim)
- ✅ Reentrancy allows theft during cancellation

**Note**: Future versions may add additional cancellation features or restrictions, but current implementation is intentional.

---

## 5. Upgrade Capabilities (QA/Low - NOT Medium)

### Unilateral Upgrades
Owner can upgrade contracts without timelock, governance, or user exit window.

**Severity: QA/Low** - Centralization/governance risk

**Why Intentional**: Institutional model with trusted admin. Rapid bug fixes and compliance updates required.

**NOT a Medium**: Admin having admin powers is not a "Medium" risk per C4 categorization.

**Note**: Storage corruption or improper upgrade patterns ARE in scope as High findings.

---

## 6. Decimal Normalization Effects (QA/Low - NOT Medium)

### All Shares 18 Decimals
Regardless of underlying asset decimals (USDC = 6, DAI = 18), all shares are 18 decimals.

**Severity: QA/Low** or INVALID - Design choice for multi-asset system

**Why Intentional**:
- Simplifies multi-asset accounting - all vaults share the same decimal precision
- Enables normalized asset aggregation across different asset decimals
- Common pattern for multi-asset vault systems
- Prevents decimal-related accounting complexity when aggregating USDC (6 decimals) + DAI (18 decimals) + other assets

**NOT a Medium**:
- No assets at risk
- Not required by ERC-7575, but intentional design choice
- Makes multi-asset calculations simpler and less error-prone

### Rounding ≤1 Wei
Normal integer division causes up to 1 wei rounding errors.

**Severity: QA/Low or INVALID** - "Rounding errors and marginal fee variations" per C4

**Why Expected**: ERC-4626 standard allows this. Mathematical reality of integer division.

**NOT a Medium**: Not exploitable for profit. Within acceptable tolerance.

**Note**: Rounding >1 unit or exploitable rounding IS in scope as Medium/High.

---

## 7. Architecture Limitations (QA/Low - NOT Medium)

### Single Vault Per Asset
Cannot register multiple vaults for same asset (e.g., only one USDC vault).

**Severity: QA/Low** - Design limitation

**Why Intentional**: Simplified design aligned with ERC-7575 standard. Clear asset ownership.

**NOT a Medium**: No assets at risk. No function impacted. Intentional simplification.

### Batch Size Limits
`MAX_BATCH_SIZE = 100` for batch transfers (state-changing operations).

**Severity: QA/Low** - Design choice

**Why Intentional**:
- Gas limit protection - prevents exceeding block gas limits
- Calculated conservatively: 30M gas limit / 25k per transfer ≈ 1000, but set to 100 for safety margin
- With complex transfers (permit checks, KYC validation, dual allowances, netting logic), each transfer costs more than simple ERC-20
- 100 transfers provides sufficient batch size while leaving headroom for complex operations
- View functions (off-chain helpers) use higher limit (1000) since they don't consume gas

**NOT a Medium**: No assets at risk. Prevents DOS from gas exhaustion.

### Self-Transfers Skipped
Batch operations skip `debtor == creditor` transfers.

**Severity: QA/Low** - Gas optimization

**Why Intentional**: No-op transfers waste gas. Mathematically equivalent to skip.

**NOT a Medium**: No assets at risk. Correct accounting maintained.

### Batch Netting - "Overdraft" Allowed Within Batch
Users can have transfers in a batch that individually exceed their balance, as long as the **final net result** is valid.

**Example**:
```solidity
// User A has balance: 100
// Batch contains:
// 1. A → B: 80
// 2. A → C: 60  (would fail if checked individually - A only has 20 left)
// 3. B → A: 50
// 4. C → A: 40

// Net effect:
// A sends: 80 + 60 = 140
// A receives: 50 + 40 = 90
// Net: 140 - 90 = 50
// Final balance: 100 - 50 = 50 ✓ VALID
```

**Severity: QA/Low** - Intentional settlement/netting logic

**Why Intentional**:
- Settlement systems process NET effects, not sequential individual transfers
- Reduces actual fund movements (gas efficiency)
- Mathematically correct - final balances are always valid
- Common pattern in professional settlement platforms (telecom, finance)

**NOT a Medium**:
- No assets at risk - final balances are always validated
- This IS the function (netting), not impacted function
- Correct accounting maintained - sum of all balances unchanged
- No actual "overdraft" - just appears that way if viewing individual transfers in isolation

**What WOULD Be a Bug** (High severity):
- ✅ Final balance incorrect after batch processing
- ✅ User ends with negative balance
- ✅ Total supply changes incorrectly
- ✅ Batch allows actual theft by bypassing final balance check

Our batch netting is intentional, mathematically sound, and maintains correct final state.

---

## 7a. Batch Transfer Operations & rBalance Management (QA/Low - Intentional Design)

### Two Different Batch Transfer Functions

**batchTransfers() - Standard Settlement Transfers**
```solidity
function batchTransfers(address[] calldata debtors, address[] calldata creditors,
    uint256[] calldata amounts) external onlyValidator returns (bool)
```
- Updates ONLY `_balances` (user balances)
- Does NOT modify `_rBalances` (investment tracking)
- Used for standard inter-carrier settlements
- Simpler, cheaper operation (no rBalance logic)

**rBatchTransfers() - Investment Wallet Transfers with rBalance Updates**
```solidity
function rBatchTransfers(address[] calldata debtors, address[] calldata creditors,
    uint256[] calldata amounts, uint256 debtorsRBalanceFlags, uint256 creditorsRBalanceFlags)
    external onlyValidator returns (bool)
```
- Updates BOTH `_balances` AND `_rBalances` selectively
- SELECTIVELY updates rBalance based on pre-computed flags
- Used for investment operations that need to track invested capital
- rBalance updates gated by boolean flags: debtorsRBalanceFlags and creditorsRBalanceFlags
- Pre-computed via `computeRBalanceFlags()` helper for integrity validation

### Design Intent

**batchTransfers Purpose**:
- Fast settlement between carriers
- No investment tracking needed
- Cheaper gas (only balance updates)

**rBatchTransfers Purpose**:
- Investment wallet operations requiring rBalance tracking
- Separate control of balance vs. rBalance updates
- Flexibility for complex investment scenarios
- Validator pre-computes flags off-chain for validation before execution

### rBalance Silent Truncation (Informational Tracking)

When rBatchTransfers reduces an account's rBalance due to credit operations:
```solidity
if (rbalance < amount) {
    _rBalances[account.owner] = 0;  // Silent truncation to 0
} else {
    _rBalances[account.owner] -= amount;
}
```

**Why Intentional**:
- rBalance represents INVESTED capital in external vaults (informational)
- When creditor receives funds, their invested position is partially liquidated
- If rBalance insufficient, capital returned exceeds tracked investment
- Silent truncation prevents revert, allowing transfer to complete
- User's actual `_balances` are always correct (they receive the full credit amount)

**What This Means**:
- User funds are never lost - actual balances always correct
- rBalance may not reflect exact invested amount in edge cases
- rBalance is informational for revenue/yield tracking, not critical for transfers
- Zero-sum invariant maintained: sum of all balance changes always = 0

**Why Not a Bug**:
- By design for institutional investment tracking
- User balances unaffected
- Actual transfers always complete successfully
- Investment tracking is secondary to fund safety (which is guaranteed)

### rBalance Adjustment Controls (Revenue Admin)

**adjustrBalance() - Yield/Loss Recording**
```solidity
function adjustrBalance(address account, uint256 ts, uint256 amounti, uint256 amountr)
    external onlyRevenueAdmin
```
- **amounti** = original invested amount
- **amountr** = returned amount (with profit/loss)
- Increases rBalance by (amountr - amounti) if profitable
- Decreases rBalance by (amounti - amountr) if loss
- Max 2x return cap: `amountr ≤ amounti * 2`
- Each (account, ts) pair can only be adjusted once
- Stored for audit trail in _rBalanceAdjustments[account][ts]

**cancelrBalanceAdjustment() - Reversal**
- Reverses previous adjustrBalance by opposite adjustment
- Deletes adjustment record

**Why Intentional**:
- Institutional investment yield tracking
- Separate from user balance accounting
- Revenue admin trusted role (centralization)
- rBalance adjustments don't affect user balances

**Severity: QA/Low** - All batch operation and rBalance behaviors are intentional design for institutional investment management.

---

## 8. DOS & Availability Scenarios (Context-Dependent)

### Intentional Availability Controls (QA/Low)

**These DOS/availability scenarios are INTENTIONAL and should be QA/Low**:

1. **Batch size limits (MAX_BATCH_SIZE = 1000)**: Intentional gas protection. Not "availability impacted" - it's availability PROTECTED.

2. **Investment Manager can delay fulfillments**: Intentional async design (see Section 4). Centralization/governance risk = QA/Low.

3. **KYC requirement blocks non-KYC'd recipients**: Intentional compliance control. Centralization/governance = QA/Low.

4. **Pause functionality**: Intentional emergency control. Admin privileges = QA/Low.

**Severity: QA/Low** for all above - These are intentional controls, not protocol dysfunction.

### What WOULD Be Medium DOS

**These would be valid Medium findings**:

- ✅ Griefing attack causing permanent lock of user funds (external requirements but realistic path)
- ✅ DOS requiring non-trivial cost that blocks core functionality (e.g., preventing all withdrawals)
- ✅ Unintended availability impact from bug (not intentional control)

**Key Distinction**: C4 Medium is "availability COULD BE IMPACTED" (broken/degraded from intended state), NOT "availability IS CONTROLLED by design."

Our intentional controls are the INTENDED state, not impacted state.

---

## 9. Privilege Escalation vs. Intentional Centralization

### Intentional Centralization (QA/Low - This Protocol)

Our protocol has **intentional** trusted roles with **documented** powers:
- Owner has admin privileges (upgrades, vault management, pause)
- Investment Manager has fulfillment control
- Validator has KYC/permit control

**Severity: QA/Low** - "All roles assigned by the system are expected to be trustworthy" per C4.

### What WOULD Be Medium/High Privilege Escalation

**These would be valid Medium/High findings**:

- ✅ **Unintended** privilege escalation (e.g., regular user can gain Owner role through bug)
- ✅ **Unauthorized** role assignment (e.g., bypass of access control allowing non-Owner to upgrade)
- ✅ **Unexpected** permission (e.g., Investment Manager can do something beyond documented scope)

**Key Distinction**:
- Intentional centralization with trustworthy roles = QA/Low
- Unintended privilege escalation allowing unauthorized actions = Medium/High

Our roles are intentional, documented before audit, and within expected scope.

---

## 10. Gas & Code Quality (INVALID - Not Security Issues)

### Gas Optimizations
Suggestions for gas savings <10% without security impact.

**Status: INVALID** - QA report at best, but not security issues

### Code Style
Alternative implementations, structure suggestions, naming conventions.

**Status: INVALID** - Not security issues

### Documentation
Typos, NatSpec formatting, comment improvements.

**Status: INVALID** - "Issues with comments" are Low risk per C4

**Note**: Misleading documentation causing integration errors IS in scope.

---

## 11. Understanding "Function of Protocol Impacted" (Medium Severity)

C4 Medium severity definition includes: "function of the protocol...could be impacted."

### What This DOES Mean (Valid Medium)

**"Impacted" = Protocol deviates from intended design due to bug**:
- ✅ Withdrawals should work but are blocked due to accounting error
- ✅ Shares should convert correctly but conversion fails due to overflow
- ✅ Funds should be safe but can be stolen via reentrancy
- ✅ Operations should complete but are DOS'd by griefing attack

### What This DOES NOT Mean (Invalid/QA-Low)

**"Impacted" ≠ Protocol works exactly as designed**:
- ❌ "Transfers require permit" - This IS the function, not impacted function
- ❌ "Fulfillments can be delayed" - This IS the async design, not impacted design
- ❌ "Owner can upgrade" - This IS the admin model, not impacted model
- ❌ "Reserved assets not invested" - This IS the safety buffer, not impacted safety

**Key Principle**: If the protocol is functioning EXACTLY as we designed and documented it, the function is NOT impacted - it's operating correctly.

Medium severity requires the function to be BROKEN, DEGRADED, or BYPASSED from its intended design, not just "has centralized controls" or "doesn't match Uniswap's design."

---

## 12. Operational & Economic (INVALID - Out of Scope)

### Operational Security
HSM recommendations, multi-sig advice, key management best practices.

**Status: INVALID** - Not smart contract vulnerabilities

### Economic Advice
Investment strategy, yield optimization, market risk, APY calculations.

**Status: INVALID** - Business decisions, not code vulnerabilities

### Legal/Compliance
Regulatory opinions, securities law, jurisdiction, licensing.

**Status: INVALID** - Legal matters, not technical issues

---

## What IS In Scope (Medium/High Eligible)

### High Severity (Assets at direct risk)
✅ Loss of funds through vulnerability
✅ Unauthorized minting/burning
✅ Asset theft vectors
✅ Access control bypass (unintended)
✅ Storage corruption in upgrades

### Medium Severity (Function impacted or hypothetical asset risk)
✅ Reentrancy attacks affecting state
✅ Signature replay attacks
✅ Accounting errors (e.g., reserved asset calculation mixing units)
✅ DOS requiring non-trivial cost
✅ Standards violations breaking functionality
✅ Significant Precision loss (exploitable)

---

## Example: Invalid vs. Valid Reports

### ❌ INVALID - Would Be Marked Known/QA

**"Investment Manager can delay fulfillments indefinitely"**
→ Known Issue #4. Intentional async design. QA/Low at best.

**"Owner can upgrade without timelock"**
→ Known Issue #5. Intentional admin control. QA/Low.

**"Transfer requires permit signature"**
→ Known Issue #2. Intentional regulatory compliance. QA/Low.

**"System is centralized"**
→ Known Issues #1, 2, 4, 5. Intentional design. QA/Low.

**"Token doesn't work with Uniswap"**
→ Known Issue #3. Out of scope. Not designed for DEXs.

**"Reserved assets earn no yield"**
→ Known Issue #4. Intentional safety buffer. QA/Low.

**"User can transfer more than balance in batch operations"**
→ Known Issue #7. Intentional netting logic. QA/Low.

**"Users can cancel pending requests and reclaim assets"**
→ Known Issue #4. Intentional user protection feature. QA/Low or Invalid.

**"No access control on request cancellation"**
→ INCORRECT - there IS access control (controller or operator only). Known Issue #4.

### ✅ VALID - Would Be Medium/High

**"Reserved asset calculation adds shares to assets without conversion"**
→ NEW bug. Accounting error. Can cause over-investment. **HIGH**

**"Reentrancy in fulfillDeposit() allows double-minting"**
→ NEW bug. Asset theft vector. **HIGH**

**"Signature replay across chains allows unauthorized transfers"**
→ NEW bug. Access control issue. **HIGH/MEDIUM**

**"Rounding in convertToAssets() can be exploited for 10% profit"**
→ NEW bug. Exploitable precision loss. **MEDIUM**

**"Storage collision between ShareTokenStorage and ERC20Upgradeable"**
→ NEW bug. Storage corruption risk. **HIGH**

---

## Summary for Wardens

**Before reporting an issue, check:**

1. Is it listed in this document? → **Known/Invalid**
2. Is it about centralization/admin powers? → **QA/Low max** (See: C4 Key Principles #1, #3)
3. Is it about non-standard ERC-20? → **QA/Low max** (See: C4 Key Principles #4)
4. Is it about external incompatibility? → **Invalid** (See: Section 3)
5. Is it about timing/delays? → **QA/Low max** (See: Section 4)
6. Is it about intentional design choices? → **QA/Low max** (See: Section 11)
7. Is it about admin mistakes? → **Invalid** (See: C4 Key Principles #2)
8. Is it intentional DOS/availability control? → **QA/Low max** (See: Section 8)
9. Can it cause direct loss of funds? → **Submit as High** (See: "What IS In Scope")
10. Can it break core functionality (unintended)? → **Submit as Medium** (See: "What IS In Scope")

**Remember**:
- We need ZERO Medium and ZERO High findings for refund eligibility
- Wardens who overstate severity may receive lower grades per C4 judging criteria
- Please help us by correctly categorizing findings according to C4 severity criteria

**Quick Severity Check**:
- Can users lose funds directly due to bug? → High
- Does bug break intended functionality? → Medium
- Is it working exactly as designed with trusted roles? → QA/Low
- Is it about compatibility we don't claim to support? → Invalid

---

## Comprehensive Severity Mapping Table

For sponsor review reference - map common claims to proper C4 categorization:

| Warden Claim | Incorrect Severity | Correct Severity | Reference Section | C4 Justification |
|--------------|-------------------|------------------|-------------------|------------------|
| "Owner can upgrade without timelock" | Medium/High | QA/Low | Section 5 | Centralization/governance risk |
| "Investment Manager can delay fulfillments" | Medium | QA/Low | Sections 1, 4 | Centralization/governance risk |
| "Transfers require permit signature" | Medium | QA/Low | Section 2 | Function incorrect as to spec (no asset risk) |
| "Token doesn't work with Uniswap" | Medium/Low | Invalid | Section 3 | Not claimed functionality |
| "Reserved assets earn no yield" | Medium | QA/Low/Invalid | Section 4 | Intentional design, not yield loss |
| "Batch size limited to 100" | Medium/Low | QA/Low | Section 7 | Intentional gas limit protection |
| "Pause blocks all operations" | Medium | QA/Low | Sections 1, 8 | Admin privileges |
| "KYC requirement blocks users" | Medium/Low | QA/Low | Sections 2, 8 | Centralization/governance risk |
| "All shares are 18 decimals" | Low | Invalid/QA-Low | Section 6 | Design choice for multi-asset system |
| "Rounding errors ≤1 wei" | Medium/Low | Invalid | Section 6 | Acceptable per ERC-4626 |
| "Admin could make mistake" | Medium/Low | Invalid | C4 Principles #2 | "Reckless admin mistakes invalid" |
| "System is centralized" | Medium | QA/Low | C4 Principles #1, #3 | "Trustworthy roles" + governance risk category |
| "User can transfer more than balance in batch" | Medium/High | QA/Low | Section 7 | Intentional netting logic, correct final state |
| "Batch allows overdraft" | Medium | QA/Low | Section 7 | Settlement netting, not actual overdraft |
| "Users can cancel pending requests" | Medium/Low | Invalid/QA-Low | Section 4 | Custom extension for user protection (not in ERC-7540) |
| "Missing access control on cancel functions" | Medium/High | Invalid | Section 4 | Access control exists (controller or operator only) |
| **"Reserved assets calculated wrong"** | **N/A** | **High** | **What IS In Scope** | **Actual accounting bug** |
| **"Reentrancy in fulfill"** | **N/A** | **High** | **What IS In Scope** | **Actual vulnerability** |
| **"Storage collision in upgrade"** | **N/A** | **High** | **What IS In Scope** | **Actual upgrade bug** |

**How to use this table during sponsor review**:
1. Find Warden's claim in left column
2. Check if they used incorrect severity
3. Reference our documentation section
4. Cite C4 justification in response
5. Request Judge downgrade to correct severity

---

## References

- **C4 Severity Categorization**: https://docs.code4rena.com/competitions/severity-categorization
- **C4 Judging Criteria**: https://docs.code4rena.com/competitions/judging-criteria
- **C4 Sponsor Terms**: https://code4rena.com/sponsor-terms

---

**Contact**: [TO BE INSERTED]

**Last Updated**: 2025-01-11
**C4 Audit Period**: November 14-24, 2025
