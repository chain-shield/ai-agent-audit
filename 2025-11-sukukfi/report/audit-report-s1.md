# 2025 11 sukukfi - Findings Report
## Commit hash: 18fe2578cf1c6203dac7ff21513533010f3dda3e

##Findings by Status


Finding Status: Valid


[H-1]. Omission of `pendingRedeemShares` in `totalAssets()` allows over-investment and leads to vault insolvency
**Derived From** : Balance
Finding Status Justification: --- Round 1 ---
This is a critical accounting bug in the reserved assets calculation. The `totalAssets()` function correctly excludes `totalPendingDepositAssets` and `totalClaimableRedeemAssets` but fails to exclude assets backing `pendingRedeemShares`. When users request redemptions, their shares are transferred to the vault but the corresponding asset value is not reserved. The Investment Manager can then call `investAssets()` with these unreserved assets, moving them to the investment vault. When `fulfillRedeem()` is later called, `totalClaimableRedeemAssets` increases but the vault has insufficient liquid assets to honor withdrawals. This creates vault insolvency where users cannot claim their redeemed assets. The bug is in production code (line 1083-1096), requires no special conditions to trigger (any pending redeem request), and directly causes loss of user funds through failed withdrawals. This is a High severity accounting invariant violation with Common likelihood.

--- Round 2 ---
The bug exists. In ERC7575VaultUpgradeable.sol line 1083-1096, `totalAssets()` calculates `reservedAssets` as `totalPendingDepositAssets + totalClaimableRedeemAssets + totalCancelDepositAssets`. However, it omits `pendingRedeemShares` (converted to assets). When users call `requestRedeem()`, shares are transferred to the vault but the corresponding asset value is not reserved. The Investment Manager can then call `investAssets()` with these unreserved assets. Later, `fulfillRedeem()` increases `totalClaimableRedeemAssets`, potentially exceeding available vault balance. This violates the invariant that reserved assets must cover all pending/claimable liabilities. No safeguard exists - the calculation is simply missing the conversion of `$.totalPendingRedeemShares` to assets using `_convertToAssets()`.

--- Round 3 ---
This is a legitimate accounting bug. The `totalAssets()` function calculates `reservedAssets` but omits `pendingRedeemShares` (converted to assets). Line 1093-1096 shows: `reservedAssets = totalPendingDepositAssets + totalClaimableRedeemAssets + totalCancelDepositAssets`. Missing: `_convertToAssets(pendingRedeemShares)`. When users request redemptions, shares are transferred to vault but assets aren't reserved, allowing Investment Manager to invest them via `investAssets()`. Later `fulfillRedeem()` increases `totalClaimableRedeemAssets` but vault lacks liquid assets. Users cannot claim redemptions, causing DoS. This violates the invariant that reserved assets must cover all pending/claimable requests. Exploitable: request redeem → manager invests → fulfillment fails or users cannot claim.
Finding Complexity: 0
Privilege: RequiresRole


[H-2]. Missing caller authorization check in `WERC7575Vault.redeem` allows arbitrary theft of funds
**Derived From** : Access Control
Finding Status Justification: --- Round 1 ---
This is a critical access control vulnerability in the synchronous vault's redeem function. The function calls `_shareToken.spendSelfAllowance(owner, shares)` to verify the self-allowance (validator permission) but never checks if `msg.sender` is authorized to act on behalf of `owner`. In the dual-authorization model, both self-allowance AND caller authorization should be verified. An attacker can monitor for any user who has obtained a withdrawal permit (self-allowance) and immediately call `redeem(shares, attackerAddress, victimAddress)`. The vault will successfully spend the victim's self-allowance, burn their shares, and transfer assets to the attacker. This affects all users including the Investment Vault contract. The vulnerability is in production code (WERC7575Vault.sol), requires no special conditions (just wait for legitimate users to get permits), and enables direct theft of funds. High severity with Common likelihood as any user with self-allowance is vulnerable.

