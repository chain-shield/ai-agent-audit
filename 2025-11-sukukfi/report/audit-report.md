# 2025 11 sukukfi - Findings Report
## Commit hash: 18fe2578cf1c6203dac7ff21513533010f3dda3e

##Findings by Status


Finding Status: Valid


[H-1]. Investment Asset Undervaluation Leading to Value Dilution
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-2]. System-wide DoS via UnregisterVault Revert on Broken Vault
**Derived From** : GriefableCallbacks
Finding Status: Valid
Finding Complexity: 0
Privilege: RequiresAdminRole


[M-3]. Missing KYC Validation in ShareTokenUpgradeable transfers
**Derived From** : AccessControl
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[H-4]. Rounding Mismatch in Withdraw Allows Asset Draining
**Derived From** : ERC4626SharePriceMismatch
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[H-5]. Share Price Inflation via Asset Clamping in ERC7575VaultUpgradeable
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Finding Complexity: 0
Privilege: RequiresRole


[M-6]. Share Price Manipulation via Asset Donation
**Derived From** : FlashLoanEconomicManipulation
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[H-7]. Yield distributed via rBalance is permanently locked in Settlement Vault causing insolvency
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Finding Complexity: 0
Privilege: RequiresAdminRole


Finding Status: LowSeverityDueToLowImpact


[L-8]. ERC4626 Preview Functions Revert Violating Standard
**Derived From** : StandardViolation
Finding Status: LowSeverityDueToLowImpact
Finding Complexity: 0
Privilege: Permissionless



Finding Status: InvalidNotExploitable + InvalidByDesign


[M-9]. rBatchTransfers silent truncation of rBalance causes persistent DoS for yield distribution
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidNotExploitable + InvalidByDesign
Finding Complexity: 0
Privilege: RequiresAdminRole


[M-10]. Missing Slippage Protection in Investment Operations
**Derived From** : SlippageMissingOrInsufficient
Finding Status: InvalidNotExploitable + InvalidByDesign
Finding Complexity: 0
Privilege: RequiresRole


[M-11]. Permanent Accounting Deadlock in rBalance Adjustments
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidNotExploitable + InvalidByDesign
Finding Complexity: 0
Privilege: RequiresAdminRole



Finding Status: LowSeverityDueToRareLikelihood


[M-12]. Read-Only Reentrancy in requestDeposit
**Derived From** : Reentrancy
Finding Status: LowSeverityDueToRareLikelihood
Finding Complexity: 0
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake


[M-13]. Unsafe Recipient in claim functions
**Derived From** : UnsafeRecipient
Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
Finding Complexity: 0
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 8
- L: 1
- I: 0

##Findings by Status


Finding Status: LowSeverityDueToLowImpact
## [L-1]. ERC4626 Preview Functions Revert Violating Standard

## id: z1eymzhwxDBnOzpZQdlbD

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
ERC7575VaultUpgradeable.previewDeposit

