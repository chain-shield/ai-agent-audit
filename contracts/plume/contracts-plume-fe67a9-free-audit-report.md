# contracts/plume - Findings Report
## Commit hash: fe67a98fa4344520c5ff2ac9293f5d9601963983

## Protocol Overview 

**Plume Protocol Overview**

Plume is an upgrade-friendly, modular ecosystem that combines a delegated staking protocol with gamified rewards.

* **Diamond-Based Staking** – The PlumeStaking contract is a Diamond proxy split into facets:
  * **StakingFacet** – stake, unstake, withdraw and restake logic.
  * **RewardsFacet** – multi-token emissions, on-demand reward calculation via checkpoint indices, and treasury payouts.
  * **ValidatorFacet** – adds validators, updates commission/capacity, and enforces unanimous slashing votes.
  * **Management & AccessControl Facets** – system parameters, pruning and role management.
* **Separate Treasury** – A UUPS proxy (PlumeStakingRewardTreasury) securely holds reward tokens; only the staking diamond can instruct distributions.
* **Checkpoint Accounting** – Both reward rates and validator commission changes create checkpoints. Binary-searched indices let the contract compute exact rewards at claim time without per-block updates.
* **Slashing** – Other validators cast time-limited votes; unanimity triggers an immediate slash that burns stake/cooling balances and freezes rewards.
* **Gamification Layer** – The upgradable Spin contract lets users pay PLUME for a daily spin that can yield tokens, raffle tickets or jackpots. Tickets are consumed in the multi-winner Raffle contract; winners are picked with Supra VRF.

Together, these pieces deliver secure, gas-efficient staking yields while driving engagement through on-chain games.
## Critical Risk Findings
[C-1]. Upgradeability Initializer Safety issue found with Critical severity


### Number of Findings
- C: 1
- H: 0
- M: 0
- L: 0
- I: 0



# Critical Risk Findings

## [C-1]. Upgradeability Initializer Safety issue in AccessControlFacet::initializeAccessControl

## Description
The `AccessControlFacet.initializeAccessControl()` function is declared as `external` and lacks any access control, allowing any address to call it. This function is responsible for setting up the entire role-based access control system for the diamond proxy. It grants the `DEFAULT_ADMIN_ROLE` and the powerful `ADMIN_ROLE` to `msg.sender`. An attacker can front-run the legitimate deployer's transaction that calls this function. By sending their own transaction with a higher gas fee, the attacker can have `initializeAccessControl()` execute with themselves as `msg.sender`, thereby seizing ownership and administrative control of the entire PlumeStaking protocol. Once the attacker becomes the admin, they can grant themselves all other privileged roles, including the `TIMELOCK_ROLE` which, according to the documentation, can perform administrative withdrawals.

## Impact
A successful exploit results in a complete hostile takeover of the protocol's governance and administrative functions. The attacker gains `ADMIN_ROLE`, allowing them to manipulate critical system parameters, manage all roles, and grant themselves the ability to drain funds from the contract via privileged functions like `adminWithdraw`. This constitutes a critical risk leading to a potential total loss of funds managed by the system.

## Proof of Concept
1. The legitimate deployer deploys the `PlumeStaking` diamond contract and adds the `AccessControlFacet`.
2. The deployer prepares and submits a transaction to call `initializeAccessControl()` to set themselves as the admin.
3. An attacker, monitoring the mempool, sees this transaction.
4. The attacker immediately creates and broadcasts their own transaction calling `initializeAccessControl()`, but with a higher gas price to ensure it gets mined first (front-running).
5. The attacker's transaction executes first. The `initializeAccessControl()` function runs, setting the attacker's address as the holder of `ADMIN_ROLE` and setting the `accessControlFacetInitialized` flag to true.
6. When the deployer's transaction is finally processed, it reverts due to the `require(!$.accessControlFacetInitialized, "ACF: init")` check.
7. The attacker is now the sole administrator of the protocol, while the legitimate deployer is locked out.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

/* --------------------------------------------------------------------------
 * Minimal copies of the production contracts just to compile the PoC
 * --------------------------------------------------------------------------*/

library PlumeRoles {
    bytes32 constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
}

contract AccessControlInternal {
    struct RoleData { mapping(address => bool) members; bytes32 adminRole; }
    struct Layout   { mapping(bytes32 => RoleData) roles; }
    bytes32 private constant SLOT = 0x17ec921503576a6b5d9518f886475051a15328905a418db700a40d5884b2565b;

    function _s() internal pure returns (Layout storage l) { assembly { l.slot := SLOT } }
    function _grantRole(bytes32 r, address a) internal { _s().roles[r].members[a] = true; }
    function _hasRole(bytes32 r, address a) internal view returns (bool) { return _s().roles[r].members[a]; }
}

contract AccessControlFacet is AccessControlInternal {
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant ADMIN_ROLE        = PlumeRoles.ADMIN_ROLE;

    bool internal initialized;

    // VULNERABLE: anyone can call once
    function initializeAccessControl() external {
        require(!initialized, "ACF: init");
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE,        msg.sender);
        initialized = true;
    }

    function hasRole(bytes32 r, address a) external view returns (bool) { return _hasRole(r,a); }
}

/* --------------------------------------------------------------------------
 * Proof-of-Code test
 * --------------------------------------------------------------------------*/

contract UnprotectedInitializerFixedTest is Test {
    AccessControlFacet facet;
    address deployer  = makeAddr("deployer");
    address attacker  = makeAddr("attacker");

    function setUp() public {
        vm.startPrank(deployer);
        facet = new AccessControlFacet();
        vm.stopPrank();
    }

    function testAttackerFrontRunsInitializer() public {
        // attacker calls initialize first
        vm.prank(attacker);
        facet.initializeAccessControl();

        // deployer’s later call reverts
        vm.prank(deployer);
        vm.expectRevert("ACF: init");
        facet.initializeAccessControl();

        // check roles
        assertTrue(facet.hasRole(PlumeRoles.ADMIN_ROLE, attacker), "attacker should be admin");
        assertFalse(facet.hasRole(PlumeRoles.ADMIN_ROLE, deployer), "deployer should NOT be admin");
    }
}


## Suggested Mitigation
The `initializeAccessControl` function must be protected to ensure it can only be called by a privileged address, typically the owner of the diamond contract who is responsible for the initial setup. This prevents a race condition where an attacker could hijack the initialization process.

A common pattern is to add a modifier that restricts access to the contract owner. If the diamond proxy follows a standard like OpenZeppelin's `Ownable`, the check can be performed against its owner.

```solidity
// It is assumed the Diamond proxy has an owner, and a way to retrieve it.
// This example uses a generic `owner()` function for illustration.
interface IDiamond {
    function owner() external view returns (address);
}

contract AccessControlFacet is IAccessControl, AccessControlInternal {
    // ... other code ...

    function initializeAccessControl() external {
        // Add this check to ensure only the owner can initialize.
        require(msg.sender == IDiamond(address(this)).owner(), "ACF: Caller is not the owner");

        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        require(!$.accessControlFacetInitialized, "ACF: init");

        // Grant roles to the owner, not an arbitrary msg.sender
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE, msg.sender);
        
        // ... rest of the function ...

        $.accessControlFacetInitialized = true;
    }

    // ... rest of the contract ...
}
```
Alternatively, if the diamond standard does not expose the owner directly, the initialization logic should be moved into a single, top-level `initialize` function in the main diamond contract. This main initializer would be protected and would then call the initializers for each facet in a single, atomic transaction, eliminating the possibility of front-running individual facet initializations.