--- Round 2 ---
The bug exists in WERC7575Vault.sol. The `redeem()` function (lines 251-262) calls `_shareToken.spendSelfAllowance(owner, shares)` to verify self-allowance but never checks if `msg.sender` is authorized to act on behalf of `owner`. In the dual-authorization model, both self-allowance (validator permission) AND caller authorization (owner delegation or msg.sender==owner) should be verified. An attacker can call `redeem(shares, attacker, victim)` where victim has self-allowance. The vault burns victim's shares and sends assets to attacker. The synchronous WERC7575Vault lacks the operator system present in the async ERC7575VaultUpgradeable. No safeguard exists - the authorization check for `msg.sender` is completely missing.

--- Round 3 ---
Critical authorization bypass in synchronous vault. The `WERC7575Vault.redeem()` function (lines 251-256 in WERC7575Vault.sol) calls `_shareToken.spendSelfAllowance(owner, shares)` to verify self-allowance but never checks if `msg.sender` is authorized. Missing check: `msg.sender == owner` OR `allowance[owner][msg.sender] >= shares`. Any attacker can call `redeem(shares, attacker, victim)` if victim has self-allowance. Since self-allowance is required for all withdrawals (validator permit), any user ready to withdraw is vulnerable. Direct fund theft: attacker monitors for self-allowance grants, front-runs legitimate withdrawal, steals assets. The `withdraw()` function has the same vulnerability. This breaks the dual-authorization model and enables permissionless theft.
Finding Complexity: 0
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 2
- M: 0
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Omission of `pendingRedeemShares` in `totalAssets()` allows over-investment and leads to vault insolvency

## id: 7Q_5IlQ5Y5C0UOXOic_o0

## Derived From Pattern/Invariant
Balance

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
This is a critical accounting bug in the reserved assets calculation. The `totalAssets()` function correctly excludes `totalPendingDepositAssets` and `totalClaimableRedeemAssets` but fails to exclude assets backing `pendingRedeemShares`. When users request redemptions, their shares are transferred to the vault but the corresponding asset value is not reserved. The Investment Manager can then call `investAssets()` with these unreserved assets, moving them to the investment vault. When `fulfillRedeem()` is later called, `totalClaimableRedeemAssets` increases but the vault has insufficient liquid assets to honor withdrawals. This creates vault insolvency where users cannot claim their redeemed assets. The bug is in production code (line 1083-1096), requires no special conditions to trigger (any pending redeem request), and directly causes loss of user funds through failed withdrawals. This is a High severity accounting invariant violation with Common likelihood.

--- Round 2 ---
The bug exists. In ERC7575VaultUpgradeable.sol line 1083-1096, `totalAssets()` calculates `reservedAssets` as `totalPendingDepositAssets + totalClaimableRedeemAssets + totalCancelDepositAssets`. However, it omits `pendingRedeemShares` (converted to assets). When users call `requestRedeem()`, shares are transferred to the vault but the corresponding asset value is not reserved. The Investment Manager can then call `investAssets()` with these unreserved assets. Later, `fulfillRedeem()` increases `totalClaimableRedeemAssets`, potentially exceeding available vault balance. This violates the invariant that reserved assets must cover all pending/claimable liabilities. No safeguard exists - the calculation is simply missing the conversion of `$.totalPendingRedeemShares` to assets using `_convertToAssets()`.

--- Round 3 ---
This is a legitimate accounting bug. The `totalAssets()` function calculates `reservedAssets` but omits `pendingRedeemShares` (converted to assets). Line 1093-1096 shows: `reservedAssets = totalPendingDepositAssets + totalClaimableRedeemAssets + totalCancelDepositAssets`. Missing: `_convertToAssets(pendingRedeemShares)`. When users request redemptions, shares are transferred to vault but assets aren't reserved, allowing Investment Manager to invest them via `investAssets()`. Later `fulfillRedeem()` increases `totalClaimableRedeemAssets` but vault lacks liquid assets. Users cannot claim redemptions, causing DoS. This violates the invariant that reserved assets must cover all pending/claimable requests. Exploitable: request redeem → manager invests → fulfillment fails or users cannot claim.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `ERC7575VaultUpgradeable` uses `totalAssets()` to calculate the amount of assets available for investment by subtracting `reservedAssets` from the total balance. The `reservedAssets` calculation in `totalAssets()` correctly includes pending deposits and claimable redeems, but **fails to include `pendingRedeemShares`** (converted to assets).

