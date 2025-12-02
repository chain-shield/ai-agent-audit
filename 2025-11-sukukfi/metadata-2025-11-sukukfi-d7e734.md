
## PROTOCOL OVERVIEW:

## WERC7575 Protocol – Detailed Technical Summary

### 1. What the Protocol Is
WERC7575 is a **multi-asset, share-based vault system** tailored for regulated, B2B settlement use-cases (e.g. telecom voice-traffic clearing).  Each supported asset (USDC, USDT, DAI …) is handled by its own Vault that speaks ERC-4626 for accounting, ERC-7540 for async request/fulfil/claim flows and ERC-7575 for multi-asset co-ordination.  All vaults mint/burn the same 18-decimal share token, allowing a single fungible representation of value across every asset.

The system is split into two layers:
* **Settlement layer (non-upgradeable):** `WERC7575Vault` + `WERC7575ShareToken` execute real-time settlements between KYC’d counterparties.  Deposits are permissionless (post-KYC); withdrawals need a validator-signed EIP-2612 permit; batch netting dramatically reduces on-chain transfers.
* **Investment layer (upgradeable):** `ERC7575VaultUpgradeable` + `ShareTokenUpgradeable` collect investor capital asynchronously, route idle funds into the settlement layer, and distribute yield.  This layer is upgradeable through UUPS so that strategies and regulatory rules can evolve without touching the settlement contracts.

### 2. How It Works
1. **Decimal normalisation** – every Vault stores an `offset = 10^(18-assetDecimals)` so 6-decimal USDC maps 1:1 into 18-decimal shares (`assets * offset = shares`).  Users therefore always interact with 18-decimal share amounts irrespective of the original token.
2. **Async requests (ERC-7540):**
   • `requestDeposit()` / `requestRedeem()` queues an intention; funds or shares are escrowed inside the vault.
   • An off-chain Investment Manager later calls `fulfillDeposit` or `fulfillRedeem`, performing the actual mint/burn and unlocking a **claimable** balance.
   • The user finally calls `deposit` / `redeem` (or `mint` / `withdraw`) to collect the unlocked assets or shares.
3. **Operator model (ERC-7540):** owners can approve dedicated operators once rather than per allowance; this is stored centrally in `ShareTokenUpgradeable` so that a single approval covers all vaults.
4. **Batch settlement netting:** the validator uses `batchTransfers()` (or `rBatchTransfers()` when touches reserved balances) on `WERC7575ShareToken`.  A linear‐time algorithm nets N transfers down to one debit/credit per party, giving 60-80 % gas savings and atomic, zero-sum execution.
5. **Reserved-balance accounting:** `_rBalances` on the ShareToken track capital that has been invested out of the vault.  An on-chain revenue admin periodically calls `adjustrBalance()` to realise P/L without moving tokens: decrease `_rBalances`, increase normal balances, and therefore push profit to holders pro-rata.
6. **Upgrade & storage safety:** the upgradeable contracts use UUPS with ERC-7201 namespaced storage + 50-slot __gap arrays, guaranteeing collision-free future additions.  Only the proxy owner can upgrade and the settlement layer is deliberately non-upgradeable for regulatory certainty.

### 3. Key Security / Compliance Mechanics
* **KYC gating** – every recipient must be pre-verified (`isKycVerified`); setters limited to a dedicated KYC admin.
* **Withdrawal control** – self-allowance is impossible via `approve()`; it must be set by an EIP-2612 permit signed by the validator, giving the operator an on-chain veto on exits during disputes or AML checks.
* **Reentrancy protection** – OpenZeppelin `ReentrancyGuard` on every external state-changing path that touches balances or external calls.
* **Input-sanity checks** – exhaustive array length / bound checks on batching, MAX_RETURN_MULTIPLIER on rBalance adjustments, minimum deposit limits, duplicate-vault guards, etc.
* **Namespaced storage** – prevents accidental corruption across upgrades and inherited parents.

### 4. Typical Flows
1. **Carrier settlement**
   a. Carrier funds USDC → `WERC7575Vault` (permissionless).
   b. Validator batches settlements; Carrier’s share balance moves, offsets another carrier’s debt.
   c. Carrier asks to withdraw → validator signs permit → carrier calls `transfer()` which internally spends self-allowance.
2. **Investor yield**
   a. Investor `requestDeposit(USDC)` into `ERC7575VaultUpgradeable`.
   b. Manager `fulfillDeposit` when new deal is ready ➜ share tokens minted, claimable.
   c. Manager `investAssets()` deposits idle USDC into settlement vault, receiving WUSD; `_rBalances` updated.
   d. Revenues accrue; admin calls `adjustrBalance()` reflecting profit.
   e. Investor later `requestRedeem(shares)`; flow in reverse, finally receiving USDC + yield.

### 5. Contract-Level Responsibilities
* `DecimalConstants` – compile-time 18-dec share & ≥6-dec asset guarantees.
* `SafeTokenTransfers` – wrapper that reverts if any transfer is fee-on-transfer or rebase.
* `WERC7575ShareToken` – non-upgradeable, holds user balances, KYC logic, validator controlled permit, batch settlement, reserved balance tracking.
* `WERC7575Vault` – per-asset 1:1 vault, synchronous ERC-4626 operations, immutable code.
* `ShareTokenUpgradeable` – upgradeable coordination layer for the async vault fleet, operator registry, investment wiring.
* `ERC7575VaultUpgradeable` – upgradeable async vault with full request-fulfil-claim state machine plus investment hooks.

### 6. Why It Is Designed This Way
The protocol targets **commercial, high-value settlements** where:
• Real-time reliability > immutability for the investment layer ⇒ UUPS
• Withdrawal must not jeopardise counterparties ⇒ validator-gated self-allowance
• Regulatory KYC/AML is mandatory ⇒ enforced at transfer time
• On-chain gas costs must be minimised ⇒ batch netting
• Multiple fiat-pegged tokens coexist ⇒ decimal normalisation + single share token

### 7. Integration Checklist
1. Ensure all interacting addresses are KYC-verified.
2. Implement EIP-2612 permit flow (validator signature) before any withdrawal or `transfer()`.
3. Handle async three-step lifecycle when using `ERC7575VaultUpgradeable`.
4. Do **NOT** assume standard ERC-20 behaviour (self-approve disallowed, transfer requires self-allowance).
5. For any upgrade monitoring, watch `Upgraded` events from the proxy addresses.

---
This architecture yields a compliant, gas-efficient, upgrade-friendly settlement & investment stack where asset custody, share accounting and role-based controls are strictly separated yet cohesively orchestrated.


## Main List of Files in Project

src/DecimalConstants.sol
src/ERC7575VaultUpgradeable.sol
src/SafeTokenTransfers.sol
src/ShareTokenUpgradeable.sol
src/WERC7575ShareToken.sol
src/WERC7575Vault.sol


 ## DOCUMENTATION: 

 ### suku-docs.md

# WERC7575 Technical Architecture Documentation

## Table of Contents

