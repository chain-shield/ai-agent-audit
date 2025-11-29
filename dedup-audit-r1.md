# 2025 11 brix money - Deduplicated Findings Report
## Commit hash: 79e36aeda1b0091fa3ecd96f398517b31603f5d2

## Summary
This report contains deduplicated findings from the original audit report, excluding Low severity findings.

### Number of Findings (After Deduplication)
- C: 0
- H: 14
- M: 33
- L: 0 (excluded)
- I: 0

**Total: 47 unique findings**

---

## High Severity Findings

### [H-2]. Missing UUPS upgradeability mechanism in iTry implementation bricks upgradeability
**Pattern**: UpgradeabilityInitializerSafety  
**Duplicates**: M-3, M-82, M-98  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 2  
**Privilege**: RequiresAdminRole

### [H-5]. Minter Access Control Bypass via `setMinter` Logic Flaw
**Pattern**: AccessControlOrAuthByPass  
**Duplicates**: M-10, H-101  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 3  
**Privilege**: RequiresAdminRole

### [H-9]. Blacklist Bypass via Cross-chain Unstake
**Pattern**: AccessControlOrAuthByPass  
**Duplicates**: M-46, M-47, M-64, H-90  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 5  
**Privilege**: RequiresRole

### [H-11]. Stuck Funds in Silo due to Incomplete Confiscation Logic for Blacklisted Users
**Pattern**: AccessControlOrAuthByPass  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 4  
**Privilege**: RequiresAdminRole

### [H-22]. Inverted Oracle Price Formula in `iTryIssuer` leads to massive over/under-issuance
**Pattern**: AccountingInvariantViolation  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 3  
**Privilege**: Permissionless

### [H-32]. Cross-chain operations revert due to strict slippage check on dust
**Pattern**: SlippageMissingOrInsufficient  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 3  
**Privilege**: Permissionless

### [H-51]. Redstone Oracle integration fails to propagate payload, causing price update failure and DoS
**Pattern**: ReserveOrPriceDesync / Oracle  
**Duplicates**: H-52, H-55, H-56  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 3  
**Privilege**: Permissionless

### [H-62]. Permanent Fund Lock via Cross-Chain Cooldown Griefing in StakediTryCrosschain
**Pattern**: MaturityorGatingByPass  
**Duplicates**: M-60, M-76, H-77  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 2  
**Privilege**: Permissionless

### [H-65]. Permanent Fund Lock due to Whitelist/Blacklist Reverts on Destination
**Pattern**: UnsafeRecipient  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 4  
**Privilege**: Permissionless

### [H-66]. Griefing/DoS of Cooldowns via Authenticated Data Spoofing in handleCompose
**Pattern**: CrossChainMessageSpoofing  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 3  
**Privilege**: Permissionless

### [H-75]. Missing Whitelist Enforcement in wiTryOFT Enables Compliance Bypass
**Pattern**: Missing Whitelist Enforcement  
**Duplicates**: M-74, M-81  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 2  
**Privilege**: Permissionless

### [H-85]. Blacklisted User Funds Permanently Locked in iTrySilo
**Pattern**: GriefableCallbacks  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 4  
**Privilege**: Permissionless

### [H-88]. Strict Slippage Check in `_fastRedeem` and `_handleUnstake` Causes DoS due to Dust
**Pattern**: Strict slippage check in cross-chain operations  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 3  
**Privilege**: Permissionless

### [H-93]. Implicit 18-Decimal Assumption on Collateral Token in iTryIssuer
**Pattern**: ERC20DecimalsMismatch  
**Duplicates**: M-94  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 1  
**Privilege**: Permissionless

---

## Medium Severity Findings

### [M-1]. Blacklisted Owner Address Causes DoS of Seizure Mechanism and Cross-Chain Channels
**Pattern**: Blacklisted Owner DoS  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 4  
**Privilege**: RequiresRole

### [M-6]. Admin Rescue Functionality Blocked in Paused State
**Pattern**: AccessControlOrAuthByPass  
**Duplicates**: M-44, M-80  
**Status**: Valid  
**Confidence**: VeryConfident  
**Complexity**: 2  
**Privilege**: RequiresAdminRole

### [M-7]. Whitelist Bypass in iTry Token Minting
**Pattern**: AccessControlOrAuthByPass
**Duplicates**: M-8
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: RequiresRole

### [M-12]. StakediTry Reward Distribution DoS
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: RequiresRole

### [M-13]. Accounting desync via FastAccessVault rescueToken
**Pattern**: AccountingInvariantViolation
**Duplicates**: M-53, M-54
**Status**: Valid
**Confidence**: Confident
**Complexity**: 2
**Privilege**: RequiresAdminRole

### [M-14]. Potential value loss due to decimal mismatch in iTryIssuer
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: Permissionless

### [M-15]. Accounting desync via direct token burning prevents yield distribution
**Pattern**: AccountingInvariantViolation
**Duplicates**: M-23, M-24
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: Permissionless

### [M-16]. Compliance Invariant Violation: Bridged Collateral Cannot Be Seized
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 4
**Privilege**: RequiresAdminRole

### [M-17]. User-provided extraOptions ignored in _handleUnstake
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: Permissionless

### [M-18]. Slippage protection bypass in fastRedeem allows loss of user funds
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: Permissionless

### [M-19]. Blacklisted Minter Can Still Mint Tokens
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: RequiresRole

### [M-20]. DoS of Reward Distribution and Confiscation due to Vesting Check
**Pattern**: AccountingInvariantViolation
**Duplicates**: M-58, M-102
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 4
**Privilege**: RequiresRole