```solidity
// ERC7575VaultUpgradeable.sol
uint256 reservedAssets = $.totalPendingDepositAssets + $.totalClaimableRedeemAssets + $.totalCancelDepositAssets;
// Missing: _convertToAssets($.pendingRedeemShares)
```

As a result, assets that should be reserved to fulfill pending redemption requests are considered 'available' and can be moved out of the vault via `investAssets()`. When the Investment Manager later attempts to `fulfillRedeem()`, the vault may not have sufficient liquid assets, leading to a state where `totalClaimableRedeemAssets > totalAssets()`. This causes user claims (`redeem()`) to revert due to insufficient balance, creating a Denial of Service and violating the invariant that the vault must hold enough liquid assets to cover claimable liabilities.

## Impact
Vault insolvency regarding liquid assets. Users with pending redemptions cannot be paid out if the assets backing their requests are invested. This blocks withdrawals and violates the system's solvency guarantees.

## Command to Run Test


## Proof of Concept
1. User calls `requestRedeem(1000 shares)`.
2. `pendingRedeemShares` increases, but `totalAssets()` (investable balance) does not decrease because `reservedAssets` calculation ignores pending redeems.
3. Investment Manager calls `investAssets(all_available_balance)`.
4. Vault transfers all assets (including those backing the redeem request) to the Investment Vault.
5. Investment Manager calls `fulfillRedeem()`.
6. `totalClaimableRedeemAssets` increases, but Vault balance is 0.
7. User calls `redeem()`. Transaction reverts because Vault has 0 assets to transfer.

## Proof of Code
function test_InsolvencyExploit() public {
    // Setup: Vault has 1000 assets. User requests redeem.
    uint256 amount = 1000;
    vm.prank(user);
    vault.requestRedeem(amount, user, user);

    // Pre-condition: Vault still reports 1000 investable assets despite pending redeem
    assertEq(vault.totalAssets(), 1000);

    // Manager invests everything
    vm.prank(manager);
    vault.investAssets(1000);

    // Manager fulfills redeem (accounting update only)
    vm.prank(manager);
    vault.fulfillRedeem(user, 1000);

    // User tries to claim - fails due to insolvency
    vm.prank(user);
    vm.expectRevert("TransferAmountMismatch"); // Reverts in SafeTokenTransfers
    vault.redeem(1000, user, user);
}

## Suggested Mitigation
Update `totalAssets()` in `ERC7575VaultUpgradeable.sol` to include pending redemptions in the reserved assets calculation: `uint256 reservedAssets = $.totalPendingDepositAssets + $.totalClaimableRedeemAssets + $.totalCancelDepositAssets + _convertToAssets($.totalPendingRedeemShares, Math.Rounding.Ceil);`


## [H-2]. Missing caller authorization check in `WERC7575Vault.redeem` allows arbitrary theft of funds

## id: m6Am6UL6csBPx8usXQL8q

## Derived From Pattern/Invariant
Access Control

## Exploit Type
AccessControl

## Location
WERC7575Vault.redeem

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
This is a critical access control vulnerability in the synchronous vault's redeem function. The function calls `_shareToken.spendSelfAllowance(owner, shares)` to verify the self-allowance (validator permission) but never checks if `msg.sender` is authorized to act on behalf of `owner`. In the dual-authorization model, both self-allowance AND caller authorization should be verified. An attacker can monitor for any user who has obtained a withdrawal permit (self-allowance) and immediately call `redeem(shares, attackerAddress, victimAddress)`. The vault will successfully spend the victim's self-allowance, burn their shares, and transfer assets to the attacker. This affects all users including the Investment Vault contract. The vulnerability is in production code (WERC7575Vault.sol), requires no special conditions (just wait for legitimate users to get permits), and enables direct theft of funds. High severity with Common likelihood as any user with self-allowance is vulnerable.