1. [System Overview](#system-overview)
2. [Contract Architecture](#contract-architecture)
3. [Data Flow Diagrams](#data-flow-diagrams)
4. [Key Algorithms](#key-algorithms)
5. [Storage Layout](#storage-layout)
6. [Security Mechanisms](#security-mechanisms)
7. [Integration Guide](#integration-guide)

---

## System Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        WERC7575 SYSTEM                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐           ┌──────────────┐                    │
│  │  ShareToken  │◄─────────►│    Vault     │                    │
│  │ (WERC/Share  │           │ (WERC/Async) │                    │
│  │  Upgradeable)│           │ Upgradeable) │                    │
│  └──────────────┘           └──────────────┘                    │
│         │                           │                           │
│         │                           │                           │
│         │                           ▼                           │
│         │                  ┌──────────────┐                     │
│         │                  │  Investment  │                     │
│         └─────────────────►│    Vault     │                     │
│                            │  (External)  │                     │
│                            └──────────────┘                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

USER FLOW:
1. User deposits USDC → Vault
2. Vault mints shares → ShareToken
3. ShareToken receives shares
4. Investment Manager invests idle USDC → External Vault
5. External Vault mints WUSD shares → ShareToken
6. User earns yield on invested portion
```

### Component Relationships

```
ShareTokenUpgradeable (IUSD)
├── Manages: Asset → Vault registry
├── Coordinates: Investment Manager across all vaults
├── Holds: Shares from Investment Vaults
└── Provides: Unified 18-decimal share representation

ERC7575VaultUpgradeable (per asset: USDC, USDT, DAI)
├── Manages: Asset deposits/withdrawals
├── Implements: ERC-7540 async operations
├── Invests: Idle assets → Investment Vault
└── Mints/Burns: Shares via ShareTokenUpgradeable

Investment Vault (WERC7575Vault - External)
├── Accepts: Asset deposits from Vault
├── Mints: WUSD shares to ShareTokenUpgradeable
└── Generates: Yield for depositors
```

---

## Contract Architecture

### 1. WERC7575ShareToken (Non-Upgradeable)

**Inheritance:**
```solidity
WERC7575ShareToken is
    ERC20,           // Standard token functions
    IERC20Permit,    // Signature-based approvals
    EIP712,          // Structured data hashing
    Nonces,          // Replay protection
    ReentrancyGuard, // Reentrancy protection
    ERC165,          // Interface detection
    Pausable,        // Emergency pause
    IERC7575Errors   // Shared error interface
```

**Key State Variables:**
```solidity
// Line 126-132
mapping(address => bool) public isKycVerified;
mapping(address => uint256) private _balances;     // Standard balance
mapping(address => uint256) private _rBalances;   // Reserved/invested balance
mapping(address => mapping(uint256 => uint256[2])) private _rBalanceAdjustments;
EnumerableMap.AddressToAddressMap private _assetToVault;
mapping(address => address) private _vaultToAsset;
address private _validator;
```

**Architecture Decision: Why Split Balances?**

```solidity
// Standard balance: Available for withdrawal
_balances[user] = 1000 ether

// Reserved balance: Invested in external vaults (not yet returned)
_rBalances[user] = 500 ether

// Total balance: Both combined
totalBalance(user) = _balances[user] + _rBalances[user] = 1500 ether
```

**Purpose:**
- Track which assets are liquid vs. invested
- Enable settlement without disturbing investments
- Support yield distribution via rBalance adjustments
- Maintain ERC-20 compatibility for `balanceOf()`

---

### 2. ShareTokenUpgradeable (UUPS Upgradeable)

**Inheritance:**
```solidity
ShareTokenUpgradeable is
    ERC20Upgradeable,
    IERC20Permit,
    EIP712Upgradeable,
    NoncesUpgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable,
    ERC165,
    IERC7575MultiAsset,
    IERC7575Errors
```

**Storage Pattern (ERC-7201):**
```solidity
// Line 84-111
bytes32 private constant SHARE_TOKEN_STORAGE_SLOT =
    keccak256("erc7575.sharetoken.storage");

struct ShareTokenStorage {
    // Asset ↔ Vault mappings
    EnumerableMap.AddressToAddressMap assetToVault;
    mapping(address vault => address asset) vaultToAsset;

    // Operator approvals (ERC-7540)
    mapping(address controller => mapping(address operator => bool)) operators;

    // Centralized management
    address investmentShareToken;  // Target for yield generation
    address investmentManager;     // Controls fulfillment/investment
}

function _getShareTokenStorage() private pure returns (ShareTokenStorage storage $) {
    assembly {
        $.slot := SHARE_TOKEN_STORAGE_SLOT
    }
}
```

**Why ERC-7201 Namespaced Storage?**

Problem:
```solidity
// ❌ Traditional storage (collision risk)
contract V1 {
    uint256 value1;  // Slot 0
    uint256 value2;  // Slot 1
}

contract V2 is V1 {
    uint256 value3;  // Slot 2 - but what if V1 was upgraded and added value3?
}
```

Solution:
```solidity
// ✅ Namespaced storage (collision-free)
contract V1 {
    bytes32 constant SLOT = keccak256("myapp.storage.v1");
    struct Storage { uint256 value1; uint256 value2; }

    function _getStorage() private pure returns (Storage storage $) {
        assembly { $.slot := SLOT }
    }
}

contract V2 is V1 {
    bytes32 constant SLOT = keccak256("myapp.storage.v2");
    struct Storage { uint256 value3; }
    // No collision possible!
}
```

---

### 3. WERC7575Vault (Non-Upgradeable)

**Inheritance:**
```solidity
WERC7575Vault is
    Ownable,
    DecimalConstants,
    Pausable,
    SafeTokenTransfers,
    IERC7575Errors
```

**Key Mechanism: Decimal Normalization:**

```solidity
// Constructor (Line 82-100)
constructor(address asset_, address shareToken_) {
    _decimals = IERC20Metadata(asset_).decimals();  // e.g., 6 for USDC
    _offset = 10 ** (18 - _decimals);                // 10^12 for USDC
    _shareToken = shareToken_;
    _asset = asset_;
}

// Conversion: Assets → Shares (Line 251-253)
function convertToShares(uint256 assets) public view returns (uint256) {
    return assets * _offset;  // 1,000,000 USDC → 1e18 shares
}

// Conversion: Shares → Assets (Line 260-262)
function convertToAssets(uint256 shares) public view returns (uint256) {
    return shares / _offset;  // 1e18 shares → 1,000,000 USDC
}
```

**Example:**
```
USDC (6 decimals)
─────────────────
Deposit:  1,000,000 USDC (1e6)
Offset:   10^12
Shares:   1,000,000 * 10^12 = 1,000,000,000,000,000,000 (1e18)

Withdraw: 1,000,000,000,000,000,000 shares (1e18)
Offset:   10^12
Assets:   1e18 / 10^12 = 1,000,000 USDC (1e6)
```

**Why 18 Decimals for Shares?**
- ERC-7575 standard requirement
- DeFi compatibility (most protocols expect 18)
- Precision for cross-asset operations
- Simplified multi-asset accounting

---

### 4. ERC7575VaultUpgradeable (UUPS Upgradeable)

**Storage Pattern (ERC-7201):**
```solidity
// Line 96-123
bytes32 private constant VAULT_STORAGE_SLOT =
    keccak256("erc7575.vault.storage");

struct VaultStorage {
    // Core vault references
    address asset;
    address shareToken;

    // Decimal handling
    uint8 decimals;
    uint256 offset;

    // Investment integration
    address investmentVault;
    address investmentShareToken;
    address investmentManager;

    // Async request tracking
    uint256 totalPendingDeposit;
    uint256 totalClaimableDeposit;
    uint256 totalPendingRedeem;
    uint256 totalClaimableRedeem;
    mapping(address controller => Request) controllerToRequest;

    // Operator system
    mapping(address controller => mapping(address operator => bool)) operators;

    // Configuration
    bool isActive;
    uint256 minimumDepositAmount;
}
```

**Request Structure:**
```solidity
struct Request {
    uint256 pendingDepositRequest;    // Assets pending fulfillment
    uint256 claimableDepositRequest;  // Shares ready to claim
    uint256 pendingRedeemRequest;     // Shares pending fulfillment
    uint256 claimableRedeemRequest;   // Assets ready to claim
}
```

---

## 5. UUPS Upgrade Pattern Implementation

The system uses the Universal Upgradeable Proxy Standard (UUPS) pattern for the investment layer contract (ERC7575VaultUpgradeable) while the settlement layer (WERC7575Vault) remains non-upgradeable for stability.

### Proxy Architecture

**Bare Proxy (ERC1967Proxy):**
```
┌─────────────────────────────┐
│    ERC1967Proxy             │
├─────────────────────────────┤
│ Storage:                    │
│ - _IMPLEMENTATION_SLOT      │ ← Points to implementation address
│ - (all vault state)         │ ← Storage delegated to implementation
│                             │
│ Functions: NONE             │ ← Only delegation, no logic
│ Delegates all calls via     │
│ delegatecall()              │
└─────────────────────────────┘
```

**Implementation Contract (ERC7575VaultUpgradeable):**
```solidity
contract ERC7575VaultUpgradeable is
    Initializable,
    Ownable2StepUpgradeable,
    ReentrancyGuard,
    // ... other interfaces
{
    // Upgrade functions ONLY in implementation
    function upgradeTo(address newImplementation) external onlyOwner {
        ERC1967Utils.upgradeToAndCall(newImplementation, "");
    }

    function upgradeToAndCall(address newImplementation, bytes calldata data)
        external payable onlyOwner
    {
        ERC1967Utils.upgradeToAndCall(newImplementation, data);
    }
}
```

### How UUPS Upgrades Work

**Step 1: Upgrade Initiation**
```
User calls: proxy.upgradeTo(newImplementation)
            ↓
ERC1967Proxy delegates to current implementation
            ↓
Current implementation's upgradeTo() executes
```

**Step 2: Implementation Slot Update**
```solidity
function upgradeTo(address newImplementation) external onlyOwner {
    // ERC1967Utils.upgradeToAndCall() performs:
    // 1. Validates newImplementation has upgradeTo function
    // 2. Updates _IMPLEMENTATION_SLOT in proxy storage
    //    _IMPLEMENTATION_SLOT = newImplementation
    // 3. Emits Upgraded(newImplementation) event
}
```

**Step 3: Storage Preservation**
```
Before Upgrade:
Proxy storage contains:
- All vault state (assets, shares, mappings, etc.)
- Points to Implementation V1

After Upgrade:
Proxy storage UNCHANGED:
- All vault state preserved
- Now points to Implementation V2
```

**Step 4: Subsequent Calls Use New Implementation**
```
User calls: proxy.someFunction()
            ↓
ERC1967Proxy reads _IMPLEMENTATION_SLOT
            ↓
Delegates to NEW implementation
            ↓
New implementation executes with original storage
```

### Access Control

**Only Owner Can Upgrade:**
```solidity
function upgradeTo(address newImplementation) external onlyOwner {
    ERC1967Utils.upgradeToAndCall(newImplementation, "");
}

function upgradeToAndCall(address newImplementation, bytes calldata data)
    external payable onlyOwner
{
    ERC1967Utils.upgradeToAndCall(newImplementation, data);
}
```

**Owner Management:**
- Uses `Ownable2StepUpgradeable` for two-step ownership transfer
- Prevents accidental owner lock-out
- Owner is the only address that can trigger upgrades
- **NO timelock** (intentional per KNOWN_ISSUES.md Section 5)

### Storage Safety

**ERC-7201 Namespaced Storage:**
```solidity
bytes32 private constant VAULT_STORAGE_SLOT =
    keccak256("erc7575.vault.storage");

struct VaultStorage {
    // All vault state packed here
    address asset;
    uint64 scalingFactor;
    bool isActive;
    // ... many more fields
}
```

**Why This Prevents Collisions:**
- Storage not at traditional slots (0, 1, 2, ...)
- Hash-based slot prevents accidental collision
- Inheritance doesn't interfere with state
- Safe to add parent classes in upgrades

**Gap Arrays for Future Expansion:**
```solidity
// At end of VaultStorage struct or separately:
uint256[50] __gap;  // Reserved for future storage variables
```
- Allows adding new state variables without shifting existing ones
- Protects against storage corruption in future upgrades
- Must NOT be removed or reordered in future versions

### Safe Upgrade Patterns

**✅ SAFE Upgrades:**
1. Adding new state variables at the END of structs
2. Adding new functions
3. Changing function implementation (not signature)
4. Adding new events
5. Modifying access control to be MORE restrictive

**❌ UNSAFE Upgrades (Will Corrupt Storage):**
1. Removing state variables
2. Changing order of state variables
3. Changing types of existing variables
4. Removing or reordering gap array slots
5. Changing parent contract order (affects storage layout)

### Why Settlement Layer is Non-Upgradeable

**WERC7575Vault (Settlement Layer) has NO upgrade capability:**
- Intentional for stability and regulatory certainty
- Carriers need guaranteed behavior
- Real-time settlements cannot change behavior
- High security: battle-tested code, no upgrade risk
- Regulatory approval tied to specific implementation

**If bug fixes needed:** Deploy new vault + migrate state (not zero-downtime, but guaranteed safety)

### Why Investment Layer is Upgradeable

**ERC7575VaultUpgradeable (Investment Layer) IS upgradeable:**
- Investment products evolve
- Regulatory requirements change
- Can add new features without affecting settlement layer
- Investor protection: can patch issues faster
- Operational flexibility: adapt to market conditions

---

## Data Flow Diagrams

### Deposit Flow (Async)

```
┌─────────┐
│  USER   │
└────┬────┘
     │ 1. requestDeposit(1000 USDC)
     ▼
┌────────────────┐
│  VaultUpgrade  │
├────────────────┤
│ Receives: 1000 │  Asset State Change:
│ USDC from user │  - User: -1000 USDC
│                │  - Vault: +1000 USDC
│ Updates:       │
│ pending += 1000│  Request State Change:
└────┬───────────┘  - pendingDepositRequest[user] += 1000
     │
     │ Time passes... (off-chain decision)
     │
     │ 2. fulfillDeposit(user, 1000 USDC) [Investment Manager]
     ▼
┌────────────────┐
│  VaultUpgrade  │
├────────────────┤
│ Calculates:    │  Request State Change:
│ shares = 1000  │  - pendingDepositRequest[user] -= 1000
│  * 10^12       │  - claimableDepositRequest[user] += 1e18
│  = 1e18        │
│                │  Share State Change:
│ Mints: 1e18    │  - Vault holds 1e18 shares for user
│ shares to      │  - ShareToken.totalSupply += 1e18
│ vault (held    │
│ for user)      │
└────┬───────────┘
     │
     │ 3. deposit(1000, user) [User claims]
     ▼
┌────────────────┐
│  VaultUpgrade  │
├────────────────┤
│ Transfers:     │  Request State Change:
│ 1e18 shares    │  - claimableDepositRequest[user] = 0
│ vault → user   │
│                │  Share State Change:
│                │  - User: +1e18 shares
│                │  - Vault: -1e18 shares
└────────────────┘
```

### Investment Flow

```
┌──────────────┐
│ Vault has:   │
│ 5000 USDC    │  Breakdown:
│              │  - pendingDeposit: 1000 USDC (reserved)
│ Reserved:    │  - claimableDeposit: 2000 USDC (reserved)
│ 3000 USDC    │  - pendingRedeem: 0
│              │  ────────────────────────────────
│ Available:   │  - Available: 2000 USDC (can invest)
│ 2000 USDC    │
└──────┬───────┘
       │ 1. investAssets(2000 USDC) [Investment Manager]
       ▼
┌──────────────┐
│ VaultUpgrade │  Asset State Change:
├──────────────┤  - Vault: -2000 USDC
│ Calls:       │  - Investment Vault: +2000 USDC
│ investment   │
│ Vault.deposit│  Share State Change:
│ (2000, share │  - ShareToken receives WUSD shares
│  Token)      │  - Amount: ~2000 WUSD (1:1 ratio)
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Investment   │  Share Accounting:
│ Vault (WUSD) │  - ShareToken now holds WUSD shares
├──────────────┤  - Represents invested USDC position
│ Mints WUSD   │  - Earns yield on behalf of users
│ to ShareToken│
│              │  User Impact:
│ Invests USDC │  - Users' IUSD shares unchanged
│ to generate  │  - Underlying value now earning yield
│ yield        │  - ShareToken owns the WUSD position
└──────────────┘
```

### Withdrawal from Investment

```
┌──────────────┐
│ User wants   │
│ to withdraw  │
│ 3000 USDC    │
└──────┬───────┘
       │ Investment Manager sees:
       │ - Vault liquid: 1000 USDC (insufficient)
       │ - Need to withdraw: 2000 USDC from investment
       │
       │ 1. withdrawFromInvestment(2000 USDC)
       ▼
┌──────────────┐
│ VaultUpgrade │  Calculates:
├──────────────┤  - ShareToken owns 2000 WUSD shares
│ Calls:       │  - Need to redeem for 2000 USDC
│ investment   │
│ Vault.redeem │  Share State Change:
│ (minShares,  │  - ShareToken: -2000 WUSD shares
│  thisVault,  │  - Investment Vault burns WUSD
│  ShareToken) │
└──────┬───────┘  Asset State Change:
       │           - Investment Vault: -2000 USDC
       │           - This Vault: +2000 USDC
       ▼
┌──────────────┐
│ Vault now    │  Now vault can fulfill the user's
│ has 3000 USDC│  withdrawal request
│ liquid       │
└──────────────┘
```

---

## Key Algorithms

### 1. Batch Transfer Netting Algorithm

**Purpose:** Process multiple transfers in a single transaction with O(n) complexity instead of O(n²).

**Algorithm:**
```solidity
// Line 628-807 (WERC7575ShareToken.sol)

Step 1: Validate inputs
─────────────────────────
if (debtors.length > MAX_BATCH_SIZE) revert ArrayTooLarge();
if (debtors.length != creditors.length) revert ArrayLengthMismatch();
if (creditors.length != amounts.length) revert ArrayLengthMismatch();

Step 2: Build accounts map (netting)
────────────────────────────────────
struct DebitAndCredit {
    address owner;
    uint256 debit;   // Total amount to deduct
    uint256 credit;  // Total amount to add
}

DebitAndCredit[] memory accounts = new DebitAndCredit[](batchSize * 2);
uint256 accountCount = 0;

for each (debtor[i], creditor[i], amount[i]):
    if debtor == creditor: continue  // Skip self-transfers

    // Find or create debtor entry
    debtorIndex = findOrCreate(debtor in accounts)
    accounts[debtorIndex].debit += amount

    // Find or create creditor entry
    creditorIndex = findOrCreate(creditor in accounts)
    accounts[creditorIndex].credit += amount

Step 3: Apply netted changes
─────────────────────────────
for each account in accounts:
    netAmount = account.debit - account.credit

    if netAmount > 0:  // Net debit (paying out)
        require(_balances[account.owner] >= netAmount)
        _balances[account.owner] -= netAmount
        _rBalances[account.owner] += netAmount

    else if netAmount < 0:  // Net credit (receiving)
        absAmount = -netAmount
        if _rBalances[account.owner] >= absAmount:
            _rBalances[account.owner] -= absAmount
        else:
            _rBalances[account.owner] = 0
        _balances[account.owner] += absAmount

Step 4: Emit events
───────────────────
for each (debtor[i], creditor[i], amount[i]):
    emit Transfer(debtor[i], creditor[i], amount[i])
```

**Example:**
```
Input Transfers:
────────────────
A → B: 100
A → C: 50
B → C: 30
C → A: 20
B → A: 10

Netting Process:
────────────────
Account A: debit = 150 (100+50), credit = 30 (20+10) → net -120 (pays out)
Account B: debit = 30, credit = 110 (100+10) → net +80 (receives)
Account C: debit = 20, credit = 80 (50+30) → net +60 (receives)

Final State Changes:
────────────────────
A: _balances -= 120, _rBalances += 120
B: _rBalances -= min(80, rBalance), _balances += 80
C: _rBalances -= min(60, rBalance), _balances += 60

Zero-Sum Property: -120 + 80 + 60 = 20 ❌ Wait, this doesn't sum to zero!

Correction: The net should be:
A sends: 150
A receives: 30
A net: -120 ✓

B sends: 30
B receives: 110
B net: +80 ✓

C sends: 20
C receives: 80
C net: +60 ✓

Total: -120 + 80 + 60 = +20 ❌

Actually, let me recalculate:
A→B: 100, A→C: 50 = A sends 150
C→A: 20, B→A: 10 = A receives 30
A net: -120 (sends more than receives)

B→C: 30 = B sends 30
A→B: 100 = B receives 100
B net: +70 (not +80, I made an error)

C sends: C→A: 20 = 20
C receives: A→C: 50, B→C: 30 = 80
C net: +60

Wait, let me redo this properly:

Transfers:
A → B: 100
A → C: 50
B → C: 30
C → A: 20
B → A: 10

For A:
  Debits (sending): A→B (100) + A→C (50) = 150
  Credits (receiving): C→A (20) + B→A (10) = 30
  Net: debit 150 - credit 30 = -120 (A loses 120)

For B:
  Debits (sending): B→C (30) + B→A (10) = 40
  Credits (receiving): A→B (100) = 100
  Net: debit 40 - credit 100 = +60 (B gains 60)

For C:
  Debits (sending): C→A (20) = 20
  Credits (receiving): A→C (50) + B→C (30) = 80
  Net: debit 20 - credit 80 = +60 (C gains 60)

Zero-sum check: -120 + 60 + 60 = 0 ✓
```

**Gas Savings:**
- Without netting: 5 transfers × 51k gas = 255k gas
- With netting: 3 net transfers × 51k + overhead = ~180k gas
- Savings: ~30% for this example

---

### 2. Reserved Asset Calculation

**Purpose:** Ensure sufficient liquidity for pending/claimable requests, prevent over-investment.

**Algorithm:**
```solidity
// Line 1083-1096 (ERC7575VaultUpgradeable.sol)

function _calculateReservedAssets() internal view returns (uint256 total) {
    total = $.totalPendingDeposit     // Assets received, shares not minted
          + $.totalClaimableDeposit   // Shares minted, assets not claimed (ERROR!)
          + $.totalPendingRedeem;     // Shares received, assets not released
}
```

**Wait, there's a bug here!** `totalClaimableDeposit` is in SHARES, not assets:

```solidity
// Line 352 (fulfillDeposit)
$.totalClaimableDeposit += shares;  // ← This is SHARES, not assets!

// But in _calculateReservedAssets:
total = $.totalPendingDeposit      // Assets ✓
      + $.totalClaimableDeposit    // SHARES ❌ (wrong unit!)
      + $.totalPendingRedeem;      // Shares ✓
```

**This is a potential vulnerability!** The reserved calculation mixes units.

Correct calculation should be:
```solidity
function _calculateReservedAssets() internal view returns (uint256 total) {
    total = $.totalPendingDeposit                           // Assets
          + _convertToAssets($.totalClaimableDeposit)       // Shares → Assets
          + _convertToAssets($.totalPendingRedeem);         // Shares → Assets
}
```

**Impact:**
- If `totalClaimableDeposit` is large, reserved assets overestimated
- Less assets available for investment (inefficiency, not security issue)
- If `totalClaimableDeposit` is small relative to offset, underestimated
- Could allow over-investment (security issue!)

**Example:**
```
Asset: USDC (6 decimals)
Offset: 10^12

Claimable Deposit: 1e18 shares (should reserve 1e6 USDC)
Current code: reserves 1e18 "assets" ← 1 trillion USDC! (wrong)
Correct code: reserves 1e6 USDC ✓
```

---

### 3. rBalance Adjustment Algorithm

**Purpose:** Update reserved balances to reflect investment returns without actual token transfers.

**Algorithm:**
```solidity
// Line 741-780 (WERC7575ShareToken.sol)

function adjustrBalance(
    address[] calldata accounts,
    uint256[] calldata amounti,  // Amount invested
    uint256[] calldata amountr,  // Amount returned
    uint256[] calldata ts        // Timestamp
) external onlyValidator nonReentrant {

    Step 1: Validate one-time application
    ──────────────────────────────────────
    bytes32 adjustmentHash = keccak256(abi.encode(accounts, amounti, amountr, ts));
    if (_rBalanceAdjustmentsApplied[adjustmentHash]) {
        revert RBalanceAdjustmentAlreadyApplied();
    }
    _rBalanceAdjustmentsApplied[adjustmentHash] = true;

    Step 2: Validate bounds
    ────────────────────────
    require(amounti > 0, "Investment must be positive");
    require(amountr <= amounti * MAX_RETURN_MULTIPLIER, "Return too large");
    require(ts <= block.timestamp, "No future timestamps");

    Step 3: Apply adjustments
    ──────────────────────────
    for each account:
        if amountr > amounti:  // Profit
            profit = amountr - amounti
            _rBalances[account] -= amounti    // Remove invested
            _balances[account] += amountr     // Add returned (principal + profit)

        else if amountr < amounti:  // Loss
            loss = amounti - amountr
            _rBalances[account] -= amounti    // Remove invested
            _balances[account] += amountr     // Add returned (principal - loss)

        else:  // Break even
            _rBalances[account] -= amounti
            _balances[account] += amounti

        // Store adjustment for potential cancellation
        _rBalanceAdjustments[account][ts] = [amounti, amountr];

    Step 4: Emit events
    ───────────────────
    emit RBalanceAdjustmentApplied(accounts, amounti, amountr, ts);
}
```

**Example Scenarios:**

```
Scenario 1: Profit
──────────────────
Initial: _balances[Alice] = 1000, _rBalances[Alice] = 500
Investment: amounti = 500 (all rBalance invested)
Return: amountr = 600 (20% profit)

Calculation:
  profit = 600 - 500 = 100
  _rBalances[Alice] -= 500 → 0
  _balances[Alice] += 600 → 1600

Final: _balances[Alice] = 1600, _rBalances[Alice] = 0
Total: 1600 (was 1500, gained 100) ✓

Scenario 2: Loss
────────────────
Initial: _balances[Alice] = 1000, _rBalances[Alice] = 500
Investment: amounti = 500
Return: amountr = 400 (20% loss)

Calculation:
  loss = 500 - 400 = 100
  _rBalances[Alice] -= 500 → 0
  _balances[Alice] += 400 → 1400

Final: _balances[Alice] = 1400, _rBalances[Alice] = 0
Total: 1400 (was 1500, lost 100) ✓

Scenario 3: Partial Investment
───────────────────────────────
Initial: _balances[Alice] = 1000, _rBalances[Alice] = 500
Investment: amounti = 300 (only 60% of rBalance)
Return: amountr = 360 (20% profit)

Calculation:
  profit = 360 - 300 = 60
  _rBalances[Alice] -= 300 → 200 (200 still invested elsewhere)
  _balances[Alice] += 360 → 1360

Final: _balances[Alice] = 1360, _rBalances[Alice] = 200
Total: 1560 (was 1500, gained 60) ✓
```

**Key Protections:**
1. `MAX_RETURN_MULTIPLIER = 2`: Prevents typos (e.g., accidentally returning 100x)
2. One-time application: Prevents double-counting returns
3. Timestamp validation: Prevents future-dated adjustments
4. Cancellation support: Can undo incorrect adjustments

---

## Storage Layout

### ShareTokenUpgradeable Storage

```
Storage Slot: keccak256("erc7575.sharetoken.storage")
─────────────────────────────────────────────────────

Offset  Size  Variable
─────────────────────────────────────────────────────
0x00    32    assetToVault._inner._positions (AddressToAddressMap)
0x01    32    assetToVault._inner._indexes
0x02    32    vaultToAsset (mapping)
0x03    32    operators (nested mapping)
0x04    20    investmentShareToken
0x05    20    investmentManager
```

### ERC7575VaultUpgradeable Storage

```
Storage Slot: keccak256("erc7575.vault.storage")
──────────────────────────────────────────────────

Offset  Size  Variable
──────────────────────────────────────────────────
0x00    20    asset
0x01    20    shareToken
0x02    1     decimals
0x03    32    offset
0x04    20    investmentVault
0x05    20    investmentShareToken
0x06    20    investmentManager
0x07    32    totalPendingDeposit
0x08    32    totalClaimableDeposit
0x09    32    totalPendingRedeem
0x0A    32    totalClaimableRedeem
0x0B    32    controllerToRequest (mapping)
0x0C    32    operators (nested mapping)
0x0D    1     isActive
0x0E    32    minimumDepositAmount
```

**Storage Gap Pattern:**
```solidity
// For future upgrades, contracts include gaps
uint256[50] private __gap;  // Reserve 50 slots for future variables
```

**Why?** If V2 adds new variables, they use the gap slots without shifting existing storage.

---

## Security Mechanisms

### 1. Reentrancy Protection

**Applied to:**
- All state-changing functions with external calls
- `batchTransfers()` - Complex multi-call operation
- `investAssets()` - External vault interaction
- `withdrawFromInvestment()` - External vault interaction

**Implementation:**
```solidity
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract WERC7575ShareToken is ReentrancyGuard {
    function batchTransfers(...) external onlyValidator nonReentrant {
        // Protected against reentrancy
    }
}
```

**Why Critical:**
```solidity
// Vulnerable pattern (if no ReentrancyGuard):
function withdraw() external {
    uint256 amount = balances[msg.sender];
    balances[msg.sender] = 0;

    (bool success,) = msg.sender.call{value: amount}("");
    // ↑ If msg.sender is malicious contract, could reenter before state updated
}

// Protected pattern:
function withdraw() external nonReentrant {
    uint256 amount = balances[msg.sender];
    balances[msg.sender] = 0;  // State updated BEFORE external call

    (bool success,) = msg.sender.call{value: amount}("");
    // ↑ Reentrancy blocked by nonReentrant modifier
}
```

---

### 2. Access Control Hierarchy

```
┌─────────────────────────────────────┐
│  TIER 1: Owner (Highest Privilege) │
├─────────────────────────────────────┤
│ - Vault registration                │
│ - Validator assignment              │
│ - Investment manager config         │
│ - Emergency pause                   │
│ - Contract upgrades                 │
└─────────────────────────────────────┘
            ↓ delegates to
┌─────────────────────────────────────┐
│  TIER 2: Validator (Medium Priv)   │
├─────────────────────────────────────┤
│ - KYC management                    │
│ - Batch transfers                   │
│ - rBalance adjustments              │
│ - Permit signatures                 │
└─────────────────────────────────────┘
            ↓ authorizes
┌─────────────────────────────────────┐
│  TIER 3: Investment Manager         │
├─────────────────────────────────────┤
│ - Fulfillment operations            │
│ - Investment decisions              │
│ - Asset movement                    │
└─────────────────────────────────────┘
            ↓ controls
┌─────────────────────────────────────┐
│  TIER 4: Vaults (Restricted)       │
├─────────────────────────────────────┤
│ - Token minting                     │
│ - Token burning                     │
│ - Allowance spending                │
└─────────────────────────────────────┘
            ↓ affects
┌─────────────────────────────────────┐
│  TIER 5: Users (Controlled Access) │
├─────────────────────────────────────┤
│ - Deposits (with KYC)               │
│ - Withdrawals (with KYC)            │
│ - Transfers (with permit + KYC)     │
└─────────────────────────────────────┘
```

**No Privilege Escalation Possible:**
- Users cannot become validators
- Validators cannot become owners
- Vaults cannot change their authorization
- Each tier is strictly separated

---

### 3. Input Validation

**Comprehensive validation at every entry point:**

```solidity
// Example: Batch transfers (Line 628-650)
function batchTransfers(...) external onlyValidator nonReentrant {
    // Array size validation
    if (debtors.length > MAX_BATCH_SIZE) revert ArrayTooLarge();

    // Array length consistency
    if (!(debtors.length == creditors.length &&
          creditors.length == amounts.length)) {
        revert ArrayLengthMismatch();
    }

    // Individual element validation in loop
    for (uint256 i = 0; i < batchSize; i++) {
        // Balance sufficiency checked before debit
        if (_balances[debtor] < amount) revert LowBalance();
        // ...
    }
}

// Example: Vault registration (Line 169-188)
function registerVault(address asset, address vaultAddress) external onlyOwner {
    // Zero address validation
    if (asset == address(0)) revert WrongAsset();
    if (vaultAddress == address(0)) revert WrongVaultAddress();

    // Duplicate prevention
    if ($.assetToVault.contains(asset)) revert AssetAlreadyRegistered();

    // Vault validation
    if (address(ERC7575VaultUpgradeable(vaultAddress).asset()) != asset) {
        revert AssetMismatch();
    }
    // ...
}
```

---

### 4. Signature Security (EIP-712)

**Domain Separator:**
```solidity
// Constructed in constructor
constructor(string memory name_, string memory symbol_)
    ERC20(name_, symbol_)
    EIP712(name_, "1")  // ← Domain separator with version
{
    // ...
}

// Domain separator includes:
// - Contract name
// - Version ("1")
// - Chain ID (automatically)
// - Contract address (automatically)
```

**Purpose:** Prevents signature replay attacks:
- **Cross-chain replay**: Different chain ID → different domain separator
- **Cross-contract replay**: Different address → different domain separator
- **Version replay**: Upgrade changes version → different domain separator

**Permit Implementation:**
```solidity
// Line 343-381
function permit(
    address owner,
    address spender,
    uint256 value,
    uint256 deadline,
    uint8 v, bytes32 r, bytes32 s
) public virtual {
    // Step 1: Check deadline
    if (block.timestamp > deadline) {
        revert ERC2612ExpiredSignature(deadline);
    }

    // Step 2: Consume nonce (prevents replay)
    uint256 nonce = _useNonce(owner);

    // Step 3: Construct EIP-712 hash
    bytes32 structHash = keccak256(
        abi.encode(
            keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"),
            owner, spender, value, nonce, deadline
        )
    );
    bytes32 hash = _hashTypedDataV4(structHash);

    // Step 4: Recover signer
    address signer = ECDSA.recover(hash, v, r, s);

    // Step 5: Validate signer (with validator requirement for self-allowance)
    if (owner == spender) {
        // Self-allowance: validator must sign
        if (signer != _validator) revert ERC2612InvalidSigner(signer, owner);
    } else {
        // Regular allowance: owner must sign
        if (signer != owner) revert ERC2612InvalidSigner(signer, owner);
    }

    // Step 6: Set allowance
    _approve(owner, spender, value);
}
```

---

## Integration Guide

### For Protocol Integrators

**❌ STOP: Read This First**

WERC7575 is **NOT COMPATIBLE** with standard ERC-20 integrations. Review the following before proceeding:

### Pre-Integration Checklist

- [ ] Can you implement a custom permit flow?
- [ ] Can you verify all recipients via KYC?
- [ ] Can you accept fulfillment delays (async operations)?
- [ ] Can you handle non-standard transfer behavior?
- [ ] Do you have direct communication with validator?

If any answer is "NO", **DO NOT INTEGRATE**.

---

### Integration Pattern 1: Direct User Deposits

**Scenario:** User deposits USDC into your protocol, which then deposits into WERC7575 vault.

```solidity
// Step 1: User approves your protocol
IERC20(usdc).approve(yourProtocol, amount);

// Step 2: Your protocol receives USDC
IERC20(usdc).transferFrom(user, address(this), amount);

// Step 3: Your protocol approves vault
IERC20(usdc).approve(vault, amount);

// Step 4: Request deposit (async)
uint256 requestId = IVault(vault).requestDeposit(amount, user, address(this));

// Step 5: Wait for investment manager to fulfill
// (Off-chain monitoring required)

// Step 6: User claims shares
IVault(vault).deposit(amount, user);

// CRITICAL: User must be KYC-verified before Step 6!
```

---

### Integration Pattern 2: Protocol-Owned Position

**Scenario:** Your protocol maintains a position in WERC7575 on behalf of users.

```solidity
// Your protocol is the controller, users have claims against you

// Step 1: Request deposit
uint256 requestId = IVault(vault).requestDeposit(
    amount,
    address(this),  // ← Your protocol is the controller
    address(this)
);

// Step 2: After fulfillment, claim shares
uint256 shares = IVault(vault).deposit(amount, address(this));

// Step 3: Track user claims internally
userShares[user] += shares;

// CRITICAL: Your protocol must be KYC-verified!
```

---

### Integration Pattern 3: Operator Delegation

**Scenario:** Users delegate redemption authority to your protocol.

```solidity
// Step 1: User approves your protocol as operator
ShareToken(shareToken).setOperator(yourProtocol, true);

// Step 2: Your protocol can request redemption on user's behalf
IVault(vault).requestRedeem(
    shares,
    user,        // ← User is controller
    address(this) // ← Your protocol is owner (operator)
);

// Step 3: After fulfillment, claim assets
uint256 assets = IVault(vault).redeem(shares, user, user);
```

---

### Common Integration Pitfalls

**❌ Pitfall 1: Using standard `transfer()`**
```solidity
// This WILL FAIL without permit
ShareToken(shareToken).transfer(recipient, amount);
// Reverts: ERC20InsufficientAllowance (no self-allowance)
```

**✅ Correct: Use permit first**
```solidity
// Off-chain: Get validator signature
(v, r, s) = getValidatorSignature(owner, owner, amount, deadline);

// On-chain: Call permit, then transfer
ShareToken(shareToken).permit(owner, owner, amount, deadline, v, r, s);
ShareToken(shareToken).transfer(recipient, amount);
```

---

**❌ Pitfall 2: Assuming immediate execution**
```solidity
// This completes immediately in standard ERC-20
uint256 shares = vault.deposit(amount, receiver);

// WERC7575: This only REQUESTS, doesn't execute!
uint256 requestId = vault.requestDeposit(amount, receiver, msg.sender);
// User must wait for fulfillment!
```

**✅ Correct: Handle async flow**
```solidity
// Step 1: Request
uint256 requestId = vault.requestDeposit(amount, receiver, msg.sender);

// Step 2: Monitor off-chain for fulfillment
// (Check pendingDepositRequest and claimableDepositRequest)

// Step 3: Claim when ready
uint256 shares = vault.deposit(amount, receiver);
```

---

**❌ Pitfall 3: Forgetting KYC requirement**
```solidity
// Minting to non-KYC address
vault.requestDeposit(amount, nonKycUser, msg.sender);
// Later, when fulfilled:
vault.deposit(amount, nonKycUser);  // ← REVERTS: KycRequired
```

**✅ Correct: Ensure KYC first**
```solidity
// Off-chain: Ensure user is KYC-verified
require(shareToken.isKycVerified(user), "User not KYC verified");

// On-chain: Safe to deposit
vault.requestDeposit(amount, user, msg.sender);
```

---

### Testing Your Integration

**Minimum Test Suite:**

```solidity
// Test 1: Permit flow
function testIntegration_PermitFlow() public {
    // Get validator signature
    (uint8 v, bytes32 r, bytes32 s) = signPermit(validator, user, user, amount, deadline);

    // Call permit
    shareToken.permit(user, user, amount, deadline, v, r, s);

    // Verify allowance
    assertEq(shareToken.allowance(user, user), amount);
}

// Test 2: Async deposit flow
function testIntegration_AsyncDeposit() public {
    // Request deposit
    vm.prank(user);
    uint256 requestId = vault.requestDeposit(amount, user, user);

    // Fulfill as investment manager
    vm.prank(investmentManager);
    vault.fulfillDeposit(user, amount);

    // Claim shares
    vm.prank(user);
    uint256 shares = vault.deposit(amount, user);

    assertGt(shares, 0);
}

// Test 3: KYC requirement
function testIntegration_KycRequired() public {
    // Non-KYC user attempts deposit
    vm.prank(nonKycUser);
    vault.requestDeposit(amount, nonKycUser, nonKycUser);

    // Fulfill
    vm.prank(investmentManager);
    vault.fulfillDeposit(nonKycUser, amount);

    // Claim should revert
    vm.prank(nonKycUser);
    vm.expectRevert(abi.encodeWithSignature("KycRequired()"));
    vault.deposit(amount, nonKycUser);
}
```

---

## Conclusion

The WERC7575 system implements a sophisticated multi-asset vault architecture with:

1. **Regulatory Compliance**: KYC/AML enforcement at token level
2. **Yield Generation**: Automated investment of idle assets
3. **Async Operations**: ERC-7540 compliant request-fulfill-claim flow
4. **Multi-Asset Support**: Unified share token across different asset types
5. **Upgrade Capability**: UUPS proxy pattern for maintainability
6. **Centralized Management**: Owner, validator, and investment manager roles

**Key Technical Innovations:**
- Decimal normalization for cross-asset compatibility
- Batch transfer netting for gas efficiency
- Reserved balance tracking for investment safety
- ERC-7201 storage slots for upgrade safety

**Security Considerations:**
- Comprehensive access control
- Reentrancy protection
- Input validation
- Signature security (EIP-712)
- Storage collision prevention

**Integration Requirements:**
- Custom permit flow implementation
- KYC verification capability
- Async operation handling
- Non-standard ERC-20 behavior awareness

For questions or clarifications, please refer to the main documentation or contact the development team.

---

**Document Version:** 1.0
**Last Updated:** 2025-01-05
**Solidity Version:** ^0.8.30
**Framework:** Foundry

# WERC7575 System - Use Case Context

## System Purpose

The WERC7575 smart contract system is the **blockchain settlement layer** within a **multi-tier telecom wholesale voice traffic settlement ecosystem**. It works in conjunction with off-chain platforms (COMMTRADE and WRAPX) and telecom OSS/BSS systems to enable efficient, transparent settlement of inter-carrier voice traffic transactions.

---

## Multi-Tier System Architecture

### Complete Ecosystem Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│           TELECOM WHOLESALE VOICE TRAFFIC SETTLEMENT ECOSYSTEM      │
└─────────────────────────────────────────────────────────────────────┘

TIER 1: Telecom OSS/BSS Systems (Carrier Operations)
┌────────────────────────────────────────────────────────────────────┐
│  Carrier A OSS/BSS  │  Carrier B OSS/BSS  │  Carrier C OSS/BSS     │
│  • Call routing     │  • Call routing     │  • Call routing        │
│  • CDR generation   │  • CDR generation   │  • CDR generation      │
│  • Rate management  │  • Rate management  │  • Rate management     │
└────────────┬───────────────────┬───────────────────┬───────────────┘
             │                   │                   │
             │    Push CDRs (Call Detail Records)    │
             ▼                   ▼                   ▼
┌────────────────────────────────────────────────────────────────────┐
│  TIER 2: COMMTRADE Platform (Off-Chain Smart Contract Engine)      │
├────────────────────────────────────────────────────────────────────┤
│  • Integrates with carrier OSS/BSS systems                         │
│  • Enforces rate exchange agreements                               │
│  • Manages call routing logic                                      │
│  • Accounting of all voice traffic transactions                    │
│  • Aggregates transactions per settlement period                   │
│  • Calculates net positions between carriers                       │
│  • Generates settlement instructions                               │
│  • Pushes individual settlement instructions to WRAPX              │
└────────────┬───────────────────────────────────────────────────────┘
             │
             │    Settlement Instructions (individual transactions)
             ▼
┌────────────────────────────────────────────────────────────────────┐
│  TIER 3: WRAPX Platform (Off-Chain Settlement Entity)              │
├────────────────────────────────────────────────────────────────────┤
│  • Receives settlement instructions from COMMTRADE                 │
│  • Validates settlement calculations                               │
│  • Optimizes transaction batching for gas efficiency               │
│  • Manages blockchain interaction                                  │
│  • Signs transactions as validator                                 │
│  • Pushes batch settlements to blockchain                          │
│  • Monitors blockchain confirmations                               │
│  • Handles settlement disputes                                     │
└────────────┬───────────────────────────────────────────────────────┘
             │
             │    Blockchain Transactions (batch transfers)
             ▼
┌────────────────────────────────────────────────────────────────────┐
│  TIER 4: Blockchain Settlement Layer (WERC7575 Smart Contracts)    │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  4A: On-Chain Settlement (WERC7575ShareToken + WERC7575Vault)      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  • Carrier wallets hold settlement balances                  │  │
│  │  • Permissionless deposits (carriers fund wallets 24/7)      │  │
│  │  • Batch settlement execution (WRAPX validator signature)    │  │
│  │  • Permission-required withdrawals (via WRAPX permit)        │  │
│  │  • Immutable settlement record on blockchain                 │  │
│  │  • Transparent audit trail                                   │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                    ▲                               │
│                                    │ Investment funding            │
│                                    │                               │
│  4B: Investment Layer (ShareTokenUpgradeable + Vault)              │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  • Investors deposit capital (async ERC-7540)                │  │
│  │  • Investment Manager deploys to settlement layer            │  │
│  │  • Funds carrier prepayments and working capital             │  │
│  │  • Earns yield from settlement activity                      │  │
│  │  • Investors redeem with profits                             │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

---

## Layer-by-Layer Breakdown

### TIER 1: Telecom OSS/BSS Systems

**Components:** Carrier operational systems (legacy telecom infrastructure)

**Responsibilities:**
- **Call Routing**: Direct voice traffic between carriers
- **CDR Generation**: Create Call Detail Records for every call
  - Origin/destination numbers
  - Call duration
  - Timestamps
  - Quality metrics
- **Rate Management**: Apply agreed rates per destination/carrier
- **Real-Time Operations**: 24/7 voice traffic handling

**Output:** CDRs (Call Detail Records) pushed to COMMTRADE

**Example:**
```
Carrier A routes call: +1-555-0100 → +44-20-7946-0958
Duration: 15 minutes
Rate: $0.02/minute
Cost: $0.30
CDR sent to COMMTRADE for accounting
```

---

### TIER 2: COMMTRADE Platform (Off-Chain Smart Contract Engine)

**Nature:** Off-chain platform with smart contract capabilities

**Responsibilities:**

**1. OSS/BSS Integration**
- Connects to multiple carrier OSS/BSS systems
- Ingests CDRs in real-time
- Normalizes data formats across carriers

**2. Rate Exchange Management**
- Enforces bilateral rate agreements
- Applies volume discounts
- Handles rate updates
- Manages currency conversions

**3. Call Routing Logic**
- Least cost routing (LCR)
- Quality-based routing
- Load balancing
- Failover management

**4. Transaction Accounting**
- Records every voice traffic transaction
- Applies agreed rates
- Calculates per-carrier balances
- Maintains detailed transaction history

**5. Settlement Preparation**
- Settlements are accounted for in quasi-real-time and pushed when they reached a defined amount or timer.
- Generates settlement instructions

**6. Data Push to WRAPX**
- Pushes individual settlement instructions (not batched)
- WRAPX receives individual transactions for batching and blockchain execution
- Provides transaction details for WRAPX validation and optimization

**Example Settlement Period:**
```
Week 1 Transactions (aggregated by COMMTRADE):
─────────────────────────────────────────────────
Carrier A → Carrier B: 1,000,000 minutes @ $0.02 = $20,000
Carrier B → Carrier C: 800,000 minutes @ $0.025 = $20,000
Carrier C → Carrier A: 500,000 minutes @ $0.03 = $15,000
Carrier A → Carrier C: 300,000 minutes @ $0.028 = $8,400
Carrier B → Carrier A: 600,000 minutes @ $0.022 = $13,200

COMMTRADE calculates net positions:
────────────────────────────────────
Carrier A: -$20,000 - $8,400 + $15,000 + $13,200 = -$200 (net payer)
Carrier B: +$20,000 - $20,000 - $13,200 = -$13,200 (net payer)
Carrier C: +$20,000 - $15,000 + $8,400 = +$13,400 (net receiver)

Individual settlement instructions sent to WRAPX:
──────────────────────────────────────────────────
Transaction 1: Transfer $200 from Carrier A to Carrier C
Transaction 2: Transfer $13,200 from Carrier B to Carrier C

(WRAPX will batch these into a single blockchain transaction)
```

---

### TIER 3: WRAPX Platform (Off-Chain Settlement Entity)

**Nature:** Off-chain settlement management platform

**Responsibilities:**

**1. Settlement Validation**
- Receives settlement instructions from COMMTRADE
- Validates calculations
- Checks for discrepancies
- Confirms carrier balances sufficient

**2. Batch Optimization**
- Receives individual settlement instructions from COMMTRADE
- Aggregates multiple instructions into optimized batches
- Optimizes for gas efficiency
- Groups similar operations
- Schedules blockchain transactions

**3. Blockchain Interaction**
- Acts as validator on WERC7575 contracts
- Signs batch settlement transactions
- Pushes `batchTransfers()` to blockchain
- Monitors transaction confirmations
- Handles failed transactions

**4. Permit Management**
- Controls withdrawal permissions
- Issues permit signatures for valid withdrawals
- Enforces withdrawal rules:
  - No outstanding settlement disputes
  - Regulatory compliance checks
  - Sufficient liquidity maintained
  - AML/KYC validation

**5. Dispute Resolution**
- Manages settlement disputes between carriers
- Holds withdrawals during investigations
- Coordinates with COMMTRADE for data verification
- Releases funds when disputes resolved

**6. Settlement Monitoring**
- Tracks all blockchain settlements
- Generates settlement reports
- Alerts on anomalies
- Maintains audit trail

**Example WRAPX Operation:**
```
WRAPX receives individual instructions from COMMTRADE:
────────────────────────────────────────────────────────
Instruction 1: Transfer $500 from Carrier A to Carrier B
Instruction 2: Transfer $300 from Carrier B to Carrier C
Instruction 3: Transfer $400 from Carrier C to Carrier A
Instruction 4: Transfer $200 from Carrier D to Carrier E
... (50 total individual settlement instructions for 20 carriers)

WRAPX batches and optimizes:
────────────────────────────
• Aggregates all 50 individual instructions
• Applies netting algorithm
• Result: 12 net transfers (76% reduction)

WRAPX pushes single batch to blockchain:
──────────────────────────────────────────
batchTransfers(
    debtors:   [Carrier A, Carrier B, ...],
    creditors: [Carrier C, Carrier D, ...],
    amounts:   [200, 13200, ...]
)

Signed by: WRAPX validator private key
Gas cost: ~200k gas (vs. 1M+ gas if each instruction was separate blockchain tx)
```

---

### TIER 4A: On-Chain Settlement Layer (WERC7575)

### Purpose: Telecom Carrier Settlement Platform

**Primary Users:** Telecom carriers (wholesale voice traffic operators)

**Core Function:** Real-time settlement of inter-carrier voice traffic transactions

### Use Case Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│                    TELECOM SETTLEMENT FLOW                          │
└─────────────────────────────────────────────────────────────────────┘

Step 1: Carrier Onboarding
──────────────────────────
Carrier A (e.g., Verizon Wholesale) → KYC verification
Carrier B (e.g., AT&T Wholesale)   → KYC verification
Carrier C (e.g., T-Mobile Wholesale) → KYC verification

Each carrier gets:
• Wallet address on WERC7575ShareToken
• KYC verification from validator
• Telecom integration deployed (for settlement enforcement)

Step 2: Funding (Permissionless Deposit)
─────────────────────────────────────────
Carrier A deposits: 1,000,000 USDC
Carrier B deposits: 500,000 USDC
Carrier C deposits: 750,000 USDC

Deposits are PERMISSIONLESS (anyone KYC-verified can fund their wallet)
Reason: Carriers need to top up quickly to maintain service

Step 3: Voice Traffic & Settlement
───────────────────────────────────
Throughout the month:
• Carrier A routes 10M minutes through Carrier B's network → owes $500k
• Carrier B routes 8M minutes through Carrier C's network → owes $400k
• Carrier C routes 5M minutes through Carrier A's network → owes $250k

Settlement platform tracks all traffic via telecom integrations

Step 4: Batch Settlement Execution
───────────────────────────────────
Validator (settlement platform) calls:

batchTransfers(
    debtors:   [Carrier A, Carrier B, Carrier C],
    creditors: [Carrier B, Carrier C, Carrier A],
    amounts:   [500000, 400000, 250000]
)

Netting algorithm optimizes:
• Carrier A: -500k + 250k = -250k (net payer)
• Carrier B: +500k - 400k = +100k (net receiver)
• Carrier C: +400k - 250k = +150k (net receiver)

Only 3 state changes instead of complex multi-transfer cascade!

Step 5: Withdrawal (Permission Required)
─────────────────────────────────────────
Carrier B wants to withdraw 100k from their balance:

• Carrier B requests withdrawal from settlement platform (off-chain)
• Settlement platform validates request (e.g., no outstanding payments)
• Settlement platform issues permit signature:
  permit(Carrier B, Carrier B, 100k, deadline, v, r, s)
• Carrier B calls transfer() with permit → withdrawal succeeds

WHY PERMISSION REQUIRED?
• Prevents withdrawal during settlement disputes
• Ensures regulatory compliance (AML checks)
• Allows settlement platform to freeze fraudulent carriers
• Ensures carriers have sufficient liquidity for ongoing settlement obligations

**Technical Note on Permission Enforcement:**
The settlement platform (WRAPX) doesn't directly gate vault withdrawals. Instead:
1. Permission is enforced at the **ShareToken level** via self-allowance
2. WRAPX controls issuance of permit signatures that grant self-allowance
3. Transfer/withdrawal operations check that self-allowance exists before proceeding
4. This creates an indirect gating mechanism - no withdrawal possible without validator-approved permit

This approach keeps the settlement vault simple and stable while the token layer controls access policy.
```

### Key Design Rationale: Settlement Layer

#### 1. Permissionless Deposits (with KYC)
```solidity
// Anyone can deposit IF they're KYC-verified
function deposit(uint256 assets, address receiver) external returns (uint256) {
    // No special permission needed
    // KYC check happens at mint() when shares are created
}
```

**Why?**
- Carriers need to top up wallets quickly (24/7 operation)
- No manual approval bottleneck
- KYC requirement ensures regulatory compliance
- Telecom integration already deployed = verified carrier

#### 2. Dual Authorization for Withdrawals

**A. Direct Transfer (owner withdraws their own funds)**
```solidity
function transfer(address to, uint256 value) public override {
    _spendAllowance(msg.sender, msg.sender, value); // ← Needs self-allowance permit!
    super.transfer(to, value);
}
```

**Why self-allowance required?**
- Settlement disputes must be resolved before withdrawal
- Prevents carriers from withdrawing during fraud investigation
- Regulatory compliance (AML checks on large withdrawals)
- Ensures sufficient liquidity for ongoing settlements
- Settlement platform controls withdrawal timing

**B. Third-Party Transfer (authorized party withdraws on owner's behalf)**
```solidity
function transferFrom(address from, address to, uint256 value) public override {
    _spendAllowance(from, from, value);        // ← Platform authorization (self-allowance)
    return super.transferFrom(from, to, value); // ← Owner delegation (caller allowance)
}
```

**Why BOTH allowances required?**

This is a **dual-authorization model**:

1. **Self-Allowance** (`allowance[from][from]`): Platform/validator permission
   - "Settlement platform permits this carrier to withdraw funds"
   - Set by: Validator via permit signature
   - Checks: No outstanding settlements, no disputes, compliance verified

2. **Caller Allowance** (`allowance[from][caller]`): Owner delegation
   - "Carrier delegates authority to this third party"
   - Set by: Carrier via standard `approve()`
   - Enables: Smart contract automation, authorized operators

**Real-World Example:**
```
Carrier A wants to use InvoicePaymentContract to auto-pay suppliers:

Step 1: Request platform permission
→ Carrier A requests withdrawal clearance from WRAPX
→ WRAPX verifies: no disputes, sufficient balance, compliance OK
→ WRAPX issues permit: allowance[CarrierA][CarrierA] = 1M USDC

Step 2: Delegate to smart contract
→ Carrier A: approve(InvoicePaymentContract, 500k USDC)
→ allowance[CarrierA][InvoicePaymentContract] = 500k

Step 3: Automated payment execution
→ InvoicePaymentContract calls: transferFrom(CarrierA, Supplier, 100k)
→ Checks platform authorization: allowance[CarrierA][CarrierA] ≥ 100k ✓
→ Checks owner delegation: allowance[CarrierA][Contract] ≥ 100k ✓
→ Payment succeeds, both allowances reduced by 100k
```

**Benefits:**
- Platform maintains oversight (prevents unauthorized withdrawals)
- Carrier retains control (can delegate to trusted parties)
- Smart contract integration possible (with platform approval)
- Granular control (different limits for platform vs. delegation)

#### 2a. Non-Standard ERC20 Behavior

The ShareToken deliberately deviates from standard ERC20 in several ways to enforce compliance and settlement safety:

**1. Self-Approval Blocked**
```solidity
function approve(address spender, uint256 value) public override {
    if (msg.sender == spender) revert ERC20InvalidSpender(msg.sender);
    super.approve(spender, value);
}
```
- Prevents users from creating their own self-allowance
- Forces validator/settlement platform to explicitly grant withdrawal permissions
- Ensures no withdrawal happens without platform approval

**2. Transfer Requires Self-Allowance**
```solidity
function transfer(address to, uint256 value) public override {
    _spendAllowance(msg.sender, msg.sender, value);  // Must have validator permit
    super.transfer(to, value);
}
```
- **Standard ERC20**: `transfer()` only requires you own the tokens
- **WERC7575**: `transfer()` also requires validator-issued self-allowance permit
- Enforces settlement safety: funds committed to pending settlements cannot be moved

**3. KYC Enforcement**
```solidity
function transfer(address to, uint256 value) public override {
    if (!isKycVerified[to]) revert KycRequired();
    // ... rest of transfer
}
```
- Recipients must be KYC-verified before receiving tokens
- Regulatory requirement for wholesale telecom settlements
- Prevents transfers to unverified addresses

**Integration Implications:**

| Scenario | Standard ERC20 | WERC7575 |
|----------|----------------|---------|
| User transfers tokens | Works without permission | Requires validator permit |
| Smart contract interaction | Can call `approve()` + `transferFrom()` | Must get validator permit for self-allowance |
| Wallet integration | MetaMask "Send" works | MetaMask fails (no permit mechanism) |
| DEX integration | Works directly | Cannot integrate (requires permits) |
| Standard approvals | Any address can approve themselves | Self-approval blocked |

**This is NOT compatible with standard DeFi tooling and is intentionally NOT designed to be.** The token enforces a compliance-first model suitable for commercial wholesale settlements, not permissionless DeFi.

#### 3. Batch Settlement Optimization
```solidity
function batchTransfers(
    address[] calldata debtors,
    address[] calldata creditors,
    uint256[] calldata amounts
) external onlyValidator nonReentrant returns (bool)
```

**Why?**
- **Gas Efficiency**: Thousands of inter-carrier transactions per month
- **Netting**: Carrier A owes B, B owes C, C owes A → optimize to single net transfer
- **Atomic Settlement**: All or nothing (prevents partial settlement)
- **Regulatory Audit Trail**: Single transaction for entire settlement period

**Example Optimization:**
```
Without netting: 1000 individual transfers = 51M gas
With netting:    200 net transfers = 10M gas
Savings:         80% gas reduction
```

#### 4. rBalance System for Investment Tracking
```solidity
// Tracks investment contract's funds: available vs. invested
_balances[investmentContract]  // Available for funding
_rBalances[investmentContract] // Invested in deals (not yet returned)
```

**Why?**
- **Investment Tracking**: Investment contract deploys capital into telecom deals
- **Yield Distribution**: When deals profitable, adjust rBalance to reflect returns
- **Liquidity Management**: Know how much available for new deals vs. locked in existing deals
- **Regulatory Reporting**: Separate liquid funds from invested capital

**Note:** Carriers do NOT have rBalance tracking. Only the investment contract (ShareTokenUpgradeable) uses rBalance to track its deployed capital and returns.

**Example:**
```
Investment contract (ShareTokenUpgradeable) has 1M USDC deposited in settlement layer:
• _balances[investmentContract] = 600k (available for funding new deals)
• _rBalances[investmentContract] = 400k (invested in deals, earning yield)

When deal returns 20% profit:
• adjustrBalance(investmentContract, 400k invested, 480k returned)
• _balances[investmentContract] = 600k (unchanged - still available)
• _rBalances[investmentContract] = 480k (increased from 400k)
• Investment contract earned 80k profit (480k - 400k)
```

---

## TIER 2: Investment Layer (Upgradeable)

### Purpose: Investment Capital for Telecom Deals

**Primary Users:** Investors (not telecom carriers)

**Core Function:** Collect investment capital and deploy it into settlement contract to fund telecom traffic deals

### Use Case Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│                    INVESTMENT FLOW                                  │
└─────────────────────────────────────────────────────────────────────┘

Step 1: Investor Onboarding
────────────────────────────
Investor deposits USDC → ERC7575VaultUpgradeable

Request → Fulfill → Claim (ERC-7540 async flow)
• Request: Investor transfers USDC to vault
• Fulfill: Investment Manager converts to shares (when ready)
• Claim: Investor receives IUSD shares

Step 2: Investment Deployment
──────────────────────────────
Investment Manager takes vault's idle USDC and invests:

investAssets(amount) → deposits into WERC7575Vault (Settlement Layer)

WERC7575Vault mints WUSD shares to ShareTokenUpgradeable

ShareTokenUpgradeable holds WUSD shares on behalf of investors

Step 3: Telecom Deal Funding
─────────────────────────────
Investment capital in Settlement Layer used for:
• Funding carrier prepayments
• Working capital for voice traffic deals
• Margin for settlement float
• Emergency liquidity reserves

Step 4: Yield Generation
────────────────────────
Telecom deals generate profit:
• Settlement fees from carriers
• Voice traffic margins
• Interest on prepayments

Settlement platform adjusts rBalance:
adjustrBalance(ShareTokenUpgradeable, invested, returned)

Step 5: Investor Redemption
────────────────────────────
Investor wants to exit:

• Request redemption of IUSD shares
• Investment Manager withdraws from Settlement Layer
• Investor receives USDC + profit
```

### Key Design Rationale: Investment Layer

#### 1. Async Operations (ERC-7540)
```solidity
// Request → Fulfill → Claim
function requestDeposit(uint256 assets, address controller, address owner)
function fulfillDeposit(address controller, uint256 assets)
function deposit(uint256 assets, address receiver)
```

**Why?**
- **Capital Efficiency**: Batch investments when deals available
- **Liquidity Management**: Don't need instant execution
- **Professional Management**: Investment Manager decides timing
- **Risk Management**: Can delay during high volatility

#### 2. Investment into Settlement Contract
```solidity
function investAssets(uint256 amount) external returns (uint256 shares) {
    // Deposit into WERC7575Vault (Settlement Layer)
    shares = IERC7575($.investmentVault).deposit(amount, $.shareToken);
}
```

**Why?**
- **Direct Exposure**: Investors get yield from actual telecom settlements
- **Transparent**: Investment goes directly into operational contract
- **Measurable**: Can track WUSD shares representing settlement position
- **Liquid**: Can withdraw from settlement (with permission) when needed

#### 3. Upgradeable Architecture
```solidity
contract ERC7575VaultUpgradeable is UUPSUpgradeable, OwnableUpgradeable
```

**Why?**
- **Regulatory Adaptation**: Investment products may need compliance updates
- **Feature Additions**: Can add new investment strategies
- **Bug Fixes**: Can patch issues without redeploying
- **Different Standards**: Settlement layer is battle-tested, investment layer evolves

---

## System Interaction: Two Layers Working Together

### Capital Flow

```
INVESTORS                    INVESTMENT LAYER              SETTLEMENT LAYER              CARRIERS
   │                              │                              │                          │
   │ 1. Deposit USDC              │                              │                          │
   ├─────────────────────────────►│                              │                          │
   │                              │                              │                          │
   │                              │ 2. Invest USDC               │                          │
   │                              ├─────────────────────────────►│                          │
   │                              │                              │                          │
   │                              │    (WUSD shares to           │                          │
   │                              │◄─────ShareToken)             │                          │
   │                              │                              │                          │
   │                              │                              │ 3. Fund telecom deals    │
   │                              │                              ├─────────────────────────►│
   │                              │                              │                          │
   │                              │                              │ 4. Settlements & fees    │
   │                              │                              │◄─────────────────────────┤
   │                              │                              │                          │
   │                              │ 5. Yield generated           │                          │
   │                              │    (rBalance adjustments)    │                          │
   │                              │◄─────────────────────────────┤                          │
   │                              │                              │                          │
   │ 6. Redeem + profit           │                              │                          │
   │◄─────────────────────────────┤                              │                          │
   │                              │                              │                          │
```

### Yield Generation Mechanism

**Settlement Layer generates profit from:**
1. **Settlement Fees**: Carriers pay fee per settlement
2. **Voice Traffic Margins**: Buy/sell voice minutes
3. **Prepayment Interest**: Carriers prepay for volume discounts
4. **Liquidity Services**: Premium for instant settlement

**Profit Distribution:**
1. Settlement platform calculates returns per period
2. Calls `adjustrBalance()` on ShareTokenUpgradeable's position
3. ShareTokenUpgradeable's WUSD shares increase in value
4. IUSD share price increases proportionally
5. Investors can redeem IUSD for more USDC than deposited

### Example: End-to-End Flow

```
Month 1:
────────
• Investor deposits 100k USDC → receives 100k IUSD shares
• Investment Manager invests 100k USDC → Settlement Layer
• Settlement Layer mints 100k WUSD shares → ShareTokenUpgradeable
• Investment used to fund Carrier A's traffic deals

Month 2:
────────
• Settlement activity generates 10k profit
• Settlement platform adjusts: adjustrBalance(ShareToken, 100k, 110k)
• ShareTokenUpgradeable now has 110k value in Settlement Layer
• IUSD share price: 110k / 100k = 1.10 USDC per IUSD

Month 3:
────────
• Investor redeems 100k IUSD shares
• Investment Manager withdraws 110k USDC from Settlement Layer
• Investor receives 110k USDC
• Profit: 10k USDC (10% return)
```

---

## Why Two Separate Systems?

### Separation of Concerns

| Aspect | Settlement Layer | Investment Layer |
|--------|------------------|------------------|
| **Users** | Telecom carriers | Investors |
| **Purpose** | Operational settlement | Capital deployment |
| **Deposits** | Permissionless (with KYC) | Async (managed) |
| **Withdrawals** | Permission required | Managed by IM |
| **Architecture** | Non-upgradeable (stable) | Upgradeable (flexible) |
| **Standards** | ERC-4626, ERC-7575, ERC-7540 | ERC-4626, ERC-7575, ERC-7540, ERC-7887 |
| **Gas Priority** | Critical (frequent) | Less critical (batched) |
| **Audit Status** | Battle-tested | Evolving |

*Note: The Settlement Layer's ShareToken (WERC7575ShareToken) additionally implements ERC-2612 (permit) for signature-based approvals.*

### Why Settlement is Non-Upgradeable

**Stability is Critical:**
- Handles millions in carrier funds
- Real-time settlements cannot fail
- Carriers need certainty of behavior
- Battle-tested code = lower risk
- Regulatory approval = hard to change

### Why Investment is Upgradeable

**Flexibility is Valuable:**
- Investment products evolve
- Regulatory requirements change
- Can add new features (e.g., different yield strategies)
- Bug fixes without affecting carriers
- Can adapt to market conditions

---

## Centralization Rationale in Context

### Settlement Layer Centralization

**Why WRAPX (Validator) Controls Withdrawals:**
```
Real-world scenario:
─────────────────────
Carrier A withdraws 1M USDC
BUT they have 500k outstanding settlement with Carrier B
PROBLEM: Carrier B cannot settle now!

Solution: WRAPX permit system
────────────────────────────────────
Carrier A requests withdrawal → WRAPX checks via COMMTRADE:
  ✓ No outstanding disputes (COMMTRADE confirms)
  ✓ No pending settlements (COMMTRADE confirms)
  ✓ Regulatory compliance (KYC status current)
  ✗ Large withdrawal → manual review

Only after approval → WRAPX issues permit signature → withdrawal succeeds
```

**Why Batch Settlements by WRAPX (Validator):**
```
Without batching (direct OSS/BSS → blockchain):
───────────────────────────────────────────────
1000 carriers × 100 transactions each = 100,000 individual blockchain transfers
Cost: Prohibitively expensive in gas
Risk: Some transfers fail = inconsistent state
No optimization possible

With multi-tier architecture (OSS/BSS → COMMTRADE → WRAPX → blockchain):
─────────────────────────────────────────────────────────────────────────
TIER 1 (OSS/BSS): Generates CDRs for all voice traffic
TIER 2 (COMMTRADE):
  • Aggregates CDRs
  • Calculates net positions
  • Sends individual settlement instructions to WRAPX
TIER 3 (WRAPX):
  • Receives individual settlement instructions from COMMTRADE
  • Batches multiple instructions together
  • Optimizes with netting algorithm
  • Pushes single atomic batch to blockchain
TIER 4 (Blockchain): Executes batched settlement

Result: 100,000 CDRs → 5,000 settlement instructions → 200 batched blockchain txs
Cost: 95% gas savings
Risk: All-or-nothing = consistent state
Benefit: COMMTRADE handles complex rate logic, WRAPX optimizes blockchain efficiency
```

**Why KYC Required:**
```
Regulatory requirement:
──────────────────────
Telecom settlements = financial services
Multi-jurisdiction carriers = AML compliance
Large transaction volumes = monitoring required
Fraudulent carriers = industry risk

Solution: KYC before wallet creation
────────────────────────────────────
Every carrier verified before COMMTRADE integration
Telecom OSS/BSS integration = identity verification
WRAPX maintains KYC status
Ongoing monitoring via COMMTRADE suspicious activity detection
```

**Why Multi-Tier Architecture (OSS/BSS → COMMTRADE → WRAPX → Blockchain):**
```
Single-tier approach problems:
──────────────────────────────
❌ Every CDR becomes a blockchain transaction = cost prohibitive
❌ Rate logic on-chain = complex, expensive, hard to update
❌ OSS/BSS systems can't directly interact with blockchain
❌ No optimization layer for gas efficiency
❌ Dispute resolution requires on-chain arbitration

Multi-tier benefits:
────────────────────
✅ TIER 1 (OSS/BSS): Legacy systems work as-is, no blockchain knowledge needed
✅ TIER 2 (COMMTRADE):
   • Complex rate logic off-chain, flexible, updateable
   • Aggregates CDRs and calculates net positions
   • Sends individual settlement instructions (not batches)
✅ TIER 3 (WRAPX):
   • Receives individual instructions from COMMTRADE
   • Batches instructions for blockchain efficiency
   • Gas optimization through netting algorithm
   • Dispute handling and permit management
✅ TIER 4 (Blockchain): Immutable settlement record, transparent, auditable

Cost efficiency:
───────────────
1M CDRs/month → COMMTRADE aggregates and sends instructions →
WRAPX batches into ~ X blockchain tx/day = 30X blockchain txs/month
Without tiers: 1M blockchain txs/month (33,333x/X more expensive!)
```

### Investment Layer Centralization

**Why Investment Manager Controls Fulfillment:**
```
Capital efficiency scenario:
───────────────────────────
100 investors deposit throughout the month
Each wants immediate shares
BUT only deploy capital when large deal available

Solution: Async fulfillment
───────────────────────────
Investors request deposits (assets secured)
Investment Manager waits for optimal deal
Fulfills all deposits together when deal ready
Capital efficiency: 100% deployed vs. 20% idle
```

**Why Investment Manager Controls Timing:**
```
Risk management scenario:
────────────────────────
High volatility period in telecom markets
Investor requests redemption
BUT withdrawing now = selling at loss

Solution: Managed redemption
────────────────────────────
Investment Manager delays fulfillment
Waits for markets to stabilize
Fulfills when profitable exit available
Protects investor returns
```

---

## Security Considerations in Context

### Settlement Layer Security Priorities

**Critical Invariants:**
1. **Zero-Sum Settlements**: Batch transfers never create/destroy value
2. **Liquidity Protection**: Can't invest reserved settlement funds
3. **Withdrawal Safety**: Permit system prevents unauthorized exits
4. **Atomic Settlements**: All transfers succeed or all revert

**Attack Vectors to Consider:**
- Manipulating batch netting for profit
- Withdrawing during settlement to cause failure
- Double-spending settlement obligations
- rBalance manipulation to fake profits

### Investment Layer Security Priorities

**Critical Invariants:**
1. **Reserved Asset Protection**: Investment can't touch pending/claimable funds
2. **Share Accounting**: IUSD supply matches underlying WUSD position
3. **Fulfillment Accuracy**: Pending → claimable conversions correct
4. **Investment Safety**: Can't over-invest beyond available balance

**Attack Vectors to Consider:**
- Manipulating reserved asset calculation to over-invest
- Exploiting async flow for double-claims
- Front-running fulfillment operations
- Storage corruption during upgrades

---

## Integration Notes for Auditors

### Understanding Context is Critical

**Common Misconception:**
> "Why can't users withdraw freely? This is centralized censorship!"

**Reality:**
This is a **settlement platform** for **commercial counterparties**, not a consumer wallet.

Analogous to:
- **Banking**: Can't withdraw during fraud investigation
- **Escrow**: Can't withdraw without counterparty release
- **Clearing House**: Can't exit during settlement period

**Proper Audit Question:**
> "Can withdrawal permission be abused to steal funds?"
> "Are there safeguards against validator refusing legitimate withdrawals?"

### What Makes This Different from DeFi

| DeFi Standard | WERC7575 Settlement |
|---------------|---------------------|
| Permissionless access | KYC required (regulatory) |
| Instant withdrawals | Permission required (settlement safety) |
| No operator control | Validator controls (operational necessity) |
| Code is law | Code + legal agreements |
| Trust-minimized | Trust professional operators |

**This is NOT a bug, it's the business model.**

---

## Conclusion

The WERC7575 system implements:

1. **Settlement Layer**: Operational platform for telecom carrier settlements
   - Permissionless deposits (operational necessity)
   - Permission-required withdrawals (settlement safety)
   - Batch settlement optimization (cost efficiency)
   - Non-upgradeable architecture (stability)

2. **Investment Layer**: Capital deployment into settlement operations
   - Async operations (capital efficiency)
   - Professional management (risk management)
   - Upgradeable architecture (regulatory flexibility)
   - Yield generation from telecom settlements

**Both architectures are centralized BY DESIGN for legitimate business reasons**, not due to oversight or lack of sophistication.

Auditors should focus on:
- Security vulnerabilities within the intended design
- Business logic correctness
- Standards compliance
- Upgrade safety

NOT:
- The centralization itself
- Comparison to DeFi ideals
- Philosophical objections

---

**Document Version:** 1.0
**Last Updated:** 2025-01-05
**Context:** Telecom Wholesale Voice Traffic Settlement + Investment Platform



 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.10.0",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/openzeppelin-contracts-upgradeable/contracts/package.json

{
  "name": "@openzeppelin/contracts-upgradeable",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.5.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts-upgradeable/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.5.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts-upgradeable/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true,
  "dependencies": {
    "minimatch": "^3.1.2"
  }
}

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.5.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.5.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true,
  "dependencies": {
    "minimatch": "^3.1.2"
  }
}


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]
solc_version = "0.8.30"
via_ir = true
optimizer = true
optimizer_runs = 200

[fmt]
line_length = 200
tab_width = 4
bracket_spacing = false
int_types = "long"
multiline_func_header = "all"
sort_imports = true

[rpc_endpoints]
mainnet = "https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_API_KEY}"
sepolia = "https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
bepolia = "${BEPOLIA_RPC_URL}"

# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options