## Finding Status: LowSeverityDueToLowImpact
### Finding Status Justification: --- Round 1 ---
The async vault intentionally reverts preview functions per ERC-7540 design (async vaults cannot preview synchronous operations). This is documented behavior, not a vulnerability. Standard violation is acknowledged in KNOWN_ISSUES.md Section 2 as intentional design for regulatory compliance. Impact is Low because it only affects off-chain integrations expecting standard ERC4626 behavior, which are explicitly out of scope per Section 3. No asset risk exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ERC7575VaultUpgradeable` implementation causes `previewDeposit`, `previewMint`, `previewWithdraw`, and `previewRedeem` to revert with `AsyncFlow`. This violates the ERC4626 specification, which states these functions MUST return as close to and no more than the exact amount and MUST NOT revert. This breaks composability with standard ERC4626 routers and integrations.

## Impact
Protocol incompatibility and failure of standard integrations expecting ERC4626 behavior.

## Command to Run Test


## Proof of Concept
1. Integrator calls `previewDeposit`. 2. Call reverts. 3. Integration fails.

## Proof of Code
function testPreviewRevert() public { 
    vm.expectRevert(); 
    vault.previewDeposit(100); 
 }

## Suggested Mitigation
Implement preview functions to return 0 or a projected value instead of reverting, or explicitly document non-compliance.





Finding Status: InvalidNotExploitable + InvalidByDesign
## [M-2]. rBatchTransfers silent truncation of rBalance causes persistent DoS for yield distribution

## id: -sqAqHLU2cIwtNkzB4q-v

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.rBatchTransfers

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
Valid Medium severity finding. The rBatchTransfers function silently truncates rBalance to zero when credit exceeds current rBalance, destroying investment tracking. This prevents adjustrBalance from recording losses (underflow revert), creating administrative DoS for yield distribution. While this requires specific conditions (user receiving transfer larger than their rBalance), it's realistic in settlement operations and blocks critical revenue admin functions. Impact is Medium as it prevents yield distribution but doesn't directly lose funds.

--- Round 2 ---
The bug exists in WERC7575ShareToken.rBatchTransfers() lines 1050-1055. When a creditor receives more than their rBalance, the code silently sets rBalance to 0. Later, adjustrBalance() attempts to subtract losses from rBalance, causing underflow revert when rBalance is 0. This creates a DoS where the revenue admin cannot process investment results for active users. No safeguard exists - the truncation logic doesn't prevent future adjustment failures.

--- Round 3 ---
The silent truncation of rBalance to zero when credit exceeds current rBalance is documented as intentional in KNOWN_ISSUES.md Section 7a. The documentation explicitly states: 'When account is creditor (credit > debit): If rBalance < net_credit, set to 0 (silent truncation)' and explains this is 'By design for institutional investment tracking.' The rBalance is described as 'informational for revenue/yield tracking, not critical for transfers' and 'User funds are never lost - actual balances always correct.' While this can cause administrative inconvenience when adjustrBalance attempts to record losses on zeroed rBalance, it's a known design tradeoff prioritizing transfer completion over perfect investment tracking. The system explicitly accepts this limitation as documented behavior.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `WERC7575ShareToken.rBatchTransfers`, if an account receives a credit (net creditor) and is flagged for `rBalance` update, the code decreases `_rBalances`. If the credit amount exceeds the current `rBalance`, it silently truncates `_rBalances` to 0. This destroys the historical investment record. If the Revenue Admin later attempts to call `adjustrBalance` to apply a loss (subtract from `rBalance`) for a previous investment period, the transaction will revert because `_rBalances` is 0 (underflow protection in `adjustrBalance`). This makes it impossible to accurately record investment outcomes for active users.

## Impact
Administrative DoS preventing yield/loss distribution. Inaccurate accounting of user investment positions.

## Command to Run Test


## Proof of Concept
1. User A has `rBalance` = 100 (invested). 
2. Validator calls `rBatchTransfers`. User A receives a transfer of 200 tokens (net credit). 
3. `rBatchTransfers` sets `_rBalances[UserA] = 0` because 100 < 200. 
4. Revenue Admin determines the original 100 investment lost value (now worth 90). 
5. Admin calls `adjustrBalance(UserA, ..., amounti=100, amountr=90)`. 
6. `adjustrBalance` tries `_rBalances[UserA] -= 10`. 
7. Since `_rBalances[UserA]` is 0, this reverts.

## Proof of Code


## Suggested Mitigation
Revise `rBatchTransfers` logic to track the 'excess' credit reduction separately or ensure `adjustrBalance` can handle zeroed balances gracefully.


## [M-3]. Missing Slippage Protection in Investment Operations

## id: i5djJ45t4Y7PiwcT5RQKk

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.investAssets

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
Valid Medium severity finding. The investAssets() and withdrawFromInvestment() functions lack slippage parameters when interacting with external ERC7575 vaults. This exposes the protocol to sandwich attacks or unfavorable exchange rate shifts during investment operations. While the Investment Manager is trusted (governance risk for timing), the external vault's exchange rate can be manipulated by third parties. Impact is Medium as it can cause loss of principal through market manipulation, though it requires specific market conditions or attacker setup.

--- Round 2 ---
The bug exists. ERC7575VaultUpgradeable.investAssets() and withdrawFromInvestment() interact with external ERC7575 vaults without slippage parameters. The functions call deposit/redeem on external vaults that may have variable exchange rates, exposing the protocol to sandwich attacks or unfavorable rate shifts. No minShares or minAssets parameters exist to protect against slippage. This is a legitimate vulnerability in investment operations.

--- Round 3 ---
The investAssets and withdrawFromInvestment functions lack slippage parameters, but this is consistent with the trusted Investment Manager role design. Per KNOWN_ISSUES.md Section 1, the Investment Manager is a trusted privileged role with 'Controls fulfillment timing' and 'Invests idle assets into external vaults.' The Investment Manager is expected to preview operations off-chain before execution and is trusted to act in good faith. Per C4 judging criteria: 'All roles assigned by the system are expected to be trustworthy' and 'Reckless admin mistakes are invalid.' The Investment Manager can check exchange rates via previewDeposit/previewRedeem before calling investAssets/withdrawFromInvestment. This is a governance/centralization risk (QA/Low), not a Medium vulnerability, as it requires the trusted Investment Manager to make errors.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `investAssets` and `withdrawFromInvestment` functions in `ERC7575VaultUpgradeable` interact with an external `IERC7575` vault to deposit and redeem assets. These functions essentially perform token swaps (Asset <-> Share) but do not allow the caller (Investment Manager) to specify a minimum output amount (`minShares` or `minAssets`). 

Because the external vault is `IERC7575` (likely ERC4626-based), the exchange rate can vary. Without slippage protection, the vault is vulnerable to sandwich attacks or unfavorable exchange rate shifts during the transaction, leading to loss of principal.

## Impact
Loss of vault assets due to sandwich attacks or market volatility during investment operations.

## Command to Run Test


## Proof of Concept
1. Investment Manager submits `investAssets(1M USDC)` transaction.
2. Attacker sees tx in mempool.
3. Attacker manipulates external vault exchange rate (e.g. via donation or large deposit/redeem if thin liquidity).
4. Manager's tx executes, receiving far fewer shares than expected.
5. Attacker unwinds, profiting from the slippage.

## Proof of Code
function test_Slippage() public {
    // Mock investment vault
    // Manipulate rate
    // Call investAssets
    // Assert shares received < expected
}

## Suggested Mitigation
Add `minShares` parameter to `investAssets` and `minAssets` parameter to `withdrawFromInvestment`. Enforce these minimums in the transaction.


## [M-4]. Permanent Accounting Deadlock in rBalance Adjustments

## id: h-LJOY7YHHfmJ1yZALyCE

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.adjustrBalance

## Finding Status: InvalidNotExploitable + InvalidByDesign
### Finding Status Justification: --- Round 1 ---
Duplicate of finding -sqAqHLU2cIwtNkzB4q-v. Both describe the same root cause: rBatchTransfers truncates rBalance to zero, then adjustrBalance cannot subtract losses due to underflow. This creates DoS for revenue distribution. Same vulnerability, same impact (Medium), same likelihood (Occasional). The finding correctly identifies the accounting deadlock but is redundant with the earlier submission.

--- Round 2 ---
This is the same issue as finding -sqAqHLU2cIwtNkzB4q-v. The bug exists where rBatchTransfers truncates rBalance to 0, then adjustrBalance attempts to subtract the original investment amount causing underflow. The code in adjustrBalance (lines 741-780) performs unchecked subtraction without verifying sufficient rBalance exists. No safeguard prevents this deadlock scenario. The suggested mitigation of checking balance before subtraction is not implemented.

--- Round 3 ---
This is the same underlying issue as finding -sqAqHLU2cIwtNkzB4q-v, documented in KNOWN_ISSUES.md Section 7a as intentional design. The silent truncation of rBalance in rBatchTransfers can cause adjustrBalance to revert when trying to subtract from insufficient rBalance. However, the documentation explicitly acknowledges this: 'rBalance is informational for revenue/yield tracking, not critical for transfers' and 'User's actual _balances are always correct.' The suggested mitigation in adjustrBalance to check and cap at zero instead of reverting is reasonable, but the current behavior is documented as a known tradeoff. The system prioritizes transfer completion over perfect investment accounting. While this creates administrative friction for the Revenue Admin, it's a documented design choice, not an exploitable vulnerability. User funds remain safe as actual balances are unaffected.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `WERC7575ShareToken` tracks invested capital via `_rBalances`. The `rBatchTransfers` function silently truncates `_rBalances` to zero if a credit transfer exceeds the current `_rBalance` (`if (rbalance < amount) _rBalances = 0`). 

However, the `adjustrBalance` function, used by the Revenue Admin to record investment outcomes, attempts to subtract the *original* investment amount (`amounti`) from `_rBalances` in loss scenarios. 

If `rBatchTransfers` has previously truncated the `_rBalance` to zero (or below `amounti`), `adjustrBalance` will revert due to integer underflow when executing `_rBalances[account] -= difference` (where difference is calculated based on original `amounti`). This creates a deadlock where the admin cannot finalize investment accounting for active users.

## Impact
Denial of Service for revenue distribution. The protocol cannot record investment results for users who have utilized their liquidity, potentially blocking yield distribution for the entire epoch.

## Command to Run Test


## Proof of Concept
1. User A has `rBalance` 100. Admin records investment `amounti` 100.
2. Validator calls `rBatchTransfers`: User A receives credit of 150. `rBalance` -> 0.
3. Investment concludes with loss (return 80). `amounti` (100) > `amountr` (80). Diff = 20.
4. Admin calls `adjustrBalance(User A, ..., 100, 80)`.
5. Logic: `_rBalances[account] -= 20`.
6. `0 - 20` Underflows/Reverts. Admin cannot process this record.

## Proof of Code
function test_rBalanceDeadlock() public {
    uint256 rBal = 0; // Truncated
    uint256 diff = 20;
    // assert(rBal - diff); // Reverts
}

## Suggested Mitigation
In `adjustrBalance`, verify `_rBalances[account] >= difference`. If not, simply set `_rBalances[account] = 0` (similar to the truncation logic in `rBatchTransfers`) instead of reverting.





Finding Status: LowSeverityDueToRareLikelihood
## [M-5]. Read-Only Reentrancy in requestDeposit

## id: hXqJmlk1v2fsTpqD27doF

## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: --- Round 1 ---
Valid Medium severity finding but likelihood is Rare. The requestDeposit() function updates totalPendingDepositAssets after safeTransferFrom, causing transient totalAssets() inflation during transfer hooks. If external protocols query share price during this hook, they receive inflated values. However, this requires: (1) asset token with callback hooks (ERC777 or similar), (2) external protocol querying during exact hook window, (3) external protocol being exploitable via price manipulation. Multiple unlikely conditions make this Rare, though impact is Medium if exploited.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestDeposit` function in `ERC7575VaultUpgradeable` follows a 'Pull-Then-Credit' pattern where it calls `safeTransferFrom` (external call) before updating the `totalPendingDepositAssets` state variable. 