--- Round 2 ---
The bug exists in WERC7575Vault.sol. The `redeem()` function (lines 251-262) calls `_shareToken.spendSelfAllowance(owner, shares)` to verify self-allowance but never checks if `msg.sender` is authorized to act on behalf of `owner`. In the dual-authorization model, both self-allowance (validator permission) AND caller authorization (owner delegation or msg.sender==owner) should be verified. An attacker can call `redeem(shares, attacker, victim)` where victim has self-allowance. The vault burns victim's shares and sends assets to attacker. The synchronous WERC7575Vault lacks the operator system present in the async ERC7575VaultUpgradeable. No safeguard exists - the authorization check for `msg.sender` is completely missing.

--- Round 3 ---
Critical authorization bypass in synchronous vault. The `WERC7575Vault.redeem()` function (lines 251-256 in WERC7575Vault.sol) calls `_shareToken.spendSelfAllowance(owner, shares)` to verify self-allowance but never checks if `msg.sender` is authorized. Missing check: `msg.sender == owner` OR `allowance[owner][msg.sender] >= shares`. Any attacker can call `redeem(shares, attacker, victim)` if victim has self-allowance. Since self-allowance is required for all withdrawals (validator permit), any user ready to withdraw is vulnerable. Direct fund theft: attacker monitors for self-allowance grants, front-runs legitimate withdrawal, steals assets. The `withdraw()` function has the same vulnerability. This breaks the dual-authorization model and enables permissionless theft.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `WERC7575Vault` (Settlement Layer) implements `redeem` and `withdraw` functions to allow users to burn shares and receive underlying assets. To authorize the burn, the vault calls `_shareToken.spendSelfAllowance(owner, shares)`. This function verifies that `allowance[owner][owner]` is sufficient, which represents the Platform/Validator permission required for withdrawals.

However, the vault fails to verify the second part of the Dual Authorization model: it does **not** check if `msg.sender` is the `owner` or an authorized delegate (via `allowance[owner][msg.sender]`).

Since obtaining a Self-Allowance (via Validator Permit) is a mandatory prerequisite for any legitimate withdrawal, any user who is ready to withdraw is vulnerable. An attacker can monitor for users with Self-Allowance (including the Investment Vault `ERC7575VaultUpgradeable`) and call `redeem(shares, attacker, victim)`. The vault successfully spends the victim's Self-Allowance, burns their shares, and transfers the assets to the attacker.

## Impact
Direct theft of user funds. Any user or contract (including the Investment Layer vault) that holds WUSD shares and has obtained a withdrawal permit can be drained by an attacker.

## Command to Run Test


## Proof of Concept
1. Honest User (or Investment Vault) obtains a Permit signature from Validator to enable withdrawal.
2. User calls `permit()` on `WERC7575ShareToken` to set `allowance[User][User] = amount`.
3. Attacker observes this allowance.
4. Attacker calls `WERC7575Vault.redeem(amount, attacker, User)`.
5. Vault calls `spendSelfAllowance(User, amount)`, which succeeds.
6. Vault burns User's shares.
7. Vault transfers `amount` of Asset to Attacker.

## Proof of Code
function test_ExploitTheft() public {
    // Setup: User has 1000 shares and has obtained Self-Allowance via permit
    uint256 amount = 1000 ether;
    vm.prank(address(vault));
    shareToken.mint(user, amount);
    
    // Simulate Validator Permit setting Self-Allowance
    vm.prank(user);
    // We use internal _approve for test simplicity to simulate permit result
    // In real scenario this comes from permit()
    // shareToken.permit(...) would result in:
    vm.store(address(shareToken), keccak256(abi.encode(user, keccak256(abi.encode(user, 0)))), bytes32(amount));
    
    // Attack
    vm.startPrank(attacker);
    // Attacker calls redeem specifying user as owner and attacker as receiver
    vault.redeem(amount, attacker, user);
    vm.stopPrank();

    // Assertions
    assertEq(asset.balanceOf(attacker), amount, "Attacker stole assets");
    assertEq(shareToken.balanceOf(user), 0, "User lost shares");
}

## Suggested Mitigation
Modify `redeem` and `withdraw` in `WERC7575Vault.sol` to strictly enforce caller authorization. Require that `msg.sender == owner` OR `shareToken.allowance(owner, msg.sender) >= shares`. If the latter, manually spend the caller's allowance.