### [M-21]. Cross-chain and local cooldown state collision
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 4
**Privilege**: Permissionless

### [M-25]. Redistribution of locked funds blocked by MIN_SHARES check
**Pattern**: AccountingInvariantViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: RequiresAdminRole

### [M-26]. SlippageMissingOrInsufficient (Unbounded Delayed Redemption)
**Pattern**: SlippageMissingOrInsufficient
**Duplicates**: M-27, M-28, M-30, M-31, M-33, M-34, M-35, M-91
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: Permissionless

### [M-29]. Public rebalanceFunds allows griefing of instant redemptions
**Pattern**: SlippageMissingOrInsufficient
**Status**: Valid
**Confidence**: Confident
**Complexity**: 2
**Privilege**: Permissionless

### [M-38]. Excess native fees permanently locked in wiTryVaultComposer
**Pattern**: StandardViolation
**Duplicates**: M-40
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: Permissionless

### [M-39]. Unsafe ERC20 transferFrom usage incompatible with non-standard tokens (USDT)
**Pattern**: StandardViolation
**Status**: Valid
**Confidence**: Confident
**Complexity**: 1
**Privilege**: Permissionless

### [M-42]. Unsafe ERC20 Transfer usage in FastAccessVault causes DoS with USDT
**Pattern**: StandardViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 1
**Privilege**: Permissionless

### [M-45]. maxWithdraw Violates ERC4626 Spec During Cooldown
**Pattern**: StandardViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: Permissionless

### [M-48]. Strict Spender Whitelist Check Breaks ERC20 Integrations
**Pattern**: StandardViolation
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: Permissionless

### [M-49]. Unsafe ERC20 transfer usage ignores SafeERC20 in iTryIssuer
**Pattern**: UncheckedERC20Return
**Duplicates**: M-50
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 1
**Privilege**: Permissionless

### [M-57]. Oracle Specification Mismatch Leading to Incorrect Accounting
**Pattern**: Oracle
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: Permissionless

### [M-59]. Bypass of Restricted Role logic in unstake()
**Pattern**: MaturityorGatingByPass
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: Permissionless

### [M-61]. Inconsistent Cooldown Bypass in Crosschain Logic forces wait during emergency release
**Pattern**: MaturityorGatingByPass
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 2
**Privilege**: RequiresRole

### [M-63]. Cross-Chain Whitelist Desynchronization Permanently Locks Bridged Funds
**Pattern**: UnsafeRecipient
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 4
**Privilege**: Permissionless

### [M-67]. Custodian address state drift between Issuer and Vault
**Pattern**: BeaconOrFactoryAuthorityDrift
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: RequiresRole

### [M-70]. Mutable Minter Variable Desync from Immutable Endpoint Bricks Bridge
**Pattern**: ConfigFootgun
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: RequiresAdminRole

### [M-71]. Admin Rescue Function DoS in Whitelist Mode
**Pattern**: ConfigFootgun
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: RequiresAdminRole

### [M-84]. Premature unstake requests can block ordered LayerZero channels
**Pattern**: FinalityOrReplayAcrossDomains
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 4
**Privilege**: Permissionless

### [M-89]. UnstakeMessenger fee buffer feature DoS due to strict msg.value check and missing refund logic
**Pattern**: ForcedAssetVsStrictEquality
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: Permissionless

### [M-95]. YieldForwarder incompatible with USDT and Fee-on-Transfer tokens violating protocol compatibility
**Pattern**: Issue Type: StandardViolation
**Duplicates**: M-96
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 1
**Privilege**: Permissionless

### [M-100]. Cross-chain Unstake Lockup when Cooldown Disabled
**Pattern**: TimelockEdgeCase
**Status**: Valid
**Confidence**: VeryConfident
**Complexity**: 3
**Privilege**: RequiresRole

---

## Deduplication Notes

### Removed Duplicates (Medium & High only):
- **M-3, M-82, M-98**: Duplicates of H-2 (UUPS upgrade missing)
- **M-8**: Duplicate of M-7 (Whitelist bypass in minting)
- **M-10, H-101**: Duplicates of H-5 (Minter access control bypass)
- **M-23, M-24**: Duplicates of M-15 (Accounting desync via burn)
- **M-27, M-28, M-30, M-31, M-33, M-34, M-35, M-91**: Duplicates of M-26 (Missing slippage protection)
- **M-40**: Duplicate of M-38 (Excess native fees locked)
- **M-44, M-80**: Duplicates of M-6 (Admin rescue blocked in paused state)
- **M-46, M-47, M-64, H-90**: Duplicates of H-9 (Blacklist bypass cross-chain)
- **M-50**: Duplicate of M-49 (Unsafe ERC20 transfer)
- **H-52, H-55, H-56**: Duplicates of H-51 (Redstone oracle payload issue)
- **M-53, M-54**: Duplicates of M-13 (Accounting desync via rescue)
- **M-58, M-102**: Duplicates of M-20 (Vesting check DoS)
- **M-60, M-76, H-77**: Duplicates of H-62 (Cooldown reset griefing)
- **M-74, M-81**: Duplicates of H-75 (Missing whitelist enforcement)
- **M-94**: Duplicate of H-93 (Decimal mismatch)
- **M-96**: Duplicate of M-95 (YieldForwarder USDT incompatibility)

### Original Count: 102 findings (20 High, 65 Medium, 17 Low)
### Deduplicated Count: 47 findings (14 High, 33 Medium, 0 Low - excluded)
### Removed: 55 findings (6 High duplicates, 32 Medium duplicates, 17 Low excluded)