While the function is `nonReentrant`, it updates `balanceOf(this)` during the transfer but not `totalPendingDepositAssets` until after. The `totalAssets()` function (used for pricing) is calculated as `balance - totalPendingDepositAssets`. 

During the transfer hook (if the asset is ERC777 or has callbacks), `balance` is increased but `totalPending` is not. This causes `totalAssets()` to transiently spike. External protocols or views querying the ShareToken price during this hook will receive an inflated value.

## Impact
Manipulation of share price for external observers/integrators. If this protocol is used as an oracle or integrated into other DeFi systems, this allows theft of funds from those systems.

## Command to Run Test


## Proof of Concept
1. Attacker calls `requestDeposit` with ERC777 asset (or hook-enabled token).
2. `safeTransferFrom` triggers attacker's hook.
3. Inside hook, `balanceOf(vault)` is high, `totalPending` is low.
4. `totalAssets()` returns inflated value.
5. Attacker calls `ShareToken.convertToAssets` or similar to exploit the price discrepancy in a secondary system.

## Proof of Code
function test_ReadOnlyReentrancy() public {
    // Mock hook
    // Call requestDeposit
    // In hook, check totalAssets()
    // Assert totalAssets is inflated by deposit amount
}

## Suggested Mitigation
Follow Checks-Effects-Interactions strictly or update state variables before the external transfer call (optimistically), rolling back on failure.





Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
## [M-6]. Unsafe Recipient in claim functions

## id: lt-lYHOze7XJQ_QwHcGBP

## Derived From Pattern/Invariant
UnsafeRecipient

## Exploit Type
UncheckedReturn

## Location
ERC7575VaultUpgradeable.withdraw

## Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
### Finding Status Justification: --- Round 1 ---
This is user error. The withdraw() and redeem() functions allow user-supplied receiver address without zero-address validation. If user accidentally passes address(0), assets are permanently lost. However, this requires explicit user mistake (passing zero address as parameter). Per C4 criteria, 'User chooses bad recipient, provides bad parameters' is user error, not protocol vulnerability. The user must intentionally or mistakenly provide address(0) as receiver parameter. Impact would be Medium (permanent loss) but likelihood is Rare (requires specific user mistake).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdraw` and `redeem` functions in `ERC7575VaultUpgradeable` transfer assets to a user-supplied `receiver` address without verifying it is non-zero. If a user or operator accidentally passes `address(0)`, the assets are sent to the zero address and permanently lost.

## Impact
Permanent loss of user funds

## Command to Run Test


## Proof of Concept
1. User calls `withdraw(amount, address(0), controller)`. 2. Contract burns shares. 3. Contract sends assets to `address(0)`. 4. Assets lost.

## Proof of Code
function testZeroAddressLoss() public { ... }

## Suggested Mitigation
Add `require(receiver != address(0))` in withdraw and redeem functions.



